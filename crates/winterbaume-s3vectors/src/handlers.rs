use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
    StatefulService,
};

use crate::state::{S3VectorsError, S3VectorsState};
use crate::views::S3VectorsStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct S3VectorsService {
    pub(crate) state: Arc<BackendState<S3VectorsState>>,
    pub(crate) notifier: StateChangeNotifier<S3VectorsStateView>,
}

impl S3VectorsService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for S3VectorsService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for S3VectorsService {
    fn service_name(&self) -> &str {
        "s3vectors"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://s3vectors\.(.+)\.api\.aws"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl S3VectorsService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_headers(&request.headers)
            .unwrap_or_else(|| winterbaume_core::auth::extract_region_from_uri(&request.uri));
        let account_id = DEFAULT_ACCOUNT_ID;

        let state = self.state.get(account_id, &region);
        let method = request.method.as_str();
        let path = extract_path(&request.uri);
        let query = extract_query(&request.uri);
        let segments: Vec<&str> = path
            .trim_start_matches('/')
            .split('/')
            .filter(|s| !s.is_empty())
            .collect();

        let query_map: HashMap<String, String> = winterbaume_core::parse_query_string(&query);

        let mutating = matches!(method, "POST" | "PUT" | "DELETE" | "PATCH");

        let response = match (method, segments.as_slice()) {
            // Tag operations use different routing
            ("GET", ["tags", arn]) => {
                let labels: &[(&str, &str)] = &[("resourceArn", arn)];
                self.handle_list_tags_for_resource(&state, &request, labels, &query_map)
                    .await
            }
            ("POST", ["tags", arn]) => {
                let labels: &[(&str, &str)] = &[("resourceArn", arn)];
                self.handle_tag_resource(&state, &request, labels, &query_map)
                    .await
            }
            ("DELETE", ["tags", arn]) => {
                let labels: &[(&str, &str)] = &[("resourceArn", arn)];
                self.handle_untag_resource(&state, &request, labels, &query_map)
                    .await
            }
            // All other operations are POST /OperationName
            ("POST", [op]) => {
                self.handle_operation(&state, op, &request, &query_map, account_id, &region)
                    .await
            }
            _ => rest_json_error(
                400,
                "UnknownOperationException",
                &format!("Unknown operation: {method} {path}"),
            ),
        };

        if mutating && response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }

