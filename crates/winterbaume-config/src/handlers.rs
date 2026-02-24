use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
    json_error_response,
};

use crate::state::{ConfigError, ConfigState, EvaluationEntry};
use crate::types::*;
use crate::views::ConfigStateView;
use crate::wire;

pub struct ConfigService {
    pub(crate) state: Arc<BackendState<ConfigState>>,
    pub(crate) notifier: StateChangeNotifier<ConfigStateView>,
}

impl ConfigService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for ConfigService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for ConfigService {
    fn service_name(&self) -> &str {
        "config"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://config\..*\.amazonaws\.com",
            r"https?://config\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl ConfigService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = DEFAULT_ACCOUNT_ID;

        let action = request
            .headers
            .get("x-amz-target")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.split('.').next_back())
            .map(|s| s.to_string());

        let action = match action {
            Some(a) => a,
            None => {
                return json_error_response(400, "MissingAction", "Missing X-Amz-Target header");
            }
        };

        if serde_json::from_slice::<Value>(&request.body).is_err() {
            return json_error_response(400, "SerializationException", "Invalid JSON body");
        }
        let body_bytes: &[u8] = &request.body;

        let state = self.state.get(account_id, &region);

        match action.as_str() {
            "PutConfigurationRecorder" => {
                self.handle_put_configuration_recorder(&state, body_bytes)
                    .await
            }
            "DescribeConfigurationRecorders" => {
                self.handle_describe_configuration_recorders(&state, body_bytes)
                    .await
            }
            "DeleteConfigurationRecorder" => {
                self.handle_delete_configuration_recorder(&state, body_bytes)
                    .await
            }
            "PutDeliveryChannel" => self.handle_put_delivery_channel(&state, body_bytes).await,
            "DeleteDeliveryChannel" => {
                self.handle_delete_delivery_channel(&state, body_bytes)
                    .await
            }
            "DescribeDeliveryChannels" => {
                self.handle_describe_delivery_channels(&state, body_bytes)
                    .await
            }
            "PutConfigRule" => {
                self.handle_put_config_rule(&state, body_bytes, account_id, &region)
                    .await
            }
            "DescribeConfigRules" => self.handle_describe_config_rules(&state, body_bytes).await,
            "DeleteConfigRule" => self.handle_delete_config_rule(&state, body_bytes).await,
            "PutAggregationAuthorization" => {
                self.handle_put_aggregation_authorization(&state, body_bytes, account_id)
                    .await
            }
            "DescribeAggregationAuthorizations" => {
                self.handle_describe_aggregation_authorizations(&state)
                    .await
            }
            "DeleteAggregationAuthorization" => {
                self.handle_delete_aggregation_authorization(&state, body_bytes)
                    .await
            }
            "PutConfigurationAggregator" => {
                self.handle_put_configuration_aggregator(&state, body_bytes, account_id, &region)
                    .await
            }
            "DescribeConfigurationAggregators" => {
                self.handle_describe_configuration_aggregators(&state, body_bytes)
                    .await
            }
            "DeleteConfigurationAggregator" => {
                self.handle_delete_configuration_aggregator(&state, body_bytes)
                    .await
            }
            "PutRetentionConfiguration" => {
                self.handle_put_retention_configuration(&state, body_bytes)
                    .await
            }
            "DescribeRetentionConfigurations" => {
                self.handle_describe_retention_configurations(&state, body_bytes)
                    .await
            }
            "DeleteRetentionConfiguration" => {
                self.handle_delete_retention_configuration(&state, body_bytes)
                    .await
            }
            "StartConfigurationRecorder" => {
                self.handle_start_configuration_recorder(&state, body_bytes)
                    .await
            }
            "StopConfigurationRecorder" => {
                self.handle_stop_configuration_recorder(&state, body_bytes)
                    .await
            }
            "DescribeConfigurationRecorderStatus" => {
                self.handle_describe_configuration_recorder_status(&state, body_bytes)
                    .await
            }
            "PutOrganizationConformancePack" => {
                self.handle_put_organization_conformance_pack(
                    &state, body_bytes, account_id, &region,
                )
                .await
            }
            "DescribeOrganizationConformancePacks" => {
                self.handle_describe_organization_conformance_packs(&state, body_bytes)
                    .await
            }
            "DescribeOrganizationConformancePackStatuses" => {
                self.handle_describe_organization_conformance_pack_statuses(&state, body_bytes)
                    .await
            }
            "DeleteOrganizationConformancePack" => {
                self.handle_delete_organization_conformance_pack(&state, body_bytes)
                    .await
            }
            "GetOrganizationConformancePackDetailedStatus" => {
                self.handle_get_organization_conformance_pack_detailed_status(&state, body_bytes)
                    .await
            }
            "PutResourceConfig" => self.handle_put_resource_config(&state, body_bytes).await,
            "DeleteResourceConfig" => self.handle_delete_resource_config(&state, body_bytes).await,
            "ListDiscoveredResources" => {
                self.handle_list_discovered_resources(&state, body_bytes)
                    .await
            }
            "ListAggregateDiscoveredResources" => {
                self.handle_list_aggregate_discovered_resources(&state, body_bytes)
                    .await
            }
            "BatchGetResourceConfig" => {
                self.handle_batch_get_resource_config(&state, body_bytes)
                    .await
            }
            "BatchGetAggregateResourceConfig" => {
                self.handle_batch_get_aggregate_resource_config(&state, body_bytes)
                    .await
            }
            "GetResourceConfigHistory" => {
                self.handle_get_resource_config_history(&state, body_bytes)
                    .await
            }
            "PutEvaluations" => self.handle_put_evaluations(&state, body_bytes).await,
            "SelectResourceConfig" => self.handle_select_resource_config(&state, body_bytes).await,
            "TagResource" => self.handle_tag_resource(&state, body_bytes).await,
            "UntagResource" => self.handle_untag_resource(&state, body_bytes).await,
            "ListTagsForResource" => self.handle_list_tags_for_resource(&state, body_bytes).await,
            // --- Unimplemented operations (auto-generated stubs) ---
            "AssociateResourceTypes" => json_error_response(
                501,
                "NotImplementedError",
                "AssociateResourceTypes is not yet implemented in winterbaume-config",
            ),
            "DeleteConformancePack" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteConformancePack is not yet implemented in winterbaume-config",
            ),
            "DeleteEvaluationResults" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteEvaluationResults is not yet implemented in winterbaume-config",
            ),
            "DeleteOrganizationConfigRule" => {
                self.handle_delete_organization_config_rule(&state, body_bytes)
                    .await
            }
            "DeletePendingAggregationRequest" => json_error_response(
                501,
                "NotImplementedError",
                "DeletePendingAggregationRequest is not yet implemented in winterbaume-config",
            ),
            "DeleteRemediationConfiguration" => {
                self.handle_delete_remediation_configuration(&state, body_bytes)
                    .await
            }
            "DeleteRemediationExceptions" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteRemediationExceptions is not yet implemented in winterbaume-config",
            ),
            "DeleteServiceLinkedConfigurationRecorder" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteServiceLinkedConfigurationRecorder is not yet implemented in winterbaume-config",
            ),
            "DeleteStoredQuery" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteStoredQuery is not yet implemented in winterbaume-config",
            ),
            "DeliverConfigSnapshot" => json_error_response(
                501,
                "NotImplementedError",
                "DeliverConfigSnapshot is not yet implemented in winterbaume-config",
            ),
            "DescribeAggregateComplianceByConfigRules" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeAggregateComplianceByConfigRules is not yet implemented in winterbaume-config",
            ),
            "DescribeAggregateComplianceByConformancePacks" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeAggregateComplianceByConformancePacks is not yet implemented in winterbaume-config",
            ),
            "DescribeComplianceByConfigRule" => {
                self.handle_describe_compliance_by_config_rule(&state, body_bytes)
                    .await
            }
            "DescribeComplianceByResource" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeComplianceByResource is not yet implemented in winterbaume-config",
            ),
            "DescribeConfigRuleEvaluationStatus" => {
                self.handle_describe_config_rule_evaluation_status(&state, body_bytes)
                    .await
            }
            "DescribeConfigurationAggregatorSourcesStatus" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeConfigurationAggregatorSourcesStatus is not yet implemented in winterbaume-config",
            ),
            "DescribeConformancePackCompliance" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeConformancePackCompliance is not yet implemented in winterbaume-config",
            ),
            "DescribeConformancePackStatus" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeConformancePackStatus is not yet implemented in winterbaume-config",
            ),
            "DescribeConformancePacks" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeConformancePacks is not yet implemented in winterbaume-config",
            ),
            "DescribeDeliveryChannelStatus" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeDeliveryChannelStatus is not yet implemented in winterbaume-config",
            ),
            "DescribeOrganizationConfigRuleStatuses" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeOrganizationConfigRuleStatuses is not yet implemented in winterbaume-config",
            ),
            "DescribeOrganizationConfigRules" => {
                self.handle_describe_organization_config_rules(&state, body_bytes)
                    .await
            }
            "DescribePendingAggregationRequests" => json_error_response(
                501,
                "NotImplementedError",
                "DescribePendingAggregationRequests is not yet implemented in winterbaume-config",
            ),
            "DescribeRemediationConfigurations" => {
                self.handle_describe_remediation_configurations(&state, body_bytes)
                    .await
            }
            "DescribeRemediationExceptions" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeRemediationExceptions is not yet implemented in winterbaume-config",
            ),
            "DescribeRemediationExecutionStatus" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeRemediationExecutionStatus is not yet implemented in winterbaume-config",
            ),
            "DisassociateResourceTypes" => json_error_response(
                501,
                "NotImplementedError",
                "DisassociateResourceTypes is not yet implemented in winterbaume-config",
            ),
            "GetAggregateComplianceDetailsByConfigRule" => json_error_response(
                501,
                "NotImplementedError",
                "GetAggregateComplianceDetailsByConfigRule is not yet implemented in winterbaume-config",
            ),
            "GetAggregateConfigRuleComplianceSummary" => json_error_response(
                501,
                "NotImplementedError",
                "GetAggregateConfigRuleComplianceSummary is not yet implemented in winterbaume-config",
            ),
            "GetAggregateConformancePackComplianceSummary" => json_error_response(
                501,
                "NotImplementedError",
                "GetAggregateConformancePackComplianceSummary is not yet implemented in winterbaume-config",
            ),
            "GetAggregateDiscoveredResourceCounts" => json_error_response(
                501,
                "NotImplementedError",
                "GetAggregateDiscoveredResourceCounts is not yet implemented in winterbaume-config",
            ),
            "GetAggregateResourceConfig" => json_error_response(
                501,
                "NotImplementedError",
                "GetAggregateResourceConfig is not yet implemented in winterbaume-config",
            ),
            "GetComplianceDetailsByConfigRule" => {
                self.handle_get_compliance_details_by_config_rule(&state, body_bytes)
                    .await
            }
            "GetComplianceDetailsByResource" => json_error_response(
                501,
                "NotImplementedError",
                "GetComplianceDetailsByResource is not yet implemented in winterbaume-config",
            ),
            "GetComplianceSummaryByConfigRule" => json_error_response(
                501,
                "NotImplementedError",
                "GetComplianceSummaryByConfigRule is not yet implemented in winterbaume-config",
            ),
            "GetComplianceSummaryByResourceType" => json_error_response(
                501,
                "NotImplementedError",
                "GetComplianceSummaryByResourceType is not yet implemented in winterbaume-config",
            ),
            "GetConformancePackComplianceDetails" => json_error_response(
                501,
                "NotImplementedError",
                "GetConformancePackComplianceDetails is not yet implemented in winterbaume-config",
            ),
            "GetConformancePackComplianceSummary" => json_error_response(
                501,
                "NotImplementedError",
                "GetConformancePackComplianceSummary is not yet implemented in winterbaume-config",
            ),
            "GetCustomRulePolicy" => json_error_response(
                501,
                "NotImplementedError",
                "GetCustomRulePolicy is not yet implemented in winterbaume-config",
            ),
            "GetDiscoveredResourceCounts" => json_error_response(
                501,
                "NotImplementedError",
                "GetDiscoveredResourceCounts is not yet implemented in winterbaume-config",
            ),
            "GetOrganizationConfigRuleDetailedStatus" => json_error_response(
                501,
                "NotImplementedError",
                "GetOrganizationConfigRuleDetailedStatus is not yet implemented in winterbaume-config",
            ),
            "GetOrganizationCustomRulePolicy" => json_error_response(
                501,
                "NotImplementedError",
                "GetOrganizationCustomRulePolicy is not yet implemented in winterbaume-config",
            ),
            "GetResourceEvaluationSummary" => json_error_response(
                501,
                "NotImplementedError",
                "GetResourceEvaluationSummary is not yet implemented in winterbaume-config",
            ),
            "GetStoredQuery" => json_error_response(
                501,
                "NotImplementedError",
                "GetStoredQuery is not yet implemented in winterbaume-config",
            ),
            "ListConfigurationRecorders" => json_error_response(
                501,
                "NotImplementedError",
                "ListConfigurationRecorders is not yet implemented in winterbaume-config",
            ),
            "ListConformancePackComplianceScores" => json_error_response(
                501,
                "NotImplementedError",
                "ListConformancePackComplianceScores is not yet implemented in winterbaume-config",
            ),
            "ListResourceEvaluations" => json_error_response(
                501,
                "NotImplementedError",
                "ListResourceEvaluations is not yet implemented in winterbaume-config",
            ),
            "ListStoredQueries" => json_error_response(
                501,
                "NotImplementedError",
                "ListStoredQueries is not yet implemented in winterbaume-config",
            ),
            "PutConformancePack" => json_error_response(
                501,
                "NotImplementedError",
                "PutConformancePack is not yet implemented in winterbaume-config",
            ),
            "PutExternalEvaluation" => json_error_response(
                501,
                "NotImplementedError",
                "PutExternalEvaluation is not yet implemented in winterbaume-config",
            ),
            "PutOrganizationConfigRule" => {
                self.handle_put_organization_config_rule(&state, body_bytes, account_id, &region)
                    .await
            }
            "PutRemediationConfigurations" => {
                self.handle_put_remediation_configurations(&state, body_bytes)
                    .await
            }
            "PutRemediationExceptions" => json_error_response(
                501,
                "NotImplementedError",
                "PutRemediationExceptions is not yet implemented in winterbaume-config",
            ),
            "PutServiceLinkedConfigurationRecorder" => json_error_response(
                501,
                "NotImplementedError",
                "PutServiceLinkedConfigurationRecorder is not yet implemented in winterbaume-config",
            ),
            "PutStoredQuery" => json_error_response(
                501,
                "NotImplementedError",
                "PutStoredQuery is not yet implemented in winterbaume-config",
            ),
            "SelectAggregateResourceConfig" => json_error_response(
                501,
                "NotImplementedError",
                "SelectAggregateResourceConfig is not yet implemented in winterbaume-config",
            ),
            "StartConfigRulesEvaluation" => self.handle_start_config_rules_evaluation().await,
            "StartRemediationExecution" => self.handle_start_remediation_execution().await,
            "StartResourceEvaluation" => json_error_response(
                501,
                "NotImplementedError",
                "StartResourceEvaluation is not yet implemented in winterbaume-config",
            ),
            _ => json_error_response(400, "InvalidAction", &format!("Unknown operation {action}")),
        }
    }

    async fn handle_put_configuration_recorder(
        &self,
        state: &Arc<tokio::sync::RwLock<ConfigState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_put_configuration_recorder_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "SerializationException", &e),
        };
        let recorder = input.configuration_recorder;
        let name = recorder.name.as_deref().unwrap_or("default");
        let role_arn = recorder.role_a_r_n.as_deref().unwrap_or("");

        let recording_group = recorder.recording_group.map(|rg| RecordingGroup {
            all_supported: rg.all_supported.unwrap_or(false),
            include_global_resource_types: rg.include_global_resource_types.unwrap_or(false),
        });

        let mut state = state.write().await;
        match state.put_configuration_recorder(name, role_arn, recording_group) {
            Ok(()) => wire::serialize_put_configuration_recorder_response(),
            Err(e) => config_error_response(&e),
        }
    }

    async fn handle_describe_configuration_recorders(
        &self,
        state: &Arc<tokio::sync::RwLock<ConfigState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_configuration_recorders_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "SerializationException", &e),
        };
        let names = input.configuration_recorder_names;

        let state = state.read().await;
        match state.describe_configuration_recorders(names.as_deref()) {
            Ok(recorders) => {
                let entries: Vec<wire::ConfigurationRecorder> = recorders
                    .iter()
                    .map(|r| build_wire_configuration_recorder(r))
                    .collect();

                wire::serialize_describe_configuration_recorders_response(
                    &wire::DescribeConfigurationRecordersResponse {
                        configuration_recorders: Some(entries),
                    },
                )
            }
            Err(e) => config_error_response(&e),
        }
    }

    async fn handle_delete_configuration_recorder(
        &self,
        state: &Arc<tokio::sync::RwLock<ConfigState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_configuration_recorder_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "SerializationException", &e),
        };
        if input.configuration_recorder_name.is_empty() {
            return json_error_response(
                400,
                "InvalidRequestException",
                "ConfigurationRecorderName is required",
            );
        }

        let mut state = state.write().await;
        match state.delete_configuration_recorder(&input.configuration_recorder_name) {
            Ok(()) => wire::serialize_delete_configuration_recorder_response(),
            Err(e) => config_error_response(&e),
        }
    }

    async fn handle_put_delivery_channel(
        &self,
        state: &Arc<tokio::sync::RwLock<ConfigState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_put_delivery_channel_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "SerializationException", &e),
        };
        let channel = input.delivery_channel;
        let name = channel.name.as_deref().unwrap_or("default");
        let s3_bucket_name = channel.s3_bucket_name.as_deref().unwrap_or("");
        let s3_key_prefix = channel.s3_key_prefix.as_deref().unwrap_or("");

        let mut state = state.write().await;
        match state.put_delivery_channel(name, s3_bucket_name, s3_key_prefix) {
            Ok(()) => wire::serialize_put_delivery_channel_response(),
            Err(e) => config_error_response(&e),
        }
    }

    async fn handle_delete_delivery_channel(
        &self,
        state: &Arc<tokio::sync::RwLock<ConfigState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_delivery_channel_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "SerializationException", &e),
        };
        if input.delivery_channel_name.is_empty() {
            return json_error_response(
                400,
                "InvalidRequestException",
                "DeliveryChannelName is required",
            );
        }

        let mut state = state.write().await;
        match state.delete_delivery_channel(&input.delivery_channel_name) {
            Ok(()) => wire::serialize_delete_delivery_channel_response(),
            Err(e) => config_error_response(&e),
        }
    }

    async fn handle_describe_delivery_channels(
        &self,
        state: &Arc<tokio::sync::RwLock<ConfigState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_delivery_channels_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "SerializationException", &e),
        };
        let names = input.delivery_channel_names;

        let state = state.read().await;
        match state.describe_delivery_channels(names.as_deref()) {
            Ok(channels) => {
                let entries: Vec<wire::DeliveryChannel> = channels
                    .iter()
                    .map(|c| wire::DeliveryChannel {
                        name: Some(c.name.clone()),
                        s3_bucket_name: Some(c.s3_bucket_name.clone()),
                        s3_key_prefix: Some(c.s3_key_prefix.clone()),
                        ..Default::default()
                    })
                    .collect();

                wire::serialize_describe_delivery_channels_response(
                    &wire::DescribeDeliveryChannelsResponse {
                        delivery_channels: Some(entries),
                    },
                )
            }
            Err(e) => config_error_response(&e),
        }
    }

    async fn handle_put_config_rule(
        &self,
        state: &Arc<tokio::sync::RwLock<ConfigState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_put_config_rule_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "SerializationException", &e),
        };
        let rule = input.config_rule;

        let config_rule_name = match rule.config_rule_name {
            Some(n) if !n.is_empty() => n,
            _ => {
                return json_error_response(
                    400,
                    "InvalidRequestException",
                    "ConfigRuleName is required",
                );
            }
        };

        let source_owner = if rule.source.owner.is_empty() {
            "AWS".to_string()
        } else {
            rule.source.owner.clone()
        };
        let source_identifier = rule.source.source_identifier.clone();

        let rule_id = uuid::Uuid::new_v4().to_string();
        let config_rule_arn = format!(
            "arn:aws:config:{region}:{account_id}:config-rule/config-rule-{id}",
            id = &rule_id[..6]
        );

        let description = rule.description;
        let input_parameters = rule.input_parameters;
        let maximum_execution_frequency = rule.maximum_execution_frequency;
        let scope_resource_types = rule
            .scope
            .as_ref()
            .and_then(|s| s.compliance_resource_types.clone());

        let config_rule = ConfigRule {
            config_rule_name,
            config_rule_arn,
            config_rule_id: rule_id,
            config_rule_state: "ACTIVE".to_string(),
            description,
            source_owner,
            source_identifier,
            input_parameters,
            maximum_execution_frequency,
            scope_resource_types,
            evaluation_mode: None,
            scope: None,
        };

        let mut state = state.write().await;
        match state.put_config_rule(config_rule) {
            Ok(()) => wire::serialize_put_config_rule_response(),
            Err(e) => config_error_response(&e),
        }
    }

    async fn handle_describe_config_rules(
        &self,
        state: &Arc<tokio::sync::RwLock<ConfigState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_config_rules_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "SerializationException", &e),
        };
        let names = input.config_rule_names;

        let state = state.read().await;
        match state.describe_config_rules(names.as_deref()) {
            Ok(rules) => {
                let entries: Vec<wire::ConfigRule> = rules
                    .iter()
                    .map(|r| wire::ConfigRule {
                        config_rule_name: Some(r.config_rule_name.clone()),
                        config_rule_arn: Some(r.config_rule_arn.clone()),
                        config_rule_id: Some(r.config_rule_id.clone()),
                        config_rule_state: Some(r.config_rule_state.clone()),
                        description: r.description.clone(),
                        input_parameters: r.input_parameters.clone(),
                        maximum_execution_frequency: r.maximum_execution_frequency.clone(),
                        scope: r.scope_resource_types.as_ref().map(|types| wire::Scope {
                            compliance_resource_types: Some(types.clone()),
                            ..Default::default()
                        }),
                        source: wire::Source {
                            owner: r.source_owner.clone(),
                            source_identifier: r.source_identifier.clone(),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .collect();

                wire::serialize_describe_config_rules_response(&wire::DescribeConfigRulesResponse {
                    config_rules: Some(entries),
                    ..Default::default()
                })
            }
            Err(e) => config_error_response(&e),
        }
    }

    async fn handle_delete_config_rule(
        &self,
        state: &Arc<tokio::sync::RwLock<ConfigState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_config_rule_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "SerializationException", &e),
        };
        if input.config_rule_name.is_empty() {
            return json_error_response(
                400,
                "InvalidRequestException",
                "ConfigRuleName is required",
            );
        }

        let mut state = state.write().await;
        match state.delete_config_rule(&input.config_rule_name) {
            Ok(()) => wire::serialize_delete_config_rule_response(),
            Err(e) => config_error_response(&e),
        }
    }

    async fn handle_put_aggregation_authorization(
        &self,
        state: &Arc<tokio::sync::RwLock<ConfigState>>,
        body: &[u8],
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_put_aggregation_authorization_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "SerializationException", &e),
        };
        if input.authorized_account_id.is_empty() {
            return json_error_response(
                400,
                "InvalidRequestException",
                "AuthorizedAccountId is required",
            );
        }
        if input.authorized_aws_region.is_empty() {
            return json_error_response(
                400,
                "InvalidRequestException",
                "AuthorizedAwsRegion is required",
            );
        }

        let mut state = state.write().await;
        let entry = state.put_aggregation_authorization(
            &input.authorized_account_id,
            &input.authorized_aws_region,
            account_id,
        );

        wire::serialize_put_aggregation_authorization_response(
            &wire::PutAggregationAuthorizationResponse {
                aggregation_authorization: Some(wire::AggregationAuthorization {
                    aggregation_authorization_arn: Some(
                        entry.aggregation_authorization_arn.clone(),
                    ),
                    authorized_account_id: Some(entry.authorized_account_id.clone()),
                    authorized_aws_region: Some(entry.authorized_aws_region.clone()),
                    creation_time: Some(entry.creation_time),
                }),
            },
        )
    }

    async fn handle_describe_aggregation_authorizations(
        &self,
        state: &Arc<tokio::sync::RwLock<ConfigState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let entries = state.describe_aggregation_authorizations();
        let wire_entries: Vec<wire::AggregationAuthorization> = entries
            .iter()
            .map(|e| wire::AggregationAuthorization {
                aggregation_authorization_arn: Some(e.aggregation_authorization_arn.clone()),
                authorized_account_id: Some(e.authorized_account_id.clone()),
                authorized_aws_region: Some(e.authorized_aws_region.clone()),
                creation_time: Some(e.creation_time),
            })
            .collect();

        wire::serialize_describe_aggregation_authorizations_response(
            &wire::DescribeAggregationAuthorizationsResponse {
                aggregation_authorizations: Some(wire_entries),
                ..Default::default()
            },
        )
    }

    async fn handle_delete_aggregation_authorization(
        &self,
        state: &Arc<tokio::sync::RwLock<ConfigState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_aggregation_authorization_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "SerializationException", &e),
        };
        if input.authorized_account_id.is_empty() {
            return json_error_response(
                400,
                "InvalidRequestException",
                "AuthorizedAccountId is required",
            );
        }
        if input.authorized_aws_region.is_empty() {
            return json_error_response(
                400,
                "InvalidRequestException",
                "AuthorizedAwsRegion is required",
            );
        }

        let mut state = state.write().await;
        match state.delete_aggregation_authorization(
            &input.authorized_account_id,
            &input.authorized_aws_region,
        ) {
            Ok(()) => wire::serialize_delete_aggregation_authorization_response(),
            Err(e) => config_error_response(&e),
        }
    }

    async fn handle_put_configuration_aggregator(
        &self,
        state: &Arc<tokio::sync::RwLock<ConfigState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_put_configuration_aggregator_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "SerializationException", &e),
        };
        if input.configuration_aggregator_name.is_empty() {
            return json_error_response(
                400,
                "InvalidRequestException",
                "ConfigurationAggregatorName is required",
            );
        }
        let name = input.configuration_aggregator_name.as_str();

        let account_aggregation_sources: Option<Vec<AccountAggregationSourceEntry>> =
            input.account_aggregation_sources.map(|arr| {
                arr.into_iter()
                    .map(|s| AccountAggregationSourceEntry {
                        account_ids: s.account_ids,
                        all_aws_regions: s.all_aws_regions,
                        aws_regions: s.aws_regions,
                    })
                    .collect()
            });

        let organization_aggregation_source: Option<OrganizationAggregationSourceEntry> = input
            .organization_aggregation_source
            .map(|s| OrganizationAggregationSourceEntry {
                role_arn: s.role_arn,
                all_aws_regions: s.all_aws_regions,
                aws_regions: s.aws_regions,
            });

        let mut state = state.write().await;
        let entry = state.put_configuration_aggregator(
            name,
            account_aggregation_sources,
            organization_aggregation_source,
            account_id,
            region,
        );

        wire::serialize_put_configuration_aggregator_response(
            &wire::PutConfigurationAggregatorResponse {
                configuration_aggregator: Some(build_wire_configuration_aggregator(entry)),
            },
        )
    }

    async fn handle_describe_configuration_aggregators(
        &self,
        state: &Arc<tokio::sync::RwLock<ConfigState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_configuration_aggregators_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "SerializationException", &e),
        };
        let names = input.configuration_aggregator_names;

        let state = state.read().await;
        match state.describe_configuration_aggregators(names.as_deref()) {
            Ok(aggregators) => {
                let entries: Vec<wire::ConfigurationAggregator> = aggregators
                    .iter()
                    .map(|a| build_wire_configuration_aggregator(a))
                    .collect();

                wire::serialize_describe_configuration_aggregators_response(
                    &wire::DescribeConfigurationAggregatorsResponse {
                        configuration_aggregators: Some(entries),
                        ..Default::default()
                    },
                )
            }
            Err(e) => config_error_response(&e),
        }
    }

    async fn handle_delete_configuration_aggregator(
        &self,
        state: &Arc<tokio::sync::RwLock<ConfigState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_configuration_aggregator_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "SerializationException", &e),
        };
        if input.configuration_aggregator_name.is_empty() {
            return json_error_response(
                400,
                "InvalidRequestException",
                "ConfigurationAggregatorName is required",
            );
        }

        let mut state = state.write().await;
        match state.delete_configuration_aggregator(&input.configuration_aggregator_name) {
            Ok(()) => wire::serialize_delete_configuration_aggregator_response(),
            Err(e) => config_error_response(&e),
        }
    }

    async fn handle_put_retention_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<ConfigState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_put_retention_configuration_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "SerializationException", &e),
        };
        let retention_period_in_days = input.retention_period_in_days;
        if retention_period_in_days == 0 {
            return json_error_response(
                400,
                "InvalidRequestException",
                "RetentionPeriodInDays is required",
            );
        }

        let mut state = state.write().await;
        let entry = state.put_retention_configuration(retention_period_in_days);

        wire::serialize_put_retention_configuration_response(
            &wire::PutRetentionConfigurationResponse {
                retention_configuration: Some(wire::RetentionConfiguration {
                    name: Some(entry.name.clone()),
                    retention_period_in_days: Some(entry.retention_period_in_days),
                }),
            },
        )
    }

    async fn handle_describe_retention_configurations(
        &self,
        state: &Arc<tokio::sync::RwLock<ConfigState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_retention_configurations_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "SerializationException", &e),
        };
        let names = input.retention_configuration_names;

        let state = state.read().await;
        match state.describe_retention_configurations(names.as_deref()) {
            Ok(configs) => {
                let entries: Vec<wire::RetentionConfiguration> = configs
                    .iter()
                    .map(|c| wire::RetentionConfiguration {
                        name: Some(c.name.clone()),
                        retention_period_in_days: Some(c.retention_period_in_days),
                    })
                    .collect();

                wire::serialize_describe_retention_configurations_response(
                    &wire::DescribeRetentionConfigurationsResponse {
                        retention_configurations: Some(entries),
                        ..Default::default()
                    },
                )
            }
            Err(e) => config_error_response(&e),
        }
    }

    async fn handle_delete_retention_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<ConfigState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_retention_configuration_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "SerializationException", &e),
        };
        if input.retention_configuration_name.is_empty() {
            return json_error_response(
                400,
                "InvalidRequestException",
                "RetentionConfigurationName is required",
            );
        }

        let mut state = state.write().await;
        match state.delete_retention_configuration(&input.retention_configuration_name) {
            Ok(()) => wire::serialize_delete_retention_configuration_response(),
            Err(e) => config_error_response(&e),
        }
    }

    async fn handle_start_configuration_recorder(
        &self,
        state: &Arc<tokio::sync::RwLock<ConfigState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_start_configuration_recorder_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "SerializationException", &e),
        };
        if input.configuration_recorder_name.is_empty() {
            return json_error_response(
                400,
                "InvalidRequestException",
                "ConfigurationRecorderName is required",
            );
        }

        let mut state = state.write().await;
        match state.start_configuration_recorder(&input.configuration_recorder_name) {
            Ok(()) => wire::serialize_start_configuration_recorder_response(),
            Err(e) => config_error_response(&e),
        }
    }

    async fn handle_stop_configuration_recorder(
        &self,
        state: &Arc<tokio::sync::RwLock<ConfigState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_stop_configuration_recorder_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "SerializationException", &e),
        };
        if input.configuration_recorder_name.is_empty() {
            return json_error_response(
                400,
                "InvalidRequestException",
                "ConfigurationRecorderName is required",
            );
        }

        let mut state = state.write().await;
        match state.stop_configuration_recorder(&input.configuration_recorder_name) {
            Ok(()) => wire::serialize_stop_configuration_recorder_response(),
            Err(e) => config_error_response(&e),
        }
    }

    async fn handle_describe_configuration_recorder_status(
        &self,
        state: &Arc<tokio::sync::RwLock<ConfigState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_configuration_recorder_status_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "SerializationException", &e),
        };
        let names = input.configuration_recorder_names;

        let state = state.read().await;
        match state.describe_configuration_recorder_status(names.as_deref()) {
            Ok(recorders) => {
                let entries: Vec<wire::ConfigurationRecorderStatus> = recorders
                    .iter()
                    .map(|r| wire::ConfigurationRecorderStatus {
                        name: Some(r.name.clone()),
                        recording: Some(r.recording),
                        last_start_time: r.last_start_time,
                        last_stop_time: r.last_stop_time,
                        last_status: if r.recording {
                            Some("PENDING".to_string())
                        } else {
                            None
                        },
                        ..Default::default()
                    })
                    .collect();

                wire::serialize_describe_configuration_recorder_status_response(
                    &wire::DescribeConfigurationRecorderStatusResponse {
                        configuration_recorders_status: Some(entries),
                    },
                )
            }
            Err(e) => config_error_response(&e),
        }
    }

    async fn handle_put_organization_conformance_pack(
        &self,
        state: &Arc<tokio::sync::RwLock<ConfigState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_put_organization_conformance_pack_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "SerializationException", &e),
        };
        if input.organization_conformance_pack_name.is_empty() {
            return json_error_response(
                400,
                "InvalidRequestException",
                "OrganizationConformancePackName is required",
            );
        }
        let name = input.organization_conformance_pack_name.as_str();

        let delivery_s3_bucket = input.delivery_s3_bucket;
        let delivery_s3_key_prefix = input.delivery_s3_key_prefix;
        let excluded_accounts = input.excluded_accounts;
        let conformance_pack_input_parameters: Option<Vec<(String, String)>> =
            input.conformance_pack_input_parameters.map(|arr| {
                arr.into_iter()
                    .map(|p| (p.parameter_name, p.parameter_value))
                    .collect()
            });

        let now = chrono::Utc::now().timestamp() as f64;
        let arn = format!(
            "arn:aws:config:{region}:{account_id}:organization-conformance-pack/{name}-{}",
            &uuid::Uuid::new_v4().to_string()[..8]
        );

        let entry = OrganizationConformancePackEntry {
            organization_conformance_pack_name: name.to_string(),
            organization_conformance_pack_arn: arn.clone(),
            delivery_s3_bucket,
            delivery_s3_key_prefix,
            excluded_accounts,
            conformance_pack_input_parameters,
            last_update_time: now,
        };

        let mut state = state.write().await;
        let entry = state.put_organization_conformance_pack(entry);

        wire::serialize_put_organization_conformance_pack_response(
            &wire::PutOrganizationConformancePackResponse {
                organization_conformance_pack_arn: Some(
                    entry.organization_conformance_pack_arn.clone(),
                ),
            },
        )
    }

    async fn handle_describe_organization_conformance_packs(
        &self,
        state: &Arc<tokio::sync::RwLock<ConfigState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_organization_conformance_packs_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "SerializationException", &e),
        };
        let names = input.organization_conformance_pack_names;

        let state = state.read().await;
        match state.describe_organization_conformance_packs(names.as_deref()) {
            Ok(packs) => {
                let mut entries = Vec::new();
                for p in packs.iter() {
                    entries.push(build_wire_organization_conformance_pack(p).await);
                }

                wire::serialize_describe_organization_conformance_packs_response(
                    &wire::DescribeOrganizationConformancePacksResponse {
                        organization_conformance_packs: Some(entries),
                        ..Default::default()
                    },
                )
            }
            Err(e) => config_error_response(&e),
        }
    }

    async fn handle_describe_organization_conformance_pack_statuses(
        &self,
        state: &Arc<tokio::sync::RwLock<ConfigState>>,
        body: &[u8],
    ) -> MockResponse {
        let input =
            match wire::deserialize_describe_organization_conformance_pack_statuses_request(body) {
                Ok(v) => v,
                Err(e) => return json_error_response(400, "SerializationException", &e),
            };
        let names = input.organization_conformance_pack_names;

        let state = state.read().await;
        match state.describe_organization_conformance_pack_statuses(names.as_deref()) {
            Ok(packs) => {
                let entries: Vec<wire::OrganizationConformancePackStatus> = packs
                    .iter()
                    .map(|p| wire::OrganizationConformancePackStatus {
                        organization_conformance_pack_name: Some(
                            p.organization_conformance_pack_name.clone(),
                        ),
                        status: Some("CREATE_SUCCESSFUL".to_string()),
                        last_update_time: Some(p.last_update_time),
                        ..Default::default()
                    })
                    .collect();

                wire::serialize_describe_organization_conformance_pack_statuses_response(
                    &wire::DescribeOrganizationConformancePackStatusesResponse {
                        organization_conformance_pack_statuses: Some(entries),
                        ..Default::default()
                    },
                )
            }
            Err(e) => config_error_response(&e),
        }
    }

    async fn handle_delete_organization_conformance_pack(
        &self,
        state: &Arc<tokio::sync::RwLock<ConfigState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_organization_conformance_pack_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "SerializationException", &e),
        };
        if input.organization_conformance_pack_name.is_empty() {
            return json_error_response(
                400,
                "InvalidRequestException",
                "OrganizationConformancePackName is required",
            );
        }

        let mut state = state.write().await;
        match state.delete_organization_conformance_pack(&input.organization_conformance_pack_name)
        {
            Ok(()) => wire::serialize_delete_organization_conformance_pack_response(),
            Err(e) => config_error_response(&e),
        }
    }

    async fn handle_get_organization_conformance_pack_detailed_status(
        &self,
        state: &Arc<tokio::sync::RwLock<ConfigState>>,
        body: &[u8],
    ) -> MockResponse {
        let input =
            match wire::deserialize_get_organization_conformance_pack_detailed_status_request(body)
            {
                Ok(v) => v,
                Err(e) => return json_error_response(400, "SerializationException", &e),
            };
        if input.organization_conformance_pack_name.is_empty() {
            return json_error_response(
                400,
                "InvalidRequestException",
                "OrganizationConformancePackName is required",
            );
        }

        let state = state.read().await;
        match state.get_organization_conformance_pack_detailed_status(
            &input.organization_conformance_pack_name,
        ) {
            Ok(_pack) => {
                // Return empty detailed statuses (no member accounts in mock)
                wire::serialize_get_organization_conformance_pack_detailed_status_response(
                    &wire::GetOrganizationConformancePackDetailedStatusResponse {
                        organization_conformance_pack_detailed_statuses: Some(vec![]),
                        ..Default::default()
                    },
                )
            }
            Err(e) => config_error_response(&e),
        }
    }

    async fn handle_put_resource_config(
        &self,
        state: &Arc<tokio::sync::RwLock<ConfigState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_put_resource_config_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "SerializationException", &e),
        };
        if input.resource_type.is_empty() {
            return json_error_response(400, "InvalidRequestException", "ResourceType is required");
        }
        if input.resource_id.is_empty() {
            return json_error_response(400, "InvalidRequestException", "ResourceId is required");
        }

        let schema_version_id = if input.schema_version_id.is_empty() {
            "1".to_string()
        } else {
            input.schema_version_id
        };
        let configuration = if input.configuration.is_empty() {
            "{}".to_string()
        } else {
            input.configuration
        };

        let entry = ResourceConfigEntry {
            resource_type: input.resource_type,
            resource_id: input.resource_id,
            schema_version_id,
            configuration,
            resource_name: input.resource_name,
            tags: None,
        };

        let mut state = state.write().await;
        state.put_resource_config(entry);
        wire::serialize_put_resource_config_response()
    }

    async fn handle_delete_resource_config(
        &self,
        state: &Arc<tokio::sync::RwLock<ConfigState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_resource_config_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "SerializationException", &e),
        };
        if input.resource_type.is_empty() {
            return json_error_response(400, "InvalidRequestException", "ResourceType is required");
        }
        if input.resource_id.is_empty() {
            return json_error_response(400, "InvalidRequestException", "ResourceId is required");
        }

        let mut state = state.write().await;
        match state.delete_resource_config(&input.resource_type, &input.resource_id) {
            Ok(()) => wire::serialize_delete_resource_config_response(),
            Err(e) => config_error_response(&e),
        }
    }

    async fn handle_list_discovered_resources(
        &self,
        state: &Arc<tokio::sync::RwLock<ConfigState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_discovered_resources_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "SerializationException", &e),
        };
        if input.resource_type.is_empty() {
            return json_error_response(400, "InvalidRequestException", "resourceType is required");
        }

        let state = state.read().await;
        let resources = state.list_discovered_resources(&input.resource_type);

        let entries: Vec<wire::ResourceIdentifier> = resources
            .iter()
            .map(|r| wire::ResourceIdentifier {
                resource_id: Some(r.resource_id.clone()),
                resource_type: Some(r.resource_type.clone()),
                resource_name: r.resource_name.clone(),
                ..Default::default()
            })
            .collect();

        wire::serialize_list_discovered_resources_response(&wire::ListDiscoveredResourcesResponse {
            resource_identifiers: Some(entries),
            ..Default::default()
        })
    }

    async fn handle_list_aggregate_discovered_resources(
        &self,
        state: &Arc<tokio::sync::RwLock<ConfigState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_aggregate_discovered_resources_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "SerializationException", &e),
        };
        if input.resource_type.is_empty() {
            return json_error_response(400, "InvalidRequestException", "ResourceType is required");
        }

        let state = state.read().await;
        let resources = state.list_aggregate_discovered_resources(
            &input.resource_type,
            &input.configuration_aggregator_name,
        );

        let entries: Vec<wire::AggregateResourceIdentifier> = resources
            .iter()
            .map(|r| wire::AggregateResourceIdentifier {
                resource_id: r.resource_id.clone(),
                resource_type: r.resource_type.clone(),
                resource_name: r.resource_name.clone(),
                source_account_id: DEFAULT_ACCOUNT_ID.to_string(),
                source_region: "us-east-1".to_string(),
            })
            .collect();

        wire::serialize_list_aggregate_discovered_resources_response(
            &wire::ListAggregateDiscoveredResourcesResponse {
                resource_identifiers: Some(entries),
                ..Default::default()
            },
        )
    }

    async fn handle_batch_get_resource_config(
        &self,
        state: &Arc<tokio::sync::RwLock<ConfigState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_batch_get_resource_config_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "SerializationException", &e),
        };
        let keys: Vec<(String, String)> = input
            .resource_keys
            .into_iter()
            .map(|k| (k.resource_type, k.resource_id))
            .collect();

        let state = state.read().await;
        let (found, not_found) = state.batch_get_resource_config(&keys);

        let base_items: Vec<wire::BaseConfigurationItem> = found
            .iter()
            .map(|r| wire::BaseConfigurationItem {
                resource_id: Some(r.resource_id.clone()),
                resource_type: Some(r.resource_type.clone()),
                resource_name: r.resource_name.clone(),
                configuration: Some(r.configuration.clone()),
                ..Default::default()
            })
            .collect();

        let unprocessed: Vec<wire::ResourceKey> = not_found
            .iter()
            .map(|(rt, rid)| wire::ResourceKey {
                resource_type: rt.clone(),
                resource_id: rid.clone(),
            })
            .collect();

        wire::serialize_batch_get_resource_config_response(&wire::BatchGetResourceConfigResponse {
            base_configuration_items: Some(base_items),
            unprocessed_resource_keys: Some(unprocessed),
        })
    }

    async fn handle_batch_get_aggregate_resource_config(
        &self,
        state: &Arc<tokio::sync::RwLock<ConfigState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_batch_get_aggregate_resource_config_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "SerializationException", &e),
        };
        let aggregator_name = input.configuration_aggregator_name;

        let keys: Vec<(String, String)> = input
            .resource_identifiers
            .into_iter()
            .map(|k| (k.resource_type, k.resource_id))
            .collect();

        let state = state.read().await;
        let (found, not_found) = state.batch_get_aggregate_resource_config(&keys, &aggregator_name);

        let base_items: Vec<wire::BaseConfigurationItem> = found
            .iter()
            .map(|r| wire::BaseConfigurationItem {
                resource_id: Some(r.resource_id.clone()),
                resource_type: Some(r.resource_type.clone()),
                resource_name: r.resource_name.clone(),
                configuration: Some(r.configuration.clone()),
                ..Default::default()
            })
            .collect();

        let unprocessed: Vec<wire::AggregateResourceIdentifier> = not_found
            .iter()
            .map(|(rt, rid)| wire::AggregateResourceIdentifier {
                resource_type: rt.clone(),
                resource_id: rid.clone(),
                source_account_id: DEFAULT_ACCOUNT_ID.to_string(),
                source_region: "us-east-1".to_string(),
                ..Default::default()
            })
            .collect();

        wire::serialize_batch_get_aggregate_resource_config_response(
            &wire::BatchGetAggregateResourceConfigResponse {
                base_configuration_items: Some(base_items),
                unprocessed_resource_identifiers: Some(unprocessed),
            },
        )
    }

    async fn handle_get_resource_config_history(
        &self,
        state: &Arc<tokio::sync::RwLock<ConfigState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_resource_config_history_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "SerializationException", &e),
        };
        if input.resource_type.is_empty() {
            return json_error_response(400, "InvalidRequestException", "resourceType is required");
        }
        if input.resource_id.is_empty() {
            return json_error_response(400, "InvalidRequestException", "resourceId is required");
        }

        let state = state.read().await;
        let items = state.get_resource_config_history(&input.resource_type, &input.resource_id);

        let entries: Vec<wire::ConfigurationItem> = items
            .iter()
            .map(|r| wire::ConfigurationItem {
                resource_id: Some(r.resource_id.clone()),
                resource_type: Some(r.resource_type.clone()),
                resource_name: r.resource_name.clone(),
                configuration: Some(r.configuration.clone()),
                ..Default::default()
            })
            .collect();

        wire::serialize_get_resource_config_history_response(
            &wire::GetResourceConfigHistoryResponse {
                configuration_items: Some(entries),
                ..Default::default()
            },
        )
    }

    async fn handle_put_evaluations(
        &self,
        state: &Arc<tokio::sync::RwLock<ConfigState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_put_evaluations_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "SerializationException", &e),
        };
        let _result_token = input.result_token;

        let evaluations: Vec<EvaluationEntry> = input
            .evaluations
            .unwrap_or_default()
            .into_iter()
            .map(|e| EvaluationEntry {
                compliance_resource_type: e.compliance_resource_type,
                compliance_resource_id: e.compliance_resource_id,
                compliance_type: e.compliance_type,
                ordering_timestamp: e.ordering_timestamp,
                annotation: e.annotation,
            })
            .collect();

        let mut state = state.write().await;
        let failed = state.put_evaluations(evaluations);

        let failed_evaluations: Vec<wire::Evaluation> = failed
            .iter()
            .map(|e| wire::Evaluation {
                compliance_resource_type: e.compliance_resource_type.clone(),
                compliance_resource_id: e.compliance_resource_id.clone(),
                compliance_type: e.compliance_type.clone(),
                ordering_timestamp: e.ordering_timestamp,
                annotation: e.annotation.clone(),
            })
            .collect();

        wire::serialize_put_evaluations_response(&wire::PutEvaluationsResponse {
            failed_evaluations: Some(failed_evaluations),
        })
    }

    // STUB[no-engine]: SQL-like query evaluation over Config resource data
    //   requires a real query engine; always returns empty results.
    async fn handle_select_resource_config(
        &self,
        state: &Arc<tokio::sync::RwLock<ConfigState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_select_resource_config_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "SerializationException", &e),
        };
        let expression = input.expression;

        let state = state.read().await;
        let results = state.select_resource_config(&expression);

        wire::serialize_select_resource_config_response(&wire::SelectResourceConfigResponse {
            results: Some(results),
            query_info: Some(wire::QueryInfo {
                select_fields: Some(vec![]),
            }),
            ..Default::default()
        })
    }

    async fn handle_tag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<ConfigState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_tag_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "SerializationException", &e),
        };
        if input.resource_arn.is_empty() {
            return json_error_response(400, "InvalidRequestException", "ResourceArn is required");
        }

        let tags: Vec<TagEntry> = input
            .tags
            .into_iter()
            .filter_map(|t| {
                let key = t.key?;
                let value = t.value?;
                Some(TagEntry { key, value })
            })
            .collect();

        let mut state = state.write().await;
        state.tag_resource(&input.resource_arn, tags);
        wire::serialize_tag_resource_response()
    }

    async fn handle_untag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<ConfigState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_untag_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "SerializationException", &e),
        };
        if input.resource_arn.is_empty() {
            return json_error_response(400, "InvalidRequestException", "ResourceArn is required");
        }

        let mut state = state.write().await;
        state.untag_resource(&input.resource_arn, &input.tag_keys);
        wire::serialize_untag_resource_response()
    }

    async fn handle_list_tags_for_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<ConfigState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_tags_for_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "SerializationException", &e),
        };
        if input.resource_arn.is_empty() {
            return json_error_response(400, "InvalidRequestException", "ResourceArn is required");
        }

        let state = state.read().await;
        let tags = state.list_tags_for_resource(&input.resource_arn);

        let entries: Vec<wire::Tag> = tags
            .iter()
            .map(|t| wire::Tag {
                key: Some(t.key.clone()),
                value: Some(t.value.clone()),
            })
            .collect();

        wire::serialize_list_tags_for_resource_response(&wire::ListTagsForResourceResponse {
            tags: Some(entries),
            ..Default::default()
        })
    }
}

