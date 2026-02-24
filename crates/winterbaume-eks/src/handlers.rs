use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::json;
use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
    StatefulService,
};

use crate::state::{EksError, EksState};
use crate::types::{FargateSelector, ScalingConfig};
use crate::views::EksStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct EksService {
    pub(crate) state: Arc<BackendState<EksState>>,
    pub(crate) notifier: StateChangeNotifier<EksStateView>,
}

impl EksService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }

    /// Returns sorted `(account_id, region)` pairs that have state.
    pub fn scopes_with_state(&self) -> Vec<(String, String)> {
        self.state.scopes_with_state()
    }
}

impl Default for EksService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for EksService {
    fn service_name(&self) -> &str {
        "eks"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://eks\..*\.amazonaws\.com",
            r"https?://eks\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl EksService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = DEFAULT_ACCOUNT_ID;
        let state = self.state.get(account_id, &region);

        let method = request.method.as_str();

        let (path_part, query_string) = extract_path_and_query(&request.uri);
        let query_map: HashMap<String, String> =
            winterbaume_core::parse_query_string(&query_string);

        let segments: Vec<String> = path_part
            .trim_start_matches('/')
            .split('/')
            .filter(|s| !s.is_empty())
            .map(percent_decode)
            .collect();
        let segs: Vec<&str> = segments.iter().map(|s| s.as_str()).collect();

        if segs.is_empty() {
            return rest_json_error(404, "UnknownOperationException", "Not found");
        }

        let response = self
            .dispatch_route(
                &state, method, &segs, &request, &query_map, account_id, &region,
            )
            .await;

        if matches!(method, "PUT" | "POST" | "DELETE") && response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    #[allow(clippy::too_many_arguments)]
    async fn dispatch_route(
        &self,
        state: &Arc<tokio::sync::RwLock<EksState>>,
        method: &str,
        segs: &[&str],
        request: &MockRequest,
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        // Handle /tags/{resourceArn+} routes
        if segs[0] == "tags" && segs.len() >= 2 {
            let resource_arn = segs[1..].join("/");
            let labels: &[(&str, &str)] = &[("resourceArn", resource_arn.as_str())];
            return match method {
                "GET" => {
                    self.handle_list_tags_for_resource(state, request, labels, query)
                        .await
                }
                "POST" => {
                    self.handle_tag_resource(state, request, labels, query)
                        .await
                }
                "DELETE" => {
                    self.handle_untag_resource(state, request, labels, query)
                        .await
                }
                _ => rest_json_error(404, "UnknownOperationException", "Not found"),
            };
        }

        // Handle /access-policies route
        if segs[0] == "access-policies" && segs.len() == 1 {
            return match method {
                "GET" => self.handle_list_access_policies(request, query).await,
                _ => rest_json_error(404, "UnknownOperationException", "Not found"),
            };
        }

        // Handle /cluster-versions route
        if segs[0] == "cluster-versions" && segs.len() == 1 {
            return match method {
                "GET" => self.handle_describe_cluster_versions(request, query).await,
                _ => rest_json_error(404, "UnknownOperationException", "Not found"),
            };
        }

        // Handle /addons/supported-versions and /addons/configuration-schemas
        if segs[0] == "addons" && segs.len() == 2 {
            return match (method, segs[1]) {
                ("GET", "supported-versions") => {
                    self.handle_describe_addon_versions(request, query).await
                }
                ("GET", "configuration-schemas") => {
                    self.handle_describe_addon_configuration(request, query)
                        .await
                }
                _ => rest_json_error(404, "UnknownOperationException", "Not found"),
            };
        }

        // Handle /eks-anywhere-subscriptions routes
        if segs[0] == "eks-anywhere-subscriptions" {
            return match (method, segs.len()) {
                ("POST", 1) => {
                    self.handle_create_eks_anywhere_subscription(
                        state,
                        request,
                        &[],
                        query,
                        account_id,
                        region,
                    )
                    .await
                }
                ("GET", 1) => {
                    self.handle_list_eks_anywhere_subscriptions(state, request, &[], query)
                        .await
                }
                ("GET", 2) => {
                    let labels: &[(&str, &str)] = &[("id", segs[1])];
                    self.handle_describe_eks_anywhere_subscription(state, request, labels, query)
                        .await
                }
                ("DELETE", 2) => {
                    let labels: &[(&str, &str)] = &[("id", segs[1])];
                    self.handle_delete_eks_anywhere_subscription(state, request, labels, query)
                        .await
                }
                ("POST", 2) => {
                    let labels: &[(&str, &str)] = &[("id", segs[1])];
                    self.handle_update_eks_anywhere_subscription(state, request, labels, query)
                        .await
                }
                _ => rest_json_error(404, "UnknownOperationException", "Not found"),
            };
        }

        // Handle /cluster-registrations routes (RegisterCluster)
        if segs[0] == "cluster-registrations" {
            return match (method, segs.len()) {
                ("POST", 1) => {
                    self.handle_register_cluster(state, request, &[], query, account_id, region)
                        .await
                }
                ("DELETE", 2) => {
                    let labels: &[(&str, &str)] = &[("name", segs[1])];
                    self.handle_deregister_cluster(state, request, labels, query)
                        .await
                }
                _ => rest_json_error(404, "UnknownOperationException", "Not found"),
            };
        }

        if segs[0] != "clusters" {
            return rest_json_error(404, "UnknownOperationException", "Not found");
        }

        match (method, segs.len()) {
            // POST /clusters - CreateCluster
            ("POST", 1) => {
                self.handle_create_cluster(state, request, &[], query, account_id, region)
                    .await
            }
            // GET /clusters - ListClusters
            ("GET", 1) => self.handle_list_clusters(state, request, &[], query).await,
            // GET /clusters/{name} - DescribeCluster
            ("GET", 2) => {
                let labels: &[(&str, &str)] = &[("name", segs[1])];
                self.handle_describe_cluster(state, request, labels, query)
                    .await
            }
            // DELETE /clusters/{name} - DeleteCluster
            ("DELETE", 2) => {
                let labels: &[(&str, &str)] = &[("name", segs[1])];
                self.handle_delete_cluster(state, request, labels, query)
                    .await
            }
            // POST /clusters/{name}/update-config - UpdateClusterConfig
            ("POST", 3) if segs[2] == "update-config" => {
                let labels: &[(&str, &str)] = &[("name", segs[1])];
                self.handle_update_cluster_config(state, request, labels, query)
                    .await
            }
            // POST /clusters/{name}/node-groups - CreateNodegroup
            ("POST", 3) if segs[2] == "node-groups" => {
                let labels: &[(&str, &str)] = &[("clusterName", segs[1])];
                self.handle_create_nodegroup(state, request, labels, query, account_id, region)
                    .await
            }
            // GET /clusters/{name}/node-groups - ListNodegroups
            ("GET", 3) if segs[2] == "node-groups" => {
                let labels: &[(&str, &str)] = &[("clusterName", segs[1])];
                self.handle_list_nodegroups(state, request, labels, query)
                    .await
            }
            // POST /clusters/{name}/fargate-profiles - CreateFargateProfile
            ("POST", 3) if segs[2] == "fargate-profiles" => {
                let labels: &[(&str, &str)] = &[("clusterName", segs[1])];
                self.handle_create_fargate_profile(
                    state, request, labels, query, account_id, region,
                )
                .await
            }
            // GET /clusters/{name}/fargate-profiles - ListFargateProfiles
            ("GET", 3) if segs[2] == "fargate-profiles" => {
                let labels: &[(&str, &str)] = &[("clusterName", segs[1])];
                self.handle_list_fargate_profiles(state, request, labels, query)
                    .await
            }
            // POST /clusters/{name}/addons - CreateAddon
            ("POST", 3) if segs[2] == "addons" => {
                let labels: &[(&str, &str)] = &[("clusterName", segs[1])];
                self.handle_create_addon(state, request, labels, query, account_id, region)
                    .await
            }
            // GET /clusters/{name}/addons - ListAddons
            ("GET", 3) if segs[2] == "addons" => {
                let labels: &[(&str, &str)] = &[("clusterName", segs[1])];
                self.handle_list_addons(state, request, labels, query).await
            }
            // POST /clusters/{name}/access-entries - CreateAccessEntry
            ("POST", 3) if segs[2] == "access-entries" => {
                let labels: &[(&str, &str)] = &[("clusterName", segs[1])];
                self.handle_create_access_entry(state, request, labels, query, account_id, region)
                    .await
            }
            // GET /clusters/{name}/access-entries - ListAccessEntries
            ("GET", 3) if segs[2] == "access-entries" => {
                let labels: &[(&str, &str)] = &[("clusterName", segs[1])];
                self.handle_list_access_entries(state, request, labels, query)
                    .await
            }
            // POST /clusters/{name}/identity-provider-configs/associate - AssociateIdentityProviderConfig
            ("POST", 4) if segs[2] == "identity-provider-configs" && segs[3] == "associate" => {
                let labels: &[(&str, &str)] = &[("clusterName", segs[1])];
                self.handle_associate_identity_provider_config(
                    state, request, labels, query, account_id, region,
                )
                .await
            }
            // GET /clusters/{name}/identity-provider-configs - ListIdentityProviderConfigs
            ("GET", 3) if segs[2] == "identity-provider-configs" => {
                let labels: &[(&str, &str)] = &[("clusterName", segs[1])];
                self.handle_list_identity_provider_configs(state, request, labels, query)
                    .await
            }
            // POST /clusters/{name}/pod-identity-associations - CreatePodIdentityAssociation
            ("POST", 3) if segs[2] == "pod-identity-associations" => {
                let labels: &[(&str, &str)] = &[("clusterName", segs[1])];
                self.handle_create_pod_identity_association(
                    state, request, labels, query, account_id, region,
                )
                .await
            }
            // GET /clusters/{name}/pod-identity-associations - ListPodIdentityAssociations
            ("GET", 3) if segs[2] == "pod-identity-associations" => {
                let labels: &[(&str, &str)] = &[("clusterName", segs[1])];
                self.handle_list_pod_identity_associations(state, request, labels, query)
                    .await
            }
            // POST /clusters/{name}/updates - UpdateClusterVersion
            ("POST", 3) if segs[2] == "updates" => {
                let labels: &[(&str, &str)] = &[("name", segs[1])];
                self.handle_update_cluster_version(state, request, labels, query)
                    .await
            }
            // GET /clusters/{name}/updates - ListUpdates
            ("GET", 3) if segs[2] == "updates" => {
                let labels: &[(&str, &str)] = &[("name", segs[1])];
                self.handle_list_updates(state, request, labels, query)
                    .await
            }
            // POST /clusters/{name}/capabilities - CreateCapability
            ("POST", 3) if segs[2] == "capabilities" => {
                let labels: &[(&str, &str)] = &[("clusterName", segs[1])];
                self.handle_create_capability(state, request, labels, query, account_id, region)
                    .await
            }
            // GET /clusters/{name}/capabilities - ListCapabilities
            ("GET", 3) if segs[2] == "capabilities" => {
                let labels: &[(&str, &str)] = &[("clusterName", segs[1])];
                self.handle_list_capabilities(state, request, labels, query)
                    .await
            }
            // POST /clusters/{name}/insights - ListInsights (body-based POST pagination)
            ("POST", 3) if segs[2] == "insights" => {
                let labels: &[(&str, &str)] = &[("clusterName", segs[1])];
                self.handle_list_insights(request, labels, query).await
            }
            // POST /clusters/{name}/encryption-config/associate - AssociateEncryptionConfig
            ("POST", 4) if segs[2] == "encryption-config" && segs[3] == "associate" => {
                let labels: &[(&str, &str)] = &[("clusterName", segs[1])];
                self.handle_associate_encryption_config(state, request, labels, query)
                    .await
            }
            // GET /clusters/{name}/node-groups/{nodegroup} - DescribeNodegroup
            ("GET", 4) if segs[2] == "node-groups" => {
                let labels: &[(&str, &str)] =
                    &[("clusterName", segs[1]), ("nodegroupName", segs[3])];
                self.handle_describe_nodegroup(state, request, labels, query)
                    .await
            }
            // DELETE /clusters/{name}/node-groups/{nodegroup} - DeleteNodegroup
            ("DELETE", 4) if segs[2] == "node-groups" => {
                let labels: &[(&str, &str)] =
                    &[("clusterName", segs[1]), ("nodegroupName", segs[3])];
                self.handle_delete_nodegroup(state, request, labels, query)
                    .await
            }
            // GET /clusters/{name}/fargate-profiles/{profile} - DescribeFargateProfile
            ("GET", 4) if segs[2] == "fargate-profiles" => {
                let labels: &[(&str, &str)] =
                    &[("clusterName", segs[1]), ("fargateProfileName", segs[3])];
                self.handle_describe_fargate_profile(state, request, labels, query)
                    .await
            }
            // DELETE /clusters/{name}/fargate-profiles/{profile} - DeleteFargateProfile
            ("DELETE", 4) if segs[2] == "fargate-profiles" => {
                let labels: &[(&str, &str)] =
                    &[("clusterName", segs[1]), ("fargateProfileName", segs[3])];
                self.handle_delete_fargate_profile(state, request, labels, query)
                    .await
            }
            // GET /clusters/{name}/addons/{addonName} - DescribeAddon
            ("GET", 4) if segs[2] == "addons" => {
                let labels: &[(&str, &str)] = &[("clusterName", segs[1]), ("addonName", segs[3])];
                self.handle_describe_addon(state, request, labels, query)
                    .await
            }
            // DELETE /clusters/{name}/addons/{addonName} - DeleteAddon
            ("DELETE", 4) if segs[2] == "addons" => {
                let labels: &[(&str, &str)] = &[("clusterName", segs[1]), ("addonName", segs[3])];
                self.handle_delete_addon(state, request, labels, query)
                    .await
            }
            // POST /clusters/{name}/addons/{addonName}/update - UpdateAddon
            ("POST", 5) if segs[2] == "addons" && segs[4] == "update" => {
                let labels: &[(&str, &str)] = &[("clusterName", segs[1]), ("addonName", segs[3])];
                self.handle_update_addon(state, request, labels, query)
                    .await
            }
            // GET /clusters/{name}/access-entries/{principalArn} - DescribeAccessEntry
            ("GET", 4) if segs[2] == "access-entries" => {
                let labels: &[(&str, &str)] =
                    &[("clusterName", segs[1]), ("principalArn", segs[3])];
                self.handle_describe_access_entry(state, request, labels, query)
                    .await
            }
            // DELETE /clusters/{name}/access-entries/{principalArn} - DeleteAccessEntry
            ("DELETE", 4) if segs[2] == "access-entries" => {
                let labels: &[(&str, &str)] =
                    &[("clusterName", segs[1]), ("principalArn", segs[3])];
                self.handle_delete_access_entry(state, request, labels, query)
                    .await
            }
            // POST /clusters/{name}/access-entries/{principalArn} - UpdateAccessEntry
            ("POST", 4) if segs[2] == "access-entries" => {
                let labels: &[(&str, &str)] =
                    &[("clusterName", segs[1]), ("principalArn", segs[3])];
                self.handle_update_access_entry(state, request, labels, query)
                    .await
            }
            // GET /clusters/{name}/access-entries/{principalArn}/access-policies - ListAssociatedAccessPolicies
            ("GET", 5) if segs[2] == "access-entries" && segs[4] == "access-policies" => {
                let labels: &[(&str, &str)] =
                    &[("clusterName", segs[1]), ("principalArn", segs[3])];
                self.handle_list_associated_access_policies(state, request, labels, query)
                    .await
            }
            // POST /clusters/{name}/access-entries/{principalArn}/access-policies - AssociateAccessPolicy
            ("POST", 5) if segs[2] == "access-entries" && segs[4] == "access-policies" => {
                let labels: &[(&str, &str)] =
                    &[("clusterName", segs[1]), ("principalArn", segs[3])];
                self.handle_associate_access_policy(state, request, labels, query)
                    .await
            }
            // DELETE /clusters/{name}/access-entries/{principalArn}/access-policies/{policyArn} - DisassociateAccessPolicy
            ("DELETE", 6) if segs[2] == "access-entries" && segs[4] == "access-policies" => {
                let labels: &[(&str, &str)] = &[
                    ("clusterName", segs[1]),
                    ("principalArn", segs[3]),
                    ("policyArn", segs[5]),
                ];
                self.handle_disassociate_access_policy(state, request, labels, query)
                    .await
            }
            // POST /clusters/{name}/identity-provider-configs/disassociate - DisassociateIdentityProviderConfig
            ("POST", 4) if segs[2] == "identity-provider-configs" && segs[3] == "disassociate" => {
                let labels: &[(&str, &str)] = &[("clusterName", segs[1])];
                self.handle_disassociate_identity_provider_config(state, request, labels, query)
                    .await
            }
            // POST /clusters/{name}/identity-provider-configs/describe - DescribeIdentityProviderConfig
            ("POST", 4) if segs[2] == "identity-provider-configs" && segs[3] == "describe" => {
                let labels: &[(&str, &str)] = &[("clusterName", segs[1])];
                self.handle_describe_identity_provider_config(state, request, labels, query)
                    .await
            }
            // GET /clusters/{name}/pod-identity-associations/{associationId} - DescribePodIdentityAssociation
            ("GET", 4) if segs[2] == "pod-identity-associations" => {
                let labels: &[(&str, &str)] =
                    &[("clusterName", segs[1]), ("associationId", segs[3])];
                self.handle_describe_pod_identity_association(state, request, labels, query)
                    .await
            }
            // DELETE /clusters/{name}/pod-identity-associations/{associationId} - DeletePodIdentityAssociation
            ("DELETE", 4) if segs[2] == "pod-identity-associations" => {
                let labels: &[(&str, &str)] =
                    &[("clusterName", segs[1]), ("associationId", segs[3])];
                self.handle_delete_pod_identity_association(state, request, labels, query)
                    .await
            }
            // POST /clusters/{name}/pod-identity-associations/{associationId} - UpdatePodIdentityAssociation
            ("POST", 4) if segs[2] == "pod-identity-associations" => {
                let labels: &[(&str, &str)] =
                    &[("clusterName", segs[1]), ("associationId", segs[3])];
                self.handle_update_pod_identity_association(state, request, labels, query)
                    .await
            }
            // GET /clusters/{name}/updates/{updateId} - DescribeUpdate
            ("GET", 4) if segs[2] == "updates" => {
                let labels: &[(&str, &str)] = &[("name", segs[1]), ("updateId", segs[3])];
                self.handle_describe_update(state, request, labels, query)
                    .await
            }
            // POST /clusters/{name}/node-groups/{nodegroup}/update-config - UpdateNodegroupConfig
            ("POST", 5) if segs[2] == "node-groups" && segs[4] == "update-config" => {
                let labels: &[(&str, &str)] =
                    &[("clusterName", segs[1]), ("nodegroupName", segs[3])];
                self.handle_update_nodegroup_config(state, request, labels, query)
                    .await
            }
            // POST /clusters/{name}/node-groups/{nodegroup}/update-version - UpdateNodegroupVersion
            ("POST", 5) if segs[2] == "node-groups" && segs[4] == "update-version" => {
                let labels: &[(&str, &str)] =
                    &[("clusterName", segs[1]), ("nodegroupName", segs[3])];
                self.handle_update_nodegroup_version(state, request, labels, query)
                    .await
            }
            // GET /clusters/{name}/capabilities/{capabilityName} - DescribeCapability
            ("GET", 4) if segs[2] == "capabilities" => {
                let labels: &[(&str, &str)] =
                    &[("clusterName", segs[1]), ("capabilityName", segs[3])];
                self.handle_describe_capability(state, request, labels, query)
                    .await
            }
            // DELETE /clusters/{name}/capabilities/{capabilityName} - DeleteCapability
            ("DELETE", 4) if segs[2] == "capabilities" => {
                let labels: &[(&str, &str)] =
                    &[("clusterName", segs[1]), ("capabilityName", segs[3])];
                self.handle_delete_capability(state, request, labels, query)
                    .await
            }
            // POST /clusters/{name}/capabilities/{capabilityName} - UpdateCapability
            ("POST", 4) if segs[2] == "capabilities" => {
                let labels: &[(&str, &str)] =
                    &[("clusterName", segs[1]), ("capabilityName", segs[3])];
                self.handle_update_capability(state, request, labels, query)
                    .await
            }
            // GET /clusters/{name}/insights/{insightId} - DescribeInsight
            ("GET", 4) if segs[2] == "insights" => {
                let labels: &[(&str, &str)] = &[("clusterName", segs[1]), ("id", segs[3])];
                self.handle_describe_insight(request, labels, query).await
            }
            // POST /clusters/{name}/insights/{insightId}/refresh - StartInsightsRefresh
            ("POST", 5) if segs[2] == "insights" && segs[4] == "refresh" => {
                let labels: &[(&str, &str)] = &[("clusterName", segs[1])];
                self.handle_start_insights_refresh(request, labels, query)
                    .await
            }
            // GET /clusters/{name}/insights/{insightId}/refresh/{refreshId} - DescribeInsightsRefresh
            ("GET", 6) if segs[2] == "insights" && segs[4] == "refresh" => {
                let labels: &[(&str, &str)] = &[("clusterName", segs[1])];
                self.handle_describe_insights_refresh(request, labels, query)
                    .await
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        }
    }

    async fn handle_create_cluster(
        &self,
        state: &Arc<tokio::sync::RwLock<EksState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_cluster_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterException", &e),
        };
        if input.name.is_empty() {
            return rest_json_error(400, "InvalidParameterException", "Missing 'name'");
        }
        if input.role_arn.is_empty() {
            return rest_json_error(400, "InvalidParameterException", "Missing 'roleArn'");
        }

        let mut state = state.write().await;
        match state.create_cluster(
            &input.name,
            &input.role_arn,
            input.version.as_deref(),
            account_id,
            region,
            input.tags,
        ) {
            Ok(cluster) => {
                let wire_c = cluster_wire(cluster);
                wire::serialize_create_cluster_response(&wire::CreateClusterResponse {
                    cluster: Some(wire_c),
                })
            }
            Err(e) => eks_error_response(&e),
        }
    }

    async fn handle_describe_cluster(
        &self,
        state: &Arc<tokio::sync::RwLock<EksState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_cluster_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterException", &e),
        };
        let state = state.read().await;
        match state.describe_cluster(&input.name) {
            Ok(cluster) => {
                let wire_c = cluster_wire(cluster);
                wire::serialize_describe_cluster_response(&wire::DescribeClusterResponse {
                    cluster: Some(wire_c),
                })
            }
            Err(e) => eks_error_response(&e),
        }
    }

