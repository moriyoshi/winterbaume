use aws_sdk_ec2instanceconnect::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_ec2instanceconnect::Ec2InstanceConnectService;

async fn make_client() -> aws_sdk_ec2instanceconnect::Client {
    let mock = MockAws::builder()
        .with_service(Ec2InstanceConnectService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_ec2instanceconnect::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_ec2instanceconnect::Client::new(&config)
}

#[tokio::test]
async fn test_send_ssh_public_key() {
    let client = make_client().await;

    let resp = client
        .send_ssh_public_key()
        .instance_id("i-1234567890abcdef0")
        .instance_os_user("ec2-user")
        .ssh_public_key("ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAABAQ...")
        .send()
        .await
        .expect("send_ssh_public_key should succeed");

    assert!(resp.success());
    assert!(resp.request_id().is_some());
}

#[tokio::test]
async fn test_send_serial_console_ssh_public_key() {
    let client = make_client().await;

    let resp = client
        .send_serial_console_ssh_public_key()
        .instance_id("i-1234567890abcdef0")
        .ssh_public_key("ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAABAQ...")
        .send()
        .await
        .expect("send_serial_console_ssh_public_key should succeed");

    assert!(resp.success());
    assert!(resp.request_id().is_some());
}

#[tokio::test]
async fn test_send_ssh_public_key_with_availability_zone() {
    let client = make_client().await;

    let resp = client
        .send_ssh_public_key()
        .instance_id("i-1234567890abcdef0")
        .instance_os_user("ec2-user")
        .ssh_public_key("ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAABAQ...")
        .availability_zone("us-east-1a")
        .send()
        .await
        .expect("send_ssh_public_key with availability_zone should succeed");

    assert!(resp.success());
    assert!(resp.request_id().is_some());
}

#[tokio::test]
async fn test_send_ssh_public_key_returns_unique_request_ids() {
    let client = make_client().await;

    let resp1 = client
        .send_ssh_public_key()
        .instance_id("i-1234567890abcdef0")
        .instance_os_user("ec2-user")
        .ssh_public_key("ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAABAQ...")
        .send()
        .await
        .expect("first send_ssh_public_key should succeed");

    let resp2 = client
        .send_ssh_public_key()
        .instance_id("i-1234567890abcdef0")
        .instance_os_user("ec2-user")
        .ssh_public_key("ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAABAQ...")
        .send()
        .await
        .expect("second send_ssh_public_key should succeed");

    let id1 = resp1
        .request_id()
        .expect("first request_id should be present");
    let id2 = resp2
        .request_id()
        .expect("second request_id should be present");
    assert_ne!(id1, id2, "each call should return a distinct request_id");
}

#[tokio::test]
async fn test_send_ssh_public_key_different_os_users() {
    let client = make_client().await;

    for user in ["ec2-user", "ubuntu", "admin", "root"] {
        let resp = client
            .send_ssh_public_key()
            .instance_id("i-1234567890abcdef0")
            .instance_os_user(user)
            .ssh_public_key("ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAABAQ...")
            .send()
            .await
            .unwrap_or_else(|e| {
                panic!("send_ssh_public_key for user '{user}' should succeed: {e}")
            });

        assert!(
            resp.success(),
            "success should be true for os_user '{user}'"
        );
    }
}

#[tokio::test]
async fn test_send_serial_console_ssh_public_key_with_serial_port_zero() {
    let client = make_client().await;

    let resp = client
        .send_serial_console_ssh_public_key()
        .instance_id("i-1234567890abcdef0")
        .ssh_public_key("ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAABAQ...")
        .serial_port(0)
        .send()
        .await
        .expect("send_serial_console_ssh_public_key with serial_port=0 should succeed");

    assert!(resp.success());
    assert!(resp.request_id().is_some());
}

#[tokio::test]
async fn test_send_serial_console_ssh_public_key_returns_unique_request_ids() {
    let client = make_client().await;

    let resp1 = client
        .send_serial_console_ssh_public_key()
        .instance_id("i-1234567890abcdef0")
        .ssh_public_key("ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAABAQ...")
        .send()
        .await
        .expect("first send_serial_console_ssh_public_key should succeed");

    let resp2 = client
        .send_serial_console_ssh_public_key()
        .instance_id("i-1234567890abcdef0")
        .ssh_public_key("ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAABAQ...")
        .send()
        .await
        .expect("second send_serial_console_ssh_public_key should succeed");

    let id1 = resp1
        .request_id()
        .expect("first request_id should be present");
    let id2 = resp2
        .request_id()
        .expect("second request_id should be present");
    assert_ne!(id1, id2, "each call should return a distinct request_id");
}

#[tokio::test]
async fn test_send_ssh_public_key_ed25519_key_type() {
    let client = make_client().await;

    // EC2 Instance Connect supports ED25519 keys in addition to RSA
    let ed25519_key = "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIOMqqnkVzrm0SdG6UOoqKLsabgH5C9okWi0dh2l9GKJl test@example.com";

    let resp = client
        .send_ssh_public_key()
        .instance_id("i-abcdef1234567890a")
        .instance_os_user("ubuntu")
        .ssh_public_key(ed25519_key)
        .send()
        .await
        .expect("send_ssh_public_key with ED25519 key should succeed");

    assert!(resp.success());
    assert!(resp.request_id().is_some());
}

#[tokio::test]
async fn test_send_serial_console_ssh_public_key_ed25519_key_type() {
    let client = make_client().await;

    let ed25519_key = "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIOMqqnkVzrm0SdG6UOoqKLsabgH5C9okWi0dh2l9GKJl test@example.com";

    let resp = client
        .send_serial_console_ssh_public_key()
        .instance_id("i-abcdef1234567890a")
        .ssh_public_key(ed25519_key)
        .send()
        .await
        .expect("send_serial_console_ssh_public_key with ED25519 key should succeed");

    assert!(resp.success());
    assert!(resp.request_id().is_some());
}

// ============================================================================
// Tests derived from AWS documentation: EC2 Instance Connect
// Validation / error-path tests and additional coverage
// ============================================================================

#[tokio::test]
async fn test_send_ssh_public_key_missing_instance_id() {
    // InstanceId is a required field; omitting it should produce a build-time or
    // service-level error rather than a successful response.
    let client = make_client().await;

    let err = client
        .send_ssh_public_key()
        // instance_id intentionally omitted
        .instance_os_user("ec2-user")
        .ssh_public_key("ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAABAQ...")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("InvalidArgsException")
            || err_str.contains("BuildError")
            || err_str.contains("instance_id"),
        "Expected missing-InstanceId error, got: {err_str}"
    );
}

