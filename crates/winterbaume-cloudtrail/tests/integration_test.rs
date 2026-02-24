use aws_sdk_cloudtrail::config::BehaviorVersion;
use winterbaume_cloudtrail::CloudTrailService;
use winterbaume_core::MockAws;

async fn make_client() -> aws_sdk_cloudtrail::Client {
    let mock = MockAws::builder()
        .with_service(CloudTrailService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_cloudtrail::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_cloudtrail::Client::new(&config)
}

#[tokio::test]
async fn test_create_trail() {
    let client = make_client().await;

    let resp = client
        .create_trail()
        .name("my-trail")
        .s3_bucket_name("my-bucket")
        .send()
        .await
        .expect("create_trail should succeed");

    assert_eq!(resp.name(), Some("my-trail"));
    assert_eq!(resp.s3_bucket_name(), Some("my-bucket"));
    assert!(resp.trail_arn().unwrap().contains("arn:aws:cloudtrail:"));
}

#[tokio::test]
async fn test_describe_trails() {
    let client = make_client().await;

    client
        .create_trail()
        .name("trail-1")
        .s3_bucket_name("bucket-1")
        .send()
        .await
        .unwrap();

    client
        .create_trail()
        .name("trail-2")
        .s3_bucket_name("bucket-2")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_trails()
        .send()
        .await
        .expect("describe_trails should succeed");

    assert_eq!(resp.trail_list().len(), 2);
}

#[tokio::test]
async fn test_delete_trail() {
    let client = make_client().await;

    client
        .create_trail()
        .name("delete-me")
        .s3_bucket_name("bucket")
        .send()
        .await
        .unwrap();

    client
        .delete_trail()
        .name("delete-me")
        .send()
        .await
        .expect("delete_trail should succeed");

    let resp = client.describe_trails().send().await.unwrap();
    assert_eq!(resp.trail_list().len(), 0);
}

#[tokio::test]
async fn test_start_stop_logging() {
    let client = make_client().await;

    client
        .create_trail()
        .name("log-trail")
        .s3_bucket_name("bucket")
        .send()
        .await
        .unwrap();

    client
        .start_logging()
        .name("log-trail")
        .send()
        .await
        .expect("start_logging should succeed");

    let status = client
        .get_trail_status()
        .name("log-trail")
        .send()
        .await
        .expect("get_trail_status should succeed");

    assert_eq!(status.is_logging(), Some(true));

    client
        .stop_logging()
        .name("log-trail")
        .send()
        .await
        .expect("stop_logging should succeed");

    let status = client
        .get_trail_status()
        .name("log-trail")
        .send()
        .await
        .unwrap();

    assert_eq!(status.is_logging(), Some(false));
}

#[tokio::test]
async fn test_get_trail_status_not_logging_by_default() {
    let client = make_client().await;

    client
        .create_trail()
        .name("status-trail")
        .s3_bucket_name("bucket")
        .send()
        .await
        .unwrap();

    let status = client
        .get_trail_status()
        .name("status-trail")
        .send()
        .await
        .expect("get_trail_status should succeed");

    assert_eq!(status.is_logging(), Some(false));
}

#[tokio::test]
async fn test_get_trail_status_nonexistent() {
    let client = make_client().await;

    let result = client
        .get_trail_status()
        .name("nonexistent-trail")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_describe_trails_with_filter() {
    let client = make_client().await;

    for name in ["filter-trail-1", "filter-trail-2", "filter-trail-3"] {
        client
            .create_trail()
            .name(name)
            .s3_bucket_name("bucket")
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .describe_trails()
        .trail_name_list("filter-trail-1")
        .trail_name_list("filter-trail-3")
        .send()
        .await
        .expect("describe_trails should succeed");

    assert_eq!(resp.trail_list().len(), 2);
    let names: Vec<_> = resp
        .trail_list()
        .iter()
        .map(|t| t.name().unwrap())
        .collect();
    assert!(names.contains(&"filter-trail-1"));
    assert!(names.contains(&"filter-trail-3"));
}

#[tokio::test]
async fn test_delete_nonexistent_trail() {
    let client = make_client().await;

    let result = client.delete_trail().name("nonexistent-trail").send().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_trail() {
    let client = make_client().await;

    client
        .create_trail()
        .name("get-trail-test")
        .s3_bucket_name("my-bucket")
        .send()
        .await
        .unwrap();

    let resp = client
        .get_trail()
        .name("get-trail-test")
        .send()
        .await
        .expect("get_trail should succeed");

    let trail = resp.trail().unwrap();
    assert_eq!(trail.name(), Some("get-trail-test"));
    assert_eq!(trail.s3_bucket_name(), Some("my-bucket"));
    assert!(trail.trail_arn().unwrap().contains("arn:aws:cloudtrail:"));
}

#[tokio::test]
async fn test_get_trail_not_found() {
    let client = make_client().await;

    let result = client.get_trail().name("nonexistent").send().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_trails() {
    let client = make_client().await;

    for name in ["list-trail-1", "list-trail-2", "list-trail-3"] {
        client
            .create_trail()
            .name(name)
            .s3_bucket_name("bucket")
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_trails()
        .send()
        .await
        .expect("list_trails should succeed");

    assert_eq!(resp.trails().len(), 3);
    let names: Vec<_> = resp.trails().iter().filter_map(|t| t.name()).collect();
    assert!(names.contains(&"list-trail-1"));
    assert!(names.contains(&"list-trail-2"));
    assert!(names.contains(&"list-trail-3"));
}

#[tokio::test]
async fn test_update_trail() {
    let client = make_client().await;

    client
        .create_trail()
        .name("update-trail-test")
        .s3_bucket_name("old-bucket")
        .send()
        .await
        .unwrap();

    let resp = client
        .update_trail()
        .name("update-trail-test")
        .s3_bucket_name("new-bucket")
        .s3_key_prefix("new-prefix")
        .is_multi_region_trail(true)
        .send()
        .await
        .expect("update_trail should succeed");

    assert_eq!(resp.name(), Some("update-trail-test"));
    assert_eq!(resp.s3_bucket_name(), Some("new-bucket"));
    assert_eq!(resp.s3_key_prefix(), Some("new-prefix"));
    assert_eq!(resp.is_multi_region_trail(), Some(true));
}

#[tokio::test]
async fn test_update_trail_not_found() {
    let client = make_client().await;

    let result = client
        .update_trail()
        .name("nonexistent")
        .s3_bucket_name("bucket")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_add_and_list_tags() {
    use aws_sdk_cloudtrail::types::Tag;

    let client = make_client().await;

    let create_resp = client
        .create_trail()
        .name("tag-trail")
        .s3_bucket_name("bucket")
        .send()
        .await
        .unwrap();
    let trail_arn = create_resp.trail_arn().unwrap().to_string();

    client
        .add_tags()
        .resource_id(&trail_arn)
        .tags_list(Tag::builder().key("env").value("prod").build().unwrap())
        .tags_list(
            Tag::builder()
                .key("team")
                .value("platform")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("add_tags should succeed");

    let resp = client
        .list_tags()
        .resource_id_list(&trail_arn)
        .send()
        .await
        .expect("list_tags should succeed");

    assert_eq!(resp.resource_tag_list().len(), 1);
    let resource_tag = &resp.resource_tag_list()[0];
    assert_eq!(resource_tag.resource_id(), Some(trail_arn.as_str()));
    let tags = resource_tag.tags_list();
    assert_eq!(tags.len(), 2);
}

#[tokio::test]
async fn test_remove_tags() {
    use aws_sdk_cloudtrail::types::Tag;

    let client = make_client().await;

    let create_resp = client
        .create_trail()
        .name("remove-tag-trail")
        .s3_bucket_name("bucket")
        .send()
        .await
        .unwrap();
    let trail_arn = create_resp.trail_arn().unwrap().to_string();

    client
        .add_tags()
        .resource_id(&trail_arn)
        .tags_list(Tag::builder().key("env").value("prod").build().unwrap())
        .tags_list(
            Tag::builder()
                .key("team")
                .value("platform")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    client
        .remove_tags()
        .resource_id(&trail_arn)
        .tags_list(Tag::builder().key("env").build().unwrap())
        .send()
        .await
        .expect("remove_tags should succeed");

    let resp = client
        .list_tags()
        .resource_id_list(&trail_arn)
        .send()
        .await
        .unwrap();

    let tags = resp.resource_tag_list()[0].tags_list();
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0].key(), "team");
}

#[tokio::test]
async fn test_put_and_get_event_selectors() {
    use aws_sdk_cloudtrail::types::{EventSelector, ReadWriteType};

    let client = make_client().await;

    client
        .create_trail()
        .name("es-trail")
        .s3_bucket_name("bucket")
        .send()
        .await
        .unwrap();

    let selector = EventSelector::builder()
        .read_write_type(ReadWriteType::ReadOnly)
        .include_management_events(true)
        .build();

    client
        .put_event_selectors()
        .trail_name("es-trail")
        .event_selectors(selector)
        .send()
        .await
        .expect("put_event_selectors should succeed");

    let resp = client
        .get_event_selectors()
        .trail_name("es-trail")
        .send()
        .await
        .expect("get_event_selectors should succeed");

    assert!(resp.trail_arn().unwrap().contains("es-trail"));
    assert_eq!(resp.event_selectors().len(), 1);
    assert_eq!(
        resp.event_selectors()[0].read_write_type(),
        Some(&ReadWriteType::ReadOnly)
    );
}

#[tokio::test]
async fn test_put_and_get_insight_selectors() {
    use aws_sdk_cloudtrail::types::{InsightSelector, InsightType};

    let client = make_client().await;

    client
        .create_trail()
        .name("insight-trail")
        .s3_bucket_name("bucket")
        .send()
        .await
        .unwrap();

    let selector = InsightSelector::builder()
        .insight_type(InsightType::ApiCallRateInsight)
        .build();

    client
        .put_insight_selectors()
        .trail_name("insight-trail")
        .insight_selectors(selector)
        .send()
        .await
        .expect("put_insight_selectors should succeed");

    let resp = client
        .get_insight_selectors()
        .trail_name("insight-trail")
        .send()
        .await
        .expect("get_insight_selectors should succeed");

    assert!(resp.trail_arn().unwrap().contains("insight-trail"));
    assert_eq!(resp.insight_selectors().len(), 1);
    assert_eq!(
        resp.insight_selectors()[0].insight_type(),
        Some(&InsightType::ApiCallRateInsight)
    );
}

// ============================================================================
// Ported from moto: test_cloudtrail.py, test_cloudtrail_eventselectors.py,
//                    test_cloudtrail_tags.py
// ============================================================================

// Ported from moto: test_cloudtrail.py::test_create_trail_simple
#[tokio::test]
async fn test_create_trail_simple_defaults() {
    let client = make_client().await;

    let resp = client
        .create_trail()
        .name("my-simple-trail")
        .s3_bucket_name("my-bucket")
        .send()
        .await
        .unwrap();

    assert_eq!(resp.name(), Some("my-simple-trail"));
    assert_eq!(resp.s3_bucket_name(), Some("my-bucket"));
    assert_eq!(resp.include_global_service_events(), Some(true));
    assert_eq!(resp.is_multi_region_trail(), Some(false));
    assert!(resp.trail_arn().unwrap().contains("arn:aws:cloudtrail:"));
    assert!(resp.trail_arn().unwrap().contains(":trail/my-simple-trail"));
}

// Ported from moto: test_cloudtrail.py::test_get_trail_status_arn_inactive
#[tokio::test]
async fn test_get_trail_status_by_arn() {
    let client = make_client().await;

    let create_resp = client
        .create_trail()
        .name("arn-status-trail")
        .s3_bucket_name("bucket")
        .send()
        .await
        .unwrap();
    let trail_arn = create_resp.trail_arn().unwrap();

    let status = client
        .get_trail_status()
        .name(trail_arn)
        .send()
        .await
        .expect("get_trail_status by ARN should succeed");

    assert_eq!(status.is_logging(), Some(false));
}

// Ported from moto: test_cloudtrail.py::test_get_trail_status_after_starting
#[tokio::test]
async fn test_get_trail_status_after_start_logging() {
    let client = make_client().await;

    client
        .create_trail()
        .name("start-log-trail")
        .s3_bucket_name("bucket")
        .send()
        .await
        .unwrap();

    client
        .start_logging()
        .name("start-log-trail")
        .send()
        .await
        .unwrap();

    let status = client
        .get_trail_status()
        .name("start-log-trail")
        .send()
        .await
        .unwrap();

    assert_eq!(status.is_logging(), Some(true));
}

// Ported from moto: test_cloudtrail.py::test_get_trail_status_after_starting_and_stopping
#[tokio::test]
async fn test_get_trail_status_after_start_and_stop() {
    let client = make_client().await;

    client
        .create_trail()
        .name("startstop-trail")
        .s3_bucket_name("bucket")
        .send()
        .await
        .unwrap();

    client
        .start_logging()
        .name("startstop-trail")
        .send()
        .await
        .unwrap();

    client
        .stop_logging()
        .name("startstop-trail")
        .send()
        .await
        .unwrap();

    let status = client
        .get_trail_status()
        .name("startstop-trail")
        .send()
        .await
        .unwrap();

    assert_eq!(status.is_logging(), Some(false));
}

// Ported from moto: test_cloudtrail.py::test_describe_trails_without_shadowtrails
#[tokio::test]
async fn test_describe_trails_has_selectors_flags() {
    let client = make_client().await;

    client
        .create_trail()
        .name("selectors-trail")
        .s3_bucket_name("bucket")
        .send()
        .await
        .unwrap();

    // Before setting event selectors, flags should be false
    let resp = client.describe_trails().send().await.unwrap();
    let trail = &resp.trail_list()[0];
    assert_eq!(trail.has_custom_event_selectors(), Some(false));
    assert_eq!(trail.has_insight_selectors(), Some(false));

    // Set event selectors
    use aws_sdk_cloudtrail::types::{EventSelector, ReadWriteType};
    let selector = EventSelector::builder()
        .read_write_type(ReadWriteType::All)
        .include_management_events(true)
        .build();
    client
        .put_event_selectors()
        .trail_name("selectors-trail")
        .event_selectors(selector)
        .send()
        .await
        .unwrap();

    let resp = client.describe_trails().send().await.unwrap();
    let trail = &resp.trail_list()[0];
    assert_eq!(trail.has_custom_event_selectors(), Some(true));
    assert_eq!(trail.has_insight_selectors(), Some(false));

    // Set insight selectors
    use aws_sdk_cloudtrail::types::{InsightSelector, InsightType};
    let is = InsightSelector::builder()
        .insight_type(InsightType::ApiCallRateInsight)
        .build();
    client
        .put_insight_selectors()
        .trail_name("selectors-trail")
        .insight_selectors(is)
        .send()
        .await
        .unwrap();

    let resp = client.describe_trails().send().await.unwrap();
    let trail = &resp.trail_list()[0];
    assert_eq!(trail.has_custom_event_selectors(), Some(true));
    assert_eq!(trail.has_insight_selectors(), Some(true));
}

// Ported from moto: test_cloudtrail_eventselectors.py::test_get_event_selectors_empty
#[tokio::test]
async fn test_get_event_selectors_empty() {
    let client = make_client().await;

    client
        .create_trail()
        .name("empty-es-trail")
        .s3_bucket_name("bucket")
        .send()
        .await
        .unwrap();

    let resp = client
        .get_event_selectors()
        .trail_name("empty-es-trail")
        .send()
        .await
        .unwrap();

    assert!(resp.trail_arn().unwrap().contains("empty-es-trail"));
    assert_eq!(resp.event_selectors().len(), 0);
}

// Ported from moto: test_cloudtrail_eventselectors.py::test_put_event_selectors (with data resources)
#[tokio::test]
async fn test_put_event_selectors_with_data_resources() {
    use aws_sdk_cloudtrail::types::{DataResource, EventSelector, ReadWriteType};

    let client = make_client().await;

    client
        .create_trail()
        .name("dr-es-trail")
        .s3_bucket_name("bucket")
        .send()
        .await
        .unwrap();

    let dr = DataResource::builder()
        .r#type("AWS::S3::Object")
        .values("arn:aws:s3:::*/*")
        .build();

    let selector = EventSelector::builder()
        .read_write_type(ReadWriteType::All)
        .include_management_events(true)
        .data_resources(dr)
        .build();

    let resp = client
        .put_event_selectors()
        .trail_name("dr-es-trail")
        .event_selectors(selector)
        .send()
        .await
        .unwrap();

    assert!(resp.trail_arn().is_some());
    assert_eq!(resp.event_selectors().len(), 1);
    let es = &resp.event_selectors()[0];
    assert_eq!(es.read_write_type(), Some(&ReadWriteType::All));
    assert_eq!(es.include_management_events(), Some(true));
    assert_eq!(es.data_resources().len(), 1);
    assert_eq!(es.data_resources()[0].r#type(), Some("AWS::S3::Object"));
    assert_eq!(es.data_resources()[0].values().len(), 1);
    assert_eq!(es.data_resources()[0].values()[0], "arn:aws:s3:::*/*");
}

// Ported from moto: test_cloudtrail_eventselectors.py::test_get_event_selectors (after put)
#[tokio::test]
async fn test_get_event_selectors_after_put() {
    use aws_sdk_cloudtrail::types::{DataResource, EventSelector, ReadWriteType};

    let client = make_client().await;

    client
        .create_trail()
        .name("get-es-trail")
        .s3_bucket_name("bucket")
        .send()
        .await
        .unwrap();

    let dr = DataResource::builder()
        .r#type("AWS::S3::Object")
        .values("arn:aws:s3:::*/*")
        .build();

    let selector = EventSelector::builder()
        .read_write_type(ReadWriteType::All)
        .include_management_events(false)
        .data_resources(dr)
        .build();

    client
        .put_event_selectors()
        .trail_name("get-es-trail")
        .event_selectors(selector)
        .send()
        .await
        .unwrap();

    let resp = client
        .get_event_selectors()
        .trail_name("get-es-trail")
        .send()
        .await
        .unwrap();

    assert!(resp.trail_arn().unwrap().contains("get-es-trail"));
    assert_eq!(resp.event_selectors().len(), 1);
    let es = &resp.event_selectors()[0];
    assert_eq!(es.read_write_type(), Some(&ReadWriteType::All));
    assert_eq!(es.include_management_events(), Some(false));
    assert_eq!(es.data_resources().len(), 1);
}

// Ported from moto: test_cloudtrail_eventselectors.py::test_get_insight_selectors (empty)
#[tokio::test]
async fn test_get_insight_selectors_empty() {
    let client = make_client().await;

    client
        .create_trail()
        .name("empty-insight-trail")
        .s3_bucket_name("bucket")
        .send()
        .await
        .unwrap();

    let resp = client
        .get_insight_selectors()
        .trail_name("empty-insight-trail")
        .send()
        .await
        .unwrap();

    assert!(resp.trail_arn().unwrap().contains("empty-insight-trail"));
    assert_eq!(resp.insight_selectors().len(), 0);
}

// Ported from moto: test_cloudtrail_tags.py::test_create_trail_without_tags_and_list_tags
#[tokio::test]
async fn test_list_tags_empty() {
    let client = make_client().await;

    let create_resp = client
        .create_trail()
        .name("no-tags-trail")
        .s3_bucket_name("bucket")
        .send()
        .await
        .unwrap();
    let trail_arn = create_resp.trail_arn().unwrap().to_string();

    let resp = client
        .list_tags()
        .resource_id_list(&trail_arn)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.resource_tag_list().len(), 1);
    assert_eq!(
        resp.resource_tag_list()[0].resource_id(),
        Some(trail_arn.as_str())
    );
    assert_eq!(resp.resource_tag_list()[0].tags_list().len(), 0);
}

// Ported from moto: test_cloudtrail_tags.py::test_add_tags
#[tokio::test]
async fn test_add_tags_single() {
    use aws_sdk_cloudtrail::types::Tag;

    let client = make_client().await;

    let create_resp = client
        .create_trail()
        .name("single-tag-trail")
        .s3_bucket_name("bucket")
        .send()
        .await
        .unwrap();
    let trail_arn = create_resp.trail_arn().unwrap().to_string();

    client
        .add_tags()
        .resource_id(&trail_arn)
        .tags_list(Tag::builder().key("k1").value("v1").build().unwrap())
        .send()
        .await
        .unwrap();

    let resp = client
        .list_tags()
        .resource_id_list(&trail_arn)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.resource_tag_list().len(), 1);
    let rt = &resp.resource_tag_list()[0];
    assert_eq!(rt.resource_id(), Some(trail_arn.as_str()));
    assert_eq!(rt.tags_list().len(), 1);
    assert_eq!(rt.tags_list()[0].key(), "k1");
    assert_eq!(rt.tags_list()[0].value(), Some("v1"));
}

// Ported from moto: test_cloudtrail.py::test_update_trail_simple (no-change update)
#[tokio::test]
async fn test_update_trail_no_changes() {
    let client = make_client().await;

    client
        .create_trail()
        .name("noop-update-trail")
        .s3_bucket_name("original-bucket")
        .send()
        .await
        .unwrap();

    let resp = client
        .update_trail()
        .name("noop-update-trail")
        .send()
        .await
        .unwrap();

    assert_eq!(resp.name(), Some("noop-update-trail"));
    assert_eq!(resp.s3_bucket_name(), Some("original-bucket"));
    assert_eq!(resp.include_global_service_events(), Some(true));
    assert_eq!(resp.is_multi_region_trail(), Some(false));

    // Verify get_trail shows the same
    let trail = client
        .get_trail()
        .name("noop-update-trail")
        .send()
        .await
        .unwrap();
    assert_eq!(trail.trail().unwrap().name(), Some("noop-update-trail"));
    assert_eq!(
        trail.trail().unwrap().s3_bucket_name(),
        Some("original-bucket")
    );
}

// Ported from moto: test_cloudtrail.py::test_describe_trails_without_shadowtrails (home_region field)
#[tokio::test]
async fn test_describe_trails_home_region() {
    let client = make_client().await;

    client
        .create_trail()
        .name("home-trail")
        .s3_bucket_name("bucket")
        .send()
        .await
        .unwrap();

    let resp = client.describe_trails().send().await.unwrap();
    assert_eq!(resp.trail_list().len(), 1);
    let trail = &resp.trail_list()[0];
    assert_eq!(trail.home_region(), Some("us-east-1"));
}

// Ported from moto: test_cloudtrail.py::test_get_trail (verify ARN format exactly)
#[tokio::test]
async fn test_get_trail_arn_format() {
    let client = make_client().await;

    client
        .create_trail()
        .name("arn-check")
        .s3_bucket_name("bucket")
        .send()
        .await
        .unwrap();

    let resp = client.get_trail().name("arn-check").send().await.unwrap();

    let trail = resp.trail().unwrap();
    let arn = trail.trail_arn().unwrap();
    // ARN should match arn:aws:cloudtrail:{region}:{account_id}:trail/{name}
    assert!(arn.starts_with("arn:aws:cloudtrail:us-east-1:"));
    assert!(arn.ends_with(":trail/arn-check"));
}

// Ported from moto: test_cloudtrail.py::test_create_trail_simple (verify duplicate fails)
#[tokio::test]
async fn test_create_duplicate_trail_fails() {
    let client = make_client().await;

    client
        .create_trail()
        .name("dup-trail")
        .s3_bucket_name("bucket")
        .send()
        .await
        .unwrap();

    let result = client
        .create_trail()
        .name("dup-trail")
        .s3_bucket_name("bucket")
        .send()
        .await;

    assert!(result.is_err());
}

// ============================================================================
// Tests derived from AWS documentation: AWS CloudTrail
// ============================================================================

#[tokio::test]
async fn test_start_logging_nonexistent_trail() {
    let client = make_client().await;

    let result = client
        .start_logging()
        .name("nonexistent-trail")
        .send()
        .await;

    assert!(
        result.is_err(),
        "start_logging on nonexistent trail should fail"
    );
}

#[tokio::test]
async fn test_stop_logging_nonexistent_trail() {
    let client = make_client().await;

    let result = client.stop_logging().name("nonexistent-trail").send().await;

    assert!(
        result.is_err(),
        "stop_logging on nonexistent trail should fail"
    );
}

#[tokio::test]
async fn test_describe_trails_empty() {
    let client = make_client().await;

    let resp = client
        .describe_trails()
        .send()
        .await
        .expect("describe_trails on empty state should succeed");

    assert_eq!(resp.trail_list().len(), 0);
}

#[tokio::test]
async fn test_list_trails_empty() {
    let client = make_client().await;

    let resp = client
        .list_trails()
        .send()
        .await
        .expect("list_trails on empty state should succeed");

    assert_eq!(resp.trails().len(), 0);
}

#[tokio::test]
async fn test_create_trail_with_s3_prefix() {
    let client = make_client().await;

    let resp = client
        .create_trail()
        .name("prefix-trail")
        .s3_bucket_name("my-bucket")
        .s3_key_prefix("logs/")
        .send()
        .await
        .expect("create_trail with s3_key_prefix should succeed");

    assert_eq!(resp.name(), Some("prefix-trail"));
    assert_eq!(resp.s3_key_prefix(), Some("logs/"));
}

#[tokio::test]
async fn test_put_event_selectors_replaces_previous() {
    use aws_sdk_cloudtrail::types::{EventSelector, ReadWriteType};

    let client = make_client().await;

    client
        .create_trail()
        .name("replace-es-trail")
        .s3_bucket_name("bucket")
        .send()
        .await
        .unwrap();

    // First put
    client
        .put_event_selectors()
        .trail_name("replace-es-trail")
        .event_selectors(
            EventSelector::builder()
                .read_write_type(ReadWriteType::ReadOnly)
                .include_management_events(true)
                .build(),
        )
        .send()
        .await
        .unwrap();

    // Second put replaces
    client
        .put_event_selectors()
        .trail_name("replace-es-trail")
        .event_selectors(
            EventSelector::builder()
                .read_write_type(ReadWriteType::WriteOnly)
                .include_management_events(false)
                .build(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .get_event_selectors()
        .trail_name("replace-es-trail")
        .send()
        .await
        .unwrap();

    assert_eq!(resp.event_selectors().len(), 1);
    assert_eq!(
        resp.event_selectors()[0].read_write_type(),
        Some(&ReadWriteType::WriteOnly)
    );
}

#[tokio::test]
async fn test_add_tags_nonexistent_trail() {
    use aws_sdk_cloudtrail::types::Tag;

    let client = make_client().await;

    let result = client
        .add_tags()
        .resource_id("arn:aws:cloudtrail:us-east-1:123456789012:trail/nonexistent")
        .tags_list(Tag::builder().key("k").value("v").build().unwrap())
        .send()
        .await;

    assert!(
        result.is_err(),
        "add_tags on nonexistent trail ARN should fail"
    );
}

#[tokio::test]
async fn test_start_logging_by_arn() {
    let client = make_client().await;

    let create_resp = client
        .create_trail()
        .name("arn-log-trail")
        .s3_bucket_name("bucket")
        .send()
        .await
        .unwrap();
    let trail_arn = create_resp.trail_arn().unwrap().to_string();

    client
        .start_logging()
        .name(&trail_arn)
        .send()
        .await
        .expect("start_logging by ARN should succeed");

    let status = client
        .get_trail_status()
        .name(&trail_arn)
        .send()
        .await
        .unwrap();

    assert_eq!(status.is_logging(), Some(true));
}

#[tokio::test]
async fn test_update_trail_by_arn() {
    let client = make_client().await;

    let create_resp = client
        .create_trail()
        .name("arn-update-trail")
        .s3_bucket_name("old-bucket")
        .send()
        .await
        .unwrap();
    let trail_arn = create_resp.trail_arn().unwrap().to_string();

    let resp = client
        .update_trail()
        .name(&trail_arn)
        .s3_bucket_name("new-bucket")
        .send()
        .await
        .expect("update_trail by ARN should succeed");

    assert_eq!(resp.s3_bucket_name(), Some("new-bucket"));
}
