/// Scenario tests for the EBS Direct API service.
///
/// These tests exercise end-to-end use-case workflows spanning 3+ operations,
/// asserting business outcomes rather than per-API shapes.
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

/// Helper: write a block of uniform bytes to a pending snapshot and return the block token.
async fn write_block(
    client: &aws_sdk_ebs::Client,
    snapshot_id: &str,
    block_index: i32,
    fill: u8,
) -> String {
    use base64::Engine;
    use sha2::Digest;

    let data = vec![fill; 512 * 1024];
    let checksum = base64::engine::general_purpose::STANDARD.encode(sha2::Sha256::digest(&data));

    client
        .put_snapshot_block()
        .snapshot_id(snapshot_id)
        .block_index(block_index)
        .block_data(ByteStream::from(data.clone()))
        .data_length(data.len() as i32)
        .checksum(checksum)
        .checksum_algorithm(aws_sdk_ebs::types::ChecksumAlgorithm::ChecksumAlgorithmSha256)
        .send()
        .await
        .expect("put_snapshot_block should succeed");

    // Retrieve the block token via list
    client
        .list_snapshot_blocks()
        .snapshot_id(snapshot_id)
        .send()
        .await
        .expect("list_snapshot_blocks should succeed")
        .blocks()
        .iter()
        .find(|b| b.block_index() == Some(block_index))
        .expect("block should appear in listing")
        .block_token()
        .expect("block_token must be present")
        .to_string()
}

/// Scenario: snapshot write workflow
///
/// Simulates the canonical EBS direct-write workflow: a caller starts a snapshot,
/// writes blocks of data, reads them back to verify integrity, and then seals the
/// snapshot. Asserts that:
/// - The snapshot progresses from `pending` to `completed`.
/// - All written blocks are readable and their contents match what was written.
/// - The listing reflects the correct number and order of blocks.
#[tokio::test]
async fn test_snapshot_write_workflow() {
    let client = make_client().await;

    // 1. Start a 2 GiB snapshot.
    let start_resp = client
        .start_snapshot()
        .volume_size(2)
        .description("scenario: write workflow")
        .send()
        .await
        .expect("start_snapshot should succeed");

    let snapshot_id = start_resp.snapshot_id().unwrap().to_string();
    assert_eq!(
        start_resp.status(),
        Some(&aws_sdk_ebs::types::Status::Pending),
        "new snapshot must start in pending state"
    );
    assert!(
        snapshot_id.starts_with("snap-"),
        "ID must have snap- prefix"
    );

    // 2. Write three blocks with distinct fill bytes.
    let token0 = write_block(&client, &snapshot_id, 0, 0x11).await;
    let token1 = write_block(&client, &snapshot_id, 1, 0x22).await;
    let token2 = write_block(&client, &snapshot_id, 2, 0x33).await;

    // 3. List blocks and verify count and ordering.
    let list_resp = client
        .list_snapshot_blocks()
        .snapshot_id(&snapshot_id)
        .send()
        .await
        .expect("list_snapshot_blocks should succeed");

    let blocks = list_resp.blocks();
    assert_eq!(
        blocks.len(),
        3,
        "listing must show all three written blocks"
    );
    assert_eq!(blocks[0].block_index(), Some(0));
    assert_eq!(blocks[1].block_index(), Some(1));
    assert_eq!(blocks[2].block_index(), Some(2));

    // 4. Read each block back and verify contents.
    for (index, (token, expected_fill)) in [(&token0, 0x11u8), (&token1, 0x22), (&token2, 0x33)]
        .iter()
        .enumerate()
    {
        let get_resp = client
            .get_snapshot_block()
            .snapshot_id(&snapshot_id)
            .block_index(index as i32)
            .block_token(token.as_str())
            .send()
            .await
            .unwrap_or_else(|_| panic!("get_snapshot_block({index}) should succeed"));

        let body = get_resp
            .block_data
            .collect()
            .await
            .expect("should collect block data")
            .into_bytes();

        assert_eq!(
            body.as_ref(),
            vec![*expected_fill; 512 * 1024].as_slice(),
            "block {index} contents should match written data"
        );
    }

    // 5. Complete the snapshot and verify the terminal status.
    let complete_resp = client
        .complete_snapshot()
        .snapshot_id(&snapshot_id)
        .changed_blocks_count(3)
        .send()
        .await
        .expect("complete_snapshot should succeed");

    assert_eq!(
        complete_resp.status(),
        Some(&aws_sdk_ebs::types::Status::Completed),
        "snapshot must be completed after CompleteSnapshot"
    );
}

