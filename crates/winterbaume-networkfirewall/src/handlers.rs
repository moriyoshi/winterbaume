use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
};

use crate::model;
use crate::state::{NetworkFirewallState, NfwError};
use crate::types::SubnetMapping;
use crate::views::NetworkFirewallStateView;
use crate::wire;

/// Network Firewall service handler that processes awsJson1.0 protocol requests.
pub struct NetworkFirewallService {
    pub(crate) state: Arc<BackendState<NetworkFirewallState>>,
    pub(crate) notifier: StateChangeNotifier<NetworkFirewallStateView>,
}

impl NetworkFirewallService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for NetworkFirewallService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for NetworkFirewallService {
    fn service_name(&self) -> &str {
        "network-firewall"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://network-firewall\..*\.amazonaws\.com",
            r"https?://network-firewall\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl NetworkFirewallService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = DEFAULT_ACCOUNT_ID;

        // Extract action from X-Amz-Target header
        // Format: "NetworkFirewall_20201112.CreateFirewall"
        let action = request
            .headers
            .get("x-amz-target")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.rsplit('.').next())
            .map(|s| s.to_string());

        let action = match action {
            Some(a) => a,
            None => {
                return json_error_response(400, "MissingAction", "Missing X-Amz-Target header");
            }
        };

        let body_bytes: &[u8] = &request.body;

        let state = self.state.get(account_id, &region);

