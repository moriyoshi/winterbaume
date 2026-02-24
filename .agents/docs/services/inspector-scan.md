# Inspector Scan

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Inspector Scan is a vulnerability discovery service that scans a provided Software Bill of Materials (SBOM) for security vulnerabilities.

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented Inspector Scan workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model workflows exposed by the modelled operation families across the `Scan` operation families, including `ScanSbom`.

## Service Identity and Protocol

- AWS model slug: `inspector-scan`
- AWS SDK for Rust slug: `inspectorscan`
- Model version: `2023-08-08`
- Model file: `vendor/api-models-aws/models/inspector-scan/service/2023-08-08/inspector-scan-2023-08-08.json`
- SDK ID: `Inspector Scan`
- Endpoint prefix: `-`
- ARN namespace: `inspector-scan`
- CloudFormation name: `InspectorScan`
- CloudTrail event source: `inspector-scan.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Scan` (1).
- Idempotency is explicit for 1 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `ScanSbom`.
- 1 operations declare modelled service errors; parity work should map exact error names and retryability where documented.

## Operation Groups

### Scan

- Operations: `ScanSbom`
- Traits: `idempotent` (1)
- Common required input members in this group: `sbom`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `ScanSbom` | `POST /scan/sbom` | `idempotent` | `sbom` | - | `ScanSbomResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Scans a provided CycloneDX 1.5 SBOM and reports on any vulnerabilities discovered in that SBOM. You can generate compatible SBOMs for your resources using the Amazon Inspector SBOM generator. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `ScanSbomRequest` | `structure` | `outputFormat`, `sbom` | - |
| `ScanSbomResponse` | `structure` | `sbom` | - |
| `AccessDeniedException` | `structure` | `message` | You do not have sufficient access to perform this action. |
| `InternalServerException` | `structure` | `message`, `reason`, `retryAfterSeconds` | The request processing has failed because of an unknown error, exception or failure. |
| `ThrottlingException` | `structure` | `message`, `retryAfterSeconds` | The request was denied due to request throttling. |
| `ValidationException` | `structure` | `fields`, `message`, `reason` | The request has failed validation due to missing required fields or having invalid inputs. |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
