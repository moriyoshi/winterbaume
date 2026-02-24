use std::collections::HashMap;

use serde_json::{Value, json};
use winterbaume_tfstate::{
    IndexKey, OutputValue, Resource, ResourceInstance, ResourceMode, TerraformState, TfStateError,
};

fn make_instance(id: &str, attrs: Value) -> ResourceInstance {
    let mut attributes = attrs;
    if let Some(obj) = attributes.as_object_mut() {
        obj.insert("id".to_string(), json!(id));
    }
    ResourceInstance {
        schema_version: 0,
        attributes,
        sensitive_attributes: vec![],
        dependencies: vec![],
        index_key: None,
        private: None,
        create_before_destroy: None,
        extra: HashMap::new(),
    }
}

fn make_resource(
    rtype: &str,
    name: &str,
    provider: &str,
    instances: Vec<ResourceInstance>,
) -> Resource {
    Resource {
        mode: ResourceMode::Managed,
        resource_type: rtype.to_string(),
        name: name.to_string(),
        provider: provider.to_string(),
        instances,
        module: None,
        extra: HashMap::new(),
    }
}

fn make_state() -> TerraformState {
    let bucket = make_resource(
        "aws_s3_bucket",
        "example",
        r#"provider["registry.terraform.io/hashicorp/aws"]"#,
        vec![make_instance(
            "my-bucket",
            json!({"bucket": "my-bucket", "arn": "arn:aws:s3:::my-bucket"}),
        )],
    );
    let role = make_resource(
        "aws_iam_role",
        "admin",
        r#"provider["registry.terraform.io/hashicorp/aws"]"#,
        vec![make_instance(
            "admin-role",
            json!({"name": "admin", "arn": "arn:aws:iam::123456789012:role/admin"}),
        )],
    );

    TerraformState {
        version: 4,
        serial: 10,
        lineage: "abc-123".to_string(),
        terraform_version: "1.5.0".to_string(),
        outputs: HashMap::from([(
            "bucket_name".to_string(),
            OutputValue {
                value: json!("my-bucket"),
                output_type: json!("string"),
                sensitive: false,
            },
        )]),
        resources: vec![bucket, role],
    }
}

#[test]
fn roundtrip_programmatic() {
    let state = make_state();

    // Serialize
    let json_str = state.to_string_pretty().expect("serialize");

    // Parse back
    let parsed = TerraformState::from_str(&json_str).expect("parse");

    assert_eq!(parsed.version, 4);
    assert_eq!(parsed.serial, 10);
    assert_eq!(parsed.lineage, "abc-123");
    assert_eq!(parsed.terraform_version, "1.5.0");
    assert_eq!(parsed.resources.len(), 2);
    assert_eq!(parsed.outputs.len(), 1);

    let bucket = parsed.get_resource("aws_s3_bucket", "example").unwrap();
    assert_eq!(bucket.primary_instance().unwrap().id(), Some("my-bucket"));

    let role = parsed.get_resource("aws_iam_role", "admin").unwrap();
    assert_eq!(role.primary_instance().unwrap().id(), Some("admin-role"));
}

