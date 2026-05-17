# winterbaume-route53recoverycluster

AWS Route 53 Application Recovery Controller -- Cluster (data-plane) implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Route 53 Recovery Cluster |
| AWS model | `route53-recovery-cluster` |
| Protocol | awsJson1.0 |
| winterbaume coverage | 4/4 operations (100.0%) |
| stubs (routed, returns empty/default) | 0/4 operations (0.0%) |
| moto coverage | 0/4 operations (0.0%) |
| floci coverage | 0/4 operations (0.0%) |
| kumo coverage | 0/4 operations (0.0%) |
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
aws route53recoverycluster help
```

## Example

```rust
use aws_sdk_route53recoverycluster::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_route53recoverycluster::RecoveryClusterService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(RecoveryClusterService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_route53recoverycluster::config::Region::new(
            "us-east-1",
        ))
        .endpoint_url("https://route53-recovery-cluster.us-east-1.amazonaws.com")
        .load()
        .await;

    let client = aws_sdk_route53recoverycluster::Client::new(&config);
    let resp = client
        .list_routing_controls()
        .send()
        .await
        .expect("list_routing_controls should succeed");
    for rc in resp.routing_controls() {
        println!(
            "{}: {:?} -> {:?}",
            rc.routing_control_name().unwrap_or_default(),
            rc.routing_control_arn(),
            rc.routing_control_state()
        );
    }
}
```

## Implemented APIs (4)

- `GetRoutingControlState`
- `ListRoutingControls`
- `UpdateRoutingControlState`
- `UpdateRoutingControlStates`
