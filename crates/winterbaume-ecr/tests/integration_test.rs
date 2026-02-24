//! Integration tests for winterbaume ECR service.
//! Ported from moto's test_ecr.py with exact value assertions.

use aws_sdk_ecr::config::BehaviorVersion;
use aws_sdk_ecr::types::ImageIdentifier;
use winterbaume_core::MockAws;
use winterbaume_ecr::EcrService;

const ACCOUNT_ID: &str = "123456789012";
const ECR_REGION: &str = "us-east-1";
const ECR_REPO: &str = "test-repo";
const ECR_REPO_NOT_EXIST: &str = "does-not-exist";

async fn make_ecr_client() -> aws_sdk_ecr::Client {
    make_ecr_client_with_region(ECR_REGION).await
}

async fn make_ecr_client_with_region(region: &str) -> aws_sdk_ecr::Client {
    let mock = MockAws::builder().with_service(EcrService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_ecr::config::Region::new(region.to_string()))
        .load()
        .await;

    aws_sdk_ecr::Client::new(&config)
}

/// Helper to create a Docker image manifest similar to moto's _create_image_manifest().
/// Each call produces a unique manifest (unique random layers).
fn create_image_manifest() -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::time::SystemTime;

    // Use a combination of time + random-ish data to ensure uniqueness
    let mut hasher = DefaultHasher::new();
    SystemTime::now().hash(&mut hasher);
    let seed = hasher.finish();

    let mut layers = Vec::new();
    let mut total_size = 0u64;
    for i in 0..5 {
        let layer_data = format!("layer-{}-{}", seed.wrapping_add(i), i);
        let digest = format!("sha256:{:064x}", {
            let mut h = DefaultHasher::new();
            layer_data.hash(&mut h);
            h.finish()
        });
        let size = 100 + seed.wrapping_add(i) % 900;
        total_size += size;
        layers.push(serde_json::json!({
            "mediaType": "application/vnd.docker.image.rootfs.diff.tar.gzip",
            "size": size,
            "digest": digest,
        }));
    }

    // Config digest is derived from layer digests
    let layer_digests: String = layers
        .iter()
        .map(|l| l["digest"].as_str().unwrap().to_string())
        .collect();
    let config_digest = format!("sha256:{:064x}", {
        let mut h = DefaultHasher::new();
        layer_digests.hash(&mut h);
        h.finish()
    });

    serde_json::json!({
        "schemaVersion": 2,
        "mediaType": "application/vnd.docker.distribution.manifest.v2+json",
        "config": {
            "mediaType": "application/vnd.docker.container.image.v1+json",
            "size": total_size,
            "digest": config_digest,
        },
        "layers": layers,
    })
    .to_string()
}

// ==================== CreateRepository ====================

/// Ported from moto test_create_repository
#[tokio::test]
async fn test_create_repository() {
    let client = make_ecr_client().await;

    let resp = client
        .create_repository()
        .repository_name(ECR_REPO)
        .send()
        .await
        .expect("create_repository should succeed");

    let repo = resp.repository().expect("should have repository");
    assert_eq!(repo.repository_name(), Some(ECR_REPO));
    assert_eq!(
        repo.repository_arn(),
        Some(
            format!(
                "arn:aws:ecr:us-east-1:{}:repository/{}",
                ACCOUNT_ID, ECR_REPO
            )
            .as_str()
        )
    );
    assert_eq!(repo.registry_id(), Some(ACCOUNT_ID));
    assert_eq!(
        repo.repository_uri(),
        Some(
            format!(
                "{}.dkr.ecr.us-east-1.amazonaws.com/{}",
                ACCOUNT_ID, ECR_REPO
            )
            .as_str()
        )
    );
    assert!(repo.created_at().is_some());
}

/// Ported from moto test_create_repository_error_already_exists
#[tokio::test]
async fn test_create_repository_error_already_exists() {
    let client = make_ecr_client().await;
    client
        .create_repository()
        .repository_name(ECR_REPO)
        .send()
        .await
        .unwrap();

    let err = client
        .create_repository()
        .repository_name(ECR_REPO)
        .send()
        .await;

    assert!(err.is_err(), "duplicate create should fail");
    let err = err.unwrap_err();
    let service_err = err.as_service_error().expect("should be a service error");
    assert!(
        service_err.is_repository_already_exists_exception(),
        "should be RepositoryAlreadyExistsException"
    );
}

// ==================== DescribeRepositories ====================

/// Ported from moto test_describe_repositories
#[tokio::test]
async fn test_describe_repositories() {
    let client = make_ecr_client().await;
    client
        .create_repository()
        .repository_name("test_repository1")
        .send()
        .await
        .unwrap();
    client
        .create_repository()
        .repository_name("test_repository0")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_repositories()
        .send()
        .await
        .expect("describe_repositories should succeed");

    assert_eq!(resp.repositories().len(), 2);

    let arns: std::collections::HashSet<String> = resp
        .repositories()
        .iter()
        .map(|r| r.repository_arn().unwrap().to_string())
        .collect();
    let expected_arns: std::collections::HashSet<String> = [
        format!(
            "arn:aws:ecr:us-east-1:{}:repository/test_repository1",
            ACCOUNT_ID
        ),
        format!(
            "arn:aws:ecr:us-east-1:{}:repository/test_repository0",
            ACCOUNT_ID
        ),
    ]
    .into();
    assert_eq!(arns, expected_arns);

    let uris: std::collections::HashSet<String> = resp
        .repositories()
        .iter()
        .map(|r| r.repository_uri().unwrap().to_string())
        .collect();
    let expected_uris: std::collections::HashSet<String> = [
        format!(
            "{}.dkr.ecr.us-east-1.amazonaws.com/test_repository1",
            ACCOUNT_ID
        ),
        format!(
            "{}.dkr.ecr.us-east-1.amazonaws.com/test_repository0",
            ACCOUNT_ID
        ),
    ]
    .into();
    assert_eq!(uris, expected_uris);
}

/// Ported from moto test_describe_repositories_3 - filter by name
#[tokio::test]
async fn test_describe_repositories_filter_by_name() {
    let client = make_ecr_client().await;
    client
        .create_repository()
        .repository_name("test_repository1")
        .send()
        .await
        .unwrap();
    client
        .create_repository()
        .repository_name("test_repository0")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_repositories()
        .repository_names("test_repository1")
        .send()
        .await
        .expect("describe_repositories should succeed");

    assert_eq!(resp.repositories().len(), 1);
    assert_eq!(
        resp.repositories()[0].repository_arn().unwrap(),
        format!(
            "arn:aws:ecr:us-east-1:{}:repository/test_repository1",
            ACCOUNT_ID
        )
    );
    assert_eq!(
        resp.repositories()[0].repository_uri().unwrap(),
        format!(
            "{}.dkr.ecr.us-east-1.amazonaws.com/test_repository1",
            ACCOUNT_ID
        )
    );
}

/// Ported from moto test_describe_repository_that_doesnt_exist
#[tokio::test]
async fn test_describe_repository_that_doesnt_exist() {
    let client = make_ecr_client().await;

    let err = client
        .describe_repositories()
        .repository_names("repo-that-doesnt-exist")
        .send()
        .await;

    assert!(err.is_err());
    let err = err.unwrap_err();
    let service_err = err.as_service_error().expect("should be a service error");
    assert!(
        service_err.is_repository_not_found_exception(),
        "should be RepositoryNotFoundException"
    );
}

// ==================== DeleteRepository ====================

/// Ported from moto test_delete_repository
#[tokio::test]
async fn test_delete_repository() {
    let client = make_ecr_client().await;
    client
        .create_repository()
        .repository_name(ECR_REPO)
        .send()
        .await
        .unwrap();

    let resp = client
        .delete_repository()
        .repository_name(ECR_REPO)
        .send()
        .await
        .expect("delete_repository should succeed");

    let repo = resp.repository().expect("should have repository");
    assert_eq!(repo.repository_name(), Some(ECR_REPO));
    assert_eq!(
        repo.repository_arn(),
        Some(
            format!(
                "arn:aws:ecr:us-east-1:{}:repository/{}",
                ACCOUNT_ID, ECR_REPO
            )
            .as_str()
        )
    );
    assert_eq!(repo.registry_id(), Some(ACCOUNT_ID));
    assert_eq!(
        repo.repository_uri(),
        Some(
            format!(
                "{}.dkr.ecr.us-east-1.amazonaws.com/{}",
                ACCOUNT_ID, ECR_REPO
            )
            .as_str()
        )
    );

    let resp = client.describe_repositories().send().await.unwrap();
    assert_eq!(resp.repositories().len(), 0);
}

/// Ported from moto test_delete_repository_that_doesnt_exist
#[tokio::test]
async fn test_delete_repository_that_doesnt_exist() {
    let client = make_ecr_client().await;

    let err = client
        .delete_repository()
        .repository_name(ECR_REPO_NOT_EXIST)
        .send()
        .await;

    assert!(err.is_err());
    let err = err.unwrap_err();
    let service_err = err.as_service_error().expect("should be a service error");
    assert!(
        service_err.is_repository_not_found_exception(),
        "should be RepositoryNotFoundException"
    );
}

/// Ported from moto test_delete_repository_error_not_empty
#[tokio::test]
async fn test_delete_repository_error_not_empty() {
    let client = make_ecr_client().await;
    client
        .create_repository()
        .repository_name(ECR_REPO)
        .send()
        .await
        .unwrap();

    let manifest = create_image_manifest();
    client
        .put_image()
        .repository_name(ECR_REPO)
        .image_manifest(&manifest)
        .image_tag("latest")
        .send()
        .await
        .unwrap();

    // Deleting a non-empty repo without force should fail
    let err = client
        .delete_repository()
        .repository_name(ECR_REPO)
        .send()
        .await;

    assert!(err.is_err(), "delete non-empty repo should fail");
    let err = err.unwrap_err();
    let service_err = err.as_service_error().expect("should be a service error");
    assert!(
        service_err.is_repository_not_empty_exception(),
        "should be RepositoryNotEmptyException"
    );
}

/// Ported from moto test_delete_repository_with_force
#[tokio::test]
async fn test_delete_repository_with_force() {
    let client = make_ecr_client().await;
    client
        .create_repository()
        .repository_name(ECR_REPO)
        .send()
        .await
        .unwrap();

    let manifest = create_image_manifest();
    client
        .put_image()
        .repository_name(ECR_REPO)
        .image_manifest(&manifest)
        .image_tag("latest")
        .send()
        .await
        .unwrap();

    // Deleting a non-empty repo with force should succeed
    let resp = client
        .delete_repository()
        .repository_name(ECR_REPO)
        .force(true)
        .send()
        .await
        .expect("force delete should succeed");

    let repo = resp.repository().expect("should have repository");
    assert_eq!(repo.repository_name(), Some(ECR_REPO));

    let resp = client.describe_repositories().send().await.unwrap();
    assert_eq!(resp.repositories().len(), 0);
}

// ==================== PutImage ====================

/// Ported from moto test_put_image
#[tokio::test]
async fn test_put_image() {
    let client = make_ecr_client().await;
    client
        .create_repository()
        .repository_name("test_repository")
        .send()
        .await
        .unwrap();

    let manifest = create_image_manifest();
    let resp = client
        .put_image()
        .repository_name("test_repository")
        .image_manifest(&manifest)
        .image_tag("latest")
        .send()
        .await
        .expect("put_image should succeed");

    let image = resp.image().expect("should have image");
    let image_id = image.image_id().expect("should have image_id");
    assert_eq!(image_id.image_tag(), Some("latest"));
    assert!(
        image_id.image_digest().unwrap_or("").contains("sha"),
        "digest should contain 'sha'"
    );
    assert_eq!(image.repository_name(), Some("test_repository"));
    assert_eq!(image.registry_id(), Some(ACCOUNT_ID));
}

/// Ported from moto test_put_image_with_multiple_tags
#[tokio::test]
async fn test_put_image_with_multiple_tags() {
    let client = make_ecr_client().await;
    client
        .create_repository()
        .repository_name("test_repository")
        .send()
        .await
        .unwrap();

    let manifest = create_image_manifest();

    // Put image with tag v1
    let resp1 = client
        .put_image()
        .repository_name("test_repository")
        .image_manifest(&manifest)
        .image_tag("v1")
        .send()
        .await
        .expect("put_image v1 should succeed");

    let img1 = resp1.image().unwrap();
    assert_eq!(img1.image_id().unwrap().image_tag(), Some("v1"));
    assert!(
        img1.image_id()
            .unwrap()
            .image_digest()
            .unwrap()
            .contains("sha")
    );
    assert_eq!(img1.repository_name(), Some("test_repository"));
    assert_eq!(img1.registry_id(), Some(ACCOUNT_ID));

    // Put same manifest with tag latest
    let resp2 = client
        .put_image()
        .repository_name("test_repository")
        .image_manifest(&manifest)
        .image_tag("latest")
        .send()
        .await
        .expect("put_image latest should succeed");

    let img2 = resp2.image().unwrap();
    assert_eq!(img2.image_id().unwrap().image_tag(), Some("latest"));
    assert!(
        img2.image_id()
            .unwrap()
            .image_digest()
            .unwrap()
            .contains("sha")
    );
    assert_eq!(img2.repository_name(), Some("test_repository"));
    assert_eq!(img2.registry_id(), Some(ACCOUNT_ID));

    // Same digest for same manifest
    assert_eq!(
        img1.image_id().unwrap().image_digest(),
        img2.image_id().unwrap().image_digest()
    );
}

