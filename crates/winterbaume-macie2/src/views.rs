//! Serde-compatible view types for Macie2 state snapshots.

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::Macie2Service;
use crate::state::Macie2State;
use crate::types::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Macie2StateView {
    pub session: Option<MacieSessionView>,
    #[serde(default)]
    pub members: HashMap<String, MacieMemberView>,
    #[serde(default)]
    pub invitations: Vec<MacieInvitationView>,
    #[serde(default)]
    pub admin_accounts: Vec<MacieAdminAccountView>,
    pub administrator: Option<MacieAdministratorView>,
    /// Per-resource tags keyed by resource ARN.
    #[serde(default)]
    pub resource_tags: HashMap<String, HashMap<String, String>>,

    // ── New resource collections ──────────────────────────────────────────────
    #[serde(default)]
    pub custom_data_identifiers: HashMap<String, MacieCustomDataIdentifierView>,
    #[serde(default)]
    pub allow_lists: HashMap<String, MacieAllowListView>,
    #[serde(default)]
    pub findings_filters: HashMap<String, MacieFindingsFilterView>,
    #[serde(default)]
    pub classification_jobs: HashMap<String, MacieClassificationJobView>,
    #[serde(default)]
    pub sensitivity_inspection_templates: HashMap<String, MacieSensitivityInspectionTemplateView>,
    pub automated_discovery_config: Option<MacieAutomatedDiscoveryConfigView>,
    #[serde(default)]
    pub automated_discovery_accounts: HashMap<String, MacieAutomatedDiscoveryAccountView>,
    pub reveal_config: Option<MacieRevealConfigView>,
    pub org_config: Option<MacieOrgConfigView>,
    pub classification_export_config: Option<MacieClassificationExportConfigView>,
    pub findings_publication_config: Option<MacieFindingsPublicationConfigView>,
    #[serde(default)]
    pub findings: HashMap<String, MacieFindingView>,
}

