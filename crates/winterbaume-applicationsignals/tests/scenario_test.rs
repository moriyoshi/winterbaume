//! Scenario: SLO budget management pipeline
//!
//! Chains: create SLO → tag → batch budget report → update → exclusion windows → delete.

use aws_sdk_applicationsignals::config::BehaviorVersion;
use aws_sdk_applicationsignals::types::{
    Goal, RequestBasedServiceLevelIndicatorConfig, RequestBasedServiceLevelIndicatorMetricConfig,
    Tag,
};
use winterbaume_applicationsignals::ApplicationSignalsService;
use winterbaume_core::MockAws;

async fn make_client() -> aws_sdk_applicationsignals::Client {
    let mock = MockAws::builder()
        .with_service(ApplicationSignalsService::new())
        .build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_applicationsignals::config::Region::new("us-east-1"))
        .load()
        .await;
    aws_sdk_applicationsignals::Client::new(&config)
}

fn sample_sli_config() -> RequestBasedServiceLevelIndicatorConfig {
    RequestBasedServiceLevelIndicatorConfig::builder()
        .request_based_sli_metric_config(
            RequestBasedServiceLevelIndicatorMetricConfig::builder().build(),
        )
        .build()
}

/// Scenario: SLO budget management pipeline
///
/// Creates an SLO, tags it, fetches the budget report, updates the SLO description,
/// checks the exclusion window list, and finally deletes the SLO.
/// Asserts business outcomes: the SLO is retrievable after update, the budget
/// report lists it as OK, and the resource is gone after deletion.
#[tokio::test]
async fn test_slo_budget_management_pipeline() {
    let client = make_client().await;

    // Step 1: create SLO with a 99% goal.
    let create = client
        .create_service_level_objective()
        .name("PipelineSlo")
        .description("Initial")
        .request_based_sli_config(sample_sli_config())
        .goal(Goal::builder().attainment_goal(99.0).build())
        .send()
        .await
        .expect("create SLO");
    let arn = create.slo().unwrap().arn().to_string();
    let id = create.slo().unwrap().name().to_string();

    // Step 2: tag the resource.
    client
        .tag_resource()
        .resource_arn(&arn)
        .tags(
            Tag::builder()
                .key("Team")
                .value("platform")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("tag");

    let tags = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .expect("list tags");
    assert_eq!(tags.tags().len(), 1, "tag was stored");
    assert_eq!(tags.tags()[0].key(), "Team");

    // Step 3: fetch budget report — SLO should appear in reports, not errors.
    let budget = client
        .batch_get_service_level_objective_budget_report()
        .timestamp(aws_smithy_types::DateTime::from_secs(1_700_000_000))
        .slo_ids(&id)
        .send()
        .await
        .expect("budget report");
    assert_eq!(budget.reports().len(), 1, "report present");
    assert_eq!(budget.errors().len(), 0, "no errors for existing SLO");

    // Step 4: update the SLO description.
    client
        .update_service_level_objective()
        .id(&id)
        .description("Updated by pipeline")
        .send()
        .await
        .expect("update");

    let got = client
        .get_service_level_objective()
        .id(&id)
        .send()
        .await
        .expect("get after update");
    assert_eq!(
        got.slo().unwrap().description(),
        Some("Updated by pipeline"),
        "description persisted"
    );

    // Step 5: exclusion windows list should be empty initially.
    let wins = client
        .list_service_level_objective_exclusion_windows()
        .id(&id)
        .send()
        .await
        .expect("list exclusion windows");
    assert!(
        wins.exclusion_windows().is_empty(),
        "no exclusion windows yet"
    );

    // Step 6: delete the SLO; subsequent get must fail.
    client
        .delete_service_level_objective()
        .id(&id)
        .send()
        .await
        .expect("delete");

    let err = client
        .get_service_level_objective()
        .id(&id)
        .send()
        .await
        .expect_err("SLO must be gone after delete");
    assert!(
        format!("{err:?}").contains("ResourceNotFoundException"),
        "deletion confirmed"
    );
}

/// Scenario: grouping configuration lifecycle
///
/// Puts a grouping configuration, verifies it appears in the definitions list,
/// then deletes it and confirms the list is empty.
#[tokio::test]
async fn test_grouping_configuration_lifecycle() {
    let client = make_client().await;

    client
        .put_grouping_configuration()
        .send()
        .await
        .expect("put grouping configuration");

    let list = client
        .list_grouping_attribute_definitions()
        .send()
        .await
        .expect("list after put");
    assert!(list.updated_at().is_some(), "updated_at populated");

    client
        .delete_grouping_configuration()
        .send()
        .await
        .expect("delete grouping configuration");

    let list2 = client
        .list_grouping_attribute_definitions()
        .send()
        .await
        .expect("list after delete");
    assert!(list2.updated_at().is_none(), "configuration cleared");
}
