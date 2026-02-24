use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde::Serialize;
use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
    StatefulService,
};

use crate::state::{S3ControlError, S3ControlState};
use crate::types::{MrapRegion, PublicAccessBlock};
use crate::views::S3ControlStateView;
use crate::wire;

const S3CONTROL_XMLNS: &str = "http://awss3control.amazonaws.com/doc/2018-08-20/";

pub struct S3ControlService {
    pub(crate) state: Arc<BackendState<S3ControlState>>,
    pub(crate) notifier: StateChangeNotifier<S3ControlStateView>,
}

impl S3ControlService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for S3ControlService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for S3ControlService {
    fn service_name(&self) -> &str {
        "s3control"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://([0-9]+)\.s3-control\.(.+)\.amazonaws\.com",
            r"https?://s3-outposts\..+\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl S3ControlService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_headers(&request.headers)
            .unwrap_or_else(|| winterbaume_core::auth::extract_region_from_uri(&request.uri));
        let account_id = request
            .headers
            .get("x-amz-account-id")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string())
            .unwrap_or_else(|| DEFAULT_ACCOUNT_ID.to_string());

        let state = self.state.get(&account_id, &region);

        let method = request.method.as_str();
        let path = extract_path(&request.uri);
        let query = parse_query_from_uri(&request.uri);
        let segments: Vec<&str> = path
            .trim_start_matches('/')
            .split('/')
            .filter(|s| !s.is_empty())
            .collect();

        let mutating = matches!(method, "PUT" | "POST" | "DELETE" | "PATCH");

