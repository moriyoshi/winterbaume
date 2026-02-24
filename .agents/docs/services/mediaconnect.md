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

### Update

- Operations: `UpdateBridge`, `UpdateBridgeOutput`, `UpdateBridgeSource`, `UpdateBridgeState`, `UpdateFlow`, `UpdateFlowEntitlement`, `UpdateFlowMediaStream`, `UpdateFlowOutput`, `UpdateFlowSource`, `UpdateGatewayInstance`, `UpdateRouterInput`, `UpdateRouterNetworkInterface`, `UpdateRouterOutput`
- Traits: `idempotent` (13)
- Common required input members in this group: `Arn`, `BridgeArn`, `DesiredState`, `EntitlementArn`, `FlowArn`, `GatewayInstanceArn`, `MediaStreamName`, `OutputArn`, `OutputName`, `SourceArn`, `SourceName`

### List

- Operations: `ListBridges`, `ListEntitlements`, `ListFlows`, `ListGatewayInstances`, `ListGateways`, `ListOfferings`, `ListReservations`, `ListRouterInputs`, `ListRouterNetworkInterfaces`, `ListRouterOutputs`, `ListTagsForGlobalResource`, `ListTagsForResource`
- Traits: `paginated` (10), `readonly` (12)
- Common required input members in this group: `ResourceArn`

### Describe

- Operations: `DescribeBridge`, `DescribeFlow`, `DescribeFlowSourceMetadata`, `DescribeFlowSourceThumbnail`, `DescribeGateway`, `DescribeGatewayInstance`, `DescribeOffering`, `DescribeReservation`
- Traits: `readonly` (8)
- Common required input members in this group: `BridgeArn`, `FlowArn`, `GatewayArn`, `GatewayInstanceArn`, `OfferingArn`, `ReservationArn`

### Add

- Operations: `AddBridgeOutputs`, `AddBridgeSources`, `AddFlowMediaStreams`, `AddFlowOutputs`, `AddFlowSources`, `AddFlowVpcInterfaces`
- Common required input members in this group: `BridgeArn`, `FlowArn`, `MediaStreams`, `Outputs`, `Sources`, `VpcInterfaces`

### Create

- Operations: `CreateBridge`, `CreateFlow`, `CreateGateway`, `CreateRouterInput`, `CreateRouterNetworkInterface`, `CreateRouterOutput`
- Traits: `idempotency-token` (3)
- Common required input members in this group: `Configuration`, `EgressCidrBlocks`, `MaximumBitrate`, `Name`, `Networks`, `PlacementArn`, `RoutingScope`, `Sources`, `Tier`

### Delete

- Operations: `DeleteBridge`, `DeleteFlow`, `DeleteGateway`, `DeleteRouterInput`, `DeleteRouterNetworkInterface`, `DeleteRouterOutput`
- Traits: `idempotent` (6)
- Common required input members in this group: `Arn`, `BridgeArn`, `FlowArn`, `GatewayArn`

### Remove

- Operations: `RemoveBridgeOutput`, `RemoveBridgeSource`, `RemoveFlowMediaStream`, `RemoveFlowOutput`, `RemoveFlowSource`, `RemoveFlowVpcInterface`
- Traits: `idempotent` (6)
- Common required input members in this group: `BridgeArn`, `FlowArn`, `MediaStreamName`, `OutputArn`, `OutputName`, `SourceArn`, `SourceName`, `VpcInterfaceName`

### Get

- Operations: `GetRouterInput`, `GetRouterInputSourceMetadata`, `GetRouterInputThumbnail`, `GetRouterNetworkInterface`, `GetRouterOutput`
- Traits: `readonly` (5)
- Common required input members in this group: `Arn`

### Batch

- Operations: `BatchGetRouterInput`, `BatchGetRouterNetworkInterface`, `BatchGetRouterOutput`
- Traits: `readonly` (3)
- Common required input members in this group: `Arns`

### Start

- Operations: `StartFlow`, `StartRouterInput`, `StartRouterOutput`
- Common required input members in this group: `Arn`, `FlowArn`