#[test]
fn roundtrip_realistic_json() {
    let state_json = r#"{
        "version": 4,
        "terraform_version": "1.6.3",
        "serial": 42,
        "lineage": "deadbeef-1234-5678-abcd-ef0123456789",
        "outputs": {
            "vpc_id": {
                "value": "vpc-12345",
                "type": "string"
            }
        },
        "resources": [
            {
                "mode": "managed",
                "type": "aws_s3_bucket",
                "name": "logs",
                "provider": "provider[\"registry.terraform.io/hashicorp/aws\"]",
                "instances": [
                    {
                        "schema_version": 0,
                        "attributes": {
                            "id": "logs-bucket",
                            "bucket": "logs-bucket",
                            "arn": "arn:aws:s3:::logs-bucket"
                        }
                    }
                ]
            },
            {
                "mode": "managed",
                "type": "aws_subnet",
                "name": "private",
                "provider": "provider[\"registry.terraform.io/hashicorp/aws\"]",
                "instances": [
                    {
                        "index_key": 0,
                        "schema_version": 1,
                        "attributes": {
                            "id": "subnet-aaa",
                            "cidr_block": "10.0.1.0/24"
                        },
                        "dependencies": ["aws_vpc.main"]
                    },
                    {
                        "index_key": 1,
                        "schema_version": 1,
                        "attributes": {
                            "id": "subnet-bbb",
                            "cidr_block": "10.0.2.0/24"
                        },
                        "dependencies": ["aws_vpc.main"]
                    }
                ]
            },
            {
                "mode": "managed",
                "type": "aws_security_group",
                "name": "web",
                "provider": "provider[\"registry.terraform.io/hashicorp/aws\"]",
                "instances": [
                    {
                        "index_key": "http",
                        "schema_version": 1,
                        "attributes": {
                            "id": "sg-http",
                            "name": "web-http"
                        }
                    },
                    {
                        "index_key": "https",
                        "schema_version": 1,
                        "attributes": {
                            "id": "sg-https",
                            "name": "web-https"
                        }
                    }
                ]
            },
            {
                "mode": "data",
                "type": "aws_caller_identity",
                "name": "current",
                "provider": "provider[\"registry.terraform.io/hashicorp/aws\"]",
                "instances": [
                    {
                        "schema_version": 0,
                        "attributes": {
                            "id": "123456789012",
                            "account_id": "123456789012"
                        }
                    }
                ]
            },
            {
                "mode": "managed",
                "type": "aws_instance",
                "name": "app",
                "provider": "provider[\"registry.terraform.io/hashicorp/aws\"]",
                "module": "module.compute",
                "instances": [
                    {
                        "schema_version": 1,
                        "attributes": {
                            "id": "i-abc123",
                            "instance_type": "t3.micro"
                        },
                        "private": "eyJzY2hlbWFfdmVyc2lvbiI6IjEifQ=="
                    }
                ]
            }
        ]
    }"#;

    let state = TerraformState::from_str(state_json).expect("parse realistic state");

    assert_eq!(state.version, 4);
    assert_eq!(state.serial, 42);
    assert_eq!(state.resources.len(), 5);

    // Count instances (index_key = Int)
    let subnets = state.get_resource("aws_subnet", "private").unwrap();
    assert_eq!(subnets.instances.len(), 2);
    let subnet_0 = subnets.get_instance(&IndexKey::Int(0)).unwrap();
    assert_eq!(subnet_0.id(), Some("subnet-aaa"));
    let subnet_1 = subnets.get_instance(&IndexKey::Int(1)).unwrap();
    assert_eq!(subnet_1.id(), Some("subnet-bbb"));

    // For_each instances (index_key = String)
    let sgs = state.get_resource("aws_security_group", "web").unwrap();
    let sg_http = sgs.get_instance(&IndexKey::String("http".into())).unwrap();
    assert_eq!(sg_http.id(), Some("sg-http"));
    let sg_https = sgs.get_instance(&IndexKey::String("https".into())).unwrap();
    assert_eq!(sg_https.id(), Some("sg-https"));

    // Data source
    let caller = state
        .get_resource("aws_caller_identity", "current")
        .unwrap();
    assert_eq!(caller.mode, ResourceMode::Data);

    // Module resource
    let app = state.get_resource("aws_instance", "app").unwrap();
    assert_eq!(app.module, Some("module.compute".to_string()));
    assert_eq!(app.address(), "module.compute.aws_instance.app");
    assert!(app.primary_instance().unwrap().private.is_some());

    // Round-trip: serialize and parse again
    let serialized = state.to_string_pretty().expect("serialize");
    let reparsed = TerraformState::from_str(&serialized).expect("reparse");
    assert_eq!(state, reparsed);
}

#[test]
fn version_validation() {
    let v3_state = r#"{
        "version": 3,
        "terraform_version": "0.12.0",
        "serial": 1,
        "lineage": "test",
        "outputs": {},
        "resources": []
    }"#;

    let result = TerraformState::from_str(v3_state);
    assert!(result.is_err());
    match result.unwrap_err() {
        TfStateError::UnsupportedVersion { version } => assert_eq!(version, 3),
        other => panic!("expected UnsupportedVersion, got: {other}"),
    }

    let v5_state = r#"{
        "version": 5,
        "terraform_version": "2.0.0",
        "serial": 1,
        "lineage": "test",
        "outputs": {},
        "resources": []
    }"#;

    let result = TerraformState::from_str(v5_state);
    assert!(result.is_err());
    match result.unwrap_err() {
        TfStateError::UnsupportedVersion { version } => assert_eq!(version, 5),
        other => panic!("expected UnsupportedVersion, got: {other}"),
    }
}

