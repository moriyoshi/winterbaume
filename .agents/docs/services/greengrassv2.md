# AWS IoT Greengrass V2

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

IoT Greengrass brings local compute, messaging, data management, sync, and ML inference capabilities to edge devices. This enables devices to collect and analyze data closer to the source of information, react autonomously to local events, and communicate securely with each other on local networks. Local devices can also communicate securely with Amazon Web Services IoT Core and export IoT data to the Amazon Web Services Cloud. IoT Greengrass developers can use Lambda functions and components to create and deploy applications to fleets of edge devices for local operation. IoT Greengrass Version 2 provides a new major version of the IoT Greengrass Core software, new APIs, and a new console.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for AWS IoT Greengrass V2 where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: cover association replacement for AWS IoT Greengrass V2 by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- Scenario insight from EC2: add full state-machine walks for AWS IoT Greengrass V2 resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented AWS IoT Greengrass V2 workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Get`, `Delete`, `Batch`, `Create` operation families, including `ListClientDevicesAssociatedWithCoreDevice`, `ListComponentVersions`, `ListComponents`, `ListCoreDevices`, `GetComponent`, `GetComponentVersionArtifact`.

## Service Identity and Protocol

- AWS model slug: `greengrassv2`
- AWS SDK for Rust slug: `greengrass`
- Model version: `2020-11-30`
- Model file: `vendor/api-models-aws/models/greengrassv2/service/2020-11-30/greengrassv2-2020-11-30.json`
- SDK ID: `GreengrassV2`
- Endpoint prefix: `greengrass`
- ARN namespace: `greengrass`
- CloudFormation name: `GreengrassV2`
- CloudTrail event source: `greengrass.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (8), `Get` (6), `Delete` (3), `Batch` (2), `Create` (2), `Associate` (1), `Cancel` (1), `Describe` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AssociateServiceRoleToAccount`, `BatchAssociateClientDeviceWithCoreDevice`, `BatchDisassociateClientDeviceFromCoreDevice`, `CancelDeployment`, `CreateComponentVersion`, `CreateDeployment`, `DeleteComponent`, `DeleteCoreDevice`, `DeleteDeployment`, `DisassociateServiceRoleFromAccount`, `TagResource`, `UntagResource`, `UpdateConnectivityInfo`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeComponent`, `GetComponent`, `GetComponentVersionArtifact`, `GetConnectivityInfo`, `GetCoreDevice`, `GetDeployment`, `GetServiceRoleForAccount`, `ListClientDevicesAssociatedWithCoreDevice`, `ListComponentVersions`, `ListComponents`, `ListCoreDevices`, `ListDeployments`, `ListEffectiveDeployments`, `ListInstalledComponents`, `ListTagsForResource`.
- Pagination is modelled for 7 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 2 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CancelDeployment`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 29 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `Lambda`.

## Operation Groups

### List

- Operations: `ListClientDevicesAssociatedWithCoreDevice`, `ListComponentVersions`, `ListComponents`, `ListCoreDevices`, `ListDeployments`, `ListEffectiveDeployments`, `ListInstalledComponents`, `ListTagsForResource`
- Traits: `paginated` (7)
- Common required input members in this group: `arn`, `coreDeviceThingName`, `resourceArn`

### Get

- Operations: `GetComponent`, `GetComponentVersionArtifact`, `GetConnectivityInfo`, `GetCoreDevice`, `GetDeployment`, `GetServiceRoleForAccount`
- Common required input members in this group: `arn`, `artifactName`, `coreDeviceThingName`, `deploymentId`, `thingName`

### Delete

- Operations: `DeleteComponent`, `DeleteCoreDevice`, `DeleteDeployment`
- Common required input members in this group: `arn`, `coreDeviceThingName`, `deploymentId`

### Batch

- Operations: `BatchAssociateClientDeviceWithCoreDevice`, `BatchDisassociateClientDeviceFromCoreDevice`
- Common required input members in this group: `coreDeviceThingName`

### Create

- Operations: `CreateComponentVersion`, `CreateDeployment`
- Traits: `idempotency-token` (2)
- Common required input members in this group: `targetArn`

### Associate

- Operations: `AssociateServiceRoleToAccount`
- Common required input members in this group: `roleArn`

### Cancel

- Operations: `CancelDeployment`
- Common required input members in this group: `deploymentId`

### Describe

- Operations: `DescribeComponent`
- Common required input members in this group: `arn`

### Disassociate

- Operations: `DisassociateServiceRoleFromAccount`

### Resolve

- Operations: `ResolveComponentCandidates`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `resourceArn`, `tags`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `resourceArn`, `tagKeys`

### Update

- Operations: `UpdateConnectivityInfo`
- Common required input members in this group: `connectivityInfo`, `thingName`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AssociateServiceRoleToAccount` | `PUT /greengrass/servicerole` | - | `roleArn` | - | `AssociateServiceRoleToAccountResponse` | `InternalServerException`, `ValidationException` | Associates a Greengrass service role with IoT Greengrass for your Amazon Web Services account in this Amazon Web Services Region. IoT Greengrass uses this role to verify the identity of client devices and manage core device connectivity information. |
| `BatchAssociateClientDeviceWithCoreDevice` | `POST /greengrass/v2/coreDevices/{coreDeviceThingName}/associateClientDevices` | - | `coreDeviceThingName` | - | `BatchAssociateClientDeviceWithCoreDeviceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Associates a list of client devices with a core device. Use this API operation to specify which client devices can discover a core device through cloud discovery. |
| `BatchDisassociateClientDeviceFromCoreDevice` | `POST /greengrass/v2/coreDevices/{coreDeviceThingName}/disassociateClientDevices` | - | `coreDeviceThingName` | - | `BatchDisassociateClientDeviceFromCoreDeviceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Disassociates a list of client devices from a core device. After you disassociate a client device from a core device, the client device won't be able to use cloud discovery to retrieve the core device's connectivity information and certificates. |
| `CancelDeployment` | `POST /greengrass/v2/deployments/{deploymentId}/cancel` | - | `deploymentId` | - | `CancelDeploymentResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Cancels a deployment. This operation cancels the deployment for devices that haven't yet received it. |
| `CreateComponentVersion` | `POST /greengrass/v2/createComponentVersion` | `idempotency-token` | - | `clientToken` | `CreateComponentVersionResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `RequestAlreadyInProgressException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a component. Components are software that run on Greengrass core devices. |
| `CreateDeployment` | `POST /greengrass/v2/deployments` | `idempotency-token` | `targetArn` | `clientToken` | `CreateDeploymentResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `RequestAlreadyInProgressException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Creates a continuous deployment for a target, which is a Greengrass core device or group of core devices. When you add a new core device to a group of core devices that has a deployment, IoT Greengrass deploys that group's deployment to the new device. |
| `DeleteComponent` | `DELETE /greengrass/v2/components/{arn}` | - | `arn` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a version of a component from IoT Greengrass. This operation deletes the component's recipe and artifacts. |
| `DeleteCoreDevice` | `DELETE /greengrass/v2/coreDevices/{coreDeviceThingName}` | - | `coreDeviceThingName` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a Greengrass core device, which is an IoT thing. This operation removes the core device from the list of core devices. |
| `DeleteDeployment` | `DELETE /greengrass/v2/deployments/{deploymentId}` | - | `deploymentId` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a deployment. To delete an active deployment, you must first cancel it. |
| `DescribeComponent` | `GET /greengrass/v2/components/{arn}/metadata` | - | `arn` | - | `DescribeComponentResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves metadata for a version of a component. |
| `DisassociateServiceRoleFromAccount` | `DELETE /greengrass/servicerole` | - | - | - | `DisassociateServiceRoleFromAccountResponse` | `InternalServerException` | Disassociates the Greengrass service role from IoT Greengrass for your Amazon Web Services account in this Amazon Web Services Region. Without a service role, IoT Greengrass can't verify the identity of client devices or manage core device connectivity... |
| `GetComponent` | `GET /greengrass/v2/components/{arn}` | - | `arn` | - | `GetComponentResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets the recipe for a version of a component. |
| `GetComponentVersionArtifact` | `GET /greengrass/v2/components/{arn}/artifacts/{artifactName+}` | - | `arn`, `artifactName` | - | `GetComponentVersionArtifactResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets the pre-signed URL to download a public or a Lambda component artifact. Core devices call this operation to identify the URL that they can use to download an artifact to install. |
| `GetConnectivityInfo` | `GET /greengrass/things/{thingName}/connectivityInfo` | - | `thingName` | - | `GetConnectivityInfoResponse` | `InternalServerException`, `ValidationException` | Retrieves connectivity information for a Greengrass core device. Connectivity information includes endpoints and ports where client devices can connect to an MQTT broker on the core device. |
| `GetCoreDevice` | `GET /greengrass/v2/coreDevices/{coreDeviceThingName}` | - | `coreDeviceThingName` | - | `GetCoreDeviceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves metadata for a Greengrass core device. IoT Greengrass relies on individual devices to send status updates to the Amazon Web Services Cloud. |
| `GetDeployment` | `GET /greengrass/v2/deployments/{deploymentId}` | - | `deploymentId` | - | `GetDeploymentResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets a deployment. Deployments define the components that run on Greengrass core devices. |
| `GetServiceRoleForAccount` | `GET /greengrass/servicerole` | - | - | - | `GetServiceRoleForAccountResponse` | `InternalServerException` | Gets the service role associated with IoT Greengrass for your Amazon Web Services account in this Amazon Web Services Region. IoT Greengrass uses this role to verify the identity of client devices and manage core device connectivity information. |
| `ListClientDevicesAssociatedWithCoreDevice` | `GET /greengrass/v2/coreDevices/{coreDeviceThingName}/associatedClientDevices` | `paginated` | `coreDeviceThingName` | - | `ListClientDevicesAssociatedWithCoreDeviceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves a paginated list of client devices that are associated with a core device. |
| `ListComponentVersions` | `GET /greengrass/v2/components/{arn}/versions` | `paginated` | `arn` | - | `ListComponentVersionsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves a paginated list of all versions for a component. Greater versions are listed first. |
| `ListComponents` | `GET /greengrass/v2/components` | `paginated` | - | - | `ListComponentsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves a paginated list of component summaries. This list includes components that you have permission to view. |
| `ListCoreDevices` | `GET /greengrass/v2/coreDevices` | `paginated` | - | - | `ListCoreDevicesResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Retrieves a paginated list of Greengrass core devices. IoT Greengrass relies on individual devices to send status updates to the Amazon Web Services Cloud. |
| `ListDeployments` | `GET /greengrass/v2/deployments` | `paginated` | - | - | `ListDeploymentsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Retrieves a paginated list of deployments. |
| `ListEffectiveDeployments` | `GET /greengrass/v2/coreDevices/{coreDeviceThingName}/effectiveDeployments` | `paginated` | `coreDeviceThingName` | - | `ListEffectiveDeploymentsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves a paginated list of deployment jobs that IoT Greengrass sends to Greengrass core devices. |
| `ListInstalledComponents` | `GET /greengrass/v2/coreDevices/{coreDeviceThingName}/installedComponents` | `paginated` | `coreDeviceThingName` | - | `ListInstalledComponentsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves a paginated list of the components that a Greengrass core device runs. By default, this list doesn't include components that are deployed as dependencies of other components. |
| `ListTagsForResource` | `GET /tags/{resourceArn}` | - | `resourceArn` | - | `ListTagsForResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Retrieves the list of tags for an IoT Greengrass resource. |
| `ResolveComponentCandidates` | `POST /greengrass/v2/resolveComponentCandidates` | - | - | - | `ResolveComponentCandidatesResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves a list of components that meet the component, version, and platform requirements of a deployment. Greengrass core devices call this operation when they receive a deployment to identify the components to install. |
| `TagResource` | `POST /tags/{resourceArn}` | - | `resourceArn`, `tags` | - | `TagResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Adds tags to an IoT Greengrass resource. If a tag already exists for the resource, this operation updates the tag's value. |
| `UntagResource` | `DELETE /tags/{resourceArn}` | - | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Removes a tag from an IoT Greengrass resource. |
| `UpdateConnectivityInfo` | `PUT /greengrass/things/{thingName}/connectivityInfo` | - | `connectivityInfo`, `thingName` | - | `UpdateConnectivityInfoResponse` | `InternalServerException`, `ValidationException` | Updates connectivity information for a Greengrass core device. Connectivity information includes endpoints and ports where client devices can connect to an MQTT broker on the core device. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InternalServerException` | `structure` | `message`, `retryAfterSeconds` | IoT Greengrass can't process your request right now. |
| `ValidationException` | `structure` | `fields`, `message`, `reason` | The request isn't valid. |
| `AccessDeniedException` | `structure` | `message` | You don't have permission to perform the action. |
| `ResourceNotFoundException` | `structure` | `message`, `resourceId`, `resourceType` | The requested resource can't be found. |
| `ThrottlingException` | `structure` | `message`, `quotaCode`, `retryAfterSeconds`, `serviceCode` | Your request exceeded a request rate quota. |
| `ConflictException` | `structure` | `message`, `resourceId`, `resourceType` | Your request has conflicting operations. |
| `RequestAlreadyInProgressException` | `structure` | `message` | The request is already in progress. |
| `AssociateServiceRoleToAccountRequest` | `structure` | `roleArn` | - |
| `AssociateServiceRoleToAccountResponse` | `structure` | `associatedAt` | - |
| `BatchAssociateClientDeviceWithCoreDeviceRequest` | `structure` | `coreDeviceThingName`, `entries` | - |
| `BatchAssociateClientDeviceWithCoreDeviceResponse` | `structure` | `errorEntries` | - |
| `BatchDisassociateClientDeviceFromCoreDeviceRequest` | `structure` | `coreDeviceThingName`, `entries` | - |
| `BatchDisassociateClientDeviceFromCoreDeviceResponse` | `structure` | `errorEntries` | - |
| `CancelDeploymentRequest` | `structure` | `deploymentId` | - |
| `CancelDeploymentResponse` | `structure` | `message` | - |
| `CreateComponentVersionRequest` | `structure` | `clientToken`, `inlineRecipe`, `lambdaFunction`, `tags` | - |
| `CreateComponentVersionResponse` | `structure` | `arn`, `componentName`, `componentVersion`, `creationTimestamp`, `status` | - |
| `ServiceQuotaExceededException` | `structure` | `message`, `quotaCode`, `resourceId`, `resourceType`, `serviceCode` | Your request exceeds a service quota. |
| `CreateDeploymentRequest` | `structure` | `clientToken`, `components`, `deploymentName`, `deploymentPolicies`, `iotJobConfiguration`, `parentTargetArn`, `tags`, `targetArn` | - |
| `CreateDeploymentResponse` | `structure` | `deploymentId`, `iotJobArn`, `iotJobId` | - |
| `DeleteComponentRequest` | `structure` | `arn` | - |
| `DeleteCoreDeviceRequest` | `structure` | `coreDeviceThingName` | - |
| `DeleteDeploymentRequest` | `structure` | `deploymentId` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
