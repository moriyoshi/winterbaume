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

#[tokio::test]
async fn test_create_case() {
    let client = make_client().await;

    let resp = client
        .create_case()
        .subject("Test case")
        .communication_body("This is a test case")
        .send()
        .await
        .expect("create_case should succeed");

    let case_id = resp.case_id().expect("should have case_id");
    assert!(case_id.starts_with("case-"));
}

#[tokio::test]
async fn test_create_and_describe_cases() {
    let client = make_client().await;

    let create_resp = client
        .create_case()
        .subject("Describe test")
        .communication_body("Body text")
        .service_code("general-info")
        .severity_code("low")
        .category_code("other")
        .send()
        .await
        .expect("create_case should succeed");

    let case_id = create_resp.case_id().unwrap().to_string();

    let describe_resp = client
        .describe_cases()
        .case_id_list(&case_id)
        .send()
        .await
        .expect("describe_cases should succeed");

    let cases = describe_resp.cases();
    assert_eq!(cases.len(), 1);
    assert_eq!(cases[0].subject().unwrap(), "Describe test");
    assert_eq!(cases[0].status().unwrap(), "opened");
    assert_eq!(cases[0].service_code().unwrap(), "general-info");
    assert_eq!(cases[0].severity_code().unwrap(), "low");
}

#[tokio::test]
async fn test_resolve_case() {
    let client = make_client().await;

    let create_resp = client
        .create_case()
        .subject("Resolve me")
        .communication_body("Please resolve")
        .send()
        .await
        .expect("create_case should succeed");

    let case_id = create_resp.case_id().unwrap().to_string();

    let resolve_resp = client
        .resolve_case()
        .case_id(&case_id)
        .send()
        .await
        .expect("resolve_case should succeed");

    assert_eq!(resolve_resp.initial_case_status().unwrap(), "opened");
    assert_eq!(resolve_resp.final_case_status().unwrap(), "resolved");

    // Verify resolved case is excluded by default
    let describe_resp = client
        .describe_cases()
        .send()
        .await
        .expect("describe_cases should succeed");

    assert_eq!(describe_resp.cases().len(), 0);

    // Verify resolved case is included when requested
    let describe_resp = client
        .describe_cases()
        .include_resolved_cases(true)
        .send()
        .await
        .expect("describe_cases should succeed");

    assert_eq!(describe_resp.cases().len(), 1);
    assert_eq!(describe_resp.cases()[0].status().unwrap(), "resolved");
}

#[tokio::test]
async fn test_describe_services() {
    let client = make_client().await;

    let resp = client
        .describe_services()
        .send()
        .await
        .expect("describe_services should succeed");

    let services = resp.services();
    assert!(!services.is_empty());

    // Check that at least one service has categories
    let first = &services[0];
    assert!(first.code().is_some());
    assert!(first.name().is_some());
    assert!(!first.categories().is_empty());
}

#[tokio::test]
async fn test_describe_services_filtered() {
    let client = make_client().await;

    let resp = client
        .describe_services()
        .service_code_list("general-info")
        .send()
        .await
        .expect("describe_services should succeed");

    let services = resp.services();
    assert_eq!(services.len(), 1);
    assert_eq!(services[0].code().unwrap(), "general-info");
}

// --- Ported from moto test_support.py ---

#[tokio::test]
async fn test_describe_trusted_advisor_checks_returns_checks() {
    // Moto expects 104 checks; we return fewer mock checks.
    // We just assert at least one is returned with proper fields.
    let client = make_client().await;
    let resp = client
        .describe_trusted_advisor_checks()
        .language("en")
        .send()
        .await
        .expect("describe_trusted_advisor_checks should succeed");

    let checks = resp.checks();
    assert!(!checks.is_empty());

    let ids: Vec<&str> = checks.iter().map(|c| c.id()).collect();
    assert!(ids.contains(&"1iG5NDGVre"), "should contain known check id");
}

