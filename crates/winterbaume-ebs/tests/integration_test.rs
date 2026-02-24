use aws_sdk_ebs::config::BehaviorVersion;
use aws_smithy_types::byte_stream::ByteStream;
use winterbaume_core::MockAws;
use winterbaume_ebs::EbsService;

async fn make_client() -> aws_sdk_ebs::Client {
    let mock = MockAws::builder().with_service(EbsService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_ebs::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_ebs::Client::new(&config)
}

#[tokio::test]
async fn test_start_snapshot() {
    let client = make_client().await;

    let resp = client
        .start_snapshot()
        .volume_size(1)
        .description("test snapshot")
        .send()
        .await
        .expect("start_snapshot should succeed");

    assert!(resp.snapshot_id().is_some());
    assert_eq!(resp.volume_size(), Some(1));
    assert_eq!(resp.block_size(), Some(524288));
    assert_eq!(resp.status(), Some(&aws_sdk_ebs::types::Status::Pending));
}

#[tokio::test]
async fn test_complete_snapshot() {
    let client = make_client().await;

    let start_resp = client
        .start_snapshot()
        .volume_size(1)
        .send()
        .await
        .expect("start_snapshot should succeed");

    let snapshot_id = start_resp.snapshot_id().unwrap();

    let complete_resp = client
        .complete_snapshot()
        .snapshot_id(snapshot_id)
        .changed_blocks_count(0)
        .send()
        .await
        .expect("complete_snapshot should succeed");

    assert_eq!(
        complete_resp.status(),
        Some(&aws_sdk_ebs::types::Status::Completed)
    );
}

#[tokio::test]
async fn test_list_snapshot_blocks() {
    let client = make_client().await;

    let start_resp = client
        .start_snapshot()
        .volume_size(1)
        .send()
        .await
        .expect("start_snapshot should succeed");

    let snapshot_id = start_resp.snapshot_id().unwrap();

    // List blocks on empty snapshot
    let list_resp = client
        .list_snapshot_blocks()
        .snapshot_id(snapshot_id)
        .send()
        .await
        .expect("list_snapshot_blocks should succeed");

    assert_eq!(list_resp.blocks().len(), 0);
    assert_eq!(list_resp.volume_size(), Some(1));
    assert_eq!(list_resp.block_size(), Some(524288));
}

#[tokio::test]
async fn test_put_and_list_blocks() {
    let client = make_client().await;

    let start_resp = client
        .start_snapshot()
        .volume_size(1)
        .send()
        .await
        .expect("start_snapshot should succeed");

    let snapshot_id = start_resp.snapshot_id().unwrap();

    // Write a block
    let block_data = vec![0xABu8; 512 * 1024]; // 512 KiB block

    // Compute checksum
    use sha2::Digest;
    let hash = sha2::Sha256::digest(&block_data);
    let checksum = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, hash);

    let put_resp = client
        .put_snapshot_block()
        .snapshot_id(snapshot_id)
        .block_index(0)
        .block_data(ByteStream::from(block_data.clone()))
        .data_length(block_data.len() as i32)
        .checksum(checksum.clone())
        .checksum_algorithm(aws_sdk_ebs::types::ChecksumAlgorithm::ChecksumAlgorithmSha256)
        .send()
        .await
        .expect("put_snapshot_block should succeed");

    assert!(put_resp.checksum().is_some());
    assert_eq!(
        put_resp.checksum_algorithm(),
        Some(&aws_sdk_ebs::types::ChecksumAlgorithm::ChecksumAlgorithmSha256)
    );

    // Write another block
    let block_data2 = vec![0xCDu8; 512 * 1024];
    let hash2 = sha2::Sha256::digest(&block_data2);
    let checksum2 = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, hash2);

    client
        .put_snapshot_block()
        .snapshot_id(snapshot_id)
        .block_index(1)
        .block_data(ByteStream::from(block_data2.clone()))
        .data_length(block_data2.len() as i32)
        .checksum(checksum2)
        .checksum_algorithm(aws_sdk_ebs::types::ChecksumAlgorithm::ChecksumAlgorithmSha256)
        .send()
        .await
        .expect("second put_snapshot_block should succeed");

    // List blocks - should now have 2
    let list_resp = client
        .list_snapshot_blocks()
        .snapshot_id(snapshot_id)
        .send()
        .await
        .expect("list_snapshot_blocks should succeed");

    assert_eq!(list_resp.blocks().len(), 2);

    // Verify blocks are sorted by index
    let blocks = list_resp.blocks();
    assert_eq!(blocks[0].block_index(), Some(0));
    assert_eq!(blocks[1].block_index(), Some(1));
    assert!(blocks[0].block_token().is_some());
}

