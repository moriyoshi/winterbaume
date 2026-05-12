use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, StatefulService,
    default_account_id,
};

use crate::state::{ServiceQuotasError, ServiceQuotasState};
use crate::views::ServiceQuotasStateView;
use crate::wire;

pub struct ServiceQuotasService {
    pub(crate) state: Arc<BackendState<ServiceQuotasState>>,
    pub(crate) notifier: StateChangeNotifier<ServiceQuotasStateView>,
}

impl ServiceQuotasService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for ServiceQuotasService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for ServiceQuotasService {
    fn service_name(&self) -> &str {
        "servicequotas"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://servicequotas\..*\.amazonaws\.com",
            r"https?://servicequotas\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl ServiceQuotasService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();

        let action = request
            .headers
            .get("x-amz-target")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.split('.').next_back())
            .map(|s| s.to_string());

        let action = match action {
            Some(a) => a,
            None => {
                return error_response(400, "MissingAction", "Missing X-Amz-Target header");
            }
        };

        if serde_json::from_slice::<Value>(&request.body).is_err() {
            return error_response(400, "SerializationException", "Invalid JSON body");
        }
        let body_bytes: &[u8] = &request.body;

        let state = self.state.get(account_id, &region);

        let response = match action.as_str() {
            "GetServiceQuota" => {
                self.handle_get_service_quota(&state, body_bytes, &region, account_id)
                    .await
            }
            "ListServiceQuotas" => {
                self.handle_list_service_quotas(&state, body_bytes, &region, account_id)
                    .await
            }
            "GetAWSDefaultServiceQuota" => {
                self.handle_get_aws_default_service_quota(&state, body_bytes, &region, account_id)
                    .await
            }
            "ListServices" => self.handle_list_services(&state).await,
            // --- Unimplemented operations (auto-generated stubs) ---
            "AssociateServiceQuotaTemplate" => error_response(
                501,
                "NotImplementedError",
                "AssociateServiceQuotaTemplate is not yet implemented in winterbaume-servicequotas",
            ),
            "CreateSupportCase" => error_response(
                501,
                "NotImplementedError",
                "CreateSupportCase is not yet implemented in winterbaume-servicequotas",
            ),
            "DeleteServiceQuotaIncreaseRequestFromTemplate" => error_response(
                501,
                "NotImplementedError",
                "DeleteServiceQuotaIncreaseRequestFromTemplate is not yet implemented in winterbaume-servicequotas",
            ),
            "DisassociateServiceQuotaTemplate" => error_response(
                501,
                "NotImplementedError",
                "DisassociateServiceQuotaTemplate is not yet implemented in winterbaume-servicequotas",
            ),
            "GetAssociationForServiceQuotaTemplate" => error_response(
                501,
                "NotImplementedError",
                "GetAssociationForServiceQuotaTemplate is not yet implemented in winterbaume-servicequotas",
            ),
            "GetAutoManagementConfiguration" => error_response(
                501,
                "NotImplementedError",
                "GetAutoManagementConfiguration is not yet implemented in winterbaume-servicequotas",
            ),
            "GetQuotaUtilizationReport" => error_response(
                501,
                "NotImplementedError",
                "GetQuotaUtilizationReport is not yet implemented in winterbaume-servicequotas",
            ),
            "GetRequestedServiceQuotaChange" => error_response(
                501,
                "NotImplementedError",
                "GetRequestedServiceQuotaChange is not yet implemented in winterbaume-servicequotas",
            ),
            "GetServiceQuotaIncreaseRequestFromTemplate" => error_response(
                501,
                "NotImplementedError",
                "GetServiceQuotaIncreaseRequestFromTemplate is not yet implemented in winterbaume-servicequotas",
            ),
            "ListAWSDefaultServiceQuotas" => {
                self.handle_list_aws_default_service_quotas(&state, body_bytes, &region, account_id)
                    .await
            }
            "ListRequestedServiceQuotaChangeHistory" => error_response(
                501,
                "NotImplementedError",
                "ListRequestedServiceQuotaChangeHistory is not yet implemented in winterbaume-servicequotas",
            ),
            "ListRequestedServiceQuotaChangeHistoryByQuota" => error_response(
                501,
                "NotImplementedError",
                "ListRequestedServiceQuotaChangeHistoryByQuota is not yet implemented in winterbaume-servicequotas",
            ),
            "ListServiceQuotaIncreaseRequestsInTemplate" => error_response(
                501,
                "NotImplementedError",
                "ListServiceQuotaIncreaseRequestsInTemplate is not yet implemented in winterbaume-servicequotas",
            ),
            "ListTagsForResource" => error_response(
                501,
                "NotImplementedError",
                "ListTagsForResource is not yet implemented in winterbaume-servicequotas",
            ),
            "PutServiceQuotaIncreaseRequestIntoTemplate" => error_response(
                501,
                "NotImplementedError",
                "PutServiceQuotaIncreaseRequestIntoTemplate is not yet implemented in winterbaume-servicequotas",
            ),
            "RequestServiceQuotaIncrease" => error_response(
                501,
                "NotImplementedError",
                "RequestServiceQuotaIncrease is not yet implemented in winterbaume-servicequotas",
            ),
            "StartAutoManagement" => error_response(
                501,
                "NotImplementedError",
                "StartAutoManagement is not yet implemented in winterbaume-servicequotas",
            ),
            "StartQuotaUtilizationReport" => error_response(
                501,
                "NotImplementedError",
                "StartQuotaUtilizationReport is not yet implemented in winterbaume-servicequotas",
            ),
            "StopAutoManagement" => error_response(
                501,
                "NotImplementedError",
                "StopAutoManagement is not yet implemented in winterbaume-servicequotas",
            ),
            "TagResource" => error_response(
                501,
                "NotImplementedError",
                "TagResource is not yet implemented in winterbaume-servicequotas",
            ),
            "UntagResource" => error_response(
                501,
                "NotImplementedError",
                "UntagResource is not yet implemented in winterbaume-servicequotas",
            ),
            "UpdateAutoManagement" => error_response(
                501,
                "NotImplementedError",
                "UpdateAutoManagement is not yet implemented in winterbaume-servicequotas",
            ),
            _ => error_response(400, "InvalidAction", &format!("Unknown operation {action}")),
        };
        if response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_get_service_quota(
        &self,
        state: &Arc<tokio::sync::RwLock<ServiceQuotasState>>,
        body: &[u8],
        region: &str,
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_get_service_quota_request(body) {
            Ok(v) => v,
            Err(e) => return error_response(400, "IllegalArgumentException", &e),
        };
        if input.service_code.is_empty() {
            return error_response(400, "IllegalArgumentException", "ServiceCode is required");
        }
        let service_code = input.service_code.as_str();
        if input.quota_code.is_empty() {
            return error_response(400, "IllegalArgumentException", "QuotaCode is required");
        }
        let quota_code = input.quota_code.as_str();

        let mut state = state.write().await;
        // Seed default quotas if none exist for this service
        seed_defaults_if_needed(&mut state, service_code, region, account_id);

        match state.get_quota(service_code, quota_code) {
            Ok(q) => {
                wire::serialize_get_service_quota_response(&crate::model::GetServiceQuotaResponse {
                    quota: Some(quota_entry_to_model(q)),
                })
            }
            Err(e) => service_quotas_error_response(&e),
        }
    }

