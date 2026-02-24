use std::collections::HashMap;

use chrono::Utc;

use crate::types::*;

#[derive(Debug, Default)]
pub struct SesState {
    pub identities: HashMap<String, EmailIdentity>,
    pub sent_emails: Vec<SentEmail>,
    pub configuration_sets: HashMap<String, ConfigurationSet>,
    pub contact_lists: HashMap<String, ContactList>,
    pub dedicated_ip_pools: HashMap<String, DedicatedIpPool>,
    pub dedicated_ips: HashMap<String, DedicatedIp>,
    pub email_templates: HashMap<String, EmailTemplate>,
    pub custom_verification_email_templates: HashMap<String, CustomVerificationEmailTemplate>,
    pub import_jobs: HashMap<String, ImportJob>,
    pub export_jobs: HashMap<String, ExportJob>,
    pub multi_region_endpoints: HashMap<String, MultiRegionEndpoint>,
    pub account_settings: AccountSettings,
    pub deliverability_dashboard: DeliverabilityDashboardOptions,
    /// Account-level suppression list keyed by email address.
    pub suppression_list: HashMap<String, SuppressedDestinationEntry>,
    pub tenants: HashMap<String, Tenant>,
    pub tenant_resource_associations: Vec<TenantResourceAssociation>,
    /// Deliverability test reports keyed by report ID.
    pub deliverability_test_reports: HashMap<String, DeliverabilityTestReportRecord>,
    /// Reputation entities keyed by "{entity_type}:{entity_reference}".
    pub reputation_entities: HashMap<String, ReputationEntityRecord>,
}

#[derive(Debug, thiserror::Error)]
pub enum SesError {
    // AlreadyExistsException (400)
    #[error("Identity {0} already exists.")]
    IdentityAlreadyExists(String),
    #[error("Policy {0} already exists.")]
    PolicyAlreadyExists(String),
    #[error("Configuration set {0} already exists.")]
    ConfigurationSetAlreadyExists(String),
    #[error("Event destination {0} already exists in configuration set {1}.")]
    EventDestinationAlreadyExists(String, String),
    #[error("Contact list {0} already exists.")]
    ContactListAlreadyExists(String),
    #[error("Contact {0} already exists.")]
    ContactAlreadyExists(String),
    #[error("Dedicated IP pool {0} already exists.")]
    DedicatedIpPoolAlreadyExists(String),
    #[error("Template {0} already exists.")]
    EmailTemplateAlreadyExists(String),
    #[error("Custom verification email template {0} already exists.")]
    CustomVerificationEmailTemplateAlreadyExists(String),
    #[error("Tenant {0} already exists.")]
    TenantAlreadyExists(String),
    #[error("Resource {0} is already associated with tenant {1}.")]
    TenantResourceAssociationAlreadyExists(String, String),

    // NotFoundException (404)
    #[error("Identity {0} does not exist.")]
    IdentityNotFound(String),
    #[error("Policy {0} does not exist.")]
    PolicyNotFound(String),
    #[error("Configuration set {0} does not exist.")]
    ConfigurationSetNotFound(String),
    #[error("Event destination {0} does not exist in configuration set {1}.")]
    EventDestinationNotFound(String, String),
    #[error("Contact list {0} does not exist.")]
    ContactListNotFound(String),
    #[error("Contact {0} does not exist in list {1}.")]
    ContactNotFound(String, String),
    #[error("Dedicated IP pool {0} does not exist.")]
    DedicatedIpPoolNotFound(String),
    #[error("Dedicated IP {0} does not exist.")]
    DedicatedIpNotFound(String),
    #[error("Template {0} does not exist.")]
    EmailTemplateNotFound(String),
    #[error("Custom verification email template {0} does not exist.")]
    CustomVerificationEmailTemplateNotFound(String),
    #[error("Import job {0} does not exist.")]
    ImportJobNotFound(String),
    #[error("Export job {0} does not exist.")]
    ExportJobNotFound(String),
    #[error("Multi-region endpoint {0} does not exist.")]
    MultiRegionEndpointNotFound(String),
    #[error("Resource {0} does not exist.")]
    ResourceNotFound(String),
    #[error("Suppressed destination {0} does not exist.")]
    SuppressedDestinationNotFound(String),
    #[error("Tenant {0} does not exist.")]
    TenantNotFound(String),
    #[error("Resource {0} is not associated with tenant {1}.")]
    TenantResourceAssociationNotFound(String, String),
    #[error("Deliverability test report {0} does not exist.")]
    DeliverabilityTestReportNotFound(String),
    #[error("Reputation entity {0}/{1} does not exist.")]
    ReputationEntityNotFound(String, String),
}

impl SesState {
    pub fn create_email_identity(
        &mut self,
        name: &str,
        tags: HashMap<String, String>,
    ) -> Result<&EmailIdentity, SesError> {
        if self.identities.contains_key(name) {
            return Err(SesError::IdentityAlreadyExists(name.to_string()));
        }

        let identity_type = if name.contains('@') {
            "EMAIL_ADDRESS"
        } else {
            "DOMAIN"
        };

        let identity = EmailIdentity {
            name: name.to_string(),
            identity_type: identity_type.to_string(),
            verified: true, // mock: auto-verify
            created_timestamp: Utc::now(),
            policies: HashMap::new(),
            tags,
            configuration_set_name: None,
            dkim_signing_enabled: false,
            dkim_signing_key_type: None,
            dkim_domain: None,
            feedback_forwarding_enabled: true,
            mail_from_domain: None,
            behavior_on_mx_failure: None,
        };

        self.identities.insert(name.to_string(), identity);
        Ok(self.identities.get(name).unwrap())
    }

    pub fn get_email_identity(&self, name: &str) -> Result<&EmailIdentity, SesError> {
        self.identities
            .get(name)
            .ok_or_else(|| SesError::IdentityNotFound(name.to_string()))
    }