        let response = match (method, segments.as_slice()) {
            // ----------------------------------------------------------------
            // Access Points
            // ----------------------------------------------------------------
            // PUT /v20180820/accesspoint/{Name} - CreateAccessPoint
            ("PUT", [_v, "accesspoint", name]) => {
                let input = match wire::deserialize_create_access_point_request(
                    &request,
                    &[("Name", name)],
                    &query,
                ) {
                    Ok(i) => i,
                    Err(e) => return s3control_error_response(400, "InvalidRequest", &e),
                };
                self.handle_create_access_point(&state, &region, &account_id, input)
                    .await
            }
            // GET /v20180820/accesspoint/{Name} - GetAccessPoint
            ("GET", [_v, "accesspoint", name]) => self.handle_get_access_point(&state, name).await,
            // DELETE /v20180820/accesspoint/{Name} - DeleteAccessPoint
            ("DELETE", [_v, "accesspoint", name]) => {
                self.handle_delete_access_point(&state, name).await
            }
            // GET /v20180820/accesspoint - ListAccessPoints
            ("GET", [_v, "accesspoint"]) => self.handle_list_access_points(&state, &query).await,
            // GET /v20180820/accesspoint/{Name}/policy - GetAccessPointPolicy
            ("GET", [_v, "accesspoint", name, "policy"]) => {
                self.handle_get_access_point_policy(&state, name).await
            }
            // PUT /v20180820/accesspoint/{Name}/policy - PutAccessPointPolicy
            ("PUT", [_v, "accesspoint", name, "policy"]) => {
                let input = match wire::deserialize_put_access_point_policy_request(
                    &request,
                    &[("Name", name)],
                    &query,
                ) {
                    Ok(i) => i,
                    Err(e) => return s3control_error_response(400, "InvalidRequest", &e),
                };
                self.handle_put_access_point_policy(&state, name, input)
                    .await
            }
            // DELETE /v20180820/accesspoint/{Name}/policy - DeleteAccessPointPolicy
            ("DELETE", [_v, "accesspoint", name, "policy"]) => {
                self.handle_delete_access_point_policy(&state, name).await
            }
            // GET /v20180820/accesspoint/{Name}/policyStatus - GetAccessPointPolicyStatus
            ("GET", [_v, "accesspoint", name, "policyStatus"]) => {
                self.handle_get_access_point_policy_status(&state, name)
                    .await
            }
            // GET /v20180820/accesspoint/{Name}/scope - GetAccessPointScope
            ("GET", [_v, "accesspoint", name, "scope"]) => {
                self.handle_get_access_point_scope(&state, name).await
            }
            // PUT /v20180820/accesspoint/{Name}/scope - PutAccessPointScope
            ("PUT", [_v, "accesspoint", name, "scope"]) => {
                self.handle_put_access_point_scope(&state, name).await
            }
            // DELETE /v20180820/accesspoint/{Name}/scope - DeleteAccessPointScope
            ("DELETE", [_v, "accesspoint", name, "scope"]) => {
                self.handle_delete_access_point_scope(&state, name).await
            }
            // GET /v20180820/accesspointfordirectory - ListAccessPointsForDirectoryBuckets
            ("GET", [_v, "accesspointfordirectory"]) => {
                self.handle_list_access_points_for_directory_buckets(&state)
                    .await
            }

            // ----------------------------------------------------------------
            // Object Lambda Access Points
            // ----------------------------------------------------------------
            // PUT /v20180820/accesspointforobjectlambda/{Name} - CreateAccessPointForObjectLambda
            ("PUT", [_v, "accesspointforobjectlambda", name]) => {
                let input = match wire::deserialize_create_access_point_for_object_lambda_request(
                    &request,
                    &[("Name", name)],
                    &query,
                ) {
                    Ok(i) => i,
                    Err(e) => return s3control_error_response(400, "InvalidRequest", &e),
                };
                self.handle_create_access_point_for_object_lambda(
                    &state,
                    &region,
                    &account_id,
                    input,
                )
                .await
            }
            // GET /v20180820/accesspointforobjectlambda/{Name} - GetAccessPointForObjectLambda
            ("GET", [_v, "accesspointforobjectlambda", name]) => {
                self.handle_get_access_point_for_object_lambda(&state, name)
                    .await
            }
            // DELETE /v20180820/accesspointforobjectlambda/{Name} - DeleteAccessPointForObjectLambda
            ("DELETE", [_v, "accesspointforobjectlambda", name]) => {
                self.handle_delete_access_point_for_object_lambda(&state, name)
                    .await
            }
            // GET /v20180820/accesspointforobjectlambda - ListAccessPointsForObjectLambda
            ("GET", [_v, "accesspointforobjectlambda"]) => {
                self.handle_list_access_points_for_object_lambda(&state)
                    .await
            }
            // GET /v20180820/accesspointforobjectlambda/{Name}/policy
            ("GET", [_v, "accesspointforobjectlambda", name, "policy"]) => {
                self.handle_get_access_point_policy_for_object_lambda(&state, name)
                    .await
            }
            // PUT /v20180820/accesspointforobjectlambda/{Name}/policy
            ("PUT", [_v, "accesspointforobjectlambda", name, "policy"]) => {
                let input =
                    match wire::deserialize_put_access_point_policy_for_object_lambda_request(
                        &request,
                        &[("Name", name)],
                        &query,
                    ) {
                        Ok(i) => i,
                        Err(e) => return s3control_error_response(400, "InvalidRequest", &e),
                    };
                self.handle_put_access_point_policy_for_object_lambda(&state, name, input)
                    .await
            }
            // DELETE /v20180820/accesspointforobjectlambda/{Name}/policy
            ("DELETE", [_v, "accesspointforobjectlambda", name, "policy"]) => {
                self.handle_delete_access_point_policy_for_object_lambda(&state, name)
                    .await
            }
            // GET /v20180820/accesspointforobjectlambda/{Name}/policyStatus
            ("GET", [_v, "accesspointforobjectlambda", name, "policyStatus"]) => {
                self.handle_get_access_point_policy_status_for_object_lambda(&state, name)
                    .await
            }
            // GET /v20180820/accesspointforobjectlambda/{Name}/configuration
            ("GET", [_v, "accesspointforobjectlambda", name, "configuration"]) => {
                self.handle_get_access_point_configuration_for_object_lambda(&state, name)
                    .await
            }
            // PUT /v20180820/accesspointforobjectlambda/{Name}/configuration
            ("PUT", [_v, "accesspointforobjectlambda", name, "configuration"]) => {
                let input =
                    match wire::deserialize_put_access_point_configuration_for_object_lambda_request(
                        &request,
                        &[("Name", name)],
                        &query,
                    ) {
                        Ok(i) => i,
                        Err(e) => return s3control_error_response(400, "InvalidRequest", &e),
                    };
                self.handle_put_access_point_configuration_for_object_lambda(&state, name, input)
                    .await
            }

            // ----------------------------------------------------------------
            // Public Access Block
            // ----------------------------------------------------------------
            ("GET", [_v, "configuration", "publicAccessBlock"]) => {
                self.handle_get_public_access_block(&state).await
            }
            ("PUT", [_v, "configuration", "publicAccessBlock"]) => {
                let input = match wire::deserialize_put_public_access_block_request(
                    &request,
                    &[],
                    &query,
                ) {
                    Ok(i) => i,
                    Err(e) => return s3control_error_response(400, "InvalidRequest", &e),
                };
                self.handle_put_public_access_block(&state, input).await
            }
            ("DELETE", [_v, "configuration", "publicAccessBlock"]) => {
                self.handle_delete_public_access_block(&state).await
            }

            // ----------------------------------------------------------------
            // Access Grants
            // ----------------------------------------------------------------
            // POST /v20180820/accessgrantsinstance - CreateAccessGrantsInstance
            ("POST", [_v, "accessgrantsinstance"]) => {
                let input = match wire::deserialize_create_access_grants_instance_request(
                    &request,
                    &[],
                    &query,
                ) {
                    Ok(i) => i,
                    Err(e) => return s3control_error_response(400, "InvalidRequest", &e),
                };
                self.handle_create_access_grants_instance(&state, &region, &account_id, input)
                    .await
            }
            // GET /v20180820/accessgrantsinstance - GetAccessGrantsInstance
            ("GET", [_v, "accessgrantsinstance"]) => {
                self.handle_get_access_grants_instance(&state).await
            }
            // DELETE /v20180820/accessgrantsinstance - DeleteAccessGrantsInstance
            ("DELETE", [_v, "accessgrantsinstance"]) => {
                self.handle_delete_access_grants_instance(&state).await
            }
            // GET /v20180820/accessgrantsinstances - ListAccessGrantsInstances
            ("GET", [_v, "accessgrantsinstances"]) => {
                self.handle_list_access_grants_instances(&state).await
            }
            // POST /v20180820/accessgrantsinstance/identitycenter - AssociateAccessGrantsIdentityCenter
            ("POST", [_v, "accessgrantsinstance", "identitycenter"]) => {
                self.handle_associate_access_grants_identity_center().await
            }
            // DELETE /v20180820/accessgrantsinstance/identitycenter - DissociateAccessGrantsIdentityCenter
            ("DELETE", [_v, "accessgrantsinstance", "identitycenter"]) => {
                self.handle_dissociate_access_grants_identity_center().await
            }
            // GET /v20180820/accessgrantsinstance/resourcepolicy
            ("GET", [_v, "accessgrantsinstance", "resourcepolicy"]) => {
                self.handle_get_access_grants_instance_resource_policy(&state)
                    .await
            }
            // PUT /v20180820/accessgrantsinstance/resourcepolicy
            ("PUT", [_v, "accessgrantsinstance", "resourcepolicy"]) => {
                let input =
                    match wire::deserialize_put_access_grants_instance_resource_policy_request(
                        &request,
                        &[],
                        &query,
                    ) {
                        Ok(i) => i,
                        Err(e) => return s3control_error_response(400, "InvalidRequest", &e),
                    };
                self.handle_put_access_grants_instance_resource_policy(&state, input)
                    .await
            }
            // DELETE /v20180820/accessgrantsinstance/resourcepolicy
            ("DELETE", [_v, "accessgrantsinstance", "resourcepolicy"]) => {
                self.handle_delete_access_grants_instance_resource_policy(&state)
                    .await
            }
            // GET /v20180820/accessgrantsinstance/prefix
            ("GET", [_v, "accessgrantsinstance", "prefix"]) => {
                self.handle_get_access_grants_instance_for_prefix().await
            }
            // POST /v20180820/accessgrantsinstance/grant - CreateAccessGrant
            ("POST", [_v, "accessgrantsinstance", "grant"]) => {
                let input =
                    match wire::deserialize_create_access_grant_request(&request, &[], &query) {
                        Ok(i) => i,
                        Err(e) => return s3control_error_response(400, "InvalidRequest", &e),
                    };
                self.handle_create_access_grant(&state, &region, &account_id, input)
                    .await
            }
            // GET /v20180820/accessgrantsinstance/grant/{AccessGrantId}
            ("GET", [_v, "accessgrantsinstance", "grant", grant_id]) => {
                self.handle_get_access_grant(&state, grant_id).await
            }
            // DELETE /v20180820/accessgrantsinstance/grant/{AccessGrantId}
            ("DELETE", [_v, "accessgrantsinstance", "grant", grant_id]) => {
                self.handle_delete_access_grant(&state, grant_id).await
            }
            // GET /v20180820/accessgrantsinstance/grants - ListAccessGrants
            ("GET", [_v, "accessgrantsinstance", "grants"]) => {
                self.handle_list_access_grants(&state).await
            }
            // GET /v20180820/accessgrantsinstance/caller/grants - ListCallerAccessGrants
            ("GET", [_v, "accessgrantsinstance", "caller", "grants"]) => {
                self.handle_list_caller_access_grants().await
            }
            // GET /v20180820/accessgrantsinstance/dataaccess - GetDataAccess
            ("GET", [_v, "accessgrantsinstance", "dataaccess"]) => {
                self.handle_get_data_access().await
            }
            // POST /v20180820/accessgrantsinstance/location - CreateAccessGrantsLocation
            ("POST", [_v, "accessgrantsinstance", "location"]) => {
                let input = match wire::deserialize_create_access_grants_location_request(
                    &request,
                    &[],
                    &query,
                ) {
                    Ok(i) => i,
                    Err(e) => return s3control_error_response(400, "InvalidRequest", &e),
                };
                self.handle_create_access_grants_location(&state, &region, &account_id, input)
                    .await
            }
            // GET /v20180820/accessgrantsinstance/location/{AccessGrantsLocationId}
            ("GET", [_v, "accessgrantsinstance", "location", loc_id]) => {
                self.handle_get_access_grants_location(&state, loc_id).await
            }
            // DELETE /v20180820/accessgrantsinstance/location/{AccessGrantsLocationId}
            ("DELETE", [_v, "accessgrantsinstance", "location", loc_id]) => {
                self.handle_delete_access_grants_location(&state, loc_id)
                    .await
            }
            // PUT /v20180820/accessgrantsinstance/location/{AccessGrantsLocationId}
            ("PUT", [_v, "accessgrantsinstance", "location", loc_id]) => {
                let input = match wire::deserialize_update_access_grants_location_request(
                    &request,
                    &[("AccessGrantsLocationId", loc_id)],
                    &query,
                ) {
                    Ok(i) => i,
                    Err(e) => return s3control_error_response(400, "InvalidRequest", &e),
                };
                self.handle_update_access_grants_location(&state, input)
                    .await
            }
            // GET /v20180820/accessgrantsinstance/locations - ListAccessGrantsLocations
            ("GET", [_v, "accessgrantsinstance", "locations"]) => {
                self.handle_list_access_grants_locations(&state).await
            }

            // ----------------------------------------------------------------
            // Multi-Region Access Points
            // ----------------------------------------------------------------
            // POST /v20180820/async-requests/mrap/create
            ("POST", [_v, "async-requests", "mrap", "create"]) => {
                let input = match wire::deserialize_create_multi_region_access_point_request(
                    &request,
                    &[],
                    &query,
                ) {
                    Ok(i) => i,
                    Err(e) => return s3control_error_response(400, "InvalidRequest", &e),
                };
                self.handle_create_multi_region_access_point(&state, &account_id, input)
                    .await
            }
            // POST /v20180820/async-requests/mrap/delete
            ("POST", [_v, "async-requests", "mrap", "delete"]) => {
                let input = match wire::deserialize_delete_multi_region_access_point_request(
                    &request,
                    &[],
                    &query,
                ) {
                    Ok(i) => i,
                    Err(e) => return s3control_error_response(400, "InvalidRequest", &e),
                };
                self.handle_delete_multi_region_access_point(&state, &account_id, input)
                    .await
            }
            // GET /v20180820/mrap/instances - ListMultiRegionAccessPoints
            ("GET", [_v, "mrap", "instances"]) => {
                self.handle_list_multi_region_access_points(&state).await
            }
            // GET /v20180820/mrap/instances/{Name+} - GetMultiRegionAccessPoint
            ("GET", [_v, "mrap", "instances", rest @ ..]) if !rest.is_empty() => {
                let full = percent_decode_path(&rest.join("/"));
                // strip trailing /policy, /policystatus, /routes etc
                if full.ends_with("/policy") {
                    let name = full.trim_end_matches("/policy");
                    self.handle_get_multi_region_access_point_policy(&state, name)
                        .await
                } else if full.ends_with("/policystatus") {
                    let name = full.trim_end_matches("/policystatus");
                    self.handle_get_multi_region_access_point_policy_status(&state, name)
                        .await
                } else {
                    self.handle_get_multi_region_access_point(&state, &full)
                        .await
                }
            }
            // GET /v20180820/mrap/instances/{Mrap+}/routes
            ("GET", [_v, "mrap", "instances", rest @ ..]) if rest.last() == Some(&"routes") => {
                let mrap_name = percent_decode_path(&rest[..rest.len() - 1].join("/"));
                self.handle_get_multi_region_access_point_routes(&state, &mrap_name)
                    .await
            }
            // PATCH /v20180820/mrap/instances/{Mrap+}/routes
            ("PATCH", [_v, "mrap", "instances", rest @ ..]) if rest.last() == Some(&"routes") => {
                let mrap_name = percent_decode_path(&rest[..rest.len() - 1].join("/"));
                let input = match wire::deserialize_submit_multi_region_access_point_routes_request(
                    &request,
                    &[("Mrap", &mrap_name)],
                    &query,
                ) {
                    Ok(i) => i,
                    Err(e) => return s3control_error_response(400, "InvalidRequest", &e),
                };
                self.handle_submit_multi_region_access_point_routes(&state, &mrap_name, input)
                    .await
            }
            // POST /v20180820/async-requests/mrap/put-policy
            ("POST", [_v, "async-requests", "mrap", "put-policy"]) => {
                let input = match wire::deserialize_put_multi_region_access_point_policy_request(
                    &request,
                    &[],
                    &query,
                ) {
                    Ok(i) => i,
                    Err(e) => return s3control_error_response(400, "InvalidRequest", &e),
                };
                self.handle_put_multi_region_access_point_policy(&state, input)
                    .await
            }
            // GET /v20180820/async-requests/mrap/{RequestTokenARN+}
            ("GET", [_v, "async-requests", "mrap", rest @ ..]) => {
                let token_arn = percent_decode_path(&rest.join("/"));
                self.handle_describe_multi_region_access_point_operation(&token_arn)
                    .await
            }

            // ----------------------------------------------------------------
            // Jobs
            // ----------------------------------------------------------------
            ("POST", [_v, "jobs"]) => {
                let input = match wire::deserialize_create_job_request(&request, &[], &query) {
                    Ok(i) => i,
                    Err(e) => return s3control_error_response(400, "InvalidRequest", &e),
                };
                self.handle_create_job(&state, &region, &account_id, input)
                    .await
            }
            ("GET", [_v, "jobs"]) => self.handle_list_jobs(&state, &query).await,
            ("GET", [_v, "jobs", job_id]) => self.handle_describe_job(&state, job_id).await,
            ("GET", [_v, "jobs", job_id, "tagging"]) => {
                self.handle_get_job_tagging(&state, job_id).await
            }
            ("PUT", [_v, "jobs", job_id, "tagging"]) => {
                let input = match wire::deserialize_put_job_tagging_request(
                    &request,
                    &[("JobId", job_id)],
                    &query,
                ) {
                    Ok(i) => i,
                    Err(e) => return s3control_error_response(400, "InvalidRequest", &e),
                };
                self.handle_put_job_tagging(&state, job_id, input).await
            }
            ("DELETE", [_v, "jobs", job_id, "tagging"]) => {
                self.handle_delete_job_tagging(&state, job_id).await
            }
            ("POST", [_v, "jobs", job_id, "priority"]) => {
                let input = match wire::deserialize_update_job_priority_request(
                    &request,
                    &[("JobId", job_id)],
                    &query,
                ) {
                    Ok(i) => i,
                    Err(e) => return s3control_error_response(400, "InvalidRequest", &e),
                };
                self.handle_update_job_priority(&state, input).await
            }
            ("POST", [_v, "jobs", job_id, "status"]) => {
                let input = match wire::deserialize_update_job_status_request(
                    &request,
                    &[("JobId", job_id)],
                    &query,
                ) {
                    Ok(i) => i,
                    Err(e) => return s3control_error_response(400, "InvalidRequest", &e),
                };
                self.handle_update_job_status(&state, input).await
            }

            // ----------------------------------------------------------------
            // Storage Lens Groups
            // ----------------------------------------------------------------
            ("POST", [_v, "storagelensgroup"]) => {
                let input = match wire::deserialize_create_storage_lens_group_request(
                    &request,
                    &[],
                    &query,
                ) {
                    Ok(i) => i,
                    Err(e) => return s3control_error_response(400, "InvalidRequest", &e),
                };
                self.handle_create_storage_lens_group(&state, &region, &account_id, input)
                    .await
            }
            ("GET", [_v, "storagelensgroup"]) => {
                self.handle_list_storage_lens_groups(&state, &region, &account_id)
                    .await
            }
            ("GET", [_v, "storagelensgroup", name]) => {
                self.handle_get_storage_lens_group(&state, &region, &account_id, name)
                    .await
            }
            ("DELETE", [_v, "storagelensgroup", name]) => {
                self.handle_delete_storage_lens_group(&state, name).await
            }
            ("PUT", [_v, "storagelensgroup", name]) => {
                let input = match wire::deserialize_update_storage_lens_group_request(
                    &request,
                    &[("Name", name)],
                    &query,
                ) {
                    Ok(i) => i,
                    Err(e) => return s3control_error_response(400, "InvalidRequest", &e),
                };
                self.handle_update_storage_lens_group(&state, input).await
            }

            // ----------------------------------------------------------------
            // Storage Lens Configurations
            // ----------------------------------------------------------------
            ("PUT", [_v, "storagelens", config_id]) => {
                self.handle_put_storage_lens_configuration(&state, config_id, &region, &account_id)
                    .await
            }
            ("GET", [_v, "storagelens", config_id]) => {
                self.handle_get_storage_lens_configuration(&state, config_id)
                    .await
            }
            ("DELETE", [_v, "storagelens", config_id]) => {
                self.handle_delete_storage_lens_configuration(&state, config_id)
                    .await
            }
            ("GET", [_v, "storagelens", config_id, "tagging"]) => {
                self.handle_get_storage_lens_configuration_tagging(&state, config_id)
                    .await
            }
            ("PUT", [_v, "storagelens", config_id, "tagging"]) => {
                let input = match wire::deserialize_put_storage_lens_configuration_tagging_request(
                    &request,
                    &[("ConfigId", config_id)],
                    &query,
                ) {
                    Ok(i) => i,
                    Err(e) => return s3control_error_response(400, "InvalidRequest", &e),
                };
                self.handle_put_storage_lens_configuration_tagging(&state, config_id, input)
                    .await
            }
            ("DELETE", [_v, "storagelens", config_id, "tagging"]) => {
                self.handle_delete_storage_lens_configuration_tagging(&state, config_id)
                    .await
            }
            ("GET", [_v, "storagelens"]) => {
                self.handle_list_storage_lens_configurations(&state).await
            }

            // ----------------------------------------------------------------
            // Buckets (Outposts)
            // ----------------------------------------------------------------
            ("PUT", [_v, "bucket", bucket]) => {
                let input = match wire::deserialize_create_bucket_request(
                    &request,
                    &[("Bucket", bucket)],
                    &query,
                ) {
                    Ok(i) => i,
                    Err(e) => return s3control_error_response(400, "InvalidRequest", &e),
                };
                self.handle_create_bucket(&state, &account_id, input).await
            }
            ("GET", [_v, "bucket", bucket]) => self.handle_get_bucket(&state, bucket).await,
            ("DELETE", [_v, "bucket", bucket]) => self.handle_delete_bucket(&state, bucket).await,
            ("GET", [_v, "bucket"]) => {
                self.handle_list_regional_buckets(&state, &region, &account_id)
                    .await
            }
            ("GET", [_v, "bucket", bucket, "policy"]) => {
                self.handle_get_bucket_policy(&state, bucket).await
            }
            ("PUT", [_v, "bucket", bucket, "policy"]) => {
                let input = match wire::deserialize_put_bucket_policy_request(
                    &request,
                    &[("Bucket", bucket)],
                    &query,
                ) {
                    Ok(i) => i,
                    Err(e) => return s3control_error_response(400, "InvalidRequest", &e),
                };
                self.handle_put_bucket_policy(&state, bucket, input).await
            }
            ("DELETE", [_v, "bucket", bucket, "policy"]) => {
                self.handle_delete_bucket_policy(&state, bucket).await
            }
            ("GET", [_v, "bucket", bucket, "tagging"]) => {
                self.handle_get_bucket_tagging(&state, bucket).await
            }
            ("PUT", [_v, "bucket", bucket, "tagging"]) => {
                let input = match wire::deserialize_put_bucket_tagging_request(
                    &request,
                    &[("Bucket", bucket)],
                    &query,
                ) {
                    Ok(i) => i,
                    Err(e) => return s3control_error_response(400, "InvalidRequest", &e),
                };
                self.handle_put_bucket_tagging(&state, bucket, input).await
            }
            ("DELETE", [_v, "bucket", bucket, "tagging"]) => {
                self.handle_delete_bucket_tagging(&state, bucket).await
            }
            // Bucket lifecycle, replication, versioning
            ("GET", [_v, "bucket", bucket, "lifecycleconfiguration"]) => {
                self.handle_get_bucket_lifecycle_configuration(&state, bucket)
                    .await
            }
            ("PUT", [_v, "bucket", bucket, "lifecycleconfiguration"]) => {
                self.handle_put_bucket_lifecycle_configuration(&state, bucket)
                    .await
            }
            ("DELETE", [_v, "bucket", bucket, "lifecycleconfiguration"]) => {
                self.handle_delete_bucket_lifecycle_configuration(&state, bucket)
                    .await
            }
            ("GET", [_v, "bucket", bucket, "replication"]) => {
                self.handle_get_bucket_replication(&state, bucket).await
            }
            ("PUT", [_v, "bucket", bucket, "replication"]) => {
                self.handle_put_bucket_replication(&state, bucket).await
            }
            ("DELETE", [_v, "bucket", bucket, "replication"]) => {
                self.handle_delete_bucket_replication(&state, bucket).await
            }
            ("GET", [_v, "bucket", bucket, "versioning"]) => {
                self.handle_get_bucket_versioning(&state, bucket).await
            }
            ("PUT", [_v, "bucket", bucket, "versioning"]) => {
                self.handle_put_bucket_versioning(&state, bucket).await
            }

            // ----------------------------------------------------------------
            // Resource Tags
            // ----------------------------------------------------------------
            // GET /v20180820/tags/{ResourceArn+}
            ("GET", [_v, "tags", rest @ ..]) if !rest.is_empty() => {
                let arn = percent_decode_path(&rest.join("/"));
                self.handle_list_tags_for_resource(&state, &arn).await
            }
            // POST /v20180820/tags/{ResourceArn+}
            ("POST", [_v, "tags", rest @ ..]) if !rest.is_empty() => {
                let arn = percent_decode_path(&rest.join("/"));
                let input = match wire::deserialize_tag_resource_request(
                    &request,
                    &[("ResourceArn", &arn)],
                    &query,
                ) {
                    Ok(i) => i,
                    Err(e) => return s3control_error_response(400, "InvalidRequest", &e),
                };
                self.handle_tag_resource(&state, &arn, input).await
            }
            // DELETE /v20180820/tags/{ResourceArn+}
            ("DELETE", [_v, "tags", rest @ ..]) if !rest.is_empty() => {
                let arn = percent_decode_path(&rest.join("/"));
                let input = match wire::deserialize_untag_resource_request(
                    &request,
                    &[("ResourceArn", &arn)],
                    &query,
                ) {
                    Ok(i) => i,
                    Err(e) => return s3control_error_response(400, "InvalidRequest", &e),
                };
                self.handle_untag_resource(&state, &arn, input).await
            }

            _ => s3control_error_response(
                400,
                "UnsupportedOperation",
                &format!("Unsupported operation: {method} {path}"),
            ),
        };

