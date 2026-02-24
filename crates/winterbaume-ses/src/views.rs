//! Serde-compatible view types for SES v1 state snapshots.

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::SesService;
use crate::state::SesV1State;
use crate::types::{
    ConfigurationSet, EventDestination, Identity, IdentityType, ReceiptRule, ReceiptRuleSet,
    SentEmail, Template, VerificationStatus,
};

// ---------------------------------------------------------------------------
// View types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SesV1StateView {
    #[serde(default)]
    pub identities: HashMap<String, IdentityView>,
    #[serde(default)]
    pub configuration_sets: HashMap<String, ConfigurationSetView>,
    #[serde(default)]
    pub receipt_rule_sets: HashMap<String, ReceiptRuleSetView>,
    #[serde(default)]
    pub active_receipt_rule_set: Option<String>,
    #[serde(default)]
    pub templates: HashMap<String, TemplateView>,
    #[serde(default)]
    pub sent_emails: Vec<SentEmailView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityView {
    pub name: String,
    pub identity_type: String,
    pub verification_status: String,
    pub verification_token: Option<String>,
    pub dkim_tokens: Vec<String>,
    pub dkim_enabled: bool,
    pub mail_from_domain: Option<String>,
    pub bounce_topic: Option<String>,
    pub complaint_topic: Option<String>,
    pub delivery_topic: Option<String>,
    pub forwarding_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigurationSetView {
    pub name: String,
    pub event_destinations: Vec<EventDestinationView>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventDestinationView {
    pub name: String,
    pub enabled: bool,
    pub matching_event_types: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceiptRuleSetView {
    pub name: String,
    pub rules: Vec<ReceiptRuleView>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceiptRuleView {
    pub name: String,
    pub enabled: bool,
    pub scan_enabled: bool,
    pub tls_policy: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateView {
    pub name: String,
    pub subject_part: Option<String>,
    pub html_part: Option<String>,
    pub text_part: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentEmailView {
    pub message_id: String,
    pub source: String,
    #[serde(default)]
    pub to_addresses: Vec<String>,
    #[serde(default)]
    pub cc_addresses: Vec<String>,
    #[serde(default)]
    pub bcc_addresses: Vec<String>,
    pub subject: String,
    pub text_body: Option<String>,
    pub html_body: Option<String>,
    pub timestamp: DateTime<Utc>,
}

// ---------------------------------------------------------------------------
// Conversions: State -> View
// ---------------------------------------------------------------------------

impl From<&SesV1State> for SesV1StateView {
    fn from(state: &SesV1State) -> Self {
        Self {
            identities: state
                .identities
                .iter()
                .map(|(k, v)| (k.clone(), IdentityView::from(v)))
                .collect(),
            configuration_sets: state
                .configuration_sets
                .iter()
                .map(|(k, v)| (k.clone(), ConfigurationSetView::from(v)))
                .collect(),
            receipt_rule_sets: state
                .receipt_rule_sets
                .iter()
                .map(|(k, v)| (k.clone(), ReceiptRuleSetView::from(v)))
                .collect(),
            active_receipt_rule_set: state.active_receipt_rule_set.clone(),
            templates: state
                .templates
                .iter()
                .map(|(k, v)| (k.clone(), TemplateView::from(v)))
                .collect(),
            sent_emails: state.sent_emails.iter().map(SentEmailView::from).collect(),
        }
    }
}

impl From<&Identity> for IdentityView {
    fn from(id: &Identity) -> Self {
        Self {
            name: id.name.clone(),
            identity_type: match id.identity_type {
                IdentityType::EmailAddress => "EmailAddress".to_string(),
                IdentityType::Domain => "Domain".to_string(),
            },
            verification_status: id.verification_status.as_str().to_string(),
            verification_token: id.verification_token.clone(),
            dkim_tokens: id.dkim_tokens.clone(),
            dkim_enabled: id.dkim_enabled,
            mail_from_domain: id.mail_from_domain.clone(),
            bounce_topic: id.bounce_topic.clone(),
            complaint_topic: id.complaint_topic.clone(),
            delivery_topic: id.delivery_topic.clone(),
            forwarding_enabled: id.forwarding_enabled,
        }
    }
}

impl From<&ConfigurationSet> for ConfigurationSetView {
    fn from(cs: &ConfigurationSet) -> Self {
        Self {
            name: cs.name.clone(),
            event_destinations: cs
                .event_destinations
                .iter()
                .map(EventDestinationView::from)
                .collect(),
            created_at: cs.created_at,
        }
    }
}

impl From<&EventDestination> for EventDestinationView {
    fn from(ed: &EventDestination) -> Self {
        Self {
            name: ed.name.clone(),
            enabled: ed.enabled,
            matching_event_types: ed.matching_event_types.clone(),
        }
    }
}

impl From<&ReceiptRuleSet> for ReceiptRuleSetView {
    fn from(rs: &ReceiptRuleSet) -> Self {
        Self {
            name: rs.name.clone(),
            rules: rs.rules.iter().map(ReceiptRuleView::from).collect(),
            created_at: rs.created_at,
        }
    }
}

impl From<&ReceiptRule> for ReceiptRuleView {
    fn from(r: &ReceiptRule) -> Self {
        Self {
            name: r.name.clone(),
            enabled: r.enabled,
            scan_enabled: r.scan_enabled,
            tls_policy: r.tls_policy.clone(),
        }
    }
}

impl From<&Template> for TemplateView {
    fn from(t: &Template) -> Self {
        Self {
            name: t.name.clone(),
            subject_part: t.subject_part.clone(),
            html_part: t.html_part.clone(),
            text_part: t.text_part.clone(),
            created_at: t.created_at,
        }
    }
}

impl From<&SentEmail> for SentEmailView {
    fn from(e: &SentEmail) -> Self {
        Self {
            message_id: e.message_id.clone(),
            source: e.source.clone(),
            to_addresses: e.to_addresses.clone(),
            cc_addresses: e.cc_addresses.clone(),
            bcc_addresses: e.bcc_addresses.clone(),
            subject: e.subject.clone(),
            text_body: e.text_body.clone(),
            html_body: e.html_body.clone(),
            timestamp: e.timestamp,
        }
    }
}

// ---------------------------------------------------------------------------
// Conversions: View -> State
// ---------------------------------------------------------------------------

impl From<SesV1StateView> for SesV1State {
    fn from(view: SesV1StateView) -> Self {
        Self {
            identities: view
                .identities
                .into_iter()
                .map(|(k, v)| (k, Identity::from(v)))
                .collect(),
            configuration_sets: view
                .configuration_sets
                .into_iter()
                .map(|(k, v)| (k, ConfigurationSet::from(v)))
                .collect(),
            receipt_rule_sets: view
                .receipt_rule_sets
                .into_iter()
                .map(|(k, v)| (k, ReceiptRuleSet::from(v)))
                .collect(),
            active_receipt_rule_set: view.active_receipt_rule_set,
            templates: view
                .templates
                .into_iter()
                .map(|(k, v)| (k, Template::from(v)))
                .collect(),
            sent_emails: view.sent_emails.into_iter().map(SentEmail::from).collect(),
        }
    }
}

fn verification_status_from_str(s: &str) -> VerificationStatus {
    match s {
        "Success" => VerificationStatus::Success,
        "Failed" => VerificationStatus::Failed,
        "TemporaryFailure" => VerificationStatus::TemporaryFailure,
        "Pending" => VerificationStatus::Pending,
        _ => VerificationStatus::NotStarted,
    }
}

impl From<IdentityView> for Identity {
    fn from(v: IdentityView) -> Self {
        Self {
            name: v.name.clone(),
            identity_type: if v.identity_type == "Domain" {
                IdentityType::Domain
            } else {
                IdentityType::EmailAddress
            },
            verification_status: verification_status_from_str(&v.verification_status),
            verification_token: v.verification_token,
            dkim_tokens: v.dkim_tokens,
            dkim_enabled: v.dkim_enabled,
            mail_from_domain: v.mail_from_domain,
            bounce_topic: v.bounce_topic,
            complaint_topic: v.complaint_topic,
            delivery_topic: v.delivery_topic,
            forwarding_enabled: v.forwarding_enabled,
        }
    }
}

impl From<ConfigurationSetView> for ConfigurationSet {
    fn from(v: ConfigurationSetView) -> Self {
        Self {
            name: v.name,
            event_destinations: v
                .event_destinations
                .into_iter()
                .map(EventDestination::from)
                .collect(),
            created_at: v.created_at,
        }
    }
}

impl From<EventDestinationView> for EventDestination {
    fn from(v: EventDestinationView) -> Self {
        Self {
            name: v.name,
            enabled: v.enabled,
            matching_event_types: v.matching_event_types,
        }
    }
}

impl From<ReceiptRuleSetView> for ReceiptRuleSet {
    fn from(v: ReceiptRuleSetView) -> Self {
        Self {
            name: v.name,
            rules: v.rules.into_iter().map(ReceiptRule::from).collect(),
            created_at: v.created_at,
        }
    }
}

impl From<ReceiptRuleView> for ReceiptRule {
    fn from(v: ReceiptRuleView) -> Self {
        Self {
            name: v.name,
            enabled: v.enabled,
            scan_enabled: v.scan_enabled,
            tls_policy: v.tls_policy,
        }
    }
}

impl From<TemplateView> for Template {
    fn from(v: TemplateView) -> Self {
        Self {
            name: v.name,
            subject_part: v.subject_part,
            html_part: v.html_part,
            text_part: v.text_part,
            created_at: v.created_at,
        }
    }
}

impl From<SentEmailView> for SentEmail {
    fn from(v: SentEmailView) -> Self {
        Self {
            message_id: v.message_id,
            source: v.source,
            to_addresses: v.to_addresses,
            cc_addresses: v.cc_addresses,
            bcc_addresses: v.bcc_addresses,
            subject: v.subject,
            text_body: v.text_body,
            html_body: v.html_body,
            timestamp: v.timestamp,
        }
    }
}

// ---------------------------------------------------------------------------
// StatefulService impl
// ---------------------------------------------------------------------------

impl StatefulService for SesService {
    type StateView = SesV1StateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        SesV1StateView::from(&*guard)
    }

    async fn restore(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), StateViewError> {
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;
            *guard = SesV1State::from(view);
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    async fn merge(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), StateViewError> {
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;
            for (k, v) in view.identities {
                guard.identities.insert(k, Identity::from(v));
            }
            for (k, v) in view.configuration_sets {
                guard
                    .configuration_sets
                    .insert(k, ConfigurationSet::from(v));
            }
            for (k, v) in view.receipt_rule_sets {
                guard.receipt_rule_sets.insert(k, ReceiptRuleSet::from(v));
            }
            if view.active_receipt_rule_set.is_some() {
                guard.active_receipt_rule_set = view.active_receipt_rule_set;
            }
            for (k, v) in view.templates {
                guard.templates.insert(k, Template::from(v));
            }
            for v in view.sent_emails {
                guard.sent_emails.push(SentEmail::from(v));
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