    pub fn delete_email_identity(&mut self, name: &str) -> Result<(), SesError> {
        if self.identities.remove(name).is_none() {
            return Err(SesError::IdentityNotFound(name.to_string()));
        }
        Ok(())
    }

    pub fn list_email_identities(&self) -> Vec<&EmailIdentity> {
        self.identities.values().collect()
    }

    pub fn get_email_identity_policies(
        &self,
        name: &str,
    ) -> Result<&HashMap<String, String>, SesError> {
        let identity = self.get_email_identity(name)?;
        Ok(&identity.policies)
    }

    pub fn create_email_identity_policy(
        &mut self,
        identity_name: &str,
        policy_name: &str,
        policy: &str,
    ) -> Result<(), SesError> {
        let identity = self
            .identities
            .get_mut(identity_name)
            .ok_or_else(|| SesError::IdentityNotFound(identity_name.to_string()))?;
        if identity.policies.contains_key(policy_name) {
            return Err(SesError::PolicyAlreadyExists(policy_name.to_string()));
        }
        identity
            .policies
            .insert(policy_name.to_string(), policy.to_string());
        Ok(())
    }

    pub fn delete_email_identity_policy(
        &mut self,
        identity_name: &str,
        policy_name: &str,
    ) -> Result<(), SesError> {
        let identity = self
            .identities
            .get_mut(identity_name)
            .ok_or_else(|| SesError::IdentityNotFound(identity_name.to_string()))?;
        if identity.policies.remove(policy_name).is_none() {
            return Err(SesError::PolicyNotFound(policy_name.to_string()));
        }
        Ok(())
    }

    pub fn update_email_identity_policy(
        &mut self,
        identity_name: &str,
        policy_name: &str,
        policy: &str,
    ) -> Result<(), SesError> {
        let identity = self
            .identities
            .get_mut(identity_name)
            .ok_or_else(|| SesError::IdentityNotFound(identity_name.to_string()))?;
        if !identity.policies.contains_key(policy_name) {
            return Err(SesError::PolicyNotFound(policy_name.to_string()));
        }
        identity
            .policies
            .insert(policy_name.to_string(), policy.to_string());
        Ok(())
    }

    pub fn put_email_identity_configuration_set_attributes(
        &mut self,
        identity_name: &str,
        configuration_set_name: Option<&str>,
    ) -> Result<(), SesError> {
        let identity = self
            .identities
            .get_mut(identity_name)
            .ok_or_else(|| SesError::IdentityNotFound(identity_name.to_string()))?;
        identity.configuration_set_name = configuration_set_name.map(|s| s.to_string());
        Ok(())
    }

    pub fn put_email_identity_dkim_attributes(
        &mut self,
        identity_name: &str,
        signing_enabled: bool,
    ) -> Result<(), SesError> {
        let identity = self
            .identities
            .get_mut(identity_name)
            .ok_or_else(|| SesError::IdentityNotFound(identity_name.to_string()))?;
        identity.dkim_signing_enabled = signing_enabled;
        Ok(())
    }

    pub fn put_email_identity_dkim_signing_attributes(
        &mut self,
        identity_name: &str,
        _signing_attributes_origin: &str,
        domain: Option<&str>,
        key_type: Option<&str>,
    ) -> Result<(), SesError> {
        let identity = self
            .identities
            .get_mut(identity_name)
            .ok_or_else(|| SesError::IdentityNotFound(identity_name.to_string()))?;
        identity.dkim_signing_key_type = key_type.map(|s| s.to_string());
        identity.dkim_domain = domain.map(|s| s.to_string());
        Ok(())
    }

    pub fn put_email_identity_feedback_attributes(
        &mut self,
        identity_name: &str,
        email_forwarding_enabled: bool,
    ) -> Result<(), SesError> {
        let identity = self
            .identities
            .get_mut(identity_name)
            .ok_or_else(|| SesError::IdentityNotFound(identity_name.to_string()))?;
        identity.feedback_forwarding_enabled = email_forwarding_enabled;
        Ok(())
    }

    pub fn put_email_identity_mail_from_attributes(
        &mut self,
        identity_name: &str,
        mail_from_domain: Option<&str>,
        behavior_on_mx_failure: Option<&str>,
    ) -> Result<(), SesError> {
        let identity = self
            .identities
            .get_mut(identity_name)
            .ok_or_else(|| SesError::IdentityNotFound(identity_name.to_string()))?;
        identity.mail_from_domain = mail_from_domain.map(|s| s.to_string());
        identity.behavior_on_mx_failure = behavior_on_mx_failure.map(|s| s.to_string());
        Ok(())
    }

    pub fn send_email(
        &mut self,
        from: &str,
        to: Vec<String>,
        subject: &str,
        body: &str,
    ) -> Result<String, SesError> {
        let message_id = format!("{}", uuid::Uuid::new_v4());

        let sent = SentEmail {
            from: from.to_string(),
            to,
            subject: subject.to_string(),
            body: body.to_string(),
            timestamp: Utc::now(),
            message_id: message_id.clone(),
        };

        self.sent_emails.push(sent);
        Ok(message_id)
    }

    // --- Configuration Sets ---

    pub fn create_configuration_set(
        &mut self,
        name: &str,
        tags: HashMap<String, String>,
    ) -> Result<(), SesError> {
        if self.configuration_sets.contains_key(name) {
            return Err(SesError::ConfigurationSetAlreadyExists(name.to_string()));
        }
        self.configuration_sets
            .insert(name.to_string(), ConfigurationSet::new(name, tags));
        Ok(())
    }

    pub fn get_configuration_set(&self, name: &str) -> Result<&ConfigurationSet, SesError> {
        self.configuration_sets
            .get(name)
            .ok_or_else(|| SesError::ConfigurationSetNotFound(name.to_string()))
    }

