use crate::harness::*;

// Note: EC2 resources are highly interconnected (subnets reference VPCs, security groups
// reference VPCs, etc.) and the terraform provider makes many cross-resource describe calls.
// All EC2 tests use isolated servers (not batch_apply) to avoid state contamination.

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn debug_ec2_subnet_smoke() {
    let result = smoke_test_terraform(
        test_services(),
        r#"
resource "aws_vpc" "test" {
  cidr_block = "10.0.0.0/16"
}

resource "aws_subnet" "test" {
  vpc_id     = aws_vpc.test.id
  cidr_block = "10.0.1.0/24"
}

resource "aws_internet_gateway" "test" {
  vpc_id = aws_vpc.test.id
}

resource "aws_security_group" "test" {
  name   = "test-sg"
  vpc_id = aws_vpc.test.id

  ingress {
    from_port   = 80
    to_port     = 80
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  egress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }
}

resource "aws_route_table" "test" {
  vpc_id = aws_vpc.test.id

  route {
    cidr_block = "0.0.0.0/0"
    gateway_id = aws_internet_gateway.test.id
  }
}

resource "aws_route_table_association" "test" {
  subnet_id      = aws_subnet.test.id
  route_table_id = aws_route_table.test.id
}
"#,
        std::time::Duration::from_secs(90),
    )
    .await;
    eprintln!("{result}");
    for call in &result.api_calls {
        eprintln!("  {} -> {}", call.operation, call.status_code);
    }
    eprintln!("Missing ops: {:?}", result.missing_operations);
    eprintln!("Failed ops: {:?}", result.failed_operations);
    assert!(result.success, "smoke test failed");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_ec2_vpc_basic() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("ec2-vpc-basic");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_vpc" "test" {
  cidr_block = "10.0.0.0/16"
}
"#,
    )
    .unwrap();
    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");
    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(state.contains("10.0.0.0/16"), "state missing CIDR");
    cleanup_tf_dir(&dir);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_ec2_vpc_with_dns() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("ec2-vpc-dns");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_vpc" "test" {
  cidr_block           = "10.1.0.0/16"
  enable_dns_hostnames = true
  enable_dns_support   = true
}
"#,
    )
    .unwrap();
    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");
    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(state.contains("10.1.0.0/16"), "state missing CIDR");
    assert!(
        state.contains("\"enable_dns_hostnames\": true"),
        "state missing dns_hostnames"
    );
    assert!(
        state.contains("\"enable_dns_support\": true"),
        "state missing dns_support"
    );
    cleanup_tf_dir(&dir);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_ec2_vpc_with_tags() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("ec2-vpc-tags");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_vpc" "test" {
  cidr_block = "10.2.0.0/16"

  tags = {
    Name = "ec2-vpc-tags-test"
    Env  = "testing"
  }
}
"#,
    )
    .unwrap();
    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");
    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(
        state.contains("ec2-vpc-tags-test"),
        "state missing tag Name"
    );
    cleanup_tf_dir(&dir);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_ec2_subnet_basic() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("ec2-subnet-basic");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_vpc" "test" {
  cidr_block = "10.3.0.0/16"
}

resource "aws_subnet" "test" {
  vpc_id     = aws_vpc.test.id
  cidr_block = "10.3.1.0/24"
}
"#,
    )
    .unwrap();
    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");
    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(state.contains("10.3.1.0/24"), "state missing subnet CIDR");
    cleanup_tf_dir(&dir);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_ec2_subnet_public() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("ec2-subnet-public");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_vpc" "test" {
  cidr_block = "10.4.0.0/16"
}

resource "aws_subnet" "test" {
  vpc_id                  = aws_vpc.test.id
  cidr_block              = "10.4.1.0/24"
  map_public_ip_on_launch = true
}
"#,
    )
    .unwrap();
    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");
    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(state.contains("10.4.1.0/24"), "state missing subnet CIDR");
    assert!(
        state.contains("\"map_public_ip_on_launch\": true"),
        "state missing map_public_ip_on_launch"
    );
    cleanup_tf_dir(&dir);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_ec2_internet_gateway_basic() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("ec2-igw-basic");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_vpc" "igw_vpc" {
  cidr_block = "10.5.0.0/16"
}

