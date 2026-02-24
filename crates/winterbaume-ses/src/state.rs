//! In-memory state for the SES v1 service.

use std::collections::HashMap;

use chrono::Utc;

use crate::types::{
    ConfigurationSet, EventDestination, Identity, IdentityType, ReceiptRule, ReceiptRuleSet,
    SentEmail, Template, VerificationStatus,
};

#[derive(Debug, Default)]
pub struct SesV1State {
    /// Identities keyed by identity name (email or domain).
    pub identities: HashMap<String, Identity>,
    /// Configuration sets keyed by name.
    pub configuration_sets: HashMap<String, ConfigurationSet>,
    /// Receipt rule sets keyed by name.
    pub receipt_rule_sets: HashMap<String, ReceiptRuleSet>,
    /// Active receipt rule set name.
    pub active_receipt_rule_set: Option<String>,
    /// Templates keyed by name.
    pub templates: HashMap<String, Template>,
    /// Emails recorded by `SendEmail` (newest at the end).
    pub sent_emails: Vec<SentEmail>,
}

#[derive(Debug, thiserror::Error)]
pub enum SesV1Error {
    #[error("{resource} does not exist")]
    NotFound { resource: String },

    #[error("{name} already exists")]
    AlreadyExists { name: String },
}

impl SesV1Error {
    pub fn not_found(resource: &str) -> Self {
        Self::NotFound {
            resource: resource.to_string(),
        }
    }

    pub fn already_exists(name: &str) -> Self {
        Self::AlreadyExists {
            name: name.to_string(),
        }
    }
}

impl SesV1State {
    // --- Identities ---

    pub fn verify_email_address(&mut self, email: &str) {
        let identity = self
            .identities
            .entry(email.to_string())
            .or_insert_with(|| Identity {
                name: email.to_string(),
                identity_type: IdentityType::EmailAddress,
                verification_status: VerificationStatus::Success,
                verification_token: None,
                dkim_tokens: vec![],
                dkim_enabled: false,
                mail_from_domain: None,
                bounce_topic: None,
                complaint_topic: None,
                delivery_topic: None,
                forwarding_enabled: true,
            });
        identity.verification_status = VerificationStatus::Success;
    }

    pub fn verify_email_identity(&mut self, email: &str) {
        self.verify_email_address(email);
    }

