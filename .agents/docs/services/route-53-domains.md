# Amazon Route 53 Domains

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Route 53 API actions let you register domain names and perform related operations.

## Possible Usage Scenarios
- Backported from `crates/winterbaume-route53domains/tests/scenario_test.rs`: manage a domain portfolio with domain listing, contacts, nameservers, tags, and cleanup-style operations.
- Backported from `scenario_test.rs`: register a domain with custom settings and verify the resulting domain metadata.
- Scenario insight from EC2: include mutable binding failover for Amazon Route 53 Domains where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: exercise account or service defaults for Amazon Route 53 Domains by toggling configuration, creating later resources without explicit overrides, and confirming the default propagates into those resources.
- Scenario insight from EC2: cover association replacement for Amazon Route 53 Domains by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- From the AWS documentation and model: model domain registration, renewal, transfer, contact privacy, nameserver updates, operation tracking, billing/price checks, and domain tag management.

## Service Identity and Protocol

- AWS model slug: `route-53-domains`
- AWS SDK for Rust slug: `route53domains`
- Model version: `2014-05-15`
- Model file: `vendor/api-models-aws/models/route-53-domains/service/2014-05-15/route-53-domains-2014-05-15.json`
- SDK ID: `Route 53 Domains`
- Endpoint prefix: `route53domains`
- ARN namespace: `route53domains`
- CloudFormation name: `Route53Domains`
- CloudTrail event source: `route53domains.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (4), `List` (4), `Update` (4), `Check` (2), `Delete` (2), `Disable` (2), `Enable` (2), `Resend` (2).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AcceptDomainTransferFromAnotherAwsAccount`, `AssociateDelegationSignerToDomain`, `CancelDomainTransferToAnotherAwsAccount`, `DeleteDomain`, `DeleteTagsForDomain`, `DisableDomainAutoRenew`, `DisableDomainTransferLock`, `DisassociateDelegationSignerFromDomain`, `EnableDomainAutoRenew`, `EnableDomainTransferLock`, `RegisterDomain`, `RejectDomainTransferFromAnotherAwsAccount`, `UpdateDomainContact`, `UpdateDomainContactPrivacy`, `UpdateDomainNameservers`, `UpdateTagsForDomain`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `CheckDomainAvailability`, `CheckDomainTransferability`, `GetContactReachabilityStatus`, `GetDomainDetail`, `GetDomainSuggestions`, `GetOperationDetail`, `ListDomains`, `ListOperations`, `ListPrices`, `ListTagsForDomain`.
- Pagination is modelled for 4 operations; token stability and page boundaries are observable API behaviour.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CancelDomainTransferToAnotherAwsAccount`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 34 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Operation Groups

### Get

- Operations: `GetContactReachabilityStatus`, `GetDomainDetail`, `GetDomainSuggestions`, `GetOperationDetail`
- Common required input members in this group: `DomainName`, `OnlyAvailable`, `OperationId`, `SuggestionCount`

### List

- Operations: `ListDomains`, `ListOperations`, `ListPrices`, `ListTagsForDomain`
- Traits: `paginated` (3)
- Common required input members in this group: `DomainName`

### Update

- Operations: `UpdateDomainContact`, `UpdateDomainContactPrivacy`, `UpdateDomainNameservers`, `UpdateTagsForDomain`
- Common required input members in this group: `DomainName`, `Nameservers`

### Check

- Operations: `CheckDomainAvailability`, `CheckDomainTransferability`
- Common required input members in this group: `DomainName`

### Delete

- Operations: `DeleteDomain`, `DeleteTagsForDomain`
- Common required input members in this group: `DomainName`, `TagsToDelete`

### Disable

- Operations: `DisableDomainAutoRenew`, `DisableDomainTransferLock`
- Common required input members in this group: `DomainName`

### Enable

- Operations: `EnableDomainAutoRenew`, `EnableDomainTransferLock`
- Common required input members in this group: `DomainName`

### Resend

- Operations: `ResendContactReachabilityEmail`, `ResendOperationAuthorization`
- Common required input members in this group: `OperationId`

### Transfer

- Operations: `TransferDomain`, `TransferDomainToAnotherAwsAccount`
- Common required input members in this group: `AccountId`, `AdminContact`, `DomainName`, `DurationInYears`, `RegistrantContact`, `TechContact`

### Accept

- Operations: `AcceptDomainTransferFromAnotherAwsAccount`
- Common required input members in this group: `DomainName`, `Password`

### Associate

- Operations: `AssociateDelegationSignerToDomain`
- Common required input members in this group: `DomainName`, `SigningAttributes`

### Cancel

- Operations: `CancelDomainTransferToAnotherAwsAccount`
- Common required input members in this group: `DomainName`

### Disassociate

- Operations: `DisassociateDelegationSignerFromDomain`
- Common required input members in this group: `DomainName`, `Id`

### Push

- Operations: `PushDomain`
- Common required input members in this group: `DomainName`, `Target`

### Register

- Operations: `RegisterDomain`
- Common required input members in this group: `AdminContact`, `DomainName`, `DurationInYears`, `RegistrantContact`, `TechContact`

### Reject

- Operations: `RejectDomainTransferFromAnotherAwsAccount`
- Common required input members in this group: `DomainName`

### Renew

- Operations: `RenewDomain`
- Common required input members in this group: `CurrentExpiryYear`, `DomainName`

### Retrieve

- Operations: `RetrieveDomainAuthCode`
- Common required input members in this group: `DomainName`

### View

- Operations: `ViewBilling`
- Traits: `paginated` (1)

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AcceptDomainTransferFromAnotherAwsAccount` | - | - | `DomainName`, `Password` | - | `AcceptDomainTransferFromAnotherAwsAccountResponse` | `DomainLimitExceeded`, `InvalidInput`, `OperationLimitExceeded`, `UnsupportedTLD` | Accepts the transfer of a domain from another Amazon Web Services account to the currentAmazon Web Services account. You initiate a transfer between Amazon Web Services accounts using TransferDomainToAnotherAwsAccount. |
| `AssociateDelegationSignerToDomain` | - | - | `DomainName`, `SigningAttributes` | - | `AssociateDelegationSignerToDomainResponse` | `DnssecLimitExceeded`, `DuplicateRequest`, `InvalidInput`, `OperationLimitExceeded`, `TLDRulesViolation`, `UnsupportedTLD` | Creates a delegation signer (DS) record in the registry zone for this domain name. Note that creating DS record at the registry impacts DNSSEC validation of your DNS records. |
| `CancelDomainTransferToAnotherAwsAccount` | - | - | `DomainName` | - | `CancelDomainTransferToAnotherAwsAccountResponse` | `InvalidInput`, `OperationLimitExceeded`, `UnsupportedTLD` | Cancels the transfer of a domain from the current Amazon Web Services account to another Amazon Web Services account. You initiate a transfer betweenAmazon Web Services accounts using TransferDomainToAnotherAwsAccount. |
| `CheckDomainAvailability` | - | - | `DomainName` | - | `CheckDomainAvailabilityResponse` | `InvalidInput`, `UnsupportedTLD` | This operation checks the availability of one domain name. Note that if the availability status of a domain is pending, you must submit another request to determine the availability of the domain name. |
| `CheckDomainTransferability` | - | - | `DomainName` | - | `CheckDomainTransferabilityResponse` | `InvalidInput`, `UnsupportedTLD` | Checks whether a domain name can be transferred to Amazon Route 53. |
| `DeleteDomain` | - | - | `DomainName` | - | `DeleteDomainResponse` | `DuplicateRequest`, `InvalidInput`, `TLDRulesViolation`, `UnsupportedTLD` | This operation deletes the specified domain. This action is permanent. |
| `DeleteTagsForDomain` | - | - | `DomainName`, `TagsToDelete` | - | `DeleteTagsForDomainResponse` | `InvalidInput`, `OperationLimitExceeded`, `UnsupportedTLD` | This operation deletes the specified tags for a domain. All tag operations are eventually consistent; subsequent operations might not immediately represent all issued operations. |
| `DisableDomainAutoRenew` | - | - | `DomainName` | - | `DisableDomainAutoRenewResponse` | `InvalidInput`, `UnsupportedTLD` | This operation disables automatic renewal of domain registration for the specified domain. |
| `DisableDomainTransferLock` | - | - | `DomainName` | - | `DisableDomainTransferLockResponse` | `DuplicateRequest`, `InvalidInput`, `OperationLimitExceeded`, `TLDRulesViolation`, `UnsupportedTLD` | This operation removes the transfer lock on the domain (specifically the `clientTransferProhibited` status) to allow domain transfers. We recommend you refrain from performing this action unless you intend to transfer the domain to a different registrar. |
| `DisassociateDelegationSignerFromDomain` | - | - | `DomainName`, `Id` | - | `DisassociateDelegationSignerFromDomainResponse` | `DuplicateRequest`, `InvalidInput`, `OperationLimitExceeded`, `TLDRulesViolation`, `UnsupportedTLD` | Deletes a delegation signer (DS) record in the registry zone for this domain name. |
| `EnableDomainAutoRenew` | - | - | `DomainName` | - | `EnableDomainAutoRenewResponse` | `InvalidInput`, `TLDRulesViolation`, `UnsupportedTLD` | This operation configures Amazon Route 53 to automatically renew the specified domain before the domain registration expires. The cost of renewing your domain registration is billed to your Amazon Web Services account. |
| `EnableDomainTransferLock` | - | - | `DomainName` | - | `EnableDomainTransferLockResponse` | `DuplicateRequest`, `InvalidInput`, `OperationLimitExceeded`, `TLDRulesViolation`, `UnsupportedTLD` | This operation sets the transfer lock on the domain (specifically the `clientTransferProhibited` status) to prevent domain transfers. Successful submission returns an operation ID that you can use to track the progress and completion of the action. |
| `GetContactReachabilityStatus` | - | - | - | - | `GetContactReachabilityStatusResponse` | `InvalidInput`, `OperationLimitExceeded`, `UnsupportedTLD` | For operations that require confirmation that the email address for the registrant contact is valid, such as registering a new domain, this operation returns information about whether the registrant contact has responded. If you want us to resend the email... |
| `GetDomainDetail` | - | - | `DomainName` | - | `GetDomainDetailResponse` | `InvalidInput`, `UnsupportedTLD` | This operation returns detailed information about a specified domain that is associated with the current Amazon Web Services account. Contact information for the domain is also returned as part of the output. |
| `GetDomainSuggestions` | - | - | `DomainName`, `OnlyAvailable`, `SuggestionCount` | - | `GetDomainSuggestionsResponse` | `InvalidInput`, `UnsupportedTLD` | The GetDomainSuggestions operation returns a list of suggested domain names. |
| `GetOperationDetail` | - | - | `OperationId` | - | `GetOperationDetailResponse` | `InvalidInput` | This operation returns the current status of an operation that is not completed. |
| `ListDomains` | - | `paginated` | - | - | `ListDomainsResponse` | `InvalidInput` | This operation returns all the domain names registered with Amazon Route 53 for the current Amazon Web Services account if no filtering conditions are used. |
| `ListOperations` | - | `paginated` | - | - | `ListOperationsResponse` | `InvalidInput` | Returns information about all of the operations that return an operation ID and that have ever been performed on domains that were registered by the current account. This command runs only in the us-east-1 Region. |
| `ListPrices` | - | `paginated` | - | - | `ListPricesResponse` | `InvalidInput`, `UnsupportedTLD` | Lists the following prices for either all the TLDs supported by Route 53, or the specified TLD: Registration Transfer Owner change Domain renewal Domain restoration |
| `ListTagsForDomain` | - | - | `DomainName` | - | `ListTagsForDomainResponse` | `InvalidInput`, `OperationLimitExceeded`, `UnsupportedTLD` | This operation returns all of the tags that are associated with the specified domain. All tag operations are eventually consistent; subsequent operations might not immediately represent all issued operations. |
| `PushDomain` | - | - | `DomainName`, `Target` | - | `Unit` | `InvalidInput`, `OperationLimitExceeded`, `UnsupportedTLD` | Moves a domain from Amazon Web Services to another registrar. Supported actions: Changes the IPS tags of a .uk domain, and pushes it to transit. |
| `RegisterDomain` | - | - | `AdminContact`, `DomainName`, `DurationInYears`, `RegistrantContact`, `TechContact` | - | `RegisterDomainResponse` | `DomainLimitExceeded`, `DuplicateRequest`, `InvalidInput`, `OperationLimitExceeded`, `TLDRulesViolation`, `UnsupportedTLD` | This operation registers a domain. For some top-level domains (TLDs), this operation requires extra parameters. |
| `RejectDomainTransferFromAnotherAwsAccount` | - | - | `DomainName` | - | `RejectDomainTransferFromAnotherAwsAccountResponse` | `InvalidInput`, `OperationLimitExceeded`, `UnsupportedTLD` | Rejects the transfer of a domain from another Amazon Web Services account to the current Amazon Web Services account. You initiate a transfer betweenAmazon Web Services accounts using TransferDomainToAnotherAwsAccount. |
| `RenewDomain` | - | - | `CurrentExpiryYear`, `DomainName` | - | `RenewDomainResponse` | `DuplicateRequest`, `InvalidInput`, `OperationLimitExceeded`, `TLDRulesViolation`, `UnsupportedTLD` | This operation renews a domain for the specified number of years. The cost of renewing your domain is billed to your Amazon Web Services account. |
| `ResendContactReachabilityEmail` | - | - | - | - | `ResendContactReachabilityEmailResponse` | `InvalidInput`, `OperationLimitExceeded`, `UnsupportedTLD` | For operations that require confirmation that the email address for the registrant contact is valid, such as registering a new domain, this operation resends the confirmation email to the current email address for the registrant contact. |
| `ResendOperationAuthorization` | - | - | `OperationId` | - | `Unit` | `InvalidInput` | Resend the form of authorization email for this operation. |
| `RetrieveDomainAuthCode` | - | - | `DomainName` | - | `RetrieveDomainAuthCodeResponse` | `InvalidInput`, `UnsupportedTLD` | This operation returns the authorization code for the domain. To transfer a domain to another registrar, you provide this value to the new registrar. |
| `TransferDomain` | - | - | `AdminContact`, `DomainName`, `DurationInYears`, `RegistrantContact`, `TechContact` | - | `TransferDomainResponse` | `DomainLimitExceeded`, `DuplicateRequest`, `InvalidInput`, `OperationLimitExceeded`, `TLDRulesViolation`, `UnsupportedTLD` | Transfers a domain from another registrar to Amazon Route 53. For more information about transferring domains, see the following topics: For transfer requirements, a detailed procedure, and information about viewing the status of a domain that you're... |
| `TransferDomainToAnotherAwsAccount` | - | - | `AccountId`, `DomainName` | - | `TransferDomainToAnotherAwsAccountResponse` | `DuplicateRequest`, `InvalidInput`, `OperationLimitExceeded`, `UnsupportedTLD` | Transfers a domain from the current Amazon Web Services account to another Amazon Web Services account. Note the following: The Amazon Web Services account that you're transferring the domain to must accept the transfer. |
| `UpdateDomainContact` | - | - | `DomainName` | - | `UpdateDomainContactResponse` | `DuplicateRequest`, `InvalidInput`, `OperationLimitExceeded`, `TLDRulesViolation`, `UnsupportedTLD` | This operation updates the contact information for a particular domain. You must specify information for at least one contact: registrant, administrator, or technical. |
| `UpdateDomainContactPrivacy` | - | - | `DomainName` | - | `UpdateDomainContactPrivacyResponse` | `DuplicateRequest`, `InvalidInput`, `OperationLimitExceeded`, `TLDRulesViolation`, `UnsupportedTLD` | This operation updates the specified domain contact's privacy setting. When privacy protection is enabled, your contact information is replaced with contact information for the registrar or with the phrase "REDACTED FOR PRIVACY", or "On behalf of owner."... |
| `UpdateDomainNameservers` | - | - | `DomainName`, `Nameservers` | - | `UpdateDomainNameserversResponse` | `DuplicateRequest`, `InvalidInput`, `OperationLimitExceeded`, `TLDRulesViolation`, `UnsupportedTLD` | This operation replaces the current set of name servers for the domain with the specified set of name servers. If you use Amazon Route 53 as your DNS service, specify the four name servers in the delegation set for the hosted zone for the domain. |
| `UpdateTagsForDomain` | - | - | `DomainName` | - | `UpdateTagsForDomainResponse` | `InvalidInput`, `OperationLimitExceeded`, `UnsupportedTLD` | This operation adds or updates tags for a specified domain. All tag operations are eventually consistent; subsequent operations might not immediately represent all issued operations. |
| `ViewBilling` | - | `paginated` | - | - | `ViewBillingResponse` | `InvalidInput` | Returns all the domain-related billing records for the current Amazon Web Services account for a specified period |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InvalidInput` | `structure` | `message` | The requested item is not acceptable. |
| `UnsupportedTLD` | `structure` | `message` | Amazon Route 53 does not support this top-level domain (TLD). |
| `OperationLimitExceeded` | `structure` | `message` | The number of operations or jobs running exceeded the allowed threshold for the account. |
| `DuplicateRequest` | `structure` | `message`, `requestId` | The request is already in progress for the domain. |
| `TLDRulesViolation` | `structure` | `message` | The top-level domain does not support this operation. |
| `DomainLimitExceeded` | `structure` | `message` | The number of domains has exceeded the allowed threshold for the account. |
| `AcceptDomainTransferFromAnotherAwsAccountRequest` | `structure` | `DomainName`, `Password` | The AcceptDomainTransferFromAnotherAwsAccount request includes the following elements. |
| `AcceptDomainTransferFromAnotherAwsAccountResponse` | `structure` | `OperationId` | The AcceptDomainTransferFromAnotherAwsAccount response includes the following element. |
| `AssociateDelegationSignerToDomainRequest` | `structure` | `DomainName`, `SigningAttributes` | - |
| `AssociateDelegationSignerToDomainResponse` | `structure` | `OperationId` | - |
| `DnssecLimitExceeded` | `structure` | `message` | This error is returned if you call `AssociateDelegationSignerToDomain` when the specified domain has reached the maximum number of DS records. |
| `CancelDomainTransferToAnotherAwsAccountRequest` | `structure` | `DomainName` | The CancelDomainTransferToAnotherAwsAccount request includes the following element. |
| `CancelDomainTransferToAnotherAwsAccountResponse` | `structure` | `OperationId` | The `CancelDomainTransferToAnotherAwsAccount` response includes the following element. |
| `CheckDomainAvailabilityRequest` | `structure` | `DomainName`, `IdnLangCode` | The CheckDomainAvailability request contains the following elements. |
| `CheckDomainAvailabilityResponse` | `structure` | `Availability` | The CheckDomainAvailability response includes the following elements. |
| `CheckDomainTransferabilityRequest` | `structure` | `AuthCode`, `DomainName` | The CheckDomainTransferability request contains the following elements. |
| `CheckDomainTransferabilityResponse` | `structure` | `Message`, `Transferability` | The CheckDomainTransferability response includes the following elements. |
| `DeleteDomainRequest` | `structure` | `DomainName` | - |
| `DeleteDomainResponse` | `structure` | `OperationId` | - |
| `DeleteTagsForDomainRequest` | `structure` | `DomainName`, `TagsToDelete` | The DeleteTagsForDomainRequest includes the following elements. |
| `DeleteTagsForDomainResponse` | `structure` | - | - |
| `DisableDomainAutoRenewRequest` | `structure` | `DomainName` | - |
| `DisableDomainAutoRenewResponse` | `structure` | - | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
