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
- Common required input members in this group: -

### Create

- Operations: `CreateHapg`, `CreateHsm`, `CreateLunaClient`
- Common required input members in this group: -

### Delete

- Operations: `DeleteHapg`, `DeleteHsm`, `DeleteLunaClient`
- Common required input members in this group: -

### Describe

- Operations: `DescribeHapg`, `DescribeHsm`, `DescribeLunaClient`
- Common required input members in this group: -

### Modify

- Operations: `ModifyHapg`, `ModifyHsm`, `ModifyLunaClient`
- Common required input members in this group: -

### Add

- Operations: `AddTagsToResource`
- Common required input members in this group: -

### Get

- Operations: `GetConfig`
- Common required input members in this group: -

### Remove

- Operations: `RemoveTagsFromResource`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AddTagsToResource` | `-` | - | `ResourceArn`, `TagList` | - | `AddTagsToResourceResponse` | `CloudHsmInternalException`, `CloudHsmServiceException`, `InvalidRequestException` | This is documentation for AWS CloudHSM Classic . For more information, see AWS CloudHSM Classic FAQs , the AWS CloudHSM Classic User Guide , and the AWS CloudHSM Classic API Reference . For information about the curr ... |
| `CreateHapg` | `-` | - | `Label` | - | `CreateHapgResponse` | `CloudHsmInternalException`, `CloudHsmServiceException`, `InvalidRequestException` | This is documentation for AWS CloudHSM Classic . For more information, see AWS CloudHSM Classic FAQs , the AWS CloudHSM Classic User Guide , and the AWS CloudHSM Classic API Reference . For information about the curr ... |
| `CreateHsm` | `-` | - | `SubnetId`, `SshKey`, `IamRoleArn`, `SubscriptionType` | - | `CreateHsmResponse` | `CloudHsmInternalException`, `CloudHsmServiceException`, `InvalidRequestException` | This is documentation for AWS CloudHSM Classic . For more information, see AWS CloudHSM Classic FAQs , the AWS CloudHSM Classic User Guide , and the AWS CloudHSM Classic API Reference . For information about the curr ... |
| `CreateLunaClient` | `-` | - | `Certificate` | - | `CreateLunaClientResponse` | `CloudHsmInternalException`, `CloudHsmServiceException`, `InvalidRequestException` | This is documentation for AWS CloudHSM Classic . For more information, see AWS CloudHSM Classic FAQs , the AWS CloudHSM Classic User Guide , and the AWS CloudHSM Classic API Reference . For information about the curr ... |
| `DeleteHapg` | `-` | - | `HapgArn` | - | `DeleteHapgResponse` | `CloudHsmInternalException`, `CloudHsmServiceException`, `InvalidRequestException` | This is documentation for AWS CloudHSM Classic . For more information, see AWS CloudHSM Classic FAQs , the AWS CloudHSM Classic User Guide , and the AWS CloudHSM Classic API Reference . For information about the curr ... |
| `DeleteHsm` | `-` | - | `HsmArn` | - | `DeleteHsmResponse` | `CloudHsmInternalException`, `CloudHsmServiceException`, `InvalidRequestException` | This is documentation for AWS CloudHSM Classic . For more information, see AWS CloudHSM Classic FAQs , the AWS CloudHSM Classic User Guide , and the AWS CloudHSM Classic API Reference . For information about the curr ... |
| `DeleteLunaClient` | `-` | - | `ClientArn` | - | `DeleteLunaClientResponse` | `CloudHsmInternalException`, `CloudHsmServiceException`, `InvalidRequestException` | This is documentation for AWS CloudHSM Classic . For more information, see AWS CloudHSM Classic FAQs , the AWS CloudHSM Classic User Guide , and the AWS CloudHSM Classic API Reference . For information about the curr ... |
| `DescribeHapg` | `-` | - | `HapgArn` | - | `DescribeHapgResponse` | `CloudHsmInternalException`, `CloudHsmServiceException`, `InvalidRequestException` | This is documentation for AWS CloudHSM Classic . For more information, see AWS CloudHSM Classic FAQs , the AWS CloudHSM Classic User Guide , and the AWS CloudHSM Classic API Reference . For information about the curr ... |
| `DescribeHsm` | `-` | - | - | - | `DescribeHsmResponse` | `CloudHsmInternalException`, `CloudHsmServiceException`, `InvalidRequestException` | This is documentation for AWS CloudHSM Classic . For more information, see AWS CloudHSM Classic FAQs , the AWS CloudHSM Classic User Guide , and the AWS CloudHSM Classic API Reference . For information about the curr ... |
| `DescribeLunaClient` | `-` | - | - | - | `DescribeLunaClientResponse` | `CloudHsmInternalException`, `CloudHsmServiceException`, `InvalidRequestException` | This is documentation for AWS CloudHSM Classic . For more information, see AWS CloudHSM Classic FAQs , the AWS CloudHSM Classic User Guide , and the AWS CloudHSM Classic API Reference . For information about the curr ... |
| `GetConfig` | `-` | - | `ClientArn`, `ClientVersion`, `HapgList` | - | `GetConfigResponse` | `CloudHsmInternalException`, `CloudHsmServiceException`, `InvalidRequestException` | This is documentation for AWS CloudHSM Classic . For more information, see AWS CloudHSM Classic FAQs , the AWS CloudHSM Classic User Guide , and the AWS CloudHSM Classic API Reference . For information about the curr ... |
| `ListAvailableZones` | `-` | - | - | - | `ListAvailableZonesResponse` | `CloudHsmInternalException`, `CloudHsmServiceException`, `InvalidRequestException` | This is documentation for AWS CloudHSM Classic . For more information, see AWS CloudHSM Classic FAQs , the AWS CloudHSM Classic User Guide , and the AWS CloudHSM Classic API Reference . For information about the curr ... |
| `ListHapgs` | `-` | - | - | - | `ListHapgsResponse` | `CloudHsmInternalException`, `CloudHsmServiceException`, `InvalidRequestException` | This is documentation for AWS CloudHSM Classic . For more information, see AWS CloudHSM Classic FAQs , the AWS CloudHSM Classic User Guide , and the AWS CloudHSM Classic API Reference . For information about the curr ... |
| `ListHsms` | `-` | - | - | - | `ListHsmsResponse` | `CloudHsmInternalException`, `CloudHsmServiceException`, `InvalidRequestException` | This is documentation for AWS CloudHSM Classic . For more information, see AWS CloudHSM Classic FAQs , the AWS CloudHSM Classic User Guide , and the AWS CloudHSM Classic API Reference . For information about the curr ... |
| `ListLunaClients` | `-` | - | - | - | `ListLunaClientsResponse` | `CloudHsmInternalException`, `CloudHsmServiceException`, `InvalidRequestException` | This is documentation for AWS CloudHSM Classic . For more information, see AWS CloudHSM Classic FAQs , the AWS CloudHSM Classic User Guide , and the AWS CloudHSM Classic API Reference . For information about the curr ... |
| `ListTagsForResource` | `-` | - | `ResourceArn` | - | `ListTagsForResourceResponse` | `CloudHsmInternalException`, `CloudHsmServiceException`, `InvalidRequestException` | This is documentation for AWS CloudHSM Classic . For more information, see AWS CloudHSM Classic FAQs , the AWS CloudHSM Classic User Guide , and the AWS CloudHSM Classic API Reference . For information about the curr ... |
| `ModifyHapg` | `-` | - | `HapgArn` | - | `ModifyHapgResponse` | `CloudHsmInternalException`, `CloudHsmServiceException`, `InvalidRequestException` | This is documentation for AWS CloudHSM Classic . For more information, see AWS CloudHSM Classic FAQs , the AWS CloudHSM Classic User Guide , and the AWS CloudHSM Classic API Reference . For information about the curr ... |
| `ModifyHsm` | `-` | - | `HsmArn` | - | `ModifyHsmResponse` | `CloudHsmInternalException`, `CloudHsmServiceException`, `InvalidRequestException` | This is documentation for AWS CloudHSM Classic . For more information, see AWS CloudHSM Classic FAQs , the AWS CloudHSM Classic User Guide , and the AWS CloudHSM Classic API Reference . For information about the curr ... |
| `ModifyLunaClient` | `-` | - | `ClientArn`, `Certificate` | - | `ModifyLunaClientResponse` | `CloudHsmServiceException` | This is documentation for AWS CloudHSM Classic . For more information, see AWS CloudHSM Classic FAQs , the AWS CloudHSM Classic User Guide , and the AWS CloudHSM Classic API Reference . For information about the curr ... |
| `RemoveTagsFromResource` | `-` | - | `ResourceArn`, `TagKeyList` | - | `RemoveTagsFromResourceResponse` | `CloudHsmInternalException`, `CloudHsmServiceException`, `InvalidRequestException` | This is documentation for AWS CloudHSM Classic . For more information, see AWS CloudHSM Classic FAQs , the AWS CloudHSM Classic User Guide , and the AWS CloudHSM Classic API Reference . For information about the curr ... |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `CloudHsmInternalException` | `structure` | message, retryable | Indicates that an internal error occurred. |
| `CloudHsmServiceException` | `structure` | message, retryable | Indicates that an exception occurred in the AWS CloudHSM service. |
| `InvalidRequestException` | `structure` | message, retryable | Indicates that one or more of the request parameters are not valid. |
| `AddTagsToResourceRequest` | `structure` | ResourceArn, TagList | - |
| `AddTagsToResourceResponse` | `structure` | Status | - |
| `CreateHapgRequest` | `structure` | Label | Contains the inputs for the CreateHapgRequest action. |
| `CreateHapgResponse` | `structure` | HapgArn | Contains the output of the CreateHAPartitionGroup action. |
| `CreateHsmRequest` | `structure` | SubnetId, SshKey, EniIp, IamRoleArn, ExternalId, SubscriptionType, ClientToken, SyslogIp | Contains the inputs for the CreateHsm operation. |
| `CreateHsmResponse` | `structure` | HsmArn | Contains the output of the CreateHsm operation. |
| `CreateLunaClientRequest` | `structure` | Label, Certificate | Contains the inputs for the CreateLunaClient action. |
| `CreateLunaClientResponse` | `structure` | ClientArn | Contains the output of the CreateLunaClient action. |
| `DeleteHapgRequest` | `structure` | HapgArn | Contains the inputs for the DeleteHapg action. |
| `DeleteHapgResponse` | `structure` | Status | Contains the output of the DeleteHapg action. |
| `DeleteHsmRequest` | `structure` | HsmArn | Contains the inputs for the DeleteHsm operation. |
| `DeleteHsmResponse` | `structure` | Status | Contains the output of the DeleteHsm operation. |
| `DeleteLunaClientRequest` | `structure` | ClientArn | - |
| `DeleteLunaClientResponse` | `structure` | Status | - |
| `DescribeHapgRequest` | `structure` | HapgArn | Contains the inputs for the DescribeHapg action. |
| `DescribeHapgResponse` | `structure` | HapgArn, HapgSerial, HsmsLastActionFailed, HsmsPendingDeletion, HsmsPendingRegistration, Label, LastModifiedTimestamp, PartitionSerialList, State | Contains the output of the DescribeHapg action. |
| `DescribeHsmRequest` | `structure` | HsmArn, HsmSerialNumber | Contains the inputs for the DescribeHsm operation. |
| `DescribeHsmResponse` | `structure` | HsmArn, Status, StatusDetails, AvailabilityZone, EniId, EniIp, SubscriptionType, SubscriptionStartDate, SubscriptionEndDate, VpcId, SubnetId, IamRoleArn, ... (+9) | Contains the output of the DescribeHsm operation. |
| `DescribeLunaClientRequest` | `structure` | ClientArn, CertificateFingerprint | - |
| `DescribeLunaClientResponse` | `structure` | ClientArn, Certificate, CertificateFingerprint, LastModifiedTimestamp, Label | - |
| `GetConfigRequest` | `structure` | ClientArn, ClientVersion, HapgList | - |
| `GetConfigResponse` | `structure` | ConfigType, ConfigFile, ConfigCred | - |
| `ListAvailableZonesRequest` | `structure` | **empty (no members)** | Contains the inputs for the ListAvailableZones action. |
| `ListAvailableZonesResponse` | `structure` | AZList | - |
| `ListHapgsRequest` | `structure` | NextToken | - |
| `ListHapgsResponse` | `structure` | HapgList, NextToken | - |
| `ListHsmsRequest` | `structure` | NextToken | - |
| `ListHsmsResponse` | `structure` | HsmList, NextToken | Contains the output of the ListHsms operation. |
| `ListLunaClientsRequest` | `structure` | NextToken | - |
| `ListLunaClientsResponse` | `structure` | ClientList, NextToken | - |
| `ListTagsForResourceRequest` | `structure` | ResourceArn | - |
| `ListTagsForResourceResponse` | `structure` | TagList | - |
| `ModifyHapgRequest` | `structure` | HapgArn, Label, PartitionSerialList | - |
| `ModifyHapgResponse` | `structure` | HapgArn | - |
| `ModifyHsmRequest` | `structure` | HsmArn, SubnetId, EniIp, IamRoleArn, ExternalId, SyslogIp | Contains the inputs for the ModifyHsm operation. |
| `ModifyHsmResponse` | `structure` | HsmArn | Contains the output of the ModifyHsm operation. |
| `ModifyLunaClientRequest` | `structure` | ClientArn, Certificate | - |
| `ClientVersion` | `enum` | FIVE_ONE, FIVE_THREE | - |
| `CloudHsmObjectState` | `enum` | READY, UPDATING, DEGRADED | - |
| `HsmStatus` | `enum` | PENDING, RUNNING, UPDATING, SUSPENDED, TERMINATING, TERMINATED, DEGRADED | - |
| `SubscriptionType` | `enum` | PRODUCTION | Specifies the type of subscription for the HSM. PRODUCTION - The HSM is being used in a production environment. TRIAL - The HSM is being used in a product t ... |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
