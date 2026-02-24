use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::ETAG;
use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
    StatefulService,
};

use crate::state::{CloudFrontError, CloudFrontState};
use crate::types::{DefaultCacheBehavior, Origin};
use crate::views::CloudFrontStateView;
#[allow(unused_imports)]
use crate::wire;

pub struct CloudFrontService {
    pub(crate) state: Arc<BackendState<CloudFrontState>>,
    pub(crate) notifier: StateChangeNotifier<CloudFrontStateView>,
}

impl CloudFrontService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for CloudFrontService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for CloudFrontService {
    fn service_name(&self) -> &str {
        "cloudfront"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://cloudfront\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl CloudFrontService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let account_id = DEFAULT_ACCOUNT_ID;
        let region = "us-east-1";
        let state = self.state.get(account_id, region);

        let (path, query_string) = extract_path_and_query(&request.uri);
        let method = request.method.as_str();

        let segments: Vec<&str> = path
            .trim_start_matches('/')
            .split('/')
            .filter(|s| !s.is_empty())
            .collect();

        // Parse query parameters
        let query_params = parse_query_string(&query_string);

        // Routes based on CloudFront REST-XML API (2020-05-31)
        if segments.is_empty() || segments[0] != "2020-05-31" {
            return xml_error_response(404, "InvalidURI", "The requested URI is not valid.");
        }

        let body_str = std::str::from_utf8(&request.body).unwrap_or("");
        let if_match = request
            .headers
            .get("if-match")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());

        let response = match (method, segments.as_slice()) {
            // POST /2020-05-31/distribution?WithTags - CreateDistributionWithTags
            ("POST", ["2020-05-31", "distribution"]) if query_params.contains_key("WithTags") => {
                self.handle_create_distribution_with_tags(&state, body_str, account_id)
                    .await
            }
            // POST /2020-05-31/distribution - CreateDistribution
            ("POST", ["2020-05-31", "distribution"]) => {
                self.handle_create_distribution(&state, body_str, account_id)
                    .await
            }
            // GET /2020-05-31/distribution - ListDistributions
            ("GET", ["2020-05-31", "distribution"]) => self.handle_list_distributions(&state).await,
            // GET /2020-05-31/distribution/{id}/config - GetDistributionConfig
            ("GET", ["2020-05-31", "distribution", id, "config"]) => {
                self.handle_get_distribution_config(&state, id).await
            }
            // PUT /2020-05-31/distribution/{id}/config - UpdateDistribution
            ("PUT", ["2020-05-31", "distribution", id, "config"]) => {
                self.handle_update_distribution(&state, id, body_str, if_match.as_deref())
                    .await
            }
            // GET /2020-05-31/distribution/{id} - GetDistribution
            ("GET", ["2020-05-31", "distribution", id]) => {
                self.handle_get_distribution(&state, id).await
            }
            // DELETE /2020-05-31/distribution/{id} - DeleteDistribution
            ("DELETE", ["2020-05-31", "distribution", id]) => {
                self.handle_delete_distribution(&state, id, if_match.as_deref())
                    .await
            }
            // POST /2020-05-31/distribution/{id}/invalidation - CreateInvalidation
            ("POST", ["2020-05-31", "distribution", id, "invalidation"]) => {
                self.handle_create_invalidation(&state, id, body_str).await
            }
            // GET /2020-05-31/distribution/{id}/invalidation - ListInvalidations
            ("GET", ["2020-05-31", "distribution", id, "invalidation"]) => {
                self.handle_list_invalidations(&state, id).await
            }
            // GET /2020-05-31/distribution/{dist_id}/invalidation/{inv_id} - GetInvalidation
            (
                "GET",
                [
                    "2020-05-31",
                    "distribution",
                    dist_id,
                    "invalidation",
                    inv_id,
                ],
            ) => self.handle_get_invalidation(&state, dist_id, inv_id).await,
            // POST /2020-05-31/key-group - CreateKeyGroup
            ("POST", ["2020-05-31", "key-group"]) => {
                self.handle_create_key_group(&state, body_str).await
            }
            // GET /2020-05-31/key-group - ListKeyGroups
            ("GET", ["2020-05-31", "key-group"]) => self.handle_list_key_groups(&state).await,
            // GET /2020-05-31/key-group/{id} - GetKeyGroup
            ("GET", ["2020-05-31", "key-group", id]) => self.handle_get_key_group(&state, id).await,
            // POST /2020-05-31/origin-access-control - CreateOriginAccessControl
            ("POST", ["2020-05-31", "origin-access-control"]) => {
                self.handle_create_origin_access_control(&state, body_str)
                    .await
            }
            // GET /2020-05-31/origin-access-control - ListOriginAccessControls
            ("GET", ["2020-05-31", "origin-access-control"]) => {
                self.handle_list_origin_access_controls(&state).await
            }
            // GET /2020-05-31/origin-access-control/{id} - GetOriginAccessControl
            ("GET", ["2020-05-31", "origin-access-control", id]) => {
                self.handle_get_origin_access_control(&state, id).await
            }
            // PUT /2020-05-31/origin-access-control/{id}/config - UpdateOriginAccessControl
            ("PUT", ["2020-05-31", "origin-access-control", id, "config"]) => {
                self.handle_update_origin_access_control(&state, id, body_str, if_match.as_deref())
                    .await
            }
            // DELETE /2020-05-31/origin-access-control/{id} - DeleteOriginAccessControl
            ("DELETE", ["2020-05-31", "origin-access-control", id]) => {
                self.handle_delete_origin_access_control(&state, id).await
            }
            // POST /2020-05-31/public-key - CreatePublicKey
            ("POST", ["2020-05-31", "public-key"]) => {
                self.handle_create_public_key(&state, body_str).await
            }
            // GET /2020-05-31/public-key - ListPublicKeys
            ("GET", ["2020-05-31", "public-key"]) => self.handle_list_public_keys(&state).await,
            // GET /2020-05-31/public-key/{id} - GetPublicKey
            ("GET", ["2020-05-31", "public-key", id]) => {
                self.handle_get_public_key(&state, id).await
            }
            // DELETE /2020-05-31/public-key/{id} - DeletePublicKey
            ("DELETE", ["2020-05-31", "public-key", id]) => {
                self.handle_delete_public_key(&state, id).await
            }
            // POST /2020-05-31/tagging?Operation=Tag&Resource={arn} - TagResource
            ("POST", ["2020-05-31", "tagging"])
                if query_params.get("Operation").map(|s| s.as_str()) == Some("Tag") =>
            {
                let arn = query_params.get("Resource").cloned().unwrap_or_default();
                self.handle_tag_resource(&state, &arn, body_str).await
            }
            // POST /2020-05-31/tagging?Operation=Untag&Resource={arn} - UntagResource
            ("POST", ["2020-05-31", "tagging"])
                if query_params.get("Operation").map(|s| s.as_str()) == Some("Untag") =>
            {
                let arn = query_params.get("Resource").cloned().unwrap_or_default();
                self.handle_untag_resource(&state, &arn, body_str).await
            }
            // GET /2020-05-31/tagging?Resource={arn} - ListTagsForResource
            ("GET", ["2020-05-31", "tagging"]) => {
                let arn = query_params.get("Resource").cloned().unwrap_or_default();
                self.handle_list_tags_for_resource(&state, &arn).await
            }
            // AssociateAlias (PUT /2020-05-31/distribution/{TargetDistributionId}/associate-alias)
            ("PUT", ["2020-05-31", "distribution", id, "associate-alias"]) => {
                self.handle_associate_alias(&state, id, &query_params).await
            }
            // AssociateDistributionTenantWebACL (PUT /2020-05-31/distribution-tenant/{Id}/associate-web-acl)
            ("PUT", ["2020-05-31", "distribution-tenant", id, "associate-web-acl"]) => {
                self.handle_associate_distribution_tenant_web_acl(&state, id, body_str)
                    .await
            }
            // AssociateDistributionWebACL (PUT /2020-05-31/distribution/{Id}/associate-web-acl)
            ("PUT", ["2020-05-31", "distribution", id, "associate-web-acl"]) => {
                self.handle_associate_distribution_web_acl(&state, id, body_str)
                    .await
            }
            // CopyDistribution (POST /2020-05-31/distribution/{PrimaryDistributionId}/copy)
            ("POST", ["2020-05-31", "distribution", id, "copy"]) => {
                self.handle_copy_distribution(&state, id).await
            }
            // CreateAnycastIpList (POST /2020-05-31/anycast-ip-list)
            ("POST", ["2020-05-31", "anycast-ip-list"]) => {
                self.handle_create_anycast_ip_list(&state, body_str, account_id)
                    .await
            }
            // CreateCachePolicy (POST /2020-05-31/cache-policy)
            ("POST", ["2020-05-31", "cache-policy"]) => {
                self.handle_create_cache_policy(&state, body_str).await
            }
            // CreateCloudFrontOriginAccessIdentity (POST /2020-05-31/origin-access-identity/cloudfront)
            ("POST", ["2020-05-31", "origin-access-identity", "cloudfront"]) => {
                self.handle_create_cloud_front_origin_access_identity(&state, body_str)
                    .await
            }
            // CreateConnectionFunction (POST /2020-05-31/connection-function)
            ("POST", ["2020-05-31", "connection-function"]) => {
                self.handle_create_connection_function(&state, body_str, account_id)
                    .await
            }
            // CreateConnectionGroup (POST /2020-05-31/connection-group)
            ("POST", ["2020-05-31", "connection-group"]) => {
                self.handle_create_connection_group(&state, body_str, account_id)
                    .await
            }
            // CreateContinuousDeploymentPolicy (POST /2020-05-31/continuous-deployment-policy)
            ("POST", ["2020-05-31", "continuous-deployment-policy"]) => {
                self.handle_create_continuous_deployment_policy(&state, body_str)
                    .await
            }
            // CreateDistributionTenant (POST /2020-05-31/distribution-tenant)
            ("POST", ["2020-05-31", "distribution-tenant"]) => {
                self.handle_create_distribution_tenant(&state, body_str, account_id)
                    .await
            }
            // CreateFieldLevelEncryptionConfig (POST /2020-05-31/field-level-encryption)
            ("POST", ["2020-05-31", "field-level-encryption"]) => {
                self.handle_create_field_level_encryption_config(&state, body_str)
                    .await
            }
            // CreateFieldLevelEncryptionProfile (POST /2020-05-31/field-level-encryption-profile)
            ("POST", ["2020-05-31", "field-level-encryption-profile"]) => {
                self.handle_create_field_level_encryption_profile(&state, body_str)
                    .await
            }
            // CreateFunction (POST /2020-05-31/function)
            ("POST", ["2020-05-31", "function"]) => {
                self.handle_create_function(&state, body_str, account_id)
                    .await
            }
            // CreateInvalidationForDistributionTenant (POST /2020-05-31/distribution-tenant/{Id}/invalidation)
            ("POST", ["2020-05-31", "distribution-tenant", id, "invalidation"]) => {
                self.handle_create_invalidation_for_distribution_tenant(&state, id, body_str)
                    .await
            }
            // CreateKeyValueStore (POST /2020-05-31/key-value-store)
            ("POST", ["2020-05-31", "key-value-store"]) => {
                self.handle_create_key_value_store(&state, body_str, account_id)
                    .await
            }
            // CreateMonitoringSubscription (POST /2020-05-31/distributions/{DistributionId}/monitoring-subscription)
            ("POST", ["2020-05-31", "distributions", id, "monitoring-subscription"]) => {
                self.handle_create_monitoring_subscription(&state, id, body_str)
                    .await
            }
            // CreateOriginRequestPolicy (POST /2020-05-31/origin-request-policy)
            ("POST", ["2020-05-31", "origin-request-policy"]) => {
                self.handle_create_origin_request_policy(&state, body_str)
                    .await
            }
            // CreateRealtimeLogConfig (POST /2020-05-31/realtime-log-config)
            ("POST", ["2020-05-31", "realtime-log-config"]) => {
                self.handle_create_realtime_log_config(&state, body_str, account_id)
                    .await
            }
            // CreateResponseHeadersPolicy (POST /2020-05-31/response-headers-policy)
            ("POST", ["2020-05-31", "response-headers-policy"]) => {
                self.handle_create_response_headers_policy(&state, body_str)
                    .await
            }
            // CreateStreamingDistribution (POST /2020-05-31/streaming-distribution) - no WithTags
            ("POST", ["2020-05-31", "streaming-distribution"])
                if !query_params.contains_key("WithTags") =>
            {
                self.handle_create_streaming_distribution(&state, body_str, account_id)
                    .await
            }
            // CreateStreamingDistributionWithTags (POST /2020-05-31/streaming-distribution?WithTags)
            ("POST", ["2020-05-31", "streaming-distribution"])
                if query_params.contains_key("WithTags") =>
            {
                self.handle_create_streaming_distribution_with_tags(&state, body_str, account_id)
                    .await
            }
            // CreateTrustStore (POST /2020-05-31/trust-store)
            ("POST", ["2020-05-31", "trust-store"]) => {
                self.handle_create_trust_store(&state, body_str, account_id)
                    .await
            }
            // CreateVpcOrigin (POST /2020-05-31/vpc-origin)
            ("POST", ["2020-05-31", "vpc-origin"]) => {
                self.handle_create_vpc_origin(&state, body_str, account_id)
                    .await
            }
            // DeleteAnycastIpList (DELETE /2020-05-31/anycast-ip-list/{Id})
            ("DELETE", ["2020-05-31", "anycast-ip-list", id]) => {
                self.handle_delete_anycast_ip_list(&state, id, if_match.as_deref())
                    .await
            }
            // DeleteCachePolicy (DELETE /2020-05-31/cache-policy/{Id})
            ("DELETE", ["2020-05-31", "cache-policy", id]) => {
                self.handle_delete_cache_policy(&state, id, if_match.as_deref())
                    .await
            }
            // DeleteCloudFrontOriginAccessIdentity (DELETE /2020-05-31/origin-access-identity/cloudfront/{Id})
            ("DELETE", ["2020-05-31", "origin-access-identity", "cloudfront", id]) => {
                self.handle_delete_cloud_front_origin_access_identity(
                    &state,
                    id,
                    if_match.as_deref(),
                )
                .await
            }
            // DeleteConnectionFunction (DELETE /2020-05-31/connection-function/{Id})
            ("DELETE", ["2020-05-31", "connection-function", id]) => {
                self.handle_delete_connection_function(&state, id).await
            }
            // DeleteConnectionGroup (DELETE /2020-05-31/connection-group/{Id})
            ("DELETE", ["2020-05-31", "connection-group", id]) => {
                self.handle_delete_connection_group(&state, id).await
            }
            // DeleteContinuousDeploymentPolicy (DELETE /2020-05-31/continuous-deployment-policy/{Id})
            ("DELETE", ["2020-05-31", "continuous-deployment-policy", id]) => {
                self.handle_delete_continuous_deployment_policy(&state, id)
                    .await
            }
            // DeleteDistributionTenant (DELETE /2020-05-31/distribution-tenant/{Id})
            ("DELETE", ["2020-05-31", "distribution-tenant", id]) => {
                self.handle_delete_distribution_tenant(&state, id).await
            }
            // DeleteFieldLevelEncryptionConfig (DELETE /2020-05-31/field-level-encryption/{Id})
            ("DELETE", ["2020-05-31", "field-level-encryption", id]) => {
                self.handle_delete_field_level_encryption_config(&state, id, if_match.as_deref())
                    .await
            }
            // DeleteFieldLevelEncryptionProfile (DELETE /2020-05-31/field-level-encryption-profile/{Id})
            ("DELETE", ["2020-05-31", "field-level-encryption-profile", id]) => {
                self.handle_delete_field_level_encryption_profile(&state, id, if_match.as_deref())
                    .await
            }
            // DeleteFunction (DELETE /2020-05-31/function/{Name})
            ("DELETE", ["2020-05-31", "function", name]) => {
                self.handle_delete_function(&state, name, if_match.as_deref())
                    .await
            }
            // DeleteKeyGroup (DELETE /2020-05-31/key-group/{Id})
            ("DELETE", ["2020-05-31", "key-group", id]) => {
                self.handle_delete_key_group(&state, id, if_match.as_deref())
                    .await
            }
            // DeleteKeyValueStore (DELETE /2020-05-31/key-value-store/{Name})
            ("DELETE", ["2020-05-31", "key-value-store", name]) => {
                self.handle_delete_key_value_store(&state, name, if_match.as_deref())
                    .await
            }
            // DeleteMonitoringSubscription (DELETE /2020-05-31/distributions/{DistributionId}/monitoring-subscription)
            ("DELETE", ["2020-05-31", "distributions", id, "monitoring-subscription"]) => {
                self.handle_delete_monitoring_subscription(&state, id).await
            }
            // DeleteOriginRequestPolicy (DELETE /2020-05-31/origin-request-policy/{Id})
            ("DELETE", ["2020-05-31", "origin-request-policy", id]) => {
                self.handle_delete_origin_request_policy(&state, id, if_match.as_deref())
                    .await
            }
            // DeleteRealtimeLogConfig (POST /2020-05-31/delete-realtime-log-config)
            ("POST", ["2020-05-31", "delete-realtime-log-config"]) => {
                self.handle_delete_realtime_log_config(&state, body_str)
                    .await
            }
            // DeleteResourcePolicy (POST /2020-05-31/delete-resource-policy)
            ("POST", ["2020-05-31", "delete-resource-policy"]) => {
                self.handle_delete_resource_policy(&state, body_str).await
            }
            // DeleteResponseHeadersPolicy (DELETE /2020-05-31/response-headers-policy/{Id})
            ("DELETE", ["2020-05-31", "response-headers-policy", id]) => {
                self.handle_delete_response_headers_policy(&state, id, if_match.as_deref())
                    .await
            }
            // DeleteStreamingDistribution (DELETE /2020-05-31/streaming-distribution/{Id})
            ("DELETE", ["2020-05-31", "streaming-distribution", id]) => {
                self.handle_delete_streaming_distribution(&state, id, if_match.as_deref())
                    .await
            }
            // DeleteTrustStore (DELETE /2020-05-31/trust-store/{Id})
            ("DELETE", ["2020-05-31", "trust-store", id]) => {
                self.handle_delete_trust_store(&state, id, if_match.as_deref())
                    .await
            }
            // DeleteVpcOrigin (DELETE /2020-05-31/vpc-origin/{Id})
            ("DELETE", ["2020-05-31", "vpc-origin", id]) => {
                self.handle_delete_vpc_origin(&state, id, if_match.as_deref())
                    .await
            }
            // DescribeConnectionFunction (GET /2020-05-31/connection-function/{Identifier}/describe)
            ("GET", ["2020-05-31", "connection-function", id, "describe"]) => {
                self.handle_describe_connection_function(&state, id).await
            }
            // DescribeFunction (GET /2020-05-31/function/{Name}/describe)
            ("GET", ["2020-05-31", "function", name, "describe"]) => {
                self.handle_describe_function(&state, name, if_match.as_deref())
                    .await
            }
            // DescribeKeyValueStore (GET /2020-05-31/key-value-store/{Name})
            ("GET", ["2020-05-31", "key-value-store", name]) => {
                self.handle_describe_key_value_store(&state, name).await
            }
            // DisassociateDistributionTenantWebACL (PUT /2020-05-31/distribution-tenant/{Id}/disassociate-web-acl)
            (
                "PUT",
                [
                    "2020-05-31",
                    "distribution-tenant",
                    id,
                    "disassociate-web-acl",
                ],
            ) => {
                self.handle_disassociate_distribution_tenant_web_acl(&state, id)
                    .await
            }
            // DisassociateDistributionWebACL (PUT /2020-05-31/distribution/{Id}/disassociate-web-acl)
            ("PUT", ["2020-05-31", "distribution", id, "disassociate-web-acl"]) => {
                self.handle_disassociate_distribution_web_acl(&state, id)
                    .await
            }
            // GetAnycastIpList (GET /2020-05-31/anycast-ip-list/{Id})
            ("GET", ["2020-05-31", "anycast-ip-list", id]) => {
                self.handle_get_anycast_ip_list(&state, id).await
            }
            // GetCachePolicy (GET /2020-05-31/cache-policy/{Id})
            ("GET", ["2020-05-31", "cache-policy", id]) => {
                self.handle_get_cache_policy(&state, id).await
            }
            // GetCachePolicyConfig (GET /2020-05-31/cache-policy/{Id}/config)
            ("GET", ["2020-05-31", "cache-policy", id, "config"]) => {
                self.handle_get_cache_policy_config(&state, id).await
            }
            // GetCloudFrontOriginAccessIdentity (GET /2020-05-31/origin-access-identity/cloudfront/{Id})
            ("GET", ["2020-05-31", "origin-access-identity", "cloudfront", id]) => {
                self.handle_get_cloud_front_origin_access_identity(&state, id)
                    .await
            }
            // GetCloudFrontOriginAccessIdentityConfig (GET /2020-05-31/origin-access-identity/cloudfront/{Id}/config)
            (
                "GET",
                [
                    "2020-05-31",
                    "origin-access-identity",
                    "cloudfront",
                    id,
                    "config",
                ],
            ) => {
                self.handle_get_cloud_front_origin_access_identity_config(&state, id)
                    .await
            }
            // GetConnectionFunction (GET /2020-05-31/connection-function/{Identifier})
            ("GET", ["2020-05-31", "connection-function", id]) => {
                self.handle_get_connection_function(&state, id).await
            }
            // GetConnectionGroup (GET /2020-05-31/connection-group/{Identifier})
            ("GET", ["2020-05-31", "connection-group", id])
                if !query_params.contains_key("RoutingEndpoint") =>
            {
                self.handle_get_connection_group(&state, id).await
            }
            // GetConnectionGroupByRoutingEndpoint (GET /2020-05-31/connection-group)
            ("GET", ["2020-05-31", "connection-group"]) => {
                let endpoint = query_params
                    .get("RoutingEndpoint")
                    .map(|s| s.as_str())
                    .unwrap_or("");
                self.handle_get_connection_group_by_routing_endpoint(&state, endpoint)
                    .await
            }
            // GetContinuousDeploymentPolicy (GET /2020-05-31/continuous-deployment-policy/{Id})
            ("GET", ["2020-05-31", "continuous-deployment-policy", id]) => {
                self.handle_get_continuous_deployment_policy(&state, id)
                    .await
            }
            // GetContinuousDeploymentPolicyConfig (GET /2020-05-31/continuous-deployment-policy/{Id}/config)
            ("GET", ["2020-05-31", "continuous-deployment-policy", id, "config"]) => {
                self.handle_get_continuous_deployment_policy_config(&state, id)
                    .await
            }
            // GetDistributionTenant (GET /2020-05-31/distribution-tenant/{Identifier})
            ("GET", ["2020-05-31", "distribution-tenant", id])
                if !query_params.contains_key("Domain") =>
            {
                self.handle_get_distribution_tenant(&state, id).await
            }
            // GetDistributionTenantByDomain (GET /2020-05-31/distribution-tenant)
            ("GET", ["2020-05-31", "distribution-tenant"]) => {
                let domain = query_params.get("Domain").map(|s| s.as_str()).unwrap_or("");
                self.handle_get_distribution_tenant_by_domain(&state, domain)
                    .await
            }
            // GetFieldLevelEncryption (GET /2020-05-31/field-level-encryption/{Id})
            ("GET", ["2020-05-31", "field-level-encryption", id]) => {
                self.handle_get_field_level_encryption(&state, id).await
            }
            // GetFieldLevelEncryptionConfig (GET /2020-05-31/field-level-encryption/{Id}/config)
            ("GET", ["2020-05-31", "field-level-encryption", id, "config"]) => {
                self.handle_get_field_level_encryption_config(&state, id)
                    .await
            }
            // GetFieldLevelEncryptionProfile (GET /2020-05-31/field-level-encryption-profile/{Id})
            ("GET", ["2020-05-31", "field-level-encryption-profile", id]) => {
                self.handle_get_field_level_encryption_profile(&state, id)
                    .await
            }
            // GetFieldLevelEncryptionProfileConfig (GET /2020-05-31/field-level-encryption-profile/{Id}/config)
            ("GET", ["2020-05-31", "field-level-encryption-profile", id, "config"]) => {
                self.handle_get_field_level_encryption_profile_config(&state, id)
                    .await
            }
            // GetFunction (GET /2020-05-31/function/{Name})
            ("GET", ["2020-05-31", "function", name]) => {
                self.handle_get_function(&state, name, if_match.as_deref())
                    .await
            }
            // GetInvalidationForDistributionTenant (GET /2020-05-31/distribution-tenant/{DistributionTenantId}/invalidation/{Id})
            (
                "GET",
                [
                    "2020-05-31",
                    "distribution-tenant",
                    tenant_id,
                    "invalidation",
                    inv_id,
                ],
            ) => {
                self.handle_get_invalidation_for_distribution_tenant(&state, tenant_id, inv_id)
                    .await
            }
            // GetKeyGroupConfig (GET /2020-05-31/key-group/{Id}/config)
            ("GET", ["2020-05-31", "key-group", id, "config"]) => {
                self.handle_get_key_group_config(&state, id).await
            }
            // GetManagedCertificateDetails (GET /2020-05-31/managed-certificate/{Identifier})
            ("GET", ["2020-05-31", "managed-certificate", id]) => {
                self.handle_get_managed_certificate_details(&state, id)
                    .await
            }
            // GetMonitoringSubscription (GET /2020-05-31/distributions/{DistributionId}/monitoring-subscription)
            ("GET", ["2020-05-31", "distributions", id, "monitoring-subscription"]) => {
                self.handle_get_monitoring_subscription(&state, id).await
            }
            // GetOriginAccessControlConfig (GET /2020-05-31/origin-access-control/{Id}/config)
            ("GET", ["2020-05-31", "origin-access-control", id, "config"]) => {
                self.handle_get_origin_access_control_config(&state, id)
                    .await
            }
            // GetOriginRequestPolicy (GET /2020-05-31/origin-request-policy/{Id})
            ("GET", ["2020-05-31", "origin-request-policy", id]) => {
                self.handle_get_origin_request_policy(&state, id).await
            }
            // GetOriginRequestPolicyConfig (GET /2020-05-31/origin-request-policy/{Id}/config)
            ("GET", ["2020-05-31", "origin-request-policy", id, "config"]) => {
                self.handle_get_origin_request_policy_config(&state, id)
                    .await
            }
            // GetPublicKeyConfig (GET /2020-05-31/public-key/{Id}/config)
            ("GET", ["2020-05-31", "public-key", id, "config"]) => {
                self.handle_get_public_key_config(&state, id).await
            }
            // GetRealtimeLogConfig (POST /2020-05-31/get-realtime-log-config)
            ("POST", ["2020-05-31", "get-realtime-log-config"]) => {
                self.handle_get_realtime_log_config(&state, body_str).await
            }
            // GetResourcePolicy (POST /2020-05-31/get-resource-policy)
            ("POST", ["2020-05-31", "get-resource-policy"]) => {
                self.handle_get_resource_policy(&state, body_str).await
            }
            // GetResponseHeadersPolicy (GET /2020-05-31/response-headers-policy/{Id})
            ("GET", ["2020-05-31", "response-headers-policy", id]) => {
                self.handle_get_response_headers_policy(&state, id).await
            }
            // GetResponseHeadersPolicyConfig (GET /2020-05-31/response-headers-policy/{Id}/config)
            ("GET", ["2020-05-31", "response-headers-policy", id, "config"]) => {
                self.handle_get_response_headers_policy_config(&state, id)
                    .await
            }
            // GetStreamingDistribution (GET /2020-05-31/streaming-distribution/{Id})
            ("GET", ["2020-05-31", "streaming-distribution", id]) => {
                self.handle_get_streaming_distribution(&state, id).await
            }
            // GetStreamingDistributionConfig (GET /2020-05-31/streaming-distribution/{Id}/config)
            ("GET", ["2020-05-31", "streaming-distribution", id, "config"]) => {
                self.handle_get_streaming_distribution_config(&state, id)
                    .await
            }
            // GetTrustStore (GET /2020-05-31/trust-store/{Identifier})
            ("GET", ["2020-05-31", "trust-store", id]) => {
                self.handle_get_trust_store(&state, id).await
            }
            // GetVpcOrigin (GET /2020-05-31/vpc-origin/{Id})
            ("GET", ["2020-05-31", "vpc-origin", id]) => {
                self.handle_get_vpc_origin(&state, id).await
            }
            // ListAnycastIpLists (GET /2020-05-31/anycast-ip-list)
            ("GET", ["2020-05-31", "anycast-ip-list"]) => {
                self.handle_list_anycast_ip_lists(&state).await
            }
            // ListCachePolicies (GET /2020-05-31/cache-policy)
            ("GET", ["2020-05-31", "cache-policy"]) => {
                self.handle_list_cache_policies(&state).await
            }
            // ListCloudFrontOriginAccessIdentities (GET /2020-05-31/origin-access-identity/cloudfront)
            ("GET", ["2020-05-31", "origin-access-identity", "cloudfront"]) => {
                self.handle_list_cloud_front_origin_access_identities(&state)
                    .await
            }
            // ListConflictingAliases (GET /2020-05-31/conflicting-alias)
            ("GET", ["2020-05-31", "conflicting-alias"]) => {
                self.handle_list_conflicting_aliases(&state).await
            }
            // ListConnectionFunctions (POST /2020-05-31/connection-functions)
            ("POST", ["2020-05-31", "connection-functions"]) => {
                self.handle_list_connection_functions(&state).await
            }
            // ListConnectionGroups (POST /2020-05-31/connection-groups)
            ("POST", ["2020-05-31", "connection-groups"]) => {
                self.handle_list_connection_groups(&state).await
            }
            // ListContinuousDeploymentPolicies (GET /2020-05-31/continuous-deployment-policy)
            ("GET", ["2020-05-31", "continuous-deployment-policy"]) => {
                self.handle_list_continuous_deployment_policies(&state)
                    .await
            }
            // ListDistributionTenants (POST /2020-05-31/distribution-tenants)
            ("POST", ["2020-05-31", "distribution-tenants"]) => {
                self.handle_list_distribution_tenants(&state).await
            }
            // ListDistributionTenantsByCustomization (POST /2020-05-31/distribution-tenants-by-customization)
            ("POST", ["2020-05-31", "distribution-tenants-by-customization"]) => {
                self.handle_list_distribution_tenants_by_customization(&state)
                    .await
            }
            // ListDistributionsByAnycastIpListId (GET /2020-05-31/distributionsByAnycastIpListId/{AnycastIpListId})
            ("GET", ["2020-05-31", "distributionsByAnycastIpListId", id]) => {
                self.handle_list_distributions_by_anycast_ip_list_id(&state, id)
                    .await
            }
            // ListDistributionsByCachePolicyId (GET /2020-05-31/distributionsByCachePolicyId/{CachePolicyId})
            ("GET", ["2020-05-31", "distributionsByCachePolicyId", id]) => {
                self.handle_list_distributions_by_cache_policy_id(&state, id)
                    .await
            }
            // ListDistributionsByConnectionFunction (GET /2020-05-31/distributionsByConnectionFunction)
            ("GET", ["2020-05-31", "distributionsByConnectionFunction"]) => {
                self.handle_list_distributions_by_connection_function(&state)
                    .await
            }
            // ListDistributionsByConnectionMode (GET /2020-05-31/distributionsByConnectionMode/{ConnectionMode})
            ("GET", ["2020-05-31", "distributionsByConnectionMode", mode]) => {
                self.handle_list_distributions_by_connection_mode(&state, mode)
                    .await
            }
            // ListDistributionsByKeyGroup (GET /2020-05-31/distributionsByKeyGroupId/{KeyGroupId})
            ("GET", ["2020-05-31", "distributionsByKeyGroupId", id]) => {
                self.handle_list_distributions_by_key_group(&state, id)
                    .await
            }
            // ListDistributionsByOriginRequestPolicyId (GET /2020-05-31/distributionsByOriginRequestPolicyId/{OriginRequestPolicyId})
            ("GET", ["2020-05-31", "distributionsByOriginRequestPolicyId", id]) => {
                self.handle_list_distributions_by_origin_request_policy_id(&state, id)
                    .await
            }
            // ListDistributionsByOwnedResource (GET /2020-05-31/distributionsByOwnedResource/{ResourceArn})
            ("GET", ["2020-05-31", "distributionsByOwnedResource", arn]) => {
                self.handle_list_distributions_by_owned_resource(&state, arn)
                    .await
            }
            // ListDistributionsByRealtimeLogConfig (POST /2020-05-31/distributionsByRealtimeLogConfig)
            ("POST", ["2020-05-31", "distributionsByRealtimeLogConfig"]) => {
                self.handle_list_distributions_by_realtime_log_config(&state)
                    .await
            }
            // ListDistributionsByResponseHeadersPolicyId (GET /2020-05-31/distributionsByResponseHeadersPolicyId/{ResponseHeadersPolicyId})
            ("GET", ["2020-05-31", "distributionsByResponseHeadersPolicyId", id]) => {
                self.handle_list_distributions_by_response_headers_policy_id(&state, id)
                    .await
            }
            // ListDistributionsByTrustStore (GET /2020-05-31/distributionsByTrustStore)
            ("GET", ["2020-05-31", "distributionsByTrustStore"]) => {
                self.handle_list_distributions_by_trust_store(&state).await
            }
            // ListDistributionsByVpcOriginId (GET /2020-05-31/distributionsByVpcOriginId/{VpcOriginId})
            ("GET", ["2020-05-31", "distributionsByVpcOriginId", id]) => {
                self.handle_list_distributions_by_vpc_origin_id(&state, id)
                    .await
            }
            // ListDistributionsByWebACLId (GET /2020-05-31/distributionsByWebACLId/{WebACLId})
            ("GET", ["2020-05-31", "distributionsByWebACLId", id]) => {
                self.handle_list_distributions_by_web_a_c_l_id(&state, id)
                    .await
            }
            // ListDomainConflicts (POST /2020-05-31/domain-conflicts)
            ("POST", ["2020-05-31", "domain-conflicts"]) => {
                self.handle_list_domain_conflicts(&state).await
            }
            // ListFieldLevelEncryptionConfigs (GET /2020-05-31/field-level-encryption)
            ("GET", ["2020-05-31", "field-level-encryption"]) => {
                self.handle_list_field_level_encryption_configs(&state)
                    .await
            }
            // ListFieldLevelEncryptionProfiles (GET /2020-05-31/field-level-encryption-profile)
            ("GET", ["2020-05-31", "field-level-encryption-profile"]) => {
                self.handle_list_field_level_encryption_profiles(&state)
                    .await
            }
            // ListFunctions (GET /2020-05-31/function)
            ("GET", ["2020-05-31", "function"]) => self.handle_list_functions(&state).await,
            // ListInvalidationsForDistributionTenant (GET /2020-05-31/distribution-tenant/{Id}/invalidation)
            ("GET", ["2020-05-31", "distribution-tenant", id, "invalidation"]) => {
                self.handle_list_invalidations_for_distribution_tenant(&state, id)
                    .await
            }
            // ListKeyValueStores (GET /2020-05-31/key-value-store)
            ("GET", ["2020-05-31", "key-value-store"]) => {
                self.handle_list_key_value_stores(&state).await
            }
            // ListOriginRequestPolicies (GET /2020-05-31/origin-request-policy)
            ("GET", ["2020-05-31", "origin-request-policy"]) => {
                self.handle_list_origin_request_policies(&state).await
            }
            // ListRealtimeLogConfigs (GET /2020-05-31/realtime-log-config)
            ("GET", ["2020-05-31", "realtime-log-config"]) => {
                self.handle_list_realtime_log_configs(&state).await
            }
            // ListResponseHeadersPolicies (GET /2020-05-31/response-headers-policy)
            ("GET", ["2020-05-31", "response-headers-policy"]) => {
                self.handle_list_response_headers_policies(&state).await
            }
            // ListStreamingDistributions (GET /2020-05-31/streaming-distribution)
            ("GET", ["2020-05-31", "streaming-distribution"]) => {
                self.handle_list_streaming_distributions(&state).await
            }
            // ListTrustStores (POST /2020-05-31/trust-stores)
            ("POST", ["2020-05-31", "trust-stores"]) => self.handle_list_trust_stores(&state).await,
            // ListVpcOrigins (GET /2020-05-31/vpc-origin)
            ("GET", ["2020-05-31", "vpc-origin"]) => self.handle_list_vpc_origins(&state).await,
            // PublishConnectionFunction (POST /2020-05-31/connection-function/{Id}/publish)
            ("POST", ["2020-05-31", "connection-function", id, "publish"]) => {
                self.handle_publish_connection_function(&state, id).await
            }
            // PublishFunction (POST /2020-05-31/function/{Name}/publish)
            ("POST", ["2020-05-31", "function", name, "publish"]) => {
                self.handle_publish_function(&state, name, if_match.as_deref())
                    .await
            }
            // PutResourcePolicy (POST /2020-05-31/put-resource-policy)
            ("POST", ["2020-05-31", "put-resource-policy"]) => {
                self.handle_put_resource_policy(&state, body_str).await
            }
            // TestConnectionFunction (POST /2020-05-31/connection-function/{Id}/test)
            ("POST", ["2020-05-31", "connection-function", id, "test"]) => {
                self.handle_test_connection_function(&state, id).await
            }
            // TestFunction (POST /2020-05-31/function/{Name}/test)
            ("POST", ["2020-05-31", "function", name, "test"]) => {
                self.handle_test_function(&state, name).await
            }
            // UpdateAnycastIpList (PUT /2020-05-31/anycast-ip-list/{Id})
            ("PUT", ["2020-05-31", "anycast-ip-list", id]) => {
                self.handle_update_anycast_ip_list(&state, id, body_str, if_match.as_deref())
                    .await
            }
            // UpdateCachePolicy (PUT /2020-05-31/cache-policy/{Id})
            ("PUT", ["2020-05-31", "cache-policy", id]) => {
                self.handle_update_cache_policy(&state, id, body_str, if_match.as_deref())
                    .await
            }
            // UpdateCloudFrontOriginAccessIdentity (PUT /2020-05-31/origin-access-identity/cloudfront/{Id}/config)
            (
                "PUT",
                [
                    "2020-05-31",
                    "origin-access-identity",
                    "cloudfront",
                    id,
                    "config",
                ],
            ) => {
                self.handle_update_cloud_front_origin_access_identity(
                    &state,
                    id,
                    body_str,
                    if_match.as_deref(),
                )
                .await
            }
            // UpdateConnectionFunction (PUT /2020-05-31/connection-function/{Id})
            ("PUT", ["2020-05-31", "connection-function", id]) => {
                self.handle_update_connection_function(&state, id, body_str, if_match.as_deref())
                    .await
            }
            // UpdateConnectionGroup (PUT /2020-05-31/connection-group/{Id})
            ("PUT", ["2020-05-31", "connection-group", id]) => {
                self.handle_update_connection_group(&state, id, body_str, if_match.as_deref())
                    .await
            }
            // UpdateContinuousDeploymentPolicy (PUT /2020-05-31/continuous-deployment-policy/{Id})
            ("PUT", ["2020-05-31", "continuous-deployment-policy", id]) => {
                self.handle_update_continuous_deployment_policy(
                    &state,
                    id,
                    body_str,
                    if_match.as_deref(),
                )
                .await
            }
            // UpdateDistributionTenant (PUT /2020-05-31/distribution-tenant/{Id})
            ("PUT", ["2020-05-31", "distribution-tenant", id]) => {
                self.handle_update_distribution_tenant(&state, id, body_str, if_match.as_deref())
                    .await
            }
            // UpdateDistributionWithStagingConfig (PUT /2020-05-31/distribution/{Id}/promote-staging-config)
            ("PUT", ["2020-05-31", "distribution", id, "promote-staging-config"]) => {
                self.handle_update_distribution_with_staging_config(&state, id)
                    .await
            }
            // UpdateDomainAssociation (POST /2020-05-31/domain-association)
            ("POST", ["2020-05-31", "domain-association"]) => {
                self.handle_update_domain_association(&state, body_str)
                    .await
            }
            // UpdateFieldLevelEncryptionConfig (PUT /2020-05-31/field-level-encryption/{Id}/config)
            ("PUT", ["2020-05-31", "field-level-encryption", id, "config"]) => {
                self.handle_update_field_level_encryption_config(
                    &state,
                    id,
                    body_str,
                    if_match.as_deref(),
                )
                .await
            }
            // UpdateFieldLevelEncryptionProfile (PUT /2020-05-31/field-level-encryption-profile/{Id}/config)
            ("PUT", ["2020-05-31", "field-level-encryption-profile", id, "config"]) => {
                self.handle_update_field_level_encryption_profile(
                    &state,
                    id,
                    body_str,
                    if_match.as_deref(),
                )
                .await
            }
            // UpdateFunction (PUT /2020-05-31/function/{Name})
            ("PUT", ["2020-05-31", "function", name]) => {
                self.handle_update_function(&state, name, body_str, if_match.as_deref())
                    .await
            }
            // UpdateKeyGroup (PUT /2020-05-31/key-group/{Id})
            ("PUT", ["2020-05-31", "key-group", id]) => {
                self.handle_update_key_group(&state, id, body_str, if_match.as_deref())
                    .await
            }
            // UpdateKeyValueStore (PUT /2020-05-31/key-value-store/{Name})
            ("PUT", ["2020-05-31", "key-value-store", name]) => {
                self.handle_update_key_value_store(&state, name, body_str, if_match.as_deref())
                    .await
            }
            // UpdateOriginRequestPolicy (PUT /2020-05-31/origin-request-policy/{Id})
            ("PUT", ["2020-05-31", "origin-request-policy", id]) => {
                self.handle_update_origin_request_policy(&state, id, body_str, if_match.as_deref())
                    .await
            }
            // UpdatePublicKey (PUT /2020-05-31/public-key/{Id}/config)
            ("PUT", ["2020-05-31", "public-key", id, "config"]) => {
                self.handle_update_public_key(&state, id, body_str, if_match.as_deref())
                    .await
            }
            // UpdateRealtimeLogConfig (PUT /2020-05-31/realtime-log-config)
            ("PUT", ["2020-05-31", "realtime-log-config"]) => {
                self.handle_update_realtime_log_config(&state, body_str)
                    .await
            }
            // UpdateResponseHeadersPolicy (PUT /2020-05-31/response-headers-policy/{Id})
            ("PUT", ["2020-05-31", "response-headers-policy", id]) => {
                self.handle_update_response_headers_policy(
                    &state,
                    id,
                    body_str,
                    if_match.as_deref(),
                )
                .await
            }
            // UpdateStreamingDistribution (PUT /2020-05-31/streaming-distribution/{Id}/config)
            ("PUT", ["2020-05-31", "streaming-distribution", id, "config"]) => {
                self.handle_update_streaming_distribution(&state, id, body_str, if_match.as_deref())
                    .await
            }
            // UpdateTrustStore (PUT /2020-05-31/trust-store/{Id})
            ("PUT", ["2020-05-31", "trust-store", id]) => {
                self.handle_update_trust_store(&state, id, if_match.as_deref())
                    .await
            }
            // UpdateVpcOrigin (PUT /2020-05-31/vpc-origin/{Id})
            ("PUT", ["2020-05-31", "vpc-origin", id]) => {
                self.handle_update_vpc_origin(&state, id, body_str, if_match.as_deref())
                    .await
            }
            // VerifyDnsConfiguration (POST /2020-05-31/verify-dns-configuration)
            ("POST", ["2020-05-31", "verify-dns-configuration"]) => {
                self.handle_verify_dns_configuration(&state, body_str).await
            }
            _ => xml_error_response(404, "InvalidURI", "The requested URI is not valid."),
        };

