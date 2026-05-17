# winterbaume-resourcegroupstagging

Resource Groups Tagging API service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Resource Groups Tagging |
| AWS model | `resource-groups-tagging-api` |
| Protocol | awsJson1.1 |
| winterbaume coverage | 5/9 operations (55.6%) |
| stubs (routed, returns empty/default) | 0/9 operations (0.0%) |
| moto coverage | 0/9 operations (0.0%) |
| floci coverage | 0/9 operations (0.0%) |
| kumo coverage | 0/9 operations (0.0%) |
| Coverage report date | 2026-05-16 |

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
aws resourcegroupstaggingapi get-resources
```

## Example

```rust
use aws_sdk_resourcegroupstagging::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_resourcegroupstagging::TaggingService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(TaggingService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_resourcegroupstagging::config::Region::new(
            "us-east-1",
        ))
        .load()
        .await;

    let client = aws_sdk_resourcegroupstagging::Client::new(&config);

    let resp = client
        .get_resources()
        .send()
        .await
        .expect("get_resources should succeed");
    println!(
        "Tagged resources: {}",
        resp.resource_tag_mapping_list().len()
    );
}
```

## Implemented APIs (5)

- `GetResources`
- `GetTagKeys`
- `GetTagValues`
- `TagResources`
- `UntagResources`

<details><summary>Not yet implemented APIs (4)</summary>

- `DescribeReportCreation`
- `GetComplianceSummary`
- `ListRequiredTags`
- `StartReportCreation`

</details>
