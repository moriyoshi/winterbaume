use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, default_account_id,
};

use crate::state::{IotError, IotState};
use crate::views::IotStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct IotService {
    pub(crate) state: Arc<BackendState<IotState>>,
    pub(crate) notifier: StateChangeNotifier<IotStateView>,
}

impl IotService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for IotService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for IotService {
    fn service_name(&self) -> &str {
        "iot"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://iot\.(.+)\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl IotService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();
        let state = self.state.get(account_id, &region);

        let (path, query) = extract_path_and_query(&request.uri);
        let method = request.method.as_str();

        let segments: Vec<&str> = path
            .trim_start_matches('/')
            .split('/')
            .filter(|s| !s.is_empty())
            .collect();

        let query_params = parse_query(&query);

        // Extract headers for operations that use them
        let principal_header = request
            .headers
            .get("x-amzn-principal")
            .or_else(|| request.headers.get("x-amzn-iot-principal"))
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");

        let policy_name_header = request
            .headers
            .get("x-amzn-iot-policy")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");

        match (method, segments.as_slice()) {
            // ==================== Thing ====================
            // POST /things/{thingName} - CreateThing
            ("POST", ["things", thing_name]) => {
                let thing_name = percent_decode(thing_name);
                self.handle_create_thing(
                    &state,
                    &thing_name,
                    &request,
                    &query_params,
                    &region,
                    account_id,
                )
                .await
            }
            // GET /things - ListThings
            ("GET", ["things"]) => self.handle_list_things(&state).await,
            // GET /things/{thingName} - DescribeThing
            ("GET", ["things", thing_name]) => {
                let thing_name = percent_decode(thing_name);
                self.handle_describe_thing(&state, &thing_name).await
            }
            // PATCH /things/{thingName} - UpdateThing
            ("PATCH", ["things", thing_name]) => {
                let thing_name = percent_decode(thing_name);
                self.handle_update_thing(&state, &thing_name, &request, &query_params)
                    .await
            }
            // DELETE /things/{thingName} - DeleteThing
            ("DELETE", ["things", thing_name]) => {
                let thing_name = percent_decode(thing_name);
                let expected_version = query_params
                    .get("expectedVersion")
                    .and_then(|v| v.parse::<i64>().ok());
                self.handle_delete_thing(&state, &thing_name, expected_version)
                    .await
            }

            // ==================== ThingType ====================
            // POST /thing-types/{thingTypeName} - CreateThingType
            ("POST", ["thing-types", ttn]) => {
                let ttn = percent_decode(ttn);
                self.handle_create_thing_type(
                    &state,
                    &ttn,
                    &request,
                    &query_params,
                    &region,
                    account_id,
                )
                .await
            }
            // GET /thing-types - ListThingTypes
            ("GET", ["thing-types"]) => self.handle_list_thing_types(&state).await,
            // GET /thing-types/{thingTypeName} - DescribeThingType
            ("GET", ["thing-types", ttn]) => {
                let ttn = percent_decode(ttn);
                self.handle_describe_thing_type(&state, &ttn).await
            }
            // DELETE /thing-types/{thingTypeName} - DeleteThingType
            ("DELETE", ["thing-types", ttn]) => {
                let ttn = percent_decode(ttn);
                self.handle_delete_thing_type(&state, &ttn).await
            }
            // POST /thing-types/{thingTypeName}/deprecate - DeprecateThingType
            ("POST", ["thing-types", ttn, "deprecate"]) => {
                let ttn = percent_decode(ttn);
                let labels: &[(&str, &str)] = &[("thingTypeName", ttn.as_str())];
                let input = match wire::deserialize_deprecate_thing_type_request(
                    &request,
                    labels,
                    &query_params,
                ) {
                    Ok(v) => v,
                    Err(e) => return rest_json_error(400, "ValidationException", &e),
                };
                let undo = input.undo_deprecate.unwrap_or(false);
                let mut st = state.write().await;
                match st.deprecate_thing_type(&ttn, undo) {
                    Ok(()) => wire::serialize_deprecate_thing_type_response(
                        &wire::DeprecateThingTypeResponse {},
                    ),
                    Err(e) => iot_error_response(&e),
                }
            }

            // ==================== ThingGroup ====================
            // POST /thing-groups/{name} - CreateThingGroup
            ("POST", ["thing-groups", name]) => {
                let name = percent_decode(name);
                self.handle_create_thing_group(
                    &state,
                    &name,
                    &request,
                    &query_params,
                    &region,
                    account_id,
                )
                .await
            }
            // GET /thing-groups - ListThingGroups
            ("GET", ["thing-groups"]) => self.handle_list_thing_groups(&state).await,
            // GET /thing-groups/{name} - DescribeThingGroup
            ("GET", ["thing-groups", name]) => {
                let name = percent_decode(name);
                self.handle_describe_thing_group(&state, &name).await
            }
            // PATCH /thing-groups/{name} - UpdateThingGroup
            ("PATCH", ["thing-groups", name]) => {
                let name = percent_decode(name);
                self.handle_update_thing_group(&state, &name, &request, &query_params)
                    .await
            }
            // DELETE /thing-groups/{name} - DeleteThingGroup
            ("DELETE", ["thing-groups", name]) => {
                let name = percent_decode(name);
                let ev = query_params
                    .get("expectedVersion")
                    .and_then(|v| v.parse::<i64>().ok());
                let mut st = state.write().await;
                match st.delete_thing_group(&name, ev) {
                    Ok(()) => wire::serialize_delete_thing_group_response(
                        &wire::DeleteThingGroupResponse {},
                    ),
                    Err(e) => iot_error_response(&e),
                }
            }
            // PUT /thing-groups/addThingToThingGroup - AddThingToThingGroup
            ("PUT", ["thing-groups", "addThingToThingGroup"]) => {
                let input = match wire::deserialize_add_thing_to_thing_group_request(
                    &request,
                    &[],
                    &query_params,
                ) {
                    Ok(v) => v,
                    Err(e) => return rest_json_error(400, "ValidationException", &e),
                };
                let gn = input.thing_group_name.as_deref();
                let tn = input.thing_name.as_deref();
                let mut st = state.write().await;
                match st.add_thing_to_thing_group(gn, tn) {
                    Ok(()) => wire::serialize_add_thing_to_thing_group_response(
                        &wire::AddThingToThingGroupResponse {},
                    ),
                    Err(e) => iot_error_response(&e),
                }
            }
            // PUT /thing-groups/removeThingFromThingGroup - RemoveThingFromThingGroup
            ("PUT", ["thing-groups", "removeThingFromThingGroup"]) => {
                let input = match wire::deserialize_remove_thing_from_thing_group_request(
                    &request,
                    &[],
                    &query_params,
                ) {
                    Ok(v) => v,
                    Err(e) => return rest_json_error(400, "ValidationException", &e),
                };
                let gn = input.thing_group_name.as_deref();
                let tn = input.thing_name.as_deref();
                let mut st = state.write().await;
                match st.remove_thing_from_thing_group(gn, tn) {
                    Ok(()) => wire::serialize_remove_thing_from_thing_group_response(
                        &wire::RemoveThingFromThingGroupResponse {},
                    ),
                    Err(e) => iot_error_response(&e),
                }
            }
            // PUT /thing-groups/updateThingGroupsForThing - UpdateThingGroupsForThing
            ("PUT", ["thing-groups", "updateThingGroupsForThing"]) => {
                let input = match wire::deserialize_update_thing_groups_for_thing_request(
                    &request,
                    &[],
                    &query_params,
                ) {
                    Ok(v) => v,
                    Err(e) => return rest_json_error(400, "ValidationException", &e),
                };
                let tn = input.thing_name.as_deref();
                let adds = input.thing_groups_to_add.clone();
                let removes = input.thing_groups_to_remove.clone();
                let mut st = state.write().await;
                match st.update_thing_groups_for_thing(tn, adds, removes) {
                    Ok(()) => wire::serialize_update_thing_groups_for_thing_response(
                        &wire::UpdateThingGroupsForThingResponse {},
                    ),
                    Err(e) => iot_error_response(&e),
                }
            }
            // GET /thing-groups/{name}/things - ListThingsInThingGroup
            ("GET", ["thing-groups", name, "things"]) => {
                let name = percent_decode(name);
                let st = state.read().await;
                match st.list_things_in_thing_group(&name) {
                    Ok(things) => wire::serialize_list_things_in_thing_group_response(
                        &wire::ListThingsInThingGroupResponse {
                            things: Some(things),
                            next_token: None,
                        },
                    ),
                    Err(e) => iot_error_response(&e),
                }
            }
            // GET /things/{thingName}/thing-groups - ListThingGroupsForThing
            ("GET", ["things", thing_name, "thing-groups"]) => {
                let thing_name = percent_decode(thing_name);
                let st = state.read().await;
                match st.list_thing_groups_for_thing(&thing_name) {
                    Ok(groups) => {
                        let entries: Vec<wire::GroupNameAndArn> = groups
                            .iter()
                            .map(|g| wire::GroupNameAndArn {
                                group_name: Some(g.thing_group_name.clone()),
                                group_arn: Some(g.thing_group_arn.clone()),
                            })
                            .collect();
                        wire::serialize_list_thing_groups_for_thing_response(
                            &wire::ListThingGroupsForThingResponse {
                                thing_groups: Some(entries),
                                next_token: None,
                            },
                        )
                    }
                    Err(e) => iot_error_response(&e),
                }
            }

            // ==================== BillingGroup ====================
            // POST /billing-groups/{name} - CreateBillingGroup
            ("POST", ["billing-groups", name]) => {
                let name = percent_decode(name);
                self.handle_create_billing_group(
                    &state,
                    &name,
                    &request,
                    &query_params,
                    &region,
                    account_id,
                )
                .await
            }
            // GET /billing-groups - ListBillingGroups
            ("GET", ["billing-groups"]) => self.handle_list_billing_groups(&state).await,
            // GET /billing-groups/{name} - DescribeBillingGroup
            ("GET", ["billing-groups", name]) => {
                let name = percent_decode(name);
                self.handle_describe_billing_group(&state, &name).await
            }
            // PATCH /billing-groups/{name} - UpdateBillingGroup
            ("PATCH", ["billing-groups", name]) => {
                let name = percent_decode(name);
                self.handle_update_billing_group(&state, &name, &request, &query_params)
                    .await
            }
            // DELETE /billing-groups/{name} - DeleteBillingGroup
            ("DELETE", ["billing-groups", name]) => {
                let name = percent_decode(name);
                let ev = query_params
                    .get("expectedVersion")
                    .and_then(|v| v.parse::<i64>().ok());
                let mut st = state.write().await;
                match st.delete_billing_group(&name, ev) {
                    Ok(()) => wire::serialize_delete_billing_group_response(
                        &wire::DeleteBillingGroupResponse {},
                    ),
                    Err(e) => iot_error_response(&e),
                }
            }
            // PUT /billing-groups/addThingToBillingGroup - AddThingToBillingGroup
            ("PUT", ["billing-groups", "addThingToBillingGroup"]) => {
                let input = match wire::deserialize_add_thing_to_billing_group_request(
                    &request,
                    &[],
                    &query_params,
                ) {
                    Ok(v) => v,
                    Err(e) => return rest_json_error(400, "ValidationException", &e),
                };
                let bgn = input.billing_group_name.as_deref();
                let tn = input.thing_name.as_deref();
                let mut st = state.write().await;
                match st.add_thing_to_billing_group(bgn, tn) {
                    Ok(()) => wire::serialize_add_thing_to_billing_group_response(
                        &wire::AddThingToBillingGroupResponse {},
                    ),
                    Err(e) => iot_error_response(&e),
                }
            }
            // PUT /billing-groups/removeThingFromBillingGroup - RemoveThingFromBillingGroup
            ("PUT", ["billing-groups", "removeThingFromBillingGroup"]) => {
                let input = match wire::deserialize_remove_thing_from_billing_group_request(
                    &request,
                    &[],
                    &query_params,
                ) {
                    Ok(v) => v,
                    Err(e) => return rest_json_error(400, "ValidationException", &e),
                };
                let bgn = input.billing_group_name.as_deref();
                let tn = input.thing_name.as_deref();
                let mut st = state.write().await;
                match st.remove_thing_from_billing_group(bgn, tn) {
                    Ok(()) => wire::serialize_remove_thing_from_billing_group_response(
                        &wire::RemoveThingFromBillingGroupResponse {},
                    ),
                    Err(e) => iot_error_response(&e),
                }
            }
            // GET /billing-groups/{name}/things - ListThingsInBillingGroup
            ("GET", ["billing-groups", name, "things"]) => {
                let name = percent_decode(name);
                let st = state.read().await;
                match st.list_things_in_billing_group(&name) {
                    Ok(things) => wire::serialize_list_things_in_billing_group_response(
                        &wire::ListThingsInBillingGroupResponse {
                            things: Some(things),
                            next_token: None,
                        },
                    ),
                    Err(e) => iot_error_response(&e),
                }
            }

            // ==================== Certificate ====================
            // POST /certificates - CreateCertificateFromCsr
            ("POST", ["certificates"]) => {
                let input = match wire::deserialize_create_certificate_from_csr_request(
                    &request,
                    &[],
                    &query_params,
                ) {
                    Ok(v) => v,
                    Err(e) => return rest_json_error(400, "ValidationException", &e),
                };
                let csr = input.certificate_signing_request.as_str();
                let set_active = input.set_as_active.unwrap_or(false);
                if !csr.is_empty() {
                    let mut st = state.write().await;
                    match st.create_certificate_from_csr(csr, set_active, &region, account_id) {
                        Ok((id, arn, pem)) => wire::serialize_create_certificate_from_csr_response(
                            &wire::CreateCertificateFromCsrResponse {
                                certificate_id: Some(id),
                                certificate_arn: Some(arn),
                                certificate_pem: Some(pem),
                            },
                        ),
                        Err(e) => iot_error_response(&e),
                    }
                } else {
                    rest_json_error(
                        400,
                        "ValidationException",
                        "Missing certificateSigningRequest",
                    )
                }
            }
            // POST /keys-and-certificate - CreateKeysAndCertificate
            ("POST", ["keys-and-certificate"]) => {
                let set_active = query_params
                    .get("setAsActive")
                    .map(|v| v == "true")
                    .unwrap_or(false);
                let mut st = state.write().await;
                match st.create_keys_and_certificate(set_active, &region, account_id) {
                    Ok((id, arn, pem, pub_key, priv_key)) => {
                        wire::serialize_create_keys_and_certificate_response(
                            &wire::CreateKeysAndCertificateResponse {
                                certificate_arn: Some(arn),
                                certificate_id: Some(id),
                                certificate_pem: Some(pem),
                                key_pair: Some(wire::KeyPair {
                                    public_key: Some(pub_key),
                                    private_key: Some(priv_key),
                                }),
                            },
                        )
                    }
                    Err(e) => iot_error_response(&e),
                }
            }
            // GET /certificates - ListCertificates
            ("GET", ["certificates"]) => {
                let st = state.read().await;
                let certs = st.list_certificates();
                let entries: Vec<wire::Certificate> = certs
                    .iter()
                    .map(|c| wire::Certificate {
                        certificate_arn: Some(c.certificate_arn.clone()),
                        certificate_id: Some(c.certificate_id.clone()),
                        creation_date: Some(c.creation_date),
                        status: Some(c.status.clone()),
                        ..Default::default()
                    })
                    .collect();
                wire::serialize_list_certificates_response(&wire::ListCertificatesResponse {
                    certificates: Some(entries),
                    next_marker: None,
                })
            }
            // GET /certificates/{certId} - DescribeCertificate
            ("GET", ["certificates", cert_id]) => {
                let cert_id = percent_decode(cert_id);
                let st = state.read().await;
                match st.describe_certificate(&cert_id) {
                    Ok(c) => wire::serialize_describe_certificate_response(
                        &wire::DescribeCertificateResponse {
                            certificate_description: Some(wire::CertificateDescription {
                                certificate_id: Some(c.certificate_id.clone()),
                                certificate_arn: Some(c.certificate_arn.clone()),
                                certificate_pem: Some(c.certificate_pem.clone()),
                                status: Some(c.status.clone()),
                                creation_date: Some(c.creation_date),
                                owned_by: Some(c.owned_by.clone()),
                                certificate_mode: Some(c.mode.clone()),
                                ca_certificate_id: c.ca_certificate_id.clone(),
                                ..Default::default()
                            }),
                        },
                    ),
                    Err(e) => iot_error_response(&e),
                }
            }
            // PUT /certificates/{certId} - UpdateCertificate
            ("PUT", ["certificates", cert_id]) => {
                let cert_id = percent_decode(cert_id);
                let new_status = query_params
                    .get("newStatus")
                    .map(|s| s.as_str())
                    .unwrap_or("INACTIVE");
                let mut st = state.write().await;
                match st.update_certificate(&cert_id, new_status) {
                    Ok(()) => MockResponse::rest_json(200, "{}".to_string()),
                    Err(e) => iot_error_response(&e),
                }
            }
            // DELETE /certificates/{certId} - DeleteCertificate
            ("DELETE", ["certificates", cert_id]) => {
                let cert_id = percent_decode(cert_id);
                let mut st = state.write().await;
                match st.delete_certificate(&cert_id) {
                    Ok(()) => MockResponse::rest_json(200, "{}".to_string()),
                    Err(e) => iot_error_response(&e),
                }
            }
            // GET /certificates-by-ca/{caCertId} - ListCertificatesByCA
            ("GET", ["certificates-by-ca", ca_cert_id]) => {
                let ca_cert_id = percent_decode(ca_cert_id);
                let st = state.read().await;
                let certs = st.list_certificates_by_ca(&ca_cert_id);
                let entries: Vec<wire::Certificate> = certs
                    .iter()
                    .map(|c| wire::Certificate {
                        certificate_arn: Some(c.certificate_arn.clone()),
                        certificate_id: Some(c.certificate_id.clone()),
                        creation_date: Some(c.creation_date),
                        status: Some(c.status.clone()),
                        ..Default::default()
                    })
                    .collect();
                wire::serialize_list_certificates_by_c_a_response(
                    &wire::ListCertificatesByCAResponse {
                        certificates: Some(entries),
                        next_marker: None,
                    },
                )
            }
            // POST /certificate/register - RegisterCertificate
            ("POST", ["certificate", "register"]) => {
                let input = match wire::deserialize_register_certificate_request(
                    &request,
                    &[],
                    &query_params,
                ) {
                    Ok(v) => v,
                    Err(e) => return rest_json_error(400, "ValidationException", &e),
                };
                let cert_pem = input.certificate_pem.as_str();
                let ca_pem = input.ca_certificate_pem.as_deref();
                let status = input.status.as_deref();
                let mut st = state.write().await;
                match st.register_certificate(cert_pem, ca_pem, status, &region, account_id) {
                    Ok((id, arn)) => wire::serialize_register_certificate_response(
                        &wire::RegisterCertificateResponse {
                            certificate_id: Some(id),
                            certificate_arn: Some(arn),
                        },
                    ),
                    Err(e) => iot_error_response(&e),
                }
            }
            // POST /certificate/register-no-ca - RegisterCertificateWithoutCA
            ("POST", ["certificate", "register-no-ca"]) => {
                let input = match wire::deserialize_register_certificate_without_c_a_request(
                    &request,
                    &[],
                    &query_params,
                ) {
                    Ok(v) => v,
                    Err(e) => return rest_json_error(400, "ValidationException", &e),
                };
                let cert_pem = input.certificate_pem.as_str();
                let status = input.status.as_deref();
                let mut st = state.write().await;
                match st.register_certificate_without_ca(cert_pem, status, &region, account_id) {
                    Ok((id, arn)) => wire::serialize_register_certificate_without_c_a_response(
                        &wire::RegisterCertificateWithoutCAResponse {
                            certificate_id: Some(id),
                            certificate_arn: Some(arn),
                        },
                    ),
                    Err(e) => iot_error_response(&e),
                }
            }

            // ==================== CA Certificate ====================
            // POST /cacertificate - RegisterCACertificate
            ("POST", ["cacertificate"]) => {
                let input = match wire::deserialize_register_c_a_certificate_request(
                    &request,
                    &[],
                    &query_params,
                ) {
                    Ok(v) => v,
                    Err(e) => return rest_json_error(400, "ValidationException", &e),
                };
                let ca_pem = input.ca_certificate.as_str();
                let set_active = input.set_as_active.unwrap_or(false);
                let allow_auto = input.allow_auto_registration.unwrap_or(false);
                let mut st = state.write().await;
                match st
                    .register_ca_certificate(ca_pem, set_active, allow_auto, &region, account_id)
                {
                    Ok((id, arn)) => wire::serialize_register_c_a_certificate_response(
                        &wire::RegisterCACertificateResponse {
                            certificate_id: Some(id),
                            certificate_arn: Some(arn),
                        },
                    ),
                    Err(e) => iot_error_response(&e),
                }
            }
            // GET /cacertificate/{certId} - DescribeCACertificate
            ("GET", ["cacertificate", cert_id]) => {
                let cert_id = percent_decode(cert_id);
                let st = state.read().await;
                match st.describe_ca_certificate(&cert_id) {
                    Ok(ca) => wire::serialize_describe_c_a_certificate_response(
                        &wire::DescribeCACertificateResponse {
                            certificate_description: Some(wire::CACertificateDescription {
                                certificate_id: Some(ca.certificate_id.clone()),
                                certificate_arn: Some(ca.certificate_arn.clone()),
                                certificate_pem: Some(ca.certificate_pem.clone()),
                                status: Some(ca.status.clone()),
                                auto_registration_status: Some(ca.auto_registration_status.clone()),
                                creation_date: Some(ca.creation_date),
                                owned_by: Some(ca.owned_by.clone()),
                                certificate_mode: Some(ca.mode.clone()),
                                ..Default::default()
                            }),
                            registration_config: None,
                        },
                    ),
                    Err(e) => iot_error_response(&e),
                }
            }
            // PUT /cacertificate/{certId} - UpdateCACertificate
            ("PUT", ["cacertificate", cert_id]) => {
                let cert_id = percent_decode(cert_id);
                let new_status = query_params.get("newStatus").map(|s| s.as_str());
                let new_auto_reg = query_params
                    .get("newAutoRegistrationStatus")
                    .map(|s| s.as_str());
                let mut st = state.write().await;
                match st.update_ca_certificate(&cert_id, new_status, new_auto_reg) {
                    Ok(()) => MockResponse::rest_json(200, "{}".to_string()),
                    Err(e) => iot_error_response(&e),
                }
            }
            // DELETE /cacertificate/{certId} - DeleteCACertificate
            ("DELETE", ["cacertificate", cert_id]) => {
                let cert_id = percent_decode(cert_id);
                let mut st = state.write().await;
                match st.delete_ca_certificate(&cert_id) {
                    Ok(()) => wire::serialize_delete_c_a_certificate_response(
                        &wire::DeleteCACertificateResponse {},
                    ),
                    Err(e) => iot_error_response(&e),
                }
            }

            // ==================== Policy ====================
            // POST /policies/{policyName} - CreatePolicy
            ("POST", ["policies", pn]) => {
                let pn = percent_decode(pn);
                self.handle_create_policy(&state, &pn, &request, &query_params, &region, account_id)
                    .await
            }
            // GET /policies - ListPolicies
            ("GET", ["policies"]) => self.handle_list_policies(&state).await,
            // GET /policies/{policyName} - GetPolicy
            ("GET", ["policies", pn]) => {
                let pn = percent_decode(pn);
                self.handle_get_policy(&state, &pn).await
            }
            // DELETE /policies/{policyName} - DeletePolicy
            ("DELETE", ["policies", pn]) => {
                let pn = percent_decode(pn);
                let mut st = state.write().await;
                match st.delete_policy(&pn) {
                    Ok(()) => MockResponse::rest_json(200, "{}".to_string()),
                    Err(e) => iot_error_response(&e),
                }
            }
            // POST /policies/{policyName}/version - CreatePolicyVersion
            ("POST", ["policies", pn, "version"]) => {
                let pn = percent_decode(pn);
                let labels: &[(&str, &str)] = &[("policyName", pn.as_str())];
                let input = match wire::deserialize_create_policy_version_request(
                    &request,
                    labels,
                    &query_params,
                ) {
                    Ok(v) => v,
                    Err(e) => return rest_json_error(400, "ValidationException", &e),
                };
                let doc = input.policy_document.as_str();
                let set_default = input.set_as_default.unwrap_or(false);
                let mut st = state.write().await;
                match st.create_policy_version(&pn, doc, set_default) {
                    Ok(pv) => {
                        let policy = st.get_policy(&pn).unwrap();
                        wire::serialize_create_policy_version_response(
                            &wire::CreatePolicyVersionResponse {
                                is_default_version: Some(pv.is_default),
                                policy_arn: Some(policy.policy_arn.clone()),
                                policy_document: Some(pv.policy_document.clone()),
                                policy_version_id: Some(pv.version_id.clone()),
                            },
                        )
                    }
                    Err(e) => iot_error_response(&e),
                }
            }
            // GET /policies/{policyName}/version - ListPolicyVersions
            ("GET", ["policies", pn, "version"]) => {
                let pn = percent_decode(pn);
                let st = state.read().await;
                match st.list_policy_versions(&pn) {
                    Ok(versions) => {
                        let entries: Vec<wire::PolicyVersion> = versions
                            .iter()
                            .map(|v| wire::PolicyVersion {
                                version_id: Some(v.version_id.clone()),
                                is_default_version: Some(v.is_default),
                                create_date: Some(v.create_date),
                            })
                            .collect();
                        wire::serialize_list_policy_versions_response(
                            &wire::ListPolicyVersionsResponse {
                                policy_versions: Some(entries),
                            },
                        )
                    }
                    Err(e) => iot_error_response(&e),
                }
            }
            // GET /policies/{policyName}/version/{versionId} - GetPolicyVersion
            ("GET", ["policies", pn, "version", vid]) => {
                let pn = percent_decode(pn);
                let vid = percent_decode(vid);
                let st = state.read().await;
                match st.get_policy_version(&pn, &vid) {
                    Ok((policy, pv)) => wire::serialize_get_policy_version_response(
                        &wire::GetPolicyVersionResponse {
                            policy_name: Some(policy.policy_name.clone()),
                            policy_arn: Some(policy.policy_arn.clone()),
                            policy_document: Some(pv.policy_document.clone()),
                            policy_version_id: Some(pv.version_id.clone()),
                            is_default_version: Some(pv.is_default),
                            creation_date: Some(pv.create_date),
                            ..Default::default()
                        },
                    ),
                    Err(e) => iot_error_response(&e),
                }
            }
            // PATCH /policies/{policyName}/version/{versionId} - SetDefaultPolicyVersion
            ("PATCH", ["policies", pn, "version", vid]) => {
                let pn = percent_decode(pn);
                let vid = percent_decode(vid);
                let mut st = state.write().await;
                match st.set_default_policy_version(&pn, &vid) {
                    Ok(()) => MockResponse::rest_json(200, "{}".to_string()),
                    Err(e) => iot_error_response(&e),
                }
            }
            // DELETE /policies/{policyName}/version/{versionId} - DeletePolicyVersion
            ("DELETE", ["policies", pn, "version", vid]) => {
                let pn = percent_decode(pn);
                let vid = percent_decode(vid);
                let mut st = state.write().await;
                match st.delete_policy_version(&pn, &vid) {
                    Ok(()) => MockResponse::rest_json(200, "{}".to_string()),
                    Err(e) => iot_error_response(&e),
                }
            }

            // ==================== Policy Attach/Detach ====================
            // PUT /target-policies/{policyName} - AttachPolicy
            ("PUT", ["target-policies", pn]) => {
                let pn = percent_decode(pn);
                let labels: &[(&str, &str)] = &[("policyName", pn.as_str())];
                let input = match wire::deserialize_attach_policy_request(
                    &request,
                    labels,
                    &query_params,
                ) {
                    Ok(v) => v,
                    Err(e) => return rest_json_error(400, "ValidationException", &e),
                };
                let target = input.target.as_str();
                let mut st = state.write().await;
                match st.attach_policy(&pn, target) {
                    Ok(()) => wire::serialize_attach_policy_response(),
                    Err(e) => iot_error_response(&e),
                }
            }
            // POST /target-policies/{policyName} - DetachPolicy
            ("POST", ["target-policies", pn]) => {
                let pn = percent_decode(pn);
                let labels: &[(&str, &str)] = &[("policyName", pn.as_str())];
                let input = match wire::deserialize_detach_policy_request(
                    &request,
                    labels,
                    &query_params,
                ) {
                    Ok(v) => v,
                    Err(e) => return rest_json_error(400, "ValidationException", &e),
                };
                let target = input.target.as_str();
                let mut st = state.write().await;
                match st.detach_policy(&pn, target) {
                    Ok(()) => wire::serialize_detach_policy_response(),
                    Err(e) => iot_error_response(&e),
                }
            }
            // PUT /principal-policies/{policyName} - AttachPrincipalPolicy
            ("PUT", ["principal-policies", pn]) => {
                let pn = percent_decode(pn);
                let mut st = state.write().await;
                match st.attach_principal_policy(&pn, principal_header) {
                    Ok(()) => wire::serialize_attach_principal_policy_response(),
                    Err(e) => iot_error_response(&e),
                }
            }
            // DELETE /principal-policies/{policyName} - DetachPrincipalPolicy
            ("DELETE", ["principal-policies", pn]) => {
                let pn = percent_decode(pn);
                let mut st = state.write().await;
                match st.detach_principal_policy(&pn, principal_header) {
                    Ok(()) => wire::serialize_detach_principal_policy_response(),
                    Err(e) => iot_error_response(&e),
                }
            }
            // POST /attached-policies/{target} - ListAttachedPolicies
            ("POST", ["attached-policies", target]) => {
                let target = percent_decode(target);
                let st = state.read().await;
                let policies = st.list_attached_policies(&target);
                let entries: Vec<wire::Policy> = policies
                    .iter()
                    .map(|p| wire::Policy {
                        policy_name: Some(p.policy_name.clone()),
                        policy_arn: Some(p.policy_arn.clone()),
                    })
                    .collect();
                wire::serialize_list_attached_policies_response(
                    &wire::ListAttachedPoliciesResponse {
                        policies: Some(entries),
                        next_marker: None,
                    },
                )
            }
            // POST /policy-targets/{policyName} - ListTargetsForPolicy
            ("POST", ["policy-targets", pn]) => {
                let pn = percent_decode(pn);
                let st = state.read().await;
                match st.list_targets_for_policy(&pn) {
                    Ok(targets) => wire::serialize_list_targets_for_policy_response(
                        &wire::ListTargetsForPolicyResponse {
                            targets: Some(targets),
                            next_marker: None,
                        },
                    ),
                    Err(e) => iot_error_response(&e),
                }
            }
            // GET /policy-principals - ListPolicyPrincipals
            ("GET", ["policy-principals"]) => {
                let st = state.read().await;
                match st.list_policy_principals(policy_name_header) {
                    Ok(principals) => wire::serialize_list_policy_principals_response(
                        &wire::ListPolicyPrincipalsResponse {
                            principals: Some(principals),
                            next_marker: None,
                        },
                    ),
                    Err(e) => iot_error_response(&e),
                }
            }
            // GET /principal-policies - ListPrincipalPolicies
            ("GET", ["principal-policies"]) => {
                let st = state.read().await;
                let policies = st.list_principal_policies(principal_header);
                let entries: Vec<wire::Policy> = policies
                    .iter()
                    .map(|p| wire::Policy {
                        policy_name: Some(p.policy_name.clone()),
                        policy_arn: Some(p.policy_arn.clone()),
                    })
                    .collect();
                wire::serialize_list_principal_policies_response(
                    &wire::ListPrincipalPoliciesResponse {
                        policies: Some(entries),
                        next_marker: None,
                    },
                )
            }

            // ==================== Thing Principal ====================
            // PUT /things/{thingName}/principals - AttachThingPrincipal
            ("PUT", ["things", thing_name, "principals"]) => {
                let thing_name = percent_decode(thing_name);
                let mut st = state.write().await;
                match st.attach_thing_principal(&thing_name, principal_header) {
                    Ok(()) => wire::serialize_attach_thing_principal_response(
                        &wire::AttachThingPrincipalResponse {},
                    ),
                    Err(e) => iot_error_response(&e),
                }
            }
            // DELETE /things/{thingName}/principals - DetachThingPrincipal
            ("DELETE", ["things", thing_name, "principals"]) => {
                let thing_name = percent_decode(thing_name);
                let mut st = state.write().await;
                match st.detach_thing_principal(&thing_name, principal_header) {
                    Ok(()) => wire::serialize_detach_thing_principal_response(
                        &wire::DetachThingPrincipalResponse {},
                    ),
                    Err(e) => iot_error_response(&e),
                }
            }
            // GET /things/{thingName}/principals - ListThingPrincipals
            ("GET", ["things", thing_name, "principals"]) => {
                let thing_name = percent_decode(thing_name);
                let st = state.read().await;
                match st.list_thing_principals(&thing_name) {
                    Ok(principals) => wire::serialize_list_thing_principals_response(
                        &wire::ListThingPrincipalsResponse {
                            principals: Some(principals),
                            next_token: None,
                        },
                    ),
                    Err(e) => iot_error_response(&e),
                }
            }
            // GET /things/{thingName}/principals-v2 - ListThingPrincipalsV2
            ("GET", ["things", thing_name, "principals-v2"]) => {
                let thing_name = percent_decode(thing_name);
                let st = state.read().await;
                match st.list_thing_principals_v2(&thing_name) {
                    Ok(principals) => {
                        let entries: Vec<wire::ThingPrincipalObject> = principals
                            .iter()
                            .map(|p| wire::ThingPrincipalObject {
                                principal: Some(p.clone()),
                                thing_principal_type: Some("EXCLUSIVE_THING".to_string()),
                            })
                            .collect();
                        wire::serialize_list_thing_principals_v2_response(
                            &wire::ListThingPrincipalsV2Response {
                                thing_principal_objects: Some(entries),
                                next_token: None,
                            },
                        )
                    }
                    Err(e) => iot_error_response(&e),
                }
            }
            // GET /principals/things - ListPrincipalThings
            ("GET", ["principals", "things"]) => {
                let st = state.read().await;
                let things = st.list_principal_things(principal_header);
                wire::serialize_list_principal_things_response(&wire::ListPrincipalThingsResponse {
                    things: Some(things),
                    next_token: None,
                })
            }

            // ==================== RoleAlias ====================
            // POST /role-aliases/{roleAlias} - CreateRoleAlias
            ("POST", ["role-aliases", ra]) => {
                let ra = percent_decode(ra);
                let labels: &[(&str, &str)] = &[("roleAlias", ra.as_str())];
                let input = match wire::deserialize_create_role_alias_request(
                    &request,
                    labels,
                    &query_params,
                ) {
                    Ok(v) => v,
                    Err(e) => return rest_json_error(400, "ValidationException", &e),
                };
                let role_arn = input.role_arn.as_str();
                let cred_dur = input.credential_duration_seconds;
                let mut st = state.write().await;
                match st.create_role_alias(&ra, role_arn, cred_dur, &region, account_id) {
                    Ok(alias) => {
                        wire::serialize_create_role_alias_response(&wire::CreateRoleAliasResponse {
                            role_alias: Some(alias.role_alias.clone()),
                            role_alias_arn: Some(alias.role_alias_arn.clone()),
                        })
                    }
                    Err(e) => iot_error_response(&e),
                }
            }
            // GET /role-aliases - ListRoleAliases
            ("GET", ["role-aliases"]) => {
                let st = state.read().await;
                let aliases = st.list_role_aliases();
                wire::serialize_list_role_aliases_response(&wire::ListRoleAliasesResponse {
                    role_aliases: Some(aliases),
                    next_marker: None,
                })
            }
            // GET /role-aliases/{roleAlias} - DescribeRoleAlias
            ("GET", ["role-aliases", ra]) => {
                let ra = percent_decode(ra);
                let st = state.read().await;
                match st.describe_role_alias(&ra) {
                    Ok(alias) => wire::serialize_describe_role_alias_response(
                        &wire::DescribeRoleAliasResponse {
                            role_alias_description: Some(wire::RoleAliasDescription {
                                role_alias: Some(alias.role_alias.clone()),
                                role_alias_arn: Some(alias.role_alias_arn.clone()),
                                role_arn: Some(alias.role_arn.clone()),
                                credential_duration_seconds: Some(
                                    alias.credential_duration_seconds,
                                ),
                                creation_date: Some(alias.creation_date),
                                last_modified_date: Some(alias.last_modified_date),
                                owner: Some(alias.owner.clone()),
                            }),
                        },
                    ),
                    Err(e) => iot_error_response(&e),
                }
            }
            // PUT /role-aliases/{roleAlias} - UpdateRoleAlias
            ("PUT", ["role-aliases", ra]) => {
                let ra = percent_decode(ra);
                let labels: &[(&str, &str)] = &[("roleAlias", ra.as_str())];
                let input = match wire::deserialize_update_role_alias_request(
                    &request,
                    labels,
                    &query_params,
                ) {
                    Ok(v) => v,
                    Err(e) => return rest_json_error(400, "ValidationException", &e),
                };
                let role_arn = input.role_arn.as_deref();
                let cred_dur = input.credential_duration_seconds;
                let mut st = state.write().await;
                match st.update_role_alias(&ra, role_arn, cred_dur) {
                    Ok(alias) => {
                        wire::serialize_update_role_alias_response(&wire::UpdateRoleAliasResponse {
                            role_alias: Some(alias.role_alias.clone()),
                            role_alias_arn: Some(alias.role_alias_arn.clone()),
                        })
                    }
                    Err(e) => iot_error_response(&e),
                }
            }
            // DELETE /role-aliases/{roleAlias} - DeleteRoleAlias
            ("DELETE", ["role-aliases", ra]) => {
                let ra = percent_decode(ra);
                let mut st = state.write().await;
                match st.delete_role_alias(&ra) {
                    Ok(()) => wire::serialize_delete_role_alias_response(
                        &wire::DeleteRoleAliasResponse {},
                    ),
                    Err(e) => iot_error_response(&e),
                }
            }

            // ==================== DomainConfiguration ====================
            // POST /domainConfigurations/{name} - CreateDomainConfiguration
            ("POST", ["domainConfigurations", name]) => {
                let name = percent_decode(name);
                let labels: &[(&str, &str)] = &[("domainConfigurationName", name.as_str())];
                let input = match wire::deserialize_create_domain_configuration_request(
                    &request,
                    labels,
                    &query_params,
                ) {
                    Ok(v) => v,
                    Err(e) => return rest_json_error(400, "ValidationException", &e),
                };
                let domain_name = input.domain_name.as_deref();
                let service_type = input.service_type.as_deref();
                let mut st = state.write().await;
                match st.create_domain_configuration(
                    &name,
                    domain_name,
                    service_type,
                    &region,
                    account_id,
                ) {
                    Ok(dc) => wire::serialize_create_domain_configuration_response(
                        &wire::CreateDomainConfigurationResponse {
                            domain_configuration_name: Some(dc.domain_configuration_name.clone()),
                            domain_configuration_arn: Some(dc.domain_configuration_arn.clone()),
                        },
                    ),
                    Err(e) => iot_error_response(&e),
                }
            }
            // GET /domainConfigurations - ListDomainConfigurations
            ("GET", ["domainConfigurations"]) => {
                let st = state.read().await;
                let dcs = st.list_domain_configurations();
                let entries: Vec<wire::DomainConfigurationSummary> = dcs
                    .iter()
                    .map(|dc| wire::DomainConfigurationSummary {
                        domain_configuration_name: Some(dc.domain_configuration_name.clone()),
                        domain_configuration_arn: Some(dc.domain_configuration_arn.clone()),
                        service_type: dc.service_type.clone(),
                    })
                    .collect();
                wire::serialize_list_domain_configurations_response(
                    &wire::ListDomainConfigurationsResponse {
                        domain_configurations: Some(entries),
                        next_marker: None,
                    },
                )
            }
            // GET /domainConfigurations/{name} - DescribeDomainConfiguration
            ("GET", ["domainConfigurations", name]) => {
                let name = percent_decode(name);
                let st = state.read().await;
                match st.describe_domain_configuration(&name) {
                    Ok(dc) => wire::serialize_describe_domain_configuration_response(
                        &wire::DescribeDomainConfigurationResponse {
                            domain_configuration_name: Some(dc.domain_configuration_name.clone()),
                            domain_configuration_arn: Some(dc.domain_configuration_arn.clone()),
                            domain_name: dc.domain_name.clone(),
                            domain_configuration_status: Some(
                                dc.domain_configuration_status.clone(),
                            ),
                            service_type: dc.service_type.clone(),
                            last_status_change_date: Some(dc.creation_date),
                            ..Default::default()
                        },
                    ),
                    Err(e) => iot_error_response(&e),
                }
            }
            // PUT /domainConfigurations/{name} - UpdateDomainConfiguration
            ("PUT", ["domainConfigurations", name]) => {
                let name = percent_decode(name);
                let labels: &[(&str, &str)] = &[("domainConfigurationName", name.as_str())];
                let input = match wire::deserialize_update_domain_configuration_request(
                    &request,
                    labels,
                    &query_params,
                ) {
                    Ok(v) => v,
                    Err(e) => return rest_json_error(400, "ValidationException", &e),
                };
                let dc_status = input.domain_configuration_status.as_deref();
                let mut st = state.write().await;
                match st.update_domain_configuration(&name, dc_status) {
                    Ok(dc) => wire::serialize_update_domain_configuration_response(
                        &wire::UpdateDomainConfigurationResponse {
                            domain_configuration_name: Some(dc.domain_configuration_name.clone()),
                            domain_configuration_arn: Some(dc.domain_configuration_arn.clone()),
                        },
                    ),
                    Err(e) => iot_error_response(&e),
                }
            }
            // DELETE /domainConfigurations/{name} - DeleteDomainConfiguration
            ("DELETE", ["domainConfigurations", name]) => {
                let name = percent_decode(name);
                let mut st = state.write().await;
                match st.delete_domain_configuration(&name) {
                    Ok(()) => wire::serialize_delete_domain_configuration_response(
                        &wire::DeleteDomainConfigurationResponse {},
                    ),
                    Err(e) => iot_error_response(&e),
                }
            }

            // ==================== Job ====================
            // PUT /jobs/{jobId} - CreateJob
            ("PUT", ["jobs", job_id]) => {
                let job_id = percent_decode(job_id);
                let labels: &[(&str, &str)] = &[("jobId", job_id.as_str())];
                let input =
                    match wire::deserialize_create_job_request(&request, labels, &query_params) {
                        Ok(v) => v,
                        Err(e) => return rest_json_error(400, "ValidationException", &e),
                    };
                let targets: Vec<String> = input.targets.clone();
                let document = input.document.as_deref();
                let document_source = input.document_source.as_deref();
                let description = input.description.as_deref();
                let target_selection = input.target_selection.as_deref();
                let mut st = state.write().await;
                match st.create_job(
                    &job_id,
                    targets,
                    document,
                    document_source,
                    description,
                    target_selection,
                    &region,
                    account_id,
                ) {
                    Ok(job) => wire::serialize_create_job_response(&wire::CreateJobResponse {
                        job_id: Some(job.job_id.clone()),
                        job_arn: Some(job.job_arn.clone()),
                        description: job.description.clone(),
                    }),
                    Err(e) => iot_error_response(&e),
                }
            }
            // GET /jobs - ListJobs
            ("GET", ["jobs"]) => {
                let st = state.read().await;
                let jobs = st.list_jobs();
                let entries: Vec<wire::JobSummary> = jobs
                    .iter()
                    .map(|j| wire::JobSummary {
                        job_id: Some(j.job_id.clone()),
                        job_arn: Some(j.job_arn.clone()),
                        status: Some(j.status.clone()),
                        created_at: Some(j.creation_date),
                        last_updated_at: Some(j.last_updated_date),
                        target_selection: j.target_selection.clone(),
                        ..Default::default()
                    })
                    .collect();
                wire::serialize_list_jobs_response(&wire::ListJobsResponse {
                    jobs: Some(entries),
                    next_token: None,
                })
            }
            // GET /jobs/{jobId} - DescribeJob
            ("GET", ["jobs", job_id]) => {
                let job_id = percent_decode(job_id);
                let st = state.read().await;
                match st.describe_job(&job_id) {
                    Ok(job) => wire::serialize_describe_job_response(&wire::DescribeJobResponse {
                        job: Some(wire::Job {
                            job_id: Some(job.job_id.clone()),
                            job_arn: Some(job.job_arn.clone()),
                            description: job.description.clone(),
                            targets: Some(job.targets.clone()),
                            status: Some(job.status.clone()),
                            target_selection: job.target_selection.clone(),
                            created_at: Some(job.creation_date),
                            last_updated_at: Some(job.last_updated_date),
                            completed_at: job.completed_date,
                            comment: job.comment.clone(),
                            reason_code: job.reason_code.clone(),
                            force_canceled: job.force_canceled,
                            ..Default::default()
                        }),
                        document_source: job.document_source.clone(),
                    }),
                    Err(e) => iot_error_response(&e),
                }
            }
            // PUT /jobs/{jobId}/cancel - CancelJob
            ("PUT", ["jobs", job_id, "cancel"]) => {
                let job_id = percent_decode(job_id);
                let labels: &[(&str, &str)] = &[("jobId", job_id.as_str())];
                let input =
                    match wire::deserialize_cancel_job_request(&request, labels, &query_params) {
                        Ok(v) => v,
                        Err(e) => return rest_json_error(400, "ValidationException", &e),
                    };
                let reason_code = input.reason_code.as_deref();
                let comment = input.comment.as_deref();
                let force = input.force.unwrap_or(false);
                let mut st = state.write().await;
                match st.cancel_job(&job_id, reason_code, comment, force) {
                    Ok(job) => wire::serialize_cancel_job_response(&wire::CancelJobResponse {
                        job_id: Some(job.job_id.clone()),
                        job_arn: Some(job.job_arn.clone()),
                        description: job.description.clone(),
                        ..Default::default()
                    }),
                    Err(e) => iot_error_response(&e),
                }
            }
            // DELETE /jobs/{jobId} - DeleteJob
            ("DELETE", ["jobs", job_id]) => {
                let job_id = percent_decode(job_id);
                let mut st = state.write().await;
                match st.delete_job(&job_id) {
                    Ok(()) => MockResponse::rest_json(200, "{}".to_string()),
                    Err(e) => iot_error_response(&e),
                }
            }
            // GET /jobs/{jobId}/job-document - GetJobDocument
            ("GET", ["jobs", job_id, "job-document"]) => {
                let job_id = percent_decode(job_id);
                let st = state.read().await;
                match st.get_job_document(&job_id) {
                    Ok(doc) => {
                        wire::serialize_get_job_document_response(&wire::GetJobDocumentResponse {
                            document: doc,
                        })
                    }
                    Err(e) => iot_error_response(&e),
                }
            }
            // GET /jobs/{jobId}/things - ListJobExecutionsForJob
            ("GET", ["jobs", job_id, "things"]) => {
                let job_id = percent_decode(job_id);
                let st = state.read().await;
                match st.list_job_executions_for_job(&job_id) {
                    Ok(execs) => {
                        let entries: Vec<wire::JobExecutionSummaryForJob> = execs
                            .iter()
                            .map(|(tn, e)| wire::JobExecutionSummaryForJob {
                                thing_arn: Some(format!(
                                    "arn:aws:iot:{region}:{account_id}:thing/{tn}"
                                )),
                                job_execution_summary: Some(wire::JobExecutionSummary {
                                    status: Some(e.status.clone()),
                                    queued_at: Some(e.queued_at),
                                    last_updated_at: Some(e.last_updated_at),
                                    execution_number: Some(e.execution_number),
                                    ..Default::default()
                                }),
                            })
                            .collect();
                        wire::serialize_list_job_executions_for_job_response(
                            &wire::ListJobExecutionsForJobResponse {
                                execution_summaries: Some(entries),
                                next_token: None,
                            },
                        )
                    }
                    Err(e) => iot_error_response(&e),
                }
            }
            // GET /things/{thingName}/jobs - ListJobExecutionsForThing
            ("GET", ["things", thing_name, "jobs"]) => {
                let thing_name = percent_decode(thing_name);
                let st = state.read().await;
                let execs = st.list_job_executions_for_thing(&thing_name);
                let entries: Vec<wire::JobExecutionSummaryForThing> = execs
                    .iter()
                    .map(|(jid, e)| wire::JobExecutionSummaryForThing {
                        job_id: Some(jid.to_string()),
                        job_execution_summary: Some(wire::JobExecutionSummary {
                            status: Some(e.status.clone()),
                            queued_at: Some(e.queued_at),
                            last_updated_at: Some(e.last_updated_at),
                            execution_number: Some(e.execution_number),
                            ..Default::default()
                        }),
                    })
                    .collect();
                wire::serialize_list_job_executions_for_thing_response(
                    &wire::ListJobExecutionsForThingResponse {
                        execution_summaries: Some(entries),
                        next_token: None,
                    },
                )
            }
            // GET /things/{thingName}/jobs/{jobId} - DescribeJobExecution
            ("GET", ["things", thing_name, "jobs", job_id]) => {
                let thing_name = percent_decode(thing_name);
                let job_id = percent_decode(job_id);
                let st = state.read().await;
                match st.describe_job_execution(&job_id, &thing_name) {
                    Ok((job, exec)) => wire::serialize_describe_job_execution_response(
                        &wire::DescribeJobExecutionResponse {
                            execution: Some(wire::JobExecution {
                                job_id: Some(job.job_id.clone()),
                                thing_arn: Some(format!(
                                    "arn:aws:iot:{region}:{account_id}:thing/{thing_name}"
                                )),
                                status: Some(exec.status.clone()),
                                queued_at: Some(exec.queued_at),
                                last_updated_at: Some(exec.last_updated_at),
                                execution_number: Some(exec.execution_number),
                                version_number: Some(exec.version_number),
                                ..Default::default()
                            }),
                        },
                    ),
                    Err(e) => iot_error_response(&e),
                }
            }
            // PUT /things/{thingName}/jobs/{jobId}/cancel - CancelJobExecution
            ("PUT", ["things", thing_name, "jobs", job_id, "cancel"]) => {
                let thing_name = percent_decode(thing_name);
                let job_id = percent_decode(job_id);
                let mut st = state.write().await;
                match st.cancel_job_execution(&job_id, &thing_name) {
                    Ok(()) => wire::serialize_cancel_job_execution_response(),
                    Err(e) => iot_error_response(&e),
                }
            }
            // DELETE /things/{thingName}/jobs/{jobId}/executionNumber/{execNum} - DeleteJobExecution
            (
                "DELETE",
                [
                    "things",
                    thing_name,
                    "jobs",
                    job_id,
                    "executionNumber",
                    _exec_num,
                ],
            ) => {
                let thing_name = percent_decode(thing_name);
                let job_id = percent_decode(job_id);
                let mut st = state.write().await;
                match st.delete_job_execution(&job_id, &thing_name) {
                    Ok(()) => MockResponse::rest_json(200, "{}".to_string()),
                    Err(e) => iot_error_response(&e),
                }
            }

            // ==================== JobTemplate ====================
            // PUT /job-templates/{id} - CreateJobTemplate
            ("PUT", ["job-templates", jt_id]) => {
                let jt_id = percent_decode(jt_id);
                let labels: &[(&str, &str)] = &[("jobTemplateId", jt_id.as_str())];
                let input = match wire::deserialize_create_job_template_request(
                    &request,
                    labels,
                    &query_params,
                ) {
                    Ok(v) => v,
                    Err(e) => return rest_json_error(400, "ValidationException", &e),
                };
                let description = input.description.as_str();
                let document = input.document.as_deref();
                let document_source = input.document_source.as_deref();
                let mut st = state.write().await;
                match st.create_job_template(
                    &jt_id,
                    description,
                    document,
                    document_source,
                    &region,
                    account_id,
                ) {
                    Ok(jt) => wire::serialize_create_job_template_response(
                        &wire::CreateJobTemplateResponse {
                            job_template_id: Some(jt.job_template_id.clone()),
                            job_template_arn: Some(jt.job_template_arn.clone()),
                        },
                    ),
                    Err(e) => iot_error_response(&e),
                }
            }
            // GET /job-templates - ListJobTemplates
            ("GET", ["job-templates"]) => {
                let st = state.read().await;
                let jts = st.list_job_templates();
                let entries: Vec<wire::JobTemplateSummary> = jts
                    .iter()
                    .map(|jt| wire::JobTemplateSummary {
                        job_template_id: Some(jt.job_template_id.clone()),
                        job_template_arn: Some(jt.job_template_arn.clone()),
                        description: Some(jt.description.clone()),
                        created_at: Some(jt.creation_date),
                    })
                    .collect();
                wire::serialize_list_job_templates_response(&wire::ListJobTemplatesResponse {
                    job_templates: Some(entries),
                    next_token: None,
                })
            }
            // GET /job-templates/{id} - DescribeJobTemplate
            ("GET", ["job-templates", jt_id]) => {
                let jt_id = percent_decode(jt_id);
                let st = state.read().await;
                match st.describe_job_template(&jt_id) {
                    Ok(jt) => wire::serialize_describe_job_template_response(
                        &wire::DescribeJobTemplateResponse {
                            job_template_id: Some(jt.job_template_id.clone()),
                            job_template_arn: Some(jt.job_template_arn.clone()),
                            description: Some(jt.description.clone()),
                            document: jt.document.clone(),
                            document_source: jt.document_source.clone(),
                            created_at: Some(jt.creation_date),
                            ..Default::default()
                        },
                    ),
                    Err(e) => iot_error_response(&e),
                }
            }
            // DELETE /job-templates/{id} - DeleteJobTemplate
            ("DELETE", ["job-templates", jt_id]) => {
                let jt_id = percent_decode(jt_id);
                let mut st = state.write().await;
                match st.delete_job_template(&jt_id) {
                    Ok(()) => MockResponse::rest_json(200, "{}".to_string()),
                    Err(e) => iot_error_response(&e),
                }
            }

            // ==================== TopicRule ====================
            // POST /rules/{ruleName} - CreateTopicRule
            ("POST", ["rules", rule_name])
                if !segments.contains(&"enable") && !segments.contains(&"disable") =>
            {
                let rule_name = percent_decode(rule_name);
                let labels: &[(&str, &str)] = &[("ruleName", rule_name.as_str())];
                let input = match wire::deserialize_create_topic_rule_request(
                    &request,
                    labels,
                    &query_params,
                ) {
                    Ok(v) => v,
                    Err(e) => return rest_json_error(400, "ValidationException", &e),
                };
                let payload = &input.topic_rule_payload;
                let sql = payload.sql.as_str();
                let description = payload.description.as_deref();
                let actions =
                    serde_json::to_value(&payload.actions).unwrap_or(Value::Array(vec![]));
                let error_action = payload
                    .error_action
                    .as_ref()
                    .and_then(|a| serde_json::to_value(a).ok());
                let aws_iot_sql_version = payload.aws_iot_sql_version.as_deref();
                let rule_disabled = payload.rule_disabled.unwrap_or(false);
                let mut st = state.write().await;
                match st.create_topic_rule(
                    &rule_name,
                    sql,
                    description,
                    actions,
                    error_action,
                    aws_iot_sql_version,
                    rule_disabled,
                    &region,
                    account_id,
                ) {
                    Ok(()) => MockResponse::rest_json(200, "{}".to_string()),
                    Err(e) => iot_error_response(&e),
                }
            }
            // GET /rules - ListTopicRules
            ("GET", ["rules"]) => {
                let st = state.read().await;
                let rules = st.list_topic_rules();
                let entries: Vec<wire::TopicRuleListItem> = rules
                    .iter()
                    .map(|r| wire::TopicRuleListItem {
                        rule_name: Some(r.rule_name.clone()),
                        rule_arn: Some(r.rule_arn.clone()),
                        rule_disabled: Some(r.rule_disabled),
                        created_at: Some(r.creation_date),
                        topic_pattern: Some(r.sql.clone()),
                    })
                    .collect();
                wire::serialize_list_topic_rules_response(&wire::ListTopicRulesResponse {
                    rules: Some(entries),
                    next_token: None,
                })
            }
            // GET /rules/{ruleName} - GetTopicRule
            ("GET", ["rules", rule_name]) => {
                let rule_name = percent_decode(rule_name);
                let st = state.read().await;
                match st.get_topic_rule(&rule_name) {
                    Ok(r) => wire::serialize_get_topic_rule_response(&wire::GetTopicRuleResponse {
                        rule_arn: Some(r.rule_arn.clone()),
                        rule: Some(wire::TopicRule {
                            rule_name: Some(r.rule_name.clone()),
                            sql: Some(r.sql.clone()),
                            description: r.description.clone(),
                            rule_disabled: Some(r.rule_disabled),
                            created_at: Some(r.creation_date),
                            aws_iot_sql_version: r.aws_iot_sql_version.clone(),
                            ..Default::default()
                        }),
                    }),
                    Err(e) => iot_error_response(&e),
                }
            }
            // PATCH /rules/{ruleName} - ReplaceTopicRule
            ("PATCH", ["rules", rule_name]) => {
                let rule_name = percent_decode(rule_name);
                let labels: &[(&str, &str)] = &[("ruleName", rule_name.as_str())];
                let input = match wire::deserialize_replace_topic_rule_request(
                    &request,
                    labels,
                    &query_params,
                ) {
                    Ok(v) => v,
                    Err(e) => return rest_json_error(400, "ValidationException", &e),
                };
                let payload = &input.topic_rule_payload;
                let sql = payload.sql.as_str();
                let description = payload.description.as_deref();
                let actions =
                    serde_json::to_value(&payload.actions).unwrap_or(Value::Array(vec![]));
                let error_action = payload
                    .error_action
                    .as_ref()
                    .and_then(|a| serde_json::to_value(a).ok());
                let aws_iot_sql_version = payload.aws_iot_sql_version.as_deref();
                let rule_disabled = payload.rule_disabled.unwrap_or(false);
                let mut st = state.write().await;
                match st.replace_topic_rule(
                    &rule_name,
                    sql,
                    description,
                    actions,
                    error_action,
                    aws_iot_sql_version,
                    rule_disabled,
                ) {
                    Ok(()) => MockResponse::rest_json(200, "{}".to_string()),
                    Err(e) => iot_error_response(&e),
                }
            }
            // DELETE /rules/{ruleName} - DeleteTopicRule
            ("DELETE", ["rules", rule_name]) => {
                let rule_name = percent_decode(rule_name);
                let mut st = state.write().await;
                match st.delete_topic_rule(&rule_name) {
                    Ok(()) => MockResponse::rest_json(200, "{}".to_string()),
                    Err(e) => iot_error_response(&e),
                }
            }
            // POST /rules/{ruleName}/enable - EnableTopicRule
            ("POST", ["rules", rule_name, "enable"]) => {
                let rule_name = percent_decode(rule_name);
                let mut st = state.write().await;
                match st.enable_topic_rule(&rule_name) {
                    Ok(()) => MockResponse::rest_json(200, "{}".to_string()),
                    Err(e) => iot_error_response(&e),
                }
            }
            // POST /rules/{ruleName}/disable - DisableTopicRule
            ("POST", ["rules", rule_name, "disable"]) => {
                let rule_name = percent_decode(rule_name);
                let mut st = state.write().await;
                match st.disable_topic_rule(&rule_name) {
                    Ok(()) => MockResponse::rest_json(200, "{}".to_string()),
                    Err(e) => iot_error_response(&e),
                }
            }

            // ==================== Misc ====================
            // GET /endpoint - DescribeEndpoint
            ("GET", ["endpoint"]) => {
                let st = state.read().await;
                let addr = st.describe_endpoint(&region);
                wire::serialize_describe_endpoint_response(&wire::DescribeEndpointResponse {
                    endpoint_address: Some(addr),
                })
            }
            // GET /registrationcode - GetRegistrationCode
            ("GET", ["registrationcode"]) => {
                let mut st = state.write().await;
                let code = st.get_registration_code();
                wire::serialize_get_registration_code_response(&wire::GetRegistrationCodeResponse {
                    registration_code: Some(code),
                })
            }
            // GET /indexing/config - GetIndexingConfiguration
            ("GET", ["indexing", "config"]) => wire::serialize_get_indexing_configuration_response(
                &wire::GetIndexingConfigurationResponse {
                    thing_indexing_configuration: Some(wire::ThingIndexingConfiguration {
                        thing_indexing_mode: "OFF".to_string(),
                        ..Default::default()
                    }),
                    thing_group_indexing_configuration: Some(
                        wire::ThingGroupIndexingConfiguration {
                            thing_group_indexing_mode: "OFF".to_string(),
                            ..Default::default()
                        },
                    ),
                },
            ),
            // POST /indexing/config - UpdateIndexingConfiguration
            ("POST", ["indexing", "config"]) => {
                wire::serialize_update_indexing_configuration_response(
                    &wire::UpdateIndexingConfigurationResponse {},
                )
            }
            // POST /indices/search - SearchIndex
            ("POST", ["indices", "search"]) => {
                let input =
                    match wire::deserialize_search_index_request(&request, &[], &query_params) {
                        Ok(v) => v,
                        Err(e) => return rest_json_error(400, "ValidationException", &e),
                    };
                let query_string = if input.query_string.is_empty() {
                    "*"
                } else {
                    input.query_string.as_str()
                };
                let st = state.read().await;
                let things = st.search_index(query_string);
                let docs: Vec<wire::ThingDocument> = things
                    .iter()
                    .map(|t| wire::ThingDocument {
                        thing_name: Some(t.thing_name.clone()),
                        thing_id: Some(t.thing_id.clone()),
                        attributes: if t.attributes.is_empty() {
                            None
                        } else {
                            Some(t.attributes.clone())
                        },
                        ..Default::default()
                    })
                    .collect();
                wire::serialize_search_index_response(&wire::SearchIndexResponse {
                    things: Some(docs),
                    next_token: None,
                    ..Default::default()
                })
            }

            // ==================== Tags ====================
            // POST /tags - TagResource
            ("POST", ["tags"]) => {
                let input =
                    match wire::deserialize_tag_resource_request(&request, &[], &query_params) {
                        Ok(v) => v,
                        Err(e) => return rest_json_error(400, "ValidationException", &e),
                    };
                let resource_arn = input.resource_arn.as_str();
                let tags: Vec<(String, String)> = input
                    .tags
                    .iter()
                    .map(|t| (t.key.clone(), t.value.clone().unwrap_or_default()))
                    .collect();
                let mut st = state.write().await;
                st.tag_resource(resource_arn, tags);
                wire::serialize_tag_resource_response(&wire::TagResourceResponse {})
            }
            // POST /untag - UntagResource
            ("POST", ["untag"]) => {
                let input =
                    match wire::deserialize_untag_resource_request(&request, &[], &query_params) {
                        Ok(v) => v,
                        Err(e) => return rest_json_error(400, "ValidationException", &e),
                    };
                let resource_arn = input.resource_arn.as_str();
                let tag_keys: Vec<String> = input.tag_keys.clone();
                let mut st = state.write().await;
                st.untag_resource(resource_arn, &tag_keys);
                wire::serialize_untag_resource_response(&wire::UntagResourceResponse {})
            }
            // GET /tags?resourceArn=... - ListTagsForResource
            ("GET", ["tags"]) => {
                let resource_arn = query_params.get("resourceArn").cloned().unwrap_or_default();
                let st = state.read().await;
                let tags = st.list_tags_for_resource(&resource_arn);
                let entries: Vec<wire::Tag> = tags
                    .into_iter()
                    .map(|(k, v)| wire::Tag {
                        key: k,
                        value: Some(v),
                    })
                    .collect();
                wire::serialize_list_tags_for_resource_response(
                    &wire::ListTagsForResourceResponse {
                        tags: Some(entries),
                        next_token: None,
                    },
                )
            }

            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        }
    }

