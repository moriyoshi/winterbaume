# winterbaume-glacier

Glacier service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Glacier |
| AWS model | `glacier` |
| Protocol | restJson1 |
| winterbaume coverage | 33/33 operations (100.0%) |
| stubs (routed, returns empty/default) | 0/33 operations (0.0%) |
| moto coverage | 10/33 operations (30.3%) |
| floci coverage | 0/33 operations (0.0%) |
| kumo coverage | 4/33 operations (12.1%) |
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
aws glacier list-vaults --account-id -
```

## Example

```rust
use aws_sdk_glacier::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_glacier::GlacierService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(GlacierService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_glacier::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_glacier::Client::new(&config);

    let resp = client
        .list_vaults()
        .account_id("-")
        .send()
        .await
        .expect("list_vaults should succeed");
    println!("Glacier vaults: {}", resp.vault_list().len());
}
```

## Implemented APIs (33)

- `AbortMultipartUpload`
- `AbortVaultLock`
- `AddTagsToVault`
- `CompleteMultipartUpload`
- `CompleteVaultLock`
- `CreateVault`
- `DeleteArchive`
- `DeleteVault`
- `DeleteVaultAccessPolicy`
- `DeleteVaultNotifications`
- `DescribeJob`
- `DescribeVault`
- `GetDataRetrievalPolicy`
- `GetJobOutput`
- `GetVaultAccessPolicy`
- `GetVaultLock`
- `GetVaultNotifications`
- `InitiateJob`
- `InitiateMultipartUpload`
- `InitiateVaultLock`
- `ListJobs`
- `ListMultipartUploads`
- `ListParts`
- `ListProvisionedCapacity`
- `ListTagsForVault`
- `ListVaults`
- `PurchaseProvisionedCapacity`
- `RemoveTagsFromVault`
- `SetDataRetrievalPolicy`
- `SetVaultAccessPolicy`
- `SetVaultNotifications`
- `UploadArchive`
- `UploadMultipartPart`
