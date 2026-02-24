# Route 53 Profiles

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

With Amazon Route 53 Profiles you can share Route 53 configurations with VPCs and AWS accounts

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for Route 53 Profiles where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: cover association replacement for Route 53 Profiles by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- From the AWS documentation and model: represent documented Route 53 Profiles workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Get`, `Associate`, `Disassociate`, `Create` operation families, including `ListProfileAssociations`, `ListProfileResourceAssociations`, `ListProfiles`, `ListTagsForResource`, `GetProfile`, `GetProfileAssociation`.

## Service Identity and Protocol

- AWS model slug: `route53profiles`
- AWS SDK for Rust slug: `route53profiles`
- Model version: `2018-05-10`
- Model file: `vendor/api-models-aws/models/route53profiles/service/2018-05-10/route53profiles-2018-05-10.json`
- SDK ID: `Route53Profiles`
- Endpoint prefix: `-`
- ARN namespace: `route53profiles`
- CloudFormation name: `-`
- CloudTrail event source: `route53profiles.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (4), `Get` (3), `Associate` (2), `Disassociate` (2), `Create` (1), `Delete` (1), `Tag` (1), `Untag` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AssociateProfile`, `AssociateResourceToProfile`, `CreateProfile`, `DeleteProfile`, `DisassociateProfile`, `DisassociateResourceFromProfile`, `TagResource`, `UntagResource`, `UpdateProfileResourceAssociation`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetProfile`, `GetProfileAssociation`, `GetProfileResourceAssociation`, `ListProfileAssociations`, `ListProfileResourceAssociations`, `ListProfiles`, `ListTagsForResource`.
- Pagination is modelled for 3 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 6 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 16 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `EC2/VPC`.

## Operation Groups

### List

- Operations: `ListProfileAssociations`, `ListProfileResourceAssociations`, `ListProfiles`, `ListTagsForResource`
- Traits: `paginated` (3), `readonly` (4)
- Common required input members in this group: `ProfileId`, `ResourceArn`

### Get

- Operations: `GetProfile`, `GetProfileAssociation`, `GetProfileResourceAssociation`
- Traits: `readonly` (3)
- Common required input members in this group: `ProfileAssociationId`, `ProfileId`, `ProfileResourceAssociationId`

### Associate

- Operations: `AssociateProfile`, `AssociateResourceToProfile`
- Common required input members in this group: `Name`, `ProfileId`, `ResourceArn`, `ResourceId`

### Disassociate

- Operations: `DisassociateProfile`, `DisassociateResourceFromProfile`
- Traits: `idempotent` (2)
- Common required input members in this group: `ProfileId`, `ResourceArn`, `ResourceId`

### Create

- Operations: `CreateProfile`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `ClientToken`, `Name`

### Delete

- Operations: `DeleteProfile`
- Traits: `idempotent` (1)
- Common required input members in this group: `ProfileId`

### Tag

