use bytes::Bytes;
use http::HeaderMap;
use serde_json::{Value, json};
use winterbaume_core::{MockRequest, MockService};
use winterbaume_resourcegroupstagging::TaggingService;

fn make_request(action: &str, body: Value) -> MockRequest {
    let mut headers = HeaderMap::new();
    headers.insert(
        "x-amz-target",
        format!("ResourceGroupsTaggingAPI_20170126.{action}")
            .parse()
            .unwrap(),
    );
    headers.insert(
        http::header::CONTENT_TYPE,
        "application/x-amz-json-1.1".parse().unwrap(),
    );
    MockRequest {
        method: "POST".to_string(),
        uri: "https://tagging.us-east-1.amazonaws.com/".to_string(),
        headers,
        body: Bytes::from(body.to_string()),
    }
}

async fn send(service: &TaggingService, action: &str, body: Value) -> (u16, Value) {
    let request = make_request(action, body);
    let response = service.handle(request).await;
    let body: Value = serde_json::from_slice(&response.body).unwrap();
    (response.status, body)
}

#[tokio::test]
async fn test_tag_and_get_resources() {
    let service = TaggingService::new();

    // Tag a resource
    let (status, body) = send(
        &service,
        "TagResources",
        json!({
            "ResourceARNList": ["arn:aws:s3:::my-bucket"],
            "Tags": {
                "Environment": "Production",
                "Team": "Backend"
            }
        }),
    )
    .await;

    assert_eq!(status, 200);
    assert!(body["FailedResourcesMap"].as_object().unwrap().is_empty());

    // Get resources
    let (status, body) = send(&service, "GetResources", json!({})).await;

    assert_eq!(status, 200);
    let mappings = body["ResourceTagMappingList"].as_array().unwrap();
    assert_eq!(mappings.len(), 1);
    assert_eq!(mappings[0]["ResourceARN"], "arn:aws:s3:::my-bucket");
    let tags = mappings[0]["Tags"].as_array().unwrap();
    assert_eq!(tags.len(), 2);
}

#[tokio::test]
async fn test_tag_and_untag_resources() {
    let service = TaggingService::new();

    // Tag a resource with two tags
    send(
        &service,
        "TagResources",
        json!({
            "ResourceARNList": ["arn:aws:ec2:us-east-1:123456789012:instance/i-abc"],
            "Tags": {
                "Name": "MyInstance",
                "Env": "Dev"
            }
        }),
    )
    .await;

    // Untag one key
    let (status, body) = send(
        &service,
        "UntagResources",
        json!({
            "ResourceARNList": ["arn:aws:ec2:us-east-1:123456789012:instance/i-abc"],
            "TagKeys": ["Env"]
        }),
    )
    .await;

    assert_eq!(status, 200);
    assert!(body["FailedResourcesMap"].as_object().unwrap().is_empty());

    // Verify only one tag remains
    let (status, body) = send(&service, "GetResources", json!({})).await;

    assert_eq!(status, 200);
    let mappings = body["ResourceTagMappingList"].as_array().unwrap();
    assert_eq!(mappings.len(), 1);
    let tags = mappings[0]["Tags"].as_array().unwrap();
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0]["Key"], "Name");
    assert_eq!(tags[0]["Value"], "MyInstance");
}

#[tokio::test]
async fn test_get_tag_keys() {
    let service = TaggingService::new();

    // Tag two resources with overlapping keys
    send(
        &service,
        "TagResources",
        json!({
            "ResourceARNList": ["arn:aws:s3:::bucket-a"],
            "Tags": {"Environment": "Prod", "Team": "Backend"}
        }),
    )
    .await;

    send(
        &service,
        "TagResources",
        json!({
            "ResourceARNList": ["arn:aws:s3:::bucket-b"],
            "Tags": {"Environment": "Staging", "CostCenter": "12345"}
        }),
    )
    .await;

    let (status, body) = send(&service, "GetTagKeys", json!({})).await;

    assert_eq!(status, 200);
    let keys = body["TagKeys"].as_array().unwrap();
    let keys: Vec<&str> = keys.iter().map(|v| v.as_str().unwrap()).collect();
    assert_eq!(keys.len(), 3);
    assert!(keys.contains(&"CostCenter"));
    assert!(keys.contains(&"Environment"));
    assert!(keys.contains(&"Team"));
}

