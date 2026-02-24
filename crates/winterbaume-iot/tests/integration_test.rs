use aws_sdk_iot::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_iot::IotService;

async fn make_iot_client() -> aws_sdk_iot::Client {
    let mock = MockAws::builder().with_service(IotService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_iot::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_iot::Client::new(&config)
}

// ==================== Thing ====================

#[tokio::test]
async fn test_create_and_describe_thing() {
    let client = make_iot_client().await;

    let create_resp = client
        .create_thing()
        .thing_name("my-sensor")
        .send()
        .await
        .expect("create_thing should succeed");

    assert_eq!(create_resp.thing_name().unwrap(), "my-sensor");
    assert!(create_resp.thing_arn().unwrap().contains("my-sensor"));
    assert!(create_resp.thing_id().is_some());

    let describe_resp = client
        .describe_thing()
        .thing_name("my-sensor")
        .send()
        .await
        .expect("describe_thing should succeed");

    assert_eq!(describe_resp.thing_name().unwrap(), "my-sensor");
    assert_eq!(describe_resp.version(), 1);
    assert_eq!(describe_resp.default_client_id().unwrap(), "my-sensor");
}

#[tokio::test]
async fn test_create_thing_with_attributes() {
    let client = make_iot_client().await;

    let attrs = aws_sdk_iot::types::AttributePayload::builder()
        .attributes("location", "warehouse-1")
        .attributes("firmware", "v2.1")
        .build();

    let create_resp = client
        .create_thing()
        .thing_name("sensor-with-attrs")
        .attribute_payload(attrs)
        .send()
        .await
        .expect("create_thing with attributes should succeed");

    assert_eq!(create_resp.thing_name().unwrap(), "sensor-with-attrs");

    let describe_resp = client
        .describe_thing()
        .thing_name("sensor-with-attrs")
        .send()
        .await
        .expect("describe_thing should succeed");

    let attributes = describe_resp
        .attributes()
        .expect("attributes should be present");
    assert_eq!(
        attributes.get("location").map(|s| s.as_str()),
        Some("warehouse-1")
    );
    assert_eq!(attributes.get("firmware").map(|s| s.as_str()), Some("v2.1"));
}

#[tokio::test]
async fn test_update_thing() {
    let client = make_iot_client().await;

    client
        .create_thing()
        .thing_name("upd-thing")
        .send()
        .await
        .unwrap();

    let attrs = aws_sdk_iot::types::AttributePayload::builder()
        .attributes("key", "val")
        .build();

    client
        .update_thing()
        .thing_name("upd-thing")
        .attribute_payload(attrs)
        .send()
        .await
        .expect("update_thing should succeed");

    let desc = client
        .describe_thing()
        .thing_name("upd-thing")
        .send()
        .await
        .unwrap();
    assert_eq!(desc.version(), 2);
    assert_eq!(
        desc.attributes().unwrap().get("key").map(|s| s.as_str()),
        Some("val")
    );
}

#[tokio::test]
async fn test_delete_thing() {
    let client = make_iot_client().await;

    client
        .create_thing()
        .thing_name("to-delete")
        .send()
        .await
        .unwrap();

    client
        .delete_thing()
        .thing_name("to-delete")
        .send()
        .await
        .expect("delete_thing should succeed");

    let result = client.describe_thing().thing_name("to-delete").send().await;
    assert!(result.is_err(), "describe after delete should fail");
}

#[tokio::test]
async fn test_list_things() {
    let client = make_iot_client().await;

    client
        .create_thing()
        .thing_name("thing-a")
        .send()
        .await
        .unwrap();

    client
        .create_thing()
        .thing_name("thing-b")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_things()
        .send()
        .await
        .expect("list_things should succeed");

    let things = resp.things();
    assert_eq!(things.len(), 2);

    let names: Vec<&str> = things.iter().map(|t| t.thing_name().unwrap()).collect();
    assert!(names.contains(&"thing-a"));
    assert!(names.contains(&"thing-b"));
}

#[tokio::test]
async fn test_create_duplicate_thing_fails() {
    let client = make_iot_client().await;

    client
        .create_thing()
        .thing_name("dup-thing")
        .send()
        .await
        .unwrap();

    let result = client.create_thing().thing_name("dup-thing").send().await;
    assert!(result.is_err(), "creating duplicate thing should fail");
}

#[tokio::test]
async fn test_describe_nonexistent_thing_fails() {
    let client = make_iot_client().await;

    let result = client
        .describe_thing()
        .thing_name("no-such-thing")
        .send()
        .await;
    assert!(result.is_err(), "describing nonexistent thing should fail");
}

// ==================== ThingType ====================

#[tokio::test]
async fn test_create_and_describe_thing_type() {
    let client = make_iot_client().await;

    let props = aws_sdk_iot::types::ThingTypeProperties::builder()
        .thing_type_description("A sensor type")
        .searchable_attributes("attr1")
        .build();

    let resp = client
        .create_thing_type()
        .thing_type_name("SensorType")
        .thing_type_properties(props)
        .send()
        .await
        .expect("create_thing_type should succeed");
    assert_eq!(resp.thing_type_name().unwrap(), "SensorType");
    assert!(resp.thing_type_arn().unwrap().contains("SensorType"));

    let desc = client
        .describe_thing_type()
        .thing_type_name("SensorType")
        .send()
        .await
        .expect("describe_thing_type should succeed");
    assert_eq!(desc.thing_type_name().unwrap(), "SensorType");
    let tt_props = desc.thing_type_properties().unwrap();
    assert_eq!(tt_props.thing_type_description().unwrap(), "A sensor type");
}

#[tokio::test]
async fn test_list_thing_types() {
    let client = make_iot_client().await;

    client
        .create_thing_type()
        .thing_type_name("TypeA")
        .send()
        .await
        .unwrap();
    client
        .create_thing_type()
        .thing_type_name("TypeB")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_thing_types()
        .send()
        .await
        .expect("list_thing_types should succeed");
    assert_eq!(resp.thing_types().len(), 2);
}

#[tokio::test]
async fn test_delete_thing_type() {
    let client = make_iot_client().await;

    client
        .create_thing_type()
        .thing_type_name("ToDelete")
        .send()
        .await
        .unwrap();
    client
        .delete_thing_type()
        .thing_type_name("ToDelete")
        .send()
        .await
        .expect("delete should succeed");
    let result = client
        .describe_thing_type()
        .thing_type_name("ToDelete")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_deprecate_thing_type() {
    let client = make_iot_client().await;

    client
        .create_thing_type()
        .thing_type_name("DepType")
        .send()
        .await
        .unwrap();

    client
        .deprecate_thing_type()
        .thing_type_name("DepType")
        .send()
        .await
        .expect("deprecate should succeed");

    let desc = client
        .describe_thing_type()
        .thing_type_name("DepType")
        .send()
        .await
        .unwrap();
    let meta = desc.thing_type_metadata().unwrap();
    assert!(meta.deprecated());

    // Undo deprecation
    client
        .deprecate_thing_type()
        .thing_type_name("DepType")
        .undo_deprecate(true)
        .send()
        .await
        .unwrap();
    let desc2 = client
        .describe_thing_type()
        .thing_type_name("DepType")
        .send()
        .await
        .unwrap();
    assert!(!desc2.thing_type_metadata().unwrap().deprecated());
}

// ==================== ThingGroup ====================

#[tokio::test]
async fn test_create_and_describe_thing_group() {
    let client = make_iot_client().await;

    let props = aws_sdk_iot::types::ThingGroupProperties::builder()
        .thing_group_description("A group")
        .build();

    let resp = client
        .create_thing_group()
        .thing_group_name("my-group")
        .thing_group_properties(props)
        .send()
        .await
        .expect("create_thing_group should succeed");
    assert_eq!(resp.thing_group_name().unwrap(), "my-group");

    let desc = client
        .describe_thing_group()
        .thing_group_name("my-group")
        .send()
        .await
        .unwrap();
    assert_eq!(desc.thing_group_name().unwrap(), "my-group");
    assert_eq!(desc.version(), 1);
}

#[tokio::test]
async fn test_list_thing_groups() {
    let client = make_iot_client().await;

    client
        .create_thing_group()
        .thing_group_name("grp-a")
        .send()
        .await
        .unwrap();
    client
        .create_thing_group()
        .thing_group_name("grp-b")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_thing_groups()
        .send()
        .await
        .expect("list_thing_groups should succeed");
    assert_eq!(resp.thing_groups().len(), 2);
}

#[tokio::test]
async fn test_update_thing_group() {
    let client = make_iot_client().await;

    client
        .create_thing_group()
        .thing_group_name("upd-grp")
        .send()
        .await
        .unwrap();

    let new_props = aws_sdk_iot::types::ThingGroupProperties::builder()
        .thing_group_description("Updated")
        .build();

    let resp = client
        .update_thing_group()
        .thing_group_name("upd-grp")
        .thing_group_properties(new_props)
        .send()
        .await
        .expect("update_thing_group should succeed");
    assert_eq!(resp.version(), 2);
}

#[tokio::test]
async fn test_delete_thing_group() {
    let client = make_iot_client().await;

    client
        .create_thing_group()
        .thing_group_name("del-grp")
        .send()
        .await
        .unwrap();
    client
        .delete_thing_group()
        .thing_group_name("del-grp")
        .send()
        .await
        .expect("delete should succeed");

    let result = client
        .describe_thing_group()
        .thing_group_name("del-grp")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_add_remove_thing_to_thing_group() {
    let client = make_iot_client().await;

    client.create_thing().thing_name("t1").send().await.unwrap();
    client
        .create_thing_group()
        .thing_group_name("g1")
        .send()
        .await
        .unwrap();

    client
        .add_thing_to_thing_group()
        .thing_group_name("g1")
        .thing_name("t1")
        .send()
        .await
        .expect("add_thing_to_thing_group should succeed");

    let resp = client
        .list_things_in_thing_group()
        .thing_group_name("g1")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.things().len(), 1);
    assert_eq!(resp.things()[0], "t1");

    // ListThingGroupsForThing
    let grp_resp = client
        .list_thing_groups_for_thing()
        .thing_name("t1")
        .send()
        .await
        .unwrap();
    assert_eq!(grp_resp.thing_groups().len(), 1);

    // RemoveThingFromThingGroup
    client
        .remove_thing_from_thing_group()
        .thing_group_name("g1")
        .thing_name("t1")
        .send()
        .await
        .expect("remove should succeed");

    let resp2 = client
        .list_things_in_thing_group()
        .thing_group_name("g1")
        .send()
        .await
        .unwrap();
    assert!(resp2.things().is_empty());
}

#[tokio::test]
async fn test_update_thing_groups_for_thing() {
    let client = make_iot_client().await;

    client
        .create_thing()
        .thing_name("t-utg")
        .send()
        .await
        .unwrap();
    client
        .create_thing_group()
        .thing_group_name("g-utg-a")
        .send()
        .await
        .unwrap();
    client
        .create_thing_group()
        .thing_group_name("g-utg-b")
        .send()
        .await
        .unwrap();

    client
        .update_thing_groups_for_thing()
        .thing_name("t-utg")
        .thing_groups_to_add("g-utg-a")
        .thing_groups_to_add("g-utg-b")
        .send()
        .await
        .expect("update_thing_groups_for_thing should succeed");

    let grps = client
        .list_thing_groups_for_thing()
        .thing_name("t-utg")
        .send()
        .await
        .unwrap();
    assert_eq!(grps.thing_groups().len(), 2);

    // Remove one group
    client
        .update_thing_groups_for_thing()
        .thing_name("t-utg")
        .thing_groups_to_remove("g-utg-a")
        .send()
        .await
        .unwrap();

    let grps2 = client
        .list_thing_groups_for_thing()
        .thing_name("t-utg")
        .send()
        .await
        .unwrap();
    assert_eq!(grps2.thing_groups().len(), 1);
}

// ==================== BillingGroup ====================

#[tokio::test]
async fn test_create_and_describe_billing_group() {
    let client = make_iot_client().await;

    let props = aws_sdk_iot::types::BillingGroupProperties::builder()
        .billing_group_description("billing desc")
        .build();

    let resp = client
        .create_billing_group()
        .billing_group_name("bg1")
        .billing_group_properties(props)
        .send()
        .await
        .expect("create_billing_group should succeed");
    assert_eq!(resp.billing_group_name().unwrap(), "bg1");

    let desc = client
        .describe_billing_group()
        .billing_group_name("bg1")
        .send()
        .await
        .unwrap();
    assert_eq!(desc.billing_group_name().unwrap(), "bg1");
    assert_eq!(desc.version(), 1);
    assert_eq!(
        desc.billing_group_properties()
            .unwrap()
            .billing_group_description()
            .unwrap(),
        "billing desc"
    );
}

#[tokio::test]
async fn test_list_billing_groups() {
    let client = make_iot_client().await;

    client
        .create_billing_group()
        .billing_group_name("bg-a")
        .send()
        .await
        .unwrap();
    client
        .create_billing_group()
        .billing_group_name("bg-b")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_billing_groups()
        .send()
        .await
        .expect("list should succeed");
    assert_eq!(resp.billing_groups().len(), 2);
}

#[tokio::test]
async fn test_update_billing_group() {
    let client = make_iot_client().await;

    client
        .create_billing_group()
        .billing_group_name("bg-upd")
        .send()
        .await
        .unwrap();

    let new_props = aws_sdk_iot::types::BillingGroupProperties::builder()
        .billing_group_description("updated desc")
        .build();

    let resp = client
        .update_billing_group()
        .billing_group_name("bg-upd")
        .billing_group_properties(new_props)
        .send()
        .await
        .expect("update should succeed");
    assert_eq!(resp.version(), 2);
}

#[tokio::test]
async fn test_delete_billing_group() {
    let client = make_iot_client().await;

    client
        .create_billing_group()
        .billing_group_name("bg-del")
        .send()
        .await
        .unwrap();
    client
        .delete_billing_group()
        .billing_group_name("bg-del")
        .send()
        .await
        .expect("delete should succeed");

    let result = client
        .describe_billing_group()
        .billing_group_name("bg-del")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_add_remove_thing_to_billing_group() {
    let client = make_iot_client().await;

    client
        .create_thing()
        .thing_name("bt1")
        .send()
        .await
        .unwrap();
    client
        .create_billing_group()
        .billing_group_name("bbg1")
        .send()
        .await
        .unwrap();

    client
        .add_thing_to_billing_group()
        .billing_group_name("bbg1")
        .thing_name("bt1")
        .send()
        .await
        .expect("add should succeed");

    let resp = client
        .list_things_in_billing_group()
        .billing_group_name("bbg1")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.things().len(), 1);
    assert_eq!(resp.things()[0], "bt1");

    client
        .remove_thing_from_billing_group()
        .billing_group_name("bbg1")
        .thing_name("bt1")
        .send()
        .await
        .expect("remove should succeed");

    let resp2 = client
        .list_things_in_billing_group()
        .billing_group_name("bbg1")
        .send()
        .await
        .unwrap();
    assert!(resp2.things().is_empty());
}

// ==================== Certificate ====================

#[tokio::test]
async fn test_create_keys_and_certificate() {
    let client = make_iot_client().await;

    let resp = client
        .create_keys_and_certificate()
        .set_as_active(true)
        .send()
        .await
        .expect("create_keys_and_certificate should succeed");

    assert!(resp.certificate_id().is_some());
    assert!(resp.certificate_arn().is_some());
    assert!(resp.certificate_pem().is_some());
    let kp = resp.key_pair().unwrap();
    assert!(kp.public_key().is_some());
    assert!(kp.private_key().is_some());
}

#[tokio::test]
async fn test_create_certificate_from_csr() {
    let client = make_iot_client().await;

    let resp = client
        .create_certificate_from_csr()
        .certificate_signing_request(
            "-----BEGIN CERTIFICATE REQUEST-----\nMOCK_CSR\n-----END CERTIFICATE REQUEST-----",
        )
        .set_as_active(true)
        .send()
        .await
        .expect("create_certificate_from_csr should succeed");

    assert!(resp.certificate_id().is_some());
    assert!(resp.certificate_arn().is_some());
    assert!(resp.certificate_pem().is_some());
}

#[tokio::test]
async fn test_list_and_describe_certificate() {
    let client = make_iot_client().await;

    let create_resp = client
        .create_keys_and_certificate()
        .set_as_active(true)
        .send()
        .await
        .unwrap();
    let cert_id = create_resp.certificate_id().unwrap();

    let list_resp = client
        .list_certificates()
        .send()
        .await
        .expect("list should succeed");
    assert!(!list_resp.certificates().is_empty());

    let desc = client
        .describe_certificate()
        .certificate_id(cert_id)
        .send()
        .await
        .expect("describe should succeed");
    let cert_desc = desc.certificate_description().unwrap();
    assert_eq!(cert_desc.certificate_id().unwrap(), cert_id);
    assert_eq!(cert_desc.status().unwrap().as_str(), "ACTIVE");
}

#[tokio::test]
async fn test_update_certificate() {
    let client = make_iot_client().await;

    let create_resp = client
        .create_keys_and_certificate()
        .set_as_active(true)
        .send()
        .await
        .unwrap();
    let cert_id = create_resp.certificate_id().unwrap();

    client
        .update_certificate()
        .certificate_id(cert_id)
        .new_status(aws_sdk_iot::types::CertificateStatus::Inactive)
        .send()
        .await
        .expect("update_certificate should succeed");

    let desc = client
        .describe_certificate()
        .certificate_id(cert_id)
        .send()
        .await
        .unwrap();
    assert_eq!(
        desc.certificate_description()
            .unwrap()
            .status()
            .unwrap()
            .as_str(),
        "INACTIVE"
    );
}

#[tokio::test]
async fn test_delete_certificate() {
    let client = make_iot_client().await;

    let create_resp = client
        .create_keys_and_certificate()
        .set_as_active(false)
        .send()
        .await
        .unwrap();
    let cert_id = create_resp.certificate_id().unwrap();

    client
        .delete_certificate()
        .certificate_id(cert_id)
        .send()
        .await
        .expect("delete should succeed");

    let result = client
        .describe_certificate()
        .certificate_id(cert_id)
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_register_certificate() {
    let client = make_iot_client().await;

    let resp = client
        .register_certificate()
        .certificate_pem("-----BEGIN CERTIFICATE-----\nTEST_CERT\n-----END CERTIFICATE-----")
        .send()
        .await
        .expect("register_certificate should succeed");

    assert!(resp.certificate_id().is_some());
    assert!(resp.certificate_arn().is_some());
}

#[tokio::test]
async fn test_register_certificate_without_ca() {
    let client = make_iot_client().await;

    let resp = client
        .register_certificate_without_ca()
        .certificate_pem("-----BEGIN CERTIFICATE-----\nNO_CA_CERT\n-----END CERTIFICATE-----")
        .send()
        .await
        .expect("register_certificate_without_ca should succeed");

    assert!(resp.certificate_id().is_some());
    assert!(resp.certificate_arn().is_some());
}

#[tokio::test]
async fn test_list_certificates_by_ca() {
    let client = make_iot_client().await;

    // Register a CA cert first
    let ca_resp = client
        .register_ca_certificate()
        .ca_certificate("-----BEGIN CERTIFICATE-----\nCA_PEM\n-----END CERTIFICATE-----")
        .send()
        .await
        .unwrap();
    let ca_id = ca_resp.certificate_id().unwrap();

    // Register a cert with the CA reference
    client
        .register_certificate()
        .certificate_pem("-----BEGIN CERTIFICATE-----\nCHILD_CERT\n-----END CERTIFICATE-----")
        .ca_certificate_pem("-----BEGIN CERTIFICATE-----\nCA_PEM\n-----END CERTIFICATE-----")
        .send()
        .await
        .unwrap();

    let list_resp = client
        .list_certificates_by_ca()
        .ca_certificate_id(ca_id)
        .send()
        .await
        .expect("list should succeed");
    assert!(!list_resp.certificates().is_empty());
}

// ==================== CA Certificate ====================

#[tokio::test]
async fn test_register_and_describe_ca_certificate() {
    let client = make_iot_client().await;

    let resp = client
        .register_ca_certificate()
        .ca_certificate("-----BEGIN CERTIFICATE-----\nMOCK_CA\n-----END CERTIFICATE-----")
        .send()
        .await
        .expect("register_ca_certificate should succeed");

    let ca_id = resp.certificate_id().unwrap();
    assert!(resp.certificate_arn().is_some());

    let desc = client
        .describe_ca_certificate()
        .certificate_id(ca_id)
        .send()
        .await
        .expect("describe should succeed");
    assert!(desc.certificate_description().is_some());
}

#[tokio::test]
async fn test_update_ca_certificate() {
    let client = make_iot_client().await;

    let resp = client
        .register_ca_certificate()
        .ca_certificate("-----BEGIN CERTIFICATE-----\nCA_UPD\n-----END CERTIFICATE-----")
        .send()
        .await
        .unwrap();
    let ca_id = resp.certificate_id().unwrap();

    client
        .update_ca_certificate()
        .certificate_id(ca_id)
        .new_status(aws_sdk_iot::types::CaCertificateStatus::Active)
        .send()
        .await
        .expect("update should succeed");
}

#[tokio::test]
async fn test_delete_ca_certificate() {
    let client = make_iot_client().await;

    let resp = client
        .register_ca_certificate()
        .ca_certificate("-----BEGIN CERTIFICATE-----\nCA_DEL\n-----END CERTIFICATE-----")
        .send()
        .await
        .unwrap();
    let ca_id = resp.certificate_id().unwrap();

    client
        .delete_ca_certificate()
        .certificate_id(ca_id)
        .send()
        .await
        .expect("delete should succeed");

    let result = client
        .describe_ca_certificate()
        .certificate_id(ca_id)
        .send()
        .await;
    assert!(result.is_err());
}

// ==================== Policy ====================

#[tokio::test]
async fn test_create_and_get_policy() {
    let client = make_iot_client().await;

    let resp = client
        .create_policy()
        .policy_name("my-policy")
        .policy_document(r#"{"Version":"2012-10-17","Statement":[]}"#)
        .send()
        .await
        .expect("create_policy should succeed");
    assert_eq!(resp.policy_name().unwrap(), "my-policy");
    assert!(resp.policy_arn().unwrap().contains("my-policy"));

    let get_resp = client
        .get_policy()
        .policy_name("my-policy")
        .send()
        .await
        .expect("get_policy should succeed");
    assert_eq!(get_resp.policy_name().unwrap(), "my-policy");
    assert!(get_resp.default_version_id().is_some());
}

#[tokio::test]
async fn test_list_policies() {
    let client = make_iot_client().await;

    client
        .create_policy()
        .policy_name("pol-a")
        .policy_document("{}")
        .send()
        .await
        .unwrap();
    client
        .create_policy()
        .policy_name("pol-b")
        .policy_document("{}")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_policies()
        .send()
        .await
        .expect("list should succeed");
    assert_eq!(resp.policies().len(), 2);
}

#[tokio::test]
async fn test_delete_policy() {
    let client = make_iot_client().await;

    client
        .create_policy()
        .policy_name("pol-del")
        .policy_document("{}")
        .send()
        .await
        .unwrap();
    client
        .delete_policy()
        .policy_name("pol-del")
        .send()
        .await
        .expect("delete should succeed");

    let result = client.get_policy().policy_name("pol-del").send().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_create_policy_version_and_list() {
    let client = make_iot_client().await;

    client
        .create_policy()
        .policy_name("ver-pol")
        .policy_document("{}")
        .send()
        .await
        .unwrap();

    let pv_resp = client
        .create_policy_version()
        .policy_name("ver-pol")
        .policy_document(r#"{"v":2}"#)
        .set_as_default(true)
        .send()
        .await
        .expect("create_policy_version should succeed");
    assert!(pv_resp.policy_version_id().is_some());
    assert!(pv_resp.is_default_version());

    let list_resp = client
        .list_policy_versions()
        .policy_name("ver-pol")
        .send()
        .await
        .expect("list_policy_versions should succeed");
    assert_eq!(list_resp.policy_versions().len(), 2);
}

#[tokio::test]
async fn test_get_policy_version() {
    let client = make_iot_client().await;

    client
        .create_policy()
        .policy_name("gpv-pol")
        .policy_document("{}")
        .send()
        .await
        .unwrap();

    let resp = client
        .get_policy_version()
        .policy_name("gpv-pol")
        .policy_version_id("1")
        .send()
        .await
        .expect("get_policy_version should succeed");
    assert_eq!(resp.policy_name().unwrap(), "gpv-pol");
    assert!(resp.is_default_version());
}

#[tokio::test]
async fn test_set_default_policy_version() {
    let client = make_iot_client().await;

    client
        .create_policy()
        .policy_name("sdpv-pol")
        .policy_document("{}")
        .send()
        .await
        .unwrap();
    let pv = client
        .create_policy_version()
        .policy_name("sdpv-pol")
        .policy_document(r#"{"v":2}"#)
        .send()
        .await
        .unwrap();
    let vid = pv.policy_version_id().unwrap();

    client
        .set_default_policy_version()
        .policy_name("sdpv-pol")
        .policy_version_id(vid)
        .send()
        .await
        .expect("set_default_policy_version should succeed");

    let get_resp = client
        .get_policy_version()
        .policy_name("sdpv-pol")
        .policy_version_id(vid)
        .send()
        .await
        .unwrap();
    assert!(get_resp.is_default_version());
}

#[tokio::test]
async fn test_delete_policy_version() {
    let client = make_iot_client().await;

    client
        .create_policy()
        .policy_name("dpv-pol")
        .policy_document("{}")
        .send()
        .await
        .unwrap();
    let pv = client
        .create_policy_version()
        .policy_name("dpv-pol")
        .policy_document(r#"{"v":2}"#)
        .send()
        .await
        .unwrap();
    let vid = pv.policy_version_id().unwrap();

    client
        .delete_policy_version()
        .policy_name("dpv-pol")
        .policy_version_id(vid)
        .send()
        .await
        .expect("delete_policy_version should succeed");

    let list_resp = client
        .list_policy_versions()
        .policy_name("dpv-pol")
        .send()
        .await
        .unwrap();
    assert_eq!(list_resp.policy_versions().len(), 1);
}

// ==================== Policy Attach/Detach ====================

#[tokio::test]
async fn test_attach_and_detach_policy() {
    let client = make_iot_client().await;

    client
        .create_policy()
        .policy_name("attach-pol")
        .policy_document("{}")
        .send()
        .await
        .unwrap();
    let cert = client
        .create_keys_and_certificate()
        .set_as_active(true)
        .send()
        .await
        .unwrap();
    let cert_arn = cert.certificate_arn().unwrap();

    client
        .attach_policy()
        .policy_name("attach-pol")
        .target(cert_arn)
        .send()
        .await
        .expect("attach_policy should succeed");

    let attached = client
        .list_attached_policies()
        .target(cert_arn)
        .send()
        .await
        .expect("list_attached_policies should succeed");
    assert!(!attached.policies().is_empty());

    // ListTargetsForPolicy
    let targets = client
        .list_targets_for_policy()
        .policy_name("attach-pol")
        .send()
        .await
        .expect("list_targets_for_policy should succeed");
    assert!(targets.targets().contains(&cert_arn.to_string()));

    // DetachPolicy
    client
        .detach_policy()
        .policy_name("attach-pol")
        .target(cert_arn)
        .send()
        .await
        .expect("detach_policy should succeed");

    let attached2 = client
        .list_attached_policies()
        .target(cert_arn)
        .send()
        .await
        .unwrap();
    assert!(attached2.policies().is_empty());
}

#[tokio::test]
#[allow(deprecated)]
async fn test_attach_and_detach_principal_policy() {
    let client = make_iot_client().await;

    client
        .create_policy()
        .policy_name("pp-pol")
        .policy_document("{}")
        .send()
        .await
        .unwrap();
    let cert = client
        .create_keys_and_certificate()
        .set_as_active(true)
        .send()
        .await
        .unwrap();
    let cert_arn = cert.certificate_arn().unwrap();

    client
        .attach_principal_policy()
        .policy_name("pp-pol")
        .principal(cert_arn)
        .send()
        .await
        .expect("attach_principal_policy should succeed");

    // ListPrincipalPolicies
    let pp_resp = client
        .list_principal_policies()
        .principal(cert_arn)
        .send()
        .await
        .expect("list_principal_policies should succeed");
    assert!(!pp_resp.policies().is_empty());

    // ListPolicyPrincipals
    let lpp_resp = client
        .list_policy_principals()
        .policy_name("pp-pol")
        .send()
        .await
        .expect("list_policy_principals should succeed");
    assert!(!lpp_resp.principals().is_empty());

    // DetachPrincipalPolicy
    client
        .detach_principal_policy()
        .policy_name("pp-pol")
        .principal(cert_arn)
        .send()
        .await
        .expect("detach should succeed");
}

// ==================== Thing Principal ====================

#[tokio::test]
async fn test_attach_detach_thing_principal() {
    let client = make_iot_client().await;

    client
        .create_thing()
        .thing_name("tp-thing")
        .send()
        .await
        .unwrap();
    let cert = client
        .create_keys_and_certificate()
        .set_as_active(true)
        .send()
        .await
        .unwrap();
    let cert_arn = cert.certificate_arn().unwrap();

    client
        .attach_thing_principal()
        .thing_name("tp-thing")
        .principal(cert_arn)
        .send()
        .await
        .expect("attach should succeed");

    // ListThingPrincipals
    let resp = client
        .list_thing_principals()
        .thing_name("tp-thing")
        .send()
        .await
        .expect("list should succeed");
    assert!(resp.principals().contains(&cert_arn.to_string()));

    // ListThingPrincipalsV2
    let resp_v2 = client
        .list_thing_principals_v2()
        .thing_name("tp-thing")
        .send()
        .await
        .expect("list_v2 should succeed");
    assert!(!resp_v2.thing_principal_objects().is_empty());

    // ListPrincipalThings
    let pt_resp = client
        .list_principal_things()
        .principal(cert_arn)
        .send()
        .await
        .expect("list_principal_things should succeed");
    assert!(pt_resp.things().contains(&"tp-thing".to_string()));

    // DetachThingPrincipal
    client
        .detach_thing_principal()
        .thing_name("tp-thing")
        .principal(cert_arn)
        .send()
        .await
        .expect("detach should succeed");

    let resp2 = client
        .list_thing_principals()
        .thing_name("tp-thing")
        .send()
        .await
        .unwrap();
    assert!(resp2.principals().is_empty());
}

// ==================== RoleAlias ====================

#[tokio::test]
async fn test_create_and_describe_role_alias() {
    let client = make_iot_client().await;

    let resp = client
        .create_role_alias()
        .role_alias("my-alias")
        .role_arn("arn:aws:iam::123456789012:role/MyRole")
        .credential_duration_seconds(3600)
        .send()
        .await
        .expect("create_role_alias should succeed");
    assert_eq!(resp.role_alias().unwrap(), "my-alias");

    let desc = client
        .describe_role_alias()
        .role_alias("my-alias")
        .send()
        .await
        .expect("describe should succeed");
    let alias_desc = desc.role_alias_description().unwrap();
    assert_eq!(alias_desc.role_alias().unwrap(), "my-alias");
    assert_eq!(alias_desc.credential_duration_seconds(), Some(3600));
}

#[tokio::test]
async fn test_list_role_aliases() {
    let client = make_iot_client().await;

    client
        .create_role_alias()
        .role_alias("alias-a")
        .role_arn("arn:aws:iam::123456789012:role/A")
        .send()
        .await
        .unwrap();
    client
        .create_role_alias()
        .role_alias("alias-b")
        .role_arn("arn:aws:iam::123456789012:role/B")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_role_aliases()
        .send()
        .await
        .expect("list should succeed");
    assert_eq!(resp.role_aliases().len(), 2);
}

#[tokio::test]
async fn test_update_role_alias() {
    let client = make_iot_client().await;

    client
        .create_role_alias()
        .role_alias("alias-upd")
        .role_arn("arn:aws:iam::123456789012:role/X")
        .send()
        .await
        .unwrap();

    let resp = client
        .update_role_alias()
        .role_alias("alias-upd")
        .credential_duration_seconds(7200)
        .send()
        .await
        .expect("update should succeed");
    assert_eq!(resp.role_alias().unwrap(), "alias-upd");
}

#[tokio::test]
async fn test_delete_role_alias() {
    let client = make_iot_client().await;

    client
        .create_role_alias()
        .role_alias("alias-del")
        .role_arn("arn:aws:iam::123456789012:role/Y")
        .send()
        .await
        .unwrap();
    client
        .delete_role_alias()
        .role_alias("alias-del")
        .send()
        .await
        .expect("delete should succeed");

    let result = client
        .describe_role_alias()
        .role_alias("alias-del")
        .send()
        .await;
    assert!(result.is_err());
}

// ==================== DomainConfiguration ====================

#[tokio::test]
async fn test_create_and_describe_domain_configuration() {
    let client = make_iot_client().await;

    let resp = client
        .create_domain_configuration()
        .domain_configuration_name("my-dc")
        .send()
        .await
        .expect("create should succeed");
    assert_eq!(resp.domain_configuration_name().unwrap(), "my-dc");

    let desc = client
        .describe_domain_configuration()
        .domain_configuration_name("my-dc")
        .send()
        .await
        .expect("describe should succeed");
    assert_eq!(desc.domain_configuration_name().unwrap(), "my-dc");
}

#[tokio::test]
async fn test_list_domain_configurations() {
    let client = make_iot_client().await;

    client
        .create_domain_configuration()
        .domain_configuration_name("dc-a")
        .send()
        .await
        .unwrap();
    client
        .create_domain_configuration()
        .domain_configuration_name("dc-b")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_domain_configurations()
        .send()
        .await
        .expect("list should succeed");
    assert_eq!(resp.domain_configurations().len(), 2);
}

#[tokio::test]
async fn test_update_domain_configuration() {
    let client = make_iot_client().await;

    client
        .create_domain_configuration()
        .domain_configuration_name("dc-upd")
        .send()
        .await
        .unwrap();

    let resp = client
        .update_domain_configuration()
        .domain_configuration_name("dc-upd")
        .domain_configuration_status(aws_sdk_iot::types::DomainConfigurationStatus::Disabled)
        .send()
        .await
        .expect("update should succeed");
    assert_eq!(resp.domain_configuration_name().unwrap(), "dc-upd");
}

#[tokio::test]
async fn test_delete_domain_configuration() {
    let client = make_iot_client().await;

    client
        .create_domain_configuration()
        .domain_configuration_name("dc-del")
        .send()
        .await
        .unwrap();
    client
        .delete_domain_configuration()
        .domain_configuration_name("dc-del")
        .send()
        .await
        .expect("delete should succeed");

    let result = client
        .describe_domain_configuration()
        .domain_configuration_name("dc-del")
        .send()
        .await;
    assert!(result.is_err());
}

// ==================== Job ====================

#[tokio::test]
async fn test_create_and_describe_job() {
    let client = make_iot_client().await;

    client
        .create_thing()
        .thing_name("job-thing")
        .send()
        .await
        .unwrap();

    let resp = client
        .create_job()
        .job_id("job-1")
        .targets("arn:aws:iot:us-east-1:123456789012:thing/job-thing")
        .document("{\"action\":\"test\"}")
        .description("test job")
        .send()
        .await
        .expect("create_job should succeed");
    assert_eq!(resp.job_id().unwrap(), "job-1");

    let desc = client
        .describe_job()
        .job_id("job-1")
        .send()
        .await
        .expect("describe should succeed");
    let job = desc.job().unwrap();
    assert_eq!(job.job_id().unwrap(), "job-1");
    assert_eq!(job.description().unwrap(), "test job");
}

#[tokio::test]
async fn test_list_jobs() {
    let client = make_iot_client().await;

    client
        .create_job()
        .job_id("lj-1")
        .targets("arn:aws:iot:us-east-1:123456789012:thing/x")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_jobs()
        .send()
        .await
        .expect("list should succeed");
    assert!(!resp.jobs().is_empty());
}

#[tokio::test]
async fn test_get_job_document() {
    let client = make_iot_client().await;

    client
        .create_job()
        .job_id("jd-1")
        .targets("arn:aws:iot:us-east-1:123456789012:thing/x")
        .document("{\"cmd\":\"reboot\"}")
        .send()
        .await
        .unwrap();

    let resp = client
        .get_job_document()
        .job_id("jd-1")
        .send()
        .await
        .expect("get_job_document should succeed");
    assert!(resp.document().is_some());
    assert!(resp.document().unwrap().contains("reboot"));
}

#[tokio::test]
async fn test_cancel_job() {
    let client = make_iot_client().await;

    client
        .create_job()
        .job_id("cj-1")
        .targets("arn:aws:iot:us-east-1:123456789012:thing/x")
        .send()
        .await
        .unwrap();

    let resp = client
        .cancel_job()
        .job_id("cj-1")
        .comment("no longer needed")
        .send()
        .await
        .expect("cancel_job should succeed");
    assert_eq!(resp.job_id().unwrap(), "cj-1");
}

#[tokio::test]
async fn test_delete_job() {
    let client = make_iot_client().await;

    client
        .create_job()
        .job_id("dj-1")
        .targets("arn:aws:iot:us-east-1:123456789012:thing/x")
        .send()
        .await
        .unwrap();

    // Cancel first since only CANCELED jobs can be deleted in real AWS
    client.cancel_job().job_id("dj-1").send().await.unwrap();

    client
        .delete_job()
        .job_id("dj-1")
        .send()
        .await
        .expect("delete should succeed");

    let result = client.describe_job().job_id("dj-1").send().await;
    assert!(result.is_err());
}

// ==================== Job Execution ====================

#[tokio::test]
async fn test_describe_job_execution() {
    let client = make_iot_client().await;

    client
        .create_thing()
        .thing_name("exec-thing")
        .send()
        .await
        .unwrap();
    client
        .create_job()
        .job_id("exec-job")
        .targets("arn:aws:iot:us-east-1:123456789012:thing/exec-thing")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_job_execution()
        .job_id("exec-job")
        .thing_name("exec-thing")
        .send()
        .await
        .expect("describe_job_execution should succeed");
    assert!(resp.execution().is_some());
}

#[tokio::test]
async fn test_list_job_executions_for_job() {
    let client = make_iot_client().await;

    client
        .create_thing()
        .thing_name("ljefj-thing")
        .send()
        .await
        .unwrap();
    client
        .create_job()
        .job_id("ljefj-job")
        .targets("arn:aws:iot:us-east-1:123456789012:thing/ljefj-thing")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_job_executions_for_job()
        .job_id("ljefj-job")
        .send()
        .await
        .expect("list should succeed");
    assert!(!resp.execution_summaries().is_empty());
}

#[tokio::test]
async fn test_list_job_executions_for_thing() {
    let client = make_iot_client().await;

    client
        .create_thing()
        .thing_name("ljeft-thing")
        .send()
        .await
        .unwrap();
    client
        .create_job()
        .job_id("ljeft-job")
        .targets("arn:aws:iot:us-east-1:123456789012:thing/ljeft-thing")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_job_executions_for_thing()
        .thing_name("ljeft-thing")
        .send()
        .await
        .expect("list should succeed");
    assert!(!resp.execution_summaries().is_empty());
}

#[tokio::test]
async fn test_cancel_job_execution() {
    let client = make_iot_client().await;

    client
        .create_thing()
        .thing_name("cje-thing")
        .send()
        .await
        .unwrap();
    client
        .create_job()
        .job_id("cje-job")
        .targets("arn:aws:iot:us-east-1:123456789012:thing/cje-thing")
        .send()
        .await
        .unwrap();

    client
        .cancel_job_execution()
        .job_id("cje-job")
        .thing_name("cje-thing")
        .send()
        .await
        .expect("cancel_job_execution should succeed");
}

#[tokio::test]
async fn test_delete_job_execution() {
    let client = make_iot_client().await;

    client
        .create_thing()
        .thing_name("dje-thing")
        .send()
        .await
        .unwrap();
    client
        .create_job()
        .job_id("dje-job")
        .targets("arn:aws:iot:us-east-1:123456789012:thing/dje-thing")
        .send()
        .await
        .unwrap();

    client
        .delete_job_execution()
        .job_id("dje-job")
        .thing_name("dje-thing")
        .execution_number(1)
        .send()
        .await
        .expect("delete_job_execution should succeed");
}

// ==================== JobTemplate ====================

#[tokio::test]
async fn test_create_and_describe_job_template() {
    let client = make_iot_client().await;

    let resp = client
        .create_job_template()
        .job_template_id("jt-1")
        .description("a template")
        .document("{\"cmd\":\"update\"}")
        .send()
        .await
        .expect("create_job_template should succeed");
    assert_eq!(resp.job_template_id().unwrap(), "jt-1");

    let desc = client
        .describe_job_template()
        .job_template_id("jt-1")
        .send()
        .await
        .expect("describe should succeed");
    assert_eq!(desc.job_template_id().unwrap(), "jt-1");
    assert_eq!(desc.description().unwrap(), "a template");
}

#[tokio::test]
async fn test_list_job_templates() {
    let client = make_iot_client().await;

    client
        .create_job_template()
        .job_template_id("ljt-1")
        .description("d1")
        .send()
        .await
        .unwrap();
    client
        .create_job_template()
        .job_template_id("ljt-2")
        .description("d2")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_job_templates()
        .send()
        .await
        .expect("list should succeed");
    assert_eq!(resp.job_templates().len(), 2);
}

#[tokio::test]
async fn test_delete_job_template() {
    let client = make_iot_client().await;

    client
        .create_job_template()
        .job_template_id("djt-1")
        .description("d")
        .send()
        .await
        .unwrap();
    client
        .delete_job_template()
        .job_template_id("djt-1")
        .send()
        .await
        .expect("delete should succeed");

    let result = client
        .describe_job_template()
        .job_template_id("djt-1")
        .send()
        .await;
    assert!(result.is_err());
}

// ==================== TopicRule ====================

#[tokio::test]
async fn test_create_and_get_topic_rule() {
    let client = make_iot_client().await;

    let payload = aws_sdk_iot::types::TopicRulePayload::builder()
        .sql("SELECT * FROM 'topic/test'")
        .description("test rule")
        .actions(aws_sdk_iot::types::Action::builder().build())
        .build()
        .unwrap();

    client
        .create_topic_rule()
        .rule_name("myRule")
        .topic_rule_payload(payload)
        .send()
        .await
        .expect("create_topic_rule should succeed");

    let resp = client
        .get_topic_rule()
        .rule_name("myRule")
        .send()
        .await
        .expect("get_topic_rule should succeed");
    assert!(resp.rule_arn().is_some());
    let rule = resp.rule().unwrap();
    assert_eq!(rule.rule_name().unwrap(), "myRule");
    assert!(rule.sql().is_some());
}

#[tokio::test]
async fn test_list_topic_rules() {
    let client = make_iot_client().await;

    let payload = aws_sdk_iot::types::TopicRulePayload::builder()
        .sql("SELECT * FROM 'a'")
        .actions(aws_sdk_iot::types::Action::builder().build())
        .build()
        .unwrap();
    client
        .create_topic_rule()
        .rule_name("ruleA")
        .topic_rule_payload(payload)
        .send()
        .await
        .unwrap();

    let payload2 = aws_sdk_iot::types::TopicRulePayload::builder()
        .sql("SELECT * FROM 'b'")
        .actions(aws_sdk_iot::types::Action::builder().build())
        .build()
        .unwrap();
    client
        .create_topic_rule()
        .rule_name("ruleB")
        .topic_rule_payload(payload2)
        .send()
        .await
        .unwrap();

    let resp = client
        .list_topic_rules()
        .send()
        .await
        .expect("list should succeed");
    assert_eq!(resp.rules().len(), 2);
}

#[tokio::test]
async fn test_replace_topic_rule() {
    let client = make_iot_client().await;

    let payload = aws_sdk_iot::types::TopicRulePayload::builder()
        .sql("SELECT * FROM 'old'")
        .actions(aws_sdk_iot::types::Action::builder().build())
        .build()
        .unwrap();
    client
        .create_topic_rule()
        .rule_name("replaceMe")
        .topic_rule_payload(payload)
        .send()
        .await
        .unwrap();

    let new_payload = aws_sdk_iot::types::TopicRulePayload::builder()
        .sql("SELECT * FROM 'new'")
        .description("replaced")
        .actions(aws_sdk_iot::types::Action::builder().build())
        .build()
        .unwrap();

    client
        .replace_topic_rule()
        .rule_name("replaceMe")
        .topic_rule_payload(new_payload)
        .send()
        .await
        .expect("replace should succeed");

    let resp = client
        .get_topic_rule()
        .rule_name("replaceMe")
        .send()
        .await
        .unwrap();
    let rule = resp.rule().unwrap();
    assert!(rule.sql().is_some());
    assert!(rule.sql().unwrap().contains("new"));
}

#[tokio::test]
async fn test_delete_topic_rule() {
    let client = make_iot_client().await;

    let payload = aws_sdk_iot::types::TopicRulePayload::builder()
        .sql("SELECT * FROM 'x'")
        .actions(aws_sdk_iot::types::Action::builder().build())
        .build()
        .unwrap();
    client
        .create_topic_rule()
        .rule_name("delRule")
        .topic_rule_payload(payload)
        .send()
        .await
        .unwrap();

    client
        .delete_topic_rule()
        .rule_name("delRule")
        .send()
        .await
        .expect("delete should succeed");

    let result = client.get_topic_rule().rule_name("delRule").send().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_enable_disable_topic_rule() {
    let client = make_iot_client().await;

    let payload = aws_sdk_iot::types::TopicRulePayload::builder()
        .sql("SELECT * FROM 'ed'")
        .rule_disabled(false)
        .actions(aws_sdk_iot::types::Action::builder().build())
        .build()
        .unwrap();
    client
        .create_topic_rule()
        .rule_name("edRule")
        .topic_rule_payload(payload)
        .send()
        .await
        .unwrap();

    client
        .disable_topic_rule()
        .rule_name("edRule")
        .send()
        .await
        .expect("disable should succeed");

    let resp = client
        .get_topic_rule()
        .rule_name("edRule")
        .send()
        .await
        .unwrap();
    assert!(resp.rule().unwrap().rule_disabled().unwrap());

    client
        .enable_topic_rule()
        .rule_name("edRule")
        .send()
        .await
        .expect("enable should succeed");

    let resp2 = client
        .get_topic_rule()
        .rule_name("edRule")
        .send()
        .await
        .unwrap();
    assert!(!resp2.rule().unwrap().rule_disabled().unwrap());
}

// ==================== Misc ====================

#[tokio::test]
async fn test_describe_endpoint() {
    let client = make_iot_client().await;

    let resp = client
        .describe_endpoint()
        .send()
        .await
        .expect("describe_endpoint should succeed");
    assert!(resp.endpoint_address().is_some());
    assert!(resp.endpoint_address().unwrap().contains("amazonaws.com"));
}

#[tokio::test]
async fn test_get_registration_code() {
    let client = make_iot_client().await;

    let resp = client
        .get_registration_code()
        .send()
        .await
        .expect("get_registration_code should succeed");
    assert!(resp.registration_code().is_some());
    assert!(!resp.registration_code().unwrap().is_empty());
}

#[tokio::test]
async fn test_get_indexing_configuration() {
    let client = make_iot_client().await;

    let resp = client
        .get_indexing_configuration()
        .send()
        .await
        .expect("get_indexing_configuration should succeed");
    assert!(resp.thing_indexing_configuration().is_some());
    assert!(resp.thing_group_indexing_configuration().is_some());
}

#[tokio::test]
async fn test_update_indexing_configuration() {
    let client = make_iot_client().await;

    client
        .update_indexing_configuration()
        .send()
        .await
        .expect("update_indexing_configuration should succeed");
}

#[tokio::test]
async fn test_search_index() {
    let client = make_iot_client().await;

    client
        .create_thing()
        .thing_name("search-thing")
        .send()
        .await
        .unwrap();

    let resp = client
        .search_index()
        .query_string("*")
        .send()
        .await
        .expect("search_index should succeed");
    assert!(!resp.things().is_empty());
}

// ============================================================================
// Ported from moto: test_iot_things.py
// ============================================================================

// Ported from moto: test_iot_things.py::test_create_thing_with_type
#[tokio::test]
async fn test_moto_create_thing_with_type() {
    let client = make_iot_client().await;

    client
        .create_thing_type()
        .thing_type_name("my-type-name")
        .send()
        .await
        .unwrap();

    let thing = client
        .create_thing()
        .thing_name("my-thing-with-type")
        .thing_type_name("my-type-name")
        .send()
        .await
        .unwrap();
    assert_eq!(thing.thing_name().unwrap(), "my-thing-with-type");
    assert!(thing.thing_arn().is_some());

    let desc = client
        .describe_thing()
        .thing_name("my-thing-with-type")
        .send()
        .await
        .unwrap();
    assert_eq!(desc.thing_type_name().unwrap(), "my-type-name");
}

// Ported from moto: test_iot_things.py::test_describe_thing (version tracking after update)
#[tokio::test]
async fn test_moto_describe_thing_version() {
    let client = make_iot_client().await;

    client
        .create_thing()
        .thing_name("ver-thing")
        .send()
        .await
        .unwrap();

    let attrs = aws_sdk_iot::types::AttributePayload::builder()
        .attributes("k1", "v1")
        .build();
    client
        .update_thing()
        .thing_name("ver-thing")
        .attribute_payload(attrs)
        .send()
        .await
        .unwrap();

    let thing = client
        .describe_thing()
        .thing_name("ver-thing")
        .send()
        .await
        .unwrap();
    assert!(thing.thing_id().is_some());
    assert_eq!(thing.thing_name().unwrap(), "ver-thing");
    assert!(thing.default_client_id().is_some());
    assert_eq!(
        thing.attributes().unwrap().get("k1").map(|s| s.as_str()),
        Some("v1")
    );
}

// ============================================================================
// Ported from moto: test_iot_deprecate_thing_type.py
// ============================================================================

// Ported from moto: test_iot_deprecate_thing_type.py::test_deprecate_undeprecate_thing_type
#[tokio::test]
async fn test_moto_deprecate_undeprecate_thing_type() {
    let client = make_iot_client().await;

    let props = aws_sdk_iot::types::ThingTypeProperties::builder()
        .searchable_attributes("s1")
        .searchable_attributes("s2")
        .searchable_attributes("s3")
        .build();
    client
        .create_thing_type()
        .thing_type_name("dep-tt")
        .thing_type_properties(props)
        .send()
        .await
        .unwrap();

    let res = client
        .describe_thing_type()
        .thing_type_name("dep-tt")
        .send()
        .await
        .unwrap();
    assert!(!res.thing_type_metadata().unwrap().deprecated());

    client
        .deprecate_thing_type()
        .thing_type_name("dep-tt")
        .undo_deprecate(false)
        .send()
        .await
        .unwrap();
    let res = client
        .describe_thing_type()
        .thing_type_name("dep-tt")
        .send()
        .await
        .unwrap();
    assert!(res.thing_type_metadata().unwrap().deprecated());

    client
        .deprecate_thing_type()
        .thing_type_name("dep-tt")
        .undo_deprecate(true)
        .send()
        .await
        .unwrap();
    let res = client
        .describe_thing_type()
        .thing_type_name("dep-tt")
        .send()
        .await
        .unwrap();
    assert!(!res.thing_type_metadata().unwrap().deprecated());
}

// Ported from moto: test_iot_deprecate_thing_type.py::test_deprecate_thing_type_not_exist
#[tokio::test]
async fn test_moto_deprecate_thing_type_not_exist() {
    let client = make_iot_client().await;

    let result = client
        .deprecate_thing_type()
        .thing_type_name("no-such-type")
        .undo_deprecate(false)
        .send()
        .await;
    assert!(result.is_err());
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

// ============================================================================
// Ported from moto: test_iot_thing_groups.py
// ============================================================================

// Ported from moto: test_iot_thing_groups.py::test_thing_groups (full CRUD with properties)
#[tokio::test]
async fn test_moto_thing_groups_crud_with_properties() {
    let client = make_iot_client().await;

    let group_name = "my-group-props";

    // Create group
    let tg = client
        .create_thing_group()
        .thing_group_name(group_name)
        .send()
        .await
        .unwrap();
    assert_eq!(tg.thing_group_name().unwrap(), group_name);
    assert!(tg.thing_group_arn().unwrap().contains(group_name));

    // List
    let res = client.list_thing_groups().send().await.unwrap();
    assert_eq!(res.thing_groups().len(), 1);

    // Describe
    let desc = client
        .describe_thing_group()
        .thing_group_name(group_name)
        .send()
        .await
        .unwrap();
    assert_eq!(desc.thing_group_name().unwrap(), group_name);
    assert!(desc.thing_group_properties().is_some());
    assert!(desc.thing_group_metadata().is_some());
    assert_eq!(desc.version(), 1);
    assert!(desc.thing_group_arn().unwrap().contains(group_name));

    // Delete
    client
        .delete_thing_group()
        .thing_group_name(group_name)
        .send()
        .await
        .unwrap();
    let res = client.list_thing_groups().send().await.unwrap();
    assert_eq!(res.thing_groups().len(), 0);

    // Create with properties
    let props = aws_sdk_iot::types::ThingGroupProperties::builder()
        .thing_group_description("my first thing group")
        .attribute_payload(
            aws_sdk_iot::types::AttributePayload::builder()
                .attributes("key1", "val01")
                .attributes("Key02", "VAL2")
                .build(),
        )
        .build();
    client
        .create_thing_group()
        .thing_group_name(group_name)
        .thing_group_properties(props)
        .send()
        .await
        .unwrap();

    let desc = client
        .describe_thing_group()
        .thing_group_name(group_name)
        .send()
        .await
        .unwrap();
    let tg_props = desc.thing_group_properties().unwrap();
    let attrs = tg_props.attribute_payload().unwrap().attributes().unwrap();
    assert_eq!(attrs.get("key1").map(|s| s.as_str()), Some("val01"));
    assert_eq!(attrs.get("Key02").map(|s| s.as_str()), Some("VAL2"));
}

// Ported from moto: test_iot_thing_groups.py::test_thing_group_relations
#[tokio::test]
async fn test_moto_thing_group_relations() {
    let client = make_iot_client().await;

    let group_name = "rel-group";
    let thing_name = "rel-thing";

    client
        .create_thing_group()
        .thing_group_name(group_name)
        .send()
        .await
        .unwrap();
    client
        .create_thing()
        .thing_name(thing_name)
        .send()
        .await
        .unwrap();

    // Add thing to group
    client
        .add_thing_to_thing_group()
        .thing_group_name(group_name)
        .thing_name(thing_name)
        .send()
        .await
        .unwrap();

    // Idempotent add
    client
        .add_thing_to_thing_group()
        .thing_group_name(group_name)
        .thing_name(thing_name)
        .send()
        .await
        .unwrap();

    let things = client
        .list_things_in_thing_group()
        .thing_group_name(group_name)
        .send()
        .await
        .unwrap();
    assert_eq!(things.things().len(), 1);

    let groups = client
        .list_thing_groups_for_thing()
        .thing_name(thing_name)
        .send()
        .await
        .unwrap();
    assert_eq!(groups.thing_groups().len(), 1);

    // Remove
    client
        .remove_thing_from_thing_group()
        .thing_group_name(group_name)
        .thing_name(thing_name)
        .send()
        .await
        .unwrap();
    let things = client
        .list_things_in_thing_group()
        .thing_group_name(group_name)
        .send()
        .await
        .unwrap();
    assert_eq!(things.things().len(), 0);

    // UpdateThingGroupsForThing
    client
        .update_thing_groups_for_thing()
        .thing_name(thing_name)
        .thing_groups_to_add(group_name)
        .send()
        .await
        .unwrap();
    let things = client
        .list_things_in_thing_group()
        .thing_group_name(group_name)
        .send()
        .await
        .unwrap();
    assert_eq!(things.things().len(), 1);

    client
        .update_thing_groups_for_thing()
        .thing_name(thing_name)
        .thing_groups_to_remove(group_name)
        .send()
        .await
        .unwrap();
    let things = client
        .list_things_in_thing_group()
        .thing_group_name(group_name)
        .send()
        .await
        .unwrap();
    assert_eq!(things.things().len(), 0);
}

// Ported from moto: test_iot_thing_groups.py::test_thing_group_already_exists_with_different_properties_raises
#[tokio::test]
async fn test_moto_thing_group_already_exists() {
    let client = make_iot_client().await;

    let props = aws_sdk_iot::types::ThingGroupProperties::builder()
        .thing_group_description("Current description")
        .build();
    client
        .create_thing_group()
        .thing_group_name("dup-group")
        .thing_group_properties(props)
        .send()
        .await
        .unwrap();

    let result = client
        .create_thing_group()
        .thing_group_name("dup-group")
        .send()
        .await;
    assert!(result.is_err());
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("ResourceAlreadyExistsException"),
        "Expected ResourceAlreadyExistsException, got: {err_str}"
    );
}

// Ported from moto: test_iot_thing_groups.py::test_thing_group_updates_description
#[tokio::test]
async fn test_moto_thing_group_updates_description() {
    let client = make_iot_client().await;

    let props = aws_sdk_iot::types::ThingGroupProperties::builder()
        .thing_group_description("initial-description")
        .build();
    client
        .create_thing_group()
        .thing_group_name("desc-grp")
        .thing_group_properties(props)
        .send()
        .await
        .unwrap();

    let new_props = aws_sdk_iot::types::ThingGroupProperties::builder()
        .thing_group_description("new description")
        .build();
    client
        .update_thing_group()
        .thing_group_name("desc-grp")
        .thing_group_properties(new_props)
        .send()
        .await
        .unwrap();

    let desc = client
        .describe_thing_group()
        .thing_group_name("desc-grp")
        .send()
        .await
        .unwrap();
    assert_eq!(
        desc.thing_group_properties()
            .unwrap()
            .thing_group_description()
            .unwrap(),
        "new description"
    );
}

// ============================================================================
// Ported from moto: test_iot_billing_groups.py
// ============================================================================

// Ported from moto: test_iot_billing_groups.py::test_create_billing_group (with duplicate error)
#[tokio::test]
async fn test_moto_create_billing_group_duplicate() {
    let client = make_iot_client().await;

    let props = aws_sdk_iot::types::BillingGroupProperties::builder()
        .billing_group_description("Test billing group")
        .build();
    let resp = client
        .create_billing_group()
        .billing_group_name("test-bg-dup")
        .billing_group_properties(props.clone())
        .send()
        .await
        .unwrap();
    assert_eq!(resp.billing_group_name().unwrap(), "test-bg-dup");
    assert!(resp.billing_group_arn().is_some());
    assert!(resp.billing_group_id().is_some());

    // Duplicate should fail
    let result = client
        .create_billing_group()
        .billing_group_name("test-bg-dup")
        .billing_group_properties(props)
        .send()
        .await;
    assert!(result.is_err());
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("ResourceAlreadyExistsException"),
        "Expected ResourceAlreadyExistsException, got: {err_str}"
    );
}

// Ported from moto: test_iot_billing_groups.py::test_describe_billing_group (non-existent)
#[tokio::test]
async fn test_moto_describe_nonexistent_billing_group() {
    let client = make_iot_client().await;

    let result = client
        .describe_billing_group()
        .billing_group_name("non-existent-bg")
        .send()
        .await;
    assert!(result.is_err());
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

// Ported from moto: test_iot_billing_groups.py::test_add_thing_to_billing_group + test_list_things_in_billing_group
#[tokio::test]
async fn test_moto_billing_group_add_list_things() {
    let client = make_iot_client().await;

    client
        .create_billing_group()
        .billing_group_name("bg-list")
        .send()
        .await
        .unwrap();

    for i in 0..5 {
        let name = format!("thing-bg-{i}");
        client
            .create_thing()
            .thing_name(&name)
            .send()
            .await
            .unwrap();
        client
            .add_thing_to_billing_group()
            .billing_group_name("bg-list")
            .thing_name(&name)
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_things_in_billing_group()
        .billing_group_name("bg-list")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.things().len(), 5);
}

// Ported from moto: test_iot_billing_groups.py::test_remove_thing_from_billing_group
#[tokio::test]
async fn test_moto_remove_thing_from_billing_group() {
    let client = make_iot_client().await;

    client
        .create_billing_group()
        .billing_group_name("bg-rem")
        .send()
        .await
        .unwrap();
    client
        .create_thing()
        .thing_name("thing-rem")
        .send()
        .await
        .unwrap();
    client
        .add_thing_to_billing_group()
        .billing_group_name("bg-rem")
        .thing_name("thing-rem")
        .send()
        .await
        .unwrap();

    client
        .remove_thing_from_billing_group()
        .billing_group_name("bg-rem")
        .thing_name("thing-rem")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_things_in_billing_group()
        .billing_group_name("bg-rem")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.things().len(), 0);
}

// Ported from moto: test_iot_billing_groups.py::test_update_billing_group (with describe check)
#[tokio::test]
async fn test_moto_update_billing_group_describe() {
    let client = make_iot_client().await;

    let props = aws_sdk_iot::types::BillingGroupProperties::builder()
        .billing_group_description("Test billing group")
        .build();
    client
        .create_billing_group()
        .billing_group_name("bg-upd-d")
        .billing_group_properties(props)
        .send()
        .await
        .unwrap();

    let new_props = aws_sdk_iot::types::BillingGroupProperties::builder()
        .billing_group_description("Updated test billing group")
        .build();
    let resp = client
        .update_billing_group()
        .billing_group_name("bg-upd-d")
        .billing_group_properties(new_props)
        .send()
        .await
        .unwrap();
    assert!(resp.version() > 0);

    let desc = client
        .describe_billing_group()
        .billing_group_name("bg-upd-d")
        .send()
        .await
        .unwrap();
    assert_eq!(
        desc.billing_group_properties()
            .unwrap()
            .billing_group_description()
            .unwrap(),
        "Updated test billing group"
    );
}

// ============================================================================
// Ported from moto: test_iot_certificates.py
// ============================================================================

// Ported from moto: test_iot_certificates.py::test_create_key_and_certificate
#[tokio::test]
async fn test_moto_create_key_and_certificate() {
    let client = make_iot_client().await;

    let cert = client
        .create_keys_and_certificate()
        .set_as_active(true)
        .send()
        .await
        .unwrap();
    assert!(cert.certificate_arn().is_some());
    assert!(cert.certificate_id().is_some());
    assert!(
        cert.certificate_pem()
            .unwrap()
            .starts_with("-----BEGIN CERTIFICATE-----")
    );
    assert!(
        cert.key_pair()
            .unwrap()
            .public_key()
            .unwrap()
            .starts_with("-----BEGIN PUBLIC KEY-----")
    );
    assert!(
        cert.key_pair()
            .unwrap()
            .private_key()
            .unwrap()
            .starts_with("-----BEGIN RSA PRIVATE KEY-----")
    );
}

// Ported from moto: test_iot_certificates.py::test_describe_certificate_by_id
#[tokio::test]
async fn test_moto_describe_certificate_by_id() {
    let client = make_iot_client().await;

    let cert = client
        .create_keys_and_certificate()
        .set_as_active(true)
        .send()
        .await
        .unwrap();
    let cert_id = cert.certificate_id().unwrap();

    let desc = client
        .describe_certificate()
        .certificate_id(cert_id)
        .send()
        .await
        .unwrap();
    let cert_desc = desc.certificate_description().unwrap();
    assert!(cert_desc.certificate_arn().is_some());
    assert!(cert_desc.certificate_id().is_some());
    assert!(cert_desc.certificate_pem().is_some());
    assert_eq!(cert_desc.status().unwrap().as_str(), "ACTIVE");
}

// Ported from moto: test_iot_certificates.py::test_list_certificates (with update to REVOKED)
#[tokio::test]
async fn test_moto_list_certificates_and_revoke() {
    let client = make_iot_client().await;

    let cert = client
        .create_keys_and_certificate()
        .set_as_active(true)
        .send()
        .await
        .unwrap();
    let cert_id = cert.certificate_id().unwrap();

    let res = client.list_certificates().send().await.unwrap();
    assert!(!res.certificates().is_empty());
    for c in res.certificates() {
        assert!(c.certificate_arn().is_some());
        assert!(c.certificate_id().is_some());
        assert!(c.status().is_some());
        assert!(c.creation_date().is_some());
    }

    client
        .update_certificate()
        .certificate_id(cert_id)
        .new_status(aws_sdk_iot::types::CertificateStatus::Revoked)
        .send()
        .await
        .unwrap();
    let desc = client
        .describe_certificate()
        .certificate_id(cert_id)
        .send()
        .await
        .unwrap();
    assert_eq!(
        desc.certificate_description()
            .unwrap()
            .status()
            .unwrap()
            .as_str(),
        "REVOKED"
    );
}

// Ported from moto: test_iot_certificates.py::test_certs_create_inactive
#[tokio::test]
async fn test_moto_certs_create_inactive() {
    let client = make_iot_client().await;

    let cert = client
        .create_keys_and_certificate()
        .set_as_active(false)
        .send()
        .await
        .unwrap();
    let cert_id = cert.certificate_id().unwrap();

    let desc = client
        .describe_certificate()
        .certificate_id(cert_id)
        .send()
        .await
        .unwrap();
    assert_eq!(
        desc.certificate_description()
            .unwrap()
            .status()
            .unwrap()
            .as_str(),
        "INACTIVE"
    );

    client
        .update_certificate()
        .certificate_id(cert_id)
        .new_status(aws_sdk_iot::types::CertificateStatus::Active)
        .send()
        .await
        .unwrap();
    let desc2 = client
        .describe_certificate()
        .certificate_id(cert_id)
        .send()
        .await
        .unwrap();
    assert_eq!(
        desc2
            .certificate_description()
            .unwrap()
            .status()
            .unwrap()
            .as_str(),
        "ACTIVE"
    );
}

// Ported from moto: test_iot_certificates.py::test_delete_certificate_with_status
#[tokio::test]
async fn test_moto_delete_certificate_with_status_revoked() {
    let client = make_iot_client().await;

    let cert = client
        .create_keys_and_certificate()
        .set_as_active(true)
        .send()
        .await
        .unwrap();
    let cert_id = cert.certificate_id().unwrap();

    client
        .update_certificate()
        .certificate_id(cert_id)
        .new_status(aws_sdk_iot::types::CertificateStatus::Revoked)
        .send()
        .await
        .unwrap();
    client
        .delete_certificate()
        .certificate_id(cert_id)
        .send()
        .await
        .unwrap();
    let res = client.list_certificates().send().await.unwrap();
    assert!(res.certificates().is_empty());
}

// Ported from moto: test_iot_certificates.py::test_delete_certificate_with_status (INACTIVE)
#[tokio::test]
async fn test_moto_delete_certificate_with_status_inactive() {
    let client = make_iot_client().await;

    let cert = client
        .create_keys_and_certificate()
        .set_as_active(true)
        .send()
        .await
        .unwrap();
    let cert_id = cert.certificate_id().unwrap();

    client
        .update_certificate()
        .certificate_id(cert_id)
        .new_status(aws_sdk_iot::types::CertificateStatus::Inactive)
        .send()
        .await
        .unwrap();
    client
        .delete_certificate()
        .certificate_id(cert_id)
        .send()
        .await
        .unwrap();
    let res = client.list_certificates().send().await.unwrap();
    assert!(res.certificates().is_empty());
}

// Ported from moto: test_iot_certificates.py::test_register_certificate_without_ca
#[tokio::test]
async fn test_moto_register_certificate_without_ca_flow() {
    let client = make_iot_client().await;

    let cert = client
        .create_keys_and_certificate()
        .set_as_active(true)
        .send()
        .await
        .unwrap();
    let cert_id = cert.certificate_id().unwrap();
    let cert_pem = cert.certificate_pem().unwrap().to_string();

    client
        .update_certificate()
        .certificate_id(cert_id)
        .new_status(aws_sdk_iot::types::CertificateStatus::Revoked)
        .send()
        .await
        .unwrap();
    client
        .delete_certificate()
        .certificate_id(cert_id)
        .send()
        .await
        .unwrap();

    // Register without CA
    let reg = client
        .register_certificate_without_ca()
        .certificate_pem(&cert_pem)
        .status(aws_sdk_iot::types::CertificateStatus::Inactive)
        .send()
        .await
        .unwrap();
    assert!(reg.certificate_id().is_some());
    assert!(reg.certificate_arn().is_some());
    let new_cert_id = reg.certificate_id().unwrap().to_string();

    let res = client.list_certificates().send().await.unwrap();
    assert_eq!(res.certificates().len(), 1);

    client
        .delete_certificate()
        .certificate_id(&new_cert_id)
        .send()
        .await
        .unwrap();
    let res = client.list_certificates().send().await.unwrap();
    assert!(res.certificates().is_empty());
}

// ============================================================================
// Ported from moto: test_iot_policies.py
// ============================================================================

// Ported from moto: test_iot_policies.py::test_policy (full CRUD)
#[tokio::test]
async fn test_moto_policy_crud() {
    let client = make_iot_client().await;

    let name = "moto-pol";
    let doc = "{}";
    let policy = client
        .create_policy()
        .policy_name(name)
        .policy_document(doc)
        .send()
        .await
        .unwrap();
    assert_eq!(policy.policy_name().unwrap(), name);
    assert!(policy.policy_arn().is_some());
    assert_eq!(policy.policy_document().unwrap(), doc);
    assert_eq!(policy.policy_version_id().unwrap(), "1");

    let get = client.get_policy().policy_name(name).send().await.unwrap();
    assert_eq!(get.policy_name().unwrap(), name);
    assert!(get.policy_arn().is_some());
    assert_eq!(get.policy_document().unwrap(), doc);
    assert_eq!(get.default_version_id().unwrap(), "1");

    let list = client.list_policies().send().await.unwrap();
    assert_eq!(list.policies().len(), 1);

    client
        .delete_policy()
        .policy_name(name)
        .send()
        .await
        .unwrap();
    let list = client.list_policies().send().await.unwrap();
    assert_eq!(list.policies().len(), 0);
}

// Ported from moto: test_iot_policies.py::test_policy_versions
#[tokio::test]
async fn test_moto_policy_versions() {
    let client = make_iot_client().await;

    let policy_name = "pv-pol";
    client
        .create_policy()
        .policy_name(policy_name)
        .policy_document("{}")
        .send()
        .await
        .unwrap();

    // Create version 2 (default)
    let pv1 = client
        .create_policy_version()
        .policy_name(policy_name)
        .policy_document(r#"{"version":"version_1"}"#)
        .set_as_default(true)
        .send()
        .await
        .unwrap();
    assert!(pv1.policy_arn().is_some());
    assert_eq!(pv1.policy_version_id().unwrap(), "2");
    assert!(pv1.is_default_version());

    // Create version 3 (not default)
    let pv2 = client
        .create_policy_version()
        .policy_name(policy_name)
        .policy_document(r#"{"version":"version_2"}"#)
        .set_as_default(false)
        .send()
        .await
        .unwrap();
    assert_eq!(pv2.policy_version_id().unwrap(), "3");
    assert!(!pv2.is_default_version());

    // GetPolicy should return default version (version_1)
    let get = client
        .get_policy()
        .policy_name(policy_name)
        .send()
        .await
        .unwrap();
    assert_eq!(get.policy_document().unwrap(), r#"{"version":"version_1"}"#);
    assert_eq!(get.default_version_id().unwrap(), "2");

    // Create versions 4 and 5
    let pv3 = client
        .create_policy_version()
        .policy_name(policy_name)
        .policy_document(r#"{"version":"version_3"}"#)
        .set_as_default(false)
        .send()
        .await
        .unwrap();
    assert_eq!(pv3.policy_version_id().unwrap(), "4");

    let pv4 = client
        .create_policy_version()
        .policy_name(policy_name)
        .policy_document(r#"{"version":"version_4"}"#)
        .set_as_default(false)
        .send()
        .await
        .unwrap();
    assert_eq!(pv4.policy_version_id().unwrap(), "5");

    // List versions: should be 5
    let versions = client
        .list_policy_versions()
        .policy_name(policy_name)
        .send()
        .await
        .unwrap();
    assert_eq!(versions.policy_versions().len(), 5);

    // Exactly 1 default
    let default_count = versions
        .policy_versions()
        .iter()
        .filter(|v| v.is_default_version())
        .count();
    assert_eq!(default_count, 1);

    // Set default to version 5
    client
        .set_default_policy_version()
        .policy_name(policy_name)
        .policy_version_id("5")
        .send()
        .await
        .unwrap();
    let get = client
        .get_policy()
        .policy_name(policy_name)
        .send()
        .await
        .unwrap();
    assert_eq!(get.policy_document().unwrap(), r#"{"version":"version_4"}"#);
    assert_eq!(get.default_version_id().unwrap(), "5");

    // Max versions (5) reached - creating another should fail
    let err = client
        .create_policy_version()
        .policy_name(policy_name)
        .policy_document(r#"{"version":"version_5"}"#)
        .set_as_default(false)
        .send()
        .await;
    assert!(err.is_err());

    // Delete non-default versions
    client
        .delete_policy_version()
        .policy_name(policy_name)
        .policy_version_id("1")
        .send()
        .await
        .unwrap();
    let versions = client
        .list_policy_versions()
        .policy_name(policy_name)
        .send()
        .await
        .unwrap();
    assert_eq!(versions.policy_versions().len(), 4);

    client
        .delete_policy_version()
        .policy_name(policy_name)
        .policy_version_id("2")
        .send()
        .await
        .unwrap();
    client
        .delete_policy_version()
        .policy_name(policy_name)
        .policy_version_id("3")
        .send()
        .await
        .unwrap();
    client
        .delete_policy_version()
        .policy_name(policy_name)
        .policy_version_id("4")
        .send()
        .await
        .unwrap();
    let versions = client
        .list_policy_versions()
        .policy_name(policy_name)
        .send()
        .await
        .unwrap();
    assert_eq!(versions.policy_versions().len(), 1);

    // Cannot delete default version
    let err = client
        .delete_policy_version()
        .policy_name(policy_name)
        .policy_version_id("5")
        .send()
        .await;
    assert!(err.is_err());
    let err_str = format!("{:?}", err.unwrap_err());
    assert!(
        err_str.contains("Cannot delete the default"),
        "Expected default version error, got: {err_str}"
    );
}

// Ported from moto: test_iot_policies.py::test_attach_policy
#[tokio::test]
async fn test_moto_attach_policy_to_cert() {
    let client = make_iot_client().await;

    client
        .create_policy()
        .policy_name("ap-pol")
        .policy_document("{}")
        .send()
        .await
        .unwrap();
    let cert = client
        .create_keys_and_certificate()
        .set_as_active(true)
        .send()
        .await
        .unwrap();
    let cert_arn = cert.certificate_arn().unwrap();

    client
        .attach_policy()
        .policy_name("ap-pol")
        .target(cert_arn)
        .send()
        .await
        .unwrap();

    let res = client
        .list_attached_policies()
        .target(cert_arn)
        .send()
        .await
        .unwrap();
    assert_eq!(res.policies().len(), 1);
    assert_eq!(res.policies()[0].policy_name().unwrap(), "ap-pol");
}

// Ported from moto: test_iot_policies.py::test_detach_policy
#[tokio::test]
async fn test_moto_detach_policy_from_cert() {
    let client = make_iot_client().await;

    client
        .create_policy()
        .policy_name("dp-pol")
        .policy_document("{}")
        .send()
        .await
        .unwrap();
    let cert = client
        .create_keys_and_certificate()
        .set_as_active(true)
        .send()
        .await
        .unwrap();
    let cert_arn = cert.certificate_arn().unwrap();

    client
        .attach_policy()
        .policy_name("dp-pol")
        .target(cert_arn)
        .send()
        .await
        .unwrap();
    let res = client
        .list_attached_policies()
        .target(cert_arn)
        .send()
        .await
        .unwrap();
    assert_eq!(res.policies().len(), 1);

    client
        .detach_policy()
        .policy_name("dp-pol")
        .target(cert_arn)
        .send()
        .await
        .unwrap();
    let res = client
        .list_attached_policies()
        .target(cert_arn)
        .send()
        .await
        .unwrap();
    assert!(res.policies().is_empty());
}

// Ported from moto: test_iot_policies.py::test_list_targets_for_policy_empty
#[tokio::test]
async fn test_moto_list_targets_for_policy_empty() {
    let client = make_iot_client().await;

    client
        .create_policy()
        .policy_name("ltp-pol")
        .policy_document("{}")
        .send()
        .await
        .unwrap();
    let res = client
        .list_targets_for_policy()
        .policy_name("ltp-pol")
        .send()
        .await
        .unwrap();
    assert_eq!(res.targets().len(), 0);
}

// Ported from moto: test_iot_policies.py::test_list_targets_for_policy_one_attached_certificate
#[tokio::test]
async fn test_moto_list_targets_for_policy_with_cert() {
    let client = make_iot_client().await;

    client
        .create_policy()
        .policy_name("ltp-cert-pol")
        .policy_document("{}")
        .send()
        .await
        .unwrap();
    let cert = client
        .create_keys_and_certificate()
        .set_as_active(true)
        .send()
        .await
        .unwrap();
    let cert_arn = cert.certificate_arn().unwrap();

    client
        .attach_policy()
        .policy_name("ltp-cert-pol")
        .target(cert_arn)
        .send()
        .await
        .unwrap();

    let res = client
        .list_targets_for_policy()
        .policy_name("ltp-cert-pol")
        .send()
        .await
        .unwrap();
    assert_eq!(res.targets().len(), 1);
    assert_eq!(res.targets()[0], cert_arn);
}

// Ported from moto: test_iot_policies.py::test_list_targets_for_policy_resource_not_found
#[tokio::test]
async fn test_moto_list_targets_for_nonexistent_policy() {
    let client = make_iot_client().await;

    let result = client
        .list_targets_for_policy()
        .policy_name("NON_EXISTENT_POLICY")
        .send()
        .await;
    assert!(result.is_err());
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

// Ported from moto: test_iot_policies.py::test_create_policy_fails_when_name_taken
#[tokio::test]
async fn test_moto_create_policy_duplicate_fails() {
    let client = make_iot_client().await;

    client
        .create_policy()
        .policy_name("dup-pol")
        .policy_document("{}")
        .send()
        .await
        .unwrap();

    let result = client
        .create_policy()
        .policy_name("dup-pol")
        .policy_document(r#"{"Version":"2012-10-17"}"#)
        .send()
        .await;
    assert!(result.is_err());
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("ResourceAlreadyExistsException"),
        "Expected ResourceAlreadyExistsException, got: {err_str}"
    );

    // Original policy should be unchanged
    let current = client
        .get_policy()
        .policy_name("dup-pol")
        .send()
        .await
        .unwrap();
    assert_eq!(current.policy_document().unwrap(), "{}");
}

// Ported from moto: test_iot_policies.py::test_attach_policy_to_thing_group
#[tokio::test]
async fn test_moto_attach_policy_to_thing_group() {
    let client = make_iot_client().await;

    client
        .create_policy()
        .policy_name("tgp-pol")
        .policy_document("{}")
        .send()
        .await
        .unwrap();
    let tg = client
        .create_thing_group()
        .thing_group_name("tgp-group")
        .send()
        .await
        .unwrap();
    let tg_arn = tg.thing_group_arn().unwrap();

    client
        .attach_policy()
        .policy_name("tgp-pol")
        .target(tg_arn)
        .send()
        .await
        .unwrap();

    let res = client
        .list_attached_policies()
        .target(tg_arn)
        .send()
        .await
        .unwrap();
    assert_eq!(res.policies().len(), 1);
    assert_eq!(res.policies()[0].policy_name().unwrap(), "tgp-pol");
}

// ============================================================================
// Ported from moto: test_iot_rolealias.py
// ============================================================================

// Ported from moto: test_iot_rolealias.py::test_create_role_alias (with ARN check)
#[tokio::test]
async fn test_moto_create_role_alias_arn() {
    let client = make_iot_client().await;

    let resp = client
        .create_role_alias()
        .role_alias("test-role-alias-arn")
        .role_arn("arn:aws:iam::123456789012:role/my-role")
        .credential_duration_seconds(1234)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.role_alias().unwrap(), "test-role-alias-arn");
    assert!(
        resp.role_alias_arn()
            .unwrap()
            .contains("rolealias/test-role-alias-arn")
    );
}

// Ported from moto: test_iot_rolealias.py::test_create_role_alias_twice
#[tokio::test]
async fn test_moto_create_role_alias_twice() {
    let client = make_iot_client().await;

    client
        .create_role_alias()
        .role_alias("dup-alias")
        .role_arn("arn:aws:iam::123456789012:role/my-role")
        .send()
        .await
        .unwrap();

    let result = client
        .create_role_alias()
        .role_alias("dup-alias")
        .role_arn("arn:aws:iam::123456789012:role/my-role")
        .send()
        .await;
    assert!(result.is_err());
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("ResourceAlreadyExistsException"),
        "Expected ResourceAlreadyExistsException, got: {err_str}"
    );
}

// Ported from moto: test_iot_rolealias.py::test_delete_nonexistent_role_alias
#[tokio::test]
async fn test_moto_delete_nonexistent_role_alias() {
    let client = make_iot_client().await;

    let result = client
        .delete_role_alias()
        .role_alias("no-such-alias")
        .send()
        .await;
    assert!(result.is_err());
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

// Ported from moto: test_iot_rolealias.py::test_describe_role_alias (default credential duration)
#[tokio::test]
async fn test_moto_describe_role_alias_default_duration() {
    let client = make_iot_client().await;

    client
        .create_role_alias()
        .role_alias("def-dur-alias")
        .role_arn("arn:aws:iam::123456789012:role/my-role")
        .send()
        .await
        .unwrap();

    let desc = client
        .describe_role_alias()
        .role_alias("def-dur-alias")
        .send()
        .await
        .unwrap();
    let rad = desc.role_alias_description().unwrap();
    assert_eq!(rad.role_alias().unwrap(), "def-dur-alias");
    assert_eq!(
        rad.role_arn().unwrap(),
        "arn:aws:iam::123456789012:role/my-role"
    );
    assert_eq!(rad.credential_duration_seconds(), Some(3600));
    assert!(rad.owner().is_some());
    assert!(rad.creation_date().is_some());
    assert!(rad.last_modified_date().is_some());
}

// Ported from moto: test_iot_rolealias.py::test_update_role_alias (with verification)
#[tokio::test]
async fn test_moto_update_role_alias_verify() {
    let client = make_iot_client().await;

    client
        .create_role_alias()
        .role_alias("upd-alias-v")
        .role_arn("arn:aws:iam::123456789012:role/my-role")
        .credential_duration_seconds(1234)
        .send()
        .await
        .unwrap();

    client
        .update_role_alias()
        .role_alias("upd-alias-v")
        .role_arn("arn:aws:iam::123456789012:role/other-role")
        .credential_duration_seconds(2345)
        .send()
        .await
        .unwrap();

    let desc = client
        .describe_role_alias()
        .role_alias("upd-alias-v")
        .send()
        .await
        .unwrap();
    let rad = desc.role_alias_description().unwrap();
    assert_eq!(
        rad.role_arn().unwrap(),
        "arn:aws:iam::123456789012:role/other-role"
    );
    assert_eq!(rad.credential_duration_seconds(), Some(2345));
}

// ============================================================================
// Ported from moto: test_iot_domain_configuration.py
// ============================================================================

// Ported from moto: test_iot_domain_configuration.py::test_create_duplicate_domain_configuration_fails
#[tokio::test]
async fn test_moto_create_duplicate_domain_configuration() {
    let client = make_iot_client().await;

    client
        .create_domain_configuration()
        .domain_configuration_name("dupConfig")
        .send()
        .await
        .unwrap();

    let result = client
        .create_domain_configuration()
        .domain_configuration_name("dupConfig")
        .send()
        .await;
    assert!(result.is_err());
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("ResourceAlreadyExistsException"),
        "Expected ResourceAlreadyExistsException, got: {err_str}"
    );
}

// Ported from moto: test_iot_domain_configuration.py::test_describe_nonexistent_domain_configuration
#[tokio::test]
async fn test_moto_describe_nonexistent_domain_configuration() {
    let client = make_iot_client().await;

    let result = client
        .describe_domain_configuration()
        .domain_configuration_name("doesntExist")
        .send()
        .await;
    assert!(result.is_err());
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

// Ported from moto: test_iot_domain_configuration.py::test_describe_domain_configuration (status check)
#[tokio::test]
async fn test_moto_describe_domain_configuration_status() {
    let client = make_iot_client().await;

    client
        .create_domain_configuration()
        .domain_configuration_name("statusConfig")
        .domain_name("example.com")
        .service_type(aws_sdk_iot::types::ServiceType::Data)
        .send()
        .await
        .unwrap();

    let desc = client
        .describe_domain_configuration()
        .domain_configuration_name("statusConfig")
        .send()
        .await
        .unwrap();
    assert_eq!(desc.domain_configuration_name().unwrap(), "statusConfig");
    assert!(desc.domain_configuration_arn().is_some());
    assert_eq!(
        desc.domain_configuration_status().unwrap().as_str(),
        "ENABLED"
    );
    assert_eq!(desc.service_type().unwrap().as_str(), "DATA");
    assert!(desc.last_status_change_date().is_some());
}

// Ported from moto: test_iot_domain_configuration.py::test_update_domain_configuration (status change)
#[tokio::test]
async fn test_moto_update_domain_configuration_status() {
    let client = make_iot_client().await;

    client
        .create_domain_configuration()
        .domain_configuration_name("updStatusConfig")
        .domain_name("example.com")
        .service_type(aws_sdk_iot::types::ServiceType::Data)
        .send()
        .await
        .unwrap();

    client
        .update_domain_configuration()
        .domain_configuration_name("updStatusConfig")
        .domain_configuration_status(aws_sdk_iot::types::DomainConfigurationStatus::Disabled)
        .send()
        .await
        .unwrap();

    let desc = client
        .describe_domain_configuration()
        .domain_configuration_name("updStatusConfig")
        .send()
        .await
        .unwrap();
    assert_eq!(
        desc.domain_configuration_status().unwrap().as_str(),
        "DISABLED"
    );
}

// Ported from moto: test_iot_domain_configuration.py::test_update_nonexistent_domain_configuration
#[tokio::test]
async fn test_moto_update_nonexistent_domain_configuration() {
    let client = make_iot_client().await;

    let result = client
        .update_domain_configuration()
        .domain_configuration_name("doesntExist")
        .send()
        .await;
    assert!(result.is_err());
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

// Ported from moto: test_iot_domain_configuration.py::test_delete_nonexistent_domain_configuration
#[tokio::test]
async fn test_moto_delete_nonexistent_domain_configuration() {
    let client = make_iot_client().await;

    let result = client
        .delete_domain_configuration()
        .domain_configuration_name("doesntExist")
        .send()
        .await;
    assert!(result.is_err());
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

// ============================================================================
// Ported from moto: test_iot_jobs.py
// ============================================================================

// Ported from moto: test_iot_jobs.py::test_create_job (with document and description)
#[tokio::test]
async fn test_moto_create_job() {
    let client = make_iot_client().await;

    let thing = client
        .create_thing()
        .thing_name("moto-job-thing")
        .send()
        .await
        .unwrap();
    let thing_arn = thing.thing_arn().unwrap();

    let job = client
        .create_job()
        .job_id("MotoTestJob")
        .targets(thing_arn)
        .document(r#"{"field":"value"}"#)
        .description("Description")
        .target_selection(aws_sdk_iot::types::TargetSelection::Continuous)
        .send()
        .await
        .unwrap();

    assert_eq!(job.job_id().unwrap(), "MotoTestJob");
    assert!(job.job_arn().is_some());
    assert!(job.description().is_some());
}

// Ported from moto: test_iot_jobs.py::test_list_jobs (multiple jobs)
#[tokio::test]
async fn test_moto_list_jobs_multiple() {
    let client = make_iot_client().await;

    let thing = client
        .create_thing()
        .thing_name("moto-lj-thing")
        .send()
        .await
        .unwrap();
    let thing_arn = thing.thing_arn().unwrap();

    client
        .create_job()
        .job_id("TestJob")
        .targets(thing_arn)
        .document(r#"{"f":"v"}"#)
        .description("D")
        .send()
        .await
        .unwrap();
    client
        .create_job()
        .job_id("TestJob1")
        .targets(thing_arn)
        .document(r#"{"f":"v"}"#)
        .description("D")
        .send()
        .await
        .unwrap();

    let jobs = client.list_jobs().send().await.unwrap();
    assert_eq!(jobs.jobs().len(), 2);
    assert_eq!(jobs.jobs()[0].job_id().unwrap(), "TestJob");
    assert_eq!(jobs.jobs()[1].job_id().unwrap(), "TestJob1");
}

// Ported from moto: test_iot_jobs.py::test_cancel_job (with reason/comment/forceCanceled check)
#[tokio::test]
async fn test_moto_cancel_job_with_reason() {
    let client = make_iot_client().await;

    let thing = client
        .create_thing()
        .thing_name("moto-cj-thing")
        .send()
        .await
        .unwrap();
    let thing_arn = thing.thing_arn().unwrap();

    client
        .create_job()
        .job_id("CancelTestJob")
        .targets(thing_arn)
        .document_source("https://s3-eu-west-1.amazonaws.com/bucket-name/job_document.json")
        .target_selection(aws_sdk_iot::types::TargetSelection::Continuous)
        .send()
        .await
        .unwrap();

    let cancel = client
        .cancel_job()
        .job_id("CancelTestJob")
        .reason_code("Because")
        .comment("You are")
        .send()
        .await
        .unwrap();
    assert_eq!(cancel.job_id().unwrap(), "CancelTestJob");
    assert!(cancel.job_arn().is_some());

    let desc = client
        .describe_job()
        .job_id("CancelTestJob")
        .send()
        .await
        .unwrap();
    let job = desc.job().unwrap();
    assert_eq!(job.status().unwrap().as_str(), "CANCELED");
    assert_eq!(job.force_canceled(), Some(false));
    assert_eq!(job.reason_code().unwrap(), "Because");
    assert_eq!(job.comment().unwrap(), "You are");
}

// Ported from moto: test_iot_jobs.py::test_delete_job
#[tokio::test]
async fn test_moto_delete_job() {
    let client = make_iot_client().await;

    let thing = client
        .create_thing()
        .thing_name("moto-dj-thing")
        .send()
        .await
        .unwrap();
    let thing_arn = thing.thing_arn().unwrap();

    client
        .create_job()
        .job_id("DelTestJob")
        .targets(thing_arn)
        .document_source("https://s3.example.com/doc.json")
        .send()
        .await
        .unwrap();
    client
        .delete_job()
        .job_id("DelTestJob")
        .send()
        .await
        .unwrap();

    let jobs = client.list_jobs().send().await.unwrap();
    assert!(jobs.jobs().is_empty());
}

// Ported from moto: test_iot_jobs.py::test_get_job_document_with_document
#[tokio::test]
async fn test_moto_get_job_document_with_document() {
    let client = make_iot_client().await;

    let thing = client
        .create_thing()
        .thing_name("moto-jdoc-thing")
        .send()
        .await
        .unwrap();
    let thing_arn = thing.thing_arn().unwrap();

    client
        .create_job()
        .job_id("JDocJob")
        .targets(thing_arn)
        .document(r#"{"field":"value"}"#)
        .send()
        .await
        .unwrap();

    let doc = client
        .get_job_document()
        .job_id("JDocJob")
        .send()
        .await
        .unwrap();
    assert_eq!(doc.document().unwrap(), r#"{"field":"value"}"#);
}

// Ported from moto: test_iot_jobs.py::test_get_job_document_with_document_source
#[tokio::test]
async fn test_moto_get_job_document_with_document_source() {
    let client = make_iot_client().await;

    let thing = client
        .create_thing()
        .thing_name("moto-jdsrc-thing")
        .send()
        .await
        .unwrap();
    let thing_arn = thing.thing_arn().unwrap();

    client
        .create_job()
        .job_id("JDocSrcJob")
        .targets(thing_arn)
        .document_source("https://s3.example.com/doc.json")
        .send()
        .await
        .unwrap();

    let doc = client
        .get_job_document()
        .job_id("JDocSrcJob")
        .send()
        .await
        .unwrap();
    // When document_source is used but no document, moto returns empty string
    assert_eq!(doc.document().unwrap(), "");
}

// ============================================================================
// Ported from moto: test_iot_job_executions.py
// ============================================================================

// Ported from moto: test_iot_job_executions.py::test_describe_job_execution
#[tokio::test]
async fn test_moto_describe_job_execution() {
    let client = make_iot_client().await;

    let thing = client
        .create_thing()
        .thing_name("moto-exec-thing")
        .send()
        .await
        .unwrap();
    let thing_arn = thing.thing_arn().unwrap();

    client
        .create_job()
        .job_id("ExecDescJob")
        .targets(thing_arn)
        .document(r#"{"field":"value"}"#)
        .description("Description")
        .target_selection(aws_sdk_iot::types::TargetSelection::Continuous)
        .send()
        .await
        .unwrap();

    let exec = client
        .describe_job_execution()
        .job_id("ExecDescJob")
        .thing_name("moto-exec-thing")
        .send()
        .await
        .unwrap();
    let e = exec.execution().unwrap();
    assert_eq!(e.job_id().unwrap(), "ExecDescJob");
    assert_eq!(e.status().unwrap().as_str(), "QUEUED");
    assert!(e.thing_arn().unwrap().contains("moto-exec-thing"));
    assert!(e.queued_at().is_some());
    assert!(e.last_updated_at().is_some());
}

// Ported from moto: test_iot_job_executions.py::test_cancel_job_execution
#[tokio::test]
async fn test_moto_cancel_job_execution() {
    let client = make_iot_client().await;

    let thing = client
        .create_thing()
        .thing_name("moto-cje-thing")
        .send()
        .await
        .unwrap();
    let thing_arn = thing.thing_arn().unwrap();

    client
        .create_job()
        .job_id("CJEJob")
        .targets(thing_arn)
        .document(r#"{"field":"value"}"#)
        .send()
        .await
        .unwrap();

    client
        .cancel_job_execution()
        .job_id("CJEJob")
        .thing_name("moto-cje-thing")
        .send()
        .await
        .unwrap();

    let exec = client
        .describe_job_execution()
        .job_id("CJEJob")
        .thing_name("moto-cje-thing")
        .send()
        .await
        .unwrap();
    assert_eq!(
        exec.execution().unwrap().status().unwrap().as_str(),
        "CANCELED"
    );
}

// Ported from moto: test_iot_job_executions.py::test_list_job_executions_for_job (multiple things)
#[tokio::test]
async fn test_moto_list_job_executions_for_job_multiple() {
    let client = make_iot_client().await;

    let mut thing_arns = Vec::new();
    for i in 0..5 {
        let t = client
            .create_thing()
            .thing_name(format!("moto-ljefj-thing-{i}"))
            .send()
            .await
            .unwrap();
        thing_arns.push(t.thing_arn().unwrap().to_string());
    }

    let mut builder = client
        .create_job()
        .job_id("LJEFJJob")
        .document(r#"{"field":"value"}"#)
        .description("Description");
    for arn in &thing_arns {
        builder = builder.targets(arn);
    }
    builder.send().await.unwrap();

    let execs = client
        .list_job_executions_for_job()
        .job_id("LJEFJJob")
        .send()
        .await
        .unwrap();
    assert_eq!(execs.execution_summaries().len(), 5);
}

// Ported from moto: test_iot_job_executions.py::test_list_job_executions_for_thing
#[tokio::test]
async fn test_moto_list_job_executions_for_thing() {
    let client = make_iot_client().await;

    let thing = client
        .create_thing()
        .thing_name("moto-ljeft-thing")
        .send()
        .await
        .unwrap();
    let thing_arn = thing.thing_arn().unwrap();

    client
        .create_job()
        .job_id("LJEFTJob")
        .targets(thing_arn)
        .document(r#"{"field":"value"}"#)
        .send()
        .await
        .unwrap();

    let exec = client
        .list_job_executions_for_thing()
        .thing_name("moto-ljeft-thing")
        .send()
        .await
        .unwrap();
    assert_eq!(exec.execution_summaries()[0].job_id().unwrap(), "LJEFTJob");
}

// ============================================================================
// Ported from moto: test_iot_topic_rules.py
// ============================================================================

// Ported from moto: test_iot_topic_rules.py::test_topic_rule_create (with duplicate)
#[tokio::test]
async fn test_moto_topic_rule_create_duplicate() {
    let client = make_iot_client().await;

    let payload = aws_sdk_iot::types::TopicRulePayload::builder()
        .sql("SELECT * FROM 'topic/*' WHERE something > 0")
        .description("my-description")
        .actions(aws_sdk_iot::types::Action::builder().build())
        .aws_iot_sql_version("2016-03-23")
        .rule_disabled(false)
        .build()
        .unwrap();

    client
        .create_topic_rule()
        .rule_name("motoRule")
        .topic_rule_payload(payload.clone())
        .send()
        .await
        .unwrap();

    let result = client
        .create_topic_rule()
        .rule_name("motoRule")
        .topic_rule_payload(payload)
        .send()
        .await;
    assert!(result.is_err());
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("ResourceAlreadyExistsException"),
        "Expected ResourceAlreadyExistsException, got: {err_str}"
    );
}

// Ported from moto: test_iot_topic_rules.py::test_topic_rule_get (with full field checks)
#[tokio::test]
async fn test_moto_topic_rule_get_full() {
    let client = make_iot_client().await;

    let payload = aws_sdk_iot::types::TopicRulePayload::builder()
        .sql("SELECT * FROM 'topic/*' WHERE something > 0")
        .description("my-description")
        .actions(aws_sdk_iot::types::Action::builder().build())
        .aws_iot_sql_version("2016-03-23")
        .rule_disabled(false)
        .build()
        .unwrap();

    client
        .create_topic_rule()
        .rule_name("fullRule")
        .topic_rule_payload(payload)
        .send()
        .await
        .unwrap();

    let resp = client
        .get_topic_rule()
        .rule_name("fullRule")
        .send()
        .await
        .unwrap();
    assert!(resp.rule_arn().is_some());
    let rule = resp.rule().unwrap();
    assert_eq!(rule.rule_name().unwrap(), "fullRule");
    assert_eq!(
        rule.sql().unwrap(),
        "SELECT * FROM 'topic/*' WHERE something > 0"
    );
    assert_eq!(rule.description().unwrap(), "my-description");
    assert_eq!(rule.aws_iot_sql_version().unwrap(), "2016-03-23");
    assert_eq!(rule.rule_disabled(), Some(false));
    assert!(rule.created_at().is_some());
}

// Ported from moto: test_iot_topic_rules.py::test_topic_rule_get (non-existent)
#[tokio::test]
async fn test_moto_topic_rule_get_nonexistent() {
    let client = make_iot_client().await;

    let result = client.get_topic_rule().rule_name("noSuchRule").send().await;
    assert!(result.is_err());
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

// Ported from moto: test_iot_topic_rules.py::test_topic_rule_replace (non-existent first, then replace)
#[tokio::test]
async fn test_moto_topic_rule_replace_nonexistent() {
    let client = make_iot_client().await;

    let payload = aws_sdk_iot::types::TopicRulePayload::builder()
        .sql("SELECT * FROM 'topic/*'")
        .actions(aws_sdk_iot::types::Action::builder().build())
        .build()
        .unwrap();

    let result = client
        .replace_topic_rule()
        .rule_name("noRule")
        .topic_rule_payload(payload)
        .send()
        .await;
    assert!(result.is_err());
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

// Ported from moto: test_iot_topic_rules.py::test_topic_rule_disable (non-existent)
#[tokio::test]
async fn test_moto_topic_rule_disable_nonexistent() {
    let client = make_iot_client().await;

    let result = client
        .disable_topic_rule()
        .rule_name("noDisRule")
        .send()
        .await;
    assert!(result.is_err());
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

// Ported from moto: test_iot_topic_rules.py::test_topic_rule_enable (non-existent + enable after disable)
#[tokio::test]
async fn test_moto_topic_rule_enable_nonexistent() {
    let client = make_iot_client().await;

    let result = client
        .enable_topic_rule()
        .rule_name("noEnRule")
        .send()
        .await;
    assert!(result.is_err());
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

// Ported from moto: test_iot_topic_rules.py::test_topic_rule_enable (full enable after create disabled)
#[tokio::test]
async fn test_moto_topic_rule_enable_after_disabled_create() {
    let client = make_iot_client().await;

    let payload = aws_sdk_iot::types::TopicRulePayload::builder()
        .sql("SELECT * FROM 'topic/*'")
        .actions(aws_sdk_iot::types::Action::builder().build())
        .rule_disabled(true)
        .build()
        .unwrap();

    client
        .create_topic_rule()
        .rule_name("disabledRule")
        .topic_rule_payload(payload)
        .send()
        .await
        .unwrap();

    // Verify it's disabled
    let resp = client
        .get_topic_rule()
        .rule_name("disabledRule")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.rule().unwrap().rule_disabled(), Some(true));

    // Enable it
    client
        .enable_topic_rule()
        .rule_name("disabledRule")
        .send()
        .await
        .unwrap();
    let resp = client
        .get_topic_rule()
        .rule_name("disabledRule")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.rule().unwrap().rule_disabled(), Some(false));
}

// Ported from moto: test_iot_topic_rules.py::test_topic_rule_delete (non-existent)
#[tokio::test]
async fn test_moto_topic_rule_delete_nonexistent() {
    let client = make_iot_client().await;

    let result = client
        .delete_topic_rule()
        .rule_name("noDelRule")
        .send()
        .await;
    assert!(result.is_err());
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

// Ported from moto: test_iot_topic_rules.py::test_topic_rule_list (with field checks)
#[tokio::test]
async fn test_moto_topic_rule_list_fields() {
    let client = make_iot_client().await;

    let payload = aws_sdk_iot::types::TopicRulePayload::builder()
        .sql("SELECT * FROM 'topic/*' WHERE something > 0")
        .description("my-description")
        .actions(aws_sdk_iot::types::Action::builder().build())
        .rule_disabled(false)
        .build()
        .unwrap();

    client
        .create_topic_rule()
        .rule_name("listRule1")
        .topic_rule_payload(payload.clone())
        .send()
        .await
        .unwrap();
    client
        .create_topic_rule()
        .rule_name("listRule2")
        .topic_rule_payload(payload)
        .send()
        .await
        .unwrap();

    let res = client.list_topic_rules().send().await.unwrap();
    assert_eq!(res.rules().len(), 2);
    for rule in res.rules() {
        assert!(rule.rule_name().is_some());
        assert!(rule.created_at().is_some());
        assert!(rule.rule_arn().is_some());
        assert_eq!(rule.rule_disabled(), Some(false));
    }
}

// ==================== New edge-case tests ====================

#[tokio::test]
async fn test_delete_thing_with_correct_expected_version() {
    let client = make_iot_client().await;

    client
        .create_thing()
        .thing_name("ev-del-ok")
        .send()
        .await
        .unwrap();

    client
        .delete_thing()
        .thing_name("ev-del-ok")
        .expected_version(1)
        .send()
        .await
        .expect("delete_thing with correct expectedVersion should succeed");

    let result = client.describe_thing().thing_name("ev-del-ok").send().await;
    assert!(result.is_err(), "thing should be gone after delete");
}

#[tokio::test]
async fn test_delete_thing_with_wrong_expected_version_fails() {
    let client = make_iot_client().await;

    client
        .create_thing()
        .thing_name("ev-del-fail")
        .send()
        .await
        .unwrap();

    let result = client
        .delete_thing()
        .thing_name("ev-del-fail")
        .expected_version(99)
        .send()
        .await;
    assert!(result.is_err(), "wrong expectedVersion should fail");
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("VersionConflict"),
        "expected VersionConflictException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_update_thing_wrong_expected_version_fails() {
    let client = make_iot_client().await;

    client
        .create_thing()
        .thing_name("ev-upd-fail")
        .send()
        .await
        .unwrap();

    let attrs = aws_sdk_iot::types::AttributePayload::builder()
        .attributes("k", "v")
        .build();

    let result = client
        .update_thing()
        .thing_name("ev-upd-fail")
        .expected_version(42)
        .attribute_payload(attrs)
        .send()
        .await;
    assert!(
        result.is_err(),
        "wrong expectedVersion on update should fail"
    );
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("VersionConflict"),
        "expected VersionConflictException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_update_thing_remove_thing_type() {
    let client = make_iot_client().await;

    client
        .create_thing_type()
        .thing_type_name("removable-type")
        .send()
        .await
        .unwrap();

    client
        .create_thing()
        .thing_name("rmt-thing")
        .thing_type_name("removable-type")
        .send()
        .await
        .unwrap();

    let desc = client
        .describe_thing()
        .thing_name("rmt-thing")
        .send()
        .await
        .unwrap();
    assert_eq!(desc.thing_type_name().unwrap(), "removable-type");

    client
        .update_thing()
        .thing_name("rmt-thing")
        .remove_thing_type(true)
        .send()
        .await
        .expect("update_thing with remove_thing_type should succeed");

    let desc2 = client
        .describe_thing()
        .thing_name("rmt-thing")
        .send()
        .await
        .unwrap();
    assert!(
        desc2.thing_type_name().is_none(),
        "thing type should be removed"
    );
}

// ==================== ThingGroup - parent and expectedVersion ====================

#[tokio::test]
async fn test_create_thing_group_with_parent() {
    let client = make_iot_client().await;

    client
        .create_thing_group()
        .thing_group_name("parent-grp")
        .send()
        .await
        .unwrap();

    let resp = client
        .create_thing_group()
        .thing_group_name("child-grp")
        .parent_group_name("parent-grp")
        .send()
        .await
        .expect("create_thing_group with parent should succeed");
    assert_eq!(resp.thing_group_name().unwrap(), "child-grp");

    let desc = client
        .describe_thing_group()
        .thing_group_name("child-grp")
        .send()
        .await
        .unwrap();
    let meta = desc.thing_group_metadata().unwrap();
    assert_eq!(meta.parent_group_name().unwrap(), "parent-grp");
}

#[tokio::test]
async fn test_delete_thing_group_with_wrong_expected_version_fails() {
    let client = make_iot_client().await;

    client
        .create_thing_group()
        .thing_group_name("ev-grp-fail")
        .send()
        .await
        .unwrap();

    let result = client
        .delete_thing_group()
        .thing_group_name("ev-grp-fail")
        .expected_version(99)
        .send()
        .await;
    assert!(
        result.is_err(),
        "wrong expectedVersion on delete group should fail"
    );
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("VersionConflict"),
        "expected VersionConflictException, got: {err_str}"
    );
}

// ==================== CA Certificate - setAsActive and allowAutoRegistration ====================

#[tokio::test]
async fn test_register_ca_certificate_with_set_active_and_auto_registration() {
    let client = make_iot_client().await;

    let resp = client
        .register_ca_certificate()
        .ca_certificate("-----BEGIN CERTIFICATE-----\nCA_ACTIVE\n-----END CERTIFICATE-----")
        .set_as_active(true)
        .allow_auto_registration(true)
        .send()
        .await
        .expect(
            "register_ca_certificate with setAsActive and allowAutoRegistration should succeed",
        );

    let ca_id = resp.certificate_id().unwrap();

    let desc = client
        .describe_ca_certificate()
        .certificate_id(ca_id)
        .send()
        .await
        .unwrap();

    let ca_desc = desc.certificate_description().unwrap();
    assert_eq!(ca_desc.status().unwrap().as_str(), "ACTIVE");
    assert_eq!(
        ca_desc.auto_registration_status().unwrap().as_str(),
        "ENABLE"
    );
}

#[tokio::test]
async fn test_update_ca_certificate_auto_registration_status() {
    let client = make_iot_client().await;

    let resp = client
        .register_ca_certificate()
        .ca_certificate("-----BEGIN CERTIFICATE-----\nCA_AUTOREG\n-----END CERTIFICATE-----")
        .set_as_active(true)
        .allow_auto_registration(false)
        .send()
        .await
        .unwrap();
    let ca_id = resp.certificate_id().unwrap();

    client
        .update_ca_certificate()
        .certificate_id(ca_id)
        .new_auto_registration_status(aws_sdk_iot::types::AutoRegistrationStatus::Enable)
        .send()
        .await
        .expect("update_ca_certificate with new_auto_registration_status should succeed");

    let desc = client
        .describe_ca_certificate()
        .certificate_id(ca_id)
        .send()
        .await
        .unwrap();

    assert_eq!(
        desc.certificate_description()
            .unwrap()
            .auto_registration_status()
            .unwrap()
            .as_str(),
        "ENABLE"
    );
}

// ==================== Job - force cancel and CONTINUOUS target selection ====================

#[tokio::test]
async fn test_cancel_job_with_force() {
    let client = make_iot_client().await;

    client
        .create_job()
        .job_id("force-cj")
        .targets("arn:aws:iot:us-east-1:123456789012:thing/x")
        .send()
        .await
        .unwrap();

    let resp = client
        .cancel_job()
        .job_id("force-cj")
        .reason_code("FORCE_STOP")
        .comment("force stopped")
        .force(true)
        .send()
        .await
        .expect("cancel_job with force should succeed");
    assert_eq!(resp.job_id().unwrap(), "force-cj");

    let desc = client
        .describe_job()
        .job_id("force-cj")
        .send()
        .await
        .unwrap();
    let job = desc.job().unwrap();
    assert_eq!(job.force_canceled(), Some(true));
}

#[tokio::test]
async fn test_create_job_with_continuous_target_selection() {
    let client = make_iot_client().await;

    let resp = client
        .create_job()
        .job_id("continuous-job")
        .targets("arn:aws:iot:us-east-1:123456789012:thinggroup/my-group")
        .document(r#"{"operation":"reboot"}"#)
        .target_selection(aws_sdk_iot::types::TargetSelection::Continuous)
        .send()
        .await
        .expect("create_job with CONTINUOUS target_selection should succeed");
    assert_eq!(resp.job_id().unwrap(), "continuous-job");

    let desc = client
        .describe_job()
        .job_id("continuous-job")
        .send()
        .await
        .unwrap();
    let job = desc.job().unwrap();
    assert_eq!(
        job.target_selection().unwrap(),
        &aws_sdk_iot::types::TargetSelection::Continuous
    );
}

// ==================== Thing - expectedVersion and removeThingType ====================
