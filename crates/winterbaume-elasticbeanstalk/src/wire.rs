//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-elasticbeanstalk

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

/// Serialize void response for awsQuery protocol.
pub fn serialize_abort_environment_update_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<AbortEnvironmentUpdateResponse xmlns="http://elasticbeanstalk.amazonaws.com/docs/2010-12-01/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</AbortEnvironmentUpdateResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_apply_environment_managed_action_response(
    result: &ApplyEnvironmentManagedActionResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<ApplyEnvironmentManagedActionResult>{inner_xml}</ApplyEnvironmentManagedActionResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ApplyEnvironmentManagedActionResponse xmlns="http://elasticbeanstalk.amazonaws.com/docs/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ApplyEnvironmentManagedActionResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_associate_environment_operations_role_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<AssociateEnvironmentOperationsRoleResponse xmlns="http://elasticbeanstalk.amazonaws.com/docs/2010-12-01/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</AssociateEnvironmentOperationsRoleResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_check_d_n_s_availability_response(
    result: &CheckDNSAvailabilityResultMessage,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<CheckDNSAvailabilityResult>{inner_xml}</CheckDNSAvailabilityResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CheckDNSAvailabilityResponse xmlns="http://elasticbeanstalk.amazonaws.com/docs/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CheckDNSAvailabilityResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_compose_environments_response(
    result: &EnvironmentDescriptionsMessage,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<ComposeEnvironmentsResult>{inner_xml}</ComposeEnvironmentsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ComposeEnvironmentsResponse xmlns="http://elasticbeanstalk.amazonaws.com/docs/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ComposeEnvironmentsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_create_application_response(
    result: &ApplicationDescriptionMessage,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<CreateApplicationResult>{inner_xml}</CreateApplicationResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateApplicationResponse xmlns="http://elasticbeanstalk.amazonaws.com/docs/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateApplicationResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_create_application_version_response(
    result: &ApplicationVersionDescriptionMessage,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<CreateApplicationVersionResult>{inner_xml}</CreateApplicationVersionResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateApplicationVersionResponse xmlns="http://elasticbeanstalk.amazonaws.com/docs/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateApplicationVersionResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_create_configuration_template_response(
    result: &ConfigurationSettingsDescription,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<CreateConfigurationTemplateResult>{inner_xml}</CreateConfigurationTemplateResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateConfigurationTemplateResponse xmlns="http://elasticbeanstalk.amazonaws.com/docs/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateConfigurationTemplateResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_create_environment_response(result: &EnvironmentDescription) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<CreateEnvironmentResult>{inner_xml}</CreateEnvironmentResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateEnvironmentResponse xmlns="http://elasticbeanstalk.amazonaws.com/docs/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateEnvironmentResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_create_platform_version_response(
    result: &CreatePlatformVersionResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<CreatePlatformVersionResult>{inner_xml}</CreatePlatformVersionResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreatePlatformVersionResponse xmlns="http://elasticbeanstalk.amazonaws.com/docs/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreatePlatformVersionResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_create_storage_location_response(
    result: &CreateStorageLocationResultMessage,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<CreateStorageLocationResult>{inner_xml}</CreateStorageLocationResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateStorageLocationResponse xmlns="http://elasticbeanstalk.amazonaws.com/docs/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateStorageLocationResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_delete_application_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteApplicationResponse xmlns="http://elasticbeanstalk.amazonaws.com/docs/2010-12-01/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteApplicationResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_delete_application_version_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteApplicationVersionResponse xmlns="http://elasticbeanstalk.amazonaws.com/docs/2010-12-01/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteApplicationVersionResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_delete_configuration_template_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteConfigurationTemplateResponse xmlns="http://elasticbeanstalk.amazonaws.com/docs/2010-12-01/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteConfigurationTemplateResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_delete_environment_configuration_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteEnvironmentConfigurationResponse xmlns="http://elasticbeanstalk.amazonaws.com/docs/2010-12-01/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteEnvironmentConfigurationResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_delete_platform_version_response(
    result: &DeletePlatformVersionResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DeletePlatformVersionResult>{inner_xml}</DeletePlatformVersionResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeletePlatformVersionResponse xmlns="http://elasticbeanstalk.amazonaws.com/docs/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeletePlatformVersionResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_account_attributes_response(
    result: &DescribeAccountAttributesResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DescribeAccountAttributesResult>{inner_xml}</DescribeAccountAttributesResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeAccountAttributesResponse xmlns="http://elasticbeanstalk.amazonaws.com/docs/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeAccountAttributesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_application_versions_response(
    result: &ApplicationVersionDescriptionsMessage,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<DescribeApplicationVersionsResult>{inner_xml}</DescribeApplicationVersionsResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeApplicationVersionsResponse xmlns="http://elasticbeanstalk.amazonaws.com/docs/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeApplicationVersionsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_applications_response(
    result: &ApplicationDescriptionsMessage,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DescribeApplicationsResult>{inner_xml}</DescribeApplicationsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeApplicationsResponse xmlns="http://elasticbeanstalk.amazonaws.com/docs/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeApplicationsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_configuration_options_response(
    result: &ConfigurationOptionsDescription,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<DescribeConfigurationOptionsResult>{inner_xml}</DescribeConfigurationOptionsResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeConfigurationOptionsResponse xmlns="http://elasticbeanstalk.amazonaws.com/docs/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeConfigurationOptionsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_configuration_settings_response(
    result: &ConfigurationSettingsDescriptions,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<DescribeConfigurationSettingsResult>{inner_xml}</DescribeConfigurationSettingsResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeConfigurationSettingsResponse xmlns="http://elasticbeanstalk.amazonaws.com/docs/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeConfigurationSettingsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_environment_health_response(
    result: &DescribeEnvironmentHealthResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DescribeEnvironmentHealthResult>{inner_xml}</DescribeEnvironmentHealthResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeEnvironmentHealthResponse xmlns="http://elasticbeanstalk.amazonaws.com/docs/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeEnvironmentHealthResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_environment_managed_action_history_response(
    result: &DescribeEnvironmentManagedActionHistoryResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<DescribeEnvironmentManagedActionHistoryResult>{inner_xml}</DescribeEnvironmentManagedActionHistoryResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeEnvironmentManagedActionHistoryResponse xmlns="http://elasticbeanstalk.amazonaws.com/docs/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeEnvironmentManagedActionHistoryResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_environment_managed_actions_response(
    result: &DescribeEnvironmentManagedActionsResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<DescribeEnvironmentManagedActionsResult>{inner_xml}</DescribeEnvironmentManagedActionsResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeEnvironmentManagedActionsResponse xmlns="http://elasticbeanstalk.amazonaws.com/docs/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeEnvironmentManagedActionsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_environment_resources_response(
    result: &EnvironmentResourceDescriptionsMessage,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<DescribeEnvironmentResourcesResult>{inner_xml}</DescribeEnvironmentResourcesResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeEnvironmentResourcesResponse xmlns="http://elasticbeanstalk.amazonaws.com/docs/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeEnvironmentResourcesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_environments_response(
    result: &EnvironmentDescriptionsMessage,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DescribeEnvironmentsResult>{inner_xml}</DescribeEnvironmentsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeEnvironmentsResponse xmlns="http://elasticbeanstalk.amazonaws.com/docs/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeEnvironmentsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_events_response(result: &EventDescriptionsMessage) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<DescribeEventsResult>{inner_xml}</DescribeEventsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeEventsResponse xmlns="http://elasticbeanstalk.amazonaws.com/docs/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeEventsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_instances_health_response(
    result: &DescribeInstancesHealthResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DescribeInstancesHealthResult>{inner_xml}</DescribeInstancesHealthResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeInstancesHealthResponse xmlns="http://elasticbeanstalk.amazonaws.com/docs/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeInstancesHealthResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_platform_version_response(
    result: &DescribePlatformVersionResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DescribePlatformVersionResult>{inner_xml}</DescribePlatformVersionResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribePlatformVersionResponse xmlns="http://elasticbeanstalk.amazonaws.com/docs/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribePlatformVersionResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_disassociate_environment_operations_role_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DisassociateEnvironmentOperationsRoleResponse xmlns="http://elasticbeanstalk.amazonaws.com/docs/2010-12-01/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DisassociateEnvironmentOperationsRoleResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_list_available_solution_stacks_response(
    result: &ListAvailableSolutionStacksResultMessage,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<ListAvailableSolutionStacksResult>{inner_xml}</ListAvailableSolutionStacksResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ListAvailableSolutionStacksResponse xmlns="http://elasticbeanstalk.amazonaws.com/docs/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListAvailableSolutionStacksResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_list_platform_branches_response(
    result: &ListPlatformBranchesResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<ListPlatformBranchesResult>{inner_xml}</ListPlatformBranchesResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ListPlatformBranchesResponse xmlns="http://elasticbeanstalk.amazonaws.com/docs/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListPlatformBranchesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_list_platform_versions_response(
    result: &ListPlatformVersionsResult,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<ListPlatformVersionsResult>{inner_xml}</ListPlatformVersionsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ListPlatformVersionsResponse xmlns="http://elasticbeanstalk.amazonaws.com/docs/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListPlatformVersionsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_list_tags_for_resource_response(
    result: &ResourceTagsDescriptionMessage,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<ListTagsForResourceResult>{inner_xml}</ListTagsForResourceResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ListTagsForResourceResponse xmlns="http://elasticbeanstalk.amazonaws.com/docs/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListTagsForResourceResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_rebuild_environment_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<RebuildEnvironmentResponse xmlns="http://elasticbeanstalk.amazonaws.com/docs/2010-12-01/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</RebuildEnvironmentResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_request_environment_info_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<RequestEnvironmentInfoResponse xmlns="http://elasticbeanstalk.amazonaws.com/docs/2010-12-01/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</RequestEnvironmentInfoResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_restart_app_server_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<RestartAppServerResponse xmlns="http://elasticbeanstalk.amazonaws.com/docs/2010-12-01/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</RestartAppServerResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_retrieve_environment_info_response(
    result: &RetrieveEnvironmentInfoResultMessage,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<RetrieveEnvironmentInfoResult>{inner_xml}</RetrieveEnvironmentInfoResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<RetrieveEnvironmentInfoResponse xmlns="http://elasticbeanstalk.amazonaws.com/docs/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</RetrieveEnvironmentInfoResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_swap_environment_c_n_a_m_es_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<SwapEnvironmentCNAMEsResponse xmlns="http://elasticbeanstalk.amazonaws.com/docs/2010-12-01/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</SwapEnvironmentCNAMEsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_terminate_environment_response(result: &EnvironmentDescription) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<TerminateEnvironmentResult>{inner_xml}</TerminateEnvironmentResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<TerminateEnvironmentResponse xmlns="http://elasticbeanstalk.amazonaws.com/docs/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</TerminateEnvironmentResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_update_application_response(
    result: &ApplicationDescriptionMessage,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<UpdateApplicationResult>{inner_xml}</UpdateApplicationResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<UpdateApplicationResponse xmlns="http://elasticbeanstalk.amazonaws.com/docs/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</UpdateApplicationResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_update_application_resource_lifecycle_response(
    result: &ApplicationResourceLifecycleDescriptionMessage,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<UpdateApplicationResourceLifecycleResult>{inner_xml}</UpdateApplicationResourceLifecycleResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<UpdateApplicationResourceLifecycleResponse xmlns="http://elasticbeanstalk.amazonaws.com/docs/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</UpdateApplicationResourceLifecycleResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_update_application_version_response(
    result: &ApplicationVersionDescriptionMessage,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<UpdateApplicationVersionResult>{inner_xml}</UpdateApplicationVersionResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<UpdateApplicationVersionResponse xmlns="http://elasticbeanstalk.amazonaws.com/docs/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</UpdateApplicationVersionResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_update_configuration_template_response(
    result: &ConfigurationSettingsDescription,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<UpdateConfigurationTemplateResult>{inner_xml}</UpdateConfigurationTemplateResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<UpdateConfigurationTemplateResponse xmlns="http://elasticbeanstalk.amazonaws.com/docs/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</UpdateConfigurationTemplateResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_update_environment_response(result: &EnvironmentDescription) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<UpdateEnvironmentResult>{inner_xml}</UpdateEnvironmentResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<UpdateEnvironmentResponse xmlns="http://elasticbeanstalk.amazonaws.com/docs/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</UpdateEnvironmentResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_update_tags_for_resource_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<UpdateTagsForResourceResponse xmlns="http://elasticbeanstalk.amazonaws.com/docs/2010-12-01/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</UpdateTagsForResourceResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_validate_configuration_settings_response(
    result: &ConfigurationSettingsValidationMessages,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<ValidateConfigurationSettingsResult>{inner_xml}</ValidateConfigurationSettingsResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ValidateConfigurationSettingsResponse xmlns="http://elasticbeanstalk.amazonaws.com/docs/2010-12-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ValidateConfigurationSettingsResponse>"#
    );
    MockResponse::xml(200, xml)
}