// ============================================================================
// Tests derived from AWS documentation: EBS direct APIs
// ============================================================================

/// Helper to write a single block of data to a pending snapshot and return its block token.
async fn put_block(
    client: &aws_sdk_ebs::Client,
    snapshot_id: &str,
    block_index: i32,
    fill_byte: u8,
) -> String {
    use base64::Engine;
    use sha2::Digest;

    let block_data = vec![fill_byte; 512 * 1024];
    let hash = sha2::Sha256::digest(&block_data);
    let checksum = base64::engine::general_purpose::STANDARD.encode(hash);

    client
        .put_snapshot_block()
        .snapshot_id(snapshot_id)
        .block_index(block_index)
        .block_data(ByteStream::from(block_data.clone()))
        .data_length(block_data.len() as i32)
        .checksum(checksum)
        .checksum_algorithm(aws_sdk_ebs::types::ChecksumAlgorithm::ChecksumAlgorithmSha256)
        .send()
        .await
        .expect("put_snapshot_block should succeed");

    // Return the block token via list
    let list_resp = client
        .list_snapshot_blocks()
        .snapshot_id(snapshot_id)
        .send()
        .await
        .expect("list_snapshot_blocks should succeed");

    list_resp
        .blocks()
        .iter()
        .find(|b| b.block_index() == Some(block_index))
        .expect("block should appear in list")
        .block_token()
        .expect("block_token should be present")
        .to_string()
}

#[tokio::test]
async fn test_start_snapshot_with_description() {
    let client = make_client().await;

    let resp = client
        .start_snapshot()
        .volume_size(2)
        .description("my test snapshot")
        .send()
        .await
        .expect("start_snapshot should succeed");

    assert!(resp.snapshot_id().is_some());
    assert_eq!(resp.volume_size(), Some(2));
    assert_eq!(resp.description(), Some("my test snapshot"));
    assert_eq!(resp.status(), Some(&aws_sdk_ebs::types::Status::Pending));
    // Snapshot ID must match snap-<hex> pattern
    let snap_id = resp.snapshot_id().unwrap();
    assert!(
        snap_id.starts_with("snap-"),
        "snapshot_id should start with 'snap-', got: {snap_id}"
    );
}

#[tokio::test]
async fn test_start_snapshot_missing_volume_size() {
    let client = make_client().await;

    // The SDK requires volume_size, so simulate missing VolumeSize by calling
    // the raw operation without it -- the builder requires it so we test
    // invalid volume_size=0 instead, which the handler rejects.
    // Note: volume_size minimum is 1 per docs; our handler validates > 0.
    // The SDK builder enforces the field is set, but 0 is out-of-range.
    // We verify that the handler rejects 0 as a ValidationException.
    // (Sending volume_size=0 is possible via the SDK builder.)
    let err = client
        .start_snapshot()
        .volume_size(0)
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ValidationException"),
        "Expected ValidationException for volume_size=0, got: {err_str}"
    );
}

#[tokio::test]
async fn test_complete_snapshot_not_found() {
    let client = make_client().await;

    let err = client
        .complete_snapshot()
        .snapshot_id("snap-0000000000000000a")
        .changed_blocks_count(0)
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_complete_snapshot_already_completed() {
    let client = make_client().await;

    let start_resp = client
        .start_snapshot()
        .volume_size(1)
        .send()
        .await
        .expect("start_snapshot should succeed");

    let snapshot_id = start_resp.snapshot_id().unwrap().to_string();

    // First completion should succeed
    client
        .complete_snapshot()
        .snapshot_id(&snapshot_id)
        .changed_blocks_count(0)
        .send()
        .await
        .expect("first complete_snapshot should succeed");

    // Second completion should fail with ConflictException
    let err = client
        .complete_snapshot()
        .snapshot_id(&snapshot_id)
        .changed_blocks_count(0)
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ConflictException"),
        "Expected ConflictException on second complete, got: {err_str}"
    );
}

