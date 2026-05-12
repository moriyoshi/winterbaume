# winterbaume-rolesanywhere

IAM Roles Anywhere service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | IAM Roles Anywhere |
| AWS model | `rolesanywhere` |
| Protocol | restJson1 |
| winterbaume coverage | 28/30 operations (93.3%) |
| stubs (routed, returns empty/default) | 2/30 operations (6.7%) |
| moto coverage | 0/30 operations (0.0%) |
| floci coverage | 0/30 operations (0.0%) |
| kumo coverage | 0/30 operations (0.0%) |
| Coverage report date | 2026-05-12 |

## Server-mode usage

Start `winterbaume-server` and point the AWS CLI at it:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws rolesanywhere help
```

## Example

```rust
use aws_sdk_rolesanywhere::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_rolesanywhere::RolesAnywhereService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(RolesAnywhereService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_rolesanywhere::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_rolesanywhere::Client::new(&config);

    let resp = client
        .list_profiles()
        .send()
        .await
        .expect("list_profiles should succeed");
    println!("Roles Anywhere profiles: {:?}", resp.profiles());
}
```

## Implemented APIs (28)

- `CreateProfile`
- `CreateTrustAnchor`
- `DeleteAttributeMapping`
- `DeleteCrl`
- `DeleteProfile`
- `DeleteTrustAnchor`
- `DisableCrl`
- `DisableProfile`
- `DisableTrustAnchor`
- `EnableCrl`
- `EnableProfile`
- `EnableTrustAnchor`
- `GetCrl`
- `GetProfile`
- `GetTrustAnchor`
- `ImportCrl`
- `ListCrls`
- `ListProfiles`
- `ListTagsForResource`
- `ListTrustAnchors`
- `PutAttributeMapping`
- `PutNotificationSettings`
- `ResetNotificationSettings`
- `TagResource`
- `UntagResource`
- `UpdateCrl`
- `UpdateProfile`
- `UpdateTrustAnchor`

<details><summary>Stubbed APIs (2) &mdash; routed but return an empty/default response</summary>

- `GetSubject`
- `ListSubjects`

</details>
