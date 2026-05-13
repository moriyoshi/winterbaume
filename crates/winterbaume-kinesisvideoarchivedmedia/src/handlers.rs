use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::json;
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, default_account_id,
};

use crate::state::{KinesisVideoArchivedMediaError, KinesisVideoArchivedMediaState};
use crate::views::KinesisVideoArchivedMediaStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct KinesisVideoArchivedMediaService {
    pub(crate) state: Arc<BackendState<KinesisVideoArchivedMediaState>>,
    pub(crate) notifier: StateChangeNotifier<KinesisVideoArchivedMediaStateView>,
}

impl KinesisVideoArchivedMediaService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for KinesisVideoArchivedMediaService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for KinesisVideoArchivedMediaService {
    fn service_name(&self) -> &str {
        "kinesisvideo"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://.*\.kinesisvideo\.(.+)\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl KinesisVideoArchivedMediaService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();
        let state = self.state.get(account_id, &region);

        let path = extract_path(&request.uri);
        let method = request.method.as_str();

        if method != "POST" {
            return rest_json_error(
                405,
                "MethodNotAllowedException",
                "Only POST method is supported",
            );
        }

        if !request.body.is_empty()
            && serde_json::from_slice::<serde_json::Value>(&request.body).is_err()
        {
            return rest_json_error(400, "InvalidArgumentException", "Invalid JSON body");
        }

        let labels: &[(&str, &str)] = &[];
        let query: HashMap<String, String> = HashMap::new();

        match path.as_str() {
            "/getMediaForFragmentList" => {
                self.handle_get_media_for_fragment_list(&state, &request, labels, &query)
                    .await
            }
            "/listFragments" => {
                self.handle_list_fragments(&state, &request, labels, &query)
                    .await
            }
            "/getHLSStreamingSessionURL" => {
                self.handle_get_hls_streaming_session_url(&state, &request, labels, &query)
                    .await
            }
            "/getDASHStreamingSessionURL" => {
                self.handle_get_dash_streaming_session_url(&state, &request, labels, &query)
                    .await
            }
            "/getClip" => self.handle_get_clip(&state, &request, labels, &query).await,
            "/getImages" => {
                self.handle_get_images(&state, &request, labels, &query)
                    .await
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        }
    }

    async fn handle_get_media_for_fragment_list(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisVideoArchivedMediaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_get_media_for_fragment_list_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "InvalidArgumentException", &e),
            };
        let stream_name = input.stream_name.as_deref();
        let stream_arn = input.stream_a_r_n.as_deref();
        let fragments: Vec<String> = input.fragments.clone();

