use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
    StatefulService,
};

use crate::model;
use crate::state::{S3TablesError, S3TablesState};
use crate::views::S3TablesStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct S3TablesService {
    pub(crate) state: Arc<BackendState<S3TablesState>>,
    pub(crate) notifier: StateChangeNotifier<S3TablesStateView>,
}

impl S3TablesService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for S3TablesService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for S3TablesService {
    fn service_name(&self) -> &str {
        "s3tables"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://s3tables\..+\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl S3TablesService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = DEFAULT_ACCOUNT_ID;
        let state = self.state.get(account_id, &region);

        let path = extract_path(&request.uri);
        let method = request.method.as_str();
        let segments: Vec<&str> = path.trim_start_matches('/').split('/').collect();

        // Validate body is well-formed JSON when present.
        if !request.body.is_empty() && serde_json::from_slice::<Value>(&request.body).is_err() {
            return rest_json_error(400, "BadRequestException", "Invalid JSON body");
        }
        let query_string = winterbaume_core::extract_query_string(&request.uri).to_string();
        let query_map: HashMap<String, String> =
            winterbaume_core::parse_query_string(&query_string);

        #[allow(clippy::result_large_err)]
        let parse_json_body = |body: &[u8]| -> Result<Value, MockResponse> {
            if body.is_empty() {
                return Ok(json!({}));
            }
            serde_json::from_slice(body)
                .map_err(|_| rest_json_error(400, "BadRequestException", "Invalid JSON body"))
        };

        let response = match (method, segments.as_slice()) {
            // --- Table Bucket operations ---
            // PUT /buckets - CreateTableBucket
            ("PUT", ["buckets"]) => {
                let labels: &[(&str, &str)] = &[];
                self.handle_create_table_bucket(
                    &state, &request, labels, &query_map, account_id, &region,
                )
                .await
            }
            // GET /buckets - ListTableBuckets
            ("GET", ["buckets"]) => {
                let prefix = extract_query_param(&request.uri, "prefix");
                self.handle_list_table_buckets(&state, prefix.as_deref())
                    .await
            }
            // GET /buckets/{tableBucketARN} - GetTableBucket
            ("GET", ["buckets", arn]) => {
                let arn = percent_decode(arn);
                self.handle_get_table_bucket(&state, &arn).await
            }
            // DELETE /buckets/{tableBucketARN} - DeleteTableBucket
            ("DELETE", ["buckets", arn]) => {
                let arn = percent_decode(arn);
                self.handle_delete_table_bucket(&state, &arn).await
            }

            // --- Namespace operations ---
            // PUT /namespaces/{tableBucketARN} - CreateNamespace
            ("PUT", ["namespaces", arn]) => {
                let arn = percent_decode(arn);
                let labels: &[(&str, &str)] = &[("tableBucketARN", arn.as_str())];
                self.handle_create_namespace(&state, &request, labels, &query_map, account_id)
                    .await
            }
            // GET /namespaces/{tableBucketARN} - ListNamespaces
            ("GET", ["namespaces", arn]) => {
                let arn = percent_decode(arn);
                let prefix = extract_query_param(&request.uri, "prefix");
                self.handle_list_namespaces(&state, &arn, prefix.as_deref())
                    .await
            }
            // GET /namespaces/{tableBucketARN}/{namespace} - GetNamespace
            ("GET", ["namespaces", arn, ns]) => {
                let arn = percent_decode(arn);
                let ns = percent_decode(ns);
                self.handle_get_namespace(&state, &arn, &ns).await
            }
            // DELETE /namespaces/{tableBucketARN}/{namespace} - DeleteNamespace
            ("DELETE", ["namespaces", arn, ns]) => {
                let arn = percent_decode(arn);
                let ns = percent_decode(ns);
                self.handle_delete_namespace(&state, &arn, &ns).await
            }

            // --- Table operations ---
            // PUT /tables/{tableBucketARN}/{namespace} - CreateTable
            ("PUT", ["tables", arn, ns]) => {
                let arn = percent_decode(arn);
                let ns = percent_decode(ns);
                let labels: &[(&str, &str)] =
                    &[("tableBucketARN", arn.as_str()), ("namespace", ns.as_str())];
                self.handle_create_table(&state, &request, labels, &query_map, account_id, &region)
                    .await
            }
            // GET /tables/{tableBucketARN} - ListTables
            ("GET", ["tables", arn]) => {
                let arn = percent_decode(arn);
                let namespace = extract_query_param(&request.uri, "namespace");
                let prefix = extract_query_param(&request.uri, "prefix");
                self.handle_list_tables(&state, &arn, namespace.as_deref(), prefix.as_deref())
                    .await
            }
            // GET /get-table - GetTable
            ("GET", ["get-table"]) => {
                let bucket_arn = extract_query_param(&request.uri, "tableBucketARN");
                let namespace = extract_query_param(&request.uri, "namespace");
                let name = extract_query_param(&request.uri, "name");
                let table_arn = extract_query_param(&request.uri, "tableArn");
                self.handle_get_table(
                    &state,
                    bucket_arn.as_deref(),
                    namespace.as_deref(),
                    name.as_deref(),
                    table_arn.as_deref(),
                )
                .await
            }
            // DELETE /tables/{tableBucketARN}/{namespace}/{name} - DeleteTable
            ("DELETE", ["tables", arn, ns, name]) => {
                let arn = percent_decode(arn);
                let ns = percent_decode(ns);
                let name = percent_decode(name);
                self.handle_delete_table(&state, &arn, &ns, &name).await
            }
            // DELETE /tables/{tableBucketARN}/{namespace}/{name}/policy - DeleteTablePolicy
            ("DELETE", ["tables", arn, ns, name, "policy"]) => {
                let arn = percent_decode(arn);
                let ns = percent_decode(ns);
                let name = percent_decode(name);
                self.handle_delete_table_policy(&state, &arn, &ns, &name)
                    .await
            }
            // GET /tables/{tableBucketARN}/{namespace}/{name}/encryption - GetTableEncryption
            ("GET", ["tables", arn, ns, name, "encryption"]) => {
                let arn = percent_decode(arn);
                let ns = percent_decode(ns);
                let name = percent_decode(name);
                self.handle_get_table_encryption(&state, &arn, &ns, &name)
                    .await
            }
            // GET /tables/{tableBucketARN}/{namespace}/{name}/maintenance - GetTableMaintenanceConfiguration
            ("GET", ["tables", arn, ns, name, "maintenance"]) => {
                let arn = percent_decode(arn);
                let ns = percent_decode(ns);
                let name = percent_decode(name);
                self.handle_get_table_maintenance_configuration(&state, &arn, &ns, &name)
                    .await
            }
            // GET /tables/{tableBucketARN}/{namespace}/{name}/maintenance-job-status - GetTableMaintenanceJobStatus
            ("GET", ["tables", arn, ns, name, "maintenance-job-status"]) => {
                let arn = percent_decode(arn);
                let ns = percent_decode(ns);
                let name = percent_decode(name);
                self.handle_get_table_maintenance_job_status(&state, &arn, &ns, &name)
                    .await
            }
            // GET /tables/{tableBucketARN}/{namespace}/{name}/metadata-location - GetTableMetadataLocation
            ("GET", ["tables", arn, ns, name, "metadata-location"]) => {
                let arn = percent_decode(arn);
                let ns = percent_decode(ns);
                let name = percent_decode(name);
                self.handle_get_table_metadata_location(&state, &arn, &ns, &name)
                    .await
            }
            // GET /tables/{tableBucketARN}/{namespace}/{name}/policy - GetTablePolicy
            ("GET", ["tables", arn, ns, name, "policy"]) => {
                let arn = percent_decode(arn);
                let ns = percent_decode(ns);
                let name = percent_decode(name);
                self.handle_get_table_policy(&state, &arn, &ns, &name).await
            }
            // GET /tables/{tableBucketARN}/{namespace}/{name}/storage-class - GetTableStorageClass
            ("GET", ["tables", arn, ns, name, "storage-class"]) => {
                let arn = percent_decode(arn);
                let ns = percent_decode(ns);
                let name = percent_decode(name);
                self.handle_get_table_storage_class(&state, &arn, &ns, &name)
                    .await
            }
            // PUT /tables/{tableBucketARN}/{namespace}/{name}/maintenance/{type} - PutTableMaintenanceConfiguration
            ("PUT", ["tables", arn, ns, name, "maintenance", type_key]) => {
                let arn = percent_decode(arn);
                let ns = percent_decode(ns);
                let name = percent_decode(name);
                let type_key = percent_decode(type_key);
                let labels: &[(&str, &str)] = &[
                    ("tableBucketARN", arn.as_str()),
                    ("namespace", ns.as_str()),
                    ("name", name.as_str()),
                    ("type", type_key.as_str()),
                ];
                self.handle_put_table_maintenance_configuration(
                    &state, &request, labels, &query_map,
                )
                .await
            }
            // PUT /tables/{tableBucketARN}/{namespace}/{name}/policy - PutTablePolicy
            ("PUT", ["tables", arn, ns, name, "policy"]) => {
                let arn = percent_decode(arn);
                let ns = percent_decode(ns);
                let name = percent_decode(name);
                let labels: &[(&str, &str)] = &[
                    ("tableBucketARN", arn.as_str()),
                    ("namespace", ns.as_str()),
                    ("name", name.as_str()),
                ];
                self.handle_put_table_policy(&state, &request, labels, &query_map)
                    .await
            }
            // PUT /tables/{tableBucketARN}/{namespace}/{name}/rename - RenameTable
            ("PUT", ["tables", arn, ns, name, "rename"]) => {
                let arn = percent_decode(arn);
                let ns = percent_decode(ns);
                let name = percent_decode(name);
                let labels: &[(&str, &str)] = &[
                    ("tableBucketARN", arn.as_str()),
                    ("namespace", ns.as_str()),
                    ("name", name.as_str()),
                ];
                self.handle_rename_table(&state, &request, labels, &query_map)
                    .await
            }
            // PUT /tables/{tableBucketARN}/{namespace}/{name}/metadata-location - UpdateTableMetadataLocation
            ("PUT", ["tables", arn, ns, name, "metadata-location"]) => {
                let arn = percent_decode(arn);
                let ns = percent_decode(ns);
                let name = percent_decode(name);
                let labels: &[(&str, &str)] = &[
                    ("tableBucketARN", arn.as_str()),
                    ("namespace", ns.as_str()),
                    ("name", name.as_str()),
                ];
                self.handle_update_table_metadata_location(&state, &request, labels, &query_map)
                    .await
            }

            // --- Bucket extended operations ---
            // DELETE /buckets/{tableBucketARN}/encryption - DeleteTableBucketEncryption
            ("DELETE", ["buckets", arn, "encryption"]) => {
                let arn = percent_decode(arn);
                self.handle_delete_table_bucket_encryption(&state, &arn)
                    .await
            }
            // DELETE /buckets/{tableBucketARN}/metrics - DeleteTableBucketMetricsConfiguration
            ("DELETE", ["buckets", arn, "metrics"]) => {
                let arn = percent_decode(arn);
                self.handle_delete_table_bucket_metrics_configuration(&state, &arn)
                    .await
            }
            // DELETE /buckets/{tableBucketARN}/policy - DeleteTableBucketPolicy
            ("DELETE", ["buckets", arn, "policy"]) => {
                let arn = percent_decode(arn);
                self.handle_delete_table_bucket_policy(&state, &arn).await
            }
            // GET /buckets/{tableBucketARN}/encryption - GetTableBucketEncryption
            ("GET", ["buckets", arn, "encryption"]) => {
                let arn = percent_decode(arn);
                self.handle_get_table_bucket_encryption(&state, &arn).await
            }
            // GET /buckets/{tableBucketARN}/maintenance - GetTableBucketMaintenanceConfiguration
            ("GET", ["buckets", arn, "maintenance"]) => {
                let arn = percent_decode(arn);
                self.handle_get_table_bucket_maintenance_configuration(&state, &arn)
                    .await
            }
            // GET /buckets/{tableBucketARN}/metrics - GetTableBucketMetricsConfiguration
            ("GET", ["buckets", arn, "metrics"]) => {
                let arn = percent_decode(arn);
                self.handle_get_table_bucket_metrics_configuration(&state, &arn)
                    .await
            }
            // GET /buckets/{tableBucketARN}/policy - GetTableBucketPolicy
            ("GET", ["buckets", arn, "policy"]) => {
                let arn = percent_decode(arn);
                self.handle_get_table_bucket_policy(&state, &arn).await
            }
            // GET /buckets/{tableBucketARN}/storage-class - GetTableBucketStorageClass
            ("GET", ["buckets", arn, "storage-class"]) => {
                let arn = percent_decode(arn);
                self.handle_get_table_bucket_storage_class(&state, &arn)
                    .await
            }
            // PUT /buckets/{tableBucketARN}/encryption - PutTableBucketEncryption
            ("PUT", ["buckets", arn, "encryption"]) => {
                let arn = percent_decode(arn);
                let labels: &[(&str, &str)] = &[("tableBucketARN", arn.as_str())];
                self.handle_put_table_bucket_encryption(&state, &request, labels, &query_map)
                    .await
            }
            // PUT /buckets/{tableBucketARN}/maintenance/{type} - PutTableBucketMaintenanceConfiguration
            ("PUT", ["buckets", arn, "maintenance", type_key]) => {
                let arn = percent_decode(arn);
                let type_key = percent_decode(type_key);
                let labels: &[(&str, &str)] = &[
                    ("tableBucketARN", arn.as_str()),
                    ("type", type_key.as_str()),
                ];
                self.handle_put_table_bucket_maintenance_configuration(
                    &state, &request, labels, &query_map,
                )
                .await
            }
            // PUT /buckets/{tableBucketARN}/metrics - PutTableBucketMetricsConfiguration
            ("PUT", ["buckets", arn, "metrics"]) => {
                let arn = percent_decode(arn);
                let body = match parse_json_body(&request.body) {
                    Ok(v) => v,
                    Err(e) => return e,
                };
                self.handle_put_table_bucket_metrics_configuration(&state, &arn, &body)
                    .await
            }
            // PUT /buckets/{tableBucketARN}/policy - PutTableBucketPolicy
            ("PUT", ["buckets", arn, "policy"]) => {
                let arn = percent_decode(arn);
                let labels: &[(&str, &str)] = &[("tableBucketARN", arn.as_str())];
                self.handle_put_table_bucket_policy(&state, &request, labels, &query_map)
                    .await
            }
            // PUT /buckets/{tableBucketARN}/storage-class - PutTableBucketStorageClass
            ("PUT", ["buckets", arn, "storage-class"]) => {
                let arn = percent_decode(arn);
                let labels: &[(&str, &str)] = &[("tableBucketARN", arn.as_str())];
                self.handle_put_table_bucket_storage_class(&state, &request, labels, &query_map)
                    .await
            }

            // --- Query-param-based operations ---
            // DELETE /table-bucket-replication - DeleteTableBucketReplication
            ("DELETE", ["table-bucket-replication"]) => {
                let arn = extract_query_param(&request.uri, "tableBucketARN");
                match arn {
                    Some(a) => {
                        self.handle_delete_table_bucket_replication(&state, &a)
                            .await
                    }
                    None => rest_json_error(400, "ValidationException", "Missing tableBucketARN"),
                }
            }
            // DELETE /table-replication - DeleteTableReplication
            ("DELETE", ["table-replication"]) => {
                let table_arn = extract_query_param(&request.uri, "tableArn");
                match table_arn {
                    Some(a) => self.handle_delete_table_replication(&state, &a).await,
                    None => rest_json_error(400, "ValidationException", "Missing tableArn"),
                }
            }
            // GET /table-bucket-replication - GetTableBucketReplication
            ("GET", ["table-bucket-replication"]) => {
                let arn = extract_query_param(&request.uri, "tableBucketARN");
                match arn {
                    Some(a) => self.handle_get_table_bucket_replication(&state, &a).await,
                    None => rest_json_error(400, "ValidationException", "Missing tableBucketARN"),
                }
            }
            // PUT /table-bucket-replication - PutTableBucketReplication
            ("PUT", ["table-bucket-replication"]) => {
                let arn = extract_query_param(&request.uri, "tableBucketARN");
                let body = match parse_json_body(&request.body) {
                    Ok(v) => v,
                    Err(e) => return e,
                };
                match arn {
                    Some(a) => {
                        self.handle_put_table_bucket_replication(&state, &a, &body)
                            .await
                    }
                    None => rest_json_error(400, "ValidationException", "Missing tableBucketARN"),
                }
            }
            // GET /table-replication - GetTableReplication
            ("GET", ["table-replication"]) => {
                let table_arn = extract_query_param(&request.uri, "tableArn");
                match table_arn {
                    Some(a) => self.handle_get_table_replication(&state, &a).await,
                    None => rest_json_error(400, "ValidationException", "Missing tableArn"),
                }
            }
            // PUT /table-replication - PutTableReplication
            ("PUT", ["table-replication"]) => {
                let table_arn = extract_query_param(&request.uri, "tableArn");
                let body = match parse_json_body(&request.body) {
                    Ok(v) => v,
                    Err(e) => return e,
                };
                match table_arn {
                    Some(a) => self.handle_put_table_replication(&state, &a, &body).await,
                    None => rest_json_error(400, "ValidationException", "Missing tableArn"),
                }
            }
            // GET /table-record-expiration - GetTableRecordExpirationConfiguration
            ("GET", ["table-record-expiration"]) => {
                let table_arn = extract_query_param(&request.uri, "tableArn");
                match table_arn {
                    Some(a) => {
                        self.handle_get_table_record_expiration_configuration(&state, &a)
                            .await
                    }
                    None => rest_json_error(400, "ValidationException", "Missing tableArn"),
                }
            }
            // PUT /table-record-expiration - PutTableRecordExpirationConfiguration
            ("PUT", ["table-record-expiration"]) => {
                let table_arn = extract_query_param(&request.uri, "tableArn");
                let body = match parse_json_body(&request.body) {
                    Ok(v) => v,
                    Err(e) => return e,
                };
                match table_arn {
                    Some(a) => {
                        self.handle_put_table_record_expiration_configuration(&state, &a, &body)
                            .await
                    }
                    None => rest_json_error(400, "ValidationException", "Missing tableArn"),
                }
            }
            // GET /table-record-expiration-job-status - GetTableRecordExpirationJobStatus
            ("GET", ["table-record-expiration-job-status"]) => {
                let table_arn = extract_query_param(&request.uri, "tableArn");
                match table_arn {
                    Some(a) => {
                        self.handle_get_table_record_expiration_job_status(&state, &a)
                            .await
                    }
                    None => rest_json_error(400, "ValidationException", "Missing tableArn"),
                }
            }
            // GET /replication-status - GetTableReplicationStatus
            ("GET", ["replication-status"]) => {
                let table_arn = extract_query_param(&request.uri, "tableArn");
                match table_arn {
                    Some(a) => self.handle_get_table_replication_status(&state, &a).await,
                    None => rest_json_error(400, "ValidationException", "Missing tableArn"),
                }
            }

            // --- Tag operations ---
            // GET /tag/{resourceArn} - ListTagsForResource
            ("GET", ["tag", resource_arn]) => {
                let resource_arn = percent_decode(resource_arn);
                self.handle_list_tags_for_resource(&state, &resource_arn)
                    .await
            }
            // POST /tag/{resourceArn} - TagResource
            ("POST", ["tag", resource_arn]) => {
                let resource_arn = percent_decode(resource_arn);
                let labels: &[(&str, &str)] = &[("resourceArn", resource_arn.as_str())];
                self.handle_tag_resource(&state, &request, labels, &query_map)
                    .await
            }
            // DELETE /tag/{resourceArn} - UntagResource
            ("DELETE", ["tag", resource_arn]) => {
                let resource_arn = percent_decode(resource_arn);
                let keys = extract_query_param_multi(&request.uri, "tagKeys");
                self.handle_untag_resource(&state, &resource_arn, keys)
                    .await
            }

            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        };

        if response.status >= 200 && response.status < 300 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    // --- Table Bucket handlers ---

    async fn handle_create_table_bucket(
        &self,
        state: &Arc<tokio::sync::RwLock<S3TablesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_table_bucket_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'name'");
        }
        let name = input.name.as_str();
        let tags = input.tags.unwrap_or_default();
        let mut st = state.write().await;
        match st.create_table_bucket(name, account_id, region, tags) {
            Ok(arn) => {
                wire::serialize_create_table_bucket_response(&model::CreateTableBucketResponse {
                    arn: Some(arn),
                })
            }
            Err(e) => s3tables_error_response(&e),
        }
    }

