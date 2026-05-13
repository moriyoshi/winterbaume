# winterbaume-accessanalyzer

IAM Access Analyzer service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | IAM Access Analyzer |
| AWS model | `accessanalyzer` |
| Protocol | restJson1 |
| winterbaume coverage | 11/37 operations (29.7%) |
| stubs (routed, returns empty/default) | 0/37 operations (0.0%) |
| moto coverage | 0/37 operations (0.0%) |
| floci coverage | 0/37 operations (0.0%) |
| kumo coverage | 0/37 operations (0.0%) |
| Coverage report date | 2026-05-13 |

## Server-mode usage

Start `winterbaume-server` and point the AWS CLI at it:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws accessanalyzer help
```

## Example

```rust
use aws_sdk_accessanalyzer::config::BehaviorVersion;
use winterbaume_accessanalyzer::AccessAnalyzerService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(AccessAnalyzerService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_accessanalyzer::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_accessanalyzer::Client::new(&config);

    let resp = client
        .list_analyzers()
        .send()
        .await
        .expect("list_analyzers should succeed");
    println!("Access Analyzer: {:?}", resp.analyzers());
}
```

## Implemented APIs (11)

- `CreateAnalyzer`
- `CreateArchiveRule`
- `DeleteAnalyzer`
- `DeleteArchiveRule`
- `GetAnalyzer`
- `GetArchiveRule`
- `ListAnalyzers`
- `ListArchiveRules`
- `ListTagsForResource`
- `TagResource`
- `UntagResource`

<details><summary>Not yet implemented APIs (26)</summary>

- `ApplyArchiveRule`
- `CancelPolicyGeneration`
- `CheckAccessNotGranted`
- `CheckNoNewAccess`
- `CheckNoPublicAccess`
- `CreateAccessPreview`
- `GenerateFindingRecommendation`
- `GetAccessPreview`
- `GetAnalyzedResource`
- `GetFinding`
- `GetFindingRecommendation`
- `GetFindingV2`
- `GetFindingsStatistics`
- `GetGeneratedPolicy`
- `ListAccessPreviewFindings`
- `ListAccessPreviews`
- `ListAnalyzedResources`
- `ListFindings`
- `ListFindingsV2`
- `ListPolicyGenerations`
- `StartPolicyGeneration`
- `StartResourceScan`
- `UpdateAnalyzer`
- `UpdateArchiveRule`
- `UpdateFindings`
- `ValidatePolicy`

</details>