    async fn handle_list_service_quotas(
        &self,
        state: &Arc<tokio::sync::RwLock<ServiceQuotasState>>,
        body: &[u8],
        region: &str,
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_list_service_quotas_request(body) {
            Ok(v) => v,
            Err(e) => return error_response(400, "IllegalArgumentException", &e),
        };
        if input.service_code.is_empty() {
            return error_response(400, "IllegalArgumentException", "ServiceCode is required");
        }
        let service_code = input.service_code.as_str();

        let mut state = state.write().await;
        seed_defaults_if_needed(&mut state, service_code, region, account_id);

        let quotas = state.list_quotas(service_code);
        let entries: Vec<crate::model::ServiceQuota> =
            quotas.iter().map(|q| quota_entry_to_model(q)).collect();

        wire::serialize_list_service_quotas_response(&crate::model::ListServiceQuotasResponse {
            next_token: None,
            quotas: Some(entries),
        })
    }

    async fn handle_get_aws_default_service_quota(
        &self,
        state: &Arc<tokio::sync::RwLock<ServiceQuotasState>>,
        body: &[u8],
        region: &str,
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_get_a_w_s_default_service_quota_request(body) {
            Ok(v) => v,
            Err(e) => return error_response(400, "IllegalArgumentException", &e),
        };
        if input.service_code.is_empty() {
            return error_response(400, "IllegalArgumentException", "ServiceCode is required");
        }
        let service_code = input.service_code.as_str();
        if input.quota_code.is_empty() {
            return error_response(400, "IllegalArgumentException", "QuotaCode is required");
        }
        let quota_code = input.quota_code.as_str();

        let mut state = state.write().await;
        seed_defaults_if_needed(&mut state, service_code, region, account_id);

        match state.get_quota(service_code, quota_code) {
            Ok(q) => wire::serialize_get_a_w_s_default_service_quota_response(
                &crate::model::GetAWSDefaultServiceQuotaResponse {
                    quota: Some(quota_entry_to_model(q)),
                },
            ),
            Err(e) => service_quotas_error_response(&e),
        }
    }

    async fn handle_list_services(
        &self,
        state: &Arc<tokio::sync::RwLock<ServiceQuotasState>>,
    ) -> MockResponse {
        let mut state = state.write().await;
        seed_services_if_needed(&mut state);

        let services = state.list_services();
        let entries: Vec<crate::model::ServiceInfo> = services
            .iter()
            .map(|s| crate::model::ServiceInfo {
                service_code: Some(s.service_code.clone()),
                service_name: Some(s.service_name.clone()),
            })
            .collect();

        wire::serialize_list_services_response(&crate::model::ListServicesResponse {
            next_token: None,
            services: Some(entries),
        })
    }

