//! The AWS endpoint that functions call: `AWS_ENDPOINT_URL` inside the guest
//! is `http://127.0.0.1:4566`, which the guest agent relays over vsock to a
//! Unix socket on the host, served here by the emulator's own services.
//!
//! Nothing listens on a host TCP port.  The services are reached through a
//! [`RequestDispatcher`]: [`MockAws::dispatcher`] in library mode, the
//! server's router in server mode.
//!
//! [`MockAws::dispatcher`]: winterbaume_core::MockAws::dispatcher

use std::convert::Infallible;
use std::path::Path;
use std::sync::{Arc, OnceLock};

use bytes::Bytes;
use http::{Request, Response};
use http_body_util::{BodyExt, Full};
use hyper::body::Incoming;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use tokio::net::UnixListener;
use winterbaume_core::{MockRequest, RequestDispatcher};

/// The services that functions' AWS calls go to, given once they exist.
///
/// An executor is created before the mock environment or server that it
/// belongs to, so it takes this handle and the services are connected later:
///
/// ```no_run
/// # use std::sync::Arc;
/// # use winterbaume_core::MockAws;
/// # use winterbaume_lambda::LambdaService;
/// # use winterbaume_lambda_vm::{AwsEndpoint, KrunExecutor, KrunExecutorConfig};
/// let endpoint = AwsEndpoint::new();
/// let executor = KrunExecutor::new(KrunExecutorConfig::new().with_aws_endpoint(endpoint.clone()));
/// let mock = MockAws::builder()
///     .with_service(LambdaService::new().with_code_execution(Arc::new(executor)))
///     .build();
/// endpoint.connect(mock.dispatcher());
/// ```
///
/// Until it is connected, functions get no `AWS_ENDPOINT_URL`.
#[derive(Clone, Default)]
pub struct AwsEndpoint {
    dispatcher: Arc<OnceLock<Arc<dyn RequestDispatcher>>>,
}

impl std::fmt::Debug for AwsEndpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AwsEndpoint")
            .field("connected", &self.dispatcher.get().is_some())
            .finish()
    }
}

impl AwsEndpoint {
    pub fn new() -> Self {
        Self::default()
    }

    /// Connect the services.  Only the first call has an effect; whether it
    /// was this one.
    pub fn connect(&self, dispatcher: Arc<dyn RequestDispatcher>) -> bool {
        self.dispatcher.set(dispatcher).is_ok()
    }

    pub fn is_connected(&self) -> bool {
        self.dispatcher.get().is_some()
    }

    pub(crate) fn dispatcher(&self) -> Option<Arc<dyn RequestDispatcher>> {
        self.dispatcher.get().cloned()
    }
}

/// An endpoint served on a Unix socket.  Dropping it stops serving.
pub(crate) struct EndpointServer {
    task: tokio::task::JoinHandle<()>,
}

impl Drop for EndpointServer {
    fn drop(&mut self) {
        self.task.abort();
    }
}

impl EndpointServer {
    pub(crate) fn bind(
        path: &Path,
        dispatcher: Arc<dyn RequestDispatcher>,
    ) -> std::io::Result<Self> {
        let listener = UnixListener::bind(path)?;
        let task = tokio::spawn(async move {
            loop {
                let Ok((stream, _)) = listener.accept().await else {
                    continue;
                };
                let dispatcher = dispatcher.clone();
                tokio::spawn(async move {
                    let io = hyper_util::rt::TokioIo::new(stream);
                    let service = service_fn(move |request| {
                        let dispatcher = dispatcher.clone();
                        async move { Ok::<_, Infallible>(handle(&*dispatcher, request).await) }
                    });
                    let _ = http1::Builder::new()
                        .keep_alive(true)
                        .serve_connection(io, service)
                        .await;
                });
            }
        });
        Ok(Self { task })
    }
}

async fn handle(
    dispatcher: &dyn RequestDispatcher,
    request: Request<Incoming>,
) -> Response<Full<Bytes>> {
    let (parts, body) = request.into_parts();
    let body = body
        .collect()
        .await
        .map(|c| c.to_bytes())
        .unwrap_or_default();
    let mut headers = parts.headers;
    let path_and_query = parts
        .uri
        .path_and_query()
        .map(|pq| pq.as_str().to_string())
        .unwrap_or_else(|| "/".to_string());
    let uri = format!("https://{}{path_and_query}", canonical_host(&headers));
    if !headers.contains_key("x-forwarded-proto") {
        headers.insert("x-forwarded-proto", http::HeaderValue::from_static("http"));
    }
    if !headers.contains_key("x-forwarded-for") {
        headers.insert(
            "x-forwarded-for",
            http::HeaderValue::from_static("127.0.0.1"),
        );
    }
    let response = dispatcher
        .dispatch(MockRequest {
            method: parts.method.to_string(),
            uri,
            headers,
            body,
        })
        .await;
    let mut builder = Response::builder().status(response.status);
    for (name, value) in &response.headers {
        builder = builder.header(name, value);
    }
    builder.body(Full::new(response.body)).unwrap_or_else(|_| {
        Response::builder()
            .status(500)
            .body(Full::new(Bytes::from_static(b"Internal Server Error")))
            .expect("static response parts are valid")
    })
}

/// The AWS endpoint host a request was meant for.
///
/// Requests arrive addressed to `127.0.0.1:4566`.  Services route and read
/// their region from AWS endpoint host names, so the host is rebuilt from
/// the SigV4 credential scope ( `<key>/<date>/<region>/<service>/aws4_request` )
/// of the `Authorization` header or of a presigned URL's `X-Amz-Credential`.
fn canonical_host(headers: &http::HeaderMap) -> String {
    let scope = headers
        .get(http::header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|auth| auth.split("Credential=").nth(1))
        .map(|rest| rest.split([',', ' ']).next().unwrap_or(rest).to_string());
    let parts: Option<(String, String)> = scope.and_then(|credential| {
        let mut fields = credential.split('/');
        let _key = fields.next()?;
        let _date = fields.next()?;
        let region = fields.next()?.to_string();
        let service = fields.next()?.to_string();
        Some((region, service))
    });
    match parts {
        Some((region, service))
            if !region.is_empty()
                && !service.is_empty()
                && region
                    .chars()
                    .chain(service.chars())
                    .all(|c| c.is_ascii_alphanumeric() || c == '-') =>
        {
            format!("{service}.{region}.amazonaws.com")
        }
        _ => headers
            .get(http::header::HOST)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("localhost")
            .to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_host_comes_from_the_credential_scope() {
        let mut headers = http::HeaderMap::new();
        headers.insert(http::header::HOST, "127.0.0.1:4566".parse().unwrap());
        assert_eq!(canonical_host(&headers), "127.0.0.1:4566");
        headers.insert(
            http::header::AUTHORIZATION,
            "AWS4-HMAC-SHA256 Credential=AKID/20260101/eu-west-1/dynamodb/aws4_request, SignedHeaders=host, Signature=x"
                .parse()
                .unwrap(),
        );
        assert_eq!(canonical_host(&headers), "dynamodb.eu-west-1.amazonaws.com");
        headers.insert(
            http::header::AUTHORIZATION,
            "AWS4-HMAC-SHA256 Credential=AKID/20260101/../evil/aws4_request"
                .parse()
                .unwrap(),
        );
        assert_eq!(canonical_host(&headers), "127.0.0.1:4566");
    }
}
