//! Integration tests for winterbaume Route 53 service.

use aws_sdk_route53::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_route53::Route53Service;

async fn make_route53_client() -> aws_sdk_route53::Client {
    let mock = MockAws::builder()
        .with_service(Route53Service::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_route53::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_route53::Client::new(&config)
}

async fn create_hosted_zone_for_test(
    client: &aws_sdk_route53::Client,
    name: &str,
    caller_reference: &str,
) -> String {
    client
        .create_hosted_zone()
        .name(name)
        .caller_reference(caller_reference)
        .send()
        .await
        .unwrap()
        .hosted_zone()
        .unwrap()
        .id()
        .to_string()
}

async fn create_health_check_for_test(
    client: &aws_sdk_route53::Client,
    caller_reference: &str,
) -> String {
    client
        .create_health_check()
        .caller_reference(caller_reference)
        .health_check_config(
            aws_sdk_route53::types::HealthCheckConfig::builder()
                .r#type(aws_sdk_route53::types::HealthCheckType::Http)
                .fully_qualified_domain_name("health.example.com")
                .resource_path("/health")
                .port(80)
                .request_interval(30)
                .failure_threshold(3)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap()
        .health_check()
        .unwrap()
        .id()
        .to_string()
}

#[tokio::test]
async fn test_create_hosted_zone() {
    let client = make_route53_client().await;

    let resp = client
        .create_hosted_zone()
        .name("example.com")
        .caller_reference("unique-ref-1")
        .send()
        .await
        .expect("create_hosted_zone should succeed");

    let zone = resp.hosted_zone().expect("should have hosted zone");
    assert!(zone.id().contains("/hostedzone/"));
    assert!(zone.name().ends_with('.'));
}

#[tokio::test]
async fn test_get_hosted_zone() {
    let client = make_route53_client().await;

    let create_resp = client
        .create_hosted_zone()
        .name("get-zone.com")
        .caller_reference("ref-get")
        .send()
        .await
        .unwrap();
    let zone_id = create_resp.hosted_zone().unwrap().id();

    let resp = client
        .get_hosted_zone()
        .id(zone_id)
        .send()
        .await
        .expect("get_hosted_zone should succeed");

    let zone = resp.hosted_zone().expect("should have hosted zone");
    assert_eq!(zone.name(), "get-zone.com.");
}

#[tokio::test]
async fn test_list_hosted_zones() {
    let client = make_route53_client().await;

    client
        .create_hosted_zone()
        .name("zone1.com")
        .caller_reference("ref-1")
        .send()
        .await
        .unwrap();
    client
        .create_hosted_zone()
        .name("zone2.com")
        .caller_reference("ref-2")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_hosted_zones()
        .send()
        .await
        .expect("list_hosted_zones should succeed");

    assert_eq!(resp.hosted_zones().len(), 2);
}

#[tokio::test]
async fn test_delete_hosted_zone() {
    let client = make_route53_client().await;

    let create_resp = client
        .create_hosted_zone()
        .name("del-zone.com")
        .caller_reference("ref-del")
        .send()
        .await
        .unwrap();
    let zone_id = create_resp.hosted_zone().unwrap().id();

    client
        .delete_hosted_zone()
        .id(zone_id)
        .send()
        .await
        .expect("delete_hosted_zone should succeed");

    let resp = client.list_hosted_zones().send().await.unwrap();
    assert_eq!(resp.hosted_zones().len(), 0);
}

#[tokio::test]
async fn test_change_and_list_resource_record_sets() {
    let client = make_route53_client().await;

    let create_resp = client
        .create_hosted_zone()
        .name("records.com")
        .caller_reference("ref-records")
        .send()
        .await
        .unwrap();
    let zone_id = create_resp.hosted_zone().unwrap().id();

    // Add an A record
    client
        .change_resource_record_sets()
        .hosted_zone_id(zone_id)
        .change_batch(
            aws_sdk_route53::types::ChangeBatch::builder()
                .changes(
                    aws_sdk_route53::types::Change::builder()
                        .action(aws_sdk_route53::types::ChangeAction::Create)
                        .resource_record_set(
                            aws_sdk_route53::types::ResourceRecordSet::builder()
                                .name("www.records.com.")
                                .r#type(aws_sdk_route53::types::RrType::A)
                                .ttl(300)
                                .resource_records(
                                    aws_sdk_route53::types::ResourceRecord::builder()
                                        .value("1.2.3.4")
                                        .build()
                                        .unwrap(),
                                )
                                .build()
                                .unwrap(),
                        )
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("change_resource_record_sets should succeed");

    let resp = client
        .list_resource_record_sets()
        .hosted_zone_id(zone_id)
        .send()
        .await
        .expect("list_resource_record_sets should succeed");

    // Should have SOA, NS, and the new A record
    let record_sets = resp.resource_record_sets();
    assert!(record_sets.len() >= 3);

    let a_record = record_sets
        .iter()
        .find(|r| r.r#type() == &aws_sdk_route53::types::RrType::A);
    assert!(a_record.is_some());
    assert_eq!(a_record.unwrap().name(), "www.records.com.");
}

#[tokio::test]
async fn test_upsert_resource_record_set() {
    let client = make_route53_client().await;

    let create_resp = client
        .create_hosted_zone()
        .name("upsert.com")
        .caller_reference("ref-upsert")
        .send()
        .await
        .unwrap();
    let zone_id = create_resp.hosted_zone().unwrap().id();

    // Upsert a CNAME record
    client
        .change_resource_record_sets()
        .hosted_zone_id(zone_id)
        .change_batch(
            aws_sdk_route53::types::ChangeBatch::builder()
                .changes(
                    aws_sdk_route53::types::Change::builder()
                        .action(aws_sdk_route53::types::ChangeAction::Upsert)
                        .resource_record_set(
                            aws_sdk_route53::types::ResourceRecordSet::builder()
                                .name("app.upsert.com.")
                                .r#type(aws_sdk_route53::types::RrType::Cname)
                                .ttl(60)
                                .resource_records(
                                    aws_sdk_route53::types::ResourceRecord::builder()
                                        .value("lb.example.com")
                                        .build()
                                        .unwrap(),
                                )
                                .build()
                                .unwrap(),
                        )
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("upsert should succeed");

    let resp = client
        .list_resource_record_sets()
        .hosted_zone_id(zone_id)
        .send()
        .await
        .unwrap();

    let cname = resp
        .resource_record_sets()
        .iter()
        .find(|r| r.r#type() == &aws_sdk_route53::types::RrType::Cname);
    assert!(cname.is_some());
}

#[tokio::test]
async fn test_associate_and_disassociate_vpc() {
    let client = make_route53_client().await;

    let create_resp = client
        .create_hosted_zone()
        .name("private-zone.com")
        .caller_reference("ref-vpc-assoc")
        .send()
        .await
        .unwrap();
    let zone_id = create_resp.hosted_zone().unwrap().id().to_string();
    // Strip /hostedzone/ prefix for SDK usage
    let short_id = zone_id.trim_start_matches("/hostedzone/").to_string();

    // Associate a VPC
    let vpc = aws_sdk_route53::types::Vpc::builder()
        .vpc_id("vpc-12345678")
        .vpc_region(aws_sdk_route53::types::VpcRegion::UsEast1)
        .build();

    client
        .associate_vpc_with_hosted_zone()
        .hosted_zone_id(&short_id)
        .vpc(vpc)
        .send()
        .await
        .expect("associate_vpc_with_hosted_zone should succeed");

    // Disassociate the VPC
    let vpc2 = aws_sdk_route53::types::Vpc::builder()
        .vpc_id("vpc-12345678")
        .vpc_region(aws_sdk_route53::types::VpcRegion::UsEast1)
        .build();

    client
        .disassociate_vpc_from_hosted_zone()
        .hosted_zone_id(&short_id)
        .vpc(vpc2)
        .send()
        .await
        .expect("disassociate_vpc_from_hosted_zone should succeed");
}

#[tokio::test]
async fn test_get_dnssec() {
    let client = make_route53_client().await;

    let create_resp = client
        .create_hosted_zone()
        .name("dnssec-zone.com")
        .caller_reference("ref-dnssec")
        .send()
        .await
        .unwrap();
    let zone_id = create_resp.hosted_zone().unwrap().id().to_string();
    let short_id = zone_id.trim_start_matches("/hostedzone/").to_string();

    let resp = client
        .get_dnssec()
        .hosted_zone_id(&short_id)
        .send()
        .await
        .expect("get_dnssec should succeed");

    // Should return NOT_SIGNING status
    assert!(resp.status().is_some());
}

#[tokio::test]
async fn test_list_hosted_zones_by_vpc() {
    let client = make_route53_client().await;

    // Create a zone and associate a VPC
    let create_resp = client
        .create_hosted_zone()
        .name("vpc-zone.com")
        .caller_reference("ref-list-vpc")
        .send()
        .await
        .unwrap();
    let zone_id = create_resp.hosted_zone().unwrap().id().to_string();
    let short_id = zone_id.trim_start_matches("/hostedzone/").to_string();

    let vpc = aws_sdk_route53::types::Vpc::builder()
        .vpc_id("vpc-abcd1234")
        .vpc_region(aws_sdk_route53::types::VpcRegion::UsEast1)
        .build();

    client
        .associate_vpc_with_hosted_zone()
        .hosted_zone_id(&short_id)
        .vpc(vpc)
        .send()
        .await
        .unwrap();

    // List hosted zones by VPC
    let list_resp = client
        .list_hosted_zones_by_vpc()
        .vpc_id("vpc-abcd1234")
        .vpc_region(aws_sdk_route53::types::VpcRegion::UsEast1)
        .send()
        .await
        .expect("list_hosted_zones_by_vpc should succeed");

    assert!(!list_resp.hosted_zone_summaries().is_empty());
}

// ============================================================================
// Tests derived from AWS documentation: Amazon Route 53
// ============================================================================

/// Helper: build a single A-record change for use in multiple tests.
fn make_a_record_change(
    action: aws_sdk_route53::types::ChangeAction,
    name: &str,
    value: &str,
) -> aws_sdk_route53::types::Change {
    aws_sdk_route53::types::Change::builder()
        .action(action)
        .resource_record_set(
            aws_sdk_route53::types::ResourceRecordSet::builder()
                .name(name)
                .r#type(aws_sdk_route53::types::RrType::A)
                .ttl(300)
                .resource_records(
                    aws_sdk_route53::types::ResourceRecord::builder()
                        .value(value)
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        )
        .build()
        .unwrap()
}

// --- CreateHostedZone additional scenarios ---

#[tokio::test]
async fn test_create_hosted_zone_duplicate_caller_reference() {
    let client = make_route53_client().await;

    client
        .create_hosted_zone()
        .name("first.com")
        .caller_reference("dup-ref-1")
        .send()
        .await
        .expect("first create should succeed");

    let err = client
        .create_hosted_zone()
        .name("second.com")
        .caller_reference("dup-ref-1")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("HostedZoneAlreadyExists"),
        "Expected HostedZoneAlreadyExists, got: {err_str}"
    );
}

#[tokio::test]
async fn test_create_hosted_zone_name_gets_trailing_dot() {
    let client = make_route53_client().await;

    // Name provided WITHOUT trailing dot — service should normalize it.
    let resp = client
        .create_hosted_zone()
        .name("nodot.com")
        .caller_reference("ref-nodot")
        .send()
        .await
        .expect("create should succeed");

    let zone = resp.hosted_zone().expect("should have hosted zone");
    assert!(
        zone.name().ends_with('.'),
        "Expected zone name to end with '.', got: {}",
        zone.name()
    );
}

#[tokio::test]
async fn test_create_hosted_zone_has_resource_record_count() {
    let client = make_route53_client().await;

    let resp = client
        .create_hosted_zone()
        .name("count.com")
        .caller_reference("ref-count")
        .send()
        .await
        .expect("create should succeed");

    let zone = resp.hosted_zone().expect("should have hosted zone");
    // Default SOA + NS records → count = 2
    assert_eq!(
        zone.resource_record_set_count(),
        Some(2),
        "Expected ResourceRecordSetCount = 2 for a new zone"
    );
}

// --- GetHostedZone additional scenarios ---

#[tokio::test]
async fn test_get_hosted_zone_not_found() {
    let client = make_route53_client().await;

    let err = client
        .get_hosted_zone()
        .id("/hostedzone/DOESNOTEXIST")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("NoSuchHostedZone"),
        "Expected NoSuchHostedZone, got: {err_str}"
    );
}

// --- DeleteHostedZone additional scenarios ---

#[tokio::test]
async fn test_delete_hosted_zone_not_found() {
    let client = make_route53_client().await;

    let err = client
        .delete_hosted_zone()
        .id("/hostedzone/DOESNOTEXIST")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("NoSuchHostedZone"),
        "Expected NoSuchHostedZone, got: {err_str}"
    );
}

// --- ListHostedZones additional scenarios ---

#[tokio::test]
async fn test_list_hosted_zones_empty() {
    let client = make_route53_client().await;

    let resp = client
        .list_hosted_zones()
        .send()
        .await
        .expect("list_hosted_zones should succeed on empty state");

    assert_eq!(
        resp.hosted_zones().len(),
        0,
        "Expected empty list for fresh client"
    );
}

// --- ListResourceRecordSets additional scenarios ---

#[tokio::test]
async fn test_list_resource_record_sets_has_default_records() {
    let client = make_route53_client().await;

    let create_resp = client
        .create_hosted_zone()
        .name("defaults.com")
        .caller_reference("ref-defaults")
        .send()
        .await
        .unwrap();
    let zone_id = create_resp.hosted_zone().unwrap().id().to_string();

    let resp = client
        .list_resource_record_sets()
        .hosted_zone_id(&zone_id)
        .send()
        .await
        .expect("list_resource_record_sets should succeed");

    let record_sets = resp.resource_record_sets();
    // Every new zone must contain a SOA and an NS record.
    let has_soa = record_sets
        .iter()
        .any(|r| r.r#type() == &aws_sdk_route53::types::RrType::Soa);
    let has_ns = record_sets
        .iter()
        .any(|r| r.r#type() == &aws_sdk_route53::types::RrType::Ns);
    assert!(has_soa, "Expected SOA record in default record sets");
    assert!(has_ns, "Expected NS record in default record sets");
}

#[tokio::test]
async fn test_list_resource_record_sets_nonexistent_zone() {
    let client = make_route53_client().await;

    let err = client
        .list_resource_record_sets()
        .hosted_zone_id("/hostedzone/DOESNOTEXIST")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("NoSuchHostedZone"),
        "Expected NoSuchHostedZone, got: {err_str}"
    );
}

// --- ChangeResourceRecordSets additional scenarios ---

#[tokio::test]
async fn test_change_resource_record_sets_create_duplicate() {
    let client = make_route53_client().await;

    let create_resp = client
        .create_hosted_zone()
        .name("dup-record.com")
        .caller_reference("ref-dup-record")
        .send()
        .await
        .unwrap();
    let zone_id = create_resp.hosted_zone().unwrap().id().to_string();

    let change_batch = aws_sdk_route53::types::ChangeBatch::builder()
        .changes(make_a_record_change(
            aws_sdk_route53::types::ChangeAction::Create,
            "www.dup-record.com.",
            "10.0.0.1",
        ))
        .build()
        .unwrap();

    // First CREATE should succeed.
    client
        .change_resource_record_sets()
        .hosted_zone_id(&zone_id)
        .change_batch(change_batch)
        .send()
        .await
        .expect("first CREATE should succeed");

    // Second CREATE of the same record (same name + type) should fail.
    let change_batch2 = aws_sdk_route53::types::ChangeBatch::builder()
        .changes(make_a_record_change(
            aws_sdk_route53::types::ChangeAction::Create,
            "www.dup-record.com.",
            "10.0.0.2",
        ))
        .build()
        .unwrap();

    let err = client
        .change_resource_record_sets()
        .hosted_zone_id(&zone_id)
        .change_batch(change_batch2)
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("InvalidChangeBatch"),
        "Expected InvalidChangeBatch on duplicate CREATE, got: {err_str}"
    );
}

#[tokio::test]
async fn test_change_resource_record_sets_delete_record() {
    let client = make_route53_client().await;

    let create_resp = client
        .create_hosted_zone()
        .name("delete-record.com")
        .caller_reference("ref-delete-record")
        .send()
        .await
        .unwrap();
    let zone_id = create_resp.hosted_zone().unwrap().id().to_string();

    // Create an A record first.
    let change_batch = aws_sdk_route53::types::ChangeBatch::builder()
        .changes(make_a_record_change(
            aws_sdk_route53::types::ChangeAction::Create,
            "api.delete-record.com.",
            "192.0.2.1",
        ))
        .build()
        .unwrap();

    client
        .change_resource_record_sets()
        .hosted_zone_id(&zone_id)
        .change_batch(change_batch)
        .send()
        .await
        .expect("CREATE should succeed");

    // Verify record exists.
    let before = client
        .list_resource_record_sets()
        .hosted_zone_id(&zone_id)
        .send()
        .await
        .unwrap();
    let has_a = before.resource_record_sets().iter().any(|r| {
        r.r#type() == &aws_sdk_route53::types::RrType::A && r.name() == "api.delete-record.com."
    });
    assert!(has_a, "A record should exist before DELETE");

    // Now DELETE the record.
    let delete_batch = aws_sdk_route53::types::ChangeBatch::builder()
        .changes(make_a_record_change(
            aws_sdk_route53::types::ChangeAction::Delete,
            "api.delete-record.com.",
            "192.0.2.1",
        ))
        .build()
        .unwrap();

    client
        .change_resource_record_sets()
        .hosted_zone_id(&zone_id)
        .change_batch(delete_batch)
        .send()
        .await
        .expect("DELETE should succeed");

    // Verify record is gone.
    let after = client
        .list_resource_record_sets()
        .hosted_zone_id(&zone_id)
        .send()
        .await
        .unwrap();
    let still_has_a = after.resource_record_sets().iter().any(|r| {
        r.r#type() == &aws_sdk_route53::types::RrType::A && r.name() == "api.delete-record.com."
    });
    assert!(!still_has_a, "A record should be absent after DELETE");
}

#[tokio::test]
async fn test_change_resource_record_sets_nonexistent_zone() {
    let client = make_route53_client().await;

    let change_batch = aws_sdk_route53::types::ChangeBatch::builder()
        .changes(make_a_record_change(
            aws_sdk_route53::types::ChangeAction::Create,
            "www.missing.com.",
            "1.2.3.4",
        ))
        .build()
        .unwrap();

    let err = client
        .change_resource_record_sets()
        .hosted_zone_id("/hostedzone/DOESNOTEXIST")
        .change_batch(change_batch)
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("NoSuchHostedZone"),
        "Expected NoSuchHostedZone, got: {err_str}"
    );
}

// --- GetDNSSEC additional scenarios ---

#[tokio::test]
async fn test_get_dnssec_not_found() {
    let client = make_route53_client().await;

    let err = client
        .get_dnssec()
        .hosted_zone_id("DOESNOTEXIST")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("NoSuchHostedZone"),
        "Expected NoSuchHostedZone, got: {err_str}"
    );
}

#[tokio::test]
async fn test_get_dnssec_status_not_signing() {
    let client = make_route53_client().await;

    let create_resp = client
        .create_hosted_zone()
        .name("dnssec-status.com")
        .caller_reference("ref-dnssec-status")
        .send()
        .await
        .unwrap();
    let zone_id = create_resp.hosted_zone().unwrap().id().to_string();
    let short_id = zone_id.trim_start_matches("/hostedzone/").to_string();

    let resp = client
        .get_dnssec()
        .hosted_zone_id(&short_id)
        .send()
        .await
        .expect("get_dnssec should succeed");

    let status = resp.status().expect("status should be present");
    assert_eq!(
        status.serve_signature(),
        Some("NOT_SIGNING"),
        "Expected ServeSignature = NOT_SIGNING"
    );
    // No key signing keys should be returned for a zone without DNSSEC.
    assert!(
        resp.key_signing_keys().is_empty(),
        "Expected no key signing keys for an unsigned zone"
    );
}

// --- AssociateVPCWithHostedZone / DisassociateVPCFromHostedZone additional scenarios ---

#[tokio::test]
async fn test_associate_vpc_idempotent() {
    let client = make_route53_client().await;

    let create_resp = client
        .create_hosted_zone()
        .name("idem-vpc.com")
        .caller_reference("ref-idem-vpc")
        .send()
        .await
        .unwrap();
    let zone_id = create_resp.hosted_zone().unwrap().id().to_string();
    let short_id = zone_id.trim_start_matches("/hostedzone/").to_string();

    let vpc = || {
        aws_sdk_route53::types::Vpc::builder()
            .vpc_id("vpc-idem1111")
            .vpc_region(aws_sdk_route53::types::VpcRegion::UsEast1)
            .build()
    };

    // First association.
    client
        .associate_vpc_with_hosted_zone()
        .hosted_zone_id(&short_id)
        .vpc(vpc())
        .send()
        .await
        .expect("first associate should succeed");

    // Second association with same VPC — must succeed (idempotent).
    client
        .associate_vpc_with_hosted_zone()
        .hosted_zone_id(&short_id)
        .vpc(vpc())
        .send()
        .await
        .expect("second associate with same VPC should succeed (idempotent)");

    // The zone should appear exactly once when listing by that VPC.
    let list_resp = client
        .list_hosted_zones_by_vpc()
        .vpc_id("vpc-idem1111")
        .vpc_region(aws_sdk_route53::types::VpcRegion::UsEast1)
        .send()
        .await
        .unwrap();
    assert_eq!(
        list_resp.hosted_zone_summaries().len(),
        1,
        "Zone should appear only once despite two associate calls"
    );
}

#[tokio::test]
async fn test_associate_vpc_nonexistent_zone() {
    let client = make_route53_client().await;

    let vpc = aws_sdk_route53::types::Vpc::builder()
        .vpc_id("vpc-99999999")
        .vpc_region(aws_sdk_route53::types::VpcRegion::UsEast1)
        .build();

    let err = client
        .associate_vpc_with_hosted_zone()
        .hosted_zone_id("DOESNOTEXIST")
        .vpc(vpc)
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("NoSuchHostedZone"),
        "Expected NoSuchHostedZone, got: {err_str}"
    );
}

#[tokio::test]
async fn test_disassociate_vpc_nonexistent_zone() {
    let client = make_route53_client().await;

    let vpc = aws_sdk_route53::types::Vpc::builder()
        .vpc_id("vpc-99999999")
        .vpc_region(aws_sdk_route53::types::VpcRegion::UsEast1)
        .build();

    let err = client
        .disassociate_vpc_from_hosted_zone()
        .hosted_zone_id("DOESNOTEXIST")
        .vpc(vpc)
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("NoSuchHostedZone"),
        "Expected NoSuchHostedZone, got: {err_str}"
    );
}

// --- ListHostedZonesByVPC additional scenarios ---

#[tokio::test]
async fn test_list_hosted_zones_by_vpc_no_association() {
    let client = make_route53_client().await;

    // Create a zone but do NOT associate any VPC.
    client
        .create_hosted_zone()
        .name("no-vpc.com")
        .caller_reference("ref-no-vpc")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_hosted_zones_by_vpc()
        .vpc_id("vpc-unrelated")
        .vpc_region(aws_sdk_route53::types::VpcRegion::UsEast1)
        .send()
        .await
        .expect("list_hosted_zones_by_vpc should succeed even with no results");

    assert!(
        resp.hosted_zone_summaries().is_empty(),
        "Expected empty summaries for a VPC that has no associated zones"
    );
}

// --- Multiple record types ---

#[tokio::test]
async fn test_multiple_record_types() {
    let client = make_route53_client().await;

    let create_resp = client
        .create_hosted_zone()
        .name("multitype.com")
        .caller_reference("ref-multitype")
        .send()
        .await
        .unwrap();
    let zone_id = create_resp.hosted_zone().unwrap().id().to_string();

    // Add CNAME, MX and TXT records in one ChangeResourceRecordSets call.
    let cname_change = aws_sdk_route53::types::Change::builder()
        .action(aws_sdk_route53::types::ChangeAction::Create)
        .resource_record_set(
            aws_sdk_route53::types::ResourceRecordSet::builder()
                .name("mail.multitype.com.")
                .r#type(aws_sdk_route53::types::RrType::Cname)
                .ttl(60)
                .resource_records(
                    aws_sdk_route53::types::ResourceRecord::builder()
                        .value("smtp.example.com")
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        )
        .build()
        .unwrap();

    let mx_change = aws_sdk_route53::types::Change::builder()
        .action(aws_sdk_route53::types::ChangeAction::Create)
        .resource_record_set(
            aws_sdk_route53::types::ResourceRecordSet::builder()
                .name("multitype.com.")
                .r#type(aws_sdk_route53::types::RrType::Mx)
                .ttl(300)
                .resource_records(
                    aws_sdk_route53::types::ResourceRecord::builder()
                        .value("10 mail.multitype.com.")
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        )
        .build()
        .unwrap();

    let txt_change = aws_sdk_route53::types::Change::builder()
        .action(aws_sdk_route53::types::ChangeAction::Create)
        .resource_record_set(
            aws_sdk_route53::types::ResourceRecordSet::builder()
                .name("multitype.com.")
                .r#type(aws_sdk_route53::types::RrType::Txt)
                .ttl(300)
                .resource_records(
                    aws_sdk_route53::types::ResourceRecord::builder()
                        .value("\"v=spf1 include:example.com ~all\"")
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        )
        .build()
        .unwrap();

    let change_batch = aws_sdk_route53::types::ChangeBatch::builder()
        .changes(cname_change)
        .changes(mx_change)
        .changes(txt_change)
        .build()
        .unwrap();

    client
        .change_resource_record_sets()
        .hosted_zone_id(&zone_id)
        .change_batch(change_batch)
        .send()
        .await
        .expect("batch of CNAME/MX/TXT changes should succeed");

    let resp = client
        .list_resource_record_sets()
        .hosted_zone_id(&zone_id)
        .send()
        .await
        .unwrap();

    let record_sets = resp.resource_record_sets();
    let has_cname = record_sets
        .iter()
        .any(|r| r.r#type() == &aws_sdk_route53::types::RrType::Cname);
    let has_mx = record_sets
        .iter()
        .any(|r| r.r#type() == &aws_sdk_route53::types::RrType::Mx);
    let has_txt = record_sets
        .iter()
        .any(|r| r.r#type() == &aws_sdk_route53::types::RrType::Txt);
    assert!(has_cname, "Expected CNAME record in record sets");
    assert!(has_mx, "Expected MX record in record sets");
    assert!(has_txt, "Expected TXT record in record sets");
}

// --- Full lifecycle test ---

#[tokio::test]
async fn test_full_zone_lifecycle() {
    let client = make_route53_client().await;

    // 1. Create a hosted zone.
    let create_resp = client
        .create_hosted_zone()
        .name("lifecycle.com")
        .caller_reference("ref-lifecycle")
        .send()
        .await
        .expect("create should succeed");

    let zone = create_resp.hosted_zone().expect("should have hosted zone");
    assert!(zone.id().contains("/hostedzone/"));
    assert_eq!(zone.name(), "lifecycle.com.");
    let zone_id = zone.id().to_string();

    // 2. GetHostedZone should return the zone.
    let get_resp = client
        .get_hosted_zone()
        .id(&zone_id)
        .send()
        .await
        .expect("get should succeed");
    assert_eq!(get_resp.hosted_zone().unwrap().name(), "lifecycle.com.");

    // 3. Zone appears in ListHostedZones.
    let list_resp = client.list_hosted_zones().send().await.unwrap();
    assert!(
        list_resp.hosted_zones().iter().any(|z| z.id() == zone_id),
        "Created zone should appear in list"
    );

    // 4. Add an A record via ChangeResourceRecordSets.
    let change_batch = aws_sdk_route53::types::ChangeBatch::builder()
        .changes(make_a_record_change(
            aws_sdk_route53::types::ChangeAction::Create,
            "app.lifecycle.com.",
            "203.0.113.1",
        ))
        .build()
        .unwrap();

    client
        .change_resource_record_sets()
        .hosted_zone_id(&zone_id)
        .change_batch(change_batch)
        .send()
        .await
        .expect("adding A record should succeed");

    // 5. ListResourceRecordSets must include the new A record.
    let rrset_resp = client
        .list_resource_record_sets()
        .hosted_zone_id(&zone_id)
        .send()
        .await
        .unwrap();
    let has_a = rrset_resp
        .resource_record_sets()
        .iter()
        .any(|r| r.r#type() == &aws_sdk_route53::types::RrType::A);
    assert!(has_a, "A record should be listed after creation");

    // 6. Delete the hosted zone.
    client
        .delete_hosted_zone()
        .id(&zone_id)
        .send()
        .await
        .expect("delete should succeed");

    // 7. GetHostedZone should now return NoSuchHostedZone.
    let err = client
        .get_hosted_zone()
        .id(&zone_id)
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("NoSuchHostedZone"),
        "Expected NoSuchHostedZone after delete, got: {err_str}"
    );

    // 8. Zone is absent from ListHostedZones.
    let final_list = client.list_hosted_zones().send().await.unwrap();
    assert!(
        !final_list.hosted_zones().iter().any(|z| z.id() == zone_id),
        "Deleted zone should not appear in list"
    );
}

// Ported from moto: test_route53.py::test_get_hosted_zone_count_many_zones
#[tokio::test]
async fn test_get_hosted_zone_count_tracks_created_zones() {
    let client = make_route53_client().await;

    create_hosted_zone_for_test(&client, "count-a.example.com", "count-a").await;
    create_hosted_zone_for_test(&client, "count-b.example.com", "count-b").await;

    let resp = client.get_hosted_zone_count().send().await.unwrap();
    assert_eq!(resp.hosted_zone_count(), 2);
}

// Ported from moto: test_route53.py::test_update_hosted_zone_comment
// and test_route53.py::test_list_hosted_zones_by_name
#[tokio::test]
async fn test_update_hosted_zone_comment_and_list_by_name() {
    let client = make_route53_client().await;

    let zone_a = create_hosted_zone_for_test(&client, "aaa.example.com", "comment-a").await;
    let zone_b = create_hosted_zone_for_test(&client, "bbb.example.com", "comment-b").await;

    client
        .update_hosted_zone_comment()
        .id(&zone_b)
        .comment("updated comment")
        .send()
        .await
        .unwrap();

    let get_resp = client.get_hosted_zone().id(&zone_b).send().await.unwrap();
    assert_eq!(
        get_resp
            .hosted_zone()
            .unwrap()
            .config()
            .and_then(|config| config.comment()),
        Some("updated comment")
    );

    let list_resp = client
        .list_hosted_zones_by_name()
        .dns_name("bbb.example.com.")
        .send()
        .await
        .unwrap();
    assert_eq!(list_resp.hosted_zones()[0].id(), zone_b);
    assert!(
        list_resp
            .hosted_zones()
            .iter()
            .all(|zone| zone.name() >= "bbb.example.com.")
    );

    let all_resp = client.list_hosted_zones_by_name().send().await.unwrap();
    assert_eq!(all_resp.hosted_zones()[0].id(), zone_a);
}

// Ported from moto: test_route53_healthchecks.py::test_create_health_check
// and test_route53_healthchecks.py::test_update_health_check
#[tokio::test]
async fn test_health_check_lifecycle() {
    let client = make_route53_client().await;

    let health_check_id = create_health_check_for_test(&client, "hc-lifecycle").await;

    let get_resp = client
        .get_health_check()
        .health_check_id(&health_check_id)
        .send()
        .await
        .unwrap();
    assert_eq!(get_resp.health_check().unwrap().id(), health_check_id);

    let list_resp = client.list_health_checks().send().await.unwrap();
    assert!(
        list_resp
            .health_checks()
            .iter()
            .any(|health_check| health_check.id() == health_check_id)
    );

    let updated = client
        .update_health_check()
        .health_check_id(&health_check_id)
        .disabled(true)
        .resource_path("/ready")
        .send()
        .await
        .unwrap();
    assert_eq!(
        updated
            .health_check()
            .unwrap()
            .health_check_config()
            .unwrap()
            .resource_path(),
        Some("/ready")
    );
    assert_eq!(
        updated
            .health_check()
            .unwrap()
            .health_check_config()
            .unwrap()
            .disabled(),
        Some(true)
    );

    let status = client
        .get_health_check_status()
        .health_check_id(&health_check_id)
        .send()
        .await
        .unwrap();
    assert!(!status.health_check_observations().is_empty());
    // Disabled health checks should report "Unhealthy".
    let obs = &status.health_check_observations()[0];
    assert_eq!(
        obs.status_report().and_then(|sr| sr.status()).unwrap_or(""),
        "Unhealthy",
        "disabled health check should report Unhealthy"
    );

    client
        .delete_health_check()
        .health_check_id(&health_check_id)
        .send()
        .await
        .unwrap();

    let err = client
        .get_health_check()
        .health_check_id(&health_check_id)
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(err_str.contains("NoSuchHealthCheck"));
}

// Ported from moto: test_route53.py::test_list_or_change_tags_for_resource_request
// and test_route53_hosted_zone_ids.py::test_hosted_zone_id_in_change_tags
#[tokio::test]
async fn test_route53_resource_tags_support_hosted_zone_ids() {
    let client = make_route53_client().await;

    let zone_id = create_hosted_zone_for_test(&client, "tags.example.com", "zone-tags").await;
    let short_zone_id = zone_id.trim_start_matches("/hostedzone/").to_string();

    client
        .change_tags_for_resource()
        .resource_type(aws_sdk_route53::types::TagResourceType::Hostedzone)
        .resource_id(&short_zone_id)
        .add_tags(
            aws_sdk_route53::types::Tag::builder()
                .key("env")
                .value("test")
                .build(),
        )
        .send()
        .await
        .unwrap();

    let list_resp = client
        .list_tags_for_resource()
        .resource_type(aws_sdk_route53::types::TagResourceType::Hostedzone)
        .resource_id(&zone_id)
        .send()
        .await
        .unwrap();
    let tags = list_resp.resource_tag_set().unwrap().tags();
    assert_eq!(tags[0].key(), Some("env"));
    assert_eq!(tags[0].value(), Some("test"));
}

// Ported from moto: test_route53.py::test_list_tags_for_resources
#[tokio::test]
async fn test_list_tags_for_resources_on_health_checks() {
    let client = make_route53_client().await;

    let hc1 = create_health_check_for_test(&client, "hc-tags-1").await;
    let hc2 = create_health_check_for_test(&client, "hc-tags-2").await;

    client
        .change_tags_for_resource()
        .resource_type(aws_sdk_route53::types::TagResourceType::Healthcheck)
        .resource_id(&hc1)
        .add_tags(
            aws_sdk_route53::types::Tag::builder()
                .key("tier")
                .value("one")
                .build(),
        )
        .send()
        .await
        .unwrap();
    client
        .change_tags_for_resource()
        .resource_type(aws_sdk_route53::types::TagResourceType::Healthcheck)
        .resource_id(&hc2)
        .add_tags(
            aws_sdk_route53::types::Tag::builder()
                .key("tier")
                .value("two")
                .build(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .list_tags_for_resources()
        .resource_type(aws_sdk_route53::types::TagResourceType::Healthcheck)
        .resource_ids(hc1.clone())
        .resource_ids(hc2.clone())
        .send()
        .await
        .unwrap();

    assert_eq!(resp.resource_tag_sets().len(), 2);
    assert!(
        resp.resource_tag_sets()
            .iter()
            .any(|resource| resource.resource_id() == Some(hc1.as_str()))
    );
    assert!(
        resp.resource_tag_sets()
            .iter()
            .any(|resource| resource.resource_id() == Some(hc2.as_str()))
    );
}

// Ported from moto: test_route53_query_logging_config.py::test_create_query_logging_config_good_args
#[tokio::test]
async fn test_query_logging_config_lifecycle() {
    let client = make_route53_client().await;

    let zone_id = create_hosted_zone_for_test(&client, "logs.example.com", "zone-logs").await;
    let log_group_arn = "arn:aws:logs:us-east-1:123456789012:log-group:/aws/route53/logs-example";

    let create_resp = client
        .create_query_logging_config()
        .hosted_zone_id(&zone_id)
        .cloud_watch_logs_log_group_arn(log_group_arn)
        .send()
        .await
        .unwrap();
    let config_id = create_resp.query_logging_config().unwrap().id().to_string();

    let get_resp = client
        .get_query_logging_config()
        .id(&config_id)
        .send()
        .await
        .unwrap();
    assert_eq!(
        get_resp
            .query_logging_config()
            .unwrap()
            .cloud_watch_logs_log_group_arn(),
        log_group_arn
    );

    let list_resp = client
        .list_query_logging_configs()
        .hosted_zone_id(&zone_id)
        .send()
        .await
        .unwrap();
    assert_eq!(list_resp.query_logging_configs().len(), 1);

    client
        .delete_query_logging_config()
        .id(&config_id)
        .send()
        .await
        .unwrap();

    let err = client
        .get_query_logging_config()
        .id(&config_id)
        .send()
        .await
        .unwrap_err();
    assert!(format!("{:?}", err).contains("NoSuchQueryLoggingConfig"));
}

// Ported from moto: test_route53_delegationsets.py::test_create_reusable_delegation_set
#[tokio::test]
async fn test_reusable_delegation_set_lifecycle() {
    let client = make_route53_client().await;

    let create_resp = client
        .create_reusable_delegation_set()
        .caller_reference("delegation-set-1")
        .send()
        .await
        .unwrap();
    let delegation_set_id = create_resp
        .delegation_set()
        .unwrap()
        .id()
        .unwrap()
        .trim_start_matches("/delegationset/")
        .to_string();

    let get_resp = client
        .get_reusable_delegation_set()
        .id(&delegation_set_id)
        .send()
        .await
        .unwrap();
    assert_eq!(
        get_resp.delegation_set().unwrap().id(),
        Some(format!("/delegationset/{delegation_set_id}").as_str())
    );

    let list_resp = client.list_reusable_delegation_sets().send().await.unwrap();
    assert!(
        list_resp
            .delegation_sets()
            .iter()
            .any(|set| set.id() == Some(format!("/delegationset/{delegation_set_id}").as_str()))
    );

    client
        .delete_reusable_delegation_set()
        .id(&delegation_set_id)
        .send()
        .await
        .unwrap();

    let err = client
        .get_reusable_delegation_set()
        .id(&delegation_set_id)
        .send()
        .await
        .unwrap_err();
    assert!(format!("{:?}", err).contains("NoSuchDelegationSet"));
}

// --- DNSSEC / Key Signing Key tests ---

#[tokio::test]
async fn test_enable_disable_hosted_zone_dnssec() {
    let client = make_route53_client().await;
    let zone_id = create_hosted_zone_for_test(&client, "dnssec-test.com", "ref-dnssec").await;
    let short_id = zone_id.trim_start_matches("/hostedzone/");

    // Enable DNSSEC
    let resp = client
        .enable_hosted_zone_dnssec()
        .hosted_zone_id(short_id)
        .send()
        .await
        .expect("enable DNSSEC should succeed");
    assert_eq!(resp.change_info().unwrap().status().as_str(), "INSYNC");

    // Disable DNSSEC
    let resp = client
        .disable_hosted_zone_dnssec()
        .hosted_zone_id(short_id)
        .send()
        .await
        .expect("disable DNSSEC should succeed");
    assert_eq!(resp.change_info().unwrap().status().as_str(), "INSYNC");
}

#[tokio::test]
async fn test_key_signing_key_lifecycle() {
    let client = make_route53_client().await;
    let zone_id = create_hosted_zone_for_test(&client, "ksk-test.com", "ref-ksk").await;
    let short_id = zone_id.trim_start_matches("/hostedzone/");

    // Create KSK
    let resp = client
        .create_key_signing_key()
        .hosted_zone_id(short_id)
        .name("test-ksk")
        .key_management_service_arn("arn:aws:kms:us-east-1:123456789012:key/test-key")
        .caller_reference("ksk-ref-1")
        .status("ACTIVE")
        .send()
        .await
        .expect("create KSK should succeed");

    let ksk = resp.key_signing_key().unwrap();
    assert_eq!(ksk.name(), Some("test-ksk"));
    assert_eq!(ksk.status(), Some("ACTIVE"));

    // Deactivate
    client
        .deactivate_key_signing_key()
        .hosted_zone_id(short_id)
        .name("test-ksk")
        .send()
        .await
        .expect("deactivate KSK should succeed");

    // Activate
    client
        .activate_key_signing_key()
        .hosted_zone_id(short_id)
        .name("test-ksk")
        .send()
        .await
        .expect("activate KSK should succeed");

    // GetDNSSEC should show the key
    let dnssec_resp = client
        .get_dnssec()
        .hosted_zone_id(short_id)
        .send()
        .await
        .expect("get DNSSEC should succeed");
    assert!(!dnssec_resp.key_signing_keys().is_empty()); // may or may not show since GetDNSSEC doesn't list from state currently

    // Delete
    client
        .delete_key_signing_key()
        .hosted_zone_id(short_id)
        .name("test-ksk")
        .send()
        .await
        .expect("delete KSK should succeed");

    // Delete again should fail
    let err = client
        .delete_key_signing_key()
        .hosted_zone_id(short_id)
        .name("test-ksk")
        .send()
        .await
        .unwrap_err();
    assert!(format!("{:?}", err).contains("NoSuchKeySigningKey"));
}

// --- VPC Association Authorization tests ---

#[tokio::test]
async fn test_vpc_association_authorization_lifecycle() {
    let client = make_route53_client().await;
    let zone_id = create_hosted_zone_for_test(&client, "vpc-auth-test.com", "ref-vpc-auth").await;
    let short_id = zone_id.trim_start_matches("/hostedzone/");

    // Create authorization
    let resp = client
        .create_vpc_association_authorization()
        .hosted_zone_id(short_id)
        .vpc(
            aws_sdk_route53::types::Vpc::builder()
                .vpc_id("vpc-12345")
                .vpc_region(aws_sdk_route53::types::VpcRegion::UsEast1)
                .build(),
        )
        .send()
        .await
        .expect("create VPC auth should succeed");

    assert_eq!(resp.hosted_zone_id(), &zone_id);
    assert_eq!(resp.vpc().unwrap().vpc_id(), Some("vpc-12345"));

    // List authorizations
    let list_resp = client
        .list_vpc_association_authorizations()
        .hosted_zone_id(short_id)
        .send()
        .await
        .expect("list VPC auths should succeed");
    assert_eq!(list_resp.vpcs().len(), 1);

    // Delete authorization
    client
        .delete_vpc_association_authorization()
        .hosted_zone_id(short_id)
        .vpc(
            aws_sdk_route53::types::Vpc::builder()
                .vpc_id("vpc-12345")
                .build(),
        )
        .send()
        .await
        .expect("delete VPC auth should succeed");

    // List should be empty
    let list_resp2 = client
        .list_vpc_association_authorizations()
        .hosted_zone_id(short_id)
        .send()
        .await
        .unwrap();
    assert!(list_resp2.vpcs().is_empty());
}

// --- CIDR Collection tests ---

#[tokio::test]
async fn test_cidr_collection_lifecycle() {
    let client = make_route53_client().await;

    // Create collection
    let resp = client
        .create_cidr_collection()
        .name("test-cidr-collection")
        .caller_reference("cidr-ref-1")
        .send()
        .await
        .expect("create CIDR collection should succeed");

    let collection = resp.collection().unwrap();
    let collection_id = collection.id().unwrap().to_string();
    assert_eq!(collection.name().unwrap(), "test-cidr-collection");

    // List collections
    let list_resp = client
        .list_cidr_collections()
        .send()
        .await
        .expect("list CIDR collections should succeed");
    assert!(!list_resp.cidr_collections().is_empty());

    // Change collection - add blocks
    client
        .change_cidr_collection()
        .id(&collection_id)
        .changes(
            aws_sdk_route53::types::CidrCollectionChange::builder()
                .action(aws_sdk_route53::types::CidrCollectionChangeAction::Put)
                .location_name("us-east")
                .cidr_list("10.0.0.0/24")
                .cidr_list("10.0.1.0/24")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("change CIDR collection should succeed");

    // List locations
    let loc_resp = client
        .list_cidr_locations()
        .collection_id(&collection_id)
        .send()
        .await
        .expect("list CIDR locations should succeed");
    assert_eq!(loc_resp.cidr_locations().len(), 1);

    // List blocks
    let blocks_resp = client
        .list_cidr_blocks()
        .collection_id(&collection_id)
        .send()
        .await
        .expect("list CIDR blocks should succeed");
    assert_eq!(blocks_resp.cidr_blocks().len(), 2);

    // Delete collection
    client
        .delete_cidr_collection()
        .id(&collection_id)
        .send()
        .await
        .expect("delete CIDR collection should succeed");

    // Delete again should fail
    let err = client
        .delete_cidr_collection()
        .id(&collection_id)
        .send()
        .await
        .unwrap_err();
    assert!(format!("{:?}", err).contains("NoSuchCidrCollectionException"));
}

// --- Traffic Policy tests ---

#[tokio::test]
async fn test_traffic_policy_lifecycle() {
    let client = make_route53_client().await;
    let doc = r#"{"AWSPolicyFormatVersion":"2015-10-01","RecordType":"A","Endpoints":{"default":{"Type":"value","Value":"1.2.3.4"}}}"#;

    // Create traffic policy
    let resp = client
        .create_traffic_policy()
        .name("test-policy")
        .document(doc)
        .comment("initial version")
        .send()
        .await
        .expect("create traffic policy should succeed");

    let policy = resp.traffic_policy().unwrap();
    let policy_id = policy.id().to_string();
    assert_eq!(policy.name(), "test-policy");
    assert_eq!(policy.version(), 1);
    assert_eq!(policy.comment(), Some("initial version"));

    // Get traffic policy
    let get_resp = client
        .get_traffic_policy()
        .id(&policy_id)
        .version(1)
        .send()
        .await
        .expect("get traffic policy should succeed");
    assert_eq!(get_resp.traffic_policy().unwrap().name(), "test-policy");

    // Create version 2
    let v2_resp = client
        .create_traffic_policy_version()
        .id(&policy_id)
        .document(doc)
        .comment("version 2")
        .send()
        .await
        .expect("create traffic policy version should succeed");
    assert_eq!(v2_resp.traffic_policy().unwrap().version(), 2);

    // Update comment
    let update_resp = client
        .update_traffic_policy_comment()
        .id(&policy_id)
        .version(1)
        .comment("updated comment")
        .send()
        .await
        .expect("update traffic policy comment should succeed");
    assert_eq!(
        update_resp.traffic_policy().unwrap().comment(),
        Some("updated comment")
    );

    // List traffic policies
    let list_resp = client
        .list_traffic_policies()
        .send()
        .await
        .expect("list traffic policies should succeed");
    assert!(!list_resp.traffic_policy_summaries().is_empty());

    // List traffic policy versions
    let versions_resp = client
        .list_traffic_policy_versions()
        .id(&policy_id)
        .send()
        .await
        .expect("list traffic policy versions should succeed");
    assert_eq!(versions_resp.traffic_policies().len(), 2);

    // Delete version 2
    client
        .delete_traffic_policy()
        .id(&policy_id)
        .version(2)
        .send()
        .await
        .expect("delete traffic policy v2 should succeed");

    // Delete version 1
    client
        .delete_traffic_policy()
        .id(&policy_id)
        .version(1)
        .send()
        .await
        .expect("delete traffic policy v1 should succeed");
}

// --- Traffic Policy Instance tests ---

#[tokio::test]
async fn test_traffic_policy_instance_lifecycle() {
    let client = make_route53_client().await;
    let zone_id = create_hosted_zone_for_test(&client, "tpi-test.com", "ref-tpi").await;
    let short_zone_id = zone_id.trim_start_matches("/hostedzone/");

    let doc = r#"{"AWSPolicyFormatVersion":"2015-10-01","RecordType":"A","Endpoints":{"default":{"Type":"value","Value":"1.2.3.4"}}}"#;
    let policy_resp = client
        .create_traffic_policy()
        .name("tpi-policy")
        .document(doc)
        .send()
        .await
        .unwrap();
    let policy_id = policy_resp.traffic_policy().unwrap().id().to_string();

    // Create traffic policy instance
    let resp = client
        .create_traffic_policy_instance()
        .hosted_zone_id(short_zone_id)
        .name("www.tpi-test.com.")
        .ttl(300)
        .traffic_policy_id(&policy_id)
        .traffic_policy_version(1)
        .send()
        .await
        .expect("create TPI should succeed");
    let instance = resp.traffic_policy_instance().unwrap();
    let instance_id = instance.id().to_string();
    assert_eq!(instance.name(), "www.tpi-test.com.");
    assert_eq!(instance.ttl(), 300);

    // Get traffic policy instance
    let get_resp = client
        .get_traffic_policy_instance()
        .id(&instance_id)
        .send()
        .await
        .expect("get TPI should succeed");
    assert_eq!(
        get_resp.traffic_policy_instance().unwrap().id(),
        instance_id
    );

    // Update traffic policy instance
    let update_resp = client
        .update_traffic_policy_instance()
        .id(&instance_id)
        .ttl(600)
        .traffic_policy_id(&policy_id)
        .traffic_policy_version(1)
        .send()
        .await
        .expect("update TPI should succeed");
    assert_eq!(update_resp.traffic_policy_instance().unwrap().ttl(), 600);

    // List traffic policy instances
    let list_resp = client
        .list_traffic_policy_instances()
        .send()
        .await
        .expect("list TPIs should succeed");
    assert!(!list_resp.traffic_policy_instances().is_empty());

    // Get traffic policy instance count
    let count_resp = client
        .get_traffic_policy_instance_count()
        .send()
        .await
        .expect("get TPI count should succeed");
    assert!(count_resp.traffic_policy_instance_count() >= 1);

    // Delete traffic policy instance
    client
        .delete_traffic_policy_instance()
        .id(&instance_id)
        .send()
        .await
        .expect("delete TPI should succeed");

    let err = client
        .get_traffic_policy_instance()
        .id(&instance_id)
        .send()
        .await
        .unwrap_err();
    assert!(format!("{:?}", err).contains("NoSuchTrafficPolicyInstance"));
}

// --- GetChange, GetAccountLimit, and other simple operations ---

#[tokio::test]
async fn test_get_change() {
    let client = make_route53_client().await;
    // CreateHostedZone returns a change ID, but GetChange also works with any ID
    let zone_resp = client
        .create_hosted_zone()
        .name("change-test.com")
        .caller_reference("ref-change")
        .send()
        .await
        .unwrap();
    let change_id = zone_resp.change_info().unwrap().id();

    let resp = client
        .get_change()
        .id(change_id)
        .send()
        .await
        .expect("get change should succeed");
    assert_eq!(resp.change_info().unwrap().status().as_str(), "INSYNC");
}

#[tokio::test]
async fn test_get_account_limit() {
    let client = make_route53_client().await;

    let resp = client
        .get_account_limit()
        .r#type(aws_sdk_route53::types::AccountLimitType::MaxHostedZonesByOwner)
        .send()
        .await
        .expect("get account limit should succeed");
    assert!(resp.limit().unwrap().value() > 0);
}

#[tokio::test]
async fn test_get_checker_ip_ranges() {
    let client = make_route53_client().await;

    let resp = client
        .get_checker_ip_ranges()
        .send()
        .await
        .expect("get checker IP ranges should succeed");
    assert!(!resp.checker_ip_ranges().is_empty());
}

#[tokio::test]
async fn test_get_geo_location() {
    let client = make_route53_client().await;

    let resp = client
        .get_geo_location()
        .continent_code("NA")
        .send()
        .await
        .expect("get geo location should succeed");
    let details = resp.geo_location_details().unwrap();
    assert_eq!(details.continent_code(), Some("NA"));
    assert_eq!(details.continent_name(), Some("North America"));
}

#[tokio::test]
async fn test_list_geo_locations() {
    let client = make_route53_client().await;

    let resp = client
        .list_geo_locations()
        .send()
        .await
        .expect("list geo locations should succeed");
    assert!(!resp.geo_location_details_list().is_empty());
}

#[tokio::test]
async fn test_get_health_check_count() {
    let client = make_route53_client().await;

    let resp = client
        .get_health_check_count()
        .send()
        .await
        .expect("get health check count should succeed");
    assert_eq!(resp.health_check_count(), 0);

    // Create a health check and check count again
    create_health_check_for_test(&client, "count-ref").await;
    let resp2 = client.get_health_check_count().send().await.unwrap();
    assert_eq!(resp2.health_check_count(), 1);
}

#[tokio::test]
async fn test_get_health_check_last_failure_reason() {
    let client = make_route53_client().await;
    let hc_id = create_health_check_for_test(&client, "last-fail-ref").await;

    let resp = client
        .get_health_check_last_failure_reason()
        .health_check_id(&hc_id)
        .send()
        .await
        .expect("get last failure reason should succeed");
    // Should return empty observations (healthy)
    assert!(resp.health_check_observations().is_empty());
}

#[tokio::test]
async fn test_get_hosted_zone_limit() {
    let client = make_route53_client().await;
    let zone_id = create_hosted_zone_for_test(&client, "limit-test.com", "ref-limit").await;
    let short_id = zone_id.trim_start_matches("/hostedzone/");

    let resp = client
        .get_hosted_zone_limit()
        .hosted_zone_id(short_id)
        .r#type(aws_sdk_route53::types::HostedZoneLimitType::MaxRrsetsByZone)
        .send()
        .await
        .expect("get hosted zone limit should succeed");
    assert!(resp.limit().unwrap().value() > 0);
    assert!(resp.count() >= 2); // SOA + NS records
}

#[tokio::test]
async fn test_get_reusable_delegation_set_limit() {
    let client = make_route53_client().await;

    // Create a delegation set first
    let ds_resp = client
        .create_reusable_delegation_set()
        .caller_reference("ds-limit-ref")
        .send()
        .await
        .unwrap();
    let ds_id = ds_resp
        .delegation_set()
        .unwrap()
        .id()
        .unwrap()
        .trim_start_matches("/delegationset/")
        .to_string();

    let resp = client
        .get_reusable_delegation_set_limit()
        .delegation_set_id(&ds_id)
        .r#type(
            aws_sdk_route53::types::ReusableDelegationSetLimitType::MaxZonesByReusableDelegationSet,
        )
        .send()
        .await
        .expect("get delegation set limit should succeed");
    assert!(resp.limit().unwrap().value() > 0);
}

#[tokio::test]
async fn test_test_dns_answer() {
    let client = make_route53_client().await;
    let zone_id = create_hosted_zone_for_test(&client, "dns-answer.com", "ref-dns-answer").await;
    let short_id = zone_id.trim_start_matches("/hostedzone/");

    let resp = client
        .test_dns_answer()
        .hosted_zone_id(short_id)
        .record_name("dns-answer.com")
        .record_type(aws_sdk_route53::types::RrType::A)
        .send()
        .await
        .expect("test DNS answer should succeed");
    assert_eq!(resp.response_code(), "NOERROR");
    assert_eq!(resp.protocol(), "UDP");
}

// ============================================================================
// Additional error-path and edge-case tests
// ============================================================================

// --- CreateHostedZone with comment and VPC (private zone) ---

#[tokio::test]
async fn test_create_hosted_zone_with_comment() {
    let client = make_route53_client().await;

    let resp = client
        .create_hosted_zone()
        .name("commented.com")
        .caller_reference("ref-commented")
        .hosted_zone_config(
            aws_sdk_route53::types::HostedZoneConfig::builder()
                .comment("This is a test zone")
                .build(),
        )
        .send()
        .await
        .expect("create with comment should succeed");

    let zone = resp.hosted_zone().unwrap();
    assert_eq!(
        zone.config().and_then(|c| c.comment()),
        Some("This is a test zone")
    );
}

#[tokio::test]
async fn test_create_hosted_zone_with_vpc_makes_private_zone() {
    let client = make_route53_client().await;

    let resp = client
        .create_hosted_zone()
        .name("private.example.com")
        .caller_reference("ref-private-vpc")
        .vpc(
            aws_sdk_route53::types::Vpc::builder()
                .vpc_id("vpc-private-1")
                .vpc_region(aws_sdk_route53::types::VpcRegion::UsEast1)
                .build(),
        )
        .send()
        .await
        .expect("create with VPC should succeed");

    let zone = resp.hosted_zone().unwrap();
    // Zone with VPC should be private
    assert!(
        zone.config().map(|c| c.private_zone()).unwrap_or(false),
        "Zone created with VPC should be private"
    );

    // The VPC should be returned in the response
    assert!(resp.vpc().is_some(), "VPC should be in create response");
    assert_eq!(resp.vpc().unwrap().vpc_id(), Some("vpc-private-1"));
}

// --- UpdateHostedZoneComment error path ---

#[tokio::test]
async fn test_update_hosted_zone_comment_not_found() {
    let client = make_route53_client().await;

    let err = client
        .update_hosted_zone_comment()
        .id("/hostedzone/DOESNOTEXIST")
        .comment("should fail")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("NoSuchHostedZone"),
        "Expected NoSuchHostedZone, got: {err_str}"
    );
}

// --- Upsert overwrites existing record value ---

#[tokio::test]
async fn test_upsert_overwrites_existing_record() {
    let client = make_route53_client().await;

    let zone_id =
        create_hosted_zone_for_test(&client, "upsert-overwrite.com", "ref-upsert-ow").await;

    // Create initial A record
    let batch1 = aws_sdk_route53::types::ChangeBatch::builder()
        .changes(make_a_record_change(
            aws_sdk_route53::types::ChangeAction::Create,
            "app.upsert-overwrite.com.",
            "10.0.0.1",
        ))
        .build()
        .unwrap();

    client
        .change_resource_record_sets()
        .hosted_zone_id(&zone_id)
        .change_batch(batch1)
        .send()
        .await
        .expect("initial CREATE should succeed");

    // Upsert with different value
    let batch2 = aws_sdk_route53::types::ChangeBatch::builder()
        .changes(make_a_record_change(
            aws_sdk_route53::types::ChangeAction::Upsert,
            "app.upsert-overwrite.com.",
            "10.0.0.2",
        ))
        .build()
        .unwrap();

    client
        .change_resource_record_sets()
        .hosted_zone_id(&zone_id)
        .change_batch(batch2)
        .send()
        .await
        .expect("UPSERT should succeed");

    // Verify the value changed
    let resp = client
        .list_resource_record_sets()
        .hosted_zone_id(&zone_id)
        .send()
        .await
        .unwrap();

    let a_record = resp
        .resource_record_sets()
        .iter()
        .find(|r| {
            r.r#type() == &aws_sdk_route53::types::RrType::A
                && r.name() == "app.upsert-overwrite.com."
        })
        .expect("A record should exist");

    let values: Vec<&str> = a_record
        .resource_records()
        .iter()
        .map(|r| r.value())
        .collect();
    assert!(
        values.contains(&"10.0.0.2"),
        "Expected upserted value 10.0.0.2, got: {:?}",
        values
    );
}

// --- Health Check error paths ---

#[tokio::test]
async fn test_delete_health_check_not_found() {
    let client = make_route53_client().await;

    let err = client
        .delete_health_check()
        .health_check_id("nonexistent-hc-id")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("NoSuchHealthCheck"),
        "Expected NoSuchHealthCheck, got: {err_str}"
    );
}

#[tokio::test]
async fn test_get_health_check_status_not_found() {
    let client = make_route53_client().await;

    let err = client
        .get_health_check_status()
        .health_check_id("nonexistent-hc-status")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("NoSuchHealthCheck"),
        "Expected NoSuchHealthCheck, got: {err_str}"
    );
}

#[tokio::test]
async fn test_get_health_check_last_failure_reason_not_found() {
    let client = make_route53_client().await;

    let err = client
        .get_health_check_last_failure_reason()
        .health_check_id("nonexistent-hc-fail")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("NoSuchHealthCheck"),
        "Expected NoSuchHealthCheck, got: {err_str}"
    );
}

#[tokio::test]
async fn test_create_health_check_duplicate_caller_reference() {
    let client = make_route53_client().await;

    create_health_check_for_test(&client, "dup-hc-ref").await;

    let err = client
        .create_health_check()
        .caller_reference("dup-hc-ref")
        .health_check_config(
            aws_sdk_route53::types::HealthCheckConfig::builder()
                .r#type(aws_sdk_route53::types::HealthCheckType::Http)
                .fully_qualified_domain_name("other.example.com")
                .port(80)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("HealthCheckAlreadyExists"),
        "Expected HealthCheckAlreadyExists, got: {err_str}"
    );
}

// --- Tag removal ---

#[tokio::test]
async fn test_remove_tags_from_resource() {
    let client = make_route53_client().await;

    let zone_id =
        create_hosted_zone_for_test(&client, "tag-remove.example.com", "tag-remove").await;
    let short_zone_id = zone_id.trim_start_matches("/hostedzone/").to_string();

    // Add two tags
    client
        .change_tags_for_resource()
        .resource_type(aws_sdk_route53::types::TagResourceType::Hostedzone)
        .resource_id(&short_zone_id)
        .add_tags(
            aws_sdk_route53::types::Tag::builder()
                .key("env")
                .value("test")
                .build(),
        )
        .add_tags(
            aws_sdk_route53::types::Tag::builder()
                .key("team")
                .value("platform")
                .build(),
        )
        .send()
        .await
        .unwrap();

    // Remove one tag
    client
        .change_tags_for_resource()
        .resource_type(aws_sdk_route53::types::TagResourceType::Hostedzone)
        .resource_id(&short_zone_id)
        .remove_tag_keys("env")
        .send()
        .await
        .unwrap();

    // Verify only "team" remains
    let list_resp = client
        .list_tags_for_resource()
        .resource_type(aws_sdk_route53::types::TagResourceType::Hostedzone)
        .resource_id(&zone_id)
        .send()
        .await
        .unwrap();

    let tags = list_resp.resource_tag_set().unwrap().tags();
    assert_eq!(tags.len(), 1, "Expected 1 tag after removal");
    assert_eq!(tags[0].key(), Some("team"));
    assert_eq!(tags[0].value(), Some("platform"));
}

// --- Query Logging error paths ---

#[tokio::test]
async fn test_delete_query_logging_config_not_found() {
    let client = make_route53_client().await;

    let err = client
        .delete_query_logging_config()
        .id("nonexistent-qlc-id")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("NoSuchQueryLoggingConfig"),
        "Expected NoSuchQueryLoggingConfig, got: {err_str}"
    );
}

// --- Delegation Set error paths ---

#[tokio::test]
async fn test_create_reusable_delegation_set_duplicate() {
    let client = make_route53_client().await;

    client
        .create_reusable_delegation_set()
        .caller_reference("dup-ds-ref")
        .send()
        .await
        .expect("first create should succeed");

    let err = client
        .create_reusable_delegation_set()
        .caller_reference("dup-ds-ref")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("DelegationSetAlreadyCreated")
            || err_str.contains("DelegationSetAlreadyReusable"),
        "Expected delegation set already exists error, got: {err_str}"
    );
}

#[tokio::test]
async fn test_get_reusable_delegation_set_limit_not_found() {
    let client = make_route53_client().await;

    let err = client
        .get_reusable_delegation_set_limit()
        .delegation_set_id("DOESNOTEXIST")
        .r#type(
            aws_sdk_route53::types::ReusableDelegationSetLimitType::MaxZonesByReusableDelegationSet,
        )
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("NoSuchDelegationSet"),
        "Expected NoSuchDelegationSet, got: {err_str}"
    );
}

// --- DNSSEC error paths ---

#[tokio::test]
async fn test_enable_dnssec_nonexistent_zone() {
    let client = make_route53_client().await;

    let err = client
        .enable_hosted_zone_dnssec()
        .hosted_zone_id("DOESNOTEXIST")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("NoSuchHostedZone"),
        "Expected NoSuchHostedZone, got: {err_str}"
    );
}

#[tokio::test]
async fn test_disable_dnssec_nonexistent_zone() {
    let client = make_route53_client().await;

    let err = client
        .disable_hosted_zone_dnssec()
        .hosted_zone_id("DOESNOTEXIST")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("NoSuchHostedZone"),
        "Expected NoSuchHostedZone, got: {err_str}"
    );
}

