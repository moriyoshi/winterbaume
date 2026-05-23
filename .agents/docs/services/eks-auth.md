# Amazon EKS Auth

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

The Amazon EKS Auth API and the `AssumeRoleForPodIdentity` action are only used by the EKS Pod Identity Agent.

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented Amazon EKS Auth workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model workflows exposed by the modelled operation families across the `Assume` operation families, including `AssumeRoleForPodIdentity`.

## Service Identity and Protocol

- AWS model slug: `eks-auth`
- AWS SDK for Rust slug: `eksauth`
- Model version: `2023-11-26`
- Model file: `vendor/api-models-aws/models/eks-auth/service/2023-11-26/eks-auth-2023-11-26.json`
- SDK ID: `EKS Auth`
- Endpoint prefix: `eks-auth`
- ARN namespace: `eks-auth`
- CloudFormation name: `-`
- CloudTrail event source: `-`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Assume` (1).
- 1 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `EC2/VPC`, `ECS`, `EKS`.

## Operation Groups

### Assume

- Operations: `AssumeRoleForPodIdentity`
- Common required input members in this group: `clusterName`, `token`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AssumeRoleForPodIdentity` | `POST /clusters/{clusterName}/assume-role-for-pod-identity` | - | `clusterName`, `token` | - | `AssumeRoleForPodIdentityResponse` | `AccessDeniedException`, `ExpiredTokenException`, `InternalServerException`, `InvalidParameterException`, `InvalidRequestException`, `InvalidTokenException`, `ResourceNotFoundException`, `ServiceUnavailableException`, ... (+1) | The Amazon EKS Auth API and the `AssumeRoleForPodIdentity` action are only used by the EKS Pod Identity Agent. We recommend that applications use the Amazon Web Services SDKs to connect to Amazon Web Services services; if credentials from an EKS Pod Identity... |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AssumeRoleForPodIdentityRequest` | `structure` | `clusterName`, `token` | - |
| `AssumeRoleForPodIdentityResponse` | `structure` | `assumedRoleUser`, `audience`, `credentials`, `podIdentityAssociation`, `subject` | - |
| `AccessDeniedException` | `structure` | `message` | You don't have permissions to perform the requested operation. |
| `ExpiredTokenException` | `structure` | `message` | The specified Kubernetes service account token is expired. |
| `InternalServerException` | `structure` | `message` | These errors are usually caused by a server-side issue. |
| `InvalidParameterException` | `structure` | `message` | The specified parameter is invalid. |
| `InvalidRequestException` | `structure` | `message` | This exception is thrown if the request contains a semantic error. |
| `InvalidTokenException` | `structure` | `message` | The specified Kubernetes service account token is invalid. |
| `ResourceNotFoundException` | `structure` | `message` | The specified resource could not be found. |
| `ServiceUnavailableException` | `structure` | `message` | The service is unavailable. |
| `ThrottlingException` | `structure` | `message` | The request was denied because your request rate is too high. |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
