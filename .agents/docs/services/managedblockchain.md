# Amazon Managed Blockchain

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Managed Blockchain is a fully managed service for creating and managing blockchain networks using open-source frameworks. Blockchain allows you to build applications where multiple parties can securely and transparently run transactions and share data without the need for a trusted, central authority. Managed Blockchain supports the Hyperledger Fabric and Ethereum open-source frameworks. Because of fundamental differences between the frameworks, some API actions or data types may only apply in the context of one framework and not the other. For example, actions related to Hyperledger Fabric network members such as `CreateMember` and `DeleteMember` don't apply to Ethereum.

## Possible Usage Scenarios
- Backported from `crates/winterbaume-managedblockchain/tests/scenario_test.rs`: expand a blockchain network by creating members and managing invitations or membership state.
- Backported from `scenario_test.rs`: deploy nodes and manage their tags.
- Backported from `scenario_test.rs`: create and rotate accessor tokens for Ethereum access.
- From the AWS documentation and model: model blockchain networks, members, nodes, proposals/votes, accessors, invitations, and tag-based administration for Hyperledger Fabric or Ethereum-style access surfaces.

## Service Identity and Protocol

- AWS model slug: `managedblockchain`
- AWS SDK for Rust slug: `managedblockchain`
- Model version: `2018-09-24`
- Model file: `vendor/api-models-aws/models/managedblockchain/service/2018-09-24/managedblockchain-2018-09-24.json`
- SDK ID: `ManagedBlockchain`
- Endpoint prefix: `managedblockchain`
- ARN namespace: `managedblockchain`
- CloudFormation name: `ManagedBlockchain`
- CloudTrail event source: `managedblockchain.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (8), `Create` (5), `Get` (5), `Delete` (3), `Update` (2), `Reject` (1), `Tag` (1), `Untag` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateAccessor`, `CreateMember`, `CreateNetwork`, `CreateNode`, `CreateProposal`, `DeleteAccessor`, `DeleteMember`, `DeleteNode`, `RejectInvitation`, `TagResource`, `UntagResource`, `UpdateMember`, `UpdateNode`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetAccessor`, `GetMember`, `GetNetwork`, `GetNode`, `GetProposal`, `ListAccessors`, `ListInvitations`, `ListMembers`, `ListNetworks`, `ListNodes`, `ListProposalVotes`, `ListProposals`, `ListTagsForResource`.
- Pagination is modelled for 7 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 5 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 27 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`.

## Operation Groups

### List

- Operations: `ListAccessors`, `ListInvitations`, `ListMembers`, `ListNetworks`, `ListNodes`, `ListProposals`, `ListProposalVotes`, `ListTagsForResource`
- Traits: `paginated` (7)
- Common required input members in this group: `NetworkId`

### Create

- Operations: `CreateAccessor`, `CreateMember`, `CreateNetwork`, `CreateNode`, `CreateProposal`
- Traits: `idempotency-token` (5)
- Common required input members in this group: `ClientRequestToken`, `NetworkId`, `MemberConfiguration`

### Get

- Operations: `GetAccessor`, `GetMember`, `GetNetwork`, `GetNode`, `GetProposal`
- Common required input members in this group: `NetworkId`

### Delete

- Operations: `DeleteAccessor`, `DeleteMember`, `DeleteNode`
- Common required input members in this group: `NetworkId`

### Update

- Operations: `UpdateMember`, `UpdateNode`
- Common required input members in this group: `NetworkId`

### Reject

- Operations: `RejectInvitation`
- Common required input members in this group: -

### Tag

- Operations: `TagResource`
- Common required input members in this group: -

### Untag

- Operations: `UntagResource`
- Common required input members in this group: -

### Vote

