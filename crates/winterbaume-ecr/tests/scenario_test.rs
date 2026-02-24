//! Smoke tests for winterbaume ECR service — realistic application scenarios.
//!
//! Each test simulates a coherent end-to-end workflow rather than exercising
//! a single API call in isolation.

use aws_sdk_ecr::config::BehaviorVersion;
use aws_sdk_ecr::types::ImageIdentifier;
use winterbaume_core::MockAws;
use winterbaume_ecr::EcrService;

async fn make_ecr_client() -> aws_sdk_ecr::Client {
    let mock = MockAws::builder().with_service(EcrService::new()).build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_ecr::config::Region::new("us-east-1"))
        .load()
        .await;
    aws_sdk_ecr::Client::new(&config)
}

/// Build a simple deterministic Docker manifest. Different `seed` values
/// produce manifests with different config digests so we get distinct
/// image digests when we want them.
fn make_manifest(seed: u64) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut layers = Vec::new();
    let mut total = 0u64;
    for i in 0..3 {
        let mut h = DefaultHasher::new();
        (seed, i, "layer").hash(&mut h);
        let digest = format!("sha256:{:064x}", h.finish());
        let size = 100 + (seed.wrapping_add(i) % 900);
        total += size;
        layers.push(serde_json::json!({
            "mediaType": "application/vnd.docker.image.rootfs.diff.tar.gzip",
            "size": size,
            "digest": digest,
        }));
    }

    let layer_digests: String = layers
        .iter()
        .map(|l| l["digest"].as_str().unwrap().to_string())
        .collect();
    let mut h = DefaultHasher::new();
    (seed, layer_digests).hash(&mut h);
    let config_digest = format!("sha256:{:064x}", h.finish());

    serde_json::json!({
        "schemaVersion": 2,
        "mediaType": "application/vnd.docker.distribution.manifest.v2+json",
        "config": {
            "mediaType": "application/vnd.docker.container.image.v1+json",
            "size": total,
            "digest": config_digest,
        },
        "layers": layers,
    })
    .to_string()
}

/// Scenario: full repository lifecycle for a CI build pipeline.
///
/// A CI pipeline:
/// 1. Creates a repository.
/// 2. Attaches a lifecycle policy that expires untagged images.
/// 3. Attaches a repository policy granting cross-account pull.
/// 4. Pushes two image tags (v1, latest).
/// 5. Lists images and verifies both tags appear.
/// 6. BatchGetImage by tag returns the manifest.
/// 7. Tears the repository down.
#[tokio::test]
async fn test_repository_full_lifecycle() {
    let client = make_ecr_client().await;
    let repo = "ci-repo";

    // 1. Create repository.
    let create = client
        .create_repository()
        .repository_name(repo)
        .send()
        .await
        .expect("create_repository");
    assert_eq!(create.repository().unwrap().repository_name(), Some(repo));

    // 2. Lifecycle policy.
    let lifecycle = r#"{"rules":[{"rulePriority":1,"selection":{"tagStatus":"untagged","countType":"sinceImagePushed","countUnit":"days","countNumber":7},"action":{"type":"expire"}}]}"#;
    let lc = client
        .put_lifecycle_policy()
        .repository_name(repo)
        .lifecycle_policy_text(lifecycle)
        .send()
        .await
        .expect("put_lifecycle_policy");
    assert_eq!(lc.lifecycle_policy_text(), Some(lifecycle));

    // 3. Repository policy.
    let policy = r#"{"Version":"2012-10-17","Statement":[{"Sid":"AllowPull","Effect":"Allow","Principal":"*","Action":["ecr:BatchGetImage"]}]}"#;
    let rp = client
        .set_repository_policy()
        .repository_name(repo)
        .policy_text(policy)
        .send()
        .await
        .expect("set_repository_policy");
    assert_eq!(rp.policy_text(), Some(policy));

    // 4. Push two tags. Use the same manifest so both tags share a digest
    // — that's the realistic CI pattern (tag :latest after :v1 succeeds).
    let manifest = make_manifest(1);
    let v1 = client
        .put_image()
        .repository_name(repo)
        .image_manifest(&manifest)
        .image_tag("v1")
        .send()
        .await
        .expect("put_image v1");
    let v1_digest = v1
        .image()
        .unwrap()
        .image_id()
        .unwrap()
        .image_digest()
        .unwrap()
        .to_string();
    assert!(v1_digest.starts_with("sha256:"));

    client
        .put_image()
        .repository_name(repo)
        .image_manifest(&manifest)
        .image_tag("latest")
        .send()
        .await
        .expect("put_image latest");

    // 5. ListImages should report both tags.
    let list = client
        .list_images()
        .repository_name(repo)
        .send()
        .await
        .expect("list_images");
    let tags: Vec<&str> = list
        .image_ids()
        .iter()
        .filter_map(|i| i.image_tag())
        .collect();
    assert!(tags.contains(&"v1"), "expected v1 in {tags:?}");
    assert!(tags.contains(&"latest"), "expected latest in {tags:?}");

    // 6. BatchGetImage by tag returns the manifest.
    let bg = client
        .batch_get_image()
        .repository_name(repo)
        .image_ids(ImageIdentifier::builder().image_tag("v1").build())
        .send()
        .await
        .expect("batch_get_image");
    assert_eq!(bg.images().len(), 1);
    assert_eq!(bg.images()[0].image_id().unwrap().image_tag(), Some("v1"));
    assert!(
        bg.images()[0]
            .image_manifest()
            .unwrap()
            .contains("schemaVersion")
    );

    // 7. Tear down. Use force because the repo has images.
    client
        .delete_repository()
        .repository_name(repo)
        .force(true)
        .send()
        .await
        .expect("delete_repository");

    // Repository should be gone — list_images should now error.
    let after = client.list_images().repository_name(repo).send().await;
    assert!(after.is_err(), "list_images on deleted repo must fail");
}

/// Scenario: pull-through cache rule round-trip.
///
/// An admin sets up an ECR pull-through cache rule pointing at the
/// public ECR registry, verifies it appears in DescribePullThroughCacheRules,
/// then deletes it and confirms it is gone.
#[tokio::test]
async fn test_pull_through_cache_rule_round_trip() {
    let client = make_ecr_client().await;

    let create = client
        .create_pull_through_cache_rule()
        .ecr_repository_prefix("public-mirror")
        .upstream_registry_url("public.ecr.aws")
        .send()
        .await
        .expect("create_pull_through_cache_rule");
    assert_eq!(create.ecr_repository_prefix(), Some("public-mirror"));
    assert_eq!(create.upstream_registry_url(), Some("public.ecr.aws"));

    let desc = client
        .describe_pull_through_cache_rules()
        .ecr_repository_prefixes("public-mirror")
        .send()
        .await
        .expect("describe_pull_through_cache_rules");
    let rules = desc.pull_through_cache_rules();
    assert_eq!(rules.len(), 1);
    assert_eq!(rules[0].ecr_repository_prefix(), Some("public-mirror"));

    let del = client
        .delete_pull_through_cache_rule()
        .ecr_repository_prefix("public-mirror")
        .send()
        .await
        .expect("delete_pull_through_cache_rule");
    assert_eq!(del.ecr_repository_prefix(), Some("public-mirror"));

    let desc_after = client
        .describe_pull_through_cache_rules()
        .send()
        .await
        .expect("describe_pull_through_cache_rules after delete");
    assert!(
        desc_after
            .pull_through_cache_rules()
            .iter()
            .all(|r| r.ecr_repository_prefix() != Some("public-mirror")),
        "deleted rule must not appear"
    );
}