#[tokio::test]
async fn test_describe_trusted_advisor_checks_returns_check_names() {
    let client = make_client().await;
    let resp = client
        .describe_trusted_advisor_checks()
        .language("en")
        .send()
        .await
        .expect("describe_trusted_advisor_checks should succeed");

    let names: Vec<&str> = resp.checks().iter().map(|c| c.name()).collect();
    assert!(
        names.contains(&"Security Groups - Specific Ports Unrestricted"),
        "should contain known check name"
    );
}

#[tokio::test]
async fn test_refresh_trusted_advisor_check_returns_check_id() {
    let client = make_client().await;
    let resp = client
        .refresh_trusted_advisor_check()
        .check_id("XXXIIIY")
        .send()
        .await
        .expect("refresh_trusted_advisor_check should succeed");

    let status = resp.status().expect("should have status");
    assert_eq!(status.check_id(), "XXXIIIY");
}

#[tokio::test]
async fn test_refresh_trusted_advisor_check_returns_valid_status() {
    let client = make_client().await;
    let resp = client
        .refresh_trusted_advisor_check()
        .check_id("XXXIIIY")
        .send()
        .await
        .expect("refresh_trusted_advisor_check should succeed");

    let possible_statuses = ["none", "enqueued", "processing", "success", "abandoned"];
    let actual_status = resp.status().expect("should have status").status();
    assert!(
        possible_statuses.contains(&actual_status),
        "status should be one of the possible statuses"
    );
}

#[tokio::test]
async fn test_refresh_trusted_advisor_check_cycles_statuses() {
    // Each new client has fresh state, cycles through statuses for the same check_id
    let client = make_client().await;
    let check_id = "XXXIIIY";
    let expected_statuses = ["none", "enqueued", "processing", "success", "abandoned"];

    let mut actual_statuses = Vec::new();
    for _ in 0..expected_statuses.len() {
        let resp = client
            .refresh_trusted_advisor_check()
            .check_id(check_id)
            .send()
            .await
            .expect("refresh_trusted_advisor_check should succeed");
        actual_statuses.push(
            resp.status()
                .expect("should have status")
                .status()
                .to_string(),
        );
    }

    assert_eq!(actual_statuses, expected_statuses);
}

#[tokio::test]
async fn test_refresh_trusted_advisor_check_cycles_after_full_cycle() {
    let client = make_client().await;
    let check_id = "XXXIIIY";
    let cycle_len = 5; // none, enqueued, processing, success, abandoned

    // Complete one full cycle
    for _ in 0..cycle_len {
        client
            .refresh_trusted_advisor_check()
            .check_id(check_id)
            .send()
            .await
            .expect("refresh should succeed");
    }

    // After a full cycle, should wrap back to "none"
    let resp = client
        .refresh_trusted_advisor_check()
        .check_id(check_id)
        .send()
        .await
        .expect("refresh should succeed");
    assert_eq!(resp.status().expect("should have status").status(), "none");
}

#[tokio::test]
async fn test_refresh_trusted_advisor_check_independent_per_check() {
    let client = make_client().await;
    let check_1 = "XXXIIIY";
    let check_2 = "XXXIIIZ";
    let expected = ["none", "enqueued", "processing", "success", "abandoned"];

    let mut statuses_1 = Vec::new();
    let mut statuses_2 = Vec::new();

    for _ in 0..expected.len() {
        let r1 = client
            .refresh_trusted_advisor_check()
            .check_id(check_1)
            .send()
            .await
            .expect("refresh check 1 should succeed");
        statuses_1.push(
            r1.status()
                .expect("should have status")
                .status()
                .to_string(),
        );
    }

    for _ in 0..expected.len() {
        let r2 = client
            .refresh_trusted_advisor_check()
            .check_id(check_2)
            .send()
            .await
            .expect("refresh check 2 should succeed");
        statuses_2.push(
            r2.status()
                .expect("should have status")
                .status()
                .to_string(),
        );
    }

    assert_eq!(statuses_1, expected);
    assert_eq!(statuses_2, expected);
}

