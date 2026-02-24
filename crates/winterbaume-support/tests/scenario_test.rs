/// Scenario: Full support case lifecycle — open, inspect, resolve, re-check
///
/// Chains CreateCase → DescribeCases (open) → ResolveCase → DescribeCases (with/without
/// include_resolved) to verify the complete durable lifecycle of a support case in the mock.
use aws_sdk_support::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_support::SupportService;

async fn make_client() -> aws_sdk_support::Client {
    let mock = MockAws::builder()
        .with_service(SupportService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_support::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_support::Client::new(&config)
}

/// Scenario: A developer files a support case, verifies it is visible, resolves it,
/// then confirms it no longer appears in default describe results but is retrievable
/// when include_resolved is set.
#[tokio::test]
async fn test_support_case_open_inspect_resolve_recheck() {
    let client = make_client().await;

    // Step 1: Create a case
    let create_resp = client
        .create_case()
        .subject("Production outage - latency spike")
        .communication_body("We are seeing elevated p99 latency since 14:00 UTC.")
        .service_code("amazon-elastic-compute-cloud-linux")
        .severity_code("high")
        .category_code("instance-issue")
        .cc_email_addresses("oncall@example.com")
        .language("en")
        .send()
        .await
        .expect("create_case should succeed");

    let case_id = create_resp
        .case_id()
        .expect("should have case_id")
        .to_string();
    assert!(
        case_id.starts_with("case-"),
        "case_id format should start with 'case-'"
    );

    // Step 2: Describe — case should be visible in open list
    let open_resp = client
        .describe_cases()
        .case_id_list(&case_id)
        .send()
        .await
        .expect("describe_cases should succeed");

    assert_eq!(
        open_resp.cases().len(),
        1,
        "opened case must appear in default describe"
    );
    let c = &open_resp.cases()[0];
    assert_eq!(c.status().unwrap(), "opened");
    assert_eq!(c.subject().unwrap(), "Production outage - latency spike");
    assert_eq!(
        c.service_code().unwrap(),
        "amazon-elastic-compute-cloud-linux"
    );
    assert_eq!(c.severity_code().unwrap(), "high");
    assert_eq!(c.language().unwrap(), "en");

    // Step 3: Resolve the case
    let resolve_resp = client
        .resolve_case()
        .case_id(&case_id)
        .send()
        .await
        .expect("resolve_case should succeed");

    assert_eq!(resolve_resp.initial_case_status().unwrap(), "opened");
    assert_eq!(resolve_resp.final_case_status().unwrap(), "resolved");

    // Step 4: Default describe should no longer show the resolved case
    let after_resolve_resp = client
        .describe_cases()
        .case_id_list(&case_id)
        .send()
        .await
        .expect("describe_cases should succeed");

    assert_eq!(
        after_resolve_resp.cases().len(),
        0,
        "resolved case must not appear without include_resolved=true"
    );

    // Step 5: With include_resolved=true, the case must reappear with status resolved
    let include_resolved_resp = client
        .describe_cases()
        .case_id_list(&case_id)
        .include_resolved_cases(true)
        .send()
        .await
        .expect("describe_cases with include_resolved should succeed");

    assert_eq!(
        include_resolved_resp.cases().len(),
        1,
        "resolved case must appear when include_resolved=true"
    );
    assert_eq!(
        include_resolved_resp.cases()[0].status().unwrap(),
        "resolved"
    );
}

/// Scenario: Trusted Advisor refresh status cycle — request refreshes for the same check
/// multiple times and confirm the status cycles through all known states before wrapping.
#[tokio::test]
async fn test_trusted_advisor_refresh_cycle_and_per_check_independence() {
    let client = make_client().await;

    // Step 1: Discover available checks
    let checks_resp = client
        .describe_trusted_advisor_checks()
        .language("en")
        .send()
        .await
        .expect("describe_trusted_advisor_checks should succeed");

    let checks = checks_resp.checks();
    assert!(!checks.is_empty(), "must return at least one check");

    let check_a = checks[0].id();
    let check_b = if checks.len() > 1 {
        checks[1].id()
    } else {
        "OTHER_CHECK_ID"
    };

    let statuses = ["none", "enqueued", "processing", "success", "abandoned"];

    // Step 2: Cycle check_a through all five states
    let mut observed_a: Vec<String> = Vec::new();
    for _ in 0..statuses.len() {
        let r = client
            .refresh_trusted_advisor_check()
            .check_id(check_a)
            .send()
            .await
            .expect("refresh should succeed");
        observed_a.push(r.status().expect("status present").status().to_string());
    }
    assert_eq!(
        observed_a, statuses,
        "check_a status should cycle through all states in order"
    );

    // Step 3: check_b should still start from the beginning (independent counter)
    let r_b = client
        .refresh_trusted_advisor_check()
        .check_id(check_b)
        .send()
        .await
        .expect("refresh check_b should succeed");

    assert_eq!(
        r_b.status().expect("status present").status(),
        "none",
        "check_b counter must be independent from check_a"
    );

    // Step 4: After a full cycle, check_a should wrap back to "none"
    let wrap_resp = client
        .refresh_trusted_advisor_check()
        .check_id(check_a)
        .send()
        .await
        .expect("wrap refresh should succeed");

    assert_eq!(
        wrap_resp.status().expect("status present").status(),
        "none",
        "after full cycle check_a must wrap back to 'none'"
    );
}
