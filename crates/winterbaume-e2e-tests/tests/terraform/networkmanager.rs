use crate::harness::*;

// Network Manager terraform resources tested here:
//   aws_networkmanager_global_network  (CreateGlobalNetwork, DescribeGlobalNetworks,
//                                       UpdateGlobalNetwork, DeleteGlobalNetwork,
//                                       TagResource, ListTagsForResource, UntagResource)
//   aws_networkmanager_site            (CreateSite, GetSites, UpdateSite, DeleteSite,
//                                       TagResource, ListTagsForResource)
//   aws_networkmanager_device          (CreateDevice, GetDevices, UpdateDevice, DeleteDevice,
//                                       TagResource, ListTagsForResource)
//   aws_networkmanager_link            (CreateLink, GetLinks, UpdateLink, DeleteLink,
//                                       TagResource, ListTagsForResource)
//   aws_networkmanager_connection      (CreateConnection, GetConnections, UpdateConnection,
//                                       DeleteConnection, TagResource, ListTagsForResource)
//   aws_networkmanager_core_network    (CreateCoreNetwork, GetCoreNetwork, UpdateCoreNetwork,
//                                       DeleteCoreNetwork, ListTagsForResource, TagResource)

// ---------------------------------------------------------------------------
// aws_networkmanager_global_network
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_networkmanager_global_network_basic() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("nm-global-network-basic");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_networkmanager_global_network" "nm_global_network_basic" {
  description = "test-global-network-basic"

  tags = {
    Name = "nm-global-network-basic"
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
        state.contains("aws_networkmanager_global_network"),
        "state missing resource type"
    );
    assert!(
        state.contains("test-global-network-basic"),
        "state missing description"
    );

    cleanup_tf_dir(&dir);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_networkmanager_global_network_no_tags() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("nm-global-network-notags");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_networkmanager_global_network" "nm_global_network_no_tags" {
  description = "global-network-no-tags"
}
"#,
    )
    .unwrap();
    terraform_init(&dir).await;
    let (ok, _stdout, stderr) = terraform_apply(&dir).await;
    assert!(ok, "terraform apply failed:\n{stderr}");

    let state = std::fs::read_to_string(dir.join("terraform.tfstate")).unwrap_or_default();
    assert!(
        state.contains("global-network-no-tags"),
        "state missing description"
    );

    cleanup_tf_dir(&dir);
}

// ---------------------------------------------------------------------------
// aws_networkmanager_site
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_networkmanager_site_basic() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("nm-site-basic");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_networkmanager_global_network" "nm_site_gn" {
  description = "gn-for-site-test"
}

resource "aws_networkmanager_site" "nm_site_basic" {
  global_network_id = aws_networkmanager_global_network.nm_site_gn.id
  description       = "test-site-basic"

  tags = {
    Name = "nm-site-basic"
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
        state.contains("aws_networkmanager_site"),
        "state missing resource type"
    );
    assert!(
        state.contains("test-site-basic"),
        "state missing description"
    );

    cleanup_tf_dir(&dir);
}

// ---------------------------------------------------------------------------
// aws_networkmanager_device
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_networkmanager_device_basic() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("nm-device-basic");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_networkmanager_global_network" "nm_device_gn" {
  description = "gn-for-device-test"
}

resource "aws_networkmanager_device" "nm_device_basic" {
  global_network_id = aws_networkmanager_global_network.nm_device_gn.id
  description       = "test-device-basic"

  tags = {
    Name = "nm-device-basic"
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
        state.contains("aws_networkmanager_device"),
        "state missing resource type"
    );
    assert!(
        state.contains("test-device-basic"),
        "state missing description"
    );

    cleanup_tf_dir(&dir);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_networkmanager_device_with_site() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("nm-device-with-site");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_networkmanager_global_network" "nm_device_site_gn" {
  description = "gn-for-device-site-test"
}

resource "aws_networkmanager_site" "nm_device_site" {
  global_network_id = aws_networkmanager_global_network.nm_device_site_gn.id
  description       = "site-for-device-test"
}

resource "aws_networkmanager_device" "nm_device_with_site" {
  global_network_id = aws_networkmanager_global_network.nm_device_site_gn.id
  site_id           = aws_networkmanager_site.nm_device_site.id
  description       = "device-with-site"

  tags = {
    Name = "nm-device-with-site"
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
        state.contains("device-with-site"),
        "state missing description"
    );

    cleanup_tf_dir(&dir);
}

// ---------------------------------------------------------------------------
// aws_networkmanager_link
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_networkmanager_link_basic() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("nm-link-basic");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_networkmanager_global_network" "nm_link_gn" {
  description = "gn-for-link-test"
}

resource "aws_networkmanager_site" "nm_link_site" {
  global_network_id = aws_networkmanager_global_network.nm_link_gn.id
  description       = "site-for-link-test"
}

resource "aws_networkmanager_link" "nm_link_basic" {
  global_network_id = aws_networkmanager_global_network.nm_link_gn.id
  site_id           = aws_networkmanager_site.nm_link_site.id
  description       = "test-link-basic"

  bandwidth {
    upload_speed   = 10
    download_speed = 50
  }

  tags = {
    Name = "nm-link-basic"
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
        state.contains("aws_networkmanager_link"),
        "state missing resource type"
    );
    assert!(
        state.contains("test-link-basic"),
        "state missing description"
    );

    cleanup_tf_dir(&dir);
}

// ---------------------------------------------------------------------------
// aws_networkmanager_connection
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_networkmanager_connection_basic() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("nm-connection-basic");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_networkmanager_global_network" "nm_conn_gn" {
  description = "gn-for-connection-test"
}

resource "aws_networkmanager_device" "nm_conn_device_a" {
  global_network_id = aws_networkmanager_global_network.nm_conn_gn.id
  description       = "device-a-for-connection"
}

resource "aws_networkmanager_device" "nm_conn_device_b" {
  global_network_id = aws_networkmanager_global_network.nm_conn_gn.id
  description       = "device-b-for-connection"
}

resource "aws_networkmanager_connection" "nm_connection_basic" {
  global_network_id   = aws_networkmanager_global_network.nm_conn_gn.id
  device_id           = aws_networkmanager_device.nm_conn_device_a.id
  connected_device_id = aws_networkmanager_device.nm_conn_device_b.id
  description         = "test-connection-basic"

  tags = {
    Name = "nm-connection-basic"
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
        state.contains("aws_networkmanager_connection"),
        "state missing resource type"
    );
    assert!(
        state.contains("test-connection-basic"),
        "state missing description"
    );

    cleanup_tf_dir(&dir);
}

// ---------------------------------------------------------------------------
// aws_networkmanager_core_network
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_networkmanager_core_network_basic() {
    let port = start_server(test_services()).await;
    let url = format!("http://127.0.0.1:{port}");
    let dir = create_tf_dir("nm-core-network-basic");
    write_provider_tf(&dir, &url);
    std::fs::write(
        dir.join("main.tf"),
        r#"
resource "aws_networkmanager_global_network" "nm_cn_gn" {
  description = "gn-for-core-network-test"
}

resource "aws_networkmanager_core_network" "nm_core_network_basic" {
  global_network_id = aws_networkmanager_global_network.nm_cn_gn.id
  description       = "test-core-network-basic"

  tags = {
    Name = "nm-core-network-basic"
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
        state.contains("aws_networkmanager_core_network"),
        "state missing resource type"
    );
    assert!(
        state.contains("test-core-network-basic"),
        "state missing description"
    );

    cleanup_tf_dir(&dir);
}
