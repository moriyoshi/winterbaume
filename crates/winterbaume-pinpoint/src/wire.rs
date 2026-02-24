//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-pinpoint

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

#[allow(unused_imports)]
use http::header::HeaderName;
use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize response for restJson protocol.
pub fn serialize_create_app_response(result: &CreateAppResponse) -> MockResponse {
    let status = 201_u16;
    let body =
        serde_json::to_string(&result.application_response).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_campaign_response(result: &CreateCampaignResponse) -> MockResponse {
    let status = 201_u16;
    let body =
        serde_json::to_string(&result.campaign_response).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_email_template_response(
    result: &CreateEmailTemplateResponse,
) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(&result.create_template_message_body)
        .unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_export_job_response(result: &CreateExportJobResponse) -> MockResponse {
    let status = 202_u16;
    let body =
        serde_json::to_string(&result.export_job_response).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_import_job_response(result: &CreateImportJobResponse) -> MockResponse {
    let status = 201_u16;
    let body =
        serde_json::to_string(&result.import_job_response).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_in_app_template_response(
    result: &CreateInAppTemplateResponse,
) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(&result.template_create_message_body)
        .unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_journey_response(result: &CreateJourneyResponse) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(&result.journey_response).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_push_template_response(
    result: &CreatePushTemplateResponse,
) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(&result.create_template_message_body)
        .unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_recommender_configuration_response(
    result: &CreateRecommenderConfigurationResponse,
) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(&result.recommender_configuration_response)
        .unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_segment_response(result: &CreateSegmentResponse) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(&result.segment_response).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_sms_template_response(result: &CreateSmsTemplateResponse) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(&result.create_template_message_body)
        .unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_voice_template_response(
    result: &CreateVoiceTemplateResponse,
) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(&result.create_template_message_body)
        .unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_adm_channel_response(result: &DeleteAdmChannelResponse) -> MockResponse {
    let status = 200_u16;
    let body =
        serde_json::to_string(&result.a_d_m_channel_response).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_apns_channel_response(result: &DeleteApnsChannelResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(&result.a_p_n_s_channel_response)
        .unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_apns_sandbox_channel_response(
    result: &DeleteApnsSandboxChannelResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(&result.a_p_n_s_sandbox_channel_response)
        .unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_apns_voip_channel_response(
    result: &DeleteApnsVoipChannelResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(&result.a_p_n_s_voip_channel_response)
        .unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_apns_voip_sandbox_channel_response(
    result: &DeleteApnsVoipSandboxChannelResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(&result.a_p_n_s_voip_sandbox_channel_response)
        .unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_app_response(result: &DeleteAppResponse) -> MockResponse {
    let status = 200_u16;
    let body =
        serde_json::to_string(&result.application_response).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_baidu_channel_response(
    result: &DeleteBaiduChannelResponse,
) -> MockResponse {
    let status = 200_u16;
    let body =
        serde_json::to_string(&result.baidu_channel_response).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_campaign_response(result: &DeleteCampaignResponse) -> MockResponse {
    let status = 200_u16;
    let body =
        serde_json::to_string(&result.campaign_response).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_email_channel_response(
    result: &DeleteEmailChannelResponse,
) -> MockResponse {
    let status = 200_u16;
    let body =
        serde_json::to_string(&result.email_channel_response).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_email_template_response(
    result: &DeleteEmailTemplateResponse,
) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(&result.message_body).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_endpoint_response(result: &DeleteEndpointResponse) -> MockResponse {
    let status = 202_u16;
    let body =
        serde_json::to_string(&result.endpoint_response).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_event_stream_response(result: &DeleteEventStreamResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(&result.event_stream).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_gcm_channel_response(result: &DeleteGcmChannelResponse) -> MockResponse {
    let status = 200_u16;
    let body =
        serde_json::to_string(&result.g_c_m_channel_response).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_in_app_template_response(
    result: &DeleteInAppTemplateResponse,
) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(&result.message_body).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_journey_response(result: &DeleteJourneyResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(&result.journey_response).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_push_template_response(
    result: &DeletePushTemplateResponse,
) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(&result.message_body).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_recommender_configuration_response(
    result: &DeleteRecommenderConfigurationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(&result.recommender_configuration_response)
        .unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_segment_response(result: &DeleteSegmentResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(&result.segment_response).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_sms_channel_response(result: &DeleteSmsChannelResponse) -> MockResponse {
    let status = 200_u16;
    let body =
        serde_json::to_string(&result.s_m_s_channel_response).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_sms_template_response(result: &DeleteSmsTemplateResponse) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(&result.message_body).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_user_endpoints_response(
    result: &DeleteUserEndpointsResponse,
) -> MockResponse {
    let status = 202_u16;
    let body =
        serde_json::to_string(&result.endpoints_response).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_voice_channel_response(
    result: &DeleteVoiceChannelResponse,
) -> MockResponse {
    let status = 200_u16;
    let body =
        serde_json::to_string(&result.voice_channel_response).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_voice_template_response(
    result: &DeleteVoiceTemplateResponse,
) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(&result.message_body).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_adm_channel_response(result: &GetAdmChannelResponse) -> MockResponse {
    let status = 200_u16;
    let body =
        serde_json::to_string(&result.a_d_m_channel_response).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_apns_channel_response(result: &GetApnsChannelResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(&result.a_p_n_s_channel_response)
        .unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_apns_sandbox_channel_response(
    result: &GetApnsSandboxChannelResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(&result.a_p_n_s_sandbox_channel_response)
        .unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_apns_voip_channel_response(
    result: &GetApnsVoipChannelResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(&result.a_p_n_s_voip_channel_response)
        .unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_apns_voip_sandbox_channel_response(
    result: &GetApnsVoipSandboxChannelResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(&result.a_p_n_s_voip_sandbox_channel_response)
        .unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_app_response(result: &GetAppResponse) -> MockResponse {
    let status = 200_u16;
    let body =
        serde_json::to_string(&result.application_response).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_application_date_range_kpi_response(
    result: &GetApplicationDateRangeKpiResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(&result.application_date_range_kpi_response)
        .unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_application_settings_response(
    result: &GetApplicationSettingsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(&result.application_settings_resource)
        .unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_apps_response(result: &GetAppsResponse) -> MockResponse {
    let status = 200_u16;
    let body =
        serde_json::to_string(&result.applications_response).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_baidu_channel_response(result: &GetBaiduChannelResponse) -> MockResponse {
    let status = 200_u16;
    let body =
        serde_json::to_string(&result.baidu_channel_response).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_campaign_response(result: &GetCampaignResponse) -> MockResponse {
    let status = 200_u16;
    let body =
        serde_json::to_string(&result.campaign_response).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_campaign_activities_response(
    result: &GetCampaignActivitiesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body =
        serde_json::to_string(&result.activities_response).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_campaign_date_range_kpi_response(
    result: &GetCampaignDateRangeKpiResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(&result.campaign_date_range_kpi_response)
        .unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_campaign_version_response(
    result: &GetCampaignVersionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body =
        serde_json::to_string(&result.campaign_response).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_campaign_versions_response(
    result: &GetCampaignVersionsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body =
        serde_json::to_string(&result.campaigns_response).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_campaigns_response(result: &GetCampaignsResponse) -> MockResponse {
    let status = 200_u16;
    let body =
        serde_json::to_string(&result.campaigns_response).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_channels_response(result: &GetChannelsResponse) -> MockResponse {
    let status = 200_u16;
    let body =
        serde_json::to_string(&result.channels_response).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_email_channel_response(result: &GetEmailChannelResponse) -> MockResponse {
    let status = 200_u16;
    let body =
        serde_json::to_string(&result.email_channel_response).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_email_template_response(result: &GetEmailTemplateResponse) -> MockResponse {
    let status = 200_u16;
    let body =
        serde_json::to_string(&result.email_template_response).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_endpoint_response(result: &GetEndpointResponse) -> MockResponse {
    let status = 200_u16;
    let body =
        serde_json::to_string(&result.endpoint_response).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_event_stream_response(result: &GetEventStreamResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(&result.event_stream).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_export_job_response(result: &GetExportJobResponse) -> MockResponse {
    let status = 200_u16;
    let body =
        serde_json::to_string(&result.export_job_response).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_export_jobs_response(result: &GetExportJobsResponse) -> MockResponse {
    let status = 200_u16;
    let body =
        serde_json::to_string(&result.export_jobs_response).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_gcm_channel_response(result: &GetGcmChannelResponse) -> MockResponse {
    let status = 200_u16;
    let body =
        serde_json::to_string(&result.g_c_m_channel_response).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_import_job_response(result: &GetImportJobResponse) -> MockResponse {
    let status = 200_u16;
    let body =
        serde_json::to_string(&result.import_job_response).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_import_jobs_response(result: &GetImportJobsResponse) -> MockResponse {
    let status = 200_u16;
    let body =
        serde_json::to_string(&result.import_jobs_response).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_in_app_messages_response(result: &GetInAppMessagesResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(&result.in_app_messages_response)
        .unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_in_app_template_response(result: &GetInAppTemplateResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(&result.in_app_template_response)
        .unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_journey_response(result: &GetJourneyResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(&result.journey_response).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_journey_date_range_kpi_response(
    result: &GetJourneyDateRangeKpiResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(&result.journey_date_range_kpi_response)
        .unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_journey_execution_activity_metrics_response(
    result: &GetJourneyExecutionActivityMetricsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(&result.journey_execution_activity_metrics_response)
        .unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_journey_execution_metrics_response(
    result: &GetJourneyExecutionMetricsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(&result.journey_execution_metrics_response)
        .unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_journey_run_execution_activity_metrics_response(
    result: &GetJourneyRunExecutionActivityMetricsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(&result.journey_run_execution_activity_metrics_response)
        .unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_journey_run_execution_metrics_response(
    result: &GetJourneyRunExecutionMetricsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(&result.journey_run_execution_metrics_response)
        .unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_journey_runs_response(result: &GetJourneyRunsResponse) -> MockResponse {
    let status = 200_u16;
    let body =
        serde_json::to_string(&result.journey_runs_response).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_push_template_response(result: &GetPushTemplateResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(&result.push_notification_template_response)
        .unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_recommender_configuration_response(
    result: &GetRecommenderConfigurationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(&result.recommender_configuration_response)
        .unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_recommender_configurations_response(
    result: &GetRecommenderConfigurationsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(&result.list_recommender_configurations_response)
        .unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_segment_response(result: &GetSegmentResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(&result.segment_response).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_segment_export_jobs_response(
    result: &GetSegmentExportJobsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body =
        serde_json::to_string(&result.export_jobs_response).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_segment_import_jobs_response(
    result: &GetSegmentImportJobsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body =
        serde_json::to_string(&result.import_jobs_response).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_segment_version_response(result: &GetSegmentVersionResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(&result.segment_response).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_segment_versions_response(
    result: &GetSegmentVersionsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body =
        serde_json::to_string(&result.segments_response).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_segments_response(result: &GetSegmentsResponse) -> MockResponse {
    let status = 200_u16;
    let body =
        serde_json::to_string(&result.segments_response).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_sms_channel_response(result: &GetSmsChannelResponse) -> MockResponse {
    let status = 200_u16;
    let body =
        serde_json::to_string(&result.s_m_s_channel_response).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_sms_template_response(result: &GetSmsTemplateResponse) -> MockResponse {
    let status = 200_u16;
    let body =
        serde_json::to_string(&result.s_m_s_template_response).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_user_endpoints_response(result: &GetUserEndpointsResponse) -> MockResponse {
    let status = 200_u16;
    let body =
        serde_json::to_string(&result.endpoints_response).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_voice_channel_response(result: &GetVoiceChannelResponse) -> MockResponse {
    let status = 200_u16;
    let body =
        serde_json::to_string(&result.voice_channel_response).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_voice_template_response(result: &GetVoiceTemplateResponse) -> MockResponse {
    let status = 200_u16;
    let body =
        serde_json::to_string(&result.voice_template_response).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_journeys_response(result: &ListJourneysResponse) -> MockResponse {
    let status = 200_u16;
    let body =
        serde_json::to_string(&result.journeys_response).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_tags_for_resource_response(
    result: &ListTagsForResourceResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(&result.tags_model).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_template_versions_response(
    result: &ListTemplateVersionsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(&result.template_versions_response)
        .unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_templates_response(result: &ListTemplatesResponse) -> MockResponse {
    let status = 200_u16;
    let body =
        serde_json::to_string(&result.templates_response).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_phone_number_validate_response(
    result: &PhoneNumberValidateResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(&result.number_validate_response)
        .unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_put_event_stream_response(result: &PutEventStreamResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(&result.event_stream).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_put_events_response(result: &PutEventsResponse) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(&result.events_response).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_remove_attributes_response(result: &RemoveAttributesResponse) -> MockResponse {
    let status = 200_u16;
    let body =
        serde_json::to_string(&result.attributes_resource).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_send_messages_response(result: &SendMessagesResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(&result.message_response).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_send_o_t_p_message_response(result: &SendOTPMessageResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(&result.message_response).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_send_users_messages_response(result: &SendUsersMessagesResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(&result.send_users_message_response)
        .unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize void response for restJson protocol.
pub fn serialize_tag_resource_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize void response for restJson protocol.
pub fn serialize_untag_resource_response() -> MockResponse {
    MockResponse::rest_json(204, "{}")
}

/// Serialize response for restJson protocol.
pub fn serialize_update_adm_channel_response(result: &UpdateAdmChannelResponse) -> MockResponse {
    let status = 200_u16;
    let body =
        serde_json::to_string(&result.a_d_m_channel_response).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_apns_channel_response(result: &UpdateApnsChannelResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(&result.a_p_n_s_channel_response)
        .unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_apns_sandbox_channel_response(
    result: &UpdateApnsSandboxChannelResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(&result.a_p_n_s_sandbox_channel_response)
        .unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_apns_voip_channel_response(
    result: &UpdateApnsVoipChannelResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(&result.a_p_n_s_voip_channel_response)
        .unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_apns_voip_sandbox_channel_response(
    result: &UpdateApnsVoipSandboxChannelResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(&result.a_p_n_s_voip_sandbox_channel_response)
        .unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_application_settings_response(
    result: &UpdateApplicationSettingsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(&result.application_settings_resource)
        .unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_baidu_channel_response(
    result: &UpdateBaiduChannelResponse,
) -> MockResponse {
    let status = 200_u16;
    let body =
        serde_json::to_string(&result.baidu_channel_response).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_campaign_response(result: &UpdateCampaignResponse) -> MockResponse {
    let status = 200_u16;
    let body =
        serde_json::to_string(&result.campaign_response).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_email_channel_response(
    result: &UpdateEmailChannelResponse,
) -> MockResponse {
    let status = 200_u16;
    let body =
        serde_json::to_string(&result.email_channel_response).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_email_template_response(
    result: &UpdateEmailTemplateResponse,
) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(&result.message_body).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_endpoint_response(result: &UpdateEndpointResponse) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(&result.message_body).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_endpoints_batch_response(
    result: &UpdateEndpointsBatchResponse,
) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(&result.message_body).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_gcm_channel_response(result: &UpdateGcmChannelResponse) -> MockResponse {
    let status = 200_u16;
    let body =
        serde_json::to_string(&result.g_c_m_channel_response).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_in_app_template_response(
    result: &UpdateInAppTemplateResponse,
) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(&result.message_body).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_journey_response(result: &UpdateJourneyResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(&result.journey_response).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_journey_state_response(
    result: &UpdateJourneyStateResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(&result.journey_response).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_push_template_response(
    result: &UpdatePushTemplateResponse,
) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(&result.message_body).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_recommender_configuration_response(
    result: &UpdateRecommenderConfigurationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(&result.recommender_configuration_response)
        .unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_segment_response(result: &UpdateSegmentResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(&result.segment_response).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_sms_channel_response(result: &UpdateSmsChannelResponse) -> MockResponse {
    let status = 200_u16;
    let body =
        serde_json::to_string(&result.s_m_s_channel_response).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_sms_template_response(result: &UpdateSmsTemplateResponse) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(&result.message_body).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_template_active_version_response(
    result: &UpdateTemplateActiveVersionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(&result.message_body).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_voice_channel_response(
    result: &UpdateVoiceChannelResponse,
) -> MockResponse {
    let status = 200_u16;
    let body =
        serde_json::to_string(&result.voice_channel_response).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_voice_template_response(
    result: &UpdateVoiceTemplateResponse,
) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(&result.message_body).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_verify_o_t_p_message_response(result: &VerifyOTPMessageResponse) -> MockResponse {
    let status = 200_u16;
    let body =
        serde_json::to_string(&result.verification_response).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}
