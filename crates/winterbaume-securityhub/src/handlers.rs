use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde_json::json;
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, StatefulService,
    default_account_id, extract_path, extract_query_string,
};

use crate::state::{SecurityHubError, SecurityHubState};
use crate::views::SecurityHubStateView;
use crate::wire;

pub struct SecurityHubService {
    pub(crate) state: Arc<BackendState<SecurityHubState>>,
    pub(crate) notifier: StateChangeNotifier<SecurityHubStateView>,
}

impl SecurityHubService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for SecurityHubService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for SecurityHubService {
    fn service_name(&self) -> &str {
        "securityhub"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://securityhub\..*\.amazonaws\.com",
            r"https?://securityhub\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl SecurityHubService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();
        let state = self.state.get(account_id, &region);

        let path = extract_path(&request.uri);
        let method = request.method.as_str();
        let raw_query = extract_query_string(&request.uri);
        let query_map: HashMap<String, String> = winterbaume_core::parse_query_string(raw_query);

        let response = self
            .dispatch_route(
                &state, method, &path, &request, &query_map, account_id, &region,
            )
            .await;

        if response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    #[allow(clippy::too_many_arguments)]
    async fn dispatch_route(
        &self,
        state: &Arc<tokio::sync::RwLock<SecurityHubState>>,
        method: &str,
        path: &str,
        request: &MockRequest,
        query_map: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        // Check for resource-ARN-based paths first: /tags/{ResourceArn}
        if let Some(resource_arn_encoded) = path.strip_prefix("/tags/") {
            let resource_arn = urldecode(resource_arn_encoded);
            let labels: &[(&str, &str)] = &[("ResourceArn", resource_arn.as_str())];
            return match method {
                "GET" => {
                    self.handle_list_tags_for_resource(
                        state, request, labels, query_map, account_id, region,
                    )
                    .await
                }
                "POST" => {
                    self.handle_tag_resource(state, request, labels, query_map, account_id, region)
                        .await
                }
                "DELETE" => {
                    self.handle_untag_resource(
                        state, request, labels, query_map, account_id, region,
                    )
                    .await
                }
                _ => rest_json_error(
                    404,
                    "UnknownOperationException",
                    &format!("Unknown operation: {} {}", method, path),
                ),
            };
        }

        // DELETE /actionTargets/{ActionTargetArn+}
        if method == "DELETE" && path.starts_with("/actionTargets/") {
            let arn_encoded = &path["/actionTargets/".len()..];
            let action_target_arn = urldecode(arn_encoded);
            let labels: &[(&str, &str)] = &[("ActionTargetArn", action_target_arn.as_str())];
            return self
                .handle_delete_action_target(state, request, labels, query_map)
                .await;
        }

        // PATCH /actionTargets/{ActionTargetArn+}
        if method == "PATCH" && path.starts_with("/actionTargets/") {
            let arn_encoded = &path["/actionTargets/".len()..];
            let action_target_arn = urldecode(arn_encoded);
            let labels: &[(&str, &str)] = &[("ActionTargetArn", action_target_arn.as_str())];
            return self
                .handle_update_action_target(state, request, labels, query_map)
                .await;
        }

        // DELETE /findingAggregator/delete/{FindingAggregatorArn+}
        if method == "DELETE" && path.starts_with("/findingAggregator/delete/") {
            let arn_encoded = &path["/findingAggregator/delete/".len()..];
            let arn = urldecode(arn_encoded);
            let labels: &[(&str, &str)] = &[("FindingAggregatorArn", arn.as_str())];
            return self
                .handle_delete_finding_aggregator(state, request, labels, query_map)
                .await;
        }

        // GET /findingAggregator/get/{FindingAggregatorArn+}
        if method == "GET" && path.starts_with("/findingAggregator/get/") {
            let arn_encoded = &path["/findingAggregator/get/".len()..];
            let arn = urldecode(arn_encoded);
            let labels: &[(&str, &str)] = &[("FindingAggregatorArn", arn.as_str())];
            return self
                .handle_get_finding_aggregator(
                    state, request, labels, query_map, account_id, region,
                )
                .await;
        }

        // DELETE /insights/{InsightArn+}
        if method == "DELETE"
            && path.starts_with("/insights/")
            && !path.starts_with("/insights/get")
            && !path.starts_with("/insights/results/")
        {
            let arn_encoded = &path["/insights/".len()..];
            let arn = urldecode(arn_encoded);
            let labels: &[(&str, &str)] = &[("InsightArn", arn.as_str())];
            return self
                .handle_delete_insight(state, request, labels, query_map)
                .await;
        }

        // PATCH /insights/{InsightArn+}
        if method == "PATCH" && path.starts_with("/insights/") {
            let arn_encoded = &path["/insights/".len()..];
            let arn = urldecode(arn_encoded);
            let labels: &[(&str, &str)] = &[("InsightArn", arn.as_str())];
            return self
                .handle_update_insight(state, request, labels, query_map)
                .await;
        }

        // GET /insights/results/{InsightArn+}
        if method == "GET" && path.starts_with("/insights/results/") {
            let arn_encoded = &path["/insights/results/".len()..];
            let arn = urldecode(arn_encoded);
            let labels: &[(&str, &str)] = &[("InsightArn", arn.as_str())];
            return self
                .handle_get_insight_results(state, request, labels, query_map)
                .await;
        }

        // DELETE /productSubscriptions/{ProductSubscriptionArn+}
        if method == "DELETE" && path.starts_with("/productSubscriptions/") {
            let arn_encoded = &path["/productSubscriptions/".len()..];
            let arn = urldecode(arn_encoded);
            let labels: &[(&str, &str)] = &[("ProductSubscriptionArn", arn.as_str())];
            return self
                .handle_disable_import_findings_for_product(state, request, labels, query_map)
                .await;
        }

        // GET /standards/controls/{StandardsSubscriptionArn+}
        if method == "GET" && path.starts_with("/standards/controls/") {
            let arn_encoded = &path["/standards/controls/".len()..];
            let arn = urldecode(arn_encoded);
            let labels: &[(&str, &str)] = &[("StandardsSubscriptionArn", arn.as_str())];
            return self
                .handle_describe_standards_controls(state, request, labels, query_map)
                .await;
        }

        // PATCH /standards/control/{StandardsControlArn+}
        if method == "PATCH" && path.starts_with("/standards/control/") {
            let arn_encoded = &path["/standards/control/".len()..];
            let arn = urldecode(arn_encoded);
            let labels: &[(&str, &str)] = &[("StandardsControlArn", arn.as_str())];
            return self
                .handle_update_standards_control(state, request, labels, query_map)
                .await;
        }

        // DELETE /aggregatorv2/delete/{AggregatorV2Arn+}
        if method == "DELETE" && path.starts_with("/aggregatorv2/delete/") {
            let arn_encoded = &path["/aggregatorv2/delete/".len()..];
            let arn = urldecode(arn_encoded);
            let labels: &[(&str, &str)] = &[("AggregatorV2Arn", arn.as_str())];
            return self
                .handle_delete_aggregator_v2(state, request, labels, query_map)
                .await;
        }

        // GET /aggregatorv2/get/{AggregatorV2Arn+}
        if method == "GET" && path.starts_with("/aggregatorv2/get/") {
            let arn_encoded = &path["/aggregatorv2/get/".len()..];
            let arn = urldecode(arn_encoded);
            let labels: &[(&str, &str)] = &[("AggregatorV2Arn", arn.as_str())];
            return self
                .handle_get_aggregator_v2(state, request, labels, query_map)
                .await;
        }

        // PATCH /aggregatorv2/update/{AggregatorV2Arn+}
        if method == "PATCH" && path.starts_with("/aggregatorv2/update/") {
            let arn_encoded = &path["/aggregatorv2/update/".len()..];
            let arn = urldecode(arn_encoded);
            let labels: &[(&str, &str)] = &[("AggregatorV2Arn", arn.as_str())];
            return self
                .handle_update_aggregator_v2(state, request, labels, query_map)
                .await;
        }

        // DELETE /automationrulesv2/{Identifier}
        if method == "DELETE" && path.starts_with("/automationrulesv2/") {
            let id = &path["/automationrulesv2/".len()..];
            // Only match if no sub-path
            if !id.contains('/') && id != "list" && id != "create" {
                let labels: &[(&str, &str)] = &[("Identifier", id)];
                return self
                    .handle_delete_automation_rule_v2(state, request, labels, query_map)
                    .await;
            }
        }

        // GET /automationrulesv2/{Identifier}
        if method == "GET" && path.starts_with("/automationrulesv2/") {
            let id = &path["/automationrulesv2/".len()..];
            if !id.contains('/') && id != "list" {
                let labels: &[(&str, &str)] = &[("Identifier", id)];
                return self
                    .handle_get_automation_rule_v2(state, request, labels, query_map)
                    .await;
            }
        }

        // PATCH /automationrulesv2/{Identifier}
        if method == "PATCH" && path.starts_with("/automationrulesv2/") {
            let id = &path["/automationrulesv2/".len()..];
            if !id.contains('/') {
                let labels: &[(&str, &str)] = &[("Identifier", id)];
                return self
                    .handle_update_automation_rule_v2(state, request, labels, query_map)
                    .await;
            }
        }

        // DELETE /configurationPolicy/{Identifier}
        if method == "DELETE" && path.starts_with("/configurationPolicy/") {
            let id = &path["/configurationPolicy/".len()..];
            if !id.contains('/') {
                let labels: &[(&str, &str)] = &[("Identifier", id)];
                return self
                    .handle_delete_configuration_policy(state, request, labels, query_map)
                    .await;
            }
        }

        // PATCH /configurationPolicy/{Identifier}
        if method == "PATCH" && path.starts_with("/configurationPolicy/") {
            let id = &path["/configurationPolicy/".len()..];
            if !id.contains('/') {
                let labels: &[(&str, &str)] = &[("Identifier", id)];
                return self
                    .handle_update_configuration_policy(state, request, labels, query_map)
                    .await;
            }
        }

        // GET /configurationPolicy/get/{Identifier}
        if method == "GET" && path.starts_with("/configurationPolicy/get/") {
            let id = &path["/configurationPolicy/get/".len()..];
            let labels: &[(&str, &str)] = &[("Identifier", id)];
            return self
                .handle_get_configuration_policy(state, request, labels, query_map)
                .await;
        }

        // GET /connectorsv2/{ConnectorId+} (but not /connectorsv2 or /connectorsv2/register)
        if method == "GET" && path.starts_with("/connectorsv2/") {
            let id = &path["/connectorsv2/".len()..];
            if !id.is_empty() && id != "register" && !id.contains('/') {
                let labels: &[(&str, &str)] = &[("ConnectorId", id)];
                return self
                    .handle_get_connector_v2(state, request, labels, query_map)
                    .await;
            }
        }

        // PATCH /connectorsv2/{ConnectorId+}
        if method == "PATCH" && path.starts_with("/connectorsv2/") {
            let id = &path["/connectorsv2/".len()..];
            if !id.is_empty() {
                let labels: &[(&str, &str)] = &[("ConnectorId", id)];
                return self
                    .handle_update_connector_v2(state, request, labels, query_map)
                    .await;
            }
        }

        // DELETE /connectorsv2/{ConnectorId+}
        if method == "DELETE" && path.starts_with("/connectorsv2/") {
            let id = &path["/connectorsv2/".len()..];
            if !id.is_empty() {
                let labels: &[(&str, &str)] = &[("ConnectorId", id)];
                return self
                    .handle_delete_connector_v2(state, request, labels, query_map)
                    .await;
            }
        }

        let labels: &[(&str, &str)] = &[];
        match (method, path) {
            // AcceptAdministratorInvitation: POST /administrator
            ("POST", "/administrator") => {
                self.handle_accept_administrator_invitation(state, request, labels, query_map)
                    .await
            }
            // AcceptInvitation: POST /master
            ("POST", "/master") => {
                self.handle_accept_invitation(state, request, labels, query_map)
                    .await
            }
            // BatchDeleteAutomationRules: POST /automationrules/delete
            ("POST", "/automationrules/delete") => {
                self.handle_batch_delete_automation_rules(state, request, labels, query_map)
                    .await
            }
            // BatchDisableStandards: POST /standards/deregister
            ("POST", "/standards/deregister") => {
                self.handle_batch_disable_standards(
                    state, request, labels, query_map, account_id, region,
                )
                .await
            }
            // BatchEnableStandards: POST /standards/register
            ("POST", "/standards/register") => {
                self.handle_batch_enable_standards(
                    state, request, labels, query_map, account_id, region,
                )
                .await
            }
            // BatchGetAutomationRules: POST /automationrules/get
            ("POST", "/automationrules/get") => {
                self.handle_batch_get_automation_rules(state, request, labels, query_map)
                    .await
            }
            // BatchGetConfigurationPolicyAssociations: POST /configurationPolicyAssociation/batchget
            ("POST", "/configurationPolicyAssociation/batchget") => {
                self.handle_batch_get_configuration_policy_associations(
                    state, request, labels, query_map,
                )
                .await
            }
            // BatchGetSecurityControls: POST /securityControls/batchGet
            ("POST", "/securityControls/batchGet") => {
                self.handle_batch_get_security_controls(state, request, labels, query_map)
                    .await
            }
            // BatchGetStandardsControlAssociations: POST /associations/batchGet
            ("POST", "/associations/batchGet") => {
                self.handle_batch_get_standards_control_associations(
                    state, request, labels, query_map,
                )
                .await
            }
            // BatchImportFindings: POST /findings/import
            ("POST", "/findings/import") => {
                self.handle_batch_import_findings(state, request, labels, query_map)
                    .await
            }
            // BatchUpdateAutomationRules: PATCH /automationrules/update
            ("PATCH", "/automationrules/update") => {
                self.handle_batch_update_automation_rules(state, request, labels, query_map)
                    .await
            }
            // BatchUpdateFindings: PATCH /findings/batchupdate
            ("PATCH", "/findings/batchupdate") => {
                self.handle_batch_update_findings(state, request, labels, query_map)
                    .await
            }
            // BatchUpdateFindingsV2: PATCH /findingsv2/batchupdatev2
            ("PATCH", "/findingsv2/batchupdatev2") => {
                self.handle_batch_update_findings_v2(state, request, labels, query_map)
                    .await
            }
            // BatchUpdateStandardsControlAssociations: PATCH /associations
            ("PATCH", "/associations") => {
                self.handle_batch_update_standards_control_associations(
                    state, request, labels, query_map,
                )
                .await
            }
            // CreateActionTarget: POST /actionTargets
            ("POST", "/actionTargets") => {
                self.handle_create_action_target(
                    state, request, labels, query_map, account_id, region,
                )
                .await
            }
            // CreateAggregatorV2: POST /aggregatorv2/create
            ("POST", "/aggregatorv2/create") => {
                self.handle_create_aggregator_v2(
                    state, request, labels, query_map, account_id, region,
                )
                .await
            }
            // CreateAutomationRule: POST /automationrules/create
            ("POST", "/automationrules/create") => {
                self.handle_create_automation_rule(
                    state, request, labels, query_map, account_id, region,
                )
                .await
            }
            // CreateAutomationRuleV2: POST /automationrulesv2/create
            ("POST", "/automationrulesv2/create") => {
                self.handle_create_automation_rule_v2(
                    state, request, labels, query_map, account_id, region,
                )
                .await
            }
            // CreateConfigurationPolicy: POST /configurationPolicy/create
            ("POST", "/configurationPolicy/create") => {
                self.handle_create_configuration_policy(
                    state, request, labels, query_map, account_id, region,
                )
                .await
            }
            // CreateConnectorV2: POST /connectorsv2
            ("POST", "/connectorsv2") => {
                self.handle_create_connector_v2(
                    state, request, labels, query_map, account_id, region,
                )
                .await
            }
            // CreateFindingAggregator: POST /findingAggregator/create
            ("POST", "/findingAggregator/create") => {
                self.handle_create_finding_aggregator(
                    state, request, labels, query_map, account_id, region,
                )
                .await
            }
            // CreateInsight: POST /insights
            ("POST", "/insights") => {
                self.handle_create_insight(state, request, labels, query_map, account_id, region)
                    .await
            }
            // CreateMembers: POST /members
            ("POST", "/members") => {
                self.handle_create_members(state, request, labels, query_map)
                    .await
            }
            // CreateTicketV2: POST /ticketsv2
            ("POST", "/ticketsv2") => {
                self.handle_create_ticket_v2(state, request, labels, query_map)
                    .await
            }
            // DeclineInvitations: POST /invitations/decline
            ("POST", "/invitations/decline") => {
                self.handle_decline_invitations(state, request, labels, query_map)
                    .await
            }
            // DeleteInvitations: POST /invitations/delete
            ("POST", "/invitations/delete") => {
                self.handle_delete_invitations(state, request, labels, query_map)
                    .await
            }
            // DeleteMembers: POST /members/delete
            ("POST", "/members/delete") => {
                self.handle_delete_members(state, request, labels, query_map)
                    .await
            }
            // DescribeActionTargets: POST /actionTargets/get
            ("POST", "/actionTargets/get") => {
                self.handle_describe_action_targets(state, request, labels, query_map)
                    .await
            }
            // DescribeHub: GET /accounts
            ("GET", "/accounts") => {
                self.handle_describe_hub(state, request, labels, query_map, account_id, region)
                    .await
            }
            // DescribeOrganizationConfiguration: GET /organization/configuration
            ("GET", "/organization/configuration") => {
                self.handle_describe_organization_configuration(state, request, labels, query_map)
                    .await
            }
            // DescribeProducts: GET /products
            ("GET", "/products") => {
                self.handle_describe_products(state, request, labels, query_map)
                    .await
            }
            // DescribeProductsV2: GET /productsV2
            ("GET", "/productsV2") => {
                self.handle_describe_products_v2(state, request, labels, query_map)
                    .await
            }
            // DescribeSecurityHubV2: GET /hubv2
            ("GET", "/hubv2") => {
                self.handle_describe_security_hub_v2(state, request, labels, query_map)
                    .await
            }
            // DescribeStandards: GET /standards
            ("GET", "/standards") => {
                self.handle_describe_standards(state, request, labels, query_map)
                    .await
            }
            // DisableOrganizationAdminAccount: POST /organization/admin/disable
            ("POST", "/organization/admin/disable") => {
                self.handle_disable_organization_admin_account(state, request, labels, query_map)
                    .await
            }
            // DisableSecurityHub: DELETE /accounts
            ("DELETE", "/accounts") => {
                self.handle_disable_security_hub(state, request, labels, query_map)
                    .await
            }
            // DisableSecurityHubV2: DELETE /hubv2
            ("DELETE", "/hubv2") => {
                self.handle_disable_security_hub_v2(state, request, labels, query_map)
                    .await
            }
            // DisassociateFromAdministratorAccount: POST /administrator/disassociate
            ("POST", "/administrator/disassociate") => {
                self.handle_disassociate_from_administrator_account(
                    state, request, labels, query_map,
                )
                .await
            }
            // DisassociateFromMasterAccount: POST /master/disassociate
            ("POST", "/master/disassociate") => {
                self.handle_disassociate_from_master_account(state, request, labels, query_map)
                    .await
            }
            // DisassociateMembers: POST /members/disassociate
            ("POST", "/members/disassociate") => {
                self.handle_disassociate_members(state, request, labels, query_map)
                    .await
            }
            // EnableImportFindingsForProduct: POST /productSubscriptions
            ("POST", "/productSubscriptions") => {
                self.handle_enable_import_findings_for_product(
                    state, request, labels, query_map, account_id, region,
                )
                .await
            }
            // EnableOrganizationAdminAccount: POST /organization/admin/enable
            ("POST", "/organization/admin/enable") => {
                self.handle_enable_organization_admin_account(state, request, labels, query_map)
                    .await
            }
            // EnableSecurityHub: POST /accounts
            ("POST", "/accounts") => {
                self.handle_enable_security_hub(state, request, labels, query_map)
                    .await
            }
            // EnableSecurityHubV2: POST /hubv2
            ("POST", "/hubv2") => {
                self.handle_enable_security_hub_v2(
                    state, request, labels, query_map, account_id, region,
                )
                .await
            }
            // GetAdministratorAccount: GET /administrator
            ("GET", "/administrator") => {
                self.handle_get_administrator_account(state, request, labels, query_map)
                    .await
            }
            // GetAutomationRuleV2 list: GET /automationrulesv2/list
            ("GET", "/automationrulesv2/list") => {
                self.handle_list_automation_rules_v2(state, request, labels, query_map)
                    .await
            }
            // GetConfigurationPolicyAssociation: POST /configurationPolicyAssociation/get
            ("POST", "/configurationPolicyAssociation/get") => {
                self.handle_get_configuration_policy_association(state, request, labels, query_map)
                    .await
            }
            // GetEnabledStandards: POST /standards/get
            ("POST", "/standards/get") => {
                self.handle_get_enabled_standards(state, request, labels, query_map)
                    .await
            }
            // GetFindingHistory: POST /findingHistory/get
            ("POST", "/findingHistory/get") => {
                self.handle_get_finding_history(state, request, labels, query_map)
                    .await
            }
            // GetFindingStatisticsV2: POST /findingsv2/statistics
            ("POST", "/findingsv2/statistics") => {
                self.handle_get_finding_statistics_v2(state, request, labels, query_map)
                    .await
            }
            // GetFindings: POST /findings
            ("POST", "/findings") => {
                self.handle_get_findings(state, request, labels, query_map)
                    .await
            }
            // GetFindingsTrendsV2: POST /findingsTrendsv2
            ("POST", "/findingsTrendsv2") => {
                self.handle_get_findings_trends_v2(state, request, labels, query_map)
                    .await
            }
            // GetFindingsV2: POST /findingsv2
            ("POST", "/findingsv2") => {
                self.handle_get_findings_v2(state, request, labels, query_map)
                    .await
            }
            // GetInsights: POST /insights/get
            ("POST", "/insights/get") => {
                self.handle_get_insights(state, request, labels, query_map)
                    .await
            }
            // GetInvitationsCount: GET /invitations/count
            ("GET", "/invitations/count") => {
                self.handle_get_invitations_count(state, request, labels, query_map)
                    .await
            }
            // GetMasterAccount: GET /master
            ("GET", "/master") => {
                self.handle_get_master_account(state, request, labels, query_map)
                    .await
            }
            // GetMembers: POST /members/get
            ("POST", "/members/get") => {
                self.handle_get_members(state, request, labels, query_map)
                    .await
            }
            // GetResourcesStatisticsV2: POST /resourcesv2/statistics
            ("POST", "/resourcesv2/statistics") => {
                self.handle_get_resources_statistics_v2(state, request, labels, query_map)
                    .await
            }
            // GetResourcesTrendsV2: POST /resourcesTrendsv2
            ("POST", "/resourcesTrendsv2") => {
                self.handle_get_resources_trends_v2(state, request, labels, query_map)
                    .await
            }
            // GetResourcesV2: POST /resourcesv2
            ("POST", "/resourcesv2") => {
                self.handle_get_resources_v2(state, request, labels, query_map)
                    .await
            }
            // GetSecurityControlDefinition: GET /securityControl/definition
            ("GET", "/securityControl/definition") => {
                self.handle_get_security_control_definition(state, request, labels, query_map)
                    .await
            }
            // InviteMembers: POST /members/invite
            ("POST", "/members/invite") => {
                self.handle_invite_members(state, request, labels, query_map)
                    .await
            }
            // ListAggregatorsV2: GET /aggregatorv2/list
            ("GET", "/aggregatorv2/list") => {
                self.handle_list_aggregators_v2(state, request, labels, query_map)
                    .await
            }
            // ListAutomationRules: GET /automationrules/list
            ("GET", "/automationrules/list") => {
                self.handle_list_automation_rules(state, request, labels, query_map)
                    .await
            }
            // ListConfigurationPolicies: GET /configurationPolicy/list
            ("GET", "/configurationPolicy/list") => {
                self.handle_list_configuration_policies(state, request, labels, query_map)
                    .await
            }
            // ListConfigurationPolicyAssociations: POST /configurationPolicyAssociation/list
            ("POST", "/configurationPolicyAssociation/list") => {
                self.handle_list_configuration_policy_associations(
                    state, request, labels, query_map,
                )
                .await
            }
            // ListConnectorsV2: GET /connectorsv2
            ("GET", "/connectorsv2") => {
                self.handle_list_connectors_v2(state, request, labels, query_map)
                    .await
            }
            // ListEnabledProductsForImport: GET /productSubscriptions
            ("GET", "/productSubscriptions") => {
                self.handle_list_enabled_products_for_import(state, request, labels, query_map)
                    .await
            }
            // ListFindingAggregators: GET /findingAggregator/list
            ("GET", "/findingAggregator/list") => {
                self.handle_list_finding_aggregators(state, request, labels, query_map)
                    .await
            }
            // ListInvitations: GET /invitations
            ("GET", "/invitations") => {
                self.handle_list_invitations(state, request, labels, query_map)
                    .await
            }
            // ListMembers: GET /members
            ("GET", "/members") => {
                self.handle_list_members(state, request, labels, query_map)
                    .await
            }
            // ListOrganizationAdminAccounts: GET /organization/admin
            ("GET", "/organization/admin") => {
                self.handle_list_organization_admin_accounts(state, request, labels, query_map)
                    .await
            }
            // ListSecurityControlDefinitions: GET /securityControls/definitions
            ("GET", "/securityControls/definitions") => {
                self.handle_list_security_control_definitions(state, request, labels, query_map)
                    .await
            }
            // ListStandardsControlAssociations: GET /associations
            ("GET", "/associations") => {
                self.handle_list_standards_control_associations(state, request, labels, query_map)
                    .await
            }
            // RegisterConnectorV2: POST /connectorsv2/register
            ("POST", "/connectorsv2/register") => {
                self.handle_register_connector_v2(state, request, labels, query_map)
                    .await
            }
            // StartConfigurationPolicyAssociation: POST /configurationPolicyAssociation/associate
            ("POST", "/configurationPolicyAssociation/associate") => {
                self.handle_start_configuration_policy_association(
                    state, request, labels, query_map,
                )
                .await
            }
            // StartConfigurationPolicyDisassociation: POST /configurationPolicyAssociation/disassociate
            ("POST", "/configurationPolicyAssociation/disassociate") => {
                self.handle_start_configuration_policy_disassociation(
                    state, request, labels, query_map,
                )
                .await
            }
            // UpdateFindingAggregator: PATCH /findingAggregator/update
            ("PATCH", "/findingAggregator/update") => {
                self.handle_update_finding_aggregator(state, request, labels, query_map)
                    .await
            }
            // UpdateFindings: PATCH /findings
            ("PATCH", "/findings") => {
                self.handle_update_findings(state, request, labels, query_map)
                    .await
            }
            // UpdateOrganizationConfiguration: POST /organization/configuration
            ("POST", "/organization/configuration") => {
                self.handle_update_organization_configuration(state, request, labels, query_map)
                    .await
            }
            // UpdateSecurityControl: PATCH /securityControl/update
            ("PATCH", "/securityControl/update") => {
                self.handle_update_security_control(state, request, labels, query_map)
                    .await
            }
            // UpdateSecurityHubConfiguration: PATCH /accounts
            ("PATCH", "/accounts") => {
                self.handle_update_security_hub_configuration(state, request, labels, query_map)
                    .await
            }

            _ => rest_json_error(
                404,
                "UnknownOperationException",
                &format!("Unknown operation: {} {}", method, path),
            ),
        }
    }

    // ---- AcceptAdministratorInvitation: POST /administrator ----
    // STUB[org-integration]: multi-account administrator invitation flow not implemented.
    async fn handle_accept_administrator_invitation(
        &self,
        _state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) =
            wire::deserialize_accept_administrator_invitation_request(request, labels, query)
        {
            return rest_json_error(400, "ValidationException", &e);
        }
        wire::serialize_accept_administrator_invitation_response(
            &wire::AcceptAdministratorInvitationResponse {},
        )
    }