        if mutating && response.status / 100 == 2 {
            self.notify_state_changed(&account_id, &region).await;
        }

        response
    }

    // -------------------------------------------------------------------------
    // Access Point handlers
    // -------------------------------------------------------------------------

    async fn handle_create_access_point(
        &self,
        state: &Arc<tokio::sync::RwLock<S3ControlState>>,
        region: &str,
        account_id: &str,
        input: crate::model::CreateAccessPointRequest,
    ) -> MockResponse {
        if input.name.is_empty() {
            return s3control_error_response(400, "InvalidRequest", "Name is required");
        }
        if input.bucket.is_empty() {
            return s3control_error_response(400, "InvalidRequest", "Bucket is required");
        }

        let vpc_id = input.vpc_configuration.as_ref().map(|v| v.vpc_id.clone());
        let pab = input.public_access_block_configuration.as_ref();
        let creation_date = chrono::Utc::now().to_rfc3339();

        let mut s = state.write().await;
        match s.create_access_point(
            input.name,
            input.bucket,
            account_id.to_string(),
            region.to_string(),
            vpc_id,
            pab.and_then(|p| p.block_public_acls).unwrap_or(true),
            pab.and_then(|p| p.ignore_public_acls).unwrap_or(true),
            pab.and_then(|p| p.block_public_policy).unwrap_or(true),
            pab.and_then(|p| p.restrict_public_buckets).unwrap_or(true),
            creation_date,
        ) {
            Ok(ap) => {
                wire::serialize_create_access_point_response(&wire::CreateAccessPointResult {
                    access_point_arn: Some(ap.arn),
                    alias: Some(ap.alias),
                })
            }
            Err(e) => s3control_state_error_response(&e),
        }
    }

    async fn handle_get_access_point(
        &self,
        state: &Arc<tokio::sync::RwLock<S3ControlState>>,
        name: &str,
    ) -> MockResponse {
        let s = state.read().await;
        match s.get_access_point(name) {
            Ok(ap) => {
                let vpc = ap
                    .vpc_id
                    .as_ref()
                    .map(|id| crate::model::VpcConfiguration { vpc_id: id.clone() });
                wire::serialize_get_access_point_response(&wire::GetAccessPointResult {
                    name: Some(ap.name.clone()),
                    bucket: Some(ap.bucket.clone()),
                    access_point_arn: Some(ap.arn.clone()),
                    alias: Some(ap.alias.clone()),
                    network_origin: Some(ap.network_origin.clone()),
                    creation_date: Some(ap.creation_date.clone()),
                    vpc_configuration: vpc,
                    public_access_block_configuration: Some(wire::PublicAccessBlockConfiguration {
                        block_public_acls: Some(ap.block_public_acls),
                        ignore_public_acls: Some(ap.ignore_public_acls),
                        block_public_policy: Some(ap.block_public_policy),
                        restrict_public_buckets: Some(ap.restrict_public_buckets),
                    }),
                    ..Default::default()
                })
            }
            Err(e) => s3control_state_error_response(&e),
        }
    }

    async fn handle_delete_access_point(
        &self,
        state: &Arc<tokio::sync::RwLock<S3ControlState>>,
        name: &str,
    ) -> MockResponse {
        let mut s = state.write().await;
        match s.delete_access_point(name) {
            Ok(()) => wire::serialize_delete_access_point_response(),
            Err(e) => s3control_state_error_response(&e),
        }
    }

    async fn handle_list_access_points(
        &self,
        state: &Arc<tokio::sync::RwLock<S3ControlState>>,
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let bucket = query.get("bucket").map(String::as_str);
        let s = state.read().await;
        let mut aps: Vec<wire::AccessPoint> = s
            .list_access_points(bucket)
            .into_iter()
            .map(|ap| {
                let vpc = ap
                    .vpc_id
                    .as_ref()
                    .map(|id| wire::VpcConfiguration { vpc_id: id.clone() });
                wire::AccessPoint {
                    name: Some(ap.name.clone()),
                    bucket: Some(ap.bucket.clone()),
                    access_point_arn: Some(ap.arn.clone()),
                    alias: Some(ap.alias.clone()),
                    network_origin: Some(ap.network_origin.clone()),
                    vpc_configuration: vpc,
                    ..Default::default()
                }
            })
            .collect();
        aps.sort_by(|a, b| a.name.cmp(&b.name));
        wire::serialize_list_access_points_response(&wire::ListAccessPointsResult {
            access_point_list: Some(aps.into()),
            next_token: None,
        })
    }

    async fn handle_get_access_point_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<S3ControlState>>,
        name: &str,
    ) -> MockResponse {
        let s = state.read().await;
        match s.get_access_point_policy(name) {
            Ok(policy) => wire::serialize_get_access_point_policy_response(
                &wire::GetAccessPointPolicyResult {
                    policy: policy.map(String::from),
                },
            ),
            Err(e) => s3control_state_error_response(&e),
        }
    }

    async fn handle_put_access_point_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<S3ControlState>>,
        name: &str,
        input: crate::model::PutAccessPointPolicyRequest,
    ) -> MockResponse {
        let mut s = state.write().await;
        match s.put_access_point_policy(name, input.policy) {
            Ok(()) => wire::serialize_put_access_point_policy_response(),
            Err(e) => s3control_state_error_response(&e),
        }
    }

    async fn handle_delete_access_point_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<S3ControlState>>,
        name: &str,
    ) -> MockResponse {
        let mut s = state.write().await;
        match s.delete_access_point_policy(name) {
            Ok(()) => wire::serialize_delete_access_point_policy_response(),
            Err(e) => s3control_state_error_response(&e),
        }
    }

    async fn handle_get_access_point_policy_status(
        &self,
        state: &Arc<tokio::sync::RwLock<S3ControlState>>,
        name: &str,
    ) -> MockResponse {
        let s = state.read().await;
        match s.get_access_point(name) {
            Ok(ap) => {
                let is_public = ap.policy.is_some();
                wire::serialize_get_access_point_policy_status_response(
                    &wire::GetAccessPointPolicyStatusResult {
                        policy_status: Some(wire::PolicyStatus {
                            is_public: Some(is_public),
                        }),
                    },
                )
            }
            Err(e) => s3control_state_error_response(&e),
        }
    }

    // STUB[no-engine]: GetAccessPointScope response serialisation for the nested Scope shape is not yet implemented; always returns empty scope.
    async fn handle_get_access_point_scope(
        &self,
        state: &Arc<tokio::sync::RwLock<S3ControlState>>,
        name: &str,
    ) -> MockResponse {
        let s = state.read().await;
        match s.get_access_point_scope(name) {
            Ok(_scope) => {
                wire::serialize_get_access_point_scope_response(&wire::GetAccessPointScopeResult {
                    ..Default::default()
                })
            }
            Err(e) => s3control_state_error_response(&e),
        }
    }

    async fn handle_put_access_point_scope(
        &self,
        state: &Arc<tokio::sync::RwLock<S3ControlState>>,
        name: &str,
    ) -> MockResponse {
        let scope = crate::types::AccessPointScope::default();
        let mut s = state.write().await;
        match s.put_access_point_scope(name, scope) {
            Ok(()) => wire::serialize_put_access_point_scope_response(),
            Err(e) => s3control_state_error_response(&e),
        }
    }

    async fn handle_delete_access_point_scope(
        &self,
        state: &Arc<tokio::sync::RwLock<S3ControlState>>,
        name: &str,
    ) -> MockResponse {
        let mut s = state.write().await;
        match s.delete_access_point_scope(name) {
            Ok(()) => wire::serialize_delete_access_point_scope_response(),
            Err(e) => s3control_state_error_response(&e),
        }
    }

    async fn handle_list_access_points_for_directory_buckets(
        &self,
        state: &Arc<tokio::sync::RwLock<S3ControlState>>,
    ) -> MockResponse {
        let s = state.read().await;
        let aps: Vec<wire::AccessPoint> = s
            .list_directory_bucket_access_points()
            .into_iter()
            .map(|ap| wire::AccessPoint {
                name: Some(ap.name.clone()),
                bucket: Some(ap.bucket.clone()),
                access_point_arn: Some(ap.arn.clone()),
                alias: Some(ap.alias.clone()),
                ..Default::default()
            })
            .collect();
        wire::serialize_list_access_points_for_directory_buckets_response(
            &wire::ListAccessPointsForDirectoryBucketsResult {
                access_point_list: if aps.is_empty() {
                    None
                } else {
                    Some(aps.into())
                },
                ..Default::default()
            },
        )
    }

    // -------------------------------------------------------------------------
    // Object Lambda Access Point handlers
    // -------------------------------------------------------------------------

    async fn handle_create_access_point_for_object_lambda(
        &self,
        state: &Arc<tokio::sync::RwLock<S3ControlState>>,
        region: &str,
        account_id: &str,
        input: crate::model::CreateAccessPointForObjectLambdaRequest,
    ) -> MockResponse {
        if input.name.is_empty() {
            return s3control_error_response(400, "InvalidRequest", "Name is required");
        }
        let creation_date = chrono::Utc::now().to_rfc3339();
        let access_point_arn = input.configuration.supporting_access_point.clone();
        let config_json = serde_json::to_string(&input.configuration).ok();
        let mut s = state.write().await;
        match s.create_object_lambda_access_point(
            account_id,
            region,
            input.name,
            access_point_arn,
            creation_date,
            config_json,
        ) {
            Ok(ap) => wire::serialize_create_access_point_for_object_lambda_response(
                &wire::CreateAccessPointForObjectLambdaResult {
                    object_lambda_access_point_arn: Some(ap.arn.clone()),
                    alias: Some(wire::ObjectLambdaAccessPointAlias {
                        value: Some(ap.alias.clone()),
                        status: Some(ap.alias_status.clone()),
                    }),
                },
            ),
            Err(e) => s3control_state_error_response(&e),
        }
    }

    async fn handle_get_access_point_for_object_lambda(
        &self,
        state: &Arc<tokio::sync::RwLock<S3ControlState>>,
        name: &str,
    ) -> MockResponse {
        let s = state.read().await;
        match s.get_object_lambda_access_point(name) {
            Ok(ap) => wire::serialize_get_access_point_for_object_lambda_response(
                &wire::GetAccessPointForObjectLambdaResult {
                    name: Some(ap.name.clone()),
                    creation_date: Some(ap.creation_date.clone()),
                    alias: Some(wire::ObjectLambdaAccessPointAlias {
                        value: Some(ap.alias.clone()),
                        status: Some(ap.alias_status.clone()),
                    }),
                    ..Default::default()
                },
            ),
            Err(e) => s3control_state_error_response(&e),
        }
    }

    async fn handle_delete_access_point_for_object_lambda(
        &self,
        state: &Arc<tokio::sync::RwLock<S3ControlState>>,
        name: &str,
    ) -> MockResponse {
        let mut s = state.write().await;
        match s.delete_object_lambda_access_point(name) {
            Ok(()) => wire::serialize_delete_access_point_for_object_lambda_response(),
            Err(e) => s3control_state_error_response(&e),
        }
    }

    async fn handle_list_access_points_for_object_lambda(
        &self,
        state: &Arc<tokio::sync::RwLock<S3ControlState>>,
    ) -> MockResponse {
        let s = state.read().await;
        let mut aps: Vec<wire::ObjectLambdaAccessPoint> = s
            .list_object_lambda_access_points()
            .into_iter()
            .map(|ap| wire::ObjectLambdaAccessPoint {
                name: Some(ap.name.clone()),
                object_lambda_access_point_arn: Some(ap.arn.clone()),
                alias: Some(wire::ObjectLambdaAccessPointAlias {
                    value: Some(ap.alias.clone()),
                    status: Some(ap.alias_status.clone()),
                }),
            })
            .collect();
        aps.sort_by(|a, b| a.name.cmp(&b.name));
        wire::serialize_list_access_points_for_object_lambda_response(
            &wire::ListAccessPointsForObjectLambdaResult {
                object_lambda_access_point_list: Some(aps.into()),
                next_token: None,
            },
        )
    }

    async fn handle_get_access_point_policy_for_object_lambda(
        &self,
        state: &Arc<tokio::sync::RwLock<S3ControlState>>,
        name: &str,
    ) -> MockResponse {
        let s = state.read().await;
        match s.get_object_lambda_access_point_policy(name) {
            Ok(policy) => wire::serialize_get_access_point_policy_for_object_lambda_response(
                &wire::GetAccessPointPolicyForObjectLambdaResult {
                    policy: policy.map(String::from),
                },
            ),
            Err(e) => s3control_state_error_response(&e),
        }
    }

    async fn handle_put_access_point_policy_for_object_lambda(
        &self,
        state: &Arc<tokio::sync::RwLock<S3ControlState>>,
        name: &str,
        input: crate::model::PutAccessPointPolicyForObjectLambdaRequest,
    ) -> MockResponse {
        let mut s = state.write().await;
        match s.put_object_lambda_access_point_policy(name, input.policy) {
            Ok(()) => wire::serialize_put_access_point_policy_for_object_lambda_response(),
            Err(e) => s3control_state_error_response(&e),
        }
    }

    async fn handle_delete_access_point_policy_for_object_lambda(
        &self,
        state: &Arc<tokio::sync::RwLock<S3ControlState>>,
        name: &str,
    ) -> MockResponse {
        let mut s = state.write().await;
        match s.delete_object_lambda_access_point_policy(name) {
            Ok(()) => wire::serialize_delete_access_point_policy_for_object_lambda_response(),
            Err(e) => s3control_state_error_response(&e),
        }
    }

    async fn handle_get_access_point_policy_status_for_object_lambda(
        &self,
        state: &Arc<tokio::sync::RwLock<S3ControlState>>,
        name: &str,
    ) -> MockResponse {
        let s = state.read().await;
        match s.get_object_lambda_access_point(name) {
            Ok(ap) => {
                let is_public = ap.policy.is_some();
                wire::serialize_get_access_point_policy_status_for_object_lambda_response(
                    &wire::GetAccessPointPolicyStatusForObjectLambdaResult {
                        policy_status: Some(wire::PolicyStatus {
                            is_public: Some(is_public),
                        }),
                    },
                )
            }
            Err(e) => s3control_state_error_response(&e),
        }
    }

    // STUB[no-engine]: GetAccessPointConfigurationForObjectLambda response serialisation for the nested ObjectLambdaConfiguration shape is not yet implemented; always returns empty configuration.
    async fn handle_get_access_point_configuration_for_object_lambda(
        &self,
        state: &Arc<tokio::sync::RwLock<S3ControlState>>,
        name: &str,
    ) -> MockResponse {
        let s = state.read().await;
        match s.get_object_lambda_access_point(name) {
            Ok(_) => wire::serialize_get_access_point_configuration_for_object_lambda_response(
                &wire::GetAccessPointConfigurationForObjectLambdaResult {
                    ..Default::default()
                },
            ),
            Err(e) => s3control_state_error_response(&e),
        }
    }

    async fn handle_put_access_point_configuration_for_object_lambda(
        &self,
        state: &Arc<tokio::sync::RwLock<S3ControlState>>,
        name: &str,
        input: crate::model::PutAccessPointConfigurationForObjectLambdaRequest,
    ) -> MockResponse {
        let config_json = serde_json::to_string(&input.configuration).ok();
        let mut s = state.write().await;
        match s.get_object_lambda_access_point(name) {
            Ok(_) => {
                if let Some(ap) = s.object_lambda_access_points.get_mut(name) {
                    ap.configuration_json = config_json;
                }
                wire::serialize_put_access_point_configuration_for_object_lambda_response()
            }
            Err(e) => s3control_state_error_response(&e),
        }
    }

    // -------------------------------------------------------------------------
    // Public Access Block handlers
    // -------------------------------------------------------------------------

    async fn handle_get_public_access_block(
        &self,
        state: &Arc<tokio::sync::RwLock<S3ControlState>>,
    ) -> MockResponse {
        let s = state.read().await;
        match s.get_public_access_block() {
            Ok(pab) => wire::serialize_get_public_access_block_response(
                &wire::PublicAccessBlockConfiguration {
                    block_public_acls: Some(pab.block_public_acls),
                    ignore_public_acls: Some(pab.ignore_public_acls),
                    block_public_policy: Some(pab.block_public_policy),
                    restrict_public_buckets: Some(pab.restrict_public_buckets),
                },
            ),
            Err(e) => s3control_state_error_response(&e),
        }
    }

    async fn handle_put_public_access_block(
        &self,
        state: &Arc<tokio::sync::RwLock<S3ControlState>>,
        input: crate::model::PutPublicAccessBlockRequest,
    ) -> MockResponse {
        let pab = input.public_access_block_configuration;
        let mut s = state.write().await;
        s.put_public_access_block(PublicAccessBlock {
            block_public_acls: pab.block_public_acls.unwrap_or(false),
            ignore_public_acls: pab.ignore_public_acls.unwrap_or(false),
            block_public_policy: pab.block_public_policy.unwrap_or(false),
            restrict_public_buckets: pab.restrict_public_buckets.unwrap_or(false),
        });
        wire::serialize_put_public_access_block_response()
    }

    async fn handle_delete_public_access_block(
        &self,
        state: &Arc<tokio::sync::RwLock<S3ControlState>>,
    ) -> MockResponse {
        let mut s = state.write().await;
        match s.delete_public_access_block() {
            Ok(()) => wire::serialize_delete_public_access_block_response(),
            Err(e) => s3control_state_error_response(&e),
        }
    }

    // -------------------------------------------------------------------------
    // Access Grants Instance handlers
    // -------------------------------------------------------------------------

    async fn handle_create_access_grants_instance(
        &self,
        state: &Arc<tokio::sync::RwLock<S3ControlState>>,
        region: &str,
        account_id: &str,
        input: crate::model::CreateAccessGrantsInstanceRequest,
    ) -> MockResponse {
        let created_at = chrono::Utc::now().to_rfc3339();
        let mut s = state.write().await;
        match s.create_access_grants_instance(
            account_id,
            region,
            input.identity_center_arn,
            created_at,
        ) {
            Ok(inst) => wire::serialize_create_access_grants_instance_response(
                &wire::CreateAccessGrantsInstanceResult {
                    access_grants_instance_id: Some(inst.instance_id.clone()),
                    access_grants_instance_arn: Some(inst.instance_arn.clone()),
                    created_at: Some(inst.created_at.clone()),
                    identity_center_arn: inst.identity_center_arn.clone(),
                    ..Default::default()
                },
            ),
            Err(e) => s3control_state_error_response(&e),
        }
    }

    async fn handle_get_access_grants_instance(
        &self,
        state: &Arc<tokio::sync::RwLock<S3ControlState>>,
    ) -> MockResponse {
        let s = state.read().await;
        match s.get_access_grants_instance() {
            Ok(inst) => wire::serialize_get_access_grants_instance_response(
                &wire::GetAccessGrantsInstanceResult {
                    access_grants_instance_id: Some(inst.instance_id.clone()),
                    access_grants_instance_arn: Some(inst.instance_arn.clone()),
                    created_at: Some(inst.created_at.clone()),
                    identity_center_arn: inst.identity_center_arn.clone(),
                    identity_center_instance_arn: inst.identity_center_instance_arn.clone(),
                    ..Default::default()
                },
            ),
            Err(e) => s3control_state_error_response(&e),
        }
    }

    async fn handle_delete_access_grants_instance(
        &self,
        state: &Arc<tokio::sync::RwLock<S3ControlState>>,
    ) -> MockResponse {
        let mut s = state.write().await;
        match s.delete_access_grants_instance() {
            Ok(()) => wire::serialize_delete_access_grants_instance_response(),
            Err(e) => s3control_state_error_response(&e),
        }
    }

    async fn handle_list_access_grants_instances(
        &self,
        state: &Arc<tokio::sync::RwLock<S3ControlState>>,
    ) -> MockResponse {
        let s = state.read().await;
        let entries: Vec<wire::ListAccessGrantsInstanceEntry> = s
            .access_grants_instance
            .iter()
            .map(|inst| wire::ListAccessGrantsInstanceEntry {
                access_grants_instance_id: Some(inst.instance_id.clone()),
                access_grants_instance_arn: Some(inst.instance_arn.clone()),
                created_at: Some(inst.created_at.clone()),
                identity_center_arn: inst.identity_center_arn.clone(),
                identity_center_instance_arn: inst.identity_center_instance_arn.clone(),
                ..Default::default()
            })
            .collect();
        wire::serialize_list_access_grants_instances_response(
            &wire::ListAccessGrantsInstancesResult {
                access_grants_instances_list: Some(entries.into()),
                next_token: None,
            },
        )
    }

    // STUB[org-integration]: Associating IAM Identity Center requires real cross-account state; accepted as no-op.
    async fn handle_associate_access_grants_identity_center(&self) -> MockResponse {
        wire::serialize_associate_access_grants_identity_center_response()
    }

    // STUB[org-integration]: Dissociating IAM Identity Center requires real cross-account state; accepted as no-op.
    async fn handle_dissociate_access_grants_identity_center(&self) -> MockResponse {
        wire::serialize_dissociate_access_grants_identity_center_response()
    }

    async fn handle_get_access_grants_instance_resource_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<S3ControlState>>,
    ) -> MockResponse {
        let s = state.read().await;
        match s.get_access_grants_resource_policy() {
            Ok(maybe_policy) => {
                let (policy, org, created_at) = maybe_policy
                    .map(|p| {
                        (
                            Some(p.policy.clone()),
                            p.organization.clone(),
                            Some(p.created_at.clone()),
                        )
                    })
                    .unwrap_or((None, None, None));
                wire::serialize_get_access_grants_instance_resource_policy_response(
                    &wire::GetAccessGrantsInstanceResourcePolicyResult {
                        policy,
                        organization: org,
                        created_at,
                    },
                )
            }
            Err(e) => s3control_state_error_response(&e),
        }
    }

    async fn handle_put_access_grants_instance_resource_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<S3ControlState>>,
        input: crate::model::PutAccessGrantsInstanceResourcePolicyRequest,
    ) -> MockResponse {
        let created_at = chrono::Utc::now().to_rfc3339();
        let mut s = state.write().await;
        match s.put_access_grants_resource_policy(input.policy, input.organization, created_at) {
            Ok(p) => wire::serialize_put_access_grants_instance_resource_policy_response(
                &wire::PutAccessGrantsInstanceResourcePolicyResult {
                    policy: Some(p.policy.clone()),
                    organization: p.organization.clone(),
                    created_at: Some(p.created_at.clone()),
                },
            ),
            Err(e) => s3control_state_error_response(&e),
        }
    }

    async fn handle_delete_access_grants_instance_resource_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<S3ControlState>>,
    ) -> MockResponse {
        let mut s = state.write().await;
        match s.delete_access_grants_resource_policy() {
            Ok(()) => wire::serialize_delete_access_grants_instance_resource_policy_response(),
            Err(e) => s3control_state_error_response(&e),
        }
    }

    // STUB[no-engine]: Prefix-based access grant lookup requires real data access evaluation; always returns empty.
    async fn handle_get_access_grants_instance_for_prefix(&self) -> MockResponse {
        wire::serialize_get_access_grants_instance_for_prefix_response(
            &wire::GetAccessGrantsInstanceForPrefixResult {
                ..Default::default()
            },
        )
    }

    // -------------------------------------------------------------------------
    // Access Grant handlers
    // -------------------------------------------------------------------------

    async fn handle_create_access_grant(
        &self,
        state: &Arc<tokio::sync::RwLock<S3ControlState>>,
        region: &str,
        account_id: &str,
        input: crate::model::CreateAccessGrantRequest,
    ) -> MockResponse {
        let created_at = chrono::Utc::now().to_rfc3339();
        let grantee_type = input.grantee.grantee_type.clone().unwrap_or_default();
        let grantee_identifier = input.grantee.grantee_identifier.clone().unwrap_or_default();
        let prefix = input
            .access_grants_location_configuration
            .as_ref()
            .and_then(|c| c.s3_sub_prefix.clone());

        let mut s = state.write().await;
        match s.create_access_grant(
            account_id,
            region,
            input.access_grants_location_id,
            prefix,
            grantee_type,
            grantee_identifier,
            input.permission,
            input.application_arn,
            created_at,
        ) {
            Ok(grant) => {
                wire::serialize_create_access_grant_response(&wire::CreateAccessGrantResult {
                    access_grant_id: Some(grant.grant_id.clone()),
                    access_grant_arn: Some(grant.grant_arn.clone()),
                    grantee: Some(wire::Grantee {
                        grantee_type: Some(grant.grantee_type.clone()),
                        grantee_identifier: Some(grant.grantee_identifier.clone()),
                    }),
                    permission: Some(grant.permission.clone()),
                    access_grants_location_id: Some(grant.location_id.clone()),
                    grant_scope: grant.grant_scope.clone(),
                    created_at: Some(grant.created_at.clone()),
                    ..Default::default()
                })
            }
            Err(e) => s3control_state_error_response(&e),
        }
    }

    async fn handle_get_access_grant(
        &self,
        state: &Arc<tokio::sync::RwLock<S3ControlState>>,
        grant_id: &str,
    ) -> MockResponse {
        let s = state.read().await;
        match s.get_access_grant(grant_id) {
            Ok(grant) => wire::serialize_get_access_grant_response(&wire::GetAccessGrantResult {
                access_grant_id: Some(grant.grant_id.clone()),
                access_grant_arn: Some(grant.grant_arn.clone()),
                grantee: Some(wire::Grantee {
                    grantee_type: Some(grant.grantee_type.clone()),
                    grantee_identifier: Some(grant.grantee_identifier.clone()),
                }),
                permission: Some(grant.permission.clone()),
                access_grants_location_id: Some(grant.location_id.clone()),
                grant_scope: grant.grant_scope.clone(),
                created_at: Some(grant.created_at.clone()),
                ..Default::default()
            }),
            Err(e) => s3control_state_error_response(&e),
        }
    }

    async fn handle_delete_access_grant(
        &self,
        state: &Arc<tokio::sync::RwLock<S3ControlState>>,
        grant_id: &str,
    ) -> MockResponse {
        let mut s = state.write().await;
        match s.delete_access_grant(grant_id) {
            Ok(()) => wire::serialize_delete_access_grant_response(),
            Err(e) => s3control_state_error_response(&e),
        }
    }

    async fn handle_list_access_grants(
        &self,
        state: &Arc<tokio::sync::RwLock<S3ControlState>>,
    ) -> MockResponse {
        let s = state.read().await;
        let mut grants: Vec<wire::ListAccessGrantEntry> = s
            .list_access_grants()
            .into_iter()
            .map(|grant| wire::ListAccessGrantEntry {
                access_grant_id: Some(grant.grant_id.clone()),
                access_grant_arn: Some(grant.grant_arn.clone()),
                grantee: Some(wire::Grantee {
                    grantee_type: Some(grant.grantee_type.clone()),
                    grantee_identifier: Some(grant.grantee_identifier.clone()),
                }),
                permission: Some(grant.permission.clone()),
                access_grants_location_id: Some(grant.location_id.clone()),
                grant_scope: grant.grant_scope.clone(),
                created_at: Some(grant.created_at.clone()),
                ..Default::default()
            })
            .collect();
        grants.sort_by(|a, b| a.access_grant_id.cmp(&b.access_grant_id));
        wire::serialize_list_access_grants_response(&wire::ListAccessGrantsResult {
            access_grants_list: Some(grants.into()),
            next_token: None,
        })
    }

    // STUB[no-auth]: Listing grants for the caller requires real caller identity; always returns empty.
    async fn handle_list_caller_access_grants(&self) -> MockResponse {
        wire::serialize_list_caller_access_grants_response(&wire::ListCallerAccessGrantsResult {
            ..Default::default()
        })
    }

    // STUB[no-engine]: Temporary credential vending (GetDataAccess) requires real data access evaluation; always returns empty.
    async fn handle_get_data_access(&self) -> MockResponse {
        wire::serialize_get_data_access_response(&wire::GetDataAccessResult {
            ..Default::default()
        })
    }

    // -------------------------------------------------------------------------
    // Access Grants Location handlers
    // -------------------------------------------------------------------------

    async fn handle_create_access_grants_location(
        &self,
        state: &Arc<tokio::sync::RwLock<S3ControlState>>,
        region: &str,
        account_id: &str,
        input: crate::model::CreateAccessGrantsLocationRequest,
    ) -> MockResponse {
        let created_at = chrono::Utc::now().to_rfc3339();
        let mut s = state.write().await;
        match s.create_access_grants_location(
            account_id,
            region,
            input.location_scope,
            input.i_a_m_role_arn,
            created_at,
        ) {
            Ok(loc) => wire::serialize_create_access_grants_location_response(
                &wire::CreateAccessGrantsLocationResult {
                    access_grants_location_id: Some(loc.location_id.clone()),
                    access_grants_location_arn: Some(loc.location_arn.clone()),
                    location_scope: Some(loc.location_scope.clone()),
                    i_a_m_role_arn: Some(loc.iam_role_arn.clone()),
                    created_at: Some(loc.created_at.clone()),
                },
            ),
            Err(e) => s3control_state_error_response(&e),
        }
    }

    async fn handle_get_access_grants_location(
        &self,
        state: &Arc<tokio::sync::RwLock<S3ControlState>>,
        location_id: &str,
    ) -> MockResponse {
        let s = state.read().await;
        match s.get_access_grants_location(location_id) {
            Ok(loc) => wire::serialize_get_access_grants_location_response(
                &wire::GetAccessGrantsLocationResult {
                    access_grants_location_id: Some(loc.location_id.clone()),
                    access_grants_location_arn: Some(loc.location_arn.clone()),
                    location_scope: Some(loc.location_scope.clone()),
                    i_a_m_role_arn: Some(loc.iam_role_arn.clone()),
                    created_at: Some(loc.created_at.clone()),
                },
            ),
            Err(e) => s3control_state_error_response(&e),
        }
    }

    async fn handle_delete_access_grants_location(
        &self,
        state: &Arc<tokio::sync::RwLock<S3ControlState>>,
        location_id: &str,
    ) -> MockResponse {
        let mut s = state.write().await;
        match s.delete_access_grants_location(location_id) {
            Ok(()) => wire::serialize_delete_access_grants_location_response(),
            Err(e) => s3control_state_error_response(&e),
        }
    }

    async fn handle_update_access_grants_location(
        &self,
        state: &Arc<tokio::sync::RwLock<S3ControlState>>,
        input: crate::model::UpdateAccessGrantsLocationRequest,
    ) -> MockResponse {
        let mut s = state.write().await;
        match s
            .update_access_grants_location(&input.access_grants_location_id, input.i_a_m_role_arn)
        {
            Ok(loc) => wire::serialize_update_access_grants_location_response(
                &wire::UpdateAccessGrantsLocationResult {
                    access_grants_location_id: Some(loc.location_id.clone()),
                    access_grants_location_arn: Some(loc.location_arn.clone()),
                    location_scope: Some(loc.location_scope.clone()),
                    i_a_m_role_arn: Some(loc.iam_role_arn.clone()),
                    created_at: Some(loc.created_at.clone()),
                },
            ),
            Err(e) => s3control_state_error_response(&e),
        }
    }

    async fn handle_list_access_grants_locations(
        &self,
        state: &Arc<tokio::sync::RwLock<S3ControlState>>,
    ) -> MockResponse {
        let s = state.read().await;
        let mut locs: Vec<wire::ListAccessGrantsLocationsEntry> = s
            .list_access_grants_locations()
            .into_iter()
            .map(|loc| wire::ListAccessGrantsLocationsEntry {
                access_grants_location_id: Some(loc.location_id.clone()),
                access_grants_location_arn: Some(loc.location_arn.clone()),
                location_scope: Some(loc.location_scope.clone()),
                i_a_m_role_arn: Some(loc.iam_role_arn.clone()),
                created_at: Some(loc.created_at.clone()),
            })
            .collect();
        locs.sort_by(|a, b| {
            a.access_grants_location_id
                .cmp(&b.access_grants_location_id)
        });
        wire::serialize_list_access_grants_locations_response(
            &wire::ListAccessGrantsLocationsResult {
                access_grants_locations_list: Some(locs.into()),
                next_token: None,
            },
        )
    }

    // -------------------------------------------------------------------------
    // Multi-Region Access Point handlers
    // -------------------------------------------------------------------------

    async fn handle_create_multi_region_access_point(
        &self,
        state: &Arc<tokio::sync::RwLock<S3ControlState>>,
        account_id: &str,
        input: crate::model::CreateMultiRegionAccessPointRequest,
    ) -> MockResponse {
        let created_at = chrono::Utc::now().to_rfc3339();
        let name = input.details.name.clone();
        let regions: Vec<MrapRegion> = input
            .details
            .regions
            .items
            .into_iter()
            .map(|r| MrapRegion {
                bucket: r.bucket,
                bucket_account_id: r.bucket_account_id,
            })
            .collect();
        let pab = input.details.public_access_block.as_ref();
        let mut s = state.write().await;
        match s.create_multi_region_access_point(
            account_id,
            name,
            regions,
            pab.and_then(|p| p.block_public_acls).unwrap_or(true),
            pab.and_then(|p| p.ignore_public_acls).unwrap_or(true),
            pab.and_then(|p| p.block_public_policy).unwrap_or(true),
            pab.and_then(|p| p.restrict_public_buckets).unwrap_or(true),
            created_at,
        ) {
            Ok(token) => {
                let token_arn =
                    format!("arn:aws:s3:us-east-1:{account_id}:async-request/mrap/create/{token}");
                wire::serialize_create_multi_region_access_point_response(
                    &wire::CreateMultiRegionAccessPointResult {
                        request_token_a_r_n: Some(token_arn),
                    },
                )
            }
            Err(e) => s3control_state_error_response(&e),
        }
    }

    async fn handle_get_multi_region_access_point(
        &self,
        state: &Arc<tokio::sync::RwLock<S3ControlState>>,
        name: &str,
    ) -> MockResponse {
        let s = state.read().await;
        match s.get_multi_region_access_point(name) {
            Ok(mrap) => {
                let regions: Vec<wire::RegionReport> = mrap
                    .regions
                    .iter()
                    .map(|r| wire::RegionReport {
                        bucket: Some(r.bucket.clone()),
                        ..Default::default()
                    })
                    .collect();
                wire::serialize_get_multi_region_access_point_response(
                    &wire::GetMultiRegionAccessPointResult {
                        access_point: Some(wire::MultiRegionAccessPointReport {
                            name: Some(mrap.name.clone()),
                            alias: Some(mrap.alias.clone()),
                            status: Some(mrap.status.clone()),
                            created_at: Some(mrap.created_at.clone()),
                            regions: Some(regions.into()),
                            public_access_block: Some(wire::PublicAccessBlockConfiguration {
                                block_public_acls: Some(mrap.block_public_acls),
                                ignore_public_acls: Some(mrap.ignore_public_acls),
                                block_public_policy: Some(mrap.block_public_policy),
                                restrict_public_buckets: Some(mrap.restrict_public_buckets),
                            }),
                        }),
                    },
                )
            }
            Err(e) => s3control_state_error_response(&e),
        }
    }

    async fn handle_delete_multi_region_access_point(
        &self,
        state: &Arc<tokio::sync::RwLock<S3ControlState>>,
        account_id: &str,
        input: crate::model::DeleteMultiRegionAccessPointRequest,
    ) -> MockResponse {
        let mut s = state.write().await;
        match s.delete_multi_region_access_point(account_id, &input.details.name) {
            Ok(token_arn) => wire::serialize_delete_multi_region_access_point_response(
                &wire::DeleteMultiRegionAccessPointResult {
                    request_token_a_r_n: Some(token_arn),
                },
            ),
            Err(e) => s3control_state_error_response(&e),
        }
    }

    async fn handle_list_multi_region_access_points(
        &self,
        state: &Arc<tokio::sync::RwLock<S3ControlState>>,
    ) -> MockResponse {
        let s = state.read().await;
        let mut mraps: Vec<wire::MultiRegionAccessPointReport> = s
            .list_multi_region_access_points()
            .into_iter()
            .map(|mrap| wire::MultiRegionAccessPointReport {
                name: Some(mrap.name.clone()),
                alias: Some(mrap.alias.clone()),
                status: Some(mrap.status.clone()),
                created_at: Some(mrap.created_at.clone()),
                ..Default::default()
            })
            .collect();
        mraps.sort_by(|a, b| a.name.cmp(&b.name));
        wire::serialize_list_multi_region_access_points_response(
            &wire::ListMultiRegionAccessPointsResult {
                access_points: Some(mraps.into()),
                next_token: None,
            },
        )
    }

    async fn handle_get_multi_region_access_point_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<S3ControlState>>,
        name: &str,
    ) -> MockResponse {
        let s = state.read().await;
        match s.get_multi_region_access_point(name) {
            Ok(mrap) => wire::serialize_get_multi_region_access_point_policy_response(
                &wire::GetMultiRegionAccessPointPolicyResult {
                    policy: mrap.policy.as_ref().map(|p| {
                        wire::MultiRegionAccessPointPolicyDocument {
                            established: Some(wire::EstablishedMultiRegionAccessPointPolicy {
                                policy: Some(p.clone()),
                            }),
                            ..Default::default()
                        }
                    }),
                },
            ),
            Err(e) => s3control_state_error_response(&e),
        }
    }

    async fn handle_get_multi_region_access_point_policy_status(
        &self,
        state: &Arc<tokio::sync::RwLock<S3ControlState>>,
        name: &str,
    ) -> MockResponse {
        let s = state.read().await;
        match s.get_multi_region_access_point(name) {
            Ok(mrap) => {
                let is_public = mrap.policy.is_some();
                wire::serialize_get_multi_region_access_point_policy_status_response(
                    &wire::GetMultiRegionAccessPointPolicyStatusResult {
                        established: Some(wire::PolicyStatus {
                            is_public: Some(is_public),
                        }),
                    },
                )
            }
            Err(e) => s3control_state_error_response(&e),
        }
    }

    async fn handle_get_multi_region_access_point_routes(
        &self,
        state: &Arc<tokio::sync::RwLock<S3ControlState>>,
        mrap_name: &str,
    ) -> MockResponse {
        let s = state.read().await;
        match s.get_mrap_routes(mrap_name) {
            Ok(routes) => {
                let wire_routes: Vec<wire::MultiRegionAccessPointRoute> = routes
                    .into_iter()
                    .map(|r| wire::MultiRegionAccessPointRoute {
                        bucket: Some(r.bucket.clone()),
                        region: Some(r.region.clone()),
                        traffic_dial_percentage: r.traffic_dial_percentage,
                    })
                    .collect();
                wire::serialize_get_multi_region_access_point_routes_response(
                    &wire::GetMultiRegionAccessPointRoutesResult {
                        mrap: Some(mrap_name.to_string()),
                        routes: if wire_routes.is_empty() {
                            None
                        } else {
                            Some(wire_routes.into())
                        },
                    },
                )
            }
            Err(e) => s3control_state_error_response(&e),
        }
    }

    async fn handle_submit_multi_region_access_point_routes(
        &self,
        state: &Arc<tokio::sync::RwLock<S3ControlState>>,
        mrap_name: &str,
        input: crate::model::SubmitMultiRegionAccessPointRoutesRequest,
    ) -> MockResponse {
        let routes: Vec<crate::types::MrapRoute> = input
            .route_updates
            .items
            .into_iter()
            .map(|r| crate::types::MrapRoute {
                bucket: r.bucket.unwrap_or_default(),
                region: r.region.unwrap_or_default(),
                traffic_dial_percentage: r.traffic_dial_percentage,
            })
            .collect();
        let mut s = state.write().await;
        match s.submit_mrap_routes(mrap_name, routes) {
            Ok(()) => wire::serialize_submit_multi_region_access_point_routes_response(),
            Err(e) => s3control_state_error_response(&e),
        }
    }

    async fn handle_put_multi_region_access_point_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<S3ControlState>>,
        input: crate::model::PutMultiRegionAccessPointPolicyRequest,
    ) -> MockResponse {
        let mut s = state.write().await;
        match s.put_multi_region_access_point_policy(&input.details.name, input.details.policy) {
            Ok(token_arn) => wire::serialize_put_multi_region_access_point_policy_response(
                &wire::PutMultiRegionAccessPointPolicyResult {
                    request_token_a_r_n: Some(token_arn),
                },
            ),
            Err(e) => s3control_state_error_response(&e),
        }
    }

    async fn handle_describe_multi_region_access_point_operation(
        &self,
        token_arn: &str,
    ) -> MockResponse {
        wire::serialize_describe_multi_region_access_point_operation_response(
            &wire::DescribeMultiRegionAccessPointOperationResult {
                async_operation: Some(wire::AsyncOperation {
                    request_token_a_r_n: Some(token_arn.to_string()),
                    request_status: Some("SUCCEEDED".to_string()),
                    ..Default::default()
                }),
            },
        )
    }

    // -------------------------------------------------------------------------
    // Job handlers
    // -------------------------------------------------------------------------

    async fn handle_create_job(
        &self,
        state: &Arc<tokio::sync::RwLock<S3ControlState>>,
        region: &str,
        account_id: &str,
        input: crate::model::CreateJobRequest,
    ) -> MockResponse {
        let created_at = chrono::Utc::now().to_rfc3339();
        // Determine operation type from the operation struct
        let operation_type = determine_job_operation_type(&input.operation);
        let mut s = state.write().await;
        match s.create_job(
            account_id,
            region,
            input.priority,
            input.role_arn,
            input.description,
            input.confirmation_required.unwrap_or(false),
            operation_type,
            created_at,
        ) {
            Ok(job_id) => wire::serialize_create_job_response(&wire::CreateJobResult {
                job_id: Some(job_id),
            }),
            Err(e) => s3control_state_error_response(&e),
        }
    }

    async fn handle_describe_job(
        &self,
        state: &Arc<tokio::sync::RwLock<S3ControlState>>,
        job_id: &str,
    ) -> MockResponse {
        let s = state.read().await;
        match s.get_job(job_id) {
            Ok(job) => wire::serialize_describe_job_response(&wire::DescribeJobResult {
                job: Some(wire::JobDescriptor {
                    job_id: Some(job.job_id.clone()),
                    job_arn: Some(job.job_arn.clone()),
                    status: Some(job.status.clone()),
                    priority: Some(job.priority),
                    description: job.description.clone(),
                    role_arn: Some(job.role_arn.clone()),
                    creation_time: Some(job.creation_time.clone()),
                    confirmation_required: Some(job.confirmation_required),
                    ..Default::default()
                }),
            }),
            Err(e) => s3control_state_error_response(&e),
        }
    }

    async fn handle_list_jobs(
        &self,
        state: &Arc<tokio::sync::RwLock<S3ControlState>>,
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let status_filter = query.get("jobStatuses").map(String::as_str);
        let s = state.read().await;
        let mut jobs: Vec<wire::JobListDescriptor> = s
            .list_jobs()
            .into_iter()
            .filter(|j| status_filter.is_none_or(|f| j.status.as_str() == f))
            .map(|job| wire::JobListDescriptor {
                job_id: Some(job.job_id.clone()),
                status: Some(job.status.clone()),
                priority: Some(job.priority),
                description: job.description.clone(),
                creation_time: Some(job.creation_time.clone()),
                ..Default::default()
            })
            .collect();
        jobs.sort_by(|a, b| a.job_id.cmp(&b.job_id));
        wire::serialize_list_jobs_response(&wire::ListJobsResult {
            jobs: Some(jobs.into()),
            next_token: None,
        })
    }

    async fn handle_get_job_tagging(
        &self,
        state: &Arc<tokio::sync::RwLock<S3ControlState>>,
        job_id: &str,
    ) -> MockResponse {
        let s = state.read().await;
        match s.get_job_tagging(job_id) {
            Ok(tags) => {
                let tag_set: Vec<wire::S3Tag> = tags
                    .into_iter()
                    .map(|(k, v)| wire::S3Tag { key: k, value: v })
                    .collect();
                wire::serialize_get_job_tagging_response(&wire::GetJobTaggingResult {
                    tags: Some(tag_set.into()),
                })
            }
            Err(e) => s3control_state_error_response(&e),
        }
    }

    async fn handle_put_job_tagging(
        &self,
        state: &Arc<tokio::sync::RwLock<S3ControlState>>,
        job_id: &str,
        input: crate::model::PutJobTaggingRequest,
    ) -> MockResponse {
        let tags: Vec<(String, String)> = input
            .tags
            .items
            .into_iter()
            .map(|t| (t.key, t.value))
            .collect();
        let mut s = state.write().await;
        match s.put_job_tagging(job_id, tags) {
            Ok(()) => wire::serialize_put_job_tagging_response(),
            Err(e) => s3control_state_error_response(&e),
        }
    }

    async fn handle_delete_job_tagging(
        &self,
        state: &Arc<tokio::sync::RwLock<S3ControlState>>,
        job_id: &str,
    ) -> MockResponse {
        let mut s = state.write().await;
        match s.delete_job_tagging(job_id) {
            Ok(()) => wire::serialize_delete_job_tagging_response(),
            Err(e) => s3control_state_error_response(&e),
        }
    }

    async fn handle_update_job_priority(
        &self,
        state: &Arc<tokio::sync::RwLock<S3ControlState>>,
        input: crate::model::UpdateJobPriorityRequest,
    ) -> MockResponse {
        let priority = input.priority;
        let mut s = state.write().await;
        match s.update_job_priority(&input.job_id, priority) {
            Ok(new_priority) => {
                wire::serialize_update_job_priority_response(&wire::UpdateJobPriorityResult {
                    job_id: Some(input.job_id),
                    priority: Some(new_priority),
                })
            }
            Err(e) => s3control_state_error_response(&e),
        }
    }

    async fn handle_update_job_status(
        &self,
        state: &Arc<tokio::sync::RwLock<S3ControlState>>,
        input: crate::model::UpdateJobStatusRequest,
    ) -> MockResponse {
        let mut s = state.write().await;
        match s.update_job_status(&input.job_id, &input.requested_job_status) {
            Ok(status) => {
                wire::serialize_update_job_status_response(&wire::UpdateJobStatusResult {
                    job_id: Some(input.job_id),
                    status: Some(status),
                    ..Default::default()
                })
            }
            Err(e) => s3control_state_error_response(&e),
        }
    }

    // -------------------------------------------------------------------------
    // Storage Lens Group handlers
    // -------------------------------------------------------------------------

    async fn handle_create_storage_lens_group(
        &self,
        state: &Arc<tokio::sync::RwLock<S3ControlState>>,
        region: &str,
        account_id: &str,
        input: crate::model::CreateStorageLensGroupRequest,
    ) -> MockResponse {
        let name = input.storage_lens_group.name.clone();
        let tags: HashMap<String, String> = input
            .tags
            .map(|tl| tl.items.into_iter().map(|t| (t.key, t.value)).collect())
            .unwrap_or_default();
        let mut s = state.write().await;
        match s.create_storage_lens_group(account_id, region, name, tags) {
            Ok(()) => wire::serialize_create_storage_lens_group_response(),
            Err(e) => s3control_state_error_response(&e),
        }
    }

    async fn handle_get_storage_lens_group(
        &self,
        state: &Arc<tokio::sync::RwLock<S3ControlState>>,
        region: &str,
        account_id: &str,
        name: &str,
    ) -> MockResponse {
        let s = state.read().await;
        match s.get_storage_lens_group(name) {
            Ok(group) => {
                let arn = format!("arn:aws:s3:{region}:{account_id}:storage-lens-group/{name}");
                wire::serialize_get_storage_lens_group_response(&wire::StorageLensGroup {
                    name: group.name.clone(),
                    storage_lens_group_arn: Some(arn),
                    filter: wire::StorageLensGroupFilter {
                        ..Default::default()
                    },
                })
            }
            Err(e) => s3control_state_error_response(&e),
        }
    }

    async fn handle_delete_storage_lens_group(
        &self,
        state: &Arc<tokio::sync::RwLock<S3ControlState>>,
        name: &str,
    ) -> MockResponse {
        let mut s = state.write().await;
        match s.delete_storage_lens_group(name) {
            Ok(()) => wire::serialize_delete_storage_lens_group_response(),
            Err(e) => s3control_state_error_response(&e),
        }
    }

    async fn handle_update_storage_lens_group(
        &self,
        state: &Arc<tokio::sync::RwLock<S3ControlState>>,
        input: crate::model::UpdateStorageLensGroupRequest,
    ) -> MockResponse {
        let mut s = state.write().await;
        match s.update_storage_lens_group(&input.name, HashMap::new()) {
            Ok(()) => wire::serialize_update_storage_lens_group_response(),
            Err(e) => s3control_state_error_response(&e),
        }
    }

    async fn handle_list_storage_lens_groups(
        &self,
        state: &Arc<tokio::sync::RwLock<S3ControlState>>,
        region: &str,
        account_id: &str,
    ) -> MockResponse {
        let s = state.read().await;
        let mut groups: Vec<wire::ListStorageLensGroupEntry> = s
            .list_storage_lens_groups()
            .into_iter()
            .map(|g| wire::ListStorageLensGroupEntry {
                name: Some(g.name.clone()),
                storage_lens_group_arn: Some(format!(
                    "arn:aws:s3:{region}:{account_id}:storage-lens-group/{}",
                    g.name
                )),
                home_region: Some(g.region.clone()),
            })
            .collect();
        groups.sort_by(|a, b| a.name.cmp(&b.name));
        wire::serialize_list_storage_lens_groups_response(&wire::ListStorageLensGroupsResult {
            storage_lens_group_list: Some(groups),
            next_token: None,
        })
    }

    // -------------------------------------------------------------------------
    // Outposts Bucket handlers
    // -------------------------------------------------------------------------

    async fn handle_create_bucket(
        &self,
        state: &Arc<tokio::sync::RwLock<S3ControlState>>,
        account_id: &str,
        input: crate::model::CreateBucketRequest,
    ) -> MockResponse {
        let creation_date = chrono::Utc::now().to_rfc3339();
        let outpost_id = input.outpost_id.unwrap_or_default();
        let mut s = state.write().await;
        match s.create_outposts_bucket(account_id, input.bucket, outpost_id, creation_date) {
            Ok(arn) => wire::serialize_create_bucket_response(&wire::CreateBucketResult {
                bucket_arn: Some(arn),
            }),
            Err(e) => s3control_state_error_response(&e),
        }
    }

    async fn handle_get_bucket(
        &self,
        state: &Arc<tokio::sync::RwLock<S3ControlState>>,
        bucket: &str,
    ) -> MockResponse {
        let s = state.read().await;
        match s.get_outposts_bucket(bucket) {
            Ok(b) => wire::serialize_get_bucket_response(&wire::GetBucketResult {
                bucket: Some(b.arn.clone()),
                creation_date: Some(b.creation_date.clone()),
                public_access_block_enabled: Some(b.public_access_block_enabled),
            }),
            Err(e) => s3control_state_error_response(&e),
        }
    }

    async fn handle_delete_bucket(
        &self,
        state: &Arc<tokio::sync::RwLock<S3ControlState>>,
        bucket: &str,
    ) -> MockResponse {
        let mut s = state.write().await;
        match s.delete_outposts_bucket(bucket) {
            Ok(()) => wire::serialize_delete_bucket_response(),
            Err(e) => s3control_state_error_response(&e),
        }
    }

    async fn handle_list_regional_buckets(
        &self,
        state: &Arc<tokio::sync::RwLock<S3ControlState>>,
        region: &str,
        account_id: &str,
    ) -> MockResponse {
        let s = state.read().await;
        let mut buckets: Vec<wire::RegionalBucket> = s
            .list_outposts_buckets()
            .into_iter()
            .map(|b| wire::RegionalBucket {
                bucket: Some(b.name.clone()),
                bucket_arn: Some(b.arn.clone()),
                creation_date: Some(b.creation_date.clone()),
                public_access_block_enabled: Some(b.public_access_block_enabled),
                outpost_id: Some(b.outpost_id.clone()),
            })
            .collect();
        buckets.sort_by(|a, b| a.bucket.cmp(&b.bucket));
        let _ = (region, account_id); // suppress unused warning
        wire::serialize_list_regional_buckets_response(&wire::ListRegionalBucketsResult {
            regional_bucket_list: Some(buckets.into()),
            next_token: None,
        })
    }

    async fn handle_get_bucket_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<S3ControlState>>,
        bucket: &str,
    ) -> MockResponse {
        let s = state.read().await;
        match s.get_outposts_bucket_policy(bucket) {
            Ok(policy) => {
                wire::serialize_get_bucket_policy_response(&wire::GetBucketPolicyResult {
                    policy: policy.map(String::from),
                })
            }
            Err(e) => s3control_state_error_response(&e),
        }
    }

    async fn handle_put_bucket_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<S3ControlState>>,
        bucket: &str,
        input: crate::model::PutBucketPolicyRequest,
    ) -> MockResponse {
        let mut s = state.write().await;
        match s.put_outposts_bucket_policy(bucket, input.policy) {
            Ok(()) => wire::serialize_put_bucket_policy_response(),
            Err(e) => s3control_state_error_response(&e),
        }
    }

    async fn handle_delete_bucket_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<S3ControlState>>,
        bucket: &str,
    ) -> MockResponse {
        let mut s = state.write().await;
        match s.delete_outposts_bucket_policy(bucket) {
            Ok(()) => wire::serialize_delete_bucket_policy_response(),
            Err(e) => s3control_state_error_response(&e),
        }
    }

    async fn handle_get_bucket_tagging(
        &self,
        state: &Arc<tokio::sync::RwLock<S3ControlState>>,
        bucket: &str,
    ) -> MockResponse {
        let s = state.read().await;
        match s.get_outposts_bucket_tagging(bucket) {
            Ok(tags) => {
                let tag_set: Vec<wire::S3Tag> = tags
                    .into_iter()
                    .map(|(k, v)| wire::S3Tag { key: k, value: v })
                    .collect();
                wire::serialize_get_bucket_tagging_response(&wire::GetBucketTaggingResult {
                    tag_set: if tag_set.is_empty() {
                        None
                    } else {
                        Some(tag_set.into())
                    },
                })
            }
            Err(e) => s3control_state_error_response(&e),
        }
    }

    async fn handle_put_bucket_tagging(
        &self,
        state: &Arc<tokio::sync::RwLock<S3ControlState>>,
        bucket: &str,
        input: crate::model::PutBucketTaggingRequest,
    ) -> MockResponse {
        let tags: Vec<(String, String)> = input
            .tagging
            .tag_set
            .items
            .into_iter()
            .map(|t| (t.key, t.value))
            .collect();
        let mut s = state.write().await;
        match s.put_outposts_bucket_tagging(bucket, tags) {
            Ok(()) => wire::serialize_put_bucket_tagging_response(),
            Err(e) => s3control_state_error_response(&e),
        }
    }

    async fn handle_delete_bucket_tagging(
        &self,
        state: &Arc<tokio::sync::RwLock<S3ControlState>>,
        bucket: &str,
    ) -> MockResponse {
        let mut s = state.write().await;
        match s.delete_outposts_bucket_tagging(bucket) {
            Ok(()) => wire::serialize_delete_bucket_tagging_response(),
            Err(e) => s3control_state_error_response(&e),
        }
    }

    // -------------------------------------------------------------------------
    // Resource Tag handlers
    // -------------------------------------------------------------------------

    async fn handle_list_tags_for_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<S3ControlState>>,
        resource_arn: &str,
    ) -> MockResponse {
        let s = state.read().await;
        let tags_map = s.list_tags_for_resource(resource_arn);
        let tags: Vec<wire::Tag> = tags_map
            .into_iter()
            .map(|(k, v)| wire::Tag { key: k, value: v })
            .collect();
        wire::serialize_list_tags_for_resource_response(&wire::ListTagsForResourceResult {
            tags: if tags.is_empty() {
                None
            } else {
                Some(tags.into())
            },
        })
    }

    async fn handle_tag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<S3ControlState>>,
        resource_arn: &str,
        input: crate::model::TagResourceRequest,
    ) -> MockResponse {
        let tags: HashMap<String, String> = input
            .tags
            .items
            .into_iter()
            .map(|t| (t.key, t.value))
            .collect();
        let mut s = state.write().await;
        s.tag_resource(resource_arn, tags);
        wire::serialize_tag_resource_response()
    }

    async fn handle_untag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<S3ControlState>>,
        resource_arn: &str,
        input: crate::model::UntagResourceRequest,
    ) -> MockResponse {
        let keys: Vec<String> = input.tag_keys.items;
        let mut s = state.write().await;
        s.untag_resource(resource_arn, &keys);
        wire::serialize_untag_resource_response()
    }

    // -------------------------------------------------------------------------
    // Storage Lens Configuration handlers
    // -------------------------------------------------------------------------

    async fn handle_put_storage_lens_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<S3ControlState>>,
        config_id: &str,
        region: &str,
        account_id: &str,
    ) -> MockResponse {
        let mut s = state.write().await;
        s.put_storage_lens_configuration(config_id, account_id, region, true);
        wire::serialize_put_storage_lens_configuration_response()
    }

    async fn handle_get_storage_lens_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<S3ControlState>>,
        config_id: &str,
    ) -> MockResponse {
        let s = state.read().await;
        match s.get_storage_lens_configuration(config_id) {
            Ok(config) => wire::serialize_get_storage_lens_configuration_response(
                &wire::StorageLensConfiguration {
                    id: config.config_id.clone(),
                    storage_lens_arn: Some(config.arn.clone()),
                    is_enabled: config.is_enabled,
                    ..Default::default()
                },
            ),
            Err(e) => s3control_state_error_response(&e),
        }
    }

    async fn handle_delete_storage_lens_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<S3ControlState>>,
        config_id: &str,
    ) -> MockResponse {
        let mut s = state.write().await;
        match s.delete_storage_lens_configuration(config_id) {
            Ok(()) => wire::serialize_delete_storage_lens_configuration_response(),
            Err(e) => s3control_state_error_response(&e),
        }
    }

    async fn handle_get_storage_lens_configuration_tagging(
        &self,
        state: &Arc<tokio::sync::RwLock<S3ControlState>>,
        config_id: &str,
    ) -> MockResponse {
        let s = state.read().await;
        match s.get_storage_lens_configuration_tagging(config_id) {
            Ok(tags) => {
                let tag_set: Vec<wire::StorageLensTag> = tags
                    .into_iter()
                    .map(|(k, v)| wire::StorageLensTag { key: k, value: v })
                    .collect();
                wire::serialize_get_storage_lens_configuration_tagging_response(
                    &wire::GetStorageLensConfigurationTaggingResult {
                        tags: if tag_set.is_empty() {
                            None
                        } else {
                            Some(tag_set.into())
                        },
                    },
                )
            }
            Err(e) => s3control_state_error_response(&e),
        }
    }

    async fn handle_put_storage_lens_configuration_tagging(
        &self,
        state: &Arc<tokio::sync::RwLock<S3ControlState>>,
        _config_id: &str,
        input: crate::model::PutStorageLensConfigurationTaggingRequest,
    ) -> MockResponse {
        let config_id = &input.config_id;
        let tags: Vec<(String, String)> = input
            .tags
            .items
            .into_iter()
            .map(|t| (t.key, t.value))
            .collect();
        let mut s = state.write().await;
        match s.put_storage_lens_configuration_tagging(config_id, tags) {
            Ok(()) => wire::serialize_put_storage_lens_configuration_tagging_response(),
            Err(e) => s3control_state_error_response(&e),
        }
    }

    async fn handle_delete_storage_lens_configuration_tagging(
        &self,
        state: &Arc<tokio::sync::RwLock<S3ControlState>>,
        config_id: &str,
    ) -> MockResponse {
        let mut s = state.write().await;
        match s.delete_storage_lens_configuration_tagging(config_id) {
            Ok(()) => wire::serialize_delete_storage_lens_configuration_tagging_response(),
            Err(e) => s3control_state_error_response(&e),
        }
    }

    async fn handle_list_storage_lens_configurations(
        &self,
        state: &Arc<tokio::sync::RwLock<S3ControlState>>,
    ) -> MockResponse {
        let s = state.read().await;
        let mut configs: Vec<wire::ListStorageLensConfigurationEntry> = s
            .list_storage_lens_configurations()
            .into_iter()
            .map(|c| wire::ListStorageLensConfigurationEntry {
                id: Some(c.config_id.clone()),
                storage_lens_arn: Some(c.arn.clone()),
                home_region: Some(c.home_region.clone()),
                is_enabled: Some(c.is_enabled),
            })
            .collect();
        configs.sort_by(|a, b| a.id.cmp(&b.id));
        wire::serialize_list_storage_lens_configurations_response(
            &wire::ListStorageLensConfigurationsResult {
                storage_lens_configuration_list: if configs.is_empty() {
                    None
                } else {
                    Some(configs)
                },
                ..Default::default()
            },
        )
    }

    // -------------------------------------------------------------------------
    // Bucket lifecycle / replication / versioning handlers
    // -------------------------------------------------------------------------

    // STUB[no-engine]: GetBucketLifecycleConfiguration response serialisation for lifecycle rules is not yet implemented; always returns empty rules.
    async fn handle_get_bucket_lifecycle_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<S3ControlState>>,
        bucket: &str,
    ) -> MockResponse {
        let s = state.read().await;
        match s.get_bucket_lifecycle_configuration(bucket) {
            Ok(_config) => wire::serialize_get_bucket_lifecycle_configuration_response(
                &wire::GetBucketLifecycleConfigurationResult {
                    ..Default::default()
                },
            ),
            Err(e) => s3control_state_error_response(&e),
        }
    }

    async fn handle_put_bucket_lifecycle_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<S3ControlState>>,
        bucket: &str,
    ) -> MockResponse {
        let config = crate::types::BucketLifecycleConfig::default();
        let mut s = state.write().await;
        match s.put_bucket_lifecycle_configuration(bucket, config) {
            Ok(()) => wire::serialize_put_bucket_lifecycle_configuration_response(),
            Err(e) => s3control_state_error_response(&e),
        }
    }

    async fn handle_delete_bucket_lifecycle_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<S3ControlState>>,
        bucket: &str,
    ) -> MockResponse {
        let mut s = state.write().await;
        match s.delete_bucket_lifecycle_configuration(bucket) {
            Ok(()) => wire::serialize_delete_bucket_lifecycle_configuration_response(),
            Err(e) => s3control_state_error_response(&e),
        }
    }

    // STUB[no-engine]: GetBucketReplication response serialisation for replication rules is not yet implemented; always returns empty configuration.
    async fn handle_get_bucket_replication(
        &self,
        state: &Arc<tokio::sync::RwLock<S3ControlState>>,
        bucket: &str,
    ) -> MockResponse {
        let s = state.read().await;
        match s.get_bucket_replication(bucket) {
            Ok(_config) => {
                wire::serialize_get_bucket_replication_response(&wire::GetBucketReplicationResult {
                    ..Default::default()
                })
            }
            Err(e) => s3control_state_error_response(&e),
        }
    }

    async fn handle_put_bucket_replication(
        &self,
        state: &Arc<tokio::sync::RwLock<S3ControlState>>,
        bucket: &str,
    ) -> MockResponse {
        let config = crate::types::BucketReplicationConfig::default();
        let mut s = state.write().await;
        match s.put_bucket_replication(bucket, config) {
            Ok(()) => wire::serialize_put_bucket_replication_response(),
            Err(e) => s3control_state_error_response(&e),
        }
    }

    async fn handle_delete_bucket_replication(
        &self,
        state: &Arc<tokio::sync::RwLock<S3ControlState>>,
        bucket: &str,
    ) -> MockResponse {
        let mut s = state.write().await;
        match s.delete_bucket_replication(bucket) {
            Ok(()) => wire::serialize_delete_bucket_replication_response(),
            Err(e) => s3control_state_error_response(&e),
        }
    }

    async fn handle_get_bucket_versioning(
        &self,
        state: &Arc<tokio::sync::RwLock<S3ControlState>>,
        bucket: &str,
    ) -> MockResponse {
        let s = state.read().await;
        match s.get_bucket_versioning(bucket) {
            Ok(config) => {
                let (status, mfa_delete) = config
                    .map(|c| (c.status.clone(), c.mfa_delete.clone()))
                    .unwrap_or_default();
                wire::serialize_get_bucket_versioning_response(&wire::GetBucketVersioningResult {
                    status: if status.is_empty() {
                        None
                    } else {
                        Some(status)
                    },
                    m_f_a_delete: if mfa_delete.is_empty() {
                        None
                    } else {
                        Some(mfa_delete)
                    },
                })
            }
            Err(e) => s3control_state_error_response(&e),
        }
    }

    async fn handle_put_bucket_versioning(
        &self,
        state: &Arc<tokio::sync::RwLock<S3ControlState>>,
        bucket: &str,
    ) -> MockResponse {
        let config = crate::types::BucketVersioningConfig::default();
        let mut s = state.write().await;
        match s.put_bucket_versioning(bucket, config) {
            Ok(()) => wire::serialize_put_bucket_versioning_response(),
            Err(e) => s3control_state_error_response(&e),
        }
    }
}