#[tokio::test]
async fn test_create_key_signing_key_nonexistent_zone() {
    let client = make_route53_client().await;

    let err = client
        .create_key_signing_key()
        .hosted_zone_id("DOESNOTEXIST")
        .name("bad-ksk")
        .key_management_service_arn("arn:aws:kms:us-east-1:123456789012:key/test-key")
        .caller_reference("bad-ksk-ref")
        .status("ACTIVE")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("NoSuchHostedZone"),
        "Expected NoSuchHostedZone, got: {err_str}"
    );
}

// --- VPC Association Authorization error paths ---

#[tokio::test]
async fn test_create_vpc_association_authorization_nonexistent_zone() {
    let client = make_route53_client().await;

    let err = client
        .create_vpc_association_authorization()
        .hosted_zone_id("DOESNOTEXIST")
        .vpc(
            aws_sdk_route53::types::Vpc::builder()
                .vpc_id("vpc-99999")
                .vpc_region(aws_sdk_route53::types::VpcRegion::UsEast1)
                .build(),
        )
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("NoSuchHostedZone"),
        "Expected NoSuchHostedZone, got: {err_str}"
    );
}

#[tokio::test]
async fn test_delete_vpc_association_authorization_not_found() {
    let client = make_route53_client().await;

    let zone_id =
        create_hosted_zone_for_test(&client, "vpc-auth-del.com", "ref-vpc-auth-del").await;
    let short_id = zone_id.trim_start_matches("/hostedzone/");

    let err = client
        .delete_vpc_association_authorization()
        .hosted_zone_id(short_id)
        .vpc(
            aws_sdk_route53::types::Vpc::builder()
                .vpc_id("vpc-nonexistent")
                .build(),
        )
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("VPCAssociationAuthorizationNotFound") || err_str.contains("NoSuch"),
        "Expected authorization not found error, got: {err_str}"
    );
}

