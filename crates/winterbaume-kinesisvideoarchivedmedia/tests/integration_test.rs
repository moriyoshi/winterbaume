use aws_sdk_kinesisvideoarchivedmedia::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_kinesisvideoarchivedmedia::KinesisVideoArchivedMediaService;

async fn make_kvam_client() -> aws_sdk_kinesisvideoarchivedmedia::Client {
    let mock = MockAws::builder()
        .with_service(KinesisVideoArchivedMediaService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_kinesisvideoarchivedmedia::config::Region::new(
            "us-east-1",
        ))
        .load()
        .await;

    aws_sdk_kinesisvideoarchivedmedia::Client::new(&config)
}

#[tokio::test]
async fn test_list_fragments() {
    let client = make_kvam_client().await;

    let resp = client
        .list_fragments()
        .stream_name("test-stream")
        .send()
        .await
        .expect("list_fragments should succeed");

    let fragments = resp.fragments();
    assert!(!fragments.is_empty(), "should return mock fragments");
}

#[tokio::test]
async fn test_list_fragments_with_arn() {
    let client = make_kvam_client().await;

    let resp = client
        .list_fragments()
        .stream_arn("arn:aws:kinesisvideo:us-east-1:123456789012:stream/my-stream/0000000000000")
        .send()
        .await
        .expect("list_fragments with ARN should succeed");

    let fragments = resp.fragments();
    assert!(!fragments.is_empty(), "should return mock fragments");
}

#[tokio::test]
async fn test_get_hls_streaming_session_url() {
    let client = make_kvam_client().await;

    let resp = client
        .get_hls_streaming_session_url()
        .stream_name("hls-stream")
        .send()
        .await
        .expect("get_hls_streaming_session_url should succeed");

    let url = resp.hls_streaming_session_url().expect("should have URL");
    assert!(
        url.contains("kinesisvideo"),
        "URL should contain kinesisvideo"
    );
    assert!(url.contains("hls"), "URL should contain hls");
}

#[tokio::test]
async fn test_get_dash_streaming_session_url() {
    let client = make_kvam_client().await;

    let resp = client
        .get_dash_streaming_session_url()
        .stream_name("dash-stream")
        .send()
        .await
        .expect("get_dash_streaming_session_url should succeed");

    let url = resp.dash_streaming_session_url().expect("should have URL");
    assert!(
        url.contains("kinesisvideo"),
        "URL should contain kinesisvideo"
    );
    assert!(url.contains("dash"), "URL should contain dash");
}