#[tokio::test]
async fn test_support_case_created_has_38_char_id() {
    let client = make_client().await;
    let resp = client
        .create_case()
        .subject("test_subject")
        .service_code("test_service_code")
        .severity_code("low")
        .category_code("test_category_code")
        .communication_body("test_communication_body")
        .cc_email_addresses("test_email_cc")
        .language("test_language")
        .send()
        .await
        .expect("create_case should succeed");

    let case_id = resp.case_id().expect("should have case_id");
    assert_eq!(
        case_id.len(),
        38,
        "case_id should be 38 chars, got {case_id:?}"
    );
}

#[tokio::test]
async fn test_support_case_can_be_resolved() {
    let client = make_client().await;
    let create_resp = client
        .create_case()
        .subject("test_subject")
        .service_code("test_service_code")
        .severity_code("low")
        .category_code("test_category_code")
        .communication_body("test_communication_body")
        .cc_email_addresses("test_email_cc")
        .language("test_language")
        .send()
        .await
        .expect("create_case should succeed");

    let case_id = create_resp.case_id().unwrap().to_string();

    let resolve_resp = client
        .resolve_case()
        .case_id(&case_id)
        .send()
        .await
        .expect("resolve_case should succeed");

    let possible_initial_statuses = [
        "opened",
        "pending-customer-action",
        "reopened",
        "unassigned",
        "resolved",
        "work-in-progress",
    ];
    let initial = resolve_resp.initial_case_status().unwrap();
    assert!(
        possible_initial_statuses.contains(&initial),
        "initial status '{initial}' should be valid"
    );
    assert_eq!(resolve_resp.final_case_status().unwrap(), "resolved");
}

#[tokio::test]
async fn test_support_case_fields_preserved() {
    let client = make_client().await;
    let create_resp = client
        .create_case()
        .subject("test_subject")
        .service_code("test_service_code")
        .severity_code("low")
        .category_code("test_category_code")
        .communication_body("test_communication_body")
        .cc_email_addresses("test_email_cc")
        .language("test_language")
        .send()
        .await
        .expect("create_case should succeed");

    let case_id = create_resp.case_id().unwrap().to_string();

    let describe_resp = client
        .describe_cases()
        .case_id_list(&case_id)
        .include_resolved_cases(true)
        .send()
        .await
        .expect("describe_cases should succeed");

    let case = &describe_resp.cases()[0];
    assert_eq!(case.subject().unwrap(), "test_subject");
    assert_eq!(case.service_code().unwrap(), "test_service_code");
    assert_eq!(case.severity_code().unwrap(), "low");
    assert_eq!(case.category_code().unwrap(), "test_category_code");
    assert_eq!(case.language().unwrap(), "test_language");
}

#[tokio::test]
async fn test_support_case_cc_emails_preserved() {
    let client = make_client().await;
    let create_resp = client
        .create_case()
        .subject("test_subject")
        .service_code("test_service_code")
        .severity_code("low")
        .category_code("test_category_code")
        .communication_body("test_communication_body")
        .cc_email_addresses("test_email_cc")
        .language("test_language")
        .send()
        .await
        .expect("create_case should succeed");

    let case_id = create_resp.case_id().unwrap().to_string();

    let describe_resp = client
        .describe_cases()
        .case_id_list(&case_id)
        .include_resolved_cases(true)
        .send()
        .await
        .expect("describe_cases should succeed");

    let case = &describe_resp.cases()[0];
    let cc_emails = case.cc_email_addresses();
    assert_eq!(cc_emails, ["test_email_cc"]);
}

#[tokio::test]
async fn test_describe_cases_without_params_returns_empty_then_case() {
    let client = make_client().await;

    let describe_resp = client
        .describe_cases()
        .send()
        .await
        .expect("describe_cases should succeed");
    assert_eq!(describe_resp.cases().len(), 0);

    client
        .create_case()
        .subject("test_subject")
        .service_code("test_service_code")
        .severity_code("low")
        .category_code("test_category_code")
        .communication_body("test_communication_body")
        .cc_email_addresses("test_email_cc")
        .language("test_language")
        .send()
        .await
        .expect("create_case should succeed");

    let describe_resp = client
        .describe_cases()
        .send()
        .await
        .expect("describe_cases should succeed");
    assert_eq!(describe_resp.cases().len(), 1);
}

