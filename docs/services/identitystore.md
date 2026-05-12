# winterbaume-identitystore

Identity Store service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Identity Store |
| AWS model | `identitystore` |
| Protocol | awsJson1.1 |
| winterbaume coverage | 17/19 operations (89.5%) |
| stubs (routed, returns empty/default) | 0/19 operations (0.0%) |
| moto coverage | 14/19 operations (73.7%) |
| floci coverage | 0/19 operations (0.0%) |
| kumo coverage | 0/19 operations (0.0%) |
| Coverage report date | 2026-05-12 |

## Server-mode usage

Start `winterbaume-server` and point the AWS CLI at it:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws identitystore list-users --identity-store-id d-1234567890
```

## Example

```rust
use aws_sdk_identitystore::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_identitystore::IdentityStoreService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(IdentityStoreService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_identitystore::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_identitystore::Client::new(&config);

    let resp = client
        .list_users()
        .identity_store_id("d-1234567890")
        .send()
        .await
        .expect("list_users should succeed");
    println!("Identity store users: {}", resp.users().len());
}
```

## Implemented APIs (17)

- `CreateGroup`
- `CreateGroupMembership`
- `CreateUser`
- `DeleteGroup`
- `DeleteGroupMembership`
- `DeleteUser`
- `DescribeGroup`
- `DescribeGroupMembership`
- `DescribeUser`
- `GetGroupId`
- `GetUserId`
- `ListGroupMemberships`
- `ListGroupMembershipsForMember`
- `ListGroups`
- `ListUsers`
- `UpdateGroup`
- `UpdateUser`

<details><summary>Not yet implemented APIs (2)</summary>

- `GetGroupMembershipId`
- `IsMemberInGroups`

</details>
