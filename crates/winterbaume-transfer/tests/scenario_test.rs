//! Smoke tests for winterbaume Transfer Family service — realistic application scenarios.
//!
//! Each test simulates a coherent end-to-end workflow rather than exercising
//! a single API call in isolation.

use aws_sdk_transfer::config::BehaviorVersion;
use aws_sdk_transfer::types::{CertificateUsageType, ProfileType};
use winterbaume_core::MockAws;
use winterbaume_transfer::TransferService;

async fn make_transfer_client() -> aws_sdk_transfer::Client {
    let mock = MockAws::builder()
        .with_service(TransferService::new())
        .build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_transfer::config::Region::new("us-east-1"))
        .load()
        .await;
    aws_sdk_transfer::Client::new(&config)
}

/// Scenario: SFTP server provisioning + SSH key lifecycle.
///
/// A platform engineer brings up a new SFTP endpoint, provisions a user
/// for it, attaches the user's SSH public key, verifies the key is visible,
/// then performs a clean teardown — first removing the key, then the user,
/// then the server.
#[tokio::test]
async fn test_server_user_ssh_key_lifecycle() {
    let client = make_transfer_client().await;

    // 1. Provision the SFTP server.
    let server = client.create_server().send().await.expect("create_server");
    let server_id = server.server_id().to_string();
    assert!(server_id.starts_with("s-"));

    // 2. Create a user on the server.
    client
        .create_user()
        .server_id(&server_id)
        .user_name("alice")
        .role("arn:aws:iam::123456789012:role/sftp-user")
        .home_directory("/orders-bucket/home/alice")
        .send()
        .await
        .expect("create_user");

    // 3. Import an SSH public key for the user.
    let key = client
        .import_ssh_public_key()
        .server_id(&server_id)
        .user_name("alice")
        .ssh_public_key_body("ssh-rsa AAAAB3Nzfake alice@example.com")
        .send()
        .await
        .expect("import_ssh_public_key");
    let key_id = key.ssh_public_key_id().to_string();
    assert!(key_id.starts_with("key-"));

    // 4. DescribeUser must reflect the imported key.
    let desc = client
        .describe_user()
        .server_id(&server_id)
        .user_name("alice")
        .send()
        .await
        .expect("describe_user");
    let user = desc.user().expect("user present");
    assert_eq!(user.user_name(), Some("alice"));
    assert_eq!(user.ssh_public_keys().len(), 1);
    assert_eq!(user.ssh_public_keys()[0].ssh_public_key_id(), key_id);

    // 5. Tear down — key, then user, then server.
    client
        .delete_ssh_public_key()
        .server_id(&server_id)
        .user_name("alice")
        .ssh_public_key_id(&key_id)
        .send()
        .await
        .expect("delete_ssh_public_key");

    let desc_after = client
        .describe_user()
        .server_id(&server_id)
        .user_name("alice")
        .send()
        .await
        .expect("describe_user after key delete");
    assert!(
        desc_after.user().unwrap().ssh_public_keys().is_empty(),
        "user should have no SSH keys after delete"
    );

    client
        .delete_user()
        .server_id(&server_id)
        .user_name("alice")
        .send()
        .await
        .expect("delete_user");

    client
        .delete_server()
        .server_id(&server_id)
        .send()
        .await
        .expect("delete_server");

    // Server is gone — describe must fail.
    let after = client.describe_server().server_id(&server_id).send().await;
    assert!(after.is_err(), "server should be gone after delete");
}

/// Scenario: AS2 partner agreement chain.
///
/// A B2B integrator wires up an AS2 trading partnership: imports a
/// signing certificate, creates LOCAL and PARTNER profiles, then ties
/// them together via a server agreement. The agreement must describe
/// successfully and tear down cleanly along with both profiles.
#[tokio::test]
async fn test_as2_profile_agreement_chain() {
    let client = make_transfer_client().await;

    // The agreement requires a server.
    let server = client.create_server().send().await.expect("create_server");
    let server_id = server.server_id().to_string();

    // LOCAL profile (this side).
    let local = client
        .create_profile()
        .profile_type(ProfileType::Local)
        .as2_id("ACME-LOCAL")
        .send()
        .await
        .expect("create_profile LOCAL");
    let local_id = local.profile_id().to_string();
    assert!(local_id.starts_with("p-"));

    // PARTNER profile (counterparty).
    let partner = client
        .create_profile()
        .profile_type(ProfileType::Partner)
        .as2_id("BIZCORP-PARTNER")
        .send()
        .await
        .expect("create_profile PARTNER");
    let partner_id = partner.profile_id().to_string();

    // Import a signing certificate for the agreement.
    let cert = client
        .import_certificate()
        .usage(CertificateUsageType::Signing)
        .certificate("-----BEGIN CERTIFICATE-----\nMIIBdummy\n-----END CERTIFICATE-----")
        .send()
        .await
        .expect("import_certificate");
    let cert_id = cert.certificate_id().to_string();
    assert!(cert_id.starts_with("cert-"));

    // Create the agreement linking the two profiles.
    let agreement = client
        .create_agreement()
        .server_id(&server_id)
        .local_profile_id(&local_id)
        .partner_profile_id(&partner_id)
        .base_directory("/as2-bucket/inbox")
        .access_role("arn:aws:iam::123456789012:role/as2-agreement")
        .send()
        .await
        .expect("create_agreement");
    let agreement_id = agreement.agreement_id().to_string();
    assert!(agreement_id.starts_with("a-"));

    // DescribeAgreement must reflect the linkage.
    let desc = client
        .describe_agreement()
        .agreement_id(&agreement_id)
        .server_id(&server_id)
        .send()
        .await
        .expect("describe_agreement");
    let a = desc.agreement().expect("agreement present");
    assert_eq!(a.agreement_id(), Some(agreement_id.as_str()));
    assert_eq!(a.local_profile_id(), Some(local_id.as_str()));
    assert_eq!(a.partner_profile_id(), Some(partner_id.as_str()));
    assert_eq!(a.base_directory(), Some("/as2-bucket/inbox"));

    // Tear down — agreement first, then both profiles.
    client
        .delete_agreement()
        .agreement_id(&agreement_id)
        .server_id(&server_id)
        .send()
        .await
        .expect("delete_agreement");

    let after = client
        .describe_agreement()
        .agreement_id(&agreement_id)
        .server_id(&server_id)
        .send()
        .await;
    assert!(after.is_err(), "agreement should be gone");

    client
        .delete_profile()
        .profile_id(&local_id)
        .send()
        .await
        .expect("delete_profile LOCAL");
    client
        .delete_profile()
        .profile_id(&partner_id)
        .send()
        .await
        .expect("delete_profile PARTNER");

    // Cert + server cleanup (best-effort housekeeping).
    client
        .delete_certificate()
        .certificate_id(&cert_id)
        .send()
        .await
        .expect("delete_certificate");
    client
        .delete_server()
        .server_id(&server_id)
        .send()
        .await
        .expect("delete_server");
}
