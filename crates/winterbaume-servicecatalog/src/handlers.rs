use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
    StatefulService,
};

use crate::state::{ServiceCatalogError, ServiceCatalogState};
use crate::views::ServiceCatalogStateView;
use crate::wire;

pub struct ServiceCatalogService {
    pub(crate) state: Arc<BackendState<ServiceCatalogState>>,
    pub(crate) notifier: StateChangeNotifier<ServiceCatalogStateView>,
}

impl ServiceCatalogService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for ServiceCatalogService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for ServiceCatalogService {
    fn service_name(&self) -> &str {
        "servicecatalog"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://servicecatalog\..*\.amazonaws\.com",
            r"https?://servicecatalog\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl ServiceCatalogService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = DEFAULT_ACCOUNT_ID;

        let action = request
            .headers
            .get("x-amz-target")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.split('.').next_back())
            .map(|s| s.to_string());

        let action = match action {
            Some(a) => a,
            None => {
                return json_error_response(400, "MissingAction", "Missing X-Amz-Target header");
            }
        };

        if serde_json::from_slice::<Value>(&request.body).is_err() {
            return json_error_response(400, "SerializationException", "Invalid JSON body");
        }
        let body_bytes: &[u8] = &request.body;

        let state = self.state.get(account_id, &region);

