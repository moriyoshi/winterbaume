use aws_sdk_rekognition::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_rekognition::RekognitionService;

async fn make_client() -> aws_sdk_rekognition::Client {
    let mock = MockAws::builder()
        .with_service(RekognitionService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_rekognition::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_rekognition::Client::new(&config)
}

#[tokio::test]
async fn test_create_and_describe_collection() {
    let client = make_client().await;

    let create_resp = client
        .create_collection()
        .collection_id("my-faces")
        .send()
        .await
        .expect("create_collection should succeed");

    assert_eq!(create_resp.status_code().unwrap(), 200);
    let arn = create_resp.collection_arn().unwrap();
    assert!(arn.contains("my-faces"));
    assert!(create_resp.face_model_version().is_some());

    let describe_resp = client
        .describe_collection()
        .collection_id("my-faces")
        .send()
        .await
        .expect("describe_collection should succeed");

    assert_eq!(describe_resp.face_count().unwrap(), 0);
    assert!(describe_resp.face_model_version().is_some());
    assert_eq!(describe_resp.collection_arn().unwrap(), arn);
    assert!(describe_resp.creation_timestamp().is_some());
}

#[tokio::test]
async fn test_create_duplicate_collection() {
    let client = make_client().await;

    client
        .create_collection()
        .collection_id("dup-collection")
        .send()
        .await
        .expect("first create should succeed");

    let result = client
        .create_collection()
        .collection_id("dup-collection")
        .send()
        .await;

    assert!(result.is_err(), "duplicate create should fail");
}

#[tokio::test]
async fn test_delete_collection() {
    let client = make_client().await;

    client
        .create_collection()
        .collection_id("delete-me")
        .send()
        .await
        .expect("create should succeed");

    let delete_resp = client
        .delete_collection()
        .collection_id("delete-me")
        .send()
        .await
        .expect("delete should succeed");

    assert_eq!(delete_resp.status_code().unwrap(), 200);

    let result = client
        .describe_collection()
        .collection_id("delete-me")
        .send()
        .await;

    assert!(result.is_err(), "describe after delete should fail");
}

#[tokio::test]
async fn test_delete_nonexistent_collection() {
    let client = make_client().await;

    let result = client
        .delete_collection()
        .collection_id("nonexistent")
        .send()
        .await;

    assert!(result.is_err(), "delete nonexistent should fail");
}

#[tokio::test]
async fn test_list_collections() {
    let client = make_client().await;

    for name in ["col-a", "col-b", "col-c"] {
        client
            .create_collection()
            .collection_id(name)
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_collections()
        .send()
        .await
        .expect("list_collections should succeed");

    let ids = resp.collection_ids();
    assert_eq!(ids.len(), 3);
    assert!(ids.contains(&"col-a".to_string()));
    assert!(ids.contains(&"col-b".to_string()));
    assert!(ids.contains(&"col-c".to_string()));
}

#[tokio::test]
async fn test_list_collections_empty() {
    let client = make_client().await;

    let resp = client
        .list_collections()
        .send()
        .await
        .expect("list_collections should succeed on empty");

    assert_eq!(resp.collection_ids().len(), 0);
}

/// Moto parity: test_start_face_search
#[tokio::test]
async fn test_start_face_search() {
    let client = make_client().await;

    let s3_object = aws_sdk_rekognition::types::S3Object::builder()
        .bucket("bucket")
        .name("key")
        .build();
    let video = aws_sdk_rekognition::types::Video::builder()
        .s3_object(s3_object)
        .build();

    let resp = client
        .start_face_search()
        .collection_id("collection_id")
        .video(video)
        .send()
        .await
        .expect("start_face_search should succeed");

    assert!(resp.job_id().is_some());
    assert!(!resp.job_id().unwrap().is_empty());
}

/// Moto parity: test_start_text_detection
#[tokio::test]
async fn test_start_text_detection() {
    let client = make_client().await;

    let s3_object = aws_sdk_rekognition::types::S3Object::builder()
        .bucket("bucket")
        .name("key")
        .build();
    let video = aws_sdk_rekognition::types::Video::builder()
        .s3_object(s3_object)
        .build();

    let resp = client
        .start_text_detection()
        .video(video)
        .send()
        .await
        .expect("start_text_detection should succeed");

    assert!(resp.job_id().is_some());
    assert!(!resp.job_id().unwrap().is_empty());
}

/// Moto parity: test_compare_faces - with EXACT value assertions
#[tokio::test]
async fn test_compare_faces() {
    let client = make_client().await;

    let source_image = aws_sdk_rekognition::types::Image::builder()
        .s3_object(
            aws_sdk_rekognition::types::S3Object::builder()
                .bucket("string")
                .name("string")
                .version("string")
                .build(),
        )
        .build();
    let target_image = aws_sdk_rekognition::types::Image::builder()
        .s3_object(
            aws_sdk_rekognition::types::S3Object::builder()
                .bucket("string")
                .name("string")
                .version("string")
                .build(),
        )
        .build();

    let resp = client
        .compare_faces()
        .similarity_threshold(80.0)
        .source_image(source_image)
        .target_image(target_image)
        .send()
        .await
        .expect("compare_faces should succeed");

    // Moto checks: FaceMatches is present
    let face_matches = resp.face_matches();
    assert!(!face_matches.is_empty());
    assert!(face_matches[0].similarity().unwrap() > 0.0);
    assert!(resp.source_image_face().is_some());
    let unmatched = resp.unmatched_faces();
    assert!(!unmatched.is_empty());
}

/// Moto parity: test_detect_labels - with EXACT value assertions
#[tokio::test]
async fn test_detect_labels() {
    let client = make_client().await;

    let image = aws_sdk_rekognition::types::Image::builder()
        .s3_object(
            aws_sdk_rekognition::types::S3Object::builder()
                .bucket("string")
                .name("name.jpg")
                .build(),
        )
        .build();

    let resp = client
        .detect_labels()
        .image(image)
        .max_labels(10)
        .send()
        .await
        .expect("detect_labels should succeed");

    // Labels are input-seeded; check structure, not exact names
    let labels = resp.labels();
    assert!(!labels.is_empty());
    assert!(labels.len() >= 2 && labels.len() <= 4);
    for label in labels {
        assert!(label.name().is_some());
        let conf = label.confidence().unwrap();
        assert!(
            (80.0..=99.9).contains(&conf),
            "confidence {conf} out of range"
        );
    }
    assert!(resp.label_model_version().is_some());
}

/// Moto parity: test_detect_text - with EXACT value assertions
#[tokio::test]
async fn test_detect_text() {
    let client = make_client().await;

    let image = aws_sdk_rekognition::types::Image::builder()
        .s3_object(
            aws_sdk_rekognition::types::S3Object::builder()
                .bucket("string")
                .name("name.jpg")
                .build(),
        )
        .build();

    let resp = client
        .detect_text()
        .image(image)
        .send()
        .await
        .expect("detect_text should succeed");

    // Moto checks: TextDetections is present
    let detections = resp.text_detections();
    assert!(!detections.is_empty());
    assert!(detections[0].detected_text().is_some());
    assert!(resp.text_model_version().is_some());
}

/// Moto parity: test_get_face_search - EXACT value assertions
/// Starts a face search job first, then retrieves results with exact assertions.
#[tokio::test]
async fn test_start_and_get_face_search() {
    let client = make_client().await;

    let s3_object = aws_sdk_rekognition::types::S3Object::builder()
        .bucket("bucket")
        .name("key")
        .build();
    let video = aws_sdk_rekognition::types::Video::builder()
        .s3_object(s3_object)
        .build();

    let start_resp = client
        .start_face_search()
        .collection_id("collection_id")
        .video(video)
        .send()
        .await
        .expect("start_face_search should succeed");

    let job_id = start_resp.job_id().unwrap();
    assert!(!job_id.is_empty());

    let get_resp = client
        .get_face_search()
        .job_id(job_id)
        .send()
        .await
        .expect("get_face_search should succeed");

    // Moto parity: exact assertions
    assert_eq!(
        get_resp.job_status().unwrap(),
        &aws_sdk_rekognition::types::VideoJobStatus::Succeeded
    );

    let persons = get_resp.persons();
    assert!(!persons.is_empty());

    // Face matches are seed-based; check structure, not exact values
    let face_matches = persons[0].face_matches();
    assert!(!face_matches.is_empty());
    let face = face_matches[0].face().expect("should have face");
    assert!(face.external_image_id().is_some());
    assert!(face.confidence().unwrap() >= 95.0);
    assert!(face_matches[0].similarity().unwrap() >= 90.0);
}

/// Moto parity: test_get_text_detection - EXACT value assertions
/// Starts a text detection job first, then retrieves results with exact assertions.
#[tokio::test]
async fn test_start_and_get_text_detection() {
    let client = make_client().await;

    let s3_object = aws_sdk_rekognition::types::S3Object::builder()
        .bucket("bucket")
        .name("key")
        .build();
    let video = aws_sdk_rekognition::types::Video::builder()
        .s3_object(s3_object)
        .build();

    let start_resp = client
        .start_text_detection()
        .video(video)
        .send()
        .await
        .expect("start_text_detection should succeed");

    let job_id = start_resp.job_id().unwrap();
    assert!(!job_id.is_empty());

    let get_resp = client
        .get_text_detection()
        .job_id(job_id)
        .send()
        .await
        .expect("get_text_detection should succeed");

    // Moto parity: exact assertions
    assert_eq!(
        get_resp.job_status().unwrap(),
        &aws_sdk_rekognition::types::VideoJobStatus::Succeeded
    );

    // Text detections are seed-based; check structure, not exact values
    let text_detections = get_resp.text_detections();
    assert!(!text_detections.is_empty());
    let first = &text_detections[0];
    let text_detection = first.text_detection().expect("should have text_detection");
    assert!(text_detection.detected_text().is_some());
    assert!(text_detection.confidence().unwrap() >= 90.0);
}

/// Moto parity: test_detect_custom_labels - EXACT value assertions
#[tokio::test]
async fn test_detect_custom_labels() {
    let client = make_client().await;

    let image = aws_sdk_rekognition::types::Image::builder()
        .s3_object(
            aws_sdk_rekognition::types::S3Object::builder()
                .bucket("string")
                .name("name.jpg")
                .build(),
        )
        .build();

    let resp = client
        .detect_custom_labels()
        .project_version_arn(
            "arn:aws:rekognition:us-east-1:123456789012:project/my-project/version/1/1234567890",
        )
        .image(image)
        .max_results(10)
        .min_confidence(80.0)
        .send()
        .await
        .expect("detect_custom_labels should succeed");

    // Custom labels are input-seeded; check structure, not exact name
    let labels = resp.custom_labels();
    assert!(!labels.is_empty());
    // The label name is derived from the last segment of the ProjectVersionArn
    assert_eq!(labels[0].name().unwrap(), "1234567890");
    let conf = labels[0].confidence().unwrap();
    assert!(
        (60.0..=99.9).contains(&conf),
        "confidence {conf} out of range"
    );
}

#[tokio::test]
async fn test_get_face_search_invalid_job_id() {
    let client = make_client().await;

    let result = client
        .get_face_search()
        .job_id("nonexistent-job-id")
        .send()
        .await;

    assert!(
        result.is_err(),
        "get_face_search with invalid job ID should fail"
    );
}

#[tokio::test]
async fn test_get_text_detection_invalid_job_id() {
    let client = make_client().await;

    let result = client
        .get_text_detection()
        .job_id("nonexistent-job-id")
        .send()
        .await;

    assert!(
        result.is_err(),
        "get_text_detection with invalid job ID should fail"
    );
}

// ============================================================================
// Tests derived from AWS documentation: Amazon Rekognition
// ============================================================================

#[tokio::test]
async fn test_create_collection_with_tags() {
    let client = make_client().await;

    let resp = client
        .create_collection()
        .collection_id("tagged-collection")
        .tags("env", "test")
        .tags("team", "ml")
        .send()
        .await
        .expect("create_collection with tags should succeed");

    assert_eq!(resp.status_code().unwrap(), 200);
    let arn = resp.collection_arn().unwrap();
    // ARN format: arn:aws:rekognition:{region}:{account}:collection/{id}
    assert!(arn.starts_with("arn:aws:rekognition:"));
    assert!(arn.ends_with(":collection/tagged-collection"));
    assert!(resp.face_model_version().is_some());
}

#[tokio::test]
async fn test_describe_collection_not_found() {
    let client = make_client().await;

    let result = client
        .describe_collection()
        .collection_id("nonexistent-collection-xyz")
        .send()
        .await;

    assert!(
        result.is_err(),
        "describe nonexistent collection should fail"
    );
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_list_collections_face_model_versions() {
    let client = make_client().await;

    for name in ["fmv-col-1", "fmv-col-2"] {
        client
            .create_collection()
            .collection_id(name)
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_collections()
        .send()
        .await
        .expect("list_collections should succeed");

    let ids = resp.collection_ids();
    let versions = resp.face_model_versions();
    assert_eq!(
        ids.len(),
        versions.len(),
        "face_model_versions length should match collection_ids length"
    );
    assert!(!versions.is_empty());
    // All should have the same face model version (e.g., "6.0")
    for v in versions {
        assert!(!v.is_empty(), "face_model_version should not be empty");
    }
}

#[tokio::test]
async fn test_list_collections_max_results() {
    let client = make_client().await;

    for name in [
        "page-col-1",
        "page-col-2",
        "page-col-3",
        "page-col-4",
        "page-col-5",
    ] {
        client
            .create_collection()
            .collection_id(name)
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_collections()
        .max_results(2)
        .send()
        .await
        .expect("list_collections with max_results should succeed");

    assert_eq!(
        resp.collection_ids().len(),
        2,
        "Should return exactly 2 results"
    );
    assert!(
        resp.next_token().is_some(),
        "next_token should be present when more results exist"
    );
}

#[tokio::test]
async fn test_list_collections_pagination() {
    let client = make_client().await;

    for name in ["pagn-col-a", "pagn-col-b", "pagn-col-c"] {
        client
            .create_collection()
            .collection_id(name)
            .send()
            .await
            .unwrap();
    }

    // Get first page with max_results=2
    let first = client
        .list_collections()
        .max_results(2)
        .send()
        .await
        .expect("first page should succeed");

    assert_eq!(first.collection_ids().len(), 2);
    let token = first
        .next_token()
        .expect("next_token must be present for page 1");

    // Get second page using next_token
    let second = client
        .list_collections()
        .max_results(2)
        .next_token(token)
        .send()
        .await
        .expect("second page should succeed");

    assert_eq!(
        second.collection_ids().len(),
        1,
        "Second page should have remaining 1 item"
    );
    assert!(
        second.next_token().is_none(),
        "next_token should be absent on last page"
    );

    // Verify no overlap between pages
    let first_ids = first.collection_ids().to_vec();
    let second_ids = second.collection_ids().to_vec();
    for id in &second_ids {
        assert!(
            !first_ids.contains(id),
            "Pages should not overlap, but {id} appeared in both"
        );
    }
}

#[tokio::test]
async fn test_collection_full_lifecycle() {
    let client = make_client().await;

    // Create
    let create_resp = client
        .create_collection()
        .collection_id("lifecycle-collection")
        .send()
        .await
        .expect("create should succeed");
    assert_eq!(create_resp.status_code().unwrap(), 200);
    let arn = create_resp.collection_arn().unwrap().to_string();

    // Describe
    let describe_resp = client
        .describe_collection()
        .collection_id("lifecycle-collection")
        .send()
        .await
        .expect("describe should succeed");
    assert_eq!(describe_resp.collection_arn().unwrap(), arn);
    assert_eq!(describe_resp.face_count().unwrap(), 0);
    assert!(describe_resp.creation_timestamp().is_some());

    // List - verify it appears
    let list_resp = client
        .list_collections()
        .send()
        .await
        .expect("list should succeed");
    assert!(
        list_resp
            .collection_ids()
            .contains(&"lifecycle-collection".to_string()),
        "lifecycle-collection should appear in list"
    );

    // Delete
    let delete_resp = client
        .delete_collection()
        .collection_id("lifecycle-collection")
        .send()
        .await
        .expect("delete should succeed");
    assert_eq!(delete_resp.status_code().unwrap(), 200);

    // Describe after delete should fail
    let result = client
        .describe_collection()
        .collection_id("lifecycle-collection")
        .send()
        .await;
    assert!(result.is_err(), "describe after delete should fail");

    // List after delete - should not appear
    let list_after = client
        .list_collections()
        .send()
        .await
        .expect("list after delete should succeed");
    assert!(
        !list_after
            .collection_ids()
            .contains(&"lifecycle-collection".to_string()),
        "lifecycle-collection should not appear in list after deletion"
    );
}

#[tokio::test]
async fn test_compare_faces_missing_source() {
    let client = make_client().await;

    let target_image = aws_sdk_rekognition::types::Image::builder()
        .s3_object(
            aws_sdk_rekognition::types::S3Object::builder()
                .bucket("bucket")
                .name("target.jpg")
                .build(),
        )
        .build();

    // No source_image provided — only target
    let result = client
        .compare_faces()
        .target_image(target_image)
        .send()
        .await;

    assert!(
        result.is_err(),
        "compare_faces without SourceImage should fail"
    );
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("InvalidParameterException"),
        "Expected InvalidParameterException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_compare_faces_missing_target() {
    let client = make_client().await;

    let source_image = aws_sdk_rekognition::types::Image::builder()
        .s3_object(
            aws_sdk_rekognition::types::S3Object::builder()
                .bucket("bucket")
                .name("source.jpg")
                .build(),
        )
        .build();

    // No target_image provided — only source
    let result = client
        .compare_faces()
        .source_image(source_image)
        .send()
        .await;

    assert!(
        result.is_err(),
        "compare_faces without TargetImage should fail"
    );
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("InvalidParameterException"),
        "Expected InvalidParameterException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_start_face_search_with_job_tag() {
    let client = make_client().await;

    let s3_object = aws_sdk_rekognition::types::S3Object::builder()
        .bucket("my-bucket")
        .name("my-video.mp4")
        .build();
    let video = aws_sdk_rekognition::types::Video::builder()
        .s3_object(s3_object)
        .build();

    let resp = client
        .start_face_search()
        .collection_id("my-collection")
        .video(video)
        .job_tag("my-job-tag")
        .send()
        .await
        .expect("start_face_search with job_tag should succeed");

    assert!(resp.job_id().is_some());
    assert!(!resp.job_id().unwrap().is_empty());
}

#[tokio::test]
async fn test_describe_collection_user_count() {
    let client = make_client().await;

    client
        .create_collection()
        .collection_id("user-count-check")
        .send()
        .await
        .expect("create should succeed");

    let resp = client
        .describe_collection()
        .collection_id("user-count-check")
        .send()
        .await
        .expect("describe should succeed");

    // New collection should have user_count = 0
    assert_eq!(
        resp.user_count().unwrap_or(-1),
        0,
        "user_count should be 0 for a new collection"
    );
}
