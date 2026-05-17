# winterbaume-support

AWS Support service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Support |
| AWS model | `support` |
| Protocol | awsJson1.1 |
| winterbaume coverage | 5/16 operations (31.2%) |
| stubs (routed, returns empty/default) | 1/16 operations (6.2%) |
| moto coverage | 5/16 operations (31.2%) |
| floci coverage | 0/16 operations (0.0%) |
| kumo coverage | 0/16 operations (0.0%) |
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
aws support describe-services
```

## Example

```rust
use aws_sdk_support::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_support::SupportService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(SupportService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_support::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_support::Client::new(&config);

    let resp = client
        .describe_services()
        .send()
        .await
        .expect("describe_services should succeed");
    println!("Support services: {}", resp.services().len());
}
```

## Implemented APIs (5)

- `CreateCase`
- `DescribeCases`
- `DescribeServices`
- `RefreshTrustedAdvisorCheck`
- `ResolveCase`

<details><summary>Stubbed APIs (1) &mdash; routed but return an empty/default response</summary>

- `DescribeTrustedAdvisorChecks`

</details>

<details><summary>Not yet implemented APIs (10)</summary>

- `AddAttachmentsToSet`
- `AddCommunicationToCase`
- `DescribeAttachment`
- `DescribeCommunications`
- `DescribeCreateCaseOptions`
- `DescribeSeverityLevels`
- `DescribeSupportedLanguages`
- `DescribeTrustedAdvisorCheckRefreshStatuses`
- `DescribeTrustedAdvisorCheckResult`
- `DescribeTrustedAdvisorCheckSummaries`

</details>