        if matches!(method, "POST" | "PUT" | "DELETE") && response.status / 100 == 2 {
            self.notify_state_changed(account_id, region).await;
        }
        response
    }

    // ---- Distribution handlers ----

    async fn handle_create_distribution(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        body: &str,
        account_id: &str,
    ) -> MockResponse {
        self.do_create_distribution(state, body, account_id, false)
            .await
    }

    async fn handle_create_distribution_with_tags(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        body: &str,
        account_id: &str,
    ) -> MockResponse {
        self.do_create_distribution(state, body, account_id, true)
            .await
    }

    async fn do_create_distribution(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        body: &str,
        account_id: &str,
        with_tags: bool,
    ) -> MockResponse {
        // FIX(terraform-e2e): parse the FULL DistributionConfig from the request body so
        // that ALL fields (Restrictions, ViewerCertificate, HttpVersion, TrustedKeyGroups,
        // etc.) can be echoed back in GetDistribution/GetDistributionConfig responses.
        // Without this the Go provider panics with nil pointer dereferences when it tries
        // to access fields that are present in the plan but absent from the response.
        // We also use the parsed model to extract caller_reference/enabled/origins instead
        // of the raw XML text extraction, which was broken: extract_xml_value("Enabled")
        // was returning the FIRST <Enabled> found anywhere in the document, which was
        // <TrustedKeyGroups><Enabled>false</Enabled> instead of the distribution-level one.
        let raw_config: crate::model::DistributionConfig = if with_tags {
            match quick_xml::de::from_str::<crate::model::DistributionConfigWithTags>(body) {
                Ok(parsed) => parsed.distribution_config,
                Err(e) => {
                    return xml_error_response(
                        400,
                        "MalformedXML",
                        &format!("Failed to parse DistributionConfigWithTags: {e}"),
                    );
                }
            }
        } else {
            match quick_xml::de::from_str::<crate::model::DistributionConfig>(body) {
                Ok(parsed) => parsed,
                Err(e) => {
                    return xml_error_response(
                        400,
                        "MalformedXML",
                        &format!("Failed to parse DistributionConfig: {e}"),
                    );
                }
            }
        };

        let caller_reference = raw_config.caller_reference.clone();
        if caller_reference.is_empty() {
            return xml_error_response(400, "MissingParameter", "Missing CallerReference");
        }
        let enabled = raw_config.enabled;

        // Build the internal Origin/DefaultCacheBehavior types from the parsed model.
        // Origins: raw_config.origins is model::Origins { items: OriginList { items: Vec<model::Origin> } }
        let origins: Vec<Origin> = raw_config
            .origins
            .items // OriginList
            .items // Vec<model::Origin>
            .iter()
            .map(|o| Origin {
                id: o.id.clone(),
                domain_name: o.domain_name.clone(),
            })
            .collect();
        // AllowedMethods: model::AllowedMethods { items: MethodsList { items: Vec<String> }, cached_methods: Option<CachedMethods> }
        let (allowed_methods, cached_methods) =
            if let Some(ref am) = raw_config.default_cache_behavior.allowed_methods {
                let allowed: Vec<String> = am.items.items.clone();
                let cached: Vec<String> = am
                    .cached_methods
                    .as_ref()
                    .map(|cm| cm.items.items.clone())
                    .unwrap_or_default();
                (allowed, cached)
            } else {
                (vec![], vec![])
            };
        let (forwarded_values_query_string, forwarded_values_cookies_forward) =
            if let Some(ref fv) = raw_config.default_cache_behavior.forwarded_values {
                (fv.query_string, fv.cookies.forward.clone())
            } else {
                (false, "none".to_string())
            };
        let default_cache_behavior = DefaultCacheBehavior {
            target_origin_id: raw_config.default_cache_behavior.target_origin_id.clone(),
            viewer_protocol_policy: raw_config
                .default_cache_behavior
                .viewer_protocol_policy
                .clone(),
            allowed_methods,
            cached_methods,
            forwarded_values_query_string,
            forwarded_values_cookies_forward,
            compress: raw_config.default_cache_behavior.compress.unwrap_or(false),
        };

        let tags = if with_tags {
            parse_tags(body)
        } else {
            HashMap::new()
        };

        let mut state = state.write().await;
        match state.create_distribution(
            &caller_reference,
            origins,
            default_cache_behavior,
            account_id,
            enabled,
            tags,
            raw_config,
        ) {
            Ok(dist) => {
                let etag_val = dist.etag.clone();
                let wire_dist = dist_to_wire(dist);
                let mut resp = wire::serialize_create_distribution_with_tags_response(&wire_dist);
                resp.headers.insert(ETAG, etag_val.parse().unwrap());
                resp
            }
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_get_distribution(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_distribution(id) {
            Ok(dist) => {
                let etag_val = dist.etag.clone();
                let wire_dist = dist_to_wire(dist);
                let mut resp = wire::serialize_get_distribution_response(&wire_dist);
                resp.headers.insert(ETAG, etag_val.parse().unwrap());
                resp
            }
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_get_distribution_config(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_distribution_config(id) {
            Ok(dist) => {
                let etag_val = dist.etag.clone();
                let wire_config = dist_config_to_wire(dist);
                let mut resp = wire::serialize_get_distribution_config_response(&wire_config);
                resp.headers.insert(ETAG, etag_val.parse().unwrap());
                resp
            }
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_update_distribution(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        id: &str,
        body: &str,
        if_match: Option<&str>,
    ) -> MockResponse {
        // FIX(terraform-e2e): parse the full DistributionConfig so we can echo back
        // ALL fields (Restrictions, ViewerCertificate, etc.) without nil panics.
        let raw_config: crate::model::DistributionConfig =
            match quick_xml::de::from_str::<crate::model::DistributionConfig>(body) {
                Ok(parsed) => parsed,
                Err(e) => {
                    return xml_error_response(
                        400,
                        "MalformedXML",
                        &format!("Failed to parse DistributionConfig: {e}"),
                    );
                }
            };

        let caller_reference = raw_config.caller_reference.clone();
        if caller_reference.is_empty() {
            return xml_error_response(400, "MissingParameter", "Missing CallerReference");
        }
        let enabled = raw_config.enabled;

        let origins: Vec<Origin> = raw_config
            .origins
            .items // OriginList
            .items // Vec<model::Origin>
            .iter()
            .map(|o| Origin {
                id: o.id.clone(),
                domain_name: o.domain_name.clone(),
            })
            .collect();
        let (allowed_methods, cached_methods) =
            if let Some(ref am) = raw_config.default_cache_behavior.allowed_methods {
                let allowed: Vec<String> = am.items.items.clone();
                let cached: Vec<String> = am
                    .cached_methods
                    .as_ref()
                    .map(|cm| cm.items.items.clone())
                    .unwrap_or_default();
                (allowed, cached)
            } else {
                (vec![], vec![])
            };
        let (forwarded_values_query_string, forwarded_values_cookies_forward) =
            if let Some(ref fv) = raw_config.default_cache_behavior.forwarded_values {
                (fv.query_string, fv.cookies.forward.clone())
            } else {
                (false, "none".to_string())
            };
        let default_cache_behavior = DefaultCacheBehavior {
            target_origin_id: raw_config.default_cache_behavior.target_origin_id.clone(),
            viewer_protocol_policy: raw_config
                .default_cache_behavior
                .viewer_protocol_policy
                .clone(),
            allowed_methods,
            cached_methods,
            forwarded_values_query_string,
            forwarded_values_cookies_forward,
            compress: raw_config.default_cache_behavior.compress.unwrap_or(false),
        };

        let mut state = state.write().await;
        match state.update_distribution(
            id,
            &caller_reference,
            origins,
            default_cache_behavior,
            enabled,
            if_match,
            raw_config,
        ) {
            Ok(dist) => {
                let etag_val = dist.etag.clone();
                let wire_dist = dist_to_wire(dist);
                let mut resp = wire::serialize_update_distribution_response(&wire_dist);
                resp.headers.insert(ETAG, etag_val.parse().unwrap());
                resp
            }
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_delete_distribution(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        id: &str,
        _if_match: Option<&str>,
    ) -> MockResponse {
        // Note: moto does not validate etag on delete, so we skip etag validation here
        let mut state = state.write().await;
        match state.delete_distribution(id) {
            Ok(()) => MockResponse::xml(204, "".to_string()),
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_list_distributions(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let dists = state.list_distributions();

        let items: Vec<wire::DistributionSummary> =
            dists.iter().map(|d| dist_to_summary(d)).collect();

        let quantity = items.len() as i32;
        let wire_list = wire::DistributionList {
            quantity: Some(quantity),
            items: if items.is_empty() {
                None
            } else {
                Some(wire::DistributionSummaryList::from(items))
            },
            max_items: Some(100),
            is_truncated: Some(false),
            ..Default::default()
        };
        serialize_distribution_list(&wire_list)
    }

    // ---- Invalidation handlers ----

    async fn handle_create_invalidation(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        distribution_id: &str,
        body: &str,
    ) -> MockResponse {
        let caller_reference = extract_xml_value(body, "CallerReference").unwrap_or_default();
        if caller_reference.is_empty() {
            return xml_error_response(400, "MissingParameter", "Missing CallerReference");
        }
        let paths = parse_path_items(body);

        let mut state = state.write().await;
        match state.create_invalidation(distribution_id, &caller_reference, paths.clone()) {
            Ok(inv) => {
                let wire_inv = inv_to_wire(inv);
                wire::serialize_create_invalidation_response(&wire_inv)
            }
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_get_invalidation(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        distribution_id: &str,
        invalidation_id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_invalidation(distribution_id, invalidation_id) {
            Ok(inv) => {
                let wire_inv = inv_to_wire(inv);
                wire::serialize_get_invalidation_response(&wire_inv)
            }
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_list_invalidations(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        distribution_id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        if state.get_distribution(distribution_id).is_err() {
            return xml_error_response(404, "NoSuchDistribution", "Distribution not found");
        }
        let invs = state.list_invalidations(distribution_id);
        let items: Vec<wire::InvalidationSummary> = invs
            .iter()
            .map(|inv| wire::InvalidationSummary {
                id: Some(inv.id.clone()),
                create_time: Some(inv.create_time.to_rfc3339()),
                status: Some(inv.status.clone()),
                ..Default::default()
            })
            .collect();
        let quantity = items.len() as i32;
        let wire_list = wire::InvalidationList {
            quantity: Some(quantity),
            items: if items.is_empty() {
                None
            } else {
                Some(wire::InvalidationSummaryList::from(items))
            },
            max_items: Some(100),
            is_truncated: Some(false),
            ..Default::default()
        };
        serialize_invalidation_list(&wire_list)
    }

    // ---- Key Group handlers ----

    async fn handle_create_key_group(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        body: &str,
    ) -> MockResponse {
        let name = match extract_xml_value(body, "Name") {
            Some(n) => n,
            None => return xml_error_response(400, "MissingParameter", "Missing Name"),
        };
        let items = parse_key_group_items(body);
        let comment = extract_xml_value(body, "Comment");

        let mut state = state.write().await;
        match state.create_key_group(&name, items, comment) {
            Ok(kg) => {
                let etag_val = kg.etag.clone();
                let wire_kg = kg_to_wire(kg);
                let mut resp = wire::serialize_create_key_group_response(&wire_kg);
                resp.headers.insert(ETAG, etag_val.parse().unwrap());
                resp
            }
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_get_key_group(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_key_group(id) {
            Ok(kg) => {
                let etag_val = kg.etag.clone();
                let wire_kg = kg_to_wire(kg);
                let mut resp = wire::serialize_get_key_group_response(&wire_kg);
                resp.headers.insert(ETAG, etag_val.parse().unwrap());
                resp
            }
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_list_key_groups(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let kgs = state.list_key_groups();

        let items: Vec<wire::KeyGroupSummary> = kgs
            .iter()
            .map(|kg| wire::KeyGroupSummary {
                key_group: Some(kg_to_wire(kg)),
                ..Default::default()
            })
            .collect();
        let quantity = items.len() as i32;
        let wire_list = wire::KeyGroupList {
            quantity: Some(quantity),
            items: if items.is_empty() {
                None
            } else {
                Some(wire::KeyGroupSummaryList::from(items))
            },
            max_items: Some(100),
            ..Default::default()
        };
        serialize_key_group_list(&wire_list)
    }

    // ---- Origin Access Control handlers ----

    async fn handle_create_origin_access_control(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        body: &str,
    ) -> MockResponse {
        let name = match extract_xml_value(body, "Name") {
            Some(n) => n,
            None => return xml_error_response(400, "MissingParameter", "Missing Name"),
        };
        let description = extract_xml_value(body, "Description");
        let origin_type = extract_xml_value(body, "OriginAccessControlOriginType")
            .unwrap_or_else(|| "s3".to_string());
        let signing_behavior =
            extract_xml_value(body, "SigningBehavior").unwrap_or_else(|| "always".to_string());
        let signing_protocol =
            extract_xml_value(body, "SigningProtocol").unwrap_or_else(|| "sigv4".to_string());

        let mut state = state.write().await;
        match state.create_origin_access_control(
            &name,
            description,
            &origin_type,
            &signing_behavior,
            &signing_protocol,
        ) {
            Ok(oac) => {
                let etag_val = oac.etag.clone();
                let wire_oac = oac_to_wire(oac);
                let mut resp = wire::serialize_create_origin_access_control_response(&wire_oac);
                resp.headers.insert(ETAG, etag_val.parse().unwrap());
                resp
            }
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_get_origin_access_control(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_origin_access_control(id) {
            Ok(oac) => {
                let etag_val = oac.etag.clone();
                let wire_oac = oac_to_wire(oac);
                let mut resp = wire::serialize_get_origin_access_control_response(&wire_oac);
                resp.headers.insert(ETAG, etag_val.parse().unwrap());
                resp
            }
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_update_origin_access_control(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        id: &str,
        body: &str,
        if_match: Option<&str>,
    ) -> MockResponse {
        let name = match extract_xml_value(body, "Name") {
            Some(n) => n,
            None => return xml_error_response(400, "MissingParameter", "Missing Name"),
        };
        let description = extract_xml_value(body, "Description");
        let origin_type = extract_xml_value(body, "OriginAccessControlOriginType")
            .unwrap_or_else(|| "s3".to_string());
        let signing_behavior =
            extract_xml_value(body, "SigningBehavior").unwrap_or_else(|| "always".to_string());
        let signing_protocol =
            extract_xml_value(body, "SigningProtocol").unwrap_or_else(|| "sigv4".to_string());

        let mut state = state.write().await;
        match state.update_origin_access_control(
            id,
            &name,
            description,
            &origin_type,
            &signing_behavior,
            &signing_protocol,
            if_match,
        ) {
            Ok(oac) => {
                let etag_val = oac.etag.clone();
                let wire_oac = oac_to_wire(oac);
                let mut resp = wire::serialize_update_origin_access_control_response(&wire_oac);
                resp.headers.insert(ETAG, etag_val.parse().unwrap());
                resp
            }
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_delete_origin_access_control(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        id: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_origin_access_control(id) {
            Ok(()) => wire::serialize_delete_origin_access_control_response(),
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_list_origin_access_controls(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let oacs = state.list_origin_access_controls();

        let items: Vec<wire::OriginAccessControlSummary> = oacs
            .iter()
            .map(|oac| wire::OriginAccessControlSummary {
                id: Some(oac.id.clone()),
                name: Some(oac.name.clone()),
                description: oac.description.clone(),
                origin_access_control_origin_type: Some(
                    oac.origin_access_control_origin_type.clone(),
                ),
                signing_behavior: Some(oac.signing_behavior.clone()),
                signing_protocol: Some(oac.signing_protocol.clone()),
                ..Default::default()
            })
            .collect();
        let quantity = items.len() as i32;
        let wire_list = wire::OriginAccessControlList {
            quantity: Some(quantity),
            items: if items.is_empty() {
                None
            } else {
                Some(wire::OriginAccessControlSummaryList::from(items))
            },
            max_items: Some(100),
            is_truncated: Some(false),
            ..Default::default()
        };
        serialize_oac_list(&wire_list)
    }

    // ---- Public Key handlers ----

    async fn handle_create_public_key(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        body: &str,
    ) -> MockResponse {
        let caller_reference = match extract_xml_value(body, "CallerReference") {
            Some(cr) => cr,
            None => return xml_error_response(400, "MissingParameter", "Missing CallerReference"),
        };
        let name = match extract_xml_value(body, "Name") {
            Some(n) => n,
            None => return xml_error_response(400, "MissingParameter", "Missing Name"),
        };
        let encoded_key = match extract_xml_value(body, "EncodedKey") {
            Some(k) => k,
            None => return xml_error_response(400, "MissingParameter", "Missing EncodedKey"),
        };
        let comment = extract_xml_value(body, "Comment");

        let mut state = state.write().await;
        match state.create_public_key(&caller_reference, &name, &encoded_key, comment) {
            Ok(pk) => {
                let etag_val = pk.etag.clone();
                let wire_pk = pk_to_wire(pk);
                let mut resp = wire::serialize_create_public_key_response(&wire_pk);
                resp.headers.insert(ETAG, etag_val.parse().unwrap());
                resp
            }
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_get_public_key(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_public_key(id) {
            Ok(pk) => {
                let etag_val = pk.etag.clone();
                let wire_pk = pk_to_wire(pk);
                let mut resp = wire::serialize_get_public_key_response(&wire_pk);
                resp.headers.insert(ETAG, etag_val.parse().unwrap());
                resp
            }
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_delete_public_key(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        id: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_public_key(id) {
            Ok(()) => wire::serialize_delete_public_key_response(),
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_list_public_keys(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let pks = state.list_public_keys();

        let items: Vec<wire::PublicKeySummary> = pks
            .iter()
            .map(|pk| wire::PublicKeySummary {
                id: Some(pk.id.clone()),
                name: Some(pk.name.clone()),
                encoded_key: Some(pk.encoded_key.clone()),
                created_time: Some(pk.created_time.to_rfc3339()),
                comment: pk.comment.clone(),
                ..Default::default()
            })
            .collect();
        let quantity = items.len() as i32;
        let wire_list = wire::PublicKeyList {
            quantity: Some(quantity),
            items: if items.is_empty() {
                None
            } else {
                Some(wire::PublicKeySummaryList::from(items))
            },
            max_items: Some(100),
            ..Default::default()
        };
        serialize_public_key_list(&wire_list)
    }

    // ---- Tag handlers ----

    async fn handle_tag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        arn: &str,
        body: &str,
    ) -> MockResponse {
        let tags = parse_tags(body);
        let mut state = state.write().await;
        match state.tag_resource(arn, tags) {
            Ok(()) => wire::serialize_tag_resource_response(),
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_untag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        arn: &str,
        body: &str,
    ) -> MockResponse {
        let tag_keys = parse_tag_keys(body);
        let mut state = state.write().await;
        match state.untag_resource(arn, &tag_keys) {
            Ok(()) => wire::serialize_untag_resource_response(),
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_list_tags_for_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        arn: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.list_tags_for_resource(arn) {
            Ok(tags) => {
                let tag_xml: String = tags
                    .iter()
                    .map(|(k, v)| {
                        format!(
                            "<Tag><Key>{}</Key><Value>{}</Value></Tag>",
                            xml_escape(k),
                            xml_escape(v),
                        )
                    })
                    .collect::<Vec<_>>()
                    .join("");
                let body = format!("<Tags><Items>{}</Items></Tags>", tag_xml,);
                MockResponse::xml(200, body)
            }
            Err(e) => cloudfront_error_response(&e),
        }
    }

    // ---- Formerly-stub handlers, now state-backed ----

    async fn handle_associate_alias(
        &self,
        _state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        _id: &str,
        _query_params: &HashMap<String, String>,
    ) -> MockResponse {
        // Alias association is a no-op in the mock — just return success.
        wire::serialize_associate_alias_response()
    }

    async fn handle_associate_distribution_tenant_web_acl(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        id: &str,
        body: &str,
    ) -> MockResponse {
        let web_acl_arn = extract_xml_value(body, "WebACLArn").unwrap_or_default();
        let mut state = state.write().await;
        state.associate_web_acl(id, &web_acl_arn);
        let result = wire::AssociateDistributionTenantWebACLResult {
            id: Some(id.to_string()),
            web_a_c_l_arn: Some(web_acl_arn),
        };
        wire::serialize_associate_distribution_tenant_web_a_c_l_response(&result)
    }

    async fn handle_associate_distribution_web_acl(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        id: &str,
        body: &str,
    ) -> MockResponse {
        let web_acl_arn = extract_xml_value(body, "WebACLArn").unwrap_or_default();
        let mut state = state.write().await;
        state.associate_web_acl(id, &web_acl_arn);
        let result = wire::AssociateDistributionWebACLResult {
            id: Some(id.to_string()),
            web_a_c_l_arn: Some(web_acl_arn),
        };
        wire::serialize_associate_distribution_web_a_c_l_response(&result)
    }

    async fn handle_copy_distribution(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        id: &str,
    ) -> MockResponse {
        // Copy the source distribution's config, then create a new distribution.
        let (raw_config, origins, default_cache_behavior, enabled) = {
            let state = state.read().await;
            match state.get_distribution(id) {
                Ok(dist) => (
                    dist.raw_config.clone(),
                    dist.origins.clone(),
                    dist.default_cache_behavior.clone(),
                    dist.enabled,
                ),
                Err(e) => return cloudfront_error_response(&e),
            }
        };

        // Generate a unique caller reference for the copy.
        let caller_reference = uuid::Uuid::new_v4().to_string();
        let account_id = DEFAULT_ACCOUNT_ID;

        let mut state = state.write().await;
        match state.create_distribution(
            &caller_reference,
            origins,
            default_cache_behavior,
            account_id,
            enabled,
            HashMap::new(),
            raw_config,
        ) {
            Ok(dist) => {
                let etag_val = dist.etag.clone();
                let wire_dist = dist_to_wire(dist);
                let mut resp = wire::serialize_copy_distribution_response(&wire_dist);
                resp.headers.insert(ETAG, etag_val.parse().unwrap());
                resp
            }
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_create_anycast_ip_list(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        body: &str,
        account_id: &str,
    ) -> MockResponse {
        let req = match quick_xml::de::from_str::<crate::model::CreateAnycastIpListRequest>(body) {
            Ok(r) => r,
            Err(e) => {
                return xml_error_response(
                    400,
                    "MalformedXML",
                    &format!("Failed to parse CreateAnycastIpListRequest: {e}"),
                );
            }
        };
        let mut state = state.write().await;
        let list = state.create_anycast_ip_list(
            &req.name,
            req.ip_count,
            req.ip_address_type.clone(),
            account_id,
        );
        let etag_val = list.etag.clone();
        let wire_list = anycast_to_wire(list);
        let mut resp = wire::serialize_create_anycast_ip_list_response(&wire_list);
        resp.headers.insert(ETAG, etag_val.parse().unwrap());
        resp
    }

    async fn handle_create_cache_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        body: &str,
    ) -> MockResponse {
        let config = match quick_xml::de::from_str::<crate::model::CachePolicyConfig>(body) {
            Ok(c) => c,
            Err(e) => {
                return xml_error_response(
                    400,
                    "MalformedXML",
                    &format!("Failed to parse CachePolicyConfig: {e}"),
                );
            }
        };
        let mut state = state.write().await;
        let cp = state.create_cache_policy(config);
        let etag_val = cp.etag.clone();
        let wire_cp = wire::CachePolicy {
            id: Some(cp.id.clone()),
            last_modified_time: Some(cp.last_modified_time.to_rfc3339()),
            cache_policy_config: Some(cp.config.clone()),
        };
        let mut resp = wire::serialize_create_cache_policy_response(&wire_cp);
        resp.headers.insert(ETAG, etag_val.parse().unwrap());
        resp
    }

    async fn handle_create_cloud_front_origin_access_identity(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        body: &str,
    ) -> MockResponse {
        let caller_reference = match extract_xml_value(body, "CallerReference") {
            Some(cr) => cr,
            None => return xml_error_response(400, "MissingParameter", "Missing CallerReference"),
        };
        let comment = extract_xml_value(body, "Comment").unwrap_or_default();
        let mut state = state.write().await;
        match state.create_oai(&caller_reference, &comment) {
            Ok(oai) => {
                let etag_val = oai.etag.clone();
                let wire_oai = oai_to_wire(oai);
                let mut resp =
                    wire::serialize_create_cloud_front_origin_access_identity_response(&wire_oai);
                resp.headers.insert(ETAG, etag_val.parse().unwrap());
                resp
            }
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_create_connection_function(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        body: &str,
        account_id: &str,
    ) -> MockResponse {
        let req =
            match quick_xml::de::from_str::<crate::model::CreateConnectionFunctionRequest>(body) {
                Ok(r) => r,
                Err(e) => {
                    return xml_error_response(
                        400,
                        "MalformedXML",
                        &format!("Failed to parse CreateConnectionFunctionRequest: {e}"),
                    );
                }
            };
        let mut state = state.write().await;
        let func = state.create_connection_function(
            &req.name,
            req.connection_function_config,
            &req.connection_function_code,
            account_id,
        );
        let etag_val = func.etag.clone();
        let result = conn_func_to_wire(func);
        let mut resp = wire::serialize_create_connection_function_response(&result);
        resp.headers.insert(ETAG, etag_val.parse().unwrap());
        resp
    }

    async fn handle_create_connection_group(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        body: &str,
        account_id: &str,
    ) -> MockResponse {
        let req = match quick_xml::de::from_str::<crate::model::CreateConnectionGroupRequest>(body)
        {
            Ok(r) => r,
            Err(e) => {
                return xml_error_response(
                    400,
                    "MalformedXML",
                    &format!("Failed to parse CreateConnectionGroupRequest: {e}"),
                );
            }
        };
        let mut state = state.write().await;
        let cg = state.create_connection_group(
            &req.name,
            req.enabled.unwrap_or(true),
            req.ipv6_enabled.unwrap_or(false),
            req.anycast_ip_list_id,
            account_id,
        );
        let etag_val = cg.etag.clone();
        let result = conn_group_to_wire(cg);
        let mut resp = wire::serialize_create_connection_group_response(&result);
        resp.headers.insert(ETAG, etag_val.parse().unwrap());
        resp
    }

    async fn handle_create_continuous_deployment_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        body: &str,
    ) -> MockResponse {
        let req = match quick_xml::de::from_str::<
            crate::model::CreateContinuousDeploymentPolicyRequest,
        >(body)
        {
            Ok(r) => r,
            Err(e) => {
                return xml_error_response(
                    400,
                    "MalformedXML",
                    &format!("Failed to parse CreateContinuousDeploymentPolicyRequest: {e}"),
                );
            }
        };
        let mut state = state.write().await;
        let cdp =
            state.create_continuous_deployment_policy(req.continuous_deployment_policy_config);
        let etag_val = cdp.etag.clone();
        let result = wire::ContinuousDeploymentPolicy {
            id: Some(cdp.id.clone()),
            last_modified_time: Some(cdp.last_modified_time.to_rfc3339()),
            continuous_deployment_policy_config: Some(cdp.config.clone()),
        };
        let mut resp = wire::serialize_create_continuous_deployment_policy_response(&result);
        resp.headers.insert(ETAG, etag_val.parse().unwrap());
        resp
    }

    async fn handle_create_distribution_tenant(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        body: &str,
        account_id: &str,
    ) -> MockResponse {
        let req =
            match quick_xml::de::from_str::<crate::model::CreateDistributionTenantRequest>(body) {
                Ok(r) => r,
                Err(e) => {
                    return xml_error_response(
                        400,
                        "MalformedXML",
                        &format!("Failed to parse CreateDistributionTenantRequest: {e}"),
                    );
                }
            };
        // Convert DomainList to DomainResultList
        let domains = crate::model::DomainResultList {
            items: req
                .domains
                .items
                .iter()
                .map(|d| crate::model::DomainResult {
                    domain: Some(d.domain.clone()),
                    status: Some("active".to_string()),
                })
                .collect(),
        };
        let mut state = state.write().await;
        let dt = state.create_distribution_tenant(
            &req.name,
            &req.distribution_id,
            req.connection_group_id,
            req.enabled.unwrap_or(true),
            domains,
            req.customizations,
            req.parameters,
            account_id,
        );
        let etag_val = dt.etag.clone();
        let result = dist_tenant_to_wire(dt);
        let mut resp = wire::serialize_create_distribution_tenant_response(&result);
        resp.headers.insert(ETAG, etag_val.parse().unwrap());
        resp
    }

    async fn handle_create_field_level_encryption_config(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        body: &str,
    ) -> MockResponse {
        let config = match quick_xml::de::from_str::<crate::model::FieldLevelEncryptionConfig>(body)
        {
            Ok(c) => c,
            Err(e) => {
                return xml_error_response(
                    400,
                    "MalformedXML",
                    &format!("Failed to parse FieldLevelEncryptionConfig: {e}"),
                );
            }
        };
        let mut state = state.write().await;
        let fle = state.create_field_level_encryption_config(config);
        let etag_val = fle.etag.clone();
        let wire_fle = wire::FieldLevelEncryption {
            id: Some(fle.id.clone()),
            last_modified_time: Some(fle.last_modified_time.to_rfc3339()),
            field_level_encryption_config: Some(fle.config.clone()),
        };
        let mut resp = wire::serialize_create_field_level_encryption_config_response(&wire_fle);
        resp.headers.insert(ETAG, etag_val.parse().unwrap());
        resp
    }

    async fn handle_create_field_level_encryption_profile(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        body: &str,
    ) -> MockResponse {
        let config = match quick_xml::de::from_str::<crate::model::FieldLevelEncryptionProfileConfig>(
            body,
        ) {
            Ok(c) => c,
            Err(e) => {
                return xml_error_response(
                    400,
                    "MalformedXML",
                    &format!("Failed to parse FieldLevelEncryptionProfileConfig: {e}"),
                );
            }
        };
        let mut state = state.write().await;
        let profile = state.create_field_level_encryption_profile(config);
        let etag_val = profile.etag.clone();
        let wire_profile = wire::FieldLevelEncryptionProfile {
            id: Some(profile.id.clone()),
            last_modified_time: Some(profile.last_modified_time.to_rfc3339()),
            field_level_encryption_profile_config: Some(profile.config.clone()),
        };
        let mut resp =
            wire::serialize_create_field_level_encryption_profile_response(&wire_profile);
        resp.headers.insert(ETAG, etag_val.parse().unwrap());
        resp
    }

    async fn handle_create_function(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        body: &str,
        account_id: &str,
    ) -> MockResponse {
        let config =
            quick_xml::de::from_str::<crate::model::FunctionConfig>(body).unwrap_or_default();
        let name = match extract_xml_value(body, "Name") {
            Some(n) => n,
            None => return xml_error_response(400, "MissingParameter", "Missing Name"),
        };
        // Function code is sent as raw bytes in the request body alongside XML metadata.
        // For simplicity, store the entire body as code bytes.
        let code = body.as_bytes().to_vec();
        let mut state = state.write().await;
        let func = state.create_function(&name, config, code, account_id);
        let etag_val = func.etag.clone();
        let wire_func = func_to_wire(func);
        let mut resp = wire::serialize_create_function_response(&wire_func);
        resp.headers.insert(ETAG, etag_val.parse().unwrap());
        resp
    }

    async fn handle_create_invalidation_for_distribution_tenant(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        tenant_id: &str,
        body: &str,
    ) -> MockResponse {
        let caller_reference = extract_xml_value(body, "CallerReference").unwrap_or_default();
        let paths: Vec<String> = extract_xml_list_values(body, "Path");
        let mut state = state.write().await;
        match state.create_invalidation_for_distribution_tenant(tenant_id, &caller_reference, paths)
        {
            Ok(inv) => {
                let path_list = wire::PathList::from(inv.paths.clone());
                let wire_paths = wire::Paths {
                    quantity: inv.paths.len() as i32,
                    items: Some(path_list),
                };
                let wire_batch = wire::InvalidationBatch {
                    caller_reference: inv.caller_reference.clone(),
                    paths: wire_paths,
                };
                let result = wire::Invalidation {
                    id: Some(inv.id.clone()),
                    status: Some(inv.status.clone()),
                    create_time: Some(inv.create_time.to_rfc3339()),
                    invalidation_batch: Some(wire_batch),
                };
                wire::serialize_create_invalidation_for_distribution_tenant_response(&result)
            }
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_create_key_value_store(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        body: &str,
        account_id: &str,
    ) -> MockResponse {
        let req = match quick_xml::de::from_str::<crate::model::CreateKeyValueStoreRequest>(body) {
            Ok(r) => r,
            Err(e) => {
                return xml_error_response(
                    400,
                    "MalformedXML",
                    &format!("Failed to parse CreateKeyValueStoreRequest: {e}"),
                );
            }
        };
        let mut state = state.write().await;
        match state.create_key_value_store(&req.name, req.comment, account_id) {
            Ok(kvs) => {
                let etag_val = kvs.etag.clone();
                let wire_kvs = kvs_to_wire(kvs);
                let mut resp = wire::serialize_create_key_value_store_response(&wire_kvs);
                resp.headers.insert(ETAG, etag_val.parse().unwrap());
                resp
            }
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_create_monitoring_subscription(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        distribution_id: &str,
        body: &str,
    ) -> MockResponse {
        let status = extract_xml_value(body, "RealtimeMetricsSubscriptionStatus")
            .unwrap_or_else(|| "Enabled".to_string());
        let mut state = state.write().await;
        match state.create_monitoring_subscription(distribution_id, &status) {
            Ok(ms) => {
                let result = wire::MonitoringSubscription {
                    realtime_metrics_subscription_config: Some(
                        wire::RealtimeMetricsSubscriptionConfig {
                            realtime_metrics_subscription_status: ms
                                .realtime_metrics_subscription_status
                                .clone(),
                        },
                    ),
                };
                wire::serialize_create_monitoring_subscription_response(&result)
            }
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_create_origin_request_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        body: &str,
    ) -> MockResponse {
        let config = match quick_xml::de::from_str::<crate::model::OriginRequestPolicyConfig>(body)
        {
            Ok(c) => c,
            Err(e) => {
                return xml_error_response(
                    400,
                    "MalformedXML",
                    &format!("Failed to parse OriginRequestPolicyConfig: {e}"),
                );
            }
        };
        let mut state = state.write().await;
        let orp = state.create_origin_request_policy(config);
        let etag_val = orp.etag.clone();
        let wire_orp = wire::OriginRequestPolicy {
            id: Some(orp.id.clone()),
            last_modified_time: Some(orp.last_modified_time.to_rfc3339()),
            origin_request_policy_config: Some(orp.config.clone()),
        };
        let mut resp = wire::serialize_create_origin_request_policy_response(&wire_orp);
        resp.headers.insert(ETAG, etag_val.parse().unwrap());
        resp
    }

    async fn handle_create_realtime_log_config(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        body: &str,
        account_id: &str,
    ) -> MockResponse {
        let req =
            match quick_xml::de::from_str::<crate::model::CreateRealtimeLogConfigRequest>(body) {
                Ok(r) => r,
                Err(e) => {
                    return xml_error_response(
                        400,
                        "MalformedXML",
                        &format!("Failed to parse CreateRealtimeLogConfigRequest: {e}"),
                    );
                }
            };
        let mut state = state.write().await;
        match state.create_realtime_log_config(
            &req.name,
            req.sampling_rate,
            req.end_points.items,
            req.fields.items,
            account_id,
        ) {
            Ok(cfg) => {
                let wire_cfg = rtlc_to_wire(cfg);
                let result = wire::CreateRealtimeLogConfigResult {
                    realtime_log_config: Some(wire_cfg),
                };
                wire::serialize_create_realtime_log_config_response(&result)
            }
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_create_response_headers_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        body: &str,
    ) -> MockResponse {
        let config =
            match quick_xml::de::from_str::<crate::model::ResponseHeadersPolicyConfig>(body) {
                Ok(c) => c,
                Err(e) => {
                    return xml_error_response(
                        400,
                        "MalformedXML",
                        &format!("Failed to parse ResponseHeadersPolicyConfig: {e}"),
                    );
                }
            };
        let mut state = state.write().await;
        let rhp = state.create_response_headers_policy(config);
        let etag_val = rhp.etag.clone();
        let wire_rhp = wire::ResponseHeadersPolicy {
            id: Some(rhp.id.clone()),
            last_modified_time: Some(rhp.last_modified_time.to_rfc3339()),
            response_headers_policy_config: Some(rhp.config.clone()),
        };
        let mut resp = wire::serialize_create_response_headers_policy_response(&wire_rhp);
        resp.headers.insert(ETAG, etag_val.parse().unwrap());
        resp
    }

    async fn handle_create_streaming_distribution(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        body: &str,
        account_id: &str,
    ) -> MockResponse {
        let config =
            match quick_xml::de::from_str::<crate::model::StreamingDistributionConfig>(body) {
                Ok(c) => c,
                Err(e) => {
                    return xml_error_response(
                        400,
                        "MalformedXML",
                        &format!("Failed to parse StreamingDistributionConfig: {e}"),
                    );
                }
            };
        let mut state = state.write().await;
        match state.create_streaming_distribution(config, account_id) {
            Ok(sd) => {
                let etag_val = sd.etag.clone();
                let wire_sd = sd_to_wire(sd);
                let mut resp = wire::serialize_create_streaming_distribution_response(&wire_sd);
                resp.headers.insert(ETAG, etag_val.parse().unwrap());
                resp
            }
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_create_streaming_distribution_with_tags(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        body: &str,
        account_id: &str,
    ) -> MockResponse {
        let config =
            match quick_xml::de::from_str::<crate::model::StreamingDistributionConfig>(body) {
                Ok(c) => c,
                Err(e) => {
                    return xml_error_response(
                        400,
                        "MalformedXML",
                        &format!("Failed to parse StreamingDistributionConfig: {e}"),
                    );
                }
            };
        let mut state = state.write().await;
        match state.create_streaming_distribution(config, account_id) {
            Ok(sd) => {
                let etag_val = sd.etag.clone();
                let wire_sd = sd_to_wire(sd);
                let mut resp =
                    wire::serialize_create_streaming_distribution_with_tags_response(&wire_sd);
                resp.headers.insert(ETAG, etag_val.parse().unwrap());
                resp
            }
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_create_trust_store(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        body: &str,
        account_id: &str,
    ) -> MockResponse {
        let req = match quick_xml::de::from_str::<crate::model::CreateTrustStoreRequest>(body) {
            Ok(r) => r,
            Err(e) => {
                return xml_error_response(
                    400,
                    "MalformedXML",
                    &format!("Failed to parse CreateTrustStoreRequest: {e}"),
                );
            }
        };
        let mut state = state.write().await;
        match state.create_trust_store(&req.name, account_id) {
            Ok(ts) => {
                let etag_val = ts.etag.clone();
                let wire_ts = ts_to_wire(ts);
                let mut resp = wire::serialize_create_trust_store_response(&wire_ts);
                resp.headers.insert(ETAG, etag_val.parse().unwrap());
                resp
            }
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_create_vpc_origin(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        body: &str,
        account_id: &str,
    ) -> MockResponse {
        let req = match quick_xml::de::from_str::<crate::model::CreateVpcOriginRequest>(body) {
            Ok(r) => r,
            Err(e) => {
                return xml_error_response(
                    400,
                    "MalformedXML",
                    &format!("Failed to parse CreateVpcOriginRequest: {e}"),
                );
            }
        };
        let mut state = state.write().await;
        let vpc = state.create_vpc_origin(req.vpc_origin_endpoint_config, account_id);
        let etag_val = vpc.etag.clone();
        let wire_vpc = vpc_to_wire(vpc);
        let mut resp = wire::serialize_create_vpc_origin_response(&wire_vpc);
        resp.headers.insert(ETAG, etag_val.parse().unwrap());
        resp
    }

    async fn handle_delete_anycast_ip_list(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        id: &str,
        if_match: Option<&str>,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_anycast_ip_list(id, if_match) {
            Ok(()) => wire::serialize_delete_anycast_ip_list_response(),
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_delete_cache_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        id: &str,
        if_match: Option<&str>,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_cache_policy(id, if_match) {
            Ok(()) => wire::serialize_delete_cache_policy_response(),
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_delete_cloud_front_origin_access_identity(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        id: &str,
        if_match: Option<&str>,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_oai(id, if_match) {
            Ok(()) => wire::serialize_delete_cloud_front_origin_access_identity_response(),
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_delete_connection_function(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        id: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_connection_function(id) {
            Ok(()) => wire::serialize_delete_connection_function_response(),
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_delete_connection_group(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        id: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_connection_group(id) {
            Ok(()) => wire::serialize_delete_connection_group_response(),
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_delete_continuous_deployment_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        id: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_continuous_deployment_policy(id) {
            Ok(()) => wire::serialize_delete_continuous_deployment_policy_response(),
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_delete_distribution_tenant(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        id: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_distribution_tenant(id) {
            Ok(()) => wire::serialize_delete_distribution_tenant_response(),
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_delete_field_level_encryption_config(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        id: &str,
        if_match: Option<&str>,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_field_level_encryption_config(id, if_match) {
            Ok(()) => wire::serialize_delete_field_level_encryption_config_response(),
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_delete_field_level_encryption_profile(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        id: &str,
        if_match: Option<&str>,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_field_level_encryption_profile(id, if_match) {
            Ok(()) => wire::serialize_delete_field_level_encryption_profile_response(),
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_delete_function(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        name: &str,
        if_match: Option<&str>,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_function(name, if_match) {
            Ok(()) => wire::serialize_delete_function_response(),
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_delete_key_group(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        id: &str,
        if_match: Option<&str>,
    ) -> MockResponse {
        let _ = if_match;
        let mut state = state.write().await;
        match state.delete_key_group(id) {
            Ok(()) => wire::serialize_delete_key_group_response(),
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_delete_key_value_store(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        name: &str,
        if_match: Option<&str>,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_key_value_store(name, if_match) {
            Ok(()) => wire::serialize_delete_key_value_store_response(),
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_delete_monitoring_subscription(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        distribution_id: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_monitoring_subscription(distribution_id) {
            Ok(()) => wire::serialize_delete_monitoring_subscription_response(),
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_delete_origin_request_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        id: &str,
        if_match: Option<&str>,
    ) -> MockResponse {
        let _ = if_match; // ORP delete doesn't use if-match in current implementation
        let mut state = state.write().await;
        match state.delete_origin_request_policy(id) {
            Ok(()) => wire::serialize_delete_origin_request_policy_response(),
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_delete_realtime_log_config(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        body: &str,
    ) -> MockResponse {
        let name = extract_xml_value(body, "Name");
        let arn = extract_xml_value(body, "ARN");
        let mut state = state.write().await;
        match state.delete_realtime_log_config(name.as_deref(), arn.as_deref()) {
            Ok(()) => wire::serialize_delete_realtime_log_config_response(),
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_delete_resource_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        body: &str,
    ) -> MockResponse {
        let resource_arn = extract_xml_value(body, "ResourceArn").unwrap_or_default();
        let mut state = state.write().await;
        match state.delete_resource_policy(&resource_arn) {
            Ok(()) => wire::serialize_delete_resource_policy_response(),
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_delete_response_headers_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        id: &str,
        if_match: Option<&str>,
    ) -> MockResponse {
        let _ = if_match; // RHP delete doesn't use if-match in current implementation
        let mut state = state.write().await;
        match state.delete_response_headers_policy(id) {
            Ok(()) => wire::serialize_delete_response_headers_policy_response(),
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_delete_streaming_distribution(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        id: &str,
        if_match: Option<&str>,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_streaming_distribution(id, if_match) {
            Ok(()) => wire::serialize_delete_streaming_distribution_response(),
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_delete_trust_store(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        id: &str,
        if_match: Option<&str>,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_trust_store(id, if_match) {
            Ok(()) => wire::serialize_delete_trust_store_response(),
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_delete_vpc_origin(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        id: &str,
        if_match: Option<&str>,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_vpc_origin(id, if_match) {
            Ok(vpc) => {
                let wire_vpc = vpc_to_wire(&vpc);
                wire::serialize_delete_vpc_origin_response(&wire_vpc)
            }
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_describe_connection_function(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_connection_function(id) {
            Ok(func) => {
                let etag_val = func.etag.clone();
                let result = conn_func_to_wire(func);
                let mut resp = wire::serialize_describe_connection_function_response(&result);
                resp.headers.insert(ETAG, etag_val.parse().unwrap());
                resp
            }
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_describe_function(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        name: &str,
        if_match: Option<&str>,
    ) -> MockResponse {
        let _ = if_match;
        let state = state.read().await;
        match state.get_function(name) {
            Ok(func) => {
                let etag_val = func.etag.clone();
                let wire_func = func_to_wire(func);
                let mut resp = wire::serialize_describe_function_response(&wire_func);
                resp.headers.insert(ETAG, etag_val.parse().unwrap());
                resp
            }
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_describe_key_value_store(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        name: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_key_value_store(name) {
            Ok(kvs) => {
                let etag_val = kvs.etag.clone();
                let wire_kvs = kvs_to_wire(kvs);
                let mut resp = wire::serialize_describe_key_value_store_response(&wire_kvs);
                resp.headers.insert(ETAG, etag_val.parse().unwrap());
                resp
            }
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_disassociate_distribution_tenant_web_acl(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        id: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        state.disassociate_web_acl(id);
        let result = wire::DisassociateDistributionTenantWebACLResult {
            id: Some(id.to_string()),
            ..Default::default()
        };
        wire::serialize_disassociate_distribution_tenant_web_a_c_l_response(&result)
    }

    async fn handle_disassociate_distribution_web_acl(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        id: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        state.disassociate_web_acl(id);
        let result = wire::DisassociateDistributionWebACLResult {
            id: Some(id.to_string()),
            ..Default::default()
        };
        wire::serialize_disassociate_distribution_web_a_c_l_response(&result)
    }

    async fn handle_get_anycast_ip_list(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_anycast_ip_list(id) {
            Ok(list) => {
                let etag_val = list.etag.clone();
                let wire_list = anycast_to_wire(list);
                let mut resp = wire::serialize_get_anycast_ip_list_response(&wire_list);
                resp.headers.insert(ETAG, etag_val.parse().unwrap());
                resp
            }
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_get_cache_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_cache_policy(id) {
            Ok(cp) => {
                let etag_val = cp.etag.clone();
                let wire_cp = wire::CachePolicy {
                    id: Some(cp.id.clone()),
                    last_modified_time: Some(cp.last_modified_time.to_rfc3339()),
                    cache_policy_config: Some(cp.config.clone()),
                };
                let mut resp = wire::serialize_get_cache_policy_response(&wire_cp);
                resp.headers.insert(ETAG, etag_val.parse().unwrap());
                resp
            }
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_get_cache_policy_config(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_cache_policy(id) {
            Ok(cp) => {
                let etag_val = cp.etag.clone();
                let wire_config = cp.config.clone();
                let mut resp = wire::serialize_get_cache_policy_config_response(&wire_config);
                resp.headers.insert(ETAG, etag_val.parse().unwrap());
                resp
            }
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_get_cloud_front_origin_access_identity(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_oai(id) {
            Ok(oai) => {
                let etag_val = oai.etag.clone();
                let wire_oai = oai_to_wire(oai);
                let mut resp =
                    wire::serialize_get_cloud_front_origin_access_identity_response(&wire_oai);
                resp.headers.insert(ETAG, etag_val.parse().unwrap());
                resp
            }
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_get_cloud_front_origin_access_identity_config(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_oai(id) {
            Ok(oai) => {
                let etag_val = oai.etag.clone();
                let wire_config = wire::CloudFrontOriginAccessIdentityConfig {
                    caller_reference: oai.caller_reference.clone(),
                    comment: oai.comment.clone(),
                };
                let mut resp =
                    wire::serialize_get_cloud_front_origin_access_identity_config_response(
                        &wire_config,
                    );
                resp.headers.insert(ETAG, etag_val.parse().unwrap());
                resp
            }
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_get_connection_function(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_connection_function(id) {
            Ok(func) => {
                let etag_val = func.etag.clone();
                let _result = conn_func_to_wire(func);
                let mut resp = wire::serialize_get_connection_function_response();
                resp.headers.insert(ETAG, etag_val.parse().unwrap());
                resp
            }
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_get_connection_group(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_connection_group(id) {
            Ok(cg) => {
                let etag_val = cg.etag.clone();
                let result = conn_group_to_wire(cg);
                let mut resp = wire::serialize_get_connection_group_response(&result);
                resp.headers.insert(ETAG, etag_val.parse().unwrap());
                resp
            }
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_get_connection_group_by_routing_endpoint(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        endpoint: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_connection_group_by_routing_endpoint(endpoint) {
            Ok(cg) => {
                let etag_val = cg.etag.clone();
                let result = conn_group_to_wire(cg);
                let mut resp =
                    wire::serialize_get_connection_group_by_routing_endpoint_response(&result);
                resp.headers.insert(ETAG, etag_val.parse().unwrap());
                resp
            }
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_get_continuous_deployment_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_continuous_deployment_policy(id) {
            Ok(cdp) => {
                let etag_val = cdp.etag.clone();
                let result = wire::ContinuousDeploymentPolicy {
                    id: Some(cdp.id.clone()),
                    last_modified_time: Some(cdp.last_modified_time.to_rfc3339()),
                    continuous_deployment_policy_config: Some(cdp.config.clone()),
                };
                let mut resp = wire::serialize_get_continuous_deployment_policy_response(&result);
                resp.headers.insert(ETAG, etag_val.parse().unwrap());
                resp
            }
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_get_continuous_deployment_policy_config(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_continuous_deployment_policy(id) {
            Ok(cdp) => {
                let etag_val = cdp.etag.clone();
                let result = cdp.config.clone();
                let mut resp =
                    wire::serialize_get_continuous_deployment_policy_config_response(&result);
                resp.headers.insert(ETAG, etag_val.parse().unwrap());
                resp
            }
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_get_distribution_tenant(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_distribution_tenant(id) {
            Ok(dt) => {
                let etag_val = dt.etag.clone();
                let result = dist_tenant_to_wire(dt);
                let mut resp = wire::serialize_get_distribution_tenant_response(&result);
                resp.headers.insert(ETAG, etag_val.parse().unwrap());
                resp
            }
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_get_distribution_tenant_by_domain(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        domain: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_distribution_tenant_by_domain(domain) {
            Ok(dt) => {
                let etag_val = dt.etag.clone();
                let result = dist_tenant_to_wire(dt);
                let mut resp = wire::serialize_get_distribution_tenant_by_domain_response(&result);
                resp.headers.insert(ETAG, etag_val.parse().unwrap());
                resp
            }
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_get_field_level_encryption(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_field_level_encryption_config(id) {
            Ok(fle) => {
                let etag_val = fle.etag.clone();
                let wire_fle = wire::FieldLevelEncryption {
                    id: Some(fle.id.clone()),
                    last_modified_time: Some(fle.last_modified_time.to_rfc3339()),
                    field_level_encryption_config: Some(fle.config.clone()),
                };
                let mut resp = wire::serialize_get_field_level_encryption_response(&wire_fle);
                resp.headers.insert(ETAG, etag_val.parse().unwrap());
                resp
            }
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_get_field_level_encryption_config(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_field_level_encryption_config(id) {
            Ok(fle) => {
                let etag_val = fle.etag.clone();
                let wire_config = fle.config.clone();
                let mut resp =
                    wire::serialize_get_field_level_encryption_config_response(&wire_config);
                resp.headers.insert(ETAG, etag_val.parse().unwrap());
                resp
            }
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_get_field_level_encryption_profile(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_field_level_encryption_profile(id) {
            Ok(profile) => {
                let etag_val = profile.etag.clone();
                let wire_profile = wire::FieldLevelEncryptionProfile {
                    id: Some(profile.id.clone()),
                    last_modified_time: Some(profile.last_modified_time.to_rfc3339()),
                    field_level_encryption_profile_config: Some(profile.config.clone()),
                };
                let mut resp =
                    wire::serialize_get_field_level_encryption_profile_response(&wire_profile);
                resp.headers.insert(ETAG, etag_val.parse().unwrap());
                resp
            }
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_get_field_level_encryption_profile_config(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_field_level_encryption_profile(id) {
            Ok(profile) => {
                let etag_val = profile.etag.clone();
                let wire_config = profile.config.clone();
                let mut resp = wire::serialize_get_field_level_encryption_profile_config_response(
                    &wire_config,
                );
                resp.headers.insert(ETAG, etag_val.parse().unwrap());
                resp
            }
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_get_function(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        name: &str,
        if_match: Option<&str>,
    ) -> MockResponse {
        let _ = if_match;
        let state = state.read().await;
        match state.get_function(name) {
            Ok(func) => {
                let etag_val = func.etag.clone();
                // GetFunction returns the function code as the response body
                let code = bytes::Bytes::from(func.code.clone());
                let mut resp = wire::serialize_get_function_response();
                resp.body = code;
                resp.headers.insert(ETAG, etag_val.parse().unwrap());
                resp
            }
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_get_invalidation_for_distribution_tenant(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        tenant_id: &str,
        inv_id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_invalidation_for_distribution_tenant(tenant_id, inv_id) {
            Ok(inv) => {
                let path_list = wire::PathList::from(inv.paths.clone());
                let wire_paths = wire::Paths {
                    quantity: inv.paths.len() as i32,
                    items: Some(path_list),
                };
                let wire_batch = wire::InvalidationBatch {
                    caller_reference: inv.caller_reference.clone(),
                    paths: wire_paths,
                };
                let result = wire::Invalidation {
                    id: Some(inv.id.clone()),
                    status: Some(inv.status.clone()),
                    create_time: Some(inv.create_time.to_rfc3339()),
                    invalidation_batch: Some(wire_batch),
                };
                wire::serialize_get_invalidation_for_distribution_tenant_response(&result)
            }
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_get_key_group_config(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_key_group(id) {
            Ok(kg) => {
                let etag_val = kg.etag.clone();
                let wire_config = wire::KeyGroupConfig {
                    name: kg.name.clone(),
                    items: wire::PublicKeyIdList::from(kg.items.clone()),
                    comment: kg.comment.clone(),
                    ..Default::default()
                };
                let mut resp = wire::serialize_get_key_group_config_response(&wire_config);
                resp.headers.insert(ETAG, etag_val.parse().unwrap());
                resp
            }
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_get_managed_certificate_details(
        &self,
        _state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        _id: &str,
    ) -> MockResponse {
        // Managed certificate details are not directly stateful; return default response.
        let result = wire::ManagedCertificateDetails::default();
        wire::serialize_get_managed_certificate_details_response(&result)
    }

    async fn handle_get_monitoring_subscription(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        distribution_id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_monitoring_subscription(distribution_id) {
            Ok(ms) => {
                let result = wire::MonitoringSubscription {
                    realtime_metrics_subscription_config: Some(
                        wire::RealtimeMetricsSubscriptionConfig {
                            realtime_metrics_subscription_status: ms
                                .realtime_metrics_subscription_status
                                .clone(),
                        },
                    ),
                };
                wire::serialize_get_monitoring_subscription_response(&result)
            }
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_get_origin_access_control_config(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_origin_access_control(id) {
            Ok(oac) => {
                let etag_val = oac.etag.clone();
                let wire_config = wire::OriginAccessControlConfig {
                    name: oac.name.clone(),
                    description: oac.description.clone(),
                    origin_access_control_origin_type: oac
                        .origin_access_control_origin_type
                        .clone(),
                    signing_behavior: oac.signing_behavior.clone(),
                    signing_protocol: oac.signing_protocol.clone(),
                    ..Default::default()
                };
                let mut resp =
                    wire::serialize_get_origin_access_control_config_response(&wire_config);
                resp.headers.insert(ETAG, etag_val.parse().unwrap());
                resp
            }
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_get_origin_request_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_origin_request_policy(id) {
            Ok(orp) => {
                let etag_val = orp.etag.clone();
                let wire_orp = wire::OriginRequestPolicy {
                    id: Some(orp.id.clone()),
                    last_modified_time: Some(orp.last_modified_time.to_rfc3339()),
                    origin_request_policy_config: Some(orp.config.clone()),
                };
                let mut resp = wire::serialize_get_origin_request_policy_response(&wire_orp);
                resp.headers.insert(ETAG, etag_val.parse().unwrap());
                resp
            }
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_get_origin_request_policy_config(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_origin_request_policy(id) {
            Ok(orp) => {
                let etag_val = orp.etag.clone();
                let wire_config = orp.config.clone();
                let mut resp =
                    wire::serialize_get_origin_request_policy_config_response(&wire_config);
                resp.headers.insert(ETAG, etag_val.parse().unwrap());
                resp
            }
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_get_public_key_config(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_public_key(id) {
            Ok(pk) => {
                let etag_val = pk.etag.clone();
                let wire_config = wire::PublicKeyConfig {
                    caller_reference: pk.caller_reference.clone(),
                    name: pk.name.clone(),
                    encoded_key: pk.encoded_key.clone(),
                    comment: pk.comment.clone(),
                    ..Default::default()
                };
                let mut resp = wire::serialize_get_public_key_config_response(&wire_config);
                resp.headers.insert(ETAG, etag_val.parse().unwrap());
                resp
            }
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_get_realtime_log_config(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        body: &str,
    ) -> MockResponse {
        let name = extract_xml_value(body, "Name");
        let arn = extract_xml_value(body, "ARN");
        let state = state.read().await;
        match state.get_realtime_log_config(name.as_deref(), arn.as_deref()) {
            Ok(cfg) => {
                let wire_cfg = rtlc_to_wire(cfg);
                let result = wire::GetRealtimeLogConfigResult {
                    realtime_log_config: Some(wire_cfg),
                };
                wire::serialize_get_realtime_log_config_response(&result)
            }
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_get_resource_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        body: &str,
    ) -> MockResponse {
        let resource_arn = extract_xml_value(body, "ResourceArn").unwrap_or_default();
        let state = state.read().await;
        match state.get_resource_policy(&resource_arn) {
            Ok(rp) => {
                let result = wire::GetResourcePolicyResult {
                    resource_arn: Some(rp.resource_arn.clone()),
                    policy_document: Some(rp.policy_document.clone()),
                };
                wire::serialize_get_resource_policy_response(&result)
            }
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_get_response_headers_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_response_headers_policy(id) {
            Ok(rhp) => {
                let etag_val = rhp.etag.clone();
                let wire_rhp = wire::ResponseHeadersPolicy {
                    id: Some(rhp.id.clone()),
                    last_modified_time: Some(rhp.last_modified_time.to_rfc3339()),
                    response_headers_policy_config: Some(rhp.config.clone()),
                };
                let mut resp = wire::serialize_get_response_headers_policy_response(&wire_rhp);
                resp.headers.insert(ETAG, etag_val.parse().unwrap());
                resp
            }
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_get_response_headers_policy_config(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_response_headers_policy(id) {
            Ok(rhp) => {
                let etag_val = rhp.etag.clone();
                let wire_config = rhp.config.clone();
                let mut resp =
                    wire::serialize_get_response_headers_policy_config_response(&wire_config);
                resp.headers.insert(ETAG, etag_val.parse().unwrap());
                resp
            }
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_get_streaming_distribution(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_streaming_distribution(id) {
            Ok(sd) => {
                let etag_val = sd.etag.clone();
                let wire_sd = sd_to_wire(sd);
                let mut resp = wire::serialize_get_streaming_distribution_response(&wire_sd);
                resp.headers.insert(ETAG, etag_val.parse().unwrap());
                resp
            }
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_get_streaming_distribution_config(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_streaming_distribution(id) {
            Ok(sd) => {
                let etag_val = sd.etag.clone();
                let wire_config = sd.config.clone();
                let mut resp =
                    wire::serialize_get_streaming_distribution_config_response(&wire_config);
                resp.headers.insert(ETAG, etag_val.parse().unwrap());
                resp
            }
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_get_trust_store(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_trust_store(id) {
            Ok(ts) => {
                let etag_val = ts.etag.clone();
                let wire_ts = ts_to_wire(ts);
                let mut resp = wire::serialize_get_trust_store_response(&wire_ts);
                resp.headers.insert(ETAG, etag_val.parse().unwrap());
                resp
            }
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_get_vpc_origin(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_vpc_origin(id) {
            Ok(vpc) => {
                let etag_val = vpc.etag.clone();
                let wire_vpc = vpc_to_wire(vpc);
                let mut resp = wire::serialize_get_vpc_origin_response(&wire_vpc);
                resp.headers.insert(ETAG, etag_val.parse().unwrap());
                resp
            }
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_list_anycast_ip_lists(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let lists = state.list_anycast_ip_lists();
        let items: Vec<wire::AnycastIpListSummary> = lists
            .iter()
            .map(|l| wire::AnycastIpListSummary {
                id: Some(l.id.clone()),
                arn: Some(l.arn.clone()),
                name: Some(l.name.clone()),
                status: Some(l.status.clone()),
                ip_count: Some(l.ip_count),
                ip_address_type: l.ip_address_type.clone(),
                last_modified_time: Some(l.last_modified_time.to_rfc3339()),
                e_tag: Some(l.etag.clone()),
                ..Default::default()
            })
            .collect();
        let quantity = items.len() as i32;
        let wire_collection = wire::AnycastIpListCollection {
            quantity: Some(quantity),
            max_items: Some(100),
            is_truncated: Some(false),
            marker: Some(String::new()),
            items: if items.is_empty() {
                None
            } else {
                Some(wire::AnycastIpListSummaries::from(items))
            },
            ..Default::default()
        };
        wire::serialize_list_anycast_ip_lists_response(&wire_collection)
    }

    async fn handle_list_cache_policies(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let policies = state.list_cache_policies();
        let items: Vec<wire::CachePolicySummary> = policies
            .iter()
            .map(|cp| wire::CachePolicySummary {
                r#type: Some("custom".to_string()),
                cache_policy: Some(wire::CachePolicy {
                    id: Some(cp.id.clone()),
                    last_modified_time: Some(cp.last_modified_time.to_rfc3339()),
                    cache_policy_config: Some(cp.config.clone()),
                }),
            })
            .collect();
        let quantity = items.len() as i32;
        let wire_list = wire::CachePolicyList {
            quantity: Some(quantity),
            max_items: Some(100),
            items: if items.is_empty() {
                None
            } else {
                Some(wire::CachePolicySummaryList::from(items))
            },
            ..Default::default()
        };
        wire::serialize_list_cache_policies_response(&wire_list)
    }

    async fn handle_list_cloud_front_origin_access_identities(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let oais = state.list_oais();
        let items: Vec<wire::CloudFrontOriginAccessIdentitySummary> = oais
            .iter()
            .map(|oai| wire::CloudFrontOriginAccessIdentitySummary {
                id: Some(oai.id.clone()),
                comment: Some(oai.comment.clone()),
                s3_canonical_user_id: Some(oai.s3_canonical_user_id.clone()),
                ..Default::default()
            })
            .collect();
        let quantity = items.len() as i32;
        let wire_list = wire::CloudFrontOriginAccessIdentityList {
            quantity: Some(quantity),
            max_items: Some(100),
            is_truncated: Some(false),
            items: if items.is_empty() {
                None
            } else {
                Some(wire::CloudFrontOriginAccessIdentitySummaryList::from(items))
            },
            ..Default::default()
        };
        wire::serialize_list_cloud_front_origin_access_identities_response(&wire_list)
    }

    async fn handle_list_conflicting_aliases(
        &self,
        _state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
    ) -> MockResponse {
        // Conflicting aliases require cross-account data; return empty list.
        let result = wire::ConflictingAliasesList::default();
        wire::serialize_list_conflicting_aliases_response(&result)
    }

    async fn handle_list_connection_functions(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let funcs = state.list_connection_functions();
        let items: Vec<wire::ConnectionFunctionSummary> =
            funcs.iter().map(|f| conn_func_to_wire(f)).collect();
        let result = wire::ListConnectionFunctionsResult {
            connection_functions: if items.is_empty() {
                None
            } else {
                Some(wire::ConnectionFunctionSummaryList::from(items))
            },
            next_marker: None,
        };
        wire::serialize_list_connection_functions_response(&result)
    }

    async fn handle_list_connection_groups(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let groups = state.list_connection_groups();
        let items: Vec<wire::ConnectionGroupSummary> = groups
            .iter()
            .map(|cg| conn_group_summary_to_wire(cg))
            .collect();
        let result = wire::ListConnectionGroupsResult {
            connection_groups: if items.is_empty() {
                None
            } else {
                Some(wire::ConnectionGroupSummaryList::from(items))
            },
            next_marker: None,
        };
        wire::serialize_list_connection_groups_response(&result)
    }

    async fn handle_list_continuous_deployment_policies(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let policies = state.list_continuous_deployment_policies();
        let items: Vec<wire::ContinuousDeploymentPolicySummary> = policies
            .iter()
            .map(|cdp| wire::ContinuousDeploymentPolicySummary {
                continuous_deployment_policy: Some(wire::ContinuousDeploymentPolicy {
                    id: Some(cdp.id.clone()),
                    last_modified_time: Some(cdp.last_modified_time.to_rfc3339()),
                    continuous_deployment_policy_config: Some(cdp.config.clone()),
                }),
            })
            .collect();
        let quantity = items.len() as i32;
        let result = wire::ContinuousDeploymentPolicyList {
            quantity: Some(quantity),
            max_items: Some(100),
            items: if items.is_empty() {
                None
            } else {
                Some(wire::ContinuousDeploymentPolicySummaryList::from(items))
            },
            next_marker: None,
        };
        wire::serialize_list_continuous_deployment_policies_response(&result)
    }

    async fn handle_list_distribution_tenants(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let tenants = state.list_distribution_tenants();
        let items: Vec<wire::DistributionTenantSummary> = tenants
            .iter()
            .map(|dt| dist_tenant_summary_to_wire(dt))
            .collect();
        let result = wire::ListDistributionTenantsResult {
            distribution_tenant_list: if items.is_empty() {
                None
            } else {
                Some(wire::DistributionTenantList::from(items))
            },
            next_marker: None,
        };
        wire::serialize_list_distribution_tenants_response(&result)
    }

    async fn handle_list_distribution_tenants_by_customization(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let tenants = state.list_distribution_tenants();
        let items: Vec<wire::DistributionTenantSummary> = tenants
            .iter()
            .map(|dt| dist_tenant_summary_to_wire(dt))
            .collect();
        let result = wire::ListDistributionTenantsByCustomizationResult {
            distribution_tenant_list: if items.is_empty() {
                None
            } else {
                Some(wire::DistributionTenantList::from(items))
            },
            next_marker: None,
        };
        wire::serialize_list_distribution_tenants_by_customization_response(&result)
    }

    async fn handle_list_distributions_by_anycast_ip_list_id(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        // Verify the anycast IP list exists.
        if let Err(e) = state.get_anycast_ip_list(id) {
            return cloudfront_error_response(&e);
        }
        // Return an empty distribution list (association tracking not implemented).
        let result = wire::DistributionList {
            quantity: Some(0),
            max_items: Some(100),
            is_truncated: Some(false),
            marker: Some(String::new()),
            ..Default::default()
        };
        wire::serialize_list_distributions_by_anycast_ip_list_id_response(&result)
    }

    async fn handle_list_distributions_by_cache_policy_id(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        // Verify the cache policy exists first.
        if let Err(e) = state.get_cache_policy(id) {
            return cloudfront_error_response(&e);
        }
        // Find distributions whose raw_config references this cache policy ID.
        let dist_ids: Vec<String> = state
            .distributions
            .values()
            .filter(|d| {
                d.raw_config
                    .default_cache_behavior
                    .cache_policy_id
                    .as_deref()
                    == Some(id)
                    || d.raw_config
                        .cache_behaviors
                        .as_ref()
                        .and_then(|cbs| cbs.items.as_ref())
                        .map(|list| {
                            list.items
                                .iter()
                                .any(|cb| cb.cache_policy_id.as_deref() == Some(id))
                        })
                        .unwrap_or(false)
            })
            .map(|d| d.id.clone())
            .collect();
        let quantity = dist_ids.len() as i32;
        let wire_list = wire::DistributionIdList {
            quantity: Some(quantity),
            items: if dist_ids.is_empty() {
                None
            } else {
                Some(wire::DistributionIdListSummary::from(dist_ids))
            },
            ..Default::default()
        };
        wire::serialize_list_distributions_by_cache_policy_id_response(&wire_list)
    }

    async fn handle_list_distributions_by_connection_function(
        &self,
        _state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
    ) -> MockResponse {
        // Connection function association tracking returns empty list.
        let result = wire::DistributionList {
            quantity: Some(0),
            max_items: Some(100),
            is_truncated: Some(false),
            marker: Some(String::new()),
            ..Default::default()
        };
        wire::serialize_list_distributions_by_connection_function_response(&result)
    }

    async fn handle_list_distributions_by_connection_mode(
        &self,
        _state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        _mode: &str,
    ) -> MockResponse {
        // Connection mode filtering returns empty list.
        let result = wire::DistributionList {
            quantity: Some(0),
            max_items: Some(100),
            is_truncated: Some(false),
            marker: Some(String::new()),
            ..Default::default()
        };
        wire::serialize_list_distributions_by_connection_mode_response(&result)
    }

    async fn handle_list_distributions_by_key_group(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        // Verify the key group exists first.
        if let Err(e) = state.get_key_group(id) {
            return cloudfront_error_response(&e);
        }
        // Find distributions whose default_cache_behavior or cache_behaviors reference this key group.
        let dist_ids: Vec<String> = state
            .distributions
            .values()
            .filter(|d| {
                d.raw_config
                    .default_cache_behavior
                    .trusted_key_groups
                    .as_ref()
                    .and_then(|tkg| tkg.items.as_ref())
                    .map(|list| list.items.iter().any(|kg_id| kg_id == id))
                    .unwrap_or(false)
                    || d.raw_config
                        .cache_behaviors
                        .as_ref()
                        .and_then(|cbs| cbs.items.as_ref())
                        .map(|list| {
                            list.items.iter().any(|cb| {
                                cb.trusted_key_groups
                                    .as_ref()
                                    .and_then(|tkg| tkg.items.as_ref())
                                    .map(|kglist| kglist.items.iter().any(|kg_id| kg_id == id))
                                    .unwrap_or(false)
                            })
                        })
                        .unwrap_or(false)
            })
            .map(|d| d.id.clone())
            .collect();
        let quantity = dist_ids.len() as i32;
        let wire_list = wire::DistributionIdList {
            quantity: Some(quantity),
            items: if dist_ids.is_empty() {
                None
            } else {
                Some(wire::DistributionIdListSummary::from(dist_ids))
            },
            ..Default::default()
        };
        wire::serialize_list_distributions_by_key_group_response(&wire_list)
    }

    async fn handle_list_distributions_by_origin_request_policy_id(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        // Verify the origin request policy exists first.
        if let Err(e) = state.get_origin_request_policy(id) {
            return cloudfront_error_response(&e);
        }
        // Find distributions that reference this origin request policy.
        let dist_ids: Vec<String> = state
            .distributions
            .values()
            .filter(|d| {
                d.raw_config
                    .default_cache_behavior
                    .origin_request_policy_id
                    .as_deref()
                    == Some(id)
                    || d.raw_config
                        .cache_behaviors
                        .as_ref()
                        .and_then(|cbs| cbs.items.as_ref())
                        .map(|list| {
                            list.items
                                .iter()
                                .any(|cb| cb.origin_request_policy_id.as_deref() == Some(id))
                        })
                        .unwrap_or(false)
            })
            .map(|d| d.id.clone())
            .collect();
        let quantity = dist_ids.len() as i32;
        let wire_list = wire::DistributionIdList {
            quantity: Some(quantity),
            items: if dist_ids.is_empty() {
                None
            } else {
                Some(wire::DistributionIdListSummary::from(dist_ids))
            },
            ..Default::default()
        };
        wire::serialize_list_distributions_by_origin_request_policy_id_response(&wire_list)
    }

    async fn handle_list_distributions_by_owned_resource(
        &self,
        _state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        _arn: &str,
    ) -> MockResponse {
        // Resource ownership index returns empty list.
        let result = wire::DistributionIdOwnerList {
            quantity: Some(0),
            max_items: Some(100),
            is_truncated: Some(false),
            marker: Some(String::new()),
            ..Default::default()
        };
        wire::serialize_list_distributions_by_owned_resource_response(&result)
    }

    async fn handle_list_distributions_by_realtime_log_config(
        &self,
        _state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
    ) -> MockResponse {
        // Realtime log config distribution association returns empty list.
        let result = wire::DistributionList {
            quantity: Some(0),
            max_items: Some(100),
            is_truncated: Some(false),
            marker: Some(String::new()),
            ..Default::default()
        };
        wire::serialize_list_distributions_by_realtime_log_config_response(&result)
    }

    async fn handle_list_distributions_by_response_headers_policy_id(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        // Verify the response headers policy exists first.
        if let Err(e) = state.get_response_headers_policy(id) {
            return cloudfront_error_response(&e);
        }
        // Find distributions that reference this response headers policy.
        let dist_ids: Vec<String> = state
            .distributions
            .values()
            .filter(|d| {
                d.raw_config
                    .default_cache_behavior
                    .response_headers_policy_id
                    .as_deref()
                    == Some(id)
                    || d.raw_config
                        .cache_behaviors
                        .as_ref()
                        .and_then(|cbs| cbs.items.as_ref())
                        .map(|list| {
                            list.items
                                .iter()
                                .any(|cb| cb.response_headers_policy_id.as_deref() == Some(id))
                        })
                        .unwrap_or(false)
            })
            .map(|d| d.id.clone())
            .collect();
        let quantity = dist_ids.len() as i32;
        let wire_list = wire::DistributionIdList {
            quantity: Some(quantity),
            items: if dist_ids.is_empty() {
                None
            } else {
                Some(wire::DistributionIdListSummary::from(dist_ids))
            },
            ..Default::default()
        };
        wire::serialize_list_distributions_by_response_headers_policy_id_response(&wire_list)
    }

    async fn handle_list_distributions_by_trust_store(
        &self,
        _state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
    ) -> MockResponse {
        // Trust store distribution association returns empty list.
        let result = wire::DistributionList {
            quantity: Some(0),
            max_items: Some(100),
            is_truncated: Some(false),
            marker: Some(String::new()),
            ..Default::default()
        };
        wire::serialize_list_distributions_by_trust_store_response(&result)
    }

    async fn handle_list_distributions_by_vpc_origin_id(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        // Verify the VPC origin exists.
        if let Err(e) = state.get_vpc_origin(id) {
            return cloudfront_error_response(&e);
        }
        // Return an empty distribution ID list (association tracking not implemented).
        let result = wire::DistributionIdList {
            quantity: Some(0),
            max_items: Some(100),
            is_truncated: Some(false),
            marker: Some(String::new()),
            ..Default::default()
        };
        wire::serialize_list_distributions_by_vpc_origin_id_response(&result)
    }

    async fn handle_list_distributions_by_web_a_c_l_id(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        web_acl_id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        let dist_ids = state.list_distributions_by_web_acl_id(web_acl_id);
        let items: Vec<wire::DistributionSummary> = dist_ids
            .iter()
            .filter_map(|id| state.distributions.get(*id))
            .map(dist_to_summary)
            .collect();
        let quantity = items.len() as i32;
        let result = wire::DistributionList {
            quantity: Some(quantity),
            max_items: Some(100),
            is_truncated: Some(false),
            marker: Some(String::new()),
            items: if items.is_empty() {
                None
            } else {
                Some(wire::DistributionSummaryList::from(items))
            },
            ..Default::default()
        };
        wire::serialize_list_distributions_by_web_a_c_l_id_response(&result)
    }

    async fn handle_list_domain_conflicts(
        &self,
        _state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
    ) -> MockResponse {
        // Domain conflicts require cross-resource analysis; return empty list.
        let result = wire::ListDomainConflictsResult::default();
        wire::serialize_list_domain_conflicts_response(&result)
    }

    async fn handle_list_field_level_encryption_configs(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let configs = state.list_field_level_encryption_configs();
        let items: Vec<wire::FieldLevelEncryptionSummary> = configs
            .iter()
            .map(|fle| wire::FieldLevelEncryptionSummary {
                id: Some(fle.id.clone()),
                last_modified_time: Some(fle.last_modified_time.to_rfc3339()),
                comment: fle.config.comment.clone(),
                content_type_profile_config: fle.config.content_type_profile_config.clone(),
                query_arg_profile_config: fle.config.query_arg_profile_config.clone(),
            })
            .collect();
        let quantity = items.len() as i32;
        let wire_list = wire::FieldLevelEncryptionList {
            quantity: Some(quantity),
            max_items: Some(100),
            items: if items.is_empty() {
                None
            } else {
                Some(wire::FieldLevelEncryptionSummaryList::from(items))
            },
            ..Default::default()
        };
        wire::serialize_list_field_level_encryption_configs_response(&wire_list)
    }

    async fn handle_list_field_level_encryption_profiles(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let profiles = state.list_field_level_encryption_profiles();
        let items: Vec<wire::FieldLevelEncryptionProfileSummary> = profiles
            .iter()
            .map(|p| wire::FieldLevelEncryptionProfileSummary {
                id: Some(p.id.clone()),
                last_modified_time: Some(p.last_modified_time.to_rfc3339()),
                name: Some(p.config.name.clone()),
                comment: p.config.comment.clone(),
                encryption_entities: Some(p.config.encryption_entities.clone()),
            })
            .collect();
        let quantity = items.len() as i32;
        let wire_list = wire::FieldLevelEncryptionProfileList {
            quantity: Some(quantity),
            max_items: Some(100),
            items: if items.is_empty() {
                None
            } else {
                Some(wire::FieldLevelEncryptionProfileSummaryList::from(items))
            },
            ..Default::default()
        };
        wire::serialize_list_field_level_encryption_profiles_response(&wire_list)
    }

    async fn handle_list_functions(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let funcs = state.list_functions();
        let items: Vec<wire::FunctionSummary> = funcs.iter().map(|f| func_to_wire(f)).collect();
        let quantity = items.len() as i32;
        let wire_list = wire::FunctionList {
            quantity: Some(quantity),
            max_items: Some(100),
            items: if items.is_empty() {
                None
            } else {
                Some(wire::FunctionSummaryList::from(items))
            },
            ..Default::default()
        };
        wire::serialize_list_functions_response(&wire_list)
    }

    async fn handle_list_invalidations_for_distribution_tenant(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        tenant_id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        let invs = state.list_invalidations_for_distribution_tenant(tenant_id);
        let items: Vec<wire::InvalidationSummary> = invs
            .iter()
            .map(|inv| wire::InvalidationSummary {
                id: Some(inv.id.clone()),
                create_time: Some(inv.create_time.to_rfc3339()),
                status: Some(inv.status.clone()),
            })
            .collect();
        let quantity = items.len() as i32;
        let result = wire::InvalidationList {
            quantity: Some(quantity),
            max_items: Some(100),
            is_truncated: Some(false),
            marker: Some(String::new()),
            items: if items.is_empty() {
                None
            } else {
                Some(wire::InvalidationSummaryList::from(items))
            },
            ..Default::default()
        };
        wire::serialize_list_invalidations_for_distribution_tenant_response(&result)
    }

    async fn handle_list_key_value_stores(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let stores = state.list_key_value_stores();
        let items: Vec<wire::KeyValueStore> = stores.iter().map(|kvs| kvs_to_wire(kvs)).collect();
        let quantity = items.len() as i32;
        let wire_list = wire::KeyValueStoreList {
            quantity: Some(quantity),
            max_items: Some(100),
            items: if items.is_empty() {
                None
            } else {
                Some(wire::KeyValueStoreSummaryList::from(items))
            },
            ..Default::default()
        };
        wire::serialize_list_key_value_stores_response(&wire_list)
    }

    async fn handle_list_origin_request_policies(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let policies = state.list_origin_request_policies();
        let items: Vec<wire::OriginRequestPolicySummary> = policies
            .iter()
            .map(|orp| wire::OriginRequestPolicySummary {
                r#type: Some("custom".to_string()),
                origin_request_policy: Some(wire::OriginRequestPolicy {
                    id: Some(orp.id.clone()),
                    last_modified_time: Some(orp.last_modified_time.to_rfc3339()),
                    origin_request_policy_config: Some(orp.config.clone()),
                }),
            })
            .collect();
        let quantity = items.len() as i32;
        let wire_list = wire::OriginRequestPolicyList {
            quantity: Some(quantity),
            max_items: Some(100),
            items: if items.is_empty() {
                None
            } else {
                Some(wire::OriginRequestPolicySummaryList::from(items))
            },
            ..Default::default()
        };
        wire::serialize_list_origin_request_policies_response(&wire_list)
    }

    async fn handle_list_realtime_log_configs(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let configs = state.list_realtime_log_configs();
        let items: Vec<wire::RealtimeLogConfig> =
            configs.iter().map(|cfg| rtlc_to_wire(cfg)).collect();
        let wire_list = wire::RealtimeLogConfigs {
            max_items: Some(100),
            is_truncated: Some(false),
            items: if items.is_empty() {
                None
            } else {
                Some(wire::RealtimeLogConfigList::from(items))
            },
            ..Default::default()
        };
        wire::serialize_list_realtime_log_configs_response(&wire_list)
    }

    async fn handle_list_response_headers_policies(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let policies = state.list_response_headers_policies();
        let items: Vec<wire::ResponseHeadersPolicySummary> = policies
            .iter()
            .map(|rhp| wire::ResponseHeadersPolicySummary {
                r#type: Some("custom".to_string()),
                response_headers_policy: Some(wire::ResponseHeadersPolicy {
                    id: Some(rhp.id.clone()),
                    last_modified_time: Some(rhp.last_modified_time.to_rfc3339()),
                    response_headers_policy_config: Some(rhp.config.clone()),
                }),
            })
            .collect();
        let quantity = items.len() as i32;
        let wire_list = wire::ResponseHeadersPolicyList {
            quantity: Some(quantity),
            max_items: Some(100),
            items: if items.is_empty() {
                None
            } else {
                Some(wire::ResponseHeadersPolicySummaryList::from(items))
            },
            ..Default::default()
        };
        wire::serialize_list_response_headers_policies_response(&wire_list)
    }

    async fn handle_list_streaming_distributions(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let sds = state.list_streaming_distributions();
        let items: Vec<wire::StreamingDistributionSummary> = sds
            .iter()
            .map(|sd| wire::StreamingDistributionSummary {
                id: Some(sd.id.clone()),
                a_r_n: Some(sd.arn.clone()),
                domain_name: Some(sd.domain_name.clone()),
                status: Some(sd.status.clone()),
                last_modified_time: Some(sd.last_modified_time.to_rfc3339()),
                enabled: Some(sd.config.enabled),
                s3_origin: Some(sd.config.s3_origin.clone()),
                ..Default::default()
            })
            .collect();
        let quantity = items.len() as i32;
        let wire_list = wire::StreamingDistributionList {
            quantity: Some(quantity),
            max_items: Some(100),
            is_truncated: Some(false),
            items: if items.is_empty() {
                None
            } else {
                Some(wire::StreamingDistributionSummaryList::from(items))
            },
            ..Default::default()
        };
        wire::serialize_list_streaming_distributions_response(&wire_list)
    }

    async fn handle_list_trust_stores(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let stores = state.list_trust_stores();
        let items: Vec<wire::TrustStoreSummary> = stores
            .iter()
            .map(|ts| wire::TrustStoreSummary {
                id: Some(ts.id.clone()),
                arn: Some(ts.arn.clone()),
                name: Some(ts.name.clone()),
                last_modified_time: Some(ts.last_modified_time.to_rfc3339()),
                status: Some(ts.status.clone()),
                number_of_ca_certificates: Some(ts.number_of_ca_certificates),
                ..Default::default()
            })
            .collect();
        let wire_list = wire::ListTrustStoresResult {
            trust_store_list: if items.is_empty() {
                None
            } else {
                Some(wire::TrustStoreList::from(items))
            },
            ..Default::default()
        };
        wire::serialize_list_trust_stores_response(&wire_list)
    }

    async fn handle_list_vpc_origins(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let vpcs = state.list_vpc_origins();
        let items: Vec<wire::VpcOriginSummary> = vpcs
            .iter()
            .map(|vpc| wire::VpcOriginSummary {
                id: Some(vpc.id.clone()),
                arn: Some(vpc.arn.clone()),
                account_id: Some(vpc.account_id.clone()),
                created_time: Some(vpc.created_time.to_rfc3339()),
                last_modified_time: Some(vpc.last_modified_time.to_rfc3339()),
                status: Some(vpc.status.clone()),
                name: Some(vpc.config.name.clone()),
                origin_endpoint_arn: Some(vpc.config.arn.clone()),
            })
            .collect();
        let quantity = items.len() as i32;
        let wire_list = wire::VpcOriginList {
            quantity: Some(quantity),
            max_items: Some(100),
            is_truncated: Some(false),
            items: if items.is_empty() {
                None
            } else {
                Some(wire::VpcOriginSummaryList::from(items))
            },
            ..Default::default()
        };
        wire::serialize_list_vpc_origins_response(&wire_list)
    }

    async fn handle_publish_connection_function(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        id: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.publish_connection_function(id) {
            Ok(func) => {
                let etag_val = func.etag.clone();
                let result = conn_func_to_wire(func);
                let mut resp = wire::serialize_publish_connection_function_response(&result);
                resp.headers.insert(ETAG, etag_val.parse().unwrap());
                resp
            }
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_publish_function(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        name: &str,
        if_match: Option<&str>,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.publish_function(name, if_match) {
            Ok(func) => {
                let etag_val = func.etag.clone();
                let wire_func = func_to_wire(func);
                let mut resp = wire::serialize_publish_function_response(&wire_func);
                resp.headers.insert(ETAG, etag_val.parse().unwrap());
                resp
            }
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_put_resource_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        body: &str,
    ) -> MockResponse {
        let resource_arn = extract_xml_value(body, "ResourceArn").unwrap_or_default();
        let policy_document = extract_xml_value(body, "PolicyDocument").unwrap_or_default();
        let mut state = state.write().await;
        let _rp = state.put_resource_policy(&resource_arn, &policy_document);
        let result = wire::PutResourcePolicyResult {
            resource_arn: Some(resource_arn),
        };
        wire::serialize_put_resource_policy_response(&result)
    }

    // STUB[no-engine]: requires connection function execution runtime.
    async fn handle_test_connection_function(
        &self,
        _state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        _id: &str,
    ) -> MockResponse {
        let result = wire::ConnectionFunctionTestResult::default();
        wire::serialize_test_connection_function_response(&result)
    }

    // STUB[no-engine]: requires CloudFront function execution runtime.
    async fn handle_test_function(
        &self,
        _state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        _name: &str,
    ) -> MockResponse {
        let result = wire::TestResult::default();
        wire::serialize_test_function_response(&result)
    }

    async fn handle_update_anycast_ip_list(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        id: &str,
        body: &str,
        if_match: Option<&str>,
    ) -> MockResponse {
        let req = match quick_xml::de::from_str::<crate::model::UpdateAnycastIpListRequest>(body) {
            Ok(r) => r,
            Err(e) => {
                return xml_error_response(
                    400,
                    "MalformedXML",
                    &format!("Failed to parse UpdateAnycastIpListRequest: {e}"),
                );
            }
        };
        let mut state = state.write().await;
        match state.update_anycast_ip_list(id, req.ip_address_type, if_match) {
            Ok(list) => {
                let etag_val = list.etag.clone();
                let wire_list = anycast_to_wire(list);
                let mut resp = wire::serialize_update_anycast_ip_list_response(&wire_list);
                resp.headers.insert(ETAG, etag_val.parse().unwrap());
                resp
            }
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_update_cache_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        id: &str,
        body: &str,
        if_match: Option<&str>,
    ) -> MockResponse {
        let config = match quick_xml::de::from_str::<crate::model::CachePolicyConfig>(body) {
            Ok(c) => c,
            Err(e) => {
                return xml_error_response(
                    400,
                    "MalformedXML",
                    &format!("Failed to parse CachePolicyConfig: {e}"),
                );
            }
        };
        let mut state = state.write().await;
        match state.update_cache_policy(id, config, if_match) {
            Ok(cp) => {
                let etag_val = cp.etag.clone();
                let wire_cp = wire::CachePolicy {
                    id: Some(cp.id.clone()),
                    last_modified_time: Some(cp.last_modified_time.to_rfc3339()),
                    cache_policy_config: Some(cp.config.clone()),
                };
                let mut resp = wire::serialize_update_cache_policy_response(&wire_cp);
                resp.headers.insert(ETAG, etag_val.parse().unwrap());
                resp
            }
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_update_cloud_front_origin_access_identity(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        id: &str,
        body: &str,
        if_match: Option<&str>,
    ) -> MockResponse {
        let caller_reference = match extract_xml_value(body, "CallerReference") {
            Some(cr) => cr,
            None => return xml_error_response(400, "MissingParameter", "Missing CallerReference"),
        };
        let comment = extract_xml_value(body, "Comment").unwrap_or_default();
        let mut state = state.write().await;
        match state.update_oai(id, &caller_reference, &comment, if_match) {
            Ok(oai) => {
                let etag_val = oai.etag.clone();
                let wire_oai = oai_to_wire(oai);
                let mut resp =
                    wire::serialize_update_cloud_front_origin_access_identity_response(&wire_oai);
                resp.headers.insert(ETAG, etag_val.parse().unwrap());
                resp
            }
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_update_connection_function(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        id: &str,
        body: &str,
        if_match: Option<&str>,
    ) -> MockResponse {
        let req =
            match quick_xml::de::from_str::<crate::model::UpdateConnectionFunctionRequest>(body) {
                Ok(r) => r,
                Err(e) => {
                    return xml_error_response(
                        400,
                        "MalformedXML",
                        &format!("Failed to parse UpdateConnectionFunctionRequest: {e}"),
                    );
                }
            };
        let mut state = state.write().await;
        match state.update_connection_function(
            id,
            req.connection_function_config,
            &req.connection_function_code,
            if_match,
        ) {
            Ok(func) => {
                let etag_val = func.etag.clone();
                let result = conn_func_to_wire(func);
                let mut resp = wire::serialize_update_connection_function_response(&result);
                resp.headers.insert(ETAG, etag_val.parse().unwrap());
                resp
            }
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_update_connection_group(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        id: &str,
        body: &str,
        if_match: Option<&str>,
    ) -> MockResponse {
        let req = match quick_xml::de::from_str::<crate::model::UpdateConnectionGroupRequest>(body)
        {
            Ok(r) => r,
            Err(e) => {
                return xml_error_response(
                    400,
                    "MalformedXML",
                    &format!("Failed to parse UpdateConnectionGroupRequest: {e}"),
                );
            }
        };
        let mut state = state.write().await;
        match state.update_connection_group(
            id,
            req.enabled,
            req.ipv6_enabled,
            req.anycast_ip_list_id,
            if_match,
        ) {
            Ok(cg) => {
                let etag_val = cg.etag.clone();
                let result = conn_group_to_wire(cg);
                let mut resp = wire::serialize_update_connection_group_response(&result);
                resp.headers.insert(ETAG, etag_val.parse().unwrap());
                resp
            }
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_update_continuous_deployment_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        id: &str,
        body: &str,
        if_match: Option<&str>,
    ) -> MockResponse {
        let req = match quick_xml::de::from_str::<
            crate::model::UpdateContinuousDeploymentPolicyRequest,
        >(body)
        {
            Ok(r) => r,
            Err(e) => {
                return xml_error_response(
                    400,
                    "MalformedXML",
                    &format!("Failed to parse UpdateContinuousDeploymentPolicyRequest: {e}"),
                );
            }
        };
        let mut state = state.write().await;
        match state.update_continuous_deployment_policy(
            id,
            req.continuous_deployment_policy_config,
            if_match,
        ) {
            Ok(cdp) => {
                let etag_val = cdp.etag.clone();
                let result = wire::ContinuousDeploymentPolicy {
                    id: Some(cdp.id.clone()),
                    last_modified_time: Some(cdp.last_modified_time.to_rfc3339()),
                    continuous_deployment_policy_config: Some(cdp.config.clone()),
                };
                let mut resp =
                    wire::serialize_update_continuous_deployment_policy_response(&result);
                resp.headers.insert(ETAG, etag_val.parse().unwrap());
                resp
            }
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_update_distribution_tenant(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        id: &str,
        body: &str,
        if_match: Option<&str>,
    ) -> MockResponse {
        let req =
            match quick_xml::de::from_str::<crate::model::UpdateDistributionTenantRequest>(body) {
                Ok(r) => r,
                Err(e) => {
                    return xml_error_response(
                        400,
                        "MalformedXML",
                        &format!("Failed to parse UpdateDistributionTenantRequest: {e}"),
                    );
                }
            };
        let mut state = state.write().await;
        match state.update_distribution_tenant(
            id,
            req.connection_group_id,
            req.enabled,
            req.domains,
            req.customizations,
            if_match,
        ) {
            Ok(dt) => {
                let etag_val = dt.etag.clone();
                let result = dist_tenant_to_wire(dt);
                let mut resp = wire::serialize_update_distribution_tenant_response(&result);
                resp.headers.insert(ETAG, etag_val.parse().unwrap());
                resp
            }
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_update_distribution_with_staging_config(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        id: &str,
    ) -> MockResponse {
        // Staging config promotion: just echo back the current distribution.
        let state = state.read().await;
        match state.get_distribution(id) {
            Ok(dist) => {
                let etag_val = dist.etag.clone();
                let wire_dist = dist_to_wire(dist);
                let mut resp =
                    wire::serialize_update_distribution_with_staging_config_response(&wire_dist);
                resp.headers.insert(ETAG, etag_val.parse().unwrap());
                resp
            }
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_update_domain_association(
        &self,
        _state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        body: &str,
    ) -> MockResponse {
        let domain = extract_xml_value(body, "Domain");
        let resource_id = extract_xml_value(body, "ResourceId");
        let result = wire::UpdateDomainAssociationResult {
            domain,
            resource_id,
        };
        wire::serialize_update_domain_association_response(&result)
    }

    async fn handle_update_field_level_encryption_config(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        id: &str,
        body: &str,
        if_match: Option<&str>,
    ) -> MockResponse {
        let config = match quick_xml::de::from_str::<crate::model::FieldLevelEncryptionConfig>(body)
        {
            Ok(c) => c,
            Err(e) => {
                return xml_error_response(
                    400,
                    "MalformedXML",
                    &format!("Failed to parse FieldLevelEncryptionConfig: {e}"),
                );
            }
        };
        let mut state = state.write().await;
        match state.update_field_level_encryption_config(id, config, if_match) {
            Ok(fle) => {
                let etag_val = fle.etag.clone();
                let wire_fle = wire::FieldLevelEncryption {
                    id: Some(fle.id.clone()),
                    last_modified_time: Some(fle.last_modified_time.to_rfc3339()),
                    field_level_encryption_config: Some(fle.config.clone()),
                };
                let mut resp =
                    wire::serialize_update_field_level_encryption_config_response(&wire_fle);
                resp.headers.insert(ETAG, etag_val.parse().unwrap());
                resp
            }
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_update_field_level_encryption_profile(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        id: &str,
        body: &str,
        if_match: Option<&str>,
    ) -> MockResponse {
        let config = match quick_xml::de::from_str::<crate::model::FieldLevelEncryptionProfileConfig>(
            body,
        ) {
            Ok(c) => c,
            Err(e) => {
                return xml_error_response(
                    400,
                    "MalformedXML",
                    &format!("Failed to parse FieldLevelEncryptionProfileConfig: {e}"),
                );
            }
        };
        let mut state = state.write().await;
        match state.update_field_level_encryption_profile(id, config, if_match) {
            Ok(p) => {
                let etag_val = p.etag.clone();
                let wire_p = wire::FieldLevelEncryptionProfile {
                    id: Some(p.id.clone()),
                    last_modified_time: Some(p.last_modified_time.to_rfc3339()),
                    field_level_encryption_profile_config: Some(p.config.clone()),
                };
                let mut resp =
                    wire::serialize_update_field_level_encryption_profile_response(&wire_p);
                resp.headers.insert(ETAG, etag_val.parse().unwrap());
                resp
            }
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_update_function(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        name: &str,
        body: &str,
        if_match: Option<&str>,
    ) -> MockResponse {
        let config =
            quick_xml::de::from_str::<crate::model::FunctionConfig>(body).unwrap_or_default();
        let code = body.as_bytes().to_vec();
        let mut state = state.write().await;
        match state.update_function(name, config, code, if_match) {
            Ok(func) => {
                let etag_val = func.etag.clone();
                let wire_func = func_to_wire(func);
                let mut resp = wire::serialize_update_function_response(&wire_func);
                resp.headers.insert(ETAG, etag_val.parse().unwrap());
                resp
            }
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_update_key_group(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        id: &str,
        body: &str,
        if_match: Option<&str>,
    ) -> MockResponse {
        let name = match extract_xml_value(body, "Name") {
            Some(n) => n,
            None => return xml_error_response(400, "MissingParameter", "Missing Name"),
        };
        let items = parse_key_group_items(body);
        let comment = extract_xml_value(body, "Comment");
        let mut state = state.write().await;
        match state.update_key_group(id, &name, items, comment, if_match) {
            Ok(kg) => {
                let etag_val = kg.etag.clone();
                let wire_kg = kg_to_wire(kg);
                let mut resp = wire::serialize_update_key_group_response(&wire_kg);
                resp.headers.insert(ETAG, etag_val.parse().unwrap());
                resp
            }
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_update_key_value_store(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        name: &str,
        body: &str,
        if_match: Option<&str>,
    ) -> MockResponse {
        let comment = extract_xml_value(body, "Comment");
        let mut state = state.write().await;
        match state.update_key_value_store(name, comment, if_match) {
            Ok(kvs) => {
                let etag_val = kvs.etag.clone();
                let wire_kvs = kvs_to_wire(kvs);
                let mut resp = wire::serialize_update_key_value_store_response(&wire_kvs);
                resp.headers.insert(ETAG, etag_val.parse().unwrap());
                resp
            }
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_update_origin_request_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        id: &str,
        body: &str,
        if_match: Option<&str>,
    ) -> MockResponse {
        let config = match quick_xml::de::from_str::<crate::model::OriginRequestPolicyConfig>(body)
        {
            Ok(c) => c,
            Err(e) => {
                return xml_error_response(
                    400,
                    "MalformedXML",
                    &format!("Failed to parse OriginRequestPolicyConfig: {e}"),
                );
            }
        };
        let mut state = state.write().await;
        match state.update_origin_request_policy(id, config, if_match) {
            Ok(orp) => {
                let etag_val = orp.etag.clone();
                let wire_orp = wire::OriginRequestPolicy {
                    id: Some(orp.id.clone()),
                    last_modified_time: Some(orp.last_modified_time.to_rfc3339()),
                    origin_request_policy_config: Some(orp.config.clone()),
                };
                let mut resp = wire::serialize_update_origin_request_policy_response(&wire_orp);
                resp.headers.insert(ETAG, etag_val.parse().unwrap());
                resp
            }
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_update_public_key(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        id: &str,
        body: &str,
        if_match: Option<&str>,
    ) -> MockResponse {
        let caller_reference = match extract_xml_value(body, "CallerReference") {
            Some(cr) => cr,
            None => return xml_error_response(400, "MissingParameter", "Missing CallerReference"),
        };
        let name = match extract_xml_value(body, "Name") {
            Some(n) => n,
            None => return xml_error_response(400, "MissingParameter", "Missing Name"),
        };
        let encoded_key = match extract_xml_value(body, "EncodedKey") {
            Some(k) => k,
            None => return xml_error_response(400, "MissingParameter", "Missing EncodedKey"),
        };
        let comment = extract_xml_value(body, "Comment");
        let mut state = state.write().await;
        match state.update_public_key(
            id,
            &caller_reference,
            &name,
            &encoded_key,
            comment,
            if_match,
        ) {
            Ok(pk) => {
                let etag_val = pk.etag.clone();
                let wire_pk = pk_to_wire(pk);
                let mut resp = wire::serialize_update_public_key_response(&wire_pk);
                resp.headers.insert(ETAG, etag_val.parse().unwrap());
                resp
            }
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_update_realtime_log_config(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        body: &str,
    ) -> MockResponse {
        let req =
            match quick_xml::de::from_str::<crate::model::UpdateRealtimeLogConfigRequest>(body) {
                Ok(r) => r,
                Err(e) => {
                    return xml_error_response(
                        400,
                        "MalformedXML",
                        &format!("Failed to parse UpdateRealtimeLogConfigRequest: {e}"),
                    );
                }
            };
        let end_points = req.end_points.map(|ep| ep.items).unwrap_or_default();
        let fields = req.fields.map(|fl| fl.items).unwrap_or_default();
        let sampling_rate = req.sampling_rate.unwrap_or(0);
        let mut state = state.write().await;
        match state.update_realtime_log_config(
            req.name.as_deref(),
            req.a_r_n.as_deref(),
            sampling_rate,
            end_points,
            fields,
        ) {
            Ok(cfg) => {
                let wire_cfg = rtlc_to_wire(cfg);
                let result = wire::UpdateRealtimeLogConfigResult {
                    realtime_log_config: Some(wire_cfg),
                };
                wire::serialize_update_realtime_log_config_response(&result)
            }
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_update_response_headers_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        id: &str,
        body: &str,
        if_match: Option<&str>,
    ) -> MockResponse {
        let config =
            match quick_xml::de::from_str::<crate::model::ResponseHeadersPolicyConfig>(body) {
                Ok(c) => c,
                Err(e) => {
                    return xml_error_response(
                        400,
                        "MalformedXML",
                        &format!("Failed to parse ResponseHeadersPolicyConfig: {e}"),
                    );
                }
            };
        let mut state = state.write().await;
        match state.update_response_headers_policy(id, config, if_match) {
            Ok(rhp) => {
                let etag_val = rhp.etag.clone();
                let wire_rhp = wire::ResponseHeadersPolicy {
                    id: Some(rhp.id.clone()),
                    last_modified_time: Some(rhp.last_modified_time.to_rfc3339()),
                    response_headers_policy_config: Some(rhp.config.clone()),
                };
                let mut resp = wire::serialize_update_response_headers_policy_response(&wire_rhp);
                resp.headers.insert(ETAG, etag_val.parse().unwrap());
                resp
            }
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_update_streaming_distribution(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        id: &str,
        body: &str,
        if_match: Option<&str>,
    ) -> MockResponse {
        let config =
            match quick_xml::de::from_str::<crate::model::StreamingDistributionConfig>(body) {
                Ok(c) => c,
                Err(e) => {
                    return xml_error_response(
                        400,
                        "MalformedXML",
                        &format!("Failed to parse StreamingDistributionConfig: {e}"),
                    );
                }
            };
        let mut state = state.write().await;
        match state.update_streaming_distribution(id, config, if_match) {
            Ok(sd) => {
                let etag_val = sd.etag.clone();
                let wire_sd = sd_to_wire(sd);
                let mut resp = wire::serialize_update_streaming_distribution_response(&wire_sd);
                resp.headers.insert(ETAG, etag_val.parse().unwrap());
                resp
            }
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_update_trust_store(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        id: &str,
        if_match: Option<&str>,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.update_trust_store(id, if_match) {
            Ok(ts) => {
                let etag_val = ts.etag.clone();
                let wire_ts = ts_to_wire(ts);
                let mut resp = wire::serialize_update_trust_store_response(&wire_ts);
                resp.headers.insert(ETAG, etag_val.parse().unwrap());
                resp
            }
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_update_vpc_origin(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        id: &str,
        body: &str,
        if_match: Option<&str>,
    ) -> MockResponse {
        let req = match quick_xml::de::from_str::<crate::model::CreateVpcOriginRequest>(body) {
            Ok(r) => r,
            Err(e) => {
                return xml_error_response(
                    400,
                    "MalformedXML",
                    &format!("Failed to parse VpcOriginEndpointConfig: {e}"),
                );
            }
        };
        let mut state = state.write().await;
        match state.update_vpc_origin(id, req.vpc_origin_endpoint_config, if_match) {
            Ok(vpc) => {
                let etag_val = vpc.etag.clone();
                let wire_vpc = vpc_to_wire(vpc);
                let mut resp = wire::serialize_update_vpc_origin_response(&wire_vpc);
                resp.headers.insert(ETAG, etag_val.parse().unwrap());
                resp
            }
            Err(e) => cloudfront_error_response(&e),
        }
    }

    async fn handle_verify_dns_configuration(
        &self,
        _state: &Arc<tokio::sync::RwLock<CloudFrontState>>,
        _body: &str,
    ) -> MockResponse {
        // DNS verification is a validation operation; return success with empty list.
        let result = wire::VerifyDnsConfigurationResult::default();
        wire::serialize_verify_dns_configuration_response(&result)
    }
}

// ---- Wire conversion helpers ----

fn dist_to_wire(dist: &crate::types::Distribution) -> wire::Distribution {
    // FIX(terraform-e2e): include ActiveTrustedSigners and ActiveTrustedKeyGroups
    // with Enabled=false/Quantity=0 so the Go provider does not dereference nil pointers.
    let active_trusted_signers = Some(wire::ActiveTrustedSigners {
        enabled: Some(false),
        quantity: Some(0),
        items: None,
    });
    let active_trusted_key_groups = Some(wire::ActiveTrustedKeyGroups {
        enabled: Some(false),
        quantity: Some(0),
        items: None,
    });
    wire::Distribution {
        id: Some(dist.id.clone()),
        a_r_n: Some(dist.arn.clone()),
        domain_name: Some(dist.domain_name.clone()),
        status: Some(dist.status.clone()),
        last_modified_time: Some(dist.created_at.to_rfc3339()),
        distribution_config: Some(dist_config_to_wire(dist)),
        active_trusted_signers,
        active_trusted_key_groups,
        in_progress_invalidation_batches: Some(0),
        ..Default::default()
    }
}

fn dist_config_to_wire(dist: &crate::types::Distribution) -> wire::DistributionConfig {
    // FIX(terraform-e2e): echo back the full DistributionConfig as received, so the Go
    // provider does not panic on nil pointer dereferences for fields it sent but that
    // we did not echo (e.g. Restrictions.GeoRestriction, ViewerCertificate).
    let mut config = dist.raw_config.clone();
    // FIX(terraform-e2e): always include OriginGroups with Quantity=0 so the Go provider
    // does not panic.  resourceDistributionRead (distribution.go:1098) does:
    //   aws.ToInt32(distributionConfig.OriginGroups.Quantity)
    // without a nil guard.  When OriginGroups is absent from the XML the Go SDK leaves
    // the field as nil, causing a nil-pointer dereference.
    if config.origin_groups.is_none() {
        config.origin_groups = Some(wire::OriginGroups {
            quantity: 0,
            items: None,
        });
    }
    config
}

fn dist_to_summary(dist: &crate::types::Distribution) -> wire::DistributionSummary {
    wire::DistributionSummary {
        id: Some(dist.id.clone()),
        a_r_n: Some(dist.arn.clone()),
        domain_name: Some(dist.domain_name.clone()),
        status: Some(dist.status.clone()),
        last_modified_time: Some(dist.created_at.to_rfc3339()),
        enabled: Some(dist.enabled),
        origins: Some(wire::Origins {
            quantity: dist.origins.len() as i32,
            items: wire::OriginList::from(
                dist.origins
                    .iter()
                    .map(|o| wire::Origin {
                        id: o.id.clone(),
                        domain_name: o.domain_name.clone(),
                        ..Default::default()
                    })
                    .collect::<Vec<_>>(),
            ),
            ..Default::default()
        }),
        default_cache_behavior: Some(wire::DefaultCacheBehavior {
            target_origin_id: dist.default_cache_behavior.target_origin_id.clone(),
            viewer_protocol_policy: dist.default_cache_behavior.viewer_protocol_policy.clone(),
            ..Default::default()
        }),
        ..Default::default()
    }
}

fn inv_to_wire(inv: &crate::types::Invalidation) -> wire::Invalidation {
    let quantity = inv.paths.len() as i32;
    wire::Invalidation {
        id: Some(inv.id.clone()),
        status: Some(inv.status.clone()),
        create_time: Some(inv.create_time.to_rfc3339()),
        invalidation_batch: Some(wire::InvalidationBatch {
            caller_reference: inv.caller_reference.clone(),
            paths: wire::Paths {
                quantity,
                items: if inv.paths.is_empty() {
                    None
                } else {
                    Some(wire::PathList::from(inv.paths.clone()))
                },
                ..Default::default()
            },
            ..Default::default()
        }),
        ..Default::default()
    }
}

fn kg_to_wire(kg: &crate::types::KeyGroupData) -> wire::KeyGroup {
    wire::KeyGroup {
        id: Some(kg.id.clone()),
        last_modified_time: Some(kg.last_modified_time.to_rfc3339()),
        key_group_config: Some(wire::KeyGroupConfig {
            name: kg.name.clone(),
            items: wire::PublicKeyIdList::from(kg.items.clone()),
            comment: kg.comment.clone(),
            ..Default::default()
        }),
        ..Default::default()
    }
}

fn oac_to_wire(oac: &crate::types::OriginAccessControlData) -> wire::OriginAccessControl {
    wire::OriginAccessControl {
        id: Some(oac.id.clone()),
        origin_access_control_config: Some(wire::OriginAccessControlConfig {
            name: oac.name.clone(),
            description: oac.description.clone(),
            origin_access_control_origin_type: oac.origin_access_control_origin_type.clone(),
            signing_behavior: oac.signing_behavior.clone(),
            signing_protocol: oac.signing_protocol.clone(),
            ..Default::default()
        }),
        ..Default::default()
    }
}

fn pk_to_wire(pk: &crate::types::PublicKeyData) -> wire::PublicKey {
    wire::PublicKey {
        id: Some(pk.id.clone()),
        created_time: Some(pk.created_time.to_rfc3339()),
        public_key_config: Some(wire::PublicKeyConfig {
            caller_reference: pk.caller_reference.clone(),
            name: pk.name.clone(),
            encoded_key: pk.encoded_key.clone(),
            comment: pk.comment.clone(),
            ..Default::default()
        }),
        ..Default::default()
    }
}

fn func_to_wire(func: &crate::types::CloudFrontFunctionData) -> wire::FunctionSummary {
    wire::FunctionSummary {
        name: Some(func.name.clone()),
        status: Some(func.status.clone()),
        function_config: Some(func.config.clone()),
        function_metadata: Some(wire::FunctionMetadata {
            function_a_r_n: Some(func.arn.clone()),
            created_time: Some(func.created_time.to_rfc3339()),
            last_modified_time: Some(func.last_modified_time.to_rfc3339()),
            stage: Some(if func.status == "DEPLOYED" {
                "LIVE".to_string()
            } else {
                "DEVELOPMENT".to_string()
            }),
        }),
    }
}

// ---- XML parsing helpers ----

fn extract_path_and_query(uri: &str) -> (String, String) {
    // FIX(terraform-e2e): when requests arrive from the local test server the URI is
    // "https://127.0.0.1:{port}/2020-05-31/..." which contains no "amazonaws.com".
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

fn parse_query_string(qs: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    if qs.is_empty() {
        return map;
    }
    for part in qs.split('&') {
        if let Some(eq) = part.find('=') {
            let key = &part[..eq];
            let value = &part[eq + 1..];
            map.insert(
                urlencoding::decode(key).unwrap_or_default().to_string(),
                urlencoding::decode(value).unwrap_or_default().to_string(),
            );
        } else if !part.is_empty() {
            map.insert(part.to_string(), String::new());
        }
    }
    map
}

fn extract_xml_value(xml: &str, tag: &str) -> Option<String> {
    use quick_xml::Reader;
    use quick_xml::events::Event;

    let mut reader = Reader::from_str(xml);
    let mut in_tag = false;
    let mut buf = String::new();
    loop {
        match reader.read_event() {
            Ok(Event::Start(e)) if e.local_name().as_ref() == tag.as_bytes() => {
                in_tag = true;
                buf.clear();
            }
            Ok(Event::Text(e)) if in_tag => {
                if let Ok(text) = e.unescape() {
                    buf.push_str(&text);
                }
            }
            Ok(Event::End(e)) if in_tag && e.local_name().as_ref() == tag.as_bytes() => {
                let trimmed = buf.trim().to_string();
                return if trimmed.is_empty() {
                    None
                } else {
                    Some(trimmed)
                };
            }
            Ok(Event::Eof) => break,
            Err(_) => break,
            _ => {}
        }
    }
    None
}

/// Extract all text values of a repeated XML element with the given tag name.
/// For example, `<Method>GET</Method><Method>HEAD</Method>` → `["GET", "HEAD"]`.
fn extract_xml_list_values(xml: &str, tag: &str) -> Vec<String> {
    use quick_xml::Reader;
    use quick_xml::events::Event;

    let mut reader = Reader::from_str(xml);
    let mut values = Vec::new();
    let mut in_tag = false;
    let mut buf = String::new();
    loop {
        match reader.read_event() {
            Ok(Event::Start(e)) if e.local_name().as_ref() == tag.as_bytes() => {
                in_tag = true;
                buf.clear();
            }
            Ok(Event::Text(e)) if in_tag => {
                if let Ok(text) = e.unescape() {
                    buf.push_str(&text);
                }
            }
            Ok(Event::End(e)) if in_tag && e.local_name().as_ref() == tag.as_bytes() => {
                let trimmed = buf.trim().to_string();
                if !trimmed.is_empty() {
                    values.push(trimmed);
                }
                buf.clear();
                in_tag = false;
            }
            Ok(Event::Eof) => break,
            Err(_) => break,
            _ => {}
        }
    }
    values
}

/// Parse AllowedMethods and CachedMethods from a CloudFront DistributionConfig XML body.
///
/// The provider sends:
///   <AllowedMethods>
///     <Items><Method>GET</Method><Method>HEAD</Method></Items>
///     <CachedMethods>
///       <Items><Method>GET</Method><Method>HEAD</Method></Items>
///     </CachedMethods>
///   </AllowedMethods>
///
/// A naive extract_xml_list_values(body, "Method") returns ALL <Method> tags (4 items
/// instead of 2).  This function uses a state-machine parser to scope extraction to
/// the correct sub-element.
fn parse_allowed_and_cached_methods(xml: &str) -> (Vec<String>, Vec<String>) {
    use quick_xml::Reader;
    use quick_xml::events::Event;

    let mut reader = Reader::from_str(xml);
    let mut allowed: Vec<String> = Vec::new();
    let mut cached: Vec<String> = Vec::new();

    // depth counters: we're "in AllowedMethods" when allowed_depth > 0,
    // "in CachedMethods" (inside AllowedMethods) when cached_depth > 0.
    let mut allowed_depth: u32 = 0;
    let mut cached_depth: u32 = 0;
    let mut in_method = false;
    let mut buf = String::new();

    loop {
        match reader.read_event() {
            Ok(Event::Start(e)) => {
                let local = e.local_name();
                let name = local.as_ref();
                if name == b"AllowedMethods" {
                    allowed_depth += 1;
                } else if name == b"CachedMethods" && allowed_depth > 0 {
                    cached_depth += 1;
                } else if name == b"Method" && allowed_depth > 0 {
                    in_method = true;
                    buf.clear();
                }
            }
            Ok(Event::Text(e)) if in_method => {
                if let Ok(text) = e.unescape() {
                    buf.push_str(&text);
                }
            }
            Ok(Event::End(e)) => {
                let local = e.local_name();
                let name = local.as_ref();
                if name == b"Method" && in_method {
                    let val = buf.trim().to_string();
                    if !val.is_empty() {
                        if cached_depth > 0 {
                            cached.push(val);
                        } else if allowed_depth > 0 {
                            allowed.push(val);
                        }
                    }
                    in_method = false;
                    buf.clear();
                } else if name == b"CachedMethods" && cached_depth > 0 {
                    cached_depth -= 1;
                } else if name == b"AllowedMethods" && allowed_depth > 0 {
                    allowed_depth -= 1;
                }
            }
            Ok(Event::Eof) => break,
            Err(_) => break,
            _ => {}
        }
    }

    (allowed, cached)
}

fn parse_origins(xml: &str) -> Vec<Origin> {
    use quick_xml::Reader;
    use quick_xml::events::Event;

    let mut reader = Reader::from_str(xml);
    let mut origins = Vec::new();
    let mut in_origin = false;
    let mut current_tag = String::new();
    let mut id = String::new();
    let mut domain_name = String::new();

    loop {
        match reader.read_event() {
            Ok(Event::Start(e)) => {
                let local = e.local_name();
                if local.as_ref() == b"Origin" {
                    in_origin = true;
                    id.clear();
                    domain_name.clear();
                } else if in_origin {
                    current_tag = String::from_utf8_lossy(local.as_ref()).to_string();
                }
            }
            Ok(Event::Text(e)) if in_origin && !current_tag.is_empty() => {
                if let Ok(text) = e.unescape() {
                    match current_tag.as_str() {
                        "Id" => id.push_str(&text),
                        "DomainName" => domain_name.push_str(&text),
                        _ => {}
                    }
                }
            }
            Ok(Event::End(e)) => {
                let local = e.local_name();
                if local.as_ref() == b"Origin" && in_origin {
                    if !id.is_empty() {
                        origins.push(Origin {
                            id: id.trim().to_string(),
                            domain_name: domain_name.trim().to_string(),
                        });
                    }
                    in_origin = false;
                }
                current_tag.clear();
            }
            Ok(Event::Eof) => break,
            Err(_) => break,
            _ => {}
        }
    }
    origins
}

fn parse_tags(xml: &str) -> HashMap<String, String> {
    use quick_xml::Reader;
    use quick_xml::events::Event;

    let mut reader = Reader::from_str(xml);
    let mut tags = HashMap::new();
    let mut in_tag_elem = false;
    let mut current_tag = String::new();
    let mut key = String::new();
    let mut value = String::new();

    loop {
        match reader.read_event() {
            Ok(Event::Start(e)) => {
                let local = e.local_name();
                if local.as_ref() == b"Tag" {
                    in_tag_elem = true;
                    key.clear();
                    value.clear();
                } else if in_tag_elem {
                    current_tag = String::from_utf8_lossy(local.as_ref()).to_string();
                }
            }
            Ok(Event::Text(e)) if in_tag_elem && !current_tag.is_empty() => {
                if let Ok(text) = e.unescape() {
                    match current_tag.as_str() {
                        "Key" => key.push_str(&text),
                        "Value" => value.push_str(&text),
                        _ => {}
                    }
                }
            }
            Ok(Event::End(e)) => {
                let local = e.local_name();
                if local.as_ref() == b"Tag" && in_tag_elem {
                    let k = key.trim().to_string();
                    let v = value.trim().to_string();
                    if !k.is_empty() {
                        tags.insert(k, v);
                    }
                    in_tag_elem = false;
                }
                current_tag.clear();
            }
            Ok(Event::Eof) => break,
            Err(_) => break,
            _ => {}
        }
    }
    tags
}

fn parse_tag_keys(xml: &str) -> Vec<String> {
    use quick_xml::Reader;
    use quick_xml::events::Event;

    let mut reader = Reader::from_str(xml);
    let mut keys = Vec::new();
    let mut in_key = false;
    let mut buf = String::new();

    loop {
        match reader.read_event() {
            Ok(Event::Start(e)) if e.local_name().as_ref() == b"Key" => {
                in_key = true;
                buf.clear();
            }
            Ok(Event::Text(e)) if in_key => {
                if let Ok(text) = e.unescape() {
                    buf.push_str(&text);
                }
            }
            Ok(Event::End(e)) if in_key && e.local_name().as_ref() == b"Key" => {
                let trimmed = buf.trim().to_string();
                if !trimmed.is_empty() {
                    keys.push(trimmed);
                }
                in_key = false;
            }
            Ok(Event::Eof) => break,
            Err(_) => break,
            _ => {}
        }
    }
    keys
}

fn parse_path_items(xml: &str) -> Vec<String> {
    // Parse <Paths><Items><Path>...</Path></Items></Paths>
    use quick_xml::Reader;
    use quick_xml::events::Event;

    let mut reader = Reader::from_str(xml);
    let mut paths = Vec::new();
    let mut in_path = false;
    let mut buf = String::new();

    loop {
        match reader.read_event() {
            Ok(Event::Start(e)) if e.local_name().as_ref() == b"Path" => {
                in_path = true;
                buf.clear();
            }
            Ok(Event::Text(e)) if in_path => {
                if let Ok(text) = e.unescape() {
                    buf.push_str(&text);
                }
            }
            Ok(Event::End(e)) if in_path && e.local_name().as_ref() == b"Path" => {
                let trimmed = buf.trim().to_string();
                if !trimmed.is_empty() {
                    paths.push(trimmed);
                }
                in_path = false;
            }
            Ok(Event::Eof) => break,
            Err(_) => break,
            _ => {}
        }
    }
    paths
}

fn parse_key_group_items(xml: &str) -> Vec<String> {
    // Parse <Items><PublicKey>...</PublicKey></Items> inside KeyGroupConfig
    use quick_xml::Reader;
    use quick_xml::events::Event;

    let mut reader = Reader::from_str(xml);
    let mut items = Vec::new();
    let mut in_items = false;
    let mut in_pk = false;
    let mut buf = String::new();

    loop {
        match reader.read_event() {
            Ok(Event::Start(e)) => {
                let local = e.local_name();
                if local.as_ref() == b"Items" {
                    in_items = true;
                } else if in_items && local.as_ref() == b"PublicKey" {
                    in_pk = true;
                    buf.clear();
                }
            }
            Ok(Event::Text(e)) if in_pk => {
                if let Ok(text) = e.unescape() {
                    buf.push_str(&text);
                }
            }
            Ok(Event::End(e)) => {
                let local = e.local_name();
                if in_pk && local.as_ref() == b"PublicKey" {
                    let trimmed = buf.trim().to_string();
                    if !trimmed.is_empty() {
                        items.push(trimmed);
                    }
                    in_pk = false;
                } else if local.as_ref() == b"Items" {
                    in_items = false;
                }
            }
            Ok(Event::Eof) => break,
            Err(_) => break,
            _ => {}
        }
    }
    items
}

/// Post-process quick_xml serialized output to fix the Vec serialization issue.
/// quick_xml serializes `Option<Vec<T>>` with `#[serde(rename = "Items")]` as
/// multiple `<Items>` elements instead of one `<Items>` wrapper containing child elements.
/// This function merges consecutive `<Tag>...</Tag>` fragments into one `<Tag>` container,
/// wrapping each original block's content in `<inner_tag>...</inner_tag>`.
fn fix_xml_list_items_wrapped(xml: &str, container_tag: &str, inner_tag: &str) -> String {
    let items_open = format!("<{container_tag}>");
    let items_close = format!("</{container_tag}>");

    let mut result = String::with_capacity(xml.len());
    let mut i = 0;

    while i < xml.len() {
        if xml[i..].starts_with(&items_open) {
            // Found opening tag. Collect all consecutive blocks.
            let mut inner_parts: Vec<String> = Vec::new();
            while i < xml.len() && xml[i..].starts_with(&items_open) {
                i += items_open.len();
                // Find matching close tag (with nesting support)
                let mut depth = 1;
                let start = i;
                while i < xml.len() && depth > 0 {
                    if xml[i..].starts_with(&items_open) {
                        depth += 1;
                        i += items_open.len();
                    } else if xml[i..].starts_with(&items_close) {
                        depth -= 1;
                        if depth == 0 {
                            inner_parts.push(xml[start..i].to_string());
                            i += items_close.len();
                        } else {
                            i += items_close.len();
                        }
                    } else {
                        i += 1;
                    }
                }
            }
            if inner_parts.len() > 1 || !inner_tag.is_empty() {
                result.push_str(&items_open);
                for part in &inner_parts {
                    if inner_tag.is_empty() {
                        result.push_str(part);
                    } else {
                        result.push_str(&format!("<{inner_tag}>{part}</{inner_tag}>"));
                    }
                }
                result.push_str(&items_close);
            } else if inner_parts.len() == 1 {
                result.push_str(&items_open);
                result.push_str(&inner_parts[0]);
                result.push_str(&items_close);
            } else {
                result.push_str(&items_open);
                result.push_str(&items_close);
            }
        } else {
            let ch = xml[i..].chars().next().unwrap();
            result.push(ch);
            i += ch.len_utf8();
        }
    }
    result
}

fn serialize_distribution_list(list: &wire::DistributionList) -> MockResponse {
    let body = quick_xml::se::to_string(list).unwrap_or_default();
    MockResponse::xml(200, body)
}

fn serialize_invalidation_list(list: &wire::InvalidationList) -> MockResponse {
    let body = quick_xml::se::to_string(list).unwrap_or_default();
    MockResponse::xml(200, body)
}

fn serialize_key_group_list(list: &wire::KeyGroupList) -> MockResponse {
    let body = quick_xml::se::to_string(list).unwrap_or_default();
    MockResponse::xml(200, body)
}

fn serialize_oac_list(list: &wire::OriginAccessControlList) -> MockResponse {
    let body = quick_xml::se::to_string(list).unwrap_or_default();
    MockResponse::xml(200, body)
}

fn serialize_public_key_list(list: &wire::PublicKeyList) -> MockResponse {
    let body = quick_xml::se::to_string(list).unwrap_or_default();
    MockResponse::xml(200, body)
}

/// Fix MockResponse body XML with the given Items/inner-tag fixup.
fn fix_resp_xml(resp: &MockResponse, container_tag: &str, inner_tag: &str) -> MockResponse {
    let body = std::str::from_utf8(&resp.body).unwrap_or("").to_string();
    let body = fix_xml_list_items_wrapped(&body, container_tag, inner_tag);
    MockResponse {
        status: resp.status,
        headers: resp.headers.clone(),
        body: body.into_bytes().into(),
    }
}

fn cloudfront_error_shape(err: &CloudFrontError) -> (u16, &'static str) {
    match err {
        CloudFrontError::DistributionAlreadyExists(_) => (409, "DistributionAlreadyExists"),
        CloudFrontError::NoSuchDistribution(_) => (404, "NoSuchDistribution"),
        CloudFrontError::PreconditionFailed => (412, "PreconditionFailed"),
        CloudFrontError::NoSuchInvalidation(_) => (404, "NoSuchInvalidation"),
        CloudFrontError::KeyGroupAlreadyExists(_) => (409, "KeyGroupAlreadyExists"),
        CloudFrontError::NoSuchResource(_) => (404, "NoSuchResource"),
        CloudFrontError::NoSuchOriginAccessControl(_) => (404, "NoSuchOriginAccessControl"),
        CloudFrontError::NoSuchPublicKey(_) => (404, "NoSuchPublicKey"),
        CloudFrontError::NoSuchCachePolicy(_) => (404, "NoSuchCachePolicy"),
        CloudFrontError::NoSuchOriginRequestPolicy(_) => (404, "NoSuchOriginRequestPolicy"),
        CloudFrontError::NoSuchResponseHeadersPolicy(_) => (404, "NoSuchResponseHeadersPolicy"),
        CloudFrontError::NoSuchFunctionExists(_) => (404, "NoSuchFunctionExists"),
        CloudFrontError::CloudFrontOriginAccessIdentityAlreadyExists(_) => {
            (409, "CloudFrontOriginAccessIdentityAlreadyExists")
        }
        CloudFrontError::NoSuchCloudFrontOriginAccessIdentity(_) => {
            (404, "NoSuchCloudFrontOriginAccessIdentity")
        }
        CloudFrontError::StreamingDistributionAlreadyExists => {
            (409, "StreamingDistributionAlreadyExists")
        }
        CloudFrontError::EntityAlreadyExists(_) => (409, "EntityAlreadyExists"),
        CloudFrontError::EntityNotFound(_) => (404, "EntityNotFound"),
        CloudFrontError::NoSuchStreamingDistribution(_) => (404, "NoSuchStreamingDistribution"),
        CloudFrontError::RealtimeLogConfigAlreadyExists(_) => {
            (409, "RealtimeLogConfigAlreadyExists")
        }
        CloudFrontError::NoSuchRealtimeLogConfig(_) => (404, "NoSuchRealtimeLogConfig"),
        CloudFrontError::InvalidArgument => (400, "InvalidArgument"),
        CloudFrontError::NoSuchFieldLevelEncryptionConfig(_) => {
            (404, "NoSuchFieldLevelEncryptionConfig")
        }
        CloudFrontError::NoSuchFieldLevelEncryptionProfile(_) => {
            (404, "NoSuchFieldLevelEncryptionProfile")
        }
        CloudFrontError::NoSuchAnycastIpList(_) => (404, "NoSuchAnycastIpList"),
        CloudFrontError::NoSuchConnectionFunction(_) => (404, "NoSuchConnectionFunction"),
        CloudFrontError::NoSuchConnectionGroup(_) => (404, "NoSuchConnectionGroup"),
        CloudFrontError::NoSuchContinuousDeploymentPolicy(_) => {
            (404, "NoSuchContinuousDeploymentPolicy")
        }
        CloudFrontError::NoSuchDistributionTenant(_) => (404, "NoSuchDistributionTenant"),
        CloudFrontError::NoSuchMonitoringSubscription(_) => (404, "NoSuchMonitoringSubscription"),
    }
}

fn cloudfront_error_response(err: &CloudFrontError) -> MockResponse {
    let (status, code) = cloudfront_error_shape(err);
    let xml = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<ErrorResponse xmlns="http://cloudfront.amazonaws.com/doc/2020-05-31/">
  <Error>
    <Type>Sender</Type>
    <Code>{}</Code>
    <Message>{}</Message>
  </Error>
  <RequestId>{}</RequestId>
</ErrorResponse>"#,
        xml_escape(code),
        xml_escape(&err.to_string()),
        uuid::Uuid::new_v4(),
    );
    MockResponse::xml(status, xml)
}

fn xml_error_response(status: u16, code: &str, message: &str) -> MockResponse {
    let xml = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<ErrorResponse xmlns="http://cloudfront.amazonaws.com/doc/2020-05-31/">
  <Error>
    <Type>Sender</Type>
    <Code>{}</Code>
    <Message>{}</Message>
  </Error>
  <RequestId>{}</RequestId>
</ErrorResponse>"#,
        xml_escape(code),
        xml_escape(message),
        uuid::Uuid::new_v4(),
    );
    MockResponse::xml(status, xml)
}

fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

fn oai_to_wire(oai: &crate::types::OaiData) -> wire::CloudFrontOriginAccessIdentity {
    wire::CloudFrontOriginAccessIdentity {
        id: Some(oai.id.clone()),
        s3_canonical_user_id: Some(oai.s3_canonical_user_id.clone()),
        cloud_front_origin_access_identity_config: Some(
            wire::CloudFrontOriginAccessIdentityConfig {
                caller_reference: oai.caller_reference.clone(),
                comment: oai.comment.clone(),
            },
        ),
    }
}

fn kvs_to_wire(kvs: &crate::types::KeyValueStoreData) -> wire::KeyValueStore {
    wire::KeyValueStore {
        name: Some(kvs.name.clone()),
        a_r_n: Some(kvs.arn.clone()),
        id: Some(kvs.id.clone()),
        comment: kvs.comment.clone(),
        last_modified_time: Some(kvs.last_modified_time.to_rfc3339()),
        status: Some(kvs.status.clone()),
    }
}

fn sd_to_wire(sd: &crate::types::StreamingDistributionData) -> wire::StreamingDistribution {
    wire::StreamingDistribution {
        id: Some(sd.id.clone()),
        a_r_n: Some(sd.arn.clone()),
        domain_name: Some(sd.domain_name.clone()),
        status: Some(sd.status.clone()),
        last_modified_time: Some(sd.last_modified_time.to_rfc3339()),
        streaming_distribution_config: Some(sd.config.clone()),
        active_trusted_signers: Some(wire::ActiveTrustedSigners {
            enabled: Some(false),
            quantity: Some(0),
            items: None,
        }),
        ..Default::default()
    }
}

fn ts_to_wire(ts: &crate::types::TrustStoreData) -> wire::TrustStore {
    wire::TrustStore {
        id: Some(ts.id.clone()),
        arn: Some(ts.arn.clone()),
        name: Some(ts.name.clone()),
        last_modified_time: Some(ts.last_modified_time.to_rfc3339()),
        status: Some(ts.status.clone()),
        number_of_ca_certificates: Some(ts.number_of_ca_certificates),
        ..Default::default()
    }
}

fn anycast_to_wire(list: &crate::types::AnycastIpListData) -> wire::AnycastIpList {
    wire::AnycastIpList {
        id: Some(list.id.clone()),
        arn: Some(list.arn.clone()),
        name: Some(list.name.clone()),
        status: Some(list.status.clone()),
        ip_count: Some(list.ip_count),
        ip_address_type: list.ip_address_type.clone(),
        last_modified_time: Some(list.last_modified_time.to_rfc3339()),
        ..Default::default()
    }
}

fn vpc_to_wire(vpc: &crate::types::VpcOriginData) -> wire::VpcOrigin {
    wire::VpcOrigin {
        id: Some(vpc.id.clone()),
        arn: Some(vpc.arn.clone()),
        account_id: Some(vpc.account_id.clone()),
        created_time: Some(vpc.created_time.to_rfc3339()),
        last_modified_time: Some(vpc.last_modified_time.to_rfc3339()),
        status: Some(vpc.status.clone()),
        vpc_origin_endpoint_config: Some(vpc.config.clone()),
    }
}

fn rtlc_to_wire(cfg: &crate::types::RealtimeLogConfigData) -> wire::RealtimeLogConfig {
    let end_points: Vec<wire::EndPoint> = cfg.end_points.clone();
    let fields: Vec<String> = cfg.fields.clone();
    wire::RealtimeLogConfig {
        name: Some(cfg.name.clone()),
        a_r_n: Some(cfg.arn.clone()),
        sampling_rate: Some(cfg.sampling_rate),
        end_points: if end_points.is_empty() {
            None
        } else {
            Some(wire::EndPointList::from(end_points))
        },
        fields: if fields.is_empty() {
            None
        } else {
            Some(wire::FieldList::from(fields))
        },
    }
}

fn conn_func_to_wire(
    func: &crate::types::ConnectionFunctionData,
) -> wire::ConnectionFunctionSummary {
    wire::ConnectionFunctionSummary {
        id: Some(func.id.clone()),
        name: Some(func.name.clone()),
        connection_function_arn: Some(func.arn.clone()),
        status: Some(func.status.clone()),
        stage: Some(func.stage.clone()),
        created_time: Some(func.created_time.to_rfc3339()),
        last_modified_time: Some(func.last_modified_time.to_rfc3339()),
        connection_function_config: Some(func.config.clone()),
    }
}

fn conn_group_to_wire(cg: &crate::types::ConnectionGroupData) -> wire::ConnectionGroup {
    wire::ConnectionGroup {
        id: Some(cg.id.clone()),
        name: Some(cg.name.clone()),
        arn: Some(cg.arn.clone()),
        status: Some(cg.status.clone()),
        routing_endpoint: Some(cg.routing_endpoint.clone()),
        enabled: Some(cg.enabled),
        ipv6_enabled: Some(cg.ipv6_enabled),
        is_default: Some(cg.is_default),
        anycast_ip_list_id: cg.anycast_ip_list_id.clone(),
        created_time: Some(cg.created_time.to_rfc3339()),
        last_modified_time: Some(cg.last_modified_time.to_rfc3339()),
        tags: None,
    }
}

fn conn_group_summary_to_wire(
    cg: &crate::types::ConnectionGroupData,
) -> wire::ConnectionGroupSummary {
    wire::ConnectionGroupSummary {
        id: Some(cg.id.clone()),
        name: Some(cg.name.clone()),
        arn: Some(cg.arn.clone()),
        status: Some(cg.status.clone()),
        routing_endpoint: Some(cg.routing_endpoint.clone()),
        enabled: Some(cg.enabled),
        is_default: Some(cg.is_default),
        anycast_ip_list_id: cg.anycast_ip_list_id.clone(),
        created_time: Some(cg.created_time.to_rfc3339()),
        last_modified_time: Some(cg.last_modified_time.to_rfc3339()),
        e_tag: Some(cg.etag.clone()),
    }
}

fn dist_tenant_to_wire(dt: &crate::types::DistributionTenantData) -> wire::DistributionTenant {
    wire::DistributionTenant {
        id: Some(dt.id.clone()),
        name: Some(dt.name.clone()),
        arn: Some(dt.arn.clone()),
        distribution_id: Some(dt.distribution_id.clone()),
        connection_group_id: dt.connection_group_id.clone(),
        enabled: Some(dt.enabled),
        status: Some(dt.status.clone()),
        domains: Some(dt.domains.clone()),
        customizations: dt.customizations.clone(),
        parameters: dt.parameters.clone(),
        created_time: Some(dt.created_time.to_rfc3339()),
        last_modified_time: Some(dt.last_modified_time.to_rfc3339()),
        ..Default::default()
    }
}

fn dist_tenant_summary_to_wire(
    dt: &crate::types::DistributionTenantData,
) -> wire::DistributionTenantSummary {
    wire::DistributionTenantSummary {
        id: Some(dt.id.clone()),
        name: Some(dt.name.clone()),
        arn: Some(dt.arn.clone()),
        distribution_id: Some(dt.distribution_id.clone()),
        connection_group_id: dt.connection_group_id.clone(),
        enabled: Some(dt.enabled),
        status: Some(dt.status.clone()),
        domains: Some(dt.domains.clone()),
        customizations: dt.customizations.clone(),
        created_time: Some(dt.created_time.to_rfc3339()),
        last_modified_time: Some(dt.last_modified_time.to_rfc3339()),
        e_tag: Some(dt.etag.clone()),
    }
}
