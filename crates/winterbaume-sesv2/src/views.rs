//! Serde-compatible view types for SES state snapshots.

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::SesV2Service;
use crate::state::SesState;
use crate::types::{
    AccountSettings, ConfigurationSet, Contact, ContactList, CustomVerificationEmailTemplate,
    DedicatedIp, DedicatedIpPool, DeliverabilityDashboardOptions, DeliverabilityTestReportRecord,
    EmailIdentity, EmailTemplate, EventDestination, ExportJob, ImportJob, MultiRegionEndpoint,
    ReputationEntityRecord, SendQuota, SentEmail, SuppressedDestinationEntry, Tenant,
    TenantResourceAssociation, TopicPreference,
};

/// Serializable view of the entire SES state for one account/region.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SesStateView {
    /// Email identities keyed by identity name.
    #[serde(default)]
    pub identities: HashMap<String, EmailIdentityView>,
    /// Configuration sets keyed by name.
    #[serde(default)]
    pub configuration_sets: HashMap<String, ConfigurationSetView>,
    /// Contact lists keyed by name.
    #[serde(default)]
    pub contact_lists: HashMap<String, ContactListView>,
    /// Dedicated IP pools keyed by pool name.
    #[serde(default)]
    pub dedicated_ip_pools: HashMap<String, DedicatedIpPoolView>,
    /// Dedicated IPs keyed by IP address.
    #[serde(default)]
    pub dedicated_ips: HashMap<String, DedicatedIpView>,
    /// Custom verification email templates keyed by template name.
    #[serde(default)]
    pub custom_verification_email_templates: HashMap<String, CustomVerificationEmailTemplateView>,
    /// Import jobs keyed by job ID.
    #[serde(default)]
    pub import_jobs: HashMap<String, ImportJobView>,
    /// Export jobs keyed by job ID.
    #[serde(default)]
    pub export_jobs: HashMap<String, ExportJobView>,
    /// Multi-region endpoints keyed by endpoint name.
    #[serde(default)]
    pub multi_region_endpoints: HashMap<String, MultiRegionEndpointView>,
    /// Tenants keyed by tenant name.
    #[serde(default)]
    pub tenants: HashMap<String, TenantView>,
    /// Tenant resource associations.
    #[serde(default)]
    pub tenant_resource_associations: Vec<TenantResourceAssociationView>,
    /// Email templates keyed by template name.
    #[serde(default)]
    pub email_templates: HashMap<String, EmailTemplateView>,
    /// Account-level settings.
    #[serde(default)]
    pub account_settings: AccountSettingsView,
    /// Deliverability dashboard options.
    #[serde(default)]
    pub deliverability_dashboard: DeliverabilityDashboardView,
    /// Account-level suppression list keyed by email address.
    #[serde(default)]
    pub suppression_list: HashMap<String, SuppressedDestinationView>,
    /// Deliverability test reports keyed by report ID.
    #[serde(default)]
    pub deliverability_test_reports: HashMap<String, DeliverabilityTestReportView>,
    /// Reputation entities keyed by "{entity_type}:{entity_reference}".
    #[serde(default)]
    pub reputation_entities: HashMap<String, ReputationEntityView>,
    /// Emails recorded by `SendEmail` (newest at the end).
    #[serde(default)]
    pub sent_emails: Vec<SentEmailView>,
}

/// Serializable view of an email identity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailIdentityView {
    pub name: String,
    pub identity_type: String,
    pub verified: bool,
    pub created_timestamp: Option<String>,
    #[serde(default)]
    pub policies: HashMap<String, String>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
    pub configuration_set_name: Option<String>,
    #[serde(default)]
    pub dkim_signing_enabled: bool,
    pub dkim_signing_key_type: Option<String>,
    pub dkim_domain: Option<String>,
    #[serde(default = "default_true")]
    pub feedback_forwarding_enabled: bool,
    pub mail_from_domain: Option<String>,
    pub behavior_on_mx_failure: Option<String>,
}

fn default_true() -> bool {
    true
}

/// Serializable view of a configuration set.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigurationSetView {
    pub name: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
    #[serde(default)]
    pub event_destinations: HashMap<String, EventDestinationView>,
    pub archiving_options: Option<serde_json::Value>,
    pub delivery_options: Option<serde_json::Value>,
    pub reputation_options: Option<serde_json::Value>,
    pub sending_options: Option<serde_json::Value>,
    pub suppression_options: Option<serde_json::Value>,
    pub tracking_options: Option<serde_json::Value>,
    pub vdm_options: Option<serde_json::Value>,
}