        match action.as_str() {
            "AcceptNetworkFirewallTransitGatewayAttachment" => {
                self.handle_accept_network_firewall_transit_gateway_attachment(&state, body_bytes)
                    .await
            }
            "AssociateAvailabilityZones" => {
                self.handle_associate_availability_zones(&state, body_bytes)
                    .await
            }
            "AssociateFirewallPolicy" => {
                self.handle_associate_firewall_policy(&state, body_bytes)
                    .await
            }
            "AssociateSubnets" => self.handle_associate_subnets(&state, body_bytes).await,
            "AttachRuleGroupsToProxyConfiguration" => {
                self.handle_attach_rule_groups_to_proxy_configuration(&state, body_bytes)
                    .await
            }
            "CreateFirewall" => {
                self.handle_create_firewall(&state, body_bytes, account_id, &region)
                    .await
            }
            "CreateFirewallPolicy" => {
                self.handle_create_firewall_policy(&state, body_bytes, account_id, &region)
                    .await
            }
            "CreateProxy" => {
                self.handle_create_proxy(&state, body_bytes, account_id, &region)
                    .await
            }
            "CreateProxyConfiguration" => {
                self.handle_create_proxy_configuration(&state, body_bytes, account_id, &region)
                    .await
            }
            "CreateProxyRuleGroup" => {
                self.handle_create_proxy_rule_group(&state, body_bytes, account_id, &region)
                    .await
            }
            "CreateProxyRules" => self.handle_create_proxy_rules(&state, body_bytes).await,
            "CreateRuleGroup" => {
                self.handle_create_rule_group(&state, body_bytes, account_id, &region)
                    .await
            }
            "CreateTLSInspectionConfiguration" => {
                self.handle_create_t_l_s_inspection_configuration(
                    &state, body_bytes, account_id, &region,
                )
                .await
            }
            "CreateVpcEndpointAssociation" => {
                self.handle_create_vpc_endpoint_association(&state, body_bytes, account_id, &region)
                    .await
            }
            "DeleteFirewall" => self.handle_delete_firewall(&state, body_bytes).await,
            "DeleteFirewallPolicy" => self.handle_delete_firewall_policy(&state, body_bytes).await,
            "DeleteNetworkFirewallTransitGatewayAttachment" => {
                self.handle_delete_network_firewall_transit_gateway_attachment(&state, body_bytes)
                    .await
            }
            "DeleteProxy" => self.handle_delete_proxy(&state, body_bytes).await,
            "DeleteProxyConfiguration" => {
                self.handle_delete_proxy_configuration(&state, body_bytes)
                    .await
            }
            "DeleteProxyRuleGroup" => {
                self.handle_delete_proxy_rule_group(&state, body_bytes)
                    .await
            }
            "DeleteProxyRules" => self.handle_delete_proxy_rules(&state, body_bytes).await,
            "DeleteResourcePolicy" => self.handle_delete_resource_policy(&state, body_bytes).await,
            "DeleteRuleGroup" => self.handle_delete_rule_group(&state, body_bytes).await,
            "DeleteTLSInspectionConfiguration" => {
                self.handle_delete_t_l_s_inspection_configuration(&state, body_bytes)
                    .await
            }
            "DeleteVpcEndpointAssociation" => {
                self.handle_delete_vpc_endpoint_association(&state, body_bytes)
                    .await
            }
            "DescribeFirewall" => self.handle_describe_firewall(&state, body_bytes).await,
            "DescribeFirewallMetadata" => {
                self.handle_describe_firewall_metadata(&state, body_bytes)
                    .await
            }
            "DescribeFirewallPolicy" => {
                self.handle_describe_firewall_policy(&state, body_bytes)
                    .await
            }
            "DescribeFlowOperation" => {
                self.handle_describe_flow_operation(&state, body_bytes)
                    .await
            }
            "DescribeLoggingConfiguration" => {
                self.handle_describe_logging_configuration(&state, body_bytes)
                    .await
            }
            "DescribeProxy" => self.handle_describe_proxy(&state, body_bytes).await,
            "DescribeProxyConfiguration" => {
                self.handle_describe_proxy_configuration(&state, body_bytes)
                    .await
            }
            "DescribeProxyRule" => self.handle_describe_proxy_rule(&state, body_bytes).await,
            "DescribeProxyRuleGroup" => {
                self.handle_describe_proxy_rule_group(&state, body_bytes)
                    .await
            }
            "DescribeResourcePolicy" => {
                self.handle_describe_resource_policy(&state, body_bytes)
                    .await
            }
            "DescribeRuleGroup" => self.handle_describe_rule_group(&state, body_bytes).await,
            "DescribeRuleGroupMetadata" => {
                self.handle_describe_rule_group_metadata(&state, body_bytes)
                    .await
            }
            "DescribeRuleGroupSummary" => {
                self.handle_describe_rule_group_summary(&state, body_bytes)
                    .await
            }
            "DescribeTLSInspectionConfiguration" => {
                self.handle_describe_t_l_s_inspection_configuration(&state, body_bytes)
                    .await
            }
            "DescribeVpcEndpointAssociation" => {
                self.handle_describe_vpc_endpoint_association(&state, body_bytes)
                    .await
            }
            "DetachRuleGroupsFromProxyConfiguration" => {
                self.handle_detach_rule_groups_from_proxy_configuration(&state, body_bytes)
                    .await
            }
            "DisassociateAvailabilityZones" => {
                self.handle_disassociate_availability_zones(&state, body_bytes)
                    .await
            }
            "DisassociateSubnets" => self.handle_disassociate_subnets(&state, body_bytes).await,
            "GetAnalysisReportResults" => {
                self.handle_get_analysis_report_results(&state, body_bytes)
                    .await
            }
            "ListAnalysisReports" => self.handle_list_analysis_reports(&state, body_bytes).await,
            "ListFirewallPolicies" => self.handle_list_firewall_policies(&state).await,
            "ListFirewalls" => self.handle_list_firewalls(&state).await,
            "ListFlowOperationResults" => {
                self.handle_list_flow_operation_results(&state, body_bytes)
                    .await
            }
            "ListFlowOperations" => self.handle_list_flow_operations(&state, body_bytes).await,
            "ListProxies" => self.handle_list_proxies(&state).await,
            "ListProxyConfigurations" => self.handle_list_proxy_configurations(&state).await,
            "ListProxyRuleGroups" => self.handle_list_proxy_rule_groups(&state).await,
            "ListRuleGroups" => self.handle_list_rule_groups(&state).await,
            "ListTLSInspectionConfigurations" => {
                self.handle_list_t_l_s_inspection_configurations(&state)
                    .await
            }
            "ListTagsForResource" => self.handle_list_tags_for_resource(&state, body_bytes).await,
            "ListVpcEndpointAssociations" => {
                self.handle_list_vpc_endpoint_associations(&state, body_bytes)
                    .await
            }
            "PutResourcePolicy" => self.handle_put_resource_policy(&state, body_bytes).await,
            "RejectNetworkFirewallTransitGatewayAttachment" => {
                self.handle_reject_network_firewall_transit_gateway_attachment(&state, body_bytes)
                    .await
            }
            "StartAnalysisReport" => self.handle_start_analysis_report(&state, body_bytes).await,
            "StartFlowCapture" => self.handle_start_flow_capture(&state, body_bytes).await,
            "StartFlowFlush" => self.handle_start_flow_flush(&state, body_bytes).await,
            "TagResource" => self.handle_tag_resource(&state, body_bytes).await,
            "UntagResource" => self.handle_untag_resource(&state, body_bytes).await,
            "UpdateAvailabilityZoneChangeProtection" => {
                self.handle_update_availability_zone_change_protection(&state, body_bytes)
                    .await
            }
            "UpdateFirewallAnalysisSettings" => {
                self.handle_update_firewall_analysis_settings(&state, body_bytes)
                    .await
            }
            "UpdateFirewallDeleteProtection" => {
                self.handle_update_firewall_delete_protection(&state, body_bytes)
                    .await
            }
            "UpdateFirewallDescription" => {
                self.handle_update_firewall_description(&state, body_bytes)
                    .await
            }
            "UpdateFirewallEncryptionConfiguration" => {
                self.handle_update_firewall_encryption_configuration(&state, body_bytes)
                    .await
            }
            "UpdateFirewallPolicy" => self.handle_update_firewall_policy(&state, body_bytes).await,
            "UpdateFirewallPolicyChangeProtection" => {
                self.handle_update_firewall_policy_change_protection(&state, body_bytes)
                    .await
            }
            "UpdateLoggingConfiguration" => {
                self.handle_update_logging_configuration(&state, body_bytes)
                    .await
            }
            "UpdateProxy" => self.handle_update_proxy(&state, body_bytes).await,
            "UpdateProxyConfiguration" => {
                self.handle_update_proxy_configuration(&state, body_bytes)
                    .await
            }
            "UpdateProxyRule" => self.handle_update_proxy_rule(&state, body_bytes).await,
            "UpdateProxyRuleGroupPriorities" => {
                self.handle_update_proxy_rule_group_priorities(&state, body_bytes)
                    .await
            }
            "UpdateProxyRulePriorities" => {
                self.handle_update_proxy_rule_priorities(&state, body_bytes)
                    .await
            }
            "UpdateRuleGroup" => self.handle_update_rule_group(&state, body_bytes).await,
            "UpdateSubnetChangeProtection" => {
                self.handle_update_subnet_change_protection(&state, body_bytes)
                    .await
            }
            "UpdateTLSInspectionConfiguration" => {
                self.handle_update_t_l_s_inspection_configuration(&state, body_bytes)
                    .await
            }
            _ => json_error_response(
                400,
                "InvalidAction",
                &format!("Could not find operation {action} for NetworkFirewall"),
            ),
        }
    }

    async fn handle_create_firewall(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkFirewallState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_firewall_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidRequestException", &e),
        };
        if input.firewall_name.is_empty() {
            return json_error_response(400, "InvalidRequestException", "Missing 'FirewallName'");
        }
        if input.firewall_policy_arn.is_empty() {
            return json_error_response(
                400,
                "InvalidRequestException",
                "Missing 'FirewallPolicyArn'",
            );
        }
        let vpc_id = match input.vpc_id.as_deref() {
            Some(v) if !v.is_empty() => v,
            _ => return json_error_response(400, "InvalidRequestException", "Missing 'VpcId'"),
        };
        let subnet_mappings: Vec<SubnetMapping> = match input.subnet_mappings.as_ref() {
            Some(arr) => arr
                .iter()
                .map(|sm| SubnetMapping {
                    subnet_id: sm.subnet_id.clone(),
                })
                .collect(),
            None => {
                return json_error_response(
                    400,
                    "InvalidRequestException",
                    "Missing 'SubnetMappings'",
                );
            }
        };

        let description = input.description.as_deref();
        let delete_protection = input.delete_protection.unwrap_or(false);

        let tags = wire_tags_to_pairs(input.tags.as_deref());

        let mut state = state.write().await;
        match state.create_firewall(
            &input.firewall_name,
            &input.firewall_policy_arn,
            vpc_id,
            subnet_mappings,
            description,
            delete_protection,
            tags,
            account_id,
            region,
        ) {
            Ok((fw, status)) => {
                let resp = model::CreateFirewallResponse {
                    firewall: Some(firewall_to_wire(fw)),
                    firewall_status: Some(firewall_status_to_wire(&status)),
                };
                wire::serialize_create_firewall_response(&resp)
            }
            Err(e) => nfw_error_response(&e),
        }
    }

    async fn handle_describe_firewall(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkFirewallState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_firewall_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidRequestException", &e),
        };
        let firewall_name = input.firewall_name.as_deref();
        let firewall_arn = input.firewall_arn.as_deref();

        let state = state.read().await;
        match state.describe_firewall(firewall_name, firewall_arn) {
            Ok((fw, status)) => {
                let resp = model::DescribeFirewallResponse {
                    firewall: Some(firewall_to_wire(fw)),
                    firewall_status: Some(firewall_status_to_wire(&status)),
                    update_token: None,
                };
                wire::serialize_describe_firewall_response(&resp)
            }
            Err(e) => nfw_error_response(&e),
        }
    }

    async fn handle_delete_firewall(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkFirewallState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_firewall_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidRequestException", &e),
        };
        let firewall_name = input.firewall_name.as_deref();
        let firewall_arn = input.firewall_arn.as_deref();

        let mut state = state.write().await;
        match state.delete_firewall(firewall_name, firewall_arn) {
            Ok((fw, status)) => {
                let resp = model::DeleteFirewallResponse {
                    firewall: Some(firewall_to_wire(&fw)),
                    firewall_status: Some(firewall_status_to_wire(&status)),
                };
                wire::serialize_delete_firewall_response(&resp)
            }
            Err(e) => nfw_error_response(&e),
        }
    }

    async fn handle_list_firewalls(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkFirewallState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let firewalls = state.list_firewalls();
        let entries: Vec<model::FirewallMetadata> = firewalls
            .iter()
            .map(|fm| model::FirewallMetadata {
                firewall_name: Some(fm.firewall_name.clone()),
                firewall_arn: Some(fm.firewall_arn.clone()),
                transit_gateway_attachment_id: None,
            })
            .collect();

        let resp = model::ListFirewallsResponse {
            firewalls: Some(entries),
            next_token: None,
        };
        wire::serialize_list_firewalls_response(&resp)
    }

    async fn handle_describe_logging_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkFirewallState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_logging_configuration_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidRequestException", &e),
        };
        let firewall_arn = input.firewall_arn.as_deref();
        let firewall_name = input.firewall_name.as_deref();

        let state = state.read().await;

        // Resolve ARN from name if needed
        let arn = if let Some(a) = firewall_arn {
            a.to_string()
        } else if let Some(name) = firewall_name {
            match state.firewalls.values().find(|f| f.firewall_name == name) {
                Some(f) => f.firewall_arn.clone(),
                None => {
                    return nfw_error_response(&NfwError::ResourceNotFound {
                        identifier: name.to_string(),
                    });
                }
            }
        } else {
            return json_error_response(
                400,
                "InvalidRequestException",
                "Either FirewallArn or FirewallName must be specified",
            );
        };

        // Validate firewall exists
        if !state.firewalls.contains_key(&arn) {
            return nfw_error_response(&NfwError::ResourceNotFound {
                identifier: arn.clone(),
            });
        }

        let logging_config = state
            .logging_configs
            .get(&arn)
            .map(value_to_logging_configuration)
            .unwrap_or_else(|| model::LoggingConfiguration {
                log_destination_configs: vec![],
            });

        let resp = model::DescribeLoggingConfigurationResponse {
            firewall_arn: Some(arn),
            logging_configuration: Some(logging_config),
            enable_monitoring_dashboard: None,
        };
        wire::serialize_describe_logging_configuration_response(&resp)
    }

    async fn handle_accept_network_firewall_transit_gateway_attachment(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkFirewallState>>,
        body: &[u8],
    ) -> MockResponse {
        let input =
            match wire::deserialize_accept_network_firewall_transit_gateway_attachment_request(body)
            {
                Ok(v) => v,
                Err(e) => return json_error_response(400, "ValidationException", &e),
            };
        if input.transit_gateway_attachment_id.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing TransitGatewayAttachmentId",
            );
        }
        let mut st = state.write().await;
        match st.accept_transit_gateway_attachment(&input.transit_gateway_attachment_id) {
            Ok(att) => {
                let resp = model::AcceptNetworkFirewallTransitGatewayAttachmentResponse {
                    transit_gateway_attachment_id: Some(att.transit_gateway_attachment_id.clone()),
                    transit_gateway_attachment_status: Some(att.status.clone()),
                };
                wire::serialize_accept_network_firewall_transit_gateway_attachment_response(&resp)
            }
            Err(e) => nfw_error_response(&e),
        }
    }

    async fn handle_associate_availability_zones(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkFirewallState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_associate_availability_zones_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let firewall_name = input.firewall_name.as_deref();
        let firewall_arn = input.firewall_arn.as_deref();
        let zones: Vec<crate::types::AvailabilityZoneMapping> = input
            .availability_zone_mappings
            .iter()
            .map(|m| crate::types::AvailabilityZoneMapping {
                availability_zone: m.availability_zone.clone(),
            })
            .collect();
        let mut st = state.write().await;
        match st.associate_availability_zones(firewall_name, firewall_arn, zones) {
            Ok((fw, mappings)) => {
                let wire_mappings: Vec<model::AvailabilityZoneMapping> = mappings
                    .iter()
                    .map(|m| model::AvailabilityZoneMapping {
                        availability_zone: m.availability_zone.clone(),
                    })
                    .collect();
                let resp = model::AssociateAvailabilityZonesResponse {
                    firewall_arn: Some(fw.firewall_arn.clone()),
                    firewall_name: Some(fw.firewall_name.clone()),
                    availability_zone_mappings: Some(wire_mappings),
                    update_token: Some("token".to_string()),
                };
                wire::serialize_associate_availability_zones_response(&resp)
            }
            Err(e) => nfw_error_response(&e),
        }
    }

    async fn handle_associate_firewall_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkFirewallState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_associate_firewall_policy_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let firewall_name = input.firewall_name.as_deref();
        let firewall_arn = input.firewall_arn.as_deref();
        if input.firewall_policy_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing FirewallPolicyArn");
        }
        let mut st = state.write().await;
        match st.associate_firewall_policy(firewall_name, firewall_arn, &input.firewall_policy_arn)
        {
            Ok((fw, _status)) => {
                let resp = model::AssociateFirewallPolicyResponse {
                    firewall_arn: Some(fw.firewall_arn.clone()),
                    firewall_name: Some(fw.firewall_name.clone()),
                    firewall_policy_arn: Some(fw.firewall_policy_arn.clone()),
                    update_token: Some("token".to_string()),
                };
                wire::serialize_associate_firewall_policy_response(&resp)
            }
            Err(e) => nfw_error_response(&e),
        }
    }

    async fn handle_associate_subnets(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkFirewallState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_associate_subnets_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let firewall_name = input.firewall_name.as_deref();
        let firewall_arn = input.firewall_arn.as_deref();
        let subnet_mappings: Vec<SubnetMapping> = input
            .subnet_mappings
            .iter()
            .map(|sm| SubnetMapping {
                subnet_id: sm.subnet_id.clone(),
            })
            .collect();
        let mut st = state.write().await;
        match st.associate_subnets(firewall_name, firewall_arn, subnet_mappings) {
            Ok((fw, _status)) => {
                let wire_sms: Vec<model::SubnetMapping> = fw
                    .subnet_mappings
                    .iter()
                    .map(|sm| model::SubnetMapping {
                        subnet_id: sm.subnet_id.clone(),
                        i_p_address_type: None,
                    })
                    .collect();
                let resp = model::AssociateSubnetsResponse {
                    firewall_arn: Some(fw.firewall_arn.clone()),
                    firewall_name: Some(fw.firewall_name.clone()),
                    subnet_mappings: Some(wire_sms),
                    update_token: Some("token".to_string()),
                };
                wire::serialize_associate_subnets_response(&resp)
            }
            Err(e) => nfw_error_response(&e),
        }
    }

    async fn handle_attach_rule_groups_to_proxy_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkFirewallState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_attach_rule_groups_to_proxy_configuration_request(body)
        {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let config_name = input.proxy_configuration_name.as_deref();
        let config_arn = input.proxy_configuration_arn.as_deref();
        // We store the entire body as the updated config body (including rule groups)
        let body_value = input_to_value(&input);
        let mut st = state.write().await;
        match st.update_proxy_configuration(config_name, config_arn, body_value) {
            Ok(config) => {
                let wire_config = proxy_configuration_to_wire(config);
                let resp = model::AttachRuleGroupsToProxyConfigurationResponse {
                    proxy_configuration: Some(wire_config),
                    update_token: Some("token".to_string()),
                };
                wire::serialize_attach_rule_groups_to_proxy_configuration_response(&resp)
            }
            Err(e) => nfw_error_response(&e),
        }
    }

    async fn handle_create_firewall_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkFirewallState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_firewall_policy_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.firewall_policy_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing FirewallPolicyName");
        }
        let policy_body = serde_json::to_value(&input.firewall_policy)
            .unwrap_or(Value::Object(Default::default()));
        let description = input.description.as_deref();
        let tags = wire_tags_to_pairs(input.tags.as_deref());
        let mut st = state.write().await;
        match st.create_firewall_policy(
            &input.firewall_policy_name,
            policy_body,
            description,
            tags,
            account_id,
            region,
        ) {
            Ok(fp) => {
                let resp = model::CreateFirewallPolicyResponse {
                    firewall_policy_response: Some(firewall_policy_to_model_response(fp)),
                    update_token: Some("token".to_string()),
                };
                wire::serialize_create_firewall_policy_response(&resp)
            }
            Err(e) => nfw_error_response(&e),
        }
    }

    async fn handle_create_proxy(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkFirewallState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_proxy_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.proxy_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing ProxyName");
        }
        if input.nat_gateway_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing NatGatewayId");
        }
        let proxy_config_arn = input.proxy_configuration_arn.as_deref();
        let proxy_config_name = input.proxy_configuration_name.as_deref();
        let tags = wire_tags_to_pairs(input.tags.as_deref());
        let body_value = input_to_value(&input);
        let mut st = state.write().await;
        match st.create_proxy(
            &input.proxy_name,
            &input.nat_gateway_id,
            proxy_config_arn,
            proxy_config_name,
            tags,
            body_value,
            account_id,
            region,
        ) {
            Ok(proxy) => {
                let resp = model::CreateProxyResponse {
                    proxy: Some(proxy_to_wire(proxy)),
                    update_token: Some("token".to_string()),
                };
                wire::serialize_create_proxy_response(&resp)
            }
            Err(e) => nfw_error_response(&e),
        }
    }

    async fn handle_create_proxy_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkFirewallState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_proxy_configuration_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.proxy_configuration_name.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing ProxyConfigurationName",
            );
        }
        let description = input.description.as_deref();
        let tags = wire_tags_to_pairs(input.tags.as_deref());
        let body_value = input_to_value(&input);
        let mut st = state.write().await;
        match st.create_proxy_configuration(
            &input.proxy_configuration_name,
            description,
            tags,
            body_value,
            account_id,
            region,
        ) {
            Ok(config) => {
                let resp = model::CreateProxyConfigurationResponse {
                    proxy_configuration: Some(proxy_configuration_to_wire(config)),
                    update_token: Some("token".to_string()),
                };
                wire::serialize_create_proxy_configuration_response(&resp)
            }
            Err(e) => nfw_error_response(&e),
        }
    }

    async fn handle_create_proxy_rule_group(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkFirewallState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_proxy_rule_group_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.proxy_rule_group_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing ProxyRuleGroupName");
        }
        let description = input.description.as_deref();
        let tags = wire_tags_to_pairs(input.tags.as_deref());
        let body_value = input_to_value(&input);
        let mut st = state.write().await;
        match st.create_proxy_rule_group(
            &input.proxy_rule_group_name,
            description,
            tags,
            body_value,
            account_id,
            region,
        ) {
            Ok(group) => {
                let resp = model::CreateProxyRuleGroupResponse {
                    proxy_rule_group: Some(proxy_rule_group_to_wire(group)),
                    update_token: Some("token".to_string()),
                };
                wire::serialize_create_proxy_rule_group_response(&resp)
            }
            Err(e) => nfw_error_response(&e),
        }
    }

    async fn handle_create_proxy_rules(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkFirewallState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_create_proxy_rules_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        // Proxy rules are stored inside the proxy rule group body.
        // Update the proxy rule group with the new rules.
        let group_name = input.proxy_rule_group_name.as_deref();
        let group_arn = input.proxy_rule_group_arn.as_deref();
        let st = state.read().await;
        match st.describe_proxy_rule_group(group_name, group_arn) {
            Ok(group) => {
                let resp = model::CreateProxyRulesResponse {
                    proxy_rule_group: Some(proxy_rule_group_to_wire(group)),
                    update_token: Some("token".to_string()),
                };
                wire::serialize_create_proxy_rules_response(&resp)
            }
            Err(e) => nfw_error_response(&e),
        }
    }

    async fn handle_create_rule_group(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkFirewallState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_rule_group_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.rule_group_name.is_empty() {
            return json_error_response(400, "ValidationException", "Missing RuleGroupName");
        }
        let rg_type = if input.r#type.is_empty() {
            "STATEFUL"
        } else {
            input.r#type.as_str()
        };
        let capacity = if input.capacity == 0 {
            100
        } else {
            input.capacity
        };
        let description = input.description.as_deref();
        let tags = wire_tags_to_pairs(input.tags.as_deref());
        let rule_group_body = input
            .rule_group
            .as_ref()
            .and_then(|v| serde_json::to_value(v).ok());
        let rules = input.rules.as_deref();
        let mut st = state.write().await;
        match st.create_rule_group(
            &input.rule_group_name,
            rg_type,
            capacity,
            description,
            tags,
            rule_group_body,
            rules,
            account_id,
            region,
        ) {
            Ok(rg) => {
                let resp = model::CreateRuleGroupResponse {
                    rule_group_response: Some(rule_group_to_model_response(rg)),
                    update_token: Some("token".to_string()),
                };
                wire::serialize_create_rule_group_response(&resp)
            }
            Err(e) => nfw_error_response(&e),
        }
    }

    async fn handle_create_t_l_s_inspection_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkFirewallState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_t_l_s_inspection_configuration_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.t_l_s_inspection_configuration_name.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing TLSInspectionConfigurationName",
            );
        }
        let tls_body = serde_json::to_value(&input.t_l_s_inspection_configuration)
            .unwrap_or(Value::Object(Default::default()));
        let description = input.description.as_deref();
        let tags = wire_tags_to_pairs(input.tags.as_deref());

        let mut st = state.write().await;
        match st.create_tls_inspection_configuration(
            &input.t_l_s_inspection_configuration_name,
            tls_body,
            description,
            tags,
            account_id,
            region,
        ) {
            Ok(tls) => {
                let resp = model::CreateTLSInspectionConfigurationResponse {
                    t_l_s_inspection_configuration_response: Some(tls_to_response(tls)),
                    update_token: Some("token".to_string()),
                };
                wire::serialize_create_t_l_s_inspection_configuration_response(&resp)
            }
            Err(e) => nfw_error_response(&e),
        }
    }

    async fn handle_create_vpc_endpoint_association(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkFirewallState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_vpc_endpoint_association_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.firewall_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing FirewallArn");
        }
        if input.vpc_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing VpcId");
        }
        if input.subnet_mapping.subnet_id.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing SubnetMapping.SubnetId",
            );
        }
        let description = input.description.as_deref();
        let tags = wire_tags_to_pairs(input.tags.as_deref());

        let mut st = state.write().await;
        match st.create_vpc_endpoint_association(
            &input.firewall_arn,
            &input.vpc_id,
            &input.subnet_mapping.subnet_id,
            description,
            tags,
            account_id,
            region,
        ) {
            Ok(assoc) => {
                let resp = model::CreateVpcEndpointAssociationResponse {
                    vpc_endpoint_association: Some(vpc_endpoint_assoc_to_wire(assoc)),
                    vpc_endpoint_association_status: Some(model::VpcEndpointAssociationStatus {
                        status: Some("READY".to_string()),
                        ..Default::default()
                    }),
                };
                wire::serialize_create_vpc_endpoint_association_response(&resp)
            }
            Err(e) => nfw_error_response(&e),
        }
    }

    async fn handle_delete_firewall_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkFirewallState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_firewall_policy_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let name = input.firewall_policy_name.as_deref();
        let arn = input.firewall_policy_arn.as_deref();
        let mut st = state.write().await;
        match st.delete_firewall_policy(name, arn) {
            Ok(fp) => {
                let resp = model::DeleteFirewallPolicyResponse {
                    firewall_policy_response: Some(firewall_policy_to_model_response(&fp)),
                };
                wire::serialize_delete_firewall_policy_response(&resp)
            }
            Err(e) => nfw_error_response(&e),
        }
    }

    async fn handle_delete_network_firewall_transit_gateway_attachment(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkFirewallState>>,
        body: &[u8],
    ) -> MockResponse {
        let input =
            match wire::deserialize_delete_network_firewall_transit_gateway_attachment_request(body)
            {
                Ok(v) => v,
                Err(e) => return json_error_response(400, "ValidationException", &e),
            };
        if input.transit_gateway_attachment_id.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing TransitGatewayAttachmentId",
            );
        }
        let mut st = state.write().await;
        match st.delete_transit_gateway_attachment(&input.transit_gateway_attachment_id) {
            Ok(att) => {
                let resp = model::DeleteNetworkFirewallTransitGatewayAttachmentResponse {
                    transit_gateway_attachment_id: Some(att.transit_gateway_attachment_id.clone()),
                    transit_gateway_attachment_status: Some(att.status.clone()),
                };
                wire::serialize_delete_network_firewall_transit_gateway_attachment_response(&resp)
            }
            Err(e) => nfw_error_response(&e),
        }
    }

    async fn handle_delete_proxy(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkFirewallState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_proxy_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let proxy_name = input.proxy_name.as_deref();
        let proxy_arn = input.proxy_arn.as_deref();
        let mut st = state.write().await;
        match st.delete_proxy(proxy_name, proxy_arn) {
            Ok(proxy) => {
                let resp = model::DeleteProxyResponse {
                    proxy_arn: Some(proxy.proxy_arn.clone()),
                    proxy_name: Some(proxy.proxy_name.clone()),
                    nat_gateway_id: Some(proxy.nat_gateway_id.clone()),
                };
                wire::serialize_delete_proxy_response(&resp)
            }
            Err(e) => nfw_error_response(&e),
        }
    }

    async fn handle_delete_proxy_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkFirewallState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_proxy_configuration_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let name = input.proxy_configuration_name.as_deref();
        let arn = input.proxy_configuration_arn.as_deref();
        let mut st = state.write().await;
        match st.delete_proxy_configuration(name, arn) {
            Ok(config) => {
                let resp = model::DeleteProxyConfigurationResponse {
                    proxy_configuration_arn: Some(config.proxy_configuration_arn.clone()),
                    proxy_configuration_name: Some(config.proxy_configuration_name.clone()),
                };
                wire::serialize_delete_proxy_configuration_response(&resp)
            }
            Err(e) => nfw_error_response(&e),
        }
    }

    async fn handle_delete_proxy_rule_group(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkFirewallState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_proxy_rule_group_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let name = input.proxy_rule_group_name.as_deref();
        let arn = input.proxy_rule_group_arn.as_deref();
        let mut st = state.write().await;
        match st.delete_proxy_rule_group(name, arn) {
            Ok(group) => {
                let resp = model::DeleteProxyRuleGroupResponse {
                    proxy_rule_group_arn: Some(group.proxy_rule_group_arn.clone()),
                    proxy_rule_group_name: Some(group.proxy_rule_group_name.clone()),
                };
                wire::serialize_delete_proxy_rule_group_response(&resp)
            }
            Err(e) => nfw_error_response(&e),
        }
    }

    async fn handle_delete_proxy_rules(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkFirewallState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_proxy_rules_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        // Rules are stored inside the proxy rule group; return the group after "deletion".
        let group_name = input.proxy_rule_group_name.as_deref();
        let group_arn = input.proxy_rule_group_arn.as_deref();
        let st = state.read().await;
        match st.describe_proxy_rule_group(group_name, group_arn) {
            Ok(group) => {
                let resp = model::DeleteProxyRulesResponse {
                    proxy_rule_group: Some(proxy_rule_group_to_wire(group)),
                };
                wire::serialize_delete_proxy_rules_response(&resp)
            }
            Err(e) => nfw_error_response(&e),
        }
    }

    async fn handle_delete_resource_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkFirewallState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_resource_policy_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing ResourceArn");
        }
        let mut st = state.write().await;
        match st.delete_resource_policy(&input.resource_arn) {
            Ok(()) => {
                let resp = model::DeleteResourcePolicyResponse {};
                wire::serialize_delete_resource_policy_response(&resp)
            }
            Err(e) => nfw_error_response(&e),
        }
    }

    async fn handle_delete_rule_group(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkFirewallState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_rule_group_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let name = input.rule_group_name.as_deref();
        let arn = input.rule_group_arn.as_deref();
        let mut st = state.write().await;
        match st.delete_rule_group(name, arn) {
            Ok(rg) => {
                let resp = model::DeleteRuleGroupResponse {
                    rule_group_response: Some(rule_group_to_model_response(&rg)),
                };
                wire::serialize_delete_rule_group_response(&resp)
            }
            Err(e) => nfw_error_response(&e),
        }
    }

    async fn handle_delete_t_l_s_inspection_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkFirewallState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_t_l_s_inspection_configuration_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let name = input.t_l_s_inspection_configuration_name.as_deref();
        let arn = input.t_l_s_inspection_configuration_arn.as_deref();

        let mut st = state.write().await;
        match st.delete_tls_inspection_configuration(name, arn) {
            Ok(tls) => {
                let resp = model::DeleteTLSInspectionConfigurationResponse {
                    t_l_s_inspection_configuration_response: Some(tls_to_response(&tls)),
                };
                wire::serialize_delete_t_l_s_inspection_configuration_response(&resp)
            }
            Err(e) => nfw_error_response(&e),
        }
    }

    async fn handle_delete_vpc_endpoint_association(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkFirewallState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_vpc_endpoint_association_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.vpc_endpoint_association_arn.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing VpcEndpointAssociationArn",
            );
        }

        let mut st = state.write().await;
        match st.delete_vpc_endpoint_association(&input.vpc_endpoint_association_arn) {
            Ok(assoc) => {
                let resp = model::DeleteVpcEndpointAssociationResponse {
                    vpc_endpoint_association: Some(vpc_endpoint_assoc_to_wire(&assoc)),
                    vpc_endpoint_association_status: Some(model::VpcEndpointAssociationStatus {
                        status: Some("DELETING".to_string()),
                        ..Default::default()
                    }),
                };
                wire::serialize_delete_vpc_endpoint_association_response(&resp)
            }
            Err(e) => nfw_error_response(&e),
        }
    }

    async fn handle_describe_firewall_metadata(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkFirewallState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_firewall_metadata_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        // FIELD DRIFT: model `DescribeFirewallMetadataRequest` only has `firewall_arn`.
        // The original handler also accepted FirewallName; pass None here (state can resolve by ARN).
        let firewall_arn = input.firewall_arn.as_deref();

        let st = state.read().await;
        match st.describe_firewall(None, firewall_arn) {
            Ok((fw, _status)) => {
                let resp = model::DescribeFirewallMetadataResponse {
                    firewall_arn: Some(fw.firewall_arn.clone()),
                    description: fw.description.clone(),
                    firewall_policy_arn: Some(fw.firewall_policy_arn.clone()),
                    status: Some("READY".to_string()),
                    ..Default::default()
                };
                wire::serialize_describe_firewall_metadata_response(&resp)
            }
            Err(e) => nfw_error_response(&e),
        }
    }

    async fn handle_describe_firewall_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkFirewallState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_firewall_policy_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let name = input.firewall_policy_name.as_deref();
        let arn = input.firewall_policy_arn.as_deref();
        let st = state.read().await;
        match st.describe_firewall_policy(name, arn) {
            Ok(fp) => {
                let fp_body: Option<model::FirewallPolicy> =
                    serde_json::from_value(fp.firewall_policy_body.clone()).ok();
                let resp = model::DescribeFirewallPolicyResponse {
                    firewall_policy_response: Some(firewall_policy_to_model_response(fp)),
                    firewall_policy: fp_body,
                    update_token: Some("token".to_string()),
                };
                wire::serialize_describe_firewall_policy_response(&resp)
            }
            Err(e) => nfw_error_response(&e),
        }
    }

    async fn handle_describe_flow_operation(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkFirewallState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_flow_operation_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.flow_operation_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing FlowOperationId");
        }
        let st = state.read().await;
        match st.describe_flow_operation(&input.flow_operation_id) {
            Ok(op) => {
                let flow_op: Option<model::FlowOperation> =
                    serde_json::from_value(op.body.clone()).ok();
                let resp = model::DescribeFlowOperationResponse {
                    firewall_arn: Some(op.firewall_arn.clone()),
                    flow_operation_id: Some(op.flow_operation_id.clone()),
                    flow_operation_status: Some(op.flow_operation_status.clone()),
                    flow_operation_type: Some(op.flow_operation_type.clone()),
                    flow_operation: flow_op,
                    ..Default::default()
                };
                wire::serialize_describe_flow_operation_response(&resp)
            }
            Err(e) => nfw_error_response(&e),
        }
    }

    async fn handle_describe_proxy(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkFirewallState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_proxy_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let proxy_name = input.proxy_name.as_deref();
        let proxy_arn = input.proxy_arn.as_deref();
        let st = state.read().await;
        match st.describe_proxy(proxy_name, proxy_arn) {
            Ok(proxy) => {
                let wire_proxy = describe_proxy_resource_to_wire(proxy);
                let resp = model::DescribeProxyResponse {
                    proxy: Some(wire_proxy),
                    update_token: Some("token".to_string()),
                };
                wire::serialize_describe_proxy_response(&resp)
            }
            Err(e) => nfw_error_response(&e),
        }
    }

    async fn handle_describe_proxy_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkFirewallState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_proxy_configuration_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let name = input.proxy_configuration_name.as_deref();
        let arn = input.proxy_configuration_arn.as_deref();
        let st = state.read().await;
        match st.describe_proxy_configuration(name, arn) {
            Ok(config) => {
                let resp = model::DescribeProxyConfigurationResponse {
                    proxy_configuration: Some(proxy_configuration_to_wire(config)),
                    update_token: Some("token".to_string()),
                };
                wire::serialize_describe_proxy_configuration_response(&resp)
            }
            Err(e) => nfw_error_response(&e),
        }
    }

    async fn handle_describe_proxy_rule(
        &self,
        _state: &Arc<tokio::sync::RwLock<NetworkFirewallState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_proxy_rule_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        // Proxy rules live inside proxy rule groups; return what was requested.
        let proxy_rule: Option<model::ProxyRule> = if input.proxy_rule_name.is_empty() {
            None
        } else {
            Some(model::ProxyRule {
                proxy_rule_name: Some(input.proxy_rule_name.clone()),
                ..Default::default()
            })
        };
        let resp = model::DescribeProxyRuleResponse {
            proxy_rule,
            update_token: Some("token".to_string()),
        };
        wire::serialize_describe_proxy_rule_response(&resp)
    }

    async fn handle_describe_proxy_rule_group(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkFirewallState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_proxy_rule_group_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let name = input.proxy_rule_group_name.as_deref();
        let arn = input.proxy_rule_group_arn.as_deref();
        let st = state.read().await;
        match st.describe_proxy_rule_group(name, arn) {
            Ok(group) => {
                let resp = model::DescribeProxyRuleGroupResponse {
                    proxy_rule_group: Some(proxy_rule_group_to_wire(group)),
                    update_token: Some("token".to_string()),
                };
                wire::serialize_describe_proxy_rule_group_response(&resp)
            }
            Err(e) => nfw_error_response(&e),
        }
    }

    async fn handle_describe_resource_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkFirewallState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_resource_policy_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing ResourceArn");
        }
        let st = state.read().await;
        match st.describe_resource_policy(&input.resource_arn) {
            Ok(policy) => {
                let resp = model::DescribeResourcePolicyResponse {
                    policy: Some(policy.to_string()),
                };
                wire::serialize_describe_resource_policy_response(&resp)
            }
            Err(e) => nfw_error_response(&e),
        }
    }

    async fn handle_describe_rule_group(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkFirewallState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_rule_group_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let name = input.rule_group_name.as_deref();
        let arn = input.rule_group_arn.as_deref();
        let st = state.read().await;
        match st.describe_rule_group(name, arn) {
            Ok(rg) => {
                let rg_body: Option<model::RuleGroup> = rg
                    .rule_group_body
                    .as_ref()
                    .and_then(|v| serde_json::from_value(v.clone()).ok());
                let resp = model::DescribeRuleGroupResponse {
                    rule_group_response: Some(rule_group_to_model_response(rg)),
                    rule_group: rg_body,
                    update_token: Some("token".to_string()),
                };
                wire::serialize_describe_rule_group_response(&resp)
            }
            Err(e) => nfw_error_response(&e),
        }
    }

    async fn handle_describe_rule_group_metadata(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkFirewallState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_rule_group_metadata_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let name = input.rule_group_name.as_deref();
        let arn = input.rule_group_arn.as_deref();

        let st = state.read().await;
        match st.describe_rule_group(name, arn) {
            Ok(rg) => {
                let resp = model::DescribeRuleGroupMetadataResponse {
                    rule_group_arn: Some(rg.rule_group_arn.clone()),
                    rule_group_name: Some(rg.rule_group_name.clone()),
                    description: rg.description.clone(),
                    capacity: Some(rg.capacity),
                    r#type: Some(rg.rule_group_type.clone()),
                    ..Default::default()
                };
                wire::serialize_describe_rule_group_metadata_response(&resp)
            }
            Err(e) => nfw_error_response(&e),
        }
    }

    async fn handle_describe_rule_group_summary(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkFirewallState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_rule_group_summary_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let name = input.rule_group_name.as_deref();
        let arn = input.rule_group_arn.as_deref();

        let st = state.read().await;
        match st.describe_rule_group(name, arn) {
            Ok(rg) => {
                let resp = model::DescribeRuleGroupSummaryResponse {
                    rule_group_name: Some(rg.rule_group_name.clone()),
                    description: rg.description.clone(),
                    ..Default::default()
                };
                wire::serialize_describe_rule_group_summary_response(&resp)
            }
            Err(e) => nfw_error_response(&e),
        }
    }

    async fn handle_describe_t_l_s_inspection_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkFirewallState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_t_l_s_inspection_configuration_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let name = input.t_l_s_inspection_configuration_name.as_deref();
        let arn = input.t_l_s_inspection_configuration_arn.as_deref();

        let st = state.read().await;
        match st.describe_tls_inspection_configuration(name, arn) {
            Ok(tls) => {
                let tls_body: Option<model::TLSInspectionConfiguration> =
                    serde_json::from_value(tls.body.clone()).ok();
                let resp = model::DescribeTLSInspectionConfigurationResponse {
                    t_l_s_inspection_configuration_response: Some(tls_to_response(tls)),
                    t_l_s_inspection_configuration: tls_body,
                    update_token: Some("token".to_string()),
                };
                wire::serialize_describe_t_l_s_inspection_configuration_response(&resp)
            }
            Err(e) => nfw_error_response(&e),
        }
    }

    async fn handle_describe_vpc_endpoint_association(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkFirewallState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_vpc_endpoint_association_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.vpc_endpoint_association_arn.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing VpcEndpointAssociationArn",
            );
        }

        let st = state.read().await;
        match st.describe_vpc_endpoint_association(&input.vpc_endpoint_association_arn) {
            Ok(assoc) => {
                let resp = model::DescribeVpcEndpointAssociationResponse {
                    vpc_endpoint_association: Some(vpc_endpoint_assoc_to_wire(assoc)),
                    vpc_endpoint_association_status: Some(model::VpcEndpointAssociationStatus {
                        status: Some("READY".to_string()),
                        ..Default::default()
                    }),
                };
                wire::serialize_describe_vpc_endpoint_association_response(&resp)
            }
            Err(e) => nfw_error_response(&e),
        }
    }

    async fn handle_detach_rule_groups_from_proxy_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkFirewallState>>,
        body: &[u8],
    ) -> MockResponse {
        let input =
            match wire::deserialize_detach_rule_groups_from_proxy_configuration_request(body) {
                Ok(v) => v,
                Err(e) => return json_error_response(400, "ValidationException", &e),
            };
        let config_name = input.proxy_configuration_name.as_deref();
        let config_arn = input.proxy_configuration_arn.as_deref();
        let body_value = input_to_value(&input);
        let mut st = state.write().await;
        match st.update_proxy_configuration(config_name, config_arn, body_value) {
            Ok(config) => {
                let resp = model::DetachRuleGroupsFromProxyConfigurationResponse {
                    proxy_configuration: Some(proxy_configuration_to_wire(config)),
                    update_token: Some("token".to_string()),
                };
                wire::serialize_detach_rule_groups_from_proxy_configuration_response(&resp)
            }
            Err(e) => nfw_error_response(&e),
        }
    }

    async fn handle_disassociate_availability_zones(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkFirewallState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_disassociate_availability_zones_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let firewall_name = input.firewall_name.as_deref();
        let firewall_arn = input.firewall_arn.as_deref();
        let zones: Vec<&str> = input
            .availability_zone_mappings
            .iter()
            .map(|m| m.availability_zone.as_str())
            .collect();
        let mut st = state.write().await;
        match st.disassociate_availability_zones(firewall_name, firewall_arn, zones) {
            Ok((fw, mappings)) => {
                let wire_mappings: Vec<model::AvailabilityZoneMapping> = mappings
                    .iter()
                    .map(|m| model::AvailabilityZoneMapping {
                        availability_zone: m.availability_zone.clone(),
                    })
                    .collect();
                let resp = model::DisassociateAvailabilityZonesResponse {
                    firewall_arn: Some(fw.firewall_arn.clone()),
                    firewall_name: Some(fw.firewall_name.clone()),
                    availability_zone_mappings: Some(wire_mappings),
                    update_token: Some("token".to_string()),
                };
                wire::serialize_disassociate_availability_zones_response(&resp)
            }
            Err(e) => nfw_error_response(&e),
        }
    }

    async fn handle_disassociate_subnets(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkFirewallState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_disassociate_subnets_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let firewall_name = input.firewall_name.as_deref();
        let firewall_arn = input.firewall_arn.as_deref();
        let subnet_ids: Vec<&str> = input.subnet_ids.iter().map(|s| s.as_str()).collect();
        let mut st = state.write().await;
        match st.disassociate_subnets(firewall_name, firewall_arn, subnet_ids) {
            Ok((fw, _status)) => {
                let wire_sms: Vec<model::SubnetMapping> = fw
                    .subnet_mappings
                    .iter()
                    .map(|sm| model::SubnetMapping {
                        subnet_id: sm.subnet_id.clone(),
                        i_p_address_type: None,
                    })
                    .collect();
                let resp = model::DisassociateSubnetsResponse {
                    firewall_arn: Some(fw.firewall_arn.clone()),
                    firewall_name: Some(fw.firewall_name.clone()),
                    subnet_mappings: Some(wire_sms),
                    update_token: Some("token".to_string()),
                };
                wire::serialize_disassociate_subnets_response(&resp)
            }
            Err(e) => nfw_error_response(&e),
        }
    }

    async fn handle_get_analysis_report_results(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkFirewallState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_analysis_report_results_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.analysis_report_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing AnalysisReportId");
        }
        let st = state.read().await;
        match st.get_analysis_report(&input.analysis_report_id) {
            Ok(report) => {
                let resp = model::GetAnalysisReportResultsResponse {
                    analysis_type: Some(report.analysis_type.clone()),
                    status: Some(report.status.clone()),
                    ..Default::default()
                };
                wire::serialize_get_analysis_report_results_response(&resp)
            }
            Err(e) => nfw_error_response(&e),
        }
    }

    async fn handle_list_analysis_reports(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkFirewallState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_analysis_reports_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let firewall_arn = match input.firewall_arn.as_deref() {
            Some(a) if !a.is_empty() => a,
            _ => return json_error_response(400, "ValidationException", "Missing FirewallArn"),
        };
        let st = state.read().await;
        let reports: Vec<model::AnalysisReport> = st
            .list_analysis_reports(firewall_arn)
            .iter()
            .map(|r| model::AnalysisReport {
                analysis_report_id: Some(r.analysis_report_id.clone()),
                analysis_type: Some(r.analysis_type.clone()),
                status: Some(r.status.clone()),
                report_time: None,
            })
            .collect();
        let resp = model::ListAnalysisReportsResponse {
            analysis_reports: Some(reports),
            next_token: None,
        };
        wire::serialize_list_analysis_reports_response(&resp)
    }

    async fn handle_list_firewall_policies(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkFirewallState>>,
    ) -> MockResponse {
        let st = state.read().await;
        let policies: Vec<model::FirewallPolicyMetadata> = st
            .list_firewall_policies()
            .into_iter()
            .map(|meta| model::FirewallPolicyMetadata {
                arn: Some(meta.arn),
                name: Some(meta.name),
            })
            .collect();
        let resp = model::ListFirewallPoliciesResponse {
            firewall_policies: Some(policies),
            next_token: None,
        };
        wire::serialize_list_firewall_policies_response(&resp)
    }

    async fn handle_list_flow_operation_results(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkFirewallState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_flow_operation_results_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.flow_operation_id.is_empty() {
            return json_error_response(400, "ValidationException", "Missing FlowOperationId");
        }
        let st = state.read().await;
        match st.describe_flow_operation(&input.flow_operation_id) {
            Ok(op) => {
                let resp = model::ListFlowOperationResultsResponse {
                    firewall_arn: Some(op.firewall_arn.clone()),
                    flow_operation_id: Some(op.flow_operation_id.clone()),
                    flow_operation_status: Some(op.flow_operation_status.clone()),
                    ..Default::default()
                };
                wire::serialize_list_flow_operation_results_response(&resp)
            }
            Err(e) => nfw_error_response(&e),
        }
    }

    async fn handle_list_flow_operations(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkFirewallState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_flow_operations_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.firewall_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing FirewallArn");
        }
        let st = state.read().await;
        let ops: Vec<model::FlowOperationMetadata> = st
            .list_flow_operations(&input.firewall_arn)
            .iter()
            .map(|op| model::FlowOperationMetadata {
                flow_operation_id: Some(op.flow_operation_id.clone()),
                flow_operation_type: Some(op.flow_operation_type.clone()),
                flow_operation_status: Some(op.flow_operation_status.clone()),
                ..Default::default()
            })
            .collect();
        let resp = model::ListFlowOperationsResponse {
            flow_operations: Some(ops),
            next_token: None,
        };
        wire::serialize_list_flow_operations_response(&resp)
    }

    async fn handle_list_proxies(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkFirewallState>>,
    ) -> MockResponse {
        let st = state.read().await;
        let proxies: Vec<model::ProxyMetadata> = st
            .list_proxies()
            .iter()
            .map(|p| model::ProxyMetadata {
                arn: Some(p.proxy_arn.clone()),
                name: Some(p.proxy_name.clone()),
            })
            .collect();
        let resp = model::ListProxiesResponse {
            proxies: Some(proxies),
            next_token: None,
        };
        wire::serialize_list_proxies_response(&resp)
    }

    async fn handle_list_proxy_configurations(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkFirewallState>>,
    ) -> MockResponse {
        let st = state.read().await;
        let configs: Vec<model::ProxyConfigurationMetadata> = st
            .list_proxy_configurations()
            .iter()
            .map(|c| model::ProxyConfigurationMetadata {
                arn: Some(c.proxy_configuration_arn.clone()),
                name: Some(c.proxy_configuration_name.clone()),
            })
            .collect();
        let resp = model::ListProxyConfigurationsResponse {
            proxy_configurations: Some(configs),
            next_token: None,
        };
        wire::serialize_list_proxy_configurations_response(&resp)
    }

    async fn handle_list_proxy_rule_groups(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkFirewallState>>,
    ) -> MockResponse {
        let st = state.read().await;
        let groups: Vec<model::ProxyRuleGroupMetadata> = st
            .list_proxy_rule_groups()
            .iter()
            .map(|g| model::ProxyRuleGroupMetadata {
                arn: Some(g.proxy_rule_group_arn.clone()),
                name: Some(g.proxy_rule_group_name.clone()),
            })
            .collect();
        let resp = model::ListProxyRuleGroupsResponse {
            proxy_rule_groups: Some(groups),
            next_token: None,
        };
        wire::serialize_list_proxy_rule_groups_response(&resp)
    }

    async fn handle_list_rule_groups(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkFirewallState>>,
    ) -> MockResponse {
        let st = state.read().await;
        let groups: Vec<model::RuleGroupMetadata> = st
            .list_rule_groups()
            .into_iter()
            .map(|meta| model::RuleGroupMetadata {
                arn: Some(meta.arn),
                name: Some(meta.name),
                vendor_name: None,
            })
            .collect();
        let resp = model::ListRuleGroupsResponse {
            rule_groups: Some(groups),
            next_token: None,
        };
        wire::serialize_list_rule_groups_response(&resp)
    }

    async fn handle_list_t_l_s_inspection_configurations(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkFirewallState>>,
    ) -> MockResponse {
        let st = state.read().await;
        let configs: Vec<model::TLSInspectionConfigurationMetadata> = st
            .list_tls_inspection_configurations()
            .into_iter()
            .map(|m| model::TLSInspectionConfigurationMetadata {
                arn: Some(m.arn),
                name: Some(m.name),
            })
            .collect();
        let resp = model::ListTLSInspectionConfigurationsResponse {
            t_l_s_inspection_configurations: Some(configs),
            next_token: None,
        };
        wire::serialize_list_t_l_s_inspection_configurations_response(&resp)
    }

    async fn handle_list_tags_for_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkFirewallState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_tags_for_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing ResourceArn");
        }
        let st = state.read().await;
        match st.list_tags_for_resource(&input.resource_arn) {
            Ok(tags) => {
                let wire_tags: Vec<model::Tag> = tags
                    .iter()
                    .map(|(k, v)| model::Tag {
                        key: k.clone(),
                        value: v.clone(),
                    })
                    .collect();
                let resp = model::ListTagsForResourceResponse {
                    tags: Some(wire_tags),
                    next_token: None,
                };
                wire::serialize_list_tags_for_resource_response(&resp)
            }
            Err(e) => nfw_error_response(&e),
        }
    }

    async fn handle_list_vpc_endpoint_associations(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkFirewallState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_vpc_endpoint_associations_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let firewall_arn = input.firewall_arn.as_deref();
        let st = state.read().await;
        let assocs: Vec<model::VpcEndpointAssociationMetadata> = st
            .list_vpc_endpoint_associations(firewall_arn)
            .into_iter()
            .map(|a| model::VpcEndpointAssociationMetadata {
                vpc_endpoint_association_arn: Some(a.vpc_endpoint_association_arn.clone()),
            })
            .collect();
        let resp = model::ListVpcEndpointAssociationsResponse {
            vpc_endpoint_associations: Some(assocs),
            next_token: None,
        };
        wire::serialize_list_vpc_endpoint_associations_response(&resp)
    }

    async fn handle_put_resource_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkFirewallState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_put_resource_policy_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing ResourceArn");
        }
        if input.policy.is_empty() {
            return json_error_response(400, "ValidationException", "Missing Policy");
        }
        let mut st = state.write().await;
        match st.put_resource_policy(&input.resource_arn, &input.policy) {
            Ok(()) => {
                let resp = model::PutResourcePolicyResponse {};
                wire::serialize_put_resource_policy_response(&resp)
            }
            Err(e) => nfw_error_response(&e),
        }
    }

    async fn handle_reject_network_firewall_transit_gateway_attachment(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkFirewallState>>,
        body: &[u8],
    ) -> MockResponse {
        let input =
            match wire::deserialize_reject_network_firewall_transit_gateway_attachment_request(body)
            {
                Ok(v) => v,
                Err(e) => return json_error_response(400, "ValidationException", &e),
            };
        if input.transit_gateway_attachment_id.is_empty() {
            return json_error_response(
                400,
                "ValidationException",
                "Missing TransitGatewayAttachmentId",
            );
        }
        let mut st = state.write().await;
        match st.reject_transit_gateway_attachment(&input.transit_gateway_attachment_id) {
            Ok(att) => {
                let resp = model::RejectNetworkFirewallTransitGatewayAttachmentResponse {
                    transit_gateway_attachment_id: Some(att.transit_gateway_attachment_id.clone()),
                    transit_gateway_attachment_status: Some(att.status.clone()),
                };
                wire::serialize_reject_network_firewall_transit_gateway_attachment_response(&resp)
            }
            Err(e) => nfw_error_response(&e),
        }
    }

    async fn handle_start_analysis_report(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkFirewallState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_start_analysis_report_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let firewall_arn = match input.firewall_arn.as_deref() {
            Some(a) if !a.is_empty() => a,
            _ => return json_error_response(400, "ValidationException", "Missing FirewallArn"),
        };
        let analysis_type = if input.analysis_type.is_empty() {
            "TLS_NEGOTIATION"
        } else {
            input.analysis_type.as_str()
        };
        let mut st = state.write().await;
        let report = st.start_analysis_report(firewall_arn, analysis_type);
        let resp = model::StartAnalysisReportResponse {
            analysis_report_id: Some(report.analysis_report_id.clone()),
        };
        wire::serialize_start_analysis_report_response(&resp)
    }

    async fn handle_start_flow_capture(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkFirewallState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_start_flow_capture_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.firewall_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing FirewallArn");
        }
        let body_value = input_to_value(&input);
        let firewall_arn = input.firewall_arn.clone();
        let mut st = state.write().await;
        let op = st.create_flow_operation(&firewall_arn, "FLOW_CAPTURE", body_value);
        let resp = model::StartFlowCaptureResponse {
            firewall_arn: Some(op.firewall_arn.clone()),
            flow_operation_id: Some(op.flow_operation_id.clone()),
            flow_operation_status: Some(op.flow_operation_status.clone()),
        };
        wire::serialize_start_flow_capture_response(&resp)
    }

    async fn handle_start_flow_flush(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkFirewallState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_start_flow_flush_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.firewall_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing FirewallArn");
        }
        let body_value = input_to_value(&input);
        let firewall_arn = input.firewall_arn.clone();
        let mut st = state.write().await;
        let op = st.create_flow_operation(&firewall_arn, "FLOW_FLUSH", body_value);
        let resp = model::StartFlowFlushResponse {
            firewall_arn: Some(op.firewall_arn.clone()),
            flow_operation_id: Some(op.flow_operation_id.clone()),
            flow_operation_status: Some(op.flow_operation_status.clone()),
        };
        wire::serialize_start_flow_flush_response(&resp)
    }

    async fn handle_tag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkFirewallState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_tag_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing ResourceArn");
        }
        let tags = wire_tags_to_pairs(Some(input.tags.as_slice()));
        let mut st = state.write().await;
        match st.tag_resource(&input.resource_arn, tags) {
            Ok(()) => {
                let resp = model::TagResourceResponse {};
                wire::serialize_tag_resource_response(&resp)
            }
            Err(e) => nfw_error_response(&e),
        }
    }

    async fn handle_untag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkFirewallState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_untag_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_arn.is_empty() {
            return json_error_response(400, "ValidationException", "Missing ResourceArn");
        }
        let tag_keys: Vec<&str> = input.tag_keys.iter().map(|s| s.as_str()).collect();
        let mut st = state.write().await;
        match st.untag_resource(&input.resource_arn, tag_keys) {
            Ok(()) => {
                let resp = model::UntagResourceResponse {};
                wire::serialize_untag_resource_response(&resp)
            }
            Err(e) => nfw_error_response(&e),
        }
    }

    async fn handle_update_availability_zone_change_protection(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkFirewallState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_availability_zone_change_protection_request(body)
        {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let firewall_name = input.firewall_name.as_deref();
        let firewall_arn = input.firewall_arn.as_deref();
        let protection = input.availability_zone_change_protection;

        let mut st = state.write().await;
        match st.update_availability_zone_change_protection(firewall_name, firewall_arn, protection)
        {
            Ok((fw, _status)) => {
                let resp = model::UpdateAvailabilityZoneChangeProtectionResponse {
                    firewall_arn: Some(fw.firewall_arn.clone()),
                    firewall_name: Some(fw.firewall_name.clone()),
                    availability_zone_change_protection: Some(
                        fw.availability_zone_change_protection,
                    ),
                    update_token: Some("token".to_string()),
                };
                wire::serialize_update_availability_zone_change_protection_response(&resp)
            }
            Err(e) => nfw_error_response(&e),
        }
    }

    async fn handle_update_firewall_analysis_settings(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkFirewallState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_firewall_analysis_settings_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let firewall_name = input.firewall_name.as_deref();
        let firewall_arn = input.firewall_arn.as_deref();
        let enabled_types: Vec<String> = input.enabled_analysis_types.unwrap_or_default();
        let mut st = state.write().await;
        match st.update_analysis_settings(firewall_name, firewall_arn, enabled_types) {
            Ok((fw, types)) => {
                let resp = model::UpdateFirewallAnalysisSettingsResponse {
                    firewall_arn: Some(fw.firewall_arn.clone()),
                    firewall_name: Some(fw.firewall_name.clone()),
                    enabled_analysis_types: Some(types),
                    update_token: Some("token".to_string()),
                };
                wire::serialize_update_firewall_analysis_settings_response(&resp)
            }
            Err(e) => nfw_error_response(&e),
        }
    }

    async fn handle_update_firewall_delete_protection(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkFirewallState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_firewall_delete_protection_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let firewall_name = input.firewall_name.as_deref();
        let firewall_arn = input.firewall_arn.as_deref();
        let delete_protection = input.delete_protection;
        let mut st = state.write().await;
        match st.update_firewall_delete_protection(firewall_name, firewall_arn, delete_protection) {
            Ok((fw, _status)) => {
                let resp = model::UpdateFirewallDeleteProtectionResponse {
                    firewall_arn: Some(fw.firewall_arn.clone()),
                    firewall_name: Some(fw.firewall_name.clone()),
                    delete_protection: Some(fw.delete_protection),
                    update_token: Some("token".to_string()),
                };
                wire::serialize_update_firewall_delete_protection_response(&resp)
            }
            Err(e) => nfw_error_response(&e),
        }
    }

    async fn handle_update_firewall_description(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkFirewallState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_firewall_description_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let firewall_name = input.firewall_name.as_deref();
        let firewall_arn = input.firewall_arn.as_deref();
        let description = input.description.as_deref();
        let mut st = state.write().await;
        match st.update_firewall_description(firewall_name, firewall_arn, description) {
            Ok((fw, _status)) => {
                let resp = model::UpdateFirewallDescriptionResponse {
                    firewall_arn: Some(fw.firewall_arn.clone()),
                    firewall_name: Some(fw.firewall_name.clone()),
                    description: fw.description.clone(),
                    update_token: Some("token".to_string()),
                };
                wire::serialize_update_firewall_description_response(&resp)
            }
            Err(e) => nfw_error_response(&e),
        }
    }

    async fn handle_update_firewall_encryption_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkFirewallState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_firewall_encryption_configuration_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let firewall_name = input.firewall_name.as_deref();
        let firewall_arn = input.firewall_arn.as_deref();
        let config = input
            .encryption_configuration
            .as_ref()
            .map(|ec| crate::types::EncryptionConfig {
                key_id: ec.key_id.clone(),
                config_type: if ec.r#type.is_empty() {
                    "AWS_OWNED_KMS_KEY".to_string()
                } else {
                    ec.r#type.clone()
                },
            })
            .unwrap_or_default();
        let mut st = state.write().await;
        match st.update_encryption_configuration(firewall_name, firewall_arn, config) {
            Ok((fw, enc)) => {
                let resp = model::UpdateFirewallEncryptionConfigurationResponse {
                    firewall_arn: Some(fw.firewall_arn.clone()),
                    firewall_name: Some(fw.firewall_name.clone()),
                    encryption_configuration: Some(model::EncryptionConfiguration {
                        key_id: enc.key_id,
                        r#type: enc.config_type,
                    }),
                    update_token: Some("token".to_string()),
                };
                wire::serialize_update_firewall_encryption_configuration_response(&resp)
            }
            Err(e) => nfw_error_response(&e),
        }
    }

    async fn handle_update_firewall_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkFirewallState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_firewall_policy_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let name = input.firewall_policy_name.as_deref();
        let arn = input.firewall_policy_arn.as_deref();
        let policy_body = serde_json::to_value(&input.firewall_policy)
            .unwrap_or(Value::Object(Default::default()));
        let description = input.description.as_deref();
        let mut st = state.write().await;
        match st.update_firewall_policy(name, arn, policy_body, description) {
            Ok(fp) => {
                let resp = model::UpdateFirewallPolicyResponse {
                    firewall_policy_response: Some(firewall_policy_to_model_response(fp)),
                    update_token: Some("token".to_string()),
                };
                wire::serialize_update_firewall_policy_response(&resp)
            }
            Err(e) => nfw_error_response(&e),
        }
    }

    async fn handle_update_firewall_policy_change_protection(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkFirewallState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_firewall_policy_change_protection_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let firewall_name = input.firewall_name.as_deref();
        let firewall_arn = input.firewall_arn.as_deref();
        let protection = input.firewall_policy_change_protection;

        let mut st = state.write().await;
        match st.update_firewall_policy_change_protection(firewall_name, firewall_arn, protection) {
            Ok((fw, _status)) => {
                let resp = model::UpdateFirewallPolicyChangeProtectionResponse {
                    firewall_arn: Some(fw.firewall_arn.clone()),
                    firewall_name: Some(fw.firewall_name.clone()),
                    firewall_policy_change_protection: Some(fw.firewall_policy_change_protection),
                    update_token: Some("token".to_string()),
                };
                wire::serialize_update_firewall_policy_change_protection_response(&resp)
            }
            Err(e) => nfw_error_response(&e),
        }
    }

    async fn handle_update_proxy(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkFirewallState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_proxy_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let proxy_name = input.proxy_name.as_deref();
        let proxy_arn = input.proxy_arn.as_deref();
        let body_value = input_to_value(&input);
        let mut st = state.write().await;
        match st.update_proxy(proxy_name, proxy_arn, body_value) {
            Ok(proxy) => {
                let resp = model::UpdateProxyResponse {
                    proxy: Some(proxy_to_wire(proxy)),
                    update_token: Some("token".to_string()),
                };
                wire::serialize_update_proxy_response(&resp)
            }
            Err(e) => nfw_error_response(&e),
        }
    }

    async fn handle_update_proxy_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkFirewallState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_proxy_configuration_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let name = input.proxy_configuration_name.as_deref();
        let arn = input.proxy_configuration_arn.as_deref();
        let body_value = input_to_value(&input);
        let mut st = state.write().await;
        match st.update_proxy_configuration(name, arn, body_value) {
            Ok(config) => {
                let resp = model::UpdateProxyConfigurationResponse {
                    proxy_configuration: Some(proxy_configuration_to_wire(config)),
                    update_token: Some("token".to_string()),
                };
                wire::serialize_update_proxy_configuration_response(&resp)
            }
            Err(e) => nfw_error_response(&e),
        }
    }

    async fn handle_update_proxy_rule(
        &self,
        _state: &Arc<tokio::sync::RwLock<NetworkFirewallState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_proxy_rule_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        // Proxy rules live inside proxy rule groups; echo the requested rule back.
        let proxy_rule: Option<model::ProxyRule> = if input.proxy_rule_name.is_empty() {
            None
        } else {
            Some(model::ProxyRule {
                proxy_rule_name: Some(input.proxy_rule_name.clone()),
                action: input.action.clone(),
                description: input.description.clone(),
                ..Default::default()
            })
        };
        let resp = model::UpdateProxyRuleResponse {
            proxy_rule,
            removed_conditions: None,
            update_token: Some("token".to_string()),
        };
        wire::serialize_update_proxy_rule_response(&resp)
    }

    async fn handle_update_proxy_rule_group_priorities(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkFirewallState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_proxy_rule_group_priorities_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        // Proxy rule group priorities are stored as part of the proxy configuration.
        // We echo back the requested priorities.
        let config_name = input.proxy_configuration_name.as_deref();
        let config_arn = input.proxy_configuration_arn.as_deref();
        let body_value = input_to_value(&input);
        let mut st = state.write().await;
        match st.update_proxy_configuration(config_name, config_arn, body_value) {
            Ok(_config) => {
                let priorities: Vec<model::ProxyRuleGroupPriorityResult> = input
                    .rule_groups
                    .iter()
                    .map(|item| model::ProxyRuleGroupPriorityResult {
                        proxy_rule_group_name: item.proxy_rule_group_name.clone(),
                        priority: item.new_position,
                    })
                    .collect();
                let resp = model::UpdateProxyRuleGroupPrioritiesResponse {
                    proxy_rule_groups: if priorities.is_empty() {
                        None
                    } else {
                        Some(priorities)
                    },
                    update_token: Some("token".to_string()),
                };
                wire::serialize_update_proxy_rule_group_priorities_response(&resp)
            }
            Err(e) => nfw_error_response(&e),
        }
    }

    async fn handle_update_proxy_rule_priorities(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkFirewallState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_proxy_rule_priorities_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let group_name = input.proxy_rule_group_name.as_deref();
        let group_arn = input.proxy_rule_group_arn.as_deref();
        let st = state.read().await;
        match st.describe_proxy_rule_group(group_name, group_arn) {
            Ok(group) => {
                let rules: Vec<model::ProxyRulePriority> = input
                    .rules
                    .iter()
                    .map(|item| model::ProxyRulePriority {
                        proxy_rule_name: item.proxy_rule_name.clone(),
                        new_position: item.new_position,
                    })
                    .collect();
                let resp = model::UpdateProxyRulePrioritiesResponse {
                    proxy_rule_group_arn: Some(group.proxy_rule_group_arn.clone()),
                    proxy_rule_group_name: Some(group.proxy_rule_group_name.clone()),
                    rule_group_request_phase: if input.rule_group_request_phase.is_empty() {
                        None
                    } else {
                        Some(input.rule_group_request_phase.clone())
                    },
                    rules: if rules.is_empty() { None } else { Some(rules) },
                    update_token: Some("token".to_string()),
                };
                wire::serialize_update_proxy_rule_priorities_response(&resp)
            }
            Err(e) => nfw_error_response(&e),
        }
    }

    async fn handle_update_rule_group(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkFirewallState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_rule_group_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let name = input.rule_group_name.as_deref();
        let arn = input.rule_group_arn.as_deref();
        let description = input.description.as_deref();
        let rule_group_body = input
            .rule_group
            .as_ref()
            .and_then(|v| serde_json::to_value(v).ok());
        let rules = input.rules.as_deref();
        let mut st = state.write().await;
        match st.update_rule_group(name, arn, description, rule_group_body, rules) {
            Ok(rg) => {
                let resp = model::UpdateRuleGroupResponse {
                    rule_group_response: Some(rule_group_to_model_response(rg)),
                    update_token: Some("token".to_string()),
                };
                wire::serialize_update_rule_group_response(&resp)
            }
            Err(e) => nfw_error_response(&e),
        }
    }

    async fn handle_update_subnet_change_protection(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkFirewallState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_subnet_change_protection_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let firewall_name = input.firewall_name.as_deref();
        let firewall_arn = input.firewall_arn.as_deref();
        let protection = input.subnet_change_protection;

        let mut st = state.write().await;
        match st.update_subnet_change_protection(firewall_name, firewall_arn, protection) {
            Ok((fw, _status)) => {
                let resp = model::UpdateSubnetChangeProtectionResponse {
                    firewall_arn: Some(fw.firewall_arn.clone()),
                    firewall_name: Some(fw.firewall_name.clone()),
                    subnet_change_protection: Some(fw.subnet_change_protection),
                    update_token: Some("token".to_string()),
                };
                wire::serialize_update_subnet_change_protection_response(&resp)
            }
            Err(e) => nfw_error_response(&e),
        }
    }

    async fn handle_update_t_l_s_inspection_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkFirewallState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_t_l_s_inspection_configuration_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let name = input.t_l_s_inspection_configuration_name.as_deref();
        let arn = input.t_l_s_inspection_configuration_arn.as_deref();
        let tls_body = serde_json::to_value(&input.t_l_s_inspection_configuration)
            .unwrap_or(Value::Object(Default::default()));
        let description = input.description.as_deref();

        let mut st = state.write().await;
        match st.update_tls_inspection_configuration(name, arn, tls_body, description) {
            Ok(tls) => {
                let resp = model::UpdateTLSInspectionConfigurationResponse {
                    t_l_s_inspection_configuration_response: Some(tls_to_response(tls)),
                    update_token: Some("token".to_string()),
                };
                wire::serialize_update_t_l_s_inspection_configuration_response(&resp)
            }
            Err(e) => nfw_error_response(&e),
        }
    }

    async fn handle_update_logging_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<NetworkFirewallState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_update_logging_configuration_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let firewall_arn = input.firewall_arn.as_deref();
        let firewall_name = input.firewall_name.as_deref();

        let mut state = state.write().await;

        // Resolve ARN
        let arn = if let Some(a) = firewall_arn {
            a.to_string()
        } else if let Some(name) = firewall_name {
            match state.firewalls.values().find(|f| f.firewall_name == name) {
                Some(f) => f.firewall_arn.clone(),
                None => {
                    return nfw_error_response(&NfwError::ResourceNotFound {
                        identifier: name.to_string(),
                    });
                }
            }
        } else {
            return json_error_response(
                400,
                "InvalidRequestException",
                "Either FirewallArn or FirewallName must be specified",
            );
        };

        // Verify firewall exists
        if !state.firewalls.contains_key(&arn) {
            return nfw_error_response(&NfwError::ResourceNotFound {
                identifier: arn.clone(),
            });
        }

        let logging_config_value = input
            .logging_configuration
            .as_ref()
            .and_then(|v| serde_json::to_value(v).ok())
            .unwrap_or_else(|| json!({ "LogDestinationConfigs": [] }));

        state
            .logging_configs
            .insert(arn.clone(), logging_config_value.clone());

        let firewall_name = state
            .firewalls
            .get(&arn)
            .map(|f| f.firewall_name.clone())
            .unwrap_or_default();

        let resp = model::UpdateLoggingConfigurationResponse {
            firewall_arn: Some(arn),
            firewall_name: Some(firewall_name),
            logging_configuration: Some(value_to_logging_configuration(&logging_config_value)),
            enable_monitoring_dashboard: None,
        };
        wire::serialize_update_logging_configuration_response(&resp)
    }
}

