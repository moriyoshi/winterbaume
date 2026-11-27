//! MockAwsClient: bridges aws-sdk-rust HTTP requests to mock service backends.

use std::fmt;
use std::sync::Arc;

use aws_smithy_runtime_api::client::http::{
    HttpClient, HttpConnector, HttpConnectorFuture, HttpConnectorSettings, SharedHttpConnector,
};
use aws_smithy_runtime_api::client::result::ConnectorError;
use aws_smithy_runtime_api::client::runtime_components::RuntimeComponents;
use aws_smithy_runtime_api::http::StatusCode;
use aws_smithy_types::body::SdkBody;
use aws_smithy_types::byte_stream::ByteStream;
use bytes::Bytes;
use regex::Regex;

use crate::auth;
use crate::service::{MockRequest, MockResponse, MockService};

/// A request body as the connector received it.
enum RequestBody {
    Buffered(Bytes),
    Streaming(SdkBody),
}

struct ServiceRoute {
    pattern: Regex,
    service: Arc<dyn MockService>,
}

/// An HTTP connector that intercepts aws-sdk-rust requests and routes them
/// to in-memory mock service backends.
#[derive(Clone)]
pub struct MockAwsClient {
    routes: Arc<Vec<ServiceRoute>>,
}

impl fmt::Debug for MockAwsClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MockAwsClient")
            .field("routes", &self.routes.len())
            .finish()
    }
}

impl MockAwsClient {
    pub(crate) fn new(services: Vec<Arc<dyn MockService>>) -> Self {
        let mut routes = Vec::new();
        for service in services {
            for pattern in service.url_patterns() {
                routes.push(ServiceRoute {
                    pattern: Regex::new(pattern)
                        .unwrap_or_else(|e| panic!("Invalid URL pattern '{}': {}", pattern, e)),
                    service: Arc::clone(&service),
                });
            }
        }
        Self {
            routes: Arc::new(routes),
        }
    }

    fn find_service_by_url(routes: &[ServiceRoute], uri: &str) -> Option<Arc<dyn MockService>> {
        for route in routes {
            if route.pattern.is_match(uri) {
                return Some(Arc::clone(&route.service));
            }
        }
        None
    }

    fn find_service_by_name(routes: &[ServiceRoute], name: &str) -> Option<Arc<dyn MockService>> {
        routes
            .iter()
            .find(|r| r.service.service_name() == name)
            .map(|r| Arc::clone(&r.service))
    }
}

impl HttpConnector for MockAwsClient {
    fn call(
        &self,
        mut request: aws_smithy_runtime_api::client::orchestrator::HttpRequest,
    ) -> HttpConnectorFuture {
        let uri_string = request.uri().to_string();
        let routes = Arc::clone(&self.routes);

        // Try URL pattern routing first, then fall back to URI-based service extraction
        let service = Self::find_service_by_url(&routes, &uri_string).or_else(|| {
            let service_name = auth::extract_service_from_uri(&uri_string)?;
            Self::find_service_by_name(&routes, &service_name)
        });

        let service = match service {
            Some(s) => s,
            None => {
                return HttpConnectorFuture::ready(Ok(not_implemented_response(&uri_string)));
            }
        };

        // Extract request parts into MockRequest
        let method = request.method().to_string();
        let uri = uri_string;
        let mut headers = http::HeaderMap::new();
        for (name, value) in request.headers() {
            if let (Ok(hn), Ok(hv)) = (
                http::header::HeaderName::from_bytes(name.as_bytes()),
                http::header::HeaderValue::from_str(value),
            ) {
                headers.insert(hn, hv);
            }
        }
        // An in-memory body is copied as is. A streaming one ( for example
        // `ByteStream::from_path` ) has no bytes to copy yet, so it is taken
        // from the request and read to the end before the service sees it;
        // copying `bytes()` alone would hand the service an empty body.
        let body = match request.body().bytes() {
            Some(bytes) => RequestBody::Buffered(Bytes::copy_from_slice(bytes)),
            None => RequestBody::Streaming(request.take_body()),
        };

        HttpConnectorFuture::new(async move {
            let body = match body {
                RequestBody::Buffered(bytes) => bytes,
                RequestBody::Streaming(body) => ByteStream::new(body)
                    .collect()
                    .await
                    .map_err(|e| ConnectorError::io(Box::new(e)))?
                    .into_bytes(),
            };
            let mock_request = MockRequest {
                method,
                uri,
                headers,
                body,
            };
            let mock_response = service.handle(mock_request).await;
            Ok(mock_response_to_http(mock_response))
        })
    }
}

