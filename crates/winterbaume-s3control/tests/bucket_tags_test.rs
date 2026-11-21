//! Issue #53: the S3 Control tagging API on a general purpose bucket ARN must
//! read and write the tag set that the S3 bucket tagging API uses.

use std::collections::BTreeMap;
use std::sync::Arc;

use aws_sdk_s3::types::{Tag as S3Tag, Tagging};
use aws_sdk_s3control::error::ProvideErrorMetadata;
use aws_sdk_s3control::types::Tag;
use winterbaume_core::MockAws;
use winterbaume_s3::S3Service;
use winterbaume_s3control::S3ControlService;

const ACCOUNT_ID: &str = "123456789012";

async fn linked_clients(region: &str) -> (aws_sdk_s3control::Client, aws_sdk_s3::Client, MockAws) {
    let s3 = Arc::new(S3Service::new());
    let s3control = S3ControlService::with_s3_service(Arc::clone(&s3));
    let mock = MockAws::builder()
        .with_service(s3)
        .with_service(s3control)
        .build();
    let config = mock.sdk_config(region).await;
    (
        aws_sdk_s3control::Client::new(&config),
        aws_sdk_s3::Client::new(&config),
        mock,
    )
}

fn bucket_arn(bucket: &str) -> String {
    format!("arn:aws:s3:::{bucket}")
}

fn control_tag(key: &str, value: &str) -> Tag {
    Tag::builder().key(key).value(value).build().unwrap()
}

fn s3_tagging(tags: &[(&str, &str)]) -> Tagging {
    let mut tagging = Tagging::builder();
    for (key, value) in tags {
        tagging = tagging.tag_set(S3Tag::builder().key(*key).value(*value).build().unwrap());
    }
    tagging.build().unwrap()
}

fn tag_map(tags: &[(&str, &str)]) -> BTreeMap<String, String> {
    tags.iter()
        .map(|(key, value)| (key.to_string(), value.to_string()))
        .collect()
}

