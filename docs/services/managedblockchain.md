# winterbaume-managedblockchain

Amazon Managed Blockchain service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Managed Blockchain |
| AWS model | `managedblockchain` |
| Protocol | restJson1 |
| winterbaume coverage | 27/27 operations (100.0%) |
| stubs (routed, returns empty/default) | 0/27 operations (0.0%) |
| moto coverage | 20/27 operations (74.1%) |
| floci coverage | 0/27 operations (0.0%) |
| kumo coverage | 0/27 operations (0.0%) |
| Coverage report date | 2026-05-17 |

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
aws managedblockchain list-networks
```

## Example

```rust
use aws_sdk_managedblockchain::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_managedblockchain::ManagedBlockchainService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(ManagedBlockchainService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_managedblockchain::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_managedblockchain::Client::new(&config);

    let resp = client
        .list_networks()
        .send()
        .await
        .expect("list_networks should succeed");
    println!("Managed Blockchain networks: {}", resp.networks().len());
}
```

## Implemented APIs (27)

- `CreateAccessor`
- `CreateMember`
- `CreateNetwork`
- `CreateNode`
- `CreateProposal`
- `DeleteAccessor`
- `DeleteMember`
- `DeleteNode`
- `GetAccessor`
- `GetMember`
- `GetNetwork`
- `GetNode`
- `GetProposal`
- `ListAccessors`
- `ListInvitations`
- `ListMembers`
- `ListNetworks`
- `ListNodes`
- `ListProposalVotes`
- `ListProposals`
- `ListTagsForResource`
- `RejectInvitation`
- `TagResource`
- `UntagResource`
- `UpdateMember`
- `UpdateNode`
- `VoteOnProposal`