impl HttpClient for MockAwsClient {
    fn http_connector(
        &self,
        _settings: &HttpConnectorSettings,
        _components: &RuntimeComponents,
    ) -> SharedHttpConnector {
        SharedHttpConnector::new(self.clone())
    }
}

fn mock_response_to_http(
    mock_response: MockResponse,
) -> aws_smithy_runtime_api::client::orchestrator::HttpResponse {
    let mut response = aws_smithy_runtime_api::client::orchestrator::HttpResponse::new(
        StatusCode::try_from(mock_response.status).unwrap_or(StatusCode::try_from(500).unwrap()),
        SdkBody::from(mock_response.body),
    );
    for (name, value) in &mock_response.headers {
        if let Ok(value_str) = value.to_str() {
            let name_owned = name.as_str().to_string();
            let value_owned = value_str.to_string();
            response.headers_mut().insert(name_owned, value_owned);
        }
    }
    response
}

fn not_implemented_response(
    uri: &str,
) -> aws_smithy_runtime_api::client::orchestrator::HttpResponse {
    let body = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<ErrorResponse>
  <Error>
    <Code>NotImplemented</Code>
    <Message>Service not yet implemented in winterbaume: {}</Message>
  </Error>
</ErrorResponse>"#,
        uri
    );
    aws_smithy_runtime_api::client::orchestrator::HttpResponse::new(
        StatusCode::try_from(501).unwrap(),
        SdkBody::from(body),
    )
}

#[cfg(test)]
mod tests {
    use std::future::Future;
    use std::pin::Pin;

    use aws_smithy_runtime_api::client::orchestrator::HttpRequest;

    use super::*;

    /// Answers with the body it received.
    struct Echo;

    impl MockService for Echo {
        fn service_name(&self) -> &str {
            "echo"
        }

        fn url_patterns(&self) -> Vec<&str> {
            vec![r"https?://echo\.example\.com"]
        }

        fn handle(
            &self,
            request: MockRequest,
        ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
            Box::pin(async move {
                MockResponse {
                    status: 200,
                    headers: http::HeaderMap::new(),
                    body: request.body,
                }
            })
        }
    }

    async fn echo(body: SdkBody) -> Bytes {
        let client = MockAwsClient::new(vec![Arc::new(Echo)]);
        let mut request = HttpRequest::new(body);
        request.set_uri("https://echo.example.com/upload").unwrap();
        let response = client.call(request).await.unwrap();
        Bytes::copy_from_slice(response.body().bytes().unwrap())
    }

    #[tokio::test]
    async fn buffered_body_reaches_the_service() {
        assert_eq!(
            echo(SdkBody::from("hello")).await,
            Bytes::from_static(b"hello")
        );
    }

    /// A streaming body ( here `ByteStream::from_path` ) has no bytes up
    /// front; it used to reach the service empty.
    #[tokio::test]
    async fn streaming_body_reaches_the_service() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("payload");
        let data: Vec<u8> = (0..200_000u32).map(|i| (i % 251) as u8).collect();
        std::fs::write(&path, &data).unwrap();

        let body = ByteStream::from_path(&path).await.unwrap().into_inner();
        assert!(body.bytes().is_none(), "the body must be streaming");
        assert_eq!(echo(body).await, Bytes::from(data));
    }

    #[tokio::test]
    async fn empty_body_stays_empty() {
        assert_eq!(echo(SdkBody::empty()).await, Bytes::new());
    }
}
