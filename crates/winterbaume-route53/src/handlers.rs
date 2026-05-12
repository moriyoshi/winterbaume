use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde::Serialize;
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, StatefulService,
    default_account_id,
};

use crate::state::{
    CidrCollectionChangeEntry, RecordChange, Route53Error, Route53State, canonical_resource_id,
};
use crate::types::{
    ResourceRecordSet, Route53DelegationSet, Route53HealthCheck, Route53HealthCheckConfig,
    Route53QueryLoggingConfig, TagResource, TrafficPolicyEntry, TrafficPolicyInstanceEntry,
};
use crate::views::Route53StateView;
use crate::wire;

/// Route 53 service handler that processes REST-XML protocol requests.
pub struct Route53Service {
    pub(crate) state: Arc<BackendState<Route53State>>,
    pub(crate) notifier: StateChangeNotifier<Route53StateView>,
}

impl Route53Service {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for Route53Service {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for Route53Service {
    fn service_name(&self) -> &str {
        "route53"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://route53\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl Route53Service {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = "us-east-1";
        let account_id = default_account_id();
        let state = self.state.get(account_id, region);

        let method = request.method.as_str();
        let path = extract_path(&request.uri);
        let query = parse_query_from_uri(&request.uri);
        let segments: Vec<&str> = path
            .trim_start_matches('/')
            .split('/')
            .filter(|segment| !segment.is_empty())
            .collect();
        let decoded_segments: Vec<String> = segments
            .iter()
            .map(|segment| percent_decode(segment))
            .collect();
        let segments: Vec<&str> = decoded_segments.iter().map(String::as_str).collect();

        let response = match (method, segments.as_slice()) {
            // POST /2013-04-01/hostedzone - CreateHostedZone
            ("POST", [_version, "hostedzone"]) => {
                self.handle_create_hosted_zone(&state, &request).await
            }
            // GET /2013-04-01/hostedzone - ListHostedZones
            ("GET", [_version, "hostedzone"]) => self.handle_list_hosted_zones(&state).await,
            // GET /2013-04-01/hostedzonesbyname - ListHostedZonesByName
            ("GET", [_version, "hostedzonesbyname"]) => {
                self.handle_list_hosted_zones_by_name(&state, &query).await
            }
            // GET /2013-04-01/hostedzonesbyvpc - ListHostedZonesByVPC
            ("GET", [_version, "hostedzonesbyvpc"]) => {
                self.handle_list_hosted_zones_by_vpc(&state, &query).await
            }
            // GET /2013-04-01/hostedzonecount - GetHostedZoneCount
            ("GET", [_version, "hostedzonecount"]) => {
                self.handle_get_hosted_zone_count(&state).await
            }
            // GET /2013-04-01/hostedzone/{Id} - GetHostedZone
            ("GET", [_version, "hostedzone", zone_id]) => {
                self.handle_get_hosted_zone(&state, zone_id).await
            }
            // POST /2013-04-01/hostedzone/{Id} - UpdateHostedZoneComment
            ("POST", [_version, "hostedzone", zone_id]) => {
                self.handle_update_hosted_zone_comment(&state, zone_id, &request)
                    .await
            }
            // DELETE /2013-04-01/hostedzone/{Id} - DeleteHostedZone
            ("DELETE", [_version, "hostedzone", zone_id]) => {
                self.handle_delete_hosted_zone(&state, zone_id).await
            }
            // POST /2013-04-01/hostedzone/{Id}/rrset - ChangeResourceRecordSets
            ("POST", [_version, "hostedzone", zone_id, "rrset"]) => {
                self.handle_change_resource_record_sets(&state, zone_id, &request)
                    .await
            }
            // GET /2013-04-01/hostedzone/{Id}/rrset - ListResourceRecordSets
            ("GET", [_version, "hostedzone", zone_id, "rrset"]) => {
                self.handle_list_resource_record_sets(&state, zone_id).await
            }
            // POST /2013-04-01/hostedzone/{Id}/associatevpc - AssociateVPCWithHostedZone
            ("POST", [_version, "hostedzone", zone_id, "associatevpc"]) => {
                self.handle_associate_vpc_with_hosted_zone(&state, zone_id, &request)
                    .await
            }
            // POST /2013-04-01/hostedzone/{Id}/disassociatevpc - DisassociateVPCFromHostedZone
            ("POST", [_version, "hostedzone", zone_id, "disassociatevpc"]) => {
                self.handle_disassociate_vpc_from_hosted_zone(&state, zone_id, &request)
                    .await
            }
            // GET /2013-04-01/hostedzone/{Id}/dnssec - GetDNSSEC
            ("GET", [_version, "hostedzone", zone_id, "dnssec"]) => {
                self.handle_get_dnssec(&state, zone_id).await
            }
            // POST /2013-04-01/tags/{ResourceType}/{ResourceId} - ChangeTagsForResource
            ("POST", [_version, "tags", resource_type, resource_id]) => {
                self.handle_change_tags_for_resource(&state, resource_type, resource_id, &request)
                    .await
            }
            // GET /2013-04-01/tags/{ResourceType}/{ResourceId} - ListTagsForResource
            ("GET", [_version, "tags", resource_type, resource_id]) => {
                self.handle_list_tags_for_resource(&state, resource_type, resource_id)
                    .await
            }
            // POST /2013-04-01/tags/{ResourceType} - ListTagsForResources
            ("POST", [_version, "tags", resource_type]) => {
                self.handle_list_tags_for_resources(&state, resource_type, &request)
                    .await
            }
            // POST /2013-04-01/healthcheck - CreateHealthCheck
            ("POST", [_version, "healthcheck"]) => {
                self.handle_create_health_check(&state, &request).await
            }
            // GET /2013-04-01/healthcheck - ListHealthChecks
            ("GET", [_version, "healthcheck"]) => self.handle_list_health_checks(&state).await,
            // GET /2013-04-01/healthcheck/{HealthCheckId} - GetHealthCheck
            ("GET", [_version, "healthcheck", health_check_id]) => {
                self.handle_get_health_check(&state, health_check_id).await
            }
            // POST /2013-04-01/healthcheck/{HealthCheckId} - UpdateHealthCheck
            ("POST", [_version, "healthcheck", health_check_id]) => {
                self.handle_update_health_check(&state, health_check_id, &request)
                    .await
            }
            // DELETE /2013-04-01/healthcheck/{HealthCheckId} - DeleteHealthCheck
            ("DELETE", [_version, "healthcheck", health_check_id]) => {
                self.handle_delete_health_check(&state, health_check_id)
                    .await
            }
            // GET /2013-04-01/healthcheck/{HealthCheckId}/status - GetHealthCheckStatus
            ("GET", [_version, "healthcheck", health_check_id, "status"]) => {
                self.handle_get_health_check_status(&state, health_check_id)
                    .await
            }
            // POST /2013-04-01/queryloggingconfig - CreateQueryLoggingConfig
            ("POST", [_version, "queryloggingconfig"]) => {
                self.handle_create_query_logging_config(&state, &request)
                    .await
            }
            // GET /2013-04-01/queryloggingconfig - ListQueryLoggingConfigs
            ("GET", [_version, "queryloggingconfig"]) => {
                self.handle_list_query_logging_configs(&state, &query).await
            }
            // GET /2013-04-01/queryloggingconfig/{Id} - GetQueryLoggingConfig
            ("GET", [_version, "queryloggingconfig", query_logging_config_id]) => {
                self.handle_get_query_logging_config(&state, query_logging_config_id)
                    .await
            }
            // DELETE /2013-04-01/queryloggingconfig/{Id} - DeleteQueryLoggingConfig
            ("DELETE", [_version, "queryloggingconfig", query_logging_config_id]) => {
                self.handle_delete_query_logging_config(&state, query_logging_config_id)
                    .await
            }
            // POST /2013-04-01/delegationset - CreateReusableDelegationSet
            ("POST", [_version, "delegationset"]) => {
                self.handle_create_reusable_delegation_set(&state, &request)
                    .await
            }
            // GET /2013-04-01/delegationset - ListReusableDelegationSets
            ("GET", [_version, "delegationset"]) => {
                self.handle_list_reusable_delegation_sets(&state).await
            }
            // GET /2013-04-01/delegationset/{Id} - GetReusableDelegationSet
            ("GET", [_version, "delegationset", delegation_set_id]) => {
                self.handle_get_reusable_delegation_set(&state, delegation_set_id)
                    .await
            }
            // DELETE /2013-04-01/delegationset/{Id} - DeleteReusableDelegationSet
            ("DELETE", [_version, "delegationset", delegation_set_id]) => {
                self.handle_delete_reusable_delegation_set(&state, delegation_set_id)
                    .await
            }
            // GET /2013-04-01/reusabledelegationsetlimit/{Id}/{Type} - GetReusableDelegationSetLimit
            (
                "GET",
                [
                    _version,
                    "reusabledelegationsetlimit",
                    delegation_set_id,
                    limit_type,
                ],
            ) => {
                self.handle_get_reusable_delegation_set_limit(&state, delegation_set_id, limit_type)
                    .await
            }
            // POST /2013-04-01/hostedzone/{Id}/enable-dnssec - EnableHostedZoneDNSSEC
            ("POST", [_version, "hostedzone", zone_id, "enable-dnssec"]) => {
                self.handle_enable_hosted_zone_dnssec(&state, zone_id).await
            }
            // POST /2013-04-01/hostedzone/{Id}/disable-dnssec - DisableHostedZoneDNSSEC
            ("POST", [_version, "hostedzone", zone_id, "disable-dnssec"]) => {
                self.handle_disable_hosted_zone_dnssec(&state, zone_id)
                    .await
            }
            // POST /2013-04-01/keysigningkey/{HostedZoneId}/{Name}/activate - ActivateKeySigningKey
            ("POST", [_version, "keysigningkey", zone_id, name, "activate"]) => {
                self.handle_activate_key_signing_key(&state, zone_id, name)
                    .await
            }
            // POST /2013-04-01/keysigningkey/{HostedZoneId}/{Name}/deactivate - DeactivateKeySigningKey
            ("POST", [_version, "keysigningkey", zone_id, name, "deactivate"]) => {
                self.handle_deactivate_key_signing_key(&state, zone_id, name)
                    .await
            }
            // POST /2013-04-01/keysigningkey - CreateKeySigningKey
            ("POST", [_version, "keysigningkey"]) => {
                self.handle_create_key_signing_key(&state, &request).await
            }
            // DELETE /2013-04-01/keysigningkey/{HostedZoneId}/{Name} - DeleteKeySigningKey
            ("DELETE", [_version, "keysigningkey", zone_id, name]) => {
                self.handle_delete_key_signing_key(&state, zone_id, name)
                    .await
            }
            // POST /2013-04-01/hostedzone/{Id}/authorizevpcassociation - CreateVPCAssociationAuthorization
            ("POST", [_version, "hostedzone", zone_id, "authorizevpcassociation"]) => {
                self.handle_create_vpc_association_authorization(&state, zone_id, &request)
                    .await
            }
            // POST /2013-04-01/hostedzone/{Id}/deauthorizevpcassociation - DeleteVPCAssociationAuthorization
            ("POST", [_version, "hostedzone", zone_id, "deauthorizevpcassociation"]) => {
                self.handle_delete_vpc_association_authorization(&state, zone_id, &request)
                    .await
            }
            // GET /2013-04-01/hostedzone/{Id}/authorizevpcassociation - ListVPCAssociationAuthorizations
            ("GET", [_version, "hostedzone", zone_id, "authorizevpcassociation"]) => {
                self.handle_list_vpc_association_authorizations(&state, zone_id)
                    .await
            }
            // POST /2013-04-01/cidrcollection - CreateCidrCollection
            ("POST", [_version, "cidrcollection"]) => {
                self.handle_create_cidr_collection(&state, &request).await
            }
            // GET /2013-04-01/cidrcollection - ListCidrCollections
            ("GET", [_version, "cidrcollection"]) => {
                self.handle_list_cidr_collections(&state).await
            }
            // DELETE /2013-04-01/cidrcollection/{Id} - DeleteCidrCollection
            ("DELETE", [_version, "cidrcollection", collection_id]) => {
                self.handle_delete_cidr_collection(&state, collection_id)
                    .await
            }
            // POST /2013-04-01/cidrcollection/{Id} - ChangeCidrCollection
            ("POST", [_version, "cidrcollection", collection_id]) => {
                self.handle_change_cidr_collection(&state, collection_id, &request)
                    .await
            }
            // GET /2013-04-01/cidrcollection/{Id} - ListCidrLocations
            ("GET", [_version, "cidrcollection", collection_id]) => {
                self.handle_list_cidr_locations(&state, collection_id).await
            }
            // GET /2013-04-01/cidrcollection/{Id}/cidrblocks - ListCidrBlocks
            ("GET", [_version, "cidrcollection", collection_id, "cidrblocks"]) => {
                self.handle_list_cidr_blocks(&state, collection_id, &query)
                    .await
            }
            // POST /2013-04-01/trafficpolicy - CreateTrafficPolicy
            ("POST", [_version, "trafficpolicy"]) => {
                self.handle_create_traffic_policy(&state, &request).await
            }
            // GET /2013-04-01/trafficpolicies - ListTrafficPolicies
            ("GET", [_version, "trafficpolicies"]) => {
                self.handle_list_traffic_policies(&state).await
            }
            // POST /2013-04-01/trafficpolicy/{Id} - CreateTrafficPolicyVersion
            ("POST", [_version, "trafficpolicy", policy_id]) => {
                self.handle_create_traffic_policy_version(&state, policy_id, &request)
                    .await
            }
            // GET /2013-04-01/trafficpolicy/{Id}/{Version} - GetTrafficPolicy
            ("GET", [_version, "trafficpolicy", policy_id, version]) => {
                self.handle_get_traffic_policy(&state, policy_id, version)
                    .await
            }
            // DELETE /2013-04-01/trafficpolicy/{Id}/{Version} - DeleteTrafficPolicy
            ("DELETE", [_version, "trafficpolicy", policy_id, version]) => {
                self.handle_delete_traffic_policy(&state, policy_id, version)
                    .await
            }
            // PUT or POST /2013-04-01/trafficpolicy/{Id}/{Version} - UpdateTrafficPolicyComment
            ("PUT" | "POST", [_version, "trafficpolicy", policy_id, version]) => {
                self.handle_update_traffic_policy_comment(&state, policy_id, version, &request)
                    .await
            }
            // GET /2013-04-01/trafficpolicies/{Id}/versions - ListTrafficPolicyVersions
            ("GET", [_version, "trafficpolicies", policy_id, "versions"]) => {
                self.handle_list_traffic_policy_versions(&state, policy_id)
                    .await
            }
            // POST /2013-04-01/trafficpolicyinstance - CreateTrafficPolicyInstance
            ("POST", [_version, "trafficpolicyinstance"]) => {
                self.handle_create_traffic_policy_instance(&state, &request)
                    .await
            }
            // GET /2013-04-01/trafficpolicyinstances - ListTrafficPolicyInstances
            ("GET", [_version, "trafficpolicyinstances"]) => {
                self.handle_list_traffic_policy_instances(&state, &query)
                    .await
            }
            // GET /2013-04-01/trafficpolicyinstances/hostedzone - ListTrafficPolicyInstancesByHostedZone
            ("GET", [_version, "trafficpolicyinstances", "hostedzone"]) => {
                self.handle_list_traffic_policy_instances_by_hosted_zone(&state, &query)
                    .await
            }
            // GET /2013-04-01/trafficpolicyinstances/trafficpolicy - ListTrafficPolicyInstancesByPolicy
            ("GET", [_version, "trafficpolicyinstances", "trafficpolicy"]) => {
                self.handle_list_traffic_policy_instances_by_policy(&state, &query)
                    .await
            }
            // GET /2013-04-01/trafficpolicyinstance/{Id} - GetTrafficPolicyInstance
            ("GET", [_version, "trafficpolicyinstance", instance_id]) => {
                self.handle_get_traffic_policy_instance(&state, instance_id)
                    .await
            }
            // DELETE /2013-04-01/trafficpolicyinstance/{Id} - DeleteTrafficPolicyInstance
            ("DELETE", [_version, "trafficpolicyinstance", instance_id]) => {
                self.handle_delete_traffic_policy_instance(&state, instance_id)
                    .await
            }
            // PUT or POST /2013-04-01/trafficpolicyinstance/{Id} - UpdateTrafficPolicyInstance
            ("PUT" | "POST", [_version, "trafficpolicyinstance", instance_id]) => {
                self.handle_update_traffic_policy_instance(&state, instance_id, &request)
                    .await
            }
            // GET /2013-04-01/trafficpolicyinstancecount - GetTrafficPolicyInstanceCount
            ("GET", [_version, "trafficpolicyinstancecount"]) => {
                self.handle_get_traffic_policy_instance_count(&state).await
            }
            // GET /2013-04-01/change/{Id} - GetChange
            ("GET", [_version, "change", _change_id]) => self.handle_get_change().await,
            // GET /2013-04-01/accountlimit/{Type} - GetAccountLimit
            ("GET", [_version, "accountlimit", limit_type]) => {
                self.handle_get_account_limit(limit_type).await
            }
            // GET /2013-04-01/checkeripranges - GetCheckerIpRanges
            ("GET", [_version, "checkeripranges"]) => self.handle_get_checker_ip_ranges().await,
            // GET /2013-04-01/geolocation - GetGeoLocation
            ("GET", [_version, "geolocation"]) => self.handle_get_geo_location(&query).await,
            // GET /2013-04-01/geolocations - ListGeoLocations
            ("GET", [_version, "geolocations"]) => self.handle_list_geo_locations().await,
            // GET /2013-04-01/healthcheckcount - GetHealthCheckCount
            ("GET", [_version, "healthcheckcount"]) => {
                self.handle_get_health_check_count(&state).await
            }
            // GET /2013-04-01/healthcheck/{HealthCheckId}/lastfailurereason - GetHealthCheckLastFailureReason
            (
                "GET",
                [
                    _version,
                    "healthcheck",
                    health_check_id,
                    "lastfailurereason",
                ],
            ) => {
                self.handle_get_health_check_last_failure_reason(&state, health_check_id)
                    .await
            }
            // GET /2013-04-01/hostedzonelimit/{Id}/{Type} - GetHostedZoneLimit
            ("GET", [_version, "hostedzonelimit", zone_id, limit_type]) => {
                self.handle_get_hosted_zone_limit(&state, zone_id, limit_type)
                    .await
            }
            // GET /2013-04-01/testdnsanswer - TestDNSAnswer
            ("GET", [_version, "testdnsanswer"]) => self.handle_test_dns_answer(&query).await,
            // POST/PUT /2013-04-01/hostedzone/{Id}/features - UpdateHostedZoneFeatures
            ("POST" | "PUT", [_version, "hostedzone", _zone_id, "features"]) => {
                self.handle_update_hosted_zone_features().await
            }
            _ => r53_error_response(&Route53Error::UnsupportedOperation {
                method: method.to_string(),
                path: path.to_string(),
            }),
        };
        if response.status >= 200 && response.status < 300 {
            self.notify_state_changed(account_id, region).await;
        }
        response
    }