    async fn handle_delete_cluster(
        &self,
        state: &Arc<tokio::sync::RwLock<EksState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_cluster_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterException", &e),
        };
        let mut state = state.write().await;
        match state.delete_cluster(&input.name) {
            Ok(cluster) => {
                let mut wire_cluster = cluster_wire(&cluster);
                wire_cluster.status = Some("DELETING".to_string());
                wire::serialize_delete_cluster_response(&wire::DeleteClusterResponse {
                    cluster: Some(wire_cluster),
                })
            }
            Err(e) => eks_error_response(&e),
        }
    }

    async fn handle_list_clusters(
        &self,
        state: &Arc<tokio::sync::RwLock<EksState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_list_clusters_request(request, labels, query) {
            return rest_json_error(400, "InvalidParameterException", &e);
        }
        let state = state.read().await;
        let clusters = state.list_clusters();
        wire::serialize_list_clusters_response(&wire::ListClustersResponse {
            clusters: Some(clusters.into_iter().map(|s| s.to_string()).collect()),
            next_token: None,
        })
    }

    async fn handle_create_nodegroup(
        &self,
        state: &Arc<tokio::sync::RwLock<EksState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_nodegroup_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterException", &e),
        };
        if input.nodegroup_name.is_empty() {
            return rest_json_error(400, "InvalidParameterException", "Missing 'nodegroupName'");
        }
        if input.node_role.is_empty() {
            return rest_json_error(400, "InvalidParameterException", "Missing 'nodeRole'");
        }

        let scaling_config = input.scaling_config.as_ref().map(|sc| ScalingConfig {
            min_size: sc.min_size.unwrap_or(1),
            max_size: sc.max_size.unwrap_or(2),
            desired_size: sc.desired_size.unwrap_or(2),
        });

        let mut state = state.write().await;
        match state.create_nodegroup(
            &input.cluster_name,
            &input.nodegroup_name,
            &input.node_role,
            scaling_config,
            account_id,
            region,
        ) {
            Ok(ng) => wire::serialize_create_nodegroup_response(&wire::CreateNodegroupResponse {
                nodegroup: Some(nodegroup_wire(ng)),
            }),
            Err(e) => eks_error_response(&e),
        }
    }

    async fn handle_describe_nodegroup(
        &self,
        state: &Arc<tokio::sync::RwLock<EksState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_nodegroup_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterException", &e),
        };
        let state = state.read().await;
        match state.describe_nodegroup(&input.cluster_name, &input.nodegroup_name) {
            Ok(ng) => {
                wire::serialize_describe_nodegroup_response(&wire::DescribeNodegroupResponse {
                    nodegroup: Some(nodegroup_wire(ng)),
                })
            }
            Err(e) => eks_error_response(&e),
        }
    }

    async fn handle_delete_nodegroup(
        &self,
        state: &Arc<tokio::sync::RwLock<EksState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_nodegroup_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterException", &e),
        };
        let mut state = state.write().await;
        match state.delete_nodegroup(&input.cluster_name, &input.nodegroup_name) {
            Ok(ng) => {
                let mut wire_ng = nodegroup_wire(&ng);
                wire_ng.status = Some("DELETING".to_string());
                wire::serialize_delete_nodegroup_response(&wire::DeleteNodegroupResponse {
                    nodegroup: Some(wire_ng),
                })
            }
            Err(e) => eks_error_response(&e),
        }
    }

    async fn handle_list_nodegroups(
        &self,
        state: &Arc<tokio::sync::RwLock<EksState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_nodegroups_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterException", &e),
        };
        let state = state.read().await;
        match state.list_nodegroups(&input.cluster_name) {
            Ok(names) => wire::serialize_list_nodegroups_response(&wire::ListNodegroupsResponse {
                nodegroups: Some(names.into_iter().map(|s| s.to_string()).collect()),
                next_token: None,
            }),
            Err(e) => eks_error_response(&e),
        }
    }

    async fn handle_update_nodegroup_config(
        &self,
        state: &Arc<tokio::sync::RwLock<EksState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_nodegroup_config_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterException", &e),
        };

        let scaling_config = input.scaling_config.as_ref().map(|sc| ScalingConfig {
            min_size: sc.min_size.unwrap_or(1),
            max_size: sc.max_size.unwrap_or(2),
            desired_size: sc.desired_size.unwrap_or(2),
        });

        let labels_add = input
            .labels
            .as_ref()
            .and_then(|l| l.add_or_update_labels.clone());
        let labels_remove = input.labels.as_ref().and_then(|l| l.remove_labels.clone());

        let taints_add = input.taints.as_ref().and_then(|t| {
            t.add_or_update_taints.as_ref().map(|arr| {
                arr.iter()
                    .filter_map(|taint| {
                        let key = taint.key.as_ref()?.clone();
                        let effect = taint.effect.as_ref()?.clone();
                        let value = taint.value.clone();
                        Some(crate::types::Taint { key, value, effect })
                    })
                    .collect()
            })
        });
        let taints_remove = input.taints.as_ref().and_then(|t| {
            t.remove_taints.as_ref().map(|arr| {
                arr.iter()
                    .filter_map(|taint| {
                        let key = taint.key.as_ref()?.clone();
                        let effect = taint.effect.as_ref()?.clone();
                        Some((key, effect))
                    })
                    .collect()
            })
        });

        let mut state = state.write().await;
        match state.update_nodegroup_config(
            &input.cluster_name,
            &input.nodegroup_name,
            scaling_config,
            labels_add,
            labels_remove,
            taints_add,
            taints_remove,
        ) {
            Ok(_ng) => {
                let update = wire::Update {
                    id: Some(uuid::Uuid::new_v4().to_string()),
                    status: Some("Successful".to_string()),
                    r#type: Some("ConfigUpdate".to_string()),
                    ..Default::default()
                };
                wire::serialize_update_nodegroup_config_response(
                    &wire::UpdateNodegroupConfigResponse {
                        update: Some(update),
                    },
                )
            }
            Err(e) => eks_error_response(&e),
        }
    }

    async fn handle_update_cluster_config(
        &self,
        state: &Arc<tokio::sync::RwLock<EksState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_cluster_config_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterException", &e),
        };
        let mut state = state.write().await;
        match state.update_cluster_config(&input.name) {
            Ok(_cluster) => {
                let update = wire::Update {
                    id: Some(uuid::Uuid::new_v4().to_string()),
                    status: Some("Successful".to_string()),
                    r#type: Some("ConfigUpdate".to_string()),
                    ..Default::default()
                };
                wire::serialize_update_cluster_config_response(&wire::UpdateClusterConfigResponse {
                    update: Some(update),
                })
            }
            Err(e) => eks_error_response(&e),
        }
    }

    // --- Fargate Profile handlers ---

    async fn handle_create_fargate_profile(
        &self,
        state: &Arc<tokio::sync::RwLock<EksState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_fargate_profile_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterException", &e),
        };
        if input.fargate_profile_name.is_empty() {
            return rest_json_error(
                400,
                "InvalidParameterException",
                "Missing 'fargateProfileName'",
            );
        }
        if input.pod_execution_role_arn.is_empty() {
            return rest_json_error(
                400,
                "InvalidParameterException",
                "Missing 'podExecutionRoleArn'",
            );
        }

        let selectors: Vec<FargateSelector> = input
            .selectors
            .unwrap_or_default()
            .into_iter()
            .map(|s| FargateSelector {
                namespace: s.namespace.unwrap_or_default(),
                labels: s.labels.unwrap_or_default(),
            })
            .collect();

        let mut state = state.write().await;
        match state.create_fargate_profile(
            &input.cluster_name,
            &input.fargate_profile_name,
            &input.pod_execution_role_arn,
            selectors,
            account_id,
            region,
        ) {
            Ok(fp) => wire::serialize_create_fargate_profile_response(
                &wire::CreateFargateProfileResponse {
                    fargate_profile: Some(fargate_profile_wire(fp)),
                },
            ),
            Err(e) => eks_error_response(&e),
        }
    }

    async fn handle_describe_fargate_profile(
        &self,
        state: &Arc<tokio::sync::RwLock<EksState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_fargate_profile_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterException", &e),
        };
        let state = state.read().await;
        match state.describe_fargate_profile(&input.cluster_name, &input.fargate_profile_name) {
            Ok(fp) => wire::serialize_describe_fargate_profile_response(
                &wire::DescribeFargateProfileResponse {
                    fargate_profile: Some(fargate_profile_wire(fp)),
                },
            ),
            Err(e) => eks_error_response(&e),
        }
    }

    async fn handle_delete_fargate_profile(
        &self,
        state: &Arc<tokio::sync::RwLock<EksState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_fargate_profile_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterException", &e),
        };
        let mut state = state.write().await;
        match state.delete_fargate_profile(&input.cluster_name, &input.fargate_profile_name) {
            Ok(fp) => {
                let mut wire_fp = fargate_profile_wire(&fp);
                wire_fp.status = Some("DELETING".to_string());
                wire::serialize_delete_fargate_profile_response(
                    &wire::DeleteFargateProfileResponse {
                        fargate_profile: Some(wire_fp),
                    },
                )
            }
            Err(e) => eks_error_response(&e),
        }
    }

    async fn handle_list_fargate_profiles(
        &self,
        state: &Arc<tokio::sync::RwLock<EksState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_fargate_profiles_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterException", &e),
        };
        let state = state.read().await;
        match state.list_fargate_profiles(&input.cluster_name) {
            Ok(names) => {
                wire::serialize_list_fargate_profiles_response(&wire::ListFargateProfilesResponse {
                    fargate_profile_names: Some(names.into_iter().map(|s| s.to_string()).collect()),
                    next_token: None,
                })
            }
            Err(e) => eks_error_response(&e),
        }
    }

    // --- Addon handlers ---

    async fn handle_create_addon(
        &self,
        state: &Arc<tokio::sync::RwLock<EksState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_addon_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterException", &e),
        };
        if input.addon_name.is_empty() {
            return rest_json_error(400, "InvalidParameterException", "Missing 'addonName'");
        }

        let mut state = state.write().await;
        match state.create_addon(
            &input.cluster_name,
            &input.addon_name,
            input.addon_version.as_deref(),
            input.service_account_role_arn.as_deref(),
            account_id,
            region,
            input.tags,
        ) {
            Ok(addon) => wire::serialize_create_addon_response(&wire::CreateAddonResponse {
                addon: Some(addon_wire(addon)),
            }),
            Err(e) => eks_error_response(&e),
        }
    }

    async fn handle_describe_addon(
        &self,
        state: &Arc<tokio::sync::RwLock<EksState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_addon_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterException", &e),
        };
        let state = state.read().await;
        match state.describe_addon(&input.cluster_name, &input.addon_name) {
            Ok(addon) => wire::serialize_describe_addon_response(&wire::DescribeAddonResponse {
                addon: Some(addon_wire(addon)),
            }),
            Err(e) => eks_error_response(&e),
        }
    }

    async fn handle_delete_addon(
        &self,
        state: &Arc<tokio::sync::RwLock<EksState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_addon_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterException", &e),
        };
        let mut state = state.write().await;
        match state.delete_addon(&input.cluster_name, &input.addon_name) {
            Ok(addon) => {
                let mut wire_addon = addon_wire(&addon);
                wire_addon.status = Some("DELETING".to_string());
                wire::serialize_delete_addon_response(&wire::DeleteAddonResponse {
                    addon: Some(wire_addon),
                })
            }
            Err(e) => eks_error_response(&e),
        }
    }

    async fn handle_update_addon(
        &self,
        state: &Arc<tokio::sync::RwLock<EksState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_addon_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterException", &e),
        };
        let mut state = state.write().await;
        match state.update_addon(
            &input.cluster_name,
            &input.addon_name,
            input.addon_version.as_deref(),
            input.service_account_role_arn.as_deref(),
        ) {
            Ok(_) => {
                let update = wire::Update {
                    id: Some(uuid::Uuid::new_v4().to_string()),
                    status: Some("Successful".to_string()),
                    r#type: Some("AddonUpdate".to_string()),
                    ..Default::default()
                };
                wire::serialize_update_addon_response(&wire::UpdateAddonResponse {
                    update: Some(update),
                })
            }
            Err(e) => eks_error_response(&e),
        }
    }

    async fn handle_list_addons(
        &self,
        state: &Arc<tokio::sync::RwLock<EksState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_addons_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterException", &e),
        };
        let state = state.read().await;
        match state.list_addons(&input.cluster_name) {
            Ok(names) => wire::serialize_list_addons_response(&wire::ListAddonsResponse {
                addons: Some(names.into_iter().map(|s| s.to_string()).collect()),
                next_token: None,
            }),
            Err(e) => eks_error_response(&e),
        }
    }

    // Returns a static addon version catalogue with common EKS addons.
    async fn handle_describe_addon_versions(
        &self,
        request: &MockRequest,
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_describe_addon_versions_request(request, &[], query) {
            return rest_json_error(400, "InvalidParameterException", &e);
        }
        let addons = vec![
            wire::AddonInfo {
                addon_name: Some("vpc-cni".to_string()),
                addon_versions: Some(vec![wire::AddonVersionInfo {
                    addon_version: Some("v1.18.0-eksbuild.1".to_string()),
                    compatibilities: Some(vec![]),
                    ..Default::default()
                }]),
                ..Default::default()
            },
            wire::AddonInfo {
                addon_name: Some("coredns".to_string()),
                addon_versions: Some(vec![wire::AddonVersionInfo {
                    addon_version: Some("v1.11.1-eksbuild.4".to_string()),
                    compatibilities: Some(vec![]),
                    ..Default::default()
                }]),
                ..Default::default()
            },
            wire::AddonInfo {
                addon_name: Some("kube-proxy".to_string()),
                addon_versions: Some(vec![wire::AddonVersionInfo {
                    addon_version: Some("v1.28.2-eksbuild.2".to_string()),
                    compatibilities: Some(vec![]),
                    ..Default::default()
                }]),
                ..Default::default()
            },
        ];
        wire::serialize_describe_addon_versions_response(&wire::DescribeAddonVersionsResponse {
            addons: Some(addons),
            next_token: None,
        })
    }

    // Returns a static addon configuration schema.
    async fn handle_describe_addon_configuration(
        &self,
        request: &MockRequest,
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_describe_addon_configuration_request(request, &[], query)
        {
            return rest_json_error(400, "InvalidParameterException", &e);
        }
        wire::serialize_describe_addon_configuration_response(
            &wire::DescribeAddonConfigurationResponse {
                addon_name: None,
                addon_version: None,
                configuration_schema: Some("{}".to_string()),
                pod_identity_configuration: None,
            },
        )
    }

    // --- Access entry handlers ---

    async fn handle_create_access_entry(
        &self,
        state: &Arc<tokio::sync::RwLock<EksState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_access_entry_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterException", &e),
        };
        if input.principal_arn.is_empty() {
            return rest_json_error(400, "InvalidParameterException", "Missing 'principalArn'");
        }
        let kubernetes_groups = input.kubernetes_groups.unwrap_or_default();

        let mut state = state.write().await;
        match state.create_access_entry(
            &input.cluster_name,
            &input.principal_arn,
            kubernetes_groups,
            input.r#type.as_deref(),
            input.username.as_deref(),
            account_id,
            region,
            input.tags,
        ) {
            Ok(entry) => {
                wire::serialize_create_access_entry_response(&wire::CreateAccessEntryResponse {
                    access_entry: Some(access_entry_wire(entry)),
                })
            }
            Err(e) => eks_error_response(&e),
        }
    }

    async fn handle_describe_access_entry(
        &self,
        state: &Arc<tokio::sync::RwLock<EksState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_access_entry_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterException", &e),
        };
        let state = state.read().await;
        match state.describe_access_entry(&input.cluster_name, &input.principal_arn) {
            Ok(entry) => {
                wire::serialize_describe_access_entry_response(&wire::DescribeAccessEntryResponse {
                    access_entry: Some(access_entry_wire(entry)),
                })
            }
            Err(e) => eks_error_response(&e),
        }
    }

    async fn handle_delete_access_entry(
        &self,
        state: &Arc<tokio::sync::RwLock<EksState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_access_entry_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterException", &e),
        };
        let mut state = state.write().await;
        match state.delete_access_entry(&input.cluster_name, &input.principal_arn) {
            Ok(()) => {
                wire::serialize_delete_access_entry_response(&wire::DeleteAccessEntryResponse {})
            }
            Err(e) => eks_error_response(&e),
        }
    }

    async fn handle_update_access_entry(
        &self,
        state: &Arc<tokio::sync::RwLock<EksState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_access_entry_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterException", &e),
        };
        let mut state = state.write().await;
        match state.update_access_entry(
            &input.cluster_name,
            &input.principal_arn,
            input.kubernetes_groups,
            input.username.as_deref(),
        ) {
            Ok(entry) => {
                wire::serialize_update_access_entry_response(&wire::UpdateAccessEntryResponse {
                    access_entry: Some(access_entry_wire(entry)),
                })
            }
            Err(e) => eks_error_response(&e),
        }
    }

    async fn handle_list_access_entries(
        &self,
        state: &Arc<tokio::sync::RwLock<EksState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_access_entries_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterException", &e),
        };
        let state = state.read().await;
        match state.list_access_entries(&input.cluster_name) {
            Ok(entries) => {
                wire::serialize_list_access_entries_response(&wire::ListAccessEntriesResponse {
                    access_entries: Some(entries),
                    next_token: None,
                })
            }
            Err(e) => eks_error_response(&e),
        }
    }

    async fn handle_associate_access_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<EksState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_associate_access_policy_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterException", &e),
        };
        if input.policy_arn.is_empty() {
            return rest_json_error(400, "InvalidParameterException", "Missing 'policyArn'");
        }

        let scope_type = input
            .access_scope
            .r#type
            .clone()
            .unwrap_or_else(|| "cluster".to_string());
        let namespaces = input.access_scope.namespaces.clone().unwrap_or_default();

        let mut state = state.write().await;
        match state.associate_access_policy(
            &input.cluster_name,
            &input.principal_arn,
            &input.policy_arn,
            &scope_type,
            namespaces,
        ) {
            Ok(policy) => {
                let now = chrono::Utc::now().timestamp() as f64;
                wire::serialize_associate_access_policy_response(
                    &wire::AssociateAccessPolicyResponse {
                        cluster_name: Some(input.cluster_name.clone()),
                        principal_arn: Some(input.principal_arn.clone()),
                        associated_access_policy: Some(wire::AssociatedAccessPolicy {
                            policy_arn: Some(policy.policy_arn.clone()),
                            access_scope: Some(wire::AccessScope {
                                r#type: Some(policy.access_scope_type.clone()),
                                namespaces: if policy.namespaces.is_empty() {
                                    None
                                } else {
                                    Some(policy.namespaces.clone())
                                },
                            }),
                            associated_at: Some(now),
                            modified_at: Some(now),
                        }),
                    },
                )
            }
            Err(e) => eks_error_response(&e),
        }
    }

    async fn handle_disassociate_access_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<EksState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_disassociate_access_policy_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "InvalidParameterException", &e),
            };
        let mut state = state.write().await;
        match state.disassociate_access_policy(
            &input.cluster_name,
            &input.principal_arn,
            &input.policy_arn,
        ) {
            Ok(()) => wire::serialize_disassociate_access_policy_response(
                &wire::DisassociateAccessPolicyResponse {},
            ),
            Err(e) => eks_error_response(&e),
        }
    }

    async fn handle_list_associated_access_policies(
        &self,
        state: &Arc<tokio::sync::RwLock<EksState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_list_associated_access_policies_request(request, labels, query)
            {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "InvalidParameterException", &e),
            };
        let state = state.read().await;
        match state.list_associated_access_policies(&input.cluster_name, &input.principal_arn) {
            Ok(policies) => {
                let now = chrono::Utc::now().timestamp() as f64;
                wire::serialize_list_associated_access_policies_response(
                    &wire::ListAssociatedAccessPoliciesResponse {
                        cluster_name: Some(input.cluster_name.clone()),
                        principal_arn: Some(input.principal_arn.clone()),
                        associated_access_policies: Some(
                            policies
                                .iter()
                                .map(|p| wire::AssociatedAccessPolicy {
                                    policy_arn: Some(p.policy_arn.clone()),
                                    access_scope: Some(wire::AccessScope {
                                        r#type: Some(p.access_scope_type.clone()),
                                        namespaces: if p.namespaces.is_empty() {
                                            None
                                        } else {
                                            Some(p.namespaces.clone())
                                        },
                                    }),
                                    associated_at: Some(now),
                                    modified_at: Some(now),
                                })
                                .collect(),
                        ),
                        next_token: None,
                    },
                )
            }
            Err(e) => eks_error_response(&e),
        }
    }

    async fn handle_list_access_policies(
        &self,
        request: &MockRequest,
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_list_access_policies_request(request, &[], query) {
            return rest_json_error(400, "InvalidParameterException", &e);
        }
        let policies = vec![
            wire::AccessPolicy {
                name: Some("AmazonEKSAdminPolicy".to_string()),
                arn: Some(
                    "arn:aws:eks::aws:cluster-access-policy/AmazonEKSAdminPolicy".to_string(),
                ),
            },
            wire::AccessPolicy {
                name: Some("AmazonEKSClusterAdminPolicy".to_string()),
                arn: Some(
                    "arn:aws:eks::aws:cluster-access-policy/AmazonEKSClusterAdminPolicy"
                        .to_string(),
                ),
            },
            wire::AccessPolicy {
                name: Some("AmazonEKSEditPolicy".to_string()),
                arn: Some("arn:aws:eks::aws:cluster-access-policy/AmazonEKSEditPolicy".to_string()),
            },
            wire::AccessPolicy {
                name: Some("AmazonEKSViewPolicy".to_string()),
                arn: Some("arn:aws:eks::aws:cluster-access-policy/AmazonEKSViewPolicy".to_string()),
            },
        ];
        wire::serialize_list_access_policies_response(&wire::ListAccessPoliciesResponse {
            access_policies: Some(policies),
            next_token: None,
        })
    }

    // --- Identity provider config handlers ---

    async fn handle_associate_identity_provider_config(
        &self,
        state: &Arc<tokio::sync::RwLock<EksState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_associate_identity_provider_config_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterException", &e),
        };
        if input.oidc.identity_provider_config_name.is_empty() {
            return rest_json_error(
                400,
                "InvalidParameterException",
                "Missing 'identityProviderConfigName'",
            );
        }

        let mut state = state.write().await;
        match state.associate_identity_provider_config(
            &input.cluster_name,
            &input.oidc.identity_provider_config_name,
            &input.oidc.issuer_url,
            &input.oidc.client_id,
            account_id,
            region,
            input.tags,
        ) {
            Ok(_config) => {
                let update = wire::Update {
                    id: Some(uuid::Uuid::new_v4().to_string()),
                    status: Some("Successful".to_string()),
                    r#type: Some("AssociateIdentityProviderConfig".to_string()),
                    ..Default::default()
                };
                wire::serialize_associate_identity_provider_config_response(
                    &wire::AssociateIdentityProviderConfigResponse {
                        update: Some(update),
                        tags: None,
                    },
                )
            }
            Err(e) => eks_error_response(&e),
        }
    }

    async fn handle_disassociate_identity_provider_config(
        &self,
        state: &Arc<tokio::sync::RwLock<EksState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_disassociate_identity_provider_config_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterException", &e),
        };
        let config_name = input.identity_provider_config.name.as_str();
        let mut state = state.write().await;
        match state.disassociate_identity_provider_config(&input.cluster_name, config_name) {
            Ok(()) => {
                let update = wire::Update {
                    id: Some(uuid::Uuid::new_v4().to_string()),
                    status: Some("Successful".to_string()),
                    r#type: Some("DisassociateIdentityProviderConfig".to_string()),
                    ..Default::default()
                };
                wire::serialize_disassociate_identity_provider_config_response(
                    &wire::DisassociateIdentityProviderConfigResponse {
                        update: Some(update),
                    },
                )
            }
            Err(e) => eks_error_response(&e),
        }
    }

    async fn handle_describe_identity_provider_config(
        &self,
        state: &Arc<tokio::sync::RwLock<EksState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_identity_provider_config_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterException", &e),
        };
        let config_name = input.identity_provider_config.name.as_str();
        let state = state.read().await;
        match state.describe_identity_provider_config(&input.cluster_name, config_name) {
            Ok(config) => wire::serialize_describe_identity_provider_config_response(
                &wire::DescribeIdentityProviderConfigResponse {
                    identity_provider_config: Some(wire::IdentityProviderConfigResponse {
                        oidc: Some(wire::OidcIdentityProviderConfig {
                            identity_provider_config_arn: Some(config.arn.clone()),
                            identity_provider_config_name: Some(config.name.clone()),
                            cluster_name: Some(config.cluster_name.clone()),
                            issuer_url: Some(config.issuer_url.clone()),
                            client_id: Some(config.client_id.clone()),
                            status: Some(config.status.clone()),
                            tags: if config.tags.is_empty() {
                                None
                            } else {
                                Some(config.tags.clone())
                            },
                            ..Default::default()
                        }),
                    }),
                },
            ),
            Err(e) => eks_error_response(&e),
        }
    }

    async fn handle_list_identity_provider_configs(
        &self,
        state: &Arc<tokio::sync::RwLock<EksState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_identity_provider_configs_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterException", &e),
        };
        let state = state.read().await;
        match state.list_identity_provider_configs(&input.cluster_name) {
            Ok(configs) => wire::serialize_list_identity_provider_configs_response(
                &wire::ListIdentityProviderConfigsResponse {
                    identity_provider_configs: Some(
                        configs
                            .iter()
                            .map(|(name, config_type)| wire::IdentityProviderConfig {
                                name: name.to_string(),
                                r#type: config_type.to_string(),
                            })
                            .collect(),
                    ),
                    next_token: None,
                },
            ),
            Err(e) => eks_error_response(&e),
        }
    }

    // --- Pod identity association handlers ---

    async fn handle_create_pod_identity_association(
        &self,
        state: &Arc<tokio::sync::RwLock<EksState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input =
            match wire::deserialize_create_pod_identity_association_request(request, labels, query)
            {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "InvalidParameterException", &e),
            };
        if input.namespace.is_empty() {
            return rest_json_error(400, "InvalidParameterException", "Missing 'namespace'");
        }
        if input.service_account.is_empty() {
            return rest_json_error(400, "InvalidParameterException", "Missing 'serviceAccount'");
        }
        if input.role_arn.is_empty() {
            return rest_json_error(400, "InvalidParameterException", "Missing 'roleArn'");
        }

        let mut state = state.write().await;
        match state.create_pod_identity_association(
            &input.cluster_name,
            &input.namespace,
            &input.service_account,
            &input.role_arn,
            account_id,
            region,
            input.tags,
        ) {
            Ok(assoc) => wire::serialize_create_pod_identity_association_response(
                &wire::CreatePodIdentityAssociationResponse {
                    association: Some(pod_identity_association_wire(assoc)),
                },
            ),
            Err(e) => eks_error_response(&e),
        }
    }

    async fn handle_describe_pod_identity_association(
        &self,
        state: &Arc<tokio::sync::RwLock<EksState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_pod_identity_association_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterException", &e),
        };
        let state = state.read().await;
        match state.describe_pod_identity_association(&input.cluster_name, &input.association_id) {
            Ok(assoc) => wire::serialize_describe_pod_identity_association_response(
                &wire::DescribePodIdentityAssociationResponse {
                    association: Some(pod_identity_association_wire(assoc)),
                },
            ),
            Err(e) => eks_error_response(&e),
        }
    }

    async fn handle_delete_pod_identity_association(
        &self,
        state: &Arc<tokio::sync::RwLock<EksState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_delete_pod_identity_association_request(request, labels, query)
            {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "InvalidParameterException", &e),
            };
        let mut state = state.write().await;
        match state.delete_pod_identity_association(&input.cluster_name, &input.association_id) {
            Ok(assoc) => wire::serialize_delete_pod_identity_association_response(
                &wire::DeletePodIdentityAssociationResponse {
                    association: Some(pod_identity_association_wire(&assoc)),
                },
            ),
            Err(e) => eks_error_response(&e),
        }
    }

    async fn handle_update_pod_identity_association(
        &self,
        state: &Arc<tokio::sync::RwLock<EksState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_update_pod_identity_association_request(request, labels, query)
            {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "InvalidParameterException", &e),
            };
        let mut state = state.write().await;
        match state.update_pod_identity_association(
            &input.cluster_name,
            &input.association_id,
            input.role_arn.as_deref(),
        ) {
            Ok(assoc) => wire::serialize_update_pod_identity_association_response(
                &wire::UpdatePodIdentityAssociationResponse {
                    association: Some(pod_identity_association_wire(assoc)),
                },
            ),
            Err(e) => eks_error_response(&e),
        }
    }

    async fn handle_list_pod_identity_associations(
        &self,
        state: &Arc<tokio::sync::RwLock<EksState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_pod_identity_associations_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterException", &e),
        };
        let state = state.read().await;
        match state.list_pod_identity_associations(&input.cluster_name) {
            Ok(assocs) => wire::serialize_list_pod_identity_associations_response(
                &wire::ListPodIdentityAssociationsResponse {
                    associations: Some(
                        assocs
                            .iter()
                            .map(|a| wire::PodIdentityAssociationSummary {
                                association_arn: Some(a.association_arn.clone()),
                                association_id: Some(a.association_id.clone()),
                                cluster_name: Some(a.cluster_name.clone()),
                                namespace: Some(a.namespace.clone()),
                                service_account: Some(a.service_account.clone()),
                                owner_arn: None,
                            })
                            .collect(),
                    ),
                    next_token: None,
                },
            ),
            Err(e) => eks_error_response(&e),
        }
    }

    // --- EKS Anywhere subscription handlers ---

    async fn handle_create_eks_anywhere_subscription(
        &self,
        state: &Arc<tokio::sync::RwLock<EksState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_eks_anywhere_subscription_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterException", &e),
        };
        if input.name.is_empty() {
            return rest_json_error(400, "InvalidParameterException", "Missing 'name'");
        }
        let auto_renew = input.auto_renew.unwrap_or(false);
        let license_quantity = input.license_quantity.unwrap_or(1);
        let license_type = input.license_type.as_deref().unwrap_or("Enterprise");

        let mut state = state.write().await;
        match state.create_eks_anywhere_subscription(
            &input.name,
            auto_renew,
            license_quantity,
            license_type,
            account_id,
            region,
            input.tags,
        ) {
            Ok(sub) => wire::serialize_create_eks_anywhere_subscription_response(
                &wire::CreateEksAnywhereSubscriptionResponse {
                    subscription: Some(eks_anywhere_subscription_wire(sub)),
                },
            ),
            Err(e) => eks_error_response(&e),
        }
    }

    async fn handle_describe_eks_anywhere_subscription(
        &self,
        state: &Arc<tokio::sync::RwLock<EksState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_eks_anywhere_subscription_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterException", &e),
        };
        let state = state.read().await;
        match state.describe_eks_anywhere_subscription(&input.id) {
            Ok(sub) => wire::serialize_describe_eks_anywhere_subscription_response(
                &wire::DescribeEksAnywhereSubscriptionResponse {
                    subscription: Some(eks_anywhere_subscription_wire(sub)),
                },
            ),
            Err(e) => eks_error_response(&e),
        }
    }

    async fn handle_delete_eks_anywhere_subscription(
        &self,
        state: &Arc<tokio::sync::RwLock<EksState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_eks_anywhere_subscription_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterException", &e),
        };
        let mut state = state.write().await;
        match state.delete_eks_anywhere_subscription(&input.id) {
            Ok(sub) => {
                let mut wire_sub = eks_anywhere_subscription_wire(&sub);
                wire_sub.status = Some("DELETING".to_string());
                wire::serialize_delete_eks_anywhere_subscription_response(
                    &wire::DeleteEksAnywhereSubscriptionResponse {
                        subscription: Some(wire_sub),
                    },
                )
            }
            Err(e) => eks_error_response(&e),
        }
    }

    async fn handle_update_eks_anywhere_subscription(
        &self,
        state: &Arc<tokio::sync::RwLock<EksState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_eks_anywhere_subscription_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterException", &e),
        };
        let mut state = state.write().await;
        match state.update_eks_anywhere_subscription(&input.id, input.auto_renew) {
            Ok(sub) => wire::serialize_update_eks_anywhere_subscription_response(
                &wire::UpdateEksAnywhereSubscriptionResponse {
                    subscription: Some(eks_anywhere_subscription_wire(sub)),
                },
            ),
            Err(e) => eks_error_response(&e),
        }
    }

    async fn handle_list_eks_anywhere_subscriptions(
        &self,
        state: &Arc<tokio::sync::RwLock<EksState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) =
            wire::deserialize_list_eks_anywhere_subscriptions_request(request, labels, query)
        {
            return rest_json_error(400, "InvalidParameterException", &e);
        }
        let state = state.read().await;
        let subs = state.list_eks_anywhere_subscriptions();
        wire::serialize_list_eks_anywhere_subscriptions_response(
            &wire::ListEksAnywhereSubscriptionsResponse {
                subscriptions: Some(
                    subs.iter()
                        .map(|s| eks_anywhere_subscription_wire(s))
                        .collect(),
                ),
                next_token: None,
            },
        )
    }

    // --- Capability handlers ---

    async fn handle_create_capability(
        &self,
        state: &Arc<tokio::sync::RwLock<EksState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_capability_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterException", &e),
        };
        if input.capability_name.is_empty() {
            return rest_json_error(400, "InvalidParameterException", "Missing 'capabilityName'");
        }
        let capability_type = if input.r#type.is_empty() {
            "ARGO_CD"
        } else {
            input.r#type.as_str()
        };

        let mut state = state.write().await;
        match state.create_capability(
            &input.cluster_name,
            &input.capability_name,
            capability_type,
            &input.role_arn,
            account_id,
            region,
            input.tags,
        ) {
            Ok(cap) => {
                wire::serialize_create_capability_response(&wire::CreateCapabilityResponse {
                    capability: Some(capability_wire(cap)),
                })
            }
            Err(e) => eks_error_response(&e),
        }
    }

    async fn handle_describe_capability(
        &self,
        state: &Arc<tokio::sync::RwLock<EksState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_capability_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterException", &e),
        };
        let state = state.read().await;
        match state.describe_capability(&input.cluster_name, &input.capability_name) {
            Ok(cap) => {
                wire::serialize_describe_capability_response(&wire::DescribeCapabilityResponse {
                    capability: Some(capability_wire(cap)),
                })
            }
            Err(e) => eks_error_response(&e),
        }
    }

    async fn handle_delete_capability(
        &self,
        state: &Arc<tokio::sync::RwLock<EksState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_capability_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterException", &e),
        };
        let mut state = state.write().await;
        match state.delete_capability(&input.cluster_name, &input.capability_name) {
            Ok(cap) => {
                let mut wire_cap = capability_wire(&cap);
                wire_cap.status = Some("DELETING".to_string());
                wire::serialize_delete_capability_response(&wire::DeleteCapabilityResponse {
                    capability: Some(wire_cap),
                })
            }
            Err(e) => eks_error_response(&e),
        }
    }

    async fn handle_update_capability(
        &self,
        state: &Arc<tokio::sync::RwLock<EksState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_capability_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterException", &e),
        };
        let mut state = state.write().await;
        match state.update_capability(
            &input.cluster_name,
            &input.capability_name,
            input.role_arn.as_deref(),
        ) {
            Ok(cap) => {
                let update = wire::Update {
                    id: Some(uuid::Uuid::new_v4().to_string()),
                    status: Some("Successful".to_string()),
                    r#type: Some("CapabilityUpdate".to_string()),
                    ..Default::default()
                };
                let _ = cap;
                wire::serialize_update_capability_response(&wire::UpdateCapabilityResponse {
                    update: Some(update),
                })
            }
            Err(e) => eks_error_response(&e),
        }
    }

    async fn handle_list_capabilities(
        &self,
        state: &Arc<tokio::sync::RwLock<EksState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_capabilities_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterException", &e),
        };
        let state = state.read().await;
        match state.list_capabilities(&input.cluster_name) {
            Ok(caps) => {
                wire::serialize_list_capabilities_response(&wire::ListCapabilitiesResponse {
                    capabilities: Some(
                        caps.iter()
                            .map(|c| wire::CapabilitySummary {
                                arn: Some(c.arn.clone()),
                                capability_name: Some(c.capability_name.clone()),
                                status: Some(c.status.clone()),
                                r#type: Some(c.capability_type.clone()),
                                created_at: Some(c.created_at.timestamp() as f64),
                                modified_at: Some(c.modified_at.timestamp() as f64),
                                version: None,
                            })
                            .collect(),
                    ),
                    next_token: None,
                })
            }
            Err(e) => eks_error_response(&e),
        }
    }

    // --- Update handlers ---

    async fn handle_update_cluster_version(
        &self,
        state: &Arc<tokio::sync::RwLock<EksState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_cluster_version_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterException", &e),
        };
        let state = state.read().await;
        match state.describe_cluster(&input.name) {
            Ok(_) => {
                let update = wire::Update {
                    id: Some(uuid::Uuid::new_v4().to_string()),
                    status: Some("InProgress".to_string()),
                    r#type: Some("VersionUpdate".to_string()),
                    ..Default::default()
                };
                wire::serialize_update_cluster_version_response(
                    &wire::UpdateClusterVersionResponse {
                        update: Some(update),
                    },
                )
            }
            Err(e) => eks_error_response(&e),
        }
    }

    async fn handle_update_nodegroup_version(
        &self,
        state: &Arc<tokio::sync::RwLock<EksState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_nodegroup_version_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterException", &e),
        };
        let state = state.read().await;
        match state.describe_nodegroup(&input.cluster_name, &input.nodegroup_name) {
            Ok(_) => {
                let update = wire::Update {
                    id: Some(uuid::Uuid::new_v4().to_string()),
                    status: Some("InProgress".to_string()),
                    r#type: Some("VersionUpdate".to_string()),
                    ..Default::default()
                };
                wire::serialize_update_nodegroup_version_response(
                    &wire::UpdateNodegroupVersionResponse {
                        update: Some(update),
                    },
                )
            }
            Err(e) => eks_error_response(&e),
        }
    }

    async fn handle_describe_update(
        &self,
        state: &Arc<tokio::sync::RwLock<EksState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_update_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterException", &e),
        };
        let guard = state.read().await;
        match guard.describe_update(&input.name, &input.update_id) {
            Ok(record) => {
                let update = wire::Update {
                    id: Some(record.id.clone()),
                    status: Some(record.status.clone()),
                    r#type: Some(record.update_type.clone()),
                    ..Default::default()
                };
                wire::serialize_describe_update_response(&wire::DescribeUpdateResponse {
                    update: Some(update),
                })
            }
            Err(e) => eks_error_response(&e),
        }
    }

    async fn handle_list_updates(
        &self,
        state: &Arc<tokio::sync::RwLock<EksState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_updates_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterException", &e),
        };
        let guard = state.read().await;
        match guard.list_updates(&input.name) {
            Ok(ids) => wire::serialize_list_updates_response(&wire::ListUpdatesResponse {
                update_ids: Some(ids.into_iter().map(|s| s.to_string()).collect()),
                next_token: None,
            }),
            Err(e) => eks_error_response(&e),
        }
    }

    async fn handle_associate_encryption_config(
        &self,
        state: &Arc<tokio::sync::RwLock<EksState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_associate_encryption_config_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "InvalidParameterException", &e),
            };
        let mut guard = state.write().await;
        match guard.record_update(&input.cluster_name, "AssociateEncryptionConfig") {
            Ok(record) => {
                let update = wire::Update {
                    id: Some(record.id.clone()),
                    status: Some(record.status.clone()),
                    r#type: Some(record.update_type.clone()),
                    ..Default::default()
                };
                wire::serialize_associate_encryption_config_response(
                    &wire::AssociateEncryptionConfigResponse {
                        update: Some(update),
                    },
                )
            }
            Err(e) => eks_error_response(&e),
        }
    }

    // --- RegisterCluster / DeregisterCluster handlers ---

    async fn handle_register_cluster(
        &self,
        state: &Arc<tokio::sync::RwLock<EksState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_register_cluster_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterException", &e),
        };
        if input.name.is_empty() {
            return rest_json_error(400, "InvalidParameterException", "Missing 'name'");
        }

        let mut state = state.write().await;
        match state.register_cluster(
            &input.name,
            &input.connector_config.provider,
            &input.connector_config.role_arn,
            account_id,
            region,
            input.tags,
        ) {
            Ok(cluster) => {
                let wire_c = cluster_wire(cluster);
                wire::serialize_register_cluster_response(&wire::RegisterClusterResponse {
                    cluster: Some(wire_c),
                })
            }
            Err(e) => eks_error_response(&e),
        }
    }

    async fn handle_deregister_cluster(
        &self,
        state: &Arc<tokio::sync::RwLock<EksState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_deregister_cluster_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterException", &e),
        };
        let mut state = state.write().await;
        match state.deregister_cluster(&input.name) {
            Ok(cluster) => {
                let wire_c = cluster_wire(&cluster);
                wire::serialize_deregister_cluster_response(&wire::DeregisterClusterResponse {
                    cluster: Some(wire_c),
                })
            }
            Err(e) => eks_error_response(&e),
        }
    }

    // --- Insights handlers ---

    // STUB[no-telemetry]: EKS Insights are driven by real infrastructure telemetry; mock has no such data.
    async fn handle_describe_insight(
        &self,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_describe_insight_request(request, labels, query) {
            return rest_json_error(400, "InvalidParameterException", &e);
        }
        wire::serialize_describe_insight_response(&wire::DescribeInsightResponse { insight: None })
    }

    // STUB[no-telemetry]: EKS Insights are driven by real infrastructure telemetry; mock has no such data.
    async fn handle_list_insights(
        &self,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_list_insights_request(request, labels, query) {
            return rest_json_error(400, "InvalidParameterException", &e);
        }
        wire::serialize_list_insights_response(&wire::ListInsightsResponse {
            insights: Some(vec![]),
            next_token: None,
        })
    }

    // STUB[no-telemetry]: EKS Insights refresh status is driven by real telemetry; always returns COMPLETED.
    async fn handle_describe_insights_refresh(
        &self,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_describe_insights_refresh_request(request, labels, query)
        {
            return rest_json_error(400, "InvalidParameterException", &e);
        }
        wire::serialize_describe_insights_refresh_response(&wire::DescribeInsightsRefreshResponse {
            status: Some("COMPLETED".to_string()),
            ..Default::default()
        })
    }

    // STUB[no-telemetry]: EKS Insights refresh is driven by real telemetry; always returns IN_PROGRESS.
    async fn handle_start_insights_refresh(
        &self,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_start_insights_refresh_request(request, labels, query) {
            return rest_json_error(400, "InvalidParameterException", &e);
        }
        wire::serialize_start_insights_refresh_response(&wire::StartInsightsRefreshResponse {
            status: Some("IN_PROGRESS".to_string()),
            ..Default::default()
        })
    }

    // --- DescribeClusterVersions ---

    async fn handle_describe_cluster_versions(
        &self,
        request: &MockRequest,
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_describe_cluster_versions_request(request, &[], query) {
            return rest_json_error(400, "InvalidParameterException", &e);
        }
        let versions = vec!["1.32", "1.31", "1.30", "1.29", "1.28"]
            .into_iter()
            .map(|v| wire::ClusterVersionInformation {
                cluster_version: Some(v.to_string()),
                cluster_type: Some("eks".to_string()),
                default_version: Some(v == "1.31"),
                status: Some("standard-support".to_string()),
                version_status: Some("standard-support".to_string()),
                ..Default::default()
            })
            .collect();
        wire::serialize_describe_cluster_versions_response(&wire::DescribeClusterVersionsResponse {
            cluster_versions: Some(versions),
            next_token: None,
        })
    }

    // --- Tag handlers ---

    async fn handle_list_tags_for_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<EksState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_tags_for_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterException", &e),
        };
        let state = state.read().await;
        let tags = state.list_tags_for_resource(&input.resource_arn);
        wire::serialize_list_tags_for_resource_response(&wire::ListTagsForResourceResponse {
            tags: Some(tags),
        })
    }

    async fn handle_tag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<EksState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_tag_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterException", &e),
        };
        let mut state = state.write().await;
        state.tag_resource(&input.resource_arn, input.tags);
        wire::serialize_tag_resource_response(&wire::TagResourceResponse {})
    }

    async fn handle_untag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<EksState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_untag_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidParameterException", &e),
        };
        let mut state = state.write().await;
        state.untag_resource(&input.resource_arn, &input.tag_keys);
        wire::serialize_untag_resource_response(&wire::UntagResourceResponse {})
    }
}

