use aws_sdk_s3vectors::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_s3vectors::S3VectorsService;

async fn make_client() -> (aws_sdk_s3vectors::Client, MockAws) {
    let svc = S3VectorsService::new();
    let mock = MockAws::builder().with_service(svc).build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_s3vectors::config::Region::new("us-east-1"))
        .load()
        .await;
    let client = aws_sdk_s3vectors::Client::new(&config);
    (client, mock)
}

// ============================================================================
// CreateVectorBucket / GetVectorBucket / DeleteVectorBucket / ListVectorBuckets
// ============================================================================

#[tokio::test]
async fn test_create_and_get_vector_bucket() {
    let (client, _mock) = make_client().await;

    let resp = client
        .create_vector_bucket()
        .vector_bucket_name("my-bucket")
        .send()
        .await
        .expect("create_vector_bucket should succeed");

    assert!(resp.vector_bucket_arn().is_some());

    let get = client
        .get_vector_bucket()
        .vector_bucket_name("my-bucket")
        .send()
        .await
        .expect("get_vector_bucket should succeed");

    let bucket = get.vector_bucket().expect("should have vector_bucket");
    assert_eq!(bucket.vector_bucket_name(), "my-bucket");
    assert!(!bucket.vector_bucket_arn().is_empty());
    let _ = bucket.creation_time(); // non-optional DateTime
}

