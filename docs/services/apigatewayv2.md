# winterbaume-apigatewayv2

API Gateway V2 service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | API Gateway V2 |
| AWS model | `apigatewayv2` |
| Protocol | restJson1 |
| winterbaume coverage | 62/103 operations (60.2%) |
| stubs (routed, returns empty/default) | 0/103 operations (0.0%) |
| moto coverage | 54/103 operations (52.4%) |
| floci coverage | 0/103 operations (0.0%) |
| kumo coverage | 22/103 operations (21.4%) |
| fakecloud coverage | 103/103 operations (100.0%) |
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
aws apigatewayv2 get-apis
```

## Current Network Resource Stub Semantics

API Gateway V2 currently models VPC links as local metadata.

- `CreateVpcLink` stores the supplied subnet IDs, security group IDs, name, and tags in `ApiGatewayV2State.vpc_links`.
- VPC link status is returned as the local stored status and is not driven by subnet availability, ENI creation, or load balancer health.
- Delete, get, list, tag, and untag operations only address the local VPC link map.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Example

```rust
use aws_sdk_apigatewayv2::config::BehaviorVersion;
use winterbaume_apigatewayv2::ApiGatewayV2Service;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(ApiGatewayV2Service::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_apigatewayv2::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_apigatewayv2::Client::new(&config);

    let resp = client
        .get_apis()
        .send()
        .await
        .expect("get_apis should succeed");

    println!("APIs: {:?}", resp.items());
}
```

## Implemented APIs (62)

- `CreateApi`
- `CreateApiMapping`
- `CreateAuthorizer`
- `CreateDeployment`
- `CreateDomainName`
- `CreateIntegration`
- `CreateIntegrationResponse`
- `CreateModel`
- `CreateRoute`
- `CreateRouteResponse`
- `CreateStage`
- `CreateVpcLink`
- `DeleteApi`
- `DeleteApiMapping`
- `DeleteAuthorizer`
- `DeleteCorsConfiguration`
- `DeleteDeployment`
- `DeleteDomainName`
- `DeleteIntegration`
- `DeleteIntegrationResponse`
- `DeleteModel`
- `DeleteRoute`
- `DeleteRouteRequestParameter`
- `DeleteRouteResponse`
- `DeleteStage`
- `DeleteVpcLink`
- `GetApi`
- `GetApiMapping`
- `GetApiMappings`
- `GetApis`
- `GetAuthorizer`
- `GetAuthorizers`
- `GetDeployment`
- `GetDeployments`
- `GetDomainName`
- `GetDomainNames`
- `GetIntegration`
- `GetIntegrationResponse`
- `GetIntegrationResponses`
- `GetIntegrations`
- `GetModel`
- `GetModels`
- `GetRoute`
- `GetRouteResponse`
- `GetRouteResponses`
- `GetRoutes`
- `GetStage`
- `GetStages`
- `GetTags`
- `GetVpcLink`
- `GetVpcLinks`
- `ReimportApi`
- `TagResource`
- `UntagResource`
- `UpdateApi`
- `UpdateAuthorizer`
- `UpdateIntegration`
- `UpdateIntegrationResponse`
- `UpdateModel`
- `UpdateRoute`
- `UpdateStage`
- `UpdateVpcLink`

<details><summary>Not yet implemented APIs (41)</summary>

- `CreatePortal` (implemented by fakecloud)
- `CreatePortalProduct` (implemented by fakecloud)
- `CreateProductPage` (implemented by fakecloud)
- `CreateProductRestEndpointPage` (implemented by fakecloud)
- `CreateRoutingRule` (implemented by fakecloud)
- `DeleteAccessLogSettings` (implemented by fakecloud)
- `DeletePortal` (implemented by fakecloud)
- `DeletePortalProduct` (implemented by fakecloud)
- `DeletePortalProductSharingPolicy` (implemented by fakecloud)
- `DeleteProductPage` (implemented by fakecloud)
- `DeleteProductRestEndpointPage` (implemented by fakecloud)
- `DeleteRouteSettings` (implemented by fakecloud)
- `DeleteRoutingRule` (implemented by fakecloud)
- `DisablePortal` (implemented by fakecloud)
- `ExportApi` (implemented by fakecloud)
- `GetModelTemplate` (implemented by fakecloud)
- `GetPortal` (implemented by fakecloud)
- `GetPortalProduct` (implemented by fakecloud)
- `GetPortalProductSharingPolicy` (implemented by fakecloud)
- `GetProductPage` (implemented by fakecloud)
- `GetProductRestEndpointPage` (implemented by fakecloud)
- `GetRoutingRule` (implemented by fakecloud)
- `ImportApi` (implemented by fakecloud)
- `ListPortalProducts` (implemented by fakecloud)
- `ListPortals` (implemented by fakecloud)
- `ListProductPages` (implemented by fakecloud)
- `ListProductRestEndpointPages` (implemented by fakecloud)
- `ListRoutingRules` (implemented by fakecloud)
- `PreviewPortal` (implemented by fakecloud)
- `PublishPortal` (implemented by fakecloud)
- `PutPortalProductSharingPolicy` (implemented by fakecloud)
- `PutRoutingRule` (implemented by fakecloud)
- `ResetAuthorizersCache` (implemented by fakecloud)
- `UpdateApiMapping` (implemented by fakecloud)
- `UpdateDeployment` (implemented by fakecloud)
- `UpdateDomainName` (implemented by fakecloud)
- `UpdatePortal` (implemented by fakecloud)
- `UpdatePortalProduct` (implemented by fakecloud)
- `UpdateProductPage` (implemented by fakecloud)
- `UpdateProductRestEndpointPage` (implemented by fakecloud)
- `UpdateRouteResponse` (implemented by fakecloud)

</details>
