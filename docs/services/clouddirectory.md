# winterbaume-clouddirectory

AWS Cloud Directory service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Cloud Directory |
| AWS model | `clouddirectory` |
| Protocol | restJson1 |
| winterbaume coverage | 13/66 operations (19.7%) |
| stubs (routed, returns empty/default) | 0/66 operations (0.0%) |
| moto coverage | 13/66 operations (19.7%) |
| floci coverage | 0/66 operations (0.0%) |
| kumo coverage | 0/66 operations (0.0%) |
| Coverage report date | 2026-05-13 |

## Server-mode usage

Start `winterbaume-server` and point the AWS CLI at it:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws clouddirectory help
```

## Example

```rust
use aws_sdk_clouddirectory::config::BehaviorVersion;
use winterbaume_clouddirectory::CloudDirectoryService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(CloudDirectoryService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_clouddirectory::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_clouddirectory::Client::new(&config);

    // Create a schema
    let resp = client
        .create_schema()
        .name("ExampleSchema")
        .send()
        .await
        .expect("create_schema should succeed");

    let schema_arn = resp.schema_arn().expect("should have schema_arn");
    println!("Created schema ARN: {}", schema_arn);

    // Delete the schema
    client
        .delete_schema()
        .schema_arn(schema_arn)
        .send()
        .await
        .expect("delete_schema should succeed");

    println!("Deleted schema successfully");
}
```

## Implemented APIs (13)

- `ApplySchema`
- `CreateDirectory`
- `CreateSchema`
- `DeleteDirectory`
- `DeleteSchema`
- `GetDirectory`
- `ListDevelopmentSchemaArns`
- `ListDirectories`
- `ListPublishedSchemaArns`
- `ListTagsForResource`
- `PublishSchema`
- `TagResource`
- `UntagResource`

<details><summary>Not yet implemented APIs (53)</summary>

- `AddFacetToObject`
- `AttachObject`
- `AttachPolicy`
- `AttachToIndex`
- `AttachTypedLink`
- `BatchRead`
- `BatchWrite`
- `CreateFacet`
- `CreateIndex`
- `CreateObject`
- `CreateTypedLinkFacet`
- `DeleteFacet`
- `DeleteObject`
- `DeleteTypedLinkFacet`
- `DetachFromIndex`
- `DetachObject`
- `DetachPolicy`
- `DetachTypedLink`
- `DisableDirectory`
- `EnableDirectory`
- `GetAppliedSchemaVersion`
- `GetFacet`
- `GetLinkAttributes`
- `GetObjectAttributes`
- `GetObjectInformation`
- `GetSchemaAsJson`
- `GetTypedLinkFacetInformation`
- `ListAppliedSchemaArns`
- `ListAttachedIndices`
- `ListFacetAttributes`
- `ListFacetNames`
- `ListIncomingTypedLinks`
- `ListIndex`
- `ListManagedSchemaArns`
- `ListObjectAttributes`
- `ListObjectChildren`
- `ListObjectParentPaths`
- `ListObjectParents`
- `ListObjectPolicies`
- `ListOutgoingTypedLinks`
- `ListPolicyAttachments`
- `ListTypedLinkFacetAttributes`
- `ListTypedLinkFacetNames`
- `LookupPolicy`
- `PutSchemaFromJson`
- `RemoveFacetFromObject`
- `UpdateFacet`
- `UpdateLinkAttributes`
- `UpdateObjectAttributes`
- `UpdateSchema`
- `UpdateTypedLinkFacet`
- `UpgradeAppliedSchema`
- `UpgradePublishedSchema`

</details>