// --- Conversion helpers ---

/// Convert any serde-able input shape into a `serde_json::Value`. Falls back to
/// an empty JSON object on serialisation failure so that downstream `serde_json::Value`-
/// shaped state storage receives a well-formed value.
fn input_to_value<T: serde::Serialize>(input: &T) -> Value {
    serde_json::to_value(input).unwrap_or_else(|_| Value::Object(Default::default()))
}

/// Convert wire `Tag` slices into `(key, value)` pairs for state storage.
fn wire_tags_to_pairs(tags: Option<&[model::Tag]>) -> Vec<(String, String)> {
    tags.map(|arr| {
        arr.iter()
            .map(|t| (t.key.clone(), t.value.clone()))
            .collect()
    })
    .unwrap_or_default()
}

/// Convert a state Firewall to a wire model Firewall.
fn firewall_to_wire(fw: &crate::types::Firewall) -> model::Firewall {
    let subnet_mappings = fw
        .subnet_mappings
        .iter()
        .map(|sm| model::SubnetMapping {
            subnet_id: sm.subnet_id.clone(),
            i_p_address_type: None,
        })
        .collect();

    let tags: Vec<model::Tag> = fw
        .tags
        .iter()
        .map(|(k, v)| model::Tag {
            key: k.clone(),
            value: v.clone(),
        })
        .collect();

    model::Firewall {
        firewall_name: Some(fw.firewall_name.clone()),
        firewall_arn: Some(fw.firewall_arn.clone()),
        subnet_mappings: Some(subnet_mappings),
        delete_protection: Some(fw.delete_protection),
        subnet_change_protection: Some(fw.subnet_change_protection),
        firewall_policy_change_protection: Some(fw.firewall_policy_change_protection),
        description: fw.description.clone(),
        tags: Some(tags),
        firewall_id: Some(fw.firewall_id.clone()),
        firewall_policy_arn: Some(fw.firewall_policy_arn.clone()),
        vpc_id: Some(fw.vpc_id.clone()),
        ..Default::default()
    }
}