    async fn handle_create_hosted_zone(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53State>>,
        request: &MockRequest,
    ) -> MockResponse {
        let query = HashMap::new();
        let input = match deserialize_route53_request(wire::deserialize_create_hosted_zone_request(
            request,
            &[],
            &query,
        )) {
            Ok(input) => input,
            Err(response) => return response,
        };

        if input.name.is_empty() || input.caller_reference.is_empty() {
            return r53_error_response(&Route53Error::MissingNameOrCallerReference);
        }

        let mut guard = state.write().await;
        match guard.create_hosted_zone(&input.name, &input.caller_reference) {
            Ok(mut zone) => {
                if let Some(config) = input.hosted_zone_config {
                    if config.comment.is_some() {
                        zone = match guard.update_hosted_zone_comment(&zone.id, config.comment) {
                            Ok(zone) => zone,
                            Err(err) => return r53_error_response(&err),
                        };
                    }
                }
                if let Some(vpc) = input.v_p_c.and_then(route53_vpc_from_wire) {
                    if let Err(err) =
                        guard.associate_vpc_with_hosted_zone(&zone.id, &vpc.vpc_id, &vpc.vpc_region)
                    {
                        return r53_error_response(&err);
                    }
                    zone = match guard.get_hosted_zone(&zone.id) {
                        Ok(zone) => zone,
                        Err(err) => return r53_error_response(&err),
                    };
                }
                let result = wire::CreateHostedZoneResponse {
                    change_info: Some(change_info()),
                    delegation_set: Some(to_model_delegation_set(&zone.delegation_set)),
                    hosted_zone: Some(to_model_hosted_zone(&zone)),
                    v_p_c: zone.vpcs.first().map(to_model_vpc),
                };
                let mut response = wire::serialize_create_hosted_zone_response(&result);
                response.headers.insert(
                    "Location",
                    format!("/2013-04-01{}", zone.id).parse().unwrap(),
                );
                response
            }
            Err(err) => r53_error_response(&err),
        }
    }

    async fn handle_get_hosted_zone(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53State>>,
        zone_id: &str,
    ) -> MockResponse {
        let guard = state.read().await;
        match guard.get_hosted_zone(zone_id) {
            Ok(zone) => {
                let result = wire::GetHostedZoneResponse {
                    delegation_set: Some(to_model_delegation_set(&zone.delegation_set)),
                    hosted_zone: Some(to_model_hosted_zone(&zone)),
                    v_p_cs: (!zone.vpcs.is_empty()).then(|| wire::VPCs {
                        items: zone.vpcs.iter().map(to_model_vpc).collect(),
                    }),
                };
                wire::serialize_get_hosted_zone_response(&result)
            }
            Err(err) => r53_error_response(&err),
        }
    }

    async fn handle_delete_hosted_zone(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53State>>,
        zone_id: &str,
    ) -> MockResponse {
        let mut guard = state.write().await;
        match guard.delete_hosted_zone(zone_id) {
            Ok(()) => {
                let result = wire::DeleteHostedZoneResponse {
                    change_info: Some(change_info()),
                };
                wire::serialize_delete_hosted_zone_response(&result)
            }
            Err(err) => r53_error_response(&err),
        }
    }