#[tokio::test]
async fn test_get_snapshot_block() {
    let client = make_client().await;

    let start_resp = client
        .start_snapshot()
        .volume_size(1)
        .send()
        .await
        .expect("start_snapshot should succeed");

    let snapshot_id = start_resp.snapshot_id().unwrap().to_string();

    let fill_byte = 0xAAu8;
    let block_data = vec![fill_byte; 512 * 1024];
    let block_token = put_block(&client, &snapshot_id, 0, fill_byte).await;

    let get_resp = client
        .get_snapshot_block()
        .snapshot_id(&snapshot_id)
        .block_index(0)
        .block_token(block_token)
        .send()
        .await
        .expect("get_snapshot_block should succeed");

    assert!(get_resp.checksum().is_some());
    assert_eq!(
        get_resp.checksum_algorithm(),
        Some(&aws_sdk_ebs::types::ChecksumAlgorithm::ChecksumAlgorithmSha256)
    );
    assert_eq!(get_resp.data_length(), Some(block_data.len() as i32));

    // Verify the returned data matches what was written
    let body_bytes = get_resp
        .block_data
        .collect()
        .await
        .expect("should collect block data")
        .into_bytes();
    assert_eq!(body_bytes.as_ref(), block_data.as_slice());
}

#[tokio::test]
async fn test_get_snapshot_block_not_found() {
    let client = make_client().await;

    let start_resp = client
        .start_snapshot()
        .volume_size(1)
        .send()
        .await
        .expect("start_snapshot should succeed");

    let snapshot_id = start_resp.snapshot_id().unwrap().to_string();

    // Block index 999 does not exist; use a dummy token (handler checks block existence)
    let err = client
        .get_snapshot_block()
        .snapshot_id(&snapshot_id)
        .block_index(999)
        .block_token("dummytoken")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException for missing block, got: {err_str}"
    );
}

#[tokio::test]
async fn test_get_snapshot_block_nonexistent_snapshot() {
    let client = make_client().await;

    let err = client
        .get_snapshot_block()
        .snapshot_id("snap-0000000000000000b")
        .block_index(0)
        .block_token("dummytoken")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException for missing snapshot, got: {err_str}"
    );
}

#[tokio::test]
async fn test_put_block_on_completed_snapshot() {
    let client = make_client().await;

    let start_resp = client
        .start_snapshot()
        .volume_size(1)
        .send()
        .await
        .expect("start_snapshot should succeed");

    let snapshot_id = start_resp.snapshot_id().unwrap().to_string();

    // Complete the snapshot
    client
        .complete_snapshot()
        .snapshot_id(&snapshot_id)
        .changed_blocks_count(0)
        .send()
        .await
        .expect("complete_snapshot should succeed");

    // Trying to put a block on a completed snapshot should fail
    use base64::Engine;
    use sha2::Digest;
    let block_data = vec![0xBBu8; 512 * 1024];
    let hash = sha2::Sha256::digest(&block_data);
    let checksum = base64::engine::general_purpose::STANDARD.encode(hash);

    let err = client
        .put_snapshot_block()
        .snapshot_id(&snapshot_id)
        .block_index(0)
        .block_data(ByteStream::from(block_data.clone()))
        .data_length(block_data.len() as i32)
        .checksum(checksum)
        .checksum_algorithm(aws_sdk_ebs::types::ChecksumAlgorithm::ChecksumAlgorithmSha256)
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ConflictException"),
        "Expected ConflictException for put on completed snapshot, got: {err_str}"
    );
}

#[tokio::test]
async fn test_list_snapshot_blocks_with_starting_block_index() {
    let client = make_client().await;

    let start_resp = client
        .start_snapshot()
        .volume_size(1)
        .send()
        .await
        .expect("start_snapshot should succeed");

    let snapshot_id = start_resp.snapshot_id().unwrap().to_string();

    // Write blocks at indices 0, 1, 2
    for idx in 0..3i32 {
        put_block(&client, &snapshot_id, idx, 0xCC).await;
    }

    // List starting from block index 1 — should see indices 1 and 2 only
    let list_resp = client
        .list_snapshot_blocks()
        .snapshot_id(&snapshot_id)
        .starting_block_index(1)
        .send()
        .await
        .expect("list_snapshot_blocks with starting_block_index should succeed");

    let blocks = list_resp.blocks();
    assert_eq!(
        blocks.len(),
        2,
        "Should return 2 blocks starting from index 1"
    );
    assert_eq!(blocks[0].block_index(), Some(1));
    assert_eq!(blocks[1].block_index(), Some(2));
}

