use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, default_account_id,
};

use crate::model;
use crate::state::{CostAndUsageReportState, CurError, ReportRecord};
use crate::views::CostAndUsageReportStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct CostAndUsageReportService {
    pub(crate) state: Arc<BackendState<CostAndUsageReportState>>,
    pub(crate) notifier: StateChangeNotifier<CostAndUsageReportStateView>,
}

impl CostAndUsageReportService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for CostAndUsageReportService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for CostAndUsageReportService {
    fn service_name(&self) -> &str {
        "cur"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://cur\..*\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

type SharedState = Arc<tokio::sync::RwLock<CostAndUsageReportState>>;

const MUTATING_ACTIONS: &[&str] = &[
    "PutReportDefinition",
    "ModifyReportDefinition",
    "DeleteReportDefinition",
    "TagResource",
    "UntagResource",
];

impl CostAndUsageReportService {
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
            None => return aws_json_error(400, "MissingAction", "Missing X-Amz-Target"),
        };

        if !request.body.is_empty() && serde_json::from_slice::<Value>(&request.body).is_err() {
            return aws_json_error(400, "SerializationException", "Invalid JSON body");
        }
        let body_owned: Vec<u8> = if request.body.is_empty() {
            b"{}".to_vec()
        } else {
            request.body.to_vec()
        };
        let body_bytes: &[u8] = &body_owned;

        let state = self.state.get(account_id, &region);

        let response = match action.as_str() {
            "PutReportDefinition" => self.handle_put(&state, body_bytes).await,
            "ModifyReportDefinition" => self.handle_modify(&state, body_bytes).await,
            "DeleteReportDefinition" => self.handle_delete(&state, body_bytes).await,
            "DescribeReportDefinitions" => self.handle_describe(&state, body_bytes).await,
            "ListTagsForResource" => self.handle_list_tags(&state, body_bytes).await,
            "TagResource" => self.handle_tag(&state, body_bytes).await,
            "UntagResource" => self.handle_untag(&state, body_bytes).await,
            other => aws_json_error(
                400,
                "UnknownOperationException",
                &format!("Unknown action: {other}"),
            ),
        };

        use winterbaume_core::StatefulService;
        if response.status / 100 == 2 && MUTATING_ACTIONS.contains(&action.as_str()) {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_put(&self, state: &SharedState, body: &[u8]) -> MockResponse {
        let raw: Value = match serde_json::from_slice(body) {
            Ok(v) => v,
            Err(e) => return aws_json_error(400, "ValidationException", &e.to_string()),
        };
        if raw.get("ReportDefinition").is_none() {
            return aws_json_error(400, "ValidationException", "ReportDefinition is required");
        }
        let input = match wire::deserialize_put_report_definition_request(body) {
            Ok(v) => v,
            Err(e) => return aws_json_error(400, "ValidationException", &e),
        };
        let raw_def = raw.get("ReportDefinition").expect("checked above");
        let record = match record_from_typed(&input.report_definition, raw_def) {
            Ok(r) => r,
            Err(r) => return r,
        };
        let tag_list: HashMap<String, String> = input
            .tags
            .as_deref()
            .map(|arr| {
                arr.iter()
                    .map(|t| (t.key.clone(), t.value.clone()))
                    .collect()
            })
            .unwrap_or_default();
        let mut state = state.write().await;
        match state.put_report(record.clone()) {
            Ok(()) => {
                if !tag_list.is_empty() {
                    state.tag_resource(&record.report_name, tag_list);
                }
                wire::serialize_put_report_definition_response(
                    &model::PutReportDefinitionResponse {},
                )
            }
            Err(e) => err_response(&e),
        }
    }

    async fn handle_modify(&self, state: &SharedState, body: &[u8]) -> MockResponse {
        let raw: Value = match serde_json::from_slice(body) {
            Ok(v) => v,
            Err(e) => return aws_json_error(400, "ValidationException", &e.to_string()),
        };
        if raw.get("ReportDefinition").is_none() {
            return aws_json_error(400, "ValidationException", "ReportDefinition is required");
        }
        let input = match wire::deserialize_modify_report_definition_request(body) {
            Ok(v) => v,
            Err(e) => return aws_json_error(400, "ValidationException", &e),
        };
        let raw_def = raw.get("ReportDefinition").expect("checked above");
        let record = match record_from_typed(&input.report_definition, raw_def) {
            Ok(r) => r,
            Err(r) => return r,
        };
        let mut state = state.write().await;
        match state.modify_report(record) {
            Ok(()) => wire::serialize_modify_report_definition_response(
                &model::ModifyReportDefinitionResponse {},
            ),
            Err(e) => err_response(&e),
        }
    }

    async fn handle_delete(&self, state: &SharedState, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_delete_report_definition_request(body) {
            Ok(v) => v,
            Err(e) => return aws_json_error(400, "ValidationException", &e),
        };
        if input.report_name.is_empty() {
            return aws_json_error(400, "ValidationException", "ReportName is required");
        }
        let name = input.report_name.clone();
        let mut state = state.write().await;
        let removed = state.delete_report(&name);
        let message = if removed {
            format!("Successfully deleted report {name}.")
        } else {
            format!("Report {name} did not exist.")
        };
        wire::serialize_delete_report_definition_response(&model::DeleteReportDefinitionResponse {
            response_message: Some(message),
        })
    }

    async fn handle_describe(&self, state: &SharedState, _body: &[u8]) -> MockResponse {
        let state = state.read().await;
        let definitions: Vec<model::ReportDefinition> = state
            .list_reports()
            .into_iter()
            .map(record_to_definition)
            .collect();
        wire::serialize_describe_report_definitions_response(
            &model::DescribeReportDefinitionsResponse {
                next_token: None,
                report_definitions: Some(definitions),
            },
        )
    }

    async fn handle_list_tags(&self, state: &SharedState, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_list_tags_for_resource_request(body) {
            Ok(v) => v,
            Err(e) => return aws_json_error(400, "ValidationException", &e),
        };
        if input.report_name.is_empty() {
            return aws_json_error(400, "ValidationException", "ReportName is required");
        }
        let name = input.report_name.clone();
        let state = state.read().await;
        let tags_map = state.list_tags(&name);
        wire::serialize_list_tags_for_resource_response(&model::ListTagsForResourceResponse {
            tags: if tags_map.is_empty() {
                None
            } else {
                Some(tags_to_wire(&tags_map))
            },
        })
    }

    async fn handle_tag(&self, state: &SharedState, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_tag_resource_request(body) {
            Ok(v) => v,
            Err(e) => return aws_json_error(400, "ValidationException", &e),
        };
        if input.report_name.is_empty() {
            return aws_json_error(400, "ValidationException", "ReportName is required");
        }
        let name = input.report_name.clone();
        let tags: HashMap<String, String> = input
            .tags
            .iter()
            .map(|t| (t.key.clone(), t.value.clone()))
            .collect();
        if tags.is_empty() {
            return aws_json_error(400, "ValidationException", "Tags is required");
        }
        let mut state = state.write().await;
        if !state.reports.contains_key(&name) {
            return err_response(&CurError::ReportNotFound { name });
        }
        state.tag_resource(&name, tags);
        wire::serialize_tag_resource_response(&model::TagResourceResponse {})
    }

    async fn handle_untag(&self, state: &SharedState, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_untag_resource_request(body) {
            Ok(v) => v,
            Err(e) => return aws_json_error(400, "ValidationException", &e),
        };
        if input.report_name.is_empty() {
            return aws_json_error(400, "ValidationException", "ReportName is required");
        }
        let name = input.report_name.clone();
        let keys = input.tag_keys.clone();
        let mut state = state.write().await;
        if !state.reports.contains_key(&name) {
            return err_response(&CurError::ReportNotFound { name });
        }
        state.untag_resource(&name, &keys);
        wire::serialize_untag_resource_response(&model::UntagResourceResponse {})
    }
}

fn record_from_typed(
    def: &model::ReportDefinition,
    raw: &Value,
) -> Result<ReportRecord, MockResponse> {
    fn require_field(value: &str, field: &str) -> Result<String, MockResponse> {
        if value.is_empty() {
            Err(aws_json_error(
                400,
                "ValidationException",
                &format!("{field} is required"),
            ))
        } else {
            Ok(value.to_string())
        }
    }
    let report_name = require_field(&def.report_name, "ReportName")?;
    let time_unit = require_field(&def.time_unit, "TimeUnit")?;
    let format = require_field(&def.format, "Format")?;
    let compression = require_field(&def.compression, "Compression")?;
    let s3_bucket = require_field(&def.s3_bucket, "S3Bucket")?;
    let s3_prefix = require_field(&def.s3_prefix, "S3Prefix")?;
    let s3_region = require_field(&def.s3_region, "S3Region")?;
    if raw.get("AdditionalSchemaElements").is_none() {
        return Err(aws_json_error(
            400,
            "ValidationException",
            "AdditionalSchemaElements is required",
        ));
    }
    let additional_schema_elements = def.additional_schema_elements.clone();
    let additional_artifacts = def
        .additional_artifacts
        .as_ref()
        .cloned()
        .unwrap_or_default();
    Ok(ReportRecord {
        report_name,
        time_unit,
        format,
        compression,
        additional_schema_elements,
        s3_bucket,
        s3_prefix,
        s3_region,
        additional_artifacts,
        refresh_closed_reports: def.refresh_closed_reports,
        report_versioning: def.report_versioning.clone(),
        billing_view_arn: def.billing_view_arn.clone(),
    })
}

fn record_to_definition(r: &ReportRecord) -> model::ReportDefinition {
    model::ReportDefinition {
        report_name: r.report_name.clone(),
        time_unit: r.time_unit.clone(),
        format: r.format.clone(),
        compression: r.compression.clone(),
        additional_schema_elements: r.additional_schema_elements.clone(),
        s3_bucket: r.s3_bucket.clone(),
        s3_prefix: r.s3_prefix.clone(),
        s3_region: r.s3_region.clone(),
        additional_artifacts: if r.additional_artifacts.is_empty() {
            None
        } else {
            Some(r.additional_artifacts.clone())
        },
        refresh_closed_reports: r.refresh_closed_reports,
        report_versioning: r.report_versioning.clone(),
        billing_view_arn: r.billing_view_arn.clone(),
        report_status: None,
    }
}

fn tags_to_wire(tags: &HashMap<String, String>) -> Vec<model::Tag> {
    let mut v: Vec<model::Tag> = tags
        .iter()
        .map(|(k, val)| model::Tag {
            key: k.clone(),
            value: val.clone(),
        })
        .collect();
    v.sort_by(|a, b| a.key.cmp(&b.key));
    v
}

fn aws_json_error(status: u16, error_type: &str, message: &str) -> MockResponse {
    let body = json!({"__type": error_type, "Message": message});
    let mut resp = MockResponse::json(status, body.to_string());
    resp.headers
        .insert(X_AMZN_ERRORTYPE, error_type.parse().unwrap());
    resp
}

fn err_response(err: &CurError) -> MockResponse {
    let (status, error_type) = match err {
        CurError::ReportNotFound { .. } => (404, "ResourceNotFoundException"),
        CurError::DuplicateReport { .. } => (400, "DuplicateReportNameException"),
        CurError::Validation { .. } => (400, "ValidationException"),
    };
    aws_json_error(status, error_type, &err.to_string())
}