    async fn handle_list_hosted_zones(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53State>>,
    ) -> MockResponse {
        let guard = state.read().await;
        let zones = guard.list_hosted_zones();
        let result = wire::ListHostedZonesResponse {
            hosted_zones: Some(wire::HostedZones {
                items: zones.iter().map(to_model_hosted_zone).collect(),
            }),
            is_truncated: Some(false),
            marker: Some(String::new()),
            max_items: Some(100),
            next_marker: None,
        };
        wire::serialize_list_hosted_zones_response(&result)
    }

    async fn handle_list_hosted_zones_by_name(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53State>>,
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let dns_name = query.get("dnsname").map(String::as_str);
        let hosted_zone_id = query.get("hostedzoneid").map(String::as_str);
        let guard = state.read().await;
        let zones = guard.list_hosted_zones_by_name(dns_name, hosted_zone_id);

        let result = wire::ListHostedZonesByNameResponse {
            d_n_s_name: dns_name.map(ensure_trailing_dot),
            hosted_zone_id: hosted_zone_id.map(|id| canonical_resource_id("hostedzone", id)),
            hosted_zones: Some(wire::HostedZones {
                items: zones.iter().map(to_model_hosted_zone).collect(),
            }),
            is_truncated: Some(false),
            max_items: Some(
                query
                    .get("maxitems")
                    .and_then(|value| value.parse::<i32>().ok())
                    .unwrap_or(100),
            ),
            next_d_n_s_name: None,
            next_hosted_zone_id: None,
        };
        wire::serialize_list_hosted_zones_by_name_response(&result)
    }

    async fn handle_get_hosted_zone_count(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53State>>,
    ) -> MockResponse {
        let guard = state.read().await;
        let result = wire::GetHostedZoneCountResponse {
            hosted_zone_count: Some(guard.get_hosted_zone_count()),
        };
        wire::serialize_get_hosted_zone_count_response(&result)
    }

    async fn handle_update_hosted_zone_comment(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53State>>,
        zone_id: &str,
        request: &MockRequest,
    ) -> MockResponse {
        let query = HashMap::new();
        let input =
            match deserialize_route53_request(wire::deserialize_update_hosted_zone_comment_request(
                request,
                &[("Id", zone_id)],
                &query,
            )) {
                Ok(input) => input,
                Err(response) => return response,
            };
        let mut guard = state.write().await;
        match guard.update_hosted_zone_comment(zone_id, input.comment) {
            Ok(zone) => {
                let result = wire::UpdateHostedZoneCommentResponse {
                    hosted_zone: Some(to_model_hosted_zone(&zone)),
                };
                wire::serialize_update_hosted_zone_comment_response(&result)
            }
            Err(err) => r53_error_response(&err),
        }
    }

    async fn handle_change_resource_record_sets(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53State>>,
        zone_id: &str,
        request: &MockRequest,
    ) -> MockResponse {
        let query = HashMap::new();
        let input = match deserialize_route53_request(
            wire::deserialize_change_resource_record_sets_request(
                request,
                &[("HostedZoneId", zone_id)],
                &query,
            ),
        ) {
            Ok(input) => input,
            Err(response) => return response,
        };
        let changes = input
            .change_batch
            .changes
            .items
            .into_iter()
            .map(route53_record_change_from_wire)
            .collect::<Vec<_>>();
        if changes.is_empty() {
            return r53_error_response(&Route53Error::NoChangesInRequest);
        }

        let mut guard = state.write().await;
        match guard.change_resource_record_sets(zone_id, changes) {
            Ok(()) => {
                let result = wire::ChangeResourceRecordSetsResponse {
                    change_info: Some(change_info()),
                };
                wire::serialize_change_resource_record_sets_response(&result)
            }
            Err(err) => r53_error_response(&err),
        }
    }

    async fn handle_list_resource_record_sets(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53State>>,
        zone_id: &str,
    ) -> MockResponse {
        let guard = state.read().await;
        match guard.list_resource_record_sets(zone_id) {
            Ok(records) => {
                let result = wire::ListResourceRecordSetsResponse {
                    is_truncated: Some(false),
                    max_items: Some(100),
                    next_record_identifier: None,
                    next_record_name: None,
                    next_record_type: None,
                    resource_record_sets: Some(wire::ResourceRecordSets {
                        items: records.iter().map(to_model_record_set).collect(),
                    }),
                };
                wire::serialize_list_resource_record_sets_response(&result)
            }
            Err(err) => r53_error_response(&err),
        }
    }

    async fn handle_associate_vpc_with_hosted_zone(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53State>>,
        zone_id: &str,
        request: &MockRequest,
    ) -> MockResponse {
        let query = HashMap::new();
        let input = match deserialize_route53_request(
            wire::deserialize_associate_v_p_c_with_hosted_zone_request(
                request,
                &[("HostedZoneId", zone_id)],
                &query,
            ),
        ) {
            Ok(input) => input,
            Err(response) => return response,
        };
        let vpc_id = input.v_p_c.v_p_c_id.unwrap_or_default();
        let vpc_region = input.v_p_c.v_p_c_region.unwrap_or_default();

        if vpc_id.is_empty() {
            return r53_error_response(&Route53Error::MissingVpcId);
        }

        let mut guard = state.write().await;
        match guard.associate_vpc_with_hosted_zone(zone_id, &vpc_id, &vpc_region) {
            Ok(()) => {
                let result = wire::AssociateVPCWithHostedZoneResponse {
                    change_info: Some(change_info()),
                };
                wire::serialize_associate_v_p_c_with_hosted_zone_response(&result)
            }
            Err(err) => r53_error_response(&err),
        }
    }

    async fn handle_disassociate_vpc_from_hosted_zone(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53State>>,
        zone_id: &str,
        request: &MockRequest,
    ) -> MockResponse {
        let query = HashMap::new();
        let input = match deserialize_route53_request(
            wire::deserialize_disassociate_v_p_c_from_hosted_zone_request(
                request,
                &[("HostedZoneId", zone_id)],
                &query,
            ),
        ) {
            Ok(input) => input,
            Err(response) => return response,
        };
        let vpc_id = input.v_p_c.v_p_c_id.unwrap_or_default();

        if vpc_id.is_empty() {
            return r53_error_response(&Route53Error::MissingVpcId);
        }

        let mut guard = state.write().await;
        match guard.disassociate_vpc_from_hosted_zone(zone_id, &vpc_id) {
            Ok(()) => {
                let result = wire::DisassociateVPCFromHostedZoneResponse {
                    change_info: Some(change_info()),
                };
                wire::serialize_disassociate_v_p_c_from_hosted_zone_response(&result)
            }
            Err(err) => r53_error_response(&err),
        }
    }

    async fn handle_get_dnssec(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53State>>,
        zone_id: &str,
    ) -> MockResponse {
        let guard = state.read().await;
        match guard.get_hosted_zone(zone_id) {
            Ok(_) => {
                let keys: Vec<wire::KeySigningKey> = guard
                    .list_key_signing_keys(zone_id)
                    .iter()
                    .map(to_model_key_signing_key)
                    .collect();
                let dnssec_enabled = guard
                    .dnssec_enabled
                    .get(&crate::state::normalize_hosted_zone_id_pub(zone_id))
                    .copied()
                    .unwrap_or(false);
                let serve_signature = if dnssec_enabled {
                    "SIGNING"
                } else {
                    "NOT_SIGNING"
                };
                let result = wire::GetDNSSECResponse {
                    key_signing_keys: Some(wire::KeySigningKeys { items: keys }),
                    status: Some(wire::DNSSECStatus {
                        serve_signature: Some(serve_signature.to_string()),
                        status_message: None,
                    }),
                };
                wire::serialize_get_d_n_s_s_e_c_response(&result)
            }
            Err(err) => r53_error_response(&err),
        }
    }

    async fn handle_list_hosted_zones_by_vpc(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53State>>,
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let vpc_id = match query.get("vpcid").or_else(|| query.get("vpcId")) {
            Some(value) => value.as_str(),
            None => {
                return r53_error_response(&Route53Error::MissingVpcIdParameter);
            }
        };
        let vpc_region = query
            .get("vpcregion")
            .or_else(|| query.get("vpcRegion"))
            .map(String::as_str)
            .unwrap_or("us-east-1");

        let guard = state.read().await;
        let zones = guard.list_hosted_zones_by_vpc(vpc_id, vpc_region);
        let result = wire::ListHostedZonesByVPCResponse {
            hosted_zone_summaries: Some(wire::HostedZoneSummaries {
                items: zones
                    .iter()
                    .map(|zone| wire::HostedZoneSummary {
                        hosted_zone_id: Some(zone.id.clone()),
                        name: Some(zone.name.clone()),
                        owner: Some(wire::HostedZoneOwner {
                            owning_account: Some(default_account_id().to_string()),
                            owning_service: None,
                        }),
                    })
                    .collect(),
            }),
            max_items: Some(
                query
                    .get("maxitems")
                    .and_then(|value| value.parse::<i32>().ok())
                    .unwrap_or(100),
            ),
            next_token: None,
        };
        wire::serialize_list_hosted_zones_by_v_p_c_response(&result)
    }

