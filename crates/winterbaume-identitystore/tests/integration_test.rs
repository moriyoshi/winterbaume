use aws_sdk_identitystore::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_identitystore::IdentityStoreService;

const IDENTITY_STORE_ID: &str = "d-1234567890";

async fn make_identitystore_client() -> aws_sdk_identitystore::Client {
    let mock = MockAws::builder()
        .with_service(IdentityStoreService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_identitystore::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_identitystore::Client::new(&config)
}

#[tokio::test]
async fn test_create_user() {
    let client = make_identitystore_client().await;

    let resp = client
        .create_user()
        .identity_store_id(IDENTITY_STORE_ID)
        .user_name("johndoe")
        .display_name("John Doe")
        .name(
            aws_sdk_identitystore::types::Name::builder()
                .given_name("John")
                .family_name("Doe")
                .build(),
        )
        .send()
        .await
        .expect("create_user should succeed");

    assert_eq!(resp.identity_store_id(), IDENTITY_STORE_ID);
    assert!(!resp.user_id().is_empty());
}

#[tokio::test]
async fn test_describe_user() {
    let client = make_identitystore_client().await;

    let create_resp = client
        .create_user()
        .identity_store_id(IDENTITY_STORE_ID)
        .user_name("janedoe")
        .display_name("Jane Doe")
        .name(
            aws_sdk_identitystore::types::Name::builder()
                .given_name("Jane")
                .family_name("Doe")
                .build(),
        )
        .send()
        .await
        .unwrap();

    let user_id = create_resp.user_id().to_string();

    let desc = client
        .describe_user()
        .identity_store_id(IDENTITY_STORE_ID)
        .user_id(&user_id)
        .send()
        .await
        .expect("describe_user should succeed");

    assert_eq!(desc.identity_store_id(), IDENTITY_STORE_ID);
    assert_eq!(desc.user_id(), user_id);
    assert_eq!(desc.user_name(), Some("janedoe"));
    assert_eq!(desc.display_name(), Some("Jane Doe"));

    let name = desc.name().expect("should have name");
    assert_eq!(name.given_name(), Some("Jane"));
    assert_eq!(name.family_name(), Some("Doe"));
}

#[tokio::test]
async fn test_delete_user() {
    let client = make_identitystore_client().await;

    let create_resp = client
        .create_user()
        .identity_store_id(IDENTITY_STORE_ID)
        .user_name("deleteme")
        .send()
        .await
        .unwrap();

    let user_id = create_resp.user_id().to_string();

    client
        .delete_user()
        .identity_store_id(IDENTITY_STORE_ID)
        .user_id(&user_id)
        .send()
        .await
        .expect("delete_user should succeed");

    // Describe should fail after deletion.
    let result = client
        .describe_user()
        .identity_store_id(IDENTITY_STORE_ID)
        .user_id(&user_id)
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_delete_nonexistent_user() {
    let client = make_identitystore_client().await;

    let result = client
        .delete_user()
        .identity_store_id(IDENTITY_STORE_ID)
        .user_id("nonexistent-user-id")
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_users() {
    let client = make_identitystore_client().await;

    for name in ["alice", "bob", "charlie"] {
        client
            .create_user()
            .identity_store_id(IDENTITY_STORE_ID)
            .user_name(name)
            .display_name(name)
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_users()
        .identity_store_id(IDENTITY_STORE_ID)
        .send()
        .await
        .expect("list_users should succeed");

    assert_eq!(resp.users().len(), 3);
}

// --- Group tests ---

#[tokio::test]
async fn test_create_group() {
    let client = make_identitystore_client().await;

    let resp = client
        .create_group()
        .identity_store_id(IDENTITY_STORE_ID)
        .display_name("Engineering")
        .description("Engineering team")
        .send()
        .await
        .expect("create_group should succeed");

    assert_eq!(resp.identity_store_id(), IDENTITY_STORE_ID);
    assert!(!resp.group_id().is_empty());
}

#[tokio::test]
async fn test_describe_group() {
    let client = make_identitystore_client().await;

    let create_resp = client
        .create_group()
        .identity_store_id(IDENTITY_STORE_ID)
        .display_name("Design")
        .description("Design team")
        .send()
        .await
        .unwrap();

    let group_id = create_resp.group_id().to_string();

    let desc = client
        .describe_group()
        .identity_store_id(IDENTITY_STORE_ID)
        .group_id(&group_id)
        .send()
        .await
        .expect("describe_group should succeed");

    assert_eq!(desc.identity_store_id(), IDENTITY_STORE_ID);
    assert_eq!(desc.group_id(), group_id);
    assert_eq!(desc.display_name(), Some("Design"));
    assert_eq!(desc.description(), Some("Design team"));
}

#[tokio::test]
async fn test_delete_group() {
    let client = make_identitystore_client().await;

    let create_resp = client
        .create_group()
        .identity_store_id(IDENTITY_STORE_ID)
        .display_name("Temp Group")
        .send()
        .await
        .unwrap();

    let group_id = create_resp.group_id().to_string();

    client
        .delete_group()
        .identity_store_id(IDENTITY_STORE_ID)
        .group_id(&group_id)
        .send()
        .await
        .expect("delete_group should succeed");

    // Describe should fail after deletion.
    let result = client
        .describe_group()
        .identity_store_id(IDENTITY_STORE_ID)
        .group_id(&group_id)
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_groups() {
    let client = make_identitystore_client().await;

    for name in ["Alpha", "Beta", "Gamma"] {
        client
            .create_group()
            .identity_store_id(IDENTITY_STORE_ID)
            .display_name(name)
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_groups()
        .identity_store_id(IDENTITY_STORE_ID)
        .send()
        .await
        .expect("list_groups should succeed");

    assert_eq!(resp.groups().len(), 3);
}

#[tokio::test]
async fn test_get_group_id() {
    let client = make_identitystore_client().await;

    let create_resp = client
        .create_group()
        .identity_store_id(IDENTITY_STORE_ID)
        .display_name("LookupGroup")
        .send()
        .await
        .unwrap();

    let expected_group_id = create_resp.group_id().to_string();

    let resp = client
        .get_group_id()
        .identity_store_id(IDENTITY_STORE_ID)
        .alternate_identifier(
            aws_sdk_identitystore::types::AlternateIdentifier::UniqueAttribute(
                aws_sdk_identitystore::types::UniqueAttribute::builder()
                    .attribute_path("displayName")
                    .attribute_value(aws_smithy_types::Document::String(
                        "LookupGroup".to_string(),
                    ))
                    .build()
                    .unwrap(),
            ),
        )
        .send()
        .await
        .expect("get_group_id should succeed");

    assert_eq!(resp.group_id(), expected_group_id);
    assert_eq!(resp.identity_store_id(), IDENTITY_STORE_ID);
}

#[tokio::test]
async fn test_get_user_id() {
    let client = make_identitystore_client().await;

    let create_resp = client
        .create_user()
        .identity_store_id(IDENTITY_STORE_ID)
        .user_name("lookupuser")
        .send()
        .await
        .unwrap();

    let expected_user_id = create_resp.user_id().to_string();

    let resp = client
        .get_user_id()
        .identity_store_id(IDENTITY_STORE_ID)
        .alternate_identifier(
            aws_sdk_identitystore::types::AlternateIdentifier::UniqueAttribute(
                aws_sdk_identitystore::types::UniqueAttribute::builder()
                    .attribute_path("userName")
                    .attribute_value(aws_smithy_types::Document::String("lookupuser".to_string()))
                    .build()
                    .unwrap(),
            ),
        )
        .send()
        .await
        .expect("get_user_id should succeed");

    assert_eq!(resp.user_id(), expected_user_id);
    assert_eq!(resp.identity_store_id(), IDENTITY_STORE_ID);
}

// --- Group membership tests ---

#[tokio::test]
async fn test_create_group_membership() {
    let client = make_identitystore_client().await;

    let group = client
        .create_group()
        .identity_store_id(IDENTITY_STORE_ID)
        .display_name("MembershipGroup")
        .send()
        .await
        .unwrap();

    let user = client
        .create_user()
        .identity_store_id(IDENTITY_STORE_ID)
        .user_name("memberuser")
        .send()
        .await
        .unwrap();

    let resp = client
        .create_group_membership()
        .identity_store_id(IDENTITY_STORE_ID)
        .group_id(group.group_id())
        .member_id(aws_sdk_identitystore::types::MemberId::UserId(
            user.user_id().to_string(),
        ))
        .send()
        .await
        .expect("create_group_membership should succeed");

    assert_eq!(resp.identity_store_id(), IDENTITY_STORE_ID);
    assert!(!resp.membership_id().is_empty());
}

#[tokio::test]
async fn test_delete_group_membership() {
    let client = make_identitystore_client().await;

    let group = client
        .create_group()
        .identity_store_id(IDENTITY_STORE_ID)
        .display_name("DeleteMemberGroup")
        .send()
        .await
        .unwrap();

    let user = client
        .create_user()
        .identity_store_id(IDENTITY_STORE_ID)
        .user_name("deletemember")
        .send()
        .await
        .unwrap();

    let membership = client
        .create_group_membership()
        .identity_store_id(IDENTITY_STORE_ID)
        .group_id(group.group_id())
        .member_id(aws_sdk_identitystore::types::MemberId::UserId(
            user.user_id().to_string(),
        ))
        .send()
        .await
        .unwrap();

    client
        .delete_group_membership()
        .identity_store_id(IDENTITY_STORE_ID)
        .membership_id(membership.membership_id())
        .send()
        .await
        .expect("delete_group_membership should succeed");
}

#[tokio::test]
async fn test_list_group_memberships() {
    let client = make_identitystore_client().await;

    let group = client
        .create_group()
        .identity_store_id(IDENTITY_STORE_ID)
        .display_name("ListMembersGroup")
        .send()
        .await
        .unwrap();

    for name in ["user_a", "user_b"] {
        let user = client
            .create_user()
            .identity_store_id(IDENTITY_STORE_ID)
            .user_name(name)
            .send()
            .await
            .unwrap();

        client
            .create_group_membership()
            .identity_store_id(IDENTITY_STORE_ID)
            .group_id(group.group_id())
            .member_id(aws_sdk_identitystore::types::MemberId::UserId(
                user.user_id().to_string(),
            ))
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_group_memberships()
        .identity_store_id(IDENTITY_STORE_ID)
        .group_id(group.group_id())
        .send()
        .await
        .expect("list_group_memberships should succeed");

    assert_eq!(resp.group_memberships().len(), 2);
}

#[tokio::test]
async fn test_list_group_memberships_for_member() {
    let client = make_identitystore_client().await;

    let user = client
        .create_user()
        .identity_store_id(IDENTITY_STORE_ID)
        .user_name("multi_group_user")
        .send()
        .await
        .unwrap();

    for name in ["GroupX", "GroupY"] {
        let group = client
            .create_group()
            .identity_store_id(IDENTITY_STORE_ID)
            .display_name(name)
            .send()
            .await
            .unwrap();

        client
            .create_group_membership()
            .identity_store_id(IDENTITY_STORE_ID)
            .group_id(group.group_id())
            .member_id(aws_sdk_identitystore::types::MemberId::UserId(
                user.user_id().to_string(),
            ))
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_group_memberships_for_member()
        .identity_store_id(IDENTITY_STORE_ID)
        .member_id(aws_sdk_identitystore::types::MemberId::UserId(
            user.user_id().to_string(),
        ))
        .send()
        .await
        .expect("list_group_memberships_for_member should succeed");

    assert_eq!(resp.group_memberships().len(), 2);
}

#[tokio::test]
async fn test_describe_user_not_found() {
    let client = make_identitystore_client().await;

    let result = client
        .describe_user()
        .identity_store_id(IDENTITY_STORE_ID)
        .user_id("nonexistent-user-id")
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_describe_group_not_found() {
    let client = make_identitystore_client().await;

    let result = client
        .describe_group()
        .identity_store_id(IDENTITY_STORE_ID)
        .group_id("nonexistent-group-id")
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_delete_nonexistent_group() {
    let client = make_identitystore_client().await;

    let result = client
        .delete_group()
        .identity_store_id(IDENTITY_STORE_ID)
        .group_id("nonexistent-group-id")
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_create_user_with_full_profile() {
    let client = make_identitystore_client().await;

    let resp = client
        .create_user()
        .identity_store_id(IDENTITY_STORE_ID)
        .user_name("fullprofile")
        .display_name("Full Profile User")
        .name(
            aws_sdk_identitystore::types::Name::builder()
                .given_name("Full")
                .family_name("Profile")
                .middle_name("A")
                .honorific_prefix("Dr.")
                .honorific_suffix("Jr.")
                .build(),
        )
        .emails(
            aws_sdk_identitystore::types::Email::builder()
                .value("full@example.com")
                .r#type("work")
                .primary(true)
                .build(),
        )
        .phone_numbers(
            aws_sdk_identitystore::types::PhoneNumber::builder()
                .value("+1-555-0100")
                .r#type("work")
                .primary(true)
                .build(),
        )
        .title("Engineer")
        .nick_name("Fully")
        .preferred_language("en-US")
        .locale("en_US")
        .timezone("America/New_York")
        .send()
        .await
        .expect("create_user with full profile should succeed");

    assert_eq!(resp.identity_store_id(), IDENTITY_STORE_ID);
    assert!(!resp.user_id().is_empty());

    let desc = client
        .describe_user()
        .identity_store_id(IDENTITY_STORE_ID)
        .user_id(resp.user_id())
        .send()
        .await
        .expect("describe_user should succeed");

    assert_eq!(desc.user_name(), Some("fullprofile"));
    assert_eq!(desc.display_name(), Some("Full Profile User"));
    assert_eq!(desc.title(), Some("Engineer"));
    assert_eq!(desc.nick_name(), Some("Fully"));
    assert_eq!(desc.preferred_language(), Some("en-US"));
    assert_eq!(desc.locale(), Some("en_US"));
    assert_eq!(desc.timezone(), Some("America/New_York"));

    let name = desc.name().expect("should have name");
    assert_eq!(name.given_name(), Some("Full"));
    assert_eq!(name.family_name(), Some("Profile"));
    assert_eq!(name.middle_name(), Some("A"));
    assert_eq!(name.honorific_prefix(), Some("Dr."));
    assert_eq!(name.honorific_suffix(), Some("Jr."));

    assert_eq!(desc.emails().len(), 1);
    assert_eq!(desc.emails()[0].value(), Some("full@example.com"));
    assert_eq!(desc.emails()[0].r#type(), Some("work"));
    assert!(desc.emails()[0].primary());

    assert_eq!(desc.phone_numbers().len(), 1);
    assert_eq!(desc.phone_numbers()[0].value(), Some("+1-555-0100"));
}

#[tokio::test]
async fn test_duplicate_group_membership_fails() {
    let client = make_identitystore_client().await;

    let group = client
        .create_group()
        .identity_store_id(IDENTITY_STORE_ID)
        .display_name("DuplicateMemberGroup")
        .send()
        .await
        .unwrap();

    let user = client
        .create_user()
        .identity_store_id(IDENTITY_STORE_ID)
        .user_name("dupuser")
        .send()
        .await
        .unwrap();

    client
        .create_group_membership()
        .identity_store_id(IDENTITY_STORE_ID)
        .group_id(group.group_id())
        .member_id(aws_sdk_identitystore::types::MemberId::UserId(
            user.user_id().to_string(),
        ))
        .send()
        .await
        .expect("first membership should succeed");

    let result = client
        .create_group_membership()
        .identity_store_id(IDENTITY_STORE_ID)
        .group_id(group.group_id())
        .member_id(aws_sdk_identitystore::types::MemberId::UserId(
            user.user_id().to_string(),
        ))
        .send()
        .await;

    assert!(result.is_err(), "duplicate membership should fail");
}

#[tokio::test]
async fn test_delete_group_membership_nonexistent() {
    let client = make_identitystore_client().await;

    let result = client
        .delete_group_membership()
        .identity_store_id(IDENTITY_STORE_ID)
        .membership_id("nonexistent-membership-id")
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_users_empty() {
    let client = make_identitystore_client().await;

    let resp = client
        .list_users()
        .identity_store_id(IDENTITY_STORE_ID)
        .send()
        .await
        .expect("list_users on empty store should succeed");

    assert_eq!(resp.users().len(), 0);
}

#[tokio::test]
async fn test_list_groups_empty() {
    let client = make_identitystore_client().await;

    let resp = client
        .list_groups()
        .identity_store_id(IDENTITY_STORE_ID)
        .send()
        .await
        .expect("list_groups on empty store should succeed");

    assert_eq!(resp.groups().len(), 0);
}

#[tokio::test]
async fn test_list_group_memberships_empty_group() {
    let client = make_identitystore_client().await;

    let group = client
        .create_group()
        .identity_store_id(IDENTITY_STORE_ID)
        .display_name("EmptyGroup")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_group_memberships()
        .identity_store_id(IDENTITY_STORE_ID)
        .group_id(group.group_id())
        .send()
        .await
        .expect("list_group_memberships on empty group should succeed");

    assert_eq!(resp.group_memberships().len(), 0);
}

#[tokio::test]
async fn test_delete_group_removes_memberships() {
    let client = make_identitystore_client().await;

    let group = client
        .create_group()
        .identity_store_id(IDENTITY_STORE_ID)
        .display_name("GroupToDeleteWithMembers")
        .send()
        .await
        .unwrap();

    let user = client
        .create_user()
        .identity_store_id(IDENTITY_STORE_ID)
        .user_name("member_of_doomed_group")
        .send()
        .await
        .unwrap();

    client
        .create_group_membership()
        .identity_store_id(IDENTITY_STORE_ID)
        .group_id(group.group_id())
        .member_id(aws_sdk_identitystore::types::MemberId::UserId(
            user.user_id().to_string(),
        ))
        .send()
        .await
        .unwrap();

    client
        .delete_group()
        .identity_store_id(IDENTITY_STORE_ID)
        .group_id(group.group_id())
        .send()
        .await
        .expect("delete_group should succeed");

    let resp = client
        .list_group_memberships_for_member()
        .identity_store_id(IDENTITY_STORE_ID)
        .member_id(aws_sdk_identitystore::types::MemberId::UserId(
            user.user_id().to_string(),
        ))
        .send()
        .await
        .expect("list_group_memberships_for_member should succeed");

    assert_eq!(
        resp.group_memberships().len(),
        0,
        "memberships for deleted group should be removed"
    );
}

#[tokio::test]
async fn test_get_user_id_not_found() {
    let client = make_identitystore_client().await;

    let result = client
        .get_user_id()
        .identity_store_id(IDENTITY_STORE_ID)
        .alternate_identifier(
            aws_sdk_identitystore::types::AlternateIdentifier::UniqueAttribute(
                aws_sdk_identitystore::types::UniqueAttribute::builder()
                    .attribute_path("userName")
                    .attribute_value(aws_smithy_types::Document::String(
                        "no_such_user".to_string(),
                    ))
                    .build()
                    .unwrap(),
            ),
        )
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_group_id_not_found() {
    let client = make_identitystore_client().await;

    let result = client
        .get_group_id()
        .identity_store_id(IDENTITY_STORE_ID)
        .alternate_identifier(
            aws_sdk_identitystore::types::AlternateIdentifier::UniqueAttribute(
                aws_sdk_identitystore::types::UniqueAttribute::builder()
                    .attribute_path("displayName")
                    .attribute_value(aws_smithy_types::Document::String(
                        "NoSuchGroup".to_string(),
                    ))
                    .build()
                    .unwrap(),
            ),
        )
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_create_group_membership_nonexistent_group_fails() {
    let client = make_identitystore_client().await;

    let user = client
        .create_user()
        .identity_store_id(IDENTITY_STORE_ID)
        .user_name("orphan_user")
        .send()
        .await
        .unwrap();

    let result = client
        .create_group_membership()
        .identity_store_id(IDENTITY_STORE_ID)
        .group_id("nonexistent-group-id")
        .member_id(aws_sdk_identitystore::types::MemberId::UserId(
            user.user_id().to_string(),
        ))
        .send()
        .await;

    assert!(
        result.is_err(),
        "membership for nonexistent group should fail"
    );
}
