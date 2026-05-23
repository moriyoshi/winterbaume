# Amazon CodeGuru Reviewer

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

This section provides documentation for the Amazon CodeGuru Reviewer API operations. CodeGuru Reviewer is a service that uses program analysis and machine learning to detect potential defects that are difficult for developers to find and recommends fixes in your Java and Python code. By proactively detecting and providing recommendations for addressing code defects and implementing best practices, CodeGuru Reviewer improves the overall quality and maintainability of your code base during the code review stage. For more information about CodeGuru Reviewer, see the Amazon CodeGuru Reviewer User Guide. To improve the security of your CodeGuru Reviewer API calls, you can establish a private connection between your VPC and CodeGuru Reviewer by creating an interface VPC endpoint .

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for Amazon CodeGuru Reviewer where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: cover association replacement for Amazon CodeGuru Reviewer by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- From the AWS documentation and model: represent documented Amazon CodeGuru Reviewer workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Describe`, `Associate`, `Create`, `Disassociate` operation families, including `ListCodeReviews`, `ListRecommendationFeedback`, `ListRecommendations`, `ListRepositoryAssociations`, `DescribeCodeReview`, `DescribeRecommendationFeedback`.

## Service Identity and Protocol

- AWS model slug: `codeguru-reviewer`
- AWS SDK for Rust slug: `codegurureviewer`
- Model version: `2019-09-19`
- Model file: `vendor/api-models-aws/models/codeguru-reviewer/service/2019-09-19/codeguru-reviewer-2019-09-19.json`
- SDK ID: `CodeGuru Reviewer`
- Endpoint prefix: `codeguru-reviewer`
- ARN namespace: `codeguru-reviewer`
- CloudFormation name: `CodeGuruReviewer`
- CloudTrail event source: `codeguru-reviewer.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (5), `Describe` (3), `Associate` (1), `Create` (1), `Disassociate` (1), `Put` (1), `Tag` (1), `Untag` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AssociateRepository`, `CreateCodeReview`, `DisassociateRepository`, `PutRecommendationFeedback`, `TagResource`, `UntagResource`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeCodeReview`, `DescribeRecommendationFeedback`, `DescribeRepositoryAssociation`, `ListCodeReviews`, `ListRecommendationFeedback`, `ListRecommendations`, `ListRepositoryAssociations`, `ListTagsForResource`.
- Pagination is modelled for 4 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 2 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 14 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `S3`, `KMS`, `EC2/VPC`, `ECR`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/codeguru/latest/reviewer-ug/welcome.html
- https://docs.aws.amazon.com/codeguru/latest/reviewer-ug/get-results.html
- https://docs.aws.amazon.com/codeguru/latest/reviewer-ug/code-reviews.html

Research outcomes:
- As of November 7, 2025, AWS documentation states that new CodeGuru Reviewer repository associations cannot be created.
- CodeGuru Reviewer analyses Java and Python repositories and produces recommendations for issues such as resource leak prevention, security analysis, and secrets detection.
- Supported source providers include CodeCommit, Bitbucket, GitHub, GitHub Enterprise Cloud, GitHub Enterprise Server, and S3 repositories through GitHub Actions.
- After a repository is associated, CodeGuru Reviewer automatically creates the first full repository analysis.
- Incremental code reviews are automatically created for pull requests after repository association.
- Additional full repository analysis reviews can be requested through console, CLI, or SDK.
- Full repository analysis reviews all code in the branch. Incremental reviews analyse code changed in the pull request.
- GitHub Actions / CI reviews can run for push, pull, or scheduled repository scans and surface results in GitHub Security.
- Recommendations can be viewed in the CodeGuru Reviewer console, through CLI or SDK, and in pull request comments where supported.
- Users can suppress recommendations and provide usefulness feedback.

Parity implications:
- Model repository associations, source provider metadata, full analyses, incremental reviews, CI reviews, recommendations, suppression state, feedback, and encryption configuration separately.
- Repository association creation should reflect the documented availability change for newly created associations where parity requires current AWS behaviour.
- Code review creation should be asynchronous and source-provider dependent, with automatic full and pull-request review side effects.

## Operation Groups

### List

