# AWS EC2 Instance Connect

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

This is the Amazon EC2 Instance Connect API Reference . It provides descriptions, syntax, and usage examples for each of the actions for Amazon EC2 Instance Connect. Amazon EC2 Instance Connect enables system administrators to publish one-time use SSH public keys to EC2, providing users a simple and secure way to connect to their instances. To view the Amazon EC2 Instance Connect content in the Amazon EC2 User Guide , see Connect to your Linux instance using EC2 Instance Connect. For Amazon EC2 APIs, see the Amazon EC2 API Reference.

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented AWS EC2 Instance Connect workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model workflows exposed by the modelled operation families across the `Send` operation families, including `SendSSHPublicKey`, `SendSerialConsoleSSHPublicKey`.

## Service Identity and Protocol

- AWS model slug: `ec2-instance-connect`
- AWS SDK for Rust slug: `ec2instanceconnect`
- Model version: `2018-04-02`
- Model file: `vendor/api-models-aws/models/ec2-instance-connect/service/2018-04-02/ec2-instance-connect-2018-04-02.json`
- SDK ID: `EC2 Instance Connect`
- Endpoint prefix: `ec2-instance-connect`
- ARN namespace: `ec2-instance-connect`
- CloudFormation name: `EC2InstanceConnect`
- CloudTrail event source: `ec2instanceconnect.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Send` (2).
- 2 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `EC2/VPC`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Current Network Resource Stub Semantics

EC2 Instance Connect stores endpoint networking as service-local endpoint records.

- Endpoint state records include subnet ID, VPC ID, security group IDs, preserve-client-IP mode, DNS name, FIPS DNS name, and network interface IDs.
- Created endpoints mint their own endpoint and network-interface-looking identifiers, and describe/list responses echo the stored fields.
- Endpoint lifecycle is not reflected into EC2 network interface state.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Operation Groups

### Send

- Operations: `SendSSHPublicKey`, `SendSerialConsoleSSHPublicKey`
- Common required input members in this group: `InstanceId`, `InstanceOSUser`, `SSHPublicKey`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `SendSSHPublicKey` | - | - | `InstanceId`, `InstanceOSUser`, `SSHPublicKey` | - | `SendSSHPublicKeyResponse` | `AuthException`, `EC2InstanceNotFoundException`, `EC2InstanceStateInvalidException`, `EC2InstanceUnavailableException`, `InvalidArgsException`, `ServiceException`, `ThrottlingException` | Pushes an SSH public key to the specified EC2 instance for use by the specified user. The key remains for 60 seconds. |
| `SendSerialConsoleSSHPublicKey` | - | - | `InstanceId`, `SSHPublicKey` | - | `SendSerialConsoleSSHPublicKeyResponse` | `AuthException`, `EC2InstanceNotFoundException`, `EC2InstanceStateInvalidException`, `EC2InstanceTypeInvalidException`, `EC2InstanceUnavailableException`, `InvalidArgsException`, `SerialConsoleAccessDisabledException`, `SerialConsoleSessionLimitExceededException`, ... (+4) | Pushes an SSH public key to the specified EC2 instance. The key remains for 60 seconds, which gives you 60 seconds to establish a serial console connection to the instance using SSH. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AuthException` | `structure` | `Message` | Either your AWS credentials are not valid or you do not have access to the EC2 instance. |
| `EC2InstanceNotFoundException` | `structure` | `Message` | The specified instance was not found. |
| `EC2InstanceStateInvalidException` | `structure` | `Message` | Unable to connect because the instance is not in a valid state. |
| `EC2InstanceUnavailableException` | `structure` | `Message` | The instance is currently unavailable. |
| `InvalidArgsException` | `structure` | `Message` | One of the parameters is not valid. |
| `ServiceException` | `structure` | `Message` | The service encountered an error. |
| `ThrottlingException` | `structure` | `Message` | The requests were made too frequently and have been throttled. |
| `SendSSHPublicKeyRequest` | `structure` | `AvailabilityZone`, `InstanceId`, `InstanceOSUser`, `SSHPublicKey` | - |
| `SendSSHPublicKeyResponse` | `structure` | `RequestId`, `Success` | - |
| `SendSerialConsoleSSHPublicKeyRequest` | `structure` | `InstanceId`, `SSHPublicKey`, `SerialPort` | - |
| `SendSerialConsoleSSHPublicKeyResponse` | `structure` | `RequestId`, `Success` | - |
| `EC2InstanceTypeInvalidException` | `structure` | `Message` | The instance type is not supported for connecting via the serial console. |
| `SerialConsoleAccessDisabledException` | `structure` | `Message` | Your account is not authorized to use the EC2 Serial Console. |
| `SerialConsoleSessionLimitExceededException` | `structure` | `Message` | The instance currently has 1 active serial console session. |
| `SerialConsoleSessionUnavailableException` | `structure` | `Message` | Unable to start a serial console session. |
| `SerialConsoleSessionUnsupportedException` | `structure` | `Message` | Your instance's BIOS version is unsupported for serial console connection. |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