#[tokio::test]
async fn test_multiple_cases_can_be_described() {
    let client = make_client().await;

    let r1 = client
        .create_case()
        .subject("test_subject")
        .service_code("test_service_code")
        .severity_code("low")
        .category_code("test_category_code")
        .communication_body("test_communication_body")
        .language("test_language")
        .send()
        .await
        .expect("create case 1 should succeed");

    let r2 = client
        .create_case()
        .subject("test_subject")
        .service_code("test_service_code")
        .severity_code("low")
        .category_code("test_category_code")
        .communication_body("test_communication_body")
        .language("test_language")
        .send()
        .await
        .expect("create case 2 should succeed");

    let id1 = r1.case_id().unwrap().to_string();
    let id2 = r2.case_id().unwrap().to_string();

    let describe_resp = client
        .describe_cases()
        .case_id_list(&id1)
        .case_id_list(&id2)
        .include_resolved_cases(true)
        .send()
        .await
        .expect("describe_cases should succeed");

    let cases = describe_resp.cases();
    assert_eq!(cases.len(), 2);
    let case_ids: Vec<&str> = cases.iter().map(|c| c.case_id().unwrap()).collect();
    assert!(case_ids.contains(&id1.as_str()));
    assert!(case_ids.contains(&id2.as_str()));
}

#[tokio::test]
async fn test_describe_cases_include_resolved_defaults_false() {
    let client = make_client().await;

    let r = client
        .create_case()
        .subject("test_subject")
        .communication_body("body")
        .send()
        .await
        .expect("create_case should succeed");

    let case_id = r.case_id().unwrap().to_string();

    // By default, open cases are included
    let describe_resp = client
        .describe_cases()
        .case_id_list(&case_id)
        .send()
        .await
        .expect("describe_cases should succeed");
    assert_eq!(describe_resp.cases().len(), 1);

    // Resolve it
    client
        .resolve_case()
        .case_id(&case_id)
        .send()
        .await
        .expect("resolve_case should succeed");

    // Now without include_resolved, it should be absent
    let describe_resp = client
        .describe_cases()
        .case_id_list(&case_id)
        .send()
        .await
        .expect("describe_cases should succeed");
    assert_eq!(describe_resp.cases().len(), 0);

    // With include_resolved=true, it should appear
    let describe_resp = client
        .describe_cases()
        .case_id_list(&case_id)
        .include_resolved_cases(true)
        .send()
        .await
        .expect("describe_cases should succeed");
    assert_eq!(describe_resp.cases().len(), 1);
    assert_eq!(describe_resp.cases()[0].case_id().unwrap(), case_id);
}

// ============================================================================
// Tests derived from AWS documentation: AWS Support
// ============================================================================

#[tokio::test]
async fn test_create_case_missing_subject() {
    let client = make_client().await;
    let err = client
        .create_case()
        .subject("")
        .communication_body("body")
        .send()
        .await
        .expect_err("empty subject must be rejected as InvalidParameterValue");
    let svc = err.into_service_error();
    assert!(
        format!("{svc:?}").contains("subject is required"),
        "expected subject-is-required error, got {svc:?}",
    );
}

#[tokio::test]
async fn test_create_case_default_fields() {
    let client = make_client().await;

    // Create with only required fields
    let create_resp = client
        .create_case()
        .subject("Default fields test")
        .communication_body("Testing defaults")
        .send()
        .await
        .expect("create_case should succeed");

    let case_id = create_resp.case_id().unwrap().to_string();

    let describe_resp = client
        .describe_cases()
        .case_id_list(&case_id)
        .send()
        .await
        .expect("describe_cases should succeed");

    let case = &describe_resp.cases()[0];
    // Verify default field values when not specified
    assert_eq!(
        case.service_code().unwrap(),
        "general-info",
        "default service_code should be general-info"
    );
    assert_eq!(
        case.severity_code().unwrap(),
        "normal",
        "default severity_code should be normal"
    );
    assert_eq!(
        case.category_code().unwrap(),
        "other",
        "default category_code should be other"
    );
    assert_eq!(
        case.language().unwrap(),
        "en",
        "default language should be en"
    );
}