resource "aws_internet_gateway" "test" {
  vpc_id = aws_vpc.igw_vpc.id
}
"#,
    )
    .unwrap();
    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");
    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(
        state.contains("aws_internet_gateway"),
        "state missing internet gateway"
    );
    cleanup_tf_dir(&dir);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_ec2_security_group_basic() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("ec2-sg-basic");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_vpc" "sg_vpc" {
  cidr_block = "10.6.0.0/16"
}

resource "aws_security_group" "test" {
  name        = "ec2-sg-basic"
  description = "Test SG"
  vpc_id      = aws_vpc.sg_vpc.id

  ingress {
    from_port   = 80
    to_port     = 80
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  egress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }
}
"#,
    )
    .unwrap();
    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");
    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(state.contains("ec2-sg-basic"), "state missing sg name");
    assert!(
        state.contains("\"description\""),
        "state missing description field"
    );
    cleanup_tf_dir(&dir);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_ec2_route_table_basic() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("ec2-rt-basic");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_vpc" "rt_vpc" {
  cidr_block = "10.7.0.0/16"
}

resource "aws_subnet" "rt_subnet" {
  vpc_id     = aws_vpc.rt_vpc.id
  cidr_block = "10.7.1.0/24"
}

resource "aws_internet_gateway" "rt_igw" {
  vpc_id = aws_vpc.rt_vpc.id
}

resource "aws_route_table" "test" {
  vpc_id = aws_vpc.rt_vpc.id

  route {
    cidr_block = "0.0.0.0/0"
    gateway_id = aws_internet_gateway.rt_igw.id
  }
}

resource "aws_route_table_association" "test" {
  subnet_id      = aws_subnet.rt_subnet.id
  route_table_id = aws_route_table.test.id
}
"#,
    )
    .unwrap();
    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");
    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(
        state.contains("aws_route_table"),
        "state missing route table"
    );
    assert!(
        state.contains("aws_route_table_association"),
        "state missing route table association"
    );
    cleanup_tf_dir(&dir);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_ec2_key_pair_basic() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("ec2-keypair-basic");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_key_pair" "test" {
  key_name   = "ec2-keypair-basic"
  public_key = "ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAABAQC test@test"
}
"#,
    )
    .unwrap();
    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");
    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(
        state.contains("ec2-keypair-basic"),
        "state missing key pair name"
    );
    cleanup_tf_dir(&dir);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_ec2_full_stack() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("ec2-full-stack");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_vpc" "main" {
  cidr_block           = "10.8.0.0/16"
  enable_dns_hostnames = true
  enable_dns_support   = true

  tags = {
    Name = "ec2-full-stack-vpc"
  }
}

resource "aws_subnet" "public" {
  vpc_id                  = aws_vpc.main.id
  cidr_block              = "10.8.1.0/24"
  map_public_ip_on_launch = true

  tags = {
    Name = "ec2-full-stack-public-subnet"
  }
}

resource "aws_internet_gateway" "main" {
  vpc_id = aws_vpc.main.id

  tags = {
    Name = "ec2-full-stack-igw"
  }
}

resource "aws_security_group" "web" {
  name        = "ec2-full-stack-web-sg"
  description = "Web security group"
  vpc_id      = aws_vpc.main.id

  ingress {
    from_port   = 80
    to_port     = 80
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  ingress {
    from_port   = 443
    to_port     = 443
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  egress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }

  tags = {
    Name = "ec2-full-stack-web-sg"
  }
}

resource "aws_route_table" "public" {
  vpc_id = aws_vpc.main.id

  route {
    cidr_block = "0.0.0.0/0"
    gateway_id = aws_internet_gateway.main.id
  }

  tags = {
    Name = "ec2-full-stack-public-rt"
  }
}

resource "aws_route_table_association" "public" {
  subnet_id      = aws_subnet.public.id
  route_table_id = aws_route_table.public.id
}
"#,
    )
    .unwrap();
    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");
    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(
        state.contains("ec2-full-stack-vpc"),
        "state missing vpc tag"
    );
    assert!(state.contains("10.8.1.0/24"), "state missing subnet CIDR");
    assert!(
        state.contains("aws_internet_gateway"),
        "state missing internet gateway"
    );
    assert!(
        state.contains("ec2-full-stack-web-sg"),
        "state missing security group"
    );
    assert!(
        state.contains("aws_route_table_association"),
        "state missing route table association"
    );
    cleanup_tf_dir(&dir);
}

