use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::future::Future;
use std::hash::{Hash, Hasher};
use std::pin::Pin;
use std::sync::Arc;

use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
    StatefulService,
};

use crate::state::{RekognitionError, RekognitionState};
use crate::types::VideoJobType;
use crate::views::RekognitionStateView;
use crate::wire;

/// Hash a serde_json::Value deterministically and return a u64 seed.
fn hash_value(v: &Value) -> u64 {
    let mut hasher = DefaultHasher::new();
    v.to_string().hash(&mut hasher);
    hasher.finish()
}

/// Map a seed to a f64 in [min, max].
fn seeded_f64(seed: u64, min: f64, max: f64) -> f64 {
    let t = (seed as f64) / (u64::MAX as f64);
    min + t * (max - min)
}

/// Remix a seed with an index to get a different but deterministic value.
fn remix(seed: u64, index: u64) -> u64 {
    let mut hasher = DefaultHasher::new();
    seed.hash(&mut hasher);
    index.hash(&mut hasher);
    hasher.finish()
}

/// Pick `count` items from `items` deterministically based on seed.
fn pick_items<T>(seed: u64, items: &[T], count: usize) -> Vec<&T> {
    if items.is_empty() || count == 0 {
        return vec![];
    }
    let count = count.min(items.len());
    // Generate indices by remixing seed
    let mut indices: Vec<usize> = Vec::new();
    let mut i = 0u64;
    while indices.len() < count {
        let idx = (remix(seed, i) as usize) % items.len();
        if !indices.contains(&idx) {
            indices.push(idx);
        }
        i += 1;
    }
    indices.iter().map(|&idx| &items[idx]).collect()
}

pub struct RekognitionService {
    pub(crate) state: Arc<BackendState<RekognitionState>>,
    pub(crate) notifier: StateChangeNotifier<RekognitionStateView>,
}

impl RekognitionService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for RekognitionService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for RekognitionService {
    fn service_name(&self) -> &str {
        "rekognition"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://rekognition\..*\.amazonaws\.com",
            r"https?://rekognition\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl RekognitionService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = DEFAULT_ACCOUNT_ID;

        let action = request
            .headers
            .get("x-amz-target")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.split('.').next_back())
            .map(|s| s.to_string());

        let action = match action {
            Some(a) => a,
            None => {
                return json_error_response(400, "MissingAction", "Missing X-Amz-Target header");
            }
        };

        if !request.body.is_empty() && serde_json::from_slice::<Value>(&request.body).is_err() {
            return json_error_response(400, "SerializationException", "Invalid JSON body");
        }
        let body_bytes: &[u8] = &request.body;

        let state = self.state.get(account_id, &region);

