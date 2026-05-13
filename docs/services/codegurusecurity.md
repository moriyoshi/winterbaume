# winterbaume-codegurusecurity

AWS CodeGuru Security service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | CodeGuru Security |
| AWS model | `codeguru-security` |
| Protocol | restJson1 |
| winterbaume coverage | 11/13 operations (84.6%) |
| stubs (routed, returns empty/default) | 0/13 operations (0.0%) |
| moto coverage | 0/13 operations (0.0%) |
| floci coverage | 0/13 operations (0.0%) |
| kumo coverage | 0/13 operations (0.0%) |
| Coverage report date | 2026-05-13 |

## Server-mode usage

Start `winterbaume-server` and point the AWS CLI at it:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws codegurusecurity help
```

## Example

```rust
use aws_sdk_codegurusecurity::config::BehaviorVersion;
use aws_sdk_codegurusecurity::types::{AnalysisType, ResourceId, ScanType};
use winterbaume_codegurusecurity::CodeGuruSecurityService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(CodeGuruSecurityService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_codegurusecurity::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_codegurusecurity::Client::new(&config);
    let resp = client
        .create_scan()
        .scan_name("demo-scan")
        .resource_id(ResourceId::CodeArtifactId("artifact-001".to_string()))
        .scan_type(ScanType::Standard)
        .analysis_type(AnalysisType::Security)
        .send()
        .await
        .expect("create_scan");
    if let Some(arn) = resp.scan_name_arn() {
        println!("Scan: {arn}");
    }
}
```

## Implemented APIs (11)

- `BatchGetFindings`
- `CreateScan`
- `GetAccountConfiguration`
- `GetFindings`
- `GetMetricsSummary`
- `GetScan`
- `ListFindingsMetrics`
- `ListScans`
- `TagResource`
- `UntagResource`
- `UpdateAccountConfiguration`

<details><summary>Not yet implemented APIs (2)</summary>

- `CreateUploadUrl`
- `ListTagsForResource`

</details>
