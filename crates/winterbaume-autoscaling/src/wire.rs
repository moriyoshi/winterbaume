//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-autoscaling

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
pub fn serialize_attach_instances_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<AttachInstancesResponse xmlns="http://autoscaling.amazonaws.com/doc/2011-01-01/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</AttachInstancesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_attach_load_balancer_target_groups_response(
    result: &AttachLoadBalancerTargetGroupsResultType,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<AttachLoadBalancerTargetGroupsResult>{inner_xml}</AttachLoadBalancerTargetGroupsResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<AttachLoadBalancerTargetGroupsResponse xmlns="http://autoscaling.amazonaws.com/doc/2011-01-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</AttachLoadBalancerTargetGroupsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_attach_load_balancers_response(
    result: &AttachLoadBalancersResultType,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<AttachLoadBalancersResult>{inner_xml}</AttachLoadBalancersResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<AttachLoadBalancersResponse xmlns="http://autoscaling.amazonaws.com/doc/2011-01-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</AttachLoadBalancersResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_attach_traffic_sources_response(
    result: &AttachTrafficSourcesResultType,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<AttachTrafficSourcesResult>{inner_xml}</AttachTrafficSourcesResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<AttachTrafficSourcesResponse xmlns="http://autoscaling.amazonaws.com/doc/2011-01-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</AttachTrafficSourcesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_batch_delete_scheduled_action_response(
    result: &BatchDeleteScheduledActionAnswer,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<BatchDeleteScheduledActionResult>{inner_xml}</BatchDeleteScheduledActionResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<BatchDeleteScheduledActionResponse xmlns="http://autoscaling.amazonaws.com/doc/2011-01-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</BatchDeleteScheduledActionResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_batch_put_scheduled_update_group_action_response(
    result: &BatchPutScheduledUpdateGroupActionAnswer,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<BatchPutScheduledUpdateGroupActionResult>{inner_xml}</BatchPutScheduledUpdateGroupActionResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<BatchPutScheduledUpdateGroupActionResponse xmlns="http://autoscaling.amazonaws.com/doc/2011-01-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</BatchPutScheduledUpdateGroupActionResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_cancel_instance_refresh_response(
    result: &CancelInstanceRefreshAnswer,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<CancelInstanceRefreshResult>{inner_xml}</CancelInstanceRefreshResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CancelInstanceRefreshResponse xmlns="http://autoscaling.amazonaws.com/doc/2011-01-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CancelInstanceRefreshResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_complete_lifecycle_action_response(
    result: &CompleteLifecycleActionAnswer,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<CompleteLifecycleActionResult>{inner_xml}</CompleteLifecycleActionResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CompleteLifecycleActionResponse xmlns="http://autoscaling.amazonaws.com/doc/2011-01-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CompleteLifecycleActionResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_create_auto_scaling_group_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateAutoScalingGroupResponse xmlns="http://autoscaling.amazonaws.com/doc/2011-01-01/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateAutoScalingGroupResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_create_launch_configuration_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateLaunchConfigurationResponse xmlns="http://autoscaling.amazonaws.com/doc/2011-01-01/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateLaunchConfigurationResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_create_or_update_tags_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<CreateOrUpdateTagsResponse xmlns="http://autoscaling.amazonaws.com/doc/2011-01-01/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</CreateOrUpdateTagsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_delete_auto_scaling_group_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteAutoScalingGroupResponse xmlns="http://autoscaling.amazonaws.com/doc/2011-01-01/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteAutoScalingGroupResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_delete_launch_configuration_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteLaunchConfigurationResponse xmlns="http://autoscaling.amazonaws.com/doc/2011-01-01/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteLaunchConfigurationResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_delete_lifecycle_hook_response(
    result: &DeleteLifecycleHookAnswer,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<DeleteLifecycleHookResult>{inner_xml}</DeleteLifecycleHookResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteLifecycleHookResponse xmlns="http://autoscaling.amazonaws.com/doc/2011-01-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteLifecycleHookResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_delete_notification_configuration_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteNotificationConfigurationResponse xmlns="http://autoscaling.amazonaws.com/doc/2011-01-01/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteNotificationConfigurationResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_delete_policy_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeletePolicyResponse xmlns="http://autoscaling.amazonaws.com/doc/2011-01-01/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeletePolicyResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_delete_scheduled_action_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteScheduledActionResponse xmlns="http://autoscaling.amazonaws.com/doc/2011-01-01/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteScheduledActionResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_delete_tags_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteTagsResponse xmlns="http://autoscaling.amazonaws.com/doc/2011-01-01/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteTagsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_delete_warm_pool_response(result: &DeleteWarmPoolAnswer) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<DeleteWarmPoolResult>{inner_xml}</DeleteWarmPoolResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteWarmPoolResponse xmlns="http://autoscaling.amazonaws.com/doc/2011-01-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteWarmPoolResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_account_limits_response(
    result: &DescribeAccountLimitsAnswer,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DescribeAccountLimitsResult>{inner_xml}</DescribeAccountLimitsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeAccountLimitsResponse xmlns="http://autoscaling.amazonaws.com/doc/2011-01-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeAccountLimitsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_adjustment_types_response(
    result: &DescribeAdjustmentTypesAnswer,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DescribeAdjustmentTypesResult>{inner_xml}</DescribeAdjustmentTypesResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeAdjustmentTypesResponse xmlns="http://autoscaling.amazonaws.com/doc/2011-01-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeAdjustmentTypesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_auto_scaling_groups_response(
    result: &AutoScalingGroupsType,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DescribeAutoScalingGroupsResult>{inner_xml}</DescribeAutoScalingGroupsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeAutoScalingGroupsResponse xmlns="http://autoscaling.amazonaws.com/doc/2011-01-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeAutoScalingGroupsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_auto_scaling_instances_response(
    result: &AutoScalingInstancesType,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<DescribeAutoScalingInstancesResult>{inner_xml}</DescribeAutoScalingInstancesResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeAutoScalingInstancesResponse xmlns="http://autoscaling.amazonaws.com/doc/2011-01-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeAutoScalingInstancesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_auto_scaling_notification_types_response(
    result: &DescribeAutoScalingNotificationTypesAnswer,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<DescribeAutoScalingNotificationTypesResult>{inner_xml}</DescribeAutoScalingNotificationTypesResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeAutoScalingNotificationTypesResponse xmlns="http://autoscaling.amazonaws.com/doc/2011-01-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeAutoScalingNotificationTypesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_instance_refreshes_response(
    result: &DescribeInstanceRefreshesAnswer,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DescribeInstanceRefreshesResult>{inner_xml}</DescribeInstanceRefreshesResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeInstanceRefreshesResponse xmlns="http://autoscaling.amazonaws.com/doc/2011-01-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeInstanceRefreshesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_launch_configurations_response(
    result: &LaunchConfigurationsType,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<DescribeLaunchConfigurationsResult>{inner_xml}</DescribeLaunchConfigurationsResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeLaunchConfigurationsResponse xmlns="http://autoscaling.amazonaws.com/doc/2011-01-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeLaunchConfigurationsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_lifecycle_hook_types_response(
    result: &DescribeLifecycleHookTypesAnswer,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DescribeLifecycleHookTypesResult>{inner_xml}</DescribeLifecycleHookTypesResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeLifecycleHookTypesResponse xmlns="http://autoscaling.amazonaws.com/doc/2011-01-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeLifecycleHookTypesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_lifecycle_hooks_response(
    result: &DescribeLifecycleHooksAnswer,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DescribeLifecycleHooksResult>{inner_xml}</DescribeLifecycleHooksResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeLifecycleHooksResponse xmlns="http://autoscaling.amazonaws.com/doc/2011-01-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeLifecycleHooksResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_load_balancer_target_groups_response(
    result: &DescribeLoadBalancerTargetGroupsResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<DescribeLoadBalancerTargetGroupsResult>{inner_xml}</DescribeLoadBalancerTargetGroupsResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeLoadBalancerTargetGroupsResponse xmlns="http://autoscaling.amazonaws.com/doc/2011-01-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeLoadBalancerTargetGroupsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_load_balancers_response(
    result: &DescribeLoadBalancersResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DescribeLoadBalancersResult>{inner_xml}</DescribeLoadBalancersResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeLoadBalancersResponse xmlns="http://autoscaling.amazonaws.com/doc/2011-01-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeLoadBalancersResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_metric_collection_types_response(
    result: &DescribeMetricCollectionTypesAnswer,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<DescribeMetricCollectionTypesResult>{inner_xml}</DescribeMetricCollectionTypesResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeMetricCollectionTypesResponse xmlns="http://autoscaling.amazonaws.com/doc/2011-01-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeMetricCollectionTypesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_notification_configurations_response(
    result: &DescribeNotificationConfigurationsAnswer,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<DescribeNotificationConfigurationsResult>{inner_xml}</DescribeNotificationConfigurationsResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeNotificationConfigurationsResponse xmlns="http://autoscaling.amazonaws.com/doc/2011-01-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeNotificationConfigurationsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_policies_response(result: &PoliciesType) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<DescribePoliciesResult>{inner_xml}</DescribePoliciesResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribePoliciesResponse xmlns="http://autoscaling.amazonaws.com/doc/2011-01-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribePoliciesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_scaling_activities_response(result: &ActivitiesType) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DescribeScalingActivitiesResult>{inner_xml}</DescribeScalingActivitiesResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeScalingActivitiesResponse xmlns="http://autoscaling.amazonaws.com/doc/2011-01-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeScalingActivitiesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_scaling_process_types_response(result: &ProcessesType) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<DescribeScalingProcessTypesResult>{inner_xml}</DescribeScalingProcessTypesResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeScalingProcessTypesResponse xmlns="http://autoscaling.amazonaws.com/doc/2011-01-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeScalingProcessTypesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_scheduled_actions_response(
    result: &ScheduledActionsType,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DescribeScheduledActionsResult>{inner_xml}</DescribeScheduledActionsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeScheduledActionsResponse xmlns="http://autoscaling.amazonaws.com/doc/2011-01-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeScheduledActionsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_tags_response(result: &TagsType) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<DescribeTagsResult>{inner_xml}</DescribeTagsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeTagsResponse xmlns="http://autoscaling.amazonaws.com/doc/2011-01-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeTagsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_termination_policy_types_response(
    result: &DescribeTerminationPolicyTypesAnswer,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<DescribeTerminationPolicyTypesResult>{inner_xml}</DescribeTerminationPolicyTypesResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeTerminationPolicyTypesResponse xmlns="http://autoscaling.amazonaws.com/doc/2011-01-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeTerminationPolicyTypesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_traffic_sources_response(
    result: &DescribeTrafficSourcesResponse,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DescribeTrafficSourcesResult>{inner_xml}</DescribeTrafficSourcesResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeTrafficSourcesResponse xmlns="http://autoscaling.amazonaws.com/doc/2011-01-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeTrafficSourcesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_warm_pool_response(result: &DescribeWarmPoolAnswer) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<DescribeWarmPoolResult>{inner_xml}</DescribeWarmPoolResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeWarmPoolResponse xmlns="http://autoscaling.amazonaws.com/doc/2011-01-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeWarmPoolResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_detach_instances_response(result: &DetachInstancesAnswer) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<DetachInstancesResult>{inner_xml}</DetachInstancesResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DetachInstancesResponse xmlns="http://autoscaling.amazonaws.com/doc/2011-01-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DetachInstancesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_detach_load_balancer_target_groups_response(
    result: &DetachLoadBalancerTargetGroupsResultType,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<DetachLoadBalancerTargetGroupsResult>{inner_xml}</DetachLoadBalancerTargetGroupsResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DetachLoadBalancerTargetGroupsResponse xmlns="http://autoscaling.amazonaws.com/doc/2011-01-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DetachLoadBalancerTargetGroupsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_detach_load_balancers_response(
    result: &DetachLoadBalancersResultType,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<DetachLoadBalancersResult>{inner_xml}</DetachLoadBalancersResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DetachLoadBalancersResponse xmlns="http://autoscaling.amazonaws.com/doc/2011-01-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DetachLoadBalancersResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_detach_traffic_sources_response(
    result: &DetachTrafficSourcesResultType,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DetachTrafficSourcesResult>{inner_xml}</DetachTrafficSourcesResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DetachTrafficSourcesResponse xmlns="http://autoscaling.amazonaws.com/doc/2011-01-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DetachTrafficSourcesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_disable_metrics_collection_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DisableMetricsCollectionResponse xmlns="http://autoscaling.amazonaws.com/doc/2011-01-01/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DisableMetricsCollectionResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_enable_metrics_collection_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<EnableMetricsCollectionResponse xmlns="http://autoscaling.amazonaws.com/doc/2011-01-01/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</EnableMetricsCollectionResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_enter_standby_response(result: &EnterStandbyAnswer) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<EnterStandbyResult>{inner_xml}</EnterStandbyResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<EnterStandbyResponse xmlns="http://autoscaling.amazonaws.com/doc/2011-01-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</EnterStandbyResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_execute_policy_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ExecutePolicyResponse xmlns="http://autoscaling.amazonaws.com/doc/2011-01-01/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ExecutePolicyResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_exit_standby_response(result: &ExitStandbyAnswer) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<ExitStandbyResult>{inner_xml}</ExitStandbyResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ExitStandbyResponse xmlns="http://autoscaling.amazonaws.com/doc/2011-01-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ExitStandbyResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_get_predictive_scaling_forecast_response(
    result: &GetPredictiveScalingForecastAnswer,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<GetPredictiveScalingForecastResult>{inner_xml}</GetPredictiveScalingForecastResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<GetPredictiveScalingForecastResponse xmlns="http://autoscaling.amazonaws.com/doc/2011-01-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</GetPredictiveScalingForecastResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_launch_instances_response(result: &LaunchInstancesResult) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<LaunchInstancesResult>{inner_xml}</LaunchInstancesResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<LaunchInstancesResponse xmlns="http://autoscaling.amazonaws.com/doc/2011-01-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</LaunchInstancesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_put_lifecycle_hook_response(result: &PutLifecycleHookAnswer) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<PutLifecycleHookResult>{inner_xml}</PutLifecycleHookResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<PutLifecycleHookResponse xmlns="http://autoscaling.amazonaws.com/doc/2011-01-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</PutLifecycleHookResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_put_notification_configuration_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<PutNotificationConfigurationResponse xmlns="http://autoscaling.amazonaws.com/doc/2011-01-01/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</PutNotificationConfigurationResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_put_scaling_policy_response(result: &PolicyARNType) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<PutScalingPolicyResult>{inner_xml}</PutScalingPolicyResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<PutScalingPolicyResponse xmlns="http://autoscaling.amazonaws.com/doc/2011-01-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</PutScalingPolicyResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_put_scheduled_update_group_action_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<PutScheduledUpdateGroupActionResponse xmlns="http://autoscaling.amazonaws.com/doc/2011-01-01/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</PutScheduledUpdateGroupActionResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_put_warm_pool_response(result: &PutWarmPoolAnswer) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<PutWarmPoolResult>{inner_xml}</PutWarmPoolResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<PutWarmPoolResponse xmlns="http://autoscaling.amazonaws.com/doc/2011-01-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</PutWarmPoolResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_record_lifecycle_action_heartbeat_response(
    result: &RecordLifecycleActionHeartbeatAnswer,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<RecordLifecycleActionHeartbeatResult>{inner_xml}</RecordLifecycleActionHeartbeatResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<RecordLifecycleActionHeartbeatResponse xmlns="http://autoscaling.amazonaws.com/doc/2011-01-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</RecordLifecycleActionHeartbeatResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_resume_processes_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ResumeProcessesResponse xmlns="http://autoscaling.amazonaws.com/doc/2011-01-01/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ResumeProcessesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_rollback_instance_refresh_response(
    result: &RollbackInstanceRefreshAnswer,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<RollbackInstanceRefreshResult>{inner_xml}</RollbackInstanceRefreshResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<RollbackInstanceRefreshResponse xmlns="http://autoscaling.amazonaws.com/doc/2011-01-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</RollbackInstanceRefreshResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_set_desired_capacity_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<SetDesiredCapacityResponse xmlns="http://autoscaling.amazonaws.com/doc/2011-01-01/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</SetDesiredCapacityResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_set_instance_health_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<SetInstanceHealthResponse xmlns="http://autoscaling.amazonaws.com/doc/2011-01-01/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</SetInstanceHealthResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_set_instance_protection_response(
    result: &SetInstanceProtectionAnswer,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<SetInstanceProtectionResult>{inner_xml}</SetInstanceProtectionResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<SetInstanceProtectionResponse xmlns="http://autoscaling.amazonaws.com/doc/2011-01-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</SetInstanceProtectionResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_start_instance_refresh_response(
    result: &StartInstanceRefreshAnswer,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<StartInstanceRefreshResult>{inner_xml}</StartInstanceRefreshResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<StartInstanceRefreshResponse xmlns="http://autoscaling.amazonaws.com/doc/2011-01-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</StartInstanceRefreshResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_suspend_processes_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<SuspendProcessesResponse xmlns="http://autoscaling.amazonaws.com/doc/2011-01-01/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</SuspendProcessesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_terminate_instance_in_auto_scaling_group_response(
    result: &ActivityType,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!(
        "<TerminateInstanceInAutoScalingGroupResult>{inner_xml}</TerminateInstanceInAutoScalingGroupResult>"
    );
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<TerminateInstanceInAutoScalingGroupResponse xmlns="http://autoscaling.amazonaws.com/doc/2011-01-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</TerminateInstanceInAutoScalingGroupResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_update_auto_scaling_group_response() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<UpdateAutoScalingGroupResponse xmlns="http://autoscaling.amazonaws.com/doc/2011-01-01/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</UpdateAutoScalingGroupResponse>"#
    );
    MockResponse::xml(200, xml)
}