// ── Existing view types ───────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacieSessionView {
    pub status: String,
    pub finding_publishing_frequency: String,
    pub created_at: String,
    pub updated_at: String,
    pub service_role: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacieMemberView {
    pub account_id: String,
    pub email: String,
    pub relationship_status: String,
    pub invited_at: Option<String>,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacieInvitationView {
    pub account_id: String,
    pub invitation_id: String,
    pub invited_at: String,
    pub relationship_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacieAdminAccountView {
    pub account_id: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacieAdministratorView {
    pub account_id: String,
    pub invitation_id: String,
    pub invited_at: String,
    pub relationship_status: String,
}

// ── New view types ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacieSeverityLevelView {
    pub occurrences_threshold: i64,
    pub severity: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacieCustomDataIdentifierView {
    pub id: String,
    pub arn: String,
    pub name: String,
    pub description: Option<String>,
    pub regex: String,
    pub keywords: Vec<String>,
    pub ignore_words: Vec<String>,
    pub maximum_match_distance: i32,
    pub severity_levels: Vec<MacieSeverityLevelView>,
    pub tags: HashMap<String, String>,
    pub created_at: String,
    pub deleted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacieAllowListCriteriaView {
    pub regex: Option<String>,
    pub s3_bucket_name: Option<String>,
    pub s3_object_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacieAllowListView {
    pub id: String,
    pub arn: String,
    pub name: String,
    pub description: Option<String>,
    pub criteria: MacieAllowListCriteriaView,
    pub tags: HashMap<String, String>,
    pub created_at: String,
    pub updated_at: String,
    pub status_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacieFindingsFilterView {
    pub id: String,
    pub arn: String,
    pub name: String,
    pub description: Option<String>,
    pub action: String,
    pub position: i32,
    pub finding_criteria: serde_json::Value,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacieClassificationJobView {
    pub job_id: String,
    pub job_arn: String,
    pub name: String,
    pub description: Option<String>,
    pub job_type: String,
    pub job_status: String,
    pub client_token: String,
    pub s3_job_definition: serde_json::Value,
    pub allow_list_ids: Vec<String>,
    pub custom_data_identifier_ids: Vec<String>,
    pub managed_data_identifier_ids: Vec<String>,
    pub managed_data_identifier_selector: Option<String>,
    pub sampling_percentage: Option<i32>,
    pub schedule_frequency: Option<serde_json::Value>,
    pub initial_run: bool,
    pub tags: HashMap<String, String>,
    pub created_at: String,
    pub last_run_time: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacieSensitivityInspectionTemplateView {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub excludes_managed_data_identifier_ids: Vec<String>,
    pub includes_allow_list_ids: Vec<String>,
    pub includes_custom_data_identifier_ids: Vec<String>,
    pub includes_managed_data_identifier_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacieAutomatedDiscoveryConfigView {
    pub status: String,
    pub auto_enable_organization_members: Option<String>,
    pub classification_scope_id: String,
    pub sensitivity_inspection_template_id: String,
    pub first_enabled_at: Option<String>,
    pub disabled_at: Option<String>,
    pub last_updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacieAutomatedDiscoveryAccountView {
    pub account_id: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacieRevealConfigView {
    pub status: String,
    pub kms_key_id: Option<String>,
    pub retrieval_mode: String,
    pub role_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacieOrgConfigView {
    pub auto_enable: bool,
    pub max_account_limit_reached: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacieClassificationExportConfigView {
    pub raw: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacieFindingsPublicationConfigView {
    pub publish_classification_findings: bool,
    pub publish_policy_findings: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacieFindingView {
    pub id: String,
    pub finding_type: String,
    pub severity: String,
    pub title: String,
    pub description: String,
    pub created_at: String,
    pub updated_at: String,
    pub category: String,
    pub sample: bool,
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn parse_dt(s: &str) -> DateTime<Utc> {
    DateTime::parse_from_rfc3339(s)
        .map(|dt| dt.with_timezone(&Utc))
        .unwrap_or_else(|_| Utc::now())
}

fn opt_parse_dt(s: Option<&str>) -> Option<DateTime<Utc>> {
    s.map(parse_dt)
}

// ── From internal types to view types ────────────────────────────────────────

impl From<&Macie2State> for Macie2StateView {
    fn from(state: &Macie2State) -> Self {
        Macie2StateView {
            session: state.session.as_ref().map(|s| MacieSessionView {
                status: s.status.clone(),
                finding_publishing_frequency: s.finding_publishing_frequency.clone(),
                created_at: s.created_at.to_rfc3339(),
                updated_at: s.updated_at.to_rfc3339(),
                service_role: s.service_role.clone(),
            }),
            members: state
                .members
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        MacieMemberView {
                            account_id: v.account_id.clone(),
                            email: v.email.clone(),
                            relationship_status: v.relationship_status.clone(),
                            invited_at: v.invited_at.map(|t| t.to_rfc3339()),
                            updated_at: v.updated_at.to_rfc3339(),
                        },
                    )
                })
                .collect(),
            invitations: state
                .invitations
                .iter()
                .map(|inv| MacieInvitationView {
                    account_id: inv.account_id.clone(),
                    invitation_id: inv.invitation_id.clone(),
                    invited_at: inv.invited_at.to_rfc3339(),
                    relationship_status: inv.relationship_status.clone(),
                })
                .collect(),
            admin_accounts: state
                .admin_accounts
                .iter()
                .map(|a| MacieAdminAccountView {
                    account_id: a.account_id.clone(),
                    status: a.status.clone(),
                })
                .collect(),
            administrator: state
                .administrator
                .as_ref()
                .map(|a| MacieAdministratorView {
                    account_id: a.account_id.clone(),
                    invitation_id: a.invitation_id.clone(),
                    invited_at: a.invited_at.to_rfc3339(),
                    relationship_status: a.relationship_status.clone(),
                }),
            resource_tags: state.resource_tags.clone(),
            custom_data_identifiers: state
                .custom_data_identifiers
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        MacieCustomDataIdentifierView {
                            id: v.id.clone(),
                            arn: v.arn.clone(),
                            name: v.name.clone(),
                            description: v.description.clone(),
                            regex: v.regex.clone(),
                            keywords: v.keywords.clone(),
                            ignore_words: v.ignore_words.clone(),
                            maximum_match_distance: v.maximum_match_distance,
                            severity_levels: v
                                .severity_levels
                                .iter()
                                .map(|sl| MacieSeverityLevelView {
                                    occurrences_threshold: sl.occurrences_threshold,
                                    severity: sl.severity.clone(),
                                })
                                .collect(),
                            tags: v.tags.clone(),
                            created_at: v.created_at.to_rfc3339(),
                            deleted: v.deleted,
                        },
                    )
                })
                .collect(),
            allow_lists: state
                .allow_lists
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        MacieAllowListView {
                            id: v.id.clone(),
                            arn: v.arn.clone(),
                            name: v.name.clone(),
                            description: v.description.clone(),
                            criteria: MacieAllowListCriteriaView {
                                regex: v.criteria.regex.clone(),
                                s3_bucket_name: v
                                    .criteria
                                    .s3_words_list
                                    .as_ref()
                                    .map(|s| s.bucket_name.clone()),
                                s3_object_key: v
                                    .criteria
                                    .s3_words_list
                                    .as_ref()
                                    .map(|s| s.object_key.clone()),
                            },
                            tags: v.tags.clone(),
                            created_at: v.created_at.to_rfc3339(),
                            updated_at: v.updated_at.to_rfc3339(),
                            status_code: v.status_code.clone(),
                        },
                    )
                })
                .collect(),
            findings_filters: state
                .findings_filters
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        MacieFindingsFilterView {
                            id: v.id.clone(),
                            arn: v.arn.clone(),
                            name: v.name.clone(),
                            description: v.description.clone(),
                            action: v.action.clone(),
                            position: v.position,
                            finding_criteria: v.finding_criteria.clone(),
                            tags: v.tags.clone(),
                        },
                    )
                })
                .collect(),
            classification_jobs: state
                .classification_jobs
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        MacieClassificationJobView {
                            job_id: v.job_id.clone(),
                            job_arn: v.job_arn.clone(),
                            name: v.name.clone(),
                            description: v.description.clone(),
                            job_type: v.job_type.clone(),
                            job_status: v.job_status.clone(),
                            client_token: v.client_token.clone(),
                            s3_job_definition: v.s3_job_definition.clone(),
                            allow_list_ids: v.allow_list_ids.clone(),
                            custom_data_identifier_ids: v.custom_data_identifier_ids.clone(),
                            managed_data_identifier_ids: v.managed_data_identifier_ids.clone(),
                            managed_data_identifier_selector: v
                                .managed_data_identifier_selector
                                .clone(),
                            sampling_percentage: v.sampling_percentage,
                            schedule_frequency: v.schedule_frequency.clone(),
                            initial_run: v.initial_run,
                            tags: v.tags.clone(),
                            created_at: v.created_at.to_rfc3339(),
                            last_run_time: v.last_run_time.map(|t| t.to_rfc3339()),
                        },
                    )
                })
                .collect(),
            sensitivity_inspection_templates: state
                .sensitivity_inspection_templates
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        MacieSensitivityInspectionTemplateView {
                            id: v.id.clone(),
                            name: v.name.clone(),
                            description: v.description.clone(),
                            excludes_managed_data_identifier_ids: v
                                .excludes_managed_data_identifier_ids
                                .clone(),
                            includes_allow_list_ids: v.includes_allow_list_ids.clone(),
                            includes_custom_data_identifier_ids: v
                                .includes_custom_data_identifier_ids
                                .clone(),
                            includes_managed_data_identifier_ids: v
                                .includes_managed_data_identifier_ids
                                .clone(),
                        },
                    )
                })
                .collect(),
            automated_discovery_config: state.automated_discovery_config.as_ref().map(|c| {
                MacieAutomatedDiscoveryConfigView {
                    status: c.status.clone(),
                    auto_enable_organization_members: c.auto_enable_organization_members.clone(),
                    classification_scope_id: c.classification_scope_id.clone(),
                    sensitivity_inspection_template_id: c
                        .sensitivity_inspection_template_id
                        .clone(),
                    first_enabled_at: c.first_enabled_at.map(|t| t.to_rfc3339()),
                    disabled_at: c.disabled_at.map(|t| t.to_rfc3339()),
                    last_updated_at: c.last_updated_at.to_rfc3339(),
                }
            }),
            automated_discovery_accounts: state
                .automated_discovery_accounts
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        MacieAutomatedDiscoveryAccountView {
                            account_id: v.account_id.clone(),
                            status: v.status.clone(),
                        },
                    )
                })
                .collect(),
            reveal_config: state.reveal_config.as_ref().map(|c| MacieRevealConfigView {
                status: c.status.clone(),
                kms_key_id: c.kms_key_id.clone(),
                retrieval_mode: c.retrieval_mode.clone(),
                role_name: c.role_name.clone(),
            }),
            org_config: state.org_config.as_ref().map(|c| MacieOrgConfigView {
                auto_enable: c.auto_enable,
                max_account_limit_reached: c.max_account_limit_reached,
            }),
            classification_export_config: state
                .classification_export_config
                .as_ref()
                .map(|c| MacieClassificationExportConfigView { raw: c.raw.clone() }),
            findings_publication_config: state.findings_publication_config.as_ref().map(|c| {
                MacieFindingsPublicationConfigView {
                    publish_classification_findings: c.publish_classification_findings,
                    publish_policy_findings: c.publish_policy_findings,
                }
            }),
            findings: state
                .findings
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        MacieFindingView {
                            id: v.id.clone(),
                            finding_type: v.finding_type.clone(),
                            severity: v.severity.clone(),
                            title: v.title.clone(),
                            description: v.description.clone(),
                            created_at: v.created_at.to_rfc3339(),
                            updated_at: v.updated_at.to_rfc3339(),
                            category: v.category.clone(),
                            sample: v.sample,
                        },
                    )
                })
                .collect(),
        }
    }
}

