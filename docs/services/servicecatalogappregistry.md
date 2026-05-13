# winterbaume-servicecatalogappregistry

Service Catalog App Registry service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Service Catalog AppRegistry |
| AWS model | `service-catalog-appregistry` |
| Protocol | restJson1 |
| winterbaume coverage | 23/24 operations (95.8%) |
| stubs (routed, returns empty/default) | 1/24 operations (4.2%) |
| moto coverage | 0/24 operations (0.0%) |
| floci coverage | 0/24 operations (0.0%) |
| kumo coverage | 0/24 operations (0.0%) |
| Coverage report date | 2026-05-13 |

## Server-mode usage

Start `winterbaume-server` and point the AWS CLI at it:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws servicecatalog-appregistry list-applications
```

## Example

```rust
use aws_sdk_servicecatalogappregistry::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_servicecatalogappregistry::ServiceCatalogAppRegistryService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(ServiceCatalogAppRegistryService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_servicecatalogappregistry::config::Region::new(
            "us-east-1",
        ))
        .load()
        .await;

    let client = aws_sdk_servicecatalogappregistry::Client::new(&config);

    let resp = client
        .list_applications()
        .send()
        .await
        .expect("list_applications should succeed");
    println!("AppRegistry applications: {}", resp.applications().len());
}
```

## Implemented APIs (23)

- `AssociateAttributeGroup`
- `AssociateResource`
- `CreateApplication`
- `CreateAttributeGroup`
- `DeleteApplication`
- `DeleteAttributeGroup`
- `DisassociateAttributeGroup`
- `DisassociateResource`
- `GetApplication`
- `GetAssociatedResource`
- `GetAttributeGroup`
- `GetConfiguration`
- `ListApplications`
- `ListAssociatedAttributeGroups`
- `ListAssociatedResources`
- `ListAttributeGroups`
- `ListAttributeGroupsForApplication`
- `ListTagsForResource`
- `PutConfiguration`
- `TagResource`
- `UntagResource`
- `UpdateApplication`
- `UpdateAttributeGroup`

<details><summary>Stubbed APIs (1) &mdash; routed but return an empty/default response</summary>

- `SyncResource`

</details>