/// Serializable view of an event destination.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventDestinationView {
    pub name: String,
    pub enabled: bool,
    #[serde(default)]
    pub matching_event_types: Vec<String>,
    pub destination: serde_json::Value,
}

/// Serializable view of a contact list.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactListView {
    pub name: String,
    pub description: Option<String>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
    pub created_timestamp: Option<String>,
    pub last_updated_timestamp: Option<String>,
    #[serde(default)]
    pub contacts: HashMap<String, ContactView>,
}

/// Serializable view of a contact within a contact list.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactView {
    pub email_address: String,
    #[serde(default)]
    pub topic_preferences: Vec<TopicPreferenceView>,
    pub unsubscribe_all: bool,
    pub created_timestamp: Option<String>,
    pub last_updated_timestamp: Option<String>,
}

/// Serializable view of a topic preference.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopicPreferenceView {
    pub topic_name: String,
    pub subscription_status: String,
}

/// Serializable view of a dedicated IP pool.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DedicatedIpPoolView {
    pub pool_name: String,
    pub scaling_mode: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

/// Serializable view of a dedicated IP address.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DedicatedIpView {
    pub ip: String,
    pub warmup_status: String,
    pub warmup_percentage: i32,
    pub pool_name: Option<String>,
}

/// Serializable view of a custom verification email template.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomVerificationEmailTemplateView {
    pub template_name: String,
    pub from_email_address: String,
    pub template_subject: String,
    pub template_content: String,
    pub success_redirection_url: String,
    pub failure_redirection_url: String,
    pub created_timestamp: Option<String>,
}

/// Serializable view of an import job.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportJobView {
    pub job_id: String,
    pub import_destination: serde_json::Value,
    pub import_data_source: serde_json::Value,
    pub job_status: String,
    pub created_timestamp: Option<String>,
}

/// Serializable view of an export job.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportJobView {
    pub job_id: String,
    pub export_source_type: String,
    pub job_status: String,
    pub created_timestamp: Option<String>,
    pub export_destination: serde_json::Value,
}

/// Serializable view of a multi-region endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiRegionEndpointView {
    pub endpoint_name: String,
    pub endpoint_id: String,
    pub status: String,
    #[serde(default)]
    pub regions: Vec<String>,
    pub created_timestamp: Option<String>,
    pub last_updated_timestamp: Option<String>,
}

/// Serializable view of a tenant.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantView {
    pub tenant_name: String,
    pub tenant_id: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
    pub created_timestamp: Option<String>,
}

/// Serializable view of a tenant resource association.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantResourceAssociationView {
    pub tenant_name: String,
    pub resource_arn: String,
}

/// Serializable view of an email template.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailTemplateView {
    pub template_name: String,
    pub subject_part: Option<String>,
    pub text_part: Option<String>,
    pub html_part: Option<String>,
    pub created_timestamp: Option<String>,
}

/// Serializable view of account-level send quota.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendQuotaView {
    pub max_24_hour_send: f64,
    pub max_send_rate: f64,
    pub sent_last_24_hours: f64,
}

impl Default for SendQuotaView {
    fn default() -> Self {
        Self {
            max_24_hour_send: 200.0,
            max_send_rate: 1.0,
            sent_last_24_hours: 0.0,
        }
    }
}

/// Serializable view of account-level settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountSettingsView {
    #[serde(default = "default_true")]
    pub sending_enabled: bool,
    #[serde(default = "default_true")]
    pub production_access_enabled: bool,
    #[serde(default)]
    pub send_quota: SendQuotaView,
    #[serde(default)]
    pub dedicated_ip_auto_warmup_enabled: bool,
    #[serde(default = "default_healthy")]
    pub enforcement_status: String,
    pub details: Option<serde_json::Value>,
    #[serde(default)]
    pub suppressed_reasons: Vec<String>,
    pub vdm_attributes: Option<serde_json::Value>,
}

fn default_healthy() -> String {
    "HEALTHY".to_string()
}

impl Default for AccountSettingsView {
    fn default() -> Self {
        Self {
            sending_enabled: true,
            production_access_enabled: true,
            send_quota: SendQuotaView::default(),
            dedicated_ip_auto_warmup_enabled: false,
            enforcement_status: "HEALTHY".to_string(),
            details: None,
            suppressed_reasons: Vec::new(),
            vdm_attributes: None,
        }
    }
}

/// Serializable view of deliverability dashboard options.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliverabilityDashboardView {
    #[serde(default)]
    pub dashboard_enabled: bool,
    pub subscription_expiry_date: Option<f64>,
    #[serde(default = "default_disabled")]
    pub account_status: String,
}

fn default_disabled() -> String {
    "DISABLED".to_string()
}

