use aws_sdk_route53domains::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_route53domains::Route53DomainsService;

async fn make_client() -> aws_sdk_route53domains::Client {
    let mock = MockAws::builder()
        .with_service(Route53DomainsService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_route53domains::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_route53domains::Client::new(&config)
}

fn make_contact() -> aws_sdk_route53domains::types::ContactDetail {
    aws_sdk_route53domains::types::ContactDetail::builder()
        .first_name("John")
        .last_name("Doe")
        .email("john@example.com")
        .phone_number("+1.5555555555")
        .contact_type(aws_sdk_route53domains::types::ContactType::Person)
        .country_code(aws_sdk_route53domains::types::CountryCode::Us)
        .city("Seattle")
        .state("WA")
        .zip_code("98101")
        .address_line1("123 Main St")
        .build()
}

#[tokio::test]
async fn test_register_and_get_domain() {
    let client = make_client().await;
    let contact = make_contact();

    let register_resp = client
        .register_domain()
        .domain_name("example.com")
        .duration_in_years(1)
        .admin_contact(contact.clone())
        .registrant_contact(contact.clone())
        .tech_contact(contact)
        .send()
        .await
        .expect("register_domain should succeed");

    assert!(register_resp.operation_id().is_some());

    let detail = client
        .get_domain_detail()
        .domain_name("example.com")
        .send()
        .await
        .expect("get_domain_detail should succeed");

    assert_eq!(detail.domain_name(), Some("example.com"));
    assert_eq!(detail.auto_renew(), Some(true));
    assert_eq!(detail.admin_privacy(), Some(true));
}

#[tokio::test]
async fn test_list_domains_empty() {
    let client = make_client().await;

    let resp = client
        .list_domains()
        .send()
        .await
        .expect("list_domains should succeed on empty state");

    assert_eq!(resp.domains().len(), 0);
}

#[tokio::test]
async fn test_list_domains_with_entries() {
    let client = make_client().await;
    let contact = make_contact();

    for name in ["foo.com", "bar.com"] {
        client
            .register_domain()
            .domain_name(name)
            .duration_in_years(1)
            .admin_contact(contact.clone())
            .registrant_contact(contact.clone())
            .tech_contact(contact.clone())
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_domains()
        .send()
        .await
        .expect("list_domains should succeed");

    assert_eq!(resp.domains().len(), 2);
}

#[tokio::test]
async fn test_check_domain_availability_available() {
    let client = make_client().await;

    let resp = client
        .check_domain_availability()
        .domain_name("available-domain.com")
        .send()
        .await
        .expect("check_domain_availability should succeed");

    assert_eq!(
        resp.availability(),
        Some(&aws_sdk_route53domains::types::DomainAvailability::Available)
    );
}

#[tokio::test]
async fn test_check_domain_availability_unavailable() {
    let client = make_client().await;
    let contact = make_contact();

    client
        .register_domain()
        .domain_name("taken.com")
        .duration_in_years(1)
        .admin_contact(contact.clone())
        .registrant_contact(contact.clone())
        .tech_contact(contact)
        .send()
        .await
        .unwrap();

    let resp = client
        .check_domain_availability()
        .domain_name("taken.com")
        .send()
        .await
        .expect("check_domain_availability should succeed");

    assert_eq!(
        resp.availability(),
        Some(&aws_sdk_route53domains::types::DomainAvailability::Unavailable)
    );
}

#[tokio::test]
async fn test_get_nonexistent_domain() {
    let client = make_client().await;

    let result = client
        .get_domain_detail()
        .domain_name("nonexistent.com")
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_register_duplicate_domain() {
    let client = make_client().await;
    let contact = make_contact();

    client
        .register_domain()
        .domain_name("dupe.com")
        .duration_in_years(1)
        .admin_contact(contact.clone())
        .registrant_contact(contact.clone())
        .tech_contact(contact.clone())
        .send()
        .await
        .unwrap();

    let result = client
        .register_domain()
        .domain_name("dupe.com")
        .duration_in_years(1)
        .admin_contact(contact.clone())
        .registrant_contact(contact.clone())
        .tech_contact(contact)
        .send()
        .await;

    assert!(result.is_err());
}

// ============================================================================
// Tests derived from AWS documentation: Route 53 Domains
// ============================================================================

#[tokio::test]
async fn test_register_domain_returns_operation_id() {
    let client = make_client().await;
    let contact = make_contact();

    let resp = client
        .register_domain()
        .domain_name("opid-check.com")
        .duration_in_years(1)
        .admin_contact(contact.clone())
        .registrant_contact(contact.clone())
        .tech_contact(contact)
        .send()
        .await
        .expect("register_domain should succeed");

    let op_id = resp.operation_id().unwrap_or_default();
    assert!(!op_id.is_empty(), "operation_id must be non-empty");
}

#[tokio::test]
async fn test_get_domain_detail_fields() {
    let client = make_client().await;
    let contact = make_contact();

    client
        .register_domain()
        .domain_name("fields-check.com")
        .duration_in_years(1)
        .admin_contact(contact.clone())
        .registrant_contact(contact.clone())
        .tech_contact(contact)
        .send()
        .await
        .unwrap();

    let detail = client
        .get_domain_detail()
        .domain_name("fields-check.com")
        .send()
        .await
        .expect("get_domain_detail should succeed");

    // Domain name and core flags
    assert_eq!(detail.domain_name(), Some("fields-check.com"));
    assert_eq!(detail.auto_renew(), Some(true));

    // Privacy flags default to true
    assert_eq!(detail.admin_privacy(), Some(true));
    assert_eq!(detail.registrant_privacy(), Some(true));
    assert_eq!(detail.tech_privacy(), Some(true));

    // Registrar info
    assert!(detail.registrar_name().is_some());
    assert!(!detail.registrar_name().unwrap_or_default().is_empty());
    assert!(detail.who_is_server().is_some());
    assert!(detail.registrar_url().is_some());
    assert!(detail.abuse_contact_email().is_some());

    // Timestamps
    assert!(detail.creation_date().is_some());
    assert!(detail.updated_date().is_some());
    assert!(detail.expiration_date().is_some());

    // Nameservers
    assert!(
        !detail.nameservers().is_empty(),
        "nameservers must be populated"
    );

    // Status list
    assert!(
        !detail.status_list().is_empty(),
        "status_list must be non-empty"
    );
}

#[tokio::test]
async fn test_register_with_auto_renew_false() {
    let client = make_client().await;
    let contact = make_contact();

    client
        .register_domain()
        .domain_name("no-renew.com")
        .duration_in_years(1)
        .auto_renew(false)
        .admin_contact(contact.clone())
        .registrant_contact(contact.clone())
        .tech_contact(contact)
        .send()
        .await
        .expect("register_domain should succeed");

    let detail = client
        .get_domain_detail()
        .domain_name("no-renew.com")
        .send()
        .await
        .expect("get_domain_detail should succeed");

    assert_eq!(detail.auto_renew(), Some(false));
}

#[tokio::test]
async fn test_register_privacy_protect_false() {
    let client = make_client().await;
    let contact = make_contact();

    client
        .register_domain()
        .domain_name("no-privacy.com")
        .duration_in_years(1)
        .privacy_protect_admin_contact(false)
        .privacy_protect_registrant_contact(false)
        .privacy_protect_tech_contact(false)
        .admin_contact(contact.clone())
        .registrant_contact(contact.clone())
        .tech_contact(contact)
        .send()
        .await
        .expect("register_domain should succeed");

    let detail = client
        .get_domain_detail()
        .domain_name("no-privacy.com")
        .send()
        .await
        .expect("get_domain_detail should succeed");

    assert_eq!(detail.admin_privacy(), Some(false));
    assert_eq!(detail.registrant_privacy(), Some(false));
    assert_eq!(detail.tech_privacy(), Some(false));
}

#[tokio::test]
async fn test_list_domains_summary_fields() {
    let client = make_client().await;
    let contact = make_contact();

    client
        .register_domain()
        .domain_name("summary-fields.com")
        .duration_in_years(1)
        .admin_contact(contact.clone())
        .registrant_contact(contact.clone())
        .tech_contact(contact)
        .send()
        .await
        .unwrap();

    let resp = client
        .list_domains()
        .send()
        .await
        .expect("list_domains should succeed");

    assert_eq!(resp.domains().len(), 1);
    let entry = &resp.domains()[0];
    assert_eq!(entry.domain_name(), Some("summary-fields.com"));
    assert!(entry.auto_renew().is_some());
    assert!(entry.transfer_lock().is_some());
    assert!(entry.expiry().is_some());
}

#[tokio::test]
async fn test_delete_nonexistent_domain() {
    let client = make_client().await;

    let result = client
        .delete_domain()
        .domain_name("not-there.com")
        .send()
        .await;

    assert!(
        result.is_err(),
        "delete_domain on nonexistent domain should fail"
    );
}

#[tokio::test]
async fn test_domain_lifecycle() {
    let client = make_client().await;
    let contact = make_contact();

    // Register
    let reg = client
        .register_domain()
        .domain_name("lifecycle.com")
        .duration_in_years(2)
        .admin_contact(contact.clone())
        .registrant_contact(contact.clone())
        .tech_contact(contact)
        .send()
        .await
        .expect("register_domain should succeed");
    assert!(reg.operation_id().is_some());

    // Get detail
    let detail = client
        .get_domain_detail()
        .domain_name("lifecycle.com")
        .send()
        .await
        .expect("get_domain_detail should succeed");
    assert_eq!(detail.domain_name(), Some("lifecycle.com"));

    // Check availability — should be UNAVAILABLE
    let avail = client
        .check_domain_availability()
        .domain_name("lifecycle.com")
        .send()
        .await
        .expect("check_domain_availability should succeed");
    assert_eq!(
        avail.availability(),
        Some(&aws_sdk_route53domains::types::DomainAvailability::Unavailable)
    );

    // Delete
    let del = client
        .delete_domain()
        .domain_name("lifecycle.com")
        .send()
        .await
        .expect("delete_domain should succeed");
    assert!(del.operation_id().is_some());

    // Verify gone
    let gone = client
        .get_domain_detail()
        .domain_name("lifecycle.com")
        .send()
        .await;
    assert!(
        gone.is_err(),
        "get_domain_detail should fail after deletion"
    );

    // Availability should be AVAILABLE again
    let avail2 = client
        .check_domain_availability()
        .domain_name("lifecycle.com")
        .send()
        .await
        .expect("check_domain_availability should succeed after deletion");
    assert_eq!(
        avail2.availability(),
        Some(&aws_sdk_route53domains::types::DomainAvailability::Available)
    );
}

#[tokio::test]
async fn test_delete_domain_returns_operation_id() {
    let client = make_client().await;
    let contact = make_contact();

    client
        .register_domain()
        .domain_name("del-opid.com")
        .duration_in_years(1)
        .admin_contact(contact.clone())
        .registrant_contact(contact.clone())
        .tech_contact(contact)
        .send()
        .await
        .unwrap();

    let del_resp = client
        .delete_domain()
        .domain_name("del-opid.com")
        .send()
        .await
        .expect("delete_domain should succeed");

    let op_id = del_resp.operation_id().unwrap_or_default();
    assert!(!op_id.is_empty(), "delete operation_id must be non-empty");
}

#[tokio::test]
async fn test_get_domain_contact_details() {
    let client = make_client().await;
    let contact = make_contact();

    client
        .register_domain()
        .domain_name("contact-check.com")
        .duration_in_years(1)
        .admin_contact(contact.clone())
        .registrant_contact(contact.clone())
        .tech_contact(contact)
        .send()
        .await
        .unwrap();

    let detail = client
        .get_domain_detail()
        .domain_name("contact-check.com")
        .send()
        .await
        .expect("get_domain_detail should succeed");

    let admin = detail
        .admin_contact()
        .expect("admin_contact must be present");
    assert_eq!(admin.first_name(), Some("John"));
    assert_eq!(admin.last_name(), Some("Doe"));
    assert_eq!(admin.email(), Some("john@example.com"));
    assert_eq!(admin.phone_number(), Some("+1.5555555555"));

    let registrant = detail
        .registrant_contact()
        .expect("registrant_contact must be present");
    assert_eq!(registrant.first_name(), Some("John"));

    let tech = detail.tech_contact().expect("tech_contact must be present");
    assert_eq!(tech.first_name(), Some("John"));
}

#[tokio::test]
async fn test_list_domains_after_delete() {
    let client = make_client().await;
    let contact = make_contact();

    for name in ["keep-me.com", "remove-me.com"] {
        client
            .register_domain()
            .domain_name(name)
            .duration_in_years(1)
            .admin_contact(contact.clone())
            .registrant_contact(contact.clone())
            .tech_contact(contact.clone())
            .send()
            .await
            .unwrap();
    }

    client
        .delete_domain()
        .domain_name("remove-me.com")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_domains()
        .send()
        .await
        .expect("list_domains should succeed");

    assert_eq!(resp.domains().len(), 1);
    assert_eq!(resp.domains()[0].domain_name(), Some("keep-me.com"));
}

#[tokio::test]
async fn test_delete_domain() {
    let client = make_client().await;
    let contact = make_contact();

    client
        .register_domain()
        .domain_name("delete-me.com")
        .duration_in_years(1)
        .admin_contact(contact.clone())
        .registrant_contact(contact.clone())
        .tech_contact(contact)
        .send()
        .await
        .unwrap();

    let delete_resp = client
        .delete_domain()
        .domain_name("delete-me.com")
        .send()
        .await
        .expect("delete_domain should succeed");

    assert!(delete_resp.operation_id().is_some());

    // Verify the domain is gone
    let result = client
        .get_domain_detail()
        .domain_name("delete-me.com")
        .send()
        .await;

    assert!(result.is_err());
}

// ============================================================================
// Tests derived from AWS documentation: Route 53 Domains (2026-03-28)
// ============================================================================

// Test 1: RegisterDomain missing required DomainName returns error
#[tokio::test]
async fn test_register_domain_missing_domain_name() {
    let client = make_client().await;
    let contact = make_contact();

    let result = client
        .register_domain()
        .set_domain_name(None)
        .duration_in_years(1)
        .admin_contact(contact.clone())
        .registrant_contact(contact.clone())
        .tech_contact(contact)
        .send()
        .await;

    assert!(
        result.is_err(),
        "register_domain without DomainName should fail"
    );
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("InvalidInput") || err_str.contains("DomainName"),
        "Expected InvalidInput error for missing DomainName, got: {err_str}"
    );
}

// Test 2: RegisterDomain missing required AdminContact returns error
#[tokio::test]
async fn test_register_domain_missing_admin_contact() {
    let client = make_client().await;
    let contact = make_contact();

    let result = client
        .register_domain()
        .domain_name("no-admin.com")
        .duration_in_years(1)
        .set_admin_contact(None)
        .registrant_contact(contact.clone())
        .tech_contact(contact)
        .send()
        .await;

    assert!(
        result.is_err(),
        "register_domain without AdminContact should fail"
    );
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("InvalidInput") || err_str.contains("AdminContact"),
        "Expected InvalidInput error for missing AdminContact, got: {err_str}"
    );
}

// Test 3: RegisterDomain missing required RegistrantContact returns error
#[tokio::test]
async fn test_register_domain_missing_registrant_contact() {
    let client = make_client().await;
    let contact = make_contact();

    let result = client
        .register_domain()
        .domain_name("no-registrant.com")
        .duration_in_years(1)
        .admin_contact(contact.clone())
        .set_registrant_contact(None)
        .tech_contact(contact)
        .send()
        .await;

    assert!(
        result.is_err(),
        "register_domain without RegistrantContact should fail"
    );
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("InvalidInput") || err_str.contains("RegistrantContact"),
        "Expected InvalidInput error for missing RegistrantContact, got: {err_str}"
    );
}

