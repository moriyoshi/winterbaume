use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, StatefulService,
};

use crate::state::{TextractError, TextractJob, TextractState};
use crate::views::TextractStateView;
use crate::wire;

fn textract_error_response(err: TextractError) -> MockResponse {
    match err {
        TextractError::MissingAction => json_error_response(400, "InvalidAction", &err.to_string()),
        TextractError::SerializationException => {
            json_error_response(400, "SerializationException", &err.to_string())
        }
        TextractError::InvalidParameter => {
            json_error_response(400, "InvalidParameterException", &err.to_string())
        }
        TextractError::UnknownOperation { .. } => {
            json_error_response(400, "UnknownOperationException", &err.to_string())
        }
    }
}

/// Amazon Textract service handler that processes awsJson1.1 protocol requests.
pub struct TextractService {
    pub(crate) state: Arc<BackendState<TextractState>>,
    pub(crate) notifier: StateChangeNotifier<TextractStateView>,
}

impl TextractService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for TextractService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for TextractService {
    fn service_name(&self) -> &str {
        "textract"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://textract\..*\.amazonaws\.com",
            r"https?://textract\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl TextractService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        // Extract action from X-Amz-Target header
        // Format: "Textract.DetectDocumentText"
        let action = request
            .headers
            .get("x-amz-target")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.split('.').next_back())
            .map(|s| s.to_string());

        let action = match action {
            Some(a) => a,
            None => {
                return textract_error_response(TextractError::MissingAction);
            }
        };

        if serde_json::from_slice::<Value>(&request.body).is_err() {
            return textract_error_response(TextractError::SerializationException);
        }
        let body_bytes: &[u8] = &request.body;

        let account_id = request
            .headers
            .get("x-amz-account-id")
            .and_then(|v| v.to_str().ok())
            .unwrap_or(winterbaume_core::DEFAULT_ACCOUNT_ID);
        let region = request
            .headers
            .get("x-amz-region")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("us-east-1")
            .to_string();