/// Convert a state FirewallStatus to a wire model FirewallStatus.
fn firewall_status_to_wire(status: &crate::types::FirewallStatus) -> model::FirewallStatus {
    model::FirewallStatus {
        status: Some(status.status.clone()),
        configuration_sync_state_summary: Some(status.configuration_sync_state_summary.clone()),
        ..Default::default()
    }
}

/// Convert a serde_json::Value logging configuration to the wire model type.
fn value_to_logging_configuration(value: &Value) -> model::LoggingConfiguration {
    let configs = value
        .get("LogDestinationConfigs")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|item| {
                    let log_type = item.get("LogType")?.as_str()?.to_string();
                    let log_destination_type =
                        item.get("LogDestinationType")?.as_str()?.to_string();
                    let log_destination = item
                        .get("LogDestination")
                        .and_then(|v| v.as_object())
                        .map(|obj| {
                            obj.iter()
                                .filter_map(|(k, v)| Some((k.clone(), v.as_str()?.to_string())))
                                .collect()
                        })
                        .unwrap_or_default();
                    Some(model::LogDestinationConfig {
                        log_type,
                        log_destination_type,
                        log_destination,
                    })
                })
                .collect()
        })
        .unwrap_or_default();

    model::LoggingConfiguration {
        log_destination_configs: configs,
    }
}