    async fn handle_get_table_bucket(
        &self,
        state: &Arc<tokio::sync::RwLock<S3TablesState>>,
        arn: &str,
    ) -> MockResponse {
        let st = state.read().await;
        match st.get_table_bucket(arn) {
            Ok(bucket) => {
                wire::serialize_get_table_bucket_response(&model::GetTableBucketResponse {
                    arn: Some(bucket.arn.clone()),
                    name: Some(bucket.name.clone()),
                    owner_account_id: Some(bucket.owner_account_id.clone()),
                    created_at: Some(bucket.created_at.to_rfc3339()),
                    ..Default::default()
                })
            }
            Err(e) => s3tables_error_response(&e),
        }
    }

    async fn handle_delete_table_bucket(
        &self,
        state: &Arc<tokio::sync::RwLock<S3TablesState>>,
        arn: &str,
    ) -> MockResponse {
        let mut st = state.write().await;
        match st.delete_table_bucket(arn) {
            Ok(()) => wire::serialize_delete_table_bucket_response(),
            Err(e) => s3tables_error_response(&e),
        }
    }

    async fn handle_list_table_buckets(
        &self,
        state: &Arc<tokio::sync::RwLock<S3TablesState>>,
        prefix: Option<&str>,
    ) -> MockResponse {
        let st = state.read().await;
        let buckets = st.list_table_buckets(prefix);
        let summaries: Vec<model::TableBucketSummary> = buckets
            .iter()
            .map(|b| model::TableBucketSummary {
                arn: Some(b.arn.clone()),
                name: Some(b.name.clone()),
                owner_account_id: Some(b.owner_account_id.clone()),
                created_at: Some(b.created_at.to_rfc3339()),
                ..Default::default()
            })
            .collect();
        wire::serialize_list_table_buckets_response(&model::ListTableBucketsResponse {
            table_buckets: Some(summaries),
            ..Default::default()
        })
    }