#[tokio::test]
async fn test_list_snapshot_blocks_not_found() {
    let client = make_client().await;

    let err = client
        .list_snapshot_blocks()
        .snapshot_id("snap-0000000000000000c")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException for missing snapshot, got: {err_str}"
    );
}

#[tokio::test]
async fn test_list_changed_blocks() {
    let client = make_client().await;

    // Create and populate a snapshot
    let start_resp = client
        .start_snapshot()
        .volume_size(1)
        .send()
        .await
        .expect("start_snapshot should succeed");

    let snapshot_id = start_resp.snapshot_id().unwrap().to_string();

    put_block(&client, &snapshot_id, 0, 0xDD).await;
    put_block(&client, &snapshot_id, 1, 0xEE).await;

    // ListChangedBlocks: handler uses secondSnapshotId to identify the target
    let list_resp = client
        .list_changed_blocks()
        .second_snapshot_id(&snapshot_id)
        .send()
        .await
        .expect("list_changed_blocks should succeed");

    let changed_blocks = list_resp.changed_blocks();
    assert_eq!(changed_blocks.len(), 2, "Should return 2 changed blocks");
    assert_eq!(changed_blocks[0].block_index(), Some(0));
    assert_eq!(changed_blocks[1].block_index(), Some(1));
    // Each block should have a second block token
    assert!(changed_blocks[0].second_block_token().is_some());
    assert!(changed_blocks[1].second_block_token().is_some());
    // Volume size and block size should be populated
    assert_eq!(list_resp.volume_size(), Some(1));
    assert_eq!(list_resp.block_size(), Some(524288));
}

#[tokio::test]
async fn test_list_changed_blocks_not_found() {
    let client = make_client().await;

    let err = client
        .list_changed_blocks()
        .second_snapshot_id("snap-0000000000000000d")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException for missing snapshot in list_changed_blocks, got: {err_str}"
    );
}

#[tokio::test]
async fn test_snapshot_full_lifecycle() {
    let client = make_client().await;

    // 1. Start snapshot
    let start_resp = client
        .start_snapshot()
        .volume_size(4)
        .description("lifecycle test")
        .send()
        .await
        .expect("start_snapshot should succeed");

    let snapshot_id = start_resp.snapshot_id().unwrap().to_string();
    assert_eq!(
        start_resp.status(),
        Some(&aws_sdk_ebs::types::Status::Pending)
    );
    assert!(snapshot_id.starts_with("snap-"));

    // 2. Write two blocks
    let token0 = put_block(&client, &snapshot_id, 0, 0x11).await;
    let token1 = put_block(&client, &snapshot_id, 1, 0x22).await;

    // 3. List and verify both blocks appear
    let list_resp = client
        .list_snapshot_blocks()
        .snapshot_id(&snapshot_id)
        .send()
        .await
        .expect("list_snapshot_blocks should succeed");

    assert_eq!(list_resp.blocks().len(), 2);
    assert_eq!(list_resp.volume_size(), Some(4));

    // 4. Get each block and verify contents
    let get0 = client
        .get_snapshot_block()
        .snapshot_id(&snapshot_id)
        .block_index(0)
        .block_token(&token0)
        .send()
        .await
        .expect("get_snapshot_block(0) should succeed");
    let data0 = get0.block_data.collect().await.unwrap().into_bytes();
    assert_eq!(data0.as_ref(), vec![0x11u8; 512 * 1024].as_slice());

    let get1 = client
        .get_snapshot_block()
        .snapshot_id(&snapshot_id)
        .block_index(1)
        .block_token(&token1)
        .send()
        .await
        .expect("get_snapshot_block(1) should succeed");
    let data1 = get1.block_data.collect().await.unwrap().into_bytes();
    assert_eq!(data1.as_ref(), vec![0x22u8; 512 * 1024].as_slice());

    // 5. Complete the snapshot
    let complete_resp = client
        .complete_snapshot()
        .snapshot_id(&snapshot_id)
        .changed_blocks_count(2)
        .send()
        .await
        .expect("complete_snapshot should succeed");

    assert_eq!(
        complete_resp.status(),
        Some(&aws_sdk_ebs::types::Status::Completed)
    );
}