impl Default for DeliverabilityDashboardView {
    fn default() -> Self {
        Self {
            dashboard_enabled: false,
            subscription_expiry_date: None,
            account_status: "DISABLED".to_string(),
        }
    }
}

/// Serializable view of a suppressed destination entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuppressedDestinationView {
    pub email_address: String,
    pub reason: String,
    pub last_update_time: f64,
}

/// Serializable view of a deliverability test report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliverabilityTestReportView {
    pub report_id: String,
    pub report_name: Option<String>,
    pub from_email_address: String,
    pub create_date: f64,
    pub deliverability_test_status: String,
}

/// Serializable view of a reputation entity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReputationEntityView {
    pub entity_type: String,
    pub entity_reference: String,
    pub customer_managed_sending_status: String,
    pub policy: Option<String>,
}

/// Serializable view of an email previously sent through `SendEmail`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentEmailView {
    pub message_id: String,
    pub from: String,
    #[serde(default)]
    pub to: Vec<String>,
    pub subject: String,
    pub body: String,
    pub timestamp: Option<String>,
}

// --- From internal types to view types ---

impl From<&SesState> for SesStateView {
    fn from(state: &SesState) -> Self {
        SesStateView {
            identities: state
                .identities
                .iter()
                .map(|(k, v)| (k.clone(), EmailIdentityView::from(v)))
                .collect(),
            configuration_sets: state
                .configuration_sets
                .iter()
                .map(|(k, v)| (k.clone(), ConfigurationSetView::from(v)))
                .collect(),
            contact_lists: state
                .contact_lists
                .iter()
                .map(|(k, v)| (k.clone(), ContactListView::from(v)))
                .collect(),
            dedicated_ip_pools: state
                .dedicated_ip_pools
                .iter()
                .map(|(k, v)| (k.clone(), DedicatedIpPoolView::from(v)))
                .collect(),
            dedicated_ips: state
                .dedicated_ips
                .iter()
                .map(|(k, v)| (k.clone(), DedicatedIpView::from(v)))
                .collect(),
            custom_verification_email_templates: state
                .custom_verification_email_templates
                .iter()
                .map(|(k, v)| (k.clone(), CustomVerificationEmailTemplateView::from(v)))
                .collect(),
            import_jobs: state
                .import_jobs
                .iter()
                .map(|(k, v)| (k.clone(), ImportJobView::from(v)))
                .collect(),
            export_jobs: state
                .export_jobs
                .iter()
                .map(|(k, v)| (k.clone(), ExportJobView::from(v)))
                .collect(),
            multi_region_endpoints: state
                .multi_region_endpoints
                .iter()
                .map(|(k, v)| (k.clone(), MultiRegionEndpointView::from(v)))
                .collect(),
            tenants: state
                .tenants
                .iter()
                .map(|(k, v)| (k.clone(), TenantView::from(v)))
                .collect(),
            tenant_resource_associations: state
                .tenant_resource_associations
                .iter()
                .map(TenantResourceAssociationView::from)
                .collect(),
            email_templates: state
                .email_templates
                .iter()
                .map(|(k, v)| (k.clone(), EmailTemplateView::from(v)))
                .collect(),
            account_settings: AccountSettingsView::from(&state.account_settings),
            deliverability_dashboard: DeliverabilityDashboardView::from(
                &state.deliverability_dashboard,
            ),
            suppression_list: state
                .suppression_list
                .iter()
                .map(|(k, v)| (k.clone(), SuppressedDestinationView::from(v)))
                .collect(),
            deliverability_test_reports: state
                .deliverability_test_reports
                .iter()
                .map(|(k, v)| (k.clone(), DeliverabilityTestReportView::from(v)))
                .collect(),
            reputation_entities: state
                .reputation_entities
                .iter()
                .map(|(k, v)| (k.clone(), ReputationEntityView::from(v)))
                .collect(),
            sent_emails: state.sent_emails.iter().map(SentEmailView::from).collect(),
        }
    }
}

impl From<&SentEmail> for SentEmailView {
    fn from(e: &SentEmail) -> Self {
        SentEmailView {
            message_id: e.message_id.clone(),
            from: e.from.clone(),
            to: e.to.clone(),
            subject: e.subject.clone(),
            body: e.body.clone(),
            timestamp: Some(e.timestamp.to_rfc3339()),
        }
    }
}