- Operations: `VoteOnProposal`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateAccessor` | `POST /accessors` | `idempotency-token` | `ClientRequestToken`, `AccessorType` | `ClientRequestToken` | `CreateAccessorOutput` | `AccessDeniedException`, `InternalServiceErrorException`, `InvalidRequestException`, `ResourceAlreadyExistsException`, `ResourceLimitExceededException`, `ThrottlingException`, `TooManyTagsException` | Creates a new accessor for use with Amazon Managed Blockchain service that supports token based access. The accessor contains information required for token based access. |
| `CreateMember` | `POST /networks/{NetworkId}/members` | `idempotency-token` | `ClientRequestToken`, `InvitationId`, `NetworkId`, `MemberConfiguration` | `ClientRequestToken` | `CreateMemberOutput` | `AccessDeniedException`, `InternalServiceErrorException`, `InvalidRequestException`, `ResourceAlreadyExistsException`, `ResourceLimitExceededException`, `ResourceNotFoundException`, `ResourceNotReadyException`, `ThrottlingException`, `TooManyTagsException` | Creates a member within a Managed Blockchain network. Applies only to Hyperledger Fabric. |
| `CreateNetwork` | `POST /networks` | `idempotency-token` | `ClientRequestToken`, `Name`, `Framework`, `FrameworkVersion`, `VotingPolicy`, `MemberConfiguration` | `ClientRequestToken` | `CreateNetworkOutput` | `AccessDeniedException`, `InternalServiceErrorException`, `InvalidRequestException`, `ResourceAlreadyExistsException`, `ResourceLimitExceededException`, `ThrottlingException`, `TooManyTagsException` | Creates a new blockchain network using Amazon Managed Blockchain. Applies only to Hyperledger Fabric. |
| `CreateNode` | `POST /networks/{NetworkId}/nodes` | `idempotency-token` | `ClientRequestToken`, `NetworkId`, `NodeConfiguration` | `ClientRequestToken` | `CreateNodeOutput` | `AccessDeniedException`, `InternalServiceErrorException`, `InvalidRequestException`, `ResourceAlreadyExistsException`, `ResourceLimitExceededException`, `ResourceNotFoundException`, `ResourceNotReadyException`, `ThrottlingException`, `TooManyTagsException` | Creates a node on the specified blockchain network. Applies to Hyperledger Fabric and Ethereum. |
| `CreateProposal` | `POST /networks/{NetworkId}/proposals` | `idempotency-token` | `ClientRequestToken`, `NetworkId`, `MemberId`, `Actions` | `ClientRequestToken` | `CreateProposalOutput` | `AccessDeniedException`, `InternalServiceErrorException`, `InvalidRequestException`, `ResourceNotFoundException`, `ResourceNotReadyException`, `ThrottlingException`, `TooManyTagsException` | Creates a proposal for a change to the network that other members of the network can vote on, for example, a proposal to add a new member to the network. Any member can create a proposal. Applies only to Hyperledger ... |
| `DeleteAccessor` | `DELETE /accessors/{AccessorId}` | - | `AccessorId` | - | `DeleteAccessorOutput` | `AccessDeniedException`, `InternalServiceErrorException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Deletes an accessor that your Amazon Web Services account owns. An accessor object is a container that has the information required for token based access to your Ethereum nodes including, the BILLING_TOKEN . After a ... |
| `DeleteMember` | `DELETE /networks/{NetworkId}/members/{MemberId}` | - | `NetworkId`, `MemberId` | - | `DeleteMemberOutput` | `AccessDeniedException`, `InternalServiceErrorException`, `InvalidRequestException`, `ResourceNotFoundException`, `ResourceNotReadyException`, `ThrottlingException` | Deletes a member. Deleting a member removes the member and all associated resources from the network. DeleteMember can only be called for a specified MemberId if the principal performing the action is associated with ... |
| `DeleteNode` | `DELETE /networks/{NetworkId}/nodes/{NodeId}` | - | `NetworkId`, `NodeId` | - | `DeleteNodeOutput` | `AccessDeniedException`, `InternalServiceErrorException`, `InvalidRequestException`, `ResourceNotFoundException`, `ResourceNotReadyException`, `ThrottlingException` | Deletes a node that your Amazon Web Services account owns. All data on the node is lost and cannot be recovered. Applies to Hyperledger Fabric and Ethereum. |
| `GetAccessor` | `GET /accessors/{AccessorId}` | - | `AccessorId` | - | `GetAccessorOutput` | `AccessDeniedException`, `InternalServiceErrorException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Returns detailed information about an accessor. An accessor object is a container that has the information required for token based access to your Ethereum nodes. |
| `GetMember` | `GET /networks/{NetworkId}/members/{MemberId}` | - | `NetworkId`, `MemberId` | - | `GetMemberOutput` | `AccessDeniedException`, `InternalServiceErrorException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Returns detailed information about a member. Applies only to Hyperledger Fabric. |
| `GetNetwork` | `GET /networks/{NetworkId}` | - | `NetworkId` | - | `GetNetworkOutput` | `AccessDeniedException`, `InternalServiceErrorException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Returns detailed information about a network. Applies to Hyperledger Fabric and Ethereum. |
| `GetNode` | `GET /networks/{NetworkId}/nodes/{NodeId}` | - | `NetworkId`, `NodeId` | - | `GetNodeOutput` | `AccessDeniedException`, `InternalServiceErrorException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Returns detailed information about a node. Applies to Hyperledger Fabric and Ethereum. |
| `GetProposal` | `GET /networks/{NetworkId}/proposals/{ProposalId}` | - | `NetworkId`, `ProposalId` | - | `GetProposalOutput` | `AccessDeniedException`, `InternalServiceErrorException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Returns detailed information about a proposal. Applies only to Hyperledger Fabric. |
| `ListAccessors` | `GET /accessors` | `paginated` | - | - | `ListAccessorsOutput` | `AccessDeniedException`, `InternalServiceErrorException`, `InvalidRequestException`, `ThrottlingException` | Returns a list of the accessors and their properties. Accessor objects are containers that have the information required for token based access to your Ethereum nodes. |
| `ListInvitations` | `GET /invitations` | `paginated` | - | - | `ListInvitationsOutput` | `AccessDeniedException`, `InternalServiceErrorException`, `InvalidRequestException`, `ResourceLimitExceededException`, `ResourceNotFoundException`, `ThrottlingException` | Returns a list of all invitations for the current Amazon Web Services account. Applies only to Hyperledger Fabric. |
| `ListMembers` | `GET /networks/{NetworkId}/members` | `paginated` | `NetworkId` | - | `ListMembersOutput` | `AccessDeniedException`, `InternalServiceErrorException`, `InvalidRequestException`, `ThrottlingException` | Returns a list of the members in a network and properties of their configurations. Applies only to Hyperledger Fabric. |
| `ListNetworks` | `GET /networks` | `paginated` | - | - | `ListNetworksOutput` | `AccessDeniedException`, `InternalServiceErrorException`, `InvalidRequestException`, `ThrottlingException` | Returns information about the networks in which the current Amazon Web Services account participates. Applies to Hyperledger Fabric and Ethereum. |
| `ListNodes` | `GET /networks/{NetworkId}/nodes` | `paginated` | `NetworkId` | - | `ListNodesOutput` | `AccessDeniedException`, `InternalServiceErrorException`, `InvalidRequestException`, `ThrottlingException` | Returns information about the nodes within a network. Applies to Hyperledger Fabric and Ethereum. |
| `ListProposals` | `GET /networks/{NetworkId}/proposals` | `paginated` | `NetworkId` | - | `ListProposalsOutput` | `AccessDeniedException`, `InternalServiceErrorException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Returns a list of proposals for the network. Applies only to Hyperledger Fabric. |
| `ListProposalVotes` | `GET /networks/{NetworkId}/proposals/{ProposalId}/votes` | `paginated` | `NetworkId`, `ProposalId` | - | `ListProposalVotesOutput` | `AccessDeniedException`, `InternalServiceErrorException`, `InvalidRequestException`, `ThrottlingException` | Returns the list of votes for a specified proposal, including the value of each vote and the unique identifier of the member that cast the vote. Applies only to Hyperledger Fabric. |
| `ListTagsForResource` | `GET /tags/{ResourceArn}` | - | `ResourceArn` | - | `ListTagsForResourceResponse` | `InternalServiceErrorException`, `InvalidRequestException`, `ResourceNotFoundException`, `ResourceNotReadyException` | Returns a list of tags for the specified resource. Each tag consists of a key and optional value. For more information about tags, see Tagging Resources in the Amazon Managed Blockchain Ethereum Developer Guide , or ... |
| `RejectInvitation` | `DELETE /invitations/{InvitationId}` | - | `InvitationId` | - | `RejectInvitationOutput` | `AccessDeniedException`, `IllegalActionException`, `InternalServiceErrorException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Rejects an invitation to join a network. This action can be called by a principal in an Amazon Web Services account that has received an invitation to create a member and join a network. Applies only to Hyperledger F ... |
| `TagResource` | `POST /tags/{ResourceArn}` | - | `ResourceArn`, `Tags` | - | `TagResourceResponse` | `InternalServiceErrorException`, `InvalidRequestException`, `ResourceNotFoundException`, `ResourceNotReadyException`, `TooManyTagsException` | Adds or overwrites the specified tags for the specified Amazon Managed Blockchain resource. Each tag consists of a key and optional value. When you specify a tag key that already exists, the tag value is overwritten ... |
| `UntagResource` | `DELETE /tags/{ResourceArn}` | - | `ResourceArn`, `TagKeys` | - | `UntagResourceResponse` | `InternalServiceErrorException`, `InvalidRequestException`, `ResourceNotFoundException`, `ResourceNotReadyException` | Removes the specified tags from the Amazon Managed Blockchain resource. For more information about tags, see Tagging Resources in the Amazon Managed Blockchain Ethereum Developer Guide , or Tagging Resources in the A ... |
| `UpdateMember` | `PATCH /networks/{NetworkId}/members/{MemberId}` | - | `NetworkId`, `MemberId` | - | `UpdateMemberOutput` | `AccessDeniedException`, `InternalServiceErrorException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Updates a member configuration with new parameters. Applies only to Hyperledger Fabric. |
| `UpdateNode` | `PATCH /networks/{NetworkId}/nodes/{NodeId}` | - | `NetworkId`, `NodeId` | - | `UpdateNodeOutput` | `AccessDeniedException`, `InternalServiceErrorException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Updates a node configuration with new parameters. Applies only to Hyperledger Fabric. |
| `VoteOnProposal` | `POST /networks/{NetworkId}/proposals/{ProposalId}/votes` | - | `NetworkId`, `ProposalId`, `VoterMemberId`, `Vote` | - | `VoteOnProposalOutput` | `AccessDeniedException`, `IllegalActionException`, `InternalServiceErrorException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Casts a vote for a specified ProposalId on behalf of a member. The member to vote as, specified by VoterMemberId , must be in the same Amazon Web Services account as the principal that calls the action. Applies only ... |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `DeleteNode` | - | `MemberId -> memberId` | - | - |
| `GetNode` | - | `MemberId -> memberId` | - | - |
| `ListAccessors` | - | `MaxResults -> maxResults`, `NextToken -> nextToken`, `NetworkType -> networkType` | - | - |
| `ListInvitations` | - | `MaxResults -> maxResults`, `NextToken -> nextToken` | - | - |
| `ListMembers` | - | `Name -> name`, `Status -> status`, `IsOwned -> isOwned`, `MaxResults -> maxResults`, `NextToken -> nextToken` | - | - |
| `ListNetworks` | - | `Name -> name`, `Framework -> framework`, `Status -> status`, `MaxResults -> maxResults`, `NextToken -> nextToken` | - | - |
| `ListNodes` | - | `MemberId -> memberId`, `Status -> status`, `MaxResults -> maxResults`, `NextToken -> nextToken` | - | - |
| `ListProposals` | - | `MaxResults -> maxResults`, `NextToken -> nextToken` | - | - |
| `ListProposalVotes` | - | `MaxResults -> maxResults`, `NextToken -> nextToken` | - | - |
| `UntagResource` | - | `TagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | Message | You don't have sufficient access to perform this action. |
| `IllegalActionException` | `structure` | Message | - |
| `InternalServiceErrorException` | `structure` | **empty (no members)** | The request processing has failed because of an unknown error, exception or failure. |
| `InvalidRequestException` | `structure` | Message | The action or operation requested is invalid. Verify that the action is typed correctly. |
| `ResourceAlreadyExistsException` | `structure` | Message | A resource request is issued for a resource that already exists. |
| `ResourceLimitExceededException` | `structure` | Message | The maximum number of resources of that type already exist. Ensure the resources requested are within the boundaries of the service edition and your account ... |
| `ResourceNotFoundException` | `structure` | Message, ResourceName | A requested resource doesn't exist. It may have been deleted or referenced incorrectly. |
| `ResourceNotReadyException` | `structure` | Message | The requested resource exists but isn't in a status that can complete the operation. |
| `ThrottlingException` | `structure` | **empty (no members)** | The request or operation couldn't be performed because a service is throttling requests. The most common source of throttling errors is creating resources t ... |
| `TooManyTagsException` | `structure` | Message, ResourceName | - |
| `CreateAccessorInput` | `structure` | ClientRequestToken, AccessorType, Tags, NetworkType | - |
| `CreateAccessorOutput` | `structure` | AccessorId, BillingToken, NetworkType | - |
| `CreateMemberInput` | `structure` | ClientRequestToken, InvitationId, NetworkId, MemberConfiguration | - |
| `CreateMemberOutput` | `structure` | MemberId | - |
| `CreateNetworkInput` | `structure` | ClientRequestToken, Name, Description, Framework, FrameworkVersion, FrameworkConfiguration, VotingPolicy, MemberConfiguration, Tags | - |
| `CreateNetworkOutput` | `structure` | NetworkId, MemberId | - |
| `CreateNodeInput` | `structure` | ClientRequestToken, NetworkId, MemberId, NodeConfiguration, Tags | - |
| `CreateNodeOutput` | `structure` | NodeId | - |
| `CreateProposalInput` | `structure` | ClientRequestToken, NetworkId, MemberId, Actions, Description, Tags | - |
| `CreateProposalOutput` | `structure` | ProposalId | - |
| `DeleteAccessorInput` | `structure` | AccessorId | - |
| `DeleteAccessorOutput` | `structure` | **empty (no members)** | - |
| `DeleteMemberInput` | `structure` | NetworkId, MemberId | - |
| `DeleteMemberOutput` | `structure` | **empty (no members)** | - |
| `DeleteNodeInput` | `structure` | NetworkId, MemberId, NodeId | - |
| `DeleteNodeOutput` | `structure` | **empty (no members)** | - |
| `GetAccessorInput` | `structure` | AccessorId | - |
| `GetAccessorOutput` | `structure` | Accessor | - |
| `GetMemberInput` | `structure` | NetworkId, MemberId | - |
| `GetMemberOutput` | `structure` | Member | - |
| `GetNetworkInput` | `structure` | NetworkId | - |
| `GetNetworkOutput` | `structure` | Network | - |
| `GetNodeInput` | `structure` | NetworkId, MemberId, NodeId | - |
| `GetNodeOutput` | `structure` | Node | - |
| `GetProposalInput` | `structure` | NetworkId, ProposalId | - |
| `GetProposalOutput` | `structure` | Proposal | - |
| `ListAccessorsInput` | `structure` | MaxResults, NextToken, NetworkType | - |
| `ListAccessorsOutput` | `structure` | Accessors, NextToken | - |
| `ListInvitationsInput` | `structure` | MaxResults, NextToken | - |
| `ListInvitationsOutput` | `structure` | Invitations, NextToken | - |
| `AccessorNetworkType` | `enum` | ETHEREUM_GOERLI, ETHEREUM_MAINNET, ETHEREUM_MAINNET_AND_GOERLI, POLYGON_MAINNET, POLYGON_MUMBAI | - |
| `AccessorStatus` | `enum` | AVAILABLE, PENDING_DELETION, DELETED | - |
| `AccessorType` | `enum` | BILLING_TOKEN | - |
| `Edition` | `enum` | STARTER, STANDARD | - |
| `Framework` | `enum` | HYPERLEDGER_FABRIC, ETHEREUM | - |
| `InvitationStatus` | `enum` | PENDING, ACCEPTED, ACCEPTING, REJECTED, EXPIRED | - |
| `MemberStatus` | `enum` | CREATING, AVAILABLE, CREATE_FAILED, UPDATING, DELETING, DELETED, INACCESSIBLE_ENCRYPTION_KEY | - |
| `NetworkStatus` | `enum` | CREATING, AVAILABLE, CREATE_FAILED, DELETING, DELETED | - |
| `NodeStatus` | `enum` | CREATING, AVAILABLE, UNHEALTHY, CREATE_FAILED, UPDATING, DELETING, DELETED, FAILED, INACCESSIBLE_ENCRYPTION_KEY | - |
| `ProposalStatus` | `enum` | IN_PROGRESS, APPROVED, REJECTED, EXPIRED, ACTION_FAILED | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