        let response = match action.as_str() {
            "CreatePortfolio" => {
                self.handle_create_portfolio(&state, body_bytes, account_id, &region)
                    .await
            }
            "DescribePortfolio" => self.handle_describe_portfolio(&state, body_bytes).await,
            "DeletePortfolio" => self.handle_delete_portfolio(&state, body_bytes).await,
            "ListPortfolios" => self.handle_list_portfolios(&state).await,
            // --- Unimplemented operations (auto-generated stubs) ---
            "AcceptPortfolioShare" => json_error_response(
                501,
                "NotImplementedError",
                "AcceptPortfolioShare is not yet implemented in winterbaume-servicecatalog",
            ),
            "AssociateBudgetWithResource" => json_error_response(
                501,
                "NotImplementedError",
                "AssociateBudgetWithResource is not yet implemented in winterbaume-servicecatalog",
            ),
            "AssociatePrincipalWithPortfolio" => json_error_response(
                501,
                "NotImplementedError",
                "AssociatePrincipalWithPortfolio is not yet implemented in winterbaume-servicecatalog",
            ),
            "AssociateProductWithPortfolio" => json_error_response(
                501,
                "NotImplementedError",
                "AssociateProductWithPortfolio is not yet implemented in winterbaume-servicecatalog",
            ),
            "AssociateServiceActionWithProvisioningArtifact" => json_error_response(
                501,
                "NotImplementedError",
                "AssociateServiceActionWithProvisioningArtifact is not yet implemented in winterbaume-servicecatalog",
            ),
            "AssociateTagOptionWithResource" => json_error_response(
                501,
                "NotImplementedError",
                "AssociateTagOptionWithResource is not yet implemented in winterbaume-servicecatalog",
            ),
            "BatchAssociateServiceActionWithProvisioningArtifact" => json_error_response(
                501,
                "NotImplementedError",
                "BatchAssociateServiceActionWithProvisioningArtifact is not yet implemented in winterbaume-servicecatalog",
            ),
            "BatchDisassociateServiceActionFromProvisioningArtifact" => json_error_response(
                501,
                "NotImplementedError",
                "BatchDisassociateServiceActionFromProvisioningArtifact is not yet implemented in winterbaume-servicecatalog",
            ),
            "CopyProduct" => json_error_response(
                501,
                "NotImplementedError",
                "CopyProduct is not yet implemented in winterbaume-servicecatalog",
            ),
            "CreateConstraint" => json_error_response(
                501,
                "NotImplementedError",
                "CreateConstraint is not yet implemented in winterbaume-servicecatalog",
            ),
            "CreatePortfolioShare" => json_error_response(
                501,
                "NotImplementedError",
                "CreatePortfolioShare is not yet implemented in winterbaume-servicecatalog",
            ),
            "CreateProduct" => json_error_response(
                501,
                "NotImplementedError",
                "CreateProduct is not yet implemented in winterbaume-servicecatalog",
            ),
            "CreateProvisionedProductPlan" => json_error_response(
                501,
                "NotImplementedError",
                "CreateProvisionedProductPlan is not yet implemented in winterbaume-servicecatalog",
            ),
            "CreateProvisioningArtifact" => json_error_response(
                501,
                "NotImplementedError",
                "CreateProvisioningArtifact is not yet implemented in winterbaume-servicecatalog",
            ),
            "CreateServiceAction" => json_error_response(
                501,
                "NotImplementedError",
                "CreateServiceAction is not yet implemented in winterbaume-servicecatalog",
            ),
            "CreateTagOption" => json_error_response(
                501,
                "NotImplementedError",
                "CreateTagOption is not yet implemented in winterbaume-servicecatalog",
            ),
            "DeleteConstraint" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteConstraint is not yet implemented in winterbaume-servicecatalog",
            ),
            "DeletePortfolioShare" => json_error_response(
                501,
                "NotImplementedError",
                "DeletePortfolioShare is not yet implemented in winterbaume-servicecatalog",
            ),
            "DeleteProduct" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteProduct is not yet implemented in winterbaume-servicecatalog",
            ),
            "DeleteProvisionedProductPlan" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteProvisionedProductPlan is not yet implemented in winterbaume-servicecatalog",
            ),
            "DeleteProvisioningArtifact" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteProvisioningArtifact is not yet implemented in winterbaume-servicecatalog",
            ),
            "DeleteServiceAction" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteServiceAction is not yet implemented in winterbaume-servicecatalog",
            ),
            "DeleteTagOption" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteTagOption is not yet implemented in winterbaume-servicecatalog",
            ),
            "DescribeConstraint" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeConstraint is not yet implemented in winterbaume-servicecatalog",
            ),
            "DescribeCopyProductStatus" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeCopyProductStatus is not yet implemented in winterbaume-servicecatalog",
            ),
            "DescribePortfolioShares" => json_error_response(
                501,
                "NotImplementedError",
                "DescribePortfolioShares is not yet implemented in winterbaume-servicecatalog",
            ),
            "DescribePortfolioShareStatus" => json_error_response(
                501,
                "NotImplementedError",
                "DescribePortfolioShareStatus is not yet implemented in winterbaume-servicecatalog",
            ),
            "DescribeProduct" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeProduct is not yet implemented in winterbaume-servicecatalog",
            ),
            "DescribeProductAsAdmin" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeProductAsAdmin is not yet implemented in winterbaume-servicecatalog",
            ),
            "DescribeProductView" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeProductView is not yet implemented in winterbaume-servicecatalog",
            ),
            "DescribeProvisionedProduct" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeProvisionedProduct is not yet implemented in winterbaume-servicecatalog",
            ),
            "DescribeProvisionedProductPlan" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeProvisionedProductPlan is not yet implemented in winterbaume-servicecatalog",
            ),
            "DescribeProvisioningArtifact" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeProvisioningArtifact is not yet implemented in winterbaume-servicecatalog",
            ),
            "DescribeProvisioningParameters" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeProvisioningParameters is not yet implemented in winterbaume-servicecatalog",
            ),
            "DescribeRecord" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeRecord is not yet implemented in winterbaume-servicecatalog",
            ),
            "DescribeServiceAction" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeServiceAction is not yet implemented in winterbaume-servicecatalog",
            ),
            "DescribeServiceActionExecutionParameters" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeServiceActionExecutionParameters is not yet implemented in winterbaume-servicecatalog",
            ),
            "DescribeTagOption" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeTagOption is not yet implemented in winterbaume-servicecatalog",
            ),
            "DisableAWSOrganizationsAccess" => json_error_response(
                501,
                "NotImplementedError",
                "DisableAWSOrganizationsAccess is not yet implemented in winterbaume-servicecatalog",
            ),
            "DisassociateBudgetFromResource" => json_error_response(
                501,
                "NotImplementedError",
                "DisassociateBudgetFromResource is not yet implemented in winterbaume-servicecatalog",
            ),
            "DisassociatePrincipalFromPortfolio" => json_error_response(
                501,
                "NotImplementedError",
                "DisassociatePrincipalFromPortfolio is not yet implemented in winterbaume-servicecatalog",
            ),
            "DisassociateProductFromPortfolio" => json_error_response(
                501,
                "NotImplementedError",
                "DisassociateProductFromPortfolio is not yet implemented in winterbaume-servicecatalog",
            ),
            "DisassociateServiceActionFromProvisioningArtifact" => json_error_response(
                501,
                "NotImplementedError",
                "DisassociateServiceActionFromProvisioningArtifact is not yet implemented in winterbaume-servicecatalog",
            ),
            "DisassociateTagOptionFromResource" => json_error_response(
                501,
                "NotImplementedError",
                "DisassociateTagOptionFromResource is not yet implemented in winterbaume-servicecatalog",
            ),
            "EnableAWSOrganizationsAccess" => json_error_response(
                501,
                "NotImplementedError",
                "EnableAWSOrganizationsAccess is not yet implemented in winterbaume-servicecatalog",
            ),
            "ExecuteProvisionedProductPlan" => json_error_response(
                501,
                "NotImplementedError",
                "ExecuteProvisionedProductPlan is not yet implemented in winterbaume-servicecatalog",
            ),
            "ExecuteProvisionedProductServiceAction" => json_error_response(
                501,
                "NotImplementedError",
                "ExecuteProvisionedProductServiceAction is not yet implemented in winterbaume-servicecatalog",
            ),
            "GetAWSOrganizationsAccessStatus" => json_error_response(
                501,
                "NotImplementedError",
                "GetAWSOrganizationsAccessStatus is not yet implemented in winterbaume-servicecatalog",
            ),
            "GetProvisionedProductOutputs" => json_error_response(
                501,
                "NotImplementedError",
                "GetProvisionedProductOutputs is not yet implemented in winterbaume-servicecatalog",
            ),
            "ImportAsProvisionedProduct" => json_error_response(
                501,
                "NotImplementedError",
                "ImportAsProvisionedProduct is not yet implemented in winterbaume-servicecatalog",
            ),
            "ListAcceptedPortfolioShares" => json_error_response(
                501,
                "NotImplementedError",
                "ListAcceptedPortfolioShares is not yet implemented in winterbaume-servicecatalog",
            ),
            "ListBudgetsForResource" => json_error_response(
                501,
                "NotImplementedError",
                "ListBudgetsForResource is not yet implemented in winterbaume-servicecatalog",
            ),
            "ListConstraintsForPortfolio" => json_error_response(
                501,
                "NotImplementedError",
                "ListConstraintsForPortfolio is not yet implemented in winterbaume-servicecatalog",
            ),
            "ListLaunchPaths" => json_error_response(
                501,
                "NotImplementedError",
                "ListLaunchPaths is not yet implemented in winterbaume-servicecatalog",
            ),
            "ListOrganizationPortfolioAccess" => json_error_response(
                501,
                "NotImplementedError",
                "ListOrganizationPortfolioAccess is not yet implemented in winterbaume-servicecatalog",
            ),
            "ListPortfolioAccess" => json_error_response(
                501,
                "NotImplementedError",
                "ListPortfolioAccess is not yet implemented in winterbaume-servicecatalog",
            ),
            "ListPortfoliosForProduct" => json_error_response(
                501,
                "NotImplementedError",
                "ListPortfoliosForProduct is not yet implemented in winterbaume-servicecatalog",
            ),
            "ListPrincipalsForPortfolio" => json_error_response(
                501,
                "NotImplementedError",
                "ListPrincipalsForPortfolio is not yet implemented in winterbaume-servicecatalog",
            ),
            "ListProvisionedProductPlans" => json_error_response(
                501,
                "NotImplementedError",
                "ListProvisionedProductPlans is not yet implemented in winterbaume-servicecatalog",
            ),
            "ListProvisioningArtifacts" => json_error_response(
                501,
                "NotImplementedError",
                "ListProvisioningArtifacts is not yet implemented in winterbaume-servicecatalog",
            ),
            "ListProvisioningArtifactsForServiceAction" => json_error_response(
                501,
                "NotImplementedError",
                "ListProvisioningArtifactsForServiceAction is not yet implemented in winterbaume-servicecatalog",
            ),
            "ListRecordHistory" => json_error_response(
                501,
                "NotImplementedError",
                "ListRecordHistory is not yet implemented in winterbaume-servicecatalog",
            ),
            "ListResourcesForTagOption" => json_error_response(
                501,
                "NotImplementedError",
                "ListResourcesForTagOption is not yet implemented in winterbaume-servicecatalog",
            ),
            "ListServiceActions" => json_error_response(
                501,
                "NotImplementedError",
                "ListServiceActions is not yet implemented in winterbaume-servicecatalog",
            ),
            "ListServiceActionsForProvisioningArtifact" => json_error_response(
                501,
                "NotImplementedError",
                "ListServiceActionsForProvisioningArtifact is not yet implemented in winterbaume-servicecatalog",
            ),
            "ListStackInstancesForProvisionedProduct" => json_error_response(
                501,
                "NotImplementedError",
                "ListStackInstancesForProvisionedProduct is not yet implemented in winterbaume-servicecatalog",
            ),
            "ListTagOptions" => json_error_response(
                501,
                "NotImplementedError",
                "ListTagOptions is not yet implemented in winterbaume-servicecatalog",
            ),
            "NotifyProvisionProductEngineWorkflowResult" => json_error_response(
                501,
                "NotImplementedError",
                "NotifyProvisionProductEngineWorkflowResult is not yet implemented in winterbaume-servicecatalog",
            ),
            "NotifyTerminateProvisionedProductEngineWorkflowResult" => json_error_response(
                501,
                "NotImplementedError",
                "NotifyTerminateProvisionedProductEngineWorkflowResult is not yet implemented in winterbaume-servicecatalog",
            ),
            "NotifyUpdateProvisionedProductEngineWorkflowResult" => json_error_response(
                501,
                "NotImplementedError",
                "NotifyUpdateProvisionedProductEngineWorkflowResult is not yet implemented in winterbaume-servicecatalog",
            ),
            "ProvisionProduct" => json_error_response(
                501,
                "NotImplementedError",
                "ProvisionProduct is not yet implemented in winterbaume-servicecatalog",
            ),
            "RejectPortfolioShare" => json_error_response(
                501,
                "NotImplementedError",
                "RejectPortfolioShare is not yet implemented in winterbaume-servicecatalog",
            ),
            "ScanProvisionedProducts" => json_error_response(
                501,
                "NotImplementedError",
                "ScanProvisionedProducts is not yet implemented in winterbaume-servicecatalog",
            ),
            "SearchProducts" => json_error_response(
                501,
                "NotImplementedError",
                "SearchProducts is not yet implemented in winterbaume-servicecatalog",
            ),
            "SearchProductsAsAdmin" => json_error_response(
                501,
                "NotImplementedError",
                "SearchProductsAsAdmin is not yet implemented in winterbaume-servicecatalog",
            ),
            "SearchProvisionedProducts" => json_error_response(
                501,
                "NotImplementedError",
                "SearchProvisionedProducts is not yet implemented in winterbaume-servicecatalog",
            ),
            "TerminateProvisionedProduct" => json_error_response(
                501,
                "NotImplementedError",
                "TerminateProvisionedProduct is not yet implemented in winterbaume-servicecatalog",
            ),
            "UpdateConstraint" => json_error_response(
                501,
                "NotImplementedError",
                "UpdateConstraint is not yet implemented in winterbaume-servicecatalog",
            ),
            "UpdatePortfolio" => json_error_response(
                501,
                "NotImplementedError",
                "UpdatePortfolio is not yet implemented in winterbaume-servicecatalog",
            ),
            "UpdatePortfolioShare" => json_error_response(
                501,
                "NotImplementedError",
                "UpdatePortfolioShare is not yet implemented in winterbaume-servicecatalog",
            ),
            "UpdateProduct" => json_error_response(
                501,
                "NotImplementedError",
                "UpdateProduct is not yet implemented in winterbaume-servicecatalog",
            ),
            "UpdateProvisionedProduct" => json_error_response(
                501,
                "NotImplementedError",
                "UpdateProvisionedProduct is not yet implemented in winterbaume-servicecatalog",
            ),
            "UpdateProvisionedProductProperties" => json_error_response(
                501,
                "NotImplementedError",
                "UpdateProvisionedProductProperties is not yet implemented in winterbaume-servicecatalog",
            ),
            "UpdateProvisioningArtifact" => json_error_response(
                501,
                "NotImplementedError",
                "UpdateProvisioningArtifact is not yet implemented in winterbaume-servicecatalog",
            ),
            "UpdateServiceAction" => json_error_response(
                501,
                "NotImplementedError",
                "UpdateServiceAction is not yet implemented in winterbaume-servicecatalog",
            ),
            "UpdateTagOption" => json_error_response(
                501,
                "NotImplementedError",
                "UpdateTagOption is not yet implemented in winterbaume-servicecatalog",
            ),
            _ => json_error_response(400, "InvalidAction", &format!("Unknown operation {action}")),
        };
        if response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_create_portfolio(
        &self,
        state: &Arc<tokio::sync::RwLock<ServiceCatalogState>>,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_portfolio_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidParametersException", &e),
        };
        if input.display_name.is_empty() {
            return json_error_response(
                400,
                "InvalidParametersException",
                "DisplayName is required",
            );
        }
        if input.provider_name.is_empty() {
            return json_error_response(
                400,
                "InvalidParametersException",
                "ProviderName is required",
            );
        }
        let display_name = input.display_name.as_str();
        let provider_name = input.provider_name.as_str();
        let description = input.description.as_deref().unwrap_or("");
        // Distinguish "absent" from "empty string" by re-parsing the raw body
        // to preserve the previous None-vs-Some(empty) semantics for idempotency.
        let idempotency_token: Option<&str> = if input.idempotency_token.is_empty() {
            None
        } else {
            Some(input.idempotency_token.as_str())
        };
        let tags: Vec<crate::types::PortfolioTag> = input
            .tags
            .as_deref()
            .map(|arr| {
                arr.iter()
                    .map(|t| crate::types::PortfolioTag {
                        key: t.key.clone(),
                        value: t.value.clone(),
                    })
                    .collect()
            })
            .unwrap_or_default();

        let mut state = state.write().await;
        match state.create_portfolio(
            display_name,
            description,
            provider_name,
            tags,
            idempotency_token,
            account_id,
            region,
        ) {
            Ok(portfolio) => {
                let wire_tags: Vec<wire::Tag> = portfolio
                    .tags
                    .iter()
                    .map(|t| wire::Tag {
                        key: t.key.clone(),
                        value: t.value.clone(),
                    })
                    .collect();
                wire::serialize_create_portfolio_response(&wire::CreatePortfolioOutput {
                    portfolio_detail: Some(build_wire_portfolio_detail(&portfolio)),
                    tags: Some(wire_tags),
                })
            }
            Err(e) => sc_error_response(&e),
        }
    }

    async fn handle_describe_portfolio(
        &self,
        state: &Arc<tokio::sync::RwLock<ServiceCatalogState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_portfolio_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidParametersException", &e),
        };
        if input.id.is_empty() {
            return json_error_response(400, "InvalidParametersException", "Id is required");
        }
        let id = input.id.as_str();

        let state = state.read().await;
        match state.describe_portfolio(id) {
            Ok(portfolio) => {
                let wire_tags: Vec<wire::Tag> = portfolio
                    .tags
                    .iter()
                    .map(|t| wire::Tag {
                        key: t.key.clone(),
                        value: t.value.clone(),
                    })
                    .collect();
                wire::serialize_describe_portfolio_response(&wire::DescribePortfolioOutput {
                    portfolio_detail: Some(build_wire_portfolio_detail(portfolio)),
                    tags: Some(wire_tags),
                    tag_options: Some(vec![]),
                    budgets: Some(vec![]),
                })
            }
            Err(e) => sc_error_response(&e),
        }
    }

    async fn handle_delete_portfolio(
        &self,
        state: &Arc<tokio::sync::RwLock<ServiceCatalogState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_portfolio_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "InvalidParametersException", &e),
        };
        if input.id.is_empty() {
            return json_error_response(400, "InvalidParametersException", "Id is required");
        }
        let id = input.id.as_str();

        let mut state = state.write().await;
        match state.delete_portfolio(id) {
            Ok(()) => wire::serialize_delete_portfolio_response(&wire::DeletePortfolioOutput {}),
            Err(e) => sc_error_response(&e),
        }
    }

    async fn handle_list_portfolios(
        &self,
        state: &Arc<tokio::sync::RwLock<ServiceCatalogState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let portfolios = state.list_portfolios();
        let entries: Vec<wire::PortfolioDetail> = portfolios
            .iter()
            .map(|p| build_wire_portfolio_detail(p))
            .collect();

        wire::serialize_list_portfolios_response(&wire::ListPortfoliosOutput {
            portfolio_details: Some(entries),
            ..Default::default()
        })
    }
}

fn build_wire_portfolio_detail(p: &crate::types::PortfolioDetail) -> wire::PortfolioDetail {
    wire::PortfolioDetail {
        id: Some(p.id.clone()),
        a_r_n: Some(p.arn.clone()),
        display_name: Some(p.display_name.clone()),
        description: Some(p.description.clone()),
        created_time: Some(p.created_time.timestamp() as f64),
        provider_name: Some(p.provider_name.clone()),
    }
}

fn sc_error_response(err: &ServiceCatalogError) -> MockResponse {
    match err {
        ServiceCatalogError::ResourceNotFound { .. } => MockResponse::json(
            400,
            json!({"__type": "ResourceNotFoundException", "message": err.to_string()}).to_string(),
        ),
    }
}

fn json_error_response(status: u16, error_type: &str, message: &str) -> MockResponse {
    MockResponse::json(
        status,
        json!({"__type": error_type, "message": message}).to_string(),
    )
}