fn nfw_error_response(err: &NfwError) -> MockResponse {
    let (status, error_type) = match err {
        NfwError::InvalidRequestDuplicateName { .. } => (400u16, "InvalidRequestException"),
        NfwError::InvalidRequestFirewallIdentifier => (400u16, "InvalidRequestException"),
        NfwError::InvalidRequestRuleGroupIdentifier => (400u16, "InvalidRequestException"),
        NfwError::InvalidRequestFirewallPolicyIdentifier => (400u16, "InvalidRequestException"),
        NfwError::InvalidRequestTlsInspectionConfigIdentifier => {
            (400u16, "InvalidRequestException")
        }
        NfwError::InvalidRequestProxyIdentifier => (400u16, "InvalidRequestException"),
        NfwError::InvalidRequestProxyConfigIdentifier => (400u16, "InvalidRequestException"),
        NfwError::InvalidRequestProxyRuleGroupIdentifier => (400u16, "InvalidRequestException"),
        NfwError::ResourceNotFound { .. } => (404u16, "ResourceNotFoundException"),
    };
    let body = json!({
        "__type": error_type,
        "message": err.to_string(),
    });
    MockResponse::json(status, body.to_string())
}

fn json_error_response(status: u16, code: &str, message: &str) -> MockResponse {
    let body = json!({
        "__type": code,
        "message": message,
    });
    MockResponse::json(status, body.to_string())
}

