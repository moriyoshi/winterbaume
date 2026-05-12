use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::{Value, json, to_value};
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, StatefulService,
    default_account_id,
    protocol::common::{extract_path, extract_query_string, parse_query_string, percent_decode},
};

use crate::state::{ChimeMeetingsError, ChimeSdkMeetingsState};
use crate::views::ChimeSdkMeetingsStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct ChimeSdkMeetingsService {
    pub(crate) state: Arc<BackendState<ChimeSdkMeetingsState>>,
    pub(crate) notifier: StateChangeNotifier<ChimeSdkMeetingsStateView>,
}

impl ChimeSdkMeetingsService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for ChimeSdkMeetingsService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for ChimeSdkMeetingsService {
    fn service_name(&self) -> &str {
        "chime"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://meetings-chime\..*\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

type SharedState = Arc<tokio::sync::RwLock<ChimeSdkMeetingsState>>;

impl ChimeSdkMeetingsService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();
        let state = self.state.get(account_id, &region);

        let path = extract_path(&request.uri);
        let raw_segments: Vec<String> = path
            .trim_start_matches('/')
            .split('/')
            .filter(|s| !s.is_empty())
            .map(percent_decode)
            .collect();
        let segments: Vec<&str> = raw_segments.iter().map(|s| s.as_str()).collect();
        let query: HashMap<String, String> = parse_query_string(extract_query_string(&request.uri));
        let operation = query.get("operation").map(|s| s.as_str()).unwrap_or("");
        let method = request.method.as_str().to_string();

        let (response, mutating) = match (method.as_str(), segments.as_slice(), operation) {
            ("POST", ["meetings"], "create-attendees") => (
                self.handle_create_meeting_with_attendees(&state, &request, &[], &query, &region)
                    .await,
                true,
            ),
            ("POST", ["meetings"], _) => (
                self.handle_create_meeting(&state, &request, &[], &query, &region)
                    .await,
                true,
            ),
            ("GET", ["meetings", meeting_id], _) => {
                let labels: &[(&str, &str)] = &[("MeetingId", meeting_id)];
                (
                    self.handle_get_meeting(&state, &request, labels, &query)
                        .await,
                    false,
                )
            }
            ("DELETE", ["meetings", meeting_id], _) => {
                let labels: &[(&str, &str)] = &[("MeetingId", meeting_id)];
                (
                    self.handle_delete_meeting(&state, &request, labels, &query)
                        .await,
                    true,
                )
            }
            ("POST", ["meetings", meeting_id, "attendees"], "batch-create") => {
                let labels: &[(&str, &str)] = &[("MeetingId", meeting_id)];
                (
                    self.handle_batch_create_attendee(&state, &request, labels, &query)
                        .await,
                    true,
                )
            }
            ("POST", ["meetings", meeting_id, "attendees"], _) => {
                let labels: &[(&str, &str)] = &[("MeetingId", meeting_id)];
                (
                    self.handle_create_attendee(&state, &request, labels, &query)
                        .await,
                    true,
                )
            }
            ("GET", ["meetings", meeting_id, "attendees"], _) => {
                let labels: &[(&str, &str)] = &[("MeetingId", meeting_id)];
                (
                    self.handle_list_attendees(&state, &request, labels, &query)
                        .await,
                    false,
                )
            }
            (
                "PUT",
                ["meetings", meeting_id, "attendees", "capabilities"],
                "batch-update-except",
            ) => {
                let labels: &[(&str, &str)] = &[("MeetingId", meeting_id)];
                (
                    self.handle_batch_update_capabilities_except(&state, &request, labels, &query)
                        .await,
                    true,
                )
            }
            ("GET", ["meetings", meeting_id, "attendees", attendee_id], _) => {
                let labels: &[(&str, &str)] =
                    &[("MeetingId", meeting_id), ("AttendeeId", attendee_id)];
                (
                    self.handle_get_attendee(&state, &request, labels, &query)
                        .await,
                    false,
                )
            }
            ("DELETE", ["meetings", meeting_id, "attendees", attendee_id], _) => {
                let labels: &[(&str, &str)] =
                    &[("MeetingId", meeting_id), ("AttendeeId", attendee_id)];
                (
                    self.handle_delete_attendee(&state, &request, labels, &query)
                        .await,
                    true,
                )
            }
            (
                "PUT",
                [
                    "meetings",
                    meeting_id,
                    "attendees",
                    attendee_id,
                    "capabilities",
                ],
                _,
            ) => {
                let labels: &[(&str, &str)] =
                    &[("MeetingId", meeting_id), ("AttendeeId", attendee_id)];
                (
                    self.handle_update_attendee_capabilities(&state, &request, labels, &query)
                        .await,
                    true,
                )
            }
            ("POST", ["meetings", meeting_id, "transcription"], "start") => {
                let labels: &[(&str, &str)] = &[("MeetingId", meeting_id)];
                (
                    self.handle_start_transcription(&state, &request, labels, &query)
                        .await,
                    true,
                )
            }
            ("POST", ["meetings", meeting_id, "transcription"], "stop") => {
                let labels: &[(&str, &str)] = &[("MeetingId", meeting_id)];
                (
                    self.handle_stop_transcription(&state, &request, labels, &query)
                        .await,
                    true,
                )
            }
            ("GET", ["tags"], _) => (
                self.handle_list_tags(&state, &request, &[], &query).await,
                false,
            ),
            ("POST", ["tags"], "tag-resource") => (
                self.handle_tag_resource(&state, &request, &[], &query)
                    .await,
                true,
            ),
            ("POST", ["tags"], "untag-resource") => (
                self.handle_untag_resource(&state, &request, &[], &query)
                    .await,
                true,
            ),
            _ => (
                rest_json_error(404, "NotFoundException", "No route matches"),
                false,
            ),
        };

        if response.status / 100 == 2 && mutating {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_create_meeting(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_meeting_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let meeting_id = uuid::Uuid::new_v4().to_string();
        let media_region = if input.media_region.is_empty() {
            region
        } else {
            input.media_region.as_str()
        };
        let meeting = build_meeting_from_input(&meeting_id, media_region, &input);
        let mut state = state.write().await;
        state.put_meeting(meeting_id.clone(), meeting.clone());
        if let Some(tags) = input.tags.as_ref() {
            let map = tag_vec_to_map(tags);
            if !map.is_empty() {
                let arn = meeting_arn(&meeting_id, region);
                state.tag_resource(&arn, map);
            }
        }
        wire::serialize_create_meeting_response(&wire::CreateMeetingResponse {
            meeting: value_to_meeting(&meeting),
        })
    }

    async fn handle_create_meeting_with_attendees(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        region: &str,
    ) -> MockResponse {
        let input =
            match wire::deserialize_create_meeting_with_attendees_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let meeting_id = uuid::Uuid::new_v4().to_string();
        let media_region = if input.media_region.is_empty() {
            region
        } else {
            input.media_region.as_str()
        };
        let meeting = build_meeting_from_input_with_attendees(&meeting_id, media_region, &input);
        let mut attendee_values: Vec<Value> = vec![];
        let mut state = state.write().await;
        state.put_meeting(meeting_id.clone(), meeting.clone());
        for item in &input.attendees {
            let attendee = build_attendee_from_request_item(item);
            let attendee_id = attendee
                .get("AttendeeId")
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string();
            state.put_attendee(&meeting_id, attendee_id.clone(), attendee.clone());
            attendee_values.push(attendee);
        }
        wire::serialize_create_meeting_with_attendees_response(
            &wire::CreateMeetingWithAttendeesResponse {
                meeting: value_to_meeting(&meeting),
                attendees: Some(
                    attendee_values
                        .iter()
                        .filter_map(value_to_attendee)
                        .collect(),
                ),
                errors: Some(vec![]),
            },
        )
    }

    async fn handle_get_meeting(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_meeting_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.meetings.get(&input.meeting_id) {
            Some(record) => wire::serialize_get_meeting_response(&wire::GetMeetingResponse {
                meeting: value_to_meeting(&record.meeting),
            }),
            None => err_response(&ChimeMeetingsError::MeetingNotFound {
                id: input.meeting_id.clone(),
            }),
        }
    }

    async fn handle_delete_meeting(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_meeting_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        if state.meetings.remove(&input.meeting_id).is_some() {
            wire::serialize_delete_meeting_response()
        } else {
            err_response(&ChimeMeetingsError::MeetingNotFound {
                id: input.meeting_id.clone(),
            })
        }
    }

    async fn handle_create_attendee(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_attendee_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        if !state.meetings.contains_key(&input.meeting_id) {
            return err_response(&ChimeMeetingsError::MeetingNotFound {
                id: input.meeting_id.clone(),
            });
        }
        let attendee = build_attendee_from_create_request(&input);
        let attendee_id = attendee
            .get("AttendeeId")
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string();
        state.put_attendee(&input.meeting_id, attendee_id, attendee.clone());
        wire::serialize_create_attendee_response(&wire::CreateAttendeeResponse {
            attendee: value_to_attendee(&attendee),
        })
    }

    async fn handle_batch_create_attendee(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_batch_create_attendee_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        if !state.meetings.contains_key(&input.meeting_id) {
            return err_response(&ChimeMeetingsError::MeetingNotFound {
                id: input.meeting_id.clone(),
            });
        }
        let mut attendee_values: Vec<Value> = vec![];
        for item in &input.attendees {
            let attendee = build_attendee_from_request_item(item);
            let attendee_id = attendee
                .get("AttendeeId")
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string();
            state.put_attendee(&input.meeting_id, attendee_id, attendee.clone());
            attendee_values.push(attendee);
        }
        wire::serialize_batch_create_attendee_response(&wire::BatchCreateAttendeeResponse {
            attendees: Some(
                attendee_values
                    .iter()
                    .filter_map(value_to_attendee)
                    .collect(),
            ),
            errors: Some(vec![]),
        })
    }

    async fn handle_get_attendee(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_attendee_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state
            .meetings
            .get(&input.meeting_id)
            .and_then(|m| m.attendees.get(&input.attendee_id))
        {
            Some(a) => wire::serialize_get_attendee_response(&wire::GetAttendeeResponse {
                attendee: value_to_attendee(a),
            }),
            None => err_response(&ChimeMeetingsError::AttendeeNotFound {
                id: input.attendee_id.clone(),
            }),
        }
    }

    async fn handle_delete_attendee(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_attendee_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        let removed = state
            .meetings
            .get_mut(&input.meeting_id)
            .and_then(|m| m.attendees.remove(&input.attendee_id));
        if removed.is_some() {
            wire::serialize_delete_attendee_response()
        } else {
            err_response(&ChimeMeetingsError::AttendeeNotFound {
                id: input.attendee_id.clone(),
            })
        }
    }

    async fn handle_list_attendees(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_attendees_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        if !state.meetings.contains_key(&input.meeting_id) {
            return err_response(&ChimeMeetingsError::MeetingNotFound {
                id: input.meeting_id.clone(),
            });
        }
        let attendees: Vec<wire::Attendee> = state
            .list_attendees(&input.meeting_id)
            .into_iter()
            .filter_map(value_to_attendee)
            .collect();
        wire::serialize_list_attendees_response(&wire::ListAttendeesResponse {
            attendees: Some(attendees),
            next_token: None,
        })
    }

    async fn handle_update_attendee_capabilities(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_update_attendee_capabilities_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let mut state = state.write().await;
        let target = state
            .meetings
            .get_mut(&input.meeting_id)
            .and_then(|m| m.attendees.get_mut(&input.attendee_id));
        match target {
            Some(attendee) => {
                if let Value::Object(map) = attendee {
                    if let Ok(caps) = to_value(&input.capabilities) {
                        map.insert("Capabilities".into(), caps);
                    }
                }
                wire::serialize_update_attendee_capabilities_response(
                    &wire::UpdateAttendeeCapabilitiesResponse {
                        attendee: value_to_attendee(attendee),
                    },
                )
            }
            None => err_response(&ChimeMeetingsError::AttendeeNotFound {
                id: input.attendee_id.clone(),
            }),
        }
    }

    async fn handle_batch_update_capabilities_except(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_batch_update_attendee_capabilities_except_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let excluded: Vec<String> = input
            .excluded_attendee_ids
            .iter()
            .map(|item| item.attendee_id.clone())
            .collect();
        let mut state = state.write().await;
        let record = match state.meetings.get_mut(&input.meeting_id) {
            Some(r) => r,
            None => {
                return err_response(&ChimeMeetingsError::MeetingNotFound {
                    id: input.meeting_id.clone(),
                });
            }
        };
        let caps_value = to_value(&input.capabilities).unwrap_or(Value::Null);
        for (id, attendee) in record.attendees.iter_mut() {
            if excluded.contains(id) {
                continue;
            }
            if let Value::Object(map) = attendee {
                map.insert("Capabilities".into(), caps_value.clone());
            }
        }
        wire::serialize_batch_update_attendee_capabilities_except_response()
    }

    async fn handle_start_transcription(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_start_meeting_transcription_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let mut state = state.write().await;
        match state.meetings.get_mut(&input.meeting_id) {
            Some(m) => {
                m.transcription_active = true;
                wire::serialize_start_meeting_transcription_response()
            }
            None => err_response(&ChimeMeetingsError::MeetingNotFound {
                id: input.meeting_id.clone(),
            }),
        }
    }

    async fn handle_stop_transcription(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_stop_meeting_transcription_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let mut state = state.write().await;
        match state.meetings.get_mut(&input.meeting_id) {
            Some(m) => {
                m.transcription_active = false;
                wire::serialize_stop_meeting_transcription_response()
            }
            None => err_response(&ChimeMeetingsError::MeetingNotFound {
                id: input.meeting_id.clone(),
            }),
        }
    }

    async fn handle_list_tags(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_tags_for_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        let map = state.list_tags(&input.resource_a_r_n);
        let tags: Vec<wire::Tag> = map
            .into_iter()
            .map(|(k, v)| wire::Tag { key: k, value: v })
            .collect();
        wire::serialize_list_tags_for_resource_response(&wire::ListTagsForResourceResponse {
            tags: Some(tags),
        })
    }

    async fn handle_tag_resource(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_tag_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.resource_a_r_n.is_empty() {
            return rest_json_error(400, "ValidationException", "ResourceARN is required");
        }
        let tags = tag_vec_to_map(&input.tags);
        let mut state = state.write().await;
        state.tag_resource(&input.resource_a_r_n, tags);
        wire::serialize_tag_resource_response(&wire::TagResourceResponse {})
    }

    async fn handle_untag_resource(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_untag_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.resource_a_r_n.is_empty() {
            return rest_json_error(400, "ValidationException", "ResourceARN is required");
        }
        let mut state = state.write().await;
        state.untag_resource(&input.resource_a_r_n, &input.tag_keys);
        wire::serialize_untag_resource_response(&wire::UntagResourceResponse {})
    }
}

fn build_meeting_from_input(
    meeting_id: &str,
    region: &str,
    input: &wire::CreateMeetingRequest,
) -> Value {
    json!({
        "MeetingId": meeting_id,
        "MeetingArn": meeting_arn(meeting_id, region),
        "ExternalMeetingId": optional_string(&input.external_meeting_id),
        "MediaRegion": region,
        "MediaPlacement": media_placement(meeting_id, region),
        "MeetingFeatures": input.meeting_features.as_ref().map(|f| to_value(f).unwrap_or(Value::Null)).unwrap_or(Value::Null),
        "MeetingHostId": input.meeting_host_id.as_ref().map(|s| Value::String(s.clone())).unwrap_or(Value::Null),
        "PrimaryMeetingId": input.primary_meeting_id.as_ref().map(|s| Value::String(s.clone())).unwrap_or(Value::Null),
        "TenantIds": input.tenant_ids.as_ref().map(|v| to_value(v).unwrap_or(Value::Null)).unwrap_or(Value::Null),
    })
}

fn build_meeting_from_input_with_attendees(
    meeting_id: &str,
    region: &str,
    input: &wire::CreateMeetingWithAttendeesRequest,
) -> Value {
    json!({
        "MeetingId": meeting_id,
        "MeetingArn": meeting_arn(meeting_id, region),
        "ExternalMeetingId": optional_string(&input.external_meeting_id),
        "MediaRegion": region,
        "MediaPlacement": media_placement(meeting_id, region),
        "MeetingFeatures": input.meeting_features.as_ref().map(|f| to_value(f).unwrap_or(Value::Null)).unwrap_or(Value::Null),
        "MeetingHostId": input.meeting_host_id.as_ref().map(|s| Value::String(s.clone())).unwrap_or(Value::Null),
        "PrimaryMeetingId": input.primary_meeting_id.as_ref().map(|s| Value::String(s.clone())).unwrap_or(Value::Null),
        "TenantIds": input.tenant_ids.as_ref().map(|v| to_value(v).unwrap_or(Value::Null)).unwrap_or(Value::Null),
    })
}

fn media_placement(meeting_id: &str, region: &str) -> Value {
    json!({
        "AudioHostUrl": format!("https://meetings-{region}.amazonaws.com/{meeting_id}/audio"),
        "AudioFallbackUrl": format!("https://meetings-{region}.amazonaws.com/{meeting_id}/fallback"),
        "ScreenDataUrl": format!("https://meetings-{region}.amazonaws.com/{meeting_id}/screen-data"),
        "ScreenSharingUrl": format!("https://meetings-{region}.amazonaws.com/{meeting_id}/screen-share"),
        "ScreenViewingUrl": format!("https://meetings-{region}.amazonaws.com/{meeting_id}/screen-view"),
        "SignalingUrl": format!("wss://meetings-{region}.amazonaws.com/{meeting_id}/signaling"),
        "TurnControlUrl": format!("https://meetings-{region}.amazonaws.com/{meeting_id}/turn"),
        "EventIngestionUrl": format!("https://meetings-{region}.amazonaws.com/{meeting_id}/events"),
    })
}

fn optional_string(s: &str) -> Value {
    if s.is_empty() {
        Value::Null
    } else {
        Value::String(s.to_string())
    }
}

fn build_attendee_from_request_item(item: &wire::CreateAttendeeRequestItem) -> Value {
    let attendee_id = uuid::Uuid::new_v4().to_string();
    let join_token = format!("token-{}", uuid::Uuid::new_v4().simple());
    let capabilities = item
        .capabilities
        .as_ref()
        .and_then(|c| to_value(c).ok())
        .unwrap_or_else(default_capabilities);
    json!({
        "AttendeeId": attendee_id,
        "ExternalUserId": optional_string(&item.external_user_id),
        "Capabilities": capabilities,
        "JoinToken": join_token,
    })
}

fn build_attendee_from_create_request(req: &wire::CreateAttendeeRequest) -> Value {
    let attendee_id = uuid::Uuid::new_v4().to_string();
    let join_token = format!("token-{}", uuid::Uuid::new_v4().simple());
    let capabilities = req
        .capabilities
        .as_ref()
        .and_then(|c| to_value(c).ok())
        .unwrap_or_else(default_capabilities);
    json!({
        "AttendeeId": attendee_id,
        "ExternalUserId": optional_string(&req.external_user_id),
        "Capabilities": capabilities,
        "JoinToken": join_token,
    })
}

fn default_capabilities() -> Value {
    json!({
        "Audio": "SendReceive",
        "Content": "SendReceive",
        "Video": "SendReceive",
    })
}

fn meeting_arn(meeting_id: &str, region: &str) -> String {
    format!("arn:aws:chime::{region}:meeting/{meeting_id}")
}

fn tag_vec_to_map(tags: &[wire::Tag]) -> HashMap<String, String> {
    tags.iter()
        .map(|t| (t.key.clone(), t.value.clone()))
        .collect()
}

/// Convert a stored meeting JSON value to a typed `wire::Meeting`.
fn value_to_meeting(v: &Value) -> Option<wire::Meeting> {
    serde_json::from_value(v.clone()).ok()
}

/// Convert a stored attendee JSON value to a typed `wire::Attendee`.
fn value_to_attendee(v: &Value) -> Option<wire::Attendee> {
    serde_json::from_value(v.clone()).ok()
}

fn rest_json_error(status: u16, error_type: &str, message: &str) -> MockResponse {
    let body = json!({"__type": error_type, "Message": message});
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers
        .insert(X_AMZN_ERRORTYPE, error_type.parse().unwrap());
    resp
}

fn err_response(err: &ChimeMeetingsError) -> MockResponse {
    let (status, error_type) = match err {
        ChimeMeetingsError::MeetingNotFound { .. }
        | ChimeMeetingsError::AttendeeNotFound { .. } => (404, "NotFoundException"),
        ChimeMeetingsError::Validation { .. } => (400, "ValidationException"),
    };
    rest_json_error(status, error_type, &err.to_string())
}