fn deserialize_memory_gi_b_per_v_cpu_request_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<MemoryGiBPerVCpuRequest>, String> {
    let mut item = MemoryGiBPerVCpuRequest::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.Max")) {
        item.max = Some(
            value
                .parse::<f64>()
                .map_err(|e| format!("failed to parse Max: {e}"))?,
        );
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Min")) {
        item.min = Some(
            value
                .parse::<f64>()
                .map_err(|e| format!("failed to parse Min: {e}"))?,
        );
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_refresh_preferences_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<RefreshPreferences>, String> {
    let mut item = RefreshPreferences::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.AutoRollback")) {
        item.auto_rollback = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse AutoRollback: {e}"))?,
        );
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.BakeTime")) {
        item.bake_time = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse BakeTime: {e}"))?,
        );
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.CheckpointDelay")) {
        item.checkpoint_delay = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse CheckpointDelay: {e}"))?,
        );
        found = true;
    }
    {
        let mut sub_items = Vec::new();
        let sub_list_prefix = format!("{prefix}.CheckpointPercentages");
        for i in 1.. {
            let item_key = format!("{sub_list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => sub_items.push(
                    value
                        .parse::<i32>()
                        .map_err(|e| format!("failed to parse list item: {e}"))?,
                ),
                None => break,
            }
        }
        if !sub_items.is_empty() {
            item.checkpoint_percentages = Some(CheckpointPercentages { items: sub_items });
            found = true;
        }
    }
    if let Some(value) = params.get(&format!("{prefix}.InstanceWarmup")) {
        item.instance_warmup = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse InstanceWarmup: {e}"))?,
        );
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.MaxHealthyPercentage")) {
        item.max_healthy_percentage = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxHealthyPercentage: {e}"))?,
        );
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.MinHealthyPercentage")) {
        item.min_healthy_percentage = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MinHealthyPercentage: {e}"))?,
        );
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.ScaleInProtectedInstances")) {
        item.scale_in_protected_instances = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.SkipMatching")) {
        item.skip_matching = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse SkipMatching: {e}"))?,
        );
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.StandbyInstances")) {
        item.standby_instances = Some(value.to_string());
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_block_device_mapping_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<BlockDeviceMapping>, String> {
    let mut item = BlockDeviceMapping::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.DeviceName")) {
        item.device_name = value.to_string();
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.NoDevice")) {
        item.no_device = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse NoDevice: {e}"))?,
        );
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.VirtualName")) {
        item.virtual_name = Some(value.to_string());
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_launch_template_overrides_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<LaunchTemplateOverrides>, String> {
    let mut item = LaunchTemplateOverrides::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.ImageId")) {
        item.image_id = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.InstanceType")) {
        item.instance_type = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.WeightedCapacity")) {
        item.weighted_capacity = Some(value.to_string());
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_memory_mi_b_request_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<MemoryMiBRequest>, String> {
    let mut item = MemoryMiBRequest::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.Max")) {
        item.max = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse Max: {e}"))?,
        );
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Min")) {
        item.min = value
            .parse::<i32>()
            .map_err(|e| format!("failed to parse Min: {e}"))?;
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_metric_stat_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<MetricStat>, String> {
    let mut item = MetricStat::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.Stat")) {
        item.stat = value.to_string();
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Unit")) {
        item.unit = Some(value.to_string());
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_traffic_source_identifier_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<TrafficSourceIdentifier>, String> {
    let mut item = TrafficSourceIdentifier::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.Identifier")) {
        item.identifier = value.to_string();
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Type")) {
        item.r#type = Some(value.to_string());
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_predictive_scaling_predefined_load_metric_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<PredictiveScalingPredefinedLoadMetric>, String> {
    let mut item = PredictiveScalingPredefinedLoadMetric::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.PredefinedMetricType")) {
        item.predefined_metric_type = value.to_string();
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.ResourceLabel")) {
        item.resource_label = Some(value.to_string());
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_predictive_scaling_customized_scaling_metric_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<PredictiveScalingCustomizedScalingMetric>, String> {
    let mut item = PredictiveScalingCustomizedScalingMetric::default();
    let mut found = false;
    {
        let mut sub_items = Vec::new();
        let sub_list_prefix = format!("{prefix}.MetricDataQueries");
        for i in 1.. {
            let item_key = format!("{sub_list_prefix}.member.{i}");
            match deserialize_metric_data_query_from_query(params, &item_key)? {
                Some(sub_item) => sub_items.push(sub_item),
                None => break,
            }
        }
        if !sub_items.is_empty() {
            item.metric_data_queries = MetricDataQueries { items: sub_items };
            found = true;
        }
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_baseline_ebs_bandwidth_mbps_request_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<BaselineEbsBandwidthMbpsRequest>, String> {
    let mut item = BaselineEbsBandwidthMbpsRequest::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.Max")) {
        item.max = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse Max: {e}"))?,
        );
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Min")) {
        item.min = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse Min: {e}"))?,
        );
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_predictive_scaling_configuration_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<PredictiveScalingConfiguration>, String> {
    let mut item = PredictiveScalingConfiguration::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.MaxCapacityBreachBehavior")) {
        item.max_capacity_breach_behavior = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.MaxCapacityBuffer")) {
        item.max_capacity_buffer = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxCapacityBuffer: {e}"))?,
        );
        found = true;
    }
    {
        let mut sub_items = Vec::new();
        let sub_list_prefix = format!("{prefix}.MetricSpecifications");
        for i in 1.. {
            let item_key = format!("{sub_list_prefix}.member.{i}");
            match deserialize_predictive_scaling_metric_specification_from_query(params, &item_key)?
            {
                Some(sub_item) => sub_items.push(sub_item),
                None => break,
            }
        }
        if !sub_items.is_empty() {
            item.metric_specifications = PredictiveScalingMetricSpecifications { items: sub_items };
            found = true;
        }
    }
    if let Some(value) = params.get(&format!("{prefix}.Mode")) {
        item.mode = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.SchedulingBufferTime")) {
        item.scheduling_buffer_time = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse SchedulingBufferTime: {e}"))?,
        );
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_instance_lifecycle_policy_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<InstanceLifecyclePolicy>, String> {
    let mut item = InstanceLifecyclePolicy::default();
    let mut found = false;
    Ok(if found { Some(item) } else { None })
}

fn deserialize_predefined_metric_specification_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<PredefinedMetricSpecification>, String> {
    let mut item = PredefinedMetricSpecification::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.PredefinedMetricType")) {
        item.predefined_metric_type = value.to_string();
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.ResourceLabel")) {
        item.resource_label = Some(value.to_string());
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_desired_configuration_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<DesiredConfiguration>, String> {
    let mut item = DesiredConfiguration::default();
    let mut found = false;
    Ok(if found { Some(item) } else { None })
}

fn deserialize_metric_dimension_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<MetricDimension>, String> {
    let mut item = MetricDimension::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.Name")) {
        item.name = value.to_string();
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Value")) {
        item.value = value.to_string();
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_accelerator_total_memory_mi_b_request_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<AcceleratorTotalMemoryMiBRequest>, String> {
    let mut item = AcceleratorTotalMemoryMiBRequest::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.Max")) {
        item.max = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse Max: {e}"))?,
        );
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Min")) {
        item.min = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse Min: {e}"))?,
        );
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_availability_zone_distribution_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<AvailabilityZoneDistribution>, String> {
    let mut item = AvailabilityZoneDistribution::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.CapacityDistributionStrategy")) {
        item.capacity_distribution_strategy = Some(value.to_string());
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_instance_metadata_options_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<InstanceMetadataOptions>, String> {
    let mut item = InstanceMetadataOptions::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.HttpEndpoint")) {
        item.http_endpoint = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.HttpPutResponseHopLimit")) {
        item.http_put_response_hop_limit = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse HttpPutResponseHopLimit: {e}"))?,
        );
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.HttpTokens")) {
        item.http_tokens = Some(value.to_string());
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_ebs_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<Ebs>, String> {
    let mut item = Ebs::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.DeleteOnTermination")) {
        item.delete_on_termination = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse DeleteOnTermination: {e}"))?,
        );
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Encrypted")) {
        item.encrypted = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse Encrypted: {e}"))?,
        );
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Iops")) {
        item.iops = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse Iops: {e}"))?,
        );
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.SnapshotId")) {
        item.snapshot_id = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Throughput")) {
        item.throughput = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse Throughput: {e}"))?,
        );
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.VolumeSize")) {
        item.volume_size = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse VolumeSize: {e}"))?,
        );
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.VolumeType")) {
        item.volume_type = Some(value.to_string());
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_predictive_scaling_customized_load_metric_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<PredictiveScalingCustomizedLoadMetric>, String> {
    let mut item = PredictiveScalingCustomizedLoadMetric::default();
    let mut found = false;
    {
        let mut sub_items = Vec::new();
        let sub_list_prefix = format!("{prefix}.MetricDataQueries");
        for i in 1.. {
            let item_key = format!("{sub_list_prefix}.member.{i}");
            match deserialize_metric_data_query_from_query(params, &item_key)? {
                Some(sub_item) => sub_items.push(sub_item),
                None => break,
            }
        }
        if !sub_items.is_empty() {
            item.metric_data_queries = MetricDataQueries { items: sub_items };
            found = true;
        }
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_scheduled_update_group_action_request_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<ScheduledUpdateGroupActionRequest>, String> {
    let mut item = ScheduledUpdateGroupActionRequest::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.DesiredCapacity")) {
        item.desired_capacity = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse DesiredCapacity: {e}"))?,
        );
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.EndTime")) {
        item.end_time = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.MaxSize")) {
        item.max_size = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxSize: {e}"))?,
        );
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.MinSize")) {
        item.min_size = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MinSize: {e}"))?,
        );
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Recurrence")) {
        item.recurrence = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.ScheduledActionName")) {
        item.scheduled_action_name = value.to_string();
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.StartTime")) {
        item.start_time = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.TimeZone")) {
        item.time_zone = Some(value.to_string());
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_network_bandwidth_gbps_request_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<NetworkBandwidthGbpsRequest>, String> {
    let mut item = NetworkBandwidthGbpsRequest::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.Max")) {
        item.max = Some(
            value
                .parse::<f64>()
                .map_err(|e| format!("failed to parse Max: {e}"))?,
        );
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Min")) {
        item.min = Some(
            value
                .parse::<f64>()
                .map_err(|e| format!("failed to parse Min: {e}"))?,
        );
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_total_local_storage_g_b_request_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<TotalLocalStorageGBRequest>, String> {
    let mut item = TotalLocalStorageGBRequest::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.Max")) {
        item.max = Some(
            value
                .parse::<f64>()
                .map_err(|e| format!("failed to parse Max: {e}"))?,
        );
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Min")) {
        item.min = Some(
            value
                .parse::<f64>()
                .map_err(|e| format!("failed to parse Min: {e}"))?,
        );
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_mixed_instances_policy_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<MixedInstancesPolicy>, String> {
    let mut item = MixedInstancesPolicy::default();
    let mut found = false;
    Ok(if found { Some(item) } else { None })
}

fn deserialize_availability_zone_impairment_policy_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<AvailabilityZoneImpairmentPolicy>, String> {
    let mut item = AvailabilityZoneImpairmentPolicy::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.ImpairedZoneHealthCheckBehavior")) {
        item.impaired_zone_health_check_behavior = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.ZonalShiftEnabled")) {
        item.zonal_shift_enabled = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse ZonalShiftEnabled: {e}"))?,
        );
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_instance_maintenance_policy_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<InstanceMaintenancePolicy>, String> {
    let mut item = InstanceMaintenancePolicy::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.MaxHealthyPercentage")) {
        item.max_healthy_percentage = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxHealthyPercentage: {e}"))?,
        );
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.MinHealthyPercentage")) {
        item.min_healthy_percentage = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MinHealthyPercentage: {e}"))?,
        );
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_lifecycle_hook_specification_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<LifecycleHookSpecification>, String> {
    let mut item = LifecycleHookSpecification::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.DefaultResult")) {
        item.default_result = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.HeartbeatTimeout")) {
        item.heartbeat_timeout = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse HeartbeatTimeout: {e}"))?,
        );
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.LifecycleHookName")) {
        item.lifecycle_hook_name = value.to_string();
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.LifecycleTransition")) {
        item.lifecycle_transition = value.to_string();
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.NotificationMetadata")) {
        item.notification_metadata = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.NotificationTargetARN")) {
        item.notification_target_a_r_n = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.RoleARN")) {
        item.role_a_r_n = Some(value.to_string());
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_instances_distribution_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<InstancesDistribution>, String> {
    let mut item = InstancesDistribution::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.OnDemandAllocationStrategy")) {
        item.on_demand_allocation_strategy = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.OnDemandBaseCapacity")) {
        item.on_demand_base_capacity = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse OnDemandBaseCapacity: {e}"))?,
        );
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.OnDemandPercentageAboveBaseCapacity")) {
        item.on_demand_percentage_above_base_capacity =
            Some(value.parse::<i32>().map_err(|e| {
                format!("failed to parse OnDemandPercentageAboveBaseCapacity: {e}")
            })?);
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.SpotAllocationStrategy")) {
        item.spot_allocation_strategy = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.SpotInstancePools")) {
        item.spot_instance_pools = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse SpotInstancePools: {e}"))?,
        );
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.SpotMaxPrice")) {
        item.spot_max_price = Some(value.to_string());
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_instance_monitoring_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<InstanceMonitoring>, String> {
    let mut item = InstanceMonitoring::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.Enabled")) {
        item.enabled = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse Enabled: {e}"))?,
        );
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_retention_triggers_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<RetentionTriggers>, String> {
    let mut item = RetentionTriggers::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.TerminateHookAbandon")) {
        item.terminate_hook_abandon = Some(value.to_string());
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_capacity_reservation_target_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<CapacityReservationTarget>, String> {
    let mut item = CapacityReservationTarget::default();
    let mut found = false;
    {
        let mut sub_items = Vec::new();
        let sub_list_prefix = format!("{prefix}.CapacityReservationIds");
        for i in 1.. {
            let item_key = format!("{sub_list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => sub_items.push(value.to_string()),
                None => break,
            }
        }
        if !sub_items.is_empty() {
            item.capacity_reservation_ids = Some(CapacityReservationIds { items: sub_items });
            found = true;
        }
    }
    {
        let mut sub_items = Vec::new();
        let sub_list_prefix = format!("{prefix}.CapacityReservationResourceGroupArns");
        for i in 1.. {
            let item_key = format!("{sub_list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => sub_items.push(value.to_string()),
                None => break,
            }
        }
        if !sub_items.is_empty() {
            item.capacity_reservation_resource_group_arns =
                Some(CapacityReservationResourceGroupArns { items: sub_items });
            found = true;
        }
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_metric_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<Metric>, String> {
    let mut item = Metric::default();
    let mut found = false;
    {
        let mut sub_items = Vec::new();
        let sub_list_prefix = format!("{prefix}.Dimensions");
        for i in 1.. {
            let item_key = format!("{sub_list_prefix}.member.{i}");
            match deserialize_metric_dimension_from_query(params, &item_key)? {
                Some(sub_item) => sub_items.push(sub_item),
                None => break,
            }
        }
        if !sub_items.is_empty() {
            item.dimensions = Some(MetricDimensions { items: sub_items });
            found = true;
        }
    }
    if let Some(value) = params.get(&format!("{prefix}.MetricName")) {
        item.metric_name = value.to_string();
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Namespace")) {
        item.namespace = value.to_string();
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_alarm_specification_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<AlarmSpecification>, String> {
    let mut item = AlarmSpecification::default();
    let mut found = false;
    {
        let mut sub_items = Vec::new();
        let sub_list_prefix = format!("{prefix}.Alarms");
        for i in 1.. {
            let item_key = format!("{sub_list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => sub_items.push(value.to_string()),
                None => break,
            }
        }
        if !sub_items.is_empty() {
            item.alarms = Some(AlarmList { items: sub_items });
            found = true;
        }
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_instance_reuse_policy_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<InstanceReusePolicy>, String> {
    let mut item = InstanceReusePolicy::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.ReuseOnScaleIn")) {
        item.reuse_on_scale_in = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse ReuseOnScaleIn: {e}"))?,
        );
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_predictive_scaling_predefined_scaling_metric_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<PredictiveScalingPredefinedScalingMetric>, String> {
    let mut item = PredictiveScalingPredefinedScalingMetric::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.PredefinedMetricType")) {
        item.predefined_metric_type = value.to_string();
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.ResourceLabel")) {
        item.resource_label = Some(value.to_string());
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_capacity_reservation_specification_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<CapacityReservationSpecification>, String> {
    let mut item = CapacityReservationSpecification::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.CapacityReservationPreference")) {
        item.capacity_reservation_preference = Some(value.to_string());
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_v_cpu_count_request_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<VCpuCountRequest>, String> {
    let mut item = VCpuCountRequest::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.Max")) {
        item.max = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse Max: {e}"))?,
        );
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Min")) {
        item.min = value
            .parse::<i32>()
            .map_err(|e| format!("failed to parse Min: {e}"))?;
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_instance_requirements_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<InstanceRequirements>, String> {
    let mut item = InstanceRequirements::default();
    let mut found = false;
    {
        let mut sub_items = Vec::new();
        let sub_list_prefix = format!("{prefix}.AcceleratorManufacturers");
        for i in 1.. {
            let item_key = format!("{sub_list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => sub_items.push(value.to_string()),
                None => break,
            }
        }
        if !sub_items.is_empty() {
            item.accelerator_manufacturers = Some(AcceleratorManufacturers { items: sub_items });
            found = true;
        }
    }
    {
        let mut sub_items = Vec::new();
        let sub_list_prefix = format!("{prefix}.AcceleratorNames");
        for i in 1.. {
            let item_key = format!("{sub_list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => sub_items.push(value.to_string()),
                None => break,
            }
        }
        if !sub_items.is_empty() {
            item.accelerator_names = Some(AcceleratorNames { items: sub_items });
            found = true;
        }
    }
    {
        let mut sub_items = Vec::new();
        let sub_list_prefix = format!("{prefix}.AcceleratorTypes");
        for i in 1.. {
            let item_key = format!("{sub_list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => sub_items.push(value.to_string()),
                None => break,
            }
        }
        if !sub_items.is_empty() {
            item.accelerator_types = Some(AcceleratorTypes { items: sub_items });
            found = true;
        }
    }
    {
        let mut sub_items = Vec::new();
        let sub_list_prefix = format!("{prefix}.AllowedInstanceTypes");
        for i in 1.. {
            let item_key = format!("{sub_list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => sub_items.push(value.to_string()),
                None => break,
            }
        }
        if !sub_items.is_empty() {
            item.allowed_instance_types = Some(AllowedInstanceTypes { items: sub_items });
            found = true;
        }
    }
    if let Some(value) = params.get(&format!("{prefix}.BareMetal")) {
        item.bare_metal = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.BurstablePerformance")) {
        item.burstable_performance = Some(value.to_string());
        found = true;
    }
    {
        let mut sub_items = Vec::new();
        let sub_list_prefix = format!("{prefix}.CpuManufacturers");
        for i in 1.. {
            let item_key = format!("{sub_list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => sub_items.push(value.to_string()),
                None => break,
            }
        }
        if !sub_items.is_empty() {
            item.cpu_manufacturers = Some(CpuManufacturers { items: sub_items });
            found = true;
        }
    }
    {
        let mut sub_items = Vec::new();
        let sub_list_prefix = format!("{prefix}.ExcludedInstanceTypes");
        for i in 1.. {
            let item_key = format!("{sub_list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => sub_items.push(value.to_string()),
                None => break,
            }
        }
        if !sub_items.is_empty() {
            item.excluded_instance_types = Some(ExcludedInstanceTypes { items: sub_items });
            found = true;
        }
    }
    {
        let mut sub_items = Vec::new();
        let sub_list_prefix = format!("{prefix}.InstanceGenerations");
        for i in 1.. {
            let item_key = format!("{sub_list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => sub_items.push(value.to_string()),
                None => break,
            }
        }
        if !sub_items.is_empty() {
            item.instance_generations = Some(InstanceGenerations { items: sub_items });
            found = true;
        }
    }
    if let Some(value) = params.get(&format!("{prefix}.LocalStorage")) {
        item.local_storage = Some(value.to_string());
        found = true;
    }
    {
        let mut sub_items = Vec::new();
        let sub_list_prefix = format!("{prefix}.LocalStorageTypes");
        for i in 1.. {
            let item_key = format!("{sub_list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => sub_items.push(value.to_string()),
                None => break,
            }
        }
        if !sub_items.is_empty() {
            item.local_storage_types = Some(LocalStorageTypes { items: sub_items });
            found = true;
        }
    }
    if let Some(value) = params.get(&format!(
        "{prefix}.MaxSpotPriceAsPercentageOfOptimalOnDemandPrice"
    )) {
        item.max_spot_price_as_percentage_of_optimal_on_demand_price =
            Some(value.parse::<i32>().map_err(|e| {
                format!("failed to parse MaxSpotPriceAsPercentageOfOptimalOnDemandPrice: {e}")
            })?);
        found = true;
    }
    if let Some(value) = params.get(&format!(
        "{prefix}.OnDemandMaxPricePercentageOverLowestPrice"
    )) {
        item.on_demand_max_price_percentage_over_lowest_price =
            Some(value.parse::<i32>().map_err(|e| {
                format!("failed to parse OnDemandMaxPricePercentageOverLowestPrice: {e}")
            })?);
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.RequireHibernateSupport")) {
        item.require_hibernate_support = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse RequireHibernateSupport: {e}"))?,
        );
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.SpotMaxPricePercentageOverLowestPrice")) {
        item.spot_max_price_percentage_over_lowest_price =
            Some(value.parse::<i32>().map_err(|e| {
                format!("failed to parse SpotMaxPricePercentageOverLowestPrice: {e}")
            })?);
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_predictive_scaling_customized_capacity_metric_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<PredictiveScalingCustomizedCapacityMetric>, String> {
    let mut item = PredictiveScalingCustomizedCapacityMetric::default();
    let mut found = false;
    {
        let mut sub_items = Vec::new();
        let sub_list_prefix = format!("{prefix}.MetricDataQueries");
        for i in 1.. {
            let item_key = format!("{sub_list_prefix}.member.{i}");
            match deserialize_metric_data_query_from_query(params, &item_key)? {
                Some(sub_item) => sub_items.push(sub_item),
                None => break,
            }
        }
        if !sub_items.is_empty() {
            item.metric_data_queries = MetricDataQueries { items: sub_items };
            found = true;
        }
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_target_tracking_metric_stat_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<TargetTrackingMetricStat>, String> {
    let mut item = TargetTrackingMetricStat::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.Period")) {
        item.period = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse Period: {e}"))?,
        );
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Stat")) {
        item.stat = value.to_string();
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Unit")) {
        item.unit = Some(value.to_string());
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_filter_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<Filter>, String> {
    let mut item = Filter::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.Name")) {
        item.name = Some(value.to_string());
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
            item.values = Some(Values { items: sub_items });
            found = true;
        }
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_target_tracking_configuration_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<TargetTrackingConfiguration>, String> {
    let mut item = TargetTrackingConfiguration::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.DisableScaleIn")) {
        item.disable_scale_in = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse DisableScaleIn: {e}"))?,
        );
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.TargetValue")) {
        item.target_value = value
            .parse::<f64>()
            .map_err(|e| format!("failed to parse TargetValue: {e}"))?;
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_accelerator_count_request_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<AcceleratorCountRequest>, String> {
    let mut item = AcceleratorCountRequest::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.Max")) {
        item.max = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse Max: {e}"))?,
        );
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Min")) {
        item.min = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse Min: {e}"))?,
        );
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_baseline_performance_factors_request_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<BaselinePerformanceFactorsRequest>, String> {
    let mut item = BaselinePerformanceFactorsRequest::default();
    let mut found = false;
    Ok(if found { Some(item) } else { None })
}

fn deserialize_target_tracking_metric_data_query_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<TargetTrackingMetricDataQuery>, String> {
    let mut item = TargetTrackingMetricDataQuery::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.Expression")) {
        item.expression = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Id")) {
        item.id = value.to_string();
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Label")) {
        item.label = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Period")) {
        item.period = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse Period: {e}"))?,
        );
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.ReturnData")) {
        item.return_data = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse ReturnData: {e}"))?,
        );
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_predictive_scaling_metric_specification_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<PredictiveScalingMetricSpecification>, String> {
    let mut item = PredictiveScalingMetricSpecification::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.TargetValue")) {
        item.target_value = value
            .parse::<f64>()
            .map_err(|e| format!("failed to parse TargetValue: {e}"))?;
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_customized_metric_specification_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<CustomizedMetricSpecification>, String> {
    let mut item = CustomizedMetricSpecification::default();
    let mut found = false;
    {
        let mut sub_items = Vec::new();
        let sub_list_prefix = format!("{prefix}.Dimensions");
        for i in 1.. {
            let item_key = format!("{sub_list_prefix}.member.{i}");
            match deserialize_metric_dimension_from_query(params, &item_key)? {
                Some(sub_item) => sub_items.push(sub_item),
                None => break,
            }
        }
        if !sub_items.is_empty() {
            item.dimensions = Some(MetricDimensions { items: sub_items });
            found = true;
        }
    }
    if let Some(value) = params.get(&format!("{prefix}.MetricName")) {
        item.metric_name = Some(value.to_string());
        found = true;
    }
    {
        let mut sub_items = Vec::new();
        let sub_list_prefix = format!("{prefix}.Metrics");
        for i in 1.. {
            let item_key = format!("{sub_list_prefix}.member.{i}");
            match deserialize_target_tracking_metric_data_query_from_query(params, &item_key)? {
                Some(sub_item) => sub_items.push(sub_item),
                None => break,
            }
        }
        if !sub_items.is_empty() {
            item.metrics = Some(TargetTrackingMetricDataQueries { items: sub_items });
            found = true;
        }
    }
    if let Some(value) = params.get(&format!("{prefix}.Namespace")) {
        item.namespace = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Period")) {
        item.period = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse Period: {e}"))?,
        );
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Statistic")) {
        item.statistic = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Unit")) {
        item.unit = Some(value.to_string());
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
    if let Some(value) = params.get(&format!("{prefix}.PropagateAtLaunch")) {
        item.propagate_at_launch = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse PropagateAtLaunch: {e}"))?,
        );
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.ResourceId")) {
        item.resource_id = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.ResourceType")) {
        item.resource_type = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Value")) {
        item.value = Some(value.to_string());
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_network_interface_count_request_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<NetworkInterfaceCountRequest>, String> {
    let mut item = NetworkInterfaceCountRequest::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.Max")) {
        item.max = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse Max: {e}"))?,
        );
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Min")) {
        item.min = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse Min: {e}"))?,
        );
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_metric_data_query_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<MetricDataQuery>, String> {
    let mut item = MetricDataQuery::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.Expression")) {
        item.expression = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Id")) {
        item.id = value.to_string();
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Label")) {
        item.label = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.ReturnData")) {
        item.return_data = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse ReturnData: {e}"))?,
        );
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_launch_template_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<LaunchTemplate>, String> {
    let mut item = LaunchTemplate::default();
    let mut found = false;
    {
        let mut sub_items = Vec::new();
        let sub_list_prefix = format!("{prefix}.Overrides");
        for i in 1.. {
            let item_key = format!("{sub_list_prefix}.member.{i}");
            match deserialize_launch_template_overrides_from_query(params, &item_key)? {
                Some(sub_item) => sub_items.push(sub_item),
                None => break,
            }
        }
        if !sub_items.is_empty() {
            item.overrides = Some(Overrides { items: sub_items });
            found = true;
        }
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_cpu_performance_factor_request_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<CpuPerformanceFactorRequest>, String> {
    let mut item = CpuPerformanceFactorRequest::default();
    let mut found = false;
    {
        let mut sub_items = Vec::new();
        let sub_list_prefix = format!("{prefix}.References");
        for i in 1.. {
            let item_key = format!("{sub_list_prefix}.item.{i}");
            match deserialize_performance_factor_reference_request_from_query(params, &item_key)? {
                Some(sub_item) => sub_items.push(sub_item),
                None => break,
            }
        }
        if !sub_items.is_empty() {
            item.references = Some(PerformanceFactorReferenceSetRequest { items: sub_items });
            found = true;
        }
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_step_adjustment_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<StepAdjustment>, String> {
    let mut item = StepAdjustment::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.MetricIntervalLowerBound")) {
        item.metric_interval_lower_bound = Some(
            value
                .parse::<f64>()
                .map_err(|e| format!("failed to parse MetricIntervalLowerBound: {e}"))?,
        );
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.MetricIntervalUpperBound")) {
        item.metric_interval_upper_bound = Some(
            value
                .parse::<f64>()
                .map_err(|e| format!("failed to parse MetricIntervalUpperBound: {e}"))?,
        );
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.ScalingAdjustment")) {
        item.scaling_adjustment = value
            .parse::<i32>()
            .map_err(|e| format!("failed to parse ScalingAdjustment: {e}"))?;
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_predictive_scaling_predefined_metric_pair_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<PredictiveScalingPredefinedMetricPair>, String> {
    let mut item = PredictiveScalingPredefinedMetricPair::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.PredefinedMetricType")) {
        item.predefined_metric_type = value.to_string();
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.ResourceLabel")) {
        item.resource_label = Some(value.to_string());
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_launch_template_specification_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<LaunchTemplateSpecification>, String> {
    let mut item = LaunchTemplateSpecification::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.LaunchTemplateId")) {
        item.launch_template_id = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.LaunchTemplateName")) {
        item.launch_template_name = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Version")) {
        item.version = Some(value.to_string());
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_performance_factor_reference_request_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<PerformanceFactorReferenceRequest>, String> {
    let mut item = PerformanceFactorReferenceRequest::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.InstanceFamily")) {
        item.instance_family = Some(value.to_string());
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

/// Deserialize awsQuery request for AttachInstances.
pub fn deserialize_attach_instances_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<AttachInstancesQuery, String> {
    let mut input = AttachInstancesQuery::default();
    if let Some(value) = params.get("AutoScalingGroupName") {
        input.auto_scaling_group_name = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "InstanceIds".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.instance_ids = Some(InstanceIds { items });
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for AttachLoadBalancerTargetGroups.
pub fn deserialize_attach_load_balancer_target_groups_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<AttachLoadBalancerTargetGroupsType, String> {
    let mut input = AttachLoadBalancerTargetGroupsType::default();
    if let Some(value) = params.get("AutoScalingGroupName") {
        input.auto_scaling_group_name = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "TargetGroupARNs".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.target_group_a_r_ns = TargetGroupARNs { items };
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for AttachLoadBalancers.
pub fn deserialize_attach_load_balancers_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<AttachLoadBalancersType, String> {
    let mut input = AttachLoadBalancersType::default();
    if let Some(value) = params.get("AutoScalingGroupName") {
        input.auto_scaling_group_name = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "LoadBalancerNames".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.load_balancer_names = LoadBalancerNames { items };
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for AttachTrafficSources.
pub fn deserialize_attach_traffic_sources_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<AttachTrafficSourcesType, String> {
    let mut input = AttachTrafficSourcesType::default();
    if let Some(value) = params.get("AutoScalingGroupName") {
        input.auto_scaling_group_name = value.to_string();
    }
    if let Some(value) = params.get("SkipZonalShiftValidation") {
        input.skip_zonal_shift_validation = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse SkipZonalShiftValidation: {e}"))?,
        );
    }
    {
        let mut items = Vec::new();
        let list_prefix = "TrafficSources".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_traffic_source_identifier_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.traffic_sources = TrafficSources { items };
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for BatchDeleteScheduledAction.
pub fn deserialize_batch_delete_scheduled_action_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<BatchDeleteScheduledActionType, String> {
    let mut input = BatchDeleteScheduledActionType::default();
    if let Some(value) = params.get("AutoScalingGroupName") {
        input.auto_scaling_group_name = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "ScheduledActionNames".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.scheduled_action_names = ScheduledActionNames { items };
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for BatchPutScheduledUpdateGroupAction.
pub fn deserialize_batch_put_scheduled_update_group_action_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<BatchPutScheduledUpdateGroupActionType, String> {
    let mut input = BatchPutScheduledUpdateGroupActionType::default();
    if let Some(value) = params.get("AutoScalingGroupName") {
        input.auto_scaling_group_name = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "ScheduledUpdateGroupActions".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_scheduled_update_group_action_request_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.scheduled_update_group_actions = ScheduledUpdateGroupActionRequests { items };
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for CancelInstanceRefresh.
pub fn deserialize_cancel_instance_refresh_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CancelInstanceRefreshType, String> {
    let mut input = CancelInstanceRefreshType::default();
    if let Some(value) = params.get("AutoScalingGroupName") {
        input.auto_scaling_group_name = value.to_string();
    }
    if let Some(value) = params.get("WaitForTransitioningInstances") {
        input.wait_for_transitioning_instances = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse WaitForTransitioningInstances: {e}"))?,
        );
    }
    Ok(input)
}

/// Deserialize awsQuery request for CompleteLifecycleAction.
pub fn deserialize_complete_lifecycle_action_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CompleteLifecycleActionType, String> {
    let mut input = CompleteLifecycleActionType::default();
    if let Some(value) = params.get("AutoScalingGroupName") {
        input.auto_scaling_group_name = value.to_string();
    }
    if let Some(value) = params.get("InstanceId") {
        input.instance_id = Some(value.to_string());
    }
    if let Some(value) = params.get("LifecycleActionResult") {
        input.lifecycle_action_result = value.to_string();
    }
    if let Some(value) = params.get("LifecycleActionToken") {
        input.lifecycle_action_token = Some(value.to_string());
    }
    if let Some(value) = params.get("LifecycleHookName") {
        input.lifecycle_hook_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for CreateAutoScalingGroup.
pub fn deserialize_create_auto_scaling_group_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateAutoScalingGroupType, String> {
    let mut input = CreateAutoScalingGroupType::default();
    if let Some(value) = params.get("AutoScalingGroupName") {
        input.auto_scaling_group_name = value.to_string();
    }
    if let Some(val) = deserialize_availability_zone_distribution_from_query(
        params,
        "AvailabilityZoneDistribution",
    )? {
        input.availability_zone_distribution = Some(val);
    }
    if let Some(val) = deserialize_availability_zone_impairment_policy_from_query(
        params,
        "AvailabilityZoneImpairmentPolicy",
    )? {
        input.availability_zone_impairment_policy = Some(val);
    }
    {
        let mut items = Vec::new();
        let list_prefix = "AvailabilityZones".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.availability_zones = Some(AvailabilityZones { items });
        }
    }
    if let Some(value) = params.get("CapacityRebalance") {
        input.capacity_rebalance = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse CapacityRebalance: {e}"))?,
        );
    }
    if let Some(val) = deserialize_capacity_reservation_specification_from_query(
        params,
        "CapacityReservationSpecification",
    )? {
        input.capacity_reservation_specification = Some(val);
    }
    if let Some(value) = params.get("Context") {
        input.context = Some(value.to_string());
    }
    if let Some(value) = params.get("DefaultCooldown") {
        input.default_cooldown = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse DefaultCooldown: {e}"))?,
        );
    }
    if let Some(value) = params.get("DefaultInstanceWarmup") {
        input.default_instance_warmup = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse DefaultInstanceWarmup: {e}"))?,
        );
    }
    if let Some(value) = params.get("DeletionProtection") {
        input.deletion_protection = Some(value.to_string());
    }
    if let Some(value) = params.get("DesiredCapacity") {
        input.desired_capacity = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse DesiredCapacity: {e}"))?,
        );
    }
    if let Some(value) = params.get("DesiredCapacityType") {
        input.desired_capacity_type = Some(value.to_string());
    }
    if let Some(value) = params.get("HealthCheckGracePeriod") {
        input.health_check_grace_period = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse HealthCheckGracePeriod: {e}"))?,
        );
    }
    if let Some(value) = params.get("HealthCheckType") {
        input.health_check_type = Some(value.to_string());
    }
    if let Some(value) = params.get("InstanceId") {
        input.instance_id = Some(value.to_string());
    }
    if let Some(val) =
        deserialize_instance_lifecycle_policy_from_query(params, "InstanceLifecyclePolicy")?
    {
        input.instance_lifecycle_policy = Some(val);
    }
    if let Some(val) =
        deserialize_instance_maintenance_policy_from_query(params, "InstanceMaintenancePolicy")?
    {
        input.instance_maintenance_policy = Some(val);
    }
    if let Some(value) = params.get("LaunchConfigurationName") {
        input.launch_configuration_name = Some(value.to_string());
    }
    if let Some(val) =
        deserialize_launch_template_specification_from_query(params, "LaunchTemplate")?
    {
        input.launch_template = Some(val);
    }
    {
        let mut items = Vec::new();
        let list_prefix = "LifecycleHookSpecificationList".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_lifecycle_hook_specification_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.lifecycle_hook_specification_list = Some(LifecycleHookSpecifications { items });
        }
    }
    {
        let mut items = Vec::new();
        let list_prefix = "LoadBalancerNames".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.load_balancer_names = Some(LoadBalancerNames { items });
        }
    }
    if let Some(value) = params.get("MaxInstanceLifetime") {
        input.max_instance_lifetime = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxInstanceLifetime: {e}"))?,
        );
    }
    if let Some(value) = params.get("MaxSize") {
        input.max_size = value
            .parse::<i32>()
            .map_err(|e| format!("failed to parse MaxSize: {e}"))?;
    }
    if let Some(value) = params.get("MinSize") {
        input.min_size = value
            .parse::<i32>()
            .map_err(|e| format!("failed to parse MinSize: {e}"))?;
    }
    if let Some(val) =
        deserialize_mixed_instances_policy_from_query(params, "MixedInstancesPolicy")?
    {
        input.mixed_instances_policy = Some(val);
    }
    if let Some(value) = params.get("NewInstancesProtectedFromScaleIn") {
        input.new_instances_protected_from_scale_in = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse NewInstancesProtectedFromScaleIn: {e}"))?,
        );
    }
    if let Some(value) = params.get("PlacementGroup") {
        input.placement_group = Some(value.to_string());
    }
    if let Some(value) = params.get("ServiceLinkedRoleARN") {
        input.service_linked_role_a_r_n = Some(value.to_string());
    }
    if let Some(value) = params.get("SkipZonalShiftValidation") {
        input.skip_zonal_shift_validation = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse SkipZonalShiftValidation: {e}"))?,
        );
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
    {
        let mut items = Vec::new();
        let list_prefix = "TargetGroupARNs".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.target_group_a_r_ns = Some(TargetGroupARNs { items });
        }
    }
    {
        let mut items = Vec::new();
        let list_prefix = "TerminationPolicies".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.termination_policies = Some(TerminationPolicies { items });
        }
    }
    {
        let mut items = Vec::new();
        let list_prefix = "TrafficSources".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_traffic_source_identifier_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.traffic_sources = Some(TrafficSources { items });
        }
    }
    if let Some(value) = params.get("VPCZoneIdentifier") {
        input.v_p_c_zone_identifier = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for CreateLaunchConfiguration.
pub fn deserialize_create_launch_configuration_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateLaunchConfigurationType, String> {
    let mut input = CreateLaunchConfigurationType::default();
    if let Some(value) = params.get("AssociatePublicIpAddress") {
        input.associate_public_ip_address = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse AssociatePublicIpAddress: {e}"))?,
        );
    }
    {
        let mut items = Vec::new();
        let list_prefix = "BlockDeviceMappings".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_block_device_mapping_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.block_device_mappings = Some(BlockDeviceMappings { items });
        }
    }
    if let Some(value) = params.get("ClassicLinkVPCId") {
        input.classic_link_v_p_c_id = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "ClassicLinkVPCSecurityGroups".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.classic_link_v_p_c_security_groups = Some(ClassicLinkVPCSecurityGroups { items });
        }
    }
    if let Some(value) = params.get("EbsOptimized") {
        input.ebs_optimized = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse EbsOptimized: {e}"))?,
        );
    }
    if let Some(value) = params.get("IamInstanceProfile") {
        input.iam_instance_profile = Some(value.to_string());
    }
    if let Some(value) = params.get("ImageId") {
        input.image_id = Some(value.to_string());
    }
    if let Some(value) = params.get("InstanceId") {
        input.instance_id = Some(value.to_string());
    }
    if let Some(val) = deserialize_instance_monitoring_from_query(params, "InstanceMonitoring")? {
        input.instance_monitoring = Some(val);
    }
    if let Some(value) = params.get("InstanceType") {
        input.instance_type = Some(value.to_string());
    }
    if let Some(value) = params.get("KernelId") {
        input.kernel_id = Some(value.to_string());
    }
    if let Some(value) = params.get("KeyName") {
        input.key_name = Some(value.to_string());
    }
    if let Some(value) = params.get("LaunchConfigurationName") {
        input.launch_configuration_name = value.to_string();
    }
    if let Some(val) = deserialize_instance_metadata_options_from_query(params, "MetadataOptions")?
    {
        input.metadata_options = Some(val);
    }
    if let Some(value) = params.get("PlacementTenancy") {
        input.placement_tenancy = Some(value.to_string());
    }
    if let Some(value) = params.get("RamdiskId") {
        input.ramdisk_id = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "SecurityGroups".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.security_groups = Some(SecurityGroups { items });
        }
    }
    if let Some(value) = params.get("SpotPrice") {
        input.spot_price = Some(value.to_string());
    }
    if let Some(value) = params.get("UserData") {
        input.user_data = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for CreateOrUpdateTags.
pub fn deserialize_create_or_update_tags_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<CreateOrUpdateTagsType, String> {
    let mut input = CreateOrUpdateTagsType::default();
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
            input.tags = Tags { items };
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteAutoScalingGroup.
pub fn deserialize_delete_auto_scaling_group_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteAutoScalingGroupType, String> {
    let mut input = DeleteAutoScalingGroupType::default();
    if let Some(value) = params.get("AutoScalingGroupName") {
        input.auto_scaling_group_name = value.to_string();
    }
    if let Some(value) = params.get("ForceDelete") {
        input.force_delete = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse ForceDelete: {e}"))?,
        );
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteLaunchConfiguration.
pub fn deserialize_delete_launch_configuration_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<LaunchConfigurationNameType, String> {
    let mut input = LaunchConfigurationNameType::default();
    if let Some(value) = params.get("LaunchConfigurationName") {
        input.launch_configuration_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteLifecycleHook.
pub fn deserialize_delete_lifecycle_hook_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteLifecycleHookType, String> {
    let mut input = DeleteLifecycleHookType::default();
    if let Some(value) = params.get("AutoScalingGroupName") {
        input.auto_scaling_group_name = value.to_string();
    }
    if let Some(value) = params.get("LifecycleHookName") {
        input.lifecycle_hook_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteNotificationConfiguration.
pub fn deserialize_delete_notification_configuration_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteNotificationConfigurationType, String> {
    let mut input = DeleteNotificationConfigurationType::default();
    if let Some(value) = params.get("AutoScalingGroupName") {
        input.auto_scaling_group_name = value.to_string();
    }
    if let Some(value) = params.get("TopicARN") {
        input.topic_a_r_n = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeletePolicy.
pub fn deserialize_delete_policy_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeletePolicyType, String> {
    let mut input = DeletePolicyType::default();
    if let Some(value) = params.get("AutoScalingGroupName") {
        input.auto_scaling_group_name = Some(value.to_string());
    }
    if let Some(value) = params.get("PolicyName") {
        input.policy_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteScheduledAction.
pub fn deserialize_delete_scheduled_action_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteScheduledActionType, String> {
    let mut input = DeleteScheduledActionType::default();
    if let Some(value) = params.get("AutoScalingGroupName") {
        input.auto_scaling_group_name = value.to_string();
    }
    if let Some(value) = params.get("ScheduledActionName") {
        input.scheduled_action_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteTags.
pub fn deserialize_delete_tags_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteTagsType, String> {
    let mut input = DeleteTagsType::default();
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
            input.tags = Tags { items };
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteWarmPool.
pub fn deserialize_delete_warm_pool_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteWarmPoolType, String> {
    let mut input = DeleteWarmPoolType::default();
    if let Some(value) = params.get("AutoScalingGroupName") {
        input.auto_scaling_group_name = value.to_string();
    }
    if let Some(value) = params.get("ForceDelete") {
        input.force_delete = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse ForceDelete: {e}"))?,
        );
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeAutoScalingGroups.
pub fn deserialize_describe_auto_scaling_groups_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<AutoScalingGroupNamesType, String> {
    let mut input = AutoScalingGroupNamesType::default();
    {
        let mut items = Vec::new();
        let list_prefix = "AutoScalingGroupNames".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.auto_scaling_group_names = Some(AutoScalingGroupNames { items });
        }
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Filters".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_filter_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.filters = Some(Filters { items });
        }
    }
    if let Some(value) = params.get("IncludeInstances") {
        input.include_instances = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse IncludeInstances: {e}"))?,
        );
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

/// Deserialize awsQuery request for DescribeAutoScalingInstances.
pub fn deserialize_describe_auto_scaling_instances_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeAutoScalingInstancesType, String> {
    let mut input = DescribeAutoScalingInstancesType::default();
    {
        let mut items = Vec::new();
        let list_prefix = "InstanceIds".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.instance_ids = Some(InstanceIds { items });
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

/// Deserialize awsQuery request for DescribeInstanceRefreshes.
pub fn deserialize_describe_instance_refreshes_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeInstanceRefreshesType, String> {
    let mut input = DescribeInstanceRefreshesType::default();
    if let Some(value) = params.get("AutoScalingGroupName") {
        input.auto_scaling_group_name = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "InstanceRefreshIds".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.instance_refresh_ids = Some(InstanceRefreshIds { items });
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

/// Deserialize awsQuery request for DescribeLaunchConfigurations.
pub fn deserialize_describe_launch_configurations_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<LaunchConfigurationNamesType, String> {
    let mut input = LaunchConfigurationNamesType::default();
    {
        let mut items = Vec::new();
        let list_prefix = "LaunchConfigurationNames".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.launch_configuration_names = Some(LaunchConfigurationNames { items });
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

/// Deserialize awsQuery request for DescribeLifecycleHooks.
pub fn deserialize_describe_lifecycle_hooks_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeLifecycleHooksType, String> {
    let mut input = DescribeLifecycleHooksType::default();
    if let Some(value) = params.get("AutoScalingGroupName") {
        input.auto_scaling_group_name = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "LifecycleHookNames".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.lifecycle_hook_names = Some(LifecycleHookNames { items });
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeLoadBalancerTargetGroups.
pub fn deserialize_describe_load_balancer_target_groups_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeLoadBalancerTargetGroupsRequest, String> {
    let mut input = DescribeLoadBalancerTargetGroupsRequest::default();
    if let Some(value) = params.get("AutoScalingGroupName") {
        input.auto_scaling_group_name = value.to_string();
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

/// Deserialize awsQuery request for DescribeLoadBalancers.
pub fn deserialize_describe_load_balancers_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeLoadBalancersRequest, String> {
    let mut input = DescribeLoadBalancersRequest::default();
    if let Some(value) = params.get("AutoScalingGroupName") {
        input.auto_scaling_group_name = value.to_string();
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

/// Deserialize awsQuery request for DescribeNotificationConfigurations.
pub fn deserialize_describe_notification_configurations_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeNotificationConfigurationsType, String> {
    let mut input = DescribeNotificationConfigurationsType::default();
    {
        let mut items = Vec::new();
        let list_prefix = "AutoScalingGroupNames".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.auto_scaling_group_names = Some(AutoScalingGroupNames { items });
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

/// Deserialize awsQuery request for DescribePolicies.
pub fn deserialize_describe_policies_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribePoliciesType, String> {
    let mut input = DescribePoliciesType::default();
    if let Some(value) = params.get("AutoScalingGroupName") {
        input.auto_scaling_group_name = Some(value.to_string());
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
        let list_prefix = "PolicyNames".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.policy_names = Some(PolicyNames { items });
        }
    }
    {
        let mut items = Vec::new();
        let list_prefix = "PolicyTypes".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.policy_types = Some(PolicyTypes { items });
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeScalingActivities.
pub fn deserialize_describe_scaling_activities_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeScalingActivitiesType, String> {
    let mut input = DescribeScalingActivitiesType::default();
    {
        let mut items = Vec::new();
        let list_prefix = "ActivityIds".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.activity_ids = Some(ActivityIds { items });
        }
    }
    if let Some(value) = params.get("AutoScalingGroupName") {
        input.auto_scaling_group_name = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Filters".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_filter_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.filters = Some(Filters { items });
        }
    }
    if let Some(value) = params.get("IncludeDeletedGroups") {
        input.include_deleted_groups = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse IncludeDeletedGroups: {e}"))?,
        );
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

/// Deserialize awsQuery request for DescribeScheduledActions.
pub fn deserialize_describe_scheduled_actions_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeScheduledActionsType, String> {
    let mut input = DescribeScheduledActionsType::default();
    if let Some(value) = params.get("AutoScalingGroupName") {
        input.auto_scaling_group_name = Some(value.to_string());
    }
    if let Some(value) = params.get("EndTime") {
        input.end_time = Some(value.to_string());
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
        let list_prefix = "ScheduledActionNames".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.scheduled_action_names = Some(ScheduledActionNames { items });
        }
    }
    if let Some(value) = params.get("StartTime") {
        input.start_time = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeTags.
pub fn deserialize_describe_tags_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeTagsType, String> {
    let mut input = DescribeTagsType::default();
    {
        let mut items = Vec::new();
        let list_prefix = "Filters".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_filter_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.filters = Some(Filters { items });
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

/// Deserialize awsQuery request for DescribeTrafficSources.
pub fn deserialize_describe_traffic_sources_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeTrafficSourcesRequest, String> {
    let mut input = DescribeTrafficSourcesRequest::default();
    if let Some(value) = params.get("AutoScalingGroupName") {
        input.auto_scaling_group_name = value.to_string();
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
    if let Some(value) = params.get("TrafficSourceType") {
        input.traffic_source_type = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeWarmPool.
pub fn deserialize_describe_warm_pool_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeWarmPoolType, String> {
    let mut input = DescribeWarmPoolType::default();
    if let Some(value) = params.get("AutoScalingGroupName") {
        input.auto_scaling_group_name = value.to_string();
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

/// Deserialize awsQuery request for DetachInstances.
pub fn deserialize_detach_instances_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DetachInstancesQuery, String> {
    let mut input = DetachInstancesQuery::default();
    if let Some(value) = params.get("AutoScalingGroupName") {
        input.auto_scaling_group_name = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "InstanceIds".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.instance_ids = Some(InstanceIds { items });
        }
    }
    if let Some(value) = params.get("ShouldDecrementDesiredCapacity") {
        input.should_decrement_desired_capacity = value
            .parse::<bool>()
            .map_err(|e| format!("failed to parse ShouldDecrementDesiredCapacity: {e}"))?;
    }
    Ok(input)
}

/// Deserialize awsQuery request for DetachLoadBalancerTargetGroups.
pub fn deserialize_detach_load_balancer_target_groups_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DetachLoadBalancerTargetGroupsType, String> {
    let mut input = DetachLoadBalancerTargetGroupsType::default();
    if let Some(value) = params.get("AutoScalingGroupName") {
        input.auto_scaling_group_name = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "TargetGroupARNs".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.target_group_a_r_ns = TargetGroupARNs { items };
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for DetachLoadBalancers.
pub fn deserialize_detach_load_balancers_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DetachLoadBalancersType, String> {
    let mut input = DetachLoadBalancersType::default();
    if let Some(value) = params.get("AutoScalingGroupName") {
        input.auto_scaling_group_name = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "LoadBalancerNames".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.load_balancer_names = LoadBalancerNames { items };
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for DetachTrafficSources.
pub fn deserialize_detach_traffic_sources_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DetachTrafficSourcesType, String> {
    let mut input = DetachTrafficSourcesType::default();
    if let Some(value) = params.get("AutoScalingGroupName") {
        input.auto_scaling_group_name = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "TrafficSources".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_traffic_source_identifier_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.traffic_sources = TrafficSources { items };
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for DisableMetricsCollection.
pub fn deserialize_disable_metrics_collection_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<DisableMetricsCollectionQuery, String> {
    let mut input = DisableMetricsCollectionQuery::default();
    if let Some(value) = params.get("AutoScalingGroupName") {
        input.auto_scaling_group_name = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Metrics".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.metrics = Some(Metrics { items });
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for EnableMetricsCollection.
pub fn deserialize_enable_metrics_collection_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<EnableMetricsCollectionQuery, String> {
    let mut input = EnableMetricsCollectionQuery::default();
    if let Some(value) = params.get("AutoScalingGroupName") {
        input.auto_scaling_group_name = value.to_string();
    }
    if let Some(value) = params.get("Granularity") {
        input.granularity = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Metrics".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.metrics = Some(Metrics { items });
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for EnterStandby.
pub fn deserialize_enter_standby_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<EnterStandbyQuery, String> {
    let mut input = EnterStandbyQuery::default();
    if let Some(value) = params.get("AutoScalingGroupName") {
        input.auto_scaling_group_name = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "InstanceIds".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.instance_ids = Some(InstanceIds { items });
        }
    }
    if let Some(value) = params.get("ShouldDecrementDesiredCapacity") {
        input.should_decrement_desired_capacity = value
            .parse::<bool>()
            .map_err(|e| format!("failed to parse ShouldDecrementDesiredCapacity: {e}"))?;
    }
    Ok(input)
}

/// Deserialize awsQuery request for ExecutePolicy.
pub fn deserialize_execute_policy_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ExecutePolicyType, String> {
    let mut input = ExecutePolicyType::default();
    if let Some(value) = params.get("AutoScalingGroupName") {
        input.auto_scaling_group_name = Some(value.to_string());
    }
    if let Some(value) = params.get("BreachThreshold") {
        input.breach_threshold = Some(
            value
                .parse::<f64>()
                .map_err(|e| format!("failed to parse BreachThreshold: {e}"))?,
        );
    }
    if let Some(value) = params.get("HonorCooldown") {
        input.honor_cooldown = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse HonorCooldown: {e}"))?,
        );
    }
    if let Some(value) = params.get("MetricValue") {
        input.metric_value = Some(
            value
                .parse::<f64>()
                .map_err(|e| format!("failed to parse MetricValue: {e}"))?,
        );
    }
    if let Some(value) = params.get("PolicyName") {
        input.policy_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for ExitStandby.
pub fn deserialize_exit_standby_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ExitStandbyQuery, String> {
    let mut input = ExitStandbyQuery::default();
    if let Some(value) = params.get("AutoScalingGroupName") {
        input.auto_scaling_group_name = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "InstanceIds".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.instance_ids = Some(InstanceIds { items });
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for GetPredictiveScalingForecast.
pub fn deserialize_get_predictive_scaling_forecast_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<GetPredictiveScalingForecastType, String> {
    let mut input = GetPredictiveScalingForecastType::default();
    if let Some(value) = params.get("AutoScalingGroupName") {
        input.auto_scaling_group_name = value.to_string();
    }
    if let Some(value) = params.get("EndTime") {
        input.end_time = value.to_string();
    }
    if let Some(value) = params.get("PolicyName") {
        input.policy_name = value.to_string();
    }
    if let Some(value) = params.get("StartTime") {
        input.start_time = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for LaunchInstances.
pub fn deserialize_launch_instances_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<LaunchInstancesRequest, String> {
    let mut input = LaunchInstancesRequest::default();
    if let Some(value) = params.get("AutoScalingGroupName") {
        input.auto_scaling_group_name = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "AvailabilityZoneIds".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.availability_zone_ids = Some(AvailabilityZoneIdsLimit1 { items });
        }
    }
    {
        let mut items = Vec::new();
        let list_prefix = "AvailabilityZones".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.availability_zones = Some(AvailabilityZonesLimit1 { items });
        }
    }
    if let Some(value) = params.get("ClientToken") {
        input.client_token = value.to_string();
    }
    if let Some(value) = params.get("RequestedCapacity") {
        input.requested_capacity = value
            .parse::<i32>()
            .map_err(|e| format!("failed to parse RequestedCapacity: {e}"))?;
    }
    if let Some(value) = params.get("RetryStrategy") {
        input.retry_strategy = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "SubnetIds".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.subnet_ids = Some(SubnetIdsLimit1 { items });
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for PutLifecycleHook.
pub fn deserialize_put_lifecycle_hook_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<PutLifecycleHookType, String> {
    let mut input = PutLifecycleHookType::default();
    if let Some(value) = params.get("AutoScalingGroupName") {
        input.auto_scaling_group_name = value.to_string();
    }
    if let Some(value) = params.get("DefaultResult") {
        input.default_result = Some(value.to_string());
    }
    if let Some(value) = params.get("HeartbeatTimeout") {
        input.heartbeat_timeout = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse HeartbeatTimeout: {e}"))?,
        );
    }
    if let Some(value) = params.get("LifecycleHookName") {
        input.lifecycle_hook_name = value.to_string();
    }
    if let Some(value) = params.get("LifecycleTransition") {
        input.lifecycle_transition = Some(value.to_string());
    }
    if let Some(value) = params.get("NotificationMetadata") {
        input.notification_metadata = Some(value.to_string());
    }
    if let Some(value) = params.get("NotificationTargetARN") {
        input.notification_target_a_r_n = Some(value.to_string());
    }
    if let Some(value) = params.get("RoleARN") {
        input.role_a_r_n = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for PutNotificationConfiguration.
pub fn deserialize_put_notification_configuration_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<PutNotificationConfigurationType, String> {
    let mut input = PutNotificationConfigurationType::default();
    if let Some(value) = params.get("AutoScalingGroupName") {
        input.auto_scaling_group_name = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "NotificationTypes".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.notification_types = AutoScalingNotificationTypes { items };
        }
    }
    if let Some(value) = params.get("TopicARN") {
        input.topic_a_r_n = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for PutScalingPolicy.
pub fn deserialize_put_scaling_policy_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<PutScalingPolicyType, String> {
    let mut input = PutScalingPolicyType::default();
    if let Some(value) = params.get("AdjustmentType") {
        input.adjustment_type = Some(value.to_string());
    }
    if let Some(value) = params.get("AutoScalingGroupName") {
        input.auto_scaling_group_name = value.to_string();
    }
    if let Some(value) = params.get("Cooldown") {
        input.cooldown = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse Cooldown: {e}"))?,
        );
    }
    if let Some(value) = params.get("Enabled") {
        input.enabled = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse Enabled: {e}"))?,
        );
    }
    if let Some(value) = params.get("EstimatedInstanceWarmup") {
        input.estimated_instance_warmup = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse EstimatedInstanceWarmup: {e}"))?,
        );
    }
    if let Some(value) = params.get("MetricAggregationType") {
        input.metric_aggregation_type = Some(value.to_string());
    }
    if let Some(value) = params.get("MinAdjustmentMagnitude") {
        input.min_adjustment_magnitude = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MinAdjustmentMagnitude: {e}"))?,
        );
    }
    if let Some(value) = params.get("MinAdjustmentStep") {
        input.min_adjustment_step = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MinAdjustmentStep: {e}"))?,
        );
    }
    if let Some(value) = params.get("PolicyName") {
        input.policy_name = value.to_string();
    }
    if let Some(value) = params.get("PolicyType") {
        input.policy_type = Some(value.to_string());
    }
    if let Some(val) = deserialize_predictive_scaling_configuration_from_query(
        params,
        "PredictiveScalingConfiguration",
    )? {
        input.predictive_scaling_configuration = Some(val);
    }
    if let Some(value) = params.get("ScalingAdjustment") {
        input.scaling_adjustment = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse ScalingAdjustment: {e}"))?,
        );
    }
    {
        let mut items = Vec::new();
        let list_prefix = "StepAdjustments".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_step_adjustment_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.step_adjustments = Some(StepAdjustments { items });
        }
    }
    if let Some(val) =
        deserialize_target_tracking_configuration_from_query(params, "TargetTrackingConfiguration")?
    {
        input.target_tracking_configuration = Some(val);
    }
    Ok(input)
}

/// Deserialize awsQuery request for PutScheduledUpdateGroupAction.
pub fn deserialize_put_scheduled_update_group_action_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<PutScheduledUpdateGroupActionType, String> {
    let mut input = PutScheduledUpdateGroupActionType::default();
    if let Some(value) = params.get("AutoScalingGroupName") {
        input.auto_scaling_group_name = value.to_string();
    }
    if let Some(value) = params.get("DesiredCapacity") {
        input.desired_capacity = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse DesiredCapacity: {e}"))?,
        );
    }
    if let Some(value) = params.get("EndTime") {
        input.end_time = Some(value.to_string());
    }
    if let Some(value) = params.get("MaxSize") {
        input.max_size = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxSize: {e}"))?,
        );
    }
    if let Some(value) = params.get("MinSize") {
        input.min_size = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MinSize: {e}"))?,
        );
    }
    if let Some(value) = params.get("Recurrence") {
        input.recurrence = Some(value.to_string());
    }
    if let Some(value) = params.get("ScheduledActionName") {
        input.scheduled_action_name = value.to_string();
    }
    if let Some(value) = params.get("StartTime") {
        input.start_time = Some(value.to_string());
    }
    if let Some(value) = params.get("Time") {
        input.time = Some(value.to_string());
    }
    if let Some(value) = params.get("TimeZone") {
        input.time_zone = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for PutWarmPool.
pub fn deserialize_put_warm_pool_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<PutWarmPoolType, String> {
    let mut input = PutWarmPoolType::default();
    if let Some(value) = params.get("AutoScalingGroupName") {
        input.auto_scaling_group_name = value.to_string();
    }
    if let Some(val) = deserialize_instance_reuse_policy_from_query(params, "InstanceReusePolicy")?
    {
        input.instance_reuse_policy = Some(val);
    }
    if let Some(value) = params.get("MaxGroupPreparedCapacity") {
        input.max_group_prepared_capacity = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxGroupPreparedCapacity: {e}"))?,
        );
    }
    if let Some(value) = params.get("MinSize") {
        input.min_size = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MinSize: {e}"))?,
        );
    }
    if let Some(value) = params.get("PoolState") {
        input.pool_state = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for RecordLifecycleActionHeartbeat.
pub fn deserialize_record_lifecycle_action_heartbeat_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<RecordLifecycleActionHeartbeatType, String> {
    let mut input = RecordLifecycleActionHeartbeatType::default();
    if let Some(value) = params.get("AutoScalingGroupName") {
        input.auto_scaling_group_name = value.to_string();
    }
    if let Some(value) = params.get("InstanceId") {
        input.instance_id = Some(value.to_string());
    }
    if let Some(value) = params.get("LifecycleActionToken") {
        input.lifecycle_action_token = Some(value.to_string());
    }
    if let Some(value) = params.get("LifecycleHookName") {
        input.lifecycle_hook_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for ResumeProcesses.
pub fn deserialize_resume_processes_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ScalingProcessQuery, String> {
    let mut input = ScalingProcessQuery::default();
    if let Some(value) = params.get("AutoScalingGroupName") {
        input.auto_scaling_group_name = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "ScalingProcesses".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.scaling_processes = Some(ProcessNames { items });
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for RollbackInstanceRefresh.
pub fn deserialize_rollback_instance_refresh_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<RollbackInstanceRefreshType, String> {
    let mut input = RollbackInstanceRefreshType::default();
    if let Some(value) = params.get("AutoScalingGroupName") {
        input.auto_scaling_group_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for SetDesiredCapacity.
pub fn deserialize_set_desired_capacity_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<SetDesiredCapacityType, String> {
    let mut input = SetDesiredCapacityType::default();
    if let Some(value) = params.get("AutoScalingGroupName") {
        input.auto_scaling_group_name = value.to_string();
    }
    if let Some(value) = params.get("DesiredCapacity") {
        input.desired_capacity = value
            .parse::<i32>()
            .map_err(|e| format!("failed to parse DesiredCapacity: {e}"))?;
    }
    if let Some(value) = params.get("HonorCooldown") {
        input.honor_cooldown = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse HonorCooldown: {e}"))?,
        );
    }
    Ok(input)
}

/// Deserialize awsQuery request for SetInstanceHealth.
pub fn deserialize_set_instance_health_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<SetInstanceHealthQuery, String> {
    let mut input = SetInstanceHealthQuery::default();
    if let Some(value) = params.get("HealthStatus") {
        input.health_status = value.to_string();
    }
    if let Some(value) = params.get("InstanceId") {
        input.instance_id = value.to_string();
    }
    if let Some(value) = params.get("ShouldRespectGracePeriod") {
        input.should_respect_grace_period = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse ShouldRespectGracePeriod: {e}"))?,
        );
    }
    Ok(input)
}

/// Deserialize awsQuery request for SetInstanceProtection.
pub fn deserialize_set_instance_protection_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<SetInstanceProtectionQuery, String> {
    let mut input = SetInstanceProtectionQuery::default();
    if let Some(value) = params.get("AutoScalingGroupName") {
        input.auto_scaling_group_name = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "InstanceIds".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.instance_ids = InstanceIds { items };
        }
    }
    if let Some(value) = params.get("ProtectedFromScaleIn") {
        input.protected_from_scale_in = value
            .parse::<bool>()
            .map_err(|e| format!("failed to parse ProtectedFromScaleIn: {e}"))?;
    }
    Ok(input)
}

/// Deserialize awsQuery request for StartInstanceRefresh.
pub fn deserialize_start_instance_refresh_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<StartInstanceRefreshType, String> {
    let mut input = StartInstanceRefreshType::default();
    if let Some(value) = params.get("AutoScalingGroupName") {
        input.auto_scaling_group_name = value.to_string();
    }
    if let Some(val) = deserialize_desired_configuration_from_query(params, "DesiredConfiguration")?
    {
        input.desired_configuration = Some(val);
    }
    if let Some(val) = deserialize_refresh_preferences_from_query(params, "Preferences")? {
        input.preferences = Some(val);
    }
    if let Some(value) = params.get("Strategy") {
        input.strategy = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for SuspendProcesses.
pub fn deserialize_suspend_processes_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<ScalingProcessQuery, String> {
    let mut input = ScalingProcessQuery::default();
    if let Some(value) = params.get("AutoScalingGroupName") {
        input.auto_scaling_group_name = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "ScalingProcesses".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.scaling_processes = Some(ProcessNames { items });
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for TerminateInstanceInAutoScalingGroup.
pub fn deserialize_terminate_instance_in_auto_scaling_group_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<TerminateInstanceInAutoScalingGroupType, String> {
    let mut input = TerminateInstanceInAutoScalingGroupType::default();
    if let Some(value) = params.get("InstanceId") {
        input.instance_id = value.to_string();
    }
    if let Some(value) = params.get("ShouldDecrementDesiredCapacity") {
        input.should_decrement_desired_capacity = value
            .parse::<bool>()
            .map_err(|e| format!("failed to parse ShouldDecrementDesiredCapacity: {e}"))?;
    }
    Ok(input)
}

/// Deserialize awsQuery request for UpdateAutoScalingGroup.
pub fn deserialize_update_auto_scaling_group_request(
    params: &std::collections::HashMap<String, String>,
) -> Result<UpdateAutoScalingGroupType, String> {
    let mut input = UpdateAutoScalingGroupType::default();
    if let Some(value) = params.get("AutoScalingGroupName") {
        input.auto_scaling_group_name = value.to_string();
    }
    if let Some(val) = deserialize_availability_zone_distribution_from_query(
        params,
        "AvailabilityZoneDistribution",
    )? {
        input.availability_zone_distribution = Some(val);
    }
    if let Some(val) = deserialize_availability_zone_impairment_policy_from_query(
        params,
        "AvailabilityZoneImpairmentPolicy",
    )? {
        input.availability_zone_impairment_policy = Some(val);
    }
    {
        let mut items = Vec::new();
        let list_prefix = "AvailabilityZones".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.availability_zones = Some(AvailabilityZones { items });
        }
    }
    if let Some(value) = params.get("CapacityRebalance") {
        input.capacity_rebalance = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse CapacityRebalance: {e}"))?,
        );
    }
    if let Some(val) = deserialize_capacity_reservation_specification_from_query(
        params,
        "CapacityReservationSpecification",
    )? {
        input.capacity_reservation_specification = Some(val);
    }
    if let Some(value) = params.get("Context") {
        input.context = Some(value.to_string());
    }
    if let Some(value) = params.get("DefaultCooldown") {
        input.default_cooldown = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse DefaultCooldown: {e}"))?,
        );
    }
    if let Some(value) = params.get("DefaultInstanceWarmup") {
        input.default_instance_warmup = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse DefaultInstanceWarmup: {e}"))?,
        );
    }
    if let Some(value) = params.get("DeletionProtection") {
        input.deletion_protection = Some(value.to_string());
    }
    if let Some(value) = params.get("DesiredCapacity") {
        input.desired_capacity = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse DesiredCapacity: {e}"))?,
        );
    }
    if let Some(value) = params.get("DesiredCapacityType") {
        input.desired_capacity_type = Some(value.to_string());
    }
    if let Some(value) = params.get("HealthCheckGracePeriod") {
        input.health_check_grace_period = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse HealthCheckGracePeriod: {e}"))?,
        );
    }
    if let Some(value) = params.get("HealthCheckType") {
        input.health_check_type = Some(value.to_string());
    }
    if let Some(val) =
        deserialize_instance_lifecycle_policy_from_query(params, "InstanceLifecyclePolicy")?
    {
        input.instance_lifecycle_policy = Some(val);
    }
    if let Some(val) =
        deserialize_instance_maintenance_policy_from_query(params, "InstanceMaintenancePolicy")?
    {
        input.instance_maintenance_policy = Some(val);
    }
    if let Some(value) = params.get("LaunchConfigurationName") {
        input.launch_configuration_name = Some(value.to_string());
    }
    if let Some(val) =
        deserialize_launch_template_specification_from_query(params, "LaunchTemplate")?
    {
        input.launch_template = Some(val);
    }
    if let Some(value) = params.get("MaxInstanceLifetime") {
        input.max_instance_lifetime = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxInstanceLifetime: {e}"))?,
        );
    }
    if let Some(value) = params.get("MaxSize") {
        input.max_size = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxSize: {e}"))?,
        );
    }
    if let Some(value) = params.get("MinSize") {
        input.min_size = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MinSize: {e}"))?,
        );
    }
    if let Some(val) =
        deserialize_mixed_instances_policy_from_query(params, "MixedInstancesPolicy")?
    {
        input.mixed_instances_policy = Some(val);
    }
    if let Some(value) = params.get("NewInstancesProtectedFromScaleIn") {
        input.new_instances_protected_from_scale_in = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse NewInstancesProtectedFromScaleIn: {e}"))?,
        );
    }
    if let Some(value) = params.get("PlacementGroup") {
        input.placement_group = Some(value.to_string());
    }
    if let Some(value) = params.get("ServiceLinkedRoleARN") {
        input.service_linked_role_a_r_n = Some(value.to_string());
    }
    if let Some(value) = params.get("SkipZonalShiftValidation") {
        input.skip_zonal_shift_validation = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse SkipZonalShiftValidation: {e}"))?,
        );
    }
    {
        let mut items = Vec::new();
        let list_prefix = "TerminationPolicies".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.termination_policies = Some(TerminationPolicies { items });
        }
    }
    if let Some(value) = params.get("VPCZoneIdentifier") {
        input.v_p_c_zone_identifier = Some(value.to_string());
    }
    Ok(input)
}
