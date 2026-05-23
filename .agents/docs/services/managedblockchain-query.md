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
- Traits: `readonly` (5), `paginated` (5)
- Common required input members in this group: `network`

### Get

- Operations: `GetAssetContract`, `GetTokenBalance`, `GetTransaction`
- Traits: `readonly` (3)
- Common required input members in this group: -

### Batch

- Operations: `BatchGetTokenBalance`
- Traits: `readonly` (1)
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `BatchGetTokenBalance` | `POST /batch-get-token-balance` | `readonly` | - | - | `BatchGetTokenBalanceOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Gets the token balance for a batch of tokens by using the BatchGetTokenBalance action for every token in the request. Only the native tokens BTC and ETH, and the ERC-20, ERC-721, and ERC 1155 token standards are supp ... |
| `GetAssetContract` | `POST /get-asset-contract` | `readonly` | `contractIdentifier` | - | `GetAssetContractOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Gets the information about a specific contract deployed on the blockchain. The Bitcoin blockchain networks do not support this operation. Metadata is currently only available for some ERC-20 contracts. Metadata will ... |
| `GetTokenBalance` | `POST /get-token-balance` | `readonly` | `tokenIdentifier`, `ownerIdentifier` | - | `GetTokenBalanceOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Gets the balance of a specific token, including native tokens, for a given address (wallet or contract) on the blockchain. Only the native tokens BTC and ETH, and the ERC-20, ERC-721, and ERC 1155 token standards are ... |
| `GetTransaction` | `POST /get-transaction` | `readonly` | `network` | - | `GetTransactionOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Gets the details of a transaction. This action will return transaction details for all transactions that are confirmed on the blockchain, even if they have not reached finality . |
| `ListAssetContracts` | `POST /list-asset-contracts` | `readonly`, `paginated` | `contractFilter` | - | `ListAssetContractsOutput` | `AccessDeniedException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Lists all the contracts for a given contract type deployed by an address (either a contract address or a wallet address). The Bitcoin blockchain networks do not support this operation. |
| `ListFilteredTransactionEvents` | `POST /list-filtered-transaction-events` | `readonly`, `paginated` | `network`, `addressIdentifierFilter` | - | `ListFilteredTransactionEventsOutput` | `AccessDeniedException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Lists all the transaction events for an address on the blockchain. This operation is only supported on the Bitcoin networks. |
| `ListTokenBalances` | `POST /list-token-balances` | `readonly`, `paginated` | `tokenFilter` | - | `ListTokenBalancesOutput` | `AccessDeniedException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | This action returns the following for a given blockchain network: Lists all token balances owned by an address (either a contract address or a wallet address). Lists all token balances for all tokens created by a con ... |
| `ListTransactionEvents` | `POST /list-transaction-events` | `readonly`, `paginated` | `network` | - | `ListTransactionEventsOutput` | `AccessDeniedException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Lists all the transaction events for a transaction This action will return transaction details for all transactions that are confirmed on the blockchain, even if they have not reached finality . |
| `ListTransactions` | `POST /list-transactions` | `readonly`, `paginated` | `address`, `network` | - | `ListTransactionsOutput` | `AccessDeniedException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Lists all the transaction events for a transaction. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | message | The Amazon Web Services account doesn’t have access to this resource. |
| `InternalServerException` | `structure` | message, retryAfterSeconds | The request processing has failed because of an internal error in the service. |
| `ResourceNotFoundException` | `structure` | message, resourceId, resourceType | The resource was not found. |
| `ServiceQuotaExceededException` | `structure` | message, resourceId, resourceType, serviceCode, quotaCode | The service quota has been exceeded for this resource. |
| `ThrottlingException` | `structure` | message, serviceCode, quotaCode, retryAfterSeconds | The request or operation couldn't be performed because a service is throttling requests. The most common source of throttling errors is when you create reso ... |
| `ValidationException` | `structure` | message, reason, fieldList | The resource passed is invalid. |
| `BatchGetTokenBalanceInput` | `structure` | getTokenBalanceInputs | - |
| `BatchGetTokenBalanceOutput` | `structure` | tokenBalances, errors | - |
| `GetAssetContractInput` | `structure` | contractIdentifier | - |
| `GetAssetContractOutput` | `structure` | contractIdentifier, tokenStandard, deployerAddress, metadata | - |
| `GetTokenBalanceInput` | `structure` | tokenIdentifier, ownerIdentifier, atBlockchainInstant | - |
| `GetTokenBalanceOutput` | `structure` | ownerIdentifier, tokenIdentifier, balance, atBlockchainInstant, lastUpdatedTime | - |
| `GetTransactionInput` | `structure` | transactionHash, transactionId, network | - |
| `GetTransactionOutput` | `structure` | transaction | - |
| `ListAssetContractsInput` | `structure` | contractFilter, nextToken, maxResults | - |
| `ListAssetContractsOutput` | `structure` | contracts, nextToken | - |
| `ListFilteredTransactionEventsInput` | `structure` | network, addressIdentifierFilter, timeFilter, voutFilter, confirmationStatusFilter, sort, nextToken, maxResults | - |
| `ListFilteredTransactionEventsOutput` | `structure` | events, nextToken | - |
| `ListTokenBalancesInput` | `structure` | ownerFilter, tokenFilter, nextToken, maxResults | - |
| `ListTokenBalancesOutput` | `structure` | tokenBalances, nextToken | - |
| `ListTransactionEventsInput` | `structure` | transactionHash, transactionId, network, nextToken, maxResults | - |
| `ListTransactionEventsOutput` | `structure` | events, nextToken | - |
| `ListTransactionsInput` | `structure` | address, network, fromBlockchainInstant, toBlockchainInstant, sort, nextToken, maxResults, confirmationStatusFilter | - |
| `ListTransactionsOutput` | `structure` | transactions, nextToken | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