    pub fn delete_configuration_set(&mut self, name: &str) -> Result<(), SesError> {
        if self.configuration_sets.remove(name).is_none() {
            return Err(SesError::ConfigurationSetNotFound(name.to_string()));
        }
        Ok(())
    }

    pub fn list_configuration_sets(&self) -> Vec<&ConfigurationSet> {
        self.configuration_sets.values().collect()
    }

    // --- Configuration Set Event Destinations ---

    pub fn create_configuration_set_event_destination(
        &mut self,
        config_set_name: &str,
        dest_name: &str,
        enabled: bool,
        matching_event_types: Vec<String>,
        destination: serde_json::Value,
    ) -> Result<(), SesError> {
        let cs = self
            .configuration_sets
            .get_mut(config_set_name)
            .ok_or_else(|| SesError::ConfigurationSetNotFound(config_set_name.to_string()))?;
        if cs.event_destinations.contains_key(dest_name) {
            return Err(SesError::EventDestinationAlreadyExists(
                dest_name.to_string(),
                config_set_name.to_string(),
            ));
        }
        cs.event_destinations.insert(
            dest_name.to_string(),
            EventDestination {
                name: dest_name.to_string(),
                enabled,
                matching_event_types,
                destination,
            },
        );
        Ok(())
    }

    pub fn get_configuration_set_event_destinations(
        &self,
        config_set_name: &str,
    ) -> Result<Vec<&EventDestination>, SesError> {
        let cs = self
            .configuration_sets
            .get(config_set_name)
            .ok_or_else(|| SesError::ConfigurationSetNotFound(config_set_name.to_string()))?;
        Ok(cs.event_destinations.values().collect())
    }

    pub fn delete_configuration_set_event_destination(
        &mut self,
        config_set_name: &str,
        dest_name: &str,
    ) -> Result<(), SesError> {
        let cs = self
            .configuration_sets
            .get_mut(config_set_name)
            .ok_or_else(|| SesError::ConfigurationSetNotFound(config_set_name.to_string()))?;
        if cs.event_destinations.remove(dest_name).is_none() {
            return Err(SesError::EventDestinationNotFound(
                dest_name.to_string(),
                config_set_name.to_string(),
            ));
        }
        Ok(())
    }

    pub fn update_configuration_set_event_destination(
        &mut self,
        config_set_name: &str,
        dest_name: &str,
        enabled: bool,
        matching_event_types: Vec<String>,
        destination: serde_json::Value,
    ) -> Result<(), SesError> {
        let cs = self
            .configuration_sets
            .get_mut(config_set_name)
            .ok_or_else(|| SesError::ConfigurationSetNotFound(config_set_name.to_string()))?;
        let dest = cs.event_destinations.get_mut(dest_name).ok_or_else(|| {
            SesError::EventDestinationNotFound(dest_name.to_string(), config_set_name.to_string())
        })?;
        dest.enabled = enabled;
        dest.matching_event_types = matching_event_types;
        dest.destination = destination;
        Ok(())
    }

    // --- Configuration Set Options ---

    pub fn put_configuration_set_option(
        &mut self,
        config_set_name: &str,
        option_type: &str,
        value: serde_json::Value,
    ) -> Result<(), SesError> {
        let cs = self
            .configuration_sets
            .get_mut(config_set_name)
            .ok_or_else(|| SesError::ConfigurationSetNotFound(config_set_name.to_string()))?;
        match option_type {
            "archiving-options" => cs.archiving_options = Some(value),
            "delivery-options" => cs.delivery_options = Some(value),
            "reputation-options" => cs.reputation_options = Some(value),
            "sending" => cs.sending_options = Some(value),
            "suppression-options" => cs.suppression_options = Some(value),
            "tracking-options" => cs.tracking_options = Some(value),
            "vdm-options" => cs.vdm_options = Some(value),
            _ => {}
        }
        Ok(())
    }

    // --- Contact Lists ---

    pub fn create_contact_list(
        &mut self,
        name: &str,
        description: Option<&str>,
        tags: HashMap<String, String>,
    ) -> Result<(), SesError> {
        if self.contact_lists.contains_key(name) {
            return Err(SesError::ContactListAlreadyExists(name.to_string()));
        }
        let now = Utc::now();
        self.contact_lists.insert(
            name.to_string(),
            ContactList {
                name: name.to_string(),
                description: description.map(|s| s.to_string()),
                tags,
                created_timestamp: now,
                last_updated_timestamp: now,
                contacts: std::collections::HashMap::new(),
            },
        );
        Ok(())
    }

    pub fn get_contact_list(&self, name: &str) -> Result<&ContactList, SesError> {
        self.contact_lists
            .get(name)
            .ok_or_else(|| SesError::ContactListNotFound(name.to_string()))
    }

    pub fn delete_contact_list(&mut self, name: &str) -> Result<(), SesError> {
        if self.contact_lists.remove(name).is_none() {
            return Err(SesError::ContactListNotFound(name.to_string()));
        }
        Ok(())
    }

    pub fn list_contact_lists(&self) -> Vec<&ContactList> {
        self.contact_lists.values().collect()
    }

    // --- Contacts ---

    pub fn create_contact(
        &mut self,
        list_name: &str,
        email_address: &str,
        unsubscribe_all: bool,
        topic_preferences: Vec<TopicPreference>,
    ) -> Result<(), SesError> {
        let list = self
            .contact_lists
            .get_mut(list_name)
            .ok_or_else(|| SesError::ContactListNotFound(list_name.to_string()))?;
        if list.contacts.contains_key(email_address) {
            return Err(SesError::ContactAlreadyExists(email_address.to_string()));
        }
        let now = Utc::now();
        list.contacts.insert(
            email_address.to_string(),
            Contact {
                email_address: email_address.to_string(),
                topic_preferences,
                unsubscribe_all,
                created_timestamp: now,
                last_updated_timestamp: now,
            },
        );
        Ok(())
    }

