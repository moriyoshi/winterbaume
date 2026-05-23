# AWS Telco Network Builder

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Web Services Telco Network Builder (TNB) is a network automation service that helps you deploy and manage telecom networks. AWS TNB helps you with the lifecycle management of your telecommunication network functions throughout planning, deployment, and post-deployment activities.

## Possible Usage Scenarios
- Scenario insight from EC2: add full state-machine walks for AWS Telco Network Builder resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented AWS Telco Network Builder workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Get`, `List`, `Create`, `Delete`, `Update` operation families, including `GetSolFunctionInstance`, `GetSolFunctionPackage`, `GetSolFunctionPackageContent`, `GetSolFunctionPackageDescriptor`, `ListSolFunctionInstances`, `ListSolFunctionPackages`.

## Service Identity and Protocol

- AWS model slug: `tnb`
- AWS SDK for Rust slug: `tnb`
- Model version: `2008-10-21`
- Model file: `vendor/api-models-aws/models/tnb/service/2008-10-21/tnb-2008-10-21.json`
- SDK ID: `tnb`
- Endpoint prefix: `-`
- ARN namespace: `tnb`
- CloudFormation name: `-`
- CloudTrail event source: `-`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (9), `List` (6), `Create` (3), `Delete` (3), `Update` (3), `Put` (2), `Validate` (2), `Cancel` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CancelSolNetworkOperation`, `CreateSolFunctionPackage`, `CreateSolNetworkInstance`, `CreateSolNetworkPackage`, `DeleteSolFunctionPackage`, `DeleteSolNetworkInstance`, `DeleteSolNetworkPackage`, `PutSolFunctionPackageContent`, `PutSolNetworkPackageContent`, `TagResource`, `TerminateSolNetworkInstance`, `UntagResource`, `UpdateSolFunctionPackage`, `UpdateSolNetworkInstance`, `UpdateSolNetworkPackage`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetSolFunctionInstance`, `GetSolFunctionPackage`, `GetSolFunctionPackageContent`, `GetSolFunctionPackageDescriptor`, `GetSolNetworkInstance`, `GetSolNetworkOperation`, `GetSolNetworkPackage`, `GetSolNetworkPackageContent`, `GetSolNetworkPackageDescriptor`, `ListSolFunctionInstances`, `ListSolFunctionPackages`, `ListSolNetworkInstances`, `ListSolNetworkOperations`, `ListSolNetworkPackages`, `ListTagsForResource`, `ValidateSolFunctionPackageContent`, `ValidateSolNetworkPackageContent`.
- Pagination is modelled for 5 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 9 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CancelSolNetworkOperation`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 33 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `EC2/VPC`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Operation Groups

### Get

- Operations: `GetSolFunctionInstance`, `GetSolFunctionPackage`, `GetSolFunctionPackageContent`, `GetSolFunctionPackageDescriptor`, `GetSolNetworkInstance`, `GetSolNetworkOperation`, `GetSolNetworkPackage`, `GetSolNetworkPackageContent`, `GetSolNetworkPackageDescriptor`
- Traits: `readonly` (9)
- Common required input members in this group: `accept`, `nsInstanceId`, `nsLcmOpOccId`, `nsdInfoId`, `vnfInstanceId`, `vnfPkgId`

### List

- Operations: `ListSolFunctionInstances`, `ListSolFunctionPackages`, `ListSolNetworkInstances`, `ListSolNetworkOperations`, `ListSolNetworkPackages`, `ListTagsForResource`
- Traits: `paginated` (5), `readonly` (6)
- Common required input members in this group: `resourceArn`

### Create

- Operations: `CreateSolFunctionPackage`, `CreateSolNetworkInstance`, `CreateSolNetworkPackage`
- Common required input members in this group: `nsName`, `nsdInfoId`

### Delete

- Operations: `DeleteSolFunctionPackage`, `DeleteSolNetworkInstance`, `DeleteSolNetworkPackage`
- Traits: `idempotent` (3)
- Common required input members in this group: `nsInstanceId`, `nsdInfoId`, `vnfPkgId`

### Update

- Operations: `UpdateSolFunctionPackage`, `UpdateSolNetworkInstance`, `UpdateSolNetworkPackage`
- Common required input members in this group: `nsInstanceId`, `nsdInfoId`, `nsdOperationalState`, `operationalState`, `updateType`, `vnfPkgId`

