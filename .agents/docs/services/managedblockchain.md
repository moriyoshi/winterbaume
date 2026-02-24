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

- Operations: `ListAccessors`, `ListInvitations`, `ListMembers`, `ListNetworks`, `ListNodes`, `ListProposalVotes`, `ListProposals`, `ListTagsForResource`
- Traits: `paginated` (7)
- Common required input members in this group: `NetworkId`, `ProposalId`, `ResourceArn`

### Create

- Operations: `CreateAccessor`, `CreateMember`, `CreateNetwork`, `CreateNode`, `CreateProposal`
- Traits: `idempotency-token` (5)
- Common required input members in this group: `AccessorType`, `Actions`, `ClientRequestToken`, `Framework`, `FrameworkVersion`, `InvitationId`, `MemberConfiguration`, `MemberId`, `Name`, `NetworkId`, `NodeConfiguration`, `VotingPolicy`

### Get

- Operations: `GetAccessor`, `GetMember`, `GetNetwork`, `GetNode`, `GetProposal`
- Common required input members in this group: `AccessorId`, `MemberId`, `NetworkId`, `NodeId`, `ProposalId`

### Delete

- Operations: `DeleteAccessor`, `DeleteMember`, `DeleteNode`
- Common required input members in this group: `AccessorId`, `MemberId`, `NetworkId`, `NodeId`

### Update

- Operations: `UpdateMember`, `UpdateNode`
- Common required input members in this group: `MemberId`, `NetworkId`, `NodeId`

### Reject

- Operations: `RejectInvitation`
- Common required input members in this group: `InvitationId`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `ResourceArn`, `Tags`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `ResourceArn`, `TagKeys`

### Vote

