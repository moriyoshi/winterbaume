/// Scenario: Savings plan lifecycle — purchase, tag, describe, return.
///
/// Exercises the full durable resource lifecycle for a Compute Savings Plan:
/// create, tag, describe-with-filter, and return.  Chains 4+ operations and
/// asserts business outcomes rather than per-API shapes.
use aws_sdk_savingsplans::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_savingsplans::SavingsPlansService;

async fn make_client() -> aws_sdk_savingsplans::Client {
    let mock = MockAws::builder()
        .with_service(SavingsPlansService::new())
        .build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_savingsplans::config::Region::new("us-east-1"))
        .load()
        .await;
    aws_sdk_savingsplans::Client::new(&config)
}

/// Scenario: purchase a savings plan, tag it, verify tags appear on describe, then return it.
#[tokio::test]
async fn test_plan_purchase_tag_and_return() {
    let client = make_client().await;

    // Step 1: Create a Compute Savings Plan.
    let create = client
        .create_savings_plan()
        .savings_plan_offering_id("offering-compute-1yr")
        .commitment("25.00")
        .upfront_payment_amount("9000.00")
        .send()
        .await
        .expect("create savings plan");
    let plan_id = create.savings_plan_id().expect("plan id").to_string();
    assert!(plan_id.starts_with("sp-"), "id should start with sp-");

    // Step 2: Look up the plan by ID and retrieve its ARN.
    let describe = client
        .describe_savings_plans()
        .savings_plan_ids(&plan_id)
        .send()
        .await
        .expect("describe after create");
    assert_eq!(describe.savings_plans().len(), 1);
    let plan = &describe.savings_plans()[0];
    let arn = plan.savings_plan_arn().expect("arn").to_string();
    assert!(arn.contains(&plan_id));

    // Step 3: Tag the plan.
    client
        .tag_resource()
        .resource_arn(&arn)
        .tags("Team", "finance")
        .tags("CostCentre", "cc-42")
        .send()
        .await
        .expect("tag resource");

    // Step 4: Verify tags are present.
    let tags_resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .expect("list tags");
    let tags = tags_resp.tags().expect("tags map");
    assert_eq!(tags.get("Team").map(String::as_str), Some("finance"));
    assert_eq!(tags.get("CostCentre").map(String::as_str), Some("cc-42"));

    // Step 5: Return the plan; status should change to returned.
    let ret = client
        .return_savings_plan()
        .savings_plan_id(&plan_id)
        .send()
        .await
        .expect("return savings plan");
    assert_eq!(ret.savings_plan_id(), Some(plan_id.as_str()));

    // Step 6: Describe with state filter confirms the plan is now returned.
    let after_return = client
        .describe_savings_plans()
        .states(aws_sdk_savingsplans::types::SavingsPlanState::Returned)
        .send()
        .await
        .expect("describe returned");
    let returned_ids: Vec<&str> = after_return
        .savings_plans()
        .iter()
        .filter_map(|p| p.savings_plan_id())
        .collect();
    assert!(
        returned_ids.contains(&plan_id.as_str()),
        "returned plan should appear in returned-state filter"
    );
}

/// Scenario: multiple plans, filter by state, untag.
#[tokio::test]
async fn test_multi_plan_filter_and_untag() {
    let client = make_client().await;

    // Create two plans.
    let id_a = client
        .create_savings_plan()
        .savings_plan_offering_id("off-a")
        .commitment("5.00")
        .send()
        .await
        .expect("create A")
        .savings_plan_id()
        .expect("id A")
        .to_string();

    let id_b = client
        .create_savings_plan()
        .savings_plan_offering_id("off-b")
        .commitment("10.00")
        .send()
        .await
        .expect("create B")
        .savings_plan_id()
        .expect("id B")
        .to_string();

    // Both are in payment-pending state initially.
    let all = client
        .describe_savings_plans()
        .send()
        .await
        .expect("describe all");
    assert!(all.savings_plans().len() >= 2);

    // Tag plan A.
    let arn_a = client
        .describe_savings_plans()
        .savings_plan_ids(&id_a)
        .send()
        .await
        .expect("describe A")
        .savings_plans()[0]
        .savings_plan_arn()
        .expect("arn A")
        .to_string();

    client
        .tag_resource()
        .resource_arn(&arn_a)
        .tags("Env", "test")
        .send()
        .await
        .expect("tag A");

    // Untag plan A.
    client
        .untag_resource()
        .resource_arn(&arn_a)
        .tag_keys("Env")
        .send()
        .await
        .expect("untag A");

    let tags_after = client
        .list_tags_for_resource()
        .resource_arn(&arn_a)
        .send()
        .await
        .expect("list after untag");
    let count = tags_after.tags().map(|m| m.len()).unwrap_or(0);
    assert_eq!(count, 0, "all tags should be removed from plan A");

    // Filter describe by IDs returns only those two.
    let filtered = client
        .describe_savings_plans()
        .savings_plan_ids(&id_a)
        .savings_plan_ids(&id_b)
        .send()
        .await
        .expect("filtered describe");
    let ids: Vec<&str> = filtered
        .savings_plans()
        .iter()
        .filter_map(|p| p.savings_plan_id())
        .collect();
    assert!(ids.contains(&id_a.as_str()));
    assert!(ids.contains(&id_b.as_str()));
}
