# winterbaume-synthetics

CloudWatch Synthetics service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Synthetics |
| AWS model | `synthetics` |
| Protocol | restJson1 |
| winterbaume coverage | 22/22 operations (100.0%) |
| stubs (routed, returns empty/default) | 0/22 operations (0.0%) |
| moto coverage | 4/22 operations (18.2%) |
| floci coverage | 0/22 operations (0.0%) |
| kumo coverage | 0/22 operations (0.0%) |
| fakecloud coverage | 0/22 operations (0.0%) |
| Coverage report date | 2026-07-03 |

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
aws synthetics describe-canaries
```

## Current Network Resource Stub Semantics

Synthetics currently has a placeholder for canary VPC configuration.

- Canary views include an optional `vpc_config` JSON slot, and current snapshot construction sets it to `None`.
- Canary create/update/read paths do not allocate ENIs or retain subnet and security group metadata in the implemented state.
- Run status is independent of VPC reachability.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Example

```rust
use aws_sdk_synthetics::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_synthetics::SyntheticsService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(SyntheticsService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_synthetics::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_synthetics::Client::new(&config);

    let resp = client
        .describe_canaries()
        .send()
        .await
        .expect("describe_canaries should succeed");
    println!("Synthetics canaries: {}", resp.canaries().len());
}
```

## Implemented APIs (22)

- `AssociateResource`
- `CreateCanary`
- `CreateGroup`
- `DeleteCanary`
- `DeleteGroup`
- `DescribeCanaries`
- `DescribeCanariesLastRun`
- `DescribeRuntimeVersions`
- `DisassociateResource`
- `GetCanary`
- `GetCanaryRuns`
- `GetGroup`
- `ListAssociatedGroups`
- `ListGroupResources`
- `ListGroups`
- `ListTagsForResource`
- `StartCanary`
- `StartCanaryDryRun`
- `StopCanary`
- `TagResource`
- `UntagResource`
- `UpdateCanary`