    pub fn get_contact(&self, list_name: &str, email_address: &str) -> Result<&Contact, SesError> {
        let list = self
            .contact_lists
            .get(list_name)
            .ok_or_else(|| SesError::ContactListNotFound(list_name.to_string()))?;
        list.contacts.get(email_address).ok_or_else(|| {
            SesError::ContactNotFound(email_address.to_string(), list_name.to_string())
        })
    }

    pub fn delete_contact(&mut self, list_name: &str, email_address: &str) -> Result<(), SesError> {
        let list = self
            .contact_lists
            .get_mut(list_name)
            .ok_or_else(|| SesError::ContactListNotFound(list_name.to_string()))?;
        if list.contacts.remove(email_address).is_none() {
            return Err(SesError::ContactNotFound(
                email_address.to_string(),
                list_name.to_string(),
            ));
        }
        Ok(())
    }

    pub fn list_contacts(&self, list_name: &str) -> Result<Vec<&Contact>, SesError> {
        let list = self
            .contact_lists
            .get(list_name)
            .ok_or_else(|| SesError::ContactListNotFound(list_name.to_string()))?;
        Ok(list.contacts.values().collect())
    }

    // --- Dedicated IP Pools ---

    pub fn create_dedicated_ip_pool(
        &mut self,
        pool_name: &str,
        scaling_mode: &str,
        tags: HashMap<String, String>,
    ) -> Result<(), SesError> {
        if self.dedicated_ip_pools.contains_key(pool_name) {
            return Err(SesError::DedicatedIpPoolAlreadyExists(
                pool_name.to_string(),
            ));
        }
        self.dedicated_ip_pools.insert(
            pool_name.to_string(),
            DedicatedIpPool {
                pool_name: pool_name.to_string(),
                scaling_mode: scaling_mode.to_string(),
                tags,
            },
        );
        Ok(())
    }

    pub fn get_dedicated_ip_pool(&self, pool_name: &str) -> Result<&DedicatedIpPool, SesError> {
        self.dedicated_ip_pools
            .get(pool_name)
            .ok_or_else(|| SesError::DedicatedIpPoolNotFound(pool_name.to_string()))
    }

    pub fn delete_dedicated_ip_pool(&mut self, pool_name: &str) -> Result<(), SesError> {
        if self.dedicated_ip_pools.remove(pool_name).is_none() {
            return Err(SesError::DedicatedIpPoolNotFound(pool_name.to_string()));
        }
        Ok(())
    }

    pub fn list_dedicated_ip_pools(&self) -> Vec<&DedicatedIpPool> {
        self.dedicated_ip_pools.values().collect()
    }

    pub fn put_dedicated_ip_pool_scaling_attributes(
        &mut self,
        pool_name: &str,
        scaling_mode: &str,
    ) -> Result<(), SesError> {
        let pool = self
            .dedicated_ip_pools
            .get_mut(pool_name)
            .ok_or_else(|| SesError::DedicatedIpPoolNotFound(pool_name.to_string()))?;
        pool.scaling_mode = scaling_mode.to_string();
        Ok(())
    }

    // --- Dedicated IPs ---

    pub fn get_dedicated_ips(&self, pool_filter: Option<&str>) -> Vec<&DedicatedIp> {
        self.dedicated_ips
            .values()
            .filter(|ip| {
                if let Some(pool) = pool_filter {
                    ip.pool_name.as_deref() == Some(pool)
                } else {
                    true
                }
            })
            .collect()
    }

    pub fn get_dedicated_ip(&self, ip: &str) -> Result<&DedicatedIp, SesError> {
        self.dedicated_ips
            .get(ip)
            .ok_or_else(|| SesError::DedicatedIpNotFound(ip.to_string()))
    }

    pub fn put_dedicated_ip_in_pool(&mut self, ip: &str, pool_name: &str) -> Result<(), SesError> {
        // Ensure the pool exists
        if !self.dedicated_ip_pools.contains_key(pool_name) {
            return Err(SesError::DedicatedIpPoolNotFound(pool_name.to_string()));
        }
        // Upsert the dedicated IP entry
        let entry = self
            .dedicated_ips
            .entry(ip.to_string())
            .or_insert_with(|| DedicatedIp {
                ip: ip.to_string(),
                warmup_status: "DONE".to_string(),
                warmup_percentage: 100,
                pool_name: None,
            });
        entry.pool_name = Some(pool_name.to_string());
        Ok(())
    }

    pub fn put_dedicated_ip_warmup_attributes(
        &mut self,
        ip: &str,
        warmup_percentage: i32,
    ) -> Result<(), SesError> {
        let entry = self
            .dedicated_ips
            .get_mut(ip)
            .ok_or_else(|| SesError::DedicatedIpNotFound(ip.to_string()))?;
        entry.warmup_percentage = warmup_percentage;
        if warmup_percentage >= 100 {
            entry.warmup_status = "DONE".to_string();
        } else {
            entry.warmup_status = "IN_PROGRESS".to_string();
        }
        Ok(())
    }

    // --- Tags ---

    pub fn list_tags_for_resource(&self, arn: &str) -> Vec<(String, String)> {
        let tags = self.get_tags_for_arn(arn);
        tags.into_iter().collect()
    }

    fn get_tags_for_arn(&self, arn: &str) -> HashMap<String, String> {
        if let Some(resource) = arn.split(':').next_back() {
            if let Some((rtype, name)) = resource.split_once('/') {
                match rtype {
                    "identity" => {
                        if let Some(id) = self.identities.get(name) {
                            return id.tags.clone();
                        }
                    }
                    "configuration-set" => {
                        if let Some(cs) = self.configuration_sets.get(name) {
                            return cs.tags.clone();
                        }
                    }
                    "contact-list" => {
                        if let Some(cl) = self.contact_lists.get(name) {
                            return cl.tags.clone();
                        }
                    }
                    "dedicated-ip-pool" => {
                        if let Some(pool) = self.dedicated_ip_pools.get(name) {
                            return pool.tags.clone();
                        }
                    }
                    _ => {}
                }
            }
        }
        HashMap::new()
    }

