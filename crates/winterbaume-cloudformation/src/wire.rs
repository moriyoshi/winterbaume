//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-cloudformation

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

use winterbaume_core::MockResponse;

pub use super::model::*;
fn strip_outer_element(xml: &str) -> &str {
    // Find the end of the opening tag
    if let Some(close_pos) = xml.find('>') {
        let inner_start = close_pos + 1;
        // Find the last closing tag
        if let Some(last_open) = xml.rfind('<') {
            if last_open >= inner_start {
                return &xml[inner_start..last_open];
            }
        }
    }
    xml
}

/// Serialize response for awsQuery protocol.
pub fn serialize_activate_organizations_access_response(
    result: &ActivateOrganizationsAccessOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<ActivateOrganizationsAccessResult>{inner_xml}</ActivateOrganizationsAccessResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ActivateOrganizationsAccessResponse xmlns="http://cloudformation.amazonaws.com/doc/2010-05-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ActivateOrganizationsAccessResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_activate_type_response(result: &ActivateTypeOutput) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<ActivateTypeResult>{inner_xml}</ActivateTypeResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ActivateTypeResponse xmlns="http://cloudformation.amazonaws.com/doc/2010-05-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ActivateTypeResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_batch_describe_type_configurations_response(
    result: &BatchDescribeTypeConfigurationsOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<BatchDescribeTypeConfigurationsResult>{inner_xml}</BatchDescribeTypeConfigurationsResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<BatchDescribeTypeConfigurationsResponse xmlns="http://cloudformation.amazonaws.com/doc/2010-05-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</BatchDescribeTypeConfigurationsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_cancel_update_stack_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CancelUpdateStackResponse xmlns="http://cloudformation.amazonaws.com/doc/2010-05-15/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CancelUpdateStackResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_continue_update_rollback_response(
    result: &ContinueUpdateRollbackOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<ContinueUpdateRollbackResult>{inner_xml}</ContinueUpdateRollbackResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ContinueUpdateRollbackResponse xmlns="http://cloudformation.amazonaws.com/doc/2010-05-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ContinueUpdateRollbackResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_create_change_set_response(result: &CreateChangeSetOutput) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<CreateChangeSetResult>{inner_xml}</CreateChangeSetResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateChangeSetResponse xmlns="http://cloudformation.amazonaws.com/doc/2010-05-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateChangeSetResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_create_generated_template_response(
    result: &CreateGeneratedTemplateOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<CreateGeneratedTemplateResult>{inner_xml}</CreateGeneratedTemplateResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateGeneratedTemplateResponse xmlns="http://cloudformation.amazonaws.com/doc/2010-05-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateGeneratedTemplateResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_create_stack_response(result: &CreateStackOutput) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<CreateStackResult>{inner_xml}</CreateStackResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateStackResponse xmlns="http://cloudformation.amazonaws.com/doc/2010-05-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateStackResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_create_stack_instances_response(
    result: &CreateStackInstancesOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<CreateStackInstancesResult>{inner_xml}</CreateStackInstancesResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateStackInstancesResponse xmlns="http://cloudformation.amazonaws.com/doc/2010-05-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateStackInstancesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_create_stack_refactor_response(
    result: &CreateStackRefactorOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<CreateStackRefactorResult>{inner_xml}</CreateStackRefactorResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateStackRefactorResponse xmlns="http://cloudformation.amazonaws.com/doc/2010-05-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateStackRefactorResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_create_stack_set_response(result: &CreateStackSetOutput) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<CreateStackSetResult>{inner_xml}</CreateStackSetResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateStackSetResponse xmlns="http://cloudformation.amazonaws.com/doc/2010-05-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateStackSetResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_deactivate_organizations_access_response(
    result: &DeactivateOrganizationsAccessOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<DeactivateOrganizationsAccessResult>{inner_xml}</DeactivateOrganizationsAccessResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeactivateOrganizationsAccessResponse xmlns="http://cloudformation.amazonaws.com/doc/2010-05-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeactivateOrganizationsAccessResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_deactivate_type_response(result: &DeactivateTypeOutput) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<DeactivateTypeResult>{inner_xml}</DeactivateTypeResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeactivateTypeResponse xmlns="http://cloudformation.amazonaws.com/doc/2010-05-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeactivateTypeResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_delete_change_set_response(result: &DeleteChangeSetOutput) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<DeleteChangeSetResult>{inner_xml}</DeleteChangeSetResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteChangeSetResponse xmlns="http://cloudformation.amazonaws.com/doc/2010-05-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteChangeSetResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_delete_generated_template_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteGeneratedTemplateResponse xmlns="http://cloudformation.amazonaws.com/doc/2010-05-15/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteGeneratedTemplateResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_delete_stack_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteStackResponse xmlns="http://cloudformation.amazonaws.com/doc/2010-05-15/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteStackResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_delete_stack_instances_response(
    result: &DeleteStackInstancesOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DeleteStackInstancesResult>{inner_xml}</DeleteStackInstancesResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteStackInstancesResponse xmlns="http://cloudformation.amazonaws.com/doc/2010-05-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteStackInstancesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_delete_stack_set_response(result: &DeleteStackSetOutput) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<DeleteStackSetResult>{inner_xml}</DeleteStackSetResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteStackSetResponse xmlns="http://cloudformation.amazonaws.com/doc/2010-05-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteStackSetResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_deregister_type_response(result: &DeregisterTypeOutput) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<DeregisterTypeResult>{inner_xml}</DeregisterTypeResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeregisterTypeResponse xmlns="http://cloudformation.amazonaws.com/doc/2010-05-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeregisterTypeResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_account_limits_response(
    result: &DescribeAccountLimitsOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DescribeAccountLimitsResult>{inner_xml}</DescribeAccountLimitsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeAccountLimitsResponse xmlns="http://cloudformation.amazonaws.com/doc/2010-05-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeAccountLimitsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_change_set_response(result: &DescribeChangeSetOutput) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<DescribeChangeSetResult>{inner_xml}</DescribeChangeSetResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeChangeSetResponse xmlns="http://cloudformation.amazonaws.com/doc/2010-05-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeChangeSetResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_change_set_hooks_response(
    result: &DescribeChangeSetHooksOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DescribeChangeSetHooksResult>{inner_xml}</DescribeChangeSetHooksResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeChangeSetHooksResponse xmlns="http://cloudformation.amazonaws.com/doc/2010-05-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeChangeSetHooksResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_events_response(result: &DescribeEventsOutput) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<DescribeEventsResult>{inner_xml}</DescribeEventsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeEventsResponse xmlns="http://cloudformation.amazonaws.com/doc/2010-05-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeEventsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_generated_template_response(
    result: &DescribeGeneratedTemplateOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DescribeGeneratedTemplateResult>{inner_xml}</DescribeGeneratedTemplateResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeGeneratedTemplateResponse xmlns="http://cloudformation.amazonaws.com/doc/2010-05-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeGeneratedTemplateResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_organizations_access_response(
    result: &DescribeOrganizationsAccessOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<DescribeOrganizationsAccessResult>{inner_xml}</DescribeOrganizationsAccessResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeOrganizationsAccessResponse xmlns="http://cloudformation.amazonaws.com/doc/2010-05-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeOrganizationsAccessResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_publisher_response(result: &DescribePublisherOutput) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<DescribePublisherResult>{inner_xml}</DescribePublisherResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribePublisherResponse xmlns="http://cloudformation.amazonaws.com/doc/2010-05-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribePublisherResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_resource_scan_response(
    result: &DescribeResourceScanOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DescribeResourceScanResult>{inner_xml}</DescribeResourceScanResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeResourceScanResponse xmlns="http://cloudformation.amazonaws.com/doc/2010-05-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeResourceScanResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_stack_drift_detection_status_response(
    result: &DescribeStackDriftDetectionStatusOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<DescribeStackDriftDetectionStatusResult>{inner_xml}</DescribeStackDriftDetectionStatusResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeStackDriftDetectionStatusResponse xmlns="http://cloudformation.amazonaws.com/doc/2010-05-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeStackDriftDetectionStatusResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_stack_events_response(
    result: &DescribeStackEventsOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<DescribeStackEventsResult>{inner_xml}</DescribeStackEventsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeStackEventsResponse xmlns="http://cloudformation.amazonaws.com/doc/2010-05-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeStackEventsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_stack_instance_response(
    result: &DescribeStackInstanceOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DescribeStackInstanceResult>{inner_xml}</DescribeStackInstanceResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeStackInstanceResponse xmlns="http://cloudformation.amazonaws.com/doc/2010-05-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeStackInstanceResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_stack_refactor_response(
    result: &DescribeStackRefactorOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DescribeStackRefactorResult>{inner_xml}</DescribeStackRefactorResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeStackRefactorResponse xmlns="http://cloudformation.amazonaws.com/doc/2010-05-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeStackRefactorResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_stack_resource_response(
    result: &DescribeStackResourceOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DescribeStackResourceResult>{inner_xml}</DescribeStackResourceResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeStackResourceResponse xmlns="http://cloudformation.amazonaws.com/doc/2010-05-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeStackResourceResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_stack_resource_drifts_response(
    result: &DescribeStackResourceDriftsOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<DescribeStackResourceDriftsResult>{inner_xml}</DescribeStackResourceDriftsResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeStackResourceDriftsResponse xmlns="http://cloudformation.amazonaws.com/doc/2010-05-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeStackResourceDriftsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_stack_resources_response(
    result: &DescribeStackResourcesOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DescribeStackResourcesResult>{inner_xml}</DescribeStackResourcesResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeStackResourcesResponse xmlns="http://cloudformation.amazonaws.com/doc/2010-05-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeStackResourcesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_stack_set_response(result: &DescribeStackSetOutput) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<DescribeStackSetResult>{inner_xml}</DescribeStackSetResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeStackSetResponse xmlns="http://cloudformation.amazonaws.com/doc/2010-05-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeStackSetResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_stack_set_operation_response(
    result: &DescribeStackSetOperationOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DescribeStackSetOperationResult>{inner_xml}</DescribeStackSetOperationResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeStackSetOperationResponse xmlns="http://cloudformation.amazonaws.com/doc/2010-05-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeStackSetOperationResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_stacks_response(result: &DescribeStacksOutput) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<DescribeStacksResult>{inner_xml}</DescribeStacksResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeStacksResponse xmlns="http://cloudformation.amazonaws.com/doc/2010-05-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeStacksResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_type_response(result: &DescribeTypeOutput) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<DescribeTypeResult>{inner_xml}</DescribeTypeResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeTypeResponse xmlns="http://cloudformation.amazonaws.com/doc/2010-05-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeTypeResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_type_registration_response(
    result: &DescribeTypeRegistrationOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DescribeTypeRegistrationResult>{inner_xml}</DescribeTypeRegistrationResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeTypeRegistrationResponse xmlns="http://cloudformation.amazonaws.com/doc/2010-05-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeTypeRegistrationResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_detect_stack_drift_response(result: &DetectStackDriftOutput) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<DetectStackDriftResult>{inner_xml}</DetectStackDriftResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DetectStackDriftResponse xmlns="http://cloudformation.amazonaws.com/doc/2010-05-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DetectStackDriftResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_detect_stack_resource_drift_response(
    result: &DetectStackResourceDriftOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DetectStackResourceDriftResult>{inner_xml}</DetectStackResourceDriftResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DetectStackResourceDriftResponse xmlns="http://cloudformation.amazonaws.com/doc/2010-05-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DetectStackResourceDriftResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_detect_stack_set_drift_response(
    result: &DetectStackSetDriftOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<DetectStackSetDriftResult>{inner_xml}</DetectStackSetDriftResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DetectStackSetDriftResponse xmlns="http://cloudformation.amazonaws.com/doc/2010-05-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DetectStackSetDriftResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_estimate_template_cost_response(
    result: &EstimateTemplateCostOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<EstimateTemplateCostResult>{inner_xml}</EstimateTemplateCostResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<EstimateTemplateCostResponse xmlns="http://cloudformation.amazonaws.com/doc/2010-05-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</EstimateTemplateCostResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_execute_change_set_response(result: &ExecuteChangeSetOutput) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<ExecuteChangeSetResult>{inner_xml}</ExecuteChangeSetResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ExecuteChangeSetResponse xmlns="http://cloudformation.amazonaws.com/doc/2010-05-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ExecuteChangeSetResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_execute_stack_refactor_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ExecuteStackRefactorResponse xmlns="http://cloudformation.amazonaws.com/doc/2010-05-15/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ExecuteStackRefactorResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_get_generated_template_response(
    result: &GetGeneratedTemplateOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<GetGeneratedTemplateResult>{inner_xml}</GetGeneratedTemplateResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<GetGeneratedTemplateResponse xmlns="http://cloudformation.amazonaws.com/doc/2010-05-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</GetGeneratedTemplateResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_get_hook_result_response(result: &GetHookResultOutput) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<GetHookResultResult>{inner_xml}</GetHookResultResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<GetHookResultResponse xmlns="http://cloudformation.amazonaws.com/doc/2010-05-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</GetHookResultResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_get_stack_policy_response(result: &GetStackPolicyOutput) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<GetStackPolicyResult>{inner_xml}</GetStackPolicyResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<GetStackPolicyResponse xmlns="http://cloudformation.amazonaws.com/doc/2010-05-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</GetStackPolicyResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_get_template_response(result: &GetTemplateOutput) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<GetTemplateResult>{inner_xml}</GetTemplateResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<GetTemplateResponse xmlns="http://cloudformation.amazonaws.com/doc/2010-05-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</GetTemplateResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_get_template_summary_response(result: &GetTemplateSummaryOutput) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<GetTemplateSummaryResult>{inner_xml}</GetTemplateSummaryResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<GetTemplateSummaryResponse xmlns="http://cloudformation.amazonaws.com/doc/2010-05-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</GetTemplateSummaryResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_import_stacks_to_stack_set_response(
    result: &ImportStacksToStackSetOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<ImportStacksToStackSetResult>{inner_xml}</ImportStacksToStackSetResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ImportStacksToStackSetResponse xmlns="http://cloudformation.amazonaws.com/doc/2010-05-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ImportStacksToStackSetResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_list_change_sets_response(result: &ListChangeSetsOutput) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<ListChangeSetsResult>{inner_xml}</ListChangeSetsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ListChangeSetsResponse xmlns="http://cloudformation.amazonaws.com/doc/2010-05-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListChangeSetsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_list_exports_response(result: &ListExportsOutput) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<ListExportsResult>{inner_xml}</ListExportsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ListExportsResponse xmlns="http://cloudformation.amazonaws.com/doc/2010-05-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListExportsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_list_generated_templates_response(
    result: &ListGeneratedTemplatesOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<ListGeneratedTemplatesResult>{inner_xml}</ListGeneratedTemplatesResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ListGeneratedTemplatesResponse xmlns="http://cloudformation.amazonaws.com/doc/2010-05-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListGeneratedTemplatesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_list_hook_results_response(result: &ListHookResultsOutput) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<ListHookResultsResult>{inner_xml}</ListHookResultsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ListHookResultsResponse xmlns="http://cloudformation.amazonaws.com/doc/2010-05-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListHookResultsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_list_imports_response(result: &ListImportsOutput) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<ListImportsResult>{inner_xml}</ListImportsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ListImportsResponse xmlns="http://cloudformation.amazonaws.com/doc/2010-05-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListImportsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_list_resource_scan_related_resources_response(
    result: &ListResourceScanRelatedResourcesOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<ListResourceScanRelatedResourcesResult>{inner_xml}</ListResourceScanRelatedResourcesResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ListResourceScanRelatedResourcesResponse xmlns="http://cloudformation.amazonaws.com/doc/2010-05-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListResourceScanRelatedResourcesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_list_resource_scan_resources_response(
    result: &ListResourceScanResourcesOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<ListResourceScanResourcesResult>{inner_xml}</ListResourceScanResourcesResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ListResourceScanResourcesResponse xmlns="http://cloudformation.amazonaws.com/doc/2010-05-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListResourceScanResourcesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_list_resource_scans_response(result: &ListResourceScansOutput) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<ListResourceScansResult>{inner_xml}</ListResourceScansResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ListResourceScansResponse xmlns="http://cloudformation.amazonaws.com/doc/2010-05-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListResourceScansResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_list_stack_instance_resource_drifts_response(
    result: &ListStackInstanceResourceDriftsOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<ListStackInstanceResourceDriftsResult>{inner_xml}</ListStackInstanceResourceDriftsResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ListStackInstanceResourceDriftsResponse xmlns="http://cloudformation.amazonaws.com/doc/2010-05-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListStackInstanceResourceDriftsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_list_stack_instances_response(result: &ListStackInstancesOutput) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<ListStackInstancesResult>{inner_xml}</ListStackInstancesResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ListStackInstancesResponse xmlns="http://cloudformation.amazonaws.com/doc/2010-05-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListStackInstancesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_list_stack_refactor_actions_response(
    result: &ListStackRefactorActionsOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<ListStackRefactorActionsResult>{inner_xml}</ListStackRefactorActionsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ListStackRefactorActionsResponse xmlns="http://cloudformation.amazonaws.com/doc/2010-05-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListStackRefactorActionsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_list_stack_refactors_response(result: &ListStackRefactorsOutput) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<ListStackRefactorsResult>{inner_xml}</ListStackRefactorsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ListStackRefactorsResponse xmlns="http://cloudformation.amazonaws.com/doc/2010-05-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListStackRefactorsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_list_stack_resources_response(result: &ListStackResourcesOutput) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<ListStackResourcesResult>{inner_xml}</ListStackResourcesResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ListStackResourcesResponse xmlns="http://cloudformation.amazonaws.com/doc/2010-05-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListStackResourcesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_list_stack_set_auto_deployment_targets_response(
    result: &ListStackSetAutoDeploymentTargetsOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<ListStackSetAutoDeploymentTargetsResult>{inner_xml}</ListStackSetAutoDeploymentTargetsResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ListStackSetAutoDeploymentTargetsResponse xmlns="http://cloudformation.amazonaws.com/doc/2010-05-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListStackSetAutoDeploymentTargetsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_list_stack_set_operation_results_response(
    result: &ListStackSetOperationResultsOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<ListStackSetOperationResultsResult>{inner_xml}</ListStackSetOperationResultsResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ListStackSetOperationResultsResponse xmlns="http://cloudformation.amazonaws.com/doc/2010-05-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListStackSetOperationResultsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_list_stack_set_operations_response(
    result: &ListStackSetOperationsOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<ListStackSetOperationsResult>{inner_xml}</ListStackSetOperationsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ListStackSetOperationsResponse xmlns="http://cloudformation.amazonaws.com/doc/2010-05-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListStackSetOperationsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_list_stack_sets_response(result: &ListStackSetsOutput) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<ListStackSetsResult>{inner_xml}</ListStackSetsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ListStackSetsResponse xmlns="http://cloudformation.amazonaws.com/doc/2010-05-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListStackSetsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_list_stacks_response(result: &ListStacksOutput) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<ListStacksResult>{inner_xml}</ListStacksResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ListStacksResponse xmlns="http://cloudformation.amazonaws.com/doc/2010-05-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListStacksResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_list_type_registrations_response(
    result: &ListTypeRegistrationsOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<ListTypeRegistrationsResult>{inner_xml}</ListTypeRegistrationsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ListTypeRegistrationsResponse xmlns="http://cloudformation.amazonaws.com/doc/2010-05-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListTypeRegistrationsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_list_type_versions_response(result: &ListTypeVersionsOutput) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<ListTypeVersionsResult>{inner_xml}</ListTypeVersionsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ListTypeVersionsResponse xmlns="http://cloudformation.amazonaws.com/doc/2010-05-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListTypeVersionsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_list_types_response(result: &ListTypesOutput) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<ListTypesResult>{inner_xml}</ListTypesResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ListTypesResponse xmlns="http://cloudformation.amazonaws.com/doc/2010-05-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListTypesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_publish_type_response(result: &PublishTypeOutput) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<PublishTypeResult>{inner_xml}</PublishTypeResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<PublishTypeResponse xmlns="http://cloudformation.amazonaws.com/doc/2010-05-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</PublishTypeResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_record_handler_progress_response(
    result: &RecordHandlerProgressOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<RecordHandlerProgressResult>{inner_xml}</RecordHandlerProgressResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<RecordHandlerProgressResponse xmlns="http://cloudformation.amazonaws.com/doc/2010-05-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</RecordHandlerProgressResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_register_publisher_response(result: &RegisterPublisherOutput) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<RegisterPublisherResult>{inner_xml}</RegisterPublisherResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<RegisterPublisherResponse xmlns="http://cloudformation.amazonaws.com/doc/2010-05-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</RegisterPublisherResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_register_type_response(result: &RegisterTypeOutput) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<RegisterTypeResult>{inner_xml}</RegisterTypeResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<RegisterTypeResponse xmlns="http://cloudformation.amazonaws.com/doc/2010-05-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</RegisterTypeResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_rollback_stack_response(result: &RollbackStackOutput) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<RollbackStackResult>{inner_xml}</RollbackStackResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<RollbackStackResponse xmlns="http://cloudformation.amazonaws.com/doc/2010-05-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</RollbackStackResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_set_stack_policy_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<SetStackPolicyResponse xmlns="http://cloudformation.amazonaws.com/doc/2010-05-15/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</SetStackPolicyResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_set_type_configuration_response(
    result: &SetTypeConfigurationOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<SetTypeConfigurationResult>{inner_xml}</SetTypeConfigurationResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<SetTypeConfigurationResponse xmlns="http://cloudformation.amazonaws.com/doc/2010-05-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</SetTypeConfigurationResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_set_type_default_version_response(
    result: &SetTypeDefaultVersionOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<SetTypeDefaultVersionResult>{inner_xml}</SetTypeDefaultVersionResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<SetTypeDefaultVersionResponse xmlns="http://cloudformation.amazonaws.com/doc/2010-05-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</SetTypeDefaultVersionResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_signal_resource_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<SignalResourceResponse xmlns="http://cloudformation.amazonaws.com/doc/2010-05-15/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</SignalResourceResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_start_resource_scan_response(result: &StartResourceScanOutput) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<StartResourceScanResult>{inner_xml}</StartResourceScanResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<StartResourceScanResponse xmlns="http://cloudformation.amazonaws.com/doc/2010-05-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</StartResourceScanResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_stop_stack_set_operation_response(
    result: &StopStackSetOperationOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<StopStackSetOperationResult>{inner_xml}</StopStackSetOperationResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<StopStackSetOperationResponse xmlns="http://cloudformation.amazonaws.com/doc/2010-05-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</StopStackSetOperationResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_test_type_response(result: &TestTypeOutput) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<TestTypeResult>{inner_xml}</TestTypeResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<TestTypeResponse xmlns="http://cloudformation.amazonaws.com/doc/2010-05-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</TestTypeResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_update_generated_template_response(
    result: &UpdateGeneratedTemplateOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<UpdateGeneratedTemplateResult>{inner_xml}</UpdateGeneratedTemplateResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<UpdateGeneratedTemplateResponse xmlns="http://cloudformation.amazonaws.com/doc/2010-05-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</UpdateGeneratedTemplateResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_update_stack_response(result: &UpdateStackOutput) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<UpdateStackResult>{inner_xml}</UpdateStackResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<UpdateStackResponse xmlns="http://cloudformation.amazonaws.com/doc/2010-05-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</UpdateStackResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_update_stack_instances_response(
    result: &UpdateStackInstancesOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<UpdateStackInstancesResult>{inner_xml}</UpdateStackInstancesResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<UpdateStackInstancesResponse xmlns="http://cloudformation.amazonaws.com/doc/2010-05-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</UpdateStackInstancesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_update_stack_set_response(result: &UpdateStackSetOutput) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<UpdateStackSetResult>{inner_xml}</UpdateStackSetResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<UpdateStackSetResponse xmlns="http://cloudformation.amazonaws.com/doc/2010-05-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</UpdateStackSetResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_update_termination_protection_response(
    result: &UpdateTerminationProtectionOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<UpdateTerminationProtectionResult>{inner_xml}</UpdateTerminationProtectionResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<UpdateTerminationProtectionResponse xmlns="http://cloudformation.amazonaws.com/doc/2010-05-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</UpdateTerminationProtectionResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_validate_template_response(result: &ValidateTemplateOutput) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<ValidateTemplateResult>{inner_xml}</ValidateTemplateResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ValidateTemplateResponse xmlns="http://cloudformation.amazonaws.com/doc/2010-05-15/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ValidateTemplateResponse>"#
    );
    MockResponse::xml(200, xml)
}

