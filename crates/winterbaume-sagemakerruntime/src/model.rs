//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-sagemaker-runtime

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InvokeEndpointAsyncInput {
    #[serde(rename = "Accept")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept: Option<String>,
    #[serde(rename = "ContentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(rename = "CustomAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_attributes: Option<String>,
    #[serde(rename = "Filename")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    #[serde(rename = "InferenceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inference_id: Option<String>,
    #[serde(rename = "InputLocation")]
    #[serde(default)]
    pub input_location: String,
    #[serde(rename = "InvocationTimeoutSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invocation_timeout_seconds: Option<i32>,
    #[serde(rename = "RequestTTLSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_t_t_l_seconds: Option<i32>,
    #[serde(rename = "S3OutputPathExtension")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_output_path_extension: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InvokeEndpointAsyncOutput {
    #[serde(rename = "FailureLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_location: Option<String>,
    #[serde(rename = "InferenceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inference_id: Option<String>,
    #[serde(rename = "OutputLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_location: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InvokeEndpointInput {
    #[serde(rename = "Accept")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept: Option<String>,
    #[serde(rename = "Body")]
    #[serde(default)]
    pub body: String,
    #[serde(rename = "ContentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(rename = "CustomAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_attributes: Option<String>,
    #[serde(rename = "EnableExplanations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_explanations: Option<String>,
    #[serde(rename = "InferenceComponentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inference_component_name: Option<String>,
    #[serde(rename = "InferenceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inference_id: Option<String>,
    #[serde(rename = "SessionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(rename = "TargetContainerHostname")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_container_hostname: Option<String>,
    #[serde(rename = "TargetModel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_model: Option<String>,
    #[serde(rename = "TargetVariant")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_variant: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InvokeEndpointOutput {
    #[serde(rename = "Body")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(rename = "ClosedSessionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closed_session_id: Option<String>,
    #[serde(rename = "ContentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(rename = "CustomAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_attributes: Option<String>,
    #[serde(rename = "InvokedProductionVariant")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoked_production_variant: Option<String>,
    #[serde(rename = "NewSessionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_session_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InvokeEndpointWithResponseStreamInput {
    #[serde(rename = "Accept")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept: Option<String>,
    #[serde(rename = "Body")]
    #[serde(default)]
    pub body: String,
    #[serde(rename = "ContentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(rename = "CustomAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_attributes: Option<String>,
    #[serde(rename = "InferenceComponentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inference_component_name: Option<String>,
    #[serde(rename = "InferenceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inference_id: Option<String>,
    #[serde(rename = "SessionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(rename = "TargetContainerHostname")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_container_hostname: Option<String>,
    #[serde(rename = "TargetVariant")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_variant: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InvokeEndpointWithResponseStreamOutput {
    #[serde(rename = "Body")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<ResponseStream>,
    #[serde(rename = "ContentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(rename = "CustomAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_attributes: Option<String>,
    #[serde(rename = "InvokedProductionVariant")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoked_production_variant: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResponseStream {
    #[serde(rename = "InternalStreamFailure")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_stream_failure: Option<InternalStreamFailure>,
    #[serde(rename = "ModelStreamError")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_stream_error: Option<ModelStreamError>,
    #[serde(rename = "PayloadPart")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_part: Option<PayloadPart>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InternalStreamFailure {
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ModelStreamError {
    #[serde(rename = "ErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PayloadPart {
    #[serde(rename = "Bytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes: Option<String>,
}