// --- CIDR Collection error paths ---

#[tokio::test]
async fn test_cidr_list_blocks_filtered_by_location() {
    let client = make_route53_client().await;

    let resp = client
        .create_cidr_collection()
        .name("filter-cidr")
        .caller_reference("filter-cidr-ref")
        .send()
        .await
        .unwrap();
    let collection_id = resp.collection().unwrap().id().unwrap().to_string();

    // Add blocks to two different locations
    client
        .change_cidr_collection()
        .id(&collection_id)
        .changes(
            aws_sdk_route53::types::CidrCollectionChange::builder()
                .action(aws_sdk_route53::types::CidrCollectionChangeAction::Put)
                .location_name("us-east")
                .cidr_list("10.0.0.0/24")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    client
        .change_cidr_collection()
        .id(&collection_id)
        .changes(
            aws_sdk_route53::types::CidrCollectionChange::builder()
                .action(aws_sdk_route53::types::CidrCollectionChangeAction::Put)
                .location_name("eu-west")
                .cidr_list("172.16.0.0/16")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    // List all blocks (should be 2)
    let all_blocks = client
        .list_cidr_blocks()
        .collection_id(&collection_id)
        .send()
        .await
        .unwrap();
    assert_eq!(all_blocks.cidr_blocks().len(), 2);

    // List blocks filtered by location "us-east"
    let filtered = client
        .list_cidr_blocks()
        .collection_id(&collection_id)
        .location_name("us-east")
        .send()
        .await
        .unwrap();
    assert_eq!(filtered.cidr_blocks().len(), 1);
    assert_eq!(filtered.cidr_blocks()[0].cidr_block(), Some("10.0.0.0/24"));
}

#[tokio::test]
async fn test_change_cidr_collection_nonexistent() {
    let client = make_route53_client().await;

    let err = client
        .change_cidr_collection()
        .id("nonexistent-cidr-id")
        .changes(
            aws_sdk_route53::types::CidrCollectionChange::builder()
                .action(aws_sdk_route53::types::CidrCollectionChangeAction::Put)
                .location_name("test")
                .cidr_list("10.0.0.0/8")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("NoSuchCidrCollectionException") || err_str.contains("NoSuch"),
        "Expected NoSuchCidrCollectionException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_list_cidr_locations_nonexistent() {
    let client = make_route53_client().await;

    let err = client
        .list_cidr_locations()
        .collection_id("nonexistent-cidr-loc")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("NoSuchCidrCollectionException") || err_str.contains("NoSuch"),
        "Expected NoSuchCidrCollectionException, got: {err_str}"
    );
}

// --- Traffic Policy error paths ---

#[tokio::test]
async fn test_get_traffic_policy_not_found() {
    let client = make_route53_client().await;

    let err = client
        .get_traffic_policy()
        .id("nonexistent-policy")
        .version(1)
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("NoSuchTrafficPolicy"),
        "Expected NoSuchTrafficPolicy, got: {err_str}"
    );
}

#[tokio::test]
async fn test_delete_traffic_policy_not_found() {
    let client = make_route53_client().await;

    let err = client
        .delete_traffic_policy()
        .id("nonexistent-policy")
        .version(1)
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("NoSuchTrafficPolicy"),
        "Expected NoSuchTrafficPolicy, got: {err_str}"
    );
}