// ── From view types to internal types ────────────────────────────────────────

impl From<Macie2StateView> for Macie2State {
    fn from(view: Macie2StateView) -> Self {
        Macie2State {
            session: view.session.map(|s| MacieSession {
                status: s.status,
                finding_publishing_frequency: s.finding_publishing_frequency,
                created_at: parse_dt(&s.created_at),
                updated_at: parse_dt(&s.updated_at),
                service_role: s.service_role,
            }),
            members: view
                .members
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        MacieMember {
                            account_id: v.account_id,
                            email: v.email,
                            relationship_status: v.relationship_status,
                            invited_at: v.invited_at.as_deref().map(parse_dt),
                            updated_at: parse_dt(&v.updated_at),
                        },
                    )
                })
                .collect(),
            invitations: view
                .invitations
                .into_iter()
                .map(|inv| MacieInvitation {
                    account_id: inv.account_id,
                    invitation_id: inv.invitation_id,
                    invited_at: parse_dt(&inv.invited_at),
                    relationship_status: inv.relationship_status,
                })
                .collect(),
            admin_accounts: view
                .admin_accounts
                .into_iter()
                .map(|a| MacieAdminAccount {
                    account_id: a.account_id,
                    status: a.status,
                })
                .collect(),
            administrator: view.administrator.map(|a| MacieAdministrator {
                account_id: a.account_id,
                invitation_id: a.invitation_id,
                invited_at: parse_dt(&a.invited_at),
                relationship_status: a.relationship_status,
            }),
            resource_tags: view.resource_tags,
            custom_data_identifiers: view
                .custom_data_identifiers
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        MacieCustomDataIdentifier {
                            id: v.id,
                            arn: v.arn,
                            name: v.name,
                            description: v.description,
                            regex: v.regex,
                            keywords: v.keywords,
                            ignore_words: v.ignore_words,
                            maximum_match_distance: v.maximum_match_distance,
                            severity_levels: v
                                .severity_levels
                                .into_iter()
                                .map(|sl| MacieSeverityLevel {
                                    occurrences_threshold: sl.occurrences_threshold,
                                    severity: sl.severity,
                                })
                                .collect(),
                            tags: v.tags,
                            created_at: parse_dt(&v.created_at),
                            deleted: v.deleted,
                        },
                    )
                })
                .collect(),
            allow_lists: view
                .allow_lists
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        MacieAllowList {
                            id: v.id,
                            arn: v.arn,
                            name: v.name,
                            description: v.description,
                            criteria: MacieAllowListCriteria {
                                regex: v.criteria.regex,
                                s3_words_list: v.criteria.s3_bucket_name.map(|b| {
                                    MacieS3WordsList {
                                        bucket_name: b,
                                        object_key: v.criteria.s3_object_key.unwrap_or_default(),
                                    }
                                }),
                            },
                            tags: v.tags,
                            created_at: parse_dt(&v.created_at),
                            updated_at: parse_dt(&v.updated_at),
                            status_code: v.status_code,
                        },
                    )
                })
                .collect(),
            findings_filters: view
                .findings_filters
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        MacieFindingsFilter {
                            id: v.id,
                            arn: v.arn,
                            name: v.name,
                            description: v.description,
                            action: v.action,
                            position: v.position,
                            finding_criteria: v.finding_criteria,
                            tags: v.tags,
                        },
                    )
                })
                .collect(),
            classification_jobs: view
                .classification_jobs
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        MacieClassificationJob {
                            job_id: v.job_id,
                            job_arn: v.job_arn,
                            name: v.name,
                            description: v.description,
                            job_type: v.job_type,
                            job_status: v.job_status,
                            client_token: v.client_token,
                            s3_job_definition: v.s3_job_definition,
                            allow_list_ids: v.allow_list_ids,
                            custom_data_identifier_ids: v.custom_data_identifier_ids,
                            managed_data_identifier_ids: v.managed_data_identifier_ids,
                            managed_data_identifier_selector: v.managed_data_identifier_selector,
                            sampling_percentage: v.sampling_percentage,
                            schedule_frequency: v.schedule_frequency,
                            initial_run: v.initial_run,
                            tags: v.tags,
                            created_at: parse_dt(&v.created_at),
                            last_run_time: v.last_run_time.as_deref().map(parse_dt),
                        },
                    )
                })
                .collect(),
            sensitivity_inspection_templates: view
                .sensitivity_inspection_templates
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        MacieSensitivityInspectionTemplate {
                            id: v.id,
                            name: v.name,
                            description: v.description,
                            excludes_managed_data_identifier_ids: v
                                .excludes_managed_data_identifier_ids,
                            includes_allow_list_ids: v.includes_allow_list_ids,
                            includes_custom_data_identifier_ids: v
                                .includes_custom_data_identifier_ids,
                            includes_managed_data_identifier_ids: v
                                .includes_managed_data_identifier_ids,
                        },
                    )
                })
                .collect(),
            automated_discovery_config: view.automated_discovery_config.map(|c| {
                MacieAutomatedDiscoveryConfig {
                    status: c.status,
                    auto_enable_organization_members: c.auto_enable_organization_members,
                    classification_scope_id: c.classification_scope_id,
                    sensitivity_inspection_template_id: c.sensitivity_inspection_template_id,
                    first_enabled_at: c.first_enabled_at.as_deref().map(parse_dt),
                    disabled_at: c.disabled_at.as_deref().map(parse_dt),
                    last_updated_at: parse_dt(&c.last_updated_at),
                }
            }),
            automated_discovery_accounts: view
                .automated_discovery_accounts
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        MacieAutomatedDiscoveryAccount {
                            account_id: v.account_id,
                            status: v.status,
                        },
                    )
                })
                .collect(),
            reveal_config: view.reveal_config.map(|c| MacieRevealConfig {
                status: c.status,
                kms_key_id: c.kms_key_id,
                retrieval_mode: c.retrieval_mode,
                role_name: c.role_name,
            }),
            org_config: view.org_config.map(|c| MacieOrgConfig {
                auto_enable: c.auto_enable,
                max_account_limit_reached: c.max_account_limit_reached,
            }),
            classification_export_config: view
                .classification_export_config
                .map(|c| MacieClassificationExportConfig { raw: c.raw }),
            findings_publication_config: view.findings_publication_config.map(|c| {
                MacieFindingsPublicationConfig {
                    publish_classification_findings: c.publish_classification_findings,
                    publish_policy_findings: c.publish_policy_findings,
                }
            }),
            findings: view
                .findings
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        MacieFinding {
                            id: v.id,
                            finding_type: v.finding_type,
                            severity: v.severity,
                            title: v.title,
                            description: v.description,
                            created_at: parse_dt(&v.created_at),
                            updated_at: parse_dt(&v.updated_at),
                            category: v.category,
                            sample: v.sample,
                        },
                    )
                })
                .collect(),
        }
    }
}

