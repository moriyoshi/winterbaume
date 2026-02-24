use std::sync::{Arc, Mutex};

use aws_sdk_savingsplans::config::BehaviorVersion;
use winterbaume_core::{MockAws, StatefulService};
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

#[tokio::test]
async fn test_create_describe_savings_plan() {
    let client = make_client().await;
    let create = client
        .create_savings_plan()
        .savings_plan_offering_id("offering-1")
        .commitment("10.00")
        .upfront_payment_amount("1000.00")
        .send()
        .await
        .expect("create");
    let id = create.savings_plan_id().expect("id").to_string();
    assert!(id.starts_with("sp-"));

    let resp = client
        .describe_savings_plans()
        .savings_plan_ids(&id)
        .send()
        .await
        .expect("describe");
    assert_eq!(resp.savings_plans().len(), 1);
    let plan = &resp.savings_plans()[0];
    assert_eq!(plan.savings_plan_id(), Some(id.as_str()));
    assert_eq!(plan.commitment(), Some("10.00"));
}

#[tokio::test]
async fn test_describe_rates_for_unknown_plan() {
    let client = make_client().await;
    let err = client
        .describe_savings_plan_rates()
        .savings_plan_id("missing")
        .send()
        .await
        .expect_err("missing");
    assert!(format!("{err:?}").contains("ResourceNotFoundException"));
}

#[tokio::test]
async fn test_describe_offerings_empty() {
    let client = make_client().await;
    let resp = client
        .describe_savings_plans_offerings()
        .send()
        .await
        .expect("describe-offerings");
    assert_eq!(resp.search_results().len(), 0);
}

#[tokio::test]
async fn test_return_savings_plan() {
    let client = make_client().await;
    let create = client
        .create_savings_plan()
        .savings_plan_offering_id("offering-1")
        .commitment("5.00")
        .send()
        .await
        .expect("create");
    let id = create.savings_plan_id().unwrap().to_string();

    let ret = client
        .return_savings_plan()
        .savings_plan_id(&id)
        .send()
        .await
        .expect("return");
    assert_eq!(ret.savings_plan_id(), Some(id.as_str()));
}

#[tokio::test]
async fn test_tag_lifecycle() {
    let client = make_client().await;
    let arn = "arn:aws:savingsplans::123:savingsplan/seed";
    client
        .tag_resource()
        .resource_arn(arn)
        .tags("Env", "prod")
        .send()
        .await
        .expect("tag");

    let resp = client
        .list_tags_for_resource()
        .resource_arn(arn)
        .send()
        .await
        .expect("list");
    assert_eq!(resp.tags().map(|m| m.len()).unwrap_or(0), 1);

    client
        .untag_resource()
        .resource_arn(arn)
        .tag_keys("Env")
        .send()
        .await
        .expect("untag");
}

#[tokio::test]
async fn test_state_change_listener_fires() {
    let svc = SavingsPlansService::new();
    let events: Arc<Mutex<Vec<(String, String)>>> = Arc::new(Mutex::new(vec![]));
    let events2 = Arc::clone(&events);
    svc.notifier().subscribe(move |account_id, region, _view| {
        events2
            .lock()
            .unwrap()
            .push((account_id.to_string(), region.to_string()));
    });
    svc.restore("123456789012", "us-east-1", Default::default())
        .await
        .expect("restore");
    assert_eq!(events.lock().unwrap().len(), 1);
}