    // ---- AcceptInvitation: POST /master ----
    // STUB[org-integration]: legacy master/member invitation flow not implemented.
    async fn handle_accept_invitation(
        &self,
        _state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_accept_invitation_request(request, labels, query) {
            return rest_json_error(400, "ValidationException", &e);
        }
        wire::serialize_accept_invitation_response(&wire::AcceptInvitationResponse {})
    }

    // ---- BatchDeleteAutomationRules: POST /automationrules/delete ----
    async fn handle_batch_delete_automation_rules(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_batch_delete_automation_rules_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let arns = input.automation_rules_arns;
        let mut s = state.write().await;
        let (processed, unprocessed) = s.batch_delete_automation_rules(&arns);
        wire::serialize_batch_delete_automation_rules_response(
            &wire::BatchDeleteAutomationRulesResponse {
                processed_automation_rules: if processed.is_empty() {
                    None
                } else {
                    Some(processed)
                },
                unprocessed_automation_rules: if unprocessed.is_empty() {
                    None
                } else {
                    Some(unprocessed)
                },
            },
        )
    }

    // ---- BatchDisableStandards: POST /standards/deregister ----
    async fn handle_batch_disable_standards(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        _account_id: &str,
        _region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_batch_disable_standards_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let arns = input.standards_subscription_arns;
        let mut s = state.write().await;
        s.enabled_standards
            .retain(|sub| !arns.contains(&sub.standards_subscription_arn));
        let remaining: Vec<wire::StandardsSubscription> = s
            .enabled_standards
            .iter()
            .map(|sub| wire::StandardsSubscription {
                standards_subscription_arn: Some(sub.standards_subscription_arn.clone()),
                standards_arn: Some(sub.standards_arn.clone()),
                standards_status: Some(sub.standards_status.clone()),
                standards_input: Some(sub.standards_input.clone()),
                ..Default::default()
            })
            .collect();
        wire::serialize_batch_disable_standards_response(&wire::BatchDisableStandardsResponse {
            standards_subscriptions: if remaining.is_empty() {
                None
            } else {
                Some(remaining)
            },
        })
    }

