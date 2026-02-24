//! Smoke tests for winterbaume EC2 service — realistic application scenarios.
//!
//! Each test simulates a coherent end-to-end workflow rather than exercising
//! a single API call in isolation. The goal is to prove that the networking,
//! compute and storage handlers compose into the topologies a real operator
//! would build, not just that each handler works in isolation.

use aws_sdk_ec2::config::BehaviorVersion;
use aws_sdk_ec2::types::{
    Filter, GatewayType, InstanceType, IpPermission, IpRange, NewDhcpConfiguration, Placement,
    RequestLaunchTemplateData, ResourceType, Tag, TagSpecification, UserIdGroupPair, VolumeType,
};
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

/// Scenario: three-tier VPC bootstrap.
///
/// An operator stands up a fresh VPC for a classic web/app/db topology:
/// a public subnet that egresses through an internet gateway, plus three
/// security-group tiers that reference each other by group-id ( web allows
/// the world on 80/443, app accepts traffic only from web, db accepts
/// Postgres only from app ). Verifies the default route lands in the
/// public route table and that the inter-tier rules are stored as
/// `UserIdGroupPair` references rather than CIDR ranges.
#[tokio::test]
async fn test_three_tier_vpc_bootstrap() {
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

    let igw_id = client
        .create_internet_gateway()
        .send()
        .await
        .expect("create_internet_gateway")
        .internet_gateway()
        .and_then(|g| g.internet_gateway_id())
        .expect("igw_id")
        .to_string();

    client
        .attach_internet_gateway()
        .internet_gateway_id(&igw_id)
        .vpc_id(&vpc_id)
        .send()
        .await
        .expect("attach_internet_gateway");

    let public_subnet = client
        .create_subnet()
        .vpc_id(&vpc_id)
        .cidr_block("10.0.1.0/24")
        .availability_zone("us-east-1a")
        .send()
        .await
        .expect("create public subnet")
        .subnet()
        .and_then(|s| s.subnet_id())
        .expect("public subnet id")
        .to_string();

    let private_subnet = client
        .create_subnet()
        .vpc_id(&vpc_id)
        .cidr_block("10.0.2.0/24")
        .availability_zone("us-east-1b")
        .send()
        .await
        .expect("create private subnet")
        .subnet()
        .and_then(|s| s.subnet_id())
        .expect("private subnet id")
        .to_string();

    let public_rtb = client
        .create_route_table()
        .vpc_id(&vpc_id)
        .send()
        .await
        .expect("create public route table")
        .route_table()
        .and_then(|r| r.route_table_id())
        .expect("public rtb id")
        .to_string();

    client
        .create_route()
        .route_table_id(&public_rtb)
        .destination_cidr_block("0.0.0.0/0")
        .gateway_id(&igw_id)
        .send()
        .await
        .expect("create default IGW route");

    client
        .associate_route_table()
        .route_table_id(&public_rtb)
        .subnet_id(&public_subnet)
        .send()
        .await
        .expect("associate public rtb");

    let web_sg = client
        .create_security_group()
        .group_name("web-tier")
        .description("Public web tier")
        .vpc_id(&vpc_id)
        .send()
        .await
        .expect("create web SG")
        .group_id()
        .expect("web sg id")
        .to_string();

    let app_sg = client
        .create_security_group()
        .group_name("app-tier")
        .description("Internal application tier")
        .vpc_id(&vpc_id)
        .send()
        .await
        .expect("create app SG")
        .group_id()
        .expect("app sg id")
        .to_string();

    let db_sg = client
        .create_security_group()
        .group_name("db-tier")
        .description("Postgres database tier")
        .vpc_id(&vpc_id)
        .send()
        .await
        .expect("create db SG")
        .group_id()
        .expect("db sg id")
        .to_string();

    // Web tier: open 80 and 443 to the world.
    client
        .authorize_security_group_ingress()
        .group_id(&web_sg)
        .ip_permissions(
            IpPermission::builder()
                .ip_protocol("tcp")
                .from_port(80)
                .to_port(80)
                .ip_ranges(IpRange::builder().cidr_ip("0.0.0.0/0").build())
                .build(),
        )
        .ip_permissions(
            IpPermission::builder()
                .ip_protocol("tcp")
                .from_port(443)
                .to_port(443)
                .ip_ranges(IpRange::builder().cidr_ip("0.0.0.0/0").build())
                .build(),
        )
        .send()
        .await
        .expect("authorize web ingress");

    // App tier accepts 8080 only from the web tier security group.
    client
        .authorize_security_group_ingress()
        .group_id(&app_sg)
        .ip_permissions(
            IpPermission::builder()
                .ip_protocol("tcp")
                .from_port(8080)
                .to_port(8080)
                .user_id_group_pairs(UserIdGroupPair::builder().group_id(&web_sg).build())
                .build(),
        )
        .send()
        .await
        .expect("authorize app ingress from web");

    // DB tier accepts Postgres only from the app tier security group.
    client
        .authorize_security_group_ingress()
        .group_id(&db_sg)
        .ip_permissions(
            IpPermission::builder()
                .ip_protocol("tcp")
                .from_port(5432)
                .to_port(5432)
                .user_id_group_pairs(UserIdGroupPair::builder().group_id(&app_sg).build())
                .build(),
        )
        .send()
        .await
        .expect("authorize db ingress from app");

    // Public route table should carry an explicit 0.0.0.0/0 → IGW route and
    // be associated with the public subnet.
    let rtbs = client
        .describe_route_tables()
        .route_table_ids(&public_rtb)
        .send()
        .await
        .expect("describe public rtb");
    let public_rt = rtbs
        .route_tables()
        .first()
        .expect("public route table present");
    assert!(
        public_rt
            .routes()
            .iter()
            .any(|r| r.destination_cidr_block() == Some("0.0.0.0/0")
                && r.gateway_id() == Some(igw_id.as_str())),
        "public route table should default-route through {igw_id}"
    );
    assert!(
        public_rt
            .associations()
            .iter()
            .any(|a| a.subnet_id() == Some(public_subnet.as_str())),
        "public route table should be associated with the public subnet"
    );

    // App tier rule should reference the web SG by group-id, not CIDR.
    let app = client
        .describe_security_groups()
        .group_ids(&app_sg)
        .send()
        .await
        .expect("describe app SG");
    let app_ingress = app.security_groups()[0].ip_permissions();
    let app_rule = app_ingress
        .iter()
        .find(|p| p.from_port() == Some(8080))
        .expect("app SG should have 8080 rule");
    assert!(
        app_rule.ip_ranges().is_empty(),
        "app rule should not use CIDR"
    );
    assert!(
        app_rule
            .user_id_group_pairs()
            .iter()
            .any(|g| g.group_id() == Some(web_sg.as_str())),
        "app SG rule should reference web SG group-id"
    );

    // DB tier rule should reference the app SG by group-id.
    let db = client
        .describe_security_groups()
        .group_ids(&db_sg)
        .send()
        .await
        .expect("describe db SG");
    let db_ingress = db.security_groups()[0].ip_permissions();
    let db_rule = db_ingress
        .iter()
        .find(|p| p.from_port() == Some(5432))
        .expect("db SG should have 5432 rule");
    assert!(
        db_rule
            .user_id_group_pairs()
            .iter()
            .any(|g| g.group_id() == Some(app_sg.as_str())),
        "db SG rule should reference app SG group-id"
    );

    // Avoid leaking — keep the test self-contained.
    let _ = client.delete_security_group().group_id(&db_sg).send().await;
    let _ = client
        .delete_security_group()
        .group_id(&app_sg)
        .send()
        .await;
    let _ = client
        .delete_security_group()
        .group_id(&web_sg)
        .send()
        .await;
    let _ = client
        .delete_subnet()
        .subnet_id(&private_subnet)
        .send()
        .await;
    let _ = client
        .delete_subnet()
        .subnet_id(&public_subnet)
        .send()
        .await;
    let _ = client
        .delete_route_table()
        .route_table_id(&public_rtb)
        .send()
        .await;
    let _ = client
        .detach_internet_gateway()
        .internet_gateway_id(&igw_id)
        .vpc_id(&vpc_id)
        .send()
        .await;
    let _ = client
        .delete_internet_gateway()
        .internet_gateway_id(&igw_id)
        .send()
        .await;
    let _ = client.delete_vpc().vpc_id(&vpc_id).send().await;
}