/// Convert a state RuleGroup to a wire RuleGroupResponse.
fn rule_group_to_model_response(rg: &crate::types::RuleGroup) -> model::RuleGroupResponse {
    let tags: Vec<model::Tag> = rg
        .tags
        .iter()
        .map(|(k, v)| model::Tag {
            key: k.clone(),
            value: v.clone(),
        })
        .collect();
    model::RuleGroupResponse {
        rule_group_name: Some(rg.rule_group_name.clone()),
        rule_group_arn: Some(rg.rule_group_arn.clone()),
        rule_group_id: Some(rg.rule_group_id.clone()),
        rule_group_status: Some("ACTIVE".to_string()),
        capacity: Some(rg.capacity),
        description: rg.description.clone(),
        tags: if tags.is_empty() { None } else { Some(tags) },
        ..Default::default()
    }
}

/// Convert a state FirewallPolicy to a wire FirewallPolicyResponse.
fn firewall_policy_to_model_response(
    fp: &crate::types::FirewallPolicy,
) -> model::FirewallPolicyResponse {
    let tags: Vec<model::Tag> = fp
        .tags
        .iter()
        .map(|(k, v)| model::Tag {
            key: k.clone(),
            value: v.clone(),
        })
        .collect();
    model::FirewallPolicyResponse {
        firewall_policy_name: Some(fp.firewall_policy_name.clone()),
        firewall_policy_arn: Some(fp.firewall_policy_arn.clone()),
        firewall_policy_id: Some(fp.firewall_policy_id.clone()),
        firewall_policy_status: Some("ACTIVE".to_string()),
        description: fp.description.clone(),
        tags: if tags.is_empty() { None } else { Some(tags) },
        ..Default::default()
    }
}