#[tokio::test]
async fn test_get_tag_values() {
    let service = TaggingService::new();

    for (arn, env) in [
        ("arn:aws:s3:::bucket-1", "Prod"),
        ("arn:aws:s3:::bucket-2", "Staging"),
        ("arn:aws:s3:::bucket-3", "Dev"),
    ] {
        send(
            &service,
            "TagResources",
            json!({
                "ResourceARNList": [arn],
                "Tags": {"Environment": env}
            }),
        )
        .await;
    }

    let (status, body) = send(&service, "GetTagValues", json!({"Key": "Environment"})).await;

    assert_eq!(status, 200);
    let values = body["TagValues"].as_array().unwrap();
    let values: Vec<&str> = values.iter().map(|v| v.as_str().unwrap()).collect();
    assert_eq!(values.len(), 3);
    assert!(values.contains(&"Dev"));
    assert!(values.contains(&"Prod"));
    assert!(values.contains(&"Staging"));
}

#[tokio::test]
async fn test_get_resources_with_tag_filter() {
    let service = TaggingService::new();

    send(
        &service,
        "TagResources",
        json!({
            "ResourceARNList": ["arn:aws:s3:::bucket-prod"],
            "Tags": {"Environment": "Prod"}
        }),
    )
    .await;

    send(
        &service,
        "TagResources",
        json!({
            "ResourceARNList": ["arn:aws:s3:::bucket-dev"],
            "Tags": {"Environment": "Dev"}
        }),
    )
    .await;

    // Filter by Environment=Prod
    let (status, body) = send(
        &service,
        "GetResources",
        json!({
            "TagFilters": [{"Key": "Environment", "Values": ["Prod"]}]
        }),
    )
    .await;

    assert_eq!(status, 200);
    let mappings = body["ResourceTagMappingList"].as_array().unwrap();
    assert_eq!(mappings.len(), 1);
    assert_eq!(mappings[0]["ResourceARN"], "arn:aws:s3:::bucket-prod");
}

#[tokio::test]
async fn test_untag_all_tags_removes_resource() {
    let service = TaggingService::new();

    send(
        &service,
        "TagResources",
        json!({
            "ResourceARNList": ["arn:aws:s3:::temp-bucket"],
            "Tags": {"Key1": "Value1"}
        }),
    )
    .await;

    // Untag all tags
    send(
        &service,
        "UntagResources",
        json!({
            "ResourceARNList": ["arn:aws:s3:::temp-bucket"],
            "TagKeys": ["Key1"]
        }),
    )
    .await;

    // Resource should no longer appear
    let (_, body) = send(&service, "GetResources", json!({})).await;
    assert!(
        body["ResourceTagMappingList"]
            .as_array()
            .unwrap()
            .is_empty()
    );
}

#[tokio::test]
async fn test_missing_target_header() {
    let service = TaggingService::new();

    let request = MockRequest {
        method: "POST".to_string(),
        uri: "https://tagging.us-east-1.amazonaws.com/".to_string(),
        headers: HeaderMap::new(),
        body: Bytes::from("{}"),
    };

    let response = service.handle(request).await;
    assert_eq!(response.status, 400);
    let body: Value = serde_json::from_slice(&response.body).unwrap();
    assert_eq!(body["__type"], "MissingAction");
}

#[tokio::test]
async fn test_unknown_action() {
    let service = TaggingService::new();

    let (status, body) = send(&service, "NonexistentAction", json!({})).await;
    assert_eq!(status, 400);
    assert_eq!(body["__type"], "InvalidAction");
}