- Operations: `TagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `ResourceArn`, `Tags`

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `ResourceArn`, `TagKeys`

### Update

- Operations: `UpdateProfileResourceAssociation`
- Common required input members in this group: `ProfileResourceAssociationId`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AssociateProfile` | `POST /profileassociation` | - | `Name`, `ProfileId`, `ResourceId` | - | `AssociateProfileResponse` | `AccessDeniedException`, `ConflictException`, `InvalidParameterException`, `LimitExceededException`, `ResourceExistsException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Associates a Route 53 Profiles profile with a VPC. A VPC can have only one Profile associated with it, but a Profile can be associated with 1000 of VPCs (and you can request a higher quota). |
| `AssociateResourceToProfile` | `POST /profileresourceassociation` | - | `Name`, `ProfileId`, `ResourceArn` | - | `AssociateResourceToProfileResponse` | `AccessDeniedException`, `ConflictException`, `InternalServiceErrorException`, `InvalidParameterException`, `LimitExceededException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Associates a DNS reource configuration to a Route 53 Profile. |
| `CreateProfile` | `POST /profile` | `idempotency-token` | `ClientToken`, `Name` | `ClientToken` | `CreateProfileResponse` | `AccessDeniedException`, `InvalidParameterException`, `LimitExceededException`, `ThrottlingException`, `ValidationException` | Creates an empty Route 53 Profile. |
| `DeleteProfile` | `DELETE /profile/{ProfileId}` | `idempotent` | `ProfileId` | - | `DeleteProfileResponse` | `AccessDeniedException`, `ConflictException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes the specified Route 53 Profile. Before you can delete a profile, you must first disassociate it from all VPCs. |
| `DisassociateProfile` | `DELETE /profileassociation/Profileid/{ProfileId}/resourceid/{ResourceId}` | `idempotent` | `ProfileId`, `ResourceId` | - | `DisassociateProfileResponse` | `AccessDeniedException`, `InvalidParameterException`, `LimitExceededException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Dissociates a specified Route 53 Profile from the specified VPC. |
| `DisassociateResourceFromProfile` | `DELETE /profileresourceassociation/profileid/{ProfileId}/resourcearn/{ResourceArn}` | `idempotent` | `ProfileId`, `ResourceArn` | - | `DisassociateResourceFromProfileResponse` | `AccessDeniedException`, `ConflictException`, `InternalServiceErrorException`, `InvalidParameterException`, `LimitExceededException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Dissoaciated a specified resource, from the Route 53 Profile. |
| `GetProfile` | `GET /profile/{ProfileId}` | `readonly` | `ProfileId` | - | `GetProfileResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns information about a specified Route 53 Profile, such as whether whether the Profile is shared, and the current status of the Profile. |
| `GetProfileAssociation` | `GET /profileassociation/{ProfileAssociationId}` | `readonly` | `ProfileAssociationId` | - | `GetProfileAssociationResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves a Route 53 Profile association for a VPC. A VPC can have only one Profile association, but a Profile can be associated with up to 5000 VPCs. |
| `GetProfileResourceAssociation` | `GET /profileresourceassociation/{ProfileResourceAssociationId}` | `readonly` | `ProfileResourceAssociationId` | - | `GetProfileResourceAssociationResponse` | `AccessDeniedException`, `InvalidParameterException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns information about a specified Route 53 Profile resource association. |
| `ListProfileAssociations` | `GET /profileassociations` | `readonly`, `paginated` | - | - | `ListProfileAssociationsResponse` | `AccessDeniedException`, `InvalidNextTokenException`, `InvalidParameterException`, `ThrottlingException`, `ValidationException` | Lists all the VPCs that the specified Route 53 Profile is associated with. |
| `ListProfileResourceAssociations` | `GET /profileresourceassociations/profileid/{ProfileId}` | `readonly`, `paginated` | `ProfileId` | - | `ListProfileResourceAssociationsResponse` | `AccessDeniedException`, `InternalServiceErrorException`, `InvalidNextTokenException`, `InvalidParameterException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists all the resource associations for the specified Route 53 Profile. |
| `ListProfiles` | `GET /profiles` | `readonly`, `paginated` | - | - | `ListProfilesResponse` | `AccessDeniedException`, `InvalidNextTokenException`, `InvalidParameterException`, `ThrottlingException`, `ValidationException` | Lists all the Route 53 Profiles associated with your Amazon Web Services account. |
| `ListTagsForResource` | `GET /tags/{ResourceArn}` | `readonly` | `ResourceArn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `ConflictException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists the tags that you associated with the specified resource. |
| `TagResource` | `POST /tags/{ResourceArn}` | `idempotent` | `ResourceArn`, `Tags` | - | `TagResourceResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Adds one or more tags to a specified resource. |
| `UntagResource` | `DELETE /tags/{ResourceArn}` | `idempotent` | `ResourceArn`, `TagKeys` | - | `UntagResourceResponse` | `AccessDeniedException`, `ConflictException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes one or more tags from a specified resource. |
| `UpdateProfileResourceAssociation` | `PATCH /profileresourceassociation/{ProfileResourceAssociationId}` | - | `ProfileResourceAssociationId` | - | `UpdateProfileResourceAssociationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServiceErrorException`, `InvalidParameterException`, `LimitExceededException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates the specified Route 53 Profile resourse association. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | `Message` | The current account doesn't have the IAM permissions required to perform the specified operation. |
| `ThrottlingException` | `structure` | `Message` | The request was throttled. |
| `ValidationException` | `structure` | `Message` | You have provided an invalid command. |
| `ResourceNotFoundException` | `structure` | `Message`, `ResourceType` | The resource you are associating is not found. |
| `InvalidParameterException` | `structure` | `FieldName`, `Message` | One or more parameters in this request are not valid. |
| `ConflictException` | `structure` | `Message` | The request you submitted conflicts with an existing request. |
| `LimitExceededException` | `structure` | `Message`, `ResourceType` | The request caused one or more limits to be exceeded. |
| `InternalServiceErrorException` | `structure` | `Message` | An internal server error occured. |
| `InvalidNextTokenException` | `structure` | `Message` | The `NextToken` you provided isn;t valid. |
| `AssociateProfileRequest` | `structure` | `Name`, `ProfileId`, `ResourceId`, `Tags` | - |
| `AssociateProfileResponse` | `structure` | `ProfileAssociation` | - |
| `ResourceExistsException` | `structure` | `Message`, `ResourceType` | The resource you are trying to associate, has already been associated. |
| `AssociateResourceToProfileRequest` | `structure` | `Name`, `ProfileId`, `ResourceArn`, `ResourceProperties` | - |
| `AssociateResourceToProfileResponse` | `structure` | `ProfileResourceAssociation` | - |
| `CreateProfileRequest` | `structure` | `ClientToken`, `Name`, `Tags` | - |
| `CreateProfileResponse` | `structure` | `Profile` | - |
| `DeleteProfileRequest` | `structure` | `ProfileId` | - |
| `DeleteProfileResponse` | `structure` | `Profile` | - |
| `DisassociateProfileRequest` | `structure` | `ProfileId`, `ResourceId` | - |
| `DisassociateProfileResponse` | `structure` | `ProfileAssociation` | - |
| `DisassociateResourceFromProfileRequest` | `structure` | `ProfileId`, `ResourceArn` | - |
| `DisassociateResourceFromProfileResponse` | `structure` | `ProfileResourceAssociation` | - |
| `GetProfileRequest` | `structure` | `ProfileId` | - |
| `GetProfileResponse` | `structure` | `Profile` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
