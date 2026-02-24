//! JSON protocol utilities for awsJson1.0/1.1 and REST-JSON services.

use http::header::HeaderName;

use crate::service::MockResponse;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

/// Create an awsJson1.0/1.1 error response with `__type` and `message` fields.
///
/// Used by services like KMS, DynamoDB, SQS, SSM, Logs, etc.
pub fn json_error_response(status: u16, error_type: &str, message: &str) -> MockResponse {
    let body = format!(
        r#"{{"__type":"{}","message":"{}"}}"#,
        escape_json_string(error_type),
        escape_json_string(message),
    );
    MockResponse::json(status, body)
}

/// Create a REST-JSON error response with `x-amzn-errortype` header.
///
/// Used by services like SES v2, Lambda, EKS, AppConfig, etc.
/// The error type is communicated via the `x-amzn-errortype` header,
/// and the body contains `Type` and `Message` fields.
pub fn rest_json_error(status: u16, code: &str, message: &str) -> MockResponse {
    let body = format!(
        r#"{{"Type":"User","Message":"{}"}}"#,
        escape_json_string(message),
    );
    let mut resp = MockResponse::rest_json(status, body);
    resp.headers.insert(X_AMZN_ERRORTYPE, code.parse().unwrap());
    resp
}

/// Check whether a top-level JSON object key is present in the raw request
/// body. Used by handlers that need to distinguish "field absent" from
/// "field present with default-equipped struct value", which the typed wire
/// model collapses when fields are non-Option with `#[serde(default)]`.
///
/// Parses the body as JSON and looks for `key` in the top-level object.
/// Returns `false` for non-object bodies, malformed JSON, or empty bodies.
pub fn body_has_top_level_field(body: &[u8], key: &str) -> bool {
    serde_json::from_slice::<serde_json::Value>(body)
        .ok()
        .and_then(|v| v.as_object().map(|o| o.contains_key(key)))
        .unwrap_or(false)
}

/// Minimal JSON string escaping for error messages.
fn escape_json_string(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_error_response() {
        let resp = json_error_response(400, "ValidationException", "Missing 'KeyId'");
        assert_eq!(resp.status, 400);
        let body: serde_json::Value = serde_json::from_slice(&resp.body).unwrap();
        assert_eq!(body["__type"], "ValidationException");
        assert_eq!(body["message"], "Missing 'KeyId'");
    }

    #[test]
    fn test_rest_json_error() {
        let resp = rest_json_error(404, "NotFoundException", "Resource not found");
        assert_eq!(resp.status, 404);
        assert_eq!(
            resp.headers.get("x-amzn-errortype").unwrap(),
            "NotFoundException"
        );
        let body: serde_json::Value = serde_json::from_slice(&resp.body).unwrap();
        assert_eq!(body["Message"], "Resource not found");
    }

    #[test]
    fn test_escape_json_string() {
        assert_eq!(escape_json_string(r#"say "hi""#), r#"say \"hi\""#);
        assert_eq!(escape_json_string("line\nnew"), "line\\nnew");
    }
}
