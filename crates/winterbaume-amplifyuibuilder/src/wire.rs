//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-amplifyuibuilder

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

#[allow(unused_imports)]
use http::header::HeaderName;
use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize response for restJson protocol.
pub fn serialize_create_component_response(result: &CreateComponentResponse) -> MockResponse {
    let status = 200_u16;
    let body = match &result.entity {
        Some(v) => serde_json::to_string(v).unwrap_or_else(|_| "{}".to_string()),
        None => String::new(),
    };
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_form_response(result: &CreateFormResponse) -> MockResponse {
    let status = 200_u16;
    let body = match &result.entity {
        Some(v) => serde_json::to_string(v).unwrap_or_else(|_| "{}".to_string()),
        None => String::new(),
    };
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_theme_response(result: &CreateThemeResponse) -> MockResponse {
    let status = 200_u16;
    let body = match &result.entity {
        Some(v) => serde_json::to_string(v).unwrap_or_else(|_| "{}".to_string()),
        None => String::new(),
    };
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_component_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_form_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_delete_theme_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_exchange_code_for_token_response(
    result: &ExchangeCodeForTokenResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_export_components_response(result: &ExportComponentsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_export_forms_response(result: &ExportFormsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_export_themes_response(result: &ExportThemesResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_codegen_job_response(result: &GetCodegenJobResponse) -> MockResponse {
    let status = 200_u16;
    let body = match &result.job {
        Some(v) => serde_json::to_string(v).unwrap_or_else(|_| "{}".to_string()),
        None => String::new(),
    };
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_component_response(result: &GetComponentResponse) -> MockResponse {
    let status = 200_u16;
    let body = match &result.component {
        Some(v) => serde_json::to_string(v).unwrap_or_else(|_| "{}".to_string()),
        None => String::new(),
    };
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_form_response(result: &GetFormResponse) -> MockResponse {
    let status = 200_u16;
    let body = match &result.form {
        Some(v) => serde_json::to_string(v).unwrap_or_else(|_| "{}".to_string()),
        None => String::new(),
    };
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_metadata_response(result: &GetMetadataResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_theme_response(result: &GetThemeResponse) -> MockResponse {
    let status = 200_u16;
    let body = match &result.theme {
        Some(v) => serde_json::to_string(v).unwrap_or_else(|_| "{}".to_string()),
        None => String::new(),
    };
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_codegen_jobs_response(result: &ListCodegenJobsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_components_response(result: &ListComponentsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_forms_response(result: &ListFormsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_tags_for_resource_response(
    result: &ListTagsForResourceResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_themes_response(result: &ListThemesResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_put_metadata_flag_response() -> MockResponse {
    MockResponse::rest_json(200, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_refresh_token_response(result: &RefreshTokenResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_start_codegen_job_response(result: &StartCodegenJobResponse) -> MockResponse {
    let status = 200_u16;
    let body = match &result.entity {
        Some(v) => serde_json::to_string(v).unwrap_or_else(|_| "{}".to_string()),
        None => String::new(),
    };
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_tag_resource_response(result: &TagResourceResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_untag_resource_response(result: &UntagResourceResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_component_response(result: &UpdateComponentResponse) -> MockResponse {
    let status = 200_u16;
    let body = match &result.entity {
        Some(v) => serde_json::to_string(v).unwrap_or_else(|_| "{}".to_string()),
        None => String::new(),
    };
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_form_response(result: &UpdateFormResponse) -> MockResponse {
    let status = 200_u16;
    let body = match &result.entity {
        Some(v) => serde_json::to_string(v).unwrap_or_else(|_| "{}".to_string()),
        None => String::new(),
    };
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_theme_response(result: &UpdateThemeResponse) -> MockResponse {
    let status = 200_u16;
    let body = match &result.entity {
        Some(v) => serde_json::to_string(v).unwrap_or_else(|_| "{}".to_string()),
        None => String::new(),
    };
    MockResponse::rest_json(status, body)
}