    // ==================== Handler methods ====================

    async fn handle_create_thing(
        &self,
        state: &Arc<tokio::sync::RwLock<IotState>>,
        thing_name: &str,
        request: &MockRequest,
        query_params: &HashMap<String, String>,
        region: &str,
        account_id: &str,
    ) -> MockResponse {
        let labels: &[(&str, &str)] = &[("thingName", thing_name)];
        let input = match wire::deserialize_create_thing_request(request, labels, query_params) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let thing_type_name = input.thing_type_name.as_deref();
        let attributes: HashMap<String, String> = input
            .attribute_payload
            .as_ref()
            .and_then(|ap| ap.attributes.clone())
            .unwrap_or_default();

        let mut state = state.write().await;
        match state.create_thing(thing_name, thing_type_name, attributes, region, account_id) {
            Ok(thing) => wire::serialize_create_thing_response(&wire::CreateThingResponse {
                thing_name: Some(thing.thing_name.clone()),
                thing_arn: Some(thing.thing_arn.clone()),
                thing_id: Some(thing.thing_id.clone()),
            }),
            Err(e) => iot_error_response(&e),
        }
    }

    async fn handle_describe_thing(
        &self,
        state: &Arc<tokio::sync::RwLock<IotState>>,
        thing_name: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.describe_thing(thing_name) {
            Ok(thing) => wire::serialize_describe_thing_response(&wire::DescribeThingResponse {
                thing_name: Some(thing.thing_name.clone()),
                thing_id: Some(thing.thing_id.clone()),
                thing_arn: Some(thing.thing_arn.clone()),
                attributes: if thing.attributes.is_empty() {
                    None
                } else {
                    Some(thing.attributes.clone())
                },
                version: Some(thing.version),
                default_client_id: Some(thing.thing_name.clone()),
                thing_type_name: thing.thing_type_name.clone(),
                billing_group_name: thing.billing_group_name.clone(),
            }),
            Err(e) => iot_error_response(&e),
        }
    }

