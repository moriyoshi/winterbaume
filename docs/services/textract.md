# winterbaume-textract

Amazon Textract service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Textract |
| AWS model | `textract` |
| Protocol | awsJson1.1 |
| winterbaume coverage | 6/25 operations (24.0%) |
| stubs (routed, returns empty/default) | 0/25 operations (0.0%) |
| moto coverage | 5/25 operations (20.0%) |
| floci coverage | 0/25 operations (0.0%) |
| kumo coverage | 0/25 operations (0.0%) |
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
aws textract list-adapters
```

## Example

```rust
use aws_sdk_textract::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_textract::TextractService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(TextractService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_textract::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_textract::Client::new(&config);

    let resp = client
        .list_adapters()
        .send()
        .await
        .expect("list_adapters should succeed");
    println!("Textract adapters: {}", resp.adapters().len());
}
```

## Implemented APIs (6)

- `AnalyzeDocument`
- `DetectDocumentText`
- `GetDocumentAnalysis`
- `GetDocumentTextDetection`
- `StartDocumentAnalysis`
- `StartDocumentTextDetection`

<details><summary>Not yet implemented APIs (19)</summary>

- `AnalyzeExpense`
- `AnalyzeID`
- `CreateAdapter`
- `CreateAdapterVersion`
- `DeleteAdapter`
- `DeleteAdapterVersion`
- `GetAdapter`
- `GetAdapterVersion`
- `GetExpenseAnalysis`
- `GetLendingAnalysis`
- `GetLendingAnalysisSummary`
- `ListAdapterVersions`
- `ListAdapters`
- `ListTagsForResource`
- `StartExpenseAnalysis`
- `StartLendingAnalysis`
- `TagResource`
- `UntagResource`
- `UpdateAdapter`

</details>