        response
    }

    async fn handle_operation(
        &self,
        state: &Arc<tokio::sync::RwLock<S3VectorsState>>,
        op: &str,
        request: &MockRequest,
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        match op {
            "CreateVectorBucket" => {
                self.handle_create_vector_bucket(state, request, &[], query, account_id, region)
                    .await
            }
            "GetVectorBucket" => {
                self.handle_get_vector_bucket(state, request, &[], query)
                    .await
            }
            "DeleteVectorBucket" => {
                self.handle_delete_vector_bucket(state, request, &[], query)
                    .await
            }
            "ListVectorBuckets" => {
                self.handle_list_vector_buckets(state, request, &[], query)
                    .await
            }
            "CreateIndex" => {
                self.handle_create_index(state, request, &[], query, account_id, region)
                    .await
            }
            "GetIndex" => self.handle_get_index(state, request, &[], query).await,
            "DeleteIndex" => self.handle_delete_index(state, request, &[], query).await,
            "ListIndexes" => self.handle_list_indexes(state, request, &[], query).await,
            "PutVectors" => self.handle_put_vectors(state, request, &[], query).await,
            "GetVectors" => self.handle_get_vectors(state, request, &[], query).await,
            "DeleteVectors" => self.handle_delete_vectors(state, request, &[], query).await,
            "ListVectors" => self.handle_list_vectors(state, request, &[], query).await,
            "QueryVectors" => self.handle_query_vectors(state, request, &[], query).await,
            "GetVectorBucketPolicy" => {
                self.handle_get_vector_bucket_policy(state, request, &[], query)
                    .await
            }
            "PutVectorBucketPolicy" => {
                self.handle_put_vector_bucket_policy(state, request, &[], query)
                    .await
            }
            "DeleteVectorBucketPolicy" => {
                self.handle_delete_vector_bucket_policy(state, request, &[], query)
                    .await
            }
            _ => rest_json_error(
                400,
                "UnknownOperationException",
                &format!("Unknown operation: {op}"),
            ),
        }
    }

    // -------------------------------------------------------------------------
    // VectorBucket
    // -------------------------------------------------------------------------

    async fn handle_create_vector_bucket(
        &self,
        state: &Arc<tokio::sync::RwLock<S3VectorsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_vector_bucket_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.vector_bucket_name.is_empty() {
            return rest_json_error(400, "ValidationException", "vectorBucketName is required");
        }
        let tags = input.tags.clone();
        let mut s = state.write().await;
        match s.create_bucket(
            input.vector_bucket_name,
            account_id,
            region,
            chrono::Utc::now().to_rfc3339(),
            tags,
        ) {
            Ok(b) => {
                wire::serialize_create_vector_bucket_response(&wire::CreateVectorBucketOutput {
                    vector_bucket_arn: Some(b.arn),
                })
            }
            Err(e) => s3vectors_error_response(&e),
        }
    }

    async fn handle_get_vector_bucket(
        &self,
        state: &Arc<tokio::sync::RwLock<S3VectorsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_vector_bucket_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let name = input.vector_bucket_name.as_deref();
        let arn = input.vector_bucket_arn.as_deref();
        let s = state.read().await;
        match s.get_bucket(name, arn) {
            Ok(b) => wire::serialize_get_vector_bucket_response(&wire::GetVectorBucketOutput {
                vector_bucket: Some(wire::VectorBucket {
                    vector_bucket_name: Some(b.name.clone()),
                    vector_bucket_arn: Some(b.arn.clone()),
                    creation_time: Some(parse_timestamp_to_f64(&b.creation_time)),
                    ..Default::default()
                }),
            }),
            Err(e) => s3vectors_error_response(&e),
        }
    }

    async fn handle_delete_vector_bucket(
        &self,
        state: &Arc<tokio::sync::RwLock<S3VectorsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_vector_bucket_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let name = input.vector_bucket_name.as_deref();
        let arn = input.vector_bucket_arn.as_deref();
        let mut s = state.write().await;
        match s.delete_bucket(name, arn) {
            Ok(()) => {
                wire::serialize_delete_vector_bucket_response(&wire::DeleteVectorBucketOutput {})
            }
            Err(e) => s3vectors_error_response(&e),
        }
    }

    async fn handle_list_vector_buckets(
        &self,
        state: &Arc<tokio::sync::RwLock<S3VectorsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_vector_buckets_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let prefix = input.prefix.as_deref();
        let s = state.read().await;
        let buckets: Vec<wire::VectorBucketSummary> = s
            .list_buckets(prefix)
            .into_iter()
            .map(|b| wire::VectorBucketSummary {
                vector_bucket_name: Some(b.name.clone()),
                vector_bucket_arn: Some(b.arn.clone()),
                creation_time: Some(parse_timestamp_to_f64(&b.creation_time)),
            })
            .collect();
        wire::serialize_list_vector_buckets_response(&wire::ListVectorBucketsOutput {
            vector_buckets: Some(buckets),
            next_token: None,
        })
    }

    // -------------------------------------------------------------------------
    // Index
    // -------------------------------------------------------------------------

    async fn handle_create_index(
        &self,
        state: &Arc<tokio::sync::RwLock<S3VectorsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_index_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.index_name.is_empty() {
            return rest_json_error(400, "ValidationException", "indexName is required");
        }
        if input.data_type.is_empty() {
            return rest_json_error(400, "ValidationException", "dataType is required");
        }
        // Note: the wire model defaults `dimension` to 0 when missing. Treat 0 as missing
        // to preserve the previous behaviour, since dimensions must be > 0 in practice.
        if input.dimension == 0 {
            return rest_json_error(400, "ValidationException", "dimension is required");
        }
        if input.distance_metric.is_empty() {
            return rest_json_error(400, "ValidationException", "distanceMetric is required");
        }
        let bucket_name = input.vector_bucket_name.as_deref();
        let bucket_arn = input.vector_bucket_arn.as_deref();
        let non_filterable = input
            .metadata_configuration
            .as_ref()
            .map(|m| m.non_filterable_metadata_keys.clone())
            .unwrap_or_default();
        let tags = input.tags.clone();
        let creation_time = chrono::Utc::now().to_rfc3339();

        let mut s = state.write().await;
        match s.create_index(
            bucket_name,
            bucket_arn,
            input.index_name,
            input.data_type,
            input.dimension,
            input.distance_metric,
            non_filterable,
            account_id,
            region,
            creation_time,
            tags,
        ) {
            Ok(idx) => wire::serialize_create_index_response(&wire::CreateIndexOutput {
                index_arn: Some(idx.arn),
            }),
            Err(e) => s3vectors_error_response(&e),
        }
    }

    async fn handle_get_index(
        &self,
        state: &Arc<tokio::sync::RwLock<S3VectorsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_index_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let bucket_name = input.vector_bucket_name.as_deref();
        let index_name = input.index_name.as_deref();
        let index_arn = input.index_arn.as_deref();
        let s = state.read().await;
        match s.get_index(bucket_name, None, index_name, index_arn) {
            Ok(idx) => {
                let meta = if !idx.non_filterable_metadata_keys.is_empty() {
                    Some(wire::MetadataConfiguration {
                        non_filterable_metadata_keys: idx.non_filterable_metadata_keys.clone(),
                    })
                } else {
                    None
                };
                wire::serialize_get_index_response(&wire::GetIndexOutput {
                    index: Some(wire::Index {
                        vector_bucket_name: Some(idx.bucket_name.clone()),
                        index_name: Some(idx.name.clone()),
                        index_arn: Some(idx.arn.clone()),
                        creation_time: Some(parse_timestamp_to_f64(&idx.creation_time)),
                        data_type: Some(idx.data_type.clone()),
                        dimension: Some(idx.dimension),
                        distance_metric: Some(idx.distance_metric.clone()),
                        metadata_configuration: meta,
                        ..Default::default()
                    }),
                })
            }
            Err(e) => s3vectors_error_response(&e),
        }
    }

    async fn handle_delete_index(
        &self,
        state: &Arc<tokio::sync::RwLock<S3VectorsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_index_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let bucket_name = input.vector_bucket_name.as_deref();
        let index_name = input.index_name.as_deref();
        let index_arn = input.index_arn.as_deref();
        let mut s = state.write().await;
        match s.delete_index(bucket_name, None, index_name, index_arn) {
            Ok(()) => wire::serialize_delete_index_response(&wire::DeleteIndexOutput {}),
            Err(e) => s3vectors_error_response(&e),
        }
    }

    async fn handle_list_indexes(
        &self,
        state: &Arc<tokio::sync::RwLock<S3VectorsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_indexes_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let bucket_name = input.vector_bucket_name.as_deref();
        let bucket_arn = input.vector_bucket_arn.as_deref();
        let prefix = input.prefix.as_deref();
        let s = state.read().await;
        match s.list_indexes(bucket_name, bucket_arn, prefix) {
            Ok(indexes) => {
                let items: Vec<wire::IndexSummary> = indexes
                    .into_iter()
                    .map(|idx| wire::IndexSummary {
                        index_name: Some(idx.name.clone()),
                        index_arn: Some(idx.arn.clone()),
                        creation_time: Some(parse_timestamp_to_f64(&idx.creation_time)),
                        ..Default::default()
                    })
                    .collect();
                wire::serialize_list_indexes_response(&wire::ListIndexesOutput {
                    indexes: Some(items),
                    next_token: None,
                })
            }
            Err(e) => s3vectors_error_response(&e),
        }
    }

    // -------------------------------------------------------------------------
    // Vectors
    // -------------------------------------------------------------------------

    async fn handle_put_vectors(
        &self,
        state: &Arc<tokio::sync::RwLock<S3VectorsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_put_vectors_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let bucket_name = input.vector_bucket_name.as_deref();
        let index_name = input.index_name.as_deref();
        let index_arn = input.index_arn.as_deref();

        if input.vectors.is_empty() {
            return rest_json_error(400, "ValidationException", "vectors is required");
        }

        let mut vectors: Vec<(String, Vec<f32>, Option<serde_json::Value>)> = Vec::new();
        for v in input.vectors {
            if v.key.is_empty() {
                return rest_json_error(400, "ValidationException", "vector key is required");
            }
            let data = v.data.float32.unwrap_or_default();
            vectors.push((v.key, data, v.metadata));
        }

        let mut s = state.write().await;
        match s.put_vectors(bucket_name, None, index_name, index_arn, vectors) {
            Ok(()) => wire::serialize_put_vectors_response(&wire::PutVectorsOutput {}),
            Err(e) => s3vectors_error_response(&e),
        }
    }

    async fn handle_get_vectors(
        &self,
        state: &Arc<tokio::sync::RwLock<S3VectorsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_vectors_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let bucket_name = input.vector_bucket_name.as_deref();
        let index_name = input.index_name.as_deref();
        let index_arn = input.index_arn.as_deref();
        let keys = input.keys.clone();
        let return_data = input.return_data.unwrap_or(false);
        let return_metadata = input.return_metadata.unwrap_or(false);

        let s = state.read().await;
        match s.get_vectors(
            bucket_name,
            None,
            index_name,
            index_arn,
            &keys,
            return_data,
            return_metadata,
        ) {
            Ok(results) => {
                let vecs: Vec<wire::GetOutputVector> = results
                    .into_iter()
                    .map(|(key, data, metadata)| wire::GetOutputVector {
                        key: Some(key),
                        data: data.map(|d| wire::VectorData { float32: Some(d) }),
                        metadata,
                    })
                    .collect();
                wire::serialize_get_vectors_response(&wire::GetVectorsOutput {
                    vectors: Some(vecs),
                })
            }
            Err(e) => s3vectors_error_response(&e),
        }
    }

    async fn handle_delete_vectors(
        &self,
        state: &Arc<tokio::sync::RwLock<S3VectorsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_vectors_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let bucket_name = input.vector_bucket_name.as_deref();
        let index_name = input.index_name.as_deref();
        let index_arn = input.index_arn.as_deref();
        let keys = input.keys.clone();

        let mut s = state.write().await;
        match s.delete_vectors(bucket_name, None, index_name, index_arn, &keys) {
            Ok(()) => wire::serialize_delete_vectors_response(&wire::DeleteVectorsOutput {}),
            Err(e) => s3vectors_error_response(&e),
        }
    }

    async fn handle_list_vectors(
        &self,
        state: &Arc<tokio::sync::RwLock<S3VectorsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_vectors_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let bucket_name = input.vector_bucket_name.as_deref();
        let index_name = input.index_name.as_deref();
        let index_arn = input.index_arn.as_deref();
        let max_results = input.max_results;
        let return_data = input.return_data.unwrap_or(false);
        let return_metadata = input.return_metadata.unwrap_or(false);

        let s = state.read().await;
        match s.list_vectors(
            bucket_name,
            None,
            index_name,
            index_arn,
            max_results,
            return_data,
            return_metadata,
        ) {
            Ok((results, next_token)) => {
                let vecs: Vec<wire::ListOutputVector> = results
                    .into_iter()
                    .map(|(key, data, metadata)| wire::ListOutputVector {
                        key: Some(key),
                        data: data.map(|d| wire::VectorData { float32: Some(d) }),
                        metadata,
                    })
                    .collect();
                wire::serialize_list_vectors_response(&wire::ListVectorsOutput {
                    vectors: Some(vecs),
                    next_token,
                })
            }
            Err(e) => s3vectors_error_response(&e),
        }
    }

    async fn handle_query_vectors(
        &self,
        state: &Arc<tokio::sync::RwLock<S3VectorsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_query_vectors_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let bucket_name = input.vector_bucket_name.as_deref();
        let index_name = input.index_name.as_deref();
        let index_arn = input.index_arn.as_deref();
        // Note: the wire model defaults `top_k` to 0 when missing. Treat 0 as missing
        // to preserve the previous behaviour, since topK must be > 0 in practice.
        if input.top_k == 0 {
            return rest_json_error(400, "ValidationException", "topK is required");
        }
        let top_k = input.top_k;
        let query_vec = input.query_vector.float32.unwrap_or_default();
        let return_metadata = input.return_metadata.unwrap_or(false);
        let return_distance = input.return_distance.unwrap_or(false);

        let s = state.read().await;
        match s.query_vectors(
            bucket_name,
            None,
            index_name,
            index_arn,
            top_k,
            &query_vec,
            return_metadata,
            return_distance,
        ) {
            Ok((results, distance_metric)) => {
                let vecs: Vec<wire::QueryOutputVector> = results
                    .into_iter()
                    .map(|(key, dist, metadata)| wire::QueryOutputVector {
                        key: Some(key),
                        distance: if return_distance { Some(dist) } else { None },
                        metadata,
                    })
                    .collect();
                wire::serialize_query_vectors_response(&wire::QueryVectorsOutput {
                    vectors: Some(vecs),
                    distance_metric: Some(distance_metric),
                })
            }
            Err(e) => s3vectors_error_response(&e),
        }
    }

    // -------------------------------------------------------------------------
    // Policy
    // -------------------------------------------------------------------------

    async fn handle_get_vector_bucket_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<S3VectorsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_vector_bucket_policy_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let name = input.vector_bucket_name.as_deref();
        let arn = input.vector_bucket_arn.as_deref();
        let s = state.read().await;
        match s.get_bucket_policy(name, arn) {
            Ok(p) => wire::serialize_get_vector_bucket_policy_response(
                &wire::GetVectorBucketPolicyOutput {
                    policy: Some(p.policy.clone()),
                },
            ),
            Err(e) => s3vectors_error_response(&e),
        }
    }

    async fn handle_put_vector_bucket_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<S3VectorsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_put_vector_bucket_policy_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let name = input.vector_bucket_name.as_deref();
        let arn = input.vector_bucket_arn.as_deref();
        if input.policy.is_empty() {
            return rest_json_error(400, "ValidationException", "policy is required");
        }
        let mut s = state.write().await;
        match s.put_bucket_policy(name, arn, input.policy) {
            Ok(()) => wire::serialize_put_vector_bucket_policy_response(
                &wire::PutVectorBucketPolicyOutput {},
            ),
            Err(e) => s3vectors_error_response(&e),
        }
    }

    async fn handle_delete_vector_bucket_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<S3VectorsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_delete_vector_bucket_policy_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let name = input.vector_bucket_name.as_deref();
        let arn = input.vector_bucket_arn.as_deref();
        let mut s = state.write().await;
        match s.delete_bucket_policy(name, arn) {
            Ok(()) => wire::serialize_delete_vector_bucket_policy_response(
                &wire::DeleteVectorBucketPolicyOutput {},
            ),
            Err(e) => s3vectors_error_response(&e),
        }
    }

    // -------------------------------------------------------------------------
    // Tags
    // -------------------------------------------------------------------------

    async fn handle_list_tags_for_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<S3VectorsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_tags_for_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let s = state.read().await;
        let tags = s.list_tags(&input.resource_arn);
        wire::serialize_list_tags_for_resource_response(&wire::ListTagsForResourceOutput {
            tags: Some(tags),
        })
    }

    async fn handle_tag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<S3VectorsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_tag_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut s = state.write().await;
        s.tag_resource(&input.resource_arn, input.tags);
        wire::serialize_tag_resource_response(&wire::TagResourceOutput {})
    }

    async fn handle_untag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<S3VectorsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_untag_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut s = state.write().await;
        s.untag_resource(&input.resource_arn, &input.tag_keys);
        wire::serialize_untag_resource_response(&wire::UntagResourceOutput {})
    }
}