    async fn handle_change_tags_for_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53State>>,
        resource_type: &str,
        resource_id: &str,
        request: &MockRequest,
    ) -> MockResponse {
        let query = HashMap::new();
        let input =
            match deserialize_route53_request(wire::deserialize_change_tags_for_resource_request(
                request,
                &[("ResourceType", resource_type), ("ResourceId", resource_id)],
                &query,
            )) {
                Ok(input) => input,
                Err(response) => return response,
            };
        let add_tags = input
            .add_tags
            .map(|tags| {
                tags.items
                    .into_iter()
                    .filter_map(|tag| tag.key.map(|key| (key, tag.value.unwrap_or_default())))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        let remove_tag_keys = input
            .remove_tag_keys
            .map(|keys| keys.items)
            .unwrap_or_default();
        let mut guard = state.write().await;
        match guard.change_tags_for_resource(
            resource_type,
            resource_id,
            &add_tags,
            &remove_tag_keys,
        ) {
            Ok(()) => wire::serialize_change_tags_for_resource_response(),
            Err(err) => r53_error_response(&err),
        }
    }

    async fn handle_list_tags_for_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53State>>,
        resource_type: &str,
        resource_id: &str,
    ) -> MockResponse {
        let guard = state.read().await;
        match guard.list_tags_for_resource(resource_type, resource_id) {
            Ok(tags) => {
                let result = wire::ListTagsForResourceResponse {
                    resource_tag_set: Some(to_model_resource_tag_set(
                        resource_type,
                        resource_id,
                        &tags,
                    )),
                };
                wire::serialize_list_tags_for_resource_response(&result)
            }
            Err(err) => r53_error_response(&err),
        }
    }

    async fn handle_list_tags_for_resources(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53State>>,
        resource_type: &str,
        request: &MockRequest,
    ) -> MockResponse {
        let query = HashMap::new();
        let input =
            match deserialize_route53_request(wire::deserialize_list_tags_for_resources_request(
                request,
                &[("ResourceType", resource_type)],
                &query,
            )) {
                Ok(input) => input,
                Err(response) => return response,
            };
        let resource_ids = input.resource_ids.items;
        let guard = state.read().await;
        match guard.list_tags_for_resources(resource_type, &resource_ids) {
            Ok(resources) => {
                let result = wire::ListTagsForResourcesResponse {
                    resource_tag_sets: Some(wire::ResourceTagSetList {
                        items: resources.iter().map(to_model_tag_resource).collect(),
                    }),
                };
                wire::serialize_list_tags_for_resources_response(&result)
            }
            Err(err) => r53_error_response(&err),
        }
    }

    async fn handle_create_health_check(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53State>>,
        request: &MockRequest,
    ) -> MockResponse {
        let query = HashMap::new();
        let input = match deserialize_route53_request(
            wire::deserialize_create_health_check_request(request, &[], &query),
        ) {
            Ok(input) => input,
            Err(response) => return response,
        };
        let config = route53_health_check_config_from_wire(input.health_check_config);
        if input.caller_reference.is_empty() || config.type_.is_empty() {
            return r53_error_response(&Route53Error::MissingHealthCheckFields);
        }

        let mut guard = state.write().await;
        match guard.create_health_check(&input.caller_reference, config) {
            Ok(health_check) => {
                let result = wire::CreateHealthCheckResponse {
                    health_check: Some(to_model_health_check(&health_check)),
                };
                let mut response = wire::serialize_create_health_check_response(&result);
                add_location_header(
                    &mut response,
                    &format!("/2013-04-01/healthcheck/{}", health_check.id),
                );
                response
            }
            Err(err) => r53_error_response(&err),
        }
    }

    async fn handle_get_health_check(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53State>>,
        health_check_id: &str,
    ) -> MockResponse {
        let guard = state.read().await;
        match guard.get_health_check(health_check_id) {
            Ok(health_check) => {
                let result = wire::GetHealthCheckResponse {
                    health_check: Some(to_model_health_check(&health_check)),
                };
                wire::serialize_get_health_check_response(&result)
            }
            Err(err) => r53_error_response(&err),
        }
    }

    async fn handle_get_health_check_status(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53State>>,
        health_check_id: &str,
    ) -> MockResponse {
        let guard = state.read().await;
        match guard.get_health_check(health_check_id) {
            Ok(health_check) => {
                let result = wire::GetHealthCheckStatusResponse {
                    health_check_observations: Some(wire::HealthCheckObservations {
                        items: vec![wire::HealthCheckObservation {
                            i_p_address: health_check.config.ip_address.clone(),
                            region: Some(
                                health_check
                                    .config
                                    .regions
                                    .first()
                                    .cloned()
                                    .unwrap_or_else(|| "us-east-1".to_string()),
                            ),
                            status_report: Some(wire::StatusReport {
                                checked_time: Some(iso_timestamp()),
                                status: Some(
                                    if health_check.config.disabled == Some(true) {
                                        "Unhealthy"
                                    } else {
                                        "Healthy"
                                    }
                                    .to_string(),
                                ),
                            }),
                        }],
                    }),
                };
                wire::serialize_get_health_check_status_response(&result)
            }
            Err(err) => r53_error_response(&err),
        }
    }

    async fn handle_list_health_checks(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53State>>,
    ) -> MockResponse {
        let guard = state.read().await;
        let health_checks = guard.list_health_checks();
        let result = wire::ListHealthChecksResponse {
            health_checks: Some(wire::HealthChecks {
                items: health_checks.iter().map(to_model_health_check).collect(),
            }),
            is_truncated: Some(false),
            marker: Some(String::new()),
            max_items: Some(100),
            next_marker: None,
        };
        wire::serialize_list_health_checks_response(&result)
    }

    async fn handle_update_health_check(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53State>>,
        health_check_id: &str,
        request: &MockRequest,
    ) -> MockResponse {
        let query = HashMap::new();
        let input =
            match deserialize_route53_request(wire::deserialize_update_health_check_request(
                request,
                &[("HealthCheckId", health_check_id)],
                &query,
            )) {
                Ok(input) => input,
                Err(response) => return response,
            };
        let updates = route53_health_check_updates_from_wire(input);
        let mut guard = state.write().await;
        match guard.update_health_check(health_check_id, updates) {
            Ok(health_check) => {
                let result = wire::UpdateHealthCheckResponse {
                    health_check: Some(to_model_health_check(&health_check)),
                };
                wire::serialize_update_health_check_response(&result)
            }
            Err(err) => r53_error_response(&err),
        }
    }

    async fn handle_delete_health_check(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53State>>,
        health_check_id: &str,
    ) -> MockResponse {
        let mut guard = state.write().await;
        match guard.delete_health_check(health_check_id) {
            Ok(()) => wire::serialize_delete_health_check_response(),
            Err(err) => r53_error_response(&err),
        }
    }

    async fn handle_create_query_logging_config(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53State>>,
        request: &MockRequest,
    ) -> MockResponse {
        let query = HashMap::new();
        let input = match deserialize_route53_request(
            wire::deserialize_create_query_logging_config_request(request, &[], &query),
        ) {
            Ok(input) => input,
            Err(response) => return response,
        };
        if input.hosted_zone_id.is_empty() || input.cloud_watch_logs_log_group_arn.is_empty() {
            return r53_error_response(&Route53Error::MissingQueryLoggingConfigFields);
        }

        let mut guard = state.write().await;
        match guard.create_query_logging_config(
            &input.hosted_zone_id,
            &input.cloud_watch_logs_log_group_arn,
        ) {
            Ok(config) => {
                let result = wire::CreateQueryLoggingConfigResponse {
                    query_logging_config: Some(to_model_query_logging_config(&config)),
                };
                let mut response = wire::serialize_create_query_logging_config_response(&result);
                add_location_header(
                    &mut response,
                    &format!("/2013-04-01/queryloggingconfig/{}", config.id),
                );
                response
            }
            Err(err) => r53_error_response(&err),
        }
    }

    async fn handle_get_query_logging_config(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53State>>,
        query_logging_config_id: &str,
    ) -> MockResponse {
        let guard = state.read().await;
        match guard.get_query_logging_config(query_logging_config_id) {
            Ok(config) => {
                let result = wire::GetQueryLoggingConfigResponse {
                    query_logging_config: Some(to_model_query_logging_config(&config)),
                };
                wire::serialize_get_query_logging_config_response(&result)
            }
            Err(err) => r53_error_response(&err),
        }
    }

    async fn handle_list_query_logging_configs(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53State>>,
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let guard = state.read().await;
        let configs =
            guard.list_query_logging_configs(query.get("hostedzoneid").map(String::as_str));
        let result = wire::ListQueryLoggingConfigsResponse {
            next_token: None,
            query_logging_configs: Some(wire::QueryLoggingConfigs {
                items: configs.iter().map(to_model_query_logging_config).collect(),
            }),
        };
        wire::serialize_list_query_logging_configs_response(&result)
    }

    async fn handle_delete_query_logging_config(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53State>>,
        query_logging_config_id: &str,
    ) -> MockResponse {
        let mut guard = state.write().await;
        match guard.delete_query_logging_config(query_logging_config_id) {
            Ok(()) => wire::serialize_delete_query_logging_config_response(),
            Err(err) => r53_error_response(&err),
        }
    }

    async fn handle_create_reusable_delegation_set(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53State>>,
        request: &MockRequest,
    ) -> MockResponse {
        let query = HashMap::new();
        let input = match deserialize_route53_request(
            wire::deserialize_create_reusable_delegation_set_request(request, &[], &query),
        ) {
            Ok(input) => input,
            Err(response) => return response,
        };
        if input.caller_reference.is_empty() {
            return r53_error_response(&Route53Error::MissingCallerReference);
        }

        let mut guard = state.write().await;
        match guard.create_reusable_delegation_set(
            &input.caller_reference,
            input.hosted_zone_id.as_deref(),
        ) {
            Ok(delegation_set) => {
                let result = wire::CreateReusableDelegationSetResponse {
                    delegation_set: Some(to_model_delegation_set(&delegation_set)),
                };
                let mut response = wire::serialize_create_reusable_delegation_set_response(&result);
                add_location_header(
                    &mut response,
                    &format!("/2013-04-01/delegationset/{}", delegation_set.id),
                );
                response
            }
            Err(err) => r53_error_response(&err),
        }
    }

    async fn handle_get_reusable_delegation_set(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53State>>,
        delegation_set_id: &str,
    ) -> MockResponse {
        let guard = state.read().await;
        match guard.get_reusable_delegation_set(delegation_set_id) {
            Ok(delegation_set) => {
                let result = wire::GetReusableDelegationSetResponse {
                    delegation_set: Some(to_model_delegation_set(&delegation_set)),
                };
                wire::serialize_get_reusable_delegation_set_response(&result)
            }
            Err(err) => r53_error_response(&err),
        }
    }

    async fn handle_list_reusable_delegation_sets(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53State>>,
    ) -> MockResponse {
        let guard = state.read().await;
        let result = wire::ListReusableDelegationSetsResponse {
            delegation_sets: Some(wire::DelegationSets {
                items: guard
                    .list_reusable_delegation_sets()
                    .iter()
                    .map(to_model_delegation_set)
                    .collect(),
            }),
            is_truncated: Some(false),
            marker: Some(String::new()),
            max_items: Some(100),
            next_marker: None,
        };
        wire::serialize_list_reusable_delegation_sets_response(&result)
    }

    async fn handle_delete_reusable_delegation_set(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53State>>,
        delegation_set_id: &str,
    ) -> MockResponse {
        let mut guard = state.write().await;
        match guard.delete_reusable_delegation_set(delegation_set_id) {
            Ok(()) => wire::serialize_delete_reusable_delegation_set_response(),
            Err(err) => r53_error_response(&err),
        }
    }

    async fn handle_get_reusable_delegation_set_limit(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53State>>,
        delegation_set_id: &str,
        limit_type: &str,
    ) -> MockResponse {
        let guard = state.read().await;
        match guard.get_reusable_delegation_set(delegation_set_id) {
            Ok(_) => {
                let result = wire::GetReusableDelegationSetLimitResponse {
                    count: Some(0),
                    limit: Some(wire::ReusableDelegationSetLimit {
                        r#type: Some(limit_type.to_string()),
                        value: Some(100),
                    }),
                };
                wire::serialize_get_reusable_delegation_set_limit_response(&result)
            }
            Err(err) => r53_error_response(&err),
        }
    }

    // --- DNSSEC / Key Signing Key handlers ---

    async fn handle_enable_hosted_zone_dnssec(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53State>>,
        zone_id: &str,
    ) -> MockResponse {
        let mut guard = state.write().await;
        match guard.enable_hosted_zone_dnssec(zone_id) {
            Ok(()) => {
                let result = wire::EnableHostedZoneDNSSECResponse {
                    change_info: Some(change_info()),
                };
                wire::serialize_enable_hosted_zone_d_n_s_s_e_c_response(&result)
            }
            Err(err) => r53_error_response(&err),
        }
    }

    async fn handle_disable_hosted_zone_dnssec(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53State>>,
        zone_id: &str,
    ) -> MockResponse {
        let mut guard = state.write().await;
        match guard.disable_hosted_zone_dnssec(zone_id) {
            Ok(()) => {
                let result = wire::DisableHostedZoneDNSSECResponse {
                    change_info: Some(change_info()),
                };
                wire::serialize_disable_hosted_zone_d_n_s_s_e_c_response(&result)
            }
            Err(err) => r53_error_response(&err),
        }
    }

    async fn handle_create_key_signing_key(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53State>>,
        request: &MockRequest,
    ) -> MockResponse {
        let query = HashMap::new();
        let input = match deserialize_route53_request(
            wire::deserialize_create_key_signing_key_request(request, &[], &query),
        ) {
            Ok(input) => input,
            Err(response) => return response,
        };
        let mut guard = state.write().await;
        match guard.create_key_signing_key(
            &input.hosted_zone_id,
            &input.name,
            &input.key_management_service_arn,
            &input.status,
        ) {
            Ok(entry) => {
                let result = wire::CreateKeySigningKeyResponse {
                    change_info: Some(change_info()),
                    key_signing_key: Some(to_model_key_signing_key(&entry)),
                };
                let mut response = wire::serialize_create_key_signing_key_response(&result);
                add_location_header(
                    &mut response,
                    &format!(
                        "/2013-04-01/keysigningkey/{}/{}",
                        entry.hosted_zone_id, entry.name
                    ),
                );
                response
            }
            Err(err) => r53_error_response(&err),
        }
    }

    async fn handle_activate_key_signing_key(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53State>>,
        zone_id: &str,
        name: &str,
    ) -> MockResponse {
        let mut guard = state.write().await;
        match guard.activate_key_signing_key(zone_id, name) {
            Ok(()) => {
                let result = wire::ActivateKeySigningKeyResponse {
                    change_info: Some(change_info()),
                };
                wire::serialize_activate_key_signing_key_response(&result)
            }
            Err(err) => r53_error_response(&err),
        }
    }

    async fn handle_deactivate_key_signing_key(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53State>>,
        zone_id: &str,
        name: &str,
    ) -> MockResponse {
        let mut guard = state.write().await;
        match guard.deactivate_key_signing_key(zone_id, name) {
            Ok(()) => {
                let result = wire::DeactivateKeySigningKeyResponse {
                    change_info: Some(change_info()),
                };
                wire::serialize_deactivate_key_signing_key_response(&result)
            }
            Err(err) => r53_error_response(&err),
        }
    }

    async fn handle_delete_key_signing_key(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53State>>,
        zone_id: &str,
        name: &str,
    ) -> MockResponse {
        let mut guard = state.write().await;
        match guard.delete_key_signing_key(zone_id, name) {
            Ok(()) => {
                let result = wire::DeleteKeySigningKeyResponse {
                    change_info: Some(change_info()),
                };
                wire::serialize_delete_key_signing_key_response(&result)
            }
            Err(err) => r53_error_response(&err),
        }
    }

    // --- VPC Association Authorization handlers ---

    async fn handle_create_vpc_association_authorization(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53State>>,
        zone_id: &str,
        request: &MockRequest,
    ) -> MockResponse {
        let query = HashMap::new();
        let input = match deserialize_route53_request(
            wire::deserialize_create_v_p_c_association_authorization_request(
                request,
                &[("HostedZoneId", zone_id)],
                &query,
            ),
        ) {
            Ok(input) => input,
            Err(response) => return response,
        };
        let vpc_id = input.v_p_c.v_p_c_id.unwrap_or_default();
        let vpc_region = input.v_p_c.v_p_c_region.unwrap_or_default();
        let mut guard = state.write().await;
        match guard.create_vpc_association_authorization(zone_id, &vpc_id, &vpc_region) {
            Ok(auth) => {
                let result = wire::CreateVPCAssociationAuthorizationResponse {
                    hosted_zone_id: Some(auth.hosted_zone_id),
                    v_p_c: Some(wire::VPC {
                        v_p_c_id: Some(auth.vpc_id),
                        v_p_c_region: Some(auth.vpc_region),
                    }),
                };
                wire::serialize_create_v_p_c_association_authorization_response(&result)
            }
            Err(err) => r53_error_response(&err),
        }
    }

    async fn handle_delete_vpc_association_authorization(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53State>>,
        zone_id: &str,
        request: &MockRequest,
    ) -> MockResponse {
        let query = HashMap::new();
        let input = match deserialize_route53_request(
            wire::deserialize_delete_v_p_c_association_authorization_request(
                request,
                &[("HostedZoneId", zone_id)],
                &query,
            ),
        ) {
            Ok(input) => input,
            Err(response) => return response,
        };
        let vpc_id = input.v_p_c.v_p_c_id.unwrap_or_default();
        let mut guard = state.write().await;
        match guard.delete_vpc_association_authorization(zone_id, &vpc_id) {
            Ok(()) => wire::serialize_delete_v_p_c_association_authorization_response(),
            Err(err) => r53_error_response(&err),
        }
    }

    async fn handle_list_vpc_association_authorizations(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53State>>,
        zone_id: &str,
    ) -> MockResponse {
        let guard = state.read().await;
        let auths = guard.list_vpc_association_authorizations(zone_id);
        let zone_key = crate::state::normalize_hosted_zone_id_pub(zone_id);
        let result = wire::ListVPCAssociationAuthorizationsResponse {
            hosted_zone_id: Some(zone_key),
            next_token: None,
            v_p_cs: Some(wire::VPCs {
                items: auths
                    .iter()
                    .map(|a| wire::VPC {
                        v_p_c_id: Some(a.vpc_id.clone()),
                        v_p_c_region: Some(a.vpc_region.clone()),
                    })
                    .collect(),
            }),
        };
        wire::serialize_list_v_p_c_association_authorizations_response(&result)
    }

    // --- CIDR Collection handlers ---

    async fn handle_create_cidr_collection(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53State>>,
        request: &MockRequest,
    ) -> MockResponse {
        let query = HashMap::new();
        let input = match deserialize_route53_request(
            wire::deserialize_create_cidr_collection_request(request, &[], &query),
        ) {
            Ok(input) => input,
            Err(response) => return response,
        };
        let mut guard = state.write().await;
        match guard.create_cidr_collection(&input.name, &input.caller_reference) {
            Ok(entry) => {
                let result = wire::CreateCidrCollectionResponse {
                    collection: Some(wire::CidrCollection {
                        arn: Some(entry.arn.clone()),
                        id: Some(entry.id.clone()),
                        name: Some(entry.name.clone()),
                        version: Some(entry.version),
                    }),
                };
                let mut response = wire::serialize_create_cidr_collection_response(&result);
                add_location_header(
                    &mut response,
                    &format!("/2013-04-01/cidrcollection/{}", entry.id),
                );
                response
            }
            Err(err) => r53_error_response(&err),
        }
    }

    async fn handle_delete_cidr_collection(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53State>>,
        collection_id: &str,
    ) -> MockResponse {
        let mut guard = state.write().await;
        match guard.delete_cidr_collection(collection_id) {
            Ok(()) => wire::serialize_delete_cidr_collection_response(),
            Err(err) => r53_error_response(&err),
        }
    }

    async fn handle_change_cidr_collection(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53State>>,
        collection_id: &str,
        request: &MockRequest,
    ) -> MockResponse {
        let query = HashMap::new();
        let input =
            match deserialize_route53_request(wire::deserialize_change_cidr_collection_request(
                request,
                &[("Id", collection_id)],
                &query,
            )) {
                Ok(input) => input,
                Err(response) => return response,
            };
        let changes: Vec<CidrCollectionChangeEntry> = input
            .changes
            .items
            .into_iter()
            .map(|c| CidrCollectionChangeEntry {
                action: c.action,
                location_name: c.location_name,
                cidr_list: c.cidr_list.items,
            })
            .collect();
        let mut guard = state.write().await;
        match guard.change_cidr_collection(collection_id, changes) {
            Ok(id) => {
                let result = wire::ChangeCidrCollectionResponse { id: Some(id) };
                wire::serialize_change_cidr_collection_response(&result)
            }
            Err(err) => r53_error_response(&err),
        }
    }

    async fn handle_list_cidr_collections(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53State>>,
    ) -> MockResponse {
        let guard = state.read().await;
        let collections = guard.list_cidr_collections();
        let result = wire::ListCidrCollectionsResponse {
            cidr_collections: Some(wire::CollectionSummaries {
                items: collections
                    .iter()
                    .map(|c| wire::CollectionSummary {
                        arn: Some(c.arn.clone()),
                        id: Some(c.id.clone()),
                        name: Some(c.name.clone()),
                        version: Some(c.version),
                    })
                    .collect(),
            }),
            next_token: None,
        };
        wire::serialize_list_cidr_collections_response(&result)
    }

    async fn handle_list_cidr_locations(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53State>>,
        collection_id: &str,
    ) -> MockResponse {
        let guard = state.read().await;
        match guard.list_cidr_locations(collection_id) {
            Ok(locations) => {
                let result = wire::ListCidrLocationsResponse {
                    cidr_locations: Some(wire::LocationSummaries {
                        items: locations
                            .into_iter()
                            .map(|name| wire::LocationSummary {
                                location_name: Some(name),
                            })
                            .collect(),
                    }),
                    next_token: None,
                };
                wire::serialize_list_cidr_locations_response(&result)
            }
            Err(err) => r53_error_response(&err),
        }
    }

    async fn handle_list_cidr_blocks(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53State>>,
        collection_id: &str,
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let location_name = query.get("location").map(String::as_str);
        let guard = state.read().await;
        match guard.list_cidr_blocks(collection_id, location_name) {
            Ok(blocks) => {
                let result = wire::ListCidrBlocksResponse {
                    cidr_blocks: Some(wire::CidrBlockSummaries {
                        items: blocks
                            .into_iter()
                            .map(|(loc, cidr)| wire::CidrBlockSummary {
                                cidr_block: Some(cidr),
                                location_name: Some(loc),
                            })
                            .collect(),
                    }),
                    next_token: None,
                };
                wire::serialize_list_cidr_blocks_response(&result)
            }
            Err(err) => r53_error_response(&err),
        }
    }

    // --- Traffic Policy handlers ---

    async fn handle_create_traffic_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53State>>,
        request: &MockRequest,
    ) -> MockResponse {
        let query = HashMap::new();
        let input = match deserialize_route53_request(
            wire::deserialize_create_traffic_policy_request(request, &[], &query),
        ) {
            Ok(input) => input,
            Err(response) => return response,
        };
        let mut guard = state.write().await;
        match guard.create_traffic_policy(&input.name, &input.document, input.comment.as_deref()) {
            Ok(entry) => {
                let result = wire::CreateTrafficPolicyResponse {
                    traffic_policy: Some(to_model_traffic_policy(&entry)),
                };
                let mut response = wire::serialize_create_traffic_policy_response(&result);
                add_location_header(
                    &mut response,
                    &format!("/2013-04-01/trafficpolicy/{}", entry.id),
                );
                response
            }
            Err(err) => r53_error_response(&err),
        }
    }

