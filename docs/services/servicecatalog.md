# winterbaume-servicecatalog

Service Catalog service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Service Catalog |
| AWS model | `service-catalog` |
| Protocol | awsJson1.1 |
| winterbaume coverage | 4/90 operations (4.4%) |
| stubs (routed, returns empty/default) | 0/90 operations (0.0%) |
| moto coverage | 0/90 operations (0.0%) |
| floci coverage | 0/90 operations (0.0%) |
| kumo coverage | 0/90 operations (0.0%) |
| Coverage report date | 2026-05-16 |

## Server-mode usage

Install `winterbaume-server` from crates.io or run it from a workspace checkout, then point the AWS CLI at it:

```sh
# Installed binary ( from crates.io ):
cargo install winterbaume-server
winterbaume-server --host 127.0.0.1 --port 5555

# Or, from a workspace checkout:
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws servicecatalog list-portfolios
```

## Example

```rust
use aws_sdk_servicecatalog::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_servicecatalog::ServiceCatalogService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(ServiceCatalogService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_servicecatalog::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_servicecatalog::Client::new(&config);

    let resp = client
        .list_portfolios()
        .send()
        .await
        .expect("list_portfolios should succeed");
    println!(
        "Service Catalog portfolios: {}",
        resp.portfolio_details().len()
    );
}
```

## Implemented APIs (4)

- `CreatePortfolio`
- `DeletePortfolio`
- `DescribePortfolio`
- `ListPortfolios`

<details><summary>Not yet implemented APIs (86)</summary>

- `AcceptPortfolioShare`
- `AssociateBudgetWithResource`
- `AssociatePrincipalWithPortfolio`
- `AssociateProductWithPortfolio`
- `AssociateServiceActionWithProvisioningArtifact`
- `AssociateTagOptionWithResource`
- `BatchAssociateServiceActionWithProvisioningArtifact`
- `BatchDisassociateServiceActionFromProvisioningArtifact`
- `CopyProduct`
- `CreateConstraint`
- `CreatePortfolioShare`
- `CreateProduct`
- `CreateProvisionedProductPlan`
- `CreateProvisioningArtifact`
- `CreateServiceAction`
- `CreateTagOption`
- `DeleteConstraint`
- `DeletePortfolioShare`
- `DeleteProduct`
- `DeleteProvisionedProductPlan`
- `DeleteProvisioningArtifact`
- `DeleteServiceAction`
- `DeleteTagOption`
- `DescribeConstraint`
- `DescribeCopyProductStatus`
- `DescribePortfolioShareStatus`
- `DescribePortfolioShares`
- `DescribeProduct`
- `DescribeProductAsAdmin`
- `DescribeProductView`
- `DescribeProvisionedProduct`
- `DescribeProvisionedProductPlan`
- `DescribeProvisioningArtifact`
- `DescribeProvisioningParameters`
- `DescribeRecord`
- `DescribeServiceAction`
- `DescribeServiceActionExecutionParameters`
- `DescribeTagOption`
- `DisableAWSOrganizationsAccess`
- `DisassociateBudgetFromResource`
- `DisassociatePrincipalFromPortfolio`
- `DisassociateProductFromPortfolio`
- `DisassociateServiceActionFromProvisioningArtifact`
- `DisassociateTagOptionFromResource`
- `EnableAWSOrganizationsAccess`
- `ExecuteProvisionedProductPlan`
- `ExecuteProvisionedProductServiceAction`
- `GetAWSOrganizationsAccessStatus`
- `GetProvisionedProductOutputs`
- `ImportAsProvisionedProduct`
- `ListAcceptedPortfolioShares`
- `ListBudgetsForResource`
- `ListConstraintsForPortfolio`
- `ListLaunchPaths`
- `ListOrganizationPortfolioAccess`
- `ListPortfolioAccess`
- `ListPortfoliosForProduct`
- `ListPrincipalsForPortfolio`
- `ListProvisionedProductPlans`
- `ListProvisioningArtifacts`
- `ListProvisioningArtifactsForServiceAction`
- `ListRecordHistory`
- `ListResourcesForTagOption`
- `ListServiceActions`
- `ListServiceActionsForProvisioningArtifact`
- `ListStackInstancesForProvisionedProduct`
- `ListTagOptions`
- `NotifyProvisionProductEngineWorkflowResult`
- `NotifyTerminateProvisionedProductEngineWorkflowResult`
- `NotifyUpdateProvisionedProductEngineWorkflowResult`
- `ProvisionProduct`
- `RejectPortfolioShare`
- `ScanProvisionedProducts`
- `SearchProducts`
- `SearchProductsAsAdmin`
- `SearchProvisionedProducts`
- `TerminateProvisionedProduct`
- `UpdateConstraint`
- `UpdatePortfolio`
- `UpdatePortfolioShare`
- `UpdateProduct`
- `UpdateProvisionedProduct`
- `UpdateProvisionedProductProperties`
- `UpdateProvisioningArtifact`
- `UpdateServiceAction`
- `UpdateTagOption`

</details>