#[tokio::test]
async fn test_tag_resources_missing_arns() {
    let service = TaggingService::new();

    let (status, body) = send(&service, "TagResources", json!({"Tags": {"Key": "Value"}})).await;
    assert_eq!(status, 400);
    assert_eq!(body["__type"], "InvalidParameterException");
}

#[tokio::test]
async fn test_tag_resources_missing_tags() {
    let service = TaggingService::new();

    let (status, body) = send(
        &service,
        "TagResources",
        json!({"ResourceARNList": ["arn:aws:s3:::bucket"]}),
    )
    .await;
    assert_eq!(status, 400);
    assert_eq!(body["__type"], "InvalidParameterException");
}

#[tokio::test]
async fn test_get_tag_values_missing_key() {
    let service = TaggingService::new();

    let (status, body) = send(&service, "GetTagValues", json!({})).await;
    assert_eq!(status, 400);
    assert_eq!(body["__type"], "InvalidParameterException");
}

#[tokio::test]
async fn test_tag_multiple_resources() {
    let service = TaggingService::new();

    let (status, body) = send(
        &service,
        "TagResources",
        json!({
            "ResourceARNList": [
                "arn:aws:s3:::bucket-1",
                "arn:aws:s3:::bucket-2",
                "arn:aws:s3:::bucket-3"
            ],
            "Tags": {"Project": "Alpha"}
        }),
    )
    .await;

    assert_eq!(status, 200);
    assert!(body["FailedResourcesMap"].as_object().unwrap().is_empty());

    // All three resources should appear
    let (_, body) = send(&service, "GetResources", json!({})).await;
    let mappings = body["ResourceTagMappingList"].as_array().unwrap();
    assert_eq!(mappings.len(), 3);
}

// ============================================================================
// Tests derived from AWS documentation: Resource Groups Tagging API
// ============================================================================

#[tokio::test]
async fn test_tag_resources_updates_existing_tags() {
    let service = TaggingService::new();
    let arn = "arn:aws:s3:::my-updatable-bucket";

    // Tag a resource with initial value
    let (status, body) = send(
        &service,
        "TagResources",
        json!({
            "ResourceARNList": [arn],
            "Tags": {"Env": "Dev"}
        }),
    )
    .await;
    assert_eq!(status, 200);
    assert!(body["FailedResourcesMap"].as_object().unwrap().is_empty());

    // Tag the same resource with an updated value for the same key
    let (status, body) = send(
        &service,
        "TagResources",
        json!({
            "ResourceARNList": [arn],
            "Tags": {"Env": "Prod"}
        }),
    )
    .await;
    assert_eq!(status, 200);
    assert!(body["FailedResourcesMap"].as_object().unwrap().is_empty());

    // Verify the tag value was updated (not duplicated)
    let (status, body) = send(
        &service,
        "GetResources",
        json!({"TagFilters": [{"Key": "Env", "Values": ["Prod"]}]}),
    )
    .await;
    assert_eq!(status, 200);
    let mappings = body["ResourceTagMappingList"].as_array().unwrap();
    assert_eq!(mappings.len(), 1);
    let tags = mappings[0]["Tags"].as_array().unwrap();
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0]["Key"], "Env");
    assert_eq!(tags[0]["Value"], "Prod");
}