fn cluster_wire(cluster: &crate::types::Cluster) -> wire::Cluster {
    wire::Cluster {
        name: Some(cluster.name.clone()),
        arn: Some(cluster.arn.clone()),
        endpoint: Some(cluster.endpoint.clone()),
        role_arn: Some(cluster.role_arn.clone()),
        status: Some(cluster.status.clone()),
        version: Some(cluster.version.clone()),
        created_at: Some(cluster.created_at.timestamp() as f64),
        tags: if cluster.tags.is_empty() {
            None
        } else {
            Some(cluster.tags.clone())
        },
        ..Default::default()
    }
}

fn nodegroup_wire(ng: &crate::types::Nodegroup) -> wire::Nodegroup {
    wire::Nodegroup {
        nodegroup_name: Some(ng.name.clone()),
        nodegroup_arn: Some(ng.arn.clone()),
        cluster_name: Some(ng.cluster_name.clone()),
        node_role: Some(ng.node_role.clone()),
        status: Some(ng.status.clone()),
        scaling_config: Some(wire::NodegroupScalingConfig {
            min_size: Some(ng.scaling_config.min_size),
            max_size: Some(ng.scaling_config.max_size),
            desired_size: Some(ng.scaling_config.desired_size),
        }),
        labels: Some(ng.labels.clone()),
        taints: Some(
            ng.taints
                .iter()
                .map(|t| wire::Taint {
                    key: Some(t.key.clone()),
                    value: t.value.clone(),
                    effect: Some(t.effect.clone()),
                })
                .collect(),
        ),
        ..Default::default()
    }
}