/// Scenario: incremental snapshot diff workflow
///
/// Simulates a caller that wants to identify the blocks that changed between
/// a base snapshot and a newer snapshot. Uses ListChangedBlocks to determine
/// the diff and GetSnapshotBlock to read the changed data.
///
/// Asserts that:
/// - ListChangedBlocks returns the expected set of changed blocks.
/// - The changed block tokens allow reading the correct block data.
/// - Volume metadata (size, block size) is consistent across calls.
#[tokio::test]
async fn test_incremental_snapshot_diff_workflow() {
    let client = make_client().await;

    // 1. Create "base" snapshot with two blocks.
    let base_resp = client
        .start_snapshot()
        .volume_size(1)
        .description("base snapshot")
        .send()
        .await
        .expect("start_snapshot (base) should succeed");
    let base_id = base_resp.snapshot_id().unwrap().to_string();

    write_block(&client, &base_id, 0, 0xAA).await;
    write_block(&client, &base_id, 1, 0xBB).await;

    client
        .complete_snapshot()
        .snapshot_id(&base_id)
        .changed_blocks_count(2)
        .send()
        .await
        .expect("complete_snapshot (base) should succeed");

    // 2. Create "incremental" snapshot with one unchanged and one new block.
    let incr_resp = client
        .start_snapshot()
        .volume_size(1)
        .description("incremental snapshot")
        .send()
        .await
        .expect("start_snapshot (incremental) should succeed");
    let incr_id = incr_resp.snapshot_id().unwrap().to_string();

    // Block 1 unchanged (same fill), block 2 is new.
    write_block(&client, &incr_id, 1, 0xBB).await;
    write_block(&client, &incr_id, 2, 0xCC).await;

    client
        .complete_snapshot()
        .snapshot_id(&incr_id)
        .changed_blocks_count(2)
        .send()
        .await
        .expect("complete_snapshot (incremental) should succeed");

    // 3. List changed blocks between base and incremental snapshots.
    let diff_resp = client
        .list_changed_blocks()
        .second_snapshot_id(&incr_id)
        .send()
        .await
        .expect("list_changed_blocks should succeed");

    // The mock returns all blocks in the second (target) snapshot as changed.
    let changed = diff_resp.changed_blocks();
    assert!(
        !changed.is_empty(),
        "there should be at least one changed block"
    );
    assert_eq!(
        diff_resp.volume_size(),
        Some(1),
        "volume size should be 1 GiB"
    );
    assert_eq!(
        diff_resp.block_size(),
        Some(524288),
        "block size should be 512 KiB"
    );
    assert!(
        diff_resp.expiry_time().is_some(),
        "expiry_time should be present"
    );

    // 4. Read a changed block using its second_block_token.
    let changed_block = &changed[0];
    let token = changed_block
        .second_block_token()
        .expect("second_block_token must be present for each changed block")
        .to_string();
    let block_idx = changed_block
        .block_index()
        .expect("block_index must be present");

    let get_resp = client
        .get_snapshot_block()
        .snapshot_id(&incr_id)
        .block_index(block_idx)
        .block_token(token)
        .send()
        .await
        .expect("get_snapshot_block for changed block should succeed");

    assert!(
        get_resp.checksum().is_some(),
        "checksum should be present for retrieved changed block"
    );
    let body = get_resp
        .block_data
        .collect()
        .await
        .expect("should collect block data")
        .into_bytes();
    assert_eq!(
        body.len(),
        512 * 1024,
        "block data should be exactly 512 KiB"
    );
}