// ---------------------------------------------------------------------------
// Placement Group tests
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_ec2_placement_group_cluster_basic() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("ec2-placement-group-cluster");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_placement_group" "test" {
  name     = "ec2-pg-cluster-basic"
  strategy = "cluster"
}
"#,
    )
    .unwrap();
    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");
    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(
        state.contains("ec2-pg-cluster-basic"),
        "state missing placement group name"
    );
    assert!(
        state.contains("\"strategy\": \"cluster\""),
        "state missing strategy"
    );
    cleanup_tf_dir(&dir);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_ec2_placement_group_partition() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("ec2-placement-group-partition");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_placement_group" "test" {
  name            = "ec2-pg-partition"
  strategy        = "partition"
  partition_count = 3
}
"#,
    )
    .unwrap();
    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");
    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(
        state.contains("ec2-pg-partition"),
        "state missing placement group name"
    );
    assert!(
        state.contains("\"partition_count\": 3"),
        "state missing partition_count"
    );
    cleanup_tf_dir(&dir);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_ec2_placement_group_with_tags() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("ec2-placement-group-tags");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_placement_group" "test" {
  name     = "ec2-pg-tagged"
  strategy = "spread"

  tags = {
    Name = "ec2-pg-tagged"
    Env  = "testing"
  }
}
"#,
    )
    .unwrap();
    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");
    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(state.contains("ec2-pg-tagged"), "state missing pg name");
    assert!(
        state.contains("\"strategy\": \"spread\""),
        "state missing strategy"
    );
    cleanup_tf_dir(&dir);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_ec2_egress_only_internet_gateway_basic() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("ec2-eigw-basic");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_vpc" "test" {
  cidr_block = "10.4.0.0/16"
}

resource "aws_egress_only_internet_gateway" "test" {
  vpc_id = aws_vpc.test.id
}
"#,
    )
    .unwrap();
    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");
    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(
        state.contains("aws_egress_only_internet_gateway"),
        "state missing aws_egress_only_internet_gateway resource"
    );
    assert!(state.contains("\"vpc_id\""), "state missing vpc_id linkage");
    cleanup_tf_dir(&dir);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_ec2_egress_only_internet_gateway_with_tags() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("ec2-eigw-tags");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_vpc" "test" {
  cidr_block = "10.5.0.0/16"
}

resource "aws_egress_only_internet_gateway" "test" {
  vpc_id = aws_vpc.test.id

  tags = {
    Name = "ec2-eigw-tagged"
    Env  = "testing"
  }
}
"#,
    )
    .unwrap();
    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");
    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(
        state.contains("ec2-eigw-tagged"),
        "state missing tagged eigw name"
    );
    cleanup_tf_dir(&dir);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_ec2_network_acl_basic() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("ec2-nacl-basic");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_vpc" "test" {
  cidr_block = "10.6.0.0/16"
}

resource "aws_network_acl" "test" {
  vpc_id = aws_vpc.test.id

  tags = {
    Name = "ec2-nacl-basic"
  }
}
"#,
    )
    .unwrap();
    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");
    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(
        state.contains("aws_network_acl"),
        "state missing aws_network_acl"
    );
    assert!(state.contains("ec2-nacl-basic"), "state missing tag Name");
    cleanup_tf_dir(&dir);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_ec2_network_acl_with_rules() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("ec2-nacl-rules");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_vpc" "test" {
  cidr_block = "10.7.0.0/16"
}

resource "aws_network_acl" "test" {
  vpc_id = aws_vpc.test.id

  ingress {
    protocol   = "tcp"
    rule_no    = 100
    action     = "allow"
    cidr_block = "0.0.0.0/0"
    from_port  = 80
    to_port    = 80
  }

  egress {
    protocol   = "-1"
    rule_no    = 200
    action     = "allow"
    cidr_block = "0.0.0.0/0"
    from_port  = 0
    to_port    = 0
  }

  tags = {
    Name = "ec2-nacl-with-rules"
  }
}
"#,
    )
    .unwrap();
    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");
    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(
        state.contains("ec2-nacl-with-rules"),
        "state missing tag Name"
    );
    cleanup_tf_dir(&dir);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_ec2_customer_gateway_basic() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("ec2-cgw-basic");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_customer_gateway" "test" {
  bgp_asn    = 65000
  ip_address = "203.0.113.12"
  type       = "ipsec.1"

  tags = {
    Name = "ec2-cgw-basic"
  }
}
"#,
    )
    .unwrap();
    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");
    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(
        state.contains("aws_customer_gateway"),
        "state missing aws_customer_gateway"
    );
    assert!(
        state.contains("203.0.113.12"),
        "state missing customer gateway IP"
    );
    assert!(state.contains("ec2-cgw-basic"), "state missing tag Name");
    cleanup_tf_dir(&dir);
}