    async fn handle_list_aws_default_service_quotas(
        &self,
        state: &Arc<tokio::sync::RwLock<ServiceQuotasState>>,
        body: &[u8],
        region: &str,
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_list_a_w_s_default_service_quotas_request(body) {
            Ok(v) => v,
            Err(e) => return error_response(400, "IllegalArgumentException", &e),
        };
        if input.service_code.is_empty() {
            return error_response(400, "IllegalArgumentException", "ServiceCode is required");
        }
        let service_code = input.service_code.as_str();

        let mut state = state.write().await;
        seed_defaults_if_needed(&mut state, service_code, region, account_id);

        let quotas = state.list_quotas(service_code);
        let entries: Vec<crate::model::ServiceQuota> =
            quotas.iter().map(|q| quota_entry_to_model(q)).collect();

        wire::serialize_list_a_w_s_default_service_quotas_response(
            &crate::model::ListAWSDefaultServiceQuotasResponse {
                next_token: None,
                quotas: Some(entries),
            },
        )
    }
}

/// Convert a state ServiceQuotaEntry to a wire model ServiceQuota.
fn quota_entry_to_model(q: &crate::types::ServiceQuotaEntry) -> crate::model::ServiceQuota {
    crate::model::ServiceQuota {
        service_code: Some(q.service_code.clone()),
        service_name: Some(q.service_name.clone()),
        quota_arn: Some(q.quota_arn.clone()),
        quota_code: Some(q.quota_code.clone()),
        quota_name: Some(q.quota_name.clone()),
        value: Some(q.value),
        unit: Some(q.unit.clone()),
        adjustable: Some(q.adjustable),
        global_quota: Some(q.global_quota),
        description: Some(q.description.clone()),
        ..Default::default()
    }
}

fn error_response(status: u16, error_type: &str, message: &str) -> MockResponse {
    MockResponse::json(
        status,
        json!({"__type": error_type, "message": message}).to_string(),
    )
}

fn service_quotas_error_response(err: &ServiceQuotasError) -> MockResponse {
    match err {
        ServiceQuotasError::NoSuchResource { .. } => {
            error_response(404, "NoSuchResourceException", &err.to_string())
        }
    }
}

/// Seeds a small set of default quotas for the given service code
/// so that GetServiceQuota and ListServiceQuotas return useful results.
fn seed_defaults_if_needed(
    state: &mut ServiceQuotasState,
    service_code: &str,
    region: &str,
    account_id: &str,
) {
    // If we already have quotas for this service, do nothing
    if state
        .quotas
        .values()
        .any(|q| q.service_code == service_code)
    {
        return;
    }

    let service_name_fallback = format!("{service_code} service");

    // Provide some common defaults per service
    let defaults: Vec<(&str, &str, &str, f64, &str, bool, &str)> = match service_code {
        "ec2" => vec![(
            "L-1216C47A",
            "Running On-Demand Standard (A, C, D, H, I, M, R, T, Z) instances",
            "Amazon Elastic Compute Cloud (Amazon EC2)",
            1152.0,
            "None",
            true,
            "Maximum number of vCPUs assigned to the Running On-Demand Standard instances",
        )],
        "s3" => vec![(
            "L-DC2B2D3D",
            "Buckets",
            "Amazon Simple Storage Service (Amazon S3)",
            100.0,
            "None",
            true,
            "The maximum number of S3 buckets that you can create in your account",
        )],
        "lambda" => vec![(
            "L-B99A9384",
            "Concurrent executions",
            "AWS Lambda",
            1000.0,
            "None",
            true,
            "The maximum number of concurrent function executions",
        )],
        _ => vec![(
            "L-F678F1CE",
            "Default quota",
            &service_name_fallback,
            100.0,
            "None",
            true,
            "Default service quota",
        )],
    };

    for (quota_code, quota_name, service_name, value, unit, adjustable, description) in defaults {
        state.add_quota(
            service_code,
            service_name,
            quota_code,
            quota_name,
            value,
            unit,
            adjustable,
            false,
            description,
            region,
            account_id,
        );
    }
}

/// Seeds a minimal set of services so ListServices returns results.
fn seed_services_if_needed(state: &mut ServiceQuotasState) {
    if !state.services.is_empty() {
        return;
    }

    let default_services = [
        ("ec2", "Amazon Elastic Compute Cloud (Amazon EC2)"),
        ("s3", "Amazon Simple Storage Service (Amazon S3)"),
        ("lambda", "AWS Lambda"),
        ("dynamodb", "Amazon DynamoDB"),
        ("iam", "AWS Identity and Access Management (IAM)"),
        ("sqs", "Amazon Simple Queue Service (Amazon SQS)"),
        ("sns", "Amazon Simple Notification Service (Amazon SNS)"),
        ("cloudformation", "AWS CloudFormation"),
    ];

    for (code, name) in default_services {
        state.add_service(code, name);
    }
}