/// Scenario: web fleet launch with EIP, EBS and golden-AMI snapshot.
///
/// A release engineer launches a three-instance web fleet into a freshly
/// minted VPC, attaches an Elastic IP to the canary node, mounts an extra
/// EBS volume, snapshots it, then bakes a golden AMI from the canary
/// instance for the next deploy. Verifies that all three instances land
/// in the right subnet/SG/AZ, that the EIP and volume are bound to the
/// canary, and that the AMI is registered as `available`.
#[tokio::test]
async fn test_web_fleet_launch_and_golden_ami_pipeline() {
    let client = make_ec2_client().await;

    let vpc_id = client
        .create_vpc()
        .cidr_block("10.20.0.0/16")
        .send()
        .await
        .expect("create_vpc")
        .vpc()
        .and_then(|v| v.vpc_id())
        .expect("vpc_id")
        .to_string();

    let subnet_id = client
        .create_subnet()
        .vpc_id(&vpc_id)
        .cidr_block("10.20.1.0/24")
        .availability_zone("us-east-1a")
        .send()
        .await
        .expect("create subnet")
        .subnet()
        .and_then(|s| s.subnet_id())
        .expect("subnet_id")
        .to_string();

    let web_sg = client
        .create_security_group()
        .group_name("web-fleet-sg")
        .description("Public-facing web fleet")
        .vpc_id(&vpc_id)
        .send()
        .await
        .expect("create web SG")
        .group_id()
        .expect("web sg id")
        .to_string();
    client
        .authorize_security_group_ingress()
        .group_id(&web_sg)
        .ip_permissions(
            IpPermission::builder()
                .ip_protocol("tcp")
                .from_port(80)
                .to_port(80)
                .ip_ranges(IpRange::builder().cidr_ip("0.0.0.0/0").build())
                .build(),
        )
        .send()
        .await
        .expect("authorize 80/tcp");

    let alloc_id = client
        .allocate_address()
        .send()
        .await
        .expect("allocate EIP")
        .allocation_id()
        .expect("allocation_id")
        .to_string();

    let run = client
        .run_instances()
        .image_id("ami-web-base")
        .instance_type(InstanceType::T3Micro)
        .min_count(3)
        .max_count(3)
        .subnet_id(&subnet_id)
        .security_group_ids(&web_sg)
        .placement(Placement::builder().availability_zone("us-east-1a").build())
        .tag_specifications(
            TagSpecification::builder()
                .resource_type(ResourceType::Instance)
                .tags(Tag::builder().key("Name").value("web-fleet").build())
                .tags(Tag::builder().key("Role").value("web").build())
                .build(),
        )
        .send()
        .await
        .expect("run_instances");
    let instance_ids: Vec<String> = run
        .instances()
        .iter()
        .map(|i| i.instance_id().expect("instance_id").to_string())
        .collect();
    assert_eq!(instance_ids.len(), 3, "should launch exactly 3 instances");
    let canary = instance_ids[0].clone();

    // Bind the EIP to the canary instance.
    client
        .associate_address()
        .allocation_id(&alloc_id)
        .instance_id(&canary)
        .send()
        .await
        .expect("associate EIP to canary");

    // Add a 50 GiB gp3 data volume on the canary's AZ and attach it.
    let volume_id = client
        .create_volume()
        .availability_zone("us-east-1a")
        .size(50)
        .volume_type(VolumeType::Gp3)
        .send()
        .await
        .expect("create_volume")
        .volume_id()
        .expect("volume_id")
        .to_string();
    client
        .attach_volume()
        .volume_id(&volume_id)
        .instance_id(&canary)
        .device("/dev/xvdf")
        .send()
        .await
        .expect("attach_volume");

    // Snapshot the data volume and bake a golden AMI from the canary.
    let snapshot_id = client
        .create_snapshot()
        .volume_id(&volume_id)
        .description("golden snapshot")
        .send()
        .await
        .expect("create_snapshot")
        .snapshot_id()
        .expect("snapshot_id")
        .to_string();

    let image_id = client
        .create_image()
        .instance_id(&canary)
        .name("web-golden")
        .description("Bootable image for the next deploy")
        .send()
        .await
        .expect("create_image")
        .image_id()
        .expect("image_id")
        .to_string();

    // All three instances should be reachable via the Name tag and live in
    // the right subnet, AZ and security group.
    let by_tag = client
        .describe_instances()
        .filters(
            Filter::builder()
                .name("tag:Name")
                .values("web-fleet")
                .build(),
        )
        .send()
        .await
        .expect("describe by tag");
    let mut found = 0usize;
    for res in by_tag.reservations() {
        for inst in res.instances() {
            found += 1;
            assert_eq!(inst.subnet_id(), Some(subnet_id.as_str()));
            assert_eq!(
                inst.placement().and_then(|p| p.availability_zone()),
                Some("us-east-1a")
            );
            assert!(
                inst.security_groups()
                    .iter()
                    .any(|g| g.group_id() == Some(web_sg.as_str())),
                "instance {} should be in web SG",
                inst.instance_id().unwrap_or("?")
            );
            assert_eq!(
                inst.state().and_then(|s| s.name()).map(|n| n.as_str()),
                Some("running")
            );
        }
    }
    assert_eq!(found, 3, "tag filter should match all 3 fleet instances");

    // Snapshot should record the source volume.
    let snaps = client
        .describe_snapshots()
        .snapshot_ids(&snapshot_id)
        .send()
        .await
        .expect("describe_snapshots");
    assert_eq!(
        snaps.snapshots()[0].volume_id(),
        Some(volume_id.as_str()),
        "snapshot should reference the source volume"
    );

    // Golden AMI should be available for the next deploy.
    let images = client
        .describe_images()
        .image_ids(&image_id)
        .send()
        .await
        .expect("describe_images");
    assert_eq!(images.images().len(), 1);
    assert_eq!(
        images.images()[0].state().map(|s| s.as_str()).unwrap_or(""),
        "available",
        "golden AMI should be available"
    );

    // Tear the fleet down.
    client
        .terminate_instances()
        .set_instance_ids(Some(instance_ids))
        .send()
        .await
        .expect("terminate fleet");
    let _ = client.detach_volume().volume_id(&volume_id).send().await;
    let _ = client.delete_volume().volume_id(&volume_id).send().await;
    let _ = client
        .release_address()
        .allocation_id(&alloc_id)
        .send()
        .await;
    let _ = client.deregister_image().image_id(&image_id).send().await;
    let _ = client
        .delete_security_group()
        .group_id(&web_sg)
        .send()
        .await;
    let _ = client.delete_subnet().subnet_id(&subnet_id).send().await;
    let _ = client.delete_vpc().vpc_id(&vpc_id).send().await;
}

