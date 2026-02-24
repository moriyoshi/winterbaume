use crate::harness::*;

// EKS terraform resources tested here:
//   aws_eks_cluster                    (CreateCluster, DescribeCluster, UpdateClusterConfig,
//                                       DeleteCluster, TagResource, ListTagsForResource)
//   aws_eks_node_group                 (CreateNodegroup, DescribeNodegroup, UpdateNodegroupConfig,
//                                       UpdateNodegroupVersion, DeleteNodegroup)
//   aws_eks_addon                      (CreateAddon, DescribeAddon, UpdateAddon, DeleteAddon,
//                                       ListAddons, DescribeAddonVersions)
//   aws_eks_access_entry               (CreateAccessEntry, DescribeAccessEntry, UpdateAccessEntry,
//                                       DeleteAccessEntry, ListAccessEntries)
//   aws_eks_access_policy_association  (AssociateAccessPolicy, DisassociateAccessPolicy,
//                                       ListAssociatedAccessPolicies)
//   aws_eks_pod_identity_association   (CreatePodIdentityAssociation, DescribePodIdentityAssociation,
//                                       UpdatePodIdentityAssociation, DeletePodIdentityAssociation)

// ---------------------------------------------------------------------------
// aws_eks_cluster
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_eks_cluster_basic() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("eks-cluster-basic");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_eks_cluster" "eks_cluster_basic" {
  name     = "eks-cluster-basic"
  role_arn = "arn:aws:iam::123456789012:role/eks-cluster-role"

  vpc_config {
    subnet_ids = ["subnet-12345678", "subnet-87654321"]
  }

  tags = {
    Name = "eks-cluster-basic"
  }
}
"#,
    )
    .unwrap();
    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");

    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(state.contains("eks-cluster-basic"), "state missing cluster");
    assert!(
        state.contains("aws_eks_cluster"),
        "state missing resource type"
    );

    cleanup_tf_dir(&dir);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_eks_cluster_with_logging() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("eks-cluster-logging");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_eks_cluster" "eks_cluster_logging" {
  name     = "eks-cluster-logging"
  role_arn = "arn:aws:iam::123456789012:role/eks-cluster-role"

  vpc_config {
    subnet_ids = ["subnet-12345678", "subnet-87654321"]
  }

  enabled_cluster_log_types = ["api", "audit"]

  tags = {
    Environment = "test"
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
        state.contains("eks-cluster-logging"),
        "state missing cluster"
    );

    cleanup_tf_dir(&dir);
}

// ---------------------------------------------------------------------------
// aws_eks_node_group
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_eks_node_group_basic() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("eks-node-group-basic");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_eks_cluster" "eks_ng_cluster" {
  name     = "eks-ng-cluster"
  role_arn = "arn:aws:iam::123456789012:role/eks-cluster-role"

  vpc_config {
    subnet_ids = ["subnet-12345678", "subnet-87654321"]
  }
}

resource "aws_eks_node_group" "eks_node_group_basic" {
  cluster_name    = aws_eks_cluster.eks_ng_cluster.name
  node_group_name = "eks-ng-basic"
  node_role_arn   = "arn:aws:iam::123456789012:role/eks-node-role"
  subnet_ids      = ["subnet-12345678", "subnet-87654321"]

  scaling_config {
    desired_size = 1
    max_size     = 2
    min_size     = 1
  }

  tags = {
    Name = "eks-ng-basic"
  }
}
"#,
    )
    .unwrap();
    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");

    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(state.contains("eks-ng-basic"), "state missing node group");
    assert!(
        state.contains("aws_eks_node_group"),
        "state missing resource type"
    );

    cleanup_tf_dir(&dir);
}

// ---------------------------------------------------------------------------
// aws_eks_addon
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_eks_addon_basic() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("eks-addon-basic");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_eks_cluster" "eks_addon_cluster" {
  name     = "eks-addon-cluster"
  role_arn = "arn:aws:iam::123456789012:role/eks-cluster-role"

  vpc_config {
    subnet_ids = ["subnet-12345678", "subnet-87654321"]
  }
}