#[tokio::test]
async fn test_tag_resources_empty_arn_in_list() {
    let service = TaggingService::new();

    // Include an empty string in the ARN list
    let (status, body) = send(
        &service,
        "TagResources",
        json!({
            "ResourceARNList": ["", "arn:aws:s3:::valid-bucket"],
            "Tags": {"Key": "Value"}
        }),
    )
    .await;
    assert_eq!(status, 200);

    // The empty ARN should appear in FailedResourcesMap
    let failed = body["FailedResourcesMap"].as_object().unwrap();
    assert!(
        failed.contains_key(""),
        "Expected empty ARN in FailedResourcesMap, got: {body:?}"
    );

    // The valid ARN should have been tagged successfully
    let (_, body) = send(
        &service,
        "GetResources",
        json!({"TagFilters": [{"Key": "Key", "Values": ["Value"]}]}),
    )
    .await;
    let mappings = body["ResourceTagMappingList"].as_array().unwrap();
    assert_eq!(mappings.len(), 1);
    assert_eq!(mappings[0]["ResourceARN"], "arn:aws:s3:::valid-bucket");
}

#[tokio::test]
async fn test_untag_resources_nonexistent_resource() {
    let service = TaggingService::new();

    // Untag an ARN that was never tagged — should succeed (idempotent per AWS docs)
    let (status, body) = send(
        &service,
        "UntagResources",
        json!({
            "ResourceARNList": ["arn:aws:s3:::never-tagged-bucket"],
            "TagKeys": ["SomeKey"]
        }),
    )
    .await;
    assert_eq!(status, 200);
    assert!(
        body["FailedResourcesMap"].as_object().unwrap().is_empty(),
        "Expected empty FailedResourcesMap for nonexistent resource, got: {body:?}"
    );
}

#[tokio::test]
async fn test_untag_resources_nonexistent_key() {
    let service = TaggingService::new();

    // Tag a resource first
    send(
        &service,
        "TagResources",
        json!({
            "ResourceARNList": ["arn:aws:s3:::my-bucket"],
            "Tags": {"ExistingKey": "Value"}
        }),
    )
    .await;

    // Untag a key that does not exist — should succeed (idempotent per AWS docs)
    let (status, body) = send(
        &service,
        "UntagResources",
        json!({
            "ResourceARNList": ["arn:aws:s3:::my-bucket"],
            "TagKeys": ["NonExistentKey"]
        }),
    )
    .await;
    assert_eq!(status, 200);
    assert!(
        body["FailedResourcesMap"].as_object().unwrap().is_empty(),
        "Expected empty FailedResourcesMap when key does not exist, got: {body:?}"
    );

    // The existing tag should still be there
    let (_, body) = send(&service, "GetResources", json!({})).await;
    let mappings = body["ResourceTagMappingList"].as_array().unwrap();
    assert_eq!(mappings.len(), 1);
    let tags = mappings[0]["Tags"].as_array().unwrap();
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0]["Key"], "ExistingKey");
}

#[tokio::test]
async fn test_untag_resources_missing_arns() {
    let service = TaggingService::new();

    let (status, body) = send(&service, "UntagResources", json!({"TagKeys": ["SomeKey"]})).await;
    assert_eq!(status, 400);
    assert_eq!(body["__type"], "InvalidParameterException");
}

#[tokio::test]
async fn test_untag_resources_missing_tag_keys() {
    let service = TaggingService::new();

    let (status, body) = send(
        &service,
        "UntagResources",
        json!({"ResourceARNList": ["arn:aws:s3:::some-bucket"]}),
    )
    .await;
    assert_eq!(status, 400);
    assert_eq!(body["__type"], "InvalidParameterException");
}

#[tokio::test]
async fn test_get_resources_with_resource_type_filter() {
    let service = TaggingService::new();

    // Tag an S3 bucket and an EC2 instance
    send(
        &service,
        "TagResources",
        json!({
            "ResourceARNList": ["arn:aws:s3:::my-bucket"],
            "Tags": {"Project": "Alpha"}
        }),
    )
    .await;
    send(
        &service,
        "TagResources",
        json!({
            "ResourceARNList": ["arn:aws:ec2:us-east-1:123456789012:instance/i-abc123"],
            "Tags": {"Project": "Alpha"}
        }),
    )
    .await;

    // Filter by "ec2" resource type
    let (status, body) = send(
        &service,
        "GetResources",
        json!({"ResourceTypeFilters": ["ec2"]}),
    )
    .await;
    assert_eq!(status, 200);
    let mappings = body["ResourceTagMappingList"].as_array().unwrap();
    assert_eq!(mappings.len(), 1);
    assert!(
        mappings[0]["ResourceARN"].as_str().unwrap().contains("ec2"),
        "Expected ec2 ARN, got: {:?}",
        mappings[0]["ResourceARN"]
    );
}