    async fn handle_update_thing(
        &self,
        state: &Arc<tokio::sync::RwLock<IotState>>,
        thing_name: &str,
        request: &MockRequest,
        query_params: &HashMap<String, String>,
    ) -> MockResponse {
        let labels: &[(&str, &str)] = &[("thingName", thing_name)];
        let input = match wire::deserialize_update_thing_request(request, labels, query_params) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let thing_type_name = input.thing_type_name.as_deref();
        let expected_version = input.expected_version;
        let remove_thing_type = input.remove_thing_type.unwrap_or(false);
        let attributes: Option<HashMap<String, String>> = input
            .attribute_payload
            .as_ref()
            .and_then(|ap| ap.attributes.clone());

        let mut state = state.write().await;
        match state.update_thing(
            thing_name,
            thing_type_name,
            attributes,
            expected_version,
            remove_thing_type,
        ) {
            Ok(()) => wire::serialize_update_thing_response(&wire::UpdateThingResponse {}),
            Err(e) => iot_error_response(&e),
        }
    }

    async fn handle_delete_thing(
        &self,
        state: &Arc<tokio::sync::RwLock<IotState>>,
        thing_name: &str,
        expected_version: Option<i64>,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_thing(thing_name, expected_version) {
            Ok(()) => wire::serialize_delete_thing_response(&wire::DeleteThingResponse {}),
            Err(e) => iot_error_response(&e),
        }
    }