    pub fn tag_resource(
        &mut self,
        arn: &str,
        new_tags: HashMap<String, String>,
    ) -> Result<(), SesError> {
        if let Some(resource) = arn.split(':').next_back() {
            if let Some((rtype, name)) = resource.split_once('/') {
                match rtype {
                    "identity" => {
                        if let Some(id) = self.identities.get_mut(name) {
                            id.tags.extend(new_tags);
                            return Ok(());
                        }
                    }
                    "configuration-set" => {
                        if let Some(cs) = self.configuration_sets.get_mut(name) {
                            cs.tags.extend(new_tags);
                            return Ok(());
                        }
                    }
                    "contact-list" => {
                        if let Some(cl) = self.contact_lists.get_mut(name) {
                            cl.tags.extend(new_tags);
                            return Ok(());
                        }
                    }
                    "dedicated-ip-pool" => {
                        if let Some(pool) = self.dedicated_ip_pools.get_mut(name) {
                            pool.tags.extend(new_tags);
                            return Ok(());
                        }
                    }
                    _ => {}
                }
            }
        }
        Err(SesError::ResourceNotFound(arn.to_string()))
    }

    pub fn untag_resource(&mut self, arn: &str, tag_keys: &[String]) -> Result<(), SesError> {
        if let Some(resource) = arn.split(':').next_back() {
            if let Some((rtype, name)) = resource.split_once('/') {
                match rtype {
                    "identity" => {
                        if let Some(id) = self.identities.get_mut(name) {
                            for key in tag_keys {
                                id.tags.remove(key);
                            }
                            return Ok(());
                        }
                    }
                    "configuration-set" => {
                        if let Some(cs) = self.configuration_sets.get_mut(name) {
                            for key in tag_keys {
                                cs.tags.remove(key);
                            }
                            return Ok(());
                        }
                    }
                    "contact-list" => {
                        if let Some(cl) = self.contact_lists.get_mut(name) {
                            for key in tag_keys {
                                cl.tags.remove(key);
                            }
                            return Ok(());
                        }
                    }
                    "dedicated-ip-pool" => {
                        if let Some(pool) = self.dedicated_ip_pools.get_mut(name) {
                            for key in tag_keys {
                                pool.tags.remove(key);
                            }
                            return Ok(());
                        }
                    }
                    _ => {}
                }
            }
        }
        Err(SesError::ResourceNotFound(arn.to_string()))
    }

    // --- Email Templates ---

    pub fn create_email_template(
        &mut self,
        template_name: &str,
        subject_part: Option<&str>,
        text_part: Option<&str>,
        html_part: Option<&str>,
    ) -> Result<(), SesError> {
        if self.email_templates.contains_key(template_name) {
            return Err(SesError::EmailTemplateAlreadyExists(
                template_name.to_string(),
            ));
        }
        self.email_templates.insert(
            template_name.to_string(),
            EmailTemplate {
                template_name: template_name.to_string(),
                subject_part: subject_part.map(|s| s.to_string()),
                text_part: text_part.map(|s| s.to_string()),
                html_part: html_part.map(|s| s.to_string()),
                created_timestamp: chrono::Utc::now(),
            },
        );
        Ok(())
    }

    pub fn get_email_template(&self, template_name: &str) -> Result<&EmailTemplate, SesError> {
        self.email_templates
            .get(template_name)
            .ok_or_else(|| SesError::EmailTemplateNotFound(template_name.to_string()))
    }

    pub fn update_email_template(
        &mut self,
        template_name: &str,
        subject_part: Option<&str>,
        text_part: Option<&str>,
        html_part: Option<&str>,
    ) -> Result<(), SesError> {
        let tmpl = self
            .email_templates
            .get_mut(template_name)
            .ok_or_else(|| SesError::EmailTemplateNotFound(template_name.to_string()))?;
        if let Some(s) = subject_part {
            tmpl.subject_part = Some(s.to_string());
        }
        if let Some(s) = text_part {
            tmpl.text_part = Some(s.to_string());
        }
        if let Some(s) = html_part {
            tmpl.html_part = Some(s.to_string());
        }
        Ok(())
    }

    pub fn delete_email_template(&mut self, template_name: &str) -> Result<(), SesError> {
        if self.email_templates.remove(template_name).is_none() {
            return Err(SesError::EmailTemplateNotFound(template_name.to_string()));
        }
        Ok(())
    }

    pub fn list_email_templates(&self) -> Vec<&EmailTemplate> {
        self.email_templates.values().collect()
    }

    // --- Custom Verification Email Templates ---

    pub fn create_custom_verification_email_template(
        &mut self,
        template_name: &str,
        from_email_address: &str,
        template_subject: &str,
        template_content: &str,
        success_redirection_url: &str,
        failure_redirection_url: &str,
    ) -> Result<(), SesError> {
        if self
            .custom_verification_email_templates
            .contains_key(template_name)
        {
            return Err(SesError::CustomVerificationEmailTemplateAlreadyExists(
                template_name.to_string(),
            ));
        }
        self.custom_verification_email_templates.insert(
            template_name.to_string(),
            CustomVerificationEmailTemplate {
                template_name: template_name.to_string(),
                from_email_address: from_email_address.to_string(),
                template_subject: template_subject.to_string(),
                template_content: template_content.to_string(),
                success_redirection_url: success_redirection_url.to_string(),
                failure_redirection_url: failure_redirection_url.to_string(),
                created_timestamp: Utc::now(),
            },
        );
        Ok(())
    }

