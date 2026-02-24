//! MockAwsClient: bridges aws-sdk-rust HTTP requests to mock service backends.

use std::fmt;
use std::sync::Arc;

use aws_smithy_runtime_api::client::http::{
    HttpClient, HttpConnector, HttpConnectorFuture, HttpConnectorSettings, SharedHttpConnector,
};
use aws_smithy_runtime_api::client::runtime_components::RuntimeComponents;
use aws_smithy_runtime_api::http::StatusCode;
use aws_smithy_types::body::SdkBody;
use bytes::Bytes;
use regex::Regex;

use crate::auth;
use crate::service::{MockRequest, MockResponse, MockService};

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
        request: aws_smithy_runtime_api::client::orchestrator::HttpRequest,
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
        let body_bytes = request
            .body()
            .bytes()
            .map(Bytes::copy_from_slice)
            .unwrap_or_default();

        let mock_request = MockRequest {
            method,
            uri,
            headers,
            body: body_bytes,
        };

        HttpConnectorFuture::new(async move {
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