// Test 4: RegisterDomain missing required TechContact returns error
#[tokio::test]
async fn test_register_domain_missing_tech_contact() {
    let client = make_client().await;
    let contact = make_contact();

    let result = client
        .register_domain()
        .domain_name("no-tech.com")
        .duration_in_years(1)
        .admin_contact(contact.clone())
        .registrant_contact(contact)
        .set_tech_contact(None)
        .send()
        .await;

    assert!(
        result.is_err(),
        "register_domain without TechContact should fail"
    );
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("InvalidInput") || err_str.contains("TechContact"),
        "Expected InvalidInput error for missing TechContact, got: {err_str}"
    );
}

// Test 5: GetDomainDetail for nonexistent domain returns meaningful error message
#[tokio::test]
async fn test_get_domain_detail_not_found_error_message() {
    let client = make_client().await;

    let err = client
        .get_domain_detail()
        .domain_name("totally-unknown-domain.com")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("InvalidInput")
            || err_str.contains("not found")
            || err_str.contains("NotFound"),
        "Expected not-found error for nonexistent domain, got: {err_str}"
    );
}

// Test 6: DurationInYears is reflected in the expiration date
#[tokio::test]
async fn test_duration_in_years_affects_expiration() {
    let client = make_client().await;
    let contact = make_contact();

    client
        .register_domain()
        .domain_name("two-year.com")
        .duration_in_years(2)
        .admin_contact(contact.clone())
        .registrant_contact(contact.clone())
        .tech_contact(contact)
        .send()
        .await
        .expect("register_domain with duration_in_years=2 should succeed");

    let detail = client
        .get_domain_detail()
        .domain_name("two-year.com")
        .send()
        .await
        .expect("get_domain_detail should succeed");

    let expiry = detail
        .expiration_date()
        .expect("expiration_date must be set");
    let now_secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    let one_year_secs: i64 = 365 * 24 * 3600;
    // Expiration should be more than 1 year from now (we registered for 2 years)
    assert!(
        expiry.secs() > now_secs + one_year_secs,
        "Expiration date for 2-year registration should be more than 1 year from now: expiry={}, now={}",
        expiry.secs(),
        now_secs
    );
}