/// Scenario: blue/green deploy via launch-template versioning.
///
/// The release pipeline maintains a single launch template whose default
/// version pins the currently shipping image. A new green build creates
/// a new launch-template version, the pipeline flips the default version
/// to green, then launches the green fleet from the new image. The blue
/// fleet keeps running until cutover. Verifies that the launch template
/// has both versions, that the default has moved to v2, and that the
/// fleets are isolated by image_id.
#[tokio::test]
async fn test_blue_green_with_launch_template_versions() {
    let client = make_ec2_client().await;

    let lt_id = client
        .create_launch_template()
        .launch_template_name("web-deploys")
        .version_description("blue baseline")
        .launch_template_data(
            RequestLaunchTemplateData::builder()
                .image_id("ami-blue")
                .instance_type(InstanceType::T3Micro)
                .build(),
        )
        .send()
        .await
        .expect("create_launch_template")
        .launch_template()
        .and_then(|l| l.launch_template_id())
        .expect("lt id")
        .to_string();

    // Blue fleet: 2 instances tagged Color=blue.
    let blue: Vec<String> = client
        .run_instances()
        .image_id("ami-blue")
        .instance_type(InstanceType::T3Micro)
        .min_count(2)
        .max_count(2)
        .tag_specifications(
            TagSpecification::builder()
                .resource_type(ResourceType::Instance)
                .tags(Tag::builder().key("Color").value("blue").build())
                .build(),
        )
        .send()
        .await
        .expect("run blue fleet")
        .instances()
        .iter()
        .map(|i| i.instance_id().unwrap().to_string())
        .collect();
    assert_eq!(blue.len(), 2);

    // Cut a v2 with the green image.
    let v2 = client
        .create_launch_template_version()
        .launch_template_id(&lt_id)
        .version_description("green build")
        .launch_template_data(
            RequestLaunchTemplateData::builder()
                .image_id("ami-green")
                .instance_type(InstanceType::T3Micro)
                .build(),
        )
        .send()
        .await
        .expect("create v2")
        .launch_template_version()
        .and_then(|v| v.version_number())
        .expect("v2 number");
    assert_eq!(v2, 2, "second version should be numbered 2");

    // Pipeline flips the default version pointer to v2.
    client
        .modify_launch_template()
        .launch_template_id(&lt_id)
        .default_version("2")
        .send()
        .await
        .expect("modify default version");

    let versions = client
        .describe_launch_template_versions()
        .launch_template_id(&lt_id)
        .send()
        .await
        .expect("describe versions");
    let nums: Vec<i64> = versions
        .launch_template_versions()
        .iter()
        .filter_map(|v| v.version_number())
        .collect();
    assert!(
        nums.contains(&1) && nums.contains(&2),
        "should see v1 and v2"
    );
    let default_version = versions
        .launch_template_versions()
        .iter()
        .find(|v| v.default_version() == Some(true))
        .and_then(|v| v.version_number())
        .expect("a default version should be marked");
    assert_eq!(default_version, 2, "default version should now be v2");

    // Green fleet: 2 instances tagged Color=green using the new image.
    let green: Vec<String> = client
        .run_instances()
        .image_id("ami-green")
        .instance_type(InstanceType::T3Micro)
        .min_count(2)
        .max_count(2)
        .tag_specifications(
            TagSpecification::builder()
                .resource_type(ResourceType::Instance)
                .tags(Tag::builder().key("Color").value("green").build())
                .build(),
        )
        .send()
        .await
        .expect("run green fleet")
        .instances()
        .iter()
        .map(|i| i.instance_id().unwrap().to_string())
        .collect();

    // Both fleets should coexist with the right images.
    let blue_seen = client
        .describe_instances()
        .filters(Filter::builder().name("tag:Color").values("blue").build())
        .send()
        .await
        .expect("describe blue");
    for res in blue_seen.reservations() {
        for inst in res.instances() {
            assert_eq!(
                inst.image_id(),
                Some("ami-blue"),
                "blue fleet should keep ami-blue"
            );
        }
    }

    let green_seen = client
        .describe_instances()
        .filters(Filter::builder().name("tag:Color").values("green").build())
        .send()
        .await
        .expect("describe green");
    let mut green_count = 0usize;
    for res in green_seen.reservations() {
        for inst in res.instances() {
            green_count += 1;
            assert_eq!(
                inst.image_id(),
                Some("ami-green"),
                "green fleet should run ami-green"
            );
        }
    }
    assert_eq!(green_count, 2, "should see exactly 2 green instances");

    // Cut over: terminate blue, leave green running.
    client
        .terminate_instances()
        .set_instance_ids(Some(blue))
        .send()
        .await
        .expect("terminate blue");
    let _ = client
        .terminate_instances()
        .set_instance_ids(Some(green))
        .send()
        .await;
    let _ = client
        .delete_launch_template()
        .launch_template_id(&lt_id)
        .send()
        .await;
}