    async fn handle_create_traffic_policy_version(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53State>>,
        policy_id: &str,
        request: &MockRequest,
    ) -> MockResponse {
        let query = HashMap::new();
        let input = match deserialize_route53_request(
            wire::deserialize_create_traffic_policy_version_request(
                request,
                &[("Id", policy_id)],
                &query,
            ),
        ) {
            Ok(input) => input,
            Err(response) => return response,
        };
        let mut guard = state.write().await;
        match guard.create_traffic_policy_version(
            policy_id,
            &input.document,
            input.comment.as_deref(),
        ) {
            Ok(entry) => {
                let result = wire::CreateTrafficPolicyVersionResponse {
                    traffic_policy: Some(to_model_traffic_policy(&entry)),
                };
                let mut response = wire::serialize_create_traffic_policy_version_response(&result);
                add_location_header(
                    &mut response,
                    &format!("/2013-04-01/trafficpolicy/{}/{}", entry.id, entry.version),
                );
                response
            }
            Err(err) => r53_error_response(&err),
        }
    }

    async fn handle_get_traffic_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53State>>,
        policy_id: &str,
        version: &str,
    ) -> MockResponse {
        let version: i32 = match version.parse() {
            Ok(v) => v,
            Err(_) => {
                return r53_error_response(&Route53Error::InvalidVersion);
            }
        };
        let guard = state.read().await;
        match guard.get_traffic_policy(policy_id, version) {
            Ok(entry) => {
                let result = wire::GetTrafficPolicyResponse {
                    traffic_policy: Some(to_model_traffic_policy(&entry)),
                };
                wire::serialize_get_traffic_policy_response(&result)
            }
            Err(err) => r53_error_response(&err),
        }
    }