// -------------------------------------------------------------------------
// Error helpers
// -------------------------------------------------------------------------

fn s3control_state_error_response(err: &S3ControlError) -> MockResponse {
    let (status, error_type) = match err {
        S3ControlError::AccessPointAlreadyOwnedByYou(_) => (409, "AccessPointAlreadyOwnedByYou"),
        S3ControlError::NoSuchAccessPoint(_) => (404, "NoSuchAccessPoint"),
        S3ControlError::NoSuchPublicAccessBlockConfiguration => {
            (404, "NoSuchPublicAccessBlockConfiguration")
        }
        S3ControlError::AccessGrantsInstanceAlreadyExists => {
            (409, "AccessGrantsInstanceAlreadyExists")
        }
        S3ControlError::NoSuchAccessGrantsInstance => (404, "NoSuchAccessGrantsInstance"),
        S3ControlError::NoSuchAccessGrantsLocation(_) => (404, "NoSuchAccessGrantsLocation"),
        S3ControlError::NoSuchAccessGrant(_) => (404, "NoSuchAccessGrant"),
        S3ControlError::MultiRegionAccessPointAlreadyExists(_) => {
            (409, "MultiRegionAccessPointAlreadyExists")
        }
        S3ControlError::NoSuchMultiRegionAccessPoint(_) => (404, "NoSuchMultiRegionAccessPoint"),
        S3ControlError::NoSuchJob(_) => (404, "NoSuchJob"),
        S3ControlError::StorageLensGroupAlreadyExists(_) => (409, "StorageLensGroupAlreadyExists"),
        S3ControlError::NoSuchStorageLensGroup(_) => (404, "NoSuchStorageLensGroup"),
        S3ControlError::ObjectLambdaAccessPointAlreadyExists(_) => {
            (409, "ObjectLambdaAccessPointAlreadyExists")
        }
        S3ControlError::NoSuchObjectLambdaAccessPoint(_) => (404, "NoSuchAccessPoint"),
        S3ControlError::BucketAlreadyExists(_) => (409, "BucketAlreadyExists"),
        S3ControlError::NoSuchBucket(_) => (404, "NoSuchBucket"),
        S3ControlError::NoSuchStorageLensConfiguration(_) => (404, "NoSuchConfiguration"),
        S3ControlError::StorageLensConfigurationAlreadyExists(_) => {
            (409, "ConfigurationAlreadyExists")
        }
        S3ControlError::DirectoryBucketAccessPointAlreadyExists(_) => {
            (409, "AccessPointAlreadyOwnedByYou")
        }
    };
    s3control_error_response(status, error_type, &err.to_string())
}