// ---------------------------------------------------------------------------
// Capacity Reservation tests
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_ec2_capacity_reservation_basic() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("ec2-capacity-reservation-basic");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_ec2_capacity_reservation" "test" {
  instance_type     = "t3.large"
  instance_platform = "Linux/UNIX"
  availability_zone = "us-east-1a"
  instance_count    = 2

  tags = {
    Name = "ec2-cr-basic"
  }
}
"#,
    )
    .unwrap();
    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");
    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(
        state.contains("aws_ec2_capacity_reservation"),
        "state missing aws_ec2_capacity_reservation"
    );
    assert!(
        state.contains("\"instance_type\": \"t3.large\""),
        "state missing instance_type"
    );
    assert!(
        state.contains("\"instance_count\": 2"),
        "state missing instance_count"
    );
    cleanup_tf_dir(&dir);
}

// ---------------------------------------------------------------------------
// Network Interface Permission tests
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_ec2_network_interface_permission_basic() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("ec2-eni-permission-basic");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_vpc" "test" {
  cidr_block = "10.20.0.0/16"
}

resource "aws_subnet" "test" {
  vpc_id     = aws_vpc.test.id
  cidr_block = "10.20.1.0/24"
}

resource "aws_network_interface" "test" {
  subnet_id = aws_subnet.test.id
}

resource "aws_network_interface_permission" "test" {
  network_interface_id = aws_network_interface.test.id
  aws_account_id       = "999988887777"
  permission           = "INSTANCE-ATTACH"
}
"#,
    )
    .unwrap();
    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");
    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(
        state.contains("aws_network_interface_permission"),
        "state missing aws_network_interface_permission"
    );
    assert!(
        state.contains("INSTANCE-ATTACH"),
        "state missing permission value"
    );
    cleanup_tf_dir(&dir);
}

// ---------------------------------------------------------------------------
// Instance Connect Endpoint tests
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_ec2_instance_connect_endpoint_basic() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("ec2-instance-connect-endpoint-basic");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_vpc" "test" {
  cidr_block = "10.21.0.0/16"
}

resource "aws_subnet" "test" {
  vpc_id     = aws_vpc.test.id
  cidr_block = "10.21.1.0/24"
}

resource "aws_ec2_instance_connect_endpoint" "test" {
  subnet_id          = aws_subnet.test.id
  preserve_client_ip = true

  tags = {
    Name = "ec2-ice-basic"
  }
}
"#,
    )
    .unwrap();
    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");
    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(
        state.contains("aws_ec2_instance_connect_endpoint"),
        "state missing aws_ec2_instance_connect_endpoint"
    );
    assert!(state.contains("ec2-ice-basic"), "state missing tag Name");
    cleanup_tf_dir(&dir);
}

// ---------------------------------------------------------------------------
// IPAM tests
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_ec2_vpc_ipam_full_stack() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("ec2-vpc-ipam-full-stack");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_vpc_ipam" "test" {
  description = "ipam stack test"
  operating_regions {
    region_name = "us-east-1"
  }
  tags = {
    Name = "ipam-stack"
  }
}

resource "aws_vpc_ipam_scope" "test" {
  ipam_id     = aws_vpc_ipam.test.id
  description = "extra private scope"
}

resource "aws_vpc_ipam_pool" "test" {
  address_family = "ipv4"
  ipam_scope_id  = aws_vpc_ipam_scope.test.id
  locale         = "us-east-1"
  description    = "stack pool"
}

resource "aws_vpc_ipam_pool_cidr" "test" {
  ipam_pool_id = aws_vpc_ipam_pool.test.id
  cidr         = "10.0.0.0/16"
}