#[test]
fn add_remove_bump_serial() {
    let mut state = make_state();
    assert_eq!(state.resources.len(), 2);
    assert_eq!(state.serial, 10);

    // Add a new resource
    let new_resource = make_resource(
        "aws_dynamodb_table",
        "users",
        r#"provider["registry.terraform.io/hashicorp/aws"]"#,
        vec![make_instance("users-table", json!({"name": "users"}))],
    );
    state.add_resource(new_resource).expect("add resource");
    assert_eq!(state.resources.len(), 3);
    assert!(state.get_resource("aws_dynamodb_table", "users").is_some());

    // Adding a duplicate should error
    let dup = make_resource(
        "aws_dynamodb_table",
        "users",
        r#"provider["registry.terraform.io/hashicorp/aws"]"#,
        vec![],
    );
    let result = state.add_resource(dup);
    assert!(result.is_err());
    match result.unwrap_err() {
        TfStateError::DuplicateResource {
            resource_type,
            name,
        } => {
            assert_eq!(resource_type, "aws_dynamodb_table");
            assert_eq!(name, "users");
        }
        other => panic!("expected DuplicateResource, got: {other}"),
    }

    // Remove
    let removed = state.remove_resource("aws_iam_role", "admin");
    assert!(removed.is_some());
    assert_eq!(removed.unwrap().name, "admin");
    assert_eq!(state.resources.len(), 2);
    assert!(state.get_resource("aws_iam_role", "admin").is_none());

    // Remove non-existent
    assert!(state.remove_resource("aws_iam_role", "admin").is_none());

    // Bump serial
    state.bump_serial();
    assert_eq!(state.serial, 11);
    state.bump_serial();
    assert_eq!(state.serial, 12);
}

#[test]
fn provider_ref_parsing() {
    let state = make_state();
    let bucket = state.get_resource("aws_s3_bucket", "example").unwrap();
    let pref = bucket.provider_ref().unwrap();
    assert_eq!(pref.hostname, "registry.terraform.io");
    assert_eq!(pref.namespace, "hashicorp");
    assert_eq!(pref.provider_type, "aws");
}

#[test]
fn resource_lookups() {
    let state = make_state();

    // resources_by_type
    let buckets = state.resources_by_type("aws_s3_bucket");
    assert_eq!(buckets.len(), 1);
    assert_eq!(buckets[0].name, "example");

    let roles = state.resources_by_type("aws_iam_role");
    assert_eq!(roles.len(), 1);

    let empty = state.resources_by_type("aws_lambda_function");
    assert!(empty.is_empty());

    // get_resource
    assert!(state.get_resource("aws_s3_bucket", "example").is_some());
    assert!(state.get_resource("aws_s3_bucket", "nonexistent").is_none());
    assert!(state.get_resource("nonexistent", "example").is_none());

    // iter_resources
    let count = state.iter_resources().count();
    assert_eq!(count, 2);

    // address
    let bucket = state.get_resource("aws_s3_bucket", "example").unwrap();
    assert_eq!(bucket.address(), "aws_s3_bucket.example");
}

#[test]
fn instance_attribute_operations() {
    let mut state = make_state();
    let bucket = state.get_resource_mut("aws_s3_bucket", "example").unwrap();
    let instance = bucket.primary_instance_mut().unwrap();

    // Get existing attribute
    assert_eq!(instance.get_attribute("bucket"), Some(&json!("my-bucket")));
    assert_eq!(instance.id(), Some("my-bucket"));

    // Set new attribute
    instance.set_attribute("tags", json!({"env": "prod"}));
    assert_eq!(
        instance.get_attribute("tags"),
        Some(&json!({"env": "prod"}))
    );

    // Overwrite attribute
    instance.set_attribute("bucket", json!("renamed-bucket"));
    assert_eq!(
        instance.get_attribute("bucket"),
        Some(&json!("renamed-bucket"))
    );
}

#[test]
fn from_reader_works() {
    let state = make_state();
    let json_bytes = serde_json::to_vec_pretty(&state).unwrap();
    let parsed = TerraformState::from_reader(json_bytes.as_slice()).expect("from_reader");
    assert_eq!(parsed.version, 4);
    assert_eq!(parsed.serial, 10);
    assert_eq!(parsed.resources.len(), 2);
}

#[test]
fn to_writer_pretty_works() {
    let state = make_state();
    let mut buf = Vec::new();
    state.to_writer_pretty(&mut buf).expect("to_writer_pretty");
    let parsed = TerraformState::from_reader(buf.as_slice()).expect("parse from written bytes");
    assert_eq!(parsed, state);
}

#[test]
fn default_outputs_and_resources() {
    let json = r#"{
        "version": 4,
        "serial": 0,
        "lineage": "test",
        "terraform_version": "1.0.0"
    }"#;
    let state = TerraformState::from_str(json).expect("parse minimal state");
    assert!(state.outputs.is_empty());
    assert!(state.resources.is_empty());
}
