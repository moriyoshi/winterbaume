/// Scenario: App + Event Stream + Tag lifecycle
///
/// Chains: CreateApp → PutEventStream → TagResource → ListTagsForResource →
///         GetEventStream → UntagResource → DeleteEventStream → DeleteApp
use aws_sdk_pinpoint::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_pinpoint::PinpointService;

async fn make_client(region: &str) -> aws_sdk_pinpoint::Client {
    let mock = MockAws::builder()
        .with_service(PinpointService::new())
        .build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_pinpoint::config::Region::new(region.to_string()))
        .load()
        .await;
    aws_sdk_pinpoint::Client::new(&config)
}

/// Scenario: full app + event-stream + tag workflow.
///
/// Steps:
/// 1. CreateApp — create the application resource.
/// 2. PutEventStream — attach a Kinesis stream.
/// 3. TagResource — label the app.
/// 4. ListTagsForResource — verify tags are present.
/// 5. GetEventStream — verify stream is configured.
/// 6. UntagResource — remove one tag.
/// 7. DeleteEventStream — detach the stream.
/// 8. DeleteApp — clean up.
#[tokio::test]
async fn test_app_event_stream_tag_workflow() {
    let client = make_client("eu-west-1").await;

    // 1. Create app
    let create_resp = client
        .create_app()
        .create_application_request(
            aws_sdk_pinpoint::types::CreateApplicationRequest::builder()
                .name("scenario-app")
                .build(),
        )
        .send()
        .await
        .expect("CreateApp should succeed");
    let app = create_resp.application_response().unwrap();
    let app_id = app.id().unwrap().to_string();
    let app_arn = app.arn().unwrap().to_string();
    assert!(app_arn.contains("eu-west-1"), "ARN should contain region");

    // 2. Attach event stream
    client
        .put_event_stream()
        .application_id(&app_id)
        .write_event_stream(
            aws_sdk_pinpoint::types::WriteEventStream::builder()
                .destination_stream_arn("arn:aws:kinesis:eu-west-1:123456789012:stream/events")
                .role_arn("arn:aws:iam::123456789012:role/pinpoint-kinesis")
                .build(),
        )
        .send()
        .await
        .expect("PutEventStream should succeed");

    // 3. Tag the app
    client
        .tag_resource()
        .resource_arn(&app_arn)
        .tags_model(
            aws_sdk_pinpoint::types::TagsModel::builder()
                .tags("env", "test")
                .tags("team", "platform")
                .build(),
        )
        .send()
        .await
        .expect("TagResource should succeed");

    // 4. Verify tags
    let tags_resp = client
        .list_tags_for_resource()
        .resource_arn(&app_arn)
        .send()
        .await
        .expect("ListTagsForResource should succeed");
    let tags = tags_resp.tags_model().unwrap().tags().unwrap();
    assert_eq!(tags.get("env").map(String::as_str), Some("test"));
    assert_eq!(tags.get("team").map(String::as_str), Some("platform"));

    // 5. Verify event stream
    let es_resp = client
        .get_event_stream()
        .application_id(&app_id)
        .send()
        .await
        .expect("GetEventStream should succeed");
    let es = es_resp.event_stream().unwrap();
    assert_eq!(
        es.destination_stream_arn(),
        Some("arn:aws:kinesis:eu-west-1:123456789012:stream/events")
    );

    // 6. Remove one tag
    client
        .untag_resource()
        .resource_arn(&app_arn)
        .tag_keys("team")
        .send()
        .await
        .expect("UntagResource should succeed");
    let tags_after = client
        .list_tags_for_resource()
        .resource_arn(&app_arn)
        .send()
        .await
        .unwrap();
    let remaining = tags_after.tags_model().unwrap().tags().unwrap();
    assert!(remaining.contains_key("env"));
    assert!(
        !remaining.contains_key("team"),
        "team tag should be removed"
    );

    // 7. Delete event stream
    client
        .delete_event_stream()
        .application_id(&app_id)
        .send()
        .await
        .expect("DeleteEventStream should succeed");

    // 8. Delete app — business outcome: no app remains
    client
        .delete_app()
        .application_id(&app_id)
        .send()
        .await
        .expect("DeleteApp should succeed");

    let err = client
        .get_app()
        .application_id(&app_id)
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("NotFoundException"),
        "App should be gone after delete, got: {err_str}"
    );
}

/// Scenario: Application settings drive campaign lifecycle.
///
/// Steps:
/// 1. CreateApp
/// 2. UpdateApplicationSettings — set campaign limits and hook.
/// 3. GetApplicationSettings — assert limits and hook persist.
/// 4. UpdateApplicationSettings again — change limits.
/// 5. GetApplicationSettings — assert updated values.
/// 6. DeleteApp.
#[tokio::test]
async fn test_application_settings_campaign_workflow() {
    let client = make_client("ap-southeast-1").await;

    // 1. Create app
    let create_resp = client
        .create_app()
        .create_application_request(
            aws_sdk_pinpoint::types::CreateApplicationRequest::builder()
                .name("settings-scenario-app")
                .build(),
        )
        .send()
        .await
        .expect("CreateApp should succeed");
    let app_id = create_resp
        .application_response()
        .unwrap()
        .id()
        .unwrap()
        .to_string();

    // 2. Set initial settings
    client
        .update_application_settings()
        .application_id(&app_id)
        .write_application_settings_request(
            aws_sdk_pinpoint::types::WriteApplicationSettingsRequest::builder()
                .campaign_hook(
                    aws_sdk_pinpoint::types::CampaignHook::builder()
                        .lambda_function_name(
                            "arn:aws:lambda:ap-southeast-1:123456789012:function/hook",
                        )
                        .build(),
                )
                .limits(
                    aws_sdk_pinpoint::types::CampaignLimits::builder()
                        .daily(100)
                        .total(1000)
                        .build(),
                )
                .build(),
        )
        .send()
        .await
        .expect("UpdateApplicationSettings (initial) should succeed");

    // 3. Verify initial settings
    let settings_resp = client
        .get_application_settings()
        .application_id(&app_id)
        .send()
        .await
        .expect("GetApplicationSettings should succeed");
    let settings = settings_resp.application_settings_resource().unwrap();
    assert_eq!(settings.limits().unwrap().daily(), Some(100));
    assert_eq!(settings.limits().unwrap().total(), Some(1000));
    assert!(settings.campaign_hook().is_some());

    // 4. Update limits
    client
        .update_application_settings()
        .application_id(&app_id)
        .write_application_settings_request(
            aws_sdk_pinpoint::types::WriteApplicationSettingsRequest::builder()
                .limits(
                    aws_sdk_pinpoint::types::CampaignLimits::builder()
                        .daily(200)
                        .build(),
                )
                .build(),
        )
        .send()
        .await
        .expect("UpdateApplicationSettings (update) should succeed");

    // 5. Verify updated limits
    let updated_resp = client
        .get_application_settings()
        .application_id(&app_id)
        .send()
        .await
        .expect("GetApplicationSettings after update should succeed");
    let updated = updated_resp.application_settings_resource().unwrap();
    assert_eq!(updated.limits().unwrap().daily(), Some(200));

    // 6. Delete app
    client
        .delete_app()
        .application_id(&app_id)
        .send()
        .await
        .expect("DeleteApp should succeed");
}
