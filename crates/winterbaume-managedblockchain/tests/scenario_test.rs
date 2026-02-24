use aws_sdk_managedblockchain::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_managedblockchain::ManagedBlockchainService;

async fn make_mb_client() -> aws_sdk_managedblockchain::Client {
    let mock = MockAws::builder()
        .with_service(ManagedBlockchainService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_managedblockchain::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_managedblockchain::Client::new(&config)
}

/// Scenario: Blockchain Network Membership Expansion
///
/// A consortium administrator creates a network, raises a governance proposal to
/// invite a new member account, votes to approve the proposal, and a new member
/// joins via the resulting invitation — demonstrating the full membership
/// lifecycle: network creation → proposal → voting → invitation → member join.
#[tokio::test]
async fn test_membership_expansion_scenario() {
    let client = make_mb_client().await;

    // Step 1: Create the initial network (returns network_id + first member_id).
    let create_resp = client
        .create_network()
        .name("consortium-network")
        .description("A Hyperledger Fabric consortium")
        .framework(aws_sdk_managedblockchain::types::Framework::HyperledgerFabric)
        .framework_version("1.4")
        .voting_policy(aws_sdk_managedblockchain::types::VotingPolicy::builder().build())
        .member_configuration(
            aws_sdk_managedblockchain::types::MemberConfiguration::builder()
                .name("founding-member")
                .framework_configuration(
                    aws_sdk_managedblockchain::types::MemberFrameworkConfiguration::builder()
                        .build(),
                )
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("create_network should succeed");

    let network_id = create_resp.network_id().unwrap().to_string();
    let founding_member_id = create_resp.member_id().unwrap().to_string();

    // Step 2: Verify the network and its founding member exist.
    let net_resp = client
        .get_network()
        .network_id(&network_id)
        .send()
        .await
        .expect("get_network should succeed");
    assert_eq!(
        net_resp.network().unwrap().status(),
        Some(&aws_sdk_managedblockchain::types::NetworkStatus::Available)
    );

    let members_resp = client
        .list_members()
        .network_id(&network_id)
        .send()
        .await
        .expect("list_members should succeed");
    assert_eq!(members_resp.members().len(), 1);

    // Step 3: Create a proposal to invite a new account (principal "999999999999").
    let proposal_resp = client
        .create_proposal()
        .network_id(&network_id)
        .member_id(&founding_member_id)
        .description("Invite account 999999999999 as a member")
        .actions(
            aws_sdk_managedblockchain::types::ProposalActions::builder()
                .invitations(
                    aws_sdk_managedblockchain::types::InviteAction::builder()
                        .principal("999999999999")
                        .build()
                        .unwrap(),
                )
                .build(),
        )
        .send()
        .await
        .expect("create_proposal should succeed");

    let proposal_id = proposal_resp.proposal_id().unwrap().to_string();

    // Step 4: Verify proposal is IN_PROGRESS and no invitations exist yet.
    let get_proposal_resp = client
        .get_proposal()
        .network_id(&network_id)
        .proposal_id(&proposal_id)
        .send()
        .await
        .expect("get_proposal should succeed");
    assert_eq!(
        get_proposal_resp.proposal().unwrap().status(),
        Some(&aws_sdk_managedblockchain::types::ProposalStatus::InProgress)
    );

    let inv_resp = client
        .list_invitations()
        .send()
        .await
        .expect("list_invitations should succeed");
    assert_eq!(inv_resp.invitations().len(), 0);

    // Step 5: The founding member votes YES — with a single member this reaches
    // the 50% approval threshold and the proposal becomes APPROVED.
    client
        .vote_on_proposal()
        .network_id(&network_id)
        .proposal_id(&proposal_id)
        .voter_member_id(&founding_member_id)
        .vote(aws_sdk_managedblockchain::types::VoteValue::Yes)
        .send()
        .await
        .expect("vote_on_proposal should succeed");

    // Step 6: Proposal should now be APPROVED and one invitation created.
    let get_proposal_resp = client
        .get_proposal()
        .network_id(&network_id)
        .proposal_id(&proposal_id)
        .send()
        .await
        .expect("get_proposal after vote should succeed");
    let proposal = get_proposal_resp.proposal().unwrap();
    assert_eq!(
        proposal.status(),
        Some(&aws_sdk_managedblockchain::types::ProposalStatus::Approved)
    );
    assert_eq!(proposal.yes_vote_count(), Some(1));
    assert_eq!(proposal.outstanding_vote_count(), Some(0));

    let inv_resp = client
        .list_invitations()
        .send()
        .await
        .expect("list_invitations should succeed");
    let invitations = inv_resp.invitations();
    assert_eq!(invitations.len(), 1);
    assert_eq!(
        invitations[0].status(),
        Some(&aws_sdk_managedblockchain::types::InvitationStatus::Pending)
    );
    let invitation_id = invitations[0].invitation_id().unwrap().to_string();

    // Step 7: The new account accepts the invitation by creating a member.
    let create_member_resp = client
        .create_member()
        .network_id(&network_id)
        .invitation_id(&invitation_id)
        .member_configuration(
            aws_sdk_managedblockchain::types::MemberConfiguration::builder()
                .name("new-member")
                .framework_configuration(
                    aws_sdk_managedblockchain::types::MemberFrameworkConfiguration::builder()
                        .build(),
                )
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("create_member should succeed");

    let new_member_id = create_member_resp.member_id().unwrap().to_string();
    assert!(!new_member_id.is_empty());

    // Step 8: Network now has two members.
    let members_resp = client
        .list_members()
        .network_id(&network_id)
        .send()
        .await
        .expect("list_members should succeed");
    assert_eq!(members_resp.members().len(), 2);
}

/// Scenario: Node Deployment and Tag Management
///
/// An operator deploys blockchain nodes for a member, tags them for cost
/// allocation, modifies tags, then decommissions a node — exercising the
/// node lifecycle: create → tag → list → update → untag → delete.
#[tokio::test]
async fn test_node_deployment_and_tag_management_scenario() {
    let client = make_mb_client().await;

    // Step 1: Create a network and get the initial member.
    let create_resp = client
        .create_network()
        .name("ops-network")
        .framework(aws_sdk_managedblockchain::types::Framework::HyperledgerFabric)
        .framework_version("1.4")
        .voting_policy(aws_sdk_managedblockchain::types::VotingPolicy::builder().build())
        .member_configuration(
            aws_sdk_managedblockchain::types::MemberConfiguration::builder()
                .name("ops-member")
                .framework_configuration(
                    aws_sdk_managedblockchain::types::MemberFrameworkConfiguration::builder()
                        .build(),
                )
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("create_network should succeed");

    let network_id = create_resp.network_id().unwrap().to_string();
    let member_id = create_resp.member_id().unwrap().to_string();

    // Step 2: Deploy two nodes with cost-allocation tags.
    let node1_resp = client
        .create_node()
        .network_id(&network_id)
        .member_id(&member_id)
        .node_configuration(
            aws_sdk_managedblockchain::types::NodeConfiguration::builder()
                .instance_type("bc.t3.small")
                .availability_zone("us-east-1a")
                .build()
                .unwrap(),
        )
        .tags("env", "prod")
        .tags("team", "ops")
        .send()
        .await
        .expect("create_node node1 should succeed");

    let node1_id = node1_resp.node_id().unwrap().to_string();

    let node2_resp = client
        .create_node()
        .network_id(&network_id)
        .member_id(&member_id)
        .node_configuration(
            aws_sdk_managedblockchain::types::NodeConfiguration::builder()
                .instance_type("bc.t3.medium")
                .availability_zone("us-east-1b")
                .build()
                .unwrap(),
        )
        .tags("env", "prod")
        .tags("team", "data")
        .send()
        .await
        .expect("create_node node2 should succeed");

    let node2_id = node2_resp.node_id().unwrap().to_string();

    // Step 3: Verify both nodes appear in the list.
    let list_resp = client
        .list_nodes()
        .network_id(&network_id)
        .member_id(&member_id)
        .send()
        .await
        .expect("list_nodes should succeed");
    assert_eq!(list_resp.nodes().len(), 2);

    // Step 4: Inspect node1 and retrieve its ARN for tag management.
    let get_resp = client
        .get_node()
        .network_id(&network_id)
        .member_id(&member_id)
        .node_id(&node1_id)
        .send()
        .await
        .expect("get_node should succeed");
    let node1 = get_resp.node().unwrap();
    assert_eq!(node1.availability_zone(), Some("us-east-1a"));
    let node1_arn = node1.arn().unwrap().to_string();

    // Step 5: Add an extra tag to node1 and verify.
    client
        .tag_resource()
        .resource_arn(&node1_arn)
        .tags("cost-centre", "blockchain-001")
        .send()
        .await
        .expect("tag_resource should succeed");

    let tags_resp = client
        .list_tags_for_resource()
        .resource_arn(&node1_arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");
    let tags = tags_resp.tags().unwrap();
    assert_eq!(tags.get("env").map(|s| s.as_str()), Some("prod"));
    assert_eq!(
        tags.get("cost-centre").map(|s| s.as_str()),
        Some("blockchain-001")
    );

    // Step 6: Remove the transient "team" tag from node1.
    client
        .untag_resource()
        .resource_arn(&node1_arn)
        .tag_keys("team")
        .send()
        .await
        .expect("untag_resource should succeed");

    let tags_resp = client
        .list_tags_for_resource()
        .resource_arn(&node1_arn)
        .send()
        .await
        .expect("list_tags_for_resource after untag should succeed");
    let tags = tags_resp.tags().unwrap();
    assert!(tags.get("team").is_none());
    assert_eq!(tags.get("env").map(|s| s.as_str()), Some("prod"));

    // Step 7: Update node1 (no-op in mock, but should succeed).
    client
        .update_node()
        .network_id(&network_id)
        .member_id(&member_id)
        .node_id(&node1_id)
        .send()
        .await
        .expect("update_node should succeed");

    // Step 8: Decommission node2.
    client
        .delete_node()
        .network_id(&network_id)
        .member_id(&member_id)
        .node_id(&node2_id)
        .send()
        .await
        .expect("delete_node should succeed");

    // Step 9: Only node1 should remain.
    let list_resp = client
        .list_nodes()
        .network_id(&network_id)
        .member_id(&member_id)
        .send()
        .await
        .expect("list_nodes after delete should succeed");
    assert_eq!(list_resp.nodes().len(), 1);
    assert_eq!(list_resp.nodes()[0].id(), Some(node1_id.as_str()));
}

/// Scenario: Accessor Token Lifecycle for Ethereum Access
///
/// A developer provisions an accessor token to call the Ethereum JSON-RPC API,
/// verifies it, tags it for tracking, and eventually rotates it by deleting the
/// old one and creating a new one.
#[tokio::test]
async fn test_accessor_token_rotation_scenario() {
    let client = make_mb_client().await;

    // Step 1: Create the initial accessor token.
    let create_resp = client
        .create_accessor()
        .accessor_type(aws_sdk_managedblockchain::types::AccessorType::BillingToken)
        .network_type(
            aws_sdk_managedblockchain::types::AccessorNetworkType::EthereumMainnetAndGoerli,
        )
        .tags("rotation-phase", "current")
        .send()
        .await
        .expect("create_accessor should succeed");

    let accessor_id = create_resp.accessor_id().unwrap().to_string();
    let billing_token = create_resp.billing_token().unwrap().to_string();
    assert!(!billing_token.is_empty());

    // Step 2: Verify the accessor details.
    let get_resp = client
        .get_accessor()
        .accessor_id(&accessor_id)
        .send()
        .await
        .expect("get_accessor should succeed");
    let accessor = get_resp.accessor().unwrap();
    assert_eq!(accessor.id(), Some(accessor_id.as_str()));
    assert_eq!(
        accessor.status(),
        Some(&aws_sdk_managedblockchain::types::AccessorStatus::Available)
    );
    assert_eq!(accessor.billing_token(), Some(billing_token.as_str()));
    let accessor_arn = accessor.arn().unwrap().to_string();

    // Step 3: Add an operational tag.
    client
        .tag_resource()
        .resource_arn(&accessor_arn)
        .tags("owner", "dev-team")
        .send()
        .await
        .expect("tag_resource should succeed");

    let tags_resp = client
        .list_tags_for_resource()
        .resource_arn(&accessor_arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");
    let tags = tags_resp.tags().unwrap();
    assert_eq!(
        tags.get("rotation-phase").map(|s| s.as_str()),
        Some("current")
    );
    assert_eq!(tags.get("owner").map(|s| s.as_str()), Some("dev-team"));

    // Step 4: Rotate — create the new accessor first.
    let new_resp = client
        .create_accessor()
        .accessor_type(aws_sdk_managedblockchain::types::AccessorType::BillingToken)
        .tags("rotation-phase", "new")
        .send()
        .await
        .expect("create new accessor should succeed");
    let new_accessor_id = new_resp.accessor_id().unwrap().to_string();
    assert_ne!(new_accessor_id, accessor_id);

    // Step 5: Two accessors now exist.
    let list_resp = client
        .list_accessors()
        .send()
        .await
        .expect("list_accessors should succeed");
    assert_eq!(list_resp.accessors().len(), 2);

    // Step 6: Delete the old accessor (completing the rotation).
    client
        .delete_accessor()
        .accessor_id(&accessor_id)
        .send()
        .await
        .expect("delete old accessor should succeed");

    // Step 7: Only the new accessor remains.
    let list_resp = client
        .list_accessors()
        .send()
        .await
        .expect("list_accessors after rotation should succeed");
    assert_eq!(list_resp.accessors().len(), 1);
    assert_eq!(
        list_resp.accessors()[0].id(),
        Some(new_accessor_id.as_str())
    );
}