    // --- Namespace handlers ---

    async fn handle_create_namespace(
        &self,
        state: &Arc<tokio::sync::RwLock<S3TablesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_namespace_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let table_bucket_arn = input.table_bucket_a_r_n.clone();
        if input.namespace.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing or invalid 'namespace'");
        }
        let namespace_parts: Vec<String> = input.namespace;
        let mut st = state.write().await;
        match st.create_namespace(&table_bucket_arn, namespace_parts.clone(), account_id) {
            Ok(()) => wire::serialize_create_namespace_response(&model::CreateNamespaceResponse {
                table_bucket_a_r_n: Some(table_bucket_arn),
                namespace: Some(namespace_parts),
            }),
            Err(e) => s3tables_error_response(&e),
        }
    }

    async fn handle_get_namespace(
        &self,
        state: &Arc<tokio::sync::RwLock<S3TablesState>>,
        table_bucket_arn: &str,
        namespace: &str,
    ) -> MockResponse {
        let st = state.read().await;
        match st.get_namespace(table_bucket_arn, namespace) {
            Ok(ns) => wire::serialize_get_namespace_response(&model::GetNamespaceResponse {
                namespace: Some(ns.namespace.clone()),
                owner_account_id: Some(ns.owner_account_id.clone()),
                created_at: Some(ns.created_at.to_rfc3339()),
                created_by: Some(ns.created_by.clone()),
                ..Default::default()
            }),
            Err(e) => s3tables_error_response(&e),
        }
    }

    async fn handle_delete_namespace(
        &self,
        state: &Arc<tokio::sync::RwLock<S3TablesState>>,
        table_bucket_arn: &str,
        namespace: &str,
    ) -> MockResponse {
        let mut st = state.write().await;
        match st.delete_namespace(table_bucket_arn, namespace) {
            Ok(()) => wire::serialize_delete_namespace_response(),
            Err(e) => s3tables_error_response(&e),
        }
    }

    async fn handle_list_namespaces(
        &self,
        state: &Arc<tokio::sync::RwLock<S3TablesState>>,
        table_bucket_arn: &str,
        prefix: Option<&str>,
    ) -> MockResponse {
        let st = state.read().await;
        let namespaces = st.list_namespaces(table_bucket_arn, prefix);
        let summaries: Vec<model::NamespaceSummary> = namespaces
            .iter()
            .map(|ns| model::NamespaceSummary {
                namespace: Some(ns.namespace.clone()),
                owner_account_id: Some(ns.owner_account_id.clone()),
                created_at: Some(ns.created_at.to_rfc3339()),
                created_by: Some(ns.created_by.clone()),
                ..Default::default()
            })
            .collect();
        wire::serialize_list_namespaces_response(&model::ListNamespacesResponse {
            namespaces: Some(summaries),
            ..Default::default()
        })
    }

    // --- Table handlers ---

    async fn handle_create_table(
        &self,
        state: &Arc<tokio::sync::RwLock<S3TablesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_table_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'name'");
        }
        if input.format.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'format'");
        }
        let table_bucket_arn = input.table_bucket_a_r_n.clone();
        let namespace = input.namespace.clone();
        let name = input.name.as_str();
        let format = input.format.as_str();
        let tags = input.tags.unwrap_or_default();
        let mut st = state.write().await;
        match st.create_table(
            &table_bucket_arn,
            &namespace,
            name,
            format,
            account_id,
            region,
            tags,
        ) {
            Ok((table_arn, version_token)) => {
                wire::serialize_create_table_response(&model::CreateTableResponse {
                    table_a_r_n: Some(table_arn),
                    version_token: Some(version_token),
                })
            }
            Err(e) => s3tables_error_response(&e),
        }
    }

    async fn handle_get_table(
        &self,
        state: &Arc<tokio::sync::RwLock<S3TablesState>>,
        table_bucket_arn: Option<&str>,
        namespace: Option<&str>,
        name: Option<&str>,
        table_arn: Option<&str>,
    ) -> MockResponse {
        let st = state.read().await;
        let table = if let Some(arn) = table_arn {
            st.get_table_by_arn(arn)
        } else {
            match (table_bucket_arn, namespace, name) {
                (Some(bucket_arn), Some(ns), Some(tbl_name)) => {
                    st.get_table(bucket_arn, ns, tbl_name)
                }
                _ => {
                    return rest_json_error(
                        400,
                        "ValidationException",
                        "Must provide either tableArn or tableBucketARN+namespace+name",
                    );
                }
            }
        };

        match table {
            Ok(t) => {
                let ns_parts: Vec<String> = t.namespace.split('.').map(|s| s.to_string()).collect();
                wire::serialize_get_table_response(&model::GetTableResponse {
                    name: Some(t.name.clone()),
                    r#type: Some("customer".to_string()),
                    table_a_r_n: Some(t.arn.clone()),
                    namespace: Some(ns_parts),
                    version_token: Some(t.version_token.clone()),
                    warehouse_location: Some(t.warehouse_location.clone()),
                    created_at: Some(t.created_at.to_rfc3339()),
                    created_by: Some(t.created_by.clone()),
                    modified_at: Some(t.modified_at.to_rfc3339()),
                    modified_by: Some(t.modified_by.clone()),
                    owner_account_id: Some(t.owner_account_id.clone()),
                    format: Some(t.format.clone()),
                    ..Default::default()
                })
            }
            Err(e) => s3tables_error_response(&e),
        }
    }

    async fn handle_delete_table(
        &self,
        state: &Arc<tokio::sync::RwLock<S3TablesState>>,
        table_bucket_arn: &str,
        namespace: &str,
        name: &str,
    ) -> MockResponse {
        let mut st = state.write().await;
        match st.delete_table(table_bucket_arn, namespace, name) {
            Ok(()) => wire::serialize_delete_table_response(),
            Err(e) => s3tables_error_response(&e),
        }
    }

    async fn handle_list_tables(
        &self,
        state: &Arc<tokio::sync::RwLock<S3TablesState>>,
        table_bucket_arn: &str,
        namespace: Option<&str>,
        prefix: Option<&str>,
    ) -> MockResponse {
        let st = state.read().await;
        let tables = st.list_tables(table_bucket_arn, namespace, prefix);
        let summaries: Vec<model::TableSummary> = tables
            .iter()
            .map(|t| {
                let ns_parts: Vec<String> = t.namespace.split('.').map(|s| s.to_string()).collect();
                model::TableSummary {
                    name: Some(t.name.clone()),
                    r#type: Some("customer".to_string()),
                    table_a_r_n: Some(t.arn.clone()),
                    namespace: Some(ns_parts),
                    created_at: Some(t.created_at.to_rfc3339()),
                    modified_at: Some(t.modified_at.to_rfc3339()),
                    ..Default::default()
                }
            })
            .collect();
        wire::serialize_list_tables_response(&model::ListTablesResponse {
            tables: Some(summaries),
            ..Default::default()
        })
    }

    // --- Tag handlers ---

    async fn handle_list_tags_for_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<S3TablesState>>,
        resource_arn: &str,
    ) -> MockResponse {
        let st = state.read().await;
        let tags = st.list_tags(resource_arn);
        wire::serialize_list_tags_for_resource_response(&model::ListTagsForResourceResponse {
            tags: if tags.is_empty() { None } else { Some(tags) },
        })
    }

    async fn handle_tag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<S3TablesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_tag_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.tags.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing or invalid 'tags'");
        }
        let resource_arn = input.resource_arn.clone();
        let tags: HashMap<String, String> = input.tags;
        let mut st = state.write().await;
        match st.tag_resource(&resource_arn, tags) {
            Ok(()) => wire::serialize_tag_resource_response(&model::TagResourceResponse {}),
            Err(e) => s3tables_error_response(&e),
        }
    }

    async fn handle_untag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<S3TablesState>>,
        resource_arn: &str,
        keys: Vec<String>,
    ) -> MockResponse {
        let mut st = state.write().await;
        match st.untag_resource(resource_arn, &keys) {
            Ok(()) => wire::serialize_untag_resource_response(&model::UntagResourceResponse {}),
            Err(e) => s3tables_error_response(&e),
        }
    }

    // --- Table Bucket extended handlers ---

    async fn handle_put_table_bucket_encryption(
        &self,
        state: &Arc<tokio::sync::RwLock<S3TablesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        // encryptionConfiguration is a default-equipped non-Option struct in the
        // wire model, so detect "absent" via the raw body.
        if !winterbaume_core::body_has_top_level_field(&request.body, "encryptionConfiguration") {
            return rest_json_error(
                400,
                "ValidationException",
                "Missing 'encryptionConfiguration'",
            );
        }
        let input =
            match wire::deserialize_put_table_bucket_encryption_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        if input.encryption_configuration.sse_algorithm.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'sseAlgorithm'");
        }
        let arn = input.table_bucket_a_r_n.clone();
        let sse_algorithm = input.encryption_configuration.sse_algorithm;
        let kms_key_arn = input.encryption_configuration.kms_key_arn;
        let mut st = state.write().await;
        match st.put_table_bucket_encryption(&arn, sse_algorithm, kms_key_arn) {
            Ok(()) => wire::serialize_put_table_bucket_encryption_response(),
            Err(e) => s3tables_error_response(&e),
        }
    }

    async fn handle_get_table_bucket_encryption(
        &self,
        state: &Arc<tokio::sync::RwLock<S3TablesState>>,
        arn: &str,
    ) -> MockResponse {
        let st = state.read().await;
        match st.get_table_bucket(arn) {
            Ok(bucket) => {
                let encryption_configuration =
                    bucket
                        .encryption
                        .as_ref()
                        .map(|e| model::EncryptionConfiguration {
                            sse_algorithm: e.sse_algorithm.clone(),
                            kms_key_arn: e.kms_key_arn.clone(),
                        });
                wire::serialize_get_table_bucket_encryption_response(
                    &model::GetTableBucketEncryptionResponse {
                        encryption_configuration,
                    },
                )
            }
            Err(e) => s3tables_error_response(&e),
        }
    }

    async fn handle_delete_table_bucket_encryption(
        &self,
        state: &Arc<tokio::sync::RwLock<S3TablesState>>,
        arn: &str,
    ) -> MockResponse {
        let mut st = state.write().await;
        match st.delete_table_bucket_encryption(arn) {
            Ok(()) => wire::serialize_delete_table_bucket_encryption_response(),
            Err(e) => s3tables_error_response(&e),
        }
    }

    async fn handle_put_table_bucket_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<S3TablesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_put_table_bucket_policy_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.resource_policy.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'resourcePolicy'");
        }
        let arn = input.table_bucket_a_r_n.clone();
        let policy = input.resource_policy;
        let mut st = state.write().await;
        match st.put_table_bucket_policy(&arn, policy) {
            Ok(()) => wire::serialize_put_table_bucket_policy_response(),
            Err(e) => s3tables_error_response(&e),
        }
    }

    async fn handle_get_table_bucket_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<S3TablesState>>,
        arn: &str,
    ) -> MockResponse {
        let st = state.read().await;
        match st.get_table_bucket(arn) {
            Ok(bucket) => wire::serialize_get_table_bucket_policy_response(
                &model::GetTableBucketPolicyResponse {
                    resource_policy: bucket.policy.clone(),
                },
            ),
            Err(e) => s3tables_error_response(&e),
        }
    }

    async fn handle_delete_table_bucket_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<S3TablesState>>,
        arn: &str,
    ) -> MockResponse {
        let mut st = state.write().await;
        match st.delete_table_bucket_policy(arn) {
            Ok(()) => wire::serialize_delete_table_bucket_policy_response(),
            Err(e) => s3tables_error_response(&e),
        }
    }

    async fn handle_put_table_bucket_storage_class(
        &self,
        state: &Arc<tokio::sync::RwLock<S3TablesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_put_table_bucket_storage_class_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.storage_class_configuration.storage_class.is_empty() {
            return rest_json_error(
                400,
                "ValidationException",
                "Missing 'storageClassConfiguration.storageClass'",
            );
        }
        let arn = input.table_bucket_a_r_n.clone();
        let storage_class = input.storage_class_configuration.storage_class;
        let mut st = state.write().await;
        match st.put_table_bucket_storage_class(&arn, storage_class) {
            Ok(()) => wire::serialize_put_table_bucket_storage_class_response(),
            Err(e) => s3tables_error_response(&e),
        }
    }

    async fn handle_get_table_bucket_storage_class(
        &self,
        state: &Arc<tokio::sync::RwLock<S3TablesState>>,
        arn: &str,
    ) -> MockResponse {
        let st = state.read().await;
        match st.get_table_bucket(arn) {
            Ok(bucket) => {
                let sc_config =
                    bucket
                        .storage_class
                        .as_ref()
                        .map(|sc| model::StorageClassConfiguration {
                            storage_class: sc.storage_class.clone(),
                        });
                wire::serialize_get_table_bucket_storage_class_response(
                    &model::GetTableBucketStorageClassResponse {
                        storage_class_configuration: sc_config,
                    },
                )
            }
            Err(e) => s3tables_error_response(&e),
        }
    }

    async fn handle_put_table_bucket_maintenance_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<S3TablesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        // Maintenance values are typed enums on the wire; for state compatibility
        // we serialise the typed value back to JSON. Verify presence in raw body
        // since the typed default collapses absent vs default-equipped struct.
        if !winterbaume_core::body_has_top_level_field(&request.body, "value") {
            return rest_json_error(400, "ValidationException", "Missing 'value'");
        }
        let input = match wire::deserialize_put_table_bucket_maintenance_configuration_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let arn = input.table_bucket_a_r_n.clone();
        let type_key = input.r#type.clone();
        let value = serde_json::to_string(&input.value).unwrap_or_default();
        let mut st = state.write().await;
        match st.put_table_bucket_maintenance(&arn, type_key, value) {
            Ok(()) => wire::serialize_put_table_bucket_maintenance_configuration_response(),
            Err(e) => s3tables_error_response(&e),
        }
    }

    async fn handle_get_table_bucket_maintenance_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<S3TablesState>>,
        arn: &str,
    ) -> MockResponse {
        let st = state.read().await;
        match st.get_table_bucket(arn) {
            Ok(bucket) => {
                let configuration = if bucket.maintenance_config.is_empty() {
                    None
                } else {
                    Some(
                        bucket
                            .maintenance_config
                            .iter()
                            .filter_map(|(k, v)| {
                                serde_json::from_str::<
                                    model::TableBucketMaintenanceConfigurationValue,
                                >(v)
                                .ok()
                                .map(|val| (k.clone(), val))
                            })
                            .collect(),
                    )
                };
                wire::serialize_get_table_bucket_maintenance_configuration_response(
                    &model::GetTableBucketMaintenanceConfigurationResponse {
                        table_bucket_a_r_n: Some(arn.to_string()),
                        configuration,
                    },
                )
            }
            Err(e) => s3tables_error_response(&e),
        }
    }

    async fn handle_put_table_bucket_metrics_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<S3TablesState>>,
        arn: &str,
        body: &Value,
    ) -> MockResponse {
        let metrics_json = body.to_string();
        let mut st = state.write().await;
        match st.put_table_bucket_metrics(arn, metrics_json) {
            Ok(()) => wire::serialize_put_table_bucket_metrics_configuration_response(),
            Err(e) => s3tables_error_response(&e),
        }
    }

    async fn handle_get_table_bucket_metrics_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<S3TablesState>>,
        arn: &str,
    ) -> MockResponse {
        let st = state.read().await;
        match st.get_table_bucket(arn) {
            Ok(bucket) => wire::serialize_get_table_bucket_metrics_configuration_response(
                &model::GetTableBucketMetricsConfigurationResponse {
                    table_bucket_a_r_n: if bucket.metrics_config.is_some() {
                        Some(arn.to_string())
                    } else {
                        None
                    },
                    id: None,
                },
            ),
            Err(e) => s3tables_error_response(&e),
        }
    }

    async fn handle_delete_table_bucket_metrics_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<S3TablesState>>,
        arn: &str,
    ) -> MockResponse {
        let mut st = state.write().await;
        match st.delete_table_bucket_metrics(arn) {
            Ok(()) => wire::serialize_delete_table_bucket_metrics_configuration_response(),
            Err(e) => s3tables_error_response(&e),
        }
    }

    async fn handle_put_table_bucket_replication(
        &self,
        state: &Arc<tokio::sync::RwLock<S3TablesState>>,
        arn: &str,
        body: &Value,
    ) -> MockResponse {
        let config_json = body.to_string();
        let mut st = state.write().await;
        match st.put_table_bucket_replication(arn, config_json) {
            Ok(()) => wire::serialize_put_table_bucket_replication_response(
                &model::PutTableBucketReplicationResponse {
                    ..Default::default()
                },
            ),
            Err(e) => s3tables_error_response(&e),
        }
    }

    async fn handle_get_table_bucket_replication(
        &self,
        state: &Arc<tokio::sync::RwLock<S3TablesState>>,
        arn: &str,
    ) -> MockResponse {
        let st = state.read().await;
        match st.get_table_bucket(arn) {
            Ok(bucket) => {
                let configuration = bucket.replication_config.as_deref().and_then(|s| {
                    serde_json::from_str::<model::TableBucketReplicationConfiguration>(s).ok()
                });
                wire::serialize_get_table_bucket_replication_response(
                    &model::GetTableBucketReplicationResponse {
                        configuration,
                        ..Default::default()
                    },
                )
            }
            Err(e) => s3tables_error_response(&e),
        }
    }

    async fn handle_delete_table_bucket_replication(
        &self,
        state: &Arc<tokio::sync::RwLock<S3TablesState>>,
        arn: &str,
    ) -> MockResponse {
        let mut st = state.write().await;
        match st.delete_table_bucket_replication(arn) {
            Ok(()) => wire::serialize_delete_table_bucket_replication_response(),
            Err(e) => s3tables_error_response(&e),
        }
    }

    // --- Table extended handlers ---

    async fn handle_put_table_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<S3TablesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_put_table_policy_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.resource_policy.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'resourcePolicy'");
        }
        let table_bucket_arn = input.table_bucket_a_r_n.clone();
        let namespace = input.namespace.clone();
        let name = input.name.clone();
        let policy = input.resource_policy;
        let mut st = state.write().await;
        match st.put_table_policy(&table_bucket_arn, &namespace, &name, policy) {
            Ok(()) => wire::serialize_put_table_policy_response(),
            Err(e) => s3tables_error_response(&e),
        }
    }

    async fn handle_get_table_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<S3TablesState>>,
        table_bucket_arn: &str,
        namespace: &str,
        name: &str,
    ) -> MockResponse {
        let st = state.read().await;
        match st.get_table(table_bucket_arn, namespace, name) {
            Ok(table) => {
                wire::serialize_get_table_policy_response(&model::GetTablePolicyResponse {
                    resource_policy: table.policy.clone(),
                })
            }
            Err(e) => s3tables_error_response(&e),
        }
    }

    async fn handle_delete_table_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<S3TablesState>>,
        table_bucket_arn: &str,
        namespace: &str,
        name: &str,
    ) -> MockResponse {
        let mut st = state.write().await;
        match st.delete_table_policy(table_bucket_arn, namespace, name) {
            Ok(()) => wire::serialize_delete_table_policy_response(),
            Err(e) => s3tables_error_response(&e),
        }
    }

    async fn handle_put_table_maintenance_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<S3TablesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        // Verify the value field was actually present (typed default collapses
        // absent vs default-equipped struct).
        if !winterbaume_core::body_has_top_level_field(&request.body, "value") {
            return rest_json_error(400, "ValidationException", "Missing 'value'");
        }
        let input = match wire::deserialize_put_table_maintenance_configuration_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let table_bucket_arn = input.table_bucket_a_r_n.clone();
        let namespace = input.namespace.clone();
        let name = input.name.clone();
        let type_key = input.r#type.clone();
        let value = serde_json::to_string(&input.value).unwrap_or_default();
        let mut st = state.write().await;
        match st.put_table_maintenance(&table_bucket_arn, &namespace, &name, type_key, value) {
            Ok(()) => wire::serialize_put_table_maintenance_configuration_response(),
            Err(e) => s3tables_error_response(&e),
        }
    }

    async fn handle_get_table_maintenance_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<S3TablesState>>,
        table_bucket_arn: &str,
        namespace: &str,
        name: &str,
    ) -> MockResponse {
        let st = state.read().await;
        match st.get_table(table_bucket_arn, namespace, name) {
            Ok(table) => {
                let configuration = if table.maintenance_config.is_empty() {
                    None
                } else {
                    Some(
                        table
                            .maintenance_config
                            .iter()
                            .filter_map(|(k, v)| {
                                serde_json::from_str::<model::TableMaintenanceConfigurationValue>(v)
                                    .ok()
                                    .map(|val| (k.clone(), val))
                            })
                            .collect(),
                    )
                };
                wire::serialize_get_table_maintenance_configuration_response(
                    &model::GetTableMaintenanceConfigurationResponse {
                        table_a_r_n: Some(table.arn.clone()),
                        configuration,
                    },
                )
            }
            Err(e) => s3tables_error_response(&e),
        }
    }

    // STUB[no-telemetry]: Maintenance job execution status requires a real job engine; always returns empty status.
    async fn handle_get_table_maintenance_job_status(
        &self,
        state: &Arc<tokio::sync::RwLock<S3TablesState>>,
        table_bucket_arn: &str,
        namespace: &str,
        name: &str,
    ) -> MockResponse {
        let st = state.read().await;
        match st.get_table(table_bucket_arn, namespace, name) {
            Ok(table) => wire::serialize_get_table_maintenance_job_status_response(
                &model::GetTableMaintenanceJobStatusResponse {
                    table_a_r_n: Some(table.arn.clone()),
                    status: None,
                },
            ),
            Err(e) => s3tables_error_response(&e),
        }
    }

    async fn handle_get_table_metadata_location(
        &self,
        state: &Arc<tokio::sync::RwLock<S3TablesState>>,
        table_bucket_arn: &str,
        namespace: &str,
        name: &str,
    ) -> MockResponse {
        let st = state.read().await;
        match st.get_table(table_bucket_arn, namespace, name) {
            Ok(table) => wire::serialize_get_table_metadata_location_response(
                &model::GetTableMetadataLocationResponse {
                    metadata_location: table.metadata_location.clone(),
                    version_token: Some(table.version_token.clone()),
                    warehouse_location: Some(table.warehouse_location.clone()),
                },
            ),
            Err(e) => s3tables_error_response(&e),
        }
    }

    async fn handle_update_table_metadata_location(
        &self,
        state: &Arc<tokio::sync::RwLock<S3TablesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_table_metadata_location_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.metadata_location.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'metadataLocation'");
        }
        if input.version_token.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'versionToken'");
        }
        let table_bucket_arn = input.table_bucket_a_r_n.clone();
        let namespace = input.namespace.clone();
        let name = input.name.clone();
        let metadata_location = input.metadata_location;
        let version_token = input.version_token;
        let table_arn = {
            let st = state.read().await;
            match st.get_table(&table_bucket_arn, &namespace, &name) {
                Ok(t) => t.arn.clone(),
                Err(e) => return s3tables_error_response(&e),
            }
        };
        let ns_parts: Vec<String> = namespace.split('.').map(|s| s.to_string()).collect();
        let mut st = state.write().await;
        match st.update_table_metadata_location(
            &table_bucket_arn,
            &namespace,
            &name,
            metadata_location.clone(),
            &version_token,
        ) {
            Ok(new_version_token) => wire::serialize_update_table_metadata_location_response(
                &model::UpdateTableMetadataLocationResponse {
                    metadata_location: Some(metadata_location),
                    name: Some(name),
                    namespace: Some(ns_parts),
                    table_a_r_n: Some(table_arn),
                    version_token: Some(new_version_token),
                },
            ),
            Err(e) => s3tables_error_response(&e),
        }
    }

    async fn handle_get_table_encryption(
        &self,
        state: &Arc<tokio::sync::RwLock<S3TablesState>>,
        table_bucket_arn: &str,
        namespace: &str,
        name: &str,
    ) -> MockResponse {
        let st = state.read().await;
        match st.get_table(table_bucket_arn, namespace, name) {
            Ok(_) => {
                let encryption_configuration = st
                    .table_buckets
                    .get(table_bucket_arn)
                    .and_then(|b| b.encryption.as_ref())
                    .map(|e| model::EncryptionConfiguration {
                        sse_algorithm: e.sse_algorithm.clone(),
                        kms_key_arn: e.kms_key_arn.clone(),
                    });
                wire::serialize_get_table_encryption_response(&model::GetTableEncryptionResponse {
                    encryption_configuration,
                })
            }
            Err(e) => s3tables_error_response(&e),
        }
    }

    async fn handle_get_table_storage_class(
        &self,
        state: &Arc<tokio::sync::RwLock<S3TablesState>>,
        table_bucket_arn: &str,
        namespace: &str,
        name: &str,
    ) -> MockResponse {
        let st = state.read().await;
        match st.get_table(table_bucket_arn, namespace, name) {
            Ok(table) => {
                let sc_config =
                    table
                        .storage_class
                        .as_ref()
                        .map(|sc| model::StorageClassConfiguration {
                            storage_class: sc.storage_class.clone(),
                        });
                wire::serialize_get_table_storage_class_response(
                    &model::GetTableStorageClassResponse {
                        storage_class_configuration: sc_config,
                    },
                )
            }
            Err(e) => s3tables_error_response(&e),
        }
    }

    async fn handle_put_table_replication(
        &self,
        state: &Arc<tokio::sync::RwLock<S3TablesState>>,
        table_arn: &str,
        body: &Value,
    ) -> MockResponse {
        let config_json = body.to_string();
        let mut st = state.write().await;
        match st.put_table_replication(table_arn, config_json) {
            Ok(()) => wire::serialize_put_table_replication_response(
                &model::PutTableReplicationResponse {
                    ..Default::default()
                },
            ),
            Err(e) => s3tables_error_response(&e),
        }
    }

    async fn handle_get_table_replication(
        &self,
        state: &Arc<tokio::sync::RwLock<S3TablesState>>,
        table_arn: &str,
    ) -> MockResponse {
        let st = state.read().await;
        match st.get_table_by_arn(table_arn) {
            Ok(table) => {
                let configuration = table.replication_config.as_deref().and_then(|s| {
                    serde_json::from_str::<model::TableReplicationConfiguration>(s).ok()
                });
                wire::serialize_get_table_replication_response(
                    &model::GetTableReplicationResponse {
                        configuration,
                        version_token: Some(table.version_token.clone()),
                    },
                )
            }
            Err(e) => s3tables_error_response(&e),
        }
    }

    async fn handle_delete_table_replication(
        &self,
        state: &Arc<tokio::sync::RwLock<S3TablesState>>,
        table_arn: &str,
    ) -> MockResponse {
        let mut st = state.write().await;
        match st.delete_table_replication(table_arn) {
            Ok(()) => wire::serialize_delete_table_replication_response(),
            Err(e) => s3tables_error_response(&e),
        }
    }

    // STUB[no-telemetry]: Table replication status requires real cross-region replication telemetry; always returns empty response.
    async fn handle_get_table_replication_status(
        &self,
        state: &Arc<tokio::sync::RwLock<S3TablesState>>,
        table_arn: &str,
    ) -> MockResponse {
        let st = state.read().await;
        match st.get_table_by_arn(table_arn) {
            Ok(_) => wire::serialize_get_table_replication_status_response(
                &model::GetTableReplicationStatusResponse {
                    ..Default::default()
                },
            ),
            Err(e) => s3tables_error_response(&e),
        }
    }

    async fn handle_put_table_record_expiration_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<S3TablesState>>,
        table_arn: &str,
        body: &Value,
    ) -> MockResponse {
        let config_json = body.to_string();
        let mut st = state.write().await;
        match st.put_table_record_expiration(table_arn, config_json) {
            Ok(()) => wire::serialize_put_table_record_expiration_configuration_response(),
            Err(e) => s3tables_error_response(&e),
        }
    }

    async fn handle_get_table_record_expiration_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<S3TablesState>>,
        table_arn: &str,
    ) -> MockResponse {
        let st = state.read().await;
        match st.get_table_by_arn(table_arn) {
            Ok(table) => {
                let configuration = table.record_expiration_config.as_deref().and_then(|s| {
                    serde_json::from_str::<model::TableRecordExpirationConfigurationValue>(s).ok()
                });
                wire::serialize_get_table_record_expiration_configuration_response(
                    &model::GetTableRecordExpirationConfigurationResponse { configuration },
                )
            }
            Err(e) => s3tables_error_response(&e),
        }
    }

    // STUB[no-telemetry]: Record expiration job status requires a real background job engine; always returns empty response.
    async fn handle_get_table_record_expiration_job_status(
        &self,
        state: &Arc<tokio::sync::RwLock<S3TablesState>>,
        table_arn: &str,
    ) -> MockResponse {
        let st = state.read().await;
        match st.get_table_by_arn(table_arn) {
            Ok(_) => wire::serialize_get_table_record_expiration_job_status_response(
                &model::GetTableRecordExpirationJobStatusResponse {
                    ..Default::default()
                },
            ),
            Err(e) => s3tables_error_response(&e),
        }
    }

    async fn handle_rename_table(
        &self,
        state: &Arc<tokio::sync::RwLock<S3TablesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_rename_table_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let table_bucket_arn = input.table_bucket_a_r_n.clone();
        let namespace = input.namespace.clone();
        let name = input.name.clone();
        let new_name = input.new_name;
        let new_namespace = input.new_namespace_name;
        let version_token = input.version_token;
        let mut st = state.write().await;
        match st.rename_table(
            &table_bucket_arn,
            &namespace,
            &name,
            new_name,
            new_namespace,
            version_token,
        ) {
            Ok(()) => wire::serialize_rename_table_response(),
            Err(e) => s3tables_error_response(&e),
        }
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn extract_path(uri: &str) -> String {
    // Delegate to the shared core helper, which correctly strips the scheme
    // and host (including custom-endpoint hostnames like `127.0.0.1:PORT`)
    // before returning the path. The previous implementation only matched on
    // `amazonaws.com` and returned the entire URI for non-AWS endpoints,
    // causing dispatch to fail with 404 against the in-process mock server.
    winterbaume_core::extract_path(uri)
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

fn extract_query_param(uri: &str, param: &str) -> Option<String> {
    let query = uri.split('?').nth(1)?;
    for part in query.split('&') {
        if let Some((k, v)) = part.split_once('=')
            && k == param
        {
            return Some(percent_decode(v));
        }
    }
    None
}

fn extract_query_param_multi(uri: &str, param: &str) -> Vec<String> {
    let mut result = Vec::new();
    let query = match uri.split('?').nth(1) {
        Some(q) => q,
        None => return result,
    };
    for part in query.split('&') {
        if let Some((k, v)) = part.split_once('=')
            && k == param
        {
            result.push(percent_decode(v));
        }
    }
    result
}

fn s3tables_error_response(err: &S3TablesError) -> MockResponse {
    let (status, error_type) = match err {
        S3TablesError::NotFound(_) => (404u16, "NotFoundException"),
        S3TablesError::Conflict(_) => (409u16, "ConflictException"),
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
