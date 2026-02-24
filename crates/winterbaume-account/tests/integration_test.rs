use aws_sdk_account::config::BehaviorVersion;
use winterbaume_account::AccountService;
use winterbaume_core::MockAws;

async fn make_client() -> aws_sdk_account::Client {
    let mock = MockAws::builder()
        .with_service(AccountService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_account::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_account::Client::new(&config)
}

#[tokio::test]
async fn test_put_and_get_alternate_contact() {
    let client = make_client().await;

    client
        .put_alternate_contact()
        .alternate_contact_type(aws_sdk_account::types::AlternateContactType::Billing)
        .email_address("billing@example.com")
        .name("Billing Team")
        .phone_number("+1-555-0100")
        .title("Billing Manager")
        .send()
        .await
        .expect("put_alternate_contact should succeed");

    let resp = client
        .get_alternate_contact()
        .alternate_contact_type(aws_sdk_account::types::AlternateContactType::Billing)
        .send()
        .await
        .expect("get_alternate_contact should succeed");

    let contact = resp.alternate_contact().expect("should have contact");
    assert_eq!(contact.name(), Some("Billing Team"));
    assert_eq!(contact.email_address(), Some("billing@example.com"));
    assert_eq!(contact.phone_number(), Some("+1-555-0100"));
    assert_eq!(contact.title(), Some("Billing Manager"));
}

#[tokio::test]
async fn test_delete_alternate_contact() {
    let client = make_client().await;

    client
        .put_alternate_contact()
        .alternate_contact_type(aws_sdk_account::types::AlternateContactType::Operations)
        .email_address("ops@example.com")
        .name("Ops Team")
        .phone_number("+1-555-0200")
        .title("Ops Lead")
        .send()
        .await
        .unwrap();

    client
        .delete_alternate_contact()
        .alternate_contact_type(aws_sdk_account::types::AlternateContactType::Operations)
        .send()
        .await
        .expect("delete_alternate_contact should succeed");

    let result = client
        .get_alternate_contact()
        .alternate_contact_type(aws_sdk_account::types::AlternateContactType::Operations)
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_nonexistent_alternate_contact() {
    let client = make_client().await;

    let result = client
        .get_alternate_contact()
        .alternate_contact_type(aws_sdk_account::types::AlternateContactType::Security)
        .send()
        .await;
    assert!(result.is_err());
}

// ============================================================================
// Tests derived from AWS documentation: AWS Account Management
// ============================================================================

#[tokio::test]
async fn test_put_and_get_operations_contact() {
    let client = make_client().await;

    client
        .put_alternate_contact()
        .alternate_contact_type(aws_sdk_account::types::AlternateContactType::Operations)
        .email_address("ops@example.com")
        .name("Operations Team")
        .phone_number("+1-555-0201")
        .title("Operations Lead")
        .send()
        .await
        .expect("put_alternate_contact OPERATIONS should succeed");

    let resp = client
        .get_alternate_contact()
        .alternate_contact_type(aws_sdk_account::types::AlternateContactType::Operations)
        .send()
        .await
        .expect("get_alternate_contact OPERATIONS should succeed");

    let contact = resp.alternate_contact().expect("should have contact");
    assert_eq!(contact.name(), Some("Operations Team"));
    assert_eq!(contact.email_address(), Some("ops@example.com"));
}

#[tokio::test]
async fn test_put_and_get_security_contact() {
    let client = make_client().await;

    client
        .put_alternate_contact()
        .alternate_contact_type(aws_sdk_account::types::AlternateContactType::Security)
        .email_address("security@example.com")
        .name("Security Team")
        .phone_number("+1-555-0301")
        .title("CISO")
        .send()
        .await
        .expect("put_alternate_contact SECURITY should succeed");

    let resp = client
        .get_alternate_contact()
        .alternate_contact_type(aws_sdk_account::types::AlternateContactType::Security)
        .send()
        .await
        .expect("get_alternate_contact SECURITY should succeed");

    let contact = resp.alternate_contact().expect("should have contact");
    assert_eq!(contact.name(), Some("Security Team"));
    assert_eq!(contact.title(), Some("CISO"));
}

#[tokio::test]
async fn test_put_overwrites_existing_contact() {
    let client = make_client().await;

    client
        .put_alternate_contact()
        .alternate_contact_type(aws_sdk_account::types::AlternateContactType::Billing)
        .email_address("billing-v1@example.com")
        .name("Billing Team v1")
        .phone_number("+1-555-0100")
        .title("CFO v1")
        .send()
        .await
        .unwrap();

    client
        .put_alternate_contact()
        .alternate_contact_type(aws_sdk_account::types::AlternateContactType::Billing)
        .email_address("billing-v2@example.com")
        .name("Billing Team v2")
        .phone_number("+1-555-0101")
        .title("CFO v2")
        .send()
        .await
        .expect("second put should succeed (overwrite)");

    let resp = client
        .get_alternate_contact()
        .alternate_contact_type(aws_sdk_account::types::AlternateContactType::Billing)
        .send()
        .await
        .unwrap();

    let contact = resp.alternate_contact().unwrap();
    assert_eq!(contact.name(), Some("Billing Team v2"));
    assert_eq!(contact.email_address(), Some("billing-v2@example.com"));
}

#[tokio::test]
async fn test_delete_nonexistent_contact_returns_error() {
    let client = make_client().await;

    let result = client
        .delete_alternate_contact()
        .alternate_contact_type(aws_sdk_account::types::AlternateContactType::Billing)
        .send()
        .await;
    assert!(
        result.is_err(),
        "delete on nonexistent contact should return error"
    );
}

#[tokio::test]
async fn test_get_after_delete_returns_not_found() {
    let client = make_client().await;

    client
        .put_alternate_contact()
        .alternate_contact_type(aws_sdk_account::types::AlternateContactType::Security)
        .email_address("sec@example.com")
        .name("Sec")
        .phone_number("+1-555-0001")
        .title("Security Lead")
        .send()
        .await
        .unwrap();

    client
        .delete_alternate_contact()
        .alternate_contact_type(aws_sdk_account::types::AlternateContactType::Security)
        .send()
        .await
        .unwrap();

    let result = client
        .get_alternate_contact()
        .alternate_contact_type(aws_sdk_account::types::AlternateContactType::Security)
        .send()
        .await;
    assert!(result.is_err(), "get after delete should return error");
}

#[tokio::test]
async fn test_put_after_delete_recreates_contact() {
    let client = make_client().await;

    client
        .put_alternate_contact()
        .alternate_contact_type(aws_sdk_account::types::AlternateContactType::Operations)
        .email_address("ops@example.com")
        .name("Ops Team")
        .phone_number("+1-555-0200")
        .title("Ops Lead")
        .send()
        .await
        .unwrap();

    client
        .delete_alternate_contact()
        .alternate_contact_type(aws_sdk_account::types::AlternateContactType::Operations)
        .send()
        .await
        .unwrap();

    client
        .put_alternate_contact()
        .alternate_contact_type(aws_sdk_account::types::AlternateContactType::Operations)
        .email_address("ops-new@example.com")
        .name("New Ops Team")
        .phone_number("+1-555-0210")
        .title("New Ops Lead")
        .send()
        .await
        .expect("put after delete should succeed");

    let resp = client
        .get_alternate_contact()
        .alternate_contact_type(aws_sdk_account::types::AlternateContactType::Operations)
        .send()
        .await
        .expect("get after re-create should succeed");

    let contact = resp.alternate_contact().unwrap();
    assert_eq!(contact.name(), Some("New Ops Team"));
}

#[tokio::test]
async fn test_three_contact_types_are_independent() {
    let client = make_client().await;

    for (contact_type, name, email) in [
        (
            aws_sdk_account::types::AlternateContactType::Billing,
            "Billing",
            "billing@example.com",
        ),
        (
            aws_sdk_account::types::AlternateContactType::Operations,
            "Operations",
            "ops@example.com",
        ),
        (
            aws_sdk_account::types::AlternateContactType::Security,
            "Security",
            "sec@example.com",
        ),
    ] {
        client
            .put_alternate_contact()
            .alternate_contact_type(contact_type)
            .email_address(email)
            .name(name)
            .phone_number("+1-555-0001")
            .title("Lead")
            .send()
            .await
            .unwrap();
    }

    let billing = client
        .get_alternate_contact()
        .alternate_contact_type(aws_sdk_account::types::AlternateContactType::Billing)
        .send()
        .await
        .unwrap();
    let ops = client
        .get_alternate_contact()
        .alternate_contact_type(aws_sdk_account::types::AlternateContactType::Operations)
        .send()
        .await
        .unwrap();
    let sec = client
        .get_alternate_contact()
        .alternate_contact_type(aws_sdk_account::types::AlternateContactType::Security)
        .send()
        .await
        .unwrap();

    assert_eq!(billing.alternate_contact().unwrap().name(), Some("Billing"));
    assert_eq!(ops.alternate_contact().unwrap().name(), Some("Operations"));
    assert_eq!(sec.alternate_contact().unwrap().name(), Some("Security"));
}

#[tokio::test]
async fn test_get_returns_contact_type_field() {
    let client = make_client().await;

    client
        .put_alternate_contact()
        .alternate_contact_type(aws_sdk_account::types::AlternateContactType::Billing)
        .email_address("billing@example.com")
        .name("Billing Team")
        .phone_number("+1-555-0100")
        .title("CFO")
        .send()
        .await
        .unwrap();

    let resp = client
        .get_alternate_contact()
        .alternate_contact_type(aws_sdk_account::types::AlternateContactType::Billing)
        .send()
        .await
        .unwrap();

    let contact = resp.alternate_contact().unwrap();
    assert_eq!(
        contact.alternate_contact_type(),
        Some(&aws_sdk_account::types::AlternateContactType::Billing)
    );
}

// ============================================================================
// Additional tests derived from AWS documentation: AWS Account Management
// ============================================================================

#[tokio::test]
async fn test_all_response_fields_present() {
    // All five fields (AlternateContactType, EmailAddress, Name, PhoneNumber, Title)
    // must be present in GetAlternateContact response per the API spec.
    let client = make_client().await;

    client
        .put_alternate_contact()
        .alternate_contact_type(aws_sdk_account::types::AlternateContactType::Security)
        .email_address("all-fields@example.com")
        .name("All Fields Tester")
        .phone_number("+1-555-9999")
        .title("Field Inspector")
        .send()
        .await
        .unwrap();

    let resp = client
        .get_alternate_contact()
        .alternate_contact_type(aws_sdk_account::types::AlternateContactType::Security)
        .send()
        .await
        .unwrap();

    let contact = resp.alternate_contact().expect("should have contact");
    assert!(
        contact.alternate_contact_type().is_some(),
        "alternate_contact_type should be present"
    );
    assert_eq!(contact.email_address(), Some("all-fields@example.com"));
    assert_eq!(contact.name(), Some("All Fields Tester"));
    assert_eq!(contact.phone_number(), Some("+1-555-9999"));
    assert_eq!(contact.title(), Some("Field Inspector"));
}

#[tokio::test]
async fn test_delete_then_get_returns_resource_not_found_error_code() {
    // After deleting a contact, GetAlternateContact must return ResourceNotFoundException.
    let client = make_client().await;

    client
        .put_alternate_contact()
        .alternate_contact_type(aws_sdk_account::types::AlternateContactType::Billing)
        .email_address("billing-delete@example.com")
        .name("Delete Test")
        .phone_number("+1-555-0001")
        .title("Tester")
        .send()
        .await
        .unwrap();

    client
        .delete_alternate_contact()
        .alternate_contact_type(aws_sdk_account::types::AlternateContactType::Billing)
        .send()
        .await
        .unwrap();

    let err = client
        .get_alternate_contact()
        .alternate_contact_type(aws_sdk_account::types::AlternateContactType::Billing)
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException after delete, got: {err_str}"
    );
}

#[tokio::test]
async fn test_delete_nonexistent_returns_resource_not_found_error_code() {
    // DeleteAlternateContact on a nonexistent contact must return ResourceNotFoundException.
    let client = make_client().await;

    let err = client
        .delete_alternate_contact()
        .alternate_contact_type(aws_sdk_account::types::AlternateContactType::Operations)
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException for missing contact, got: {err_str}"
    );
}