resource "aws_vpc_ipam_resource_discovery" "test" {
  description = "stack resource discovery"
  operating_regions {
    region_name = "us-east-1"
  }
}
"#,
    )
    .unwrap();
    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");
    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(state.contains("aws_vpc_ipam"), "state missing aws_vpc_ipam");
    assert!(
        state.contains("aws_vpc_ipam_scope"),
        "state missing aws_vpc_ipam_scope"
    );
    assert!(
        state.contains("aws_vpc_ipam_pool"),
        "state missing aws_vpc_ipam_pool"
    );
    assert!(
        state.contains("aws_vpc_ipam_pool_cidr"),
        "state missing aws_vpc_ipam_pool_cidr"
    );
    assert!(
        state.contains("aws_vpc_ipam_resource_discovery"),
        "state missing aws_vpc_ipam_resource_discovery"
    );
    assert!(state.contains("10.0.0.0/16"), "state missing pool CIDR");
    cleanup_tf_dir(&dir);
}

// ---------------------------------------------------------------------------
// Verified Access tests
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_ec2_verifiedaccess_stack() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("ec2-verifiedaccess-stack");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_verifiedaccess_trust_provider" "test" {
  policy_reference_name    = "policy_user"
  trust_provider_type      = "user"
  user_trust_provider_type = "iam-identity-center"
}

resource "aws_verifiedaccess_instance" "test" {
  description = "verified access stack"
  tags = {
    Name = "vai-stack"
  }
}

resource "aws_verifiedaccess_group" "test" {
  verifiedaccess_instance_id = aws_verifiedaccess_instance.test.id
  description                = "stack group"
}
"#,
    )
    .unwrap();
    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");
    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(
        state.contains("aws_verifiedaccess_instance"),
        "state missing aws_verifiedaccess_instance"
    );
    assert!(
        state.contains("aws_verifiedaccess_trust_provider"),
        "state missing aws_verifiedaccess_trust_provider"
    );
    assert!(
        state.contains("aws_verifiedaccess_group"),
        "state missing aws_verifiedaccess_group"
    );
    cleanup_tf_dir(&dir);
}

// ---------------------------------------------------------------------------
// Traffic Mirror tests
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_ec2_traffic_mirror_stack() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("ec2-traffic-mirror-stack");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_vpc" "test" {
  cidr_block = "10.50.0.0/16"
}

resource "aws_subnet" "test" {
  vpc_id     = aws_vpc.test.id
  cidr_block = "10.50.1.0/24"
}

resource "aws_network_interface" "target" {
  subnet_id = aws_subnet.test.id
}

resource "aws_network_interface" "source" {
  subnet_id = aws_subnet.test.id
}

resource "aws_ec2_traffic_mirror_target" "test" {
  description          = "tm target"
  network_interface_id = aws_network_interface.target.id
}

resource "aws_ec2_traffic_mirror_filter" "test" {
  description      = "tm filter"
  network_services = ["amazon-dns"]
}

resource "aws_ec2_traffic_mirror_filter_rule" "test" {
  traffic_mirror_filter_id = aws_ec2_traffic_mirror_filter.test.id
  destination_cidr_block   = "10.50.0.0/16"
  source_cidr_block        = "0.0.0.0/0"
  rule_action              = "accept"
  rule_number              = 100
  traffic_direction        = "ingress"
}

resource "aws_ec2_traffic_mirror_session" "test" {
  description              = "tm session"
  network_interface_id     = aws_network_interface.source.id
  session_number           = 1
  traffic_mirror_filter_id = aws_ec2_traffic_mirror_filter.test.id
  traffic_mirror_target_id = aws_ec2_traffic_mirror_target.test.id
}
"#,
    )
    .unwrap();
    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");
    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(
        state.contains("aws_ec2_traffic_mirror_target"),
        "state missing aws_ec2_traffic_mirror_target"
    );
    assert!(
        state.contains("aws_ec2_traffic_mirror_filter"),
        "state missing aws_ec2_traffic_mirror_filter"
    );
    assert!(
        state.contains("aws_ec2_traffic_mirror_filter_rule"),
        "state missing aws_ec2_traffic_mirror_filter_rule"
    );
    assert!(
        state.contains("aws_ec2_traffic_mirror_session"),
        "state missing aws_ec2_traffic_mirror_session"
    );
    cleanup_tf_dir(&dir);
}