    pub fn get_custom_verification_email_template(
        &self,
        template_name: &str,
    ) -> Result<&CustomVerificationEmailTemplate, SesError> {
        self.custom_verification_email_templates
            .get(template_name)
            .ok_or_else(|| {
                SesError::CustomVerificationEmailTemplateNotFound(template_name.to_string())
            })
    }

    pub fn update_custom_verification_email_template(
        &mut self,
        template_name: &str,
        from_email_address: Option<&str>,
        template_subject: Option<&str>,
        template_content: Option<&str>,
        success_redirection_url: Option<&str>,
        failure_redirection_url: Option<&str>,
    ) -> Result<(), SesError> {
        let tmpl = self
            .custom_verification_email_templates
            .get_mut(template_name)
            .ok_or_else(|| {
                SesError::CustomVerificationEmailTemplateNotFound(template_name.to_string())
            })?;
        if let Some(s) = from_email_address {
            tmpl.from_email_address = s.to_string();
        }
        if let Some(s) = template_subject {
            tmpl.template_subject = s.to_string();
        }
        if let Some(s) = template_content {
            tmpl.template_content = s.to_string();
        }
        if let Some(s) = success_redirection_url {
            tmpl.success_redirection_url = s.to_string();
        }
        if let Some(s) = failure_redirection_url {
            tmpl.failure_redirection_url = s.to_string();
        }
        Ok(())
    }

    pub fn delete_custom_verification_email_template(
        &mut self,
        template_name: &str,
    ) -> Result<(), SesError> {
        if self
            .custom_verification_email_templates
            .remove(template_name)
            .is_none()
        {
            return Err(SesError::CustomVerificationEmailTemplateNotFound(
                template_name.to_string(),
            ));
        }
        Ok(())
    }

    pub fn list_custom_verification_email_templates(
        &self,
    ) -> Vec<&CustomVerificationEmailTemplate> {
        self.custom_verification_email_templates.values().collect()
    }

    // --- Import Jobs ---

    pub fn create_import_job(
        &mut self,
        import_destination: serde_json::Value,
        import_data_source: serde_json::Value,
    ) -> String {
        let job_id = format!("{}", uuid::Uuid::new_v4());
        self.import_jobs.insert(
            job_id.clone(),
            ImportJob {
                job_id: job_id.clone(),
                import_destination,
                import_data_source,
                job_status: "CREATED".to_string(),
                created_timestamp: chrono::Utc::now(),
            },
        );
        job_id
    }

    pub fn get_import_job(&self, job_id: &str) -> Result<&ImportJob, SesError> {
        self.import_jobs
            .get(job_id)
            .ok_or_else(|| SesError::ImportJobNotFound(job_id.to_string()))
    }

    pub fn list_import_jobs(&self) -> Vec<&ImportJob> {
        self.import_jobs.values().collect()
    }

    // --- Export Jobs ---

    pub fn create_export_job(
        &mut self,
        export_source_type: &str,
        export_destination: serde_json::Value,
    ) -> String {
        let job_id = format!("{}", uuid::Uuid::new_v4());
        self.export_jobs.insert(
            job_id.clone(),
            ExportJob {
                job_id: job_id.clone(),
                export_source_type: export_source_type.to_string(),
                job_status: "CREATED".to_string(),
                created_timestamp: Utc::now(),
                export_destination,
            },
        );
        job_id
    }

    pub fn get_export_job(&self, job_id: &str) -> Result<&ExportJob, SesError> {
        self.export_jobs
            .get(job_id)
            .ok_or_else(|| SesError::ExportJobNotFound(job_id.to_string()))
    }

    pub fn list_export_jobs(&self) -> Vec<&ExportJob> {
        self.export_jobs.values().collect()
    }

    pub fn cancel_export_job(&mut self, job_id: &str) -> Result<(), SesError> {
        let job = self
            .export_jobs
            .get_mut(job_id)
            .ok_or_else(|| SesError::ExportJobNotFound(job_id.to_string()))?;
        job.job_status = "CANCELLED".to_string();
        Ok(())
    }

    // --- Multi-Region Endpoints ---

    pub fn create_multi_region_endpoint(
        &mut self,
        endpoint_name: &str,
        regions: Vec<String>,
    ) -> String {
        let endpoint_id = format!("{}", uuid::Uuid::new_v4());
        let now = Utc::now();
        self.multi_region_endpoints.insert(
            endpoint_name.to_string(),
            MultiRegionEndpoint {
                endpoint_name: endpoint_name.to_string(),
                endpoint_id: endpoint_id.clone(),
                status: "CREATING".to_string(),
                regions,
                created_timestamp: now,
                last_updated_timestamp: now,
            },
        );
        endpoint_id
    }

    pub fn get_multi_region_endpoint(
        &self,
        endpoint_name: &str,
    ) -> Result<&MultiRegionEndpoint, SesError> {
        self.multi_region_endpoints
            .get(endpoint_name)
            .ok_or_else(|| SesError::MultiRegionEndpointNotFound(endpoint_name.to_string()))
    }

    pub fn delete_multi_region_endpoint(&mut self, endpoint_name: &str) -> Result<(), SesError> {
        if self.multi_region_endpoints.remove(endpoint_name).is_none() {
            return Err(SesError::MultiRegionEndpointNotFound(
                endpoint_name.to_string(),
            ));
        }
        Ok(())
    }

    pub fn list_multi_region_endpoints(&self) -> Vec<&MultiRegionEndpoint> {
        self.multi_region_endpoints.values().collect()
    }

    // --- Update Contact List ---

    pub fn update_contact_list(
        &mut self,
        name: &str,
        description: Option<&str>,
        tags: Option<HashMap<String, String>>,
    ) -> Result<(), SesError> {
        let list = self
            .contact_lists
            .get_mut(name)
            .ok_or_else(|| SesError::ContactListNotFound(name.to_string()))?;
        if let Some(d) = description {
            list.description = Some(d.to_string());
        }
        if let Some(t) = tags {
            list.tags = t;
        }
        list.last_updated_timestamp = Utc::now();
        Ok(())
    }