fn deserialize_platform_filter_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<PlatformFilter>, String> {
    let mut item = PlatformFilter::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.Operator")) {
        item.operator = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Type")) {
        item.r#type = Some(value.to_string());
        found = true;
    }
    {
        let mut sub_items = Vec::new();
        let sub_list_prefix = format!("{prefix}.Values");
        for i in 1.. {
            let item_key = format!("{sub_list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => sub_items.push(value.to_string()),
                None => break,
            }
        }
        if !sub_items.is_empty() {
            item.values = Some(PlatformFilterValueList { items: sub_items });
            found = true;
        }
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_application_resource_lifecycle_config_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<ApplicationResourceLifecycleConfig>, String> {
    let mut item = ApplicationResourceLifecycleConfig::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.ServiceRole")) {
        item.service_role = Some(value.to_string());
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
        item.key = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Value")) {
        item.value = Some(value.to_string());
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_source_configuration_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<SourceConfiguration>, String> {
    let mut item = SourceConfiguration::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.ApplicationName")) {
        item.application_name = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.TemplateName")) {
        item.template_name = Some(value.to_string());
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_build_configuration_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<BuildConfiguration>, String> {
    let mut item = BuildConfiguration::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.ArtifactName")) {
        item.artifact_name = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.CodeBuildServiceRole")) {
        item.code_build_service_role = value.to_string();
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.ComputeType")) {
        item.compute_type = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Image")) {
        item.image = value.to_string();
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.TimeoutInMinutes")) {
        item.timeout_in_minutes = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse TimeoutInMinutes: {e}"))?,
        );
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_source_build_information_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<SourceBuildInformation>, String> {
    let mut item = SourceBuildInformation::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.SourceLocation")) {
        item.source_location = value.to_string();
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.SourceRepository")) {
        item.source_repository = value.to_string();
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.SourceType")) {
        item.source_type = value.to_string();
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_s3_location_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<S3Location>, String> {
    let mut item = S3Location::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.S3Bucket")) {
        item.s3_bucket = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.S3Key")) {
        item.s3_key = Some(value.to_string());
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_search_filter_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<SearchFilter>, String> {
    let mut item = SearchFilter::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.Attribute")) {
        item.attribute = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Operator")) {
        item.operator = Some(value.to_string());
        found = true;
    }
    {
        let mut sub_items = Vec::new();
        let sub_list_prefix = format!("{prefix}.Values");
        for i in 1.. {
            let item_key = format!("{sub_list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => sub_items.push(value.to_string()),
                None => break,
            }
        }
        if !sub_items.is_empty() {
            item.values = Some(SearchFilterValues { items: sub_items });
            found = true;
        }
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_application_version_lifecycle_config_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<ApplicationVersionLifecycleConfig>, String> {
    let mut item = ApplicationVersionLifecycleConfig::default();
    let mut found = false;
    Ok(if found { Some(item) } else { None })
}

fn deserialize_max_age_rule_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<MaxAgeRule>, String> {
    let mut item = MaxAgeRule::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.DeleteSourceFromS3")) {
        item.delete_source_from_s3 = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse DeleteSourceFromS3: {e}"))?,
        );
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Enabled")) {
        item.enabled = value
            .parse::<bool>()
            .map_err(|e| format!("failed to parse Enabled: {e}"))?;
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.MaxAgeInDays")) {
        item.max_age_in_days = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxAgeInDays: {e}"))?,
        );
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_configuration_option_setting_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<ConfigurationOptionSetting>, String> {
    let mut item = ConfigurationOptionSetting::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.Namespace")) {
        item.namespace = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.OptionName")) {
        item.option_name = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.ResourceName")) {
        item.resource_name = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Value")) {
        item.value = Some(value.to_string());
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_max_count_rule_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<MaxCountRule>, String> {
    let mut item = MaxCountRule::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.DeleteSourceFromS3")) {
        item.delete_source_from_s3 = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse DeleteSourceFromS3: {e}"))?,
        );
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Enabled")) {
        item.enabled = value
            .parse::<bool>()
            .map_err(|e| format!("failed to parse Enabled: {e}"))?;
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.MaxCount")) {
        item.max_count = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxCount: {e}"))?,
        );
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_option_specification_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<OptionSpecification>, String> {
    let mut item = OptionSpecification::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.Namespace")) {
        item.namespace = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.OptionName")) {
        item.option_name = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.ResourceName")) {
        item.resource_name = Some(value.to_string());
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_environment_tier_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<EnvironmentTier>, String> {
    let mut item = EnvironmentTier::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.Name")) {
        item.name = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Type")) {
        item.r#type = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Version")) {
        item.version = Some(value.to_string());
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

/// Deserialize awsQuery request for AbortEnvironmentUpdate.
pub fn deserialize_abort_environment_update_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<AbortEnvironmentUpdateMessage, String> {
    let mut input = AbortEnvironmentUpdateMessage::default();
    if let Some(value) = params.get("EnvironmentId") {
        input.environment_id = Some(value.to_string());
    }
    if let Some(value) = params.get("EnvironmentName") {
        input.environment_name = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for ApplyEnvironmentManagedAction.
pub fn deserialize_apply_environment_managed_action_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ApplyEnvironmentManagedActionRequest, String> {
    let mut input = ApplyEnvironmentManagedActionRequest::default();
    if let Some(value) = params.get("ActionId") {
        input.action_id = value.to_string();
    }
    if let Some(value) = params.get("EnvironmentId") {
        input.environment_id = Some(value.to_string());
    }
    if let Some(value) = params.get("EnvironmentName") {
        input.environment_name = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for AssociateEnvironmentOperationsRole.
pub fn deserialize_associate_environment_operations_role_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<AssociateEnvironmentOperationsRoleMessage, String> {
    let mut input = AssociateEnvironmentOperationsRoleMessage::default();
    if let Some(value) = params.get("EnvironmentName") {
        input.environment_name = value.to_string();
    }
    if let Some(value) = params.get("OperationsRole") {
        input.operations_role = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for CheckDNSAvailability.
pub fn deserialize_check_d_n_s_availability_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CheckDNSAvailabilityMessage, String> {
    let mut input = CheckDNSAvailabilityMessage::default();
    if let Some(value) = params.get("CNAMEPrefix") {
        input.c_n_a_m_e_prefix = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for ComposeEnvironments.
pub fn deserialize_compose_environments_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ComposeEnvironmentsMessage, String> {
    let mut input = ComposeEnvironmentsMessage::default();
    if let Some(value) = params.get("ApplicationName") {
        input.application_name = Some(value.to_string());
    }
    if let Some(value) = params.get("GroupName") {
        input.group_name = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "VersionLabels".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.version_labels = Some(VersionLabels { items });
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for CreateApplication.
pub fn deserialize_create_application_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateApplicationMessage, String> {
    let mut input = CreateApplicationMessage::default();
    if let Some(value) = params.get("ApplicationName") {
        input.application_name = value.to_string();
    }
    if let Some(value) = params.get("Description") {
        input.description = Some(value.to_string());
    }
    if let Some(val) = deserialize_application_resource_lifecycle_config_from_query(
        params,
        "ResourceLifecycleConfig",
    )? {
        input.resource_lifecycle_config = Some(val);
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
    Ok(input)
}

/// Deserialize awsQuery request for CreateApplicationVersion.
pub fn deserialize_create_application_version_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateApplicationVersionMessage, String> {
    let mut input = CreateApplicationVersionMessage::default();
    if let Some(value) = params.get("ApplicationName") {
        input.application_name = value.to_string();
    }
    if let Some(value) = params.get("AutoCreateApplication") {
        input.auto_create_application = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse AutoCreateApplication: {e}"))?,
        );
    }
    if let Some(val) = deserialize_build_configuration_from_query(params, "BuildConfiguration")? {
        input.build_configuration = Some(val);
    }
    if let Some(value) = params.get("Description") {
        input.description = Some(value.to_string());
    }
    if let Some(value) = params.get("Process") {
        input.process = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse Process: {e}"))?,
        );
    }
    if let Some(val) =
        deserialize_source_build_information_from_query(params, "SourceBuildInformation")?
    {
        input.source_build_information = Some(val);
    }
    if let Some(val) = deserialize_s3_location_from_query(params, "SourceBundle")? {
        input.source_bundle = Some(val);
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
    if let Some(value) = params.get("VersionLabel") {
        input.version_label = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for CreateConfigurationTemplate.
pub fn deserialize_create_configuration_template_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateConfigurationTemplateMessage, String> {
    let mut input = CreateConfigurationTemplateMessage::default();
    if let Some(value) = params.get("ApplicationName") {
        input.application_name = value.to_string();
    }
    if let Some(value) = params.get("Description") {
        input.description = Some(value.to_string());
    }
    if let Some(value) = params.get("EnvironmentId") {
        input.environment_id = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "OptionSettings".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_configuration_option_setting_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.option_settings = Some(ConfigurationOptionSettingsList { items });
        }
    }
    if let Some(value) = params.get("PlatformArn") {
        input.platform_arn = Some(value.to_string());
    }
    if let Some(value) = params.get("SolutionStackName") {
        input.solution_stack_name = Some(value.to_string());
    }
    if let Some(val) = deserialize_source_configuration_from_query(params, "SourceConfiguration")? {
        input.source_configuration = Some(val);
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
    if let Some(value) = params.get("TemplateName") {
        input.template_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for CreateEnvironment.
pub fn deserialize_create_environment_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateEnvironmentMessage, String> {
    let mut input = CreateEnvironmentMessage::default();
    if let Some(value) = params.get("ApplicationName") {
        input.application_name = value.to_string();
    }
    if let Some(value) = params.get("CNAMEPrefix") {
        input.c_n_a_m_e_prefix = Some(value.to_string());
    }
    if let Some(value) = params.get("Description") {
        input.description = Some(value.to_string());
    }
    if let Some(value) = params.get("EnvironmentName") {
        input.environment_name = Some(value.to_string());
    }
    if let Some(value) = params.get("GroupName") {
        input.group_name = Some(value.to_string());
    }
    if let Some(value) = params.get("OperationsRole") {
        input.operations_role = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "OptionSettings".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_configuration_option_setting_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.option_settings = Some(ConfigurationOptionSettingsList { items });
        }
    }
    {
        let mut items = Vec::new();
        let list_prefix = "OptionsToRemove".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_option_specification_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.options_to_remove = Some(OptionsSpecifierList { items });
        }
    }
    if let Some(value) = params.get("PlatformArn") {
        input.platform_arn = Some(value.to_string());
    }
    if let Some(value) = params.get("SolutionStackName") {
        input.solution_stack_name = Some(value.to_string());
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
    if let Some(value) = params.get("TemplateName") {
        input.template_name = Some(value.to_string());
    }
    if let Some(val) = deserialize_environment_tier_from_query(params, "Tier")? {
        input.tier = Some(val);
    }
    if let Some(value) = params.get("VersionLabel") {
        input.version_label = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for CreatePlatformVersion.
pub fn deserialize_create_platform_version_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreatePlatformVersionRequest, String> {
    let mut input = CreatePlatformVersionRequest::default();
    if let Some(value) = params.get("EnvironmentName") {
        input.environment_name = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "OptionSettings".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_configuration_option_setting_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.option_settings = Some(ConfigurationOptionSettingsList { items });
        }
    }
    if let Some(val) = deserialize_s3_location_from_query(params, "PlatformDefinitionBundle")? {
        input.platform_definition_bundle = val;
    }
    if let Some(value) = params.get("PlatformName") {
        input.platform_name = value.to_string();
    }
    if let Some(value) = params.get("PlatformVersion") {
        input.platform_version = value.to_string();
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
    Ok(input)
}

/// Deserialize awsQuery request for DeleteApplication.
pub fn deserialize_delete_application_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteApplicationMessage, String> {
    let mut input = DeleteApplicationMessage::default();
    if let Some(value) = params.get("ApplicationName") {
        input.application_name = value.to_string();
    }
    if let Some(value) = params.get("TerminateEnvByForce") {
        input.terminate_env_by_force = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse TerminateEnvByForce: {e}"))?,
        );
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteApplicationVersion.
pub fn deserialize_delete_application_version_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteApplicationVersionMessage, String> {
    let mut input = DeleteApplicationVersionMessage::default();
    if let Some(value) = params.get("ApplicationName") {
        input.application_name = value.to_string();
    }
    if let Some(value) = params.get("DeleteSourceBundle") {
        input.delete_source_bundle = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse DeleteSourceBundle: {e}"))?,
        );
    }
    if let Some(value) = params.get("VersionLabel") {
        input.version_label = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteConfigurationTemplate.
pub fn deserialize_delete_configuration_template_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteConfigurationTemplateMessage, String> {
    let mut input = DeleteConfigurationTemplateMessage::default();
    if let Some(value) = params.get("ApplicationName") {
        input.application_name = value.to_string();
    }
    if let Some(value) = params.get("TemplateName") {
        input.template_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteEnvironmentConfiguration.
pub fn deserialize_delete_environment_configuration_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteEnvironmentConfigurationMessage, String> {
    let mut input = DeleteEnvironmentConfigurationMessage::default();
    if let Some(value) = params.get("ApplicationName") {
        input.application_name = value.to_string();
    }
    if let Some(value) = params.get("EnvironmentName") {
        input.environment_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeletePlatformVersion.
pub fn deserialize_delete_platform_version_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeletePlatformVersionRequest, String> {
    let mut input = DeletePlatformVersionRequest::default();
    if let Some(value) = params.get("PlatformArn") {
        input.platform_arn = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeApplicationVersions.
pub fn deserialize_describe_application_versions_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeApplicationVersionsMessage, String> {
    let mut input = DescribeApplicationVersionsMessage::default();
    if let Some(value) = params.get("ApplicationName") {
        input.application_name = Some(value.to_string());
    }
    if let Some(value) = params.get("MaxRecords") {
        input.max_records = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxRecords: {e}"))?,
        );
    }
    if let Some(value) = params.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "VersionLabels".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.version_labels = Some(VersionLabelsList { items });
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeApplications.
pub fn deserialize_describe_applications_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeApplicationsMessage, String> {
    let mut input = DescribeApplicationsMessage::default();
    {
        let mut items = Vec::new();
        let list_prefix = "ApplicationNames".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.application_names = Some(ApplicationNamesList { items });
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeConfigurationOptions.
pub fn deserialize_describe_configuration_options_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeConfigurationOptionsMessage, String> {
    let mut input = DescribeConfigurationOptionsMessage::default();
    if let Some(value) = params.get("ApplicationName") {
        input.application_name = Some(value.to_string());
    }
    if let Some(value) = params.get("EnvironmentName") {
        input.environment_name = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Options".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_option_specification_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.options = Some(OptionsSpecifierList { items });
        }
    }
    if let Some(value) = params.get("PlatformArn") {
        input.platform_arn = Some(value.to_string());
    }
    if let Some(value) = params.get("SolutionStackName") {
        input.solution_stack_name = Some(value.to_string());
    }
    if let Some(value) = params.get("TemplateName") {
        input.template_name = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeConfigurationSettings.
pub fn deserialize_describe_configuration_settings_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeConfigurationSettingsMessage, String> {
    let mut input = DescribeConfigurationSettingsMessage::default();
    if let Some(value) = params.get("ApplicationName") {
        input.application_name = value.to_string();
    }
    if let Some(value) = params.get("EnvironmentName") {
        input.environment_name = Some(value.to_string());
    }
    if let Some(value) = params.get("TemplateName") {
        input.template_name = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeEnvironmentHealth.
pub fn deserialize_describe_environment_health_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeEnvironmentHealthRequest, String> {
    let mut input = DescribeEnvironmentHealthRequest::default();
    {
        let mut items = Vec::new();
        let list_prefix = "AttributeNames".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.attribute_names = Some(EnvironmentHealthAttributes { items });
        }
    }
    if let Some(value) = params.get("EnvironmentId") {
        input.environment_id = Some(value.to_string());
    }
    if let Some(value) = params.get("EnvironmentName") {
        input.environment_name = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeEnvironmentManagedActionHistory.
pub fn deserialize_describe_environment_managed_action_history_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeEnvironmentManagedActionHistoryRequest, String> {
    let mut input = DescribeEnvironmentManagedActionHistoryRequest::default();
    if let Some(value) = params.get("EnvironmentId") {
        input.environment_id = Some(value.to_string());
    }
    if let Some(value) = params.get("EnvironmentName") {
        input.environment_name = Some(value.to_string());
    }
    if let Some(value) = params.get("MaxItems") {
        input.max_items = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxItems: {e}"))?,
        );
    }
    if let Some(value) = params.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeEnvironmentManagedActions.
pub fn deserialize_describe_environment_managed_actions_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeEnvironmentManagedActionsRequest, String> {
    let mut input = DescribeEnvironmentManagedActionsRequest::default();
    if let Some(value) = params.get("EnvironmentId") {
        input.environment_id = Some(value.to_string());
    }
    if let Some(value) = params.get("EnvironmentName") {
        input.environment_name = Some(value.to_string());
    }
    if let Some(value) = params.get("Status") {
        input.status = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeEnvironmentResources.
pub fn deserialize_describe_environment_resources_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeEnvironmentResourcesMessage, String> {
    let mut input = DescribeEnvironmentResourcesMessage::default();
    if let Some(value) = params.get("EnvironmentId") {
        input.environment_id = Some(value.to_string());
    }
    if let Some(value) = params.get("EnvironmentName") {
        input.environment_name = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeEnvironments.
pub fn deserialize_describe_environments_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeEnvironmentsMessage, String> {
    let mut input = DescribeEnvironmentsMessage::default();
    if let Some(value) = params.get("ApplicationName") {
        input.application_name = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "EnvironmentIds".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.environment_ids = Some(EnvironmentIdList { items });
        }
    }
    {
        let mut items = Vec::new();
        let list_prefix = "EnvironmentNames".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.environment_names = Some(EnvironmentNamesList { items });
        }
    }
    if let Some(value) = params.get("IncludeDeleted") {
        input.include_deleted = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse IncludeDeleted: {e}"))?,
        );
    }
    if let Some(value) = params.get("IncludedDeletedBackTo") {
        input.included_deleted_back_to = Some(value.to_string());
    }
    if let Some(value) = params.get("MaxRecords") {
        input.max_records = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxRecords: {e}"))?,
        );
    }
    if let Some(value) = params.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = params.get("VersionLabel") {
        input.version_label = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeEvents.
pub fn deserialize_describe_events_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeEventsMessage, String> {
    let mut input = DescribeEventsMessage::default();
    if let Some(value) = params.get("ApplicationName") {
        input.application_name = Some(value.to_string());
    }
    if let Some(value) = params.get("EndTime") {
        input.end_time = Some(value.to_string());
    }
    if let Some(value) = params.get("EnvironmentId") {
        input.environment_id = Some(value.to_string());
    }
    if let Some(value) = params.get("EnvironmentName") {
        input.environment_name = Some(value.to_string());
    }
    if let Some(value) = params.get("MaxRecords") {
        input.max_records = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxRecords: {e}"))?,
        );
    }
    if let Some(value) = params.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = params.get("PlatformArn") {
        input.platform_arn = Some(value.to_string());
    }
    if let Some(value) = params.get("RequestId") {
        input.request_id = Some(value.to_string());
    }
    if let Some(value) = params.get("Severity") {
        input.severity = Some(value.to_string());
    }
    if let Some(value) = params.get("StartTime") {
        input.start_time = Some(value.to_string());
    }
    if let Some(value) = params.get("TemplateName") {
        input.template_name = Some(value.to_string());
    }
    if let Some(value) = params.get("VersionLabel") {
        input.version_label = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeInstancesHealth.
pub fn deserialize_describe_instances_health_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeInstancesHealthRequest, String> {
    let mut input = DescribeInstancesHealthRequest::default();
    {
        let mut items = Vec::new();
        let list_prefix = "AttributeNames".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.attribute_names = Some(InstancesHealthAttributes { items });
        }
    }
    if let Some(value) = params.get("EnvironmentId") {
        input.environment_id = Some(value.to_string());
    }
    if let Some(value) = params.get("EnvironmentName") {
        input.environment_name = Some(value.to_string());
    }
    if let Some(value) = params.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribePlatformVersion.
pub fn deserialize_describe_platform_version_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribePlatformVersionRequest, String> {
    let mut input = DescribePlatformVersionRequest::default();
    if let Some(value) = params.get("PlatformArn") {
        input.platform_arn = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for DisassociateEnvironmentOperationsRole.
pub fn deserialize_disassociate_environment_operations_role_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DisassociateEnvironmentOperationsRoleMessage, String> {
    let mut input = DisassociateEnvironmentOperationsRoleMessage::default();
    if let Some(value) = params.get("EnvironmentName") {
        input.environment_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for ListPlatformBranches.
pub fn deserialize_list_platform_branches_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ListPlatformBranchesRequest, String> {
    let mut input = ListPlatformBranchesRequest::default();
    {
        let mut items = Vec::new();
        let list_prefix = "Filters".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_search_filter_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.filters = Some(SearchFilters { items });
        }
    }
    if let Some(value) = params.get("MaxRecords") {
        input.max_records = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxRecords: {e}"))?,
        );
    }
    if let Some(value) = params.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for ListPlatformVersions.
pub fn deserialize_list_platform_versions_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ListPlatformVersionsRequest, String> {
    let mut input = ListPlatformVersionsRequest::default();
    {
        let mut items = Vec::new();
        let list_prefix = "Filters".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_platform_filter_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.filters = Some(PlatformFilters { items });
        }
    }
    if let Some(value) = params.get("MaxRecords") {
        input.max_records = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxRecords: {e}"))?,
        );
    }
    if let Some(value) = params.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for ListTagsForResource.
pub fn deserialize_list_tags_for_resource_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ListTagsForResourceMessage, String> {
    let mut input = ListTagsForResourceMessage::default();
    if let Some(value) = params.get("ResourceArn") {
        input.resource_arn = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for RebuildEnvironment.
pub fn deserialize_rebuild_environment_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<RebuildEnvironmentMessage, String> {
    let mut input = RebuildEnvironmentMessage::default();
    if let Some(value) = params.get("EnvironmentId") {
        input.environment_id = Some(value.to_string());
    }
    if let Some(value) = params.get("EnvironmentName") {
        input.environment_name = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for RequestEnvironmentInfo.
pub fn deserialize_request_environment_info_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<RequestEnvironmentInfoMessage, String> {
    let mut input = RequestEnvironmentInfoMessage::default();
    if let Some(value) = params.get("EnvironmentId") {
        input.environment_id = Some(value.to_string());
    }
    if let Some(value) = params.get("EnvironmentName") {
        input.environment_name = Some(value.to_string());
    }
    if let Some(value) = params.get("InfoType") {
        input.info_type = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for RestartAppServer.
pub fn deserialize_restart_app_server_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<RestartAppServerMessage, String> {
    let mut input = RestartAppServerMessage::default();
    if let Some(value) = params.get("EnvironmentId") {
        input.environment_id = Some(value.to_string());
    }
    if let Some(value) = params.get("EnvironmentName") {
        input.environment_name = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for RetrieveEnvironmentInfo.
pub fn deserialize_retrieve_environment_info_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<RetrieveEnvironmentInfoMessage, String> {
    let mut input = RetrieveEnvironmentInfoMessage::default();
    if let Some(value) = params.get("EnvironmentId") {
        input.environment_id = Some(value.to_string());
    }
    if let Some(value) = params.get("EnvironmentName") {
        input.environment_name = Some(value.to_string());
    }
    if let Some(value) = params.get("InfoType") {
        input.info_type = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for SwapEnvironmentCNAMEs.
pub fn deserialize_swap_environment_c_n_a_m_es_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<SwapEnvironmentCNAMEsMessage, String> {
    let mut input = SwapEnvironmentCNAMEsMessage::default();
    if let Some(value) = params.get("DestinationEnvironmentId") {
        input.destination_environment_id = Some(value.to_string());
    }
    if let Some(value) = params.get("DestinationEnvironmentName") {
        input.destination_environment_name = Some(value.to_string());
    }
    if let Some(value) = params.get("SourceEnvironmentId") {
        input.source_environment_id = Some(value.to_string());
    }
    if let Some(value) = params.get("SourceEnvironmentName") {
        input.source_environment_name = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for TerminateEnvironment.
pub fn deserialize_terminate_environment_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<TerminateEnvironmentMessage, String> {
    let mut input = TerminateEnvironmentMessage::default();
    if let Some(value) = params.get("EnvironmentId") {
        input.environment_id = Some(value.to_string());
    }
    if let Some(value) = params.get("EnvironmentName") {
        input.environment_name = Some(value.to_string());
    }
    if let Some(value) = params.get("ForceTerminate") {
        input.force_terminate = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse ForceTerminate: {e}"))?,
        );
    }
    if let Some(value) = params.get("TerminateResources") {
        input.terminate_resources = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse TerminateResources: {e}"))?,
        );
    }
    Ok(input)
}

/// Deserialize awsQuery request for UpdateApplication.
pub fn deserialize_update_application_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<UpdateApplicationMessage, String> {
    let mut input = UpdateApplicationMessage::default();
    if let Some(value) = params.get("ApplicationName") {
        input.application_name = value.to_string();
    }
    if let Some(value) = params.get("Description") {
        input.description = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for UpdateApplicationResourceLifecycle.
pub fn deserialize_update_application_resource_lifecycle_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<UpdateApplicationResourceLifecycleMessage, String> {
    let mut input = UpdateApplicationResourceLifecycleMessage::default();
    if let Some(value) = params.get("ApplicationName") {
        input.application_name = value.to_string();
    }
    if let Some(val) = deserialize_application_resource_lifecycle_config_from_query(
        params,
        "ResourceLifecycleConfig",
    )? {
        input.resource_lifecycle_config = val;
    }
    Ok(input)
}

/// Deserialize awsQuery request for UpdateApplicationVersion.
pub fn deserialize_update_application_version_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<UpdateApplicationVersionMessage, String> {
    let mut input = UpdateApplicationVersionMessage::default();
    if let Some(value) = params.get("ApplicationName") {
        input.application_name = value.to_string();
    }
    if let Some(value) = params.get("Description") {
        input.description = Some(value.to_string());
    }
    if let Some(value) = params.get("VersionLabel") {
        input.version_label = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for UpdateConfigurationTemplate.
pub fn deserialize_update_configuration_template_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<UpdateConfigurationTemplateMessage, String> {
    let mut input = UpdateConfigurationTemplateMessage::default();
    if let Some(value) = params.get("ApplicationName") {
        input.application_name = value.to_string();
    }
    if let Some(value) = params.get("Description") {
        input.description = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "OptionSettings".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_configuration_option_setting_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.option_settings = Some(ConfigurationOptionSettingsList { items });
        }
    }
    {
        let mut items = Vec::new();
        let list_prefix = "OptionsToRemove".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_option_specification_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.options_to_remove = Some(OptionsSpecifierList { items });
        }
    }
    if let Some(value) = params.get("TemplateName") {
        input.template_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for UpdateEnvironment.
pub fn deserialize_update_environment_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<UpdateEnvironmentMessage, String> {
    let mut input = UpdateEnvironmentMessage::default();
    if let Some(value) = params.get("ApplicationName") {
        input.application_name = Some(value.to_string());
    }
    if let Some(value) = params.get("Description") {
        input.description = Some(value.to_string());
    }
    if let Some(value) = params.get("EnvironmentId") {
        input.environment_id = Some(value.to_string());
    }
    if let Some(value) = params.get("EnvironmentName") {
        input.environment_name = Some(value.to_string());
    }
    if let Some(value) = params.get("GroupName") {
        input.group_name = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "OptionSettings".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_configuration_option_setting_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.option_settings = Some(ConfigurationOptionSettingsList { items });
        }
    }
    {
        let mut items = Vec::new();
        let list_prefix = "OptionsToRemove".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_option_specification_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.options_to_remove = Some(OptionsSpecifierList { items });
        }
    }
    if let Some(value) = params.get("PlatformArn") {
        input.platform_arn = Some(value.to_string());
    }
    if let Some(value) = params.get("SolutionStackName") {
        input.solution_stack_name = Some(value.to_string());
    }
    if let Some(value) = params.get("TemplateName") {
        input.template_name = Some(value.to_string());
    }
    if let Some(val) = deserialize_environment_tier_from_query(params, "Tier")? {
        input.tier = Some(val);
    }
    if let Some(value) = params.get("VersionLabel") {
        input.version_label = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for UpdateTagsForResource.
pub fn deserialize_update_tags_for_resource_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<UpdateTagsForResourceMessage, String> {
    let mut input = UpdateTagsForResourceMessage::default();
    if let Some(value) = params.get("ResourceArn") {
        input.resource_arn = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "TagsToAdd".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_tag_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tags_to_add = Some(TagList { items });
        }
    }
    {
        let mut items = Vec::new();
        let list_prefix = "TagsToRemove".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tags_to_remove = Some(TagKeyList { items });
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for ValidateConfigurationSettings.
pub fn deserialize_validate_configuration_settings_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ValidateConfigurationSettingsMessage, String> {
    let mut input = ValidateConfigurationSettingsMessage::default();
    if let Some(value) = params.get("ApplicationName") {
        input.application_name = value.to_string();
    }
    if let Some(value) = params.get("EnvironmentName") {
        input.environment_name = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "OptionSettings".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_configuration_option_setting_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.option_settings = ConfigurationOptionSettingsList { items };
        }
    }
    if let Some(value) = params.get("TemplateName") {
        input.template_name = Some(value.to_string());
    }
    Ok(input)
}
