# winterbaume-ram

Resource Access Manager service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | RAM |
| AWS model | `ram` |
| Protocol | restJson1 |
| winterbaume coverage | 35/35 operations (100.0%) |
| stubs (routed, returns empty/default) | 0/35 operations (0.0%) |
| moto coverage | 8/35 operations (22.9%) |
| floci coverage | 0/35 operations (0.0%) |
| kumo coverage | 0/35 operations (0.0%) |
| Coverage report date | 2026-05-13 |

## Server-mode usage

Start `winterbaume-server` and point the AWS CLI at it:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws ram list-resources --resource-owner SELF
```

## Current Network Resource Stub Semantics

RAM currently recognises EC2 subnet sharing as permission metadata only.

- The default permission catalogue includes an `ec2:Subnet` resource type and subnet permission names.
- Resource shares store resource ARNs as opaque strings, so a shared subnet ARN is not resolved against EC2 subnet state.
- RAM association, invitation, and permission logic does not alter EC2 resource visibility in other service crates.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Example

```rust
use aws_sdk_ram::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_ram::RamService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder().with_service(RamService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_ram::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_ram::Client::new(&config);

    use aws_sdk_ram::types::ResourceOwner;
    let resp = client
        .list_resources()
        .resource_owner(ResourceOwner::SelfValue)
        .send()
        .await
        .expect("list_resources should succeed");
    println!("RAM resources: {}", resp.resources().len());
}
```

## Implemented APIs (35)

- `AcceptResourceShareInvitation`
- `AssociateResourceShare`
- `AssociateResourceSharePermission`
- `CreatePermission`
- `CreatePermissionVersion`
- `CreateResourceShare`
- `DeletePermission`
- `DeletePermissionVersion`
- `DeleteResourceShare`
- `DisassociateResourceShare`
- `DisassociateResourceSharePermission`
- `EnableSharingWithAwsOrganization`
- `GetPermission`
- `GetResourcePolicies`
- `GetResourceShareAssociations`
- `GetResourceShareInvitations`
- `GetResourceShares`
- `ListPendingInvitationResources`
- `ListPermissionAssociations`
- `ListPermissionVersions`
- `ListPermissions`
- `ListPrincipals`
- `ListReplacePermissionAssociationsWork`
- `ListResourceSharePermissions`
- `ListResourceTypes`
- `ListResources`
- `ListSourceAssociations`
- `PromotePermissionCreatedFromPolicy`
- `PromoteResourceShareCreatedFromPolicy`
- `RejectResourceShareInvitation`
- `ReplacePermissionAssociations`
- `SetDefaultPermissionVersion`
- `TagResource`
- `UntagResource`
- `UpdateResourceShare`