#[tokio::test]
async fn test_get_resources_tag_filter_key_only() {
    let service = TaggingService::new();

    // Tag two resources with different values for the same key
    send(
        &service,
        "TagResources",
        json!({
            "ResourceARNList": ["arn:aws:s3:::bucket-a", "arn:aws:s3:::bucket-b"],
            "Tags": {"Owner": "TeamA"}
        }),
    )
    .await;
    send(
        &service,
        "TagResources",
        json!({
            "ResourceARNList": ["arn:aws:s3:::bucket-c"],
            "Tags": {"Owner": "TeamB"}
        }),
    )
    .await;
    // Tag a resource without "Owner" key
    send(
        &service,
        "TagResources",
        json!({
            "ResourceARNList": ["arn:aws:s3:::bucket-no-owner"],
            "Tags": {"Other": "Value"}
        }),
    )
    .await;

    // Filter by key "Owner" with no values — should match any value
    let (status, body) = send(
        &service,
        "GetResources",
        json!({"TagFilters": [{"Key": "Owner", "Values": []}]}),
    )
    .await;
    assert_eq!(status, 200);
    let mappings = body["ResourceTagMappingList"].as_array().unwrap();
    assert_eq!(
        mappings.len(),
        3,
        "Expected 3 resources with Owner key, got: {mappings:?}"
    );
}

#[tokio::test]
async fn test_get_resources_multi_value_tag_filter() {
    let service = TaggingService::new();

    for (arn, env) in [
        ("arn:aws:s3:::bucket-prod", "Prod"),
        ("arn:aws:s3:::bucket-staging", "Staging"),
        ("arn:aws:s3:::bucket-dev", "Dev"),
    ] {
        send(
            &service,
            "TagResources",
            json!({
                "ResourceARNList": [arn],
                "Tags": {"Env": env}
            }),
        )
        .await;
    }

    // Filter for Prod OR Staging (OR semantics within a single TagFilter)
    let (status, body) = send(
        &service,
        "GetResources",
        json!({"TagFilters": [{"Key": "Env", "Values": ["Prod", "Staging"]}]}),
    )
    .await;
    assert_eq!(status, 200);
    let mappings = body["ResourceTagMappingList"].as_array().unwrap();
    assert_eq!(
        mappings.len(),
        2,
        "Expected 2 matching resources, got: {mappings:?}"
    );
    let arns: Vec<&str> = mappings
        .iter()
        .map(|m| m["ResourceARN"].as_str().unwrap())
        .collect();
    assert!(arns.contains(&"arn:aws:s3:::bucket-prod"));
    assert!(arns.contains(&"arn:aws:s3:::bucket-staging"));
}

