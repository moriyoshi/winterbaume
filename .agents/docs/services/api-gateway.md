# Amazon API Gateway

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon API Gateway Amazon API Gateway helps developers deliver robust, secure, and scalable mobile and web application back ends. API Gateway allows developers to securely connect mobile and web applications to APIs that run on Lambda, Amazon EC2, or other publicly addressable web services that are hosted outside of AWS.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for Amazon API Gateway where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- From the AWS documentation and model: create REST APIs with resources, methods, integrations, deployments, stages, authorisers, API keys, usage plans, and custom domains.
- From the operation surface: model front-door HTTP integration workflows for Lambda, HTTP backends, VPC links, request/response mapping, throttling, access policies, and deployment promotion.

## Service Identity and Protocol

- AWS model slug: `api-gateway`
- AWS SDK for Rust slug: `apigateway`
- Model version: `2015-07-09`
- Model file: `vendor/api-models-aws/models/api-gateway/service/2015-07-09/api-gateway-2015-07-09.json`
- SDK ID: `API Gateway`
- Endpoint prefix: `apigateway`
- ARN namespace: `apigateway`
- CloudFormation name: `ApiGateway`
- CloudTrail event source: `apigateway.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (47), `Delete` (22), `Update` (22), `Create` (16), `Put` (6), `Import` (3), `Flush` (2), `Test` (2).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateApiKey`, `CreateAuthorizer`, `CreateBasePathMapping`, `CreateDeployment`, `CreateDocumentationPart`, `CreateDocumentationVersion`, `CreateDomainName`, `CreateDomainNameAccessAssociation`, `CreateModel`, `CreateRequestValidator`, `CreateResource`, `CreateRestApi`, `CreateStage`, `CreateUsagePlan`, `CreateUsagePlanKey`, `CreateVpcLink`, `DeleteApiKey`, `DeleteAuthorizer`, `DeleteBasePathMapping`, `DeleteClientCertificate`, ... (+52).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetAccount`, `GetApiKey`, `GetApiKeys`, `GetAuthorizer`, `GetAuthorizers`, `GetBasePathMapping`, `GetBasePathMappings`, `GetClientCertificate`, `GetClientCertificates`, `GetDeployment`, `GetDeployments`, `GetDocumentationPart`, `GetDocumentationParts`, `GetDocumentationVersion`, `GetDocumentationVersions`, `GetDomainName`, `GetDomainNameAccessAssociations`, `GetDomainNames`, `GetExport`, `GetGatewayResponse`, ... (+27).
- Pagination is modelled for 12 operations; token stability and page boundaries are observable API behaviour.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `GetExport`, `ImportApiKeys`, `ImportDocumentationParts`, `ImportRestApi`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 124 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `S3`, `Lambda`, `EC2/VPC`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/apigateway/latest/developerguide/how-to-deploy-api.html
- https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-use-lambda-authorizer.html
- https://docs.aws.amazon.com/apigateway/latest/developerguide/how-to-cors.html

Research outcomes:
- A REST API is callable only after a deployment is associated with a stage. Stage names such as dev or prod appear in the invoke URL.
- API configuration changes generally require redeployment before callers observe them. Stage-level settings are the main exception.
- Stages can carry canary release settings, throttling, caching, logging, client certificates, stage variables, and custom-domain base path mappings.
- Lambda authorizers run before the backend method integration and return an IAM policy plus principal information.
- REQUEST authorizer identity sources can include headers, query strings, stage variables, and request context values. When caching is enabled, all configured identity sources form the cache key in order.
- If a cached REQUEST authorizer has a missing, null, or empty identity source, API Gateway can return 401 without invoking the Lambda authorizer.
- TOKEN authorizers use a bearer-token identity source and can apply a regular expression validation step before invoking Lambda.
- Authorizer denies are distinct from authentication failures: denied policies produce 403, while missing or invalid identity commonly produces 401.

Parity implications:
- Model APIs, resources, methods, integrations, deployments, and stages as separate resources. Do not expose draft configuration as deployed behaviour until deployment semantics allow it.
- Treat authorizer evaluation, identity-source validation, policy decisions, and cache keys as first-class request processing steps.
- Stage variables, canary settings, throttling, logging, and custom-domain mappings should influence invocation behaviour rather than remaining inert metadata.

## v1/v2 State Coherence

- **Paired with `apigatewayv2` ( different SDK slug, different API surface ).** API Gateway v1 ( REST APIs ) and API Gateway v2 ( HTTP APIs and WebSocket APIs ) are **separate API products** in real AWS. A REST API created via v1 is not visible to v2 `GetApis`, and an HTTP/WebSocket API created via v2 is not visible to v1 `GetRestApis`. Resources ( methods, integrations, stages, deployments, authorizers ) are scoped under the parent API, so they are also disjoint between the two surfaces.
- **Shared resources to be aware of:**
  - **Custom domain names.** A single custom domain name in an account+region can host both REST APIs ( via v1 base-path mappings, `(domain_name, base_path) → restApiId/stage`) and HTTP APIs ( via v2 API mappings, `(domain_name, mapping_id) → apiId/stage` ). Real AWS reserves the domain name in a single namespace; v1 `CreateDomainName` and v2 `CreateDomainName` cannot both create the same domain.
  - **VPC links.** v1 and v2 each define their own `VpcLink` resource and the lists are not shared in real AWS, so independent state per crate is correct here.
  - **API key / usage plan.** v1-only ( HTTP APIs do not support usage plans ); no coherence concern.
- **Current Winterbaume status: separated, with one observable gap.** Each crate owns its own `domain_names` map. A consumer that creates `example.com` via v1 and then tries to create the same domain via v2 (or vice versa) will succeed in Winterbaume but fail in real AWS. Custom-domain coherence is the realistic follow-up; the rest of the resource models are correctly separate.

## Cross-Service Integration Gaps

- **`apigateway-lambda`** ( primary ): API Gateway and API Gateway V2 store integration endpoints but do not invoke Lambda functions or any other integration target when requests arrive. The first invocation seam to add is Lambda proxy / non-proxy integration; HTTP / HTTP_PROXY and AWS / AWS_PROXY ( other services ) are next. Tracked in `.agents/docs/TODO.md` ( "Cross-Service Integration Gaps" → `apigateway-lambda` ).

## Current Network Resource Stub Semantics

API Gateway currently keeps networking resources inside its own service state.

- REST API endpoint configuration stores `vpcEndpointIds` as a vector of raw strings. The values are echoed through later REST API reads; there is no EC2 VPC endpoint lookup and no check that the endpoint IDs match the REST API region or account.
- VPC links are stored in `ApiGatewayState.vpc_links` keyed by the generated `vpc_link_id`. `CreateVpcLink` records the name, optional description, target ARNs, and tags, and `UpdateVpcLink` only mutates the name and description.
- `VpcLink.status` is service-local state returned by the API Gateway view; target load balancer ARNs are not checked against ELB or ELBv2 state.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Operation Groups

### Get

- Operations: `GetAccount`, `GetApiKey`, `GetApiKeys`, `GetAuthorizer`, `GetAuthorizers`, `GetBasePathMapping`, `GetBasePathMappings`, `GetClientCertificate`, `GetClientCertificates`, `GetDeployment`, `GetDeployments`, `GetDocumentationPart`, `GetDocumentationParts`, `GetDocumentationVersion`, `GetDocumentationVersions`, `GetDomainName`, `GetDomainNameAccessAssociations`, `GetDomainNames`, `GetExport`, `GetGatewayResponse`, `GetGatewayResponses`, `GetIntegration`, `GetIntegrationResponse`, `GetMethod`, `GetMethodResponse`, `GetModel`, `GetModels`, `GetModelTemplate`, `GetRequestValidator`, `GetRequestValidators`, `GetResource`, `GetResources`, `GetRestApi`, `GetRestApis`, `GetSdk`, `GetSdkType`, `GetSdkTypes`, `GetStage`, `GetStages`, `GetTags`, `GetUsage`, `GetUsagePlan`, `GetUsagePlanKey`, `GetUsagePlanKeys`, `GetUsagePlans`, `GetVpcLink`, `GetVpcLinks`
- Traits: `paginated` (12)
- Common required input members in this group: `restApiId`, `domainName`, `stageName`, `resourceId`, `httpMethod`, `statusCode`, `modelName`, `usagePlanId`

### Delete

- Operations: `DeleteApiKey`, `DeleteAuthorizer`, `DeleteBasePathMapping`, `DeleteClientCertificate`, `DeleteDeployment`, `DeleteDocumentationPart`, `DeleteDocumentationVersion`, `DeleteDomainName`, `DeleteDomainNameAccessAssociation`, `DeleteGatewayResponse`, `DeleteIntegration`, `DeleteIntegrationResponse`, `DeleteMethod`, `DeleteMethodResponse`, `DeleteModel`, `DeleteRequestValidator`, `DeleteResource`, `DeleteRestApi`, `DeleteStage`, `DeleteUsagePlan`, `DeleteUsagePlanKey`, `DeleteVpcLink`
- Common required input members in this group: `restApiId`, `domainName`, `resourceId`, `httpMethod`, `statusCode`, `usagePlanId`

### Update

- Operations: `UpdateAccount`, `UpdateApiKey`, `UpdateAuthorizer`, `UpdateBasePathMapping`, `UpdateClientCertificate`, `UpdateDeployment`, `UpdateDocumentationPart`, `UpdateDocumentationVersion`, `UpdateDomainName`, `UpdateGatewayResponse`, `UpdateIntegration`, `UpdateIntegrationResponse`, `UpdateMethod`, `UpdateMethodResponse`, `UpdateModel`, `UpdateRequestValidator`, `UpdateResource`, `UpdateRestApi`, `UpdateStage`, `UpdateUsage`, `UpdateUsagePlan`, `UpdateVpcLink`
- Common required input members in this group: `restApiId`, `domainName`, `resourceId`, `httpMethod`, `statusCode`, `usagePlanId`

### Create

- Operations: `CreateApiKey`, `CreateAuthorizer`, `CreateBasePathMapping`, `CreateDeployment`, `CreateDocumentationPart`, `CreateDocumentationVersion`, `CreateDomainName`, `CreateDomainNameAccessAssociation`, `CreateModel`, `CreateRequestValidator`, `CreateResource`, `CreateRestApi`, `CreateStage`, `CreateUsagePlan`, `CreateUsagePlanKey`, `CreateVpcLink`
- Common required input members in this group: `restApiId`, `name`, `domainName`

### Put

- Operations: `PutGatewayResponse`, `PutIntegration`, `PutIntegrationResponse`, `PutMethod`, `PutMethodResponse`, `PutRestApi`
- Common required input members in this group: `restApiId`, `resourceId`, `httpMethod`, `statusCode`

### Import

- Operations: `ImportApiKeys`, `ImportDocumentationParts`, `ImportRestApi`
- Common required input members in this group: `body`

### Flush

- Operations: `FlushStageAuthorizersCache`, `FlushStageCache`
- Common required input members in this group: `restApiId`, `stageName`

### Test

- Operations: `TestInvokeAuthorizer`, `TestInvokeMethod`
- Common required input members in this group: `restApiId`

### Generate

- Operations: `GenerateClientCertificate`
- Common required input members in this group: -

### Reject

- Operations: `RejectDomainNameAccessAssociation`
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
| `CreateApiKey` | `POST /apikeys` | - | - | - | `ApiKey` | `BadRequestException`, `ConflictException`, `LimitExceededException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Create an ApiKey resource. |
| `CreateAuthorizer` | `POST /restapis/{restApiId}/authorizers` | - | `restApiId`, `name`, `type` | - | `Authorizer` | `BadRequestException`, `ConflictException`, `LimitExceededException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Adds a new Authorizer resource to an existing RestApi resource. |
| `CreateBasePathMapping` | `POST /domainnames/{domainName}/basepathmappings` | - | `domainName`, `restApiId` | - | `BasePathMapping` | `BadRequestException`, `ConflictException`, `LimitExceededException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Creates a new BasePathMapping resource. |
| `CreateDeployment` | `POST /restapis/{restApiId}/deployments` | - | `restApiId` | - | `Deployment` | `BadRequestException`, `ConflictException`, `LimitExceededException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException`, `UnauthorizedException` | Creates a Deployment resource, which makes a specified RestApi callable over the internet. |
| `CreateDocumentationPart` | `POST /restapis/{restApiId}/documentation/parts` | - | `restApiId`, `location`, `properties` | - | `DocumentationPart` | `BadRequestException`, `ConflictException`, `LimitExceededException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Creates a documentation part. |
| `CreateDocumentationVersion` | `POST /restapis/{restApiId}/documentation/versions` | - | `restApiId`, `documentationVersion` | - | `DocumentationVersion` | `BadRequestException`, `ConflictException`, `LimitExceededException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Creates a documentation version |
| `CreateDomainName` | `POST /domainnames` | - | `domainName` | - | `DomainName` | `BadRequestException`, `ConflictException`, `LimitExceededException`, `TooManyRequestsException`, `UnauthorizedException` | Creates a new domain name. |
| `CreateDomainNameAccessAssociation` | `POST /domainnameaccessassociations` | - | `domainNameArn`, `accessAssociationSourceType`, `accessAssociationSource` | - | `DomainNameAccessAssociation` | `BadRequestException`, `ConflictException`, `LimitExceededException`, `TooManyRequestsException`, `UnauthorizedException` | Creates a domain name access association resource between an access association source and a private custom domain name. |
| `CreateModel` | `POST /restapis/{restApiId}/models` | - | `restApiId`, `name`, `contentType` | - | `Model` | `BadRequestException`, `ConflictException`, `LimitExceededException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Adds a new Model resource to an existing RestApi resource. |
| `CreateRequestValidator` | `POST /restapis/{restApiId}/requestvalidators` | - | `restApiId` | - | `RequestValidator` | `BadRequestException`, `ConflictException`, `LimitExceededException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Creates a RequestValidator of a given RestApi. |
| `CreateResource` | `POST /restapis/{restApiId}/resources/{parentId}` | - | `restApiId`, `parentId`, `pathPart` | - | `Resource` | `BadRequestException`, `ConflictException`, `LimitExceededException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Creates a Resource resource. |
| `CreateRestApi` | `POST /restapis` | - | `name` | - | `RestApi` | `BadRequestException`, `ConflictException`, `LimitExceededException`, `TooManyRequestsException`, `UnauthorizedException` | Creates a new RestApi resource. |
| `CreateStage` | `POST /restapis/{restApiId}/stages` | - | `restApiId`, `stageName`, `deploymentId` | - | `Stage` | `BadRequestException`, `ConflictException`, `LimitExceededException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Creates a new Stage resource that references a pre-existing Deployment for the API. |
| `CreateUsagePlan` | `POST /usageplans` | - | `name` | - | `UsagePlan` | `BadRequestException`, `ConflictException`, `LimitExceededException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Creates a usage plan with the throttle and quota limits, as well as the associated API stages, specified in the payload. |
| `CreateUsagePlanKey` | `POST /usageplans/{usagePlanId}/keys` | - | `usagePlanId`, `keyId`, `keyType` | - | `UsagePlanKey` | `BadRequestException`, `ConflictException`, `LimitExceededException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Creates a usage plan key for adding an existing API key to a usage plan. |
| `CreateVpcLink` | `POST /vpclinks` | - | `name`, `targetArns` | - | `VpcLink` | `BadRequestException`, `ConflictException`, `LimitExceededException`, `TooManyRequestsException`, `UnauthorizedException` | Creates a VPC link, under the caller's account in a selected region, in an asynchronous operation that typically takes 2-4 minutes to complete and become operational. The caller must have permissions to create and up ... |
| `DeleteApiKey` | `DELETE /apikeys/{apiKey}` | - | `apiKey` | - | `Unit` | `BadRequestException`, `ConflictException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Deletes the ApiKey resource. |
| `DeleteAuthorizer` | `DELETE /restapis/{restApiId}/authorizers/{authorizerId}` | - | `restApiId`, `authorizerId` | - | `Unit` | `BadRequestException`, `ConflictException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Deletes an existing Authorizer resource. |
| `DeleteBasePathMapping` | `DELETE /domainnames/{domainName}/basepathmappings/{basePath}` | - | `domainName`, `basePath` | - | `Unit` | `BadRequestException`, `ConflictException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Deletes the BasePathMapping resource. |
| `DeleteClientCertificate` | `DELETE /clientcertificates/{clientCertificateId}` | - | `clientCertificateId` | - | `Unit` | `BadRequestException`, `ConflictException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Deletes the ClientCertificate resource. |
| `DeleteDeployment` | `DELETE /restapis/{restApiId}/deployments/{deploymentId}` | - | `restApiId`, `deploymentId` | - | `Unit` | `BadRequestException`, `ConflictException`, `LimitExceededException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Deletes a Deployment resource. Deleting a deployment will only succeed if there are no Stage resources associated with it. |
| `DeleteDocumentationPart` | `DELETE /restapis/{restApiId}/documentation/parts/{documentationPartId}` | - | `restApiId`, `documentationPartId` | - | `Unit` | `BadRequestException`, `ConflictException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Deletes a documentation part |
| `DeleteDocumentationVersion` | `DELETE /restapis/{restApiId}/documentation/versions/{documentationVersion}` | - | `restApiId`, `documentationVersion` | - | `Unit` | `BadRequestException`, `ConflictException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Deletes a documentation version. |
| `DeleteDomainName` | `DELETE /domainnames/{domainName}` | - | `domainName` | - | `Unit` | `BadRequestException`, `ConflictException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Deletes the DomainName resource. |
| `DeleteDomainNameAccessAssociation` | `DELETE /domainnameaccessassociations/{domainNameAccessAssociationArn}` | - | `domainNameAccessAssociationArn` | - | `Unit` | `BadRequestException`, `ConflictException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Deletes the DomainNameAccessAssociation resource. Only the AWS account that created the DomainNameAccessAssociation resource can delete it. To stop an access association source in another AWS account from accessing y ... |
| `DeleteGatewayResponse` | `DELETE /restapis/{restApiId}/gatewayresponses/{responseType}` | - | `restApiId`, `responseType` | - | `Unit` | `BadRequestException`, `ConflictException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Clears any customization of a GatewayResponse of a specified response type on the given RestApi and resets it with the default settings. |
| `DeleteIntegration` | `DELETE /restapis/{restApiId}/resources/{resourceId}/methods/{httpMethod}/integration` | - | `restApiId`, `resourceId`, `httpMethod` | - | `Unit` | `BadRequestException`, `ConflictException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Represents a delete integration. |
| `DeleteIntegrationResponse` | `DELETE /restapis/{restApiId}/resources/{resourceId}/methods/{httpMethod}/integration/responses/{statusCode}` | - | `restApiId`, `resourceId`, `httpMethod`, `statusCode` | - | `Unit` | `BadRequestException`, `ConflictException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Represents a delete integration response. |
| `DeleteMethod` | `DELETE /restapis/{restApiId}/resources/{resourceId}/methods/{httpMethod}` | - | `restApiId`, `resourceId`, `httpMethod` | - | `Unit` | `ConflictException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Deletes an existing Method resource. |
| `DeleteMethodResponse` | `DELETE /restapis/{restApiId}/resources/{resourceId}/methods/{httpMethod}/responses/{statusCode}` | - | `restApiId`, `resourceId`, `httpMethod`, `statusCode` | - | `Unit` | `BadRequestException`, `ConflictException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Deletes an existing MethodResponse resource. |
| `DeleteModel` | `DELETE /restapis/{restApiId}/models/{modelName}` | - | `restApiId`, `modelName` | - | `Unit` | `BadRequestException`, `ConflictException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Deletes a model. |
| `DeleteRequestValidator` | `DELETE /restapis/{restApiId}/requestvalidators/{requestValidatorId}` | - | `restApiId`, `requestValidatorId` | - | `Unit` | `BadRequestException`, `ConflictException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Deletes a RequestValidator of a given RestApi. |
| `DeleteResource` | `DELETE /restapis/{restApiId}/resources/{resourceId}` | - | `restApiId`, `resourceId` | - | `Unit` | `BadRequestException`, `ConflictException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Deletes a Resource resource. |
| `DeleteRestApi` | `DELETE /restapis/{restApiId}` | - | `restApiId` | - | `Unit` | `BadRequestException`, `ConflictException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Deletes the specified API. |
| `DeleteStage` | `DELETE /restapis/{restApiId}/stages/{stageName}` | - | `restApiId`, `stageName` | - | `Unit` | `BadRequestException`, `ConflictException`, `LimitExceededException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Deletes a Stage resource. |
| `DeleteUsagePlan` | `DELETE /usageplans/{usagePlanId}` | - | `usagePlanId` | - | `Unit` | `BadRequestException`, `ConflictException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Deletes a usage plan of a given plan Id. |
| `DeleteUsagePlanKey` | `DELETE /usageplans/{usagePlanId}/keys/{keyId}` | - | `usagePlanId`, `keyId` | - | `Unit` | `BadRequestException`, `ConflictException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Deletes a usage plan key and remove the underlying API key from the associated usage plan. |
| `DeleteVpcLink` | `DELETE /vpclinks/{vpcLinkId}` | - | `vpcLinkId` | - | `Unit` | `BadRequestException`, `ConflictException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Deletes an existing VpcLink of a specified identifier. |
| `FlushStageAuthorizersCache` | `DELETE /restapis/{restApiId}/stages/{stageName}/cache/authorizers` | - | `restApiId`, `stageName` | - | `Unit` | `BadRequestException`, `ConflictException`, `LimitExceededException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Flushes all authorizer cache entries on a stage. |
| `FlushStageCache` | `DELETE /restapis/{restApiId}/stages/{stageName}/cache/data` | - | `restApiId`, `stageName` | - | `Unit` | `BadRequestException`, `ConflictException`, `LimitExceededException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Flushes a stage's cache. |
| `GenerateClientCertificate` | `POST /clientcertificates` | - | - | - | `ClientCertificate` | `BadRequestException`, `ConflictException`, `LimitExceededException`, `TooManyRequestsException`, `UnauthorizedException` | Generates a ClientCertificate resource. |
| `GetAccount` | `GET /account` | - | - | - | `Account` | `BadRequestException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Gets information about the current Account resource. |
| `GetApiKey` | `GET /apikeys/{apiKey}` | - | `apiKey` | - | `ApiKey` | `BadRequestException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Gets information about the current ApiKey resource. |
| `GetApiKeys` | `GET /apikeys` | `paginated` | - | - | `ApiKeys` | `BadRequestException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Gets information about the current ApiKeys resource. |
| `GetAuthorizer` | `GET /restapis/{restApiId}/authorizers/{authorizerId}` | - | `restApiId`, `authorizerId` | - | `Authorizer` | `BadRequestException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Describe an existing Authorizer resource. |
| `GetAuthorizers` | `GET /restapis/{restApiId}/authorizers` | - | `restApiId` | - | `Authorizers` | `BadRequestException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Describe an existing Authorizers resource. |
| `GetBasePathMapping` | `GET /domainnames/{domainName}/basepathmappings/{basePath}` | - | `domainName`, `basePath` | - | `BasePathMapping` | `BadRequestException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Describe a BasePathMapping resource. |
| `GetBasePathMappings` | `GET /domainnames/{domainName}/basepathmappings` | `paginated` | `domainName` | - | `BasePathMappings` | `BadRequestException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Represents a collection of BasePathMapping resources. |
| `GetClientCertificate` | `GET /clientcertificates/{clientCertificateId}` | - | `clientCertificateId` | - | `ClientCertificate` | `BadRequestException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Gets information about the current ClientCertificate resource. |
| `GetClientCertificates` | `GET /clientcertificates` | `paginated` | - | - | `ClientCertificates` | `BadRequestException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Gets a collection of ClientCertificate resources. |
| `GetDeployment` | `GET /restapis/{restApiId}/deployments/{deploymentId}` | - | `restApiId`, `deploymentId` | - | `Deployment` | `BadRequestException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException`, `UnauthorizedException` | Gets information about a Deployment resource. |
| `GetDeployments` | `GET /restapis/{restApiId}/deployments` | `paginated` | `restApiId` | - | `Deployments` | `BadRequestException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException`, `UnauthorizedException` | Gets information about a Deployments collection. |
| `GetDocumentationPart` | `GET /restapis/{restApiId}/documentation/parts/{documentationPartId}` | - | `restApiId`, `documentationPartId` | - | `DocumentationPart` | `BadRequestException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Gets a documentation part. |
| `GetDocumentationParts` | `GET /restapis/{restApiId}/documentation/parts` | - | `restApiId` | - | `DocumentationParts` | `BadRequestException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Gets documentation parts. |
| `GetDocumentationVersion` | `GET /restapis/{restApiId}/documentation/versions/{documentationVersion}` | - | `restApiId`, `documentationVersion` | - | `DocumentationVersion` | `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Gets a documentation version. |
| `GetDocumentationVersions` | `GET /restapis/{restApiId}/documentation/versions` | - | `restApiId` | - | `DocumentationVersions` | `BadRequestException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Gets documentation versions. |
| `GetDomainName` | `GET /domainnames/{domainName}` | - | `domainName` | - | `DomainName` | `BadRequestException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Represents a domain name that is contained in a simpler, more intuitive URL that can be called. |
| `GetDomainNameAccessAssociations` | `GET /domainnameaccessassociations` | - | - | - | `DomainNameAccessAssociations` | `BadRequestException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Represents a collection on DomainNameAccessAssociations resources. |
| `GetDomainNames` | `GET /domainnames` | `paginated` | - | - | `DomainNames` | `BadRequestException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Represents a collection of DomainName resources. |
| `GetExport` | `GET /restapis/{restApiId}/stages/{stageName}/exports/{exportType}` | - | `restApiId`, `stageName`, `exportType` | - | `ExportResponse` | `BadRequestException`, `ConflictException`, `LimitExceededException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Exports a deployed version of a RestApi in a specified format. |
| `GetGatewayResponse` | `GET /restapis/{restApiId}/gatewayresponses/{responseType}` | - | `restApiId`, `responseType` | - | `GatewayResponse` | `BadRequestException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Gets a GatewayResponse of a specified response type on the given RestApi. |
| `GetGatewayResponses` | `GET /restapis/{restApiId}/gatewayresponses` | - | `restApiId` | - | `GatewayResponses` | `BadRequestException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Gets the GatewayResponses collection on the given RestApi. If an API developer has not added any definitions for gateway responses, the result will be the API Gateway-generated default GatewayResponses collection for ... |
| `GetIntegration` | `GET /restapis/{restApiId}/resources/{resourceId}/methods/{httpMethod}/integration` | - | `restApiId`, `resourceId`, `httpMethod` | - | `Integration` | `BadRequestException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Get the integration settings. |
| `GetIntegrationResponse` | `GET /restapis/{restApiId}/resources/{resourceId}/methods/{httpMethod}/integration/responses/{statusCode}` | - | `restApiId`, `resourceId`, `httpMethod`, `statusCode` | - | `IntegrationResponse` | `BadRequestException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Represents a get integration response. |
| `GetMethod` | `GET /restapis/{restApiId}/resources/{resourceId}/methods/{httpMethod}` | - | `restApiId`, `resourceId`, `httpMethod` | - | `Method` | `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Describe an existing Method resource. |
| `GetMethodResponse` | `GET /restapis/{restApiId}/resources/{resourceId}/methods/{httpMethod}/responses/{statusCode}` | - | `restApiId`, `resourceId`, `httpMethod`, `statusCode` | - | `MethodResponse` | `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Describes a MethodResponse resource. |
| `GetModel` | `GET /restapis/{restApiId}/models/{modelName}` | - | `restApiId`, `modelName` | - | `Model` | `BadRequestException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Describes an existing model defined for a RestApi resource. |
| `GetModels` | `GET /restapis/{restApiId}/models` | `paginated` | `restApiId` | - | `Models` | `BadRequestException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Describes existing Models defined for a RestApi resource. |
| `GetModelTemplate` | `GET /restapis/{restApiId}/models/{modelName}/default_template` | - | `restApiId`, `modelName` | - | `Template` | `BadRequestException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Generates a sample mapping template that can be used to transform a payload into the structure of a model. |
| `GetRequestValidator` | `GET /restapis/{restApiId}/requestvalidators/{requestValidatorId}` | - | `restApiId`, `requestValidatorId` | - | `RequestValidator` | `BadRequestException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Gets a RequestValidator of a given RestApi. |
| `GetRequestValidators` | `GET /restapis/{restApiId}/requestvalidators` | - | `restApiId` | - | `RequestValidators` | `BadRequestException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Gets the RequestValidators collection of a given RestApi. |
| `GetResource` | `GET /restapis/{restApiId}/resources/{resourceId}` | - | `restApiId`, `resourceId` | - | `Resource` | `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Lists information about a resource. |
| `GetResources` | `GET /restapis/{restApiId}/resources` | `paginated` | `restApiId` | - | `Resources` | `BadRequestException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Lists information about a collection of Resource resources. |
| `GetRestApi` | `GET /restapis/{restApiId}` | - | `restApiId` | - | `RestApi` | `BadRequestException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Lists the RestApi resource in the collection. |
| `GetRestApis` | `GET /restapis` | `paginated` | - | - | `RestApis` | `BadRequestException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Lists the RestApis resources for your collection. |
| `GetSdk` | `GET /restapis/{restApiId}/stages/{stageName}/sdks/{sdkType}` | - | `restApiId`, `stageName`, `sdkType` | - | `SdkResponse` | `BadRequestException`, `ConflictException`, `LimitExceededException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Generates a client SDK for a RestApi and Stage. |
| `GetSdkType` | `GET /sdktypes/{id}` | - | `id` | - | `SdkType` | `BadRequestException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Gets an SDK type. |
| `GetSdkTypes` | `GET /sdktypes` | - | - | - | `SdkTypes` | `BadRequestException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Gets SDK types |
| `GetStage` | `GET /restapis/{restApiId}/stages/{stageName}` | - | `restApiId`, `stageName` | - | `Stage` | `BadRequestException`, `ConflictException`, `LimitExceededException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Gets information about a Stage resource. |
| `GetStages` | `GET /restapis/{restApiId}/stages` | - | `restApiId` | - | `Stages` | `BadRequestException`, `ConflictException`, `LimitExceededException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Gets information about one or more Stage resources. |
| `GetTags` | `GET /tags/{resourceArn}` | - | `resourceArn` | - | `Tags` | `BadRequestException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Gets the Tags collection for a given resource. |
| `GetUsage` | `GET /usageplans/{usagePlanId}/usage` | `paginated` | `usagePlanId`, `startDate`, `endDate` | - | `Usage` | `BadRequestException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Gets the usage data of a usage plan in a specified time interval. |
| `GetUsagePlan` | `GET /usageplans/{usagePlanId}` | - | `usagePlanId` | - | `UsagePlan` | `BadRequestException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Gets a usage plan of a given plan identifier. |
| `GetUsagePlanKey` | `GET /usageplans/{usagePlanId}/keys/{keyId}` | - | `usagePlanId`, `keyId` | - | `UsagePlanKey` | `BadRequestException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Gets a usage plan key of a given key identifier. |
| `GetUsagePlanKeys` | `GET /usageplans/{usagePlanId}/keys` | `paginated` | `usagePlanId` | - | `UsagePlanKeys` | `BadRequestException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Gets all the usage plan keys representing the API keys added to a specified usage plan. |
| `GetUsagePlans` | `GET /usageplans` | `paginated` | - | - | `UsagePlans` | `BadRequestException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Gets all the usage plans of the caller's account. |
| `GetVpcLink` | `GET /vpclinks/{vpcLinkId}` | - | `vpcLinkId` | - | `VpcLink` | `BadRequestException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Gets a specified VPC link under the caller's account in a region. |
| `GetVpcLinks` | `GET /vpclinks` | `paginated` | - | - | `VpcLinks` | `BadRequestException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Gets the VpcLinks collection under the caller's account in a selected region. |
| `ImportApiKeys` | `POST /apikeys?mode=import` | - | `body`, `format` | - | `ApiKeyIds` | `BadRequestException`, `ConflictException`, `LimitExceededException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Import API keys from an external source, such as a CSV-formatted file. |
| `ImportDocumentationParts` | `PUT /restapis/{restApiId}/documentation/parts` | - | `restApiId`, `body` | - | `DocumentationPartIds` | `BadRequestException`, `ConflictException`, `LimitExceededException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Imports documentation parts |
| `ImportRestApi` | `POST /restapis?mode=import` | - | `body` | - | `RestApi` | `BadRequestException`, `ConflictException`, `LimitExceededException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | A feature of the API Gateway control service for creating a new API from an external API definition file. |
| `PutGatewayResponse` | `PUT /restapis/{restApiId}/gatewayresponses/{responseType}` | - | `restApiId`, `responseType` | - | `GatewayResponse` | `BadRequestException`, `ConflictException`, `LimitExceededException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Creates a customization of a GatewayResponse of a specified response type and status code on the given RestApi. |
| `PutIntegration` | `PUT /restapis/{restApiId}/resources/{resourceId}/methods/{httpMethod}/integration` | - | `restApiId`, `resourceId`, `httpMethod`, `type` | - | `Integration` | `BadRequestException`, `ConflictException`, `LimitExceededException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Sets up a method's integration. |
| `PutIntegrationResponse` | `PUT /restapis/{restApiId}/resources/{resourceId}/methods/{httpMethod}/integration/responses/{statusCode}` | - | `restApiId`, `resourceId`, `httpMethod`, `statusCode` | - | `IntegrationResponse` | `BadRequestException`, `ConflictException`, `LimitExceededException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Represents a put integration. |
| `PutMethod` | `PUT /restapis/{restApiId}/resources/{resourceId}/methods/{httpMethod}` | - | `restApiId`, `resourceId`, `httpMethod`, `authorizationType` | - | `Method` | `BadRequestException`, `ConflictException`, `LimitExceededException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Add a method to an existing Resource resource. |
| `PutMethodResponse` | `PUT /restapis/{restApiId}/resources/{resourceId}/methods/{httpMethod}/responses/{statusCode}` | - | `restApiId`, `resourceId`, `httpMethod`, `statusCode` | - | `MethodResponse` | `BadRequestException`, `ConflictException`, `LimitExceededException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Adds a MethodResponse to an existing Method resource. |
| `PutRestApi` | `PUT /restapis/{restApiId}` | - | `restApiId`, `body` | - | `RestApi` | `BadRequestException`, `ConflictException`, `LimitExceededException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | A feature of the API Gateway control service for updating an existing API with an input of external API definitions. The update can take the form of merging the supplied definition into the existing API or overwritin ... |
| `RejectDomainNameAccessAssociation` | `POST /rejectdomainnameaccessassociations` | - | `domainNameAccessAssociationArn`, `domainNameArn` | - | `Unit` | `BadRequestException`, `ConflictException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Rejects a domain name access association with a private custom domain name. To reject a domain name access association with an access association source in another AWS account, use this operation. To remove a domain ... |
| `TagResource` | `PUT /tags/{resourceArn}` | - | `resourceArn`, `tags` | - | `Unit` | `BadRequestException`, `ConflictException`, `LimitExceededException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Adds or updates a tag on a given resource. |
| `TestInvokeAuthorizer` | `POST /restapis/{restApiId}/authorizers/{authorizerId}` | - | `restApiId`, `authorizerId` | - | `TestInvokeAuthorizerResponse` | `BadRequestException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Simulate the execution of an Authorizer in your RestApi with headers, parameters, and an incoming request body. |
| `TestInvokeMethod` | `POST /restapis/{restApiId}/resources/{resourceId}/methods/{httpMethod}` | - | `restApiId`, `resourceId`, `httpMethod` | - | `TestInvokeMethodResponse` | `BadRequestException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Simulate the invocation of a Method in your RestApi with headers, parameters, and an incoming request body. |
| `UntagResource` | `DELETE /tags/{resourceArn}` | - | `resourceArn`, `tagKeys` | - | `Unit` | `BadRequestException`, `ConflictException`, `LimitExceededException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Removes a tag from a given resource. |
| `UpdateAccount` | `PATCH /account` | - | - | - | `Account` | `BadRequestException`, `ConflictException`, `LimitExceededException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Changes information about the current Account resource. |
| `UpdateApiKey` | `PATCH /apikeys/{apiKey}` | - | `apiKey` | - | `ApiKey` | `BadRequestException`, `ConflictException`, `LimitExceededException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Changes information about an ApiKey resource. |
| `UpdateAuthorizer` | `PATCH /restapis/{restApiId}/authorizers/{authorizerId}` | - | `restApiId`, `authorizerId` | - | `Authorizer` | `BadRequestException`, `ConflictException`, `LimitExceededException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Updates an existing Authorizer resource. |
| `UpdateBasePathMapping` | `PATCH /domainnames/{domainName}/basepathmappings/{basePath}` | - | `domainName`, `basePath` | - | `BasePathMapping` | `BadRequestException`, `ConflictException`, `LimitExceededException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Changes information about the BasePathMapping resource. |
| `UpdateClientCertificate` | `PATCH /clientcertificates/{clientCertificateId}` | - | `clientCertificateId` | - | `ClientCertificate` | `BadRequestException`, `ConflictException`, `LimitExceededException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Changes information about an ClientCertificate resource. |
| `UpdateDeployment` | `PATCH /restapis/{restApiId}/deployments/{deploymentId}` | - | `restApiId`, `deploymentId` | - | `Deployment` | `BadRequestException`, `ConflictException`, `LimitExceededException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException`, `UnauthorizedException` | Changes information about a Deployment resource. |
| `UpdateDocumentationPart` | `PATCH /restapis/{restApiId}/documentation/parts/{documentationPartId}` | - | `restApiId`, `documentationPartId` | - | `DocumentationPart` | `BadRequestException`, `ConflictException`, `LimitExceededException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Updates a documentation part. |
| `UpdateDocumentationVersion` | `PATCH /restapis/{restApiId}/documentation/versions/{documentationVersion}` | - | `restApiId`, `documentationVersion` | - | `DocumentationVersion` | `BadRequestException`, `ConflictException`, `LimitExceededException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Updates a documentation version. |
| `UpdateDomainName` | `PATCH /domainnames/{domainName}` | - | `domainName` | - | `DomainName` | `BadRequestException`, `ConflictException`, `LimitExceededException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Changes information about the DomainName resource. |
| `UpdateGatewayResponse` | `PATCH /restapis/{restApiId}/gatewayresponses/{responseType}` | - | `restApiId`, `responseType` | - | `GatewayResponse` | `BadRequestException`, `ConflictException`, `LimitExceededException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Updates a GatewayResponse of a specified response type on the given RestApi. |
| `UpdateIntegration` | `PATCH /restapis/{restApiId}/resources/{resourceId}/methods/{httpMethod}/integration` | - | `restApiId`, `resourceId`, `httpMethod` | - | `Integration` | `BadRequestException`, `ConflictException`, `LimitExceededException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Represents an update integration. |
| `UpdateIntegrationResponse` | `PATCH /restapis/{restApiId}/resources/{resourceId}/methods/{httpMethod}/integration/responses/{statusCode}` | - | `restApiId`, `resourceId`, `httpMethod`, `statusCode` | - | `IntegrationResponse` | `BadRequestException`, `ConflictException`, `LimitExceededException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Represents an update integration response. |
| `UpdateMethod` | `PATCH /restapis/{restApiId}/resources/{resourceId}/methods/{httpMethod}` | - | `restApiId`, `resourceId`, `httpMethod` | - | `Method` | `BadRequestException`, `ConflictException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Updates an existing Method resource. |
| `UpdateMethodResponse` | `PATCH /restapis/{restApiId}/resources/{resourceId}/methods/{httpMethod}/responses/{statusCode}` | - | `restApiId`, `resourceId`, `httpMethod`, `statusCode` | - | `MethodResponse` | `BadRequestException`, `ConflictException`, `LimitExceededException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Updates an existing MethodResponse resource. |
| `UpdateModel` | `PATCH /restapis/{restApiId}/models/{modelName}` | - | `restApiId`, `modelName` | - | `Model` | `BadRequestException`, `ConflictException`, `LimitExceededException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Changes information about a model. The maximum size of the model is 400 KB. |
| `UpdateRequestValidator` | `PATCH /restapis/{restApiId}/requestvalidators/{requestValidatorId}` | - | `restApiId`, `requestValidatorId` | - | `RequestValidator` | `BadRequestException`, `ConflictException`, `LimitExceededException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Updates a RequestValidator of a given RestApi. |
| `UpdateResource` | `PATCH /restapis/{restApiId}/resources/{resourceId}` | - | `restApiId`, `resourceId` | - | `Resource` | `BadRequestException`, `ConflictException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Changes information about a Resource resource. |
| `UpdateRestApi` | `PATCH /restapis/{restApiId}` | - | `restApiId` | - | `RestApi` | `BadRequestException`, `ConflictException`, `LimitExceededException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Changes information about the specified API. |
| `UpdateStage` | `PATCH /restapis/{restApiId}/stages/{stageName}` | - | `restApiId`, `stageName` | - | `Stage` | `BadRequestException`, `ConflictException`, `LimitExceededException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Changes information about a Stage resource. |
| `UpdateUsage` | `PATCH /usageplans/{usagePlanId}/keys/{keyId}/usage` | - | `usagePlanId`, `keyId` | - | `Usage` | `BadRequestException`, `ConflictException`, `LimitExceededException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Grants a temporary extension to the remaining quota of a usage plan associated with a specified API key. |
| `UpdateUsagePlan` | `PATCH /usageplans/{usagePlanId}` | - | `usagePlanId` | - | `UsagePlan` | `BadRequestException`, `ConflictException`, `LimitExceededException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Updates a usage plan of a given plan Id. |
| `UpdateVpcLink` | `PATCH /vpclinks/{vpcLinkId}` | - | `vpcLinkId` | - | `VpcLink` | `BadRequestException`, `ConflictException`, `LimitExceededException`, `NotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Updates an existing VpcLink of a specified identifier. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `CreateBasePathMapping` | - | `domainNameId -> domainNameId` | - | - |
| `DeleteBasePathMapping` | - | `domainNameId -> domainNameId` | - | - |
| `DeleteDomainName` | - | `domainNameId -> domainNameId` | - | - |
| `GetApiKey` | - | `includeValue -> includeValue` | - | - |
| `GetApiKeys` | - | `position -> position`, `limit -> limit`, `nameQuery -> name`, `customerId -> customerId`, `includeValues -> includeValues` | - | - |
| `GetAuthorizers` | - | `position -> position`, `limit -> limit` | - | - |
| `GetBasePathMapping` | - | `domainNameId -> domainNameId` | - | - |
| `GetBasePathMappings` | - | `domainNameId -> domainNameId`, `position -> position`, `limit -> limit` | - | - |
| `GetClientCertificates` | - | `position -> position`, `limit -> limit` | - | - |
| `GetDeployment` | - | `embed -> embed` | - | - |
| `GetDeployments` | - | `position -> position`, `limit -> limit` | - | - |
| `GetDocumentationParts` | - | `type -> type`, `nameQuery -> name`, `path -> path`, `position -> position`, `limit -> limit`, `locationStatus -> locationStatus` | - | - |
| `GetDocumentationVersions` | - | `position -> position`, `limit -> limit` | - | - |
| `GetDomainName` | - | `domainNameId -> domainNameId` | - | - |
| `GetDomainNameAccessAssociations` | - | `position -> position`, `limit -> limit`, `resourceOwner -> resourceOwner` | - | - |
| `GetDomainNames` | - | `position -> position`, `limit -> limit`, `resourceOwner -> resourceOwner` | - | - |
| `GetExport` | `accepts -> Accept` | - | - | - |
| `GetGatewayResponses` | - | `position -> position`, `limit -> limit` | - | - |
| `GetModel` | - | `flatten -> flatten` | - | - |
| `GetModels` | - | `position -> position`, `limit -> limit` | - | - |
| `GetRequestValidators` | - | `position -> position`, `limit -> limit` | - | - |
| `GetResource` | - | `embed -> embed` | - | - |
| `GetResources` | - | `position -> position`, `limit -> limit`, `embed -> embed` | - | - |
| `GetRestApis` | - | `position -> position`, `limit -> limit` | - | - |
| `GetSdkTypes` | - | `position -> position`, `limit -> limit` | - | - |
| `GetStages` | - | `deploymentId -> deploymentId` | - | - |
| `GetTags` | - | `position -> position`, `limit -> limit` | - | - |
| `GetUsage` | - | `keyId -> keyId`, `startDate -> startDate`, `endDate -> endDate`, `position -> position`, `limit -> limit` | - | - |
| `GetUsagePlanKeys` | - | `position -> position`, `limit -> limit`, `nameQuery -> name` | - | - |
| `GetUsagePlans` | - | `position -> position`, `keyId -> keyId`, `limit -> limit` | - | - |
| `GetVpcLinks` | - | `position -> position`, `limit -> limit` | - | - |
| `ImportApiKeys` | - | `format -> format`, `failOnWarnings -> failonwarnings` | - | `body` |
| `ImportDocumentationParts` | - | `mode -> mode`, `failOnWarnings -> failonwarnings` | - | `body` |
| `ImportRestApi` | - | `failOnWarnings -> failonwarnings` | - | `body` |
| `PutRestApi` | - | `mode -> mode`, `failOnWarnings -> failonwarnings` | - | `body` |
| `RejectDomainNameAccessAssociation` | - | `domainNameAccessAssociationArn -> domainNameAccessAssociationArn`, `domainNameArn -> domainNameArn` | - | - |
| `UntagResource` | - | `tagKeys -> tagKeys` | - | - |
| `UpdateBasePathMapping` | - | `domainNameId -> domainNameId` | - | - |
| `UpdateDomainName` | - | `domainNameId -> domainNameId` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `BadRequestException` | `structure` | message | The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details. |
| `ConflictException` | `structure` | message | The request configuration has conflicts. For details, see the accompanying error message. |
| `LimitExceededException` | `structure` | retryAfterSeconds, message | The request exceeded the rate limit. Retry after the specified time period. |
| `NotFoundException` | `structure` | message | The requested resource is not found. Make sure that the request URI is correct. |
| `ServiceUnavailableException` | `structure` | retryAfterSeconds, message | The requested service is not available. For details see the accompanying error message. Retry after the specified time period. |
| `TooManyRequestsException` | `structure` | retryAfterSeconds, message | The request has reached its throttling limit. Retry after the specified time period. |
| `UnauthorizedException` | `structure` | message | The request is denied because the caller has insufficient permissions. |
| `CreateApiKeyRequest` | `structure` | name, description, enabled, generateDistinctId, value, stageKeys, customerId, tags | Request to create an ApiKey resource. |
| `ApiKey` | `structure` | id, value, name, customerId, description, enabled, createdDate, lastUpdatedDate, stageKeys, tags | A resource that can be distributed to callers for executing Method resources that require an API key. API keys can be mapped to any Stage on any RestApi, wh ... |
| `CreateAuthorizerRequest` | `structure` | restApiId, name, type, providerARNs, authType, authorizerUri, authorizerCredentials, identitySource, identityValidationExpression, authorizerResultTtlInSeconds | Request to add a new Authorizer to an existing RestApi resource. |
| `Authorizer` | `structure` | id, name, type, providerARNs, authType, authorizerUri, authorizerCredentials, identitySource, identityValidationExpression, authorizerResultTtlInSeconds | Represents an authorization layer for methods. If enabled on a method, API Gateway will activate the authorizer when a client calls the method. |
| `CreateBasePathMappingRequest` | `structure` | domainName, domainNameId, basePath, restApiId, stage | Requests API Gateway to create a new BasePathMapping resource. |
| `BasePathMapping` | `structure` | basePath, restApiId, stage | Represents the base path that callers of the API must provide as part of the URL after the domain name. |
| `CreateDeploymentRequest` | `structure` | restApiId, stageName, stageDescription, description, cacheClusterEnabled, cacheClusterSize, variables, canarySettings, tracingEnabled | Requests API Gateway to create a Deployment resource. |
| `Deployment` | `structure` | id, description, createdDate, apiSummary | An immutable representation of a RestApi resource that can be called by users using Stages. A deployment must be associated with a Stage for it to be callab ... |
| `CreateDocumentationPartRequest` | `structure` | restApiId, location, properties | Creates a new documentation part of a given API. |
| `DocumentationPart` | `structure` | id, location, properties | A documentation part for a targeted API entity. |
| `CreateDocumentationVersionRequest` | `structure` | restApiId, documentationVersion, stageName, description | Creates a new documentation version of a given API. |
| `DocumentationVersion` | `structure` | version, createdDate, description | A snapshot of the documentation of an API. |
| `CreateDomainNameRequest` | `structure` | domainName, certificateName, certificateBody, certificatePrivateKey, certificateChain, certificateArn, regionalCertificateName, regionalCertificateArn, endpointConfiguration, tags, securityPolicy, endpointAccessMode, ... (+4) | A request to create a new domain name. |
| `DomainName` | `structure` | domainName, domainNameId, domainNameArn, certificateName, certificateArn, certificateUploadDate, regionalDomainName, regionalHostedZoneId, regionalCertificateName, regionalCertificateArn, distributionDomainName, distributionHostedZoneId, ... (+11) | Represents a custom domain name as a user-friendly host name of an API (RestApi). |
| `CreateDomainNameAccessAssociationRequest` | `structure` | domainNameArn, accessAssociationSourceType, accessAssociationSource, tags | - |
| `DomainNameAccessAssociation` | `structure` | domainNameAccessAssociationArn, domainNameArn, accessAssociationSourceType, accessAssociationSource, tags | Represents a domain name access association between an access association source and a private custom domain name. With a domain name access association, an ... |
| `CreateModelRequest` | `structure` | restApiId, name, description, schema, contentType | Request to add a new Model to an existing RestApi resource. |
| `Model` | `structure` | id, name, description, schema, contentType | Represents the data structure of a method's request or response payload. |
| `CreateRequestValidatorRequest` | `structure` | restApiId, name, validateRequestBody, validateRequestParameters | Creates a RequestValidator of a given RestApi. |
| `RequestValidator` | `structure` | id, name, validateRequestBody, validateRequestParameters | A set of validation rules for incoming Method requests. |
| `CreateResourceRequest` | `structure` | restApiId, parentId, pathPart | Requests API Gateway to create a Resource resource. |
| `Resource` | `structure` | id, parentId, pathPart, path, resourceMethods | Represents an API resource. |
| `CreateRestApiRequest` | `structure` | name, description, version, cloneFrom, binaryMediaTypes, minimumCompressionSize, apiKeySource, endpointConfiguration, policy, tags, disableExecuteApiEndpoint, securityPolicy, ... (+1) | The POST Request to add a new RestApi resource to your collection. |
| `RestApi` | `structure` | id, name, description, createdDate, version, warnings, binaryMediaTypes, minimumCompressionSize, apiKeySource, endpointConfiguration, policy, tags, ... (+6) | Represents a REST API. |
| `CreateStageRequest` | `structure` | restApiId, stageName, deploymentId, description, cacheClusterEnabled, cacheClusterSize, variables, documentationVersion, canarySettings, tracingEnabled, tags | Requests API Gateway to create a Stage resource. |
| `Stage` | `structure` | deploymentId, clientCertificateId, stageName, description, cacheClusterEnabled, cacheClusterSize, cacheClusterStatus, methodSettings, variables, documentationVersion, accessLogSettings, canarySettings, ... (+5) | Represents a unique identifier for a version of a deployed RestApi that is callable by users. |
| `CreateUsagePlanRequest` | `structure` | name, description, apiStages, throttle, quota, tags | The POST request to create a usage plan with the name, description, throttle limits and quota limits, as well as the associated API stages, specified in the ... |
| `UsagePlan` | `structure` | id, name, description, apiStages, throttle, quota, productCode, tags | Represents a usage plan used to specify who can assess associated API stages. Optionally, target request rate and quota limits can be set. In some cases cli ... |
| `CreateUsagePlanKeyRequest` | `structure` | usagePlanId, keyId, keyType | The POST request to create a usage plan key for adding an existing API key to a usage plan. |
| `UsagePlanKey` | `structure` | id, type, value, name | Represents a usage plan key to identify a plan customer. |
| `CreateVpcLinkRequest` | `structure` | name, description, targetArns, tags | Creates a VPC link, under the caller's account in a selected region, in an asynchronous operation that typically takes 2-4 minutes to complete and become op ... |
| `VpcLink` | `structure` | id, name, description, targetArns, status, statusMessage, tags | An API Gateway VPC link for a RestApi to access resources in an Amazon Virtual Private Cloud (VPC). |
| `DeleteApiKeyRequest` | `structure` | apiKey | A request to delete the ApiKey resource. |
| `AccessAssociationSourceType` | `enum` | VPCE | - |
| `ApiKeySourceType` | `enum` | HEADER, AUTHORIZER | - |
| `ApiKeysFormat` | `enum` | csv | - |
| `ApiStatus` | `enum` | UPDATING, AVAILABLE, PENDING, FAILED | - |
| `AuthorizerType` | `enum` | TOKEN, REQUEST, COGNITO_USER_POOLS | The authorizer type. Valid values are TOKEN for a Lambda function using a single authorization token submitted in a custom header, REQUEST for a Lambda func ... |
| `CacheClusterSize` | `enum` | SIZE_0_POINT_5_GB, SIZE_1_POINT_6_GB, SIZE_6_POINT_1_GB, SIZE_13_POINT_5_GB, SIZE_28_POINT_4_GB, SIZE_58_POINT_2_GB, SIZE_118_GB, SIZE_237_GB | Returns the size of the CacheCluster. |
| `CacheClusterStatus` | `enum` | CREATE_IN_PROGRESS, AVAILABLE, DELETE_IN_PROGRESS, NOT_AVAILABLE, FLUSH_IN_PROGRESS | Returns the status of the CacheCluster. |
| `ConnectionType` | `enum` | INTERNET, VPC_LINK | - |
| `ContentHandlingStrategy` | `enum` | CONVERT_TO_BINARY, CONVERT_TO_TEXT | - |
| `DocumentationPartType` | `enum` | API, AUTHORIZER, MODEL, RESOURCE, METHOD, PATH_PARAMETER, QUERY_PARAMETER, REQUEST_HEADER, REQUEST_BODY, RESPONSE, RESPONSE_HEADER, RESPONSE_BODY | - |
## Winterbaume LTM Notes

