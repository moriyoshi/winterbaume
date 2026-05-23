# AmazonApiGatewayV2

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon API Gateway V2

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for AmazonApiGatewayV2 where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: exercise account or service defaults for AmazonApiGatewayV2 by toggling configuration, creating later resources without explicit overrides, and confirming the default propagates into those resources.
- Scenario insight from EC2: cover association replacement for AmazonApiGatewayV2 by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- From the AWS documentation and model: build HTTP and WebSocket APIs with routes, integrations, deployments, stages, authorisers, API mappings, and custom domains.
- From the operation surface: model lightweight API front doors, Lambda proxy integrations, private integrations through VPC links, route-level configuration, and stage promotion.

## Service Identity and Protocol

- AWS model slug: `apigatewayv2`
- AWS SDK for Rust slug: `apigatewayv2`
- Model version: `2018-11-29`
- Model file: `vendor/api-models-aws/models/apigatewayv2/service/2018-11-29/apigatewayv2-2018-11-29.json`
- SDK ID: `ApiGatewayV2`
- Endpoint prefix: `apigateway`
- ARN namespace: `apigateway`
- CloudFormation name: `ApiGatewayV2`
- CloudTrail event source: `apigatewayv2.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (32), `Delete` (22), `Create` (17), `Update` (16), `List` (5), `Put` (2), `Disable` (1), `Export` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateApi`, `CreateApiMapping`, `CreateAuthorizer`, `CreateDeployment`, `CreateDomainName`, `CreateIntegration`, `CreateIntegrationResponse`, `CreateModel`, `CreatePortal`, `CreatePortalProduct`, `CreateProductPage`, `CreateProductRestEndpointPage`, `CreateRoute`, `CreateRouteResponse`, `CreateRoutingRule`, `CreateStage`, `CreateVpcLink`, `DeleteAccessLogSettings`, `DeleteApi`, `DeleteApiMapping`, ... (+41).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetApi`, `GetApiMapping`, `GetApiMappings`, `GetApis`, `GetAuthorizer`, `GetAuthorizers`, `GetDeployment`, `GetDeployments`, `GetDomainName`, `GetDomainNames`, `GetIntegration`, `GetIntegrationResponse`, `GetIntegrationResponses`, `GetIntegrations`, `GetModel`, `GetModelTemplate`, `GetModels`, `GetPortal`, `GetPortalProduct`, `GetPortalProductSharingPolicy`, ... (+17).
- Pagination is modelled for 1 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 2 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `ExportApi`, `ImportApi`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 103 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `Lambda`, `EC2/VPC`.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api-develop-routes.html
- https://docs.aws.amazon.com/apigateway/latest/developerguide/http-api-stages.html
- https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-use-lambda-authorizer.html

Research outcomes:
- HTTP API stages are named lifecycle references to deployments. A `$default` stage is served from the API base URL.
- A deployment is a snapshot of API configuration. Without automatic deployments, configuration changes must be deployed to a stage before callers observe them.
- Stage selection happens before route selection. If no explicit stage matches, API Gateway tries the `$default` stage; without it, the request returns Not Found and does not generate CloudWatch logs.
- Route selection uses the most specific match: exact route and method, then greedy path variable route, then `$default` route.
- Greedy path variables such as `{proxy+}` must appear at the end of the route path and match all child resources.
- Query string parameters are forwarded to HTTP API integrations by default.
- Lambda authorizer identity-source and cache-key semantics apply to HTTP APIs where configured.

Parity implications:
- Implement stage resolution before route resolution, including `$default` stage behaviour and the no-log Not Found path.
- Route matching needs exact, greedy, ANY-method, and `$default` precedence rather than insertion order.
- Auto-deploy and manual deployment modes should determine when route, integration, and authorizer changes affect invocations.

## v1/v2 State Coherence

- **Paired with `apigateway` ( different SDK slug, different API surface ).** API Gateway v2 ( HTTP APIs and WebSocket APIs, this crate ) and API Gateway v1 ( REST APIs ) are **separate API products** in real AWS. An HTTP/WebSocket API created via v2 is not visible to v1 `GetRestApis`, and a REST API created via v1 is not visible to v2 `GetApis`. Resources scoped under each parent API ( routes/integrations/stages/deployments/authorizers vs methods/integrations/stages/deployments/authorizers ) are also disjoint between the two surfaces.
- **Shared resources to be aware of:**
  - **Custom domain names.** A single custom domain name in an account+region can host both HTTP/WebSocket APIs ( via v2 API mappings, `(domain_name, mapping_id) → apiId/stage` ) and REST APIs ( via v1 base-path mappings, `(domain_name, base_path) → restApiId/stage` ). Real AWS reserves the domain name in a single namespace; v2 `CreateDomainName` and v1 `CreateDomainName` cannot both create the same domain.
  - **VPC links.** v2 and v1 each define their own `VpcLink` resource and the lists are not shared in real AWS, so independent state per crate is correct here.
  - HTTP APIs do not support API keys or usage plans; no coherence concern with v1's API-key surface.
- **Current Winterbaume status: separated, with one observable gap.** Each crate owns its own `domain_names` map. A consumer that creates `example.com` via v2 and then tries to create the same domain via v1 ( or vice versa ) will succeed in Winterbaume but fail in real AWS. Custom-domain coherence is the realistic follow-up; the rest of the resource models are correctly separate.

## Cross-Service Integration Gaps

- **`apigateway-lambda`** ( primary ): API Gateway V2 ( this crate ) and API Gateway V1 store integration endpoints but do not invoke Lambda functions or any other integration target when requests arrive. For HTTP APIs, the first invocation seam to add is `AWS_PROXY` Lambda integration; the WebSocket route → integration dispatch path on `$connect` / `$disconnect` / `$default` plus custom routes is the second. Tracked in `.agents/docs/TODO.md` ( "Cross-Service Integration Gaps" → `apigateway-lambda` ).

## Current Network Resource Stub Semantics

API Gateway V2 currently models VPC links as local metadata.

- `CreateVpcLink` stores the supplied subnet IDs, security group IDs, name, and tags in `ApiGatewayV2State.vpc_links`.
- VPC link status is returned as the local stored status and is not driven by subnet availability, ENI creation, or load balancer health.
- Delete, get, list, tag, and untag operations only address the local VPC link map.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Operation Groups

### Get

- Operations: `GetApi`, `GetApiMapping`, `GetApiMappings`, `GetApis`, `GetAuthorizer`, `GetAuthorizers`, `GetDeployment`, `GetDeployments`, `GetDomainName`, `GetDomainNames`, `GetIntegration`, `GetIntegrationResponse`, `GetIntegrationResponses`, `GetIntegrations`, `GetModel`, `GetModels`, `GetModelTemplate`, `GetPortal`, `GetPortalProduct`, `GetPortalProductSharingPolicy`, `GetProductPage`, `GetProductRestEndpointPage`, `GetRoute`, `GetRouteResponse`, `GetRouteResponses`, `GetRoutes`, `GetRoutingRule`, `GetStage`, `GetStages`, `GetTags`, `GetVpcLink`, `GetVpcLinks`
- Common required input members in this group: `ApiId`, `DomainName`, `IntegrationId`, `ModelId`, `PortalProductId`, `RouteId`

### Delete

- Operations: `DeleteAccessLogSettings`, `DeleteApi`, `DeleteApiMapping`, `DeleteAuthorizer`, `DeleteCorsConfiguration`, `DeleteDeployment`, `DeleteDomainName`, `DeleteIntegration`, `DeleteIntegrationResponse`, `DeleteModel`, `DeletePortal`, `DeletePortalProduct`, `DeletePortalProductSharingPolicy`, `DeleteProductPage`, `DeleteProductRestEndpointPage`, `DeleteRoute`, `DeleteRouteRequestParameter`, `DeleteRouteResponse`, `DeleteRouteSettings`, `DeleteRoutingRule`, `DeleteStage`, `DeleteVpcLink`
- Traits: `idempotent` (1)
- Common required input members in this group: `ApiId`, `StageName`, `DomainName`, `IntegrationId`, `PortalProductId`, `RouteId`

### Create

- Operations: `CreateApi`, `CreateApiMapping`, `CreateAuthorizer`, `CreateDeployment`, `CreateDomainName`, `CreateIntegration`, `CreateIntegrationResponse`, `CreateModel`, `CreatePortal`, `CreatePortalProduct`, `CreateProductPage`, `CreateProductRestEndpointPage`, `CreateRoute`, `CreateRouteResponse`, `CreateRoutingRule`, `CreateStage`, `CreateVpcLink`
- Common required input members in this group: `Name`, `ApiId`, `DomainName`, `PortalProductId`

### Update

- Operations: `UpdateApi`, `UpdateApiMapping`, `UpdateAuthorizer`, `UpdateDeployment`, `UpdateDomainName`, `UpdateIntegration`, `UpdateIntegrationResponse`, `UpdateModel`, `UpdatePortal`, `UpdatePortalProduct`, `UpdateProductPage`, `UpdateProductRestEndpointPage`, `UpdateRoute`, `UpdateRouteResponse`, `UpdateStage`, `UpdateVpcLink`
- Common required input members in this group: `ApiId`, `DomainName`, `IntegrationId`, `PortalProductId`, `RouteId`

### List

- Operations: `ListPortalProducts`, `ListPortals`, `ListProductPages`, `ListProductRestEndpointPages`, `ListRoutingRules`
- Traits: `paginated` (1)
- Common required input members in this group: `PortalProductId`

### Put

- Operations: `PutPortalProductSharingPolicy`, `PutRoutingRule`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Disable

- Operations: `DisablePortal`
- Common required input members in this group: -

### Export

- Operations: `ExportApi`
- Common required input members in this group: -

### Import

- Operations: `ImportApi`
- Common required input members in this group: -

### Preview

- Operations: `PreviewPortal`
- Common required input members in this group: -

### Publish

- Operations: `PublishPortal`
- Common required input members in this group: -

### Reimport

- Operations: `ReimportApi`
- Common required input members in this group: -

### Reset

- Operations: `ResetAuthorizersCache`
- Common required input members in this group: -

### Tag

- Operations: `TagResource`
- Common required input members in this group: -

### Untag

- Operations: `UntagResource`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateApi` | `POST /v2/apis` | - | `Name`, `ProtocolType` | - | `CreateApiResponse` | `BadRequestException`, `ConflictException`, `NotFoundException`, `TooManyRequestsException` | Creates an Api resource. |
| `CreateApiMapping` | `POST /v2/domainnames/{DomainName}/apimappings` | - | `ApiId`, `DomainName`, `Stage` | - | `CreateApiMappingResponse` | `BadRequestException`, `ConflictException`, `NotFoundException`, `TooManyRequestsException` | Creates an API mapping. |
| `CreateAuthorizer` | `POST /v2/apis/{ApiId}/authorizers` | - | `ApiId`, `AuthorizerType`, `IdentitySource`, `Name` | - | `CreateAuthorizerResponse` | `BadRequestException`, `ConflictException`, `NotFoundException`, `TooManyRequestsException` | Creates an Authorizer for an API. |
| `CreateDeployment` | `POST /v2/apis/{ApiId}/deployments` | - | `ApiId` | - | `CreateDeploymentResponse` | `BadRequestException`, `ConflictException`, `NotFoundException`, `TooManyRequestsException` | Creates a Deployment for an API. |
| `CreateDomainName` | `POST /v2/domainnames` | - | `DomainName` | - | `CreateDomainNameResponse` | `AccessDeniedException`, `BadRequestException`, `ConflictException`, `NotFoundException`, `TooManyRequestsException` | Creates a domain name. |
| `CreateIntegration` | `POST /v2/apis/{ApiId}/integrations` | - | `ApiId`, `IntegrationType` | - | `CreateIntegrationResult` | `BadRequestException`, `ConflictException`, `NotFoundException`, `TooManyRequestsException` | Creates an Integration. |
| `CreateIntegrationResponse` | `POST /v2/apis/{ApiId}/integrations/{IntegrationId}/integrationresponses` | - | `ApiId`, `IntegrationId`, `IntegrationResponseKey` | - | `CreateIntegrationResponseResponse` | `BadRequestException`, `ConflictException`, `NotFoundException`, `TooManyRequestsException` | Creates an IntegrationResponses. |
| `CreateModel` | `POST /v2/apis/{ApiId}/models` | - | `ApiId`, `Name`, `Schema` | - | `CreateModelResponse` | `BadRequestException`, `ConflictException`, `NotFoundException`, `TooManyRequestsException` | Creates a Model for an API. |
| `CreatePortal` | `POST /v2/portals` | - | `Authorization`, `EndpointConfiguration`, `PortalContent` | - | `CreatePortalResponse` | `AccessDeniedException`, `BadRequestException`, `TooManyRequestsException` | Creates a portal. |
| `CreatePortalProduct` | `POST /v2/portalproducts` | - | `DisplayName` | - | `CreatePortalProductResponse` | `AccessDeniedException`, `BadRequestException`, `TooManyRequestsException` | Creates a new portal product. |
| `CreateProductPage` | `POST /v2/portalproducts/{PortalProductId}/productpages` | - | `DisplayContent`, `PortalProductId` | - | `CreateProductPageResponse` | `AccessDeniedException`, `BadRequestException`, `NotFoundException`, `TooManyRequestsException` | Creates a new product page for a portal product. |
| `CreateProductRestEndpointPage` | `POST /v2/portalproducts/{PortalProductId}/productrestendpointpages` | - | `PortalProductId`, `RestEndpointIdentifier` | - | `CreateProductRestEndpointPageResponse` | `AccessDeniedException`, `BadRequestException`, `NotFoundException`, `TooManyRequestsException` | Creates a product REST endpoint page for a portal product. |
| `CreateRoute` | `POST /v2/apis/{ApiId}/routes` | - | `ApiId`, `RouteKey` | - | `CreateRouteResult` | `BadRequestException`, `ConflictException`, `NotFoundException`, `TooManyRequestsException` | Creates a Route for an API. |
| `CreateRouteResponse` | `POST /v2/apis/{ApiId}/routes/{RouteId}/routeresponses` | - | `ApiId`, `RouteId`, `RouteResponseKey` | - | `CreateRouteResponseResponse` | `BadRequestException`, `ConflictException`, `NotFoundException`, `TooManyRequestsException` | Creates a RouteResponse for a Route. |
| `CreateRoutingRule` | `POST /v2/domainnames/{DomainName}/routingrules` | - | `Actions`, `Conditions`, `DomainName`, `Priority` | - | `CreateRoutingRuleResponse` | `BadRequestException`, `ConflictException`, `NotFoundException`, `TooManyRequestsException` | Creates a RoutingRule. |
| `CreateStage` | `POST /v2/apis/{ApiId}/stages` | - | `ApiId`, `StageName` | - | `CreateStageResponse` | `BadRequestException`, `ConflictException`, `NotFoundException`, `TooManyRequestsException` | Creates a Stage for an API. |
| `CreateVpcLink` | `POST /v2/vpclinks` | - | `Name`, `SubnetIds` | - | `CreateVpcLinkResponse` | `BadRequestException`, `TooManyRequestsException` | Creates a VPC link. |
| `DeleteAccessLogSettings` | `DELETE /v2/apis/{ApiId}/stages/{StageName}/accesslogsettings` | - | `ApiId`, `StageName` | - | `Unit` | `NotFoundException`, `TooManyRequestsException` | Deletes the AccessLogSettings for a Stage. To disable access logging for a Stage, delete its AccessLogSettings. |
| `DeleteApi` | `DELETE /v2/apis/{ApiId}` | - | `ApiId` | - | `Unit` | `NotFoundException`, `TooManyRequestsException` | Deletes an Api resource. |
| `DeleteApiMapping` | `DELETE /v2/domainnames/{DomainName}/apimappings/{ApiMappingId}` | - | `ApiMappingId`, `DomainName` | - | `Unit` | `BadRequestException`, `NotFoundException`, `TooManyRequestsException` | Deletes an API mapping. |
| `DeleteAuthorizer` | `DELETE /v2/apis/{ApiId}/authorizers/{AuthorizerId}` | - | `ApiId`, `AuthorizerId` | - | `Unit` | `NotFoundException`, `TooManyRequestsException` | Deletes an Authorizer. |
| `DeleteCorsConfiguration` | `DELETE /v2/apis/{ApiId}/cors` | - | `ApiId` | - | `Unit` | `NotFoundException`, `TooManyRequestsException` | Deletes a CORS configuration. |
| `DeleteDeployment` | `DELETE /v2/apis/{ApiId}/deployments/{DeploymentId}` | - | `ApiId`, `DeploymentId` | - | `Unit` | `NotFoundException`, `TooManyRequestsException` | Deletes a Deployment. |
| `DeleteDomainName` | `DELETE /v2/domainnames/{DomainName}` | - | `DomainName` | - | `Unit` | `NotFoundException`, `TooManyRequestsException` | Deletes a domain name. |
| `DeleteIntegration` | `DELETE /v2/apis/{ApiId}/integrations/{IntegrationId}` | - | `ApiId`, `IntegrationId` | - | `Unit` | `NotFoundException`, `TooManyRequestsException` | Deletes an Integration. |
| `DeleteIntegrationResponse` | `DELETE /v2/apis/{ApiId}/integrations/{IntegrationId}/integrationresponses/{IntegrationResponseId}` | - | `ApiId`, `IntegrationId`, `IntegrationResponseId` | - | `Unit` | `NotFoundException`, `TooManyRequestsException` | Deletes an IntegrationResponses. |
| `DeleteModel` | `DELETE /v2/apis/{ApiId}/models/{ModelId}` | - | `ApiId`, `ModelId` | - | `Unit` | `NotFoundException`, `TooManyRequestsException` | Deletes a Model. |
| `DeletePortal` | `DELETE /v2/portals/{PortalId}` | - | `PortalId` | - | `Unit` | `AccessDeniedException`, `BadRequestException`, `TooManyRequestsException` | Deletes a portal. |
| `DeletePortalProduct` | `DELETE /v2/portalproducts/{PortalProductId}` | - | `PortalProductId` | - | `Unit` | `AccessDeniedException`, `BadRequestException`, `NotFoundException`, `TooManyRequestsException` | Deletes a portal product. |
| `DeletePortalProductSharingPolicy` | `DELETE /v2/portalproducts/{PortalProductId}/sharingpolicy` | - | `PortalProductId` | - | `Unit` | `AccessDeniedException`, `BadRequestException`, `NotFoundException`, `TooManyRequestsException` | Deletes the sharing policy for a portal product. |
| `DeleteProductPage` | `DELETE /v2/portalproducts/{PortalProductId}/productpages/{ProductPageId}` | - | `PortalProductId`, `ProductPageId` | - | `Unit` | `AccessDeniedException`, `BadRequestException`, `NotFoundException`, `TooManyRequestsException` | Deletes a product page of a portal product. |
| `DeleteProductRestEndpointPage` | `DELETE /v2/portalproducts/{PortalProductId}/productrestendpointpages/{ProductRestEndpointPageId}` | - | `PortalProductId`, `ProductRestEndpointPageId` | - | `Unit` | `AccessDeniedException`, `BadRequestException`, `NotFoundException`, `TooManyRequestsException` | Deletes a product REST endpoint page. |
| `DeleteRoute` | `DELETE /v2/apis/{ApiId}/routes/{RouteId}` | - | `ApiId`, `RouteId` | - | `Unit` | `NotFoundException`, `TooManyRequestsException` | Deletes a Route. |
| `DeleteRouteRequestParameter` | `DELETE /v2/apis/{ApiId}/routes/{RouteId}/requestparameters/{RequestParameterKey}` | - | `ApiId`, `RequestParameterKey`, `RouteId` | - | `Unit` | `NotFoundException`, `TooManyRequestsException` | Deletes a route request parameter. Supported only for WebSocket APIs. |
| `DeleteRouteResponse` | `DELETE /v2/apis/{ApiId}/routes/{RouteId}/routeresponses/{RouteResponseId}` | - | `ApiId`, `RouteId`, `RouteResponseId` | - | `Unit` | `NotFoundException`, `TooManyRequestsException` | Deletes a RouteResponse. |
| `DeleteRouteSettings` | `DELETE /v2/apis/{ApiId}/stages/{StageName}/routesettings/{RouteKey}` | - | `ApiId`, `RouteKey`, `StageName` | - | `Unit` | `NotFoundException`, `TooManyRequestsException` | Deletes the RouteSettings for a stage. |
| `DeleteRoutingRule` | `DELETE /v2/domainnames/{DomainName}/routingrules/{RoutingRuleId}` | `idempotent` | `DomainName`, `RoutingRuleId` | - | `Unit` | `BadRequestException`, `NotFoundException`, `TooManyRequestsException` | Deletes a routing rule. |
| `DeleteStage` | `DELETE /v2/apis/{ApiId}/stages/{StageName}` | - | `ApiId`, `StageName` | - | `Unit` | `NotFoundException`, `TooManyRequestsException` | Deletes a Stage. |
| `DeleteVpcLink` | `DELETE /v2/vpclinks/{VpcLinkId}` | - | `VpcLinkId` | - | `DeleteVpcLinkResponse` | `NotFoundException`, `TooManyRequestsException` | Deletes a VPC link. |
| `DisablePortal` | `DELETE /v2/portals/{PortalId}/publish` | - | `PortalId` | - | `Unit` | `AccessDeniedException`, `BadRequestException`, `ConflictException`, `NotFoundException`, `TooManyRequestsException` | Deletes the publication of a portal portal. |
| `ExportApi` | `GET /v2/apis/{ApiId}/exports/{Specification}` | - | `ApiId`, `OutputType`, `Specification` | - | `ExportApiResponse` | `BadRequestException`, `NotFoundException`, `TooManyRequestsException` | - |
| `GetApi` | `GET /v2/apis/{ApiId}` | - | `ApiId` | - | `GetApiResponse` | `NotFoundException`, `TooManyRequestsException` | Gets an Api resource. |
| `GetApiMapping` | `GET /v2/domainnames/{DomainName}/apimappings/{ApiMappingId}` | - | `ApiMappingId`, `DomainName` | - | `GetApiMappingResponse` | `BadRequestException`, `NotFoundException`, `TooManyRequestsException` | Gets an API mapping. |
| `GetApiMappings` | `GET /v2/domainnames/{DomainName}/apimappings` | - | `DomainName` | - | `GetApiMappingsResponse` | `BadRequestException`, `NotFoundException`, `TooManyRequestsException` | Gets API mappings. |
| `GetApis` | `GET /v2/apis` | - | - | - | `GetApisResponse` | `BadRequestException`, `NotFoundException`, `TooManyRequestsException` | Gets a collection of Api resources. |
| `GetAuthorizer` | `GET /v2/apis/{ApiId}/authorizers/{AuthorizerId}` | - | `ApiId`, `AuthorizerId` | - | `GetAuthorizerResponse` | `NotFoundException`, `TooManyRequestsException` | Gets an Authorizer. |
| `GetAuthorizers` | `GET /v2/apis/{ApiId}/authorizers` | - | `ApiId` | - | `GetAuthorizersResponse` | `BadRequestException`, `NotFoundException`, `TooManyRequestsException` | Gets the Authorizers for an API. |
| `GetDeployment` | `GET /v2/apis/{ApiId}/deployments/{DeploymentId}` | - | `ApiId`, `DeploymentId` | - | `GetDeploymentResponse` | `NotFoundException`, `TooManyRequestsException` | Gets a Deployment. |
| `GetDeployments` | `GET /v2/apis/{ApiId}/deployments` | - | `ApiId` | - | `GetDeploymentsResponse` | `BadRequestException`, `NotFoundException`, `TooManyRequestsException` | Gets the Deployments for an API. |
| `GetDomainName` | `GET /v2/domainnames/{DomainName}` | - | `DomainName` | - | `GetDomainNameResponse` | `NotFoundException`, `TooManyRequestsException` | Gets a domain name. |
| `GetDomainNames` | `GET /v2/domainnames` | - | - | - | `GetDomainNamesResponse` | `BadRequestException`, `NotFoundException`, `TooManyRequestsException` | Gets the domain names for an AWS account. |
| `GetIntegration` | `GET /v2/apis/{ApiId}/integrations/{IntegrationId}` | - | `ApiId`, `IntegrationId` | - | `GetIntegrationResult` | `NotFoundException`, `TooManyRequestsException` | Gets an Integration. |
| `GetIntegrationResponse` | `GET /v2/apis/{ApiId}/integrations/{IntegrationId}/integrationresponses/{IntegrationResponseId}` | - | `ApiId`, `IntegrationId`, `IntegrationResponseId` | - | `GetIntegrationResponseResponse` | `NotFoundException`, `TooManyRequestsException` | Gets an IntegrationResponses. |
| `GetIntegrationResponses` | `GET /v2/apis/{ApiId}/integrations/{IntegrationId}/integrationresponses` | - | `ApiId`, `IntegrationId` | - | `GetIntegrationResponsesResponse` | `BadRequestException`, `NotFoundException`, `TooManyRequestsException` | Gets the IntegrationResponses for an Integration. |
| `GetIntegrations` | `GET /v2/apis/{ApiId}/integrations` | - | `ApiId` | - | `GetIntegrationsResponse` | `BadRequestException`, `NotFoundException`, `TooManyRequestsException` | Gets the Integrations for an API. |
| `GetModel` | `GET /v2/apis/{ApiId}/models/{ModelId}` | - | `ApiId`, `ModelId` | - | `GetModelResponse` | `NotFoundException`, `TooManyRequestsException` | Gets a Model. |
| `GetModels` | `GET /v2/apis/{ApiId}/models` | - | `ApiId` | - | `GetModelsResponse` | `BadRequestException`, `NotFoundException`, `TooManyRequestsException` | Gets the Models for an API. |
| `GetModelTemplate` | `GET /v2/apis/{ApiId}/models/{ModelId}/template` | - | `ApiId`, `ModelId` | - | `GetModelTemplateResponse` | `NotFoundException`, `TooManyRequestsException` | Gets a model template. |
| `GetPortal` | `GET /v2/portals/{PortalId}` | - | `PortalId` | - | `GetPortalResponse` | `AccessDeniedException`, `BadRequestException`, `NotFoundException`, `TooManyRequestsException` | Gets a portal. |
| `GetPortalProduct` | `GET /v2/portalproducts/{PortalProductId}` | - | `PortalProductId` | - | `GetPortalProductResponse` | `AccessDeniedException`, `BadRequestException`, `NotFoundException`, `TooManyRequestsException` | Gets a portal product. |
| `GetPortalProductSharingPolicy` | `GET /v2/portalproducts/{PortalProductId}/sharingpolicy` | - | `PortalProductId` | - | `GetPortalProductSharingPolicyResponse` | `AccessDeniedException`, `BadRequestException`, `NotFoundException`, `TooManyRequestsException` | Gets the sharing policy for a portal product. |
| `GetProductPage` | `GET /v2/portalproducts/{PortalProductId}/productpages/{ProductPageId}` | - | `PortalProductId`, `ProductPageId` | - | `GetProductPageResponse` | `AccessDeniedException`, `BadRequestException`, `NotFoundException`, `TooManyRequestsException` | Gets a product page of a portal product. |
| `GetProductRestEndpointPage` | `GET /v2/portalproducts/{PortalProductId}/productrestendpointpages/{ProductRestEndpointPageId}` | - | `PortalProductId`, `ProductRestEndpointPageId` | - | `GetProductRestEndpointPageResponse` | `AccessDeniedException`, `BadRequestException`, `NotFoundException`, `TooManyRequestsException` | Gets a product REST endpoint page. |
| `GetRoute` | `GET /v2/apis/{ApiId}/routes/{RouteId}` | - | `ApiId`, `RouteId` | - | `GetRouteResult` | `NotFoundException`, `TooManyRequestsException` | Gets a Route. |
| `GetRouteResponse` | `GET /v2/apis/{ApiId}/routes/{RouteId}/routeresponses/{RouteResponseId}` | - | `ApiId`, `RouteId`, `RouteResponseId` | - | `GetRouteResponseResponse` | `NotFoundException`, `TooManyRequestsException` | Gets a RouteResponse. |
| `GetRouteResponses` | `GET /v2/apis/{ApiId}/routes/{RouteId}/routeresponses` | - | `ApiId`, `RouteId` | - | `GetRouteResponsesResponse` | `BadRequestException`, `NotFoundException`, `TooManyRequestsException` | Gets the RouteResponses for a Route. |
| `GetRoutes` | `GET /v2/apis/{ApiId}/routes` | - | `ApiId` | - | `GetRoutesResponse` | `BadRequestException`, `NotFoundException`, `TooManyRequestsException` | Gets the Routes for an API. |
| `GetRoutingRule` | `GET /v2/domainnames/{DomainName}/routingrules/{RoutingRuleId}` | - | `DomainName`, `RoutingRuleId` | - | `GetRoutingRuleResponse` | `BadRequestException`, `NotFoundException`, `TooManyRequestsException` | Gets a routing rule. |
| `GetStage` | `GET /v2/apis/{ApiId}/stages/{StageName}` | - | `ApiId`, `StageName` | - | `GetStageResponse` | `NotFoundException`, `TooManyRequestsException` | Gets a Stage. |
| `GetStages` | `GET /v2/apis/{ApiId}/stages` | - | `ApiId` | - | `GetStagesResponse` | `BadRequestException`, `NotFoundException`, `TooManyRequestsException` | Gets the Stages for an API. |
| `GetTags` | `GET /v2/tags/{ResourceArn}` | - | `ResourceArn` | - | `GetTagsResponse` | `BadRequestException`, `ConflictException`, `NotFoundException`, `TooManyRequestsException` | Gets a collection of Tag resources. |
| `GetVpcLink` | `GET /v2/vpclinks/{VpcLinkId}` | - | `VpcLinkId` | - | `GetVpcLinkResponse` | `NotFoundException`, `TooManyRequestsException` | Gets a VPC link. |
| `GetVpcLinks` | `GET /v2/vpclinks` | - | - | - | `GetVpcLinksResponse` | `BadRequestException`, `TooManyRequestsException` | Gets a collection of VPC links. |
| `ImportApi` | `PUT /v2/apis` | - | `Body` | - | `ImportApiResponse` | `BadRequestException`, `ConflictException`, `NotFoundException`, `TooManyRequestsException` | Imports an API. |
| `ListPortalProducts` | `GET /v2/portalproducts` | - | - | - | `ListPortalProductsResponse` | `AccessDeniedException`, `BadRequestException`, `TooManyRequestsException` | Lists portal products. |
| `ListPortals` | `GET /v2/portals` | - | - | - | `ListPortalsResponse` | `AccessDeniedException`, `BadRequestException`, `TooManyRequestsException` | Lists portals. |
| `ListProductPages` | `GET /v2/portalproducts/{PortalProductId}/productpages` | - | `PortalProductId` | - | `ListProductPagesResponse` | `AccessDeniedException`, `BadRequestException`, `NotFoundException`, `TooManyRequestsException` | Lists the product pages for a portal product. |
| `ListProductRestEndpointPages` | `GET /v2/portalproducts/{PortalProductId}/productrestendpointpages` | - | `PortalProductId` | - | `ListProductRestEndpointPagesResponse` | `AccessDeniedException`, `BadRequestException`, `NotFoundException`, `TooManyRequestsException` | Lists the product REST endpoint pages of a portal product. |
| `ListRoutingRules` | `GET /v2/domainnames/{DomainName}/routingrules` | `paginated` | `DomainName` | - | `ListRoutingRulesResponse` | `BadRequestException`, `NotFoundException`, `TooManyRequestsException` | Lists routing rules. |
| `PreviewPortal` | `POST /v2/portals/{PortalId}/preview` | - | `PortalId` | - | `PreviewPortalResponse` | `AccessDeniedException`, `BadRequestException`, `ConflictException`, `NotFoundException`, `TooManyRequestsException` | Creates a portal preview. |
| `PublishPortal` | `POST /v2/portals/{PortalId}/publish` | - | `PortalId` | - | `PublishPortalResponse` | `AccessDeniedException`, `BadRequestException`, `ConflictException`, `NotFoundException`, `TooManyRequestsException` | Publishes a portal. |
| `PutPortalProductSharingPolicy` | `PUT /v2/portalproducts/{PortalProductId}/sharingpolicy` | - | `PolicyDocument`, `PortalProductId` | - | `PutPortalProductSharingPolicyResponse` | `AccessDeniedException`, `BadRequestException`, `NotFoundException`, `TooManyRequestsException` | Updates the sharing policy for a portal product. |
| `PutRoutingRule` | `PUT /v2/domainnames/{DomainName}/routingrules/{RoutingRuleId}` | `idempotent` | `Actions`, `Conditions`, `DomainName`, `Priority`, `RoutingRuleId` | - | `PutRoutingRuleResponse` | `BadRequestException`, `ConflictException`, `NotFoundException`, `TooManyRequestsException` | Updates a routing rule. |
| `ReimportApi` | `PUT /v2/apis/{ApiId}` | - | `ApiId`, `Body` | - | `ReimportApiResponse` | `BadRequestException`, `ConflictException`, `NotFoundException`, `TooManyRequestsException` | Puts an Api resource. |
| `ResetAuthorizersCache` | `DELETE /v2/apis/{ApiId}/stages/{StageName}/cache/authorizers` | - | `ApiId`, `StageName` | - | `Unit` | `NotFoundException`, `TooManyRequestsException` | Resets all authorizer cache entries on a stage. Supported only for HTTP APIs. |
| `TagResource` | `POST /v2/tags/{ResourceArn}` | - | `ResourceArn` | - | `TagResourceResponse` | `BadRequestException`, `ConflictException`, `NotFoundException`, `TooManyRequestsException` | Creates a new Tag resource to represent a tag. |
| `UntagResource` | `DELETE /v2/tags/{ResourceArn}` | - | `ResourceArn`, `TagKeys` | - | `Unit` | `BadRequestException`, `ConflictException`, `NotFoundException`, `TooManyRequestsException` | Deletes a Tag. |
| `UpdateApi` | `PATCH /v2/apis/{ApiId}` | - | `ApiId` | - | `UpdateApiResponse` | `BadRequestException`, `ConflictException`, `NotFoundException`, `TooManyRequestsException` | Updates an Api resource. |
| `UpdateApiMapping` | `PATCH /v2/domainnames/{DomainName}/apimappings/{ApiMappingId}` | - | `ApiId`, `ApiMappingId`, `DomainName` | - | `UpdateApiMappingResponse` | `BadRequestException`, `ConflictException`, `NotFoundException`, `TooManyRequestsException` | The API mapping. |
| `UpdateAuthorizer` | `PATCH /v2/apis/{ApiId}/authorizers/{AuthorizerId}` | - | `ApiId`, `AuthorizerId` | - | `UpdateAuthorizerResponse` | `BadRequestException`, `ConflictException`, `NotFoundException`, `TooManyRequestsException` | Updates an Authorizer. |
| `UpdateDeployment` | `PATCH /v2/apis/{ApiId}/deployments/{DeploymentId}` | - | `ApiId`, `DeploymentId` | - | `UpdateDeploymentResponse` | `BadRequestException`, `ConflictException`, `NotFoundException`, `TooManyRequestsException` | Updates a Deployment. |
| `UpdateDomainName` | `PATCH /v2/domainnames/{DomainName}` | - | `DomainName` | - | `UpdateDomainNameResponse` | `BadRequestException`, `ConflictException`, `NotFoundException`, `TooManyRequestsException` | Updates a domain name. |
| `UpdateIntegration` | `PATCH /v2/apis/{ApiId}/integrations/{IntegrationId}` | - | `ApiId`, `IntegrationId` | - | `UpdateIntegrationResult` | `BadRequestException`, `ConflictException`, `NotFoundException`, `TooManyRequestsException` | Updates an Integration. |
| `UpdateIntegrationResponse` | `PATCH /v2/apis/{ApiId}/integrations/{IntegrationId}/integrationresponses/{IntegrationResponseId}` | - | `ApiId`, `IntegrationId`, `IntegrationResponseId` | - | `UpdateIntegrationResponseResponse` | `BadRequestException`, `ConflictException`, `NotFoundException`, `TooManyRequestsException` | Updates an IntegrationResponses. |
| `UpdateModel` | `PATCH /v2/apis/{ApiId}/models/{ModelId}` | - | `ApiId`, `ModelId` | - | `UpdateModelResponse` | `BadRequestException`, `ConflictException`, `NotFoundException`, `TooManyRequestsException` | Updates a Model. |
| `UpdatePortal` | `PATCH /v2/portals/{PortalId}` | - | `PortalId` | - | `UpdatePortalResponse` | `AccessDeniedException`, `BadRequestException`, `ConflictException`, `NotFoundException`, `TooManyRequestsException` | Updates a portal. |
| `UpdatePortalProduct` | `PATCH /v2/portalproducts/{PortalProductId}` | - | `PortalProductId` | - | `UpdatePortalProductResponse` | `AccessDeniedException`, `BadRequestException`, `NotFoundException`, `TooManyRequestsException` | Updates the portal product. |
| `UpdateProductPage` | `PATCH /v2/portalproducts/{PortalProductId}/productpages/{ProductPageId}` | - | `PortalProductId`, `ProductPageId` | - | `UpdateProductPageResponse` | `AccessDeniedException`, `BadRequestException`, `NotFoundException`, `TooManyRequestsException` | Updates a product page of a portal product. |
| `UpdateProductRestEndpointPage` | `PATCH /v2/portalproducts/{PortalProductId}/productrestendpointpages/{ProductRestEndpointPageId}` | - | `PortalProductId`, `ProductRestEndpointPageId` | - | `UpdateProductRestEndpointPageResponse` | `AccessDeniedException`, `BadRequestException`, `NotFoundException`, `TooManyRequestsException` | Updates a product REST endpoint page. |
| `UpdateRoute` | `PATCH /v2/apis/{ApiId}/routes/{RouteId}` | - | `ApiId`, `RouteId` | - | `UpdateRouteResult` | `BadRequestException`, `ConflictException`, `NotFoundException`, `TooManyRequestsException` | Updates a Route. |
| `UpdateRouteResponse` | `PATCH /v2/apis/{ApiId}/routes/{RouteId}/routeresponses/{RouteResponseId}` | - | `ApiId`, `RouteId`, `RouteResponseId` | - | `UpdateRouteResponseResponse` | `BadRequestException`, `ConflictException`, `NotFoundException`, `TooManyRequestsException` | Updates a RouteResponse. |
| `UpdateStage` | `PATCH /v2/apis/{ApiId}/stages/{StageName}` | - | `ApiId`, `StageName` | - | `UpdateStageResponse` | `BadRequestException`, `ConflictException`, `NotFoundException`, `TooManyRequestsException` | Updates a Stage. |
| `UpdateVpcLink` | `PATCH /v2/vpclinks/{VpcLinkId}` | - | `VpcLinkId` | - | `UpdateVpcLinkResponse` | `BadRequestException`, `NotFoundException`, `TooManyRequestsException` | Updates a VPC link. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `CreateRoutingRule` | - | `DomainNameId -> domainNameId` | - | - |
| `DeleteRoutingRule` | - | `DomainNameId -> domainNameId` | - | - |
| `ExportApi` | - | `ExportVersion -> exportVersion`, `IncludeExtensions -> includeExtensions`, `OutputType -> outputType`, `StageName -> stageName` | - | - |
| `GetApiMappings` | - | `MaxResults -> maxResults`, `NextToken -> nextToken` | - | - |
| `GetApis` | - | `MaxResults -> maxResults`, `NextToken -> nextToken` | - | - |
| `GetAuthorizers` | - | `MaxResults -> maxResults`, `NextToken -> nextToken` | - | - |
| `GetDeployments` | - | `MaxResults -> maxResults`, `NextToken -> nextToken` | - | - |
| `GetDomainNames` | - | `MaxResults -> maxResults`, `NextToken -> nextToken` | - | - |
| `GetIntegrationResponses` | - | `MaxResults -> maxResults`, `NextToken -> nextToken` | - | - |
| `GetIntegrations` | - | `MaxResults -> maxResults`, `NextToken -> nextToken` | - | - |
| `GetModels` | - | `MaxResults -> maxResults`, `NextToken -> nextToken` | - | - |
| `GetPortalProduct` | - | `ResourceOwnerAccountId -> resourceOwnerAccountId` | - | - |
| `GetProductPage` | - | `ResourceOwnerAccountId -> resourceOwnerAccountId` | - | - |
| `GetProductRestEndpointPage` | - | `IncludeRawDisplayContent -> includeRawDisplayContent`, `ResourceOwnerAccountId -> resourceOwnerAccountId` | - | - |
| `GetRouteResponses` | - | `MaxResults -> maxResults`, `NextToken -> nextToken` | - | - |
| `GetRoutes` | - | `MaxResults -> maxResults`, `NextToken -> nextToken` | - | - |
| `GetRoutingRule` | - | `DomainNameId -> domainNameId` | - | - |
| `GetStages` | - | `MaxResults -> maxResults`, `NextToken -> nextToken` | - | - |
| `GetVpcLinks` | - | `MaxResults -> maxResults`, `NextToken -> nextToken` | - | - |
| `ImportApi` | - | `Basepath -> basepath`, `FailOnWarnings -> failOnWarnings` | - | - |
| `ListPortalProducts` | - | `MaxResults -> maxResults`, `NextToken -> nextToken`, `ResourceOwner -> resourceOwner` | - | - |
| `ListPortals` | - | `MaxResults -> maxResults`, `NextToken -> nextToken` | - | - |
| `ListProductPages` | - | `MaxResults -> maxResults`, `NextToken -> nextToken`, `ResourceOwnerAccountId -> resourceOwnerAccountId` | - | - |
| `ListProductRestEndpointPages` | - | `MaxResults -> maxResults`, `NextToken -> nextToken`, `ResourceOwnerAccountId -> resourceOwnerAccountId` | - | - |
| `ListRoutingRules` | - | `DomainNameId -> domainNameId`, `MaxResults -> maxResults`, `NextToken -> nextToken` | - | - |
| `PutRoutingRule` | - | `DomainNameId -> domainNameId` | - | - |
| `ReimportApi` | - | `Basepath -> basepath`, `FailOnWarnings -> failOnWarnings` | - | - |
| `UntagResource` | - | `TagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | Message | - |
| `BadRequestException` | `structure` | Message | The request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details. |
| `ConflictException` | `structure` | Message | The requested operation would cause a conflict with the current state of a service resource associated with the request. Resolve the conflict before retryin ... |
| `NotFoundException` | `structure` | Message, ResourceType | The resource specified in the request was not found. See the message field for more information. |
| `TooManyRequestsException` | `structure` | LimitType, Message | A limit has been exceeded. See the accompanying error message for details. |
| `CreateApiRequest` | `structure` | ApiKeySelectionExpression, CorsConfiguration, CredentialsArn, Description, DisableSchemaValidation, DisableExecuteApiEndpoint, IpAddressType, Name, ProtocolType, RouteKey, RouteSelectionExpression, Tags, ... (+2) | Creates a new Api resource to represent an API. |
| `CreateApiResponse` | `structure` | ApiEndpoint, ApiGatewayManaged, ApiId, ApiKeySelectionExpression, CorsConfiguration, CreatedDate, Description, DisableSchemaValidation, DisableExecuteApiEndpoint, ImportInfo, IpAddressType, Name, ... (+5) | - |
| `CreateApiMappingRequest` | `structure` | ApiId, ApiMappingKey, DomainName, Stage | Creates a new ApiMapping resource to represent an API mapping. |
| `CreateApiMappingResponse` | `structure` | ApiId, ApiMappingId, ApiMappingKey, Stage | - |
| `CreateAuthorizerRequest` | `structure` | ApiId, AuthorizerCredentialsArn, AuthorizerPayloadFormatVersion, AuthorizerResultTtlInSeconds, AuthorizerType, AuthorizerUri, EnableSimpleResponses, IdentitySource, IdentityValidationExpression, JwtConfiguration, Name | Creates a new Authorizer resource to represent an authorizer. |
| `CreateAuthorizerResponse` | `structure` | AuthorizerCredentialsArn, AuthorizerId, AuthorizerPayloadFormatVersion, AuthorizerResultTtlInSeconds, AuthorizerType, AuthorizerUri, EnableSimpleResponses, IdentitySource, IdentityValidationExpression, JwtConfiguration, Name | - |
| `CreateDeploymentRequest` | `structure` | ApiId, Description, StageName | Creates a new Deployment resource to represent a deployment. |
| `CreateDeploymentResponse` | `structure` | AutoDeployed, CreatedDate, DeploymentId, DeploymentStatus, DeploymentStatusMessage, Description | - |
| `CreateDomainNameRequest` | `structure` | DomainName, DomainNameConfigurations, MutualTlsAuthentication, RoutingMode, Tags | Creates a new DomainName resource to represent a domain name. |
| `CreateDomainNameResponse` | `structure` | ApiMappingSelectionExpression, DomainName, DomainNameArn, DomainNameConfigurations, MutualTlsAuthentication, RoutingMode, Tags | - |
| `CreateIntegrationRequest` | `structure` | ApiId, ConnectionId, ConnectionType, ContentHandlingStrategy, CredentialsArn, Description, IntegrationMethod, IntegrationSubtype, IntegrationType, IntegrationUri, PassthroughBehavior, PayloadFormatVersion, ... (+6) | Creates a new Integration resource to represent an integration. |
| `CreateIntegrationResult` | `structure` | ApiGatewayManaged, ConnectionId, ConnectionType, ContentHandlingStrategy, CredentialsArn, Description, IntegrationId, IntegrationMethod, IntegrationResponseSelectionExpression, IntegrationSubtype, IntegrationType, IntegrationUri, ... (+8) | - |
| `CreateIntegrationResponseRequest` | `structure` | ApiId, ContentHandlingStrategy, IntegrationId, IntegrationResponseKey, ResponseParameters, ResponseTemplates, TemplateSelectionExpression | Creates a new IntegrationResponse resource to represent an integration response. |
| `CreateIntegrationResponseResponse` | `structure` | ContentHandlingStrategy, IntegrationResponseId, IntegrationResponseKey, ResponseParameters, ResponseTemplates, TemplateSelectionExpression | - |
| `CreateModelRequest` | `structure` | ApiId, ContentType, Description, Name, Schema | Creates a new Model. |
| `CreateModelResponse` | `structure` | ContentType, Description, ModelId, Name, Schema | - |
| `CreatePortalRequest` | `structure` | Authorization, EndpointConfiguration, IncludedPortalProductArns, LogoUri, PortalContent, RumAppMonitorName, Tags | The request body for the post operation. |
| `CreatePortalResponse` | `structure` | Authorization, EndpointConfiguration, IncludedPortalProductArns, LastModified, LastPublished, LastPublishedDescription, PortalArn, PortalContent, PortalId, PublishStatus, RumAppMonitorName, StatusException, ... (+1) | - |
| `CreatePortalProductRequest` | `structure` | Description, DisplayName, Tags | The request body for the post operation. |
| `CreatePortalProductResponse` | `structure` | Description, DisplayName, DisplayOrder, LastModified, PortalProductArn, PortalProductId, Tags | - |
| `CreateProductPageRequest` | `structure` | DisplayContent, PortalProductId | The request body for the post operation. |
| `CreateProductPageResponse` | `structure` | DisplayContent, LastModified, ProductPageArn, ProductPageId | - |
| `CreateProductRestEndpointPageRequest` | `structure` | DisplayContent, PortalProductId, RestEndpointIdentifier, TryItState | The request body for the post operation. |
| `CreateProductRestEndpointPageResponse` | `structure` | DisplayContent, LastModified, ProductRestEndpointPageArn, ProductRestEndpointPageId, RestEndpointIdentifier, Status, StatusException, TryItState | - |
| `CreateRouteRequest` | `structure` | ApiId, ApiKeyRequired, AuthorizationScopes, AuthorizationType, AuthorizerId, ModelSelectionExpression, OperationName, RequestModels, RequestParameters, RouteKey, RouteResponseSelectionExpression, Target | Creates a new Route resource to represent a route. |
| `CreateRouteResult` | `structure` | ApiGatewayManaged, ApiKeyRequired, AuthorizationScopes, AuthorizationType, AuthorizerId, ModelSelectionExpression, OperationName, RequestModels, RequestParameters, RouteId, RouteKey, RouteResponseSelectionExpression, ... (+1) | - |
| `CreateRouteResponseRequest` | `structure` | ApiId, ModelSelectionExpression, ResponseModels, ResponseParameters, RouteId, RouteResponseKey | Creates a new RouteResponse resource to represent a route response. |
| `CreateRouteResponseResponse` | `structure` | ModelSelectionExpression, ResponseModels, ResponseParameters, RouteResponseId, RouteResponseKey | - |
| `CreateRoutingRuleRequest` | `structure` | Actions, Conditions, DomainName, DomainNameId, Priority | - |
| `CreateRoutingRuleResponse` | `structure` | Actions, Conditions, Priority, RoutingRuleArn, RoutingRuleId | - |
| `CreateStageRequest` | `structure` | AccessLogSettings, ApiId, AutoDeploy, ClientCertificateId, DefaultRouteSettings, DeploymentId, Description, RouteSettings, StageName, StageVariables, Tags | Creates a new Stage resource to represent a stage. |
| `CreateStageResponse` | `structure` | AccessLogSettings, ApiGatewayManaged, AutoDeploy, ClientCertificateId, CreatedDate, DefaultRouteSettings, DeploymentId, Description, LastDeploymentStatusMessage, LastUpdatedDate, RouteSettings, StageName, ... (+2) | - |
| `CreateVpcLinkRequest` | `structure` | Name, SecurityGroupIds, SubnetIds, Tags | Creates a VPC link |
| `CreateVpcLinkResponse` | `structure` | CreatedDate, Name, SecurityGroupIds, SubnetIds, Tags, VpcLinkId, VpcLinkStatus, VpcLinkStatusMessage, VpcLinkVersion | - |
| `DeleteAccessLogSettingsRequest` | `structure` | ApiId, StageName | - |
| `AuthorizationType` | `enum` | NONE, AWS_IAM, CUSTOM, JWT | The authorization type. For WebSocket APIs, valid values are NONE for open access, AWS_IAM for using AWS IAM permissions, and CUSTOM for using a Lambda auth ... |
| `AuthorizerType` | `enum` | REQUEST, JWT | The authorizer type. Specify REQUEST for a Lambda function using incoming request parameters. Specify JWT to use JSON Web Tokens (supported only for HTTP APIs). |
| `ConnectionType` | `enum` | INTERNET, VPC_LINK | Represents a connection type. |
| `ContentHandlingStrategy` | `enum` | CONVERT_TO_BINARY, CONVERT_TO_TEXT | Specifies how to handle response payload content type conversions. Supported only for WebSocket APIs. |
| `DeploymentStatus` | `enum` | PENDING, FAILED, DEPLOYED | Represents a deployment status. |
| `DomainNameStatus` | `enum` | AVAILABLE, UPDATING, PENDING_CERTIFICATE_REIMPORT, PENDING_OWNERSHIP_VERIFICATION | The status of the domain name migration. The valid values are AVAILABLE, UPDATING, PENDING_CERTIFICATE_REIMPORT, and PENDING_OWNERSHIP_VERIFICATION. If the ... |
| `EndpointType` | `enum` | REGIONAL, EDGE | Represents an endpoint type. |
| `IntegrationType` | `enum` | AWS, HTTP, MOCK, HTTP_PROXY, AWS_PROXY | Represents an API method integration type. |
| `IpAddressType` | `enum` | ipv4, dualstack | The IP address types that can invoke your API or domain name. |
| `LoggingLevel` | `enum` | ERROR, INFO, OFF | The logging level. |
## Winterbaume LTM Notes

Sources: .agents/docs/LTM/aws-inter-service-integration-priorities.md, .agents/docs/LTM/cross-service-integration-and-engine-boundaries-synthesis.md.

Mode: reference summaries.

- `.agents/docs/LTM/aws-inter-service-integration-priorities.md`: summarises API Gateway V2's front-door role. Open it when modelling Lambda-backed HTTP APIs or AppSync HTTP data-source scenarios.
- `.agents/docs/LTM/cross-service-integration-and-engine-boundaries-synthesis.md`: summarises the boundary between control-plane route resources and request execution. Open it before adding execution semantics to route/integration CRUD.
- Service implication: integration tests should distinguish route/integration resources from execution behaviour and preserve the Lambda payload contract for the selected API type.
- Service implication: AppSync HTTP data sources can target API Gateway backed endpoints, so API Gateway V2 behaviour may become part of AppSync resolver scenarios.

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