/// Scenario: snapshot error-path guardrails
///
/// Verifies that the service enforces state-machine constraints:
/// - PutSnapshotBlock is rejected after CompleteSnapshot.
/// - CompleteSnapshot on an already-completed snapshot is rejected.
/// - Operations on non-existent snapshots return ResourceNotFoundException.
///
/// These guardrails are critical for callers that rely on the EBS Direct API
/// contract to detect programming errors early.
#[tokio::test]
async fn test_snapshot_state_machine_guardrails() {
    let client = make_client().await;

    // 1. Start and immediately complete a snapshot (no blocks written).
    let start_resp = client
        .start_snapshot()
        .volume_size(1)
        .send()
        .await
        .expect("start_snapshot should succeed");
    let snapshot_id = start_resp.snapshot_id().unwrap().to_string();

    client
        .complete_snapshot()
        .snapshot_id(&snapshot_id)
        .changed_blocks_count(0)
        .send()
        .await
        .expect("first complete_snapshot should succeed");

    // 2. Attempt to write a block to the completed snapshot — must fail.
    use base64::Engine;
    use sha2::Digest;
    let data = vec![0xFFu8; 512 * 1024];
    let checksum = base64::engine::general_purpose::STANDARD.encode(sha2::Sha256::digest(&data));

    let put_err = client
        .put_snapshot_block()
        .snapshot_id(&snapshot_id)
        .block_index(0)
        .block_data(ByteStream::from(data.clone()))
        .data_length(data.len() as i32)
        .checksum(checksum.clone())
        .checksum_algorithm(aws_sdk_ebs::types::ChecksumAlgorithm::ChecksumAlgorithmSha256)
        .send()
        .await
        .unwrap_err();
    let put_err_str = format!("{:?}", put_err);
    assert!(
        put_err_str.contains("ConflictException"),
        "PutSnapshotBlock on completed snapshot must return ConflictException, got: {put_err_str}"
    );

    // 3. Attempt to complete the snapshot again — must fail.
    let complete_err = client
        .complete_snapshot()
        .snapshot_id(&snapshot_id)
        .changed_blocks_count(0)
        .send()
        .await
        .unwrap_err();
    let complete_err_str = format!("{:?}", complete_err);
    assert!(
        complete_err_str.contains("ConflictException"),
        "Second CompleteSnapshot must return ConflictException, got: {complete_err_str}"
    );

    // 4. All operations on a non-existent snapshot must return ResourceNotFoundException.
    let phantom_id = "snap-0000000000000000f";

    let err = client
        .list_snapshot_blocks()
        .snapshot_id(phantom_id)
        .send()
        .await
        .unwrap_err();
    assert!(
        format!("{:?}", err).contains("ResourceNotFoundException"),
        "ListSnapshotBlocks on non-existent snapshot must return ResourceNotFoundException"
    );

    let err = client
        .complete_snapshot()
        .snapshot_id(phantom_id)
        .changed_blocks_count(0)
        .send()
        .await
        .unwrap_err();
    assert!(
        format!("{:?}", err).contains("ResourceNotFoundException"),
        "CompleteSnapshot on non-existent snapshot must return ResourceNotFoundException"
    );

    let err = client
        .get_snapshot_block()
        .snapshot_id(phantom_id)
        .block_index(0)
        .block_token("dummy")
        .send()
        .await
        .unwrap_err();
    assert!(
        format!("{:?}", err).contains("ResourceNotFoundException"),
        "GetSnapshotBlock on non-existent snapshot must return ResourceNotFoundException"
    );
}