/// Scenario: private-subnet egress through a NAT gateway.
///
/// The VPC has one public and one private subnet. The public subnet
/// default-routes to an internet gateway. A NAT gateway lives in the
/// public subnet and the private subnet's route table default-routes
/// to it. Verifies both route tables resolve to the correct next-hop
/// gateway and are associated with the right subnet — the canonical
/// invariant for "private workloads can phone home, the internet
/// cannot reach them directly."
#[tokio::test]
async fn test_private_subnet_nat_egress_path() {
    let client = make_ec2_client().await;

    let vpc_id = client
        .create_vpc()
        .cidr_block("10.30.0.0/16")
        .send()
        .await
        .expect("create_vpc")
        .vpc()
        .and_then(|v| v.vpc_id())
        .expect("vpc_id")
        .to_string();

    let public_subnet = client
        .create_subnet()
        .vpc_id(&vpc_id)
        .cidr_block("10.30.1.0/24")
        .availability_zone("us-east-1a")
        .send()
        .await
        .expect("create public subnet")
        .subnet()
        .and_then(|s| s.subnet_id())
        .expect("public subnet id")
        .to_string();

    let private_subnet = client
        .create_subnet()
        .vpc_id(&vpc_id)
        .cidr_block("10.30.2.0/24")
        .availability_zone("us-east-1a")
        .send()
        .await
        .expect("create private subnet")
        .subnet()
        .and_then(|s| s.subnet_id())
        .expect("private subnet id")
        .to_string();

    let igw_id = client
        .create_internet_gateway()
        .send()
        .await
        .expect("create_internet_gateway")
        .internet_gateway()
        .and_then(|g| g.internet_gateway_id())
        .expect("igw id")
        .to_string();
    client
        .attach_internet_gateway()
        .internet_gateway_id(&igw_id)
        .vpc_id(&vpc_id)
        .send()
        .await
        .expect("attach IGW");

    let alloc_id = client
        .allocate_address()
        .send()
        .await
        .expect("allocate EIP for NAT")
        .allocation_id()
        .expect("allocation_id")
        .to_string();

    let nat_id = client
        .create_nat_gateway()
        .subnet_id(&public_subnet)
        .allocation_id(&alloc_id)
        .send()
        .await
        .expect("create_nat_gateway")
        .nat_gateway()
        .and_then(|n| n.nat_gateway_id().map(str::to_owned))
        .expect("nat id");

    // Public subnet's route table → IGW.
    let public_rtb = client
        .create_route_table()
        .vpc_id(&vpc_id)
        .send()
        .await
        .expect("create public rtb")
        .route_table()
        .and_then(|r| r.route_table_id())
        .expect("public rtb id")
        .to_string();
    client
        .create_route()
        .route_table_id(&public_rtb)
        .destination_cidr_block("0.0.0.0/0")
        .gateway_id(&igw_id)
        .send()
        .await
        .expect("create public default route");
    client
        .associate_route_table()
        .route_table_id(&public_rtb)
        .subnet_id(&public_subnet)
        .send()
        .await
        .expect("associate public rtb");

    // Private subnet's route table → NAT.
    let private_rtb = client
        .create_route_table()
        .vpc_id(&vpc_id)
        .send()
        .await
        .expect("create private rtb")
        .route_table()
        .and_then(|r| r.route_table_id())
        .expect("private rtb id")
        .to_string();
    client
        .create_route()
        .route_table_id(&private_rtb)
        .destination_cidr_block("0.0.0.0/0")
        .nat_gateway_id(&nat_id)
        .send()
        .await
        .expect("create private default route");
    client
        .associate_route_table()
        .route_table_id(&private_rtb)
        .subnet_id(&private_subnet)
        .send()
        .await
        .expect("associate private rtb");

    let described = client
        .describe_route_tables()
        .route_table_ids(&public_rtb)
        .route_table_ids(&private_rtb)
        .send()
        .await
        .expect("describe both rtbs");

    let public_rt = described
        .route_tables()
        .iter()
        .find(|rt| rt.route_table_id() == Some(public_rtb.as_str()))
        .expect("public rtb described");
    let private_rt = described
        .route_tables()
        .iter()
        .find(|rt| rt.route_table_id() == Some(private_rtb.as_str()))
        .expect("private rtb described");

    let public_default = public_rt
        .routes()
        .iter()
        .find(|r| r.destination_cidr_block() == Some("0.0.0.0/0"))
        .expect("public default route");
    assert_eq!(
        public_default.gateway_id(),
        Some(igw_id.as_str()),
        "public default should hop to the IGW"
    );

    let private_default = private_rt
        .routes()
        .iter()
        .find(|r| r.destination_cidr_block() == Some("0.0.0.0/0"))
        .expect("private default route");
    // winterbaume's RouteView stores all next-hop ids in `gateway_id`; the
    // NAT identifier is recognisable by its `nat-` prefix even when it
    // surfaces through the gateway field.
    let private_hop = private_default
        .gateway_id()
        .or_else(|| private_default.nat_gateway_id())
        .unwrap_or("");
    assert_eq!(
        private_hop, nat_id,
        "private default should hop to the NAT gateway, got {private_hop}"
    );

    assert!(
        public_rt
            .associations()
            .iter()
            .any(|a| a.subnet_id() == Some(public_subnet.as_str())),
        "public rtb should be associated with public subnet"
    );
    assert!(
        private_rt
            .associations()
            .iter()
            .any(|a| a.subnet_id() == Some(private_subnet.as_str())),
        "private rtb should be associated with private subnet"
    );

    // Teardown.
    let _ = client
        .delete_nat_gateway()
        .nat_gateway_id(&nat_id)
        .send()
        .await;
    let _ = client
        .release_address()
        .allocation_id(&alloc_id)
        .send()
        .await;
    let _ = client
        .delete_route_table()
        .route_table_id(&private_rtb)
        .send()
        .await;
    let _ = client
        .delete_route_table()
        .route_table_id(&public_rtb)
        .send()
        .await;
    let _ = client
        .detach_internet_gateway()
        .internet_gateway_id(&igw_id)
        .vpc_id(&vpc_id)
        .send()
        .await;
    let _ = client
        .delete_internet_gateway()
        .internet_gateway_id(&igw_id)
        .send()
        .await;
    let _ = client
        .delete_subnet()
        .subnet_id(&private_subnet)
        .send()
        .await;
    let _ = client
        .delete_subnet()
        .subnet_id(&public_subnet)
        .send()
        .await;
    let _ = client.delete_vpc().vpc_id(&vpc_id).send().await;
}

/// Scenario: VPC peering handshake between two accounts' workloads.
///
/// A platform team owns a "shared services" VPC ( DNS, monitoring, package
/// mirrors ); a product team owns a workload VPC. The platform team
/// initiates a peering request from the workload side, the shared-services
/// side accepts, and both sides should now see the connection in the
/// `active` state. Verifies that `CreateVpcPeeringConnection` starts in
/// `pending-acceptance`, that `AcceptVpcPeeringConnection` flips it to
/// `active`, and that `DescribeVpcPeeringConnections` reflects the change
/// without requiring a re-create.
#[tokio::test]
async fn test_vpc_peering_handshake_between_accounts() {
    let client = make_ec2_client().await;

    let workload_vpc = client
        .create_vpc()
        .cidr_block("10.40.0.0/16")
        .send()
        .await
        .expect("create workload VPC")
        .vpc()
        .and_then(|v| v.vpc_id())
        .expect("workload vpc id")
        .to_string();

    let shared_vpc = client
        .create_vpc()
        .cidr_block("10.41.0.0/16")
        .send()
        .await
        .expect("create shared VPC")
        .vpc()
        .and_then(|v| v.vpc_id())
        .expect("shared vpc id")
        .to_string();

    let create = client
        .create_vpc_peering_connection()
        .vpc_id(&workload_vpc)
        .peer_vpc_id(&shared_vpc)
        .send()
        .await
        .expect("create peering")
        .vpc_peering_connection()
        .cloned()
        .expect("peering returned");
    let pcx_id = create
        .vpc_peering_connection_id()
        .expect("pcx id")
        .to_string();
    assert_eq!(
        create.status().and_then(|s| s.code()).map(|c| c.as_str()),
        Some("pending-acceptance"),
        "fresh peering should start as pending-acceptance"
    );

    client
        .accept_vpc_peering_connection()
        .vpc_peering_connection_id(&pcx_id)
        .send()
        .await
        .expect("accept peering");

    let listed = client
        .describe_vpc_peering_connections()
        .vpc_peering_connection_ids(&pcx_id)
        .send()
        .await
        .expect("describe peering");
    let pcx = listed
        .vpc_peering_connections()
        .iter()
        .find(|p| p.vpc_peering_connection_id() == Some(pcx_id.as_str()))
        .expect("peering present after accept");
    assert_eq!(
        pcx.status().and_then(|s| s.code()).map(|c| c.as_str()),
        Some("active"),
        "peering should be active after accept"
    );

    let _ = client
        .delete_vpc_peering_connection()
        .vpc_peering_connection_id(&pcx_id)
        .send()
        .await;
    let _ = client.delete_vpc().vpc_id(&shared_vpc).send().await;
    let _ = client.delete_vpc().vpc_id(&workload_vpc).send().await;
}