    async fn handle_delete_traffic_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53State>>,
        policy_id: &str,
        version: &str,
    ) -> MockResponse {
        let version: i32 = match version.parse() {
            Ok(v) => v,
            Err(_) => {
                return r53_error_response(&Route53Error::InvalidVersion);
            }
        };
        let mut guard = state.write().await;
        match guard.delete_traffic_policy(policy_id, version) {
            Ok(()) => wire::serialize_delete_traffic_policy_response(),
            Err(err) => r53_error_response(&err),
        }
    }

    async fn handle_update_traffic_policy_comment(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53State>>,
        policy_id: &str,
        version: &str,
        request: &MockRequest,
    ) -> MockResponse {
        let version_num: i32 = match version.parse() {
            Ok(v) => v,
            Err(_) => {
                return r53_error_response(&Route53Error::InvalidVersion);
            }
        };
        let query = HashMap::new();
        let input = match deserialize_route53_request(
            wire::deserialize_update_traffic_policy_comment_request(
                request,
                &[("Id", policy_id), ("Version", version)],
                &query,
            ),
        ) {
            Ok(input) => input,
            Err(response) => return response,
        };
        let mut guard = state.write().await;
        match guard.update_traffic_policy_comment(policy_id, version_num, &input.comment) {
            Ok(entry) => {
                let result = wire::UpdateTrafficPolicyCommentResponse {
                    traffic_policy: Some(to_model_traffic_policy(&entry)),
                };
                wire::serialize_update_traffic_policy_comment_response(&result)
            }
            Err(err) => r53_error_response(&err),
        }
    }

    async fn handle_list_traffic_policies(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53State>>,
    ) -> MockResponse {
        let guard = state.read().await;
        let policies = guard.list_traffic_policies();
        let result = wire::ListTrafficPoliciesResponse {
            is_truncated: Some(false),
            max_items: Some(100),
            traffic_policy_id_marker: Some(String::new()),
            traffic_policy_summaries: Some(wire::TrafficPolicySummaries {
                items: policies
                    .iter()
                    .map(|p| wire::TrafficPolicySummary {
                        id: Some(p.id.clone()),
                        latest_version: Some(p.version),
                        name: Some(p.name.clone()),
                        traffic_policy_count: Some(1),
                        r#type: Some(p.type_.clone()),
                    })
                    .collect(),
            }),
        };
        wire::serialize_list_traffic_policies_response(&result)
    }

    async fn handle_list_traffic_policy_versions(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53State>>,
        policy_id: &str,
    ) -> MockResponse {
        let guard = state.read().await;
        match guard.list_traffic_policy_versions(policy_id) {
            Ok(versions) => {
                let result = wire::ListTrafficPolicyVersionsResponse {
                    is_truncated: Some(false),
                    max_items: Some(100),
                    traffic_policies: Some(wire::TrafficPolicies {
                        items: versions.iter().map(to_model_traffic_policy).collect(),
                    }),
                    traffic_policy_version_marker: Some(String::new()),
                };
                wire::serialize_list_traffic_policy_versions_response(&result)
            }
            Err(err) => r53_error_response(&err),
        }
    }

    // --- Traffic Policy Instance handlers ---

    async fn handle_create_traffic_policy_instance(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53State>>,
        request: &MockRequest,
    ) -> MockResponse {
        let query = HashMap::new();
        let input = match deserialize_route53_request(
            wire::deserialize_create_traffic_policy_instance_request(request, &[], &query),
        ) {
            Ok(input) => input,
            Err(response) => return response,
        };
        let mut guard = state.write().await;
        match guard.create_traffic_policy_instance(
            &input.hosted_zone_id,
            &input.name,
            input.t_t_l,
            &input.traffic_policy_id,
            input.traffic_policy_version,
        ) {
            Ok(entry) => {
                let result = wire::CreateTrafficPolicyInstanceResponse {
                    traffic_policy_instance: Some(to_model_traffic_policy_instance(&entry)),
                };
                let mut response = wire::serialize_create_traffic_policy_instance_response(&result);
                add_location_header(
                    &mut response,
                    &format!("/2013-04-01/trafficpolicyinstance/{}", entry.id),
                );
                response
            }
            Err(err) => r53_error_response(&err),
        }
    }

    async fn handle_get_traffic_policy_instance(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53State>>,
        instance_id: &str,
    ) -> MockResponse {
        let guard = state.read().await;
        match guard.get_traffic_policy_instance(instance_id) {
            Ok(entry) => {
                let result = wire::GetTrafficPolicyInstanceResponse {
                    traffic_policy_instance: Some(to_model_traffic_policy_instance(&entry)),
                };
                wire::serialize_get_traffic_policy_instance_response(&result)
            }
            Err(err) => r53_error_response(&err),
        }
    }

    async fn handle_delete_traffic_policy_instance(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53State>>,
        instance_id: &str,
    ) -> MockResponse {
        let mut guard = state.write().await;
        match guard.delete_traffic_policy_instance(instance_id) {
            Ok(()) => wire::serialize_delete_traffic_policy_instance_response(),
            Err(err) => r53_error_response(&err),
        }
    }

    async fn handle_update_traffic_policy_instance(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53State>>,
        instance_id: &str,
        request: &MockRequest,
    ) -> MockResponse {
        let query = HashMap::new();
        let input = match deserialize_route53_request(
            wire::deserialize_update_traffic_policy_instance_request(
                request,
                &[("Id", instance_id)],
                &query,
            ),
        ) {
            Ok(input) => input,
            Err(response) => return response,
        };
        let mut guard = state.write().await;
        match guard.update_traffic_policy_instance(
            instance_id,
            input.t_t_l,
            &input.traffic_policy_id,
            input.traffic_policy_version,
        ) {
            Ok(entry) => {
                let result = wire::UpdateTrafficPolicyInstanceResponse {
                    traffic_policy_instance: Some(to_model_traffic_policy_instance(&entry)),
                };
                wire::serialize_update_traffic_policy_instance_response(&result)
            }
            Err(err) => r53_error_response(&err),
        }
    }

    async fn handle_list_traffic_policy_instances(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53State>>,
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let guard = state.read().await;
        // Check if filtering by hosted zone or policy
        let instances = if let Some(hz) = query.get("hostedzoneid") {
            guard.list_traffic_policy_instances_by_hosted_zone(hz)
        } else if let (Some(pid), Some(pv)) = (query.get("id"), query.get("version")) {
            let version: i32 = pv.parse().unwrap_or(1);
            guard.list_traffic_policy_instances_by_policy(pid, version)
        } else {
            guard.list_traffic_policy_instances()
        };
        let result = wire::ListTrafficPolicyInstancesResponse {
            hosted_zone_id_marker: None,
            is_truncated: Some(false),
            max_items: Some(100),
            traffic_policy_instance_name_marker: None,
            traffic_policy_instance_type_marker: None,
            traffic_policy_instances: Some(wire::TrafficPolicyInstances {
                items: instances
                    .iter()
                    .map(to_model_traffic_policy_instance)
                    .collect(),
            }),
        };
        wire::serialize_list_traffic_policy_instances_response(&result)
    }

    async fn handle_list_traffic_policy_instances_by_hosted_zone(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53State>>,
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let hosted_zone_id = query.get("id").map(String::as_str).unwrap_or("");
        let guard = state.read().await;
        let instances = guard.list_traffic_policy_instances_by_hosted_zone(hosted_zone_id);
        let result = wire::ListTrafficPolicyInstancesByHostedZoneResponse {
            is_truncated: Some(false),
            max_items: Some(100),
            traffic_policy_instance_name_marker: None,
            traffic_policy_instance_type_marker: None,
            traffic_policy_instances: Some(wire::TrafficPolicyInstances {
                items: instances
                    .iter()
                    .map(to_model_traffic_policy_instance)
                    .collect(),
            }),
        };
        wire::serialize_list_traffic_policy_instances_by_hosted_zone_response(&result)
    }

    async fn handle_list_traffic_policy_instances_by_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53State>>,
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let policy_id = query.get("id").map(String::as_str).unwrap_or("");
        let version: i32 = query
            .get("version")
            .and_then(|v| v.parse().ok())
            .unwrap_or(1);
        let guard = state.read().await;
        let instances = guard.list_traffic_policy_instances_by_policy(policy_id, version);
        let result = wire::ListTrafficPolicyInstancesByPolicyResponse {
            hosted_zone_id_marker: None,
            is_truncated: Some(false),
            max_items: Some(100),
            traffic_policy_instance_name_marker: None,
            traffic_policy_instance_type_marker: None,
            traffic_policy_instances: Some(wire::TrafficPolicyInstances {
                items: instances
                    .iter()
                    .map(to_model_traffic_policy_instance)
                    .collect(),
            }),
        };
        wire::serialize_list_traffic_policy_instances_by_policy_response(&result)
    }

    async fn handle_get_traffic_policy_instance_count(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53State>>,
    ) -> MockResponse {
        let guard = state.read().await;
        let result = wire::GetTrafficPolicyInstanceCountResponse {
            traffic_policy_instance_count: Some(guard.get_traffic_policy_instance_count()),
        };
        wire::serialize_get_traffic_policy_instance_count_response(&result)
    }

    // --- Simple/default data handlers ---

    async fn handle_get_change(&self) -> MockResponse {
        let result = wire::GetChangeResponse {
            change_info: Some(change_info()),
        };
        wire::serialize_get_change_response(&result)
    }

    async fn handle_get_account_limit(&self, limit_type: &str) -> MockResponse {
        let (value, count) = match limit_type {
            "MAX_HOSTED_ZONES_BY_OWNER" => (500, 0),
            "MAX_HEALTH_CHECKS_BY_OWNER" => (200, 0),
            "MAX_REUSABLE_DELEGATION_SETS_BY_OWNER" => (100, 0),
            "MAX_TRAFFIC_POLICIES_BY_OWNER" => (50, 0),
            "MAX_TRAFFIC_POLICY_INSTANCES_BY_OWNER" => (5, 0),
            _ => (100, 0),
        };
        let result = wire::GetAccountLimitResponse {
            count: Some(count),
            limit: Some(wire::AccountLimit {
                r#type: Some(limit_type.to_string()),
                value: Some(value),
            }),
        };
        wire::serialize_get_account_limit_response(&result)
    }

    async fn handle_get_checker_ip_ranges(&self) -> MockResponse {
        let result = wire::GetCheckerIpRangesResponse {
            checker_ip_ranges: Some(wire::CheckerIpRanges {
                items: vec![
                    "54.228.16.0/26".to_string(),
                    "176.34.159.192/26".to_string(),
                    "54.232.40.64/26".to_string(),
                    "177.71.207.128/26".to_string(),
                ],
            }),
        };
        wire::serialize_get_checker_ip_ranges_response(&result)
    }

    async fn handle_get_geo_location(&self, query: &HashMap<String, String>) -> MockResponse {
        let continent_code = query.get("continentcode").cloned();
        let country_code = query.get("countrycode").cloned();
        let subdivision_code = query.get("subdivisioncode").cloned();

        let result = wire::GetGeoLocationResponse {
            geo_location_details: Some(wire::GeoLocationDetails {
                continent_code: continent_code.clone(),
                continent_name: continent_code.as_deref().map(continent_name),
                country_code: country_code.clone(),
                country_name: country_code.as_deref().map(|_| "United States".to_string()),
                subdivision_code,
                subdivision_name: None,
            }),
        };
        wire::serialize_get_geo_location_response(&result)
    }

    async fn handle_list_geo_locations(&self) -> MockResponse {
        let result = wire::ListGeoLocationsResponse {
            geo_location_details_list: Some(wire::GeoLocationDetailsList {
                items: vec![
                    wire::GeoLocationDetails {
                        continent_code: Some("AF".to_string()),
                        continent_name: Some("Africa".to_string()),
                        ..Default::default()
                    },
                    wire::GeoLocationDetails {
                        continent_code: Some("AN".to_string()),
                        continent_name: Some("Antarctica".to_string()),
                        ..Default::default()
                    },
                    wire::GeoLocationDetails {
                        continent_code: Some("AS".to_string()),
                        continent_name: Some("Asia".to_string()),
                        ..Default::default()
                    },
                    wire::GeoLocationDetails {
                        continent_code: Some("EU".to_string()),
                        continent_name: Some("Europe".to_string()),
                        ..Default::default()
                    },
                    wire::GeoLocationDetails {
                        continent_code: Some("NA".to_string()),
                        continent_name: Some("North America".to_string()),
                        ..Default::default()
                    },
                    wire::GeoLocationDetails {
                        continent_code: Some("OC".to_string()),
                        continent_name: Some("Oceania".to_string()),
                        ..Default::default()
                    },
                    wire::GeoLocationDetails {
                        continent_code: Some("SA".to_string()),
                        continent_name: Some("South America".to_string()),
                        ..Default::default()
                    },
                ],
            }),
            is_truncated: Some(false),
            max_items: Some(100),
            next_continent_code: None,
            next_country_code: None,
            next_subdivision_code: None,
        };
        wire::serialize_list_geo_locations_response(&result)
    }

    async fn handle_get_health_check_count(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53State>>,
    ) -> MockResponse {
        let guard = state.read().await;
        let result = wire::GetHealthCheckCountResponse {
            health_check_count: Some(guard.list_health_checks().len() as i64),
        };
        wire::serialize_get_health_check_count_response(&result)
    }

    async fn handle_get_health_check_last_failure_reason(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53State>>,
        health_check_id: &str,
    ) -> MockResponse {
        let guard = state.read().await;
        match guard.get_health_check(health_check_id) {
            Ok(_) => {
                let result = wire::GetHealthCheckLastFailureReasonResponse {
                    health_check_observations: Some(wire::HealthCheckObservations {
                        items: Vec::new(),
                    }),
                };
                wire::serialize_get_health_check_last_failure_reason_response(&result)
            }
            Err(err) => r53_error_response(&err),
        }
    }

    async fn handle_get_hosted_zone_limit(
        &self,
        state: &Arc<tokio::sync::RwLock<Route53State>>,
        zone_id: &str,
        limit_type: &str,
    ) -> MockResponse {
        let guard = state.read().await;
        match guard.get_hosted_zone(zone_id) {
            Ok(zone) => {
                let (value, count) = match limit_type {
                    "MAX_RRSETS_BY_ZONE" => (10000, zone.resource_record_count as i64),
                    "MAX_VPCS_ASSOCIATED_BY_ZONE" => (100, zone.vpcs.len() as i64),
                    _ => (10000, 0),
                };
                let result = wire::GetHostedZoneLimitResponse {
                    count: Some(count),
                    limit: Some(wire::HostedZoneLimit {
                        r#type: Some(limit_type.to_string()),
                        value: Some(value),
                    }),
                };
                wire::serialize_get_hosted_zone_limit_response(&result)
            }
            Err(err) => r53_error_response(&err),
        }
    }

    async fn handle_test_dns_answer(&self, query: &HashMap<String, String>) -> MockResponse {
        let record_name = query.get("recordname").cloned().unwrap_or_default();
        let record_type = query.get("recordtype").cloned().unwrap_or_default();
        let result = wire::TestDNSAnswerResponse {
            nameserver: Some("ns-1.awsdns-00.org.".to_string()),
            protocol: Some("UDP".to_string()),
            record_data: Some(wire::RecordData { items: Vec::new() }),
            record_name: Some(ensure_trailing_dot(&record_name)),
            record_type: Some(record_type),
            response_code: Some("NOERROR".to_string()),
        };
        wire::serialize_test_d_n_s_answer_response(&result)
    }

    async fn handle_update_hosted_zone_features(&self) -> MockResponse {
        wire::serialize_update_hosted_zone_features_response()
    }
}

