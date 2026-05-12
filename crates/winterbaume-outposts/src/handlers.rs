use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::json;
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, StatefulService,
    default_account_id, extract_path, parse_query_string, rest_json_error,
};

use crate::state::{OutpostsError, OutpostsState};
use crate::views::OutpostsStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct OutpostsService {
    pub(crate) state: Arc<BackendState<OutpostsState>>,
    pub(crate) notifier: StateChangeNotifier<OutpostsStateView>,
}

impl OutpostsService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for OutpostsService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for OutpostsService {
    fn service_name(&self) -> &str {
        "outposts"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://outposts\..*\.amazonaws\.com",
            r"https?://outposts\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl OutpostsService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();
        let state = self.state.get(account_id, &region);

        let path = extract_path(&request.uri);
        let method = request.method.as_str();

        // Validate JSON body up-front; the typed deserialisers in `wire` re-parse the bytes
        // per operation.
        if !request.body.is_empty()
            && serde_json::from_slice::<serde_json::Value>(&request.body).is_err()
        {
            return rest_json_error(400, "ValidationException", "Invalid JSON body");
        }

        let segments: Vec<String> = path
            .trim_start_matches('/')
            .split('/')
            .filter(|s| !s.is_empty())
            .map(|s| {
                s.replace("%3A", ":")
                    .replace("%2F", "/")
                    .replace("%3a", ":")
                    .replace("%2f", "/")
            })
            .collect();

        let query_str = winterbaume_core::extract_query_string(&request.uri);
        let query: HashMap<String, String> = parse_query_string(query_str);

        if segments.is_empty() {
            return rest_json_error(404, "NotFoundException", "Not found");
        }

        // --- Tags: /tags/{ResourceArn+} ---
        if segments[0] == "tags" && segments.len() >= 2 {
            let resource_arn = segments[1..].join("/");
            let labels: &[(&str, &str)] = &[("ResourceArn", resource_arn.as_str())];
            let response = match method {
                "GET" => {
                    self.handle_list_tags_for_resource(&state, &request, labels, &query)
                        .await
                }
                "POST" => {
                    self.handle_tag_resource(&state, &request, labels, &query)
                        .await
                }
                "DELETE" => {
                    self.handle_untag_resource(&state, &request, labels, &query)
                        .await
                }
                _ => rest_json_error(404, "NotFoundException", "Not found"),
            };
            if matches!(method, "POST" | "DELETE") && response.status / 100 == 2 {
                self.notify_state_changed(account_id, &region).await;
            }
            return response;
        }

        let segments_ref: Vec<&str> = segments.iter().map(String::as_str).collect();
        let response = match (method, segments_ref.as_slice()) {
            // --- Outposts ---
            ("POST", ["outposts"]) => {
                self.handle_create_outpost(&state, &request, &[], &query, &region, account_id)
                    .await
            }
            ("GET", ["outposts"]) => {
                self.handle_list_outposts(&state, &request, &[], &query)
                    .await
            }
            ("GET", ["outposts", id]) => {
                let labels: &[(&str, &str)] = &[("OutpostId", id)];
                self.handle_get_outpost(&state, &request, labels, &query)
                    .await
            }
            ("DELETE", ["outposts", id]) => {
                let labels: &[(&str, &str)] = &[("OutpostId", id)];
                self.handle_delete_outpost(&state, &request, labels, &query)
                    .await
            }
            ("PATCH", ["outposts", id]) => {
                let labels: &[(&str, &str)] = &[("OutpostId", id)];
                self.handle_update_outpost(&state, &request, labels, &query)
                    .await
            }

            // --- Sites ---
            ("POST", ["sites"]) => {
                self.handle_create_site(&state, &request, &[], &query, &region, account_id)
                    .await
            }
            ("GET", ["sites"]) => self.handle_list_sites(&state, &request, &[], &query).await,
            ("GET", ["sites", id]) => {
                let labels: &[(&str, &str)] = &[("SiteId", id)];
                self.handle_get_site(&state, &request, labels, &query).await
            }
            ("DELETE", ["sites", id]) => {
                let labels: &[(&str, &str)] = &[("SiteId", id)];
                self.handle_delete_site(&state, &request, labels, &query)
                    .await
            }
            ("PATCH", ["sites", id]) => {
                let labels: &[(&str, &str)] = &[("SiteId", id)];
                self.handle_update_site(&state, &request, labels, &query)
                    .await
            }

            // --- Unimplemented operations (auto-generated stubs) ---
            // POST /outposts/{OutpostIdentifier}/capacity/{CapacityTaskId} => CancelCapacityTask
            // POST /orders/{OrderId}/cancel => CancelOrder
            // POST /orders => CreateOrder
            // GET /outposts/{OutpostIdentifier}/capacity/{CapacityTaskId} => GetCapacityTask
            // GET /catalog/item/{CatalogItemId} => GetCatalogItem
            // GET /connections/{ConnectionId} => GetConnection
            // GET /orders/{OrderId} => GetOrder
            // GET /outpost/{OutpostIdentifier}/billing-information => GetOutpostBillingInformation
            // GET /outposts/{OutpostId}/instanceTypes => GetOutpostInstanceTypes
            // GET /outposts/{OutpostIdentifier}/supportedInstanceTypes => GetOutpostSupportedInstanceTypes
            // GET /sites/{SiteId}/address => GetSiteAddress
            // GET /outposts/{OutpostIdentifier}/assetInstances => ListAssetInstances
            // GET /outposts/{OutpostIdentifier}/assets => ListAssets
            // GET /outposts/{OutpostIdentifier}/capacity/{CapacityTaskId}/blockingInstances => ListBlockingInstancesForCapacityTask
            // GET /capacity/tasks => ListCapacityTasks
            // GET /catalog/items => ListCatalogItems
            // GET /list-orders => ListOrders
            // POST /outposts/{OutpostIdentifier}/capacity => StartCapacityTask
            // POST /connections => StartConnection
            // POST /outposts/{OutpostIdentifier}/decommission => StartOutpostDecommission
            // PUT /sites/{SiteId}/address => UpdateSiteAddress
            // PATCH /sites/{SiteId}/rackPhysicalProperties => UpdateSiteRackPhysicalProperties
            _ => rest_json_error(501, "NotImplementedException", "Operation not implemented"),
        };

        // Notify on successful mutations
        if matches!(method, "POST" | "DELETE" | "PATCH" | "PUT") && response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }

        response
    }

    // -------------------------------------------------------------------------
    // Outpost handlers
    // -------------------------------------------------------------------------

    async fn handle_create_outpost(
        &self,
        state: &Arc<tokio::sync::RwLock<OutpostsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        region: &str,
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_outpost_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'Name'");
        }
        if input.site_id.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'SiteId'");
        }
        let tags = input.tags.unwrap_or_default();
        let mut state = state.write().await;
        match state.create_outpost(
            &input.name,
            &input.site_id,
            input.description.as_deref(),
            input.availability_zone.as_deref(),
            input.availability_zone_id.as_deref(),
            input.supported_hardware_type.as_deref(),
            tags,
            region,
            account_id,
        ) {
            Ok(outpost) => wire::serialize_create_outpost_response(&wire::CreateOutpostOutput {
                outpost: Some(outpost_to_wire(outpost)),
            }),
            Err(e) => outposts_error_response(&e),
        }
    }

    async fn handle_get_outpost(
        &self,
        state: &Arc<tokio::sync::RwLock<OutpostsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_outpost_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.get_outpost(&input.outpost_id) {
            Ok(outpost) => wire::serialize_get_outpost_response(&wire::GetOutpostOutput {
                outpost: Some(outpost_to_wire(outpost)),
            }),
            Err(e) => outposts_error_response(&e),
        }
    }

    async fn handle_delete_outpost(
        &self,
        state: &Arc<tokio::sync::RwLock<OutpostsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_outpost_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.delete_outpost(&input.outpost_id) {
            Ok(()) => wire::serialize_delete_outpost_response(&wire::DeleteOutpostOutput {}),
            Err(e) => outposts_error_response(&e),
        }
    }

    async fn handle_update_outpost(
        &self,
        state: &Arc<tokio::sync::RwLock<OutpostsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_outpost_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.update_outpost(
            &input.outpost_id,
            input.name.as_deref(),
            input.description.as_deref(),
            input.supported_hardware_type.as_deref(),
        ) {
            Ok(outpost) => wire::serialize_update_outpost_response(&wire::UpdateOutpostOutput {
                outpost: Some(outpost_to_wire(outpost)),
            }),
            Err(e) => outposts_error_response(&e),
        }
    }

    async fn handle_list_outposts(
        &self,
        state: &Arc<tokio::sync::RwLock<OutpostsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_list_outposts_request(request, labels, query) {
            return rest_json_error(400, "ValidationException", &e);
        }
        let state = state.read().await;
        let outposts: Vec<wire::Outpost> = state
            .list_outposts()
            .iter()
            .map(|o| outpost_to_wire(o))
            .collect();
        wire::serialize_list_outposts_response(&wire::ListOutpostsOutput {
            outposts: Some(outposts),
            next_token: None,
        })
    }

    // -------------------------------------------------------------------------
    // Site handlers
    // -------------------------------------------------------------------------

    async fn handle_create_site(
        &self,
        state: &Arc<tokio::sync::RwLock<OutpostsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        region: &str,
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_site_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'Name'");
        }
        let tags = input.tags.unwrap_or_default();
        let mut state = state.write().await;
        let site = state.create_site(
            &input.name,
            input.description.as_deref(),
            input.notes.as_deref(),
            tags,
            region,
            account_id,
        );
        wire::serialize_create_site_response(&wire::CreateSiteOutput {
            site: Some(site_to_wire(site)),
        })
    }

    async fn handle_get_site(
        &self,
        state: &Arc<tokio::sync::RwLock<OutpostsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_site_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.get_site(&input.site_id) {
            Ok(site) => wire::serialize_get_site_response(&wire::GetSiteOutput {
                site: Some(site_to_wire(site)),
            }),
            Err(e) => outposts_error_response(&e),
        }
    }

    async fn handle_delete_site(
        &self,
        state: &Arc<tokio::sync::RwLock<OutpostsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_site_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.delete_site(&input.site_id) {
            Ok(()) => wire::serialize_delete_site_response(&wire::DeleteSiteOutput {}),
            Err(e) => outposts_error_response(&e),
        }
    }

    async fn handle_update_site(
        &self,
        state: &Arc<tokio::sync::RwLock<OutpostsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_site_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.update_site(
            &input.site_id,
            input.name.as_deref(),
            input.description.as_deref(),
            input.notes.as_deref(),
        ) {
            Ok(site) => wire::serialize_update_site_response(&wire::UpdateSiteOutput {
                site: Some(site_to_wire(site)),
            }),
            Err(e) => outposts_error_response(&e),
        }
    }

    async fn handle_list_sites(
        &self,
        state: &Arc<tokio::sync::RwLock<OutpostsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_list_sites_request(request, labels, query) {
            return rest_json_error(400, "ValidationException", &e);
        }
        let state = state.read().await;
        let sites: Vec<wire::Site> = state.list_sites().iter().map(|s| site_to_wire(s)).collect();
        wire::serialize_list_sites_response(&wire::ListSitesOutput {
            sites: Some(sites),
            next_token: None,
        })
    }

    // -------------------------------------------------------------------------
    // Tag handlers
    // -------------------------------------------------------------------------

    async fn handle_list_tags_for_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<OutpostsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_tags_for_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.get_tags_for_resource(&input.resource_arn) {
            Ok(tags) => wire::serialize_list_tags_for_resource_response(
                &wire::ListTagsForResourceResponse {
                    tags: if tags.is_empty() { None } else { Some(tags) },
                },
            ),
            Err(e) => outposts_error_response(&e),
        }
    }

    async fn handle_tag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<OutpostsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_tag_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.tags.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'Tags'");
        }
        let mut state = state.write().await;
        match state.tag_resource(&input.resource_arn, input.tags) {
            Ok(()) => wire::serialize_tag_resource_response(&wire::TagResourceResponse {}),
            Err(e) => outposts_error_response(&e),
        }
    }

    async fn handle_untag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<OutpostsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_untag_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.untag_resource(&input.resource_arn, &input.tag_keys) {
            Ok(()) => wire::serialize_untag_resource_response(&wire::UntagResourceResponse {}),
            Err(e) => outposts_error_response(&e),
        }
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn outpost_to_wire(o: &crate::types::Outpost) -> wire::Outpost {
    wire::Outpost {
        outpost_id: Some(o.outpost_id.clone()),
        outpost_arn: Some(o.outpost_arn.clone()),
        owner_id: Some(o.owner_id.clone()),
        name: Some(o.name.clone()),
        description: o.description.clone(),
        site_id: Some(o.site_id.clone()),
        site_arn: Some(o.site_arn.clone()),
        availability_zone: o.availability_zone.clone(),
        availability_zone_id: o.availability_zone_id.clone(),
        life_cycle_status: Some(o.life_cycle_status.clone()),
        supported_hardware_type: o.supported_hardware_type.clone(),
        tags: if o.tags.is_empty() {
            None
        } else {
            Some(o.tags.clone())
        },
    }
}

fn site_to_wire(s: &crate::types::Site) -> wire::Site {
    wire::Site {
        site_id: Some(s.site_id.clone()),
        site_arn: Some(s.site_arn.clone()),
        account_id: Some(s.account_id.clone()),
        name: Some(s.name.clone()),
        description: s.description.clone(),
        notes: s.notes.clone(),
        operating_address_country_code: s.operating_address_country_code.clone(),
        operating_address_state_or_region: s.operating_address_state_or_region.clone(),
        operating_address_city: s.operating_address_city.clone(),
        tags: if s.tags.is_empty() {
            None
        } else {
            Some(s.tags.clone())
        },
        ..Default::default()
    }
}

fn outposts_error_response(err: &OutpostsError) -> MockResponse {
    let (status, error_type) = match err {
        OutpostsError::OutpostNotFound { .. } => (404, "NotFoundException"),
        OutpostsError::SiteNotFound { .. } => (404, "NotFoundException"),
        OutpostsError::ResourceNotFound { .. } => (404, "NotFoundException"),
        OutpostsError::Validation { .. } => (400, "ValidationException"),
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