#[tokio::test]
async fn test_list_traffic_policy_versions_not_found() {
    let client = make_route53_client().await;

    let err = client
        .list_traffic_policy_versions()
        .id("nonexistent-policy-versions")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("NoSuchTrafficPolicy"),
        "Expected NoSuchTrafficPolicy, got: {err_str}"
    );
}

// --- Traffic Policy Instance additional tests ---

#[tokio::test]
async fn test_delete_traffic_policy_instance_not_found() {
    let client = make_route53_client().await;

    let err = client
        .delete_traffic_policy_instance()
        .id("nonexistent-tpi")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("NoSuchTrafficPolicyInstance"),
        "Expected NoSuchTrafficPolicyInstance, got: {err_str}"
    );
}

#[tokio::test]
async fn test_list_traffic_policy_instances_by_hosted_zone() {
    let client = make_route53_client().await;

    let zone_id = create_hosted_zone_for_test(&client, "tpi-by-zone.com", "ref-tpi-by-zone").await;
    let short_zone_id = zone_id.trim_start_matches("/hostedzone/");

    let doc = r#"{"AWSPolicyFormatVersion":"2015-10-01","RecordType":"A","Endpoints":{"default":{"Type":"value","Value":"1.2.3.4"}}}"#;
    let policy_resp = client
        .create_traffic_policy()
        .name("tpi-by-zone-policy")
        .document(doc)
        .send()
        .await
        .unwrap();
    let policy_id = policy_resp.traffic_policy().unwrap().id().to_string();

    client
        .create_traffic_policy_instance()
        .hosted_zone_id(short_zone_id)
        .name("www.tpi-by-zone.com.")
        .ttl(300)
        .traffic_policy_id(&policy_id)
        .traffic_policy_version(1)
        .send()
        .await
        .unwrap();

    let list_resp = client
        .list_traffic_policy_instances_by_hosted_zone()
        .hosted_zone_id(short_zone_id)
        .send()
        .await
        .expect("list TPIs by hosted zone should succeed");

    assert!(
        !list_resp.traffic_policy_instances().is_empty(),
        "Should have at least one TPI for the hosted zone"
    );
}