impl From<&EmailIdentity> for EmailIdentityView {
    fn from(id: &EmailIdentity) -> Self {
        EmailIdentityView {
            name: id.name.clone(),
            identity_type: id.identity_type.clone(),
            verified: id.verified,
            created_timestamp: Some(id.created_timestamp.to_rfc3339()),
            policies: id.policies.clone(),
            tags: id.tags.clone(),
            configuration_set_name: id.configuration_set_name.clone(),
            dkim_signing_enabled: id.dkim_signing_enabled,
            dkim_signing_key_type: id.dkim_signing_key_type.clone(),
            dkim_domain: id.dkim_domain.clone(),
            feedback_forwarding_enabled: id.feedback_forwarding_enabled,
            mail_from_domain: id.mail_from_domain.clone(),
            behavior_on_mx_failure: id.behavior_on_mx_failure.clone(),
        }
    }
}

impl From<&ConfigurationSet> for ConfigurationSetView {
    fn from(cs: &ConfigurationSet) -> Self {
        ConfigurationSetView {
            name: cs.name.clone(),
            tags: cs.tags.clone(),
            event_destinations: cs
                .event_destinations
                .iter()
                .map(|(k, v)| (k.clone(), EventDestinationView::from(v)))
                .collect(),
            archiving_options: cs.archiving_options.clone(),
            delivery_options: cs.delivery_options.clone(),
            reputation_options: cs.reputation_options.clone(),
            sending_options: cs.sending_options.clone(),
            suppression_options: cs.suppression_options.clone(),
            tracking_options: cs.tracking_options.clone(),
            vdm_options: cs.vdm_options.clone(),
        }
    }
}

impl From<&EventDestination> for EventDestinationView {
    fn from(ed: &EventDestination) -> Self {
        EventDestinationView {
            name: ed.name.clone(),
            enabled: ed.enabled,
            matching_event_types: ed.matching_event_types.clone(),
            destination: ed.destination.clone(),
        }
    }
}

impl From<&ContactList> for ContactListView {
    fn from(cl: &ContactList) -> Self {
        ContactListView {
            name: cl.name.clone(),
            description: cl.description.clone(),
            tags: cl.tags.clone(),
            created_timestamp: Some(cl.created_timestamp.to_rfc3339()),
            last_updated_timestamp: Some(cl.last_updated_timestamp.to_rfc3339()),
            contacts: cl
                .contacts
                .iter()
                .map(|(k, v)| (k.clone(), ContactView::from(v)))
                .collect(),
        }
    }
}

impl From<&Contact> for ContactView {
    fn from(c: &Contact) -> Self {
        ContactView {
            email_address: c.email_address.clone(),
            topic_preferences: c
                .topic_preferences
                .iter()
                .map(TopicPreferenceView::from)
                .collect(),
            unsubscribe_all: c.unsubscribe_all,
            created_timestamp: Some(c.created_timestamp.to_rfc3339()),
            last_updated_timestamp: Some(c.last_updated_timestamp.to_rfc3339()),
        }
    }
}

impl From<&TopicPreference> for TopicPreferenceView {
    fn from(tp: &TopicPreference) -> Self {
        TopicPreferenceView {
            topic_name: tp.topic_name.clone(),
            subscription_status: tp.subscription_status.clone(),
        }
    }
}

impl From<&DedicatedIpPool> for DedicatedIpPoolView {
    fn from(pool: &DedicatedIpPool) -> Self {
        DedicatedIpPoolView {
            pool_name: pool.pool_name.clone(),
            scaling_mode: pool.scaling_mode.clone(),
            tags: pool.tags.clone(),
        }
    }
}

impl From<&DedicatedIp> for DedicatedIpView {
    fn from(ip: &DedicatedIp) -> Self {
        DedicatedIpView {
            ip: ip.ip.clone(),
            warmup_status: ip.warmup_status.clone(),
            warmup_percentage: ip.warmup_percentage,
            pool_name: ip.pool_name.clone(),
        }
    }
}

impl From<&CustomVerificationEmailTemplate> for CustomVerificationEmailTemplateView {
    fn from(t: &CustomVerificationEmailTemplate) -> Self {
        CustomVerificationEmailTemplateView {
            template_name: t.template_name.clone(),
            from_email_address: t.from_email_address.clone(),
            template_subject: t.template_subject.clone(),
            template_content: t.template_content.clone(),
            success_redirection_url: t.success_redirection_url.clone(),
            failure_redirection_url: t.failure_redirection_url.clone(),
            created_timestamp: Some(t.created_timestamp.to_rfc3339()),
        }
    }
}

impl From<&ImportJob> for ImportJobView {
    fn from(j: &ImportJob) -> Self {
        ImportJobView {
            job_id: j.job_id.clone(),
            import_destination: j.import_destination.clone(),
            import_data_source: j.import_data_source.clone(),
            job_status: j.job_status.clone(),
            created_timestamp: Some(j.created_timestamp.to_rfc3339()),
        }
    }
}