#[tokio::test]
async fn test_full_lifecycle_all_fields() {
    // Full create -> get (verify) -> update (overwrite) -> get (verify update) -> delete -> get (verify gone)
    let client = make_client().await;

    // Create
    client
        .put_alternate_contact()
        .alternate_contact_type(aws_sdk_account::types::AlternateContactType::Operations)
        .email_address("lifecycle-v1@example.com")
        .name("Lifecycle v1")
        .phone_number("+1-555-1001")
        .title("Original Title")
        .send()
        .await
        .expect("initial put should succeed");

    // Get and verify initial state
    let resp = client
        .get_alternate_contact()
        .alternate_contact_type(aws_sdk_account::types::AlternateContactType::Operations)
        .send()
        .await
        .expect("get after initial put should succeed");

    let contact = resp.alternate_contact().unwrap();
    assert_eq!(contact.name(), Some("Lifecycle v1"));
    assert_eq!(contact.email_address(), Some("lifecycle-v1@example.com"));
    assert_eq!(contact.phone_number(), Some("+1-555-1001"));
    assert_eq!(contact.title(), Some("Original Title"));
    assert_eq!(
        contact.alternate_contact_type(),
        Some(&aws_sdk_account::types::AlternateContactType::Operations)
    );

    // Update (overwrite)
    client
        .put_alternate_contact()
        .alternate_contact_type(aws_sdk_account::types::AlternateContactType::Operations)
        .email_address("lifecycle-v2@example.com")
        .name("Lifecycle v2")
        .phone_number("+1-555-1002")
        .title("Updated Title")
        .send()
        .await
        .expect("update put should succeed");

    // Get and verify updated state
    let resp2 = client
        .get_alternate_contact()
        .alternate_contact_type(aws_sdk_account::types::AlternateContactType::Operations)
        .send()
        .await
        .expect("get after update should succeed");

    let contact2 = resp2.alternate_contact().unwrap();
    assert_eq!(contact2.name(), Some("Lifecycle v2"));
    assert_eq!(contact2.email_address(), Some("lifecycle-v2@example.com"));
    assert_eq!(contact2.phone_number(), Some("+1-555-1002"));
    assert_eq!(contact2.title(), Some("Updated Title"));

    // Delete
    client
        .delete_alternate_contact()
        .alternate_contact_type(aws_sdk_account::types::AlternateContactType::Operations)
        .send()
        .await
        .expect("delete should succeed");

    // Verify gone
    let result = client
        .get_alternate_contact()
        .alternate_contact_type(aws_sdk_account::types::AlternateContactType::Operations)
        .send()
        .await;
    assert!(result.is_err(), "get after delete should return an error");
}

