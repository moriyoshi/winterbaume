# winterbaume-shield

Shield service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Shield |
| AWS model | `shield` |
| Protocol | awsJson1.1 |
| winterbaume coverage | 9/36 operations (25.0%) |
| stubs (routed, returns empty/default) | 0/36 operations (0.0%) |
| moto coverage | 9/36 operations (25.0%) |
| floci coverage | 0/36 operations (0.0%) |
| kumo coverage | 0/36 operations (0.0%) |
| fakecloud coverage | 0/36 operations (0.0%) |
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
aws shield list-protections
```

## Example

```rust
use aws_sdk_shield::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_shield::ShieldService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(ShieldService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_shield::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_shield::Client::new(&config);

    let resp = client
        .list_protections()
        .send()
        .await
        .expect("list_protections should succeed");
    println!("Shield protections: {}", resp.protections().len());
}
```

## Implemented APIs (9)

- `CreateProtection`
- `CreateSubscription`
- `DeleteProtection`
- `DescribeProtection`
- `DescribeSubscription`
- `ListProtections`
- `ListTagsForResource`
- `TagResource`
- `UntagResource`

<details><summary>Not yet implemented APIs (27)</summary>

- `AssociateDRTLogBucket`
- `AssociateDRTRole`
- `AssociateHealthCheck`
- `AssociateProactiveEngagementDetails`
- `CreateProtectionGroup`
- `DeleteProtectionGroup`
- `DeleteSubscription`
- `DescribeAttack`
- `DescribeAttackStatistics`
- `DescribeDRTAccess`
- `DescribeEmergencyContactSettings`
- `DescribeProtectionGroup`
- `DisableApplicationLayerAutomaticResponse`
- `DisableProactiveEngagement`
- `DisassociateDRTLogBucket`
- `DisassociateDRTRole`
- `DisassociateHealthCheck`
- `EnableApplicationLayerAutomaticResponse`
- `EnableProactiveEngagement`
- `GetSubscriptionState`
- `ListAttacks`
- `ListProtectionGroups`
- `ListResourcesInProtectionGroup`
- `UpdateApplicationLayerAutomaticResponse`
- `UpdateEmergencyContactSettings`
- `UpdateProtectionGroup`
- `UpdateSubscription`

</details>
