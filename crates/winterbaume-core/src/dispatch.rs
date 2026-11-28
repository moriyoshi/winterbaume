//! Handing a request to whichever registered service it is for.

use std::future::Future;
use std::pin::Pin;

use crate::service::{MockRequest, MockResponse};

/// Routes a request to the service it is for and returns that service's
/// answer, as an AWS endpoint would.
///
/// [`MockAws::dispatcher`](crate::MockAws::dispatcher) provides one over the
/// services of a mock environment; `winterbaume-server` provides one over
/// its own services.  Code that serves requests arriving from elsewhere (
/// such as the AWS endpoint of a Lambda execution environment ) takes one.
pub trait RequestDispatcher: Send + Sync + 'static {
    fn dispatch(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>>;
}

/// The answer for a request that no registered service is for.
pub fn unknown_service_response(uri: &str) -> MockResponse {
    MockResponse::json(
        404,
        serde_json::json!({
            "__type": "UnknownService",
            "message": format!("No service registered for: {uri}"),
        })
        .to_string(),
    )
}
