# winterbaume-opensearchserverless

OpenSearch Serverless service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | OpenSearch Serverless |
| AWS model | `opensearchserverless` |
| Protocol | awsJson1.0 |
| winterbaume coverage | 12/46 operations (26.1%) |
| stubs (routed, returns empty/default) | 0/46 operations (0.0%) |
| moto coverage | 12/46 operations (26.1%) |
| floci coverage | 0/46 operations (0.0%) |
| kumo coverage | 0/46 operations (0.0%) |
| fakecloud coverage | 0/46 operations (0.0%) |
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
aws opensearchserverless help
```

## Current Network Resource Stub Semantics

OpenSearch Serverless currently has one implemented VPC endpoint create path and several unimplemented endpoint operations.

- `CreateVpcEndpoint` requires a name, VPC ID, subnet IDs, and optional security group IDs, then stores them in `OpenSearchServerlessState.vpc_endpoints` with a generated `vpce-` ID.
- Duplicate endpoint names are rejected by the OpenSearch Serverless state map.
- `BatchGetVpcEndpoint`, `ListVpcEndpoints`, `UpdateVpcEndpoint`, and `DeleteVpcEndpoint` currently return not-implemented errors despite the stored endpoint map.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Example

```rust
use aws_sdk_opensearchserverless::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_opensearchserverless::OpenSearchServerlessService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(OpenSearchServerlessService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_opensearchserverless::config::Region::new(
            "us-east-1",
        ))
        .load()
        .await;

    let client = aws_sdk_opensearchserverless::Client::new(&config);

    let resp = client
        .list_collections()
        .send()
        .await
        .expect("list_collections should succeed");
    println!(
        "OpenSearch Serverless collections: {:?}",
        resp.collection_summaries()
    );
}
```

## Implemented APIs (12)

- `BatchGetCollection`
- `CreateCollection`
- `CreateSecurityPolicy`
- `CreateVpcEndpoint`
- `DeleteCollection`
- `GetSecurityPolicy`
- `ListCollections`
- `ListSecurityPolicies`
- `ListTagsForResource`
- `TagResource`
- `UntagResource`
- `UpdateSecurityPolicy`

<details><summary>Not yet implemented APIs (34)</summary>

- `BatchGetCollectionGroup`
- `BatchGetEffectiveLifecyclePolicy`
- `BatchGetLifecyclePolicy`
- `BatchGetVpcEndpoint`
- `CreateAccessPolicy`
- `CreateCollectionGroup`
- `CreateIndex`
- `CreateLifecyclePolicy`
- `CreateSecurityConfig`
- `DeleteAccessPolicy`
- `DeleteCollectionGroup`
- `DeleteIndex`
- `DeleteLifecyclePolicy`
- `DeleteSecurityConfig`
- `DeleteSecurityPolicy`
- `DeleteVpcEndpoint`
- `GetAccessPolicy`
- `GetAccountSettings`
- `GetIndex`
- `GetPoliciesStats`
- `GetSecurityConfig`
- `ListAccessPolicies`
- `ListCollectionGroups`
- `ListLifecyclePolicies`
- `ListSecurityConfigs`
- `ListVpcEndpoints`
- `UpdateAccessPolicy`
- `UpdateAccountSettings`
- `UpdateCollection`
- `UpdateCollectionGroup`
- `UpdateIndex`
- `UpdateLifecyclePolicy`
- `UpdateSecurityConfig`
- `UpdateVpcEndpoint`

</details>