// ---------------------------------------------------------------------------
// Transit Gateway extensions tests
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_ec2_tgw_extensions_stack() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("ec2-tgw-extensions-stack");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_ec2_transit_gateway" "test" {
  description                     = "tgw extensions test"
  multicast_support               = "enable"
  transit_gateway_cidr_blocks     = ["10.99.0.0/24"]
}

resource "aws_vpc" "test" {
  cidr_block = "10.60.0.0/16"
}

resource "aws_subnet" "test" {
  vpc_id     = aws_vpc.test.id
  cidr_block = "10.60.1.0/24"
}

resource "aws_ec2_transit_gateway_vpc_attachment" "test" {
  subnet_ids         = [aws_subnet.test.id]
  transit_gateway_id = aws_ec2_transit_gateway.test.id
  vpc_id             = aws_vpc.test.id
}

resource "aws_ec2_transit_gateway_multicast_domain" "test" {
  transit_gateway_id              = aws_ec2_transit_gateway.test.id
  igmpv2_support                  = "enable"
  static_sources_support          = "disable"
  auto_accept_shared_associations = "enable"
}

resource "aws_ec2_transit_gateway_connect" "test" {
  transit_gateway_id      = aws_ec2_transit_gateway.test.id
  transport_attachment_id = aws_ec2_transit_gateway_vpc_attachment.test.id
}

resource "aws_ec2_transit_gateway_policy_table" "test" {
  transit_gateway_id = aws_ec2_transit_gateway.test.id
}
"#,
    )
    .unwrap();
    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");
    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(
        state.contains("aws_ec2_transit_gateway_multicast_domain"),
        "state missing aws_ec2_transit_gateway_multicast_domain"
    );
    assert!(
        state.contains("aws_ec2_transit_gateway_connect"),
        "state missing aws_ec2_transit_gateway_connect"
    );
    assert!(
        state.contains("aws_ec2_transit_gateway_policy_table"),
        "state missing aws_ec2_transit_gateway_policy_table"
    );
    cleanup_tf_dir(&dir);
}

// ---------------------------------------------------------------------------
// Network Insights tests
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_ec2_network_insights_stack() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("ec2-network-insights-stack");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_vpc" "test" {
  cidr_block = "10.70.0.0/16"
}

resource "aws_subnet" "test" {
  vpc_id     = aws_vpc.test.id
  cidr_block = "10.70.1.0/24"
}

resource "aws_network_interface" "src" {
  subnet_id = aws_subnet.test.id
}

resource "aws_network_interface" "dst" {
  subnet_id = aws_subnet.test.id
}

resource "aws_ec2_network_insights_path" "test" {
  source           = aws_network_interface.src.id
  destination      = aws_network_interface.dst.id
  protocol         = "tcp"
  destination_port = 443
}

resource "aws_ec2_network_insights_analysis" "test" {
  network_insights_path_id = aws_ec2_network_insights_path.test.id
}
"#,
    )
    .unwrap();
    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");
    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(
        state.contains("aws_ec2_network_insights_path"),
        "state missing aws_ec2_network_insights_path"
    );
    assert!(
        state.contains("aws_ec2_network_insights_analysis"),
        "state missing aws_ec2_network_insights_analysis"
    );
    cleanup_tf_dir(&dir);
}

// ---------------------------------------------------------------------------
// Batch C E2E tests: verify EC2 handler upgrades from Default::default()
// stubs to real state-backed implementations (Batches A and B).
//
// Each test runs `terraform apply` against a real start_server, then asserts
// the expected resource lands in TF state. The TF AWS provider issues a
// follow-up Describe* call after Create*, so these tests effectively cover
// both the create-path state mutator and the describe-path read.
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_ec2_local_gateway_route_basic() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("ec2-local-gateway-route-basic");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_ec2_local_gateway_route" "test" {
  destination_cidr_block                   = "172.16.0.0/16"
  local_gateway_route_table_id             = "lgw-rtb-0123456789abcdef0"
  local_gateway_virtual_interface_group_id = "lgw-vif-grp-0aaaaaaaaaaaaaaaa"
}
"#,
    )
    .unwrap();
    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");
    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(
        state.contains("aws_ec2_local_gateway_route"),
        "state missing aws_ec2_local_gateway_route"
    );
    assert!(
        state.contains("172.16.0.0/16"),
        "state missing destination CIDR"
    );
    cleanup_tf_dir(&dir);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_ec2_local_gateway_route_table_vpc_association_basic() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("ec2-lgw-rtb-vpc-assoc-basic");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_vpc" "test" {
  cidr_block = "10.80.0.0/16"
}

