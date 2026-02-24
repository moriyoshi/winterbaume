use aws_sdk_cloudfront::config::BehaviorVersion;
use winterbaume_cloudfront::CloudFrontService;
use winterbaume_core::MockAws;

async fn make_cloudfront_client() -> aws_sdk_cloudfront::Client {
    let mock = MockAws::builder()
        .with_service(CloudFrontService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_cloudfront::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_cloudfront::Client::new(&config)
}

fn make_distribution_config(caller_ref: &str) -> aws_sdk_cloudfront::types::DistributionConfig {
    aws_sdk_cloudfront::types::DistributionConfig::builder()
        .caller_reference(caller_ref)
        .origins(
            aws_sdk_cloudfront::types::Origins::builder()
                .quantity(1)
                .items(
                    aws_sdk_cloudfront::types::Origin::builder()
                        .id("my-origin")
                        .domain_name("my-bucket.s3.amazonaws.com")
                        .s3_origin_config(
                            aws_sdk_cloudfront::types::S3OriginConfig::builder()
                                .origin_access_identity("")
                                .build(),
                        )
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        )
        .default_cache_behavior(
            aws_sdk_cloudfront::types::DefaultCacheBehavior::builder()
                .target_origin_id("my-origin")
                .viewer_protocol_policy(aws_sdk_cloudfront::types::ViewerProtocolPolicy::AllowAll)
                .build()
                .unwrap(),
        )
        .comment("")
        .enabled(true)
        .build()
        .unwrap()
}

// ---- Distribution tests ----

#[tokio::test]
async fn test_create_distribution() {
    let client = make_cloudfront_client().await;

    let resp = client
        .create_distribution()
        .distribution_config(make_distribution_config("ref-1"))
        .send()
        .await
        .expect("create_distribution should succeed");

    let dist = resp.distribution().expect("should have distribution");
    assert!(!dist.id().is_empty());
    assert!(dist.arn().contains("arn:aws:cloudfront::"));
    assert!(dist.domain_name().contains("cloudfront.net"));
}

#[tokio::test]
async fn test_create_distribution_with_tags() {
    let client = make_cloudfront_client().await;

    let tag = aws_sdk_cloudfront::types::Tag::builder()
        .key("env")
        .value("test")
        .build()
        .unwrap();

    let resp = client
        .create_distribution_with_tags()
        .distribution_config_with_tags(
            aws_sdk_cloudfront::types::DistributionConfigWithTags::builder()
                .distribution_config(make_distribution_config("ref-tags-1"))
                .tags(
                    aws_sdk_cloudfront::types::Tags::builder()
                        .items(tag)
                        .build(),
                )
                .build(),
        )
        .send()
        .await
        .expect("create_distribution_with_tags should succeed");

    let dist = resp.distribution().expect("should have distribution");
    assert!(!dist.id().is_empty());

    // Verify tags were set
    let tag_resp = client
        .list_tags_for_resource()
        .resource(dist.arn())
        .send()
        .await
        .expect("list_tags_for_resource should succeed");

    let tags = tag_resp.tags().unwrap();
    let items = tags.items();
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].key(), "env");
}

#[tokio::test]
async fn test_get_distribution() {
    let client = make_cloudfront_client().await;

    let create_resp = client
        .create_distribution()
        .distribution_config(make_distribution_config("ref-get"))
        .send()
        .await
        .unwrap();
    let dist_id = create_resp.distribution().unwrap().id().to_string();

    let resp = client
        .get_distribution()
        .id(&dist_id)
        .send()
        .await
        .expect("get_distribution should succeed");

    let dist = resp.distribution().unwrap();
    assert_eq!(dist.id(), dist_id);
    assert_eq!(dist.status(), "Deployed");
}

#[tokio::test]
async fn test_get_distribution_config() {
    let client = make_cloudfront_client().await;

    let create_resp = client
        .create_distribution()
        .distribution_config(make_distribution_config("ref-get-config"))
        .send()
        .await
        .unwrap();
    let dist_id = create_resp.distribution().unwrap().id().to_string();

    let resp = client
        .get_distribution_config()
        .id(&dist_id)
        .send()
        .await
        .expect("get_distribution_config should succeed");

    let config = resp.distribution_config().unwrap();
    assert_eq!(config.caller_reference(), "ref-get-config");
    assert!(resp.e_tag().is_some());
}