resource "aws_eks_addon" "eks_addon_basic" {
  cluster_name = aws_eks_cluster.eks_addon_cluster.name
  addon_name   = "vpc-cni"

  tags = {
    Name = "eks-addon-basic"
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
        state.contains("aws_eks_addon"),
        "state missing resource type"
    );
    assert!(state.contains("vpc-cni"), "state missing addon name");

    cleanup_tf_dir(&dir);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_eks_addon_with_version() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("eks-addon-version");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_eks_cluster" "eks_addon_ver_cluster" {
  name     = "eks-addon-ver-cluster"
  role_arn = "arn:aws:iam::123456789012:role/eks-cluster-role"

  vpc_config {
    subnet_ids = ["subnet-12345678", "subnet-87654321"]
  }
}

resource "aws_eks_addon" "eks_addon_with_version" {
  cluster_name                = aws_eks_cluster.eks_addon_ver_cluster.name
  addon_name                  = "coredns"
  addon_version               = "v1.10.1-eksbuild.1"
  resolve_conflicts_on_create = "OVERWRITE"

  tags = {
    Environment = "test"
  }
}
"#,
    )
    .unwrap();
    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");

    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(state.contains("coredns"), "state missing addon name");

    cleanup_tf_dir(&dir);
}

// ---------------------------------------------------------------------------
// aws_eks_access_entry
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_eks_access_entry_basic() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("eks-access-entry-basic");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_eks_cluster" "eks_ae_cluster" {
  name     = "eks-ae-cluster"
  role_arn = "arn:aws:iam::123456789012:role/eks-cluster-role"

  vpc_config {
    subnet_ids = ["subnet-12345678", "subnet-87654321"]
  }
}

resource "aws_eks_access_entry" "eks_access_entry_basic" {
  cluster_name  = aws_eks_cluster.eks_ae_cluster.name
  principal_arn = "arn:aws:iam::123456789012:role/my-role"
  type          = "STANDARD"

  tags = {
    Name = "eks-access-entry-basic"
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
        state.contains("aws_eks_access_entry"),
        "state missing resource type"
    );
    assert!(state.contains("my-role"), "state missing principal role");

    cleanup_tf_dir(&dir);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_eks_access_entry_with_k8s_groups() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("eks-access-entry-k8s");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_eks_cluster" "eks_ae_k8s_cluster" {
  name     = "eks-ae-k8s-cluster"
  role_arn = "arn:aws:iam::123456789012:role/eks-cluster-role"

  vpc_config {
    subnet_ids = ["subnet-12345678", "subnet-87654321"]
  }
}

resource "aws_eks_access_entry" "eks_access_entry_with_k8s_groups" {
  cluster_name      = aws_eks_cluster.eks_ae_k8s_cluster.name
  principal_arn     = "arn:aws:iam::123456789012:role/dev-role"
  type              = "STANDARD"
  kubernetes_groups = ["viewers", "developers"]
}
"#,
    )
    .unwrap();
    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");

    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(
        state.contains("aws_eks_access_entry"),
        "state missing resource type"
    );

    cleanup_tf_dir(&dir);
}

// ---------------------------------------------------------------------------
// aws_eks_access_policy_association
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_eks_access_policy_association_basic() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("eks-access-policy-assoc");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_eks_cluster" "eks_apa_cluster" {
  name     = "eks-apa-cluster"
  role_arn = "arn:aws:iam::123456789012:role/eks-cluster-role"

  vpc_config {
    subnet_ids = ["subnet-12345678", "subnet-87654321"]
  }
}

resource "aws_eks_access_entry" "eks_apa_entry" {
  cluster_name  = aws_eks_cluster.eks_apa_cluster.name
  principal_arn = "arn:aws:iam::123456789012:role/admin-role"
  type          = "STANDARD"
}