fn parse_query_from_uri(uri: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    if let Some(query_string) = uri.split('?').nth(1) {
        for pair in query_string.split('&') {
            if let Some((key, value)) = pair.split_once('=') {
                map.insert(percent_decode(key).to_lowercase(), percent_decode(value));
            } else if !pair.is_empty() {
                map.insert(percent_decode(pair).to_lowercase(), String::new());
            }
        }
    }
    map
}

fn percent_decode(value: &str) -> String {
    let mut result = String::with_capacity(value.len());
    let mut bytes = value.bytes();
    while let Some(byte) = bytes.next() {
        match byte {
            b'%' => {
                let hi = bytes.next();
                let lo = bytes.next();
                if let (Some(hi), Some(lo)) = (hi, lo) {
                    if let Ok(decoded) =
                        u8::from_str_radix(std::str::from_utf8(&[hi, lo]).unwrap_or(""), 16)
                    {
                        result.push(decoded as char);
                        continue;
                    }
                }
                result.push('%');
            }
            b'+' => result.push(' '),
            _ => result.push(byte as char),
        }
    }
    result
}

fn extract_path(uri: &str) -> String {
    let after_scheme = uri
        .strip_prefix("https://")
        .or_else(|| uri.strip_prefix("http://"))
        .unwrap_or(uri);
    let slash = after_scheme.find('/').unwrap_or(after_scheme.len());
    after_scheme[slash..]
        .split('?')
        .next()
        .unwrap_or("/")
        .to_string()
}

fn ensure_trailing_dot(name: &str) -> String {
    if name.ends_with('.') {
        name.to_string()
    } else {
        format!("{name}.")
    }
}

fn iso_timestamp() -> String {
    chrono::Utc::now()
        .format("%Y-%m-%dT%H:%M:%S.000Z")
        .to_string()
}

fn change_info() -> wire::ChangeInfo {
    wire::ChangeInfo {
        comment: None,
        id: Some(format!("/change/{}", uuid::Uuid::new_v4())),
        status: Some("INSYNC".to_string()),
        submitted_at: Some(iso_timestamp()),
    }
}

fn to_model_delegation_set(set: &Route53DelegationSet) -> wire::DelegationSet {
    wire::DelegationSet {
        caller_reference: set.caller_reference.clone(),
        id: Some(format!("/delegationset/{}", set.id)),
        name_servers: Some(wire::DelegationSetNameServers {
            items: set.name_servers.clone(),
        }),
    }
}

fn to_model_hosted_zone(zone: &crate::types::HostedZone) -> wire::HostedZone {
    wire::HostedZone {
        caller_reference: Some(zone.caller_reference.clone()),
        config: Some(wire::HostedZoneConfig {
            comment: zone.comment.clone(),
            private_zone: Some(!zone.vpcs.is_empty()),
        }),
        features: None,
        id: Some(zone.id.clone()),
        linked_service: None,
        name: Some(zone.name.clone()),
        resource_record_set_count: Some(zone.resource_record_count as i64),
    }
}

fn to_model_vpc(vpc: &crate::types::Vpc) -> wire::VPC {
    wire::VPC {
        v_p_c_id: Some(vpc.vpc_id.clone()),
        v_p_c_region: Some(vpc.vpc_region.clone()),
    }
}

fn to_model_query_logging_config(config: &Route53QueryLoggingConfig) -> wire::QueryLoggingConfig {
    wire::QueryLoggingConfig {
        cloud_watch_logs_log_group_arn: Some(config.cloud_watch_logs_log_group_arn.clone()),
        hosted_zone_id: Some(config.hosted_zone_id.clone()),
        id: Some(config.id.clone()),
    }
}

fn to_model_health_check(health_check: &Route53HealthCheck) -> wire::HealthCheck {
    wire::HealthCheck {
        caller_reference: Some(health_check.caller_reference.clone()),
        cloud_watch_alarm_configuration: None,
        health_check_config: Some(to_model_health_check_config(&health_check.config)),
        health_check_version: Some(health_check.version),
        id: Some(health_check.id.clone()),
        linked_service: None,
    }
}

fn to_model_health_check_config(config: &Route53HealthCheckConfig) -> wire::HealthCheckConfig {
    wire::HealthCheckConfig {
        alarm_identifier: None,
        child_health_checks: (!config.child_health_checks.is_empty()).then(|| {
            wire::ChildHealthCheckList {
                items: config.child_health_checks.clone(),
            }
        }),
        disabled: config.disabled,
        enable_s_n_i: config.enable_sni,
        failure_threshold: config.failure_threshold,
        fully_qualified_domain_name: config.fully_qualified_domain_name.clone(),
        health_threshold: config.health_threshold,
        i_p_address: config.ip_address.clone(),
        insufficient_data_health_status: config.insufficient_data_health_status.clone(),
        inverted: config.inverted,
        measure_latency: config.measure_latency,
        port: config.port,
        regions: (!config.regions.is_empty()).then(|| wire::HealthCheckRegionList {
            items: config.regions.clone(),
        }),
        request_interval: config.request_interval,
        resource_path: config.resource_path.clone(),
        routing_control_arn: None,
        search_string: config.search_string.clone(),
        r#type: config.type_.clone(),
    }
}

