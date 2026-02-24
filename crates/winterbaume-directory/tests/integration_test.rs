//! Integration tests for winterbaume Directory Service.
//!
//! These tests verify that aws-sdk-directory operations work end-to-end
//! through the winterbaume mock infrastructure.

use aws_sdk_directory::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_directory::DirectoryService;

/// Helper to create a configured Directory Service client backed by winterbaume.
async fn make_ds_client() -> aws_sdk_directory::Client {
    let mock = MockAws::builder()
        .with_service(DirectoryService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_directory::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_directory::Client::new(&config)
}

#[tokio::test]
async fn test_create_directory() {
    let client = make_ds_client().await;

    let resp = client
        .create_directory()
        .name("corp.example.com")
        .password("SuperSecret123!")
        .size(aws_sdk_directory::types::DirectorySize::Small)
        .send()
        .await
        .expect("create_directory should succeed");

    let dir_id = resp.directory_id().expect("should have directory ID");
    assert!(
        dir_id.starts_with("d-"),
        "directory ID should start with 'd-'"
    );
}

#[tokio::test]
async fn test_create_directory_with_vpc_settings() {
    let client = make_ds_client().await;

    let vpc_settings = aws_sdk_directory::types::DirectoryVpcSettings::builder()
        .vpc_id("vpc-12345678")
        .subnet_ids("subnet-aaaaaaaa")
        .subnet_ids("subnet-bbbbbbbb")
        .build()
        .unwrap();

    let resp = client
        .create_directory()
        .name("corp.example.com")
        .password("SuperSecret123!")
        .size(aws_sdk_directory::types::DirectorySize::Small)
        .vpc_settings(vpc_settings)
        .description("Test directory")
        .short_name("CORP")
        .send()
        .await
        .expect("create_directory with VPC settings should succeed");

    let dir_id = resp.directory_id().expect("should have directory ID");
    assert!(dir_id.starts_with("d-"));
}

#[tokio::test]
async fn test_describe_directories() {
    let client = make_ds_client().await;

    // Create two directories
    let resp1 = client
        .create_directory()
        .name("dir1.example.com")
        .password("Pass1234!")
        .size(aws_sdk_directory::types::DirectorySize::Small)
        .send()
        .await
        .unwrap();
    let id1 = resp1.directory_id().unwrap().to_string();

    let resp2 = client
        .create_directory()
        .name("dir2.example.com")
        .password("Pass5678!")
        .size(aws_sdk_directory::types::DirectorySize::Large)
        .send()
        .await
        .unwrap();
    let id2 = resp2.directory_id().unwrap().to_string();

    // Describe all
    let desc_resp = client
        .describe_directories()
        .send()
        .await
        .expect("describe_directories should succeed");

    let dirs = desc_resp.directory_descriptions();
    assert_eq!(dirs.len(), 2, "should have 2 directories");

    // Describe by ID
    let desc_one = client
        .describe_directories()
        .directory_ids(&id1)
        .send()
        .await
        .expect("describe_directories by ID should succeed");

    let dirs = desc_one.directory_descriptions();
    assert_eq!(dirs.len(), 1);
    assert_eq!(dirs[0].directory_id(), Some(id1.as_str()));
    assert_eq!(dirs[0].name(), Some("dir1.example.com"));
    assert_eq!(
        dirs[0].size(),
        Some(&aws_sdk_directory::types::DirectorySize::Small)
    );

    // Verify the second directory
    let desc_two = client
        .describe_directories()
        .directory_ids(&id2)
        .send()
        .await
        .unwrap();

    let dirs = desc_two.directory_descriptions();
    assert_eq!(dirs[0].name(), Some("dir2.example.com"));
}

#[tokio::test]
async fn test_delete_directory() {
    let client = make_ds_client().await;

    let resp = client
        .create_directory()
        .name("delete-me.example.com")
        .password("DeleteMe123!")
        .size(aws_sdk_directory::types::DirectorySize::Small)
        .send()
        .await
        .unwrap();

    let dir_id = resp.directory_id().unwrap().to_string();

    // Delete
    let del_resp = client
        .delete_directory()
        .directory_id(&dir_id)
        .send()
        .await
        .expect("delete_directory should succeed");

    assert_eq!(del_resp.directory_id(), Some(dir_id.as_str()));

    // Verify it's gone
    let desc = client
        .describe_directories()
        .directory_ids(&dir_id)
        .send()
        .await;

    assert!(desc.is_err(), "describe after delete should fail");
}

#[tokio::test]
async fn test_delete_nonexistent_directory_fails() {
    let client = make_ds_client().await;

    let result = client
        .delete_directory()
        .directory_id("d-0000000000")
        .send()
        .await;

    assert!(
        result.is_err(),
        "delete_directory for nonexistent should fail"
    );
}

#[tokio::test]
async fn test_connect_directory() {
    let client = make_ds_client().await;

    let connect_settings = aws_sdk_directory::types::DirectoryConnectSettings::builder()
        .vpc_id("vpc-12345678")
        .subnet_ids("subnet-aaaaaaaa")
        .subnet_ids("subnet-bbbbbbbb")
        .customer_dns_ips("10.0.0.1")
        .customer_dns_ips("10.0.0.2")
        .customer_user_name("Admin")
        .build()
        .unwrap();

    let resp = client
        .connect_directory()
        .name("onprem.example.com")
        .password("ConnectPass123!")
        .size(aws_sdk_directory::types::DirectorySize::Small)
        .connect_settings(connect_settings)
        .send()
        .await
        .expect("connect_directory should succeed");

    let dir_id = resp.directory_id().expect("should have directory ID");
    assert!(dir_id.starts_with("d-"));

    // Verify it appears in describe
    let desc = client
        .describe_directories()
        .directory_ids(dir_id)
        .send()
        .await
        .expect("describe should succeed");

    let dirs = desc.directory_descriptions();
    assert_eq!(dirs.len(), 1);
    assert_eq!(dirs[0].name(), Some("onprem.example.com"));
    assert_eq!(
        dirs[0].r#type(),
        Some(&aws_sdk_directory::types::DirectoryType::AdConnector)
    );

    // Verify connect settings are present
    let cs = dirs[0]
        .connect_settings()
        .expect("should have connect settings");
    assert_eq!(cs.customer_user_name(), Some("Admin"));
}

#[tokio::test]
async fn test_create_directory_missing_password_fails() {
    let client = make_ds_client().await;

    // The SDK likely requires password, but test the mock's validation
    let result = client
        .create_directory()
        .name("no-pass.example.com")
        .size(aws_sdk_directory::types::DirectorySize::Small)
        .send()
        .await;

    // SDK may fail before reaching mock, but either way it should error
    assert!(
        result.is_err(),
        "create_directory without password should fail"
    );
}

#[tokio::test]
async fn test_describe_nonexistent_directory_fails() {
    let client = make_ds_client().await;

    let result = client
        .describe_directories()
        .directory_ids("d-0000000000")
        .send()
        .await;

    assert!(
        result.is_err(),
        "describe_directories for nonexistent should fail"
    );
}

#[tokio::test]
async fn test_create_directory_large_size() {
    let client = make_ds_client().await;

    let resp = client
        .create_directory()
        .name("large.example.com")
        .password("LargePass123!")
        .size(aws_sdk_directory::types::DirectorySize::Large)
        .send()
        .await
        .expect("create_directory with Large size should succeed");

    let dir_id = resp.directory_id().expect("should have directory ID");
    assert!(
        dir_id.starts_with("d-"),
        "directory ID should start with 'd-'"
    );

    let desc = client
        .describe_directories()
        .directory_ids(dir_id)
        .send()
        .await
        .unwrap();
    let dirs = desc.directory_descriptions();
    assert_eq!(dirs.len(), 1);
    assert_eq!(
        dirs[0].size(),
        Some(&aws_sdk_directory::types::DirectorySize::Large)
    );
}

#[tokio::test]
async fn test_create_directory_missing_name_fails() {
    let client = make_ds_client().await;

    // The SDK requires Name, so this should fail before or at the mock
    let result = client
        .create_directory()
        .password("SomePass123!")
        .size(aws_sdk_directory::types::DirectorySize::Small)
        .send()
        .await;

    assert!(result.is_err(), "create_directory without name should fail");
}

#[tokio::test]
async fn test_describe_directories_empty_returns_all() {
    let client = make_ds_client().await;

    // No directories exist yet; describe_all should return empty list
    let resp = client
        .describe_directories()
        .send()
        .await
        .expect("describe_directories with no directories should succeed");

    assert_eq!(
        resp.directory_descriptions().len(),
        0,
        "should return empty list when no directories exist"
    );
}

#[tokio::test]
async fn test_describe_directories_multiple_ids() {
    let client = make_ds_client().await;

    let id1 = client
        .create_directory()
        .name("multi1.example.com")
        .password("Multi1Pass!")
        .size(aws_sdk_directory::types::DirectorySize::Small)
        .send()
        .await
        .unwrap()
        .directory_id()
        .unwrap()
        .to_string();

    let id2 = client
        .create_directory()
        .name("multi2.example.com")
        .password("Multi2Pass!")
        .size(aws_sdk_directory::types::DirectorySize::Small)
        .send()
        .await
        .unwrap()
        .directory_id()
        .unwrap()
        .to_string();

    let id3 = client
        .create_directory()
        .name("multi3.example.com")
        .password("Multi3Pass!")
        .size(aws_sdk_directory::types::DirectorySize::Small)
        .send()
        .await
        .unwrap()
        .directory_id()
        .unwrap()
        .to_string();

    // Request only id1 and id3
    let resp = client
        .describe_directories()
        .directory_ids(&id1)
        .directory_ids(&id3)
        .send()
        .await
        .expect("describe_directories with multiple IDs should succeed");

    let dirs = resp.directory_descriptions();
    assert_eq!(
        dirs.len(),
        2,
        "should return exactly the two requested dirs"
    );

    let returned_ids: Vec<&str> = dirs.iter().filter_map(|d| d.directory_id()).collect();
    assert!(returned_ids.contains(&id1.as_str()));
    assert!(returned_ids.contains(&id3.as_str()));
    assert!(!returned_ids.contains(&id2.as_str()));
}

#[tokio::test]
async fn test_describe_directory_fields_populated() {
    let client = make_ds_client().await;

    let resp = client
        .create_directory()
        .name("fields.example.com")
        .password("Fields123!")
        .size(aws_sdk_directory::types::DirectorySize::Small)
        .short_name("FIELDS")
        .description("A test directory for field verification")
        .vpc_settings(
            aws_sdk_directory::types::DirectoryVpcSettings::builder()
                .vpc_id("vpc-aabbccdd")
                .subnet_ids("subnet-11111111")
                .subnet_ids("subnet-22222222")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let dir_id = resp.directory_id().unwrap().to_string();

    let desc = client
        .describe_directories()
        .directory_ids(&dir_id)
        .send()
        .await
        .unwrap();

    let d = &desc.directory_descriptions()[0];
    assert_eq!(d.name(), Some("fields.example.com"));
    assert_eq!(d.short_name(), Some("FIELDS"));
    assert_eq!(
        d.description(),
        Some("A test directory for field verification")
    );
    assert_eq!(
        d.r#type(),
        Some(&aws_sdk_directory::types::DirectoryType::SimpleAd)
    );
    assert_eq!(
        d.stage(),
        Some(&aws_sdk_directory::types::DirectoryStage::Active)
    );

    // VPC settings should be present and populated
    let vpc = d.vpc_settings().expect("should have vpc_settings");
    assert_eq!(vpc.vpc_id(), Some("vpc-aabbccdd"));
    let subnets = vpc.subnet_ids();
    assert_eq!(subnets.len(), 2);
    assert!(subnets.contains(&"subnet-11111111".to_string()));
    assert!(subnets.contains(&"subnet-22222222".to_string()));

    // access_url and alias should be derived from directory_id
    assert!(d.access_url().unwrap_or("").ends_with(".awsapps.com"));
    assert_eq!(d.alias(), Some(dir_id.as_str()));

    // DNS IP addresses should be set
    assert!(!d.dns_ip_addrs().is_empty(), "should have dns ip addresses");
}

#[tokio::test]
async fn test_delete_directory_twice_fails() {
    let client = make_ds_client().await;

    let dir_id = client
        .create_directory()
        .name("double-delete.example.com")
        .password("DblDelete1!")
        .size(aws_sdk_directory::types::DirectorySize::Small)
        .send()
        .await
        .unwrap()
        .directory_id()
        .unwrap()
        .to_string();

    // First delete should succeed
    client
        .delete_directory()
        .directory_id(&dir_id)
        .send()
        .await
        .expect("first delete should succeed");

    // Second delete of the same ID should fail
    let result = client.delete_directory().directory_id(&dir_id).send().await;

    assert!(
        result.is_err(),
        "second delete of same directory should fail"
    );
}

#[tokio::test]
async fn test_connect_directory_large_size() {
    let client = make_ds_client().await;

    let connect_settings = aws_sdk_directory::types::DirectoryConnectSettings::builder()
        .vpc_id("vpc-99887766")
        .subnet_ids("subnet-cccccccc")
        .subnet_ids("subnet-dddddddd")
        .customer_dns_ips("192.168.1.10")
        .customer_user_name("svcaccount")
        .build()
        .unwrap();

    let resp = client
        .connect_directory()
        .name("large-onprem.example.com")
        .password("LargeConnect1!")
        .size(aws_sdk_directory::types::DirectorySize::Large)
        .connect_settings(connect_settings)
        .send()
        .await
        .expect("connect_directory with Large size should succeed");

    let dir_id = resp.directory_id().unwrap().to_string();

    let desc = client
        .describe_directories()
        .directory_ids(&dir_id)
        .send()
        .await
        .unwrap();

    let d = &desc.directory_descriptions()[0];
    assert_eq!(
        d.size(),
        Some(&aws_sdk_directory::types::DirectorySize::Large)
    );
    assert_eq!(
        d.r#type(),
        Some(&aws_sdk_directory::types::DirectoryType::AdConnector)
    );
}

#[tokio::test]
async fn test_connect_directory_with_optional_fields() {
    let client = make_ds_client().await;

    let connect_settings = aws_sdk_directory::types::DirectoryConnectSettings::builder()
        .vpc_id("vpc-55443322")
        .subnet_ids("subnet-eeeeeeee")
        .subnet_ids("subnet-ffffffff")
        .customer_dns_ips("10.10.10.10")
        .customer_dns_ips("10.10.10.11")
        .customer_user_name("DomainAdmin")
        .build()
        .unwrap();

    let resp = client
        .connect_directory()
        .name("optional.example.com")
        .password("OptFields1!")
        .size(aws_sdk_directory::types::DirectorySize::Small)
        .short_name("OPT")
        .description("Connector with all optional fields")
        .connect_settings(connect_settings)
        .send()
        .await
        .expect("connect_directory with optional fields should succeed");

    let dir_id = resp.directory_id().unwrap().to_string();
    assert!(dir_id.starts_with("d-"));

    let desc = client
        .describe_directories()
        .directory_ids(&dir_id)
        .send()
        .await
        .unwrap();

    let d = &desc.directory_descriptions()[0];
    assert_eq!(d.short_name(), Some("OPT"));
    assert_eq!(d.description(), Some("Connector with all optional fields"));

    let cs = d.connect_settings().expect("should have connect_settings");
    assert_eq!(cs.customer_user_name(), Some("DomainAdmin"));
    assert_eq!(cs.vpc_id(), Some("vpc-55443322"));
}

#[tokio::test]
async fn test_create_and_delete_multiple_directories_independently() {
    let client = make_ds_client().await;

    let id_a = client
        .create_directory()
        .name("keep.example.com")
        .password("KeepMe1234!")
        .size(aws_sdk_directory::types::DirectorySize::Small)
        .send()
        .await
        .unwrap()
        .directory_id()
        .unwrap()
        .to_string();

    let id_b = client
        .create_directory()
        .name("remove.example.com")
        .password("RemoveMe1!")
        .size(aws_sdk_directory::types::DirectorySize::Small)
        .send()
        .await
        .unwrap()
        .directory_id()
        .unwrap()
        .to_string();

    // Delete only id_b
    client
        .delete_directory()
        .directory_id(&id_b)
        .send()
        .await
        .expect("delete id_b should succeed");

    // id_a should still be describable
    let resp = client
        .describe_directories()
        .directory_ids(&id_a)
        .send()
        .await
        .expect("id_a should still exist after id_b deleted");

    assert_eq!(resp.directory_descriptions().len(), 1);
    assert_eq!(
        resp.directory_descriptions()[0].name(),
        Some("keep.example.com")
    );

    // id_b should be gone
    let result = client
        .describe_directories()
        .directory_ids(&id_b)
        .send()
        .await;
    assert!(result.is_err(), "id_b should no longer exist");

    // describe all should return only one
    let all = client.describe_directories().send().await.unwrap();
    assert_eq!(all.directory_descriptions().len(), 1);
}

// ============================================================================
// Tests derived from AWS documentation: Directory Service (2026-03-28)
// ============================================================================

/// Verify that the DirectoryId returned by CreateDirectory matches the AWS
/// documented pattern: ^d-[0-9a-f]{10}$
#[tokio::test]
async fn test_directory_id_format() {
    let client = make_ds_client().await;

    let resp = client
        .create_directory()
        .name("format-check.example.com")
        .password("Format1234!")
        .size(aws_sdk_directory::types::DirectorySize::Small)
        .send()
        .await
        .expect("create_directory should succeed");

    let dir_id = resp.directory_id().expect("should have directory ID");
    // AWS pattern: ^d-[0-9a-f]{10}$
    assert!(
        dir_id.starts_with("d-"),
        "directory ID must start with 'd-', got: {dir_id}"
    );
    let suffix = &dir_id["d-".len()..];
    assert_eq!(
        suffix.len(),
        10,
        "directory ID suffix must be exactly 10 chars, got: {dir_id}"
    );
    assert!(
        suffix
            .chars()
            .all(|c| c.is_ascii_hexdigit() && !c.is_uppercase()),
        "directory ID suffix must be lowercase hex, got: {dir_id}"
    );
}

/// Verify that deleting a nonexistent directory returns EntityDoesNotExistException.
#[tokio::test]
async fn test_delete_directory_error_code() {
    let client = make_ds_client().await;

    let err = client
        .delete_directory()
        .directory_id("d-0000000000")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{err:?}");
    assert!(
        err_str.contains("EntityDoesNotExistException"),
        "Expected EntityDoesNotExistException, got: {err_str}"
    );
}

/// Verify that describing a nonexistent directory returns EntityDoesNotExistException.
#[tokio::test]
async fn test_describe_nonexistent_error_code() {
    let client = make_ds_client().await;

    let err = client
        .describe_directories()
        .directory_ids("d-0000000000")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{err:?}");
    assert!(
        err_str.contains("EntityDoesNotExistException"),
        "Expected EntityDoesNotExistException, got: {err_str}"
    );
}

/// Verify that ConnectDirectory fails when ConnectSettings is missing.
/// ConnectSettings is a required parameter per AWS docs.
#[tokio::test]
async fn test_connect_directory_missing_connect_settings_fails() {
    let client = make_ds_client().await;

    // The SDK requires ConnectSettings; building without it should either
    // fail at the SDK build step or at the mock with a ValidationException.
    let result = client
        .connect_directory()
        .name("no-settings.example.com")
        .password("NoSettings1!")
        .size(aws_sdk_directory::types::DirectorySize::Small)
        // ConnectSettings intentionally omitted
        .send()
        .await;

    assert!(
        result.is_err(),
        "connect_directory without ConnectSettings should fail"
    );
}

/// Verify that ConnectDirectory fails when CustomerDnsIps is empty.
/// At least one DNS IP is required by winterbaume validation.
#[tokio::test]
async fn test_connect_directory_no_dns_ips_fails() {
    let client = make_ds_client().await;

    let connect_settings = aws_sdk_directory::types::DirectoryConnectSettings::builder()
        .vpc_id("vpc-aabbccdd")
        .subnet_ids("subnet-11111111")
        .subnet_ids("subnet-22222222")
        // CustomerDnsIps intentionally empty
        .customer_user_name("Admin")
        .build()
        .unwrap();

    let result = client
        .connect_directory()
        .name("no-dns.example.com")
        .password("NoDns1234!")
        .size(aws_sdk_directory::types::DirectorySize::Small)
        .connect_settings(connect_settings)
        .send()
        .await;

    assert!(
        result.is_err(),
        "connect_directory with empty CustomerDnsIps should fail"
    );
}

/// Verify that LaunchTime and Stage are populated in DescribeDirectories response.
#[tokio::test]
async fn test_directory_timestamps_populated() {
    let client = make_ds_client().await;

    let dir_id = client
        .create_directory()
        .name("timestamps.example.com")
        .password("Timestamps1!")
        .size(aws_sdk_directory::types::DirectorySize::Small)
        .send()
        .await
        .unwrap()
        .directory_id()
        .unwrap()
        .to_string();

    let desc = client
        .describe_directories()
        .directory_ids(&dir_id)
        .send()
        .await
        .unwrap();

    let d = &desc.directory_descriptions()[0];
    assert!(d.launch_time().is_some(), "launch_time should be populated");
    assert!(
        d.stage_last_updated_date_time().is_some(),
        "stage_last_updated_date_time should be populated"
    );
    assert_eq!(
        d.stage(),
        Some(&aws_sdk_directory::types::DirectoryStage::Active),
        "stage should be Active"
    );
    assert!(!d.sso_enabled(), "sso_enabled should default to false");
}

/// Verify that CustomerDnsIps provided to ConnectDirectory are reflected in
/// DescribeDirectories under the ConnectSettings field.
#[tokio::test]
async fn test_connect_directory_dns_ips_reflected() {
    let client = make_ds_client().await;

    let connect_settings = aws_sdk_directory::types::DirectoryConnectSettings::builder()
        .vpc_id("vpc-dnstest")
        .subnet_ids("subnet-dns1111")
        .subnet_ids("subnet-dns2222")
        .customer_dns_ips("10.20.30.40")
        .customer_dns_ips("10.20.30.41")
        .customer_user_name("DnsAdmin")
        .build()
        .unwrap();

    let dir_id = client
        .connect_directory()
        .name("dns-reflect.example.com")
        .password("DnsRefl1234!")
        .size(aws_sdk_directory::types::DirectorySize::Small)
        .connect_settings(connect_settings)
        .send()
        .await
        .unwrap()
        .directory_id()
        .unwrap()
        .to_string();

    let desc = client
        .describe_directories()
        .directory_ids(&dir_id)
        .send()
        .await
        .unwrap();

    let d = &desc.directory_descriptions()[0];

    // DnsIpAddrs on the directory should reflect the CustomerDnsIps
    let dns_ips = d.dns_ip_addrs();
    assert!(
        dns_ips.contains(&"10.20.30.40".to_string()),
        "dns_ip_addrs should include 10.20.30.40"
    );
    assert!(
        dns_ips.contains(&"10.20.30.41".to_string()),
        "dns_ip_addrs should include 10.20.30.41"
    );

    // ConnectSettings should have the correct vpc_id
    let cs = d.connect_settings().expect("should have connect_settings");
    assert_eq!(cs.vpc_id(), Some("vpc-dnstest"));
    assert_eq!(cs.customer_user_name(), Some("DnsAdmin"));
}

/// Verify behavior when DescribeDirectories is called with an empty DirectoryIds array.
/// Per AWS docs: "An empty list results in an InvalidParameterException being thrown."
/// The SDK builder omits the field entirely when no IDs are provided, so passing no
/// directory_ids is the "describe all" path (None on the wire), not the empty-array path.
/// The empty-array validation is covered by the unit test in handlers::tests.
#[tokio::test]
async fn test_describe_directories_empty_array_behavior() {
    let client = make_ds_client().await;

    // Create one directory so we can observe what the mock returns
    client
        .create_directory()
        .name("empty-arr.example.com")
        .password("EmptyArr1!")
        .size(aws_sdk_directory::types::DirectorySize::Small)
        .send()
        .await
        .unwrap();

    // The SDK does not let us pass an explicitly empty DirectoryIds list
    // through the builder (it omits the field), so we call without any
    // directory_ids — which is the same as None on the wire — and expect
    // all directories to be returned.
    let resp = client
        .describe_directories()
        .send()
        .await
        .expect("describe_directories with no IDs should return all directories");

    // winterbaume returns all directories when DirectoryIds is omitted/null
    assert!(
        !resp.directory_descriptions().is_empty(),
        "should return at least one directory when DirectoryIds omitted"
    );
}

#[tokio::test]
async fn test_unimplemented_operations_return_error() {
    let client = make_ds_client().await;

    // GetDirectoryLimits is listed as a stub (NotImplementedError)
    let result = client.get_directory_limits().send().await;
    assert!(
        result.is_err(),
        "unimplemented GetDirectoryLimits should return error"
    );

    // ListCertificates is also a stub; requires a directory_id
    // First create a directory to satisfy any ID requirement
    let dir_id = client
        .create_directory()
        .name("stub-test.example.com")
        .password("StubTest1!")
        .size(aws_sdk_directory::types::DirectorySize::Small)
        .send()
        .await
        .unwrap()
        .directory_id()
        .unwrap()
        .to_string();

    let result = client
        .list_certificates()
        .directory_id(&dir_id)
        .send()
        .await;
    assert!(
        result.is_err(),
        "unimplemented ListCertificates should return error"
    );
}