#[tokio::test]
async fn test_get_resources_multiple_filters() {
    let service = TaggingService::new();

    // Resource with both keys
    send(
        &service,
        "TagResources",
        json!({
            "ResourceARNList": ["arn:aws:s3:::bucket-both"],
            "Tags": {"Env": "Prod", "Team": "Backend"}
        }),
    )
    .await;
    // Resource with only Env key
    send(
        &service,
        "TagResources",
        json!({
            "ResourceARNList": ["arn:aws:s3:::bucket-env-only"],
            "Tags": {"Env": "Prod"}
        }),
    )
    .await;
    // Resource with only Team key
    send(
        &service,
        "TagResources",
        json!({
            "ResourceARNList": ["arn:aws:s3:::bucket-team-only"],
            "Tags": {"Team": "Backend"}
        }),
    )
    .await;

    // Filter by both Env=Prod AND Team=Backend (AND semantics across multiple TagFilters)
    let (status, body) = send(
        &service,
        "GetResources",
        json!({
            "TagFilters": [
                {"Key": "Env", "Values": ["Prod"]},
                {"Key": "Team", "Values": ["Backend"]}
            ]
        }),
    )
    .await;
    assert_eq!(status, 200);
    let mappings = body["ResourceTagMappingList"].as_array().unwrap();
    assert_eq!(
        mappings.len(),
        1,
        "Expected only the resource with both tags, got: {mappings:?}"
    );
    assert_eq!(mappings[0]["ResourceARN"], "arn:aws:s3:::bucket-both");
}

#[tokio::test]
async fn test_get_resources_by_arn_list() {
    let service = TaggingService::new();

    for arn in [
        "arn:aws:s3:::bucket-x",
        "arn:aws:s3:::bucket-y",
        "arn:aws:s3:::bucket-z",
    ] {
        send(
            &service,
            "TagResources",
            json!({
                "ResourceARNList": [arn],
                "Tags": {"Project": "Test"}
            }),
        )
        .await;
    }

    // Fetch only two specific ARNs via ResourceARNList
    let (status, body) = send(
        &service,
        "GetResources",
        json!({
            "ResourceARNList": [
                "arn:aws:s3:::bucket-x",
                "arn:aws:s3:::bucket-z"
            ]
        }),
    )
    .await;
    assert_eq!(status, 200);
    let mappings = body["ResourceTagMappingList"].as_array().unwrap();
    assert_eq!(
        mappings.len(),
        2,
        "Expected exactly 2 resources, got: {mappings:?}"
    );
    let arns: Vec<&str> = mappings
        .iter()
        .map(|m| m["ResourceARN"].as_str().unwrap())
        .collect();
    assert!(arns.contains(&"arn:aws:s3:::bucket-x"));
    assert!(arns.contains(&"arn:aws:s3:::bucket-z"));
}

#[tokio::test]
async fn test_get_tag_keys_empty() {
    let service = TaggingService::new();

    // No resources tagged — TagKeys should be empty
    let (status, body) = send(&service, "GetTagKeys", json!({})).await;
    assert_eq!(status, 200);
    let keys = body["TagKeys"].as_array().unwrap();
    assert!(
        keys.is_empty(),
        "Expected empty TagKeys for new service, got: {keys:?}"
    );
}

#[tokio::test]
async fn test_get_tag_values_no_resources_with_key() {
    let service = TaggingService::new();

    // Tag a resource with a different key
    send(
        &service,
        "TagResources",
        json!({
            "ResourceARNList": ["arn:aws:s3:::bucket-a"],
            "Tags": {"OtherKey": "OtherValue"}
        }),
    )
    .await;

    // Query for a key that does not exist — should return empty list, not an error
    let (status, body) = send(&service, "GetTagValues", json!({"Key": "NonExistentKey"})).await;
    assert_eq!(status, 200);
    let values = body["TagValues"].as_array().unwrap();
    assert!(
        values.is_empty(),
        "Expected empty TagValues for unknown key, got: {values:?}"
    );
}