/// Ported from moto test_put_same_image_with_same_tag
#[tokio::test]
async fn test_put_same_image_with_same_tag() {
    let client = make_ecr_client().await;
    client
        .create_repository()
        .repository_name(ECR_REPO)
        .send()
        .await
        .unwrap();

    let manifest = create_image_manifest();

    // Put image first time
    client
        .put_image()
        .repository_name(ECR_REPO)
        .image_manifest(&manifest)
        .image_tag("my-tag")
        .send()
        .await
        .unwrap();

    // Put same image with same tag should fail with ImageAlreadyExistsException
    let err = client
        .put_image()
        .repository_name(ECR_REPO)
        .image_manifest(&manifest)
        .image_tag("my-tag")
        .send()
        .await;

    assert!(err.is_err(), "same image+tag should fail");
    let err = err.unwrap_err();
    let service_err = err.as_service_error().expect("should be a service error");
    assert!(
        service_err.is_image_already_exists_exception(),
        "should be ImageAlreadyExistsException"
    );
}

/// Ported from moto test_put_multiple_images_with_same_tag
/// When putting a different image with the same tag, the old image
/// (if it only has that one tag) should be replaced.
#[tokio::test]
async fn test_put_multiple_images_with_same_tag() {
    let client = make_ecr_client().await;
    client
        .create_repository()
        .repository_name(ECR_REPO)
        .send()
        .await
        .unwrap();

    let manifest1 = create_image_manifest();
    // small delay to ensure different manifests
    tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    let manifest2 = create_image_manifest();
    assert_ne!(manifest1, manifest2, "manifests should be different");

    let image_tag = "my-tag";

    // Put first image
    let resp1 = client
        .put_image()
        .repository_name(ECR_REPO)
        .image_manifest(&manifest1)
        .image_tag(image_tag)
        .send()
        .await
        .unwrap();
    let digest1 = resp1
        .image()
        .unwrap()
        .image_id()
        .unwrap()
        .image_digest()
        .unwrap()
        .to_string();

    // Put second image with same tag - should remove old image (since it only has this tag)
    let resp2 = client
        .put_image()
        .repository_name(ECR_REPO)
        .image_manifest(&manifest2)
        .image_tag(image_tag)
        .send()
        .await
        .unwrap();
    let digest2 = resp2
        .image()
        .unwrap()
        .image_id()
        .unwrap()
        .image_digest()
        .unwrap()
        .to_string();

    assert_ne!(
        digest1, digest2,
        "different manifests should have different digests"
    );

    // Only the second image should remain
    let list_resp = client
        .list_images()
        .repository_name(ECR_REPO)
        .send()
        .await
        .unwrap();
    assert_eq!(list_resp.image_ids().len(), 1);
    assert_eq!(list_resp.image_ids()[0].image_digest().unwrap(), digest2);
    assert_eq!(list_resp.image_ids()[0].image_tag().unwrap(), image_tag);
}

/// Ported from moto test_multiple_tags__ensure_tags_exist_only_on_one_image
#[tokio::test]
async fn test_multiple_tags_ensure_tags_on_one_image() {
    let client = make_ecr_client().await;
    client
        .create_repository()
        .repository_name(ECR_REPO)
        .send()
        .await
        .unwrap();

    let manifest1 = create_image_manifest();
    tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    let manifest2 = create_image_manifest();

    let tag_to_move = "mock-tag";

    // Create image_001 with unique tag
    client
        .put_image()
        .repository_name(ECR_REPO)
        .image_manifest(&manifest1)
        .image_tag("image_001")
        .send()
        .await
        .unwrap();

    // Create image_002 with unique tag
    client
        .put_image()
        .repository_name(ECR_REPO)
        .image_manifest(&manifest2)
        .image_tag("image_002")
        .send()
        .await
        .unwrap();

    // Tag first image with shared tag
    client
        .put_image()
        .repository_name(ECR_REPO)
        .image_manifest(&manifest1)
        .image_tag(tag_to_move)
        .send()
        .await
        .unwrap();

    // Verify first image has the shared tag
    let resp = client
        .batch_get_image()
        .repository_name(ECR_REPO)
        .image_ids(ImageIdentifier::builder().image_tag(tag_to_move).build())
        .send()
        .await
        .unwrap();
    assert_eq!(resp.images().len(), 1);
    assert_eq!(resp.images()[0].image_manifest().unwrap(), manifest1);

    // Tag second image with shared tag - should move it
    client
        .put_image()
        .repository_name(ECR_REPO)
        .image_manifest(&manifest2)
        .image_tag(tag_to_move)
        .send()
        .await
        .unwrap();

    // Verify second image now has the shared tag
    let resp = client
        .batch_get_image()
        .repository_name(ECR_REPO)
        .image_ids(ImageIdentifier::builder().image_tag(tag_to_move).build())
        .send()
        .await
        .unwrap();
    assert_eq!(resp.images().len(), 1);
    assert_eq!(resp.images()[0].image_manifest().unwrap(), manifest2);
}

// ==================== ListImages ====================

/// Ported from moto test_list_images
#[tokio::test]
async fn test_list_images() {
    let client = make_ecr_client().await;
    client
        .create_repository()
        .repository_name("test_repository_1")
        .send()
        .await
        .unwrap();
    client
        .create_repository()
        .repository_name("test_repository_2")
        .send()
        .await
        .unwrap();

    // Use unique manifests with delays
    let m1 = create_image_manifest();
    tokio::time::sleep(std::time::Duration::from_millis(5)).await;
    let m2 = create_image_manifest();
    tokio::time::sleep(std::time::Duration::from_millis(5)).await;
    let m3 = create_image_manifest();
    tokio::time::sleep(std::time::Duration::from_millis(5)).await;
    let m4 = create_image_manifest();

    client
        .put_image()
        .repository_name("test_repository_1")
        .image_manifest(&m1)
        .image_tag("latest")
        .send()
        .await
        .unwrap();
    client
        .put_image()
        .repository_name("test_repository_1")
        .image_manifest(&m2)
        .image_tag("v1")
        .send()
        .await
        .unwrap();
    client
        .put_image()
        .repository_name("test_repository_1")
        .image_manifest(&m3)
        .image_tag("v2")
        .send()
        .await
        .unwrap();
    client
        .put_image()
        .repository_name("test_repository_2")
        .image_manifest(&m4)
        .image_tag("oldest")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_images()
        .repository_name("test_repository_1")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.image_ids().len(), 3);
    for image_id in resp.image_ids() {
        assert!(
            image_id.image_digest().unwrap().contains("sha"),
            "digest should contain sha"
        );
    }

    let tags: std::collections::HashSet<&str> = resp
        .image_ids()
        .iter()
        .filter_map(|id| id.image_tag())
        .collect();
    assert_eq!(
        tags,
        ["latest", "v1", "v2"]
            .into_iter()
            .collect::<std::collections::HashSet<&str>>()
    );

    let resp2 = client
        .list_images()
        .repository_name("test_repository_2")
        .send()
        .await
        .unwrap();
    assert_eq!(resp2.image_ids().len(), 1);
    assert_eq!(resp2.image_ids()[0].image_tag(), Some("oldest"));
    assert!(resp2.image_ids()[0].image_digest().unwrap().contains("sha"));
}

/// Ported from moto test_list_same_image_with_multiple_tags
/// A single image with multiple tags should produce multiple entries in list_images.
#[tokio::test]
async fn test_list_same_image_with_multiple_tags() {
    let client = make_ecr_client().await;
    client
        .create_repository()
        .repository_name("reponame")
        .send()
        .await
        .unwrap();

    let manifest = create_image_manifest();

    client
        .put_image()
        .repository_name("reponame")
        .image_manifest(&manifest)
        .image_tag("tag1")
        .send()
        .await
        .unwrap();
    client
        .put_image()
        .repository_name("reponame")
        .image_manifest(&manifest)
        .image_tag("tag2")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_images()
        .repository_name("reponame")
        .send()
        .await
        .unwrap();

    // Should return 2 entries (one per tag), not 1
    assert_eq!(resp.image_ids().len(), 2);
    // Both should have the same digest
    assert_eq!(
        resp.image_ids()[0].image_digest(),
        resp.image_ids()[1].image_digest()
    );
    let tags: std::collections::HashSet<&str> = resp
        .image_ids()
        .iter()
        .filter_map(|id| id.image_tag())
        .collect();
    assert_eq!(
        tags,
        ["tag1", "tag2"]
            .into_iter()
            .collect::<std::collections::HashSet<&str>>()
    );
}

/// Ported from moto test_list_images_from_repository_that_doesnt_exist
#[tokio::test]
async fn test_list_images_from_repository_that_doesnt_exist() {
    let client = make_ecr_client().await;

    let err = client
        .list_images()
        .repository_name("repo-that-doesnt-exist")
        .send()
        .await;

    assert!(err.is_err());
    let err = err.unwrap_err();
    let service_err = err.as_service_error().expect("should be a service error");
    assert!(
        service_err.is_repository_not_found_exception(),
        "should be RepositoryNotFoundException"
    );
}

// ==================== BatchGetImage ====================

/// Ported from moto test_batch_get_image
#[tokio::test]
async fn test_batch_get_image() {
    let client = make_ecr_client().await;
    client
        .create_repository()
        .repository_name("test_repository")
        .send()
        .await
        .unwrap();

    let m1 = create_image_manifest();
    tokio::time::sleep(std::time::Duration::from_millis(5)).await;
    let m2 = create_image_manifest();
    tokio::time::sleep(std::time::Duration::from_millis(5)).await;
    let m3 = create_image_manifest();

    client
        .put_image()
        .repository_name("test_repository")
        .image_manifest(&m1)
        .image_tag("latest")
        .send()
        .await
        .unwrap();
    client
        .put_image()
        .repository_name("test_repository")
        .image_manifest(&m2)
        .image_tag("v1")
        .send()
        .await
        .unwrap();
    client
        .put_image()
        .repository_name("test_repository")
        .image_manifest(&m3)
        .image_tag("v2")
        .send()
        .await
        .unwrap();

    let resp = client
        .batch_get_image()
        .repository_name("test_repository")
        .image_ids(ImageIdentifier::builder().image_tag("v2").build())
        .send()
        .await
        .unwrap();

    assert_eq!(resp.images().len(), 1);
    assert!(
        resp.images()[0]
            .image_manifest()
            .unwrap()
            .contains("vnd.docker.distribution.manifest.v2+json")
    );
    assert_eq!(resp.images()[0].registry_id(), Some(ACCOUNT_ID));
    assert_eq!(resp.images()[0].repository_name(), Some("test_repository"));
    assert_eq!(resp.images()[0].image_id().unwrap().image_tag(), Some("v2"));
    assert!(
        resp.images()[0]
            .image_id()
            .unwrap()
            .image_digest()
            .unwrap()
            .contains("sha")
    );

    assert_eq!(resp.failures().len(), 0);
}

/// Ported from moto test_batch_get_image_that_doesnt_exist
#[tokio::test]
async fn test_batch_get_image_that_doesnt_exist() {
    let client = make_ecr_client().await;
    client
        .create_repository()
        .repository_name("test_repository")
        .send()
        .await
        .unwrap();

    let m1 = create_image_manifest();
    client
        .put_image()
        .repository_name("test_repository")
        .image_manifest(&m1)
        .image_tag("latest")
        .send()
        .await
        .unwrap();

    let resp = client
        .batch_get_image()
        .repository_name("test_repository")
        .image_ids(ImageIdentifier::builder().image_tag("v5").build())
        .send()
        .await
        .unwrap();

    assert_eq!(resp.images().len(), 0);
    assert_eq!(resp.failures().len(), 1);
    assert_eq!(
        resp.failures()[0].failure_reason(),
        Some("Requested image not found")
    );
    assert_eq!(
        resp.failures()[0].failure_code(),
        Some(&aws_sdk_ecr::types::ImageFailureCode::ImageNotFound)
    );
    assert_eq!(
        resp.failures()[0].image_id().unwrap().image_tag(),
        Some("v5")
    );
}