resource "aws_ec2_local_gateway_route_table_vpc_association" "test" {
  local_gateway_route_table_id = "lgw-rtb-0123456789abcdef0"
  vpc_id                       = aws_vpc.test.id

  tags = {
    Name = "ec2-lgw-vpc-assoc-basic"
  }
}
"#,
    )
    .unwrap();
    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");
    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(
        state.contains("aws_ec2_local_gateway_route_table_vpc_association"),
        "state missing aws_ec2_local_gateway_route_table_vpc_association"
    );
    assert!(
        state.contains("lgw-rtb-0123456789abcdef0"),
        "state missing local_gateway_route_table_id"
    );
    cleanup_tf_dir(&dir);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_ec2_vpc_ipam_pool_cidr_allocation_basic() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("ec2-ipam-pool-cidr-allocation-basic");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_vpc_ipam" "test" {
  operating_regions {
    region_name = "us-east-1"
  }
}

resource "aws_vpc_ipam_pool" "test" {
  address_family = "ipv4"
  ipam_scope_id  = aws_vpc_ipam.test.private_default_scope_id
  locale         = "us-east-1"
}

resource "aws_vpc_ipam_pool_cidr" "test" {
  ipam_pool_id = aws_vpc_ipam_pool.test.id
  cidr         = "10.0.0.0/16"
}

resource "aws_vpc_ipam_pool_cidr_allocation" "test" {
  ipam_pool_id = aws_vpc_ipam_pool.test.id
  cidr         = "10.0.5.0/24"
  description  = "ec2-ipam-pool-cidr-alloc-basic"

  depends_on = [aws_vpc_ipam_pool_cidr.test]
}
"#,
    )
    .unwrap();
    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");
    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(
        state.contains("aws_vpc_ipam_pool_cidr_allocation"),
        "state missing aws_vpc_ipam_pool_cidr_allocation"
    );
    assert!(
        state.contains("10.0.5.0/24"),
        "state missing allocation CIDR"
    );
    cleanup_tf_dir(&dir);
}

// Disabled: blocked by an upstream Terraform AWS provider v6.43.0 AutoFlex bug.
// The framework-SDK `aws_ec2_capacity_block_reservation` resource does not bridge
// SDK fields `CapacityReservationArn`, `CreateDate`, `TotalInstanceCount` to model
// fields `ARN`, `CreatedDate`, `InstanceCount`, so `terraform apply` fails with
// "Provider returned invalid result object after apply" regardless of the mock
// response. Mock-server changes alone cannot fix this; re-enable once a fixed
// provider release lands. Tracked as `ec2-capacity-block-reservation-autoflex`
// in `.agents/docs/TODO.md`; see `.agents/docs/services/ec2.md` for the analysis.
//
// #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
// #[ignore]
// async fn test_ec2_capacity_block_reservation_basic() {
//     let port = start_server(test_services()).await;
//     let url = format!("http://127.0.0.1:{port}");
//     let dir = create_tf_dir("ec2-capacity-block-reservation-basic");
//     write_provider_tf(&dir, &url);
//     std::fs::write(
//         dir.join("main.tf"),
//         r#"
// resource "aws_ec2_capacity_block_reservation" "test" {
//   capacity_block_offering_id = "cbo-0aaaaaaaaaaaaaaaa"
//   instance_platform          = "Linux/UNIX"
//
//   tags = {
//     Name = "ec2-cb-basic"
//   }
// }
// "#,
//     )
//     .unwrap();
//     terraform_init(&dir).await;
//     let (ok, _stdout, stderr) = terraform_apply(&dir).await;
//     assert!(ok, "terraform apply failed:\n{stderr}");
//     let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
//     assert!(
//         state.contains("aws_ec2_capacity_block_reservation"),
//         "state missing aws_ec2_capacity_block_reservation"
//     );
//     cleanup_tf_dir(&dir);
// }
