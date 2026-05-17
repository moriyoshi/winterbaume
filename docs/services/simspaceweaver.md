# winterbaume-simspaceweaver

AWS SimSpace Weaver service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | SimSpace Weaver |
| AWS model | `simspaceweaver` |
| Protocol | restJson1 |
| winterbaume coverage | 15/16 operations (93.8%) |
| stubs (routed, returns empty/default) | 0/16 operations (0.0%) |
| moto coverage | 0/16 operations (0.0%) |
| floci coverage | 0/16 operations (0.0%) |
| kumo coverage | 0/16 operations (0.0%) |
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
aws simspaceweaver help
```

## Example

```rust
use aws_sdk_simspaceweaver::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_simspaceweaver::SimSpaceWeaverService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(SimSpaceWeaverService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_simspaceweaver::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_simspaceweaver::Client::new(&config);
    client
        .start_simulation()
        .name("demo")
        .role_arn("arn:aws:iam::123:role/SimRole")
        .send()
        .await
        .expect("start_simulation should succeed");
    let resp = client
        .list_simulations()
        .send()
        .await
        .expect("list_simulations should succeed");
    for s in resp.simulations() {
        println!("Simulation: {:?}", s.name());
    }
}
```

## Implemented APIs (15)

- `CreateSnapshot`
- `DeleteApp`
- `DeleteSimulation`
- `DescribeApp`
- `DescribeSimulation`
- `ListApps`
- `ListSimulations`
- `StartApp`
- `StartClock`
- `StartSimulation`
- `StopApp`
- `StopClock`
- `StopSimulation`
- `TagResource`
- `UntagResource`

<details><summary>Not yet implemented APIs (1)</summary>

- `ListTagsForResource`

</details>