resource "aws_eks_access_policy_association" "eks_access_policy_association_basic" {
  cluster_name  = aws_eks_cluster.eks_apa_cluster.name
  principal_arn = aws_eks_access_entry.eks_apa_entry.principal_arn
  policy_arn    = "arn:aws:eks::aws:cluster-access-policy/AmazonEKSClusterAdminPolicy"

  access_scope {
    type = "cluster"
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
        state.contains("aws_eks_access_policy_association"),
        "state missing resource type"
    );
    assert!(
        state.contains("AmazonEKSClusterAdminPolicy"),
        "state missing policy"
    );

    cleanup_tf_dir(&dir);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_eks_access_policy_association_namespace() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("eks-access-policy-ns");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_eks_cluster" "eks_apa_ns_cluster" {
  name     = "eks-apa-ns-cluster"
  role_arn = "arn:aws:iam::123456789012:role/eks-cluster-role"

  vpc_config {
    subnet_ids = ["subnet-12345678", "subnet-87654321"]
  }
}

resource "aws_eks_access_entry" "eks_apa_ns_entry" {
  cluster_name  = aws_eks_cluster.eks_apa_ns_cluster.name
  principal_arn = "arn:aws:iam::123456789012:role/ns-admin-role"
  type          = "STANDARD"
}

resource "aws_eks_access_policy_association" "eks_access_policy_association_namespace" {
  cluster_name  = aws_eks_cluster.eks_apa_ns_cluster.name
  principal_arn = aws_eks_access_entry.eks_apa_ns_entry.principal_arn
  policy_arn    = "arn:aws:eks::aws:cluster-access-policy/AmazonEKSAdminPolicy"

  access_scope {
    type       = "namespace"
    namespaces = ["default", "kube-system"]
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
        state.contains("aws_eks_access_policy_association"),
        "state missing resource type"
    );

    cleanup_tf_dir(&dir);
}

// ---------------------------------------------------------------------------
// aws_eks_pod_identity_association
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_eks_pod_identity_association_basic() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("eks-pod-id-basic");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_eks_cluster" "eks_pid_cluster" {
  name     = "eks-pid-cluster"
  role_arn = "arn:aws:iam::123456789012:role/eks-cluster-role"

  vpc_config {
    subnet_ids = ["subnet-12345678", "subnet-87654321"]
  }
}

resource "aws_eks_pod_identity_association" "eks_pod_identity_association_basic" {
  cluster_name    = aws_eks_cluster.eks_pid_cluster.name
  namespace       = "default"
  service_account = "my-service-account"
  role_arn        = "arn:aws:iam::123456789012:role/pod-role"

  tags = {
    Name = "eks-pod-id-basic"
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
        state.contains("aws_eks_pod_identity_association"),
        "state missing resource type"
    );
    assert!(
        state.contains("my-service-account"),
        "state missing service account"
    );

    cleanup_tf_dir(&dir);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_eks_pod_identity_association_kube_system() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("eks-pod-id-ksys");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_eks_cluster" "eks_pid_ksys_cluster" {
  name     = "eks-pid-ksys-cluster"
  role_arn = "arn:aws:iam::123456789012:role/eks-cluster-role"

  vpc_config {
    subnet_ids = ["subnet-12345678", "subnet-87654321"]
  }
}

resource "aws_eks_pod_identity_association" "eks_pod_identity_association_kube_system" {
  cluster_name    = aws_eks_cluster.eks_pid_ksys_cluster.name
  namespace       = "kube-system"
  service_account = "aws-node"
  role_arn        = "arn:aws:iam::123456789012:role/aws-node-role"
}
"#,
    )
    .unwrap();
    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");

    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(
        state.contains("aws_eks_pod_identity_association"),
        "state missing resource type"
    );
    assert!(state.contains("aws-node"), "state missing service account");

    cleanup_tf_dir(&dir);
}