#[tokio::test]
async fn test_describe_cases_display_id_present() {
    let client = make_client().await;

    let create_resp = client
        .create_case()
        .subject("Display ID test")
        .communication_body("Checking displayId")
        .send()
        .await
        .expect("create_case should succeed");

    let case_id = create_resp.case_id().unwrap().to_string();

    let describe_resp = client
        .describe_cases()
        .case_id_list(&case_id)
        .send()
        .await
        .expect("describe_cases should succeed");

    let case = &describe_resp.cases()[0];
    let display_id = case.display_id().unwrap_or_default();
    assert!(
        !display_id.is_empty(),
        "displayId should be non-empty, got: {display_id:?}"
    );
}

#[tokio::test]
async fn test_describe_cases_submitted_by_present() {
    let client = make_client().await;

    let create_resp = client
        .create_case()
        .subject("SubmittedBy test")
        .communication_body("Checking submittedBy")
        .send()
        .await
        .expect("create_case should succeed");

    let case_id = create_resp.case_id().unwrap().to_string();

    let describe_resp = client
        .describe_cases()
        .case_id_list(&case_id)
        .send()
        .await
        .expect("describe_cases should succeed");

    let case = &describe_resp.cases()[0];
    let submitted_by = case.submitted_by().unwrap_or_default();
    assert!(!submitted_by.is_empty(), "submittedBy should be non-empty");
}

#[tokio::test]
async fn test_describe_cases_time_created_present() {
    let client = make_client().await;

    let create_resp = client
        .create_case()
        .subject("TimeCreated test")
        .communication_body("Checking timeCreated")
        .send()
        .await
        .expect("create_case should succeed");

    let case_id = create_resp.case_id().unwrap().to_string();

    let describe_resp = client
        .describe_cases()
        .case_id_list(&case_id)
        .send()
        .await
        .expect("describe_cases should succeed");

    let case = &describe_resp.cases()[0];
    let time_created = case.time_created().unwrap_or_default();
    assert!(!time_created.is_empty(), "timeCreated should be non-empty");
}

#[tokio::test]
async fn test_resolve_case_not_found() {
    let client = make_client().await;

    let err = client
        .resolve_case()
        .case_id("case-000000000000-2026-000000000000000")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("CaseIdNotFound"),
        "Expected CaseIdNotFound error, got: {err_str}"
    );
}

#[tokio::test]
async fn test_describe_services_empty_list_returns_all() {
    let client = make_client().await;

    let resp = client
        .describe_services()
        .send()
        .await
        .expect("describe_services should succeed");

    // Should return multiple services
    assert!(
        resp.services().len() >= 2,
        "should return at least 2 services"
    );
}

#[tokio::test]
async fn test_describe_services_multiple_codes() {
    let client = make_client().await;

    let resp = client
        .describe_services()
        .service_code_list("amazon-elastic-compute-cloud-linux")
        .service_code_list("amazon-simple-storage-service")
        .send()
        .await
        .expect("describe_services should succeed");

    let services = resp.services();
    assert_eq!(
        services.len(),
        2,
        "should return exactly 2 services when filtering by 2 known codes"
    );

    let codes: Vec<&str> = services
        .iter()
        .map(|s| s.code().unwrap_or_default())
        .collect();
    assert!(codes.contains(&"amazon-elastic-compute-cloud-linux"));
    assert!(codes.contains(&"amazon-simple-storage-service"));
}

#[tokio::test]
async fn test_describe_services_unknown_code() {
    let client = make_client().await;

    let resp = client
        .describe_services()
        .service_code_list("nonexistent-service-xyz")
        .send()
        .await
        .expect("describe_services should succeed");

    assert_eq!(
        resp.services().len(),
        0,
        "unknown service code should return empty list"
    );
}