    async fn handle_list_things(&self, state: &Arc<tokio::sync::RwLock<IotState>>) -> MockResponse {
        let state = state.read().await;
        let things = state.list_things();
        let entries: Vec<wire::ThingAttribute> = things
            .iter()
            .map(|t| wire::ThingAttribute {
                thing_name: Some(t.thing_name.clone()),
                thing_arn: Some(t.thing_arn.clone()),
                attributes: if t.attributes.is_empty() {
                    None
                } else {
                    Some(t.attributes.clone())
                },
                version: Some(t.version),
                thing_type_name: t.thing_type_name.clone(),
            })
            .collect();
        wire::serialize_list_things_response(&wire::ListThingsResponse {
            next_token: None,
            things: Some(entries),
        })
    }

    async fn handle_create_thing_type(
        &self,
        state: &Arc<tokio::sync::RwLock<IotState>>,
        thing_type_name: &str,
        request: &MockRequest,
        query_params: &HashMap<String, String>,
        region: &str,
        account_id: &str,
    ) -> MockResponse {
        let labels: &[(&str, &str)] = &[("thingTypeName", thing_type_name)];
        let input = match wire::deserialize_create_thing_type_request(request, labels, query_params)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let props = input.thing_type_properties.as_ref();
        let description = props.and_then(|p| p.thing_type_description.as_deref());
        let searchable = props.and_then(|p| p.searchable_attributes.clone());
        let mut st = state.write().await;
        match st.create_thing_type(thing_type_name, description, searchable, region, account_id) {
            Ok(tt) => wire::serialize_create_thing_type_response(&wire::CreateThingTypeResponse {
                thing_type_name: Some(tt.thing_type_name.clone()),
                thing_type_arn: Some(tt.thing_type_arn.clone()),
                thing_type_id: Some(tt.thing_type_id.clone()),
            }),
            Err(e) => iot_error_response(&e),
        }
    }

