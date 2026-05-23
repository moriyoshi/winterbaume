# AWS Service Catalog

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Service Catalog Service Catalog enables organizations to create and manage catalogs of IT services that are approved for Amazon Web Services. To get the most out of this documentation, you should be familiar with the terminology discussed in Service Catalog Concepts.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for AWS Service Catalog where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: exercise account or service defaults for AWS Service Catalog by toggling configuration, creating later resources without explicit overrides, and confirming the default propagates into those resources.
- Scenario insight from EC2: cover association replacement for AWS Service Catalog by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- From the AWS documentation and model: represent documented AWS Service Catalog workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Describe`, `Update`, `Create`, `Delete` operation families, including `ListAcceptedPortfolioShares`, `ListBudgetsForResource`, `ListConstraintsForPortfolio`, `ListLaunchPaths`, `DescribeConstraint`, `DescribeCopyProductStatus`.

## Service Identity and Protocol

- AWS model slug: `service-catalog`
- AWS SDK for Rust slug: `servicecatalog`
- Model version: `2015-12-10`
- Model file: `vendor/api-models-aws/models/service-catalog/service/2015-12-10/service-catalog-2015-12-10.json`
- SDK ID: `Service Catalog`
- Endpoint prefix: `servicecatalog`
- ARN namespace: `servicecatalog`
- CloudFormation name: `ServiceCatalog`
- CloudTrail event source: `servicecatalog.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (18), `Describe` (16), `Update` (9), `Create` (8), `Delete` (8), `Associate` (5), `Disassociate` (5), `Notify` (3).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AcceptPortfolioShare`, `AssociateBudgetWithResource`, `AssociatePrincipalWithPortfolio`, `AssociateProductWithPortfolio`, `AssociateServiceActionWithProvisioningArtifact`, `AssociateTagOptionWithResource`, `BatchAssociateServiceActionWithProvisioningArtifact`, `BatchDisassociateServiceActionFromProvisioningArtifact`, `CreateConstraint`, `CreatePortfolio`, `CreatePortfolioShare`, `CreateProduct`, `CreateProvisionedProductPlan`, `CreateProvisioningArtifact`, `CreateServiceAction`, `CreateTagOption`, `DeleteConstraint`, `DeletePortfolio`, `DeletePortfolioShare`, `DeleteProduct`, ... (+23).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeConstraint`, `DescribeCopyProductStatus`, `DescribePortfolio`, `DescribePortfolioShareStatus`, `DescribePortfolioShares`, `DescribeProduct`, `DescribeProductAsAdmin`, `DescribeProductView`, `DescribeProvisionedProduct`, `DescribeProvisionedProductPlan`, `DescribeProvisioningArtifact`, `DescribeProvisioningParameters`, `DescribeRecord`, `DescribeServiceAction`, `DescribeServiceActionExecutionParameters`, `DescribeTagOption`, `GetAWSOrganizationsAccessStatus`, `GetProvisionedProductOutputs`, `ListAcceptedPortfolioShares`, `ListBudgetsForResource`, ... (+19).
- Pagination is modelled for 19 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 20 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `DescribeServiceActionExecutionParameters`, `ImportAsProvisionedProduct`, `ScanProvisionedProducts`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 90 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `CloudWatch`, `EC2/VPC`.

## Operation Groups

### List

- Operations: `ListAcceptedPortfolioShares`, `ListBudgetsForResource`, `ListConstraintsForPortfolio`, `ListLaunchPaths`, `ListOrganizationPortfolioAccess`, `ListPortfolioAccess`, `ListPortfolios`, `ListPortfoliosForProduct`, `ListPrincipalsForPortfolio`, `ListProvisionedProductPlans`, `ListProvisioningArtifacts`, `ListProvisioningArtifactsForServiceAction`, `ListRecordHistory`, `ListResourcesForTagOption`, `ListServiceActions`, `ListServiceActionsForProvisioningArtifact`, `ListStackInstancesForProvisionedProduct`, `ListTagOptions`
- Traits: `paginated` (14)
- Common required input members in this group: `PortfolioId`, `ProductId`

### Describe

- Operations: `DescribeConstraint`, `DescribeCopyProductStatus`, `DescribePortfolio`, `DescribePortfolioShares`, `DescribePortfolioShareStatus`, `DescribeProduct`, `DescribeProductAsAdmin`, `DescribeProductView`, `DescribeProvisionedProduct`, `DescribeProvisionedProductPlan`, `DescribeProvisioningArtifact`, `DescribeProvisioningParameters`, `DescribeRecord`, `DescribeServiceAction`, `DescribeServiceActionExecutionParameters`, `DescribeTagOption`
- Traits: `paginated` (1)
- Common required input members in this group: `Id`

### Update

- Operations: `UpdateConstraint`, `UpdatePortfolio`, `UpdatePortfolioShare`, `UpdateProduct`, `UpdateProvisionedProduct`, `UpdateProvisionedProductProperties`, `UpdateProvisioningArtifact`, `UpdateServiceAction`, `UpdateTagOption`
- Traits: `idempotency-token` (2)
- Common required input members in this group: `Id`

### Create

- Operations: `CreateConstraint`, `CreatePortfolio`, `CreatePortfolioShare`, `CreateProduct`, `CreateProvisionedProductPlan`, `CreateProvisioningArtifact`, `CreateServiceAction`, `CreateTagOption`
- Traits: `idempotency-token` (6)
- Common required input members in this group: `PortfolioId`, `ProductId`, `Parameters`, `IdempotencyToken`, `Name`

