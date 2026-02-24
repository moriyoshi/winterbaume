# AWS Service Catalog App Registry

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Web Services Service Catalog AppRegistry enables organizations to understand the application context of their Amazon Web Services resources. AppRegistry provides a repository of your applications, their resources, and the application metadata that you use within your enterprise.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for AWS Service Catalog App Registry where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: cover association replacement for AWS Service Catalog App Registry by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- From the AWS documentation and model: represent documented AWS Service Catalog App Registry workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Get`, `Associate`, `Create`, `Delete` operation families, including `ListApplications`, `ListAssociatedAttributeGroups`, `ListAssociatedResources`, `ListAttributeGroups`, `GetApplication`, `GetAssociatedResource`.

## Service Identity and Protocol

- AWS model slug: `service-catalog-appregistry`
- AWS SDK for Rust slug: `servicecatalogappregistry`
- Model version: `2020-06-24`
- Model file: `vendor/api-models-aws/models/service-catalog-appregistry/service/2020-06-24/service-catalog-appregistry-2020-06-24.json`
- SDK ID: `Service Catalog AppRegistry`
- Endpoint prefix: `servicecatalog-appregistry`
- ARN namespace: `servicecatalog`
- CloudFormation name: `ServiceCatalogAppRegistry`
- CloudTrail event source: `servicecatalogappregistry.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (6), `Get` (4), `Associate` (2), `Create` (2), `Delete` (2), `Disassociate` (2), `Update` (2), `Put` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AssociateAttributeGroup`, `AssociateResource`, `CreateApplication`, `CreateAttributeGroup`, `DeleteApplication`, `DeleteAttributeGroup`, `DisassociateAttributeGroup`, `DisassociateResource`, `PutConfiguration`, `TagResource`, `UntagResource`, `UpdateApplication`, `UpdateAttributeGroup`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetApplication`, `GetAssociatedResource`, `GetAttributeGroup`, `GetConfiguration`, `ListApplications`, `ListAssociatedAttributeGroups`, `ListAssociatedResources`, `ListAttributeGroups`, `ListAttributeGroupsForApplication`, `ListTagsForResource`.
- Pagination is modelled for 5 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 8 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 24 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `ECR`.

## Operation Groups

### List

- Operations: `ListApplications`, `ListAssociatedAttributeGroups`, `ListAssociatedResources`, `ListAttributeGroups`, `ListAttributeGroupsForApplication`, `ListTagsForResource`
- Traits: `idempotent` (5), `paginated` (5)
- Common required input members in this group: `application`, `resourceArn`

### Get

- Operations: `GetApplication`, `GetAssociatedResource`, `GetAttributeGroup`, `GetConfiguration`
- Traits: `idempotent` (1)
- Common required input members in this group: `application`, `attributeGroup`, `resource`, `resourceType`

### Associate

- Operations: `AssociateAttributeGroup`, `AssociateResource`
- Common required input members in this group: `application`, `attributeGroup`, `resource`, `resourceType`

### Create

- Operations: `CreateApplication`, `CreateAttributeGroup`
- Traits: `idempotency-token` (2)
- Common required input members in this group: `attributes`, `clientToken`, `name`

### Delete

- Operations: `DeleteApplication`, `DeleteAttributeGroup`
- Common required input members in this group: `application`, `attributeGroup`

### Disassociate

- Operations: `DisassociateAttributeGroup`, `DisassociateResource`
- Common required input members in this group: `application`, `attributeGroup`, `resource`, `resourceType`

### Update

- Operations: `UpdateApplication`, `UpdateAttributeGroup`
- Common required input members in this group: `application`, `attributeGroup`

### Put

- Operations: `PutConfiguration`
- Common required input members in this group: `configuration`

### Sync