    async fn handle_describe_thing_type(
        &self,
        state: &Arc<tokio::sync::RwLock<IotState>>,
        thing_type_name: &str,
    ) -> MockResponse {
        let st = state.read().await;
        match st.describe_thing_type(thing_type_name) {
            Ok(tt) => {
                wire::serialize_describe_thing_type_response(&wire::DescribeThingTypeResponse {
                    thing_type_name: Some(tt.thing_type_name.clone()),
                    thing_type_arn: Some(tt.thing_type_arn.clone()),
                    thing_type_id: Some(tt.thing_type_id.clone()),
                    thing_type_properties: Some(wire::ThingTypeProperties {
                        thing_type_description: tt.thing_type_description.clone(),
                        searchable_attributes: tt.searchable_attributes.clone(),
                        ..Default::default()
                    }),
                    thing_type_metadata: Some(wire::ThingTypeMetadata {
                        creation_date: Some(tt.creation_date),
                        deprecated: Some(tt.deprecated),
                        deprecation_date: tt.deprecation_date,
                    }),
                })
            }
            Err(e) => iot_error_response(&e),
        }
    }

    async fn handle_delete_thing_type(
        &self,
        state: &Arc<tokio::sync::RwLock<IotState>>,
        thing_type_name: &str,
    ) -> MockResponse {
        let mut st = state.write().await;
        match st.delete_thing_type(thing_type_name) {
            Ok(()) => wire::serialize_delete_thing_type_response(&wire::DeleteThingTypeResponse {}),
            Err(e) => iot_error_response(&e),
        }
    }

