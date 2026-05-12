# winterbaume-workspacesweb

Amazon WorkSpaces Web service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | WorkSpaces Web |
| AWS model | `workspaces-web` |
| Protocol | restJson1 |
| winterbaume coverage | 68/75 operations (90.7%) |
| stubs (routed, returns empty/default) | 0/75 operations (0.0%) |
| moto coverage | 27/75 operations (36.0%) |
| floci coverage | 0/75 operations (0.0%) |
| kumo coverage | 0/75 operations (0.0%) |
| Coverage report date | 2026-05-12 |

## Server-mode usage

Start `winterbaume-server` and point the AWS CLI at it:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws workspaces-web list-portals
```

## Current Network Resource Stub Semantics

WorkSpaces Web currently stores portal networking as portal/resource metadata.

- Portal and network settings shapes include VPC ID, subnet IDs, and security group IDs.
- Current implemented state records those values only where the surrounding resource handler persists the portal or settings object.
- Browser session lifecycle does not create VPC endpoints or ENIs.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Example

```rust
use aws_sdk_workspacesweb::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_workspacesweb::WorkspacesWebService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(WorkspacesWebService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_workspacesweb::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_workspacesweb::Client::new(&config);

    let resp = client
        .list_portals()
        .send()
        .await
        .expect("list_portals should succeed");
    println!("WorkSpaces Web portals: {}", resp.portals().len());
}
```

## Implemented APIs (68)

- `AssociateBrowserSettings`
- `AssociateDataProtectionSettings`
- `AssociateNetworkSettings`
- `AssociateSessionLogger`
- `AssociateTrustStore`
- `AssociateUserAccessLoggingSettings`
- `AssociateUserSettings`
- `CreateBrowserSettings`
- `CreateDataProtectionSettings`
- `CreateIdentityProvider`
- `CreateNetworkSettings`
- `CreatePortal`
- `CreateSessionLogger`
- `CreateTrustStore`
- `CreateUserAccessLoggingSettings`
- `CreateUserSettings`
- `DeleteBrowserSettings`
- `DeleteDataProtectionSettings`
- `DeleteIdentityProvider`
- `DeleteNetworkSettings`
- `DeletePortal`
- `DeleteSessionLogger`
- `DeleteTrustStore`
- `DeleteUserAccessLoggingSettings`
- `DeleteUserSettings`
- `DisassociateBrowserSettings`
- `DisassociateDataProtectionSettings`
- `DisassociateNetworkSettings`
- `DisassociateSessionLogger`
- `DisassociateTrustStore`
- `DisassociateUserAccessLoggingSettings`
- `DisassociateUserSettings`
- `ExpireSession`
- `GetBrowserSettings`
- `GetDataProtectionSettings`
- `GetIdentityProvider`
- `GetNetworkSettings`
- `GetPortal`
- `GetPortalServiceProviderMetadata`
- `GetSession`
- `GetSessionLogger`
- `GetTrustStore`
- `GetTrustStoreCertificate`
- `GetUserAccessLoggingSettings`
- `GetUserSettings`
- `ListBrowserSettings`
- `ListDataProtectionSettings`
- `ListIdentityProviders`
- `ListNetworkSettings`
- `ListPortals`
- `ListSessionLoggers`
- `ListSessions`
- `ListTagsForResource`
- `ListTrustStoreCertificates`
- `ListTrustStores`
- `ListUserAccessLoggingSettings`
- `ListUserSettings`
- `TagResource`
- `UntagResource`
- `UpdateBrowserSettings`
- `UpdateDataProtectionSettings`
- `UpdateIdentityProvider`
- `UpdateNetworkSettings`
- `UpdatePortal`
- `UpdateSessionLogger`
- `UpdateTrustStore`
- `UpdateUserAccessLoggingSettings`
- `UpdateUserSettings`

<details><summary>Not yet implemented APIs (7)</summary>

- `AssociateIpAccessSettings`
- `CreateIpAccessSettings`
- `DeleteIpAccessSettings`
- `DisassociateIpAccessSettings`
- `GetIpAccessSettings`
- `ListIpAccessSettings`
- `UpdateIpAccessSettings`

</details>