// ============================================================================
// Additional tests derived from AWS documentation: EBS direct APIs (2026-03-28)
// ============================================================================

#[tokio::test]
async fn test_put_snapshot_block_on_nonexistent_snapshot() {
    let client = make_client().await;

    use base64::Engine;
    use sha2::Digest;
    let block_data = vec![0xFFu8; 512 * 1024];
    let hash = sha2::Sha256::digest(&block_data);
    let checksum = base64::engine::general_purpose::STANDARD.encode(hash);

    let err = client
        .put_snapshot_block()
        .snapshot_id("snap-0000000000000000e")
        .block_index(0)
        .block_data(ByteStream::from(block_data.clone()))
        .data_length(block_data.len() as i32)
        .checksum(checksum)
        .checksum_algorithm(aws_sdk_ebs::types::ChecksumAlgorithm::ChecksumAlgorithmSha256)
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException for put on nonexistent snapshot, got: {err_str}"
    );
}

#[tokio::test]
async fn test_put_snapshot_block_overwrite() {
    let client = make_client().await;

    let start_resp = client
        .start_snapshot()
        .volume_size(1)
        .send()
        .await
        .expect("start_snapshot should succeed");

    let snapshot_id = start_resp.snapshot_id().unwrap().to_string();

    // Write first value to block 0 (fill byte 0x11)
    put_block(&client, &snapshot_id, 0, 0x11).await;

    // Overwrite block 0 with a new value (fill byte 0x22)
    put_block(&client, &snapshot_id, 0, 0x22).await;

    // List blocks — still only one block at index 0
    let list_resp = client
        .list_snapshot_blocks()
        .snapshot_id(&snapshot_id)
        .send()
        .await
        .expect("list_snapshot_blocks should succeed");

    assert_eq!(
        list_resp.blocks().len(),
        1,
        "Overwriting should not create a duplicate block entry"
    );
    assert_eq!(list_resp.blocks()[0].block_index(), Some(0));

    // Get block and verify it has the second (overwritten) value
    let block_token = list_resp.blocks()[0].block_token().unwrap().to_string();
    let get_resp = client
        .get_snapshot_block()
        .snapshot_id(&snapshot_id)
        .block_index(0)
        .block_token(block_token)
        .send()
        .await
        .expect("get_snapshot_block should succeed");

    let body_bytes = get_resp
        .block_data
        .collect()
        .await
        .expect("should collect block data")
        .into_bytes();
    assert_eq!(
        body_bytes.as_ref(),
        vec![0x22u8; 512 * 1024].as_slice(),
        "Block content should reflect the overwritten data"
    );
}

#[tokio::test]
async fn test_start_snapshot_returns_owner_id() {
    let client = make_client().await;

    let resp = client
        .start_snapshot()
        .volume_size(1)
        .send()
        .await
        .expect("start_snapshot should succeed");

    // owner_id should be populated with the mock account ID
    assert!(
        resp.owner_id().is_some(),
        "owner_id should be present in StartSnapshot response"
    );
    let owner_id = resp.owner_id().unwrap();
    assert!(!owner_id.is_empty(), "owner_id should not be empty");
    // Should be a numeric account ID
    assert!(
        owner_id.chars().all(|c| c.is_ascii_digit()),
        "owner_id should be numeric, got: {owner_id}"
    );
}

#[tokio::test]
async fn test_list_snapshot_blocks_max_results() {
    let client = make_client().await;

    let start_resp = client
        .start_snapshot()
        .volume_size(1)
        .send()
        .await
        .expect("start_snapshot should succeed");

    let snapshot_id = start_resp.snapshot_id().unwrap().to_string();

    // Write 5 blocks at indices 0..4
    for idx in 0..5i32 {
        put_block(&client, &snapshot_id, idx, 0xAA).await;
    }

    // List with max_results=100 (minimum allowed by docs); all 5 blocks should be returned
    let list_resp = client
        .list_snapshot_blocks()
        .snapshot_id(&snapshot_id)
        .max_results(100)
        .send()
        .await
        .expect("list_snapshot_blocks with max_results should succeed");

    assert_eq!(list_resp.blocks().len(), 5);

    // Blocks should be sorted by index
    let blocks = list_resp.blocks();
    for (i, block) in blocks.iter().enumerate() {
        assert_eq!(block.block_index(), Some(i as i32));
    }
}