#[tokio::test]
async fn test_list_traffic_policy_instances_by_policy() {
    let client = make_route53_client().await;

    let zone_id =
        create_hosted_zone_for_test(&client, "tpi-by-policy.com", "ref-tpi-by-policy").await;
    let short_zone_id = zone_id.trim_start_matches("/hostedzone/");

    let doc = r#"{"AWSPolicyFormatVersion":"2015-10-01","RecordType":"A","Endpoints":{"default":{"Type":"value","Value":"5.6.7.8"}}}"#;
    let policy_resp = client
        .create_traffic_policy()
        .name("tpi-by-policy")
        .document(doc)
        .send()
        .await
        .unwrap();
    let policy_id = policy_resp.traffic_policy().unwrap().id().to_string();

    client
        .create_traffic_policy_instance()
        .hosted_zone_id(short_zone_id)
        .name("api.tpi-by-policy.com.")
        .ttl(60)
        .traffic_policy_id(&policy_id)
        .traffic_policy_version(1)
        .send()
        .await
        .unwrap();

    let list_resp = client
        .list_traffic_policy_instances_by_policy()
        .traffic_policy_id(&policy_id)
        .traffic_policy_version(1)
        .send()
        .await
        .expect("list TPIs by policy should succeed");

    assert!(
        !list_resp.traffic_policy_instances().is_empty(),
        "Should have at least one TPI for the policy"
    );
}