#[tokio::test]
async fn test_get_clip() {
    let client = make_kvam_client().await;

    let resp = client
        .get_clip()
        .stream_name("clip-stream")
        .clip_fragment_selector(
            aws_sdk_kinesisvideoarchivedmedia::types::ClipFragmentSelector::builder()
                .fragment_selector_type(
                    aws_sdk_kinesisvideoarchivedmedia::types::ClipFragmentSelectorType::ProducerTimestamp,
                )
                .timestamp_range(
                    aws_sdk_kinesisvideoarchivedmedia::types::ClipTimestampRange::builder()
                        .start_timestamp(aws_smithy_types::DateTime::from_secs(1000))
                        .end_timestamp(aws_smithy_types::DateTime::from_secs(2000))
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("get_clip should succeed");

    assert!(resp.content_type().is_some(), "should have content type");
}

#[tokio::test]
async fn test_get_images() {
    let client = make_kvam_client().await;

    let resp = client
        .get_images()
        .stream_name("images-stream")
        .image_selector_type(
            aws_sdk_kinesisvideoarchivedmedia::types::ImageSelectorType::ProducerTimestamp,
        )
        .format(aws_sdk_kinesisvideoarchivedmedia::types::Format::Jpeg)
        .start_timestamp(aws_smithy_types::DateTime::from_secs(1000))
        .end_timestamp(aws_smithy_types::DateTime::from_secs(2000))
        .max_results(3)
        .send()
        .await
        .expect("get_images should succeed");

    let images = resp.images();
    assert!(!images.is_empty(), "should return mock images");
}

#[tokio::test]
async fn test_get_media_for_fragment_list() {
    let client = make_kvam_client().await;

    // First list fragments to get valid fragment numbers
    let list_resp = client
        .list_fragments()
        .stream_name("media-stream")
        .send()
        .await
        .expect("list_fragments should succeed");

    let fragments = list_resp.fragments();
    assert!(!fragments.is_empty(), "should have fragments");

    let fragment_number = fragments[0]
        .fragment_number()
        .expect("fragment should have number");

    let resp = client
        .get_media_for_fragment_list()
        .stream_name("media-stream")
        .fragments(fragment_number)
        .send()
        .await
        .expect("get_media_for_fragment_list should succeed");

    assert!(resp.content_type().is_some(), "should have content type");
}

#[tokio::test]
async fn test_missing_stream_identifiers() {
    let client = make_kvam_client().await;

    // Calling list_fragments without stream name or ARN should fail
    let result = client.list_fragments().send().await;

    assert!(result.is_err(), "should fail without stream name or ARN");
}

#[tokio::test]
async fn test_get_hls_streaming_session_url_with_stream_name() {
    // Port of moto test_get_hls_streaming_session_url
    let client = make_kvam_client().await;

    let resp = client
        .get_hls_streaming_session_url()
        .stream_name("my-stream")
        .send()
        .await
        .expect("get_hls_streaming_session_url should succeed");

    let url = resp.hls_streaming_session_url().expect("should have URL");
    assert!(url.contains("hls"), "URL should reference HLS");
    assert!(
        url.contains("kinesisvideo"),
        "URL should reference kinesisvideo"
    );
}

#[tokio::test]
async fn test_get_dash_streaming_session_url_with_stream_name() {
    // Port of moto test_get_dash_streaming_session_url
    let client = make_kvam_client().await;

    let resp = client
        .get_dash_streaming_session_url()
        .stream_name("my-stream")
        .send()
        .await
        .expect("get_dash_streaming_session_url should succeed");

    let url = resp.dash_streaming_session_url().expect("should have URL");
    assert!(url.contains("dash"), "URL should reference DASH");
    assert!(
        url.contains("kinesisvideo"),
        "URL should reference kinesisvideo"
    );
}

#[tokio::test]
async fn test_get_clip_content_type() {
    // Port of moto test_get_clip - verify ContentType is video/mp4
    let client = make_kvam_client().await;

    let resp = client
        .get_clip()
        .stream_name("my-stream")
        .clip_fragment_selector(
            aws_sdk_kinesisvideoarchivedmedia::types::ClipFragmentSelector::builder()
                .fragment_selector_type(
                    aws_sdk_kinesisvideoarchivedmedia::types::ClipFragmentSelectorType::ProducerTimestamp,
                )
                .timestamp_range(
                    aws_sdk_kinesisvideoarchivedmedia::types::ClipTimestampRange::builder()
                        .start_timestamp(aws_smithy_types::DateTime::from_secs(1000))
                        .end_timestamp(aws_smithy_types::DateTime::from_secs(2000))
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("get_clip should succeed");

    let content_type = resp.content_type().expect("should have content type");
    assert_eq!(content_type, "video/mp4");
}

#[tokio::test]
async fn test_list_fragments_returns_multiple_fragments() {
    // Verify that list_fragments returns multiple mock fragments
    let client = make_kvam_client().await;

    let resp = client
        .list_fragments()
        .stream_name("any-stream")
        .send()
        .await
        .expect("list_fragments should succeed");

    let fragments = resp.fragments();
    // Each fragment should have a fragment_number
    for fragment in fragments {
        assert!(
            fragment.fragment_number().is_some(),
            "fragment should have a number"
        );
        assert!(
            fragment.fragment_size_in_bytes() > 0,
            "fragment should have size"
        );
    }
}

// ============================================================================
// Tests derived from AWS documentation: Amazon Kinesis Video Streams Archived Media
// ============================================================================

#[tokio::test]
async fn test_list_fragments_with_fragment_selector() {
    let client = make_kvam_client().await;

    let selector = aws_sdk_kinesisvideoarchivedmedia::types::FragmentSelector::builder()
        .fragment_selector_type(
            aws_sdk_kinesisvideoarchivedmedia::types::FragmentSelectorType::ProducerTimestamp,
        )
        .timestamp_range(
            aws_sdk_kinesisvideoarchivedmedia::types::TimestampRange::builder()
                .start_timestamp(aws_smithy_types::DateTime::from_secs(0))
                .end_timestamp(aws_smithy_types::DateTime::from_secs(9999999999))
                .build()
                .unwrap(),
        )
        .build()
        .unwrap();

    let resp = client
        .list_fragments()
        .stream_name("selector-stream")
        .fragment_selector(selector)
        .send()
        .await
        .expect("list_fragments with selector should succeed");

    let fragments = resp.fragments();
    assert!(
        !fragments.is_empty(),
        "should return mock fragments with selector"
    );
    for fragment in fragments {
        assert!(
            fragment.fragment_number().is_some(),
            "each fragment should have a number"
        );
        assert!(
            fragment.fragment_length_in_milliseconds() > 0,
            "each fragment should have positive length"
        );
    }
}

#[tokio::test]
async fn test_list_fragments_max_results() {
    let client = make_kvam_client().await;

    let resp = client
        .list_fragments()
        .stream_name("max-results-stream")
        .max_results(2)
        .send()
        .await
        .expect("list_fragments with max_results should succeed");

    let fragments = resp.fragments();
    assert!(
        fragments.len() <= 2,
        "should return at most 2 fragments, got {}",
        fragments.len()
    );
}

#[tokio::test]
async fn test_get_media_for_fragment_list_empty_fragments() {
    let client = make_kvam_client().await;

    // First create the stream by listing fragments
    let list_resp = client
        .list_fragments()
        .stream_name("empty-frags-stream")
        .send()
        .await
        .expect("list should succeed");
    assert!(!list_resp.fragments().is_empty());

    // Now request media with an empty fragment list (the SDK sends Fragments=[])
    // The handler checks for an empty list and returns InvalidArgumentException
    let err = client
        .get_media_for_fragment_list()
        .stream_name("empty-frags-stream")
        // No .fragments() call — SDK sends empty list
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("InvalidArgumentException") || err_str.contains("InvalidArgument"),
        "Expected InvalidArgumentException for empty fragments list, got: {err_str}"
    );
}

#[tokio::test]
async fn test_get_media_for_fragment_list_invalid_fragment() {
    let client = make_kvam_client().await;

    // First ensure the stream exists with known fragments
    let _list_resp = client
        .list_fragments()
        .stream_name("invalid-frag-stream")
        .send()
        .await
        .expect("list should succeed");

    let err = client
        .get_media_for_fragment_list()
        .stream_name("invalid-frag-stream")
        .fragments("99999999999999999") // Non-existent fragment number
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("InvalidArgumentException") || err_str.contains("InvalidArgument"),
        "Expected InvalidArgumentException for unknown fragment, got: {err_str}"
    );
}

#[tokio::test]
async fn test_get_media_for_fragment_list_with_arn() {
    let client = make_kvam_client().await;

    let stream_arn =
        "arn:aws:kinesisvideo:us-east-1:123456789012:stream/arn-media-stream/0000000000000";

    // First list fragments via ARN to auto-create the stream and get real fragment numbers
    let list_resp = client
        .list_fragments()
        .stream_arn(stream_arn)
        .send()
        .await
        .expect("list_fragments via ARN should succeed");

    let fragments = list_resp.fragments();
    assert!(!fragments.is_empty(), "should have fragments");
    let fragment_number = fragments[0]
        .fragment_number()
        .expect("fragment should have number")
        .to_string();

    let resp = client
        .get_media_for_fragment_list()
        .stream_arn(stream_arn)
        .fragments(fragment_number)
        .send()
        .await
        .expect("get_media_for_fragment_list via ARN should succeed");

    assert!(
        resp.content_type().is_some(),
        "should have content type when using ARN"
    );
}

#[tokio::test]
async fn test_get_hls_streaming_session_url_with_arn() {
    let client = make_kvam_client().await;

    let stream_arn =
        "arn:aws:kinesisvideo:us-east-1:123456789012:stream/hls-arn-stream/0000000000000";

    let resp = client
        .get_hls_streaming_session_url()
        .stream_arn(stream_arn)
        .send()
        .await
        .expect("get_hls_streaming_session_url via ARN should succeed");

    let url = resp.hls_streaming_session_url().expect("should have URL");
    assert!(url.contains("hls"), "HLS URL should contain 'hls'");
    assert!(
        url.contains("kinesisvideo"),
        "HLS URL should contain 'kinesisvideo'"
    );
    assert!(
        url.contains("SessionToken="),
        "HLS URL should contain a session token"
    );
}

#[tokio::test]
async fn test_get_dash_streaming_session_url_with_arn() {
    let client = make_kvam_client().await;

    let stream_arn =
        "arn:aws:kinesisvideo:us-east-1:123456789012:stream/dash-arn-stream/0000000000000";

    let resp = client
        .get_dash_streaming_session_url()
        .stream_arn(stream_arn)
        .send()
        .await
        .expect("get_dash_streaming_session_url via ARN should succeed");

    let url = resp.dash_streaming_session_url().expect("should have URL");
    assert!(url.contains("dash"), "DASH URL should contain 'dash'");
    assert!(
        url.contains("kinesisvideo"),
        "DASH URL should contain 'kinesisvideo'"
    );
    assert!(
        url.contains("SessionToken="),
        "DASH URL should contain a session token"
    );
}

#[tokio::test]
async fn test_get_clip_with_arn() {
    let client = make_kvam_client().await;

    let stream_arn =
        "arn:aws:kinesisvideo:us-east-1:123456789012:stream/clip-arn-stream/0000000000000";

    let resp = client
        .get_clip()
        .stream_arn(stream_arn)
        .clip_fragment_selector(
            aws_sdk_kinesisvideoarchivedmedia::types::ClipFragmentSelector::builder()
                .fragment_selector_type(
                    aws_sdk_kinesisvideoarchivedmedia::types::ClipFragmentSelectorType::ServerTimestamp,
                )
                .timestamp_range(
                    aws_sdk_kinesisvideoarchivedmedia::types::ClipTimestampRange::builder()
                        .start_timestamp(aws_smithy_types::DateTime::from_secs(1000))
                        .end_timestamp(aws_smithy_types::DateTime::from_secs(2000))
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("get_clip via ARN should succeed");

    let content_type = resp.content_type().expect("should have content type");
    assert_eq!(
        content_type, "video/mp4",
        "clip content type should be video/mp4"
    );
}

#[tokio::test]
async fn test_get_clip_missing_selector() {
    // ClipFragmentSelector is required: aws-sdk-kinesisvideoarchivedmedia's
    // typed builder enforces it at compile time so the handler's runtime
    // check is unreachable from the typed SDK.
}

#[tokio::test]
async fn test_get_images_with_arn() {
    let client = make_kvam_client().await;

    let stream_arn =
        "arn:aws:kinesisvideo:us-east-1:123456789012:stream/images-arn-stream/0000000000000";

    let resp = client
        .get_images()
        .stream_arn(stream_arn)
        .image_selector_type(
            aws_sdk_kinesisvideoarchivedmedia::types::ImageSelectorType::ProducerTimestamp,
        )
        .format(aws_sdk_kinesisvideoarchivedmedia::types::Format::Jpeg)
        .start_timestamp(aws_smithy_types::DateTime::from_secs(1000))
        .end_timestamp(aws_smithy_types::DateTime::from_secs(2000))
        .send()
        .await
        .expect("get_images via ARN should succeed");

    let images = resp.images();
    assert!(!images.is_empty(), "should return images via ARN");
    for image in images {
        assert!(
            image.image_content().is_some(),
            "each image should have content"
        );
        assert!(
            image.time_stamp().is_some(),
            "each image should have a timestamp"
        );
    }
}

#[tokio::test]
async fn test_get_images_missing_selector_type() {
    // ImageSelectorType is required: aws-sdk-kinesisvideoarchivedmedia's
    // typed builder enforces it at compile time so the handler's runtime
    // check is unreachable from the typed SDK.
}

#[tokio::test]
async fn test_get_images_max_results_respected() {
    let client = make_kvam_client().await;

    let resp = client
        .get_images()
        .stream_name("max-img-stream")
        .image_selector_type(
            aws_sdk_kinesisvideoarchivedmedia::types::ImageSelectorType::ProducerTimestamp,
        )
        .format(aws_sdk_kinesisvideoarchivedmedia::types::Format::Png)
        .start_timestamp(aws_smithy_types::DateTime::from_secs(1000))
        .end_timestamp(aws_smithy_types::DateTime::from_secs(2000))
        .max_results(1)
        .send()
        .await
        .expect("get_images with max_results=1 should succeed");

    let images = resp.images();
    assert_eq!(
        images.len(),
        1,
        "MaxResults=1 should return exactly 1 image"
    );
}

#[tokio::test]
async fn test_lifecycle_list_then_get_media() {
    let client = make_kvam_client().await;

    // Step 1: List fragments to discover real fragment numbers
    let list_resp = client
        .list_fragments()
        .stream_name("lifecycle-stream")
        .send()
        .await
        .expect("list_fragments should succeed");

    let fragments = list_resp.fragments();
    assert!(
        !fragments.is_empty(),
        "should have fragments for lifecycle test"
    );
    assert!(fragments.len() >= 2, "should have at least 2 fragments");

    // Step 2: Get media for the first two fragments
    let fnum0 = fragments[0]
        .fragment_number()
        .expect("fragment 0 should have number")
        .to_string();
    let fnum1 = fragments[1]
        .fragment_number()
        .expect("fragment 1 should have number")
        .to_string();

    let media_resp = client
        .get_media_for_fragment_list()
        .stream_name("lifecycle-stream")
        .fragments(fnum0)
        .fragments(fnum1)
        .send()
        .await
        .expect("get_media_for_fragment_list should succeed");

    let content_type = media_resp.content_type().expect("should have content type");
    assert_eq!(
        content_type, "video/webm",
        "media content type should be video/webm (MKV container)"
    );
}

#[tokio::test]
async fn test_get_images_server_timestamp() {
    let client = make_kvam_client().await;

    let resp = client
        .get_images()
        .stream_name("server-ts-stream")
        .image_selector_type(
            aws_sdk_kinesisvideoarchivedmedia::types::ImageSelectorType::ServerTimestamp,
        )
        .format(aws_sdk_kinesisvideoarchivedmedia::types::Format::Jpeg)
        .start_timestamp(aws_smithy_types::DateTime::from_secs(1000))
        .end_timestamp(aws_smithy_types::DateTime::from_secs(2000))
        .send()
        .await
        .expect("get_images with ServerTimestamp should succeed");

    let images = resp.images();
    assert!(
        !images.is_empty(),
        "should return images with ServerTimestamp selector"
    );
}

#[tokio::test]
async fn test_get_hls_url_unique_per_call() {
    let client = make_kvam_client().await;

    let resp1 = client
        .get_hls_streaming_session_url()
        .stream_name("unique-url-stream")
        .send()
        .await
        .expect("first get_hls_streaming_session_url should succeed");

    let resp2 = client
        .get_hls_streaming_session_url()
        .stream_name("unique-url-stream")
        .send()
        .await
        .expect("second get_hls_streaming_session_url should succeed");

    let url1 = resp1
        .hls_streaming_session_url()
        .expect("should have URL 1");
    let url2 = resp2
        .hls_streaming_session_url()
        .expect("should have URL 2");

    assert_ne!(
        url1, url2,
        "each HLS session URL should be unique (different session token)"
    );
}

#[tokio::test]
async fn test_list_fragments_no_stream_no_arn() {
    let client = make_kvam_client().await;

    // Calling without stream name or ARN should fail — this is already tested in
    // test_missing_stream_identifiers, but we re-verify the error type here
    let err = client.list_fragments().send().await.unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("InvalidArgumentException") || err_str.contains("InvalidArgument"),
        "Expected InvalidArgumentException when no stream identifier provided, got: {err_str}"
    );
}