// Test 7: TransferLock defaults to true for newly registered domains
#[tokio::test]
async fn test_domain_transfer_lock_default() {
    let client = make_client().await;
    let contact = make_contact();

    client
        .register_domain()
        .domain_name("transfer-lock-test.com")
        .duration_in_years(1)
        .admin_contact(contact.clone())
        .registrant_contact(contact.clone())
        .tech_contact(contact)
        .send()
        .await
        .expect("register_domain should succeed");

    let resp = client
        .list_domains()
        .send()
        .await
        .expect("list_domains should succeed");

    let domain = resp
        .domains()
        .iter()
        .find(|d| d.domain_name() == Some("transfer-lock-test.com"))
        .expect("registered domain must appear in list");

    assert_eq!(
        domain.transfer_lock(),
        Some(true),
        "TransferLock should be true by default"
    );
}

// Test 8: CheckDomainAvailability missing required DomainName returns error
#[tokio::test]
async fn test_check_domain_availability_missing_domain_name() {
    let client = make_client().await;

    let result = client
        .check_domain_availability()
        .set_domain_name(None)
        .send()
        .await;

    assert!(
        result.is_err(),
        "check_domain_availability without DomainName should fail"
    );
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("InvalidInput") || err_str.contains("DomainName"),
        "Expected InvalidInput error for missing DomainName, got: {err_str}"
    );
}

// Test 9: ListDomains with max_items returns at most max_items domains
#[tokio::test]
async fn test_list_domains_max_items() {
    let client = make_client().await;
    let contact = make_contact();

    for name in ["alpha.com", "beta.com", "gamma.com"] {
        client
            .register_domain()
            .domain_name(name)
            .duration_in_years(1)
            .admin_contact(contact.clone())
            .registrant_contact(contact.clone())
            .tech_contact(contact.clone())
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_domains()
        .max_items(2)
        .send()
        .await
        .expect("list_domains with max_items should succeed");

    assert!(
        resp.domains().len() <= 2,
        "list_domains with max_items=2 should return at most 2 entries, got {}",
        resp.domains().len()
    );
}