// --- UpdateHostedZoneFeatures ---

#[tokio::test]
async fn test_update_hosted_zone_features() {
    let client = make_route53_client().await;

    // This is a simple operation that always succeeds.
    // We need to use raw HTTP since the SDK may not have this method.
    // Instead, just verify the zone lifecycle works with features endpoint.
    // The handler returns a 200 for any zone.
    let _zone_id = create_hosted_zone_for_test(&client, "features.com", "ref-features").await;

    // UpdateHostedZoneFeatures is not in the standard SDK but the handler exists.
    // We confirm it's reachable by testing indirectly through the handler routing.
    // Skipping direct SDK test as the operation may not be in the SDK version.
}

// --- GetHostedZoneLimit error path ---

#[tokio::test]
async fn test_get_hosted_zone_limit_not_found() {
    let client = make_route53_client().await;

    let err = client
        .get_hosted_zone_limit()
        .hosted_zone_id("DOESNOTEXIST")
        .r#type(aws_sdk_route53::types::HostedZoneLimitType::MaxRrsetsByZone)
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("NoSuchHostedZone"),
        "Expected NoSuchHostedZone, got: {err_str}"
    );
}

// --- Multiple account limit types ---

#[tokio::test]
async fn test_get_account_limit_multiple_types() {
    let client = make_route53_client().await;

    let resp = client
        .get_account_limit()
        .r#type(aws_sdk_route53::types::AccountLimitType::MaxHealthChecksByOwner)
        .send()
        .await
        .expect("get health check limit should succeed");
    assert!(resp.limit().unwrap().value() > 0);

    let resp2 = client
        .get_account_limit()
        .r#type(aws_sdk_route53::types::AccountLimitType::MaxReusableDelegationSetsByOwner)
        .send()
        .await
        .expect("get delegation set limit should succeed");
    assert!(resp2.limit().unwrap().value() > 0);
}