fn s3vectors_error_response(err: &S3VectorsError) -> MockResponse {
    let (status, error_type) = match err {
        S3VectorsError::BucketIdentifierRequired => (400, "ValidationException"),
        S3VectorsError::IndexIdentifierRequired => (400, "ValidationException"),
        S3VectorsError::BucketAlreadyExists(_) => (409, "ConflictException"),
        S3VectorsError::BucketNotEmpty(_) => (409, "ConflictException"),
        S3VectorsError::BucketNotFound(_) => (404, "NotFoundException"),
        S3VectorsError::IndexAlreadyExists(_, _) => (409, "ConflictException"),
        S3VectorsError::IndexNotFound(_, _) => (404, "NotFoundException"),
        S3VectorsError::PolicyNotFound(_) => (404, "NotFoundException"),
    };
    rest_json_error(status, error_type, &err.to_string())
}

fn rest_json_error(status: u16, code: &str, message: &str) -> MockResponse {
    let body = serde_json::json!({
        "message": message,
    });
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers.insert(X_AMZN_ERRORTYPE, code.parse().unwrap());
    resp
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

fn extract_query(uri: &str) -> String {
    if let Some(idx) = uri.find('?') {
        uri[idx + 1..].to_string()
    } else {
        String::new()
    }
}

fn parse_timestamp_to_f64(ts: &str) -> f64 {
    chrono::DateTime::parse_from_rfc3339(ts)
        .map(|dt| dt.timestamp() as f64)
        .unwrap_or(0.0)
}
