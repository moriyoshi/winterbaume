//! Serde-compatible view types for IoT state snapshots.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::IotService;
use crate::state::IotState;
use crate::types::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IotStateView {
    #[serde(default)]
    pub things: HashMap<String, ThingView>,
    #[serde(default)]
    pub thing_types: HashMap<String, ThingTypeView>,
    #[serde(default)]
    pub thing_groups: HashMap<String, ThingGroupView>,
    #[serde(default)]
    pub billing_groups: HashMap<String, BillingGroupView>,
    #[serde(default)]
    pub certificates: HashMap<String, CertificateView>,
    #[serde(default)]
    pub ca_certificates: HashMap<String, CACertificateView>,
    #[serde(default)]
    pub policies: HashMap<String, IotPolicyView>,
    #[serde(default)]
    pub role_aliases: HashMap<String, RoleAliasView>,
    #[serde(default)]
    pub domain_configurations: HashMap<String, DomainConfigurationView>,
    #[serde(default)]
    pub jobs: HashMap<String, IotJobView>,
    #[serde(default)]
    pub job_templates: HashMap<String, JobTemplateView>,
    #[serde(default)]
    pub topic_rules: HashMap<String, TopicRuleView>,
    #[serde(default)]
    pub registration_code: String,
    pub indexing_config_thing: Option<Value>,
    pub indexing_config_thing_group: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThingView {
    pub thing_name: String,
    pub thing_id: String,
    pub thing_type_name: Option<String>,
    #[serde(default)]
    pub attributes: HashMap<String, String>,
    pub version: i64,
    pub thing_arn: String,
    pub billing_group_name: Option<String>,
    #[serde(default)]
    pub principals: Vec<String>,
    #[serde(default)]
    pub thing_groups: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThingTypeView {
    pub thing_type_name: String,
    pub thing_type_id: String,
    pub thing_type_arn: String,
    pub thing_type_description: Option<String>,
    pub searchable_attributes: Option<Vec<String>>,
    pub creation_date: f64,
    pub deprecated: bool,
    pub deprecation_date: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThingGroupView {
    pub thing_group_name: String,
    pub thing_group_id: String,
    pub thing_group_arn: String,
    pub parent_group_name: Option<String>,
    pub thing_group_description: Option<String>,
    #[serde(default)]
    pub attributes: HashMap<String, String>,
    pub version: i64,
    pub creation_date: f64,
    #[serde(default)]
    pub things: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingGroupView {
    pub billing_group_name: String,
    pub billing_group_id: String,
    pub billing_group_arn: String,
    pub billing_group_description: Option<String>,
    pub version: i64,
    pub creation_date: f64,
    #[serde(default)]
    pub things: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificateView {
    pub certificate_id: String,
    pub certificate_arn: String,
    pub certificate_pem: String,
    pub status: String,
    pub creation_date: f64,
    pub ca_certificate_id: Option<String>,
    pub owned_by: String,
    pub mode: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(clippy::upper_case_acronyms)]
pub struct CACertificateView {
    pub certificate_id: String,
    pub certificate_arn: String,
    pub certificate_pem: String,
    pub status: String,
    pub auto_registration_status: String,
    pub creation_date: f64,
    pub owned_by: String,
    pub mode: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyVersionDataView {
    pub version_id: String,
    pub policy_document: String,
    pub is_default: bool,
    pub create_date: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IotPolicyView {
    pub policy_name: String,
    pub policy_arn: String,
    pub policy_document: String,
    pub creation_date: f64,
    pub last_modified_date: f64,
    pub generation_id: String,
    #[serde(default)]
    pub versions: Vec<PolicyVersionDataView>,
    pub default_version_id: String,
    #[serde(default)]
    pub targets: Vec<String>,
    #[serde(default)]
    pub principals: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleAliasView {
    pub role_alias: String,
    pub role_alias_arn: String,
    pub role_arn: String,
    pub credential_duration_seconds: i32,
    pub creation_date: f64,
    pub last_modified_date: f64,
    pub owner: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainConfigurationView {
    pub domain_configuration_name: String,
    pub domain_configuration_arn: String,
    pub domain_name: Option<String>,
    pub domain_configuration_status: String,
    pub service_type: Option<String>,
    pub creation_date: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobExecDataView {
    pub status: String,
    pub queued_at: f64,
    pub started_at: Option<f64>,
    pub last_updated_at: f64,
    pub execution_number: i64,
    pub version_number: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IotJobView {
    pub job_id: String,
    pub job_arn: String,
    pub description: Option<String>,
    #[serde(default)]
    pub targets: Vec<String>,
    pub document: Option<String>,
    pub document_source: Option<String>,
    pub status: String,
    pub target_selection: Option<String>,
    pub creation_date: f64,
    pub last_updated_date: f64,
    pub completed_date: Option<f64>,
    pub comment: Option<String>,
    pub reason_code: Option<String>,
    pub force_canceled: Option<bool>,
    pub job_template_arn: Option<String>,
    #[serde(default)]
    pub executions: HashMap<String, JobExecDataView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobTemplateView {
    pub job_template_id: String,
    pub job_template_arn: String,
    pub description: String,
    pub document: Option<String>,
    pub document_source: Option<String>,
    pub creation_date: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopicRuleView {
    pub rule_name: String,
    pub rule_arn: String,
    pub sql: String,
    pub description: Option<String>,
    pub rule_disabled: bool,
    pub creation_date: f64,
    pub actions_json: Value,
    pub error_action_json: Option<Value>,
    pub aws_iot_sql_version: Option<String>,
}

// --- From internal types to view types ---

impl From<&IotState> for IotStateView {
    fn from(state: &IotState) -> Self {
        IotStateView {
            things: state
                .things
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        ThingView {
                            thing_name: v.thing_name.clone(),
                            thing_id: v.thing_id.clone(),
                            thing_type_name: v.thing_type_name.clone(),
                            attributes: v.attributes.clone(),
                            version: v.version,
                            thing_arn: v.thing_arn.clone(),
                            billing_group_name: v.billing_group_name.clone(),
                            principals: v.principals.clone(),
                            thing_groups: v.thing_groups.clone(),
                        },
                    )
                })
                .collect(),
            thing_types: state
                .thing_types
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        ThingTypeView {
                            thing_type_name: v.thing_type_name.clone(),
                            thing_type_id: v.thing_type_id.clone(),
                            thing_type_arn: v.thing_type_arn.clone(),
                            thing_type_description: v.thing_type_description.clone(),
                            searchable_attributes: v.searchable_attributes.clone(),
                            creation_date: v.creation_date,
                            deprecated: v.deprecated,
                            deprecation_date: v.deprecation_date,
                        },
                    )
                })
                .collect(),
            thing_groups: state
                .thing_groups
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        ThingGroupView {
                            thing_group_name: v.thing_group_name.clone(),
                            thing_group_id: v.thing_group_id.clone(),
                            thing_group_arn: v.thing_group_arn.clone(),
                            parent_group_name: v.parent_group_name.clone(),
                            thing_group_description: v.thing_group_description.clone(),
                            attributes: v.attributes.clone(),
                            version: v.version,
                            creation_date: v.creation_date,
                            things: v.things.clone(),
                        },
                    )
                })
                .collect(),
            billing_groups: state
                .billing_groups
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        BillingGroupView {
                            billing_group_name: v.billing_group_name.clone(),
                            billing_group_id: v.billing_group_id.clone(),
                            billing_group_arn: v.billing_group_arn.clone(),
                            billing_group_description: v.billing_group_description.clone(),
                            version: v.version,
                            creation_date: v.creation_date,
                            things: v.things.clone(),
                        },
                    )
                })
                .collect(),
            certificates: state
                .certificates
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        CertificateView {
                            certificate_id: v.certificate_id.clone(),
                            certificate_arn: v.certificate_arn.clone(),
                            certificate_pem: v.certificate_pem.clone(),
                            status: v.status.clone(),
                            creation_date: v.creation_date,
                            ca_certificate_id: v.ca_certificate_id.clone(),
                            owned_by: v.owned_by.clone(),
                            mode: v.mode.clone(),
                        },
                    )
                })
                .collect(),
            ca_certificates: state
                .ca_certificates
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        CACertificateView {
                            certificate_id: v.certificate_id.clone(),
                            certificate_arn: v.certificate_arn.clone(),
                            certificate_pem: v.certificate_pem.clone(),
                            status: v.status.clone(),
                            auto_registration_status: v.auto_registration_status.clone(),
                            creation_date: v.creation_date,
                            owned_by: v.owned_by.clone(),
                            mode: v.mode.clone(),
                        },
                    )
                })
                .collect(),
            policies: state
                .policies
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        IotPolicyView {
                            policy_name: v.policy_name.clone(),
                            policy_arn: v.policy_arn.clone(),
                            policy_document: v.policy_document.clone(),
                            creation_date: v.creation_date,
                            last_modified_date: v.last_modified_date,
                            generation_id: v.generation_id.clone(),
                            versions: v
                                .versions
                                .iter()
                                .map(|pv| PolicyVersionDataView {
                                    version_id: pv.version_id.clone(),
                                    policy_document: pv.policy_document.clone(),
                                    is_default: pv.is_default,
                                    create_date: pv.create_date,
                                })
                                .collect(),
                            default_version_id: v.default_version_id.clone(),
                            targets: v.targets.clone(),
                            principals: v.principals.clone(),
                        },
                    )
                })
                .collect(),
            role_aliases: state
                .role_aliases
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        RoleAliasView {
                            role_alias: v.role_alias.clone(),
                            role_alias_arn: v.role_alias_arn.clone(),
                            role_arn: v.role_arn.clone(),
                            credential_duration_seconds: v.credential_duration_seconds,
                            creation_date: v.creation_date,
                            last_modified_date: v.last_modified_date,
                            owner: v.owner.clone(),
                        },
                    )
                })
                .collect(),
            domain_configurations: state
                .domain_configurations
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        DomainConfigurationView {
                            domain_configuration_name: v.domain_configuration_name.clone(),
                            domain_configuration_arn: v.domain_configuration_arn.clone(),
                            domain_name: v.domain_name.clone(),
                            domain_configuration_status: v.domain_configuration_status.clone(),
                            service_type: v.service_type.clone(),
                            creation_date: v.creation_date,
                        },
                    )
                })
                .collect(),
            jobs: state
                .jobs
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        IotJobView {
                            job_id: v.job_id.clone(),
                            job_arn: v.job_arn.clone(),
                            description: v.description.clone(),
                            targets: v.targets.clone(),
                            document: v.document.clone(),
                            document_source: v.document_source.clone(),
                            status: v.status.clone(),
                            target_selection: v.target_selection.clone(),
                            creation_date: v.creation_date,
                            last_updated_date: v.last_updated_date,
                            completed_date: v.completed_date,
                            comment: v.comment.clone(),
                            reason_code: v.reason_code.clone(),
                            force_canceled: v.force_canceled,
                            job_template_arn: v.job_template_arn.clone(),
                            executions: v
                                .executions
                                .iter()
                                .map(|(ek, ev)| {
                                    (
                                        ek.clone(),
                                        JobExecDataView {
                                            status: ev.status.clone(),
                                            queued_at: ev.queued_at,
                                            started_at: ev.started_at,
                                            last_updated_at: ev.last_updated_at,
                                            execution_number: ev.execution_number,
                                            version_number: ev.version_number,
                                        },
                                    )
                                })
                                .collect(),
                        },
                    )
                })
                .collect(),
            job_templates: state
                .job_templates
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        JobTemplateView {
                            job_template_id: v.job_template_id.clone(),
                            job_template_arn: v.job_template_arn.clone(),
                            description: v.description.clone(),
                            document: v.document.clone(),
                            document_source: v.document_source.clone(),
                            creation_date: v.creation_date,
                        },
                    )
                })
                .collect(),
            topic_rules: state
                .topic_rules
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        TopicRuleView {
                            rule_name: v.rule_name.clone(),
                            rule_arn: v.rule_arn.clone(),
                            sql: v.sql.clone(),
                            description: v.description.clone(),
                            rule_disabled: v.rule_disabled,
                            creation_date: v.creation_date,
                            actions_json: v.actions_json.clone(),
                            error_action_json: v.error_action_json.clone(),
                            aws_iot_sql_version: v.aws_iot_sql_version.clone(),
                        },
                    )
                })
                .collect(),
            registration_code: state.registration_code.clone(),
            indexing_config_thing: state.indexing_config_thing.clone(),
            indexing_config_thing_group: state.indexing_config_thing_group.clone(),
        }
    }
}