/// Scenario: cross-AZ disaster recovery via snapshot restore.
///
/// A primary instance in `us-east-1a` writes to a data volume. To survive
/// an AZ outage, an operator snapshots that volume and restores a fresh
/// volume from the snapshot in `us-east-1b`, then attaches it to a
/// stand-by instance in the recovery AZ. Verifies that the restored
/// volume keeps the source size and records `snapshot_id` lineage, and
/// that it can be attached to an instance in the recovery AZ.
#[tokio::test]
async fn test_cross_az_disaster_recovery_snapshot_restore() {
    let client = make_ec2_client().await;

    let primary = client
        .run_instances()
        .image_id("ami-app")
        .instance_type(InstanceType::T3Micro)
        .min_count(1)
        .max_count(1)
        .placement(Placement::builder().availability_zone("us-east-1a").build())
        .send()
        .await
        .expect("run primary instance")
        .instances()
        .first()
        .and_then(|i| i.instance_id())
        .expect("primary id")
        .to_string();

    let primary_vol = client
        .create_volume()
        .availability_zone("us-east-1a")
        .size(40)
        .volume_type(VolumeType::Gp3)
        .send()
        .await
        .expect("create primary volume")
        .volume_id()
        .expect("primary vol id")
        .to_string();
    client
        .attach_volume()
        .volume_id(&primary_vol)
        .instance_id(&primary)
        .device("/dev/xvdf")
        .send()
        .await
        .expect("attach primary volume");

    let snapshot_id = client
        .create_snapshot()
        .volume_id(&primary_vol)
        .description("nightly DR snapshot")
        .send()
        .await
        .expect("create snapshot")
        .snapshot_id()
        .expect("snapshot id")
        .to_string();

    let standby = client
        .run_instances()
        .image_id("ami-app")
        .instance_type(InstanceType::T3Micro)
        .min_count(1)
        .max_count(1)
        .placement(Placement::builder().availability_zone("us-east-1b").build())
        .send()
        .await
        .expect("run standby instance")
        .instances()
        .first()
        .and_then(|i| i.instance_id())
        .expect("standby id")
        .to_string();

    let restored_vol = client
        .create_volume()
        .availability_zone("us-east-1b")
        .snapshot_id(&snapshot_id)
        .volume_type(VolumeType::Gp3)
        .send()
        .await
        .expect("restore volume from snapshot")
        .volume_id()
        .expect("restored vol id")
        .to_string();

    let restored = client
        .describe_volumes()
        .volume_ids(&restored_vol)
        .send()
        .await
        .expect("describe restored volume");
    let restored = &restored.volumes()[0];
    assert_eq!(
        restored.snapshot_id(),
        Some(snapshot_id.as_str()),
        "restored volume should remember its source snapshot"
    );
    assert_eq!(
        restored.size(),
        Some(40),
        "restored volume should inherit the source size"
    );
    assert_eq!(
        restored.availability_zone(),
        Some("us-east-1b"),
        "restored volume should land in the recovery AZ"
    );

    client
        .attach_volume()
        .volume_id(&restored_vol)
        .instance_id(&standby)
        .device("/dev/xvdf")
        .send()
        .await
        .expect("attach restored volume to standby");

    let after = client
        .describe_volumes()
        .volume_ids(&restored_vol)
        .send()
        .await
        .expect("describe after attach");
    assert_eq!(
        after.volumes()[0].state().map(|s| s.as_str()),
        Some("in-use"),
        "restored volume should be in-use once attached to standby"
    );
    assert!(
        after.volumes()[0]
            .attachments()
            .iter()
            .any(|a| a.instance_id() == Some(standby.as_str())),
        "restored volume should be attached to the standby instance"
    );

    let _ = client.detach_volume().volume_id(&restored_vol).send().await;
    let _ = client.delete_volume().volume_id(&restored_vol).send().await;
    let _ = client.detach_volume().volume_id(&primary_vol).send().await;
    let _ = client.delete_volume().volume_id(&primary_vol).send().await;
    let _ = client
        .delete_snapshot()
        .snapshot_id(&snapshot_id)
        .send()
        .await;
    let _ = client
        .terminate_instances()
        .instance_ids(&standby)
        .instance_ids(&primary)
        .send()
        .await;
}

/// Scenario: custom DHCP option set for an internal domain.
///
/// A platform team runs internal DNS at `corp.local` with private
/// resolvers. They publish a custom DHCP option set carrying that domain
/// name and resolver IPs, then attach it to a workload VPC so newly
/// launched instances pick up corporate DNS automatically. Verifies that
/// the option set stores the configured key/value pairs and that
/// `AssociateDhcpOptions` rewrites the VPC's `dhcp_options_id` away from
/// the auto-generated default.
#[tokio::test]
async fn test_custom_dhcp_options_attached_to_vpc() {
    let client = make_ec2_client().await;

    let vpc_id = client
        .create_vpc()
        .cidr_block("10.50.0.0/16")
        .send()
        .await
        .expect("create_vpc")
        .vpc()
        .and_then(|v| v.vpc_id())
        .expect("vpc id")
        .to_string();

    let dopt_id = client
        .create_dhcp_options()
        .dhcp_configurations(
            NewDhcpConfiguration::builder()
                .key("domain-name")
                .values("corp.local")
                .build(),
        )
        .dhcp_configurations(
            NewDhcpConfiguration::builder()
                .key("domain-name-servers")
                .values("10.50.0.2")
                .values("10.50.0.3")
                .build(),
        )
        .send()
        .await
        .expect("create_dhcp_options")
        .dhcp_options()
        .and_then(|d| d.dhcp_options_id())
        .expect("dopt id")
        .to_string();

    let configs = client
        .describe_dhcp_options()
        .dhcp_options_ids(&dopt_id)
        .send()
        .await
        .expect("describe_dhcp_options")
        .dhcp_options()
        .first()
        .cloned()
        .expect("dopt described");
    let pairs = configs.dhcp_configurations();
    assert!(
        pairs.iter().any(|c| c.key() == Some("domain-name")
            && c.values().iter().any(|v| v.value() == Some("corp.local"))),
        "domain-name=corp.local should be stored"
    );
    assert!(
        pairs
            .iter()
            .any(|c| c.key() == Some("domain-name-servers") && c.values().len() >= 2),
        "domain-name-servers should preserve both resolvers"
    );

    client
        .associate_dhcp_options()
        .dhcp_options_id(&dopt_id)
        .vpc_id(&vpc_id)
        .send()
        .await
        .expect("associate_dhcp_options to VPC");

    let after = client
        .describe_vpcs()
        .vpc_ids(&vpc_id)
        .send()
        .await
        .expect("describe after associate");
    assert_eq!(
        after.vpcs()[0].dhcp_options_id(),
        Some(dopt_id.as_str()),
        "VPC should now reference the custom option set"
    );

    // Detach back to the AWS-default by associating "default" then drop the option set.
    let _ = client
        .associate_dhcp_options()
        .dhcp_options_id("default")
        .vpc_id(&vpc_id)
        .send()
        .await;
    let _ = client
        .delete_dhcp_options()
        .dhcp_options_id(&dopt_id)
        .send()
        .await;
    let _ = client.delete_vpc().vpc_id(&vpc_id).send().await;
}

