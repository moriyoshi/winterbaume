# Amazon Cognito Identity

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Cognito Federated Identities Amazon Cognito Federated Identities is a web service that delivers scoped temporary credentials to mobile devices and other untrusted environments. It uniquely identifies a device and supplies the user with a consistent identity over the lifetime of an application. Using Amazon Cognito Federated Identities, you can enable authentication with one or more third-party identity providers (Facebook, Google, or Login with Amazon) or an Amazon Cognito user pool, and you can also choose to support unauthenticated access from your app. Cognito delivers a unique identifier for each user and acts as an OpenID token provider trusted by Security Token Service (STS) to access temporary, limited-privilege Amazon Web Services credentials. For a description of the authentication flow from the Amazon Cognito Developer Guide see Authentication Flow.

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented Amazon Cognito Identity workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model discovery and reporting workflows that retrieve, list, filter, or query service-managed state across the `Get`, `List`, `Delete`, `Describe`, `Set` operation families, including `GetCredentialsForIdentity`, `GetId`, `GetIdentityPoolRoles`, `GetOpenIdToken`, `ListIdentities`, `ListIdentityPools`.

## Service Identity and Protocol

- AWS model slug: `cognito-identity`
- AWS SDK for Rust slug: `cognitoidentity`
- Model version: `2014-06-30`
- Model file: `vendor/api-models-aws/models/cognito-identity/service/2014-06-30/cognito-identity-2014-06-30.json`
- SDK ID: `Cognito Identity`
- Endpoint prefix: `cognito-identity`
- ARN namespace: `cognito-identity`
- CloudFormation name: `Cognito`
- CloudTrail event source: `cognito-identity.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (6), `List` (3), `Delete` (2), `Describe` (2), `Set` (2), `Unlink` (2), `Create` (1), `Lookup` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateIdentityPool`, `DeleteIdentities`, `DeleteIdentityPool`, `SetIdentityPoolRoles`, `SetPrincipalTagAttributeMap`, `TagResource`, `UntagResource`, `UpdateIdentityPool`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeIdentity`, `DescribeIdentityPool`, `GetCredentialsForIdentity`, `GetId`, `GetIdentityPoolRoles`, `GetOpenIdToken`, `GetOpenIdTokenForDeveloperIdentity`, `GetPrincipalTagAttributeMap`, `ListIdentities`, `ListIdentityPools`, `ListTagsForResource`.
- Pagination is modelled for 1 operations; token stability and page boundaries are observable API behaviour.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 23 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `STS`.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-identity.html
- https://docs.aws.amazon.com/cognito/latest/developerguide/identity-pools.html

Research outcomes:
- An identity pool is a directory of federated identities that can be exchanged for temporary AWS credentials.
- Identity pools issue credentials for authenticated and unauthenticated users, with IAM roles and policies controlling access.
- Different roles can be used for authenticated and unauthenticated identities, and role selection can be based on user characteristics or token claims.
- Identity pools accept tokens or assertions from user pools, SAML, social/OIDC providers, and developer authenticated identity providers.
- Cognito customises SAML, OAuth, or OIDC provider claims into STS `AssumeRoleWithWebIdentity` requests for short-term credentials.
- Guest access for unauthenticated identities is supported.
- Cognito Sync can sync datasets per identity across devices. Client libraries can cache locally and sync when online.
- Console edits to dataset records are not saved until synchronised and are not visible to the end user until the identity synchronises.
- Push sync notifies all devices for an identity when a dataset changes. Cognito Streams can publish dataset changes to Kinesis, and Cognito Events can invoke Lambda on sync triggers.

Parity implications:
- Model identity pools, identities, login provider mappings, authenticated/unauthenticated roles, role mapping rules, OpenID tokens, credential issuance, datasets, records, sync sessions, streams, and triggers.
- Identity merge and role selection should depend on the supplied login map and configured identity-pool rules.
- Sync APIs should track per-identity datasets, record versions, local/server conflicts, and event/stream side effects.

## Operation Groups

### Get

- Operations: `GetCredentialsForIdentity`, `GetId`, `GetIdentityPoolRoles`, `GetOpenIdToken`, `GetOpenIdTokenForDeveloperIdentity`, `GetPrincipalTagAttributeMap`
- Common required input members in this group: `IdentityId`, `IdentityPoolId`, `IdentityProviderName`, `Logins`

### List

- Operations: `ListIdentities`, `ListIdentityPools`, `ListTagsForResource`
- Traits: `paginated` (1)
- Common required input members in this group: `IdentityPoolId`, `MaxResults`, `ResourceArn`

### Delete

- Operations: `DeleteIdentities`, `DeleteIdentityPool`
- Common required input members in this group: `IdentityIdsToDelete`, `IdentityPoolId`

### Describe

- Operations: `DescribeIdentity`, `DescribeIdentityPool`
- Common required input members in this group: `IdentityId`, `IdentityPoolId`

### Set

- Operations: `SetIdentityPoolRoles`, `SetPrincipalTagAttributeMap`
- Common required input members in this group: `IdentityPoolId`, `IdentityProviderName`, `Roles`

### Unlink

- Operations: `UnlinkDeveloperIdentity`, `UnlinkIdentity`
- Common required input members in this group: `DeveloperProviderName`, `DeveloperUserIdentifier`, `IdentityId`, `IdentityPoolId`, `Logins`, `LoginsToRemove`

### Create

- Operations: `CreateIdentityPool`
- Common required input members in this group: `AllowUnauthenticatedIdentities`, `IdentityPoolName`

### Lookup

- Operations: `LookupDeveloperIdentity`
- Common required input members in this group: `IdentityPoolId`

### Merge

- Operations: `MergeDeveloperIdentities`
- Common required input members in this group: `DestinationUserIdentifier`, `DeveloperProviderName`, `IdentityPoolId`, `SourceUserIdentifier`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `ResourceArn`, `Tags`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `ResourceArn`, `TagKeys`

### Update

- Operations: `UpdateIdentityPool`
- Common required input members in this group: `AllowUnauthenticatedIdentities`, `IdentityPoolId`, `IdentityPoolName`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateIdentityPool` | - | - | `AllowUnauthenticatedIdentities`, `IdentityPoolName` | - | `IdentityPool` | `InternalErrorException`, `InvalidParameterException`, `LimitExceededException`, `NotAuthorizedException`, `ResourceConflictException`, `TooManyRequestsException` | Creates a new identity pool. The identity pool is a store of user identity information that is specific to your Amazon Web Services account. |
| `DeleteIdentities` | - | - | `IdentityIdsToDelete` | - | `DeleteIdentitiesResponse` | `InternalErrorException`, `InvalidParameterException`, `TooManyRequestsException` | Deletes identities from an identity pool. You can specify a list of 1-60 identities that you want to delete. |
| `DeleteIdentityPool` | - | - | `IdentityPoolId` | - | `Unit` | `InternalErrorException`, `InvalidParameterException`, `NotAuthorizedException`, `ResourceNotFoundException`, `TooManyRequestsException` | Deletes an identity pool. Once a pool is deleted, users will not be able to authenticate with the pool. |
| `DescribeIdentity` | - | - | `IdentityId` | - | `IdentityDescription` | `InternalErrorException`, `InvalidParameterException`, `NotAuthorizedException`, `ResourceNotFoundException`, `TooManyRequestsException` | Returns metadata related to the given identity, including when the identity was created and any associated linked logins. You must use Amazon Web Services developer credentials to call this operation. |
| `DescribeIdentityPool` | - | - | `IdentityPoolId` | - | `IdentityPool` | `InternalErrorException`, `InvalidParameterException`, `NotAuthorizedException`, `ResourceNotFoundException`, `TooManyRequestsException` | Gets details about a particular identity pool, including the pool name, ID description, creation date, and current number of users. You must use Amazon Web Services developer credentials to call this operation. |
| `GetCredentialsForIdentity` | - | - | `IdentityId` | - | `GetCredentialsForIdentityResponse` | `ExternalServiceException`, `InternalErrorException`, `InvalidIdentityPoolConfigurationException`, `InvalidParameterException`, `NotAuthorizedException`, `ResourceConflictException`, `ResourceNotFoundException`, `TooManyRequestsException` | Returns credentials for the provided identity ID. Any provided logins will be validated against supported login providers. |
| `GetId` | - | - | `IdentityPoolId` | - | `GetIdResponse` | `ExternalServiceException`, `InternalErrorException`, `InvalidParameterException`, `LimitExceededException`, `NotAuthorizedException`, `ResourceConflictException`, `ResourceNotFoundException`, `TooManyRequestsException` | Generates (or retrieves) IdentityID. Supplying multiple logins will create an implicit linked account. |
| `GetIdentityPoolRoles` | - | - | `IdentityPoolId` | - | `GetIdentityPoolRolesResponse` | `InternalErrorException`, `InvalidParameterException`, `NotAuthorizedException`, `ResourceConflictException`, `ResourceNotFoundException`, `TooManyRequestsException` | Gets the roles for an identity pool. You must use Amazon Web Services developer credentials to call this operation. |
| `GetOpenIdToken` | - | - | `IdentityId` | - | `GetOpenIdTokenResponse` | `ExternalServiceException`, `InternalErrorException`, `InvalidParameterException`, `NotAuthorizedException`, `ResourceConflictException`, `ResourceNotFoundException`, `TooManyRequestsException` | Gets an OpenID token, using a known Cognito ID. This known Cognito ID is returned by GetId. |
| `GetOpenIdTokenForDeveloperIdentity` | - | - | `IdentityPoolId`, `Logins` | - | `GetOpenIdTokenForDeveloperIdentityResponse` | `DeveloperUserAlreadyRegisteredException`, `InternalErrorException`, `InvalidParameterException`, `NotAuthorizedException`, `ResourceConflictException`, `ResourceNotFoundException`, `TooManyRequestsException` | Registers (or retrieves) a Cognito `IdentityId` and an OpenID Connect token for a user authenticated by your backend authentication process. Supplying multiple logins will create an implicit linked account. |
| `GetPrincipalTagAttributeMap` | - | - | `IdentityPoolId`, `IdentityProviderName` | - | `GetPrincipalTagAttributeMapResponse` | `InternalErrorException`, `InvalidParameterException`, `NotAuthorizedException`, `ResourceNotFoundException`, `TooManyRequestsException` | Use `GetPrincipalTagAttributeMap` to list all mappings between `PrincipalTags` and user attributes. |
| `ListIdentities` | - | - | `IdentityPoolId`, `MaxResults` | - | `ListIdentitiesResponse` | `InternalErrorException`, `InvalidParameterException`, `NotAuthorizedException`, `ResourceNotFoundException`, `TooManyRequestsException` | Lists the identities in an identity pool. You must use Amazon Web Services developer credentials to call this operation. |
| `ListIdentityPools` | - | `paginated` | `MaxResults` | - | `ListIdentityPoolsResponse` | `InternalErrorException`, `InvalidParameterException`, `NotAuthorizedException`, `ResourceNotFoundException`, `TooManyRequestsException` | Lists all of the Cognito identity pools registered for your account. You must use Amazon Web Services developer credentials to call this operation. |
| `ListTagsForResource` | - | - | `ResourceArn` | - | `ListTagsForResourceResponse` | `InternalErrorException`, `InvalidParameterException`, `NotAuthorizedException`, `ResourceNotFoundException`, `TooManyRequestsException` | Lists the tags that are assigned to an Amazon Cognito identity pool. A tag is a label that you can apply to identity pools to categorize and manage them in different ways, such as by purpose, owner, environment, or other criteria. |
| `LookupDeveloperIdentity` | - | - | `IdentityPoolId` | - | `LookupDeveloperIdentityResponse` | `InternalErrorException`, `InvalidParameterException`, `NotAuthorizedException`, `ResourceConflictException`, `ResourceNotFoundException`, `TooManyRequestsException` | Retrieves the `IdentityID` associated with a `DeveloperUserIdentifier` or the list of `DeveloperUserIdentifier` values associated with an `IdentityId` for an existing identity. Either `IdentityID` or `DeveloperUserIdentifier` must not be null. |
| `MergeDeveloperIdentities` | - | - | `DestinationUserIdentifier`, `DeveloperProviderName`, `IdentityPoolId`, `SourceUserIdentifier` | - | `MergeDeveloperIdentitiesResponse` | `InternalErrorException`, `InvalidParameterException`, `NotAuthorizedException`, `ResourceConflictException`, `ResourceNotFoundException`, `TooManyRequestsException` | Merges two users having different `IdentityId`s, existing in the same identity pool, and identified by the same developer provider. You can use this action to request that discrete users be merged and identified as a single user in the Cognito environment. |
| `SetIdentityPoolRoles` | - | - | `IdentityPoolId`, `Roles` | - | `Unit` | `ConcurrentModificationException`, `InternalErrorException`, `InvalidParameterException`, `NotAuthorizedException`, `ResourceConflictException`, `ResourceNotFoundException`, `TooManyRequestsException` | Sets the roles for an identity pool. These roles are used when making calls to GetCredentialsForIdentity action. |
| `SetPrincipalTagAttributeMap` | - | - | `IdentityPoolId`, `IdentityProviderName` | - | `SetPrincipalTagAttributeMapResponse` | `InternalErrorException`, `InvalidParameterException`, `NotAuthorizedException`, `ResourceNotFoundException`, `TooManyRequestsException` | You can use this operation to use default (username and clientID) attribute or custom attribute mappings. |
| `TagResource` | - | - | `ResourceArn`, `Tags` | - | `TagResourceResponse` | `InternalErrorException`, `InvalidParameterException`, `NotAuthorizedException`, `ResourceNotFoundException`, `TooManyRequestsException` | Assigns a set of tags to the specified Amazon Cognito identity pool. A tag is a label that you can use to categorize and manage identity pools in different ways, such as by purpose, owner, environment, or other criteria. |
| `UnlinkDeveloperIdentity` | - | - | `DeveloperProviderName`, `DeveloperUserIdentifier`, `IdentityId`, `IdentityPoolId` | - | `Unit` | `InternalErrorException`, `InvalidParameterException`, `NotAuthorizedException`, `ResourceConflictException`, `ResourceNotFoundException`, `TooManyRequestsException` | Unlinks a `DeveloperUserIdentifier` from an existing identity. Unlinked developer users will be considered new identities next time they are seen. |
| `UnlinkIdentity` | - | - | `IdentityId`, `Logins`, `LoginsToRemove` | - | `Unit` | `ExternalServiceException`, `InternalErrorException`, `InvalidParameterException`, `NotAuthorizedException`, `ResourceConflictException`, `ResourceNotFoundException`, `TooManyRequestsException` | Unlinks a federated identity from an existing account. Unlinked logins will be considered new identities next time they are seen. |
| `UntagResource` | - | - | `ResourceArn`, `TagKeys` | - | `UntagResourceResponse` | `InternalErrorException`, `InvalidParameterException`, `NotAuthorizedException`, `ResourceNotFoundException`, `TooManyRequestsException` | Removes the specified tags from the specified Amazon Cognito identity pool. You can use this action up to 5 times per second, per account |
| `UpdateIdentityPool` | - | - | `AllowUnauthenticatedIdentities`, `IdentityPoolId`, `IdentityPoolName` | - | `IdentityPool` | `ConcurrentModificationException`, `InternalErrorException`, `InvalidParameterException`, `LimitExceededException`, `NotAuthorizedException`, `ResourceConflictException`, `ResourceNotFoundException`, `TooManyRequestsException` | Updates the configuration of an identity pool. If you don't provide a value for a parameter, Amazon Cognito sets it to its default value. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InternalErrorException` | `structure` | `message` | Thrown when the service encounters an error during processing the request. |
| `InvalidParameterException` | `structure` | `message` | Thrown for missing or bad input parameter(s). |
| `TooManyRequestsException` | `structure` | `message` | Thrown when a request is throttled. |
| `NotAuthorizedException` | `structure` | `message` | Thrown when a user is not authorized to access the requested resource. |
| `ResourceNotFoundException` | `structure` | `message` | Thrown when the requested resource (for example, a dataset or record) does not exist. |
| `ResourceConflictException` | `structure` | `message` | Thrown when a user tries to use a login which is already linked to another account. |
| `IdentityPool` | `structure` | `AllowClassicFlow`, `AllowUnauthenticatedIdentities`, `CognitoIdentityProviders`, `DeveloperProviderName`, `IdentityPoolId`, `IdentityPoolName`, `IdentityPoolTags`, `OpenIdConnectProviderARNs`, `SamlProviderARNs`, `SupportedLoginProviders` | An object representing an Amazon Cognito identity pool. |
| `ExternalServiceException` | `structure` | `message` | An exception thrown when a dependent service such as Facebook or Twitter is not responding |
| `LimitExceededException` | `structure` | `message` | Thrown when the total number of user pools has exceeded a preset limit. |
| `ConcurrentModificationException` | `structure` | `message` | Thrown if there are parallel requests to modify a resource. |
| `CreateIdentityPoolInput` | `structure` | `AllowClassicFlow`, `AllowUnauthenticatedIdentities`, `CognitoIdentityProviders`, `DeveloperProviderName`, `IdentityPoolName`, `IdentityPoolTags`, `OpenIdConnectProviderARNs`, `SamlProviderARNs`, `SupportedLoginProviders` | Input to the CreateIdentityPool action. |
| `DeleteIdentitiesInput` | `structure` | `IdentityIdsToDelete` | Input to the `DeleteIdentities` action. |
| `DeleteIdentitiesResponse` | `structure` | `UnprocessedIdentityIds` | Returned in response to a successful `DeleteIdentities` operation. |
| `DeleteIdentityPoolInput` | `structure` | `IdentityPoolId` | Input to the DeleteIdentityPool action. |
| `DescribeIdentityInput` | `structure` | `IdentityId` | Input to the `DescribeIdentity` action. |
| `IdentityDescription` | `structure` | `CreationDate`, `IdentityId`, `LastModifiedDate`, `Logins` | A description of the identity. |
| `DescribeIdentityPoolInput` | `structure` | `IdentityPoolId` | Input to the DescribeIdentityPool action. |
| `GetCredentialsForIdentityInput` | `structure` | `CustomRoleArn`, `IdentityId`, `Logins` | Input to the `GetCredentialsForIdentity` action. |
| `GetCredentialsForIdentityResponse` | `structure` | `Credentials`, `IdentityId` | Returned in response to a successful `GetCredentialsForIdentity` operation. |
| `InvalidIdentityPoolConfigurationException` | `structure` | `message` | If you provided authentication information in the request, the identity pool has no authenticated role configured, or STS returned an error response to the request to assume the... |
| `GetIdInput` | `structure` | `AccountId`, `IdentityPoolId`, `Logins` | Input to the GetId action. |
| `GetIdResponse` | `structure` | `IdentityId` | Returned in response to a GetId request. |
| `GetIdentityPoolRolesInput` | `structure` | `IdentityPoolId` | Input to the `GetIdentityPoolRoles` action. |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