    // ---- BatchEnableStandards: POST /standards/register ----
    async fn handle_batch_enable_standards(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        use crate::types::StandardsSubscriptionInfo;
        let input = match wire::deserialize_batch_enable_standards_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut s = state.write().await;
        let mut new_subs = Vec::new();
        for req in &input.standards_subscription_requests {
            let standards_arn = req.standards_arn.clone();
            let sub_arn = format!(
                "arn:aws:securityhub:{region}:{account_id}:subscription/{}",
                uuid::Uuid::new_v4()
            );
            let sub = StandardsSubscriptionInfo {
                standards_subscription_arn: sub_arn.clone(),
                standards_arn: standards_arn.clone(),
                standards_status: "READY".to_string(),
                standards_input: HashMap::new(),
            };
            s.enabled_standards.push(sub);
            new_subs.push(wire::StandardsSubscription {
                standards_subscription_arn: Some(sub_arn),
                standards_arn: Some(standards_arn),
                standards_status: Some("READY".to_string()),
                ..Default::default()
            });
        }
        wire::serialize_batch_enable_standards_response(&wire::BatchEnableStandardsResponse {
            standards_subscriptions: if new_subs.is_empty() {
                None
            } else {
                Some(new_subs)
            },
        })
    }

    // ---- BatchGetAutomationRules: POST /automationrules/get ----
    async fn handle_batch_get_automation_rules(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_batch_get_automation_rules_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let arns = input.automation_rules_arns;
        let s = state.read().await;
        let rules = s.batch_get_automation_rules(&arns);
        wire::serialize_batch_get_automation_rules_response(
            &wire::BatchGetAutomationRulesResponse {
                rules: if rules.is_empty() { None } else { Some(rules) },
                unprocessed_automation_rules: None,
            },
        )
    }

    // ---- BatchGetConfigurationPolicyAssociations: POST /configurationPolicyAssociation/batchget ----
    async fn handle_batch_get_configuration_policy_associations(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_batch_get_configuration_policy_associations_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let identifiers: Vec<(String, String)> = input
            .configuration_policy_association_identifiers
            .iter()
            .filter_map(|item| {
                let target = item.target.as_ref()?;
                if let Some(id) = target.account_id.as_deref() {
                    Some((id.to_string(), "ACCOUNT".to_string()))
                } else if let Some(id) = target.organizational_unit_id.as_deref() {
                    Some((id.to_string(), "ORGANIZATIONAL_UNIT".to_string()))
                } else {
                    target
                        .root_id
                        .as_deref()
                        .map(|id| (id.to_string(), "ROOT".to_string()))
                }
            })
            .collect();
        let s = state.read().await;
        let (found, unprocessed) = s.batch_get_configuration_policy_associations(&identifiers);
        wire::serialize_batch_get_configuration_policy_associations_response(
            &wire::BatchGetConfigurationPolicyAssociationsResponse {
                configuration_policy_associations: if found.is_empty() {
                    None
                } else {
                    Some(found)
                },
                unprocessed_configuration_policy_associations: if unprocessed.is_empty() {
                    None
                } else {
                    Some(unprocessed)
                },
            },
        )
    }

    // ---- BatchGetSecurityControls: POST /securityControls/batchGet ----
    async fn handle_batch_get_security_controls(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_batch_get_security_controls_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let ids = input.security_control_ids;
        let s = state.read().await;
        let (found, unprocessed) = s.batch_get_security_controls(&ids);
        let controls: Vec<wire::SecurityControl> = found
            .into_iter()
            .map(|ctrl| wire::SecurityControl {
                security_control_id: Some(ctrl.security_control_id.clone()),
                security_control_arn: Some(ctrl.security_control_arn.clone()),
                title: Some(ctrl.title.clone()),
                description: Some(ctrl.description.clone()),
                remediation_url: Some(ctrl.remediation_url.clone()),
                severity_rating: Some(ctrl.severity_rating.clone()),
                security_control_status: Some(ctrl.security_control_status.clone()),
                last_update_reason: ctrl.last_update_reason.clone(),
                parameters: None,
                ..Default::default()
            })
            .collect();
        let unprocessed_ids: Vec<wire::UnprocessedSecurityControl> = unprocessed
            .into_iter()
            .map(|(id, code, reason)| wire::UnprocessedSecurityControl {
                security_control_id: Some(id),
                error_code: Some(code),
                error_reason: Some(reason),
            })
            .collect();
        wire::serialize_batch_get_security_controls_response(
            &wire::BatchGetSecurityControlsResponse {
                security_controls: if controls.is_empty() {
                    None
                } else {
                    Some(controls)
                },
                unprocessed_ids: if unprocessed_ids.is_empty() {
                    None
                } else {
                    Some(unprocessed_ids)
                },
            },
        )
    }

    // ---- BatchGetStandardsControlAssociations: POST /associations/batchGet ----
    async fn handle_batch_get_standards_control_associations(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_batch_get_standards_control_associations_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let ids: Vec<(String, String)> = input
            .standards_control_association_ids
            .iter()
            .filter_map(|item| {
                if item.security_control_id.is_empty() || item.standards_arn.is_empty() {
                    None
                } else {
                    Some((item.security_control_id.clone(), item.standards_arn.clone()))
                }
            })
            .collect();
        let s = state.read().await;
        let (found, unprocessed) = s.batch_get_standards_control_associations(&ids);
        let details: Vec<wire::StandardsControlAssociationDetail> = found
            .into_iter()
            .map(|a| wire::StandardsControlAssociationDetail {
                security_control_id: Some(a.security_control_id.clone()),
                standards_arn: Some(a.standards_arn.clone()),
                association_status: Some(a.association_status.clone()),
                security_control_arn: a.security_control_arn.clone(),
                standards_control_title: a.standards_control_title.clone(),
                standards_control_description: a.standards_control_description.clone(),
                standards_control_arns: if a.standards_control_arns.is_empty() {
                    None
                } else {
                    Some(a.standards_control_arns.clone())
                },
                related_requirements: if a.related_requirements.is_empty() {
                    None
                } else {
                    Some(a.related_requirements.clone())
                },
                updated_reason: a.updated_reason.clone(),
                ..Default::default()
            })
            .collect();
        let unprocessed_assocs: Vec<wire::UnprocessedStandardsControlAssociation> = unprocessed
            .into_iter()
            .map(
                |(scid, sarn, code, reason)| wire::UnprocessedStandardsControlAssociation {
                    standards_control_association_id: Some(wire::StandardsControlAssociationId {
                        security_control_id: scid,
                        standards_arn: sarn,
                    }),
                    error_code: Some(code),
                    error_reason: Some(reason),
                },
            )
            .collect();
        wire::serialize_batch_get_standards_control_associations_response(
            &wire::BatchGetStandardsControlAssociationsResponse {
                standards_control_association_details: if details.is_empty() {
                    None
                } else {
                    Some(details)
                },
                unprocessed_associations: if unprocessed_assocs.is_empty() {
                    None
                } else {
                    Some(unprocessed_assocs)
                },
            },
        )
    }

    // ---- BatchImportFindings: POST /findings/import ----
    async fn handle_batch_import_findings(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_batch_import_findings_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        // BatchImportFindings is forwarded to state as JSON; convert typed back via to_value.
        let findings: Vec<serde_json::Value> = input
            .findings
            .iter()
            .map(|f| serde_json::to_value(f).unwrap_or_default())
            .collect();
        let mut s = state.write().await;
        let result = s.batch_import_findings(&findings);
        wire::serialize_batch_import_findings_response(&wire::BatchImportFindingsResponse {
            failed_count: Some(result.failed_count),
            success_count: Some(result.success_count),
            failed_findings: if result.failed_findings.is_empty() {
                None
            } else {
                Some(result.failed_findings)
            },
        })
    }

    // ---- BatchUpdateAutomationRules: PATCH /automationrules/update ----
    async fn handle_batch_update_automation_rules(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_batch_update_automation_rules_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        // The state model handles updates via raw JSON; round-trip through serde_json::to_value.
        let updates: Vec<serde_json::Value> = input
            .update_automation_rules_request_items
            .iter()
            .map(|item| serde_json::to_value(item).unwrap_or_default())
            .collect();
        let mut s = state.write().await;
        let (processed, unprocessed) = s.batch_update_automation_rules(&updates);
        wire::serialize_batch_update_automation_rules_response(
            &wire::BatchUpdateAutomationRulesResponse {
                processed_automation_rules: if processed.is_empty() {
                    None
                } else {
                    Some(processed)
                },
                unprocessed_automation_rules: if unprocessed.is_empty() {
                    None
                } else {
                    Some(unprocessed)
                },
            },
        )
    }

    // ---- BatchUpdateFindings: PATCH /findings/batchupdate ----
    async fn handle_batch_update_findings(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_batch_update_findings_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        // The state still consumes raw JSON values for nested updates; round-trip the typed
        // sub-structures back to Value so identical state semantics are preserved.
        let finding_identifiers: Vec<serde_json::Value> = input
            .finding_identifiers
            .iter()
            .map(|fi| serde_json::to_value(fi).unwrap_or_default())
            .collect();
        let note = input
            .note
            .as_ref()
            .map(|n| serde_json::to_value(n).unwrap_or_default());
        let workflow = input
            .workflow
            .as_ref()
            .map(|w| serde_json::to_value(w).unwrap_or_default());
        let severity = input
            .severity
            .as_ref()
            .map(|s| serde_json::to_value(s).unwrap_or_default());
        let confidence = input.confidence.map(|v| v as i64);
        let criticality = input.criticality.map(|v| v as i64);
        let types: Option<Vec<serde_json::Value>> = input.types.as_ref().map(|t| {
            t.iter()
                .map(|s| serde_json::Value::String(s.clone()))
                .collect()
        });
        let user_defined_fields = input
            .user_defined_fields
            .as_ref()
            .map(|m| serde_json::to_value(m).unwrap_or_default());
        let related_findings: Option<Vec<serde_json::Value>> =
            input.related_findings.as_ref().map(|rfs| {
                rfs.iter()
                    .map(|rf| serde_json::to_value(rf).unwrap_or_default())
                    .collect()
            });
        let mut s = state.write().await;
        let (processed, unprocessed) = s.batch_update_findings(
            &finding_identifiers,
            note.as_ref(),
            workflow.as_ref(),
            severity.as_ref(),
            confidence,
            criticality,
            types.as_deref(),
            user_defined_fields.as_ref(),
            related_findings.as_deref(),
        );
        wire::serialize_batch_update_findings_response(&wire::BatchUpdateFindingsResponse {
            processed_findings: if processed.is_empty() {
                None
            } else {
                Some(processed)
            },
            unprocessed_findings: if unprocessed.is_empty() {
                None
            } else {
                Some(unprocessed)
            },
        })
    }