fn to_model_record_set(record_set: &ResourceRecordSet) -> wire::ResourceRecordSet {
    wire::ResourceRecordSet {
        alias_target: None,
        cidr_routing_config: None,
        failover: None,
        geo_location: None,
        geo_proximity_location: None,
        health_check_id: None,
        multi_value_answer: None,
        name: record_set.name.clone(),
        region: None,
        resource_records: (!record_set.resource_records.is_empty()).then(|| {
            wire::ResourceRecords {
                items: record_set
                    .resource_records
                    .iter()
                    .map(|value| wire::ResourceRecord {
                        value: value.clone(),
                    })
                    .collect(),
            }
        }),
        set_identifier: None,
        t_t_l: record_set.ttl.map(|ttl| ttl as i64),
        traffic_policy_instance_id: None,
        r#type: record_set.record_type.clone(),
        weight: None,
    }
}

fn to_model_resource_tag_set(
    resource_type: &str,
    resource_id: &str,
    tags: &HashMap<String, String>,
) -> wire::ResourceTagSet {
    let mut sorted_tags: Vec<_> = tags.iter().collect();
    sorted_tags.sort_by(|a, b| a.0.cmp(b.0));
    wire::ResourceTagSet {
        resource_id: Some(canonical_resource_id(resource_type, resource_id)),
        resource_type: Some(resource_type.to_string()),
        tags: Some(wire::TagList {
            items: sorted_tags
                .into_iter()
                .map(|(key, value)| wire::Tag {
                    key: Some(key.clone()),
                    value: Some(value.clone()),
                })
                .collect(),
        }),
    }
}

fn to_model_tag_resource(resource: &TagResource) -> wire::ResourceTagSet {
    to_model_resource_tag_set(
        &resource.resource_type,
        &resource.resource_id,
        &resource.tags,
    )
}

fn to_model_key_signing_key(entry: &crate::types::KeySigningKeyEntry) -> wire::KeySigningKey {
    wire::KeySigningKey {
        created_date: Some(entry.created_date.clone()),
        d_n_s_k_e_y_record: None,
        d_s_record: None,
        digest_algorithm_mnemonic: None,
        digest_algorithm_type: None,
        digest_value: None,
        flag: Some(256),
        key_tag: Some(12345),
        kms_arn: Some(entry.kms_arn.clone()),
        last_modified_date: Some(entry.last_modified_date.clone()),
        name: Some(entry.name.clone()),
        public_key: None,
        signing_algorithm_mnemonic: Some("ECDSAP256SHA256".to_string()),
        signing_algorithm_type: Some(13),
        status: Some(entry.status.clone()),
        status_message: None,
    }
}

fn to_model_traffic_policy(entry: &TrafficPolicyEntry) -> wire::TrafficPolicy {
    wire::TrafficPolicy {
        comment: entry.comment.clone(),
        document: Some(entry.document.clone()),
        id: Some(entry.id.clone()),
        name: Some(entry.name.clone()),
        r#type: Some(entry.type_.clone()),
        version: Some(entry.version),
    }
}

fn to_model_traffic_policy_instance(
    entry: &TrafficPolicyInstanceEntry,
) -> wire::TrafficPolicyInstance {
    wire::TrafficPolicyInstance {
        hosted_zone_id: Some(entry.hosted_zone_id.clone()),
        id: Some(entry.id.clone()),
        message: Some(entry.message.clone()),
        name: Some(entry.name.clone()),
        state: Some(entry.state.clone()),
        t_t_l: Some(entry.ttl),
        traffic_policy_id: Some(entry.traffic_policy_id.clone()),
        traffic_policy_type: Some(entry.traffic_policy_type.clone()),
        traffic_policy_version: Some(entry.traffic_policy_version),
    }
}

fn continent_name(code: &str) -> String {
    match code {
        "AF" => "Africa",
        "AN" => "Antarctica",
        "AS" => "Asia",
        "EU" => "Europe",
        "NA" => "North America",
        "OC" => "Oceania",
        "SA" => "South America",
        _ => "Unknown",
    }
    .to_string()
}

fn deserialize_route53_request<T>(result: Result<T, String>) -> Result<T, MockResponse> {
    result.map_err(|message| r53_error_response(&Route53Error::InvalidInput(message)))
}

fn route53_record_change_from_wire(change: wire::Change) -> RecordChange {
    RecordChange {
        action: change.action,
        record_set: ResourceRecordSet {
            name: change.resource_record_set.name,
            record_type: change.resource_record_set.r#type,
            ttl: change
                .resource_record_set
                .t_t_l
                .and_then(|ttl| u64::try_from(ttl).ok()),
            resource_records: change
                .resource_record_set
                .resource_records
                .map(|records| {
                    records
                        .items
                        .into_iter()
                        .map(|record| record.value)
                        .collect()
                })
                .unwrap_or_default(),
        },
    }
}

fn route53_health_check_config_from_wire(
    config: wire::HealthCheckConfig,
) -> Route53HealthCheckConfig {
    Route53HealthCheckConfig {
        type_: config.r#type,
        ip_address: config.i_p_address,
        port: config.port,
        resource_path: config.resource_path,
        fully_qualified_domain_name: config.fully_qualified_domain_name,
        failure_threshold: config.failure_threshold,
        request_interval: config.request_interval,
        inverted: config.inverted,
        disabled: config.disabled,
        measure_latency: config.measure_latency,
        search_string: config.search_string,
        enable_sni: config.enable_s_n_i,
        health_threshold: config.health_threshold,
        insufficient_data_health_status: config.insufficient_data_health_status,
        regions: config
            .regions
            .map(|regions| regions.items)
            .unwrap_or_default(),
        child_health_checks: config
            .child_health_checks
            .map(|checks| checks.items)
            .unwrap_or_default(),
    }
}

fn route53_health_check_updates_from_wire(
    input: wire::UpdateHealthCheckRequest,
) -> Route53HealthCheckConfig {
    Route53HealthCheckConfig {
        type_: String::new(),
        ip_address: input.i_p_address,
        port: input.port,
        resource_path: input.resource_path,
        fully_qualified_domain_name: input.fully_qualified_domain_name,
        failure_threshold: input.failure_threshold,
        request_interval: None,
        inverted: input.inverted,
        disabled: input.disabled,
        measure_latency: None,
        search_string: input.search_string,
        enable_sni: input.enable_s_n_i,
        health_threshold: input.health_threshold,
        insufficient_data_health_status: input.insufficient_data_health_status,
        regions: input
            .regions
            .map(|regions| regions.items)
            .unwrap_or_default(),
        child_health_checks: input
            .child_health_checks
            .map(|checks| checks.items)
            .unwrap_or_default(),
    }
}

fn route53_vpc_from_wire(vpc: wire::VPC) -> Option<crate::types::Vpc> {
    Some(crate::types::Vpc {
        vpc_id: vpc.v_p_c_id?,
        vpc_region: vpc.v_p_c_region.unwrap_or_default(),
    })
}

fn add_location_header(response: &mut MockResponse, location: &str) {
    response
        .headers
        .insert("Location", location.parse().unwrap());
}

fn xml_response<T: Serialize>(value: &T, status: u16) -> MockResponse {
    MockResponse::xml(status, quick_xml::se::to_string(value).unwrap_or_default())
}

fn r53_error_response(err: &Route53Error) -> MockResponse {
    let (status, error_type) = match err {
        Route53Error::NoSuchHostedZone { .. } => (404, "NoSuchHostedZone"),
        Route53Error::HostedZoneAlreadyExists { .. } => (409, "HostedZoneAlreadyExists"),
        Route53Error::InvalidChangeBatchRecordExists { .. } => (400, "InvalidChangeBatch"),
        Route53Error::InvalidChangeBatchAction { .. } => (400, "InvalidInput"),
        Route53Error::NoChangesInRequest => (400, "InvalidChangeBatch"),
        Route53Error::NoSuchHealthCheck { .. } => (404, "NoSuchHealthCheck"),
        Route53Error::HealthCheckAlreadyExists { .. } => (409, "HealthCheckAlreadyExists"),
        Route53Error::QueryLoggingConfigAlreadyExists => (400, "QueryLoggingConfigAlreadyExists"),
        Route53Error::NoSuchQueryLoggingConfig => (404, "NoSuchQueryLoggingConfig"),
        Route53Error::DelegationSetAlreadyCreated => (409, "DelegationSetAlreadyCreated"),
        Route53Error::NoSuchDelegationSet { .. } => (404, "NoSuchDelegationSet"),
        Route53Error::KeySigningKeyAlreadyExists { .. } => (409, "KeySigningKeyAlreadyExists"),
        Route53Error::NoSuchKeySigningKey { .. } => (404, "NoSuchKeySigningKey"),
        Route53Error::CidrCollectionAlreadyExistsException { .. } => {
            (409, "CidrCollectionAlreadyExistsException")
        }
        Route53Error::NoSuchCidrCollectionException { .. } => {
            (404, "NoSuchCidrCollectionException")
        }
        Route53Error::NoSuchTrafficPolicy { .. } => (404, "NoSuchTrafficPolicy"),
        Route53Error::NoSuchTrafficPolicyVersion { .. } => (404, "NoSuchTrafficPolicy"),
        Route53Error::NoSuchTrafficPolicyInstance { .. } => (404, "NoSuchTrafficPolicyInstance"),
        Route53Error::VpcAssociationAuthorizationNotFound { .. } => {
            (404, "VPCAssociationAuthorizationNotFound")
        }
        Route53Error::UnsupportedResourceType { .. } => (400, "InvalidInput"),
        Route53Error::UnsupportedOperation { .. } => (400, "InvalidInput"),
        Route53Error::MissingNameOrCallerReference => (400, "InvalidInput"),
        Route53Error::MissingVpcId => (400, "InvalidInput"),
        Route53Error::MissingVpcIdParameter => (400, "InvalidInput"),
        Route53Error::MissingHealthCheckFields => (400, "InvalidInput"),
        Route53Error::MissingQueryLoggingConfigFields => (400, "InvalidInput"),
        Route53Error::MissingCallerReference => (400, "InvalidInput"),
        Route53Error::InvalidVersion => (400, "InvalidInput"),
        Route53Error::InvalidInput(_) => (400, "InvalidInput"),
    };
    xml_response(
        &ErrorResponseXml {
            xmlns: ROUTE53_XMLNS,
            error: ErrorXml {
                kind: "Sender".to_string(),
                code: error_type.to_string(),
                message: err.to_string(),
            },
            request_id: uuid::Uuid::new_v4().to_string(),
        },
        status,
    )
}

const ROUTE53_XMLNS: &str = "https://route53.amazonaws.com/doc/2013-04-01/";

#[derive(Serialize)]
#[serde(rename = "ErrorResponse")]
struct ErrorResponseXml {
    #[serde(rename = "@xmlns")]
    xmlns: &'static str,
    #[serde(rename = "Error")]
    error: ErrorXml,
    #[serde(rename = "RequestId")]
    request_id: String,
}

#[derive(Serialize)]
struct ErrorXml {
    #[serde(rename = "Type")]
    kind: String,
    #[serde(rename = "Code")]
    code: String,
    #[serde(rename = "Message")]
    message: String,
}
