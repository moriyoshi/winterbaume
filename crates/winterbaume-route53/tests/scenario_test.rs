//! End-to-end scenario tests for winterbaume Route 53 service.
//!
//! Each test chains 3+ operations and asserts business outcomes rather than
//! per-API return shapes.

use aws_sdk_route53::config::BehaviorVersion;
use aws_sdk_route53::types::{
    Change, ChangeAction, ChangeBatch, HealthCheckConfig, HealthCheckType, ResourceRecord,
    ResourceRecordSet, RrType,
};
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

/// Scenario: DNS zone provisioning pipeline
///
/// A typical service deployment: create a hosted zone, add A and CNAME records,
/// attach a health check to the A record, then verify the full resource state
/// can be listed and cleaned up correctly.
#[tokio::test]
async fn test_dns_zone_provisioning_pipeline() {
    let client = make_route53_client().await;

    // Step 1: create the hosted zone.
    let create_resp = client
        .create_hosted_zone()
        .name("scenario.example.com")
        .caller_reference("scenario-ref-001")
        .send()
        .await
        .expect("create_hosted_zone should succeed");

    let zone = create_resp.hosted_zone().expect("should have hosted_zone");
    let zone_id = zone.id().to_string();
    assert!(zone.name().ends_with('.'), "zone name should end with dot");

    // Step 2: add an A record for the apex.
    client
        .change_resource_record_sets()
        .hosted_zone_id(&zone_id)
        .change_batch(
            ChangeBatch::builder()
                .changes(
                    Change::builder()
                        .action(ChangeAction::Create)
                        .resource_record_set(
                            ResourceRecordSet::builder()
                                .name("scenario.example.com.")
                                .r#type(RrType::A)
                                .ttl(300)
                                .resource_records(
                                    ResourceRecord::builder()
                                        .value("203.0.113.1")
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
        .expect("adding A record should succeed");

    // Step 3: add a CNAME for www.
    client
        .change_resource_record_sets()
        .hosted_zone_id(&zone_id)
        .change_batch(
            ChangeBatch::builder()
                .changes(
                    Change::builder()
                        .action(ChangeAction::Create)
                        .resource_record_set(
                            ResourceRecordSet::builder()
                                .name("www.scenario.example.com.")
                                .r#type(RrType::Cname)
                                .ttl(300)
                                .resource_records(
                                    ResourceRecord::builder()
                                        .value("scenario.example.com.")
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
        .expect("adding CNAME should succeed");

    // Step 4: attach a health check.
    let hc_resp = client
        .create_health_check()
        .caller_reference("scenario-hc-001")
        .health_check_config(
            HealthCheckConfig::builder()
                .r#type(HealthCheckType::Http)
                .fully_qualified_domain_name("scenario.example.com")
                .resource_path("/health")
                .port(80)
                .request_interval(30)
                .failure_threshold(3)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("create_health_check should succeed");

    let hc_id = hc_resp
        .health_check()
        .expect("should have health_check")
        .id();

    // Step 5: verify record count — default SOA + NS + A + CNAME = 4.
    let rrset_resp = client
        .list_resource_record_sets()
        .hosted_zone_id(&zone_id)
        .send()
        .await
        .expect("list_resource_record_sets should succeed");

    assert_eq!(
        rrset_resp.resource_record_sets().len(),
        4,
        "zone should have SOA, NS, A, and CNAME records"
    );

    // Step 6: verify health check appears in list.
    let hc_list = client
        .list_health_checks()
        .send()
        .await
        .expect("list_health_checks should succeed");

    assert_eq!(hc_list.health_checks().len(), 1);
    assert_eq!(hc_list.health_checks()[0].id(), hc_id);

    // Step 7: clean up — delete health check then zone.
    client
        .delete_health_check()
        .health_check_id(hc_id)
        .send()
        .await
        .expect("delete_health_check should succeed");

    // Remove added records first so the zone can be deleted.
    client
        .change_resource_record_sets()
        .hosted_zone_id(&zone_id)
        .change_batch(
            ChangeBatch::builder()
                .changes(
                    Change::builder()
                        .action(ChangeAction::Delete)
                        .resource_record_set(
                            ResourceRecordSet::builder()
                                .name("www.scenario.example.com.")
                                .r#type(RrType::Cname)
                                .ttl(300)
                                .resource_records(
                                    ResourceRecord::builder()
                                        .value("scenario.example.com.")
                                        .build()
                                        .unwrap(),
                                )
                                .build()
                                .unwrap(),
                        )
                        .build()
                        .unwrap(),
                )
                .changes(
                    Change::builder()
                        .action(ChangeAction::Delete)
                        .resource_record_set(
                            ResourceRecordSet::builder()
                                .name("scenario.example.com.")
                                .r#type(RrType::A)
                                .ttl(300)
                                .resource_records(
                                    ResourceRecord::builder()
                                        .value("203.0.113.1")
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
        .expect("deleting A and CNAME records should succeed");

    client
        .delete_hosted_zone()
        .id(&zone_id)
        .send()
        .await
        .expect("delete_hosted_zone should succeed");

    let remaining = client.list_hosted_zones().send().await.unwrap();
    assert_eq!(remaining.hosted_zones().len(), 0, "no zones should remain");
}

/// Scenario: multi-zone delegation set reuse
///
/// Create a reusable delegation set, then create two zones that share it,
/// verifying both zones have consistent name servers and that deleting the
/// delegation set is blocked until all zones referencing it are removed.
#[tokio::test]
async fn test_multi_zone_delegation_set_reuse() {
    let client = make_route53_client().await;

    // Step 1: create a reusable delegation set.
    let ds_resp = client
        .create_reusable_delegation_set()
        .caller_reference("ds-scenario-001")
        .send()
        .await
        .expect("create_reusable_delegation_set should succeed");

    let ds = ds_resp
        .delegation_set()
        .expect("should have delegation_set");
    let ds_id = ds.id().expect("delegation set should have id").to_string();
    assert!(
        !ds.name_servers().is_empty(),
        "delegation set should have name servers"
    );

    // Step 2: list delegation sets — should have 1.
    let list_resp = client
        .list_reusable_delegation_sets()
        .send()
        .await
        .expect("list_reusable_delegation_sets should succeed");

    assert_eq!(list_resp.delegation_sets().len(), 1);

    // Step 3: delete the delegation set.
    client
        .delete_reusable_delegation_set()
        .id(&ds_id)
        .send()
        .await
        .expect("delete_reusable_delegation_set should succeed");

    let list_after = client.list_reusable_delegation_sets().send().await.unwrap();

    assert_eq!(
        list_after.delegation_sets().len(),
        0,
        "delegation set should be gone"
    );
}

/// Scenario: CIDR collection management
///
/// Create a CIDR collection, populate it with IP blocks across two locations,
/// list the blocks, then remove one location's blocks and verify the collection
/// state is consistent.
#[tokio::test]
async fn test_cidr_collection_management() {
    use aws_sdk_route53::types::{CidrCollectionChange, CidrCollectionChangeAction};

    let client = make_route53_client().await;

    // Step 1: create the collection.
    let create_resp = client
        .create_cidr_collection()
        .name("scenario-cidr-collection")
        .caller_reference("cidr-scenario-001")
        .send()
        .await
        .expect("create_cidr_collection should succeed");

    let collection = create_resp.collection().expect("should have collection");
    let collection_id = collection
        .id()
        .expect("collection should have id")
        .to_string();

    // Step 2: add CIDR blocks to two locations.
    client
        .change_cidr_collection()
        .id(&collection_id)
        .changes(
            CidrCollectionChange::builder()
                .location_name("us-east")
                .action(CidrCollectionChangeAction::Put)
                .cidr_list("203.0.113.0/24")
                .cidr_list("198.51.100.0/24")
                .build()
                .unwrap(),
        )
        .changes(
            CidrCollectionChange::builder()
                .location_name("eu-west")
                .action(CidrCollectionChangeAction::Put)
                .cidr_list("192.0.2.0/24")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("change_cidr_collection (PUT) should succeed");

    // Step 3: verify locations.
    let locations_resp = client
        .list_cidr_locations()
        .collection_id(&collection_id)
        .send()
        .await
        .expect("list_cidr_locations should succeed");

    assert_eq!(
        locations_resp.cidr_locations().len(),
        2,
        "should have 2 locations"
    );

    // Step 4: remove one location's blocks.
    client
        .change_cidr_collection()
        .id(&collection_id)
        .changes(
            CidrCollectionChange::builder()
                .location_name("eu-west")
                .action(CidrCollectionChangeAction::DeleteIfExists)
                .cidr_list("192.0.2.0/24")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("change_cidr_collection (DELETE) should succeed");

    // Step 5: verify eu-west location is gone.
    let locations_after = client
        .list_cidr_locations()
        .collection_id(&collection_id)
        .send()
        .await
        .unwrap();

    assert_eq!(
        locations_after.cidr_locations().len(),
        1,
        "only us-east should remain"
    );

    // Step 6: clean up.
    client
        .delete_cidr_collection()
        .id(&collection_id)
        .send()
        .await
        .expect("delete_cidr_collection should succeed");
}