fn s3control_error_response(status: u16, code: &str, message: &str) -> MockResponse {
    xml_response(
        &ErrorResponseXml {
            xmlns: S3CONTROL_XMLNS,
            code: code.to_string(),
            message: message.to_string(),
            request_id: uuid::Uuid::new_v4().to_string(),
            host_id: uuid::Uuid::new_v4().to_string(),
        },
        status,
    )
}

fn xml_response<T: Serialize>(value: &T, status: u16) -> MockResponse {
    MockResponse::xml(status, quick_xml::se::to_string(value).unwrap_or_default())
}

#[derive(Serialize)]
#[serde(rename = "Error")]
struct ErrorResponseXml {
    #[serde(rename = "@xmlns")]
    xmlns: &'static str,
    #[serde(rename = "Code")]
    code: String,
    #[serde(rename = "Message")]
    message: String,
    #[serde(rename = "RequestId")]
    request_id: String,
    #[serde(rename = "HostId")]
    host_id: String,
}

// -------------------------------------------------------------------------
// Utility functions
// -------------------------------------------------------------------------

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

fn parse_query_from_uri(uri: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    if let Some(query_string) = uri.split('?').nth(1) {
        for pair in query_string.split('&') {
            if let Some((key, value)) = pair.split_once('=') {
                map.insert(key.to_string(), value.to_string());
            } else if !pair.is_empty() {
                map.insert(pair.to_string(), String::new());
            }
        }
    }
    map
}

