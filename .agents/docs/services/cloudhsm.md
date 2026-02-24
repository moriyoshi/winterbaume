# Amazon CloudHSM

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

AWS CloudHSM Service This is documentation for AWS CloudHSM Classic . For more information, see AWS CloudHSM Classic FAQs, the AWS CloudHSM Classic User Guide, and the AWS CloudHSM Classic API Reference. For information about the current version of AWS CloudHSM , see AWS CloudHSM, the AWS CloudHSM User Guide, and the AWS CloudHSM API Reference.

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented Amazon CloudHSM workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Create`, `Delete`, `Describe`, `Modify` operation families, including `ListAvailableZones`, `ListHapgs`, `ListHsms`, `ListLunaClients`, `CreateHapg`, `CreateHsm`.

## Service Identity and Protocol

- AWS model slug: `cloudhsm`
- AWS SDK for Rust slug: `cloudhsm`
- Model version: `2014-05-30`
- Model file: `vendor/api-models-aws/models/cloudhsm/service/2014-05-30/cloudhsm-2014-05-30.json`
- SDK ID: `CloudHSM`
- Endpoint prefix: `cloudhsm`
- ARN namespace: `cloudhsm`
- CloudFormation name: `CloudHSM`
- CloudTrail event source: `cloudhsm.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (5), `Create` (3), `Delete` (3), `Describe` (3), `Modify` (3), `Add` (1), `Get` (1), `Remove` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AddTagsToResource`, `CreateHapg`, `CreateHsm`, `CreateLunaClient`, `DeleteHapg`, `DeleteHsm`, `DeleteLunaClient`, `ModifyHapg`, `ModifyHsm`, `ModifyLunaClient`, `RemoveTagsFromResource`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeHapg`, `DescribeHsm`, `DescribeLunaClient`, `GetConfig`, `ListAvailableZones`, `ListHapgs`, `ListHsms`, `ListLunaClients`, `ListTagsForResource`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 20 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `EC2/VPC`.

## Operation Groups

### List

- Operations: `ListAvailableZones`, `ListHapgs`, `ListHsms`, `ListLunaClients`, `ListTagsForResource`
- Common required input members in this group: `ResourceArn`

### Create

- Operations: `CreateHapg`, `CreateHsm`, `CreateLunaClient`
- Common required input members in this group: `Certificate`, `IamRoleArn`, `Label`, `SshKey`, `SubnetId`, `SubscriptionType`

### Delete

- Operations: `DeleteHapg`, `DeleteHsm`, `DeleteLunaClient`
- Common required input members in this group: `ClientArn`, `HapgArn`, `HsmArn`

### Describe

- Operations: `DescribeHapg`, `DescribeHsm`, `DescribeLunaClient`
- Common required input members in this group: `HapgArn`

### Modify

- Operations: `ModifyHapg`, `ModifyHsm`, `ModifyLunaClient`
- Common required input members in this group: `Certificate`, `ClientArn`, `HapgArn`, `HsmArn`

### Add

- Operations: `AddTagsToResource`
- Common required input members in this group: `ResourceArn`, `TagList`

### Get

- Operations: `GetConfig`
- Common required input members in this group: `ClientArn`, `ClientVersion`, `HapgList`

### Remove

