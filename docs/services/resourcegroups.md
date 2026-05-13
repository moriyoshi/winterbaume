# winterbaume-resourcegroups

Resource Groups service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Resource Groups |
| AWS model | `resource-groups` |
| Protocol | restJson1 |
| winterbaume coverage | 22/23 operations (95.7%) |
| stubs (routed, returns empty/default) | 1/23 operations (4.3%) |
| moto coverage | 15/23 operations (65.2%) |
| floci coverage | 0/23 operations (0.0%) |
| kumo coverage | 0/23 operations (0.0%) |
| Coverage report date | 2026-05-13 |

## Server-mode usage

Start `winterbaume-server` and point the AWS CLI at it:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws resource-groups list-groups
```

## Example

```rust
use aws_sdk_resourcegroups::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_resourcegroups::ResourceGroupsService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(ResourceGroupsService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_resourcegroups::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_resourcegroups::Client::new(&config);

    let resp = client
        .list_groups()
        .send()
        .await
        .expect("list_groups should succeed");
    println!("Resource groups: {}", resp.group_identifiers().len());
}
```

## Implemented APIs (22)

- `CancelTagSyncTask`
- `CreateGroup`
- `DeleteGroup`
- `GetAccountSettings`
- `GetGroup`
- `GetGroupConfiguration`
- `GetGroupQuery`
- `GetTagSyncTask`
- `GetTags`
- `GroupResources`
- `ListGroupResources`
- `ListGroupingStatuses`
- `ListGroups`
- `ListTagSyncTasks`
- `PutGroupConfiguration`
- `StartTagSyncTask`
- `Tag`
- `UngroupResources`
- `Untag`
- `UpdateAccountSettings`
- `UpdateGroup`
- `UpdateGroupQuery`

<details><summary>Stubbed APIs (1) &mdash; routed but return an empty/default response</summary>

- `SearchResources`

</details>