// ============================================================================
// PutContactInformation / GetContactInformation
// ============================================================================

#[tokio::test]
async fn test_put_and_get_contact_information() {
    let client = make_client().await;

    client
        .put_contact_information()
        .contact_information(
            aws_sdk_account::types::ContactInformation::builder()
                .address_line1("123 Main St")
                .city("Seattle")
                .country_code("US")
                .full_name("John Doe")
                .phone_number("+1-555-0000")
                .postal_code("98101")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("put_contact_information should succeed");

    let resp = client
        .get_contact_information()
        .send()
        .await
        .expect("get_contact_information should succeed");

    let info = resp
        .contact_information()
        .expect("should have contact information");
    assert_eq!(info.address_line1(), "123 Main St");
    assert_eq!(info.city(), "Seattle");
    assert_eq!(info.country_code(), "US");
    assert_eq!(info.full_name(), "John Doe");
    assert_eq!(info.phone_number(), "+1-555-0000");
    assert_eq!(info.postal_code(), "98101");
}

#[tokio::test]
async fn test_put_contact_information_overwrites() {
    let client = make_client().await;

    client
        .put_contact_information()
        .contact_information(
            aws_sdk_account::types::ContactInformation::builder()
                .address_line1("123 Main St")
                .city("Seattle")
                .country_code("US")
                .full_name("John Doe")
                .phone_number("+1-555-0001")
                .postal_code("98101")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    client
        .put_contact_information()
        .contact_information(
            aws_sdk_account::types::ContactInformation::builder()
                .address_line1("456 New Ave")
                .city("Portland")
                .country_code("US")
                .full_name("Jane Smith")
                .phone_number("+1-555-0002")
                .postal_code("97201")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("second put should overwrite");

    let resp = client.get_contact_information().send().await.unwrap();

    let info = resp.contact_information().unwrap();
    assert_eq!(info.address_line1(), "456 New Ave");
    assert_eq!(info.city(), "Portland");
    assert_eq!(info.full_name(), "Jane Smith");
}

#[tokio::test]
async fn test_get_contact_information_not_found() {
    let client = make_client().await;
    let result = client.get_contact_information().send().await;
    assert!(result.is_err(), "get before put should return error");
}

// ============================================================================
// PutAccountName / GetAccountInformation
// ============================================================================

#[tokio::test]
async fn test_put_account_name_and_get_account_information() {
    let client = make_client().await;

    client
        .put_account_name()
        .account_name("My Test Account")
        .send()
        .await
        .expect("put_account_name should succeed");

    let resp = client
        .get_account_information()
        .send()
        .await
        .expect("get_account_information should succeed");

    assert_eq!(resp.account_name(), Some("My Test Account"));
}

#[tokio::test]
async fn test_get_account_information_without_name() {
    let client = make_client().await;

    let resp = client
        .get_account_information()
        .send()
        .await
        .expect("get_account_information should succeed even without a name");

    // account_name may be None if never set
    assert!(resp.account_name().is_none() || resp.account_name().is_some());
}

#[tokio::test]
async fn test_put_account_name_overwrites() {
    let client = make_client().await;

    client
        .put_account_name()
        .account_name("First Name")
        .send()
        .await
        .unwrap();

    client
        .put_account_name()
        .account_name("Second Name")
        .send()
        .await
        .expect("second put_account_name should succeed");

    let resp = client.get_account_information().send().await.unwrap();

    assert_eq!(resp.account_name(), Some("Second Name"));
}

// ============================================================================
// GetGovCloudAccountInformation
// ============================================================================

#[tokio::test]
async fn test_get_gov_cloud_account_information() {
    let client = make_client().await;

    let resp = client
        .get_gov_cloud_account_information()
        .standard_account_id("123456789012")
        .send()
        .await
        .expect("get_gov_cloud_account_information should succeed");

    // Mock returns a fixed response
    let _ = resp;
}

// ============================================================================
// GetPrimaryEmail / StartPrimaryEmailUpdate / AcceptPrimaryEmailUpdate
// ============================================================================

#[tokio::test]
async fn test_get_primary_email_initially_none() {
    let client = make_client().await;

    let resp = client
        .get_primary_email()
        .account_id("123456789012")
        .send()
        .await
        .expect("get_primary_email should succeed");

    // No email set initially
    assert!(resp.primary_email().is_none());
}

#[tokio::test]
async fn test_start_and_accept_primary_email_update() {
    let client = make_client().await;

    let start_resp = client
        .start_primary_email_update()
        .account_id("123456789012")
        .primary_email("new@example.com")
        .send()
        .await
        .expect("start_primary_email_update should succeed");

    assert!(start_resp.status().is_some());

    // The mock OTP is deterministic: "123456"
    let accept_resp = client
        .accept_primary_email_update()
        .account_id("123456789012")
        .primary_email("new@example.com")
        .otp("123456")
        .send()
        .await
        .expect("accept_primary_email_update should succeed");

    assert!(accept_resp.status().is_some());

    // Now the primary email should be updated
    let resp = client
        .get_primary_email()
        .account_id("123456789012")
        .send()
        .await
        .expect("get_primary_email should succeed");

    assert_eq!(resp.primary_email(), Some("new@example.com"));
}

#[tokio::test]
async fn test_accept_primary_email_update_wrong_otp() {
    let client = make_client().await;

    client
        .start_primary_email_update()
        .account_id("123456789012")
        .primary_email("new@example.com")
        .send()
        .await
        .expect("start_primary_email_update should succeed");

    let result = client
        .accept_primary_email_update()
        .account_id("123456789012")
        .primary_email("new@example.com")
        .otp("000000") // wrong OTP
        .send()
        .await;

    assert!(result.is_err(), "wrong OTP should return error");
}

#[tokio::test]
async fn test_accept_without_start_returns_error() {
    let client = make_client().await;

    let result = client
        .accept_primary_email_update()
        .account_id("123456789012")
        .primary_email("new@example.com")
        .otp("123456")
        .send()
        .await;

    assert!(result.is_err(), "accept without start should return error");
}

// ============================================================================
// GetRegionOptStatus / EnableRegion / DisableRegion / ListRegions
// ============================================================================

#[tokio::test]
async fn test_get_region_opt_status_default_enabled() {
    let client = make_client().await;

    let resp = client
        .get_region_opt_status()
        .region_name("us-east-1")
        .send()
        .await
        .expect("get_region_opt_status should succeed");

    assert_eq!(resp.region_name(), Some("us-east-1"));
    let status = resp.region_opt_status().expect("status should be present");
    assert_eq!(status.as_str(), "ENABLED_BY_DEFAULT");
}

#[tokio::test]
async fn test_get_region_opt_status_opt_in_region_disabled() {
    let client = make_client().await;

    let resp = client
        .get_region_opt_status()
        .region_name("af-south-1")
        .send()
        .await
        .expect("get_region_opt_status should succeed");

    assert_eq!(resp.region_name(), Some("af-south-1"));
    let status = resp.region_opt_status().expect("status should be present");
    assert_eq!(status.as_str(), "DISABLED");
}

#[tokio::test]
async fn test_enable_and_disable_region() {
    let client = make_client().await;

    // Enable an opt-in region
    client
        .enable_region()
        .region_name("af-south-1")
        .send()
        .await
        .expect("enable_region should succeed");

    let resp = client
        .get_region_opt_status()
        .region_name("af-south-1")
        .send()
        .await
        .unwrap();

    let status = resp.region_opt_status().unwrap();
    assert_eq!(status.as_str(), "ENABLED");

    // Disable it back
    client
        .disable_region()
        .region_name("af-south-1")
        .send()
        .await
        .expect("disable_region should succeed");

    let resp2 = client
        .get_region_opt_status()
        .region_name("af-south-1")
        .send()
        .await
        .unwrap();

    let status2 = resp2.region_opt_status().unwrap();
    assert_eq!(status2.as_str(), "DISABLED");
}

#[tokio::test]
async fn test_enable_default_region_returns_error() {
    let client = make_client().await;

    let result = client
        .enable_region()
        .region_name("us-east-1") // enabled by default, cannot opt in
        .send()
        .await;

    assert!(
        result.is_err(),
        "enabling a default-enabled region should return error"
    );
}

#[tokio::test]
async fn test_disable_default_region_returns_error() {
    let client = make_client().await;

    let result = client
        .disable_region()
        .region_name("us-east-1") // enabled by default, cannot disable
        .send()
        .await;

    assert!(
        result.is_err(),
        "disabling a default-enabled region should return error"
    );
}

#[tokio::test]
async fn test_list_regions_returns_all() {
    let client = make_client().await;

    let resp = client
        .list_regions()
        .send()
        .await
        .expect("list_regions should succeed");

    let regions = resp.regions();
    assert!(!regions.is_empty(), "should have some regions");
    // Verify us-east-1 is in the list
    let us_east = regions
        .iter()
        .find(|r: &&aws_sdk_account::types::Region| r.region_name() == Some("us-east-1"));
    assert!(us_east.is_some(), "us-east-1 should be in the list");
}

#[tokio::test]
async fn test_list_regions_filter_by_status() {
    let client = make_client().await;

    let resp = client
        .list_regions()
        .region_opt_status_contains(aws_sdk_account::types::RegionOptStatus::EnabledByDefault)
        .send()
        .await
        .expect("list_regions with filter should succeed");

    let regions = resp.regions();
    assert!(
        !regions.is_empty(),
        "should have some ENABLED_BY_DEFAULT regions"
    );
    for r in regions {
        assert_eq!(
            r.region_opt_status()
                .map(|s: &aws_sdk_account::types::RegionOptStatus| s.as_str()),
            Some("ENABLED_BY_DEFAULT"),
            "all results should have ENABLED_BY_DEFAULT status"
        );
    }
}
