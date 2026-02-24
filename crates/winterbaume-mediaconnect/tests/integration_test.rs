use aws_sdk_mediaconnect::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_mediaconnect::MediaConnectService;

async fn make_mediaconnect_client() -> aws_sdk_mediaconnect::Client {
    let mock = MockAws::builder()
        .with_service(MediaConnectService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_mediaconnect::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_mediaconnect::Client::new(&config)
}

#[tokio::test]
async fn test_create_and_describe_flow() {
    let client = make_mediaconnect_client().await;

    let create_resp = client
        .create_flow()
        .name("test-flow")
        .send()
        .await
        .expect("create_flow should succeed");

    let flow = create_resp.flow().expect("response should contain flow");
    assert_eq!(flow.name(), Some("test-flow"));
    assert!(flow.flow_arn().unwrap_or("").contains("test-flow"));

    let flow_arn = flow.flow_arn().expect("flow should have arn").to_string();

    let describe_resp = client
        .describe_flow()
        .flow_arn(&flow_arn)
        .send()
        .await
        .expect("describe_flow should succeed");

    let described = describe_resp.flow().expect("response should contain flow");
    assert_eq!(described.name(), Some("test-flow"));
    assert_eq!(described.flow_arn(), Some(flow_arn.as_str()));
}

#[tokio::test]
async fn test_create_flow_with_description() {
    let client = make_mediaconnect_client().await;

    let create_resp = client
        .create_flow()
        .name("described-flow")
        .send()
        .await
        .expect("create_flow should succeed");

    let flow = create_resp.flow().expect("response should contain flow");
    assert_eq!(flow.name(), Some("described-flow"));
}

#[tokio::test]
async fn test_delete_flow() {
    let client = make_mediaconnect_client().await;

    let create_resp = client.create_flow().name("to-delete").send().await.unwrap();

    let flow_arn = create_resp
        .flow()
        .unwrap()
        .flow_arn()
        .expect("flow should have arn")
        .to_string();

    let delete_resp = client
        .delete_flow()
        .flow_arn(&flow_arn)
        .send()
        .await
        .expect("delete_flow should succeed");

    assert_eq!(delete_resp.flow_arn(), Some(flow_arn.as_str()));

    let result = client.describe_flow().flow_arn(&flow_arn).send().await;
    assert!(result.is_err(), "describe after delete should fail");
}

#[tokio::test]
async fn test_list_flows_empty() {
    let client = make_mediaconnect_client().await;

    let resp = client
        .list_flows()
        .send()
        .await
        .expect("list_flows should succeed");

    assert_eq!(resp.flows().len(), 0);
}

#[tokio::test]
async fn test_list_flows() {
    let client = make_mediaconnect_client().await;

    client.create_flow().name("flow-a").send().await.unwrap();
    client.create_flow().name("flow-b").send().await.unwrap();

    let resp = client
        .list_flows()
        .send()
        .await
        .expect("list_flows should succeed");

    assert_eq!(resp.flows().len(), 2);
}

#[tokio::test]
async fn test_describe_nonexistent_flow() {
    let client = make_mediaconnect_client().await;

    let result = client
        .describe_flow()
        .flow_arn("arn:aws:mediaconnect:us-east-1:123456789012:flow:nonexistent:no")
        .send()
        .await;
    assert!(result.is_err(), "describe nonexistent flow should fail");
}

#[tokio::test]
async fn test_delete_nonexistent_flow() {
    let client = make_mediaconnect_client().await;

    let result = client
        .delete_flow()
        .flow_arn("arn:aws:mediaconnect:us-east-1:123456789012:flow:nonexistent:no")
        .send()
        .await;
    assert!(result.is_err(), "delete nonexistent flow should fail");
}

// --- StartFlow / StopFlow ---

#[tokio::test]
async fn test_start_flow() {
    let client = make_mediaconnect_client().await;

    let create_resp = client.create_flow().name("start-me").send().await.unwrap();
    let flow_arn = create_resp.flow().unwrap().flow_arn().unwrap().to_string();

    let start_resp = client
        .start_flow()
        .flow_arn(&flow_arn)
        .send()
        .await
        .expect("start_flow should succeed");

    assert_eq!(start_resp.flow_arn(), Some(flow_arn.as_str()));
    assert_eq!(start_resp.status().map(|s| s.as_str()), Some("ACTIVE"));

    // Verify status via describe
    let desc = client
        .describe_flow()
        .flow_arn(&flow_arn)
        .send()
        .await
        .unwrap();
    let flow = desc.flow().unwrap();
    assert_eq!(flow.status().map(|s| s.as_str()), Some("ACTIVE"));
}

#[tokio::test]
async fn test_stop_flow() {
    let client = make_mediaconnect_client().await;

    let create_resp = client.create_flow().name("stop-me").send().await.unwrap();
    let flow_arn = create_resp.flow().unwrap().flow_arn().unwrap().to_string();

    // Start it first
    client
        .start_flow()
        .flow_arn(&flow_arn)
        .send()
        .await
        .unwrap();

    // Stop it
    let stop_resp = client
        .stop_flow()
        .flow_arn(&flow_arn)
        .send()
        .await
        .expect("stop_flow should succeed");

    assert_eq!(stop_resp.flow_arn(), Some(flow_arn.as_str()));
    assert_eq!(stop_resp.status().map(|s| s.as_str()), Some("STANDBY"));
}

#[tokio::test]
async fn test_start_nonexistent_flow() {
    let client = make_mediaconnect_client().await;
    let result = client
        .start_flow()
        .flow_arn("arn:aws:mediaconnect:us-east-1:123456789012:flow:nonexistent:no")
        .send()
        .await;
    assert!(result.is_err(), "start nonexistent flow should fail");
}

// --- UpdateFlow ---

#[tokio::test]
async fn test_update_flow() {
    let client = make_mediaconnect_client().await;

    let create_resp = client.create_flow().name("update-me").send().await.unwrap();
    let flow_arn = create_resp.flow().unwrap().flow_arn().unwrap().to_string();

    let update_resp = client
        .update_flow()
        .flow_arn(&flow_arn)
        .send()
        .await
        .expect("update_flow should succeed");

    let flow = update_resp.flow().unwrap();
    assert_eq!(flow.name(), Some("update-me"));
}

#[tokio::test]
async fn test_update_nonexistent_flow() {
    let client = make_mediaconnect_client().await;
    let result = client
        .update_flow()
        .flow_arn("arn:aws:mediaconnect:us-east-1:123456789012:flow:nonexistent:no")
        .send()
        .await;
    assert!(result.is_err(), "update nonexistent flow should fail");
}

// --- AddFlowOutputs / RemoveFlowOutput / UpdateFlowOutput ---

#[tokio::test]
async fn test_add_and_remove_flow_outputs() {
    use aws_sdk_mediaconnect::types::AddOutputRequest;

    let client = make_mediaconnect_client().await;

    let create_resp = client
        .create_flow()
        .name("output-flow")
        .send()
        .await
        .unwrap();
    let flow_arn = create_resp.flow().unwrap().flow_arn().unwrap().to_string();

    let output_req = AddOutputRequest::builder()
        .name("my-output")
        .description("test output")
        .protocol(aws_sdk_mediaconnect::types::Protocol::SrtListener)
        .build();

    let add_resp = client
        .add_flow_outputs()
        .flow_arn(&flow_arn)
        .outputs(output_req)
        .send()
        .await
        .expect("add_flow_outputs should succeed");

    assert_eq!(add_resp.flow_arn(), Some(flow_arn.as_str()));
    let outputs = add_resp.outputs();
    assert_eq!(outputs.len(), 1);
    assert_eq!(outputs[0].name(), Some("my-output"));

    let output_arn = outputs[0]
        .output_arn()
        .expect("output should have arn")
        .to_string();

    // Remove the output
    let remove_resp = client
        .remove_flow_output()
        .flow_arn(&flow_arn)
        .output_arn(&output_arn)
        .send()
        .await
        .expect("remove_flow_output should succeed");

    assert_eq!(remove_resp.flow_arn(), Some(flow_arn.as_str()));
    assert_eq!(remove_resp.output_arn(), Some(output_arn.as_str()));

    // Verify output is gone
    let desc = client
        .describe_flow()
        .flow_arn(&flow_arn)
        .send()
        .await
        .unwrap();
    assert_eq!(desc.flow().unwrap().outputs().len(), 0);
}

#[tokio::test]
async fn test_update_flow_output() {
    use aws_sdk_mediaconnect::types::AddOutputRequest;

    let client = make_mediaconnect_client().await;

    let create_resp = client
        .create_flow()
        .name("upd-output-flow")
        .send()
        .await
        .unwrap();
    let flow_arn = create_resp.flow().unwrap().flow_arn().unwrap().to_string();

    let output_req = AddOutputRequest::builder()
        .name("upd-output")
        .description("orig desc")
        .protocol(aws_sdk_mediaconnect::types::Protocol::SrtListener)
        .build();

    let add_resp = client
        .add_flow_outputs()
        .flow_arn(&flow_arn)
        .outputs(output_req)
        .send()
        .await
        .unwrap();

    let output_arn = add_resp.outputs()[0]
        .output_arn()
        .expect("output should have arn")
        .to_string();

    let upd_resp = client
        .update_flow_output()
        .flow_arn(&flow_arn)
        .output_arn(&output_arn)
        .description("new desc")
        .send()
        .await
        .expect("update_flow_output should succeed");

    let output = upd_resp.output().unwrap();
    assert_eq!(output.description(), Some("new desc"));
}

// --- AddFlowSources / RemoveFlowSource / UpdateFlowSource ---

#[tokio::test]
async fn test_add_and_remove_flow_sources() {
    use aws_sdk_mediaconnect::types::SetSourceRequest;

    let client = make_mediaconnect_client().await;

    let create_resp = client
        .create_flow()
        .name("source-flow")
        .send()
        .await
        .unwrap();
    let flow_arn = create_resp.flow().unwrap().flow_arn().unwrap().to_string();

    let source_req = SetSourceRequest::builder()
        .name("my-source")
        .description("test source")
        .protocol(aws_sdk_mediaconnect::types::Protocol::SrtListener)
        .build();

    let add_resp = client
        .add_flow_sources()
        .flow_arn(&flow_arn)
        .sources(source_req)
        .send()
        .await
        .expect("add_flow_sources should succeed");

    assert_eq!(add_resp.flow_arn(), Some(flow_arn.as_str()));
    let sources = add_resp.sources();
    assert_eq!(sources.len(), 1);
    assert_eq!(sources[0].name(), Some("my-source"));

    let source_arn = sources[0]
        .source_arn()
        .expect("source should have arn")
        .to_string();

    // Remove the source
    let remove_resp = client
        .remove_flow_source()
        .flow_arn(&flow_arn)
        .source_arn(&source_arn)
        .send()
        .await
        .expect("remove_flow_source should succeed");

    assert_eq!(remove_resp.flow_arn(), Some(flow_arn.as_str()));
    assert_eq!(remove_resp.source_arn(), Some(source_arn.as_str()));
}

#[tokio::test]
async fn test_update_flow_source() {
    use aws_sdk_mediaconnect::types::SetSourceRequest;

    let client = make_mediaconnect_client().await;

    let create_resp = client
        .create_flow()
        .name("upd-source-flow")
        .send()
        .await
        .unwrap();
    let flow_arn = create_resp.flow().unwrap().flow_arn().unwrap().to_string();

    let source_req = SetSourceRequest::builder()
        .name("upd-source")
        .description("orig source")
        .protocol(aws_sdk_mediaconnect::types::Protocol::SrtListener)
        .build();

    let add_resp = client
        .add_flow_sources()
        .flow_arn(&flow_arn)
        .sources(source_req)
        .send()
        .await
        .unwrap();

    let source_arn = add_resp.sources()[0]
        .source_arn()
        .expect("source should have arn")
        .to_string();

    let upd_resp = client
        .update_flow_source()
        .flow_arn(&flow_arn)
        .source_arn(&source_arn)
        .description("new source desc")
        .send()
        .await
        .expect("update_flow_source should succeed");

    let source = upd_resp.source().unwrap();
    assert_eq!(source.description(), Some("new source desc"));
}

// --- AddFlowVpcInterfaces / RemoveFlowVpcInterface ---

#[tokio::test]
async fn test_add_and_remove_flow_vpc_interfaces() {
    use aws_sdk_mediaconnect::types::{NetworkInterfaceType, VpcInterfaceRequest};

    let client = make_mediaconnect_client().await;

    let create_resp = client.create_flow().name("vpc-flow").send().await.unwrap();
    let flow_arn = create_resp.flow().unwrap().flow_arn().unwrap().to_string();

    let vpc_req = VpcInterfaceRequest::builder()
        .name("my-vpc")
        .role_arn("arn:aws:iam::123456789012:role/test")
        .security_group_ids("sg-12345")
        .subnet_id("subnet-12345")
        .network_interface_type(NetworkInterfaceType::Ena)
        .build();

    let add_resp = client
        .add_flow_vpc_interfaces()
        .flow_arn(&flow_arn)
        .vpc_interfaces(vpc_req)
        .send()
        .await
        .expect("add_flow_vpc_interfaces should succeed");

    assert_eq!(add_resp.flow_arn(), Some(flow_arn.as_str()));
    let vpcs = add_resp.vpc_interfaces();
    assert_eq!(vpcs.len(), 1);
    assert_eq!(vpcs[0].name(), Some("my-vpc"));

    // Remove the VPC interface
    let remove_resp = client
        .remove_flow_vpc_interface()
        .flow_arn(&flow_arn)
        .vpc_interface_name("my-vpc")
        .send()
        .await
        .expect("remove_flow_vpc_interface should succeed");

    assert_eq!(remove_resp.flow_arn(), Some(flow_arn.as_str()));
    assert_eq!(remove_resp.vpc_interface_name(), Some("my-vpc"));
}

// --- TagResource / ListTagsForResource / UntagResource ---

#[tokio::test]
async fn test_tag_list_untag_resource() {
    let client = make_mediaconnect_client().await;

    let create_resp = client.create_flow().name("tag-flow").send().await.unwrap();
    let flow_arn = create_resp.flow().unwrap().flow_arn().unwrap().to_string();

    // Tag the resource
    client
        .tag_resource()
        .resource_arn(&flow_arn)
        .tags("env", "test")
        .tags("team", "platform")
        .send()
        .await
        .expect("tag_resource should succeed");

    // List tags
    let list_resp = client
        .list_tags_for_resource()
        .resource_arn(&flow_arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");

    let tags = list_resp.tags().unwrap();
    assert_eq!(tags.get("env"), Some(&"test".to_string()));
    assert_eq!(tags.get("team"), Some(&"platform".to_string()));

    // Untag
    client
        .untag_resource()
        .resource_arn(&flow_arn)
        .tag_keys("env")
        .send()
        .await
        .expect("untag_resource should succeed");

    // Verify
    let list_resp2 = client
        .list_tags_for_resource()
        .resource_arn(&flow_arn)
        .send()
        .await
        .unwrap();

    let tags2 = list_resp2.tags().unwrap();
    assert!(!tags2.contains_key("env"));
    assert_eq!(tags2.get("team"), Some(&"platform".to_string()));
}

// --- Lifecycle test ---

#[tokio::test]
async fn test_flow_lifecycle() {
    let client = make_mediaconnect_client().await;

    // Create
    let create_resp = client.create_flow().name("lifecycle").send().await.unwrap();
    let flow_arn = create_resp.flow().unwrap().flow_arn().unwrap().to_string();

    // Describe
    let desc = client
        .describe_flow()
        .flow_arn(&flow_arn)
        .send()
        .await
        .unwrap();
    assert_eq!(
        desc.flow().unwrap().status().map(|s| s.as_str()),
        Some("STANDBY")
    );

    // Start
    client
        .start_flow()
        .flow_arn(&flow_arn)
        .send()
        .await
        .unwrap();
    let desc2 = client
        .describe_flow()
        .flow_arn(&flow_arn)
        .send()
        .await
        .unwrap();
    assert_eq!(
        desc2.flow().unwrap().status().map(|s| s.as_str()),
        Some("ACTIVE")
    );

    // Stop
    client.stop_flow().flow_arn(&flow_arn).send().await.unwrap();
    let desc3 = client
        .describe_flow()
        .flow_arn(&flow_arn)
        .send()
        .await
        .unwrap();
    assert_eq!(
        desc3.flow().unwrap().status().map(|s| s.as_str()),
        Some("STANDBY")
    );

    // Delete
    client
        .delete_flow()
        .flow_arn(&flow_arn)
        .send()
        .await
        .unwrap();

    // Verify gone
    let result = client.describe_flow().flow_arn(&flow_arn).send().await;
    assert!(result.is_err());
}

// ---- Entitlement tests ----

#[tokio::test]
async fn test_grant_flow_entitlements() {
    let client = make_mediaconnect_client().await;

    let create_resp = client
        .create_flow()
        .name("entitlement-flow")
        .send()
        .await
        .expect("create_flow should succeed");
    let flow_arn = create_resp
        .flow()
        .unwrap()
        .flow_arn()
        .expect("flow_arn")
        .to_string();

    let ent = aws_sdk_mediaconnect::types::GrantEntitlementRequest::builder()
        .name("my-entitlement")
        .description("test")
        .subscribers("111111111111")
        .build();

    let grant_resp = client
        .grant_flow_entitlements()
        .flow_arn(&flow_arn)
        .entitlements(ent)
        .send()
        .await
        .expect("grant_flow_entitlements should succeed");

    let entitlements = grant_resp.entitlements();
    assert_eq!(entitlements.len(), 1);
    assert_eq!(entitlements[0].name().unwrap_or(""), "my-entitlement");
}

#[tokio::test]
async fn test_revoke_flow_entitlement() {
    let client = make_mediaconnect_client().await;

    let create_resp = client
        .create_flow()
        .name("revoke-flow")
        .send()
        .await
        .unwrap();
    let flow_arn = create_resp
        .flow()
        .unwrap()
        .flow_arn()
        .expect("flow_arn")
        .to_string();

    let ent = aws_sdk_mediaconnect::types::GrantEntitlementRequest::builder()
        .name("to-revoke")
        .subscribers("222222222222")
        .build();

    let grant_resp = client
        .grant_flow_entitlements()
        .flow_arn(&flow_arn)
        .entitlements(ent)
        .send()
        .await
        .unwrap();

    let entitlement_arn = grant_resp.entitlements()[0]
        .entitlement_arn()
        .unwrap_or("")
        .to_string();

    let revoke_resp = client
        .revoke_flow_entitlement()
        .flow_arn(&flow_arn)
        .entitlement_arn(&entitlement_arn)
        .send()
        .await
        .expect("revoke_flow_entitlement should succeed");

    assert_eq!(revoke_resp.entitlement_arn().unwrap_or(""), entitlement_arn);
}

#[tokio::test]
async fn test_update_flow_entitlement() {
    let client = make_mediaconnect_client().await;

    let create_resp = client
        .create_flow()
        .name("update-ent-flow")
        .send()
        .await
        .unwrap();
    let flow_arn = create_resp
        .flow()
        .unwrap()
        .flow_arn()
        .expect("flow_arn")
        .to_string();

    let ent = aws_sdk_mediaconnect::types::GrantEntitlementRequest::builder()
        .name("updatable-ent")
        .subscribers("333333333333")
        .build();

    let grant_resp = client
        .grant_flow_entitlements()
        .flow_arn(&flow_arn)
        .entitlements(ent)
        .send()
        .await
        .unwrap();

    let entitlement_arn = grant_resp.entitlements()[0]
        .entitlement_arn()
        .unwrap_or("")
        .to_string();

    let update_resp = client
        .update_flow_entitlement()
        .flow_arn(&flow_arn)
        .entitlement_arn(&entitlement_arn)
        .description("updated description")
        .send()
        .await
        .expect("update_flow_entitlement should succeed");

    let updated = update_resp.entitlement().expect("should have entitlement");
    assert_eq!(updated.description().unwrap(), "updated description");
}