- Operations: `RemoveTagsFromResource`
- Common required input members in this group: `ResourceArn`, `TagKeyList`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AddTagsToResource` | - | - | `ResourceArn`, `TagList` | - | `AddTagsToResourceResponse` | `CloudHsmInternalException`, `CloudHsmServiceException`, `InvalidRequestException` | This is documentation for AWS CloudHSM Classic . For more information, see AWS CloudHSM Classic FAQs, the AWS CloudHSM Classic User Guide, and the AWS CloudHSM Classic API Reference. |
| `CreateHapg` | - | - | `Label` | - | `CreateHapgResponse` | `CloudHsmInternalException`, `CloudHsmServiceException`, `InvalidRequestException` | This is documentation for AWS CloudHSM Classic . For more information, see AWS CloudHSM Classic FAQs, the AWS CloudHSM Classic User Guide, and the AWS CloudHSM Classic API Reference. |
| `CreateHsm` | - | - | `IamRoleArn`, `SshKey`, `SubnetId`, `SubscriptionType` | - | `CreateHsmResponse` | `CloudHsmInternalException`, `CloudHsmServiceException`, `InvalidRequestException` | This is documentation for AWS CloudHSM Classic . For more information, see AWS CloudHSM Classic FAQs, the AWS CloudHSM Classic User Guide, and the AWS CloudHSM Classic API Reference. |
| `CreateLunaClient` | - | - | `Certificate` | - | `CreateLunaClientResponse` | `CloudHsmInternalException`, `CloudHsmServiceException`, `InvalidRequestException` | This is documentation for AWS CloudHSM Classic . For more information, see AWS CloudHSM Classic FAQs, the AWS CloudHSM Classic User Guide, and the AWS CloudHSM Classic API Reference. |
| `DeleteHapg` | - | - | `HapgArn` | - | `DeleteHapgResponse` | `CloudHsmInternalException`, `CloudHsmServiceException`, `InvalidRequestException` | This is documentation for AWS CloudHSM Classic . For more information, see AWS CloudHSM Classic FAQs, the AWS CloudHSM Classic User Guide, and the AWS CloudHSM Classic API Reference. |
| `DeleteHsm` | - | - | `HsmArn` | - | `DeleteHsmResponse` | `CloudHsmInternalException`, `CloudHsmServiceException`, `InvalidRequestException` | This is documentation for AWS CloudHSM Classic . For more information, see AWS CloudHSM Classic FAQs, the AWS CloudHSM Classic User Guide, and the AWS CloudHSM Classic API Reference. |
| `DeleteLunaClient` | - | - | `ClientArn` | - | `DeleteLunaClientResponse` | `CloudHsmInternalException`, `CloudHsmServiceException`, `InvalidRequestException` | This is documentation for AWS CloudHSM Classic . For more information, see AWS CloudHSM Classic FAQs, the AWS CloudHSM Classic User Guide, and the AWS CloudHSM Classic API Reference. |
| `DescribeHapg` | - | - | `HapgArn` | - | `DescribeHapgResponse` | `CloudHsmInternalException`, `CloudHsmServiceException`, `InvalidRequestException` | This is documentation for AWS CloudHSM Classic . For more information, see AWS CloudHSM Classic FAQs, the AWS CloudHSM Classic User Guide, and the AWS CloudHSM Classic API Reference. |
| `DescribeHsm` | - | - | - | - | `DescribeHsmResponse` | `CloudHsmInternalException`, `CloudHsmServiceException`, `InvalidRequestException` | This is documentation for AWS CloudHSM Classic . For more information, see AWS CloudHSM Classic FAQs, the AWS CloudHSM Classic User Guide, and the AWS CloudHSM Classic API Reference. |
| `DescribeLunaClient` | - | - | - | - | `DescribeLunaClientResponse` | `CloudHsmInternalException`, `CloudHsmServiceException`, `InvalidRequestException` | This is documentation for AWS CloudHSM Classic . For more information, see AWS CloudHSM Classic FAQs, the AWS CloudHSM Classic User Guide, and the AWS CloudHSM Classic API Reference. |
| `GetConfig` | - | - | `ClientArn`, `ClientVersion`, `HapgList` | - | `GetConfigResponse` | `CloudHsmInternalException`, `CloudHsmServiceException`, `InvalidRequestException` | This is documentation for AWS CloudHSM Classic . For more information, see AWS CloudHSM Classic FAQs, the AWS CloudHSM Classic User Guide, and the AWS CloudHSM Classic API Reference. |
| `ListAvailableZones` | - | - | - | - | `ListAvailableZonesResponse` | `CloudHsmInternalException`, `CloudHsmServiceException`, `InvalidRequestException` | This is documentation for AWS CloudHSM Classic . For more information, see AWS CloudHSM Classic FAQs, the AWS CloudHSM Classic User Guide, and the AWS CloudHSM Classic API Reference. |
| `ListHapgs` | - | - | - | - | `ListHapgsResponse` | `CloudHsmInternalException`, `CloudHsmServiceException`, `InvalidRequestException` | This is documentation for AWS CloudHSM Classic . For more information, see AWS CloudHSM Classic FAQs, the AWS CloudHSM Classic User Guide, and the AWS CloudHSM Classic API Reference. |
| `ListHsms` | - | - | - | - | `ListHsmsResponse` | `CloudHsmInternalException`, `CloudHsmServiceException`, `InvalidRequestException` | This is documentation for AWS CloudHSM Classic . For more information, see AWS CloudHSM Classic FAQs, the AWS CloudHSM Classic User Guide, and the AWS CloudHSM Classic API Reference. |
| `ListLunaClients` | - | - | - | - | `ListLunaClientsResponse` | `CloudHsmInternalException`, `CloudHsmServiceException`, `InvalidRequestException` | This is documentation for AWS CloudHSM Classic . For more information, see AWS CloudHSM Classic FAQs, the AWS CloudHSM Classic User Guide, and the AWS CloudHSM Classic API Reference. |
| `ListTagsForResource` | - | - | `ResourceArn` | - | `ListTagsForResourceResponse` | `CloudHsmInternalException`, `CloudHsmServiceException`, `InvalidRequestException` | This is documentation for AWS CloudHSM Classic . For more information, see AWS CloudHSM Classic FAQs, the AWS CloudHSM Classic User Guide, and the AWS CloudHSM Classic API Reference. |
| `ModifyHapg` | - | - | `HapgArn` | - | `ModifyHapgResponse` | `CloudHsmInternalException`, `CloudHsmServiceException`, `InvalidRequestException` | This is documentation for AWS CloudHSM Classic . For more information, see AWS CloudHSM Classic FAQs, the AWS CloudHSM Classic User Guide, and the AWS CloudHSM Classic API Reference. |
| `ModifyHsm` | - | - | `HsmArn` | - | `ModifyHsmResponse` | `CloudHsmInternalException`, `CloudHsmServiceException`, `InvalidRequestException` | This is documentation for AWS CloudHSM Classic . For more information, see AWS CloudHSM Classic FAQs, the AWS CloudHSM Classic User Guide, and the AWS CloudHSM Classic API Reference. |
| `ModifyLunaClient` | - | - | `Certificate`, `ClientArn` | - | `ModifyLunaClientResponse` | `CloudHsmServiceException` | This is documentation for AWS CloudHSM Classic . For more information, see AWS CloudHSM Classic FAQs, the AWS CloudHSM Classic User Guide, and the AWS CloudHSM Classic API Reference. |
| `RemoveTagsFromResource` | - | - | `ResourceArn`, `TagKeyList` | - | `RemoveTagsFromResourceResponse` | `CloudHsmInternalException`, `CloudHsmServiceException`, `InvalidRequestException` | This is documentation for AWS CloudHSM Classic . For more information, see AWS CloudHSM Classic FAQs, the AWS CloudHSM Classic User Guide, and the AWS CloudHSM Classic API Reference. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `CloudHsmServiceException` | `structure` | `message`, `retryable` | Indicates that an exception occurred in the AWS CloudHSM service. |
| `CloudHsmInternalException` | `structure` | `message`, `retryable` | Indicates that an internal error occurred. |
| `InvalidRequestException` | `structure` | `message`, `retryable` | Indicates that one or more of the request parameters are not valid. |
| `AddTagsToResourceRequest` | `structure` | `ResourceArn`, `TagList` | - |
| `AddTagsToResourceResponse` | `structure` | `Status` | - |
| `CreateHapgRequest` | `structure` | `Label` | Contains the inputs for the CreateHapgRequest action. |
| `CreateHapgResponse` | `structure` | `HapgArn` | Contains the output of the CreateHAPartitionGroup action. |
| `CreateHsmRequest` | `structure` | `ClientToken`, `EniIp`, `ExternalId`, `IamRoleArn`, `SshKey`, `SubnetId`, `SubscriptionType`, `SyslogIp` | Contains the inputs for the `CreateHsm` operation. |
| `CreateHsmResponse` | `structure` | `HsmArn` | Contains the output of the `CreateHsm` operation. |
| `CreateLunaClientRequest` | `structure` | `Certificate`, `Label` | Contains the inputs for the CreateLunaClient action. |
| `CreateLunaClientResponse` | `structure` | `ClientArn` | Contains the output of the CreateLunaClient action. |
| `DeleteHapgRequest` | `structure` | `HapgArn` | Contains the inputs for the DeleteHapg action. |
| `DeleteHapgResponse` | `structure` | `Status` | Contains the output of the DeleteHapg action. |
| `DeleteHsmRequest` | `structure` | `HsmArn` | Contains the inputs for the DeleteHsm operation. |
| `DeleteHsmResponse` | `structure` | `Status` | Contains the output of the DeleteHsm operation. |
| `DeleteLunaClientRequest` | `structure` | `ClientArn` | - |
| `DeleteLunaClientResponse` | `structure` | `Status` | - |
| `DescribeHapgRequest` | `structure` | `HapgArn` | Contains the inputs for the DescribeHapg action. |
| `DescribeHapgResponse` | `structure` | `HapgArn`, `HapgSerial`, `HsmsLastActionFailed`, `HsmsPendingDeletion`, `HsmsPendingRegistration`, `Label`, `LastModifiedTimestamp`, `PartitionSerialList`, `State` | Contains the output of the DescribeHapg action. |
| `DescribeHsmRequest` | `structure` | `HsmArn`, `HsmSerialNumber` | Contains the inputs for the DescribeHsm operation. |
| `DescribeHsmResponse` | `structure` | `AvailabilityZone`, `EniId`, `EniIp`, `HsmArn`, `HsmType`, `IamRoleArn`, `Partitions`, `SerialNumber`, `ServerCertLastUpdated`, `ServerCertUri`, `SoftwareVersion`, `SshKeyLastUpdated`, ... (+9) | Contains the output of the DescribeHsm operation. |
| `DescribeLunaClientRequest` | `structure` | `CertificateFingerprint`, `ClientArn` | - |
| `DescribeLunaClientResponse` | `structure` | `Certificate`, `CertificateFingerprint`, `ClientArn`, `Label`, `LastModifiedTimestamp` | - |
| `GetConfigRequest` | `structure` | `ClientArn`, `ClientVersion`, `HapgList` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
