# winterbaume-ecr

ECR service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | ECR |
| AWS model | `ecr` |
| Protocol | awsJson1.1 |
| winterbaume coverage | 58/58 operations (100.0%) |
| stubs (routed, returns empty/default) | 0/58 operations (0.0%) |
| moto coverage | 29/58 operations (50.0%) |
| floci coverage | 0/58 operations (0.0%) |
| kumo coverage | 11/58 operations (19.0%) |
| Coverage report date | 2026-05-17 |

## Server-mode usage

Install `winterbaume-server` from crates.io or run it from a workspace checkout, then point the AWS CLI at it:

```sh
# Installed binary ( from crates.io ):
cargo install winterbaume-server
winterbaume-server --host 127.0.0.1 --port 5555

# Or, from a workspace checkout:
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws ecr describe-repositories
```

## Example

```rust
use aws_sdk_ecr::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_ecr::EcrService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder().with_service(EcrService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_ecr::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_ecr::Client::new(&config);

    let resp = client
        .describe_repositories()
        .send()
        .await
        .expect("describe_repositories should succeed");
    println!("ECR repositories: {}", resp.repositories().len());
}
```

## Implemented APIs (58)

- `BatchCheckLayerAvailability`
- `BatchDeleteImage`
- `BatchGetImage`
- `BatchGetRepositoryScanningConfiguration`
- `CompleteLayerUpload`
- `CreatePullThroughCacheRule`
- `CreateRepository`
- `CreateRepositoryCreationTemplate`
- `DeleteLifecyclePolicy`
- `DeletePullThroughCacheRule`
- `DeleteRegistryPolicy`
- `DeleteRepository`
- `DeleteRepositoryCreationTemplate`
- `DeleteRepositoryPolicy`
- `DeleteSigningConfiguration`
- `DeregisterPullTimeUpdateExclusion`
- `DescribeImageReplicationStatus`
- `DescribeImageScanFindings`
- `DescribeImageSigningStatus`
- `DescribeImages`
- `DescribePullThroughCacheRules`
- `DescribeRegistry`
- `DescribeRepositories`
- `DescribeRepositoryCreationTemplates`
- `GetAccountSetting`
- `GetAuthorizationToken`
- `GetDownloadUrlForLayer`
- `GetLifecyclePolicy`
- `GetLifecyclePolicyPreview`
- `GetRegistryPolicy`
- `GetRegistryScanningConfiguration`
- `GetRepositoryPolicy`
- `GetSigningConfiguration`
- `InitiateLayerUpload`
- `ListImageReferrers`
- `ListImages`
- `ListPullTimeUpdateExclusions`
- `ListTagsForResource`
- `PutAccountSetting`
- `PutImage`
- `PutImageScanningConfiguration`
- `PutImageTagMutability`
- `PutLifecyclePolicy`
- `PutRegistryPolicy`
- `PutRegistryScanningConfiguration`
- `PutReplicationConfiguration`
- `PutSigningConfiguration`
- `RegisterPullTimeUpdateExclusion`
- `SetRepositoryPolicy`
- `StartImageScan`
- `StartLifecyclePolicyPreview`
- `TagResource`
- `UntagResource`
- `UpdateImageStorageClass`
- `UpdatePullThroughCacheRule`
- `UpdateRepositoryCreationTemplate`
- `UploadLayerPart`
- `ValidatePullThroughCacheRule`