fn fargate_profile_wire(fp: &crate::types::FargateProfile) -> wire::FargateProfile {
    wire::FargateProfile {
        fargate_profile_name: Some(fp.name.clone()),
        fargate_profile_arn: Some(fp.arn.clone()),
        cluster_name: Some(fp.cluster_name.clone()),
        pod_execution_role_arn: Some(fp.pod_execution_role_arn.clone()),
        status: Some(fp.status.clone()),
        selectors: Some(
            fp.selectors
                .iter()
                .map(|s| wire::FargateProfileSelector {
                    namespace: Some(s.namespace.clone()),
                    labels: if s.labels.is_empty() {
                        None
                    } else {
                        Some(s.labels.clone())
                    },
                })
                .collect(),
        ),
        ..Default::default()
    }
}

fn addon_wire(addon: &crate::types::Addon) -> wire::Addon {
    wire::Addon {
        addon_name: Some(addon.addon_name.clone()),
        addon_arn: Some(addon.addon_arn.clone()),
        cluster_name: Some(addon.cluster_name.clone()),
        addon_version: Some(addon.addon_version.clone()),
        service_account_role_arn: addon.service_account_role_arn.clone(),
        status: Some(addon.status.clone()),
        created_at: Some(addon.created_at.timestamp() as f64),
        modified_at: Some(addon.modified_at.timestamp() as f64),
        tags: if addon.tags.is_empty() {
            None
        } else {
            Some(addon.tags.clone())
        },
        ..Default::default()
    }
}

