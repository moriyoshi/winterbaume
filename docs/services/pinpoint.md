# winterbaume-pinpoint

Pinpoint service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Pinpoint |
| AWS model | `pinpoint` |
| Protocol | restJson1 |
| winterbaume coverage | 15/122 operations (12.3%) |
| stubs (routed, returns empty/default) | 0/122 operations (0.0%) |
| moto coverage | 12/122 operations (9.8%) |
| floci coverage | 0/122 operations (0.0%) |
| kumo coverage | 0/122 operations (0.0%) |
| Coverage report date | 2026-05-06 |

## Server-mode usage

Start `winterbaume-server` and point the AWS CLI at it:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws pinpoint get-apps
```

## Example

```rust
use aws_sdk_pinpoint::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_pinpoint::PinpointService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(PinpointService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_pinpoint::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_pinpoint::Client::new(&config);

    let resp = client
        .get_apps()
        .send()
        .await
        .expect("get_apps should succeed");
    let count = resp
        .applications_response()
        .map(|r| r.item().len())
        .unwrap_or(0);
    println!("Pinpoint apps: {}", count);
}
```

## Implemented APIs (15)

- `CreateApp`
- `DeleteApp`
- `DeleteEmailChannel`
- `DeleteEventStream`
- `GetApp`
- `GetApplicationSettings`
- `GetApps`
- `GetEmailChannel`
- `GetEventStream`
- `ListTagsForResource`
- `PutEventStream`
- `TagResource`
- `UntagResource`
- `UpdateApplicationSettings`
- `UpdateEmailChannel`

<details><summary>Not yet implemented APIs (107)</summary>

- `CreateCampaign`
- `CreateEmailTemplate`
- `CreateExportJob`
- `CreateImportJob`
- `CreateInAppTemplate`
- `CreateJourney`
- `CreatePushTemplate`
- `CreateRecommenderConfiguration`
- `CreateSegment`
- `CreateSmsTemplate`
- `CreateVoiceTemplate`
- `DeleteAdmChannel`
- `DeleteApnsChannel`
- `DeleteApnsSandboxChannel`
- `DeleteApnsVoipChannel`
- `DeleteApnsVoipSandboxChannel`
- `DeleteBaiduChannel`
- `DeleteCampaign`
- `DeleteEmailTemplate`
- `DeleteEndpoint`
- `DeleteGcmChannel`
- `DeleteInAppTemplate`
- `DeleteJourney`
- `DeletePushTemplate`
- `DeleteRecommenderConfiguration`
- `DeleteSegment`
- `DeleteSmsChannel`
- `DeleteSmsTemplate`
- `DeleteUserEndpoints`
- `DeleteVoiceChannel`
- `DeleteVoiceTemplate`
- `GetAdmChannel`
- `GetApnsChannel`
- `GetApnsSandboxChannel`
- `GetApnsVoipChannel`
- `GetApnsVoipSandboxChannel`
- `GetApplicationDateRangeKpi`
- `GetBaiduChannel`
- `GetCampaign`
- `GetCampaignActivities`
- `GetCampaignDateRangeKpi`
- `GetCampaignVersion`
- `GetCampaignVersions`
- `GetCampaigns`
- `GetChannels`
- `GetEmailTemplate`
- `GetEndpoint`
- `GetExportJob`
- `GetExportJobs`
- `GetGcmChannel`
- `GetImportJob`
- `GetImportJobs`
- `GetInAppMessages`
- `GetInAppTemplate`
- `GetJourney`
- `GetJourneyDateRangeKpi`
- `GetJourneyExecutionActivityMetrics`
- `GetJourneyExecutionMetrics`
- `GetJourneyRunExecutionActivityMetrics`
- `GetJourneyRunExecutionMetrics`
- `GetJourneyRuns`
- `GetPushTemplate`
- `GetRecommenderConfiguration`
- `GetRecommenderConfigurations`
- `GetSegment`
- `GetSegmentExportJobs`
- `GetSegmentImportJobs`
- `GetSegmentVersion`
- `GetSegmentVersions`
- `GetSegments`
- `GetSmsChannel`
- `GetSmsTemplate`
- `GetUserEndpoints`
- `GetVoiceChannel`
- `GetVoiceTemplate`
- `ListJourneys`
- `ListTemplateVersions`
- `ListTemplates`
- `PhoneNumberValidate`
- `PutEvents`
- `RemoveAttributes`
- `SendMessages`
- `SendOTPMessage`
- `SendUsersMessages`
- `UpdateAdmChannel`
- `UpdateApnsChannel`
- `UpdateApnsSandboxChannel`
- `UpdateApnsVoipChannel`
- `UpdateApnsVoipSandboxChannel`
- `UpdateBaiduChannel`
- `UpdateCampaign`
- `UpdateEmailTemplate`
- `UpdateEndpoint`
- `UpdateEndpointsBatch`
- `UpdateGcmChannel`
- `UpdateInAppTemplate`
- `UpdateJourney`
- `UpdateJourneyState`
- `UpdatePushTemplate`
- `UpdateRecommenderConfiguration`
- `UpdateSegment`
- `UpdateSmsChannel`
- `UpdateSmsTemplate`
- `UpdateTemplateActiveVersion`
- `UpdateVoiceChannel`
- `UpdateVoiceTemplate`
- `VerifyOTPMessage`

</details>