/// Scenario: hybrid connectivity to on-prem via a VPN gateway.
///
/// A network engineer wires the workload VPC to the corporate data centre
/// over IPSec. The on-prem router becomes a customer gateway, the AWS
/// side is a VPN gateway attached to the VPC, and a VPN connection links
/// the two. Verifies that the resulting `VpnConnection` references both
/// the customer-side and AWS-side gateway IDs, and that the VPN gateway
/// shows the VPC attachment.
#[tokio::test]
async fn test_hybrid_vpn_connectivity_to_on_prem() {
    let client = make_ec2_client().await;

    let vpc_id = client
        .create_vpc()
        .cidr_block("10.60.0.0/16")
        .send()
        .await
        .expect("create_vpc")
        .vpc()
        .and_then(|v| v.vpc_id())
        .expect("vpc id")
        .to_string();

    let cgw_id = client
        .create_customer_gateway()
        .bgp_asn(65001)
        .ip_address("198.51.100.42")
        .r#type(GatewayType::Ipsec1)
        .send()
        .await
        .expect("create_customer_gateway")
        .customer_gateway()
        .and_then(|c| c.customer_gateway_id())
        .expect("cgw id")
        .to_string();

    let vgw_id = client
        .create_vpn_gateway()
        .r#type(GatewayType::Ipsec1)
        .send()
        .await
        .expect("create_vpn_gateway")
        .vpn_gateway()
        .and_then(|v| v.vpn_gateway_id())
        .expect("vgw id")
        .to_string();

    client
        .attach_vpn_gateway()
        .vpn_gateway_id(&vgw_id)
        .vpc_id(&vpc_id)
        .send()
        .await
        .expect("attach VGW to VPC");

    let vpn_id = client
        .create_vpn_connection()
        .customer_gateway_id(&cgw_id)
        .vpn_gateway_id(&vgw_id)
        .r#type("ipsec.1")
        .send()
        .await
        .expect("create_vpn_connection")
        .vpn_connection()
        .and_then(|v| v.vpn_connection_id())
        .expect("vpn id")
        .to_string();

    // VPN connection should report both endpoints by ID.
    let conns = client
        .describe_vpn_connections()
        .vpn_connection_ids(&vpn_id)
        .send()
        .await
        .expect("describe_vpn_connections");
    let conn = conns
        .vpn_connections()
        .iter()
        .find(|v| v.vpn_connection_id() == Some(vpn_id.as_str()))
        .expect("vpn present");
    assert_eq!(
        conn.customer_gateway_id(),
        Some(cgw_id.as_str()),
        "VPN connection should reference the customer gateway"
    );
    assert_eq!(
        conn.vpn_gateway_id(),
        Some(vgw_id.as_str()),
        "VPN connection should reference the VPN gateway"
    );

    // VPN gateway should show the VPC attachment.
    let vgws = client
        .describe_vpn_gateways()
        .vpn_gateway_ids(&vgw_id)
        .send()
        .await
        .expect("describe_vpn_gateways");
    let vgw = vgws
        .vpn_gateways()
        .iter()
        .find(|g| g.vpn_gateway_id() == Some(vgw_id.as_str()))
        .expect("vgw present");
    assert!(
        vgw.vpc_attachments()
            .iter()
            .any(|a| a.vpc_id() == Some(vpc_id.as_str())),
        "VGW should report attachment to the workload VPC"
    );

    let _ = client
        .delete_vpn_connection()
        .vpn_connection_id(&vpn_id)
        .send()
        .await;
    let _ = client
        .detach_vpn_gateway()
        .vpn_gateway_id(&vgw_id)
        .vpc_id(&vpc_id)
        .send()
        .await;
    let _ = client
        .delete_vpn_gateway()
        .vpn_gateway_id(&vgw_id)
        .send()
        .await;
    let _ = client
        .delete_customer_gateway()
        .customer_gateway_id(&cgw_id)
        .send()
        .await;
    let _ = client.delete_vpc().vpc_id(&vpc_id).send().await;
}

/// Scenario: EIP re-association during a canary failover.
///
/// A canary EIP starts bound to instance A. After A fails health checks,
/// the operator binds the same EIP to a fresh instance B. Verifies that
/// the EIP's underlying allocation survives the move, that the second
/// `AssociateAddress` returns a fresh association id ( real AWS does not
/// reuse association ids ), and that `DescribeAddresses` reflects the
/// new instance binding rather than the old.
#[tokio::test]
async fn test_eip_failover_reassociation_across_instances() {
    let client = make_ec2_client().await;

    let alloc_id = client
        .allocate_address()
        .send()
        .await
        .expect("allocate EIP")
        .allocation_id()
        .expect("allocation_id")
        .to_string();

    let canary_a = client
        .run_instances()
        .image_id("ami-canary")
        .instance_type(InstanceType::T3Micro)
        .min_count(1)
        .max_count(1)
        .send()
        .await
        .expect("run canary A")
        .instances()
        .first()
        .and_then(|i| i.instance_id())
        .expect("canary A id")
        .to_string();
    let canary_b = client
        .run_instances()
        .image_id("ami-canary")
        .instance_type(InstanceType::T3Micro)
        .min_count(1)
        .max_count(1)
        .send()
        .await
        .expect("run canary B")
        .instances()
        .first()
        .and_then(|i| i.instance_id())
        .expect("canary B id")
        .to_string();

    let assoc_a = client
        .associate_address()
        .allocation_id(&alloc_id)
        .instance_id(&canary_a)
        .send()
        .await
        .expect("associate to A")
        .association_id()
        .expect("first assoc id")
        .to_string();

    // Failover: rebind the same EIP to canary B.
    let assoc_b = client
        .associate_address()
        .allocation_id(&alloc_id)
        .instance_id(&canary_b)
        .send()
        .await
        .expect("re-associate to B")
        .association_id()
        .expect("second assoc id")
        .to_string();

    assert_ne!(
        assoc_a, assoc_b,
        "re-associating must mint a fresh association id, not reuse the old one"
    );

    // EIP should be reported bound to B now, not A.
    let addrs = client
        .describe_addresses()
        .allocation_ids(&alloc_id)
        .send()
        .await
        .expect("describe EIP")
        .addresses()
        .first()
        .cloned()
        .expect("EIP described");
    assert_eq!(
        addrs.instance_id(),
        Some(canary_b.as_str()),
        "EIP should be bound to canary B after failover"
    );
    assert_eq!(
        addrs.association_id(),
        Some(assoc_b.as_str()),
        "EIP's current association id should match the second AssociateAddress"
    );

    let _ = client
        .disassociate_address()
        .association_id(&assoc_b)
        .send()
        .await;
    let _ = client
        .release_address()
        .allocation_id(&alloc_id)
        .send()
        .await;
    let _ = client
        .terminate_instances()
        .instance_ids(&canary_a)
        .instance_ids(&canary_b)
        .send()
        .await;
}

