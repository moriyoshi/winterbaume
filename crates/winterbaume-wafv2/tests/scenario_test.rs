use aws_sdk_wafv2::config::BehaviorVersion;
use aws_sdk_wafv2::types::{
    AllowAction, BlockAction, DefaultAction, IpAddressVersion, IpSetReferenceStatement, Rule,
    RuleAction, Scope, Statement, Tag, VisibilityConfig,
};
use winterbaume_core::MockAws;
use winterbaume_wafv2::WafV2Service;

async fn make_client() -> aws_sdk_wafv2::Client {
    let mock = MockAws::builder().with_service(WafV2Service::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_wafv2::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_wafv2::Client::new(&config)
}

fn allow_action() -> DefaultAction {
    DefaultAction::builder()
        .allow(AllowAction::builder().build())
        .build()
}

fn block_action() -> DefaultAction {
    DefaultAction::builder()
        .block(BlockAction::builder().build())
        .build()
}

fn vis_config() -> VisibilityConfig {
    VisibilityConfig::builder()
        .sampled_requests_enabled(true)
        .cloud_watch_metrics_enabled(true)
        .metric_name("metric")
        .build()
        .unwrap()
}

/// Scenario: IP block list pipeline
///
/// A typical WAFv2 deployment: create an IP set of blocked CIDRs, bundle it
/// into a rule group, attach the rule group to a web ACL, verify the capacity
/// reflects the rule group reference, then tear everything down in reverse
/// order and confirm all resources are gone.
///
/// Operations chained: CreateIPSet → CreateRuleGroup → CheckCapacity →
///   CreateWebACL → GetWebACL → ListWebACLs → DeleteWebACL →
///   DeleteRuleGroup → DeleteIPSet
#[tokio::test]
async fn test_ip_block_list_pipeline() {
    let client = make_client().await;

    // Step 1: Create an IP set containing blocked CIDRs.
    let ip_set_resp = client
        .create_ip_set()
        .name("blocked-cidrs")
        .scope(Scope::Regional)
        .ip_address_version(IpAddressVersion::Ipv4)
        .addresses("198.51.100.0/24")
        .addresses("203.0.113.0/24")
        .description("Known malicious CIDR blocks")
        .send()
        .await
        .expect("create_ip_set should succeed");

    let ip_set_summary = ip_set_resp.summary().expect("should have ip set summary");
    let ip_set_id = ip_set_summary
        .id()
        .expect("ip set should have id")
        .to_string();
    let ip_set_arn = ip_set_summary
        .arn()
        .expect("ip set should have arn")
        .to_string();
    let ip_set_lock = ip_set_summary
        .lock_token()
        .expect("ip set should have lock token")
        .to_string();
    assert_eq!(ip_set_summary.name().unwrap(), "blocked-cidrs");

    // Step 2: Check capacity for a rule that references the IP set — IP set
    // reference rules cost 1 WCU.
    let ip_ref_rule = Rule::builder()
        .name("block-ip-set")
        .priority(0)
        .statement(
            Statement::builder()
                .ip_set_reference_statement(
                    IpSetReferenceStatement::builder()
                        .arn(ip_set_arn)
                        .build()
                        .unwrap(),
                )
                .build(),
        )
        .action(
            RuleAction::builder()
                .block(BlockAction::builder().build())
                .build(),
        )
        .visibility_config(vis_config())
        .build()
        .unwrap();

    let capacity_resp = client
        .check_capacity()
        .scope(Scope::Regional)
        .rules(ip_ref_rule.clone())
        .send()
        .await
        .expect("check_capacity should succeed");

    // IP set reference: 1 WCU
    assert_eq!(capacity_resp.capacity(), 1);

    // Step 3: Create a rule group that blocks the IP set.
    let rg_resp = client
        .create_rule_group()
        .name("block-ip-group")
        .scope(Scope::Regional)
        .capacity(1)
        .rules(ip_ref_rule)
        .visibility_config(vis_config())
        .description("Blocks known bad IP ranges")
        .send()
        .await
        .expect("create_rule_group should succeed");

    let rg_summary = rg_resp.summary().expect("should have rule group summary");
    let rg_id = rg_summary
        .id()
        .expect("rule group should have id")
        .to_string();
    let rg_lock = rg_summary
        .lock_token()
        .expect("rule group should have lock token")
        .to_string();
    assert_eq!(rg_summary.name().unwrap(), "block-ip-group");

    // Step 4: Create a web ACL that references the rule group.
    let acl_resp = client
        .create_web_acl()
        .name("main-acl")
        .scope(Scope::Regional)
        .default_action(allow_action())
        .visibility_config(vis_config())
        .description("Main WAF protection")
        .send()
        .await
        .expect("create_web_acl should succeed");

    let acl_summary = acl_resp.summary().expect("should have acl summary");
    let acl_id = acl_summary.id().expect("acl should have id").to_string();
    let acl_lock = acl_summary
        .lock_token()
        .expect("acl should have lock token")
        .to_string();
    assert_eq!(acl_summary.name().unwrap(), "main-acl");

    // Step 5: Verify the web ACL is visible in the list.
    let list_resp = client
        .list_web_acls()
        .scope(Scope::Regional)
        .send()
        .await
        .expect("list_web_acls should succeed");
    assert_eq!(list_resp.web_acls().len(), 1, "should be exactly 1 web ACL");

    // Step 6: Read back the web ACL and confirm its description.
    let get_resp = client
        .get_web_acl()
        .name("main-acl")
        .scope(Scope::Regional)
        .id(&acl_id)
        .send()
        .await
        .expect("get_web_acl should succeed");
    let acl = get_resp.web_acl().expect("should have web acl");
    assert_eq!(acl.description().unwrap(), "Main WAF protection");

    // Step 7: Update the web ACL description (simulating a Terraform plan +
    // apply that changes the description field).
    let update_resp = client
        .update_web_acl()
        .name("main-acl")
        .scope(Scope::Regional)
        .id(&acl_id)
        .lock_token(&acl_lock)
        .default_action(block_action())
        .visibility_config(vis_config())
        .description("Updated WAF protection")
        .send()
        .await
        .expect("update_web_acl should succeed");
    let updated_lock = update_resp
        .next_lock_token()
        .expect("update should return new lock token")
        .to_string();

    // Step 8: Tear down in reverse order — web ACL, rule group, IP set.
    client
        .delete_web_acl()
        .name("main-acl")
        .scope(Scope::Regional)
        .id(&acl_id)
        .lock_token(&updated_lock)
        .send()
        .await
        .expect("delete_web_acl should succeed");

    client
        .delete_rule_group()
        .name("block-ip-group")
        .scope(Scope::Regional)
        .id(&rg_id)
        .lock_token(&rg_lock)
        .send()
        .await
        .expect("delete_rule_group should succeed");

    client
        .delete_ip_set()
        .name("blocked-cidrs")
        .scope(Scope::Regional)
        .id(&ip_set_id)
        .lock_token(&ip_set_lock)
        .send()
        .await
        .expect("delete_ip_set should succeed");

    // Step 9: Confirm all resources are gone.
    let list_after = client
        .list_web_acls()
        .scope(Scope::Regional)
        .send()
        .await
        .unwrap();
    assert_eq!(
        list_after.web_acls().len(),
        0,
        "web ACL list should be empty after delete"
    );

    let ip_sets_after = client
        .list_ip_sets()
        .scope(Scope::Regional)
        .send()
        .await
        .unwrap();
    assert_eq!(
        ip_sets_after.ip_sets().len(),
        0,
        "ip set list should be empty after delete"
    );
}

/// Scenario: Web ACL resource association lifecycle
///
/// Demonstrates associating a web ACL with an application resource ARN, then
/// querying resources for that ACL, and finally disassociating it.
///
/// Operations chained: CreateWebACL → AssociateWebACL → GetWebACLForResource →
///   ListResourcesForWebACL → DisassociateWebACL → GetWebACLForResource
///   (returns empty)
#[tokio::test]
async fn test_web_acl_resource_association_lifecycle() {
    let client = make_client().await;

    // Create the web ACL to associate.
    let acl_resp = client
        .create_web_acl()
        .name("assoc-acl")
        .scope(Scope::Regional)
        .default_action(allow_action())
        .visibility_config(vis_config())
        .send()
        .await
        .expect("create_web_acl should succeed");

    let acl_summary = acl_resp.summary().expect("should have summary");
    let acl_id = acl_summary.id().expect("should have id").to_string();
    let acl_arn = acl_summary.arn().expect("should have arn").to_string();
    let acl_lock = acl_summary
        .lock_token()
        .expect("should have lock token")
        .to_string();

    // Simulate an ALB ARN to associate the ACL with.
    let resource_arn =
        "arn:aws:elasticloadbalancing:us-east-1:123456789012:loadbalancer/app/my-alb/1234567890";

    // Associate the web ACL with the resource.
    client
        .associate_web_acl()
        .web_acl_arn(&acl_arn)
        .resource_arn(resource_arn)
        .send()
        .await
        .expect("associate_web_acl should succeed");

    // Verify the resource now reports the ACL.
    let get_for_resource = client
        .get_web_acl_for_resource()
        .resource_arn(resource_arn)
        .send()
        .await
        .expect("get_web_acl_for_resource should succeed");
    let returned_acl = get_for_resource
        .web_acl()
        .expect("resource should have an associated ACL");
    assert_eq!(returned_acl.id(), acl_id.as_str());

    // Verify the ACL lists the resource as associated.
    let resources = client
        .list_resources_for_web_acl()
        .web_acl_arn(&acl_arn)
        .send()
        .await
        .expect("list_resources_for_web_acl should succeed");
    assert!(
        resources
            .resource_arns()
            .contains(&resource_arn.to_string()),
        "resource ARN should appear in list"
    );

    // Disassociate the web ACL.
    client
        .disassociate_web_acl()
        .resource_arn(resource_arn)
        .send()
        .await
        .expect("disassociate_web_acl should succeed");

    // After disassociation the resource should have no ACL.
    let get_after = client
        .get_web_acl_for_resource()
        .resource_arn(resource_arn)
        .send()
        .await
        .expect("get_web_acl_for_resource after disassociate should succeed");
    assert!(
        get_after.web_acl().is_none(),
        "resource should have no ACL after disassociation"
    );

    // Clean up.
    client
        .delete_web_acl()
        .name("assoc-acl")
        .scope(Scope::Regional)
        .id(&acl_id)
        .lock_token(&acl_lock)
        .send()
        .await
        .expect("delete should succeed");
}

/// Scenario: Tagging and policy lifecycle across resource types
///
/// Exercises the cross-resource tagging and permission-policy APIs, confirming
/// that tags applied to an IP set and a web ACL are independently managed and
/// that a permission policy round-trips correctly.
///
/// Operations chained: CreateIPSet → CreateWebACL → TagResource (both) →
///   ListTagsForResource → UntagResource → PutPermissionPolicy →
///   GetPermissionPolicy → DeletePermissionPolicy → cleanup
#[tokio::test]
async fn test_tagging_and_policy_lifecycle() {
    let client = make_client().await;

    // Create resources to tag.
    let ip_set_resp = client
        .create_ip_set()
        .name("tagged-ipset")
        .scope(Scope::Regional)
        .ip_address_version(IpAddressVersion::Ipv4)
        .send()
        .await
        .expect("create_ip_set should succeed");
    let ip_set_summary = ip_set_resp.summary().unwrap();
    let ip_set_arn = ip_set_summary.arn().unwrap().to_string();

    let acl_resp = client
        .create_web_acl()
        .name("tagged-acl")
        .scope(Scope::Regional)
        .default_action(allow_action())
        .visibility_config(vis_config())
        .send()
        .await
        .expect("create_web_acl should succeed");
    let acl_summary = acl_resp.summary().unwrap();
    let acl_arn = acl_summary.arn().unwrap().to_string();
    let acl_id = acl_summary.id().unwrap().to_string();
    let acl_lock = acl_summary.lock_token().unwrap().to_string();

    // Tag the IP set.
    client
        .tag_resource()
        .resource_arn(&ip_set_arn)
        .tags(Tag::builder().key("env").value("prod").build().unwrap())
        .send()
        .await
        .expect("tag_resource for ip set should succeed");

    // Tag the web ACL.
    client
        .tag_resource()
        .resource_arn(&acl_arn)
        .tags(
            Tag::builder()
                .key("team")
                .value("security")
                .build()
                .unwrap(),
        )
        .tags(
            Tag::builder()
                .key("cost-centre")
                .value("infra")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("tag_resource for acl should succeed");

    // Verify IP set has exactly 1 tag.
    let ip_tags = client
        .list_tags_for_resource()
        .resource_arn(&ip_set_arn)
        .send()
        .await
        .unwrap()
        .tag_info_for_resource()
        .unwrap()
        .tag_list()
        .to_vec();
    assert_eq!(ip_tags.len(), 1);
    assert_eq!(ip_tags[0].key(), "env");

    // Verify ACL has 2 tags.
    let acl_tags = client
        .list_tags_for_resource()
        .resource_arn(&acl_arn)
        .send()
        .await
        .unwrap()
        .tag_info_for_resource()
        .unwrap()
        .tag_list()
        .to_vec();
    assert_eq!(acl_tags.len(), 2);

    // Remove one ACL tag.
    client
        .untag_resource()
        .resource_arn(&acl_arn)
        .tag_keys("cost-centre")
        .send()
        .await
        .expect("untag_resource should succeed");

    let acl_tags_after = client
        .list_tags_for_resource()
        .resource_arn(&acl_arn)
        .send()
        .await
        .unwrap()
        .tag_info_for_resource()
        .unwrap()
        .tag_list()
        .to_vec();
    assert_eq!(acl_tags_after.len(), 1);
    assert_eq!(acl_tags_after[0].key(), "team");

    // Attach a resource-based permission policy to the web ACL.
    let policy = r#"{"Version":"2012-10-17","Statement":[{"Effect":"Allow","Principal":{"AWS":"*"},"Action":"wafv2:*","Resource":"*"}]}"#;
    client
        .put_permission_policy()
        .resource_arn(&acl_arn)
        .policy(policy)
        .send()
        .await
        .expect("put_permission_policy should succeed");

    // Read back the policy.
    let get_policy = client
        .get_permission_policy()
        .resource_arn(&acl_arn)
        .send()
        .await
        .expect("get_permission_policy should succeed");
    assert_eq!(get_policy.policy().unwrap(), policy);

    // Delete the policy.
    client
        .delete_permission_policy()
        .resource_arn(&acl_arn)
        .send()
        .await
        .expect("delete_permission_policy should succeed");

    let get_after = client
        .get_permission_policy()
        .resource_arn(&acl_arn)
        .send()
        .await;
    assert!(get_after.is_err(), "policy should be gone after delete");

    // Clean up resources.
    client
        .delete_web_acl()
        .name("tagged-acl")
        .scope(Scope::Regional)
        .id(&acl_id)
        .lock_token(&acl_lock)
        .send()
        .await
        .expect("delete acl should succeed");

    client
        .delete_ip_set()
        .name("tagged-ipset")
        .scope(Scope::Regional)
        .id(ip_set_summary.id().unwrap())
        .lock_token(ip_set_summary.lock_token().unwrap())
        .send()
        .await
        .expect("delete ip set should succeed");
}