#[tokio::test]
async fn test_lifecycle_full() {
    let service = TaggingService::new();
    let arn = "arn:aws:ec2:us-east-1:123456789012:instance/i-lifecycle";

    // Step 1: Tag the resource
    let (status, body) = send(
        &service,
        "TagResources",
        json!({
            "ResourceARNList": [arn],
            "Tags": {"Stage": "Alpha", "Owner": "team-a"}
        }),
    )
    .await;
    assert_eq!(status, 200);
    assert!(body["FailedResourcesMap"].as_object().unwrap().is_empty());

    // Step 2: Verify it appears in GetResources
    let (status, body) = send(
        &service,
        "GetResources",
        json!({"TagFilters": [{"Key": "Stage", "Values": ["Alpha"]}]}),
    )
    .await;
    assert_eq!(status, 200);
    let mappings = body["ResourceTagMappingList"].as_array().unwrap();
    assert_eq!(mappings.len(), 1);
    assert_eq!(mappings[0]["ResourceARN"], arn);
    assert_eq!(mappings[0]["Tags"].as_array().unwrap().len(), 2);

    // Step 3: Verify keys and values
    let (status, body) = send(&service, "GetTagKeys", json!({})).await;
    assert_eq!(status, 200);
    let keys: Vec<&str> = body["TagKeys"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_str().unwrap())
        .collect();
    assert!(keys.contains(&"Stage"));
    assert!(keys.contains(&"Owner"));

    let (status, body) = send(&service, "GetTagValues", json!({"Key": "Stage"})).await;
    assert_eq!(status, 200);
    let values: Vec<&str> = body["TagValues"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_str().unwrap())
        .collect();
    assert!(values.contains(&"Alpha"));

    // Step 4: Update one tag
    let (status, body) = send(
        &service,
        "TagResources",
        json!({
            "ResourceARNList": [arn],
            "Tags": {"Stage": "Beta"}
        }),
    )
    .await;
    assert_eq!(status, 200);
    assert!(body["FailedResourcesMap"].as_object().unwrap().is_empty());

    let (status, body) = send(&service, "GetTagValues", json!({"Key": "Stage"})).await;
    assert_eq!(status, 200);
    let values: Vec<&str> = body["TagValues"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_str().unwrap())
        .collect();
    assert!(values.contains(&"Beta"));
    assert!(!values.contains(&"Alpha"), "Old value Alpha should be gone");

    // Step 5: Untag all keys
    let (status, body) = send(
        &service,
        "UntagResources",
        json!({
            "ResourceARNList": [arn],
            "TagKeys": ["Stage", "Owner"]
        }),
    )
    .await;
    assert_eq!(status, 200);
    assert!(body["FailedResourcesMap"].as_object().unwrap().is_empty());

    // Step 6: Verify resource is gone from GetResources (no tags remain)
    let (status, body) = send(&service, "GetResources", json!({})).await;
    assert_eq!(status, 200);
    assert!(
        body["ResourceTagMappingList"]
            .as_array()
            .unwrap()
            .is_empty(),
        "Expected no resources after untagging all keys"
    );

    // Step 7: GetTagKeys should now be empty
    let (status, body) = send(&service, "GetTagKeys", json!({})).await;
    assert_eq!(status, 200);
    assert!(body["TagKeys"].as_array().unwrap().is_empty());
}

// ============================================================================
// SDK-style integration tests (aws-sdk-resourcegroupstagging)
// ============================================================================

use aws_sdk_resourcegroupstagging::config::BehaviorVersion;
use winterbaume_core::MockAws;

async fn make_client() -> aws_sdk_resourcegroupstagging::Client {
    let mock = MockAws::builder()
        .with_service(TaggingService::new())
        .build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_resourcegroupstagging::config::Region::new(
            "us-east-1",
        ))
        .load()
        .await;
    aws_sdk_resourcegroupstagging::Client::new(&config)
}

#[tokio::test]
async fn sdk_tag_resources() {
    let client = make_client().await;

    let resp = client
        .tag_resources()
        .resource_arn_list("arn:aws:s3:::sdk-bucket-1")
        .resource_arn_list("arn:aws:s3:::sdk-bucket-2")
        .tags("Project", "SDK-Test")
        .tags("Env", "CI")
        .send()
        .await
        .expect("tag_resources should succeed");

    let failed = resp.failed_resources_map();
    assert!(
        failed.is_none() || failed.unwrap().is_empty(),
        "Expected no failures, got: {failed:?}"
    );
}