    // --- Update Contact ---

    pub fn update_contact(
        &mut self,
        list_name: &str,
        email_address: &str,
        unsubscribe_all: Option<bool>,
        topic_preferences: Option<Vec<TopicPreference>>,
    ) -> Result<(), SesError> {
        let list = self
            .contact_lists
            .get_mut(list_name)
            .ok_or_else(|| SesError::ContactListNotFound(list_name.to_string()))?;
        let contact = list.contacts.get_mut(email_address).ok_or_else(|| {
            SesError::ContactNotFound(email_address.to_string(), list_name.to_string())
        })?;
        if let Some(u) = unsubscribe_all {
            contact.unsubscribe_all = u;
        }
        if let Some(tp) = topic_preferences {
            contact.topic_preferences = tp;
        }
        contact.last_updated_timestamp = Utc::now();
        Ok(())
    }

    // --- Suppression List ---

    pub fn put_suppressed_destination(&mut self, email_address: &str, reason: &str) {
        let now = Utc::now().timestamp() as f64;
        self.suppression_list.insert(
            email_address.to_string(),
            SuppressedDestinationEntry {
                email_address: email_address.to_string(),
                reason: reason.to_string(),
                last_update_time: now,
            },
        );
    }

    pub fn get_suppressed_destination(
        &self,
        email_address: &str,
    ) -> Result<&SuppressedDestinationEntry, SesError> {
        self.suppression_list
            .get(email_address)
            .ok_or_else(|| SesError::SuppressedDestinationNotFound(email_address.to_string()))
    }

    pub fn delete_suppressed_destination(&mut self, email_address: &str) -> Result<(), SesError> {
        if self.suppression_list.remove(email_address).is_none() {
            return Err(SesError::SuppressedDestinationNotFound(
                email_address.to_string(),
            ));
        }
        Ok(())
    }

    pub fn list_suppressed_destinations(&self) -> Vec<&SuppressedDestinationEntry> {
        self.suppression_list.values().collect()
    }

    // --- Account Settings ---

    pub fn get_account_settings(&self) -> &AccountSettings {
        &self.account_settings
    }

    pub fn put_account_sending_enabled(&mut self, enabled: bool) {
        self.account_settings.sending_enabled = enabled;
    }

    pub fn put_account_details(&mut self, details: Option<serde_json::Value>) {
        self.account_settings.details = details;
    }

    pub fn put_account_dedicated_ip_warmup_attributes(&mut self, auto_warmup_enabled: bool) {
        self.account_settings.dedicated_ip_auto_warmup_enabled = auto_warmup_enabled;
    }

    pub fn put_account_suppression_attributes(&mut self, suppressed_reasons: Vec<String>) {
        self.account_settings.suppressed_reasons = suppressed_reasons;
    }

    pub fn put_account_vdm_attributes(&mut self, vdm_attributes: serde_json::Value) {
        self.account_settings.vdm_attributes = Some(vdm_attributes);
    }

    // --- Deliverability Dashboard ---

    pub fn get_deliverability_dashboard_options(&self) -> &DeliverabilityDashboardOptions {
        &self.deliverability_dashboard
    }

    pub fn put_deliverability_dashboard_option(&mut self, enabled: bool) {
        self.deliverability_dashboard.dashboard_enabled = enabled;
    }

    // --- Bulk Email ---

    pub fn send_bulk_email(&mut self, from: &str, count: usize) -> Vec<String> {
        let _ = from;
        (0..count)
            .map(|_| {
                let id = format!("{}", uuid::Uuid::new_v4());
                id
            })
            .collect()
    }

    // --- Tenants ---

    pub fn create_tenant(
        &mut self,
        tenant_name: &str,
        tags: HashMap<String, String>,
    ) -> Result<String, SesError> {
        if self.tenants.contains_key(tenant_name) {
            return Err(SesError::TenantAlreadyExists(tenant_name.to_string()));
        }
        let tenant_id = format!("{}", uuid::Uuid::new_v4());
        self.tenants.insert(
            tenant_name.to_string(),
            Tenant {
                tenant_name: tenant_name.to_string(),
                tenant_id: tenant_id.clone(),
                tags,
                created_timestamp: Utc::now(),
            },
        );
        Ok(tenant_id)
    }

    pub fn get_tenant(&self, tenant_name: &str) -> Result<&Tenant, SesError> {
        self.tenants
            .get(tenant_name)
            .ok_or_else(|| SesError::TenantNotFound(tenant_name.to_string()))
    }

    pub fn delete_tenant(&mut self, tenant_name: &str) -> Result<(), SesError> {
        if self.tenants.remove(tenant_name).is_none() {
            return Err(SesError::TenantNotFound(tenant_name.to_string()));
        }
        // Remove all resource associations for this tenant
        self.tenant_resource_associations
            .retain(|a| a.tenant_name != tenant_name);
        Ok(())
    }

    pub fn list_tenants(&self) -> Vec<&Tenant> {
        self.tenants.values().collect()
    }

    pub fn create_tenant_resource_association(
        &mut self,
        tenant_name: &str,
        resource_arn: &str,
    ) -> Result<(), SesError> {
        // Ensure tenant exists
        if !self.tenants.contains_key(tenant_name) {
            return Err(SesError::TenantNotFound(tenant_name.to_string()));
        }
        // Check duplicate
        let already_exists = self
            .tenant_resource_associations
            .iter()
            .any(|a| a.tenant_name == tenant_name && a.resource_arn == resource_arn);
        if already_exists {
            return Err(SesError::TenantResourceAssociationAlreadyExists(
                resource_arn.to_string(),
                tenant_name.to_string(),
            ));
        }
        self.tenant_resource_associations
            .push(TenantResourceAssociation {
                tenant_name: tenant_name.to_string(),
                resource_arn: resource_arn.to_string(),
            });
        Ok(())
    }

