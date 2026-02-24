# winterbaume-cloudsearchdomain

AWS CloudSearch Domain service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | CloudSearch Domain |
| AWS model | `cloudsearch-domain` |
| Protocol | restJson1 |
| winterbaume coverage | 2/3 operations (66.7%) |
| stubs (routed, returns empty/default) | 0/3 operations (0.0%) |
| moto coverage | 0/3 operations (0.0%) |
| floci coverage | 0/3 operations (0.0%) |
| kumo coverage | 0/3 operations (0.0%) |
| Coverage report date | 2026-05-06 |

## Server-mode usage

Start `winterbaume-server` and point the AWS CLI at it:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws cloudsearchdomain help
```

## Example

```rust
use aws_sdk_cloudsearchdomain::config::BehaviorVersion;
use aws_sdk_cloudsearchdomain::primitives::ByteStream;
use winterbaume_cloudsearchdomain::CloudSearchDomainService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(CloudSearchDomainService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_cloudsearchdomain::config::Region::new("us-east-1"))
        .endpoint_url("https://search-demo-abc.us-east-1.cloudsearch.amazonaws.com")
        .load()
        .await;

    let client = aws_sdk_cloudsearchdomain::Client::new(&config);
    client
        .upload_documents()
        .content_type(aws_sdk_cloudsearchdomain::types::ContentType::ApplicationJson)
        .documents(ByteStream::from_static(
            br#"[{"type":"add","id":"1","fields":{"title":"Hello"}}]"#,
        ))
        .send()
        .await
        .expect("upload should succeed");
    let resp = client
        .search()
        .query("Hello")
        .send()
        .await
        .expect("search should succeed");
    println!("Hits: {}", resp.hits().map(|h| h.found()).unwrap_or(0));
}
```

## Implemented APIs (2)

- `Search`
- `Suggest`

<details><summary>Not yet implemented APIs (1)</summary>

- `UploadDocuments`

</details>