#[tokio::test]
async fn test_update_distribution() {
    let client = make_cloudfront_client().await;

    let create_resp = client
        .create_distribution()
        .distribution_config(make_distribution_config("ref-update"))
        .send()
        .await
        .unwrap();
    let dist_id = create_resp.distribution().unwrap().id().to_string();
    let etag = create_resp.e_tag().unwrap().to_string();

    let updated_config = aws_sdk_cloudfront::types::DistributionConfig::builder()
        .caller_reference("ref-update")
        .origins(
            aws_sdk_cloudfront::types::Origins::builder()
                .quantity(1)
                .items(
                    aws_sdk_cloudfront::types::Origin::builder()
                        .id("updated-origin")
                        .domain_name("updated.s3.amazonaws.com")
                        .s3_origin_config(
                            aws_sdk_cloudfront::types::S3OriginConfig::builder()
                                .origin_access_identity("")
                                .build(),
                        )
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        )
        .default_cache_behavior(
            aws_sdk_cloudfront::types::DefaultCacheBehavior::builder()
                .target_origin_id("updated-origin")
                .viewer_protocol_policy(aws_sdk_cloudfront::types::ViewerProtocolPolicy::HttpsOnly)
                .build()
                .unwrap(),
        )
        .comment("")
        .enabled(false)
        .build()
        .unwrap();

    let resp = client
        .update_distribution()
        .id(&dist_id)
        .if_match(&etag)
        .distribution_config(updated_config)
        .send()
        .await
        .expect("update_distribution should succeed");

    let dist = resp.distribution().unwrap();
    assert_eq!(dist.id(), dist_id);
    // Verify ETag changed
    assert_ne!(resp.e_tag().unwrap(), etag);
}

#[tokio::test]
async fn test_delete_distribution() {
    let client = make_cloudfront_client().await;

    let create_resp = client
        .create_distribution()
        .distribution_config(make_distribution_config("ref-del"))
        .send()
        .await
        .unwrap();
    let dist_id = create_resp.distribution().unwrap().id().to_string();
    let etag = create_resp.e_tag().unwrap_or_default().to_string();

    client
        .delete_distribution()
        .id(&dist_id)
        .if_match(&etag)
        .send()
        .await
        .expect("delete_distribution should succeed");

    let result = client.get_distribution().id(&dist_id).send().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_distributions() {
    let client = make_cloudfront_client().await;

    for i in 0..3 {
        client
            .create_distribution()
            .distribution_config(make_distribution_config(&format!("ref-list-{i}")))
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_distributions()
        .send()
        .await
        .expect("list_distributions should succeed");

    let list = resp
        .distribution_list()
        .expect("should have distribution list");
    assert_eq!(list.quantity(), 3);
}

#[tokio::test]
async fn test_get_nonexistent_distribution() {
    let client = make_cloudfront_client().await;

    let result = client.get_distribution().id("ENOTEXIST12345").send().await;

    assert!(result.is_err());
}

// ---- Invalidation tests ----

#[tokio::test]
async fn test_create_invalidation() {
    let client = make_cloudfront_client().await;

    let create_resp = client
        .create_distribution()
        .distribution_config(make_distribution_config("ref-inv-create"))
        .send()
        .await
        .unwrap();
    let dist_id = create_resp.distribution().unwrap().id().to_string();

    let resp = client
        .create_invalidation()
        .distribution_id(&dist_id)
        .invalidation_batch(
            aws_sdk_cloudfront::types::InvalidationBatch::builder()
                .caller_reference("inv-ref-1")
                .paths(
                    aws_sdk_cloudfront::types::Paths::builder()
                        .quantity(1)
                        .items("/index.html")
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("create_invalidation should succeed");

    let inv = resp.invalidation().unwrap();
    assert!(!inv.id().is_empty());
    assert_eq!(inv.status(), "COMPLETED");
}

#[tokio::test]
async fn test_get_invalidation() {
    let client = make_cloudfront_client().await;

    let create_resp = client
        .create_distribution()
        .distribution_config(make_distribution_config("ref-inv-get"))
        .send()
        .await
        .unwrap();
    let dist_id = create_resp.distribution().unwrap().id().to_string();

    let inv_resp = client
        .create_invalidation()
        .distribution_id(&dist_id)
        .invalidation_batch(
            aws_sdk_cloudfront::types::InvalidationBatch::builder()
                .caller_reference("inv-ref-get")
                .paths(
                    aws_sdk_cloudfront::types::Paths::builder()
                        .quantity(1)
                        .items("/*")
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();
    let inv_id = inv_resp.invalidation().unwrap().id().to_string();

    let resp = client
        .get_invalidation()
        .distribution_id(&dist_id)
        .id(&inv_id)
        .send()
        .await
        .expect("get_invalidation should succeed");

    let inv = resp.invalidation().unwrap();
    assert_eq!(inv.id(), inv_id);
    assert_eq!(inv.status(), "COMPLETED");
}

#[tokio::test]
async fn test_list_invalidations() {
    let client = make_cloudfront_client().await;

    let create_resp = client
        .create_distribution()
        .distribution_config(make_distribution_config("ref-inv-list"))
        .send()
        .await
        .unwrap();
    let dist_id = create_resp.distribution().unwrap().id().to_string();

    for i in 0..2 {
        client
            .create_invalidation()
            .distribution_id(&dist_id)
            .invalidation_batch(
                aws_sdk_cloudfront::types::InvalidationBatch::builder()
                    .caller_reference(format!("inv-list-{i}"))
                    .paths(
                        aws_sdk_cloudfront::types::Paths::builder()
                            .quantity(1)
                            .items("/*")
                            .build()
                            .unwrap(),
                    )
                    .build()
                    .unwrap(),
            )
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_invalidations()
        .distribution_id(&dist_id)
        .send()
        .await
        .expect("list_invalidations should succeed");

    let list = resp.invalidation_list().unwrap();
    assert_eq!(list.quantity(), 2);
}

// ---- Key Group tests ----

#[tokio::test]
async fn test_create_key_group() {
    let client = make_cloudfront_client().await;

    let resp = client
        .create_key_group()
        .key_group_config(
            aws_sdk_cloudfront::types::KeyGroupConfig::builder()
                .name("test-key-group")
                .items("pk-id-123")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("create_key_group should succeed");

    let kg = resp.key_group().unwrap();
    assert!(!kg.id().is_empty());
    assert_eq!(kg.key_group_config().unwrap().name(), "test-key-group");
}

#[tokio::test]
async fn test_get_key_group() {
    let client = make_cloudfront_client().await;

    let create_resp = client
        .create_key_group()
        .key_group_config(
            aws_sdk_cloudfront::types::KeyGroupConfig::builder()
                .name("get-key-group")
                .items("pk-id-456")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();
    let kg_id = create_resp.key_group().unwrap().id().to_string();

    let resp = client
        .get_key_group()
        .id(&kg_id)
        .send()
        .await
        .expect("get_key_group should succeed");

    let kg = resp.key_group().unwrap();
    assert_eq!(kg.id(), kg_id);
    assert!(resp.e_tag().is_some());
}

#[tokio::test]
async fn test_list_key_groups() {
    let client = make_cloudfront_client().await;

    client
        .create_key_group()
        .key_group_config(
            aws_sdk_cloudfront::types::KeyGroupConfig::builder()
                .name("list-kg-1")
                .items("pk-1")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .list_key_groups()
        .send()
        .await
        .expect("list_key_groups should succeed");

    let list = resp.key_group_list().unwrap();
    assert!(list.quantity() >= 1);
}

// ---- Origin Access Control tests ----

#[tokio::test]
async fn test_create_origin_access_control() {
    let client = make_cloudfront_client().await;

    let resp = client
        .create_origin_access_control()
        .origin_access_control_config(
            aws_sdk_cloudfront::types::OriginAccessControlConfig::builder()
                .name("test-oac")
                .signing_protocol(
                    aws_sdk_cloudfront::types::OriginAccessControlSigningProtocols::Sigv4,
                )
                .signing_behavior(
                    aws_sdk_cloudfront::types::OriginAccessControlSigningBehaviors::Always,
                )
                .origin_access_control_origin_type(
                    aws_sdk_cloudfront::types::OriginAccessControlOriginTypes::S3,
                )
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("create_origin_access_control should succeed");

    let oac = resp.origin_access_control().unwrap();
    assert!(!oac.id().is_empty());
}

#[tokio::test]
async fn test_get_origin_access_control() {
    let client = make_cloudfront_client().await;

    let create_resp = client
        .create_origin_access_control()
        .origin_access_control_config(
            aws_sdk_cloudfront::types::OriginAccessControlConfig::builder()
                .name("get-oac")
                .signing_protocol(
                    aws_sdk_cloudfront::types::OriginAccessControlSigningProtocols::Sigv4,
                )
                .signing_behavior(
                    aws_sdk_cloudfront::types::OriginAccessControlSigningBehaviors::Always,
                )
                .origin_access_control_origin_type(
                    aws_sdk_cloudfront::types::OriginAccessControlOriginTypes::S3,
                )
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();
    let oac_id = create_resp
        .origin_access_control()
        .unwrap()
        .id()
        .to_string();

    let resp = client
        .get_origin_access_control()
        .id(&oac_id)
        .send()
        .await
        .expect("get_origin_access_control should succeed");

    let oac = resp.origin_access_control().unwrap();
    assert_eq!(oac.id(), oac_id);
    assert!(resp.e_tag().is_some());
}

#[tokio::test]
async fn test_update_origin_access_control() {
    let client = make_cloudfront_client().await;

    let create_resp = client
        .create_origin_access_control()
        .origin_access_control_config(
            aws_sdk_cloudfront::types::OriginAccessControlConfig::builder()
                .name("update-oac")
                .signing_protocol(
                    aws_sdk_cloudfront::types::OriginAccessControlSigningProtocols::Sigv4,
                )
                .signing_behavior(
                    aws_sdk_cloudfront::types::OriginAccessControlSigningBehaviors::Always,
                )
                .origin_access_control_origin_type(
                    aws_sdk_cloudfront::types::OriginAccessControlOriginTypes::S3,
                )
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();
    let oac_id = create_resp
        .origin_access_control()
        .unwrap()
        .id()
        .to_string();
    let etag = create_resp.e_tag().unwrap().to_string();

    let resp = client
        .update_origin_access_control()
        .id(&oac_id)
        .if_match(&etag)
        .origin_access_control_config(
            aws_sdk_cloudfront::types::OriginAccessControlConfig::builder()
                .name("updated-oac")
                .description("Updated description")
                .signing_protocol(
                    aws_sdk_cloudfront::types::OriginAccessControlSigningProtocols::Sigv4,
                )
                .signing_behavior(
                    aws_sdk_cloudfront::types::OriginAccessControlSigningBehaviors::Always,
                )
                .origin_access_control_origin_type(
                    aws_sdk_cloudfront::types::OriginAccessControlOriginTypes::S3,
                )
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("update_origin_access_control should succeed");

    let oac = resp.origin_access_control().unwrap();
    assert_eq!(oac.id(), oac_id);
    let config = oac.origin_access_control_config().unwrap();
    assert_eq!(config.name(), "updated-oac");
}

#[tokio::test]
async fn test_delete_origin_access_control() {
    let client = make_cloudfront_client().await;

    let create_resp = client
        .create_origin_access_control()
        .origin_access_control_config(
            aws_sdk_cloudfront::types::OriginAccessControlConfig::builder()
                .name("del-oac")
                .signing_protocol(
                    aws_sdk_cloudfront::types::OriginAccessControlSigningProtocols::Sigv4,
                )
                .signing_behavior(
                    aws_sdk_cloudfront::types::OriginAccessControlSigningBehaviors::Always,
                )
                .origin_access_control_origin_type(
                    aws_sdk_cloudfront::types::OriginAccessControlOriginTypes::S3,
                )
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();
    let oac_id = create_resp
        .origin_access_control()
        .unwrap()
        .id()
        .to_string();
    let etag = create_resp.e_tag().unwrap().to_string();

    client
        .delete_origin_access_control()
        .id(&oac_id)
        .if_match(&etag)
        .send()
        .await
        .expect("delete should succeed");

    let result = client.get_origin_access_control().id(&oac_id).send().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_origin_access_controls() {
    let client = make_cloudfront_client().await;

    client
        .create_origin_access_control()
        .origin_access_control_config(
            aws_sdk_cloudfront::types::OriginAccessControlConfig::builder()
                .name("list-oac-1")
                .signing_protocol(
                    aws_sdk_cloudfront::types::OriginAccessControlSigningProtocols::Sigv4,
                )
                .signing_behavior(
                    aws_sdk_cloudfront::types::OriginAccessControlSigningBehaviors::Always,
                )
                .origin_access_control_origin_type(
                    aws_sdk_cloudfront::types::OriginAccessControlOriginTypes::S3,
                )
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .list_origin_access_controls()
        .send()
        .await
        .expect("list_origin_access_controls should succeed");

    let list = resp.origin_access_control_list().unwrap();
    assert!(list.quantity() >= 1);
}

// ---- Public Key tests ----

#[tokio::test]
async fn test_create_public_key() {
    let client = make_cloudfront_client().await;

    let resp = client
        .create_public_key()
        .public_key_config(
            aws_sdk_cloudfront::types::PublicKeyConfig::builder()
                .caller_reference("pk-ref-1")
                .name("test-public-key")
                .encoded_key("-----BEGIN PUBLIC KEY-----\nMIIBIjANBg==\n-----END PUBLIC KEY-----")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("create_public_key should succeed");

    let pk = resp.public_key().unwrap();
    assert!(!pk.id().is_empty());
}

#[tokio::test]
async fn test_get_public_key() {
    let client = make_cloudfront_client().await;

    let create_resp = client
        .create_public_key()
        .public_key_config(
            aws_sdk_cloudfront::types::PublicKeyConfig::builder()
                .caller_reference("pk-ref-get")
                .name("get-public-key")
                .encoded_key("-----BEGIN PUBLIC KEY-----\nMIIBIjANBg==\n-----END PUBLIC KEY-----")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();
    let pk_id = create_resp.public_key().unwrap().id().to_string();

    let resp = client
        .get_public_key()
        .id(&pk_id)
        .send()
        .await
        .expect("get_public_key should succeed");

    let pk = resp.public_key().unwrap();
    assert_eq!(pk.id(), pk_id);
    assert!(resp.e_tag().is_some());
}

#[tokio::test]
async fn test_delete_public_key() {
    let client = make_cloudfront_client().await;

    let create_resp = client
        .create_public_key()
        .public_key_config(
            aws_sdk_cloudfront::types::PublicKeyConfig::builder()
                .caller_reference("pk-ref-del")
                .name("del-public-key")
                .encoded_key("-----BEGIN PUBLIC KEY-----\nMIIBIjANBg==\n-----END PUBLIC KEY-----")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();
    let pk_id = create_resp.public_key().unwrap().id().to_string();
    let etag = create_resp.e_tag().unwrap().to_string();

    client
        .delete_public_key()
        .id(&pk_id)
        .if_match(&etag)
        .send()
        .await
        .expect("delete should succeed");

    let result = client.get_public_key().id(&pk_id).send().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_public_keys() {
    let client = make_cloudfront_client().await;

    client
        .create_public_key()
        .public_key_config(
            aws_sdk_cloudfront::types::PublicKeyConfig::builder()
                .caller_reference("pk-ref-list")
                .name("list-public-key")
                .encoded_key("-----BEGIN PUBLIC KEY-----\nMIIBIjANBg==\n-----END PUBLIC KEY-----")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .list_public_keys()
        .send()
        .await
        .expect("list_public_keys should succeed");

    let list = resp.public_key_list().unwrap();
    assert!(list.quantity() >= 1);
}

// ---- Tag tests ----

#[tokio::test]
async fn test_tag_untag_list_tags() {
    let client = make_cloudfront_client().await;

    let create_resp = client
        .create_distribution()
        .distribution_config(make_distribution_config("ref-tag-test"))
        .send()
        .await
        .unwrap();
    let arn = create_resp.distribution().unwrap().arn().to_string();

    // Tag
    client
        .tag_resource()
        .resource(&arn)
        .tags(
            aws_sdk_cloudfront::types::Tags::builder()
                .items(
                    aws_sdk_cloudfront::types::Tag::builder()
                        .key("project")
                        .value("winterbaume")
                        .build()
                        .unwrap(),
                )
                .items(
                    aws_sdk_cloudfront::types::Tag::builder()
                        .key("env")
                        .value("test")
                        .build()
                        .unwrap(),
                )
                .build(),
        )
        .send()
        .await
        .expect("tag_resource should succeed");

    // List tags
    let tag_resp = client
        .list_tags_for_resource()
        .resource(&arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");

    let tags = tag_resp.tags().unwrap();
    let items = tags.items();
    assert_eq!(items.len(), 2);

    // Untag
    client
        .untag_resource()
        .resource(&arn)
        .tag_keys(
            aws_sdk_cloudfront::types::TagKeys::builder()
                .items("env")
                .build(),
        )
        .send()
        .await
        .expect("untag_resource should succeed");

    // Verify tag removed
    let tag_resp2 = client
        .list_tags_for_resource()
        .resource(&arn)
        .send()
        .await
        .unwrap();

    let tags2 = tag_resp2.tags().unwrap();
    let items2 = tags2.items();
    assert_eq!(items2.len(), 1);
    assert_eq!(items2[0].key(), "project");
}

// ---- Error path tests ----

#[tokio::test]
async fn test_get_nonexistent_invalidation() {
    let client = make_cloudfront_client().await;

    let create_resp = client
        .create_distribution()
        .distribution_config(make_distribution_config("ref-inv-noexist"))
        .send()
        .await
        .unwrap();
    let dist_id = create_resp.distribution().unwrap().id().to_string();

    let result = client
        .get_invalidation()
        .distribution_id(&dist_id)
        .id("INOTEXIST")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_nonexistent_origin_access_control() {
    let client = make_cloudfront_client().await;

    let result = client
        .get_origin_access_control()
        .id("ENOTEXIST")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_nonexistent_public_key() {
    let client = make_cloudfront_client().await;

    let result = client.get_public_key().id("KNOTEXIST").send().await;
    assert!(result.is_err());
}

// ============================================================================
// Ported from moto: test_cloudfront_distributions.py
// ============================================================================

// Ported from moto: test_cloudfront_distributions.py::test_create_distribution_needs_unique_caller_reference
#[tokio::test]
async fn test_create_distribution_needs_unique_caller_reference() {
    let client = make_cloudfront_client().await;

    // Create first distribution
    let resp1 = client
        .create_distribution()
        .distribution_config(make_distribution_config("ref"))
        .send()
        .await
        .unwrap();
    let dist1_id = resp1.distribution().unwrap().id().to_string();

    // Try to create distribution with the same caller reference
    let err = client
        .create_distribution()
        .distribution_config(make_distribution_config("ref"))
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("DistributionAlreadyExists"),
        "Expected DistributionAlreadyExists error, got: {err_str}"
    );
    assert!(
        err_str.contains(&dist1_id),
        "Expected error to contain dist ID {dist1_id}, got: {err_str}"
    );

    // Creating another distribution with a different reference should succeed
    let resp2 = client
        .create_distribution()
        .distribution_config(make_distribution_config("ref2"))
        .send()
        .await
        .unwrap();
    assert_ne!(dist1_id, resp2.distribution().unwrap().id());

    let list = client.list_distributions().send().await.unwrap();
    assert_eq!(list.distribution_list().unwrap().quantity(), 2);
}

// Ported from moto: test_cloudfront_distributions.py::test_list_distributions_without_any
#[tokio::test]
async fn test_list_distributions_without_any() {
    let client = make_cloudfront_client().await;

    let resp = client.list_distributions().send().await.unwrap();
    let dlist = resp.distribution_list().unwrap();
    assert_eq!(dlist.quantity(), 0);
    assert!(dlist.items().is_empty());
}

// Ported from moto: test_cloudfront_distributions.py::test_list_distributions (detailed)
#[tokio::test]
async fn test_list_distributions_detailed() {
    let client = make_cloudfront_client().await;

    let resp1 = client
        .create_distribution()
        .distribution_config(make_distribution_config("ref1-detail"))
        .send()
        .await
        .unwrap();
    let dist1_id = resp1.distribution().unwrap().id().to_string();

    let resp2 = client
        .create_distribution()
        .distribution_config(make_distribution_config("ref2-detail"))
        .send()
        .await
        .unwrap();
    let dist2_id = resp2.distribution().unwrap().id().to_string();

    let resp = client.list_distributions().send().await.unwrap();
    let dlist = resp.distribution_list().unwrap();
    assert_eq!(dlist.quantity(), 2);

    let items = dlist.items();
    assert_eq!(items.len(), 2);

    // Both items should have ARN and status
    for item in items {
        assert!(!item.arn().is_empty());
        assert_eq!(item.status(), "Deployed");
    }

    let ids: Vec<&str> = items.iter().map(|i| i.id()).collect();
    assert!(ids.contains(&dist1_id.as_str()));
    assert!(ids.contains(&dist2_id.as_str()));
}

// Ported from moto: test_cloudfront_distributions.py::test_get_distribution
#[tokio::test]
async fn test_get_distribution_details() {
    let client = make_cloudfront_client().await;

    let create_resp = client
        .create_distribution()
        .distribution_config(make_distribution_config("ref"))
        .send()
        .await
        .unwrap();
    let dist_id = create_resp.distribution().unwrap().id().to_string();

    let resp = client.get_distribution().id(&dist_id).send().await.unwrap();

    assert!(resp.e_tag().is_some());
    let dist = resp.distribution().unwrap();
    assert_eq!(dist.id(), dist_id);
    assert_eq!(dist.status(), "Deployed");
    assert!(dist.domain_name().contains("cloudfront.net"));

    let config = dist.distribution_config().unwrap();
    assert_eq!(config.caller_reference(), "ref");
}

// Ported from moto: test_cloudfront_distributions.py::test_delete_unknown_distribution
#[tokio::test]
async fn test_delete_unknown_distribution() {
    let client = make_cloudfront_client().await;

    let err = client
        .delete_distribution()
        .id("unknown")
        .if_match("..")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("NoSuchDistribution"),
        "Expected NoSuchDistribution error, got: {err_str}"
    );
}

// Ported from moto: test_cloudfront_distributions.py::test_delete_distribution_random_etag
#[tokio::test]
async fn test_delete_distribution_random_etag() {
    let client = make_cloudfront_client().await;

    let create_resp = client
        .create_distribution()
        .distribution_config(make_distribution_config("ref"))
        .send()
        .await
        .unwrap();
    let dist_id = create_resp.distribution().unwrap().id().to_string();

    // Delete with any etag should succeed (etag validation is loose)
    client
        .delete_distribution()
        .id(&dist_id)
        .if_match("anything")
        .send()
        .await
        .expect("delete_distribution with random etag should succeed");

    // Verify distribution is gone
    let err = client
        .get_distribution()
        .id(&dist_id)
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("NoSuchDistribution"),
        "Expected NoSuchDistribution, got: {err_str}"
    );
}

// ============================================================================
// Ported from moto: test_cloudfront_invalidation.py
// ============================================================================

// Ported from moto: test_cloudfront_invalidation.py::test_create_invalidation_with_single_path
#[tokio::test]
async fn test_create_invalidation_with_single_path() {
    let client = make_cloudfront_client().await;

    let create_resp = client
        .create_distribution()
        .distribution_config(make_distribution_config("ref"))
        .send()
        .await
        .unwrap();
    let dist_id = create_resp.distribution().unwrap().id().to_string();

    let resp = client
        .create_invalidation()
        .distribution_id(&dist_id)
        .invalidation_batch(
            aws_sdk_cloudfront::types::InvalidationBatch::builder()
                .caller_reference("ref2")
                .paths(
                    aws_sdk_cloudfront::types::Paths::builder()
                        .quantity(1)
                        .items("/path1")
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let inv = resp.invalidation().unwrap();
    assert!(!inv.id().is_empty());
    assert_eq!(inv.status(), "COMPLETED");
    // create_time is a required DateTime field, verify it's accessible
    let _ = inv.create_time();

    let batch = inv.invalidation_batch().unwrap();
    assert_eq!(batch.caller_reference(), "ref2");
    let paths = batch.paths().unwrap();
    assert_eq!(paths.quantity(), 1);
    assert_eq!(paths.items(), &["/path1"]);
}

// Ported from moto: test_cloudfront_invalidation.py::test_create_invalidation_with_multiple_paths
#[tokio::test]
async fn test_create_invalidation_with_multiple_paths() {
    let client = make_cloudfront_client().await;

    let create_resp = client
        .create_distribution()
        .distribution_config(make_distribution_config("ref"))
        .send()
        .await
        .unwrap();
    let dist_id = create_resp.distribution().unwrap().id().to_string();

    let resp = client
        .create_invalidation()
        .distribution_id(&dist_id)
        .invalidation_batch(
            aws_sdk_cloudfront::types::InvalidationBatch::builder()
                .caller_reference("ref2")
                .paths(
                    aws_sdk_cloudfront::types::Paths::builder()
                        .quantity(2)
                        .items("/path1")
                        .items("/path2")
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let inv = resp.invalidation().unwrap();
    assert!(!inv.id().is_empty());
    assert_eq!(inv.status(), "COMPLETED");

    let batch = inv.invalidation_batch().unwrap();
    assert_eq!(batch.caller_reference(), "ref2");
    let paths = batch.paths().unwrap();
    assert_eq!(paths.quantity(), 2);
    let path_items = paths.items();
    assert!(path_items.contains(&"/path1".to_string()));
    assert!(path_items.contains(&"/path2".to_string()));
}

// Ported from moto: test_cloudfront_invalidation.py::test_list_invalidations__no_entries
#[tokio::test]
async fn test_list_invalidations_no_entries() {
    let client = make_cloudfront_client().await;

    // Create a real distribution first so list_invalidations has a valid target
    let create_resp = client
        .create_distribution()
        .distribution_config(make_distribution_config("ref-inv-empty"))
        .send()
        .await
        .unwrap();
    let dist_id = create_resp.distribution().unwrap().id().to_string();

    let resp = client
        .list_invalidations()
        .distribution_id(&dist_id)
        .send()
        .await
        .expect("list_invalidations for valid dist with no invalidations should succeed");

    let list = resp.invalidation_list().unwrap();
    assert_eq!(list.quantity(), 0);
    assert!(list.items().is_empty());
}

// Ported from moto: test_cloudfront_invalidation.py::test_get_invalidation
#[tokio::test]
async fn test_get_invalidation_matches_create() {
    let client = make_cloudfront_client().await;

    let create_resp = client
        .create_distribution()
        .distribution_config(make_distribution_config("ref"))
        .send()
        .await
        .unwrap();
    let dist_id = create_resp.distribution().unwrap().id().to_string();

    let create_inv_resp = client
        .create_invalidation()
        .distribution_id(&dist_id)
        .invalidation_batch(
            aws_sdk_cloudfront::types::InvalidationBatch::builder()
                .caller_reference("ref2")
                .paths(
                    aws_sdk_cloudfront::types::Paths::builder()
                        .quantity(1)
                        .items("/path1")
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let created = create_inv_resp.invalidation().unwrap();
    let created_id = created.id().to_string();

    let get_resp = client
        .get_invalidation()
        .distribution_id(&dist_id)
        .id(&created_id)
        .send()
        .await
        .unwrap();

    let returned = get_resp.invalidation().unwrap();
    assert_eq!(returned.id(), created.id());
    assert_eq!(returned.status(), created.status());

    let returned_batch = returned.invalidation_batch().unwrap();
    let created_batch = created.invalidation_batch().unwrap();
    assert_eq!(
        returned_batch.caller_reference(),
        created_batch.caller_reference()
    );
    assert_eq!(
        returned_batch.paths().unwrap().items(),
        created_batch.paths().unwrap().items()
    );
}

// Ported from moto: test_cloudfront_invalidation.py::test_get_invalidation_dist_not_found
#[tokio::test]
async fn test_get_invalidation_dist_not_found() {
    let client = make_cloudfront_client().await;

    let err = client
        .get_invalidation()
        .distribution_id("notfound")
        .id("notfound")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("NoSuchDistribution"),
        "Expected NoSuchDistribution error, got: {err_str}"
    );
}

// Ported from moto: test_cloudfront_invalidation.py::test_get_invalidation_id_not_found
#[tokio::test]
async fn test_get_invalidation_id_not_found() {
    let client = make_cloudfront_client().await;

    let create_resp = client
        .create_distribution()
        .distribution_config(make_distribution_config("ref"))
        .send()
        .await
        .unwrap();
    let dist_id = create_resp.distribution().unwrap().id().to_string();

    let err = client
        .get_invalidation()
        .distribution_id(&dist_id)
        .id("notfound")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("NoSuchInvalidation"),
        "Expected NoSuchInvalidation error, got: {err_str}"
    );
}

// ============================================================================
// Ported from moto: test_cloudfront_oac.py
// ============================================================================

// Ported from moto: test_cloudfront_oac.py::test_create_origin_access_control (full CRUD)
#[tokio::test]
async fn test_oac_full_crud() {
    let client = make_cloudfront_client().await;

    // List should be empty initially
    let list_resp = client.list_origin_access_controls().send().await.unwrap();
    let oac_list = list_resp.origin_access_control_list().unwrap();
    assert!(oac_list.items().is_empty());

    // Create
    let create_resp = client
        .create_origin_access_control()
        .origin_access_control_config(
            aws_sdk_cloudfront::types::OriginAccessControlConfig::builder()
                .name("my_oac")
                .signing_protocol(
                    aws_sdk_cloudfront::types::OriginAccessControlSigningProtocols::Sigv4,
                )
                .signing_behavior(
                    aws_sdk_cloudfront::types::OriginAccessControlSigningBehaviors::Always,
                )
                .origin_access_control_origin_type(
                    aws_sdk_cloudfront::types::OriginAccessControlOriginTypes::S3,
                )
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let oac = create_resp.origin_access_control().unwrap();
    let control_id = oac.id().to_string();
    assert!(!control_id.is_empty());

    let config = oac.origin_access_control_config().unwrap();
    assert_eq!(config.name(), "my_oac");
    assert_eq!(config.signing_protocol().as_str(), "sigv4");
    assert_eq!(config.signing_behavior().as_str(), "always");
    assert_eq!(config.origin_access_control_origin_type().as_str(), "s3");

    // Get
    let get_resp = client
        .get_origin_access_control()
        .id(&control_id)
        .send()
        .await
        .unwrap();
    let got = get_resp.origin_access_control().unwrap();
    assert_eq!(got.id(), control_id);
    let got_config = got.origin_access_control_config().unwrap();
    assert_eq!(got_config.name(), "my_oac");

    // List should contain our OAC
    let list_resp2 = client.list_origin_access_controls().send().await.unwrap();
    let oac_list2 = list_resp2.origin_access_control_list().unwrap();
    assert_eq!(oac_list2.quantity(), 1);

    // Delete
    let etag = create_resp.e_tag().unwrap().to_string();
    client
        .delete_origin_access_control()
        .id(&control_id)
        .if_match(&etag)
        .send()
        .await
        .unwrap();

    // List should be empty
    let list_resp3 = client.list_origin_access_controls().send().await.unwrap();
    let oac_list3 = list_resp3.origin_access_control_list().unwrap();
    assert!(oac_list3.items().is_empty());

    // Get should fail
    let err = client
        .get_origin_access_control()
        .id(&control_id)
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("NoSuchOriginAccessControl"),
        "Expected NoSuchOriginAccessControl, got: {err_str}"
    );
}

// Ported from moto: test_cloudfront_oac.py::test_update_origin_access_control
#[tokio::test]
async fn test_update_origin_access_control_with_description() {
    let client = make_cloudfront_client().await;

    let create_resp = client
        .create_origin_access_control()
        .origin_access_control_config(
            aws_sdk_cloudfront::types::OriginAccessControlConfig::builder()
                .name("my_oac")
                .signing_protocol(
                    aws_sdk_cloudfront::types::OriginAccessControlSigningProtocols::Sigv4,
                )
                .signing_behavior(
                    aws_sdk_cloudfront::types::OriginAccessControlSigningBehaviors::Always,
                )
                .origin_access_control_origin_type(
                    aws_sdk_cloudfront::types::OriginAccessControlOriginTypes::S3,
                )
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let control_id = create_resp
        .origin_access_control()
        .unwrap()
        .id()
        .to_string();
    let etag = create_resp.e_tag().unwrap().to_string();

    let update_resp = client
        .update_origin_access_control()
        .id(&control_id)
        .if_match(&etag)
        .origin_access_control_config(
            aws_sdk_cloudfront::types::OriginAccessControlConfig::builder()
                .name("my_oac")
                .description("updated")
                .signing_protocol(
                    aws_sdk_cloudfront::types::OriginAccessControlSigningProtocols::Sigv4,
                )
                .signing_behavior(
                    aws_sdk_cloudfront::types::OriginAccessControlSigningBehaviors::Always,
                )
                .origin_access_control_origin_type(
                    aws_sdk_cloudfront::types::OriginAccessControlOriginTypes::S3,
                )
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let control = update_resp.origin_access_control().unwrap();
    assert_eq!(control.id(), control_id);
    let config = control.origin_access_control_config().unwrap();
    assert_eq!(config.name(), "my_oac");
    assert_eq!(config.description(), Some("updated"));
}

// ============================================================================
// Ported from moto: test_cloudfront_keys.py
// ============================================================================

// Ported from moto: test_cloudfront_keys.py::test_list_public_keys
#[tokio::test]
async fn test_list_public_keys_detailed() {
    let client = make_cloudfront_client().await;

    // Empty list
    let resp = client.list_public_keys().send().await.unwrap();
    let key_list = resp.public_key_list().unwrap();
    assert_eq!(key_list.quantity(), 0);

    // Create a key
    let create_resp = client
        .create_public_key()
        .public_key_config(
            aws_sdk_cloudfront::types::PublicKeyConfig::builder()
                .caller_reference("someref")
                .name("somekey")
                .encoded_key("some data")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let key = create_resp.public_key().unwrap();
    let key_id = key.id().to_string();
    // created_time is a required DateTime field, verify it's accessible
    let _ = key.created_time();

    // List should contain the key
    let resp = client.list_public_keys().send().await.unwrap();
    let key_list = resp.public_key_list().unwrap();
    assert_eq!(key_list.quantity(), 1);

    let items = key_list.items();
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].id(), key_id);
    assert_eq!(items[0].name(), "somekey");
}

// Ported from moto: test_cloudfront_keys.py::test_create_public_key_group
#[tokio::test]
async fn test_create_public_key_group() {
    let client = make_cloudfront_client().await;

    // Create a public key first
    let pk_resp = client
        .create_public_key()
        .public_key_config(
            aws_sdk_cloudfront::types::PublicKeyConfig::builder()
                .caller_reference("pk-ref")
                .name("pk-name")
                .encoded_key("-----BEGIN PUBLIC KEY-----\nMIIBIjANBg==\n-----END PUBLIC KEY-----")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();
    let key_id = pk_resp.public_key().unwrap().id().to_string();

    // Create key group
    let resp = client
        .create_key_group()
        .key_group_config(
            aws_sdk_cloudfront::types::KeyGroupConfig::builder()
                .name("mygroup")
                .items(&key_id)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let kg = resp.key_group().unwrap();
    let key_group_id = kg.id().to_string();
    assert!(!key_group_id.is_empty());

    let kg_config = kg.key_group_config().unwrap();
    assert_eq!(kg_config.name(), "mygroup");
    assert_eq!(kg_config.items(), std::slice::from_ref(&key_id));

    // Get key group
    let get_resp = client
        .get_key_group()
        .id(&key_group_id)
        .send()
        .await
        .unwrap();
    assert_eq!(get_resp.e_tag(), resp.e_tag());
    let got_kg = get_resp.key_group().unwrap();
    assert_eq!(got_kg.id(), key_group_id);
    assert_eq!(got_kg.key_group_config().unwrap().name(), "mygroup");
    assert_eq!(
        got_kg.key_group_config().unwrap().items(),
        std::slice::from_ref(&key_id)
    );

    // List key groups
    let list_resp = client.list_key_groups().send().await.unwrap();
    let groups = list_resp.key_group_list().unwrap();
    assert_eq!(groups.quantity(), 1);
    let items = groups.items();
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].key_group().unwrap().id(), key_group_id);
    assert_eq!(
        items[0]
            .key_group()
            .unwrap()
            .key_group_config()
            .unwrap()
            .name(),
        "mygroup"
    );
}

// ============================================================================
// Ported from moto: test_cloudfront_dist_tags.py
// ============================================================================

// Ported from moto: test_cloudfront_dist_tags.py::test_create_distribution_with_tags
#[tokio::test]
async fn test_create_distribution_with_multiple_tags() {
    let client = make_cloudfront_client().await;

    let tag1 = aws_sdk_cloudfront::types::Tag::builder()
        .key("k1")
        .value("v1")
        .build()
        .unwrap();
    let tag2 = aws_sdk_cloudfront::types::Tag::builder()
        .key("k2")
        .value("v2")
        .build()
        .unwrap();

    let resp = client
        .create_distribution_with_tags()
        .distribution_config_with_tags(
            aws_sdk_cloudfront::types::DistributionConfigWithTags::builder()
                .distribution_config(make_distribution_config("ref-tags-multi"))
                .tags(
                    aws_sdk_cloudfront::types::Tags::builder()
                        .items(tag1)
                        .items(tag2)
                        .build(),
                )
                .build(),
        )
        .send()
        .await
        .unwrap();

    let arn = resp.distribution().unwrap().arn().to_string();

    let tag_resp = client
        .list_tags_for_resource()
        .resource(&arn)
        .send()
        .await
        .unwrap();

    let tags = tag_resp.tags().unwrap();
    let items = tags.items();
    assert_eq!(items.len(), 2);

    let keys: Vec<&str> = items.iter().map(|t| t.key()).collect();
    assert!(keys.contains(&"k1"));
    assert!(keys.contains(&"k2"));
}

// Ported from moto: test_cloudfront_dist_tags.py::test_tag_resource_on_existing_distribution
#[tokio::test]
async fn test_tag_resource_on_existing_distribution() {
    let client = make_cloudfront_client().await;

    let resp = client
        .create_distribution()
        .distribution_config(make_distribution_config("ref-tag-existing"))
        .send()
        .await
        .unwrap();
    let dist_arn = resp.distribution().unwrap().arn().to_string();

    // Initially no tags
    let tag_resp = client
        .list_tags_for_resource()
        .resource(&dist_arn)
        .send()
        .await
        .unwrap();
    assert_eq!(tag_resp.tags().unwrap().items().len(), 0);

    // Add tags
    client
        .tag_resource()
        .resource(&dist_arn)
        .tags(
            aws_sdk_cloudfront::types::Tags::builder()
                .items(
                    aws_sdk_cloudfront::types::Tag::builder()
                        .key("k1")
                        .value("v1")
                        .build()
                        .unwrap(),
                )
                .build(),
        )
        .send()
        .await
        .unwrap();

    let tag_resp = client
        .list_tags_for_resource()
        .resource(&dist_arn)
        .send()
        .await
        .unwrap();
    let items = tag_resp.tags().unwrap().items();
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].key(), "k1");
    assert_eq!(items[0].value(), Some("v1"));
}

// Ported from moto: test_cloudfront_dist_tags.py::test_untag_resource_on_existing_distribution
#[tokio::test]
async fn test_untag_resource_on_existing_distribution() {
    let client = make_cloudfront_client().await;

    let resp = client
        .create_distribution()
        .distribution_config(make_distribution_config("ref-untag"))
        .send()
        .await
        .unwrap();
    let dist_arn = resp.distribution().unwrap().arn().to_string();

    // Add two tags
    client
        .tag_resource()
        .resource(&dist_arn)
        .tags(
            aws_sdk_cloudfront::types::Tags::builder()
                .items(
                    aws_sdk_cloudfront::types::Tag::builder()
                        .key("k1")
                        .value("v1")
                        .build()
                        .unwrap(),
                )
                .items(
                    aws_sdk_cloudfront::types::Tag::builder()
                        .key("k2")
                        .value("v2")
                        .build()
                        .unwrap(),
                )
                .build(),
        )
        .send()
        .await
        .unwrap();

    let tag_resp = client
        .list_tags_for_resource()
        .resource(&dist_arn)
        .send()
        .await
        .unwrap();
    assert_eq!(tag_resp.tags().unwrap().items().len(), 2);

    // Remove one tag
    client
        .untag_resource()
        .resource(&dist_arn)
        .tag_keys(
            aws_sdk_cloudfront::types::TagKeys::builder()
                .items("k1")
                .build(),
        )
        .send()
        .await
        .unwrap();

    let tag_resp = client
        .list_tags_for_resource()
        .resource(&dist_arn)
        .send()
        .await
        .unwrap();
    let items = tag_resp.tags().unwrap().items();
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].key(), "k2");
    assert_eq!(items[0].value(), Some("v2"));
}

// ============================================================================
// Tests derived from AWS documentation: Amazon CloudFront
// ============================================================================

#[tokio::test]
async fn test_update_distribution_wrong_etag() {
    let client = make_cloudfront_client().await;

    let create_resp = client
        .create_distribution()
        .distribution_config(make_distribution_config("ref-wrong-etag"))
        .send()
        .await
        .unwrap();
    let dist_id = create_resp.distribution().unwrap().id().to_string();

    let result = client
        .update_distribution()
        .id(&dist_id)
        .if_match("WRONG-ETAG")
        .distribution_config(make_distribution_config("ref-wrong-etag"))
        .send()
        .await;

    assert!(
        result.is_err(),
        "update_distribution with wrong ETag should fail"
    );
}

#[tokio::test]
async fn test_update_distribution_nonexistent() {
    let client = make_cloudfront_client().await;

    let result = client
        .update_distribution()
        .id("NONEXISTENT")
        .if_match("ETAG123")
        .distribution_config(make_distribution_config("ref-nonexistent"))
        .send()
        .await;

    assert!(
        result.is_err(),
        "update_distribution on nonexistent should fail"
    );
}

#[tokio::test]
async fn test_get_key_group_not_found() {
    let client = make_cloudfront_client().await;

    let result = client
        .get_key_group()
        .id("nonexistent-key-group-id")
        .send()
        .await;

    assert!(result.is_err(), "get_key_group on nonexistent should fail");
}

#[tokio::test]
async fn test_list_key_groups_empty() {
    let client = make_cloudfront_client().await;

    let resp = client
        .list_key_groups()
        .send()
        .await
        .expect("list_key_groups on empty state should succeed");

    let list = resp.key_group_list().unwrap();
    assert_eq!(list.items().len(), 0);
}

#[tokio::test]
async fn test_list_origin_access_controls_empty() {
    let client = make_cloudfront_client().await;

    let resp = client
        .list_origin_access_controls()
        .send()
        .await
        .expect("list_origin_access_controls on empty state should succeed");

    let list = resp.origin_access_control_list().unwrap();
    assert_eq!(list.items().len(), 0);
}

#[tokio::test]
async fn test_list_public_keys_empty() {
    let client = make_cloudfront_client().await;

    let resp = client
        .list_public_keys()
        .send()
        .await
        .expect("list_public_keys on empty state should succeed");

    let list = resp.public_key_list().unwrap();
    assert_eq!(list.items().len(), 0);
}

#[tokio::test]
async fn test_list_invalidations_nonexistent_distribution() {
    let client = make_cloudfront_client().await;

    let result = client
        .list_invalidations()
        .distribution_id("nonexistent-dist")
        .send()
        .await;

    assert!(
        result.is_err(),
        "list_invalidations on nonexistent distribution should fail"
    );
}

#[tokio::test]
async fn test_update_distribution_disable() {
    let client = make_cloudfront_client().await;

    let create_resp = client
        .create_distribution()
        .distribution_config(make_distribution_config("ref-disable"))
        .send()
        .await
        .unwrap();
    let dist_id = create_resp.distribution().unwrap().id().to_string();
    let etag = create_resp.e_tag().unwrap().to_string();

    let disabled_config = aws_sdk_cloudfront::types::DistributionConfig::builder()
        .caller_reference("ref-disable")
        .origins(
            aws_sdk_cloudfront::types::Origins::builder()
                .quantity(1)
                .items(
                    aws_sdk_cloudfront::types::Origin::builder()
                        .id("my-origin")
                        .domain_name("my-bucket.s3.amazonaws.com")
                        .s3_origin_config(
                            aws_sdk_cloudfront::types::S3OriginConfig::builder()
                                .origin_access_identity("")
                                .build(),
                        )
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        )
        .default_cache_behavior(
            aws_sdk_cloudfront::types::DefaultCacheBehavior::builder()
                .target_origin_id("my-origin")
                .viewer_protocol_policy(aws_sdk_cloudfront::types::ViewerProtocolPolicy::AllowAll)
                .build()
                .unwrap(),
        )
        .comment("")
        .enabled(false)
        .build()
        .unwrap();

    client
        .update_distribution()
        .id(&dist_id)
        .if_match(&etag)
        .distribution_config(disabled_config)
        .send()
        .await
        .expect("update_distribution with enabled=false should succeed");

    let get_resp = client
        .get_distribution_config()
        .id(&dist_id)
        .send()
        .await
        .unwrap();

    assert!(!get_resp.distribution_config().unwrap().enabled());
}

#[tokio::test]
async fn test_list_tags_for_nonexistent_resource() {
    let client = make_cloudfront_client().await;

    let result = client
        .list_tags_for_resource()
        .resource("arn:aws:cloudfront::123456789012:distribution/NONEXISTENT")
        .send()
        .await;

    assert!(
        result.is_err(),
        "list_tags_for_resource on nonexistent resource should fail"
    );
}

// ---- Field-Level Encryption tests ----

#[tokio::test]
async fn test_field_level_encryption_config_crud() {
    let client = make_cloudfront_client().await;

    let config = aws_sdk_cloudfront::types::FieldLevelEncryptionConfig::builder()
        .caller_reference("fle-ref-1")
        .comment("test fle config")
        .query_arg_profile_config(
            aws_sdk_cloudfront::types::QueryArgProfileConfig::builder()
                .forward_when_query_arg_profile_is_unknown(true)
                .build()
                .unwrap(),
        )
        .content_type_profile_config(
            aws_sdk_cloudfront::types::ContentTypeProfileConfig::builder()
                .forward_when_content_type_is_unknown(true)
                .build()
                .unwrap(),
        )
        .build()
        .unwrap();

    let create_resp = client
        .create_field_level_encryption_config()
        .field_level_encryption_config(config.clone())
        .send()
        .await
        .expect("create_field_level_encryption_config should succeed");

    let fle = create_resp
        .field_level_encryption()
        .expect("should have fle");
    let fle_id = fle.id().to_string();
    let etag = create_resp.e_tag().unwrap().to_string();
    assert!(!fle_id.is_empty());

    // Get
    let get_resp = client
        .get_field_level_encryption()
        .id(&fle_id)
        .send()
        .await
        .expect("get_field_level_encryption should succeed");
    assert_eq!(get_resp.field_level_encryption().unwrap().id(), fle_id);

    // List
    let list_resp = client
        .list_field_level_encryption_configs()
        .send()
        .await
        .expect("list_field_level_encryption_configs should succeed");
    assert!(
        list_resp
            .field_level_encryption_list()
            .unwrap()
            .items()
            .iter()
            .any(|i| i.id() == fle_id)
    );

    // Update
    let updated_config = aws_sdk_cloudfront::types::FieldLevelEncryptionConfig::builder()
        .caller_reference("fle-ref-1")
        .comment("updated comment")
        .query_arg_profile_config(
            aws_sdk_cloudfront::types::QueryArgProfileConfig::builder()
                .forward_when_query_arg_profile_is_unknown(true)
                .build()
                .unwrap(),
        )
        .content_type_profile_config(
            aws_sdk_cloudfront::types::ContentTypeProfileConfig::builder()
                .forward_when_content_type_is_unknown(true)
                .build()
                .unwrap(),
        )
        .build()
        .unwrap();
    let update_resp = client
        .update_field_level_encryption_config()
        .id(&fle_id)
        .if_match(&etag)
        .field_level_encryption_config(updated_config)
        .send()
        .await
        .expect("update_field_level_encryption_config should succeed");
    let new_etag = update_resp.e_tag().unwrap().to_string();

    // Delete
    client
        .delete_field_level_encryption_config()
        .id(&fle_id)
        .if_match(&new_etag)
        .send()
        .await
        .expect("delete_field_level_encryption_config should succeed");

    // Verify gone
    let get_result = client.get_field_level_encryption().id(&fle_id).send().await;
    assert!(get_result.is_err(), "get after delete should fail");
}

#[tokio::test]
async fn test_field_level_encryption_profile_crud() {
    let client = make_cloudfront_client().await;

    let config = aws_sdk_cloudfront::types::FieldLevelEncryptionProfileConfig::builder()
        .caller_reference("flep-ref-1")
        .name("test-fle-profile")
        .encryption_entities(
            aws_sdk_cloudfront::types::EncryptionEntities::builder()
                .quantity(0)
                .build()
                .unwrap(),
        )
        .build()
        .unwrap();

    let create_resp = client
        .create_field_level_encryption_profile()
        .field_level_encryption_profile_config(config)
        .send()
        .await
        .expect("create_field_level_encryption_profile should succeed");

    let profile = create_resp
        .field_level_encryption_profile()
        .expect("should have profile");
    let profile_id = profile.id().to_string();
    let etag = create_resp.e_tag().unwrap().to_string();
    assert!(!profile_id.is_empty());

    // Get
    let get_resp = client
        .get_field_level_encryption_profile()
        .id(&profile_id)
        .send()
        .await
        .expect("get_field_level_encryption_profile should succeed");
    assert_eq!(
        get_resp.field_level_encryption_profile().unwrap().id(),
        profile_id
    );

    // List
    let list_resp = client
        .list_field_level_encryption_profiles()
        .send()
        .await
        .expect("list_field_level_encryption_profiles should succeed");
    assert!(
        list_resp
            .field_level_encryption_profile_list()
            .unwrap()
            .items()
            .iter()
            .any(|i| i.id() == profile_id)
    );

    // Update
    let updated_config = aws_sdk_cloudfront::types::FieldLevelEncryptionProfileConfig::builder()
        .caller_reference("flep-ref-1")
        .name("updated-fle-profile")
        .encryption_entities(
            aws_sdk_cloudfront::types::EncryptionEntities::builder()
                .quantity(0)
                .build()
                .unwrap(),
        )
        .build()
        .unwrap();
    let update_resp = client
        .update_field_level_encryption_profile()
        .id(&profile_id)
        .if_match(&etag)
        .field_level_encryption_profile_config(updated_config)
        .send()
        .await
        .expect("update_field_level_encryption_profile should succeed");
    let new_etag = update_resp.e_tag().unwrap().to_string();

    // Delete
    client
        .delete_field_level_encryption_profile()
        .id(&profile_id)
        .if_match(&new_etag)
        .send()
        .await
        .expect("delete_field_level_encryption_profile should succeed");

    // Verify gone
    let get_result = client
        .get_field_level_encryption_profile()
        .id(&profile_id)
        .send()
        .await;
    assert!(get_result.is_err(), "get after delete should fail");
}

// ---- Key Value Store tests ----

#[tokio::test]
async fn test_key_value_store_crud() {
    let client = make_cloudfront_client().await;

    let create_resp = client
        .create_key_value_store()
        .name("test-kvs")
        .comment("test key value store")
        .send()
        .await
        .expect("create_key_value_store should succeed");

    let kvs = create_resp.key_value_store().expect("should have kvs");
    assert_eq!(kvs.name(), "test-kvs");
    let etag = create_resp.e_tag().unwrap().to_string();

    // Describe
    let describe_resp = client
        .describe_key_value_store()
        .name("test-kvs")
        .send()
        .await
        .expect("describe_key_value_store should succeed");
    assert_eq!(describe_resp.key_value_store().unwrap().name(), "test-kvs");

    // List
    let list_resp = client
        .list_key_value_stores()
        .send()
        .await
        .expect("list_key_value_stores should succeed");
    assert!(
        list_resp
            .key_value_store_list()
            .unwrap()
            .items()
            .iter()
            .any(|i| i.name() == "test-kvs")
    );

    // Update
    let update_resp = client
        .update_key_value_store()
        .name("test-kvs")
        .if_match(&etag)
        .comment("updated comment")
        .send()
        .await
        .expect("update_key_value_store should succeed");
    let new_etag = update_resp.e_tag().unwrap().to_string();

    // Delete
    client
        .delete_key_value_store()
        .name("test-kvs")
        .if_match(&new_etag)
        .send()
        .await
        .expect("delete_key_value_store should succeed");

    // Verify gone
    let result = client
        .describe_key_value_store()
        .name("test-kvs")
        .send()
        .await;
    assert!(result.is_err(), "describe after delete should fail");
}

#[tokio::test]
async fn test_key_value_store_duplicate_fails() {
    let client = make_cloudfront_client().await;

    client
        .create_key_value_store()
        .name("kvs-dup")
        .send()
        .await
        .expect("first create should succeed");

    let result = client.create_key_value_store().name("kvs-dup").send().await;
    assert!(result.is_err(), "duplicate kvs creation should fail");
}

// ---- Realtime Log Config tests ----

#[tokio::test]
async fn test_realtime_log_config_crud() {
    let client = make_cloudfront_client().await;

    let endpoint = aws_sdk_cloudfront::types::EndPoint::builder()
        .stream_type("Kinesis")
        .kinesis_stream_config(
            aws_sdk_cloudfront::types::KinesisStreamConfig::builder()
                .role_arn("arn:aws:iam::123456789012:role/CloudFrontRealtimeLogConfigRole")
                .stream_arn("arn:aws:kinesis:us-east-1:123456789012:stream/test-stream")
                .build()
                .unwrap(),
        )
        .build()
        .unwrap();

    let create_resp = client
        .create_realtime_log_config()
        .name("test-rtlc")
        .sampling_rate(100)
        .end_points(endpoint.clone())
        .fields("timestamp")
        .fields("c-ip")
        .send()
        .await
        .expect("create_realtime_log_config should succeed");

    let cfg = create_resp
        .realtime_log_config()
        .expect("should have config");
    assert_eq!(cfg.name(), "test-rtlc");
    assert_eq!(cfg.sampling_rate(), 100);

    // Get by name
    let get_resp = client
        .get_realtime_log_config()
        .name("test-rtlc")
        .send()
        .await
        .expect("get_realtime_log_config should succeed");
    assert_eq!(get_resp.realtime_log_config().unwrap().name(), "test-rtlc");

    // List
    let list_resp = client
        .list_realtime_log_configs()
        .send()
        .await
        .expect("list_realtime_log_configs should succeed");
    assert!(
        list_resp
            .realtime_log_configs()
            .unwrap()
            .items()
            .iter()
            .any(|c| c.name() == "test-rtlc")
    );

    // Update
    let update_resp = client
        .update_realtime_log_config()
        .name("test-rtlc")
        .sampling_rate(50)
        .end_points(endpoint)
        .fields("timestamp")
        .send()
        .await
        .expect("update_realtime_log_config should succeed");
    assert_eq!(
        update_resp.realtime_log_config().unwrap().sampling_rate(),
        50
    );

    // Delete
    client
        .delete_realtime_log_config()
        .name("test-rtlc")
        .send()
        .await
        .expect("delete_realtime_log_config should succeed");

    // Verify gone
    let result = client
        .get_realtime_log_config()
        .name("test-rtlc")
        .send()
        .await;
    assert!(result.is_err(), "get after delete should fail");
}

// ---- Streaming Distribution tests ----

#[tokio::test]
async fn test_streaming_distribution_crud() {
    let client = make_cloudfront_client().await;

    let sd_config = aws_sdk_cloudfront::types::StreamingDistributionConfig::builder()
        .caller_reference("sd-ref-1")
        .s3_origin(
            aws_sdk_cloudfront::types::S3Origin::builder()
                .domain_name("my-bucket.s3.amazonaws.com")
                .origin_access_identity("origin-access-identity/cloudfront/ABCDEF")
                .build()
                .unwrap(),
        )
        .comment("test streaming distribution")
        .trusted_signers(
            aws_sdk_cloudfront::types::TrustedSigners::builder()
                .enabled(false)
                .quantity(0)
                .build()
                .unwrap(),
        )
        .enabled(true)
        .build()
        .unwrap();

    let create_resp = client
        .create_streaming_distribution()
        .streaming_distribution_config(sd_config.clone())
        .send()
        .await
        .expect("create_streaming_distribution should succeed");

    let sd = create_resp
        .streaming_distribution()
        .expect("should have streaming distribution");
    let sd_id = sd.id().to_string();
    let etag = create_resp.e_tag().unwrap().to_string();
    assert!(!sd_id.is_empty());
    assert!(sd.domain_name().contains("cloudfront.net"));

    // Get
    let get_resp = client
        .get_streaming_distribution()
        .id(&sd_id)
        .send()
        .await
        .expect("get_streaming_distribution should succeed");
    assert_eq!(get_resp.streaming_distribution().unwrap().id(), sd_id);

    // List
    let list_resp = client
        .list_streaming_distributions()
        .send()
        .await
        .expect("list_streaming_distributions should succeed");
    assert!(
        list_resp
            .streaming_distribution_list()
            .unwrap()
            .items()
            .iter()
            .any(|i| i.id() == sd_id)
    );

    // Update
    let updated_config = aws_sdk_cloudfront::types::StreamingDistributionConfig::builder()
        .caller_reference("sd-ref-1")
        .s3_origin(
            aws_sdk_cloudfront::types::S3Origin::builder()
                .domain_name("my-other-bucket.s3.amazonaws.com")
                .origin_access_identity("origin-access-identity/cloudfront/ABCDEF")
                .build()
                .unwrap(),
        )
        .comment("updated streaming distribution")
        .trusted_signers(
            aws_sdk_cloudfront::types::TrustedSigners::builder()
                .enabled(false)
                .quantity(0)
                .build()
                .unwrap(),
        )
        .enabled(true)
        .build()
        .unwrap();
    let update_resp = client
        .update_streaming_distribution()
        .id(&sd_id)
        .if_match(&etag)
        .streaming_distribution_config(updated_config)
        .send()
        .await
        .expect("update_streaming_distribution should succeed");
    let new_etag = update_resp.e_tag().unwrap().to_string();

    // Delete
    client
        .delete_streaming_distribution()
        .id(&sd_id)
        .if_match(&new_etag)
        .send()
        .await
        .expect("delete_streaming_distribution should succeed");

    // Verify gone
    let result = client.get_streaming_distribution().id(&sd_id).send().await;
    assert!(result.is_err(), "get after delete should fail");
}

// ---- Trust Store tests ----

#[tokio::test]
async fn test_trust_store_crud() {
    let client = make_cloudfront_client().await;

    let create_resp = client
        .create_trust_store()
        .name("test-trust-store")
        .send()
        .await
        .expect("create_trust_store should succeed");

    let ts = create_resp.trust_store().expect("should have trust store");
    let ts_id = ts.id().unwrap().to_string();
    let etag = create_resp.e_tag().unwrap().to_string();
    assert!(!ts_id.is_empty());
    assert_eq!(ts.name(), Some("test-trust-store"));

    // Get
    let get_resp = client
        .get_trust_store()
        .identifier(&ts_id)
        .send()
        .await
        .expect("get_trust_store should succeed");
    assert_eq!(get_resp.trust_store().unwrap().id(), Some(ts_id.as_str()));

    // List
    let list_resp = client
        .list_trust_stores()
        .send()
        .await
        .expect("list_trust_stores should succeed");
    assert!(
        list_resp
            .trust_store_list()
            .iter()
            .any(|i: &aws_sdk_cloudfront::types::TrustStoreSummary| i.id() == ts_id.as_str())
    );

    // Update
    let update_resp = client
        .update_trust_store()
        .id(&ts_id)
        .if_match(&etag)
        .send()
        .await
        .expect("update_trust_store should succeed");
    let new_etag = update_resp.e_tag().unwrap().to_string();

    // Delete
    client
        .delete_trust_store()
        .id(&ts_id)
        .if_match(&new_etag)
        .send()
        .await
        .expect("delete_trust_store should succeed");

    // Verify gone (use list to check absence instead)
    let list_after = client
        .list_trust_stores()
        .send()
        .await
        .expect("list after delete should succeed");
    assert!(
        !list_after
            .trust_store_list()
            .iter()
            .any(|i: &aws_sdk_cloudfront::types::TrustStoreSummary| i.id() == ts_id.as_str())
    );
}

// ---- VPC Origin tests ----

#[tokio::test]
async fn test_vpc_origin_crud() {
    let client = make_cloudfront_client().await;

    let vpc_config = aws_sdk_cloudfront::types::VpcOriginEndpointConfig::builder()
        .name("test-vpc-origin")
        .arn("arn:aws:elasticloadbalancing:us-east-1:123456789012:loadbalancer/app/my-lb/1234")
        .http_port(80)
        .https_port(443)
        .origin_protocol_policy(aws_sdk_cloudfront::types::OriginProtocolPolicy::HttpsOnly)
        .build()
        .unwrap();

    let create_resp = client
        .create_vpc_origin()
        .vpc_origin_endpoint_config(vpc_config.clone())
        .send()
        .await
        .expect("create_vpc_origin should succeed");

    let vpc = create_resp.vpc_origin().expect("should have vpc origin");
    let vpc_id = vpc.id().to_string();
    let etag = create_resp.e_tag().unwrap().to_string();
    assert!(!vpc_id.is_empty());

    // Get
    let get_resp = client
        .get_vpc_origin()
        .id(&vpc_id)
        .send()
        .await
        .expect("get_vpc_origin should succeed");
    assert_eq!(get_resp.vpc_origin().unwrap().id(), vpc_id);

    // List
    let list_resp = client
        .list_vpc_origins()
        .send()
        .await
        .expect("list_vpc_origins should succeed");
    assert!(
        list_resp
            .vpc_origin_list()
            .unwrap()
            .items()
            .iter()
            .any(|i| i.id() == vpc_id)
    );

    // Update
    let updated_vpc_config = aws_sdk_cloudfront::types::VpcOriginEndpointConfig::builder()
        .name("updated-vpc-origin")
        .arn("arn:aws:elasticloadbalancing:us-east-1:123456789012:loadbalancer/app/my-lb/1234")
        .http_port(8080)
        .https_port(8443)
        .origin_protocol_policy(aws_sdk_cloudfront::types::OriginProtocolPolicy::HttpsOnly)
        .build()
        .unwrap();
    let update_resp = client
        .update_vpc_origin()
        .id(&vpc_id)
        .if_match(&etag)
        .vpc_origin_endpoint_config(updated_vpc_config)
        .send()
        .await
        .expect("update_vpc_origin should succeed");
    let new_etag = update_resp.e_tag().unwrap().to_string();

    // Delete
    let delete_resp = client
        .delete_vpc_origin()
        .id(&vpc_id)
        .if_match(&new_etag)
        .send()
        .await
        .expect("delete_vpc_origin should succeed");
    assert_eq!(delete_resp.vpc_origin().unwrap().id(), vpc_id);

    // Verify gone
    let result = client.get_vpc_origin().id(&vpc_id).send().await;
    assert!(result.is_err(), "get after delete should fail");
}

// ---- OAI update test ----

#[tokio::test]
async fn test_update_cloud_front_origin_access_identity() {
    let client = make_cloudfront_client().await;

    let create_resp = client
        .create_cloud_front_origin_access_identity()
        .cloud_front_origin_access_identity_config(
            aws_sdk_cloudfront::types::CloudFrontOriginAccessIdentityConfig::builder()
                .caller_reference("oai-update-ref")
                .comment("original comment")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("create OAI should succeed");

    let oai = create_resp.cloud_front_origin_access_identity().unwrap();
    let oai_id = oai.id().to_string();
    let etag = create_resp.e_tag().unwrap().to_string();

    let update_resp = client
        .update_cloud_front_origin_access_identity()
        .id(&oai_id)
        .if_match(&etag)
        .cloud_front_origin_access_identity_config(
            aws_sdk_cloudfront::types::CloudFrontOriginAccessIdentityConfig::builder()
                .caller_reference("oai-update-ref")
                .comment("updated comment")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("update OAI should succeed");

    let updated_oai = update_resp.cloud_front_origin_access_identity().unwrap();
    assert_eq!(updated_oai.id(), oai_id);
    assert_eq!(
        updated_oai
            .cloud_front_origin_access_identity_config()
            .unwrap()
            .comment(),
        "updated comment"
    );
}

// ---- Anycast IP List tests ----

#[tokio::test]
async fn test_anycast_ip_list_crud() {
    let client = make_cloudfront_client().await;

    // Create
    let create_resp = client
        .create_anycast_ip_list()
        .name("test-anycast-list")
        .ip_count(3)
        .send()
        .await
        .expect("create_anycast_ip_list should succeed");

    let list = create_resp
        .anycast_ip_list()
        .expect("should have anycast_ip_list");
    let list_id = list.id().to_string();
    let etag = create_resp.e_tag().unwrap().to_string();

    assert!(!list_id.is_empty());
    assert_eq!(list.name(), "test-anycast-list");
    assert_eq!(list.ip_count(), 3);
    assert_eq!(list.status(), "Deployed");
    assert!(!list.arn().is_empty());

    // Get
    let get_resp = client
        .get_anycast_ip_list()
        .id(&list_id)
        .send()
        .await
        .expect("get_anycast_ip_list should succeed");

    let got = get_resp
        .anycast_ip_list()
        .expect("should have anycast_ip_list");
    assert_eq!(got.id(), list_id);
    assert_eq!(got.name(), "test-anycast-list");
    assert_eq!(got.ip_count(), 3);

    // List
    let list_resp = client
        .list_anycast_ip_lists()
        .send()
        .await
        .expect("list_anycast_ip_lists should succeed");

    let collection = list_resp
        .anycast_ip_lists()
        .expect("should have anycast_ip_lists collection");
    assert!(collection.items().iter().any(|i| i.id() == list_id));

    // Update
    let _update_resp = client
        .update_anycast_ip_list()
        .id(&list_id)
        .if_match(&etag)
        .ip_address_type(aws_sdk_cloudfront::types::IpAddressType::Ipv4)
        .send()
        .await
        .expect("update_anycast_ip_list should succeed");

    // Delete
    client
        .delete_anycast_ip_list()
        .id(&list_id)
        .send()
        .await
        .expect("delete_anycast_ip_list should succeed");

    // Verify gone
    let result = client.get_anycast_ip_list().id(&list_id).send().await;
    assert!(result.is_err(), "get after delete should fail");
}

#[tokio::test]
async fn test_anycast_ip_list_not_found() {
    let client = make_cloudfront_client().await;

    let result = client
        .get_anycast_ip_list()
        .id("nonexistent-id")
        .send()
        .await;
    assert!(
        result.is_err(),
        "get nonexistent anycast IP list should fail"
    );
}

#[tokio::test]
async fn test_list_distributions_by_anycast_ip_list_id() {
    let client = make_cloudfront_client().await;

    // Create an anycast IP list first
    let create_resp = client
        .create_anycast_ip_list()
        .name("test-list-for-distributions")
        .ip_count(3)
        .send()
        .await
        .expect("create should succeed");

    let list_id = create_resp.anycast_ip_list().unwrap().id().to_string();

    // List distributions - should succeed with 0 results
    let resp = client
        .list_distributions_by_anycast_ip_list_id()
        .anycast_ip_list_id(&list_id)
        .send()
        .await
        .expect("list_distributions_by_anycast_ip_list_id should succeed");

    assert_eq!(
        resp.distribution_list()
            .map(|l| l.items().len())
            .unwrap_or(0),
        0
    );

    // Cleanup
    let etag = create_resp.e_tag().unwrap().to_string();
    client
        .delete_anycast_ip_list()
        .id(&list_id)
        .send()
        .await
        .expect("cleanup delete should succeed");
    let _ = etag;
}

#[tokio::test]
async fn test_list_distributions_by_anycast_ip_list_id_not_found() {
    let client = make_cloudfront_client().await;

    let result = client
        .list_distributions_by_anycast_ip_list_id()
        .anycast_ip_list_id("nonexistent-id")
        .send()
        .await;
    assert!(
        result.is_err(),
        "listing for nonexistent anycast list should fail"
    );
}

// ---- ListDistributionsByVpcOriginId tests ----

#[tokio::test]
async fn test_list_distributions_by_vpc_origin_id() {
    let client = make_cloudfront_client().await;

    // Create a VPC origin
    let vpc_config = aws_sdk_cloudfront::types::VpcOriginEndpointConfig::builder()
        .name("test-vpc-for-list-by-id")
        .arn("arn:aws:elasticloadbalancing:us-east-1:123456789012:loadbalancer/app/my-lb/abcd")
        .http_port(80)
        .https_port(443)
        .origin_protocol_policy(aws_sdk_cloudfront::types::OriginProtocolPolicy::HttpsOnly)
        .build()
        .unwrap();

    let create_resp = client
        .create_vpc_origin()
        .vpc_origin_endpoint_config(vpc_config)
        .send()
        .await
        .expect("create_vpc_origin should succeed");

    let vpc_id = create_resp.vpc_origin().unwrap().id().to_string();

    // List distributions by VPC origin ID - should succeed with 0 results
    let resp = client
        .list_distributions_by_vpc_origin_id()
        .vpc_origin_id(&vpc_id)
        .send()
        .await
        .expect("list_distributions_by_vpc_origin_id should succeed");

    assert_eq!(
        resp.distribution_id_list()
            .map(|l| l.items().len())
            .unwrap_or(0),
        0
    );
}

#[tokio::test]
async fn test_list_distributions_by_vpc_origin_id_not_found() {
    let client = make_cloudfront_client().await;

    let result = client
        .list_distributions_by_vpc_origin_id()
        .vpc_origin_id("nonexistent-vpc-origin-id")
        .send()
        .await;
    assert!(
        result.is_err(),
        "listing for nonexistent VPC origin should fail"
    );
}

// ============================================================================
// Coverage for FIX(terraform-e2e) handler fixes
// ============================================================================

/// Helper: build a DistributionConfig with Restrictions, ViewerCertificate,
/// and HttpVersion so we can verify the full config round-trips.
fn make_full_distribution_config(
    caller_ref: &str,
) -> aws_sdk_cloudfront::types::DistributionConfig {
    aws_sdk_cloudfront::types::DistributionConfig::builder()
        .caller_reference(caller_ref)
        .origins(
            aws_sdk_cloudfront::types::Origins::builder()
                .quantity(1)
                .items(
                    aws_sdk_cloudfront::types::Origin::builder()
                        .id("my-origin")
                        .domain_name("my-bucket.s3.amazonaws.com")
                        .s3_origin_config(
                            aws_sdk_cloudfront::types::S3OriginConfig::builder()
                                .origin_access_identity("")
                                .build(),
                        )
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        )
        .default_cache_behavior(
            aws_sdk_cloudfront::types::DefaultCacheBehavior::builder()
                .target_origin_id("my-origin")
                .viewer_protocol_policy(
                    aws_sdk_cloudfront::types::ViewerProtocolPolicy::RedirectToHttps,
                )
                .build()
                .unwrap(),
        )
        .comment("full-config-test")
        .enabled(true)
        .restrictions(
            aws_sdk_cloudfront::types::Restrictions::builder()
                .geo_restriction(
                    aws_sdk_cloudfront::types::GeoRestriction::builder()
                        .restriction_type(aws_sdk_cloudfront::types::GeoRestrictionType::Whitelist)
                        .quantity(1)
                        .items("US")
                        .build()
                        .unwrap(),
                )
                .build(),
        )
        .viewer_certificate(
            aws_sdk_cloudfront::types::ViewerCertificate::builder()
                .cloud_front_default_certificate(true)
                .minimum_protocol_version(
                    aws_sdk_cloudfront::types::MinimumProtocolVersion::TlSv12016,
                )
                .build(),
        )
        .http_version(aws_sdk_cloudfront::types::HttpVersion::Http2)
        .build()
        .unwrap()
}

// Covers FIX(terraform-e2e): parse the FULL DistributionConfig from the request
// body so that ALL fields (Restrictions, ViewerCertificate, HttpVersion, etc.)
// are echoed back in GetDistribution responses.  Without this the Go provider
// panics with nil pointer dereferences.
#[tokio::test]
async fn test_full_distribution_config_round_trips_via_get_distribution() {
    let client = make_cloudfront_client().await;

    let create_resp = client
        .create_distribution()
        .distribution_config(make_full_distribution_config("ref-full-config-get"))
        .send()
        .await
        .expect("create_distribution should succeed");
    let dist_id = create_resp.distribution().unwrap().id().to_string();

    let get_resp = client
        .get_distribution()
        .id(&dist_id)
        .send()
        .await
        .expect("get_distribution should succeed");

    let dist = get_resp.distribution().unwrap();
    let config = dist.distribution_config().unwrap();

    // Restrictions.GeoRestriction must be present
    let restrictions = config
        .restrictions()
        .expect("restrictions should be present");
    let geo = restrictions
        .geo_restriction()
        .expect("geo_restriction should be present");
    assert_eq!(
        geo.restriction_type(),
        &aws_sdk_cloudfront::types::GeoRestrictionType::Whitelist,
        "restriction_type should round-trip"
    );
    assert_eq!(geo.quantity(), 1);
    assert_eq!(geo.items(), &["US"]);

    // ViewerCertificate must be present
    let vc = config
        .viewer_certificate()
        .expect("viewer_certificate should be present");
    assert_eq!(
        vc.cloud_front_default_certificate(),
        Some(true),
        "cloud_front_default_certificate should round-trip"
    );
    assert_eq!(
        vc.minimum_protocol_version(),
        Some(&aws_sdk_cloudfront::types::MinimumProtocolVersion::TlSv12016),
        "minimum_protocol_version should round-trip"
    );

    // HttpVersion must be present
    assert_eq!(
        config.http_version(),
        Some(&aws_sdk_cloudfront::types::HttpVersion::Http2),
        "http_version should round-trip"
    );
}

// Covers FIX(terraform-e2e): GetDistributionConfig also echoes back the full
// DistributionConfig (same underlying wire conversion).
#[tokio::test]
async fn test_full_distribution_config_round_trips_via_get_distribution_config() {
    let client = make_cloudfront_client().await;

    let create_resp = client
        .create_distribution()
        .distribution_config(make_full_distribution_config("ref-full-config-cfg"))
        .send()
        .await
        .expect("create_distribution should succeed");
    let dist_id = create_resp.distribution().unwrap().id().to_string();

    let cfg_resp = client
        .get_distribution_config()
        .id(&dist_id)
        .send()
        .await
        .expect("get_distribution_config should succeed");

    let config = cfg_resp.distribution_config().unwrap();

    // Restrictions
    let restrictions = config
        .restrictions()
        .expect("restrictions should be present");
    let geo = restrictions
        .geo_restriction()
        .expect("geo_restriction should be present");
    assert_eq!(
        geo.restriction_type(),
        &aws_sdk_cloudfront::types::GeoRestrictionType::Whitelist
    );
    assert_eq!(geo.items(), &["US"]);

    // ViewerCertificate
    let vc = config
        .viewer_certificate()
        .expect("viewer_certificate should be present");
    assert_eq!(vc.cloud_front_default_certificate(), Some(true));

    // HttpVersion
    assert_eq!(
        config.http_version(),
        Some(&aws_sdk_cloudfront::types::HttpVersion::Http2),
    );
}

// Covers FIX(terraform-e2e): UpdateDistribution also parses the full
// DistributionConfig and echoes it back.
#[tokio::test]
async fn test_update_distribution_preserves_full_config() {
    let client = make_cloudfront_client().await;

    // Create with a minimal config first
    let create_resp = client
        .create_distribution()
        .distribution_config(make_distribution_config("ref-update-full"))
        .send()
        .await
        .unwrap();
    let dist_id = create_resp.distribution().unwrap().id().to_string();
    let etag = create_resp.e_tag().unwrap().to_string();

    // Update with the full config (adding Restrictions, ViewerCertificate, etc.)
    let update_resp = client
        .update_distribution()
        .id(&dist_id)
        .if_match(&etag)
        .distribution_config(make_full_distribution_config("ref-update-full"))
        .send()
        .await
        .expect("update_distribution should succeed");

    let dist = update_resp.distribution().unwrap();
    let config = dist.distribution_config().unwrap();

    // Restrictions must survive the update
    let restrictions = config
        .restrictions()
        .expect("restrictions should survive update");
    let geo = restrictions
        .geo_restriction()
        .expect("geo_restriction should survive update");
    assert_eq!(
        geo.restriction_type(),
        &aws_sdk_cloudfront::types::GeoRestrictionType::Whitelist
    );

    // ViewerCertificate must survive the update
    let vc = config
        .viewer_certificate()
        .expect("viewer_certificate should survive update");
    assert_eq!(vc.cloud_front_default_certificate(), Some(true));
}

// Covers FIX(terraform-e2e): include ActiveTrustedSigners and
// ActiveTrustedKeyGroups with Enabled=false/Quantity=0 in GetDistribution
// responses so the Go provider does not dereference nil pointers.
#[tokio::test]
async fn test_get_distribution_includes_active_trusted_signers_and_key_groups() {
    let client = make_cloudfront_client().await;

    let create_resp = client
        .create_distribution()
        .distribution_config(make_distribution_config("ref-trusted-fields"))
        .send()
        .await
        .unwrap();
    let dist_id = create_resp.distribution().unwrap().id().to_string();

    let get_resp = client
        .get_distribution()
        .id(&dist_id)
        .send()
        .await
        .expect("get_distribution should succeed");

    let dist = get_resp.distribution().unwrap();

    // ActiveTrustedSigners must be present (Enabled=false, Quantity=0)
    let ats = dist
        .active_trusted_signers()
        .expect("active_trusted_signers should be present");
    assert!(
        !ats.enabled(),
        "active_trusted_signers.enabled should be false"
    );
    assert_eq!(
        ats.quantity(),
        0,
        "active_trusted_signers.quantity should be 0"
    );

    // ActiveTrustedKeyGroups must be present (Enabled=false, Quantity=0)
    let atkg = dist
        .active_trusted_key_groups()
        .expect("active_trusted_key_groups should be present");
    assert!(
        !atkg.enabled(),
        "active_trusted_key_groups.enabled should be false"
    );
    assert_eq!(
        atkg.quantity(),
        0,
        "active_trusted_key_groups.quantity should be 0"
    );
}

// Covers FIX(terraform-e2e): always include OriginGroups with Quantity=0 in
// the response when the request did not specify OriginGroups.  The Go provider
// dereferences OriginGroups.Quantity without a nil guard.
#[tokio::test]
async fn test_get_distribution_always_includes_origin_groups() {
    let client = make_cloudfront_client().await;

    // Create a distribution WITHOUT specifying OriginGroups
    let create_resp = client
        .create_distribution()
        .distribution_config(make_distribution_config("ref-origin-groups"))
        .send()
        .await
        .unwrap();
    let dist_id = create_resp.distribution().unwrap().id().to_string();

    let get_resp = client
        .get_distribution()
        .id(&dist_id)
        .send()
        .await
        .expect("get_distribution should succeed");

    let config = get_resp
        .distribution()
        .unwrap()
        .distribution_config()
        .unwrap();

    // OriginGroups must be present even though we did not send it
    let og = config
        .origin_groups()
        .expect("origin_groups should always be present");
    assert_eq!(
        og.quantity(),
        0,
        "origin_groups.quantity should be 0 when none were sent"
    );
}

// Covers FIX(terraform-e2e): OriginGroups with Quantity=0 is also present in
// GetDistributionConfig responses.
#[tokio::test]
async fn test_get_distribution_config_always_includes_origin_groups() {
    let client = make_cloudfront_client().await;

    let create_resp = client
        .create_distribution()
        .distribution_config(make_distribution_config("ref-origin-groups-cfg"))
        .send()
        .await
        .unwrap();
    let dist_id = create_resp.distribution().unwrap().id().to_string();

    let cfg_resp = client
        .get_distribution_config()
        .id(&dist_id)
        .send()
        .await
        .expect("get_distribution_config should succeed");

    let config = cfg_resp.distribution_config().unwrap();
    let og = config
        .origin_groups()
        .expect("origin_groups should always be present in config response");
    assert_eq!(og.quantity(), 0);
}
