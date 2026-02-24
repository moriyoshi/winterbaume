use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::{Value, json};
use tokio::io::AsyncReadExt;
use winterbaume_core::{
    BackendState, BlobStore, BlobStoreMap, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse,
    MockService, StateChangeNotifier, Vfs,
};

use crate::state::{EbsError, EbsState};
use crate::views::EbsStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct EbsService {
    pub(crate) state: Arc<BackendState<EbsState>>,
    pub(crate) blobs: Arc<BlobStoreMap>,
    pub(crate) notifier: StateChangeNotifier<EbsStateView>,
}

impl EbsService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            blobs: Arc::new(BlobStoreMap::mem("ebs")),
            notifier: StateChangeNotifier::new(),
        }
    }

    /// Returns sorted `(account_id, region)` pairs that have state.
    pub fn scopes_with_state(&self) -> Vec<(String, String)> {
        self.state.scopes_with_state()
    }

    /// Create an EBS service backed by the given VFS for blob storage.
    pub fn with_vfs(vfs: Arc<dyn Vfs>) -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            blobs: Arc::new(BlobStoreMap::new(vfs, "ebs")),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for EbsService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for EbsService {
    fn service_name(&self) -> &str {
        "ebs"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://ebs\.(.+)\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl EbsService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = DEFAULT_ACCOUNT_ID;
        let state = self.state.get(account_id, &region);
        let blobs = self.blobs.get(account_id, &region);

        let path = extract_path(&request.uri);
        let query = extract_query(&request.uri);
        let method = request.method.as_str();

        let segments: Vec<&str> = path
            .trim_start_matches('/')
            .split('/')
            .filter(|s| !s.is_empty())
            .collect();

        // Routes:
        // POST /snapshots - StartSnapshot
        // POST /snapshots/completion/{SnapshotId} - CompleteSnapshot
        // GET  /snapshots/{SnapshotId}/blocks - ListSnapshotBlocks
        // PUT  /snapshots/{SnapshotId}/blocks/{BlockIndex} - PutSnapshotBlock

        if segments.is_empty() || segments[0] != "snapshots" {
            return rest_json_error(404, "UnknownOperationException", "Not found");
        }

        use winterbaume_core::StatefulService;
        let response = match (method, segments.len()) {
            // POST /snapshots - StartSnapshot
            ("POST", 1) => {
                let body: Value = match serde_json::from_slice(&request.body) {
                    Ok(v) => v,
                    Err(_) => {
                        return rest_json_error(400, "ValidationException", "Invalid JSON body");
                    }
                };
                self.handle_start_snapshot(&state, &body).await
            }
            // POST /snapshots/completion/{SnapshotId} - CompleteSnapshot
            ("POST", 3) if segments[1] == "completion" => {
                let snapshot_id = percent_decode(segments[2]);
                self.handle_complete_snapshot(&state, &snapshot_id, &request)
                    .await
            }
            // GET /snapshots/{SnapshotId}/blocks - ListSnapshotBlocks
            ("GET", 3) if segments[2] == "blocks" => {
                let snapshot_id = percent_decode(segments[1]);
                self.handle_list_snapshot_blocks(&state, &snapshot_id, &query)
                    .await
            }
            // PUT /snapshots/{SnapshotId}/blocks/{BlockIndex} - PutSnapshotBlock
            ("PUT", 4) if segments[2] == "blocks" => {
                let snapshot_id = percent_decode(segments[1]);
                let block_index: i32 = match segments[3].parse() {
                    Ok(v) => v,
                    Err(_) => {
                        return rest_json_error(400, "ValidationException", "Invalid BlockIndex");
                    }
                };
                self.handle_put_snapshot_block(&blobs, &state, &snapshot_id, block_index, &request)
                    .await
            }
            // GET /snapshots/{SnapshotId}/blocks/{BlockIndex} - GetSnapshotBlock
            ("GET", 4) if segments[2] == "blocks" => {
                let snapshot_id = percent_decode(segments[1]);
                let block_index: i32 = match segments[3].parse() {
                    Ok(v) => v,
                    Err(_) => {
                        return rest_json_error(400, "ValidationException", "Invalid BlockIndex");
                    }
                };
                self.handle_get_snapshot_block(&blobs, &state, &snapshot_id, block_index, &query)
                    .await
            }
            // GET /snapshots/{SnapshotId}/changedblocks - ListChangedBlocks
            ("GET", 3) if segments[2] == "changedblocks" => {
                let snapshot_id = percent_decode(segments[1]);
                self.handle_list_changed_blocks(&state, &snapshot_id, &query)
                    .await
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        };
        if response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_start_snapshot(
        &self,
        state: &Arc<tokio::sync::RwLock<EbsState>>,
        body: &Value,
    ) -> MockResponse {
        let volume_size = match body.get("VolumeSize").and_then(|v| v.as_i64()) {
            Some(n) => n,
            None => {
                return rest_json_error(400, "ValidationException", "VolumeSize is required");
            }
        };

        let description = body
            .get("Description")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        let mut state = state.write().await;
        match state.start_snapshot(volume_size, description) {
            Ok(snapshot) => {
                let resp = wire::StartSnapshotResponse {
                    snapshot_id: Some(snapshot.snapshot_id.clone()),
                    volume_size: Some(snapshot.volume_size),
                    block_size: Some(snapshot.block_size),
                    description: Some(snapshot.description.clone()),
                    status: Some(snapshot.status.as_str().to_string()),
                    owner_id: Some(DEFAULT_ACCOUNT_ID.to_string()),
                    ..Default::default()
                };
                wire::serialize_start_snapshot_response(&resp)
            }
            Err(e) => ebs_error_response(&e),
        }
    }

    async fn handle_complete_snapshot(
        &self,
        state: &Arc<tokio::sync::RwLock<EbsState>>,
        snapshot_id: &str,
        request: &MockRequest,
    ) -> MockResponse {
        // x-amz-ChangedBlocksCount is sent as a header
        let changed_blocks_count = request
            .headers
            .get("x-amz-changedblockscount")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.parse::<i32>().ok())
            .unwrap_or(0);

        let mut state = state.write().await;
        match state.complete_snapshot(snapshot_id, changed_blocks_count) {
            Ok(snapshot) => {
                let resp = wire::CompleteSnapshotResponse {
                    status: Some(snapshot.status.as_str().to_string()),
                };
                wire::serialize_complete_snapshot_response(&resp)
            }
            Err(e) => ebs_error_response(&e),
        }
    }

    async fn handle_list_snapshot_blocks(
        &self,
        state: &Arc<tokio::sync::RwLock<EbsState>>,
        snapshot_id: &str,
        query: &str,
    ) -> MockResponse {
        let max_results =
            parse_query_param(query, "maxResults").and_then(|v| v.parse::<i32>().ok());
        let starting_block_index =
            parse_query_param(query, "startingBlockIndex").and_then(|v| v.parse::<i32>().ok());

        let state = state.read().await;
        match state.list_snapshot_blocks(snapshot_id, max_results, starting_block_index) {
            Ok((blocks, volume_size, block_size)) => {
                let block_entries: Vec<wire::Block> = blocks
                    .iter()
                    .map(|b| wire::Block {
                        block_index: Some(b.block_index),
                        block_token: Some(b.block_token.clone()),
                    })
                    .collect();
                let resp = wire::ListSnapshotBlocksResponse {
                    blocks: Some(block_entries),
                    volume_size: Some(volume_size),
                    block_size: Some(block_size),
                    expiry_time: Some(1700000000.0),
                    ..Default::default()
                };
                wire::serialize_list_snapshot_blocks_response(&resp)
            }
            Err(e) => ebs_error_response(&e),
        }
    }

    async fn handle_put_snapshot_block(
        &self,
        blobs: &BlobStore,
        state: &Arc<tokio::sync::RwLock<EbsState>>,
        snapshot_id: &str,
        block_index: i32,
        request: &MockRequest,
    ) -> MockResponse {
        let checksum = request
            .headers
            .get("x-amz-checksum")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("")
            .to_string();

        let checksum_algorithm = request
            .headers
            .get("x-amz-checksum-algorithm")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("SHA256")
            .to_string();

        let data = request.body.clone();
        let content_length = data.len() as u64;

        // If no checksum provided, compute one
        let checksum = if checksum.is_empty() {
            use base64::Engine;
            use sha2::Digest;
            let hash = sha2::Sha256::digest(&data);
            base64::engine::general_purpose::STANDARD.encode(hash)
        } else {
            checksum
        };

        // Store block data in the BlobStore before updating state.
        let blob_key = format!("{snapshot_id}/{block_index}");
        if let Err(e) = blobs.put_from(&blob_key, &mut data.as_ref()).await {
            return rest_json_error(500, "InternalError", &e.to_string());
        }

        let mut state = state.write().await;
        match state.put_snapshot_block(
            snapshot_id,
            block_index,
            blob_key,
            content_length,
            &checksum,
            &checksum_algorithm,
        ) {
            Ok((checksum, algorithm)) => {
                let wire_resp = wire::PutSnapshotBlockResponse {
                    checksum: Some(checksum.to_string()),
                    checksum_algorithm: Some(algorithm.to_string()),
                };
                let mut resp = wire::serialize_put_snapshot_block_response(&wire_resp);
                resp.headers.insert(
                    HeaderName::from_static("x-amz-checksum"),
                    checksum.parse().unwrap(),
                );
                resp.headers.insert(
                    HeaderName::from_static("x-amz-checksum-algorithm"),
                    algorithm.parse().unwrap(),
                );
                resp
            }
            Err(e) => ebs_error_response(&e),
        }
    }

    async fn handle_get_snapshot_block(
        &self,
        blobs: &BlobStore,
        state: &Arc<tokio::sync::RwLock<EbsState>>,
        snapshot_id: &str,
        block_index: i32,
        _query: &str,
    ) -> MockResponse {
        // Extract all needed fields inside a block so the RwLockReadGuard
        // is dropped before the first .await point.
        let block_info = {
            let state = state.read().await;
            let Some(snapshot) = state.snapshots.get(snapshot_id) else {
                return rest_json_error(
                    404,
                    "ResourceNotFoundException",
                    &format!("Snapshot {snapshot_id} not found"),
                );
            };
            snapshot.blocks.get(&block_index).map(|block| {
                (
                    block.blob_key.clone(),
                    block.content_length,
                    block.checksum.clone(),
                    block.checksum_algorithm.clone(),
                )
            })
        };

        match block_info {
            Some((blob_key, content_length, checksum, checksum_algorithm)) => {
                let mut reader = match blobs.get_reader(&blob_key).await {
                    Ok(r) => r,
                    Err(e) => return rest_json_error(500, "InternalError", &e.to_string()),
                };
                let mut buf = Vec::new();
                if let Err(e) = reader.read_to_end(&mut buf).await {
                    return rest_json_error(500, "InternalError", &e.to_string());
                }
                let mut resp = MockResponse {
                    status: 200,
                    headers: http::HeaderMap::new(),
                    body: buf.into(),
                };
                resp.headers.insert(
                    http::header::CONTENT_TYPE,
                    "application/octet-stream".parse().unwrap(),
                );
                resp.headers.insert(
                    HeaderName::from_static("x-amz-checksum"),
                    checksum.parse().unwrap(),
                );
                resp.headers.insert(
                    HeaderName::from_static("x-amz-checksum-algorithm"),
                    checksum_algorithm.parse().unwrap(),
                );
                resp.headers.insert(
                    HeaderName::from_static("x-amz-data-length"),
                    content_length.to_string().parse().unwrap(),
                );
                resp
            }
            None => rest_json_error(
                404,
                "ResourceNotFoundException",
                &format!("Block {block_index} not found in snapshot {snapshot_id}"),
            ),
        }
    }

    async fn handle_list_changed_blocks(
        &self,
        state: &Arc<tokio::sync::RwLock<EbsState>>,
        snapshot_id: &str,
        query: &str,
    ) -> MockResponse {
        let second_snapshot_id = parse_query_param(query, "secondSnapshotId");
        let starting_block_index = parse_query_param(query, "startingBlockIndex")
            .and_then(|v| v.parse::<i32>().ok())
            .unwrap_or(0);

        // The second snapshot is the one we're comparing TO (required)
        // The first snapshot (from path) is the base
        let target_id = second_snapshot_id.unwrap_or(snapshot_id);

        let state = state.read().await;
        let snapshot = match state.snapshots.get(target_id) {
            Some(s) => s,
            None => {
                return rest_json_error(
                    404,
                    "ResourceNotFoundException",
                    &format!("Snapshot {target_id} not found"),
                );
            }
        };

        // Return all blocks in the target snapshot as "changed", filtered by startingBlockIndex
        let mut blocks: Vec<&crate::types::Block> = snapshot
            .blocks
            .values()
            .filter(|b| b.block_index >= starting_block_index)
            .collect();
        blocks.sort_by_key(|b| b.block_index);

        let entries: Vec<wire::ChangedBlock> = blocks
            .iter()
            .map(|b| wire::ChangedBlock {
                block_index: Some(b.block_index),
                first_block_token: Some(format!("first-token-{}-{}", target_id, b.block_index)),
                second_block_token: Some(b.block_token.clone()),
            })
            .collect();

        let resp = wire::ListChangedBlocksResponse {
            changed_blocks: Some(entries),
            volume_size: Some(snapshot.volume_size),
            block_size: Some(snapshot.block_size),
            expiry_time: Some(1700000000.0),
            ..Default::default()
        };
        wire::serialize_list_changed_blocks_response(&resp)
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
        let path_part = uri.split("://").nth(1).unwrap_or(uri);
        let after_host = path_part.find('/').map(|i| &path_part[i..]).unwrap_or("/");
        after_host
            .split('?')
            .next()
            .unwrap_or(after_host)
            .to_string()
    }
}

fn extract_query(uri: &str) -> String {
    uri.split('?').nth(1).unwrap_or("").to_string()
}

fn parse_query_param<'a>(query: &'a str, key: &str) -> Option<&'a str> {
    query.split('&').find_map(|pair| {
        let (k, v) = pair.split_once('=')?;

        if k == key { Some(v) } else { None }
    })
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

fn ebs_error_response(err: &EbsError) -> MockResponse {
    let (status, error_type) = match err {
        EbsError::InvalidVolumeSize => (400, "ValidationException"),
        EbsError::SnapshotNotFound { .. } => (404, "ResourceNotFoundException"),
        EbsError::SnapshotNotPending { .. } => (409, "ConflictException"),
    };
    let body = json!({
        "Message": err.to_string(),
    });
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers
        .insert(X_AMZN_ERRORTYPE, error_type.parse().unwrap());
    resp
}

fn rest_json_error(status: u16, code: &str, message: &str) -> MockResponse {
    let body = json!({
        "Message": message,
    });
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers.insert(X_AMZN_ERRORTYPE, code.parse().unwrap());
    resp
}
