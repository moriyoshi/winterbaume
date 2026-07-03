# winterbaume-appsync

AWS AppSync service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | AppSync |
| AWS model | `appsync` |
| Protocol | restJson1 |
| winterbaume coverage | 27/74 operations (36.5%) |
| stubs (routed, returns empty/default) | 0/74 operations (0.0%) |
| moto coverage | 27/74 operations (36.5%) |
| floci coverage | 0/74 operations (0.0%) |
| kumo coverage | 3/74 operations (4.1%) |
| fakecloud coverage | 0/74 operations (0.0%) |
| Coverage report date | 2026-07-03 |

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
aws appsync list-graphql-apis
```

## Example

```rust
use aws_sdk_appsync::config::BehaviorVersion;
use winterbaume_appsync::AppSyncService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(AppSyncService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_appsync::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_appsync::Client::new(&config);

    let resp = client
        .list_graphql_apis()
        .send()
        .await
        .expect("list_graphql_apis should succeed");
    println!("GraphQL APIs: {}", resp.graphql_apis().len());
}
```

## Implemented APIs (27)

- `CreateApi`
- `CreateApiCache`
- `CreateApiKey`
- `CreateChannelNamespace`
- `CreateGraphqlApi`
- `DeleteApi`
- `DeleteApiCache`
- `DeleteApiKey`
- `DeleteChannelNamespace`
- `DeleteGraphqlApi`
- `FlushApiCache`
- `GetApi`
- `GetApiCache`
- `GetGraphqlApi`
- `GetSchemaCreationStatus`
- `GetType`
- `ListApiKeys`
- `ListApis`
- `ListChannelNamespaces`
- `ListGraphqlApis`
- `ListTagsForResource`
- `StartSchemaCreation`
- `TagResource`
- `UntagResource`
- `UpdateApiCache`
- `UpdateApiKey`
- `UpdateGraphqlApi`

<details><summary>Not yet implemented APIs (47)</summary>

- `AssociateApi`
- `AssociateMergedGraphqlApi`
- `AssociateSourceGraphqlApi`
- `CreateDataSource` (implemented by kumo)
- `CreateDomainName`
- `CreateFunction`
- `CreateResolver` (implemented by kumo)
- `CreateType`
- `DeleteDataSource`
- `DeleteDomainName`
- `DeleteFunction`
- `DeleteResolver`
- `DeleteType`
- `DisassociateApi`
- `DisassociateMergedGraphqlApi`
- `DisassociateSourceGraphqlApi`
- `EvaluateCode`
- `EvaluateMappingTemplate`
- `GetApiAssociation`
- `GetChannelNamespace`
- `GetDataSource`
- `GetDataSourceIntrospection`
- `GetDomainName`
- `GetFunction`
- `GetGraphqlApiEnvironmentVariables`
- `GetIntrospectionSchema`
- `GetResolver`
- `GetSourceApiAssociation`
- `ListDataSources`
- `ListDomainNames`
- `ListFunctions`
- `ListResolvers`
- `ListResolversByFunction`
- `ListSourceApiAssociations`
- `ListTypes`
- `ListTypesByAssociation`
- `PutGraphqlApiEnvironmentVariables`
- `StartDataSourceIntrospection`
- `StartSchemaMerge`
- `UpdateApi`
- `UpdateChannelNamespace`
- `UpdateDataSource`
- `UpdateDomainName`
- `UpdateFunction`
- `UpdateResolver`
- `UpdateSourceApiAssociation`
- `UpdateType`

</details>
