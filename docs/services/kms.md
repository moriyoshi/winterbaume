# winterbaume-kms

KMS service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | KMS |
| AWS model | `kms` |
| Protocol | awsJson1.1 |
| winterbaume coverage | 53/54 operations (98.1%) |
| stubs (routed, returns empty/default) | 0/54 operations (0.0%) |
| moto coverage | 40/54 operations (74.1%) |
| floci coverage | 0/54 operations (0.0%) |
| kumo coverage | 19/54 operations (35.2%) |
| Coverage report date | 2026-05-12 |

## Server-mode usage

Start `winterbaume-server` and point the AWS CLI at it:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws kms list-keys
```

## Example

```rust
use aws_sdk_kms::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_kms::KmsService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder().with_service(KmsService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_kms::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_kms::Client::new(&config);

    let resp = client
        .list_keys()
        .send()
        .await
        .expect("list_keys should succeed");
    println!("KMS keys: {}", resp.keys().len());
}
```

## Implemented APIs (53)

- `CancelKeyDeletion`
- `ConnectCustomKeyStore`
- `CreateAlias`
- `CreateCustomKeyStore`
- `CreateGrant`
- `CreateKey`
- `Decrypt`
- `DeleteAlias`
- `DeleteCustomKeyStore`
- `DeleteImportedKeyMaterial`
- `DeriveSharedSecret`
- `DescribeCustomKeyStores`
- `DescribeKey`
- `DisableKey`
- `DisableKeyRotation`
- `DisconnectCustomKeyStore`
- `EnableKey`
- `EnableKeyRotation`
- `Encrypt`
- `GenerateDataKey`
- `GenerateDataKeyPair`
- `GenerateDataKeyPairWithoutPlaintext`
- `GenerateDataKeyWithoutPlaintext`
- `GenerateMac`
- `GenerateRandom`
- `GetKeyPolicy`
- `GetKeyRotationStatus`
- `GetParametersForImport`
- `GetPublicKey`
- `ImportKeyMaterial`
- `ListAliases`
- `ListGrants`
- `ListKeyPolicies`
- `ListKeyRotations`
- `ListKeys`
- `ListResourceTags`
- `ListRetirableGrants`
- `PutKeyPolicy`
- `ReEncrypt`
- `ReplicateKey`
- `RetireGrant`
- `RevokeGrant`
- `RotateKeyOnDemand`
- `ScheduleKeyDeletion`
- `Sign`
- `TagResource`
- `UntagResource`
- `UpdateAlias`
- `UpdateCustomKeyStore`
- `UpdateKeyDescription`
- `UpdatePrimaryRegion`
- `Verify`
- `VerifyMac`

<details><summary>Not yet implemented APIs (1)</summary>

- `GetKeyLastUsage`

</details>