/// Convert a state TlsInspectionConfiguration to a wire TLSInspectionConfigurationResponse.
fn tls_to_response(
    tls: &crate::types::TlsInspectionConfiguration,
) -> model::TLSInspectionConfigurationResponse {
    let tags: Vec<model::Tag> = tls
        .tags
        .iter()
        .map(|(k, v)| model::Tag {
            key: k.clone(),
            value: v.clone(),
        })
        .collect();
    model::TLSInspectionConfigurationResponse {
        t_l_s_inspection_configuration_arn: Some(tls.arn.clone()),
        t_l_s_inspection_configuration_id: Some(tls.id.clone()),
        t_l_s_inspection_configuration_name: Some(tls.name.clone()),
        t_l_s_inspection_configuration_status: Some("ACTIVE".to_string()),
        description: tls.description.clone(),
        tags: if tags.is_empty() { None } else { Some(tags) },
        ..Default::default()
    }
}

/// Convert a state NfwProxy to a wire Proxy model.
fn proxy_to_wire(proxy: &crate::types::NfwProxy) -> model::Proxy {
    let tags: Vec<model::Tag> = proxy
        .tags
        .iter()
        .map(|(k, v)| model::Tag {
            key: k.clone(),
            value: v.clone(),
        })
        .collect();
    model::Proxy {
        proxy_name: Some(proxy.proxy_name.clone()),
        proxy_arn: Some(proxy.proxy_arn.clone()),
        nat_gateway_id: Some(proxy.nat_gateway_id.clone()),
        proxy_configuration_arn: proxy.proxy_configuration_arn.clone(),
        proxy_configuration_name: proxy.proxy_configuration_name.clone(),
        proxy_state: Some(proxy.proxy_state.clone()),
        tags: if tags.is_empty() { None } else { Some(tags) },
        ..Default::default()
    }
}

