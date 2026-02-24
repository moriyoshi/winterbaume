# winterbaume-codeartifact

CodeArtifact service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | CodeArtifact |
| AWS model | `codeartifact` |
| Protocol | restJson1 |
| winterbaume coverage | 9/48 operations (18.8%) |
| stubs (routed, returns empty/default) | 0/48 operations (0.0%) |
| moto coverage | 0/48 operations (0.0%) |
| floci coverage | 0/48 operations (0.0%) |
| kumo coverage | 0/48 operations (0.0%) |
| Coverage report date | 2026-05-06 |

## Server-mode usage

Start `winterbaume-server` and point the AWS CLI at it:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws codeartifact help
```

## Example

```rust
use aws_sdk_codeartifact::config::BehaviorVersion;
use winterbaume_codeartifact::CodeArtifactService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(CodeArtifactService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_codeartifact::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_codeartifact::Client::new(&config);

    let resp = client
        .list_domains()
        .send()
        .await
        .expect("list_domains should succeed");
    println!("CodeArtifact domains: {:?}", resp.domains());
}
```

## Implemented APIs (9)

- `CreateDomain`
- `CreateRepository`
- `DeleteDomain`
- `DeleteRepository`
- `DescribeDomain`
- `DescribeRepository`
- `ListDomains`
- `ListRepositories`
- `ListRepositoriesInDomain`

<details><summary>Not yet implemented APIs (39)</summary>

- `AssociateExternalConnection`
- `CopyPackageVersions`
- `CreatePackageGroup`
- `DeleteDomainPermissionsPolicy`
- `DeletePackage`
- `DeletePackageGroup`
- `DeletePackageVersions`
- `DeleteRepositoryPermissionsPolicy`
- `DescribePackage`
- `DescribePackageGroup`
- `DescribePackageVersion`
- `DisassociateExternalConnection`
- `DisposePackageVersions`
- `GetAssociatedPackageGroup`
- `GetAuthorizationToken`
- `GetDomainPermissionsPolicy`
- `GetPackageVersionAsset`
- `GetPackageVersionReadme`
- `GetRepositoryEndpoint`
- `GetRepositoryPermissionsPolicy`
- `ListAllowedRepositoriesForGroup`
- `ListAssociatedPackages`
- `ListPackageGroups`
- `ListPackageVersionAssets`
- `ListPackageVersionDependencies`
- `ListPackageVersions`
- `ListPackages`
- `ListSubPackageGroups`
- `ListTagsForResource`
- `PublishPackageVersion`
- `PutDomainPermissionsPolicy`
- `PutPackageOriginConfiguration`
- `PutRepositoryPermissionsPolicy`
- `TagResource`
- `UntagResource`
- `UpdatePackageGroup`
- `UpdatePackageGroupOriginConfiguration`
- `UpdatePackageVersionsStatus`
- `UpdateRepository`

</details>
