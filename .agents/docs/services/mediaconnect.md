# AWS MediaConnect

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Welcome to the Elemental MediaConnect API reference. MediaConnect is a service that lets you ingest live video content into the cloud and distribute it to destinations all over the world, both inside and outside the Amazon Web Services cloud. This API reference provides descriptions, syntax, and usage examples for each of the actions and data types that are supported by MediaConnect. Use the following links to get started with the MediaConnect API: Actions: An alphabetical list of all MediaConnect API operations. Data types: An alphabetical list of all MediaConnect data types.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for AWS MediaConnect where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: cover association replacement for AWS MediaConnect by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- Scenario insight from EC2: add full state-machine walks for AWS MediaConnect resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented AWS MediaConnect workflows in the local mock. Key resources include `BridgeResource`, `EntitlementResource`, `FlowMediaStreamResource`, `FlowOutputResource`, `FlowResource`.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Update`, `List`, `Describe`, `Add`, `Create` operation families, including `UpdateBridge`, `UpdateBridgeOutput`, `UpdateBridgeSource`, `UpdateBridgeState`, `ListBridges`, `ListEntitlements`.

## Service Identity and Protocol

- AWS model slug: `mediaconnect`
- AWS SDK for Rust slug: `mediaconnect`
- Model version: `2018-11-14`
- Model file: `vendor/api-models-aws/models/mediaconnect/service/2018-11-14/mediaconnect-2018-11-14.json`
- SDK ID: `MediaConnect`
- Endpoint prefix: `-`
- ARN namespace: `mediaconnect`
- CloudFormation name: `MediaConnect`
- CloudTrail event source: `-`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Update` (13), `List` (12), `Describe` (8), `Add` (6), `Create` (6), `Delete` (6), `Remove` (6), `Get` (5).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AddBridgeOutputs`, `AddBridgeSources`, `AddFlowMediaStreams`, `AddFlowOutputs`, `AddFlowSources`, `AddFlowVpcInterfaces`, `BatchGetRouterInput`, `BatchGetRouterNetworkInterface`, `BatchGetRouterOutput`, `CreateBridge`, `CreateFlow`, `CreateGateway`, `CreateRouterInput`, `CreateRouterNetworkInterface`, `CreateRouterOutput`, `DeleteBridge`, `DeleteFlow`, `DeleteGateway`, `DeleteRouterInput`, `DeleteRouterNetworkInterface`, ... (+32).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `BatchGetRouterInput`, `BatchGetRouterNetworkInterface`, `BatchGetRouterOutput`, `DescribeBridge`, `DescribeFlow`, `DescribeFlowSourceMetadata`, `DescribeFlowSourceThumbnail`, `DescribeGateway`, `DescribeGatewayInstance`, `DescribeOffering`, `DescribeReservation`, `GetRouterInput`, `GetRouterInputSourceMetadata`, `GetRouterInputThumbnail`, `GetRouterNetworkInterface`, `GetRouterOutput`, `ListBridges`, `ListEntitlements`, `ListFlows`, `ListGatewayInstances`, ... (+8).
- Pagination is modelled for 10 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 33 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `StartFlow`, `StartRouterInput`, `StartRouterOutput`, `StopFlow`, `StopRouterInput`, `StopRouterOutput`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 82 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `KMS`, `CloudWatch`, `EC2/VPC`, `Secrets Manager`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `BridgeResource` | `BridgeArn` | create: `CreateBridge`; read: `DescribeBridge`; update: `UpdateBridge`; delete: `DeleteBridge`; list: `ListBridges` | `AddBridgeOutputs`, `AddBridgeSources`, `RemoveBridgeOutput`, `RemoveBridgeSource`, `UpdateBridgeOutput`, `UpdateBridgeSource`, `UpdateBridgeState` | Represents a connection that is between your datacenter's Instances and the AWS cloud |
| `EntitlementResource` | - | - | - | Represents a permission that enables the receiving AWS account (the subscriber) to create flows using the granting originator's flow as a source |
| `FlowMediaStreamResource` | - | - | - | Represents a single track or stream of media containing video, audio, or ancillary data that is transported using the SMPTE 2110 JPEG XS or CDI protocol |
| `FlowOutputResource` | - | - | - | Represents a destination address, protocol, and port that AWS Elemental MediaConnect sends video to |
| `FlowResource` | `FlowArn` | create: `CreateFlow`; read: `DescribeFlow`; update: `UpdateFlow`; delete: `DeleteFlow`; list: `ListFlows` | `AddFlowMediaStreams`, `AddFlowOutputs`, `AddFlowSources`, `AddFlowVpcInterfaces`, `DescribeFlowSourceMetadata`, `DescribeFlowSourceThumbnail`, `GrantFlowEntitlements`, `RemoveFlowMediaStream`, `RemoveFlowOutput`, `RemoveFlowSource`, ... (+8) | Represents a connection between a video source and one or more outputs. |
| `FlowSourceResource` | - | - | - | Represents a protocol, ingest port, and any encryption information that AWS Elemental MediaConnect receives video from |
| `FlowVpcInterfaceResource` | - | - | - | Represents a connection between a MediaConnect flow and a Virtual Private Cloud (VPC) that enables private network communication without traversing the public internet |
| `GatewayInstanceResource` | `GatewayInstanceArn` | read: `DescribeGatewayInstance`; update: `UpdateGatewayInstance`; delete: `DeregisterGatewayInstance`; list: `ListGatewayInstances` | - | Represents a compute instance that is running on equipment in your datacenter and managed by MediaConnect Gateway |
| `GatewayResource` | `GatewayArn` | create: `CreateGateway`; read: `DescribeGateway`; delete: `DeleteGateway`; list: `ListGateways` | - | Represents a logical grouping of Instances that is designed to replicate the functionality of an AWS Availability Zone on your local network |
| `OfferingResource` | `OfferingArn` | read: `DescribeOffering`; list: `ListOfferings` | - | Represents a set of reservation terms that provide discounted pricing for AWS Elemental MediaConnect Flows |
| `ReservationResource` | `ReservationArn` | create: `PurchaseOffering`; read: `DescribeReservation`; list: `ListReservations` | - | Represents a purchased AWS Elemental MediaConnect offering that provides discounted pricing for a usage commitment |
| `RouterInputResource` | `Arn` | create: `CreateRouterInput`; read: `GetRouterInput`; update: `UpdateRouterInput`; delete: `DeleteRouterInput`; list: `ListRouterInputs` | `BatchGetRouterInput`, `GetRouterInputSourceMetadata`, `GetRouterInputThumbnail`, `RestartRouterInput`, `StartRouterInput`, `StopRouterInput` | Represents a router input in AWS Elemental MediaConnect that is used to ingest content to be transmitted to router outputs |
| `RouterNetworkInterfaceResource` | `Arn` | create: `CreateRouterNetworkInterface`; read: `GetRouterNetworkInterface`; update: `UpdateRouterNetworkInterface`; delete: `DeleteRouterNetworkInterface`; list: `ListRouterNetworkInterfaces` | `BatchGetRouterNetworkInterface` | Represents a router network interface in AWS Elemental MediaConnect that is used to define a network boundary for router resources |
| `RouterOutputResource` | `Arn` | create: `CreateRouterOutput`; read: `GetRouterOutput`; update: `UpdateRouterOutput`; delete: `DeleteRouterOutput`; list: `ListRouterOutputs` | `BatchGetRouterOutput`, `RestartRouterOutput`, `StartRouterOutput`, `StopRouterOutput`, `TakeRouterInput` | Represents a router input in AWS Elemental MediaConnect that can be used to egress content transmitted from router inputs |

## Current Network Resource Stub Semantics

MediaConnect currently stores flow VPC interfaces inside MediaConnect flow state.

- VPC interface records keep name, role ARN, subnet ID, security group IDs, network interface type, and locally generated network interface IDs.
- `AddFlowVpcInterfaces` appends these records to the flow; `RemoveFlowVpcInterface` removes by name and returns an empty non-deleted network interface list.
- The generated network interface IDs are not inserted into EC2 state, and the role ARN is not checked against IAM.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Operation Groups

### List

- Operations: `ListEntitlements`, `ListTagsForGlobalResource`, `ListTagsForResource`
- Traits: `readonly` (3), `paginated` (1)
- Common required input members in this group: `ResourceArn`

### Tag

- Operations: `TagGlobalResource`, `TagResource`
- Common required input members in this group: `ResourceArn`, `Tags`

### Untag

- Operations: `UntagGlobalResource`, `UntagResource`
- Traits: `idempotent` (2)
- Common required input members in this group: `ResourceArn`, `TagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `ListEntitlements` | `GET /v1/entitlements` | `readonly`, `paginated` | - | - | `ListEntitlementsResponse` | `BadRequestException`, `InternalServerErrorException`, `ServiceUnavailableException`, `TooManyRequestsException` | Displays a list of all entitlements that have been granted to this account. This request returns 20 results per page. |
| `ListTagsForGlobalResource` | `GET /tags/global/{ResourceArn}` | `readonly` | `ResourceArn` | - | `ListTagsForGlobalResourceResponse` | `BadRequestException`, `InternalServerErrorException`, `NotFoundException` | Lists the tags associated with a global resource in AWS Elemental MediaConnect. The API supports the following global resources: router inputs, router outputs and router network interfaces. |
| `ListTagsForResource` | `GET /tags/{ResourceArn}` | `readonly` | `ResourceArn` | - | `ListTagsForResourceResponse` | `BadRequestException`, `InternalServerErrorException`, `NotFoundException` | List all tags on a MediaConnect resource in the current region. |
| `TagGlobalResource` | `POST /tags/global/{ResourceArn}` | - | `ResourceArn`, `Tags` | - | `Unit` | `BadRequestException`, `InternalServerErrorException`, `NotFoundException` | Adds tags to a global resource in AWS Elemental MediaConnect. The API supports the following global resources: router inputs, router outputs and router network interfaces. |
| `TagResource` | `POST /tags/{ResourceArn}` | - | `ResourceArn`, `Tags` | - | `Unit` | `BadRequestException`, `InternalServerErrorException`, `NotFoundException` | Associates the specified tags to a resource with the specified resourceArn in the current region. If existing tags on a resource are not specified in the request parameters, they are not changed. When a resource is d ... |
| `UntagGlobalResource` | `DELETE /tags/global/{ResourceArn}` | `idempotent` | `ResourceArn`, `TagKeys` | - | `Unit` | `BadRequestException`, `InternalServerErrorException`, `NotFoundException` | Removes tags from a global resource in AWS Elemental MediaConnect. The API supports the following global resources: router inputs, router outputs and router network interfaces. |
| `UntagResource` | `DELETE /tags/{ResourceArn}` | `idempotent` | `ResourceArn`, `TagKeys` | - | `Unit` | `BadRequestException`, `InternalServerErrorException`, `NotFoundException` | Deletes specified tags from a resource in the current region. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `ListEntitlements` | - | `MaxResults -> maxResults`, `NextToken -> nextToken` | - | - |
| `UntagGlobalResource` | - | `TagKeys -> tagKeys` | - | - |
| `UntagResource` | - | `TagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AddFlowOutputs420Exception` | `structure` | Message | Exception raised by Elemental MediaConnect when adding the flow output. See the error message for the operation for more information on the cause of this ex ... |
| `BadRequestException` | `structure` | Message | This exception is thrown if the request contains a semantic error. The precise meaning depends on the API, and is documented in the error message. |
| `ConflictException` | `structure` | Message | The requested operation would cause a conflict with the current state of a service resource associated with the request. Resolve the conflict before retryin ... |
| `CreateBridge420Exception` | `structure` | Message | Exception raised by Elemental MediaConnect when creating the bridge. See the error message for the operation for more information on the cause of this excep ... |
| `CreateFlow420Exception` | `structure` | Message | Exception raised by Elemental MediaConnect when creating the flow. See the error message for the operation for more information on the cause of this exception. |
| `CreateGateway420Exception` | `structure` | Message | Exception raised by Elemental MediaConnect when creating the gateway. See the error message for the operation for more information on the cause of this exce ... |
| `ForbiddenException` | `structure` | Message | You do not have sufficient access to perform this action. |
| `GrantFlowEntitlements420Exception` | `structure` | Message | Exception raised by Elemental MediaConnect when granting the entitlement. See the error message for the operation for more information on the cause of this ... |
| `InternalServerErrorException` | `structure` | Message | The server encountered an internal error and is unable to complete the request. |
| `NotFoundException` | `structure` | Message | One or more of the resources in the request does not exist in the system. |
| `RouterInputServiceQuotaExceededException` | `structure` | Message | The request to create a new router input would exceed the service quotas for the account. |
| `RouterNetworkInterfaceServiceQuotaExceededException` | `structure` | Message | The request to create a new router network interface would exceed the service quotas (limits) set for the account. |
| `RouterOutputServiceQuotaExceededException` | `structure` | Message | The request to create a new router output would exceed the service quotas (limits) set for the account. |
| `ServiceUnavailableException` | `structure` | Message | The service is currently unavailable or busy. |
| `TooManyRequestsException` | `structure` | Message | The request was denied due to request throttling. |
| `ListEntitlementsRequest` | `structure` | MaxResults, NextToken | - |
| `ListEntitlementsResponse` | `structure` | Entitlements, NextToken | - |
| `ListTagsForGlobalResourceRequest` | `structure` | ResourceArn | - |
| `ListTagsForGlobalResourceResponse` | `structure` | Tags | - |
| `ListTagsForResourceRequest` | `structure` | ResourceArn | - |
| `ListTagsForResourceResponse` | `structure` | Tags | - |
| `TagGlobalResourceRequest` | `structure` | ResourceArn, Tags | - |
| `TagResourceRequest` | `structure` | ResourceArn, Tags | - |
| `UntagGlobalResourceRequest` | `structure` | ResourceArn, TagKeys | - |
| `UntagResourceRequest` | `structure` | ResourceArn, TagKeys | - |
| `Algorithm` | `enum` | aes128, aes192, aes256 | - |
| `BridgePlacement` | `enum` | AVAILABLE, LOCKED | - |
| `BridgeState` | `enum` | CREATING, STANDBY, STARTING, DEPLOYING, ACTIVE, STOPPING, DELETING, DELETED, START_FAILED, START_PENDING, STOP_FAILED, UPDATING | - |
| `Colorimetry` | `enum` | BT601, BT709, BT2020, BT2100, ST2065_1, ST2065_3, XYZ | - |
| `ConnectionStatus` | `enum` | CONNECTED, DISCONNECTED | - |
| `ContentQualityAnalysisState` | `enum` | ENABLED, DISABLED | - |
| `Day` | `enum` | MONDAY, TUESDAY, WEDNESDAY, THURSDAY, FRIDAY, SATURDAY, SUNDAY | - |
| `DesiredState` | `enum` | ACTIVE, STANDBY, DELETED | - |
| `DurationUnits` | `enum` | MONTHS | - |
| `EncoderProfile` | `enum` | main, high | - |
| `EncodingName` | `enum` | jxsv, raw, smpte291, pcm | - |
| `EncodingProfile` | `enum` | DISTRIBUTION_H264_DEFAULT, CONTRIBUTION_H264_DEFAULT | - |
| `EntitlementStatus` | `enum` | ENABLED, DISABLED | - |
| `FailoverInputSourcePriorityMode` | `enum` | NO_PRIORITY, PRIMARY_SECONDARY | - |
| `FailoverMode` | `enum` | MERGE, FAILOVER | - |
| `FlowSize` | `enum` | MEDIUM, LARGE, LARGE_4X | - |
| `FlowTransitEncryptionKeyType` | `enum` | SECRETS_MANAGER, AUTOMATIC | - |
| `ForwardErrorCorrectionState` | `enum` | ENABLED, DISABLED | - |
| `GatewayState` | `enum` | CREATING, ACTIVE, UPDATING, ERROR, DELETING, DELETED | - |
| `InstanceState` | `enum` | REGISTERING, ACTIVE, DEREGISTERING, DEREGISTERED, REGISTRATION_ERROR, DEREGISTRATION_ERROR | - |
| `KeyType` | `enum` | speke, static_key, srt_password | - |
| `MaintenanceDay` | `enum` | Monday, Tuesday, Wednesday, Thursday, Friday, Saturday, Sunday | - |
| `MaintenanceScheduleType` | `enum` | WINDOW | - |
| `MaintenanceType` | `enum` | PREFERRED_DAY_TIME, DEFAULT | - |
| `MediaLiveChannelPipelineId` | `enum` | PIPELINE_0, PIPELINE_1 | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