#[tokio::test]
async fn test_get_vector_bucket_not_found() {
    let (client, _mock) = make_client().await;

    let result = client
        .get_vector_bucket()
        .vector_bucket_name("nonexistent")
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_create_vector_bucket_duplicate() {
    let (client, _mock) = make_client().await;

    client
        .create_vector_bucket()
        .vector_bucket_name("dup-bucket")
        .send()
        .await
        .unwrap();

    let result = client
        .create_vector_bucket()
        .vector_bucket_name("dup-bucket")
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_delete_vector_bucket() {
    let (client, _mock) = make_client().await;

    client
        .create_vector_bucket()
        .vector_bucket_name("del-bucket")
        .send()
        .await
        .unwrap();

    client
        .delete_vector_bucket()
        .vector_bucket_name("del-bucket")
        .send()
        .await
        .expect("delete_vector_bucket should succeed");

    let result = client
        .get_vector_bucket()
        .vector_bucket_name("del-bucket")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_vector_buckets() {
    let (client, _mock) = make_client().await;

    let list = client.list_vector_buckets().send().await.unwrap();
    assert!(list.vector_buckets().is_empty());

    client
        .create_vector_bucket()
        .vector_bucket_name("bucket-a")
        .send()
        .await
        .unwrap();
    client
        .create_vector_bucket()
        .vector_bucket_name("bucket-b")
        .send()
        .await
        .unwrap();

    let list = client.list_vector_buckets().send().await.unwrap();
    assert_eq!(list.vector_buckets().len(), 2);
}

// ============================================================================
// CreateIndex / GetIndex / DeleteIndex / ListIndexes
// ============================================================================

#[tokio::test]
async fn test_create_and_get_index() {
    let (client, _mock) = make_client().await;

    client
        .create_vector_bucket()
        .vector_bucket_name("idx-bucket")
        .send()
        .await
        .unwrap();

    let resp = client
        .create_index()
        .vector_bucket_name("idx-bucket")
        .index_name("my-index")
        .data_type(aws_sdk_s3vectors::types::DataType::Float32)
        .dimension(128)
        .distance_metric(aws_sdk_s3vectors::types::DistanceMetric::Euclidean)
        .send()
        .await
        .expect("create_index should succeed");

    assert!(resp.index_arn().is_some());

    let get = client
        .get_index()
        .vector_bucket_name("idx-bucket")
        .index_name("my-index")
        .send()
        .await
        .expect("get_index should succeed");

    let idx = get.index().expect("should have index");
    assert_eq!(idx.index_name(), "my-index");
    assert_eq!(idx.vector_bucket_name(), "idx-bucket");
    assert_eq!(idx.dimension(), 128);
    assert_eq!(idx.distance_metric().as_str(), "euclidean");
    assert_eq!(idx.data_type().as_str(), "float32");
}

#[tokio::test]
async fn test_create_index_bucket_not_found() {
    let (client, _mock) = make_client().await;

    let result = client
        .create_index()
        .vector_bucket_name("nonexistent-bucket")
        .index_name("my-index")
        .data_type(aws_sdk_s3vectors::types::DataType::Float32)
        .dimension(128)
        .distance_metric(aws_sdk_s3vectors::types::DistanceMetric::Euclidean)
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_index_not_found() {
    let (client, _mock) = make_client().await;
    client
        .create_vector_bucket()
        .vector_bucket_name("b")
        .send()
        .await
        .unwrap();

    let result = client
        .get_index()
        .vector_bucket_name("b")
        .index_name("no-such-index")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_delete_index() {
    let (client, _mock) = make_client().await;

    client
        .create_vector_bucket()
        .vector_bucket_name("del-idx-bucket")
        .send()
        .await
        .unwrap();
    client
        .create_index()
        .vector_bucket_name("del-idx-bucket")
        .index_name("to-delete")
        .data_type(aws_sdk_s3vectors::types::DataType::Float32)
        .dimension(4)
        .distance_metric(aws_sdk_s3vectors::types::DistanceMetric::Cosine)
        .send()
        .await
        .unwrap();

    client
        .delete_index()
        .vector_bucket_name("del-idx-bucket")
        .index_name("to-delete")
        .send()
        .await
        .expect("delete_index should succeed");

    let result = client
        .get_index()
        .vector_bucket_name("del-idx-bucket")
        .index_name("to-delete")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_indexes() {
    let (client, _mock) = make_client().await;

    client
        .create_vector_bucket()
        .vector_bucket_name("list-idx-bucket")
        .send()
        .await
        .unwrap();

    let list = client
        .list_indexes()
        .vector_bucket_name("list-idx-bucket")
        .send()
        .await
        .unwrap();
    assert!(list.indexes().is_empty());

    client
        .create_index()
        .vector_bucket_name("list-idx-bucket")
        .index_name("idx-a")
        .data_type(aws_sdk_s3vectors::types::DataType::Float32)
        .dimension(4)
        .distance_metric(aws_sdk_s3vectors::types::DistanceMetric::Euclidean)
        .send()
        .await
        .unwrap();

    client
        .create_index()
        .vector_bucket_name("list-idx-bucket")
        .index_name("idx-b")
        .data_type(aws_sdk_s3vectors::types::DataType::Float32)
        .dimension(4)
        .distance_metric(aws_sdk_s3vectors::types::DistanceMetric::Euclidean)
        .send()
        .await
        .unwrap();

    let list = client
        .list_indexes()
        .vector_bucket_name("list-idx-bucket")
        .send()
        .await
        .unwrap();
    assert_eq!(list.indexes().len(), 2);
}

// ============================================================================
// PutVectors / GetVectors / DeleteVectors / ListVectors / QueryVectors
// ============================================================================

async fn setup_index(client: &aws_sdk_s3vectors::Client, bucket: &str, index: &str, dim: i32) {
    client
        .create_vector_bucket()
        .vector_bucket_name(bucket)
        .send()
        .await
        .unwrap();
    client
        .create_index()
        .vector_bucket_name(bucket)
        .index_name(index)
        .data_type(aws_sdk_s3vectors::types::DataType::Float32)
        .dimension(dim)
        .distance_metric(aws_sdk_s3vectors::types::DistanceMetric::Euclidean)
        .send()
        .await
        .unwrap();
}

#[tokio::test]
async fn test_put_and_get_vectors() {
    let (client, _mock) = make_client().await;
    setup_index(&client, "vec-bucket", "vec-index", 3).await;

    client
        .put_vectors()
        .vector_bucket_name("vec-bucket")
        .index_name("vec-index")
        .vectors(
            aws_sdk_s3vectors::types::PutInputVector::builder()
                .key("v1")
                .data(aws_sdk_s3vectors::types::VectorData::Float32(vec![
                    1.0, 2.0, 3.0,
                ]))
                .build()
                .unwrap(),
        )
        .vectors(
            aws_sdk_s3vectors::types::PutInputVector::builder()
                .key("v2")
                .data(aws_sdk_s3vectors::types::VectorData::Float32(vec![
                    4.0, 5.0, 6.0,
                ]))
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("put_vectors should succeed");

    let resp = client
        .get_vectors()
        .vector_bucket_name("vec-bucket")
        .index_name("vec-index")
        .keys("v1")
        .keys("v2")
        .return_data(true)
        .send()
        .await
        .expect("get_vectors should succeed");

    assert_eq!(resp.vectors().len(), 2);
    let keys: Vec<&str> = resp.vectors().iter().map(|v| v.key()).collect();
    assert!(keys.contains(&"v1"));
    assert!(keys.contains(&"v2"));
}

#[tokio::test]
async fn test_delete_vectors() {
    let (client, _mock) = make_client().await;
    setup_index(&client, "del-vec-bucket", "del-vec-index", 2).await;

    client
        .put_vectors()
        .vector_bucket_name("del-vec-bucket")
        .index_name("del-vec-index")
        .vectors(
            aws_sdk_s3vectors::types::PutInputVector::builder()
                .key("k1")
                .data(aws_sdk_s3vectors::types::VectorData::Float32(vec![
                    1.0, 0.0,
                ]))
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    client
        .delete_vectors()
        .vector_bucket_name("del-vec-bucket")
        .index_name("del-vec-index")
        .keys("k1")
        .send()
        .await
        .expect("delete_vectors should succeed");

    let resp = client
        .get_vectors()
        .vector_bucket_name("del-vec-bucket")
        .index_name("del-vec-index")
        .keys("k1")
        .send()
        .await
        .unwrap();
    assert!(resp.vectors().is_empty());
}

#[tokio::test]
async fn test_list_vectors() {
    let (client, _mock) = make_client().await;
    setup_index(&client, "list-vec-bucket", "list-vec-index", 2).await;

    client
        .put_vectors()
        .vector_bucket_name("list-vec-bucket")
        .index_name("list-vec-index")
        .vectors(
            aws_sdk_s3vectors::types::PutInputVector::builder()
                .key("a")
                .data(aws_sdk_s3vectors::types::VectorData::Float32(vec![
                    1.0, 0.0,
                ]))
                .build()
                .unwrap(),
        )
        .vectors(
            aws_sdk_s3vectors::types::PutInputVector::builder()
                .key("b")
                .data(aws_sdk_s3vectors::types::VectorData::Float32(vec![
                    0.0, 1.0,
                ]))
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .list_vectors()
        .vector_bucket_name("list-vec-bucket")
        .index_name("list-vec-index")
        .send()
        .await
        .expect("list_vectors should succeed");

    assert_eq!(resp.vectors().len(), 2);
}

#[tokio::test]
async fn test_query_vectors() {
    let (client, _mock) = make_client().await;
    setup_index(&client, "query-bucket", "query-index", 2).await;

    client
        .put_vectors()
        .vector_bucket_name("query-bucket")
        .index_name("query-index")
        .vectors(
            aws_sdk_s3vectors::types::PutInputVector::builder()
                .key("near")
                .data(aws_sdk_s3vectors::types::VectorData::Float32(vec![
                    1.0, 0.0,
                ]))
                .build()
                .unwrap(),
        )
        .vectors(
            aws_sdk_s3vectors::types::PutInputVector::builder()
                .key("far")
                .data(aws_sdk_s3vectors::types::VectorData::Float32(vec![
                    100.0, 100.0,
                ]))
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .query_vectors()
        .vector_bucket_name("query-bucket")
        .index_name("query-index")
        .top_k(1)
        .query_vector(aws_sdk_s3vectors::types::VectorData::Float32(vec![
            1.0, 0.1,
        ]))
        .send()
        .await
        .expect("query_vectors should succeed");

    assert_eq!(resp.vectors().len(), 1);
    assert_eq!(resp.vectors()[0].key(), "near");
}

#[tokio::test]
async fn test_put_vectors_index_not_found() {
    let (client, _mock) = make_client().await;
    client
        .create_vector_bucket()
        .vector_bucket_name("b")
        .send()
        .await
        .unwrap();

    let result = client
        .put_vectors()
        .vector_bucket_name("b")
        .index_name("nonexistent")
        .vectors(
            aws_sdk_s3vectors::types::PutInputVector::builder()
                .key("k")
                .data(aws_sdk_s3vectors::types::VectorData::Float32(vec![1.0]))
                .build()
                .unwrap(),
        )
        .send()
        .await;
    assert!(result.is_err());
}

// ============================================================================
// Policy
// ============================================================================

#[tokio::test]
async fn test_vector_bucket_policy() {
    let (client, _mock) = make_client().await;
    client
        .create_vector_bucket()
        .vector_bucket_name("pol-bucket")
        .send()
        .await
        .unwrap();

    // No policy initially
    let result = client
        .get_vector_bucket_policy()
        .vector_bucket_name("pol-bucket")
        .send()
        .await;
    assert!(result.is_err());

    // Put policy
    client
        .put_vector_bucket_policy()
        .vector_bucket_name("pol-bucket")
        .policy("{\"Version\":\"2012-10-17\"}")
        .send()
        .await
        .expect("put_vector_bucket_policy should succeed");

    let get = client
        .get_vector_bucket_policy()
        .vector_bucket_name("pol-bucket")
        .send()
        .await
        .expect("get_vector_bucket_policy should succeed");
    assert!(get.policy().is_some());

    // Delete policy
    client
        .delete_vector_bucket_policy()
        .vector_bucket_name("pol-bucket")
        .send()
        .await
        .expect("delete_vector_bucket_policy should succeed");

    let result = client
        .get_vector_bucket_policy()
        .vector_bucket_name("pol-bucket")
        .send()
        .await;
    assert!(result.is_err());
}

// ============================================================================
// Tags
// ============================================================================

#[tokio::test]
async fn test_tags_lifecycle() {
    let (client, _mock) = make_client().await;

    let resp = client
        .create_vector_bucket()
        .vector_bucket_name("tag-bucket")
        .send()
        .await
        .unwrap();
    let arn = resp.vector_bucket_arn().unwrap().to_string();

    // Initially empty
    let tags = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .unwrap();
    assert!(tags.tags().is_empty());

    // Add tags
    client
        .tag_resource()
        .resource_arn(&arn)
        .tags("env", "test")
        .tags("team", "platform")
        .send()
        .await
        .expect("tag_resource should succeed");

    let tags = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .unwrap();
    assert_eq!(tags.tags().len(), 2);
    assert_eq!(tags.tags().get("env"), Some(&"test".to_string()));

    // Remove one tag
    client
        .untag_resource()
        .resource_arn(&arn)
        .tag_keys("env")
        .send()
        .await
        .expect("untag_resource should succeed");

    let tags = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .unwrap();
    assert_eq!(tags.tags().len(), 1);
    assert!(!tags.tags().contains_key("env"));
}

// ============================================================================
// Full lifecycle
// ============================================================================

#[tokio::test]
async fn test_full_lifecycle() {
    let (client, _mock) = make_client().await;

    // Create bucket
    let bucket_resp = client
        .create_vector_bucket()
        .vector_bucket_name("lifecycle-bucket")
        .send()
        .await
        .unwrap();
    let bucket_arn = bucket_resp.vector_bucket_arn().unwrap().to_string();

    // List has 1 bucket
    let list = client.list_vector_buckets().send().await.unwrap();
    assert_eq!(list.vector_buckets().len(), 1);

    // Create index
    let idx_resp = client
        .create_index()
        .vector_bucket_name("lifecycle-bucket")
        .index_name("lifecycle-index")
        .data_type(aws_sdk_s3vectors::types::DataType::Float32)
        .dimension(4)
        .distance_metric(aws_sdk_s3vectors::types::DistanceMetric::Cosine)
        .send()
        .await
        .unwrap();
    let idx_arn = idx_resp.index_arn().unwrap().to_string();
    assert!(idx_arn.contains("lifecycle-index"));

    // Put vectors
    client
        .put_vectors()
        .vector_bucket_name("lifecycle-bucket")
        .index_name("lifecycle-index")
        .vectors(
            aws_sdk_s3vectors::types::PutInputVector::builder()
                .key("v1")
                .data(aws_sdk_s3vectors::types::VectorData::Float32(vec![
                    1.0, 0.0, 0.0, 0.0,
                ]))
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    // Query
    let qr = client
        .query_vectors()
        .vector_bucket_name("lifecycle-bucket")
        .index_name("lifecycle-index")
        .top_k(1)
        .query_vector(aws_sdk_s3vectors::types::VectorData::Float32(vec![
            0.9, 0.1, 0.0, 0.0,
        ]))
        .send()
        .await
        .unwrap();
    assert_eq!(qr.vectors().len(), 1);

    // Delete index
    client
        .delete_index()
        .vector_bucket_name("lifecycle-bucket")
        .index_name("lifecycle-index")
        .send()
        .await
        .unwrap();

    // Delete bucket
    client
        .delete_vector_bucket()
        .vector_bucket_name("lifecycle-bucket")
        .send()
        .await
        .unwrap();

    let list = client.list_vector_buckets().send().await.unwrap();
    assert!(list.vector_buckets().is_empty());
    let _ = bucket_arn; // use variable
}

// ============================================================================
// Tests derived from AWS documentation: S3 Vectors
// ============================================================================

#[tokio::test]
async fn test_delete_vector_bucket_not_found() {
    let (client, _mock) = make_client().await;

    let err = client
        .delete_vector_bucket()
        .vector_bucket_name("no-such-bucket")
        .send()
        .await
        .unwrap_err();
    let s = format!("{err:?}");
    assert!(
        s.contains("NotFoundException") || s.contains("NoSuch"),
        "expected not-found error, got: {s}"
    );
}

#[tokio::test]
async fn test_delete_vector_bucket_with_indexes() {
    let (client, _mock) = make_client().await;

    client
        .create_vector_bucket()
        .vector_bucket_name("bkt-with-idx")
        .send()
        .await
        .unwrap();
    client
        .create_index()
        .vector_bucket_name("bkt-with-idx")
        .index_name("some-index")
        .data_type(aws_sdk_s3vectors::types::DataType::Float32)
        .dimension(4)
        .distance_metric(aws_sdk_s3vectors::types::DistanceMetric::Euclidean)
        .send()
        .await
        .unwrap();

    // Must fail: bucket still has an index
    let err = client
        .delete_vector_bucket()
        .vector_bucket_name("bkt-with-idx")
        .send()
        .await
        .unwrap_err();
    let s = format!("{err:?}");
    assert!(
        s.contains("ConflictException"),
        "expected ConflictException, got: {s}"
    );
}

#[tokio::test]
async fn test_create_index_duplicate() {
    let (client, _mock) = make_client().await;

    client
        .create_vector_bucket()
        .vector_bucket_name("dup-idx-bucket")
        .send()
        .await
        .unwrap();
    client
        .create_index()
        .vector_bucket_name("dup-idx-bucket")
        .index_name("dup-index")
        .data_type(aws_sdk_s3vectors::types::DataType::Float32)
        .dimension(4)
        .distance_metric(aws_sdk_s3vectors::types::DistanceMetric::Euclidean)
        .send()
        .await
        .unwrap();

    let err = client
        .create_index()
        .vector_bucket_name("dup-idx-bucket")
        .index_name("dup-index")
        .data_type(aws_sdk_s3vectors::types::DataType::Float32)
        .dimension(4)
        .distance_metric(aws_sdk_s3vectors::types::DistanceMetric::Euclidean)
        .send()
        .await
        .unwrap_err();
    let s = format!("{err:?}");
    assert!(
        s.contains("ConflictException"),
        "expected ConflictException, got: {s}"
    );
}

#[tokio::test]
async fn test_delete_index_not_found() {
    let (client, _mock) = make_client().await;
    client
        .create_vector_bucket()
        .vector_bucket_name("bkt-del-idx")
        .send()
        .await
        .unwrap();

    let err = client
        .delete_index()
        .vector_bucket_name("bkt-del-idx")
        .index_name("nonexistent-index")
        .send()
        .await
        .unwrap_err();
    let s = format!("{err:?}");
    assert!(
        s.contains("NotFoundException"),
        "expected NotFoundException, got: {s}"
    );
}

#[tokio::test]
async fn test_delete_index_cascades_vectors() {
    let (client, _mock) = make_client().await;
    setup_index(&client, "cascade-bucket", "cascade-index", 2).await;

    client
        .put_vectors()
        .vector_bucket_name("cascade-bucket")
        .index_name("cascade-index")
        .vectors(
            aws_sdk_s3vectors::types::PutInputVector::builder()
                .key("v1")
                .data(aws_sdk_s3vectors::types::VectorData::Float32(vec![
                    1.0, 0.0,
                ]))
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    // Delete index (should also remove all vectors)
    client
        .delete_index()
        .vector_bucket_name("cascade-bucket")
        .index_name("cascade-index")
        .send()
        .await
        .unwrap();

    // Recreate the index to verify no vectors remain
    client
        .create_index()
        .vector_bucket_name("cascade-bucket")
        .index_name("cascade-index")
        .data_type(aws_sdk_s3vectors::types::DataType::Float32)
        .dimension(2)
        .distance_metric(aws_sdk_s3vectors::types::DistanceMetric::Euclidean)
        .send()
        .await
        .unwrap();

    let resp = client
        .list_vectors()
        .vector_bucket_name("cascade-bucket")
        .index_name("cascade-index")
        .send()
        .await
        .unwrap();
    assert!(
        resp.vectors().is_empty(),
        "expected no vectors after index deletion and recreation"
    );
}

#[tokio::test]
async fn test_put_vectors_upsert() {
    let (client, _mock) = make_client().await;
    setup_index(&client, "upsert-bucket", "upsert-index", 2).await;

    // Put initial vector
    client
        .put_vectors()
        .vector_bucket_name("upsert-bucket")
        .index_name("upsert-index")
        .vectors(
            aws_sdk_s3vectors::types::PutInputVector::builder()
                .key("k1")
                .data(aws_sdk_s3vectors::types::VectorData::Float32(vec![
                    1.0, 0.0,
                ]))
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    // Overwrite with new data
    client
        .put_vectors()
        .vector_bucket_name("upsert-bucket")
        .index_name("upsert-index")
        .vectors(
            aws_sdk_s3vectors::types::PutInputVector::builder()
                .key("k1")
                .data(aws_sdk_s3vectors::types::VectorData::Float32(vec![
                    0.0, 1.0,
                ]))
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .get_vectors()
        .vector_bucket_name("upsert-bucket")
        .index_name("upsert-index")
        .keys("k1")
        .return_data(true)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.vectors().len(), 1);
    if let Some(aws_sdk_s3vectors::types::VectorData::Float32(vals)) = resp.vectors()[0].data() {
        assert!(
            (vals[0] - 0.0f32).abs() < 1e-5,
            "expected 0.0 after upsert, got {}",
            vals[0]
        );
        assert!(
            (vals[1] - 1.0f32).abs() < 1e-5,
            "expected 1.0 after upsert, got {}",
            vals[1]
        );
    } else {
        panic!("expected float32 data");
    }
}

#[tokio::test]
async fn test_get_vectors_missing_keys_omitted() {
    let (client, _mock) = make_client().await;
    setup_index(&client, "omit-bucket", "omit-index", 2).await;

    client
        .put_vectors()
        .vector_bucket_name("omit-bucket")
        .index_name("omit-index")
        .vectors(
            aws_sdk_s3vectors::types::PutInputVector::builder()
                .key("exists")
                .data(aws_sdk_s3vectors::types::VectorData::Float32(vec![
                    1.0, 0.0,
                ]))
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    // Request both an existing and a nonexistent key
    let resp = client
        .get_vectors()
        .vector_bucket_name("omit-bucket")
        .index_name("omit-index")
        .keys("exists")
        .keys("does-not-exist")
        .send()
        .await
        .unwrap();

    // Only the existing key should be in the response
    assert_eq!(resp.vectors().len(), 1);
    assert_eq!(resp.vectors()[0].key(), "exists");
}

#[tokio::test]
async fn test_get_vectors_no_data_by_default() {
    let (client, _mock) = make_client().await;
    setup_index(&client, "nodata-bucket", "nodata-index", 2).await;

    client
        .put_vectors()
        .vector_bucket_name("nodata-bucket")
        .index_name("nodata-index")
        .vectors(
            aws_sdk_s3vectors::types::PutInputVector::builder()
                .key("v1")
                .data(aws_sdk_s3vectors::types::VectorData::Float32(vec![
                    1.0, 0.0,
                ]))
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    // Do NOT set return_data — should default to false
    let resp = client
        .get_vectors()
        .vector_bucket_name("nodata-bucket")
        .index_name("nodata-index")
        .keys("v1")
        .send()
        .await
        .unwrap();

    assert_eq!(resp.vectors().len(), 1);
    assert!(
        resp.vectors()[0].data().is_none(),
        "data should be None when returnData is not set"
    );
}

#[tokio::test]
async fn test_delete_vectors_nonexistent_succeeds() {
    let (client, _mock) = make_client().await;
    setup_index(&client, "nodel-bucket", "nodel-index", 2).await;

    // Deleting keys that don't exist should succeed silently
    client
        .delete_vectors()
        .vector_bucket_name("nodel-bucket")
        .index_name("nodel-index")
        .keys("ghost-key-1")
        .keys("ghost-key-2")
        .send()
        .await
        .expect("delete_vectors on nonexistent keys should succeed");
}

#[tokio::test]
async fn test_list_vectors_with_return_data() {
    let (client, _mock) = make_client().await;
    setup_index(&client, "listdata-bucket", "listdata-index", 2).await;

    client
        .put_vectors()
        .vector_bucket_name("listdata-bucket")
        .index_name("listdata-index")
        .vectors(
            aws_sdk_s3vectors::types::PutInputVector::builder()
                .key("v1")
                .data(aws_sdk_s3vectors::types::VectorData::Float32(vec![
                    1.0, 2.0,
                ]))
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .list_vectors()
        .vector_bucket_name("listdata-bucket")
        .index_name("listdata-index")
        .return_data(true)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.vectors().len(), 1);
    assert!(
        resp.vectors()[0].data().is_some(),
        "data should be present when returnData=true"
    );
}

#[tokio::test]
async fn test_list_vectors_with_max_results() {
    let (client, _mock) = make_client().await;
    setup_index(&client, "maxres-bucket", "maxres-index", 2).await;

    for i in 0..3u32 {
        client
            .put_vectors()
            .vector_bucket_name("maxres-bucket")
            .index_name("maxres-index")
            .vectors(
                aws_sdk_s3vectors::types::PutInputVector::builder()
                    .key(format!("v{i}"))
                    .data(aws_sdk_s3vectors::types::VectorData::Float32(vec![
                        i as f32, 0.0,
                    ]))
                    .build()
                    .unwrap(),
            )
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_vectors()
        .vector_bucket_name("maxres-bucket")
        .index_name("maxres-index")
        .max_results(2)
        .send()
        .await
        .unwrap();

    assert_eq!(
        resp.vectors().len(),
        2,
        "maxResults=2 should return at most 2 vectors"
    );
}

#[tokio::test]
async fn test_list_vector_buckets_prefix() {
    let (client, _mock) = make_client().await;

    client
        .create_vector_bucket()
        .vector_bucket_name("alpha-one")
        .send()
        .await
        .unwrap();
    client
        .create_vector_bucket()
        .vector_bucket_name("alpha-two")
        .send()
        .await
        .unwrap();
    client
        .create_vector_bucket()
        .vector_bucket_name("beta-one")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_vector_buckets()
        .prefix("alpha")
        .send()
        .await
        .unwrap();

    assert_eq!(resp.vector_buckets().len(), 2);
    for b in resp.vector_buckets() {
        assert!(
            b.vector_bucket_name().starts_with("alpha"),
            "unexpected bucket: {}",
            b.vector_bucket_name()
        );
    }
}

#[tokio::test]
async fn test_list_indexes_prefix() {
    let (client, _mock) = make_client().await;
    client
        .create_vector_bucket()
        .vector_bucket_name("prefix-bucket")
        .send()
        .await
        .unwrap();

    for name in &["abc-one", "abc-two", "xyz-one"] {
        client
            .create_index()
            .vector_bucket_name("prefix-bucket")
            .index_name(*name)
            .data_type(aws_sdk_s3vectors::types::DataType::Float32)
            .dimension(4)
            .distance_metric(aws_sdk_s3vectors::types::DistanceMetric::Euclidean)
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_indexes()
        .vector_bucket_name("prefix-bucket")
        .prefix("abc")
        .send()
        .await
        .unwrap();

    assert_eq!(resp.indexes().len(), 2);
    for idx in resp.indexes() {
        assert!(
            idx.index_name().starts_with("abc"),
            "unexpected index: {}",
            idx.index_name()
        );
    }
}

#[tokio::test]
async fn test_query_vectors_return_distance() {
    let (client, _mock) = make_client().await;
    setup_index(&client, "dist-bucket", "dist-index", 2).await;

    client
        .put_vectors()
        .vector_bucket_name("dist-bucket")
        .index_name("dist-index")
        .vectors(
            aws_sdk_s3vectors::types::PutInputVector::builder()
                .key("v1")
                .data(aws_sdk_s3vectors::types::VectorData::Float32(vec![
                    1.0, 0.0,
                ]))
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .query_vectors()
        .vector_bucket_name("dist-bucket")
        .index_name("dist-index")
        .top_k(1)
        .query_vector(aws_sdk_s3vectors::types::VectorData::Float32(vec![
            1.0, 0.0,
        ]))
        .return_distance(true)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.vectors().len(), 1);
    assert!(
        resp.vectors()[0].distance().is_some(),
        "distance should be present when returnDistance=true"
    );
    // Euclidean distance from [1,0] to [1,0] is 0
    let d = resp.vectors()[0].distance().unwrap();
    assert!(d < 0.001, "distance to itself should be ~0, got {d}");
}

#[tokio::test]
async fn test_query_vectors_empty_index() {
    let (client, _mock) = make_client().await;
    setup_index(&client, "empty-q-bucket", "empty-q-index", 4).await;

    let resp = client
        .query_vectors()
        .vector_bucket_name("empty-q-bucket")
        .index_name("empty-q-index")
        .top_k(5)
        .query_vector(aws_sdk_s3vectors::types::VectorData::Float32(vec![
            1.0, 0.0, 0.0, 0.0,
        ]))
        .send()
        .await
        .expect("query on empty index should succeed");

    assert!(
        resp.vectors().is_empty(),
        "empty index should return empty results"
    );
}

#[tokio::test]
async fn test_tags_on_index() {
    let (client, _mock) = make_client().await;
    client
        .create_vector_bucket()
        .vector_bucket_name("tag-idx-bucket")
        .send()
        .await
        .unwrap();

    let idx_resp = client
        .create_index()
        .vector_bucket_name("tag-idx-bucket")
        .index_name("tag-idx")
        .data_type(aws_sdk_s3vectors::types::DataType::Float32)
        .dimension(4)
        .distance_metric(aws_sdk_s3vectors::types::DistanceMetric::Euclidean)
        .send()
        .await
        .unwrap();

    let idx_arn = idx_resp.index_arn().unwrap().to_string();

    // Initially no tags
    let tags = client
        .list_tags_for_resource()
        .resource_arn(&idx_arn)
        .send()
        .await
        .unwrap();
    assert!(tags.tags().is_empty());

    // Tag the index
    client
        .tag_resource()
        .resource_arn(&idx_arn)
        .tags("purpose", "search")
        .tags("tier", "prod")
        .send()
        .await
        .expect("tag_resource on index should succeed");

    let tags = client
        .list_tags_for_resource()
        .resource_arn(&idx_arn)
        .send()
        .await
        .unwrap();
    assert_eq!(tags.tags().len(), 2);
    assert_eq!(tags.tags().get("purpose"), Some(&"search".to_string()));

    // Remove one tag
    client
        .untag_resource()
        .resource_arn(&idx_arn)
        .tag_keys("tier")
        .send()
        .await
        .expect("untag_resource on index should succeed");

    let tags = client
        .list_tags_for_resource()
        .resource_arn(&idx_arn)
        .send()
        .await
        .unwrap();
    assert_eq!(tags.tags().len(), 1);
    assert!(!tags.tags().contains_key("tier"));
}

// ============================================================================
// State views
// ============================================================================

#[tokio::test]
async fn test_state_change_listener_fires() {
    use std::sync::{Arc, Mutex};

    use winterbaume_core::StatefulService;

    let svc = S3VectorsService::new();
    let events: Arc<Mutex<Vec<(String, String)>>> = Arc::new(Mutex::new(vec![]));
    let events2 = Arc::clone(&events);
    svc.notifier().subscribe(move |account_id, region, _view| {
        events2
            .lock()
            .unwrap()
            .push((account_id.to_string(), region.to_string()));
    });

    svc.restore("123456789012", "us-east-1", Default::default())
        .await
        .unwrap();

    let got = events.lock().unwrap();
    assert_eq!(got.len(), 1);
    assert_eq!(got[0].0, "123456789012");
    assert_eq!(got[0].1, "us-east-1");
}

#[tokio::test]
async fn test_state_snapshot_restore() {
    use winterbaume_core::StatefulService;

    let svc = S3VectorsService::new();

    let view = winterbaume_s3vectors::S3VectorsStateView {
        buckets: {
            let mut m = std::collections::HashMap::new();
            m.insert(
                "restored-bucket".to_string(),
                winterbaume_s3vectors::views::VectorBucketView {
                    name: "restored-bucket".to_string(),
                    arn: "arn:aws:s3vectors:us-east-1:123456789012:bucket/restored-bucket"
                        .to_string(),
                    creation_time: "2024-01-01T00:00:00Z".to_string(),
                },
            );
            m
        },
        ..Default::default()
    };
    svc.restore("123456789012", "us-east-1", view)
        .await
        .unwrap();

    let snap = svc.snapshot("123456789012", "us-east-1").await;
    assert!(snap.buckets.contains_key("restored-bucket"));
}

#[tokio::test]
async fn test_state_merge() {
    use winterbaume_core::StatefulService;

    let svc = S3VectorsService::new();

    // Pre-seed bucket "a"
    let view_a = winterbaume_s3vectors::S3VectorsStateView {
        buckets: {
            let mut m = std::collections::HashMap::new();
            m.insert(
                "a".to_string(),
                winterbaume_s3vectors::views::VectorBucketView {
                    name: "a".to_string(),
                    arn: "arn:aws:s3vectors:us-east-1:123456789012:bucket/a".to_string(),
                    creation_time: "2024-01-01T00:00:00Z".to_string(),
                },
            );
            m
        },
        ..Default::default()
    };
    svc.restore("123456789012", "us-east-1", view_a)
        .await
        .unwrap();

    // Merge in bucket "b"
    let view_b = winterbaume_s3vectors::S3VectorsStateView {
        buckets: {
            let mut m = std::collections::HashMap::new();
            m.insert(
                "b".to_string(),
                winterbaume_s3vectors::views::VectorBucketView {
                    name: "b".to_string(),
                    arn: "arn:aws:s3vectors:us-east-1:123456789012:bucket/b".to_string(),
                    creation_time: "2024-01-02T00:00:00Z".to_string(),
                },
            );
            m
        },
        ..Default::default()
    };
    svc.merge("123456789012", "us-east-1", view_b)
        .await
        .unwrap();

    let snap = svc.snapshot("123456789012", "us-east-1").await;
    // Both "a" and "b" should be present (merge is additive)
    assert!(snap.buckets.contains_key("a"));
    assert!(snap.buckets.contains_key("b"));
}
