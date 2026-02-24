use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, StatefulService,
};

use crate::state::{SupportError, SupportState};
use crate::views::SupportStateView;
use crate::wire;

fn json_error_response(status: u16, error_type: &str, message: &str) -> MockResponse {
    MockResponse::json(
        status,
        json!({"__type": error_type, "message": message}).to_string(),
    )
}

pub struct SupportService {
    pub(crate) state: Arc<BackendState<SupportState>>,
    pub(crate) notifier: StateChangeNotifier<SupportStateView>,
}

impl SupportService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for SupportService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for SupportService {
    fn service_name(&self) -> &str {
        "support"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://support\..*\.amazonaws\.com",
            r"https?://support\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl SupportService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = winterbaume_core::DEFAULT_ACCOUNT_ID;

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
            "CreateCase" => self.handle_create_case(&state, body_bytes).await,
            "DescribeCases" => self.handle_describe_cases(&state, body_bytes).await,
            "ResolveCase" => self.handle_resolve_case(&state, body_bytes).await,
            "DescribeServices" => self.handle_describe_services(&state, body_bytes).await,
            "DescribeTrustedAdvisorChecks" => {
                self.handle_describe_trusted_advisor_checks(body_bytes)
                    .await
            }
            "RefreshTrustedAdvisorCheck" => {
                self.handle_refresh_trusted_advisor_check(&state, body_bytes)
                    .await
            }
            // --- Unimplemented operations (auto-generated stubs) ---
            "AddAttachmentsToSet" => json_error_response(
                501,
                "NotImplementedError",
                "AddAttachmentsToSet is not yet implemented in winterbaume-support",
            ),
            "AddCommunicationToCase" => json_error_response(
                501,
                "NotImplementedError",
                "AddCommunicationToCase is not yet implemented in winterbaume-support",
            ),
            "DescribeAttachment" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeAttachment is not yet implemented in winterbaume-support",
            ),
            "DescribeCommunications" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeCommunications is not yet implemented in winterbaume-support",
            ),
            "DescribeCreateCaseOptions" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeCreateCaseOptions is not yet implemented in winterbaume-support",
            ),
            "DescribeSeverityLevels" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeSeverityLevels is not yet implemented in winterbaume-support",
            ),
            "DescribeSupportedLanguages" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeSupportedLanguages is not yet implemented in winterbaume-support",
            ),
            "DescribeTrustedAdvisorCheckRefreshStatuses" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeTrustedAdvisorCheckRefreshStatuses is not yet implemented in winterbaume-support",
            ),
            "DescribeTrustedAdvisorCheckResult" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeTrustedAdvisorCheckResult is not yet implemented in winterbaume-support",
            ),
            "DescribeTrustedAdvisorCheckSummaries" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeTrustedAdvisorCheckSummaries is not yet implemented in winterbaume-support",
            ),
            _ => json_error_response(400, "InvalidAction", &format!("Unknown operation {action}")),
        };
        if response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_create_case(
        &self,
        state: &Arc<tokio::sync::RwLock<SupportState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_create_case_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.subject.is_empty() {
            return json_error_response(400, "InvalidParameterValue", "subject is required");
        }
        if input.communication_body.is_empty() {
            return json_error_response(
                400,
                "InvalidParameterValue",
                "communicationBody is required",
            );
        }
        let subject = input.subject.as_str();
        let communication_body = input.communication_body.as_str();
        let service_code = input.service_code.as_deref();
        let severity_code = input.severity_code.as_deref();
        let category_code = input.category_code.as_deref();
        let language = input.language.as_deref();
        let cc_email_addresses: Vec<String> = input.cc_email_addresses.unwrap_or_default();

        let mut state = state.write().await;
        match state.create_case(
            subject,
            communication_body,
            service_code,
            severity_code,
            category_code,
            cc_email_addresses,
            language,
        ) {
            Ok(case) => wire::serialize_create_case_response(&wire::CreateCaseResponse {
                case_id: Some(case.case_id.clone()),
            }),
            Err(e) => support_error_response(&e),
        }
    }

    async fn handle_describe_cases(
        &self,
        state: &Arc<tokio::sync::RwLock<SupportState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_cases_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let case_id_list = input.case_id_list;
        let include_resolved = input.include_resolved_cases.unwrap_or(false);

        let state = state.read().await;
        let cases = state.describe_cases(case_id_list.as_deref(), include_resolved);

        let entries: Vec<wire::CaseDetails> = cases
            .iter()
            .map(|c| wire::CaseDetails {
                case_id: Some(c.case_id.clone()),
                display_id: Some(c.display_id.clone()),
                subject: Some(c.subject.clone()),
                status: Some(c.status.clone()),
                service_code: Some(c.service_code.clone()),
                category_code: Some(c.category_code.clone()),
                severity_code: Some(c.severity_code.clone()),
                submitted_by: Some(c.submitted_by.clone()),
                time_created: Some(c.time_created.clone()),
                cc_email_addresses: Some(c.cc_email_addresses.clone()),
                language: Some(c.language.clone()),
                ..Default::default()
            })
            .collect();

        wire::serialize_describe_cases_response(&wire::DescribeCasesResponse {
            cases: Some(entries),
            ..Default::default()
        })
    }

    async fn handle_resolve_case(
        &self,
        state: &Arc<tokio::sync::RwLock<SupportState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_resolve_case_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let case_id = input.case_id.as_deref();

        let mut state = state.write().await;
        match state.resolve_case(case_id) {
            Ok((initial, final_status)) => {
                wire::serialize_resolve_case_response(&wire::ResolveCaseResponse {
                    initial_case_status: Some(initial),
                    final_case_status: Some(final_status),
                })
            }
            Err(e) => support_error_response(&e),
        }
    }

    async fn handle_describe_services(
        &self,
        state: &Arc<tokio::sync::RwLock<SupportState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_describe_services_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let service_code_list = input.service_code_list;

        let state = state.read().await;
        let services = state.describe_services(service_code_list.as_deref());

        let entries: Vec<wire::Service> = services
            .iter()
            .map(|s| wire::Service {
                code: Some(s.code.clone()),
                name: Some(s.name.clone()),
                categories: Some(
                    s.categories
                        .iter()
                        .map(|c| wire::Category {
                            code: Some(c.code.clone()),
                            name: Some(c.name.clone()),
                        })
                        .collect(),
                ),
            })
            .collect();

        wire::serialize_describe_services_response(&wire::DescribeServicesResponse {
            services: Some(entries),
        })
    }

    // STUB[no-telemetry]: DescribeTrustedAdvisorChecks returns a static set of three well-known
    //   check IDs. Real Trusted Advisor check metadata is account-level and driven by real
    //   infrastructure telemetry; the mock has no such data and returns a fixed catalogue.
    async fn handle_describe_trusted_advisor_checks(&self, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_describe_trusted_advisor_checks_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let _language = input.language;

        // Return a small set of mock Trusted Advisor checks
        let checks = vec![
            wire::TrustedAdvisorCheckDescription {
                id: Some("1iG5NDGVre".to_string()),
                name: Some("Security Groups - Specific Ports Unrestricted".to_string()),
                description: Some("Checks security groups for rules that allow unrestricted access to specific ports.".to_string()),
                category: Some("security".to_string()),
                metadata: Some(vec!["Region", "Security Group Name", "Security Group ID", "Protocol", "Port", "Status", "IP Address"]
                    .into_iter().map(String::from).collect()),
            },
            wire::TrustedAdvisorCheckDescription {
                id: Some("HCP4007jGY".to_string()),
                name: Some("S3 Bucket Permissions".to_string()),
                description: Some("Checks buckets in Amazon S3 that have open access permissions.".to_string()),
                category: Some("security".to_string()),
                metadata: Some(vec!["Region", "Bucket Name", "ACL Allows List", "ACL Allows Upload/Delete", "Status"]
                    .into_iter().map(String::from).collect()),
            },
            wire::TrustedAdvisorCheckDescription {
                id: Some("Qch7DwouX1".to_string()),
                name: Some("Low Utilization Amazon EC2 Instances".to_string()),
                description: Some("Checks the Amazon Elastic Compute Cloud (Amazon EC2) instances that were running at any time during the last 14 days.".to_string()),
                category: Some("cost_optimizing".to_string()),
                metadata: Some(vec!["Region", "Instance ID", "Instance Name", "Instance Type", "Estimated Monthly Savings", "Day 1-14 CPU Utilization"]
                    .into_iter().map(String::from).collect()),
            },
        ];

        wire::serialize_describe_trusted_advisor_checks_response(
            &wire::DescribeTrustedAdvisorChecksResponse {
                checks: Some(checks),
            },
        )
    }

    async fn handle_refresh_trusted_advisor_check(
        &self,
        state: &Arc<tokio::sync::RwLock<SupportState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_refresh_trusted_advisor_check_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.check_id.is_empty() {
            return json_error_response(400, "InvalidParameterValue", "checkId is required");
        }
        let check_id = input.check_id.as_str();

        let mut state = state.write().await;
        let status = state.next_refresh_status(check_id).to_string();

        wire::serialize_refresh_trusted_advisor_check_response(
            &wire::RefreshTrustedAdvisorCheckResponse {
                status: Some(wire::TrustedAdvisorCheckRefreshStatus {
                    check_id: Some(check_id.to_string()),
                    status: Some(status),
                    millis_until_next_refreshable: Some(3600000),
                }),
            },
        )
    }
}

fn support_error_response(err: &SupportError) -> MockResponse {
    match err {
        SupportError::CaseIdNotFound => MockResponse::json(
            400,
            json!({"__type": "CaseIdNotFound", "message": err.to_string()}).to_string(),
        ),
    }
}
