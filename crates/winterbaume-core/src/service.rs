//! Service trait for pluggable AWS service backends.

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use bytes::Bytes;
use http::HeaderMap;

/// An incoming mock AWS request, parsed from the SDK's HTTP request.
#[derive(Debug, Clone)]
pub struct MockRequest {
    pub method: String,
    pub uri: String,
    pub headers: HeaderMap,
    pub body: Bytes,
}

/// A mock AWS response to return to the SDK.
#[derive(Debug, Clone)]
pub struct MockResponse {
    pub status: u16,
    pub headers: HeaderMap,
    pub body: Bytes,
}

impl MockResponse {
    /// Create an XML response.
    pub fn xml(status: u16, body: impl Into<Bytes>) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(http::header::CONTENT_TYPE, "text/xml".parse().unwrap());
        Self {
            status,
            headers,
            body: body.into(),
        }
    }

    /// Create a JSON response (awsJson1.0/1.1 protocol).
    pub fn json(status: u16, body: impl Into<Bytes>) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(
            http::header::CONTENT_TYPE,
            "application/x-amz-json-1.0".parse().unwrap(),
        );
        Self {
            status,
            headers,
            body: body.into(),
        }
    }

    /// Create a REST JSON response (restJson1 protocol).
    /// Uses `application/json` content type instead of `application/x-amz-json-1.0`.
    pub fn rest_json(status: u16, body: impl Into<Bytes>) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(
            http::header::CONTENT_TYPE,
            "application/json".parse().unwrap(),
        );
        Self {
            status,
            headers,
            body: body.into(),
        }
    }

    /// Create a CBOR response (rpc-v2-cbor protocol).
    /// Sets `Content-Type: application/cbor` and `smithy-protocol: rpc-v2-cbor`.
    pub fn cbor(status: u16, body: impl Into<Bytes>) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(
            http::header::CONTENT_TYPE,
            "application/cbor".parse().unwrap(),
        );
        headers.insert("smithy-protocol", "rpc-v2-cbor".parse().unwrap());
        Self {
            status,
            headers,
            body: body.into(),
        }
    }

    /// Create an error response (generic, no service-specific namespace).
    pub fn error(status: u16, code: &str, message: &str) -> Self {
        let body = format!(
            r#"<ErrorResponse>
  <Error>
    <Type>Sender</Type>
    <Code>{code}</Code>
    <Message>{message}</Message>
  </Error>
  <RequestId>00000000-0000-0000-0000-000000000000</RequestId>
</ErrorResponse>"#
        );
        Self::xml(status, body)
    }
}

/// A generic stub service that returns 501 Not Implemented for all requests.
///
/// Used to register services that are recognized (routable) but not yet
/// fully implemented. This ensures the router returns a proper "not implemented"
/// error instead of "unknown service".
pub struct StubService {
    name: String,
    patterns: Vec<String>,
}

impl StubService {
    /// Create a new stub service with the given service name and URL patterns.
    pub fn new(name: impl Into<String>, patterns: Vec<String>) -> Self {
        Self {
            name: name.into(),
            patterns,
        }
    }
}

impl MockService for StubService {
    fn service_name(&self) -> &str {
        &self.name
    }

    fn url_patterns(&self) -> Vec<&str> {
        self.patterns.iter().map(|s| s.as_str()).collect()
    }

    fn handle(
        &self,
        _request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        let name = self.name.clone();
        Box::pin(async move {
            MockResponse::json(
                501,
                format!(
                    r#"{{"__type":"NotImplementedException","message":"Service '{}' is recognized but not yet implemented in winterbaume"}}"#,
                    name
                ),
            )
        })
    }
}

/// Trait for a mock AWS service backend.
///
/// Each service (STS, IAM, S3, etc.) implements this trait.
pub trait MockService: Send + Sync + 'static {
    /// The service identifier (e.g., "sts", "iam", "s3").
    fn service_name(&self) -> &str;

    /// URL patterns this service handles (as regex strings).
    fn url_patterns(&self) -> Vec<&str>;

    /// Handle an incoming request and produce a response.
    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>>;
}

/// Allow `Arc<S>` to be registered as a service so callers can keep a
/// shared handle (e.g. for inspecting state via `StatefulService::snapshot`)
/// while still passing the service to `MockAws::builder().with_service(...)`.
impl<T: MockService> MockService for Arc<T> {
    fn service_name(&self) -> &str {
        (**self).service_name()
    }

    fn url_patterns(&self) -> Vec<&str> {
        (**self).url_patterns()
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        (**self).handle(request)
    }
}