impl From<&ExportJob> for ExportJobView {
    fn from(j: &ExportJob) -> Self {
        ExportJobView {
            job_id: j.job_id.clone(),
            export_source_type: j.export_source_type.clone(),
            job_status: j.job_status.clone(),
            created_timestamp: Some(j.created_timestamp.to_rfc3339()),
            export_destination: j.export_destination.clone(),
        }
    }
}

impl From<&MultiRegionEndpoint> for MultiRegionEndpointView {
    fn from(ep: &MultiRegionEndpoint) -> Self {
        MultiRegionEndpointView {
            endpoint_name: ep.endpoint_name.clone(),
            endpoint_id: ep.endpoint_id.clone(),
            status: ep.status.clone(),
            regions: ep.regions.clone(),
            created_timestamp: Some(ep.created_timestamp.to_rfc3339()),
            last_updated_timestamp: Some(ep.last_updated_timestamp.to_rfc3339()),
        }
    }
}

impl From<&Tenant> for TenantView {
    fn from(t: &Tenant) -> Self {
        TenantView {
            tenant_name: t.tenant_name.clone(),
            tenant_id: t.tenant_id.clone(),
            tags: t.tags.clone(),
            created_timestamp: Some(t.created_timestamp.to_rfc3339()),
        }
    }
}

impl From<&TenantResourceAssociation> for TenantResourceAssociationView {
    fn from(a: &TenantResourceAssociation) -> Self {
        TenantResourceAssociationView {
            tenant_name: a.tenant_name.clone(),
            resource_arn: a.resource_arn.clone(),
        }
    }
}

// --- From view types to internal types ---

impl From<SesStateView> for SesState {
    fn from(view: SesStateView) -> Self {
        SesState {
            identities: view
                .identities
                .into_iter()
                .map(|(k, v)| (k, EmailIdentity::from(v)))
                .collect(),
            sent_emails: view.sent_emails.into_iter().map(SentEmail::from).collect(),
            configuration_sets: view
                .configuration_sets
                .into_iter()
                .map(|(k, v)| (k, ConfigurationSet::from(v)))
                .collect(),
            contact_lists: view
                .contact_lists
                .into_iter()
                .map(|(k, v)| (k, ContactList::from(v)))
                .collect(),
            dedicated_ip_pools: view
                .dedicated_ip_pools
                .into_iter()
                .map(|(k, v)| (k, DedicatedIpPool::from(v)))
                .collect(),
            dedicated_ips: view
                .dedicated_ips
                .into_iter()
                .map(|(k, v)| (k, DedicatedIp::from(v)))
                .collect(),
            email_templates: view
                .email_templates
                .into_iter()
                .map(|(k, v)| (k, EmailTemplate::from(v)))
                .collect(),
            custom_verification_email_templates: view
                .custom_verification_email_templates
                .into_iter()
                .map(|(k, v)| (k, CustomVerificationEmailTemplate::from(v)))
                .collect(),
            import_jobs: view
                .import_jobs
                .into_iter()
                .map(|(k, v)| (k, ImportJob::from(v)))
                .collect(),
            export_jobs: view
                .export_jobs
                .into_iter()
                .map(|(k, v)| (k, ExportJob::from(v)))
                .collect(),
            multi_region_endpoints: view
                .multi_region_endpoints
                .into_iter()
                .map(|(k, v)| (k, MultiRegionEndpoint::from(v)))
                .collect(),
            account_settings: AccountSettings::from(view.account_settings),
            deliverability_dashboard: DeliverabilityDashboardOptions::from(
                view.deliverability_dashboard,
            ),
            suppression_list: view
                .suppression_list
                .into_iter()
                .map(|(k, v)| (k, SuppressedDestinationEntry::from(v)))
                .collect(),
            tenants: view
                .tenants
                .into_iter()
                .map(|(k, v)| (k, Tenant::from(v)))
                .collect(),
            tenant_resource_associations: view
                .tenant_resource_associations
                .into_iter()
                .map(TenantResourceAssociation::from)
                .collect(),
            deliverability_test_reports: view
                .deliverability_test_reports
                .into_iter()
                .map(|(k, v)| (k, DeliverabilityTestReportRecord::from(v)))
                .collect(),
            reputation_entities: view
                .reputation_entities
                .into_iter()
                .map(|(k, v)| (k, ReputationEntityRecord::from(v)))
                .collect(),
        }
    }
}