fn access_entry_wire(entry: &crate::types::AccessEntry) -> wire::AccessEntry {
    wire::AccessEntry {
        access_entry_arn: Some(entry.access_entry_arn.clone()),
        cluster_name: Some(entry.cluster_name.clone()),
        principal_arn: Some(entry.principal_arn.clone()),
        kubernetes_groups: if entry.kubernetes_groups.is_empty() {
            None
        } else {
            Some(entry.kubernetes_groups.clone())
        },
        r#type: Some(entry.entry_type.clone()),
        username: entry.username.clone(),
        created_at: Some(entry.created_at.timestamp() as f64),
        modified_at: Some(entry.modified_at.timestamp() as f64),
        tags: if entry.tags.is_empty() {
            None
        } else {
            Some(entry.tags.clone())
        },
    }
}

fn pod_identity_association_wire(
    assoc: &crate::types::PodIdentityAssociation,
) -> wire::PodIdentityAssociation {
    wire::PodIdentityAssociation {
        association_arn: Some(assoc.association_arn.clone()),
        association_id: Some(assoc.association_id.clone()),
        cluster_name: Some(assoc.cluster_name.clone()),
        namespace: Some(assoc.namespace.clone()),
        service_account: Some(assoc.service_account.clone()),
        role_arn: Some(assoc.role_arn.clone()),
        created_at: Some(assoc.created_at.timestamp() as f64),
        modified_at: Some(assoc.modified_at.timestamp() as f64),
        tags: if assoc.tags.is_empty() {
            None
        } else {
            Some(assoc.tags.clone())
        },
        ..Default::default()
    }
}

