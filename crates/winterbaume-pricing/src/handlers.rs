use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, default_account_id,
};

use crate::state::PricingState;
use crate::views::PricingStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct PricingService {
    pub(crate) state: Arc<BackendState<PricingState>>,
    pub(crate) notifier: StateChangeNotifier<PricingStateView>,
}

impl PricingService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for PricingService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for PricingService {
    fn service_name(&self) -> &str {
        "api.pricing"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://api\.pricing\..*\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

type SharedState = Arc<tokio::sync::RwLock<PricingState>>;

impl PricingService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        // Pricing is a global service — account scope is not meaningful; use the default.
        let account_id = default_account_id();

        let action = request
            .headers
            .get("x-amz-target")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.split('.').next_back())
            .map(|s| s.to_string());
        let action = match action {
            Some(a) => a,
            None => return aws_json_error(400, "MissingAction", "Missing X-Amz-Target"),
        };

        let body: Value = if request.body.is_empty() {
            json!({})
        } else {
            match serde_json::from_slice(&request.body) {
                Ok(v) => v,
                Err(_) => {
                    return aws_json_error(400, "SerializationException", "Invalid JSON body");
                }
            }
        };

        let state = self.state.get(account_id, &region);

        let response = match action.as_str() {
            "DescribeServices" => self.handle_describe_services(&state, &body).await,
            "GetAttributeValues" => self.handle_get_attribute_values(&state, &body).await,
            "GetPriceListFileUrl" => self.handle_get_price_list_file_url(&state, &body).await,
            "GetProducts" => self.handle_get_products(&state, &body).await,
            "ListPriceLists" => self.handle_list_price_lists(&state, &body).await,
            other => aws_json_error(
                400,
                "UnknownOperationException",
                &format!("Unknown action: {other}"),
            ),
        };

        // Pricing is read-only, but record_call mutated state, so always notify on 2xx.
        use winterbaume_core::StatefulService;
        if response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_describe_services(&self, state: &SharedState, body: &Value) -> MockResponse {
        let mut state = state.write().await;
        state.record_call("DescribeServices");
        let service_code = body.get("ServiceCode").and_then(|v| v.as_str());
        let services: Vec<wire::Service> = match service_code {
            Some(code) if !code.is_empty() => vec![mock_service(code)],
            _ => CATALOGUE.iter().map(|c| mock_service(c)).collect(),
        };
        wire::serialize_describe_services_response(&wire::DescribeServicesResponse {
            format_version: Some("aws_v1".to_string()),
            next_token: None,
            services: Some(services),
        })
    }

    async fn handle_get_attribute_values(&self, state: &SharedState, body: &Value) -> MockResponse {
        let attribute = body
            .get("AttributeName")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let mut state = state.write().await;
        state.record_call("GetAttributeValues");
        let values: Vec<wire::AttributeValue> = match attribute {
            "instanceType" => ["t2.micro", "t3.medium", "m5.large", "c5.xlarge"]
                .iter()
                .map(|v| wire::AttributeValue {
                    value: Some((*v).to_string()),
                })
                .collect(),
            "location" => ["US East (N. Virginia)", "US West (Oregon)", "EU (Ireland)"]
                .iter()
                .map(|v| wire::AttributeValue {
                    value: Some((*v).to_string()),
                })
                .collect(),
            _ => vec![],
        };
        wire::serialize_get_attribute_values_response(&wire::GetAttributeValuesResponse {
            attribute_values: Some(values),
            next_token: None,
        })
    }

    async fn handle_get_price_list_file_url(
        &self,
        state: &SharedState,
        body: &Value,
    ) -> MockResponse {
        let arn = body
            .get("PriceListArn")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let format = body
            .get("FileFormat")
            .and_then(|v| v.as_str())
            .unwrap_or("json");
        let mut state = state.write().await;
        state.record_call("GetPriceListFileUrl");
        let url = format!(
            "https://pricing.us-east-1.amazonaws.com/offers/v1.0/aws/{}/index.{}",
            arn.split('/').next_back().unwrap_or("price-list"),
            format.to_lowercase()
        );
        wire::serialize_get_price_list_file_url_response(&wire::GetPriceListFileUrlResponse {
            url: Some(url),
        })
    }

    async fn handle_get_products(&self, state: &SharedState, body: &Value) -> MockResponse {
        let service_code = body
            .get("ServiceCode")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let mut state = state.write().await;
        state.record_call("GetProducts");
        // Each entry in PriceList is a stringified JSON document; mock with a
        // minimal product per service code.
        let product = json!({
            "product": {
                "sku": format!("MOCK-{service_code}-001"),
                "productFamily": service_code,
                "attributes": {"servicecode": service_code},
            },
            "version": "20240101000000",
            "publicationDate": "2024-01-01T00:00:00Z",
        })
        .to_string();
        wire::serialize_get_products_response(&wire::GetProductsResponse {
            format_version: Some("aws_v1".to_string()),
            next_token: None,
            price_list: Some(vec![product]),
        })
    }

    async fn handle_list_price_lists(&self, state: &SharedState, body: &Value) -> MockResponse {
        let service_code = body
            .get("ServiceCode")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let currency = body
            .get("CurrencyCode")
            .and_then(|v| v.as_str())
            .unwrap_or("USD");
        let region = body.get("RegionCode").and_then(|v| v.as_str());
        let mut state = state.write().await;
        state.record_call("ListPriceLists");
        let price_list = wire::PriceList {
            currency_code: Some(currency.to_string()),
            file_formats: Some(vec!["json".to_string(), "csv".to_string()]),
            price_list_arn: Some(format!(
                "arn:aws:pricing:::price-list/{service_code}/{}/{currency}",
                region.unwrap_or("us-east-1")
            )),
            region_code: region.map(String::from),
        };
        wire::serialize_list_price_lists_response(&wire::ListPriceListsResponse {
            next_token: None,
            price_lists: Some(vec![price_list]),
        })
    }
}

const CATALOGUE: &[&str] = &[
    "AmazonEC2",
    "AmazonS3",
    "AmazonRDS",
    "AWSLambda",
    "AmazonDynamoDB",
];

fn mock_service(code: &str) -> wire::Service {
    let attributes: Vec<String> = match code {
        "AmazonEC2" => vec![
            "instanceType".into(),
            "location".into(),
            "operatingSystem".into(),
            "tenancy".into(),
        ],
        "AmazonS3" => vec!["storageClass".into(), "location".into()],
        "AmazonRDS" => vec![
            "instanceType".into(),
            "engineCode".into(),
            "location".into(),
        ],
        _ => vec!["location".into()],
    };
    wire::Service {
        attribute_names: Some(attributes),
        service_code: Some(code.to_string()),
    }
}

fn aws_json_error(status: u16, error_type: &str, message: &str) -> MockResponse {
    let body = json!({"__type": error_type, "Message": message});
    let mut resp = MockResponse::json(status, body.to_string());
    resp.headers
        .insert(X_AMZN_ERRORTYPE, error_type.parse().unwrap());
    resp
}
