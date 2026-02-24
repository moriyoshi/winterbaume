# winterbaume-apigatewaymanagement

API Gateway Management API service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | API Gateway Management API |
| AWS model | `apigatewaymanagementapi` |
| Protocol | restJson1 |
| winterbaume coverage | 3/3 operations (100.0%) |
| stubs (routed, returns empty/default) | 0/3 operations (0.0%) |
| moto coverage | 3/3 operations (100.0%) |
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
aws apigatewaymanagementapi get-connection --connection-id dummy
```

## Example

```rust
use aws_sdk_apigatewaymanagement::config::BehaviorVersion;
use winterbaume_apigatewaymanagement::ApiGatewayManagementService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(ApiGatewayManagementService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_apigatewaymanagement::config::Region::new(
            "us-east-1",
        ))
        .load()
        .await;

    let client = aws_sdk_apigatewaymanagement::Client::new(&config);

    let resp = client
        .get_connection()
        .connection_id("example-conn-001")
        .send()
        .await
        .expect("get_connection should succeed");

    println!(
        "API Gateway Management API: connectedAt={:?}, identity={:?}",
        resp.connected_at(),
        resp.identity()
    );
}
```

## Implemented APIs (3)

- `DeleteConnection`
- `GetConnection`
- `PostToConnection`