fn eks_anywhere_subscription_wire(
    sub: &crate::types::EksAnywhereSubscription,
) -> wire::EksAnywhereSubscription {
    wire::EksAnywhereSubscription {
        id: Some(sub.id.clone()),
        arn: Some(sub.arn.clone()),
        auto_renew: Some(sub.auto_renew),
        license_quantity: Some(sub.license_quantity),
        license_type: Some(sub.license_type.clone()),
        status: Some(sub.status.clone()),
        created_at: Some(sub.created_at.timestamp() as f64),
        tags: if sub.tags.is_empty() {
            None
        } else {
            Some(sub.tags.clone())
        },
        ..Default::default()
    }
}

fn capability_wire(cap: &crate::types::CapabilityStore) -> wire::Capability {
    wire::Capability {
        arn: Some(cap.arn.clone()),
        capability_name: Some(cap.capability_name.clone()),
        cluster_name: Some(cap.cluster_name.clone()),
        role_arn: Some(cap.role_arn.clone()),
        r#type: Some(cap.capability_type.clone()),
        status: Some(cap.status.clone()),
        created_at: Some(cap.created_at.timestamp() as f64),
        modified_at: Some(cap.modified_at.timestamp() as f64),
        tags: if cap.tags.is_empty() {
            None
        } else {
            Some(cap.tags.clone())
        },
        ..Default::default()
    }
}

