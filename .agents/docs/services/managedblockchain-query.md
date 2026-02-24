# Amazon Managed Blockchain Query

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Managed Blockchain (AMB) Query provides you with convenient access to multi-blockchain network data, which makes it easier for you to extract contextual data related to blockchain activity. You can use AMB Query to read data from public blockchain networks, such as Bitcoin Mainnet and Ethereum Mainnet. You can also get information such as the current and historical balances of addresses, or you can get a list of blockchain transactions for a given time period. Additionally, you can get details of a given transaction, such as transaction events, which you can further analyze or use in business logic for your applications.

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented Amazon Managed Blockchain Query workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model discovery and reporting workflows that retrieve, list, filter, or query service-managed state across the `List`, `Get`, `Batch` operation families, including `ListAssetContracts`, `ListFilteredTransactionEvents`, `ListTokenBalances`, `ListTransactionEvents`, `GetAssetContract`, `GetTokenBalance`.

## Service Identity and Protocol

- AWS model slug: `managedblockchain-query`
- AWS SDK for Rust slug: `managedblockchainquery`
- Model version: `2023-05-04`
- Model file: `vendor/api-models-aws/models/managedblockchain-query/service/2023-05-04/managedblockchain-query-2023-05-04.json`
- SDK ID: `ManagedBlockchain Query`
- Endpoint prefix: `-`
- ARN namespace: `managedblockchain-query`
- CloudFormation name: `-`
- CloudTrail event source: `-`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (5), `Get` (3), `Batch` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `BatchGetTokenBalance`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `BatchGetTokenBalance`, `GetAssetContract`, `GetTokenBalance`, `GetTransaction`, `ListAssetContracts`, `ListFilteredTransactionEvents`, `ListTokenBalances`, `ListTransactionEvents`, `ListTransactions`.
- Pagination is modelled for 5 operations; token stability and page boundaries are observable API behaviour.
- 9 operations declare modelled service errors; parity work should map exact error names and retryability where documented.

## Operation Groups

### List

- Operations: `ListAssetContracts`, `ListFilteredTransactionEvents`, `ListTokenBalances`, `ListTransactionEvents`, `ListTransactions`
- Traits: `paginated` (5), `readonly` (5)
- Common required input members in this group: `address`, `addressIdentifierFilter`, `contractFilter`, `network`, `tokenFilter`

### Get

- Operations: `GetAssetContract`, `GetTokenBalance`, `GetTransaction`
- Traits: `readonly` (3)
- Common required input members in this group: `contractIdentifier`, `network`, `ownerIdentifier`, `tokenIdentifier`

### Batch

