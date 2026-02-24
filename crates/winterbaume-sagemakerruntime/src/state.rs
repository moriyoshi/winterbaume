use std::collections::HashMap;

use chrono::Utc;
use thiserror::Error;

use crate::types::*;

#[derive(Debug, Default)]
pub struct SageMakerRuntimeState {
    /// Registered endpoints that can be invoked.
    pub endpoints: HashMap<String, MockEndpoint>,
    /// Recorded synchronous invocations.
    pub invocations: Vec<InvocationRecord>,
    /// Recorded async invocations.
    pub async_invocations: Vec<AsyncInvocationRecord>,
    /// Counter for generating unique output locations.
    pub(crate) async_invocation_counter: u64,
}

#[derive(Debug, Error)]
pub enum SageMakerRuntimeError {
    #[error("Received client error ({status}) for model {model_id} with message \"{message}\"")]
    ModelError {
        message: String,
        model_id: String,
        status: u16,
    },

    #[error("ModelNotReadyException: Model {model_id} is not ready")]
    ModelNotReady { model_id: String },

    #[error("{message}")]
    ValidationError { message: String },

    #[error("Internal failure")]
    InternalFailure,

    #[error("Service unavailable")]
    ServiceUnavailable,
}

impl SageMakerRuntimeState {
    /// Register an endpoint so it can be invoked.
    /// In production, endpoints are created via the SageMaker API (not Runtime).
    /// For mock purposes, we auto-create endpoints on first invocation.
    pub fn ensure_endpoint(&mut self, endpoint_name: &str) {
        self.endpoints
            .entry(endpoint_name.to_string())
            .or_insert_with(|| MockEndpoint {
                endpoint_name: endpoint_name.to_string(),
                created_at: Utc::now(),
            });
    }

    /// Invoke an endpoint synchronously.
    pub fn invoke_endpoint(
        &mut self,
        endpoint_name: &str,
        content_type: Option<String>,
        accept: Option<String>,
        custom_attributes: Option<String>,
        target_model: Option<String>,
        target_variant: Option<String>,
        target_container_hostname: Option<String>,
        inference_id: Option<String>,
        inference_component_name: Option<String>,
    ) -> Result<InvocationRecord, SageMakerRuntimeError> {
        // Auto-register endpoint
        self.ensure_endpoint(endpoint_name);

        let record = InvocationRecord {
            endpoint_name: endpoint_name.to_string(),
            content_type: content_type.clone(),
            accept: accept.clone(),
            custom_attributes,
            target_model,
            target_variant,
            target_container_hostname,
            inference_id,
            inference_component_name,
            timestamp: Utc::now(),
        };

        self.invocations.push(record.clone());
        Ok(record)
    }

    /// Invoke an endpoint asynchronously.
    pub fn invoke_endpoint_async(
        &mut self,
        endpoint_name: &str,
        content_type: Option<String>,
        accept: Option<String>,
        custom_attributes: Option<String>,
        inference_id: Option<String>,
        input_location: Option<String>,
    ) -> Result<AsyncInvocationRecord, SageMakerRuntimeError> {
        // Auto-register endpoint
        self.ensure_endpoint(endpoint_name);

        self.async_invocation_counter += 1;
        let output_location = format!(
            "s3://sagemaker-output-bucket/async-inference/{endpoint_name}/{}/output",
            self.async_invocation_counter
        );

        let record = AsyncInvocationRecord {
            endpoint_name: endpoint_name.to_string(),
            content_type,
            accept,
            custom_attributes,
            inference_id,
            input_location,
            output_location,
            failure_location: None,
            timestamp: Utc::now(),
        };

        self.async_invocations.push(record.clone());
        Ok(record)
    }
}
