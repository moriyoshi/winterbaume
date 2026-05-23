# Amazon Mechanical Turk

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Mechanical Turk API Reference

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for Amazon Mechanical Turk where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: cover association replacement for Amazon Mechanical Turk by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- From the AWS documentation and model: represent documented Amazon Mechanical Turk workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Create`, `Get`, `Update`, `Delete` operation families, including `ListAssignmentsForHIT`, `ListBonusPayments`, `ListHITs`, `ListHITsForQualificationType`, `CreateAdditionalAssignmentsForHIT`, `CreateHIT`.

## Service Identity and Protocol

- AWS model slug: `mturk`
- AWS SDK for Rust slug: `mturk`
- Model version: `2017-01-17`
- Model file: `vendor/api-models-aws/models/mturk/service/2017-01-17/mturk-2017-01-17.json`
- SDK ID: `MTurk`
- Endpoint prefix: `mturk-requester`
- ARN namespace: `mturk-requester`
- CloudFormation name: `MTurk`
- CloudTrail event source: `mturk.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (10), `Create` (6), `Get` (6), `Update` (5), `Delete` (3), `Reject` (2), `Send` (2), `Accept` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AcceptQualificationRequest`, `AssociateQualificationWithWorker`, `CreateAdditionalAssignmentsForHIT`, `CreateHIT`, `CreateHITType`, `CreateHITWithHITType`, `CreateQualificationType`, `CreateWorkerBlock`, `DeleteHIT`, `DeleteQualificationType`, `DeleteWorkerBlock`, `DisassociateQualificationFromWorker`, `RejectAssignment`, `RejectQualificationRequest`, `UpdateExpirationForHIT`, `UpdateHITReviewStatus`, `UpdateHITTypeOfHIT`, `UpdateNotificationSettings`, `UpdateQualificationType`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetAccountBalance`, `GetAssignment`, `GetFileUploadURL`, `GetHIT`, `GetQualificationScore`, `GetQualificationType`, `ListAssignmentsForHIT`, `ListBonusPayments`, `ListHITs`, `ListHITsForQualificationType`, `ListQualificationRequests`, `ListQualificationTypes`, `ListReviewPolicyResultsForHIT`, `ListReviewableHITs`, `ListWorkerBlocks`, `ListWorkersWithQualificationType`.
- Pagination is modelled for 10 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 26 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- 39 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `ECS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Operation Groups

### List

- Operations: `ListAssignmentsForHIT`, `ListBonusPayments`, `ListHITs`, `ListHITsForQualificationType`, `ListQualificationRequests`, `ListQualificationTypes`, `ListReviewableHITs`, `ListReviewPolicyResultsForHIT`, `ListWorkerBlocks`, `ListWorkersWithQualificationType`
- Traits: `idempotent` (10), `paginated` (10)
- Common required input members in this group: `HITId`, `QualificationTypeId`

### Create

- Operations: `CreateAdditionalAssignmentsForHIT`, `CreateHIT`, `CreateHITType`, `CreateHITWithHITType`, `CreateQualificationType`, `CreateWorkerBlock`
- Traits: `idempotent` (1)
- Common required input members in this group: `LifetimeInSeconds`, `AssignmentDurationInSeconds`, `Reward`, `Title`, `Description`

### Get

- Operations: `GetAccountBalance`, `GetAssignment`, `GetFileUploadURL`, `GetHIT`, `GetQualificationScore`, `GetQualificationType`
- Traits: `idempotent` (6)
- Common required input members in this group: `AssignmentId`, `QualificationTypeId`

### Update

- Operations: `UpdateExpirationForHIT`, `UpdateHITReviewStatus`, `UpdateHITTypeOfHIT`, `UpdateNotificationSettings`, `UpdateQualificationType`
- Traits: `idempotent` (4)
- Common required input members in this group: `HITId`, `HITTypeId`

### Delete

- Operations: `DeleteHIT`, `DeleteQualificationType`, `DeleteWorkerBlock`
- Traits: `idempotent` (3)
- Common required input members in this group: -

### Reject

- Operations: `RejectAssignment`, `RejectQualificationRequest`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Send

- Operations: `SendBonus`, `SendTestEventNotification`
- Common required input members in this group: -

### Accept

- Operations: `AcceptQualificationRequest`
- Common required input members in this group: -

### Approve

- Operations: `ApproveAssignment`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Associate

- Operations: `AssociateQualificationWithWorker`
- Common required input members in this group: -

### Disassociate

- Operations: `DisassociateQualificationFromWorker`
- Common required input members in this group: -

### Notify

- Operations: `NotifyWorkers`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AcceptQualificationRequest` | `-` | - | `QualificationRequestId` | - | `AcceptQualificationRequestResponse` | `RequestError`, `ServiceFault` | The AcceptQualificationRequest operation approves a Worker's request for a Qualification. Only the owner of the Qualification type can grant a Qualification request for that type. A successful request for the AcceptQ ... |
| `ApproveAssignment` | `-` | `idempotent` | `AssignmentId` | - | `ApproveAssignmentResponse` | `RequestError`, `ServiceFault` | The ApproveAssignment operation approves the results of a completed assignment. Approving an assignment initiates two payments from the Requester's Amazon.com account The Worker who submitted the results is paid the ... |
| `AssociateQualificationWithWorker` | `-` | - | `QualificationTypeId`, `WorkerId` | - | `AssociateQualificationWithWorkerResponse` | `RequestError`, `ServiceFault` | The AssociateQualificationWithWorker operation gives a Worker a Qualification. AssociateQualificationWithWorker does not require that the Worker submit a Qualification request. It gives the Qualification directly to ... |
| `CreateAdditionalAssignmentsForHIT` | `-` | - | `HITId`, `NumberOfAdditionalAssignments` | - | `CreateAdditionalAssignmentsForHITResponse` | `RequestError`, `ServiceFault` | The CreateAdditionalAssignmentsForHIT operation increases the maximum number of assignments of an existing HIT. To extend the maximum number of assignments, specify the number of additional assignments. HITs created ... |
| `CreateHIT` | `-` | - | `LifetimeInSeconds`, `AssignmentDurationInSeconds`, `Reward`, `Title`, `Description` | - | `CreateHITResponse` | `RequestError`, `ServiceFault` | The CreateHIT operation creates a new Human Intelligence Task (HIT). The new HIT is made available for Workers to find and accept on the Amazon Mechanical Turk website. This operation allows you to specify a new HIT ... |
| `CreateHITType` | `-` | `idempotent` | `AssignmentDurationInSeconds`, `Reward`, `Title`, `Description` | - | `CreateHITTypeResponse` | `RequestError`, `ServiceFault` | The CreateHITType operation creates a new HIT type. This operation allows you to define a standard set of HIT properties to use when creating HITs. If you register a HIT type with values that match an existing HIT ty ... |
| `CreateHITWithHITType` | `-` | - | `HITTypeId`, `LifetimeInSeconds` | - | `CreateHITWithHITTypeResponse` | `RequestError`, `ServiceFault` | The CreateHITWithHITType operation creates a new Human Intelligence Task (HIT) using an existing HITTypeID generated by the CreateHITType operation. This is an alternative way to create HITs from the CreateHIT operat ... |
| `CreateQualificationType` | `-` | - | `Name`, `Description`, `QualificationTypeStatus` | - | `CreateQualificationTypeResponse` | `RequestError`, `ServiceFault` | The CreateQualificationType operation creates a new Qualification type, which is represented by a QualificationType data structure. |
| `CreateWorkerBlock` | `-` | - | `WorkerId`, `Reason` | - | `CreateWorkerBlockResponse` | `RequestError`, `ServiceFault` | The CreateWorkerBlock operation allows you to prevent a Worker from working on your HITs. For example, you can block a Worker who is producing poor quality work. You can block up to 100,000 Workers. |
| `DeleteHIT` | `-` | `idempotent` | `HITId` | - | `DeleteHITResponse` | `RequestError`, `ServiceFault` | The DeleteHIT operation is used to delete HIT that is no longer needed. Only the Requester who created the HIT can delete it. You can only dispose of HITs that are in the Reviewable state, with all of their submitted ... |
| `DeleteQualificationType` | `-` | `idempotent` | `QualificationTypeId` | - | `DeleteQualificationTypeResponse` | `RequestError`, `ServiceFault` | The DeleteQualificationType deletes a Qualification type and deletes any HIT types that are associated with the Qualification type. This operation does not revoke Qualifications already assigned to Workers because th ... |
| `DeleteWorkerBlock` | `-` | `idempotent` | `WorkerId` | - | `DeleteWorkerBlockResponse` | `RequestError`, `ServiceFault` | The DeleteWorkerBlock operation allows you to reinstate a blocked Worker to work on your HITs. This operation reverses the effects of the CreateWorkerBlock operation. You need the Worker ID to use this operation. If ... |
| `DisassociateQualificationFromWorker` | `-` | - | `WorkerId`, `QualificationTypeId` | - | `DisassociateQualificationFromWorkerResponse` | `RequestError`, `ServiceFault` | The DisassociateQualificationFromWorker revokes a previously granted Qualification from a user. You can provide a text message explaining why the Qualification was revoked. The user who had the Qualification can see ... |
| `GetAccountBalance` | `-` | `idempotent` | - | - | `GetAccountBalanceResponse` | `RequestError`, `ServiceFault` | The GetAccountBalance operation retrieves the Prepaid HITs balance in your Amazon Mechanical Turk account if you are a Prepaid Requester. Alternatively, this operation will retrieve the remaining available AWS Billin ... |
| `GetAssignment` | `-` | `idempotent` | `AssignmentId` | - | `GetAssignmentResponse` | `RequestError`, `ServiceFault` | The GetAssignment operation retrieves the details of the specified Assignment. |
| `GetFileUploadURL` | `-` | `idempotent` | `AssignmentId`, `QuestionIdentifier` | - | `GetFileUploadURLResponse` | `RequestError`, `ServiceFault` | The GetFileUploadURL operation generates and returns a temporary URL. You use the temporary URL to retrieve a file uploaded by a Worker as an answer to a FileUploadAnswer question for a HIT. The temporary URL is gene ... |
| `GetHIT` | `-` | `idempotent` | `HITId` | - | `GetHITResponse` | `RequestError`, `ServiceFault` | The GetHIT operation retrieves the details of the specified HIT. |
| `GetQualificationScore` | `-` | `idempotent` | `QualificationTypeId`, `WorkerId` | - | `GetQualificationScoreResponse` | `RequestError`, `ServiceFault` | The GetQualificationScore operation returns the value of a Worker's Qualification for a given Qualification type. To get a Worker's Qualification, you must know the Worker's ID. The Worker's ID is included in the ass ... |
| `GetQualificationType` | `-` | `idempotent` | `QualificationTypeId` | - | `GetQualificationTypeResponse` | `RequestError`, `ServiceFault` | The GetQualificationType operation retrieves information about a Qualification type using its ID. |
| `ListAssignmentsForHIT` | `-` | `idempotent`, `paginated` | `HITId` | - | `ListAssignmentsForHITResponse` | `RequestError`, `ServiceFault` | The ListAssignmentsForHIT operation retrieves completed assignments for a HIT. You can use this operation to retrieve the results for a HIT. You can get assignments for a HIT at any time, even if the HIT is not yet R ... |
| `ListBonusPayments` | `-` | `idempotent`, `paginated` | - | - | `ListBonusPaymentsResponse` | `RequestError`, `ServiceFault` | The ListBonusPayments operation retrieves the amounts of bonuses you have paid to Workers for a given HIT or assignment. |
| `ListHITs` | `-` | `idempotent`, `paginated` | - | - | `ListHITsResponse` | `RequestError`, `ServiceFault` | The ListHITs operation returns all of a Requester's HITs. The operation returns HITs of any status, except for HITs that have been deleted of with the DeleteHIT operation or that have been auto-deleted. |
| `ListHITsForQualificationType` | `-` | `idempotent`, `paginated` | `QualificationTypeId` | - | `ListHITsForQualificationTypeResponse` | `RequestError`, `ServiceFault` | The ListHITsForQualificationType operation returns the HITs that use the given Qualification type for a Qualification requirement. The operation returns HITs of any status, except for HITs that have been deleted with ... |
| `ListQualificationRequests` | `-` | `idempotent`, `paginated` | - | - | `ListQualificationRequestsResponse` | `RequestError`, `ServiceFault` | The ListQualificationRequests operation retrieves requests for Qualifications of a particular Qualification type. The owner of the Qualification type calls this operation to poll for pending requests, and accepts the ... |
| `ListQualificationTypes` | `-` | `idempotent`, `paginated` | `MustBeRequestable` | - | `ListQualificationTypesResponse` | `RequestError`, `ServiceFault` | The ListQualificationTypes operation returns a list of Qualification types, filtered by an optional search term. |
| `ListReviewableHITs` | `-` | `idempotent`, `paginated` | - | - | `ListReviewableHITsResponse` | `RequestError`, `ServiceFault` | The ListReviewableHITs operation retrieves the HITs with Status equal to Reviewable or Status equal to Reviewing that belong to the Requester calling the operation. |
| `ListReviewPolicyResultsForHIT` | `-` | `idempotent`, `paginated` | `HITId` | - | `ListReviewPolicyResultsForHITResponse` | `RequestError`, `ServiceFault` | The ListReviewPolicyResultsForHIT operation retrieves the computed results and the actions taken in the course of executing your Review Policies for a given HIT. For information about how to specify Review Policies w ... |
| `ListWorkerBlocks` | `-` | `idempotent`, `paginated` | - | - | `ListWorkerBlocksResponse` | `RequestError`, `ServiceFault` | The ListWorkersBlocks operation retrieves a list of Workers who are blocked from working on your HITs. |
| `ListWorkersWithQualificationType` | `-` | `idempotent`, `paginated` | `QualificationTypeId` | - | `ListWorkersWithQualificationTypeResponse` | `RequestError`, `ServiceFault` | The ListWorkersWithQualificationType operation returns all of the Workers that have been associated with a given Qualification type. |
| `NotifyWorkers` | `-` | - | `Subject`, `MessageText`, `WorkerIds` | - | `NotifyWorkersResponse` | `RequestError`, `ServiceFault` | The NotifyWorkers operation sends an email to one or more Workers that you specify with the Worker ID. You can specify up to 100 Worker IDs to send the same message with a single call to the NotifyWorkers operation. ... |
| `RejectAssignment` | `-` | `idempotent` | `AssignmentId`, `RequesterFeedback` | - | `RejectAssignmentResponse` | `RequestError`, `ServiceFault` | The RejectAssignment operation rejects the results of a completed assignment. You can include an optional feedback message with the rejection, which the Worker can see in the Status section of the web site. When you ... |
| `RejectQualificationRequest` | `-` | - | `QualificationRequestId` | - | `RejectQualificationRequestResponse` | `RequestError`, `ServiceFault` | The RejectQualificationRequest operation rejects a user's request for a Qualification. You can provide a text message explaining why the request was rejected. The Worker who made the request can see this message. |
| `SendBonus` | `-` | - | `WorkerId`, `BonusAmount`, `AssignmentId`, `Reason` | - | `SendBonusResponse` | `RequestError`, `ServiceFault` | The SendBonus operation issues a payment of money from your account to a Worker. This payment happens separately from the reward you pay to the Worker when you approve the Worker's assignment. The SendBonus operation ... |
| `SendTestEventNotification` | `-` | - | `Notification`, `TestEventType` | - | `SendTestEventNotificationResponse` | `RequestError`, `ServiceFault` | The SendTestEventNotification operation causes Amazon Mechanical Turk to send a notification message as if a HIT event occurred, according to the provided notification specification. This allows you to test notificat ... |
| `UpdateExpirationForHIT` | `-` | `idempotent` | `HITId`, `ExpireAt` | - | `UpdateExpirationForHITResponse` | `RequestError`, `ServiceFault` | The UpdateExpirationForHIT operation allows you update the expiration time of a HIT. If you update it to a time in the past, the HIT will be immediately expired. |
| `UpdateHITReviewStatus` | `-` | `idempotent` | `HITId` | - | `UpdateHITReviewStatusResponse` | `RequestError`, `ServiceFault` | The UpdateHITReviewStatus operation updates the status of a HIT. If the status is Reviewable, this operation can update the status to Reviewing, or it can revert a Reviewing HIT back to the Reviewable status. |
| `UpdateHITTypeOfHIT` | `-` | `idempotent` | `HITId`, `HITTypeId` | - | `UpdateHITTypeOfHITResponse` | `RequestError`, `ServiceFault` | The UpdateHITTypeOfHIT operation allows you to change the HITType properties of a HIT. This operation disassociates the HIT from its old HITType properties and associates it with the new HITType properties. The HIT t ... |
| `UpdateNotificationSettings` | `-` | `idempotent` | `HITTypeId` | - | `UpdateNotificationSettingsResponse` | `RequestError`, `ServiceFault` | The UpdateNotificationSettings operation creates, updates, disables or re-enables notifications for a HIT type. If you call the UpdateNotificationSettings operation for a HIT type that already has a notification spec ... |
| `UpdateQualificationType` | `-` | - | `QualificationTypeId` | - | `UpdateQualificationTypeResponse` | `RequestError`, `ServiceFault` | The UpdateQualificationType operation modifies the attributes of an existing Qualification type, which is represented by a QualificationType data structure. Only the owner of a Qualification type can modify its attri ... |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `RequestError` | `structure` | Message, TurkErrorCode | Your request is invalid. |
| `ServiceFault` | `structure` | Message, TurkErrorCode | Amazon Mechanical Turk is temporarily unable to process your request. Try your call again. |
| `AcceptQualificationRequestRequest` | `structure` | QualificationRequestId, IntegerValue | - |
| `AcceptQualificationRequestResponse` | `structure` | **empty (no members)** | - |
| `ApproveAssignmentRequest` | `structure` | AssignmentId, RequesterFeedback, OverrideRejection | - |
| `ApproveAssignmentResponse` | `structure` | **empty (no members)** | - |
| `AssociateQualificationWithWorkerRequest` | `structure` | QualificationTypeId, WorkerId, IntegerValue, SendNotification | - |
| `AssociateQualificationWithWorkerResponse` | `structure` | **empty (no members)** | - |
| `CreateAdditionalAssignmentsForHITRequest` | `structure` | HITId, NumberOfAdditionalAssignments, UniqueRequestToken | - |
| `CreateAdditionalAssignmentsForHITResponse` | `structure` | **empty (no members)** | - |
| `CreateHITRequest` | `structure` | MaxAssignments, AutoApprovalDelayInSeconds, LifetimeInSeconds, AssignmentDurationInSeconds, Reward, Title, Keywords, Description, Question, RequesterAnnotation, QualificationRequirements, UniqueRequestToken, ... (+4) | - |
| `CreateHITResponse` | `structure` | HIT | - |
| `CreateHITTypeRequest` | `structure` | AutoApprovalDelayInSeconds, AssignmentDurationInSeconds, Reward, Title, Keywords, Description, QualificationRequirements | - |
| `CreateHITTypeResponse` | `structure` | HITTypeId | - |
| `CreateHITWithHITTypeRequest` | `structure` | HITTypeId, MaxAssignments, LifetimeInSeconds, Question, RequesterAnnotation, UniqueRequestToken, AssignmentReviewPolicy, HITReviewPolicy, HITLayoutId, HITLayoutParameters | - |
| `CreateHITWithHITTypeResponse` | `structure` | HIT | - |
| `CreateQualificationTypeRequest` | `structure` | Name, Keywords, Description, QualificationTypeStatus, RetryDelayInSeconds, Test, AnswerKey, TestDurationInSeconds, AutoGranted, AutoGrantedValue | - |
| `CreateQualificationTypeResponse` | `structure` | QualificationType | - |
| `CreateWorkerBlockRequest` | `structure` | WorkerId, Reason | - |
| `CreateWorkerBlockResponse` | `structure` | **empty (no members)** | - |
| `DeleteHITRequest` | `structure` | HITId | - |
| `DeleteHITResponse` | `structure` | **empty (no members)** | - |
| `DeleteQualificationTypeRequest` | `structure` | QualificationTypeId | - |
| `DeleteQualificationTypeResponse` | `structure` | **empty (no members)** | - |
| `DeleteWorkerBlockRequest` | `structure` | WorkerId, Reason | - |
| `DeleteWorkerBlockResponse` | `structure` | **empty (no members)** | - |
| `DisassociateQualificationFromWorkerRequest` | `structure` | WorkerId, QualificationTypeId, Reason | - |
| `DisassociateQualificationFromWorkerResponse` | `structure` | **empty (no members)** | - |
| `GetAccountBalanceRequest` | `structure` | **empty (no members)** | - |
| `GetAccountBalanceResponse` | `structure` | AvailableBalance, OnHoldBalance | - |
| `GetAssignmentRequest` | `structure` | AssignmentId | - |
| `GetAssignmentResponse` | `structure` | Assignment, HIT | - |
| `GetFileUploadURLRequest` | `structure` | AssignmentId, QuestionIdentifier | - |
| `GetFileUploadURLResponse` | `structure` | FileUploadURL | - |
| `GetHITRequest` | `structure` | HITId | - |
| `GetHITResponse` | `structure` | HIT | - |
| `GetQualificationScoreRequest` | `structure` | QualificationTypeId, WorkerId | - |
| `GetQualificationScoreResponse` | `structure` | Qualification | - |
| `GetQualificationTypeRequest` | `structure` | QualificationTypeId | - |
| `GetQualificationTypeResponse` | `structure` | QualificationType | - |
| `AssignmentStatus` | `enum` | Submitted, Approved, Rejected | - |
| `Comparator` | `enum` | LessThan, LessThanOrEqualTo, GreaterThan, GreaterThanOrEqualTo, EqualTo, NotEqualTo, Exists, DoesNotExist, In, NotIn | - |
| `EventType` | `enum` | AssignmentAccepted, AssignmentAbandoned, AssignmentReturned, AssignmentSubmitted, AssignmentRejected, AssignmentApproved, HITCreated, HITExpired, HITReviewable, HITExtended, HITDisposed, Ping | - |
| `HITAccessActions` | `enum` | Accept, PreviewAndAccept, DiscoverPreviewAndAccept | - |
| `HITReviewStatus` | `enum` | NotReviewed, MarkedForReview, ReviewedAppropriate, ReviewedInappropriate | - |
| `HITStatus` | `enum` | Assignable, Unassignable, Reviewable, Reviewing, Disposed | - |
| `NotificationTransport` | `enum` | Email, SQS, SNS | - |
| `NotifyWorkersFailureCode` | `enum` | SoftFailure, HardFailure | - |
| `QualificationStatus` | `enum` | Granted, Revoked | - |
| `QualificationTypeStatus` | `enum` | Active, Inactive | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