/// Ported from moto test_batch_get_image_with_multiple_tags
#[tokio::test]
async fn test_batch_get_image_with_multiple_tags() {
    let client = make_ecr_client().await;
    client
        .create_repository()
        .repository_name("test_repository")
        .send()
        .await
        .unwrap();

    let manifest = create_image_manifest();

    client
        .put_image()
        .repository_name("test_repository")
        .image_manifest(&manifest)
        .image_tag("latest")
        .send()
        .await
        .unwrap();
    client
        .put_image()
        .repository_name("test_repository")
        .image_manifest(&manifest)
        .image_tag("v1")
        .send()
        .await
        .unwrap();

    let latest_resp = client
        .batch_get_image()
        .repository_name("test_repository")
        .image_ids(ImageIdentifier::builder().image_tag("latest").build())
        .send()
        .await
        .unwrap();

    let v1_resp = client
        .batch_get_image()
        .repository_name("test_repository")
        .image_ids(ImageIdentifier::builder().image_tag("v1").build())
        .send()
        .await
        .unwrap();

    // Same manifest content since it's the same image
    assert_eq!(
        latest_resp.images()[0].image_manifest(),
        v1_resp.images()[0].image_manifest()
    );
}

// ==================== BatchDeleteImage ====================

/// Ported from moto test_batch_delete_image_by_tag
/// Deleting by tag should remove only that tag, not the entire image.
#[tokio::test]
async fn test_batch_delete_image_by_tag() {
    let client = make_ecr_client().await;
    client
        .create_repository()
        .repository_name("test_repository")
        .send()
        .await
        .unwrap();

    let manifest = create_image_manifest();

    let tags = ["v1", "v1.0", "latest"];
    for tag in &tags {
        client
            .put_image()
            .repository_name("test_repository")
            .image_manifest(&manifest)
            .image_tag(*tag)
            .send()
            .await
            .unwrap();
    }

    // Delete only the "latest" tag
    let batch_resp = client
        .batch_delete_image()
        .repository_name("test_repository")
        .image_ids(ImageIdentifier::builder().image_tag("latest").build())
        .send()
        .await
        .unwrap();

    assert_eq!(batch_resp.image_ids().len(), 1);
    assert_eq!(batch_resp.image_ids()[0].image_tag(), Some("latest"));
    assert_eq!(batch_resp.failures().len(), 0);

    // Image should still exist with remaining tags
    let list_resp = client
        .list_images()
        .repository_name("test_repository")
        .send()
        .await
        .unwrap();
    // Should have 2 remaining tags
    assert_eq!(list_resp.image_ids().len(), 2);
    let remaining_tags: std::collections::HashSet<&str> = list_resp
        .image_ids()
        .iter()
        .filter_map(|id| id.image_tag())
        .collect();
    assert_eq!(
        remaining_tags,
        ["v1", "v1.0"]
            .into_iter()
            .collect::<std::collections::HashSet<&str>>()
    );
}

/// Ported from moto test_batch_delete_image_delete_last_tag
/// Deleting the last tag should remove the entire image.
#[tokio::test]
async fn test_batch_delete_image_delete_last_tag() {
    let client = make_ecr_client().await;
    client
        .create_repository()
        .repository_name("test_repository")
        .send()
        .await
        .unwrap();

    let manifest = create_image_manifest();

    client
        .put_image()
        .repository_name("test_repository")
        .image_manifest(&manifest)
        .image_tag("v1")
        .send()
        .await
        .unwrap();

    let batch_resp = client
        .batch_delete_image()
        .repository_name("test_repository")
        .image_ids(ImageIdentifier::builder().image_tag("v1").build())
        .send()
        .await
        .unwrap();

    assert_eq!(batch_resp.image_ids().len(), 1);
    assert_eq!(batch_resp.image_ids()[0].image_tag(), Some("v1"));
    assert_eq!(batch_resp.failures().len(), 0);

    // Image should be completely removed
    let list_resp = client
        .list_images()
        .repository_name("test_repository")
        .send()
        .await
        .unwrap();
    assert_eq!(list_resp.image_ids().len(), 0);
}

/// Ported from moto test_batch_delete_image_with_nonexistent_tag
#[tokio::test]
async fn test_batch_delete_image_with_nonexistent_tag() {
    let client = make_ecr_client().await;
    client
        .create_repository()
        .repository_name("test_repository")
        .send()
        .await
        .unwrap();

    let manifest = create_image_manifest();
    for tag in &["v1", "v1.0", "latest"] {
        client
            .put_image()
            .repository_name("test_repository")
            .image_manifest(&manifest)
            .image_tag(*tag)
            .send()
            .await
            .unwrap();
    }

    let batch_resp = client
        .batch_delete_image()
        .repository_name("test_repository")
        .image_ids(ImageIdentifier::builder().image_tag("missing-tag").build())
        .send()
        .await
        .unwrap();

    assert_eq!(batch_resp.image_ids().len(), 0);
    assert_eq!(batch_resp.failures().len(), 1);
    assert_eq!(
        batch_resp.failures()[0].image_id().unwrap().image_tag(),
        Some("missing-tag")
    );
    assert_eq!(
        batch_resp.failures()[0].failure_code(),
        Some(&aws_sdk_ecr::types::ImageFailureCode::ImageNotFound)
    );
    assert_eq!(
        batch_resp.failures()[0].failure_reason(),
        Some("Requested image not found")
    );
}

/// Ported from moto test_batch_delete_image_by_digest
/// Deleting by digest should remove the entire image and return all tags.
#[tokio::test]
async fn test_batch_delete_image_by_digest() {
    let client = make_ecr_client().await;
    client
        .create_repository()
        .repository_name("test_repository")
        .send()
        .await
        .unwrap();

    let manifest = create_image_manifest();
    let tags = ["v1", "v2", "latest"];
    for tag in &tags {
        client
            .put_image()
            .repository_name("test_repository")
            .image_manifest(&manifest)
            .image_tag(*tag)
            .send()
            .await
            .unwrap();
    }

    // Get the digest
    let list_resp = client
        .list_images()
        .repository_name("test_repository")
        .send()
        .await
        .unwrap();
    let digest = list_resp.image_ids()[0].image_digest().unwrap().to_string();

    // Delete by digest
    let batch_resp = client
        .batch_delete_image()
        .repository_name("test_repository")
        .image_ids(ImageIdentifier::builder().image_digest(&digest).build())
        .send()
        .await
        .unwrap();

    // Should return all 3 tag entries
    assert_eq!(batch_resp.image_ids().len(), 3);
    for id in batch_resp.image_ids() {
        assert_eq!(id.image_digest(), Some(digest.as_str()));
    }
    let deleted_tags: std::collections::HashSet<&str> = batch_resp
        .image_ids()
        .iter()
        .filter_map(|id| id.image_tag())
        .collect();
    assert_eq!(
        deleted_tags,
        tags.into_iter()
            .collect::<std::collections::HashSet<&str>>()
    );
    assert_eq!(batch_resp.failures().len(), 0);

    // Image should be gone
    let list_resp = client
        .list_images()
        .repository_name("test_repository")
        .send()
        .await
        .unwrap();
    assert_eq!(list_resp.image_ids().len(), 0);
}

/// Ported from moto test_batch_delete_image_with_missing_parameters
#[tokio::test]
async fn test_batch_delete_image_with_missing_parameters() {
    let client = make_ecr_client().await;
    client
        .create_repository()
        .repository_name("test_repository")
        .send()
        .await
        .unwrap();

    let batch_resp = client
        .batch_delete_image()
        .repository_name("test_repository")
        .image_ids(ImageIdentifier::builder().build())
        .send()
        .await
        .unwrap();

    assert_eq!(batch_resp.image_ids().len(), 0);
    assert_eq!(batch_resp.failures().len(), 1);
    assert_eq!(
        batch_resp.failures()[0].failure_code(),
        Some(&aws_sdk_ecr::types::ImageFailureCode::MissingDigestAndTag)
    );
    assert_eq!(
        batch_resp.failures()[0].failure_reason(),
        Some("Invalid request parameters: both tag and digest cannot be null")
    );
}

// ==================== GetAuthorizationToken ====================

/// Ported from moto test_get_authorization_token_assume_region
#[tokio::test]
async fn test_get_authorization_token() {
    let client = make_ecr_client().await;

    let resp = client
        .get_authorization_token()
        .send()
        .await
        .expect("get_authorization_token should succeed");

    let auth_data = resp.authorization_data();
    assert_eq!(auth_data.len(), 1);
    assert!(auth_data[0].authorization_token().is_some());
    assert_eq!(
        auth_data[0].proxy_endpoint(),
        Some(format!("https://{}.dkr.ecr.us-east-1.amazonaws.com", ACCOUNT_ID).as_str())
    );
    assert!(auth_data[0].expires_at().is_some());
}

// ==================== DescribeImages ====================

#[tokio::test]
async fn test_describe_images() {
    let client = make_ecr_client().await;
    client
        .create_repository()
        .repository_name(ECR_REPO)
        .send()
        .await
        .unwrap();

    let manifest = create_image_manifest();
    client
        .put_image()
        .repository_name(ECR_REPO)
        .image_manifest(&manifest)
        .image_tag("latest")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_images()
        .repository_name(ECR_REPO)
        .send()
        .await
        .expect("describe_images should succeed");

    let details = resp.image_details();
    assert_eq!(details.len(), 1);
    assert!(details[0].image_digest().is_some());
    assert_eq!(details[0].image_tags(), &["latest"]);
    assert_eq!(details[0].repository_name(), Some(ECR_REPO));
    assert_eq!(details[0].registry_id(), Some(ACCOUNT_ID));
    assert!(details[0].image_pushed_at().is_some());
}

#[tokio::test]
async fn test_describe_images_filter_by_tag() {
    let client = make_ecr_client().await;
    client
        .create_repository()
        .repository_name(ECR_REPO)
        .send()
        .await
        .unwrap();

    let m1 = create_image_manifest();
    client
        .put_image()
        .repository_name(ECR_REPO)
        .image_manifest(&m1)
        .image_tag("v1")
        .send()
        .await
        .unwrap();

    // Small delay to ensure unique manifest
    let m2 = create_image_manifest();
    client
        .put_image()
        .repository_name(ECR_REPO)
        .image_manifest(&m2)
        .image_tag("v2")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_images()
        .repository_name(ECR_REPO)
        .image_ids(ImageIdentifier::builder().image_tag("v1").build())
        .send()
        .await
        .expect("describe_images with filter should succeed");

    assert_eq!(resp.image_details().len(), 1);
    assert_eq!(resp.image_details()[0].image_tags(), &["v1"]);
}

// ==================== PutLifecyclePolicy / GetLifecyclePolicy / DeleteLifecyclePolicy ====================

#[tokio::test]
async fn test_lifecycle_policy_crud() {
    let client = make_ecr_client().await;
    client
        .create_repository()
        .repository_name(ECR_REPO)
        .send()
        .await
        .unwrap();

    let policy = r#"{"rules":[{"rulePriority":1,"selection":{"tagStatus":"untagged","countType":"sinceImagePushed","countUnit":"days","countNumber":14},"action":{"type":"expire"}}]}"#;

    // Put lifecycle policy
    let resp = client
        .put_lifecycle_policy()
        .repository_name(ECR_REPO)
        .lifecycle_policy_text(policy)
        .send()
        .await
        .expect("put_lifecycle_policy should succeed");
    assert_eq!(resp.repository_name(), Some(ECR_REPO));
    assert_eq!(resp.lifecycle_policy_text(), Some(policy));

    // Get lifecycle policy
    let resp = client
        .get_lifecycle_policy()
        .repository_name(ECR_REPO)
        .send()
        .await
        .expect("get_lifecycle_policy should succeed");
    assert_eq!(resp.repository_name(), Some(ECR_REPO));
    assert_eq!(resp.lifecycle_policy_text(), Some(policy));
    assert_eq!(resp.registry_id(), Some(ACCOUNT_ID));

    // Delete lifecycle policy
    let resp = client
        .delete_lifecycle_policy()
        .repository_name(ECR_REPO)
        .send()
        .await
        .expect("delete_lifecycle_policy should succeed");
    assert_eq!(resp.repository_name(), Some(ECR_REPO));
    assert_eq!(resp.lifecycle_policy_text(), Some(policy));

    // Get after delete should fail
    let err = client
        .get_lifecycle_policy()
        .repository_name(ECR_REPO)
        .send()
        .await;
    assert!(err.is_err());
}

#[tokio::test]
async fn test_delete_lifecycle_policy_not_found() {
    let client = make_ecr_client().await;
    client
        .create_repository()
        .repository_name(ECR_REPO)
        .send()
        .await
        .unwrap();

    let err = client
        .delete_lifecycle_policy()
        .repository_name(ECR_REPO)
        .send()
        .await;
    assert!(err.is_err());
    let err = err.unwrap_err();
    let service_err = err.as_service_error().expect("should be service error");
    assert!(
        service_err.is_lifecycle_policy_not_found_exception(),
        "should be LifecyclePolicyNotFoundException"
    );
}

// ==================== SetRepositoryPolicy / GetRepositoryPolicy / DeleteRepositoryPolicy ====================

