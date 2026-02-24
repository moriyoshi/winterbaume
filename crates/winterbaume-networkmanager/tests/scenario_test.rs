//! End-to-end scenario tests for winterbaume-networkmanager.
//!
//! Each test chains 3+ operations and asserts business outcomes rather than
//! individual API response shapes.

use aws_sdk_networkmanager::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_networkmanager::NetworkManagerService;

async fn make_client() -> aws_sdk_networkmanager::Client {
    let mock = MockAws::builder()
        .with_service(NetworkManagerService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_networkmanager::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_networkmanager::Client::new(&config)
}

/// Scenario: Full SD-WAN topology — global network + sites + devices + links + associations.
///
/// Creates a global network, two sites, a device at each site, links for bandwidth,
/// then associates the devices with their links, and verifies the topology can be queried.
#[tokio::test]
async fn test_sd_wan_topology_lifecycle() {
    // Scenario: SD-WAN topology
    // 1. Create a global network
    // 2. Add two sites (headquarters and branch)
    // 3. Create a device per site
    // 4. Create a link per site with bandwidth info
    // 5. Associate devices with their links
    // 6. Verify associations are visible
    // 7. Tear down in reverse order

    let client = make_client().await;

    // Step 1: Create global network
    let gn = client
        .create_global_network()
        .description("SD-WAN topology test")
        .send()
        .await
        .expect("create_global_network");
    let gn_id = gn
        .global_network()
        .unwrap()
        .global_network_id()
        .unwrap()
        .to_string();

    // Step 2: Create two sites
    let hq_site = client
        .create_site()
        .global_network_id(&gn_id)
        .description("Headquarters")
        .send()
        .await
        .expect("create_site HQ");
    let hq_site_id = hq_site.site().unwrap().site_id().unwrap().to_string();

    let branch_site = client
        .create_site()
        .global_network_id(&gn_id)
        .description("Branch Office")
        .send()
        .await
        .expect("create_site branch");
    let branch_site_id = branch_site.site().unwrap().site_id().unwrap().to_string();

    // Step 3: Create devices at each site
    let hq_device = client
        .create_device()
        .global_network_id(&gn_id)
        .site_id(&hq_site_id)
        .description("HQ Router")
        .send()
        .await
        .expect("create_device HQ");
    let hq_device_id = hq_device.device().unwrap().device_id().unwrap().to_string();

    let branch_device = client
        .create_device()
        .global_network_id(&gn_id)
        .site_id(&branch_site_id)
        .description("Branch Router")
        .send()
        .await
        .expect("create_device branch");
    let branch_device_id = branch_device
        .device()
        .unwrap()
        .device_id()
        .unwrap()
        .to_string();

    // Step 4: Create links with bandwidth
    let hq_link = client
        .create_link()
        .global_network_id(&gn_id)
        .site_id(&hq_site_id)
        .description("HQ WAN Link")
        .r#type("broadband")
        .provider("ISP-A")
        .bandwidth(
            aws_sdk_networkmanager::types::Bandwidth::builder()
                .download_speed(1000)
                .upload_speed(500)
                .build(),
        )
        .send()
        .await
        .expect("create_link HQ");
    let hq_link_id = hq_link.link().unwrap().link_id().unwrap().to_string();

    let branch_link = client
        .create_link()
        .global_network_id(&gn_id)
        .site_id(&branch_site_id)
        .description("Branch WAN Link")
        .r#type("broadband")
        .provider("ISP-B")
        .bandwidth(
            aws_sdk_networkmanager::types::Bandwidth::builder()
                .download_speed(100)
                .upload_speed(50)
                .build(),
        )
        .send()
        .await
        .expect("create_link branch");
    let branch_link_id = branch_link.link().unwrap().link_id().unwrap().to_string();

    // Step 5: Associate devices with their links
    let hq_assoc = client
        .associate_link()
        .global_network_id(&gn_id)
        .device_id(&hq_device_id)
        .link_id(&hq_link_id)
        .send()
        .await
        .expect("associate_link HQ");
    assert_eq!(
        hq_assoc
            .link_association()
            .unwrap()
            .link_association_state()
            .map(|s| s.as_str()),
        Some("AVAILABLE"),
        "HQ link association should be AVAILABLE"
    );

    let branch_assoc = client
        .associate_link()
        .global_network_id(&gn_id)
        .device_id(&branch_device_id)
        .link_id(&branch_link_id)
        .send()
        .await
        .expect("associate_link branch");
    assert_eq!(
        branch_assoc
            .link_association()
            .unwrap()
            .link_association_state()
            .map(|s| s.as_str()),
        Some("AVAILABLE"),
        "Branch link association should be AVAILABLE"
    );

    // Step 6: Verify topology is queryable
    let links_resp = client
        .get_links()
        .global_network_id(&gn_id)
        .send()
        .await
        .expect("get_links");
    assert_eq!(
        links_resp.links().len(),
        2,
        "Should have 2 links in the global network"
    );

    let assocs_resp = client
        .get_link_associations()
        .global_network_id(&gn_id)
        .send()
        .await
        .expect("get_link_associations");
    assert_eq!(
        assocs_resp.link_associations().len(),
        2,
        "Should have 2 link associations"
    );

    let devices_resp = client
        .get_devices()
        .global_network_id(&gn_id)
        .send()
        .await
        .expect("get_devices");
    assert_eq!(
        devices_resp.devices().len(),
        2,
        "Should have 2 devices in the global network"
    );

    // Step 7: Tear down — disassociate links, delete resources
    client
        .disassociate_link()
        .global_network_id(&gn_id)
        .device_id(&hq_device_id)
        .link_id(&hq_link_id)
        .send()
        .await
        .expect("disassociate_link HQ");

    client
        .disassociate_link()
        .global_network_id(&gn_id)
        .device_id(&branch_device_id)
        .link_id(&branch_link_id)
        .send()
        .await
        .expect("disassociate_link branch");

    let empty_assocs = client
        .get_link_associations()
        .global_network_id(&gn_id)
        .send()
        .await
        .expect("get_link_associations after disassociate");
    assert_eq!(
        empty_assocs.link_associations().len(),
        0,
        "No link associations should remain after disassociation"
    );
}

/// Scenario: Core network + VPC attachment lifecycle.
///
/// Creates a global network, a core network, a VPC attachment, accepts it, then
/// verifies the attachment state and tears everything down.
#[tokio::test]
async fn test_core_network_vpc_attachment_lifecycle() {
    // Scenario: Core network cloud WAN attachment
    // 1. Create global network
    // 2. Create core network
    // 3. Create VPC attachment to core network
    // 4. Accept the attachment
    // 5. Verify attachment is listed and has correct state
    // 6. Delete attachment
    // 7. Delete core network

    let client = make_client().await;

    // Step 1: Global network
    let gn = client
        .create_global_network()
        .description("CloudWAN test")
        .send()
        .await
        .expect("create_global_network");
    let gn_id = gn
        .global_network()
        .unwrap()
        .global_network_id()
        .unwrap()
        .to_string();

    // Step 2: Core network
    let cn = client
        .create_core_network()
        .global_network_id(&gn_id)
        .description("Test Core Network")
        .send()
        .await
        .expect("create_core_network");
    let cn_id = cn
        .core_network()
        .unwrap()
        .core_network_id()
        .unwrap()
        .to_string();

    // Step 3: VPC attachment
    let att = client
        .create_vpc_attachment()
        .core_network_id(&cn_id)
        .vpc_arn("arn:aws:ec2:us-east-1:123456789012:vpc/vpc-0123456789abcdef0")
        .subnet_arns("arn:aws:ec2:us-east-1:123456789012:subnet/subnet-0123456789abcdef0")
        .send()
        .await
        .expect("create_vpc_attachment");
    let att_id = att
        .vpc_attachment()
        .unwrap()
        .attachment()
        .unwrap()
        .attachment_id()
        .unwrap()
        .to_string();

    // Step 4: Accept the attachment
    let accepted = client
        .accept_attachment()
        .attachment_id(&att_id)
        .send()
        .await
        .expect("accept_attachment");
    assert_eq!(
        accepted.attachment().unwrap().state().map(|s| s.as_str()),
        Some("AVAILABLE"),
        "Accepted attachment should be AVAILABLE"
    );

    // Step 5: Verify it appears in list
    let list = client
        .list_attachments()
        .send()
        .await
        .expect("list_attachments");
    assert_eq!(list.attachments().len(), 1, "Should have 1 attachment");
    assert_eq!(
        list.attachments()[0].attachment_id(),
        Some(att_id.as_str()),
        "Listed attachment should match created one"
    );

    // Step 6: Delete attachment
    let deleted = client
        .delete_attachment()
        .attachment_id(&att_id)
        .send()
        .await
        .expect("delete_attachment");
    assert_eq!(
        deleted.attachment().unwrap().state().map(|s| s.as_str()),
        Some("DELETING"),
        "Deleted attachment should be DELETING"
    );

    let empty_list = client
        .list_attachments()
        .send()
        .await
        .expect("list_attachments after delete");
    assert_eq!(
        empty_list.attachments().len(),
        0,
        "No attachments should remain after deletion"
    );

    // Step 7: Delete core network
    let deleted_cn = client
        .delete_core_network()
        .core_network_id(&cn_id)
        .send()
        .await
        .expect("delete_core_network");
    assert_eq!(
        deleted_cn
            .core_network()
            .unwrap()
            .state()
            .map(|s| s.as_str()),
        Some("DELETING"),
        "Deleted core network should be DELETING"
    );
}

/// Scenario: Transit gateway registration lifecycle.
///
/// Creates a global network, registers a transit gateway, lists registrations,
/// and deregisters it, asserting proper state transitions throughout.
#[tokio::test]
async fn test_transit_gateway_registration_lifecycle() {
    // Scenario: TGW registration
    // 1. Create global network
    // 2. Register a transit gateway
    // 3. Verify it appears in the list
    // 4. Deregister the transit gateway
    // 5. Verify the list is empty

    let client = make_client().await;

    let tgw_arn = "arn:aws:ec2:us-east-1:123456789012:transit-gateway/tgw-0123456789abcdef0";

    // Step 1: Global network
    let gn = client
        .create_global_network()
        .description("TGW registration test")
        .send()
        .await
        .expect("create_global_network");
    let gn_id = gn
        .global_network()
        .unwrap()
        .global_network_id()
        .unwrap()
        .to_string();

    // Step 2: Register TGW
    let reg = client
        .register_transit_gateway()
        .global_network_id(&gn_id)
        .transit_gateway_arn(tgw_arn)
        .send()
        .await
        .expect("register_transit_gateway");
    assert_eq!(
        reg.transit_gateway_registration()
            .unwrap()
            .transit_gateway_arn(),
        Some(tgw_arn),
        "Registered TGW ARN should match"
    );

    // Step 3: Verify it appears
    let list = client
        .get_transit_gateway_registrations()
        .global_network_id(&gn_id)
        .send()
        .await
        .expect("get_transit_gateway_registrations");
    assert_eq!(
        list.transit_gateway_registrations().len(),
        1,
        "Should have 1 TGW registration"
    );

    // Step 4: Deregister
    let dereg = client
        .deregister_transit_gateway()
        .global_network_id(&gn_id)
        .transit_gateway_arn(tgw_arn)
        .send()
        .await
        .expect("deregister_transit_gateway");
    assert_eq!(
        dereg
            .transit_gateway_registration()
            .unwrap()
            .state()
            .and_then(|s| s.code())
            .map(|c| c.as_str()),
        Some("DELETING"),
        "Deregistered TGW state should be DELETING"
    );

    // Step 5: List is empty
    let empty = client
        .get_transit_gateway_registrations()
        .global_network_id(&gn_id)
        .send()
        .await
        .expect("get_transit_gateway_registrations after deregister");
    assert_eq!(
        empty.transit_gateway_registrations().len(),
        0,
        "No TGW registrations should remain after deregistration"
    );
}