// ── StatefulService implementation ───────────────────────────────────────────

impl StatefulService for Macie2Service {
    type StateView = Macie2StateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        Macie2StateView::from(&*guard)
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
            *guard = Macie2State::from(view);
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
            let incoming = Macie2State::from(view);
            if incoming.session.is_some() {
                guard.session = incoming.session;
            }
            for (k, v) in incoming.members {
                guard.members.insert(k, v);
            }
            for inv in incoming.invitations {
                guard.invitations.push(inv);
            }
            for a in incoming.admin_accounts {
                guard.admin_accounts.push(a);
            }
            if incoming.administrator.is_some() {
                guard.administrator = incoming.administrator;
            }
            for (arn, tags) in incoming.resource_tags {
                let entry = guard.resource_tags.entry(arn).or_default();
                for (k, v) in tags {
                    entry.insert(k, v);
                }
            }
            for (k, v) in incoming.custom_data_identifiers {
                guard.custom_data_identifiers.insert(k, v);
            }
            for (k, v) in incoming.allow_lists {
                guard.allow_lists.insert(k, v);
            }
            for (k, v) in incoming.findings_filters {
                guard.findings_filters.insert(k, v);
            }
            for (k, v) in incoming.classification_jobs {
                guard.classification_jobs.insert(k, v);
            }
            for (k, v) in incoming.sensitivity_inspection_templates {
                guard.sensitivity_inspection_templates.insert(k, v);
            }
            if incoming.automated_discovery_config.is_some() {
                guard.automated_discovery_config = incoming.automated_discovery_config;
            }
            for (k, v) in incoming.automated_discovery_accounts {
                guard.automated_discovery_accounts.insert(k, v);
            }
            if incoming.reveal_config.is_some() {
                guard.reveal_config = incoming.reveal_config;
            }
            if incoming.org_config.is_some() {
                guard.org_config = incoming.org_config;
            }
            if incoming.classification_export_config.is_some() {
                guard.classification_export_config = incoming.classification_export_config;
            }
            if incoming.findings_publication_config.is_some() {
                guard.findings_publication_config = incoming.findings_publication_config;
            }
            for (k, v) in incoming.findings {
                guard.findings.insert(k, v);
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