    // ---- BatchUpdateFindingsV2: PATCH /findingsv2/batchupdatev2 ----
    async fn handle_batch_update_findings_v2(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_batch_update_findings_v2_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let finding_identifiers: Vec<serde_json::Value> = input
            .finding_identifiers
            .as_deref()
            .unwrap_or(&[])
            .iter()
            .map(|fi| serde_json::to_value(fi).unwrap_or_default())
            .collect();
        // The smithy model has no top-level "Updates" struct on this op, but downstream state
        // expects an Updates json envelope. Build it synthetically from the typed fields.
        let mut updates = serde_json::Map::new();
        if let Some(comment) = input.comment.as_ref() {
            updates.insert(
                "Comment".to_string(),
                serde_json::Value::String(comment.clone()),
            );
        }
        if let Some(severity_id) = input.severity_id {
            updates.insert("SeverityId".to_string(), serde_json::json!(severity_id));
        }
        if let Some(status_id) = input.status_id {
            updates.insert("StatusId".to_string(), serde_json::json!(status_id));
        }
        if let Some(metadata_uids) = input.metadata_uids.as_ref() {
            updates.insert(
                "MetadataUids".to_string(),
                serde_json::Value::Array(
                    metadata_uids
                        .iter()
                        .map(|s| serde_json::Value::String(s.clone()))
                        .collect(),
                ),
            );
        }
        let updates = serde_json::Value::Object(updates);
        let mut s = state.write().await;
        let (processed, unprocessed) = s.batch_update_findings_v2(&finding_identifiers, &updates);
        let processed_findings: Vec<wire::BatchUpdateFindingsV2ProcessedFinding> = processed
            .into_iter()
            .map(|id| {
                let metadata_uid = id
                    .get("MetadataUid")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                wire::BatchUpdateFindingsV2ProcessedFinding {
                    finding_identifier: Some(wire::OcsfFindingIdentifier {
                        cloud_account_uid: id
                            .get("FindingIdentifier")
                            .and_then(|fi| fi.get("CloudAccountUid"))
                            .or_else(|| id.get("CloudAccountUid"))
                            .and_then(|v| v.as_str())
                            .unwrap_or("")
                            .to_string(),
                        finding_info_uid: id
                            .get("FindingIdentifier")
                            .and_then(|fi| fi.get("FindingInfoUid"))
                            .or_else(|| id.get("FindingInfoUid"))
                            .and_then(|v| v.as_str())
                            .unwrap_or("")
                            .to_string(),
                        metadata_product_uid: id
                            .get("FindingIdentifier")
                            .and_then(|fi| fi.get("MetadataProductUid"))
                            .or_else(|| id.get("MetadataProductUid"))
                            .and_then(|v| v.as_str())
                            .unwrap_or("")
                            .to_string(),
                    }),
                    metadata_uid: if metadata_uid.is_empty() {
                        None
                    } else {
                        Some(metadata_uid)
                    },
                }
            })
            .collect();
        let unprocessed_findings: Vec<wire::BatchUpdateFindingsV2UnprocessedFinding> = unprocessed
            .into_iter()
            .map(|id| {
                let metadata_uid = id
                    .get("MetadataUid")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                wire::BatchUpdateFindingsV2UnprocessedFinding {
                    finding_identifier: Some(wire::OcsfFindingIdentifier {
                        cloud_account_uid: id
                            .get("CloudAccountUid")
                            .and_then(|v| v.as_str())
                            .unwrap_or("")
                            .to_string(),
                        finding_info_uid: id
                            .get("FindingInfoUid")
                            .and_then(|v| v.as_str())
                            .unwrap_or("")
                            .to_string(),
                        metadata_product_uid: id
                            .get("MetadataProductUid")
                            .and_then(|v| v.as_str())
                            .unwrap_or("")
                            .to_string(),
                    }),
                    metadata_uid: if metadata_uid.is_empty() {
                        None
                    } else {
                        Some(metadata_uid)
                    },
                    error_code: Some("FindingNotFound".to_string()),
                    error_message: Some("Finding not found".to_string()),
                }
            })
            .collect();
        wire::serialize_batch_update_findings_v2_response(&wire::BatchUpdateFindingsV2Response {
            processed_findings: if processed_findings.is_empty() {
                None
            } else {
                Some(processed_findings)
            },
            unprocessed_findings: if unprocessed_findings.is_empty() {
                None
            } else {
                Some(unprocessed_findings)
            },
        })
    }

    // ---- BatchUpdateStandardsControlAssociations: PATCH /associations ----
    async fn handle_batch_update_standards_control_associations(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_batch_update_standards_control_associations_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let updates: Vec<(String, String, String, Option<String>)> = input
            .standards_control_association_updates
            .iter()
            .filter_map(|item| {
                if item.security_control_id.is_empty() || item.standards_arn.is_empty() {
                    return None;
                }
                let status = if item.association_status.is_empty() {
                    "ENABLED".to_string()
                } else {
                    item.association_status.clone()
                };
                Some((
                    item.security_control_id.clone(),
                    item.standards_arn.clone(),
                    status,
                    item.updated_reason.clone(),
                ))
            })
            .collect();
        let mut s = state.write().await;
        let _unprocessed = s.batch_update_standards_control_associations(&updates);
        wire::serialize_batch_update_standards_control_associations_response(
            &wire::BatchUpdateStandardsControlAssociationsResponse {
                unprocessed_association_updates: None,
            },
        )
    }

    // ---- CreateActionTarget: POST /actionTargets ----
    async fn handle_create_action_target(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_action_target_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.name.is_empty() || input.id.is_empty() {
            return rest_json_error(400, "InvalidInputException", "Name and Id are required");
        }
        let mut s = state.write().await;
        match s.create_action_target(
            &input.name,
            &input.description,
            &input.id,
            account_id,
            region,
        ) {
            Ok(result) => {
                wire::serialize_create_action_target_response(&wire::CreateActionTargetResponse {
                    action_target_arn: Some(result.action_target_arn),
                })
            }
            Err(e) => sh_error_response(&e),
        }
    }

    // ---- CreateAggregatorV2: POST /aggregatorv2/create ----
    async fn handle_create_aggregator_v2(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_aggregator_v2_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let region_linking_mode = if input.region_linking_mode.is_empty() {
            "ALL_REGIONS"
        } else {
            input.region_linking_mode.as_str()
        };
        let linked_regions = input.linked_regions.unwrap_or_default();
        let mut s = state.write().await;
        let info = s.create_aggregator_v2(region_linking_mode, linked_regions, account_id, region);
        wire::serialize_create_aggregator_v2_response(&wire::CreateAggregatorV2Response {
            aggregator_v2_arn: Some(info.arn.clone()),
            aggregation_region: Some(info.aggregation_region.clone()),
            linked_regions: if info.linked_regions.is_empty() {
                None
            } else {
                Some(info.linked_regions.clone())
            },
            region_linking_mode: Some(info.region_linking_mode.clone()),
        })
    }

    // ---- CreateAutomationRule: POST /automationrules/create ----
    async fn handle_create_automation_rule(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_automation_rule_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let body_value = serde_json::to_value(&input).unwrap_or_default();
        let rule_arn = format!(
            "arn:aws:securityhub:{region}:{account_id}:automation-rule/{}",
            uuid::Uuid::new_v4()
        );
        let mut s = state.write().await;
        s.create_automation_rule(rule_arn.clone(), None, &body_value);
        wire::serialize_create_automation_rule_response(&wire::CreateAutomationRuleResponse {
            rule_arn: Some(rule_arn),
        })
    }

    // ---- CreateAutomationRuleV2: POST /automationrulesv2/create ----
    async fn handle_create_automation_rule_v2(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input =
            match wire::deserialize_create_automation_rule_v2_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let body_value = serde_json::to_value(&input).unwrap_or_default();
        let mut s = state.write().await;
        let rule = s.create_automation_rule_v2(&body_value, account_id, region);
        let rule_arn = rule.rule_arn.clone();
        let rule_id = rule.rule_id.clone();
        wire::serialize_create_automation_rule_v2_response(&wire::CreateAutomationRuleV2Response {
            rule_arn: Some(rule_arn),
            rule_id,
        })
    }

    // ---- CreateConfigurationPolicy: POST /configurationPolicy/create ----
    async fn handle_create_configuration_policy(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input =
            match wire::deserialize_create_configuration_policy_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let name = if input.name.is_empty() {
            "policy".to_string()
        } else {
            input.name.clone()
        };
        let description = input.description.clone();
        let configuration_policy = serde_json::to_value(&input.configuration_policy).ok();
        let id = uuid::Uuid::new_v4().to_string();
        let arn = format!("arn:aws:securityhub:{region}:{account_id}:configuration-policy/{id}");
        let mut s = state.write().await;
        let policy = s.create_config_policy(id, arn, name, description, configuration_policy);
        wire::serialize_create_configuration_policy_response(
            &wire::CreateConfigurationPolicyResponse {
                arn: Some(policy.arn.clone()),
                id: Some(policy.id.clone()),
                name: Some(policy.name.clone()),
                description: policy.description.clone(),
                updated_at: Some(policy.updated_at.clone()),
                created_at: Some(policy.created_at.clone()),
                configuration_policy: None,
            },
        )
    }

    // ---- CreateConnectorV2: POST /connectorsv2 ----
    async fn handle_create_connector_v2(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_connector_v2_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let body_value = serde_json::to_value(&input).unwrap_or_default();
        let mut s = state.write().await;
        let connector = s.create_connector_v2(&body_value, account_id, region);
        let connector_id = connector.connector_id.clone();
        let connector_arn = connector.connector_arn.clone();
        let connector_status = connector.connector_status.clone();
        wire::serialize_create_connector_v2_response(&wire::CreateConnectorV2Response {
            connector_id: Some(connector_id),
            connector_arn: Some(connector_arn),
            auth_url: None,
            connector_status: Some(connector_status),
        })
    }

    // ---- CreateFindingAggregator: POST /findingAggregator/create ----
    async fn handle_create_finding_aggregator(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input =
            match wire::deserialize_create_finding_aggregator_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let region_linking_mode = if input.region_linking_mode.is_empty() {
            "ALL_REGIONS"
        } else {
            input.region_linking_mode.as_str()
        };
        let regions = input.regions.unwrap_or_default();
        let mut s = state.write().await;
        match s.create_finding_aggregator(region_linking_mode, regions, account_id, region) {
            Ok(result) => wire::serialize_create_finding_aggregator_response(
                &wire::CreateFindingAggregatorResponse {
                    finding_aggregator_arn: Some(result.finding_aggregator_arn),
                    finding_aggregation_region: Some(result.finding_aggregation_region),
                    region_linking_mode: Some(result.region_linking_mode),
                    regions: if result.regions.is_empty() {
                        None
                    } else {
                        Some(result.regions)
                    },
                },
            ),
            Err(e) => sh_error_response(&e),
        }
    }

    // ---- CreateInsight: POST /insights ----
    async fn handle_create_insight(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_insight_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let body_value = serde_json::to_value(&input).unwrap_or_default();
        let mut s = state.write().await;
        let insight_arn = s.create_insight(&body_value, account_id, region);
        wire::serialize_create_insight_response(&wire::CreateInsightResponse {
            insight_arn: Some(insight_arn),
        })
    }

    // ---- CreateMembers: POST /members ----
    async fn handle_create_members(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_members_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let account_details: Vec<serde_json::Value> = input
            .account_details
            .iter()
            .map(|d| serde_json::to_value(d).unwrap_or_default())
            .collect();
        let mut s = state.write().await;
        let result = s.create_members(&account_details);
        wire::serialize_create_members_response(&wire::CreateMembersResponse {
            unprocessed_accounts: if result.unprocessed_accounts.is_empty() {
                None
            } else {
                Some(result.unprocessed_accounts)
            },
        })
    }

    // ---- CreateTicketV2: POST /ticketsv2 ----
    async fn handle_create_ticket_v2(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_ticket_v2_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let body_value = serde_json::to_value(&input).unwrap_or_default();
        let mut s = state.write().await;
        let ticket = s.create_ticket_v2(&body_value);
        wire::serialize_create_ticket_v2_response(&wire::CreateTicketV2Response {
            ticket_id: Some(ticket.ticket_id.clone()),
            ticket_src_url: ticket.ticket_src_url.clone(),
        })
    }

    // ---- DeclineInvitations: POST /invitations/decline ----
    // STUB[org-integration]: multi-account invitation flow not implemented.
    async fn handle_decline_invitations(
        &self,
        _state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_decline_invitations_request(request, labels, query) {
            return rest_json_error(400, "ValidationException", &e);
        }
        wire::serialize_decline_invitations_response(&wire::DeclineInvitationsResponse {
            unprocessed_accounts: None,
        })
    }

    // ---- DeleteActionTarget (inner helper) ----
    async fn handle_delete_action_target(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_action_target_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut s = state.write().await;
        match s.delete_action_target(&input.action_target_arn) {
            Ok(result) => {
                wire::serialize_delete_action_target_response(&wire::DeleteActionTargetResponse {
                    action_target_arn: Some(result.action_target_arn),
                })
            }
            Err(e) => sh_error_response(&e),
        }
    }

    // ---- DeleteAggregatorV2 inner ----
    async fn handle_delete_aggregator_v2(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_aggregator_v2_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let arn = &input.aggregator_v2_arn;
        let mut s = state.write().await;
        if s.delete_aggregator_v2(arn) {
            wire::serialize_delete_aggregator_v2_response(&wire::DeleteAggregatorV2Response {})
        } else {
            rest_json_error(
                404,
                "ResourceNotFoundException",
                &format!("AggregatorV2 with ARN {arn} not found"),
            )
        }
    }

    // ---- DeleteAutomationRuleV2 inner ----
    async fn handle_delete_automation_rule_v2(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_delete_automation_rule_v2_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let id = &input.identifier;
        let mut s = state.write().await;
        if s.delete_automation_rule_v2(id) {
            wire::serialize_delete_automation_rule_v2_response(
                &wire::DeleteAutomationRuleV2Response {},
            )
        } else {
            rest_json_error(
                404,
                "ResourceNotFoundException",
                &format!("Automation rule {id} not found"),
            )
        }
    }