### Put

- Operations: `PutSolFunctionPackageContent`, `PutSolNetworkPackageContent`
- Traits: `idempotent` (2)
- Common required input members in this group: `file`, `nsdInfoId`, `vnfPkgId`

### Validate

- Operations: `ValidateSolFunctionPackageContent`, `ValidateSolNetworkPackageContent`
- Traits: `idempotent` (2)
- Common required input members in this group: `file`, `nsdInfoId`, `vnfPkgId`

### Cancel

- Operations: `CancelSolNetworkOperation`
- Common required input members in this group: `nsLcmOpOccId`

### Instantiate

- Operations: `InstantiateSolNetworkInstance`
- Common required input members in this group: `nsInstanceId`

### Tag

- Operations: `TagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `resourceArn`, `tags`

### Terminate

- Operations: `TerminateSolNetworkInstance`
- Common required input members in this group: `nsInstanceId`

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `resourceArn`, `tagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CancelSolNetworkOperation` | `POST /sol/nslcm/v1/ns_lcm_op_occs/{nsLcmOpOccId}/cancel` | - | `nsLcmOpOccId` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Cancels a network operation. A network operation is any operation that is done to your network, such as network instance instantiation or termination. |
| `CreateSolFunctionPackage` | `POST /sol/vnfpkgm/v1/vnf_packages` | - | - | - | `CreateSolFunctionPackageOutput` | `AccessDeniedException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a function package. A function package is a .zip file in CSAR (Cloud Service Archive) format that contains a network function (an ETSI standard telecommunication application) and function package descriptor that uses the TOSCA standard to describe how... |
| `CreateSolNetworkInstance` | `POST /sol/nslcm/v1/ns_instances` | - | `nsName`, `nsdInfoId` | - | `CreateSolNetworkInstanceOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a network instance. A network instance is a single network created in Amazon Web Services TNB that can be deployed and on which life-cycle operations (like terminate, update, and delete) can be performed. |
| `CreateSolNetworkPackage` | `POST /sol/nsd/v1/ns_descriptors` | - | - | - | `CreateSolNetworkPackageOutput` | `AccessDeniedException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a network package. A network package is a .zip file in CSAR (Cloud Service Archive) format defines the function packages you want to deploy and the Amazon Web Services infrastructure you want to deploy them on. |
| `DeleteSolFunctionPackage` | `DELETE /sol/vnfpkgm/v1/vnf_packages/{vnfPkgId}` | `idempotent` | `vnfPkgId` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a function package. A function package is a .zip file in CSAR (Cloud Service Archive) format that contains a network function (an ETSI standard telecommunication application) and function package descriptor that uses the TOSCA standard to describe how... |
| `DeleteSolNetworkInstance` | `DELETE /sol/nslcm/v1/ns_instances/{nsInstanceId}` | `idempotent` | `nsInstanceId` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a network instance. A network instance is a single network created in Amazon Web Services TNB that can be deployed and on which life-cycle operations (like terminate, update, and delete) can be performed. |
| `DeleteSolNetworkPackage` | `DELETE /sol/nsd/v1/ns_descriptors/{nsdInfoId}` | `idempotent` | `nsdInfoId` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes network package. A network package is a .zip file in CSAR (Cloud Service Archive) format defines the function packages you want to deploy and the Amazon Web Services infrastructure you want to deploy them on. |
| `GetSolFunctionInstance` | `GET /sol/vnflcm/v1/vnf_instances/{vnfInstanceId}` | `readonly` | `vnfInstanceId` | - | `GetSolFunctionInstanceOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets the details of a network function instance, including the instantiation state and metadata from the function package descriptor in the network function package. A network function instance is a function in a function package . |
| `GetSolFunctionPackage` | `GET /sol/vnfpkgm/v1/vnf_packages/{vnfPkgId}` | `readonly` | `vnfPkgId` | - | `GetSolFunctionPackageOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets the details of an individual function package, such as the operational state and whether the package is in use. A function package is a .zip file in CSAR (Cloud Service Archive) format that contains a network function (an ETSI standard telecommunication... |
| `GetSolFunctionPackageContent` | `GET /sol/vnfpkgm/v1/vnf_packages/{vnfPkgId}/package_content` | `readonly` | `accept`, `vnfPkgId` | - | `GetSolFunctionPackageContentOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets the contents of a function package. A function package is a .zip file in CSAR (Cloud Service Archive) format that contains a network function (an ETSI standard telecommunication application) and function package descriptor that uses the TOSCA standard to... |
| `GetSolFunctionPackageDescriptor` | `GET /sol/vnfpkgm/v1/vnf_packages/{vnfPkgId}/vnfd` | `readonly` | `accept`, `vnfPkgId` | - | `GetSolFunctionPackageDescriptorOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets a function package descriptor in a function package. A function package descriptor is a .yaml file in a function package that uses the TOSCA standard to describe how the network function in the function package should run on your network. |
| `GetSolNetworkInstance` | `GET /sol/nslcm/v1/ns_instances/{nsInstanceId}` | `readonly` | `nsInstanceId` | - | `GetSolNetworkInstanceOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets the details of the network instance. A network instance is a single network created in Amazon Web Services TNB that can be deployed and on which life-cycle operations (like terminate, update, and delete) can be performed. |
| `GetSolNetworkOperation` | `GET /sol/nslcm/v1/ns_lcm_op_occs/{nsLcmOpOccId}` | `readonly` | `nsLcmOpOccId` | - | `GetSolNetworkOperationOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets the details of a network operation, including the tasks involved in the network operation and the status of the tasks. A network operation is any operation that is done to your network, such as network instance instantiation or termination. |
| `GetSolNetworkPackage` | `GET /sol/nsd/v1/ns_descriptors/{nsdInfoId}` | `readonly` | `nsdInfoId` | - | `GetSolNetworkPackageOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets the details of a network package. A network package is a .zip file in CSAR (Cloud Service Archive) format defines the function packages you want to deploy and the Amazon Web Services infrastructure you want to deploy them on. |
| `GetSolNetworkPackageContent` | `GET /sol/nsd/v1/ns_descriptors/{nsdInfoId}/nsd_content` | `readonly` | `accept`, `nsdInfoId` | - | `GetSolNetworkPackageContentOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets the contents of a network package. A network package is a .zip file in CSAR (Cloud Service Archive) format defines the function packages you want to deploy and the Amazon Web Services infrastructure you want to deploy them on. |
| `GetSolNetworkPackageDescriptor` | `GET /sol/nsd/v1/ns_descriptors/{nsdInfoId}/nsd` | `readonly` | `nsdInfoId` | - | `GetSolNetworkPackageDescriptorOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets the content of the network service descriptor. A network service descriptor is a .yaml file in a network package that uses the TOSCA standard to describe the network functions you want to deploy and the Amazon Web Services infrastructure you want to... |
| `InstantiateSolNetworkInstance` | `POST /sol/nslcm/v1/ns_instances/{nsInstanceId}/instantiate` | - | `nsInstanceId` | - | `InstantiateSolNetworkInstanceOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Instantiates a network instance. A network instance is a single network created in Amazon Web Services TNB that can be deployed and on which life-cycle operations (like terminate, update, and delete) can be performed. |
| `ListSolFunctionInstances` | `GET /sol/vnflcm/v1/vnf_instances` | `readonly`, `paginated` | - | - | `ListSolFunctionInstancesOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists network function instances. A network function instance is a function in a function package . |
| `ListSolFunctionPackages` | `GET /sol/vnfpkgm/v1/vnf_packages` | `readonly`, `paginated` | - | - | `ListSolFunctionPackagesOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists information about function packages. A function package is a .zip file in CSAR (Cloud Service Archive) format that contains a network function (an ETSI standard telecommunication application) and function package descriptor that uses the TOSCA standard... |
| `ListSolNetworkInstances` | `GET /sol/nslcm/v1/ns_instances` | `readonly`, `paginated` | - | - | `ListSolNetworkInstancesOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists your network instances. A network instance is a single network created in Amazon Web Services TNB that can be deployed and on which life-cycle operations (like terminate, update, and delete) can be performed. |
| `ListSolNetworkOperations` | `GET /sol/nslcm/v1/ns_lcm_op_occs` | `readonly`, `paginated` | - | - | `ListSolNetworkOperationsOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists details for a network operation, including when the operation started and the status of the operation. A network operation is any operation that is done to your network, such as network instance instantiation or termination. |
| `ListSolNetworkPackages` | `GET /sol/nsd/v1/ns_descriptors` | `readonly`, `paginated` | - | - | `ListSolNetworkPackagesOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists network packages. A network package is a .zip file in CSAR (Cloud Service Archive) format defines the function packages you want to deploy and the Amazon Web Services infrastructure you want to deploy them on. |
| `ListTagsForResource` | `GET /tags/{resourceArn}` | `readonly` | `resourceArn` | - | `ListTagsForResourceOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists tags for AWS TNB resources. |
| `PutSolFunctionPackageContent` | `PUT /sol/vnfpkgm/v1/vnf_packages/{vnfPkgId}/package_content` | `idempotent` | `file`, `vnfPkgId` | - | `PutSolFunctionPackageContentOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Uploads the contents of a function package. A function package is a .zip file in CSAR (Cloud Service Archive) format that contains a network function (an ETSI standard telecommunication application) and function package descriptor that uses the TOSCA standard... |
| `PutSolNetworkPackageContent` | `PUT /sol/nsd/v1/ns_descriptors/{nsdInfoId}/nsd_content` | `idempotent` | `file`, `nsdInfoId` | - | `PutSolNetworkPackageContentOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Uploads the contents of a network package. A network package is a .zip file in CSAR (Cloud Service Archive) format defines the function packages you want to deploy and the Amazon Web Services infrastructure you want to deploy them on. |
| `TagResource` | `POST /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tags` | - | `TagResourceOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Tags an AWS TNB resource. A tag is a label that you assign to an Amazon Web Services resource. |
| `TerminateSolNetworkInstance` | `POST /sol/nslcm/v1/ns_instances/{nsInstanceId}/terminate` | - | `nsInstanceId` | - | `TerminateSolNetworkInstanceOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Terminates a network instance. A network instance is a single network created in Amazon Web Services TNB that can be deployed and on which life-cycle operations (like terminate, update, and delete) can be performed. |
| `UntagResource` | `DELETE /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tagKeys` | - | `UntagResourceOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Untags an AWS TNB resource. A tag is a label that you assign to an Amazon Web Services resource. |
| `UpdateSolFunctionPackage` | `PATCH /sol/vnfpkgm/v1/vnf_packages/{vnfPkgId}` | - | `operationalState`, `vnfPkgId` | - | `UpdateSolFunctionPackageOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates the operational state of function package. A function package is a .zip file in CSAR (Cloud Service Archive) format that contains a network function (an ETSI standard telecommunication application) and function package descriptor that uses the TOSCA... |
| `UpdateSolNetworkInstance` | `POST /sol/nslcm/v1/ns_instances/{nsInstanceId}/update` | - | `nsInstanceId`, `updateType` | - | `UpdateSolNetworkInstanceOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Update a network instance. A network instance is a single network created in Amazon Web Services TNB that can be deployed and on which life-cycle operations (like terminate, update, and delete) can be performed. |
| `UpdateSolNetworkPackage` | `PATCH /sol/nsd/v1/ns_descriptors/{nsdInfoId}` | - | `nsdInfoId`, `nsdOperationalState` | - | `UpdateSolNetworkPackageOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates the operational state of a network package. A network package is a .zip file in CSAR (Cloud Service Archive) format defines the function packages you want to deploy and the Amazon Web Services infrastructure you want to deploy them on. |
| `ValidateSolFunctionPackageContent` | `PUT /sol/vnfpkgm/v1/vnf_packages/{vnfPkgId}/package_content/validate` | `idempotent` | `file`, `vnfPkgId` | - | `ValidateSolFunctionPackageContentOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Validates function package content. This can be used as a dry run before uploading function package content with PutSolFunctionPackageContent. |
| `ValidateSolNetworkPackageContent` | `PUT /sol/nsd/v1/ns_descriptors/{nsdInfoId}/nsd_content/validate` | `idempotent` | `file`, `nsdInfoId` | - | `ValidateSolNetworkPackageContentOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Validates network package content. This can be used as a dry run before uploading network package content with PutSolNetworkPackageContent. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `GetSolFunctionPackageContent` | `accept -> Accept` | - | - | - |
| `GetSolFunctionPackageDescriptor` | `accept -> Accept` | - | - | - |
| `GetSolNetworkPackageContent` | `accept -> Accept` | - | - | - |
| `InstantiateSolNetworkInstance` | - | `dryRun -> dry_run` | - | - |
| `ListSolFunctionInstances` | - | `maxResults -> max_results`, `nextToken -> nextpage_opaque_marker` | - | - |
| `ListSolFunctionPackages` | - | `maxResults -> max_results`, `nextToken -> nextpage_opaque_marker` | - | - |
| `ListSolNetworkInstances` | - | `maxResults -> max_results`, `nextToken -> nextpage_opaque_marker` | - | - |
| `ListSolNetworkOperations` | - | `nsInstanceId -> nsInstanceId`, `maxResults -> max_results`, `nextToken -> nextpage_opaque_marker` | - | - |
| `ListSolNetworkPackages` | - | `maxResults -> max_results`, `nextToken -> nextpage_opaque_marker` | - | - |
| `PutSolFunctionPackageContent` | `contentType -> Content-Type` | - | - | `file` |
| `PutSolNetworkPackageContent` | `contentType -> Content-Type` | - | - | `file` |
| `UntagResource` | - | `tagKeys -> tagKeys` | - | - |
| `ValidateSolFunctionPackageContent` | `contentType -> Content-Type` | - | - | `file` |
| `ValidateSolNetworkPackageContent` | `contentType -> Content-Type` | - | - | `file` |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | `message` | Insufficient permissions to make request. |
| `InternalServerException` | `structure` | `message` | Unexpected error occurred. |
| `ThrottlingException` | `structure` | `message` | Exception caused by throttling. |
| `ValidationException` | `structure` | `message` | Unable to process the request because the client provided input failed to satisfy request constraints. |
| `ResourceNotFoundException` | `structure` | `message` | Request references a resource that doesn't exist. |
| `ServiceQuotaExceededException` | `structure` | `message` | Service quotas have been exceeded. |
| `CancelSolNetworkOperationInput` | `structure` | `nsLcmOpOccId` | - |
| `CreateSolFunctionPackageInput` | `structure` | `tags` | - |
| `CreateSolFunctionPackageOutput` | `structure` | `arn`, `id`, `onboardingState`, `operationalState`, `tags`, `usageState` | - |
| `CreateSolNetworkInstanceInput` | `structure` | `nsDescription`, `nsName`, `nsdInfoId`, `tags` | - |
| `CreateSolNetworkInstanceOutput` | `structure` | `arn`, `id`, `nsInstanceName`, `nsdInfoId`, `tags` | - |
| `CreateSolNetworkPackageInput` | `structure` | `tags` | - |
| `CreateSolNetworkPackageOutput` | `structure` | `arn`, `id`, `nsdOnboardingState`, `nsdOperationalState`, `nsdUsageState`, `tags` | - |
| `DeleteSolFunctionPackageInput` | `structure` | `vnfPkgId` | - |
| `DeleteSolNetworkInstanceInput` | `structure` | `nsInstanceId` | - |
| `DeleteSolNetworkPackageInput` | `structure` | `nsdInfoId` | - |
| `GetSolFunctionInstanceInput` | `structure` | `vnfInstanceId` | - |
| `GetSolFunctionInstanceOutput` | `structure` | `arn`, `id`, `instantiatedVnfInfo`, `instantiationState`, `metadata`, `nsInstanceId`, `tags`, `vnfPkgId`, `vnfProductName`, `vnfProvider`, `vnfdId`, `vnfdVersion` | - |
| `GetSolFunctionPackageInput` | `structure` | `vnfPkgId` | - |
| `GetSolFunctionPackageOutput` | `structure` | `arn`, `id`, `metadata`, `onboardingState`, `operationalState`, `tags`, `usageState`, `vnfProductName`, `vnfProvider`, `vnfdId`, `vnfdVersion` | - |
| `GetSolFunctionPackageContentInput` | `structure` | `accept`, `vnfPkgId` | - |
| `GetSolFunctionPackageContentOutput` | `structure` | `contentType`, `packageContent` | - |
| `GetSolFunctionPackageDescriptorInput` | `structure` | `accept`, `vnfPkgId` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