    pub fn verify_domain_identity(&mut self, domain: &str) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        domain.hash(&mut hasher);
        let token = format!("{:x}", hasher.finish());
        let token_str = format!("{token}{token}");
        self.identities
            .entry(domain.to_string())
            .or_insert_with(|| Identity {
                name: domain.to_string(),
                identity_type: IdentityType::Domain,
                verification_status: VerificationStatus::Success,
                verification_token: Some(token_str.clone()),
                dkim_tokens: vec![
                    format!("dkim1.{domain}"),
                    format!("dkim2.{domain}"),
                    format!("dkim3.{domain}"),
                ],
                dkim_enabled: true,
                mail_from_domain: None,
                bounce_topic: None,
                complaint_topic: None,
                delivery_topic: None,
                forwarding_enabled: true,
            });
        token_str
    }

    pub fn delete_identity(&mut self, identity: &str) -> Result<(), SesV1Error> {
        self.identities.remove(identity);
        Ok(())
    }

    pub fn list_identities(&self, identity_type: Option<&str>) -> Vec<String> {
        self.identities
            .values()
            .filter(|id| match identity_type {
                Some("EmailAddress") => id.identity_type == IdentityType::EmailAddress,
                Some("Domain") => id.identity_type == IdentityType::Domain,
                _ => true,
            })
            .map(|id| id.name.clone())
            .collect()
    }

    pub fn list_verified_email_addresses(&self) -> Vec<String> {
        self.identities
            .values()
            .filter(|id| {
                id.identity_type == IdentityType::EmailAddress
                    && id.verification_status == VerificationStatus::Success
            })
            .map(|id| id.name.clone())
            .collect()
    }

    pub fn get_identity_verification_attributes(
        &self,
        identities: &[String],
    ) -> HashMap<String, (String, Option<String>)> {
        let mut result = HashMap::new();
        for name in identities {
            if let Some(id) = self.identities.get(name) {
                result.insert(
                    name.clone(),
                    (
                        id.verification_status.as_str().to_string(),
                        id.verification_token.clone(),
                    ),
                );
            } else {
                result.insert(name.clone(), ("NotStarted".to_string(), None));
            }
        }
        result
    }

    pub fn set_identity_feedback_forwarding_enabled(
        &mut self,
        identity: &str,
        enabled: bool,
    ) -> Result<(), SesV1Error> {
        let id = self
            .identities
            .get_mut(identity)
            .ok_or_else(|| SesV1Error::not_found(identity))?;
        id.forwarding_enabled = enabled;
        Ok(())
    }

    pub fn set_identity_mail_from_domain(
        &mut self,
        identity: &str,
        mail_from_domain: Option<String>,
    ) -> Result<(), SesV1Error> {
        // Create if not exists (SES allows setting mail-from on unverified identities)
        let id = self
            .identities
            .entry(identity.to_string())
            .or_insert_with(|| Identity {
                name: identity.to_string(),
                identity_type: if identity.contains('@') {
                    IdentityType::EmailAddress
                } else {
                    IdentityType::Domain
                },
                verification_status: VerificationStatus::NotStarted,
                verification_token: None,
                dkim_tokens: vec![],
                dkim_enabled: false,
                mail_from_domain: None,
                bounce_topic: None,
                complaint_topic: None,
                delivery_topic: None,
                forwarding_enabled: true,
            });
        id.mail_from_domain = mail_from_domain;
        Ok(())
    }

    pub fn set_identity_notification_topic(
        &mut self,
        identity: &str,
        notification_type: &str,
        sns_topic: Option<String>,
    ) -> Result<(), SesV1Error> {
        let id = self
            .identities
            .entry(identity.to_string())
            .or_insert_with(|| Identity {
                name: identity.to_string(),
                identity_type: if identity.contains('@') {
                    IdentityType::EmailAddress
                } else {
                    IdentityType::Domain
                },
                verification_status: VerificationStatus::NotStarted,
                verification_token: None,
                dkim_tokens: vec![],
                dkim_enabled: false,
                mail_from_domain: None,
                bounce_topic: None,
                complaint_topic: None,
                delivery_topic: None,
                forwarding_enabled: true,
            });
        match notification_type {
            "Bounce" => id.bounce_topic = sns_topic,
            "Complaint" => id.complaint_topic = sns_topic,
            "Delivery" => id.delivery_topic = sns_topic,
            _ => {}
        }
        Ok(())
    }

    // --- Configuration Sets ---

    pub fn create_configuration_set(&mut self, name: &str) -> Result<(), SesV1Error> {
        if self.configuration_sets.contains_key(name) {
            return Err(SesV1Error::already_exists(name));
        }
        self.configuration_sets.insert(
            name.to_string(),
            ConfigurationSet {
                name: name.to_string(),
                event_destinations: vec![],
                created_at: Utc::now(),
            },
        );
        Ok(())
    }

    pub fn delete_configuration_set(&mut self, name: &str) -> Result<(), SesV1Error> {
        if self.configuration_sets.remove(name).is_none() {
            return Err(SesV1Error::not_found(name));
        }
        Ok(())
    }

    pub fn describe_configuration_set(&self, name: &str) -> Result<&ConfigurationSet, SesV1Error> {
        self.configuration_sets
            .get(name)
            .ok_or_else(|| SesV1Error::not_found(name))
    }

    pub fn list_configuration_sets(&self) -> Vec<String> {
        self.configuration_sets.keys().cloned().collect()
    }

    pub fn create_configuration_set_event_destination(
        &mut self,
        config_set_name: &str,
        destination: EventDestination,
    ) -> Result<(), SesV1Error> {
        let cs = self
            .configuration_sets
            .get_mut(config_set_name)
            .ok_or_else(|| SesV1Error::not_found(config_set_name))?;
        cs.event_destinations.push(destination);
        Ok(())
    }

    // --- Receipt Rule Sets ---

    pub fn create_receipt_rule_set(&mut self, name: &str) -> Result<(), SesV1Error> {
        if self.receipt_rule_sets.contains_key(name) {
            return Err(SesV1Error::already_exists(name));
        }
        self.receipt_rule_sets.insert(
            name.to_string(),
            ReceiptRuleSet {
                name: name.to_string(),
                rules: vec![],
                created_at: Utc::now(),
            },
        );
        Ok(())
    }

    pub fn delete_receipt_rule_set(&mut self, name: &str) -> Result<(), SesV1Error> {
        if self.receipt_rule_sets.remove(name).is_none() {
            return Err(SesV1Error::not_found(name));
        }
        if self.active_receipt_rule_set.as_deref() == Some(name) {
            self.active_receipt_rule_set = None;
        }
        Ok(())
    }

    pub fn clone_receipt_rule_set(
        &mut self,
        rule_set_name: &str,
        original_rule_set_name: &str,
    ) -> Result<(), SesV1Error> {
        if self.receipt_rule_sets.contains_key(rule_set_name) {
            return Err(SesV1Error::already_exists(rule_set_name));
        }
        let original = self
            .receipt_rule_sets
            .get(original_rule_set_name)
            .ok_or_else(|| SesV1Error::not_found(original_rule_set_name))?
            .clone();
        self.receipt_rule_sets.insert(
            rule_set_name.to_string(),
            ReceiptRuleSet {
                name: rule_set_name.to_string(),
                rules: original.rules,
                created_at: Utc::now(),
            },
        );
        Ok(())
    }

    pub fn set_active_receipt_rule_set(&mut self, name: Option<&str>) -> Result<(), SesV1Error> {
        if let Some(name) = name {
            if !self.receipt_rule_sets.contains_key(name) {
                return Err(SesV1Error::not_found(name));
            }
            self.active_receipt_rule_set = Some(name.to_string());
        } else {
            self.active_receipt_rule_set = None;
        }
        Ok(())
    }

    pub fn describe_active_receipt_rule_set(&self) -> Option<(&str, &ReceiptRuleSet)> {
        self.active_receipt_rule_set.as_ref().and_then(|name| {
            self.receipt_rule_sets
                .get(name)
                .map(|rs| (name.as_str(), rs))
        })
    }

    pub fn describe_receipt_rule_set(&self, name: &str) -> Result<&ReceiptRuleSet, SesV1Error> {
        self.receipt_rule_sets
            .get(name)
            .ok_or_else(|| SesV1Error::not_found(name))
    }

    pub fn list_receipt_rule_sets(&self) -> Vec<&ReceiptRuleSet> {
        self.receipt_rule_sets.values().collect()
    }

    pub fn create_receipt_rule(
        &mut self,
        rule_set_name: &str,
        rule: ReceiptRule,
    ) -> Result<(), SesV1Error> {
        let rs = self
            .receipt_rule_sets
            .get_mut(rule_set_name)
            .ok_or_else(|| SesV1Error::not_found(rule_set_name))?;
        rs.rules.push(rule);
        Ok(())
    }

    pub fn describe_receipt_rule(
        &self,
        rule_set_name: &str,
        rule_name: &str,
    ) -> Result<&ReceiptRule, SesV1Error> {
        let rs = self
            .receipt_rule_sets
            .get(rule_set_name)
            .ok_or_else(|| SesV1Error::not_found(rule_set_name))?;
        rs.rules
            .iter()
            .find(|r| r.name == rule_name)
            .ok_or_else(|| SesV1Error::not_found(rule_name))
    }

    // --- Templates ---

    pub fn create_template(&mut self, template: Template) -> Result<(), SesV1Error> {
        if self.templates.contains_key(&template.name) {
            return Err(SesV1Error::already_exists(&template.name.clone()));
        }
        self.templates.insert(template.name.clone(), template);
        Ok(())
    }

    pub fn delete_template(&mut self, name: &str) -> Result<(), SesV1Error> {
        if self.templates.remove(name).is_none() {
            return Err(SesV1Error::not_found(name));
        }
        Ok(())
    }

    pub fn get_template(&self, name: &str) -> Result<&Template, SesV1Error> {
        self.templates
            .get(name)
            .ok_or_else(|| SesV1Error::not_found(name))
    }

    pub fn list_templates(&self) -> Vec<&Template> {
        self.templates.values().collect()
    }

    pub fn update_template(&mut self, template: Template) -> Result<(), SesV1Error> {
        if !self.templates.contains_key(&template.name) {
            return Err(SesV1Error::not_found(&template.name.clone()));
        }
        self.templates.insert(template.name.clone(), template);
        Ok(())
    }

    pub fn update_receipt_rule(
        &mut self,
        rule_set_name: &str,
        rule: ReceiptRule,
    ) -> Result<(), SesV1Error> {
        let rs = self
            .receipt_rule_sets
            .get_mut(rule_set_name)
            .ok_or_else(|| SesV1Error::not_found(rule_set_name))?;
        let pos = rs
            .rules
            .iter()
            .position(|r| r.name == rule.name)
            .ok_or_else(|| SesV1Error::not_found(&rule.name.clone()))?;
        rs.rules[pos] = rule;
        Ok(())
    }

    // --- Sent emails ---

    /// Record a `SendEmail` invocation and return the assigned message ID.
    #[allow(clippy::too_many_arguments)]
    pub fn record_sent_email(
        &mut self,
        source: String,
        to_addresses: Vec<String>,
        cc_addresses: Vec<String>,
        bcc_addresses: Vec<String>,
        subject: String,
        text_body: Option<String>,
        html_body: Option<String>,
    ) -> String {
        let message_id = format!(
            "01000000000000000000000000000000-{}-000000",
            uuid::Uuid::new_v4()
        );
        self.sent_emails.push(SentEmail {
            message_id: message_id.clone(),
            source,
            to_addresses,
            cc_addresses,
            bcc_addresses,
            subject,
            text_body,
            html_body,
            timestamp: Utc::now(),
        });
        message_id
    }
}