    // ---- DeleteConfigurationPolicy inner ----
    async fn handle_delete_configuration_policy(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_delete_configuration_policy_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let id = &input.identifier;
        let mut s = state.write().await;
        if s.delete_config_policy(id) {
            wire::serialize_delete_configuration_policy_response(
                &wire::DeleteConfigurationPolicyResponse {},
            )
        } else {
            rest_json_error(
                404,
                "ResourceNotFoundException",
                &format!("Configuration policy {id} not found"),
            )
        }
    }

    // ---- DeleteConnectorV2 inner ----
    async fn handle_delete_connector_v2(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_connector_v2_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let id = &input.connector_id;
        let mut s = state.write().await;
        if s.delete_connector_v2(id) {
            wire::serialize_delete_connector_v2_response(&wire::DeleteConnectorV2Response {})
        } else {
            rest_json_error(
                404,
                "ResourceNotFoundException",
                &format!("Connector {id} not found"),
            )
        }
    }

    // ---- DeleteFindingAggregator inner ----
    async fn handle_delete_finding_aggregator(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_delete_finding_aggregator_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let arn = &input.finding_aggregator_arn;
        let mut s = state.write().await;
        s.finding_aggregators.remove(arn);
        wire::serialize_delete_finding_aggregator_response(
            &wire::DeleteFindingAggregatorResponse {},
        )
    }

    // ---- DeleteInsight inner ----
    async fn handle_delete_insight(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_insight_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let arn = &input.insight_arn;
        let mut s = state.write().await;
        s.delete_insight(arn);
        wire::serialize_delete_insight_response(&wire::DeleteInsightResponse {
            insight_arn: Some(arn.clone()),
        })
    }

    // ---- DeleteInvitations: POST /invitations/delete ----
    // STUB[org-integration]: multi-account invitation flow not implemented.
    async fn handle_delete_invitations(
        &self,
        _state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_delete_invitations_request(request, labels, query) {
            return rest_json_error(400, "ValidationException", &e);
        }
        wire::serialize_delete_invitations_response(&wire::DeleteInvitationsResponse {
            unprocessed_accounts: None,
        })
    }

    // ---- DeleteMembers: POST /members/delete ----
    async fn handle_delete_members(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_members_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut s = state.write().await;
        for id in &input.account_ids {
            s.members.remove(id);
        }
        wire::serialize_delete_members_response(&wire::DeleteMembersResponse {
            unprocessed_accounts: None,
        })
    }

    // ---- DescribeActionTargets: POST /actionTargets/get ----
    async fn handle_describe_action_targets(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_action_targets_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let action_target_arns = input.action_target_arns;
        let max_results = input.max_results.map(|v| v as i64);
        let next_token = input.next_token.as_deref();
        let s = state.read().await;
        match s.describe_action_targets(action_target_arns.as_deref(), max_results, next_token) {
            Ok(result) => wire::serialize_describe_action_targets_response(
                &wire::DescribeActionTargetsResponse {
                    action_targets: Some(result.action_targets),
                    next_token: result.next_token,
                },
            ),
            Err(e) => sh_error_response(&e),
        }
    }

    // ---- DescribeHub: GET /accounts ----
    async fn handle_describe_hub(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_hub_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let s = state.read().await;
        match s.describe_hub(input.hub_arn.as_deref(), account_id, region) {
            Ok(result) => wire::serialize_describe_hub_response(&wire::DescribeHubResponse {
                hub_arn: Some(result.hub_arn),
                subscribed_at: result.subscribed_at,
                auto_enable_controls: Some(result.auto_enable_controls),
                control_finding_generator: Some(result.control_finding_generator),
            }),
            Err(e) => sh_error_response(&e),
        }
    }