#[tokio::test]
async fn sdk_untag_resources() {
    let client = make_client().await;

    // First tag a resource
    client
        .tag_resources()
        .resource_arn_list("arn:aws:s3:::sdk-untag-bucket")
        .tags("KeyA", "ValueA")
        .tags("KeyB", "ValueB")
        .send()
        .await
        .expect("tag_resources should succeed");

    // Untag one key
    let resp = client
        .untag_resources()
        .resource_arn_list("arn:aws:s3:::sdk-untag-bucket")
        .tag_keys("KeyA")
        .send()
        .await
        .expect("untag_resources should succeed");

    let failed = resp.failed_resources_map();
    assert!(
        failed.is_none() || failed.unwrap().is_empty(),
        "Expected no failures, got: {failed:?}"
    );

    // Verify only KeyB remains
    let get_resp = client
        .get_resources()
        .send()
        .await
        .expect("get_resources should succeed");

    let mappings = get_resp.resource_tag_mapping_list();
    assert_eq!(mappings.len(), 1);
    let tags = mappings[0].tags();
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0].key(), "KeyB");
    assert_eq!(tags[0].value(), "ValueB");
}

#[tokio::test]
async fn sdk_get_resources() {
    let client = make_client().await;

    // Tag two resources
    client
        .tag_resources()
        .resource_arn_list("arn:aws:s3:::sdk-res-a")
        .tags("Color", "Red")
        .send()
        .await
        .expect("tag_resources should succeed");
    client
        .tag_resources()
        .resource_arn_list("arn:aws:ec2:us-east-1:123456789012:instance/i-sdk1")
        .tags("Color", "Blue")
        .send()
        .await
        .expect("tag_resources should succeed");

    let resp = client
        .get_resources()
        .send()
        .await
        .expect("get_resources should succeed");

    let mappings = resp.resource_tag_mapping_list();
    assert_eq!(mappings.len(), 2);
    let arns: Vec<&str> = mappings
        .iter()
        .map(|m| m.resource_arn().unwrap_or(""))
        .collect();
    assert!(arns.contains(&"arn:aws:s3:::sdk-res-a"));
    assert!(arns.contains(&"arn:aws:ec2:us-east-1:123456789012:instance/i-sdk1"));
}

#[tokio::test]
async fn sdk_get_tag_keys() {
    let client = make_client().await;

    client
        .tag_resources()
        .resource_arn_list("arn:aws:s3:::sdk-keys-bucket")
        .tags("Alpha", "1")
        .tags("Beta", "2")
        .tags("Gamma", "3")
        .send()
        .await
        .expect("tag_resources should succeed");

    let resp = client
        .get_tag_keys()
        .send()
        .await
        .expect("get_tag_keys should succeed");

    let keys = resp.tag_keys();
    assert_eq!(keys.len(), 3);
    assert!(keys.contains(&"Alpha".to_string()));
    assert!(keys.contains(&"Beta".to_string()));
    assert!(keys.contains(&"Gamma".to_string()));
}

#[tokio::test]
async fn sdk_get_tag_values() {
    let client = make_client().await;

    for (arn, val) in [
        ("arn:aws:s3:::sdk-val-1", "Low"),
        ("arn:aws:s3:::sdk-val-2", "Medium"),
        ("arn:aws:s3:::sdk-val-3", "High"),
    ] {
        client
            .tag_resources()
            .resource_arn_list(arn)
            .tags("Priority", val)
            .send()
            .await
            .expect("tag_resources should succeed");
    }

    let resp = client
        .get_tag_values()
        .key("Priority")
        .send()
        .await
        .expect("get_tag_values should succeed");

    let values = resp.tag_values();
    assert_eq!(values.len(), 3);
    assert!(values.contains(&"Low".to_string()));
    assert!(values.contains(&"Medium".to_string()));
    assert!(values.contains(&"High".to_string()));
}