impl From<EmailIdentityView> for EmailIdentity {
    fn from(view: EmailIdentityView) -> Self {
        let created_timestamp = view
            .created_timestamp
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);
        EmailIdentity {
            name: view.name,
            identity_type: view.identity_type,
            verified: view.verified,
            created_timestamp,
            policies: view.policies,
            tags: view.tags,
            configuration_set_name: view.configuration_set_name,
            dkim_signing_enabled: view.dkim_signing_enabled,
            dkim_signing_key_type: view.dkim_signing_key_type,
            dkim_domain: view.dkim_domain,
            feedback_forwarding_enabled: view.feedback_forwarding_enabled,
            mail_from_domain: view.mail_from_domain,
            behavior_on_mx_failure: view.behavior_on_mx_failure,
        }
    }
}

impl From<ConfigurationSetView> for ConfigurationSet {
    fn from(view: ConfigurationSetView) -> Self {
        ConfigurationSet {
            name: view.name,
            tags: view.tags,
            event_destinations: view
                .event_destinations
                .into_iter()
                .map(|(k, v)| (k, EventDestination::from(v)))
                .collect(),
            archiving_options: view.archiving_options,
            delivery_options: view.delivery_options,
            reputation_options: view.reputation_options,
            sending_options: view.sending_options,
            suppression_options: view.suppression_options,
            tracking_options: view.tracking_options,
            vdm_options: view.vdm_options,
        }
    }
}

impl From<EventDestinationView> for EventDestination {
    fn from(view: EventDestinationView) -> Self {
        EventDestination {
            name: view.name,
            enabled: view.enabled,
            matching_event_types: view.matching_event_types,
            destination: view.destination,
        }
    }
}

impl From<ContactListView> for ContactList {
    fn from(view: ContactListView) -> Self {
        let now = Utc::now();
        let created_timestamp = view
            .created_timestamp
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or(now);
        let last_updated_timestamp = view
            .last_updated_timestamp
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or(now);
        ContactList {
            name: view.name,
            description: view.description,
            tags: view.tags,
            created_timestamp,
            last_updated_timestamp,
            contacts: view
                .contacts
                .into_iter()
                .map(|(k, v)| (k, Contact::from(v)))
                .collect(),
        }
    }
}

impl From<ContactView> for Contact {
    fn from(view: ContactView) -> Self {
        let now = Utc::now();
        let created_timestamp = view
            .created_timestamp
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or(now);
        let last_updated_timestamp = view
            .last_updated_timestamp
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or(now);
        Contact {
            email_address: view.email_address,
            topic_preferences: view
                .topic_preferences
                .into_iter()
                .map(TopicPreference::from)
                .collect(),
            unsubscribe_all: view.unsubscribe_all,
            created_timestamp,
            last_updated_timestamp,
        }
    }
}

impl From<TopicPreferenceView> for TopicPreference {
    fn from(view: TopicPreferenceView) -> Self {
        TopicPreference {
            topic_name: view.topic_name,
            subscription_status: view.subscription_status,
        }
    }
}

impl From<DedicatedIpPoolView> for DedicatedIpPool {
    fn from(view: DedicatedIpPoolView) -> Self {
        DedicatedIpPool {
            pool_name: view.pool_name,
            scaling_mode: view.scaling_mode,
            tags: view.tags,
        }
    }
}

impl From<DedicatedIpView> for DedicatedIp {
    fn from(view: DedicatedIpView) -> Self {
        DedicatedIp {
            ip: view.ip,
            warmup_status: view.warmup_status,
            warmup_percentage: view.warmup_percentage,
            pool_name: view.pool_name,
        }
    }
}

impl From<CustomVerificationEmailTemplateView> for CustomVerificationEmailTemplate {
    fn from(view: CustomVerificationEmailTemplateView) -> Self {
        let created_timestamp = view
            .created_timestamp
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);
        CustomVerificationEmailTemplate {
            template_name: view.template_name,
            from_email_address: view.from_email_address,
            template_subject: view.template_subject,
            template_content: view.template_content,
            success_redirection_url: view.success_redirection_url,
            failure_redirection_url: view.failure_redirection_url,
            created_timestamp,
        }
    }
}

impl From<ImportJobView> for ImportJob {
    fn from(view: ImportJobView) -> Self {
        let created_timestamp = view
            .created_timestamp
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);
        ImportJob {
            job_id: view.job_id,
            import_destination: view.import_destination,
            import_data_source: view.import_data_source,
            job_status: view.job_status,
            created_timestamp,
        }
    }
}

impl From<ExportJobView> for ExportJob {
    fn from(view: ExportJobView) -> Self {
        let created_timestamp = view
            .created_timestamp
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);
        ExportJob {
            job_id: view.job_id,
            export_source_type: view.export_source_type,
            job_status: view.job_status,
            created_timestamp,
            export_destination: view.export_destination,
        }
    }
}