- Operations: `VoteOnProposal`
- Common required input members in this group: `NetworkId`, `ProposalId`, `Vote`, `VoterMemberId`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateAccessor` | `POST /accessors` | `idempotency-token` | `AccessorType`, `ClientRequestToken` | `ClientRequestToken` | `CreateAccessorOutput` | `AccessDeniedException`, `InternalServiceErrorException`, `InvalidRequestException`, `ResourceAlreadyExistsException`, `ResourceLimitExceededException`, `ThrottlingException`, `TooManyTagsException` | Creates a new accessor for use with Amazon Managed Blockchain service that supports token based access. The accessor contains information required for token based access. |
| `CreateMember` | `POST /networks/{NetworkId}/members` | `idempotency-token` | `ClientRequestToken`, `InvitationId`, `MemberConfiguration`, `NetworkId` | `ClientRequestToken` | `CreateMemberOutput` | `AccessDeniedException`, `InternalServiceErrorException`, `InvalidRequestException`, `ResourceAlreadyExistsException`, `ResourceLimitExceededException`, `ResourceNotFoundException`, `ResourceNotReadyException`, `ThrottlingException`, ... (+1) | Creates a member within a Managed Blockchain network. Applies only to Hyperledger Fabric. |
| `CreateNetwork` | `POST /networks` | `idempotency-token` | `ClientRequestToken`, `Framework`, `FrameworkVersion`, `MemberConfiguration`, `Name`, `VotingPolicy` | `ClientRequestToken` | `CreateNetworkOutput` | `AccessDeniedException`, `InternalServiceErrorException`, `InvalidRequestException`, `ResourceAlreadyExistsException`, `ResourceLimitExceededException`, `ThrottlingException`, `TooManyTagsException` | Creates a new blockchain network using Amazon Managed Blockchain. Applies only to Hyperledger Fabric. |
| `CreateNode` | `POST /networks/{NetworkId}/nodes` | `idempotency-token` | `ClientRequestToken`, `NetworkId`, `NodeConfiguration` | `ClientRequestToken` | `CreateNodeOutput` | `AccessDeniedException`, `InternalServiceErrorException`, `InvalidRequestException`, `ResourceAlreadyExistsException`, `ResourceLimitExceededException`, `ResourceNotFoundException`, `ResourceNotReadyException`, `ThrottlingException`, ... (+1) | Creates a node on the specified blockchain network. Applies to Hyperledger Fabric and Ethereum. |
| `CreateProposal` | `POST /networks/{NetworkId}/proposals` | `idempotency-token` | `Actions`, `ClientRequestToken`, `MemberId`, `NetworkId` | `ClientRequestToken` | `CreateProposalOutput` | `AccessDeniedException`, `InternalServiceErrorException`, `InvalidRequestException`, `ResourceNotFoundException`, `ResourceNotReadyException`, `ThrottlingException`, `TooManyTagsException` | Creates a proposal for a change to the network that other members of the network can vote on, for example, a proposal to add a new member to the network. Any member can create a proposal. |
| `DeleteAccessor` | `DELETE /accessors/{AccessorId}` | - | `AccessorId` | - | `DeleteAccessorOutput` | `AccessDeniedException`, `InternalServiceErrorException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Deletes an accessor that your Amazon Web Services account owns. An accessor object is a container that has the information required for token based access to your Ethereum nodes including, the `BILLING_TOKEN`. |
| `DeleteMember` | `DELETE /networks/{NetworkId}/members/{MemberId}` | - | `MemberId`, `NetworkId` | - | `DeleteMemberOutput` | `AccessDeniedException`, `InternalServiceErrorException`, `InvalidRequestException`, `ResourceNotFoundException`, `ResourceNotReadyException`, `ThrottlingException` | Deletes a member. Deleting a member removes the member and all associated resources from the network. |
| `DeleteNode` | `DELETE /networks/{NetworkId}/nodes/{NodeId}` | - | `NetworkId`, `NodeId` | - | `DeleteNodeOutput` | `AccessDeniedException`, `InternalServiceErrorException`, `InvalidRequestException`, `ResourceNotFoundException`, `ResourceNotReadyException`, `ThrottlingException` | Deletes a node that your Amazon Web Services account owns. All data on the node is lost and cannot be recovered. |
| `GetAccessor` | `GET /accessors/{AccessorId}` | - | `AccessorId` | - | `GetAccessorOutput` | `AccessDeniedException`, `InternalServiceErrorException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Returns detailed information about an accessor. An accessor object is a container that has the information required for token based access to your Ethereum nodes. |
| `GetMember` | `GET /networks/{NetworkId}/members/{MemberId}` | - | `MemberId`, `NetworkId` | - | `GetMemberOutput` | `AccessDeniedException`, `InternalServiceErrorException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Returns detailed information about a member. Applies only to Hyperledger Fabric. |
| `GetNetwork` | `GET /networks/{NetworkId}` | - | `NetworkId` | - | `GetNetworkOutput` | `AccessDeniedException`, `InternalServiceErrorException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Returns detailed information about a network. Applies to Hyperledger Fabric and Ethereum. |
| `GetNode` | `GET /networks/{NetworkId}/nodes/{NodeId}` | - | `NetworkId`, `NodeId` | - | `GetNodeOutput` | `AccessDeniedException`, `InternalServiceErrorException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Returns detailed information about a node. Applies to Hyperledger Fabric and Ethereum. |
| `GetProposal` | `GET /networks/{NetworkId}/proposals/{ProposalId}` | - | `NetworkId`, `ProposalId` | - | `GetProposalOutput` | `AccessDeniedException`, `InternalServiceErrorException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Returns detailed information about a proposal. Applies only to Hyperledger Fabric. |
| `ListAccessors` | `GET /accessors` | `paginated` | - | - | `ListAccessorsOutput` | `AccessDeniedException`, `InternalServiceErrorException`, `InvalidRequestException`, `ThrottlingException` | Returns a list of the accessors and their properties. Accessor objects are containers that have the information required for token based access to your Ethereum nodes. |
| `ListInvitations` | `GET /invitations` | `paginated` | - | - | `ListInvitationsOutput` | `AccessDeniedException`, `InternalServiceErrorException`, `InvalidRequestException`, `ResourceLimitExceededException`, `ResourceNotFoundException`, `ThrottlingException` | Returns a list of all invitations for the current Amazon Web Services account. Applies only to Hyperledger Fabric. |
| `ListMembers` | `GET /networks/{NetworkId}/members` | `paginated` | `NetworkId` | - | `ListMembersOutput` | `AccessDeniedException`, `InternalServiceErrorException`, `InvalidRequestException`, `ThrottlingException` | Returns a list of the members in a network and properties of their configurations. Applies only to Hyperledger Fabric. |
| `ListNetworks` | `GET /networks` | `paginated` | - | - | `ListNetworksOutput` | `AccessDeniedException`, `InternalServiceErrorException`, `InvalidRequestException`, `ThrottlingException` | Returns information about the networks in which the current Amazon Web Services account participates. Applies to Hyperledger Fabric and Ethereum. |
| `ListNodes` | `GET /networks/{NetworkId}/nodes` | `paginated` | `NetworkId` | - | `ListNodesOutput` | `AccessDeniedException`, `InternalServiceErrorException`, `InvalidRequestException`, `ThrottlingException` | Returns information about the nodes within a network. Applies to Hyperledger Fabric and Ethereum. |
| `ListProposalVotes` | `GET /networks/{NetworkId}/proposals/{ProposalId}/votes` | `paginated` | `NetworkId`, `ProposalId` | - | `ListProposalVotesOutput` | `AccessDeniedException`, `InternalServiceErrorException`, `InvalidRequestException`, `ThrottlingException` | Returns the list of votes for a specified proposal, including the value of each vote and the unique identifier of the member that cast the vote. Applies only to Hyperledger Fabric. |
| `ListProposals` | `GET /networks/{NetworkId}/proposals` | `paginated` | `NetworkId` | - | `ListProposalsOutput` | `AccessDeniedException`, `InternalServiceErrorException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Returns a list of proposals for the network. Applies only to Hyperledger Fabric. |
| `ListTagsForResource` | `GET /tags/{ResourceArn}` | - | `ResourceArn` | - | `ListTagsForResourceResponse` | `InternalServiceErrorException`, `InvalidRequestException`, `ResourceNotFoundException`, `ResourceNotReadyException` | Returns a list of tags for the specified resource. Each tag consists of a key and optional value. |
| `RejectInvitation` | `DELETE /invitations/{InvitationId}` | - | `InvitationId` | - | `RejectInvitationOutput` | `AccessDeniedException`, `IllegalActionException`, `InternalServiceErrorException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Rejects an invitation to join a network. This action can be called by a principal in an Amazon Web Services account that has received an invitation to create a member and join a network. |
| `TagResource` | `POST /tags/{ResourceArn}` | - | `ResourceArn`, `Tags` | - | `TagResourceResponse` | `InternalServiceErrorException`, `InvalidRequestException`, `ResourceNotFoundException`, `ResourceNotReadyException`, `TooManyTagsException` | Adds or overwrites the specified tags for the specified Amazon Managed Blockchain resource. Each tag consists of a key and optional value. |
| `UntagResource` | `DELETE /tags/{ResourceArn}` | - | `ResourceArn`, `TagKeys` | - | `UntagResourceResponse` | `InternalServiceErrorException`, `InvalidRequestException`, `ResourceNotFoundException`, `ResourceNotReadyException` | Removes the specified tags from the Amazon Managed Blockchain resource. For more information about tags, see Tagging Resources in the Amazon Managed Blockchain Ethereum Developer Guide , or Tagging Resources in the Amazon Managed Blockchain Hyperledger Fabric... |
| `UpdateMember` | `PATCH /networks/{NetworkId}/members/{MemberId}` | - | `MemberId`, `NetworkId` | - | `UpdateMemberOutput` | `AccessDeniedException`, `InternalServiceErrorException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Updates a member configuration with new parameters. Applies only to Hyperledger Fabric. |
| `UpdateNode` | `PATCH /networks/{NetworkId}/nodes/{NodeId}` | - | `NetworkId`, `NodeId` | - | `UpdateNodeOutput` | `AccessDeniedException`, `InternalServiceErrorException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Updates a node configuration with new parameters. Applies only to Hyperledger Fabric. |
| `VoteOnProposal` | `POST /networks/{NetworkId}/proposals/{ProposalId}/votes` | - | `NetworkId`, `ProposalId`, `Vote`, `VoterMemberId` | - | `VoteOnProposalOutput` | `AccessDeniedException`, `IllegalActionException`, `InternalServiceErrorException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException` | Casts a vote for a specified `ProposalId` on behalf of a member. The member to vote as, specified by `VoterMemberId`, must be in the same Amazon Web Services account as the principal that calls the action. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InternalServiceErrorException` | `structure` | - | The request processing has failed because of an unknown error, exception or failure. |
| `InvalidRequestException` | `structure` | `Message` | The action or operation requested is invalid. |
| `AccessDeniedException` | `structure` | `Message` | You don't have sufficient access to perform this action. |
| `ThrottlingException` | `structure` | - | The request or operation couldn't be performed because a service is throttling requests. |
| `ResourceNotFoundException` | `structure` | `Message`, `ResourceName` | A requested resource doesn't exist. |
| `ResourceNotReadyException` | `structure` | `Message` | The requested resource exists but isn't in a status that can complete the operation. |
| `TooManyTagsException` | `structure` | `Message`, `ResourceName` | - |
| `ResourceLimitExceededException` | `structure` | `Message` | The maximum number of resources of that type already exist. |
| `ResourceAlreadyExistsException` | `structure` | `Message` | A resource request is issued for a resource that already exists. |
| `IllegalActionException` | `structure` | `Message` | - |
| `CreateAccessorInput` | `structure` | `AccessorType`, `ClientRequestToken`, `NetworkType`, `Tags` | - |
| `CreateAccessorOutput` | `structure` | `AccessorId`, `BillingToken`, `NetworkType` | - |
| `CreateMemberInput` | `structure` | `ClientRequestToken`, `InvitationId`, `MemberConfiguration`, `NetworkId` | - |
| `CreateMemberOutput` | `structure` | `MemberId` | - |
| `CreateNetworkInput` | `structure` | `ClientRequestToken`, `Description`, `Framework`, `FrameworkConfiguration`, `FrameworkVersion`, `MemberConfiguration`, `Name`, `Tags`, `VotingPolicy` | - |
| `CreateNetworkOutput` | `structure` | `MemberId`, `NetworkId` | - |
| `CreateNodeInput` | `structure` | `ClientRequestToken`, `MemberId`, `NetworkId`, `NodeConfiguration`, `Tags` | - |
| `CreateNodeOutput` | `structure` | `NodeId` | - |
| `CreateProposalInput` | `structure` | `Actions`, `ClientRequestToken`, `Description`, `MemberId`, `NetworkId`, `Tags` | - |
| `CreateProposalOutput` | `structure` | `ProposalId` | - |
| `DeleteAccessorInput` | `structure` | `AccessorId` | - |
| `DeleteAccessorOutput` | `structure` | - | - |
| `DeleteMemberInput` | `structure` | `MemberId`, `NetworkId` | - |
| `DeleteMemberOutput` | `structure` | - | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