#[tokio::test]
async fn test_list_changed_blocks_starting_block_index() {
    let client = make_client().await;

    let start_resp = client
        .start_snapshot()
        .volume_size(1)
        .send()
        .await
        .expect("start_snapshot should succeed");

    let snapshot_id = start_resp.snapshot_id().unwrap().to_string();

    // Write blocks at indices 0, 1, 2
    for idx in 0..3i32 {
        put_block(&client, &snapshot_id, idx, 0xBB).await;
    }

    // ListChangedBlocks with starting_block_index=2 — should only return block at index 2
    let list_resp = client
        .list_changed_blocks()
        .second_snapshot_id(&snapshot_id)
        .starting_block_index(2)
        .send()
        .await
        .expect("list_changed_blocks with starting_block_index should succeed");

    let changed_blocks = list_resp.changed_blocks();
    assert_eq!(
        changed_blocks.len(),
        1,
        "Only block at index 2 should be returned when starting_block_index=2"
    );
    assert_eq!(changed_blocks[0].block_index(), Some(2));
    assert!(changed_blocks[0].second_block_token().is_some());
    // Volume size and block size should still be populated
    assert_eq!(list_resp.volume_size(), Some(1));
    assert_eq!(list_resp.block_size(), Some(524288));
}

#[tokio::test]
async fn test_list_changed_blocks_expiry_time_present() {
    let client = make_client().await;

    let start_resp = client
        .start_snapshot()
        .volume_size(1)
        .send()
        .await
        .expect("start_snapshot should succeed");

    let snapshot_id = start_resp.snapshot_id().unwrap().to_string();
    put_block(&client, &snapshot_id, 0, 0xCC).await;

    let list_resp = client
        .list_changed_blocks()
        .second_snapshot_id(&snapshot_id)
        .send()
        .await
        .expect("list_changed_blocks should succeed");

    // ExpiryTime should be present per the API spec
    assert!(
        list_resp.expiry_time().is_some(),
        "expiry_time should be present in ListChangedBlocks response"
    );
}

#[tokio::test]
async fn test_list_snapshot_blocks_expiry_time_present() {
    let client = make_client().await;

    let start_resp = client
        .start_snapshot()
        .volume_size(1)
        .send()
        .await
        .expect("start_snapshot should succeed");

    let snapshot_id = start_resp.snapshot_id().unwrap().to_string();
    put_block(&client, &snapshot_id, 0, 0xDD).await;

    let list_resp = client
        .list_snapshot_blocks()
        .snapshot_id(&snapshot_id)
        .send()
        .await
        .expect("list_snapshot_blocks should succeed");

    // ExpiryTime should be present per the API spec
    assert!(
        list_resp.expiry_time().is_some(),
        "expiry_time should be present in ListSnapshotBlocks response"
    );
}

// ---------------------------------------------------------------------------
// BlobBackedService smoke tests
// ---------------------------------------------------------------------------

mod blob_backed {
    use std::collections::HashMap;
    use std::pin::Pin;
    use std::sync::Arc;

    use bytes::Bytes;
    use tokio::io::AsyncReadExt;
    use winterbaume_core::{
        BlobBackedService, BlobExportEntry, BlobSource, BlobStore, BlobVisitor, MemVfs,
        StatefulService, VfsError,
    };
    use winterbaume_ebs::EbsService;
    use winterbaume_ebs::views::{BlockView, EbsStateView, SnapshotView};

    struct BlobCollector {
        blobs: HashMap<String, Bytes>,
    }

    impl BlobCollector {
        fn new() -> Self {
            Self {
                blobs: HashMap::new(),
            }
        }
    }