impl From<MultiRegionEndpointView> for MultiRegionEndpoint {
    fn from(view: MultiRegionEndpointView) -> Self {
        let now = Utc::now();
        let created_timestamp = view
            .created_timestamp
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or(now);
        let last_updated_timestamp = view
            .last_updated_timestamp
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or(now);
        MultiRegionEndpoint {
            endpoint_name: view.endpoint_name,
            endpoint_id: view.endpoint_id,
            status: view.status,
            regions: view.regions,
            created_timestamp,
            last_updated_timestamp,
        }
    }
}

impl From<TenantView> for Tenant {
    fn from(view: TenantView) -> Self {
        let created_timestamp = view
            .created_timestamp
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);
        Tenant {
            tenant_name: view.tenant_name,
            tenant_id: view.tenant_id,
            tags: view.tags,
            created_timestamp,
        }
    }
}

impl From<TenantResourceAssociationView> for TenantResourceAssociation {
    fn from(view: TenantResourceAssociationView) -> Self {
        TenantResourceAssociation {
            tenant_name: view.tenant_name,
            resource_arn: view.resource_arn,
        }
    }
}

impl From<SentEmailView> for SentEmail {
    fn from(view: SentEmailView) -> Self {
        let timestamp = view
            .timestamp
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);
        SentEmail {
            message_id: view.message_id,
            from: view.from,
            to: view.to,
            subject: view.subject,
            body: view.body,
            timestamp,
        }
    }
}

impl From<&EmailTemplate> for EmailTemplateView {
    fn from(t: &EmailTemplate) -> Self {
        EmailTemplateView {
            template_name: t.template_name.clone(),
            subject_part: t.subject_part.clone(),
            text_part: t.text_part.clone(),
            html_part: t.html_part.clone(),
            created_timestamp: Some(t.created_timestamp.to_rfc3339()),
        }
    }
}

impl From<EmailTemplateView> for EmailTemplate {
    fn from(view: EmailTemplateView) -> Self {
        let created_timestamp = view
            .created_timestamp
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);
        EmailTemplate {
            template_name: view.template_name,
            subject_part: view.subject_part,
            text_part: view.text_part,
            html_part: view.html_part,
            created_timestamp,
        }
    }
}

impl From<&AccountSettings> for AccountSettingsView {
    fn from(a: &AccountSettings) -> Self {
        AccountSettingsView {
            sending_enabled: a.sending_enabled,
            production_access_enabled: a.production_access_enabled,
            send_quota: SendQuotaView {
                max_24_hour_send: a.send_quota.max_24_hour_send,
                max_send_rate: a.send_quota.max_send_rate,
                sent_last_24_hours: a.send_quota.sent_last_24_hours,
            },
            dedicated_ip_auto_warmup_enabled: a.dedicated_ip_auto_warmup_enabled,
            enforcement_status: a.enforcement_status.clone(),
            details: a.details.clone(),
            suppressed_reasons: a.suppressed_reasons.clone(),
            vdm_attributes: a.vdm_attributes.clone(),
        }
    }
}

impl From<AccountSettingsView> for AccountSettings {
    fn from(view: AccountSettingsView) -> Self {
        AccountSettings {
            sending_enabled: view.sending_enabled,
            production_access_enabled: view.production_access_enabled,
            send_quota: SendQuota {
                max_24_hour_send: view.send_quota.max_24_hour_send,
                max_send_rate: view.send_quota.max_send_rate,
                sent_last_24_hours: view.send_quota.sent_last_24_hours,
            },
            dedicated_ip_auto_warmup_enabled: view.dedicated_ip_auto_warmup_enabled,
            enforcement_status: view.enforcement_status,
            details: view.details,
            suppressed_reasons: view.suppressed_reasons,
            vdm_attributes: view.vdm_attributes,
        }
    }
}

impl From<&DeliverabilityDashboardOptions> for DeliverabilityDashboardView {
    fn from(d: &DeliverabilityDashboardOptions) -> Self {
        DeliverabilityDashboardView {
            dashboard_enabled: d.dashboard_enabled,
            subscription_expiry_date: d.subscription_expiry_date,
            account_status: d.account_status.clone(),
        }
    }
}

impl From<DeliverabilityDashboardView> for DeliverabilityDashboardOptions {
    fn from(view: DeliverabilityDashboardView) -> Self {
        DeliverabilityDashboardOptions {
            dashboard_enabled: view.dashboard_enabled,
            subscription_expiry_date: view.subscription_expiry_date,
            account_status: view.account_status,
        }
    }
}

impl From<&SuppressedDestinationEntry> for SuppressedDestinationView {
    fn from(e: &SuppressedDestinationEntry) -> Self {
        SuppressedDestinationView {
            email_address: e.email_address.clone(),
            reason: e.reason.clone(),
            last_update_time: e.last_update_time,
        }
    }
}