// --- From view types to internal types ---

impl From<IotStateView> for IotState {
    fn from(view: IotStateView) -> Self {
        let mut state = IotState::default();
        state.registration_code = view.registration_code;
        state.indexing_config_thing = view.indexing_config_thing;
        state.indexing_config_thing_group = view.indexing_config_thing_group;
        state.things = view
            .things
            .into_iter()
            .map(|(k, v)| {
                (
                    k,
                    Thing {
                        thing_name: v.thing_name,
                        thing_id: v.thing_id,
                        thing_type_name: v.thing_type_name,
                        attributes: v.attributes,
                        version: v.version,
                        thing_arn: v.thing_arn,
                        billing_group_name: v.billing_group_name,
                        principals: v.principals,
                        thing_groups: v.thing_groups,
                    },
                )
            })
            .collect();
        state.thing_types = view
            .thing_types
            .into_iter()
            .map(|(k, v)| {
                (
                    k,
                    ThingType {
                        thing_type_name: v.thing_type_name,
                        thing_type_id: v.thing_type_id,
                        thing_type_arn: v.thing_type_arn,
                        thing_type_description: v.thing_type_description,
                        searchable_attributes: v.searchable_attributes,
                        creation_date: v.creation_date,
                        deprecated: v.deprecated,
                        deprecation_date: v.deprecation_date,
                    },
                )
            })
            .collect();
        state.thing_groups = view
            .thing_groups
            .into_iter()
            .map(|(k, v)| {
                (
                    k,
                    ThingGroup {
                        thing_group_name: v.thing_group_name,
                        thing_group_id: v.thing_group_id,
                        thing_group_arn: v.thing_group_arn,
                        parent_group_name: v.parent_group_name,
                        thing_group_description: v.thing_group_description,
                        attributes: v.attributes,
                        version: v.version,
                        creation_date: v.creation_date,
                        things: v.things,
                    },
                )
            })
            .collect();
        state.billing_groups = view
            .billing_groups
            .into_iter()
            .map(|(k, v)| {
                (
                    k,
                    BillingGroup {
                        billing_group_name: v.billing_group_name,
                        billing_group_id: v.billing_group_id,
                        billing_group_arn: v.billing_group_arn,
                        billing_group_description: v.billing_group_description,
                        version: v.version,
                        creation_date: v.creation_date,
                        things: v.things,
                    },
                )
            })
            .collect();
        state.certificates = view
            .certificates
            .into_iter()
            .map(|(k, v)| {
                (
                    k,
                    Certificate {
                        certificate_id: v.certificate_id,
                        certificate_arn: v.certificate_arn,
                        certificate_pem: v.certificate_pem,
                        status: v.status,
                        creation_date: v.creation_date,
                        ca_certificate_id: v.ca_certificate_id,
                        owned_by: v.owned_by,
                        mode: v.mode,
                    },
                )
            })
            .collect();
        state.ca_certificates = view
            .ca_certificates
            .into_iter()
            .map(|(k, v)| {
                (
                    k,
                    CACertificate {
                        certificate_id: v.certificate_id,
                        certificate_arn: v.certificate_arn,
                        certificate_pem: v.certificate_pem,
                        status: v.status,
                        auto_registration_status: v.auto_registration_status,
                        creation_date: v.creation_date,
                        owned_by: v.owned_by,
                        mode: v.mode,
                    },
                )
            })
            .collect();
        state.policies = view
            .policies
            .into_iter()
            .map(|(k, v)| {
                (
                    k,
                    IotPolicy {
                        policy_name: v.policy_name,
                        policy_arn: v.policy_arn,
                        policy_document: v.policy_document,
                        creation_date: v.creation_date,
                        last_modified_date: v.last_modified_date,
                        generation_id: v.generation_id,
                        versions: v
                            .versions
                            .into_iter()
                            .map(|pv| PolicyVersionData {
                                version_id: pv.version_id,
                                policy_document: pv.policy_document,
                                is_default: pv.is_default,
                                create_date: pv.create_date,
                            })
                            .collect(),
                        default_version_id: v.default_version_id,
                        targets: v.targets,
                        principals: v.principals,
                    },
                )
            })
            .collect();
        state.role_aliases = view
            .role_aliases
            .into_iter()
            .map(|(k, v)| {
                (
                    k,
                    RoleAlias {
                        role_alias: v.role_alias,
                        role_alias_arn: v.role_alias_arn,
                        role_arn: v.role_arn,
                        credential_duration_seconds: v.credential_duration_seconds,
                        creation_date: v.creation_date,
                        last_modified_date: v.last_modified_date,
                        owner: v.owner,
                    },
                )
            })
            .collect();
        state.domain_configurations = view
            .domain_configurations
            .into_iter()
            .map(|(k, v)| {
                (
                    k,
                    DomainConfiguration {
                        domain_configuration_name: v.domain_configuration_name,
                        domain_configuration_arn: v.domain_configuration_arn,
                        domain_name: v.domain_name,
                        domain_configuration_status: v.domain_configuration_status,
                        service_type: v.service_type,
                        creation_date: v.creation_date,
                    },
                )
            })
            .collect();
        state.jobs = view
            .jobs
            .into_iter()
            .map(|(k, v)| {
                (
                    k,
                    IotJob {
                        job_id: v.job_id,
                        job_arn: v.job_arn,
                        description: v.description,
                        targets: v.targets,
                        document: v.document,
                        document_source: v.document_source,
                        status: v.status,
                        target_selection: v.target_selection,
                        creation_date: v.creation_date,
                        last_updated_date: v.last_updated_date,
                        completed_date: v.completed_date,
                        comment: v.comment,
                        reason_code: v.reason_code,
                        force_canceled: v.force_canceled,
                        job_template_arn: v.job_template_arn,
                        executions: v
                            .executions
                            .into_iter()
                            .map(|(ek, ev)| {
                                (
                                    ek,
                                    JobExecData {
                                        status: ev.status,
                                        queued_at: ev.queued_at,
                                        started_at: ev.started_at,
                                        last_updated_at: ev.last_updated_at,
                                        execution_number: ev.execution_number,
                                        version_number: ev.version_number,
                                    },
                                )
                            })
                            .collect(),
                    },
                )
            })
            .collect();
        state.job_templates = view
            .job_templates
            .into_iter()
            .map(|(k, v)| {
                (
                    k,
                    JobTemplate {
                        job_template_id: v.job_template_id,
                        job_template_arn: v.job_template_arn,
                        description: v.description,
                        document: v.document,
                        document_source: v.document_source,
                        creation_date: v.creation_date,
                    },
                )
            })
            .collect();
        state.topic_rules = view
            .topic_rules
            .into_iter()
            .map(|(k, v)| {
                (
                    k,
                    TopicRule {
                        rule_name: v.rule_name,
                        rule_arn: v.rule_arn,
                        sql: v.sql,
                        description: v.description,
                        rule_disabled: v.rule_disabled,
                        creation_date: v.creation_date,
                        actions_json: v.actions_json,
                        error_action_json: v.error_action_json,
                        aws_iot_sql_version: v.aws_iot_sql_version,
                    },
                )
            })
            .collect();
        state
    }
}