fn percent_decode(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c == '%' {
            let hex: String = chars.by_ref().take(2).collect();
            if let Ok(byte) = u8::from_str_radix(&hex, 16) {
                result.push(byte as char);
            } else {
                result.push('%');
                result.push_str(&hex);
            }
        } else {
            result.push(c);
        }
    }
    result
}

fn extract_path_and_query(uri: &str) -> (String, String) {
    // FIX(terraform-e2e): when requests arrive from the local test server the URI is
    // "https://127.0.0.1:{port}/clusters/..." which contains no "amazonaws.com".
    // Fall back to stripping scheme://host so the path component is correctly
    // extracted in both production (amazonaws.com) and test (127.0.0.1) contexts.
    let path_part = if let Some(idx) = uri.find("amazonaws.com") {
        &uri[idx + "amazonaws.com".len()..]
    } else if let Some(idx) = uri.find("://") {
        let after_scheme = &uri[idx + 3..];
        if let Some(slash) = after_scheme.find('/') {
            &after_scheme[slash..]
        } else {
            "/"
        }
    } else {
        uri
    };

    if let Some(q) = path_part.find('?') {
        (path_part[..q].to_string(), path_part[q + 1..].to_string())
    } else {
        (path_part.to_string(), String::new())
    }
}

fn eks_error_response(err: &EksError) -> MockResponse {
    let (status, error_type) = match err {
        EksError::ClusterAlreadyExists(_) => (409, "ResourceInUseException"),
        EksError::ClusterAlreadyExistsPlain(_) => (409, "ResourceInUseException"),
        EksError::ClusterHasActiveResources => (409, "ResourceInUseException"),
        EksError::ClusterNotFound(_) => (404, "ResourceNotFoundException"),
        EksError::NodegroupAlreadyExists(_) => (409, "ResourceInUseException"),
        EksError::NodegroupNotFound(_) => (404, "ResourceNotFoundException"),
        EksError::FargateProfileAlreadyExists(_) => (409, "ResourceInUseException"),
        EksError::FargateProfileNotFound(_) => (404, "ResourceNotFoundException"),
        EksError::AddonAlreadyExists(_) => (409, "ResourceInUseException"),
        EksError::AddonNotFound(_) => (404, "ResourceNotFoundException"),
        EksError::AccessEntryAlreadyExists(_) => (409, "ResourceInUseException"),
        EksError::AccessEntryNotFound(_) => (404, "ResourceNotFoundException"),
        EksError::IdentityProviderConfigAlreadyExists(_) => (409, "ResourceInUseException"),
        EksError::IdentityProviderConfigNotFound(_) => (404, "ResourceNotFoundException"),
        EksError::PodIdentityAssociationNotFound(_) => (404, "ResourceNotFoundException"),
        EksError::EksAnywhereSubscriptionNotFound(_) => (404, "ResourceNotFoundException"),
        EksError::CapabilityAlreadyExists(_) => (409, "ResourceInUseException"),
        EksError::CapabilityNotFound(_) => (404, "ResourceNotFoundException"),
    };
    let body = json!({
        "Type": "User",
        "Message": err.to_string(),
    });
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers
        .insert(X_AMZN_ERRORTYPE, error_type.parse().unwrap());
    resp
}

fn rest_json_error(status: u16, code: &str, message: &str) -> MockResponse {
    let body = json!({
        "Type": "User",
        "Message": message,
    });
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers.insert(X_AMZN_ERRORTYPE, code.parse().unwrap());
    resp
}
