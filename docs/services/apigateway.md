# winterbaume-apigateway

API Gateway service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | API Gateway |
| AWS model | `api-gateway` |
| Protocol | restJson1 |
| winterbaume coverage | 117/124 operations (94.4%) |
| stubs (routed, returns empty/default) | 2/124 operations (1.6%) |
| moto coverage | 78/124 operations (62.9%) |
| floci coverage | 70/124 operations (56.5%) |
| kumo coverage | 17/124 operations (13.7%) |
| Coverage report date | 2026-05-16 |

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
aws apigateway get-rest-apis
```

## Current Network Resource Stub Semantics

API Gateway currently keeps networking resources inside its own service state.

- REST API endpoint configuration stores `vpcEndpointIds` as a vector of raw strings. The values are echoed through later REST API reads; there is no EC2 VPC endpoint lookup and no check that the endpoint IDs match the REST API region or account.
- VPC links are stored in `ApiGatewayState.vpc_links` keyed by the generated `vpc_link_id`. `CreateVpcLink` records the name, optional description, target ARNs, and tags, and `UpdateVpcLink` only mutates the name and description.
- `VpcLink.status` is service-local state returned by the API Gateway view; target load balancer ARNs are not checked against ELB or ELBv2 state.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Example

```rust
use aws_sdk_apigateway::config::BehaviorVersion;
use winterbaume_apigateway::ApiGatewayService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(ApiGatewayService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_apigateway::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_apigateway::Client::new(&config);

    let resp = client
        .create_rest_api()
        .name("my-api")
        .description("Example API")
        .send()
        .await
        .expect("create_rest_api should succeed");

    let api_id = resp.id().expect("id should be set");
    println!("Created REST API: {}", api_id);
    println!("Name: {}", resp.name().unwrap_or_default());
    println!("Description: {}", resp.description().unwrap_or_default());

    let get_resp = client
        .get_rest_api()
        .rest_api_id(api_id)
        .send()
        .await
        .expect("get_rest_api should succeed");

    println!("Fetched API name: {}", get_resp.name().unwrap_or_default());
}
```

## Implemented APIs (117)

- `CreateApiKey`
- `CreateAuthorizer`
- `CreateBasePathMapping`
- `CreateDeployment`
- `CreateDocumentationPart`
- `CreateDocumentationVersion`
- `CreateDomainName`
- `CreateDomainNameAccessAssociation`
- `CreateModel`
- `CreateRequestValidator`
- `CreateResource`
- `CreateRestApi`
- `CreateStage`
- `CreateUsagePlan`
- `CreateUsagePlanKey`
- `CreateVpcLink`
- `DeleteApiKey`
- `DeleteAuthorizer`
- `DeleteBasePathMapping`
- `DeleteClientCertificate`
- `DeleteDeployment`
- `DeleteDocumentationPart`
- `DeleteDocumentationVersion`
- `DeleteDomainName`
- `DeleteDomainNameAccessAssociation`
- `DeleteGatewayResponse`
- `DeleteIntegration`
- `DeleteIntegrationResponse`
- `DeleteMethod`
- `DeleteMethodResponse`
- `DeleteModel`
- `DeleteRequestValidator`
- `DeleteResource`
- `DeleteRestApi`
- `DeleteStage`
- `DeleteUsagePlan`
- `DeleteUsagePlanKey`
- `DeleteVpcLink`
- `FlushStageAuthorizersCache`
- `FlushStageCache`
- `GenerateClientCertificate`
- `GetAccount`
- `GetApiKey`
- `GetApiKeys`
- `GetAuthorizer`
- `GetAuthorizers`
- `GetBasePathMapping`
- `GetBasePathMappings`
- `GetClientCertificate`
- `GetClientCertificates`
- `GetDeployment`
- `GetDeployments`
- `GetDocumentationPart`
- `GetDocumentationParts`
- `GetDocumentationVersion`
- `GetDocumentationVersions`
- `GetDomainName`
- `GetDomainNameAccessAssociations`
- `GetDomainNames`
- `GetGatewayResponse`
- `GetGatewayResponses`
- `GetIntegration`
- `GetIntegrationResponse`
- `GetMethod`
- `GetMethodResponse`
- `GetModel`
- `GetModels`
- `GetRequestValidator`
- `GetRequestValidators`
- `GetResource`
- `GetResources`
- `GetRestApi`
- `GetRestApis`
- `GetStage`
- `GetStages`
- `GetTags`
- `GetUsage`
- `GetUsagePlan`
- `GetUsagePlanKey`
- `GetUsagePlanKeys`
- `GetUsagePlans`
- `GetVpcLink`
- `GetVpcLinks`
- `ImportDocumentationParts`
- `ImportRestApi`
- `PutGatewayResponse`
- `PutIntegration`
- `PutIntegrationResponse`
- `PutMethod`
- `PutMethodResponse`
- `PutRestApi`
- `RejectDomainNameAccessAssociation`
- `TagResource`
- `TestInvokeAuthorizer`
- `TestInvokeMethod`
- `UntagResource`
- `UpdateAccount`
- `UpdateApiKey`
- `UpdateAuthorizer`
- `UpdateBasePathMapping`
- `UpdateClientCertificate`
- `UpdateDeployment`
- `UpdateDocumentationPart`
- `UpdateDocumentationVersion`
- `UpdateDomainName`
- `UpdateGatewayResponse`
- `UpdateIntegration`
- `UpdateIntegrationResponse`
- `UpdateMethod`
- `UpdateMethodResponse`
- `UpdateModel`
- `UpdateRequestValidator`
- `UpdateResource`
- `UpdateRestApi`
- `UpdateStage`
- `UpdateUsage`
- `UpdateUsagePlan`

<details><summary>Stubbed APIs (2) &mdash; routed but return an empty/default response</summary>

- `GetExport`
- `GetModelTemplate`

</details>

<details><summary>Not yet implemented APIs (5)</summary>

- `GetSdk`
- `GetSdkType`
- `GetSdkTypes`
- `ImportApiKeys`
- `UpdateVpcLink`

</details>