- Operations: `BatchGetTokenBalance`
- Traits: `readonly` (1)

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `BatchGetTokenBalance` | `POST /batch-get-token-balance` | `readonly` | - | - | `BatchGetTokenBalanceOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Gets the token balance for a batch of tokens by using the `BatchGetTokenBalance` action for every token in the request. Only the native tokens BTC and ETH, and the ERC-20, ERC-721, and ERC 1155 token standards are supported. |
| `GetAssetContract` | `POST /get-asset-contract` | `readonly` | `contractIdentifier` | - | `GetAssetContractOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Gets the information about a specific contract deployed on the blockchain. The Bitcoin blockchain networks do not support this operation. |
| `GetTokenBalance` | `POST /get-token-balance` | `readonly` | `ownerIdentifier`, `tokenIdentifier` | - | `GetTokenBalanceOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Gets the balance of a specific token, including native tokens, for a given address (wallet or contract) on the blockchain. Only the native tokens BTC and ETH, and the ERC-20, ERC-721, and ERC 1155 token standards are supported. |
| `GetTransaction` | `POST /get-transaction` | `readonly` | `network` | - | `GetTransactionOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Gets the details of a transaction. This action will return transaction details for all transactions that are confirmed on the blockchain, even if they have not reached finality. |
| `ListAssetContracts` | `POST /list-asset-contracts` | `readonly`, `paginated` | `contractFilter` | - | `ListAssetContractsOutput` | `AccessDeniedException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Lists all the contracts for a given contract type deployed by an address (either a contract address or a wallet address). The Bitcoin blockchain networks do not support this operation. |
| `ListFilteredTransactionEvents` | `POST /list-filtered-transaction-events` | `readonly`, `paginated` | `addressIdentifierFilter`, `network` | - | `ListFilteredTransactionEventsOutput` | `AccessDeniedException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Lists all the transaction events for an address on the blockchain. This operation is only supported on the Bitcoin networks. |
| `ListTokenBalances` | `POST /list-token-balances` | `readonly`, `paginated` | `tokenFilter` | - | `ListTokenBalancesOutput` | `AccessDeniedException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | This action returns the following for a given blockchain network: Lists all token balances owned by an address (either a contract address or a wallet address). Lists all token balances for all tokens created by a contract. |
| `ListTransactionEvents` | `POST /list-transaction-events` | `readonly`, `paginated` | `network` | - | `ListTransactionEventsOutput` | `AccessDeniedException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Lists all the transaction events for a transaction This action will return transaction details for all transactions that are confirmed on the blockchain, even if they have not reached finality. |
| `ListTransactions` | `POST /list-transactions` | `readonly`, `paginated` | `address`, `network` | - | `ListTransactionsOutput` | `AccessDeniedException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Lists all the transaction events for a transaction. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | `message` | The Amazon Web Services account doesn’t have access to this resource. |
| `InternalServerException` | `structure` | `message`, `retryAfterSeconds` | The request processing has failed because of an internal error in the service. |
| `ServiceQuotaExceededException` | `structure` | `message`, `quotaCode`, `resourceId`, `resourceType`, `serviceCode` | The service quota has been exceeded for this resource. |
| `ThrottlingException` | `structure` | `message`, `quotaCode`, `retryAfterSeconds`, `serviceCode` | The request or operation couldn't be performed because a service is throttling requests. |
| `ValidationException` | `structure` | `fieldList`, `message`, `reason` | The resource passed is invalid. |
| `ResourceNotFoundException` | `structure` | `message`, `resourceId`, `resourceType` | The resource was not found. |
| `BatchGetTokenBalanceInput` | `structure` | `getTokenBalanceInputs` | - |
| `BatchGetTokenBalanceOutput` | `structure` | `errors`, `tokenBalances` | - |
| `GetAssetContractInput` | `structure` | `contractIdentifier` | - |
| `GetAssetContractOutput` | `structure` | `contractIdentifier`, `deployerAddress`, `metadata`, `tokenStandard` | - |
| `GetTokenBalanceInput` | `structure` | `atBlockchainInstant`, `ownerIdentifier`, `tokenIdentifier` | - |
| `GetTokenBalanceOutput` | `structure` | `atBlockchainInstant`, `balance`, `lastUpdatedTime`, `ownerIdentifier`, `tokenIdentifier` | - |
| `GetTransactionInput` | `structure` | `network`, `transactionHash`, `transactionId` | - |
| `GetTransactionOutput` | `structure` | `transaction` | - |
| `ListAssetContractsInput` | `structure` | `contractFilter`, `maxResults`, `nextToken` | - |
| `ListAssetContractsOutput` | `structure` | `contracts`, `nextToken` | - |
| `ListFilteredTransactionEventsInput` | `structure` | `addressIdentifierFilter`, `confirmationStatusFilter`, `maxResults`, `network`, `nextToken`, `sort`, `timeFilter`, `voutFilter` | - |
| `ListFilteredTransactionEventsOutput` | `structure` | `events`, `nextToken` | - |
| `ListTokenBalancesInput` | `structure` | `maxResults`, `nextToken`, `ownerFilter`, `tokenFilter` | - |
| `ListTokenBalancesOutput` | `structure` | `nextToken`, `tokenBalances` | - |
| `ListTransactionEventsInput` | `structure` | `maxResults`, `network`, `nextToken`, `transactionHash`, `transactionId` | - |
| `ListTransactionEventsOutput` | `structure` | `events`, `nextToken` | - |
| `ListTransactionsInput` | `structure` | `address`, `confirmationStatusFilter`, `fromBlockchainInstant`, `maxResults`, `network`, `nextToken`, `sort`, `toBlockchainInstant` | - |
| `ListTransactionsOutput` | `structure` | `nextToken`, `transactions` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