- Operations: `SyncResource`
- Common required input members in this group: `resource`, `resourceType`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `resourceArn`, `tags`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `resourceArn`, `tagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AssociateAttributeGroup` | `PUT /applications/{application}/attribute-groups/{attributeGroup}` | - | `application`, `attributeGroup` | - | `AssociateAttributeGroupResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Associates an attribute group with an application to augment the application's metadata with the group's attributes. This feature enables applications to be described with user-defined details that are machine-readable, such as third-party integrations. |
| `AssociateResource` | `PUT /applications/{application}/resources/{resourceType}/{resource}` | - | `application`, `resource`, `resourceType` | - | `AssociateResourceResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Associates a resource with an application. The resource can be specified by its ARN or name. |
| `CreateApplication` | `POST /applications` | `idempotency-token` | `clientToken`, `name` | `clientToken` | `CreateApplicationResponse` | `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a new application that is the top-level node in a hierarchy of related cloud resource abstractions. |
| `CreateAttributeGroup` | `POST /attribute-groups` | `idempotency-token` | `attributes`, `clientToken`, `name` | `clientToken` | `CreateAttributeGroupResponse` | `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ValidationException` | Creates a new attribute group as a container for user-defined attributes. This feature enables users to have full control over their cloud application's metadata in a rich machine-readable format to facilitate integration with automated workflows and... |
| `DeleteApplication` | `DELETE /applications/{application}` | - | `application` | - | `DeleteApplicationResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Deletes an application that is specified either by its application ID, name, or ARN. All associated attribute groups and resources must be disassociated from it before deleting an application. |
| `DeleteAttributeGroup` | `DELETE /attribute-groups/{attributeGroup}` | - | `attributeGroup` | - | `DeleteAttributeGroupResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Deletes an attribute group, specified either by its attribute group ID, name, or ARN. |
| `DisassociateAttributeGroup` | `DELETE /applications/{application}/attribute-groups/{attributeGroup}` | - | `application`, `attributeGroup` | - | `DisassociateAttributeGroupResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Disassociates an attribute group from an application to remove the extra attributes contained in the attribute group from the application's metadata. This operation reverts `AssociateAttributeGroup`. |
| `DisassociateResource` | `DELETE /applications/{application}/resources/{resourceType}/{resource}` | - | `application`, `resource`, `resourceType` | - | `DisassociateResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Disassociates a resource from application. Both the resource and the application can be specified either by ID or name. |
| `GetApplication` | `GET /applications/{application}` | - | `application` | - | `GetApplicationResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Retrieves metadata information about one of your applications. The application can be specified by its ARN, ID, or name (which is unique within one account in one region at a given point in time). |
| `GetAssociatedResource` | `GET /applications/{application}/resources/{resourceType}/{resource}` | `idempotent` | `application`, `resource`, `resourceType` | - | `GetAssociatedResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Gets the resource associated with the application. |
| `GetAttributeGroup` | `GET /attribute-groups/{attributeGroup}` | - | `attributeGroup` | - | `GetAttributeGroupResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Retrieves an attribute group by its ARN, ID, or name. The attribute group can be specified by its ARN, ID, or name. |
| `GetConfiguration` | `GET /configuration` | - | - | - | `GetConfigurationResponse` | `InternalServerException` | Retrieves a `TagKey` configuration from an account. |
| `ListApplications` | `GET /applications` | `idempotent`, `paginated` | - | - | `ListApplicationsResponse` | `InternalServerException`, `ValidationException` | Retrieves a list of all of your applications. Results are paginated. |
| `ListAssociatedAttributeGroups` | `GET /applications/{application}/attribute-groups` | `idempotent`, `paginated` | `application` | - | `ListAssociatedAttributeGroupsResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Lists all attribute groups that are associated with specified application. Results are paginated. |
| `ListAssociatedResources` | `GET /applications/{application}/resources` | `idempotent`, `paginated` | `application` | - | `ListAssociatedResourcesResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Lists all of the resources that are associated with the specified application. Results are paginated. |
| `ListAttributeGroups` | `GET /attribute-groups` | `idempotent`, `paginated` | - | - | `ListAttributeGroupsResponse` | `InternalServerException`, `ValidationException` | Lists all attribute groups which you have access to. Results are paginated. |
| `ListAttributeGroupsForApplication` | `GET /applications/{application}/attribute-group-details` | `idempotent`, `paginated` | `application` | - | `ListAttributeGroupsForApplicationResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Lists the details of all attribute groups associated with a specific application. The results display in pages. |
| `ListTagsForResource` | `GET /tags/{resourceArn}` | - | `resourceArn` | - | `ListTagsForResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Lists all of the tags on the resource. |
| `PutConfiguration` | `PUT /configuration` | - | `configuration` | - | `Unit` | `ConflictException`, `InternalServerException`, `ValidationException` | Associates a `TagKey` configuration to an account. |
| `SyncResource` | `POST /sync/{resourceType}/{resource}` | - | `resource`, `resourceType` | - | `SyncResourceResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Syncs the resource with current AppRegistry records. Specifically, the resource’s AppRegistry system tags sync with its associated application. |
| `TagResource` | `POST /tags/{resourceArn}` | - | `resourceArn`, `tags` | - | `TagResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Assigns one or more tags (key-value pairs) to the specified resource. Each tag consists of a key and an optional value. |
| `UntagResource` | `DELETE /tags/{resourceArn}` | - | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Removes tags from a resource. This operation returns an empty response if the call was successful. |
| `UpdateApplication` | `PATCH /applications/{application}` | - | `application` | - | `UpdateApplicationResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates an existing application with new attributes. |
| `UpdateAttributeGroup` | `PATCH /attribute-groups/{attributeGroup}` | - | `attributeGroup` | - | `UpdateAttributeGroupResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Updates an existing attribute group with new details. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InternalServerException` | `structure` | `message` | The service is experiencing internal problems. |
| `ValidationException` | `structure` | `message` | The request has invalid or missing parameters. |
| `ResourceNotFoundException` | `structure` | `message` | The specified resource does not exist. |
| `ConflictException` | `structure` | `message` | There was a conflict when processing the request (for example, a resource with the given name already exists within the account). |
| `ThrottlingException` | `structure` | `message`, `serviceCode` | The maximum number of API requests has been exceeded. |
| `ServiceQuotaExceededException` | `structure` | `message` | The maximum number of resources per account has been reached. |
| `AssociateAttributeGroupRequest` | `structure` | `application`, `attributeGroup` | - |
| `AssociateAttributeGroupResponse` | `structure` | `applicationArn`, `attributeGroupArn` | - |
| `AssociateResourceRequest` | `structure` | `application`, `options`, `resource`, `resourceType` | - |
| `AssociateResourceResponse` | `structure` | `applicationArn`, `options`, `resourceArn` | - |
| `CreateApplicationRequest` | `structure` | `clientToken`, `description`, `name`, `tags` | - |
| `CreateApplicationResponse` | `structure` | `application` | - |
| `CreateAttributeGroupRequest` | `structure` | `attributes`, `clientToken`, `description`, `name`, `tags` | - |
| `CreateAttributeGroupResponse` | `structure` | `attributeGroup` | - |
| `DeleteApplicationRequest` | `structure` | `application` | - |
| `DeleteApplicationResponse` | `structure` | `application` | - |
| `DeleteAttributeGroupRequest` | `structure` | `attributeGroup` | - |
| `DeleteAttributeGroupResponse` | `structure` | `attributeGroup` | - |
| `DisassociateAttributeGroupRequest` | `structure` | `application`, `attributeGroup` | - |
| `DisassociateAttributeGroupResponse` | `structure` | `applicationArn`, `attributeGroupArn` | - |
| `DisassociateResourceRequest` | `structure` | `application`, `resource`, `resourceType` | - |
| `DisassociateResourceResponse` | `structure` | `applicationArn`, `resourceArn` | - |
| `GetApplicationRequest` | `structure` | `application` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