/// Scenario: account-wide EBS encryption-by-default enforcement.
///
/// A compliance team enables EBS encryption-by-default for the account.
/// All subsequent volumes — including those that did not request
/// encryption explicitly — must be created encrypted. After disabling
/// the toggle, brand new volumes return to the unencrypted default.
/// Verifies the toggle propagates into `CreateVolume` rather than
/// living only in `Get/EnableEbsEncryptionByDefault`.
#[tokio::test]
async fn test_ebs_encryption_by_default_propagates_to_new_volumes() {
    let client = make_ec2_client().await;

    // Sanity: with the toggle off, omitted Encrypted means unencrypted.
    let unencrypted_baseline = client
        .create_volume()
        .availability_zone("us-east-1a")
        .size(10)
        .send()
        .await
        .expect("create baseline volume")
        .volume_id()
        .expect("baseline vol id")
        .to_string();
    let baseline = client
        .describe_volumes()
        .volume_ids(&unencrypted_baseline)
        .send()
        .await
        .expect("describe baseline");
    assert_eq!(
        baseline.volumes()[0].encrypted(),
        Some(false),
        "with the toggle off, omitted Encrypted should default to false"
    );

    // Turn the account-wide knob on.
    client
        .enable_ebs_encryption_by_default()
        .send()
        .await
        .expect("enable encryption-by-default");

    let after_enable = client
        .create_volume()
        .availability_zone("us-east-1a")
        .size(10)
        .send()
        .await
        .expect("create after enable")
        .volume_id()
        .expect("vol id")
        .to_string();
    let described = client
        .describe_volumes()
        .volume_ids(&after_enable)
        .send()
        .await
        .expect("describe after enable");
    assert_eq!(
        described.volumes()[0].encrypted(),
        Some(true),
        "encryption-by-default must apply when the request omits Encrypted"
    );

    // An explicit Encrypted=false should still be allowed to override
    // ( AWS actually rejects this, but at minimum the toggle should not
    // silently flip user-requested non-encrypted volumes ). We assert
    // only that the toggle applies to the implicit default path.
    client
        .disable_ebs_encryption_by_default()
        .send()
        .await
        .expect("disable encryption-by-default");

    let after_disable = client
        .create_volume()
        .availability_zone("us-east-1a")
        .size(10)
        .send()
        .await
        .expect("create after disable")
        .volume_id()
        .expect("after-disable vol id")
        .to_string();
    let post = client
        .describe_volumes()
        .volume_ids(&after_disable)
        .send()
        .await
        .expect("describe after disable");
    assert_eq!(
        post.volumes()[0].encrypted(),
        Some(false),
        "after disabling the toggle, omitted Encrypted should return to false"
    );

    let _ = client
        .delete_volume()
        .volume_id(&unencrypted_baseline)
        .send()
        .await;
    let _ = client.delete_volume().volume_id(&after_enable).send().await;
    let _ = client
        .delete_volume()
        .volume_id(&after_disable)
        .send()
        .await;
}

/// Scenario: locking down a sensitive subnet with a custom NACL.
///
/// A subnet starts associated with the VPC's default ( allow-all ) NACL.
/// A security engineer creates a deny-by-default NACL, attaches a single
/// allow rule for SSH from the bastion CIDR, then takes over the
/// subnet's NACL slot via `ReplaceNetworkAclAssociation`. Verifies that
/// the subnet's association_id moves, the default NACL no longer lists
/// the subnet, the custom NACL does, and the custom rules are visible
/// on the new association.
#[tokio::test]
async fn test_subnet_nacl_takeover_with_deny_default() {
    let client = make_ec2_client().await;

    let vpc_id = client
        .create_vpc()
        .cidr_block("10.70.0.0/16")
        .send()
        .await
        .expect("create_vpc")
        .vpc()
        .and_then(|v| v.vpc_id())
        .expect("vpc id")
        .to_string();

    let subnet_id = client
        .create_subnet()
        .vpc_id(&vpc_id)
        .cidr_block("10.70.1.0/24")
        .send()
        .await
        .expect("create subnet")
        .subnet()
        .and_then(|s| s.subnet_id())
        .expect("subnet id")
        .to_string();

    // Find the default NACL and its auto-association for the new subnet.
    let acls = client
        .describe_network_acls()
        .filters(Filter::builder().name("vpc-id").values(&vpc_id).build())
        .send()
        .await
        .expect("describe NACLs in VPC");
    let default_nacl = acls
        .network_acls()
        .iter()
        .find(|n| n.is_default() == Some(true))
        .expect("default NACL exists for VPC");
    let default_nacl_id = default_nacl
        .network_acl_id()
        .expect("default nacl id")
        .to_string();
    let original_assoc = default_nacl
        .associations()
        .iter()
        .find(|a| a.subnet_id() == Some(subnet_id.as_str()))
        .and_then(|a| a.network_acl_association_id())
        .expect("subnet auto-associated with default NACL")
        .to_string();

    // Custom NACL: starts deny-all by default, plus an explicit allow for SSH from a bastion CIDR.
    let custom_nacl = client
        .create_network_acl()
        .vpc_id(&vpc_id)
        .send()
        .await
        .expect("create custom NACL")
        .network_acl()
        .and_then(|n| n.network_acl_id())
        .expect("custom nacl id")
        .to_string();
    client
        .create_network_acl_entry()
        .network_acl_id(&custom_nacl)
        .rule_number(100)
        .protocol("6")
        .rule_action(aws_sdk_ec2::types::RuleAction::Allow)
        .egress(false)
        .cidr_block("10.70.99.0/24")
        .port_range(
            aws_sdk_ec2::types::PortRange::builder()
                .from(22)
                .to(22)
                .build(),
        )
        .send()
        .await
        .expect("authorize SSH from bastion");

    // Swap the subnet's NACL.
    let new_assoc = client
        .replace_network_acl_association()
        .association_id(&original_assoc)
        .network_acl_id(&custom_nacl)
        .send()
        .await
        .expect("replace NACL association")
        .new_association_id()
        .expect("new association id")
        .to_string();
    assert_ne!(
        new_assoc, original_assoc,
        "replacing should mint a fresh association id"
    );

    // Default NACL should no longer list the subnet; custom NACL should.
    let after = client
        .describe_network_acls()
        .filters(Filter::builder().name("vpc-id").values(&vpc_id).build())
        .send()
        .await
        .expect("re-describe NACLs");
    let default_after = after
        .network_acls()
        .iter()
        .find(|n| n.network_acl_id() == Some(default_nacl_id.as_str()))
        .expect("default NACL still present");
    assert!(
        !default_after
            .associations()
            .iter()
            .any(|a| a.subnet_id() == Some(subnet_id.as_str())),
        "default NACL should no longer list the locked-down subnet"
    );
    let custom_after = after
        .network_acls()
        .iter()
        .find(|n| n.network_acl_id() == Some(custom_nacl.as_str()))
        .expect("custom NACL present");
    let assoc = custom_after
        .associations()
        .iter()
        .find(|a| a.subnet_id() == Some(subnet_id.as_str()))
        .expect("custom NACL now owns the subnet");
    assert_eq!(
        assoc.network_acl_association_id(),
        Some(new_assoc.as_str()),
        "custom NACL's association should match the id returned by replace"
    );
    assert!(
        custom_after
            .entries()
            .iter()
            .any(|e| e.rule_number() == Some(100)
                && e.egress() == Some(false)
                && e.cidr_block() == Some("10.70.99.0/24")),
        "custom NACL should expose the bastion-only allow rule"
    );

    let _ = client
        .replace_network_acl_association()
        .association_id(&new_assoc)
        .network_acl_id(&default_nacl_id)
        .send()
        .await;
    let _ = client
        .delete_network_acl()
        .network_acl_id(&custom_nacl)
        .send()
        .await;
    let _ = client.delete_subnet().subnet_id(&subnet_id).send().await;
    let _ = client.delete_vpc().vpc_id(&vpc_id).send().await;
}