/// Convert a state ConfigurationRecorder to the wire format.
fn build_wire_configuration_recorder(
    r: &crate::types::ConfigurationRecorder,
) -> wire::ConfigurationRecorder {
    wire::ConfigurationRecorder {
        name: Some(r.name.clone()),
        role_a_r_n: Some(r.role_arn.clone()),
        recording_group: r.recording_group.as_ref().map(build_wire_recording_group),
        ..Default::default()
    }
}

/// Convert a state RecordingGroup to the wire format.
fn build_wire_recording_group(rg: &crate::types::RecordingGroup) -> wire::RecordingGroup {
    wire::RecordingGroup {
        all_supported: Some(rg.all_supported),
        include_global_resource_types: Some(rg.include_global_resource_types),
        ..Default::default()
    }
}

fn build_wire_configuration_aggregator(
    a: &ConfigurationAggregatorEntry,
) -> wire::ConfigurationAggregator {
    wire::ConfigurationAggregator {
        configuration_aggregator_name: Some(a.configuration_aggregator_name.clone()),
        configuration_aggregator_arn: Some(a.configuration_aggregator_arn.clone()),
        account_aggregation_sources: a.account_aggregation_sources.as_ref().map(|sources| {
            sources
                .iter()
                .map(|s| wire::AccountAggregationSource {
                    account_ids: s.account_ids.clone(),
                    all_aws_regions: s.all_aws_regions,
                    aws_regions: s.aws_regions.clone(),
                })
                .collect()
        }),
        organization_aggregation_source: a.organization_aggregation_source.as_ref().map(|s| {
            wire::OrganizationAggregationSource {
                role_arn: s.role_arn.clone(),
                all_aws_regions: s.all_aws_regions,
                aws_regions: s.aws_regions.clone(),
            }
        }),
        creation_time: Some(a.creation_time),
        last_updated_time: Some(a.last_updated_time),
        ..Default::default()
    }
}