Sources: .agents/docs/LTM/aws-inter-service-integration-priorities.md, .agents/docs/LTM/cross-service-integration-and-engine-boundaries-synthesis.md.

Mode: reference summaries.

- `.agents/docs/LTM/aws-inter-service-integration-priorities.md`: summarises API Gateway's front-door integration paths. Open it when modelling Lambda proxy/custom integrations, Step Functions API invocation, or AppSync HTTP data-source scenarios.
- `.agents/docs/LTM/cross-service-integration-and-engine-boundaries-synthesis.md`: summarises execution-boundary rules. Open it before treating API Gateway as generic HTTP passthrough.
- `.agents/docs/LTM/smithy-codegen-and-wire-serialization.md`: records API Gateway's request-deserialiser exception. Update operations should call generated `wire::deserialize_*_request` first for labels and `patchOperations`, then inspect the raw body only for flat scalar fields that the Smithy `Update*Request` shape does not model.
- Service implication: API Gateway to Lambda is the fundamental front-door path for cross-service behaviour and should preserve the selected integration type's payload and error contract.
- Service implication: API Gateway can invoke Step Functions APIs such as `StartExecution`, and Step Functions can invoke API Gateway directly. These are concrete bidirectional orchestration boundaries.
- Service implication: AppSync HTTP data sources can use API Gateway backed endpoints, so AppSync integration tests may need API Gateway as an intermediate service.
- Service implication: API Gateway's residual body reads are an intentional PATCH-style hybrid, not evidence that restJson1 request deserialisers are absent. Keep the hybrid scoped to fields outside the Smithy model.

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