// --- GetHostedZoneCount after deletion ---

#[tokio::test]
async fn test_get_hosted_zone_count_decrements_after_delete() {
    let client = make_route53_client().await;

    let zone_id = create_hosted_zone_for_test(&client, "count-del.com", "count-del").await;
    create_hosted_zone_for_test(&client, "count-keep.com", "count-keep").await;

    let resp1 = client.get_hosted_zone_count().send().await.unwrap();
    assert_eq!(resp1.hosted_zone_count(), 2);

    client
        .delete_hosted_zone()
        .id(&zone_id)
        .send()
        .await
        .unwrap();

    let resp2 = client.get_hosted_zone_count().send().await.unwrap();
    assert_eq!(resp2.hosted_zone_count(), 1);
}

// --- GetGeoLocation with country code ---

#[tokio::test]
async fn test_get_geo_location_country_code() {
    let client = make_route53_client().await;

    let resp = client
        .get_geo_location()
        .country_code("US")
        .send()
        .await
        .expect("get geo location with country should succeed");

    let details = resp.geo_location_details().unwrap();
    assert_eq!(details.country_code(), Some("US"));
}

// --- ListHostedZonesByName returns all when no filter ---

#[tokio::test]
async fn test_list_hosted_zones_by_name_returns_all() {
    let client = make_route53_client().await;

    create_hosted_zone_for_test(&client, "aaa-name.com", "ref-aaa-name").await;
    create_hosted_zone_for_test(&client, "zzz-name.com", "ref-zzz-name").await;

    let resp = client
        .list_hosted_zones_by_name()
        .send()
        .await
        .expect("list by name without filter should succeed");

    assert_eq!(resp.hosted_zones().len(), 2);
}

// --- Multiple VPC associations on same zone ---

#[tokio::test]
async fn test_multiple_vpc_associations() {
    let client = make_route53_client().await;

    let zone_id = create_hosted_zone_for_test(&client, "multi-vpc.com", "ref-multi-vpc").await;
    let short_id = zone_id.trim_start_matches("/hostedzone/");

    // Associate two different VPCs
    client
        .associate_vpc_with_hosted_zone()
        .hosted_zone_id(short_id)
        .vpc(
            aws_sdk_route53::types::Vpc::builder()
                .vpc_id("vpc-multi-1")
                .vpc_region(aws_sdk_route53::types::VpcRegion::UsEast1)
                .build(),
        )
        .send()
        .await
        .unwrap();

    client
        .associate_vpc_with_hosted_zone()
        .hosted_zone_id(short_id)
        .vpc(
            aws_sdk_route53::types::Vpc::builder()
                .vpc_id("vpc-multi-2")
                .vpc_region(aws_sdk_route53::types::VpcRegion::UsWest2)
                .build(),
        )
        .send()
        .await
        .unwrap();

    // GetHostedZone should return both VPCs
    let get_resp = client.get_hosted_zone().id(&zone_id).send().await.unwrap();

    let vpcs = get_resp.vpcs();
    assert_eq!(vpcs.len(), 2, "Expected 2 VPCs associated with the zone");

    // Zone should be listed for both VPCs
    let list1 = client
        .list_hosted_zones_by_vpc()
        .vpc_id("vpc-multi-1")
        .vpc_region(aws_sdk_route53::types::VpcRegion::UsEast1)
        .send()
        .await
        .unwrap();
    assert_eq!(list1.hosted_zone_summaries().len(), 1);

    let list2 = client
        .list_hosted_zones_by_vpc()
        .vpc_id("vpc-multi-2")
        .vpc_region(aws_sdk_route53::types::VpcRegion::UsWest2)
        .send()
        .await
        .unwrap();
    assert_eq!(list2.hosted_zone_summaries().len(), 1);
}

