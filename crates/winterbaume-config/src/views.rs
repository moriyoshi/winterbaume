//! Serde-compatible view types for Config state snapshots.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::ConfigService;
use crate::state::{ConfigState, EvaluationEntry};

/// Serializable view of the entire Config state for one account/region.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConfigStateView {
    #[serde(default)]
    pub recorders: HashMap<String, ConfigurationRecorderView>,
    #[serde(default)]
    pub delivery_channels: HashMap<String, DeliveryChannelView>,
    #[serde(default)]
    pub config_rules: HashMap<String, ConfigRuleView>,
    #[serde(default)]
    pub aggregation_authorizations: HashMap<String, AggregationAuthorizationView>,
    #[serde(default)]
    pub configuration_aggregators: HashMap<String, ConfigurationAggregatorView>,
    #[serde(default)]
    pub retention_configurations: HashMap<String, RetentionConfigurationView>,
    #[serde(default)]
    pub retention_counter: u32,
    #[serde(default)]
    pub organization_conformance_packs: HashMap<String, OrganizationConformancePackView>,
    /// Resource configs as list (tuple key `(resource_type, resource_id)` flattened).
    #[serde(default)]
    pub resource_configs: Vec<ResourceConfigView>,
    #[serde(default)]
    pub tags: HashMap<String, Vec<TagView>>,
    #[serde(default)]
    pub evaluations: Vec<EvaluationView>,
    #[serde(default)]
    pub remediation_configs: HashMap<String, RemediationConfigView>,
    #[serde(default)]
    pub organization_config_rules: HashMap<String, OrganizationConfigRuleView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigurationRecorderView {
    pub name: String,
    pub role_arn: String,
    pub recording_group: Option<RecordingGroupView>,
    pub recording: bool,
    pub last_start_time: Option<f64>,
    pub last_stop_time: Option<f64>,
    #[serde(default)]
    pub recording_mode: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordingGroupView {
    pub all_supported: bool,
    pub include_global_resource_types: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryChannelView {
    pub name: String,
    pub s3_bucket_name: String,
    pub s3_key_prefix: String,
    #[serde(default)]
    pub snapshot_delivery_properties: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigRuleView {
    pub config_rule_name: String,
    pub config_rule_arn: String,
    pub config_rule_id: String,
    pub config_rule_state: String,
    pub description: Option<String>,
    pub source_owner: String,
    pub source_identifier: Option<String>,
    pub input_parameters: Option<String>,
    pub maximum_execution_frequency: Option<String>,
    pub scope_resource_types: Option<Vec<String>>,
    #[serde(default)]
    pub evaluation_mode: Option<serde_json::Value>,
    #[serde(default)]
    pub scope: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregationAuthorizationView {
    pub authorized_account_id: String,
    pub authorized_aws_region: String,
    pub aggregation_authorization_arn: String,
    pub creation_time: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigurationAggregatorView {
    pub configuration_aggregator_name: String,
    pub configuration_aggregator_arn: String,
    pub account_aggregation_sources: Option<Vec<AccountAggregationSourceView>>,
    pub organization_aggregation_source: Option<OrganizationAggregationSourceView>,
    pub creation_time: f64,
    pub last_updated_time: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountAggregationSourceView {
    pub account_ids: Vec<String>,
    pub all_aws_regions: Option<bool>,
    pub aws_regions: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationAggregationSourceView {
    pub role_arn: String,
    pub all_aws_regions: Option<bool>,
    pub aws_regions: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetentionConfigurationView {
    pub name: String,
    pub retention_period_in_days: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationConformancePackView {
    pub organization_conformance_pack_name: String,
    pub organization_conformance_pack_arn: String,
    pub delivery_s3_bucket: Option<String>,
    pub delivery_s3_key_prefix: Option<String>,
    pub excluded_accounts: Option<Vec<String>>,
    /// Conformance pack input parameters as list of [key, value] pairs.
    pub conformance_pack_input_parameters: Option<Vec<[String; 2]>>,
    pub last_update_time: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceConfigView {
    pub resource_type: String,
    pub resource_id: String,
    pub schema_version_id: String,
    pub configuration: String,
    pub resource_name: Option<String>,
    pub tags: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagView {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationView {
    pub compliance_resource_type: String,
    pub compliance_resource_id: String,
    pub compliance_type: String,
    pub ordering_timestamp: f64,
    pub annotation: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemediationConfigView {
    pub config_rule_name: String,
    pub target_type: String,
    pub target_id: String,
    pub target_version: Option<String>,
    pub automatic: Option<bool>,
    pub maximum_automatic_attempts: Option<i32>,
    pub retry_attempt_seconds: Option<i64>,
    pub resource_type: Option<String>,
    pub parameters: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationConfigRuleView {
    pub organization_config_rule_name: String,
    pub organization_config_rule_arn: String,
    pub excluded_accounts: Option<Vec<String>>,
    pub last_update_time: f64,
    pub managed_rule_metadata: Option<serde_json::Value>,
    pub custom_rule_metadata: Option<serde_json::Value>,
    pub custom_policy_rule_metadata: Option<serde_json::Value>,
}

// --- From internal types to view types ---

impl From<&ConfigState> for ConfigStateView {
    fn from(state: &ConfigState) -> Self {
        ConfigStateView {
            recorders: state
                .recorders
                .iter()
                .map(|(k, r)| {
                    (
                        k.clone(),
                        ConfigurationRecorderView {
                            name: r.name.clone(),
                            role_arn: r.role_arn.clone(),
                            recording_group: r.recording_group.as_ref().map(|rg| {
                                RecordingGroupView {
                                    all_supported: rg.all_supported,
                                    include_global_resource_types: rg.include_global_resource_types,
                                }
                            }),
                            recording: r.recording,
                            last_start_time: r.last_start_time,
                            last_stop_time: r.last_stop_time,
                            recording_mode: r.recording_mode.clone(),
                        },
                    )
                })
                .collect(),
            delivery_channels: state
                .delivery_channels
                .iter()
                .map(|(k, dc)| {
                    (
                        k.clone(),
                        DeliveryChannelView {
                            name: dc.name.clone(),
                            s3_bucket_name: dc.s3_bucket_name.clone(),
                            s3_key_prefix: dc.s3_key_prefix.clone(),
                            snapshot_delivery_properties: dc.snapshot_delivery_properties.clone(),
                        },
                    )
                })
                .collect(),
            config_rules: state
                .config_rules
                .iter()
                .map(|(k, cr)| {
                    (
                        k.clone(),
                        ConfigRuleView {
                            config_rule_name: cr.config_rule_name.clone(),
                            config_rule_arn: cr.config_rule_arn.clone(),
                            config_rule_id: cr.config_rule_id.clone(),
                            config_rule_state: cr.config_rule_state.clone(),
                            description: cr.description.clone(),
                            source_owner: cr.source_owner.clone(),
                            source_identifier: cr.source_identifier.clone(),
                            input_parameters: cr.input_parameters.clone(),
                            maximum_execution_frequency: cr.maximum_execution_frequency.clone(),
                            scope_resource_types: cr.scope_resource_types.clone(),
                            evaluation_mode: cr.evaluation_mode.clone(),
                            scope: cr.scope.clone(),
                        },
                    )
                })
                .collect(),
            aggregation_authorizations: state
                .aggregation_authorizations
                .iter()
                .map(|(k, aa)| {
                    (
                        k.clone(),
                        AggregationAuthorizationView {
                            authorized_account_id: aa.authorized_account_id.clone(),
                            authorized_aws_region: aa.authorized_aws_region.clone(),
                            aggregation_authorization_arn: aa.aggregation_authorization_arn.clone(),
                            creation_time: aa.creation_time,
                        },
                    )
                })
                .collect(),
            configuration_aggregators: state
                .configuration_aggregators
                .iter()
                .map(|(k, ca)| {
                    (
                        k.clone(),
                        ConfigurationAggregatorView {
                            configuration_aggregator_name: ca.configuration_aggregator_name.clone(),
                            configuration_aggregator_arn: ca.configuration_aggregator_arn.clone(),
                            account_aggregation_sources: ca
                                .account_aggregation_sources
                                .as_ref()
                                .map(|srcs| {
                                    srcs.iter()
                                        .map(|s| AccountAggregationSourceView {
                                            account_ids: s.account_ids.clone(),
                                            all_aws_regions: s.all_aws_regions,
                                            aws_regions: s.aws_regions.clone(),
                                        })
                                        .collect()
                                }),
                            organization_aggregation_source: ca
                                .organization_aggregation_source
                                .as_ref()
                                .map(|oas| OrganizationAggregationSourceView {
                                    role_arn: oas.role_arn.clone(),
                                    all_aws_regions: oas.all_aws_regions,
                                    aws_regions: oas.aws_regions.clone(),
                                }),
                            creation_time: ca.creation_time,
                            last_updated_time: ca.last_updated_time,
                        },
                    )
                })
                .collect(),
            retention_configurations: state
                .retention_configurations
                .iter()
                .map(|(k, rc)| {
                    (
                        k.clone(),
                        RetentionConfigurationView {
                            name: rc.name.clone(),
                            retention_period_in_days: rc.retention_period_in_days,
                        },
                    )
                })
                .collect(),
            retention_counter: state.retention_counter,
            organization_conformance_packs: state
                .organization_conformance_packs
                .iter()
                .map(|(k, ocp)| {
                    (
                        k.clone(),
                        OrganizationConformancePackView {
                            organization_conformance_pack_name: ocp
                                .organization_conformance_pack_name
                                .clone(),
                            organization_conformance_pack_arn: ocp
                                .organization_conformance_pack_arn
                                .clone(),
                            delivery_s3_bucket: ocp.delivery_s3_bucket.clone(),
                            delivery_s3_key_prefix: ocp.delivery_s3_key_prefix.clone(),
                            excluded_accounts: ocp.excluded_accounts.clone(),
                            conformance_pack_input_parameters: ocp
                                .conformance_pack_input_parameters
                                .as_ref()
                                .map(|params| {
                                    params.iter().map(|(k, v)| [k.clone(), v.clone()]).collect()
                                }),
                            last_update_time: ocp.last_update_time,
                        },
                    )
                })
                .collect(),
            resource_configs: state
                .resource_configs
                .values()
                .map(|rc| ResourceConfigView {
                    resource_type: rc.resource_type.clone(),
                    resource_id: rc.resource_id.clone(),
                    schema_version_id: rc.schema_version_id.clone(),
                    configuration: rc.configuration.clone(),
                    resource_name: rc.resource_name.clone(),
                    tags: rc.tags.clone(),
                })
                .collect(),
            tags: state
                .tags
                .iter()
                .map(|(k, ts)| {
                    (
                        k.clone(),
                        ts.iter()
                            .map(|t| TagView {
                                key: t.key.clone(),
                                value: t.value.clone(),
                            })
                            .collect(),
                    )
                })
                .collect(),
            evaluations: state
                .evaluations
                .iter()
                .map(|e| EvaluationView {
                    compliance_resource_type: e.compliance_resource_type.clone(),
                    compliance_resource_id: e.compliance_resource_id.clone(),
                    compliance_type: e.compliance_type.clone(),
                    ordering_timestamp: e.ordering_timestamp,
                    annotation: e.annotation.clone(),
                })
                .collect(),
            remediation_configs: state
                .remediation_configs
                .iter()
                .map(|(k, rc)| {
                    (
                        k.clone(),
                        RemediationConfigView {
                            config_rule_name: rc.config_rule_name.clone(),
                            target_type: rc.target_type.clone(),
                            target_id: rc.target_id.clone(),
                            target_version: rc.target_version.clone(),
                            automatic: rc.automatic,
                            maximum_automatic_attempts: rc.maximum_automatic_attempts,
                            retry_attempt_seconds: rc.retry_attempt_seconds,
                            resource_type: rc.resource_type.clone(),
                            parameters: rc.parameters.clone(),
                        },
                    )
                })
                .collect(),
            organization_config_rules: state
                .organization_config_rules
                .iter()
                .map(|(k, r)| {
                    (
                        k.clone(),
                        OrganizationConfigRuleView {
                            organization_config_rule_name: r.organization_config_rule_name.clone(),
                            organization_config_rule_arn: r.organization_config_rule_arn.clone(),
                            excluded_accounts: r.excluded_accounts.clone(),
                            last_update_time: r.last_update_time,
                            managed_rule_metadata: r.managed_rule_metadata.clone(),
                            custom_rule_metadata: r.custom_rule_metadata.clone(),
                            custom_policy_rule_metadata: r.custom_policy_rule_metadata.clone(),
                        },
                    )
                })
                .collect(),
        }
    }
}

// --- From view types to internal types ---

impl From<ConfigStateView> for ConfigState {
    fn from(view: ConfigStateView) -> Self {
        let mut state = ConfigState {
            retention_counter: view.retention_counter,
            ..ConfigState::default()
        };
        for (k, r) in view.recorders {
            state.recorders.insert(
                k,
                crate::types::ConfigurationRecorder {
                    name: r.name,
                    role_arn: r.role_arn,
                    recording_group: r.recording_group.map(|rg| crate::types::RecordingGroup {
                        all_supported: rg.all_supported,
                        include_global_resource_types: rg.include_global_resource_types,
                    }),
                    recording: r.recording,
                    last_start_time: r.last_start_time,
                    last_stop_time: r.last_stop_time,
                    recording_mode: r.recording_mode,
                },
            );
        }
        for (k, dc) in view.delivery_channels {
            state.delivery_channels.insert(
                k,
                crate::types::DeliveryChannel {
                    name: dc.name,
                    s3_bucket_name: dc.s3_bucket_name,
                    s3_key_prefix: dc.s3_key_prefix,
                    snapshot_delivery_properties: dc.snapshot_delivery_properties,
                },
            );
        }
        for (k, cr) in view.config_rules {
            state.config_rules.insert(
                k,
                crate::types::ConfigRule {
                    config_rule_name: cr.config_rule_name,
                    config_rule_arn: cr.config_rule_arn,
                    config_rule_id: cr.config_rule_id,
                    config_rule_state: cr.config_rule_state,
                    description: cr.description,
                    source_owner: cr.source_owner,
                    source_identifier: cr.source_identifier,
                    input_parameters: cr.input_parameters,
                    maximum_execution_frequency: cr.maximum_execution_frequency,
                    scope_resource_types: cr.scope_resource_types,
                    evaluation_mode: cr.evaluation_mode,
                    scope: cr.scope,
                },
            );
        }
        for (k, aa) in view.aggregation_authorizations {
            state.aggregation_authorizations.insert(
                k,
                crate::types::AggregationAuthorizationEntry {
                    authorized_account_id: aa.authorized_account_id,
                    authorized_aws_region: aa.authorized_aws_region,
                    aggregation_authorization_arn: aa.aggregation_authorization_arn,
                    creation_time: aa.creation_time,
                },
            );
        }
        for (k, ca) in view.configuration_aggregators {
            state.configuration_aggregators.insert(
                k,
                crate::types::ConfigurationAggregatorEntry {
                    configuration_aggregator_name: ca.configuration_aggregator_name,
                    configuration_aggregator_arn: ca.configuration_aggregator_arn,
                    account_aggregation_sources: ca.account_aggregation_sources.map(|srcs| {
                        srcs.into_iter()
                            .map(|s| crate::types::AccountAggregationSourceEntry {
                                account_ids: s.account_ids,
                                all_aws_regions: s.all_aws_regions,
                                aws_regions: s.aws_regions,
                            })
                            .collect()
                    }),
                    organization_aggregation_source: ca.organization_aggregation_source.map(
                        |oas| crate::types::OrganizationAggregationSourceEntry {
                            role_arn: oas.role_arn,
                            all_aws_regions: oas.all_aws_regions,
                            aws_regions: oas.aws_regions,
                        },
                    ),
                    creation_time: ca.creation_time,
                    last_updated_time: ca.last_updated_time,
                },
            );
        }
        for (k, rc) in view.retention_configurations {
            state.retention_configurations.insert(
                k,
                crate::types::RetentionConfigurationEntry {
                    name: rc.name,
                    retention_period_in_days: rc.retention_period_in_days,
                },
            );
        }
        for (k, ocp) in view.organization_conformance_packs {
            state.organization_conformance_packs.insert(
                k,
                crate::types::OrganizationConformancePackEntry {
                    organization_conformance_pack_name: ocp.organization_conformance_pack_name,
                    organization_conformance_pack_arn: ocp.organization_conformance_pack_arn,
                    delivery_s3_bucket: ocp.delivery_s3_bucket,
                    delivery_s3_key_prefix: ocp.delivery_s3_key_prefix,
                    excluded_accounts: ocp.excluded_accounts,
                    conformance_pack_input_parameters: ocp
                        .conformance_pack_input_parameters
                        .map(|params| params.into_iter().map(|[k, v]| (k, v)).collect()),
                    last_update_time: ocp.last_update_time,
                },
            );
        }
        for rc in view.resource_configs {
            let key = (rc.resource_type.clone(), rc.resource_id.clone());
            state.resource_configs.insert(
                key,
                crate::types::ResourceConfigEntry {
                    resource_type: rc.resource_type,
                    resource_id: rc.resource_id,
                    schema_version_id: rc.schema_version_id,
                    configuration: rc.configuration,
                    resource_name: rc.resource_name,
                    tags: rc.tags,
                },
            );
        }
        for (k, ts) in view.tags {
            state.tags.insert(
                k,
                ts.into_iter()
                    .map(|t| crate::types::TagEntry {
                        key: t.key,
                        value: t.value,
                    })
                    .collect(),
            );
        }
        state.evaluations = view
            .evaluations
            .into_iter()
            .map(|e| EvaluationEntry {
                compliance_resource_type: e.compliance_resource_type,
                compliance_resource_id: e.compliance_resource_id,
                compliance_type: e.compliance_type,
                ordering_timestamp: e.ordering_timestamp,
                annotation: e.annotation,
            })
            .collect();
        for (k, rc) in view.remediation_configs {
            state.remediation_configs.insert(
                k,
                crate::types::RemediationConfigEntry {
                    config_rule_name: rc.config_rule_name,
                    target_type: rc.target_type,
                    target_id: rc.target_id,
                    target_version: rc.target_version,
                    automatic: rc.automatic,
                    maximum_automatic_attempts: rc.maximum_automatic_attempts,
                    retry_attempt_seconds: rc.retry_attempt_seconds,
                    resource_type: rc.resource_type,
                    parameters: rc.parameters,
                },
            );
        }
        for (k, r) in view.organization_config_rules {
            state.organization_config_rules.insert(
                k,
                crate::types::OrganizationConfigRuleEntry {
                    organization_config_rule_name: r.organization_config_rule_name,
                    organization_config_rule_arn: r.organization_config_rule_arn,
                    excluded_accounts: r.excluded_accounts,
                    last_update_time: r.last_update_time,
                    managed_rule_metadata: r.managed_rule_metadata,
                    custom_rule_metadata: r.custom_rule_metadata,
                    custom_policy_rule_metadata: r.custom_policy_rule_metadata,
                },
            );
        }
        state
    }
}

// --- StatefulService implementation ---

impl StatefulService for ConfigService {
    type StateView = ConfigStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        ConfigStateView::from(&*guard)
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
            *guard = ConfigState::from(view);
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
            let merged = ConfigState::from(view);
            for (k, v) in merged.recorders {
                guard.recorders.insert(k, v);
            }
            for (k, v) in merged.delivery_channels {
                guard.delivery_channels.insert(k, v);
            }
            for (k, v) in merged.config_rules {
                guard.config_rules.insert(k, v);
            }
            for (k, v) in merged.aggregation_authorizations {
                guard.aggregation_authorizations.insert(k, v);
            }
            for (k, v) in merged.configuration_aggregators {
                guard.configuration_aggregators.insert(k, v);
            }
            for (k, v) in merged.retention_configurations {
                guard.retention_configurations.insert(k, v);
            }
            for (k, v) in merged.organization_conformance_packs {
                guard.organization_conformance_packs.insert(k, v);
            }
            for (k, v) in merged.resource_configs {
                guard.resource_configs.insert(k, v);
            }
            for (k, v) in merged.tags {
                guard.tags.insert(k, v);
            }
            guard.evaluations.extend(merged.evaluations);
            for (k, v) in merged.remediation_configs {
                guard.remediation_configs.insert(k, v);
            }
            for (k, v) in merged.organization_config_rules {
                guard.organization_config_rules.insert(k, v);
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