    // ---- DescribeOrganizationConfiguration: GET /organization/configuration ----
    async fn handle_describe_organization_configuration(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) =
            wire::deserialize_describe_organization_configuration_request(request, labels, query)
        {
            return rest_json_error(400, "ValidationException", &e);
        }
        let s = state.read().await;
        let cfg = s.describe_organization_configuration();
        wire::serialize_describe_organization_configuration_response(
            &wire::DescribeOrganizationConfigurationResponse {
                auto_enable: Some(cfg.auto_enable),
                member_account_limit_reached: Some(cfg.member_account_limit_reached),
                auto_enable_standards: if cfg.auto_enable_standards.is_empty() {
                    None
                } else {
                    Some(cfg.auto_enable_standards.clone())
                },
                ..Default::default()
            },
        )
    }

    // ---- DescribeProducts: GET /products ----
    async fn handle_describe_products(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_describe_products_request(request, labels, query) {
            return rest_json_error(400, "ValidationException", &e);
        }
        let s = state.read().await;
        let products: Vec<wire::Product> = s
            .describe_products()
            .into_iter()
            .map(|p| wire::Product {
                product_arn: Some(p.product_arn.clone()),
                product_name: Some(p.product_name.clone()),
                company_name: Some(p.company_name.clone()),
                description: Some(p.description.clone()),
                categories: if p.categories.is_empty() {
                    None
                } else {
                    Some(p.categories.clone())
                },
                integration_types: if p.integration_types.is_empty() {
                    None
                } else {
                    Some(p.integration_types.clone())
                },
                marketplace_url: p.marketplace_url.clone(),
                activation_url: p.activation_url.clone(),
                product_subscription_resource_policy: p
                    .product_subscription_resource_policy
                    .clone(),
            })
            .collect();
        wire::serialize_describe_products_response(&wire::DescribeProductsResponse {
            products: Some(products),
            next_token: None,
        })
    }

    // ---- DescribeProductsV2: GET /productsV2 ----
    async fn handle_describe_products_v2(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_describe_products_v2_request(request, labels, query) {
            return rest_json_error(400, "ValidationException", &e);
        }
        let s = state.read().await;
        let products: Vec<wire::ProductV2> = s
            .describe_products()
            .into_iter()
            .map(|p| wire::ProductV2 {
                product_v2_name: Some(p.product_name.clone()),
                company_name: Some(p.company_name.clone()),
                description: Some(p.description.clone()),
                categories: if p.categories.is_empty() {
                    None
                } else {
                    Some(p.categories.clone())
                },
                activation_url: p.activation_url.clone(),
                marketplace_product_id: None,
                marketplace_url: p.marketplace_url.clone(),
                integration_v2_types: if p.integration_types.is_empty() {
                    None
                } else {
                    Some(p.integration_types.clone())
                },
            })
            .collect();
        wire::serialize_describe_products_v2_response(&wire::DescribeProductsV2Response {
            products_v2: if products.is_empty() {
                None
            } else {
                Some(products)
            },
            next_token: None,
        })
    }

    // ---- DescribeSecurityHubV2: GET /hubv2 ----
    async fn handle_describe_security_hub_v2(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_describe_security_hub_v2_request(request, labels, query) {
            return rest_json_error(400, "ValidationException", &e);
        }
        let s = state.read().await;
        wire::serialize_describe_security_hub_v2_response(&wire::DescribeSecurityHubV2Response {
            hub_v2_arn: s.hub_v2.hub_v2_arn.clone(),
            subscribed_at: s.hub_v2.subscribed_at.clone(),
        })
    }

    // ---- DescribeStandards: GET /standards ----
    async fn handle_describe_standards(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_describe_standards_request(request, labels, query) {
            return rest_json_error(400, "ValidationException", &e);
        }
        let s = state.read().await;
        let standards: Vec<wire::Standard> = s
            .describe_standards()
            .into_iter()
            .map(|std| wire::Standard {
                standards_arn: Some(std.standards_arn.clone()),
                name: Some(std.name.clone()),
                description: Some(std.description.clone()),
                enabled_by_default: Some(std.enabled_by_default),
                standards_managed_by: None,
            })
            .collect();
        wire::serialize_describe_standards_response(&wire::DescribeStandardsResponse {
            standards: if standards.is_empty() {
                None
            } else {
                Some(standards)
            },
            next_token: None,
        })
    }

    // ---- DescribeStandardsControls inner ----
    async fn handle_describe_standards_controls(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_describe_standards_controls_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let arn = &input.standards_subscription_arn;
        let s = state.read().await;
        let controls_info = s.describe_standards_controls(arn);
        let controls: Vec<wire::StandardsControl> = controls_info
            .into_iter()
            .map(|c| wire::StandardsControl {
                standards_control_arn: Some(c.standards_control_arn.clone()),
                control_status: Some(c.control_status.clone()),
                disabled_reason: c.disabled_reason.clone(),
                control_status_updated_at: c.control_status_updated_at.clone(),
                control_id: c.control_id.clone(),
                title: c.title.clone(),
                description: c.description.clone(),
                remediation_url: c.remediation_url.clone(),
                severity_rating: c.severity_rating.clone(),
                related_requirements: if c.related_requirements.is_empty() {
                    None
                } else {
                    Some(c.related_requirements.clone())
                },
            })
            .collect();
        wire::serialize_describe_standards_controls_response(
            &wire::DescribeStandardsControlsResponse {
                controls: if controls.is_empty() {
                    None
                } else {
                    Some(controls)
                },
                next_token: None,
            },
        )
    }

    // ---- DisableImportFindingsForProduct inner ----
    async fn handle_disable_import_findings_for_product(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_disable_import_findings_for_product_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let arn = &input.product_subscription_arn;
        let mut s = state.write().await;
        s.disable_import_findings_for_product(arn);
        wire::serialize_disable_import_findings_for_product_response(
            &wire::DisableImportFindingsForProductResponse {},
        )
    }

    // ---- DisableOrganizationAdminAccount: POST /organization/admin/disable ----
    async fn handle_disable_organization_admin_account(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) =
            wire::deserialize_disable_organization_admin_account_request(request, labels, query)
        {
            return rest_json_error(400, "ValidationException", &e);
        }
        let mut s = state.write().await;
        s.org_admin_account = None;
        wire::serialize_disable_organization_admin_account_response(
            &wire::DisableOrganizationAdminAccountResponse {},
        )
    }

    // ---- DisableSecurityHub: DELETE /accounts ----
    async fn handle_disable_security_hub(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_disable_security_hub_request(request, labels, query) {
            return rest_json_error(400, "ValidationException", &e);
        }
        let mut s = state.write().await;
        match s.disable_security_hub() {
            Ok(()) => {
                wire::serialize_disable_security_hub_response(&wire::DisableSecurityHubResponse {})
            }
            Err(e) => sh_error_response(&e),
        }
    }

    // ---- DisableSecurityHubV2: DELETE /hubv2 ----
    async fn handle_disable_security_hub_v2(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_disable_security_hub_v2_request(request, labels, query) {
            return rest_json_error(400, "ValidationException", &e);
        }
        let mut s = state.write().await;
        s.disable_security_hub_v2();
        wire::serialize_disable_security_hub_v2_response(&wire::DisableSecurityHubV2Response {})
    }

    // ---- DisassociateFromAdministratorAccount: POST /administrator/disassociate ----
    // STUB[org-integration]: multi-account administrator association not implemented.
    async fn handle_disassociate_from_administrator_account(
        &self,
        _state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_disassociate_from_administrator_account_request(
            request, labels, query,
        ) {
            return rest_json_error(400, "ValidationException", &e);
        }
        wire::serialize_disassociate_from_administrator_account_response(
            &wire::DisassociateFromAdministratorAccountResponse {},
        )
    }

    // ---- DisassociateFromMasterAccount: POST /master/disassociate ----
    // STUB[org-integration]: legacy master/member disassociation not implemented.
    async fn handle_disassociate_from_master_account(
        &self,
        _state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) =
            wire::deserialize_disassociate_from_master_account_request(request, labels, query)
        {
            return rest_json_error(400, "ValidationException", &e);
        }
        wire::serialize_disassociate_from_master_account_response(
            &wire::DisassociateFromMasterAccountResponse {},
        )
    }

    // ---- DisassociateMembers: POST /members/disassociate ----
    // STUB[org-integration]: member account disassociation not implemented.
    async fn handle_disassociate_members(
        &self,
        _state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_disassociate_members_request(request, labels, query) {
            return rest_json_error(400, "ValidationException", &e);
        }
        wire::serialize_disassociate_members_response(&wire::DisassociateMembersResponse {})
    }

    // ---- EnableImportFindingsForProduct: POST /productSubscriptions ----
    async fn handle_enable_import_findings_for_product(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_enable_import_findings_for_product_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let product_arn = input.product_arn.as_str();
        let mut s = state.write().await;
        let subscription_arn =
            s.enable_import_findings_for_product(product_arn, account_id, region);
        wire::serialize_enable_import_findings_for_product_response(
            &wire::EnableImportFindingsForProductResponse {
                product_subscription_arn: Some(subscription_arn),
            },
        )
    }

    // ---- EnableOrganizationAdminAccount: POST /organization/admin/enable ----
    async fn handle_enable_organization_admin_account(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_enable_organization_admin_account_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let admin_account_id = input.admin_account_id.as_str();
        let mut s = state.write().await;
        s.enable_organization_admin_account(admin_account_id);
        wire::serialize_enable_organization_admin_account_response(
            &wire::EnableOrganizationAdminAccountResponse {
                admin_account_id: if admin_account_id.is_empty() {
                    None
                } else {
                    Some(admin_account_id.to_string())
                },
                ..Default::default()
            },
        )
    }

    // ---- EnableSecurityHub: POST /accounts ----
    async fn handle_enable_security_hub(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_enable_security_hub_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let enable_default_standards = input.enable_default_standards.unwrap_or(true);
        let tags = input.tags.unwrap_or_default();
        let mut s = state.write().await;
        s.enable_security_hub(enable_default_standards, tags);
        wire::serialize_enable_security_hub_response(&wire::EnableSecurityHubResponse {})
    }

    // ---- EnableSecurityHubV2: POST /hubv2 ----
    async fn handle_enable_security_hub_v2(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_enable_security_hub_v2_request(request, labels, query) {
            return rest_json_error(400, "ValidationException", &e);
        }
        let mut s = state.write().await;
        let arn = s.enable_security_hub_v2(account_id, region);
        wire::serialize_enable_security_hub_v2_response(&wire::EnableSecurityHubV2Response {
            hub_v2_arn: Some(arn),
        })
    }

    // ---- GetAdministratorAccount: GET /administrator ----
    async fn handle_get_administrator_account(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_get_administrator_account_request(request, labels, query)
        {
            return rest_json_error(400, "ValidationException", &e);
        }
        let s = state.read().await;
        let inv = s.get_administrator_account().map(|a| wire::Invitation {
            account_id: Some(a.account_id.clone()),
            ..Default::default()
        });
        wire::serialize_get_administrator_account_response(&wire::GetAdministratorAccountResponse {
            administrator: inv,
        })
    }

    // ---- GetAggregatorV2 inner ----
    async fn handle_get_aggregator_v2(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_aggregator_v2_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let arn = &input.aggregator_v2_arn;
        let s = state.read().await;
        match s.get_aggregator_v2(arn) {
            Some(info) => {
                wire::serialize_get_aggregator_v2_response(&wire::GetAggregatorV2Response {
                    aggregator_v2_arn: Some(info.arn.clone()),
                    aggregation_region: Some(info.aggregation_region.clone()),
                    linked_regions: if info.linked_regions.is_empty() {
                        None
                    } else {
                        Some(info.linked_regions.clone())
                    },
                    region_linking_mode: Some(info.region_linking_mode.clone()),
                })
            }
            None => rest_json_error(
                404,
                "ResourceNotFoundException",
                &format!("AggregatorV2 with ARN {arn} not found"),
            ),
        }
    }

    // ---- GetAutomationRuleV2 inner ----
    async fn handle_get_automation_rule_v2(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_automation_rule_v2_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let id = &input.identifier;
        let s = state.read().await;
        match s.get_automation_rule_v2(id) {
            Some(rule) => wire::serialize_get_automation_rule_v2_response(
                &wire::GetAutomationRuleV2Response {
                    rule_arn: Some(rule.rule_arn.clone()),
                    rule_id: rule.rule_id.clone(),
                    rule_status: Some(rule.rule_status.clone()),
                    rule_order: Some(rule.rule_order as f32),
                    rule_name: Some(rule.rule_name.clone()),
                    description: rule.description.clone(),
                    created_at: Some(rule.created_at.clone()),
                    updated_at: Some(rule.updated_at.clone()),
                    criteria: None,
                    actions: None,
                },
            ),
            None => rest_json_error(
                404,
                "ResourceNotFoundException",
                &format!("Automation rule {id} not found"),
            ),
        }
    }

    // ---- GetConfigurationPolicy inner ----
    async fn handle_get_configuration_policy(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_configuration_policy_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let id = &input.identifier;
        let s = state.read().await;
        match s.get_config_policy(id) {
            Some(policy) => wire::serialize_get_configuration_policy_response(
                &wire::GetConfigurationPolicyResponse {
                    arn: Some(policy.arn.clone()),
                    id: Some(policy.id.clone()),
                    name: Some(policy.name.clone()),
                    description: policy.description.clone(),
                    updated_at: Some(policy.updated_at.clone()),
                    created_at: Some(policy.created_at.clone()),
                    configuration_policy: None,
                },
            ),
            None => rest_json_error(
                404,
                "ResourceNotFoundException",
                &format!("Configuration policy {id} not found"),
            ),
        }
    }

    // ---- GetConfigurationPolicyAssociation: POST /configurationPolicyAssociation/get ----
    async fn handle_get_configuration_policy_association(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_configuration_policy_association_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let t = &input.target;
        let target_id = match t
            .account_id
            .as_deref()
            .or(t.organizational_unit_id.as_deref())
            .or(t.root_id.as_deref())
        {
            Some(id) => id.to_string(),
            None => {
                return rest_json_error(400, "InvalidInputException", "Target is required");
            }
        };
        let s = state.read().await;
        match s.get_configuration_policy_association(&target_id) {
            Some(assoc) => wire::serialize_get_configuration_policy_association_response(
                &wire::GetConfigurationPolicyAssociationResponse {
                    configuration_policy_id: Some(assoc.configuration_policy_id.clone()),
                    association_status: Some(assoc.association_status.clone()),
                    association_status_message: assoc.association_status_message.clone(),
                    association_type: Some(assoc.association_type.clone()),
                    target_id: Some(assoc.target_id.clone()),
                    target_type: Some(assoc.target_type.clone()),
                    updated_at: Some(assoc.updated_at.clone()),
                },
            ),
            None => rest_json_error(
                404,
                "ResourceNotFoundException",
                &format!("Configuration policy association for target {target_id} not found"),
            ),
        }
    }

    // ---- GetConnectorV2 inner ----
    async fn handle_get_connector_v2(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_connector_v2_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let id = &input.connector_id;
        let s = state.read().await;
        match s.get_connector_v2(id) {
            Some(connector) => {
                wire::serialize_get_connector_v2_response(&wire::GetConnectorV2Response {
                    connector_id: Some(connector.connector_id.clone()),
                    connector_arn: Some(connector.connector_arn.clone()),
                    name: Some(connector.name.clone()),
                    description: connector.description.clone(),
                    created_at: Some(connector.created_at.clone()),
                    last_updated_at: Some(connector.last_updated_at.clone()),
                    provider_detail: None,
                    health: None,
                    ..Default::default()
                })
            }
            None => rest_json_error(
                404,
                "ResourceNotFoundException",
                &format!("Connector {id} not found"),
            ),
        }
    }

    // ---- GetEnabledStandards: POST /standards/get ----
    async fn handle_get_enabled_standards(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_enabled_standards_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let max_results = input.max_results.map(|v| v as i64);
        let next_token = input.next_token.as_deref();
        let s = state.read().await;
        let result = s.get_enabled_standards(max_results, next_token);
        wire::serialize_get_enabled_standards_response(&wire::GetEnabledStandardsResponse {
            standards_subscriptions: if result.standards_subscriptions.is_empty() {
                None
            } else {
                Some(result.standards_subscriptions)
            },
            next_token: result.next_token,
        })
    }

    // ---- GetFindingAggregator inner ----
    async fn handle_get_finding_aggregator(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        _account_id: &str,
        _region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_get_finding_aggregator_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let arn = &input.finding_aggregator_arn;
        let s = state.read().await;
        match s.finding_aggregators.get(arn) {
            Some(info) => wire::serialize_get_finding_aggregator_response(
                &wire::GetFindingAggregatorResponse {
                    finding_aggregator_arn: Some(info.arn.clone()),
                    finding_aggregation_region: Some(info.finding_aggregation_region.clone()),
                    region_linking_mode: Some(info.region_linking_mode.clone()),
                    regions: if info.regions.is_empty() {
                        None
                    } else {
                        Some(info.regions.clone())
                    },
                },
            ),
            None => rest_json_error(
                404,
                "ResourceNotFoundException",
                &format!("FindingAggregator with ARN {arn} not found"),
            ),
        }
    }

    // ---- GetFindingHistory: POST /findingHistory/get ----
    async fn handle_get_finding_history(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_get_finding_history_request(request, labels, query) {
            return rest_json_error(400, "ValidationException", &e);
        }
        // Finding history is not tracked in the mock; we acknowledge the state
        // but return empty records (no historical change tracking).
        let _s = state.read().await;
        wire::serialize_get_finding_history_response(&wire::GetFindingHistoryResponse {
            records: Some(Vec::new()),
            next_token: None,
        })
    }

    // ---- GetFindingStatisticsV2: POST /findingsv2/statistics ----
    async fn handle_get_finding_statistics_v2(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_get_finding_statistics_v2_request(request, labels, query)
        {
            return rest_json_error(400, "ValidationException", &e);
        }
        // Return empty statistics derived from current v2 findings state.
        let _s = state.read().await;
        wire::serialize_get_finding_statistics_v2_response(&wire::GetFindingStatisticsV2Response {
            group_by_results: Some(Vec::new()),
        })
    }

    // ---- GetFindings: POST /findings ----
    async fn handle_get_findings(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_findings_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let max_results = input.max_results.map(|v| v as i64);
        let next_token = input.next_token.as_deref();
        let s = state.read().await;
        match s.get_findings(max_results, next_token) {
            Ok(result) => {
                let findings = result
                    .findings
                    .into_iter()
                    .map(|finding| serde_json::from_value(finding).unwrap_or_default())
                    .collect();
                wire::serialize_get_findings_response(&wire::GetFindingsResponse {
                    findings: Some(findings),
                    next_token: result.next_token,
                })
            }
            Err(e) => sh_error_response(&e),
        }
    }

    // ---- GetFindingsTrendsV2: POST /findingsTrendsv2 ----
    async fn handle_get_findings_trends_v2(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_get_findings_trends_v2_request(request, labels, query) {
            return rest_json_error(400, "ValidationException", &e);
        }
        // Return empty trends from current v2 findings state.
        let _s = state.read().await;
        wire::serialize_get_findings_trends_v2_response(&wire::GetFindingsTrendsV2Response {
            trends_metrics: Some(Vec::new()),
            granularity: None,
            next_token: None,
        })
    }

    // ---- GetFindingsV2: POST /findingsv2 ----
    async fn handle_get_findings_v2(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_findings_v2_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let max_results = input.max_results.map(|v| v as i64);
        let next_token = input.next_token.as_deref();
        let s = state.read().await;
        let (findings, new_token) = s.get_findings_v2(max_results, next_token);
        let findings_values: Vec<serde_json::Value> =
            findings.into_iter().map(|f| f.data.clone()).collect();
        wire::serialize_get_findings_v2_response(&wire::GetFindingsV2Response {
            findings: if findings_values.is_empty() {
                None
            } else {
                Some(findings_values)
            },
            next_token: new_token,
        })
    }

    // ---- GetInsightResults inner ----
    async fn handle_get_insight_results(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_insight_results_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let arn = &input.insight_arn;
        let s = state.read().await;
        if s.get_insight(arn).is_none() {
            return rest_json_error(
                404,
                "ResourceNotFoundException",
                &format!("Insight {arn} not found"),
            );
        }
        wire::serialize_get_insight_results_response(&wire::GetInsightResultsResponse {
            insight_results: None,
        })
    }

    // ---- GetInsights: POST /insights/get ----
    async fn handle_get_insights(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_insights_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let insight_arns = input.insight_arns;
        let max_results = input.max_results.unwrap_or(100) as usize;
        let next_token = input
            .next_token
            .as_deref()
            .and_then(|t| t.parse::<usize>().ok())
            .unwrap_or(0);
        let s = state.read().await;
        let all_insights: Vec<wire::Insight> = if let Some(arns) = insight_arns {
            arns.iter()
                .filter_map(|arn| s.get_insight(arn))
                .map(|i| wire::Insight {
                    insight_arn: Some(i.arn.clone()),
                    name: Some(i.name.clone()),
                    group_by_attribute: Some(i.group_by_attribute.clone()),
                    filters: None,
                })
                .collect()
        } else {
            s.list_insights()
                .into_iter()
                .map(|i| wire::Insight {
                    insight_arn: Some(i.arn.clone()),
                    name: Some(i.name.clone()),
                    group_by_attribute: Some(i.group_by_attribute.clone()),
                    filters: None,
                })
                .collect()
        };
        let total = all_insights.len();
        let end = (next_token + max_results).min(total);
        let page = all_insights[next_token..end].to_vec();
        let new_token = if end < total {
            Some(end.to_string())
        } else {
            None
        };
        wire::serialize_get_insights_response(&wire::GetInsightsResponse {
            insights: if page.is_empty() { None } else { Some(page) },
            next_token: new_token,
        })
    }

    // ---- GetInvitationsCount: GET /invitations/count ----
    // STUB[org-integration]: multi-account invitation count not implemented.
    async fn handle_get_invitations_count(
        &self,
        _state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_get_invitations_count_request(request, labels, query) {
            return rest_json_error(400, "ValidationException", &e);
        }
        wire::serialize_get_invitations_count_response(&wire::GetInvitationsCountResponse {
            invitations_count: Some(0),
        })
    }

    // ---- GetMasterAccount: GET /master ----
    async fn handle_get_master_account(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_get_master_account_request(request, labels, query) {
            return rest_json_error(400, "ValidationException", &e);
        }
        let s = state.read().await;
        let inv = s.get_administrator_account().map(|a| wire::Invitation {
            account_id: Some(a.account_id.clone()),
            ..Default::default()
        });
        wire::serialize_get_master_account_response(&wire::GetMasterAccountResponse { master: inv })
    }

    // ---- GetMembers: POST /members/get ----
    async fn handle_get_members(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_members_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let s = state.read().await;
        let result = s.get_members(&input.account_ids);
        wire::serialize_get_members_response(&wire::GetMembersResponse {
            members: if result.members.is_empty() {
                None
            } else {
                Some(result.members)
            },
            unprocessed_accounts: if result.unprocessed_accounts.is_empty() {
                None
            } else {
                Some(result.unprocessed_accounts)
            },
        })
    }

    // ---- GetResourcesStatisticsV2: POST /resourcesv2/statistics ----
    async fn handle_get_resources_statistics_v2(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) =
            wire::deserialize_get_resources_statistics_v2_request(request, labels, query)
        {
            return rest_json_error(400, "ValidationException", &e);
        }
        // Return empty statistics derived from current state.
        let _s = state.read().await;
        wire::serialize_get_resources_statistics_v2_response(
            &wire::GetResourcesStatisticsV2Response {
                group_by_results: Some(Vec::new()),
            },
        )
    }

    // ---- GetResourcesTrendsV2: POST /resourcesTrendsv2 ----
    async fn handle_get_resources_trends_v2(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_get_resources_trends_v2_request(request, labels, query) {
            return rest_json_error(400, "ValidationException", &e);
        }
        // Return empty trends derived from current state.
        let _s = state.read().await;
        wire::serialize_get_resources_trends_v2_response(&wire::GetResourcesTrendsV2Response {
            trends_metrics: Some(Vec::new()),
            granularity: None,
            next_token: None,
        })
    }

    // ---- GetResourcesV2: POST /resourcesv2 ----
    async fn handle_get_resources_v2(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_get_resources_v2_request(request, labels, query) {
            return rest_json_error(400, "ValidationException", &e);
        }
        // Return empty resources derived from current state.
        let _s = state.read().await;
        wire::serialize_get_resources_v2_response(&wire::GetResourcesV2Response {
            resources: Some(Vec::new()),
            next_token: None,
        })
    }

    // ---- GetSecurityControlDefinition: GET /securityControl/definition ----
    async fn handle_get_security_control_definition(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_get_security_control_definition_request(request, labels, query)
            {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let security_control_id = input.security_control_id.as_str();
        let s = state.read().await;
        match s.get_security_control(security_control_id) {
            Some(ctrl) => wire::serialize_get_security_control_definition_response(
                &wire::GetSecurityControlDefinitionResponse {
                    security_control_definition: Some(wire::SecurityControlDefinition {
                        security_control_id: Some(ctrl.security_control_id.clone()),
                        title: Some(ctrl.title.clone()),
                        description: Some(ctrl.description.clone()),
                        remediation_url: Some(ctrl.remediation_url.clone()),
                        severity_rating: Some(ctrl.severity_rating.clone()),
                        current_region_availability: Some(ctrl.current_region_availability.clone()),
                        parameter_definitions: None,
                        customizable_properties: None,
                    }),
                },
            ),
            None => wire::serialize_get_security_control_definition_response(
                &wire::GetSecurityControlDefinitionResponse {
                    security_control_definition: None,
                },
            ),
        }
    }

    // ---- InviteMembers: POST /members/invite ----
    // STUB[org-integration]: multi-account member invitation not implemented.
    async fn handle_invite_members(
        &self,
        _state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_invite_members_request(request, labels, query) {
            return rest_json_error(400, "ValidationException", &e);
        }
        wire::serialize_invite_members_response(&wire::InviteMembersResponse {
            unprocessed_accounts: None,
        })
    }

    // ---- ListAggregatorsV2: GET /aggregatorv2/list ----
    async fn handle_list_aggregators_v2(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_list_aggregators_v2_request(request, labels, query) {
            return rest_json_error(400, "ValidationException", &e);
        }
        let s = state.read().await;
        let aggregators: Vec<wire::AggregatorV2> = s
            .list_aggregators_v2()
            .into_iter()
            .map(|info| wire::AggregatorV2 {
                aggregator_v2_arn: Some(info.arn.clone()),
            })
            .collect();
        wire::serialize_list_aggregators_v2_response(&wire::ListAggregatorsV2Response {
            aggregators_v2: if aggregators.is_empty() {
                None
            } else {
                Some(aggregators)
            },
            next_token: None,
        })
    }

    // ---- ListAutomationRules: GET /automationrules/list ----
    async fn handle_list_automation_rules(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_list_automation_rules_request(request, labels, query) {
            return rest_json_error(400, "ValidationException", &e);
        }
        let s = state.read().await;
        let metadata = s.list_automation_rules();
        wire::serialize_list_automation_rules_response(&wire::ListAutomationRulesResponse {
            automation_rules_metadata: if metadata.is_empty() {
                None
            } else {
                Some(metadata)
            },
            next_token: None,
        })
    }

    // ---- ListAutomationRulesV2: GET /automationrulesv2/list (via inner) ----
    async fn handle_list_automation_rules_v2(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_list_automation_rules_v2_request(request, labels, query) {
            return rest_json_error(400, "ValidationException", &e);
        }
        let s = state.read().await;
        let rules: Vec<wire::AutomationRulesMetadataV2> = s
            .list_automation_rules_v2()
            .into_iter()
            .map(|rule| wire::AutomationRulesMetadataV2 {
                rule_arn: Some(rule.rule_arn.clone()),
                rule_id: rule.rule_id.clone(),
                rule_name: Some(rule.rule_name.clone()),
                rule_order: Some(rule.rule_order as f32),
                rule_status: Some(rule.rule_status.clone()),
                description: rule.description.clone(),
                created_at: Some(rule.created_at.clone()),
                updated_at: Some(rule.updated_at.clone()),
                actions: None,
            })
            .collect();
        wire::serialize_list_automation_rules_v2_response(&wire::ListAutomationRulesV2Response {
            rules: if rules.is_empty() { None } else { Some(rules) },
            next_token: None,
        })
    }

    // ---- ListConfigurationPolicies: GET /configurationPolicy/list ----
    async fn handle_list_configuration_policies(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) =
            wire::deserialize_list_configuration_policies_request(request, labels, query)
        {
            return rest_json_error(400, "ValidationException", &e);
        }
        let s = state.read().await;
        let summaries: Vec<wire::ConfigurationPolicySummary> = s
            .list_config_policies()
            .into_iter()
            .map(|p| wire::ConfigurationPolicySummary {
                arn: Some(p.arn.clone()),
                id: Some(p.id.clone()),
                name: Some(p.name.clone()),
                description: p.description.clone(),
                updated_at: Some(p.updated_at.clone()),
                service_enabled: None,
            })
            .collect();
        wire::serialize_list_configuration_policies_response(
            &wire::ListConfigurationPoliciesResponse {
                configuration_policy_summaries: if summaries.is_empty() {
                    None
                } else {
                    Some(summaries)
                },
                next_token: None,
            },
        )
    }

    // ---- ListConfigurationPolicyAssociations: POST /configurationPolicyAssociation/list ----
    async fn handle_list_configuration_policy_associations(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) =
            wire::deserialize_list_configuration_policy_associations_request(request, labels, query)
        {
            return rest_json_error(400, "ValidationException", &e);
        }
        let s = state.read().await;
        let summaries: Vec<wire::ConfigurationPolicyAssociationSummary> = s
            .list_configuration_policy_associations()
            .into_iter()
            .map(|assoc| wire::ConfigurationPolicyAssociationSummary {
                configuration_policy_id: Some(assoc.configuration_policy_id.clone()),
                target_id: Some(assoc.target_id.clone()),
                target_type: Some(assoc.target_type.clone()),
                association_type: Some(assoc.association_type.clone()),
                association_status: Some(assoc.association_status.clone()),
                association_status_message: assoc.association_status_message.clone(),
                updated_at: Some(assoc.updated_at.clone()),
            })
            .collect();
        wire::serialize_list_configuration_policy_associations_response(
            &wire::ListConfigurationPolicyAssociationsResponse {
                configuration_policy_association_summaries: if summaries.is_empty() {
                    None
                } else {
                    Some(summaries)
                },
                next_token: None,
            },
        )
    }

    // ---- ListConnectorsV2: GET /connectorsv2 ----
    async fn handle_list_connectors_v2(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_list_connectors_v2_request(request, labels, query) {
            return rest_json_error(400, "ValidationException", &e);
        }
        let s = state.read().await;
        let connectors: Vec<wire::ConnectorSummary> = s
            .list_connectors_v2()
            .into_iter()
            .map(|c| wire::ConnectorSummary {
                connector_id: Some(c.connector_id.clone()),
                connector_arn: Some(c.connector_arn.clone()),
                name: Some(c.name.clone()),
                description: c.description.clone(),
                created_at: Some(c.created_at.clone()),
                provider_summary: None,
            })
            .collect();
        wire::serialize_list_connectors_v2_response(&wire::ListConnectorsV2Response {
            connectors: if connectors.is_empty() {
                None
            } else {
                Some(connectors)
            },
            next_token: None,
        })
    }

    // ---- ListEnabledProductsForImport: GET /productSubscriptions ----
    async fn handle_list_enabled_products_for_import(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) =
            wire::deserialize_list_enabled_products_for_import_request(request, labels, query)
        {
            return rest_json_error(400, "ValidationException", &e);
        }
        let s = state.read().await;
        let subscriptions = s.list_enabled_products_for_import();
        wire::serialize_list_enabled_products_for_import_response(
            &wire::ListEnabledProductsForImportResponse {
                product_subscriptions: if subscriptions.is_empty() {
                    None
                } else {
                    Some(subscriptions)
                },
                next_token: None,
            },
        )
    }

    // ---- ListFindingAggregators: GET /findingAggregator/list ----
    async fn handle_list_finding_aggregators(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_list_finding_aggregators_request(request, labels, query) {
            return rest_json_error(400, "ValidationException", &e);
        }
        let s = state.read().await;
        let aggregators: Vec<wire::FindingAggregator> = s
            .finding_aggregators
            .values()
            .map(|info| wire::FindingAggregator {
                finding_aggregator_arn: Some(info.arn.clone()),
            })
            .collect();
        wire::serialize_list_finding_aggregators_response(&wire::ListFindingAggregatorsResponse {
            finding_aggregators: if aggregators.is_empty() {
                None
            } else {
                Some(aggregators)
            },
            next_token: None,
        })
    }

    // ---- ListInvitations: GET /invitations ----
    // STUB[org-integration]: multi-account invitation listing not implemented.
    async fn handle_list_invitations(
        &self,
        _state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_list_invitations_request(request, labels, query) {
            return rest_json_error(400, "ValidationException", &e);
        }
        wire::serialize_list_invitations_response(&wire::ListInvitationsResponse {
            invitations: None,
            next_token: None,
        })
    }

    // ---- ListMembers: GET /members ----
    async fn handle_list_members(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_members_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let only_associated = input.only_associated;
        let max_results = input.max_results.map(|v| v as i64);
        let next_token = input.next_token.as_deref();
        let s = state.read().await;
        let result = s.list_members(only_associated, max_results, next_token);
        wire::serialize_list_members_response(&wire::ListMembersResponse {
            members: if result.members.is_empty() {
                None
            } else {
                Some(result.members)
            },
            next_token: result.next_token,
        })
    }

    // ---- ListOrganizationAdminAccounts: GET /organization/admin ----
    async fn handle_list_organization_admin_accounts(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) =
            wire::deserialize_list_organization_admin_accounts_request(request, labels, query)
        {
            return rest_json_error(400, "ValidationException", &e);
        }
        let s = state.read().await;
        let admins: Vec<wire::AdminAccount> = if let Some(ref admin) = s.org_admin_account {
            vec![wire::AdminAccount {
                account_id: Some(admin.account_id.clone()),
                status: Some(admin.status.clone()),
            }]
        } else {
            Vec::new()
        };
        wire::serialize_list_organization_admin_accounts_response(
            &wire::ListOrganizationAdminAccountsResponse {
                admin_accounts: if admins.is_empty() {
                    None
                } else {
                    Some(admins)
                },
                feature: None,
                next_token: None,
            },
        )
    }

    // ---- ListSecurityControlDefinitions: GET /securityControls/definitions ----
    async fn handle_list_security_control_definitions(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) =
            wire::deserialize_list_security_control_definitions_request(request, labels, query)
        {
            return rest_json_error(400, "ValidationException", &e);
        }
        let s = state.read().await;
        let defs: Vec<wire::SecurityControlDefinition> = s
            .list_security_control_definitions()
            .into_iter()
            .map(|ctrl| wire::SecurityControlDefinition {
                security_control_id: Some(ctrl.security_control_id.clone()),
                title: Some(ctrl.title.clone()),
                description: Some(ctrl.description.clone()),
                remediation_url: Some(ctrl.remediation_url.clone()),
                severity_rating: Some(ctrl.severity_rating.clone()),
                current_region_availability: Some(ctrl.current_region_availability.clone()),
                parameter_definitions: None,
                customizable_properties: None,
            })
            .collect();
        wire::serialize_list_security_control_definitions_response(
            &wire::ListSecurityControlDefinitionsResponse {
                security_control_definitions: if defs.is_empty() { None } else { Some(defs) },
                next_token: None,
            },
        )
    }

    // ---- ListStandardsControlAssociations: GET /associations ----
    async fn handle_list_standards_control_associations(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) =
            wire::deserialize_list_standards_control_associations_request(request, labels, query)
        {
            return rest_json_error(400, "ValidationException", &e);
        }
        let s = state.read().await;
        let summaries: Vec<wire::StandardsControlAssociationSummary> = s
            .list_standards_control_associations()
            .into_iter()
            .map(|a| wire::StandardsControlAssociationSummary {
                security_control_id: Some(a.security_control_id.clone()),
                standards_arn: Some(a.standards_arn.clone()),
                association_status: Some(a.association_status.clone()),
                security_control_arn: a.security_control_arn.clone(),
                standards_control_title: a.standards_control_title.clone(),
                standards_control_description: a.standards_control_description.clone(),
                related_requirements: if a.related_requirements.is_empty() {
                    None
                } else {
                    Some(a.related_requirements.clone())
                },
                ..Default::default()
            })
            .collect();
        wire::serialize_list_standards_control_associations_response(
            &wire::ListStandardsControlAssociationsResponse {
                standards_control_association_summaries: if summaries.is_empty() {
                    None
                } else {
                    Some(summaries)
                },
                next_token: None,
            },
        )
    }

    // ---- ListTagsForResource (inner) ----
    async fn handle_list_tags_for_resource(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_list_tags_for_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let resource_arn = &input.resource_arn;
        let s = state.read().await;
        match s.list_tags_for_resource(resource_arn, account_id, region) {
            Ok(tags) => wire::serialize_list_tags_for_resource_response(
                &wire::ListTagsForResourceResponse {
                    tags: if tags.is_empty() { None } else { Some(tags) },
                },
            ),
            Err(e) => sh_error_response(&e),
        }
    }

    // ---- RegisterConnectorV2: POST /connectorsv2/register ----
    async fn handle_register_connector_v2(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_register_connector_v2_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        // RegisterConnectorV2 activates an already-created connector via auth code/state.
        // In the mock, we treat this as a no-op but return the connector_id from the auth_state
        // field if it corresponds to a known connector, otherwise create a placeholder.
        let connector_id = input.auth_state.as_str();
        let s = state.read().await;
        // If auth_state matches a known connector_id, return that connector's ARN
        let (cid, carn) = if let Some(connector) = s.connectors.get(connector_id) {
            (
                connector.connector_id.clone(),
                connector.connector_arn.clone(),
            )
        } else {
            // auth_state does not match; return a placeholder
            (connector_id.to_string(), String::new())
        };
        wire::serialize_register_connector_v2_response(&wire::RegisterConnectorV2Response {
            connector_id: if cid.is_empty() { None } else { Some(cid) },
            connector_arn: if carn.is_empty() { None } else { Some(carn) },
        })
    }

    // ---- StartConfigurationPolicyAssociation: POST /configurationPolicyAssociation/associate ----
    async fn handle_start_configuration_policy_association(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_start_configuration_policy_association_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let policy_identifier = input.configuration_policy_identifier.as_str();
        let t = &input.target;
        let (target_id, target_type) = if let Some(id) = t.account_id.as_deref() {
            (id.to_string(), "ACCOUNT".to_string())
        } else if let Some(id) = t.organizational_unit_id.as_deref() {
            (id.to_string(), "ORGANIZATIONAL_UNIT".to_string())
        } else if let Some(id) = t.root_id.as_deref() {
            (id.to_string(), "ROOT".to_string())
        } else {
            return rest_json_error(400, "InvalidInputException", "Target is required");
        };
        let mut s = state.write().await;
        let assoc =
            s.start_configuration_policy_association(policy_identifier, &target_id, &target_type);
        wire::serialize_start_configuration_policy_association_response(
            &wire::StartConfigurationPolicyAssociationResponse {
                configuration_policy_id: Some(assoc.configuration_policy_id.clone()),
                target_id: Some(assoc.target_id.clone()),
                target_type: Some(assoc.target_type.clone()),
                association_type: Some(assoc.association_type.clone()),
                updated_at: Some(assoc.updated_at.clone()),
                association_status: Some(assoc.association_status.clone()),
                association_status_message: assoc.association_status_message.clone(),
            },
        )
    }

    // ---- StartConfigurationPolicyDisassociation: POST /configurationPolicyAssociation/disassociate ----
    async fn handle_start_configuration_policy_disassociation(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_start_configuration_policy_disassociation_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let target_id = input.target.as_ref().and_then(|t| {
            t.account_id
                .as_deref()
                .or(t.organizational_unit_id.as_deref())
                .or(t.root_id.as_deref())
        });
        if let Some(id) = target_id {
            let mut s = state.write().await;
            s.start_configuration_policy_disassociation(id);
        }
        wire::serialize_start_configuration_policy_disassociation_response(
            &wire::StartConfigurationPolicyDisassociationResponse {},
        )
    }

    // ---- TagResource (inner) ----
    async fn handle_tag_resource(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_tag_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let resource_arn = &input.resource_arn;
        let tags = input.tags;
        let mut s = state.write().await;
        match s.tag_resource(resource_arn, tags, account_id, region) {
            Ok(()) => wire::serialize_tag_resource_response(&wire::TagResourceResponse {}),
            Err(e) => sh_error_response(&e),
        }
    }

    // ---- UntagResource (inner) ----
    async fn handle_untag_resource(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_untag_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let resource_arn = &input.resource_arn;
        let tag_keys = input.tag_keys;
        let mut s = state.write().await;
        match s.untag_resource(resource_arn, &tag_keys, account_id, region) {
            Ok(()) => wire::serialize_untag_resource_response(&wire::UntagResourceResponse {}),
            Err(e) => sh_error_response(&e),
        }
    }

    // ---- UpdateActionTarget (inner) ----
    async fn handle_update_action_target(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_action_target_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let action_target_arn = &input.action_target_arn;
        let name = input.name.as_deref();
        let description = input.description.as_deref();
        let mut s = state.write().await;
        if let Some(target) = s.action_targets.get_mut(action_target_arn) {
            if let Some(n) = name {
                target.name = n.to_string();
            }
            if let Some(d) = description {
                target.description = d.to_string();
            }
            wire::serialize_update_action_target_response(&wire::UpdateActionTargetResponse {})
        } else {
            rest_json_error(
                404,
                "ResourceNotFoundException",
                &format!("ActionTarget with ARN {action_target_arn} not found"),
            )
        }
    }

    // ---- UpdateAggregatorV2 inner ----
    async fn handle_update_aggregator_v2(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_aggregator_v2_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let arn = &input.aggregator_v2_arn;
        let region_linking_mode = if input.region_linking_mode.is_empty() {
            None
        } else {
            Some(input.region_linking_mode.as_str())
        };
        let linked_regions = input.linked_regions;
        let mut s = state.write().await;
        match s.update_aggregator_v2(arn, region_linking_mode, linked_regions) {
            Some(info) => {
                wire::serialize_update_aggregator_v2_response(&wire::UpdateAggregatorV2Response {
                    aggregator_v2_arn: Some(info.arn.clone()),
                    aggregation_region: Some(info.aggregation_region.clone()),
                    linked_regions: if info.linked_regions.is_empty() {
                        None
                    } else {
                        Some(info.linked_regions.clone())
                    },
                    region_linking_mode: Some(info.region_linking_mode.clone()),
                })
            }
            None => rest_json_error(
                404,
                "ResourceNotFoundException",
                &format!("AggregatorV2 with ARN {arn} not found"),
            ),
        }
    }

    // ---- UpdateAutomationRuleV2 inner ----
    async fn handle_update_automation_rule_v2(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_update_automation_rule_v2_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let id = input.identifier.clone();
        let body_value = serde_json::to_value(&input).unwrap_or_default();
        let mut s = state.write().await;
        if s.update_automation_rule_v2(&id, &body_value) {
            wire::serialize_update_automation_rule_v2_response(
                &wire::UpdateAutomationRuleV2Response {},
            )
        } else {
            rest_json_error(
                404,
                "ResourceNotFoundException",
                &format!("Automation rule {id} not found"),
            )
        }
    }

    // ---- UpdateConfigurationPolicy inner ----
    async fn handle_update_configuration_policy(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_update_configuration_policy_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let id = &input.identifier;
        let name = input.name.clone();
        let description = input.description.clone();
        let configuration_policy = input
            .configuration_policy
            .as_ref()
            .map(|v| serde_json::to_value(v).unwrap_or_default());
        let mut s = state.write().await;
        match s.update_config_policy(id, name, description, configuration_policy) {
            Some(policy) => wire::serialize_update_configuration_policy_response(
                &wire::UpdateConfigurationPolicyResponse {
                    arn: Some(policy.arn.clone()),
                    id: Some(policy.id.clone()),
                    name: Some(policy.name.clone()),
                    description: policy.description.clone(),
                    updated_at: Some(policy.updated_at.clone()),
                    created_at: Some(policy.created_at.clone()),
                    configuration_policy: None,
                },
            ),
            None => rest_json_error(
                404,
                "ResourceNotFoundException",
                &format!("Configuration policy {id} not found"),
            ),
        }
    }

    // ---- UpdateConnectorV2 inner ----
    async fn handle_update_connector_v2(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_connector_v2_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let id = input.connector_id.clone();
        let body_value = serde_json::to_value(&input).unwrap_or_default();
        let mut s = state.write().await;
        if s.update_connector_v2(&id, &body_value) {
            wire::serialize_update_connector_v2_response(&wire::UpdateConnectorV2Response {})
        } else {
            rest_json_error(
                404,
                "ResourceNotFoundException",
                &format!("Connector {id} not found"),
            )
        }
    }

    // ---- UpdateFindingAggregator: PATCH /findingAggregator/update ----
    async fn handle_update_finding_aggregator(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_update_finding_aggregator_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let arn = input.finding_aggregator_arn.clone();
        let region_linking_mode = input.region_linking_mode.clone();
        let regions = input.regions.unwrap_or_default();
        let mut s = state.write().await;
        if let Some(info) = s.finding_aggregators.get_mut(&arn) {
            info.region_linking_mode = region_linking_mode.clone();
            info.regions = regions.clone();
        }
        wire::serialize_update_finding_aggregator_response(&wire::UpdateFindingAggregatorResponse {
            finding_aggregator_arn: Some(arn),
            finding_aggregation_region: None,
            region_linking_mode: Some(region_linking_mode),
            regions: if regions.is_empty() {
                None
            } else {
                Some(regions)
            },
        })
    }

    // ---- UpdateFindings: PATCH /findings ----
    async fn handle_update_findings(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_findings_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let filters = serde_json::to_value(&input.filters).unwrap_or(json!({}));
        let note = input
            .note
            .as_ref()
            .map(|n| serde_json::to_value(n).unwrap_or_default());
        let record_state = input.record_state.as_deref();
        let mut s = state.write().await;
        s.update_findings(&filters, note.as_ref(), record_state);
        wire::serialize_update_findings_response(&wire::UpdateFindingsResponse {})
    }

    // ---- UpdateInsight inner ----
    async fn handle_update_insight(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_insight_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let arn = input.insight_arn.clone();
        let body_value = serde_json::to_value(&input).unwrap_or_default();
        let mut s = state.write().await;
        if s.update_insight(&arn, &body_value) {
            wire::serialize_update_insight_response(&wire::UpdateInsightResponse {})
        } else {
            rest_json_error(
                404,
                "ResourceNotFoundException",
                &format!("Insight {arn} not found"),
            )
        }
    }

    // ---- UpdateOrganizationConfiguration: POST /organization/configuration ----
    async fn handle_update_organization_configuration(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_organization_configuration_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let auto_enable = Some(input.auto_enable);
        let auto_enable_standards = input.auto_enable_standards.as_deref();
        let mut s = state.write().await;
        s.update_organization_configuration(auto_enable, auto_enable_standards);
        wire::serialize_update_organization_configuration_response(
            &wire::UpdateOrganizationConfigurationResponse {},
        )
    }

    // ---- UpdateSecurityControl: PATCH /securityControl/update ----
    async fn handle_update_security_control(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_security_control_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let security_control_id = input.security_control_id.as_str();
        let parameters = if input.parameters.is_empty() {
            None
        } else {
            Some(serde_json::to_value(&input.parameters).unwrap_or_default())
        };
        let last_update_reason = input.last_update_reason.as_deref();
        let mut s = state.write().await;
        s.update_security_control(security_control_id, parameters, last_update_reason);
        wire::serialize_update_security_control_response(&wire::UpdateSecurityControlResponse {})
    }

    // ---- UpdateSecurityHubConfiguration: PATCH /accounts ----
    async fn handle_update_security_hub_configuration(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_security_hub_configuration_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let auto_enable_controls = input.auto_enable_controls;
        let control_finding_generator = input.control_finding_generator.as_deref();
        let mut s = state.write().await;
        s.update_security_hub_configuration(auto_enable_controls, control_finding_generator);
        wire::serialize_update_security_hub_configuration_response(
            &wire::UpdateSecurityHubConfigurationResponse {},
        )
    }

    // ---- UpdateStandardsControl inner ----
    async fn handle_update_standards_control(
        &self,
        state: &tokio::sync::RwLock<SecurityHubState>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_standards_control_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let arn = &input.standards_control_arn;
        let control_status = input.control_status.as_deref();
        let disabled_reason = input.disabled_reason.as_deref();
        let mut s = state.write().await;
        s.update_standards_control(arn, control_status, disabled_reason);
        wire::serialize_update_standards_control_response(&wire::UpdateStandardsControlResponse {})
    }
}

