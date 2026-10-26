use std::sync::Arc;

use serde_json::{Value, json};
use winterbaume_cloudcontrol::CloudControlService;
use winterbaume_core::{MockAws, default_account_id};
use winterbaume_ssm::SsmService;

const TYPE: &str = "AWS::SSM::Parameter";

async fn clients(region: &str) -> (aws_sdk_cloudcontrol::Client, aws_sdk_ssm::Client) {
    let ssm = Arc::new(SsmService::new());
    let cloudcontrol = CloudControlService::with_ssm_service(Arc::clone(&ssm));
    let mock = MockAws::builder()
        .with_service(ssm)
        .with_service(cloudcontrol)
        .build();
    let config = mock.sdk_config(region).await;
    (
        aws_sdk_cloudcontrol::Client::new(&config),
        aws_sdk_ssm::Client::new(&config),
    )
}

async fn properties(client: &aws_sdk_cloudcontrol::Client, name: &str) -> Value {
    let result = client
        .get_resource()
        .type_name(TYPE)
        .identifier(name)
        .send()
        .await
        .unwrap();
    serde_json::from_str(result.resource_description().unwrap().properties().unwrap()).unwrap()
}

#[tokio::test]
async fn ssm_parameter_shared_lifecycle() {
    let (cc, ssm) = clients("us-east-1").await;
    let name = "/carina-awscc-459/probe";
    let created = cc
        .create_resource()
        .type_name(TYPE)
        .desired_state(
            json!({
                "Name": name, "Type": "String", "Value": "v1", "Description": "probe",
                "Tier": "Standard", "AllowedPattern": ".*", "Policies": "[]",
                "Tags": {"env": "dev", "owner": "carina"}
            })
            .to_string(),
        )
        .send()
        .await
        .unwrap();
    let event = created.progress_event().unwrap();
    assert_eq!(event.identifier(), Some(name));
    assert_eq!(event.operation_status().unwrap().as_str(), "SUCCESS");
    let expected = json!({
        "Name": name, "Type": "String", "Value": "v1", "DataType": "text",
        "Arn": format!("arn:aws:ssm:us-east-1:{}:parameter{name}", default_account_id()),
        "Tags": {"env": "dev", "owner": "carina"}
    });
    assert_eq!(properties(&cc, name).await, expected);
    assert_eq!(
        serde_json::from_str::<Value>(event.resource_model().unwrap()).unwrap(),
        expected
    );
    let status = cc
        .get_resource_request_status()
        .request_token(event.request_token().unwrap())
        .send()
        .await
        .unwrap();
    assert_eq!(
        status.progress_event().unwrap().resource_model(),
        event.resource_model()
    );
    let parameter = ssm.get_parameter().name(name).send().await.unwrap();
    let parameter = parameter.parameter().unwrap();
    assert_eq!(parameter.value(), Some("v1"));
    assert_eq!(parameter.arn(), expected["Arn"].as_str());
    let tags = ssm
        .list_tags_for_resource()
        .resource_type(aws_sdk_ssm::types::ResourceTypeForTagging::Parameter)
        .resource_id(name)
        .send()
        .await
        .unwrap();
    assert_eq!(tags.tag_list().len(), 2);

    // A direct SSM write must be visible without another Cloud Control mutation.
    ssm.put_parameter()
        .name(name)
        .value("v3")
        .overwrite(true)
        .send()
        .await
        .unwrap();
    let current = properties(&cc, name).await;
    assert_eq!(current["Value"], "v3");
    assert_eq!(current["Tags"], expected["Tags"]);
    cc.update_resource()
        .type_name(TYPE)
        .identifier(name)
        .patch_document(
            json!([
                {"op": "replace", "path": "/Value", "value": "v4"},
                {"op": "replace", "path": "/Tags", "value": {"env": "prod"}},
                {"op": "add", "path": "/Description", "value": "still write-only"}
            ])
            .to_string(),
        )
        .send()
        .await
        .unwrap();
    assert_eq!(
        ssm.get_parameter()
            .name(name)
            .send()
            .await
            .unwrap()
            .parameter()
            .unwrap()
            .value(),
        Some("v4")
    );
    let current = properties(&cc, name).await;
    assert_eq!(current["Tags"], json!({"env": "prod"}));
    assert!(current.get("Description").is_none());
    let tags = ssm
        .list_tags_for_resource()
        .resource_type(aws_sdk_ssm::types::ResourceTypeForTagging::Parameter)
        .resource_id(name)
        .send()
        .await
        .unwrap();
    assert_eq!(tags.tag_list().len(), 1);
    assert_eq!(tags.tag_list()[0].value(), "prod");

    // Nested tag patches and direct tag writes share the same live model.
    cc.update_resource()
        .type_name(TYPE)
        .identifier(name)
        .patch_document(r#"[{"op":"add","path":"/Tags/team~1owner","value":"ops"}]"#)
        .send()
        .await
        .unwrap();
    ssm.remove_tags_from_resource()
        .resource_type(aws_sdk_ssm::types::ResourceTypeForTagging::Parameter)
        .resource_id(name)
        .tag_keys("env")
        .send()
        .await
        .unwrap();
    assert_eq!(
        properties(&cc, name).await["Tags"],
        json!({"team/owner": "ops"})
    );
    let listed = cc.list_resources().type_name(TYPE).send().await.unwrap();
    assert_eq!(listed.resource_descriptions().len(), 1);
    let listed: Value =
        serde_json::from_str(listed.resource_descriptions()[0].properties().unwrap()).unwrap();
    assert_eq!(listed, properties(&cc, name).await);

    cc.delete_resource()
        .type_name(TYPE)
        .identifier(name)
        .send()
        .await
        .unwrap();
    let err = ssm.get_parameter().name(name).send().await.unwrap_err();
    assert!(err.as_service_error().unwrap().is_parameter_not_found());
    assert!(
        cc.list_resources()
            .type_name(TYPE)
            .send()
            .await
            .unwrap()
            .resource_descriptions()
            .is_empty()
    );
    assert!(
        ssm.list_tags_for_resource()
            .resource_type(aws_sdk_ssm::types::ResourceTypeForTagging::Parameter)
            .resource_id(name)
            .send()
            .await
            .unwrap()
            .tag_list()
            .is_empty()
    );
}

#[tokio::test]
async fn ssm_parameter_reads_native_resources_and_rejects_duplicate_creates() {
    let (cc, ssm) = clients("eu-west-1").await;
    ssm.put_parameter()
        .name("native")
        .r#type(aws_sdk_ssm::types::ParameterType::StringList)
        .value("one,two")
        .send()
        .await
        .unwrap();
    let current = properties(&cc, "native").await;
    let other_region = aws_sdk_cloudcontrol::Client::from_conf(
        cc.config()
            .to_builder()
            .region(aws_sdk_cloudcontrol::config::Region::new("us-west-2"))
            .build(),
    );
    let err = other_region
        .get_resource()
        .type_name(TYPE)
        .identifier("native")
        .send()
        .await
        .unwrap_err();
    assert!(
        err.as_service_error()
            .unwrap()
            .is_resource_not_found_exception()
    );
    assert_eq!(current["Type"], "StringList");
    assert_eq!(
        current["Arn"],
        format!(
            "arn:aws:ssm:eu-west-1:{}:parameter/native",
            default_account_id()
        )
    );
    let err = cc
        .create_resource()
        .type_name(TYPE)
        .desired_state(r#"{"Name":"native","Type":"String","Value":"overwrite"}"#)
        .send()
        .await
        .unwrap_err();
    assert!(
        err.as_service_error()
            .unwrap()
            .is_already_exists_exception()
    );
    assert_eq!(properties(&cc, "native").await["Value"], "one,two");
    ssm.delete_parameter().name("native").send().await.unwrap();
    let err = cc
        .get_resource()
        .type_name(TYPE)
        .identifier("native")
        .send()
        .await
        .unwrap_err();
    assert!(
        err.as_service_error()
            .unwrap()
            .is_resource_not_found_exception()
    );
}

#[tokio::test]
async fn ssm_parameter_defaults_validation_and_failed_update_are_atomic() {
    let (cc, ssm) = clients("us-east-1").await;
    for desired in [
        json!({"Type":"SecureString", "Value":"secret"}),
        json!({"Type":"String"}),
        json!({"Type":"String", "Value":"v", "Tags":[{"Key":"bad","Value":"shape"}]}),
    ] {
        let err = cc
            .create_resource()
            .type_name(TYPE)
            .desired_state(desired.to_string())
            .send()
            .await
            .unwrap_err();
        assert!(
            err.as_service_error()
                .unwrap()
                .is_invalid_request_exception()
        );
    }
    let created = cc
        .create_resource()
        .type_name(TYPE)
        .desired_state(r#"{"Type":"String","Value":"v1","DataType":"aws:ec2:image"}"#)
        .send()
        .await
        .unwrap();
    let name = created.progress_event().unwrap().identifier().unwrap();
    let original = properties(&cc, name).await;
    assert_eq!(original["Name"], name);
    assert_eq!(original["DataType"], "aws:ec2:image");
    assert!(original["Tags"].is_object());
    for patch in [
        r#"[{"op":"replace","path":"/Value","value":"bad"},{"op":"replace","path":"/Name","value":"renamed"}]"#,
        r#"[{"op":"replace","path":"/Arn","value":"forged"}]"#,
        r#"[{"op":"remove","path":"/Value"}]"#,
        r#"[{"op":"test","path":"/Description"}]"#,
        r#"[{"op":"test","path":"/Value","value":"wrong"}]"#,
        "invalid JSON",
    ] {
        let err = cc
            .update_resource()
            .type_name(TYPE)
            .identifier(name)
            .patch_document(patch)
            .send()
            .await
            .unwrap_err();
        assert!(
            err.as_service_error()
                .unwrap()
                .is_invalid_request_exception()
        );
        assert_eq!(properties(&cc, name).await, original);
        assert_eq!(
            ssm.get_parameter()
                .name(name)
                .send()
                .await
                .unwrap()
                .parameter()
                .unwrap()
                .version(),
            1
        );
    }
    // A tag-only update must not create a new parameter version.
    cc.update_resource()
        .type_name(TYPE)
        .identifier(name)
        .patch_document(r#"[{"op":"add","path":"/Tags/env","value":"test"}]"#)
        .send()
        .await
        .unwrap();
    assert_eq!(
        ssm.get_parameter()
            .name(name)
            .send()
            .await
            .unwrap()
            .parameter()
            .unwrap()
            .version(),
        1
    );
    ssm.delete_parameter().name(name).send().await.unwrap();
    let err = cc
        .get_resource()
        .type_name(TYPE)
        .identifier(name)
        .send()
        .await
        .unwrap_err();
    assert!(
        err.as_service_error()
            .unwrap()
            .is_resource_not_found_exception()
    );
}
