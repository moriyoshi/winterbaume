/// Scenario tests for winterbaume-route53domains.
///
/// Each test covers an end-to-end use-case scenario chaining 3+ operations and
/// asserts business outcomes rather than per-API return shapes.
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
        .first_name("Jane")
        .last_name("Smith")
        .email("jane@example.com")
        .phone_number("+1.4155550123")
        .contact_type(aws_sdk_route53domains::types::ContactType::Person)
        .country_code(aws_sdk_route53domains::types::CountryCode::Us)
        .city("San Francisco")
        .state("CA")
        .zip_code("94105")
        .address_line1("1 Market Street")
        .build()
}

/// Scenario: domain portfolio management.
///
/// A user registers two domains, verifies their details and availability, then
/// deletes one and confirms the portfolio reflects the change.
#[tokio::test]
async fn test_domain_portfolio_management() {
    // Scenario: domain portfolio management
    let client = make_client().await;
    let contact = make_contact();

    // Step 1: Register the first domain
    let reg1 = client
        .register_domain()
        .domain_name("portfolio-alpha.com")
        .duration_in_years(1)
        .admin_contact(contact.clone())
        .registrant_contact(contact.clone())
        .tech_contact(contact.clone())
        .send()
        .await
        .expect("register first domain should succeed");
    assert!(
        reg1.operation_id().is_some(),
        "first registration must return an operation id"
    );

    // Step 2: Register the second domain
    let reg2 = client
        .register_domain()
        .domain_name("portfolio-beta.com")
        .duration_in_years(2)
        .admin_contact(contact.clone())
        .registrant_contact(contact.clone())
        .tech_contact(contact.clone())
        .send()
        .await
        .expect("register second domain should succeed");
    assert!(
        reg2.operation_id().is_some(),
        "second registration must return an operation id"
    );

    // Step 3: Both domains appear in the list
    let list = client
        .list_domains()
        .send()
        .await
        .expect("list_domains should succeed");
    let names: Vec<_> = list
        .domains()
        .iter()
        .filter_map(|d| d.domain_name())
        .collect();
    assert!(
        names.contains(&"portfolio-alpha.com"),
        "portfolio-alpha.com must appear in domain list"
    );
    assert!(
        names.contains(&"portfolio-beta.com"),
        "portfolio-beta.com must appear in domain list"
    );

    // Step 4: Availability check confirms both are UNAVAILABLE
    let avail_alpha = client
        .check_domain_availability()
        .domain_name("portfolio-alpha.com")
        .send()
        .await
        .expect("check availability of alpha should succeed");
    assert_eq!(
        avail_alpha.availability(),
        Some(&aws_sdk_route53domains::types::DomainAvailability::Unavailable),
        "registered domain must be UNAVAILABLE"
    );

    // Step 5: Delete the first domain
    let del = client
        .delete_domain()
        .domain_name("portfolio-alpha.com")
        .send()
        .await
        .expect("delete_domain should succeed");
    assert!(
        del.operation_id().is_some(),
        "delete must return an operation id"
    );

    // Step 6: Deleted domain is no longer listed, the other still is
    let list2 = client
        .list_domains()
        .send()
        .await
        .expect("list_domains after deletion should succeed");
    let names2: Vec<_> = list2
        .domains()
        .iter()
        .filter_map(|d| d.domain_name())
        .collect();
    assert!(
        !names2.contains(&"portfolio-alpha.com"),
        "deleted domain must not appear in list"
    );
    assert!(
        names2.contains(&"portfolio-beta.com"),
        "remaining domain must still appear in list"
    );

    // Step 7: Deleted domain is now AVAILABLE again
    let avail_after = client
        .check_domain_availability()
        .domain_name("portfolio-alpha.com")
        .send()
        .await
        .expect("check availability after deletion should succeed");
    assert_eq!(
        avail_after.availability(),
        Some(&aws_sdk_route53domains::types::DomainAvailability::Available),
        "deleted domain must be AVAILABLE again"
    );
}

/// Scenario: domain registration with custom settings.
///
/// A user registers a domain with auto-renew disabled and privacy protection
/// explicitly set to false, then verifies those settings are preserved.
#[tokio::test]
async fn test_domain_registration_with_custom_settings() {
    // Scenario: domain registration with custom settings
    let client = make_client().await;
    let contact = make_contact();

    // Step 1: Register domain with non-default settings
    client
        .register_domain()
        .domain_name("custom-settings.com")
        .duration_in_years(3)
        .auto_renew(false)
        .privacy_protect_admin_contact(false)
        .privacy_protect_registrant_contact(false)
        .privacy_protect_tech_contact(false)
        .admin_contact(contact.clone())
        .registrant_contact(contact.clone())
        .tech_contact(contact)
        .send()
        .await
        .expect("register_domain with custom settings should succeed");

    // Step 2: Retrieve full domain detail
    let detail = client
        .get_domain_detail()
        .domain_name("custom-settings.com")
        .send()
        .await
        .expect("get_domain_detail should succeed");

    // Step 3: Verify all custom settings are preserved
    assert_eq!(
        detail.auto_renew(),
        Some(false),
        "auto_renew must be false as registered"
    );
    assert_eq!(
        detail.admin_privacy(),
        Some(false),
        "admin_privacy must be false as registered"
    );
    assert_eq!(
        detail.registrant_privacy(),
        Some(false),
        "registrant_privacy must be false as registered"
    );
    assert_eq!(
        detail.tech_privacy(),
        Some(false),
        "tech_privacy must be false as registered"
    );

    // Step 4: Confirm expiration date reflects 3-year duration
    let expiry = detail
        .expiration_date()
        .expect("expiration_date must be set");
    let now_secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    let two_years_secs: i64 = 2 * 365 * 24 * 3600;
    assert!(
        expiry.secs() > now_secs + two_years_secs,
        "3-year registration expiry must be more than 2 years from now"
    );

    // Step 5: Contact details are round-tripped correctly
    let admin = detail
        .admin_contact()
        .expect("admin_contact must be present");
    assert_eq!(admin.first_name(), Some("Jane"));
    assert_eq!(admin.email(), Some("jane@example.com"));
}