async fn control_tags(
    client: &aws_sdk_s3control::Client,
    resource_arn: &str,
) -> BTreeMap<String, String> {
    client
        .list_tags_for_resource()
        .account_id(ACCOUNT_ID)
        .resource_arn(resource_arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed")
        .tags()
        .iter()
        .map(|tag| (tag.key().to_string(), tag.value().to_string()))
        .collect()
}

/// Tags as `GetBucketTagging` reports them; an empty map for `NoSuchTagSet`.
async fn s3_tags(client: &aws_sdk_s3::Client, bucket: &str) -> BTreeMap<String, String> {
    match client.get_bucket_tagging().bucket(bucket).send().await {
        Ok(output) => output
            .tag_set()
            .iter()
            .map(|tag| (tag.key().to_string(), tag.value().to_string()))
            .collect(),
        Err(err) => {
            assert_eq!(
                err.code(),
                Some("NoSuchTagSet"),
                "unexpected GetBucketTagging error: {err:?}"
            );
            BTreeMap::new()
        }
    }
}

#[tokio::test]
async fn test_bucket_tags_from_s3_are_listed_by_s3control() {
    let (control, s3, _mock) = linked_clients("us-east-1").await;
    s3.create_bucket()
        .bucket("s3-to-control")
        .send()
        .await
        .unwrap();

    // A bucket without tags lists none rather than failing.
    assert!(
        control_tags(&control, &bucket_arn("s3-to-control"))
            .await
            .is_empty()
    );

    s3.put_bucket_tagging()
        .bucket("s3-to-control")
        .tagging(s3_tagging(&[("Environment", "dev"), ("Owner", "carina")]))
        .send()
        .await
        .unwrap();
    assert_eq!(
        control_tags(&control, &bucket_arn("s3-to-control")).await,
        tag_map(&[("Environment", "dev"), ("Owner", "carina")])
    );

    // PutBucketTagging replaces the whole tag set.
    s3.put_bucket_tagging()
        .bucket("s3-to-control")
        .tagging(s3_tagging(&[("Environment", "prod")]))
        .send()
        .await
        .unwrap();
    assert_eq!(
        control_tags(&control, &bucket_arn("s3-to-control")).await,
        tag_map(&[("Environment", "prod")])
    );

    s3.delete_bucket_tagging()
        .bucket("s3-to-control")
        .send()
        .await
        .unwrap();
    assert!(
        control_tags(&control, &bucket_arn("s3-to-control"))
            .await
            .is_empty()
    );
}

#[tokio::test]
async fn test_bucket_tags_from_create_bucket_are_listed_by_s3control() {
    let (control, s3, _mock) = linked_clients("us-east-1").await;
    s3.create_bucket()
        .bucket("created-with-tags")
        .create_bucket_configuration(
            aws_sdk_s3::types::CreateBucketConfiguration::builder()
                .tags(
                    S3Tag::builder()
                        .key("Team")
                        .value("storage")
                        .build()
                        .unwrap(),
                )
                .build(),
        )
        .send()
        .await
        .unwrap();

    assert_eq!(
        control_tags(&control, &bucket_arn("created-with-tags")).await,
        tag_map(&[("Team", "storage")])
    );
}

#[tokio::test]
async fn test_tag_resource_on_bucket_is_visible_through_s3() {
    let (control, s3, _mock) = linked_clients("us-east-1").await;
    s3.create_bucket()
        .bucket("control-to-s3")
        .send()
        .await
        .unwrap();
    let arn = bucket_arn("control-to-s3");

    control
        .tag_resource()
        .account_id(ACCOUNT_ID)
        .resource_arn(&arn)
        .tags(control_tag("Environment", "dev"))
        .tags(control_tag("Owner", "carina"))
        .send()
        .await
        .expect("tag_resource should succeed");
    assert_eq!(
        s3_tags(&s3, "control-to-s3").await,
        tag_map(&[("Environment", "dev"), ("Owner", "carina")])
    );

    // TagResource overwrites the named keys and keeps the others.
    control
        .tag_resource()
        .account_id(ACCOUNT_ID)
        .resource_arn(&arn)
        .tags(control_tag("Environment", "prod"))
        .tags(control_tag("CostCentre", "42"))
        .send()
        .await
        .expect("tag_resource should succeed");
    let expected = tag_map(&[
        ("CostCentre", "42"),
        ("Environment", "prod"),
        ("Owner", "carina"),
    ]);
    assert_eq!(s3_tags(&s3, "control-to-s3").await, expected);
    assert_eq!(control_tags(&control, &arn).await, expected);
}

#[tokio::test]
async fn test_untag_resource_on_bucket_is_visible_through_s3() {
    let (control, s3, _mock) = linked_clients("us-east-1").await;
    s3.create_bucket()
        .bucket("untag-bucket")
        .send()
        .await
        .unwrap();
    let arn = bucket_arn("untag-bucket");
    s3.put_bucket_tagging()
        .bucket("untag-bucket")
        .tagging(s3_tagging(&[
            ("Environment", "dev"),
            ("Owner", "carina"),
            ("cost centre", "4/2"),
        ]))
        .send()
        .await
        .unwrap();

    // Several keys in one request, one of them needing percent-encoding.
    control
        .untag_resource()
        .account_id(ACCOUNT_ID)
        .resource_arn(&arn)
        .tag_keys("Owner")
        .tag_keys("cost centre")
        .tag_keys("NotThere")
        .send()
        .await
        .expect("untag_resource should succeed");
    assert_eq!(
        s3_tags(&s3, "untag-bucket").await,
        tag_map(&[("Environment", "dev")])
    );

    // Removing the last tag leaves S3 with no tag set at all.
    control
        .untag_resource()
        .account_id(ACCOUNT_ID)
        .resource_arn(&arn)
        .tag_keys("Environment")
        .send()
        .await
        .expect("untag_resource should succeed");
    assert!(control_tags(&control, &arn).await.is_empty());
    let err = s3
        .get_bucket_tagging()
        .bucket("untag-bucket")
        .send()
        .await
        .expect_err("GetBucketTagging should report NoSuchTagSet");
    assert_eq!(err.code(), Some("NoSuchTagSet"));
}

/// The bucket ARN carries no Region, so the bucket is found whichever Region
/// the S3 Control request is signed for.
#[tokio::test]
async fn test_bucket_tags_resolve_bucket_in_another_region() {
    let s3_service = Arc::new(S3Service::new());
    let mock = MockAws::builder()
        .with_service(Arc::clone(&s3_service))
        .with_service(S3ControlService::with_s3_service(s3_service))
        .build();
    let s3 = aws_sdk_s3::Client::new(&mock.sdk_config("eu-west-1").await);
    let control = aws_sdk_s3control::Client::new(&mock.sdk_config("us-east-1").await);

    s3.create_bucket()
        .bucket("bucket-in-ireland")
        .create_bucket_configuration(
            aws_sdk_s3::types::CreateBucketConfiguration::builder()
                .location_constraint(aws_sdk_s3::types::BucketLocationConstraint::EuWest1)
                .build(),
        )
        .send()
        .await
        .unwrap();
    let arn = bucket_arn("bucket-in-ireland");
    control
        .tag_resource()
        .account_id(ACCOUNT_ID)
        .resource_arn(&arn)
        .tags(control_tag("Region", "ireland"))
        .send()
        .await
        .expect("tag_resource should succeed");

    assert_eq!(
        s3_tags(&s3, "bucket-in-ireland").await,
        tag_map(&[("Region", "ireland")])
    );
}

#[tokio::test]
async fn test_bucket_tagging_on_missing_bucket_reports_no_such_bucket() {
    let (control, _s3, _mock) = linked_clients("us-east-1").await;
    let arn = bucket_arn("no-such-bucket");

    let err = control
        .list_tags_for_resource()
        .account_id(ACCOUNT_ID)
        .resource_arn(&arn)
        .send()
        .await
        .expect_err("list_tags_for_resource should fail");
    assert_eq!(err.code(), Some("NoSuchBucket"));
    assert_eq!(err.raw_response().unwrap().status().as_u16(), 404);

    let err = control
        .tag_resource()
        .account_id(ACCOUNT_ID)
        .resource_arn(&arn)
        .tags(control_tag("Environment", "dev"))
        .send()
        .await
        .expect_err("tag_resource should fail");
    assert_eq!(err.code(), Some("NoSuchBucket"));

    let err = control
        .untag_resource()
        .account_id(ACCOUNT_ID)
        .resource_arn(&arn)
        .tag_keys("Environment")
        .send()
        .await
        .expect_err("untag_resource should fail");
    assert_eq!(err.code(), Some("NoSuchBucket"));

    // A failed TagResource must not leave tags behind for a later bucket.
    assert!(
        control
            .list_tags_for_resource()
            .account_id(ACCOUNT_ID)
            .resource_arn(&arn)
            .send()
            .await
            .is_err()
    );
}

/// ARNs that are not general purpose buckets stay in S3 Control's own map
/// even when an S3 service is attached.
#[tokio::test]
async fn test_non_bucket_arn_is_unaffected_by_s3_link() {
    let (control, _s3, _mock) = linked_clients("us-east-1").await;
    let arn = "arn:aws:s3:us-east-1:123456789012:storage-lens-group/my-group";

    control
        .tag_resource()
        .account_id(ACCOUNT_ID)
        .resource_arn(arn)
        .tags(control_tag("Environment", "dev"))
        .send()
        .await
        .expect("tag_resource should succeed");
    assert_eq!(
        control_tags(&control, arn).await,
        tag_map(&[("Environment", "dev")])
    );
}

/// Without an attached S3 service a bucket ARN is tagged like any other ARN:
/// no bucket lookup, no `NoSuchBucket`.
#[tokio::test]
async fn test_bucket_arn_without_s3_service_uses_s3control_tags() {
    let mock = MockAws::builder()
        .with_service(S3ControlService::new())
        .build();
    let control = aws_sdk_s3control::Client::new(&mock.sdk_config("us-east-1").await);
    let arn = bucket_arn("unlinked-bucket");

    assert!(control_tags(&control, &arn).await.is_empty());

    control
        .tag_resource()
        .account_id(ACCOUNT_ID)
        .resource_arn(&arn)
        .tags(control_tag("Environment", "dev"))
        .tags(control_tag("Owner", "carina"))
        .send()
        .await
        .expect("tag_resource should succeed");
    assert_eq!(
        control_tags(&control, &arn).await,
        tag_map(&[("Environment", "dev"), ("Owner", "carina")])
    );

    control
        .untag_resource()
        .account_id(ACCOUNT_ID)
        .resource_arn(&arn)
        .tag_keys("Environment")
        .tag_keys("Owner")
        .send()
        .await
        .expect("untag_resource should succeed");
    assert!(control_tags(&control, &arn).await.is_empty());
}