#[tokio::test]
async fn test_describe_services_categories_have_code_and_name() {
    let client = make_client().await;

    let resp = client
        .describe_services()
        .send()
        .await
        .expect("describe_services should succeed");

    for service in resp.services() {
        for category in service.categories() {
            let code = category.code().unwrap_or_default();
            let name = category.name().unwrap_or_default();
            assert!(
                !code.is_empty(),
                "category code should not be empty for service {:?}",
                service.code()
            );
            assert!(
                !name.is_empty(),
                "category name should not be empty for service {:?}",
                service.code()
            );
        }
    }
}

#[tokio::test]
async fn test_describe_trusted_advisor_check_full_fields() {
    let client = make_client().await;

    let resp = client
        .describe_trusted_advisor_checks()
        .language("en")
        .send()
        .await
        .expect("describe_trusted_advisor_checks should succeed");

    for check in resp.checks() {
        assert!(!check.id().is_empty(), "check id should not be empty");
        assert!(!check.name().is_empty(), "check name should not be empty");
        assert!(
            !check.category().is_empty(),
            "check category should not be empty"
        );
        assert!(
            !check.description().is_empty(),
            "check description should not be empty"
        );
        assert!(
            !check.metadata().is_empty(),
            "check metadata should not be empty"
        );
    }
}

#[tokio::test]
async fn test_refresh_trusted_advisor_check_millis_field() {
    let client = make_client().await;

    let resp = client
        .refresh_trusted_advisor_check()
        .check_id("1iG5NDGVre")
        .send()
        .await
        .expect("refresh_trusted_advisor_check should succeed");

    let status = resp.status().expect("should have status");
    assert!(
        status.millis_until_next_refreshable() > 0,
        "millisUntilNextRefreshable should be > 0, got: {}",
        status.millis_until_next_refreshable()
    );
}

#[tokio::test]
async fn test_full_case_lifecycle() {
    let client = make_client().await;

    // Step 1: Create
    let create_resp = client
        .create_case()
        .subject("Lifecycle test")
        .communication_body("Full lifecycle test case")
        .service_code("amazon-simple-storage-service")
        .severity_code("low")
        .category_code("general-info")
        .send()
        .await
        .expect("create_case should succeed");

    let case_id = create_resp.case_id().unwrap().to_string();
    assert!(
        case_id.starts_with("case-"),
        "case_id should start with 'case-'"
    );

    // Step 2: Describe (open case is visible)
    let describe_resp = client
        .describe_cases()
        .case_id_list(&case_id)
        .send()
        .await
        .expect("describe_cases should succeed");

    assert_eq!(describe_resp.cases().len(), 1);
    assert_eq!(describe_resp.cases()[0].status().unwrap(), "opened");
    assert_eq!(
        describe_resp.cases()[0].subject().unwrap(),
        "Lifecycle test"
    );

    // Step 3: Resolve
    let resolve_resp = client
        .resolve_case()
        .case_id(&case_id)
        .send()
        .await
        .expect("resolve_case should succeed");

    assert_eq!(resolve_resp.initial_case_status().unwrap(), "opened");
    assert_eq!(resolve_resp.final_case_status().unwrap(), "resolved");

    // Step 4: Describe without include_resolved (should be empty)
    let describe_resp = client
        .describe_cases()
        .case_id_list(&case_id)
        .send()
        .await
        .expect("describe_cases should succeed");
    assert_eq!(
        describe_resp.cases().len(),
        0,
        "resolved case should not appear without include_resolved=true"
    );

    // Step 5: Describe with include_resolved=true (should appear)
    let describe_resp = client
        .describe_cases()
        .case_id_list(&case_id)
        .include_resolved_cases(true)
        .send()
        .await
        .expect("describe_cases should succeed");
    assert_eq!(describe_resp.cases().len(), 1);
    assert_eq!(describe_resp.cases()[0].status().unwrap(), "resolved");
}