### Delete

- Operations: `DeleteConstraint`, `DeletePortfolio`, `DeletePortfolioShare`, `DeleteProduct`, `DeleteProvisionedProductPlan`, `DeleteProvisioningArtifact`, `DeleteServiceAction`, `DeleteTagOption`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `Id`

### Associate

- Operations: `AssociateBudgetWithResource`, `AssociatePrincipalWithPortfolio`, `AssociateProductWithPortfolio`, `AssociateServiceActionWithProvisioningArtifact`, `AssociateTagOptionWithResource`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `ResourceId`, `PortfolioId`, `ProductId`

### Disassociate

- Operations: `DisassociateBudgetFromResource`, `DisassociatePrincipalFromPortfolio`, `DisassociateProductFromPortfolio`, `DisassociateServiceActionFromProvisioningArtifact`, `DisassociateTagOptionFromResource`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `ResourceId`, `PortfolioId`, `ProductId`

### Notify

- Operations: `NotifyProvisionProductEngineWorkflowResult`, `NotifyTerminateProvisionedProductEngineWorkflowResult`, `NotifyUpdateProvisionedProductEngineWorkflowResult`
- Traits: `idempotency-token` (3)
- Common required input members in this group: `WorkflowToken`, `RecordId`, `Status`, `IdempotencyToken`

### Search

- Operations: `SearchProducts`, `SearchProductsAsAdmin`, `SearchProvisionedProducts`
- Traits: `paginated` (3)
- Common required input members in this group: -

### Batch

- Operations: `BatchAssociateServiceActionWithProvisioningArtifact`, `BatchDisassociateServiceActionFromProvisioningArtifact`
- Common required input members in this group: `ServiceActionAssociations`

### Execute

- Operations: `ExecuteProvisionedProductPlan`, `ExecuteProvisionedProductServiceAction`
- Traits: `idempotency-token` (2)
- Common required input members in this group: -

### Get

- Operations: `GetAWSOrganizationsAccessStatus`, `GetProvisionedProductOutputs`
- Traits: `paginated` (1)
- Common required input members in this group: -

### Accept

- Operations: `AcceptPortfolioShare`
- Common required input members in this group: -

### Copy

- Operations: `CopyProduct`
- Traits: `idempotency-token` (1)
- Common required input members in this group: -

### Disable

- Operations: `DisableAWSOrganizationsAccess`
- Common required input members in this group: -

### Enable

- Operations: `EnableAWSOrganizationsAccess`
- Common required input members in this group: -

### Import

- Operations: `ImportAsProvisionedProduct`
- Traits: `idempotency-token` (1)
- Common required input members in this group: -

### Provision

- Operations: `ProvisionProduct`
- Traits: `idempotency-token` (1)
- Common required input members in this group: -

### Reject

- Operations: `RejectPortfolioShare`
- Common required input members in this group: -

### Scan

- Operations: `ScanProvisionedProducts`
- Common required input members in this group: -

### Terminate

