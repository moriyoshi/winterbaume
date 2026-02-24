# winterbaume-pcaconnectorscep

AWS Private CA Connector for SCEP service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Private CA Connector for SCEP |
| AWS model | `pca-connector-scep` |
| Protocol | restJson1 |
| winterbaume coverage | 11/12 operations (91.7%) |
| stubs (routed, returns empty/default) | 0/12 operations (0.0%) |
| moto coverage | 0/12 operations (0.0%) |
| floci coverage | 0/12 operations (0.0%) |
| kumo coverage | 0/12 operations (0.0%) |
| Coverage report date | 2026-05-06 |

## Server-mode usage

Start `winterbaume-server` and point the AWS CLI at it:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws pcaconnectorscep help
```

## Example

```rust
use aws_sdk_pcaconnectorscep::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_pcaconnectorscep::PcaConnectorScepService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(PcaConnectorScepService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_pcaconnectorscep::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_pcaconnectorscep::Client::new(&config);
    let create = client
        .create_connector()
        .certificate_authority_arn("arn:aws:acm-pca:us-east-1:123:certificate-authority/demo")
        .send()
        .await
        .expect("create_connector should succeed");
    println!("Created connector: {:?}", create.connector_arn());
}
```

## Implemented APIs (11)

- `CreateChallenge`
- `CreateConnector`
- `DeleteChallenge`
- `DeleteConnector`
- `GetChallengeMetadata`
- `GetChallengePassword`
- `GetConnector`
- `ListChallengeMetadata`
- `ListConnectors`
- `TagResource`
- `UntagResource`

<details><summary>Not yet implemented APIs (1)</summary>

- `ListTagsForResource`

</details>