        let response = match action.as_str() {
            "DetectDocumentText" => self.handle_detect_document_text(body_bytes).await,
            "AnalyzeDocument" => self.handle_analyze_document(body_bytes).await,
            "GetDocumentAnalysis" => {
                self.handle_get_document_analysis(body_bytes, account_id, &region)
                    .await
            }
            // --- Unimplemented operations (auto-generated stubs) ---
            "AnalyzeExpense" => json_error_response(
                501,
                "NotImplementedError",
                "AnalyzeExpense is not yet implemented in winterbaume-textract",
            ),
            "AnalyzeID" => json_error_response(
                501,
                "NotImplementedError",
                "AnalyzeID is not yet implemented in winterbaume-textract",
            ),
            "CreateAdapter" => json_error_response(
                501,
                "NotImplementedError",
                "CreateAdapter is not yet implemented in winterbaume-textract",
            ),
            "CreateAdapterVersion" => json_error_response(
                501,
                "NotImplementedError",
                "CreateAdapterVersion is not yet implemented in winterbaume-textract",
            ),
            "DeleteAdapter" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteAdapter is not yet implemented in winterbaume-textract",
            ),
            "DeleteAdapterVersion" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteAdapterVersion is not yet implemented in winterbaume-textract",
            ),
            "GetAdapter" => json_error_response(
                501,
                "NotImplementedError",
                "GetAdapter is not yet implemented in winterbaume-textract",
            ),
            "GetAdapterVersion" => json_error_response(
                501,
                "NotImplementedError",
                "GetAdapterVersion is not yet implemented in winterbaume-textract",
            ),
            "GetDocumentTextDetection" => {
                self.handle_get_document_text_detection(body_bytes, account_id, &region)
                    .await
            }
            "GetExpenseAnalysis" => json_error_response(
                501,
                "NotImplementedError",
                "GetExpenseAnalysis is not yet implemented in winterbaume-textract",
            ),
            "GetLendingAnalysis" => json_error_response(
                501,
                "NotImplementedError",
                "GetLendingAnalysis is not yet implemented in winterbaume-textract",
            ),
            "GetLendingAnalysisSummary" => json_error_response(
                501,
                "NotImplementedError",
                "GetLendingAnalysisSummary is not yet implemented in winterbaume-textract",
            ),
            "ListAdapterVersions" => json_error_response(
                501,
                "NotImplementedError",
                "ListAdapterVersions is not yet implemented in winterbaume-textract",
            ),
            "ListAdapters" => json_error_response(
                501,
                "NotImplementedError",
                "ListAdapters is not yet implemented in winterbaume-textract",
            ),
            "ListTagsForResource" => json_error_response(
                501,
                "NotImplementedError",
                "ListTagsForResource is not yet implemented in winterbaume-textract",
            ),
            "StartDocumentAnalysis" => {
                self.handle_start_document_analysis(body_bytes, account_id, &region)
                    .await
            }
            "StartDocumentTextDetection" => {
                self.handle_start_document_text_detection(body_bytes, account_id, &region)
                    .await
            }
            "StartExpenseAnalysis" => json_error_response(
                501,
                "NotImplementedError",
                "StartExpenseAnalysis is not yet implemented in winterbaume-textract",
            ),
            "StartLendingAnalysis" => json_error_response(
                501,
                "NotImplementedError",
                "StartLendingAnalysis is not yet implemented in winterbaume-textract",
            ),
            "TagResource" => json_error_response(
                501,
                "NotImplementedError",
                "TagResource is not yet implemented in winterbaume-textract",
            ),
            "UntagResource" => json_error_response(
                501,
                "NotImplementedError",
                "UntagResource is not yet implemented in winterbaume-textract",
            ),
            "UpdateAdapter" => json_error_response(
                501,
                "NotImplementedError",
                "UpdateAdapter is not yet implemented in winterbaume-textract",
            ),
            _ => textract_error_response(TextractError::UnknownOperation {
                action: action.clone(),
            }),
        };
        if response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    /// DetectDocumentText: takes a Document and returns Blocks.
    /// Returns an empty Blocks list as a plausible mock response.
    async fn handle_detect_document_text(&self, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_detect_document_text_request(body) {
            Ok(v) => v,
            Err(_) => return textract_error_response(TextractError::InvalidParameter),
        };
        // Validate that Document is provided (i.e. has at least one source)
        if input.document.bytes.is_none() && input.document.s3_object.is_none() {
            return textract_error_response(TextractError::InvalidParameter);
        }

        let resp = wire::DetectDocumentTextResponse {
            document_metadata: Some(wire::DocumentMetadata { pages: Some(1) }),
            blocks: Some(vec![]),
            detect_document_text_model_version: Some("1.0".to_string()),
        };
        wire::serialize_detect_document_text_response(&resp)
    }

    /// AnalyzeDocument: takes a Document and FeatureTypes, returns Blocks.
    /// Returns an empty Blocks list as a plausible mock response.
    async fn handle_analyze_document(&self, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_analyze_document_request(body) {
            Ok(v) => v,
            Err(_) => return textract_error_response(TextractError::InvalidParameter),
        };
        // Validate that Document is provided
        if input.document.bytes.is_none() && input.document.s3_object.is_none() {
            return textract_error_response(TextractError::InvalidParameter);
        }
        // Validate that FeatureTypes is provided
        if input.feature_types.is_empty() {
            return textract_error_response(TextractError::InvalidParameter);
        }

        let resp = wire::AnalyzeDocumentResponse {
            document_metadata: Some(wire::DocumentMetadata { pages: Some(1) }),
            blocks: Some(vec![]),
            analyze_document_model_version: Some("1.0".to_string()),
            human_loop_activation_output: None,
        };
        wire::serialize_analyze_document_response(&resp)
    }

    /// GetDocumentAnalysis: retrieves async job results by JobId.
    /// Returns a mock SUCCEEDED status with blocks stored at start time.
    async fn handle_get_document_analysis(
        &self,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_get_document_analysis_request(body) {
            Ok(v) => v,
            Err(_) => {
                return json_error_response(
                    400,
                    "InvalidParameterException",
                    "Request has invalid parameters",
                );
            }
        };
        if input.job_id.is_empty() {
            return json_error_response(
                400,
                "InvalidParameterException",
                "Request has invalid parameters",
            );
        }
        let job_id = input.job_id.as_str();

        let blocks = {
            let state = self.state.get(account_id, region);
            let guard = state.read().await;
            guard
                .jobs
                .get(job_id)
                .map(|job| job.blocks.clone())
                .unwrap_or_default()
        };

        let resp = wire::GetDocumentAnalysisResponse {
            document_metadata: Some(wire::DocumentMetadata { pages: Some(1) }),
            job_status: Some("SUCCEEDED".to_string()),
            blocks: Some(blocks),
            analyze_document_model_version: Some("1.0".to_string()),
            next_token: None,
            status_message: None,
            warnings: None,
        };
        wire::serialize_get_document_analysis_response(&resp)
    }

    /// GetDocumentTextDetection: retrieves async text detection results by JobId.
    async fn handle_get_document_text_detection(
        &self,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_get_document_text_detection_request(body) {
            Ok(v) => v,
            Err(_) => {
                return json_error_response(
                    400,
                    "InvalidParameterException",
                    "Request has invalid parameters",
                );
            }
        };
        if input.job_id.is_empty() {
            return json_error_response(
                400,
                "InvalidParameterException",
                "Request has invalid parameters",
            );
        }
        let job_id = input.job_id.as_str();

        let blocks = {
            let state = self.state.get(account_id, region);
            let guard = state.read().await;
            guard
                .jobs
                .get(job_id)
                .map(|job| job.blocks.clone())
                .unwrap_or_default()
        };

        let resp = wire::GetDocumentTextDetectionResponse {
            document_metadata: Some(wire::DocumentMetadata { pages: Some(1) }),
            job_status: Some("SUCCEEDED".to_string()),
            blocks: Some(blocks),
            detect_document_text_model_version: Some("1.0".to_string()),
            next_token: None,
            status_message: None,
            warnings: None,
        };
        wire::serialize_get_document_text_detection_response(&resp)
    }

    /// StartDocumentAnalysis: starts an async document analysis job.
    async fn handle_start_document_analysis(
        &self,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_start_document_analysis_request(body) {
            Ok(v) => v,
            Err(_) => {
                return json_error_response(
                    400,
                    "InvalidParameterException",
                    "Request has invalid parameters",
                );
            }
        };
        if input.document_location.s3_object.is_none() {
            return json_error_response(
                400,
                "InvalidParameterException",
                "Request has invalid parameters",
            );
        }
        if input.feature_types.is_empty() {
            return json_error_response(
                400,
                "InvalidParameterException",
                "Request has invalid parameters",
            );
        }

        let job_id = uuid::Uuid::new_v4().to_string();
        let mock_block = wire::Block {
            block_type: Some("PAGE".to_string()),
            id: Some("mock-page-1".to_string()),
            confidence: Some(99.0),
            page: Some(1),
            ..Default::default()
        };
        {
            let state = self.state.get(account_id, region);
            let mut guard = state.write().await;
            guard.jobs.insert(
                job_id.clone(),
                TextractJob {
                    job_type: "DocumentAnalysis".to_string(),
                    blocks: vec![mock_block],
                },
            );
        }
        let resp = wire::StartDocumentAnalysisResponse {
            job_id: Some(job_id),
        };
        wire::serialize_start_document_analysis_response(&resp)
    }

    /// StartDocumentTextDetection: starts an async text detection job.
    async fn handle_start_document_text_detection(
        &self,
        body: &[u8],
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_start_document_text_detection_request(body) {
            Ok(v) => v,
            Err(_) => {
                return json_error_response(
                    400,
                    "InvalidParameterException",
                    "Request has invalid parameters",
                );
            }
        };
        if input.document_location.s3_object.is_none() {
            return json_error_response(
                400,
                "InvalidParameterException",
                "Request has invalid parameters",
            );
        }

        let job_id = uuid::Uuid::new_v4().to_string();
        let mock_block = wire::Block {
            block_type: Some("PAGE".to_string()),
            id: Some("mock-page-1".to_string()),
            confidence: Some(99.0),
            page: Some(1),
            ..Default::default()
        };
        {
            let state = self.state.get(account_id, region);
            let mut guard = state.write().await;
            guard.jobs.insert(
                job_id.clone(),
                TextractJob {
                    job_type: "DocumentTextDetection".to_string(),
                    blocks: vec![mock_block],
                },
            );
        }
        let resp = wire::StartDocumentTextDetectionResponse {
            job_id: Some(job_id),
        };
        wire::serialize_start_document_text_detection_response(&resp)
    }
}

fn json_error_response(status: u16, error_type: &str, message: &str) -> MockResponse {
    let body = json!({
        "__type": error_type,
        "message": message,
    });
    MockResponse::json(status, body.to_string())
}