impl From<SuppressedDestinationView> for SuppressedDestinationEntry {
    fn from(view: SuppressedDestinationView) -> Self {
        SuppressedDestinationEntry {
            email_address: view.email_address,
            reason: view.reason,
            last_update_time: view.last_update_time,
        }
    }
}

impl From<&DeliverabilityTestReportRecord> for DeliverabilityTestReportView {
    fn from(r: &DeliverabilityTestReportRecord) -> Self {
        DeliverabilityTestReportView {
            report_id: r.report_id.clone(),
            report_name: r.report_name.clone(),
            from_email_address: r.from_email_address.clone(),
            create_date: r.create_date,
            deliverability_test_status: r.deliverability_test_status.clone(),
        }
    }
}

impl From<DeliverabilityTestReportView> for DeliverabilityTestReportRecord {
    fn from(view: DeliverabilityTestReportView) -> Self {
        DeliverabilityTestReportRecord {
            report_id: view.report_id,
            report_name: view.report_name,
            from_email_address: view.from_email_address,
            create_date: view.create_date,
            deliverability_test_status: view.deliverability_test_status,
        }
    }
}

impl From<&ReputationEntityRecord> for ReputationEntityView {
    fn from(r: &ReputationEntityRecord) -> Self {
        ReputationEntityView {
            entity_type: r.entity_type.clone(),
            entity_reference: r.entity_reference.clone(),
            customer_managed_sending_status: r.customer_managed_sending_status.clone(),
            policy: r.policy.clone(),
        }
    }
}

impl From<ReputationEntityView> for ReputationEntityRecord {
    fn from(view: ReputationEntityView) -> Self {
        ReputationEntityRecord {
            entity_type: view.entity_type,
            entity_reference: view.entity_reference,
            customer_managed_sending_status: view.customer_managed_sending_status,
            policy: view.policy,
        }
    }
}

// --- StatefulService implementation ---

impl StatefulService for SesV2Service {
    type StateView = SesStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        SesStateView::from(&*guard)
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
            *guard = SesState::from(view);
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
            for (name, id_view) in view.identities {
                guard.identities.insert(name, EmailIdentity::from(id_view));
            }
            for (name, cs_view) in view.configuration_sets {
                guard
                    .configuration_sets
                    .insert(name, ConfigurationSet::from(cs_view));
            }
            for (name, cl_view) in view.contact_lists {
                guard.contact_lists.insert(name, ContactList::from(cl_view));
            }
            for (name, pool_view) in view.dedicated_ip_pools {
                guard
                    .dedicated_ip_pools
                    .insert(name, DedicatedIpPool::from(pool_view));
            }
            for (ip, ip_view) in view.dedicated_ips {
                guard.dedicated_ips.insert(ip, DedicatedIp::from(ip_view));
            }
            for (name, tmpl_view) in view.custom_verification_email_templates {
                guard
                    .custom_verification_email_templates
                    .insert(name, CustomVerificationEmailTemplate::from(tmpl_view));
            }
            for (name, tmpl_view) in view.email_templates {
                guard
                    .email_templates
                    .insert(name, EmailTemplate::from(tmpl_view));
            }
            for (id, job_view) in view.import_jobs {
                guard.import_jobs.insert(id, ImportJob::from(job_view));
            }
            for (id, job_view) in view.export_jobs {
                guard.export_jobs.insert(id, ExportJob::from(job_view));
            }
            for (name, ep_view) in view.multi_region_endpoints {
                guard
                    .multi_region_endpoints
                    .insert(name, MultiRegionEndpoint::from(ep_view));
            }
            for (name, tenant_view) in view.tenants {
                guard.tenants.insert(name, Tenant::from(tenant_view));
            }
            for assoc_view in view.tenant_resource_associations {
                let assoc = TenantResourceAssociation::from(assoc_view);
                let already = guard.tenant_resource_associations.iter().any(|a| {
                    a.tenant_name == assoc.tenant_name && a.resource_arn == assoc.resource_arn
                });
                if !already {
                    guard.tenant_resource_associations.push(assoc);
                }
            }
            for (addr, entry_view) in view.suppression_list {
                guard
                    .suppression_list
                    .insert(addr, SuppressedDestinationEntry::from(entry_view));
            }
            for (id, report_view) in view.deliverability_test_reports {
                guard
                    .deliverability_test_reports
                    .insert(id, DeliverabilityTestReportRecord::from(report_view));
            }
            for (key, entity_view) in view.reputation_entities {
                guard
                    .reputation_entities
                    .insert(key, ReputationEntityRecord::from(entity_view));
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
