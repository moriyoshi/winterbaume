# AWS Single Sign-On

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

AWS IAM Identity Center (successor to AWS Single Sign-On) Portal is a web service that makes it easy for you to assign user access to IAM Identity Center resources such as the AWS access portal. Users can get AWS account applications and roles assigned to them and get federated into the application. Although AWS Single Sign-On was renamed, the `sso` and `identitystore` API namespaces will continue to retain their original name for backward compatibility purposes. For more information, see IAM Identity Center rename. This reference guide describes the IAM Identity Center Portal operations that you can call programatically and includes detailed information on data types and errors.

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented AWS Single Sign-On workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model discovery and reporting workflows that retrieve, list, filter, or query service-managed state across the `List`, `Get`, `Logout` operation families, including `ListAccountRoles`, `ListAccounts`, `GetRoleCredentials`, `Logout`.

## Service Identity and Protocol

- AWS model slug: `sso`
- AWS SDK for Rust slug: `sso`
- Model version: `2019-06-10`
- Model file: `vendor/api-models-aws/models/sso/service/2019-06-10/sso-2019-06-10.json`
- SDK ID: `SSO`
- Endpoint prefix: `portal.sso`
- ARN namespace: `awsssoportal`
- CloudFormation name: `SSO`
- CloudTrail event source: `sso.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (2), `Get` (1), `Logout` (1).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetRoleCredentials`, `ListAccountRoles`, `ListAccounts`.
- Pagination is modelled for 2 operations; token stability and page boundaries are observable API behaviour.
- 4 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `STS`.

## Operation Groups

### List

- Operations: `ListAccountRoles`, `ListAccounts`
- Traits: `paginated` (2)
- Common required input members in this group: `accessToken`

### Get

- Operations: `GetRoleCredentials`
- Common required input members in this group: -

### Logout

- Operations: `Logout`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `GetRoleCredentials` | `GET /federation/credentials` | - | `roleName`, `accountId`, `accessToken` | - | `GetRoleCredentialsResponse` | `InvalidRequestException`, `ResourceNotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Returns the STS short-term credentials for a given role name that is assigned to the user. |
| `ListAccountRoles` | `GET /assignment/roles` | `paginated` | `accessToken`, `accountId` | - | `ListAccountRolesResponse` | `InvalidRequestException`, `ResourceNotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Lists all roles that are assigned to the user for a given AWS account. |
| `ListAccounts` | `GET /assignment/accounts` | `paginated` | `accessToken` | - | `ListAccountsResponse` | `InvalidRequestException`, `ResourceNotFoundException`, `TooManyRequestsException`, `UnauthorizedException` | Lists all AWS accounts assigned to the user. These AWS accounts are assigned by the administrator of the account. For more information, see Assign User Access in the IAM Identity Center User Guide . This operation re ... |
| `Logout` | `POST /logout` | - | `accessToken` | - | `Unit` | `InvalidRequestException`, `TooManyRequestsException`, `UnauthorizedException` | Removes the locally stored SSO tokens from the client-side cache and sends an API call to the IAM Identity Center service to invalidate the corresponding server-side IAM Identity Center sign in session. If a user use ... |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `GetRoleCredentials` | `accessToken -> x-amz-sso_bearer_token` | `roleName -> role_name`, `accountId -> account_id` | - | - |
| `ListAccountRoles` | `accessToken -> x-amz-sso_bearer_token` | `nextToken -> next_token`, `maxResults -> max_result`, `accountId -> account_id` | - | - |
| `ListAccounts` | `accessToken -> x-amz-sso_bearer_token` | `nextToken -> next_token`, `maxResults -> max_result` | - | - |
| `Logout` | `accessToken -> x-amz-sso_bearer_token` | - | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InvalidRequestException` | `structure` | message | Indicates that a problem occurred with the input to the request. For example, a required parameter might be missing or out of range. |
| `ResourceNotFoundException` | `structure` | message | The specified resource doesn't exist. |
| `TooManyRequestsException` | `structure` | message | Indicates that the request is being made too frequently and is more than what the server can handle. |
| `UnauthorizedException` | `structure` | message | Indicates that the request is not authorized. This can happen due to an invalid access token in the request. |
| `GetRoleCredentialsRequest` | `structure` | roleName, accountId, accessToken | - |
| `GetRoleCredentialsResponse` | `structure` | roleCredentials | - |
| `ListAccountRolesRequest` | `structure` | nextToken, maxResults, accessToken, accountId | - |
| `ListAccountRolesResponse` | `structure` | nextToken, roleList | - |
| `ListAccountsRequest` | `structure` | nextToken, maxResults, accessToken | - |
| `ListAccountsResponse` | `structure` | nextToken, accountList | - |
| `LogoutRequest` | `structure` | accessToken | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