fn sh_error_response(err: &SecurityHubError) -> MockResponse {
    let (status, error_type) = match err {
        SecurityHubError::NotSubscribed => (403u16, "InvalidAccessException"),
        SecurityHubError::HubNotFound { .. } => (404, "ResourceNotFoundException"),
        SecurityHubError::MaxResultsOutOfRange => (400, "InvalidInputException"),
        SecurityHubError::ActionTargetAlreadyExists { .. } => (409, "ResourceConflictException"),
        SecurityHubError::ActionTargetNotFound { .. } => (404, "ResourceNotFoundException"),
        SecurityHubError::ResourceNotFound { .. } => (404, "ResourceNotFoundException"),
    };
    let message = err.to_string();
    let body = json!({
        "__type": error_type,
        "message": message,
        "Message": message,
    });
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers.insert(
        http::header::HeaderName::from_static("x-amzn-errortype"),
        error_type.parse().unwrap(),
    );
    resp
}

fn urldecode(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.bytes();
    while let Some(b) = chars.next() {
        if b == b'%' {
            let hi = chars.next().unwrap_or(b'0');
            let lo = chars.next().unwrap_or(b'0');
            let hex = [hi, lo];
            if let Ok(s) = std::str::from_utf8(&hex)
                && let Ok(val) = u8::from_str_radix(s, 16)
            {
                result.push(val as char);
                continue;
            }
            result.push('%');
            result.push(hi as char);
            result.push(lo as char);
        } else {
            result.push(b as char);
        }
    }
    result
}

fn rest_json_error(status: u16, code: &str, message: &str) -> MockResponse {
    let body = json!({
        "__type": code,
        "message": message,
        "Message": message,
    });
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers.insert(
        http::header::HeaderName::from_static("x-amzn-errortype"),
        code.parse().unwrap(),
    );
    resp
}