    pub fn delete_tenant_resource_association(
        &mut self,
        tenant_name: &str,
        resource_arn: &str,
    ) -> Result<(), SesError> {
        let before = self.tenant_resource_associations.len();
        self.tenant_resource_associations
            .retain(|a| !(a.tenant_name == tenant_name && a.resource_arn == resource_arn));
        if self.tenant_resource_associations.len() == before {
            return Err(SesError::TenantResourceAssociationNotFound(
                resource_arn.to_string(),
                tenant_name.to_string(),
            ));
        }
        Ok(())
    }

    pub fn list_tenant_resources(&self, tenant_name: &str) -> Vec<&TenantResourceAssociation> {
        self.tenant_resource_associations
            .iter()
            .filter(|a| a.tenant_name == tenant_name)
            .collect()
    }

    pub fn list_resource_tenants(&self, resource_arn: &str) -> Vec<&TenantResourceAssociation> {
        self.tenant_resource_associations
            .iter()
            .filter(|a| a.resource_arn == resource_arn)
            .collect()
    }

    // --- Deliverability Test Reports ---

    pub fn create_deliverability_test_report(
        &mut self,
        from_email_address: &str,
        report_name: Option<&str>,
    ) -> String {
        let report_id = format!("{}", uuid::Uuid::new_v4());
        let create_date = Utc::now().timestamp() as f64;
        self.deliverability_test_reports.insert(
            report_id.clone(),
            DeliverabilityTestReportRecord {
                report_id: report_id.clone(),
                report_name: report_name.map(|s| s.to_string()),
                from_email_address: from_email_address.to_string(),
                create_date,
                deliverability_test_status: "COMPLETED".to_string(),
            },
        );
        report_id
    }

    pub fn get_deliverability_test_report(
        &self,
        report_id: &str,
    ) -> Result<&DeliverabilityTestReportRecord, SesError> {
        self.deliverability_test_reports
            .get(report_id)
            .ok_or_else(|| SesError::DeliverabilityTestReportNotFound(report_id.to_string()))
    }

    pub fn list_deliverability_test_reports(&self) -> Vec<&DeliverabilityTestReportRecord> {
        self.deliverability_test_reports.values().collect()
    }

    // --- Reputation Entities ---

    pub fn get_reputation_entity(
        &self,
        entity_type: &str,
        entity_reference: &str,
    ) -> Result<&ReputationEntityRecord, SesError> {
        let key = format!("{}:{}", entity_type, entity_reference);
        self.reputation_entities.get(&key).ok_or_else(|| {
            SesError::ReputationEntityNotFound(
                entity_type.to_string(),
                entity_reference.to_string(),
            )
        })
    }

    pub fn list_reputation_entities(&self) -> Vec<&ReputationEntityRecord> {
        self.reputation_entities.values().collect()
    }

    /// Upsert a reputation entity's customer-managed sending status.
    pub fn update_reputation_entity_customer_managed_status(
        &mut self,
        entity_type: &str,
        entity_reference: &str,
        sending_status: &str,
    ) {
        let key = format!("{}:{}", entity_type, entity_reference);
        let entry = self
            .reputation_entities
            .entry(key)
            .or_insert_with(|| ReputationEntityRecord {
                entity_type: entity_type.to_string(),
                entity_reference: entity_reference.to_string(),
                customer_managed_sending_status: "ENABLED".to_string(),
                policy: None,
            });
        entry.customer_managed_sending_status = sending_status.to_string();
    }

    /// Upsert a reputation entity's policy.
    pub fn update_reputation_entity_policy(
        &mut self,
        entity_type: &str,
        entity_reference: &str,
        policy: Option<&str>,
    ) {
        let key = format!("{}:{}", entity_type, entity_reference);
        let entry = self
            .reputation_entities
            .entry(key)
            .or_insert_with(|| ReputationEntityRecord {
                entity_type: entity_type.to_string(),
                entity_reference: entity_reference.to_string(),
                customer_managed_sending_status: "ENABLED".to_string(),
                policy: None,
            });
        entry.policy = policy.map(|s| s.to_string());
    }

    // --- Send Custom Verification Email ---

    pub fn send_custom_verification_email(
        &mut self,
        email_address: &str,
        template_name: &str,
    ) -> Result<String, SesError> {
        // Verify the template exists.
        self.custom_verification_email_templates
            .get(template_name)
            .ok_or_else(|| {
                SesError::CustomVerificationEmailTemplateNotFound(template_name.to_string())
            })?;
        let message_id = format!("{}", uuid::Uuid::new_v4());
        let now = Utc::now();
        self.sent_emails.push(SentEmail {
            from: format!(
                "verification@{}",
                email_address.split('@').nth(1).unwrap_or("example.com")
            ),
            to: vec![email_address.to_string()],
            subject: "Please verify your email address".to_string(),
            body: format!(
                "Custom verification email sent via template '{}'",
                template_name
            ),
            timestamp: now,
            message_id: message_id.clone(),
        });
        Ok(message_id)
    }

    // --- Test Render Email Template ---

    pub fn test_render_email_template(
        &self,
        template_name: &str,
        _template_data: &str,
    ) -> Result<String, SesError> {
        let tmpl = self
            .email_templates
            .get(template_name)
            .ok_or_else(|| SesError::EmailTemplateNotFound(template_name.to_string()))?;
        // Simple mock render: return the HTML part if available, otherwise text part, otherwise subject.
        let rendered = tmpl
            .html_part
            .clone()
            .or_else(|| tmpl.text_part.clone())
            .or_else(|| tmpl.subject_part.clone())
            .unwrap_or_default();
        Ok(rendered)
    }

    // --- Message Insights ---

    pub fn get_message_insights(&self, message_id: &str) -> Option<&SentEmail> {
        self.sent_emails.iter().find(|e| e.message_id == message_id)
    }
}
