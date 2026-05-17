# winterbaume-pipes

EventBridge Pipes service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | EventBridge Pipes |
| AWS model | `pipes` |
| Protocol | restJson1 |
| winterbaume coverage | 10/10 operations (100.0%) |
| stubs (routed, returns empty/default) | 0/10 operations (0.0%) |
| moto coverage | 9/10 operations (90.0%) |
| floci coverage | 0/10 operations (0.0%) |
| kumo coverage | 10/10 operations (100.0%) |
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
aws pipes list-pipes
```

## Example

```rust
use aws_sdk_pipes::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_pipes::PipesService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder().with_service(PipesService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_pipes::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_pipes::Client::new(&config);

    let resp = client
        .list_pipes()
        .send()
        .await
        .expect("list_pipes should succeed");
    println!("EventBridge Pipes: {}", resp.pipes().len());
}
```

## Implemented APIs (10)

- `CreatePipe`
- `DeletePipe`
- `DescribePipe`
- `ListPipes`
- `ListTagsForResource`
- `StartPipe`
- `StopPipe`
- `TagResource`
- `UntagResource`
- `UpdatePipe`