- Operations: `TerminateProvisionedProduct`
- Traits: `idempotency-token` (1)
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AcceptPortfolioShare` | `-` | - | `PortfolioId` | - | `AcceptPortfolioShareOutput` | `InvalidParametersException`, `LimitExceededException`, `ResourceNotFoundException` | Accepts an offer to share the specified portfolio. |
| `AssociateBudgetWithResource` | `-` | - | `BudgetName`, `ResourceId` | - | `AssociateBudgetWithResourceOutput` | `DuplicateResourceException`, `InvalidParametersException`, `LimitExceededException`, `ResourceNotFoundException` | Associates the specified budget with the specified resource. |
| `AssociatePrincipalWithPortfolio` | `-` | - | `PortfolioId`, `PrincipalARN`, `PrincipalType` | - | `AssociatePrincipalWithPortfolioOutput` | `InvalidParametersException`, `LimitExceededException`, `ResourceNotFoundException` | Associates the specified principal ARN with the specified portfolio. If you share the portfolio with principal name sharing enabled, the PrincipalARN association is included in the share. The PortfolioID , PrincipalA ... |
| `AssociateProductWithPortfolio` | `-` | - | `ProductId`, `PortfolioId` | - | `AssociateProductWithPortfolioOutput` | `InvalidParametersException`, `LimitExceededException`, `ResourceNotFoundException` | Associates the specified product with the specified portfolio. A delegated admin is authorized to invoke this command. |
| `AssociateServiceActionWithProvisioningArtifact` | `-` | `idempotency-token` | `ProductId`, `ProvisioningArtifactId`, `ServiceActionId` | `IdempotencyToken` | `AssociateServiceActionWithProvisioningArtifactOutput` | `DuplicateResourceException`, `InvalidParametersException`, `LimitExceededException`, `ResourceNotFoundException` | Associates a self-service action with a provisioning artifact. |
| `AssociateTagOptionWithResource` | `-` | - | `ResourceId`, `TagOptionId` | - | `AssociateTagOptionWithResourceOutput` | `DuplicateResourceException`, `InvalidParametersException`, `InvalidStateException`, `LimitExceededException`, `ResourceNotFoundException`, `TagOptionNotMigratedException` | Associate the specified TagOption with the specified portfolio or product. |
| `BatchAssociateServiceActionWithProvisioningArtifact` | `-` | - | `ServiceActionAssociations` | - | `BatchAssociateServiceActionWithProvisioningArtifactOutput` | `InvalidParametersException` | Associates multiple self-service actions with provisioning artifacts. |
| `BatchDisassociateServiceActionFromProvisioningArtifact` | `-` | - | `ServiceActionAssociations` | - | `BatchDisassociateServiceActionFromProvisioningArtifactOutput` | `InvalidParametersException` | Disassociates a batch of self-service actions from the specified provisioning artifact. |
| `CopyProduct` | `-` | `idempotency-token` | `SourceProductArn`, `IdempotencyToken` | `IdempotencyToken` | `CopyProductOutput` | `InvalidParametersException`, `ResourceNotFoundException` | Copies the specified source product to the specified target product or a new product. You can copy a product to the same account or another account. You can copy a product to the same Region or another Region. If you ... |
| `CreateConstraint` | `-` | `idempotency-token` | `PortfolioId`, `ProductId`, `Parameters`, `Type`, `IdempotencyToken` | `IdempotencyToken` | `CreateConstraintOutput` | `DuplicateResourceException`, `InvalidParametersException`, `LimitExceededException`, `ResourceNotFoundException` | Creates a constraint. A delegated admin is authorized to invoke this command. |
| `CreatePortfolio` | `-` | `idempotency-token` | `DisplayName`, `ProviderName`, `IdempotencyToken` | `IdempotencyToken` | `CreatePortfolioOutput` | `InvalidParametersException`, `LimitExceededException`, `TagOptionNotMigratedException` | Creates a portfolio. A delegated admin is authorized to invoke this command. |
| `CreatePortfolioShare` | `-` | - | `PortfolioId` | - | `CreatePortfolioShareOutput` | `InvalidParametersException`, `InvalidStateException`, `LimitExceededException`, `OperationNotSupportedException`, `ResourceNotFoundException` | Shares the specified portfolio with the specified account or organization node. Shares to an organization node can only be created by the management account of an organization or by a delegated administrator. You can ... |
| `CreateProduct` | `-` | `idempotency-token` | `Name`, `Owner`, `ProductType`, `IdempotencyToken` | `IdempotencyToken` | `CreateProductOutput` | `InvalidParametersException`, `LimitExceededException`, `TagOptionNotMigratedException` | Creates a product. A delegated admin is authorized to invoke this command. The user or role that performs this operation must have the cloudformation:GetTemplate IAM policy permission. This policy permission is requi ... |
| `CreateProvisionedProductPlan` | `-` | `idempotency-token` | `PlanName`, `PlanType`, `ProductId`, `ProvisionedProductName`, `ProvisioningArtifactId`, `IdempotencyToken` | `IdempotencyToken` | `CreateProvisionedProductPlanOutput` | `InvalidParametersException`, `InvalidStateException`, `ResourceNotFoundException` | Creates a plan. A plan includes the list of resources to be created (when provisioning a new product) or modified (when updating a provisioned product) when the plan is executed. You can create one plan for each prov ... |
| `CreateProvisioningArtifact` | `-` | `idempotency-token` | `ProductId`, `Parameters`, `IdempotencyToken` | `IdempotencyToken` | `CreateProvisioningArtifactOutput` | `InvalidParametersException`, `LimitExceededException`, `ResourceNotFoundException` | Creates a provisioning artifact (also known as a version) for the specified product. You cannot create a provisioning artifact for a product that was shared with you. The user or role that performs this operation mus ... |
| `CreateServiceAction` | `-` | `idempotency-token` | `Name`, `DefinitionType`, `Definition`, `IdempotencyToken` | `IdempotencyToken` | `CreateServiceActionOutput` | `InvalidParametersException`, `LimitExceededException` | Creates a self-service action. |
| `CreateTagOption` | `-` | - | `Key`, `Value` | - | `CreateTagOptionOutput` | `DuplicateResourceException`, `LimitExceededException`, `TagOptionNotMigratedException` | Creates a TagOption. |
| `DeleteConstraint` | `-` | - | `Id` | - | `DeleteConstraintOutput` | `InvalidParametersException`, `ResourceNotFoundException` | Deletes the specified constraint. A delegated admin is authorized to invoke this command. |
| `DeletePortfolio` | `-` | - | `Id` | - | `DeletePortfolioOutput` | `InvalidParametersException`, `ResourceInUseException`, `ResourceNotFoundException`, `TagOptionNotMigratedException` | Deletes the specified portfolio. You cannot delete a portfolio if it was shared with you or if it has associated products, users, constraints, or shared accounts. A delegated admin is authorized to invoke this command. |
| `DeletePortfolioShare` | `-` | - | `PortfolioId` | - | `DeletePortfolioShareOutput` | `InvalidParametersException`, `InvalidStateException`, `OperationNotSupportedException`, `ResourceNotFoundException` | Stops sharing the specified portfolio with the specified account or organization node. Shares to an organization node can only be deleted by the management account of an organization or by a delegated administrator. ... |
| `DeleteProduct` | `-` | - | `Id` | - | `DeleteProductOutput` | `InvalidParametersException`, `ResourceInUseException`, `ResourceNotFoundException`, `TagOptionNotMigratedException` | Deletes the specified product. You cannot delete a product if it was shared with you or is associated with a portfolio. A delegated admin is authorized to invoke this command. |
| `DeleteProvisionedProductPlan` | `-` | - | `PlanId` | - | `DeleteProvisionedProductPlanOutput` | `InvalidParametersException`, `ResourceNotFoundException` | Deletes the specified plan. |
| `DeleteProvisioningArtifact` | `-` | - | `ProductId`, `ProvisioningArtifactId` | - | `DeleteProvisioningArtifactOutput` | `InvalidParametersException`, `ResourceInUseException`, `ResourceNotFoundException` | Deletes the specified provisioning artifact (also known as a version) for the specified product. You cannot delete a provisioning artifact associated with a product that was shared with you. You cannot delete the las ... |
| `DeleteServiceAction` | `-` | `idempotency-token` | `Id` | `IdempotencyToken` | `DeleteServiceActionOutput` | `InvalidParametersException`, `ResourceInUseException`, `ResourceNotFoundException` | Deletes a self-service action. |
| `DeleteTagOption` | `-` | - | `Id` | - | `DeleteTagOptionOutput` | `ResourceInUseException`, `ResourceNotFoundException`, `TagOptionNotMigratedException` | Deletes the specified TagOption. You cannot delete a TagOption if it is associated with a product or portfolio. |
| `DescribeConstraint` | `-` | - | `Id` | - | `DescribeConstraintOutput` | `ResourceNotFoundException` | Gets information about the specified constraint. |
| `DescribeCopyProductStatus` | `-` | - | `CopyProductToken` | - | `DescribeCopyProductStatusOutput` | `ResourceNotFoundException` | Gets the status of the specified copy product operation. |
| `DescribePortfolio` | `-` | - | `Id` | - | `DescribePortfolioOutput` | `ResourceNotFoundException` | Gets information about the specified portfolio. A delegated admin is authorized to invoke this command. |
| `DescribePortfolioShares` | `-` | `paginated` | `PortfolioId`, `Type` | - | `DescribePortfolioSharesOutput` | `InvalidParametersException`, `ResourceNotFoundException` | Returns a summary of each of the portfolio shares that were created for the specified portfolio. You can use this API to determine which accounts or organizational nodes this portfolio have been shared, whether the r ... |
| `DescribePortfolioShareStatus` | `-` | - | `PortfolioShareToken` | - | `DescribePortfolioShareStatusOutput` | `InvalidParametersException`, `OperationNotSupportedException`, `ResourceNotFoundException` | Gets the status of the specified portfolio share operation. This API can only be called by the management account in the organization or by a delegated admin. |
| `DescribeProduct` | `-` | - | - | - | `DescribeProductOutput` | `InvalidParametersException`, `ResourceNotFoundException` | Gets information about the specified product. Running this operation with administrator access results in a failure. DescribeProductAsAdmin should be used instead. |
| `DescribeProductAsAdmin` | `-` | - | - | - | `DescribeProductAsAdminOutput` | `InvalidParametersException`, `ResourceNotFoundException` | Gets information about the specified product. This operation is run with administrator access. |
| `DescribeProductView` | `-` | - | `Id` | - | `DescribeProductViewOutput` | `InvalidParametersException`, `ResourceNotFoundException` | Gets information about the specified product. |
| `DescribeProvisionedProduct` | `-` | - | - | - | `DescribeProvisionedProductOutput` | `InvalidParametersException`, `ResourceNotFoundException` | Gets information about the specified provisioned product. |
| `DescribeProvisionedProductPlan` | `-` | - | `PlanId` | - | `DescribeProvisionedProductPlanOutput` | `InvalidParametersException`, `ResourceNotFoundException` | Gets information about the resource changes for the specified plan. |
| `DescribeProvisioningArtifact` | `-` | - | - | - | `DescribeProvisioningArtifactOutput` | `InvalidParametersException`, `ResourceNotFoundException` | Gets information about the specified provisioning artifact (also known as a version) for the specified product. |
| `DescribeProvisioningParameters` | `-` | - | - | - | `DescribeProvisioningParametersOutput` | `InvalidParametersException`, `ResourceNotFoundException` | Gets information about the configuration required to provision the specified product using the specified provisioning artifact. If the output contains a TagOption key with an empty list of values, there is a TagOptio ... |
| `DescribeRecord` | `-` | - | `Id` | - | `DescribeRecordOutput` | `ResourceNotFoundException` | Gets information about the specified request operation. Use this operation after calling a request operation (for example, ProvisionProduct , TerminateProvisionedProduct , or UpdateProvisionedProduct ). If a provisio ... |
| `DescribeServiceAction` | `-` | - | `Id` | - | `DescribeServiceActionOutput` | `ResourceNotFoundException` | Describes a self-service action. |
| `DescribeServiceActionExecutionParameters` | `-` | - | `ProvisionedProductId`, `ServiceActionId` | - | `DescribeServiceActionExecutionParametersOutput` | `InvalidParametersException`, `ResourceNotFoundException` | Finds the default parameters for a specific self-service action on a specific provisioned product and returns a map of the results to the user. |
| `DescribeTagOption` | `-` | - | `Id` | - | `DescribeTagOptionOutput` | `ResourceNotFoundException`, `TagOptionNotMigratedException` | Gets information about the specified TagOption. |
| `DisableAWSOrganizationsAccess` | `-` | - | - | - | `DisableAWSOrganizationsAccessOutput` | `InvalidStateException`, `OperationNotSupportedException`, `ResourceNotFoundException` | Disable portfolio sharing through the Organizations service. This command will not delete your current shares, but prevents you from creating new shares throughout your organization. Current shares are not kept in sy ... |
| `DisassociateBudgetFromResource` | `-` | - | `BudgetName`, `ResourceId` | - | `DisassociateBudgetFromResourceOutput` | `ResourceNotFoundException` | Disassociates the specified budget from the specified resource. |
| `DisassociatePrincipalFromPortfolio` | `-` | - | `PortfolioId`, `PrincipalARN` | - | `DisassociatePrincipalFromPortfolioOutput` | `InvalidParametersException`, `ResourceNotFoundException` | Disassociates a previously associated principal ARN from a specified portfolio. The PrincipalType and PrincipalARN must match the AssociatePrincipalWithPortfolio call request details. For example, to disassociate an ... |
| `DisassociateProductFromPortfolio` | `-` | - | `ProductId`, `PortfolioId` | - | `DisassociateProductFromPortfolioOutput` | `InvalidParametersException`, `ResourceInUseException`, `ResourceNotFoundException` | Disassociates the specified product from the specified portfolio. A delegated admin is authorized to invoke this command. |
| `DisassociateServiceActionFromProvisioningArtifact` | `-` | `idempotency-token` | `ProductId`, `ProvisioningArtifactId`, `ServiceActionId` | `IdempotencyToken` | `DisassociateServiceActionFromProvisioningArtifactOutput` | `InvalidParametersException`, `ResourceNotFoundException` | Disassociates the specified self-service action association from the specified provisioning artifact. |
| `DisassociateTagOptionFromResource` | `-` | - | `ResourceId`, `TagOptionId` | - | `DisassociateTagOptionFromResourceOutput` | `ResourceNotFoundException`, `TagOptionNotMigratedException` | Disassociates the specified TagOption from the specified resource. |
| `EnableAWSOrganizationsAccess` | `-` | - | - | - | `EnableAWSOrganizationsAccessOutput` | `InvalidStateException`, `OperationNotSupportedException`, `ResourceNotFoundException` | Enable portfolio sharing feature through Organizations. This API will allow Service Catalog to receive updates on your organization in order to sync your shares with the current structure. This API can only be called ... |
| `ExecuteProvisionedProductPlan` | `-` | `idempotency-token` | `PlanId`, `IdempotencyToken` | `IdempotencyToken` | `ExecuteProvisionedProductPlanOutput` | `InvalidParametersException`, `InvalidStateException`, `ResourceNotFoundException` | Provisions or modifies a product based on the resource changes for the specified plan. |
| `ExecuteProvisionedProductServiceAction` | `-` | `idempotency-token` | `ProvisionedProductId`, `ServiceActionId`, `ExecuteToken` | `ExecuteToken` | `ExecuteProvisionedProductServiceActionOutput` | `InvalidParametersException`, `InvalidStateException`, `ResourceNotFoundException` | Executes a self-service action against a provisioned product. |
| `GetAWSOrganizationsAccessStatus` | `-` | - | - | - | `GetAWSOrganizationsAccessStatusOutput` | `OperationNotSupportedException`, `ResourceNotFoundException` | Get the Access Status for Organizations portfolio share feature. This API can only be called by the management account in the organization or by a delegated admin. |
| `GetProvisionedProductOutputs` | `-` | `paginated` | - | - | `GetProvisionedProductOutputsOutput` | `InvalidParametersException`, `ResourceNotFoundException` | This API takes either a ProvisonedProductId or a ProvisionedProductName , along with a list of one or more output keys, and responds with the key/value pairs of those outputs. |
| `ImportAsProvisionedProduct` | `-` | `idempotency-token` | `ProductId`, `ProvisioningArtifactId`, `ProvisionedProductName`, `PhysicalId`, `IdempotencyToken` | `IdempotencyToken` | `ImportAsProvisionedProductOutput` | `DuplicateResourceException`, `InvalidParametersException`, `InvalidStateException`, `ResourceNotFoundException` | Requests the import of a resource as an Service Catalog provisioned product that is associated to an Service Catalog product and provisioning artifact. Once imported, all supported governance actions are supported on ... |
| `ListAcceptedPortfolioShares` | `-` | `paginated` | - | - | `ListAcceptedPortfolioSharesOutput` | `InvalidParametersException`, `OperationNotSupportedException` | Lists all imported portfolios for which account-to-account shares were accepted by this account. By specifying the PortfolioShareType , you can list portfolios for which organizational shares were accepted by this ac ... |
| `ListBudgetsForResource` | `-` | `paginated` | `ResourceId` | - | `ListBudgetsForResourceOutput` | `InvalidParametersException`, `ResourceNotFoundException` | Lists all the budgets associated to the specified resource. |
| `ListConstraintsForPortfolio` | `-` | `paginated` | `PortfolioId` | - | `ListConstraintsForPortfolioOutput` | `InvalidParametersException`, `ResourceNotFoundException` | Lists the constraints for the specified portfolio and product. |
| `ListLaunchPaths` | `-` | `paginated` | `ProductId` | - | `ListLaunchPathsOutput` | `InvalidParametersException`, `ResourceNotFoundException` | Lists the paths to the specified product. A path describes how the user gets access to a specified product and is necessary when provisioning a product. A path also determines the constraints that are put on a produc ... |
| `ListOrganizationPortfolioAccess` | `-` | `paginated` | `PortfolioId`, `OrganizationNodeType` | - | `ListOrganizationPortfolioAccessOutput` | `InvalidParametersException`, `OperationNotSupportedException`, `ResourceNotFoundException` | Lists the organization nodes that have access to the specified portfolio. This API can only be called by the management account in the organization or by a delegated admin. If a delegated admin is de-registered, they ... |
| `ListPortfolioAccess` | `-` | `paginated` | `PortfolioId` | - | `ListPortfolioAccessOutput` | `InvalidParametersException`, `ResourceNotFoundException` | Lists the account IDs that have access to the specified portfolio. A delegated admin can list the accounts that have access to the shared portfolio. Note that if a delegated admin is de-registered, they can no longer ... |
| `ListPortfolios` | `-` | `paginated` | - | - | `ListPortfoliosOutput` | `InvalidParametersException` | Lists all portfolios in the catalog. |
| `ListPortfoliosForProduct` | `-` | `paginated` | `ProductId` | - | `ListPortfoliosForProductOutput` | `InvalidParametersException`, `ResourceNotFoundException` | Lists all portfolios that the specified product is associated with. |
| `ListPrincipalsForPortfolio` | `-` | `paginated` | `PortfolioId` | - | `ListPrincipalsForPortfolioOutput` | `InvalidParametersException`, `ResourceNotFoundException` | Lists all PrincipalARN s and corresponding PrincipalType s associated with the specified portfolio. |
| `ListProvisionedProductPlans` | `-` | - | - | - | `ListProvisionedProductPlansOutput` | `InvalidParametersException`, `ResourceNotFoundException` | Lists the plans for the specified provisioned product or all plans to which the user has access. |
| `ListProvisioningArtifacts` | `-` | - | `ProductId` | - | `ListProvisioningArtifactsOutput` | `InvalidParametersException`, `ResourceNotFoundException` | Lists all provisioning artifacts (also known as versions) for the specified product. |
| `ListProvisioningArtifactsForServiceAction` | `-` | `paginated` | `ServiceActionId` | - | `ListProvisioningArtifactsForServiceActionOutput` | `InvalidParametersException`, `ResourceNotFoundException` | Lists all provisioning artifacts (also known as versions) for the specified self-service action. |
| `ListRecordHistory` | `-` | - | - | - | `ListRecordHistoryOutput` | `InvalidParametersException` | Lists the specified requests or all performed requests. |
| `ListResourcesForTagOption` | `-` | `paginated` | `TagOptionId` | - | `ListResourcesForTagOptionOutput` | `InvalidParametersException`, `ResourceNotFoundException`, `TagOptionNotMigratedException` | Lists the resources associated with the specified TagOption. |
| `ListServiceActions` | `-` | `paginated` | - | - | `ListServiceActionsOutput` | `InvalidParametersException` | Lists all self-service actions. |
| `ListServiceActionsForProvisioningArtifact` | `-` | `paginated` | `ProductId`, `ProvisioningArtifactId` | - | `ListServiceActionsForProvisioningArtifactOutput` | `InvalidParametersException`, `ResourceNotFoundException` | Returns a paginated list of self-service actions associated with the specified Product ID and Provisioning Artifact ID. |
| `ListStackInstancesForProvisionedProduct` | `-` | - | `ProvisionedProductId` | - | `ListStackInstancesForProvisionedProductOutput` | `InvalidParametersException`, `ResourceNotFoundException` | Returns summary information about stack instances that are associated with the specified CFN_STACKSET type provisioned product. You can filter for stack instances that are associated with a specific Amazon Web Servic ... |
| `ListTagOptions` | `-` | `paginated` | - | - | `ListTagOptionsOutput` | `InvalidParametersException`, `TagOptionNotMigratedException` | Lists the specified TagOptions or all TagOptions. |
| `NotifyProvisionProductEngineWorkflowResult` | `-` | `idempotency-token` | `WorkflowToken`, `RecordId`, `Status`, `IdempotencyToken` | `IdempotencyToken` | `NotifyProvisionProductEngineWorkflowResultOutput` | `InvalidParametersException`, `ResourceNotFoundException` | Notifies the result of the provisioning engine execution. |
| `NotifyTerminateProvisionedProductEngineWorkflowResult` | `-` | `idempotency-token` | `WorkflowToken`, `RecordId`, `Status`, `IdempotencyToken` | `IdempotencyToken` | `NotifyTerminateProvisionedProductEngineWorkflowResultOutput` | `InvalidParametersException`, `ResourceNotFoundException` | Notifies the result of the terminate engine execution. |
| `NotifyUpdateProvisionedProductEngineWorkflowResult` | `-` | `idempotency-token` | `WorkflowToken`, `RecordId`, `Status`, `IdempotencyToken` | `IdempotencyToken` | `NotifyUpdateProvisionedProductEngineWorkflowResultOutput` | `InvalidParametersException`, `ResourceNotFoundException` | Notifies the result of the update engine execution. |
| `ProvisionProduct` | `-` | `idempotency-token` | `ProvisionedProductName`, `ProvisionToken` | `ProvisionToken` | `ProvisionProductOutput` | `DuplicateResourceException`, `InvalidParametersException`, `ResourceNotFoundException` | Provisions the specified product. A provisioned product is a resourced instance of a product. For example, provisioning a product that's based on an CloudFormation template launches an CloudFormation stack and its un ... |
| `RejectPortfolioShare` | `-` | - | `PortfolioId` | - | `RejectPortfolioShareOutput` | `ResourceNotFoundException` | Rejects an offer to share the specified portfolio. |
| `ScanProvisionedProducts` | `-` | - | - | - | `ScanProvisionedProductsOutput` | `InvalidParametersException` | Lists the provisioned products that are available (not terminated). To use additional filtering, see SearchProvisionedProducts . |
| `SearchProducts` | `-` | `paginated` | - | - | `SearchProductsOutput` | `InvalidParametersException` | Gets information about the products to which the caller has access. |
| `SearchProductsAsAdmin` | `-` | `paginated` | - | - | `SearchProductsAsAdminOutput` | `InvalidParametersException`, `ResourceNotFoundException` | Gets information about the products for the specified portfolio or all products. |
| `SearchProvisionedProducts` | `-` | `paginated` | - | - | `SearchProvisionedProductsOutput` | `InvalidParametersException` | Gets information about the provisioned products that meet the specified criteria. |
| `TerminateProvisionedProduct` | `-` | `idempotency-token` | `TerminateToken` | `TerminateToken` | `TerminateProvisionedProductOutput` | `ResourceNotFoundException` | Terminates the specified provisioned product. This operation does not delete any records associated with the provisioned product. You can check the status of this request using DescribeRecord . |
| `UpdateConstraint` | `-` | - | `Id` | - | `UpdateConstraintOutput` | `InvalidParametersException`, `ResourceNotFoundException` | Updates the specified constraint. |
| `UpdatePortfolio` | `-` | - | `Id` | - | `UpdatePortfolioOutput` | `InvalidParametersException`, `LimitExceededException`, `ResourceNotFoundException`, `TagOptionNotMigratedException` | Updates the specified portfolio. You cannot update a product that was shared with you. |
| `UpdatePortfolioShare` | `-` | - | `PortfolioId` | - | `UpdatePortfolioShareOutput` | `InvalidParametersException`, `InvalidStateException`, `OperationNotSupportedException`, `ResourceNotFoundException` | Updates the specified portfolio share. You can use this API to enable or disable TagOptions sharing or Principal sharing for an existing portfolio share. The portfolio share cannot be updated if the CreatePortfolioSh ... |
| `UpdateProduct` | `-` | - | `Id` | - | `UpdateProductOutput` | `InvalidParametersException`, `ResourceNotFoundException`, `TagOptionNotMigratedException` | Updates the specified product. |
| `UpdateProvisionedProduct` | `-` | `idempotency-token` | `UpdateToken` | `UpdateToken` | `UpdateProvisionedProductOutput` | `InvalidParametersException`, `ResourceNotFoundException` | Requests updates to the configuration of the specified provisioned product. If there are tags associated with the object, they cannot be updated or added. Depending on the specific updates requested, this operation c ... |
| `UpdateProvisionedProductProperties` | `-` | `idempotency-token` | `ProvisionedProductId`, `ProvisionedProductProperties`, `IdempotencyToken` | `IdempotencyToken` | `UpdateProvisionedProductPropertiesOutput` | `InvalidParametersException`, `InvalidStateException`, `ResourceNotFoundException` | Requests updates to the properties of the specified provisioned product. |
| `UpdateProvisioningArtifact` | `-` | - | `ProductId`, `ProvisioningArtifactId` | - | `UpdateProvisioningArtifactOutput` | `InvalidParametersException`, `ResourceNotFoundException` | Updates the specified provisioning artifact (also known as a version) for the specified product. You cannot update a provisioning artifact for a product that was shared with you. |
| `UpdateServiceAction` | `-` | - | `Id` | - | `UpdateServiceActionOutput` | `InvalidParametersException`, `ResourceNotFoundException` | Updates a self-service action. |
| `UpdateTagOption` | `-` | - | `Id` | - | `UpdateTagOptionOutput` | `DuplicateResourceException`, `InvalidParametersException`, `ResourceNotFoundException`, `TagOptionNotMigratedException` | Updates the specified TagOption. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `DeleteTagOption` | - | `Id -> id` | - | - |
| `DescribeTagOption` | - | `Id -> id` | - | - |
| `DisassociateTagOptionFromResource` | - | `ResourceId -> resourceId`, `TagOptionId -> tagOptionId` | - | - |
| `ListResourcesForTagOption` | - | `TagOptionId -> tagOptionId`, `ResourceType -> resourceType`, `PageSize -> pageSize`, `PageToken -> pageToken` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `DuplicateResourceException` | `structure` | Message | The specified resource is a duplicate. |
| `InvalidParametersException` | `structure` | Message | One or more parameters provided to the operation are not valid. |
| `InvalidStateException` | `structure` | Message | An attempt was made to modify a resource that is in a state that is not valid. Check your resources to ensure that they are in valid states before retrying ... |
| `LimitExceededException` | `structure` | Message | The current limits of the service would have been exceeded by this operation. Decrease your resource use or increase your service limits and retry the opera ... |
| `OperationNotSupportedException` | `structure` | Message | The operation is not supported. |
| `ResourceInUseException` | `structure` | Message | A resource that is currently in use. Ensure that the resource is not in use and retry the operation. |
| `ResourceNotFoundException` | `structure` | Message | The specified resource was not found. |
| `TagOptionNotMigratedException` | `structure` | Message | An operation requiring TagOptions failed because the TagOptions migration process has not been performed for this account. Use the Amazon Web Services Manag ... |
| `AcceptPortfolioShareInput` | `structure` | AcceptLanguage, PortfolioId, PortfolioShareType | - |
| `AcceptPortfolioShareOutput` | `structure` | **empty (no members)** | - |
| `AssociateBudgetWithResourceInput` | `structure` | BudgetName, ResourceId | - |
| `AssociateBudgetWithResourceOutput` | `structure` | **empty (no members)** | - |
| `AssociatePrincipalWithPortfolioInput` | `structure` | AcceptLanguage, PortfolioId, PrincipalARN, PrincipalType | - |
| `AssociatePrincipalWithPortfolioOutput` | `structure` | **empty (no members)** | - |
| `AssociateProductWithPortfolioInput` | `structure` | AcceptLanguage, ProductId, PortfolioId, SourcePortfolioId | - |
| `AssociateProductWithPortfolioOutput` | `structure` | **empty (no members)** | - |
| `AssociateServiceActionWithProvisioningArtifactInput` | `structure` | ProductId, ProvisioningArtifactId, ServiceActionId, AcceptLanguage, IdempotencyToken | - |
| `AssociateServiceActionWithProvisioningArtifactOutput` | `structure` | **empty (no members)** | - |
| `AssociateTagOptionWithResourceInput` | `structure` | ResourceId, TagOptionId | - |
| `AssociateTagOptionWithResourceOutput` | `structure` | **empty (no members)** | - |
| `BatchAssociateServiceActionWithProvisioningArtifactInput` | `structure` | ServiceActionAssociations, AcceptLanguage | - |
| `BatchAssociateServiceActionWithProvisioningArtifactOutput` | `structure` | FailedServiceActionAssociations | - |
| `BatchDisassociateServiceActionFromProvisioningArtifactInput` | `structure` | ServiceActionAssociations, AcceptLanguage | - |
| `BatchDisassociateServiceActionFromProvisioningArtifactOutput` | `structure` | FailedServiceActionAssociations | - |
| `CopyProductInput` | `structure` | AcceptLanguage, SourceProductArn, TargetProductId, TargetProductName, SourceProvisioningArtifactIdentifiers, CopyOptions, IdempotencyToken | - |
| `CopyProductOutput` | `structure` | CopyProductToken | - |
| `CreateConstraintInput` | `structure` | AcceptLanguage, PortfolioId, ProductId, Parameters, Type, Description, IdempotencyToken | - |
| `CreateConstraintOutput` | `structure` | ConstraintDetail, ConstraintParameters, Status | - |
| `CreatePortfolioInput` | `structure` | AcceptLanguage, DisplayName, Description, ProviderName, Tags, IdempotencyToken | - |
| `CreatePortfolioOutput` | `structure` | PortfolioDetail, Tags | - |
| `CreatePortfolioShareInput` | `structure` | AcceptLanguage, PortfolioId, AccountId, OrganizationNode, ShareTagOptions, SharePrincipals | - |
| `CreatePortfolioShareOutput` | `structure` | PortfolioShareToken | - |
| `CreateProductInput` | `structure` | AcceptLanguage, Name, Owner, Description, Distributor, SupportDescription, SupportEmail, SupportUrl, ProductType, Tags, ProvisioningArtifactParameters, IdempotencyToken, ... (+1) | - |
| `CreateProductOutput` | `structure` | ProductViewDetail, ProvisioningArtifactDetail, Tags | - |
| `CreateProvisionedProductPlanInput` | `structure` | AcceptLanguage, PlanName, PlanType, NotificationArns, PathId, ProductId, ProvisionedProductName, ProvisioningArtifactId, ProvisioningParameters, IdempotencyToken, Tags | - |
| `CreateProvisionedProductPlanOutput` | `structure` | PlanName, PlanId, ProvisionProductId, ProvisionedProductName, ProvisioningArtifactId | - |
| `CreateProvisioningArtifactInput` | `structure` | AcceptLanguage, ProductId, Parameters, IdempotencyToken | - |
| `CreateProvisioningArtifactOutput` | `structure` | ProvisioningArtifactDetail, Info, Status | - |
| `CreateServiceActionInput` | `structure` | Name, DefinitionType, Definition, Description, AcceptLanguage, IdempotencyToken | - |
| `CreateServiceActionOutput` | `structure` | ServiceActionDetail | - |
| `AccessLevelFilterKey` | `enum` | ACCOUNT, ROLE, USER | - |
| `AccessStatus` | `enum` | ENABLED, UNDER_CHANGE, DISABLED | - |
| `ChangeAction` | `enum` | ADD, MODIFY, REMOVE | - |
| `CopyOption` | `enum` | CopyTags | - |
| `CopyProductStatus` | `enum` | SUCCEEDED, IN_PROGRESS, FAILED | - |
| `DescribePortfolioShareType` | `enum` | ACCOUNT, ORGANIZATION, ORGANIZATIONAL_UNIT, ORGANIZATION_MEMBER_ACCOUNT | - |
| `EngineWorkflowStatus` | `enum` | SUCCEEDED, FAILED | - |
| `EvaluationType` | `enum` | STATIC, DYNAMIC | - |
| `LastSyncStatus` | `enum` | SUCCEEDED, FAILED | - |
| `OrganizationNodeType` | `enum` | ORGANIZATION, ORGANIZATIONAL_UNIT, ACCOUNT | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