#[tokio::test]
async fn test_send_ssh_public_key_missing_os_user() {
    let client = make_client().await;

    let err = client
        .send_ssh_public_key()
        .instance_id("i-1234567890abcdef0")
        // instance_os_user intentionally omitted
        .ssh_public_key("ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAABAQ...")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("InvalidArgsException")
            || err_str.contains("BuildError")
            || err_str.contains("instance_os_user"),
        "Expected missing-InstanceOSUser error, got: {err_str}"
    );
}

#[tokio::test]
async fn test_send_ssh_public_key_missing_ssh_key() {
    let client = make_client().await;

    let err = client
        .send_ssh_public_key()
        .instance_id("i-1234567890abcdef0")
        .instance_os_user("ec2-user")
        // ssh_public_key intentionally omitted
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("InvalidArgsException")
            || err_str.contains("BuildError")
            || err_str.contains("ssh_public_key"),
        "Expected missing-SSHPublicKey error, got: {err_str}"
    );
}

#[tokio::test]
async fn test_send_serial_console_ssh_public_key_missing_instance_id() {
    let client = make_client().await;

    let err = client
        .send_serial_console_ssh_public_key()
        // instance_id intentionally omitted
        .ssh_public_key("ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAABAQ...")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("InvalidArgsException")
            || err_str.contains("BuildError")
            || err_str.contains("instance_id"),
        "Expected missing-InstanceId error, got: {err_str}"
    );
}

#[tokio::test]
async fn test_send_serial_console_ssh_public_key_missing_ssh_key() {
    let client = make_client().await;

    let err = client
        .send_serial_console_ssh_public_key()
        .instance_id("i-1234567890abcdef0")
        // ssh_public_key intentionally omitted
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("InvalidArgsException")
            || err_str.contains("BuildError")
            || err_str.contains("ssh_public_key"),
        "Expected missing-SSHPublicKey error, got: {err_str}"
    );
}

#[tokio::test]
async fn test_send_ssh_public_key_different_instance_ids() {
    let client = make_client().await;

    // Instance IDs follow pattern ^i-[a-f0-9]+$, length 10-32.
    // Test a variety of valid formats including the common 19-char form
    // and shorter valid forms.
    let instance_ids = [
        "i-1234567890abcdef0", // 19 chars -- common modern format
        "i-abcdef12",          // 10 chars -- minimum length
        "i-00000000",          // minimum with zeroes
        "i-ffffffffffff",      // all f's
    ];

    for instance_id in instance_ids {
        let resp = client
            .send_ssh_public_key()
            .instance_id(instance_id)
            .instance_os_user("ec2-user")
            .ssh_public_key("ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAABAQ...")
            .send()
            .await
            .unwrap_or_else(|e| {
                panic!("send_ssh_public_key for instance_id '{instance_id}' should succeed: {e}")
            });

        assert!(
            resp.success(),
            "success should be true for instance_id '{instance_id}'"
        );
        assert!(
            resp.request_id().is_some(),
            "request_id should be present for instance_id '{instance_id}'"
        );
    }
}