fn percent_decode_path(s: &str) -> String {
    // Simple percent-decode for ARNs in paths
    let mut result = String::new();
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%'
            && i + 2 < bytes.len()
            && let Ok(c) =
                u8::from_str_radix(std::str::from_utf8(&bytes[i + 1..i + 3]).unwrap_or(""), 16)
        {
            result.push(c as char);
            i += 3;
            continue;
        }
        result.push(bytes[i] as char);
        i += 1;
    }
    result
}

fn determine_job_operation_type(operation: &crate::model::JobOperation) -> String {
    if operation.s3_put_object_copy.is_some() {
        "S3PutObjectCopy".to_string()
    } else if operation.s3_put_object_acl.is_some() {
        "S3PutObjectAcl".to_string()
    } else if operation.s3_put_object_tagging.is_some() {
        "S3PutObjectTagging".to_string()
    } else if operation.s3_delete_object_tagging.is_some() {
        "S3DeleteObjectTagging".to_string()
    } else if operation.s3_initiate_restore_object.is_some() {
        "S3InitiateRestoreObject".to_string()
    } else if operation.s3_put_object_legal_hold.is_some() {
        "S3PutObjectLegalHold".to_string()
    } else if operation.s3_put_object_retention.is_some() {
        "S3PutObjectRetention".to_string()
    } else if operation.lambda_invoke.is_some() {
        "LambdaInvoke".to_string()
    } else {
        "Unknown".to_string()
    }
}
