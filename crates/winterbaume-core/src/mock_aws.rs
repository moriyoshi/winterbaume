//! MockAws builder for creating mock AWS environments.

use std::sync::Arc;

use aws_credential_types::Credentials;
use aws_credential_types::provider::SharedCredentialsProvider;
use aws_smithy_runtime_api::client::http::SharedHttpClient;

use crate::mock_client::MockAwsClient;
use crate::service::MockService;
use crate::state::DEFAULT_ACCOUNT_ID;
use crate::vfs::{MemVfs, Vfs};

/// A mock AWS environment containing registered service backends.
pub struct MockAws {
    client: MockAwsClient,
    account_id: String,
    vfs: Arc<dyn Vfs>,
}

impl MockAws {
    pub fn builder() -> MockAwsBuilder {
        MockAwsBuilder::new()
    }

    /// Returns a `SharedHttpClient` to pass to aws-sdk-rust config.
    pub fn http_client(&self) -> SharedHttpClient {
        SharedHttpClient::new(self.client.clone())
    }

    /// Returns a mock credentials provider.
    pub fn credentials_provider(&self) -> SharedCredentialsProvider {
        SharedCredentialsProvider::new(Credentials::new(
            "AKIAIOSFODNN7EXAMPLE",
            "wJalrXUtnFEMI/K7MDENG/bPxRfiCYzEXAMPLEKEY",
            None,
            None,
            "winterbaume",
        ))
    }

    /// Returns the mock account ID.
    pub fn account_id(&self) -> &str {
        &self.account_id
    }

    /// Returns the shared VFS used by this mock environment.
    ///
    /// Pass this to service constructors via their `with_vfs()` method to share
    /// a single backing store across all services:
    ///
    /// ```ignore
    /// # use std::sync::Arc;
    /// # use winterbaume_core::{MockAws, FsVfs};
    /// let mock = MockAws::builder()
    ///     .vfs(Arc::new(FsVfs::new("/tmp/winterbaume").unwrap()))
    ///     .build();
    /// // Pass mock.vfs() to each service's with_vfs() constructor.
    /// ```
    pub fn vfs(&self) -> Arc<dyn Vfs> {
        Arc::clone(&self.vfs)
    }

    /// Convenience: create an SdkConfig with winterbaume's HTTP client.
    pub async fn sdk_config(&self, region: &str) -> aws_config::SdkConfig {
        aws_config::defaults(aws_config::BehaviorVersion::latest())
            .http_client(self.http_client())
            .credentials_provider(self.credentials_provider())
            .region(aws_config::Region::new(region.to_owned()))
            .load()
            .await
    }

    /// Create a MockResponseInterceptor with allow_passthrough enabled.
    /// Unmatched requests fall through to winterbaume's HTTP backend.
    #[cfg(feature = "smithy-mocks")]
    pub fn mock_interceptor(
        &self,
        rule_mode: aws_smithy_mocks::RuleMode,
        rules: &[&aws_smithy_mocks::Rule],
    ) -> aws_smithy_mocks::MockResponseInterceptor {
        let mut interceptor = aws_smithy_mocks::MockResponseInterceptor::new()
            .rule_mode(rule_mode)
            .allow_passthrough();
        for rule in rules {
            interceptor = interceptor.with_rule(rule);
        }
        interceptor
    }
}

/// Builder for constructing a `MockAws` instance.
pub struct MockAwsBuilder {
    services: Vec<Arc<dyn MockService>>,
    account_id: String,
    vfs: Option<Arc<dyn Vfs>>,
}

impl MockAwsBuilder {
    fn new() -> Self {
        Self {
            services: Vec::new(),
            account_id: DEFAULT_ACCOUNT_ID.to_string(),
            vfs: None,
        }
    }

    /// Register a mock service backend.
    pub fn with_service(mut self, service: impl MockService) -> Self {
        self.services.push(Arc::new(service));
        self
    }

    /// Set the mock AWS account ID (default: "123456789012").
    pub fn account_id(mut self, id: impl Into<String>) -> Self {
        self.account_id = id.into();
        self
    }

    /// Set a shared VFS for blob storage across all services.
    ///
    /// After building, retrieve the VFS via [`MockAws::vfs()`] and pass it to
    /// each service's `with_vfs()` constructor so all services share one store.
    /// Defaults to an in-memory [`MemVfs`] if not set.
    pub fn vfs(mut self, vfs: Arc<dyn Vfs>) -> Self {
        self.vfs = Some(vfs);
        self
    }

    /// Build the MockAws instance.
    pub fn build(self) -> MockAws {
        let vfs: Arc<dyn Vfs> = self.vfs.unwrap_or_else(|| Arc::new(MemVfs::default()));
        let client = MockAwsClient::new(self.services);
        MockAws {
            client,
            account_id: self.account_id,
            vfs,
        }
    }
}