async fn build_wire_organization_conformance_pack(
    p: &OrganizationConformancePackEntry,
) -> wire::OrganizationConformancePack {
    wire::OrganizationConformancePack {
        organization_conformance_pack_name: Some(p.organization_conformance_pack_name.clone()),
        organization_conformance_pack_arn: Some(p.organization_conformance_pack_arn.clone()),
        delivery_s3_bucket: p.delivery_s3_bucket.clone(),
        delivery_s3_key_prefix: p.delivery_s3_key_prefix.clone(),
        excluded_accounts: p.excluded_accounts.clone(),
        conformance_pack_input_parameters: p.conformance_pack_input_parameters.as_ref().map(
            |params| {
                params
                    .iter()
                    .map(|(name, value)| wire::ConformancePackInputParameter {
                        parameter_name: name.clone(),
                        parameter_value: value.clone(),
                    })
                    .collect()
            },
        ),
        last_update_time: Some(p.last_update_time),
    }
}

impl ConfigService {
    async fn handle_put_remediation_configurations(
        &self,
        state: &Arc<tokio::sync::RwLock<ConfigState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_put_remediation_configurations_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "SerializationException", &e),
        };
        let configs: Vec<RemediationConfigEntry> = input
            .remediation_configurations
            .into_iter()
            .map(|c| {
                let parameters = c.parameters.map(|m| {
                    m.into_iter()
                        .map(|(k, v)| {
                            (
                                k,
                                serde_json::to_value(v).unwrap_or(serde_json::Value::Null),
                            )
                        })
                        .collect()
                });
                RemediationConfigEntry {
                    config_rule_name: c.config_rule_name,
                    target_type: if c.target_type.is_empty() {
                        "SSM_DOCUMENT".to_string()
                    } else {
                        c.target_type
                    },
                    target_id: c.target_id,
                    target_version: c.target_version,
                    automatic: c.automatic,
                    maximum_automatic_attempts: c.maximum_automatic_attempts,
                    retry_attempt_seconds: c.retry_attempt_seconds,
                    resource_type: c.resource_type,
                    parameters,
                }
            })
            .collect();

        let mut state = state.write().await;
        state.put_remediation_configurations(configs);
        wire::serialize_put_remediation_configurations_response(
            &wire::PutRemediationConfigurationsResponse {
                failed_batches: Some(vec![]),
            },
        )
    }

    async fn handle_describe_remediation_configurations(
        &self,
        state: &Arc<tokio::sync::RwLock<ConfigState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_remediation_configurations_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "SerializationException", &e),
        };
        let config_rule_names = input.config_rule_names;

        let state = state.read().await;
        let configs = state.describe_remediation_configurations(&config_rule_names);
        let entries: Vec<wire::RemediationConfiguration> = configs
            .iter()
            .map(|c| wire::RemediationConfiguration {
                config_rule_name: c.config_rule_name.clone(),
                target_type: c.target_type.clone(),
                target_id: c.target_id.clone(),
                target_version: c.target_version.clone(),
                automatic: c.automatic,
                maximum_automatic_attempts: c.maximum_automatic_attempts,
                retry_attempt_seconds: c.retry_attempt_seconds,
                resource_type: c.resource_type.clone(),
                ..Default::default()
            })
            .collect();

        wire::serialize_describe_remediation_configurations_response(
            &wire::DescribeRemediationConfigurationsResponse {
                remediation_configurations: Some(entries),
            },
        )
    }

    async fn handle_delete_remediation_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<ConfigState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_remediation_configuration_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "SerializationException", &e),
        };
        if input.config_rule_name.is_empty() {
            return json_error_response(
                400,
                "InvalidRequestException",
                "ConfigRuleName is required",
            );
        }

        let mut state = state.write().await;
        match state.delete_remediation_configuration(&input.config_rule_name) {
            Ok(()) => wire::serialize_delete_remediation_configuration_response(
                &wire::DeleteRemediationConfigurationResponse {},
            ),
            Err(e) => config_error_response(&e),
        }
    }

    // STUB[no-engine]: Remediation execution requires invoking real SSM automation
    //   documents or Lambda functions; always returns success with no failures.
    async fn handle_start_remediation_execution(&self) -> MockResponse {
        wire::serialize_start_remediation_execution_response(
            &wire::StartRemediationExecutionResponse {
                failed_items: Some(vec![]),
                failure_message: None,
            },
        )
    }

    // STUB[no-engine]: Config rules evaluation requires real AWS Config engine
    //   to evaluate resources against rules; always accepts and returns success.
    async fn handle_start_config_rules_evaluation(&self) -> MockResponse {
        wire::serialize_start_config_rules_evaluation_response(
            &wire::StartConfigRulesEvaluationResponse {},
        )
    }

    async fn handle_put_organization_config_rule(
        &self,
        state: &Arc<tokio::sync::RwLock<ConfigState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_put_organization_config_rule_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "SerializationException", &e),
        };
        if input.organization_config_rule_name.is_empty() {
            return json_error_response(
                400,
                "InvalidRequestException",
                "OrganizationConfigRuleName is required",
            );
        }
        let name = input.organization_config_rule_name.as_str();

        let now = chrono::Utc::now().timestamp() as f64;
        let arn = format!(
            "arn:aws:config:{region}:{account_id}:organization-config-rule/{name}-{}",
            &uuid::Uuid::new_v4().to_string()[..8]
        );

        let entry = OrganizationConfigRuleEntry {
            organization_config_rule_name: name.to_string(),
            organization_config_rule_arn: arn.clone(),
            excluded_accounts: input.excluded_accounts,
            last_update_time: now,
            managed_rule_metadata: input
                .organization_managed_rule_metadata
                .as_ref()
                .and_then(|m| serde_json::to_value(m).ok()),
            custom_rule_metadata: input
                .organization_custom_rule_metadata
                .as_ref()
                .and_then(|m| serde_json::to_value(m).ok()),
            custom_policy_rule_metadata: input
                .organization_custom_policy_rule_metadata
                .as_ref()
                .and_then(|m| serde_json::to_value(m).ok()),
        };

        let mut state = state.write().await;
        let entry = state.put_organization_config_rule(entry);
        wire::serialize_put_organization_config_rule_response(
            &wire::PutOrganizationConfigRuleResponse {
                organization_config_rule_arn: Some(entry.organization_config_rule_arn.clone()),
            },
        )
    }

    async fn handle_describe_organization_config_rules(
        &self,
        state: &Arc<tokio::sync::RwLock<ConfigState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_organization_config_rules_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "SerializationException", &e),
        };
        let names = input.organization_config_rule_names;

        let state = state.read().await;
        match state.describe_organization_config_rules(names.as_deref()) {
            Ok(rules) => {
                let entries: Vec<wire::OrganizationConfigRule> = rules
                    .iter()
                    .map(|r| wire::OrganizationConfigRule {
                        organization_config_rule_name: Some(
                            r.organization_config_rule_name.clone(),
                        ),
                        organization_config_rule_arn: Some(r.organization_config_rule_arn.clone()),
                        excluded_accounts: r.excluded_accounts.clone(),
                        last_update_time: Some(r.last_update_time),
                        ..Default::default()
                    })
                    .collect();

                wire::serialize_describe_organization_config_rules_response(
                    &wire::DescribeOrganizationConfigRulesResponse {
                        organization_config_rules: Some(entries),
                        ..Default::default()
                    },
                )
            }
            Err(e) => config_error_response(&e),
        }
    }

    async fn handle_delete_organization_config_rule(
        &self,
        state: &Arc<tokio::sync::RwLock<ConfigState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_organization_config_rule_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "SerializationException", &e),
        };
        if input.organization_config_rule_name.is_empty() {
            return json_error_response(
                400,
                "InvalidRequestException",
                "OrganizationConfigRuleName is required",
            );
        }

        let mut state = state.write().await;
        match state.delete_organization_config_rule(&input.organization_config_rule_name) {
            Ok(()) => wire::serialize_delete_organization_config_rule_response(),
            Err(e) => config_error_response(&e),
        }
    }

    async fn handle_get_compliance_details_by_config_rule(
        &self,
        state: &Arc<tokio::sync::RwLock<ConfigState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_compliance_details_by_config_rule_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "SerializationException", &e),
        };
        let config_rule_name = input.config_rule_name;

        let state = state.read().await;
        let evals = state.get_compliance_details_by_config_rule(&config_rule_name);

        let results: Vec<wire::EvaluationResult> = evals
            .iter()
            .map(|e| wire::EvaluationResult {
                compliance_type: Some(e.compliance_type.clone()),
                result_recorded_time: Some(e.ordering_timestamp),
                config_rule_invoked_time: Some(e.ordering_timestamp),
                annotation: e.annotation.clone(),
                evaluation_result_identifier: Some(wire::EvaluationResultIdentifier {
                    evaluation_result_qualifier: Some(wire::EvaluationResultQualifier {
                        config_rule_name: Some(config_rule_name.clone()),
                        resource_type: Some(e.compliance_resource_type.clone()),
                        resource_id: Some(e.compliance_resource_id.clone()),
                        ..Default::default()
                    }),
                    ordering_timestamp: Some(e.ordering_timestamp),
                    resource_evaluation_id: None,
                }),
                ..Default::default()
            })
            .collect();

        wire::serialize_get_compliance_details_by_config_rule_response(
            &wire::GetComplianceDetailsByConfigRuleResponse {
                evaluation_results: Some(results),
                ..Default::default()
            },
        )
    }

    async fn handle_describe_compliance_by_config_rule(
        &self,
        state: &Arc<tokio::sync::RwLock<ConfigState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_compliance_by_config_rule_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "SerializationException", &e),
        };
        let names = input.config_rule_names;

        let state = state.read().await;
        let compliance = state.describe_compliance_by_config_rule(names.as_deref());

        let entries: Vec<wire::ComplianceByConfigRule> = compliance
            .iter()
            .map(|(rule, compliance_type)| wire::ComplianceByConfigRule {
                config_rule_name: Some(rule.config_rule_name.clone()),
                compliance: Some(wire::Compliance {
                    compliance_type: Some(compliance_type.to_string()),
                    ..Default::default()
                }),
            })
            .collect();

        wire::serialize_describe_compliance_by_config_rule_response(
            &wire::DescribeComplianceByConfigRuleResponse {
                compliance_by_config_rules: Some(entries),
                ..Default::default()
            },
        )
    }

    async fn handle_describe_config_rule_evaluation_status(
        &self,
        state: &Arc<tokio::sync::RwLock<ConfigState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_config_rule_evaluation_status_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "SerializationException", &e),
        };
        let names = input.config_rule_names;

        let state = state.read().await;
        let rules = match state.describe_config_rules(names.as_deref()) {
            Ok(rules) => rules,
            Err(e) => return config_error_response(&e),
        };

        let now = chrono::Utc::now().timestamp() as f64;
        let entries: Vec<wire::ConfigRuleEvaluationStatus> = rules
            .iter()
            .map(|r| wire::ConfigRuleEvaluationStatus {
                config_rule_name: Some(r.config_rule_name.clone()),
                config_rule_arn: Some(r.config_rule_arn.clone()),
                config_rule_id: Some(r.config_rule_id.clone()),
                last_successful_evaluation_time: Some(now),
                last_successful_invocation_time: Some(now),
                first_evaluation_started: Some(true),
                ..Default::default()
            })
            .collect();

        wire::serialize_describe_config_rule_evaluation_status_response(
            &wire::DescribeConfigRuleEvaluationStatusResponse {
                config_rules_evaluation_status: Some(entries),
                ..Default::default()
            },
        )
    }
}

fn config_error_response(err: &ConfigError) -> MockResponse {
    let (status, error_type) = match err {
        ConfigError::InvalidConfigurationRecorderName => {
            (400, "InvalidConfigurationRecorderNameException")
        }
        ConfigError::NoSuchConfigurationRecorder { .. } => {
            (400, "NoSuchConfigurationRecorderException")
        }
        ConfigError::NoSuchDeliveryChannel { .. } => (400, "NoSuchDeliveryChannelException"),
        ConfigError::NoSuchConfigRule { .. } => (400, "NoSuchConfigRuleException"),
        ConfigError::NoSuchConfigurationAggregator => {
            (400, "NoSuchConfigurationAggregatorException")
        }
        ConfigError::NoSuchRetentionConfiguration { .. } => {
            (400, "NoSuchRetentionConfigurationException")
        }
        ConfigError::NoSuchOrganizationConformancePack { .. } => {
            (400, "NoSuchOrganizationConformancePackException")
        }
        ConfigError::NoSuchOrganizationConfigRule { .. } => {
            (400, "NoSuchOrganizationConfigRuleException")
        }
    };
    MockResponse::json(
        status,
        json!({"__type": error_type, "message": err.to_string()}).to_string(),
    )
}
