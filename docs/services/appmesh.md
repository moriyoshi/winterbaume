# winterbaume-appmesh

AWS App Mesh service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | App Mesh |
| AWS model | `app-mesh` |
| Protocol | restJson1 |
| winterbaume coverage | 38/38 operations (100.0%) |
| stubs (routed, returns empty/default) | 0/38 operations (0.0%) |
| moto coverage | 0/38 operations (0.0%) |
| floci coverage | 0/38 operations (0.0%) |
| kumo coverage | 25/38 operations (65.8%) |
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
aws appmesh list-meshes
```

## Example

```rust
use aws_sdk_appmesh::config::BehaviorVersion;
use winterbaume_appmesh::AppMeshService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(AppMeshService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_appmesh::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_appmesh::Client::new(&config);

    let resp = client
        .list_meshes()
        .send()
        .await
        .expect("list_meshes should succeed");
    println!("Meshes: {}", resp.meshes().len());
}
```

## Implemented APIs (38)

- `CreateGatewayRoute`
- `CreateMesh`
- `CreateRoute`
- `CreateVirtualGateway`
- `CreateVirtualNode`
- `CreateVirtualRouter`
- `CreateVirtualService`
- `DeleteGatewayRoute`
- `DeleteMesh`
- `DeleteRoute`
- `DeleteVirtualGateway`
- `DeleteVirtualNode`
- `DeleteVirtualRouter`
- `DeleteVirtualService`
- `DescribeGatewayRoute`
- `DescribeMesh`
- `DescribeRoute`
- `DescribeVirtualGateway`
- `DescribeVirtualNode`
- `DescribeVirtualRouter`
- `DescribeVirtualService`
- `ListGatewayRoutes`
- `ListMeshes`
- `ListRoutes`
- `ListTagsForResource`
- `ListVirtualGateways`
- `ListVirtualNodes`
- `ListVirtualRouters`
- `ListVirtualServices`
- `TagResource`
- `UntagResource`
- `UpdateGatewayRoute`
- `UpdateMesh`
- `UpdateRoute`
- `UpdateVirtualGateway`
- `UpdateVirtualNode`
- `UpdateVirtualRouter`
- `UpdateVirtualService`