    impl BlobVisitor for BlobCollector {
        fn visit(
            &mut self,
            batch: Vec<BlobExportEntry>,
        ) -> Pin<Box<dyn std::future::Future<Output = Result<(), VfsError>> + Send + '_>> {
            Box::pin(async move {
                for mut entry in batch {
                    let mut buf = Vec::new();
                    entry
                        .reader
                        .read_to_end(&mut buf)
                        .await
                        .map_err(VfsError::Io)?;
                    self.blobs.insert(entry.key.clone(), Bytes::from(buf));
                }
                Ok(())
            })
        }
    }

    struct MapBlobSource {
        data: HashMap<String, Bytes>,
    }

    impl BlobSource for MapBlobSource {
        fn fetch(
            &mut self,
            key: String,
        ) -> Pin<
            Box<
                dyn std::future::Future<
                        Output = Result<Box<dyn tokio::io::AsyncRead + Send + Unpin>, VfsError>,
                    > + Send
                    + '_,
            >,
        > {
            Box::pin(async move {
                let bytes = self
                    .data
                    .get(&key)
                    .cloned()
                    .ok_or(VfsError::NotFound(key))?;
                Ok(Box::new(std::io::Cursor::new(bytes))
                    as Box<dyn tokio::io::AsyncRead + Send + Unpin>)
            })
        }
    }

    /// The EbsService internally scopes blobs to `ebs/{account_id}/{region}` via
    /// `BlobStoreMap`. The returned `BlobStore` mirrors that child namespace so that
    /// test helper seeds and assertions use the same path the service reads from.
    fn make_service_with_shared_vfs() -> (EbsService, BlobStore) {
        let vfs = Arc::new(MemVfs::new());
        let svc = EbsService::with_vfs(vfs.clone());
        // Match the child namespace the service uses for account "111111111111" / "us-east-1".
        let blobs = BlobStore::new(vfs, "ebs").child("111111111111/us-east-1");
        (svc, blobs)
    }

    fn seed_view(blob_key: &str) -> EbsStateView {
        let mut blocks = HashMap::new();
        blocks.insert(
            0,
            BlockView {
                block_index: 0,
                block_token: "tok-0".to_string(),
                blob_key: blob_key.to_string(),
                content_length: 512,
                checksum: "abc".to_string(),
                checksum_algorithm: "SHA256".to_string(),
            },
        );
        let mut snapshots = HashMap::new();
        snapshots.insert(
            "snap-001".to_string(),
            SnapshotView {
                snapshot_id: "snap-001".to_string(),
                volume_size: 1,
                block_size: 512,
                description: "test".to_string(),
                status: "completed".to_string(),
                blocks,
            },
        );
        EbsStateView {
            snapshots,
            ..Default::default()
        }
    }

    #[tokio::test]
    async fn roundtrip_snapshot_restore_with_blobs() {
        let (src_svc, src_blobs) = make_service_with_shared_vfs();
        let blob_key = "snap-001/block-0";

        // Seed.
        src_blobs
            .put(blob_key, Bytes::from(vec![0xAA; 512]))
            .await
            .unwrap();
        src_svc
            .restore("111111111111", "us-east-1", seed_view(blob_key))
            .await
            .unwrap();

        // Export.
        let mut collector = BlobCollector::new();
        let view = src_svc
            .snapshot_with_blobs("111111111111", "us-east-1", &mut collector)
            .await
            .unwrap();

        assert_eq!(view.snapshots.len(), 1);
        assert!(collector.blobs.contains_key(blob_key));
        assert_eq!(collector.blobs[blob_key].len(), 512);

        // Import into fresh service.
        let (dst_svc, dst_blobs) = make_service_with_shared_vfs();
        let mut source = MapBlobSource {
            data: collector.blobs,
        };
        dst_svc
            .restore_with_blobs("111111111111", "us-east-1", view, &mut source)
            .await
            .unwrap();

        let dst_view = dst_svc.snapshot("111111111111", "us-east-1").await;
        assert_eq!(dst_view.snapshots.len(), 1);
        assert_eq!(dst_view.snapshots["snap-001"].blocks[&0].blob_key, blob_key);
        let blob = dst_blobs.get(blob_key).await.unwrap();
        assert_eq!(blob.len(), 512);
    }

    #[tokio::test]
    async fn snapshot_with_blobs_empty_state_exports_nothing() {
        let (svc, _) = make_service_with_shared_vfs();
        let mut collector = BlobCollector::new();
        let view = svc
            .snapshot_with_blobs("111111111111", "us-east-1", &mut collector)
            .await
            .unwrap();
        assert!(view.snapshots.is_empty());
        assert!(collector.blobs.is_empty());
    }
}