### Stop

- Operations: `StopFlow`, `StopRouterInput`, `StopRouterOutput`
- Common required input members in this group: `Arn`, `FlowArn`

### Restart

- Operations: `RestartRouterInput`, `RestartRouterOutput`
- Common required input members in this group: `Arn`

### Tag

- Operations: `TagGlobalResource`, `TagResource`
- Common required input members in this group: `ResourceArn`, `Tags`

### Untag

- Operations: `UntagGlobalResource`, `UntagResource`
- Traits: `idempotent` (2)
- Common required input members in this group: `ResourceArn`, `TagKeys`

### Deregister

- Operations: `DeregisterGatewayInstance`
- Traits: `idempotent` (1)
- Common required input members in this group: `GatewayInstanceArn`

### Grant

- Operations: `GrantFlowEntitlements`
- Common required input members in this group: `Entitlements`, `FlowArn`

### Purchase

- Operations: `PurchaseOffering`
- Common required input members in this group: `OfferingArn`, `ReservationName`, `Start`

### Revoke

- Operations: `RevokeFlowEntitlement`
- Traits: `idempotent` (1)
- Common required input members in this group: `EntitlementArn`, `FlowArn`

### Take

- Operations: `TakeRouterInput`
- Traits: `idempotent` (1)
- Common required input members in this group: `RouterOutputArn`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AddBridgeOutputs` | `POST /v1/bridges/{BridgeArn}/outputs` | - | `BridgeArn`, `Outputs` | - | `AddBridgeOutputsResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException` | Adds outputs to an existing bridge. |
| `AddBridgeSources` | `POST /v1/bridges/{BridgeArn}/sources` | - | `BridgeArn`, `Sources` | - | `AddBridgeSourcesResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException` | Adds sources to an existing bridge. |
| `AddFlowMediaStreams` | `POST /v1/flows/{FlowArn}/mediaStreams` | - | `FlowArn`, `MediaStreams` | - | `AddFlowMediaStreamsResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException` | Adds media streams to an existing flow. After you add a media stream to a flow, you can associate it with a source and/or an output that uses the ST 2110 JPEG XS or CDI protocol. |
| `AddFlowOutputs` | `POST /v1/flows/{FlowArn}/outputs` | - | `FlowArn`, `Outputs` | - | `AddFlowOutputsResponse` | `AddFlowOutputs420Exception`, `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException` | Adds outputs to an existing flow. You can create up to 50 outputs per flow. |
| `AddFlowSources` | `POST /v1/flows/{FlowArn}/source` | - | `FlowArn`, `Sources` | - | `AddFlowSourcesResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException` | Adds sources to a flow. |
| `AddFlowVpcInterfaces` | `POST /v1/flows/{FlowArn}/vpcInterfaces` | - | `FlowArn`, `VpcInterfaces` | - | `AddFlowVpcInterfacesResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException` | Adds VPC interfaces to a flow. |
| `BatchGetRouterInput` | `GET /v1/routerInputs` | `readonly` | `Arns` | - | `BatchGetRouterInputResponse` | `BadRequestException`, `ConflictException`, `InternalServerErrorException`, `ServiceUnavailableException`, `TooManyRequestsException` | Retrieves information about multiple router inputs in AWS Elemental MediaConnect. |
| `BatchGetRouterNetworkInterface` | `GET /v1/routerNetworkInterfaces` | `readonly` | `Arns` | - | `BatchGetRouterNetworkInterfaceResponse` | `BadRequestException`, `ConflictException`, `InternalServerErrorException`, `ServiceUnavailableException`, `TooManyRequestsException` | Retrieves information about multiple router network interfaces in AWS Elemental MediaConnect. |
| `BatchGetRouterOutput` | `GET /v1/routerOutputs` | `readonly` | `Arns` | - | `BatchGetRouterOutputResponse` | `BadRequestException`, `ConflictException`, `InternalServerErrorException`, `ServiceUnavailableException`, `TooManyRequestsException` | Retrieves information about multiple router outputs in AWS Elemental MediaConnect. |
| `CreateBridge` | `POST /v1/bridges` | - | `Name`, `PlacementArn`, `Sources` | - | `CreateBridgeResponse` | `BadRequestException`, `ConflictException`, `CreateBridge420Exception`, `ForbiddenException`, `InternalServerErrorException`, `ServiceUnavailableException`, `TooManyRequestsException` | Creates a new bridge. The request must include one source. |
| `CreateFlow` | `POST /v1/flows` | - | `Name` | - | `CreateFlowResponse` | `BadRequestException`, `CreateFlow420Exception`, `ForbiddenException`, `InternalServerErrorException`, `ServiceUnavailableException`, `TooManyRequestsException` | Creates a new flow. The request must include one source. |
| `CreateGateway` | `POST /v1/gateways` | - | `EgressCidrBlocks`, `Name`, `Networks` | - | `CreateGatewayResponse` | `BadRequestException`, `ConflictException`, `CreateGateway420Exception`, `ForbiddenException`, `InternalServerErrorException`, `ServiceUnavailableException`, `TooManyRequestsException` | Creates a new gateway. The request must include at least one network (up to four). |
| `CreateRouterInput` | `POST /v1/routerInput` | `idempotency-token` | `Configuration`, `MaximumBitrate`, `Name`, `RoutingScope`, `Tier` | `ClientToken` | `CreateRouterInputResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `RouterInputServiceQuotaExceededException`, `ServiceUnavailableException`, `TooManyRequestsException` | Creates a new router input in AWS Elemental MediaConnect. |
| `CreateRouterNetworkInterface` | `POST /v1/routerNetworkInterface` | `idempotency-token` | `Configuration`, `Name` | `ClientToken` | `CreateRouterNetworkInterfaceResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `RouterNetworkInterfaceServiceQuotaExceededException`, `ServiceUnavailableException`, `TooManyRequestsException` | Creates a new router network interface in AWS Elemental MediaConnect. |
| `CreateRouterOutput` | `POST /v1/routerOutput` | `idempotency-token` | `Configuration`, `MaximumBitrate`, `Name`, `RoutingScope`, `Tier` | `ClientToken` | `CreateRouterOutputResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `RouterOutputServiceQuotaExceededException`, `ServiceUnavailableException`, `TooManyRequestsException` | Creates a new router output in AWS Elemental MediaConnect. |
| `DeleteBridge` | `DELETE /v1/bridges/{BridgeArn}` | `idempotent` | `BridgeArn` | - | `DeleteBridgeResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException` | Deletes a bridge. Before you can delete a bridge, you must stop the bridge. |
| `DeleteFlow` | `DELETE /v1/flows/{FlowArn}` | `idempotent` | `FlowArn` | - | `DeleteFlowResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException` | Deletes a flow. Before you can delete a flow, you must stop the flow. |
| `DeleteGateway` | `DELETE /v1/gateways/{GatewayArn}` | `idempotent` | `GatewayArn` | - | `DeleteGatewayResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException` | Deletes a gateway. Before you can delete a gateway, you must deregister its instances and delete its bridges. |
| `DeleteRouterInput` | `DELETE /v1/routerInput/{Arn}` | `idempotent` | `Arn` | - | `DeleteRouterInputResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException` | Deletes a router input from AWS Elemental MediaConnect. |
| `DeleteRouterNetworkInterface` | `DELETE /v1/routerNetworkInterface/{Arn}` | `idempotent` | `Arn` | - | `DeleteRouterNetworkInterfaceResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException` | Deletes a router network interface from AWS Elemental MediaConnect. |
| `DeleteRouterOutput` | `DELETE /v1/routerOutput/{Arn}` | `idempotent` | `Arn` | - | `DeleteRouterOutputResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException` | Deletes a router output from AWS Elemental MediaConnect. |
| `DeregisterGatewayInstance` | `DELETE /v1/gateway-instances/{GatewayInstanceArn}` | `idempotent` | `GatewayInstanceArn` | - | `DeregisterGatewayInstanceResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException` | Deregisters an instance. Before you deregister an instance, all bridges running on the instance must be stopped. |
| `DescribeBridge` | `GET /v1/bridges/{BridgeArn}` | `readonly` | `BridgeArn` | - | `DescribeBridgeResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException` | Displays the details of a bridge. |
| `DescribeFlow` | `GET /v1/flows/{FlowArn}` | `readonly` | `FlowArn` | - | `DescribeFlowResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException` | Displays the details of a flow. The response includes the flow Amazon Resource Name (ARN), name, and Availability Zone, as well as details about the source, outputs, and entitlements. |
| `DescribeFlowSourceMetadata` | `GET /v1/flows/{FlowArn}/source-metadata` | `readonly` | `FlowArn` | - | `DescribeFlowSourceMetadataResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException` | The `DescribeFlowSourceMetadata` API is used to view information about the flow's source transport stream and programs. This API displays status messages about the flow's source as well as details about the program's video, audio, and other data. |
| `DescribeFlowSourceThumbnail` | `GET /v1/flows/{FlowArn}/source-thumbnail` | `readonly` | `FlowArn` | - | `DescribeFlowSourceThumbnailResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException` | Describes the thumbnail for the flow source. |
| `DescribeGateway` | `GET /v1/gateways/{GatewayArn}` | `readonly` | `GatewayArn` | - | `DescribeGatewayResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException` | Displays the details of a gateway. The response includes the gateway Amazon Resource Name (ARN), name, and CIDR blocks, as well as details about the networks. |
| `DescribeGatewayInstance` | `GET /v1/gateway-instances/{GatewayInstanceArn}` | `readonly` | `GatewayInstanceArn` | - | `DescribeGatewayInstanceResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException` | Displays the details of an instance. |
| `DescribeOffering` | `GET /v1/offerings/{OfferingArn}` | `readonly` | `OfferingArn` | - | `DescribeOfferingResponse` | `BadRequestException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException` | Displays the details of an offering. The response includes the offering description, duration, outbound bandwidth, price, and Amazon Resource Name (ARN). |
| `DescribeReservation` | `GET /v1/reservations/{ReservationArn}` | `readonly` | `ReservationArn` | - | `DescribeReservationResponse` | `BadRequestException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException` | Displays the details of a reservation. The response includes the reservation name, state, start date and time, and the details of the offering that make up the rest of the reservation (such as price, duration, and outbound bandwidth). |
| `GetRouterInput` | `GET /v1/routerInput/{Arn}` | `readonly` | `Arn` | - | `GetRouterInputResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException` | Retrieves information about a specific router input in AWS Elemental MediaConnect. |
| `GetRouterInputSourceMetadata` | `GET /v1/routerInput/{Arn}/source-metadata` | `readonly` | `Arn` | - | `GetRouterInputSourceMetadataResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException` | Retrieves detailed metadata information about a specific router input source, including stream details and connection state. |
| `GetRouterInputThumbnail` | `GET /v1/routerInput/{Arn}/thumbnail` | `readonly` | `Arn` | - | `GetRouterInputThumbnailResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException` | Retrieves the thumbnail for a router input in AWS Elemental MediaConnect. |
| `GetRouterNetworkInterface` | `GET /v1/routerNetworkInterface/{Arn}` | `readonly` | `Arn` | - | `GetRouterNetworkInterfaceResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException` | Retrieves information about a specific router network interface in AWS Elemental MediaConnect. |
| `GetRouterOutput` | `GET /v1/routerOutput/{Arn}` | `readonly` | `Arn` | - | `GetRouterOutputResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException` | Retrieves information about a specific router output in AWS Elemental MediaConnect. |
| `GrantFlowEntitlements` | `POST /v1/flows/{FlowArn}/entitlements` | - | `Entitlements`, `FlowArn` | - | `GrantFlowEntitlementsResponse` | `BadRequestException`, `ForbiddenException`, `GrantFlowEntitlements420Exception`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException` | Grants entitlements to an existing flow. |
| `ListBridges` | `GET /v1/bridges` | `readonly`, `paginated` | - | - | `ListBridgesResponse` | `BadRequestException`, `ConflictException`, `InternalServerErrorException`, `ServiceUnavailableException`, `TooManyRequestsException` | Displays a list of bridges that are associated with this account and an optionally specified Amazon Resource Name (ARN). This request returns a paginated result. |
| `ListEntitlements` | `GET /v1/entitlements` | `readonly`, `paginated` | - | - | `ListEntitlementsResponse` | `BadRequestException`, `InternalServerErrorException`, `ServiceUnavailableException`, `TooManyRequestsException` | Displays a list of all entitlements that have been granted to this account. This request returns 20 results per page. |
| `ListFlows` | `GET /v1/flows` | `readonly`, `paginated` | - | - | `ListFlowsResponse` | `BadRequestException`, `InternalServerErrorException`, `ServiceUnavailableException`, `TooManyRequestsException` | Displays a list of flows that are associated with this account. This request returns a paginated result. |
| `ListGatewayInstances` | `GET /v1/gateway-instances` | `readonly`, `paginated` | - | - | `ListGatewayInstancesResponse` | `BadRequestException`, `ConflictException`, `InternalServerErrorException`, `ServiceUnavailableException`, `TooManyRequestsException` | Displays a list of instances associated with the Amazon Web Services account. This request returns a paginated result. |
| `ListGateways` | `GET /v1/gateways` | `readonly`, `paginated` | - | - | `ListGatewaysResponse` | `BadRequestException`, `ConflictException`, `InternalServerErrorException`, `ServiceUnavailableException`, `TooManyRequestsException` | Displays a list of gateways that are associated with this account. This request returns a paginated result. |
| `ListOfferings` | `GET /v1/offerings` | `readonly`, `paginated` | - | - | `ListOfferingsResponse` | `BadRequestException`, `InternalServerErrorException`, `ServiceUnavailableException`, `TooManyRequestsException` | Displays a list of all offerings that are available to this account in the current Amazon Web Services Region. If you have an active reservation (which means you've purchased an offering that has already started and hasn't expired yet), your account isn't... |
| `ListReservations` | `GET /v1/reservations` | `readonly`, `paginated` | - | - | `ListReservationsResponse` | `BadRequestException`, `InternalServerErrorException`, `ServiceUnavailableException`, `TooManyRequestsException` | Displays a list of all reservations that have been purchased by this account in the current Amazon Web Services Region. This list includes all reservations in all states (such as active and expired). |
| `ListRouterInputs` | `POST /v1/routerInputs` | `readonly`, `paginated` | - | - | `ListRouterInputsResponse` | `BadRequestException`, `ConflictException`, `InternalServerErrorException`, `ServiceUnavailableException`, `TooManyRequestsException` | Retrieves a list of router inputs in AWS Elemental MediaConnect. |
| `ListRouterNetworkInterfaces` | `POST /v1/routerNetworkInterfaces` | `readonly`, `paginated` | - | - | `ListRouterNetworkInterfacesResponse` | `BadRequestException`, `ConflictException`, `InternalServerErrorException`, `ServiceUnavailableException`, `TooManyRequestsException` | Retrieves a list of router network interfaces in AWS Elemental MediaConnect. |
| `ListRouterOutputs` | `POST /v1/routerOutputs` | `readonly`, `paginated` | - | - | `ListRouterOutputsResponse` | `BadRequestException`, `ConflictException`, `InternalServerErrorException`, `ServiceUnavailableException`, `TooManyRequestsException` | Retrieves a list of router outputs in AWS Elemental MediaConnect. |
| `ListTagsForGlobalResource` | `GET /tags/global/{ResourceArn}` | `readonly` | `ResourceArn` | - | `ListTagsForGlobalResourceResponse` | `BadRequestException`, `InternalServerErrorException`, `NotFoundException` | Lists the tags associated with a global resource in AWS Elemental MediaConnect. The API supports the following global resources: router inputs, router outputs and router network interfaces. |
| `ListTagsForResource` | `GET /tags/{ResourceArn}` | `readonly` | `ResourceArn` | - | `ListTagsForResourceResponse` | `BadRequestException`, `InternalServerErrorException`, `NotFoundException` | List all tags on a MediaConnect resource in the current region. |
| `PurchaseOffering` | `POST /v1/offerings/{OfferingArn}` | - | `OfferingArn`, `ReservationName`, `Start` | - | `PurchaseOfferingResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException` | Submits a request to purchase an offering. If you already have an active reservation, you can't purchase another offering. |
| `RemoveBridgeOutput` | `DELETE /v1/bridges/{BridgeArn}/outputs/{OutputName}` | `idempotent` | `BridgeArn`, `OutputName` | - | `RemoveBridgeOutputResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException` | Removes an output from a bridge. |
| `RemoveBridgeSource` | `DELETE /v1/bridges/{BridgeArn}/sources/{SourceName}` | `idempotent` | `BridgeArn`, `SourceName` | - | `RemoveBridgeSourceResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException` | Removes a source from a bridge. |
| `RemoveFlowMediaStream` | `DELETE /v1/flows/{FlowArn}/mediaStreams/{MediaStreamName}` | `idempotent` | `FlowArn`, `MediaStreamName` | - | `RemoveFlowMediaStreamResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException` | Removes a media stream from a flow. This action is only available if the media stream is not associated with a source or output. |
| `RemoveFlowOutput` | `DELETE /v1/flows/{FlowArn}/outputs/{OutputArn}` | `idempotent` | `FlowArn`, `OutputArn` | - | `RemoveFlowOutputResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException` | Removes an output from an existing flow. This request can be made only on an output that does not have an entitlement associated with it. |
| `RemoveFlowSource` | `DELETE /v1/flows/{FlowArn}/source/{SourceArn}` | `idempotent` | `FlowArn`, `SourceArn` | - | `RemoveFlowSourceResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException` | Removes a source from an existing flow. This request can be made only if there is more than one source on the flow. |
| `RemoveFlowVpcInterface` | `DELETE /v1/flows/{FlowArn}/vpcInterfaces/{VpcInterfaceName}` | `idempotent` | `FlowArn`, `VpcInterfaceName` | - | `RemoveFlowVpcInterfaceResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException` | Removes a VPC Interface from an existing flow. This request can be made only on a VPC interface that does not have a Source or Output associated with it. |
| `RestartRouterInput` | `POST /v1/routerInput/restart/{Arn}` | - | `Arn` | - | `RestartRouterInputResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException` | Restarts a router input. This operation can be used to recover from errors or refresh the input state. |
| `RestartRouterOutput` | `POST /v1/routerOutput/restart/{Arn}` | - | `Arn` | - | `RestartRouterOutputResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException` | Restarts a router output. This operation can be used to recover from errors or refresh the output state. |
| `RevokeFlowEntitlement` | `DELETE /v1/flows/{FlowArn}/entitlements/{EntitlementArn}` | `idempotent` | `EntitlementArn`, `FlowArn` | - | `RevokeFlowEntitlementResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException` | Revokes an entitlement from a flow. Once an entitlement is revoked, the content becomes unavailable to the subscriber and the associated output is removed. |
| `StartFlow` | `POST /v1/flows/start/{FlowArn}` | - | `FlowArn` | - | `StartFlowResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException` | Starts a flow. |
| `StartRouterInput` | `POST /v1/routerInput/start/{Arn}` | - | `Arn` | - | `StartRouterInputResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException` | Starts a router input in AWS Elemental MediaConnect. |
| `StartRouterOutput` | `POST /v1/routerOutput/start/{Arn}` | - | `Arn` | - | `StartRouterOutputResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException` | Starts a router output in AWS Elemental MediaConnect. |
| `StopFlow` | `POST /v1/flows/stop/{FlowArn}` | - | `FlowArn` | - | `StopFlowResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException` | Stops a flow. |
| `StopRouterInput` | `POST /v1/routerInput/stop/{Arn}` | - | `Arn` | - | `StopRouterInputResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException` | Stops a router input in AWS Elemental MediaConnect. |
| `StopRouterOutput` | `POST /v1/routerOutput/stop/{Arn}` | - | `Arn` | - | `StopRouterOutputResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException` | Stops a router output in AWS Elemental MediaConnect. |
| `TagGlobalResource` | `POST /tags/global/{ResourceArn}` | - | `ResourceArn`, `Tags` | - | `Unit` | `BadRequestException`, `InternalServerErrorException`, `NotFoundException` | Adds tags to a global resource in AWS Elemental MediaConnect. The API supports the following global resources: router inputs, router outputs and router network interfaces. |
| `TagResource` | `POST /tags/{ResourceArn}` | - | `ResourceArn`, `Tags` | - | `Unit` | `BadRequestException`, `InternalServerErrorException`, `NotFoundException` | Associates the specified tags to a resource with the specified `resourceArn` in the current region. If existing tags on a resource are not specified in the request parameters, they are not changed. |
| `TakeRouterInput` | `PUT /v1/routerOutput/takeRouterInput/{RouterOutputArn}` | `idempotent` | `RouterOutputArn` | - | `TakeRouterInputResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException` | Associates a router input with a router output in AWS Elemental MediaConnect. |
| `UntagGlobalResource` | `DELETE /tags/global/{ResourceArn}` | `idempotent` | `ResourceArn`, `TagKeys` | - | `Unit` | `BadRequestException`, `InternalServerErrorException`, `NotFoundException` | Removes tags from a global resource in AWS Elemental MediaConnect. The API supports the following global resources: router inputs, router outputs and router network interfaces. |
| `UntagResource` | `DELETE /tags/{ResourceArn}` | `idempotent` | `ResourceArn`, `TagKeys` | - | `Unit` | `BadRequestException`, `InternalServerErrorException`, `NotFoundException` | Deletes specified tags from a resource in the current region. |
| `UpdateBridge` | `PUT /v1/bridges/{BridgeArn}` | `idempotent` | `BridgeArn` | - | `UpdateBridgeResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException` | Updates the bridge. |
| `UpdateBridgeOutput` | `PUT /v1/bridges/{BridgeArn}/outputs/{OutputName}` | `idempotent` | `BridgeArn`, `OutputName` | - | `UpdateBridgeOutputResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException` | Updates an existing bridge output. |
| `UpdateBridgeSource` | `PUT /v1/bridges/{BridgeArn}/sources/{SourceName}` | `idempotent` | `BridgeArn`, `SourceName` | - | `UpdateBridgeSourceResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException` | Updates an existing bridge source. |
| `UpdateBridgeState` | `PUT /v1/bridges/{BridgeArn}/state` | `idempotent` | `BridgeArn`, `DesiredState` | - | `UpdateBridgeStateResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException` | Updates the bridge state. |
| `UpdateFlow` | `PUT /v1/flows/{FlowArn}` | `idempotent` | `FlowArn` | - | `UpdateFlowResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException` | Updates an existing flow. Because `UpdateFlowSources` and `UpdateFlow` are separate operations, you can't change both the source type AND the flow size in a single request. |
| `UpdateFlowEntitlement` | `PUT /v1/flows/{FlowArn}/entitlements/{EntitlementArn}` | `idempotent` | `EntitlementArn`, `FlowArn` | - | `UpdateFlowEntitlementResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException` | Updates an entitlement. You can change an entitlement's description, subscribers, and encryption. |
| `UpdateFlowMediaStream` | `PUT /v1/flows/{FlowArn}/mediaStreams/{MediaStreamName}` | `idempotent` | `FlowArn`, `MediaStreamName` | - | `UpdateFlowMediaStreamResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException` | Updates an existing media stream. |
| `UpdateFlowOutput` | `PUT /v1/flows/{FlowArn}/outputs/{OutputArn}` | `idempotent` | `FlowArn`, `OutputArn` | - | `UpdateFlowOutputResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException` | Updates an existing flow output. |
| `UpdateFlowSource` | `PUT /v1/flows/{FlowArn}/source/{SourceArn}` | `idempotent` | `FlowArn`, `SourceArn` | - | `UpdateFlowSourceResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException` | Updates the source of a flow. Because `UpdateFlowSources` and `UpdateFlow` are separate operations, you can't change both the source type AND the flow size in a single request. |
| `UpdateGatewayInstance` | `PUT /v1/gateway-instances/{GatewayInstanceArn}` | `idempotent` | `GatewayInstanceArn` | - | `UpdateGatewayInstanceResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException` | Updates an existing gateway instance. |
| `UpdateRouterInput` | `PUT /v1/routerInput/{Arn}` | `idempotent` | `Arn` | - | `UpdateRouterInputResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException` | Updates the configuration of an existing router input in AWS Elemental MediaConnect. |
| `UpdateRouterNetworkInterface` | `PUT /v1/routerNetworkInterface/{Arn}` | `idempotent` | `Arn` | - | `UpdateRouterNetworkInterfaceResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `ServiceUnavailableException`, `TooManyRequestsException` | Updates the configuration of an existing router network interface in AWS Elemental MediaConnect. |
| `UpdateRouterOutput` | `PUT /v1/routerOutput/{Arn}` | `idempotent` | `Arn` | - | `UpdateRouterOutputResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceUnavailableException`, `TooManyRequestsException` | Updates the configuration of an existing router output in AWS Elemental MediaConnect. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `BadRequestException` | `structure` | `Message` | This exception is thrown if the request contains a semantic error. |
| `InternalServerErrorException` | `structure` | `Message` | The server encountered an internal error and is unable to complete the request. |
| `ServiceUnavailableException` | `structure` | `Message` | The service is currently unavailable or busy. |
| `TooManyRequestsException` | `structure` | `Message` | The request was denied due to request throttling. |
| `NotFoundException` | `structure` | `Message` | One or more of the resources in the request does not exist in the system. |
| `ForbiddenException` | `structure` | `Message` | You do not have sufficient access to perform this action. |
| `ConflictException` | `structure` | `Message` | The requested operation would cause a conflict with the current state of a service resource associated with the request. |
| `AddBridgeOutputsRequest` | `structure` | `BridgeArn`, `Outputs` | - |
| `AddBridgeOutputsResponse` | `structure` | `BridgeArn`, `Outputs` | - |
| `AddBridgeSourcesRequest` | `structure` | `BridgeArn`, `Sources` | - |
| `AddBridgeSourcesResponse` | `structure` | `BridgeArn`, `Sources` | - |
| `AddFlowMediaStreamsRequest` | `structure` | `FlowArn`, `MediaStreams` | - |
| `AddFlowMediaStreamsResponse` | `structure` | `FlowArn`, `MediaStreams` | - |
| `AddFlowOutputsRequest` | `structure` | `FlowArn`, `Outputs` | - |
| `AddFlowOutputsResponse` | `structure` | `FlowArn`, `Outputs` | - |
| `AddFlowOutputs420Exception` | `structure` | `Message` | Exception raised by Elemental MediaConnect when adding the flow output. |
| `AddFlowSourcesRequest` | `structure` | `FlowArn`, `Sources` | - |
| `AddFlowSourcesResponse` | `structure` | `FlowArn`, `Sources` | - |
| `AddFlowVpcInterfacesRequest` | `structure` | `FlowArn`, `VpcInterfaces` | - |
| `AddFlowVpcInterfacesResponse` | `structure` | `FlowArn`, `VpcInterfaces` | - |
| `BatchGetRouterInputRequest` | `structure` | `Arns` | - |
| `BatchGetRouterInputResponse` | `structure` | `Errors`, `RouterInputs` | - |
| `BatchGetRouterNetworkInterfaceRequest` | `structure` | `Arns` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