        let mut state = state.write().await;
        match state.get_media_for_fragment_list(stream_name, stream_arn, &fragments) {
            Ok((content_type, payload)) => {
                let mut resp = wire::serialize_get_media_for_fragment_list_response(
                    &wire::GetMediaForFragmentListOutput {
                        content_type: Some(content_type.clone()),
                        payload: Some(String::from_utf8_lossy(&payload).to_string()),
                    },
                );
                resp.headers
                    .insert(http::header::CONTENT_TYPE, content_type.parse().unwrap());
                resp
            }
            Err(e) => kvam_error_response(&e),
        }
    }

    async fn handle_list_fragments(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisVideoArchivedMediaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_fragments_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidArgumentException", &e),
        };
        let stream_name = input.stream_name.as_deref();
        let stream_arn = input.stream_a_r_n.as_deref();
        let max_results = input.max_results.unwrap_or(100) as usize;

        let mut state = state.write().await;
        match state.list_fragments(stream_name, stream_arn) {
            Ok(fragments) => {
                let limited: Vec<wire::Fragment> = fragments
                    .iter()
                    .take(max_results)
                    .map(fragment_to_wire)
                    .collect();

                wire::serialize_list_fragments_response(&wire::ListFragmentsOutput {
                    fragments: Some(limited),
                    next_token: None,
                })
            }
            Err(e) => kvam_error_response(&e),
        }
    }

    async fn handle_get_hls_streaming_session_url(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisVideoArchivedMediaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_h_l_s_streaming_session_u_r_l_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidArgumentException", &e),
        };
        let stream_name = input.stream_name.as_deref();
        let stream_arn = input.stream_a_r_n.as_deref();

        let mut state = state.write().await;
        match state.get_hls_streaming_session_url(stream_name, stream_arn) {
            Ok(url) => wire::serialize_get_h_l_s_streaming_session_u_r_l_response(
                &wire::GetHLSStreamingSessionURLOutput {
                    h_l_s_streaming_session_u_r_l: Some(url),
                },
            ),
            Err(e) => kvam_error_response(&e),
        }
    }

    async fn handle_get_dash_streaming_session_url(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisVideoArchivedMediaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_d_a_s_h_streaming_session_u_r_l_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidArgumentException", &e),
        };
        let stream_name = input.stream_name.as_deref();
        let stream_arn = input.stream_a_r_n.as_deref();

        let mut state = state.write().await;
        match state.get_dash_streaming_session_url(stream_name, stream_arn) {
            Ok(url) => wire::serialize_get_d_a_s_h_streaming_session_u_r_l_response(
                &wire::GetDASHStreamingSessionURLOutput {
                    d_a_s_h_streaming_session_u_r_l: Some(url),
                },
            ),
            Err(e) => kvam_error_response(&e),
        }
    }

    async fn handle_get_clip(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisVideoArchivedMediaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        // ClipFragmentSelector is a default-equipped non-Option struct in the
        // wire model, so detect "absent" by checking the raw body directly.
        if !winterbaume_core::body_has_top_level_field(&request.body, "ClipFragmentSelector") {
            return rest_json_error(
                400,
                "InvalidArgumentException",
                "ClipFragmentSelector is required",
            );
        }

        let input = match wire::deserialize_get_clip_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidArgumentException", &e),
        };
        let stream_name = input.stream_name.as_deref();
        let stream_arn = input.stream_a_r_n.as_deref();

        let mut state = state.write().await;
        match state.get_clip(stream_name, stream_arn) {
            Ok(clip) => {
                let mut resp = wire::serialize_get_clip_response(&wire::GetClipOutput {
                    content_type: Some(clip.content_type.clone()),
                    payload: Some(String::from_utf8_lossy(&clip.payload).to_string()),
                });
                resp.headers.insert(
                    http::header::CONTENT_TYPE,
                    clip.content_type.parse().unwrap(),
                );
                resp
            }
            Err(e) => kvam_error_response(&e),
        }
    }

    async fn handle_get_images(
        &self,
        state: &Arc<tokio::sync::RwLock<KinesisVideoArchivedMediaState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_images_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidArgumentException", &e),
        };
        let stream_name = input.stream_name.as_deref();
        let stream_arn = input.stream_a_r_n.as_deref();
        let format = if input.format.is_empty() {
            None
        } else {
            Some(input.format.as_str())
        };

        if input.image_selector_type.is_empty() {
            return rest_json_error(
                400,
                "InvalidArgumentException",
                "ImageSelectorType is required",
            );
        }

        let max_results = input.max_results.unwrap_or(3) as usize;

        let mut state = state.write().await;
        match state.get_images(stream_name, stream_arn, format, max_results) {
            Ok(images) => {
                let wire_images: Vec<wire::Image> = images.iter().map(image_to_wire).collect();

                wire::serialize_get_images_response(&wire::GetImagesOutput {
                    images: Some(wire_images),
                    next_token: None,
                })
            }
            Err(e) => kvam_error_response(&e),
        }
    }
}

fn fragment_to_wire(f: &crate::types::Fragment) -> wire::Fragment {
    wire::Fragment {
        fragment_number: Some(f.fragment_number.clone()),
        fragment_size_in_bytes: Some(f.fragment_size_in_bytes),
        producer_timestamp: Some(f.producer_timestamp.timestamp() as f64),
        server_timestamp: Some(f.server_timestamp.timestamp() as f64),
        fragment_length_in_milliseconds: Some(f.fragment_length_in_milliseconds),
    }
}

fn image_to_wire(img: &crate::types::ImageResult) -> wire::Image {
    wire::Image {
        image_content: Some(img.image_content.clone()),
        time_stamp: Some(img.timestamp.timestamp() as f64),
        error: img
            .error
            .as_ref()
            .map(|e| format!("{}:{}", e.error_code, e.error_message)),
    }
}

fn extract_path(uri: &str) -> String {
    if let Some(idx) = uri.find("amazonaws.com") {
        let after_host = &uri[idx + "amazonaws.com".len()..];
        if let Some(q) = after_host.find('?') {
            after_host[..q].to_string()
        } else {
            after_host.to_string()
        }
    } else {
        // For non-amazonaws.com URIs, extract path from the full URI
        if let Ok(parsed) = uri.parse::<http::Uri>() {
            parsed.path().to_string()
        } else {
            uri.split('?').next().unwrap_or(uri).to_string()
        }
    }
}

fn kvam_error_response(err: &KinesisVideoArchivedMediaError) -> MockResponse {
    let (status, error_type) = match err {
        KinesisVideoArchivedMediaError::MissingStreamIdentifier => {
            (400, "InvalidArgumentException")
        }
        KinesisVideoArchivedMediaError::EmptyFragmentList => (400, "InvalidArgumentException"),
        KinesisVideoArchivedMediaError::FragmentNotFound { .. } => {
            (400, "InvalidArgumentException")
        }
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