/// Scenario: full instance state-machine walk.
///
/// An operator drives one instance through the canonical lifecycle:
/// `running` ( from RunInstances ) → `stopped` → `running` ( restart )
/// → `running` ( reboot is a no-op for state ) → `stopped` →
/// `terminated`. Verifies that each state-change handler reports the
/// previous and current state codes consistent with `DescribeInstances`
/// observed between calls — the property terraform / autoscaling rely
/// on for retry loops.
#[tokio::test]
async fn test_instance_full_state_machine_walk() {
    let client = make_ec2_client().await;

    let id = client
        .run_instances()
        .image_id("ami-state-walk")
        .instance_type(InstanceType::T3Micro)
        .min_count(1)
        .max_count(1)
        .send()
        .await
        .expect("run_instances")
        .instances()
        .first()
        .and_then(|i| i.instance_id())
        .expect("id")
        .to_string();

    async fn current_state(c: &aws_sdk_ec2::Client, id: &str) -> String {
        c.describe_instances()
            .instance_ids(id)
            .send()
            .await
            .expect("describe_instances")
            .reservations()
            .iter()
            .flat_map(|r| r.instances().iter())
            .find(|i| i.instance_id() == Some(id))
            .and_then(|i| i.state().and_then(|s| s.name()))
            .map(|n| n.as_str().to_string())
            .unwrap_or_else(|| "missing".to_string())
    }

    assert_eq!(current_state(&client, &id).await, "running");

    let stop = client
        .stop_instances()
        .instance_ids(&id)
        .send()
        .await
        .expect("stop_instances");
    let stop_change = &stop.stopping_instances()[0];
    assert_eq!(
        stop_change
            .previous_state()
            .and_then(|s| s.name())
            .map(|n| n.as_str()),
        Some("running")
    );
    assert_eq!(
        stop_change
            .current_state()
            .and_then(|s| s.name())
            .map(|n| n.as_str()),
        Some("stopped")
    );
    assert_eq!(current_state(&client, &id).await, "stopped");

    let start = client
        .start_instances()
        .instance_ids(&id)
        .send()
        .await
        .expect("start_instances");
    let start_change = &start.starting_instances()[0];
    assert_eq!(
        start_change
            .previous_state()
            .and_then(|s| s.name())
            .map(|n| n.as_str()),
        Some("stopped")
    );
    assert_eq!(
        start_change
            .current_state()
            .and_then(|s| s.name())
            .map(|n| n.as_str()),
        Some("running")
    );
    assert_eq!(current_state(&client, &id).await, "running");

    // Reboot does not change state.
    client
        .reboot_instances()
        .instance_ids(&id)
        .send()
        .await
        .expect("reboot_instances");
    assert_eq!(current_state(&client, &id).await, "running");

    // Stop again, then terminate from stopped.
    client
        .stop_instances()
        .instance_ids(&id)
        .send()
        .await
        .expect("second stop");
    assert_eq!(current_state(&client, &id).await, "stopped");

    let term = client
        .terminate_instances()
        .instance_ids(&id)
        .send()
        .await
        .expect("terminate_instances");
    let term_change = &term.terminating_instances()[0];
    assert_eq!(
        term_change
            .previous_state()
            .and_then(|s| s.name())
            .map(|n| n.as_str()),
        Some("stopped"),
        "terminate should report previous state as stopped"
    );
    assert_eq!(
        term_change
            .current_state()
            .and_then(|s| s.name())
            .map(|n| n.as_str()),
        Some("terminated")
    );
}

/// Scenario: secondary ENI hot-attach across two instances.
///
/// Invariant: per-call uniqueness — each `AttachNetworkInterface` mints a
/// fresh `eni-attach-XXXX` id even when called against distinct ENIs in
/// rapid succession. This was the same shape as the EIP-association bug
/// surfaced in the 2026-05-02 EC2 audit: the helper read the ENI counter
/// without incrementing, so back-to-back attaches collided on suffix.
#[tokio::test]
async fn test_eni_attach_ids_are_unique_per_call() {
    let client = make_ec2_client().await;

    let vpc_id = client
        .create_vpc()
        .cidr_block("10.180.0.0/16")
        .send()
        .await
        .expect("create_vpc")
        .vpc()
        .unwrap()
        .vpc_id()
        .unwrap()
        .to_string();
    let subnet_id = client
        .create_subnet()
        .vpc_id(&vpc_id)
        .cidr_block("10.180.1.0/24")
        .send()
        .await
        .expect("create_subnet")
        .subnet()
        .unwrap()
        .subnet_id()
        .unwrap()
        .to_string();

    let mut instance_ids = Vec::new();
    for _ in 0..2 {
        let run = client
            .run_instances()
            .min_count(1)
            .max_count(1)
            .image_id("ami-12345678")
            .instance_type(InstanceType::T2Micro)
            .subnet_id(&subnet_id)
            .send()
            .await
            .expect("run_instances");
        instance_ids.push(run.instances()[0].instance_id().unwrap().to_string());
    }

    let mut attach_ids = Vec::new();
    for instance_id in &instance_ids {
        let eni_id = client
            .create_network_interface()
            .subnet_id(&subnet_id)
            .send()
            .await
            .expect("create_network_interface")
            .network_interface()
            .unwrap()
            .network_interface_id()
            .unwrap()
            .to_string();
        let attach_id = client
            .attach_network_interface()
            .network_interface_id(&eni_id)
            .instance_id(instance_id)
            .device_index(1)
            .send()
            .await
            .expect("attach_network_interface")
            .attachment_id()
            .expect("attachment_id should be returned")
            .to_string();
        assert!(
            attach_id.starts_with("eni-attach-"),
            "unexpected attachment id format: {attach_id}"
        );
        attach_ids.push(attach_id);
    }

    assert_ne!(
        attach_ids[0], attach_ids[1],
        "AttachNetworkInterface must mint a distinct eni-attach id per call; \
         got {} both times",
        attach_ids[0]
    );
}