fn deserialize_parameter_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<Parameter>, String> {
    let mut item = Parameter::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.ParameterKey")) {
        item.parameter_key = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.ParameterValue")) {
        item.parameter_value = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.ResolvedValue")) {
        item.resolved_value = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.UsePreviousValue")) {
        item.use_previous_value = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse UsePreviousValue: {e}"))?,
        );
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_tag_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<Tag>, String> {
    let mut item = Tag::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.Key")) {
        item.key = value.to_string();
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Value")) {
        item.value = value.to_string();
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_template_configuration_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<TemplateConfiguration>, String> {
    let mut item = TemplateConfiguration::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.DeletionPolicy")) {
        item.deletion_policy = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.UpdateReplacePolicy")) {
        item.update_replace_policy = Some(value.to_string());
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_resource_location_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<ResourceLocation>, String> {
    let mut item = ResourceLocation::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.LogicalResourceId")) {
        item.logical_resource_id = value.to_string();
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.StackName")) {
        item.stack_name = value.to_string();
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_stack_definition_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<StackDefinition>, String> {
    let mut item = StackDefinition::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.StackName")) {
        item.stack_name = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.TemplateBody")) {
        item.template_body = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.TemplateURL")) {
        item.template_u_r_l = Some(value.to_string());
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_template_summary_config_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<TemplateSummaryConfig>, String> {
    let mut item = TemplateSummaryConfig::default();
    let mut found = false;
    if let Some(value) = params.get(&format!(
        "{prefix}.TreatUnrecognizedResourceTypesAsWarnings"
    )) {
        item.treat_unrecognized_resource_types_as_warnings =
            Some(value.parse::<bool>().map_err(|e| {
                format!("failed to parse TreatUnrecognizedResourceTypesAsWarnings: {e}")
            })?);
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_logging_config_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<LoggingConfig>, String> {
    let mut item = LoggingConfig::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.LogGroupName")) {
        item.log_group_name = value.to_string();
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.LogRoleArn")) {
        item.log_role_arn = value.to_string();
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_operation_result_filter_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<OperationResultFilter>, String> {
    let mut item = OperationResultFilter::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.Name")) {
        item.name = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Values")) {
        item.values = Some(value.to_string());
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_type_filters_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<TypeFilters>, String> {
    let mut item = TypeFilters::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.Category")) {
        item.category = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.PublisherId")) {
        item.publisher_id = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.TypeNamePrefix")) {
        item.type_name_prefix = Some(value.to_string());
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_scan_filter_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<ScanFilter>, String> {
    let mut item = ScanFilter::default();
    let mut found = false;
    {
        let mut sub_items = Vec::new();
        let sub_list_prefix = format!("{prefix}.Types");
        for i in 1.. {
            let item_key = format!("{sub_list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => sub_items.push(value.to_string()),
                None => break,
            }
        }
        if !sub_items.is_empty() {
            item.types = Some(ResourceTypeFilters { items: sub_items });
            found = true;
        }
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_rollback_configuration_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<RollbackConfiguration>, String> {
    let mut item = RollbackConfiguration::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.MonitoringTimeInMinutes")) {
        item.monitoring_time_in_minutes = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MonitoringTimeInMinutes: {e}"))?,
        );
        found = true;
    }
    {
        let mut sub_items = Vec::new();
        let sub_list_prefix = format!("{prefix}.RollbackTriggers");
        for i in 1.. {
            let item_key = format!("{sub_list_prefix}.member.{i}");
            match deserialize_rollback_trigger_from_query(params, &item_key)? {
                Some(sub_item) => sub_items.push(sub_item),
                None => break,
            }
        }
        if !sub_items.is_empty() {
            item.rollback_triggers = Some(RollbackTriggers { items: sub_items });
            found = true;
        }
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_event_filter_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<EventFilter>, String> {
    let mut item = EventFilter::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.FailedEvents")) {
        item.failed_events = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse FailedEvents: {e}"))?,
        );
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_resource_definition_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<ResourceDefinition>, String> {
    let mut item = ResourceDefinition::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.LogicalResourceId")) {
        item.logical_resource_id = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.ResourceType")) {
        item.resource_type = value.to_string();
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_rollback_trigger_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<RollbackTrigger>, String> {
    let mut item = RollbackTrigger::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.Arn")) {
        item.arn = value.to_string();
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Type")) {
        item.r#type = value.to_string();
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_resource_mapping_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<ResourceMapping>, String> {
    let mut item = ResourceMapping::default();
    let mut found = false;
    Ok(if found { Some(item) } else { None })
}

fn deserialize_auto_deployment_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<AutoDeployment>, String> {
    let mut item = AutoDeployment::default();
    let mut found = false;
    {
        let mut sub_items = Vec::new();
        let sub_list_prefix = format!("{prefix}.DependsOn");
        for i in 1.. {
            let item_key = format!("{sub_list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => sub_items.push(value.to_string()),
                None => break,
            }
        }
        if !sub_items.is_empty() {
            item.depends_on = Some(StackSetARNList { items: sub_items });
            found = true;
        }
    }
    if let Some(value) = params.get(&format!("{prefix}.Enabled")) {
        item.enabled = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse Enabled: {e}"))?,
        );
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.RetainStacksOnAccountRemoval")) {
        item.retain_stacks_on_account_removal = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse RetainStacksOnAccountRemoval: {e}"))?,
        );
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_scanned_resource_identifier_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<ScannedResourceIdentifier>, String> {
    let mut item = ScannedResourceIdentifier::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.ResourceType")) {
        item.resource_type = value.to_string();
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_deployment_targets_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<DeploymentTargets>, String> {
    let mut item = DeploymentTargets::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.AccountFilterType")) {
        item.account_filter_type = Some(value.to_string());
        found = true;
    }
    {
        let mut sub_items = Vec::new();
        let sub_list_prefix = format!("{prefix}.Accounts");
        for i in 1.. {
            let item_key = format!("{sub_list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => sub_items.push(value.to_string()),
                None => break,
            }
        }
        if !sub_items.is_empty() {
            item.accounts = Some(AccountList { items: sub_items });
            found = true;
        }
    }
    if let Some(value) = params.get(&format!("{prefix}.AccountsUrl")) {
        item.accounts_url = Some(value.to_string());
        found = true;
    }
    {
        let mut sub_items = Vec::new();
        let sub_list_prefix = format!("{prefix}.OrganizationalUnitIds");
        for i in 1.. {
            let item_key = format!("{sub_list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => sub_items.push(value.to_string()),
                None => break,
            }
        }
        if !sub_items.is_empty() {
            item.organizational_unit_ids = Some(OrganizationalUnitIdList { items: sub_items });
            found = true;
        }
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_resource_to_import_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<ResourceToImport>, String> {
    let mut item = ResourceToImport::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.LogicalResourceId")) {
        item.logical_resource_id = value.to_string();
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.ResourceType")) {
        item.resource_type = value.to_string();
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_managed_execution_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<ManagedExecution>, String> {
    let mut item = ManagedExecution::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.Active")) {
        item.active = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse Active: {e}"))?,
        );
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_stack_set_operation_preferences_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<StackSetOperationPreferences>, String> {
    let mut item = StackSetOperationPreferences::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.ConcurrencyMode")) {
        item.concurrency_mode = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.FailureToleranceCount")) {
        item.failure_tolerance_count = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse FailureToleranceCount: {e}"))?,
        );
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.FailureTolerancePercentage")) {
        item.failure_tolerance_percentage = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse FailureTolerancePercentage: {e}"))?,
        );
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.MaxConcurrentCount")) {
        item.max_concurrent_count = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxConcurrentCount: {e}"))?,
        );
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.MaxConcurrentPercentage")) {
        item.max_concurrent_percentage = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxConcurrentPercentage: {e}"))?,
        );
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.RegionConcurrencyType")) {
        item.region_concurrency_type = Some(value.to_string());
        found = true;
    }
    {
        let mut sub_items = Vec::new();
        let sub_list_prefix = format!("{prefix}.RegionOrder");
        for i in 1.. {
            let item_key = format!("{sub_list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => sub_items.push(value.to_string()),
                None => break,
            }
        }
        if !sub_items.is_empty() {
            item.region_order = Some(RegionList { items: sub_items });
            found = true;
        }
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_type_configuration_identifier_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<TypeConfigurationIdentifier>, String> {
    let mut item = TypeConfigurationIdentifier::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.Type")) {
        item.r#type = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.TypeArn")) {
        item.type_arn = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.TypeConfigurationAlias")) {
        item.type_configuration_alias = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.TypeConfigurationArn")) {
        item.type_configuration_arn = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.TypeName")) {
        item.type_name = Some(value.to_string());
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_stack_instance_filter_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<StackInstanceFilter>, String> {
    let mut item = StackInstanceFilter::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.Name")) {
        item.name = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Values")) {
        item.values = Some(value.to_string());
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

/// Deserialize awsQuery request for ActivateOrganizationsAccess.
pub fn deserialize_activate_organizations_access_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ActivateOrganizationsAccessInput, String> {
    let input = ActivateOrganizationsAccessInput {};
    Ok(input)
}

/// Deserialize awsQuery request for ActivateType.
pub fn deserialize_activate_type_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ActivateTypeInput, String> {
    let mut input = ActivateTypeInput::default();
    if let Some(value) = params.get("AutoUpdate") {
        input.auto_update = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse AutoUpdate: {e}"))?,
        );
    }
    if let Some(value) = params.get("ExecutionRoleArn") {
        input.execution_role_arn = Some(value.to_string());
    }
    if let Some(val) = deserialize_logging_config_from_query(params, "LoggingConfig")? {
        input.logging_config = Some(val);
    }
    if let Some(value) = params.get("MajorVersion") {
        input.major_version = Some(
            value
                .parse::<i64>()
                .map_err(|e| format!("failed to parse MajorVersion: {e}"))?,
        );
    }
    if let Some(value) = params.get("PublicTypeArn") {
        input.public_type_arn = Some(value.to_string());
    }
    if let Some(value) = params.get("PublisherId") {
        input.publisher_id = Some(value.to_string());
    }
    if let Some(value) = params.get("Type") {
        input.r#type = Some(value.to_string());
    }
    if let Some(value) = params.get("TypeName") {
        input.type_name = Some(value.to_string());
    }
    if let Some(value) = params.get("TypeNameAlias") {
        input.type_name_alias = Some(value.to_string());
    }
    if let Some(value) = params.get("VersionBump") {
        input.version_bump = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for BatchDescribeTypeConfigurations.
pub fn deserialize_batch_describe_type_configurations_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<BatchDescribeTypeConfigurationsInput, String> {
    let mut input = BatchDescribeTypeConfigurationsInput::default();
    {
        let mut items = Vec::new();
        let list_prefix = "TypeConfigurationIdentifiers".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_type_configuration_identifier_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.type_configuration_identifiers = TypeConfigurationIdentifiers { items };
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for CancelUpdateStack.
pub fn deserialize_cancel_update_stack_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CancelUpdateStackInput, String> {
    let mut input = CancelUpdateStackInput::default();
    if let Some(value) = params.get("ClientRequestToken") {
        input.client_request_token = Some(value.to_string());
    }
    if let Some(value) = params.get("StackName") {
        input.stack_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for ContinueUpdateRollback.
pub fn deserialize_continue_update_rollback_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ContinueUpdateRollbackInput, String> {
    let mut input = ContinueUpdateRollbackInput::default();
    if let Some(value) = params.get("ClientRequestToken") {
        input.client_request_token = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "ResourcesToSkip".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.resources_to_skip = Some(ResourcesToSkip { items });
        }
    }
    if let Some(value) = params.get("RoleARN") {
        input.role_a_r_n = Some(value.to_string());
    }
    if let Some(value) = params.get("StackName") {
        input.stack_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for CreateChangeSet.
pub fn deserialize_create_change_set_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateChangeSetInput, String> {
    let mut input = CreateChangeSetInput::default();
    {
        let mut items = Vec::new();
        let list_prefix = "Capabilities".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.capabilities = Some(Capabilities { items });
        }
    }
    if let Some(value) = params.get("ChangeSetName") {
        input.change_set_name = value.to_string();
    }
    if let Some(value) = params.get("ChangeSetType") {
        input.change_set_type = Some(value.to_string());
    }
    if let Some(value) = params.get("ClientToken") {
        input.client_token = Some(value.to_string());
    }
    if let Some(value) = params.get("DeploymentMode") {
        input.deployment_mode = Some(value.to_string());
    }
    if let Some(value) = params.get("Description") {
        input.description = Some(value.to_string());
    }
    if let Some(value) = params.get("ImportExistingResources") {
        input.import_existing_resources = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse ImportExistingResources: {e}"))?,
        );
    }
    if let Some(value) = params.get("IncludeNestedStacks") {
        input.include_nested_stacks = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse IncludeNestedStacks: {e}"))?,
        );
    }
    {
        let mut items = Vec::new();
        let list_prefix = "NotificationARNs".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.notification_a_r_ns = Some(NotificationARNs { items });
        }
    }
    if let Some(value) = params.get("OnStackFailure") {
        input.on_stack_failure = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Parameters".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_parameter_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.parameters = Some(Parameters { items });
        }
    }
    {
        let mut items = Vec::new();
        let list_prefix = "ResourceTypes".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.resource_types = Some(ResourceTypes { items });
        }
    }
    {
        let mut items = Vec::new();
        let list_prefix = "ResourcesToImport".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_resource_to_import_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.resources_to_import = Some(ResourcesToImport { items });
        }
    }
    if let Some(value) = params.get("RoleARN") {
        input.role_a_r_n = Some(value.to_string());
    }
    if let Some(val) =
        deserialize_rollback_configuration_from_query(params, "RollbackConfiguration")?
    {
        input.rollback_configuration = Some(val);
    }
    if let Some(value) = params.get("StackName") {
        input.stack_name = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Tags".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_tag_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tags = Some(Tags { items });
        }
    }
    if let Some(value) = params.get("TemplateBody") {
        input.template_body = Some(value.to_string());
    }
    if let Some(value) = params.get("TemplateURL") {
        input.template_u_r_l = Some(value.to_string());
    }
    if let Some(value) = params.get("UsePreviousTemplate") {
        input.use_previous_template = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse UsePreviousTemplate: {e}"))?,
        );
    }
    Ok(input)
}

/// Deserialize awsQuery request for CreateGeneratedTemplate.
pub fn deserialize_create_generated_template_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateGeneratedTemplateInput, String> {
    let mut input = CreateGeneratedTemplateInput::default();
    if let Some(value) = params.get("GeneratedTemplateName") {
        input.generated_template_name = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Resources".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_resource_definition_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.resources = Some(ResourceDefinitions { items });
        }
    }
    if let Some(value) = params.get("StackName") {
        input.stack_name = Some(value.to_string());
    }
    if let Some(val) =
        deserialize_template_configuration_from_query(params, "TemplateConfiguration")?
    {
        input.template_configuration = Some(val);
    }
    Ok(input)
}

/// Deserialize awsQuery request for CreateStack.
pub fn deserialize_create_stack_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateStackInput, String> {
    let mut input = CreateStackInput::default();
    {
        let mut items = Vec::new();
        let list_prefix = "Capabilities".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.capabilities = Some(Capabilities { items });
        }
    }
    if let Some(value) = params.get("ClientRequestToken") {
        input.client_request_token = Some(value.to_string());
    }
    if let Some(value) = params.get("DisableRollback") {
        input.disable_rollback = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse DisableRollback: {e}"))?,
        );
    }
    if let Some(value) = params.get("EnableTerminationProtection") {
        input.enable_termination_protection = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse EnableTerminationProtection: {e}"))?,
        );
    }
    {
        let mut items = Vec::new();
        let list_prefix = "NotificationARNs".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.notification_a_r_ns = Some(NotificationARNs { items });
        }
    }
    if let Some(value) = params.get("OnFailure") {
        input.on_failure = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Parameters".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_parameter_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.parameters = Some(Parameters { items });
        }
    }
    {
        let mut items = Vec::new();
        let list_prefix = "ResourceTypes".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.resource_types = Some(ResourceTypes { items });
        }
    }
    if let Some(value) = params.get("RetainExceptOnCreate") {
        input.retain_except_on_create = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse RetainExceptOnCreate: {e}"))?,
        );
    }
    if let Some(value) = params.get("RoleARN") {
        input.role_a_r_n = Some(value.to_string());
    }
    if let Some(val) =
        deserialize_rollback_configuration_from_query(params, "RollbackConfiguration")?
    {
        input.rollback_configuration = Some(val);
    }
    if let Some(value) = params.get("StackName") {
        input.stack_name = value.to_string();
    }
    if let Some(value) = params.get("StackPolicyBody") {
        input.stack_policy_body = Some(value.to_string());
    }
    if let Some(value) = params.get("StackPolicyURL") {
        input.stack_policy_u_r_l = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Tags".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_tag_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tags = Some(Tags { items });
        }
    }
    if let Some(value) = params.get("TemplateBody") {
        input.template_body = Some(value.to_string());
    }
    if let Some(value) = params.get("TemplateURL") {
        input.template_u_r_l = Some(value.to_string());
    }
    if let Some(value) = params.get("TimeoutInMinutes") {
        input.timeout_in_minutes = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse TimeoutInMinutes: {e}"))?,
        );
    }
    Ok(input)
}

/// Deserialize awsQuery request for CreateStackInstances.
pub fn deserialize_create_stack_instances_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateStackInstancesInput, String> {
    let mut input = CreateStackInstancesInput::default();
    {
        let mut items = Vec::new();
        let list_prefix = "Accounts".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.accounts = Some(AccountList { items });
        }
    }
    if let Some(value) = params.get("CallAs") {
        input.call_as = Some(value.to_string());
    }
    if let Some(val) = deserialize_deployment_targets_from_query(params, "DeploymentTargets")? {
        input.deployment_targets = Some(val);
    }
    if let Some(value) = params.get("OperationId") {
        input.operation_id = Some(value.to_string());
    }
    if let Some(val) =
        deserialize_stack_set_operation_preferences_from_query(params, "OperationPreferences")?
    {
        input.operation_preferences = Some(val);
    }
    {
        let mut items = Vec::new();
        let list_prefix = "ParameterOverrides".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_parameter_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.parameter_overrides = Some(Parameters { items });
        }
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Regions".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.regions = RegionList { items };
        }
    }
    if let Some(value) = params.get("StackSetName") {
        input.stack_set_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for CreateStackRefactor.
pub fn deserialize_create_stack_refactor_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateStackRefactorInput, String> {
    let mut input = CreateStackRefactorInput::default();
    if let Some(value) = params.get("Description") {
        input.description = Some(value.to_string());
    }
    if let Some(value) = params.get("EnableStackCreation") {
        input.enable_stack_creation = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse EnableStackCreation: {e}"))?,
        );
    }
    {
        let mut items = Vec::new();
        let list_prefix = "ResourceMappings".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_resource_mapping_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.resource_mappings = Some(ResourceMappings { items });
        }
    }
    {
        let mut items = Vec::new();
        let list_prefix = "StackDefinitions".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_stack_definition_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.stack_definitions = StackDefinitions { items };
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for CreateStackSet.
pub fn deserialize_create_stack_set_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateStackSetInput, String> {
    let mut input = CreateStackSetInput::default();
    if let Some(value) = params.get("AdministrationRoleARN") {
        input.administration_role_a_r_n = Some(value.to_string());
    }
    if let Some(val) = deserialize_auto_deployment_from_query(params, "AutoDeployment")? {
        input.auto_deployment = Some(val);
    }
    if let Some(value) = params.get("CallAs") {
        input.call_as = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Capabilities".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.capabilities = Some(Capabilities { items });
        }
    }
    if let Some(value) = params.get("ClientRequestToken") {
        input.client_request_token = Some(value.to_string());
    }
    if let Some(value) = params.get("Description") {
        input.description = Some(value.to_string());
    }
    if let Some(value) = params.get("ExecutionRoleName") {
        input.execution_role_name = Some(value.to_string());
    }
    if let Some(val) = deserialize_managed_execution_from_query(params, "ManagedExecution")? {
        input.managed_execution = Some(val);
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Parameters".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_parameter_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.parameters = Some(Parameters { items });
        }
    }
    if let Some(value) = params.get("PermissionModel") {
        input.permission_model = Some(value.to_string());
    }
    if let Some(value) = params.get("StackId") {
        input.stack_id = Some(value.to_string());
    }
    if let Some(value) = params.get("StackSetName") {
        input.stack_set_name = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Tags".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_tag_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tags = Some(Tags { items });
        }
    }
    if let Some(value) = params.get("TemplateBody") {
        input.template_body = Some(value.to_string());
    }
    if let Some(value) = params.get("TemplateURL") {
        input.template_u_r_l = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeactivateOrganizationsAccess.
pub fn deserialize_deactivate_organizations_access_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeactivateOrganizationsAccessInput, String> {
    let input = DeactivateOrganizationsAccessInput {};
    Ok(input)
}

/// Deserialize awsQuery request for DeactivateType.
pub fn deserialize_deactivate_type_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeactivateTypeInput, String> {
    let mut input = DeactivateTypeInput::default();
    if let Some(value) = params.get("Arn") {
        input.arn = Some(value.to_string());
    }
    if let Some(value) = params.get("Type") {
        input.r#type = Some(value.to_string());
    }
    if let Some(value) = params.get("TypeName") {
        input.type_name = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteChangeSet.
pub fn deserialize_delete_change_set_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteChangeSetInput, String> {
    let mut input = DeleteChangeSetInput::default();
    if let Some(value) = params.get("ChangeSetName") {
        input.change_set_name = value.to_string();
    }
    if let Some(value) = params.get("StackName") {
        input.stack_name = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteGeneratedTemplate.
pub fn deserialize_delete_generated_template_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteGeneratedTemplateInput, String> {
    let mut input = DeleteGeneratedTemplateInput::default();
    if let Some(value) = params.get("GeneratedTemplateName") {
        input.generated_template_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteStack.
pub fn deserialize_delete_stack_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteStackInput, String> {
    let mut input = DeleteStackInput::default();
    if let Some(value) = params.get("ClientRequestToken") {
        input.client_request_token = Some(value.to_string());
    }
    if let Some(value) = params.get("DeletionMode") {
        input.deletion_mode = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "RetainResources".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.retain_resources = Some(RetainResources { items });
        }
    }
    if let Some(value) = params.get("RoleARN") {
        input.role_a_r_n = Some(value.to_string());
    }
    if let Some(value) = params.get("StackName") {
        input.stack_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteStackInstances.
pub fn deserialize_delete_stack_instances_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteStackInstancesInput, String> {
    let mut input = DeleteStackInstancesInput::default();
    {
        let mut items = Vec::new();
        let list_prefix = "Accounts".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.accounts = Some(AccountList { items });
        }
    }
    if let Some(value) = params.get("CallAs") {
        input.call_as = Some(value.to_string());
    }
    if let Some(val) = deserialize_deployment_targets_from_query(params, "DeploymentTargets")? {
        input.deployment_targets = Some(val);
    }
    if let Some(value) = params.get("OperationId") {
        input.operation_id = Some(value.to_string());
    }
    if let Some(val) =
        deserialize_stack_set_operation_preferences_from_query(params, "OperationPreferences")?
    {
        input.operation_preferences = Some(val);
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Regions".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.regions = RegionList { items };
        }
    }
    if let Some(value) = params.get("RetainStacks") {
        input.retain_stacks = value
            .parse::<bool>()
            .map_err(|e| format!("failed to parse RetainStacks: {e}"))?;
    }
    if let Some(value) = params.get("StackSetName") {
        input.stack_set_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteStackSet.
pub fn deserialize_delete_stack_set_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteStackSetInput, String> {
    let mut input = DeleteStackSetInput::default();
    if let Some(value) = params.get("CallAs") {
        input.call_as = Some(value.to_string());
    }
    if let Some(value) = params.get("StackSetName") {
        input.stack_set_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeregisterType.
pub fn deserialize_deregister_type_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeregisterTypeInput, String> {
    let mut input = DeregisterTypeInput::default();
    if let Some(value) = params.get("Arn") {
        input.arn = Some(value.to_string());
    }
    if let Some(value) = params.get("Type") {
        input.r#type = Some(value.to_string());
    }
    if let Some(value) = params.get("TypeName") {
        input.type_name = Some(value.to_string());
    }
    if let Some(value) = params.get("VersionId") {
        input.version_id = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeAccountLimits.
pub fn deserialize_describe_account_limits_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeAccountLimitsInput, String> {
    let mut input = DescribeAccountLimitsInput::default();
    if let Some(value) = params.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeChangeSet.
pub fn deserialize_describe_change_set_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeChangeSetInput, String> {
    let mut input = DescribeChangeSetInput::default();
    if let Some(value) = params.get("ChangeSetName") {
        input.change_set_name = value.to_string();
    }
    if let Some(value) = params.get("IncludePropertyValues") {
        input.include_property_values = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse IncludePropertyValues: {e}"))?,
        );
    }
    if let Some(value) = params.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = params.get("StackName") {
        input.stack_name = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeChangeSetHooks.
pub fn deserialize_describe_change_set_hooks_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeChangeSetHooksInput, String> {
    let mut input = DescribeChangeSetHooksInput::default();
    if let Some(value) = params.get("ChangeSetName") {
        input.change_set_name = value.to_string();
    }
    if let Some(value) = params.get("LogicalResourceId") {
        input.logical_resource_id = Some(value.to_string());
    }
    if let Some(value) = params.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = params.get("StackName") {
        input.stack_name = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeEvents.
pub fn deserialize_describe_events_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeEventsInput, String> {
    let mut input = DescribeEventsInput::default();
    if let Some(value) = params.get("ChangeSetName") {
        input.change_set_name = Some(value.to_string());
    }
    if let Some(val) = deserialize_event_filter_from_query(params, "Filters")? {
        input.filters = Some(val);
    }
    if let Some(value) = params.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = params.get("OperationId") {
        input.operation_id = Some(value.to_string());
    }
    if let Some(value) = params.get("StackName") {
        input.stack_name = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeGeneratedTemplate.
pub fn deserialize_describe_generated_template_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeGeneratedTemplateInput, String> {
    let mut input = DescribeGeneratedTemplateInput::default();
    if let Some(value) = params.get("GeneratedTemplateName") {
        input.generated_template_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeOrganizationsAccess.
pub fn deserialize_describe_organizations_access_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeOrganizationsAccessInput, String> {
    let mut input = DescribeOrganizationsAccessInput::default();
    if let Some(value) = params.get("CallAs") {
        input.call_as = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribePublisher.
pub fn deserialize_describe_publisher_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribePublisherInput, String> {
    let mut input = DescribePublisherInput::default();
    if let Some(value) = params.get("PublisherId") {
        input.publisher_id = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeResourceScan.
pub fn deserialize_describe_resource_scan_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeResourceScanInput, String> {
    let mut input = DescribeResourceScanInput::default();
    if let Some(value) = params.get("ResourceScanId") {
        input.resource_scan_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeStackDriftDetectionStatus.
pub fn deserialize_describe_stack_drift_detection_status_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeStackDriftDetectionStatusInput, String> {
    let mut input = DescribeStackDriftDetectionStatusInput::default();
    if let Some(value) = params.get("StackDriftDetectionId") {
        input.stack_drift_detection_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeStackEvents.
pub fn deserialize_describe_stack_events_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeStackEventsInput, String> {
    let mut input = DescribeStackEventsInput::default();
    if let Some(value) = params.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = params.get("StackName") {
        input.stack_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeStackInstance.
pub fn deserialize_describe_stack_instance_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeStackInstanceInput, String> {
    let mut input = DescribeStackInstanceInput::default();
    if let Some(value) = params.get("CallAs") {
        input.call_as = Some(value.to_string());
    }
    if let Some(value) = params.get("StackInstanceAccount") {
        input.stack_instance_account = value.to_string();
    }
    if let Some(value) = params.get("StackInstanceRegion") {
        input.stack_instance_region = value.to_string();
    }
    if let Some(value) = params.get("StackSetName") {
        input.stack_set_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeStackRefactor.
pub fn deserialize_describe_stack_refactor_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeStackRefactorInput, String> {
    let mut input = DescribeStackRefactorInput::default();
    if let Some(value) = params.get("StackRefactorId") {
        input.stack_refactor_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeStackResource.
pub fn deserialize_describe_stack_resource_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeStackResourceInput, String> {
    let mut input = DescribeStackResourceInput::default();
    if let Some(value) = params.get("LogicalResourceId") {
        input.logical_resource_id = value.to_string();
    }
    if let Some(value) = params.get("StackName") {
        input.stack_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeStackResourceDrifts.
pub fn deserialize_describe_stack_resource_drifts_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeStackResourceDriftsInput, String> {
    let mut input = DescribeStackResourceDriftsInput::default();
    if let Some(value) = params.get("MaxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxResults: {e}"))?,
        );
    }
    if let Some(value) = params.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = params.get("StackName") {
        input.stack_name = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "StackResourceDriftStatusFilters".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.stack_resource_drift_status_filters =
                Some(StackResourceDriftStatusFilters { items });
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeStackResources.
pub fn deserialize_describe_stack_resources_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeStackResourcesInput, String> {
    let mut input = DescribeStackResourcesInput::default();
    if let Some(value) = params.get("LogicalResourceId") {
        input.logical_resource_id = Some(value.to_string());
    }
    if let Some(value) = params.get("PhysicalResourceId") {
        input.physical_resource_id = Some(value.to_string());
    }
    if let Some(value) = params.get("StackName") {
        input.stack_name = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeStackSet.
pub fn deserialize_describe_stack_set_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeStackSetInput, String> {
    let mut input = DescribeStackSetInput::default();
    if let Some(value) = params.get("CallAs") {
        input.call_as = Some(value.to_string());
    }
    if let Some(value) = params.get("StackSetName") {
        input.stack_set_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeStackSetOperation.
pub fn deserialize_describe_stack_set_operation_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeStackSetOperationInput, String> {
    let mut input = DescribeStackSetOperationInput::default();
    if let Some(value) = params.get("CallAs") {
        input.call_as = Some(value.to_string());
    }
    if let Some(value) = params.get("OperationId") {
        input.operation_id = value.to_string();
    }
    if let Some(value) = params.get("StackSetName") {
        input.stack_set_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeStacks.
pub fn deserialize_describe_stacks_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeStacksInput, String> {
    let mut input = DescribeStacksInput::default();
    if let Some(value) = params.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = params.get("StackName") {
        input.stack_name = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeType.
pub fn deserialize_describe_type_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeTypeInput, String> {
    let mut input = DescribeTypeInput::default();
    if let Some(value) = params.get("Arn") {
        input.arn = Some(value.to_string());
    }
    if let Some(value) = params.get("PublicVersionNumber") {
        input.public_version_number = Some(value.to_string());
    }
    if let Some(value) = params.get("PublisherId") {
        input.publisher_id = Some(value.to_string());
    }
    if let Some(value) = params.get("Type") {
        input.r#type = Some(value.to_string());
    }
    if let Some(value) = params.get("TypeName") {
        input.type_name = Some(value.to_string());
    }
    if let Some(value) = params.get("VersionId") {
        input.version_id = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeTypeRegistration.
pub fn deserialize_describe_type_registration_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeTypeRegistrationInput, String> {
    let mut input = DescribeTypeRegistrationInput::default();
    if let Some(value) = params.get("RegistrationToken") {
        input.registration_token = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DetectStackDrift.
pub fn deserialize_detect_stack_drift_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DetectStackDriftInput, String> {
    let mut input = DetectStackDriftInput::default();
    {
        let mut items = Vec::new();
        let list_prefix = "LogicalResourceIds".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.logical_resource_ids = Some(LogicalResourceIds { items });
        }
    }
    if let Some(value) = params.get("StackName") {
        input.stack_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DetectStackResourceDrift.
pub fn deserialize_detect_stack_resource_drift_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DetectStackResourceDriftInput, String> {
    let mut input = DetectStackResourceDriftInput::default();
    if let Some(value) = params.get("LogicalResourceId") {
        input.logical_resource_id = value.to_string();
    }
    if let Some(value) = params.get("StackName") {
        input.stack_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DetectStackSetDrift.
pub fn deserialize_detect_stack_set_drift_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DetectStackSetDriftInput, String> {
    let mut input = DetectStackSetDriftInput::default();
    if let Some(value) = params.get("CallAs") {
        input.call_as = Some(value.to_string());
    }
    if let Some(value) = params.get("OperationId") {
        input.operation_id = Some(value.to_string());
    }
    if let Some(val) =
        deserialize_stack_set_operation_preferences_from_query(params, "OperationPreferences")?
    {
        input.operation_preferences = Some(val);
    }
    if let Some(value) = params.get("StackSetName") {
        input.stack_set_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for EstimateTemplateCost.
pub fn deserialize_estimate_template_cost_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<EstimateTemplateCostInput, String> {
    let mut input = EstimateTemplateCostInput::default();
    {
        let mut items = Vec::new();
        let list_prefix = "Parameters".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_parameter_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.parameters = Some(Parameters { items });
        }
    }
    if let Some(value) = params.get("TemplateBody") {
        input.template_body = Some(value.to_string());
    }
    if let Some(value) = params.get("TemplateURL") {
        input.template_u_r_l = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for ExecuteChangeSet.
pub fn deserialize_execute_change_set_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ExecuteChangeSetInput, String> {
    let mut input = ExecuteChangeSetInput::default();
    if let Some(value) = params.get("ChangeSetName") {
        input.change_set_name = value.to_string();
    }
    if let Some(value) = params.get("ClientRequestToken") {
        input.client_request_token = Some(value.to_string());
    }
    if let Some(value) = params.get("DisableRollback") {
        input.disable_rollback = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse DisableRollback: {e}"))?,
        );
    }
    if let Some(value) = params.get("RetainExceptOnCreate") {
        input.retain_except_on_create = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse RetainExceptOnCreate: {e}"))?,
        );
    }
    if let Some(value) = params.get("StackName") {
        input.stack_name = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for ExecuteStackRefactor.
pub fn deserialize_execute_stack_refactor_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ExecuteStackRefactorInput, String> {
    let mut input = ExecuteStackRefactorInput::default();
    if let Some(value) = params.get("StackRefactorId") {
        input.stack_refactor_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for GetGeneratedTemplate.
pub fn deserialize_get_generated_template_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<GetGeneratedTemplateInput, String> {
    let mut input = GetGeneratedTemplateInput::default();
    if let Some(value) = params.get("Format") {
        input.format = Some(value.to_string());
    }
    if let Some(value) = params.get("GeneratedTemplateName") {
        input.generated_template_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for GetHookResult.
pub fn deserialize_get_hook_result_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<GetHookResultInput, String> {
    let mut input = GetHookResultInput::default();
    if let Some(value) = params.get("HookResultId") {
        input.hook_result_id = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for GetStackPolicy.
pub fn deserialize_get_stack_policy_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<GetStackPolicyInput, String> {
    let mut input = GetStackPolicyInput::default();
    if let Some(value) = params.get("StackName") {
        input.stack_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for GetTemplate.
pub fn deserialize_get_template_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<GetTemplateInput, String> {
    let mut input = GetTemplateInput::default();
    if let Some(value) = params.get("ChangeSetName") {
        input.change_set_name = Some(value.to_string());
    }
    if let Some(value) = params.get("StackName") {
        input.stack_name = Some(value.to_string());
    }
    if let Some(value) = params.get("TemplateStage") {
        input.template_stage = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for GetTemplateSummary.
pub fn deserialize_get_template_summary_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<GetTemplateSummaryInput, String> {
    let mut input = GetTemplateSummaryInput::default();
    if let Some(value) = params.get("CallAs") {
        input.call_as = Some(value.to_string());
    }
    if let Some(value) = params.get("StackName") {
        input.stack_name = Some(value.to_string());
    }
    if let Some(value) = params.get("StackSetName") {
        input.stack_set_name = Some(value.to_string());
    }
    if let Some(value) = params.get("TemplateBody") {
        input.template_body = Some(value.to_string());
    }
    if let Some(val) =
        deserialize_template_summary_config_from_query(params, "TemplateSummaryConfig")?
    {
        input.template_summary_config = Some(val);
    }
    if let Some(value) = params.get("TemplateURL") {
        input.template_u_r_l = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for ImportStacksToStackSet.
pub fn deserialize_import_stacks_to_stack_set_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ImportStacksToStackSetInput, String> {
    let mut input = ImportStacksToStackSetInput::default();
    if let Some(value) = params.get("CallAs") {
        input.call_as = Some(value.to_string());
    }
    if let Some(value) = params.get("OperationId") {
        input.operation_id = Some(value.to_string());
    }
    if let Some(val) =
        deserialize_stack_set_operation_preferences_from_query(params, "OperationPreferences")?
    {
        input.operation_preferences = Some(val);
    }
    {
        let mut items = Vec::new();
        let list_prefix = "OrganizationalUnitIds".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.organizational_unit_ids = Some(OrganizationalUnitIdList { items });
        }
    }
    {
        let mut items = Vec::new();
        let list_prefix = "StackIds".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.stack_ids = Some(StackIdList { items });
        }
    }
    if let Some(value) = params.get("StackIdsUrl") {
        input.stack_ids_url = Some(value.to_string());
    }
    if let Some(value) = params.get("StackSetName") {
        input.stack_set_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for ListChangeSets.
pub fn deserialize_list_change_sets_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ListChangeSetsInput, String> {
    let mut input = ListChangeSetsInput::default();
    if let Some(value) = params.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = params.get("StackName") {
        input.stack_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for ListExports.
pub fn deserialize_list_exports_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ListExportsInput, String> {
    let mut input = ListExportsInput::default();
    if let Some(value) = params.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for ListGeneratedTemplates.
pub fn deserialize_list_generated_templates_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ListGeneratedTemplatesInput, String> {
    let mut input = ListGeneratedTemplatesInput::default();
    if let Some(value) = params.get("MaxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxResults: {e}"))?,
        );
    }
    if let Some(value) = params.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for ListHookResults.
pub fn deserialize_list_hook_results_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ListHookResultsInput, String> {
    let mut input = ListHookResultsInput::default();
    if let Some(value) = params.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = params.get("Status") {
        input.status = Some(value.to_string());
    }
    if let Some(value) = params.get("TargetId") {
        input.target_id = Some(value.to_string());
    }
    if let Some(value) = params.get("TargetType") {
        input.target_type = Some(value.to_string());
    }
    if let Some(value) = params.get("TypeArn") {
        input.type_arn = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for ListImports.
pub fn deserialize_list_imports_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ListImportsInput, String> {
    let mut input = ListImportsInput::default();
    if let Some(value) = params.get("ExportName") {
        input.export_name = value.to_string();
    }
    if let Some(value) = params.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for ListResourceScanRelatedResources.
pub fn deserialize_list_resource_scan_related_resources_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ListResourceScanRelatedResourcesInput, String> {
    let mut input = ListResourceScanRelatedResourcesInput::default();
    if let Some(value) = params.get("MaxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxResults: {e}"))?,
        );
    }
    if let Some(value) = params.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = params.get("ResourceScanId") {
        input.resource_scan_id = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Resources".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_scanned_resource_identifier_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.resources = ScannedResourceIdentifiers { items };
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for ListResourceScanResources.
pub fn deserialize_list_resource_scan_resources_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ListResourceScanResourcesInput, String> {
    let mut input = ListResourceScanResourcesInput::default();
    if let Some(value) = params.get("MaxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxResults: {e}"))?,
        );
    }
    if let Some(value) = params.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = params.get("ResourceIdentifier") {
        input.resource_identifier = Some(value.to_string());
    }
    if let Some(value) = params.get("ResourceScanId") {
        input.resource_scan_id = value.to_string();
    }
    if let Some(value) = params.get("ResourceTypePrefix") {
        input.resource_type_prefix = Some(value.to_string());
    }
    if let Some(value) = params.get("TagKey") {
        input.tag_key = Some(value.to_string());
    }
    if let Some(value) = params.get("TagValue") {
        input.tag_value = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for ListResourceScans.
pub fn deserialize_list_resource_scans_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ListResourceScansInput, String> {
    let mut input = ListResourceScansInput::default();
    if let Some(value) = params.get("MaxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxResults: {e}"))?,
        );
    }
    if let Some(value) = params.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = params.get("ScanTypeFilter") {
        input.scan_type_filter = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for ListStackInstanceResourceDrifts.
pub fn deserialize_list_stack_instance_resource_drifts_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ListStackInstanceResourceDriftsInput, String> {
    let mut input = ListStackInstanceResourceDriftsInput::default();
    if let Some(value) = params.get("CallAs") {
        input.call_as = Some(value.to_string());
    }
    if let Some(value) = params.get("MaxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxResults: {e}"))?,
        );
    }
    if let Some(value) = params.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = params.get("OperationId") {
        input.operation_id = value.to_string();
    }
    if let Some(value) = params.get("StackInstanceAccount") {
        input.stack_instance_account = value.to_string();
    }
    if let Some(value) = params.get("StackInstanceRegion") {
        input.stack_instance_region = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "StackInstanceResourceDriftStatuses".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.stack_instance_resource_drift_statuses =
                Some(StackResourceDriftStatusFilters { items });
        }
    }
    if let Some(value) = params.get("StackSetName") {
        input.stack_set_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for ListStackInstances.
pub fn deserialize_list_stack_instances_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ListStackInstancesInput, String> {
    let mut input = ListStackInstancesInput::default();
    if let Some(value) = params.get("CallAs") {
        input.call_as = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Filters".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_stack_instance_filter_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.filters = Some(StackInstanceFilters { items });
        }
    }
    if let Some(value) = params.get("MaxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxResults: {e}"))?,
        );
    }
    if let Some(value) = params.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = params.get("StackInstanceAccount") {
        input.stack_instance_account = Some(value.to_string());
    }
    if let Some(value) = params.get("StackInstanceRegion") {
        input.stack_instance_region = Some(value.to_string());
    }
    if let Some(value) = params.get("StackSetName") {
        input.stack_set_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for ListStackRefactorActions.
pub fn deserialize_list_stack_refactor_actions_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ListStackRefactorActionsInput, String> {
    let mut input = ListStackRefactorActionsInput::default();
    if let Some(value) = params.get("MaxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxResults: {e}"))?,
        );
    }
    if let Some(value) = params.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = params.get("StackRefactorId") {
        input.stack_refactor_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for ListStackRefactors.
pub fn deserialize_list_stack_refactors_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ListStackRefactorsInput, String> {
    let mut input = ListStackRefactorsInput::default();
    {
        let mut items = Vec::new();
        let list_prefix = "ExecutionStatusFilter".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.execution_status_filter = Some(StackRefactorExecutionStatusFilter { items });
        }
    }
    if let Some(value) = params.get("MaxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxResults: {e}"))?,
        );
    }
    if let Some(value) = params.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for ListStackResources.
pub fn deserialize_list_stack_resources_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ListStackResourcesInput, String> {
    let mut input = ListStackResourcesInput::default();
    if let Some(value) = params.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = params.get("StackName") {
        input.stack_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for ListStackSetAutoDeploymentTargets.
pub fn deserialize_list_stack_set_auto_deployment_targets_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ListStackSetAutoDeploymentTargetsInput, String> {
    let mut input = ListStackSetAutoDeploymentTargetsInput::default();
    if let Some(value) = params.get("CallAs") {
        input.call_as = Some(value.to_string());
    }
    if let Some(value) = params.get("MaxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxResults: {e}"))?,
        );
    }
    if let Some(value) = params.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = params.get("StackSetName") {
        input.stack_set_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for ListStackSetOperationResults.
pub fn deserialize_list_stack_set_operation_results_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ListStackSetOperationResultsInput, String> {
    let mut input = ListStackSetOperationResultsInput::default();
    if let Some(value) = params.get("CallAs") {
        input.call_as = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Filters".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_operation_result_filter_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.filters = Some(OperationResultFilters { items });
        }
    }
    if let Some(value) = params.get("MaxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxResults: {e}"))?,
        );
    }
    if let Some(value) = params.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = params.get("OperationId") {
        input.operation_id = value.to_string();
    }
    if let Some(value) = params.get("StackSetName") {
        input.stack_set_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for ListStackSetOperations.
pub fn deserialize_list_stack_set_operations_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ListStackSetOperationsInput, String> {
    let mut input = ListStackSetOperationsInput::default();
    if let Some(value) = params.get("CallAs") {
        input.call_as = Some(value.to_string());
    }
    if let Some(value) = params.get("MaxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxResults: {e}"))?,
        );
    }
    if let Some(value) = params.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = params.get("StackSetName") {
        input.stack_set_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for ListStackSets.
pub fn deserialize_list_stack_sets_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ListStackSetsInput, String> {
    let mut input = ListStackSetsInput::default();
    if let Some(value) = params.get("CallAs") {
        input.call_as = Some(value.to_string());
    }
    if let Some(value) = params.get("MaxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxResults: {e}"))?,
        );
    }
    if let Some(value) = params.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = params.get("Status") {
        input.status = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for ListStacks.
pub fn deserialize_list_stacks_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ListStacksInput, String> {
    let mut input = ListStacksInput::default();
    if let Some(value) = params.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "StackStatusFilter".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.stack_status_filter = Some(StackStatusFilter { items });
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for ListTypeRegistrations.
pub fn deserialize_list_type_registrations_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ListTypeRegistrationsInput, String> {
    let mut input = ListTypeRegistrationsInput::default();
    if let Some(value) = params.get("MaxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxResults: {e}"))?,
        );
    }
    if let Some(value) = params.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = params.get("RegistrationStatusFilter") {
        input.registration_status_filter = Some(value.to_string());
    }
    if let Some(value) = params.get("Type") {
        input.r#type = Some(value.to_string());
    }
    if let Some(value) = params.get("TypeArn") {
        input.type_arn = Some(value.to_string());
    }
    if let Some(value) = params.get("TypeName") {
        input.type_name = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for ListTypeVersions.
pub fn deserialize_list_type_versions_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ListTypeVersionsInput, String> {
    let mut input = ListTypeVersionsInput::default();
    if let Some(value) = params.get("Arn") {
        input.arn = Some(value.to_string());
    }
    if let Some(value) = params.get("DeprecatedStatus") {
        input.deprecated_status = Some(value.to_string());
    }
    if let Some(value) = params.get("MaxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxResults: {e}"))?,
        );
    }
    if let Some(value) = params.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = params.get("PublisherId") {
        input.publisher_id = Some(value.to_string());
    }
    if let Some(value) = params.get("Type") {
        input.r#type = Some(value.to_string());
    }
    if let Some(value) = params.get("TypeName") {
        input.type_name = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for ListTypes.
pub fn deserialize_list_types_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ListTypesInput, String> {
    let mut input = ListTypesInput::default();
    if let Some(value) = params.get("DeprecatedStatus") {
        input.deprecated_status = Some(value.to_string());
    }
    if let Some(val) = deserialize_type_filters_from_query(params, "Filters")? {
        input.filters = Some(val);
    }
    if let Some(value) = params.get("MaxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxResults: {e}"))?,
        );
    }
    if let Some(value) = params.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = params.get("ProvisioningType") {
        input.provisioning_type = Some(value.to_string());
    }
    if let Some(value) = params.get("Type") {
        input.r#type = Some(value.to_string());
    }
    if let Some(value) = params.get("Visibility") {
        input.visibility = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for PublishType.
pub fn deserialize_publish_type_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<PublishTypeInput, String> {
    let mut input = PublishTypeInput::default();
    if let Some(value) = params.get("Arn") {
        input.arn = Some(value.to_string());
    }
    if let Some(value) = params.get("PublicVersionNumber") {
        input.public_version_number = Some(value.to_string());
    }
    if let Some(value) = params.get("Type") {
        input.r#type = Some(value.to_string());
    }
    if let Some(value) = params.get("TypeName") {
        input.type_name = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for RecordHandlerProgress.
pub fn deserialize_record_handler_progress_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<RecordHandlerProgressInput, String> {
    let mut input = RecordHandlerProgressInput::default();
    if let Some(value) = params.get("BearerToken") {
        input.bearer_token = value.to_string();
    }
    if let Some(value) = params.get("ClientRequestToken") {
        input.client_request_token = Some(value.to_string());
    }
    if let Some(value) = params.get("CurrentOperationStatus") {
        input.current_operation_status = Some(value.to_string());
    }
    if let Some(value) = params.get("ErrorCode") {
        input.error_code = Some(value.to_string());
    }
    if let Some(value) = params.get("OperationStatus") {
        input.operation_status = value.to_string();
    }
    if let Some(value) = params.get("ResourceModel") {
        input.resource_model = Some(value.to_string());
    }
    if let Some(value) = params.get("StatusMessage") {
        input.status_message = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for RegisterPublisher.
pub fn deserialize_register_publisher_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<RegisterPublisherInput, String> {
    let mut input = RegisterPublisherInput::default();
    if let Some(value) = params.get("AcceptTermsAndConditions") {
        input.accept_terms_and_conditions = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse AcceptTermsAndConditions: {e}"))?,
        );
    }
    if let Some(value) = params.get("ConnectionArn") {
        input.connection_arn = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for RegisterType.
pub fn deserialize_register_type_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<RegisterTypeInput, String> {
    let mut input = RegisterTypeInput::default();
    if let Some(value) = params.get("ClientRequestToken") {
        input.client_request_token = Some(value.to_string());
    }
    if let Some(value) = params.get("ExecutionRoleArn") {
        input.execution_role_arn = Some(value.to_string());
    }
    if let Some(val) = deserialize_logging_config_from_query(params, "LoggingConfig")? {
        input.logging_config = Some(val);
    }
    if let Some(value) = params.get("SchemaHandlerPackage") {
        input.schema_handler_package = value.to_string();
    }
    if let Some(value) = params.get("Type") {
        input.r#type = Some(value.to_string());
    }
    if let Some(value) = params.get("TypeName") {
        input.type_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for RollbackStack.
pub fn deserialize_rollback_stack_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<RollbackStackInput, String> {
    let mut input = RollbackStackInput::default();
    if let Some(value) = params.get("ClientRequestToken") {
        input.client_request_token = Some(value.to_string());
    }
    if let Some(value) = params.get("RetainExceptOnCreate") {
        input.retain_except_on_create = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse RetainExceptOnCreate: {e}"))?,
        );
    }
    if let Some(value) = params.get("RoleARN") {
        input.role_a_r_n = Some(value.to_string());
    }
    if let Some(value) = params.get("StackName") {
        input.stack_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for SetStackPolicy.
pub fn deserialize_set_stack_policy_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<SetStackPolicyInput, String> {
    let mut input = SetStackPolicyInput::default();
    if let Some(value) = params.get("StackName") {
        input.stack_name = value.to_string();
    }
    if let Some(value) = params.get("StackPolicyBody") {
        input.stack_policy_body = Some(value.to_string());
    }
    if let Some(value) = params.get("StackPolicyURL") {
        input.stack_policy_u_r_l = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for SetTypeConfiguration.
pub fn deserialize_set_type_configuration_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<SetTypeConfigurationInput, String> {
    let mut input = SetTypeConfigurationInput::default();
    if let Some(value) = params.get("Configuration") {
        input.configuration = value.to_string();
    }
    if let Some(value) = params.get("ConfigurationAlias") {
        input.configuration_alias = Some(value.to_string());
    }
    if let Some(value) = params.get("Type") {
        input.r#type = Some(value.to_string());
    }
    if let Some(value) = params.get("TypeArn") {
        input.type_arn = Some(value.to_string());
    }
    if let Some(value) = params.get("TypeName") {
        input.type_name = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for SetTypeDefaultVersion.
pub fn deserialize_set_type_default_version_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<SetTypeDefaultVersionInput, String> {
    let mut input = SetTypeDefaultVersionInput::default();
    if let Some(value) = params.get("Arn") {
        input.arn = Some(value.to_string());
    }
    if let Some(value) = params.get("Type") {
        input.r#type = Some(value.to_string());
    }
    if let Some(value) = params.get("TypeName") {
        input.type_name = Some(value.to_string());
    }
    if let Some(value) = params.get("VersionId") {
        input.version_id = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for SignalResource.
pub fn deserialize_signal_resource_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<SignalResourceInput, String> {
    let mut input = SignalResourceInput::default();
    if let Some(value) = params.get("LogicalResourceId") {
        input.logical_resource_id = value.to_string();
    }
    if let Some(value) = params.get("StackName") {
        input.stack_name = value.to_string();
    }
    if let Some(value) = params.get("Status") {
        input.status = value.to_string();
    }
    if let Some(value) = params.get("UniqueId") {
        input.unique_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for StartResourceScan.
pub fn deserialize_start_resource_scan_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<StartResourceScanInput, String> {
    let mut input = StartResourceScanInput::default();
    if let Some(value) = params.get("ClientRequestToken") {
        input.client_request_token = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "ScanFilters".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_scan_filter_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.scan_filters = Some(ScanFilters { items });
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for StopStackSetOperation.
pub fn deserialize_stop_stack_set_operation_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<StopStackSetOperationInput, String> {
    let mut input = StopStackSetOperationInput::default();
    if let Some(value) = params.get("CallAs") {
        input.call_as = Some(value.to_string());
    }
    if let Some(value) = params.get("OperationId") {
        input.operation_id = value.to_string();
    }
    if let Some(value) = params.get("StackSetName") {
        input.stack_set_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for TestType.
pub fn deserialize_test_type_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<TestTypeInput, String> {
    let mut input = TestTypeInput::default();
    if let Some(value) = params.get("Arn") {
        input.arn = Some(value.to_string());
    }
    if let Some(value) = params.get("LogDeliveryBucket") {
        input.log_delivery_bucket = Some(value.to_string());
    }
    if let Some(value) = params.get("Type") {
        input.r#type = Some(value.to_string());
    }
    if let Some(value) = params.get("TypeName") {
        input.type_name = Some(value.to_string());
    }
    if let Some(value) = params.get("VersionId") {
        input.version_id = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for UpdateGeneratedTemplate.
pub fn deserialize_update_generated_template_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<UpdateGeneratedTemplateInput, String> {
    let mut input = UpdateGeneratedTemplateInput::default();
    {
        let mut items = Vec::new();
        let list_prefix = "AddResources".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_resource_definition_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.add_resources = Some(ResourceDefinitions { items });
        }
    }
    if let Some(value) = params.get("GeneratedTemplateName") {
        input.generated_template_name = value.to_string();
    }
    if let Some(value) = params.get("NewGeneratedTemplateName") {
        input.new_generated_template_name = Some(value.to_string());
    }
    if let Some(value) = params.get("RefreshAllResources") {
        input.refresh_all_resources = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse RefreshAllResources: {e}"))?,
        );
    }
    {
        let mut items = Vec::new();
        let list_prefix = "RemoveResources".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.remove_resources = Some(JazzLogicalResourceIds { items });
        }
    }
    if let Some(val) =
        deserialize_template_configuration_from_query(params, "TemplateConfiguration")?
    {
        input.template_configuration = Some(val);
    }
    Ok(input)
}

/// Deserialize awsQuery request for UpdateStack.
pub fn deserialize_update_stack_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<UpdateStackInput, String> {
    let mut input = UpdateStackInput::default();
    {
        let mut items = Vec::new();
        let list_prefix = "Capabilities".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.capabilities = Some(Capabilities { items });
        }
    }
    if let Some(value) = params.get("ClientRequestToken") {
        input.client_request_token = Some(value.to_string());
    }
    if let Some(value) = params.get("DisableRollback") {
        input.disable_rollback = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse DisableRollback: {e}"))?,
        );
    }
    {
        let mut items = Vec::new();
        let list_prefix = "NotificationARNs".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.notification_a_r_ns = Some(NotificationARNs { items });
        }
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Parameters".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_parameter_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.parameters = Some(Parameters { items });
        }
    }
    {
        let mut items = Vec::new();
        let list_prefix = "ResourceTypes".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.resource_types = Some(ResourceTypes { items });
        }
    }
    if let Some(value) = params.get("RetainExceptOnCreate") {
        input.retain_except_on_create = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse RetainExceptOnCreate: {e}"))?,
        );
    }
    if let Some(value) = params.get("RoleARN") {
        input.role_a_r_n = Some(value.to_string());
    }
    if let Some(val) =
        deserialize_rollback_configuration_from_query(params, "RollbackConfiguration")?
    {
        input.rollback_configuration = Some(val);
    }
    if let Some(value) = params.get("StackName") {
        input.stack_name = value.to_string();
    }
    if let Some(value) = params.get("StackPolicyBody") {
        input.stack_policy_body = Some(value.to_string());
    }
    if let Some(value) = params.get("StackPolicyDuringUpdateBody") {
        input.stack_policy_during_update_body = Some(value.to_string());
    }
    if let Some(value) = params.get("StackPolicyDuringUpdateURL") {
        input.stack_policy_during_update_u_r_l = Some(value.to_string());
    }
    if let Some(value) = params.get("StackPolicyURL") {
        input.stack_policy_u_r_l = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Tags".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_tag_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tags = Some(Tags { items });
        }
    }
    if let Some(value) = params.get("TemplateBody") {
        input.template_body = Some(value.to_string());
    }
    if let Some(value) = params.get("TemplateURL") {
        input.template_u_r_l = Some(value.to_string());
    }
    if let Some(value) = params.get("UsePreviousTemplate") {
        input.use_previous_template = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse UsePreviousTemplate: {e}"))?,
        );
    }
    Ok(input)
}

/// Deserialize awsQuery request for UpdateStackInstances.
pub fn deserialize_update_stack_instances_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<UpdateStackInstancesInput, String> {
    let mut input = UpdateStackInstancesInput::default();
    {
        let mut items = Vec::new();
        let list_prefix = "Accounts".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.accounts = Some(AccountList { items });
        }
    }
    if let Some(value) = params.get("CallAs") {
        input.call_as = Some(value.to_string());
    }
    if let Some(val) = deserialize_deployment_targets_from_query(params, "DeploymentTargets")? {
        input.deployment_targets = Some(val);
    }
    if let Some(value) = params.get("OperationId") {
        input.operation_id = Some(value.to_string());
    }
    if let Some(val) =
        deserialize_stack_set_operation_preferences_from_query(params, "OperationPreferences")?
    {
        input.operation_preferences = Some(val);
    }
    {
        let mut items = Vec::new();
        let list_prefix = "ParameterOverrides".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_parameter_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.parameter_overrides = Some(Parameters { items });
        }
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Regions".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.regions = RegionList { items };
        }
    }
    if let Some(value) = params.get("StackSetName") {
        input.stack_set_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for UpdateStackSet.
pub fn deserialize_update_stack_set_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<UpdateStackSetInput, String> {
    let mut input = UpdateStackSetInput::default();
    {
        let mut items = Vec::new();
        let list_prefix = "Accounts".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.accounts = Some(AccountList { items });
        }
    }
    if let Some(value) = params.get("AdministrationRoleARN") {
        input.administration_role_a_r_n = Some(value.to_string());
    }
    if let Some(val) = deserialize_auto_deployment_from_query(params, "AutoDeployment")? {
        input.auto_deployment = Some(val);
    }
    if let Some(value) = params.get("CallAs") {
        input.call_as = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Capabilities".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.capabilities = Some(Capabilities { items });
        }
    }
    if let Some(val) = deserialize_deployment_targets_from_query(params, "DeploymentTargets")? {
        input.deployment_targets = Some(val);
    }
    if let Some(value) = params.get("Description") {
        input.description = Some(value.to_string());
    }
    if let Some(value) = params.get("ExecutionRoleName") {
        input.execution_role_name = Some(value.to_string());
    }
    if let Some(val) = deserialize_managed_execution_from_query(params, "ManagedExecution")? {
        input.managed_execution = Some(val);
    }
    if let Some(value) = params.get("OperationId") {
        input.operation_id = Some(value.to_string());
    }
    if let Some(val) =
        deserialize_stack_set_operation_preferences_from_query(params, "OperationPreferences")?
    {
        input.operation_preferences = Some(val);
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Parameters".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_parameter_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.parameters = Some(Parameters { items });
        }
    }
    if let Some(value) = params.get("PermissionModel") {
        input.permission_model = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Regions".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.regions = Some(RegionList { items });
        }
    }
    if let Some(value) = params.get("StackSetName") {
        input.stack_set_name = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Tags".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_tag_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tags = Some(Tags { items });
        }
    }
    if let Some(value) = params.get("TemplateBody") {
        input.template_body = Some(value.to_string());
    }
    if let Some(value) = params.get("TemplateURL") {
        input.template_u_r_l = Some(value.to_string());
    }
    if let Some(value) = params.get("UsePreviousTemplate") {
        input.use_previous_template = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse UsePreviousTemplate: {e}"))?,
        );
    }
    Ok(input)
}

/// Deserialize awsQuery request for UpdateTerminationProtection.
pub fn deserialize_update_termination_protection_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<UpdateTerminationProtectionInput, String> {
    let mut input = UpdateTerminationProtectionInput::default();
    if let Some(value) = params.get("EnableTerminationProtection") {
        input.enable_termination_protection = value
            .parse::<bool>()
            .map_err(|e| format!("failed to parse EnableTerminationProtection: {e}"))?;
    }
    if let Some(value) = params.get("StackName") {
        input.stack_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for ValidateTemplate.
pub fn deserialize_validate_template_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ValidateTemplateInput, String> {
    let mut input = ValidateTemplateInput::default();
    if let Some(value) = params.get("TemplateBody") {
        input.template_body = Some(value.to_string());
    }
    if let Some(value) = params.get("TemplateURL") {
        input.template_u_r_l = Some(value.to_string());
    }
    Ok(input)
}
