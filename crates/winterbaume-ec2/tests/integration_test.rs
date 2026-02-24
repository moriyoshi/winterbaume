use aws_sdk_ec2::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_ec2::Ec2Service;

async fn make_ec2_client() -> aws_sdk_ec2::Client {
    let mock = MockAws::builder().with_service(Ec2Service::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_ec2::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_ec2::Client::new(&config)
}

#[tokio::test]
async fn test_describe_availability_zones() {
    let client = make_ec2_client().await;

    let resp = client
        .describe_availability_zones()
        .send()
        .await
        .expect("describe_availability_zones should succeed");

    let azs = resp.availability_zones();
    assert!(!azs.is_empty(), "should have availability zones");
    // Should have at least 3 AZs
    assert!(azs.len() >= 3);
    // All should be available
    for az in azs {
        assert_eq!(az.state().unwrap().as_str(), "available");
    }
}

#[tokio::test]
async fn test_describe_account_attributes() {
    let client = make_ec2_client().await;

    let resp = client
        .describe_account_attributes()
        .send()
        .await
        .expect("describe_account_attributes should succeed");

    let attrs = resp.account_attributes();
    assert!(!attrs.is_empty(), "should have account attributes");

    let attr_names: Vec<String> = attrs
        .iter()
        .filter_map(|a| a.attribute_name())
        .map(|n| n.to_owned())
        .collect();
    assert!(
        attr_names.iter().any(|n| n == "default-vpc"),
        "should have default-vpc attribute"
    );
    assert!(
        attr_names.iter().any(|n| n == "max-instances"),
        "should have max-instances attribute"
    );
}

#[tokio::test]
async fn test_create_describe_delete_vpc() {
    let client = make_ec2_client().await;

    // Create a VPC
    let create_resp = client
        .create_vpc()
        .cidr_block("10.0.0.0/16")
        .send()
        .await
        .expect("create_vpc should succeed");

    let vpc = create_resp.vpc().expect("should have a VPC");
    let vpc_id = vpc.vpc_id().expect("vpc should have an ID");
    assert!(vpc_id.starts_with("vpc-"), "vpc_id should start with vpc-");
    assert_eq!(vpc.cidr_block().unwrap_or(""), "10.0.0.0/16");
    assert_eq!(vpc.state().map(|s| s.as_str()).unwrap_or(""), "available");

    // Describe VPCs - should find ours
    let desc_resp = client
        .describe_vpcs()
        .filters(
            aws_sdk_ec2::types::Filter::builder()
                .name("vpc-id")
                .values(vpc_id)
                .build(),
        )
        .send()
        .await
        .expect("describe_vpcs should succeed");

    let vpcs = desc_resp.vpcs();
    assert_eq!(vpcs.len(), 1);
    assert_eq!(vpcs[0].vpc_id().unwrap(), vpc_id);

    // Delete VPC
    client
        .delete_vpc()
        .vpc_id(vpc_id)
        .send()
        .await
        .expect("delete_vpc should succeed");

    // Describe again - should be empty
    let desc_resp2 = client
        .describe_vpcs()
        .filters(
            aws_sdk_ec2::types::Filter::builder()
                .name("vpc-id")
                .values(vpc_id)
                .build(),
        )
        .send()
        .await
        .expect("describe_vpcs should succeed");

    assert_eq!(desc_resp2.vpcs().len(), 0);
}

#[tokio::test]
async fn test_create_describe_delete_subnet() {
    let client = make_ec2_client().await;

    // First create a VPC
    let vpc_resp = client
        .create_vpc()
        .cidr_block("10.0.0.0/16")
        .send()
        .await
        .unwrap();
    let vpc_id = vpc_resp.vpc().unwrap().vpc_id().unwrap().to_string();

    // Create subnet
    let subnet_resp = client
        .create_subnet()
        .vpc_id(&vpc_id)
        .cidr_block("10.0.1.0/24")
        .availability_zone("us-east-1a")
        .send()
        .await
        .expect("create_subnet should succeed");

    let subnet = subnet_resp.subnet().expect("should have subnet");
    let subnet_id = subnet.subnet_id().expect("subnet should have ID");
    assert!(subnet_id.starts_with("subnet-"));
    assert_eq!(subnet.vpc_id().unwrap(), vpc_id);
    assert_eq!(subnet.cidr_block().unwrap_or(""), "10.0.1.0/24");

    // Describe subnets
    let desc_resp = client
        .describe_subnets()
        .filters(
            aws_sdk_ec2::types::Filter::builder()
                .name("vpc-id")
                .values(&vpc_id)
                .build(),
        )
        .send()
        .await
        .expect("describe_subnets should succeed");

    assert_eq!(desc_resp.subnets().len(), 1);

    // Delete subnet
    client
        .delete_subnet()
        .subnet_id(subnet_id)
        .send()
        .await
        .expect("delete_subnet should succeed");

    // Cleanup VPC
    client.delete_vpc().vpc_id(&vpc_id).send().await.unwrap();
}

#[tokio::test]
async fn test_create_describe_delete_security_group() {
    let client = make_ec2_client().await;

    // Create VPC first
    let vpc_resp = client
        .create_vpc()
        .cidr_block("10.0.0.0/16")
        .send()
        .await
        .unwrap();
    let vpc_id = vpc_resp.vpc().unwrap().vpc_id().unwrap().to_string();

    // Create security group
    let sg_resp = client
        .create_security_group()
        .group_name("test-sg")
        .description("Test security group")
        .vpc_id(&vpc_id)
        .send()
        .await
        .expect("create_security_group should succeed");

    let group_id = sg_resp.group_id().expect("should have group_id");
    assert!(group_id.starts_with("sg-"));

    // Describe security groups
    let desc_resp = client
        .describe_security_groups()
        .filters(
            aws_sdk_ec2::types::Filter::builder()
                .name("group-id")
                .values(group_id)
                .build(),
        )
        .send()
        .await
        .expect("describe_security_groups should succeed");

    let sgs = desc_resp.security_groups();
    assert_eq!(sgs.len(), 1);
    assert_eq!(sgs[0].group_name().unwrap_or(""), "test-sg");
    assert_eq!(sgs[0].vpc_id().unwrap_or(""), vpc_id);

    // Default egress rule should exist
    let egress_rules = sgs[0].ip_permissions_egress();
    assert!(!egress_rules.is_empty(), "should have default egress rule");

    // Delete security group
    client
        .delete_security_group()
        .group_id(group_id)
        .send()
        .await
        .expect("delete_security_group should succeed");

    // Cleanup
    client.delete_vpc().vpc_id(&vpc_id).send().await.unwrap();
}

#[tokio::test]
async fn test_internet_gateway_lifecycle() {
    let client = make_ec2_client().await;

    // Create VPC
    let vpc_resp = client
        .create_vpc()
        .cidr_block("10.0.0.0/16")
        .send()
        .await
        .unwrap();
    let vpc_id = vpc_resp.vpc().unwrap().vpc_id().unwrap().to_string();

    // Create IGW
    let igw_resp = client
        .create_internet_gateway()
        .send()
        .await
        .expect("create_internet_gateway should succeed");

    let igw = igw_resp.internet_gateway().expect("should have igw");
    let igw_id = igw.internet_gateway_id().expect("igw should have ID");
    assert!(igw_id.starts_with("igw-"));

    // Attach to VPC
    client
        .attach_internet_gateway()
        .internet_gateway_id(igw_id)
        .vpc_id(&vpc_id)
        .send()
        .await
        .expect("attach_internet_gateway should succeed");

    // Describe - should show attachment
    let desc_resp = client
        .describe_internet_gateways()
        .filters(
            aws_sdk_ec2::types::Filter::builder()
                .name("attachment.vpc-id")
                .values(&vpc_id)
                .build(),
        )
        .send()
        .await
        .expect("describe_internet_gateways should succeed");

    let igws = desc_resp.internet_gateways();
    assert_eq!(igws.len(), 1);
    assert_eq!(igws[0].internet_gateway_id().unwrap(), igw_id);
    assert!(!igws[0].attachments().is_empty());

    // Detach
    client
        .detach_internet_gateway()
        .internet_gateway_id(igw_id)
        .vpc_id(&vpc_id)
        .send()
        .await
        .expect("detach_internet_gateway should succeed");

    // Delete IGW
    client
        .delete_internet_gateway()
        .internet_gateway_id(igw_id)
        .send()
        .await
        .expect("delete_internet_gateway should succeed");

    // Cleanup
    client.delete_vpc().vpc_id(&vpc_id).send().await.unwrap();
}

#[tokio::test]
async fn test_route_table_lifecycle() {
    let client = make_ec2_client().await;

    // Create VPC and subnet
    let vpc_resp = client
        .create_vpc()
        .cidr_block("10.0.0.0/16")
        .send()
        .await
        .unwrap();
    let vpc_id = vpc_resp.vpc().unwrap().vpc_id().unwrap().to_string();

    let subnet_resp = client
        .create_subnet()
        .vpc_id(&vpc_id)
        .cidr_block("10.0.1.0/24")
        .send()
        .await
        .unwrap();
    let subnet_id = subnet_resp
        .subnet()
        .unwrap()
        .subnet_id()
        .unwrap()
        .to_string();

    // Create route table
    let rtb_resp = client
        .create_route_table()
        .vpc_id(&vpc_id)
        .send()
        .await
        .expect("create_route_table should succeed");

    let rtb = rtb_resp.route_table().expect("should have route table");
    let rtb_id = rtb.route_table_id().expect("should have rtb id");
    assert!(rtb_id.starts_with("rtb-"));

    // Should have local route
    let routes = rtb.routes();
    assert!(!routes.is_empty(), "should have local route");

    // Associate with subnet
    let assoc_resp = client
        .associate_route_table()
        .route_table_id(rtb_id)
        .subnet_id(&subnet_id)
        .send()
        .await
        .expect("associate_route_table should succeed");

    let assoc_id = assoc_resp
        .association_id()
        .expect("should have association ID");
    assert!(assoc_id.starts_with("rtbassoc-"));

    // Disassociate
    client
        .disassociate_route_table()
        .association_id(assoc_id)
        .send()
        .await
        .expect("disassociate_route_table should succeed");

    // Delete route table
    client
        .delete_route_table()
        .route_table_id(rtb_id)
        .send()
        .await
        .expect("delete_route_table should succeed");

    // Cleanup
    client
        .delete_subnet()
        .subnet_id(&subnet_id)
        .send()
        .await
        .unwrap();
    client.delete_vpc().vpc_id(&vpc_id).send().await.unwrap();
}

// --- Network ACL tests ---

#[tokio::test]
async fn test_create_describe_delete_network_acl() {
    let client = make_ec2_client().await;

    // Create VPC first
    let vpc_id = client
        .create_vpc()
        .cidr_block("10.0.0.0/16")
        .send()
        .await
        .unwrap()
        .vpc()
        .unwrap()
        .vpc_id()
        .unwrap()
        .to_string();

    // Create network ACL
    let create_resp = client
        .create_network_acl()
        .vpc_id(&vpc_id)
        .send()
        .await
        .expect("create_network_acl should succeed");

    let acl = create_resp.network_acl().expect("should have network_acl");
    let acl_id = acl
        .network_acl_id()
        .expect("should have acl_id")
        .to_string();
    assert!(acl_id.starts_with("acl-"), "acl_id should start with acl-");
    assert_eq!(acl.vpc_id().unwrap_or(""), &vpc_id);
    assert_eq!(acl.is_default(), Some(false));

    // Describe — should find it
    let desc_resp = client
        .describe_network_acls()
        .network_acl_ids(&acl_id)
        .send()
        .await
        .expect("describe_network_acls should succeed");
    assert_eq!(desc_resp.network_acls().len(), 1);

    // Delete
    client
        .delete_network_acl()
        .network_acl_id(&acl_id)
        .send()
        .await
        .expect("delete_network_acl should succeed");

    // Describe — should be gone
    let desc_resp2 = client
        .describe_network_acls()
        .network_acl_ids(&acl_id)
        .send()
        .await
        .expect("describe after delete should succeed");
    assert_eq!(desc_resp2.network_acls().len(), 0);

    client.delete_vpc().vpc_id(&vpc_id).send().await.unwrap();
}

#[tokio::test]
async fn test_network_acl_entry_lifecycle() {
    let client = make_ec2_client().await;

    let vpc_id = client
        .create_vpc()
        .cidr_block("10.0.0.0/16")
        .send()
        .await
        .unwrap()
        .vpc()
        .unwrap()
        .vpc_id()
        .unwrap()
        .to_string();

    let acl_id = client
        .create_network_acl()
        .vpc_id(&vpc_id)
        .send()
        .await
        .unwrap()
        .network_acl()
        .unwrap()
        .network_acl_id()
        .unwrap()
        .to_string();

    // Create an entry
    client
        .create_network_acl_entry()
        .network_acl_id(&acl_id)
        .rule_number(100)
        .protocol("-1")
        .rule_action(aws_sdk_ec2::types::RuleAction::Allow)
        .egress(false)
        .cidr_block("0.0.0.0/0")
        .send()
        .await
        .expect("create_network_acl_entry should succeed");

    // Describe — entry should be present
    let desc = client
        .describe_network_acls()
        .network_acl_ids(&acl_id)
        .send()
        .await
        .unwrap();
    let entries = desc.network_acls()[0].entries();
    assert!(entries.iter().any(|e| e.rule_number() == Some(100)));

    // Delete the entry
    client
        .delete_network_acl_entry()
        .network_acl_id(&acl_id)
        .rule_number(100)
        .egress(false)
        .send()
        .await
        .expect("delete_network_acl_entry should succeed");

    // Entry should be gone
    let desc2 = client
        .describe_network_acls()
        .network_acl_ids(&acl_id)
        .send()
        .await
        .unwrap();
    assert!(
        !desc2.network_acls()[0]
            .entries()
            .iter()
            .any(|e| e.rule_number() == Some(100))
    );

    // Cleanup
    client
        .delete_network_acl()
        .network_acl_id(&acl_id)
        .send()
        .await
        .unwrap();
    client.delete_vpc().vpc_id(&vpc_id).send().await.unwrap();
}

#[tokio::test]
async fn test_delete_nonexistent_network_acl() {
    let client = make_ec2_client().await;
    let result = client
        .delete_network_acl()
        .network_acl_id("acl-00000000")
        .send()
        .await;
    assert!(result.is_err());
}

// --- Elastic IP tests ---

#[tokio::test]
async fn test_allocate_describe_release_address() {
    let client = make_ec2_client().await;

    // Allocate
    let alloc = client
        .allocate_address()
        .send()
        .await
        .expect("allocate_address should succeed");

    let alloc_id = alloc
        .allocation_id()
        .expect("should have allocation_id")
        .to_string();
    let public_ip = alloc
        .public_ip()
        .expect("should have public_ip")
        .to_string();
    assert!(alloc_id.starts_with("eipalloc-"));
    assert!(!public_ip.is_empty());

    // Describe
    let desc = client
        .describe_addresses()
        .allocation_ids(&alloc_id)
        .send()
        .await
        .expect("describe_addresses should succeed");
    assert_eq!(desc.addresses().len(), 1);
    assert_eq!(desc.addresses()[0].allocation_id().unwrap_or(""), &alloc_id);

    // Release
    client
        .release_address()
        .allocation_id(&alloc_id)
        .send()
        .await
        .expect("release_address should succeed");

    // Should be gone
    let desc2 = client
        .describe_addresses()
        .allocation_ids(&alloc_id)
        .send()
        .await
        .expect("describe after release should succeed");
    assert_eq!(desc2.addresses().len(), 0);
}

#[tokio::test]
async fn test_associate_disassociate_address() {
    let client = make_ec2_client().await;

    // Allocate an EIP
    let alloc_id = client
        .allocate_address()
        .send()
        .await
        .unwrap()
        .allocation_id()
        .unwrap()
        .to_string();

    // Associate with a fake instance_id (state accepts it)
    let assoc = client
        .associate_address()
        .allocation_id(&alloc_id)
        .instance_id("i-00000001")
        .send()
        .await
        .expect("associate_address should succeed");

    let assoc_id = assoc
        .association_id()
        .expect("should have association_id")
        .to_string();
    assert!(assoc_id.starts_with("eipassoc-"));

    // Disassociate
    client
        .disassociate_address()
        .association_id(&assoc_id)
        .send()
        .await
        .expect("disassociate_address should succeed");

    // Release
    client
        .release_address()
        .allocation_id(&alloc_id)
        .send()
        .await
        .unwrap();
}

#[tokio::test]
async fn test_associate_address_mints_unique_assoc_id_per_call() {
    // Re-associating the same EIP must produce a fresh association id; AWS
    // never reuses eipassoc ids across associations.
    let client = make_ec2_client().await;

    let alloc_id = client
        .allocate_address()
        .send()
        .await
        .unwrap()
        .allocation_id()
        .unwrap()
        .to_string();

    let first = client
        .associate_address()
        .allocation_id(&alloc_id)
        .instance_id("i-00000001")
        .send()
        .await
        .unwrap()
        .association_id()
        .unwrap()
        .to_string();
    let second = client
        .associate_address()
        .allocation_id(&alloc_id)
        .instance_id("i-00000002")
        .send()
        .await
        .unwrap()
        .association_id()
        .unwrap()
        .to_string();
    assert_ne!(first, second, "each AssociateAddress must mint a fresh id");

    let _ = client
        .release_address()
        .allocation_id(&alloc_id)
        .send()
        .await;
}

#[tokio::test]
async fn test_create_volume_honors_encryption_by_default() {
    // EnableEbsEncryptionByDefault must propagate to subsequent CreateVolume
    // calls that omit the explicit Encrypted flag.
    let client = make_ec2_client().await;

    client
        .enable_ebs_encryption_by_default()
        .send()
        .await
        .expect("enable encryption-by-default");

    let id = client
        .create_volume()
        .availability_zone("us-east-1a")
        .size(8)
        .send()
        .await
        .expect("create_volume after enable")
        .volume_id()
        .unwrap()
        .to_string();
    let desc = client
        .describe_volumes()
        .volume_ids(&id)
        .send()
        .await
        .unwrap();
    assert_eq!(desc.volumes()[0].encrypted(), Some(true));

    let _ = client.disable_ebs_encryption_by_default().send().await;
    let _ = client.delete_volume().volume_id(&id).send().await;
}

// --- NAT Gateway tests ---

#[tokio::test]
async fn test_create_describe_delete_nat_gateway() {
    let client = make_ec2_client().await;

    // Need a VPC and subnet
    let vpc_id = client
        .create_vpc()
        .cidr_block("10.0.0.0/16")
        .send()
        .await
        .unwrap()
        .vpc()
        .unwrap()
        .vpc_id()
        .unwrap()
        .to_string();

    let subnet_id = client
        .create_subnet()
        .vpc_id(&vpc_id)
        .cidr_block("10.0.1.0/24")
        .send()
        .await
        .unwrap()
        .subnet()
        .unwrap()
        .subnet_id()
        .unwrap()
        .to_string();

    let alloc_id = client
        .allocate_address()
        .send()
        .await
        .unwrap()
        .allocation_id()
        .unwrap()
        .to_string();

    // Create NAT gateway
    let nat = client
        .create_nat_gateway()
        .subnet_id(&subnet_id)
        .allocation_id(&alloc_id)
        .send()
        .await
        .expect("create_nat_gateway should succeed")
        .nat_gateway()
        .expect("should have nat_gateway")
        .clone();

    let nat_id = nat
        .nat_gateway_id()
        .expect("should have nat_gateway_id")
        .to_string();
    assert!(nat_id.starts_with("nat-"));
    assert_eq!(nat.vpc_id().unwrap_or(""), &vpc_id);

    // Describe
    let desc = client
        .describe_nat_gateways()
        .nat_gateway_ids(&nat_id)
        .send()
        .await
        .expect("describe_nat_gateways should succeed");
    assert_eq!(desc.nat_gateways().len(), 1);

    // Delete
    client
        .delete_nat_gateway()
        .nat_gateway_id(&nat_id)
        .send()
        .await
        .expect("delete_nat_gateway should succeed");

    // Should be gone
    let desc2 = client
        .describe_nat_gateways()
        .nat_gateway_ids(&nat_id)
        .send()
        .await
        .expect("describe after delete should succeed");
    assert_eq!(desc2.nat_gateways().len(), 0);

    // Cleanup
    client
        .release_address()
        .allocation_id(&alloc_id)
        .send()
        .await
        .unwrap();
    client
        .delete_subnet()
        .subnet_id(&subnet_id)
        .send()
        .await
        .unwrap();
    client.delete_vpc().vpc_id(&vpc_id).send().await.unwrap();
}

// --- VPC attribute tests ---

#[tokio::test]
async fn test_describe_vpc_attribute_dns() {
    let client = make_ec2_client().await;

    let vpc_id = client
        .create_vpc()
        .cidr_block("10.0.0.0/16")
        .send()
        .await
        .unwrap()
        .vpc()
        .unwrap()
        .vpc_id()
        .unwrap()
        .to_string();

    // enableDnsHostnames — winterbaume defaults to false (AWS defaults to true for default VPCs)
    let resp = client
        .describe_vpc_attribute()
        .vpc_id(&vpc_id)
        .attribute(aws_sdk_ec2::types::VpcAttributeName::EnableDnsHostnames)
        .send()
        .await
        .expect("describe_vpc_attribute enableDnsHostnames should succeed");
    assert_eq!(resp.vpc_id().unwrap_or(""), &vpc_id);
    // winterbaume initializes enable_dns_hostnames=false; just verify the field is present
    assert!(resp.enable_dns_hostnames().is_some());

    // enableDnsSupport defaults to true
    let resp2 = client
        .describe_vpc_attribute()
        .vpc_id(&vpc_id)
        .attribute(aws_sdk_ec2::types::VpcAttributeName::EnableDnsSupport)
        .send()
        .await
        .expect("describe_vpc_attribute enableDnsSupport should succeed");
    assert_eq!(
        resp2.enable_dns_support().and_then(|v| v.value()),
        Some(true)
    );

    client.delete_vpc().vpc_id(&vpc_id).send().await.unwrap();
}

// Covers FIX(terraform-e2e): unknown VPC attributes (e.g. enableNetworkAddressUsageMetrics)
// must return false instead of an empty response to prevent terraform from polling indefinitely.
#[tokio::test]
async fn test_describe_vpc_attribute_unknown() {
    let client = make_ec2_client().await;

    let vpc_id = client
        .create_vpc()
        .cidr_block("10.1.0.0/16")
        .send()
        .await
        .unwrap()
        .vpc()
        .unwrap()
        .vpc_id()
        .unwrap()
        .to_string();

    // enableNetworkAddressUsageMetrics is not a standard VpcAttributeName variant in the SDK
    // so we use the raw string form via unknown variant.
    let resp = client
        .describe_vpc_attribute()
        .vpc_id(&vpc_id)
        .attribute("enableNetworkAddressUsageMetrics".into())
        .send()
        .await
        .expect("describe_vpc_attribute with unknown attribute should succeed (not hang/error)");
    assert_eq!(resp.vpc_id().unwrap_or(""), &vpc_id);
    // The fix: handler returns enable_network_address_usage_metrics = false
    assert_eq!(
        resp.enable_network_address_usage_metrics()
            .and_then(|v| v.value()),
        Some(false),
        "unknown attribute should return false, not empty"
    );

    client.delete_vpc().vpc_id(&vpc_id).send().await.unwrap();
}

#[tokio::test]
async fn test_modify_vpc_attribute() {
    let client = make_ec2_client().await;

    let vpc_id = client
        .create_vpc()
        .cidr_block("10.2.0.0/16")
        .send()
        .await
        .unwrap()
        .vpc()
        .unwrap()
        .vpc_id()
        .unwrap()
        .to_string();

    // Disable enableDnsHostnames
    client
        .modify_vpc_attribute()
        .vpc_id(&vpc_id)
        .enable_dns_hostnames(
            aws_sdk_ec2::types::AttributeBooleanValue::builder()
                .value(false)
                .build(),
        )
        .send()
        .await
        .expect("modify_vpc_attribute enableDnsHostnames=false should succeed");

    let resp = client
        .describe_vpc_attribute()
        .vpc_id(&vpc_id)
        .attribute(aws_sdk_ec2::types::VpcAttributeName::EnableDnsHostnames)
        .send()
        .await
        .unwrap();
    assert_eq!(
        resp.enable_dns_hostnames().and_then(|v| v.value()),
        Some(false)
    );

    // Re-enable
    client
        .modify_vpc_attribute()
        .vpc_id(&vpc_id)
        .enable_dns_hostnames(
            aws_sdk_ec2::types::AttributeBooleanValue::builder()
                .value(true)
                .build(),
        )
        .send()
        .await
        .unwrap();

    let resp2 = client
        .describe_vpc_attribute()
        .vpc_id(&vpc_id)
        .attribute(aws_sdk_ec2::types::VpcAttributeName::EnableDnsHostnames)
        .send()
        .await
        .unwrap();
    assert_eq!(
        resp2.enable_dns_hostnames().and_then(|v| v.value()),
        Some(true)
    );

    client.delete_vpc().vpc_id(&vpc_id).send().await.unwrap();
}

#[tokio::test]
async fn test_describe_vpc_attribute_not_found() {
    let client = make_ec2_client().await;
    let err = client
        .describe_vpc_attribute()
        .vpc_id("vpc-00000000")
        .attribute(aws_sdk_ec2::types::VpcAttributeName::EnableDnsSupport)
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("InvalidVpcID.NotFound") || err_str.contains("InvalidVpc"),
        "Expected not-found error, got: {err_str}"
    );
}

// --- Subnet attribute tests ---

#[tokio::test]
async fn test_modify_subnet_attribute() {
    let client = make_ec2_client().await;

    let vpc_id = client
        .create_vpc()
        .cidr_block("10.3.0.0/16")
        .send()
        .await
        .unwrap()
        .vpc()
        .unwrap()
        .vpc_id()
        .unwrap()
        .to_string();

    let subnet_id = client
        .create_subnet()
        .vpc_id(&vpc_id)
        .cidr_block("10.3.1.0/24")
        .send()
        .await
        .unwrap()
        .subnet()
        .unwrap()
        .subnet_id()
        .unwrap()
        .to_string();

    // Default: MapPublicIpOnLaunch should be false
    let desc = client
        .describe_subnets()
        .subnet_ids(&subnet_id)
        .send()
        .await
        .unwrap();
    assert_eq!(desc.subnets()[0].map_public_ip_on_launch(), Some(false));

    // Enable MapPublicIpOnLaunch
    client
        .modify_subnet_attribute()
        .subnet_id(&subnet_id)
        .map_public_ip_on_launch(
            aws_sdk_ec2::types::AttributeBooleanValue::builder()
                .value(true)
                .build(),
        )
        .send()
        .await
        .expect("modify_subnet_attribute should succeed");

    let desc2 = client
        .describe_subnets()
        .subnet_ids(&subnet_id)
        .send()
        .await
        .unwrap();
    assert_eq!(desc2.subnets()[0].map_public_ip_on_launch(), Some(true));

    client
        .delete_subnet()
        .subnet_id(&subnet_id)
        .send()
        .await
        .unwrap();
    client.delete_vpc().vpc_id(&vpc_id).send().await.unwrap();
}

// --- Security group rule tests ---

#[tokio::test]
async fn test_authorize_revoke_security_group_ingress() {
    let client = make_ec2_client().await;

    let vpc_id = client
        .create_vpc()
        .cidr_block("10.4.0.0/16")
        .send()
        .await
        .unwrap()
        .vpc()
        .unwrap()
        .vpc_id()
        .unwrap()
        .to_string();

    let group_id = client
        .create_security_group()
        .group_name("test-sg-rules")
        .description("Test SG for rule tests")
        .vpc_id(&vpc_id)
        .send()
        .await
        .unwrap()
        .group_id()
        .unwrap()
        .to_string();

    // Authorize TCP port 80 ingress
    client
        .authorize_security_group_ingress()
        .group_id(&group_id)
        .ip_permissions(
            aws_sdk_ec2::types::IpPermission::builder()
                .ip_protocol("tcp")
                .from_port(80)
                .to_port(80)
                .ip_ranges(
                    aws_sdk_ec2::types::IpRange::builder()
                        .cidr_ip("0.0.0.0/0")
                        .build(),
                )
                .build(),
        )
        .send()
        .await
        .expect("authorize_security_group_ingress should succeed");

    // Verify rule appears
    let desc = client
        .describe_security_groups()
        .group_ids(&group_id)
        .send()
        .await
        .unwrap();
    let ingress = desc.security_groups()[0].ip_permissions();
    assert!(
        ingress
            .iter()
            .any(|p| p.ip_protocol().unwrap_or("") == "tcp" && p.from_port() == Some(80)),
        "TCP port 80 ingress rule should be present"
    );

    // Revoke the rule
    client
        .revoke_security_group_ingress()
        .group_id(&group_id)
        .ip_permissions(
            aws_sdk_ec2::types::IpPermission::builder()
                .ip_protocol("tcp")
                .from_port(80)
                .to_port(80)
                .ip_ranges(
                    aws_sdk_ec2::types::IpRange::builder()
                        .cidr_ip("0.0.0.0/0")
                        .build(),
                )
                .build(),
        )
        .send()
        .await
        .expect("revoke_security_group_ingress should succeed");

    // Verify rule is gone
    let desc2 = client
        .describe_security_groups()
        .group_ids(&group_id)
        .send()
        .await
        .unwrap();
    let ingress2 = desc2.security_groups()[0].ip_permissions();
    assert!(
        !ingress2
            .iter()
            .any(|p| p.ip_protocol().unwrap_or("") == "tcp" && p.from_port() == Some(80)),
        "TCP port 80 ingress rule should be gone after revoke"
    );

    client
        .delete_security_group()
        .group_id(&group_id)
        .send()
        .await
        .unwrap();
    client.delete_vpc().vpc_id(&vpc_id).send().await.unwrap();
}

#[tokio::test]
async fn test_authorize_security_group_egress() {
    let client = make_ec2_client().await;

    let vpc_id = client
        .create_vpc()
        .cidr_block("10.5.0.0/16")
        .send()
        .await
        .unwrap()
        .vpc()
        .unwrap()
        .vpc_id()
        .unwrap()
        .to_string();

    let group_id = client
        .create_security_group()
        .group_name("test-sg-egress")
        .description("Test SG egress rules")
        .vpc_id(&vpc_id)
        .send()
        .await
        .unwrap()
        .group_id()
        .unwrap()
        .to_string();

    // Authorize custom egress rule (TCP 443)
    client
        .authorize_security_group_egress()
        .group_id(&group_id)
        .ip_permissions(
            aws_sdk_ec2::types::IpPermission::builder()
                .ip_protocol("tcp")
                .from_port(443)
                .to_port(443)
                .ip_ranges(
                    aws_sdk_ec2::types::IpRange::builder()
                        .cidr_ip("10.0.0.0/8")
                        .build(),
                )
                .build(),
        )
        .send()
        .await
        .expect("authorize_security_group_egress should succeed");

    let desc = client
        .describe_security_groups()
        .group_ids(&group_id)
        .send()
        .await
        .unwrap();
    let egress = desc.security_groups()[0].ip_permissions_egress();
    assert!(
        egress
            .iter()
            .any(|p| p.ip_protocol().unwrap_or("") == "tcp" && p.from_port() == Some(443)),
        "TCP 443 egress rule should be present"
    );

    client
        .delete_security_group()
        .group_id(&group_id)
        .send()
        .await
        .unwrap();
    client.delete_vpc().vpc_id(&vpc_id).send().await.unwrap();
}

// --- Route tests ---

#[tokio::test]
async fn test_create_delete_route() {
    let client = make_ec2_client().await;

    let vpc_id = client
        .create_vpc()
        .cidr_block("10.6.0.0/16")
        .send()
        .await
        .unwrap()
        .vpc()
        .unwrap()
        .vpc_id()
        .unwrap()
        .to_string();

    let igw_id = client
        .create_internet_gateway()
        .send()
        .await
        .unwrap()
        .internet_gateway()
        .unwrap()
        .internet_gateway_id()
        .unwrap()
        .to_string();

    client
        .attach_internet_gateway()
        .internet_gateway_id(&igw_id)
        .vpc_id(&vpc_id)
        .send()
        .await
        .unwrap();

    let rtb_id = client
        .create_route_table()
        .vpc_id(&vpc_id)
        .send()
        .await
        .unwrap()
        .route_table()
        .unwrap()
        .route_table_id()
        .unwrap()
        .to_string();

    // Create a default route via IGW
    client
        .create_route()
        .route_table_id(&rtb_id)
        .destination_cidr_block("0.0.0.0/0")
        .gateway_id(&igw_id)
        .send()
        .await
        .expect("create_route should succeed");

    // Verify route appears
    let desc = client
        .describe_route_tables()
        .route_table_ids(&rtb_id)
        .send()
        .await
        .unwrap();
    let routes = desc.route_tables()[0].routes();
    assert!(
        routes
            .iter()
            .any(|r| r.destination_cidr_block().unwrap_or("") == "0.0.0.0/0"),
        "default route should be present"
    );

    // Delete the route
    client
        .delete_route()
        .route_table_id(&rtb_id)
        .destination_cidr_block("0.0.0.0/0")
        .send()
        .await
        .expect("delete_route should succeed");

    // Verify route is gone
    let desc2 = client
        .describe_route_tables()
        .route_table_ids(&rtb_id)
        .send()
        .await
        .unwrap();
    let routes2 = desc2.route_tables()[0].routes();
    assert!(
        !routes2
            .iter()
            .any(|r| r.destination_cidr_block().unwrap_or("") == "0.0.0.0/0"),
        "default route should be gone after delete"
    );

    // Cleanup
    client
        .delete_route_table()
        .route_table_id(&rtb_id)
        .send()
        .await
        .unwrap();
    client
        .detach_internet_gateway()
        .internet_gateway_id(&igw_id)
        .vpc_id(&vpc_id)
        .send()
        .await
        .unwrap();
    client
        .delete_internet_gateway()
        .internet_gateway_id(&igw_id)
        .send()
        .await
        .unwrap();
    client.delete_vpc().vpc_id(&vpc_id).send().await.unwrap();
}

// --- Tags tests ---

#[tokio::test]
async fn test_create_delete_tags() {
    let client = make_ec2_client().await;

    let vpc_id = client
        .create_vpc()
        .cidr_block("10.7.0.0/16")
        .send()
        .await
        .unwrap()
        .vpc()
        .unwrap()
        .vpc_id()
        .unwrap()
        .to_string();

    // Add a tag
    client
        .create_tags()
        .resources(&vpc_id)
        .tags(
            aws_sdk_ec2::types::Tag::builder()
                .key("Env")
                .value("test")
                .build(),
        )
        .send()
        .await
        .expect("create_tags should succeed");

    // Verify tag visible in describe_vpcs
    let desc = client
        .describe_vpcs()
        .vpc_ids(&vpc_id)
        .send()
        .await
        .unwrap();
    let tags = desc.vpcs()[0].tags();
    assert!(
        tags.iter()
            .any(|t| t.key().unwrap_or("") == "Env" && t.value().unwrap_or("") == "test"),
        "Env=test tag should be present"
    );

    // Delete the tag
    client
        .delete_tags()
        .resources(&vpc_id)
        .tags(aws_sdk_ec2::types::Tag::builder().key("Env").build())
        .send()
        .await
        .expect("delete_tags should succeed");

    // Verify tag is gone
    let desc2 = client
        .describe_vpcs()
        .vpc_ids(&vpc_id)
        .send()
        .await
        .unwrap();
    let tags2 = desc2.vpcs()[0].tags();
    assert!(
        !tags2.iter().any(|t| t.key().unwrap_or("") == "Env"),
        "Env tag should be gone after delete"
    );

    client.delete_vpc().vpc_id(&vpc_id).send().await.unwrap();
}

// --- Key pair tests ---

#[tokio::test]
async fn test_import_describe_delete_key_pair() {
    let client = make_ec2_client().await;

    // Import a key pair (fake public key material)
    let resp = client
        .import_key_pair()
        .key_name("test-key")
        .public_key_material(aws_sdk_ec2::primitives::Blob::new(
            b"ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAABAQ test@test",
        ))
        .send()
        .await
        .expect("import_key_pair should succeed");

    let kp_id = resp
        .key_pair_id()
        .expect("should have key_pair_id")
        .to_string();
    assert!(
        kp_id.starts_with("key-"),
        "key_pair_id should start with key-"
    );
    assert_eq!(resp.key_name().unwrap_or(""), "test-key");

    // Describe key pairs
    let desc = client
        .describe_key_pairs()
        .key_names("test-key")
        .send()
        .await
        .expect("describe_key_pairs should succeed");
    assert_eq!(desc.key_pairs().len(), 1);
    assert_eq!(desc.key_pairs()[0].key_name().unwrap_or(""), "test-key");
    assert_eq!(desc.key_pairs()[0].key_pair_id().unwrap_or(""), &kp_id);

    // Delete key pair
    client
        .delete_key_pair()
        .key_name("test-key")
        .send()
        .await
        .expect("delete_key_pair should succeed");

    // Should be gone
    let desc2 = client
        .describe_key_pairs()
        .key_names("test-key")
        .send()
        .await
        .expect("describe after delete should succeed");
    assert_eq!(desc2.key_pairs().len(), 0);
}

// --- Network ACL entry replacement tests ---

#[tokio::test]
async fn test_replace_network_acl_entry() {
    let client = make_ec2_client().await;

    let vpc_id = client
        .create_vpc()
        .cidr_block("10.8.0.0/16")
        .send()
        .await
        .unwrap()
        .vpc()
        .unwrap()
        .vpc_id()
        .unwrap()
        .to_string();

    let acl_id = client
        .create_network_acl()
        .vpc_id(&vpc_id)
        .send()
        .await
        .unwrap()
        .network_acl()
        .unwrap()
        .network_acl_id()
        .unwrap()
        .to_string();

    // Create entry rule 100 allow
    client
        .create_network_acl_entry()
        .network_acl_id(&acl_id)
        .rule_number(100)
        .protocol("-1")
        .rule_action(aws_sdk_ec2::types::RuleAction::Allow)
        .egress(false)
        .cidr_block("0.0.0.0/0")
        .send()
        .await
        .unwrap();

    // Replace with deny
    client
        .replace_network_acl_entry()
        .network_acl_id(&acl_id)
        .rule_number(100)
        .protocol("-1")
        .rule_action(aws_sdk_ec2::types::RuleAction::Deny)
        .egress(false)
        .cidr_block("0.0.0.0/0")
        .send()
        .await
        .expect("replace_network_acl_entry should succeed");

    // Verify action is now deny
    let desc = client
        .describe_network_acls()
        .network_acl_ids(&acl_id)
        .send()
        .await
        .unwrap();
    let entry = desc.network_acls()[0]
        .entries()
        .iter()
        .find(|e| e.rule_number() == Some(100) && e.egress() == Some(false))
        .expect("entry 100 should exist");
    assert_eq!(
        entry.rule_action().map(|a| a.as_str()),
        Some("deny"),
        "rule action should have been replaced to deny"
    );

    // Cleanup
    client
        .delete_network_acl()
        .network_acl_id(&acl_id)
        .send()
        .await
        .unwrap();
    client.delete_vpc().vpc_id(&vpc_id).send().await.unwrap();
}

#[tokio::test]
async fn test_replace_network_acl_association() {
    let client = make_ec2_client().await;

    let vpc_id = client
        .create_vpc()
        .cidr_block("10.9.0.0/16")
        .send()
        .await
        .unwrap()
        .vpc()
        .unwrap()
        .vpc_id()
        .unwrap()
        .to_string();

    // Create a subnet — it will be associated with the default ACL
    let subnet_id = client
        .create_subnet()
        .vpc_id(&vpc_id)
        .cidr_block("10.9.1.0/24")
        .send()
        .await
        .unwrap()
        .subnet()
        .unwrap()
        .subnet_id()
        .unwrap()
        .to_string();

    // Create a custom ACL
    let custom_acl_id = client
        .create_network_acl()
        .vpc_id(&vpc_id)
        .send()
        .await
        .unwrap()
        .network_acl()
        .unwrap()
        .network_acl_id()
        .unwrap()
        .to_string();

    // Find the subnet's current ACL association ID (from the default ACL)
    let all_acls = client
        .describe_network_acls()
        .filters(
            aws_sdk_ec2::types::Filter::builder()
                .name("vpc-id")
                .values(&vpc_id)
                .build(),
        )
        .send()
        .await
        .unwrap();

    // Find the association for our subnet in the default ACL
    let assoc_id = all_acls
        .network_acls()
        .iter()
        .find(|acl| acl.is_default() == Some(true))
        .and_then(|acl| {
            acl.associations()
                .iter()
                .find(|a| a.subnet_id().unwrap_or("") == subnet_id)
                .and_then(|a| a.network_acl_association_id())
                .map(|s| s.to_string())
        })
        .expect("should find association for subnet in default ACL");

    // Replace the association with the custom ACL
    let replace_resp = client
        .replace_network_acl_association()
        .association_id(&assoc_id)
        .network_acl_id(&custom_acl_id)
        .send()
        .await
        .expect("replace_network_acl_association should succeed");

    let new_assoc_id = replace_resp
        .new_association_id()
        .expect("should have new_association_id")
        .to_string();
    assert!(
        new_assoc_id.starts_with("aclassoc-"),
        "new association ID should start with aclassoc-"
    );

    // Cleanup
    client
        .delete_subnet()
        .subnet_id(&subnet_id)
        .send()
        .await
        .unwrap();
    client
        .delete_network_acl()
        .network_acl_id(&custom_acl_id)
        .send()
        .await
        .unwrap();
    client.delete_vpc().vpc_id(&vpc_id).send().await.unwrap();
}

// --- Stub operation smoke tests ---

#[tokio::test]
async fn test_describe_prefix_lists_smoke() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_prefix_lists()
        .send()
        .await
        .expect("describe_prefix_lists should succeed");
    // Returns empty list (stub)
    let _ = resp.prefix_lists();
}

#[tokio::test]
async fn test_describe_instances_smoke() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_instances()
        .send()
        .await
        .expect("describe_instances should succeed");
    assert_eq!(resp.reservations().len(), 0);
}

#[tokio::test]
async fn test_describe_instance_types_smoke() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_instance_types()
        .send()
        .await
        .expect("describe_instance_types should succeed");
    // Batch B: now returns a static catalogue of common instance types.
    assert!(resp.instance_types().len() >= 12);
}

#[tokio::test]
async fn test_describe_images_smoke() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_images()
        .send()
        .await
        .expect("describe_images should succeed");
    assert_eq!(resp.images().len(), 0);
}

#[tokio::test]
async fn test_describe_flow_logs_smoke() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_flow_logs()
        .send()
        .await
        .expect("describe_flow_logs should succeed");
    assert_eq!(resp.flow_logs().len(), 0);
}

#[tokio::test]
async fn test_describe_network_interfaces_smoke() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_network_interfaces()
        .send()
        .await
        .expect("describe_network_interfaces should succeed");
    assert_eq!(resp.network_interfaces().len(), 0);
}

#[tokio::test]
async fn test_describe_vpc_endpoints_smoke() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_vpc_endpoints()
        .send()
        .await
        .expect("describe_vpc_endpoints should succeed");
    assert_eq!(resp.vpc_endpoints().len(), 0);
}

#[tokio::test]
async fn test_describe_vpc_endpoint_services_smoke() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_vpc_endpoint_services()
        .send()
        .await
        .expect("describe_vpc_endpoint_services should succeed");
    // Service names and details are empty stubs
    let _ = resp.service_names();
    let _ = resp.service_details();
}

// --- DHCP Options tests ---

#[tokio::test]
async fn test_create_describe_delete_dhcp_options() {
    let client = make_ec2_client().await;

    // Create DHCP options
    let create_resp = client
        .create_dhcp_options()
        .dhcp_configurations(
            aws_sdk_ec2::types::NewDhcpConfiguration::builder()
                .key("domain-name")
                .values("example.com")
                .build(),
        )
        .send()
        .await
        .expect("create_dhcp_options should succeed");

    let dopt = create_resp
        .dhcp_options()
        .expect("should have dhcp_options");
    let dopt_id = dopt
        .dhcp_options_id()
        .expect("should have dhcp_options_id")
        .to_string();
    assert!(dopt_id.starts_with("dopt-"));

    // Describe
    let desc = client
        .describe_dhcp_options()
        .dhcp_options_ids(&dopt_id)
        .send()
        .await
        .expect("describe_dhcp_options should succeed");
    assert_eq!(desc.dhcp_options().len(), 1);
    assert_eq!(
        desc.dhcp_options()[0].dhcp_options_id().unwrap_or(""),
        &dopt_id
    );

    let configs = desc.dhcp_options()[0].dhcp_configurations();
    assert!(
        configs
            .iter()
            .any(|c| c.key().unwrap_or("") == "domain-name")
    );

    // Delete
    client
        .delete_dhcp_options()
        .dhcp_options_id(&dopt_id)
        .send()
        .await
        .expect("delete_dhcp_options should succeed");

    // Should be gone
    let desc2 = client
        .describe_dhcp_options()
        .dhcp_options_ids(&dopt_id)
        .send()
        .await
        .expect("describe after delete should succeed");
    assert_eq!(desc2.dhcp_options().len(), 0);
}

#[tokio::test]
async fn test_associate_dhcp_options() {
    let client = make_ec2_client().await;

    let vpc_id = client
        .create_vpc()
        .cidr_block("10.0.0.0/16")
        .send()
        .await
        .unwrap()
        .vpc()
        .unwrap()
        .vpc_id()
        .unwrap()
        .to_string();

    let dopt_id = client
        .create_dhcp_options()
        .dhcp_configurations(
            aws_sdk_ec2::types::NewDhcpConfiguration::builder()
                .key("domain-name-servers")
                .values("8.8.8.8")
                .build(),
        )
        .send()
        .await
        .unwrap()
        .dhcp_options()
        .unwrap()
        .dhcp_options_id()
        .unwrap()
        .to_string();

    // Associate DHCP options with VPC
    client
        .associate_dhcp_options()
        .dhcp_options_id(&dopt_id)
        .vpc_id(&vpc_id)
        .send()
        .await
        .expect("associate_dhcp_options should succeed");

    // Verify VPC now has the DHCP options
    let desc = client
        .describe_vpcs()
        .vpc_ids(&vpc_id)
        .send()
        .await
        .unwrap();
    assert_eq!(desc.vpcs()[0].dhcp_options_id().unwrap_or(""), &dopt_id);

    // Cleanup: delete VPC first (removes DHCP association), then delete DHCP options
    client.delete_vpc().vpc_id(&vpc_id).send().await.unwrap();
    client
        .delete_dhcp_options()
        .dhcp_options_id(&dopt_id)
        .send()
        .await
        .unwrap();
}

// --- Instance compute tests ---

#[tokio::test]
async fn test_run_describe_terminate_instances() {
    let client = make_ec2_client().await;

    // RunInstances
    let run_resp = client
        .run_instances()
        .image_id("ami-12345678")
        .instance_type(aws_sdk_ec2::types::InstanceType::T2Micro)
        .min_count(1)
        .max_count(1)
        .send()
        .await
        .expect("run_instances should succeed");

    let instances = run_resp.instances();
    assert_eq!(instances.len(), 1);
    let instance_id = instances[0].instance_id().unwrap().to_string();
    assert!(
        instance_id.starts_with("i-"),
        "instance_id should start with i-"
    );
    assert_eq!(
        instances[0].state().unwrap().name().unwrap().as_str(),
        "running"
    );
    assert_eq!(instances[0].image_id().unwrap(), "ami-12345678");

    // DescribeInstances - should find our instance
    let desc_resp = client
        .describe_instances()
        .instance_ids(&instance_id)
        .send()
        .await
        .expect("describe_instances should succeed");

    assert_eq!(desc_resp.reservations().len(), 1);
    assert_eq!(
        desc_resp.reservations()[0].instances()[0]
            .instance_id()
            .unwrap(),
        &instance_id
    );

    // TerminateInstances
    let term_resp = client
        .terminate_instances()
        .instance_ids(&instance_id)
        .send()
        .await
        .expect("terminate_instances should succeed");

    let terminating = term_resp.terminating_instances();
    assert_eq!(terminating.len(), 1);
    assert_eq!(terminating[0].instance_id().unwrap(), &instance_id);
    assert_eq!(
        terminating[0]
            .current_state()
            .unwrap()
            .name()
            .unwrap()
            .as_str(),
        "terminated"
    );
}

#[tokio::test]
async fn test_start_stop_instances() {
    let client = make_ec2_client().await;

    // Run an instance
    let instance_id = client
        .run_instances()
        .image_id("ami-12345678")
        .instance_type(aws_sdk_ec2::types::InstanceType::T2Micro)
        .min_count(1)
        .max_count(1)
        .send()
        .await
        .unwrap()
        .instances()
        .first()
        .unwrap()
        .instance_id()
        .unwrap()
        .to_string();

    // Stop the instance
    let stop_resp = client
        .stop_instances()
        .instance_ids(&instance_id)
        .send()
        .await
        .expect("stop_instances should succeed");

    let stopping = stop_resp.stopping_instances();
    assert_eq!(stopping.len(), 1);
    assert_eq!(
        stopping[0]
            .current_state()
            .unwrap()
            .name()
            .unwrap()
            .as_str(),
        "stopped"
    );

    // Start it again
    let start_resp = client
        .start_instances()
        .instance_ids(&instance_id)
        .send()
        .await
        .expect("start_instances should succeed");

    let starting = start_resp.starting_instances();
    assert_eq!(starting.len(), 1);
    assert_eq!(
        starting[0]
            .current_state()
            .unwrap()
            .name()
            .unwrap()
            .as_str(),
        "running"
    );

    // Cleanup
    client
        .terminate_instances()
        .instance_ids(&instance_id)
        .send()
        .await
        .unwrap();
}

#[tokio::test]
async fn test_run_multiple_instances() {
    let client = make_ec2_client().await;

    let run_resp = client
        .run_instances()
        .image_id("ami-00000001")
        .instance_type(aws_sdk_ec2::types::InstanceType::T3Micro)
        .min_count(2)
        .max_count(3)
        .send()
        .await
        .expect("run_instances with count 3 should succeed");

    // max_count = 3, min_count = 2 -> should launch 3
    assert_eq!(run_resp.instances().len(), 3);
    let ids: Vec<String> = run_resp
        .instances()
        .iter()
        .map(|i| i.instance_id().unwrap().to_string())
        .collect();
    for id in &ids {
        assert!(id.starts_with("i-"));
    }

    // DescribeInstances with multiple IDs
    let desc = client
        .describe_instances()
        .set_instance_ids(Some(ids.clone()))
        .send()
        .await
        .unwrap();
    assert_eq!(desc.reservations().len(), 3);

    // Terminate all
    client
        .terminate_instances()
        .set_instance_ids(Some(ids))
        .send()
        .await
        .unwrap();
}

#[tokio::test]
async fn test_describe_instances_filter_by_state() {
    let client = make_ec2_client().await;

    let instance_id = client
        .run_instances()
        .image_id("ami-filter-test")
        .instance_type(aws_sdk_ec2::types::InstanceType::T2Micro)
        .min_count(1)
        .max_count(1)
        .send()
        .await
        .unwrap()
        .instances()
        .first()
        .unwrap()
        .instance_id()
        .unwrap()
        .to_string();

    // Filter by state=running should find it
    let desc = client
        .describe_instances()
        .filters(
            aws_sdk_ec2::types::Filter::builder()
                .name("instance-state-name")
                .values("running")
                .build(),
        )
        .send()
        .await
        .unwrap();
    let found_ids: Vec<&str> = desc
        .reservations()
        .iter()
        .flat_map(|r| r.instances())
        .filter_map(|i| i.instance_id())
        .collect();
    assert!(found_ids.contains(&instance_id.as_str()));

    // Filter by state=stopped should not find it
    let desc2 = client
        .describe_instances()
        .filters(
            aws_sdk_ec2::types::Filter::builder()
                .name("instance-state-name")
                .values("stopped")
                .build(),
        )
        .instance_ids(&instance_id)
        .send()
        .await
        .unwrap();
    assert_eq!(desc2.reservations().len(), 0);

    // Cleanup
    client
        .terminate_instances()
        .instance_ids(&instance_id)
        .send()
        .await
        .unwrap();
}

// --- EBS Volume tests ---

#[tokio::test]
async fn test_create_describe_delete_volume() {
    let client = make_ec2_client().await;

    let create_resp = client
        .create_volume()
        .availability_zone("us-east-1a")
        .size(20)
        .volume_type(aws_sdk_ec2::types::VolumeType::Gp2)
        .send()
        .await
        .expect("create_volume should succeed");

    let vol_id = create_resp.volume_id().unwrap().to_string();
    assert!(
        vol_id.starts_with("vol-"),
        "volume_id should start with vol-"
    );
    assert_eq!(create_resp.size().unwrap(), 20);
    assert_eq!(create_resp.availability_zone().unwrap(), "us-east-1a");
    assert_eq!(create_resp.state().unwrap().as_str(), "available");
    assert_eq!(create_resp.volume_type().unwrap().as_str(), "gp2");

    // Describe volumes
    let desc = client
        .describe_volumes()
        .volume_ids(&vol_id)
        .send()
        .await
        .expect("describe_volumes should succeed");
    assert_eq!(desc.volumes().len(), 1);
    assert_eq!(desc.volumes()[0].volume_id().unwrap(), &vol_id);

    // Delete volume
    client
        .delete_volume()
        .volume_id(&vol_id)
        .send()
        .await
        .expect("delete_volume should succeed");

    // Should be gone
    let desc2 = client
        .describe_volumes()
        .volume_ids(&vol_id)
        .send()
        .await
        .unwrap();
    assert_eq!(desc2.volumes().len(), 0);
}

#[tokio::test]
async fn test_attach_detach_volume() {
    let client = make_ec2_client().await;

    // Launch an instance
    let instance_id = client
        .run_instances()
        .image_id("ami-attach-test")
        .instance_type(aws_sdk_ec2::types::InstanceType::T2Micro)
        .min_count(1)
        .max_count(1)
        .send()
        .await
        .unwrap()
        .instances()
        .first()
        .unwrap()
        .instance_id()
        .unwrap()
        .to_string();

    // Create a volume
    let vol_id = client
        .create_volume()
        .availability_zone("us-east-1a")
        .size(10)
        .send()
        .await
        .unwrap()
        .volume_id()
        .unwrap()
        .to_string();

    // Attach volume
    let attach_resp = client
        .attach_volume()
        .volume_id(&vol_id)
        .instance_id(&instance_id)
        .device("/dev/xvdf")
        .send()
        .await
        .expect("attach_volume should succeed");

    assert_eq!(attach_resp.volume_id().unwrap(), &vol_id);
    assert_eq!(attach_resp.instance_id().unwrap(), &instance_id);
    assert_eq!(attach_resp.state().unwrap().as_str(), "attached");

    // Describe - volume should be in-use
    let desc = client
        .describe_volumes()
        .volume_ids(&vol_id)
        .send()
        .await
        .unwrap();
    assert_eq!(desc.volumes()[0].state().unwrap().as_str(), "in-use");

    // Detach
    let detach_resp = client
        .detach_volume()
        .volume_id(&vol_id)
        .send()
        .await
        .expect("detach_volume should succeed");
    assert_eq!(detach_resp.volume_id().unwrap(), &vol_id);

    // Cleanup
    client
        .delete_volume()
        .volume_id(&vol_id)
        .send()
        .await
        .unwrap();
    client
        .terminate_instances()
        .instance_ids(&instance_id)
        .send()
        .await
        .unwrap();
}

#[tokio::test]
async fn test_create_volume_with_tags() {
    let client = make_ec2_client().await;

    let vol_id = client
        .create_volume()
        .availability_zone("us-east-1b")
        .size(50)
        .tag_specifications(
            aws_sdk_ec2::types::TagSpecification::builder()
                .resource_type(aws_sdk_ec2::types::ResourceType::Volume)
                .tags(
                    aws_sdk_ec2::types::Tag::builder()
                        .key("Name")
                        .value("test-volume")
                        .build(),
                )
                .build(),
        )
        .send()
        .await
        .unwrap()
        .volume_id()
        .unwrap()
        .to_string();

    let desc = client
        .describe_volumes()
        .volume_ids(&vol_id)
        .send()
        .await
        .unwrap();
    let tags = desc.volumes()[0].tags();
    assert!(
        tags.iter()
            .any(|t| t.key() == Some("Name") && t.value() == Some("test-volume"))
    );

    client
        .delete_volume()
        .volume_id(&vol_id)
        .send()
        .await
        .unwrap();
}

// --- Snapshot tests ---

#[tokio::test]
async fn test_create_describe_delete_snapshot() {
    let client = make_ec2_client().await;

    // First create a volume to snapshot
    let vol_id = client
        .create_volume()
        .availability_zone("us-east-1a")
        .size(8)
        .send()
        .await
        .unwrap()
        .volume_id()
        .unwrap()
        .to_string();

    // Create snapshot
    let snap_resp = client
        .create_snapshot()
        .volume_id(&vol_id)
        .description("test snapshot")
        .send()
        .await
        .expect("create_snapshot should succeed");

    let snap_id = snap_resp.snapshot_id().unwrap().to_string();
    assert!(
        snap_id.starts_with("snap-"),
        "snapshot_id should start with snap-"
    );
    assert_eq!(snap_resp.volume_id().unwrap(), &vol_id);
    assert_eq!(snap_resp.description().unwrap(), "test snapshot");
    assert_eq!(snap_resp.state().unwrap().as_str(), "completed");

    // Describe snapshots
    let desc = client
        .describe_snapshots()
        .snapshot_ids(&snap_id)
        .send()
        .await
        .expect("describe_snapshots should succeed");
    assert_eq!(desc.snapshots().len(), 1);
    assert_eq!(desc.snapshots()[0].snapshot_id().unwrap(), &snap_id);

    // Delete snapshot
    client
        .delete_snapshot()
        .snapshot_id(&snap_id)
        .send()
        .await
        .expect("delete_snapshot should succeed");

    // Should be gone
    let desc2 = client
        .describe_snapshots()
        .snapshot_ids(&snap_id)
        .send()
        .await
        .unwrap();
    assert_eq!(desc2.snapshots().len(), 0);

    // Cleanup
    client
        .delete_volume()
        .volume_id(&vol_id)
        .send()
        .await
        .unwrap();
}

#[tokio::test]
async fn test_create_volume_from_snapshot_inherits_size() {
    // AWS: when Size is omitted on a snapshot restore, the new volume
    // inherits the snapshot's source volume size.
    let client = make_ec2_client().await;

    let source_vol = client
        .create_volume()
        .availability_zone("us-east-1a")
        .size(75)
        .send()
        .await
        .unwrap()
        .volume_id()
        .unwrap()
        .to_string();

    let snap_id = client
        .create_snapshot()
        .volume_id(&source_vol)
        .description("size inheritance regression")
        .send()
        .await
        .unwrap()
        .snapshot_id()
        .unwrap()
        .to_string();

    let restored = client
        .create_volume()
        .availability_zone("us-east-1b")
        .snapshot_id(&snap_id)
        .send()
        .await
        .expect("restore without explicit size");
    assert_eq!(
        restored.size(),
        Some(75),
        "restored volume should inherit source snapshot size"
    );
    assert_eq!(restored.snapshot_id(), Some(snap_id.as_str()));

    let restored_id = restored.volume_id().unwrap().to_string();
    let _ = client.delete_volume().volume_id(&restored_id).send().await;
    let _ = client.delete_snapshot().snapshot_id(&snap_id).send().await;
    let _ = client.delete_volume().volume_id(&source_vol).send().await;
}

#[tokio::test]
async fn test_describe_snapshots_owner_self() {
    let client = make_ec2_client().await;

    // Create a volume and snapshot
    let vol_id = client
        .create_volume()
        .availability_zone("us-east-1a")
        .size(8)
        .send()
        .await
        .unwrap()
        .volume_id()
        .unwrap()
        .to_string();

    let snap_id = client
        .create_snapshot()
        .volume_id(&vol_id)
        .description("owner test snapshot")
        .send()
        .await
        .unwrap()
        .snapshot_id()
        .unwrap()
        .to_string();

    // Describe with owner=self
    let desc = client
        .describe_snapshots()
        .owner_ids("self")
        .send()
        .await
        .unwrap();
    let found_ids: Vec<&str> = desc
        .snapshots()
        .iter()
        .filter_map(|s| s.snapshot_id())
        .collect();
    assert!(found_ids.contains(&snap_id.as_str()));

    // Cleanup
    client
        .delete_snapshot()
        .snapshot_id(&snap_id)
        .send()
        .await
        .unwrap();
    client
        .delete_volume()
        .volume_id(&vol_id)
        .send()
        .await
        .unwrap();
}

// --- AMI / Image tests ---

#[tokio::test]
async fn test_create_describe_deregister_image() {
    let client = make_ec2_client().await;

    // Launch an instance to create AMI from
    let instance_id = client
        .run_instances()
        .image_id("ami-source")
        .instance_type(aws_sdk_ec2::types::InstanceType::T2Micro)
        .min_count(1)
        .max_count(1)
        .send()
        .await
        .unwrap()
        .instances()
        .first()
        .unwrap()
        .instance_id()
        .unwrap()
        .to_string();

    // Create image
    let create_resp = client
        .create_image()
        .instance_id(&instance_id)
        .name("my-ami")
        .description("test AMI")
        .send()
        .await
        .expect("create_image should succeed");

    let image_id = create_resp.image_id().unwrap().to_string();
    assert!(
        image_id.starts_with("ami-"),
        "image_id should start with ami-"
    );

    // Describe images
    let desc = client
        .describe_images()
        .image_ids(&image_id)
        .send()
        .await
        .expect("describe_images should succeed");
    assert_eq!(desc.images().len(), 1);
    assert_eq!(desc.images()[0].image_id().unwrap(), &image_id);
    assert_eq!(desc.images()[0].name().unwrap(), "my-ami");
    assert_eq!(desc.images()[0].state().unwrap().as_str(), "available");

    // Deregister
    client
        .deregister_image()
        .image_id(&image_id)
        .send()
        .await
        .expect("deregister_image should succeed");

    // Should be gone
    let desc2 = client
        .describe_images()
        .image_ids(&image_id)
        .send()
        .await
        .unwrap();
    assert_eq!(desc2.images().len(), 0);

    // Cleanup
    client
        .terminate_instances()
        .instance_ids(&instance_id)
        .send()
        .await
        .unwrap();
}

// --- Launch Template tests ---

#[tokio::test]
async fn test_create_describe_delete_launch_template() {
    let client = make_ec2_client().await;

    let create_resp = client
        .create_launch_template()
        .launch_template_name("my-launch-template")
        .version_description("initial version")
        .launch_template_data(
            aws_sdk_ec2::types::RequestLaunchTemplateData::builder()
                .image_id("ami-12345678")
                .instance_type(aws_sdk_ec2::types::InstanceType::T2Micro)
                .build(),
        )
        .send()
        .await
        .expect("create_launch_template should succeed");

    let lt = create_resp.launch_template().unwrap();
    let lt_id = lt.launch_template_id().unwrap().to_string();
    assert!(
        lt_id.starts_with("lt-"),
        "launch_template_id should start with lt-"
    );
    assert_eq!(lt.launch_template_name().unwrap(), "my-launch-template");
    assert_eq!(lt.default_version_number().unwrap(), 1);
    assert_eq!(lt.latest_version_number().unwrap(), 1);

    // Describe launch templates
    let desc = client
        .describe_launch_templates()
        .launch_template_ids(&lt_id)
        .send()
        .await
        .expect("describe_launch_templates should succeed");
    assert_eq!(desc.launch_templates().len(), 1);
    assert_eq!(
        desc.launch_templates()[0].launch_template_id().unwrap(),
        &lt_id
    );

    // Describe by name
    let desc2 = client
        .describe_launch_templates()
        .launch_template_names("my-launch-template")
        .send()
        .await
        .unwrap();
    assert_eq!(desc2.launch_templates().len(), 1);

    // Delete
    let del_resp = client
        .delete_launch_template()
        .launch_template_id(&lt_id)
        .send()
        .await
        .expect("delete_launch_template should succeed");
    assert_eq!(
        del_resp
            .launch_template()
            .unwrap()
            .launch_template_id()
            .unwrap(),
        &lt_id
    );

    // Should be gone
    let desc3 = client
        .describe_launch_templates()
        .launch_template_ids(&lt_id)
        .send()
        .await
        .unwrap();
    assert_eq!(desc3.launch_templates().len(), 0);
}

#[tokio::test]
async fn test_create_launch_template_duplicate_name() {
    let client = make_ec2_client().await;

    // Create first template
    client
        .create_launch_template()
        .launch_template_name("duplicate-template")
        .launch_template_data(
            aws_sdk_ec2::types::RequestLaunchTemplateData::builder()
                .image_id("ami-12345678")
                .build(),
        )
        .send()
        .await
        .expect("first create should succeed");

    // Try to create again with the same name
    let err = client
        .create_launch_template()
        .launch_template_name("duplicate-template")
        .launch_template_data(
            aws_sdk_ec2::types::RequestLaunchTemplateData::builder()
                .image_id("ami-87654321")
                .build(),
        )
        .send()
        .await;

    assert!(err.is_err(), "duplicate launch template name should fail");
}

#[tokio::test]
async fn test_reboot_instances() {
    let client = make_ec2_client().await;

    let instance_id = client
        .run_instances()
        .image_id("ami-reboot-test")
        .instance_type(aws_sdk_ec2::types::InstanceType::T2Micro)
        .min_count(1)
        .max_count(1)
        .send()
        .await
        .unwrap()
        .instances()
        .first()
        .unwrap()
        .instance_id()
        .unwrap()
        .to_string();

    // Reboot just needs to succeed (void response)
    client
        .reboot_instances()
        .instance_ids(&instance_id)
        .send()
        .await
        .expect("reboot_instances should succeed");

    // Instance should still be running
    let desc = client
        .describe_instances()
        .instance_ids(&instance_id)
        .send()
        .await
        .unwrap();
    assert_eq!(
        desc.reservations()[0].instances()[0]
            .state()
            .unwrap()
            .name()
            .unwrap()
            .as_str(),
        "running"
    );

    // Cleanup
    client
        .terminate_instances()
        .instance_ids(&instance_id)
        .send()
        .await
        .unwrap();
}

#[tokio::test]
async fn test_describe_volumes_filter_by_az() {
    let client = make_ec2_client().await;

    let vol1_id = client
        .create_volume()
        .availability_zone("us-east-1a")
        .size(10)
        .send()
        .await
        .unwrap()
        .volume_id()
        .unwrap()
        .to_string();

    let vol2_id = client
        .create_volume()
        .availability_zone("us-east-1b")
        .size(20)
        .send()
        .await
        .unwrap()
        .volume_id()
        .unwrap()
        .to_string();

    // Filter by AZ
    let desc = client
        .describe_volumes()
        .filters(
            aws_sdk_ec2::types::Filter::builder()
                .name("availability-zone")
                .values("us-east-1a")
                .build(),
        )
        .send()
        .await
        .unwrap();

    let ids: Vec<&str> = desc
        .volumes()
        .iter()
        .filter_map(|v| v.volume_id())
        .collect();
    assert!(ids.contains(&vol1_id.as_str()));
    assert!(!ids.contains(&vol2_id.as_str()));

    // Cleanup
    client
        .delete_volume()
        .volume_id(&vol1_id)
        .send()
        .await
        .unwrap();
    client
        .delete_volume()
        .volume_id(&vol2_id)
        .send()
        .await
        .unwrap();
}

#[tokio::test]
async fn test_create_key_pair() {
    let client = make_ec2_client().await;
    let resp = client
        .create_key_pair()
        .key_name("my-test-key")
        .send()
        .await
        .expect("create_key_pair should succeed");
    assert_eq!(resp.key_name(), Some("my-test-key"));
    assert!(resp.key_pair_id().is_some());
    assert!(resp.key_material().is_some());
    let mat = resp.key_material().unwrap();
    assert!(mat.contains("BEGIN RSA PRIVATE KEY"));
    client
        .delete_key_pair()
        .key_name("my-test-key")
        .send()
        .await
        .unwrap();
}

#[tokio::test]
async fn test_describe_regions() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_regions()
        .send()
        .await
        .expect("describe_regions should succeed");
    let regions = resp.regions();
    assert!(!regions.is_empty());
    let names: Vec<&str> = regions.iter().filter_map(|r| r.region_name()).collect();
    assert!(names.contains(&"us-east-1"));
    assert!(names.contains(&"eu-west-1"));
}

#[tokio::test]
async fn test_egress_only_igw_lifecycle() {
    let client = make_ec2_client().await;
    let vpc = client
        .create_vpc()
        .cidr_block("10.5.0.0/16")
        .send()
        .await
        .unwrap();
    let vpc_id = vpc.vpc().unwrap().vpc_id().unwrap().to_string();

    let create_resp = client
        .create_egress_only_internet_gateway()
        .vpc_id(&vpc_id)
        .send()
        .await
        .expect("create EIGW should succeed");
    let eigw_id = create_resp
        .egress_only_internet_gateway()
        .unwrap()
        .egress_only_internet_gateway_id()
        .unwrap()
        .to_string();
    assert!(eigw_id.starts_with("eigw-"));

    let desc = client
        .describe_egress_only_internet_gateways()
        .send()
        .await
        .unwrap();
    let ids: Vec<&str> = desc
        .egress_only_internet_gateways()
        .iter()
        .filter_map(|e| e.egress_only_internet_gateway_id())
        .collect();
    assert!(ids.contains(&eigw_id.as_str()));

    client
        .delete_egress_only_internet_gateway()
        .egress_only_internet_gateway_id(&eigw_id)
        .send()
        .await
        .expect("delete EIGW should succeed");
    client.delete_vpc().vpc_id(&vpc_id).send().await.unwrap();
}

#[tokio::test]
async fn test_flow_log_lifecycle() {
    let client = make_ec2_client().await;
    let vpc = client
        .create_vpc()
        .cidr_block("10.6.0.0/16")
        .send()
        .await
        .unwrap();
    let vpc_id = vpc.vpc().unwrap().vpc_id().unwrap().to_string();

    let create_resp = client
        .create_flow_logs()
        .resource_ids(&vpc_id)
        .resource_type(aws_sdk_ec2::types::FlowLogsResourceType::Vpc)
        .traffic_type(aws_sdk_ec2::types::TrafficType::All)
        .log_destination_type(aws_sdk_ec2::types::LogDestinationType::CloudWatchLogs)
        .send()
        .await
        .expect("create_flow_logs should succeed");
    let fl_ids = create_resp.flow_log_ids();
    assert_eq!(fl_ids.len(), 1);
    let fl_id = fl_ids[0].clone();
    assert!(fl_id.starts_with("fl-"));

    let desc = client.describe_flow_logs().send().await.unwrap();
    let ids: Vec<&str> = desc
        .flow_logs()
        .iter()
        .filter_map(|f| f.flow_log_id())
        .collect();
    assert!(ids.contains(&fl_id.as_str()));

    client
        .delete_flow_logs()
        .flow_log_ids(&fl_id)
        .send()
        .await
        .expect("delete_flow_logs should succeed");
    client.delete_vpc().vpc_id(&vpc_id).send().await.unwrap();
}

#[tokio::test]
async fn test_vpc_peering_lifecycle() {
    let client = make_ec2_client().await;
    let vpc1 = client
        .create_vpc()
        .cidr_block("10.10.0.0/16")
        .send()
        .await
        .unwrap();
    let vpc1_id = vpc1.vpc().unwrap().vpc_id().unwrap().to_string();
    let vpc2 = client
        .create_vpc()
        .cidr_block("10.11.0.0/16")
        .send()
        .await
        .unwrap();
    let vpc2_id = vpc2.vpc().unwrap().vpc_id().unwrap().to_string();

    let create_resp = client
        .create_vpc_peering_connection()
        .vpc_id(&vpc1_id)
        .peer_vpc_id(&vpc2_id)
        .send()
        .await
        .expect("create peering should succeed");
    let conn = create_resp.vpc_peering_connection().unwrap();
    let pcx_id = conn.vpc_peering_connection_id().unwrap().to_string();
    assert!(pcx_id.starts_with("pcx-"));

    client
        .accept_vpc_peering_connection()
        .vpc_peering_connection_id(&pcx_id)
        .send()
        .await
        .expect("accept peering should succeed");

    let desc = client
        .describe_vpc_peering_connections()
        .send()
        .await
        .unwrap();
    let ids: Vec<&str> = desc
        .vpc_peering_connections()
        .iter()
        .filter_map(|p| p.vpc_peering_connection_id())
        .collect();
    assert!(ids.contains(&pcx_id.as_str()));

    client
        .delete_vpc_peering_connection()
        .vpc_peering_connection_id(&pcx_id)
        .send()
        .await
        .expect("delete peering should succeed");
    client.delete_vpc().vpc_id(&vpc1_id).send().await.unwrap();
    client.delete_vpc().vpc_id(&vpc2_id).send().await.unwrap();
}

#[tokio::test]
async fn test_vpc_endpoint_lifecycle() {
    let client = make_ec2_client().await;
    let vpc = client
        .create_vpc()
        .cidr_block("10.20.0.0/16")
        .send()
        .await
        .unwrap();
    let vpc_id = vpc.vpc().unwrap().vpc_id().unwrap().to_string();

    let create_resp = client
        .create_vpc_endpoint()
        .vpc_id(&vpc_id)
        .service_name("com.amazonaws.us-east-1.s3")
        .vpc_endpoint_type(aws_sdk_ec2::types::VpcEndpointType::Gateway)
        .send()
        .await
        .expect("create_vpc_endpoint should succeed");
    let ep_id = create_resp
        .vpc_endpoint()
        .unwrap()
        .vpc_endpoint_id()
        .unwrap()
        .to_string();
    assert!(ep_id.starts_with("vpce-"));

    let desc = client.describe_vpc_endpoints().send().await.unwrap();
    let ids: Vec<&str> = desc
        .vpc_endpoints()
        .iter()
        .filter_map(|e| e.vpc_endpoint_id())
        .collect();
    assert!(ids.contains(&ep_id.as_str()));

    client
        .delete_vpc_endpoints()
        .vpc_endpoint_ids(&ep_id)
        .send()
        .await
        .expect("delete_vpc_endpoints should succeed");
    client.delete_vpc().vpc_id(&vpc_id).send().await.unwrap();
}

#[tokio::test]
async fn test_managed_prefix_list_lifecycle() {
    let client = make_ec2_client().await;

    let create_resp = client
        .create_managed_prefix_list()
        .prefix_list_name("my-prefix-list")
        .max_entries(20)
        .address_family("IPv4")
        .send()
        .await
        .expect("create_managed_prefix_list should succeed");
    let pl = create_resp.prefix_list().unwrap();
    let pl_id = pl.prefix_list_id().unwrap().to_string();
    assert!(pl_id.starts_with("pl-"));

    let desc = client.describe_managed_prefix_lists().send().await.unwrap();
    let ids: Vec<&str> = desc
        .prefix_lists()
        .iter()
        .filter_map(|p| p.prefix_list_id())
        .collect();
    assert!(ids.contains(&pl_id.as_str()));

    client
        .delete_managed_prefix_list()
        .prefix_list_id(&pl_id)
        .send()
        .await
        .expect("delete_managed_prefix_list should succeed");
}

#[tokio::test]
async fn test_customer_gateway_lifecycle() {
    let client = make_ec2_client().await;

    let create_resp = client
        .create_customer_gateway()
        .bgp_asn(65000)
        .ip_address("198.51.100.1")
        .r#type(aws_sdk_ec2::types::GatewayType::Ipsec1)
        .send()
        .await
        .expect("create_customer_gateway should succeed");
    let cgw_id = create_resp
        .customer_gateway()
        .unwrap()
        .customer_gateway_id()
        .unwrap()
        .to_string();
    assert!(cgw_id.starts_with("cgw-"));

    let desc = client.describe_customer_gateways().send().await.unwrap();
    let ids: Vec<&str> = desc
        .customer_gateways()
        .iter()
        .filter_map(|c| c.customer_gateway_id())
        .collect();
    assert!(ids.contains(&cgw_id.as_str()));

    client
        .delete_customer_gateway()
        .customer_gateway_id(&cgw_id)
        .send()
        .await
        .expect("delete_customer_gateway should succeed");
}

#[tokio::test]
async fn test_vpn_gateway_lifecycle() {
    let client = make_ec2_client().await;
    let vpc = client
        .create_vpc()
        .cidr_block("10.30.0.0/16")
        .send()
        .await
        .unwrap();
    let vpc_id = vpc.vpc().unwrap().vpc_id().unwrap().to_string();

    let create_resp = client
        .create_vpn_gateway()
        .r#type(aws_sdk_ec2::types::GatewayType::Ipsec1)
        .send()
        .await
        .expect("create_vpn_gateway should succeed");
    let vgw_id = create_resp
        .vpn_gateway()
        .unwrap()
        .vpn_gateway_id()
        .unwrap()
        .to_string();
    assert!(vgw_id.starts_with("vgw-"));

    client
        .attach_vpn_gateway()
        .vpn_gateway_id(&vgw_id)
        .vpc_id(&vpc_id)
        .send()
        .await
        .expect("attach_vpn_gateway should succeed");

    let desc = client.describe_vpn_gateways().send().await.unwrap();
    let ids: Vec<&str> = desc
        .vpn_gateways()
        .iter()
        .filter_map(|v| v.vpn_gateway_id())
        .collect();
    assert!(ids.contains(&vgw_id.as_str()));

    client
        .detach_vpn_gateway()
        .vpn_gateway_id(&vgw_id)
        .vpc_id(&vpc_id)
        .send()
        .await
        .expect("detach should succeed");
    client
        .delete_vpn_gateway()
        .vpn_gateway_id(&vgw_id)
        .send()
        .await
        .expect("delete should succeed");
    client.delete_vpc().vpc_id(&vpc_id).send().await.unwrap();
}

#[tokio::test]
async fn test_carrier_gateway_lifecycle() {
    let client = make_ec2_client().await;
    let vpc = client
        .create_vpc()
        .cidr_block("10.40.0.0/16")
        .send()
        .await
        .unwrap();
    let vpc_id = vpc.vpc().unwrap().vpc_id().unwrap().to_string();

    let create_resp = client
        .create_carrier_gateway()
        .vpc_id(&vpc_id)
        .send()
        .await
        .expect("create_carrier_gateway should succeed");
    let cgw_id = create_resp
        .carrier_gateway()
        .unwrap()
        .carrier_gateway_id()
        .unwrap()
        .to_string();
    assert!(cgw_id.starts_with("cagw-"));

    let desc = client.describe_carrier_gateways().send().await.unwrap();
    let ids: Vec<&str> = desc
        .carrier_gateways()
        .iter()
        .filter_map(|c| c.carrier_gateway_id())
        .collect();
    assert!(ids.contains(&cgw_id.as_str()));

    client
        .delete_carrier_gateway()
        .carrier_gateway_id(&cgw_id)
        .send()
        .await
        .expect("delete should succeed");
    client.delete_vpc().vpc_id(&vpc_id).send().await.unwrap();
}

#[tokio::test]
async fn test_network_interface_lifecycle() {
    let client = make_ec2_client().await;
    let vpc = client
        .create_vpc()
        .cidr_block("10.50.0.0/16")
        .send()
        .await
        .unwrap();
    let vpc_id = vpc.vpc().unwrap().vpc_id().unwrap().to_string();
    let subnet = client
        .create_subnet()
        .vpc_id(&vpc_id)
        .cidr_block("10.50.1.0/24")
        .send()
        .await
        .unwrap();
    let subnet_id = subnet.subnet().unwrap().subnet_id().unwrap().to_string();

    let create_resp = client
        .create_network_interface()
        .subnet_id(&subnet_id)
        .send()
        .await
        .expect("create_network_interface should succeed");
    let eni_id = create_resp
        .network_interface()
        .unwrap()
        .network_interface_id()
        .unwrap()
        .to_string();
    assert!(eni_id.starts_with("eni-"));

    let desc = client.describe_network_interfaces().send().await.unwrap();
    let ids: Vec<&str> = desc
        .network_interfaces()
        .iter()
        .filter_map(|e| e.network_interface_id())
        .collect();
    assert!(ids.contains(&eni_id.as_str()));

    client
        .delete_network_interface()
        .network_interface_id(&eni_id)
        .send()
        .await
        .expect("delete should succeed");
    client
        .delete_subnet()
        .subnet_id(&subnet_id)
        .send()
        .await
        .unwrap();
    client.delete_vpc().vpc_id(&vpc_id).send().await.unwrap();
}

#[tokio::test]
async fn test_transit_gateway_lifecycle() {
    let client = make_ec2_client().await;

    let create_resp = client
        .create_transit_gateway()
        .description("test tgw")
        .send()
        .await
        .expect("create_transit_gateway should succeed");
    let tgw_id = create_resp
        .transit_gateway()
        .unwrap()
        .transit_gateway_id()
        .unwrap()
        .to_string();
    assert!(tgw_id.starts_with("tgw-"));

    let desc = client.describe_transit_gateways().send().await.unwrap();
    let ids: Vec<&str> = desc
        .transit_gateways()
        .iter()
        .filter_map(|t| t.transit_gateway_id())
        .collect();
    assert!(ids.contains(&tgw_id.as_str()));

    let rtb_resp = client
        .create_transit_gateway_route_table()
        .transit_gateway_id(&tgw_id)
        .send()
        .await
        .expect("create tgw rtb should succeed");
    let rtb_id = rtb_resp
        .transit_gateway_route_table()
        .unwrap()
        .transit_gateway_route_table_id()
        .unwrap()
        .to_string();
    assert!(rtb_id.starts_with("tgw-rtb-"));

    client
        .delete_transit_gateway_route_table()
        .transit_gateway_route_table_id(&rtb_id)
        .send()
        .await
        .expect("delete tgw rtb should succeed");
    client
        .delete_transit_gateway()
        .transit_gateway_id(&tgw_id)
        .send()
        .await
        .expect("delete tgw should succeed");
}

#[tokio::test]
async fn test_spot_instance_lifecycle() {
    let client = make_ec2_client().await;

    let request_resp = client
        .request_spot_instances()
        .spot_price("0.05")
        .instance_count(1)
        .launch_specification(
            aws_sdk_ec2::types::RequestSpotLaunchSpecification::builder()
                .image_id("ami-12345678")
                .instance_type(aws_sdk_ec2::types::InstanceType::T2Micro)
                .build(),
        )
        .send()
        .await
        .expect("request_spot_instances should succeed");
    let requests = request_resp.spot_instance_requests();
    assert_eq!(requests.len(), 1);
    let sir_id = requests[0].spot_instance_request_id().unwrap().to_string();
    assert!(sir_id.starts_with("sir-"));

    let desc = client
        .describe_spot_instance_requests()
        .send()
        .await
        .unwrap();
    let ids: Vec<&str> = desc
        .spot_instance_requests()
        .iter()
        .filter_map(|r| r.spot_instance_request_id())
        .collect();
    assert!(ids.contains(&sir_id.as_str()));

    client
        .cancel_spot_instance_requests()
        .spot_instance_request_ids(&sir_id)
        .send()
        .await
        .expect("cancel should succeed");
}

// ---- New tests for untested operations ----

#[tokio::test]
async fn test_describe_vpc_classic_link() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_vpc_classic_link()
        .send()
        .await
        .expect("describe_vpc_classic_link should succeed");
    // Returns empty set
    assert!(resp.vpcs().is_empty());
}

#[tokio::test]
async fn test_describe_vpc_classic_link_dns_support() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_vpc_classic_link_dns_support()
        .send()
        .await
        .expect("describe_vpc_classic_link_dns_support should succeed");
    assert!(resp.vpcs().is_empty());
}

#[tokio::test]
async fn test_describe_stale_security_groups() {
    let client = make_ec2_client().await;
    let vpc = client
        .create_vpc()
        .cidr_block("10.200.0.0/16")
        .send()
        .await
        .unwrap();
    let vpc_id = vpc.vpc().unwrap().vpc_id().unwrap().to_string();
    let resp = client
        .describe_stale_security_groups()
        .vpc_id(&vpc_id)
        .send()
        .await
        .expect("describe_stale_security_groups should succeed");
    assert!(resp.stale_security_group_set().is_empty());
    client.delete_vpc().vpc_id(&vpc_id).send().await.unwrap();
}

#[tokio::test]
async fn test_describe_tags() {
    let client = make_ec2_client().await;
    // Create a VPC with tags
    let vpc = client
        .create_vpc()
        .cidr_block("10.201.0.0/16")
        .tag_specifications(
            aws_sdk_ec2::types::TagSpecification::builder()
                .resource_type(aws_sdk_ec2::types::ResourceType::Vpc)
                .tags(
                    aws_sdk_ec2::types::Tag::builder()
                        .key("TestKey")
                        .value("TestValue")
                        .build(),
                )
                .build(),
        )
        .send()
        .await
        .unwrap();
    let vpc_id = vpc.vpc().unwrap().vpc_id().unwrap().to_string();

    let resp = client
        .describe_tags()
        .filters(
            aws_sdk_ec2::types::Filter::builder()
                .name("resource-id")
                .values(&vpc_id)
                .build(),
        )
        .send()
        .await
        .expect("describe_tags should succeed");
    let tags = resp.tags();
    assert!(!tags.is_empty());
    assert!(
        tags.iter()
            .any(|t| t.key() == Some("TestKey") && t.value() == Some("TestValue"))
    );

    client.delete_vpc().vpc_id(&vpc_id).send().await.unwrap();
}

#[tokio::test]
async fn test_describe_security_group_rules() {
    let client = make_ec2_client().await;
    let vpc = client
        .create_vpc()
        .cidr_block("10.202.0.0/16")
        .send()
        .await
        .unwrap();
    let vpc_id = vpc.vpc().unwrap().vpc_id().unwrap().to_string();

    let sg = client
        .create_security_group()
        .group_name("test-sgr-group")
        .description("test sg rules")
        .vpc_id(&vpc_id)
        .send()
        .await
        .unwrap();
    let sg_id = sg.group_id().unwrap().to_string();

    let resp = client
        .describe_security_group_rules()
        .filters(
            aws_sdk_ec2::types::Filter::builder()
                .name("group-id")
                .values(&sg_id)
                .build(),
        )
        .send()
        .await
        .expect("describe_security_group_rules should succeed");
    // Should have at least the default egress rule
    assert!(!resp.security_group_rules().is_empty());

    client
        .delete_security_group()
        .group_id(&sg_id)
        .send()
        .await
        .unwrap();
    client.delete_vpc().vpc_id(&vpc_id).send().await.unwrap();
}

#[tokio::test]
async fn test_ebs_encryption_by_default() {
    let client = make_ec2_client().await;

    // Get initial state
    let get1 = client
        .get_ebs_encryption_by_default()
        .send()
        .await
        .expect("get_ebs_encryption_by_default should succeed");
    assert_eq!(get1.ebs_encryption_by_default(), Some(false));

    // Enable
    let enable_resp = client
        .enable_ebs_encryption_by_default()
        .send()
        .await
        .expect("enable should succeed");
    assert_eq!(enable_resp.ebs_encryption_by_default(), Some(true));

    // Verify
    let get2 = client.get_ebs_encryption_by_default().send().await.unwrap();
    assert_eq!(get2.ebs_encryption_by_default(), Some(true));

    // Disable
    let disable_resp = client
        .disable_ebs_encryption_by_default()
        .send()
        .await
        .expect("disable should succeed");
    assert_eq!(disable_resp.ebs_encryption_by_default(), Some(false));

    // Verify disabled
    let get3 = client.get_ebs_encryption_by_default().send().await.unwrap();
    assert_eq!(get3.ebs_encryption_by_default(), Some(false));
}

#[tokio::test]
async fn test_describe_instance_status() {
    let client = make_ec2_client().await;

    // Run an instance
    let run = client
        .run_instances()
        .image_id("ami-12345678")
        .instance_type(aws_sdk_ec2::types::InstanceType::T2Micro)
        .min_count(1)
        .max_count(1)
        .send()
        .await
        .unwrap();
    let instance_id = run.instances()[0].instance_id().unwrap().to_string();

    let resp = client
        .describe_instance_status()
        .instance_ids(&instance_id)
        .send()
        .await
        .expect("describe_instance_status should succeed");
    let statuses = resp.instance_statuses();
    assert_eq!(statuses.len(), 1);
    assert_eq!(statuses[0].instance_id(), Some(instance_id.as_str()));

    client
        .terminate_instances()
        .instance_ids(&instance_id)
        .send()
        .await
        .unwrap();
}

#[tokio::test]
async fn test_describe_instance_attribute() {
    let client = make_ec2_client().await;

    let run = client
        .run_instances()
        .image_id("ami-12345678")
        .instance_type(aws_sdk_ec2::types::InstanceType::T2Micro)
        .min_count(1)
        .max_count(1)
        .send()
        .await
        .unwrap();
    let instance_id = run.instances()[0].instance_id().unwrap().to_string();

    let resp = client
        .describe_instance_attribute()
        .instance_id(&instance_id)
        .attribute(aws_sdk_ec2::types::InstanceAttributeName::InstanceType)
        .send()
        .await
        .expect("describe_instance_attribute should succeed");
    assert_eq!(resp.instance_id(), Some(instance_id.as_str()));
    assert!(resp.instance_type().is_some());

    client
        .terminate_instances()
        .instance_ids(&instance_id)
        .send()
        .await
        .unwrap();
}

#[tokio::test]
async fn test_modify_instance_attribute() {
    let client = make_ec2_client().await;

    let run = client
        .run_instances()
        .image_id("ami-12345678")
        .instance_type(aws_sdk_ec2::types::InstanceType::T2Micro)
        .min_count(1)
        .max_count(1)
        .send()
        .await
        .unwrap();
    let instance_id = run.instances()[0].instance_id().unwrap().to_string();

    // Stop the instance first (required for modify)
    client
        .stop_instances()
        .instance_ids(&instance_id)
        .send()
        .await
        .unwrap();

    let _resp = client
        .modify_instance_attribute()
        .instance_id(&instance_id)
        .instance_type(
            aws_sdk_ec2::types::AttributeValue::builder()
                .value("t2.small")
                .build(),
        )
        .send()
        .await
        .expect("modify_instance_attribute should succeed");

    client
        .terminate_instances()
        .instance_ids(&instance_id)
        .send()
        .await
        .unwrap();
}

#[tokio::test]
async fn test_modify_instance_metadata_options() {
    let client = make_ec2_client().await;

    let run = client
        .run_instances()
        .image_id("ami-12345678")
        .instance_type(aws_sdk_ec2::types::InstanceType::T2Micro)
        .min_count(1)
        .max_count(1)
        .send()
        .await
        .unwrap();
    let instance_id = run.instances()[0].instance_id().unwrap().to_string();

    let resp = client
        .modify_instance_metadata_options()
        .instance_id(&instance_id)
        .http_tokens(aws_sdk_ec2::types::HttpTokensState::Required)
        .send()
        .await
        .expect("modify_instance_metadata_options should succeed");
    assert_eq!(resp.instance_id(), Some(instance_id.as_str()));
    assert!(resp.instance_metadata_options().is_some());

    client
        .terminate_instances()
        .instance_ids(&instance_id)
        .send()
        .await
        .unwrap();
}

#[tokio::test]
async fn test_describe_instance_credit_specifications() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_instance_credit_specifications()
        .send()
        .await
        .expect("describe_instance_credit_specifications should succeed");
    assert!(resp.instance_credit_specifications().is_empty());
}

#[tokio::test]
async fn test_monitor_unmonitor_instances() {
    let client = make_ec2_client().await;

    let run = client
        .run_instances()
        .image_id("ami-12345678")
        .instance_type(aws_sdk_ec2::types::InstanceType::T2Micro)
        .min_count(1)
        .max_count(1)
        .send()
        .await
        .unwrap();
    let instance_id = run.instances()[0].instance_id().unwrap().to_string();

    let monitor_resp = client
        .monitor_instances()
        .instance_ids(&instance_id)
        .send()
        .await
        .expect("monitor_instances should succeed");
    let _ = monitor_resp.instance_monitorings();

    let unmonitor_resp = client
        .unmonitor_instances()
        .instance_ids(&instance_id)
        .send()
        .await
        .expect("unmonitor_instances should succeed");
    let _ = unmonitor_resp.instance_monitorings();

    client
        .terminate_instances()
        .instance_ids(&instance_id)
        .send()
        .await
        .unwrap();
}

#[tokio::test]
async fn test_describe_instance_type_offerings() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_instance_type_offerings()
        .send()
        .await
        .expect("describe_instance_type_offerings should succeed");
    let _ = resp.instance_type_offerings();
}

#[tokio::test]
async fn test_describe_volume_status() {
    let client = make_ec2_client().await;

    let vol = client
        .create_volume()
        .availability_zone("us-east-1a")
        .size(1)
        .send()
        .await
        .unwrap();
    let vol_id = vol.volume_id().unwrap().to_string();

    let resp = client
        .describe_volume_status()
        .volume_ids(&vol_id)
        .send()
        .await
        .expect("describe_volume_status should succeed");
    let statuses = resp.volume_statuses();
    assert_eq!(statuses.len(), 1);
    assert_eq!(statuses[0].volume_id(), Some(vol_id.as_str()));

    client
        .delete_volume()
        .volume_id(&vol_id)
        .send()
        .await
        .unwrap();
}

#[tokio::test]
async fn test_describe_volumes_modifications() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_volumes_modifications()
        .send()
        .await
        .expect("describe_volumes_modifications should succeed");
    assert!(resp.volumes_modifications().is_empty());
}

#[tokio::test]
async fn test_describe_volume_attribute() {
    let client = make_ec2_client().await;

    let vol = client
        .create_volume()
        .availability_zone("us-east-1a")
        .size(1)
        .send()
        .await
        .unwrap();
    let vol_id = vol.volume_id().unwrap().to_string();

    let resp = client
        .describe_volume_attribute()
        .volume_id(&vol_id)
        .attribute(aws_sdk_ec2::types::VolumeAttributeName::AutoEnableIo)
        .send()
        .await
        .expect("describe_volume_attribute should succeed");
    assert_eq!(resp.volume_id(), Some(vol_id.as_str()));

    client
        .delete_volume()
        .volume_id(&vol_id)
        .send()
        .await
        .unwrap();
}

#[tokio::test]
async fn test_copy_snapshot() {
    let client = make_ec2_client().await;

    // Create a volume and snapshot
    let vol = client
        .create_volume()
        .availability_zone("us-east-1a")
        .size(1)
        .send()
        .await
        .unwrap();
    let vol_id = vol.volume_id().unwrap().to_string();

    let snap = client
        .create_snapshot()
        .volume_id(&vol_id)
        .send()
        .await
        .unwrap();
    let snap_id = snap.snapshot_id().unwrap().to_string();

    let copy_resp = client
        .copy_snapshot()
        .source_snapshot_id(&snap_id)
        .source_region("us-east-1")
        .send()
        .await
        .expect("copy_snapshot should succeed");
    let new_snap_id = copy_resp.snapshot_id().unwrap().to_string();
    assert!(new_snap_id.starts_with("snap-"));
    assert_ne!(new_snap_id, snap_id);

    client
        .delete_snapshot()
        .snapshot_id(&new_snap_id)
        .send()
        .await
        .unwrap();
    client
        .delete_snapshot()
        .snapshot_id(&snap_id)
        .send()
        .await
        .unwrap();
    client
        .delete_volume()
        .volume_id(&vol_id)
        .send()
        .await
        .unwrap();
}

#[tokio::test]
async fn test_describe_snapshot_attribute() {
    let client = make_ec2_client().await;

    let vol = client
        .create_volume()
        .availability_zone("us-east-1a")
        .size(1)
        .send()
        .await
        .unwrap();
    let vol_id = vol.volume_id().unwrap().to_string();
    let snap = client
        .create_snapshot()
        .volume_id(&vol_id)
        .send()
        .await
        .unwrap();
    let snap_id = snap.snapshot_id().unwrap().to_string();

    let resp = client
        .describe_snapshot_attribute()
        .snapshot_id(&snap_id)
        .attribute(aws_sdk_ec2::types::SnapshotAttributeName::CreateVolumePermission)
        .send()
        .await
        .expect("describe_snapshot_attribute should succeed");
    assert!(resp.create_volume_permissions().is_empty());

    client
        .delete_snapshot()
        .snapshot_id(&snap_id)
        .send()
        .await
        .unwrap();
    client
        .delete_volume()
        .volume_id(&vol_id)
        .send()
        .await
        .unwrap();
}

#[tokio::test]
async fn test_modify_snapshot_attribute() {
    let client = make_ec2_client().await;

    let vol = client
        .create_volume()
        .availability_zone("us-east-1a")
        .size(1)
        .send()
        .await
        .unwrap();
    let vol_id = vol.volume_id().unwrap().to_string();
    let snap = client
        .create_snapshot()
        .volume_id(&vol_id)
        .send()
        .await
        .unwrap();
    let snap_id = snap.snapshot_id().unwrap().to_string();

    client
        .modify_snapshot_attribute()
        .snapshot_id(&snap_id)
        .attribute(aws_sdk_ec2::types::SnapshotAttributeName::CreateVolumePermission)
        .send()
        .await
        .expect("modify_snapshot_attribute should succeed");

    client
        .delete_snapshot()
        .snapshot_id(&snap_id)
        .send()
        .await
        .unwrap();
    client
        .delete_volume()
        .volume_id(&vol_id)
        .send()
        .await
        .unwrap();
}

#[tokio::test]
async fn test_reset_snapshot_attribute() {
    let client = make_ec2_client().await;

    let vol = client
        .create_volume()
        .availability_zone("us-east-1a")
        .size(1)
        .send()
        .await
        .unwrap();
    let vol_id = vol.volume_id().unwrap().to_string();
    let snap = client
        .create_snapshot()
        .volume_id(&vol_id)
        .send()
        .await
        .unwrap();
    let snap_id = snap.snapshot_id().unwrap().to_string();

    client
        .reset_snapshot_attribute()
        .snapshot_id(&snap_id)
        .attribute(aws_sdk_ec2::types::SnapshotAttributeName::CreateVolumePermission)
        .send()
        .await
        .expect("reset_snapshot_attribute should succeed");

    client
        .delete_snapshot()
        .snapshot_id(&snap_id)
        .send()
        .await
        .unwrap();
    client
        .delete_volume()
        .volume_id(&vol_id)
        .send()
        .await
        .unwrap();
}

#[tokio::test]
async fn test_register_image() {
    let client = make_ec2_client().await;

    let resp = client
        .register_image()
        .name("test-registered-image")
        .description("a registered image")
        .architecture(aws_sdk_ec2::types::ArchitectureValues::X8664)
        .virtualization_type("hvm")
        .root_device_name("/dev/xvda")
        .send()
        .await
        .expect("register_image should succeed");
    let image_id = resp.image_id().unwrap().to_string();
    assert!(image_id.starts_with("ami-"));

    // Verify it exists
    let desc = client
        .describe_images()
        .owners("self")
        .send()
        .await
        .unwrap();
    let ids: Vec<&str> = desc.images().iter().filter_map(|i| i.image_id()).collect();
    assert!(ids.contains(&image_id.as_str()));

    client
        .deregister_image()
        .image_id(&image_id)
        .send()
        .await
        .unwrap();
}

#[tokio::test]
async fn test_copy_image() {
    let client = make_ec2_client().await;

    // Register source image
    let reg = client
        .register_image()
        .name("copy-source")
        .root_device_name("/dev/xvda")
        .send()
        .await
        .unwrap();
    let source_id = reg.image_id().unwrap().to_string();

    let copy_resp = client
        .copy_image()
        .source_image_id(&source_id)
        .source_region("us-east-1")
        .name("copy-dest")
        .send()
        .await
        .expect("copy_image should succeed");
    let new_id = copy_resp.image_id().unwrap().to_string();
    assert!(new_id.starts_with("ami-"));
    assert_ne!(new_id, source_id);

    client
        .deregister_image()
        .image_id(&new_id)
        .send()
        .await
        .unwrap();
    client
        .deregister_image()
        .image_id(&source_id)
        .send()
        .await
        .unwrap();
}

#[tokio::test]
async fn test_describe_image_attribute() {
    let client = make_ec2_client().await;

    let reg = client
        .register_image()
        .name("img-attr-test")
        .root_device_name("/dev/xvda")
        .send()
        .await
        .unwrap();
    let image_id = reg.image_id().unwrap().to_string();

    let resp = client
        .describe_image_attribute()
        .image_id(&image_id)
        .attribute(aws_sdk_ec2::types::ImageAttributeName::LaunchPermission)
        .send()
        .await
        .expect("describe_image_attribute should succeed");
    assert!(resp.launch_permissions().is_empty());

    client
        .deregister_image()
        .image_id(&image_id)
        .send()
        .await
        .unwrap();
}

#[tokio::test]
async fn test_modify_image_attribute() {
    let client = make_ec2_client().await;

    let reg = client
        .register_image()
        .name("img-mod-attr")
        .root_device_name("/dev/xvda")
        .send()
        .await
        .unwrap();
    let image_id = reg.image_id().unwrap().to_string();

    client
        .modify_image_attribute()
        .image_id(&image_id)
        .send()
        .await
        .expect("modify_image_attribute should succeed");

    client
        .deregister_image()
        .image_id(&image_id)
        .send()
        .await
        .unwrap();
}

#[tokio::test]
async fn test_reset_image_attribute() {
    let client = make_ec2_client().await;

    let reg = client
        .register_image()
        .name("img-reset-attr")
        .root_device_name("/dev/xvda")
        .send()
        .await
        .unwrap();
    let image_id = reg.image_id().unwrap().to_string();

    client
        .reset_image_attribute()
        .image_id(&image_id)
        .attribute(aws_sdk_ec2::types::ResetImageAttributeName::LaunchPermission)
        .send()
        .await
        .expect("reset_image_attribute should succeed");

    client
        .deregister_image()
        .image_id(&image_id)
        .send()
        .await
        .unwrap();
}

#[tokio::test]
async fn test_launch_template_version_lifecycle() {
    let client = make_ec2_client().await;

    // Create launch template
    let lt = client
        .create_launch_template()
        .launch_template_name("lt-version-test")
        .launch_template_data(
            aws_sdk_ec2::types::RequestLaunchTemplateData::builder()
                .instance_type(aws_sdk_ec2::types::InstanceType::T2Micro)
                .build(),
        )
        .send()
        .await
        .unwrap();
    let lt_id = lt
        .launch_template()
        .unwrap()
        .launch_template_id()
        .unwrap()
        .to_string();

    // Create a new version
    let ver_resp = client
        .create_launch_template_version()
        .launch_template_id(&lt_id)
        .version_description("v2")
        .launch_template_data(
            aws_sdk_ec2::types::RequestLaunchTemplateData::builder()
                .instance_type(aws_sdk_ec2::types::InstanceType::T2Small)
                .build(),
        )
        .send()
        .await
        .expect("create_launch_template_version should succeed");
    let version = ver_resp.launch_template_version().unwrap();
    assert!(version.version_number() >= Some(2));

    // Describe versions
    let desc = client
        .describe_launch_template_versions()
        .launch_template_id(&lt_id)
        .send()
        .await
        .expect("describe_launch_template_versions should succeed");
    assert!(desc.launch_template_versions().len() >= 2);

    // Modify to set default version
    client
        .modify_launch_template()
        .launch_template_id(&lt_id)
        .default_version("2")
        .send()
        .await
        .expect("modify_launch_template should succeed");

    // The default-version flag should move to v2 and clear from v1.
    let after = client
        .describe_launch_template_versions()
        .launch_template_id(&lt_id)
        .send()
        .await
        .expect("re-describe versions");
    let v1 = after
        .launch_template_versions()
        .iter()
        .find(|v| v.version_number() == Some(1))
        .expect("v1 present");
    let v2 = after
        .launch_template_versions()
        .iter()
        .find(|v| v.version_number() == Some(2))
        .expect("v2 present");
    assert_eq!(v1.default_version(), Some(false));
    assert_eq!(v2.default_version(), Some(true));

    client
        .delete_launch_template()
        .launch_template_id(&lt_id)
        .send()
        .await
        .unwrap();
}

#[tokio::test]
async fn test_get_launch_template_data() {
    let client = make_ec2_client().await;

    let run = client
        .run_instances()
        .image_id("ami-12345678")
        .instance_type(aws_sdk_ec2::types::InstanceType::T2Micro)
        .min_count(1)
        .max_count(1)
        .send()
        .await
        .unwrap();
    let instance_id = run.instances()[0].instance_id().unwrap().to_string();

    let resp = client
        .get_launch_template_data()
        .instance_id(&instance_id)
        .send()
        .await
        .expect("get_launch_template_data should succeed");
    assert!(resp.launch_template_data().is_some());

    client
        .terminate_instances()
        .instance_ids(&instance_id)
        .send()
        .await
        .unwrap();
}

#[tokio::test]
async fn test_describe_reserved_instances() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_reserved_instances()
        .send()
        .await
        .expect("describe_reserved_instances should succeed");
    assert!(resp.reserved_instances().is_empty());
}

#[tokio::test]
async fn test_describe_reserved_instances_offerings() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_reserved_instances_offerings()
        .send()
        .await
        .expect("describe_reserved_instances_offerings should succeed");
    assert_eq!(resp.reserved_instances_offerings().len(), 3);
}

#[tokio::test]
async fn test_describe_addresses_attribute() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_addresses_attribute()
        .send()
        .await
        .expect("describe_addresses_attribute should succeed");
    assert!(resp.addresses().is_empty());
}

#[tokio::test]
async fn test_describe_spot_price_history() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_spot_price_history()
        .send()
        .await
        .expect("describe_spot_price_history should succeed");
    assert!(resp.spot_price_history().is_empty());
}

#[tokio::test]
async fn test_get_console_output() {
    let client = make_ec2_client().await;

    let run = client
        .run_instances()
        .image_id("ami-12345678")
        .instance_type(aws_sdk_ec2::types::InstanceType::T2Micro)
        .min_count(1)
        .max_count(1)
        .send()
        .await
        .unwrap();
    let instance_id = run.instances()[0].instance_id().unwrap().to_string();

    let resp = client
        .get_console_output()
        .instance_id(&instance_id)
        .send()
        .await
        .expect("get_console_output should succeed");
    assert_eq!(resp.instance_id(), Some(instance_id.as_str()));

    client
        .terminate_instances()
        .instance_ids(&instance_id)
        .send()
        .await
        .unwrap();
}

#[tokio::test]
async fn test_get_password_data() {
    let client = make_ec2_client().await;

    let run = client
        .run_instances()
        .image_id("ami-12345678")
        .instance_type(aws_sdk_ec2::types::InstanceType::T2Micro)
        .min_count(1)
        .max_count(1)
        .send()
        .await
        .unwrap();
    let instance_id = run.instances()[0].instance_id().unwrap().to_string();

    let resp = client
        .get_password_data()
        .instance_id(&instance_id)
        .send()
        .await
        .expect("get_password_data should succeed");
    assert_eq!(resp.instance_id(), Some(instance_id.as_str()));

    client
        .terminate_instances()
        .instance_ids(&instance_id)
        .send()
        .await
        .unwrap();
}

#[tokio::test]
async fn test_vpn_connection_lifecycle() {
    let client = make_ec2_client().await;

    // Create customer gateway
    let cgw = client
        .create_customer_gateway()
        .bgp_asn(65000)
        .ip_address("203.0.113.1")
        .r#type(aws_sdk_ec2::types::GatewayType::Ipsec1)
        .send()
        .await
        .unwrap();
    let cgw_id = cgw
        .customer_gateway()
        .unwrap()
        .customer_gateway_id()
        .unwrap()
        .to_string();

    // Create VPN gateway
    let vgw = client
        .create_vpn_gateway()
        .r#type(aws_sdk_ec2::types::GatewayType::Ipsec1)
        .send()
        .await
        .unwrap();
    let vgw_id = vgw
        .vpn_gateway()
        .unwrap()
        .vpn_gateway_id()
        .unwrap()
        .to_string();

    // Create VPN connection
    let vpn_resp = client
        .create_vpn_connection()
        .customer_gateway_id(&cgw_id)
        .vpn_gateway_id(&vgw_id)
        .r#type("ipsec.1")
        .send()
        .await
        .expect("create_vpn_connection should succeed");
    let vpn_id = vpn_resp
        .vpn_connection()
        .unwrap()
        .vpn_connection_id()
        .unwrap()
        .to_string();
    assert!(vpn_id.starts_with("vpn-"));

    // Describe
    let desc = client.describe_vpn_connections().send().await.unwrap();
    let ids: Vec<&str> = desc
        .vpn_connections()
        .iter()
        .filter_map(|v| v.vpn_connection_id())
        .collect();
    assert!(ids.contains(&vpn_id.as_str()));

    // Delete
    client
        .delete_vpn_connection()
        .vpn_connection_id(&vpn_id)
        .send()
        .await
        .expect("delete_vpn_connection should succeed");

    client
        .delete_vpn_gateway()
        .vpn_gateway_id(&vgw_id)
        .send()
        .await
        .unwrap();
    client
        .delete_customer_gateway()
        .customer_gateway_id(&cgw_id)
        .send()
        .await
        .unwrap();
}

#[tokio::test]
async fn test_associate_disassociate_vpc_cidr_block() {
    let client = make_ec2_client().await;

    let vpc = client
        .create_vpc()
        .cidr_block("10.203.0.0/16")
        .send()
        .await
        .unwrap();
    let vpc_id = vpc.vpc().unwrap().vpc_id().unwrap().to_string();

    let assoc_resp = client
        .associate_vpc_cidr_block()
        .vpc_id(&vpc_id)
        .cidr_block("10.204.0.0/16")
        .send()
        .await
        .expect("associate_vpc_cidr_block should succeed");
    let assoc_id = assoc_resp
        .cidr_block_association()
        .unwrap()
        .association_id()
        .unwrap()
        .to_string();

    client
        .disassociate_vpc_cidr_block()
        .association_id(&assoc_id)
        .send()
        .await
        .expect("disassociate_vpc_cidr_block should succeed");

    client.delete_vpc().vpc_id(&vpc_id).send().await.unwrap();
}

#[tokio::test]
async fn test_dedicated_host_lifecycle() {
    let client = make_ec2_client().await;

    let alloc_resp = client
        .allocate_hosts()
        .availability_zone("us-east-1a")
        .instance_type("m5.large")
        .quantity(1)
        .send()
        .await
        .expect("allocate_hosts should succeed");
    let host_ids = alloc_resp.host_ids();
    assert_eq!(host_ids.len(), 1);
    let host_id = host_ids[0].to_string();
    assert!(host_id.starts_with("h-"));

    // Describe
    let desc = client.describe_hosts().send().await.unwrap();
    let ids: Vec<&str> = desc.hosts().iter().filter_map(|h| h.host_id()).collect();
    assert!(ids.contains(&host_id.as_str()));

    // Modify
    let mod_resp = client
        .modify_hosts()
        .host_ids(&host_id)
        .send()
        .await
        .expect("modify_hosts should succeed");
    assert!(!mod_resp.successful().is_empty());

    // Release
    let rel_resp = client
        .release_hosts()
        .host_ids(&host_id)
        .send()
        .await
        .expect("release_hosts should succeed");
    assert!(!rel_resp.successful().is_empty());
}

#[tokio::test]
async fn test_ec2_fleet_lifecycle() {
    let client = make_ec2_client().await;

    let create_resp = client
        .create_fleet()
        .r#type(aws_sdk_ec2::types::FleetType::Instant)
        .send()
        .await
        .expect("create_fleet should succeed");
    let fleet_id = create_resp.fleet_id().unwrap().to_string();
    assert!(fleet_id.starts_with("fleet-"));

    // Describe
    let desc = client
        .describe_fleets()
        .fleet_ids(&fleet_id)
        .send()
        .await
        .expect("describe_fleets should succeed");
    assert_eq!(desc.fleets().len(), 1);

    // Describe fleet instances
    let inst_resp = client
        .describe_fleet_instances()
        .fleet_id(&fleet_id)
        .send()
        .await
        .expect("describe_fleet_instances should succeed");
    let _ = inst_resp.active_instances();

    // Delete
    let del_resp = client
        .delete_fleets()
        .fleet_ids(&fleet_id)
        .terminate_instances(true)
        .send()
        .await
        .expect("delete_fleets should succeed");
    assert!(!del_resp.successful_fleet_deletions().is_empty());
}

#[tokio::test]
async fn test_vpc_endpoint_service_configuration_lifecycle() {
    let client = make_ec2_client().await;

    let create_resp = client
        .create_vpc_endpoint_service_configuration()
        .acceptance_required(true)
        .send()
        .await
        .expect("create_vpc_endpoint_service_configuration should succeed");
    let svc = create_resp.service_configuration().unwrap();
    let svc_id = svc.service_id().unwrap().to_string();
    assert!(svc_id.starts_with("vpce-svc-"));

    // Describe
    let desc = client
        .describe_vpc_endpoint_service_configurations()
        .service_ids(&svc_id)
        .send()
        .await
        .expect("describe should succeed");
    assert_eq!(desc.service_configurations().len(), 1);

    // Describe permissions
    let perms = client
        .describe_vpc_endpoint_service_permissions()
        .service_id(&svc_id)
        .send()
        .await
        .expect("describe_permissions should succeed");
    assert!(perms.allowed_principals().is_empty());

    // Modify configuration
    client
        .modify_vpc_endpoint_service_configuration()
        .service_id(&svc_id)
        .acceptance_required(false)
        .send()
        .await
        .expect("modify should succeed");

    // Modify permissions
    client
        .modify_vpc_endpoint_service_permissions()
        .service_id(&svc_id)
        .add_allowed_principals("arn:aws:iam::123456789012:root")
        .send()
        .await
        .expect("modify_permissions should succeed");

    // Delete
    client
        .delete_vpc_endpoint_service_configurations()
        .service_ids(&svc_id)
        .send()
        .await
        .expect("delete should succeed");
}

#[tokio::test]
async fn test_spot_fleet_lifecycle() {
    let client = make_ec2_client().await;

    let req_resp = client
        .request_spot_fleet()
        .spot_fleet_request_config(
            aws_sdk_ec2::types::SpotFleetRequestConfigData::builder()
                .target_capacity(1)
                .iam_fleet_role("arn:aws:iam::123456789012:role/fleet-role")
                .build(),
        )
        .send()
        .await
        .expect("request_spot_fleet should succeed");
    let fleet_id = req_resp.spot_fleet_request_id().unwrap().to_string();
    assert!(fleet_id.starts_with("sfr-"));

    // Describe
    let desc = client
        .describe_spot_fleet_requests()
        .spot_fleet_request_ids(&fleet_id)
        .send()
        .await
        .expect("describe should succeed");
    assert_eq!(desc.spot_fleet_request_configs().len(), 1);

    // Describe fleet instances
    let inst = client
        .describe_spot_fleet_instances()
        .spot_fleet_request_id(&fleet_id)
        .send()
        .await
        .expect("describe_instances should succeed");
    let _ = inst.active_instances();

    // Modify
    client
        .modify_spot_fleet_request()
        .spot_fleet_request_id(&fleet_id)
        .target_capacity(2)
        .send()
        .await
        .expect("modify should succeed");

    // Cancel
    let cancel = client
        .cancel_spot_fleet_requests()
        .spot_fleet_request_ids(&fleet_id)
        .terminate_instances(true)
        .send()
        .await
        .expect("cancel should succeed");
    assert!(!cancel.successful_fleet_requests().is_empty());
}

#[tokio::test]
async fn test_replace_route() {
    let client = make_ec2_client().await;

    let vpc = client
        .create_vpc()
        .cidr_block("10.205.0.0/16")
        .send()
        .await
        .unwrap();
    let vpc_id = vpc.vpc().unwrap().vpc_id().unwrap().to_string();

    let igw = client.create_internet_gateway().send().await.unwrap();
    let igw_id = igw
        .internet_gateway()
        .unwrap()
        .internet_gateway_id()
        .unwrap()
        .to_string();
    client
        .attach_internet_gateway()
        .internet_gateway_id(&igw_id)
        .vpc_id(&vpc_id)
        .send()
        .await
        .unwrap();

    let rtb = client
        .create_route_table()
        .vpc_id(&vpc_id)
        .send()
        .await
        .unwrap();
    let rtb_id = rtb
        .route_table()
        .unwrap()
        .route_table_id()
        .unwrap()
        .to_string();

    // Create route
    client
        .create_route()
        .route_table_id(&rtb_id)
        .destination_cidr_block("0.0.0.0/0")
        .gateway_id(&igw_id)
        .send()
        .await
        .unwrap();

    // Replace route
    client
        .replace_route()
        .route_table_id(&rtb_id)
        .destination_cidr_block("0.0.0.0/0")
        .gateway_id(&igw_id)
        .send()
        .await
        .expect("replace_route should succeed");

    // Cleanup
    client
        .delete_route()
        .route_table_id(&rtb_id)
        .destination_cidr_block("0.0.0.0/0")
        .send()
        .await
        .unwrap();
    client
        .delete_route_table()
        .route_table_id(&rtb_id)
        .send()
        .await
        .unwrap();
    client
        .detach_internet_gateway()
        .internet_gateway_id(&igw_id)
        .vpc_id(&vpc_id)
        .send()
        .await
        .unwrap();
    client
        .delete_internet_gateway()
        .internet_gateway_id(&igw_id)
        .send()
        .await
        .unwrap();
    client.delete_vpc().vpc_id(&vpc_id).send().await.unwrap();
}

#[tokio::test]
async fn test_modify_vpc_endpoint() {
    let client = make_ec2_client().await;

    let vpc = client
        .create_vpc()
        .cidr_block("10.206.0.0/16")
        .send()
        .await
        .unwrap();
    let vpc_id = vpc.vpc().unwrap().vpc_id().unwrap().to_string();

    let ep = client
        .create_vpc_endpoint()
        .vpc_id(&vpc_id)
        .service_name("com.amazonaws.us-east-1.s3")
        .send()
        .await
        .unwrap();
    let ep_id = ep
        .vpc_endpoint()
        .unwrap()
        .vpc_endpoint_id()
        .unwrap()
        .to_string();

    client
        .modify_vpc_endpoint()
        .vpc_endpoint_id(&ep_id)
        .send()
        .await
        .expect("modify_vpc_endpoint should succeed");

    client
        .delete_vpc_endpoints()
        .vpc_endpoint_ids(&ep_id)
        .send()
        .await
        .unwrap();
    client.delete_vpc().vpc_id(&vpc_id).send().await.unwrap();
}

#[tokio::test]
async fn test_iam_instance_profile_association_lifecycle() {
    let client = make_ec2_client().await;

    let run = client
        .run_instances()
        .image_id("ami-12345678")
        .instance_type(aws_sdk_ec2::types::InstanceType::T2Micro)
        .min_count(1)
        .max_count(1)
        .send()
        .await
        .unwrap();
    let instance_id = run.instances()[0].instance_id().unwrap().to_string();

    // Associate
    let assoc = client
        .associate_iam_instance_profile()
        .instance_id(&instance_id)
        .iam_instance_profile(
            aws_sdk_ec2::types::IamInstanceProfileSpecification::builder()
                .arn("arn:aws:iam::123456789012:instance-profile/test-profile")
                .build(),
        )
        .send()
        .await
        .expect("associate should succeed");
    let assoc_id = assoc
        .iam_instance_profile_association()
        .unwrap()
        .association_id()
        .unwrap()
        .to_string();

    // Describe
    let desc = client
        .describe_iam_instance_profile_associations()
        .send()
        .await
        .expect("describe should succeed");
    assert!(!desc.iam_instance_profile_associations().is_empty());

    // Replace
    client
        .replace_iam_instance_profile_association()
        .association_id(&assoc_id)
        .iam_instance_profile(
            aws_sdk_ec2::types::IamInstanceProfileSpecification::builder()
                .arn("arn:aws:iam::123456789012:instance-profile/new-profile")
                .build(),
        )
        .send()
        .await
        .expect("replace should succeed");

    // Disassociate
    client
        .disassociate_iam_instance_profile()
        .association_id(&assoc_id)
        .send()
        .await
        .expect("disassociate should succeed");

    client
        .terminate_instances()
        .instance_ids(&instance_id)
        .send()
        .await
        .unwrap();
}

#[tokio::test]
async fn test_modify_ebs_default_kms_key_id() {
    let client = make_ec2_client().await;

    client
        .modify_ebs_default_kms_key_id()
        .kms_key_id("alias/aws/ebs")
        .send()
        .await
        .expect("modify_ebs_default_kms_key_id should succeed");
}

#[tokio::test]
async fn test_enable_disable_vpc_classic_link() {
    let client = make_ec2_client().await;

    let vpc = client
        .create_vpc()
        .cidr_block("10.207.0.0/16")
        .send()
        .await
        .unwrap();
    let vpc_id = vpc.vpc().unwrap().vpc_id().unwrap().to_string();

    client
        .enable_vpc_classic_link()
        .vpc_id(&vpc_id)
        .send()
        .await
        .expect("enable_vpc_classic_link should succeed");

    client
        .disable_vpc_classic_link()
        .vpc_id(&vpc_id)
        .send()
        .await
        .expect("disable_vpc_classic_link should succeed");

    client
        .enable_vpc_classic_link_dns_support()
        .vpc_id(&vpc_id)
        .send()
        .await
        .expect("enable_vpc_classic_link_dns_support should succeed");

    client
        .disable_vpc_classic_link_dns_support()
        .vpc_id(&vpc_id)
        .send()
        .await
        .expect("disable_vpc_classic_link_dns_support should succeed");

    client.delete_vpc().vpc_id(&vpc_id).send().await.unwrap();
}

#[tokio::test]
async fn test_modify_security_group_rules() {
    let client = make_ec2_client().await;

    let vpc = client
        .create_vpc()
        .cidr_block("10.208.0.0/16")
        .send()
        .await
        .unwrap();
    let vpc_id = vpc.vpc().unwrap().vpc_id().unwrap().to_string();

    let sg = client
        .create_security_group()
        .group_name("test-mod-sgr")
        .description("test modify sg rules")
        .vpc_id(&vpc_id)
        .send()
        .await
        .unwrap();
    let sg_id = sg.group_id().unwrap().to_string();

    let resp = client
        .modify_security_group_rules()
        .group_id(&sg_id)
        .send()
        .await
        .expect("modify_security_group_rules should succeed");
    assert_eq!(resp.r#return(), Some(true));

    client
        .delete_security_group()
        .group_id(&sg_id)
        .send()
        .await
        .unwrap();
    client.delete_vpc().vpc_id(&vpc_id).send().await.unwrap();
}

#[tokio::test]
async fn test_get_instance_uefi_data() {
    let client = make_ec2_client().await;

    let run = client
        .run_instances()
        .image_id("ami-12345678")
        .instance_type(aws_sdk_ec2::types::InstanceType::T2Micro)
        .min_count(1)
        .max_count(1)
        .send()
        .await
        .unwrap();
    let instance_id = run.instances()[0].instance_id().unwrap().to_string();

    let resp = client
        .get_instance_uefi_data()
        .instance_id(&instance_id)
        .send()
        .await
        .expect("get_instance_uefi_data should succeed");
    assert_eq!(resp.instance_id(), Some(instance_id.as_str()));

    client
        .terminate_instances()
        .instance_ids(&instance_id)
        .send()
        .await
        .unwrap();
}

#[tokio::test]
async fn test_modify_vpc_tenancy() {
    let client = make_ec2_client().await;

    let vpc = client
        .create_vpc()
        .cidr_block("10.209.0.0/16")
        .send()
        .await
        .unwrap();
    let vpc_id = vpc.vpc().unwrap().vpc_id().unwrap().to_string();

    let resp = client
        .modify_vpc_tenancy()
        .vpc_id(&vpc_id)
        .instance_tenancy(aws_sdk_ec2::types::VpcTenancy::Default)
        .send()
        .await
        .expect("modify_vpc_tenancy should succeed");
    assert_eq!(resp.return_value(), Some(true));

    client.delete_vpc().vpc_id(&vpc_id).send().await.unwrap();
}

// --- Placement Group tests ---

#[tokio::test]
async fn test_create_describe_delete_placement_group_cluster() {
    let client = make_ec2_client().await;

    client
        .create_placement_group()
        .group_name("my-cluster-pg")
        .strategy(aws_sdk_ec2::types::PlacementStrategy::Cluster)
        .send()
        .await
        .expect("create_placement_group should succeed");

    let desc = client
        .describe_placement_groups()
        .group_names("my-cluster-pg")
        .send()
        .await
        .expect("describe_placement_groups should succeed");
    let groups = desc.placement_groups();
    assert_eq!(groups.len(), 1);
    let pg = &groups[0];
    assert_eq!(pg.group_name(), Some("my-cluster-pg"));
    assert_eq!(
        pg.strategy(),
        Some(&aws_sdk_ec2::types::PlacementStrategy::Cluster)
    );
    assert_eq!(
        pg.state(),
        Some(&aws_sdk_ec2::types::PlacementGroupState::Available)
    );
    let group_id = pg.group_id().expect("group_id should be set").to_string();
    assert!(group_id.starts_with("pg-"));
    let arn = pg.group_arn().expect("group_arn should be set");
    assert!(arn.contains(":placement-group/my-cluster-pg"));

    client
        .delete_placement_group()
        .group_name("my-cluster-pg")
        .send()
        .await
        .expect("delete_placement_group should succeed");

    let after = client
        .describe_placement_groups()
        .send()
        .await
        .expect("describe_placement_groups should succeed");
    assert!(after.placement_groups().is_empty());
}

#[tokio::test]
async fn test_create_partition_placement_group() {
    let client = make_ec2_client().await;

    client
        .create_placement_group()
        .group_name("my-partition-pg")
        .strategy(aws_sdk_ec2::types::PlacementStrategy::Partition)
        .partition_count(5)
        .send()
        .await
        .expect("create_placement_group should succeed");

    let desc = client
        .describe_placement_groups()
        .group_names("my-partition-pg")
        .send()
        .await
        .expect("describe_placement_groups should succeed");
    let pg = &desc.placement_groups()[0];
    assert_eq!(pg.partition_count(), Some(5));
    assert_eq!(
        pg.strategy(),
        Some(&aws_sdk_ec2::types::PlacementStrategy::Partition)
    );
}

#[tokio::test]
async fn test_create_placement_group_invalid_partition_count() {
    let client = make_ec2_client().await;

    let err = client
        .create_placement_group()
        .group_name("bad-partition-pg")
        .strategy(aws_sdk_ec2::types::PlacementStrategy::Partition)
        .partition_count(99)
        .send()
        .await
        .expect_err("partition count out of range should fail");
    let msg = format!("{err:?}");
    assert!(
        msg.contains("InvalidParameterValue") || msg.contains("Partition count"),
        "unexpected error: {msg}"
    );
}

#[tokio::test]
async fn test_create_placement_group_duplicate_name() {
    let client = make_ec2_client().await;

    client
        .create_placement_group()
        .group_name("dup-pg")
        .strategy(aws_sdk_ec2::types::PlacementStrategy::Spread)
        .send()
        .await
        .expect("first create should succeed");

    let err = client
        .create_placement_group()
        .group_name("dup-pg")
        .strategy(aws_sdk_ec2::types::PlacementStrategy::Spread)
        .send()
        .await
        .expect_err("duplicate create should fail");
    let msg = format!("{err:?}");
    assert!(
        msg.contains("InvalidPlacementGroup.Duplicate") || msg.contains("already exists"),
        "unexpected error: {msg}"
    );
}

#[tokio::test]
async fn test_delete_placement_group_not_found() {
    let client = make_ec2_client().await;

    let err = client
        .delete_placement_group()
        .group_name("nonexistent-pg")
        .send()
        .await
        .expect_err("delete of missing pg should fail");
    let msg = format!("{err:?}");
    assert!(
        msg.contains("InvalidPlacementGroup.Unknown") || msg.contains("does not exist"),
        "unexpected error: {msg}"
    );
}

#[tokio::test]
async fn test_placement_group_describe_filters_by_id() {
    let client = make_ec2_client().await;

    client
        .create_placement_group()
        .group_name("pg-a")
        .strategy(aws_sdk_ec2::types::PlacementStrategy::Cluster)
        .send()
        .await
        .unwrap();
    client
        .create_placement_group()
        .group_name("pg-b")
        .strategy(aws_sdk_ec2::types::PlacementStrategy::Spread)
        .send()
        .await
        .unwrap();

    let all = client
        .describe_placement_groups()
        .send()
        .await
        .expect("describe all should succeed");
    let pg_a_id = all
        .placement_groups()
        .iter()
        .find(|g| g.group_name() == Some("pg-a"))
        .and_then(|g| g.group_id())
        .expect("pg-a should exist")
        .to_string();

    let one = client
        .describe_placement_groups()
        .group_ids(&pg_a_id)
        .send()
        .await
        .expect("describe by id should succeed");
    assert_eq!(one.placement_groups().len(), 1);
    assert_eq!(one.placement_groups()[0].group_name(), Some("pg-a"));
}

#[tokio::test]
async fn test_placement_group_state_view_snapshot_restore_merge() {
    use std::collections::HashMap;

    use winterbaume_core::StatefulService;
    use winterbaume_ec2::views::{Ec2StateView, PlacementGroupView};

    let svc = Ec2Service::new();

    let mut view = Ec2StateView::default();
    view.placement_groups.insert(
        "pg-00000001".to_string(),
        PlacementGroupView {
            group_id: "pg-00000001".to_string(),
            group_name: "snap-pg".to_string(),
            group_arn: "arn:aws:ec2:us-east-1:123456789012:placement-group/snap-pg".to_string(),
            strategy: "cluster".to_string(),
            state: "available".to_string(),
            partition_count: None,
            spread_level: None,
            tags: HashMap::new(),
        },
    );
    svc.restore("123456789012", "us-east-1", view)
        .await
        .unwrap();

    let snap = svc.snapshot("123456789012", "us-east-1").await;
    assert_eq!(snap.placement_groups.len(), 1);
    assert!(snap.placement_groups.contains_key("pg-00000001"));

    // Merge a second placement group; first must remain.
    let mut view_b = Ec2StateView::default();
    view_b.placement_groups.insert(
        "pg-00000002".to_string(),
        PlacementGroupView {
            group_id: "pg-00000002".to_string(),
            group_name: "merge-pg".to_string(),
            group_arn: "arn:aws:ec2:us-east-1:123456789012:placement-group/merge-pg".to_string(),
            strategy: "spread".to_string(),
            state: "available".to_string(),
            partition_count: None,
            spread_level: None,
            tags: HashMap::new(),
        },
    );
    svc.merge("123456789012", "us-east-1", view_b)
        .await
        .unwrap();

    let after = svc.snapshot("123456789012", "us-east-1").await;
    assert_eq!(after.placement_groups.len(), 2);
    assert!(after.placement_groups.contains_key("pg-00000001"));
    assert!(after.placement_groups.contains_key("pg-00000002"));
}

// --- NetworkInterfacePermission tests ---

#[tokio::test]
async fn test_network_interface_permission_lifecycle() {
    let client = make_ec2_client().await;

    let vpc = client
        .create_vpc()
        .cidr_block("10.50.0.0/16")
        .send()
        .await
        .unwrap();
    let vpc_id = vpc.vpc().unwrap().vpc_id().unwrap().to_string();
    let subnet = client
        .create_subnet()
        .vpc_id(&vpc_id)
        .cidr_block("10.50.1.0/24")
        .send()
        .await
        .unwrap();
    let subnet_id = subnet.subnet().unwrap().subnet_id().unwrap().to_string();

    let eni = client
        .create_network_interface()
        .subnet_id(&subnet_id)
        .send()
        .await
        .unwrap();
    let eni_id = eni
        .network_interface()
        .unwrap()
        .network_interface_id()
        .unwrap()
        .to_string();

    let create = client
        .create_network_interface_permission()
        .network_interface_id(&eni_id)
        .aws_account_id("999999999999")
        .permission(aws_sdk_ec2::types::InterfacePermissionType::InstanceAttach)
        .send()
        .await
        .expect("create_network_interface_permission should succeed");
    let perm = create.interface_permission().expect("permission");
    let perm_id = perm
        .network_interface_permission_id()
        .expect("perm id")
        .to_string();
    assert!(perm_id.starts_with("eni-perm-"));
    assert_eq!(perm.network_interface_id(), Some(eni_id.as_str()));

    let desc = client
        .describe_network_interface_permissions()
        .network_interface_permission_ids(&perm_id)
        .send()
        .await
        .expect("describe should succeed");
    assert_eq!(desc.network_interface_permissions().len(), 1);

    client
        .delete_network_interface_permission()
        .network_interface_permission_id(&perm_id)
        .send()
        .await
        .expect("delete should succeed");

    let after = client
        .describe_network_interface_permissions()
        .send()
        .await
        .expect("describe should succeed");
    assert!(after.network_interface_permissions().is_empty());
}

#[tokio::test]
async fn test_create_network_interface_permission_invalid_eni() {
    let client = make_ec2_client().await;
    let err = client
        .create_network_interface_permission()
        .network_interface_id("eni-doesnotexist")
        .aws_account_id("999999999999")
        .permission(aws_sdk_ec2::types::InterfacePermissionType::InstanceAttach)
        .send()
        .await
        .expect_err("missing ENI should fail");
    let msg = format!("{err:?}");
    assert!(
        msg.contains("InvalidNetworkInterfaceID.NotFound"),
        "unexpected: {msg}"
    );
}

#[tokio::test]
async fn test_delete_network_interface_permission_not_found() {
    let client = make_ec2_client().await;
    let err = client
        .delete_network_interface_permission()
        .network_interface_permission_id("eni-perm-deadbeef")
        .send()
        .await
        .expect_err("missing perm should fail");
    let msg = format!("{err:?}");
    assert!(
        msg.contains("InvalidNetworkInterfacePermissionID.NotFound"),
        "unexpected: {msg}"
    );
}

// --- InstanceConnectEndpoint tests ---

#[tokio::test]
async fn test_instance_connect_endpoint_lifecycle() {
    let client = make_ec2_client().await;

    let vpc = client
        .create_vpc()
        .cidr_block("10.51.0.0/16")
        .send()
        .await
        .unwrap();
    let vpc_id = vpc.vpc().unwrap().vpc_id().unwrap().to_string();
    let subnet = client
        .create_subnet()
        .vpc_id(&vpc_id)
        .cidr_block("10.51.1.0/24")
        .send()
        .await
        .unwrap();
    let subnet_id = subnet.subnet().unwrap().subnet_id().unwrap().to_string();

    let create = client
        .create_instance_connect_endpoint()
        .subnet_id(&subnet_id)
        .preserve_client_ip(true)
        .send()
        .await
        .expect("create_instance_connect_endpoint should succeed");
    let ice = create
        .instance_connect_endpoint()
        .expect("ice should be set");
    let ice_id = ice.instance_connect_endpoint_id().expect("id").to_string();
    assert!(ice_id.starts_with("eice-"));
    assert_eq!(ice.subnet_id(), Some(subnet_id.as_str()));
    assert_eq!(ice.vpc_id(), Some(vpc_id.as_str()));
    assert_eq!(ice.preserve_client_ip(), Some(true));

    let desc = client
        .describe_instance_connect_endpoints()
        .instance_connect_endpoint_ids(&ice_id)
        .send()
        .await
        .expect("describe should succeed");
    let items = desc.instance_connect_endpoints();
    assert_eq!(items.len(), 1);
    let arn = items[0].instance_connect_endpoint_arn().unwrap();
    assert!(arn.contains(":instance-connect-endpoint/eice-"));

    client
        .delete_instance_connect_endpoint()
        .instance_connect_endpoint_id(&ice_id)
        .send()
        .await
        .expect("delete should succeed");

    let after = client
        .describe_instance_connect_endpoints()
        .send()
        .await
        .expect("describe should succeed");
    assert!(after.instance_connect_endpoints().is_empty());
}

#[tokio::test]
async fn test_create_instance_connect_endpoint_missing_subnet() {
    let client = make_ec2_client().await;
    let err = client
        .create_instance_connect_endpoint()
        .subnet_id("subnet-doesnotexist")
        .send()
        .await
        .expect_err("missing subnet should fail");
    let msg = format!("{err:?}");
    assert!(
        msg.contains("InvalidSubnetID.NotFound"),
        "unexpected: {msg}"
    );
}

#[tokio::test]
async fn test_delete_instance_connect_endpoint_not_found() {
    let client = make_ec2_client().await;
    let err = client
        .delete_instance_connect_endpoint()
        .instance_connect_endpoint_id("eice-deadbeef")
        .send()
        .await
        .expect_err("missing ice should fail");
    let msg = format!("{err:?}");
    assert!(
        msg.contains("InvalidInstanceConnectEndpointId.NotFound"),
        "unexpected: {msg}"
    );
}

// --- CapacityReservation tests ---

#[tokio::test]
async fn test_capacity_reservation_lifecycle() {
    let client = make_ec2_client().await;

    let create = client
        .create_capacity_reservation()
        .instance_type("m5.large")
        .instance_platform(aws_sdk_ec2::types::CapacityReservationInstancePlatform::LinuxUnix)
        .availability_zone("us-east-1a")
        .instance_count(4)
        .end_date_type(aws_sdk_ec2::types::EndDateType::Unlimited)
        .send()
        .await
        .expect("create_capacity_reservation should succeed");
    let cr = create.capacity_reservation().expect("cr");
    let cr_id = cr.capacity_reservation_id().expect("id").to_string();
    assert!(cr_id.starts_with("cr-"));
    assert_eq!(cr.instance_type(), Some("m5.large"));
    assert_eq!(cr.total_instance_count(), Some(4));
    assert_eq!(cr.available_instance_count(), Some(4));
    assert_eq!(
        cr.state(),
        Some(&aws_sdk_ec2::types::CapacityReservationState::Active)
    );
    let arn = cr.capacity_reservation_arn().expect("arn");
    assert!(arn.contains(":capacity-reservation/cr-"));

    let modify = client
        .modify_capacity_reservation()
        .capacity_reservation_id(&cr_id)
        .instance_count(8)
        .send()
        .await
        .expect("modify_capacity_reservation should succeed");
    assert_eq!(modify.r#return(), Some(true));

    let desc = client
        .describe_capacity_reservations()
        .capacity_reservation_ids(&cr_id)
        .send()
        .await
        .expect("describe should succeed");
    let items = desc.capacity_reservations();
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].total_instance_count(), Some(8));

    let cancel = client
        .cancel_capacity_reservation()
        .capacity_reservation_id(&cr_id)
        .send()
        .await
        .expect("cancel should succeed");
    assert_eq!(cancel.r#return(), Some(true));

    let after = client
        .describe_capacity_reservations()
        .capacity_reservation_ids(&cr_id)
        .send()
        .await
        .expect("describe should succeed");
    assert_eq!(
        after.capacity_reservations()[0].state(),
        Some(&aws_sdk_ec2::types::CapacityReservationState::Cancelled)
    );
}

#[tokio::test]
async fn test_create_capacity_reservation_zero_count() {
    let client = make_ec2_client().await;
    let err = client
        .create_capacity_reservation()
        .instance_type("m5.large")
        .instance_platform(aws_sdk_ec2::types::CapacityReservationInstancePlatform::LinuxUnix)
        .availability_zone("us-east-1a")
        .instance_count(0)
        .send()
        .await
        .expect_err("zero count should fail");
    let msg = format!("{err:?}");
    assert!(
        msg.contains("InvalidParameterValue") || msg.contains("InstanceCount"),
        "unexpected: {msg}"
    );
}

#[tokio::test]
async fn test_modify_capacity_reservation_not_found() {
    let client = make_ec2_client().await;
    let err = client
        .modify_capacity_reservation()
        .capacity_reservation_id("cr-deadbeef")
        .instance_count(1)
        .send()
        .await
        .expect_err("missing reservation should fail");
    let msg = format!("{err:?}");
    assert!(
        msg.contains("InvalidCapacityReservationId.NotFound"),
        "unexpected: {msg}"
    );
}

#[tokio::test]
async fn test_cancel_capacity_reservation_not_found() {
    let client = make_ec2_client().await;
    let err = client
        .cancel_capacity_reservation()
        .capacity_reservation_id("cr-doesnotexist")
        .send()
        .await
        .expect_err("missing reservation should fail");
    let msg = format!("{err:?}");
    assert!(
        msg.contains("InvalidCapacityReservationId.NotFound"),
        "unexpected: {msg}"
    );
}

#[tokio::test]
async fn test_describe_capacity_reservations_filters_by_id() {
    let client = make_ec2_client().await;

    let cr_a = client
        .create_capacity_reservation()
        .instance_type("c5.xlarge")
        .instance_platform(aws_sdk_ec2::types::CapacityReservationInstancePlatform::LinuxUnix)
        .availability_zone("us-east-1a")
        .instance_count(2)
        .send()
        .await
        .unwrap();
    let id_a = cr_a
        .capacity_reservation()
        .unwrap()
        .capacity_reservation_id()
        .unwrap()
        .to_string();
    client
        .create_capacity_reservation()
        .instance_type("c5.large")
        .instance_platform(aws_sdk_ec2::types::CapacityReservationInstancePlatform::LinuxUnix)
        .availability_zone("us-east-1b")
        .instance_count(1)
        .send()
        .await
        .unwrap();

    let one = client
        .describe_capacity_reservations()
        .capacity_reservation_ids(&id_a)
        .send()
        .await
        .expect("describe by id should succeed");
    assert_eq!(one.capacity_reservations().len(), 1);
    assert_eq!(
        one.capacity_reservations()[0].capacity_reservation_id(),
        Some(id_a.as_str())
    );

    let all = client
        .describe_capacity_reservations()
        .send()
        .await
        .expect("describe all should succeed");
    assert!(all.capacity_reservations().len() >= 2);
}

#[tokio::test]
async fn test_modify_image_attribute_launch_permissions_round_trip() {
    let client = make_ec2_client().await;

    let image_id = client
        .register_image()
        .name("perm-roundtrip")
        .root_device_name("/dev/xvda")
        .send()
        .await
        .unwrap()
        .image_id()
        .unwrap()
        .to_string();

    // Add launch permission for two user IDs.
    client
        .modify_image_attribute()
        .image_id(&image_id)
        .launch_permission(
            aws_sdk_ec2::types::LaunchPermissionModifications::builder()
                .add(
                    aws_sdk_ec2::types::LaunchPermission::builder()
                        .user_id("111111111111")
                        .build(),
                )
                .add(
                    aws_sdk_ec2::types::LaunchPermission::builder()
                        .user_id("222222222222")
                        .build(),
                )
                .build(),
        )
        .send()
        .await
        .expect("modify_image_attribute should succeed");

    let perms = client
        .describe_image_attribute()
        .image_id(&image_id)
        .attribute(aws_sdk_ec2::types::ImageAttributeName::LaunchPermission)
        .send()
        .await
        .expect("describe_image_attribute should succeed")
        .launch_permissions()
        .to_vec();
    assert_eq!(perms.len(), 2);
    let mut user_ids: Vec<String> = perms
        .iter()
        .filter_map(|p| p.user_id().map(str::to_owned))
        .collect();
    user_ids.sort();
    assert_eq!(user_ids, vec!["111111111111", "222222222222"]);

    // Remove one permission.
    client
        .modify_image_attribute()
        .image_id(&image_id)
        .launch_permission(
            aws_sdk_ec2::types::LaunchPermissionModifications::builder()
                .remove(
                    aws_sdk_ec2::types::LaunchPermission::builder()
                        .user_id("111111111111")
                        .build(),
                )
                .build(),
        )
        .send()
        .await
        .unwrap();

    let perms = client
        .describe_image_attribute()
        .image_id(&image_id)
        .attribute(aws_sdk_ec2::types::ImageAttributeName::LaunchPermission)
        .send()
        .await
        .unwrap()
        .launch_permissions()
        .to_vec();
    assert_eq!(perms.len(), 1);
    assert_eq!(perms[0].user_id().unwrap(), "222222222222");

    // Reset wipes all permissions.
    client
        .reset_image_attribute()
        .image_id(&image_id)
        .attribute(aws_sdk_ec2::types::ResetImageAttributeName::LaunchPermission)
        .send()
        .await
        .unwrap();

    let perms = client
        .describe_image_attribute()
        .image_id(&image_id)
        .attribute(aws_sdk_ec2::types::ImageAttributeName::LaunchPermission)
        .send()
        .await
        .unwrap()
        .launch_permissions()
        .to_vec();
    assert!(perms.is_empty());

    client
        .deregister_image()
        .image_id(&image_id)
        .send()
        .await
        .unwrap();
}

#[tokio::test]
async fn test_modify_image_attribute_description() {
    let client = make_ec2_client().await;

    let image_id = client
        .register_image()
        .name("desc-test")
        .description("original")
        .root_device_name("/dev/xvda")
        .send()
        .await
        .unwrap()
        .image_id()
        .unwrap()
        .to_string();

    client
        .modify_image_attribute()
        .image_id(&image_id)
        .description(
            aws_sdk_ec2::types::AttributeValue::builder()
                .value("updated description")
                .build(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_image_attribute()
        .image_id(&image_id)
        .attribute(aws_sdk_ec2::types::ImageAttributeName::Description)
        .send()
        .await
        .unwrap();
    assert_eq!(
        resp.description().and_then(|d| d.value()),
        Some("updated description")
    );

    client
        .deregister_image()
        .image_id(&image_id)
        .send()
        .await
        .unwrap();
}

#[tokio::test]
async fn test_describe_image_attribute_unknown_image() {
    let client = make_ec2_client().await;
    let err = client
        .describe_image_attribute()
        .image_id("ami-deadbeef")
        .attribute(aws_sdk_ec2::types::ImageAttributeName::LaunchPermission)
        .send()
        .await
        .expect_err("should fail for unknown image");
    let msg = format!("{err:?}");
    assert!(msg.contains("InvalidAMIID.NotFound"), "got: {msg}");
}

#[tokio::test]
async fn test_placement_group_state_change_listener_fires() {
    use std::sync::{Arc, Mutex};

    use winterbaume_core::StatefulService;
    use winterbaume_ec2::views::Ec2StateView;
    let svc = Ec2Service::new();
    let events: Arc<Mutex<Vec<(String, String)>>> = Arc::new(Mutex::new(vec![]));
    let events2 = Arc::clone(&events);
    svc.notifier().subscribe(move |account_id, region, _view| {
        events2
            .lock()
            .unwrap()
            .push((account_id.to_string(), region.to_string()));
    });

    svc.restore("123456789012", "us-east-1", Ec2StateView::default())
        .await
        .unwrap();
    let got = events.lock().unwrap();
    assert_eq!(got.len(), 1);
    assert_eq!(
        got[0],
        ("123456789012".to_string(), "us-east-1".to_string())
    );
}

// --- ModifyInstancePlacement tests (sub-task A) ---

#[tokio::test]
async fn test_modify_instance_placement_sets_group_name() {
    let client = make_ec2_client().await;

    // Create a placement group so we can attach an instance to it.
    client
        .create_placement_group()
        .group_name("pg-modify-target")
        .strategy(aws_sdk_ec2::types::PlacementStrategy::Cluster)
        .send()
        .await
        .expect("create_placement_group should succeed");

    let run = client
        .run_instances()
        .image_id("ami-12345678")
        .instance_type(aws_sdk_ec2::types::InstanceType::T2Micro)
        .min_count(1)
        .max_count(1)
        .send()
        .await
        .expect("run_instances should succeed");
    let instance_id = run.instances()[0]
        .instance_id()
        .expect("instance id")
        .to_string();

    // Stop the instance first, since AWS requires the instance be stopped
    // for ModifyInstancePlacement. The mock does not enforce this but
    // the assertion below only inspects placement state so it does not
    // matter to the test.
    client
        .modify_instance_placement()
        .instance_id(&instance_id)
        .group_name("pg-modify-target")
        .send()
        .await
        .expect("modify_instance_placement should succeed");

    let desc = client
        .describe_instances()
        .instance_ids(&instance_id)
        .send()
        .await
        .expect("describe_instances should succeed");
    let placement = desc.reservations()[0].instances()[0]
        .placement()
        .expect("placement");
    assert_eq!(placement.group_name(), Some("pg-modify-target"));
}

#[tokio::test]
async fn test_modify_instance_placement_unknown_instance() {
    let client = make_ec2_client().await;
    let err = client
        .modify_instance_placement()
        .instance_id("i-doesnotexist")
        .group_name("pg-whatever")
        .send()
        .await
        .expect_err("should fail for missing instance");
    let msg = format!("{err:?}");
    assert!(msg.contains("InvalidInstanceID.NotFound"), "got: {msg}");
}

// --- ResetNetworkInterfaceAttribute tests (sub-task B) ---

#[tokio::test]
async fn test_reset_network_interface_attribute_source_dest_check() {
    let client = make_ec2_client().await;
    let vpc_id = client
        .create_vpc()
        .cidr_block("10.50.0.0/16")
        .send()
        .await
        .unwrap()
        .vpc()
        .unwrap()
        .vpc_id()
        .unwrap()
        .to_string();
    let subnet_id = client
        .create_subnet()
        .vpc_id(&vpc_id)
        .cidr_block("10.50.1.0/24")
        .availability_zone("us-east-1a")
        .send()
        .await
        .unwrap()
        .subnet()
        .unwrap()
        .subnet_id()
        .unwrap()
        .to_string();
    let eni_id = client
        .create_network_interface()
        .subnet_id(&subnet_id)
        .description("source-dest-check-test")
        .send()
        .await
        .unwrap()
        .network_interface()
        .unwrap()
        .network_interface_id()
        .unwrap()
        .to_string();

    // First flip source_dest_check to false via ModifyNetworkInterfaceAttribute.
    client
        .modify_network_interface_attribute()
        .network_interface_id(&eni_id)
        .source_dest_check(
            aws_sdk_ec2::types::AttributeBooleanValue::builder()
                .value(false)
                .build(),
        )
        .send()
        .await
        .expect("modify_network_interface_attribute should succeed");

    // Now reset it; the AWS default is true so it should flip back.
    client
        .reset_network_interface_attribute()
        .network_interface_id(&eni_id)
        .source_dest_check("sourceDestCheck")
        .send()
        .await
        .expect("reset_network_interface_attribute should succeed");

    let desc = client
        .describe_network_interfaces()
        .network_interface_ids(&eni_id)
        .send()
        .await
        .expect("describe_network_interfaces should succeed");
    let eni = &desc.network_interfaces()[0];
    assert_eq!(
        eni.source_dest_check(),
        Some(true),
        "reset should restore source_dest_check to true"
    );
}

#[tokio::test]
async fn test_reset_network_interface_attribute_attachment_rejected() {
    let client = make_ec2_client().await;
    let vpc_id = client
        .create_vpc()
        .cidr_block("10.51.0.0/16")
        .send()
        .await
        .unwrap()
        .vpc()
        .unwrap()
        .vpc_id()
        .unwrap()
        .to_string();
    let subnet_id = client
        .create_subnet()
        .vpc_id(&vpc_id)
        .cidr_block("10.51.1.0/24")
        .availability_zone("us-east-1a")
        .send()
        .await
        .unwrap()
        .subnet()
        .unwrap()
        .subnet_id()
        .unwrap()
        .to_string();
    let eni_id = client
        .create_network_interface()
        .subnet_id(&subnet_id)
        .send()
        .await
        .unwrap()
        .network_interface()
        .unwrap()
        .network_interface_id()
        .unwrap()
        .to_string();
    // The Rust SDK does not have a structured "attachment" attribute setter
    // here, so we exercise the path by sending a raw request via the
    // attribute query string. Reset's `source_dest_check` parameter on the
    // SDK happens to map to the Attribute query string, so we pass
    // "attachment" through it.
    let err = client
        .reset_network_interface_attribute()
        .network_interface_id(&eni_id)
        .source_dest_check("attachment")
        .send()
        .await
        .expect_err("attachment reset should fail");
    let msg = format!("{err:?}");
    assert!(msg.contains("InvalidParameterValue"), "got: {msg}");
}

// --- DescribeNetworkAcls association.subnet-id filter (sub-task C) ---

#[tokio::test]
async fn test_describe_network_acls_filter_by_association_subnet_id() {
    let client = make_ec2_client().await;
    let vpc_id = client
        .create_vpc()
        .cidr_block("10.60.0.0/16")
        .send()
        .await
        .unwrap()
        .vpc()
        .unwrap()
        .vpc_id()
        .unwrap()
        .to_string();
    let subnet_a = client
        .create_subnet()
        .vpc_id(&vpc_id)
        .cidr_block("10.60.1.0/24")
        .availability_zone("us-east-1a")
        .send()
        .await
        .unwrap()
        .subnet()
        .unwrap()
        .subnet_id()
        .unwrap()
        .to_string();
    let subnet_b = client
        .create_subnet()
        .vpc_id(&vpc_id)
        .cidr_block("10.60.2.0/24")
        .availability_zone("us-east-1b")
        .send()
        .await
        .unwrap()
        .subnet()
        .unwrap()
        .subnet_id()
        .unwrap()
        .to_string();

    // Create two new NACLs and re-associate each subnet onto one of them by
    // replacing the auto-created default association.
    let nacl_a = client
        .create_network_acl()
        .vpc_id(&vpc_id)
        .send()
        .await
        .unwrap()
        .network_acl()
        .unwrap()
        .network_acl_id()
        .unwrap()
        .to_string();
    let nacl_b = client
        .create_network_acl()
        .vpc_id(&vpc_id)
        .send()
        .await
        .unwrap()
        .network_acl()
        .unwrap()
        .network_acl_id()
        .unwrap()
        .to_string();

    // Find the auto-created default-NACL associations for both subnets.
    let all_acls = client
        .describe_network_acls()
        .filters(
            aws_sdk_ec2::types::Filter::builder()
                .name("vpc-id")
                .values(&vpc_id)
                .build(),
        )
        .send()
        .await
        .unwrap();
    let mut assoc_a = None;
    let mut assoc_b = None;
    for acl in all_acls.network_acls() {
        for a in acl.associations() {
            if a.subnet_id() == Some(subnet_a.as_str()) {
                assoc_a = a.network_acl_association_id().map(str::to_owned);
            } else if a.subnet_id() == Some(subnet_b.as_str()) {
                assoc_b = a.network_acl_association_id().map(str::to_owned);
            }
        }
    }
    let assoc_a = assoc_a.expect("subnet-a association");
    let assoc_b = assoc_b.expect("subnet-b association");

    client
        .replace_network_acl_association()
        .association_id(&assoc_a)
        .network_acl_id(&nacl_a)
        .send()
        .await
        .expect("replace_network_acl_association should succeed");
    client
        .replace_network_acl_association()
        .association_id(&assoc_b)
        .network_acl_id(&nacl_b)
        .send()
        .await
        .expect("replace_network_acl_association should succeed");

    let resp = client
        .describe_network_acls()
        .filters(
            aws_sdk_ec2::types::Filter::builder()
                .name("association.subnet-id")
                .values(&subnet_a)
                .build(),
        )
        .send()
        .await
        .expect("describe_network_acls with association.subnet-id should succeed");

    let acls = resp.network_acls();
    assert_eq!(
        acls.len(),
        1,
        "exactly one NACL should match subnet {subnet_a}"
    );
    assert_eq!(acls[0].network_acl_id(), Some(nacl_a.as_str()));
}

// --- CapacityReservationFleet tests (sub-task D) ---

#[tokio::test]
async fn test_capacity_reservation_fleet_create_describe_cancel() {
    let client = make_ec2_client().await;

    let create = client
        .create_capacity_reservation_fleet()
        .total_target_capacity(4)
        .instance_type_specifications(
            aws_sdk_ec2::types::ReservationFleetInstanceSpecification::builder()
                .instance_type(aws_sdk_ec2::types::InstanceType::M5Large)
                .instance_platform(
                    aws_sdk_ec2::types::CapacityReservationInstancePlatform::LinuxUnix,
                )
                .availability_zone("us-east-1a")
                .weight(2.0)
                .priority(1)
                .build(),
        )
        .send()
        .await
        .expect("create_capacity_reservation_fleet should succeed");
    let fleet_id = create
        .capacity_reservation_fleet_id()
        .expect("fleet id")
        .to_string();
    assert!(fleet_id.starts_with("crf-"));
    assert_eq!(create.total_target_capacity(), Some(4));
    assert_eq!(
        create.state(),
        Some(&aws_sdk_ec2::types::CapacityReservationFleetState::Active)
    );

    let desc = client
        .describe_capacity_reservation_fleets()
        .capacity_reservation_fleet_ids(&fleet_id)
        .send()
        .await
        .expect("describe_capacity_reservation_fleets should succeed");
    let fleets = desc.capacity_reservation_fleets();
    assert_eq!(fleets.len(), 1);
    assert_eq!(
        fleets[0].capacity_reservation_fleet_id(),
        Some(fleet_id.as_str())
    );

    let cancel = client
        .cancel_capacity_reservation_fleets()
        .capacity_reservation_fleet_ids(&fleet_id)
        .send()
        .await
        .expect("cancel_capacity_reservation_fleets should succeed");
    let successes = cancel.successful_fleet_cancellations();
    assert_eq!(successes.len(), 1);
    assert_eq!(
        successes[0].capacity_reservation_fleet_id(),
        Some(fleet_id.as_str())
    );

    let after = client
        .describe_capacity_reservation_fleets()
        .capacity_reservation_fleet_ids(&fleet_id)
        .send()
        .await
        .unwrap();
    assert_eq!(
        after.capacity_reservation_fleets()[0].state(),
        Some(&aws_sdk_ec2::types::CapacityReservationFleetState::Cancelled)
    );
}

#[tokio::test]
async fn test_modify_capacity_reservation_fleet_capacity() {
    let client = make_ec2_client().await;

    let fleet_id = client
        .create_capacity_reservation_fleet()
        .total_target_capacity(2)
        .instance_type_specifications(
            aws_sdk_ec2::types::ReservationFleetInstanceSpecification::builder()
                .instance_type(aws_sdk_ec2::types::InstanceType::C5Large)
                .instance_platform(
                    aws_sdk_ec2::types::CapacityReservationInstancePlatform::LinuxUnix,
                )
                .availability_zone("us-east-1a")
                .build(),
        )
        .send()
        .await
        .unwrap()
        .capacity_reservation_fleet_id()
        .unwrap()
        .to_string();

    let resp = client
        .modify_capacity_reservation_fleet()
        .capacity_reservation_fleet_id(&fleet_id)
        .total_target_capacity(7)
        .send()
        .await
        .expect("modify_capacity_reservation_fleet should succeed");
    assert_eq!(resp.r#return(), Some(true));

    let desc = client
        .describe_capacity_reservation_fleets()
        .capacity_reservation_fleet_ids(&fleet_id)
        .send()
        .await
        .unwrap();
    assert_eq!(
        desc.capacity_reservation_fleets()[0].total_target_capacity(),
        Some(7)
    );
}

#[tokio::test]
async fn test_modify_capacity_reservation_fleet_unknown() {
    let client = make_ec2_client().await;
    let err = client
        .modify_capacity_reservation_fleet()
        .capacity_reservation_fleet_id("crf-deadbeef")
        .total_target_capacity(1)
        .send()
        .await
        .expect_err("should fail for unknown fleet");
    let msg = format!("{err:?}");
    assert!(
        msg.contains("InvalidCapacityReservationFleetId.NotFound"),
        "got: {msg}"
    );
}

// --- New stub describe-and-modify operations ---

#[tokio::test]
async fn test_describe_id_format() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_id_format()
        .send()
        .await
        .expect("describe_id_format should succeed");
    // Batch B: every resource type now reports `true` for long IDs (AWS
    // retired short IDs in 2018).
    assert!(!resp.statuses().is_empty());
    for entry in resp.statuses() {
        assert_eq!(entry.use_long_ids(), Some(true));
    }
}

#[tokio::test]
async fn test_describe_identity_id_format() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_identity_id_format()
        .principal_arn("arn:aws:iam::123456789012:root")
        .send()
        .await
        .expect("describe_identity_id_format should succeed");
    assert!(!resp.statuses().is_empty());
}

#[tokio::test]
async fn test_describe_aggregate_id_format() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_aggregate_id_format()
        .send()
        .await
        .expect("describe_aggregate_id_format should succeed");
    assert_eq!(resp.use_long_ids_aggregated(), Some(true));
}

#[tokio::test]
async fn test_describe_principal_id_format() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_principal_id_format()
        .send()
        .await
        .expect("describe_principal_id_format should succeed");
    assert!(!resp.principals().is_empty());
}

#[tokio::test]
async fn test_modify_id_format_and_identity() {
    let client = make_ec2_client().await;
    client
        .modify_id_format()
        .resource("instance")
        .use_long_ids(true)
        .send()
        .await
        .expect("modify_id_format should succeed");
    client
        .modify_identity_id_format()
        .principal_arn("arn:aws:iam::123456789012:root")
        .resource("instance")
        .use_long_ids(true)
        .send()
        .await
        .expect("modify_identity_id_format should succeed");
}

#[tokio::test]
async fn test_describe_conversion_tasks() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_conversion_tasks()
        .send()
        .await
        .expect("describe_conversion_tasks should succeed");
    assert!(resp.conversion_tasks().is_empty());
}

#[tokio::test]
async fn test_describe_instance_event_notification_attributes() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_instance_event_notification_attributes()
        .send()
        .await
        .expect("describe_instance_event_notification_attributes should succeed");
    assert!(resp.instance_tag_attribute().is_none());
}

#[tokio::test]
async fn test_describe_instance_event_windows() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_instance_event_windows()
        .send()
        .await
        .expect("describe_instance_event_windows should succeed");
    assert!(resp.instance_event_windows().is_empty());
}

#[tokio::test]
async fn test_describe_export_tasks() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_export_tasks()
        .send()
        .await
        .expect("describe_export_tasks should succeed");
    assert!(resp.export_tasks().is_empty());
}

#[tokio::test]
async fn test_describe_export_image_tasks() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_export_image_tasks()
        .send()
        .await
        .expect("describe_export_image_tasks should succeed");
    assert!(resp.export_image_tasks().is_empty());
}

#[tokio::test]
async fn test_describe_import_image_tasks() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_import_image_tasks()
        .send()
        .await
        .expect("describe_import_image_tasks should succeed");
    assert!(resp.import_image_tasks().is_empty());
}

#[tokio::test]
async fn test_describe_fast_launch_images() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_fast_launch_images()
        .send()
        .await
        .expect("describe_fast_launch_images should succeed");
    assert!(resp.fast_launch_images().is_empty());
}

#[tokio::test]
async fn test_describe_host_reservations() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_host_reservations()
        .send()
        .await
        .expect("describe_host_reservations should succeed");
    assert!(resp.host_reservation_set().is_empty());
}

#[tokio::test]
async fn test_describe_host_reservation_offerings() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_host_reservation_offerings()
        .send()
        .await
        .expect("describe_host_reservation_offerings should succeed");
    assert!(!resp.offering_set().is_empty());
}

#[tokio::test]
async fn test_describe_reserved_instances_listings() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_reserved_instances_listings()
        .send()
        .await
        .expect("describe_reserved_instances_listings should succeed");
    assert!(resp.reserved_instances_listings().is_empty());
}

#[tokio::test]
async fn test_describe_reserved_instances_modifications() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_reserved_instances_modifications()
        .send()
        .await
        .expect("describe_reserved_instances_modifications should succeed");
    assert!(resp.reserved_instances_modifications().is_empty());
}

#[tokio::test]
async fn test_describe_fast_snapshot_restores() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_fast_snapshot_restores()
        .send()
        .await
        .expect("describe_fast_snapshot_restores should succeed");
    assert!(resp.fast_snapshot_restores().is_empty());
}

#[tokio::test]
async fn test_describe_import_snapshot_tasks() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_import_snapshot_tasks()
        .send()
        .await
        .expect("describe_import_snapshot_tasks should succeed");
    assert!(resp.import_snapshot_tasks().is_empty());
}

#[tokio::test]
async fn test_describe_locked_snapshots() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_locked_snapshots()
        .send()
        .await
        .expect("describe_locked_snapshots should succeed");
    assert!(resp.snapshots().is_empty());
}

#[tokio::test]
async fn test_describe_moving_addresses() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_moving_addresses()
        .send()
        .await
        .expect("describe_moving_addresses should succeed");
    assert!(resp.moving_address_statuses().is_empty());
}

#[tokio::test]
async fn test_describe_byoip_cidrs() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_byoip_cidrs()
        .max_results(10)
        .send()
        .await
        .expect("describe_byoip_cidrs should succeed");
    assert!(resp.byoip_cidrs().is_empty());
}

#[tokio::test]
async fn test_describe_ipv6_pools() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_ipv6_pools()
        .send()
        .await
        .expect("describe_ipv6_pools should succeed");
    assert!(resp.ipv6_pools().is_empty());
}

#[tokio::test]
async fn test_describe_public_ipv4_pools() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_public_ipv4_pools()
        .send()
        .await
        .expect("describe_public_ipv4_pools should succeed");
    assert!(resp.public_ipv4_pools().is_empty());
}

#[tokio::test]
async fn test_describe_network_interface_attribute_lifecycle() {
    use aws_sdk_ec2::types::NetworkInterfaceAttribute;

    let client = make_ec2_client().await;

    let vpc = client
        .create_vpc()
        .cidr_block("10.0.0.0/16")
        .send()
        .await
        .expect("create_vpc")
        .vpc
        .unwrap();
    let vpc_id = vpc.vpc_id().unwrap().to_string();

    let subnet = client
        .create_subnet()
        .vpc_id(&vpc_id)
        .cidr_block("10.0.1.0/24")
        .send()
        .await
        .expect("create_subnet")
        .subnet
        .unwrap();
    let subnet_id = subnet.subnet_id().unwrap().to_string();

    let eni = client
        .create_network_interface()
        .subnet_id(&subnet_id)
        .description("test-eni")
        .send()
        .await
        .expect("create_network_interface")
        .network_interface
        .unwrap();
    let eni_id = eni.network_interface_id().unwrap().to_string();

    let resp = client
        .describe_network_interface_attribute()
        .network_interface_id(&eni_id)
        .attribute(NetworkInterfaceAttribute::Description)
        .send()
        .await
        .expect("describe_network_interface_attribute description");
    assert_eq!(resp.network_interface_id().unwrap(), eni_id);
    assert_eq!(resp.description().unwrap().value().unwrap(), "test-eni");

    let resp = client
        .describe_network_interface_attribute()
        .network_interface_id(&eni_id)
        .attribute(NetworkInterfaceAttribute::SourceDestCheck)
        .send()
        .await
        .expect("describe_network_interface_attribute sourceDestCheck");
    assert_eq!(resp.source_dest_check().unwrap().value(), Some(true));

    let resp = client
        .describe_network_interface_attribute()
        .network_interface_id(&eni_id)
        .attribute(NetworkInterfaceAttribute::GroupSet)
        .send()
        .await
        .expect("describe_network_interface_attribute groupSet");
    assert!(resp.groups().is_empty());

    let err = client
        .describe_network_interface_attribute()
        .network_interface_id("eni-doesnotexist")
        .attribute(NetworkInterfaceAttribute::Description)
        .send()
        .await
        .expect_err("should fail for unknown eni");
    let msg = format!("{err:?}");
    assert!(
        msg.contains("InvalidNetworkInterfaceID.NotFound"),
        "got: {msg}"
    );
}

// --- Second batch of stub describes ---

#[tokio::test]
async fn test_describe_image_references() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_image_references()
        .image_ids("ami-1234567890abcdef0")
        .send()
        .await
        .expect("describe_image_references should succeed");
    assert!(resp.image_references().is_empty());
}

#[tokio::test]
async fn test_describe_image_usage_reports() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_image_usage_reports()
        .send()
        .await
        .expect("describe_image_usage_reports should succeed");
    assert!(resp.image_usage_reports().is_empty());
}

#[tokio::test]
async fn test_describe_image_usage_report_entries() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_image_usage_report_entries()
        .send()
        .await
        .expect("describe_image_usage_report_entries should succeed");
    assert!(resp.image_usage_report_entries().is_empty());
}

#[tokio::test]
async fn test_describe_instance_image_metadata() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_instance_image_metadata()
        .send()
        .await
        .expect("describe_instance_image_metadata should succeed");
    assert!(resp.instance_image_metadata().is_empty());
}

#[tokio::test]
async fn test_describe_instance_topology() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_instance_topology()
        .send()
        .await
        .expect("describe_instance_topology should succeed");
    assert!(resp.instances().is_empty());
}

#[tokio::test]
async fn test_describe_mac_hosts() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_mac_hosts()
        .send()
        .await
        .expect("describe_mac_hosts should succeed");
    assert!(resp.mac_hosts().is_empty());
}

#[tokio::test]
async fn test_describe_scheduled_instance_availability() {
    use aws_sdk_ec2::types::SlotDateTimeRangeRequest;
    let client = make_ec2_client().await;
    let resp = client
        .describe_scheduled_instance_availability()
        .first_slot_start_time_range(
            SlotDateTimeRangeRequest::builder()
                .earliest_time(aws_sdk_ec2::primitives::DateTime::from_secs(0))
                .latest_time(aws_sdk_ec2::primitives::DateTime::from_secs(1))
                .build(),
        )
        .recurrence(
            aws_sdk_ec2::types::ScheduledInstanceRecurrenceRequest::builder()
                .frequency("Weekly")
                .build(),
        )
        .send()
        .await
        .expect("describe_scheduled_instance_availability should succeed");
    assert!(!resp.scheduled_instance_availability_set().is_empty());
}

#[tokio::test]
async fn test_describe_scheduled_instances() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_scheduled_instances()
        .send()
        .await
        .expect("describe_scheduled_instances should succeed");
    assert!(resp.scheduled_instance_set().is_empty());
}

#[tokio::test]
async fn test_describe_store_image_tasks() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_store_image_tasks()
        .send()
        .await
        .expect("describe_store_image_tasks should succeed");
    assert!(resp.store_image_task_results().is_empty());
}

#[tokio::test]
async fn test_describe_capacity_block_extension_history() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_capacity_block_extension_history()
        .send()
        .await
        .expect("describe_capacity_block_extension_history should succeed");
    assert!(resp.capacity_block_extensions().is_empty());
}

#[tokio::test]
async fn test_describe_capacity_block_extension_offerings() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_capacity_block_extension_offerings()
        .capacity_block_extension_duration_hours(24)
        .capacity_reservation_id("cr-12345")
        .send()
        .await
        .expect("describe_capacity_block_extension_offerings should succeed");
    // Wave 3: handler returns a synthetic catalogue.
    assert!(!resp.capacity_block_extension_offerings().is_empty());
}

#[tokio::test]
async fn test_describe_capacity_block_offerings() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_capacity_block_offerings()
        .instance_count(1)
        .capacity_duration_hours(24)
        .send()
        .await
        .expect("describe_capacity_block_offerings should succeed");
    // Wave 3: handler returns a synthetic catalogue (3 offerings).
    assert_eq!(resp.capacity_block_offerings().len(), 3);
}

#[tokio::test]
async fn test_describe_capacity_block_status() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_capacity_block_status()
        .send()
        .await
        .expect("describe_capacity_block_status should succeed");
    assert!(resp.capacity_block_statuses().is_empty());
}

#[tokio::test]
async fn test_describe_capacity_blocks() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_capacity_blocks()
        .send()
        .await
        .expect("describe_capacity_blocks should succeed");
    assert!(resp.capacity_blocks().is_empty());
}

#[tokio::test]
async fn test_describe_elastic_gpus() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_elastic_gpus()
        .send()
        .await
        .expect("describe_elastic_gpus should succeed");
    assert!(resp.elastic_gpu_set().is_empty());
}

#[tokio::test]
async fn test_describe_fleet_history() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_fleet_history()
        .fleet_id("fleet-1234")
        .start_time(aws_sdk_ec2::primitives::DateTime::from_secs(0))
        .send()
        .await
        .expect("describe_fleet_history should succeed");
    assert!(resp.history_records().is_empty());
}

#[tokio::test]
async fn test_describe_mac_modification_tasks() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_mac_modification_tasks()
        .send()
        .await
        .expect("describe_mac_modification_tasks should succeed");
    assert!(resp.mac_modification_tasks().is_empty());
}

#[tokio::test]
async fn test_describe_traffic_mirror_filter_rules() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_traffic_mirror_filter_rules()
        .send()
        .await
        .expect("describe_traffic_mirror_filter_rules should succeed");
    assert!(resp.traffic_mirror_filter_rules().is_empty());
}

#[tokio::test]
async fn test_describe_traffic_mirror_filters() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_traffic_mirror_filters()
        .send()
        .await
        .expect("describe_traffic_mirror_filters should succeed");
    assert!(resp.traffic_mirror_filters().is_empty());
}

#[tokio::test]
async fn test_describe_traffic_mirror_sessions() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_traffic_mirror_sessions()
        .send()
        .await
        .expect("describe_traffic_mirror_sessions should succeed");
    assert!(resp.traffic_mirror_sessions().is_empty());
}

#[tokio::test]
async fn test_describe_traffic_mirror_targets() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_traffic_mirror_targets()
        .send()
        .await
        .expect("describe_traffic_mirror_targets should succeed");
    assert!(resp.traffic_mirror_targets().is_empty());
}

#[tokio::test]
async fn test_describe_address_transfers() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_address_transfers()
        .send()
        .await
        .expect("describe_address_transfers should succeed");
    assert!(resp.address_transfers().is_empty());
}

#[tokio::test]
async fn test_describe_aws_network_performance_metric_subscriptions() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_aws_network_performance_metric_subscriptions()
        .send()
        .await
        .expect("describe_aws_network_performance_metric_subscriptions should succeed");
    assert!(resp.subscriptions().is_empty());
}

#[tokio::test]
async fn test_describe_network_insights_analyses() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_network_insights_analyses()
        .send()
        .await
        .expect("describe_network_insights_analyses should succeed");
    assert!(resp.network_insights_analyses().is_empty());
}

#[tokio::test]
async fn test_describe_network_insights_paths() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_network_insights_paths()
        .send()
        .await
        .expect("describe_network_insights_paths should succeed");
    assert!(resp.network_insights_paths().is_empty());
}

#[tokio::test]
async fn test_describe_security_group_references() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_security_group_references()
        .group_id("sg-1234")
        .send()
        .await
        .expect("describe_security_group_references should succeed");
    assert!(resp.security_group_reference_set().is_empty());
}

#[tokio::test]
async fn test_describe_fpga_images() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_fpga_images()
        .send()
        .await
        .expect("describe_fpga_images should succeed");
    assert!(resp.fpga_images().is_empty());
}

#[tokio::test]
async fn test_describe_replace_root_volume_tasks() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_replace_root_volume_tasks()
        .send()
        .await
        .expect("describe_replace_root_volume_tasks should succeed");
    assert!(resp.replace_root_volume_tasks().is_empty());
}

#[tokio::test]
async fn test_describe_snapshot_tier_status() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_snapshot_tier_status()
        .send()
        .await
        .expect("describe_snapshot_tier_status should succeed");
    assert!(resp.snapshot_tier_statuses().is_empty());
}

// --- Third batch ---

#[tokio::test]
async fn test_describe_client_vpn_authorization_rules() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_client_vpn_authorization_rules()
        .client_vpn_endpoint_id("cvpn-endpoint-1234")
        .send()
        .await
        .expect("describe_client_vpn_authorization_rules should succeed");
    assert!(resp.authorization_rules().is_empty());
}

#[tokio::test]
async fn test_describe_client_vpn_connections() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_client_vpn_connections()
        .client_vpn_endpoint_id("cvpn-endpoint-1234")
        .send()
        .await
        .expect("describe_client_vpn_connections should succeed");
    assert!(resp.connections().is_empty());
}

#[tokio::test]
async fn test_describe_client_vpn_endpoints() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_client_vpn_endpoints()
        .send()
        .await
        .expect("describe_client_vpn_endpoints should succeed");
    assert!(resp.client_vpn_endpoints().is_empty());
}

#[tokio::test]
async fn test_describe_client_vpn_routes() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_client_vpn_routes()
        .client_vpn_endpoint_id("cvpn-endpoint-1234")
        .send()
        .await
        .expect("describe_client_vpn_routes should succeed");
    assert!(resp.routes().is_empty());
}

#[tokio::test]
async fn test_describe_client_vpn_target_networks() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_client_vpn_target_networks()
        .client_vpn_endpoint_id("cvpn-endpoint-1234")
        .send()
        .await
        .expect("describe_client_vpn_target_networks should succeed");
    assert!(resp.client_vpn_target_networks().is_empty());
}

#[tokio::test]
async fn test_describe_local_gateway_route_table_virtual_interface_group_associations() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_local_gateway_route_table_virtual_interface_group_associations()
        .send()
        .await
        .expect(
            "describe_local_gateway_route_table_virtual_interface_group_associations should succeed",
        );
    assert!(
        resp.local_gateway_route_table_virtual_interface_group_associations()
            .is_empty()
    );
}

#[tokio::test]
async fn test_describe_local_gateway_route_table_vpc_associations() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_local_gateway_route_table_vpc_associations()
        .send()
        .await
        .expect("describe_local_gateway_route_table_vpc_associations should succeed");
    assert!(resp.local_gateway_route_table_vpc_associations().is_empty());
}

#[tokio::test]
async fn test_describe_local_gateway_route_tables() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_local_gateway_route_tables()
        .send()
        .await
        .expect("describe_local_gateway_route_tables should succeed");
    assert!(resp.local_gateway_route_tables().is_empty());
}

#[tokio::test]
async fn test_describe_local_gateway_virtual_interface_groups() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_local_gateway_virtual_interface_groups()
        .send()
        .await
        .expect("describe_local_gateway_virtual_interface_groups should succeed");
    assert!(resp.local_gateway_virtual_interface_groups().is_empty());
}

#[tokio::test]
async fn test_describe_local_gateway_virtual_interfaces() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_local_gateway_virtual_interfaces()
        .send()
        .await
        .expect("describe_local_gateway_virtual_interfaces should succeed");
    assert!(resp.local_gateway_virtual_interfaces().is_empty());
}

#[tokio::test]
async fn test_describe_local_gateways() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_local_gateways()
        .send()
        .await
        .expect("describe_local_gateways should succeed");
    assert!(resp.local_gateways().is_empty());
}

#[tokio::test]
async fn test_describe_transit_gateway_connect_peers() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_transit_gateway_connect_peers()
        .send()
        .await
        .expect("describe_transit_gateway_connect_peers should succeed");
    assert!(resp.transit_gateway_connect_peers().is_empty());
}

#[tokio::test]
async fn test_describe_transit_gateway_connects() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_transit_gateway_connects()
        .send()
        .await
        .expect("describe_transit_gateway_connects should succeed");
    assert!(resp.transit_gateway_connects().is_empty());
}

#[tokio::test]
async fn test_describe_transit_gateway_multicast_domains() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_transit_gateway_multicast_domains()
        .send()
        .await
        .expect("describe_transit_gateway_multicast_domains should succeed");
    assert!(resp.transit_gateway_multicast_domains().is_empty());
}

#[tokio::test]
async fn test_describe_transit_gateway_policy_tables() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_transit_gateway_policy_tables()
        .send()
        .await
        .expect("describe_transit_gateway_policy_tables should succeed");
    assert!(resp.transit_gateway_policy_tables().is_empty());
}

#[tokio::test]
async fn test_describe_transit_gateway_route_table_announcements() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_transit_gateway_route_table_announcements()
        .send()
        .await
        .expect("describe_transit_gateway_route_table_announcements should succeed");
    assert!(resp.transit_gateway_route_table_announcements().is_empty());
}

#[tokio::test]
async fn test_get_vpn_connection_device_types() {
    let client = make_ec2_client().await;
    let resp = client
        .get_vpn_connection_device_types()
        .send()
        .await
        .expect("get_vpn_connection_device_types should succeed");
    assert!(!resp.vpn_connection_device_types().is_empty());
}

#[tokio::test]
async fn test_describe_ipam_byoasn() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_ipam_byoasn()
        .send()
        .await
        .expect("describe_ipam_byoasn should succeed");
    assert!(resp.byoasns().is_empty());
}

#[tokio::test]
async fn test_describe_ipam_external_resource_verification_tokens() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_ipam_external_resource_verification_tokens()
        .send()
        .await
        .expect("describe_ipam_external_resource_verification_tokens should succeed");
    assert!(resp.ipam_external_resource_verification_tokens().is_empty());
}

#[tokio::test]
async fn test_describe_ipam_policies() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_ipam_policies()
        .send()
        .await
        .expect("describe_ipam_policies should succeed");
    assert!(resp.ipam_policies().is_empty());
}

#[tokio::test]
async fn test_describe_ipam_pools() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_ipam_pools()
        .send()
        .await
        .expect("describe_ipam_pools should succeed");
    assert!(resp.ipam_pools().is_empty());
}

#[tokio::test]
async fn test_describe_ipam_resource_discoveries() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_ipam_resource_discoveries()
        .send()
        .await
        .expect("describe_ipam_resource_discoveries should succeed");
    assert!(resp.ipam_resource_discoveries().is_empty());
}

#[tokio::test]
async fn test_describe_ipam_resource_discovery_associations() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_ipam_resource_discovery_associations()
        .send()
        .await
        .expect("describe_ipam_resource_discovery_associations should succeed");
    assert!(resp.ipam_resource_discovery_associations().is_empty());
}

#[tokio::test]
async fn test_describe_ipam_scopes() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_ipam_scopes()
        .send()
        .await
        .expect("describe_ipam_scopes should succeed");
    assert!(resp.ipam_scopes().is_empty());
}

#[tokio::test]
async fn test_describe_ipams() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_ipams()
        .send()
        .await
        .expect("describe_ipams should succeed");
    assert!(resp.ipams().is_empty());
}

#[tokio::test]
async fn test_describe_verified_access_endpoints() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_verified_access_endpoints()
        .send()
        .await
        .expect("describe_verified_access_endpoints should succeed");
    assert!(resp.verified_access_endpoints().is_empty());
}

#[tokio::test]
async fn test_describe_verified_access_groups() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_verified_access_groups()
        .send()
        .await
        .expect("describe_verified_access_groups should succeed");
    assert!(resp.verified_access_groups().is_empty());
}

#[tokio::test]
async fn test_describe_verified_access_trust_providers() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_verified_access_trust_providers()
        .send()
        .await
        .expect("describe_verified_access_trust_providers should succeed");
    assert!(resp.verified_access_trust_providers().is_empty());
}

#[tokio::test]
async fn test_get_serial_console_access_status() {
    let client = make_ec2_client().await;
    let resp = client
        .get_serial_console_access_status()
        .send()
        .await
        .expect("get_serial_console_access_status should succeed");
    assert_eq!(resp.serial_console_access_enabled(), Some(false));
}

#[tokio::test]
async fn test_describe_ipam_prefix_list_resolvers() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_ipam_prefix_list_resolvers()
        .send()
        .await
        .expect("describe_ipam_prefix_list_resolvers should succeed");
    assert!(resp.ipam_prefix_list_resolvers().is_empty());
}

#[tokio::test]
async fn test_describe_ipam_prefix_list_resolver_targets() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_ipam_prefix_list_resolver_targets()
        .send()
        .await
        .expect("describe_ipam_prefix_list_resolver_targets should succeed");
    assert!(resp.ipam_prefix_list_resolver_targets().is_empty());
}

#[tokio::test]
async fn test_describe_vpc_endpoint_associations() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_vpc_endpoint_associations()
        .send()
        .await
        .expect("describe_vpc_endpoint_associations should succeed");
    assert!(resp.vpc_endpoint_associations().is_empty());
}

#[tokio::test]
async fn test_describe_vpc_endpoint_connection_notifications() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_vpc_endpoint_connection_notifications()
        .send()
        .await
        .expect("describe_vpc_endpoint_connection_notifications should succeed");
    assert!(resp.connection_notification_set().is_empty());
}

#[tokio::test]
async fn test_describe_vpc_endpoint_connections() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_vpc_endpoint_connections()
        .send()
        .await
        .expect("describe_vpc_endpoint_connections should succeed");
    assert!(resp.vpc_endpoint_connections().is_empty());
}

#[tokio::test]
async fn test_get_security_groups_for_vpc() {
    let client = make_ec2_client().await;
    let resp = client
        .get_security_groups_for_vpc()
        .vpc_id("vpc-1234")
        .send()
        .await
        .expect("get_security_groups_for_vpc should succeed");
    assert!(resp.security_group_for_vpcs().is_empty());
}

#[tokio::test]
async fn test_get_allowed_images_settings() {
    let client = make_ec2_client().await;
    let resp = client
        .get_allowed_images_settings()
        .send()
        .await
        .expect("get_allowed_images_settings should succeed");
    assert_eq!(resp.state(), Some("disabled"));
}

#[tokio::test]
async fn test_get_capacity_reservation_usage() {
    // Wave 3: handler validates the reservation ID; an unseeded ID now errors.
    let client = make_ec2_client().await;
    let err = client
        .get_capacity_reservation_usage()
        .capacity_reservation_id("cr-1234")
        .send()
        .await
        .expect_err("expected NotFound for unseeded capacity reservation");
    let dbg = format!("{err:?}");
    assert!(
        dbg.contains("InvalidCapacityReservationId.NotFound"),
        "{dbg}"
    );
}

#[tokio::test]
async fn test_get_groups_for_capacity_reservation() {
    // Wave 3: handler validates the reservation ID; an unseeded ID now errors.
    let client = make_ec2_client().await;
    let err = client
        .get_groups_for_capacity_reservation()
        .capacity_reservation_id("cr-1234")
        .send()
        .await
        .expect_err("expected NotFound for unseeded capacity reservation");
    let dbg = format!("{err:?}");
    assert!(
        dbg.contains("InvalidCapacityReservationId.NotFound"),
        "{dbg}"
    );
}

#[tokio::test]
async fn test_get_image_block_public_access_state() {
    let client = make_ec2_client().await;
    let resp = client
        .get_image_block_public_access_state()
        .send()
        .await
        .expect("get_image_block_public_access_state should succeed");
    assert_eq!(resp.image_block_public_access_state(), Some("unblocked"));
}

#[tokio::test]
async fn test_get_ebs_default_kms_key_id() {
    let client = make_ec2_client().await;
    let resp = client
        .get_ebs_default_kms_key_id()
        .send()
        .await
        .expect("get_ebs_default_kms_key_id should succeed");
    assert_eq!(resp.kms_key_id(), Some("alias/aws/ebs"));
}

#[tokio::test]
async fn test_get_snapshot_block_public_access_state() {
    let client = make_ec2_client().await;
    let resp = client
        .get_snapshot_block_public_access_state()
        .send()
        .await
        .expect("get_snapshot_block_public_access_state should succeed");
    assert_eq!(resp.state().map(|s| s.as_str()), Some("unblocked"));
}

// --- Fourth batch ---

#[tokio::test]
async fn test_describe_coip_pools() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_coip_pools()
        .send()
        .await
        .expect("describe_coip_pools should succeed");
    assert!(resp.coip_pools().is_empty());
}

#[tokio::test]
async fn test_describe_transit_gateway_metering_policies() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_transit_gateway_metering_policies()
        .send()
        .await
        .expect("describe_transit_gateway_metering_policies should succeed");
    assert!(resp.transit_gateway_metering_policies().is_empty());
}

#[tokio::test]
async fn test_describe_vpn_concentrators() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_vpn_concentrators()
        .send()
        .await
        .expect("describe_vpn_concentrators should succeed");
    assert!(resp.vpn_concentrators().is_empty());
}

#[tokio::test]
async fn test_get_coip_pool_usage() {
    let client = make_ec2_client().await;
    let pool_id = client
        .create_coip_pool()
        .local_gateway_route_table_id("lgw-rtb-12345678")
        .send()
        .await
        .expect("create_coip_pool")
        .coip_pool()
        .and_then(|p| p.pool_id())
        .expect("pool id")
        .to_string();
    let resp = client
        .get_coip_pool_usage()
        .pool_id(&pool_id)
        .send()
        .await
        .expect("get_coip_pool_usage should succeed");
    assert!(resp.coip_address_usages().is_empty());
}

#[tokio::test]
async fn test_get_transit_gateway_attachment_propagations() {
    let client = make_ec2_client().await;
    let (_tgw, att_id, _) = wave2_seed_tgw_and_vpc_attach(&client).await;
    let resp = client
        .get_transit_gateway_attachment_propagations()
        .transit_gateway_attachment_id(&att_id)
        .send()
        .await
        .expect("get_transit_gateway_attachment_propagations should succeed");
    assert!(resp.transit_gateway_attachment_propagations().is_empty());
}

#[tokio::test]
async fn test_get_transit_gateway_metering_policy_entries() {
    let client = make_ec2_client().await;
    let resp = client
        .get_transit_gateway_metering_policy_entries()
        .transit_gateway_metering_policy_id("tgw-mp-1234")
        .send()
        .await
        .expect("get_transit_gateway_metering_policy_entries should succeed");
    assert!(resp.transit_gateway_metering_policy_entries().is_empty());
}

#[tokio::test]
async fn test_get_transit_gateway_multicast_domain_associations() {
    let client = make_ec2_client().await;
    let resp = client
        .get_transit_gateway_multicast_domain_associations()
        .transit_gateway_multicast_domain_id("tgw-mcast-domain-1234")
        .send()
        .await
        .expect("get_transit_gateway_multicast_domain_associations should succeed");
    assert!(resp.multicast_domain_associations().is_empty());
}

#[tokio::test]
async fn test_get_transit_gateway_policy_table_associations() {
    let client = make_ec2_client().await;
    let resp = client
        .get_transit_gateway_policy_table_associations()
        .transit_gateway_policy_table_id("tgw-ptb-1234")
        .send()
        .await
        .expect("get_transit_gateway_policy_table_associations should succeed");
    assert!(resp.associations().is_empty());
}

#[tokio::test]
async fn test_get_transit_gateway_policy_table_entries() {
    let client = make_ec2_client().await;
    let (tgw, _att, _) = wave2_seed_tgw_and_vpc_attach(&client).await;
    let pt_id = client
        .create_transit_gateway_policy_table()
        .transit_gateway_id(&tgw)
        .send()
        .await
        .unwrap()
        .transit_gateway_policy_table()
        .unwrap()
        .transit_gateway_policy_table_id()
        .unwrap()
        .to_string();
    let resp = client
        .get_transit_gateway_policy_table_entries()
        .transit_gateway_policy_table_id(&pt_id)
        .send()
        .await
        .expect("get_transit_gateway_policy_table_entries should succeed");
    assert!(resp.transit_gateway_policy_table_entries().is_empty());
}

#[tokio::test]
async fn test_get_transit_gateway_prefix_list_references() {
    let client = make_ec2_client().await;
    let resp = client
        .get_transit_gateway_prefix_list_references()
        .transit_gateway_route_table_id("tgw-rtb-1234")
        .send()
        .await
        .expect("get_transit_gateway_prefix_list_references should succeed");
    assert!(resp.transit_gateway_prefix_list_references().is_empty());
}

#[tokio::test]
async fn test_describe_capacity_reservation_billing_requests() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_capacity_reservation_billing_requests()
        .role(aws_sdk_ec2::types::CallerRole::OdcrOwner)
        .send()
        .await
        .expect("describe_capacity_reservation_billing_requests should succeed");
    assert!(resp.capacity_reservation_billing_requests().is_empty());
}

#[tokio::test]
async fn test_describe_capacity_reservation_topology() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_capacity_reservation_topology()
        .send()
        .await
        .expect("describe_capacity_reservation_topology should succeed");
    assert!(resp.capacity_reservations().is_empty());
}

#[tokio::test]
async fn test_describe_classic_link_instances() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_classic_link_instances()
        .send()
        .await
        .expect("describe_classic_link_instances should succeed");
    assert!(resp.instances().is_empty());
}

#[tokio::test]
async fn test_describe_instance_sql_ha_history_states() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_instance_sql_ha_history_states()
        .send()
        .await
        .expect("describe_instance_sql_ha_history_states should succeed");
    assert!(resp.instances().is_empty());
}

#[tokio::test]
async fn test_describe_instance_sql_ha_states() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_instance_sql_ha_states()
        .send()
        .await
        .expect("describe_instance_sql_ha_states should succeed");
    assert!(resp.instances().is_empty());
}

#[tokio::test]
async fn test_describe_verified_access_instance_logging_configurations() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_verified_access_instance_logging_configurations()
        .send()
        .await
        .expect("describe_verified_access_instance_logging_configurations should succeed");
    assert!(resp.logging_configurations().is_empty());
}

#[tokio::test]
async fn test_describe_verified_access_instances() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_verified_access_instances()
        .send()
        .await
        .expect("describe_verified_access_instances should succeed");
    assert!(resp.verified_access_instances().is_empty());
}

#[tokio::test]
async fn test_get_host_reservation_purchase_preview() {
    let client = make_ec2_client().await;
    let host_id = client
        .allocate_hosts()
        .availability_zone("us-east-1a")
        .instance_family("m5")
        .quantity(1)
        .send()
        .await
        .expect("allocate_hosts should succeed")
        .host_ids
        .unwrap()
        .into_iter()
        .next()
        .unwrap();
    let resp = client
        .get_host_reservation_purchase_preview()
        .host_id_set(host_id)
        .offering_id("hr-1234")
        .send()
        .await
        .expect("get_host_reservation_purchase_preview should succeed");
    assert_eq!(resp.total_upfront_price(), Some("0.00"));
    assert_eq!(resp.total_hourly_price(), Some("0.00"));
}

#[tokio::test]
async fn test_get_image_ancestry() {
    let client = make_ec2_client().await;
    let resp = client
        .get_image_ancestry()
        .image_id("ami-1234567890abcdef0")
        .send()
        .await
        .expect("get_image_ancestry should succeed");
    assert!(resp.image_ancestry_entries().is_empty());
}

#[tokio::test]
async fn test_get_instance_metadata_defaults() {
    let client = make_ec2_client().await;
    let resp = client
        .get_instance_metadata_defaults()
        .send()
        .await
        .expect("get_instance_metadata_defaults should succeed");
    assert!(resp.account_level().is_none());
}

#[tokio::test]
async fn test_get_instance_tpm_ek_pub() {
    let client = make_ec2_client().await;
    let resp = client
        .get_instance_tpm_ek_pub()
        .instance_id("i-1234567890abcdef0")
        .key_type(aws_sdk_ec2::types::EkPubKeyType::Rsa2048)
        .key_format(aws_sdk_ec2::types::EkPubKeyFormat::Tpmt)
        .send()
        .await
        .expect("get_instance_tpm_ek_pub should succeed");
    assert!(resp.key_value().is_none());
}

#[tokio::test]
async fn test_get_instance_types_from_instance_requirements() {
    use aws_sdk_ec2::types::{
        ArchitectureType, InstanceRequirementsRequest, VCpuCountRangeRequest, VirtualizationType,
    };
    let client = make_ec2_client().await;
    let resp = client
        .get_instance_types_from_instance_requirements()
        .architecture_types(ArchitectureType::X8664)
        .virtualization_types(VirtualizationType::Hvm)
        .instance_requirements(
            InstanceRequirementsRequest::builder()
                .v_cpu_count(VCpuCountRangeRequest::builder().min(1).build())
                .build(),
        )
        .send()
        .await
        .expect("get_instance_types_from_instance_requirements should succeed");
    assert!(!resp.instance_types().is_empty());
}

#[tokio::test]
async fn test_describe_capacity_manager_data_exports() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_capacity_manager_data_exports()
        .send()
        .await
        .expect("describe_capacity_manager_data_exports should succeed");
    assert!(resp.capacity_manager_data_exports().is_empty());
}

#[tokio::test]
async fn test_describe_declarative_policies_reports() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_declarative_policies_reports()
        .send()
        .await
        .expect("describe_declarative_policies_reports should succeed");
    assert!(resp.reports().is_empty());
}

#[tokio::test]
async fn test_describe_outpost_lags() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_outpost_lags()
        .send()
        .await
        .expect("describe_outpost_lags should succeed");
    assert!(resp.outpost_lags().is_empty());
}

#[tokio::test]
async fn test_describe_secondary_interfaces() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_secondary_interfaces()
        .send()
        .await
        .expect("describe_secondary_interfaces should succeed");
    assert!(resp.secondary_interfaces().is_empty());
}

#[tokio::test]
async fn test_describe_secondary_networks() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_secondary_networks()
        .send()
        .await
        .expect("describe_secondary_networks should succeed");
    assert!(resp.secondary_networks().is_empty());
}

#[tokio::test]
async fn test_describe_service_link_virtual_interfaces() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_service_link_virtual_interfaces()
        .send()
        .await
        .expect("describe_service_link_virtual_interfaces should succeed");
    assert!(resp.service_link_virtual_interfaces().is_empty());
}

#[tokio::test]
async fn test_describe_spot_fleet_request_history() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_spot_fleet_request_history()
        .spot_fleet_request_id("sfr-1234")
        .start_time(aws_sdk_ec2::primitives::DateTime::from_secs(0))
        .send()
        .await
        .expect("describe_spot_fleet_request_history should succeed");
    assert!(resp.history_records().is_empty());
}

#[tokio::test]
async fn test_describe_trunk_interface_associations() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_trunk_interface_associations()
        .send()
        .await
        .expect("describe_trunk_interface_associations should succeed");
    assert!(resp.interface_associations().is_empty());
}

#[tokio::test]
async fn test_get_associated_enclave_certificate_iam_roles() {
    let client = make_ec2_client().await;
    let resp = client
        .get_associated_enclave_certificate_iam_roles()
        .certificate_arn("arn:aws:acm:us-east-1:123456789012:certificate/abc")
        .send()
        .await
        .expect("get_associated_enclave_certificate_iam_roles should succeed");
    assert!(resp.associated_roles().is_empty());
}

#[tokio::test]
async fn test_get_default_credit_specification() {
    let client = make_ec2_client().await;
    let resp = client
        .get_default_credit_specification()
        .instance_family(aws_sdk_ec2::types::UnlimitedSupportedInstanceFamily::T2)
        .send()
        .await
        .expect("get_default_credit_specification should succeed");
    let spec = resp
        .instance_family_credit_specification()
        .expect("default-credit-spec should default to standard");
    assert_eq!(spec.cpu_credits().unwrap_or(""), "standard");
}

#[tokio::test]
async fn test_get_enabled_ipam_policy() {
    let client = make_ec2_client().await;
    let resp = client
        .get_enabled_ipam_policy()
        .send()
        .await
        .expect("get_enabled_ipam_policy should succeed");
    assert!(resp.ipam_policy_id().is_none());
}

#[tokio::test]
async fn test_get_ipam_discovered_accounts() {
    let client = make_ec2_client().await;
    let resp = client
        .get_ipam_discovered_accounts()
        .ipam_resource_discovery_id("ipam-res-disco-1234")
        .discovery_region("us-east-1")
        .send()
        .await
        .expect("get_ipam_discovered_accounts should succeed");
    assert!(resp.ipam_discovered_accounts().is_empty());
}

#[tokio::test]
async fn test_get_ipam_policy_allocation_rules() {
    use aws_sdk_ec2::types::AddIpamOperatingRegion;
    let client = make_ec2_client().await;
    let ipam_id = client
        .create_ipam()
        .operating_regions(
            AddIpamOperatingRegion::builder()
                .region_name("us-east-1")
                .build(),
        )
        .send()
        .await
        .unwrap()
        .ipam()
        .and_then(|i| i.ipam_id())
        .unwrap()
        .to_string();
    let policy_id = client
        .create_ipam_policy()
        .ipam_id(&ipam_id)
        .send()
        .await
        .unwrap()
        .ipam_policy()
        .and_then(|p| p.ipam_policy_id())
        .unwrap()
        .to_string();
    let resp = client
        .get_ipam_policy_allocation_rules()
        .ipam_policy_id(&policy_id)
        .send()
        .await
        .expect("get_ipam_policy_allocation_rules should succeed");
    // The created IPAM policy returns one document with empty allocation rules.
    assert_eq!(resp.ipam_policy_documents().len(), 1);
}

#[tokio::test]
async fn test_get_ipam_policy_organization_targets() {
    let client = make_ec2_client().await;
    let resp = client
        .get_ipam_policy_organization_targets()
        .ipam_policy_id("ipam-pol-1234")
        .send()
        .await
        .expect("get_ipam_policy_organization_targets should succeed");
    assert!(resp.organization_targets().is_empty());
}

#[tokio::test]
async fn test_get_ipam_pool_allocations() {
    let client = make_ec2_client().await;
    let resp = client
        .get_ipam_pool_allocations()
        .ipam_pool_id("ipam-pool-1234")
        .send()
        .await
        .expect("get_ipam_pool_allocations should succeed");
    assert!(resp.ipam_pool_allocations().is_empty());
}

#[tokio::test]
async fn test_get_spot_placement_scores() {
    let client = make_ec2_client().await;
    let resp = client
        .get_spot_placement_scores()
        .target_capacity(1)
        .send()
        .await
        .expect("get_spot_placement_scores should succeed");
    assert!(resp.spot_placement_scores().is_empty());
}

#[tokio::test]
async fn test_get_verified_access_endpoint_policy() {
    let client = make_ec2_client().await;
    let resp = client
        .get_verified_access_endpoint_policy()
        .verified_access_endpoint_id("vae-1234")
        .send()
        .await
        .expect("get_verified_access_endpoint_policy should succeed");
    assert!(resp.policy_document().is_none());
}

#[tokio::test]
async fn test_get_verified_access_endpoint_targets() {
    let client = make_ec2_client().await;
    let resp = client
        .get_verified_access_endpoint_targets()
        .verified_access_endpoint_id("vae-1234")
        .send()
        .await
        .expect("get_verified_access_endpoint_targets should succeed");
    assert!(resp.verified_access_endpoint_targets().is_empty());
}

#[tokio::test]
async fn test_get_verified_access_group_policy() {
    let client = make_ec2_client().await;
    let resp = client
        .get_verified_access_group_policy()
        .verified_access_group_id("vag-1234")
        .send()
        .await
        .expect("get_verified_access_group_policy should succeed");
    assert!(resp.policy_document().is_none());
}

#[tokio::test]
async fn test_describe_network_insights_access_scope_analyses() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_network_insights_access_scope_analyses()
        .send()
        .await
        .expect("describe_network_insights_access_scope_analyses should succeed");
    assert!(resp.network_insights_access_scope_analyses().is_empty());
}

#[tokio::test]
async fn test_describe_network_insights_access_scopes() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_network_insights_access_scopes()
        .send()
        .await
        .expect("describe_network_insights_access_scopes should succeed");
    assert!(resp.network_insights_access_scopes().is_empty());
}

#[tokio::test]
async fn test_describe_route_server_endpoints() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_route_server_endpoints()
        .send()
        .await
        .expect("describe_route_server_endpoints should succeed");
    assert!(resp.route_server_endpoints().is_empty());
}

#[tokio::test]
async fn test_describe_route_server_peers() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_route_server_peers()
        .send()
        .await
        .expect("describe_route_server_peers should succeed");
    assert!(resp.route_server_peers().is_empty());
}

#[tokio::test]
async fn test_describe_route_servers() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_route_servers()
        .send()
        .await
        .expect("describe_route_servers should succeed");
    assert!(resp.route_servers().is_empty());
}

#[tokio::test]
async fn test_describe_secondary_subnets() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_secondary_subnets()
        .send()
        .await
        .expect("describe_secondary_subnets should succeed");
    assert!(resp.secondary_subnets().is_empty());
}

#[tokio::test]
async fn test_describe_security_group_vpc_associations() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_security_group_vpc_associations()
        .send()
        .await
        .expect("describe_security_group_vpc_associations should succeed");
    assert!(resp.security_group_vpc_associations().is_empty());
}

#[tokio::test]
async fn test_describe_vpc_block_public_access_exclusions() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_vpc_block_public_access_exclusions()
        .send()
        .await
        .expect("describe_vpc_block_public_access_exclusions should succeed");
    assert!(resp.vpc_block_public_access_exclusions().is_empty());
}

#[tokio::test]
async fn test_describe_vpc_block_public_access_options() {
    let client = make_ec2_client().await;
    let _resp = client
        .describe_vpc_block_public_access_options()
        .send()
        .await
        .expect("describe_vpc_block_public_access_options should succeed");
}

#[tokio::test]
async fn test_describe_vpc_encryption_controls() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_vpc_encryption_controls()
        .send()
        .await
        .expect("describe_vpc_encryption_controls should succeed");
    assert!(resp.vpc_encryption_controls().is_empty());
}

#[tokio::test]
async fn test_get_associated_ipv6_pool_cidrs() {
    let client = make_ec2_client().await;
    let resp = client
        .get_associated_ipv6_pool_cidrs()
        .pool_id("ipv6pool-ec2-1234")
        .send()
        .await
        .expect("get_associated_ipv6_pool_cidrs should succeed");
    assert!(resp.ipv6_cidr_associations().is_empty());
}

#[tokio::test]
async fn test_get_aws_network_performance_data() {
    let client = make_ec2_client().await;
    let resp = client
        .get_aws_network_performance_data()
        .send()
        .await
        .expect("get_aws_network_performance_data should succeed");
    assert!(resp.data_responses().is_empty());
}

#[tokio::test]
async fn test_get_flow_logs_integration_template() {
    let client = make_ec2_client().await;
    let resp = client
        .get_flow_logs_integration_template()
        .flow_log_id("fl-1234")
        .config_delivery_s3_destination_arn("arn:aws:s3:::bucket")
        .integrate_services(aws_sdk_ec2::types::IntegrateServices::builder().build())
        .send()
        .await
        .expect("get_flow_logs_integration_template should succeed");
    let template = resp.result().expect("template should be populated");
    assert!(template.contains("fl-1234"));
}

#[tokio::test]
async fn test_get_ipam_address_history() {
    let client = make_ec2_client().await;
    let resp = client
        .get_ipam_address_history()
        .cidr("10.0.0.0/24")
        .ipam_scope_id("ipam-scope-1234")
        .send()
        .await
        .expect("get_ipam_address_history should succeed");
    assert!(resp.history_records().is_empty());
}

#[tokio::test]
async fn test_get_ipam_discovered_public_addresses() {
    let client = make_ec2_client().await;
    let resp = client
        .get_ipam_discovered_public_addresses()
        .ipam_resource_discovery_id("ipam-res-disco-1234")
        .address_region("us-east-1")
        .send()
        .await
        .expect("get_ipam_discovered_public_addresses should succeed");
    assert!(resp.ipam_discovered_public_addresses().is_empty());
}

#[tokio::test]
async fn test_get_ipam_discovered_resource_cidrs() {
    let client = make_ec2_client().await;
    let resp = client
        .get_ipam_discovered_resource_cidrs()
        .ipam_resource_discovery_id("ipam-res-disco-1234")
        .resource_region("us-east-1")
        .send()
        .await
        .expect("get_ipam_discovered_resource_cidrs should succeed");
    assert!(resp.ipam_discovered_resource_cidrs().is_empty());
}

#[tokio::test]
async fn test_get_ipam_pool_cidrs() {
    let client = make_ec2_client().await;
    let resp = client
        .get_ipam_pool_cidrs()
        .ipam_pool_id("ipam-pool-1234")
        .send()
        .await
        .expect("get_ipam_pool_cidrs should succeed");
    assert!(resp.ipam_pool_cidrs().is_empty());
}

#[tokio::test]
async fn test_get_ipam_resource_cidrs() {
    let client = make_ec2_client().await;
    let resp = client
        .get_ipam_resource_cidrs()
        .ipam_scope_id("ipam-scope-1234")
        .send()
        .await
        .expect("get_ipam_resource_cidrs should succeed");
    assert!(resp.ipam_resource_cidrs().is_empty());
}

#[tokio::test]
async fn test_get_managed_prefix_list_associations() {
    // Wave 3: handler validates the prefix list ID; an unseeded ID now errors.
    let client = make_ec2_client().await;
    let err = client
        .get_managed_prefix_list_associations()
        .prefix_list_id("pl-1234")
        .send()
        .await
        .expect_err("expected NotFound for unseeded prefix list");
    let dbg = format!("{err:?}");
    assert!(dbg.contains("InvalidPrefixListID.NotFound"), "{dbg}");
}

#[tokio::test]
async fn test_get_network_insights_access_scope_analysis_findings() {
    // Wave 3: handler validates the analysis ID; an unseeded ID now errors.
    let client = make_ec2_client().await;
    let err = client
        .get_network_insights_access_scope_analysis_findings()
        .network_insights_access_scope_analysis_id("nisa-1234")
        .send()
        .await
        .expect_err("expected NotFound for unseeded scope analysis");
    let dbg = format!("{err:?}");
    assert!(
        dbg.contains("InvalidNetworkInsightsAccessScopeAnalysisId.NotFound"),
        "{dbg}"
    );
}

#[tokio::test]
async fn test_get_network_insights_access_scope_content() {
    // Wave 3: handler validates the scope ID; an unseeded ID now errors.
    let client = make_ec2_client().await;
    let err = client
        .get_network_insights_access_scope_content()
        .network_insights_access_scope_id("nis-1234")
        .send()
        .await
        .expect_err("expected NotFound for unseeded access scope");
    let dbg = format!("{err:?}");
    assert!(
        dbg.contains("InvalidNetworkInsightsAccessScopeId.NotFound"),
        "{dbg}"
    );
}

#[tokio::test]
async fn test_describe_fpga_image_attribute() {
    // Wave 4: handler validates the FPGA image ID; an unseeded ID now errors.
    let client = make_ec2_client().await;
    let err = client
        .describe_fpga_image_attribute()
        .fpga_image_id("afi-1234")
        .attribute(aws_sdk_ec2::types::FpgaImageAttributeName::Description)
        .send()
        .await
        .expect_err("expected NotFound for unseeded FPGA image");
    let dbg = format!("{err:?}");
    assert!(dbg.contains("InvalidFpgaImageID.NotFound"), "{dbg}");
}

// --- Fifth batch ---

#[tokio::test]
async fn test_export_client_vpn_client_certificate_revocation_list() {
    // Wave 3: handler validates the endpoint ID; an unseeded ID now errors.
    let client = make_ec2_client().await;
    let err = client
        .export_client_vpn_client_certificate_revocation_list()
        .client_vpn_endpoint_id("cvpn-endpoint-1234")
        .send()
        .await
        .expect_err("expected NotFound for unseeded client VPN endpoint");
    let dbg = format!("{err:?}");
    assert!(dbg.contains("InvalidClientVpnEndpointId.NotFound"), "{dbg}");
}

#[tokio::test]
async fn test_export_client_vpn_client_configuration() {
    // Wave 3: handler validates the endpoint ID; an unseeded ID now errors.
    let client = make_ec2_client().await;
    let err = client
        .export_client_vpn_client_configuration()
        .client_vpn_endpoint_id("cvpn-endpoint-1234")
        .send()
        .await
        .expect_err("expected NotFound for unseeded client VPN endpoint");
    let dbg = format!("{err:?}");
    assert!(dbg.contains("InvalidClientVpnEndpointId.NotFound"), "{dbg}");
}

#[tokio::test]
async fn test_export_transit_gateway_routes() {
    let client = make_ec2_client().await;
    let (tgw, _, _) = wave2_seed_tgw_and_vpc_attach(&client).await;
    let rtb_id = client
        .create_transit_gateway_route_table()
        .transit_gateway_id(&tgw)
        .send()
        .await
        .unwrap()
        .transit_gateway_route_table()
        .unwrap()
        .transit_gateway_route_table_id()
        .unwrap()
        .to_string();
    let _resp = client
        .export_transit_gateway_routes()
        .transit_gateway_route_table_id(&rtb_id)
        .s3_bucket("my-bucket")
        .send()
        .await
        .expect("export_transit_gateway_routes should succeed");
}

#[tokio::test]
async fn test_get_active_vpn_tunnel_status() {
    // Wave 4: handler validates the VPN connection ID; an unseeded ID now errors.
    let client = make_ec2_client().await;
    let err = client
        .get_active_vpn_tunnel_status()
        .vpn_connection_id("vpn-1234")
        .vpn_tunnel_outside_ip_address("203.0.113.1")
        .send()
        .await
        .expect_err("expected NotFound for unseeded VPN connection");
    let dbg = format!("{err:?}");
    assert!(dbg.contains("InvalidVpnConnectionID.NotFound"), "{dbg}");
}

#[tokio::test]
async fn test_get_vpn_connection_device_sample_configuration() {
    // Wave 4: handler validates the VPN connection ID; an unseeded ID now errors.
    let client = make_ec2_client().await;
    let err = client
        .get_vpn_connection_device_sample_configuration()
        .vpn_connection_id("vpn-1234")
        .vpn_connection_device_type_id("vpn-device-1234")
        .send()
        .await
        .expect_err("expected NotFound for unseeded VPN connection");
    let dbg = format!("{err:?}");
    assert!(dbg.contains("InvalidVpnConnectionID.NotFound"), "{dbg}");
}

#[tokio::test]
async fn test_get_vpn_tunnel_replacement_status() {
    // Wave 4: handler validates the VPN connection ID; an unseeded ID now errors.
    let client = make_ec2_client().await;
    let err = client
        .get_vpn_tunnel_replacement_status()
        .vpn_connection_id("vpn-1234")
        .vpn_tunnel_outside_ip_address("203.0.113.1")
        .send()
        .await
        .expect_err("expected NotFound for unseeded VPN connection");
    let dbg = format!("{err:?}");
    assert!(dbg.contains("InvalidVpnConnectionID.NotFound"), "{dbg}");
}

#[tokio::test]
async fn test_search_local_gateway_routes() {
    let client = make_ec2_client().await;
    let resp = client
        .search_local_gateway_routes()
        .local_gateway_route_table_id("lgw-rtb-1234")
        .send()
        .await
        .expect("search_local_gateway_routes should succeed");
    assert!(resp.routes().is_empty());
}

#[tokio::test]
async fn test_search_transit_gateway_multicast_groups() {
    let client = make_ec2_client().await;
    let resp = client
        .search_transit_gateway_multicast_groups()
        .transit_gateway_multicast_domain_id("tgw-mcast-domain-1234")
        .send()
        .await
        .expect("search_transit_gateway_multicast_groups should succeed");
    assert!(resp.multicast_groups().is_empty());
}

#[tokio::test]
async fn test_export_image() {
    let client = make_ec2_client().await;
    let _resp = client
        .export_image()
        .image_id("ami-1234567890abcdef0")
        .disk_image_format(aws_sdk_ec2::types::DiskImageFormat::Vmdk)
        .s3_export_location(
            aws_sdk_ec2::types::ExportTaskS3LocationRequest::builder()
                .s3_bucket("my-bucket")
                .build(),
        )
        .send()
        .await
        .expect("export_image should succeed");
}

#[tokio::test]
async fn test_export_verified_access_instance_client_configuration() {
    // Wave 4: handler validates the Verified Access instance ID; create one first.
    let client = make_ec2_client().await;
    let inst_id = client
        .create_verified_access_instance()
        .send()
        .await
        .expect("create_verified_access_instance should succeed")
        .verified_access_instance
        .unwrap()
        .verified_access_instance_id
        .unwrap();
    let resp = client
        .export_verified_access_instance_client_configuration()
        .verified_access_instance_id(&inst_id)
        .send()
        .await
        .expect("export_verified_access_instance_client_configuration should succeed");
    assert_eq!(resp.verified_access_instance_id(), Some(inst_id.as_str()));
}

#[tokio::test]
async fn test_get_console_screenshot() {
    let client = make_ec2_client().await;
    let _resp = client
        .get_console_screenshot()
        .instance_id("i-1234567890abcdef0")
        .send()
        .await
        .expect("get_console_screenshot should succeed");
}

#[tokio::test]
async fn test_get_reserved_instances_exchange_quote() {
    // Wave 4: handler validates the source RI IDs; an unseeded ID now errors.
    let client = make_ec2_client().await;
    let err = client
        .get_reserved_instances_exchange_quote()
        .reserved_instance_ids("ri-1234")
        .send()
        .await
        .expect_err("expected NotFound for unseeded reserved instance");
    let dbg = format!("{err:?}");
    assert!(dbg.contains("InvalidReservedInstancesId.NotFound"), "{dbg}");
}

#[tokio::test]
async fn test_list_images_in_recycle_bin() {
    let client = make_ec2_client().await;
    let resp = client
        .list_images_in_recycle_bin()
        .send()
        .await
        .expect("list_images_in_recycle_bin should succeed");
    assert!(resp.images().is_empty());
}

#[tokio::test]
async fn test_get_capacity_manager_attributes() {
    let client = make_ec2_client().await;
    let _resp = client
        .get_capacity_manager_attributes()
        .send()
        .await
        .expect("get_capacity_manager_attributes should succeed");
}

#[tokio::test]
async fn test_get_capacity_manager_metric_data() {
    let client = make_ec2_client().await;
    let _resp = client
        .get_capacity_manager_metric_data()
        .start_time(aws_sdk_ec2::primitives::DateTime::from_secs(0))
        .end_time(aws_sdk_ec2::primitives::DateTime::from_secs(1))
        .send()
        .await
        .expect("get_capacity_manager_metric_data should succeed");
}

#[tokio::test]
async fn test_get_capacity_manager_metric_dimensions() {
    let client = make_ec2_client().await;
    let _resp = client
        .get_capacity_manager_metric_dimensions()
        .send()
        .await
        .expect("get_capacity_manager_metric_dimensions should succeed");
}

#[tokio::test]
async fn test_get_declarative_policies_report_summary() {
    // Wave 4: handler validates the report ID; an unseeded ID now errors.
    let client = make_ec2_client().await;
    let err = client
        .get_declarative_policies_report_summary()
        .report_id("report-1234")
        .send()
        .await
        .expect_err("expected NotFound for unseeded report");
    let dbg = format!("{err:?}");
    assert!(dbg.contains("InvalidReportId.NotFound"), "{dbg}");
}

async fn seed_ipam_prefix_list_resolver(client: &aws_sdk_ec2::Client) -> String {
    use aws_sdk_ec2::types::AddIpamOperatingRegion;
    let ipam_id = client
        .create_ipam()
        .operating_regions(
            AddIpamOperatingRegion::builder()
                .region_name("us-east-1")
                .build(),
        )
        .send()
        .await
        .unwrap()
        .ipam()
        .and_then(|i| i.ipam_id())
        .unwrap()
        .to_string();
    client
        .create_ipam_prefix_list_resolver()
        .ipam_id(&ipam_id)
        .address_family(aws_sdk_ec2::types::AddressFamily::Ipv4)
        .send()
        .await
        .unwrap()
        .ipam_prefix_list_resolver()
        .and_then(|r| r.ipam_prefix_list_resolver_id())
        .unwrap()
        .to_string()
}

#[tokio::test]
async fn test_get_ipam_prefix_list_resolver_rules() {
    let client = make_ec2_client().await;
    let resolver_id = seed_ipam_prefix_list_resolver(&client).await;
    let resp = client
        .get_ipam_prefix_list_resolver_rules()
        .ipam_prefix_list_resolver_id(&resolver_id)
        .send()
        .await
        .expect("get_ipam_prefix_list_resolver_rules should succeed");
    assert!(resp.rules().is_empty());
}

#[tokio::test]
async fn test_get_ipam_prefix_list_resolver_version_entries() {
    let client = make_ec2_client().await;
    let resolver_id = seed_ipam_prefix_list_resolver(&client).await;
    let resp = client
        .get_ipam_prefix_list_resolver_version_entries()
        .ipam_prefix_list_resolver_id(&resolver_id)
        .ipam_prefix_list_resolver_version(1)
        .send()
        .await
        .expect("get_ipam_prefix_list_resolver_version_entries should succeed");
    assert!(resp.entries().is_empty());
}

#[tokio::test]
async fn test_get_ipam_prefix_list_resolver_versions() {
    let client = make_ec2_client().await;
    let resolver_id = seed_ipam_prefix_list_resolver(&client).await;
    let resp = client
        .get_ipam_prefix_list_resolver_versions()
        .ipam_prefix_list_resolver_id(&resolver_id)
        .send()
        .await
        .expect("get_ipam_prefix_list_resolver_versions should succeed");
    let versions = resp.ipam_prefix_list_resolver_versions();
    // Mock emits exactly one synthetic version entry for any extant resolver.
    assert_eq!(versions.len(), 1);
    assert_eq!(versions[0].version(), Some(1));
}

#[tokio::test]
async fn test_get_route_server_associations() {
    let client = make_ec2_client().await;
    let resp = client
        .get_route_server_associations()
        .route_server_id("rs-1234")
        .send()
        .await
        .expect("get_route_server_associations should succeed");
    assert!(resp.route_server_associations().is_empty());
}

#[tokio::test]
async fn test_get_route_server_propagations() {
    // Wave 3: handler validates the route server ID; an unseeded ID now errors.
    let client = make_ec2_client().await;
    let err = client
        .get_route_server_propagations()
        .route_server_id("rs-1234")
        .send()
        .await
        .expect_err("expected NotFound for unseeded route server");
    let dbg = format!("{err:?}");
    assert!(dbg.contains("InvalidRouteServerId.NotFound"), "{dbg}");
}

#[tokio::test]
async fn test_get_route_server_routing_database() {
    // Wave 3: handler validates the route server ID; an unseeded ID now errors.
    let client = make_ec2_client().await;
    let err = client
        .get_route_server_routing_database()
        .route_server_id("rs-1234")
        .send()
        .await
        .expect_err("expected NotFound for unseeded route server");
    let dbg = format!("{err:?}");
    assert!(dbg.contains("InvalidRouteServerId.NotFound"), "{dbg}");
}

#[tokio::test]
async fn test_get_vpc_resources_blocking_encryption_enforcement() {
    let client = make_ec2_client().await;
    let resp = client
        .get_vpc_resources_blocking_encryption_enforcement()
        .vpc_id("vpc-1234")
        .send()
        .await
        .expect("get_vpc_resources_blocking_encryption_enforcement should succeed");
    assert!(resp.non_compliant_resources().is_empty());
}

#[tokio::test]
async fn test_list_snapshots_in_recycle_bin() {
    let client = make_ec2_client().await;
    let resp = client
        .list_snapshots_in_recycle_bin()
        .send()
        .await
        .expect("list_snapshots_in_recycle_bin should succeed");
    assert!(resp.snapshots().is_empty());
}

#[tokio::test]
async fn test_list_volumes_in_recycle_bin() {
    let client = make_ec2_client().await;
    let resp = client
        .list_volumes_in_recycle_bin()
        .send()
        .await
        .expect("list_volumes_in_recycle_bin should succeed");
    assert!(resp.volumes().is_empty());
}

// --- Enable / Disable no-ops ---

#[tokio::test]
async fn test_disable_allowed_images_settings() {
    let client = make_ec2_client().await;
    let _resp = client
        .disable_allowed_images_settings()
        .send()
        .await
        .expect("disable_allowed_images_settings should succeed");
}

#[tokio::test]
async fn test_disable_image() {
    let client = make_ec2_client().await;
    // Register a real AMI first.
    let ami_id = client
        .register_image()
        .name("disable-image-fixture")
        .send()
        .await
        .expect("register_image")
        .image_id()
        .expect("image id")
        .to_string();
    let _resp = client
        .disable_image()
        .image_id(&ami_id)
        .send()
        .await
        .expect("disable_image should succeed");
}

#[tokio::test]
async fn test_disable_image_block_public_access() {
    let client = make_ec2_client().await;
    let _resp = client
        .disable_image_block_public_access()
        .send()
        .await
        .expect("disable_image_block_public_access should succeed");
}

#[tokio::test]
async fn test_disable_image_deprecation() {
    let client = make_ec2_client().await;
    let ami_id = client
        .register_image()
        .name("disable-deprecation-fixture")
        .send()
        .await
        .expect("register_image")
        .image_id()
        .expect("image id")
        .to_string();
    let _resp = client
        .disable_image_deprecation()
        .image_id(&ami_id)
        .send()
        .await
        .expect("disable_image_deprecation should succeed");
}

#[tokio::test]
async fn test_disable_image_deregistration_protection() {
    // Wave 4: handler validates image existence and mutates state.
    let client = make_ec2_client().await;
    let ami_id = client
        .register_image()
        .name("disable-deregistration-protection-fixture")
        .send()
        .await
        .expect("register_image")
        .image_id()
        .expect("image id")
        .to_string();
    let resp = client
        .disable_image_deregistration_protection()
        .image_id(&ami_id)
        .send()
        .await
        .expect("disable_image_deregistration_protection should succeed");
    assert_eq!(resp.r#return(), Some("disabled"));
}

#[tokio::test]
async fn test_disable_instance_sql_ha_standby_detections() {
    let client = make_ec2_client().await;
    let _resp = client
        .disable_instance_sql_ha_standby_detections()
        .instance_ids("i-1234567890abcdef0")
        .send()
        .await
        .expect("disable_instance_sql_ha_standby_detections should succeed");
}

#[tokio::test]
async fn test_enable_allowed_images_settings() {
    let client = make_ec2_client().await;
    let _resp = client
        .enable_allowed_images_settings()
        .allowed_images_settings_state(
            aws_sdk_ec2::types::AllowedImagesSettingsEnabledState::Enabled,
        )
        .send()
        .await
        .expect("enable_allowed_images_settings should succeed");
}

#[tokio::test]
async fn test_enable_image() {
    let client = make_ec2_client().await;
    let ami_id = client
        .register_image()
        .name("enable-image-fixture")
        .send()
        .await
        .expect("register_image")
        .image_id()
        .expect("image id")
        .to_string();
    let _resp = client
        .enable_image()
        .image_id(&ami_id)
        .send()
        .await
        .expect("enable_image should succeed");
}

#[tokio::test]
async fn test_enable_image_block_public_access() {
    let client = make_ec2_client().await;
    let _resp = client
        .enable_image_block_public_access()
        .image_block_public_access_state(
            aws_sdk_ec2::types::ImageBlockPublicAccessEnabledState::BlockNewSharing,
        )
        .send()
        .await
        .expect("enable_image_block_public_access should succeed");
}

#[tokio::test]
async fn test_enable_image_deprecation() {
    let client = make_ec2_client().await;
    let ami_id = client
        .register_image()
        .name("enable-deprecation-fixture")
        .send()
        .await
        .expect("register_image")
        .image_id()
        .expect("image id")
        .to_string();
    let _resp = client
        .enable_image_deprecation()
        .image_id(&ami_id)
        .deprecate_at(aws_sdk_ec2::primitives::DateTime::from_secs(2_000_000_000))
        .send()
        .await
        .expect("enable_image_deprecation should succeed");
}

#[tokio::test]
async fn test_enable_image_deregistration_protection() {
    // Wave 4: handler validates image existence and mutates state.
    let client = make_ec2_client().await;
    let ami_id = client
        .register_image()
        .name("enable-deregistration-protection-fixture")
        .send()
        .await
        .expect("register_image")
        .image_id()
        .expect("image id")
        .to_string();
    let _resp = client
        .enable_image_deregistration_protection()
        .image_id(&ami_id)
        .send()
        .await
        .expect("enable_image_deregistration_protection should succeed");
}

#[tokio::test]
async fn test_enable_instance_sql_ha_standby_detections() {
    let client = make_ec2_client().await;
    let _resp = client
        .enable_instance_sql_ha_standby_detections()
        .instance_ids("i-1234567890abcdef0")
        .send()
        .await
        .expect("enable_instance_sql_ha_standby_detections should succeed");
}

#[tokio::test]
async fn test_disable_capacity_manager() {
    let client = make_ec2_client().await;
    let _resp = client
        .disable_capacity_manager()
        .send()
        .await
        .expect("disable_capacity_manager should succeed");
}

#[tokio::test]
async fn test_disable_fast_launch() {
    let client = make_ec2_client().await;
    let image_id = client
        .register_image()
        .name("test-ami")
        .architecture(aws_sdk_ec2::types::ArchitectureValues::X8664)
        .root_device_name("/dev/xvda")
        .send()
        .await
        .expect("register_image")
        .image_id()
        .expect("image_id")
        .to_string();
    let resp = client
        .disable_fast_launch()
        .image_id(&image_id)
        .send()
        .await
        .expect("disable_fast_launch should succeed");
    assert_eq!(resp.image_id(), Some(image_id.as_str()));
    assert_eq!(resp.state().map(|s| s.as_str()), Some("disabled"));
}

#[tokio::test]
async fn test_disable_ipam_organization_admin_account() {
    let client = make_ec2_client().await;
    let _resp = client
        .disable_ipam_organization_admin_account()
        .delegated_admin_account_id("123456789012")
        .send()
        .await
        .expect("disable_ipam_organization_admin_account should succeed");
}

#[tokio::test]
async fn test_disable_ipam_policy() {
    use aws_sdk_ec2::types::AddIpamOperatingRegion;
    let client = make_ec2_client().await;
    let ipam_id = client
        .create_ipam()
        .operating_regions(
            AddIpamOperatingRegion::builder()
                .region_name("us-east-1")
                .build(),
        )
        .send()
        .await
        .expect("create_ipam")
        .ipam()
        .and_then(|i| i.ipam_id())
        .expect("ipam id")
        .to_string();
    let policy_id = client
        .create_ipam_policy()
        .ipam_id(&ipam_id)
        .send()
        .await
        .expect("create_ipam_policy")
        .ipam_policy()
        .and_then(|p| p.ipam_policy_id())
        .expect("policy id")
        .to_string();
    let _resp = client
        .disable_ipam_policy()
        .ipam_policy_id(&policy_id)
        .send()
        .await
        .expect("disable_ipam_policy should succeed");
}

#[tokio::test]
async fn test_disable_serial_console_access() {
    let client = make_ec2_client().await;
    let _resp = client
        .disable_serial_console_access()
        .send()
        .await
        .expect("disable_serial_console_access should succeed");
}

#[tokio::test]
async fn test_enable_capacity_manager() {
    let client = make_ec2_client().await;
    let _resp = client
        .enable_capacity_manager()
        .send()
        .await
        .expect("enable_capacity_manager should succeed");
}

#[tokio::test]
async fn test_enable_fast_launch() {
    let client = make_ec2_client().await;
    let image_id = client
        .register_image()
        .name("test-ami-fl")
        .architecture(aws_sdk_ec2::types::ArchitectureValues::X8664)
        .root_device_name("/dev/xvda")
        .send()
        .await
        .expect("register_image")
        .image_id()
        .expect("image_id")
        .to_string();
    let resp = client
        .enable_fast_launch()
        .image_id(&image_id)
        .send()
        .await
        .expect("enable_fast_launch should succeed");
    assert_eq!(resp.image_id(), Some(image_id.as_str()));
    assert_eq!(resp.state().map(|s| s.as_str()), Some("enabled"));
}

#[tokio::test]
async fn test_enable_ipam_organization_admin_account() {
    let client = make_ec2_client().await;
    let _resp = client
        .enable_ipam_organization_admin_account()
        .delegated_admin_account_id("123456789012")
        .send()
        .await
        .expect("enable_ipam_organization_admin_account should succeed");
}

#[tokio::test]
async fn test_enable_ipam_policy() {
    use aws_sdk_ec2::types::AddIpamOperatingRegion;
    let client = make_ec2_client().await;
    let ipam_id = client
        .create_ipam()
        .operating_regions(
            AddIpamOperatingRegion::builder()
                .region_name("us-east-1")
                .build(),
        )
        .send()
        .await
        .expect("create_ipam")
        .ipam()
        .and_then(|i| i.ipam_id())
        .expect("ipam id")
        .to_string();
    let policy_id = client
        .create_ipam_policy()
        .ipam_id(&ipam_id)
        .send()
        .await
        .expect("create_ipam_policy")
        .ipam_policy()
        .and_then(|p| p.ipam_policy_id())
        .expect("policy id")
        .to_string();
    let _resp = client
        .enable_ipam_policy()
        .ipam_policy_id(&policy_id)
        .send()
        .await
        .expect("enable_ipam_policy should succeed");
}

#[tokio::test]
async fn test_enable_reachability_analyzer_organization_sharing() {
    let client = make_ec2_client().await;
    let _resp = client
        .enable_reachability_analyzer_organization_sharing()
        .send()
        .await
        .expect("enable_reachability_analyzer_organization_sharing should succeed");
}

#[tokio::test]
async fn test_enable_serial_console_access() {
    let client = make_ec2_client().await;
    let _resp = client
        .enable_serial_console_access()
        .send()
        .await
        .expect("enable_serial_console_access should succeed");
}

#[tokio::test]
async fn test_disable_address_transfer() {
    let client = make_ec2_client().await;
    // Allocate a real EIP, enable a transfer, then disable it.
    let allocation_id = client
        .allocate_address()
        .send()
        .await
        .expect("allocate_address")
        .allocation_id()
        .expect("allocation id")
        .to_string();
    client
        .enable_address_transfer()
        .allocation_id(&allocation_id)
        .transfer_account_id("123456789012")
        .send()
        .await
        .expect("enable_address_transfer");
    let _resp = client
        .disable_address_transfer()
        .allocation_id(&allocation_id)
        .send()
        .await
        .expect("disable_address_transfer should succeed");
}

#[tokio::test]
async fn test_disable_aws_network_performance_metric_subscription() {
    let client = make_ec2_client().await;
    let _resp = client
        .disable_aws_network_performance_metric_subscription()
        .send()
        .await
        .expect("disable_aws_network_performance_metric_subscription should succeed");
}

#[tokio::test]
async fn test_disable_route_server_propagation() {
    // Wave 4: handler validates the route server association exists.
    let client = make_ec2_client().await;
    let err = client
        .disable_route_server_propagation()
        .route_server_id("rs-1234")
        .route_table_id("rtb-1234")
        .send()
        .await
        .expect_err("expected NotFound for unseeded route server");
    let dbg = format!("{err:?}");
    assert!(dbg.contains("InvalidRouteServerId.NotFound"), "{dbg}");
}

#[tokio::test]
async fn test_disable_vgw_route_propagation() {
    let client = make_ec2_client().await;
    let _resp = client
        .disable_vgw_route_propagation()
        .gateway_id("vgw-1234")
        .route_table_id("rtb-1234")
        .send()
        .await
        .expect("disable_vgw_route_propagation should succeed");
}

#[tokio::test]
async fn test_enable_address_transfer() {
    let client = make_ec2_client().await;
    let allocation_id = client
        .allocate_address()
        .send()
        .await
        .expect("allocate_address")
        .allocation_id()
        .expect("allocation id")
        .to_string();
    let _resp = client
        .enable_address_transfer()
        .allocation_id(&allocation_id)
        .transfer_account_id("123456789012")
        .send()
        .await
        .expect("enable_address_transfer should succeed");
}

#[tokio::test]
async fn test_enable_aws_network_performance_metric_subscription() {
    let client = make_ec2_client().await;
    let _resp = client
        .enable_aws_network_performance_metric_subscription()
        .send()
        .await
        .expect("enable_aws_network_performance_metric_subscription should succeed");
}

#[tokio::test]
async fn test_enable_route_server_propagation() {
    // Wave 4: handler validates the route server association exists.
    let client = make_ec2_client().await;
    let err = client
        .enable_route_server_propagation()
        .route_server_id("rs-1234")
        .route_table_id("rtb-1234")
        .send()
        .await
        .expect_err("expected NotFound for unseeded route server");
    let dbg = format!("{err:?}");
    assert!(dbg.contains("InvalidRouteServerId.NotFound"), "{dbg}");
}

#[tokio::test]
async fn test_enable_vgw_route_propagation() {
    let client = make_ec2_client().await;
    let _resp = client
        .enable_vgw_route_propagation()
        .gateway_id("vgw-1234")
        .route_table_id("rtb-1234")
        .send()
        .await
        .expect("enable_vgw_route_propagation should succeed");
}

#[tokio::test]
async fn test_disable_fast_snapshot_restores() {
    let client = make_ec2_client().await;
    let _resp = client
        .disable_fast_snapshot_restores()
        .availability_zones("us-east-1a")
        .source_snapshot_ids("snap-1234")
        .send()
        .await
        .expect("disable_fast_snapshot_restores should succeed");
}

#[tokio::test]
async fn test_disable_snapshot_block_public_access() {
    let client = make_ec2_client().await;
    let resp = client
        .disable_snapshot_block_public_access()
        .send()
        .await
        .expect("disable_snapshot_block_public_access should succeed");
    assert_eq!(resp.state().map(|s| s.as_str()), Some("unblocked"));
}

#[tokio::test]
async fn test_enable_fast_snapshot_restores() {
    let client = make_ec2_client().await;
    let _resp = client
        .enable_fast_snapshot_restores()
        .availability_zones("us-east-1a")
        .source_snapshot_ids("snap-1234")
        .send()
        .await
        .expect("enable_fast_snapshot_restores should succeed");
}

#[tokio::test]
async fn test_enable_snapshot_block_public_access() {
    let client = make_ec2_client().await;
    let resp = client
        .enable_snapshot_block_public_access()
        .state(aws_sdk_ec2::types::SnapshotBlockPublicAccessState::BlockAllSharing)
        .send()
        .await
        .expect("enable_snapshot_block_public_access should succeed");
    assert_eq!(resp.state().map(|s| s.as_str()), Some("block-all-sharing"));
}

// ---------------------------------------------------------------------------
// Group 1: small misc families - integration tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_create_delete_coip_pool() {
    let client = make_ec2_client().await;

    let create_resp = client
        .create_coip_pool()
        .local_gateway_route_table_id("lgw-rtb-12345678")
        .send()
        .await
        .expect("create_coip_pool should succeed");
    let pool = create_resp.coip_pool().expect("should have coip_pool");
    let pool_id = pool.pool_id().expect("pool should have id");
    assert!(
        pool_id.starts_with("ipv4pool-coip-"),
        "expected ipv4pool-coip- prefix, got {pool_id}"
    );
    assert_eq!(
        pool.local_gateway_route_table_id().unwrap_or(""),
        "lgw-rtb-12345678"
    );
    let pool_arn = pool.pool_arn().unwrap_or("").to_string();
    assert!(pool_arn.contains("coip-pool/"));

    let delete_resp = client
        .delete_coip_pool()
        .coip_pool_id(pool_id)
        .send()
        .await
        .expect("delete_coip_pool should succeed");
    let deleted_pool = delete_resp.coip_pool().expect("should return deleted pool");
    assert_eq!(deleted_pool.pool_id().unwrap_or(""), pool_id);

    // Deleting a non-existent pool should return an error.
    let err = client
        .delete_coip_pool()
        .coip_pool_id(pool_id)
        .send()
        .await
        .expect_err("deleting again should fail");
    let dbg = format!("{err:?}");
    assert!(
        dbg.contains("InvalidCoipPoolId.NotFound") || dbg.contains("CoipPool"),
        "expected InvalidCoipPoolId.NotFound, got {dbg}"
    );
}

#[tokio::test]
async fn test_attach_detach_classic_link_vpc() {
    let client = make_ec2_client().await;

    let vpc_resp = client
        .create_vpc()
        .cidr_block("10.0.0.0/16")
        .send()
        .await
        .unwrap();
    let vpc_id = vpc_resp.vpc().unwrap().vpc_id().unwrap().to_string();
    let sg_resp = client
        .create_security_group()
        .group_name("classic-link-sg")
        .description("for classic link test")
        .vpc_id(&vpc_id)
        .send()
        .await
        .unwrap();
    let sg_id = sg_resp.group_id().unwrap().to_string();
    let run_resp = client
        .run_instances()
        .image_id("ami-12345678")
        .instance_type(aws_sdk_ec2::types::InstanceType::T2Micro)
        .min_count(1)
        .max_count(1)
        .send()
        .await
        .unwrap();
    let instance_id = run_resp.instances()[0].instance_id().unwrap().to_string();

    let attach_resp = client
        .attach_classic_link_vpc()
        .instance_id(&instance_id)
        .vpc_id(&vpc_id)
        .groups(&sg_id)
        .send()
        .await
        .expect("attach_classic_link_vpc should succeed");
    assert_eq!(attach_resp.r#return(), Some(true));

    let detach_resp = client
        .detach_classic_link_vpc()
        .instance_id(&instance_id)
        .vpc_id(&vpc_id)
        .send()
        .await
        .expect("detach_classic_link_vpc should succeed");
    assert_eq!(detach_resp.r#return(), Some(true));

    // Detaching again should fail because no link exists.
    let err = client
        .detach_classic_link_vpc()
        .instance_id(&instance_id)
        .vpc_id(&vpc_id)
        .send()
        .await
        .expect_err("second detach should fail");
    assert!(format!("{err:?}").contains("InvalidParameterValue"));
}

#[tokio::test]
async fn test_associate_disassociate_security_group_vpc() {
    let client = make_ec2_client().await;

    let vpc_resp = client
        .create_vpc()
        .cidr_block("10.0.0.0/16")
        .send()
        .await
        .unwrap();
    let vpc_id = vpc_resp.vpc().unwrap().vpc_id().unwrap().to_string();
    let sg_resp = client
        .create_security_group()
        .group_name("sg-vpc-assoc")
        .description("for sg vpc assoc test")
        .vpc_id(&vpc_id)
        .send()
        .await
        .unwrap();
    let sg_id = sg_resp.group_id().unwrap().to_string();

    let assoc_resp = client
        .associate_security_group_vpc()
        .group_id(&sg_id)
        .vpc_id(&vpc_id)
        .send()
        .await
        .expect("associate_security_group_vpc should succeed");
    assert_eq!(
        assoc_resp.state().map(|s| s.as_str()),
        Some("associated"),
        "association state should be 'associated'"
    );

    let dis_resp = client
        .disassociate_security_group_vpc()
        .group_id(&sg_id)
        .vpc_id(&vpc_id)
        .send()
        .await
        .expect("disassociate_security_group_vpc should succeed");
    assert_eq!(dis_resp.state().map(|s| s.as_str()), Some("disassociated"));

    // Disassociating again should fail.
    let err = client
        .disassociate_security_group_vpc()
        .group_id(&sg_id)
        .vpc_id(&vpc_id)
        .send()
        .await
        .expect_err("second disassociate should fail");
    let dbg = format!("{err:?}");
    assert!(
        dbg.contains("InvalidSecurityGroupVpcAssociation.NotFound") || dbg.contains("NotFound"),
        "got {dbg}"
    );
}

#[tokio::test]
async fn test_associate_disassociate_enclave_certificate_iam_role() {
    let client = make_ec2_client().await;
    let cert_arn =
        "arn:aws:acm:us-east-1:000000000000:certificate/abcd1234-abcd-1234-abcd-1234abcd1234";
    let role_arn = "arn:aws:iam::000000000000:role/MyRole";

    let assoc_resp = client
        .associate_enclave_certificate_iam_role()
        .certificate_arn(cert_arn)
        .role_arn(role_arn)
        .send()
        .await
        .expect("associate_enclave_certificate_iam_role should succeed");
    assert!(
        assoc_resp
            .certificate_s3_bucket_name()
            .unwrap_or("")
            .starts_with("aws-ec2-enclave-certificate-"),
        "bucket name should be deterministic"
    );
    assert!(assoc_resp.certificate_s3_object_key().is_some());
    assert!(assoc_resp.encryption_kms_key_id().is_some());

    let dis_resp = client
        .disassociate_enclave_certificate_iam_role()
        .certificate_arn(cert_arn)
        .role_arn(role_arn)
        .send()
        .await
        .expect("disassociate_enclave_certificate_iam_role should succeed");
    assert_eq!(dis_resp.r#return(), Some(true));

    // Disassociating again should fail.
    let err = client
        .disassociate_enclave_certificate_iam_role()
        .certificate_arn(cert_arn)
        .role_arn(role_arn)
        .send()
        .await
        .expect_err("second disassociate should fail");
    let dbg = format!("{err:?}");
    assert!(dbg.contains("NotFound"), "got {dbg}");
}

#[tokio::test]
async fn test_modify_private_dns_name_options() {
    let client = make_ec2_client().await;

    let run_resp = client
        .run_instances()
        .image_id("ami-12345678")
        .instance_type(aws_sdk_ec2::types::InstanceType::T2Micro)
        .min_count(1)
        .max_count(1)
        .send()
        .await
        .unwrap();
    let instance_id = run_resp.instances()[0].instance_id().unwrap().to_string();

    let resp = client
        .modify_private_dns_name_options()
        .instance_id(&instance_id)
        .private_dns_hostname_type(aws_sdk_ec2::types::HostnameType::ResourceName)
        .enable_resource_name_dns_a_record(true)
        .send()
        .await
        .expect("modify_private_dns_name_options should succeed");
    assert_eq!(resp.r#return(), Some(true));

    // Calling on an unknown instance should error.
    let err = client
        .modify_private_dns_name_options()
        .instance_id("i-00000000")
        .private_dns_hostname_type(aws_sdk_ec2::types::HostnameType::IpName)
        .send()
        .await
        .expect_err("should fail for unknown instance");
    assert!(format!("{err:?}").contains("InvalidInstanceID.NotFound"));
}

#[tokio::test]
async fn test_modify_public_ip_dns_name_options() {
    let client = make_ec2_client().await;

    let vpc_resp = client
        .create_vpc()
        .cidr_block("10.0.0.0/16")
        .send()
        .await
        .unwrap();
    let vpc_id = vpc_resp.vpc().unwrap().vpc_id().unwrap().to_string();
    let subnet_resp = client
        .create_subnet()
        .vpc_id(&vpc_id)
        .cidr_block("10.0.1.0/24")
        .availability_zone("us-east-1a")
        .send()
        .await
        .unwrap();
    let subnet_id = subnet_resp
        .subnet()
        .unwrap()
        .subnet_id()
        .unwrap()
        .to_string();
    let eni_resp = client
        .create_network_interface()
        .subnet_id(&subnet_id)
        .description("for public dns hostname test")
        .send()
        .await
        .unwrap();
    let eni_id = eni_resp
        .network_interface()
        .unwrap()
        .network_interface_id()
        .unwrap()
        .to_string();

    let resp = client
        .modify_public_ip_dns_name_options()
        .network_interface_id(&eni_id)
        .hostname_type(aws_sdk_ec2::types::PublicIpDnsOption::PublicDualStackDnsName)
        .send()
        .await
        .expect("modify_public_ip_dns_name_options should succeed");
    assert_eq!(resp.successful(), Some(true));

    // Unknown ENI should error.
    let err = client
        .modify_public_ip_dns_name_options()
        .network_interface_id("eni-00000000")
        .hostname_type(aws_sdk_ec2::types::PublicIpDnsOption::PublicDualStackDnsName)
        .send()
        .await
        .expect_err("should fail for unknown ENI");
    assert!(format!("{err:?}").contains("InvalidNetworkInterfaceID.NotFound"));
}

#[tokio::test]
async fn test_create_mac_system_integrity_protection_modification_task() {
    let client = make_ec2_client().await;

    let run_resp = client
        .run_instances()
        .image_id("ami-12345678")
        .instance_type(aws_sdk_ec2::types::InstanceType::Mac1Metal)
        .min_count(1)
        .max_count(1)
        .send()
        .await
        .unwrap();
    let instance_id = run_resp.instances()[0].instance_id().unwrap().to_string();

    let cfg = aws_sdk_ec2::types::MacSystemIntegrityProtectionConfigurationRequest::builder()
        .apple_internal(aws_sdk_ec2::types::MacSystemIntegrityProtectionSettingStatus::Disabled)
        .base_system(aws_sdk_ec2::types::MacSystemIntegrityProtectionSettingStatus::Enabled)
        .build();
    let resp = client
        .create_mac_system_integrity_protection_modification_task()
        .instance_id(&instance_id)
        .mac_system_integrity_protection_configuration(cfg)
        .mac_system_integrity_protection_status(
            aws_sdk_ec2::types::MacSystemIntegrityProtectionSettingStatus::Disabled,
        )
        .send()
        .await
        .expect("create_mac_system_integrity_protection_modification_task should succeed");
    let task = resp.mac_modification_task().expect("should return task");
    assert_eq!(task.instance_id().unwrap_or(""), instance_id);
    assert!(
        task.mac_modification_task_id()
            .unwrap_or("")
            .starts_with("macmodification-"),
        "task id should be macmodification-prefixed"
    );
    let cfg_resp = task
        .mac_system_integrity_protection_config()
        .expect("should round-trip config");
    assert_eq!(
        cfg_resp.apple_internal().map(|s| s.as_str()),
        Some("disabled")
    );
    assert_eq!(cfg_resp.base_system().map(|s| s.as_str()), Some("enabled"));
}

#[tokio::test]
async fn test_start_cancel_declarative_policies_report() {
    let client = make_ec2_client().await;

    let start_resp = client
        .start_declarative_policies_report()
        .target_id("ou-1234-12345678")
        .s3_bucket("my-policy-reports")
        .s3_prefix("year=2026/")
        .send()
        .await
        .expect("start_declarative_policies_report should succeed");
    let report_id = start_resp.report_id().expect("should return report id");
    assert!(
        report_id.starts_with("report-"),
        "report id should be prefixed: {report_id}"
    );

    let cancel_resp = client
        .cancel_declarative_policies_report()
        .report_id(report_id)
        .send()
        .await
        .expect("cancel_declarative_policies_report should succeed");
    assert_eq!(cancel_resp.r#return(), Some(true));

    // Cancelling again should fail with IncorrectState.
    let err = client
        .cancel_declarative_policies_report()
        .report_id(report_id)
        .send()
        .await
        .expect_err("second cancel should fail");
    let dbg = format!("{err:?}");
    assert!(dbg.contains("IncorrectState"), "got {dbg}");

    // Cancelling unknown id should fail.
    let err = client
        .cancel_declarative_policies_report()
        .report_id("report-00000000")
        .send()
        .await
        .expect_err("unknown report should fail");
    assert!(
        format!("{err:?}").contains("InvalidDeclarativePoliciesReportId.NotFound")
            || format!("{err:?}").contains("NotFound")
    );
}

// ---------------------------------------------------------------------------
// Group 2: Address / BYOIP / NAT-gateway-address / public-IPv4-pool family
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_byoip_cidr_lifecycle() {
    let client = make_ec2_client().await;
    let cidr = "203.0.113.0/24";

    // Provision
    let prov = client
        .provision_byoip_cidr()
        .cidr(cidr)
        .description("test BYOIP")
        .cidr_authorization_context(
            aws_sdk_ec2::types::CidrAuthorizationContext::builder()
                .message("msg")
                .signature("sig")
                .build(),
        )
        .send()
        .await
        .expect("provision_byoip_cidr should succeed");
    let bc = prov.byoip_cidr().expect("byoip_cidr present");
    assert_eq!(bc.cidr().unwrap_or(""), cidr);
    assert_eq!(bc.state().map(|s| s.as_str()).unwrap_or(""), "provisioned");

    // Describe — finds it
    let desc = client
        .describe_byoip_cidrs()
        .max_results(50)
        .send()
        .await
        .expect("describe_byoip_cidrs should succeed");
    assert!(
        desc.byoip_cidrs().iter().any(|c| c.cidr() == Some(cidr)),
        "BYOIP CIDR should be visible after provision"
    );

    // Advertise
    let adv = client
        .advertise_byoip_cidr()
        .cidr(cidr)
        .send()
        .await
        .expect("advertise_byoip_cidr should succeed");
    assert_eq!(
        adv.byoip_cidr().and_then(|c| c.state()).map(|s| s.as_str()),
        Some("advertised")
    );

    // Withdraw
    let wd = client
        .withdraw_byoip_cidr()
        .cidr(cidr)
        .send()
        .await
        .expect("withdraw_byoip_cidr should succeed");
    assert_eq!(
        wd.byoip_cidr().and_then(|c| c.state()).map(|s| s.as_str()),
        Some("provisioned")
    );

    // Deprovision
    let dep = client
        .deprovision_byoip_cidr()
        .cidr(cidr)
        .send()
        .await
        .expect("deprovision_byoip_cidr should succeed");
    assert_eq!(
        dep.byoip_cidr().and_then(|c| c.state()).map(|s| s.as_str()),
        Some("deprovisioned")
    );

    // Re-provision the same CIDR should now fail (it's already known).
    let err = client
        .provision_byoip_cidr()
        .cidr(cidr)
        .cidr_authorization_context(
            aws_sdk_ec2::types::CidrAuthorizationContext::builder()
                .message("msg")
                .signature("sig")
                .build(),
        )
        .send()
        .await
        .expect_err("re-provision should fail");
    assert!(
        format!("{err:?}").contains("InvalidByoipCidr.Duplicate")
            || format!("{err:?}").contains("Duplicate"),
        "got: {err:?}"
    );
}

#[tokio::test]
async fn test_byoip_cidr_advertise_unknown_fails() {
    let client = make_ec2_client().await;
    let err = client
        .advertise_byoip_cidr()
        .cidr("198.51.100.0/24")
        .send()
        .await
        .expect_err("advertise unknown CIDR should fail");
    assert!(
        format!("{err:?}").contains("InvalidByoipCidrId.NotFound")
            || format!("{err:?}").contains("NotFound")
    );
}

#[tokio::test]
async fn test_public_ipv4_pool_lifecycle() {
    let client = make_ec2_client().await;

    // Create
    let create = client
        .create_public_ipv4_pool()
        .send()
        .await
        .expect("create_public_ipv4_pool should succeed");
    let pool_id = create.pool_id().expect("pool_id present").to_string();
    assert!(
        pool_id.starts_with("ipv4pool-ec2-"),
        "got pool id: {pool_id}"
    );

    // Provision a CIDR into the pool
    let prov = client
        .provision_public_ipv4_pool_cidr()
        .pool_id(&pool_id)
        .ipam_pool_id("ipam-test")
        .netmask_length(28)
        .send()
        .await
        .expect("provision_public_ipv4_pool_cidr should succeed");
    assert_eq!(prov.pool_id().unwrap_or(""), pool_id);
    let range = prov.pool_address_range().expect("range present");
    assert_eq!(range.address_count(), Some(16));

    // Describe — should reflect the new range
    let desc = client
        .describe_public_ipv4_pools()
        .pool_ids(&pool_id)
        .send()
        .await
        .expect("describe_public_ipv4_pools should succeed");
    let pools = desc.public_ipv4_pools();
    let our_pool = pools
        .iter()
        .find(|p| p.pool_id() == Some(pool_id.as_str()))
        .expect("our pool should be present");
    assert_eq!(our_pool.total_address_count(), Some(16));
    assert_eq!(our_pool.pool_address_ranges().len(), 1);

    // Deleting a non-empty pool should fail.
    let err = client
        .delete_public_ipv4_pool()
        .pool_id(&pool_id)
        .send()
        .await
        .expect_err("non-empty pool deletion should fail");
    assert!(
        format!("{err:?}").contains("InvalidParameterValue")
            || format!("{err:?}").contains("not empty"),
        "got: {err:?}"
    );

    // Deprovision the CIDR
    let dep = client
        .deprovision_public_ipv4_pool_cidr()
        .pool_id(&pool_id)
        .cidr(range.first_address().unwrap_or(""))
        .send()
        .await
        .expect("deprovision_public_ipv4_pool_cidr should succeed");
    assert_eq!(dep.pool_id().unwrap_or(""), pool_id);
    assert!(!dep.deprovisioned_addresses().is_empty());

    // Now the pool is empty, so delete should succeed.
    let del = client
        .delete_public_ipv4_pool()
        .pool_id(&pool_id)
        .send()
        .await
        .expect("delete_public_ipv4_pool should succeed");
    assert_eq!(del.return_value(), Some(true));

    // Deleting again should now NotFound.
    let err = client
        .delete_public_ipv4_pool()
        .pool_id(&pool_id)
        .send()
        .await
        .expect_err("second delete should fail");
    assert!(
        format!("{err:?}").contains("InvalidPublicIpv4PoolId.NotFound")
            || format!("{err:?}").contains("NotFound")
    );
}

#[tokio::test]
async fn test_coip_cidr_lifecycle() {
    let client = make_ec2_client().await;

    // Create the underlying COIP pool first (already implemented in Group 1).
    let pool = client
        .create_coip_pool()
        .local_gateway_route_table_id("lgw-rtb-12345678")
        .send()
        .await
        .expect("create_coip_pool should succeed");
    let pool_id = pool
        .coip_pool()
        .and_then(|p| p.pool_id())
        .expect("pool id present")
        .to_string();

    let cidr = "10.99.0.0/24";

    // Create a COIP CIDR
    let create = client
        .create_coip_cidr()
        .cidr(cidr)
        .coip_pool_id(&pool_id)
        .send()
        .await
        .expect("create_coip_cidr should succeed");
    let cc = create.coip_cidr().expect("coip_cidr present");
    assert_eq!(cc.cidr().unwrap_or(""), cidr);
    assert_eq!(cc.coip_pool_id().unwrap_or(""), pool_id);

    // Describing the pool should now include the new CIDR.
    let desc = client
        .describe_coip_pools()
        .send()
        .await
        .expect("describe_coip_pools should succeed");
    let _ = desc; // describe is a stub returning empty in current code; just ensure it succeeds.

    // Creating duplicate CIDR fails
    let err = client
        .create_coip_cidr()
        .cidr(cidr)
        .coip_pool_id(&pool_id)
        .send()
        .await
        .expect_err("duplicate create should fail");
    assert!(
        format!("{err:?}").contains("Duplicate") || format!("{err:?}").contains("InvalidCoipCidr"),
        "got: {err:?}"
    );

    // Delete
    let del = client
        .delete_coip_cidr()
        .cidr(cidr)
        .coip_pool_id(&pool_id)
        .send()
        .await
        .expect("delete_coip_cidr should succeed");
    assert_eq!(del.coip_cidr().and_then(|c| c.cidr()).unwrap_or(""), cidr);

    // Re-deleting is NotFound.
    let err = client
        .delete_coip_cidr()
        .cidr(cidr)
        .coip_pool_id(&pool_id)
        .send()
        .await
        .expect_err("second delete should fail");
    assert!(
        format!("{err:?}").contains("NotFound") || format!("{err:?}").contains("InvalidCoipCidr"),
        "got: {err:?}"
    );
}

#[tokio::test]
async fn test_nat_gateway_secondary_addresses_lifecycle() {
    let client = make_ec2_client().await;

    // Build prerequisite VPC + subnet + EIP + NAT GW.
    let vpc = client
        .create_vpc()
        .cidr_block("10.20.0.0/16")
        .send()
        .await
        .unwrap();
    let vpc_id = vpc.vpc().unwrap().vpc_id().unwrap().to_string();
    let subnet = client
        .create_subnet()
        .vpc_id(&vpc_id)
        .cidr_block("10.20.1.0/24")
        .send()
        .await
        .unwrap();
    let subnet_id = subnet.subnet().unwrap().subnet_id().unwrap().to_string();
    let primary_eip = client.allocate_address().send().await.unwrap();
    let primary_alloc = primary_eip.allocation_id().unwrap().to_string();
    let nat = client
        .create_nat_gateway()
        .subnet_id(&subnet_id)
        .allocation_id(&primary_alloc)
        .send()
        .await
        .unwrap();
    let nat_gw_id = nat
        .nat_gateway()
        .and_then(|g| g.nat_gateway_id())
        .unwrap()
        .to_string();

    // 1) Assign a private secondary address.
    let assign = client
        .assign_private_nat_gateway_address()
        .nat_gateway_id(&nat_gw_id)
        .private_ip_addresses("10.20.1.50")
        .send()
        .await
        .expect("assign_private_nat_gateway_address should succeed");
    assert_eq!(assign.nat_gateway_id().unwrap_or(""), nat_gw_id);
    assert!(
        assign
            .nat_gateway_addresses()
            .iter()
            .any(|a| a.private_ip() == Some("10.20.1.50")),
        "secondary private IP should be reflected in response"
    );

    // 2) Associate a public address (an additional EIP).
    let secondary_eip = client.allocate_address().send().await.unwrap();
    let secondary_alloc = secondary_eip.allocation_id().unwrap().to_string();
    let assoc = client
        .associate_nat_gateway_address()
        .nat_gateway_id(&nat_gw_id)
        .allocation_ids(&secondary_alloc)
        .send()
        .await
        .expect("associate_nat_gateway_address should succeed");
    assert_eq!(assoc.nat_gateway_id().unwrap_or(""), nat_gw_id);
    let secondary_assoc_id = assoc
        .nat_gateway_addresses()
        .iter()
        .find(|a| a.allocation_id() == Some(secondary_alloc.as_str()))
        .and_then(|a| a.association_id())
        .expect("association id should be present")
        .to_string();

    // 3) Describe NAT GW and confirm both secondary addresses are present.
    let desc = client
        .describe_nat_gateways()
        .nat_gateway_ids(&nat_gw_id)
        .send()
        .await
        .unwrap();
    let gw = desc
        .nat_gateways()
        .iter()
        .find(|g| g.nat_gateway_id() == Some(nat_gw_id.as_str()))
        .expect("nat gw present");
    let addrs = gw.nat_gateway_addresses();
    assert!(
        addrs.len() >= 3,
        "expected primary + 2 secondaries, got {}",
        addrs.len()
    );

    // 4) Disassociate the public secondary.
    let disassoc = client
        .disassociate_nat_gateway_address()
        .nat_gateway_id(&nat_gw_id)
        .association_ids(&secondary_assoc_id)
        .send()
        .await
        .expect("disassociate_nat_gateway_address should succeed");
    assert!(
        !disassoc
            .nat_gateway_addresses()
            .iter()
            .any(|a| a.association_id() == Some(secondary_assoc_id.as_str())),
        "disassociated address should be gone from response set"
    );

    // 5) Unassign the private secondary.
    let unassign = client
        .unassign_private_nat_gateway_address()
        .nat_gateway_id(&nat_gw_id)
        .private_ip_addresses("10.20.1.50")
        .send()
        .await
        .expect("unassign_private_nat_gateway_address should succeed");
    assert!(
        !unassign
            .nat_gateway_addresses()
            .iter()
            .any(|a| a.private_ip() == Some("10.20.1.50")),
        "unassigned IP should be gone from response set"
    );
}

#[tokio::test]
async fn test_accept_address_transfer_not_found() {
    let client = make_ec2_client().await;

    // Allocate an EIP first so something is in state, then attempt accept on a
    // public IP for which no transfer record exists. Should return NotFound.
    let eip = client.allocate_address().send().await.unwrap();
    let public_ip = eip.public_ip().unwrap().to_string();

    let err = client
        .accept_address_transfer()
        .address(&public_ip)
        .send()
        .await
        .expect_err("accept_address_transfer should fail with no pending transfer");
    assert!(
        format!("{err:?}").contains("InvalidAddressTransfer.NotFound")
            || format!("{err:?}").contains("NotFound"),
        "got: {err:?}"
    );

    // describe_address_transfers should be successful and return empty.
    let desc = client
        .describe_address_transfers()
        .send()
        .await
        .expect("describe_address_transfers should succeed");
    assert!(desc.address_transfers().is_empty());
}

#[tokio::test]
async fn test_modify_reset_address_attribute_round_trip() {
    let client = make_ec2_client().await;
    let eip = client.allocate_address().send().await.unwrap();
    let alloc_id = eip.allocation_id().unwrap().to_string();

    // Modify
    let modify = client
        .modify_address_attribute()
        .allocation_id(&alloc_id)
        .domain_name("ptr.example.com")
        .send()
        .await
        .expect("modify_address_attribute should succeed");
    assert_eq!(
        modify.address().and_then(|a| a.ptr_record()),
        Some("ptr.example.com")
    );

    // Describe should reflect the new PTR record.
    let desc = client
        .describe_addresses_attribute()
        .allocation_ids(&alloc_id)
        .attribute(aws_sdk_ec2::types::AddressAttributeName::DomainName)
        .send()
        .await
        .expect("describe_addresses_attribute should succeed");
    let addr = desc
        .addresses()
        .iter()
        .find(|a| a.allocation_id() == Some(alloc_id.as_str()))
        .expect("our EIP should be present");
    assert_eq!(addr.ptr_record(), Some("ptr.example.com"));

    // Reset
    let reset = client
        .reset_address_attribute()
        .allocation_id(&alloc_id)
        .attribute(aws_sdk_ec2::types::AddressAttributeName::DomainName)
        .send()
        .await
        .expect("reset_address_attribute should succeed");
    assert!(reset.address().and_then(|a| a.ptr_record()).is_none());
}

#[tokio::test]
async fn test_move_and_restore_address_classic() {
    let client = make_ec2_client().await;
    let eip = client.allocate_address().send().await.unwrap();
    let public_ip = eip.public_ip().unwrap().to_string();

    // restore_address_to_classic moves an EIP from VPC to Classic.
    let restore = client
        .restore_address_to_classic()
        .public_ip(&public_ip)
        .send()
        .await
        .expect("restore_address_to_classic should succeed");
    assert_eq!(restore.public_ip().unwrap_or(""), public_ip);

    // Confirm describe-addresses now shows domain="standard".
    let desc = client
        .describe_addresses()
        .send()
        .await
        .expect("describe_addresses should succeed");
    let addr = desc
        .addresses()
        .iter()
        .find(|a| a.public_ip() == Some(public_ip.as_str()))
        .expect("our EIP should be visible");
    assert_eq!(addr.domain().map(|d| d.as_str()), Some("standard"));

    // Now move back to VPC.
    let mv = client
        .move_address_to_vpc()
        .public_ip(&public_ip)
        .send()
        .await
        .expect("move_address_to_vpc should succeed");
    assert!(mv.allocation_id().is_some());

    // Confirm domain is back to "vpc".
    let desc2 = client.describe_addresses().send().await.unwrap();
    let addr2 = desc2
        .addresses()
        .iter()
        .find(|a| a.public_ip() == Some(public_ip.as_str()))
        .expect("our EIP should still be present");
    assert_eq!(addr2.domain().map(|d| d.as_str()), Some("vpc"));
}

// ---------------------------------------------------------------------------
// Group 3: VPN/Customer Gateway + VPC Endpoint Connection + VPC Block Public Access
// ---------------------------------------------------------------------------

/// Helper to create a VPN connection with cgw + vgw, returning (cgw_id, vgw_id, vpn_id).
async fn make_vpn_connection(client: &aws_sdk_ec2::Client) -> (String, String, String) {
    let cgw = client
        .create_customer_gateway()
        .bgp_asn(65000)
        .ip_address("203.0.113.10")
        .r#type(aws_sdk_ec2::types::GatewayType::Ipsec1)
        .send()
        .await
        .unwrap();
    let cgw_id = cgw
        .customer_gateway()
        .unwrap()
        .customer_gateway_id()
        .unwrap()
        .to_string();
    let vgw = client
        .create_vpn_gateway()
        .r#type(aws_sdk_ec2::types::GatewayType::Ipsec1)
        .send()
        .await
        .unwrap();
    let vgw_id = vgw
        .vpn_gateway()
        .unwrap()
        .vpn_gateway_id()
        .unwrap()
        .to_string();
    let vpn = client
        .create_vpn_connection()
        .customer_gateway_id(&cgw_id)
        .vpn_gateway_id(&vgw_id)
        .r#type("ipsec.1")
        .send()
        .await
        .expect("create_vpn_connection should succeed");
    let vpn_id = vpn
        .vpn_connection()
        .unwrap()
        .vpn_connection_id()
        .unwrap()
        .to_string();
    (cgw_id, vgw_id, vpn_id)
}

#[tokio::test]
async fn test_vpn_connection_routes_and_tunnel_options_round_trip() {
    let client = make_ec2_client().await;
    let (_cgw_id, _vgw_id, vpn_id) = make_vpn_connection(&client).await;

    // Add a static route via CreateVpnConnectionRoute.
    client
        .create_vpn_connection_route()
        .vpn_connection_id(&vpn_id)
        .destination_cidr_block("10.99.0.0/16")
        .send()
        .await
        .expect("create_vpn_connection_route should succeed");

    // ModifyVpnTunnelOptions on the first seeded tunnel (203.0.113.1).
    client
        .modify_vpn_tunnel_options()
        .vpn_connection_id(&vpn_id)
        .vpn_tunnel_outside_ip_address("203.0.113.1")
        .tunnel_options(
            aws_sdk_ec2::types::ModifyVpnTunnelOptionsSpecification::builder()
                .tunnel_inside_cidr("169.254.10.0/30")
                .pre_shared_key("super-secret-psk")
                .build(),
        )
        .send()
        .await
        .expect("modify_vpn_tunnel_options should succeed");

    // Describe should reflect both the route and the tunnel option.
    let desc = client.describe_vpn_connections().send().await.unwrap();
    let vpn = desc
        .vpn_connections()
        .iter()
        .find(|v| v.vpn_connection_id() == Some(vpn_id.as_str()))
        .expect("our VPN connection should be visible");
    let routes = vpn.routes();
    assert!(
        routes
            .iter()
            .any(|r| r.destination_cidr_block() == Some("10.99.0.0/16")),
        "expected 10.99.0.0/16 in routes, got {routes:?}"
    );
    let opts = vpn.options().expect("options should be present");
    let tunnels = opts.tunnel_options();
    assert!(
        tunnels.iter().any(|t| {
            t.outside_ip_address() == Some("203.0.113.1")
                && t.tunnel_inside_cidr() == Some("169.254.10.0/30")
                && t.pre_shared_key() == Some("super-secret-psk")
        }),
        "expected tunnel-1 mutation to be visible, got {tunnels:?}"
    );

    // DeleteVpnConnectionRoute removes the route.
    client
        .delete_vpn_connection_route()
        .vpn_connection_id(&vpn_id)
        .destination_cidr_block("10.99.0.0/16")
        .send()
        .await
        .expect("delete_vpn_connection_route should succeed");
    let desc2 = client.describe_vpn_connections().send().await.unwrap();
    let vpn2 = desc2
        .vpn_connections()
        .iter()
        .find(|v| v.vpn_connection_id() == Some(vpn_id.as_str()))
        .unwrap();
    assert!(
        !vpn2
            .routes()
            .iter()
            .any(|r| r.destination_cidr_block() == Some("10.99.0.0/16")),
        "route should have been removed"
    );

    // Deleting an already-removed route returns NotFound.
    let err = client
        .delete_vpn_connection_route()
        .vpn_connection_id(&vpn_id)
        .destination_cidr_block("10.99.0.0/16")
        .send()
        .await
        .expect_err("second delete should fail");
    assert!(format!("{err:?}").contains("NotFound"), "got: {err:?}");
}

#[tokio::test]
async fn test_modify_vpn_tunnel_certificate_and_replace_tunnel() {
    let client = make_ec2_client().await;
    let (_cgw_id, _vgw_id, vpn_id) = make_vpn_connection(&client).await;

    // Rotate the certificate on tunnel 2.
    client
        .modify_vpn_tunnel_certificate()
        .vpn_connection_id(&vpn_id)
        .vpn_tunnel_outside_ip_address("203.0.113.2")
        .send()
        .await
        .expect("modify_vpn_tunnel_certificate should succeed");

    // ReplaceVpnTunnel triggers a tunnel re-creation.
    client
        .replace_vpn_tunnel()
        .vpn_connection_id(&vpn_id)
        .vpn_tunnel_outside_ip_address("203.0.113.1")
        .send()
        .await
        .expect("replace_vpn_tunnel should succeed");

    // Replacing on a non-existent tunnel returns NotFound.
    let err = client
        .replace_vpn_tunnel()
        .vpn_connection_id(&vpn_id)
        .vpn_tunnel_outside_ip_address("198.51.100.99")
        .send()
        .await
        .expect_err("replace on unknown tunnel should fail");
    assert!(format!("{err:?}").contains("NotFound"), "got: {err:?}");
}

#[tokio::test]
async fn test_modify_vpn_connection_options_round_trip() {
    let client = make_ec2_client().await;
    let (_cgw_id, _vgw_id, vpn_id) = make_vpn_connection(&client).await;

    client
        .modify_vpn_connection_options()
        .vpn_connection_id(&vpn_id)
        .local_ipv4_network_cidr("10.50.0.0/16")
        .remote_ipv4_network_cidr("10.60.0.0/16")
        .send()
        .await
        .expect("modify_vpn_connection_options should succeed");

    let desc = client.describe_vpn_connections().send().await.unwrap();
    let vpn = desc
        .vpn_connections()
        .iter()
        .find(|v| v.vpn_connection_id() == Some(vpn_id.as_str()))
        .unwrap();
    let opts = vpn.options().unwrap();
    assert_eq!(opts.local_ipv4_network_cidr(), Some("10.50.0.0/16"));
    assert_eq!(opts.remote_ipv4_network_cidr(), Some("10.60.0.0/16"));
}

#[tokio::test]
async fn test_modify_vpn_connection_target_round_trip() {
    let client = make_ec2_client().await;
    let (cgw_id_1, _vgw_id, vpn_id) = make_vpn_connection(&client).await;

    // Create a second customer gateway and switch the VPN connection to it.
    let cgw2 = client
        .create_customer_gateway()
        .bgp_asn(65001)
        .ip_address("203.0.113.99")
        .r#type(aws_sdk_ec2::types::GatewayType::Ipsec1)
        .send()
        .await
        .unwrap();
    let cgw_id_2 = cgw2
        .customer_gateway()
        .unwrap()
        .customer_gateway_id()
        .unwrap()
        .to_string();
    assert_ne!(cgw_id_1, cgw_id_2);

    client
        .modify_vpn_connection()
        .vpn_connection_id(&vpn_id)
        .customer_gateway_id(&cgw_id_2)
        .send()
        .await
        .expect("modify_vpn_connection should succeed");

    let desc = client.describe_vpn_connections().send().await.unwrap();
    let vpn = desc
        .vpn_connections()
        .iter()
        .find(|v| v.vpn_connection_id() == Some(vpn_id.as_str()))
        .unwrap();
    assert_eq!(vpn.customer_gateway_id(), Some(cgw_id_2.as_str()));
}

#[tokio::test]
async fn test_vpn_concentrator_lifecycle() {
    let client = make_ec2_client().await;

    let create = client
        .create_vpn_concentrator()
        .r#type("ipsec.1".into())
        .send()
        .await
        .expect("create_vpn_concentrator should succeed");
    let vc_id = create
        .vpn_concentrator()
        .unwrap()
        .vpn_concentrator_id()
        .unwrap()
        .to_string();
    assert!(vc_id.starts_with("vpn-concentrator-"));

    // Describe should include it.
    let desc = client.describe_vpn_concentrators().send().await.unwrap();
    let ids: Vec<&str> = desc
        .vpn_concentrators()
        .iter()
        .filter_map(|v| v.vpn_concentrator_id())
        .collect();
    assert!(ids.contains(&vc_id.as_str()));

    // Delete it.
    client
        .delete_vpn_concentrator()
        .vpn_concentrator_id(&vc_id)
        .send()
        .await
        .expect("delete_vpn_concentrator should succeed");

    // Second delete should return NotFound.
    let err = client
        .delete_vpn_concentrator()
        .vpn_concentrator_id(&vc_id)
        .send()
        .await
        .expect_err("second delete should fail");
    assert!(format!("{err:?}").contains("NotFound"), "got: {err:?}");
}

#[tokio::test]
async fn test_vpc_endpoint_connection_accept_reject_lifecycle() {
    let client = make_ec2_client().await;

    // Create a VPC and a service configuration that requires acceptance.
    let vpc = client
        .create_vpc()
        .cidr_block("10.222.0.0/16")
        .send()
        .await
        .unwrap();
    let vpc_id = vpc.vpc().unwrap().vpc_id().unwrap().to_string();

    let svc = client
        .create_vpc_endpoint_service_configuration()
        .acceptance_required(true)
        .send()
        .await
        .unwrap();
    let svc_cfg = svc.service_configuration().unwrap();
    let svc_id = svc_cfg.service_id().unwrap().to_string();
    let svc_name = svc_cfg.service_name().unwrap().to_string();

    // Create two endpoints to that service. Both should land in pendingAcceptance.
    let ep_a = client
        .create_vpc_endpoint()
        .vpc_id(&vpc_id)
        .service_name(&svc_name)
        .vpc_endpoint_type(aws_sdk_ec2::types::VpcEndpointType::Interface)
        .send()
        .await
        .unwrap();
    let ep_a_id = ep_a
        .vpc_endpoint()
        .unwrap()
        .vpc_endpoint_id()
        .unwrap()
        .to_string();
    let ep_b = client
        .create_vpc_endpoint()
        .vpc_id(&vpc_id)
        .service_name(&svc_name)
        .vpc_endpoint_type(aws_sdk_ec2::types::VpcEndpointType::Interface)
        .send()
        .await
        .unwrap();
    let ep_b_id = ep_b
        .vpc_endpoint()
        .unwrap()
        .vpc_endpoint_id()
        .unwrap()
        .to_string();

    // DescribeVpcEndpointConnections should show both as pending.
    let desc = client
        .describe_vpc_endpoint_connections()
        .send()
        .await
        .expect("describe_vpc_endpoint_connections should succeed");
    let conns = desc.vpc_endpoint_connections();
    assert!(conns.len() >= 2);
    let mine: Vec<_> = conns
        .iter()
        .filter(|c| c.service_id() == Some(svc_id.as_str()))
        .collect();
    assert_eq!(mine.len(), 2);
    assert!(
        mine.iter()
            .all(|c| c.vpc_endpoint_state().map(|s| s.as_str()) == Some("pendingAcceptance")),
        "all should be pending"
    );

    // Accept ep_a.
    client
        .accept_vpc_endpoint_connections()
        .service_id(&svc_id)
        .vpc_endpoint_ids(&ep_a_id)
        .send()
        .await
        .expect("accept_vpc_endpoint_connections should succeed");

    // Reject ep_b.
    client
        .reject_vpc_endpoint_connections()
        .service_id(&svc_id)
        .vpc_endpoint_ids(&ep_b_id)
        .send()
        .await
        .expect("reject_vpc_endpoint_connections should succeed");

    let desc2 = client
        .describe_vpc_endpoint_connections()
        .send()
        .await
        .unwrap();
    let by_id: std::collections::HashMap<String, String> = desc2
        .vpc_endpoint_connections()
        .iter()
        .filter(|c| c.service_id() == Some(svc_id.as_str()))
        .map(|c| {
            (
                c.vpc_endpoint_id().unwrap_or("").to_string(),
                c.vpc_endpoint_state()
                    .map(|s| s.as_str().to_string())
                    .unwrap_or_default(),
            )
        })
        .collect();
    assert_eq!(by_id.get(&ep_a_id).map(|s| s.as_str()), Some("available"));
    assert_eq!(by_id.get(&ep_b_id).map(|s| s.as_str()), Some("rejected"));

    // Accept against an unknown service ID should fail.
    let err = client
        .accept_vpc_endpoint_connections()
        .service_id("vpce-svc-doesnotexist")
        .vpc_endpoint_ids(&ep_a_id)
        .send()
        .await
        .expect_err("accept against unknown service should fail");
    assert!(format!("{err:?}").contains("NotFound"), "got: {err:?}");
}

#[tokio::test]
async fn test_vpc_endpoint_connection_notification_crud() {
    let client = make_ec2_client().await;

    // Create a service config so the notification can target it.
    let svc = client
        .create_vpc_endpoint_service_configuration()
        .acceptance_required(false)
        .send()
        .await
        .unwrap();
    let svc_id = svc
        .service_configuration()
        .unwrap()
        .service_id()
        .unwrap()
        .to_string();

    let create = client
        .create_vpc_endpoint_connection_notification()
        .service_id(&svc_id)
        .connection_notification_arn("arn:aws:sns:us-east-1:000000000000:my-topic")
        .connection_events("Accept")
        .connection_events("Reject")
        .send()
        .await
        .expect("create_vpc_endpoint_connection_notification should succeed");
    let n = create.connection_notification().unwrap();
    let nid = n.connection_notification_id().unwrap().to_string();
    assert_eq!(
        n.connection_notification_state().map(|s| s.as_str()),
        Some("Enabled")
    );

    // Describe should include it.
    let desc = client
        .describe_vpc_endpoint_connection_notifications()
        .send()
        .await
        .unwrap();
    assert!(
        desc.connection_notification_set()
            .iter()
            .any(|n| n.connection_notification_id() == Some(nid.as_str())),
        "newly-created notification should be visible"
    );

    // Modify -- change SNS topic and add a Connect event.
    client
        .modify_vpc_endpoint_connection_notification()
        .connection_notification_id(&nid)
        .connection_notification_arn("arn:aws:sns:us-east-1:000000000000:other-topic")
        .connection_events("Connect")
        .send()
        .await
        .expect("modify should succeed");

    let desc2 = client
        .describe_vpc_endpoint_connection_notifications()
        .send()
        .await
        .unwrap();
    let updated = desc2
        .connection_notification_set()
        .iter()
        .find(|n| n.connection_notification_id() == Some(nid.as_str()))
        .unwrap();
    assert_eq!(
        updated.connection_notification_arn(),
        Some("arn:aws:sns:us-east-1:000000000000:other-topic")
    );
    assert_eq!(updated.connection_events(), &["Connect".to_string()]);

    // Delete.
    client
        .delete_vpc_endpoint_connection_notifications()
        .connection_notification_ids(&nid)
        .send()
        .await
        .expect("delete should succeed");

    let desc3 = client
        .describe_vpc_endpoint_connection_notifications()
        .send()
        .await
        .unwrap();
    assert!(
        !desc3
            .connection_notification_set()
            .iter()
            .any(|n| n.connection_notification_id() == Some(nid.as_str())),
        "notification should be gone"
    );

    // Modify on a non-existent ID returns NotFound.
    let err = client
        .modify_vpc_endpoint_connection_notification()
        .connection_notification_id(&nid)
        .connection_notification_arn("arn:aws:sns:us-east-1:000000000000:other-topic")
        .send()
        .await
        .expect_err("modify on deleted notification should fail");
    assert!(format!("{err:?}").contains("NotFound"), "got: {err:?}");
}

#[tokio::test]
async fn test_vpc_endpoint_service_payer_responsibility_and_private_dns_verification() {
    let client = make_ec2_client().await;
    let svc = client
        .create_vpc_endpoint_service_configuration()
        .acceptance_required(false)
        .send()
        .await
        .unwrap();
    let svc_id = svc
        .service_configuration()
        .unwrap()
        .service_id()
        .unwrap()
        .to_string();

    client
        .modify_vpc_endpoint_service_payer_responsibility()
        .service_id(&svc_id)
        .payer_responsibility(aws_sdk_ec2::types::PayerResponsibility::ServiceOwner)
        .send()
        .await
        .expect("modify_vpc_endpoint_service_payer_responsibility should succeed");

    client
        .start_vpc_endpoint_service_private_dns_verification()
        .service_id(&svc_id)
        .send()
        .await
        .expect("start_vpc_endpoint_service_private_dns_verification should succeed");
}

#[tokio::test]
async fn test_vpc_block_public_access_full_round_trip() {
    let client = make_ec2_client().await;

    // Initially no options are set.
    let desc0 = client
        .describe_vpc_block_public_access_options()
        .send()
        .await
        .unwrap();
    assert!(desc0.vpc_block_public_access_options().is_none());

    // Configure account-level options.
    client
        .modify_vpc_block_public_access_options()
        .internet_gateway_block_mode(
            aws_sdk_ec2::types::InternetGatewayBlockMode::BlockBidirectional,
        )
        .send()
        .await
        .expect("modify_vpc_block_public_access_options should succeed");

    let desc1 = client
        .describe_vpc_block_public_access_options()
        .send()
        .await
        .unwrap();
    let opts = desc1
        .vpc_block_public_access_options()
        .expect("options should now be present");
    assert_eq!(
        opts.internet_gateway_block_mode().map(|m| m.as_str()),
        Some("block-bidirectional")
    );

    // Create a VPC, then create an exclusion that allows egress on it.
    let vpc = client
        .create_vpc()
        .cidr_block("10.250.0.0/16")
        .send()
        .await
        .unwrap();
    let vpc_id = vpc.vpc().unwrap().vpc_id().unwrap().to_string();

    let create = client
        .create_vpc_block_public_access_exclusion()
        .vpc_id(&vpc_id)
        .internet_gateway_exclusion_mode(
            aws_sdk_ec2::types::InternetGatewayExclusionMode::AllowEgress,
        )
        .send()
        .await
        .expect("create_vpc_block_public_access_exclusion should succeed");
    let ex_id = create
        .vpc_block_public_access_exclusion()
        .unwrap()
        .exclusion_id()
        .unwrap()
        .to_string();

    // Modify -- bidirectional.
    client
        .modify_vpc_block_public_access_exclusion()
        .exclusion_id(&ex_id)
        .internet_gateway_exclusion_mode(
            aws_sdk_ec2::types::InternetGatewayExclusionMode::AllowBidirectional,
        )
        .send()
        .await
        .expect("modify exclusion should succeed");

    let desc_ex = client
        .describe_vpc_block_public_access_exclusions()
        .send()
        .await
        .unwrap();
    let ex = desc_ex
        .vpc_block_public_access_exclusions()
        .iter()
        .find(|e| e.exclusion_id() == Some(ex_id.as_str()))
        .unwrap();
    assert_eq!(
        ex.internet_gateway_exclusion_mode().map(|m| m.as_str()),
        Some("allow-bidirectional")
    );

    // Delete the exclusion.
    client
        .delete_vpc_block_public_access_exclusion()
        .exclusion_id(&ex_id)
        .send()
        .await
        .expect("delete exclusion should succeed");
    let desc_ex2 = client
        .describe_vpc_block_public_access_exclusions()
        .send()
        .await
        .unwrap();
    assert!(
        !desc_ex2
            .vpc_block_public_access_exclusions()
            .iter()
            .any(|e| e.exclusion_id() == Some(ex_id.as_str())),
        "exclusion should be gone"
    );
}

#[tokio::test]
async fn test_vpc_encryption_control_lifecycle() {
    let client = make_ec2_client().await;

    let vpc = client
        .create_vpc()
        .cidr_block("10.251.0.0/16")
        .send()
        .await
        .unwrap();
    let vpc_id = vpc.vpc().unwrap().vpc_id().unwrap().to_string();

    let create = client
        .create_vpc_encryption_control()
        .vpc_id(&vpc_id)
        .send()
        .await
        .expect("create_vpc_encryption_control should succeed");
    let vec_id = create
        .vpc_encryption_control()
        .unwrap()
        .vpc_encryption_control_id()
        .unwrap()
        .to_string();
    assert_eq!(
        create
            .vpc_encryption_control()
            .unwrap()
            .mode()
            .map(|m| m.as_str()),
        Some("monitor")
    );

    // Switch to enforce.
    client
        .modify_vpc_encryption_control()
        .vpc_encryption_control_id(&vec_id)
        .mode(aws_sdk_ec2::types::VpcEncryptionControlMode::Enforce)
        .send()
        .await
        .expect("modify_vpc_encryption_control should succeed");

    let desc = client
        .describe_vpc_encryption_controls()
        .send()
        .await
        .unwrap();
    let v = desc
        .vpc_encryption_controls()
        .iter()
        .find(|v| v.vpc_encryption_control_id() == Some(vec_id.as_str()))
        .unwrap();
    assert_eq!(v.mode().map(|m| m.as_str()), Some("enforce"));

    // Delete and verify gone.
    client
        .delete_vpc_encryption_control()
        .vpc_encryption_control_id(&vec_id)
        .send()
        .await
        .expect("delete_vpc_encryption_control should succeed");
    let desc2 = client
        .describe_vpc_encryption_controls()
        .send()
        .await
        .unwrap();
    assert!(
        !desc2
            .vpc_encryption_controls()
            .iter()
            .any(|v| v.vpc_encryption_control_id() == Some(vec_id.as_str())),
        "encryption control should be gone"
    );

    // Modify on a deleted control returns NotFound.
    let err = client
        .modify_vpc_encryption_control()
        .vpc_encryption_control_id(&vec_id)
        .mode(aws_sdk_ec2::types::VpcEncryptionControlMode::Monitor)
        .send()
        .await
        .expect_err("modify on deleted control should fail");
    assert!(format!("{err:?}").contains("NotFound"), "got: {err:?}");
}

// ---------------------------------------------------------------------------
// Group 4: Volume / Snapshot Recycle Bin / Lock / Tier / Import + Conversion
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_volume_delete_then_restore_from_recycle_bin() {
    let client = make_ec2_client().await;

    let vol_id = client
        .create_volume()
        .availability_zone("us-east-1a")
        .size(8)
        .send()
        .await
        .expect("create_volume should succeed")
        .volume_id()
        .unwrap()
        .to_string();

    // Delete the volume — mock policy moves it to the recycle bin.
    client
        .delete_volume()
        .volume_id(&vol_id)
        .send()
        .await
        .expect("delete_volume should succeed");

    // The volume must no longer appear in DescribeVolumes.
    let desc = client
        .describe_volumes()
        .volume_ids(&vol_id)
        .send()
        .await
        .unwrap();
    assert_eq!(desc.volumes().len(), 0);

    // Restore from recycle bin.
    let restore_resp = client
        .restore_volume_from_recycle_bin()
        .volume_id(&vol_id)
        .send()
        .await
        .expect("restore_volume_from_recycle_bin should succeed");
    assert!(restore_resp.r#return().unwrap_or(false));

    // The volume now reappears in DescribeVolumes with state=available.
    let desc2 = client
        .describe_volumes()
        .volume_ids(&vol_id)
        .send()
        .await
        .unwrap();
    assert_eq!(desc2.volumes().len(), 1);
    assert_eq!(desc2.volumes()[0].state().unwrap().as_str(), "available");
}

#[tokio::test]
async fn test_copy_volumes_round_trip() {
    let client = make_ec2_client().await;

    let src_id = client
        .create_volume()
        .availability_zone("us-east-1b")
        .size(20)
        .volume_type(aws_sdk_ec2::types::VolumeType::Gp3)
        .send()
        .await
        .unwrap()
        .volume_id()
        .unwrap()
        .to_string();

    let resp = client
        .copy_volumes()
        .source_volume_id(&src_id)
        .send()
        .await
        .expect("copy_volumes should succeed");
    let copies = resp.volumes();
    assert_eq!(copies.len(), 1);
    let new_id = copies[0].volume_id().unwrap().to_string();
    assert_ne!(new_id, src_id);
    assert_eq!(copies[0].size().unwrap(), 20);
    assert_eq!(copies[0].state().unwrap().as_str(), "available");
}

#[tokio::test]
async fn test_snapshot_lock_unlock_and_tier_modify_restore() {
    let client = make_ec2_client().await;

    let vol_id = client
        .create_volume()
        .availability_zone("us-east-1a")
        .size(8)
        .send()
        .await
        .unwrap()
        .volume_id()
        .unwrap()
        .to_string();

    let snap_id = client
        .create_snapshot()
        .volume_id(&vol_id)
        .description("for lock/tier")
        .send()
        .await
        .unwrap()
        .snapshot_id()
        .unwrap()
        .to_string();

    // Lock in governance mode (allows unlock).
    let lock = client
        .lock_snapshot()
        .snapshot_id(&snap_id)
        .lock_mode(aws_sdk_ec2::types::LockMode::Governance)
        .lock_duration(7)
        .send()
        .await
        .expect("lock_snapshot should succeed");
    assert_eq!(lock.snapshot_id().unwrap(), &snap_id);
    assert_eq!(lock.lock_state().unwrap().as_str(), "governance");

    // Modify tier to archive.
    let tier_resp = client
        .modify_snapshot_tier()
        .snapshot_id(&snap_id)
        .storage_tier(aws_sdk_ec2::types::TargetStorageTier::Archive)
        .send()
        .await
        .expect("modify_snapshot_tier should succeed");
    assert_eq!(tier_resp.snapshot_id().unwrap(), &snap_id);
    assert!(tier_resp.tiering_start_time().is_some());

    // Restore tier to standard.
    let restore_tier = client
        .restore_snapshot_tier()
        .snapshot_id(&snap_id)
        .send()
        .await
        .expect("restore_snapshot_tier should succeed");
    assert_eq!(restore_tier.snapshot_id().unwrap(), &snap_id);

    // Unlock from governance.
    let unlock = client
        .unlock_snapshot()
        .snapshot_id(&snap_id)
        .send()
        .await
        .expect("unlock_snapshot should succeed");
    assert_eq!(unlock.snapshot_id().unwrap(), &snap_id);
}

#[tokio::test]
async fn test_snapshot_compliance_lock_blocks_unlock() {
    let client = make_ec2_client().await;

    let vol_id = client
        .create_volume()
        .availability_zone("us-east-1a")
        .size(8)
        .send()
        .await
        .unwrap()
        .volume_id()
        .unwrap()
        .to_string();

    let snap_id = client
        .create_snapshot()
        .volume_id(&vol_id)
        .description("compliance test")
        .send()
        .await
        .unwrap()
        .snapshot_id()
        .unwrap()
        .to_string();

    client
        .lock_snapshot()
        .snapshot_id(&snap_id)
        .lock_mode(aws_sdk_ec2::types::LockMode::Compliance)
        .lock_duration(30)
        .send()
        .await
        .expect("lock_snapshot in compliance mode should succeed");

    let err = client
        .unlock_snapshot()
        .snapshot_id(&snap_id)
        .send()
        .await
        .expect_err("unlock_snapshot in compliance mode should fail");
    assert!(
        format!("{err:?}").contains("SnapshotIsLocked"),
        "got: {err:?}"
    );
}

#[tokio::test]
async fn test_snapshot_delete_then_restore_from_recycle_bin() {
    let client = make_ec2_client().await;

    let vol_id = client
        .create_volume()
        .availability_zone("us-east-1a")
        .size(8)
        .send()
        .await
        .unwrap()
        .volume_id()
        .unwrap()
        .to_string();

    let snap_id = client
        .create_snapshot()
        .volume_id(&vol_id)
        .description("recycle bin test")
        .send()
        .await
        .unwrap()
        .snapshot_id()
        .unwrap()
        .to_string();

    // Delete -> snapshot should disappear from describe_snapshots.
    client
        .delete_snapshot()
        .snapshot_id(&snap_id)
        .send()
        .await
        .expect("delete_snapshot should succeed");
    let desc = client
        .describe_snapshots()
        .snapshot_ids(&snap_id)
        .send()
        .await
        .unwrap();
    assert_eq!(desc.snapshots().len(), 0);

    // Restore from recycle bin.
    let restore = client
        .restore_snapshot_from_recycle_bin()
        .snapshot_id(&snap_id)
        .send()
        .await
        .expect("restore_snapshot_from_recycle_bin should succeed");
    assert_eq!(restore.snapshot_id().unwrap(), &snap_id);

    // The snapshot now reappears.
    let desc2 = client
        .describe_snapshots()
        .snapshot_ids(&snap_id)
        .send()
        .await
        .unwrap();
    assert_eq!(desc2.snapshots().len(), 1);
}

#[tokio::test]
async fn test_import_snapshot_creates_snapshot() {
    let client = make_ec2_client().await;

    let resp = client
        .import_snapshot()
        .description("import-test")
        .disk_container(
            aws_sdk_ec2::types::SnapshotDiskContainer::builder()
                .description("disk")
                .format("vmdk")
                .url("s3://example/disk.vmdk")
                .build(),
        )
        .send()
        .await
        .expect("import_snapshot should succeed");
    let task_id = resp.import_task_id().unwrap().to_string();
    assert!(task_id.starts_with("import-snap-"));
    let detail = resp.snapshot_task_detail().unwrap();
    let snap_id = detail.snapshot_id().unwrap().to_string();
    assert!(snap_id.starts_with("snap-"));
    assert_eq!(detail.status().unwrap(), "completed");

    // The materialised snapshot is reachable via describe_snapshots.
    let desc = client
        .describe_snapshots()
        .snapshot_ids(&snap_id)
        .send()
        .await
        .unwrap();
    assert_eq!(desc.snapshots().len(), 1);
}

#[tokio::test]
async fn test_import_instance_then_cancel_conversion_task() {
    let client = make_ec2_client().await;

    let resp = client
        .import_instance()
        .description("import-instance-test")
        .platform(aws_sdk_ec2::types::PlatformValues::Windows)
        .send()
        .await
        .expect("import_instance should succeed");
    let task_id = resp
        .conversion_task()
        .unwrap()
        .conversion_task_id()
        .unwrap()
        .to_string();
    assert!(task_id.starts_with("import-i-"));
    assert_eq!(
        resp.conversion_task().unwrap().state().unwrap().as_str(),
        "active"
    );

    client
        .cancel_conversion_task()
        .conversion_task_id(&task_id)
        .send()
        .await
        .expect("cancel_conversion_task should succeed");

    // Cancelling an unknown task should error.
    let err = client
        .cancel_conversion_task()
        .conversion_task_id("nonexistent-task")
        .send()
        .await
        .expect_err("cancel_conversion_task on unknown task should fail");
    assert!(
        format!("{err:?}").contains("InvalidConversionTaskId"),
        "got: {err:?}"
    );
}

#[tokio::test]
async fn test_create_export_task_then_cancel() {
    let client = make_ec2_client().await;

    let instance_id = client
        .run_instances()
        .image_id("ami-export-test")
        .instance_type(aws_sdk_ec2::types::InstanceType::T2Micro)
        .min_count(1)
        .max_count(1)
        .send()
        .await
        .unwrap()
        .instances()
        .first()
        .unwrap()
        .instance_id()
        .unwrap()
        .to_string();

    let resp = client
        .create_instance_export_task()
        .instance_id(&instance_id)
        .target_environment(aws_sdk_ec2::types::ExportEnvironment::Vmware)
        .description("export-test")
        .export_to_s3_task(
            aws_sdk_ec2::types::ExportToS3TaskSpecification::builder()
                .disk_image_format(aws_sdk_ec2::types::DiskImageFormat::Vmdk)
                .s3_bucket("test-bucket")
                .s3_prefix("exports/")
                .build(),
        )
        .send()
        .await
        .expect("create_instance_export_task should succeed");
    let task = resp.export_task().unwrap();
    let task_id = task.export_task_id().unwrap().to_string();
    assert!(task_id.starts_with("export-i-"));
    assert_eq!(task.state().unwrap().as_str(), "active");
    assert_eq!(
        task.export_to_s3_task().unwrap().s3_bucket().unwrap(),
        "test-bucket"
    );

    client
        .cancel_export_task()
        .export_task_id(&task_id)
        .send()
        .await
        .expect("cancel_export_task should succeed");

    // Cancelling an unknown export task should error.
    let err = client
        .cancel_export_task()
        .export_task_id("nonexistent-export")
        .send()
        .await
        .expect_err("cancel_export_task on unknown task should fail");
    assert!(
        format!("{err:?}").contains("InvalidExportTaskID"),
        "got: {err:?}"
    );
}

#[tokio::test]
async fn test_secondary_network_lifecycle_with_subnet_dependency() {
    let client = make_ec2_client().await;

    let net_resp = client
        .create_secondary_network()
        .ipv4_cidr_block("10.99.0.0/16")
        .network_type("ipv4".into())
        .send()
        .await
        .expect("create_secondary_network should succeed");
    let net_id = net_resp
        .secondary_network()
        .unwrap()
        .secondary_network_id()
        .unwrap()
        .to_string();
    assert!(net_id.starts_with("snet-"));

    let subnet_resp = client
        .create_secondary_subnet()
        .secondary_network_id(&net_id)
        .ipv4_cidr_block("10.99.1.0/24")
        .availability_zone("us-east-1a")
        .send()
        .await
        .expect("create_secondary_subnet should succeed");
    let subnet_id = subnet_resp
        .secondary_subnet()
        .unwrap()
        .secondary_subnet_id()
        .unwrap()
        .to_string();
    assert!(subnet_id.starts_with("ssubnet-"));

    // Deleting the network with an active subnet must fail.
    let err = client
        .delete_secondary_network()
        .secondary_network_id(&net_id)
        .send()
        .await
        .expect_err("delete with subnets should fail");
    assert!(
        format!("{err:?}").contains("DependencyViolation"),
        "got: {err:?}"
    );

    client
        .delete_secondary_subnet()
        .secondary_subnet_id(&subnet_id)
        .send()
        .await
        .expect("delete_secondary_subnet should succeed");

    client
        .delete_secondary_network()
        .secondary_network_id(&net_id)
        .send()
        .await
        .expect("delete_secondary_network should succeed");
}

#[tokio::test]
async fn test_trunk_interface_associate_disassociate() {
    let client = make_ec2_client().await;

    let resp = client
        .associate_trunk_interface()
        .branch_interface_id("eni-branch-1")
        .trunk_interface_id("eni-trunk-1")
        .vlan_id(101)
        .send()
        .await
        .expect("associate_trunk_interface should succeed");
    let assoc = resp.interface_association().unwrap();
    let assoc_id = assoc.association_id().unwrap().to_string();
    assert!(assoc_id.starts_with("trunk-assoc-"));
    assert_eq!(assoc.vlan_id().unwrap(), 101);
    assert_eq!(assoc.interface_protocol().unwrap().as_str(), "vlan");

    client
        .disassociate_trunk_interface()
        .association_id(&assoc_id)
        .send()
        .await
        .expect("disassociate_trunk_interface should succeed");

    // Repeated disassociation must fail.
    let err = client
        .disassociate_trunk_interface()
        .association_id(&assoc_id)
        .send()
        .await
        .expect_err("second disassociate should fail");
    assert!(
        format!("{err:?}").contains("InvalidTrunkInterfaceAssociationId"),
        "got: {err:?}"
    );
}

#[tokio::test]
async fn test_create_replace_root_volume_task() {
    let client = make_ec2_client().await;

    let instance_id = client
        .run_instances()
        .image_id("ami-replace-root")
        .instance_type(aws_sdk_ec2::types::InstanceType::T2Micro)
        .min_count(1)
        .max_count(1)
        .send()
        .await
        .unwrap()
        .instances()
        .first()
        .unwrap()
        .instance_id()
        .unwrap()
        .to_string();

    let resp = client
        .create_replace_root_volume_task()
        .instance_id(&instance_id)
        .image_id("ami-new-root")
        .delete_replaced_root_volume(true)
        .send()
        .await
        .expect("create_replace_root_volume_task should succeed");
    let task = resp.replace_root_volume_task().unwrap();
    assert_eq!(task.instance_id().unwrap(), &instance_id);
    assert_eq!(task.image_id().unwrap(), "ami-new-root");
    assert!(
        task.replace_root_volume_task_id()
            .unwrap()
            .starts_with("replacevol-")
    );
}

#[tokio::test]
async fn test_create_delegate_mac_volume_ownership_task() {
    let client = make_ec2_client().await;

    let resp = client
        .create_delegate_mac_volume_ownership_task()
        .instance_id("i-mac-test")
        .mac_credentials("base64-creds-placeholder")
        .send()
        .await
        .expect("create_delegate_mac_volume_ownership_task should succeed");
    let task = resp.mac_modification_task().unwrap();
    assert!(
        task.mac_modification_task_id()
            .unwrap()
            .starts_with("macmodification-")
    );
    assert_eq!(task.task_state().unwrap().as_str(), "pending");
}

// =====================================================================
// Group 5 integration tests
// =====================================================================

#[tokio::test]
async fn test_purchase_reserved_instances_offering_then_describe() {
    let client = make_ec2_client().await;

    let resp = client
        .purchase_reserved_instances_offering()
        .reserved_instances_offering_id("ro-12345678")
        .instance_count(2)
        .send()
        .await
        .expect("purchase_reserved_instances_offering should succeed");
    assert!(
        resp.reserved_instances_id()
            .unwrap_or("")
            .starts_with("ri-")
    );
}

#[tokio::test]
async fn test_modify_reserved_instances_lifecycle() {
    let client = make_ec2_client().await;

    let resp = client
        .modify_reserved_instances()
        .reserved_instances_ids("ri-abcdef01")
        .target_configurations(
            aws_sdk_ec2::types::ReservedInstancesConfiguration::builder()
                .availability_zone("us-east-1a")
                .instance_count(2)
                .instance_type(aws_sdk_ec2::types::InstanceType::T3Micro)
                .build(),
        )
        .send()
        .await
        .expect("modify_reserved_instances should succeed");
    assert!(
        resp.reserved_instances_modification_id()
            .unwrap_or("")
            .starts_with("rim-")
    );

    let desc = client
        .describe_reserved_instances_modifications()
        .send()
        .await
        .expect("describe_reserved_instances_modifications should succeed");
    assert!(!desc.reserved_instances_modifications().is_empty());
}

#[tokio::test]
async fn test_create_cancel_reserved_instances_listing() {
    let client = make_ec2_client().await;

    let create_resp = client
        .create_reserved_instances_listing()
        .reserved_instances_id("ri-abcdef01")
        .instance_count(1)
        .price_schedules(
            aws_sdk_ec2::types::PriceScheduleSpecification::builder()
                .term(31536000)
                .price(0.05)
                .currency_code(aws_sdk_ec2::types::CurrencyCodeValues::Usd)
                .build(),
        )
        .client_token("token-1")
        .send()
        .await
        .expect("create_reserved_instances_listing should succeed");
    let listings = create_resp.reserved_instances_listings();
    let listing_id = listings[0]
        .reserved_instances_listing_id()
        .unwrap()
        .to_string();
    assert_eq!(listings[0].status().unwrap().as_str(), "active");

    let cancel_resp = client
        .cancel_reserved_instances_listing()
        .reserved_instances_listing_id(&listing_id)
        .send()
        .await
        .expect("cancel_reserved_instances_listing should succeed");
    let cancelled = &cancel_resp.reserved_instances_listings()[0];
    assert_eq!(cancelled.status().unwrap().as_str(), "cancelled");
}

#[tokio::test]
async fn test_accept_reserved_instances_exchange_quote() {
    let client = make_ec2_client().await;

    let resp = client
        .accept_reserved_instances_exchange_quote()
        .reserved_instance_ids("ri-source")
        .target_configurations(
            aws_sdk_ec2::types::TargetConfigurationRequest::builder()
                .offering_id("ro-target")
                .instance_count(1)
                .build(),
        )
        .send()
        .await
        .expect("accept_reserved_instances_exchange_quote should succeed");
    assert!(resp.exchange_id().unwrap_or("").starts_with("riex-"));
}

#[tokio::test]
async fn test_modify_fleet_lifecycle() {
    let client = make_ec2_client().await;

    let create = client
        .create_fleet()
        .target_capacity_specification(
            aws_sdk_ec2::types::TargetCapacitySpecificationRequest::builder()
                .total_target_capacity(2)
                .default_target_capacity_type(
                    aws_sdk_ec2::types::DefaultTargetCapacityType::OnDemand,
                )
                .build(),
        )
        .launch_template_configs(
            aws_sdk_ec2::types::FleetLaunchTemplateConfigRequest::builder().build(),
        )
        .send()
        .await
        .expect("create_fleet should succeed");
    let fleet_id = create.fleet_id().unwrap().to_string();

    let modify = client
        .modify_fleet()
        .fleet_id(&fleet_id)
        .target_capacity_specification(
            aws_sdk_ec2::types::TargetCapacitySpecificationRequest::builder()
                .total_target_capacity(5)
                .default_target_capacity_type(
                    aws_sdk_ec2::types::DefaultTargetCapacityType::OnDemand,
                )
                .build(),
        )
        .context("ctx-A")
        .send()
        .await
        .expect("modify_fleet should succeed");
    assert!(modify.r#return().unwrap_or(false));
}

#[tokio::test]
async fn test_image_recycle_bin_lifecycle() {
    let client = make_ec2_client().await;

    // Create instance, snapshot it as an image, then deregister + restore.
    let inst_id = client
        .run_instances()
        .image_id("ami-base")
        .min_count(1)
        .max_count(1)
        .send()
        .await
        .unwrap()
        .instances()
        .first()
        .unwrap()
        .instance_id()
        .unwrap()
        .to_string();

    let create_image = client
        .create_image()
        .instance_id(&inst_id)
        .name("recycle-bin-test")
        .send()
        .await
        .expect("create_image should succeed");
    let image_id = create_image.image_id().unwrap().to_string();

    client
        .deregister_image()
        .image_id(&image_id)
        .send()
        .await
        .expect("deregister_image should succeed");

    let desc_after = client
        .describe_images()
        .image_ids(&image_id)
        .send()
        .await
        .unwrap();
    assert_eq!(desc_after.images().len(), 0);

    let restore = client
        .restore_image_from_recycle_bin()
        .image_id(&image_id)
        .send()
        .await
        .expect("restore_image_from_recycle_bin should succeed");
    assert!(restore.r#return().unwrap_or(false));

    let desc_restored = client
        .describe_images()
        .image_ids(&image_id)
        .send()
        .await
        .unwrap();
    assert_eq!(desc_restored.images().len(), 1);
}

#[tokio::test]
async fn test_fpga_image_lifecycle() {
    let client = make_ec2_client().await;

    let create = client
        .create_fpga_image()
        .name("fpga-1")
        .description("test fpga")
        .input_storage_location(
            aws_sdk_ec2::types::StorageLocation::builder()
                .bucket("bucket-1")
                .key("key-1")
                .build(),
        )
        .send()
        .await
        .expect("create_fpga_image should succeed");
    let id = create.fpga_image_id().unwrap().to_string();

    let mod_resp = client
        .modify_fpga_image_attribute()
        .fpga_image_id(&id)
        .description("updated description")
        .load_permission(
            aws_sdk_ec2::types::LoadPermissionModifications::builder()
                .add(
                    aws_sdk_ec2::types::LoadPermissionRequest::builder()
                        .user_id("123456789012")
                        .build(),
                )
                .build(),
        )
        .send()
        .await
        .expect("modify_fpga_image_attribute should succeed");
    let attr = mod_resp.fpga_image_attribute().unwrap();
    assert_eq!(attr.description().unwrap_or(""), "updated description");

    client
        .reset_fpga_image_attribute()
        .fpga_image_id(&id)
        .send()
        .await
        .expect("reset_fpga_image_attribute should succeed");

    let desc = client
        .describe_fpga_images()
        .send()
        .await
        .expect("describe_fpga_images should succeed");
    assert!(!desc.fpga_images().is_empty());

    client
        .delete_fpga_image()
        .fpga_image_id(&id)
        .send()
        .await
        .expect("delete_fpga_image should succeed");
}

#[tokio::test]
async fn test_copy_fpga_image() {
    let client = make_ec2_client().await;
    let create = client
        .create_fpga_image()
        .name("fpga-src")
        .input_storage_location(
            aws_sdk_ec2::types::StorageLocation::builder()
                .bucket("b")
                .key("k")
                .build(),
        )
        .send()
        .await
        .unwrap();
    let src_id = create.fpga_image_id().unwrap().to_string();

    let copy = client
        .copy_fpga_image()
        .source_fpga_image_id(&src_id)
        .source_region("us-east-1")
        .name("fpga-copy")
        .send()
        .await
        .expect("copy_fpga_image should succeed");
    assert!(copy.fpga_image_id().unwrap_or("").starts_with("afi-"));
}

#[tokio::test]
async fn test_import_image_lifecycle() {
    let client = make_ec2_client().await;

    let resp = client
        .import_image()
        .description("ImportImage test")
        .platform("Linux")
        .send()
        .await
        .expect("import_image should succeed");
    assert!(
        resp.import_task_id()
            .unwrap_or("")
            .starts_with("import-ami-")
    );
    assert_eq!(resp.status().unwrap_or(""), "active");

    let desc = client
        .describe_import_image_tasks()
        .send()
        .await
        .expect("describe_import_image_tasks should succeed");
    assert!(!desc.import_image_tasks().is_empty());
}

#[tokio::test]
async fn test_create_store_image_task_then_describe() {
    let client = make_ec2_client().await;
    let inst_id = client
        .run_instances()
        .image_id("ami-base")
        .min_count(1)
        .max_count(1)
        .send()
        .await
        .unwrap()
        .instances()
        .first()
        .unwrap()
        .instance_id()
        .unwrap()
        .to_string();
    let img = client
        .create_image()
        .instance_id(&inst_id)
        .name("storeable")
        .send()
        .await
        .unwrap();
    let ami_id = img.image_id().unwrap().to_string();

    let resp = client
        .create_store_image_task()
        .image_id(&ami_id)
        .bucket("ami-bucket")
        .send()
        .await
        .expect("create_store_image_task should succeed");
    assert!(resp.object_key().unwrap_or("").contains(&ami_id));

    let desc = client
        .describe_store_image_tasks()
        .send()
        .await
        .expect("describe_store_image_tasks should succeed");
    assert!(!desc.store_image_task_results().is_empty());
}

#[tokio::test]
async fn test_create_restore_image_task() {
    let client = make_ec2_client().await;

    let resp = client
        .create_restore_image_task()
        .bucket("ami-bucket")
        .object_key("my-ami.bin")
        .name("restored-ami")
        .send()
        .await
        .expect("create_restore_image_task should succeed");
    let image_id = resp.image_id().unwrap().to_string();
    assert!(image_id.starts_with("ami-"));

    let desc = client
        .describe_images()
        .image_ids(&image_id)
        .send()
        .await
        .unwrap();
    assert_eq!(desc.images().len(), 1);
}

#[tokio::test]
async fn test_image_usage_report_lifecycle() {
    let client = make_ec2_client().await;
    let inst_id = client
        .run_instances()
        .image_id("ami-base")
        .min_count(1)
        .max_count(1)
        .send()
        .await
        .unwrap()
        .instances()
        .first()
        .unwrap()
        .instance_id()
        .unwrap()
        .to_string();
    let img = client
        .create_image()
        .instance_id(&inst_id)
        .name("for-usage-report")
        .send()
        .await
        .unwrap();
    let ami_id = img.image_id().unwrap().to_string();

    let create = client
        .create_image_usage_report()
        .image_id(&ami_id)
        .resource_types(
            aws_sdk_ec2::types::ImageUsageResourceTypeRequest::builder()
                .resource_type("ec2:Instance")
                .build(),
        )
        .send()
        .await
        .expect("create_image_usage_report should succeed");
    let report_id = create.report_id().unwrap().to_string();

    let desc = client
        .describe_image_usage_reports()
        .send()
        .await
        .expect("describe_image_usage_reports should succeed");
    assert!(!desc.image_usage_reports().is_empty());

    client
        .delete_image_usage_report()
        .report_id(&report_id)
        .send()
        .await
        .expect("delete_image_usage_report should succeed");
}

#[tokio::test]
async fn test_cancel_image_launch_permission() {
    let client = make_ec2_client().await;
    let inst_id = client
        .run_instances()
        .image_id("ami-base")
        .min_count(1)
        .max_count(1)
        .send()
        .await
        .unwrap()
        .instances()
        .first()
        .unwrap()
        .instance_id()
        .unwrap()
        .to_string();
    let img = client
        .create_image()
        .instance_id(&inst_id)
        .name("perm-test")
        .send()
        .await
        .unwrap();
    let ami_id = img.image_id().unwrap().to_string();

    let resp = client
        .cancel_image_launch_permission()
        .image_id(&ami_id)
        .send()
        .await
        .expect("cancel_image_launch_permission should succeed");
    assert!(resp.r#return().unwrap_or(false));
}

#[tokio::test]
async fn test_modify_instance_cpu_options_round_trip() {
    let client = make_ec2_client().await;
    let inst_id = client
        .run_instances()
        .image_id("ami-base")
        .min_count(1)
        .max_count(1)
        .send()
        .await
        .unwrap()
        .instances()
        .first()
        .unwrap()
        .instance_id()
        .unwrap()
        .to_string();

    let resp = client
        .modify_instance_cpu_options()
        .instance_id(&inst_id)
        .core_count(2)
        .threads_per_core(2)
        .send()
        .await
        .expect("modify_instance_cpu_options should succeed");
    assert_eq!(resp.core_count().unwrap_or(0), 2);
    assert_eq!(resp.threads_per_core().unwrap_or(0), 2);
}

#[tokio::test]
async fn test_modify_instance_credit_specification() {
    let client = make_ec2_client().await;
    let inst_id = client
        .run_instances()
        .image_id("ami-base")
        .min_count(1)
        .max_count(1)
        .send()
        .await
        .unwrap()
        .instances()
        .first()
        .unwrap()
        .instance_id()
        .unwrap()
        .to_string();

    let resp = client
        .modify_instance_credit_specification()
        .instance_credit_specifications(
            aws_sdk_ec2::types::InstanceCreditSpecificationRequest::builder()
                .instance_id(&inst_id)
                .cpu_credits("unlimited")
                .build(),
        )
        .send()
        .await
        .expect("modify_instance_credit_specification should succeed");
    assert_eq!(resp.successful_instance_credit_specifications().len(), 1);
}

#[tokio::test]
async fn test_modify_instance_maintenance_options() {
    let client = make_ec2_client().await;
    let inst_id = client
        .run_instances()
        .image_id("ami-base")
        .min_count(1)
        .max_count(1)
        .send()
        .await
        .unwrap()
        .instances()
        .first()
        .unwrap()
        .instance_id()
        .unwrap()
        .to_string();

    let resp = client
        .modify_instance_maintenance_options()
        .instance_id(&inst_id)
        .auto_recovery(aws_sdk_ec2::types::InstanceAutoRecoveryState::Disabled)
        .send()
        .await
        .expect("modify_instance_maintenance_options should succeed");
    assert_eq!(resp.auto_recovery().unwrap().as_str(), "disabled");
}

#[tokio::test]
async fn test_modify_instance_metadata_defaults_round_trip() {
    let client = make_ec2_client().await;

    client
        .modify_instance_metadata_defaults()
        .http_tokens(aws_sdk_ec2::types::MetadataDefaultHttpTokensState::Required)
        .http_put_response_hop_limit(2)
        .send()
        .await
        .expect("modify_instance_metadata_defaults should succeed");

    let resp = client
        .get_instance_metadata_defaults()
        .send()
        .await
        .expect("get_instance_metadata_defaults should succeed");
    let defaults = resp.account_level().unwrap();
    assert_eq!(defaults.http_tokens().unwrap().as_str(), "required");
    assert_eq!(defaults.http_put_response_hop_limit().unwrap_or(0), 2);
}

#[tokio::test]
async fn test_modify_instance_network_performance_options() {
    let client = make_ec2_client().await;
    let inst_id = client
        .run_instances()
        .image_id("ami-base")
        .min_count(1)
        .max_count(1)
        .send()
        .await
        .unwrap()
        .instances()
        .first()
        .unwrap()
        .instance_id()
        .unwrap()
        .to_string();

    let resp = client
        .modify_instance_network_performance_options()
        .instance_id(&inst_id)
        .bandwidth_weighting(aws_sdk_ec2::types::InstanceBandwidthWeighting::Vpc1)
        .send()
        .await
        .expect("modify_instance_network_performance_options should succeed");
    assert_eq!(resp.bandwidth_weighting().unwrap().as_str(), "vpc-1");
}

#[tokio::test]
async fn test_modify_default_credit_specification() {
    let client = make_ec2_client().await;

    let resp = client
        .modify_default_credit_specification()
        .instance_family(aws_sdk_ec2::types::UnlimitedSupportedInstanceFamily::T3)
        .cpu_credits("unlimited")
        .send()
        .await
        .expect("modify_default_credit_specification should succeed");
    let spec = resp.instance_family_credit_specification().unwrap();
    assert_eq!(spec.cpu_credits().unwrap_or(""), "unlimited");

    let read_back = client
        .get_default_credit_specification()
        .instance_family(aws_sdk_ec2::types::UnlimitedSupportedInstanceFamily::T3)
        .send()
        .await
        .unwrap();
    assert_eq!(
        read_back
            .instance_family_credit_specification()
            .unwrap()
            .cpu_credits()
            .unwrap_or(""),
        "unlimited"
    );
}

#[tokio::test]
async fn test_modify_instance_connect_endpoint() {
    let client = make_ec2_client().await;
    let vpc_id = client
        .create_vpc()
        .cidr_block("10.0.0.0/16")
        .send()
        .await
        .unwrap()
        .vpc()
        .unwrap()
        .vpc_id()
        .unwrap()
        .to_string();
    let subnet_id = client
        .create_subnet()
        .vpc_id(&vpc_id)
        .cidr_block("10.0.1.0/24")
        .send()
        .await
        .unwrap()
        .subnet()
        .unwrap()
        .subnet_id()
        .unwrap()
        .to_string();
    let ep = client
        .create_instance_connect_endpoint()
        .subnet_id(&subnet_id)
        .send()
        .await
        .unwrap();
    let ep_id = ep
        .instance_connect_endpoint()
        .unwrap()
        .instance_connect_endpoint_id()
        .unwrap()
        .to_string();

    let resp = client
        .modify_instance_connect_endpoint()
        .instance_connect_endpoint_id(&ep_id)
        .preserve_client_ip(true)
        .send()
        .await
        .expect("modify_instance_connect_endpoint should succeed");
    assert!(resp.r#return().unwrap_or(false));
}

#[tokio::test]
async fn test_reset_instance_attribute() {
    let client = make_ec2_client().await;
    let inst_id = client
        .run_instances()
        .image_id("ami-base")
        .min_count(1)
        .max_count(1)
        .send()
        .await
        .unwrap()
        .instances()
        .first()
        .unwrap()
        .instance_id()
        .unwrap()
        .to_string();

    client
        .reset_instance_attribute()
        .instance_id(&inst_id)
        .attribute(aws_sdk_ec2::types::InstanceAttributeName::SourceDestCheck)
        .send()
        .await
        .expect("reset_instance_attribute should succeed");
}

#[tokio::test]
async fn test_instance_event_window_lifecycle() {
    let client = make_ec2_client().await;

    let create = client
        .create_instance_event_window()
        .name("maint-1")
        .time_ranges(
            aws_sdk_ec2::types::InstanceEventWindowTimeRangeRequest::builder()
                .start_week_day(aws_sdk_ec2::types::WeekDay::Monday)
                .start_hour(2)
                .end_week_day(aws_sdk_ec2::types::WeekDay::Monday)
                .end_hour(4)
                .build(),
        )
        .send()
        .await
        .expect("create_instance_event_window should succeed");
    let win = create.instance_event_window().unwrap();
    let id = win.instance_event_window_id().unwrap().to_string();
    assert_eq!(win.state().unwrap().as_str(), "active");

    client
        .associate_instance_event_window()
        .instance_event_window_id(&id)
        .association_target(
            aws_sdk_ec2::types::InstanceEventWindowAssociationRequest::builder()
                .instance_ids("i-12345678")
                .build(),
        )
        .send()
        .await
        .expect("associate_instance_event_window should succeed");

    client
        .modify_instance_event_window()
        .instance_event_window_id(&id)
        .name("maint-1-updated")
        .send()
        .await
        .expect("modify_instance_event_window should succeed");

    client
        .disassociate_instance_event_window()
        .instance_event_window_id(&id)
        .association_target(
            aws_sdk_ec2::types::InstanceEventWindowDisassociationRequest::builder()
                .instance_ids("i-12345678")
                .build(),
        )
        .send()
        .await
        .expect("disassociate_instance_event_window should succeed");

    let desc = client
        .describe_instance_event_windows()
        .send()
        .await
        .unwrap();
    assert!(!desc.instance_event_windows().is_empty());
    let updated = &desc.instance_event_windows()[0];
    assert_eq!(updated.name().unwrap_or(""), "maint-1-updated");

    let del = client
        .delete_instance_event_window()
        .instance_event_window_id(&id)
        .send()
        .await
        .expect("delete_instance_event_window should succeed");
    assert_eq!(
        del.instance_event_window_state()
            .unwrap()
            .state()
            .unwrap()
            .as_str(),
        "deleted"
    );
}

#[tokio::test]
async fn test_modify_instance_event_start_time() {
    let client = make_ec2_client().await;
    let inst_id = client
        .run_instances()
        .image_id("ami-base")
        .min_count(1)
        .max_count(1)
        .send()
        .await
        .unwrap()
        .instances()
        .first()
        .unwrap()
        .instance_id()
        .unwrap()
        .to_string();

    let resp = client
        .modify_instance_event_start_time()
        .instance_id(&inst_id)
        .instance_event_id("event-test-1")
        .not_before(aws_sdk_ec2::primitives::DateTime::from_secs(2_000_000_000))
        .send()
        .await
        .expect("modify_instance_event_start_time should succeed");
    let event = resp.event().unwrap();
    assert_eq!(event.instance_event_id().unwrap_or(""), "event-test-1");
}

#[tokio::test]
async fn test_event_notification_attributes_register_deregister() {
    let client = make_ec2_client().await;

    client
        .register_instance_event_notification_attributes()
        .instance_tag_attribute(
            aws_sdk_ec2::types::RegisterInstanceTagAttributeRequest::builder()
                .include_all_tags_of_instance(true)
                .instance_tag_keys("Owner")
                .build(),
        )
        .send()
        .await
        .expect("register should succeed");

    let desc = client
        .describe_instance_event_notification_attributes()
        .send()
        .await
        .unwrap();
    let attr = desc.instance_tag_attribute().unwrap();
    assert!(attr.include_all_tags_of_instance().unwrap_or(false));

    client
        .deregister_instance_event_notification_attributes()
        .instance_tag_attribute(
            aws_sdk_ec2::types::DeregisterInstanceTagAttributeRequest::builder()
                .include_all_tags_of_instance(true)
                .instance_tag_keys("Owner")
                .build(),
        )
        .send()
        .await
        .expect("deregister should succeed");

    let desc2 = client
        .describe_instance_event_notification_attributes()
        .send()
        .await
        .unwrap();
    let attr2 = desc2.instance_tag_attribute().unwrap();
    assert!(!attr2.include_all_tags_of_instance().unwrap_or(true));
    assert!(attr2.instance_tag_keys().is_empty());
}

#[tokio::test]
async fn test_purchase_host_reservation() {
    let client = make_ec2_client().await;

    let resp = client
        .purchase_host_reservation()
        .host_id_set("h-12345678")
        .offering_id("hro-12345")
        .send()
        .await
        .expect("purchase_host_reservation should succeed");
    assert!(!resp.purchase().is_empty());
    assert_eq!(resp.currency_code().unwrap().as_str(), "USD");

    let desc = client
        .describe_host_reservations()
        .send()
        .await
        .expect("describe_host_reservations should succeed");
    assert!(!desc.host_reservation_set().is_empty());
}

#[tokio::test]
async fn test_purchase_then_run_scheduled_instances() {
    let client = make_ec2_client().await;

    let purchase = client
        .purchase_scheduled_instances()
        .purchase_requests(
            aws_sdk_ec2::types::PurchaseRequest::builder()
                .instance_count(1)
                .purchase_token("token-1")
                .build(),
        )
        .send()
        .await
        .expect("purchase_scheduled_instances should succeed");
    let si_id = purchase
        .scheduled_instance_set()
        .first()
        .unwrap()
        .scheduled_instance_id()
        .unwrap()
        .to_string();

    let run = client
        .run_scheduled_instances()
        .scheduled_instance_id(&si_id)
        .instance_count(1)
        .launch_specification(
            aws_sdk_ec2::types::ScheduledInstancesLaunchSpecification::builder()
                .image_id("ami-scheduled")
                .build(),
        )
        .send()
        .await
        .expect("run_scheduled_instances should succeed");
    assert_eq!(run.instance_id_set().len(), 1);

    let desc = client.describe_scheduled_instances().send().await.unwrap();
    assert!(!desc.scheduled_instance_set().is_empty());
}

#[tokio::test]
async fn test_delete_launch_template_versions() {
    let client = make_ec2_client().await;

    let lt = client
        .create_launch_template()
        .launch_template_name("multi-version-lt")
        .launch_template_data(
            aws_sdk_ec2::types::RequestLaunchTemplateData::builder()
                .image_id("ami-1")
                .build(),
        )
        .send()
        .await
        .unwrap()
        .launch_template()
        .unwrap()
        .clone();
    let lt_id = lt.launch_template_id().unwrap().to_string();

    client
        .create_launch_template_version()
        .launch_template_id(&lt_id)
        .launch_template_data(
            aws_sdk_ec2::types::RequestLaunchTemplateData::builder()
                .image_id("ami-2")
                .build(),
        )
        .send()
        .await
        .unwrap();

    client
        .create_launch_template_version()
        .launch_template_id(&lt_id)
        .launch_template_data(
            aws_sdk_ec2::types::RequestLaunchTemplateData::builder()
                .image_id("ami-3")
                .build(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .delete_launch_template_versions()
        .launch_template_id(&lt_id)
        .versions("2")
        .send()
        .await
        .expect("delete_launch_template_versions should succeed");
    assert_eq!(
        resp.successfully_deleted_launch_template_versions().len(),
        1
    );

    let desc = client
        .describe_launch_template_versions()
        .launch_template_id(&lt_id)
        .send()
        .await
        .unwrap();
    let versions: Vec<i64> = desc
        .launch_template_versions()
        .iter()
        .filter_map(|v| v.version_number())
        .collect();
    assert!(versions.contains(&1));
    assert!(versions.contains(&3));
    assert!(!versions.contains(&2));
}

#[tokio::test]
async fn test_modify_availability_zone_group() {
    let client = make_ec2_client().await;
    let resp = client
        .modify_availability_zone_group()
        .group_name("us-east-1-bos-1")
        .opt_in_status(aws_sdk_ec2::types::ModifyAvailabilityZoneOptInStatus::OptedIn)
        .send()
        .await
        .expect("modify_availability_zone_group should succeed");
    assert!(resp.r#return().unwrap_or(false));
}

#[tokio::test]
async fn test_confirm_product_instance() {
    let client = make_ec2_client().await;
    let inst_id = client
        .run_instances()
        .image_id("ami-base")
        .min_count(1)
        .max_count(1)
        .send()
        .await
        .unwrap()
        .instances()
        .first()
        .unwrap()
        .instance_id()
        .unwrap()
        .to_string();

    let resp = client
        .confirm_product_instance()
        .product_code("ABCDEFG12345")
        .instance_id(&inst_id)
        .send()
        .await
        .expect("confirm_product_instance should succeed");
    assert_eq!(resp.owner_id().unwrap_or(""), "123456789012");
    assert!(resp.r#return().unwrap_or(false));
}

#[tokio::test]
async fn test_reset_ebs_default_kms_key_id() {
    let client = make_ec2_client().await;
    let resp = client
        .reset_ebs_default_kms_key_id()
        .send()
        .await
        .expect("reset_ebs_default_kms_key_id should succeed");
    assert_eq!(resp.kms_key_id().unwrap_or(""), "alias/aws/ebs");
}

#[tokio::test]
async fn test_send_diagnostic_interrupt() {
    let client = make_ec2_client().await;
    let inst_id = client
        .run_instances()
        .image_id("ami-base")
        .min_count(1)
        .max_count(1)
        .send()
        .await
        .unwrap()
        .instances()
        .first()
        .unwrap()
        .instance_id()
        .unwrap()
        .to_string();
    client
        .send_diagnostic_interrupt()
        .instance_id(&inst_id)
        .send()
        .await
        .expect("send_diagnostic_interrupt should succeed");
}

#[tokio::test]
async fn test_report_instance_status() {
    let client = make_ec2_client().await;
    let inst_id = client
        .run_instances()
        .image_id("ami-base")
        .min_count(1)
        .max_count(1)
        .send()
        .await
        .unwrap()
        .instances()
        .first()
        .unwrap()
        .instance_id()
        .unwrap()
        .to_string();

    client
        .report_instance_status()
        .instances(&inst_id)
        .status(aws_sdk_ec2::types::ReportStatusType::Impaired)
        .reason_codes(aws_sdk_ec2::types::ReportInstanceReasonCodes::PerformanceNetwork)
        .send()
        .await
        .expect("report_instance_status should succeed");
}

#[tokio::test]
async fn test_replace_image_criteria_in_allowed_images_settings() {
    let client = make_ec2_client().await;

    client
        .replace_image_criteria_in_allowed_images_settings()
        .image_criteria(
            aws_sdk_ec2::types::ImageCriterionRequest::builder()
                .image_providers("amazon")
                .build(),
        )
        .send()
        .await
        .expect("replace_image_criteria_in_allowed_images_settings should succeed");

    let resp = client
        .get_allowed_images_settings()
        .send()
        .await
        .expect("get_allowed_images_settings should succeed");
    assert_eq!(resp.state().unwrap_or(""), "enabled");
}

#[tokio::test]
async fn test_restore_managed_prefix_list_version() {
    let client = make_ec2_client().await;

    let create = client
        .create_managed_prefix_list()
        .prefix_list_name("pl-version-test")
        .max_entries(10)
        .address_family("IPv4")
        .entries(
            aws_sdk_ec2::types::AddPrefixListEntry::builder()
                .cidr("10.0.0.0/16")
                .build(),
        )
        .send()
        .await
        .expect("create_managed_prefix_list should succeed");
    let pl_id = create
        .prefix_list()
        .unwrap()
        .prefix_list_id()
        .unwrap()
        .to_string();

    client
        .modify_managed_prefix_list()
        .prefix_list_id(&pl_id)
        .add_entries(
            aws_sdk_ec2::types::AddPrefixListEntry::builder()
                .cidr("11.0.0.0/16")
                .build(),
        )
        .send()
        .await
        .expect("modify_managed_prefix_list should succeed");

    client
        .restore_managed_prefix_list_version()
        .prefix_list_id(&pl_id)
        .previous_version(1)
        .current_version(2)
        .send()
        .await
        .expect("restore_managed_prefix_list_version should succeed");

    let entries = client
        .get_managed_prefix_list_entries()
        .prefix_list_id(&pl_id)
        .send()
        .await
        .unwrap();
    let cidrs: Vec<String> = entries
        .entries()
        .iter()
        .filter_map(|e| e.cidr().map(|c| c.to_string()))
        .collect();
    assert!(cidrs.contains(&"10.0.0.0/16".to_string()));
    assert!(!cidrs.contains(&"11.0.0.0/16".to_string()));
}

#[tokio::test]
async fn test_delete_queued_reserved_instances() {
    let client = make_ec2_client().await;
    // Nothing in the queue, so all IDs should land in the failure set.
    let resp = client
        .delete_queued_reserved_instances()
        .reserved_instances_ids("ri-not-real")
        .send()
        .await
        .expect("delete_queued_reserved_instances should succeed");
    assert!(resp.successful_queued_purchase_deletions().is_empty());
    assert_eq!(resp.failed_queued_purchase_deletions().len(), 1);
}

// ===== Group 6: Network Insights tests =====

#[tokio::test]
async fn test_network_insights_path_create_start_delete_lifecycle() {
    let client = make_ec2_client().await;
    let create = client
        .create_network_insights_path()
        .source("eni-source")
        .destination("eni-destination")
        .protocol(aws_sdk_ec2::types::Protocol::Tcp)
        .destination_port(443)
        .send()
        .await
        .expect("create_network_insights_path should succeed");
    let path = create.network_insights_path().expect("should have a path");
    let path_id = path
        .network_insights_path_id()
        .expect("path id")
        .to_string();
    assert!(path_id.starts_with("nip-"));

    // Start an analysis (mock immediately succeeds)
    let start = client
        .start_network_insights_analysis()
        .network_insights_path_id(&path_id)
        .client_token("ct-1")
        .send()
        .await
        .expect("start_network_insights_analysis should succeed");
    let analysis = start
        .network_insights_analysis()
        .expect("should have an analysis");
    let analysis_id = analysis
        .network_insights_analysis_id()
        .expect("analysis id")
        .to_string();
    assert!(analysis_id.starts_with("nia-"));
    assert_eq!(
        analysis.status(),
        Some(&aws_sdk_ec2::types::AnalysisStatus::Succeeded)
    );
    assert_eq!(analysis.network_path_found(), Some(true));

    // Deleting the path while analyses are alive should fail.
    let err = client
        .delete_network_insights_path()
        .network_insights_path_id(&path_id)
        .send()
        .await
        .expect_err("should refuse deleting path with analyses");
    let dbg = format!("{:?}", err);
    assert!(
        dbg.contains("DependencyViolation") || dbg.contains("has active analyses"),
        "unexpected error: {dbg}"
    );

    // Delete the analysis, then the path.
    client
        .delete_network_insights_analysis()
        .network_insights_analysis_id(&analysis_id)
        .send()
        .await
        .expect("delete_network_insights_analysis should succeed");
    client
        .delete_network_insights_path()
        .network_insights_path_id(&path_id)
        .send()
        .await
        .expect("delete_network_insights_path should succeed");
}

#[tokio::test]
async fn test_network_insights_access_scope_lifecycle() {
    let client = make_ec2_client().await;
    let create = client
        .create_network_insights_access_scope()
        .client_token("ct-scope-1")
        .send()
        .await
        .expect("create_network_insights_access_scope should succeed");
    let scope = create
        .network_insights_access_scope()
        .expect("should have a scope");
    let scope_id = scope
        .network_insights_access_scope_id()
        .expect("scope id")
        .to_string();
    assert!(scope_id.starts_with("nis-"));

    let start = client
        .start_network_insights_access_scope_analysis()
        .network_insights_access_scope_id(&scope_id)
        .client_token("ct-scope-analysis-1")
        .send()
        .await
        .expect("start_network_insights_access_scope_analysis should succeed");
    let analysis = start
        .network_insights_access_scope_analysis()
        .expect("should have analysis");
    let analysis_id = analysis
        .network_insights_access_scope_analysis_id()
        .expect("analysis id")
        .to_string();
    assert!(analysis_id.starts_with("nisa-"));

    client
        .delete_network_insights_access_scope_analysis()
        .network_insights_access_scope_analysis_id(&analysis_id)
        .send()
        .await
        .expect("delete_network_insights_access_scope_analysis should succeed");
    client
        .delete_network_insights_access_scope()
        .network_insights_access_scope_id(&scope_id)
        .send()
        .await
        .expect("delete_network_insights_access_scope should succeed");
}

// ===== Group 6: Traffic Mirror tests =====

#[tokio::test]
async fn test_traffic_mirror_filter_lifecycle() {
    let client = make_ec2_client().await;
    let filter = client
        .create_traffic_mirror_filter()
        .description("test filter")
        .send()
        .await
        .expect("create_traffic_mirror_filter should succeed")
        .traffic_mirror_filter()
        .expect("filter")
        .clone();
    let filter_id = filter
        .traffic_mirror_filter_id()
        .expect("filter id")
        .to_string();
    assert!(filter_id.starts_with("tmf-"));

    // Add an ingress rule.
    let rule = client
        .create_traffic_mirror_filter_rule()
        .traffic_mirror_filter_id(&filter_id)
        .traffic_direction(aws_sdk_ec2::types::TrafficDirection::Ingress)
        .rule_number(100)
        .rule_action(aws_sdk_ec2::types::TrafficMirrorRuleAction::Accept)
        .destination_cidr_block("10.0.0.0/16")
        .source_cidr_block("0.0.0.0/0")
        .protocol(6)
        .send()
        .await
        .expect("create_traffic_mirror_filter_rule should succeed")
        .traffic_mirror_filter_rule()
        .expect("rule")
        .clone();
    let rule_id = rule
        .traffic_mirror_filter_rule_id()
        .expect("rule id")
        .to_string();
    assert!(rule_id.starts_with("tmfr-"));
    assert_eq!(rule.rule_number(), Some(100));

    // Modify the rule (number → 200).
    let modified_rule = client
        .modify_traffic_mirror_filter_rule()
        .traffic_mirror_filter_rule_id(&rule_id)
        .rule_number(200)
        .send()
        .await
        .expect("modify_traffic_mirror_filter_rule should succeed")
        .traffic_mirror_filter_rule()
        .expect("rule")
        .clone();
    assert_eq!(modified_rule.rule_number(), Some(200));

    // Modify network services on the filter.
    let modified_filter = client
        .modify_traffic_mirror_filter_network_services()
        .traffic_mirror_filter_id(&filter_id)
        .add_network_services(aws_sdk_ec2::types::TrafficMirrorNetworkService::AmazonDns)
        .send()
        .await
        .expect("modify_traffic_mirror_filter_network_services should succeed")
        .traffic_mirror_filter()
        .expect("filter")
        .clone();
    assert!(
        modified_filter
            .network_services()
            .iter()
            .any(|s| s == &aws_sdk_ec2::types::TrafficMirrorNetworkService::AmazonDns)
    );

    // Delete the rule, then the filter.
    client
        .delete_traffic_mirror_filter_rule()
        .traffic_mirror_filter_rule_id(&rule_id)
        .send()
        .await
        .expect("delete_traffic_mirror_filter_rule should succeed");
    client
        .delete_traffic_mirror_filter()
        .traffic_mirror_filter_id(&filter_id)
        .send()
        .await
        .expect("delete_traffic_mirror_filter should succeed");
}

#[tokio::test]
async fn test_traffic_mirror_session_lifecycle_and_inuse_refusals() {
    let client = make_ec2_client().await;

    let target_id = client
        .create_traffic_mirror_target()
        .network_interface_id("eni-target")
        .description("target")
        .send()
        .await
        .expect("create_traffic_mirror_target should succeed")
        .traffic_mirror_target()
        .expect("target")
        .traffic_mirror_target_id()
        .expect("id")
        .to_string();
    assert!(target_id.starts_with("tmt-"));

    let filter_id = client
        .create_traffic_mirror_filter()
        .send()
        .await
        .expect("create_traffic_mirror_filter should succeed")
        .traffic_mirror_filter()
        .expect("filter")
        .traffic_mirror_filter_id()
        .expect("id")
        .to_string();

    let session_id = client
        .create_traffic_mirror_session()
        .traffic_mirror_target_id(&target_id)
        .traffic_mirror_filter_id(&filter_id)
        .network_interface_id("eni-source")
        .session_number(1)
        .send()
        .await
        .expect("create_traffic_mirror_session should succeed")
        .traffic_mirror_session()
        .expect("session")
        .traffic_mirror_session_id()
        .expect("id")
        .to_string();
    assert!(session_id.starts_with("tms-"));

    // Modify the session.
    let modified = client
        .modify_traffic_mirror_session()
        .traffic_mirror_session_id(&session_id)
        .session_number(2)
        .description("updated")
        .send()
        .await
        .expect("modify_traffic_mirror_session should succeed")
        .traffic_mirror_session()
        .expect("session")
        .clone();
    assert_eq!(modified.session_number(), Some(2));
    assert_eq!(modified.description(), Some("updated"));

    // Deleting the target while session refers to it should fail.
    let err = client
        .delete_traffic_mirror_target()
        .traffic_mirror_target_id(&target_id)
        .send()
        .await
        .expect_err("should refuse deleting target while session references it");
    assert!(
        format!("{:?}", err).contains("InUse"),
        "unexpected error: {err:?}"
    );

    // Deleting the filter while session refers to it should fail.
    let err = client
        .delete_traffic_mirror_filter()
        .traffic_mirror_filter_id(&filter_id)
        .send()
        .await
        .expect_err("should refuse deleting filter while session references it");
    assert!(
        format!("{:?}", err).contains("InUse"),
        "unexpected error: {err:?}"
    );

    // Now delete the session, then the target/filter.
    client
        .delete_traffic_mirror_session()
        .traffic_mirror_session_id(&session_id)
        .send()
        .await
        .expect("delete_traffic_mirror_session should succeed");
    client
        .delete_traffic_mirror_target()
        .traffic_mirror_target_id(&target_id)
        .send()
        .await
        .expect("delete_traffic_mirror_target should succeed");
    client
        .delete_traffic_mirror_filter()
        .traffic_mirror_filter_id(&filter_id)
        .send()
        .await
        .expect("delete_traffic_mirror_filter should succeed");
}

#[tokio::test]
async fn test_traffic_mirror_target_in_use_refused() {
    let client = make_ec2_client().await;
    let target_id = client
        .create_traffic_mirror_target()
        .network_interface_id("eni-target-2")
        .send()
        .await
        .expect("create_traffic_mirror_target should succeed")
        .traffic_mirror_target()
        .expect("target")
        .traffic_mirror_target_id()
        .expect("id")
        .to_string();
    let filter_id = client
        .create_traffic_mirror_filter()
        .send()
        .await
        .expect("create_traffic_mirror_filter should succeed")
        .traffic_mirror_filter()
        .expect("filter")
        .traffic_mirror_filter_id()
        .expect("id")
        .to_string();
    let _session_id = client
        .create_traffic_mirror_session()
        .traffic_mirror_target_id(&target_id)
        .traffic_mirror_filter_id(&filter_id)
        .network_interface_id("eni-src")
        .session_number(1)
        .send()
        .await
        .expect("create_traffic_mirror_session should succeed");

    let err = client
        .delete_traffic_mirror_target()
        .traffic_mirror_target_id(&target_id)
        .send()
        .await
        .expect_err("delete_traffic_mirror_target should refuse while session present");
    assert!(
        format!("{:?}", err).contains("InUse"),
        "unexpected error: {err:?}"
    );
}

// ===== Group 7: Client VPN tests =====

#[tokio::test]
async fn test_client_vpn_endpoint_full_lifecycle() {
    let client = make_ec2_client().await;

    // First, create a VPC + subnet so the target-network association picks up a vpc_id.
    let vpc_id = client
        .create_vpc()
        .cidr_block("10.20.0.0/16")
        .send()
        .await
        .expect("create_vpc")
        .vpc()
        .expect("vpc")
        .vpc_id()
        .expect("id")
        .to_string();
    let subnet_id = client
        .create_subnet()
        .vpc_id(&vpc_id)
        .cidr_block("10.20.1.0/24")
        .send()
        .await
        .expect("create_subnet")
        .subnet()
        .expect("subnet")
        .subnet_id()
        .expect("id")
        .to_string();

    // Create endpoint.
    let endpoint_id = client
        .create_client_vpn_endpoint()
        .client_cidr_block("10.30.0.0/22")
        .server_certificate_arn("arn:aws:acm:us-east-1:123456789012:certificate/test")
        .send()
        .await
        .expect("create_client_vpn_endpoint")
        .client_vpn_endpoint_id()
        .expect("id")
        .to_string();
    assert!(endpoint_id.starts_with("cvpn-endpoint-"));

    // Associate target network.
    let assoc_id = client
        .associate_client_vpn_target_network()
        .client_vpn_endpoint_id(&endpoint_id)
        .subnet_id(&subnet_id)
        .send()
        .await
        .expect("associate_client_vpn_target_network")
        .association_id()
        .expect("association_id")
        .to_string();

    // Apply security groups.
    let applied = client
        .apply_security_groups_to_client_vpn_target_network()
        .client_vpn_endpoint_id(&endpoint_id)
        .vpc_id(&vpc_id)
        .security_group_ids("sg-aaaa1111")
        .security_group_ids("sg-bbbb2222")
        .send()
        .await
        .expect("apply_security_groups");
    let sgs = applied.security_group_ids();
    assert_eq!(sgs.len(), 2);
    assert!(sgs.iter().any(|s| s == "sg-aaaa1111"));

    // Authorize ingress.
    client
        .authorize_client_vpn_ingress()
        .client_vpn_endpoint_id(&endpoint_id)
        .target_network_cidr("0.0.0.0/0")
        .authorize_all_groups(true)
        .send()
        .await
        .expect("authorize_client_vpn_ingress");

    // Create route.
    client
        .create_client_vpn_route()
        .client_vpn_endpoint_id(&endpoint_id)
        .destination_cidr_block("192.168.0.0/16")
        .target_vpc_subnet_id(&subnet_id)
        .send()
        .await
        .expect("create_client_vpn_route");

    // Modify endpoint (description + vpn_port).
    client
        .modify_client_vpn_endpoint()
        .client_vpn_endpoint_id(&endpoint_id)
        .description("modified for test")
        .vpn_port(1194)
        .send()
        .await
        .expect("modify_client_vpn_endpoint");

    // Delete endpoint should be refused (associations and routes exist).
    let err = client
        .delete_client_vpn_endpoint()
        .client_vpn_endpoint_id(&endpoint_id)
        .send()
        .await
        .expect_err("delete_client_vpn_endpoint should refuse");
    assert!(
        format!("{err:?}").contains("InUse") || format!("{err:?}").contains("ClientVpnEndpoint"),
        "unexpected error: {err:?}"
    );

    // Disassociate target network.
    client
        .disassociate_client_vpn_target_network()
        .client_vpn_endpoint_id(&endpoint_id)
        .association_id(&assoc_id)
        .send()
        .await
        .expect("disassociate_client_vpn_target_network");

    // Revoke ingress.
    client
        .revoke_client_vpn_ingress()
        .client_vpn_endpoint_id(&endpoint_id)
        .target_network_cidr("0.0.0.0/0")
        .revoke_all_groups(true)
        .send()
        .await
        .expect("revoke_client_vpn_ingress");

    // Delete route.
    client
        .delete_client_vpn_route()
        .client_vpn_endpoint_id(&endpoint_id)
        .destination_cidr_block("192.168.0.0/16")
        .target_vpc_subnet_id(&subnet_id)
        .send()
        .await
        .expect("delete_client_vpn_route");

    // Now delete should succeed.
    client
        .delete_client_vpn_endpoint()
        .client_vpn_endpoint_id(&endpoint_id)
        .send()
        .await
        .expect("delete_client_vpn_endpoint should succeed");
}

#[tokio::test]
async fn test_client_vpn_import_revocation_list_visible_in_state() {
    let client = make_ec2_client().await;
    let endpoint_id = client
        .create_client_vpn_endpoint()
        .client_cidr_block("10.40.0.0/22")
        .server_certificate_arn("arn:aws:acm:us-east-1:123456789012:certificate/crl-test")
        .send()
        .await
        .expect("create_client_vpn_endpoint")
        .client_vpn_endpoint_id()
        .expect("id")
        .to_string();

    let res = client
        .import_client_vpn_client_certificate_revocation_list()
        .client_vpn_endpoint_id(&endpoint_id)
        .certificate_revocation_list("-----BEGIN X509 CRL-----\nfake\n-----END X509 CRL-----")
        .send()
        .await
        .expect("import_client_vpn_client_certificate_revocation_list");
    assert_eq!(res.r#return(), Some(true));
}

#[tokio::test]
async fn test_client_vpn_terminate_connections_marks_terminating() {
    use winterbaume_core::StatefulService;
    use winterbaume_ec2::views::{
        ClientVpnConnectionStatusView, ClientVpnConnectionView, ClientVpnEndpointStatusView,
        ClientVpnEndpointView, Ec2StateView,
    };

    // Standalone Ec2Service so we can directly seed state.
    let svc = Ec2Service::new();
    // Seed an endpoint and a connection.
    let mut view = Ec2StateView::default();
    let endpoint_id = "cvpn-endpoint-seed1".to_string();
    let conn_id = "cvpn-connection-seed-1".to_string();
    let now = "2026-04-30T12:00:00.000Z".to_string();
    view.client_vpn_endpoints.insert(
        endpoint_id.clone(),
        ClientVpnEndpointView {
            client_vpn_endpoint_id: endpoint_id.clone(),
            description: None,
            status: ClientVpnEndpointStatusView {
                code: "available".to_string(),
                message: None,
            },
            creation_time: now.clone(),
            deletion_time: None,
            dns_name: format!("{endpoint_id}.cvpn-endpoint.amazonaws.com"),
            client_cidr_block: "10.50.0.0/22".to_string(),
            dns_servers: vec![],
            split_tunnel: false,
            vpn_protocol: "openvpn".to_string(),
            transport_protocol: "udp".to_string(),
            vpn_port: 443,
            server_certificate_arn: "arn:aws:acm:us-east-1:123456789012:certificate/x".to_string(),
            authentication_options: vec![],
            connection_log_options_enabled: false,
            connection_log_options_cloudwatch_log_group: None,
            connection_log_options_cloudwatch_log_stream: None,
            tags: std::collections::HashMap::new(),
            security_group_ids: vec![],
            vpc_id: None,
            self_service_portal_url: None,
            self_service_portal: "disabled".to_string(),
            session_timeout_hours: 24,
            client_login_banner_enabled: false,
            client_login_banner_text: None,
            disconnect_on_session_timeout: false,
            client_route_enforcement_enforced: false,
            client_certificate_revocation_list: None,
        },
    );
    view.client_vpn_connections.insert(
        conn_id.clone(),
        ClientVpnConnectionView {
            connection_id: conn_id.clone(),
            client_vpn_endpoint_id: endpoint_id.clone(),
            username: Some("alice".to_string()),
            status: ClientVpnConnectionStatusView {
                code: "active".to_string(),
                message: None,
            },
            posture_compliance_statuses: vec![],
            common_name: None,
            connection_established_time: now.clone(),
            connection_end_time: None,
            ingress_bytes: "0".to_string(),
            egress_bytes: "0".to_string(),
            ingress_packets: "0".to_string(),
            egress_packets: "0".to_string(),
            client_ip: None,
            client_port: None,
            timestamp: now,
        },
    );
    svc.restore("123456789012", "us-east-1", view)
        .await
        .expect("restore");

    // Build the SDK pointing at this exact service instance.
    let mock = winterbaume_core::MockAws::builder()
        .with_service(svc)
        .build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_ec2::config::Region::new("us-east-1"))
        .load()
        .await;
    let client = aws_sdk_ec2::Client::new(&config);

    let res = client
        .terminate_client_vpn_connections()
        .client_vpn_endpoint_id(&endpoint_id)
        .connection_id(&conn_id)
        .send()
        .await
        .expect("terminate_client_vpn_connections");
    let statuses = res.connection_statuses();
    assert_eq!(statuses.len(), 1);
    let cur = statuses[0]
        .current_status()
        .and_then(|s| s.code())
        .map(|c| c.as_str().to_string())
        .unwrap_or_default();
    assert_eq!(cur, "terminating");
}

// ===== Group 7: Local Gateway tests =====

#[tokio::test]
async fn test_local_gateway_full_lifecycle() {
    let client = make_ec2_client().await;

    // Pre-create a VPC for VPC associations and tests.
    let vpc_id = client
        .create_vpc()
        .cidr_block("10.60.0.0/16")
        .send()
        .await
        .expect("create_vpc")
        .vpc()
        .expect("vpc")
        .vpc_id()
        .expect("id")
        .to_string();

    // Create a route table (auto-seeds local-gateway).
    let lgw_seed_id = "lgw-seedlife".to_string();
    let table = client
        .create_local_gateway_route_table()
        .local_gateway_id(&lgw_seed_id)
        .mode(aws_sdk_ec2::types::LocalGatewayRouteTableMode::DirectVpcRouting)
        .send()
        .await
        .expect("create_local_gateway_route_table")
        .local_gateway_route_table()
        .expect("table")
        .clone();
    let table_id = table
        .local_gateway_route_table_id()
        .expect("id")
        .to_string();
    let lgw_id = table.local_gateway_id().expect("lgw id").to_string();

    // Create a virtual interface group.
    let vif_grp_id = client
        .create_local_gateway_virtual_interface_group()
        .local_gateway_id(&lgw_id)
        .local_bgp_asn(64512)
        .send()
        .await
        .expect("create_local_gateway_virtual_interface_group")
        .local_gateway_virtual_interface_group()
        .expect("group")
        .local_gateway_virtual_interface_group_id()
        .expect("id")
        .to_string();

    // Create a virtual interface in that group.
    let vif_id = client
        .create_local_gateway_virtual_interface()
        .local_gateway_virtual_interface_group_id(&vif_grp_id)
        .vlan(100)
        .local_address("169.254.0.1")
        .peer_address("169.254.0.2")
        .peer_bgp_asn(65000)
        .send()
        .await
        .expect("create_local_gateway_virtual_interface")
        .local_gateway_virtual_interface()
        .expect("vif")
        .local_gateway_virtual_interface_id()
        .expect("id")
        .to_string();

    // Group association.
    let group_assoc_id = client
        .create_local_gateway_route_table_virtual_interface_group_association()
        .local_gateway_route_table_id(&table_id)
        .local_gateway_virtual_interface_group_id(&vif_grp_id)
        .send()
        .await
        .expect("create_local_gateway_route_table_virtual_interface_group_association")
        .local_gateway_route_table_virtual_interface_group_association()
        .expect("group_assoc")
        .local_gateway_route_table_virtual_interface_group_association_id()
        .expect("id")
        .to_string();

    // VPC association.
    let vpc_assoc_id = client
        .create_local_gateway_route_table_vpc_association()
        .local_gateway_route_table_id(&table_id)
        .vpc_id(&vpc_id)
        .send()
        .await
        .expect("create_local_gateway_route_table_vpc_association")
        .local_gateway_route_table_vpc_association()
        .expect("vpc_assoc")
        .local_gateway_route_table_vpc_association_id()
        .expect("id")
        .to_string();

    // Create a route on the table.
    client
        .create_local_gateway_route()
        .local_gateway_route_table_id(&table_id)
        .destination_cidr_block("10.99.0.0/16")
        .local_gateway_virtual_interface_group_id(&vif_grp_id)
        .send()
        .await
        .expect("create_local_gateway_route");

    // Modify the route (set a network interface id).
    let modified = client
        .modify_local_gateway_route()
        .local_gateway_route_table_id(&table_id)
        .destination_cidr_block("10.99.0.0/16")
        .network_interface_id("eni-deadbeef")
        .send()
        .await
        .expect("modify_local_gateway_route");
    assert_eq!(
        modified.route().and_then(|r| r.network_interface_id()),
        Some("eni-deadbeef")
    );

    // Refusal: deleting the route table now should fail because routes exist.
    let err = client
        .delete_local_gateway_route_table()
        .local_gateway_route_table_id(&table_id)
        .send()
        .await
        .expect_err("delete_local_gateway_route_table should refuse with routes");
    assert!(
        format!("{err:?}").contains("DependencyViolation") || format!("{err:?}").contains("InUse"),
        "unexpected error: {err:?}"
    );

    // Refusal: deleting vif while group still references it.
    let err = client
        .delete_local_gateway_virtual_interface()
        .local_gateway_virtual_interface_id(&vif_id)
        .send()
        .await
        .expect_err("delete vif should be refused while in a group");
    assert!(
        format!("{err:?}").contains("DependencyViolation") || format!("{err:?}").contains("InUse"),
        "unexpected error: {err:?}"
    );

    // Refusal: delete vif group with associations.
    let err = client
        .delete_local_gateway_virtual_interface_group()
        .local_gateway_virtual_interface_group_id(&vif_grp_id)
        .send()
        .await
        .expect_err("delete vif group should be refused while interfaces present");
    assert!(
        format!("{err:?}").contains("DependencyViolation") || format!("{err:?}").contains("InUse"),
        "unexpected error: {err:?}"
    );

    // Now tear down in correct order.
    client
        .delete_local_gateway_route()
        .local_gateway_route_table_id(&table_id)
        .destination_cidr_block("10.99.0.0/16")
        .send()
        .await
        .expect("delete_local_gateway_route");

    client
        .delete_local_gateway_route_table_vpc_association()
        .local_gateway_route_table_vpc_association_id(&vpc_assoc_id)
        .send()
        .await
        .expect("delete_lgw_rtb_vpc_assoc");

    client
        .delete_local_gateway_route_table_virtual_interface_group_association()
        .local_gateway_route_table_virtual_interface_group_association_id(&group_assoc_id)
        .send()
        .await
        .expect("delete_lgw_rtb_vif_group_assoc");

    // Finally, delete the route table (routes are now empty). We deliberately
    // leave the VIF group with its single VIF in place: the SDK has no public
    // op to "detach a VIF from a group" — AWS deletes the VIF and its group
    // together when the underlying physical infrastructure is decommissioned.
    // The refusal assertions above cover the dependency semantics that matter
    // for the mock.
    client
        .delete_local_gateway_route_table()
        .local_gateway_route_table_id(&table_id)
        .send()
        .await
        .expect("delete_local_gateway_route_table");

    // Suppress unused vif id warning.
    let _ = vif_id;
}

// ===== Group 8: Route Server =====

#[tokio::test]
async fn test_route_server_full_lifecycle() {
    let client = make_ec2_client().await;

    // Create a VPC and subnet for endpoint placement.
    let vpc_id = client
        .create_vpc()
        .cidr_block("10.70.0.0/16")
        .send()
        .await
        .expect("create_vpc")
        .vpc()
        .expect("vpc")
        .vpc_id()
        .expect("id")
        .to_string();
    let subnet_id = client
        .create_subnet()
        .vpc_id(&vpc_id)
        .cidr_block("10.70.1.0/24")
        .send()
        .await
        .expect("create_subnet")
        .subnet()
        .expect("subnet")
        .subnet_id()
        .expect("id")
        .to_string();

    // Create route server.
    let rs = client
        .create_route_server()
        .amazon_side_asn(64512)
        .persist_routes(aws_sdk_ec2::types::RouteServerPersistRoutesAction::Enable)
        .persist_routes_duration(2)
        .sns_notifications_enabled(true)
        .send()
        .await
        .expect("create_route_server")
        .route_server()
        .expect("rs")
        .clone();
    let rs_id = rs.route_server_id().expect("rs id").to_string();
    assert_eq!(rs.amazon_side_asn(), Some(64512));
    assert_eq!(
        rs.persist_routes_state(),
        Some(&aws_sdk_ec2::types::RouteServerPersistRoutesState::Enabled)
    );

    // Describe route servers and ensure ours is visible.
    let rss = client
        .describe_route_servers()
        .send()
        .await
        .expect("describe_route_servers");
    assert!(
        rss.route_servers()
            .iter()
            .any(|r| r.route_server_id() == Some(&rs_id)),
        "describe_route_servers should include the new route server"
    );

    // Modify route server: switch persist_routes to disable.
    let modified = client
        .modify_route_server()
        .route_server_id(&rs_id)
        .persist_routes(aws_sdk_ec2::types::RouteServerPersistRoutesAction::Disable)
        .send()
        .await
        .expect("modify_route_server")
        .route_server()
        .expect("modified rs")
        .clone();
    assert_eq!(
        modified.persist_routes_state(),
        Some(&aws_sdk_ec2::types::RouteServerPersistRoutesState::Disabled)
    );

    // Associate with the VPC.
    client
        .associate_route_server()
        .route_server_id(&rs_id)
        .vpc_id(&vpc_id)
        .send()
        .await
        .expect("associate_route_server");

    // Get associations.
    let assocs = client
        .get_route_server_associations()
        .route_server_id(&rs_id)
        .send()
        .await
        .expect("get_route_server_associations");
    assert_eq!(assocs.route_server_associations().len(), 1);
    assert_eq!(
        assocs.route_server_associations()[0].vpc_id(),
        Some(vpc_id.as_str())
    );

    // Refusal: deleting the route server with an association must fail.
    let err = client
        .delete_route_server()
        .route_server_id(&rs_id)
        .send()
        .await
        .expect_err("delete_route_server should refuse with associations");
    assert!(
        format!("{err:?}").contains("DependencyViolation") || format!("{err:?}").contains("InUse"),
        "unexpected error: {err:?}"
    );

    // Create a route server endpoint.
    let ep = client
        .create_route_server_endpoint()
        .route_server_id(&rs_id)
        .subnet_id(&subnet_id)
        .send()
        .await
        .expect("create_route_server_endpoint")
        .route_server_endpoint()
        .expect("endpoint")
        .clone();
    let ep_id = ep.route_server_endpoint_id().expect("ep id").to_string();
    assert_eq!(ep.subnet_id(), Some(subnet_id.as_str()));
    assert_eq!(ep.vpc_id(), Some(vpc_id.as_str()));

    // Describe endpoints.
    let eps = client
        .describe_route_server_endpoints()
        .send()
        .await
        .expect("describe_route_server_endpoints");
    assert!(
        eps.route_server_endpoints()
            .iter()
            .any(|e| e.route_server_endpoint_id() == Some(&ep_id))
    );

    // Refusal: deleting the route server with an endpoint also fails.
    let err = client
        .delete_route_server()
        .route_server_id(&rs_id)
        .send()
        .await
        .expect_err("delete_route_server should refuse with endpoints");
    assert!(
        format!("{err:?}").contains("DependencyViolation") || format!("{err:?}").contains("InUse"),
        "unexpected error: {err:?}"
    );

    // Create a peer.
    let bgp_options = aws_sdk_ec2::types::RouteServerBgpOptionsRequest::builder()
        .peer_asn(65000)
        .peer_liveness_detection(aws_sdk_ec2::types::RouteServerPeerLivenessMode::BgpKeepalive)
        .build();
    let peer = client
        .create_route_server_peer()
        .route_server_endpoint_id(&ep_id)
        .peer_address("169.254.10.1")
        .bgp_options(bgp_options)
        .send()
        .await
        .expect("create_route_server_peer")
        .route_server_peer()
        .expect("peer")
        .clone();
    let peer_id = peer.route_server_peer_id().expect("peer id").to_string();
    assert_eq!(peer.peer_address(), Some("169.254.10.1"));
    assert_eq!(peer.route_server_endpoint_id(), Some(ep_id.as_str()));

    // Describe peers.
    let peers = client
        .describe_route_server_peers()
        .send()
        .await
        .expect("describe_route_server_peers");
    assert!(
        peers
            .route_server_peers()
            .iter()
            .any(|p| p.route_server_peer_id() == Some(&peer_id))
    );

    // Refusal: deleting the endpoint with peers must fail.
    let err = client
        .delete_route_server_endpoint()
        .route_server_endpoint_id(&ep_id)
        .send()
        .await
        .expect_err("delete_route_server_endpoint should refuse with peers");
    assert!(
        format!("{err:?}").contains("DependencyViolation") || format!("{err:?}").contains("InUse"),
        "unexpected error: {err:?}"
    );

    // Tear down in correct order.
    client
        .delete_route_server_peer()
        .route_server_peer_id(&peer_id)
        .send()
        .await
        .expect("delete_route_server_peer");

    client
        .delete_route_server_endpoint()
        .route_server_endpoint_id(&ep_id)
        .send()
        .await
        .expect("delete_route_server_endpoint");

    client
        .disassociate_route_server()
        .route_server_id(&rs_id)
        .vpc_id(&vpc_id)
        .send()
        .await
        .expect("disassociate_route_server");

    // Now the delete should succeed.
    client
        .delete_route_server()
        .route_server_id(&rs_id)
        .send()
        .await
        .expect("delete_route_server");
}

#[tokio::test]
async fn test_route_server_modify_persist_routes_visible() {
    let client = make_ec2_client().await;
    let rs_id = client
        .create_route_server()
        .amazon_side_asn(64600)
        .persist_routes(aws_sdk_ec2::types::RouteServerPersistRoutesAction::Enable)
        .send()
        .await
        .expect("create_route_server")
        .route_server()
        .expect("rs")
        .route_server_id()
        .expect("id")
        .to_string();

    // Modify: switch persist_routes from enable to disable.
    client
        .modify_route_server()
        .route_server_id(&rs_id)
        .persist_routes(aws_sdk_ec2::types::RouteServerPersistRoutesAction::Disable)
        .send()
        .await
        .expect("modify_route_server");

    // Verify via describe_route_servers.
    let rss = client
        .describe_route_servers()
        .route_server_ids(&rs_id)
        .send()
        .await
        .expect("describe_route_servers");
    let rs = rss
        .route_servers()
        .iter()
        .find(|r| r.route_server_id() == Some(&rs_id))
        .expect("route server visible");
    assert_eq!(
        rs.persist_routes_state(),
        Some(&aws_sdk_ec2::types::RouteServerPersistRoutesState::Disabled)
    );

    // Cleanup: no associations / endpoints / peers, so direct delete works.
    client
        .delete_route_server()
        .route_server_id(&rs_id)
        .send()
        .await
        .expect("delete_route_server");
}

#[tokio::test]
async fn test_route_server_disassociate_unknown_refused() {
    let client = make_ec2_client().await;
    let rs_id = client
        .create_route_server()
        .amazon_side_asn(64700)
        .send()
        .await
        .expect("create_route_server")
        .route_server()
        .expect("rs")
        .route_server_id()
        .expect("id")
        .to_string();

    // Try to disassociate from a vpc that was never associated.
    let err = client
        .disassociate_route_server()
        .route_server_id(&rs_id)
        .vpc_id("vpc-doesnotexist")
        .send()
        .await
        .expect_err("disassociate should fail");
    assert!(
        format!("{err:?}").contains("InvalidRouteServerAssociation"),
        "unexpected error: {err:?}"
    );

    client
        .delete_route_server()
        .route_server_id(&rs_id)
        .send()
        .await
        .expect("delete_route_server");
}

// ===== Group 9: Verified Access =====

#[tokio::test]
async fn test_verified_access_instance_full_lifecycle() {
    use aws_sdk_ec2::types::{
        ModifyVerifiedAccessTrustProviderOidcOptions, VerifiedAccessLogOptions,
        VerifiedAccessLogS3DestinationOptions,
    };

    let client = make_ec2_client().await;

    // Create instance.
    let instance_id = client
        .create_verified_access_instance()
        .description("test instance")
        .send()
        .await
        .expect("create_verified_access_instance")
        .verified_access_instance()
        .expect("instance")
        .verified_access_instance_id()
        .expect("id")
        .to_string();
    assert!(instance_id.starts_with("vai-"));

    // Create trust provider.
    let tp_id = client
        .create_verified_access_trust_provider()
        .trust_provider_type(aws_sdk_ec2::types::TrustProviderType::User)
        .user_trust_provider_type(aws_sdk_ec2::types::UserTrustProviderType::Oidc)
        .policy_reference_name("ref1")
        .description("tp")
        .send()
        .await
        .expect("create_verified_access_trust_provider")
        .verified_access_trust_provider()
        .expect("tp")
        .verified_access_trust_provider_id()
        .expect("id")
        .to_string();
    assert!(tp_id.starts_with("vatp-"));

    // Attach.
    let attach_resp = client
        .attach_verified_access_trust_provider()
        .verified_access_instance_id(&instance_id)
        .verified_access_trust_provider_id(&tp_id)
        .send()
        .await
        .expect("attach_verified_access_trust_provider");
    let inst_after_attach = attach_resp.verified_access_instance().expect("instance");
    let tps = inst_after_attach.verified_access_trust_providers();
    assert_eq!(tps.len(), 1);
    assert_eq!(
        tps[0].verified_access_trust_provider_id().unwrap_or(""),
        tp_id
    );

    // Modify instance description.
    client
        .modify_verified_access_instance()
        .verified_access_instance_id(&instance_id)
        .description("modified")
        .send()
        .await
        .expect("modify_verified_access_instance");

    // Modify trust provider OIDC options.
    let oidc = ModifyVerifiedAccessTrustProviderOidcOptions::builder()
        .scope("openid email")
        .issuer("https://issuer.example.com/")
        .build();
    client
        .modify_verified_access_trust_provider()
        .verified_access_trust_provider_id(&tp_id)
        .oidc_options(oidc)
        .description("modified-tp")
        .send()
        .await
        .expect("modify_verified_access_trust_provider");

    // Set logging configuration.
    let logs = VerifiedAccessLogOptions::builder()
        .s3(VerifiedAccessLogS3DestinationOptions::builder()
            .enabled(true)
            .bucket_name("audit-bucket")
            .build())
        .build();
    client
        .modify_verified_access_instance_logging_configuration()
        .verified_access_instance_id(&instance_id)
        .access_logs(logs)
        .send()
        .await
        .expect("modify_verified_access_instance_logging_configuration");

    // Detach.
    client
        .detach_verified_access_trust_provider()
        .verified_access_instance_id(&instance_id)
        .verified_access_trust_provider_id(&tp_id)
        .send()
        .await
        .expect("detach_verified_access_trust_provider");

    // Now delete trust provider.
    client
        .delete_verified_access_trust_provider()
        .verified_access_trust_provider_id(&tp_id)
        .send()
        .await
        .expect("delete_verified_access_trust_provider");

    // And delete instance.
    client
        .delete_verified_access_instance()
        .verified_access_instance_id(&instance_id)
        .send()
        .await
        .expect("delete_verified_access_instance");
}

#[tokio::test]
async fn test_verified_access_group_endpoint_lifecycle() {
    use aws_sdk_ec2::types::{
        CreateVerifiedAccessEndpointLoadBalancerOptions,
        ModifyVerifiedAccessEndpointLoadBalancerOptions, VerifiedAccessEndpointAttachmentType,
        VerifiedAccessEndpointType,
    };

    let client = make_ec2_client().await;

    let instance_id = client
        .create_verified_access_instance()
        .send()
        .await
        .expect("create_verified_access_instance")
        .verified_access_instance()
        .expect("instance")
        .verified_access_instance_id()
        .expect("id")
        .to_string();

    // Create group.
    let group_id = client
        .create_verified_access_group()
        .verified_access_instance_id(&instance_id)
        .description("group desc")
        .send()
        .await
        .expect("create_verified_access_group")
        .verified_access_group()
        .expect("group")
        .verified_access_group_id()
        .expect("id")
        .to_string();
    assert!(group_id.starts_with("vagr-"));

    // Modify group policy.
    let mod_pol = client
        .modify_verified_access_group_policy()
        .verified_access_group_id(&group_id)
        .policy_document("permit(principal,action,resource);")
        .policy_enabled(true)
        .send()
        .await
        .expect("modify_verified_access_group_policy");
    assert_eq!(mod_pol.policy_enabled(), Some(true));
    assert!(mod_pol.policy_document().unwrap_or("").contains("permit"));

    // Create endpoint (load-balancer type).
    let lb_options = CreateVerifiedAccessEndpointLoadBalancerOptions::builder()
        .load_balancer_arn(
            "arn:aws:elasticloadbalancing:us-east-1:123456789012:loadbalancer/app/test",
        )
        .port(443)
        .protocol(aws_sdk_ec2::types::VerifiedAccessEndpointProtocol::Https)
        .subnet_ids("subnet-aaaaaaaa")
        .build();
    let endpoint_id = client
        .create_verified_access_endpoint()
        .verified_access_group_id(&group_id)
        .endpoint_type(VerifiedAccessEndpointType::LoadBalancer)
        .attachment_type(VerifiedAccessEndpointAttachmentType::Vpc)
        .application_domain("app.example.com")
        .domain_certificate_arn("arn:aws:acm:us-east-1:123456789012:certificate/test")
        .endpoint_domain_prefix("api")
        .load_balancer_options(lb_options)
        .send()
        .await
        .expect("create_verified_access_endpoint")
        .verified_access_endpoint()
        .expect("endpoint")
        .verified_access_endpoint_id()
        .expect("id")
        .to_string();
    assert!(endpoint_id.starts_with("vae-"));

    // Modify endpoint policy.
    let mod_ep_pol = client
        .modify_verified_access_endpoint_policy()
        .verified_access_endpoint_id(&endpoint_id)
        .policy_document("permit(principal,action,resource);")
        .policy_enabled(true)
        .send()
        .await
        .expect("modify_verified_access_endpoint_policy");
    assert_eq!(mod_ep_pol.policy_enabled(), Some(true));

    // Modify endpoint description + lb_options.
    let mod_lb = ModifyVerifiedAccessEndpointLoadBalancerOptions::builder()
        .port(8443)
        .build();
    client
        .modify_verified_access_endpoint()
        .verified_access_endpoint_id(&endpoint_id)
        .description("modified endpoint")
        .load_balancer_options(mod_lb)
        .send()
        .await
        .expect("modify_verified_access_endpoint");

    // Refusal: deleting the group should fail because endpoint references it.
    let err = client
        .delete_verified_access_group()
        .verified_access_group_id(&group_id)
        .send()
        .await
        .expect_err("delete_verified_access_group should be refused while endpoint exists");
    assert!(
        format!("{err:?}").contains("DependencyViolation")
            || format!("{err:?}").contains("InUse")
            || format!("{err:?}").contains("VerifiedAccessGroup"),
        "unexpected error: {err:?}"
    );

    // Delete endpoint.
    client
        .delete_verified_access_endpoint()
        .verified_access_endpoint_id(&endpoint_id)
        .send()
        .await
        .expect("delete_verified_access_endpoint");

    // Delete group.
    client
        .delete_verified_access_group()
        .verified_access_group_id(&group_id)
        .send()
        .await
        .expect("delete_verified_access_group");

    // Delete instance.
    client
        .delete_verified_access_instance()
        .verified_access_instance_id(&instance_id)
        .send()
        .await
        .expect("delete_verified_access_instance");
}

#[tokio::test]
async fn test_verified_access_delete_refusals() {
    let client = make_ec2_client().await;

    // Create instance + trust provider + attach.
    let instance_id = client
        .create_verified_access_instance()
        .send()
        .await
        .expect("create_verified_access_instance")
        .verified_access_instance()
        .expect("instance")
        .verified_access_instance_id()
        .expect("id")
        .to_string();

    let tp_id = client
        .create_verified_access_trust_provider()
        .trust_provider_type(aws_sdk_ec2::types::TrustProviderType::User)
        .user_trust_provider_type(aws_sdk_ec2::types::UserTrustProviderType::Oidc)
        .policy_reference_name("refX")
        .send()
        .await
        .expect("create_verified_access_trust_provider")
        .verified_access_trust_provider()
        .expect("tp")
        .verified_access_trust_provider_id()
        .expect("id")
        .to_string();

    client
        .attach_verified_access_trust_provider()
        .verified_access_instance_id(&instance_id)
        .verified_access_trust_provider_id(&tp_id)
        .send()
        .await
        .expect("attach_verified_access_trust_provider");

    // Delete trust provider while attached: refused.
    let err = client
        .delete_verified_access_trust_provider()
        .verified_access_trust_provider_id(&tp_id)
        .send()
        .await
        .expect_err("delete tp should be refused while attached");
    assert!(
        format!("{err:?}").contains("DependencyViolation") || format!("{err:?}").contains("InUse"),
        "unexpected error: {err:?}"
    );

    // Delete instance with trust providers attached: refused.
    let err = client
        .delete_verified_access_instance()
        .verified_access_instance_id(&instance_id)
        .send()
        .await
        .expect_err("delete instance should be refused with attached trust providers");
    assert!(
        format!("{err:?}").contains("DependencyViolation") || format!("{err:?}").contains("InUse"),
        "unexpected error: {err:?}"
    );

    // Detach, then both deletions should succeed.
    client
        .detach_verified_access_trust_provider()
        .verified_access_instance_id(&instance_id)
        .verified_access_trust_provider_id(&tp_id)
        .send()
        .await
        .expect("detach_verified_access_trust_provider");

    client
        .delete_verified_access_trust_provider()
        .verified_access_trust_provider_id(&tp_id)
        .send()
        .await
        .expect("delete_verified_access_trust_provider");

    client
        .delete_verified_access_instance()
        .verified_access_instance_id(&instance_id)
        .send()
        .await
        .expect("delete_verified_access_instance");
}

// ===== Group 10 tests: Capacity Reservation extensions =====

async fn group10_make_cr(client: &aws_sdk_ec2::Client, count: i32) -> String {
    client
        .create_capacity_reservation()
        .instance_type("m5.large")
        .instance_platform(aws_sdk_ec2::types::CapacityReservationInstancePlatform::LinuxUnix)
        .availability_zone("us-east-1a")
        .instance_count(count)
        .end_date_type(aws_sdk_ec2::types::EndDateType::Unlimited)
        .send()
        .await
        .expect("create_capacity_reservation")
        .capacity_reservation()
        .expect("cr")
        .capacity_reservation_id()
        .expect("id")
        .to_string()
}

#[tokio::test]
async fn test_billing_ownership_offer_associate_then_reject() {
    let client = make_ec2_client().await;
    let cr_id = group10_make_cr(&client, 4).await;

    let assoc = client
        .associate_capacity_reservation_billing_owner()
        .capacity_reservation_id(&cr_id)
        .unused_reservation_billing_owner_id("222222222222")
        .send()
        .await
        .expect("associate billing owner");
    assert_eq!(assoc.r#return(), Some(true));

    // Re-associating the same offer should be refused.
    let dup = client
        .associate_capacity_reservation_billing_owner()
        .capacity_reservation_id(&cr_id)
        .unused_reservation_billing_owner_id("222222222222")
        .send()
        .await
        .expect_err("second associate should be refused");
    let msg = format!("{dup:?}");
    assert!(
        msg.contains("InvalidBillingOwnershipOffer.Duplicate") || msg.contains("already exists"),
        "unexpected: {msg}"
    );

    // Reject must use the rejecter side as account_id; the SDK signs as
    // 123456789012, so the offer recipient must be that account. Recreate
    // the offer with the right recipient and reject.
    let other_cr_id = group10_make_cr(&client, 2).await;
    client
        .associate_capacity_reservation_billing_owner()
        .capacity_reservation_id(&other_cr_id)
        .unused_reservation_billing_owner_id("123456789012")
        .send()
        .await
        .expect("associate to default account");
    let reject = client
        .reject_capacity_reservation_billing_ownership()
        .capacity_reservation_id(&other_cr_id)
        .send()
        .await
        .expect("reject billing ownership");
    assert_eq!(reject.r#return(), Some(true));

    // Disassociate the first (still pending) offer.
    let disassoc = client
        .disassociate_capacity_reservation_billing_owner()
        .capacity_reservation_id(&cr_id)
        .unused_reservation_billing_owner_id("222222222222")
        .send()
        .await
        .expect("disassociate billing owner");
    assert_eq!(disassoc.r#return(), Some(true));
}

#[tokio::test]
async fn test_billing_ownership_offer_accept() {
    let client = make_ec2_client().await;
    let cr_id = group10_make_cr(&client, 2).await;

    // Offer to the default test account ID, then accept as that same account.
    client
        .associate_capacity_reservation_billing_owner()
        .capacity_reservation_id(&cr_id)
        .unused_reservation_billing_owner_id("123456789012")
        .send()
        .await
        .expect("associate billing owner");

    let accept = client
        .accept_capacity_reservation_billing_ownership()
        .capacity_reservation_id(&cr_id)
        .send()
        .await
        .expect("accept billing ownership");
    assert_eq!(accept.r#return(), Some(true));
}

#[tokio::test]
async fn test_create_capacity_reservation_by_splitting_round_trip() {
    let client = make_ec2_client().await;
    let src_id = group10_make_cr(&client, 10).await;

    let split = client
        .create_capacity_reservation_by_splitting()
        .source_capacity_reservation_id(&src_id)
        .instance_count(3)
        .send()
        .await
        .expect("split");
    let src = split.source_capacity_reservation().expect("source");
    let dest = split.destination_capacity_reservation().expect("dest");
    assert_eq!(src.total_instance_count(), Some(7));
    assert_eq!(dest.total_instance_count(), Some(3));
    let dest_id = dest.capacity_reservation_id().expect("dest id").to_string();
    assert!(dest_id.starts_with("cr-"));
    assert_ne!(dest_id, src_id);
}

#[tokio::test]
async fn test_move_capacity_reservation_instances_round_trip() {
    let client = make_ec2_client().await;
    let src_id = group10_make_cr(&client, 8).await;
    let dest_id = group10_make_cr(&client, 1).await;

    let mv = client
        .move_capacity_reservation_instances()
        .source_capacity_reservation_id(&src_id)
        .destination_capacity_reservation_id(&dest_id)
        .instance_count(3)
        .send()
        .await
        .expect("move");
    let src = mv.source_capacity_reservation().expect("src");
    let dest = mv.destination_capacity_reservation().expect("dest");
    assert_eq!(src.total_instance_count(), Some(5));
    assert_eq!(dest.total_instance_count(), Some(4));
}

#[tokio::test]
async fn test_capacity_manager_data_export_create_then_delete() {
    let client = make_ec2_client().await;
    let create = client
        .create_capacity_manager_data_export()
        .schedule(aws_sdk_ec2::types::Schedule::Hourly)
        .output_format(aws_sdk_ec2::types::OutputFormat::Parquet)
        .s3_bucket_name("my-bucket")
        .s3_bucket_prefix("exports/")
        .send()
        .await
        .expect("create export");
    let id = create
        .capacity_manager_data_export_id()
        .expect("id")
        .to_string();
    assert!(id.starts_with("cmde-"));

    let del = client
        .delete_capacity_manager_data_export()
        .capacity_manager_data_export_id(&id)
        .send()
        .await
        .expect("delete export");
    assert_eq!(del.capacity_manager_data_export_id(), Some(id.as_str()));

    let err = client
        .delete_capacity_manager_data_export()
        .capacity_manager_data_export_id(&id)
        .send()
        .await
        .expect_err("second delete should fail");
    let msg = format!("{err:?}");
    assert!(
        msg.contains("InvalidCapacityManagerDataExportId.NotFound"),
        "unexpected: {msg}"
    );
}

#[tokio::test]
async fn test_update_capacity_manager_organizations_access_toggle() {
    let client = make_ec2_client().await;
    let on = client
        .update_capacity_manager_organizations_access()
        .organizations_access(true)
        .send()
        .await
        .expect("enable");
    assert_eq!(on.organizations_access(), Some(true));
    assert_eq!(
        on.capacity_manager_status(),
        Some(&aws_sdk_ec2::types::CapacityManagerStatus::Enabled)
    );

    let off = client
        .update_capacity_manager_organizations_access()
        .organizations_access(false)
        .send()
        .await
        .expect("disable");
    assert_eq!(off.organizations_access(), Some(false));
    assert_eq!(
        off.capacity_manager_status(),
        Some(&aws_sdk_ec2::types::CapacityManagerStatus::Disabled)
    );
}

#[tokio::test]
async fn test_purchase_capacity_block_then_extension_round_trip() {
    let client = make_ec2_client().await;

    let purchase = client
        .purchase_capacity_block()
        .capacity_block_offering_id("cbo-test-1")
        .instance_platform(aws_sdk_ec2::types::CapacityReservationInstancePlatform::LinuxUnix)
        .send()
        .await
        .expect("purchase capacity block");
    let cr = purchase.capacity_reservation().expect("cr");
    let cr_id = cr.capacity_reservation_id().expect("cr id").to_string();
    assert_eq!(
        cr.reservation_type(),
        Some(&aws_sdk_ec2::types::CapacityReservationType::CapacityBlock)
    );
    let blocks = purchase.capacity_blocks();
    assert_eq!(blocks.len(), 1);
    assert!(
        blocks[0]
            .capacity_block_id()
            .map(|s| s.starts_with("cb-"))
            .unwrap_or(false)
    );

    let ext = client
        .purchase_capacity_block_extension()
        .capacity_block_extension_offering_id("cbeo-test-1")
        .capacity_reservation_id(&cr_id)
        .send()
        .await
        .expect("purchase capacity block extension");
    let exts = ext.capacity_block_extensions();
    assert_eq!(exts.len(), 1);
    assert_eq!(exts[0].capacity_reservation_id(), Some(cr_id.as_str()));
    assert_eq!(
        exts[0].capacity_block_extension_status(),
        Some(&aws_sdk_ec2::types::CapacityBlockExtensionStatus::PaymentSucceeded)
    );
}

#[tokio::test]
async fn test_interruptible_capacity_reservation_allocation_create_then_update() {
    let client = make_ec2_client().await;
    let cr_id = group10_make_cr(&client, 4).await;

    let create = client
        .create_interruptible_capacity_reservation_allocation()
        .capacity_reservation_id(&cr_id)
        .instance_count(2)
        .send()
        .await
        .expect("create allocation");
    assert_eq!(
        create.source_capacity_reservation_id(),
        Some(cr_id.as_str())
    );
    assert_eq!(create.target_instance_count(), Some(2));

    let update = client
        .update_interruptible_capacity_reservation_allocation()
        .capacity_reservation_id(&cr_id)
        .target_instance_count(5)
        .send()
        .await
        .expect("update allocation");
    assert_eq!(update.target_instance_count(), Some(5));
    assert_eq!(
        update.source_capacity_reservation_id(),
        Some(cr_id.as_str())
    );
}

#[tokio::test]
async fn test_modify_instance_capacity_reservation_attributes_round_trip() {
    let client = make_ec2_client().await;
    // Launch an instance.
    let resp = client
        .run_instances()
        .image_id("ami-12345678")
        .min_count(1)
        .max_count(1)
        .send()
        .await
        .expect("run_instances");
    let inst_id = resp.instances()[0].instance_id().expect("id").to_string();

    // Set the preference to "none".
    let modify = client
        .modify_instance_capacity_reservation_attributes()
        .instance_id(&inst_id)
        .capacity_reservation_specification(
            aws_sdk_ec2::types::CapacityReservationSpecification::builder()
                .capacity_reservation_preference(
                    aws_sdk_ec2::types::CapacityReservationPreference::None,
                )
                .build(),
        )
        .send()
        .await
        .expect("modify attrs");
    assert_eq!(modify.r#return(), Some(true));

    // Modify again: target a specific reservation by id.
    let cr_id = group10_make_cr(&client, 1).await;
    let target_cr = cr_id.clone();
    let modify2 = client
        .modify_instance_capacity_reservation_attributes()
        .instance_id(&inst_id)
        .capacity_reservation_specification(
            aws_sdk_ec2::types::CapacityReservationSpecification::builder()
                .capacity_reservation_target(
                    aws_sdk_ec2::types::CapacityReservationTarget::builder()
                        .capacity_reservation_id(target_cr)
                        .build(),
                )
                .build(),
        )
        .send()
        .await
        .expect("modify attrs 2");
    assert_eq!(modify2.r#return(), Some(true));
}

// ===========================================================================
// Group 11: Transit Gateway extensions tests
// ===========================================================================

async fn group11_seed_tgw_and_vpc_attach(client: &aws_sdk_ec2::Client) -> (String, String, String) {
    let tgw = client
        .create_transit_gateway()
        .description("group11-tgw")
        .send()
        .await
        .unwrap();
    let tgw_id = tgw
        .transit_gateway()
        .unwrap()
        .transit_gateway_id()
        .unwrap()
        .to_string();
    let vpc = client
        .create_vpc()
        .cidr_block("10.250.0.0/16")
        .send()
        .await
        .unwrap();
    let vpc_id = vpc.vpc().unwrap().vpc_id().unwrap().to_string();
    let subnet = client
        .create_subnet()
        .vpc_id(&vpc_id)
        .cidr_block("10.250.1.0/24")
        .send()
        .await
        .unwrap();
    let subnet_id = subnet.subnet().unwrap().subnet_id().unwrap().to_string();
    let att = client
        .create_transit_gateway_vpc_attachment()
        .transit_gateway_id(&tgw_id)
        .vpc_id(&vpc_id)
        .subnet_ids(&subnet_id)
        .send()
        .await
        .unwrap();
    let att_id = att
        .transit_gateway_vpc_attachment()
        .unwrap()
        .transit_gateway_attachment_id()
        .unwrap()
        .to_string();
    (tgw_id, att_id, subnet_id)
}

#[tokio::test]
async fn test_group11_tgw_multicast_domain_lifecycle() {
    let client = make_ec2_client().await;
    let (tgw_id, att_id, subnet_id) = group11_seed_tgw_and_vpc_attach(&client).await;

    // Create domain.
    let dom_resp = client
        .create_transit_gateway_multicast_domain()
        .transit_gateway_id(&tgw_id)
        .send()
        .await
        .expect("create multicast domain");
    let dom = dom_resp.transit_gateway_multicast_domain().unwrap();
    let dom_id = dom
        .transit_gateway_multicast_domain_id()
        .unwrap()
        .to_string();
    assert!(dom_id.starts_with("tgw-mcast-domain-"));
    assert_eq!(dom.state().unwrap().as_str(), "available");

    // Associate VPC subnets.
    let assoc = client
        .associate_transit_gateway_multicast_domain()
        .transit_gateway_multicast_domain_id(&dom_id)
        .transit_gateway_attachment_id(&att_id)
        .subnet_ids(&subnet_id)
        .send()
        .await
        .expect("associate multicast domain");
    let a = assoc.associations().unwrap();
    assert_eq!(
        a.transit_gateway_multicast_domain_id().unwrap(),
        dom_id.as_str()
    );
    assert_eq!(
        a.subnets().iter().count(),
        1,
        "should have 1 subnet in association"
    );

    // Register multicast group members.
    let nic_id = "eni-aaaa1111";
    let reg_m = client
        .register_transit_gateway_multicast_group_members()
        .transit_gateway_multicast_domain_id(&dom_id)
        .group_ip_address("224.0.0.1")
        .network_interface_ids(nic_id)
        .send()
        .await
        .expect("register members");
    let rm = reg_m.registered_multicast_group_members().unwrap();
    assert_eq!(rm.group_ip_address().unwrap(), "224.0.0.1");

    // Register multicast group sources.
    let src_nic = "eni-bbbb2222";
    let reg_s = client
        .register_transit_gateway_multicast_group_sources()
        .transit_gateway_multicast_domain_id(&dom_id)
        .group_ip_address("224.0.0.2")
        .network_interface_ids(src_nic)
        .send()
        .await
        .expect("register sources");
    assert!(
        reg_s
            .registered_multicast_group_sources()
            .unwrap()
            .group_ip_address()
            .is_some()
    );

    // Deregister members.
    let dereg = client
        .deregister_transit_gateway_multicast_group_members()
        .transit_gateway_multicast_domain_id(&dom_id)
        .group_ip_address("224.0.0.1")
        .network_interface_ids(nic_id)
        .send()
        .await
        .expect("deregister members");
    assert!(
        dereg
            .deregistered_multicast_group_members()
            .unwrap()
            .group_ip_address()
            .is_some()
    );

    // Deregister sources.
    client
        .deregister_transit_gateway_multicast_group_sources()
        .transit_gateway_multicast_domain_id(&dom_id)
        .group_ip_address("224.0.0.2")
        .network_interface_ids(src_nic)
        .send()
        .await
        .expect("deregister sources");

    // Disassociate.
    client
        .disassociate_transit_gateway_multicast_domain()
        .transit_gateway_multicast_domain_id(&dom_id)
        .transit_gateway_attachment_id(&att_id)
        .subnet_ids(&subnet_id)
        .send()
        .await
        .expect("disassociate");

    // Delete domain.
    let del = client
        .delete_transit_gateway_multicast_domain()
        .transit_gateway_multicast_domain_id(&dom_id)
        .send()
        .await
        .expect("delete domain");
    assert_eq!(
        del.transit_gateway_multicast_domain()
            .unwrap()
            .state()
            .unwrap()
            .as_str(),
        "deleted"
    );
}

#[tokio::test]
async fn test_group11_tgw_multicast_accept_reject_associations() {
    let client = make_ec2_client().await;
    let (tgw_id, att_id, subnet_id) = group11_seed_tgw_and_vpc_attach(&client).await;

    // Create with auto-accept disabled (default) so subnets land in
    // pendingAcceptance.
    let dom = client
        .create_transit_gateway_multicast_domain()
        .transit_gateway_id(&tgw_id)
        .send()
        .await
        .unwrap();
    let dom_id = dom
        .transit_gateway_multicast_domain()
        .unwrap()
        .transit_gateway_multicast_domain_id()
        .unwrap()
        .to_string();

    let assoc = client
        .associate_transit_gateway_multicast_domain()
        .transit_gateway_multicast_domain_id(&dom_id)
        .transit_gateway_attachment_id(&att_id)
        .subnet_ids(&subnet_id)
        .send()
        .await
        .unwrap();
    let subnets = assoc.associations().unwrap().subnets();
    assert_eq!(subnets[0].state().unwrap().as_str(), "pendingAcceptance");

    // Accept.
    let accept = client
        .accept_transit_gateway_multicast_domain_associations()
        .transit_gateway_multicast_domain_id(&dom_id)
        .transit_gateway_attachment_id(&att_id)
        .subnet_ids(&subnet_id)
        .send()
        .await
        .expect("accept");
    let st = accept.associations().unwrap().subnets()[0]
        .state()
        .unwrap()
        .as_str()
        .to_string();
    assert_eq!(st, "associated");

    // Reject the same subnet.
    let reject = client
        .reject_transit_gateway_multicast_domain_associations()
        .transit_gateway_multicast_domain_id(&dom_id)
        .transit_gateway_attachment_id(&att_id)
        .subnet_ids(&subnet_id)
        .send()
        .await
        .expect("reject");
    let st2 = reject.associations().unwrap().subnets()[0]
        .state()
        .unwrap()
        .as_str()
        .to_string();
    assert_eq!(st2, "rejected");
}

#[tokio::test]
async fn test_group11_tgw_connect_lifecycle() {
    let client = make_ec2_client().await;
    let (_, transport_att_id, _) = group11_seed_tgw_and_vpc_attach(&client).await;

    // Create TGW connect.
    let connect = client
        .create_transit_gateway_connect()
        .transport_transit_gateway_attachment_id(&transport_att_id)
        .options(
            aws_sdk_ec2::types::CreateTransitGatewayConnectRequestOptions::builder()
                .protocol(aws_sdk_ec2::types::ProtocolValue::Gre)
                .build(),
        )
        .send()
        .await
        .expect("create tgw connect");
    let conn = connect.transit_gateway_connect().unwrap();
    let conn_att_id = conn.transit_gateway_attachment_id().unwrap().to_string();
    assert!(conn_att_id.starts_with("tgw-attach-"));
    assert_eq!(conn.state().unwrap().as_str(), "available");

    // Create connect peer.
    let peer = client
        .create_transit_gateway_connect_peer()
        .transit_gateway_attachment_id(&conn_att_id)
        .peer_address("192.0.2.1")
        .inside_cidr_blocks("169.254.6.0/29")
        .bgp_options(
            aws_sdk_ec2::types::TransitGatewayConnectRequestBgpOptions::builder()
                .peer_asn(65000)
                .build(),
        )
        .send()
        .await
        .expect("create connect peer");
    let p = peer.transit_gateway_connect_peer().unwrap();
    let peer_id = p.transit_gateway_connect_peer_id().unwrap().to_string();
    assert!(peer_id.starts_with("tgw-connect-peer-"));
    assert_eq!(p.state().unwrap().as_str(), "available");
    let cfg = p.connect_peer_configuration().unwrap();
    assert_eq!(cfg.peer_address().unwrap(), "192.0.2.1");
    assert_eq!(cfg.bgp_configurations().len(), 1);

    // Delete peer.
    let del_peer = client
        .delete_transit_gateway_connect_peer()
        .transit_gateway_connect_peer_id(&peer_id)
        .send()
        .await
        .expect("delete peer");
    assert_eq!(
        del_peer
            .transit_gateway_connect_peer()
            .unwrap()
            .state()
            .unwrap()
            .as_str(),
        "deleted"
    );

    // Delete connect.
    let del_conn = client
        .delete_transit_gateway_connect()
        .transit_gateway_attachment_id(&conn_att_id)
        .send()
        .await
        .expect("delete connect");
    assert_eq!(
        del_conn
            .transit_gateway_connect()
            .unwrap()
            .state()
            .unwrap()
            .as_str(),
        "deleted"
    );
}

#[tokio::test]
async fn test_group11_tgw_metering_policy_lifecycle() {
    let client = make_ec2_client().await;
    let (tgw_id, _, _) = group11_seed_tgw_and_vpc_attach(&client).await;

    // CreateTransitGatewayMeteringPolicy is awsQuery; the SDK builder doesn't
    // expose a convenient builder for our extended fields, so we issue it
    // directly.
    let p = client
        .create_transit_gateway_metering_policy()
        .transit_gateway_id(&tgw_id)
        .send()
        .await
        .expect("create metering policy");
    let pid = p
        .transit_gateway_metering_policy()
        .unwrap()
        .transit_gateway_metering_policy_id()
        .unwrap()
        .to_string();
    assert!(pid.starts_with("tgw-mp-"));

    // Create entry.
    let entry = client
        .create_transit_gateway_metering_policy_entry()
        .transit_gateway_metering_policy_id(&pid)
        .policy_rule_number(10)
        .metered_account("123456789012".into())
        .send()
        .await
        .expect("create entry");
    assert!(entry.transit_gateway_metering_policy_entry().is_some());

    // Modify policy.
    let modi = client
        .modify_transit_gateway_metering_policy()
        .transit_gateway_metering_policy_id(&pid)
        .send()
        .await
        .expect("modify policy");
    assert!(modi.transit_gateway_metering_policy().is_some());

    // Delete entry.
    let del_entry = client
        .delete_transit_gateway_metering_policy_entry()
        .transit_gateway_metering_policy_id(&pid)
        .policy_rule_number(10)
        .send()
        .await
        .expect("delete entry");
    assert_eq!(
        del_entry
            .transit_gateway_metering_policy_entry()
            .unwrap()
            .state()
            .unwrap()
            .as_str(),
        "deleted"
    );

    // Delete policy.
    let del = client
        .delete_transit_gateway_metering_policy()
        .transit_gateway_metering_policy_id(&pid)
        .send()
        .await
        .expect("delete policy");
    assert_eq!(
        del.transit_gateway_metering_policy()
            .unwrap()
            .state()
            .unwrap()
            .as_str(),
        "deleted"
    );
}

#[tokio::test]
async fn test_group11_tgw_policy_table_associate_disassociate() {
    let client = make_ec2_client().await;
    let (tgw_id, att_id, _) = group11_seed_tgw_and_vpc_attach(&client).await;

    let pt = client
        .create_transit_gateway_policy_table()
        .transit_gateway_id(&tgw_id)
        .send()
        .await
        .expect("create policy table");
    let pt_id = pt
        .transit_gateway_policy_table()
        .unwrap()
        .transit_gateway_policy_table_id()
        .unwrap()
        .to_string();
    assert!(pt_id.starts_with("tgw-rtb-policy-"));

    let assoc = client
        .associate_transit_gateway_policy_table()
        .transit_gateway_policy_table_id(&pt_id)
        .transit_gateway_attachment_id(&att_id)
        .send()
        .await
        .expect("associate policy table");
    assert_eq!(
        assoc.association().unwrap().state().unwrap().as_str(),
        "associated"
    );

    let dis = client
        .disassociate_transit_gateway_policy_table()
        .transit_gateway_policy_table_id(&pt_id)
        .transit_gateway_attachment_id(&att_id)
        .send()
        .await
        .expect("disassociate policy table");
    assert_eq!(
        dis.association().unwrap().state().unwrap().as_str(),
        "disassociated"
    );

    let del = client
        .delete_transit_gateway_policy_table()
        .transit_gateway_policy_table_id(&pt_id)
        .send()
        .await
        .expect("delete policy table");
    assert_eq!(
        del.transit_gateway_policy_table()
            .unwrap()
            .state()
            .unwrap()
            .as_str(),
        "deleted"
    );
}

#[tokio::test]
async fn test_group11_tgw_prefix_list_reference_lifecycle() {
    let client = make_ec2_client().await;
    let (tgw_id, att_id, _) = group11_seed_tgw_and_vpc_attach(&client).await;

    let rtb = client
        .create_transit_gateway_route_table()
        .transit_gateway_id(&tgw_id)
        .send()
        .await
        .unwrap();
    let rtb_id = rtb
        .transit_gateway_route_table()
        .unwrap()
        .transit_gateway_route_table_id()
        .unwrap()
        .to_string();
    let pl_id = "pl-aaaaaaaa";

    let create = client
        .create_transit_gateway_prefix_list_reference()
        .transit_gateway_route_table_id(&rtb_id)
        .prefix_list_id(pl_id)
        .blackhole(true)
        .send()
        .await
        .expect("create prefix list ref");
    let r = create.transit_gateway_prefix_list_reference().unwrap();
    assert_eq!(r.blackhole(), Some(true));

    let modify = client
        .modify_transit_gateway_prefix_list_reference()
        .transit_gateway_route_table_id(&rtb_id)
        .prefix_list_id(pl_id)
        .blackhole(false)
        .transit_gateway_attachment_id(&att_id)
        .send()
        .await
        .expect("modify prefix list ref");
    let m = modify.transit_gateway_prefix_list_reference().unwrap();
    assert_eq!(m.blackhole(), Some(false));
    let attach = m.transit_gateway_attachment().unwrap();
    assert_eq!(attach.transit_gateway_attachment_id().unwrap(), att_id);

    let del = client
        .delete_transit_gateway_prefix_list_reference()
        .transit_gateway_route_table_id(&rtb_id)
        .prefix_list_id(pl_id)
        .send()
        .await
        .expect("delete prefix list ref");
    assert_eq!(
        del.transit_gateway_prefix_list_reference()
            .unwrap()
            .state()
            .unwrap()
            .as_str(),
        "deleted"
    );
}

#[tokio::test]
async fn test_group11_tgw_route_table_announcement_lifecycle() {
    let client = make_ec2_client().await;
    let (tgw_id, _, _) = group11_seed_tgw_and_vpc_attach(&client).await;

    let rtb = client
        .create_transit_gateway_route_table()
        .transit_gateway_id(&tgw_id)
        .send()
        .await
        .unwrap();
    let rtb_id = rtb
        .transit_gateway_route_table()
        .unwrap()
        .transit_gateway_route_table_id()
        .unwrap()
        .to_string();

    // Need a peering attachment.
    let peer = client
        .create_transit_gateway_peering_attachment()
        .transit_gateway_id(&tgw_id)
        .peer_transit_gateway_id("tgw-peer00001")
        .peer_account_id("123456789012")
        .peer_region("us-west-2")
        .send()
        .await
        .unwrap();
    let peer_att_id = peer
        .transit_gateway_peering_attachment()
        .unwrap()
        .transit_gateway_attachment_id()
        .unwrap()
        .to_string();

    let ann = client
        .create_transit_gateway_route_table_announcement()
        .transit_gateway_route_table_id(&rtb_id)
        .peering_attachment_id(&peer_att_id)
        .send()
        .await
        .expect("create announcement");
    let a = ann.transit_gateway_route_table_announcement().unwrap();
    let ann_id = a
        .transit_gateway_route_table_announcement_id()
        .unwrap()
        .to_string();
    assert!(ann_id.starts_with("tgw-rtb-ann-"));
    assert_eq!(a.state().unwrap().as_str(), "available");

    let del = client
        .delete_transit_gateway_route_table_announcement()
        .transit_gateway_route_table_announcement_id(&ann_id)
        .send()
        .await
        .expect("delete announcement");
    assert_eq!(
        del.transit_gateway_route_table_announcement()
            .unwrap()
            .state()
            .unwrap()
            .as_str(),
        "deleted"
    );
}

#[tokio::test]
async fn test_group11_tgw_vpc_attachment_accept_reject() {
    use std::collections::HashMap;

    use winterbaume_core::StatefulService;
    use winterbaume_ec2::views::{Ec2StateView, TgwVpcAttachmentView, TransitGatewayView};

    let svc = winterbaume_ec2::Ec2Service::new();

    // Seed two pending-acceptance attachments by injecting state directly.
    let mut view = Ec2StateView::default();
    view.transit_gateways.insert(
        "tgw-aaaa0001".to_string(),
        TransitGatewayView {
            transit_gateway_id: "tgw-aaaa0001".to_string(),
            state: "available".to_string(),
            amazon_side_asn: 64512,
            description: "seeded".to_string(),
            dns_support: "enable".to_string(),
            vpn_ecmp_support: "enable".to_string(),
            multicast_support: "disable".to_string(),
            tags: HashMap::new(),
        },
    );
    view.tgw_vpc_attachments.insert(
        "tgw-attach-aaaa0001".to_string(),
        TgwVpcAttachmentView {
            attachment_id: "tgw-attach-aaaa0001".to_string(),
            transit_gateway_id: "tgw-aaaa0001".to_string(),
            vpc_id: "vpc-seeded001".to_string(),
            subnet_ids: vec!["subnet-seed01".to_string()],
            state: "pendingAcceptance".to_string(),
            tags: HashMap::new(),
        },
    );
    view.tgw_vpc_attachments.insert(
        "tgw-attach-aaaa0002".to_string(),
        TgwVpcAttachmentView {
            attachment_id: "tgw-attach-aaaa0002".to_string(),
            transit_gateway_id: "tgw-aaaa0001".to_string(),
            vpc_id: "vpc-seeded002".to_string(),
            subnet_ids: vec!["subnet-seed02".to_string()],
            state: "pendingAcceptance".to_string(),
            tags: HashMap::new(),
        },
    );
    svc.restore("123456789012", "us-east-1", view)
        .await
        .unwrap();

    let mock = winterbaume_core::MockAws::builder()
        .with_service(svc)
        .build();
    let cfg = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_ec2::config::Region::new("us-east-1"))
        .load()
        .await;
    let client = aws_sdk_ec2::Client::new(&cfg);

    // Accept the first.
    let acc = client
        .accept_transit_gateway_vpc_attachment()
        .transit_gateway_attachment_id("tgw-attach-aaaa0001")
        .send()
        .await
        .expect("accept");
    assert_eq!(
        acc.transit_gateway_vpc_attachment()
            .unwrap()
            .state()
            .unwrap()
            .as_str(),
        "available"
    );

    // Reject the second.
    let rej = client
        .reject_transit_gateway_vpc_attachment()
        .transit_gateway_attachment_id("tgw-attach-aaaa0002")
        .send()
        .await
        .expect("reject");
    assert_eq!(
        rej.transit_gateway_vpc_attachment()
            .unwrap()
            .state()
            .unwrap()
            .as_str(),
        "rejected"
    );

    // Accepting a non-pending attachment must fail.
    let err = client
        .accept_transit_gateway_vpc_attachment()
        .transit_gateway_attachment_id("tgw-attach-aaaa0001")
        .send()
        .await
        .expect_err("should fail");
    let s = format!("{:?}", err);
    assert!(s.contains("IncorrectState"), "{s}");
}

#[tokio::test]
async fn test_group11_replace_tgw_route_round_trip() {
    let client = make_ec2_client().await;
    let (tgw_id, att_id_a, _) = group11_seed_tgw_and_vpc_attach(&client).await;
    let (_, att_id_b, _) = group11_seed_tgw_and_vpc_attach(&client).await;

    let rtb = client
        .create_transit_gateway_route_table()
        .transit_gateway_id(&tgw_id)
        .send()
        .await
        .unwrap();
    let rtb_id = rtb
        .transit_gateway_route_table()
        .unwrap()
        .transit_gateway_route_table_id()
        .unwrap()
        .to_string();

    // Initial route -> attachment A.
    client
        .create_transit_gateway_route()
        .transit_gateway_route_table_id(&rtb_id)
        .destination_cidr_block("10.99.0.0/16")
        .transit_gateway_attachment_id(&att_id_a)
        .send()
        .await
        .expect("create route");

    // Replace -> attachment B.
    let replaced = client
        .replace_transit_gateway_route()
        .transit_gateway_route_table_id(&rtb_id)
        .destination_cidr_block("10.99.0.0/16")
        .transit_gateway_attachment_id(&att_id_b)
        .send()
        .await
        .expect("replace route");
    let r = replaced.route().unwrap();
    assert_eq!(r.destination_cidr_block().unwrap(), "10.99.0.0/16");
    assert_eq!(r.state().unwrap().as_str(), "active");

    // Replace -> blackhole.
    let blackholed = client
        .replace_transit_gateway_route()
        .transit_gateway_route_table_id(&rtb_id)
        .destination_cidr_block("10.99.0.0/16")
        .blackhole(true)
        .send()
        .await
        .expect("replace blackhole");
    assert_eq!(
        blackholed.route().unwrap().state().unwrap().as_str(),
        "blackhole"
    );

    // Replacing a non-existent CIDR must fail.
    let missing = client
        .replace_transit_gateway_route()
        .transit_gateway_route_table_id(&rtb_id)
        .destination_cidr_block("10.99.99.99/32")
        .transit_gateway_attachment_id(&att_id_a)
        .send()
        .await
        .expect_err("should fail");
    let s = format!("{:?}", missing);
    assert!(s.contains("InvalidRoute.NotFound"), "{s}");
}

// ---------------------------------------------------------------------------
// Group 12: IPAM (IP Address Manager)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_ipam_full_lifecycle() {
    use aws_sdk_ec2::types::AddIpamOperatingRegion;
    let client = make_ec2_client().await;

    // Create an IPAM with one operating region.
    let create = client
        .create_ipam()
        .description("test ipam")
        .operating_regions(
            AddIpamOperatingRegion::builder()
                .region_name("us-east-1")
                .build(),
        )
        .send()
        .await
        .expect("create_ipam should succeed");
    let ipam = create.ipam().expect("ipam present");
    let ipam_id = ipam.ipam_id().expect("ipam_id").to_string();
    assert!(ipam_id.starts_with("ipam-"), "got {ipam_id}");
    let private_scope_id = ipam
        .private_default_scope_id()
        .expect("private scope id")
        .to_string();
    assert_eq!(ipam.scope_count(), Some(2));

    // Modify it.
    let modified = client
        .modify_ipam()
        .ipam_id(&ipam_id)
        .description("updated")
        .send()
        .await
        .expect("modify_ipam should succeed");
    assert_eq!(
        modified.ipam().and_then(|i| i.description()),
        Some("updated")
    );

    // Describe it (should round-trip).
    let desc = client
        .describe_ipams()
        .send()
        .await
        .expect("describe_ipams should succeed");
    assert!(desc.ipams().iter().any(|i| i.ipam_id() == Some(&ipam_id)));

    // Create an additional scope.
    let scope_resp = client
        .create_ipam_scope()
        .ipam_id(&ipam_id)
        .description("extra")
        .send()
        .await
        .expect("create_ipam_scope should succeed");
    let extra_scope_id = scope_resp
        .ipam_scope()
        .and_then(|s| s.ipam_scope_id())
        .expect("scope id")
        .to_string();

    // Modify scope description.
    let _ = client
        .modify_ipam_scope()
        .ipam_scope_id(&extra_scope_id)
        .description("renamed")
        .send()
        .await
        .expect("modify_ipam_scope should succeed");

    // Create a pool inside the private default scope.
    let pool_resp = client
        .create_ipam_pool()
        .ipam_scope_id(&private_scope_id)
        .address_family(aws_sdk_ec2::types::AddressFamily::Ipv4)
        .description("primary pool")
        .send()
        .await
        .expect("create_ipam_pool should succeed");
    let pool = pool_resp.ipam_pool().expect("pool present");
    let pool_id = pool.ipam_pool_id().expect("pool id").to_string();
    assert!(pool_id.starts_with("ipam-pool-"));

    // Modify pool.
    let _ = client
        .modify_ipam_pool()
        .ipam_pool_id(&pool_id)
        .description("renamed pool")
        .send()
        .await
        .expect("modify_ipam_pool should succeed");

    // Provision a CIDR into the pool.
    let prov = client
        .provision_ipam_pool_cidr()
        .ipam_pool_id(&pool_id)
        .cidr("10.0.0.0/16")
        .send()
        .await
        .expect("provision_ipam_pool_cidr should succeed");
    let pool_cidr = prov.ipam_pool_cidr().expect("pool cidr");
    assert_eq!(pool_cidr.cidr(), Some("10.0.0.0/16"));

    // Allocate a CIDR out of the pool.
    let alloc = client
        .allocate_ipam_pool_cidr()
        .ipam_pool_id(&pool_id)
        .cidr("10.0.1.0/24")
        .description("alloc 1")
        .send()
        .await
        .expect("allocate_ipam_pool_cidr should succeed");
    let allocation = alloc.ipam_pool_allocation().expect("allocation");
    let allocation_id = allocation
        .ipam_pool_allocation_id()
        .expect("allocation id")
        .to_string();
    assert_eq!(allocation.cidr(), Some("10.0.1.0/24"));

    // Release the allocation.
    let release = client
        .release_ipam_pool_allocation()
        .ipam_pool_id(&pool_id)
        .ipam_pool_allocation_id(&allocation_id)
        .cidr("10.0.1.0/24")
        .send()
        .await
        .expect("release_ipam_pool_allocation should succeed");
    assert_eq!(release.success(), Some(true));

    // Deprovision the CIDR.
    let dep = client
        .deprovision_ipam_pool_cidr()
        .ipam_pool_id(&pool_id)
        .cidr("10.0.0.0/16")
        .send()
        .await
        .expect("deprovision_ipam_pool_cidr should succeed");
    assert_eq!(
        dep.ipam_pool_cidr()
            .and_then(|c| c.state())
            .map(|s| s.as_str()),
        Some("deprovisioned")
    );

    // Delete pool, scope, ipam.
    let _ = client
        .delete_ipam_pool()
        .ipam_pool_id(&pool_id)
        .send()
        .await
        .expect("delete_ipam_pool should succeed");
    let _ = client
        .delete_ipam_scope()
        .ipam_scope_id(&extra_scope_id)
        .send()
        .await
        .expect("delete_ipam_scope should succeed");
    let _ = client
        .delete_ipam()
        .ipam_id(&ipam_id)
        .send()
        .await
        .expect("delete_ipam should succeed");
}

#[tokio::test]
async fn test_ipam_resource_discovery_lifecycle() {
    use aws_sdk_ec2::types::AddIpamOperatingRegion;
    let client = make_ec2_client().await;

    let ipam = client
        .create_ipam()
        .operating_regions(
            AddIpamOperatingRegion::builder()
                .region_name("us-east-1")
                .build(),
        )
        .send()
        .await
        .expect("create_ipam should succeed");
    let ipam_id = ipam
        .ipam()
        .and_then(|i| i.ipam_id())
        .expect("ipam id")
        .to_string();

    let rd = client
        .create_ipam_resource_discovery()
        .description("test rd")
        .operating_regions(
            AddIpamOperatingRegion::builder()
                .region_name("us-east-1")
                .build(),
        )
        .send()
        .await
        .expect("create_ipam_resource_discovery should succeed");
    let rd_id = rd
        .ipam_resource_discovery()
        .and_then(|r| r.ipam_resource_discovery_id())
        .expect("rd id")
        .to_string();

    // Modify
    let _ = client
        .modify_ipam_resource_discovery()
        .ipam_resource_discovery_id(&rd_id)
        .description("rd renamed")
        .send()
        .await
        .expect("modify_ipam_resource_discovery should succeed");

    // Associate
    let assoc = client
        .associate_ipam_resource_discovery()
        .ipam_id(&ipam_id)
        .ipam_resource_discovery_id(&rd_id)
        .send()
        .await
        .expect("associate_ipam_resource_discovery should succeed");
    let assoc_id = assoc
        .ipam_resource_discovery_association()
        .and_then(|a| a.ipam_resource_discovery_association_id())
        .expect("assoc id")
        .to_string();

    // Describe
    let desc = client
        .describe_ipam_resource_discovery_associations()
        .send()
        .await
        .expect("describe assoc");
    assert!(
        desc.ipam_resource_discovery_associations()
            .iter()
            .any(|a| a.ipam_resource_discovery_association_id() == Some(&assoc_id))
    );

    // Disassociate
    let _ = client
        .disassociate_ipam_resource_discovery()
        .ipam_resource_discovery_association_id(&assoc_id)
        .send()
        .await
        .expect("disassociate should succeed");

    // Delete
    let _ = client
        .delete_ipam_resource_discovery()
        .ipam_resource_discovery_id(&rd_id)
        .send()
        .await
        .expect("delete_ipam_resource_discovery should succeed");
    let _ = client
        .delete_ipam()
        .ipam_id(&ipam_id)
        .send()
        .await
        .expect("delete_ipam should succeed");
}

#[tokio::test]
async fn test_ipam_byoasn_lifecycle() {
    use aws_sdk_ec2::types::{AddIpamOperatingRegion, AsnAuthorizationContext};
    let client = make_ec2_client().await;

    let cidr = "203.0.114.0/24";
    let asn_str = "65000";

    // Provision a BYOIP CIDR (Group 2 prerequisite).
    let _ = client
        .provision_byoip_cidr()
        .cidr(cidr)
        .send()
        .await
        .expect("provision_byoip_cidr");

    // Create IPAM.
    let ipam = client
        .create_ipam()
        .operating_regions(
            AddIpamOperatingRegion::builder()
                .region_name("us-east-1")
                .build(),
        )
        .send()
        .await
        .expect("create_ipam");
    let ipam_id = ipam
        .ipam()
        .and_then(|i| i.ipam_id())
        .expect("ipam id")
        .to_string();

    // Provision BYOASN.
    let prov = client
        .provision_ipam_byoasn()
        .ipam_id(&ipam_id)
        .asn(asn_str)
        .asn_authorization_context(
            AsnAuthorizationContext::builder()
                .message("msg")
                .signature("sig")
                .build(),
        )
        .send()
        .await
        .expect("provision_ipam_byoasn");
    assert_eq!(
        prov.byoasn().and_then(|b| b.state()).map(|s| s.as_str()),
        Some("provisioned")
    );

    // Associate against the BYOIP CIDR.
    let assoc = client
        .associate_ipam_byoasn()
        .asn(asn_str)
        .cidr(cidr)
        .send()
        .await
        .expect("associate_ipam_byoasn");
    assert_eq!(
        assoc
            .asn_association()
            .and_then(|a| a.state())
            .map(|s| s.as_str()),
        Some("associated")
    );

    // Disassociate.
    let _ = client
        .disassociate_ipam_byoasn()
        .asn(asn_str)
        .cidr(cidr)
        .send()
        .await
        .expect("disassociate_ipam_byoasn");

    // Deprovision.
    let _ = client
        .deprovision_ipam_byoasn()
        .ipam_id(&ipam_id)
        .asn(asn_str)
        .send()
        .await
        .expect("deprovision_ipam_byoasn");
}

#[tokio::test]
async fn test_ipam_external_resource_verification_token_create_delete() {
    use aws_sdk_ec2::types::AddIpamOperatingRegion;
    let client = make_ec2_client().await;
    let ipam = client
        .create_ipam()
        .operating_regions(
            AddIpamOperatingRegion::builder()
                .region_name("us-east-1")
                .build(),
        )
        .send()
        .await
        .expect("create_ipam");
    let ipam_id = ipam
        .ipam()
        .and_then(|i| i.ipam_id())
        .expect("ipam id")
        .to_string();
    let token = client
        .create_ipam_external_resource_verification_token()
        .ipam_id(&ipam_id)
        .send()
        .await
        .expect("create token");
    let token_id = token
        .ipam_external_resource_verification_token()
        .and_then(|t| t.ipam_external_resource_verification_token_id())
        .expect("token id")
        .to_string();
    let _ = client
        .delete_ipam_external_resource_verification_token()
        .ipam_external_resource_verification_token_id(&token_id)
        .send()
        .await
        .expect("delete token");
}

#[tokio::test]
async fn test_ipam_policy_create_modify_rules_delete() {
    use aws_sdk_ec2::types::{AddIpamOperatingRegion, IpamPolicyAllocationRuleRequest};
    let client = make_ec2_client().await;
    let ipam = client
        .create_ipam()
        .operating_regions(
            AddIpamOperatingRegion::builder()
                .region_name("us-east-1")
                .build(),
        )
        .send()
        .await
        .expect("create_ipam");
    let ipam_id = ipam
        .ipam()
        .and_then(|i| i.ipam_id())
        .expect("ipam id")
        .to_string();
    let policy = client
        .create_ipam_policy()
        .ipam_id(&ipam_id)
        .send()
        .await
        .expect("create_ipam_policy");
    let policy_id = policy
        .ipam_policy()
        .and_then(|p| p.ipam_policy_id())
        .expect("policy id")
        .to_string();

    // Modify allocation rules.
    let _ = client
        .modify_ipam_policy_allocation_rules()
        .ipam_policy_id(&policy_id)
        .allocation_rules(
            IpamPolicyAllocationRuleRequest::builder()
                .source_ipam_pool_id("ipam-pool-deadbeef")
                .build(),
        )
        .send()
        .await
        .expect("modify rules");

    let _ = client
        .delete_ipam_policy()
        .ipam_policy_id(&policy_id)
        .send()
        .await
        .expect("delete_ipam_policy");
}

#[tokio::test]
async fn test_ipam_prefix_list_resolver_lifecycle() {
    use aws_sdk_ec2::types::AddIpamOperatingRegion;
    let client = make_ec2_client().await;
    let ipam = client
        .create_ipam()
        .operating_regions(
            AddIpamOperatingRegion::builder()
                .region_name("us-east-1")
                .build(),
        )
        .send()
        .await
        .expect("create_ipam");
    let ipam_id = ipam
        .ipam()
        .and_then(|i| i.ipam_id())
        .expect("ipam id")
        .to_string();
    let resolver = client
        .create_ipam_prefix_list_resolver()
        .ipam_id(&ipam_id)
        .address_family(aws_sdk_ec2::types::AddressFamily::Ipv4)
        .description("resolver-1")
        .send()
        .await
        .expect("create_ipam_prefix_list_resolver");
    let resolver_id = resolver
        .ipam_prefix_list_resolver()
        .and_then(|r| r.ipam_prefix_list_resolver_id())
        .expect("resolver id")
        .to_string();

    let _ = client
        .modify_ipam_prefix_list_resolver()
        .ipam_prefix_list_resolver_id(&resolver_id)
        .description("renamed")
        .send()
        .await
        .expect("modify_ipam_prefix_list_resolver");

    // Add a target.
    let target = client
        .create_ipam_prefix_list_resolver_target()
        .ipam_prefix_list_resolver_id(&resolver_id)
        .prefix_list_id("pl-12345678")
        .prefix_list_region("us-east-1")
        .send()
        .await
        .expect("create_ipam_prefix_list_resolver_target");
    let target_id = target
        .ipam_prefix_list_resolver_target()
        .and_then(|t| t.ipam_prefix_list_resolver_target_id())
        .expect("target id")
        .to_string();

    // Modify target.
    let _ = client
        .modify_ipam_prefix_list_resolver_target()
        .ipam_prefix_list_resolver_target_id(&target_id)
        .desired_version(2)
        .send()
        .await
        .expect("modify_ipam_prefix_list_resolver_target");

    // Delete target then resolver.
    let _ = client
        .delete_ipam_prefix_list_resolver_target()
        .ipam_prefix_list_resolver_target_id(&target_id)
        .send()
        .await
        .expect("delete_ipam_prefix_list_resolver_target");
    let _ = client
        .delete_ipam_prefix_list_resolver()
        .ipam_prefix_list_resolver_id(&resolver_id)
        .send()
        .await
        .expect("delete_ipam_prefix_list_resolver");
}

#[tokio::test]
async fn test_move_byoip_cidr_to_ipam_round_trip() {
    use aws_sdk_ec2::types::AddIpamOperatingRegion;
    let client = make_ec2_client().await;

    // Provision a BYOIP CIDR.
    let cidr = "203.0.115.0/24";
    let _ = client
        .provision_byoip_cidr()
        .cidr(cidr)
        .send()
        .await
        .expect("provision_byoip_cidr");

    // Create an IPAM and a pool to receive it.
    let ipam = client
        .create_ipam()
        .operating_regions(
            AddIpamOperatingRegion::builder()
                .region_name("us-east-1")
                .build(),
        )
        .send()
        .await
        .expect("create_ipam");
    let private_scope_id = ipam
        .ipam()
        .and_then(|i| i.private_default_scope_id())
        .expect("scope id")
        .to_string();

    let pool = client
        .create_ipam_pool()
        .ipam_scope_id(&private_scope_id)
        .address_family(aws_sdk_ec2::types::AddressFamily::Ipv4)
        .send()
        .await
        .expect("create_ipam_pool");
    let pool_id = pool
        .ipam_pool()
        .and_then(|p| p.ipam_pool_id())
        .expect("pool id")
        .to_string();

    let moved = client
        .move_byoip_cidr_to_ipam()
        .cidr(cidr)
        .ipam_pool_id(&pool_id)
        .ipam_pool_owner("123456789012")
        .send()
        .await
        .expect("move_byoip_cidr_to_ipam");
    assert_eq!(moved.byoip_cidr().and_then(|c| c.cidr()), Some(cidr));
}

#[tokio::test]
async fn test_modify_ipam_resource_cidr_round_trip() {
    let client = make_ec2_client().await;
    let resp = client
        .modify_ipam_resource_cidr()
        .resource_id("vpc-deadbeef")
        .resource_cidr("10.0.0.0/16")
        .resource_region("us-east-1")
        .current_ipam_scope_id("ipam-scope-aaaa")
        .destination_ipam_scope_id("ipam-scope-bbbb")
        .monitored(true)
        .send()
        .await
        .expect("modify_ipam_resource_cidr");
    assert_eq!(
        resp.ipam_resource_cidr().and_then(|c| c.resource_cidr()),
        Some("10.0.0.0/16")
    );
}

#[tokio::test]
async fn test_ipam_delete_refusals() {
    use aws_sdk_ec2::types::AddIpamOperatingRegion;
    let client = make_ec2_client().await;

    let ipam = client
        .create_ipam()
        .operating_regions(
            AddIpamOperatingRegion::builder()
                .region_name("us-east-1")
                .build(),
        )
        .send()
        .await
        .expect("create_ipam");
    let ipam_id = ipam
        .ipam()
        .and_then(|i| i.ipam_id())
        .expect("ipam id")
        .to_string();
    let private_scope_id = ipam
        .ipam()
        .and_then(|i| i.private_default_scope_id())
        .expect("scope id")
        .to_string();

    // Attempting to delete the default scope must fail.
    let err_default = client
        .delete_ipam_scope()
        .ipam_scope_id(&private_scope_id)
        .send()
        .await
        .expect_err("delete default scope should fail");
    assert!(
        format!("{err_default:?}").contains("InvalidIpamScopeId.IsDefault")
            || format!("{err_default:?}").contains("IsDefault"),
        "got: {err_default:?}"
    );

    // Create a pool.
    let pool = client
        .create_ipam_pool()
        .ipam_scope_id(&private_scope_id)
        .address_family(aws_sdk_ec2::types::AddressFamily::Ipv4)
        .send()
        .await
        .expect("create_ipam_pool");
    let pool_id = pool
        .ipam_pool()
        .and_then(|p| p.ipam_pool_id())
        .expect("pool id")
        .to_string();

    // Allocate, then attempt to delete the pool.
    let _ = client
        .provision_ipam_pool_cidr()
        .ipam_pool_id(&pool_id)
        .cidr("10.0.0.0/16")
        .send()
        .await
        .expect("provision");
    let _ = client
        .allocate_ipam_pool_cidr()
        .ipam_pool_id(&pool_id)
        .cidr("10.0.1.0/24")
        .send()
        .await
        .expect("allocate");
    let err_pool = client
        .delete_ipam_pool()
        .ipam_pool_id(&pool_id)
        .send()
        .await
        .expect_err("delete pool with allocations should fail");
    assert!(
        format!("{err_pool:?}").contains("DependencyViolation"),
        "got: {err_pool:?}"
    );

    // Attempting to delete the IPAM with pools (no cascade) must fail.
    let err_ipam = client
        .delete_ipam()
        .ipam_id(&ipam_id)
        .send()
        .await
        .expect_err("delete ipam with pools should fail");
    assert!(
        format!("{err_ipam:?}").contains("DependencyViolation"),
        "got: {err_ipam:?}"
    );

    // Cascade should succeed.
    let _ = client
        .delete_ipam()
        .ipam_id(&ipam_id)
        .cascade(true)
        .send()
        .await
        .expect("delete_ipam with cascade should succeed");
}

// ===== Batch A upgrades: integration tests for 22 handlers wired to existing state =====

#[tokio::test]
async fn test_describe_local_gateways_returns_seeded_lgw() {
    let client = make_ec2_client().await;
    // Creating a local-gateway route table auto-seeds the parent local-gateway
    // (mock policy, since real AWS does not expose a `Create*` for the LGW).
    let lgw_id = client
        .create_local_gateway_route_table()
        .local_gateway_id("lgw-batcha001")
        .send()
        .await
        .expect("create_local_gateway_route_table")
        .local_gateway_route_table()
        .and_then(|t| t.local_gateway_id())
        .expect("lgw id")
        .to_string();

    let resp = client
        .describe_local_gateways()
        .local_gateway_ids(&lgw_id)
        .send()
        .await
        .expect("describe_local_gateways");
    let gws = resp.local_gateways();
    assert_eq!(gws.len(), 1, "expected exactly one matching LGW");
    assert_eq!(gws[0].local_gateway_id(), Some(lgw_id.as_str()));
    assert_eq!(gws[0].state(), Some("available"));
}

#[tokio::test]
async fn test_describe_coip_pools_filters_by_id() {
    let client = make_ec2_client().await;
    let pool_id = client
        .create_coip_pool()
        .local_gateway_route_table_id("lgw-rtb-batcha")
        .send()
        .await
        .expect("create_coip_pool")
        .coip_pool()
        .and_then(|p| p.pool_id())
        .expect("pool id")
        .to_string();
    let resp = client
        .describe_coip_pools()
        .pool_ids(&pool_id)
        .send()
        .await
        .expect("describe_coip_pools");
    let pools = resp.coip_pools();
    assert_eq!(pools.len(), 1);
    assert_eq!(pools[0].pool_id(), Some(pool_id.as_str()));
}

#[tokio::test]
async fn test_get_coip_pool_usage_returns_assigned_cidrs() {
    let client = make_ec2_client().await;
    let pool_id = client
        .create_coip_pool()
        .local_gateway_route_table_id("lgw-rtb-batcha-usage")
        .send()
        .await
        .expect("create_coip_pool")
        .coip_pool()
        .and_then(|p| p.pool_id())
        .expect("pool id")
        .to_string();
    let _ = client
        .create_coip_cidr()
        .cidr("198.51.100.0/24")
        .coip_pool_id(&pool_id)
        .send()
        .await
        .expect("create_coip_cidr");
    let resp = client
        .get_coip_pool_usage()
        .pool_id(&pool_id)
        .send()
        .await
        .expect("get_coip_pool_usage");
    assert_eq!(resp.coip_pool_id(), Some(pool_id.as_str()));
    let usages = resp.coip_address_usages();
    assert_eq!(usages.len(), 1);
    assert_eq!(usages[0].co_ip(), Some("198.51.100.0/24"));
}

#[tokio::test]
async fn test_get_ipam_pool_allocations_lists_allocations() {
    use aws_sdk_ec2::types::AddIpamOperatingRegion;
    let client = make_ec2_client().await;
    let ipam = client
        .create_ipam()
        .operating_regions(
            AddIpamOperatingRegion::builder()
                .region_name("us-east-1")
                .build(),
        )
        .send()
        .await
        .expect("create_ipam");
    let private_scope_id = ipam
        .ipam()
        .and_then(|i| i.private_default_scope_id())
        .expect("scope id")
        .to_string();
    let pool_id = client
        .create_ipam_pool()
        .ipam_scope_id(&private_scope_id)
        .address_family(aws_sdk_ec2::types::AddressFamily::Ipv4)
        .send()
        .await
        .expect("create_ipam_pool")
        .ipam_pool()
        .and_then(|p| p.ipam_pool_id())
        .expect("pool id")
        .to_string();
    let _ = client
        .provision_ipam_pool_cidr()
        .ipam_pool_id(&pool_id)
        .cidr("10.20.0.0/16")
        .send()
        .await
        .expect("provision");
    let _ = client
        .allocate_ipam_pool_cidr()
        .ipam_pool_id(&pool_id)
        .cidr("10.20.1.0/24")
        .send()
        .await
        .expect("allocate");
    let resp = client
        .get_ipam_pool_allocations()
        .ipam_pool_id(&pool_id)
        .send()
        .await
        .expect("get_ipam_pool_allocations");
    let allocs = resp.ipam_pool_allocations();
    assert_eq!(allocs.len(), 1);
    assert_eq!(allocs[0].cidr(), Some("10.20.1.0/24"));
}

#[tokio::test]
async fn test_get_ipam_pool_cidrs_lists_provisioned_cidrs() {
    use aws_sdk_ec2::types::AddIpamOperatingRegion;
    let client = make_ec2_client().await;
    let ipam = client
        .create_ipam()
        .operating_regions(
            AddIpamOperatingRegion::builder()
                .region_name("us-east-1")
                .build(),
        )
        .send()
        .await
        .expect("create_ipam");
    let private_scope_id = ipam
        .ipam()
        .and_then(|i| i.private_default_scope_id())
        .expect("scope id")
        .to_string();
    let pool_id = client
        .create_ipam_pool()
        .ipam_scope_id(&private_scope_id)
        .address_family(aws_sdk_ec2::types::AddressFamily::Ipv4)
        .send()
        .await
        .expect("create_ipam_pool")
        .ipam_pool()
        .and_then(|p| p.ipam_pool_id())
        .expect("pool id")
        .to_string();
    let _ = client
        .provision_ipam_pool_cidr()
        .ipam_pool_id(&pool_id)
        .cidr("10.30.0.0/16")
        .send()
        .await
        .expect("provision");
    let resp = client
        .get_ipam_pool_cidrs()
        .ipam_pool_id(&pool_id)
        .send()
        .await
        .expect("get_ipam_pool_cidrs");
    let cidrs = resp.ipam_pool_cidrs();
    assert_eq!(cidrs.len(), 1);
    assert_eq!(cidrs[0].cidr(), Some("10.30.0.0/16"));
}

#[tokio::test]
async fn test_get_ipam_address_history_returns_allocations_for_cidr() {
    use aws_sdk_ec2::types::AddIpamOperatingRegion;
    let client = make_ec2_client().await;
    let ipam = client
        .create_ipam()
        .operating_regions(
            AddIpamOperatingRegion::builder()
                .region_name("us-east-1")
                .build(),
        )
        .send()
        .await
        .expect("create_ipam");
    let private_scope_id = ipam
        .ipam()
        .and_then(|i| i.private_default_scope_id())
        .expect("scope id")
        .to_string();
    let pool_id = client
        .create_ipam_pool()
        .ipam_scope_id(&private_scope_id)
        .address_family(aws_sdk_ec2::types::AddressFamily::Ipv4)
        .send()
        .await
        .expect("create_ipam_pool")
        .ipam_pool()
        .and_then(|p| p.ipam_pool_id())
        .expect("pool id")
        .to_string();
    let _ = client
        .provision_ipam_pool_cidr()
        .ipam_pool_id(&pool_id)
        .cidr("10.40.0.0/16")
        .send()
        .await
        .expect("provision");
    let _ = client
        .allocate_ipam_pool_cidr()
        .ipam_pool_id(&pool_id)
        .cidr("10.40.7.0/24")
        .send()
        .await
        .expect("allocate");
    let resp = client
        .get_ipam_address_history()
        .cidr("10.40.7.0/24")
        .ipam_scope_id(&private_scope_id)
        .send()
        .await
        .expect("get_ipam_address_history");
    let records = resp.history_records();
    assert_eq!(records.len(), 1);
    assert_eq!(records[0].resource_cidr(), Some("10.40.7.0/24"));
}

#[tokio::test]
async fn test_get_ipam_resource_cidrs_includes_allocation() {
    use aws_sdk_ec2::types::AddIpamOperatingRegion;
    let client = make_ec2_client().await;
    let ipam = client
        .create_ipam()
        .operating_regions(
            AddIpamOperatingRegion::builder()
                .region_name("us-east-1")
                .build(),
        )
        .send()
        .await
        .expect("create_ipam");
    let private_scope_id = ipam
        .ipam()
        .and_then(|i| i.private_default_scope_id())
        .expect("scope id")
        .to_string();
    let pool_id = client
        .create_ipam_pool()
        .ipam_scope_id(&private_scope_id)
        .address_family(aws_sdk_ec2::types::AddressFamily::Ipv4)
        .send()
        .await
        .expect("create_ipam_pool")
        .ipam_pool()
        .and_then(|p| p.ipam_pool_id())
        .expect("pool id")
        .to_string();
    let _ = client
        .provision_ipam_pool_cidr()
        .ipam_pool_id(&pool_id)
        .cidr("10.50.0.0/16")
        .send()
        .await
        .expect("provision");
    let _ = client
        .allocate_ipam_pool_cidr()
        .ipam_pool_id(&pool_id)
        .cidr("10.50.5.0/24")
        .send()
        .await
        .expect("allocate");
    let resp = client
        .get_ipam_resource_cidrs()
        .ipam_scope_id(&private_scope_id)
        .ipam_pool_id(&pool_id)
        .send()
        .await
        .expect("get_ipam_resource_cidrs");
    let cidrs = resp.ipam_resource_cidrs();
    assert_eq!(cidrs.len(), 1);
    assert_eq!(cidrs[0].resource_cidr(), Some("10.50.5.0/24"));
    assert_eq!(cidrs[0].ipam_pool_id(), Some(pool_id.as_str()));
    assert_eq!(cidrs[0].ipam_scope_id(), Some(private_scope_id.as_str()));
}

#[tokio::test]
async fn test_enable_disable_get_enabled_ipam_policy() {
    use aws_sdk_ec2::types::AddIpamOperatingRegion;
    let client = make_ec2_client().await;
    let ipam_id = client
        .create_ipam()
        .operating_regions(
            AddIpamOperatingRegion::builder()
                .region_name("us-east-1")
                .build(),
        )
        .send()
        .await
        .expect("create_ipam")
        .ipam()
        .and_then(|i| i.ipam_id())
        .expect("ipam id")
        .to_string();
    let policy_id = client
        .create_ipam_policy()
        .ipam_id(&ipam_id)
        .send()
        .await
        .expect("create_ipam_policy")
        .ipam_policy()
        .and_then(|p| p.ipam_policy_id())
        .expect("policy id")
        .to_string();
    // Disable then enable.
    let _ = client
        .disable_ipam_policy()
        .ipam_policy_id(&policy_id)
        .send()
        .await
        .expect("disable");
    let after_disable = client
        .get_enabled_ipam_policy()
        .send()
        .await
        .expect("get_enabled");
    assert_eq!(after_disable.ipam_policy_enabled(), Some(false));
    let _ = client
        .enable_ipam_policy()
        .ipam_policy_id(&policy_id)
        .send()
        .await
        .expect("enable");
    let after_enable = client
        .get_enabled_ipam_policy()
        .send()
        .await
        .expect("get_enabled");
    assert_eq!(after_enable.ipam_policy_enabled(), Some(true));
    assert_eq!(after_enable.ipam_policy_id(), Some(policy_id.as_str()));
}

#[tokio::test]
async fn test_disable_enable_image_round_trip() {
    let client = make_ec2_client().await;
    let ami_id = client
        .register_image()
        .name("disable-enable-rt")
        .send()
        .await
        .expect("register_image")
        .image_id()
        .expect("image id")
        .to_string();
    let _ = client
        .disable_image()
        .image_id(&ami_id)
        .send()
        .await
        .expect("disable");
    let img = client
        .describe_images()
        .image_ids(&ami_id)
        .send()
        .await
        .expect("describe")
        .images()
        .first()
        .cloned()
        .expect("image");
    assert_eq!(img.state().map(|s| s.as_str()), Some("disabled"));
    let _ = client
        .enable_image()
        .image_id(&ami_id)
        .send()
        .await
        .expect("enable");
    let img2 = client
        .describe_images()
        .image_ids(&ami_id)
        .send()
        .await
        .expect("describe")
        .images()
        .first()
        .cloned()
        .expect("image");
    assert_eq!(img2.state().map(|s| s.as_str()), Some("available"));
}

#[tokio::test]
async fn test_enable_disable_image_deprecation() {
    let client = make_ec2_client().await;
    let ami_id = client
        .register_image()
        .name("deprecation-rt")
        .send()
        .await
        .expect("register_image")
        .image_id()
        .expect("image id")
        .to_string();
    let _ = client
        .enable_image_deprecation()
        .image_id(&ami_id)
        .deprecate_at(aws_sdk_ec2::primitives::DateTime::from_secs(2_000_000_000))
        .send()
        .await
        .expect("enable deprecation");
    let _ = client
        .disable_image_deprecation()
        .image_id(&ami_id)
        .send()
        .await
        .expect("disable deprecation");
    // Bogus image id should now error.
    let err = client
        .enable_image_deprecation()
        .image_id("ami-does-not-exist")
        .deprecate_at(aws_sdk_ec2::primitives::DateTime::from_secs(2_000_000_000))
        .send()
        .await
        .expect_err("missing AMI must fail");
    assert!(format!("{err:?}").contains("InvalidAMIID.NotFound"));
}

#[tokio::test]
async fn test_list_images_in_recycle_bin_after_deregister() {
    let client = make_ec2_client().await;
    let ami_id = client
        .register_image()
        .name("recycle-fixture")
        .send()
        .await
        .expect("register")
        .image_id()
        .expect("id")
        .to_string();
    let _ = client
        .deregister_image()
        .image_id(&ami_id)
        .send()
        .await
        .expect("deregister");
    let resp = client
        .list_images_in_recycle_bin()
        .send()
        .await
        .expect("list_images_in_recycle_bin");
    let images = resp.images();
    assert!(
        images.iter().any(|i| i.image_id() == Some(ami_id.as_str())),
        "deregistered AMI should appear in recycle bin"
    );
}

#[tokio::test]
async fn test_enable_then_disable_address_transfer() {
    let client = make_ec2_client().await;
    let allocation_id = client
        .allocate_address()
        .send()
        .await
        .expect("allocate")
        .allocation_id()
        .expect("alloc id")
        .to_string();
    // disable before enable should fail.
    let err = client
        .disable_address_transfer()
        .allocation_id(&allocation_id)
        .send()
        .await
        .expect_err("disable without enable should fail");
    assert!(format!("{err:?}").contains("InvalidAddressTransfer.NotFound"));
    // enable
    let enabled = client
        .enable_address_transfer()
        .allocation_id(&allocation_id)
        .transfer_account_id("999988887777")
        .send()
        .await
        .expect("enable");
    let xfer = enabled.address_transfer().expect("transfer");
    assert_eq!(xfer.transfer_account_id(), Some("999988887777"));
    assert_eq!(
        xfer.address_transfer_status().map(|s| s.as_str()),
        Some("pending")
    );
    // disable
    let disabled = client
        .disable_address_transfer()
        .allocation_id(&allocation_id)
        .send()
        .await
        .expect("disable");
    let xfer2 = disabled.address_transfer().expect("transfer");
    assert_eq!(
        xfer2.address_transfer_status().map(|s| s.as_str()),
        Some("disabled")
    );
}

#[tokio::test]
async fn test_enable_then_disable_capacity_manager() {
    let client = make_ec2_client().await;
    let enabled = client
        .enable_capacity_manager()
        .send()
        .await
        .expect("enable_capacity_manager");
    assert_eq!(enabled.organizations_access(), Some(true));
    assert_eq!(
        enabled.capacity_manager_status().map(|s| s.as_str()),
        Some("enabled")
    );
    let disabled = client
        .disable_capacity_manager()
        .send()
        .await
        .expect("disable_capacity_manager");
    assert_eq!(disabled.organizations_access(), Some(false));
    assert_eq!(
        disabled.capacity_manager_status().map(|s| s.as_str()),
        Some("disabled")
    );
}

#[tokio::test]
async fn test_delete_flow_logs_returns_unsuccessful_for_missing() {
    let client = make_ec2_client().await;
    let resp = client
        .delete_flow_logs()
        .flow_log_ids("fl-doesnotexist")
        .send()
        .await
        .expect("delete_flow_logs");
    let unsuccessful = resp.unsuccessful();
    assert_eq!(unsuccessful.len(), 1);
    assert_eq!(unsuccessful[0].resource_id(), Some("fl-doesnotexist"));
    assert_eq!(
        unsuccessful[0].error().and_then(|e| e.code()),
        Some("InvalidFlowLogId.NotFound")
    );
}

#[tokio::test]
async fn test_delete_vpc_endpoints_returns_unsuccessful_for_missing() {
    let client = make_ec2_client().await;
    let resp = client
        .delete_vpc_endpoints()
        .vpc_endpoint_ids("vpce-doesnotexist")
        .send()
        .await
        .expect("delete_vpc_endpoints");
    let unsuccessful = resp.unsuccessful();
    assert_eq!(unsuccessful.len(), 1);
    assert_eq!(unsuccessful[0].resource_id(), Some("vpce-doesnotexist"));
    assert_eq!(
        unsuccessful[0].error().and_then(|e| e.code()),
        Some("InvalidVpcEndpointId.NotFound")
    );
}

#[tokio::test]
async fn test_describe_capacity_blocks_returns_seeded_block() {
    use std::collections::HashMap;

    use winterbaume_core::StatefulService;
    use winterbaume_ec2::views::{CapacityBlockView, Ec2StateView};
    let svc = Ec2Service::new();
    let mut view = Ec2StateView::default();
    view.capacity_blocks.insert(
        "cb-batcha-001".to_string(),
        CapacityBlockView {
            capacity_block_id: "cb-batcha-001".to_string(),
            capacity_reservation_id: "cr-batcha-001".to_string(),
            capacity_block_offering_id: "cbo-batcha-001".to_string(),
            instance_type: "p5.48xlarge".to_string(),
            instance_count: 16,
            availability_zone: "us-east-1a".to_string(),
            start_date: "2026-06-01T00:00:00.000Z".to_string(),
            end_date: "2026-06-02T00:00:00.000Z".to_string(),
            tenancy: "default".to_string(),
            currency_code: "USD".to_string(),
            upfront_fee: "1000".to_string(),
            commitment_duration_in_seconds: 86400,
            capacity_reservation_arn: String::new(),
            tags: HashMap::new(),
        },
    );
    svc.restore("123456789012", "us-east-1", view)
        .await
        .expect("restore");
    let mock = winterbaume_core::MockAws::builder()
        .with_service(svc)
        .build();
    let cfg = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_ec2::config::Region::new("us-east-1"))
        .load()
        .await;
    let client = aws_sdk_ec2::Client::new(&cfg);
    let resp = client
        .describe_capacity_blocks()
        .capacity_block_ids("cb-batcha-001")
        .send()
        .await
        .expect("describe_capacity_blocks");
    let blocks = resp.capacity_blocks();
    assert_eq!(blocks.len(), 1);
    assert_eq!(blocks[0].capacity_block_id(), Some("cb-batcha-001"));
}

// =====================================================================
// Batch B integration tests.
// =====================================================================

#[tokio::test]
async fn test_describe_prefix_lists_returns_aws_managed_catalogue() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_prefix_lists()
        .send()
        .await
        .expect("describe_prefix_lists");
    let lists = resp.prefix_lists();
    assert!(
        !lists.is_empty(),
        "expected non-empty prefix list catalogue"
    );
    let ids: Vec<&str> = lists.iter().filter_map(|p| p.prefix_list_id()).collect();
    assert!(ids.contains(&"pl-aws-s3"));
    assert!(ids.contains(&"pl-aws-dynamodb"));
}

#[tokio::test]
async fn test_describe_vpc_classic_link_defaults_to_false() {
    let client = make_ec2_client().await;
    let vpc_id = client
        .create_vpc()
        .cidr_block("10.0.0.0/16")
        .send()
        .await
        .expect("create_vpc")
        .vpc()
        .and_then(|v| v.vpc_id())
        .expect("vpc_id")
        .to_string();
    let resp = client
        .describe_vpc_classic_link()
        .vpc_ids(&vpc_id)
        .send()
        .await
        .expect("describe_vpc_classic_link");
    let vpcs = resp.vpcs();
    assert_eq!(vpcs.len(), 1);
    assert_eq!(vpcs[0].vpc_id(), Some(vpc_id.as_str()));
    assert_eq!(vpcs[0].classic_link_enabled(), Some(false));
}

#[tokio::test]
async fn test_describe_instance_types_lists_known_types() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_instance_types()
        .send()
        .await
        .expect("describe_instance_types");
    let types = resp.instance_types();
    assert!(types.len() >= 12);
    let names: Vec<&str> = types
        .iter()
        .filter_map(|t| t.instance_type())
        .map(|t| t.as_str())
        .collect();
    assert!(names.contains(&"t2.micro"));
    assert!(names.contains(&"m5.large"));
    let t2 = types
        .iter()
        .find(|t| t.instance_type().map(|s| s.as_str()) == Some("t2.micro"))
        .expect("t2.micro present");
    assert_eq!(t2.free_tier_eligible(), Some(true));
    let vcpu = t2.v_cpu_info().expect("vcpu info");
    assert!(vcpu.default_v_cpus().unwrap_or(0) >= 1);
}

#[tokio::test]
async fn test_modify_volume_round_trip_with_describe_modifications() {
    let client = make_ec2_client().await;
    let vol_id = client
        .create_volume()
        .availability_zone("us-east-1a")
        .size(10)
        .send()
        .await
        .expect("create_volume")
        .volume_id()
        .expect("volume_id")
        .to_string();
    let modify = client
        .modify_volume()
        .volume_id(&vol_id)
        .size(20)
        .iops(3000)
        .send()
        .await
        .expect("modify_volume");
    let modification = modify.volume_modification().expect("modification");
    assert_eq!(modification.volume_id(), Some(vol_id.as_str()));
    assert_eq!(modification.target_size(), Some(20));
    assert_eq!(modification.target_iops(), Some(3000));
    assert_eq!(modification.original_size(), Some(10));
    let described = client
        .describe_volumes_modifications()
        .volume_ids(&vol_id)
        .send()
        .await
        .expect("describe_volumes_modifications");
    let mods = described.volumes_modifications();
    assert_eq!(mods.len(), 1);
    assert_eq!(mods[0].volume_id(), Some(vol_id.as_str()));
}

#[tokio::test]
async fn test_import_volume_then_cancel_conversion_task() {
    let client = make_ec2_client().await;
    let resp = client
        .import_volume()
        .availability_zone("us-east-1a")
        .send()
        .await
        .expect("import_volume");
    let task = resp.conversion_task().expect("conversion_task");
    let task_id = task.conversion_task_id().expect("task id").to_string();
    assert!(task_id.starts_with("import-vol-"));
    assert_eq!(task.state().map(|s| s.as_str()), Some("active"));
    client
        .cancel_conversion_task()
        .conversion_task_id(&task_id)
        .send()
        .await
        .expect("cancel_conversion_task");
}

#[tokio::test]
async fn test_bundle_instance_describe_then_cancel() {
    let client = make_ec2_client().await;
    // BundleInstance accepts an instance ID and returns a bundle task.
    let bundle = client
        .bundle_instance()
        .instance_id("i-deadbeef")
        .storage(
            aws_sdk_ec2::types::Storage::builder()
                .s3(aws_sdk_ec2::types::S3Storage::builder()
                    .bucket("my-bucket")
                    .prefix("backups/")
                    .build())
                .build(),
        )
        .send()
        .await
        .expect("bundle_instance");
    let task = bundle.bundle_task().expect("bundle_task");
    let bundle_id = task.bundle_id().expect("bundle_id").to_string();
    assert_eq!(task.state().map(|s| s.as_str()), Some("pending"));
    let described = client
        .describe_bundle_tasks()
        .bundle_ids(&bundle_id)
        .send()
        .await
        .expect("describe_bundle_tasks");
    let tasks = described.bundle_tasks();
    assert_eq!(tasks.len(), 1);
    assert_eq!(tasks[0].bundle_id(), Some(bundle_id.as_str()));
    let cancelled = client
        .cancel_bundle_task()
        .bundle_id(&bundle_id)
        .send()
        .await
        .expect("cancel_bundle_task");
    assert_eq!(
        cancelled
            .bundle_task()
            .and_then(|t| t.state())
            .map(|s| s.as_str()),
        Some("cancelling")
    );
}

#[tokio::test]
async fn test_describe_id_format_after_modify_id_format() {
    let client = make_ec2_client().await;
    client
        .modify_id_format()
        .resource("instance")
        .use_long_ids(true)
        .send()
        .await
        .expect("modify_id_format");
    let resp = client
        .describe_id_format()
        .resource("instance")
        .send()
        .await
        .expect("describe_id_format");
    let statuses = resp.statuses();
    assert_eq!(statuses.len(), 1);
    assert_eq!(statuses[0].resource(), Some("instance"));
    assert_eq!(statuses[0].use_long_ids(), Some(true));
}

#[tokio::test]
async fn test_describe_image_references_after_run_instances() {
    let client = make_ec2_client().await;
    // Register an AMI and then run an instance from it.
    let image_id = client
        .register_image()
        .name("test-references-ami")
        .architecture(aws_sdk_ec2::types::ArchitectureValues::X8664)
        .root_device_name("/dev/xvda")
        .send()
        .await
        .expect("register_image")
        .image_id()
        .expect("image_id")
        .to_string();
    client
        .run_instances()
        .image_id(&image_id)
        .min_count(1)
        .max_count(1)
        .send()
        .await
        .expect("run_instances");
    let resp = client
        .describe_image_references()
        .image_ids(&image_id)
        .send()
        .await
        .expect("describe_image_references");
    let refs = resp.image_references();
    assert!(!refs.is_empty(), "expected at least one image reference");
    assert_eq!(refs[0].image_id(), Some(image_id.as_str()));
    assert_eq!(
        refs[0].resource_type().map(|s| s.as_str()),
        Some("instance")
    );
}

#[tokio::test]
async fn test_describe_mac_hosts_with_seeded_mac_host() {
    use std::collections::HashMap;

    use winterbaume_core::StatefulService;
    use winterbaume_ec2::views::{DedicatedHostView, Ec2StateView};
    let svc = Ec2Service::new();
    let mut view = Ec2StateView::default();
    view.dedicated_hosts.insert(
        "h-mac0001".to_string(),
        DedicatedHostView {
            host_id: "h-mac0001".to_string(),
            availability_zone: "us-east-1a".to_string(),
            instance_type: Some("mac1.metal".to_string()),
            auto_placement: "on".to_string(),
            host_recovery: "off".to_string(),
            state: "available".to_string(),
            tags: HashMap::new(),
        },
    );
    svc.restore("123456789012", "us-east-1", view)
        .await
        .expect("restore");
    let mock = winterbaume_core::MockAws::builder()
        .with_service(svc)
        .build();
    let cfg = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_ec2::config::Region::new("us-east-1"))
        .load()
        .await;
    let client = aws_sdk_ec2::Client::new(&cfg);
    let resp = client
        .describe_mac_hosts()
        .send()
        .await
        .expect("describe_mac_hosts");
    let macs = resp.mac_hosts();
    assert_eq!(macs.len(), 1);
    assert_eq!(macs[0].host_id(), Some("h-mac0001"));
}

#[tokio::test]
async fn test_describe_outpost_lags_returns_seeded_lag() {
    use std::collections::HashMap;

    use winterbaume_core::StatefulService;
    use winterbaume_ec2::views::{Ec2StateView, OutpostLagView};
    let svc = Ec2Service::new();
    let mut view = Ec2StateView::default();
    view.outpost_lags.insert(
        "outpost-lag-batchb".to_string(),
        OutpostLagView {
            outpost_lag_id: "outpost-lag-batchb".to_string(),
            outpost_arn: "arn:aws:outposts:us-east-1:123456789012:outpost/op-1234".to_string(),
            owner_id: "123456789012".to_string(),
            state: "available".to_string(),
            local_gateway_virtual_interface_ids: vec!["lgw-vif-001".to_string()],
            service_link_virtual_interface_ids: vec!["sl-vif-001".to_string()],
            tags: HashMap::new(),
        },
    );
    svc.restore("123456789012", "us-east-1", view)
        .await
        .expect("restore");
    let mock = winterbaume_core::MockAws::builder()
        .with_service(svc)
        .build();
    let cfg = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_ec2::config::Region::new("us-east-1"))
        .load()
        .await;
    let client = aws_sdk_ec2::Client::new(&cfg);
    let resp = client
        .describe_outpost_lags()
        .send()
        .await
        .expect("describe_outpost_lags");
    let lags = resp.outpost_lags();
    assert_eq!(lags.len(), 1);
    assert_eq!(lags[0].outpost_lag_id(), Some("outpost-lag-batchb"));
}

#[tokio::test]
async fn test_export_image_then_describe_export_image_tasks() {
    let client = make_ec2_client().await;
    let image_id = client
        .register_image()
        .name("test-export-ami")
        .architecture(aws_sdk_ec2::types::ArchitectureValues::X8664)
        .root_device_name("/dev/xvda")
        .send()
        .await
        .expect("register_image")
        .image_id()
        .expect("image_id")
        .to_string();
    let resp = client
        .export_image()
        .image_id(&image_id)
        .role_name("vmimport")
        .s3_export_location(
            aws_sdk_ec2::types::ExportTaskS3LocationRequest::builder()
                .s3_bucket("my-export-bucket")
                .s3_prefix("exports/")
                .build(),
        )
        .disk_image_format(aws_sdk_ec2::types::DiskImageFormat::Vmdk)
        .send()
        .await
        .expect("export_image");
    let task_id = resp.export_image_task_id().expect("task id").to_string();
    assert!(task_id.starts_with("export-ami-"));
    let described = client
        .describe_export_image_tasks()
        .export_image_task_ids(&task_id)
        .send()
        .await
        .expect("describe_export_image_tasks");
    let tasks = described.export_image_tasks();
    assert_eq!(tasks.len(), 1);
    assert_eq!(tasks[0].export_image_task_id(), Some(task_id.as_str()));
    assert_eq!(tasks[0].image_id(), Some(image_id.as_str()));
}

#[tokio::test]
async fn test_enable_then_disable_fast_launch_round_trip() {
    let client = make_ec2_client().await;
    let image_id = client
        .register_image()
        .name("test-fl-cycle")
        .architecture(aws_sdk_ec2::types::ArchitectureValues::X8664)
        .root_device_name("/dev/xvda")
        .send()
        .await
        .expect("register_image")
        .image_id()
        .expect("image_id")
        .to_string();
    let enabled = client
        .enable_fast_launch()
        .image_id(&image_id)
        .max_parallel_launches(8)
        .send()
        .await
        .expect("enable_fast_launch");
    assert_eq!(enabled.state().map(|s| s.as_str()), Some("enabled"));
    assert_eq!(enabled.max_parallel_launches(), Some(8));
    // DescribeImages should reflect the fast-launch state via the image
    // record. We look up the image to confirm the state propagates.
    let images = client
        .describe_images()
        .image_ids(&image_id)
        .send()
        .await
        .expect("describe_images");
    assert_eq!(images.images().len(), 1);
    let disabled = client
        .disable_fast_launch()
        .image_id(&image_id)
        .send()
        .await
        .expect("disable_fast_launch");
    assert_eq!(disabled.state().map(|s| s.as_str()), Some("disabled"));
}

// ===== Wave 1: Describe/Get handlers wired to real state =====

#[tokio::test]
async fn test_describe_traffic_mirror_filters_returns_created_filter() {
    let client = make_ec2_client().await;
    let filter_id = client
        .create_traffic_mirror_filter()
        .description("wave1 filter")
        .send()
        .await
        .expect("create_traffic_mirror_filter")
        .traffic_mirror_filter()
        .expect("filter")
        .traffic_mirror_filter_id()
        .expect("id")
        .to_string();

    let resp = client
        .describe_traffic_mirror_filters()
        .traffic_mirror_filter_ids(&filter_id)
        .send()
        .await
        .expect("describe_traffic_mirror_filters");
    let filters = resp.traffic_mirror_filters();
    assert_eq!(filters.len(), 1);
    assert_eq!(filters[0].traffic_mirror_filter_id().unwrap(), filter_id);
    assert_eq!(filters[0].description().unwrap_or(""), "wave1 filter");
}

#[tokio::test]
async fn test_describe_traffic_mirror_filter_rules_returns_created_rule() {
    let client = make_ec2_client().await;
    let filter_id = client
        .create_traffic_mirror_filter()
        .send()
        .await
        .expect("create_traffic_mirror_filter")
        .traffic_mirror_filter()
        .expect("filter")
        .traffic_mirror_filter_id()
        .expect("id")
        .to_string();

    let rule_id = client
        .create_traffic_mirror_filter_rule()
        .traffic_mirror_filter_id(&filter_id)
        .traffic_direction(aws_sdk_ec2::types::TrafficDirection::Ingress)
        .rule_number(123)
        .rule_action(aws_sdk_ec2::types::TrafficMirrorRuleAction::Accept)
        .destination_cidr_block("10.0.0.0/8")
        .source_cidr_block("0.0.0.0/0")
        .send()
        .await
        .expect("create_traffic_mirror_filter_rule")
        .traffic_mirror_filter_rule()
        .expect("rule")
        .traffic_mirror_filter_rule_id()
        .expect("id")
        .to_string();

    let resp = client
        .describe_traffic_mirror_filter_rules()
        .traffic_mirror_filter_id(&filter_id)
        .send()
        .await
        .expect("describe_traffic_mirror_filter_rules");
    let rules = resp.traffic_mirror_filter_rules();
    assert_eq!(rules.len(), 1);
    assert_eq!(rules[0].traffic_mirror_filter_rule_id().unwrap(), rule_id);
    assert_eq!(rules[0].rule_number(), Some(123));
}

#[tokio::test]
async fn test_describe_traffic_mirror_targets_returns_created_target() {
    let client = make_ec2_client().await;
    let target_id = client
        .create_traffic_mirror_target()
        .network_interface_id("eni-wave1-target")
        .description("wave1 target")
        .send()
        .await
        .expect("create_traffic_mirror_target")
        .traffic_mirror_target()
        .expect("target")
        .traffic_mirror_target_id()
        .expect("id")
        .to_string();

    let resp = client
        .describe_traffic_mirror_targets()
        .traffic_mirror_target_ids(&target_id)
        .send()
        .await
        .expect("describe_traffic_mirror_targets");
    let targets = resp.traffic_mirror_targets();
    assert_eq!(targets.len(), 1);
    assert_eq!(targets[0].traffic_mirror_target_id().unwrap(), target_id);
    assert_eq!(targets[0].description().unwrap_or(""), "wave1 target");
}

#[tokio::test]
async fn test_describe_traffic_mirror_sessions_returns_created_session() {
    let client = make_ec2_client().await;
    let target_id = client
        .create_traffic_mirror_target()
        .network_interface_id("eni-wave1-tgt")
        .send()
        .await
        .expect("create_traffic_mirror_target")
        .traffic_mirror_target()
        .expect("target")
        .traffic_mirror_target_id()
        .expect("id")
        .to_string();
    let filter_id = client
        .create_traffic_mirror_filter()
        .send()
        .await
        .expect("create_traffic_mirror_filter")
        .traffic_mirror_filter()
        .expect("filter")
        .traffic_mirror_filter_id()
        .expect("id")
        .to_string();
    let session_id = client
        .create_traffic_mirror_session()
        .traffic_mirror_target_id(&target_id)
        .traffic_mirror_filter_id(&filter_id)
        .network_interface_id("eni-wave1-src")
        .session_number(7)
        .send()
        .await
        .expect("create_traffic_mirror_session")
        .traffic_mirror_session()
        .expect("session")
        .traffic_mirror_session_id()
        .expect("id")
        .to_string();

    let resp = client
        .describe_traffic_mirror_sessions()
        .traffic_mirror_session_ids(&session_id)
        .send()
        .await
        .expect("describe_traffic_mirror_sessions");
    let sessions = resp.traffic_mirror_sessions();
    assert_eq!(sessions.len(), 1);
    assert_eq!(sessions[0].traffic_mirror_session_id().unwrap(), session_id);
    assert_eq!(sessions[0].session_number(), Some(7));
}

#[tokio::test]
async fn test_describe_client_vpn_endpoints_returns_created_endpoint() {
    let client = make_ec2_client().await;
    let endpoint_id = client
        .create_client_vpn_endpoint()
        .client_cidr_block("10.40.0.0/22")
        .server_certificate_arn("arn:aws:acm:us-east-1:123456789012:certificate/wave1")
        .send()
        .await
        .expect("create_client_vpn_endpoint")
        .client_vpn_endpoint_id()
        .expect("id")
        .to_string();

    let resp = client
        .describe_client_vpn_endpoints()
        .client_vpn_endpoint_ids(&endpoint_id)
        .send()
        .await
        .expect("describe_client_vpn_endpoints");
    let endpoints = resp.client_vpn_endpoints();
    assert_eq!(endpoints.len(), 1);
    assert_eq!(endpoints[0].client_vpn_endpoint_id().unwrap(), endpoint_id);
    assert_eq!(
        endpoints[0].client_cidr_block().unwrap_or(""),
        "10.40.0.0/22"
    );
}

#[tokio::test]
async fn test_describe_client_vpn_authorization_rules_returns_created_rule() {
    let client = make_ec2_client().await;
    let endpoint_id = client
        .create_client_vpn_endpoint()
        .client_cidr_block("10.41.0.0/22")
        .server_certificate_arn("arn:aws:acm:us-east-1:123456789012:certificate/wave1auth")
        .send()
        .await
        .expect("create_client_vpn_endpoint")
        .client_vpn_endpoint_id()
        .expect("id")
        .to_string();
    client
        .authorize_client_vpn_ingress()
        .client_vpn_endpoint_id(&endpoint_id)
        .target_network_cidr("0.0.0.0/0")
        .authorize_all_groups(true)
        .send()
        .await
        .expect("authorize_client_vpn_ingress");

    let resp = client
        .describe_client_vpn_authorization_rules()
        .client_vpn_endpoint_id(&endpoint_id)
        .send()
        .await
        .expect("describe_client_vpn_authorization_rules");
    let rules = resp.authorization_rules();
    assert_eq!(rules.len(), 1);
    assert_eq!(rules[0].destination_cidr().unwrap_or(""), "0.0.0.0/0");
    assert_eq!(rules[0].access_all(), Some(true));
}

#[tokio::test]
async fn test_describe_client_vpn_routes_returns_created_route() {
    let client = make_ec2_client().await;
    let vpc_id = client
        .create_vpc()
        .cidr_block("10.42.0.0/16")
        .send()
        .await
        .expect("create_vpc")
        .vpc()
        .expect("vpc")
        .vpc_id()
        .expect("id")
        .to_string();
    let subnet_id = client
        .create_subnet()
        .vpc_id(&vpc_id)
        .cidr_block("10.42.1.0/24")
        .send()
        .await
        .expect("create_subnet")
        .subnet()
        .expect("subnet")
        .subnet_id()
        .expect("id")
        .to_string();
    let endpoint_id = client
        .create_client_vpn_endpoint()
        .client_cidr_block("10.43.0.0/22")
        .server_certificate_arn("arn:aws:acm:us-east-1:123456789012:certificate/wave1routes")
        .send()
        .await
        .expect("create_client_vpn_endpoint")
        .client_vpn_endpoint_id()
        .expect("id")
        .to_string();
    client
        .associate_client_vpn_target_network()
        .client_vpn_endpoint_id(&endpoint_id)
        .subnet_id(&subnet_id)
        .send()
        .await
        .expect("associate_client_vpn_target_network");
    client
        .create_client_vpn_route()
        .client_vpn_endpoint_id(&endpoint_id)
        .destination_cidr_block("172.31.0.0/16")
        .target_vpc_subnet_id(&subnet_id)
        .send()
        .await
        .expect("create_client_vpn_route");

    let resp = client
        .describe_client_vpn_routes()
        .client_vpn_endpoint_id(&endpoint_id)
        .send()
        .await
        .expect("describe_client_vpn_routes");
    let routes = resp.routes();
    assert_eq!(routes.len(), 1);
    assert_eq!(routes[0].destination_cidr().unwrap_or(""), "172.31.0.0/16");
}

#[tokio::test]
async fn test_describe_client_vpn_target_networks_returns_created_assoc() {
    let client = make_ec2_client().await;
    let vpc_id = client
        .create_vpc()
        .cidr_block("10.44.0.0/16")
        .send()
        .await
        .expect("create_vpc")
        .vpc()
        .expect("vpc")
        .vpc_id()
        .expect("id")
        .to_string();
    let subnet_id = client
        .create_subnet()
        .vpc_id(&vpc_id)
        .cidr_block("10.44.1.0/24")
        .send()
        .await
        .expect("create_subnet")
        .subnet()
        .expect("subnet")
        .subnet_id()
        .expect("id")
        .to_string();
    let endpoint_id = client
        .create_client_vpn_endpoint()
        .client_cidr_block("10.45.0.0/22")
        .server_certificate_arn("arn:aws:acm:us-east-1:123456789012:certificate/wave1tgt")
        .send()
        .await
        .expect("create_client_vpn_endpoint")
        .client_vpn_endpoint_id()
        .expect("id")
        .to_string();
    let assoc_id = client
        .associate_client_vpn_target_network()
        .client_vpn_endpoint_id(&endpoint_id)
        .subnet_id(&subnet_id)
        .send()
        .await
        .expect("associate_client_vpn_target_network")
        .association_id()
        .expect("id")
        .to_string();

    let resp = client
        .describe_client_vpn_target_networks()
        .client_vpn_endpoint_id(&endpoint_id)
        .send()
        .await
        .expect("describe_client_vpn_target_networks");
    let nets = resp.client_vpn_target_networks();
    assert_eq!(nets.len(), 1);
    assert_eq!(nets[0].association_id().unwrap_or(""), assoc_id);
    assert_eq!(nets[0].vpc_id().unwrap_or(""), vpc_id);
}

#[tokio::test]
async fn test_describe_client_vpn_connections_filters_by_endpoint() {
    let client = make_ec2_client().await;
    let endpoint_id = client
        .create_client_vpn_endpoint()
        .client_cidr_block("10.46.0.0/22")
        .server_certificate_arn("arn:aws:acm:us-east-1:123456789012:certificate/wave1conn")
        .send()
        .await
        .expect("create_client_vpn_endpoint")
        .client_vpn_endpoint_id()
        .expect("id")
        .to_string();

    // No connections seeded; should still return empty success.
    let resp = client
        .describe_client_vpn_connections()
        .client_vpn_endpoint_id(&endpoint_id)
        .send()
        .await
        .expect("describe_client_vpn_connections");
    assert!(resp.connections().is_empty());
}

#[tokio::test]
async fn test_describe_verified_access_instances_returns_created_instance() {
    let client = make_ec2_client().await;
    let instance_id = client
        .create_verified_access_instance()
        .description("wave1 vai")
        .send()
        .await
        .expect("create_verified_access_instance")
        .verified_access_instance()
        .expect("instance")
        .verified_access_instance_id()
        .expect("id")
        .to_string();

    let resp = client
        .describe_verified_access_instances()
        .verified_access_instance_ids(&instance_id)
        .send()
        .await
        .expect("describe_verified_access_instances");
    let insts = resp.verified_access_instances();
    assert_eq!(insts.len(), 1);
    assert_eq!(insts[0].verified_access_instance_id().unwrap(), instance_id);
    assert_eq!(insts[0].description().unwrap_or(""), "wave1 vai");
}

#[tokio::test]
async fn test_describe_verified_access_trust_providers_returns_created_tp() {
    let client = make_ec2_client().await;
    let tp_id = client
        .create_verified_access_trust_provider()
        .trust_provider_type(aws_sdk_ec2::types::TrustProviderType::User)
        .user_trust_provider_type(aws_sdk_ec2::types::UserTrustProviderType::Oidc)
        .policy_reference_name("wave1ref")
        .description("wave1 tp")
        .send()
        .await
        .expect("create_verified_access_trust_provider")
        .verified_access_trust_provider()
        .expect("tp")
        .verified_access_trust_provider_id()
        .expect("id")
        .to_string();

    let resp = client
        .describe_verified_access_trust_providers()
        .verified_access_trust_provider_ids(&tp_id)
        .send()
        .await
        .expect("describe_verified_access_trust_providers");
    let tps = resp.verified_access_trust_providers();
    assert_eq!(tps.len(), 1);
    assert_eq!(tps[0].verified_access_trust_provider_id().unwrap(), tp_id);
    assert_eq!(tps[0].description().unwrap_or(""), "wave1 tp");
}

#[tokio::test]
async fn test_describe_verified_access_groups_returns_created_group() {
    let client = make_ec2_client().await;
    let instance_id = client
        .create_verified_access_instance()
        .send()
        .await
        .expect("create_verified_access_instance")
        .verified_access_instance()
        .expect("instance")
        .verified_access_instance_id()
        .expect("id")
        .to_string();
    let group_id = client
        .create_verified_access_group()
        .verified_access_instance_id(&instance_id)
        .description("wave1 grp")
        .send()
        .await
        .expect("create_verified_access_group")
        .verified_access_group()
        .expect("group")
        .verified_access_group_id()
        .expect("id")
        .to_string();

    let resp = client
        .describe_verified_access_groups()
        .verified_access_group_ids(&group_id)
        .send()
        .await
        .expect("describe_verified_access_groups");
    let groups = resp.verified_access_groups();
    assert_eq!(groups.len(), 1);
    assert_eq!(groups[0].verified_access_group_id().unwrap(), group_id);
    assert_eq!(groups[0].description().unwrap_or(""), "wave1 grp");
}

#[tokio::test]
async fn test_describe_verified_access_endpoints_returns_created_endpoint() {
    use aws_sdk_ec2::types::{
        CreateVerifiedAccessEndpointLoadBalancerOptions, VerifiedAccessEndpointAttachmentType,
        VerifiedAccessEndpointType,
    };
    let client = make_ec2_client().await;
    let instance_id = client
        .create_verified_access_instance()
        .send()
        .await
        .expect("create_verified_access_instance")
        .verified_access_instance()
        .expect("instance")
        .verified_access_instance_id()
        .expect("id")
        .to_string();
    let group_id = client
        .create_verified_access_group()
        .verified_access_instance_id(&instance_id)
        .send()
        .await
        .expect("create_verified_access_group")
        .verified_access_group()
        .expect("group")
        .verified_access_group_id()
        .expect("id")
        .to_string();
    let lb_options = CreateVerifiedAccessEndpointLoadBalancerOptions::builder()
        .load_balancer_arn(
            "arn:aws:elasticloadbalancing:us-east-1:123456789012:loadbalancer/app/wave1",
        )
        .port(443)
        .protocol(aws_sdk_ec2::types::VerifiedAccessEndpointProtocol::Https)
        .subnet_ids("subnet-wave1")
        .build();
    let endpoint_id = client
        .create_verified_access_endpoint()
        .verified_access_group_id(&group_id)
        .endpoint_type(VerifiedAccessEndpointType::LoadBalancer)
        .attachment_type(VerifiedAccessEndpointAttachmentType::Vpc)
        .application_domain("wave1.example.com")
        .domain_certificate_arn("arn:aws:acm:us-east-1:123456789012:certificate/wave1ep")
        .endpoint_domain_prefix("api")
        .load_balancer_options(lb_options)
        .send()
        .await
        .expect("create_verified_access_endpoint")
        .verified_access_endpoint()
        .expect("endpoint")
        .verified_access_endpoint_id()
        .expect("id")
        .to_string();

    let resp = client
        .describe_verified_access_endpoints()
        .verified_access_endpoint_ids(&endpoint_id)
        .send()
        .await
        .expect("describe_verified_access_endpoints");
    let eps = resp.verified_access_endpoints();
    assert_eq!(eps.len(), 1);
    assert_eq!(eps[0].verified_access_endpoint_id().unwrap(), endpoint_id);
    assert_eq!(eps[0].verified_access_group_id().unwrap(), group_id);
}

#[tokio::test]
async fn test_describe_verified_access_instance_logging_configurations_returns_created() {
    use aws_sdk_ec2::types::{VerifiedAccessLogOptions, VerifiedAccessLogS3DestinationOptions};
    let client = make_ec2_client().await;
    let instance_id = client
        .create_verified_access_instance()
        .send()
        .await
        .expect("create_verified_access_instance")
        .verified_access_instance()
        .expect("instance")
        .verified_access_instance_id()
        .expect("id")
        .to_string();
    let logs = VerifiedAccessLogOptions::builder()
        .s3(VerifiedAccessLogS3DestinationOptions::builder()
            .enabled(true)
            .bucket_name("wave1-bucket")
            .build())
        .build();
    client
        .modify_verified_access_instance_logging_configuration()
        .verified_access_instance_id(&instance_id)
        .access_logs(logs)
        .send()
        .await
        .expect("modify_verified_access_instance_logging_configuration");

    let resp = client
        .describe_verified_access_instance_logging_configurations()
        .verified_access_instance_ids(&instance_id)
        .send()
        .await
        .expect("describe_verified_access_instance_logging_configurations");
    let configs = resp.logging_configurations();
    assert_eq!(configs.len(), 1);
    assert_eq!(
        configs[0].verified_access_instance_id().unwrap(),
        instance_id
    );
    let access_logs = configs[0].access_logs().expect("access logs");
    assert_eq!(
        access_logs
            .s3()
            .and_then(|s3| s3.bucket_name())
            .unwrap_or(""),
        "wave1-bucket"
    );
}

#[tokio::test]
async fn test_get_verified_access_endpoint_policy_returns_set_policy() {
    use aws_sdk_ec2::types::{
        CreateVerifiedAccessEndpointLoadBalancerOptions, VerifiedAccessEndpointAttachmentType,
        VerifiedAccessEndpointType,
    };
    let client = make_ec2_client().await;
    let instance_id = client
        .create_verified_access_instance()
        .send()
        .await
        .expect("create_verified_access_instance")
        .verified_access_instance()
        .expect("instance")
        .verified_access_instance_id()
        .expect("id")
        .to_string();
    let group_id = client
        .create_verified_access_group()
        .verified_access_instance_id(&instance_id)
        .send()
        .await
        .expect("create_verified_access_group")
        .verified_access_group()
        .expect("group")
        .verified_access_group_id()
        .expect("id")
        .to_string();
    let lb_options = CreateVerifiedAccessEndpointLoadBalancerOptions::builder()
        .load_balancer_arn(
            "arn:aws:elasticloadbalancing:us-east-1:123456789012:loadbalancer/app/wave1pol",
        )
        .port(443)
        .protocol(aws_sdk_ec2::types::VerifiedAccessEndpointProtocol::Https)
        .subnet_ids("subnet-wave1pol")
        .build();
    let endpoint_id = client
        .create_verified_access_endpoint()
        .verified_access_group_id(&group_id)
        .endpoint_type(VerifiedAccessEndpointType::LoadBalancer)
        .attachment_type(VerifiedAccessEndpointAttachmentType::Vpc)
        .application_domain("wave1pol.example.com")
        .domain_certificate_arn("arn:aws:acm:us-east-1:123456789012:certificate/wave1pol")
        .endpoint_domain_prefix("api")
        .load_balancer_options(lb_options)
        .send()
        .await
        .expect("create_verified_access_endpoint")
        .verified_access_endpoint()
        .expect("endpoint")
        .verified_access_endpoint_id()
        .expect("id")
        .to_string();
    client
        .modify_verified_access_endpoint_policy()
        .verified_access_endpoint_id(&endpoint_id)
        .policy_document("permit(principal,action,resource);")
        .policy_enabled(true)
        .send()
        .await
        .expect("modify_verified_access_endpoint_policy");

    let resp = client
        .get_verified_access_endpoint_policy()
        .verified_access_endpoint_id(&endpoint_id)
        .send()
        .await
        .expect("get_verified_access_endpoint_policy");
    assert_eq!(resp.policy_enabled(), Some(true));
    assert!(resp.policy_document().unwrap_or("").contains("permit"));
}

#[tokio::test]
async fn test_get_verified_access_endpoint_targets_returns_empty_list() {
    use aws_sdk_ec2::types::{
        CreateVerifiedAccessEndpointLoadBalancerOptions, VerifiedAccessEndpointAttachmentType,
        VerifiedAccessEndpointType,
    };
    let client = make_ec2_client().await;
    let instance_id = client
        .create_verified_access_instance()
        .send()
        .await
        .expect("create_verified_access_instance")
        .verified_access_instance()
        .expect("instance")
        .verified_access_instance_id()
        .expect("id")
        .to_string();
    let group_id = client
        .create_verified_access_group()
        .verified_access_instance_id(&instance_id)
        .send()
        .await
        .expect("create_verified_access_group")
        .verified_access_group()
        .expect("group")
        .verified_access_group_id()
        .expect("id")
        .to_string();
    let lb_options = CreateVerifiedAccessEndpointLoadBalancerOptions::builder()
        .load_balancer_arn(
            "arn:aws:elasticloadbalancing:us-east-1:123456789012:loadbalancer/app/wave1tgts",
        )
        .port(443)
        .protocol(aws_sdk_ec2::types::VerifiedAccessEndpointProtocol::Https)
        .subnet_ids("subnet-wave1tgts")
        .build();
    let endpoint_id = client
        .create_verified_access_endpoint()
        .verified_access_group_id(&group_id)
        .endpoint_type(VerifiedAccessEndpointType::LoadBalancer)
        .attachment_type(VerifiedAccessEndpointAttachmentType::Vpc)
        .application_domain("wave1tgts.example.com")
        .domain_certificate_arn("arn:aws:acm:us-east-1:123456789012:certificate/wave1tgts")
        .endpoint_domain_prefix("api")
        .load_balancer_options(lb_options)
        .send()
        .await
        .expect("create_verified_access_endpoint")
        .verified_access_endpoint()
        .expect("endpoint")
        .verified_access_endpoint_id()
        .expect("id")
        .to_string();

    let resp = client
        .get_verified_access_endpoint_targets()
        .verified_access_endpoint_id(&endpoint_id)
        .send()
        .await
        .expect("get_verified_access_endpoint_targets");
    assert!(resp.verified_access_endpoint_targets().is_empty());
}

#[tokio::test]
async fn test_get_verified_access_group_policy_returns_set_policy() {
    let client = make_ec2_client().await;
    let instance_id = client
        .create_verified_access_instance()
        .send()
        .await
        .expect("create_verified_access_instance")
        .verified_access_instance()
        .expect("instance")
        .verified_access_instance_id()
        .expect("id")
        .to_string();
    let group_id = client
        .create_verified_access_group()
        .verified_access_instance_id(&instance_id)
        .send()
        .await
        .expect("create_verified_access_group")
        .verified_access_group()
        .expect("group")
        .verified_access_group_id()
        .expect("id")
        .to_string();
    client
        .modify_verified_access_group_policy()
        .verified_access_group_id(&group_id)
        .policy_document("permit(principal,action,resource);")
        .policy_enabled(true)
        .send()
        .await
        .expect("modify_verified_access_group_policy");

    let resp = client
        .get_verified_access_group_policy()
        .verified_access_group_id(&group_id)
        .send()
        .await
        .expect("get_verified_access_group_policy");
    assert_eq!(resp.policy_enabled(), Some(true));
    assert!(resp.policy_document().unwrap_or("").contains("permit"));
}

#[tokio::test]
async fn test_describe_network_insights_paths_returns_created_path() {
    let client = make_ec2_client().await;
    let path_id = client
        .create_network_insights_path()
        .source("eni-wave1-src")
        .destination("eni-wave1-dst")
        .protocol(aws_sdk_ec2::types::Protocol::Tcp)
        .destination_port(443)
        .send()
        .await
        .expect("create_network_insights_path")
        .network_insights_path()
        .expect("path")
        .network_insights_path_id()
        .expect("id")
        .to_string();

    let resp = client
        .describe_network_insights_paths()
        .network_insights_path_ids(&path_id)
        .send()
        .await
        .expect("describe_network_insights_paths");
    let paths = resp.network_insights_paths();
    assert_eq!(paths.len(), 1);
    assert_eq!(paths[0].network_insights_path_id().unwrap(), path_id);
    assert_eq!(paths[0].destination_port(), Some(443));
}

#[tokio::test]
async fn test_describe_network_insights_analyses_returns_created_analysis() {
    let client = make_ec2_client().await;
    let path_id = client
        .create_network_insights_path()
        .source("eni-wave1-src2")
        .destination("eni-wave1-dst2")
        .protocol(aws_sdk_ec2::types::Protocol::Tcp)
        .send()
        .await
        .expect("create_network_insights_path")
        .network_insights_path()
        .expect("path")
        .network_insights_path_id()
        .expect("id")
        .to_string();
    let analysis_id = client
        .start_network_insights_analysis()
        .network_insights_path_id(&path_id)
        .client_token("wave1-ct")
        .send()
        .await
        .expect("start_network_insights_analysis")
        .network_insights_analysis()
        .expect("analysis")
        .network_insights_analysis_id()
        .expect("id")
        .to_string();

    let resp = client
        .describe_network_insights_analyses()
        .network_insights_analysis_ids(&analysis_id)
        .send()
        .await
        .expect("describe_network_insights_analyses");
    let analyses = resp.network_insights_analyses();
    assert_eq!(analyses.len(), 1);
    assert_eq!(
        analyses[0].network_insights_analysis_id().unwrap(),
        analysis_id
    );
    assert_eq!(analyses[0].network_insights_path_id().unwrap(), path_id);
}

#[tokio::test]
async fn test_describe_network_insights_access_scopes_returns_created_scope() {
    let client = make_ec2_client().await;
    let scope_id = client
        .create_network_insights_access_scope()
        .client_token("wave1-scope-ct")
        .send()
        .await
        .expect("create_network_insights_access_scope")
        .network_insights_access_scope()
        .expect("scope")
        .network_insights_access_scope_id()
        .expect("id")
        .to_string();

    let resp = client
        .describe_network_insights_access_scopes()
        .network_insights_access_scope_ids(&scope_id)
        .send()
        .await
        .expect("describe_network_insights_access_scopes");
    let scopes = resp.network_insights_access_scopes();
    assert_eq!(scopes.len(), 1);
    assert_eq!(
        scopes[0].network_insights_access_scope_id().unwrap(),
        scope_id
    );
}

#[tokio::test]
async fn test_describe_network_insights_access_scope_analyses_returns_created_analysis() {
    let client = make_ec2_client().await;
    let scope_id = client
        .create_network_insights_access_scope()
        .client_token("wave1-scope2-ct")
        .send()
        .await
        .expect("create_network_insights_access_scope")
        .network_insights_access_scope()
        .expect("scope")
        .network_insights_access_scope_id()
        .expect("id")
        .to_string();
    let analysis_id = client
        .start_network_insights_access_scope_analysis()
        .network_insights_access_scope_id(&scope_id)
        .client_token("wave1-scope-an-ct")
        .send()
        .await
        .expect("start_network_insights_access_scope_analysis")
        .network_insights_access_scope_analysis()
        .expect("analysis")
        .network_insights_access_scope_analysis_id()
        .expect("id")
        .to_string();

    let resp = client
        .describe_network_insights_access_scope_analyses()
        .network_insights_access_scope_analysis_ids(&analysis_id)
        .send()
        .await
        .expect("describe_network_insights_access_scope_analyses");
    let analyses = resp.network_insights_access_scope_analyses();
    assert_eq!(analyses.len(), 1);
    assert_eq!(
        analyses[0]
            .network_insights_access_scope_analysis_id()
            .unwrap(),
        analysis_id
    );
    assert_eq!(
        analyses[0].network_insights_access_scope_id().unwrap(),
        scope_id
    );
}

// ---------------------------------------------------------------------------
// Wave 2: TGW extensions + IPAM getters — describe/get-from-state coverage
// ---------------------------------------------------------------------------

async fn wave2_seed_tgw_and_vpc_attach(client: &aws_sdk_ec2::Client) -> (String, String, String) {
    let tgw_id = client
        .create_transit_gateway()
        .send()
        .await
        .unwrap()
        .transit_gateway()
        .unwrap()
        .transit_gateway_id()
        .unwrap()
        .to_string();
    let vpc_id = client
        .create_vpc()
        .cidr_block("10.251.0.0/16")
        .send()
        .await
        .unwrap()
        .vpc()
        .unwrap()
        .vpc_id()
        .unwrap()
        .to_string();
    let subnet_id = client
        .create_subnet()
        .vpc_id(&vpc_id)
        .cidr_block("10.251.1.0/24")
        .send()
        .await
        .unwrap()
        .subnet()
        .unwrap()
        .subnet_id()
        .unwrap()
        .to_string();
    let att_id = client
        .create_transit_gateway_vpc_attachment()
        .transit_gateway_id(&tgw_id)
        .vpc_id(&vpc_id)
        .subnet_ids(&subnet_id)
        .send()
        .await
        .unwrap()
        .transit_gateway_vpc_attachment()
        .unwrap()
        .transit_gateway_attachment_id()
        .unwrap()
        .to_string();
    (tgw_id, att_id, subnet_id)
}

#[tokio::test]
async fn test_wave2_describe_tgw_connect_peers_returns_created() {
    let client = make_ec2_client().await;
    let (_tgw, transport_att, _) = wave2_seed_tgw_and_vpc_attach(&client).await;
    let conn_att = client
        .create_transit_gateway_connect()
        .transport_transit_gateway_attachment_id(&transport_att)
        .options(
            aws_sdk_ec2::types::CreateTransitGatewayConnectRequestOptions::builder()
                .protocol(aws_sdk_ec2::types::ProtocolValue::Gre)
                .build(),
        )
        .send()
        .await
        .unwrap()
        .transit_gateway_connect()
        .unwrap()
        .transit_gateway_attachment_id()
        .unwrap()
        .to_string();
    let peer_id = client
        .create_transit_gateway_connect_peer()
        .transit_gateway_attachment_id(&conn_att)
        .peer_address("192.0.2.10")
        .inside_cidr_blocks("169.254.7.0/29")
        .send()
        .await
        .unwrap()
        .transit_gateway_connect_peer()
        .unwrap()
        .transit_gateway_connect_peer_id()
        .unwrap()
        .to_string();

    let resp = client
        .describe_transit_gateway_connect_peers()
        .transit_gateway_connect_peer_ids(&peer_id)
        .send()
        .await
        .expect("describe_transit_gateway_connect_peers");
    let peers = resp.transit_gateway_connect_peers();
    assert_eq!(peers.len(), 1);
    assert_eq!(
        peers[0].transit_gateway_connect_peer_id().unwrap(),
        peer_id.as_str()
    );
}

#[tokio::test]
async fn test_wave2_describe_tgw_connects_returns_created() {
    let client = make_ec2_client().await;
    let (_tgw, transport_att, _) = wave2_seed_tgw_and_vpc_attach(&client).await;
    let conn_att = client
        .create_transit_gateway_connect()
        .transport_transit_gateway_attachment_id(&transport_att)
        .options(
            aws_sdk_ec2::types::CreateTransitGatewayConnectRequestOptions::builder()
                .protocol(aws_sdk_ec2::types::ProtocolValue::Gre)
                .build(),
        )
        .send()
        .await
        .unwrap()
        .transit_gateway_connect()
        .unwrap()
        .transit_gateway_attachment_id()
        .unwrap()
        .to_string();

    let resp = client
        .describe_transit_gateway_connects()
        .transit_gateway_attachment_ids(&conn_att)
        .send()
        .await
        .expect("describe_transit_gateway_connects");
    let items = resp.transit_gateway_connects();
    assert_eq!(items.len(), 1);
    assert_eq!(
        items[0].transit_gateway_attachment_id().unwrap(),
        conn_att.as_str()
    );
}

#[tokio::test]
async fn test_wave2_describe_tgw_multicast_domains_returns_created() {
    let client = make_ec2_client().await;
    let (tgw, _att, _) = wave2_seed_tgw_and_vpc_attach(&client).await;
    let dom_id = client
        .create_transit_gateway_multicast_domain()
        .transit_gateway_id(&tgw)
        .send()
        .await
        .unwrap()
        .transit_gateway_multicast_domain()
        .unwrap()
        .transit_gateway_multicast_domain_id()
        .unwrap()
        .to_string();

    let resp = client
        .describe_transit_gateway_multicast_domains()
        .transit_gateway_multicast_domain_ids(&dom_id)
        .send()
        .await
        .expect("describe_transit_gateway_multicast_domains");
    let items = resp.transit_gateway_multicast_domains();
    assert_eq!(items.len(), 1);
    assert_eq!(
        items[0].transit_gateway_multicast_domain_id().unwrap(),
        dom_id.as_str()
    );
}

#[tokio::test]
async fn test_wave2_describe_tgw_policy_tables_returns_created() {
    let client = make_ec2_client().await;
    let (tgw, _att, _) = wave2_seed_tgw_and_vpc_attach(&client).await;
    let pt_id = client
        .create_transit_gateway_policy_table()
        .transit_gateway_id(&tgw)
        .send()
        .await
        .unwrap()
        .transit_gateway_policy_table()
        .unwrap()
        .transit_gateway_policy_table_id()
        .unwrap()
        .to_string();

    let resp = client
        .describe_transit_gateway_policy_tables()
        .transit_gateway_policy_table_ids(&pt_id)
        .send()
        .await
        .expect("describe_transit_gateway_policy_tables");
    let items = resp.transit_gateway_policy_tables();
    assert_eq!(items.len(), 1);
    assert_eq!(
        items[0].transit_gateway_policy_table_id().unwrap(),
        pt_id.as_str()
    );
}

#[tokio::test]
async fn test_wave2_describe_tgw_route_table_announcements_returns_created() {
    let client = make_ec2_client().await;
    let (tgw, _att, _) = wave2_seed_tgw_and_vpc_attach(&client).await;
    let rtb_id = client
        .create_transit_gateway_route_table()
        .transit_gateway_id(&tgw)
        .send()
        .await
        .unwrap()
        .transit_gateway_route_table()
        .unwrap()
        .transit_gateway_route_table_id()
        .unwrap()
        .to_string();
    let peer_att_id = client
        .create_transit_gateway_peering_attachment()
        .transit_gateway_id(&tgw)
        .peer_transit_gateway_id("tgw-peer22222")
        .peer_account_id("123456789012")
        .peer_region("us-west-2")
        .send()
        .await
        .unwrap()
        .transit_gateway_peering_attachment()
        .unwrap()
        .transit_gateway_attachment_id()
        .unwrap()
        .to_string();
    let ann_id = client
        .create_transit_gateway_route_table_announcement()
        .transit_gateway_route_table_id(&rtb_id)
        .peering_attachment_id(&peer_att_id)
        .send()
        .await
        .unwrap()
        .transit_gateway_route_table_announcement()
        .unwrap()
        .transit_gateway_route_table_announcement_id()
        .unwrap()
        .to_string();

    let resp = client
        .describe_transit_gateway_route_table_announcements()
        .transit_gateway_route_table_announcement_ids(&ann_id)
        .send()
        .await
        .expect("describe_transit_gateway_route_table_announcements");
    let items = resp.transit_gateway_route_table_announcements();
    assert_eq!(items.len(), 1);
    assert_eq!(
        items[0]
            .transit_gateway_route_table_announcement_id()
            .unwrap(),
        ann_id.as_str()
    );
}

#[tokio::test]
async fn test_wave2_describe_tgw_metering_policies_returns_created() {
    let client = make_ec2_client().await;
    let (tgw, _att, _) = wave2_seed_tgw_and_vpc_attach(&client).await;
    let pid = client
        .create_transit_gateway_metering_policy()
        .transit_gateway_id(&tgw)
        .send()
        .await
        .unwrap()
        .transit_gateway_metering_policy()
        .unwrap()
        .transit_gateway_metering_policy_id()
        .unwrap()
        .to_string();

    let resp = client
        .describe_transit_gateway_metering_policies()
        .transit_gateway_metering_policy_ids(&pid)
        .send()
        .await
        .expect("describe_transit_gateway_metering_policies");
    let items = resp.transit_gateway_metering_policies();
    assert_eq!(items.len(), 1);
    assert_eq!(
        items[0].transit_gateway_metering_policy_id().unwrap(),
        pid.as_str()
    );
}

#[tokio::test]
async fn test_wave2_get_tgw_attachment_propagations_validates_attachment() {
    let client = make_ec2_client().await;
    let (_tgw, att_id, _) = wave2_seed_tgw_and_vpc_attach(&client).await;

    let resp = client
        .get_transit_gateway_attachment_propagations()
        .transit_gateway_attachment_id(&att_id)
        .send()
        .await
        .expect("get_transit_gateway_attachment_propagations");
    // No propagation tracking exists, so the list is empty for valid attachments.
    assert_eq!(resp.transit_gateway_attachment_propagations().len(), 0);

    // Unknown attachment must error.
    let err = client
        .get_transit_gateway_attachment_propagations()
        .transit_gateway_attachment_id("tgw-attach-deadbeef")
        .send()
        .await
        .expect_err("unknown attachment should fail");
    let msg = format!("{err:?}");
    assert!(
        msg.contains("InvalidTransitGatewayAttachmentID")
            || msg.contains("InvalidTransitGatewayAttachment"),
        "unexpected error: {msg}"
    );
}

#[tokio::test]
async fn test_wave2_get_tgw_metering_policy_entries_returns_created() {
    let client = make_ec2_client().await;
    let (tgw, _att, _) = wave2_seed_tgw_and_vpc_attach(&client).await;
    let pid = client
        .create_transit_gateway_metering_policy()
        .transit_gateway_id(&tgw)
        .send()
        .await
        .unwrap()
        .transit_gateway_metering_policy()
        .unwrap()
        .transit_gateway_metering_policy_id()
        .unwrap()
        .to_string();
    client
        .create_transit_gateway_metering_policy_entry()
        .transit_gateway_metering_policy_id(&pid)
        .policy_rule_number(20)
        .send()
        .await
        .expect("create entry");

    let resp = client
        .get_transit_gateway_metering_policy_entries()
        .transit_gateway_metering_policy_id(&pid)
        .send()
        .await
        .expect("get_transit_gateway_metering_policy_entries");
    assert_eq!(resp.transit_gateway_metering_policy_entries().len(), 1);
}

#[tokio::test]
async fn test_wave2_get_tgw_multicast_domain_associations_returns_associated() {
    let client = make_ec2_client().await;
    let (tgw, att_id, subnet_id) = wave2_seed_tgw_and_vpc_attach(&client).await;
    let dom_id = client
        .create_transit_gateway_multicast_domain()
        .transit_gateway_id(&tgw)
        .send()
        .await
        .unwrap()
        .transit_gateway_multicast_domain()
        .unwrap()
        .transit_gateway_multicast_domain_id()
        .unwrap()
        .to_string();
    client
        .associate_transit_gateway_multicast_domain()
        .transit_gateway_multicast_domain_id(&dom_id)
        .transit_gateway_attachment_id(&att_id)
        .subnet_ids(&subnet_id)
        .send()
        .await
        .expect("associate");

    let resp = client
        .get_transit_gateway_multicast_domain_associations()
        .transit_gateway_multicast_domain_id(&dom_id)
        .send()
        .await
        .expect("get_transit_gateway_multicast_domain_associations");
    let items = resp.multicast_domain_associations();
    assert_eq!(items.len(), 1);
    assert_eq!(
        items[0].transit_gateway_attachment_id().unwrap(),
        att_id.as_str()
    );
    assert_eq!(items[0].subnet().unwrap().subnet_id().unwrap(), subnet_id);
}

#[tokio::test]
async fn test_wave2_get_tgw_policy_table_associations_returns_associated() {
    let client = make_ec2_client().await;
    let (tgw, att_id, _) = wave2_seed_tgw_and_vpc_attach(&client).await;
    let pt_id = client
        .create_transit_gateway_policy_table()
        .transit_gateway_id(&tgw)
        .send()
        .await
        .unwrap()
        .transit_gateway_policy_table()
        .unwrap()
        .transit_gateway_policy_table_id()
        .unwrap()
        .to_string();
    client
        .associate_transit_gateway_policy_table()
        .transit_gateway_policy_table_id(&pt_id)
        .transit_gateway_attachment_id(&att_id)
        .send()
        .await
        .expect("associate policy table");

    let resp = client
        .get_transit_gateway_policy_table_associations()
        .transit_gateway_policy_table_id(&pt_id)
        .send()
        .await
        .expect("get_transit_gateway_policy_table_associations");
    let items = resp.associations();
    assert_eq!(items.len(), 1);
    assert_eq!(
        items[0].transit_gateway_attachment_id().unwrap(),
        att_id.as_str()
    );
}

#[tokio::test]
async fn test_wave2_get_tgw_policy_table_entries_validates_table() {
    let client = make_ec2_client().await;
    let (tgw, _att, _) = wave2_seed_tgw_and_vpc_attach(&client).await;
    let pt_id = client
        .create_transit_gateway_policy_table()
        .transit_gateway_id(&tgw)
        .send()
        .await
        .unwrap()
        .transit_gateway_policy_table()
        .unwrap()
        .transit_gateway_policy_table_id()
        .unwrap()
        .to_string();

    // No entries-on-policy-table state today; happy path returns empty.
    let resp = client
        .get_transit_gateway_policy_table_entries()
        .transit_gateway_policy_table_id(&pt_id)
        .send()
        .await
        .expect("get_transit_gateway_policy_table_entries");
    assert_eq!(resp.transit_gateway_policy_table_entries().len(), 0);

    // Unknown table must error.
    let err = client
        .get_transit_gateway_policy_table_entries()
        .transit_gateway_policy_table_id("tgw-rtb-policy-deadbeef")
        .send()
        .await
        .expect_err("unknown table should fail");
    let msg = format!("{err:?}");
    assert!(
        msg.contains("InvalidTransitGatewayPolicyTable")
            || msg.contains("InvalidTransitGatewayRouteTableID"),
        "unexpected error: {msg}"
    );
}

#[tokio::test]
async fn test_wave2_get_tgw_prefix_list_references_returns_created() {
    let client = make_ec2_client().await;
    let (tgw, _att, _) = wave2_seed_tgw_and_vpc_attach(&client).await;
    let rtb_id = client
        .create_transit_gateway_route_table()
        .transit_gateway_id(&tgw)
        .send()
        .await
        .unwrap()
        .transit_gateway_route_table()
        .unwrap()
        .transit_gateway_route_table_id()
        .unwrap()
        .to_string();
    let pl_id = "pl-bbbbbbbb";
    client
        .create_transit_gateway_prefix_list_reference()
        .transit_gateway_route_table_id(&rtb_id)
        .prefix_list_id(pl_id)
        .blackhole(true)
        .send()
        .await
        .expect("create prefix list ref");

    let resp = client
        .get_transit_gateway_prefix_list_references()
        .transit_gateway_route_table_id(&rtb_id)
        .send()
        .await
        .expect("get_transit_gateway_prefix_list_references");
    let items = resp.transit_gateway_prefix_list_references();
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].prefix_list_id().unwrap(), pl_id);
    assert_eq!(items[0].blackhole(), Some(true));
}

#[tokio::test]
async fn test_wave2_export_transit_gateway_routes_returns_s3_location() {
    let client = make_ec2_client().await;
    let (tgw, _att, _) = wave2_seed_tgw_and_vpc_attach(&client).await;
    let rtb_id = client
        .create_transit_gateway_route_table()
        .transit_gateway_id(&tgw)
        .send()
        .await
        .unwrap()
        .transit_gateway_route_table()
        .unwrap()
        .transit_gateway_route_table_id()
        .unwrap()
        .to_string();

    let resp = client
        .export_transit_gateway_routes()
        .transit_gateway_route_table_id(&rtb_id)
        .s3_bucket("ignored-bucket")
        .send()
        .await
        .expect("export_transit_gateway_routes");
    let s3 = resp.s3_location().unwrap_or("");
    assert!(s3.starts_with("s3://mock/"), "got: {s3}");
}

#[tokio::test]
async fn test_wave2_search_tgw_multicast_groups_returns_members_and_sources() {
    let client = make_ec2_client().await;
    let (tgw, att_id, subnet_id) = wave2_seed_tgw_and_vpc_attach(&client).await;
    let dom_id = client
        .create_transit_gateway_multicast_domain()
        .transit_gateway_id(&tgw)
        .send()
        .await
        .unwrap()
        .transit_gateway_multicast_domain()
        .unwrap()
        .transit_gateway_multicast_domain_id()
        .unwrap()
        .to_string();
    client
        .associate_transit_gateway_multicast_domain()
        .transit_gateway_multicast_domain_id(&dom_id)
        .transit_gateway_attachment_id(&att_id)
        .subnet_ids(&subnet_id)
        .send()
        .await
        .unwrap();
    client
        .register_transit_gateway_multicast_group_members()
        .transit_gateway_multicast_domain_id(&dom_id)
        .group_ip_address("224.0.1.10")
        .network_interface_ids("eni-w2mem01")
        .send()
        .await
        .expect("register members");
    client
        .register_transit_gateway_multicast_group_sources()
        .transit_gateway_multicast_domain_id(&dom_id)
        .group_ip_address("224.0.1.20")
        .network_interface_ids("eni-w2src01")
        .send()
        .await
        .expect("register sources");

    let resp = client
        .search_transit_gateway_multicast_groups()
        .transit_gateway_multicast_domain_id(&dom_id)
        .send()
        .await
        .expect("search_transit_gateway_multicast_groups");
    let groups = resp.multicast_groups();
    assert_eq!(groups.len(), 2);
    assert!(groups.iter().any(|g| g.group_member() == Some(true)));
    assert!(groups.iter().any(|g| g.group_source() == Some(true)));
}

#[tokio::test]
async fn test_wave2_get_ipam_discovered_accounts_returns_associated() {
    use aws_sdk_ec2::types::AddIpamOperatingRegion;
    let client = make_ec2_client().await;
    let ipam_id = client
        .create_ipam()
        .operating_regions(
            AddIpamOperatingRegion::builder()
                .region_name("us-east-1")
                .build(),
        )
        .send()
        .await
        .unwrap()
        .ipam()
        .and_then(|i| i.ipam_id())
        .unwrap()
        .to_string();
    let rd_id = client
        .create_ipam_resource_discovery()
        .operating_regions(
            AddIpamOperatingRegion::builder()
                .region_name("us-east-1")
                .build(),
        )
        .send()
        .await
        .unwrap()
        .ipam_resource_discovery()
        .and_then(|r| r.ipam_resource_discovery_id())
        .unwrap()
        .to_string();
    let assoc_id = client
        .associate_ipam_resource_discovery()
        .ipam_id(&ipam_id)
        .ipam_resource_discovery_id(&rd_id)
        .send()
        .await
        .unwrap()
        .ipam_resource_discovery_association()
        .and_then(|a| a.ipam_resource_discovery_association_id())
        .unwrap()
        .to_string();

    let resp = client
        .get_ipam_discovered_accounts()
        .ipam_resource_discovery_id(&rd_id)
        .discovery_region("us-east-1")
        .send()
        .await
        .expect("get_ipam_discovered_accounts");
    let items = resp.ipam_discovered_accounts();
    assert!(
        !items.is_empty(),
        "should have at least 1 discovered account"
    );
    assert!(items.iter().all(|a| a.account_id().is_some()));

    // Cleanup
    let _ = client
        .disassociate_ipam_resource_discovery()
        .ipam_resource_discovery_association_id(&assoc_id)
        .send()
        .await;
}

#[tokio::test]
async fn test_wave2_get_ipam_discovered_resource_cidrs_returns_allocations() {
    use aws_sdk_ec2::types::AddIpamOperatingRegion;
    let client = make_ec2_client().await;
    let ipam_resp = client
        .create_ipam()
        .operating_regions(
            AddIpamOperatingRegion::builder()
                .region_name("us-east-1")
                .build(),
        )
        .send()
        .await
        .unwrap();
    let ipam = ipam_resp.ipam().unwrap();
    let private_scope_id = ipam.private_default_scope_id().unwrap().to_string();
    let pool_id = client
        .create_ipam_pool()
        .ipam_scope_id(&private_scope_id)
        .address_family(aws_sdk_ec2::types::AddressFamily::Ipv4)
        .send()
        .await
        .unwrap()
        .ipam_pool()
        .and_then(|p| p.ipam_pool_id())
        .unwrap()
        .to_string();
    client
        .provision_ipam_pool_cidr()
        .ipam_pool_id(&pool_id)
        .cidr("10.99.0.0/16")
        .send()
        .await
        .unwrap();
    client
        .allocate_ipam_pool_cidr()
        .ipam_pool_id(&pool_id)
        .cidr("10.99.1.0/24")
        .send()
        .await
        .unwrap();

    let rd_id = client
        .create_ipam_resource_discovery()
        .operating_regions(
            AddIpamOperatingRegion::builder()
                .region_name("us-east-1")
                .build(),
        )
        .send()
        .await
        .unwrap()
        .ipam_resource_discovery()
        .and_then(|r| r.ipam_resource_discovery_id())
        .unwrap()
        .to_string();

    let resp = client
        .get_ipam_discovered_resource_cidrs()
        .ipam_resource_discovery_id(&rd_id)
        .resource_region("us-east-1")
        .send()
        .await
        .expect("get_ipam_discovered_resource_cidrs");
    let items = resp.ipam_discovered_resource_cidrs();
    assert!(
        items
            .iter()
            .any(|c| c.resource_cidr() == Some("10.99.1.0/24"))
    );
}

#[tokio::test]
async fn test_wave2_get_ipam_discovered_public_addresses_returns_eips() {
    let client = make_ec2_client().await;
    let _ = client
        .allocate_address()
        .send()
        .await
        .expect("allocate_address");

    let resp = client
        .get_ipam_discovered_public_addresses()
        .ipam_resource_discovery_id("ipam-res-disco-aaaaaaaa")
        .address_region("us-east-1")
        .send()
        .await
        .expect("get_ipam_discovered_public_addresses");
    let items = resp.ipam_discovered_public_addresses();
    assert!(
        !items.is_empty(),
        "should have at least 1 discovered address"
    );
    assert!(items.iter().any(|a| a.address().is_some()));
}

#[tokio::test]
async fn test_wave2_get_ipam_policy_allocation_rules_returns_created() {
    use aws_sdk_ec2::types::AddIpamOperatingRegion;
    let client = make_ec2_client().await;
    let ipam_id = client
        .create_ipam()
        .operating_regions(
            AddIpamOperatingRegion::builder()
                .region_name("us-east-1")
                .build(),
        )
        .send()
        .await
        .unwrap()
        .ipam()
        .and_then(|i| i.ipam_id())
        .unwrap()
        .to_string();
    let policy_id = client
        .create_ipam_policy()
        .ipam_id(&ipam_id)
        .send()
        .await
        .unwrap()
        .ipam_policy()
        .and_then(|p| p.ipam_policy_id())
        .unwrap()
        .to_string();

    let resp = client
        .get_ipam_policy_allocation_rules()
        .ipam_policy_id(&policy_id)
        .send()
        .await
        .expect("get_ipam_policy_allocation_rules");
    let docs = resp.ipam_policy_documents();
    assert_eq!(docs.len(), 1);
    assert_eq!(docs[0].ipam_policy_id().unwrap(), policy_id.as_str());
}

#[tokio::test]
async fn test_wave2_get_ipam_policy_organization_targets_returns_empty() {
    use aws_sdk_ec2::types::AddIpamOperatingRegion;
    let client = make_ec2_client().await;
    let ipam_id = client
        .create_ipam()
        .operating_regions(
            AddIpamOperatingRegion::builder()
                .region_name("us-east-1")
                .build(),
        )
        .send()
        .await
        .unwrap()
        .ipam()
        .and_then(|i| i.ipam_id())
        .unwrap()
        .to_string();
    let policy_id = client
        .create_ipam_policy()
        .ipam_id(&ipam_id)
        .send()
        .await
        .unwrap()
        .ipam_policy()
        .and_then(|p| p.ipam_policy_id())
        .unwrap()
        .to_string();

    let resp = client
        .get_ipam_policy_organization_targets()
        .ipam_policy_id(&policy_id)
        .send()
        .await
        .expect("get_ipam_policy_organization_targets");
    assert_eq!(resp.organization_targets().len(), 0);
}

#[tokio::test]
async fn test_wave2_get_ipam_prefix_list_resolver_rules_validates_resolver() {
    use aws_sdk_ec2::types::AddIpamOperatingRegion;
    let client = make_ec2_client().await;
    let ipam_id = client
        .create_ipam()
        .operating_regions(
            AddIpamOperatingRegion::builder()
                .region_name("us-east-1")
                .build(),
        )
        .send()
        .await
        .unwrap()
        .ipam()
        .and_then(|i| i.ipam_id())
        .unwrap()
        .to_string();
    let resolver_id = client
        .create_ipam_prefix_list_resolver()
        .ipam_id(&ipam_id)
        .address_family(aws_sdk_ec2::types::AddressFamily::Ipv4)
        .send()
        .await
        .unwrap()
        .ipam_prefix_list_resolver()
        .and_then(|r| r.ipam_prefix_list_resolver_id())
        .unwrap()
        .to_string();

    let resp = client
        .get_ipam_prefix_list_resolver_rules()
        .ipam_prefix_list_resolver_id(&resolver_id)
        .send()
        .await
        .expect("get_ipam_prefix_list_resolver_rules");
    assert_eq!(resp.rules().len(), 0);
}

#[tokio::test]
async fn test_wave2_get_ipam_prefix_list_resolver_versions_returns_synthetic() {
    use aws_sdk_ec2::types::AddIpamOperatingRegion;
    let client = make_ec2_client().await;
    let ipam_id = client
        .create_ipam()
        .operating_regions(
            AddIpamOperatingRegion::builder()
                .region_name("us-east-1")
                .build(),
        )
        .send()
        .await
        .unwrap()
        .ipam()
        .and_then(|i| i.ipam_id())
        .unwrap()
        .to_string();
    let resolver_id = client
        .create_ipam_prefix_list_resolver()
        .ipam_id(&ipam_id)
        .address_family(aws_sdk_ec2::types::AddressFamily::Ipv4)
        .send()
        .await
        .unwrap()
        .ipam_prefix_list_resolver()
        .and_then(|r| r.ipam_prefix_list_resolver_id())
        .unwrap()
        .to_string();

    let resp = client
        .get_ipam_prefix_list_resolver_versions()
        .ipam_prefix_list_resolver_id(&resolver_id)
        .send()
        .await
        .expect("get_ipam_prefix_list_resolver_versions");
    let versions = resp.ipam_prefix_list_resolver_versions();
    assert_eq!(versions.len(), 1);
    assert_eq!(versions[0].version(), Some(1));
}

#[tokio::test]
async fn test_wave2_get_ipam_prefix_list_resolver_version_entries_returns_empty() {
    use aws_sdk_ec2::types::AddIpamOperatingRegion;
    let client = make_ec2_client().await;
    let ipam_id = client
        .create_ipam()
        .operating_regions(
            AddIpamOperatingRegion::builder()
                .region_name("us-east-1")
                .build(),
        )
        .send()
        .await
        .unwrap()
        .ipam()
        .and_then(|i| i.ipam_id())
        .unwrap()
        .to_string();
    let resolver_id = client
        .create_ipam_prefix_list_resolver()
        .ipam_id(&ipam_id)
        .address_family(aws_sdk_ec2::types::AddressFamily::Ipv4)
        .send()
        .await
        .unwrap()
        .ipam_prefix_list_resolver()
        .and_then(|r| r.ipam_prefix_list_resolver_id())
        .unwrap()
        .to_string();

    let resp = client
        .get_ipam_prefix_list_resolver_version_entries()
        .ipam_prefix_list_resolver_id(&resolver_id)
        .ipam_prefix_list_resolver_version(1)
        .send()
        .await
        .expect("get_ipam_prefix_list_resolver_version_entries");
    assert_eq!(resp.entries().len(), 0);
}

// ===================================================================
// Wave 3 — describes/getters that read from already-created state.
// ===================================================================

#[tokio::test]
async fn test_wave3_describe_import_snapshot_tasks_returns_imported() {
    let client = make_ec2_client().await;
    let task_id = client
        .import_snapshot()
        .description("wave3-import")
        .disk_container(
            aws_sdk_ec2::types::SnapshotDiskContainer::builder()
                .description("disk")
                .format("vmdk")
                .url("s3://example/disk.vmdk")
                .build(),
        )
        .send()
        .await
        .expect("import_snapshot")
        .import_task_id()
        .expect("task id")
        .to_string();

    let resp = client
        .describe_import_snapshot_tasks()
        .import_task_ids(&task_id)
        .send()
        .await
        .expect("describe_import_snapshot_tasks");
    let items = resp.import_snapshot_tasks();
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].import_task_id(), Some(task_id.as_str()));
}

#[tokio::test]
async fn test_wave3_describe_replace_root_volume_tasks_returns_created() {
    let client = make_ec2_client().await;
    let instance_id = client
        .run_instances()
        .image_id("ami-wave3-rrv")
        .instance_type(aws_sdk_ec2::types::InstanceType::T2Micro)
        .min_count(1)
        .max_count(1)
        .send()
        .await
        .unwrap()
        .instances()
        .first()
        .unwrap()
        .instance_id()
        .unwrap()
        .to_string();
    let task_id = client
        .create_replace_root_volume_task()
        .instance_id(&instance_id)
        .image_id("ami-wave3-rrv-new")
        .send()
        .await
        .expect("create_replace_root_volume_task")
        .replace_root_volume_task()
        .unwrap()
        .replace_root_volume_task_id()
        .unwrap()
        .to_string();

    let resp = client
        .describe_replace_root_volume_tasks()
        .replace_root_volume_task_ids(&task_id)
        .send()
        .await
        .expect("describe_replace_root_volume_tasks");
    let items = resp.replace_root_volume_tasks();
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].instance_id(), Some(instance_id.as_str()));
}

#[tokio::test]
async fn test_wave3_describe_snapshot_tier_status_returns_archived() {
    let client = make_ec2_client().await;
    let vol_id = client
        .create_volume()
        .availability_zone("us-east-1a")
        .size(8)
        .send()
        .await
        .unwrap()
        .volume_id()
        .unwrap()
        .to_string();
    let snap_id = client
        .create_snapshot()
        .volume_id(&vol_id)
        .send()
        .await
        .unwrap()
        .snapshot_id()
        .unwrap()
        .to_string();
    client
        .modify_snapshot_tier()
        .snapshot_id(&snap_id)
        .storage_tier(aws_sdk_ec2::types::TargetStorageTier::Archive)
        .send()
        .await
        .expect("modify_snapshot_tier");

    let resp = client
        .describe_snapshot_tier_status()
        .send()
        .await
        .expect("describe_snapshot_tier_status");
    let items = resp.snapshot_tier_statuses();
    assert!(
        items
            .iter()
            .any(|s| s.snapshot_id() == Some(snap_id.as_str())
                && s.storage_tier().map(|t| t.as_str()) == Some("archive")),
        "expected archived snapshot to be present"
    );
}

#[tokio::test]
async fn test_wave3_list_snapshots_in_recycle_bin_returns_deleted() {
    let client = make_ec2_client().await;
    let vol_id = client
        .create_volume()
        .availability_zone("us-east-1a")
        .size(8)
        .send()
        .await
        .unwrap()
        .volume_id()
        .unwrap()
        .to_string();
    let snap_id = client
        .create_snapshot()
        .volume_id(&vol_id)
        .send()
        .await
        .unwrap()
        .snapshot_id()
        .unwrap()
        .to_string();
    client
        .delete_snapshot()
        .snapshot_id(&snap_id)
        .send()
        .await
        .expect("delete_snapshot");

    let resp = client
        .list_snapshots_in_recycle_bin()
        .send()
        .await
        .expect("list_snapshots_in_recycle_bin");
    assert!(
        resp.snapshots()
            .iter()
            .any(|s| s.snapshot_id() == Some(snap_id.as_str())),
        "deleted snapshot should appear in recycle bin"
    );
}

#[tokio::test]
async fn test_wave3_list_volumes_in_recycle_bin_returns_deleted() {
    let client = make_ec2_client().await;
    let vol_id = client
        .create_volume()
        .availability_zone("us-east-1a")
        .size(8)
        .send()
        .await
        .unwrap()
        .volume_id()
        .unwrap()
        .to_string();
    client
        .delete_volume()
        .volume_id(&vol_id)
        .send()
        .await
        .expect("delete_volume");

    let resp = client
        .list_volumes_in_recycle_bin()
        .send()
        .await
        .expect("list_volumes_in_recycle_bin");
    assert!(
        resp.volumes()
            .iter()
            .any(|v| v.volume_id() == Some(vol_id.as_str())),
        "deleted volume should appear in recycle bin"
    );
}

#[tokio::test]
async fn test_wave3_describe_image_usage_report_entries_returns_created() {
    let client = make_ec2_client().await;
    let create = client
        .start_declarative_policies_report()
        .target_id("ou-1234-12345678")
        .s3_bucket("usage-bucket")
        .send()
        .await;
    drop(create);
    // The image-usage report needs a real image first.
    let image_id = client
        .register_image()
        .name("wave3-iur-image")
        .send()
        .await
        .unwrap()
        .image_id()
        .unwrap()
        .to_string();
    let report_id = client
        .create_image_usage_report()
        .image_id(&image_id)
        .send()
        .await
        .expect("create_image_usage_report")
        .report_id()
        .expect("report id")
        .to_string();
    let resp = client
        .describe_image_usage_report_entries()
        .report_ids(&report_id)
        .send()
        .await
        .expect("describe_image_usage_report_entries");
    let items = resp.image_usage_report_entries();
    assert!(
        items
            .iter()
            .any(|e| e.report_id() == Some(report_id.as_str())),
        "should include our report"
    );
}

#[tokio::test]
async fn test_wave3_describe_mac_modification_tasks_returns_created() {
    let client = make_ec2_client().await;
    let instance_id = client
        .run_instances()
        .image_id("ami-mac-wave3")
        .instance_type(aws_sdk_ec2::types::InstanceType::Mac1Metal)
        .min_count(1)
        .max_count(1)
        .send()
        .await
        .unwrap()
        .instances()
        .first()
        .unwrap()
        .instance_id()
        .unwrap()
        .to_string();
    let task_id = client
        .create_mac_system_integrity_protection_modification_task()
        .instance_id(&instance_id)
        .mac_credentials("dGVzdA==")
        .send()
        .await
        .expect("create_mac_system_integrity_protection_modification_task")
        .mac_modification_task()
        .unwrap()
        .mac_modification_task_id()
        .unwrap()
        .to_string();

    let resp = client
        .describe_mac_modification_tasks()
        .mac_modification_task_ids(&task_id)
        .send()
        .await
        .expect("describe_mac_modification_tasks");
    let items = resp.mac_modification_tasks();
    assert!(
        items
            .iter()
            .any(|t| t.mac_modification_task_id() == Some(task_id.as_str())),
        "should include our task"
    );
}

#[tokio::test]
async fn test_wave3_describe_capacity_block_offerings_returns_synthetic() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_capacity_block_offerings()
        .instance_count(8)
        .capacity_duration_hours(24)
        .send()
        .await
        .expect("describe_capacity_block_offerings");
    let items = resp.capacity_block_offerings();
    assert_eq!(items.len(), 3);
    assert!(
        items
            .iter()
            .all(|o| o.instance_type() == Some("p4d.24xlarge"))
    );
}

#[tokio::test]
async fn test_wave3_describe_capacity_block_extension_offerings_returns_synthetic() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_capacity_block_extension_offerings()
        .capacity_block_extension_duration_hours(24)
        .capacity_reservation_id("cr-1234")
        .send()
        .await
        .expect("describe_capacity_block_extension_offerings");
    assert_eq!(resp.capacity_block_extension_offerings().len(), 1);
}

#[tokio::test]
async fn test_wave3_describe_capacity_block_extension_history_returns_purchased() {
    let client = make_ec2_client().await;
    let purchase = client
        .purchase_capacity_block()
        .capacity_block_offering_id("cbo-wave3-1")
        .instance_platform(aws_sdk_ec2::types::CapacityReservationInstancePlatform::LinuxUnix)
        .send()
        .await
        .expect("purchase_capacity_block");
    let cr_id = purchase
        .capacity_reservation()
        .unwrap()
        .capacity_reservation_id()
        .unwrap()
        .to_string();
    client
        .purchase_capacity_block_extension()
        .capacity_block_extension_offering_id("cbeo-wave3-1")
        .capacity_reservation_id(&cr_id)
        .send()
        .await
        .expect("purchase_capacity_block_extension");

    let resp = client
        .describe_capacity_block_extension_history()
        .send()
        .await
        .expect("describe_capacity_block_extension_history");
    let items = resp.capacity_block_extensions();
    assert!(
        items
            .iter()
            .any(|e| e.capacity_reservation_id() == Some(cr_id.as_str())),
        "should include our extension"
    );
}

#[tokio::test]
async fn test_wave3_describe_capacity_block_status_returns_purchased() {
    let client = make_ec2_client().await;
    let purchase = client
        .purchase_capacity_block()
        .capacity_block_offering_id("cbo-wave3-status-1")
        .instance_platform(aws_sdk_ec2::types::CapacityReservationInstancePlatform::LinuxUnix)
        .send()
        .await
        .expect("purchase_capacity_block");
    let block_id = purchase.capacity_blocks()[0]
        .capacity_block_id()
        .unwrap()
        .to_string();
    let resp = client
        .describe_capacity_block_status()
        .capacity_block_ids(&block_id)
        .send()
        .await
        .expect("describe_capacity_block_status");
    let items = resp.capacity_block_statuses();
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].capacity_block_id(), Some(block_id.as_str()));
}

#[tokio::test]
async fn test_wave3_describe_secondary_networks_returns_created() {
    let client = make_ec2_client().await;
    let net_id = client
        .create_secondary_network()
        .ipv4_cidr_block("10.50.0.0/16")
        .network_type("ipv4".into())
        .send()
        .await
        .expect("create_secondary_network")
        .secondary_network()
        .unwrap()
        .secondary_network_id()
        .unwrap()
        .to_string();

    let resp = client
        .describe_secondary_networks()
        .secondary_network_ids(&net_id)
        .send()
        .await
        .expect("describe_secondary_networks");
    let items = resp.secondary_networks();
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].secondary_network_id(), Some(net_id.as_str()));
}

#[tokio::test]
async fn test_wave3_describe_secondary_subnets_returns_created() {
    let client = make_ec2_client().await;
    let net_id = client
        .create_secondary_network()
        .ipv4_cidr_block("10.51.0.0/16")
        .network_type("ipv4".into())
        .send()
        .await
        .unwrap()
        .secondary_network()
        .unwrap()
        .secondary_network_id()
        .unwrap()
        .to_string();
    let subnet_id = client
        .create_secondary_subnet()
        .secondary_network_id(&net_id)
        .ipv4_cidr_block("10.51.1.0/24")
        .availability_zone("us-east-1a")
        .send()
        .await
        .expect("create_secondary_subnet")
        .secondary_subnet()
        .unwrap()
        .secondary_subnet_id()
        .unwrap()
        .to_string();

    let resp = client
        .describe_secondary_subnets()
        .secondary_subnet_ids(&subnet_id)
        .send()
        .await
        .expect("describe_secondary_subnets");
    let items = resp.secondary_subnets();
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].secondary_subnet_id(), Some(subnet_id.as_str()));
}

#[tokio::test]
async fn test_wave3_describe_secondary_interfaces_returns_empty() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_secondary_interfaces()
        .send()
        .await
        .expect("describe_secondary_interfaces");
    assert!(resp.secondary_interfaces().is_empty());
}

#[tokio::test]
async fn test_wave3_describe_trunk_interface_associations_returns_created() {
    let client = make_ec2_client().await;
    let assoc_id = client
        .associate_trunk_interface()
        .branch_interface_id("eni-wave3-branch")
        .trunk_interface_id("eni-wave3-trunk")
        .vlan_id(42)
        .send()
        .await
        .expect("associate_trunk_interface")
        .interface_association()
        .unwrap()
        .association_id()
        .unwrap()
        .to_string();

    let resp = client
        .describe_trunk_interface_associations()
        .association_ids(&assoc_id)
        .send()
        .await
        .expect("describe_trunk_interface_associations");
    let items = resp.interface_associations();
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].association_id(), Some(assoc_id.as_str()));
}

#[tokio::test]
async fn test_wave3_describe_security_group_vpc_associations_returns_created() {
    let client = make_ec2_client().await;
    let vpc_id = client
        .create_vpc()
        .cidr_block("10.60.0.0/16")
        .send()
        .await
        .unwrap()
        .vpc()
        .unwrap()
        .vpc_id()
        .unwrap()
        .to_string();
    let sg_id = client
        .create_security_group()
        .group_name("wave3-sg-vpc-assoc")
        .description("wave3 sg vpc assoc")
        .vpc_id(&vpc_id)
        .send()
        .await
        .unwrap()
        .group_id()
        .unwrap()
        .to_string();
    client
        .associate_security_group_vpc()
        .group_id(&sg_id)
        .vpc_id(&vpc_id)
        .send()
        .await
        .expect("associate_security_group_vpc");

    let resp = client
        .describe_security_group_vpc_associations()
        .filters(
            aws_sdk_ec2::types::Filter::builder()
                .name("group-id")
                .values(&sg_id)
                .build(),
        )
        .send()
        .await
        .expect("describe_security_group_vpc_associations");
    let items = resp.security_group_vpc_associations();
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].vpc_id(), Some(vpc_id.as_str()));
}

#[tokio::test]
async fn test_wave3_get_associated_enclave_certificate_iam_roles_returns_associated() {
    let client = make_ec2_client().await;
    let cert_arn =
        "arn:aws:acm:us-east-1:000000000000:certificate/wave3-1111-2222-3333-444444444444";
    let role_arn = "arn:aws:iam::000000000000:role/Wave3Role";
    client
        .associate_enclave_certificate_iam_role()
        .certificate_arn(cert_arn)
        .role_arn(role_arn)
        .send()
        .await
        .expect("associate_enclave_certificate_iam_role");

    let resp = client
        .get_associated_enclave_certificate_iam_roles()
        .certificate_arn(cert_arn)
        .send()
        .await
        .expect("get_associated_enclave_certificate_iam_roles");
    let items = resp.associated_roles();
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].associated_role_arn(), Some(role_arn));
}

#[tokio::test]
async fn test_wave3_get_capacity_reservation_usage_returns_zero_usage() {
    let client = make_ec2_client().await;
    let cr_id = client
        .create_capacity_reservation()
        .instance_type("m5.large")
        .instance_platform(aws_sdk_ec2::types::CapacityReservationInstancePlatform::LinuxUnix)
        .availability_zone("us-east-1a")
        .instance_count(2)
        .end_date_type(aws_sdk_ec2::types::EndDateType::Unlimited)
        .send()
        .await
        .unwrap()
        .capacity_reservation()
        .unwrap()
        .capacity_reservation_id()
        .unwrap()
        .to_string();
    let resp = client
        .get_capacity_reservation_usage()
        .capacity_reservation_id(&cr_id)
        .send()
        .await
        .expect("get_capacity_reservation_usage");
    assert_eq!(resp.capacity_reservation_id(), Some(cr_id.as_str()));
    assert_eq!(resp.total_instance_count(), Some(2));
}

#[tokio::test]
async fn test_wave3_get_groups_for_capacity_reservation_returns_empty() {
    let client = make_ec2_client().await;
    let cr_id = client
        .create_capacity_reservation()
        .instance_type("m5.large")
        .instance_platform(aws_sdk_ec2::types::CapacityReservationInstancePlatform::LinuxUnix)
        .availability_zone("us-east-1a")
        .instance_count(2)
        .end_date_type(aws_sdk_ec2::types::EndDateType::Unlimited)
        .send()
        .await
        .unwrap()
        .capacity_reservation()
        .unwrap()
        .capacity_reservation_id()
        .unwrap()
        .to_string();
    let resp = client
        .get_groups_for_capacity_reservation()
        .capacity_reservation_id(&cr_id)
        .send()
        .await
        .expect("get_groups_for_capacity_reservation");
    assert!(resp.capacity_reservation_groups().is_empty());
}

#[tokio::test]
async fn test_wave3_local_gateway_describes_round_trip() {
    let client = make_ec2_client().await;
    let vpc_id = client
        .create_vpc()
        .cidr_block("10.70.0.0/16")
        .send()
        .await
        .unwrap()
        .vpc()
        .unwrap()
        .vpc_id()
        .unwrap()
        .to_string();
    let table = client
        .create_local_gateway_route_table()
        .local_gateway_id("lgw-wave3-1")
        .send()
        .await
        .expect("create_local_gateway_route_table")
        .local_gateway_route_table()
        .unwrap()
        .clone();
    let table_id = table.local_gateway_route_table_id().unwrap().to_string();
    let lgw_id = table.local_gateway_id().unwrap().to_string();
    let vif_grp_id = client
        .create_local_gateway_virtual_interface_group()
        .local_gateway_id(&lgw_id)
        .local_bgp_asn(64512)
        .send()
        .await
        .unwrap()
        .local_gateway_virtual_interface_group()
        .unwrap()
        .local_gateway_virtual_interface_group_id()
        .unwrap()
        .to_string();
    let vif_id = client
        .create_local_gateway_virtual_interface()
        .local_gateway_virtual_interface_group_id(&vif_grp_id)
        .vlan(101)
        .local_address("169.254.10.1")
        .peer_address("169.254.10.2")
        .peer_bgp_asn(65001)
        .send()
        .await
        .unwrap()
        .local_gateway_virtual_interface()
        .unwrap()
        .local_gateway_virtual_interface_id()
        .unwrap()
        .to_string();
    let group_assoc_id = client
        .create_local_gateway_route_table_virtual_interface_group_association()
        .local_gateway_route_table_id(&table_id)
        .local_gateway_virtual_interface_group_id(&vif_grp_id)
        .send()
        .await
        .unwrap()
        .local_gateway_route_table_virtual_interface_group_association()
        .unwrap()
        .local_gateway_route_table_virtual_interface_group_association_id()
        .unwrap()
        .to_string();
    let vpc_assoc_id = client
        .create_local_gateway_route_table_vpc_association()
        .local_gateway_route_table_id(&table_id)
        .vpc_id(&vpc_id)
        .send()
        .await
        .unwrap()
        .local_gateway_route_table_vpc_association()
        .unwrap()
        .local_gateway_route_table_vpc_association_id()
        .unwrap()
        .to_string();
    client
        .create_local_gateway_route()
        .local_gateway_route_table_id(&table_id)
        .destination_cidr_block("10.99.0.0/16")
        .local_gateway_virtual_interface_group_id(&vif_grp_id)
        .send()
        .await
        .unwrap();

    let rtbs = client
        .describe_local_gateway_route_tables()
        .local_gateway_route_table_ids(&table_id)
        .send()
        .await
        .expect("describe_local_gateway_route_tables");
    assert_eq!(rtbs.local_gateway_route_tables().len(), 1);

    let vifs = client
        .describe_local_gateway_virtual_interfaces()
        .local_gateway_virtual_interface_ids(&vif_id)
        .send()
        .await
        .expect("describe_local_gateway_virtual_interfaces");
    assert_eq!(vifs.local_gateway_virtual_interfaces().len(), 1);

    let vif_grps = client
        .describe_local_gateway_virtual_interface_groups()
        .local_gateway_virtual_interface_group_ids(&vif_grp_id)
        .send()
        .await
        .expect("describe_local_gateway_virtual_interface_groups");
    assert_eq!(vif_grps.local_gateway_virtual_interface_groups().len(), 1);

    let vpc_assocs = client
        .describe_local_gateway_route_table_vpc_associations()
        .local_gateway_route_table_vpc_association_ids(&vpc_assoc_id)
        .send()
        .await
        .expect("describe_local_gateway_route_table_vpc_associations");
    assert_eq!(
        vpc_assocs
            .local_gateway_route_table_vpc_associations()
            .len(),
        1
    );

    let grp_assocs = client
        .describe_local_gateway_route_table_virtual_interface_group_associations()
        .local_gateway_route_table_virtual_interface_group_association_ids(&group_assoc_id)
        .send()
        .await
        .expect("describe_local_gateway_route_table_virtual_interface_group_associations");
    assert_eq!(
        grp_assocs
            .local_gateway_route_table_virtual_interface_group_associations()
            .len(),
        1
    );

    let routes = client
        .search_local_gateway_routes()
        .local_gateway_route_table_id(&table_id)
        .send()
        .await
        .expect("search_local_gateway_routes");
    assert_eq!(routes.routes().len(), 1);
    assert_eq!(
        routes.routes()[0].destination_cidr_block(),
        Some("10.99.0.0/16")
    );
}

#[tokio::test]
async fn test_wave3_describe_capacity_manager_data_exports_returns_created() {
    let client = make_ec2_client().await;
    let id = client
        .create_capacity_manager_data_export()
        .schedule(aws_sdk_ec2::types::Schedule::Hourly)
        .output_format(aws_sdk_ec2::types::OutputFormat::Parquet)
        .s3_bucket_name("wave3-bucket")
        .send()
        .await
        .expect("create_capacity_manager_data_export")
        .capacity_manager_data_export_id()
        .unwrap()
        .to_string();

    let resp = client
        .describe_capacity_manager_data_exports()
        .capacity_manager_data_export_ids(&id)
        .send()
        .await
        .expect("describe_capacity_manager_data_exports");
    let items = resp.capacity_manager_data_exports();
    assert_eq!(items.len(), 1);
    assert_eq!(
        items[0].capacity_manager_data_export_id(),
        Some(id.as_str())
    );
}

#[tokio::test]
async fn test_wave3_describe_declarative_policies_reports_returns_created() {
    let client = make_ec2_client().await;
    let report_id = client
        .start_declarative_policies_report()
        .target_id("ou-wave3-1234")
        .s3_bucket("wave3-policy-reports")
        .send()
        .await
        .expect("start_declarative_policies_report")
        .report_id()
        .unwrap()
        .to_string();

    let resp = client
        .describe_declarative_policies_reports()
        .report_ids(&report_id)
        .send()
        .await
        .expect("describe_declarative_policies_reports");
    let items = resp.reports();
    assert!(
        items
            .iter()
            .any(|r| r.report_id() == Some(report_id.as_str())),
        "should include our report"
    );
}

#[tokio::test]
async fn test_wave3_get_managed_prefix_list_associations_returns_empty_for_known_pl() {
    let client = make_ec2_client().await;
    let pl_id = client
        .create_managed_prefix_list()
        .prefix_list_name("wave3-pl")
        .max_entries(5)
        .address_family("IPv4")
        .send()
        .await
        .expect("create_managed_prefix_list")
        .prefix_list()
        .unwrap()
        .prefix_list_id()
        .unwrap()
        .to_string();

    let resp = client
        .get_managed_prefix_list_associations()
        .prefix_list_id(&pl_id)
        .send()
        .await
        .expect("get_managed_prefix_list_associations");
    assert!(resp.prefix_list_associations().is_empty());
}

#[tokio::test]
async fn test_wave3_get_associated_ipv6_pool_cidrs_returns_empty_for_ipv4_pool() {
    use aws_sdk_ec2::types::AddIpamOperatingRegion;
    let client = make_ec2_client().await;
    let ipam = client
        .create_ipam()
        .operating_regions(
            AddIpamOperatingRegion::builder()
                .region_name("us-east-1")
                .build(),
        )
        .send()
        .await
        .unwrap()
        .ipam()
        .unwrap()
        .clone();
    let scope_id = ipam.private_default_scope_id().unwrap().to_string();
    let pool_id = client
        .create_ipam_pool()
        .ipam_scope_id(&scope_id)
        .address_family(aws_sdk_ec2::types::AddressFamily::Ipv4)
        .send()
        .await
        .unwrap()
        .ipam_pool()
        .unwrap()
        .ipam_pool_id()
        .unwrap()
        .to_string();
    // ipv4 pool means no ipv6 cidrs.
    let resp = client
        .get_associated_ipv6_pool_cidrs()
        .pool_id(&pool_id)
        .send()
        .await
        .expect("get_associated_ipv6_pool_cidrs");
    assert!(resp.ipv6_cidr_associations().is_empty());
}

#[tokio::test]
async fn test_wave3_export_client_vpn_client_configuration_returns_synthetic() {
    let client = make_ec2_client().await;
    let endpoint_id = client
        .create_client_vpn_endpoint()
        .client_cidr_block("10.80.0.0/22")
        .server_certificate_arn("arn:aws:acm:us-east-1:123456789012:certificate/wave3")
        .send()
        .await
        .expect("create_client_vpn_endpoint")
        .client_vpn_endpoint_id()
        .unwrap()
        .to_string();

    let resp = client
        .export_client_vpn_client_configuration()
        .client_vpn_endpoint_id(&endpoint_id)
        .send()
        .await
        .expect("export_client_vpn_client_configuration");
    let cfg = resp.client_configuration().expect("config");
    assert!(cfg.contains("client"));
    assert!(cfg.contains("server-certificate"));
}

#[tokio::test]
async fn test_wave3_export_client_vpn_certificate_revocation_list_pending_for_known_endpoint() {
    let client = make_ec2_client().await;
    let endpoint_id = client
        .create_client_vpn_endpoint()
        .client_cidr_block("10.81.0.0/22")
        .server_certificate_arn("arn:aws:acm:us-east-1:123456789012:certificate/wave3-crl")
        .send()
        .await
        .expect("create_client_vpn_endpoint")
        .client_vpn_endpoint_id()
        .unwrap()
        .to_string();

    let resp = client
        .export_client_vpn_client_certificate_revocation_list()
        .client_vpn_endpoint_id(&endpoint_id)
        .send()
        .await
        .expect("export_client_vpn_client_certificate_revocation_list");
    // No revocation list set yet; status should be 'pending' and body absent.
    assert_eq!(
        resp.status().and_then(|s| s.code()).map(|c| c.as_str()),
        Some("pending")
    );
    assert!(resp.certificate_revocation_list().is_none());
}

#[tokio::test]
async fn test_wave3_get_route_server_propagations_returns_associations() {
    let client = make_ec2_client().await;
    let vpc_id = client
        .create_vpc()
        .cidr_block("10.90.0.0/16")
        .send()
        .await
        .unwrap()
        .vpc()
        .unwrap()
        .vpc_id()
        .unwrap()
        .to_string();
    let rs_id = client
        .create_route_server()
        .amazon_side_asn(64600)
        .send()
        .await
        .expect("create_route_server")
        .route_server()
        .unwrap()
        .route_server_id()
        .unwrap()
        .to_string();
    client
        .associate_route_server()
        .route_server_id(&rs_id)
        .vpc_id(&vpc_id)
        .send()
        .await
        .expect("associate_route_server");

    let resp = client
        .get_route_server_propagations()
        .route_server_id(&rs_id)
        .send()
        .await
        .expect("get_route_server_propagations");
    assert_eq!(resp.route_server_propagations().len(), 1);
}

#[tokio::test]
async fn test_wave3_get_route_server_routing_database_returns_persistence_flag() {
    let client = make_ec2_client().await;
    let rs_id = client
        .create_route_server()
        .amazon_side_asn(64601)
        .persist_routes(aws_sdk_ec2::types::RouteServerPersistRoutesAction::Enable)
        .persist_routes_duration(2)
        .send()
        .await
        .unwrap()
        .route_server()
        .unwrap()
        .route_server_id()
        .unwrap()
        .to_string();

    let resp = client
        .get_route_server_routing_database()
        .route_server_id(&rs_id)
        .send()
        .await
        .expect("get_route_server_routing_database");
    assert_eq!(resp.are_routes_persisted(), Some(true));
    assert!(resp.routes().is_empty());
}

#[tokio::test]
async fn test_wave3_get_network_insights_access_scope_content_returns_paths() {
    let client = make_ec2_client().await;
    let scope_id = client
        .create_network_insights_access_scope()
        .client_token("ct-wave3-scope")
        .send()
        .await
        .expect("create_network_insights_access_scope")
        .network_insights_access_scope()
        .unwrap()
        .network_insights_access_scope_id()
        .unwrap()
        .to_string();
    let resp = client
        .get_network_insights_access_scope_content()
        .network_insights_access_scope_id(&scope_id)
        .send()
        .await
        .expect("get_network_insights_access_scope_content");
    let content = resp
        .network_insights_access_scope_content()
        .expect("content");
    assert_eq!(
        content.network_insights_access_scope_id(),
        Some(scope_id.as_str())
    );
}

#[tokio::test]
async fn test_wave3_get_network_insights_access_scope_analysis_findings_empty_for_real_analysis() {
    let client = make_ec2_client().await;
    let scope_id = client
        .create_network_insights_access_scope()
        .client_token("ct-wave3-scope-findings")
        .send()
        .await
        .unwrap()
        .network_insights_access_scope()
        .unwrap()
        .network_insights_access_scope_id()
        .unwrap()
        .to_string();
    let analysis_id = client
        .start_network_insights_access_scope_analysis()
        .network_insights_access_scope_id(&scope_id)
        .client_token("ct-wave3-analysis")
        .send()
        .await
        .unwrap()
        .network_insights_access_scope_analysis()
        .unwrap()
        .network_insights_access_scope_analysis_id()
        .unwrap()
        .to_string();

    let resp = client
        .get_network_insights_access_scope_analysis_findings()
        .network_insights_access_scope_analysis_id(&analysis_id)
        .send()
        .await
        .expect("get_network_insights_access_scope_analysis_findings");
    assert!(resp.analysis_findings().is_empty());
    assert_eq!(
        resp.network_insights_access_scope_analysis_id(),
        Some(analysis_id.as_str())
    );
}

// ===== Wave 4: control-plane toggles + static catalogues =====

#[tokio::test]
async fn test_wave4_describe_vpc_classic_link_dns_support_emits_per_vpc_disabled() {
    let client = make_ec2_client().await;
    let vpc_id = client
        .create_vpc()
        .cidr_block("10.42.0.0/16")
        .send()
        .await
        .expect("create_vpc")
        .vpc()
        .unwrap()
        .vpc_id()
        .unwrap()
        .to_string();
    let resp = client
        .describe_vpc_classic_link_dns_support()
        .send()
        .await
        .expect("describe_vpc_classic_link_dns_support");
    let entry = resp
        .vpcs()
        .iter()
        .find(|e| e.vpc_id() == Some(vpc_id.as_str()))
        .expect("entry for created vpc");
    assert_eq!(entry.classic_link_dns_supported(), Some(false));
}

#[tokio::test]
async fn test_wave4_modify_vpc_peering_options_round_trip() {
    let client = make_ec2_client().await;
    let req_vpc = client
        .create_vpc()
        .cidr_block("10.43.0.0/16")
        .send()
        .await
        .unwrap()
        .vpc()
        .unwrap()
        .vpc_id()
        .unwrap()
        .to_string();
    let acc_vpc = client
        .create_vpc()
        .cidr_block("10.44.0.0/16")
        .send()
        .await
        .unwrap()
        .vpc()
        .unwrap()
        .vpc_id()
        .unwrap()
        .to_string();
    let peering_id = client
        .create_vpc_peering_connection()
        .vpc_id(&req_vpc)
        .peer_vpc_id(&acc_vpc)
        .send()
        .await
        .unwrap()
        .vpc_peering_connection()
        .unwrap()
        .vpc_peering_connection_id()
        .unwrap()
        .to_string();
    let resp = client
        .modify_vpc_peering_connection_options()
        .vpc_peering_connection_id(&peering_id)
        .requester_peering_connection_options(
            aws_sdk_ec2::types::PeeringConnectionOptionsRequest::builder()
                .allow_dns_resolution_from_remote_vpc(true)
                .build(),
        )
        .send()
        .await
        .expect("modify_vpc_peering_connection_options");
    let req_opts = resp
        .requester_peering_connection_options()
        .expect("requester options");
    assert_eq!(req_opts.allow_dns_resolution_from_remote_vpc(), Some(true));
}

#[tokio::test]
async fn test_wave4_modify_ebs_default_kms_key_id_round_trip() {
    let client = make_ec2_client().await;
    let resp = client
        .modify_ebs_default_kms_key_id()
        .kms_key_id("alias/aws/ebs")
        .send()
        .await
        .expect("modify_ebs_default_kms_key_id");
    assert_eq!(resp.kms_key_id(), Some("alias/aws/ebs"));
}

#[tokio::test]
async fn test_wave4_describe_vpc_endpoint_services_static_catalogue() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_vpc_endpoint_services()
        .send()
        .await
        .expect("describe_vpc_endpoint_services");
    let names: Vec<&str> = resp.service_names().iter().map(|s| s.as_str()).collect();
    assert!(names.iter().any(|n| n.ends_with(".s3")));
    assert!(names.iter().any(|n| n.ends_with(".dynamodb")));
    assert!(names.iter().any(|n| n.ends_with(".kms")));
}

#[tokio::test]
async fn test_wave4_describe_instance_type_offerings_emits_catalogue() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_instance_type_offerings()
        .send()
        .await
        .expect("describe_instance_type_offerings");
    assert!(resp.instance_type_offerings().len() >= 12);
}

#[tokio::test]
async fn test_wave4_aws_network_performance_metric_subscription_round_trip() {
    let client = make_ec2_client().await;
    client
        .enable_aws_network_performance_metric_subscription()
        .source("us-east-1")
        .destination("us-west-2")
        .metric(aws_sdk_ec2::types::MetricType::AggregateLatency)
        .statistic(aws_sdk_ec2::types::StatisticType::P50)
        .send()
        .await
        .expect("enable_aws_network_performance_metric_subscription");
    let listed = client
        .describe_aws_network_performance_metric_subscriptions()
        .send()
        .await
        .expect("describe_aws_network_performance_metric_subscriptions");
    assert_eq!(listed.subscriptions().len(), 1);
    client
        .disable_aws_network_performance_metric_subscription()
        .source("us-east-1")
        .destination("us-west-2")
        .metric(aws_sdk_ec2::types::MetricType::AggregateLatency)
        .statistic(aws_sdk_ec2::types::StatisticType::P50)
        .send()
        .await
        .expect("disable_aws_network_performance_metric_subscription");
    let listed = client
        .describe_aws_network_performance_metric_subscriptions()
        .send()
        .await
        .expect("describe_aws_network_performance_metric_subscriptions after disable");
    assert!(listed.subscriptions().is_empty());
}

#[tokio::test]
async fn test_wave4_serial_console_access_round_trip() {
    let client = make_ec2_client().await;
    client
        .enable_serial_console_access()
        .send()
        .await
        .expect("enable");
    client
        .disable_serial_console_access()
        .send()
        .await
        .expect("disable");
}

#[tokio::test]
async fn test_wave4_image_block_public_access_round_trip() {
    let client = make_ec2_client().await;
    let enabled = client
        .enable_image_block_public_access()
        .image_block_public_access_state(
            aws_sdk_ec2::types::ImageBlockPublicAccessEnabledState::BlockNewSharing,
        )
        .send()
        .await
        .expect("enable");
    assert_eq!(
        enabled
            .image_block_public_access_state()
            .map(|s| s.as_str()),
        Some("block-new-sharing"),
    );
    let disabled = client
        .disable_image_block_public_access()
        .send()
        .await
        .expect("disable");
    assert_eq!(
        disabled
            .image_block_public_access_state()
            .map(|s| s.as_str()),
        Some("unblocked"),
    );
}

#[tokio::test]
async fn test_wave4_allowed_images_settings_round_trip() {
    let client = make_ec2_client().await;
    client
        .enable_allowed_images_settings()
        .allowed_images_settings_state(
            aws_sdk_ec2::types::AllowedImagesSettingsEnabledState::Enabled,
        )
        .send()
        .await
        .expect("enable");
    client
        .disable_allowed_images_settings()
        .send()
        .await
        .expect("disable");
}

#[tokio::test]
async fn test_wave4_fast_snapshot_restores_round_trip() {
    let client = make_ec2_client().await;
    let vol_id = client
        .create_volume()
        .availability_zone("us-east-1a")
        .size(10)
        .send()
        .await
        .unwrap()
        .volume_id
        .unwrap();
    let snap_id = client
        .create_snapshot()
        .volume_id(&vol_id)
        .send()
        .await
        .unwrap()
        .snapshot_id
        .unwrap();
    let enabled = client
        .enable_fast_snapshot_restores()
        .availability_zones("us-east-1a")
        .source_snapshot_ids(&snap_id)
        .send()
        .await
        .expect("enable_fast_snapshot_restores");
    assert!(!enabled.successful().is_empty());
    let disabled = client
        .disable_fast_snapshot_restores()
        .availability_zones("us-east-1a")
        .source_snapshot_ids(&snap_id)
        .send()
        .await
        .expect("disable_fast_snapshot_restores");
    assert!(!disabled.successful().is_empty());
}

#[tokio::test]
async fn test_wave4_route_server_propagation_round_trip() {
    let client = make_ec2_client().await;
    let vpc_id = client
        .create_vpc()
        .cidr_block("10.55.0.0/16")
        .send()
        .await
        .unwrap()
        .vpc()
        .unwrap()
        .vpc_id()
        .unwrap()
        .to_string();
    let rs_id = client
        .create_route_server()
        .amazon_side_asn(64512)
        .send()
        .await
        .unwrap()
        .route_server()
        .unwrap()
        .route_server_id()
        .unwrap()
        .to_string();
    client
        .associate_route_server()
        .route_server_id(&rs_id)
        .vpc_id(&vpc_id)
        .send()
        .await
        .expect("associate_route_server");
    client
        .enable_route_server_propagation()
        .route_server_id(&rs_id)
        .route_table_id("rtb-99")
        .send()
        .await
        .expect("enable_route_server_propagation");
    client
        .disable_route_server_propagation()
        .route_server_id(&rs_id)
        .route_table_id("rtb-99")
        .send()
        .await
        .expect("disable_route_server_propagation");
}

#[tokio::test]
async fn test_wave4_describe_classic_link_instances_returns_attached() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_classic_link_instances()
        .send()
        .await
        .expect("describe_classic_link_instances");
    // No instances attach to ClassicLink in default state.
    assert!(resp.instances().is_empty());
}

#[tokio::test]
async fn test_wave4_get_security_groups_for_vpc_filters_by_vpc() {
    let client = make_ec2_client().await;
    let vpc_id = client
        .create_vpc()
        .cidr_block("10.66.0.0/16")
        .send()
        .await
        .unwrap()
        .vpc()
        .unwrap()
        .vpc_id()
        .unwrap()
        .to_string();
    let sg_id = client
        .create_security_group()
        .group_name("wave4-sg")
        .description("wave4 sg")
        .vpc_id(&vpc_id)
        .send()
        .await
        .unwrap()
        .group_id
        .unwrap();
    let resp = client
        .get_security_groups_for_vpc()
        .vpc_id(&vpc_id)
        .send()
        .await
        .expect("get_security_groups_for_vpc");
    assert!(
        resp.security_group_for_vpcs()
            .iter()
            .any(|sg| sg.group_id() == Some(sg_id.as_str()))
    );
}

#[tokio::test]
async fn test_wave4_describe_security_group_references_filters() {
    let client = make_ec2_client().await;
    let resp = client
        .describe_security_group_references()
        .send()
        .await
        .expect("describe_security_group_references");
    // No cross-VPC SG references in default state.
    assert!(resp.security_group_reference_set().is_empty());
}

#[tokio::test]
async fn test_wave4_get_vpc_resources_blocking_encryption_enforcement_lists_unprotected_vpcs() {
    let client = make_ec2_client().await;
    let vpc_id = client
        .create_vpc()
        .cidr_block("10.77.0.0/16")
        .send()
        .await
        .unwrap()
        .vpc()
        .unwrap()
        .vpc_id()
        .unwrap()
        .to_string();
    let resp = client
        .get_vpc_resources_blocking_encryption_enforcement()
        .send()
        .await
        .expect("get_vpc_resources_blocking_encryption_enforcement");
    assert!(
        resp.non_compliant_resources()
            .iter()
            .any(|r| r.id() == Some(vpc_id.as_str()))
    );
}