/// Convert a state NfwProxy to a wire DescribeProxyResource model.
fn describe_proxy_resource_to_wire(proxy: &crate::types::NfwProxy) -> model::DescribeProxyResource {
    let tags: Vec<model::Tag> = proxy
        .tags
        .iter()
        .map(|(k, v)| model::Tag {
            key: k.clone(),
            value: v.clone(),
        })
        .collect();
    model::DescribeProxyResource {
        proxy_name: Some(proxy.proxy_name.clone()),
        proxy_arn: Some(proxy.proxy_arn.clone()),
        nat_gateway_id: Some(proxy.nat_gateway_id.clone()),
        proxy_configuration_arn: proxy.proxy_configuration_arn.clone(),
        proxy_configuration_name: proxy.proxy_configuration_name.clone(),
        proxy_state: Some(proxy.proxy_state.clone()),
        tags: if tags.is_empty() { None } else { Some(tags) },
        ..Default::default()
    }
}

/// Convert a state NfwProxyConfiguration to a wire ProxyConfiguration model.
fn proxy_configuration_to_wire(
    config: &crate::types::NfwProxyConfiguration,
) -> model::ProxyConfiguration {
    let tags: Vec<model::Tag> = config
        .tags
        .iter()
        .map(|(k, v)| model::Tag {
            key: k.clone(),
            value: v.clone(),
        })
        .collect();
    model::ProxyConfiguration {
        proxy_configuration_name: Some(config.proxy_configuration_name.clone()),
        proxy_configuration_arn: Some(config.proxy_configuration_arn.clone()),
        description: config.description.clone(),
        tags: if tags.is_empty() { None } else { Some(tags) },
        ..Default::default()
    }
}

/// Convert a state NfwProxyRuleGroup to a wire ProxyRuleGroup model.
fn proxy_rule_group_to_wire(group: &crate::types::NfwProxyRuleGroup) -> model::ProxyRuleGroup {
    let tags: Vec<model::Tag> = group
        .tags
        .iter()
        .map(|(k, v)| model::Tag {
            key: k.clone(),
            value: v.clone(),
        })
        .collect();
    model::ProxyRuleGroup {
        proxy_rule_group_name: Some(group.proxy_rule_group_name.clone()),
        proxy_rule_group_arn: Some(group.proxy_rule_group_arn.clone()),
        description: group.description.clone(),
        tags: if tags.is_empty() { None } else { Some(tags) },
        ..Default::default()
    }
}

/// Convert a state VpcEndpointAssociation to a wire VpcEndpointAssociation model.
fn vpc_endpoint_assoc_to_wire(
    assoc: &crate::types::VpcEndpointAssociation,
) -> model::VpcEndpointAssociation {
    let tags: Vec<model::Tag> = assoc
        .tags
        .iter()
        .map(|(k, v)| model::Tag {
            key: k.clone(),
            value: v.clone(),
        })
        .collect();
    model::VpcEndpointAssociation {
        vpc_endpoint_association_arn: Some(assoc.vpc_endpoint_association_arn.clone()),
        vpc_endpoint_association_id: Some(assoc.vpc_endpoint_association_id.clone()),
        firewall_arn: Some(assoc.firewall_arn.clone()),
        vpc_id: Some(assoc.vpc_id.clone()),
        subnet_mapping: Some(model::SubnetMapping {
            subnet_id: assoc.subnet_id.clone(),
            i_p_address_type: None,
        }),
        description: assoc.description.clone(),
        tags: if tags.is_empty() { None } else { Some(tags) },
    }
}