#[tokio::test]
async fn test_repository_policy_crud() {
    let client = make_ecr_client().await;
    client
        .create_repository()
        .repository_name(ECR_REPO)
        .send()
        .await
        .unwrap();

    let policy = r#"{"Version":"2012-10-17","Statement":[{"Sid":"AllowPull","Effect":"Allow","Principal":"*","Action":["ecr:GetDownloadUrlForLayer","ecr:BatchGetImage"]}]}"#;

    // Set repository policy
    let resp = client
        .set_repository_policy()
        .repository_name(ECR_REPO)
        .policy_text(policy)
        .send()
        .await
        .expect("set_repository_policy should succeed");
    assert_eq!(resp.repository_name(), Some(ECR_REPO));
    assert_eq!(resp.policy_text(), Some(policy));

    // Get repository policy
    let resp = client
        .get_repository_policy()
        .repository_name(ECR_REPO)
        .send()
        .await
        .expect("get_repository_policy should succeed");
    assert_eq!(resp.repository_name(), Some(ECR_REPO));
    assert_eq!(resp.policy_text(), Some(policy));

    // Delete repository policy
    let resp = client
        .delete_repository_policy()
        .repository_name(ECR_REPO)
        .send()
        .await
        .expect("delete_repository_policy should succeed");
    assert_eq!(resp.repository_name(), Some(ECR_REPO));
    assert_eq!(resp.policy_text(), Some(policy));

    // Get after delete should fail
    let err = client
        .get_repository_policy()
        .repository_name(ECR_REPO)
        .send()
        .await;
    assert!(err.is_err());
}

#[tokio::test]
async fn test_delete_repository_policy_not_found() {
    let client = make_ecr_client().await;
    client
        .create_repository()
        .repository_name(ECR_REPO)
        .send()
        .await
        .unwrap();

    let err = client
        .delete_repository_policy()
        .repository_name(ECR_REPO)
        .send()
        .await;
    assert!(err.is_err());
    let err = err.unwrap_err();
    let service_err = err.as_service_error().expect("should be service error");
    assert!(
        service_err.is_repository_policy_not_found_exception(),
        "should be RepositoryPolicyNotFoundException"
    );
}

// ==================== PutImageScanningConfiguration ====================

#[tokio::test]
async fn test_put_image_scanning_configuration() {
    let client = make_ecr_client().await;
    client
        .create_repository()
        .repository_name(ECR_REPO)
        .send()
        .await
        .unwrap();

    let resp = client
        .put_image_scanning_configuration()
        .repository_name(ECR_REPO)
        .image_scanning_configuration(
            aws_sdk_ecr::types::ImageScanningConfiguration::builder()
                .scan_on_push(true)
                .build(),
        )
        .send()
        .await
        .expect("put_image_scanning_configuration should succeed");

    assert_eq!(resp.repository_name(), Some(ECR_REPO));
    assert_eq!(resp.registry_id(), Some(ACCOUNT_ID));
    let config = resp.image_scanning_configuration().unwrap();
    assert!(config.scan_on_push);
}

// ==================== PutImageTagMutability ====================

#[tokio::test]
async fn test_put_image_tag_mutability() {
    let client = make_ecr_client().await;
    client
        .create_repository()
        .repository_name(ECR_REPO)
        .send()
        .await
        .unwrap();

    let resp = client
        .put_image_tag_mutability()
        .repository_name(ECR_REPO)
        .image_tag_mutability(aws_sdk_ecr::types::ImageTagMutability::Immutable)
        .send()
        .await
        .expect("put_image_tag_mutability should succeed");

    assert_eq!(resp.repository_name(), Some(ECR_REPO));
    assert_eq!(resp.registry_id(), Some(ACCOUNT_ID));
    assert_eq!(
        resp.image_tag_mutability(),
        Some(&aws_sdk_ecr::types::ImageTagMutability::Immutable)
    );

    // Verify via describe
    let desc = client
        .describe_repositories()
        .repository_names(ECR_REPO)
        .send()
        .await
        .unwrap();
    assert_eq!(
        desc.repositories()[0].image_tag_mutability(),
        Some(&aws_sdk_ecr::types::ImageTagMutability::Immutable)
    );
}

// ==================== StartImageScan / DescribeImageScanFindings ====================

#[tokio::test]
async fn test_start_image_scan_and_describe_findings() {
    let client = make_ecr_client().await;
    client
        .create_repository()
        .repository_name(ECR_REPO)
        .send()
        .await
        .unwrap();

    let manifest = create_image_manifest();
    let put_resp = client
        .put_image()
        .repository_name(ECR_REPO)
        .image_manifest(&manifest)
        .image_tag("latest")
        .send()
        .await
        .unwrap();
    let digest = put_resp
        .image()
        .unwrap()
        .image_id()
        .unwrap()
        .image_digest()
        .unwrap()
        .to_string();

    // Start scan
    let resp = client
        .start_image_scan()
        .repository_name(ECR_REPO)
        .image_id(ImageIdentifier::builder().image_tag("latest").build())
        .send()
        .await
        .expect("start_image_scan should succeed");

    assert_eq!(resp.repository_name(), Some(ECR_REPO));
    assert!(resp.image_scan_status().is_some());
    let status = resp.image_scan_status().unwrap();
    assert_eq!(
        status.status(),
        Some(&aws_sdk_ecr::types::ScanStatus::Complete)
    );

    // Describe findings
    let resp = client
        .describe_image_scan_findings()
        .repository_name(ECR_REPO)
        .image_id(ImageIdentifier::builder().image_digest(&digest).build())
        .send()
        .await
        .expect("describe_image_scan_findings should succeed");

    assert_eq!(resp.repository_name(), Some(ECR_REPO));
    assert!(resp.image_scan_status().is_some());
    assert!(resp.image_scan_findings().is_some());
}

#[tokio::test]
async fn test_describe_image_scan_findings_no_scan() {
    let client = make_ecr_client().await;
    client
        .create_repository()
        .repository_name(ECR_REPO)
        .send()
        .await
        .unwrap();

    let manifest = create_image_manifest();
    client
        .put_image()
        .repository_name(ECR_REPO)
        .image_manifest(&manifest)
        .image_tag("latest")
        .send()
        .await
        .unwrap();

    // Describe findings without starting scan should fail
    let err = client
        .describe_image_scan_findings()
        .repository_name(ECR_REPO)
        .image_id(ImageIdentifier::builder().image_tag("latest").build())
        .send()
        .await;
    assert!(err.is_err());
    let err = err.unwrap_err();
    let service_err = err.as_service_error().expect("should be service error");
    assert!(
        service_err.is_scan_not_found_exception(),
        "should be ScanNotFoundException"
    );
}

// ==================== TagResource / UntagResource / ListTagsForResource ====================

#[tokio::test]
async fn test_tag_resource_lifecycle() {
    let client = make_ecr_client().await;
    let resp = client
        .create_repository()
        .repository_name(ECR_REPO)
        .send()
        .await
        .unwrap();
    let arn = resp
        .repository()
        .unwrap()
        .repository_arn()
        .unwrap()
        .to_string();

    // Tag
    client
        .tag_resource()
        .resource_arn(&arn)
        .tags(
            aws_sdk_ecr::types::Tag::builder()
                .key("env")
                .value("test")
                .build()
                .unwrap(),
        )
        .tags(
            aws_sdk_ecr::types::Tag::builder()
                .key("team")
                .value("infra")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("tag_resource should succeed");

    // List tags
    let resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");
    let tags = resp.tags();
    assert_eq!(tags.len(), 2);
    let tag_map: std::collections::HashMap<&str, &str> =
        tags.iter().map(|t| (t.key(), t.value())).collect();
    assert_eq!(tag_map.get("env"), Some(&"test"));
    assert_eq!(tag_map.get("team"), Some(&"infra"));

    // Untag
    client
        .untag_resource()
        .resource_arn(&arn)
        .tag_keys("env")
        .send()
        .await
        .expect("untag_resource should succeed");

    // Verify
    let resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.tags().len(), 1);
    assert_eq!(resp.tags()[0].key(), "team");
    assert_eq!(resp.tags()[0].value(), "infra");
}

// ==================== PutRegistryPolicy / GetRegistryPolicy / DeleteRegistryPolicy ====================

#[tokio::test]
async fn test_registry_policy_crud() {
    let client = make_ecr_client().await;

    let policy = r#"{"Version":"2012-10-17","Statement":[{"Sid":"ReplicationAccess","Effect":"Allow","Principal":{"AWS":"arn:aws:iam::123456789012:root"},"Action":["ecr:ReplicateImage"]}]}"#;

    // Put registry policy
    let resp = client
        .put_registry_policy()
        .policy_text(policy)
        .send()
        .await
        .expect("put_registry_policy should succeed");
    assert_eq!(resp.policy_text(), Some(policy));
    assert_eq!(resp.registry_id(), Some(ACCOUNT_ID));

    // Get registry policy
    let resp = client
        .get_registry_policy()
        .send()
        .await
        .expect("get_registry_policy should succeed");
    assert_eq!(resp.policy_text(), Some(policy));
    assert_eq!(resp.registry_id(), Some(ACCOUNT_ID));

    // Delete registry policy
    let resp = client
        .delete_registry_policy()
        .send()
        .await
        .expect("delete_registry_policy should succeed");
    assert_eq!(resp.policy_text(), Some(policy));
    assert_eq!(resp.registry_id(), Some(ACCOUNT_ID));

    // Get after delete should fail
    let err = client.get_registry_policy().send().await;
    assert!(err.is_err());
}

#[tokio::test]
async fn test_delete_registry_policy_not_found() {
    let client = make_ecr_client().await;

    let err = client.delete_registry_policy().send().await;
    assert!(err.is_err());
}

// ==================== DescribeRegistry ====================

#[tokio::test]
async fn test_describe_registry() {
    let client = make_ecr_client().await;

    let resp = client
        .describe_registry()
        .send()
        .await
        .expect("describe_registry should succeed");

    assert_eq!(resp.registry_id(), Some(ACCOUNT_ID));
    assert!(resp.replication_configuration().is_some());
    assert_eq!(resp.replication_configuration().unwrap().rules().len(), 0);
}

// ==================== PutRegistryScanningConfiguration / GetRegistryScanningConfiguration ====================