    async fn handle_list_thing_types(
        &self,
        state: &Arc<tokio::sync::RwLock<IotState>>,
    ) -> MockResponse {
        let st = state.read().await;
        let tts = st.list_thing_types();
        let entries: Vec<wire::ThingTypeDefinition> = tts
            .iter()
            .map(|tt| wire::ThingTypeDefinition {
                thing_type_name: Some(tt.thing_type_name.clone()),
                thing_type_arn: Some(tt.thing_type_arn.clone()),
                thing_type_properties: Some(wire::ThingTypeProperties {
                    thing_type_description: tt.thing_type_description.clone(),
                    searchable_attributes: tt.searchable_attributes.clone(),
                    ..Default::default()
                }),
                thing_type_metadata: Some(wire::ThingTypeMetadata {
                    creation_date: Some(tt.creation_date),
                    deprecated: Some(tt.deprecated),
                    deprecation_date: tt.deprecation_date,
                }),
            })
            .collect();
        wire::serialize_list_thing_types_response(&wire::ListThingTypesResponse {
            thing_types: Some(entries),
            next_token: None,
        })
    }

    async fn handle_create_thing_group(
        &self,
        state: &Arc<tokio::sync::RwLock<IotState>>,
        name: &str,
        request: &MockRequest,
        query_params: &HashMap<String, String>,
        region: &str,
        account_id: &str,
    ) -> MockResponse {
        let labels: &[(&str, &str)] = &[("thingGroupName", name)];
        let input =
            match wire::deserialize_create_thing_group_request(request, labels, query_params) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let parent = input.parent_group_name.as_deref();
        let props = input.thing_group_properties.as_ref();
        let description = props.and_then(|p| p.thing_group_description.as_deref());
        let attributes: HashMap<String, String> = props
            .and_then(|p| p.attribute_payload.as_ref())
            .and_then(|ap| ap.attributes.clone())
            .unwrap_or_default();
        let mut st = state.write().await;
        match st.create_thing_group(name, parent, description, attributes, region, account_id) {
            Ok(tg) => {
                wire::serialize_create_thing_group_response(&wire::CreateThingGroupResponse {
                    thing_group_name: Some(tg.thing_group_name.clone()),
                    thing_group_arn: Some(tg.thing_group_arn.clone()),
                    thing_group_id: Some(tg.thing_group_id.clone()),
                })
            }
            Err(e) => iot_error_response(&e),
        }
    }