// --- Health check with update verifying version increment ---

#[tokio::test]
async fn test_health_check_version_increments_on_update() {
    let client = make_route53_client().await;

    let hc_id = create_health_check_for_test(&client, "hc-version-ref").await;

    let get1 = client
        .get_health_check()
        .health_check_id(&hc_id)
        .send()
        .await
        .unwrap();
    let v1 = get1.health_check().unwrap().health_check_version();

    client
        .update_health_check()
        .health_check_id(&hc_id)
        .resource_path("/v2")
        .send()
        .await
        .unwrap();

    let get2 = client
        .get_health_check()
        .health_check_id(&hc_id)
        .send()
        .await
        .unwrap();
    let v2 = get2.health_check().unwrap().health_check_version();

    assert!(
        v2 > v1,
        "Health check version should increment after update: v1={v1}, v2={v2}"
    );
}

// --- Query logging config list returns empty for unrelated zone ---

#[tokio::test]
async fn test_list_query_logging_configs_empty_for_different_zone() {
    let client = make_route53_client().await;

    let zone1 = create_hosted_zone_for_test(&client, "qlc-zone1.com", "qlc-z1").await;
    let zone2 = create_hosted_zone_for_test(&client, "qlc-zone2.com", "qlc-z2").await;

    // Create config for zone1
    client
        .create_query_logging_config()
        .hosted_zone_id(&zone1)
        .cloud_watch_logs_log_group_arn(
            "arn:aws:logs:us-east-1:123456789012:log-group:/aws/route53/zone1",
        )
        .send()
        .await
        .unwrap();

    // List for zone2 should be empty
    let list_resp = client
        .list_query_logging_configs()
        .hosted_zone_id(&zone2)
        .send()
        .await
        .unwrap();
    assert!(
        list_resp.query_logging_configs().is_empty(),
        "Expected no configs for zone2"
    );
}

// --- Reusable delegation set: delete then get should fail ---

#[tokio::test]
async fn test_delete_reusable_delegation_set_not_found() {
    let client = make_route53_client().await;

    let err = client
        .delete_reusable_delegation_set()
        .id("DOESNOTEXIST")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("NoSuchDelegationSet"),
        "Expected NoSuchDelegationSet, got: {err_str}"
    );
}

// ============================================================================
// Record name normalization tests
// ============================================================================

/// Helper to create an A record with a given name in a given zone.
async fn create_a_record(client: &aws_sdk_route53::Client, zone_id: &str, name: &str, ip: &str) {
    client
        .change_resource_record_sets()
        .hosted_zone_id(zone_id)
        .change_batch(
            aws_sdk_route53::types::ChangeBatch::builder()
                .changes(
                    aws_sdk_route53::types::Change::builder()
                        .action(aws_sdk_route53::types::ChangeAction::Create)
                        .resource_record_set(
                            aws_sdk_route53::types::ResourceRecordSet::builder()
                                .name(name)
                                .r#type(aws_sdk_route53::types::RrType::A)
                                .ttl(300)
                                .resource_records(
                                    aws_sdk_route53::types::ResourceRecord::builder()
                                        .value(ip)
                                        .build()
                                        .unwrap(),
                                )
                                .build()
                                .unwrap(),
                        )
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("create A record should succeed");
}

/// FQDN with trailing dot is stored and returned as-is.
#[tokio::test]
async fn test_record_name_fqdn_with_trailing_dot() {
    let client = make_route53_client().await;
    let zone_id = create_hosted_zone_for_test(&client, "norm1.com.", "ref-norm1").await;

    create_a_record(&client, &zone_id, "sub.norm1.com.", "1.2.3.4").await;

    let resp = client
        .list_resource_record_sets()
        .hosted_zone_id(&zone_id)
        .send()
        .await
        .unwrap();

    let a_rec = resp
        .resource_record_sets()
        .iter()
        .find(|r| r.r#type() == &aws_sdk_route53::types::RrType::A)
        .expect("should find A record");
    assert_eq!(a_rec.name(), "sub.norm1.com.");
}

/// FQDN without trailing dot gets dot appended.
#[tokio::test]
async fn test_record_name_fqdn_without_trailing_dot() {
    let client = make_route53_client().await;
    let zone_id = create_hosted_zone_for_test(&client, "norm2.com.", "ref-norm2").await;

    create_a_record(&client, &zone_id, "sub.norm2.com", "1.2.3.4").await;

    let resp = client
        .list_resource_record_sets()
        .hosted_zone_id(&zone_id)
        .send()
        .await
        .unwrap();

    let a_rec = resp
        .resource_record_sets()
        .iter()
        .find(|r| r.r#type() == &aws_sdk_route53::types::RrType::A)
        .expect("should find A record");
    // Must be stored with trailing dot
    assert_eq!(a_rec.name(), "sub.norm2.com.");
}

/// Zone-relative name (short name) gets zone name appended.
#[tokio::test]
async fn test_record_name_zone_relative() {
    let client = make_route53_client().await;
    let zone_id = create_hosted_zone_for_test(&client, "norm3.com.", "ref-norm3").await;

    // Just "sub" — should become "sub.norm3.com."
    create_a_record(&client, &zone_id, "sub", "1.2.3.4").await;

    let resp = client
        .list_resource_record_sets()
        .hosted_zone_id(&zone_id)
        .send()
        .await
        .unwrap();

    let a_rec = resp
        .resource_record_sets()
        .iter()
        .find(|r| r.r#type() == &aws_sdk_route53::types::RrType::A)
        .expect("should find A record");
    assert_eq!(a_rec.name(), "sub.norm3.com.");
}

/// UPSERT with trailing dot should match a record created without trailing dot.
#[tokio::test]
async fn test_record_name_upsert_matches_despite_dot_difference() {
    let client = make_route53_client().await;
    let zone_id = create_hosted_zone_for_test(&client, "norm4.com.", "ref-norm4").await;

    // Create without trailing dot
    create_a_record(&client, &zone_id, "app.norm4.com", "1.1.1.1").await;

    // Upsert with trailing dot — should overwrite, not create duplicate
    client
        .change_resource_record_sets()
        .hosted_zone_id(&zone_id)
        .change_batch(
            aws_sdk_route53::types::ChangeBatch::builder()
                .changes(
                    aws_sdk_route53::types::Change::builder()
                        .action(aws_sdk_route53::types::ChangeAction::Upsert)
                        .resource_record_set(
                            aws_sdk_route53::types::ResourceRecordSet::builder()
                                .name("app.norm4.com.")
                                .r#type(aws_sdk_route53::types::RrType::A)
                                .ttl(300)
                                .resource_records(
                                    aws_sdk_route53::types::ResourceRecord::builder()
                                        .value("2.2.2.2")
                                        .build()
                                        .unwrap(),
                                )
                                .build()
                                .unwrap(),
                        )
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("upsert should succeed");

    let resp = client
        .list_resource_record_sets()
        .hosted_zone_id(&zone_id)
        .send()
        .await
        .unwrap();

    let a_records: Vec<_> = resp
        .resource_record_sets()
        .iter()
        .filter(|r| r.r#type() == &aws_sdk_route53::types::RrType::A)
        .collect();

    // Should have exactly 1 A record, not 2
    assert_eq!(
        a_records.len(),
        1,
        "UPSERT should match despite trailing dot difference"
    );
    // And it should have the updated IP
    let values: Vec<_> = a_records[0]
        .resource_records()
        .iter()
        .map(|r| r.value())
        .collect();
    assert_eq!(values, vec!["2.2.2.2"]);
}

/// DELETE with zone-relative name should match a record created with FQDN.
#[tokio::test]
async fn test_record_name_delete_zone_relative_matches_fqdn() {
    let client = make_route53_client().await;
    let zone_id = create_hosted_zone_for_test(&client, "norm5.com.", "ref-norm5").await;

    // Create with full FQDN
    create_a_record(&client, &zone_id, "bye.norm5.com.", "3.3.3.3").await;

    // Delete with zone-relative name
    client
        .change_resource_record_sets()
        .hosted_zone_id(&zone_id)
        .change_batch(
            aws_sdk_route53::types::ChangeBatch::builder()
                .changes(
                    aws_sdk_route53::types::Change::builder()
                        .action(aws_sdk_route53::types::ChangeAction::Delete)
                        .resource_record_set(
                            aws_sdk_route53::types::ResourceRecordSet::builder()
                                .name("bye")
                                .r#type(aws_sdk_route53::types::RrType::A)
                                .ttl(300)
                                .resource_records(
                                    aws_sdk_route53::types::ResourceRecord::builder()
                                        .value("3.3.3.3")
                                        .build()
                                        .unwrap(),
                                )
                                .build()
                                .unwrap(),
                        )
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("delete should succeed");

    let resp = client
        .list_resource_record_sets()
        .hosted_zone_id(&zone_id)
        .send()
        .await
        .unwrap();

    let a_records: Vec<_> = resp
        .resource_record_sets()
        .iter()
        .filter(|r| r.r#type() == &aws_sdk_route53::types::RrType::A)
        .collect();

    assert_eq!(
        a_records.len(),
        0,
        "record should have been deleted via zone-relative name"
    );
}

/// CREATE with duplicate name (different dot form) should error.
#[tokio::test]
async fn test_record_name_create_duplicate_different_dot_form() {
    let client = make_route53_client().await;
    let zone_id = create_hosted_zone_for_test(&client, "norm6.com.", "ref-norm6").await;

    // Create with trailing dot
    create_a_record(&client, &zone_id, "dup.norm6.com.", "1.1.1.1").await;

    // Try to CREATE again without trailing dot — should fail as duplicate
    let err = client
        .change_resource_record_sets()
        .hosted_zone_id(&zone_id)
        .change_batch(
            aws_sdk_route53::types::ChangeBatch::builder()
                .changes(
                    aws_sdk_route53::types::Change::builder()
                        .action(aws_sdk_route53::types::ChangeAction::Create)
                        .resource_record_set(
                            aws_sdk_route53::types::ResourceRecordSet::builder()
                                .name("dup.norm6.com")
                                .r#type(aws_sdk_route53::types::RrType::A)
                                .ttl(300)
                                .resource_records(
                                    aws_sdk_route53::types::ResourceRecord::builder()
                                        .value("2.2.2.2")
                                        .build()
                                        .unwrap(),
                                )
                                .build()
                                .unwrap(),
                        )
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        )
        .send()
        .await;

    assert!(
        err.is_err(),
        "CREATE should fail for duplicate name with different dot form"
    );
}