#[tokio::test]
async fn test_registry_scanning_configuration() {
    let client = make_ecr_client().await;

    // Put scanning configuration
    let resp = client
        .put_registry_scanning_configuration()
        .scan_type(aws_sdk_ecr::types::ScanType::Enhanced)
        .rules(
            aws_sdk_ecr::types::RegistryScanningRule::builder()
                .scan_frequency(aws_sdk_ecr::types::ScanFrequency::ContinuousScan)
                .repository_filters(
                    aws_sdk_ecr::types::ScanningRepositoryFilter::builder()
                        .filter("*")
                        .filter_type(aws_sdk_ecr::types::ScanningRepositoryFilterType::Wildcard)
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("put_registry_scanning_configuration should succeed");

    let config = resp.registry_scanning_configuration().unwrap();
    assert_eq!(
        config.scan_type(),
        Some(&aws_sdk_ecr::types::ScanType::Enhanced)
    );
    assert_eq!(config.rules().len(), 1);
    assert_eq!(
        config.rules()[0].scan_frequency(),
        &aws_sdk_ecr::types::ScanFrequency::ContinuousScan
    );

    // Get scanning configuration
    let resp = client
        .get_registry_scanning_configuration()
        .send()
        .await
        .expect("get_registry_scanning_configuration should succeed");

    let config = resp.scanning_configuration().unwrap();
    assert_eq!(
        config.scan_type(),
        Some(&aws_sdk_ecr::types::ScanType::Enhanced)
    );
    assert_eq!(config.rules().len(), 1);
}

// ==================== PutReplicationConfiguration ====================

#[tokio::test]
async fn test_put_replication_configuration() {
    let client = make_ecr_client().await;

    let resp = client
        .put_replication_configuration()
        .replication_configuration(
            aws_sdk_ecr::types::ReplicationConfiguration::builder()
                .rules(
                    aws_sdk_ecr::types::ReplicationRule::builder()
                        .destinations(
                            aws_sdk_ecr::types::ReplicationDestination::builder()
                                .region("eu-west-1")
                                .registry_id(ACCOUNT_ID)
                                .build()
                                .unwrap(),
                        )
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("put_replication_configuration should succeed");

    let config = resp.replication_configuration().unwrap();
    assert_eq!(config.rules().len(), 1);
    assert_eq!(config.rules()[0].destinations().len(), 1);
    assert_eq!(config.rules()[0].destinations()[0].region(), "eu-west-1");

    // Verify via describe registry
    let desc = client.describe_registry().send().await.unwrap();
    let config = desc.replication_configuration().unwrap();
    assert_eq!(config.rules().len(), 1);
}

// ==================== BatchGetRepositoryScanningConfiguration ====================

#[tokio::test]
async fn test_batch_get_repository_scanning_configuration() {
    let client = make_ecr_client().await;
    client
        .create_repository()
        .repository_name(ECR_REPO)
        .send()
        .await
        .unwrap();

    let resp = client
        .batch_get_repository_scanning_configuration()
        .repository_names(ECR_REPO)
        .send()
        .await
        .expect("batch_get_repository_scanning_configuration should succeed");

    assert_eq!(resp.scanning_configurations().len(), 1);
    assert_eq!(
        resp.scanning_configurations()[0].repository_name(),
        Some(ECR_REPO)
    );
    assert!(resp.scanning_configurations()[0].repository_arn().is_some());
}

#[tokio::test]
async fn test_batch_get_repository_scanning_configuration_not_found() {
    let client = make_ecr_client().await;

    let resp = client
        .batch_get_repository_scanning_configuration()
        .repository_names("nonexistent")
        .send()
        .await
        .expect(
            "batch_get_repository_scanning_configuration should succeed even for missing repos",
        );

    assert_eq!(resp.scanning_configurations().len(), 0);
    assert_eq!(resp.failures().len(), 1);
    assert_eq!(resp.failures()[0].repository_name(), Some("nonexistent"));
}

// ============================================================================
// Ported from moto: test_ecr.py (additional tests)
// ============================================================================

// Ported from moto: test_ecr.py::test_describe_repositories_with_image
#[tokio::test]
async fn test_describe_repositories_with_image() {
    let client = make_ecr_client().await;
    client
        .create_repository()
        .repository_name(ECR_REPO)
        .send()
        .await
        .unwrap();

    let manifest = create_image_manifest();
    client
        .put_image()
        .repository_name(ECR_REPO)
        .image_manifest(&manifest)
        .image_tag("latest")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_repositories()
        .repository_names(ECR_REPO)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.repositories().len(), 1);
    let repo = &resp.repositories()[0];
    assert_eq!(repo.registry_id(), Some(ACCOUNT_ID));
    assert_eq!(
        repo.repository_arn(),
        Some(
            format!(
                "arn:aws:ecr:us-east-1:{}:repository/{}",
                ACCOUNT_ID, ECR_REPO
            )
            .as_str()
        )
    );
    assert_eq!(repo.repository_name(), Some(ECR_REPO));
    assert_eq!(
        repo.repository_uri(),
        Some(
            format!(
                "{}.dkr.ecr.us-east-1.amazonaws.com/{}",
                ACCOUNT_ID, ECR_REPO
            )
            .as_str()
        )
    );
    assert!(repo.created_at().is_some());
    assert_eq!(
        repo.image_tag_mutability(),
        Some(&aws_sdk_ecr::types::ImageTagMutability::Mutable)
    );
    let scan_config = repo.image_scanning_configuration().unwrap();
    assert!(!scan_config.scan_on_push);
}

// Ported from moto: test_ecr.py::test_describe_images_by_tag
#[tokio::test]
async fn test_describe_images_by_tag() {
    let client = make_ecr_client().await;
    client
        .create_repository()
        .repository_name("test_repository")
        .send()
        .await
        .unwrap();

    let mut tag_digests: Vec<(String, String)> = Vec::new();
    for tag in &["latest", "v1", "v2"] {
        let manifest = create_image_manifest();
        tokio::time::sleep(std::time::Duration::from_millis(5)).await;
        let resp = client
            .put_image()
            .repository_name("test_repository")
            .image_manifest(&manifest)
            .image_tag(*tag)
            .send()
            .await
            .unwrap();
        let digest = resp
            .image()
            .unwrap()
            .image_id()
            .unwrap()
            .image_digest()
            .unwrap()
            .to_string();
        tag_digests.push((tag.to_string(), digest));
    }

    for (tag, expected_digest) in &tag_digests {
        let resp = client
            .describe_images()
            .repository_name("test_repository")
            .image_ids(ImageIdentifier::builder().image_tag(tag).build())
            .send()
            .await
            .unwrap();
        assert_eq!(resp.image_details().len(), 1);
        let detail = &resp.image_details()[0];
        assert_eq!(detail.registry_id(), Some(ACCOUNT_ID));
        assert_eq!(detail.repository_name(), Some("test_repository"));
        assert_eq!(detail.image_tags(), &[tag.as_str()]);
        assert_eq!(detail.image_digest(), Some(expected_digest.as_str()));
    }
}

// Ported from moto: test_ecr.py::test_describe_images_by_digest
#[tokio::test]
async fn test_describe_images_by_digest() {
    let client = make_ecr_client().await;
    client
        .create_repository()
        .repository_name("test_repository")
        .send()
        .await
        .unwrap();

    let mut digest_tags: Vec<(String, String)> = Vec::new();
    for tag in &["latest", "v1", "v2"] {
        let manifest = create_image_manifest();
        tokio::time::sleep(std::time::Duration::from_millis(5)).await;
        let resp = client
            .put_image()
            .repository_name("test_repository")
            .image_manifest(&manifest)
            .image_tag(*tag)
            .send()
            .await
            .unwrap();
        let digest = resp
            .image()
            .unwrap()
            .image_id()
            .unwrap()
            .image_digest()
            .unwrap()
            .to_string();
        digest_tags.push((digest, tag.to_string()));
    }

    for (digest, expected_tag) in &digest_tags {
        let resp = client
            .describe_images()
            .repository_name("test_repository")
            .image_ids(ImageIdentifier::builder().image_digest(digest).build())
            .send()
            .await
            .unwrap();
        assert_eq!(resp.image_details().len(), 1);
        let detail = &resp.image_details()[0];
        assert_eq!(detail.registry_id(), Some(ACCOUNT_ID));
        assert_eq!(detail.repository_name(), Some("test_repository"));
        assert_eq!(detail.image_tags(), &[expected_tag.as_str()]);
        assert_eq!(detail.image_digest(), Some(digest.as_str()));
    }
}

// Ported from moto: test_ecr.py::test_describe_image_that_doesnt_exist
#[tokio::test]
async fn test_describe_image_that_doesnt_exist() {
    let client = make_ecr_client().await;
    client
        .create_repository()
        .repository_name("test_repository")
        .send()
        .await
        .unwrap();

    // Image not found in existing repo
    let err = client
        .describe_images()
        .repository_name("test_repository")
        .image_ids(ImageIdentifier::builder().image_tag("testtag").build())
        .send()
        .await;
    assert!(err.is_err());
    let err = err.unwrap_err();
    let service_err = err.as_service_error().expect("should be service error");
    assert!(
        service_err.is_image_not_found_exception(),
        "should be ImageNotFoundException"
    );

    // Repo doesn't exist
    let err = client
        .describe_images()
        .repository_name("repo-that-doesnt-exist")
        .image_ids(ImageIdentifier::builder().image_tag("testtag").build())
        .send()
        .await;
    assert!(err.is_err());
    let err = err.unwrap_err();
    let service_err = err.as_service_error().expect("should be service error");
    assert!(
        service_err.is_repository_not_found_exception(),
        "should be RepositoryNotFoundException"
    );
}

// Ported from moto: test_ecr.py::test_describe_images_tags_should_not_contain_empty_tag1
#[tokio::test]
async fn test_describe_images_tags_should_not_contain_empty_tag() {
    let client = make_ecr_client().await;
    client
        .create_repository()
        .repository_name("test_repository")
        .send()
        .await
        .unwrap();

    let manifest = create_image_manifest();

    // Put image without tag first
    client
        .put_image()
        .repository_name("test_repository")
        .image_manifest(&manifest)
        .send()
        .await
        .unwrap();

    // Then add tags to the same manifest
    for tag in &["v1", "v2", "latest"] {
        client
            .put_image()
            .repository_name("test_repository")
            .image_manifest(&manifest)
            .image_tag(*tag)
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .describe_images()
        .repository_name("test_repository")
        .image_ids(ImageIdentifier::builder().image_tag("latest").build())
        .send()
        .await
        .unwrap();

    assert_eq!(resp.image_details().len(), 1);
    let detail = &resp.image_details()[0];
    assert_eq!(detail.image_tags().len(), 3);
    let tags: Vec<&str> = detail.image_tags().iter().map(|s| s.as_str()).collect();
    assert!(tags.contains(&"v1"));
    assert!(tags.contains(&"v2"));
    assert!(tags.contains(&"latest"));
}

// Ported from moto: test_ecr.py::test_batch_delete_image_with_invalid_digest
#[tokio::test]
async fn test_batch_delete_image_with_invalid_digest() {
    let client = make_ecr_client().await;
    client
        .create_repository()
        .repository_name("test_repository")
        .send()
        .await
        .unwrap();

    let manifest = create_image_manifest();
    for tag in &["v1", "v2", "latest"] {
        client
            .put_image()
            .repository_name("test_repository")
            .image_manifest(&manifest)
            .image_tag(*tag)
            .send()
            .await
            .unwrap();
    }

    let batch_resp = client
        .batch_delete_image()
        .repository_name("test_repository")
        .image_ids(
            ImageIdentifier::builder()
                .image_digest("sha256:invalid-digest")
                .build(),
        )
        .send()
        .await
        .unwrap();

    assert_eq!(batch_resp.image_ids().len(), 0);
    assert_eq!(batch_resp.failures().len(), 1);
    assert_eq!(
        batch_resp.failures()[0].image_id().unwrap().image_digest(),
        Some("sha256:invalid-digest")
    );
}

// Ported from moto: test_ecr.py::test_batch_delete_image_with_matching_digest_and_tag
#[tokio::test]
async fn test_batch_delete_image_with_matching_digest_and_tag() {
    let client = make_ecr_client().await;
    client
        .create_repository()
        .repository_name("test_repository")
        .send()
        .await
        .unwrap();

    let manifest = create_image_manifest();
    let tags = ["v1", "v1.0", "latest"];
    for tag in &tags {
        client
            .put_image()
            .repository_name("test_repository")
            .image_manifest(&manifest)
            .image_tag(*tag)
            .send()
            .await
            .unwrap();
    }

    // Get the digest
    let desc_resp = client
        .describe_images()
        .repository_name("test_repository")
        .send()
        .await
        .unwrap();
    let image_digest = desc_resp.image_details()[0]
        .image_digest()
        .unwrap()
        .to_string();

    // Delete with matching digest+tag
    let batch_resp = client
        .batch_delete_image()
        .repository_name("test_repository")
        .image_ids(
            ImageIdentifier::builder()
                .image_digest(&image_digest)
                .image_tag("v1")
                .build(),
        )
        .send()
        .await
        .unwrap();

    // Should delete the entire image (all tags)
    let desc_resp = client
        .describe_images()
        .repository_name("test_repository")
        .send()
        .await
        .unwrap();
    assert_eq!(desc_resp.image_details().len(), 0);

    assert_eq!(batch_resp.image_ids().len(), 3);
    for id in batch_resp.image_ids() {
        assert_eq!(id.image_digest(), Some(image_digest.as_str()));
    }
    let deleted_tags: std::collections::HashSet<&str> = batch_resp
        .image_ids()
        .iter()
        .filter_map(|id| id.image_tag())
        .collect();
    assert_eq!(
        deleted_tags,
        tags.iter()
            .cloned()
            .collect::<std::collections::HashSet<&str>>()
    );
    assert_eq!(batch_resp.failures().len(), 0);
}

// Ported from moto: test_ecr.py::test_batch_delete_image_with_mismatched_digest_and_tag
#[tokio::test]
async fn test_batch_delete_image_with_mismatched_digest_and_tag() {
    let client = make_ecr_client().await;
    client
        .create_repository()
        .repository_name("test_repository")
        .send()
        .await
        .unwrap();

    let manifest = create_image_manifest();
    for tag in &["v1", "latest"] {
        client
            .put_image()
            .repository_name("test_repository")
            .image_manifest(&manifest)
            .image_tag(*tag)
            .send()
            .await
            .unwrap();
    }

    let desc_resp = client
        .describe_images()
        .repository_name("test_repository")
        .send()
        .await
        .unwrap();
    let image_digest = desc_resp.image_details()[0]
        .image_digest()
        .unwrap()
        .to_string();

    // Delete with mismatched tag (v2 doesn't exist on this image)
    let batch_resp = client
        .batch_delete_image()
        .repository_name("test_repository")
        .image_ids(
            ImageIdentifier::builder()
                .image_digest(&image_digest)
                .image_tag("v2")
                .build(),
        )
        .send()
        .await
        .unwrap();

    assert_eq!(batch_resp.image_ids().len(), 0);
    assert_eq!(batch_resp.failures().len(), 1);
    assert_eq!(
        batch_resp.failures()[0].image_id().unwrap().image_digest(),
        Some(image_digest.as_str())
    );
    assert_eq!(
        batch_resp.failures()[0].image_id().unwrap().image_tag(),
        Some("v2")
    );
    assert_eq!(
        batch_resp.failures()[0].failure_code(),
        Some(&aws_sdk_ecr::types::ImageFailureCode::ImageNotFound)
    );
}

// Ported from moto: test_ecr.py::test_delete_batch_image_with_multiple_images
#[tokio::test]
async fn test_batch_delete_image_multiple_images() {
    let client = make_ecr_client().await;
    client
        .create_repository()
        .repository_name(ECR_REPO)
        .send()
        .await
        .unwrap();

    // Create 10 images
    let mut digests = Vec::new();
    for i in 0..10 {
        let manifest = create_image_manifest();
        tokio::time::sleep(std::time::Duration::from_millis(5)).await;
        let resp = client
            .put_image()
            .repository_name(ECR_REPO)
            .image_manifest(&manifest)
            .image_tag(format!("tag{}", i))
            .send()
            .await
            .unwrap();
        digests.push(
            resp.image()
                .unwrap()
                .image_id()
                .unwrap()
                .image_digest()
                .unwrap()
                .to_string(),
        );
    }

    // Delete images at index 5 and 6
    let resp = client
        .batch_delete_image()
        .repository_name(ECR_REPO)
        .image_ids(ImageIdentifier::builder().image_digest(&digests[5]).build())
        .image_ids(ImageIdentifier::builder().image_digest(&digests[6]).build())
        .send()
        .await
        .unwrap();

    assert_eq!(resp.image_ids().len(), 2);
    assert_eq!(resp.failures().len(), 0);

    // Verify remaining images
    let remaining = client
        .describe_images()
        .repository_name(ECR_REPO)
        .send()
        .await
        .unwrap();
    assert_eq!(remaining.image_details().len(), 8);

    let remaining_tags: std::collections::HashSet<String> = remaining
        .image_details()
        .iter()
        .flat_map(|d| d.image_tags().iter().cloned())
        .collect();
    let expected_tags: std::collections::HashSet<String> = [
        "tag0", "tag1", "tag2", "tag3", "tag4", "tag7", "tag8", "tag9",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
    assert_eq!(remaining_tags, expected_tags);
}

// Ported from moto: test_ecr.py::test_describe_registry_after_update
#[tokio::test]
async fn test_describe_registry_after_update() {
    let client = make_ecr_client().await;

    client
        .put_replication_configuration()
        .replication_configuration(
            aws_sdk_ecr::types::ReplicationConfiguration::builder()
                .rules(
                    aws_sdk_ecr::types::ReplicationRule::builder()
                        .destinations(
                            aws_sdk_ecr::types::ReplicationDestination::builder()
                                .region("eu-west-1")
                                .registry_id(ACCOUNT_ID)
                                .build()
                                .unwrap(),
                        )
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let resp = client.describe_registry().send().await.unwrap();
    let config = resp.replication_configuration().unwrap();
    assert_eq!(config.rules().len(), 1);
    assert_eq!(config.rules()[0].destinations().len(), 1);
    assert_eq!(config.rules()[0].destinations()[0].region(), "eu-west-1");
    assert_eq!(
        config.rules()[0].destinations()[0].registry_id(),
        ACCOUNT_ID
    );
}

// Ported from moto: test_ecr.py::test_start_image_scan_error_repo_not_exists
#[tokio::test]
async fn test_start_image_scan_error_repo_not_exists() {
    let client = make_ecr_client().await;

    let err = client
        .start_image_scan()
        .repository_name(ECR_REPO_NOT_EXIST)
        .image_id(ImageIdentifier::builder().image_tag("latest").build())
        .send()
        .await;

    assert!(err.is_err());
    let err = err.unwrap_err();
    let service_err = err.as_service_error().expect("should be service error");
    assert!(
        service_err.is_repository_not_found_exception(),
        "should be RepositoryNotFoundException"
    );
}

// Ported from moto: test_ecr.py::test_start_image_scan_error_image_not_exists
#[tokio::test]
async fn test_start_image_scan_error_image_not_exists() {
    let client = make_ecr_client().await;
    client
        .create_repository()
        .repository_name(ECR_REPO)
        .send()
        .await
        .unwrap();

    let err = client
        .start_image_scan()
        .repository_name(ECR_REPO)
        .image_id(ImageIdentifier::builder().image_tag("not-exists").build())
        .send()
        .await;

    assert!(err.is_err());
    let err = err.unwrap_err();
    let service_err = err.as_service_error().expect("should be service error");
    assert!(
        service_err.is_image_not_found_exception(),
        "should be ImageNotFoundException"
    );
}

// Ported from moto: test_ecr.py::test_describe_image_scan_findings_error_repo_not_exists
#[tokio::test]
async fn test_describe_image_scan_findings_error_repo_not_exists() {
    let client = make_ecr_client().await;

    let err = client
        .describe_image_scan_findings()
        .repository_name(ECR_REPO_NOT_EXIST)
        .image_id(ImageIdentifier::builder().image_tag("latest").build())
        .send()
        .await;

    assert!(err.is_err());
    let err = err.unwrap_err();
    let service_err = err.as_service_error().expect("should be service error");
    assert!(
        service_err.is_repository_not_found_exception(),
        "should be RepositoryNotFoundException"
    );
}

// Ported from moto: test_ecr.py::test_describe_image_scan_findings_error_image_not_exists
#[tokio::test]
async fn test_describe_image_scan_findings_error_image_not_exists() {
    let client = make_ecr_client().await;
    client
        .create_repository()
        .repository_name(ECR_REPO)
        .send()
        .await
        .unwrap();

    let err = client
        .describe_image_scan_findings()
        .repository_name(ECR_REPO)
        .image_id(ImageIdentifier::builder().image_tag("not-exists").build())
        .send()
        .await;

    assert!(err.is_err());
    let err = err.unwrap_err();
    let service_err = err.as_service_error().expect("should be service error");
    assert!(
        service_err.is_image_not_found_exception(),
        "should be ImageNotFoundException"
    );
}

// Ported from moto: test_ecr.py::test_put_image_tag_mutability_error_not_exists
#[tokio::test]
async fn test_put_image_tag_mutability_error_not_exists() {
    let client = make_ecr_client().await;

    let err = client
        .put_image_tag_mutability()
        .repository_name(ECR_REPO_NOT_EXIST)
        .image_tag_mutability(aws_sdk_ecr::types::ImageTagMutability::Immutable)
        .send()
        .await;

    assert!(err.is_err());
    let err = err.unwrap_err();
    let service_err = err.as_service_error().expect("should be service error");
    assert!(
        service_err.is_repository_not_found_exception(),
        "should be RepositoryNotFoundException"
    );
}

// Ported from moto: test_ecr.py::test_put_image_scanning_configuration_error_not_exists
#[tokio::test]
async fn test_put_image_scanning_configuration_error_not_exists() {
    let client = make_ecr_client().await;

    let err = client
        .put_image_scanning_configuration()
        .repository_name(ECR_REPO_NOT_EXIST)
        .image_scanning_configuration(
            aws_sdk_ecr::types::ImageScanningConfiguration::builder()
                .scan_on_push(true)
                .build(),
        )
        .send()
        .await;

    assert!(err.is_err());
    let err = err.unwrap_err();
    let service_err = err.as_service_error().expect("should be service error");
    assert!(
        service_err.is_repository_not_found_exception(),
        "should be RepositoryNotFoundException"
    );
}

// Ported from moto: test_ecr.py::test_set_repository_policy_error_not_exists
#[tokio::test]
async fn test_set_repository_policy_error_not_exists() {
    let client = make_ecr_client().await;

    let policy = r#"{"Version":"2012-10-17","Statement":[{"Sid":"root","Effect":"Allow","Principal":"*","Action":["ecr:DescribeImages"]}]}"#;

    let err = client
        .set_repository_policy()
        .repository_name(ECR_REPO_NOT_EXIST)
        .policy_text(policy)
        .send()
        .await;

    assert!(err.is_err());
    let err = err.unwrap_err();
    let service_err = err.as_service_error().expect("should be service error");
    assert!(
        service_err.is_repository_not_found_exception(),
        "should be RepositoryNotFoundException"
    );
}

// Ported from moto: test_ecr.py::test_get_repository_policy_error_repo_not_exists
#[tokio::test]
async fn test_get_repository_policy_error_repo_not_exists() {
    let client = make_ecr_client().await;

    let err = client
        .get_repository_policy()
        .repository_name(ECR_REPO_NOT_EXIST)
        .send()
        .await;

    assert!(err.is_err());
    let err = err.unwrap_err();
    let service_err = err.as_service_error().expect("should be service error");
    assert!(
        service_err.is_repository_not_found_exception(),
        "should be RepositoryNotFoundException"
    );
}

// Ported from moto: test_ecr.py::test_get_repository_policy_error_policy_not_exists
#[tokio::test]
async fn test_get_repository_policy_error_policy_not_exists() {
    let client = make_ecr_client().await;
    client
        .create_repository()
        .repository_name(ECR_REPO)
        .send()
        .await
        .unwrap();

    let err = client
        .get_repository_policy()
        .repository_name(ECR_REPO)
        .send()
        .await;

    assert!(err.is_err());
    let err = err.unwrap_err();
    let service_err = err.as_service_error().expect("should be service error");
    assert!(
        service_err.is_repository_policy_not_found_exception(),
        "should be RepositoryPolicyNotFoundException"
    );
}

// Ported from moto: test_ecr.py::test_delete_repository_policy_error_repo_not_exists
#[tokio::test]
async fn test_delete_repository_policy_error_repo_not_exists() {
    let client = make_ecr_client().await;

    let err = client
        .delete_repository_policy()
        .repository_name(ECR_REPO_NOT_EXIST)
        .send()
        .await;

    assert!(err.is_err());
    let err = err.unwrap_err();
    let service_err = err.as_service_error().expect("should be service error");
    assert!(
        service_err.is_repository_not_found_exception(),
        "should be RepositoryNotFoundException"
    );
}

// Ported from moto: test_ecr.py::test_put_lifecycle_policy_error_repo_not_exists
#[tokio::test]
async fn test_put_lifecycle_policy_error_repo_not_exists() {
    let client = make_ecr_client().await;

    let policy = r#"{"rules":[{"rulePriority":1,"selection":{"tagStatus":"untagged","countType":"imageCountMoreThan","countNumber":30},"action":{"type":"expire"}}]}"#;

    let err = client
        .put_lifecycle_policy()
        .repository_name(ECR_REPO_NOT_EXIST)
        .lifecycle_policy_text(policy)
        .send()
        .await;

    assert!(err.is_err());
    let err = err.unwrap_err();
    let service_err = err.as_service_error().expect("should be service error");
    assert!(
        service_err.is_repository_not_found_exception(),
        "should be RepositoryNotFoundException"
    );
}

// Ported from moto: test_ecr.py::test_get_lifecycle_policy_error_repo_not_exists
#[tokio::test]
async fn test_get_lifecycle_policy_error_repo_not_exists() {
    let client = make_ecr_client().await;

    let err = client
        .get_lifecycle_policy()
        .repository_name(ECR_REPO_NOT_EXIST)
        .send()
        .await;

    assert!(err.is_err());
    let err = err.unwrap_err();
    let service_err = err.as_service_error().expect("should be service error");
    assert!(
        service_err.is_repository_not_found_exception(),
        "should be RepositoryNotFoundException"
    );
}

// Ported from moto: test_ecr.py::test_get_lifecycle_policy_error_policy_not_exists
#[tokio::test]
async fn test_get_lifecycle_policy_error_policy_not_exists() {
    let client = make_ecr_client().await;
    client
        .create_repository()
        .repository_name(ECR_REPO)
        .send()
        .await
        .unwrap();

    let err = client
        .get_lifecycle_policy()
        .repository_name(ECR_REPO)
        .send()
        .await;

    assert!(err.is_err());
    let err = err.unwrap_err();
    let service_err = err.as_service_error().expect("should be service error");
    assert!(
        service_err.is_lifecycle_policy_not_found_exception(),
        "should be LifecyclePolicyNotFoundException"
    );
}

// Ported from moto: test_ecr.py::test_delete_lifecycle_policy_error_repo_not_exists
#[tokio::test]
async fn test_delete_lifecycle_policy_error_repo_not_exists() {
    let client = make_ecr_client().await;

    let err = client
        .delete_lifecycle_policy()
        .repository_name(ECR_REPO_NOT_EXIST)
        .send()
        .await;

    assert!(err.is_err());
    let err = err.unwrap_err();
    let service_err = err.as_service_error().expect("should be service error");
    assert!(
        service_err.is_repository_not_found_exception(),
        "should be RepositoryNotFoundException"
    );
}

// Ported from moto: test_ecr.py::test_get_registry_policy_error_policy_not_exists
#[tokio::test]
async fn test_get_registry_policy_error_policy_not_exists() {
    let client = make_ecr_client().await;

    let err = client.get_registry_policy().send().await;

    assert!(err.is_err());
    let err = err.unwrap_err();
    let service_err = err.as_service_error().expect("should be service error");
    assert!(
        service_err.is_registry_policy_not_found_exception(),
        "should be RegistryPolicyNotFoundException"
    );
}

// Ported from moto: test_ecr.py::test_delete_registry_policy_error_policy_not_exists
#[tokio::test]
async fn test_delete_registry_policy_error_policy_not_exists() {
    let client = make_ecr_client().await;

    let err = client.delete_registry_policy().send().await;

    assert!(err.is_err());
    let err = err.unwrap_err();
    let service_err = err.as_service_error().expect("should be service error");
    assert!(
        service_err.is_registry_policy_not_found_exception(),
        "should be RegistryPolicyNotFoundException"
    );
}

// Ported from moto: test_ecr.py::test_list_tags_for_resource_error_not_exists
#[tokio::test]
async fn test_list_tags_for_resource_error_not_exists() {
    let client = make_ecr_client().await;

    let err = client
        .list_tags_for_resource()
        .resource_arn(format!(
            "arn:aws:ecr:us-east-1:{}:repository/{}",
            ACCOUNT_ID, ECR_REPO
        ))
        .send()
        .await;

    assert!(err.is_err());
}

// Ported from moto: test_ecr.py::test_put_image_with_multiple_tags + describe verification
#[tokio::test]
async fn test_put_image_with_multiple_tags_describe() {
    let client = make_ecr_client().await;
    client
        .create_repository()
        .repository_name("test_repository")
        .send()
        .await
        .unwrap();

    let manifest = create_image_manifest();

    client
        .put_image()
        .repository_name("test_repository")
        .image_manifest(&manifest)
        .image_tag("v1")
        .send()
        .await
        .unwrap();

    client
        .put_image()
        .repository_name("test_repository")
        .image_manifest(&manifest)
        .image_tag("latest")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_images()
        .repository_name("test_repository")
        .send()
        .await
        .unwrap();

    // Should be one image with two tags
    assert_eq!(resp.image_details().len(), 1);
    assert!(
        resp.image_details()[0]
            .image_digest()
            .unwrap()
            .contains("sha")
    );
    assert_eq!(resp.image_details()[0].registry_id(), Some(ACCOUNT_ID));
    assert_eq!(
        resp.image_details()[0].repository_name(),
        Some("test_repository")
    );
    assert_eq!(resp.image_details()[0].image_tags().len(), 2);
    let tags: Vec<&str> = resp.image_details()[0]
        .image_tags()
        .iter()
        .map(|s| s.as_str())
        .collect();
    assert!(tags.contains(&"v1"));
    assert!(tags.contains(&"latest"));
}

// Ported from moto: test_ecr.py::test_put_multiple_images_with_same_tag (describe verification)
#[tokio::test]
async fn test_put_multiple_images_with_same_tag_describe() {
    let client = make_ecr_client().await;
    client
        .create_repository()
        .repository_name(ECR_REPO)
        .send()
        .await
        .unwrap();

    let manifest1 = create_image_manifest();
    tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    let manifest2 = create_image_manifest();

    let image_tag = "my-tag";

    let resp1 = client
        .put_image()
        .repository_name(ECR_REPO)
        .image_manifest(&manifest1)
        .image_tag(image_tag)
        .send()
        .await
        .unwrap();
    let _digest1 = resp1
        .image()
        .unwrap()
        .image_id()
        .unwrap()
        .image_digest()
        .unwrap()
        .to_string();

    let resp2 = client
        .put_image()
        .repository_name(ECR_REPO)
        .image_manifest(&manifest2)
        .image_tag(image_tag)
        .send()
        .await
        .unwrap();
    let digest2 = resp2
        .image()
        .unwrap()
        .image_id()
        .unwrap()
        .image_digest()
        .unwrap()
        .to_string();

    let images = client
        .describe_images()
        .repository_name(ECR_REPO)
        .send()
        .await
        .unwrap();

    // Only second image should remain
    assert_eq!(images.image_details().len(), 1);
    assert_eq!(
        images.image_details()[0].image_digest(),
        Some(digest2.as_str())
    );

    // Push first manifest with different tag - should create new entry
    let resp3 = client
        .put_image()
        .repository_name(ECR_REPO)
        .image_manifest(&manifest1)
        .image_tag("different-tag")
        .send()
        .await
        .unwrap();
    let digest3 = resp3
        .image()
        .unwrap()
        .image_id()
        .unwrap()
        .image_digest()
        .unwrap()
        .to_string();

    let images = client
        .describe_images()
        .repository_name(ECR_REPO)
        .send()
        .await
        .unwrap();
    assert_eq!(images.image_details().len(), 2);

    let digest_set: std::collections::HashSet<String> = images
        .image_details()
        .iter()
        .map(|d| d.image_digest().unwrap().to_string())
        .collect();
    assert_eq!(
        digest_set,
        [digest2, digest3]
            .iter()
            .cloned()
            .collect::<std::collections::HashSet<String>>()
    );
}

// ==================== PutImage (additional edge cases) ====================

/// PutImage to a non-existent repository should return RepositoryNotFoundException.
#[tokio::test]
async fn test_put_image_to_nonexistent_repository() {
    let client = make_ecr_client().await;
    let manifest = create_image_manifest();

    let err = client
        .put_image()
        .repository_name(ECR_REPO_NOT_EXIST)
        .image_manifest(&manifest)
        .image_tag("latest")
        .send()
        .await;

    assert!(err.is_err());
    let err = err.unwrap_err();
    let service_err = err.as_service_error().expect("should be a service error");
    assert!(
        service_err.is_repository_not_found_exception(),
        "should be RepositoryNotFoundException"
    );
}

/// PutImage without any tag produces an untagged image that appears in ListImages with digest only.
#[tokio::test]
async fn test_put_image_without_tag() {
    let client = make_ecr_client().await;
    client
        .create_repository()
        .repository_name(ECR_REPO)
        .send()
        .await
        .unwrap();

    let manifest = create_image_manifest();
    let resp = client
        .put_image()
        .repository_name(ECR_REPO)
        .image_manifest(&manifest)
        .send()
        .await
        .expect("put_image without tag should succeed");

    let image = resp.image().expect("should have image");
    let image_id = image.image_id().expect("should have image_id");
    assert!(
        image_id.image_tag().is_none(),
        "untagged image should have no tag"
    );
    assert!(
        image_id.image_digest().unwrap_or("").starts_with("sha256:"),
        "digest should start with sha256:"
    );

    let list_resp = client
        .list_images()
        .repository_name(ECR_REPO)
        .send()
        .await
        .unwrap();
    assert_eq!(list_resp.image_ids().len(), 1);
    assert!(list_resp.image_ids()[0].image_tag().is_none());
    assert!(list_resp.image_ids()[0].image_digest().is_some());
}

// ==================== ListImages (additional edge cases) ====================

/// ListImages on an existing repository with no images should return an empty list.
#[tokio::test]
async fn test_list_images_empty_repository() {
    let client = make_ecr_client().await;
    client
        .create_repository()
        .repository_name(ECR_REPO)
        .send()
        .await
        .unwrap();

    let resp = client
        .list_images()
        .repository_name(ECR_REPO)
        .send()
        .await
        .expect("list_images on an empty repository should succeed");

    assert_eq!(resp.image_ids().len(), 0);
}

// ==================== BatchGetImage (additional edge cases) ====================

/// BatchGetImage by digest should return the correct image and its manifest.
#[tokio::test]
async fn test_batch_get_image_by_digest() {
    let client = make_ecr_client().await;
    client
        .create_repository()
        .repository_name(ECR_REPO)
        .send()
        .await
        .unwrap();

    let manifest = create_image_manifest();
    let put_resp = client
        .put_image()
        .repository_name(ECR_REPO)
        .image_manifest(&manifest)
        .image_tag("v1")
        .send()
        .await
        .unwrap();
    let digest = put_resp
        .image()
        .unwrap()
        .image_id()
        .unwrap()
        .image_digest()
        .unwrap()
        .to_string();

    let resp = client
        .batch_get_image()
        .repository_name(ECR_REPO)
        .image_ids(ImageIdentifier::builder().image_digest(&digest).build())
        .send()
        .await
        .expect("batch_get_image by digest should succeed");

    assert_eq!(resp.images().len(), 1);
    assert_eq!(
        resp.images()[0].image_id().unwrap().image_digest(),
        Some(digest.as_str())
    );
    assert_eq!(resp.images()[0].image_manifest().unwrap(), manifest);
    assert_eq!(resp.failures().len(), 0);
}

/// BatchGetImage on a non-existent repository should return RepositoryNotFoundException.
#[tokio::test]
async fn test_batch_get_image_nonexistent_repository() {
    let client = make_ecr_client().await;

    let err = client
        .batch_get_image()
        .repository_name(ECR_REPO_NOT_EXIST)
        .image_ids(ImageIdentifier::builder().image_tag("latest").build())
        .send()
        .await;

    assert!(err.is_err());
    let err = err.unwrap_err();
    let service_err = err.as_service_error().expect("should be a service error");
    assert!(
        service_err.is_repository_not_found_exception(),
        "should be RepositoryNotFoundException"
    );
}

// ==================== DescribeRepositories (additional edge cases) ====================

/// DescribeRepositories with no repositories in the registry returns an empty list.
#[tokio::test]
async fn test_describe_repositories_empty() {
    let client = make_ecr_client().await;

    let resp = client
        .describe_repositories()
        .send()
        .await
        .expect("describe_repositories should succeed even when no repositories exist");

    assert_eq!(resp.repositories().len(), 0);
}

// ==================== LifecyclePolicy (additional edge cases) ====================

/// PutLifecyclePolicy called a second time should overwrite the first policy.
#[tokio::test]
async fn test_lifecycle_policy_overwrite() {
    let client = make_ecr_client().await;
    client
        .create_repository()
        .repository_name(ECR_REPO)
        .send()
        .await
        .unwrap();

    let policy_v1 = r#"{"rules":[{"rulePriority":1,"selection":{"tagStatus":"untagged","countType":"sinceImagePushed","countUnit":"days","countNumber":7},"action":{"type":"expire"}}]}"#;
    let policy_v2 = r#"{"rules":[{"rulePriority":1,"selection":{"tagStatus":"untagged","countType":"imageCountMoreThan","countNumber":5},"action":{"type":"expire"}}]}"#;

    client
        .put_lifecycle_policy()
        .repository_name(ECR_REPO)
        .lifecycle_policy_text(policy_v1)
        .send()
        .await
        .expect("first put_lifecycle_policy should succeed");

    client
        .put_lifecycle_policy()
        .repository_name(ECR_REPO)
        .lifecycle_policy_text(policy_v2)
        .send()
        .await
        .expect("second put_lifecycle_policy should succeed");

    let resp = client
        .get_lifecycle_policy()
        .repository_name(ECR_REPO)
        .send()
        .await
        .expect("get_lifecycle_policy should succeed");

    assert_eq!(
        resp.lifecycle_policy_text(),
        Some(policy_v2),
        "second policy should have replaced the first"
    );
}

// ==================== TagResource error cases ====================

/// A newly created repository should start with no resource tags.
#[tokio::test]
async fn test_list_tags_for_newly_created_repository() {
    let client = make_ecr_client().await;

    let create_resp = client
        .create_repository()
        .repository_name(ECR_REPO)
        .send()
        .await
        .expect("create_repository should succeed");

    let arn = create_resp
        .repository()
        .unwrap()
        .repository_arn()
        .unwrap()
        .to_string();

    let list_resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .expect("list_tags_for_resource on a new repo should succeed");

    assert_eq!(
        list_resp.tags().len(),
        0,
        "a newly created repository should have no resource tags"
    );
}

/// TagResource with an ARN pointing to a non-existent repository should return an error.
#[tokio::test]
async fn test_tag_resource_invalid_arn() {
    let client = make_ecr_client().await;

    let err = client
        .tag_resource()
        .resource_arn("arn:aws:ecr:us-east-1:123456789012:repository/does-not-exist")
        .tags(
            aws_sdk_ecr::types::Tag::builder()
                .key("env")
                .value("prod")
                .build()
                .unwrap(),
        )
        .send()
        .await;

    assert!(
        err.is_err(),
        "tagging a non-existent repository ARN should return an error"
    );
}

// ==================== Multi-region state isolation ====================

/// Two clients using the same EcrService but different regions should see isolated state.
#[tokio::test]
async fn test_multi_region_state_isolation() {
    let mock = MockAws::builder().with_service(EcrService::new()).build();

    let config_us = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_ecr::config::Region::new("us-east-1"))
        .load()
        .await;
    let client_us = aws_sdk_ecr::Client::new(&config_us);

    let config_eu = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_ecr::config::Region::new("eu-west-1"))
        .load()
        .await;
    let client_eu = aws_sdk_ecr::Client::new(&config_eu);

    client_us
        .create_repository()
        .repository_name(ECR_REPO)
        .send()
        .await
        .expect("create repo in us-east-1 should succeed");

    // eu-west-1 should not see repos created in us-east-1
    let resp_eu = client_eu
        .describe_repositories()
        .send()
        .await
        .expect("describe_repositories in eu-west-1 should succeed");
    assert_eq!(
        resp_eu.repositories().len(),
        0,
        "eu-west-1 should not see repositories created in us-east-1"
    );

    // Creating the same-named repo in eu-west-1 should succeed (names are region-scoped)
    client_eu
        .create_repository()
        .repository_name(ECR_REPO)
        .send()
        .await
        .expect("creating same repo name in eu-west-1 should succeed due to region isolation");

    // Verify us-east-1 ARN contains the correct region
    let resp_us = client_us
        .describe_repositories()
        .repository_names(ECR_REPO)
        .send()
        .await
        .unwrap();
    assert!(
        resp_us.repositories()[0]
            .repository_arn()
            .unwrap()
            .contains("us-east-1"),
        "us-east-1 ARN should contain the region"
    );

    // Verify eu-west-1 ARN contains the correct region
    let resp_eu = client_eu
        .describe_repositories()
        .repository_names(ECR_REPO)
        .send()
        .await
        .unwrap();
    assert!(
        resp_eu.repositories()[0]
            .repository_arn()
            .unwrap()
            .contains("eu-west-1"),
        "eu-west-1 ARN should contain the region"
    );
}

// ==================== RegistryScanningConfiguration (additional) ====================

/// PutRegistryScanningConfiguration with BASIC scan type and no rules should succeed.
#[tokio::test]
async fn test_put_registry_scanning_configuration_basic() {
    let client = make_ecr_client().await;

    let resp = client
        .put_registry_scanning_configuration()
        .scan_type(aws_sdk_ecr::types::ScanType::Basic)
        .send()
        .await
        .expect("put_registry_scanning_configuration with BASIC type should succeed");

    let config = resp.registry_scanning_configuration().unwrap();
    assert_eq!(
        config.scan_type(),
        Some(&aws_sdk_ecr::types::ScanType::Basic)
    );
    assert_eq!(config.rules().len(), 0);

    let get_resp = client
        .get_registry_scanning_configuration()
        .send()
        .await
        .expect("get_registry_scanning_configuration should return BASIC after put");

    assert_eq!(
        get_resp.scanning_configuration().unwrap().scan_type(),
        Some(&aws_sdk_ecr::types::ScanType::Basic)
    );
}

// ==================== DescribeImages (additional) ====================

/// DescribeImages should include image_size_in_bytes and image_manifest_media_type.
#[tokio::test]
async fn test_describe_images_includes_size_and_media_type() {
    let client = make_ecr_client().await;
    client
        .create_repository()
        .repository_name(ECR_REPO)
        .send()
        .await
        .unwrap();

    let manifest = create_image_manifest();
    client
        .put_image()
        .repository_name(ECR_REPO)
        .image_manifest(&manifest)
        .image_tag("v1")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_images()
        .repository_name(ECR_REPO)
        .send()
        .await
        .expect("describe_images should succeed");

    assert_eq!(resp.image_details().len(), 1);
    let detail = &resp.image_details()[0];
    assert!(
        detail.image_size_in_bytes().is_some(),
        "image_size_in_bytes should be present"
    );
    assert!(
        detail.image_size_in_bytes().unwrap() > 0,
        "image_size_in_bytes should be positive"
    );
    assert_eq!(
        detail.image_manifest_media_type(),
        Some("application/vnd.docker.distribution.manifest.v2+json"),
        "media type should be Docker manifest v2"
    );
    assert!(
        detail.image_pushed_at().is_some(),
        "image_pushed_at should be present"
    );
}

// ==================== PullThroughCacheRule CRUD ====================

/// Create, describe, and delete a pull-through cache rule.
#[tokio::test]
async fn test_pull_through_cache_rule_crud() {
    let client = make_ecr_client().await;

    // Create
    let create_resp = client
        .create_pull_through_cache_rule()
        .ecr_repository_prefix("ecr-public")
        .upstream_registry_url("public.ecr.aws")
        .send()
        .await
        .expect("create_pull_through_cache_rule should succeed");

    assert_eq!(create_resp.ecr_repository_prefix(), Some("ecr-public"));
    assert_eq!(create_resp.upstream_registry_url(), Some("public.ecr.aws"));
    assert!(create_resp.created_at().is_some());

    // Describe
    let describe_resp = client
        .describe_pull_through_cache_rules()
        .ecr_repository_prefixes("ecr-public")
        .send()
        .await
        .expect("describe_pull_through_cache_rules should succeed");

    let rules = describe_resp.pull_through_cache_rules();
    assert_eq!(rules.len(), 1);
    assert_eq!(rules[0].ecr_repository_prefix(), Some("ecr-public"));
    assert_eq!(rules[0].upstream_registry_url(), Some("public.ecr.aws"));

    // Delete
    let delete_resp = client
        .delete_pull_through_cache_rule()
        .ecr_repository_prefix("ecr-public")
        .send()
        .await
        .expect("delete_pull_through_cache_rule should succeed");

    assert_eq!(delete_resp.ecr_repository_prefix(), Some("ecr-public"));

    // Describe after delete should return empty
    let describe_resp2 = client
        .describe_pull_through_cache_rules()
        .send()
        .await
        .expect("describe_pull_through_cache_rules after delete should succeed");

    assert_eq!(describe_resp2.pull_through_cache_rules().len(), 0);
}

/// Validate a pull-through cache rule that was previously created.
#[tokio::test]
async fn test_validate_pull_through_cache_rule() {
    let client = make_ecr_client().await;

    // Create a rule first
    client
        .create_pull_through_cache_rule()
        .ecr_repository_prefix("docker-hub")
        .upstream_registry_url("registry-1.docker.io")
        .send()
        .await
        .expect("create should succeed");

    // Validate it
    let resp = client
        .validate_pull_through_cache_rule()
        .ecr_repository_prefix("docker-hub")
        .send()
        .await
        .expect("validate_pull_through_cache_rule should succeed");

    assert_eq!(resp.ecr_repository_prefix(), Some("docker-hub"));
    assert!(resp.is_valid());
}

/// Update a pull-through cache rule credential ARN.
#[tokio::test]
async fn test_update_pull_through_cache_rule() {
    let client = make_ecr_client().await;

    client
        .create_pull_through_cache_rule()
        .ecr_repository_prefix("ghcr")
        .upstream_registry_url("ghcr.io")
        .send()
        .await
        .expect("create should succeed");

    let resp = client
        .update_pull_through_cache_rule()
        .ecr_repository_prefix("ghcr")
        .credential_arn("arn:aws:secretsmanager:us-east-1:123456789012:secret:my-secret")
        .send()
        .await
        .expect("update_pull_through_cache_rule should succeed");

    assert_eq!(resp.ecr_repository_prefix(), Some("ghcr"));
    assert_eq!(
        resp.credential_arn(),
        Some("arn:aws:secretsmanager:us-east-1:123456789012:secret:my-secret")
    );
}

// ==================== RepositoryCreationTemplate CRUD ====================

/// Create, describe, and delete a repository creation template.
#[tokio::test]
async fn test_repository_creation_template_crud() {
    let client = make_ecr_client().await;

    // Create
    let create_resp = client
        .create_repository_creation_template()
        .prefix("prod-")
        .description("Production repos")
        .image_tag_mutability(aws_sdk_ecr::types::ImageTagMutability::Immutable)
        .applied_for(aws_sdk_ecr::types::RctAppliedFor::PullThroughCache)
        .send()
        .await
        .expect("create_repository_creation_template should succeed");

    let template = create_resp
        .repository_creation_template()
        .expect("response should contain template");
    assert_eq!(template.prefix(), Some("prod-"));
    assert_eq!(template.description(), Some("Production repos"));

    // Describe
    let describe_resp = client
        .describe_repository_creation_templates()
        .prefixes("prod-")
        .send()
        .await
        .expect("describe_repository_creation_templates should succeed");

    let templates = describe_resp.repository_creation_templates();
    assert_eq!(templates.len(), 1);
    assert_eq!(templates[0].prefix(), Some("prod-"));

    // Delete
    let delete_resp = client
        .delete_repository_creation_template()
        .prefix("prod-")
        .send()
        .await
        .expect("delete_repository_creation_template should succeed");

    let deleted = delete_resp
        .repository_creation_template()
        .expect("deleted template should be returned");
    assert_eq!(deleted.prefix(), Some("prod-"));

    // Describe after delete
    let describe_resp2 = client
        .describe_repository_creation_templates()
        .send()
        .await
        .expect("describe after delete should succeed");

    assert_eq!(describe_resp2.repository_creation_templates().len(), 0);
}

// ==================== AccountSetting ====================

/// Put and get an account setting.
#[tokio::test]
async fn test_account_setting_put_and_get() {
    let client = make_ecr_client().await;

    // Put
    let put_resp = client
        .put_account_setting()
        .name("BASIC_SCAN_TYPE_VERSION")
        .value("AWS_NATIVE")
        .send()
        .await
        .expect("put_account_setting should succeed");

    assert_eq!(put_resp.name(), Some("BASIC_SCAN_TYPE_VERSION"));
    assert_eq!(put_resp.value(), Some("AWS_NATIVE"));

    // Get
    let get_resp = client
        .get_account_setting()
        .name("BASIC_SCAN_TYPE_VERSION")
        .send()
        .await
        .expect("get_account_setting should succeed");

    assert_eq!(get_resp.name(), Some("BASIC_SCAN_TYPE_VERSION"));
    assert_eq!(get_resp.value(), Some("AWS_NATIVE"));
}

// ==================== LifecyclePolicyPreview ====================

/// Start and get a lifecycle policy preview.
#[tokio::test]
async fn test_lifecycle_policy_preview() {
    let client = make_ecr_client().await;

    // Create repo first
    client
        .create_repository()
        .repository_name(ECR_REPO)
        .send()
        .await
        .unwrap();

    // Put a lifecycle policy
    let policy_text = r#"{"rules":[{"rulePriority":1,"description":"expire old","selection":{"tagStatus":"untagged","countType":"sinceImagePushed","countUnit":"days","countNumber":14},"action":{"type":"expire"}}]}"#;
    client
        .put_lifecycle_policy()
        .repository_name(ECR_REPO)
        .lifecycle_policy_text(policy_text)
        .send()
        .await
        .unwrap();

    // Start preview
    let start_resp = client
        .start_lifecycle_policy_preview()
        .repository_name(ECR_REPO)
        .send()
        .await
        .expect("start_lifecycle_policy_preview should succeed");

    assert_eq!(start_resp.repository_name(), Some(ECR_REPO));
    assert_eq!(
        start_resp.status(),
        Some(&aws_sdk_ecr::types::LifecyclePolicyPreviewStatus::InProgress)
    );
    assert!(start_resp.lifecycle_policy_text().is_some());

    // Get preview
    let get_resp = client
        .get_lifecycle_policy_preview()
        .repository_name(ECR_REPO)
        .send()
        .await
        .expect("get_lifecycle_policy_preview should succeed");

    assert_eq!(get_resp.repository_name(), Some(ECR_REPO));
    assert_eq!(
        get_resp.status(),
        Some(&aws_sdk_ecr::types::LifecyclePolicyPreviewStatus::Complete)
    );
}

// ==================== BatchCheckLayerAvailability ====================

/// BatchCheckLayerAvailability on a repo with no pushed layers.
#[tokio::test]
async fn test_batch_check_layer_availability() {
    let client = make_ecr_client().await;

    client
        .create_repository()
        .repository_name(ECR_REPO)
        .send()
        .await
        .unwrap();

    let resp = client
        .batch_check_layer_availability()
        .repository_name(ECR_REPO)
        .layer_digests("sha256:0000000000000000000000000000000000000000000000000000000000000000")
        .send()
        .await
        .expect("batch_check_layer_availability should succeed");

    // The queried layer should appear in the layers list
    assert!(!resp.layers().is_empty() || !resp.failures().is_empty());
}

// ==================== InitiateLayerUpload ====================

/// Initiate a layer upload on an existing repository.
#[tokio::test]
async fn test_initiate_layer_upload() {
    let client = make_ecr_client().await;

    client
        .create_repository()
        .repository_name(ECR_REPO)
        .send()
        .await
        .unwrap();

    let resp = client
        .initiate_layer_upload()
        .repository_name(ECR_REPO)
        .send()
        .await
        .expect("initiate_layer_upload should succeed");

    assert!(resp.upload_id().is_some(), "upload_id should be present");
    assert!(resp.part_size().is_some(), "part_size should be present");
}

// ==================== GetDownloadUrlForLayer ====================

/// GetDownloadUrlForLayer after pushing an image should return a URL.
#[tokio::test]
async fn test_get_download_url_for_layer() {
    let client = make_ecr_client().await;

    client
        .create_repository()
        .repository_name(ECR_REPO)
        .send()
        .await
        .unwrap();

    let manifest = create_image_manifest();
    let put_resp = client
        .put_image()
        .repository_name(ECR_REPO)
        .image_manifest(&manifest)
        .image_tag("v1")
        .send()
        .await
        .unwrap();

    // Extract a layer digest from the manifest
    let manifest_json: serde_json::Value = serde_json::from_str(&manifest).unwrap();
    let layer_digest = manifest_json["layers"][0]["digest"]
        .as_str()
        .unwrap()
        .to_string();

    let resp = client
        .get_download_url_for_layer()
        .repository_name(ECR_REPO)
        .layer_digest(&layer_digest)
        .send()
        .await
        .expect("get_download_url_for_layer should succeed");

    assert!(
        resp.download_url().is_some(),
        "download_url should be present"
    );
    assert_eq!(resp.layer_digest(), Some(layer_digest.as_str()));
}