// --- StatefulService implementation ---

impl StatefulService for IotService {
    type StateView = IotStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        IotStateView::from(&*guard)
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
            *guard = IotState::from(view);
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
            let incoming = IotState::from(view);
            for (k, v) in incoming.things {
                guard.things.insert(k, v);
            }
            for (k, v) in incoming.thing_types {
                guard.thing_types.insert(k, v);
            }
            for (k, v) in incoming.thing_groups {
                guard.thing_groups.insert(k, v);
            }
            for (k, v) in incoming.billing_groups {
                guard.billing_groups.insert(k, v);
            }
            for (k, v) in incoming.certificates {
                guard.certificates.insert(k, v);
            }
            for (k, v) in incoming.ca_certificates {
                guard.ca_certificates.insert(k, v);
            }
            for (k, v) in incoming.policies {
                guard.policies.insert(k, v);
            }
            for (k, v) in incoming.role_aliases {
                guard.role_aliases.insert(k, v);
            }
            for (k, v) in incoming.domain_configurations {
                guard.domain_configurations.insert(k, v);
            }
            for (k, v) in incoming.jobs {
                guard.jobs.insert(k, v);
            }
            for (k, v) in incoming.job_templates {
                guard.job_templates.insert(k, v);
            }
            for (k, v) in incoming.topic_rules {
                guard.topic_rules.insert(k, v);
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