        let response = match action.as_str() {
            "CreateCollection" => {
                self.handle_create_collection(&state, body_bytes, &region, account_id)
                    .await
            }
            "DescribeCollection" => self.handle_describe_collection(&state, body_bytes).await,
            "DeleteCollection" => self.handle_delete_collection(&state, body_bytes).await,
            "ListCollections" => self.handle_list_collections(&state, body_bytes).await,
            "CompareFaces" => self.handle_compare_faces(body_bytes).await,
            "DetectCustomLabels" => self.handle_detect_custom_labels(body_bytes).await,
            "DetectLabels" => self.handle_detect_labels(body_bytes).await,
            "DetectText" => self.handle_detect_text(body_bytes).await,
            "StartFaceSearch" => self.handle_start_face_search(&state, body_bytes).await,
            "StartTextDetection" => self.handle_start_text_detection(&state, body_bytes).await,
            "GetFaceSearch" => self.handle_get_face_search(&state, body_bytes).await,
            "GetTextDetection" => self.handle_get_text_detection(&state, body_bytes).await,
            // --- Unimplemented operations (auto-generated stubs) ---
            "AssociateFaces" => json_error_response(
                501,
                "NotImplementedError",
                "AssociateFaces is not yet implemented in winterbaume-rekognition",
            ),
            "CopyProjectVersion" => json_error_response(
                501,
                "NotImplementedError",
                "CopyProjectVersion is not yet implemented in winterbaume-rekognition",
            ),
            "CreateDataset" => json_error_response(
                501,
                "NotImplementedError",
                "CreateDataset is not yet implemented in winterbaume-rekognition",
            ),
            "CreateFaceLivenessSession" => json_error_response(
                501,
                "NotImplementedError",
                "CreateFaceLivenessSession is not yet implemented in winterbaume-rekognition",
            ),
            "CreateProject" => json_error_response(
                501,
                "NotImplementedError",
                "CreateProject is not yet implemented in winterbaume-rekognition",
            ),
            "CreateProjectVersion" => json_error_response(
                501,
                "NotImplementedError",
                "CreateProjectVersion is not yet implemented in winterbaume-rekognition",
            ),
            "CreateStreamProcessor" => json_error_response(
                501,
                "NotImplementedError",
                "CreateStreamProcessor is not yet implemented in winterbaume-rekognition",
            ),
            "CreateUser" => json_error_response(
                501,
                "NotImplementedError",
                "CreateUser is not yet implemented in winterbaume-rekognition",
            ),
            "DeleteDataset" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteDataset is not yet implemented in winterbaume-rekognition",
            ),
            "DeleteFaces" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteFaces is not yet implemented in winterbaume-rekognition",
            ),
            "DeleteProject" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteProject is not yet implemented in winterbaume-rekognition",
            ),
            "DeleteProjectPolicy" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteProjectPolicy is not yet implemented in winterbaume-rekognition",
            ),
            "DeleteProjectVersion" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteProjectVersion is not yet implemented in winterbaume-rekognition",
            ),
            "DeleteStreamProcessor" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteStreamProcessor is not yet implemented in winterbaume-rekognition",
            ),
            "DeleteUser" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteUser is not yet implemented in winterbaume-rekognition",
            ),
            "DescribeDataset" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeDataset is not yet implemented in winterbaume-rekognition",
            ),
            "DescribeProjectVersions" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeProjectVersions is not yet implemented in winterbaume-rekognition",
            ),
            "DescribeProjects" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeProjects is not yet implemented in winterbaume-rekognition",
            ),
            "DescribeStreamProcessor" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeStreamProcessor is not yet implemented in winterbaume-rekognition",
            ),
            "DetectFaces" => json_error_response(
                501,
                "NotImplementedError",
                "DetectFaces is not yet implemented in winterbaume-rekognition",
            ),
            "DetectModerationLabels" => json_error_response(
                501,
                "NotImplementedError",
                "DetectModerationLabels is not yet implemented in winterbaume-rekognition",
            ),
            "DetectProtectiveEquipment" => json_error_response(
                501,
                "NotImplementedError",
                "DetectProtectiveEquipment is not yet implemented in winterbaume-rekognition",
            ),
            "DisassociateFaces" => json_error_response(
                501,
                "NotImplementedError",
                "DisassociateFaces is not yet implemented in winterbaume-rekognition",
            ),
            "DistributeDatasetEntries" => json_error_response(
                501,
                "NotImplementedError",
                "DistributeDatasetEntries is not yet implemented in winterbaume-rekognition",
            ),
            "GetCelebrityInfo" => json_error_response(
                501,
                "NotImplementedError",
                "GetCelebrityInfo is not yet implemented in winterbaume-rekognition",
            ),
            "GetCelebrityRecognition" => json_error_response(
                501,
                "NotImplementedError",
                "GetCelebrityRecognition is not yet implemented in winterbaume-rekognition",
            ),
            "GetContentModeration" => json_error_response(
                501,
                "NotImplementedError",
                "GetContentModeration is not yet implemented in winterbaume-rekognition",
            ),
            "GetFaceDetection" => json_error_response(
                501,
                "NotImplementedError",
                "GetFaceDetection is not yet implemented in winterbaume-rekognition",
            ),
            "GetFaceLivenessSessionResults" => json_error_response(
                501,
                "NotImplementedError",
                "GetFaceLivenessSessionResults is not yet implemented in winterbaume-rekognition",
            ),
            "GetLabelDetection" => json_error_response(
                501,
                "NotImplementedError",
                "GetLabelDetection is not yet implemented in winterbaume-rekognition",
            ),
            "GetMediaAnalysisJob" => json_error_response(
                501,
                "NotImplementedError",
                "GetMediaAnalysisJob is not yet implemented in winterbaume-rekognition",
            ),
            "GetPersonTracking" => json_error_response(
                501,
                "NotImplementedError",
                "GetPersonTracking is not yet implemented in winterbaume-rekognition",
            ),
            "GetSegmentDetection" => json_error_response(
                501,
                "NotImplementedError",
                "GetSegmentDetection is not yet implemented in winterbaume-rekognition",
            ),
            "IndexFaces" => json_error_response(
                501,
                "NotImplementedError",
                "IndexFaces is not yet implemented in winterbaume-rekognition",
            ),
            "ListDatasetEntries" => json_error_response(
                501,
                "NotImplementedError",
                "ListDatasetEntries is not yet implemented in winterbaume-rekognition",
            ),
            "ListDatasetLabels" => json_error_response(
                501,
                "NotImplementedError",
                "ListDatasetLabels is not yet implemented in winterbaume-rekognition",
            ),
            "ListFaces" => json_error_response(
                501,
                "NotImplementedError",
                "ListFaces is not yet implemented in winterbaume-rekognition",
            ),
            "ListMediaAnalysisJobs" => json_error_response(
                501,
                "NotImplementedError",
                "ListMediaAnalysisJobs is not yet implemented in winterbaume-rekognition",
            ),
            "ListProjectPolicies" => json_error_response(
                501,
                "NotImplementedError",
                "ListProjectPolicies is not yet implemented in winterbaume-rekognition",
            ),
            "ListStreamProcessors" => json_error_response(
                501,
                "NotImplementedError",
                "ListStreamProcessors is not yet implemented in winterbaume-rekognition",
            ),
            "ListTagsForResource" => json_error_response(
                501,
                "NotImplementedError",
                "ListTagsForResource is not yet implemented in winterbaume-rekognition",
            ),
            "ListUsers" => json_error_response(
                501,
                "NotImplementedError",
                "ListUsers is not yet implemented in winterbaume-rekognition",
            ),
            "PutProjectPolicy" => json_error_response(
                501,
                "NotImplementedError",
                "PutProjectPolicy is not yet implemented in winterbaume-rekognition",
            ),
            "RecognizeCelebrities" => json_error_response(
                501,
                "NotImplementedError",
                "RecognizeCelebrities is not yet implemented in winterbaume-rekognition",
            ),
            "SearchFaces" => json_error_response(
                501,
                "NotImplementedError",
                "SearchFaces is not yet implemented in winterbaume-rekognition",
            ),
            "SearchFacesByImage" => json_error_response(
                501,
                "NotImplementedError",
                "SearchFacesByImage is not yet implemented in winterbaume-rekognition",
            ),
            "SearchUsers" => json_error_response(
                501,
                "NotImplementedError",
                "SearchUsers is not yet implemented in winterbaume-rekognition",
            ),
            "SearchUsersByImage" => json_error_response(
                501,
                "NotImplementedError",
                "SearchUsersByImage is not yet implemented in winterbaume-rekognition",
            ),
            "StartCelebrityRecognition" => json_error_response(
                501,
                "NotImplementedError",
                "StartCelebrityRecognition is not yet implemented in winterbaume-rekognition",
            ),
            "StartContentModeration" => json_error_response(
                501,
                "NotImplementedError",
                "StartContentModeration is not yet implemented in winterbaume-rekognition",
            ),
            "StartFaceDetection" => json_error_response(
                501,
                "NotImplementedError",
                "StartFaceDetection is not yet implemented in winterbaume-rekognition",
            ),
            "StartLabelDetection" => json_error_response(
                501,
                "NotImplementedError",
                "StartLabelDetection is not yet implemented in winterbaume-rekognition",
            ),
            "StartMediaAnalysisJob" => json_error_response(
                501,
                "NotImplementedError",
                "StartMediaAnalysisJob is not yet implemented in winterbaume-rekognition",
            ),
            "StartPersonTracking" => json_error_response(
                501,
                "NotImplementedError",
                "StartPersonTracking is not yet implemented in winterbaume-rekognition",
            ),
            "StartProjectVersion" => json_error_response(
                501,
                "NotImplementedError",
                "StartProjectVersion is not yet implemented in winterbaume-rekognition",
            ),
            "StartSegmentDetection" => json_error_response(
                501,
                "NotImplementedError",
                "StartSegmentDetection is not yet implemented in winterbaume-rekognition",
            ),
            "StartStreamProcessor" => json_error_response(
                501,
                "NotImplementedError",
                "StartStreamProcessor is not yet implemented in winterbaume-rekognition",
            ),
            "StopProjectVersion" => json_error_response(
                501,
                "NotImplementedError",
                "StopProjectVersion is not yet implemented in winterbaume-rekognition",
            ),
            "StopStreamProcessor" => json_error_response(
                501,
                "NotImplementedError",
                "StopStreamProcessor is not yet implemented in winterbaume-rekognition",
            ),
            "TagResource" => json_error_response(
                501,
                "NotImplementedError",
                "TagResource is not yet implemented in winterbaume-rekognition",
            ),
            "UntagResource" => json_error_response(
                501,
                "NotImplementedError",
                "UntagResource is not yet implemented in winterbaume-rekognition",
            ),
            "UpdateDatasetEntries" => json_error_response(
                501,
                "NotImplementedError",
                "UpdateDatasetEntries is not yet implemented in winterbaume-rekognition",
            ),
            "UpdateStreamProcessor" => json_error_response(
                501,
                "NotImplementedError",
                "UpdateStreamProcessor is not yet implemented in winterbaume-rekognition",
            ),
            _ => json_error_response(400, "InvalidAction", &format!("Unknown operation {action}")),
        };
        if response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_create_collection(
        &self,
        state: &Arc<tokio::sync::RwLock<RekognitionState>>,
        body: &[u8],
        region: &str,
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_collection_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.collection_id.is_empty() {
            return json_error_response(
                400,
                "InvalidParameterException",
                "CollectionId is required",
            );
        }
        let collection_id = input.collection_id.as_str();
        let tags: HashMap<String, String> = input.tags.unwrap_or_default();

        let mut state = state.write().await;
        match state.create_collection(collection_id, tags, region, account_id) {
            Ok(collection) => {
                wire::serialize_create_collection_response(&wire::CreateCollectionResponse {
                    status_code: Some(200),
                    collection_arn: Some(collection.collection_arn.clone()),
                    face_model_version: Some(collection.face_model_version.clone()),
                })
            }
            Err(e) => rekognition_error_response(&e),
        }
    }

    async fn handle_describe_collection(
        &self,
        state: &Arc<tokio::sync::RwLock<RekognitionState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_collection_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.collection_id.is_empty() {
            return json_error_response(
                400,
                "InvalidParameterException",
                "CollectionId is required",
            );
        }
        let collection_id = input.collection_id.as_str();

        let state = state.read().await;
        match state.describe_collection(collection_id) {
            Ok(collection) => {
                wire::serialize_describe_collection_response(&wire::DescribeCollectionResponse {
                    face_count: Some(collection.face_count as i64),
                    face_model_version: Some(collection.face_model_version.clone()),
                    collection_a_r_n: Some(collection.collection_arn.clone()),
                    creation_timestamp: Some(collection.creation_timestamp),
                    user_count: Some(collection.user_count as i64),
                })
            }
            Err(e) => rekognition_error_response(&e),
        }
    }

    async fn handle_delete_collection(
        &self,
        state: &Arc<tokio::sync::RwLock<RekognitionState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_collection_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.collection_id.is_empty() {
            return json_error_response(
                400,
                "InvalidParameterException",
                "CollectionId is required",
            );
        }
        let collection_id = input.collection_id.as_str();

        let mut state = state.write().await;
        match state.delete_collection(collection_id) {
            Ok(()) => wire::serialize_delete_collection_response(&wire::DeleteCollectionResponse {
                status_code: Some(200),
            }),
            Err(e) => rekognition_error_response(&e),
        }
    }

    async fn handle_list_collections(
        &self,
        state: &Arc<tokio::sync::RwLock<RekognitionState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_collections_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let max_results = input.max_results.map(|v| v as usize);
        let next_token = input.next_token.as_deref();

        let state = state.read().await;
        let (collections, next) = state.list_collections(max_results, next_token);

        let collection_ids: Vec<String> = collections
            .iter()
            .map(|c| c.collection_id.clone())
            .collect();
        let face_model_versions: Vec<String> = collections
            .iter()
            .map(|c| c.face_model_version.clone())
            .collect();

        wire::serialize_list_collections_response(&wire::ListCollectionsResponse {
            collection_ids: Some(collection_ids),
            next_token: next,
            face_model_versions: Some(face_model_versions),
        })
    }

    /// CompareFaces - returns input-seeded, deterministic face comparison results.
    /// This is a stateless operation since we don't do real image analysis.
    // STUB[no-engine]: Image comparison requires a real face-recognition engine; returns input-seeded mock match data.
    async fn handle_compare_faces(&self, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_compare_faces_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let raw: Value = serde_json::from_slice(body).unwrap_or(Value::Null);
        // Validate required fields - check that the user actually provided them
        if raw.get("SourceImage").is_none() {
            return json_error_response(
                400,
                "InvalidParameterException",
                "SourceImage is required",
            );
        }
        if raw.get("TargetImage").is_none() {
            return json_error_response(
                400,
                "InvalidParameterException",
                "TargetImage is required",
            );
        }
        let _ = &input;

        // Derive seed from source + target image content
        let mut hasher = DefaultHasher::new();
        raw.get("SourceImage")
            .map(|v| v.to_string())
            .unwrap_or_default()
            .hash(&mut hasher);
        raw.get("TargetImage")
            .map(|v| v.to_string())
            .unwrap_or_default()
            .hash(&mut hasher);
        let seed = hasher.finish();

        let confidence = seeded_f64(remix(seed, 0), 85.0, 99.99);
        let similarity = seeded_f64(remix(seed, 1), 85.0, 100.0);
        let src_width = seeded_f64(remix(seed, 2), 0.3, 0.7);
        let src_height = seeded_f64(remix(seed, 3), 0.2, 0.5);
        let src_left = seeded_f64(remix(seed, 4), 0.1, 0.4);
        let src_top = seeded_f64(remix(seed, 5), 0.05, 0.3);

        let tgt_width = seeded_f64(remix(seed, 6), 0.3, 0.7);
        let tgt_height = seeded_f64(remix(seed, 7), 0.2, 0.5);
        let tgt_left = seeded_f64(remix(seed, 8), 0.1, 0.4);
        let tgt_top = seeded_f64(remix(seed, 9), 0.05, 0.3);

        let unmatched_width = seeded_f64(remix(seed, 10), 0.3, 0.6);
        let unmatched_height = seeded_f64(remix(seed, 11), 0.2, 0.4);
        let unmatched_left = seeded_f64(remix(seed, 12), 0.05, 0.3);
        let unmatched_top = seeded_f64(remix(seed, 13), 0.4, 0.8);
        let unmatched_confidence = seeded_f64(remix(seed, 14), 90.0, 99.999);

        let yaw = seeded_f64(remix(seed, 15), -90.0, 90.0);
        let roll = seeded_f64(remix(seed, 16), -120.0, 120.0);
        let pitch = seeded_f64(remix(seed, 17), -10.0, 10.0);

        // Generate landmark positions relative to bounding box
        let lm = |i: u64| -> (f64, f64) {
            (
                seeded_f64(remix(seed, 20 + i * 2), 0.2, 0.8),
                seeded_f64(remix(seed, 21 + i * 2), 0.2, 0.8),
            )
        };
        let (el_x, el_y) = lm(0);
        let (er_x, er_y) = lm(1);
        let (n_x, n_y) = lm(2);
        let (ml_x, ml_y) = lm(3);
        let (mr_x, mr_y) = lm(4);

        wire::serialize_compare_faces_response(&wire::CompareFacesResponse {
            source_image_face: serde_json::from_value(json!({
                "BoundingBox": {
                    "Width": src_width,
                    "Top": src_top,
                    "Left": src_left,
                    "Height": src_height
                },
                "Confidence": confidence
            }))
            .ok(),
            face_matches: serde_json::from_value(json!([{
                "Face": {
                    "BoundingBox": {
                        "Width": tgt_width,
                        "Top": tgt_top,
                        "Left": tgt_left,
                        "Height": tgt_height
                    },
                    "Confidence": confidence,
                    "Pose": {
                        "Yaw": yaw,
                        "Roll": roll,
                        "Pitch": pitch
                    },
                    "Quality": {
                        "Sharpness": seeded_f64(remix(seed, 30), 80.0, 99.999),
                        "Brightness": seeded_f64(remix(seed, 31), 40.0, 80.0)
                    },
                    "Landmarks": [
                        {"Y": el_y, "X": el_x, "Type": "eyeLeft"},
                        {"Y": er_y, "X": er_x, "Type": "eyeRight"},
                        {"Y": n_y, "X": n_x, "Type": "nose"},
                        {"Y": ml_y, "X": ml_x, "Type": "mouthLeft"},
                        {"Y": mr_y, "X": mr_x, "Type": "mouthRight"}
                    ]
                },
                "Similarity": similarity
            }]))
            .ok(),
            unmatched_faces: serde_json::from_value(json!([{
                "BoundingBox": {
                    "Width": unmatched_width,
                    "Top": unmatched_top,
                    "Left": unmatched_left,
                    "Height": unmatched_height
                },
                "Confidence": unmatched_confidence,
                "Pose": {
                    "Yaw": seeded_f64(remix(seed, 40), -90.0, 90.0),
                    "Roll": seeded_f64(remix(seed, 41), -120.0, 120.0),
                    "Pitch": seeded_f64(remix(seed, 42), -10.0, 10.0)
                },
                "Quality": {
                    "Sharpness": seeded_f64(remix(seed, 43), 80.0, 99.999),
                    "Brightness": seeded_f64(remix(seed, 44), 40.0, 80.0)
                },
                "Landmarks": [
                    {"Y": seeded_f64(remix(seed, 50), 0.2, 0.9), "X": seeded_f64(remix(seed, 51), 0.2, 0.5), "Type": "eyeLeft"},
                    {"Y": seeded_f64(remix(seed, 52), 0.2, 0.9), "X": seeded_f64(remix(seed, 53), 0.2, 0.5), "Type": "eyeRight"},
                    {"Y": seeded_f64(remix(seed, 54), 0.2, 0.9), "X": seeded_f64(remix(seed, 55), 0.2, 0.5), "Type": "nose"},
                    {"Y": seeded_f64(remix(seed, 56), 0.2, 0.9), "X": seeded_f64(remix(seed, 57), 0.3, 0.6), "Type": "mouthLeft"},
                    {"Y": seeded_f64(remix(seed, 58), 0.2, 0.9), "X": seeded_f64(remix(seed, 59), 0.3, 0.6), "Type": "mouthRight"}
                ]
            }]))
            .ok(),
            source_image_orientation_correction: Some("ROTATE_0".to_string()),
            target_image_orientation_correction: Some("ROTATE_0".to_string()),
        })
    }

    /// DetectLabels - returns input-seeded, deterministic label detection results.
    // STUB[no-engine]: Label detection requires a real image analysis engine; returns input-seeded mock labels.
    async fn handle_detect_labels(&self, body: &[u8]) -> MockResponse {
        let _input = match wire::deserialize_detect_labels_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let raw: Value = serde_json::from_slice(body).unwrap_or(Value::Null);
        let image = match raw.get("Image") {
            Some(v) => v.clone(),
            None => {
                return json_error_response(400, "InvalidParameterException", "Image is required");
            }
        };
        let image = &image;

        static LABEL_DEFS: &[(&str, &str, &str)] = &[
            ("Person", "Human", "People and Faces"),
            ("Car", "Vehicle", "Transport and Logistics"),
            ("Dog", "Animal", "Animals and Pets"),
            ("Tree", "Plant", "Nature and Outdoors"),
            ("Building", "Architecture", "Buildings and Architecture"),
            ("Sky", "Nature", "Nature and Outdoors"),
            ("Water", "Nature", "Nature and Outdoors"),
            ("Food", "Cuisine", "Food and Beverage"),
            ("Text", "Writing", "Text and Documents"),
            ("Furniture", "Home", "Home and Garden"),
        ];

        let seed = hash_value(image);
        let label_count = 2 + (remix(seed, 0) % 3) as usize; // 2-4 labels
        let selected = pick_items(seed, LABEL_DEFS, label_count);

        let labels: Vec<Value> = selected
            .iter()
            .enumerate()
            .map(|(i, &&(name, parent, category))| {
                let conf = seeded_f64(remix(seed, 100 + i as u64), 80.0, 99.9);
                let bb_w = seeded_f64(remix(seed, 200 + i as u64), 0.1, 0.5);
                let bb_h = seeded_f64(remix(seed, 201 + i as u64), 0.3, 0.9);
                let bb_l = seeded_f64(remix(seed, 202 + i as u64), 0.1, 0.5);
                let bb_t = seeded_f64(remix(seed, 203 + i as u64), 0.05, 0.4);
                json!({
                    "Name": name,
                    "Parents": [{"Name": parent}],
                    "Aliases": [],
                    "Categories": [{"Name": category}],
                    "Confidence": conf,
                    "Instances": [{
                        "BoundingBox": {
                            "Width": bb_w,
                            "Height": bb_h,
                            "Left": bb_l,
                            "Top": bb_t
                        },
                        "Confidence": conf,
                        "DominantColors": [{
                            "Red": (remix(seed, 300 + i as u64) % 256) as u32,
                            "Green": (remix(seed, 301 + i as u64) % 256) as u32,
                            "Blue": (remix(seed, 302 + i as u64) % 256) as u32,
                            "HexCode": format!(
                                "{:02X}{:02X}{:02X}",
                                remix(seed, 300 + i as u64) % 256,
                                remix(seed, 301 + i as u64) % 256,
                                remix(seed, 302 + i as u64) % 256
                            ),
                            "SimplifiedColor": "grey",
                            "CSSColor": "grey",
                            "PixelPercent": seeded_f64(remix(seed, 310 + i as u64), 10.0, 50.0)
                        }]
                    }]
                })
            })
            .collect();

        let brightness = seeded_f64(remix(seed, 400), 30.0, 70.0);
        let sharpness = seeded_f64(remix(seed, 401), 30.0, 70.0);
        let contrast = seeded_f64(remix(seed, 402), 15.0, 40.0);

        wire::serialize_detect_labels_response(&wire::DetectLabelsResponse {
            labels: serde_json::from_value(json!(labels)).ok(),
            orientation_correction: None,
            label_model_version: Some("3.0".to_string()),
            image_properties: serde_json::from_value(json!({
                "Quality": {
                    "Brightness": brightness,
                    "Sharpness": sharpness,
                    "Contrast": contrast
                },
                "DominantColors": [{
                    "Red": (remix(seed, 410) % 256) as u32,
                    "Green": (remix(seed, 411) % 256) as u32,
                    "Blue": (remix(seed, 412) % 256) as u32,
                    "HexCode": format!(
                        "{:02X}{:02X}{:02X}",
                        remix(seed, 410) % 256,
                        remix(seed, 411) % 256,
                        remix(seed, 412) % 256
                    ),
                    "SimplifiedColor": "grey",
                    "CSSColor": "grey",
                    "PixelPercent": seeded_f64(remix(seed, 413), 20.0, 60.0)
                }],
                "Foreground": {
                    "Quality": {
                        "Brightness": brightness + 5.0,
                        "Sharpness": sharpness + 5.0
                    },
                    "DominantColors": [{
                        "Red": (remix(seed, 420) % 256) as u32,
                        "Green": (remix(seed, 421) % 256) as u32,
                        "Blue": (remix(seed, 422) % 256) as u32,
                        "HexCode": format!(
                            "{:02X}{:02X}{:02X}",
                            remix(seed, 420) % 256,
                            remix(seed, 421) % 256,
                            remix(seed, 422) % 256
                        ),
                        "SimplifiedColor": "grey",
                        "PixelPercent": seeded_f64(remix(seed, 423), 15.0, 45.0)
                    }]
                },
                "Background": {
                    "Quality": {
                        "Brightness": brightness - 5.0,
                        "Sharpness": sharpness - 5.0
                    },
                    "DominantColors": [{
                        "Red": (remix(seed, 430) % 256) as u32,
                        "Green": (remix(seed, 431) % 256) as u32,
                        "Blue": (remix(seed, 432) % 256) as u32,
                        "HexCode": format!(
                            "{:02X}{:02X}{:02X}",
                            remix(seed, 430) % 256,
                            remix(seed, 431) % 256,
                            remix(seed, 432) % 256
                        ),
                        "SimplifiedColor": "grey",
                        "PixelPercent": seeded_f64(remix(seed, 433), 5.0, 25.0)
                    }]
                }
            }))
            .ok(),
        })
    }

    /// DetectText - returns input-seeded, deterministic text detection results.
    // STUB[no-engine]: Text detection requires a real OCR engine; returns input-seeded mock text results.
    async fn handle_detect_text(&self, body: &[u8]) -> MockResponse {
        let _input = match wire::deserialize_detect_text_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let raw: Value = serde_json::from_slice(body).unwrap_or(Value::Null);
        let image = match raw.get("Image") {
            Some(v) => v.clone(),
            None => {
                return json_error_response(400, "InvalidParameterException", "Image is required");
            }
        };
        let image = &image;

        static SAMPLE_TEXTS: &[&str] = &[
            "HELLO", "WORLD", "OPEN", "EXIT", "STOP", "WARNING", "ENTER", "WELCOME",
        ];

        let seed = hash_value(image);
        let text_count = 1 + (remix(seed, 0) % 3) as usize; // 1-3 texts
        let selected = pick_items(seed, SAMPLE_TEXTS, text_count);

        let mut detections = Vec::new();
        let mut id = 0i64;

        for (i, &&text) in selected.iter().enumerate() {
            let conf = seeded_f64(remix(seed, 100 + i as u64), 90.0, 99.9);
            let bb_left = seeded_f64(remix(seed, 200 + i as u64 * 4), 0.1, 0.7);
            let bb_top = seeded_f64(remix(seed, 201 + i as u64 * 4), 0.1, 0.8);
            let bb_width = seeded_f64(remix(seed, 202 + i as u64 * 4), 0.1, 0.4);
            let bb_height = seeded_f64(remix(seed, 203 + i as u64 * 4), 0.05, 0.15);

            let line_id = id;
            // LINE detection
            detections.push(json!({
                "Confidence": conf,
                "DetectedText": text,
                "Geometry": {
                    "BoundingBox": {
                        "Height": bb_height,
                        "Left": bb_left,
                        "Top": bb_top,
                        "Width": bb_width
                    },
                    "Polygon": [
                        {"X": bb_left, "Y": bb_top},
                        {"X": bb_left + bb_width, "Y": bb_top},
                        {"X": bb_left + bb_width, "Y": bb_top + bb_height},
                        {"X": bb_left, "Y": bb_top + bb_height}
                    ]
                },
                "Id": line_id,
                "Type": "LINE"
            }));
            id += 1;

            // WORD detection (same text as LINE but with ParentId)
            detections.push(json!({
                "Confidence": conf,
                "DetectedText": text,
                "Geometry": {
                    "BoundingBox": {
                        "Height": bb_height,
                        "Left": bb_left,
                        "Top": bb_top,
                        "Width": bb_width
                    },
                    "Polygon": [
                        {"X": bb_left, "Y": bb_top},
                        {"X": bb_left + bb_width, "Y": bb_top},
                        {"X": bb_left + bb_width, "Y": bb_top + bb_height},
                        {"X": bb_left, "Y": bb_top + bb_height}
                    ]
                },
                "Id": id,
                "ParentId": line_id,
                "Type": "WORD"
            }));
            id += 1;
        }

        wire::serialize_detect_text_response(&wire::DetectTextResponse {
            text_detections: serde_json::from_value(json!(detections)).ok(),
            text_model_version: Some("3.0".to_string()),
        })
    }

    /// DetectCustomLabels - returns input-seeded, deterministic custom label results.
    // STUB[no-engine]: Custom label detection requires a real model inference engine; returns input-seeded mock labels.
    async fn handle_detect_custom_labels(&self, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_detect_custom_labels_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let raw: Value = serde_json::from_slice(body).unwrap_or(Value::Null);
        if input.project_version_arn.is_empty() {
            return json_error_response(
                400,
                "InvalidParameterException",
                "ProjectVersionArn is required",
            );
        }
        let project_version_arn = input.project_version_arn.as_str();
        let image = match raw.get("Image") {
            Some(v) => v.clone(),
            None => {
                return json_error_response(400, "InvalidParameterException", "Image is required");
            }
        };

        // Derive seed from project version ARN + image content
        let mut hasher = DefaultHasher::new();
        project_version_arn.hash(&mut hasher);
        image.to_string().hash(&mut hasher);
        let seed = hasher.finish();

        // Extract a label name from the project version ARN (last segment after /)
        let label_name = project_version_arn
            .rsplit('/')
            .next()
            .unwrap_or("CustomLabel");

        let confidence = seeded_f64(remix(seed, 0), 60.0, 99.0);
        let bb_width = seeded_f64(remix(seed, 1), 0.1, 0.5);
        let bb_height = seeded_f64(remix(seed, 2), 0.1, 0.5);
        let bb_left = seeded_f64(remix(seed, 3), 0.05, 0.4);
        let bb_top = seeded_f64(remix(seed, 4), 0.1, 0.6);

        wire::serialize_detect_custom_labels_response(&wire::DetectCustomLabelsResponse {
            custom_labels: serde_json::from_value(json!([{
                "Name": label_name,
                "Confidence": confidence,
                "Geometry": {
                    "BoundingBox": {
                        "Width": bb_width,
                        "Height": bb_height,
                        "Left": bb_left,
                        "Top": bb_top
                    }
                }
            }]))
            .ok(),
        })
    }

    /// StartFaceSearch - starts an async face search job and returns a job ID.
    async fn handle_start_face_search(
        &self,
        state: &Arc<tokio::sync::RwLock<RekognitionState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_start_face_search_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let raw: Value = serde_json::from_slice(body).unwrap_or(Value::Null);
        if raw.get("Video").is_none() {
            return json_error_response(400, "InvalidParameterException", "Video is required");
        }
        if input.collection_id.is_empty() {
            return json_error_response(
                400,
                "InvalidParameterException",
                "CollectionId is required",
            );
        }
        let collection_id = input.collection_id.clone();
        let job_tag = input.job_tag;

        let mut state = state.write().await;
        let job_id = state.start_video_job(VideoJobType::FaceSearch, job_tag, Some(collection_id));

        wire::serialize_start_face_search_response(&wire::StartFaceSearchResponse {
            job_id: Some(job_id),
        })
    }

    /// StartTextDetection - starts an async text detection job and returns a job ID.
    async fn handle_start_text_detection(
        &self,
        state: &Arc<tokio::sync::RwLock<RekognitionState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_start_text_detection_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let raw: Value = serde_json::from_slice(body).unwrap_or(Value::Null);
        if raw.get("Video").is_none() {
            return json_error_response(400, "InvalidParameterException", "Video is required");
        }
        let job_tag = input.job_tag;

        let mut state = state.write().await;
        let job_id = state.start_video_job(VideoJobType::TextDetection, job_tag, None);

        wire::serialize_start_text_detection_response(&wire::StartTextDetectionResponse {
            job_id: Some(job_id),
        })
    }

    /// GetFaceSearch - returns seed-based, deterministic face search results for a previously started job.
    async fn handle_get_face_search(
        &self,
        state: &Arc<tokio::sync::RwLock<RekognitionState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_face_search_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let job_id = match Some(input.job_id.as_str()).filter(|s| !s.is_empty()) {
            Some(id) => id,
            None => {
                return json_error_response(400, "InvalidParameterException", "JobId is required");
            }
        };

        let state = state.read().await;
        let job = match state.get_video_job(job_id) {
            Ok(j) => j,
            Err(e) => return rekognition_error_response(&e),
        };

        let job_tag = job.job_tag.clone().unwrap_or_default();
        let seed = job.seed;

        let duration_millis = 5000 + (remix(seed, 0) % 30000) as i64;
        let frame_rate = seeded_f64(remix(seed, 1), 15.0, 60.0);
        let frame_height = 360 + (remix(seed, 2) % 5) as i64 * 180; // 360, 540, 720, 900, 1080
        let frame_width = frame_height * 16 / 9;

        let face_confidence = seeded_f64(remix(seed, 10), 95.0, 99.999);
        let similarity = seeded_f64(remix(seed, 11), 90.0, 99.999);
        let match_confidence = seeded_f64(remix(seed, 12), 95.0, 99.999);

        let bb_w = seeded_f64(remix(seed, 20), 0.2, 0.6);
        let bb_h = seeded_f64(remix(seed, 21), 0.5, 0.95);
        let bb_l = seeded_f64(remix(seed, 22), 0.1, 0.5);
        let bb_t = seeded_f64(remix(seed, 23), -0.01, 0.1);

        let match_bb_w = seeded_f64(remix(seed, 30), 0.4, 0.8);
        let match_bb_h = seeded_f64(remix(seed, 31), 0.6, 0.95);
        let match_bb_l = seeded_f64(remix(seed, 32), 0.1, 0.4);
        let match_bb_t = seeded_f64(remix(seed, 33), -0.02, 0.05);

        // Generate deterministic UUIDs from seed
        let face_id = format!(
            "{:08x}-{:04x}-{:04x}-{:04x}-{:012x}",
            remix(seed, 40) as u32,
            (remix(seed, 41) & 0xFFFF) as u16,
            (remix(seed, 42) & 0xFFFF) as u16,
            (remix(seed, 43) & 0xFFFF) as u16,
            remix(seed, 44) & 0xFFFF_FFFF_FFFF
        );
        let image_id = format!(
            "{:08x}-{:04x}-{:04x}-{:04x}-{:012x}",
            remix(seed, 45) as u32,
            (remix(seed, 46) & 0xFFFF) as u16,
            (remix(seed, 47) & 0xFFFF) as u16,
            (remix(seed, 48) & 0xFFFF) as u16,
            remix(seed, 49) & 0xFFFF_FFFF_FFFF
        );

        wire::serialize_get_face_search_response(&wire::GetFaceSearchResponse {
            job_status: Some("SUCCEEDED".to_string()),
            status_message: Some(String::new()),
            next_token: None,
            video_metadata: serde_json::from_value(json!({
                "Codec": "h264",
                "DurationMillis": duration_millis,
                "Format": "QuickTime / MOV",
                "FrameRate": frame_rate,
                "FrameHeight": frame_height,
                "FrameWidth": frame_width,
                "ColorRange": "LIMITED"
            }))
            .ok(),
            persons: serde_json::from_value(json!([{
                "Timestamp": 0,
                "Person": {
                    "Index": 0,
                    "Face": {
                        "BoundingBox": {
                            "Width": bb_w,
                            "Height": bb_h,
                            "Left": bb_l,
                            "Top": bb_t
                        },
                        "Landmarks": [
                            {"Type": "eyeLeft", "X": seeded_f64(remix(seed, 50), 0.3, 0.5), "Y": seeded_f64(remix(seed, 51), 0.15, 0.35)},
                            {"Type": "eyeRight", "X": seeded_f64(remix(seed, 52), 0.5, 0.7), "Y": seeded_f64(remix(seed, 53), 0.15, 0.35)},
                            {"Type": "mouthLeft", "X": seeded_f64(remix(seed, 54), 0.35, 0.55), "Y": seeded_f64(remix(seed, 55), 0.5, 0.7)},
                            {"Type": "mouthRight", "X": seeded_f64(remix(seed, 56), 0.55, 0.75), "Y": seeded_f64(remix(seed, 57), 0.5, 0.7)},
                            {"Type": "nose", "X": seeded_f64(remix(seed, 58), 0.4, 0.6), "Y": seeded_f64(remix(seed, 59), 0.3, 0.5)}
                        ],
                        "Pose": {
                            "Roll": seeded_f64(remix(seed, 60), -15.0, 15.0),
                            "Yaw": seeded_f64(remix(seed, 61), -30.0, 30.0),
                            "Pitch": seeded_f64(remix(seed, 62), -15.0, 15.0)
                        },
                        "Quality": {
                            "Brightness": seeded_f64(remix(seed, 63), 60.0, 95.0),
                            "Sharpness": seeded_f64(remix(seed, 64), 50.0, 90.0)
                        },
                        "Confidence": face_confidence
                    }
                },
                "FaceMatches": [
                    {
                        "Similarity": similarity,
                        "Face": {
                            "FaceId": face_id,
                            "BoundingBox": {
                                "Width": match_bb_w,
                                "Height": match_bb_h,
                                "Left": match_bb_l,
                                "Top": match_bb_t
                            },
                            "ImageId": image_id,
                            "ExternalImageId": "Match_Person",
                            "Confidence": match_confidence
                        }
                    }
                ]
            }]))
            .ok(),
            job_id: Some(job_id.to_string()),
            job_tag: Some(job_tag),
            ..Default::default()
        })
    }

    /// GetTextDetection - returns seed-based, deterministic text detection results for a previously started job.
    async fn handle_get_text_detection(
        &self,
        state: &Arc<tokio::sync::RwLock<RekognitionState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_text_detection_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let job_id = match Some(input.job_id.as_str()).filter(|s| !s.is_empty()) {
            Some(id) => id,
            None => {
                return json_error_response(400, "InvalidParameterException", "JobId is required");
            }
        };

        let state = state.read().await;
        let job = match state.get_video_job(job_id) {
            Ok(j) => j,
            Err(e) => return rekognition_error_response(&e),
        };

        let job_tag = job.job_tag.clone().unwrap_or_default();
        let seed = job.seed;

        static VIDEO_TEXTS: &[&str] = &[
            "HELLO", "WORLD", "OPEN", "EXIT", "STOP", "WARNING", "ENTER", "WELCOME",
        ];

        let duration_millis = 5000 + (remix(seed, 0) % 30000) as i64;
        let frame_rate = seeded_f64(remix(seed, 1), 15.0, 60.0);
        let frame_height = 360 + (remix(seed, 2) % 5) as i64 * 180;
        let frame_width = frame_height * 16 / 9;

        let text_count = 1 + (remix(seed, 3) % 3) as usize; // 1-3 texts
        let selected = pick_items(seed, VIDEO_TEXTS, text_count);

        let mut detections = Vec::new();
        let mut id = 0i64;

        for (i, &&text) in selected.iter().enumerate() {
            let conf = seeded_f64(remix(seed, 100 + i as u64), 90.0, 99.9);
            let bb_left = seeded_f64(remix(seed, 200 + i as u64 * 4), 0.1, 0.7);
            let bb_top = seeded_f64(remix(seed, 201 + i as u64 * 4), 0.5, 0.95);
            let bb_width = seeded_f64(remix(seed, 202 + i as u64 * 4), 0.05, 0.2);
            let bb_height = seeded_f64(remix(seed, 203 + i as u64 * 4), 0.02, 0.05);

            let line_id = id;
            // LINE detection
            detections.push(json!({
                "Timestamp": 0,
                "TextDetection": {
                    "DetectedText": text,
                    "Type": "LINE",
                    "Id": line_id,
                    "Confidence": conf,
                    "Geometry": {
                        "BoundingBox": {
                            "Width": bb_width,
                            "Height": bb_height,
                            "Left": bb_left,
                            "Top": bb_top
                        },
                        "Polygon": [
                            {"X": bb_left, "Y": bb_top},
                            {"X": bb_left + bb_width, "Y": bb_top},
                            {"X": bb_left + bb_width, "Y": bb_top + bb_height},
                            {"X": bb_left, "Y": bb_top + bb_height}
                        ]
                    }
                }
            }));
            id += 1;

            // WORD detection
            detections.push(json!({
                "Timestamp": 0,
                "TextDetection": {
                    "DetectedText": text,
                    "Type": "WORD",
                    "Id": id,
                    "ParentId": line_id,
                    "Confidence": conf,
                    "Geometry": {
                        "BoundingBox": {
                            "Width": bb_width,
                            "Height": bb_height,
                            "Left": bb_left,
                            "Top": bb_top
                        },
                        "Polygon": [
                            {"X": bb_left, "Y": bb_top},
                            {"X": bb_left + bb_width, "Y": bb_top},
                            {"X": bb_left + bb_width, "Y": bb_top + bb_height},
                            {"X": bb_left, "Y": bb_top + bb_height}
                        ]
                    }
                }
            }));
            id += 1;
        }

        wire::serialize_get_text_detection_response(&wire::GetTextDetectionResponse {
            job_status: Some("SUCCEEDED".to_string()),
            status_message: Some(String::new()),
            video_metadata: serde_json::from_value(json!({
                "Codec": "h264",
                "DurationMillis": duration_millis,
                "Format": "QuickTime / MOV",
                "FrameRate": frame_rate,
                "FrameHeight": frame_height,
                "FrameWidth": frame_width,
                "ColorRange": "LIMITED"
            }))
            .ok(),
            text_detections: serde_json::from_value(json!(detections)).ok(),
            next_token: None,
            text_model_version: Some("3.1".to_string()),
            job_id: Some(job_id.to_string()),
            job_tag: Some(job_tag),
            ..Default::default()
        })
    }
}

fn rekognition_error_response(err: &RekognitionError) -> MockResponse {
    let (status, error_type) = match err {
        RekognitionError::ResourceAlreadyExists { .. } => (400, "ResourceAlreadyExistsException"),
        RekognitionError::CollectionNotFound { .. } => (400, "ResourceNotFoundException"),
        RekognitionError::VideoJobNotFound { .. } => (400, "ResourceNotFoundException"),
    };
    MockResponse::json(
        status,
        json!({"__type": error_type, "message": err.to_string()}).to_string(),
    )
}

fn json_error_response(status: u16, code: &str, message: &str) -> MockResponse {
    MockResponse::json(
        status,
        json!({"__type": code, "message": message}).to_string(),
    )
}