- Operations: `ListCodeReviews`, `ListRecommendationFeedback`, `ListRecommendations`, `ListRepositoryAssociations`, `ListTagsForResource`
- Traits: `paginated` (4)
- Common required input members in this group: `CodeReviewArn`, `Type`, `resourceArn`

### Describe

- Operations: `DescribeCodeReview`, `DescribeRecommendationFeedback`, `DescribeRepositoryAssociation`
- Common required input members in this group: `AssociationArn`, `CodeReviewArn`, `RecommendationId`

### Associate

- Operations: `AssociateRepository`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `Repository`

### Create

- Operations: `CreateCodeReview`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `Name`, `RepositoryAssociationArn`, `Type`

### Disassociate

- Operations: `DisassociateRepository`
- Common required input members in this group: `AssociationArn`

### Put

- Operations: `PutRecommendationFeedback`
- Common required input members in this group: `CodeReviewArn`, `Reactions`, `RecommendationId`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `Tags`, `resourceArn`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `TagKeys`, `resourceArn`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AssociateRepository` | `POST /associations` | `idempotency-token` | `Repository` | `ClientRequestToken` | `AssociateRepositoryResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Use to associate an Amazon Web Services CodeCommit repository or a repository managed by Amazon Web Services CodeStar Connections with Amazon CodeGuru Reviewer. When you associate a repository, CodeGuru Reviewer reviews source code changes in the repository's... |
| `CreateCodeReview` | `POST /codereviews` | `idempotency-token` | `Name`, `RepositoryAssociationArn`, `Type` | `ClientRequestToken` | `CreateCodeReviewResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Use to create a code review with a CodeReviewType of `RepositoryAnalysis`. This type of code review analyzes all code under a specified branch in an associated repository. |
| `DescribeCodeReview` | `GET /codereviews/{CodeReviewArn}` | - | `CodeReviewArn` | - | `DescribeCodeReviewResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns the metadata associated with the code review along with its status. |
| `DescribeRecommendationFeedback` | `GET /feedback/{CodeReviewArn}` | - | `CodeReviewArn`, `RecommendationId` | - | `DescribeRecommendationFeedbackResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Describes the customer feedback for a CodeGuru Reviewer recommendation. |
| `DescribeRepositoryAssociation` | `GET /associations/{AssociationArn}` | - | `AssociationArn` | - | `DescribeRepositoryAssociationResponse` | `AccessDeniedException`, `InternalServerException`, `NotFoundException`, `ThrottlingException`, `ValidationException` | Returns a RepositoryAssociation object that contains information about the requested repository association. |
| `DisassociateRepository` | `DELETE /associations/{AssociationArn}` | - | `AssociationArn` | - | `DisassociateRepositoryResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `NotFoundException`, `ThrottlingException`, `ValidationException` | Removes the association between Amazon CodeGuru Reviewer and a repository. |
| `ListCodeReviews` | `GET /codereviews` | `paginated` | `Type` | - | `ListCodeReviewsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists all the code reviews that the customer has created in the past 90 days. |
| `ListRecommendationFeedback` | `GET /feedback/{CodeReviewArn}/RecommendationFeedback` | `paginated` | `CodeReviewArn` | - | `ListRecommendationFeedbackResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns a list of RecommendationFeedbackSummary objects that contain customer recommendation feedback for all CodeGuru Reviewer users. |
| `ListRecommendations` | `GET /codereviews/{CodeReviewArn}/Recommendations` | `paginated` | `CodeReviewArn` | - | `ListRecommendationsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns the list of all recommendations for a completed code review. |
| `ListRepositoryAssociations` | `GET /associations` | `paginated` | - | - | `ListRepositoryAssociationsResponse` | `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns a list of RepositoryAssociationSummary objects that contain summary information about a repository association. You can filter the returned list by ProviderType, Name, State, and Owner. |
| `ListTagsForResource` | `GET /tags/{resourceArn}` | - | `resourceArn` | - | `ListTagsForResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Returns the list of tags associated with an associated repository resource. |
| `PutRecommendationFeedback` | `PUT /feedback` | - | `CodeReviewArn`, `Reactions`, `RecommendationId` | - | `PutRecommendationFeedbackResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Stores customer feedback for a CodeGuru Reviewer recommendation. When this API is called again with different reactions the previous feedback is overwritten. |
| `TagResource` | `POST /tags/{resourceArn}` | - | `Tags`, `resourceArn` | - | `TagResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Adds one or more tags to an associated repository. |
| `UntagResource` | `DELETE /tags/{resourceArn}` | - | `TagKeys`, `resourceArn` | - | `UntagResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Removes a tag from an associated repository. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `DescribeRecommendationFeedback` | - | `RecommendationId -> RecommendationId`, `UserId -> UserId` | - | - |
| `ListCodeReviews` | - | `ProviderTypes -> ProviderTypes`, `States -> States`, `RepositoryNames -> RepositoryNames`, `Type -> Type`, `MaxResults -> MaxResults`, `NextToken -> NextToken` | - | - |
| `ListRecommendationFeedback` | - | `NextToken -> NextToken`, `MaxResults -> MaxResults`, `UserIds -> UserIds`, `RecommendationIds -> RecommendationIds` | - | - |
| `ListRecommendations` | - | `NextToken -> NextToken`, `MaxResults -> MaxResults` | - | - |
| `ListRepositoryAssociations` | - | `ProviderTypes -> ProviderType`, `States -> State`, `Names -> Name`, `Owners -> Owner`, `MaxResults -> MaxResults`, `NextToken -> NextToken` | - | - |
| `UntagResource` | - | `TagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InternalServerException` | `structure` | `Message` | The server encountered an internal error and is unable to complete the request. |
| `ValidationException` | `structure` | `Message` | The input fails to satisfy the specified constraints. |
| `ThrottlingException` | `structure` | `Message` | The request was denied due to request throttling. |
| `AccessDeniedException` | `structure` | `Message` | You do not have sufficient access to perform this action. |
| `ResourceNotFoundException` | `structure` | `Message` | The resource specified in the request was not found. |
| `ConflictException` | `structure` | `Message` | The requested operation would cause a conflict with the current state of a service resource associated with the request. |
| `NotFoundException` | `structure` | `Message` | The resource specified in the request was not found. |
| `AssociateRepositoryRequest` | `structure` | `ClientRequestToken`, `KMSKeyDetails`, `Repository`, `Tags` | - |
| `AssociateRepositoryResponse` | `structure` | `RepositoryAssociation`, `Tags` | - |
| `CreateCodeReviewRequest` | `structure` | `ClientRequestToken`, `Name`, `RepositoryAssociationArn`, `Type` | - |
| `CreateCodeReviewResponse` | `structure` | `CodeReview` | - |
| `DescribeCodeReviewRequest` | `structure` | `CodeReviewArn` | - |
| `DescribeCodeReviewResponse` | `structure` | `CodeReview` | - |
| `DescribeRecommendationFeedbackRequest` | `structure` | `CodeReviewArn`, `RecommendationId`, `UserId` | - |
| `DescribeRecommendationFeedbackResponse` | `structure` | `RecommendationFeedback` | - |
| `DescribeRepositoryAssociationRequest` | `structure` | `AssociationArn` | - |
| `DescribeRepositoryAssociationResponse` | `structure` | `RepositoryAssociation`, `Tags` | - |
| `DisassociateRepositoryRequest` | `structure` | `AssociationArn` | - |
| `DisassociateRepositoryResponse` | `structure` | `RepositoryAssociation`, `Tags` | - |
| `ListCodeReviewsRequest` | `structure` | `MaxResults`, `NextToken`, `ProviderTypes`, `RepositoryNames`, `States`, `Type` | - |
| `ListCodeReviewsResponse` | `structure` | `CodeReviewSummaries`, `NextToken` | - |
| `ListRecommendationFeedbackRequest` | `structure` | `CodeReviewArn`, `MaxResults`, `NextToken`, `RecommendationIds`, `UserIds` | - |
| `ListRecommendationFeedbackResponse` | `structure` | `NextToken`, `RecommendationFeedbackSummaries` | - |
| `ListRecommendationsRequest` | `structure` | `CodeReviewArn`, `MaxResults`, `NextToken` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