    async fn handle_describe_thing_group(
        &self,
        state: &Arc<tokio::sync::RwLock<IotState>>,
        name: &str,
    ) -> MockResponse {
        let st = state.read().await;
        match st.describe_thing_group(name) {
            Ok(tg) => {
                wire::serialize_describe_thing_group_response(&wire::DescribeThingGroupResponse {
                    thing_group_name: Some(tg.thing_group_name.clone()),
                    thing_group_arn: Some(tg.thing_group_arn.clone()),
                    thing_group_id: Some(tg.thing_group_id.clone()),
                    version: Some(tg.version),
                    thing_group_properties: Some(wire::ThingGroupProperties {
                        thing_group_description: tg.thing_group_description.clone(),
                        attribute_payload: if tg.attributes.is_empty() {
                            None
                        } else {
                            Some(wire::AttributePayload {
                                attributes: Some(tg.attributes.clone()),
                                merge: None,
                            })
                        },
                    }),
                    thing_group_metadata: Some(wire::ThingGroupMetadata {
                        creation_date: Some(tg.creation_date),
                        parent_group_name: tg.parent_group_name.clone(),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
            }
            Err(e) => iot_error_response(&e),
        }
    }

    async fn handle_update_thing_group(
        &self,
        state: &Arc<tokio::sync::RwLock<IotState>>,
        name: &str,
        request: &MockRequest,
        query_params: &HashMap<String, String>,
    ) -> MockResponse {
        let labels: &[(&str, &str)] = &[("thingGroupName", name)];
        let input =
            match wire::deserialize_update_thing_group_request(request, labels, query_params) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let props = &input.thing_group_properties;
        let description = props.thing_group_description.as_deref();
        let attributes: Option<HashMap<String, String>> = props
            .attribute_payload
            .as_ref()
            .and_then(|ap| ap.attributes.clone());
        let ev = input.expected_version;
        let mut st = state.write().await;
        match st.update_thing_group(name, description, attributes, ev) {
            Ok(version) => {
                wire::serialize_update_thing_group_response(&wire::UpdateThingGroupResponse {
                    version: Some(version),
                })
            }
            Err(e) => iot_error_response(&e),
        }
    }

    async fn handle_list_thing_groups(
        &self,
        state: &Arc<tokio::sync::RwLock<IotState>>,
    ) -> MockResponse {
        let st = state.read().await;
        let gs = st.list_thing_groups();
        let entries: Vec<wire::GroupNameAndArn> = gs
            .iter()
            .map(|g| wire::GroupNameAndArn {
                group_name: Some(g.thing_group_name.clone()),
                group_arn: Some(g.thing_group_arn.clone()),
            })
            .collect();
        wire::serialize_list_thing_groups_response(&wire::ListThingGroupsResponse {
            thing_groups: Some(entries),
            next_token: None,
        })
    }

    async fn handle_create_billing_group(
        &self,
        state: &Arc<tokio::sync::RwLock<IotState>>,
        name: &str,
        request: &MockRequest,
        query_params: &HashMap<String, String>,
        region: &str,
        account_id: &str,
    ) -> MockResponse {
        let labels: &[(&str, &str)] = &[("billingGroupName", name)];
        let input =
            match wire::deserialize_create_billing_group_request(request, labels, query_params) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let description = input
            .billing_group_properties
            .as_ref()
            .and_then(|p| p.billing_group_description.as_deref());
        let mut st = state.write().await;
        match st.create_billing_group(name, description, region, account_id) {
            Ok(bg) => {
                wire::serialize_create_billing_group_response(&wire::CreateBillingGroupResponse {
                    billing_group_name: Some(bg.billing_group_name.clone()),
                    billing_group_arn: Some(bg.billing_group_arn.clone()),
                    billing_group_id: Some(bg.billing_group_id.clone()),
                })
            }
            Err(e) => iot_error_response(&e),
        }
    }

    async fn handle_describe_billing_group(
        &self,
        state: &Arc<tokio::sync::RwLock<IotState>>,
        name: &str,
    ) -> MockResponse {
        let st = state.read().await;
        match st.describe_billing_group(name) {
            Ok(bg) => wire::serialize_describe_billing_group_response(
                &wire::DescribeBillingGroupResponse {
                    billing_group_name: Some(bg.billing_group_name.clone()),
                    billing_group_arn: Some(bg.billing_group_arn.clone()),
                    billing_group_id: Some(bg.billing_group_id.clone()),
                    version: Some(bg.version),
                    billing_group_properties: Some(wire::BillingGroupProperties {
                        billing_group_description: bg.billing_group_description.clone(),
                    }),
                    billing_group_metadata: Some(wire::BillingGroupMetadata {
                        creation_date: Some(bg.creation_date),
                    }),
                },
            ),
            Err(e) => iot_error_response(&e),
        }
    }

    async fn handle_update_billing_group(
        &self,
        state: &Arc<tokio::sync::RwLock<IotState>>,
        name: &str,
        request: &MockRequest,
        query_params: &HashMap<String, String>,
    ) -> MockResponse {
        let labels: &[(&str, &str)] = &[("billingGroupName", name)];
        let input =
            match wire::deserialize_update_billing_group_request(request, labels, query_params) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let description = input
            .billing_group_properties
            .billing_group_description
            .as_deref();
        let ev = input.expected_version;
        let mut st = state.write().await;
        match st.update_billing_group(name, description, ev) {
            Ok(version) => {
                wire::serialize_update_billing_group_response(&wire::UpdateBillingGroupResponse {
                    version: Some(version),
                })
            }
            Err(e) => iot_error_response(&e),
        }
    }

    async fn handle_list_billing_groups(
        &self,
        state: &Arc<tokio::sync::RwLock<IotState>>,
    ) -> MockResponse {
        let st = state.read().await;
        let bgs = st.list_billing_groups();
        let entries: Vec<wire::GroupNameAndArn> = bgs
            .iter()
            .map(|bg| wire::GroupNameAndArn {
                group_name: Some(bg.billing_group_name.clone()),
                group_arn: Some(bg.billing_group_arn.clone()),
            })
            .collect();
        wire::serialize_list_billing_groups_response(&wire::ListBillingGroupsResponse {
            billing_groups: Some(entries),
            next_token: None,
        })
    }

    async fn handle_create_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<IotState>>,
        policy_name: &str,
        request: &MockRequest,
        query_params: &HashMap<String, String>,
        region: &str,
        account_id: &str,
    ) -> MockResponse {
        let labels: &[(&str, &str)] = &[("policyName", policy_name)];
        let input = match wire::deserialize_create_policy_request(request, labels, query_params) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let doc = if input.policy_document.is_empty() {
            "{}".to_string()
        } else {
            input.policy_document.clone()
        };
        let mut st = state.write().await;
        match st.create_policy(policy_name, doc.as_str(), region, account_id) {
            Ok(policy) => wire::serialize_create_policy_response(&wire::CreatePolicyResponse {
                policy_name: Some(policy.policy_name.clone()),
                policy_arn: Some(policy.policy_arn.clone()),
                policy_document: Some(policy.policy_document.clone()),
                policy_version_id: Some(policy.default_version_id.clone()),
            }),
            Err(e) => iot_error_response(&e),
        }
    }

    async fn handle_get_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<IotState>>,
        policy_name: &str,
    ) -> MockResponse {
        let st = state.read().await;
        match st.get_policy(policy_name) {
            Ok(policy) => wire::serialize_get_policy_response(&wire::GetPolicyResponse {
                policy_name: Some(policy.policy_name.clone()),
                policy_arn: Some(policy.policy_arn.clone()),
                policy_document: Some(policy.policy_document.clone()),
                default_version_id: Some(policy.default_version_id.clone()),
                creation_date: Some(policy.creation_date),
                last_modified_date: Some(policy.last_modified_date),
                generation_id: Some(policy.generation_id.clone()),
            }),
            Err(e) => iot_error_response(&e),
        }
    }

    async fn handle_list_policies(
        &self,
        state: &Arc<tokio::sync::RwLock<IotState>>,
    ) -> MockResponse {
        let st = state.read().await;
        let policies = st.list_policies();
        let entries: Vec<wire::Policy> = policies
            .iter()
            .map(|p| wire::Policy {
                policy_name: Some(p.policy_name.clone()),
                policy_arn: Some(p.policy_arn.clone()),
            })
            .collect();
        wire::serialize_list_policies_response(&wire::ListPoliciesResponse {
            policies: Some(entries),
            next_marker: None,
        })
    }
}

// ==================== Helper functions ====================

fn extract_path_and_query(uri: &str) -> (String, Option<String>) {
    // Strip scheme and authority, leaving everything from the first '/' of the
    // path onward. This works for AWS endpoints
    // (`https://iot.us-east-1.amazonaws.com/policies/Foo`) and for custom
    // endpoints used by tests (`http://127.0.0.1:1234/policies/Foo`).
    let after_scheme = uri
        .strip_prefix("https://")
        .or_else(|| uri.strip_prefix("http://"))
        .unwrap_or(uri);
    let path_and_query = match after_scheme.find('/') {
        Some(idx) => &after_scheme[idx..],
        None => "/",
    };

    if let Some(q) = path_and_query.find('?') {
        (
            path_and_query[..q].to_string(),
            Some(path_and_query[q + 1..].to_string()),
        )
    } else {
        (path_and_query.to_string(), None)
    }
}

fn parse_query(query: &Option<String>) -> HashMap<String, String> {
    let mut params = HashMap::new();
    if let Some(q) = query {
        for part in q.split('&') {
            if let Some(eq) = part.find('=') {
                let key = &part[..eq];
                let value = &part[eq + 1..];
                params.insert(key.to_string(), percent_decode(value));
            }
        }
    }
    params
}

fn percent_decode(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut bytes = s.bytes();
    while let Some(b) = bytes.next() {
        match b {
            b'%' => {
                let hi = bytes.next().and_then(hex_val);
                let lo = bytes.next().and_then(hex_val);
                if let (Some(hi), Some(lo)) = (hi, lo) {
                    result.push((hi << 4 | lo) as char);
                }
            }
            b'+' => result.push(' '),
            _ => result.push(b as char),
        }
    }
    result
}

fn hex_val(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
    }
}

fn iot_error_response(err: &IotError) -> MockResponse {
    let (status, error_type) = match err {
        IotError::ThingAlreadyExists { .. } => (409, "ResourceAlreadyExistsException"),
        IotError::ThingTypeAlreadyExists { .. } => (409, "ResourceAlreadyExistsException"),
        IotError::ThingGroupAlreadyExists { .. } => (409, "ResourceAlreadyExistsException"),
        IotError::BillingGroupAlreadyExists { .. } => (409, "ResourceAlreadyExistsException"),
        IotError::PolicyAlreadyExists { .. } => (409, "ResourceAlreadyExistsException"),
        IotError::RoleAliasAlreadyExists { .. } => (409, "ResourceAlreadyExistsException"),
        IotError::DomainConfigurationAlreadyExists { .. } => {
            (409, "ResourceAlreadyExistsException")
        }
        IotError::JobAlreadyExists { .. } => (409, "ResourceAlreadyExistsException"),
        IotError::TopicRuleAlreadyExists { .. } => (409, "ResourceAlreadyExistsException"),
        IotError::JobTemplateAlreadyExists { .. } => (409, "ConflictException"),
        IotError::ThingNotFound { .. } => (404, "ResourceNotFoundException"),
        IotError::ThingTypeNotFound { .. } => (404, "ResourceNotFoundException"),
        IotError::ThingGroupNotFound { .. } => (404, "ResourceNotFoundException"),
        IotError::BillingGroupNotFound { .. } => (404, "ResourceNotFoundException"),
        IotError::CertificateNotFound { .. } => (404, "ResourceNotFoundException"),
        IotError::CaCertificateNotFound { .. } => (404, "ResourceNotFoundException"),
        IotError::PolicyNotFound { .. } => (404, "ResourceNotFoundException"),
        IotError::PolicyVersionNotFound { .. } => (404, "ResourceNotFoundException"),
        IotError::RoleAliasNotFound { .. } => (404, "ResourceNotFoundException"),
        IotError::DomainConfigurationNotFound { .. } => (404, "ResourceNotFoundException"),
        IotError::JobNotFound { .. } => (404, "ResourceNotFoundException"),
        IotError::JobExecutionNotFound { .. } => (404, "ResourceNotFoundException"),
        IotError::JobTemplateNotFound { .. } => (404, "ResourceNotFoundException"),
        IotError::TopicRuleNotFound { .. } => (404, "ResourceNotFoundException"),
        IotError::VersionConflict => (409, "VersionConflictException"),
        IotError::VersionMismatch { .. } => (409, "VersionConflictException"),
        IotError::PolicyVersionsLimitExceeded => (409, "VersionsLimitExceededException"),
        IotError::CannotDeleteDefaultPolicyVersion => (400, "InvalidRequestException"),
    };
    let body = json!({
        "message": err.to_string(),
    });
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers
        .insert(X_AMZN_ERRORTYPE, error_type.parse().unwrap());
    resp
}

fn rest_json_error(status: u16, code: &str, message: &str) -> MockResponse {
    let body = json!({
        "message": message,
    });
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers.insert(X_AMZN_ERRORTYPE, code.parse().unwrap());
    resp
}
