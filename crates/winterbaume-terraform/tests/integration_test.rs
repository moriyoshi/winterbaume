use std::sync::Arc;

use serde_json::json;
use winterbaume_apigateway::ApiGatewayService;
use winterbaume_appconfig::AppConfigService;
use winterbaume_autoscaling::AutoScalingService;
use winterbaume_batch::BatchService;
use winterbaume_cloudtrail::CloudTrailService;
use winterbaume_core::{MockAws, StatefulService};
use winterbaume_databasemigration::DatabaseMigrationService;
use winterbaume_dax::DaxService;
use winterbaume_dsql::DsqlService;
use winterbaume_ebs::EbsService;
use winterbaume_ec2::Ec2Service;
use winterbaume_ec2instanceconnect::Ec2InstanceConnectService;
use winterbaume_elasticbeanstalk::ElasticBeanstalkService;
use winterbaume_fsx::FsxService;
use winterbaume_glacier::GlacierService;
use winterbaume_iam::IamService;
use winterbaume_memorydb::MemoryDbService;
use winterbaume_mq::MqService;
use winterbaume_neptune::NeptuneService;
use winterbaume_pinpoint::PinpointService;
use winterbaume_rds::RdsService;
use winterbaume_s3::S3Service;
use winterbaume_sagemaker::SageMakerService;
use winterbaume_securityhub::SecurityHubService;
use winterbaume_servicecatalog::ServiceCatalogService;
use winterbaume_sqs::SqsService;
use winterbaume_terraform::{
    ConversionContext, TerraformInjector, TerraformResourceConverter,
    converters::{
        apigateway::{AwsApiGatewayRestApiConverter, AwsApiGatewayStageConverter},
        appconfig::{
            AwsAppconfigApplicationConverter, AwsAppconfigDeploymentStrategyConverter,
            AwsAppconfigEnvironmentConverter,
        },
        autoscaling::{AwsAutoscalingGroupConverter, AwsAutoscalingPolicyConverter},
        batch::{AwsBatchComputeEnvironmentConverter, AwsBatchJobQueueConverter},
        cloudtrail::AwsCloudtrailConverter,
        dax::{AwsDaxClusterConverter, AwsDaxParameterGroupConverter, AwsDaxSubnetGroupConverter},
        dms::{AwsDmsEndpointConverter, AwsDmsReplicationInstanceConverter},
        dsql::AwsDsqlClusterConverter,
        ebs::{AwsEbsSnapshotConverter, AwsEbsVolumeConverter},
        ec2::{
            AwsInternetGatewayConverter, AwsSecurityGroupConverter, AwsSubnetConverter,
            AwsVpcConverter,
        },
        ec2instanceconnect::AwsEc2InstanceConnectEndpointConverter,
        elasticbeanstalk::AwsElasticBeanstalkEnvironmentConverter,
        fsx::AwsFsxLustreFileSystemConverter,
        glacier::AwsGlacierVaultConverter,
        iam::{
            AwsIamPolicyConverter, AwsIamRoleConverter, AwsIamRolePolicyAttachmentConverter,
            AwsIamUserConverter,
        },
        memorydb::{
            AwsMemoryDbAclConverter, AwsMemoryDbClusterConverter, AwsMemoryDbSubnetGroupConverter,
        },
        mq::{AwsMqBrokerConverter, AwsMqConfigurationConverter},
        neptune::{AwsNeptuneClusterConverter, AwsNeptuneSubnetGroupConverter},
        pinpoint::{AwsPinpointAppConverter, AwsPinpointEmailChannelConverter},
        rds::{AwsDbInstanceConverter, AwsDbSubnetGroupConverter},
        s3::AwsS3BucketConverter,
        sagemaker::{
            AwsSagemakerDomainConverter, AwsSagemakerEndpointConverter,
            AwsSagemakerNotebookInstanceConverter,
        },
        servicecatalog::AwsServicecatalogProductConverter,
        sqs::AwsSqsQueueConverter,
        timestreamwrite::{AwsTimestreamwriteDatabaseConverter, AwsTimestreamwriteTableConverter},
    },
};
use winterbaume_tfstate::{Resource, ResourceInstance, ResourceMode, TerraformState};
use winterbaume_timestreamwrite::TimestreamWriteService;

fn default_ctx() -> ConversionContext {
    ConversionContext {
        default_account_id: "123456789012".to_string(),
        default_region: "us-east-1".to_string(),
    }
}

fn make_resource(resource_type: &str, name: &str, attributes: serde_json::Value) -> Resource {
    Resource {
        mode: ResourceMode::Managed,
        resource_type: resource_type.to_string(),
        name: name.to_string(),
        provider: r#"provider["registry.terraform.io/hashicorp/aws"]"#.to_string(),
        instances: vec![ResourceInstance {
            schema_version: 0,
            attributes,
            sensitive_attributes: vec![],
            dependencies: vec![],
            index_key: None,
            private: None,
            create_before_destroy: None,
            extra: Default::default(),
        }],
        module: None,
        extra: Default::default(),
    }
}

fn make_tfstate(resources: Vec<Resource>) -> TerraformState {
    TerraformState {
        version: 4,
        serial: 1,
        lineage: "test-lineage".to_string(),
        terraform_version: "1.5.0".to_string(),
        outputs: Default::default(),
        resources,
    }
}

// ---------------------------------------------------------------------------
// S3 injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_s3_bucket() {
    let s3 = Arc::new(S3Service::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsS3BucketConverter::new(Arc::clone(&s3)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_s3_bucket",
        "my_bucket",
        json!({
            "id": "my-test-bucket",
            "bucket": "my-test-bucket",
            "region": "us-east-1",
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    // Verify the bucket exists in S3 state
    let view = s3.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.buckets.contains_key("my-test-bucket"));
    let bucket = &view.buckets["my-test-bucket"];
    assert_eq!(bucket.name, "my-test-bucket");
    assert_eq!(bucket.region, "us-east-1");
}

#[tokio::test]
async fn test_inject_s3_bucket_verify_via_sdk() {
    let s3 = Arc::new(S3Service::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsS3BucketConverter::new(Arc::clone(&s3)));

    let tfstate = make_tfstate(vec![
        make_resource(
            "aws_s3_bucket",
            "bucket_a",
            json!({
                "bucket": "alpha-bucket",
                "region": "us-east-1",
            }),
        ),
        make_resource(
            "aws_s3_bucket",
            "bucket_b",
            json!({
                "bucket": "beta-bucket",
                "region": "us-east-1",
            }),
        ),
    ]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success());
    assert_eq!(report.injected, 2);

    // Transfer injected state to a new service for SDK verification
    let s3_view = s3.snapshot(&ctx.default_account_id, "us-east-1").await;
    let s3_for_mock = S3Service::new();
    s3_for_mock
        .restore(&ctx.default_account_id, "us-east-1", s3_view)
        .await
        .unwrap();

    let mock = MockAws::builder().with_service(s3_for_mock).build();
    let config = mock.sdk_config("us-east-1").await;
    let client = aws_sdk_s3::Client::new(&config);

    let resp = client.list_buckets().send().await.unwrap();
    let bucket_names: Vec<&str> = resp
        .buckets()
        .iter()
        .map(|b| b.name().unwrap_or(""))
        .collect();
    assert!(bucket_names.contains(&"alpha-bucket"));
    assert!(bucket_names.contains(&"beta-bucket"));
}

// ---------------------------------------------------------------------------
// IAM injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_iam_role() {
    let iam = Arc::new(IamService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsIamRoleConverter::new(Arc::clone(&iam)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_iam_role",
        "test_role",
        json!({
            "name": "my-test-role",
            "path": "/",
            "arn": "arn:aws:iam::123456789012:role/my-test-role",
            "unique_id": "AROAEXAMPLEID12345",
            "assume_role_policy": r#"{"Version":"2012-10-17","Statement":[{"Effect":"Allow","Principal":{"Service":"ec2.amazonaws.com"},"Action":"sts:AssumeRole"}]}"#,
            "description": "Test role for terraform injection",
            "max_session_duration": 3600,
            "tags": {"Environment": "test"},
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    // Verify via IAM state view
    let view = iam
        .snapshot(&ctx.default_account_id, &ctx.default_region)
        .await;
    assert!(view.roles.contains_key("my-test-role"));
    let role = &view.roles["my-test-role"];
    assert_eq!(role.name, "my-test-role");
    assert_eq!(role.description, "Test role for terraform injection");
    assert_eq!(role.tags.get("Environment"), Some(&"test".to_string()));
}

#[tokio::test]
async fn test_inject_iam_user() {
    let iam = Arc::new(IamService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsIamUserConverter::new(Arc::clone(&iam)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_iam_user",
        "admin",
        json!({
            "name": "admin-user",
            "path": "/admins/",
            "arn": "arn:aws:iam::123456789012:user/admins/admin-user",
            "unique_id": "AIDAEXAMPLEID12345",
            "tags": {"Team": "platform"},
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);

    let view = iam
        .snapshot(&ctx.default_account_id, &ctx.default_region)
        .await;
    assert!(view.users.contains_key("admin-user"));
    let user = &view.users["admin-user"];
    assert_eq!(user.path, "/admins/");
    assert_eq!(user.tags.get("Team"), Some(&"platform".to_string()));
}

#[tokio::test]
async fn test_inject_iam_policy() {
    let iam = Arc::new(IamService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsIamPolicyConverter::new(Arc::clone(&iam)));

    let policy_doc = r#"{"Version":"2012-10-17","Statement":[{"Effect":"Allow","Action":"s3:*","Resource":"*"}]}"#;
    let tfstate = make_tfstate(vec![make_resource(
        "aws_iam_policy",
        "s3_full",
        json!({
            "name": "s3-full-access",
            "arn": "arn:aws:iam::123456789012:policy/s3-full-access",
            "path": "/",
            "description": "Full S3 access",
            "policy": policy_doc,
            "policy_id": "ANPAEXAMPLEID12345",
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);

    let view = iam
        .snapshot(&ctx.default_account_id, &ctx.default_region)
        .await;
    // Policy is keyed by ARN
    let policy_arn = "arn:aws:iam::123456789012:policy/s3-full-access";
    assert!(
        view.policies.contains_key(policy_arn),
        "policies: {:?}",
        view.policies.keys().collect::<Vec<_>>()
    );
    let policy = &view.policies[policy_arn];
    assert_eq!(policy.policy_name, "s3-full-access");
    assert_eq!(policy.document, policy_doc);
}

#[tokio::test]
async fn test_inject_iam_role_with_policy_attachment() {
    let iam = Arc::new(IamService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsIamRoleConverter::new(Arc::clone(&iam)));
    injector.register(AwsIamPolicyConverter::new(Arc::clone(&iam)));
    injector.register(AwsIamRolePolicyAttachmentConverter::new(Arc::clone(&iam)));

    let tfstate = make_tfstate(vec![
        make_resource(
            "aws_iam_role",
            "app_role",
            json!({
                "name": "app-role",
                "path": "/",
                "assume_role_policy": "{}",
                "description": "",
                "max_session_duration": 3600,
            }),
        ),
        make_resource(
            "aws_iam_policy",
            "app_policy",
            json!({
                "name": "app-policy",
                "arn": "arn:aws:iam::123456789012:policy/app-policy",
                "path": "/",
                "description": "",
                "policy": "{}",
            }),
        ),
        make_resource(
            "aws_iam_role_policy_attachment",
            "attach",
            json!({
                "role": "app-role",
                "policy_arn": "arn:aws:iam::123456789012:policy/app-policy",
            }),
        ),
    ]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 3);

    // Verify the role has the policy attached
    let view = iam
        .snapshot(&ctx.default_account_id, &ctx.default_region)
        .await;
    let role = &view.roles["app-role"];
    assert_eq!(role.attached_policies.len(), 1);
    assert_eq!(
        role.attached_policies[0].policy_arn,
        "arn:aws:iam::123456789012:policy/app-policy"
    );
}

// ---------------------------------------------------------------------------
// SQS injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_sqs_queue() {
    let sqs = Arc::new(SqsService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsSqsQueueConverter::new(Arc::clone(&sqs)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_sqs_queue",
        "events",
        json!({
            "name": "events-queue",
            "arn": "arn:aws:sqs:us-east-1:123456789012:events-queue",
            "url": "https://sqs.us-east-1.amazonaws.com/123456789012/events-queue",
            "visibility_timeout_seconds": 60,
            "delay_seconds": 5,
            "max_message_size": 131072,
            "message_retention_seconds": 86400,
            "receive_wait_time_seconds": 10,
            "fifo_queue": false,
            "tags": {"service": "events"},
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);

    let view = sqs.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.queues.contains_key("events-queue"));
    let queue = &view.queues["events-queue"];
    assert_eq!(queue.visibility_timeout, 60);
    assert_eq!(queue.delay_seconds, 5);
    assert_eq!(queue.maximum_message_size, 131072);
    assert!(!queue.fifo_queue);
    assert_eq!(queue.tags.get("service"), Some(&"events".to_string()));
}

// ---------------------------------------------------------------------------
// Multi-service injection test
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_multi_service_state() {
    let s3 = Arc::new(S3Service::new());
    let iam = Arc::new(IamService::new());
    let sqs = Arc::new(SqsService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsS3BucketConverter::new(Arc::clone(&s3)));
    injector.register(AwsIamRoleConverter::new(Arc::clone(&iam)));
    injector.register(AwsIamUserConverter::new(Arc::clone(&iam)));
    injector.register(AwsSqsQueueConverter::new(Arc::clone(&sqs)));

    let tfstate = make_tfstate(vec![
        make_resource(
            "aws_s3_bucket",
            "data",
            json!({
                "bucket": "data-lake",
                "region": "us-east-1",
            }),
        ),
        make_resource(
            "aws_iam_role",
            "lambda_role",
            json!({
                "name": "lambda-exec",
                "path": "/service-role/",
                "assume_role_policy": "{}",
                "description": "Lambda execution role",
                "max_session_duration": 3600,
            }),
        ),
        make_resource(
            "aws_iam_user",
            "deployer",
            json!({
                "name": "ci-deployer",
                "path": "/",
            }),
        ),
        make_resource(
            "aws_sqs_queue",
            "tasks",
            json!({
                "name": "task-queue",
                "visibility_timeout_seconds": 30,
            }),
        ),
        // Unsupported resource type - should be skipped
        make_resource(
            "aws_lambda_function",
            "handler",
            json!({
                "function_name": "my-handler",
            }),
        ),
    ]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 4);
    assert!(report.skipped.contains(&"aws_lambda_function".to_string()));

    // Verify all services have the injected resources
    let s3_view = s3.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(s3_view.buckets.contains_key("data-lake"));

    let iam_view = iam
        .snapshot(&ctx.default_account_id, &ctx.default_region)
        .await;
    assert!(iam_view.roles.contains_key("lambda-exec"));
    assert!(iam_view.users.contains_key("ci-deployer"));

    let sqs_view = sqs.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(sqs_view.queues.contains_key("task-queue"));
}

// ---------------------------------------------------------------------------
// Extraction tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_extract_s3_buckets() {
    let s3 = Arc::new(S3Service::new());
    let ctx = default_ctx();

    // Inject a bucket first
    let mut injector = TerraformInjector::new();
    injector.register(AwsS3BucketConverter::new(Arc::clone(&s3)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_s3_bucket",
        "test",
        json!({"bucket": "extract-test-bucket", "region": "us-east-1"}),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success());

    // Now extract
    let converter = AwsS3BucketConverter::new(Arc::clone(&s3));
    let extracted = converter.extract(&ctx).await.unwrap();
    assert_eq!(extracted.len(), 1);
    assert_eq!(extracted[0].name, "extract-test-bucket");
    assert_eq!(extracted[0].attributes["bucket"], "extract-test-bucket");
    assert_eq!(extracted[0].attributes["region"], "us-east-1");
}

// ---------------------------------------------------------------------------
// Data source skip test
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_data_sources_are_skipped() {
    let s3 = Arc::new(S3Service::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsS3BucketConverter::new(Arc::clone(&s3)));

    let mut tfstate = make_tfstate(vec![]);
    tfstate.resources.push(Resource {
        mode: ResourceMode::Data,
        resource_type: "aws_s3_bucket".to_string(),
        name: "existing".to_string(),
        provider: r#"provider["registry.terraform.io/hashicorp/aws"]"#.to_string(),
        instances: vec![ResourceInstance {
            schema_version: 0,
            attributes: json!({"bucket": "should-not-inject"}),
            sensitive_attributes: vec![],
            dependencies: vec![],
            index_key: None,
            private: None,
            create_before_destroy: None,
            extra: Default::default(),
        }],
        module: None,
        extra: Default::default(),
    });

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert_eq!(report.injected, 0);

    let view = s3.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.buckets.is_empty());
}

// ---------------------------------------------------------------------------
// Parse + inject end-to-end test
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_parse_and_inject_tfstate_json() {
    let tfstate_json = r#"{
        "version": 4,
        "serial": 42,
        "lineage": "abc-123",
        "terraform_version": "1.5.7",
        "outputs": {},
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
                            "id": "my-logs-bucket",
                            "bucket": "my-logs-bucket",
                            "region": "us-west-2"
                        }
                    }
                ]
            },
            {
                "mode": "managed",
                "type": "aws_sqs_queue",
                "name": "notifications",
                "provider": "provider[\"registry.terraform.io/hashicorp/aws\"]",
                "instances": [
                    {
                        "schema_version": 0,
                        "attributes": {
                            "name": "notify-queue",
                            "arn": "arn:aws:sqs:us-west-2:123456789012:notify-queue",
                            "url": "https://sqs.us-west-2.amazonaws.com/123456789012/notify-queue",
                            "visibility_timeout_seconds": 45,
                            "delay_seconds": 0,
                            "max_message_size": 262144,
                            "message_retention_seconds": 345600,
                            "receive_wait_time_seconds": 0,
                            "fifo_queue": false,
                            "tags": {}
                        }
                    }
                ]
            }
        ]
    }"#;

    let tfstate = TerraformState::from_str(tfstate_json).unwrap();
    assert_eq!(tfstate.serial, 42);
    assert_eq!(tfstate.resources.len(), 2);

    let s3 = Arc::new(S3Service::new());
    let sqs = Arc::new(SqsService::new());
    let ctx = ConversionContext {
        default_account_id: "123456789012".to_string(),
        default_region: "us-west-2".to_string(),
    };

    let mut injector = TerraformInjector::new();
    injector.register(AwsS3BucketConverter::new(Arc::clone(&s3)));
    injector.register(AwsSqsQueueConverter::new(Arc::clone(&sqs)));

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 2);

    // Verify S3 bucket was injected into the correct region
    let s3_view = s3.snapshot("123456789012", "us-west-2").await;
    assert!(s3_view.buckets.contains_key("my-logs-bucket"));
    assert_eq!(s3_view.buckets["my-logs-bucket"].region, "us-west-2");

    // Verify SQS queue
    let sqs_view = sqs.snapshot("123456789012", "us-west-2").await;
    assert!(sqs_view.queues.contains_key("notify-queue"));
    assert_eq!(sqs_view.queues["notify-queue"].visibility_timeout, 45);
}

// ---------------------------------------------------------------------------
// SNS injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_sns_topic() {
    use winterbaume_sns::SnsService;
    use winterbaume_terraform::converters::sns::AwsSnsTopicConverter;

    let sns = Arc::new(SnsService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsSnsTopicConverter::new(Arc::clone(&sns)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_sns_topic",
        "alerts",
        json!({
            "name": "alerts-topic",
            "arn": "arn:aws:sns:us-east-1:123456789012:alerts-topic",
            "tags": {"env": "prod"},
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    let view = sns.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(
        view.topics
            .contains_key("arn:aws:sns:us-east-1:123456789012:alerts-topic")
    );
    let topic = &view.topics["arn:aws:sns:us-east-1:123456789012:alerts-topic"];
    assert_eq!(topic.name, "alerts-topic");
}

// ---------------------------------------------------------------------------
// SSM injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_ssm_parameter() {
    use winterbaume_ssm::SsmService;
    use winterbaume_terraform::converters::ssm::AwsSsmParameterConverter;

    let ssm = Arc::new(SsmService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsSsmParameterConverter::new(Arc::clone(&ssm)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_ssm_parameter",
        "db_pass",
        json!({
            "name": "/app/db/password",
            "type": "SecureString",
            "value": "secret123",
            "arn": "arn:aws:ssm:us-east-1:123456789012:parameter/app/db/password",
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);

    let view = ssm.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.parameters.contains_key("/app/db/password"));
    let param = &view.parameters["/app/db/password"];
    assert_eq!(param.r#type, "SecureString");
    assert_eq!(param.value, "secret123");
}

// ---------------------------------------------------------------------------
// KMS injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_kms_key_and_alias() {
    use winterbaume_kms::KmsService;
    use winterbaume_terraform::converters::kms::{AwsKmsAliasConverter, AwsKmsKeyConverter};

    let kms = Arc::new(KmsService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsKmsKeyConverter::new(Arc::clone(&kms)));
    injector.register(AwsKmsAliasConverter::new(Arc::clone(&kms)));

    let tfstate = make_tfstate(vec![
        make_resource(
            "aws_kms_key",
            "main",
            json!({
                "key_id": "mrk-deadbeef-1234-5678-abcd-ef0123456789",
                "arn": "arn:aws:kms:us-east-1:123456789012:key/mrk-deadbeef-1234-5678-abcd-ef0123456789",
                "description": "Main encryption key",
                "enable_key_rotation": true,
            }),
        ),
        make_resource(
            "aws_kms_alias",
            "main_alias",
            json!({
                "name": "alias/main",
                "target_key_id": "mrk-deadbeef-1234-5678-abcd-ef0123456789",
                "target_key_arn": "arn:aws:kms:us-east-1:123456789012:key/mrk-deadbeef-1234-5678-abcd-ef0123456789",
                "arn": "arn:aws:kms:us-east-1:123456789012:alias/main",
            }),
        ),
    ]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 2);

    let view = kms.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(
        view.keys
            .contains_key("mrk-deadbeef-1234-5678-abcd-ef0123456789")
    );
    assert!(view.aliases.contains_key("alias/main"));
}

// ---------------------------------------------------------------------------
// Secrets Manager injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_secretsmanager_secret() {
    use winterbaume_secretsmanager::SecretsManagerService;
    use winterbaume_terraform::converters::secretsmanager::{
        AwsSecretsmanagerSecretConverter, AwsSecretsmanagerSecretVersionConverter,
    };

    let sm = Arc::new(SecretsManagerService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsSecretsmanagerSecretConverter::new(Arc::clone(&sm)));
    injector.register(AwsSecretsmanagerSecretVersionConverter::new(Arc::clone(
        &sm,
    )));

    let secret_arn = "arn:aws:secretsmanager:us-east-1:123456789012:secret:my-secret-AbCdEf";
    let tfstate = make_tfstate(vec![
        make_resource(
            "aws_secretsmanager_secret",
            "db",
            json!({
                "name": "my-secret",
                "arn": secret_arn,
                "description": "Database credentials",
            }),
        ),
        make_resource(
            "aws_secretsmanager_secret_version",
            "db_v1",
            json!({
                "id": format!("{}|AWSCURRENT", secret_arn),
                "secret_id": secret_arn,
                "secret_string": r#"{"username":"admin","password":"pass"}"#,
            }),
        ),
    ]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 2);

    let view = sm.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(
        view.secrets.contains_key("my-secret"),
        "keys: {:?}",
        view.secrets.keys().collect::<Vec<_>>()
    );
    assert_eq!(view.secrets["my-secret"].name, "my-secret");
}

// ---------------------------------------------------------------------------
// CloudWatch Logs injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_cloudwatch_log_group_and_stream() {
    use winterbaume_cloudwatchlogs::CloudWatchLogsService;
    use winterbaume_terraform::converters::logs::{
        AwsCloudwatchLogGroupConverter, AwsCloudwatchLogStreamConverter,
    };

    let logs = Arc::new(CloudWatchLogsService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsCloudwatchLogGroupConverter::new(Arc::clone(&logs)));
    injector.register(AwsCloudwatchLogStreamConverter::new(Arc::clone(&logs)));

    let tfstate = make_tfstate(vec![
        make_resource(
            "aws_cloudwatch_log_group",
            "app",
            json!({
                "name": "/app/logs",
                "retention_in_days": 30,
                "arn": "arn:aws:logs:us-east-1:123456789012:log-group:/app/logs",
                "tags": {"app": "myapp"},
            }),
        ),
        make_resource(
            "aws_cloudwatch_log_stream",
            "worker",
            json!({
                "name": "worker-stream",
                "log_group_name": "/app/logs",
                "arn": "arn:aws:logs:us-east-1:123456789012:log-group:/app/logs:log-stream:worker-stream",
            }),
        ),
    ]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 2);

    let view = logs.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.log_groups.contains_key("/app/logs"));
    assert!(
        view.log_groups["/app/logs"]
            .streams
            .contains_key("worker-stream")
    );
}

// ---------------------------------------------------------------------------
// Kinesis injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_kinesis_stream() {
    use winterbaume_kinesis::KinesisService;
    use winterbaume_terraform::converters::kinesis::AwsKinesisStreamConverter;

    let kinesis = Arc::new(KinesisService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsKinesisStreamConverter::new(Arc::clone(&kinesis)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_kinesis_stream",
        "events",
        json!({
            "name": "events-stream",
            "shard_count": 2,
            "stream_mode_details": [{"stream_mode": "PROVISIONED"}],
            "arn": "arn:aws:kinesis:us-east-1:123456789012:stream/events-stream",
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);

    let view = kinesis.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.streams.contains_key("events-stream"));
    assert_eq!(view.streams["events-stream"].shards.len(), 2);
}

// ---------------------------------------------------------------------------
// ACM injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_acm_certificate() {
    use winterbaume_acm::AcmService;
    use winterbaume_terraform::converters::acm::AwsAcmCertificateConverter;

    let acm = Arc::new(AcmService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsAcmCertificateConverter::new(Arc::clone(&acm)));

    let cert_arn =
        "arn:aws:acm:us-east-1:123456789012:certificate/abc12345-1234-1234-1234-abcdef012345";
    let tfstate = make_tfstate(vec![make_resource(
        "aws_acm_certificate",
        "tls",
        json!({
            "arn": cert_arn,
            "domain_name": "example.com",
            "validation_method": "DNS",
            "tags": {"env": "prod"},
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);

    let view = acm.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.certificates.contains_key(cert_arn));
    assert_eq!(view.certificates[cert_arn].domain_name, "example.com");
}

// ---------------------------------------------------------------------------
// Step Functions injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_sfn_state_machine() {
    use winterbaume_sfn::SfnService;
    use winterbaume_terraform::converters::stepfunctions::AwsSfnStateMachineConverter;

    let sfn = Arc::new(SfnService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsSfnStateMachineConverter::new(Arc::clone(&sfn)));

    let definition = r#"{"Comment":"A hello world example","StartAt":"HelloWorld","States":{"HelloWorld":{"Type":"Pass","End":true}}}"#;
    let tfstate = make_tfstate(vec![make_resource(
        "aws_sfn_state_machine",
        "workflow",
        json!({
            "name": "my-workflow",
            "arn": "arn:aws:states:us-east-1:123456789012:stateMachine:my-workflow",
            "role_arn": "arn:aws:iam::123456789012:role/sfn-role",
            "definition": definition,
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);

    let view = sfn.snapshot(&ctx.default_account_id, "us-east-1").await;
    let sfn_arn = "arn:aws:states:us-east-1:123456789012:stateMachine:my-workflow";
    assert!(
        view.state_machines.contains_key(sfn_arn),
        "keys: {:?}",
        view.state_machines.keys().collect::<Vec<_>>()
    );
    assert_eq!(view.state_machines[sfn_arn].name, "my-workflow");
}

// ---------------------------------------------------------------------------
// DynamoDB injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_dynamodb_table() {
    use winterbaume_dynamodb::DynamoDbService;
    use winterbaume_terraform::converters::dynamodb::AwsDynamodbTableConverter;

    let ddb = Arc::new(DynamoDbService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsDynamodbTableConverter::new(Arc::clone(&ddb)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_dynamodb_table",
        "users",
        json!({
            "name": "users",
            "billing_mode": "PAY_PER_REQUEST",
            "hash_key": "user_id",
            "arn": "arn:aws:dynamodb:us-east-1:123456789012:table/users",
            "attribute": [{"name": "user_id", "type": "S"}],
            "tags": {"app": "backend"},
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);

    let view = ddb.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.tables.contains_key("users"));
    assert_eq!(view.tables["users"].name, "users");
}

// ---------------------------------------------------------------------------
// ECR injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_ecr_repository() {
    use winterbaume_ecr::EcrService;
    use winterbaume_terraform::converters::ecr::AwsEcrRepositoryConverter;

    let ecr = Arc::new(EcrService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsEcrRepositoryConverter::new(Arc::clone(&ecr)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_ecr_repository",
        "app",
        json!({
            "name": "my-app",
            "arn": "arn:aws:ecr:us-east-1:123456789012:repository/my-app",
            "repository_url": "123456789012.dkr.ecr.us-east-1.amazonaws.com/my-app",
            "image_tag_mutability": "MUTABLE",
            "tags": {"service": "app"},
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);

    let view = ecr.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.repositories.contains_key("my-app"));
    assert_eq!(view.repositories["my-app"].repository_name, "my-app");
}

// ---------------------------------------------------------------------------
// EFS injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_efs_file_system() {
    use winterbaume_efs::EfsService;
    use winterbaume_terraform::converters::efs::AwsEfsFileSystemConverter;

    let efs = Arc::new(EfsService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsEfsFileSystemConverter::new(Arc::clone(&efs)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_efs_file_system",
        "shared",
        json!({
            "id": "fs-12345678",
            "arn": "arn:aws:elasticfilesystem:us-east-1:123456789012:file-system/fs-12345678",
            "performance_mode": "generalPurpose",
            "throughput_mode": "bursting",
            "encrypted": true,
            "tags": {"Name": "shared-fs"},
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);

    let view = efs.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.file_systems.contains_key("fs-12345678"));
    assert!(view.file_systems["fs-12345678"].encrypted);
}

// ---------------------------------------------------------------------------
// EventBridge injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_eventbridge_rule_and_target() {
    use winterbaume_eventbridge::EventBridgeService;
    use winterbaume_terraform::converters::events::{
        AwsCloudwatchEventRuleConverter, AwsCloudwatchEventTargetConverter,
    };

    let events = Arc::new(EventBridgeService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsCloudwatchEventRuleConverter::new(Arc::clone(&events)));
    injector.register(AwsCloudwatchEventTargetConverter::new(Arc::clone(&events)));

    let tfstate = make_tfstate(vec![
        make_resource(
            "aws_cloudwatch_event_rule",
            "schedule",
            json!({
                "name": "daily-job",
                "arn": "arn:aws:events:us-east-1:123456789012:rule/daily-job",
                "schedule_expression": "rate(1 day)",
                "state": "ENABLED",
                "event_bus_name": "default",
            }),
        ),
        make_resource(
            "aws_cloudwatch_event_target",
            "lambda_target",
            json!({
                "rule": "daily-job",
                "target_id": "MyLambdaTarget",
                "arn": "arn:aws:lambda:us-east-1:123456789012:function:my-fn",
                "event_bus_name": "default",
            }),
        ),
    ]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 2);

    let view = events.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.rules.contains_key("daily-job"));
    assert_eq!(view.rules["daily-job"].targets.len(), 1);
    assert_eq!(view.rules["daily-job"].targets[0].id, "MyLambdaTarget");
}

// ---------------------------------------------------------------------------
// Route53 injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_route53_zone_and_record() {
    use winterbaume_route53::Route53Service;
    use winterbaume_terraform::converters::route53::{
        AwsRoute53RecordConverter, AwsRoute53ZoneConverter,
    };

    let r53 = Arc::new(Route53Service::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsRoute53ZoneConverter::new(Arc::clone(&r53)));
    injector.register(AwsRoute53RecordConverter::new(Arc::clone(&r53)));

    let zone_id = "Z1234567890ABCDEF";
    let tfstate = make_tfstate(vec![
        make_resource(
            "aws_route53_zone",
            "main",
            json!({
                "zone_id": zone_id,
                "name": "example.com",
                "comment": "Managed by Terraform",
                "tags": {"env": "prod"},
            }),
        ),
        make_resource(
            "aws_route53_record",
            "www",
            json!({
                "zone_id": zone_id,
                "name": "www.example.com",
                "type": "A",
                "ttl": 300,
                "records": ["1.2.3.4"],
            }),
        ),
    ]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 2);

    let view = r53.snapshot(&ctx.default_account_id, "us-east-1").await;
    let zone_key = format!("/hostedzone/{}", zone_id);
    assert!(view.hosted_zones.contains_key(&zone_key));
    assert_eq!(view.hosted_zones[&zone_key].records.len(), 1);
    assert_eq!(
        view.hosted_zones[&zone_key].records[0].name,
        "www.example.com"
    );
}

// ---------------------------------------------------------------------------
// Cognito IDP injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_cognito_user_pool_and_client() {
    use winterbaume_cognitoidentityprovider::CognitoIdentityProviderService;
    use winterbaume_terraform::converters::cognitoidp::{
        AwsCognitoUserPoolClientConverter, AwsCognitoUserPoolConverter,
    };

    let cognito = Arc::new(CognitoIdentityProviderService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsCognitoUserPoolConverter::new(Arc::clone(&cognito)));
    injector.register(AwsCognitoUserPoolClientConverter::new(Arc::clone(&cognito)));

    let pool_id = "us-east-1_abc123";
    let tfstate = make_tfstate(vec![
        make_resource(
            "aws_cognito_user_pool",
            "main",
            json!({
                "id": pool_id,
                "name": "my-user-pool",
                "arn": "arn:aws:cognito-idp:us-east-1:123456789012:userpool/us-east-1_abc123",
                "tags": {"app": "web"},
            }),
        ),
        make_resource(
            "aws_cognito_user_pool_client",
            "web",
            json!({
                "id": "clientid123abc",
                "name": "web-client",
                "user_pool_id": pool_id,
            }),
        ),
    ]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 2);

    let view = cognito.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.user_pools.contains_key(pool_id));
    assert!(
        view.user_pools[pool_id]
            .clients
            .contains_key("clientid123abc")
    );
}

// ---------------------------------------------------------------------------
// CloudWatch metric alarm injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_cloudwatch_metric_alarm() {
    use winterbaume_cloudwatch::CloudWatchService;
    use winterbaume_terraform::converters::cloudwatch::AwsCloudwatchMetricAlarmConverter;

    let cw = Arc::new(CloudWatchService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsCloudwatchMetricAlarmConverter::new(Arc::clone(&cw)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_cloudwatch_metric_alarm",
        "cpu_high",
        json!({
            "alarm_name": "high-cpu",
            "arn": "arn:aws:cloudwatch:us-east-1:123456789012:alarm:high-cpu",
            "metric_name": "CPUUtilization",
            "namespace": "AWS/EC2",
            "threshold": 80.0,
            "comparison_operator": "GreaterThanThreshold",
            "evaluation_periods": 2,
            "period": 300,
            "statistic": "Average",
            "actions_enabled": true,
            "dimensions": {"InstanceId": "i-0123456789abcdef0"},
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);

    let view = cw.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.alarms.contains_key("high-cpu"));
    let alarm = &view.alarms["high-cpu"];
    assert_eq!(alarm.threshold, 80.0);
    assert_eq!(alarm.metric_name, "CPUUtilization");
    assert_eq!(alarm.dimensions.len(), 1);
    assert_eq!(alarm.dimensions[0].name, "InstanceId");
}

// ---------------------------------------------------------------------------
// Lambda injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_lambda_function_and_alias() {
    use winterbaume_lambda::LambdaService;
    use winterbaume_terraform::converters::lambda::{
        AwsLambdaAliasConverter, AwsLambdaFunctionConverter,
    };

    let lambda = Arc::new(LambdaService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsLambdaFunctionConverter::new(Arc::clone(&lambda)));
    injector.register(AwsLambdaAliasConverter::new(Arc::clone(&lambda)));

    let tfstate = make_tfstate(vec![
        make_resource(
            "aws_lambda_function",
            "handler",
            json!({
                "function_name": "my-handler",
                "arn": "arn:aws:lambda:us-east-1:123456789012:function:my-handler",
                "runtime": "python3.11",
                "handler": "index.handler",
                "role": "arn:aws:iam::123456789012:role/lambda-role",
                "memory_size": 256,
                "timeout": 30,
                "description": "My Lambda function",
                "tags": {"team": "backend"},
            }),
        ),
        make_resource(
            "aws_lambda_alias",
            "live",
            json!({
                "name": "live",
                "function_name": "my-handler",
                "function_version": "$LATEST",
                "description": "Live alias",
                "arn": "arn:aws:lambda:us-east-1:123456789012:function:my-handler:live",
            }),
        ),
    ]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 2);

    let view = lambda.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.functions.contains_key("my-handler"));
    assert_eq!(view.functions["my-handler"].runtime, "python3.11");
    assert_eq!(view.functions["my-handler"].memory_size, 256);
    assert!(view.aliases.contains_key("my-handler:live"));
}

#[tokio::test]
async fn test_inject_lambda_permission() {
    use winterbaume_lambda::LambdaService;
    use winterbaume_terraform::converters::lambda::{
        AwsLambdaFunctionConverter, AwsLambdaPermissionConverter,
    };

    let lambda = Arc::new(LambdaService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsLambdaFunctionConverter::new(Arc::clone(&lambda)));
    injector.register(AwsLambdaPermissionConverter::new(Arc::clone(&lambda)));

    let tfstate = make_tfstate(vec![
        make_resource(
            "aws_lambda_function",
            "fn",
            json!({
                "function_name": "my-fn",
                "runtime": "python3.11",
                "handler": "index.handler",
                "role": "arn:aws:iam::123456789012:role/lambda-role",
            }),
        ),
        make_resource(
            "aws_lambda_permission",
            "allow_events",
            json!({
                "statement_id": "AllowExecutionFromEvents",
                "action": "lambda:InvokeFunction",
                "function_name": "my-fn",
                "principal": "events.amazonaws.com",
                "source_arn": "arn:aws:events:us-east-1:123456789012:rule/my-rule",
            }),
        ),
    ]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);

    let view = lambda.snapshot(&ctx.default_account_id, "us-east-1").await;
    let perms = view.function_permissions.get("my-fn").unwrap();
    assert_eq!(perms.len(), 1);
    assert_eq!(perms[0].statement_id, "AllowExecutionFromEvents");
    assert_eq!(perms[0].principal, "events.amazonaws.com");
    // The converter must bump function_policy_revisions in lockstep with
    // function_permissions so that GetPolicy returns a non-empty RevisionId
    // (regression for the AddPermission/GetPolicy revision-id fix).
    let revision = view
        .function_policy_revisions
        .get("my-fn")
        .expect("revision id missing for injected permission");
    assert!(!revision.is_empty(), "revision id should be non-empty");
}

#[tokio::test]
async fn test_inject_lambda_permission_bumps_revision_between_statements() {
    use winterbaume_lambda::LambdaService;
    use winterbaume_terraform::converters::lambda::{
        AwsLambdaFunctionConverter, AwsLambdaPermissionConverter,
    };

    let lambda = Arc::new(LambdaService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsLambdaFunctionConverter::new(Arc::clone(&lambda)));
    injector.register(AwsLambdaPermissionConverter::new(Arc::clone(&lambda)));

    let tfstate = make_tfstate(vec![
        make_resource(
            "aws_lambda_function",
            "fn",
            json!({
                "function_name": "multi-perm-fn",
                "runtime": "python3.11",
                "handler": "index.handler",
                "role": "arn:aws:iam::123456789012:role/lambda-role",
            }),
        ),
        make_resource(
            "aws_lambda_permission",
            "allow_events",
            json!({
                "statement_id": "AllowEvents",
                "action": "lambda:InvokeFunction",
                "function_name": "multi-perm-fn",
                "principal": "events.amazonaws.com",
                "source_arn": "arn:aws:events:us-east-1:123456789012:rule/my-rule",
            }),
        ),
        make_resource(
            "aws_lambda_permission",
            "allow_sns",
            json!({
                "statement_id": "AllowSns",
                "action": "lambda:InvokeFunction",
                "function_name": "multi-perm-fn",
                "principal": "sns.amazonaws.com",
                "source_arn": "arn:aws:sns:us-east-1:123456789012:my-topic",
            }),
        ),
    ]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);

    let view = lambda.snapshot(&ctx.default_account_id, "us-east-1").await;
    let perms = view.function_permissions.get("multi-perm-fn").unwrap();
    assert_eq!(perms.len(), 2);
    let revision = view
        .function_policy_revisions
        .get("multi-perm-fn")
        .expect("revision id missing");
    assert!(!revision.is_empty(), "revision id should be non-empty");
}

// ---------------------------------------------------------------------------
// ECS injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_ecs_cluster_and_service() {
    use winterbaume_ecs::EcsService;
    use winterbaume_terraform::converters::ecs::{
        AwsEcsClusterConverter, AwsEcsServiceConverter, AwsEcsTaskDefinitionConverter,
    };

    let ecs = Arc::new(EcsService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsEcsClusterConverter::new(Arc::clone(&ecs)));
    injector.register(AwsEcsTaskDefinitionConverter::new(Arc::clone(&ecs)));
    injector.register(AwsEcsServiceConverter::new(Arc::clone(&ecs)));

    let cluster_arn = "arn:aws:ecs:us-east-1:123456789012:cluster/my-cluster";
    let td_arn = "arn:aws:ecs:us-east-1:123456789012:task-definition/my-task:1";
    let container_defs =
        r#"[{"name":"app","image":"nginx:latest","cpu":256,"memory":512,"essential":true}]"#;

    let tfstate = make_tfstate(vec![
        make_resource(
            "aws_ecs_cluster",
            "main",
            json!({
                "name": "my-cluster",
                "arn": cluster_arn,
                "tags": {"env": "prod"},
            }),
        ),
        make_resource(
            "aws_ecs_task_definition",
            "app",
            json!({
                "family": "my-task",
                "revision": 1,
                "arn": td_arn,
                "container_definitions": container_defs,
                "network_mode": "awsvpc",
                "requires_compatibilities": ["FARGATE"],
                "cpu": "256",
                "memory": "512",
                "execution_role_arn": "arn:aws:iam::123456789012:role/ecsTaskExecutionRole",
            }),
        ),
        make_resource(
            "aws_ecs_service",
            "app_svc",
            json!({
                "name": "app-service",
                "id": "arn:aws:ecs:us-east-1:123456789012:service/my-cluster/app-service",
                "cluster": cluster_arn,
                "task_definition": td_arn,
                "desired_count": 2,
                "launch_type": "FARGATE",
                "scheduling_strategy": "REPLICA",
                "tags": {"env": "prod"},
            }),
        ),
    ]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 3);

    let view = ecs.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.clusters.contains_key("my-cluster"));
    assert!(view.task_definitions.contains_key(td_arn));
    assert_eq!(view.services.len(), 1);
    let svc = view.services.values().next().unwrap();
    assert_eq!(svc.name, "app-service");
    assert_eq!(svc.desired_count, 2);
}

// ---------------------------------------------------------------------------
// ELBv2 injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_elbv2_lb_target_group_listener() {
    use winterbaume_elasticloadbalancingv2::ElasticLoadBalancingV2Service;
    use winterbaume_terraform::converters::elbv2::{
        AwsLbConverter, AwsLbListenerConverter, AwsLbTargetGroupConverter,
    };

    let elbv2 = Arc::new(ElasticLoadBalancingV2Service::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsLbConverter::new(Arc::clone(&elbv2)));
    injector.register(AwsLbTargetGroupConverter::new(Arc::clone(&elbv2)));
    injector.register(AwsLbListenerConverter::new(Arc::clone(&elbv2)));

    let lb_arn =
        "arn:aws:elasticloadbalancing:us-east-1:123456789012:loadbalancer/app/my-alb/abc123";
    let tg_arn = "arn:aws:elasticloadbalancing:us-east-1:123456789012:targetgroup/my-tg/xyz789";

    let tfstate = make_tfstate(vec![
        make_resource(
            "aws_lb",
            "main",
            json!({
                "name": "my-alb",
                "arn": lb_arn,
                "dns_name": "my-alb-123456789.us-east-1.elb.amazonaws.com",
                "load_balancer_type": "application",
                "internal": false,
                "vpc_id": "vpc-12345",
                "subnets": ["subnet-111", "subnet-222"],
                "security_groups": ["sg-abc123"],
                "ip_address_type": "ipv4",
                "tags": {"env": "prod"},
            }),
        ),
        make_resource(
            "aws_lb_target_group",
            "app",
            json!({
                "name": "my-tg",
                "arn": tg_arn,
                "protocol": "HTTP",
                "port": 8080,
                "vpc_id": "vpc-12345",
                "target_type": "ip",
            }),
        ),
        make_resource(
            "aws_lb_listener",
            "http",
            json!({
                "load_balancer_arn": lb_arn,
                "port": 80,
                "protocol": "HTTP",
                "default_action": [{"type": "forward", "target_group_arn": tg_arn}],
            }),
        ),
    ]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 3);

    let view = elbv2.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.load_balancers.contains_key(lb_arn));
    assert_eq!(view.load_balancers[lb_arn].name, "my-alb");
    assert!(view.target_groups.contains_key(tg_arn));
    assert_eq!(view.listeners.len(), 1);
}

// ---------------------------------------------------------------------------
// CloudFront injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_cloudfront_distribution() {
    use winterbaume_cloudfront::CloudFrontService;
    use winterbaume_terraform::converters::cloudfront::AwsCloudfrontDistributionConverter;

    let cf = Arc::new(CloudFrontService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsCloudfrontDistributionConverter::new(Arc::clone(&cf)));

    let dist_id = "E1EXAMPLE12345678";
    let tfstate = make_tfstate(vec![make_resource(
        "aws_cloudfront_distribution",
        "cdn",
        json!({
            "id": dist_id,
            "arn": format!("arn:aws:cloudfront::123456789012:distribution/{}", dist_id),
            "domain_name": format!("{}.cloudfront.net", dist_id.to_lowercase()),
            "enabled": true,
            "origin": [{
                "origin_id": "myS3Origin",
                "domain_name": "mybucket.s3.amazonaws.com",
            }],
            "default_cache_behavior": [{
                "target_origin_id": "myS3Origin",
                "viewer_protocol_policy": "redirect-to-https",
            }],
            "tags": {"project": "cdn"},
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);

    let view = cf.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.distributions.contains_key(dist_id));
    assert_eq!(view.distributions[dist_id].id, dist_id);
    assert!(view.distributions[dist_id].enabled);
    assert_eq!(view.distributions[dist_id].origins.len(), 1);
    assert_eq!(view.distributions[dist_id].origins[0].id, "myS3Origin");
}

// ---------------------------------------------------------------------------
// EKS injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_eks_cluster() {
    use winterbaume_eks::EksService;
    use winterbaume_terraform::converters::eks::AwsEksClusterConverter;
    let svc = Arc::new(EksService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsEksClusterConverter::new(Arc::clone(&svc)));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_eks_cluster",
        "my_cluster",
        json!({
            "name": "my-cluster",
            "role_arn": "arn:aws:iam::123456789012:role/eks-role",
            "version": "1.29",
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.clusters.contains_key("my-cluster"));
    assert_eq!(view.clusters["my-cluster"].version, "1.29");
}

// ---------------------------------------------------------------------------
// CodeCommit injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_codecommit_repository() {
    use winterbaume_codecommit::CodeCommitService;
    use winterbaume_terraform::converters::codecommit::AwsCodecommitRepositoryConverter;
    let svc = Arc::new(CodeCommitService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsCodecommitRepositoryConverter::new(Arc::clone(&svc)));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_codecommit_repository",
        "my_repo",
        json!({
            "repository_name": "my-repo",
            "description": "Test repository",
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.repositories.contains_key("my-repo"));
    assert_eq!(view.repositories["my-repo"].description, "Test repository");
}

// ---------------------------------------------------------------------------
// Firehose injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_firehose_stream() {
    use winterbaume_firehose::FirehoseService;
    use winterbaume_terraform::converters::firehose::AwsKinesisFirehoseDeliveryStreamConverter;
    let svc = Arc::new(FirehoseService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsKinesisFirehoseDeliveryStreamConverter::new(Arc::clone(
        &svc,
    )));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_kinesis_firehose_delivery_stream",
        "my_stream",
        json!({
            "name": "my-firehose-stream",
            "destination": "s3",
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.streams.contains_key("my-firehose-stream"));
    assert_eq!(view.streams["my-firehose-stream"].destination_type, "s3");
}

// ---------------------------------------------------------------------------
// OpenSearch injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_opensearch_domain() {
    use winterbaume_opensearch::OpenSearchService;
    use winterbaume_terraform::converters::opensearch::AwsOpensearchDomainConverter;
    let svc = Arc::new(OpenSearchService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsOpensearchDomainConverter::new(Arc::clone(&svc)));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_opensearch_domain",
        "my_domain",
        json!({
            "domain_name": "my-domain",
            "engine_version": "OpenSearch_2.11",
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.domains.contains_key("my-domain"));
    assert_eq!(view.domains["my-domain"].engine_version, "OpenSearch_2.11");
}

// ---------------------------------------------------------------------------
// Glue injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_glue_catalog_database() {
    use winterbaume_glue::GlueService;
    use winterbaume_terraform::converters::glue::AwsGlueCatalogDatabaseConverter;
    let svc = Arc::new(GlueService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsGlueCatalogDatabaseConverter::new(Arc::clone(&svc)));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_glue_catalog_database",
        "my_db",
        json!({
            "name": "my-database",
            "description": "My Glue database",
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.databases.contains_key("my-database"));
}

#[tokio::test]
async fn test_inject_glue_job() {
    use winterbaume_glue::GlueService;
    use winterbaume_terraform::converters::glue::AwsGlueJobConverter;
    let svc = Arc::new(GlueService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsGlueJobConverter::new(Arc::clone(&svc)));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_glue_job",
        "my_job",
        json!({
            "name": "my-glue-job",
            "role_arn": "arn:aws:iam::123456789012:role/glue-role",
            "glue_version": "4.0",
            "worker_type": "G.1X",
            "number_of_workers": 2,
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.jobs.contains_key("my-glue-job"));
    assert_eq!(
        view.jobs["my-glue-job"].glue_version.as_deref(),
        Some("4.0")
    );
}

// ---------------------------------------------------------------------------
// CodeBuild injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_codebuild_project() {
    use winterbaume_codebuild::CodeBuildService;
    use winterbaume_terraform::converters::codebuild::AwsCodebuildProjectConverter;
    let svc = Arc::new(CodeBuildService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsCodebuildProjectConverter::new(Arc::clone(&svc)));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_codebuild_project",
        "my_project",
        json!({
            "name": "my-project",
            "service_role": "arn:aws:iam::123456789012:role/codebuild-role",
            "source": [{"type": "GITHUB", "location": "https://github.com/example/repo"}],
            "artifacts": [{"type": "NO_ARTIFACTS"}],
            "environment": [{"type": "LINUX_CONTAINER", "image": "aws/codebuild/standard:7.0", "compute_type": "BUILD_GENERAL1_SMALL"}],
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.projects.contains_key("my-project"));
    assert_eq!(view.projects["my-project"].source_type, "GITHUB");
}

// ---------------------------------------------------------------------------
// Backup injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_backup_vault() {
    use winterbaume_backup::BackupService;
    use winterbaume_terraform::converters::backup::AwsBackupVaultConverter;
    let svc = Arc::new(BackupService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsBackupVaultConverter::new(Arc::clone(&svc)));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_backup_vault",
        "my_vault",
        json!({
            "name": "my-vault",
            "tags": {"Environment": "prod"},
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.vaults.contains_key("my-vault"));
    assert_eq!(view.vaults["my-vault"].tags["Environment"], "prod");
}

#[tokio::test]
async fn test_inject_backup_plan() {
    use winterbaume_backup::BackupService;
    use winterbaume_terraform::converters::backup::AwsBackupPlanConverter;
    let svc = Arc::new(BackupService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsBackupPlanConverter::new(Arc::clone(&svc)));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_backup_plan",
        "my_plan",
        json!({
            "name": "my-backup-plan",
            "rule": [{"rule_name": "daily", "target_vault_name": "my-vault", "schedule": "cron(0 12 * * ? *)"}],
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert_eq!(view.backup_plans.len(), 1);
    let plan = view.backup_plans.values().next().unwrap();
    assert_eq!(plan.backup_plan_name, "my-backup-plan");
}

// ---------------------------------------------------------------------------
// Athena injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_athena_workgroup() {
    use winterbaume_athena::AthenaService;
    use winterbaume_terraform::converters::athena::AwsAthenaWorkgroupConverter;
    let svc = Arc::new(AthenaService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsAthenaWorkgroupConverter::new(Arc::clone(&svc)));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_athena_workgroup",
        "my_wg",
        json!({
            "name": "my-workgroup",
            "description": "My Athena workgroup",
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.work_groups.contains_key("my-workgroup"));
    assert_eq!(
        view.work_groups["my-workgroup"].description,
        "My Athena workgroup"
    );
}

// ---------------------------------------------------------------------------
// CodePipeline injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_codepipeline() {
    use winterbaume_codepipeline::CodePipelineService;
    use winterbaume_terraform::converters::codepipeline::AwsCodepipelinePipelineConverter;
    let svc = Arc::new(CodePipelineService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsCodepipelinePipelineConverter::new(Arc::clone(&svc)));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_codepipeline",
        "my_pipeline",
        json!({
            "name": "my-pipeline",
            "role_arn": "arn:aws:iam::123456789012:role/pipeline-role",
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.pipelines.contains_key("my-pipeline"));
    assert_eq!(
        view.pipelines["my-pipeline"].role_arn,
        "arn:aws:iam::123456789012:role/pipeline-role"
    );
}

// ---------------------------------------------------------------------------
// CodeDeploy injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_codedeploy_app() {
    use winterbaume_codedeploy::CodeDeployService;
    use winterbaume_terraform::converters::codedeploy::AwsCodedeployAppConverter;
    let svc = Arc::new(CodeDeployService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsCodedeployAppConverter::new(Arc::clone(&svc)));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_codedeploy_app",
        "my_app",
        json!({
            "name": "my-app",
            "compute_platform": "ECS",
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.applications.contains_key("my-app"));
    assert_eq!(view.applications["my-app"].compute_platform, "ECS");
}

#[tokio::test]
async fn test_inject_codedeploy_deployment_group() {
    use winterbaume_codedeploy::CodeDeployService;
    use winterbaume_terraform::converters::codedeploy::{
        AwsCodedeployAppConverter, AwsCodedeployDeploymentGroupConverter,
    };
    let svc = Arc::new(CodeDeployService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsCodedeployAppConverter::new(Arc::clone(&svc)));
    injector.register(AwsCodedeployDeploymentGroupConverter::new(Arc::clone(&svc)));
    let tfstate = make_tfstate(vec![
        make_resource(
            "aws_codedeploy_app",
            "my_app",
            json!({"name": "my-app", "compute_platform": "Server"}),
        ),
        make_resource(
            "aws_codedeploy_deployment_group",
            "my_dg",
            json!({
                "app_name": "my-app",
                "deployment_group_name": "my-dg",
                "service_role_arn": "arn:aws:iam::123456789012:role/dg-role",
            }),
        ),
    ]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert_eq!(view.deployment_groups.len(), 1);
    assert_eq!(view.deployment_groups[0].deployment_group_name, "my-dg");
}

// ---------------------------------------------------------------------------
// WAFv2 injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_wafv2_web_acl() {
    use winterbaume_terraform::converters::wafv2::AwsWafv2WebAclConverter;
    use winterbaume_wafv2::WafV2Service;
    let svc = Arc::new(WafV2Service::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsWafv2WebAclConverter::new(Arc::clone(&svc)));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_wafv2_web_acl",
        "my_acl",
        json!({
            "name": "my-web-acl",
            "scope": "REGIONAL",
            "description": "My WAF ACL",
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.web_acls.contains_key("REGIONAL:my-web-acl"));
    assert_eq!(
        view.web_acls["REGIONAL:my-web-acl"].description,
        "My WAF ACL"
    );
}

#[tokio::test]
async fn test_inject_wafv2_ip_set() {
    use winterbaume_terraform::converters::wafv2::AwsWafv2IpSetConverter;
    use winterbaume_wafv2::WafV2Service;
    let svc = Arc::new(WafV2Service::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsWafv2IpSetConverter::new(Arc::clone(&svc)));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_wafv2_ip_set",
        "my_ipset",
        json!({
            "name": "my-ip-set",
            "scope": "REGIONAL",
            "ip_address_version": "IPV4",
            "addresses": ["1.2.3.4/32", "5.6.7.8/32"],
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.ip_sets.contains_key("REGIONAL:my-ip-set"));
    assert_eq!(view.ip_sets["REGIONAL:my-ip-set"].addresses.len(), 2);
}

// ---------------------------------------------------------------------------
// Scheduler injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_scheduler_schedule_group() {
    use winterbaume_scheduler::SchedulerService;
    use winterbaume_terraform::converters::scheduler::AwsSchedulerScheduleGroupConverter;
    let svc = Arc::new(SchedulerService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsSchedulerScheduleGroupConverter::new(Arc::clone(&svc)));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_scheduler_schedule_group",
        "my_group",
        json!({
            "name": "my-group",
            "tags": {"Env": "test"},
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.groups.contains_key("my-group"));
}

#[tokio::test]
async fn test_inject_scheduler_schedule() {
    use winterbaume_scheduler::SchedulerService;
    use winterbaume_terraform::converters::scheduler::{
        AwsSchedulerScheduleConverter, AwsSchedulerScheduleGroupConverter,
    };
    let svc = Arc::new(SchedulerService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsSchedulerScheduleGroupConverter::new(Arc::clone(&svc)));
    injector.register(AwsSchedulerScheduleConverter::new(Arc::clone(&svc)));
    let tfstate = make_tfstate(vec![
        make_resource(
            "aws_scheduler_schedule_group",
            "my_group",
            json!({"name": "my-group"}),
        ),
        make_resource(
            "aws_scheduler_schedule",
            "my_schedule",
            json!({
                "name": "my-schedule",
                "group_name": "my-group",
                "schedule_expression": "rate(1 hour)",
                "flexible_time_window": [{"mode": "OFF"}],
                "target": [{"arn": "arn:aws:lambda:us-east-1:123456789012:function:my-fn", "role_arn": "arn:aws:iam::123456789012:role/scheduler-role"}],
            }),
        ),
    ]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    let key = "my-group\x00my-schedule";
    assert!(view.schedules.contains_key(key));
    assert_eq!(view.schedules[key].schedule_expression, "rate(1 hour)");
}

// ---------------------------------------------------------------------------
// SES injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_ses_email_identity() {
    use winterbaume_sesv2::SesV2Service;
    use winterbaume_terraform::converters::ses::AwsSesv2EmailIdentityConverter;
    let svc = Arc::new(SesV2Service::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsSesv2EmailIdentityConverter::new(Arc::clone(&svc)));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_sesv2_email_identity",
        "my_identity",
        json!({"email_identity": "example.com"}),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.identities.contains_key("example.com"));
    assert_eq!(view.identities["example.com"].identity_type, "DOMAIN");
}

// ---------------------------------------------------------------------------
// AppSync injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_appsync_graphql_api() {
    use winterbaume_appsync::AppSyncService;
    use winterbaume_terraform::converters::appsync::AwsAppsyncGraphqlApiConverter;
    let svc = Arc::new(AppSyncService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsAppsyncGraphqlApiConverter::new(Arc::clone(&svc)));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_appsync_graphql_api",
        "my_api",
        json!({
            "name": "my-api",
            "authentication_type": "API_KEY",
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert_eq!(view.apis.len(), 1);
    let api = view.apis.values().next().unwrap();
    assert_eq!(api.name, "my-api");
    assert_eq!(api.authentication_type, "API_KEY");
}

// ---------------------------------------------------------------------------
// Kafka (MSK) injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_msk_cluster() {
    use winterbaume_kafka::KafkaService;
    use winterbaume_terraform::converters::kafka::AwsMskClusterConverter;
    let svc = Arc::new(KafkaService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsMskClusterConverter::new(Arc::clone(&svc)));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_msk_cluster",
        "my_cluster",
        json!({
            "cluster_name": "my-kafka-cluster",
            "kafka_version": "3.5.1",
            "number_of_broker_nodes": 3,
            "broker_node_group_info": [{"instance_type": "kafka.m5.large", "client_subnets": ["subnet-1", "subnet-2", "subnet-3"], "security_groups": ["sg-1"]}],
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert_eq!(view.clusters.len(), 1);
    let cluster = view.clusters.values().next().unwrap();
    assert_eq!(cluster.cluster_name, "my-kafka-cluster");
    assert!(cluster.provisioned.is_some());
}

// ---------------------------------------------------------------------------
// Amplify injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_amplify_app() {
    use winterbaume_amplify::AmplifyService;
    use winterbaume_terraform::converters::amplify::AwsAmplifyAppConverter;
    let svc = Arc::new(AmplifyService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsAmplifyAppConverter::new(Arc::clone(&svc)));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_amplify_app",
        "my_app",
        json!({
            "name": "my-amplify-app",
            "description": "Test app",
            "platform": "WEB",
            "repository": "https://github.com/example/repo",
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert_eq!(view.apps.len(), 1);
    let app = view.apps.values().next().unwrap();
    assert_eq!(app.name, "my-amplify-app");
    assert_eq!(app.platform.as_deref(), Some("WEB"));
}

#[tokio::test]
async fn test_inject_amplify_branch() {
    use winterbaume_amplify::AmplifyService;
    use winterbaume_terraform::converters::amplify::{
        AwsAmplifyAppConverter, AwsAmplifyBranchConverter,
    };
    let svc = Arc::new(AmplifyService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsAmplifyAppConverter::new(Arc::clone(&svc)));
    injector.register(AwsAmplifyBranchConverter::new(Arc::clone(&svc)));
    let tfstate = make_tfstate(vec![
        make_resource(
            "aws_amplify_app",
            "my_app",
            json!({
                "app_id": "abc123",
                "name": "my-amplify-app",
            }),
        ),
        make_resource(
            "aws_amplify_branch",
            "main_branch",
            json!({
                "app_id": "abc123",
                "branch_name": "main",
                "stage": "PRODUCTION",
                "framework": "React",
            }),
        ),
    ]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 2);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert_eq!(view.branches.len(), 1);
    let branch = view.branches.values().next().unwrap();
    assert_eq!(branch.branch_name, "main");
    assert_eq!(branch.framework.as_deref(), Some("React"));
}

// ---------------------------------------------------------------------------
// API Gateway V2 injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_apigatewayv2_api() {
    use winterbaume_apigatewayv2::ApiGatewayV2Service;
    use winterbaume_terraform::converters::apigatewayv2::AwsApigatewayv2ApiConverter;
    let svc = Arc::new(ApiGatewayV2Service::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsApigatewayv2ApiConverter::new(Arc::clone(&svc)));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_apigatewayv2_api",
        "my_api",
        json!({
            "name": "my-http-api",
            "protocol_type": "HTTP",
            "description": "My API Gateway v2 HTTP API",
            "tags": {"env": "dev"},
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert_eq!(view.apis.len(), 1);
    let api = view.apis.values().next().unwrap();
    assert_eq!(api.name, "my-http-api");
    assert_eq!(api.protocol_type, "HTTP");
}

// ---------------------------------------------------------------------------
// App Runner injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_apprunner_service() {
    use winterbaume_apprunner::AppRunnerService;
    use winterbaume_terraform::converters::apprunner::AwsAppRunnerServiceConverter;
    let svc = Arc::new(AppRunnerService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsAppRunnerServiceConverter::new(Arc::clone(&svc)));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_apprunner_service",
        "my_service",
        json!({
            "service_name": "my-app-runner-service",
            "status": "RUNNING",
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.services.contains_key("my-app-runner-service"));
    assert_eq!(view.services["my-app-runner-service"].status, "RUNNING");
}

// ---------------------------------------------------------------------------
// Audit Manager injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_auditmanager_control() {
    use winterbaume_auditmanager::AuditManagerService;
    use winterbaume_terraform::converters::auditmanager::AwsAuditManagerControlConverter;
    let svc = Arc::new(AuditManagerService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsAuditManagerControlConverter::new(Arc::clone(&svc)));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_auditmanager_control",
        "my_control",
        json!({
            "id": "ctrl-abc123",
            "name": "my-audit-control",
            "description": "Custom audit control",
            "tags": {"team": "security"},
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.controls.contains_key("ctrl-abc123"));
    assert_eq!(view.controls["ctrl-abc123"].name, "my-audit-control");
}

#[tokio::test]
async fn test_inject_auditmanager_framework() {
    use winterbaume_auditmanager::AuditManagerService;
    use winterbaume_terraform::converters::auditmanager::AwsAuditManagerFrameworkConverter;
    let svc = Arc::new(AuditManagerService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsAuditManagerFrameworkConverter::new(Arc::clone(&svc)));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_auditmanager_framework",
        "my_framework",
        json!({
            "id": "fw-xyz789",
            "name": "my-audit-framework",
            "description": "Custom audit framework",
            "compliance_type": "SOC2",
            "tags": {"team": "compliance"},
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.frameworks.contains_key("fw-xyz789"));
    assert_eq!(view.frameworks["fw-xyz789"].name, "my-audit-framework");
    assert_eq!(
        view.frameworks["fw-xyz789"].compliance_type.as_deref(),
        Some("SOC2")
    );
}

// ---------------------------------------------------------------------------
// Chatbot injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_chatbot_slack_channel_configuration() {
    use winterbaume_chatbot::ChatbotService;
    use winterbaume_terraform::converters::chatbot::AwsChatbotSlackChannelConfigurationConverter;
    let svc = Arc::new(ChatbotService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsChatbotSlackChannelConfigurationConverter::new(
        Arc::clone(&svc),
    ));
    let arn =
        "arn:aws:chatbot:us-east-1:123456789012:chat-configuration/slack-channel/my-slack-config";
    let tfstate = make_tfstate(vec![make_resource(
        "aws_chatbot_slack_channel_configuration",
        "my_slack",
        json!({
            "configuration_name": "my-slack-config",
            "slack_team_id": "T01234567",
            "slack_channel_id": "C01234567",
            "iam_role_arn": "arn:aws:iam::123456789012:role/chatbot-role",
            "chat_configuration_arn": arn,
            "logging_level": "INFO",
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.slack_configs.contains_key(arn));
    assert_eq!(
        view.slack_configs[arn].configuration_name,
        "my-slack-config"
    );
    assert_eq!(view.slack_configs[arn].slack_team_id, "T01234567");
}

#[tokio::test]
async fn test_inject_chatbot_teams_channel_configuration() {
    use winterbaume_chatbot::ChatbotService;
    use winterbaume_terraform::converters::chatbot::AwsChatbotTeamsChannelConfigurationConverter;
    let svc = Arc::new(ChatbotService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsChatbotTeamsChannelConfigurationConverter::new(
        Arc::clone(&svc),
    ));
    let arn = "arn:aws:chatbot:us-east-1:123456789012:chat-configuration/microsoft-teams-channel/my-teams-config";
    let tfstate = make_tfstate(vec![make_resource(
        "aws_chatbot_microsoft_teams_channel_configuration",
        "my_teams",
        json!({
            "configuration_name": "my-teams-config",
            "team_id": "team-guid-1234",
            "tenant_id": "tenant-guid-5678",
            "channel_id": "channel-guid-abcd",
            "iam_role_arn": "arn:aws:iam::123456789012:role/chatbot-role",
            "chat_configuration_arn": arn,
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.teams_configs.contains_key(arn));
    assert_eq!(
        view.teams_configs[arn].configuration_name,
        "my-teams-config"
    );
    assert_eq!(view.teams_configs[arn].team_id, "team-guid-1234");
}

// ---------------------------------------------------------------------------
// CloudFormation injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_cloudformation_stack() {
    use winterbaume_cloudformation::CloudFormationService;
    use winterbaume_terraform::converters::cloudformation::AwsCloudformationStackConverter;
    let svc = Arc::new(CloudFormationService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsCloudformationStackConverter::new(Arc::clone(&svc)));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_cloudformation_stack",
        "my_stack",
        json!({
            "name": "my-cfn-stack",
            "template_body": "{\"AWSTemplateFormatVersion\":\"2010-09-09\",\"Resources\":{}}",
            "parameters": {"Env": "prod"},
            "tags": {"Project": "infra"},
            "timeout_in_minutes": 30,
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.stacks.contains_key("my-cfn-stack"));
    assert_eq!(view.stacks["my-cfn-stack"].stack_status, "CREATE_COMPLETE");
    assert_eq!(view.stacks["my-cfn-stack"].timeout_in_minutes, Some(30));
}

// ---------------------------------------------------------------------------
// CodeArtifact injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_codeartifact_domain() {
    use winterbaume_codeartifact::CodeArtifactService;
    use winterbaume_terraform::converters::codeartifact::AwsCodeArtifactDomainConverter;
    let svc = Arc::new(CodeArtifactService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsCodeArtifactDomainConverter::new(Arc::clone(&svc)));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_codeartifact_domain",
        "my_domain",
        json!({
            "domain": "my-artifact-domain",
            "tags": {"env": "prod"},
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.domains.contains_key("my-artifact-domain"));
    assert_eq!(
        view.domains["my-artifact-domain"].name,
        "my-artifact-domain"
    );
    assert_eq!(
        view.domains["my-artifact-domain"].owner,
        ctx.default_account_id
    );
}

#[tokio::test]
async fn test_inject_codeartifact_repository() {
    use winterbaume_codeartifact::CodeArtifactService;
    use winterbaume_terraform::converters::codeartifact::{
        AwsCodeArtifactDomainConverter, AwsCodeArtifactRepositoryConverter,
    };
    let svc = Arc::new(CodeArtifactService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsCodeArtifactDomainConverter::new(Arc::clone(&svc)));
    injector.register(AwsCodeArtifactRepositoryConverter::new(Arc::clone(&svc)));
    let tfstate = make_tfstate(vec![
        make_resource(
            "aws_codeartifact_domain",
            "my_domain",
            json!({"domain": "my-domain"}),
        ),
        make_resource(
            "aws_codeartifact_repository",
            "my_repo",
            json!({
                "domain": "my-domain",
                "repository": "my-repo",
                "description": "My artifact repository",
            }),
        ),
    ]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 2);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    let repo_key = "my-domain/my-repo";
    assert!(
        view.repositories.contains_key(repo_key),
        "repositories: {:?}",
        view.repositories.keys().collect::<Vec<_>>()
    );
    assert_eq!(view.repositories[repo_key].name, "my-repo");
    assert_eq!(view.repositories[repo_key].domain_name, "my-domain");
}

// ---------------------------------------------------------------------------
// ElastiCache injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_elasticache_cluster() {
    use winterbaume_elasticache::ElastiCacheService;
    use winterbaume_terraform::converters::elasticache::AwsElastiCacheClusterConverter;
    let svc = Arc::new(ElastiCacheService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsElastiCacheClusterConverter::new(Arc::clone(&svc)));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_elasticache_cluster",
        "my_cluster",
        json!({
            "cluster_id": "my-cache-cluster",
            "engine": "redis",
            "engine_version": "7.0",
            "node_type": "cache.t3.micro",
            "num_cache_nodes": 1,
            "tags": {"env": "prod"},
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.cache_clusters.contains_key("my-cache-cluster"));
    assert_eq!(view.cache_clusters["my-cache-cluster"].engine, "redis");
    assert_eq!(
        view.cache_clusters["my-cache-cluster"].cache_node_type,
        "cache.t3.micro"
    );
}

#[tokio::test]
async fn test_inject_elasticache_replication_group() {
    use winterbaume_elasticache::ElastiCacheService;
    use winterbaume_terraform::converters::elasticache::AwsElastiCacheReplicationGroupConverter;
    let svc = Arc::new(ElastiCacheService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsElastiCacheReplicationGroupConverter::new(Arc::clone(
        &svc,
    )));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_elasticache_replication_group",
        "my_rg",
        json!({
            "replication_group_id": "my-replication-group",
            "description": "Primary Redis cluster",
            "node_type": "cache.t3.medium",
            "num_cache_clusters": 2,
            "automatic_failover_enabled": true,
            "multi_az_enabled": false,
            "tags": {"tier": "cache"},
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.replication_groups.contains_key("my-replication-group"));
    let rg = &view.replication_groups["my-replication-group"];
    assert_eq!(rg.num_cache_clusters, 2);
    assert_eq!(rg.automatic_failover, "enabled");
    assert_eq!(rg.cache_node_type, "cache.t3.medium");
}

#[tokio::test]
async fn test_inject_elasticache_subnet_group() {
    use winterbaume_elasticache::ElastiCacheService;
    use winterbaume_terraform::converters::elasticache::AwsElastiCacheSubnetGroupConverter;
    let svc = Arc::new(ElastiCacheService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsElastiCacheSubnetGroupConverter::new(Arc::clone(&svc)));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_elasticache_subnet_group",
        "my_sg",
        json!({
            "name": "my-cache-subnet-group",
            "description": "Cache subnet group",
            "subnet_ids": ["subnet-111", "subnet-222"],
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(
        view.cache_subnet_groups
            .contains_key("my-cache-subnet-group")
    );
    let sg = &view.cache_subnet_groups["my-cache-subnet-group"];
    assert_eq!(sg.subnet_ids, vec!["subnet-111", "subnet-222"]);
}

#[tokio::test]
async fn test_inject_elasticache_parameter_group() {
    use winterbaume_elasticache::ElastiCacheService;
    use winterbaume_terraform::converters::elasticache::AwsElastiCacheParameterGroupConverter;
    let svc = Arc::new(ElastiCacheService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsElastiCacheParameterGroupConverter::new(Arc::clone(&svc)));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_elasticache_parameter_group",
        "my_pg",
        json!({
            "name": "my-cache-params",
            "family": "redis7",
            "description": "Custom Redis 7 parameter group",
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.cache_parameter_groups.contains_key("my-cache-params"));
    let pg = &view.cache_parameter_groups["my-cache-params"];
    assert_eq!(pg.family, "redis7");
    assert_eq!(pg.description, "Custom Redis 7 parameter group");
}

// ---------------------------------------------------------------------------
// EMR injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_emr_cluster() {
    use winterbaume_emr::EmrService;
    use winterbaume_terraform::converters::emr::AwsEmrClusterConverter;
    let svc = Arc::new(EmrService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsEmrClusterConverter::new(Arc::clone(&svc)));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_emr_cluster",
        "my_cluster",
        json!({
            "id": "j-EXAMPLECLUSTERID",
            "name": "my-emr-cluster",
            "release_label": "emr-6.15.0",
            "service_role": "arn:aws:iam::123456789012:role/EMR_DefaultRole",
            "log_uri": "s3://my-bucket/emr-logs/",
            "tags": {"Project": "analytics"},
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.clusters.contains_key("j-EXAMPLECLUSTERID"));
    let cluster = &view.clusters["j-EXAMPLECLUSTERID"];
    assert_eq!(cluster.name, "my-emr-cluster");
    assert_eq!(cluster.release_label.as_deref(), Some("emr-6.15.0"));
}

#[tokio::test]
async fn test_inject_emr_security_configuration() {
    use winterbaume_emr::EmrService;
    use winterbaume_terraform::converters::emr::AwsEmrSecurityConfigurationConverter;
    let svc = Arc::new(EmrService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsEmrSecurityConfigurationConverter::new(Arc::clone(&svc)));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_emr_security_configuration",
        "my_sec_config",
        json!({
            "name": "my-emr-security-config",
            "configuration": r#"{"EncryptionConfiguration":{"EnableInTransitEncryption":false,"EnableAtRestEncryption":false}}"#,
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(
        view.security_configurations
            .contains_key("my-emr-security-config")
    );
    assert_eq!(
        view.security_configurations["my-emr-security-config"].name,
        "my-emr-security-config"
    );
}

// ---------------------------------------------------------------------------
// OpenSearch Serverless injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_opensearchserverless_collection() {
    use winterbaume_opensearchserverless::OpenSearchServerlessService;
    use winterbaume_terraform::converters::opensearchserverless::AwsOpensearchserverlessCollectionConverter;
    let svc = Arc::new(OpenSearchServerlessService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsOpensearchserverlessCollectionConverter::new(Arc::clone(
        &svc,
    )));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_opensearchserverless_collection",
        "my_collection",
        json!({
            "id": "aoss1234abcd",
            "name": "my-aoss-collection",
            "type": "SEARCH",
            "description": "My AOSS search collection",
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.collections.contains_key("aoss1234abcd"));
    let coll = &view.collections["aoss1234abcd"];
    assert_eq!(coll.name, "my-aoss-collection");
    assert_eq!(coll.type_, "SEARCH");
}

#[tokio::test]
async fn test_inject_opensearchserverless_security_policy() {
    use winterbaume_opensearchserverless::OpenSearchServerlessService;
    use winterbaume_terraform::converters::opensearchserverless::AwsOpensearchserverlessSecurityPolicyConverter;
    let svc = Arc::new(OpenSearchServerlessService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsOpensearchserverlessSecurityPolicyConverter::new(
        Arc::clone(&svc),
    ));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_opensearchserverless_security_policy",
        "my_policy",
        json!({
            "name": "my-enc-policy",
            "type": "encryption",
            "policy": r#"{"Rules":[{"Resource":["collection/my-collection"],"ResourceType":"collection"}],"AWSOwnedKey":true}"#,
            "description": "Encryption policy",
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    let policy_key = "encryption/my-enc-policy";
    assert!(
        view.security_policies.contains_key(policy_key),
        "security_policies: {:?}",
        view.security_policies.keys().collect::<Vec<_>>()
    );
    assert_eq!(view.security_policies[policy_key].type_, "encryption");
}

// ---------------------------------------------------------------------------
// Redshift injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_redshift_cluster() {
    use winterbaume_redshift::RedshiftService;
    use winterbaume_terraform::converters::redshift::AwsRedshiftClusterConverter;
    let svc = Arc::new(RedshiftService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsRedshiftClusterConverter::new(Arc::clone(&svc)));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_redshift_cluster",
        "my_cluster",
        json!({
            "cluster_identifier": "my-redshift-cluster",
            "node_type": "dc2.large",
            "master_username": "admin",
            "database_name": "mydb",
            "number_of_nodes": 2,
            "encrypted": true,
            "tags": {"env": "prod"},
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.clusters.contains_key("my-redshift-cluster"));
    let cluster = &view.clusters["my-redshift-cluster"];
    assert_eq!(cluster.node_type, "dc2.large");
    assert_eq!(cluster.number_of_nodes, 2);
    assert!(cluster.encrypted);
}

#[tokio::test]
async fn test_inject_redshift_subnet_group() {
    use winterbaume_redshift::RedshiftService;
    use winterbaume_terraform::converters::redshift::AwsRedshiftSubnetGroupConverter;
    let svc = Arc::new(RedshiftService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsRedshiftSubnetGroupConverter::new(Arc::clone(&svc)));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_redshift_subnet_group",
        "my_sg",
        json!({
            "name": "my-redshift-subnet-group",
            "description": "Redshift subnet group",
            "subnet_ids": ["subnet-aaa", "subnet-bbb"],
            "tags": {"tier": "data"},
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.subnet_groups.contains_key("my-redshift-subnet-group"));
    let sg = &view.subnet_groups["my-redshift-subnet-group"];
    assert_eq!(sg.description, "Redshift subnet group");
    assert_eq!(sg.subnet_ids, vec!["subnet-aaa", "subnet-bbb"]);
}

// ---------------------------------------------------------------------------
// SES v1 injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_ses_v1_email_identity() {
    use winterbaume_ses::SesService;
    use winterbaume_terraform::converters::sesv1::AwsSesEmailIdentityConverter;
    let svc = Arc::new(SesService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsSesEmailIdentityConverter::new(Arc::clone(&svc)));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_ses_email_identity",
        "my_email",
        json!({
            "email": "sender@example.com",
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.identities.contains_key("sender@example.com"));
    assert_eq!(
        view.identities["sender@example.com"].identity_type,
        "EmailAddress"
    );
}

#[tokio::test]
async fn test_inject_ses_v1_domain_identity() {
    use winterbaume_ses::SesService;
    use winterbaume_terraform::converters::sesv1::AwsSesDomainIdentityConverter;
    let svc = Arc::new(SesService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsSesDomainIdentityConverter::new(Arc::clone(&svc)));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_ses_domain_identity",
        "my_domain",
        json!({
            "domain": "example.com",
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.identities.contains_key("example.com"));
    assert_eq!(view.identities["example.com"].identity_type, "Domain");
}

// ---------------------------------------------------------------------------
// SSO Admin injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_ssoadmin_permission_set() {
    use winterbaume_ssoadmin::SsoAdminService;
    use winterbaume_terraform::converters::ssoadmin::AwsSsoadminPermissionSetConverter;
    let svc = Arc::new(SsoAdminService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsSsoadminPermissionSetConverter::new(Arc::clone(&svc)));
    let ps_arn = "arn:aws:sso:::permissionSet/ssoins-0123456789abcdef/ps-example123";
    let tfstate = make_tfstate(vec![make_resource(
        "aws_ssoadmin_permission_set",
        "my_ps",
        json!({
            "name": "ReadOnlyAccess",
            "arn": ps_arn,
            "description": "Read-only permission set",
            "session_duration": "PT4H",
            "tags": {"team": "platform"},
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(
        view.permission_sets.contains_key(ps_arn),
        "permission_sets: {:?}",
        view.permission_sets.keys().collect::<Vec<_>>()
    );
    assert_eq!(view.permission_sets[ps_arn].name, "ReadOnlyAccess");
    assert_eq!(
        view.permission_sets[ps_arn].session_duration.as_deref(),
        Some("PT4H")
    );
}

#[tokio::test]
async fn test_inject_ssoadmin_account_assignment() {
    use winterbaume_ssoadmin::SsoAdminService;
    use winterbaume_terraform::converters::ssoadmin::{
        AwsSsoadminAccountAssignmentConverter, AwsSsoadminPermissionSetConverter,
    };
    let svc = Arc::new(SsoAdminService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsSsoadminPermissionSetConverter::new(Arc::clone(&svc)));
    injector.register(AwsSsoadminAccountAssignmentConverter::new(Arc::clone(&svc)));
    let ps_arn = "arn:aws:sso:::permissionSet/ssoins-0123456789abcdef/ps-abc123";
    let tfstate = make_tfstate(vec![
        make_resource(
            "aws_ssoadmin_permission_set",
            "my_ps",
            json!({"name": "AdminAccess", "arn": ps_arn}),
        ),
        make_resource(
            "aws_ssoadmin_account_assignment",
            "my_assignment",
            json!({
                "permission_set_arn": ps_arn,
                "principal_id": "user-guid-1234",
                "principal_type": "USER",
                "target_id": "123456789012",
                "target_type": "AWS_ACCOUNT",
            }),
        ),
    ]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 2);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert_eq!(view.account_assignments.len(), 1);
    let assignment = &view.account_assignments[0];
    assert_eq!(assignment.permission_set_arn, ps_arn);
    assert_eq!(assignment.principal_id, "user-guid-1234");
    assert_eq!(assignment.account_id, "123456789012");
}

// ---------------------------------------------------------------------------
// WorkSpaces injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_workspaces_directory() {
    use winterbaume_terraform::converters::workspaces::AwsWorkspacesDirectoryConverter;
    use winterbaume_workspaces::WorkSpacesService;
    let svc = Arc::new(WorkSpacesService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsWorkspacesDirectoryConverter::new(Arc::clone(&svc)));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_workspaces_directory",
        "my_dir",
        json!({
            "directory_id": "d-1234567890",
            "directory_name": "corp.example.com",
            "directory_type": "SIMPLE_AD",
            "alias": "corp-example",
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.directories.contains_key("d-1234567890"));
    let dir = &view.directories["d-1234567890"];
    assert_eq!(dir.directory_name, "corp.example.com");
    assert_eq!(dir.directory_type, "SIMPLE_AD");
}

#[tokio::test]
async fn test_inject_workspaces_workspace() {
    use winterbaume_terraform::converters::workspaces::AwsWorkspacesWorkspaceConverter;
    use winterbaume_workspaces::WorkSpacesService;
    let svc = Arc::new(WorkSpacesService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsWorkspacesWorkspaceConverter::new(Arc::clone(&svc)));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_workspaces_workspace",
        "my_workspace",
        json!({
            "id": "ws-abc123def456",
            "directory_id": "d-1234567890",
            "user_name": "jdoe",
            "bundle_id": "wsb-0123456789abcdef0",
            "state": "AVAILABLE",
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.workspaces.contains_key("ws-abc123def456"));
    let ws = &view.workspaces["ws-abc123def456"];
    assert_eq!(ws.user_name, "jdoe");
    assert_eq!(ws.directory_id, "d-1234567890");
    assert_eq!(ws.bundle_id, "wsb-0123456789abcdef0");
}

// ---------------------------------------------------------------------------
// Athena data catalog injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_athena_data_catalog() {
    use winterbaume_athena::AthenaService;
    use winterbaume_terraform::converters::athena::AwsAthenaDataCatalogConverter;
    let svc = Arc::new(AthenaService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsAthenaDataCatalogConverter::new(Arc::clone(&svc)));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_athena_data_catalog",
        "my_catalog",
        json!({
            "name": "my-data-catalog",
            "type": "GLUE",
            "description": "Glue data catalog",
            "parameters": {"catalog-id": "123456789012"},
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.data_catalogs.contains_key("my-data-catalog"));
    assert_eq!(view.data_catalogs["my-data-catalog"].catalog_type, "GLUE");
}

// ---------------------------------------------------------------------------
// EventBridge event bus injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_cloudwatch_event_bus() {
    use winterbaume_eventbridge::EventBridgeService;
    use winterbaume_terraform::converters::events::AwsCloudwatchEventBusConverter;
    let svc = Arc::new(EventBridgeService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsCloudwatchEventBusConverter::new(Arc::clone(&svc)));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_cloudwatch_event_bus",
        "my_bus",
        json!({
            "name": "my-custom-bus",
            "arn": "arn:aws:events:us-east-1:123456789012:event-bus/my-custom-bus",
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.event_buses.contains_key("my-custom-bus"));
    assert_eq!(view.event_buses["my-custom-bus"].name, "my-custom-bus");
}

// ---------------------------------------------------------------------------
// EKS node group injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_eks_node_group() {
    use winterbaume_eks::EksService;
    use winterbaume_terraform::converters::eks::{
        AwsEksClusterConverter, AwsEksNodeGroupConverter,
    };
    let svc = Arc::new(EksService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsEksClusterConverter::new(Arc::clone(&svc)));
    injector.register(AwsEksNodeGroupConverter::new(Arc::clone(&svc)));
    let tfstate = make_tfstate(vec![
        make_resource(
            "aws_eks_cluster",
            "my_cluster",
            json!({
                "name": "my-cluster",
                "role_arn": "arn:aws:iam::123456789012:role/eks-role",
                "version": "1.29",
            }),
        ),
        make_resource(
            "aws_eks_node_group",
            "my_ng",
            json!({
                "cluster_name": "my-cluster",
                "node_group_name": "my-node-group",
                "node_role_arn": "arn:aws:iam::123456789012:role/eks-node-role",
                "scaling_config": [{"desired_size": 2, "min_size": 1, "max_size": 4}],
                "instance_types": ["t3.medium"],
                "tags": {"env": "prod"},
            }),
        ),
    ]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 2);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.clusters.contains_key("my-cluster"));
    assert!(
        view.clusters["my-cluster"]
            .nodegroups
            .contains_key("my-node-group")
    );
    let ng = &view.clusters["my-cluster"].nodegroups["my-node-group"];
    assert_eq!(ng.name, "my-node-group");
    assert_eq!(ng.scaling_config.desired_size, 2);
}

// ---------------------------------------------------------------------------
// Glue crawler injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_glue_crawler() {
    use winterbaume_glue::GlueService;
    use winterbaume_terraform::converters::glue::AwsGlueCrawlerConverter;
    let svc = Arc::new(GlueService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsGlueCrawlerConverter::new(Arc::clone(&svc)));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_glue_crawler",
        "my_crawler",
        json!({
            "name": "my-glue-crawler",
            "role": "arn:aws:iam::123456789012:role/glue-role",
            "database_name": "my-database",
            "description": "S3 data crawler",
            "schedule": "cron(0 12 * * ? *)",
            "s3_target": [{"path": "s3://my-bucket/data/"}],
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.crawlers.contains_key("my-glue-crawler"));
    let crawler = &view.crawlers["my-glue-crawler"];
    assert_eq!(crawler.database_name, "my-database");
    assert_eq!(crawler.schedule.as_deref(), Some("cron(0 12 * * ? *)"));
}

// ---------------------------------------------------------------------------
// IAM group, instance profile and user policy attachment injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_iam_group() {
    use winterbaume_terraform::converters::iam::AwsIamGroupConverter;
    let iam = Arc::new(IamService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsIamGroupConverter::new(Arc::clone(&iam)));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_iam_group",
        "devs",
        json!({
            "name": "developers",
            "path": "/teams/",
            "arn": "arn:aws:iam::123456789012:group/teams/developers",
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);
    let view = iam
        .snapshot(&ctx.default_account_id, &ctx.default_region)
        .await;
    assert!(view.groups.contains_key("developers"));
    assert_eq!(view.groups["developers"].path, "/teams/");
}

#[tokio::test]
async fn test_inject_iam_instance_profile() {
    use winterbaume_terraform::converters::iam::{
        AwsIamInstanceProfileConverter, AwsIamRoleConverter,
    };
    let iam = Arc::new(IamService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsIamRoleConverter::new(Arc::clone(&iam)));
    injector.register(AwsIamInstanceProfileConverter::new(Arc::clone(&iam)));
    let tfstate = make_tfstate(vec![
        make_resource(
            "aws_iam_role",
            "ec2_role",
            json!({
                "name": "ec2-instance-role",
                "path": "/",
                "assume_role_policy": "{}",
                "description": "",
                "max_session_duration": 3600,
            }),
        ),
        make_resource(
            "aws_iam_instance_profile",
            "ec2_profile",
            json!({
                "name": "ec2-instance-profile",
                "path": "/",
                "role": "ec2-instance-role",
                "arn": "arn:aws:iam::123456789012:instance-profile/ec2-instance-profile",
            }),
        ),
    ]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 2);
    let view = iam
        .snapshot(&ctx.default_account_id, &ctx.default_region)
        .await;
    assert!(view.instance_profiles.contains_key("ec2-instance-profile"));
    assert_eq!(
        view.instance_profiles["ec2-instance-profile"].roles,
        vec!["ec2-instance-role"]
    );
}

#[tokio::test]
async fn test_inject_iam_user_policy_attachment() {
    use winterbaume_terraform::converters::iam::{
        AwsIamPolicyConverter, AwsIamUserConverter, AwsIamUserPolicyAttachmentConverter,
    };
    let iam = Arc::new(IamService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsIamUserConverter::new(Arc::clone(&iam)));
    injector.register(AwsIamPolicyConverter::new(Arc::clone(&iam)));
    injector.register(AwsIamUserPolicyAttachmentConverter::new(Arc::clone(&iam)));
    let policy_arn = "arn:aws:iam::123456789012:policy/s3-read-policy";
    let tfstate = make_tfstate(vec![
        make_resource(
            "aws_iam_user",
            "ci",
            json!({"name": "ci-user", "path": "/"}),
        ),
        make_resource(
            "aws_iam_policy",
            "s3_read",
            json!({
                "name": "s3-read-policy",
                "arn": policy_arn,
                "path": "/",
                "description": "",
                "policy": "{}",
            }),
        ),
        make_resource(
            "aws_iam_user_policy_attachment",
            "ci_s3",
            json!({
                "user": "ci-user",
                "policy_arn": policy_arn,
            }),
        ),
    ]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 3);
    let view = iam
        .snapshot(&ctx.default_account_id, &ctx.default_region)
        .await;
    let user = &view.users["ci-user"];
    assert_eq!(user.attached_policies.len(), 1);
    assert_eq!(user.attached_policies[0].policy_arn, policy_arn);
}

// ---------------------------------------------------------------------------
// Lambda event source mapping injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_lambda_event_source_mapping() {
    use winterbaume_lambda::LambdaService;
    use winterbaume_terraform::converters::lambda::{
        AwsLambdaEventSourceMappingConverter, AwsLambdaFunctionConverter,
    };
    let svc = Arc::new(LambdaService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsLambdaFunctionConverter::new(Arc::clone(&svc)));
    injector.register(AwsLambdaEventSourceMappingConverter::new(Arc::clone(&svc)));
    let esm_uuid = "a1b2c3d4-e5f6-7890-abcd-ef1234567890";
    let tfstate = make_tfstate(vec![
        make_resource(
            "aws_lambda_function",
            "processor",
            json!({
                "function_name": "event-processor",
                "runtime": "python3.11",
                "handler": "index.handler",
                "role": "arn:aws:iam::123456789012:role/lambda-role",
            }),
        ),
        make_resource(
            "aws_lambda_event_source_mapping",
            "sqs_trigger",
            json!({
                "uuid": esm_uuid,
                "function_name": "event-processor",
                "event_source_arn": "arn:aws:sqs:us-east-1:123456789012:my-queue",
                "batch_size": 10,
                "enabled": true,
                "starting_position": null,
            }),
        ),
    ]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 2);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.event_source_mappings.contains_key(esm_uuid));
    let esm = &view.event_source_mappings[esm_uuid];
    assert_eq!(
        esm.event_source_arn.as_deref(),
        Some("arn:aws:sqs:us-east-1:123456789012:my-queue")
    );
    assert_eq!(esm.batch_size, Some(10));
}

// ---------------------------------------------------------------------------
// SESv2 configuration set and dedicated IP pool injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_sesv2_configuration_set() {
    use winterbaume_sesv2::SesV2Service;
    use winterbaume_terraform::converters::ses::AwsSesv2ConfigurationSetConverter;
    let svc = Arc::new(SesV2Service::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsSesv2ConfigurationSetConverter::new(Arc::clone(&svc)));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_sesv2_configuration_set",
        "my_cs",
        json!({
            "configuration_set_name": "my-config-set",
            "tags": {"env": "prod"},
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.configuration_sets.contains_key("my-config-set"));
    assert_eq!(
        view.configuration_sets["my-config-set"].name,
        "my-config-set"
    );
}

#[tokio::test]
async fn test_inject_sesv2_dedicated_ip_pool() {
    use winterbaume_sesv2::SesV2Service;
    use winterbaume_terraform::converters::ses::AwsSesv2DedicatedIpPoolConverter;
    let svc = Arc::new(SesV2Service::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsSesv2DedicatedIpPoolConverter::new(Arc::clone(&svc)));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_sesv2_dedicated_ip_pool",
        "my_pool",
        json!({
            "pool_name": "my-dedicated-pool",
            "scaling_mode": "MANAGED",
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.dedicated_ip_pools.contains_key("my-dedicated-pool"));
    assert_eq!(
        view.dedicated_ip_pools["my-dedicated-pool"].scaling_mode,
        "MANAGED"
    );
}

// ---------------------------------------------------------------------------
// SNS topic subscription injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_sns_topic_subscription() {
    use winterbaume_sns::SnsService;
    use winterbaume_terraform::converters::sns::{
        AwsSnsTopicConverter, AwsSnsTopicSubscriptionConverter,
    };
    let svc = Arc::new(SnsService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsSnsTopicConverter::new(Arc::clone(&svc)));
    injector.register(AwsSnsTopicSubscriptionConverter::new(Arc::clone(&svc)));
    let topic_arn = "arn:aws:sns:us-east-1:123456789012:notifications";
    let sub_arn = "arn:aws:sns:us-east-1:123456789012:notifications:abc-123";
    let tfstate = make_tfstate(vec![
        make_resource(
            "aws_sns_topic",
            "notifications",
            json!({
                "name": "notifications",
                "arn": topic_arn,
            }),
        ),
        make_resource(
            "aws_sns_topic_subscription",
            "email_sub",
            json!({
                "topic_arn": topic_arn,
                "protocol": "email",
                "endpoint": "ops@example.com",
                "arn": sub_arn,
            }),
        ),
    ]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 2);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.subscriptions.contains_key(sub_arn));
    let sub = &view.subscriptions[sub_arn];
    assert_eq!(sub.protocol, "email");
    assert_eq!(sub.endpoint, "ops@example.com");
    assert_eq!(sub.topic_arn, topic_arn);
}

// ---------------------------------------------------------------------------
// WAFv2 rule group injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_wafv2_rule_group() {
    use winterbaume_terraform::converters::wafv2::AwsWafv2RuleGroupConverter;
    use winterbaume_wafv2::WafV2Service;
    let svc = Arc::new(WafV2Service::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsWafv2RuleGroupConverter::new(Arc::clone(&svc)));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_wafv2_rule_group",
        "my_rg",
        json!({
            "name": "my-rule-group",
            "scope": "REGIONAL",
            "description": "Custom rule group",
            "capacity": 100,
            "visibility_config": [{"cloudwatch_metrics_enabled": false, "metric_name": "my-rule-group", "sampled_requests_enabled": false}],
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.rule_groups.contains_key("REGIONAL:my-rule-group"));
    let rg = &view.rule_groups["REGIONAL:my-rule-group"];
    assert_eq!(rg.name, "my-rule-group");
    assert_eq!(rg.capacity, 100);
}

// ---------------------------------------------------------------------------
// Config delivery channel injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_config_config_rule() {
    use winterbaume_config::ConfigService;
    use winterbaume_terraform::converters::config::AwsConfigConfigRuleConverter;
    let svc = Arc::new(ConfigService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsConfigConfigRuleConverter::new(Arc::clone(&svc)));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_config_config_rule",
        "my_rule",
        json!({
            "name": "my-config-rule",
            "source": [{"owner": "AWS", "source_identifier": "S3_BUCKET_VERSIONING_ENABLED"}],
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.config_rules.contains_key("my-config-rule"));
    assert_eq!(
        view.config_rules["my-config-rule"]
            .source_identifier
            .as_deref(),
        Some("S3_BUCKET_VERSIONING_ENABLED")
    );
}

#[tokio::test]
async fn test_inject_config_recorder() {
    use winterbaume_config::ConfigService;
    use winterbaume_terraform::converters::config::AwsConfigConfigurationRecorderConverter;
    let svc = Arc::new(ConfigService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsConfigConfigurationRecorderConverter::new(Arc::clone(
        &svc,
    )));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_config_configuration_recorder",
        "my_recorder",
        json!({
            "name": "my-recorder",
            "role_arn": "arn:aws:iam::123456789012:role/config-role",
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.recorders.contains_key("my-recorder"));
    assert_eq!(
        view.recorders["my-recorder"].role_arn,
        "arn:aws:iam::123456789012:role/config-role"
    );
}

#[tokio::test]
async fn test_inject_config_delivery_channel() {
    use winterbaume_config::ConfigService;
    use winterbaume_terraform::converters::config::AwsConfigDeliveryChannelConverter;
    let svc = Arc::new(ConfigService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsConfigDeliveryChannelConverter::new(Arc::clone(&svc)));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_config_delivery_channel",
        "my_channel",
        json!({
            "name": "my-delivery-channel",
            "s3_bucket_name": "config-bucket",
            "s3_key_prefix": "config/",
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.delivery_channels.contains_key("my-delivery-channel"));
    let ch = &view.delivery_channels["my-delivery-channel"];
    assert_eq!(ch.s3_bucket_name, "config-bucket");
    assert_eq!(ch.s3_key_prefix, "config/");
}

// ---------------------------------------------------------------------------
// EC2 injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_ec2_vpc_and_subnet() {
    let ec2 = Arc::new(Ec2Service::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsVpcConverter::new(Arc::clone(&ec2)));
    injector.register(AwsSubnetConverter::new(Arc::clone(&ec2)));

    let tfstate = make_tfstate(vec![
        make_resource(
            "aws_vpc",
            "main",
            json!({
                "id": "vpc-12345",
                "cidr_block": "10.0.0.0/16",
                "enable_dns_support": true,
                "enable_dns_hostnames": true,
                "tags": { "Name": "main-vpc" },
            }),
        ),
        make_resource(
            "aws_subnet",
            "public",
            json!({
                "id": "subnet-67890",
                "vpc_id": "vpc-12345",
                "cidr_block": "10.0.1.0/24",
                "availability_zone": "us-east-1a",
                "map_public_ip_on_launch": true,
                "tags": { "Name": "public-subnet" },
            }),
        ),
    ]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 2);

    let view = ec2.snapshot(&ctx.default_account_id, "us-east-1").await;

    // Verify VPC
    assert!(view.vpcs.contains_key("vpc-12345"));
    let vpc = &view.vpcs["vpc-12345"];
    assert_eq!(vpc.cidr_block, "10.0.0.0/16");
    assert!(vpc.enable_dns_support);
    assert!(vpc.enable_dns_hostnames);
    assert_eq!(vpc.tags.get("Name").unwrap(), "main-vpc");

    // Verify subnet
    assert!(view.subnets.contains_key("subnet-67890"));
    let subnet = &view.subnets["subnet-67890"];
    assert_eq!(subnet.vpc_id, "vpc-12345");
    assert_eq!(subnet.cidr_block, "10.0.1.0/24");
    assert_eq!(subnet.availability_zone, "us-east-1a");
    assert!(subnet.map_public_ip_on_launch);
}

#[tokio::test]
async fn test_inject_ec2_security_group() {
    let ec2 = Arc::new(Ec2Service::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsVpcConverter::new(Arc::clone(&ec2)));
    injector.register(AwsSecurityGroupConverter::new(Arc::clone(&ec2)));

    let tfstate = make_tfstate(vec![
        make_resource(
            "aws_vpc",
            "main",
            json!({
                "id": "vpc-12345",
                "cidr_block": "10.0.0.0/16",
            }),
        ),
        make_resource(
            "aws_security_group",
            "web",
            json!({
                "id": "sg-web123",
                "name": "web-sg",
                "description": "Allow HTTP",
                "vpc_id": "vpc-12345",
                "ingress": [
                    {
                        "from_port": 80,
                        "to_port": 80,
                        "protocol": "tcp",
                        "cidr_blocks": ["0.0.0.0/0"],
                    }
                ],
                "egress": [
                    {
                        "from_port": 0,
                        "to_port": 0,
                        "protocol": "-1",
                        "cidr_blocks": ["0.0.0.0/0"],
                    }
                ],
                "tags": { "Name": "web-sg" },
            }),
        ),
    ]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 2);

    let view = ec2.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.security_groups.contains_key("sg-web123"));
    let sg = &view.security_groups["sg-web123"];
    assert_eq!(sg.group_name, "web-sg");
    assert_eq!(sg.vpc_id, "vpc-12345");
    assert_eq!(sg.ingress_rules.len(), 1);
    assert_eq!(sg.ingress_rules[0].from_port, Some(80));
    assert_eq!(sg.ingress_rules[0].ip_ranges[0].cidr_ip, "0.0.0.0/0");
    assert_eq!(sg.egress_rules.len(), 1);
}

#[tokio::test]
async fn test_inject_ec2_igw() {
    let ec2 = Arc::new(Ec2Service::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsVpcConverter::new(Arc::clone(&ec2)));
    injector.register(AwsInternetGatewayConverter::new(Arc::clone(&ec2)));

    let tfstate = make_tfstate(vec![
        make_resource(
            "aws_vpc",
            "main",
            json!({
                "id": "vpc-12345",
                "cidr_block": "10.0.0.0/16",
            }),
        ),
        make_resource(
            "aws_internet_gateway",
            "gw",
            json!({
                "id": "igw-abc123",
                "vpc_id": "vpc-12345",
                "tags": { "Name": "main-igw" },
            }),
        ),
    ]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 2);

    let view = ec2.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.igws.contains_key("igw-abc123"));
    let igw = &view.igws["igw-abc123"];
    assert_eq!(igw.attachments.len(), 1);
    assert_eq!(igw.attachments[0].vpc_id, "vpc-12345");
    assert_eq!(igw.tags.get("Name").unwrap(), "main-igw");
}

// ---------------------------------------------------------------------------
// EC2 extract tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_ec2_vpc_extract_round_trip() {
    let ec2 = Arc::new(Ec2Service::new());
    let ctx = default_ctx();

    let converter = AwsVpcConverter::new(Arc::clone(&ec2));

    // Inject a VPC
    let mut injector = TerraformInjector::new();
    injector.register(AwsVpcConverter::new(Arc::clone(&ec2)));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_vpc",
        "main",
        json!({
            "id": "vpc-rt1",
            "cidr_block": "10.0.0.0/16",
            "enable_dns_support": true,
            "tags": { "Env": "test" },
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success());

    // Extract
    let extracted = converter
        .extract(&ctx)
        .await
        .expect("extract should succeed");
    assert_eq!(extracted.len(), 1);
    assert_eq!(extracted[0].attributes["cidr_block"], "10.0.0.0/16");
    assert_eq!(extracted[0].attributes["id"], "vpc-rt1");
}

// ---------------------------------------------------------------------------
// RDS injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_rds_db_instance() {
    let rds = Arc::new(RdsService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsDbSubnetGroupConverter::new(Arc::clone(&rds)));
    injector.register(AwsDbInstanceConverter::new(Arc::clone(&rds)));

    let tfstate = make_tfstate(vec![
        make_resource(
            "aws_db_subnet_group",
            "default",
            json!({
                "name": "my-subnet-group",
                "description": "My DB subnet group",
                "subnet_ids": ["subnet-1", "subnet-2"],
            }),
        ),
        make_resource(
            "aws_db_instance",
            "main",
            json!({
                "identifier": "my-db",
                "engine": "mysql",
                "engine_version": "8.0",
                "instance_class": "db.t3.micro",
                "allocated_storage": 20,
                "storage_type": "gp2",
                "username": "admin",
                "db_name": "mydb",
                "port": 3306,
                "multi_az": false,
                "db_subnet_group_name": "my-subnet-group",
                "vpc_security_group_ids": ["sg-123"],
                "parameter_group_name": "default.mysql8.0",
                "backup_retention_period": 7,
                "deletion_protection": true,
                "storage_encrypted": true,
                "tags": { "Env": "production" },
            }),
        ),
    ]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 2);

    let view = rds.snapshot(&ctx.default_account_id, "us-east-1").await;

    // Verify subnet group
    assert!(view.db_subnet_groups.contains_key("my-subnet-group"));
    let sg = &view.db_subnet_groups["my-subnet-group"];
    assert_eq!(sg.subnet_ids, vec!["subnet-1", "subnet-2"]);

    // Verify DB instance
    assert!(view.db_instances.contains_key("my-db"));
    let dbi = &view.db_instances["my-db"];
    assert_eq!(dbi.engine, "mysql");
    assert_eq!(dbi.engine_version, "8.0");
    assert_eq!(dbi.db_instance_class, "db.t3.micro");
    assert_eq!(dbi.allocated_storage, 20);
    assert_eq!(dbi.master_username, Some("admin".to_string()));
    assert_eq!(dbi.db_name, Some("mydb".to_string()));
    assert_eq!(dbi.port, Some(3306));
    assert!(!dbi.multi_az);
    assert_eq!(
        dbi.db_subnet_group_name,
        Some("my-subnet-group".to_string())
    );
    assert_eq!(dbi.vpc_security_group_ids, vec!["sg-123"]);
    assert_eq!(dbi.backup_retention_period, 7);
    assert!(dbi.deletion_protection);
    assert!(dbi.storage_encrypted);
    assert!(
        dbi.tags
            .iter()
            .any(|t| t.key == "Env" && t.value == "production")
    );
}

#[tokio::test]
async fn test_rds_db_instance_extract_round_trip() {
    let rds = Arc::new(RdsService::new());
    let ctx = default_ctx();

    let converter = AwsDbInstanceConverter::new(Arc::clone(&rds));

    let mut injector = TerraformInjector::new();
    injector.register(AwsDbInstanceConverter::new(Arc::clone(&rds)));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_db_instance",
        "main",
        json!({
            "identifier": "roundtrip-db",
            "engine": "postgres",
            "engine_version": "15.4",
            "instance_class": "db.t3.small",
            "allocated_storage": 50,
            "username": "pguser",
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success());

    let extracted = converter
        .extract(&ctx)
        .await
        .expect("extract should succeed");
    assert_eq!(extracted.len(), 1);
    assert_eq!(extracted[0].attributes["identifier"], "roundtrip-db");
    assert_eq!(extracted[0].attributes["engine"], "postgres");
    assert_eq!(extracted[0].attributes["instance_class"], "db.t3.small");
    assert_eq!(extracted[0].attributes["allocated_storage"], 50);
}

// ---------------------------------------------------------------------------
// DAX Parameter Group injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_dax_parameter_group() {
    let dax = Arc::new(DaxService::new());
    let ctx = default_ctx();

    let converter = AwsDaxParameterGroupConverter::new(Arc::clone(&dax));

    let mut injector = TerraformInjector::new();
    injector.register(AwsDaxParameterGroupConverter::new(Arc::clone(&dax)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_dax_parameter_group",
        "test_pg",
        json!({
            "name": "my-dax-params",
            "description": "Custom DAX parameter group",
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    // Verify via snapshot
    let view = dax
        .snapshot(&ctx.default_account_id, &ctx.default_region)
        .await;
    assert!(view.parameter_groups.contains_key("my-dax-params"));
    let pg = &view.parameter_groups["my-dax-params"];
    assert_eq!(pg.parameter_group_name, "my-dax-params");
    assert_eq!(pg.description, "Custom DAX parameter group");

    // Verify extract round-trip
    let extracted = converter
        .extract(&ctx)
        .await
        .expect("extract should succeed");
    assert_eq!(extracted.len(), 1);
    assert_eq!(extracted[0].attributes["name"], "my-dax-params");
    assert_eq!(
        extracted[0].attributes["description"],
        "Custom DAX parameter group"
    );
}

// ---------------------------------------------------------------------------
// DAX Subnet Group injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_dax_subnet_group() {
    let dax = Arc::new(DaxService::new());
    let ctx = default_ctx();

    let converter = AwsDaxSubnetGroupConverter::new(Arc::clone(&dax));

    let mut injector = TerraformInjector::new();
    injector.register(AwsDaxSubnetGroupConverter::new(Arc::clone(&dax)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_dax_subnet_group",
        "test_sg",
        json!({
            "name": "my-dax-subnets",
            "description": "DAX subnet group for test",
            "subnet_ids": ["subnet-aaa111", "subnet-bbb222"],
            "vpc_id": "vpc-12345",
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    // Verify via snapshot
    let view = dax
        .snapshot(&ctx.default_account_id, &ctx.default_region)
        .await;
    assert!(view.subnet_groups.contains_key("my-dax-subnets"));
    let sg = &view.subnet_groups["my-dax-subnets"];
    assert_eq!(sg.subnet_group_name, "my-dax-subnets");
    assert_eq!(sg.description, "DAX subnet group for test");
    assert_eq!(sg.subnet_ids, vec!["subnet-aaa111", "subnet-bbb222"]);
    assert_eq!(sg.vpc_id, Some("vpc-12345".to_string()));

    // Verify extract round-trip
    let extracted = converter
        .extract(&ctx)
        .await
        .expect("extract should succeed");
    assert_eq!(extracted.len(), 1);
    assert_eq!(extracted[0].attributes["name"], "my-dax-subnets");
    assert_eq!(
        extracted[0].attributes["subnet_ids"],
        json!(["subnet-aaa111", "subnet-bbb222"])
    );
    assert_eq!(extracted[0].attributes["vpc_id"], "vpc-12345");
}

// ---------------------------------------------------------------------------
// EBS Volume injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_ebs_volume() {
    let ebs = Arc::new(EbsService::new());
    let ctx = default_ctx();

    let converter = AwsEbsVolumeConverter::new(Arc::clone(&ebs));

    let mut injector = TerraformInjector::new();
    injector.register(AwsEbsVolumeConverter::new(Arc::clone(&ebs)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_ebs_volume",
        "data_vol",
        json!({
            "id": "vol-0123456789abcdef0",
            "availability_zone": "us-east-1a",
            "size": 100,
            "type": "gp3",
            "iops": 3000,
            "throughput": 125,
            "encrypted": true,
            "tags": {"Name": "data-volume"},
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    // Verify via snapshot
    let view = ebs
        .snapshot(&ctx.default_account_id, &ctx.default_region)
        .await;
    assert!(view.volumes.contains_key("vol-0123456789abcdef0"));
    let vol = &view.volumes["vol-0123456789abcdef0"];
    assert_eq!(vol.availability_zone, "us-east-1a");
    assert_eq!(vol.size, 100);
    assert_eq!(vol.volume_type, "gp3");
    assert_eq!(vol.iops, Some(3000));
    assert_eq!(vol.throughput, Some(125));
    assert!(vol.encrypted);
    assert_eq!(vol.state, "available");
    assert_eq!(vol.tags.get("Name"), Some(&"data-volume".to_string()));

    // Verify extract round-trip
    let extracted = converter
        .extract(&ctx)
        .await
        .expect("extract should succeed");
    assert_eq!(extracted.len(), 1);
    assert_eq!(
        extracted[0].attributes["volume_id"],
        "vol-0123456789abcdef0"
    );
    assert_eq!(extracted[0].attributes["size"], 100);
    assert_eq!(extracted[0].attributes["type"], "gp3");
    assert_eq!(extracted[0].attributes["encrypted"], true);
}

// ---------------------------------------------------------------------------
// EC2 Instance Connect Endpoint injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_ec2_instance_connect_endpoint() {
    let svc = Arc::new(Ec2InstanceConnectService::new());
    let ctx = default_ctx();

    let converter = AwsEc2InstanceConnectEndpointConverter::new(Arc::clone(&svc));

    let mut injector = TerraformInjector::new();
    injector.register(AwsEc2InstanceConnectEndpointConverter::new(Arc::clone(
        &svc,
    )));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_ec2_instance_connect_endpoint",
        "test_ep",
        json!({
            "id": "eice-0123456789abcdef0",
            "subnet_id": "subnet-abc123",
            "vpc_id": "vpc-xyz789",
            "security_group_ids": ["sg-11111"],
            "state": "create-complete",
            "owner_id": "123456789012",
            "availability_zone": "us-east-1a",
            "created_at": "2024-06-01T12:00:00Z",
            "tags": {"Purpose": "testing"},
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    // Verify via snapshot
    let view = svc
        .snapshot(&ctx.default_account_id, &ctx.default_region)
        .await;
    assert!(view.endpoints.contains_key("eice-0123456789abcdef0"));
    let ep = &view.endpoints["eice-0123456789abcdef0"];
    assert_eq!(ep.subnet_id, "subnet-abc123");
    assert_eq!(ep.vpc_id, "vpc-xyz789");
    assert_eq!(ep.security_group_ids, vec!["sg-11111"]);
    assert_eq!(ep.state, "create-complete");
    assert_eq!(ep.owner_id, "123456789012");
    assert_eq!(ep.availability_zone, "us-east-1a");
    assert_eq!(ep.tags.get("Purpose"), Some(&"testing".to_string()));

    // Verify extract round-trip
    let extracted = converter
        .extract(&ctx)
        .await
        .expect("extract should succeed");
    assert_eq!(extracted.len(), 1);
    assert_eq!(
        extracted[0].attributes["instance_connect_endpoint_id"],
        "eice-0123456789abcdef0"
    );
    assert_eq!(extracted[0].attributes["subnet_id"], "subnet-abc123");
    assert_eq!(extracted[0].attributes["vpc_id"], "vpc-xyz789");
}

// ---------------------------------------------------------------------------
// MemoryDB ACL injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_memorydb_acl() {
    let memorydb = Arc::new(MemoryDbService::new());
    let ctx = default_ctx();

    let converter = AwsMemoryDbAclConverter::new(Arc::clone(&memorydb));

    let mut injector = TerraformInjector::new();
    injector.register(AwsMemoryDbAclConverter::new(Arc::clone(&memorydb)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_memorydb_acl",
        "test_acl",
        json!({
            "name": "my-acl",
            "user_names": ["default", "admin-user"],
            "arn": "arn:aws:memorydb:us-east-1:123456789012:acl/my-acl",
            "minimum_engine_version": "7.0",
            "tags": {"Team": "backend"},
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    // Verify via snapshot
    let view = memorydb
        .snapshot(&ctx.default_account_id, &ctx.default_region)
        .await;
    assert!(view.acls.contains_key("my-acl"));
    let acl = &view.acls["my-acl"];
    assert_eq!(acl.acl_name, "my-acl");
    assert_eq!(acl.user_names, vec!["default", "admin-user"]);
    assert_eq!(acl.minimum_engine_version, "7.0");
    assert_eq!(acl.status, "active");
    assert_eq!(
        acl.arn,
        "arn:aws:memorydb:us-east-1:123456789012:acl/my-acl"
    );

    // Verify extract round-trip
    let extracted = converter
        .extract(&ctx)
        .await
        .expect("extract should succeed");
    assert_eq!(extracted.len(), 1);
    assert_eq!(extracted[0].attributes["name"], "my-acl");
    assert_eq!(
        extracted[0].attributes["user_names"],
        json!(["default", "admin-user"])
    );
    assert_eq!(extracted[0].attributes["minimum_engine_version"], "7.0");
}

// ---------------------------------------------------------------------------
// Pinpoint Email Channel injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_pinpoint_email_channel() {
    let pinpoint = Arc::new(PinpointService::new());
    let ctx = default_ctx();

    let email_converter = AwsPinpointEmailChannelConverter::new(Arc::clone(&pinpoint));

    let mut injector = TerraformInjector::new();
    injector.register(AwsPinpointAppConverter::new(Arc::clone(&pinpoint)));
    injector.register(AwsPinpointEmailChannelConverter::new(Arc::clone(&pinpoint)));

    let tfstate = make_tfstate(vec![
        make_resource(
            "aws_pinpoint_app",
            "my_app",
            json!({
                "application_id": "abc123def456",
                "name": "my-pinpoint-app",
                "arn": "arn:aws:mobiletargeting:us-east-1:123456789012:apps/abc123def456",
            }),
        ),
        make_resource(
            "aws_pinpoint_email_channel",
            "email",
            json!({
                "application_id": "abc123def456",
                "enabled": true,
                "from_address": "noreply@example.com",
                "identity": "arn:aws:ses:us-east-1:123456789012:identity/example.com",
                "role_arn": "arn:aws:iam::123456789012:role/pinpoint-ses",
            }),
        ),
    ]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 2);

    // Verify via snapshot
    let view = pinpoint
        .snapshot(&ctx.default_account_id, &ctx.default_region)
        .await;
    assert!(view.apps.contains_key("abc123def456"));
    assert!(view.email_channels.contains_key("abc123def456"));
    let ec = &view.email_channels["abc123def456"];
    assert!(ec.enabled);
    assert_eq!(ec.from_address, "noreply@example.com");
    assert_eq!(
        ec.identity,
        "arn:aws:ses:us-east-1:123456789012:identity/example.com"
    );
    assert_eq!(
        ec.role_arn,
        Some("arn:aws:iam::123456789012:role/pinpoint-ses".to_string())
    );

    // Verify extract round-trip for email channel
    let extracted = email_converter
        .extract(&ctx)
        .await
        .expect("extract should succeed");
    assert_eq!(extracted.len(), 1);
    assert_eq!(extracted[0].attributes["application_id"], "abc123def456");
    assert_eq!(extracted[0].attributes["enabled"], true);
    assert_eq!(
        extracted[0].attributes["from_address"],
        "noreply@example.com"
    );
}

// ---------------------------------------------------------------------------
// ServiceCatalog Product injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_servicecatalog_product() {
    let sc = Arc::new(ServiceCatalogService::new());
    let ctx = default_ctx();

    let converter = AwsServicecatalogProductConverter::new(Arc::clone(&sc));

    let mut injector = TerraformInjector::new();
    injector.register(AwsServicecatalogProductConverter::new(Arc::clone(&sc)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_servicecatalog_product",
        "test_product",
        json!({
            "id": "prod-abc123xyz456",
            "name": "My CloudFormation Product",
            "owner": "Platform Team",
            "description": "A test product for validation",
            "type": "CLOUD_FORMATION_TEMPLATE",
            "distributor": "ACME Corp",
            "support_email": "support@example.com",
            "tags": {"Env": "staging"},
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    // Verify via snapshot
    let view = sc
        .snapshot(&ctx.default_account_id, &ctx.default_region)
        .await;
    assert!(view.products.contains_key("prod-abc123xyz456"));
    let product = &view.products["prod-abc123xyz456"];
    assert_eq!(product.name, "My CloudFormation Product");
    assert_eq!(product.owner, "Platform Team");
    assert_eq!(
        product.description,
        Some("A test product for validation".to_string())
    );
    assert_eq!(product.product_type, "CLOUD_FORMATION_TEMPLATE");
    assert_eq!(product.distributor, Some("ACME Corp".to_string()));
    assert_eq!(
        product.support_email,
        Some("support@example.com".to_string())
    );

    // Verify extract round-trip
    let extracted = converter
        .extract(&ctx)
        .await
        .expect("extract should succeed");
    assert_eq!(extracted.len(), 1);
    assert_eq!(extracted[0].attributes["name"], "My CloudFormation Product");
    assert_eq!(extracted[0].attributes["owner"], "Platform Team");
    assert_eq!(extracted[0].attributes["type"], "CLOUD_FORMATION_TEMPLATE");
    assert_eq!(extracted[0].attributes["distributor"], "ACME Corp");
}

// ---------------------------------------------------------------------------
// FSx Lustre File System injection tests (with expanded fields)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_fsx_with_expanded_fields() {
    let fsx = Arc::new(FsxService::new());
    let ctx = default_ctx();

    let converter = AwsFsxLustreFileSystemConverter::new(Arc::clone(&fsx));

    let mut injector = TerraformInjector::new();
    injector.register(AwsFsxLustreFileSystemConverter::new(Arc::clone(&fsx)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_fsx_lustre_file_system",
        "lustre_fs",
        json!({
            "id": "fs-lustre-test1234",
            "storage_capacity": 2400,
            "storage_type": "SSD",
            "subnet_ids": ["subnet-lustre-1"],
            "security_group_ids": ["sg-lustre-1"],
            "creation_time": "2024-03-15T10:30:00Z",
            "lifecycle": "AVAILABLE",
            "deployment_type": "PERSISTENT_2",
            "vpc_id": "vpc-lustre-abc",
            "owner_id": "123456789012",
            "copy_tags_to_backups": true,
            "automatic_backup_retention_days": 7,
            "weekly_maintenance_start_time": "1:05:00",
            "tags": {"Project": "data-lake"},
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    // Verify via snapshot
    let view = fsx
        .snapshot(&ctx.default_account_id, &ctx.default_region)
        .await;
    assert!(view.file_systems.contains_key("fs-lustre-test1234"));
    let fs = &view.file_systems["fs-lustre-test1234"];
    assert_eq!(fs.file_system_type, "LUSTRE");
    assert_eq!(fs.storage_capacity, 2400);
    assert_eq!(fs.creation_time, Some("2024-03-15T10:30:00Z".to_string()));
    assert_eq!(fs.lifecycle, "AVAILABLE");
    assert_eq!(fs.deployment_type, Some("PERSISTENT_2".to_string()));
    assert_eq!(fs.vpc_id, Some("vpc-lustre-abc".to_string()));
    assert_eq!(fs.owner_id, Some("123456789012".to_string()));
    assert!(fs.copy_tags_to_backups);
    assert_eq!(fs.automatic_backup_retention_days, 7);
    assert_eq!(
        fs.weekly_maintenance_start_time,
        Some("1:05:00".to_string())
    );

    // Verify extract round-trip
    let extracted = converter
        .extract(&ctx)
        .await
        .expect("extract should succeed");
    assert_eq!(extracted.len(), 1);
    assert_eq!(extracted[0].attributes["id"], "fs-lustre-test1234");
    assert_eq!(extracted[0].attributes["storage_capacity"], 2400);
    assert_eq!(
        extracted[0].attributes["creation_time"],
        "2024-03-15T10:30:00Z"
    );
    assert_eq!(extracted[0].attributes["lifecycle"], "AVAILABLE");
    assert_eq!(extracted[0].attributes["deployment_type"], "PERSISTENT_2");
    assert_eq!(extracted[0].attributes["copy_tags_to_backups"], true);
    assert_eq!(
        extracted[0].attributes["automatic_backup_retention_days"],
        7
    );
}

// ---------------------------------------------------------------------------
// SageMaker Domain injection tests (with VPC fields)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_sagemaker_domain_with_vpc() {
    let sagemaker = Arc::new(SageMakerService::new());
    let ctx = default_ctx();

    let converter = AwsSagemakerDomainConverter::new(Arc::clone(&sagemaker));

    let mut injector = TerraformInjector::new();
    injector.register(AwsSagemakerDomainConverter::new(Arc::clone(&sagemaker)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_sagemaker_domain",
        "ml_domain",
        json!({
            "id": "d-mltest1234",
            "domain_name": "ml-workspace",
            "arn": "arn:aws:sagemaker:us-east-1:123456789012:domain/d-mltest1234",
            "vpc_id": "vpc-ml-12345",
            "subnet_ids": ["subnet-ml-1", "subnet-ml-2"],
            "auth_mode": "IAM",
            "app_network_access_type": "VpcOnly",
            "kms_key_id": "arn:aws:kms:us-east-1:123456789012:key/mrk-abc",
            "security_group_ids": ["sg-ml-1"],
            "tags": {"Team": "data-science"},
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    // Verify via snapshot
    let view = sagemaker
        .snapshot(&ctx.default_account_id, &ctx.default_region)
        .await;
    assert!(view.domains.contains_key("d-mltest1234"));
    let domain = &view.domains["d-mltest1234"];
    assert_eq!(domain.domain_name, "ml-workspace");
    assert_eq!(domain.vpc_id, Some("vpc-ml-12345".to_string()));
    assert_eq!(domain.subnet_ids, vec!["subnet-ml-1", "subnet-ml-2"]);
    assert_eq!(domain.auth_mode, Some("IAM".to_string()));
    assert_eq!(domain.app_network_access_type, Some("VpcOnly".to_string()));
    assert_eq!(
        domain.kms_key_id,
        Some("arn:aws:kms:us-east-1:123456789012:key/mrk-abc".to_string())
    );
    assert_eq!(domain.security_group_ids, vec!["sg-ml-1"]);
    assert_eq!(domain.status, "InService");

    // Verify extract round-trip
    let extracted = converter
        .extract(&ctx)
        .await
        .expect("extract should succeed");
    assert_eq!(extracted.len(), 1);
    assert_eq!(extracted[0].attributes["domain_name"], "ml-workspace");
    assert_eq!(extracted[0].attributes["vpc_id"], "vpc-ml-12345");
    assert_eq!(
        extracted[0].attributes["subnet_ids"],
        json!(["subnet-ml-1", "subnet-ml-2"])
    );
    assert_eq!(extracted[0].attributes["auth_mode"], "IAM");
    assert_eq!(
        extracted[0].attributes["app_network_access_type"],
        "VpcOnly"
    );
}

// ---------------------------------------------------------------------------
// Elastic Beanstalk Environment injection tests (with new fields)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_beanstalk_environment_with_cname() {
    let eb = Arc::new(ElasticBeanstalkService::new());
    let ctx = default_ctx();

    let converter = AwsElasticBeanstalkEnvironmentConverter::new(Arc::clone(&eb));

    let mut injector = TerraformInjector::new();
    injector.register(AwsElasticBeanstalkEnvironmentConverter::new(Arc::clone(
        &eb,
    )));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_elastic_beanstalk_environment",
        "prod_env",
        json!({
            "id": "e-prodenv123",
            "name": "my-prod-env",
            "application": "my-beanstalk-app",
            "platform_arn": "arn:aws:elasticbeanstalk:us-east-1::platform/Python 3.9 running on 64bit Amazon Linux 2/3.5.0",
            "cname": "my-prod-env.us-east-1.elasticbeanstalk.com",
            "version_label": "v1.2.3",
            "endpoint_url": "awseb-e-prod-12345.us-east-1.elb.amazonaws.com",
            "solution_stack_name": "64bit Amazon Linux 2 v3.5.0 running Python 3.9",
            "tier": "WebServer",
            "tags": {"Release": "v1.2.3"},
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    // Verify via snapshot
    let view = eb
        .snapshot(&ctx.default_account_id, &ctx.default_region)
        .await;
    assert!(view.environments.contains_key("my-prod-env"));
    let env = &view.environments["my-prod-env"];
    assert_eq!(env.environment_name, "my-prod-env");
    assert_eq!(env.application_name, "my-beanstalk-app");
    assert_eq!(env.environment_id, "e-prodenv123");
    assert_eq!(
        env.platform_arn,
        Some("arn:aws:elasticbeanstalk:us-east-1::platform/Python 3.9 running on 64bit Amazon Linux 2/3.5.0".to_string())
    );
    assert_eq!(
        env.cname,
        Some("my-prod-env.us-east-1.elasticbeanstalk.com".to_string())
    );
    assert_eq!(env.version_label, Some("v1.2.3".to_string()));
    assert_eq!(
        env.endpoint_url,
        Some("awseb-e-prod-12345.us-east-1.elb.amazonaws.com".to_string())
    );
    assert_eq!(
        env.solution_stack_name,
        Some("64bit Amazon Linux 2 v3.5.0 running Python 3.9".to_string())
    );
    assert_eq!(env.status, "Ready");
    assert_eq!(env.tier_name, "WebServer");

    // Verify extract round-trip
    let extracted = converter
        .extract(&ctx)
        .await
        .expect("extract should succeed");
    assert_eq!(extracted.len(), 1);
    assert_eq!(extracted[0].attributes["name"], "my-prod-env");
    assert_eq!(extracted[0].attributes["application"], "my-beanstalk-app");
    assert_eq!(
        extracted[0].attributes["platform_arn"],
        "arn:aws:elasticbeanstalk:us-east-1::platform/Python 3.9 running on 64bit Amazon Linux 2/3.5.0"
    );
    assert_eq!(
        extracted[0].attributes["cname"],
        "my-prod-env.us-east-1.elasticbeanstalk.com"
    );
    assert_eq!(extracted[0].attributes["version_label"], "v1.2.3");
    assert_eq!(
        extracted[0].attributes["endpoint_url"],
        "awseb-e-prod-12345.us-east-1.elb.amazonaws.com"
    );
}

// ---------------------------------------------------------------------------
// VPC Lattice injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_vpclattice_service() {
    use winterbaume_terraform::converters::vpclattice::AwsVpcLatticeServiceConverter;
    use winterbaume_vpclattice::VpcLatticeService;
    let svc = Arc::new(VpcLatticeService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsVpcLatticeServiceConverter::new(Arc::clone(&svc)));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_vpclattice_service",
        "my_svc",
        json!({
            "name": "my-lattice-svc",
            "arn": "arn:aws:vpc-lattice:us-east-1:123456789012:service/svc-abc123",
            "auth_type": "AWS_IAM",
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert_eq!(view.services.len(), 1);
    let s = view.services.values().next().unwrap();
    assert_eq!(s.name, "my-lattice-svc");
    assert_eq!(
        s.arn,
        "arn:aws:vpc-lattice:us-east-1:123456789012:service/svc-abc123"
    );
    assert_eq!(s.auth_type, "AWS_IAM");
}

#[tokio::test]
async fn test_inject_vpclattice_service_network() {
    use winterbaume_terraform::converters::vpclattice::AwsVpcLatticeServiceNetworkConverter;
    use winterbaume_vpclattice::VpcLatticeService;
    let svc = Arc::new(VpcLatticeService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsVpcLatticeServiceNetworkConverter::new(Arc::clone(&svc)));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_vpclattice_service_network",
        "my_sn",
        json!({
            "name": "my-service-network",
            "auth_type": "NONE",
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert_eq!(view.service_networks.len(), 1);
    let sn = view.service_networks.values().next().unwrap();
    assert_eq!(sn.name, "my-service-network");
    assert_eq!(sn.auth_type, "NONE");
}

#[tokio::test]
async fn test_inject_vpclattice_target_group() {
    use winterbaume_terraform::converters::vpclattice::AwsVpcLatticeTargetGroupConverter;
    use winterbaume_vpclattice::VpcLatticeService;
    let svc = Arc::new(VpcLatticeService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsVpcLatticeTargetGroupConverter::new(Arc::clone(&svc)));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_vpclattice_target_group",
        "my_tg",
        json!({
            "name": "my-target-group",
            "type": "IP",
            "config": [{
                "port": 80,
                "protocol": "HTTP",
                "vpc_identifier": "vpc-12345",
            }],
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert_eq!(view.target_groups.len(), 1);
    let tg = view.target_groups.values().next().unwrap();
    assert_eq!(tg.name, "my-target-group");
    assert_eq!(tg.target_group_type, "IP");
    assert_eq!(tg.config_port, Some(80));
    assert_eq!(tg.config_protocol.as_deref(), Some("HTTP"));
    assert_eq!(tg.config_vpc_identifier.as_deref(), Some("vpc-12345"));
}

// ---------------------------------------------------------------------------
// Route53 Resolver injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_route53_resolver_endpoint() {
    use winterbaume_route53resolver::Route53ResolverService;
    use winterbaume_terraform::converters::route53resolver::AwsRoute53ResolverEndpointConverter;
    let svc = Arc::new(Route53ResolverService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsRoute53ResolverEndpointConverter::new(Arc::clone(&svc)));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_route53_resolver_endpoint",
        "my_ep",
        json!({
            "direction": "INBOUND",
            "security_group_ids": ["sg-111", "sg-222"],
            "ip_address": [
                { "subnet_id": "subnet-aaa", "ip": "10.0.0.10" },
                { "subnet_id": "subnet-bbb", "ip": "10.0.1.10" },
            ],
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert_eq!(view.endpoints.len(), 1);
    let ep = view.endpoints.values().next().unwrap();
    assert_eq!(ep.direction, "INBOUND");
    assert_eq!(ep.security_group_ids, vec!["sg-111", "sg-222"]);
    assert_eq!(ep.ip_addresses.len(), 2);
    assert_eq!(ep.ip_addresses[0].subnet_id, "subnet-aaa");
    assert_eq!(ep.ip_addresses[0].ip.as_deref(), Some("10.0.0.10"));
    assert_eq!(ep.ip_addresses[1].subnet_id, "subnet-bbb");
    assert_eq!(ep.ip_addresses[1].ip.as_deref(), Some("10.0.1.10"));
}

#[tokio::test]
async fn test_inject_route53_resolver_rule() {
    use winterbaume_route53resolver::Route53ResolverService;
    use winterbaume_terraform::converters::route53resolver::AwsRoute53ResolverRuleConverter;
    let svc = Arc::new(Route53ResolverService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsRoute53ResolverRuleConverter::new(Arc::clone(&svc)));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_route53_resolver_rule",
        "my_rule",
        json!({
            "domain_name": "example.com.",
            "rule_type": "FORWARD",
            "target_ip": [
                { "ip": "10.0.0.2", "port": 53 },
                { "ip": "10.0.1.2", "port": 53 },
            ],
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert_eq!(view.resolver_rules.len(), 1);
    let rule = view.resolver_rules.values().next().unwrap();
    assert_eq!(rule.domain_name, "example.com.");
    assert_eq!(rule.rule_type, "FORWARD");
    assert_eq!(rule.target_ips.len(), 2);
    assert_eq!(rule.target_ips[0].ip.as_deref(), Some("10.0.0.2"));
    assert_eq!(rule.target_ips[0].port, Some(53));
    assert_eq!(rule.target_ips[1].ip.as_deref(), Some("10.0.1.2"));
    assert_eq!(rule.target_ips[1].port, Some(53));
}

// ---------------------------------------------------------------------------
// Network Firewall injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_networkfirewall_firewall() {
    use winterbaume_networkfirewall::NetworkFirewallService;
    use winterbaume_terraform::converters::networkfirewall::AwsNetworkFirewallFirewallConverter;
    let svc = Arc::new(NetworkFirewallService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsNetworkFirewallFirewallConverter::new(Arc::clone(&svc)));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_networkfirewall_firewall",
        "my_fw",
        json!({
            "name": "my-firewall",
            "vpc_id": "vpc-99999",
            "firewall_policy_arn": "arn:aws:network-firewall:us-east-1:123456789012:firewall-policy/my-policy",
            "subnet_mapping": [
                { "subnet_id": "subnet-aaa" },
                { "subnet_id": "subnet-bbb" },
            ],
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert_eq!(view.firewalls.len(), 1);
    let fw = view.firewalls.values().next().unwrap();
    assert_eq!(fw.firewall_name, "my-firewall");
    assert_eq!(fw.vpc_id, "vpc-99999");
    assert_eq!(
        fw.firewall_policy_arn,
        "arn:aws:network-firewall:us-east-1:123456789012:firewall-policy/my-policy"
    );
    assert_eq!(fw.subnet_mappings.len(), 2);
    assert_eq!(fw.subnet_mappings[0].subnet_id, "subnet-aaa");
    assert_eq!(fw.subnet_mappings[1].subnet_id, "subnet-bbb");
}

// ---------------------------------------------------------------------------
// Transfer Family injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_transfer_server() {
    use winterbaume_terraform::converters::transfer::AwsTransferServerConverter;
    use winterbaume_transfer::TransferService;
    let svc = Arc::new(TransferService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsTransferServerConverter::new(Arc::clone(&svc)));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_transfer_server",
        "my_server",
        json!({
            "id": "s-12345abcde",
            "protocols": ["SFTP", "FTPS"],
            "identity_provider_type": "API_GATEWAY",
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.servers.contains_key("s-12345abcde"));
    let srv = &view.servers["s-12345abcde"];
    assert_eq!(srv.protocols, vec!["SFTP", "FTPS"]);
    assert_eq!(srv.identity_provider_type, "API_GATEWAY");
}

#[tokio::test]
async fn test_inject_transfer_user() {
    use winterbaume_terraform::converters::transfer::{
        AwsTransferServerConverter, AwsTransferUserConverter,
    };
    use winterbaume_transfer::TransferService;
    let svc = Arc::new(TransferService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsTransferServerConverter::new(Arc::clone(&svc)));
    injector.register(AwsTransferUserConverter::new(Arc::clone(&svc)));
    let tfstate = make_tfstate(vec![
        make_resource(
            "aws_transfer_server",
            "my_server",
            json!({
                "id": "s-srv001",
                "protocols": ["SFTP"],
            }),
        ),
        make_resource(
            "aws_transfer_user",
            "my_user",
            json!({
                "server_id": "s-srv001",
                "user_name": "testuser",
                "role": "arn:aws:iam::123456789012:role/transfer-role",
            }),
        ),
    ]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 2);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.servers.contains_key("s-srv001"));
    let srv = &view.servers["s-srv001"];
    assert!(srv.users.contains_key("testuser"));
    let user = &srv.users["testuser"];
    assert_eq!(user.user_name, "testuser");
    assert_eq!(user.server_id, "s-srv001");
    assert_eq!(user.role, "arn:aws:iam::123456789012:role/transfer-role");
}

// ---------------------------------------------------------------------------
// Direct Connect injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_dx_connection() {
    use winterbaume_directconnect::DirectConnectService;
    use winterbaume_terraform::converters::directconnect::AwsDxConnectionConverter;
    let svc = Arc::new(DirectConnectService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsDxConnectionConverter::new(Arc::clone(&svc)));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_dx_connection",
        "my_conn",
        json!({
            "id": "dxcon-abc123",
            "name": "my-dx-connection",
            "bandwidth": "10Gbps",
            "location": "EqSe2",
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.connections.contains_key("dxcon-abc123"));
    let conn = &view.connections["dxcon-abc123"];
    assert_eq!(conn.connection_name, "my-dx-connection");
    assert_eq!(conn.bandwidth, "10Gbps");
    assert_eq!(conn.location, "EqSe2");
}

// ---------------------------------------------------------------------------
// Network Manager injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_networkmanager_global_network() {
    use winterbaume_networkmanager::NetworkManagerService;
    use winterbaume_terraform::converters::networkmanager::AwsNetworkmanagerGlobalNetworkConverter;
    let svc = Arc::new(NetworkManagerService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsNetworkmanagerGlobalNetworkConverter::new(Arc::clone(
        &svc,
    )));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_networkmanager_global_network",
        "my_gn",
        json!({
            "id": "global-network-001",
            "description": "My global network",
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.global_networks.contains_key("global-network-001"));
    let gn = &view.global_networks["global-network-001"];
    assert_eq!(gn.description, "My global network");
}

// ---------------------------------------------------------------------------
// Service Discovery injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_service_discovery_namespace() {
    use winterbaume_servicediscovery::ServiceDiscoveryService;
    use winterbaume_terraform::converters::servicediscovery::AwsServiceDiscoveryPrivateDnsNamespaceConverter;
    let svc = Arc::new(ServiceDiscoveryService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsServiceDiscoveryPrivateDnsNamespaceConverter::new(
        Arc::clone(&svc),
    ));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_service_discovery_private_dns_namespace",
        "my_ns",
        json!({
            "name": "local.example",
            "vpc": "vpc-12345",
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert_eq!(view.namespaces.len(), 1);
    let ns = view.namespaces.values().next().unwrap();
    assert_eq!(ns.name, "local.example");
    assert_eq!(ns.vpc.as_deref(), Some("vpc-12345"));
    assert_eq!(ns.namespace_type, "DNS_PRIVATE");
}

// ---------------------------------------------------------------------------
// API Gateway injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_api_gateway_rest_api() {
    let service = Arc::new(ApiGatewayService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsApiGatewayRestApiConverter::new(Arc::clone(&service)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_api_gateway_rest_api",
        "my_api",
        json!({
            "id": "abc123",
            "name": "my-rest-api",
            "description": "A test REST API",
            "endpoint_configuration": [
                {
                    "types": ["REGIONAL"],
                    "vpc_endpoint_ids": []
                }
            ]
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    let view = service.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.rest_apis.contains_key("abc123"));
    let api = &view.rest_apis["abc123"];
    assert_eq!(api.name, "my-rest-api");
    assert_eq!(api.description, Some("A test REST API".to_string()));
    assert_eq!(api.endpoint_types, vec!["REGIONAL".to_string()]);
}

#[tokio::test]
async fn test_inject_api_gateway_stage() {
    let service = Arc::new(ApiGatewayService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsApiGatewayRestApiConverter::new(Arc::clone(&service)));
    injector.register(AwsApiGatewayStageConverter::new(Arc::clone(&service)));

    let tfstate = make_tfstate(vec![
        make_resource(
            "aws_api_gateway_rest_api",
            "my_api",
            json!({
                "id": "api-001",
                "name": "stage-test-api",
            }),
        ),
        make_resource(
            "aws_api_gateway_stage",
            "my_stage",
            json!({
                "rest_api_id": "api-001",
                "stage_name": "prod",
                "deployment_id": "deploy-001",
                "description": "Production stage",
                "xray_tracing_enabled": true,
                "cache_cluster_enabled": false,
            }),
        ),
    ]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 2);

    let view = service.snapshot(&ctx.default_account_id, "us-east-1").await;
    let stage_key = "api-001/prod";
    assert!(view.stages.contains_key(stage_key));
    let stage = &view.stages[stage_key];
    assert_eq!(stage.rest_api_id, "api-001");
    assert_eq!(stage.stage_name, "prod");
    assert_eq!(stage.deployment_id, "deploy-001");
    assert_eq!(stage.description, Some("Production stage".to_string()));
    assert_eq!(stage.tracing_enabled, Some(true));
    assert_eq!(stage.cache_cluster_enabled, Some(false));
}

// ---------------------------------------------------------------------------
// AppConfig injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_appconfig_application() {
    let service = Arc::new(AppConfigService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsAppconfigApplicationConverter::new(Arc::clone(&service)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_appconfig_application",
        "my_app",
        json!({
            "id": "app-001",
            "name": "my-appconfig-app",
            "description": "A test AppConfig application",
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    let view = service.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.applications.contains_key("app-001"));
    let app = &view.applications["app-001"];
    assert_eq!(app.name, "my-appconfig-app");
    assert_eq!(app.description, "A test AppConfig application");
}

#[tokio::test]
async fn test_inject_appconfig_environment() {
    let service = Arc::new(AppConfigService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsAppconfigApplicationConverter::new(Arc::clone(&service)));
    injector.register(AwsAppconfigEnvironmentConverter::new(Arc::clone(&service)));

    let tfstate = make_tfstate(vec![
        make_resource(
            "aws_appconfig_application",
            "my_app",
            json!({
                "id": "app-100",
                "name": "env-test-app",
                "description": "App for env test",
            }),
        ),
        make_resource(
            "aws_appconfig_environment",
            "my_env",
            json!({
                "environment_id": "env-200",
                "application_id": "app-100",
                "name": "staging",
                "description": "Staging environment",
            }),
        ),
    ]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 2);

    let view = service.snapshot(&ctx.default_account_id, "us-east-1").await;
    let env_key = "app-100/env-200";
    assert!(view.environments.contains_key(env_key));
    let env = &view.environments[env_key];
    assert_eq!(env.application_id, "app-100");
    assert_eq!(env.name, "staging");
    assert_eq!(env.description, "Staging environment");
}

#[tokio::test]
async fn test_inject_appconfig_deployment_strategy() {
    let service = Arc::new(AppConfigService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsAppconfigDeploymentStrategyConverter::new(Arc::clone(
        &service,
    )));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_appconfig_deployment_strategy",
        "my_strategy",
        json!({
            "id": "ds-001",
            "name": "my-linear-strategy",
            "description": "Linear 10% every 10 min",
            "deployment_duration_in_minutes": 20,
            "final_bake_time_in_minutes": 10,
            "growth_factor": 10.0,
            "growth_type": "LINEAR",
            "replicate_to": "NONE",
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    let view = service.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.deployment_strategies.contains_key("ds-001"));
    let ds = &view.deployment_strategies["ds-001"];
    assert_eq!(ds.name, "my-linear-strategy");
    assert_eq!(ds.deployment_duration_in_minutes, 20);
    assert_eq!(ds.final_bake_time_in_minutes, 10);
    assert!((ds.growth_factor - 10.0).abs() < 0.01);
    assert_eq!(ds.growth_type, "LINEAR");
    assert_eq!(ds.replicate_to, "NONE");
}

// ---------------------------------------------------------------------------
// Auto Scaling injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_autoscaling_group() {
    let service = Arc::new(AutoScalingService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsAutoscalingGroupConverter::new(Arc::clone(&service)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_autoscaling_group",
        "my_asg",
        json!({
            "name": "my-asg",
            "min_size": 1,
            "max_size": 5,
            "desired_capacity": 2,
            "launch_configuration": "my-lc",
            "availability_zones": ["us-east-1a", "us-east-1b"],
            "health_check_type": "ELB",
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    let view = service.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.groups.contains_key("my-asg"));
    let asg = &view.groups["my-asg"];
    assert_eq!(asg.min_size, 1);
    assert_eq!(asg.max_size, 5);
    assert_eq!(asg.desired_capacity, 2);
    assert_eq!(asg.launch_configuration_name, Some("my-lc".to_string()));
    assert_eq!(asg.health_check_type, "ELB");
    assert_eq!(
        asg.availability_zones,
        vec!["us-east-1a".to_string(), "us-east-1b".to_string()]
    );
}

#[tokio::test]
async fn test_inject_autoscaling_policy() {
    let service = Arc::new(AutoScalingService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsAutoscalingGroupConverter::new(Arc::clone(&service)));
    injector.register(AwsAutoscalingPolicyConverter::new(Arc::clone(&service)));

    let tfstate = make_tfstate(vec![
        make_resource(
            "aws_autoscaling_group",
            "my_asg",
            json!({
                "name": "policy-test-asg",
                "min_size": 1,
                "max_size": 10,
                "desired_capacity": 3,
            }),
        ),
        make_resource(
            "aws_autoscaling_policy",
            "scale_up",
            json!({
                "name": "scale-up-policy",
                "autoscaling_group_name": "policy-test-asg",
                "policy_type": "SimpleScaling",
                "adjustment_type": "ChangeInCapacity",
                "scaling_adjustment": 2,
                "cooldown": 300,
            }),
        ),
    ]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 2);

    let view = service.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.policies.contains_key("scale-up-policy"));
    let pol = &view.policies["scale-up-policy"];
    assert_eq!(pol.group_name, "policy-test-asg");
    assert_eq!(pol.policy_type, Some("SimpleScaling".to_string()));
    assert_eq!(pol.adjustment_type, Some("ChangeInCapacity".to_string()));
    assert_eq!(pol.scaling_adjustment, Some(2));
    assert_eq!(pol.cooldown, Some(300));
}

// ---------------------------------------------------------------------------
// Batch injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_batch_compute_environment() {
    let service = Arc::new(BatchService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsBatchComputeEnvironmentConverter::new(Arc::clone(
        &service,
    )));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_batch_compute_environment",
        "my_ce",
        json!({
            "name": "my-compute-env",
            "type": "MANAGED",
            "state": "ENABLED",
            "service_role": "arn:aws:iam::123456789012:role/batch-service-role",
            "tags": {
                "Team": "platform"
            },
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    let view = service.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.compute_environments.contains_key("my-compute-env"));
    let ce = &view.compute_environments["my-compute-env"];
    assert_eq!(ce.compute_environment_name, "my-compute-env");
    assert_eq!(ce.ce_type, "MANAGED");
    assert_eq!(ce.state, "ENABLED");
    assert_eq!(ce.status, "VALID");
    assert_eq!(
        ce.service_role,
        Some("arn:aws:iam::123456789012:role/batch-service-role".to_string())
    );
    assert_eq!(ce.tags.get("Team").map(|s| s.as_str()), Some("platform"));
}

#[tokio::test]
async fn test_inject_batch_job_queue() {
    let service = Arc::new(BatchService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsBatchComputeEnvironmentConverter::new(Arc::clone(
        &service,
    )));
    injector.register(AwsBatchJobQueueConverter::new(Arc::clone(&service)));

    let tfstate = make_tfstate(vec![
        make_resource(
            "aws_batch_compute_environment",
            "my_ce",
            json!({
                "name": "queue-test-ce",
                "type": "MANAGED",
            }),
        ),
        make_resource(
            "aws_batch_job_queue",
            "my_queue",
            json!({
                "name": "my-job-queue",
                "state": "ENABLED",
                "priority": 10,
                "compute_environments": ["queue-test-ce"],
                "tags": {
                    "Env": "test"
                },
            }),
        ),
    ]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 2);

    let view = service.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.job_queues.contains_key("my-job-queue"));
    let jq = &view.job_queues["my-job-queue"];
    assert_eq!(jq.job_queue_name, "my-job-queue");
    assert_eq!(jq.state, "ENABLED");
    assert_eq!(jq.priority, 10);
    assert_eq!(jq.status, "VALID");
    assert_eq!(jq.tags.get("Env").map(|s| s.as_str()), Some("test"));
}

// ---------------------------------------------------------------------------
// CloudTrail injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_cloudtrail() {
    let service = Arc::new(CloudTrailService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsCloudtrailConverter::new(Arc::clone(&service)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_cloudtrail",
        "main",
        json!({
            "name": "my-trail",
            "s3_bucket_name": "my-cloudtrail-bucket",
            "is_multi_region_trail": true,
            "include_global_service_events": true,
            "enable_logging": true,
            "tags": {
                "Project": "audit"
            },
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    let view = service.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.trails.contains_key("my-trail"));
    let trail = &view.trails["my-trail"];
    assert_eq!(trail.name, "my-trail");
    assert_eq!(trail.s3_bucket_name, "my-cloudtrail-bucket");
    assert!(trail.is_multi_region_trail);
    assert!(trail.include_global_service_events);
    assert!(trail.is_logging);
    assert_eq!(trail.tags.get("Project").map(|s| s.as_str()), Some("audit"));
}

// ---------------------------------------------------------------------------
// Neptune Cluster injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_neptune_cluster() {
    let neptune = Arc::new(NeptuneService::new());
    let ctx = default_ctx();

    let converter = AwsNeptuneClusterConverter::new(Arc::clone(&neptune));

    let mut injector = TerraformInjector::new();
    injector.register(AwsNeptuneClusterConverter::new(Arc::clone(&neptune)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_neptune_cluster",
        "test_cluster",
        json!({
            "cluster_identifier": "my-neptune-cluster",
            "engine": "neptune",
            "engine_version": "1.2.0.2",
            "tags": { "Environment": "test" },
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    // Verify via snapshot
    let view = neptune
        .snapshot(&ctx.default_account_id, &ctx.default_region)
        .await;
    assert!(view.db_clusters.contains_key("my-neptune-cluster"));
    let cluster = &view.db_clusters["my-neptune-cluster"];
    assert_eq!(cluster.identifier, "my-neptune-cluster");
    assert_eq!(cluster.engine, "neptune");
    assert_eq!(cluster.engine_version, Some("1.2.0.2".to_string()));

    // Verify extract round-trip
    let extracted = converter
        .extract(&ctx)
        .await
        .expect("extract should succeed");
    assert_eq!(extracted.len(), 1);
    assert_eq!(
        extracted[0].attributes["cluster_identifier"],
        "my-neptune-cluster"
    );
    assert_eq!(extracted[0].attributes["engine"], "neptune");
}

// ---------------------------------------------------------------------------
// Neptune Subnet Group injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_neptune_subnet_group() {
    let neptune = Arc::new(NeptuneService::new());
    let ctx = default_ctx();

    let converter = AwsNeptuneSubnetGroupConverter::new(Arc::clone(&neptune));

    let mut injector = TerraformInjector::new();
    injector.register(AwsNeptuneSubnetGroupConverter::new(Arc::clone(&neptune)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_neptune_subnet_group",
        "test_sg",
        json!({
            "name": "my-neptune-subnets",
            "subnet_ids": ["subnet-aaa111", "subnet-bbb222"],
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    // Verify via snapshot
    let view = neptune
        .snapshot(&ctx.default_account_id, &ctx.default_region)
        .await;
    assert!(view.db_subnet_groups.contains_key("my-neptune-subnets"));
    let sg = &view.db_subnet_groups["my-neptune-subnets"];
    assert_eq!(sg.name, "my-neptune-subnets");
    assert_eq!(sg.subnet_ids, vec!["subnet-aaa111", "subnet-bbb222"]);

    // Verify extract round-trip
    let extracted = converter
        .extract(&ctx)
        .await
        .expect("extract should succeed");
    assert_eq!(extracted.len(), 1);
    assert_eq!(extracted[0].attributes["name"], "my-neptune-subnets");
    assert_eq!(
        extracted[0].attributes["subnet_ids"],
        json!(["subnet-aaa111", "subnet-bbb222"])
    );
}

// ---------------------------------------------------------------------------
// MemoryDB Cluster injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_memorydb_cluster() {
    let memorydb = Arc::new(MemoryDbService::new());
    let ctx = default_ctx();

    let converter = AwsMemoryDbClusterConverter::new(Arc::clone(&memorydb));

    let mut injector = TerraformInjector::new();
    injector.register(AwsMemoryDbClusterConverter::new(Arc::clone(&memorydb)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_memorydb_cluster",
        "test_cluster",
        json!({
            "name": "my-memorydb-cluster",
            "node_type": "db.r6g.xlarge",
            "num_shards": 2,
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    // Verify via snapshot
    let view = memorydb
        .snapshot(&ctx.default_account_id, &ctx.default_region)
        .await;
    assert!(view.clusters.contains_key("my-memorydb-cluster"));
    let cluster = &view.clusters["my-memorydb-cluster"];
    assert_eq!(cluster.name, "my-memorydb-cluster");
    assert_eq!(cluster.node_type, "db.r6g.xlarge");
    assert_eq!(cluster.num_shards, 2);

    // Verify extract round-trip
    let extracted = converter
        .extract(&ctx)
        .await
        .expect("extract should succeed");
    assert_eq!(extracted.len(), 1);
    assert_eq!(extracted[0].attributes["name"], "my-memorydb-cluster");
    assert_eq!(extracted[0].attributes["node_type"], "db.r6g.xlarge");
    assert_eq!(extracted[0].attributes["num_shards"], 2);
}

// ---------------------------------------------------------------------------
// MemoryDB Subnet Group injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_memorydb_subnet_group() {
    let memorydb = Arc::new(MemoryDbService::new());
    let ctx = default_ctx();

    let converter = AwsMemoryDbSubnetGroupConverter::new(Arc::clone(&memorydb));

    let mut injector = TerraformInjector::new();
    injector.register(AwsMemoryDbSubnetGroupConverter::new(Arc::clone(&memorydb)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_memorydb_subnet_group",
        "test_sg",
        json!({
            "name": "my-memorydb-subnets",
            "subnet_ids": ["subnet-111aaa", "subnet-222bbb"],
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    // Verify via snapshot
    let view = memorydb
        .snapshot(&ctx.default_account_id, &ctx.default_region)
        .await;
    assert!(view.subnet_groups.contains_key("my-memorydb-subnets"));
    let sg = &view.subnet_groups["my-memorydb-subnets"];
    assert_eq!(sg.name, "my-memorydb-subnets");
    assert_eq!(sg.subnet_ids, vec!["subnet-111aaa", "subnet-222bbb"]);

    // Verify extract round-trip
    let extracted = converter
        .extract(&ctx)
        .await
        .expect("extract should succeed");
    assert_eq!(extracted.len(), 1);
    assert_eq!(extracted[0].attributes["name"], "my-memorydb-subnets");
    assert_eq!(
        extracted[0].attributes["subnet_ids"],
        json!(["subnet-111aaa", "subnet-222bbb"])
    );
}

// ---------------------------------------------------------------------------
// MQ Broker injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_mq_broker() {
    let mq = Arc::new(MqService::new());
    let ctx = default_ctx();

    let converter = AwsMqBrokerConverter::new(Arc::clone(&mq));

    let mut injector = TerraformInjector::new();
    injector.register(AwsMqBrokerConverter::new(Arc::clone(&mq)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_mq_broker",
        "test_broker",
        json!({
            "broker_name": "my-mq-broker",
            "engine_type": "ActiveMQ",
            "deployment_mode": "SINGLE_INSTANCE",
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    // Verify via snapshot
    let view = mq
        .snapshot(&ctx.default_account_id, &ctx.default_region)
        .await;
    assert!(!view.brokers.is_empty());
    let broker = view.brokers.values().next().unwrap();
    assert_eq!(broker.broker_name, "my-mq-broker");
    assert_eq!(broker.engine_type, "ActiveMQ");
    assert_eq!(broker.deployment_mode, "SINGLE_INSTANCE");

    // Verify extract round-trip
    let extracted = converter
        .extract(&ctx)
        .await
        .expect("extract should succeed");
    assert_eq!(extracted.len(), 1);
    assert_eq!(extracted[0].attributes["broker_name"], "my-mq-broker");
    assert_eq!(extracted[0].attributes["engine_type"], "ActiveMQ");
    assert_eq!(
        extracted[0].attributes["deployment_mode"],
        "SINGLE_INSTANCE"
    );
}

// ---------------------------------------------------------------------------
// MQ Configuration injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_mq_configuration() {
    let mq = Arc::new(MqService::new());
    let ctx = default_ctx();

    let converter = AwsMqConfigurationConverter::new(Arc::clone(&mq));

    let mut injector = TerraformInjector::new();
    injector.register(AwsMqConfigurationConverter::new(Arc::clone(&mq)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_mq_configuration",
        "test_config",
        json!({
            "name": "my-mq-config",
            "engine_type": "ActiveMQ",
            "engine_version": "5.17.6",
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    // Verify via snapshot
    let view = mq
        .snapshot(&ctx.default_account_id, &ctx.default_region)
        .await;
    assert!(!view.configurations.is_empty());
    let config = view.configurations.values().next().unwrap();
    assert_eq!(config.name, "my-mq-config");
    assert_eq!(config.engine_type, "ActiveMQ");
    assert_eq!(config.engine_version, "5.17.6");

    // Verify extract round-trip
    let extracted = converter
        .extract(&ctx)
        .await
        .expect("extract should succeed");
    assert_eq!(extracted.len(), 1);
    assert_eq!(extracted[0].attributes["name"], "my-mq-config");
    assert_eq!(extracted[0].attributes["engine_type"], "ActiveMQ");
    assert_eq!(extracted[0].attributes["engine_version"], "5.17.6");
}

// ---------------------------------------------------------------------------
// Glacier Vault injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_glacier_vault() {
    let glacier = Arc::new(GlacierService::new());
    let ctx = default_ctx();

    let converter = AwsGlacierVaultConverter::new(Arc::clone(&glacier));

    let mut injector = TerraformInjector::new();
    injector.register(AwsGlacierVaultConverter::new(Arc::clone(&glacier)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_glacier_vault",
        "test_vault",
        json!({
            "name": "my-glacier-vault",
            "tags": { "Project": "archive" },
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    // Verify via snapshot
    let view = glacier
        .snapshot(&ctx.default_account_id, &ctx.default_region)
        .await;
    assert!(view.vaults.contains_key("my-glacier-vault"));
    let vault = &view.vaults["my-glacier-vault"];
    assert_eq!(vault.vault_name, "my-glacier-vault");
    assert_eq!(vault.tags.get("Project"), Some(&"archive".to_string()));

    // Verify extract round-trip
    let extracted = converter
        .extract(&ctx)
        .await
        .expect("extract should succeed");
    assert_eq!(extracted.len(), 1);
    assert_eq!(extracted[0].attributes["name"], "my-glacier-vault");
}

// ---------------------------------------------------------------------------
// DAX Cluster injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_dax_cluster() {
    let dax = Arc::new(DaxService::new());
    let ctx = default_ctx();

    let converter = AwsDaxClusterConverter::new(Arc::clone(&dax));

    let mut injector = TerraformInjector::new();
    injector.register(AwsDaxClusterConverter::new(Arc::clone(&dax)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_dax_cluster",
        "test_cluster",
        json!({
            "cluster_name": "my-dax-cluster",
            "node_type": "dax.r4.large",
            "replication_factor": 3,
            "iam_role_arn": "arn:aws:iam::123456789012:role/dax-role",
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    // Verify via snapshot
    let view = dax
        .snapshot(&ctx.default_account_id, &ctx.default_region)
        .await;
    assert!(view.clusters.contains_key("my-dax-cluster"));
    let cluster = &view.clusters["my-dax-cluster"];
    assert_eq!(cluster.cluster_name, "my-dax-cluster");
    assert_eq!(cluster.node_type, "dax.r4.large");
    assert_eq!(cluster.replication_factor, 3);

    // Verify extract round-trip
    let extracted = converter
        .extract(&ctx)
        .await
        .expect("extract should succeed");
    assert_eq!(extracted.len(), 1);
    assert_eq!(extracted[0].attributes["cluster_name"], "my-dax-cluster");
    assert_eq!(extracted[0].attributes["node_type"], "dax.r4.large");
    assert_eq!(extracted[0].attributes["replication_factor"], 3);
}

// ---------------------------------------------------------------------------
// DMS Endpoint injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_dms_endpoint() {
    let dms = Arc::new(DatabaseMigrationService::new());
    let ctx = default_ctx();

    let converter = AwsDmsEndpointConverter::new(Arc::clone(&dms));

    let mut injector = TerraformInjector::new();
    injector.register(AwsDmsEndpointConverter::new(Arc::clone(&dms)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_dms_endpoint",
        "test_ep",
        json!({
            "endpoint_id": "my-dms-source",
            "endpoint_type": "source",
            "engine_name": "mysql",
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    // Verify via snapshot
    let view = dms
        .snapshot(&ctx.default_account_id, &ctx.default_region)
        .await;
    assert!(view.endpoints.contains_key("my-dms-source"));
    let ep = &view.endpoints["my-dms-source"];
    assert_eq!(ep.endpoint_identifier, "my-dms-source");
    assert_eq!(ep.endpoint_type, "source");
    assert_eq!(ep.engine_name, "mysql");

    // Verify extract round-trip
    let extracted = converter
        .extract(&ctx)
        .await
        .expect("extract should succeed");
    assert_eq!(extracted.len(), 1);
    assert_eq!(extracted[0].attributes["endpoint_id"], "my-dms-source");
    assert_eq!(extracted[0].attributes["endpoint_type"], "source");
    assert_eq!(extracted[0].attributes["engine_name"], "mysql");
}

// ---------------------------------------------------------------------------
// DMS Replication Instance injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_dms_replication_instance() {
    let dms = Arc::new(DatabaseMigrationService::new());
    let ctx = default_ctx();

    let converter = AwsDmsReplicationInstanceConverter::new(Arc::clone(&dms));

    let mut injector = TerraformInjector::new();
    injector.register(AwsDmsReplicationInstanceConverter::new(Arc::clone(&dms)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_dms_replication_instance",
        "test_ri",
        json!({
            "replication_instance_id": "my-dms-repl",
            "replication_instance_class": "dms.r5.large",
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    // Verify via snapshot
    let view = dms
        .snapshot(&ctx.default_account_id, &ctx.default_region)
        .await;
    assert!(view.replication_instances.contains_key("my-dms-repl"));
    let ri = &view.replication_instances["my-dms-repl"];
    assert_eq!(ri.replication_instance_identifier, "my-dms-repl");
    assert_eq!(ri.replication_instance_class, "dms.r5.large");

    // Verify extract round-trip
    let extracted = converter
        .extract(&ctx)
        .await
        .expect("extract should succeed");
    assert_eq!(extracted.len(), 1);
    assert_eq!(
        extracted[0].attributes["replication_instance_id"],
        "my-dms-repl"
    );
    assert_eq!(
        extracted[0].attributes["replication_instance_class"],
        "dms.r5.large"
    );
}

// ---------------------------------------------------------------------------
// DSQL Cluster injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_dsql_cluster() {
    let dsql = Arc::new(DsqlService::new());
    let ctx = default_ctx();

    let converter = AwsDsqlClusterConverter::new(Arc::clone(&dsql));

    let mut injector = TerraformInjector::new();
    injector.register(AwsDsqlClusterConverter::new(Arc::clone(&dsql)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_dsql_cluster",
        "test_cluster",
        json!({
            "id": "my-dsql-cluster-id",
            "deletion_protection_enabled": true,
            "tags": { "Env": "prod" },
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    // Verify via snapshot
    let view = dsql
        .snapshot(&ctx.default_account_id, &ctx.default_region)
        .await;
    assert!(view.clusters.contains_key("my-dsql-cluster-id"));
    let cluster = &view.clusters["my-dsql-cluster-id"];
    assert_eq!(cluster.identifier, "my-dsql-cluster-id");
    assert!(cluster.deletion_protection_enabled);

    // Verify extract round-trip
    let extracted = converter
        .extract(&ctx)
        .await
        .expect("extract should succeed");
    assert_eq!(extracted.len(), 1);
    assert_eq!(extracted[0].attributes["id"], "my-dsql-cluster-id");
    assert_eq!(extracted[0].attributes["deletion_protection_enabled"], true);
}

// ---------------------------------------------------------------------------
// Timestream Write Database injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_timestreamwrite_database() {
    let tsw = Arc::new(TimestreamWriteService::new());
    let ctx = default_ctx();

    let converter = AwsTimestreamwriteDatabaseConverter::new(Arc::clone(&tsw));

    let mut injector = TerraformInjector::new();
    injector.register(AwsTimestreamwriteDatabaseConverter::new(Arc::clone(&tsw)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_timestreamwrite_database",
        "test_db",
        json!({
            "database_name": "my-tsw-database",
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    // Verify via snapshot
    let view = tsw
        .snapshot(&ctx.default_account_id, &ctx.default_region)
        .await;
    assert!(view.databases.contains_key("my-tsw-database"));
    let db = &view.databases["my-tsw-database"];
    assert_eq!(db.database_name, "my-tsw-database");

    // Verify extract round-trip
    let extracted = converter
        .extract(&ctx)
        .await
        .expect("extract should succeed");
    assert_eq!(extracted.len(), 1);
    assert_eq!(extracted[0].attributes["database_name"], "my-tsw-database");
}

// ---------------------------------------------------------------------------
// Timestream Write Table injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_timestreamwrite_table() {
    let tsw = Arc::new(TimestreamWriteService::new());
    let ctx = default_ctx();

    let converter = AwsTimestreamwriteTableConverter::new(Arc::clone(&tsw));

    let mut injector = TerraformInjector::new();
    injector.register(AwsTimestreamwriteTableConverter::new(Arc::clone(&tsw)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_timestreamwrite_table",
        "test_tbl",
        json!({
            "table_name": "my-tsw-table",
            "database_name": "my-tsw-database",
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    // Verify via snapshot
    let view = tsw
        .snapshot(&ctx.default_account_id, &ctx.default_region)
        .await;
    let key = "my-tsw-database\x1fmy-tsw-table";
    assert!(view.tables.contains_key(key));
    let tbl = &view.tables[key];
    assert_eq!(tbl.table_name, "my-tsw-table");
    assert_eq!(tbl.database_name, "my-tsw-database");

    // Verify extract round-trip
    let extracted = converter
        .extract(&ctx)
        .await
        .expect("extract should succeed");
    assert_eq!(extracted.len(), 1);
    assert_eq!(extracted[0].attributes["table_name"], "my-tsw-table");
    assert_eq!(extracted[0].attributes["database_name"], "my-tsw-database");
}

// ---------------------------------------------------------------------------
// EBS Snapshot injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_ebs_snapshot() {
    let ebs = Arc::new(EbsService::new());
    let ctx = default_ctx();

    let converter = AwsEbsSnapshotConverter::new(Arc::clone(&ebs));

    let mut injector = TerraformInjector::new();
    injector.register(AwsEbsSnapshotConverter::new(Arc::clone(&ebs)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_ebs_snapshot",
        "test_snap",
        json!({
            "snapshot_id": "snap-0123456789abcdef0",
            "volume_size": 50,
            "description": "Test snapshot for backup",
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    // Verify via snapshot
    let view = ebs
        .snapshot(&ctx.default_account_id, &ctx.default_region)
        .await;
    assert!(view.snapshots.contains_key("snap-0123456789abcdef0"));
    let snap = &view.snapshots["snap-0123456789abcdef0"];
    assert_eq!(snap.snapshot_id, "snap-0123456789abcdef0");
    assert_eq!(snap.volume_size, 50);
    assert_eq!(snap.description, "Test snapshot for backup");

    // Verify extract round-trip
    let extracted = converter
        .extract(&ctx)
        .await
        .expect("extract should succeed");
    assert_eq!(extracted.len(), 1);
    assert_eq!(
        extracted[0].attributes["snapshot_id"],
        "snap-0123456789abcdef0"
    );
    assert_eq!(extracted[0].attributes["volume_size"], 50);
    assert_eq!(
        extracted[0].attributes["description"],
        "Test snapshot for backup"
    );
}

// ---------------------------------------------------------------------------
// App Mesh injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_appmesh_mesh() {
    use winterbaume_appmesh::AppMeshService;
    use winterbaume_terraform::converters::appmesh::AwsAppmeshMeshConverter;
    let svc = Arc::new(AppMeshService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsAppmeshMeshConverter::new(Arc::clone(&svc)));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_appmesh_mesh",
        "my_mesh",
        json!({
            "name": "my-mesh",
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.meshes.contains_key("my-mesh"));
    assert_eq!(view.meshes["my-mesh"].mesh_name, "my-mesh");
}

// ---------------------------------------------------------------------------
// AMP (Prometheus) injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_prometheus_workspace() {
    use winterbaume_amp::AmpService;
    use winterbaume_terraform::converters::amp::AwsPrometheusWorkspaceConverter;
    let svc = Arc::new(AmpService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsPrometheusWorkspaceConverter::new(Arc::clone(&svc)));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_prometheus_workspace",
        "my_ws",
        json!({
            "id": "ws-12345",
            "alias": "my-prometheus",
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.workspaces.contains_key("ws-12345"));
    assert_eq!(
        view.workspaces["ws-12345"].alias,
        Some("my-prometheus".to_string())
    );
}

// ---------------------------------------------------------------------------
// Cognito Identity injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_cognito_identity_pool() {
    use winterbaume_cognitoidentity::CognitoIdentityService;
    use winterbaume_terraform::converters::cognitoidentity::AwsCognitoIdentityPoolConverter;
    let svc = Arc::new(CognitoIdentityService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsCognitoIdentityPoolConverter::new(Arc::clone(&svc)));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_cognito_identity_pool",
        "my_pool",
        json!({
            "id": "us-east-1:aaaa-bbbb-cccc",
            "identity_pool_name": "my-identity-pool",
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.identity_pools.contains_key("us-east-1:aaaa-bbbb-cccc"));
    assert_eq!(
        view.identity_pools["us-east-1:aaaa-bbbb-cccc"].identity_pool_name,
        "my-identity-pool"
    );
}

// ---------------------------------------------------------------------------
// Cost Explorer injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_ce_anomaly_monitor() {
    use winterbaume_costexplorer::CostExplorerService;
    use winterbaume_terraform::converters::costexplorer::AwsCeAnomalyMonitorConverter;
    let svc = Arc::new(CostExplorerService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsCeAnomalyMonitorConverter::new(Arc::clone(&svc)));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_ce_anomaly_monitor",
        "my_monitor",
        json!({
            "name": "my-cost-monitor",
            "monitor_type": "DIMENSIONAL",
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(!view.anomaly_monitors.is_empty());
    let monitor = view.anomaly_monitors.values().next().unwrap();
    assert_eq!(monitor.monitor_name, "my-cost-monitor");
    assert_eq!(monitor.monitor_type, "DIMENSIONAL");
}

// ---------------------------------------------------------------------------
// Data Pipeline injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_datapipeline_pipeline() {
    use winterbaume_datapipeline::DataPipelineService;
    use winterbaume_terraform::converters::datapipeline::AwsDatapipelinePipelineConverter;
    let svc = Arc::new(DataPipelineService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsDatapipelinePipelineConverter::new(Arc::clone(&svc)));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_datapipeline_pipeline",
        "my_pipeline",
        json!({
            "name": "my-data-pipeline",
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(!view.pipelines.is_empty());
    let pipeline = view.pipelines.values().next().unwrap();
    assert_eq!(pipeline.name, "my-data-pipeline");
}

// ---------------------------------------------------------------------------
// DataSync injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_datasync_task() {
    use winterbaume_datasync::DataSyncService;
    use winterbaume_terraform::converters::datasync::AwsDatasyncTaskConverter;
    let svc = Arc::new(DataSyncService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsDatasyncTaskConverter::new(Arc::clone(&svc)));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_datasync_task",
        "my_task",
        json!({
            "source_location_arn": "arn:aws:datasync:us-east-1:123456789012:location/loc-src",
            "destination_location_arn": "arn:aws:datasync:us-east-1:123456789012:location/loc-dst",
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(!view.tasks.is_empty());
    let task = view.tasks.values().next().unwrap();
    assert_eq!(
        task.source_location_arn,
        "arn:aws:datasync:us-east-1:123456789012:location/loc-src"
    );
    assert_eq!(
        task.destination_location_arn,
        "arn:aws:datasync:us-east-1:123456789012:location/loc-dst"
    );
}

// ---------------------------------------------------------------------------
// Directory Service injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_directory_service_directory() {
    use winterbaume_directory::DirectoryService;
    use winterbaume_terraform::converters::directory::AwsDirectoryServiceDirectoryConverter;
    let svc = Arc::new(DirectoryService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsDirectoryServiceDirectoryConverter::new(Arc::clone(&svc)));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_directory_service_directory",
        "my_dir",
        json!({
            "name": "corp.example.com",
            "type": "MicrosoftAD",
            "size": "Large",
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(!view.directories.is_empty());
    let dir = view.directories.values().next().unwrap();
    assert_eq!(dir.name, "corp.example.com");
    assert_eq!(dir.directory_type, "MicrosoftAD");
    assert_eq!(dir.size, "Large");
}

// ---------------------------------------------------------------------------
// Elastic Beanstalk application injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_elastic_beanstalk_application() {
    use winterbaume_terraform::converters::elasticbeanstalk::AwsElasticBeanstalkApplicationConverter;
    let svc = Arc::new(ElasticBeanstalkService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsElasticBeanstalkApplicationConverter::new(Arc::clone(
        &svc,
    )));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_elastic_beanstalk_application",
        "my_app",
        json!({
            "name": "my-beanstalk-app",
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.applications.contains_key("my-beanstalk-app"));
    assert_eq!(
        view.applications["my-beanstalk-app"].application_name,
        "my-beanstalk-app"
    );
}

// ---------------------------------------------------------------------------
// Classic ELB injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_classic_elb() {
    use winterbaume_elasticloadbalancing::ElasticLoadBalancingService;
    use winterbaume_terraform::converters::elasticloadbalancing::AwsElbConverter;
    let svc = Arc::new(ElasticLoadBalancingService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsElbConverter::new(Arc::clone(&svc)));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_elb",
        "my_elb",
        json!({
            "name": "my-classic-lb",
            "listener": [
                {
                    "lb_port": 80,
                    "instance_port": 8080,
                    "lb_protocol": "HTTP",
                    "instance_protocol": "HTTP"
                }
            ],
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.load_balancers.contains_key("my-classic-lb"));
    assert_eq!(view.load_balancers["my-classic-lb"].listeners.len(), 1);
    assert_eq!(
        view.load_balancers["my-classic-lb"].listeners[0].load_balancer_port,
        80
    );
}

// ---------------------------------------------------------------------------
// EMR Containers injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_emrcontainers_virtual_cluster() {
    use winterbaume_emrcontainers::EmrContainersService;
    use winterbaume_terraform::converters::emrcontainers::AwsEmrcontainersVirtualClusterConverter;
    let svc = Arc::new(EmrContainersService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsEmrcontainersVirtualClusterConverter::new(Arc::clone(
        &svc,
    )));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_emrcontainers_virtual_cluster",
        "my_vc",
        json!({
            "name": "my-virtual-cluster",
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(!view.virtual_clusters.is_empty());
    let vc = view.virtual_clusters.values().next().unwrap();
    assert_eq!(vc.name, "my-virtual-cluster");
}

// ---------------------------------------------------------------------------
// EMR Serverless injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_emrserverless_application() {
    use winterbaume_emrserverless::EmrServerlessService;
    use winterbaume_terraform::converters::emrserverless::AwsEmrserverlessApplicationConverter;
    let svc = Arc::new(EmrServerlessService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsEmrserverlessApplicationConverter::new(Arc::clone(&svc)));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_emrserverless_application",
        "my_app",
        json!({
            "name": "my-emr-app",
            "release_label": "emr-6.15.0",
            "type": "SPARK",
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(!view.applications.is_empty());
    let app = view.applications.values().next().unwrap();
    assert_eq!(app.name, "my-emr-app");
    assert_eq!(app.release_label, "emr-6.15.0");
    assert_eq!(app.application_type, "SPARK");
}

// ---------------------------------------------------------------------------
// Identity Store injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_identitystore_group() {
    use winterbaume_identitystore::IdentityStoreService;
    use winterbaume_terraform::converters::identitystore::AwsIdentitystoreGroupConverter;
    let svc = Arc::new(IdentityStoreService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsIdentitystoreGroupConverter::new(Arc::clone(&svc)));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_identitystore_group",
        "my_group",
        json!({
            "identity_store_id": "d-1234567890",
            "group_id": "g-abcdef",
            "display_name": "Engineering",
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    let key = "d-1234567890/g-abcdef";
    assert!(view.groups.contains_key(key));
    assert_eq!(
        view.groups[key].display_name,
        Some("Engineering".to_string())
    );
}

// ---------------------------------------------------------------------------
// IoT injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_iot_thing() {
    use winterbaume_iot::IotService;
    use winterbaume_terraform::converters::iot::AwsIotThingConverter;
    let svc = Arc::new(IotService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsIotThingConverter::new(Arc::clone(&svc)));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_iot_thing",
        "my_thing",
        json!({
            "name": "my-iot-thing",
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.things.contains_key("my-iot-thing"));
    assert_eq!(view.things["my-iot-thing"].thing_name, "my-iot-thing");
}

// ---------------------------------------------------------------------------
// IVS injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_ivs_channel() {
    use winterbaume_ivs::IvsService;
    use winterbaume_terraform::converters::ivs::AwsIvsChannelConverter;
    let svc = Arc::new(IvsService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsIvsChannelConverter::new(Arc::clone(&svc)));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_ivs_channel",
        "my_channel",
        json!({
            "name": "my-ivs-channel",
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(!view.channels.is_empty());
    let ch = view.channels.values().next().unwrap();
    assert_eq!(ch.name, "my-ivs-channel");
}

// ---------------------------------------------------------------------------
// Kinesis Video injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_kinesis_video_stream() {
    use winterbaume_kinesisvideo::KinesisVideoService;
    use winterbaume_terraform::converters::kinesisvideo::AwsKinesisVideoStreamConverter;
    let svc = Arc::new(KinesisVideoService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsKinesisVideoStreamConverter::new(Arc::clone(&svc)));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_kinesis_video_stream",
        "my_stream",
        json!({
            "name": "my-video-stream",
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.streams.contains_key("my-video-stream"));
    assert_eq!(
        view.streams["my-video-stream"].stream_name,
        "my-video-stream"
    );
}

// ---------------------------------------------------------------------------
// Kinesis Analytics V2 injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_kinesisanalyticsv2_application() {
    use winterbaume_kinesisanalyticsv2::KinesisAnalyticsV2Service;
    use winterbaume_terraform::converters::kinesisanalyticsv2::AwsKinesisanalyticsv2ApplicationConverter;
    let svc = Arc::new(KinesisAnalyticsV2Service::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsKinesisanalyticsv2ApplicationConverter::new(Arc::clone(
        &svc,
    )));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_kinesisanalyticsv2_application",
        "my_app",
        json!({
            "name": "my-analytics-app",
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.applications.contains_key("my-analytics-app"));
    assert_eq!(
        view.applications["my-analytics-app"].application_name,
        "my-analytics-app"
    );
}

// ---------------------------------------------------------------------------
// Lake Formation injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_lakeformation_resource() {
    use winterbaume_lakeformation::LakeFormationService;
    use winterbaume_terraform::converters::lakeformation::AwsLakeformationResourceConverter;
    let svc = Arc::new(LakeFormationService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsLakeformationResourceConverter::new(Arc::clone(&svc)));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_lakeformation_resource",
        "my_resource",
        json!({
            "arn": "arn:aws:s3:::my-data-lake-bucket",
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(
        view.resources
            .contains_key("arn:aws:s3:::my-data-lake-bucket")
    );
    assert_eq!(
        view.resources["arn:aws:s3:::my-data-lake-bucket"].resource_arn,
        "arn:aws:s3:::my-data-lake-bucket"
    );
}

// ---------------------------------------------------------------------------
// Lex Bot injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_lex_bot() {
    use winterbaume_lexmodelsv2::LexModelsV2Service;
    use winterbaume_terraform::converters::lexmodelsv2::AwsLexv2modelsBotConverter;
    let svc = Arc::new(LexModelsV2Service::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsLexv2modelsBotConverter::new(Arc::clone(&svc)));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_lexv2models_bot",
        "my_bot",
        json!({
            "id": "bot-12345",
            "name": "my-lex-bot",
            "role_arn": "arn:aws:iam::123456789012:role/lex-role",
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.bots.contains_key("bot-12345"));
    assert_eq!(view.bots["bot-12345"].bot_name, "my-lex-bot");
    assert_eq!(
        view.bots["bot-12345"].role_arn,
        "arn:aws:iam::123456789012:role/lex-role"
    );
}

// ---------------------------------------------------------------------------
// MediaLive injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_medialive_input() {
    use winterbaume_medialive::MediaLiveService;
    use winterbaume_terraform::converters::medialive::AwsMedialiveInputConverter;
    let svc = Arc::new(MediaLiveService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsMedialiveInputConverter::new(Arc::clone(&svc)));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_medialive_input",
        "my_input",
        json!({
            "input_id": "inp-12345",
            "name": "my-media-input",
            "type": "RTMP_PUSH",
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(!view.inputs.is_empty());
    assert_eq!(view.inputs[0].name, "my-media-input");
    assert_eq!(view.inputs[0].input_type, "RTMP_PUSH");
}

// ---------------------------------------------------------------------------
// MediaStore injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_media_store_container() {
    use winterbaume_mediastore::MediaStoreService;
    use winterbaume_terraform::converters::mediastore::AwsMediaStoreContainerConverter;
    let svc = Arc::new(MediaStoreService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsMediaStoreContainerConverter::new(Arc::clone(&svc)));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_media_store_container",
        "my_container",
        json!({
            "name": "my-media-container",
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.containers.contains_key("my-media-container"));
    assert_eq!(
        view.containers["my-media-container"].name,
        "my-media-container"
    );
}

// ---------------------------------------------------------------------------
// OSIS injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_osis_pipeline() {
    use winterbaume_osis::OsisService;
    use winterbaume_terraform::converters::osis::AwsOsisPipelineConverter;
    let svc = Arc::new(OsisService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsOsisPipelineConverter::new(Arc::clone(&svc)));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_osis_pipeline",
        "my_pipeline",
        json!({
            "pipeline_name": "my-osis-pipeline",
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.pipelines.contains_key("my-osis-pipeline"));
    assert_eq!(
        view.pipelines["my-osis-pipeline"].pipeline_name,
        "my-osis-pipeline"
    );
}

// ---------------------------------------------------------------------------
// Organizations injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_organizations_policy() {
    use winterbaume_organizations::OrganizationsService;
    use winterbaume_terraform::converters::organizations::AwsOrganizationsPolicyConverter;
    let svc = Arc::new(OrganizationsService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsOrganizationsPolicyConverter::new(Arc::clone(&svc)));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_organizations_policy",
        "my_scp",
        json!({
            "name": "deny-all-scp",
            "type": "SERVICE_CONTROL_POLICY",
            "content": "{\"Version\":\"2012-10-17\",\"Statement\":[{\"Effect\":\"Deny\",\"Action\":\"*\",\"Resource\":\"*\"}]}",
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(!view.policies.is_empty());
    let policy = view.policies.values().next().unwrap();
    assert_eq!(policy.name, "deny-all-scp");
    assert_eq!(policy.policy_type, "SERVICE_CONTROL_POLICY");
    assert!(policy.content.contains("Deny"));
}

// ---------------------------------------------------------------------------
// Pipes injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_pipes_pipe() {
    use winterbaume_pipes::PipesService;
    use winterbaume_terraform::converters::pipes::AwsPipesPipeConverter;
    let svc = Arc::new(PipesService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsPipesPipeConverter::new(Arc::clone(&svc)));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_pipes_pipe",
        "my_pipe",
        json!({
            "name": "my-event-pipe",
            "source": "arn:aws:sqs:us-east-1:123456789012:my-queue",
            "target": "arn:aws:lambda:us-east-1:123456789012:function:my-fn",
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.pipes.contains_key("my-event-pipe"));
    assert_eq!(
        view.pipes["my-event-pipe"].source,
        "arn:aws:sqs:us-east-1:123456789012:my-queue"
    );
    assert_eq!(
        view.pipes["my-event-pipe"].target,
        "arn:aws:lambda:us-east-1:123456789012:function:my-fn"
    );
}

// ---------------------------------------------------------------------------
// QuickSight injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_quicksight_group() {
    use winterbaume_quicksight::QuickSightService;
    use winterbaume_terraform::converters::quicksight::AwsQuicksightGroupConverter;
    let svc = Arc::new(QuickSightService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsQuicksightGroupConverter::new(Arc::clone(&svc)));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_quicksight_group",
        "my_group",
        json!({
            "group_name": "analysts",
            "namespace": "default",
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.groups.contains_key("default"));
    assert!(view.groups["default"].contains_key("analysts"));
    assert_eq!(view.groups["default"]["analysts"].group_name, "analysts");
}

// ---------------------------------------------------------------------------
// RAM injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_ram_resource_share() {
    use winterbaume_ram::RamService;
    use winterbaume_terraform::converters::ram::AwsRamResourceShareConverter;
    let svc = Arc::new(RamService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsRamResourceShareConverter::new(Arc::clone(&svc)));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_ram_resource_share",
        "my_share",
        json!({
            "name": "my-resource-share",
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(!view.resource_shares.is_empty());
    let share = view.resource_shares.values().next().unwrap();
    assert_eq!(share.name, "my-resource-share");
}

// ---------------------------------------------------------------------------
// Rekognition injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_rekognition_collection() {
    use winterbaume_rekognition::RekognitionService;
    use winterbaume_terraform::converters::rekognition::AwsRekognitionCollectionConverter;
    let svc = Arc::new(RekognitionService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsRekognitionCollectionConverter::new(Arc::clone(&svc)));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_rekognition_collection",
        "my_collection",
        json!({
            "collection_id": "my-faces-collection",
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.collections.contains_key("my-faces-collection"));
    assert_eq!(
        view.collections["my-faces-collection"].collection_id,
        "my-faces-collection"
    );
}

// ---------------------------------------------------------------------------
// Resource Groups injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_resourcegroups_group() {
    use winterbaume_resourcegroups::ResourceGroupsService;
    use winterbaume_terraform::converters::resourcegroups::AwsResourcegroupsGroupConverter;
    let svc = Arc::new(ResourceGroupsService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsResourcegroupsGroupConverter::new(Arc::clone(&svc)));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_resourcegroups_group",
        "my_group",
        json!({
            "name": "my-resource-group",
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.groups.contains_key("my-resource-group"));
    assert_eq!(view.groups["my-resource-group"].name, "my-resource-group");
}

// ---------------------------------------------------------------------------
// S3 Tables injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_s3tables_table_bucket() {
    use winterbaume_s3tables::S3TablesService;
    use winterbaume_terraform::converters::s3tables::AwsS3tablesTableBucketConverter;
    let svc = Arc::new(S3TablesService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsS3tablesTableBucketConverter::new(Arc::clone(&svc)));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_s3tables_table_bucket",
        "my_tb",
        json!({
            "name": "my-table-bucket",
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(!view.table_buckets.is_empty());
    let tb = view.table_buckets.values().next().unwrap();
    assert_eq!(tb.name, "my-table-bucket");
}

// ---------------------------------------------------------------------------
// Service Catalog portfolio injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_servicecatalog_portfolio() {
    use winterbaume_terraform::converters::servicecatalog::AwsServicecatalogPortfolioConverter;
    let svc = Arc::new(ServiceCatalogService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsServicecatalogPortfolioConverter::new(Arc::clone(&svc)));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_servicecatalog_portfolio",
        "my_portfolio",
        json!({
            "name": "my-portfolio",
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(!view.portfolios.is_empty());
    let portfolio = view.portfolios.values().next().unwrap();
    assert_eq!(portfolio.display_name, "my-portfolio");
}

// ---------------------------------------------------------------------------
// Service Catalog App Registry injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_servicecatalogappregistry_application() {
    use winterbaume_servicecatalogappregistry::ServiceCatalogAppRegistryService;
    use winterbaume_terraform::converters::servicecatalogappregistry::AwsServicecatalogappregistryApplicationConverter;
    let svc = Arc::new(ServiceCatalogAppRegistryService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsServicecatalogappregistryApplicationConverter::new(
        Arc::clone(&svc),
    ));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_servicecatalogappregistry_application",
        "my_app",
        json!({
            "name": "my-app-registry-app",
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(!view.applications.is_empty());
    let app = view.applications.values().next().unwrap();
    assert_eq!(app.name, "my-app-registry-app");
}

// ---------------------------------------------------------------------------
// Service Quotas injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_servicequotas_service_quota() {
    use winterbaume_servicequotas::ServiceQuotasService;
    use winterbaume_terraform::converters::servicequotas::AwsServicequotasServiceQuotaConverter;
    let svc = Arc::new(ServiceQuotasService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsServicequotasServiceQuotaConverter::new(Arc::clone(&svc)));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_servicequotas_service_quota",
        "my_quota",
        json!({
            "service_code": "ec2",
            "quota_code": "L-1216C47A",
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.quotas.contains_key("ec2/L-1216C47A"));
    assert_eq!(view.quotas["ec2/L-1216C47A"].service_code, "ec2");
    assert_eq!(view.quotas["ec2/L-1216C47A"].quota_code, "L-1216C47A");
}

// ---------------------------------------------------------------------------
// Signer injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_signer_signing_profile() {
    use winterbaume_signer::SignerService;
    use winterbaume_terraform::converters::signer::AwsSignerSigningProfileConverter;
    let svc = Arc::new(SignerService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsSignerSigningProfileConverter::new(Arc::clone(&svc)));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_signer_signing_profile",
        "my_profile",
        json!({
            "name": "my-signing-profile",
            "platform_id": "AWSLambda-SHA384-ECDSA",
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.profiles.contains_key("my-signing-profile"));
    assert_eq!(
        view.profiles["my-signing-profile"].platform_id,
        "AWSLambda-SHA384-ECDSA"
    );
}

// ---------------------------------------------------------------------------
// SimpleDB injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_simpledb_domain() {
    use winterbaume_simpledbv2::SimpleDbV2Service;
    use winterbaume_terraform::converters::simpledbv2::AwsSimpleDbDomainConverter;
    let svc = Arc::new(SimpleDbV2Service::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsSimpleDbDomainConverter::new(Arc::clone(&svc)));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_simpledb_domain",
        "my_domain",
        json!({
            "name": "my-sdb-domain",
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.domains.contains("my-sdb-domain"));
}

// ---------------------------------------------------------------------------
// SWF injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_swf_domain() {
    use winterbaume_swf::SwfService;
    use winterbaume_terraform::converters::swf::AwsSwfDomainConverter;
    let svc = Arc::new(SwfService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsSwfDomainConverter::new(Arc::clone(&svc)));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_swf_domain",
        "my_domain",
        json!({
            "name": "my-swf-domain",
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.domains.contains_key("my-swf-domain"));
    assert_eq!(view.domains["my-swf-domain"].name, "my-swf-domain");
}

// ---------------------------------------------------------------------------
// Synthetics injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_synthetics_canary() {
    use winterbaume_synthetics::SyntheticsService;
    use winterbaume_terraform::converters::synthetics::AwsSyntheticsCanaryConverter;
    let svc = Arc::new(SyntheticsService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsSyntheticsCanaryConverter::new(Arc::clone(&svc)));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_synthetics_canary",
        "my_canary",
        json!({
            "name": "my-canary",
            "runtime_version": "syn-nodejs-puppeteer-6.2",
            "handler": "index.handler",
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.canaries.contains_key("my-canary"));
    assert_eq!(view.canaries["my-canary"].name, "my-canary");
    assert_eq!(
        view.canaries["my-canary"].runtime_version,
        "syn-nodejs-puppeteer-6.2"
    );
}

// ---------------------------------------------------------------------------
// Transcribe injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_transcribe_vocabulary() {
    use winterbaume_terraform::converters::transcribe::AwsTranscribeVocabularyConverter;
    use winterbaume_transcribe::TranscribeService;
    let svc = Arc::new(TranscribeService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsTranscribeVocabularyConverter::new(Arc::clone(&svc)));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_transcribe_vocabulary",
        "my_vocab",
        json!({
            "vocabulary_name": "my-vocab",
            "language_code": "en-US",
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.vocabularies.contains_key("my-vocab"));
    assert_eq!(view.vocabularies["my-vocab"].vocabulary_name, "my-vocab");
    assert_eq!(view.vocabularies["my-vocab"].language_code, "en-US");
}

// ---------------------------------------------------------------------------
// X-Ray injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_xray_group() {
    use winterbaume_terraform::converters::xray::AwsXrayGroupConverter;
    use winterbaume_xray::XRayService;
    let svc = Arc::new(XRayService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsXrayGroupConverter::new(Arc::clone(&svc)));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_xray_group",
        "my_group",
        json!({
            "group_name": "my-xray-group",
            "filter_expression": "responsetime > 5",
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.groups.contains_key("my-xray-group"));
    assert_eq!(view.groups["my-xray-group"].group_name, "my-xray-group");
    assert_eq!(
        view.groups["my-xray-group"].filter_expression,
        "responsetime > 5"
    );
}

#[tokio::test]
async fn test_inject_xray_sampling_rule() {
    use winterbaume_terraform::converters::xray::AwsXraySamplingRuleConverter;
    use winterbaume_xray::XRayService;
    let svc = Arc::new(XRayService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsXraySamplingRuleConverter::new(Arc::clone(&svc)));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_xray_sampling_rule",
        "my_rule",
        json!({
            "rule_name": "my-sampling-rule",
            "priority": 100,
            "fixed_rate": 0.1,
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.sampling_rules.contains_key("my-sampling-rule"));
    assert_eq!(
        view.sampling_rules["my-sampling-rule"].rule_name,
        "my-sampling-rule"
    );
    assert_eq!(view.sampling_rules["my-sampling-rule"].priority, 100);
    assert!((view.sampling_rules["my-sampling-rule"].fixed_rate - 0.1).abs() < f64::EPSILON);
}

// ---------------------------------------------------------------------------
// MediaPackage injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_media_package_channel() {
    use winterbaume_mediapackage::MediaPackageService;
    use winterbaume_terraform::converters::mediapackage::AwsMediaPackageChannelConverter;
    let svc = Arc::new(MediaPackageService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsMediaPackageChannelConverter::new(Arc::clone(&svc)));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_media_package_channel",
        "my_channel",
        json!({
            "channel_id": "my-mp-channel",
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.channels.contains_key("my-mp-channel"));
    assert_eq!(view.channels["my-mp-channel"].id, "my-mp-channel");
}

// ---------------------------------------------------------------------------
// Resilience Hub injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_resiliencehub_policy() {
    use winterbaume_resiliencehub::ResilienceHubService;
    use winterbaume_terraform::converters::resiliencehub::AwsResilienceHubResiliencyPolicyConverter;
    let svc = Arc::new(ResilienceHubService::new());
    let ctx = default_ctx();
    let mut injector = TerraformInjector::new();
    injector.register(AwsResilienceHubResiliencyPolicyConverter::new(Arc::clone(
        &svc,
    )));
    let tfstate = make_tfstate(vec![make_resource(
        "aws_resiliencehub_resiliency_policy",
        "my_policy",
        json!({
            "policy_name": "my-resilience-policy",
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(!view.policies.is_empty());
    let policy = view.policies.values().next().unwrap();
    assert_eq!(policy.policy_name, "my-resilience-policy");
}

// ---------------------------------------------------------------------------
// GuardDuty injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_guardduty_detector() {
    use winterbaume_guardduty::GuardDutyService;
    use winterbaume_terraform::converters::guardduty::AwsGuarddutyDetectorConverter;

    let svc = Arc::new(GuardDutyService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsGuarddutyDetectorConverter::new(Arc::clone(&svc)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_guardduty_detector",
        "main",
        json!({
            "enable": "true",
            "finding_publishing_frequency": "ONE_HOUR",
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert_eq!(view.detectors.len(), 1);
    let det = view.detectors.values().next().unwrap();
    assert_eq!(det.status, "ENABLED");
    assert_eq!(det.finding_publishing_frequency, "ONE_HOUR");
}

// ---------------------------------------------------------------------------
// SecurityHub injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_securityhub_account() {
    use winterbaume_terraform::converters::securityhub::AwsSecurityhubAccountConverter;

    let svc = Arc::new(SecurityHubService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsSecurityhubAccountConverter::new(Arc::clone(&svc)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_securityhub_account",
        "main",
        json!({
            "enable_default_standards": true,
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);
    // Note: SecurityHub merge currently handles findings/members/action_targets
    // but not hub-level settings. The inject call succeeds without error.
}

// ---------------------------------------------------------------------------
// ACM PCA injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_acmpca_certificate_authority() {
    use winterbaume_acmpca::AcmPcaService;
    use winterbaume_terraform::converters::acmpca::AwsAcmpcaCertificateAuthorityConverter;

    let svc = Arc::new(AcmPcaService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsAcmpcaCertificateAuthorityConverter::new(Arc::clone(
        &svc,
    )));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_acmpca_certificate_authority",
        "root",
        json!({
            "type": "ROOT",
            "certificate_authority_configuration": {
                "key_algorithm": "RSA_4096",
                "signing_algorithm": "SHA512WITHRSA",
                "subject": {
                    "common_name": "My Root CA",
                    "organization": "TestOrg",
                },
            },
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert_eq!(view.certificate_authorities.len(), 1);
    let ca = view.certificate_authorities.values().next().unwrap();
    assert_eq!(ca.ca_type, "ROOT");
    assert_eq!(ca.ca_config.key_algorithm, "RSA_4096");
    assert_eq!(ca.ca_config.signing_algorithm, "SHA512WITHRSA");
}

// ---------------------------------------------------------------------------
// Shield injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_shield_protection() {
    use winterbaume_shield::ShieldService;
    use winterbaume_terraform::converters::shield::AwsShieldProtectionConverter;

    let svc = Arc::new(ShieldService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsShieldProtectionConverter::new(Arc::clone(&svc)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_shield_protection",
        "web",
        json!({
            "name": "web-protection",
            "resource_arn": "arn:aws:elasticloadbalancing:us-east-1:123456789012:loadbalancer/app/my-alb/1234567890",
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert_eq!(view.protections.len(), 1);
    let prot = view.protections.values().next().unwrap();
    assert_eq!(prot.name, "web-protection");
    assert_eq!(
        prot.resource_arn,
        "arn:aws:elasticloadbalancing:us-east-1:123456789012:loadbalancer/app/my-alb/1234567890"
    );
}

// ---------------------------------------------------------------------------
// Bedrock injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_bedrock_guardrail() {
    use winterbaume_bedrock::BedrockService;
    use winterbaume_terraform::converters::bedrock::AwsBedrockGuardrailConverter;

    let svc = Arc::new(BedrockService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsBedrockGuardrailConverter::new(Arc::clone(&svc)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_bedrock_guardrail",
        "safety",
        json!({
            "name": "content-safety",
            "blocked_input_messaging": "Input blocked by safety policy",
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert_eq!(view.guardrails.len(), 1);
    let g = view.guardrails.values().next().unwrap();
    assert_eq!(g.name, "content-safety");
    assert_eq!(g.blocked_input_messaging, "Input blocked by safety policy");
}

// ---------------------------------------------------------------------------
// Bedrock Agent injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_bedrockagent_agent() {
    use winterbaume_bedrockagent::BedrockAgentService;
    use winterbaume_terraform::converters::bedrockagent::AwsBedrockagentAgentConverter;

    let svc = Arc::new(BedrockAgentService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsBedrockagentAgentConverter::new(Arc::clone(&svc)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_bedrockagent_agent",
        "assistant",
        json!({
            "agent_name": "my-assistant",
            "foundation_model": "anthropic.claude-3-haiku-20240307-v1:0",
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert_eq!(view.agents.len(), 1);
    let agent = view.agents.values().next().unwrap();
    assert_eq!(agent.agent_name, "my-assistant");
    assert_eq!(
        agent.foundation_model.as_deref(),
        Some("anthropic.claude-3-haiku-20240307-v1:0")
    );
}

// ---------------------------------------------------------------------------
// SageMaker endpoint / notebook injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_sagemaker_endpoint() {
    let svc = Arc::new(SageMakerService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsSagemakerEndpointConverter::new(Arc::clone(&svc)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_sagemaker_endpoint",
        "prod",
        json!({
            "name": "my-endpoint",
            "endpoint_config_name": "my-endpoint-config",
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert_eq!(view.endpoints.len(), 1);
    let ep = &view.endpoints["my-endpoint"];
    assert_eq!(ep.endpoint_name, "my-endpoint");
    assert_eq!(ep.endpoint_config_name, "my-endpoint-config");
}

#[tokio::test]
async fn test_inject_sagemaker_notebook_instance() {
    let svc = Arc::new(SageMakerService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsSagemakerNotebookInstanceConverter::new(Arc::clone(&svc)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_sagemaker_notebook_instance",
        "dev",
        json!({
            "name": "dev-notebook",
            "instance_type": "ml.t3.large",
            "role_arn": "arn:aws:iam::123456789012:role/SageMakerRole",
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert_eq!(view.notebook_instances.len(), 1);
    let nb = &view.notebook_instances["dev-notebook"];
    assert_eq!(nb.notebook_instance_name, "dev-notebook");
    assert_eq!(nb.instance_type, "ml.t3.large");
    assert_eq!(nb.role_arn, "arn:aws:iam::123456789012:role/SageMakerRole");
}

// ---------------------------------------------------------------------------
// Comprehend injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_comprehend_entity_recognizer() {
    use winterbaume_comprehend::ComprehendService;
    use winterbaume_terraform::converters::comprehend::AwsComprehendEntityRecognizerConverter;

    let svc = Arc::new(ComprehendService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsComprehendEntityRecognizerConverter::new(Arc::clone(
        &svc,
    )));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_comprehend_entity_recognizer",
        "ner",
        json!({
            "name": "my-entity-recognizer",
            "language_code": "en",
            "data_access_role_arn": "arn:aws:iam::123456789012:role/ComprehendRole",
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert_eq!(view.entity_recognizers.len(), 1);
    let rec = view.entity_recognizers.values().next().unwrap();
    assert_eq!(rec.name, "my-entity-recognizer");
    assert_eq!(rec.language_code, "en");
    assert_eq!(
        rec.data_access_role_arn,
        "arn:aws:iam::123456789012:role/ComprehendRole"
    );
}

// ---------------------------------------------------------------------------
// Macie2 injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_macie2_account() {
    use winterbaume_macie2::Macie2Service;
    use winterbaume_terraform::converters::macie2::AwsMacie2AccountConverter;

    let svc = Arc::new(Macie2Service::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsMacie2AccountConverter::new(Arc::clone(&svc)));

    let tfstate = make_tfstate(vec![make_resource("aws_macie2_account", "main", json!({}))]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.session.is_some());
    let session = view.session.unwrap();
    assert_eq!(session.status, "ENABLED");
}

// ---------------------------------------------------------------------------
// Lake Formation data lake settings injection test
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_lakeformation_data_lake_settings() {
    use winterbaume_lakeformation::LakeFormationService;
    use winterbaume_terraform::converters::lakeformation::AwsLakeformationDataLakeSettingsConverter;

    let svc = Arc::new(LakeFormationService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsLakeformationDataLakeSettingsConverter::new(Arc::clone(
        &svc,
    )));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_lakeformation_data_lake_settings",
        "main",
        json!({
            "admins": ["arn:aws:iam::123456789012:role/Admin"],
            "allow_external_data_filtering": true,
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert_eq!(
        view.data_lake_settings.data_lake_admins,
        vec!["arn:aws:iam::123456789012:role/Admin"]
    );
    assert!(view.data_lake_settings.allow_external_data_filtering);
}

// ---------------------------------------------------------------------------
// Auto Scaling launch configuration injection test
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_launch_configuration() {
    use winterbaume_terraform::converters::autoscaling::AwsLaunchConfigurationConverter;

    let svc = Arc::new(AutoScalingService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsLaunchConfigurationConverter::new(Arc::clone(&svc)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_launch_configuration",
        "lc",
        json!({
            "name": "my-lc",
            "image_id": "ami-123",
            "instance_type": "t2.micro",
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.launch_configs.contains_key("my-lc"));
    let lc = &view.launch_configs["my-lc"];
    assert_eq!(lc.image_id, Some("ami-123".to_string()));
    assert_eq!(lc.instance_type, Some("t2.micro".to_string()));
}

// ---------------------------------------------------------------------------
// Macie2 classification job injection test
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_macie2_classification_job() {
    use winterbaume_macie2::Macie2Service;
    use winterbaume_terraform::converters::macie2::AwsMacie2ClassificationJobConverter;

    let svc = Arc::new(Macie2Service::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsMacie2ClassificationJobConverter::new(Arc::clone(&svc)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_macie2_classification_job",
        "job",
        json!({
            "name": "my-job",
            "job_type": "ONE_TIME",
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert_eq!(view.classification_jobs.len(), 1);
    let job = view.classification_jobs.values().next().unwrap();
    assert_eq!(job.name, "my-job");
    assert_eq!(job.job_type, "ONE_TIME");
}

// ---------------------------------------------------------------------------
// MediaPackageV2 channel group injection test
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_media_packagev2_channel_group() {
    use winterbaume_mediapackagev2::MediaPackageV2Service;
    use winterbaume_terraform::converters::mediapackagev2::AwsMediaPackageV2ChannelGroupConverter;

    let svc = Arc::new(MediaPackageV2Service::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsMediaPackageV2ChannelGroupConverter::new(Arc::clone(
        &svc,
    )));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_media_packagev2_channel_group",
        "cg",
        json!({
            "name": "my-group",
            "channel_group_name": "my-group",
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.channel_groups.contains_key("my-group"));
}

// ---------------------------------------------------------------------------
// MediaLive channel injection test
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_medialive_channel() {
    use winterbaume_medialive::MediaLiveService;
    use winterbaume_terraform::converters::medialive::AwsMedialiveChannelConverter;

    let svc = Arc::new(MediaLiveService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsMedialiveChannelConverter::new(Arc::clone(&svc)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_medialive_channel",
        "ch",
        json!({
            "channel_id": "ch-001",
            "name": "my-channel",
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert_eq!(view.channels.len(), 1);
    assert_eq!(view.channels[0].name, "my-channel");
}

// ---------------------------------------------------------------------------
// EC2 NAT gateway injection test
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_nat_gateway() {
    use winterbaume_terraform::converters::ec2::AwsNatGatewayConverter;

    let svc = Arc::new(Ec2Service::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsNatGatewayConverter::new(Arc::clone(&svc)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_nat_gateway",
        "ngw",
        json!({
            "id": "nat-123",
            "subnet_id": "subnet-123",
            "allocation_id": "eipalloc-456",
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.nat_gateways.contains_key("nat-123"));
    let ngw = &view.nat_gateways["nat-123"];
    assert_eq!(ngw.subnet_id, "subnet-123");
    assert_eq!(ngw.allocation_id, Some("eipalloc-456".to_string()));
}

// ---------------------------------------------------------------------------
// EC2 placement group injection test
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_placement_group() {
    use winterbaume_terraform::converters::ec2::AwsPlacementGroupConverter;

    let svc = Arc::new(Ec2Service::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsPlacementGroupConverter::new(Arc::clone(&svc)));

    let tfstate = make_tfstate(vec![
        make_resource(
            "aws_placement_group",
            "cluster_pg",
            json!({
                "id": "my-cluster-pg",
                "name": "my-cluster-pg",
                "strategy": "cluster",
                "tags": {"Env": "test"},
            }),
        ),
        make_resource(
            "aws_placement_group",
            "partition_pg",
            json!({
                "id": "my-partition-pg",
                "name": "my-partition-pg",
                "strategy": "partition",
                "partition_count": 4,
            }),
        ),
    ]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 2);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert_eq!(view.placement_groups.len(), 2);
    let cluster_pg = view
        .placement_groups
        .values()
        .find(|p| p.group_name == "my-cluster-pg")
        .expect("cluster pg should exist");
    assert_eq!(cluster_pg.strategy, "cluster");
    assert_eq!(cluster_pg.tags.get("Env").map(|s| s.as_str()), Some("test"));
    let partition_pg = view
        .placement_groups
        .values()
        .find(|p| p.group_name == "my-partition-pg")
        .expect("partition pg should exist");
    assert_eq!(partition_pg.strategy, "partition");
    assert_eq!(partition_pg.partition_count, Some(4));

    // Verify extract round-trip.
    let converter = AwsPlacementGroupConverter::new(Arc::clone(&svc));
    let extracted = converter
        .extract(&ctx)
        .await
        .expect("extract should succeed");
    assert_eq!(extracted.len(), 2);
    let names: Vec<_> = extracted
        .iter()
        .filter_map(|r| r.attributes.get("name").and_then(|v| v.as_str()))
        .collect();
    assert!(names.contains(&"my-cluster-pg"));
    assert!(names.contains(&"my-partition-pg"));
}

#[tokio::test]
async fn test_inject_placement_group_preserves_pg_id_for_describe_by_id() {
    use winterbaume_terraform::converters::ec2::AwsPlacementGroupConverter;

    let svc = Arc::new(Ec2Service::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsPlacementGroupConverter::new(Arc::clone(&svc)));

    // tfstate carries the name in `id` (matches what extract emits) and the
    // real `pg-...` identifier in `placement_group_id`. The injector should
    // use the `placement_group_id` value as the AWS GroupId so that SDK
    // calls filtering by GroupId can find the resource.
    let tfstate = make_tfstate(vec![make_resource(
        "aws_placement_group",
        "cluster_pg",
        json!({
            "id": "my-cluster-pg",
            "name": "my-cluster-pg",
            "placement_group_id": "pg-1234567890abcdef0",
            "strategy": "cluster",
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    // Verify that the in-memory state stored the `pg-...` identifier as the
    // GroupId and the human-friendly name as the GroupName.
    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert_eq!(view.placement_groups.len(), 1);
    let pg = view
        .placement_groups
        .get("pg-1234567890abcdef0")
        .expect("placement group keyed by pg-... id should exist");
    assert_eq!(pg.group_id, "pg-1234567890abcdef0");
    assert_eq!(pg.group_name, "my-cluster-pg");

    // Bridge state into a fresh service for SDK verification.
    let ec2_for_mock = Ec2Service::new();
    ec2_for_mock
        .restore(&ctx.default_account_id, "us-east-1", view)
        .await
        .unwrap();

    let mock = MockAws::builder().with_service(ec2_for_mock).build();
    let config = mock.sdk_config("us-east-1").await;
    let client = aws_sdk_ec2::Client::new(&config);

    // DescribePlacementGroups filtered by GroupId.N must locate the group by
    // its `pg-...` identifier.
    let resp = client
        .describe_placement_groups()
        .group_ids("pg-1234567890abcdef0")
        .send()
        .await
        .expect("describe_placement_groups by id should succeed");
    let groups = resp.placement_groups();
    assert_eq!(
        groups.len(),
        1,
        "expected exactly one placement group filtered by pg-... id"
    );
    assert_eq!(groups[0].group_id(), Some("pg-1234567890abcdef0"));
    assert_eq!(groups[0].group_name(), Some("my-cluster-pg"));

    // Round-trip: the extract side should re-emit the same `pg-...` id under
    // `placement_group_id` and the friendly name under `id`/`name`.
    let converter = AwsPlacementGroupConverter::new(Arc::clone(&svc));
    let extracted = converter
        .extract(&ctx)
        .await
        .expect("extract should succeed");
    assert_eq!(extracted.len(), 1);
    let attrs = &extracted[0].attributes;
    assert_eq!(
        attrs.get("placement_group_id").and_then(|v| v.as_str()),
        Some("pg-1234567890abcdef0")
    );
    assert_eq!(
        attrs.get("id").and_then(|v| v.as_str()),
        Some("my-cluster-pg")
    );
    assert_eq!(
        attrs.get("name").and_then(|v| v.as_str()),
        Some("my-cluster-pg")
    );
}

// ---------------------------------------------------------------------------
// Neptune cluster instance injection test
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_neptune_cluster_instance() {
    use winterbaume_terraform::converters::neptune::{
        AwsNeptuneClusterConverter, AwsNeptuneClusterInstanceConverter,
    };

    let svc = Arc::new(NeptuneService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsNeptuneClusterConverter::new(Arc::clone(&svc)));
    injector.register(AwsNeptuneClusterInstanceConverter::new(Arc::clone(&svc)));

    let tfstate = make_tfstate(vec![
        make_resource(
            "aws_neptune_cluster",
            "cl",
            json!({
                "cluster_identifier": "my-cluster",
                "engine": "neptune",
            }),
        ),
        make_resource(
            "aws_neptune_cluster_instance",
            "inst",
            json!({
                "identifier": "my-instance",
                "cluster_identifier": "my-cluster",
                "instance_class": "db.r5.large",
            }),
        ),
    ]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 2);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.db_instances.contains_key("my-instance"));
    let inst = &view.db_instances["my-instance"];
    assert_eq!(inst.db_cluster_identifier, Some("my-cluster".to_string()));
}

// ---------------------------------------------------------------------------
// Neptune parameter group injection test
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_neptune_parameter_group() {
    use winterbaume_terraform::converters::neptune::AwsNeptuneParameterGroupConverter;

    let svc = Arc::new(NeptuneService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsNeptuneParameterGroupConverter::new(Arc::clone(&svc)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_neptune_parameter_group",
        "pg",
        json!({
            "name": "my-pg",
            "family": "neptune1.2",
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.db_parameter_groups.contains_key("my-pg"));
    let pg = &view.db_parameter_groups["my-pg"];
    assert_eq!(pg.family, "neptune1.2");
}

// ---------------------------------------------------------------------------
// Network Firewall firewall policy injection test
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_networkfirewall_firewall_policy() {
    use winterbaume_networkfirewall::NetworkFirewallService;
    use winterbaume_terraform::converters::networkfirewall::AwsNetworkFirewallFirewallPolicyConverter;

    let svc = Arc::new(NetworkFirewallService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsNetworkFirewallFirewallPolicyConverter::new(Arc::clone(
        &svc,
    )));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_networkfirewall_firewall_policy",
        "pol",
        json!({
            "name": "my-policy",
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert_eq!(view.firewall_policies.len(), 1);
    let fp = view.firewall_policies.values().next().unwrap();
    assert_eq!(fp.firewall_policy_name, "my-policy");
}

// ---------------------------------------------------------------------------
// Network Firewall rule group injection test
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_networkfirewall_rule_group() {
    use winterbaume_networkfirewall::NetworkFirewallService;
    use winterbaume_terraform::converters::networkfirewall::AwsNetworkFirewallRuleGroupConverter;

    let svc = Arc::new(NetworkFirewallService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsNetworkFirewallRuleGroupConverter::new(Arc::clone(&svc)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_networkfirewall_rule_group",
        "rg",
        json!({
            "name": "my-rg",
            "capacity": 100,
            "type": "STATELESS",
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert_eq!(view.rule_groups.len(), 1);
    let rg = view.rule_groups.values().next().unwrap();
    assert_eq!(rg.rule_group_name, "my-rg");
    assert_eq!(rg.rule_group_type, "STATELESS");
    assert_eq!(rg.capacity, 100);
}

// ---------------------------------------------------------------------------
// Network Manager device injection test
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_networkmanager_device() {
    use winterbaume_networkmanager::NetworkManagerService;
    use winterbaume_terraform::converters::networkmanager::{
        AwsNetworkmanagerDeviceConverter, AwsNetworkmanagerGlobalNetworkConverter,
    };

    let svc = Arc::new(NetworkManagerService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsNetworkmanagerGlobalNetworkConverter::new(Arc::clone(
        &svc,
    )));
    injector.register(AwsNetworkmanagerDeviceConverter::new(Arc::clone(&svc)));

    let tfstate = make_tfstate(vec![
        make_resource(
            "aws_networkmanager_global_network",
            "gn",
            json!({
                "id": "gn-001",
            }),
        ),
        make_resource(
            "aws_networkmanager_device",
            "dev",
            json!({
                "id": "dev-001",
                "global_network_id": "gn-001",
            }),
        ),
    ]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 2);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.devices.contains_key("dev-001"));
    assert_eq!(view.devices["dev-001"].global_network_id, "gn-001");
}

// ---------------------------------------------------------------------------
// Network Manager site injection test
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_networkmanager_site() {
    use winterbaume_networkmanager::NetworkManagerService;
    use winterbaume_terraform::converters::networkmanager::{
        AwsNetworkmanagerGlobalNetworkConverter, AwsNetworkmanagerSiteConverter,
    };

    let svc = Arc::new(NetworkManagerService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsNetworkmanagerGlobalNetworkConverter::new(Arc::clone(
        &svc,
    )));
    injector.register(AwsNetworkmanagerSiteConverter::new(Arc::clone(&svc)));

    let tfstate = make_tfstate(vec![
        make_resource(
            "aws_networkmanager_global_network",
            "gn",
            json!({
                "id": "gn-002",
            }),
        ),
        make_resource(
            "aws_networkmanager_site",
            "site",
            json!({
                "id": "site-001",
                "global_network_id": "gn-002",
            }),
        ),
    ]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 2);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.sites.contains_key("site-001"));
    assert_eq!(view.sites["site-001"].global_network_id, "gn-002");
}

// ---------------------------------------------------------------------------
// Organizations account injection test
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_organizations_account() {
    use winterbaume_organizations::OrganizationsService;
    use winterbaume_terraform::converters::organizations::AwsOrganizationsAccountConverter;

    let svc = Arc::new(OrganizationsService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsOrganizationsAccountConverter::new(Arc::clone(&svc)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_organizations_account",
        "dev",
        json!({
            "name": "dev-account",
            "email": "dev@example.com",
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert_eq!(view.accounts.len(), 1);
    let acct = view.accounts.values().next().unwrap();
    assert_eq!(acct.name, "dev-account");
    assert_eq!(acct.email, "dev@example.com");
}

// ---------------------------------------------------------------------------
// Organizations organizational unit injection test
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_organizations_organizational_unit() {
    use winterbaume_organizations::OrganizationsService;
    use winterbaume_terraform::converters::organizations::AwsOrganizationsOuConverter;

    let svc = Arc::new(OrganizationsService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsOrganizationsOuConverter::new(Arc::clone(&svc)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_organizations_organizational_unit",
        "eng",
        json!({
            "name": "Engineering",
            "parent_id": "r-root",
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert_eq!(view.ous.len(), 1);
    let ou = view.ous.values().next().unwrap();
    assert_eq!(ou.name, "Engineering");
}

// ---------------------------------------------------------------------------
// QuickSight data source injection test
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_quicksight_data_source() {
    use winterbaume_quicksight::QuickSightService;
    use winterbaume_terraform::converters::quicksight::AwsQuicksightDataSourceConverter;

    let svc = Arc::new(QuickSightService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsQuicksightDataSourceConverter::new(Arc::clone(&svc)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_quicksight_data_source",
        "ds",
        json!({
            "data_source_id": "ds-1",
            "name": "my-ds",
            "type": "S3",
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.data_sources.contains_key("ds-1"));
    let ds = &view.data_sources["ds-1"];
    assert_eq!(ds.name, "my-ds");
    assert_eq!(ds.r#type, "S3");
}

// ---------------------------------------------------------------------------
// QuickSight user injection test
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_quicksight_user() {
    use winterbaume_quicksight::QuickSightService;
    use winterbaume_terraform::converters::quicksight::AwsQuicksightUserConverter;

    let svc = Arc::new(QuickSightService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsQuicksightUserConverter::new(Arc::clone(&svc)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_quicksight_user",
        "user",
        json!({
            "user_name": "jdoe",
            "email": "j@example.com",
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    let ns_users = view.users.get("default").unwrap();
    assert!(ns_users.contains_key("jdoe"));
    assert_eq!(ns_users["jdoe"].email, "j@example.com");
}

// ---------------------------------------------------------------------------
// RDS cluster injection test
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_rds_cluster() {
    use winterbaume_terraform::converters::rds::AwsRdsClusterConverter;

    let svc = Arc::new(RdsService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsRdsClusterConverter::new(Arc::clone(&svc)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_rds_cluster",
        "cl",
        json!({
            "cluster_identifier": "my-cluster",
            "engine": "aurora-mysql",
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.db_clusters.contains_key("my-cluster"));
    let cl = &view.db_clusters["my-cluster"];
    assert_eq!(cl.engine, "aurora-mysql");
}

// ---------------------------------------------------------------------------
// RDS cluster parameter group injection test
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_rds_cluster_parameter_group() {
    use winterbaume_terraform::converters::rds::AwsRdsClusterParameterGroupConverter;

    let svc = Arc::new(RdsService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsRdsClusterParameterGroupConverter::new(Arc::clone(&svc)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_rds_cluster_parameter_group",
        "cpg",
        json!({
            "name": "my-cpg",
            "family": "aurora-mysql8.0",
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.db_cluster_parameter_groups.contains_key("my-cpg"));
    let cpg = &view.db_cluster_parameter_groups["my-cpg"];
    assert_eq!(cpg.family, "aurora-mysql8.0");
}

// ---------------------------------------------------------------------------
// EC2 route injection test
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_route() {
    use winterbaume_terraform::converters::ec2::{
        AwsRouteConverter, AwsRouteTableConverter, AwsVpcConverter,
    };

    let svc = Arc::new(Ec2Service::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsVpcConverter::new(Arc::clone(&svc)));
    injector.register(AwsRouteTableConverter::new(Arc::clone(&svc)));
    injector.register(AwsRouteConverter::new(Arc::clone(&svc)));

    let tfstate = make_tfstate(vec![
        make_resource(
            "aws_vpc",
            "vpc",
            json!({
                "id": "vpc-111",
                "cidr_block": "10.0.0.0/16",
            }),
        ),
        make_resource(
            "aws_route_table",
            "rtb",
            json!({
                "id": "rtb-222",
                "vpc_id": "vpc-111",
            }),
        ),
        make_resource(
            "aws_route",
            "r",
            json!({
                "route_table_id": "rtb-222",
                "destination_cidr_block": "0.0.0.0/0",
                "gateway_id": "igw-999",
            }),
        ),
    ]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 3);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    let rtb = &view.route_tables["rtb-222"];
    assert!(
        rtb.routes
            .iter()
            .any(|r| r.destination_cidr_block == Some("0.0.0.0/0".to_string()))
    );
}

// ---------------------------------------------------------------------------
// EC2 route table injection test
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_route_table() {
    use winterbaume_terraform::converters::ec2::{AwsRouteTableConverter, AwsVpcConverter};

    let svc = Arc::new(Ec2Service::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsVpcConverter::new(Arc::clone(&svc)));
    injector.register(AwsRouteTableConverter::new(Arc::clone(&svc)));

    let tfstate = make_tfstate(vec![
        make_resource(
            "aws_vpc",
            "vpc",
            json!({
                "id": "vpc-rt1",
                "cidr_block": "10.0.0.0/16",
            }),
        ),
        make_resource(
            "aws_route_table",
            "rtb",
            json!({
                "id": "rtb-rt1",
                "vpc_id": "vpc-rt1",
            }),
        ),
    ]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 2);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.route_tables.contains_key("rtb-rt1"));
    assert_eq!(view.route_tables["rtb-rt1"].vpc_id, "vpc-rt1");
}

// ---------------------------------------------------------------------------
// Route53 Domains registered domain injection test
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_route53domains_registered_domain() {
    use winterbaume_route53domains::Route53DomainsService;
    use winterbaume_terraform::converters::route53domains::AwsRoute53DomainsRegisteredDomainConverter;

    let svc = Arc::new(Route53DomainsService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsRoute53DomainsRegisteredDomainConverter::new(Arc::clone(
        &svc,
    )));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_route53domains_registered_domain",
        "dom",
        json!({
            "domain_name": "example.com",
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.domains.contains_key("example.com"));
}

// ---------------------------------------------------------------------------
// S3 Control access point injection test
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_s3control_access_point() {
    use winterbaume_s3control::S3ControlService;
    use winterbaume_terraform::converters::s3control::AwsS3controlAccessPointConverter;

    let svc = Arc::new(S3ControlService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsS3controlAccessPointConverter::new(Arc::clone(&svc)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_s3control_access_point",
        "ap",
        json!({
            "name": "my-ap",
            "bucket": "my-bucket",
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.access_points.contains_key("my-ap"));
    assert_eq!(view.access_points["my-ap"].bucket, "my-bucket");
}

// ---------------------------------------------------------------------------
// S3 Tables namespace injection test
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_s3tables_namespace() {
    use winterbaume_s3tables::S3TablesService;
    use winterbaume_terraform::converters::s3tables::AwsS3tablesNamespaceConverter;

    let svc = Arc::new(S3TablesService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsS3tablesNamespaceConverter::new(Arc::clone(&svc)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_s3tables_namespace",
        "ns",
        json!({
            "namespace": "my-ns",
            "table_bucket_arn": "arn:aws:s3tables:us-east-1:123456789012:bucket/test-bucket",
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert_eq!(view.namespaces.len(), 1);
    assert_eq!(view.namespaces[0].name, "my-ns");
    assert_eq!(
        view.namespaces[0].table_bucket_arn,
        "arn:aws:s3tables:us-east-1:123456789012:bucket/test-bucket"
    );
}

// ---------------------------------------------------------------------------
// SageMaker endpoint configuration injection test
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_sagemaker_endpoint_configuration() {
    use winterbaume_terraform::converters::sagemaker::AwsSagemakerEndpointConfigurationConverter;

    let svc = Arc::new(SageMakerService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsSagemakerEndpointConfigurationConverter::new(Arc::clone(
        &svc,
    )));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_sagemaker_endpoint_configuration",
        "epc",
        json!({
            "name": "my-epc",
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.endpoint_configs.contains_key("my-epc"));
}

// ---------------------------------------------------------------------------
// SageMaker model injection test
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_sagemaker_model() {
    use winterbaume_terraform::converters::sagemaker::AwsSagemakerModelConverter;

    let svc = Arc::new(SageMakerService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsSagemakerModelConverter::new(Arc::clone(&svc)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_sagemaker_model",
        "mdl",
        json!({
            "name": "my-model",
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.models.contains_key("my-model"));
}

// ---------------------------------------------------------------------------
// SecurityHub standards subscription injection test
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_securityhub_standards_subscription() {
    use winterbaume_terraform::converters::securityhub::AwsSecurityhubStandardsSubscriptionConverter;

    let svc = Arc::new(SecurityHubService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsSecurityhubStandardsSubscriptionConverter::new(
        Arc::clone(&svc),
    ));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_securityhub_standards_subscription",
        "sub",
        json!({
            "standards_arn": "arn:aws:securityhub:::standards/aws-foundational-security-best-practices/v/1.0.0",
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(!view.enabled_standards.is_empty());
    assert_eq!(
        view.enabled_standards[0].standards_arn,
        "arn:aws:securityhub:::standards/aws-foundational-security-best-practices/v/1.0.0"
    );
}

// ---------------------------------------------------------------------------
// Service Discovery service injection test
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_service_discovery_service() {
    use winterbaume_servicediscovery::ServiceDiscoveryService;
    use winterbaume_terraform::converters::servicediscovery::{
        AwsServiceDiscoveryPrivateDnsNamespaceConverter, AwsServiceDiscoveryServiceConverter,
    };

    let svc = Arc::new(ServiceDiscoveryService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsServiceDiscoveryPrivateDnsNamespaceConverter::new(
        Arc::clone(&svc),
    ));
    injector.register(AwsServiceDiscoveryServiceConverter::new(Arc::clone(&svc)));

    let tfstate = make_tfstate(vec![
        make_resource(
            "aws_service_discovery_private_dns_namespace",
            "ns",
            json!({
                "id": "ns-001",
                "name": "local",
                "vpc": "vpc-123",
            }),
        ),
        make_resource(
            "aws_service_discovery_service",
            "svc",
            json!({
                "id": "srv-001",
                "name": "my-service",
                "namespace_id": "ns-001",
            }),
        ),
    ]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 2);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.services.contains_key("srv-001"));
    assert_eq!(view.services["srv-001"].name, "my-service");
}

// ---------------------------------------------------------------------------
// Timestream InfluxDB instance injection test
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_timestreaminfluxdb_db_instance() {
    use winterbaume_terraform::converters::timestreaminfluxdb::AwsTimestreaminfluxdbDbInstanceConverter;
    use winterbaume_timestreaminfluxdb::TimestreamInfluxDbService;

    let svc = Arc::new(TimestreamInfluxDbService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsTimestreaminfluxdbDbInstanceConverter::new(Arc::clone(
        &svc,
    )));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_timestreaminfluxdb_db_instance",
        "inst",
        json!({
            "name": "my-instance",
            "db_instance_type": "db.influx.medium",
            "allocated_storage": 100,
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert_eq!(view.db_instances.len(), 1);
    let inst = view.db_instances.values().next().unwrap();
    assert_eq!(inst.name, "my-instance");
    assert_eq!(inst.db_instance_type, "db.influx.medium");
    assert_eq!(inst.allocated_storage, 100);
}

// ---------------------------------------------------------------------------
// Timestream Query scheduled query injection test
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_timestreamquery_scheduled_query() {
    use winterbaume_terraform::converters::timestreamquery::AwsTimestreamQueryScheduledQueryConverter;
    use winterbaume_timestreamquery::TimestreamQueryService;

    let svc = Arc::new(TimestreamQueryService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsTimestreamQueryScheduledQueryConverter::new(Arc::clone(
        &svc,
    )));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_timestreamquery_scheduled_query",
        "sq",
        json!({
            "name": "my-query",
            "query_string": "SELECT 1",
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert_eq!(view.scheduled_queries.len(), 1);
    let sq = view.scheduled_queries.values().next().unwrap();
    assert_eq!(sq.name, "my-query");
    assert_eq!(sq.query_string, "SELECT 1");
}

// ---------------------------------------------------------------------------
// Transcribe language model injection test
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_transcribe_language_model() {
    use winterbaume_terraform::converters::transcribe::AwsTranscribeLanguageModelConverter;
    use winterbaume_transcribe::TranscribeService;

    let svc = Arc::new(TranscribeService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsTranscribeLanguageModelConverter::new(Arc::clone(&svc)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_transcribe_language_model",
        "lm",
        json!({
            "model_name": "my-model",
            "language_code": "en-US",
            "base_model_name": "NarrowBand",
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.vocabularies.contains_key("language-model:my-model"));
}

// ---------------------------------------------------------------------------
// VPC Lattice listener injection test
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_vpclattice_listener() {
    use winterbaume_terraform::converters::vpclattice::{
        AwsVpcLatticeListenerConverter, AwsVpcLatticeServiceConverter,
    };
    use winterbaume_vpclattice::VpcLatticeService;

    let svc = Arc::new(VpcLatticeService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsVpcLatticeServiceConverter::new(Arc::clone(&svc)));
    injector.register(AwsVpcLatticeListenerConverter::new(Arc::clone(&svc)));

    let tfstate = make_tfstate(vec![
        make_resource(
            "aws_vpclattice_service",
            "svc",
            json!({
                "id": "svc-001",
                "name": "my-service",
            }),
        ),
        make_resource(
            "aws_vpclattice_listener",
            "listener",
            json!({
                "id": "listener-001",
                "name": "my-listener",
                "service_identifier": "svc-001",
                "port": 80,
                "protocol": "HTTP",
            }),
        ),
    ]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 2);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.listeners.contains_key("listener-001"));
    let l = &view.listeners["listener-001"];
    assert_eq!(l.name, "my-listener");
    assert_eq!(l.port, Some(80));
    assert_eq!(l.protocol, "HTTP");
}

// ---------------------------------------------------------------------------
// S3 Control bucket injection test
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_s3control_bucket() {
    use winterbaume_s3control::S3ControlService;
    use winterbaume_terraform::converters::s3control::AwsS3controlBucketConverter;

    let svc = Arc::new(S3ControlService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsS3controlBucketConverter::new(Arc::clone(&svc)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_s3control_bucket",
        "bkt",
        json!({
            "bucket": "my-outpost-bucket",
            "outpost_id": "op-123",
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.outposts_buckets.contains_key("my-outpost-bucket"));
    assert_eq!(
        view.outposts_buckets["my-outpost-bucket"].outpost_id,
        "op-123"
    );
}

// ---------------------------------------------------------------------------
// GuardDuty member injection test
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_guardduty_member() {
    use winterbaume_guardduty::GuardDutyService;
    use winterbaume_terraform::converters::guardduty::{
        AwsGuarddutyDetectorConverter, AwsGuarddutyMemberConverter,
    };

    let svc = Arc::new(GuardDutyService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsGuarddutyDetectorConverter::new(Arc::clone(&svc)));
    injector.register(AwsGuarddutyMemberConverter::new(Arc::clone(&svc)));

    let tfstate = make_tfstate(vec![
        make_resource(
            "aws_guardduty_detector",
            "det",
            json!({
                "id": "detector-001",
                "enable": "true",
            }),
        ),
        make_resource(
            "aws_guardduty_member",
            "member",
            json!({
                "detector_id": "detector-001",
                "account_id": "222233334444",
                "email": "member@example.com",
            }),
        ),
    ]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 2);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.detectors.contains_key("detector-001"));
}

// ---------------------------------------------------------------------------
// Account alternate contact injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_account_alternate_contact() {
    use winterbaume_account::AccountService;
    use winterbaume_terraform::converters::account::AwsAccountAlternateContactConverter;

    let svc = Arc::new(AccountService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsAccountAlternateContactConverter::new(Arc::clone(&svc)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_account_alternate_contact",
        "billing",
        json!({
            "alternate_contact_type": "BILLING",
            "name": "Jane",
            "email_address": "jane@example.com",
            "phone_number": "555-1234",
            "title": "Billing Admin",
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    let contact = &view.alternate_contacts["BILLING"];
    assert_eq!(contact.name, "Jane");
    assert_eq!(contact.email_address, "jane@example.com");
    assert_eq!(contact.phone_number, "555-1234");
    assert_eq!(contact.title, "Billing Admin");
}

// ---------------------------------------------------------------------------
// API Gateway additional injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_api_gateway_api_key() {
    use winterbaume_terraform::converters::apigateway::AwsApiGatewayApiKeyConverter;

    let svc = Arc::new(ApiGatewayService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsApiGatewayApiKeyConverter::new(Arc::clone(&svc)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_api_gateway_api_key",
        "my_key",
        json!({
            "id": "key-001",
            "name": "my-key",
            "enabled": true,
            "value": "abc123",
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    let key = &view.api_keys["key-001"];
    assert_eq!(key.name, "my-key");
    assert!(key.enabled);
    assert_eq!(key.value, "abc123");
}

#[tokio::test]
async fn test_inject_api_gateway_deployment() {
    use winterbaume_terraform::converters::apigateway::{
        AwsApiGatewayDeploymentConverter, AwsApiGatewayRestApiConverter,
    };

    let svc = Arc::new(ApiGatewayService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsApiGatewayRestApiConverter::new(Arc::clone(&svc)));
    injector.register(AwsApiGatewayDeploymentConverter::new(Arc::clone(&svc)));

    let tfstate = make_tfstate(vec![
        make_resource(
            "aws_api_gateway_rest_api",
            "api",
            json!({
                "id": "api-001",
                "name": "my-api",
            }),
        ),
        make_resource(
            "aws_api_gateway_deployment",
            "deploy",
            json!({
                "id": "dep-001",
                "rest_api_id": "api-001",
                "description": "Initial deployment",
            }),
        ),
    ]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 2);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    let dep = &view.deployments["api-001/dep-001"];
    assert_eq!(dep.rest_api_id, "api-001");
    assert_eq!(dep.description, Some("Initial deployment".to_string()));
}

#[tokio::test]
async fn test_inject_api_gateway_method() {
    use winterbaume_terraform::converters::apigateway::{
        AwsApiGatewayMethodConverter, AwsApiGatewayResourceConverter, AwsApiGatewayRestApiConverter,
    };

    let svc = Arc::new(ApiGatewayService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsApiGatewayRestApiConverter::new(Arc::clone(&svc)));
    injector.register(AwsApiGatewayResourceConverter::new(Arc::clone(&svc)));
    injector.register(AwsApiGatewayMethodConverter::new(Arc::clone(&svc)));

    let tfstate = make_tfstate(vec![
        make_resource(
            "aws_api_gateway_rest_api",
            "api",
            json!({ "id": "api-001", "name": "my-api" }),
        ),
        make_resource(
            "aws_api_gateway_resource",
            "res",
            json!({
                "id": "res-001",
                "rest_api_id": "api-001",
                "path_part": "items",
            }),
        ),
        make_resource(
            "aws_api_gateway_method",
            "get",
            json!({
                "rest_api_id": "api-001",
                "resource_id": "res-001",
                "http_method": "GET",
                "authorization": "NONE",
            }),
        ),
    ]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 3);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    let res = &view.resources["api-001/res-001"];
    assert!(res.methods.contains_key("GET"));
    assert_eq!(res.methods["GET"].authorization_type, "NONE");
}

#[tokio::test]
async fn test_inject_api_gateway_resource() {
    use winterbaume_terraform::converters::apigateway::{
        AwsApiGatewayResourceConverter, AwsApiGatewayRestApiConverter,
    };

    let svc = Arc::new(ApiGatewayService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsApiGatewayRestApiConverter::new(Arc::clone(&svc)));
    injector.register(AwsApiGatewayResourceConverter::new(Arc::clone(&svc)));

    let tfstate = make_tfstate(vec![
        make_resource(
            "aws_api_gateway_rest_api",
            "api",
            json!({ "id": "api-001", "name": "my-api", "root_resource_id": "root-001" }),
        ),
        make_resource(
            "aws_api_gateway_resource",
            "users",
            json!({
                "id": "res-002",
                "rest_api_id": "api-001",
                "path_part": "users",
                "parent_id": "root-001",
            }),
        ),
    ]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 2);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    let res = &view.resources["api-001/res-002"];
    assert_eq!(res.path_part, Some("users".to_string()));
    assert_eq!(res.parent_id, Some("root-001".to_string()));
}

// ---------------------------------------------------------------------------
// Application Auto Scaling injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_appautoscaling_target() {
    use winterbaume_applicationautoscaling::ApplicationAutoScalingService;
    use winterbaume_terraform::converters::applicationautoscaling::AwsAppautoscalingTargetConverter;

    let svc = Arc::new(ApplicationAutoScalingService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsAppautoscalingTargetConverter::new(Arc::clone(&svc)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_appautoscaling_target",
        "ecs",
        json!({
            "service_namespace": "ecs",
            "resource_id": "service/cluster/svc",
            "scalable_dimension": "ecs:service:DesiredCount",
            "min_capacity": 1,
            "max_capacity": 10,
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert_eq!(view.scalable_targets.len(), 1);
    let target = &view.scalable_targets[0];
    assert_eq!(target.service_namespace, "ecs");
    assert_eq!(target.resource_id, "service/cluster/svc");
    assert_eq!(target.min_capacity, 1);
    assert_eq!(target.max_capacity, 10);
}

#[tokio::test]
async fn test_inject_appautoscaling_policy() {
    use winterbaume_applicationautoscaling::ApplicationAutoScalingService;
    use winterbaume_terraform::converters::applicationautoscaling::{
        AwsAppautoscalingPolicyConverter, AwsAppautoscalingTargetConverter,
    };

    let svc = Arc::new(ApplicationAutoScalingService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsAppautoscalingTargetConverter::new(Arc::clone(&svc)));
    injector.register(AwsAppautoscalingPolicyConverter::new(Arc::clone(&svc)));

    let tfstate = make_tfstate(vec![
        make_resource(
            "aws_appautoscaling_target",
            "ecs",
            json!({
                "service_namespace": "ecs",
                "resource_id": "service/cluster/svc",
                "scalable_dimension": "ecs:service:DesiredCount",
                "min_capacity": 1,
                "max_capacity": 10,
            }),
        ),
        make_resource(
            "aws_appautoscaling_policy",
            "scale_up",
            json!({
                "name": "scale-up",
                "service_namespace": "ecs",
                "resource_id": "service/cluster/svc",
                "scalable_dimension": "ecs:service:DesiredCount",
                "policy_type": "TargetTrackingScaling",
            }),
        ),
    ]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 2);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert_eq!(view.scaling_policies.len(), 1);
    assert_eq!(view.scaling_policies[0].policy_name, "scale-up");
}

// ---------------------------------------------------------------------------
// AppConfig configuration profile injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_appconfig_configuration_profile() {
    use winterbaume_terraform::converters::appconfig::{
        AwsAppconfigApplicationConverter, AwsAppconfigConfigurationProfileConverter,
    };

    let svc = Arc::new(AppConfigService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsAppconfigApplicationConverter::new(Arc::clone(&svc)));
    injector.register(AwsAppconfigConfigurationProfileConverter::new(Arc::clone(
        &svc,
    )));

    let tfstate = make_tfstate(vec![
        make_resource(
            "aws_appconfig_application",
            "app",
            json!({ "id": "app-001", "name": "my-app" }),
        ),
        make_resource(
            "aws_appconfig_configuration_profile",
            "profile",
            json!({
                "configuration_profile_id": "prof-001",
                "application_id": "app-001",
                "name": "my-profile",
                "location_uri": "hosted",
            }),
        ),
    ]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 2);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert_eq!(view.configuration_profiles.len(), 1);
    assert_eq!(view.configuration_profiles[0].name, "my-profile");
    assert_eq!(view.configuration_profiles[0].location_uri, "hosted");
}

// ---------------------------------------------------------------------------
// App Mesh injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_appmesh_virtual_node() {
    use winterbaume_appmesh::AppMeshService;
    use winterbaume_terraform::converters::appmesh::{
        AwsAppmeshMeshConverter, AwsAppmeshVirtualNodeConverter,
    };

    let svc = Arc::new(AppMeshService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsAppmeshMeshConverter::new(Arc::clone(&svc)));
    injector.register(AwsAppmeshVirtualNodeConverter::new(Arc::clone(&svc)));

    let tfstate = make_tfstate(vec![
        make_resource("aws_appmesh_mesh", "mesh", json!({ "name": "my-mesh" })),
        make_resource(
            "aws_appmesh_virtual_node",
            "node",
            json!({
                "mesh_name": "my-mesh",
                "name": "my-node",
            }),
        ),
    ]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 2);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.virtual_nodes.contains_key("my-mesh/my-node"));
    let vn = &view.virtual_nodes["my-mesh/my-node"];
    assert_eq!(vn.virtual_node_name, "my-node");
    assert_eq!(vn.mesh_name, "my-mesh");
}

#[tokio::test]
async fn test_inject_appmesh_virtual_service() {
    use winterbaume_appmesh::AppMeshService;
    use winterbaume_terraform::converters::appmesh::{
        AwsAppmeshMeshConverter, AwsAppmeshVirtualServiceConverter,
    };

    let svc = Arc::new(AppMeshService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsAppmeshMeshConverter::new(Arc::clone(&svc)));
    injector.register(AwsAppmeshVirtualServiceConverter::new(Arc::clone(&svc)));

    let tfstate = make_tfstate(vec![
        make_resource("aws_appmesh_mesh", "mesh", json!({ "name": "my-mesh" })),
        make_resource(
            "aws_appmesh_virtual_service",
            "vs",
            json!({
                "mesh_name": "my-mesh",
                "name": "my-service.local",
            }),
        ),
    ]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 2);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(
        view.virtual_services
            .contains_key("my-mesh/my-service.local")
    );
    let vs = &view.virtual_services["my-mesh/my-service.local"];
    assert_eq!(vs.virtual_service_name, "my-service.local");
}

// ---------------------------------------------------------------------------
// Auto Scaling schedule injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_autoscaling_schedule() {
    use winterbaume_terraform::converters::autoscaling::AwsAutoscalingScheduleConverter;

    let svc = Arc::new(AutoScalingService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsAutoscalingScheduleConverter::new(Arc::clone(&svc)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_autoscaling_schedule",
        "morning",
        json!({
            "scheduled_action_name": "scale-up-morning",
            "autoscaling_group_name": "my-asg",
            "recurrence": "0 8 * * *",
            "min_size": 2,
            "max_size": 10,
            "desired_capacity": 5,
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    let sa = &view.scheduled_actions["scale-up-morning"];
    assert_eq!(sa.group_name, "my-asg");
    assert_eq!(sa.recurrence, Some("0 8 * * *".to_string()));
    assert_eq!(sa.desired_capacity, Some(5));
}

// ---------------------------------------------------------------------------
// Batch job definition and scheduling policy injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_batch_job_definition() {
    use winterbaume_terraform::converters::batch::AwsBatchJobDefinitionConverter;

    let svc = Arc::new(BatchService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsBatchJobDefinitionConverter::new(Arc::clone(&svc)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_batch_job_definition",
        "job",
        json!({
            "name": "my-job",
            "type": "container",
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.job_definitions.contains_key("my-job"));
    let jd = &view.job_definitions["my-job"][0];
    assert_eq!(jd.job_definition_name, "my-job");
    assert_eq!(jd.job_definition_type, "container");
}

#[tokio::test]
async fn test_inject_batch_scheduling_policy() {
    use winterbaume_terraform::converters::batch::AwsBatchSchedulingPolicyConverter;

    let svc = Arc::new(BatchService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsBatchSchedulingPolicyConverter::new(Arc::clone(&svc)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_batch_scheduling_policy",
        "policy",
        json!({
            "name": "my-policy",
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert!(view.scheduling_policies.contains_key("my-policy"));
    assert_eq!(view.scheduling_policies["my-policy"].name, "my-policy");
}

// ---------------------------------------------------------------------------
// Bedrock model invocation logging injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_bedrock_model_invocation_logging() {
    use winterbaume_bedrock::BedrockService;
    use winterbaume_terraform::converters::bedrock::AwsBedrockModelInvocationLoggingConfigurationConverter;

    let svc = Arc::new(BedrockService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsBedrockModelInvocationLoggingConfigurationConverter::new(
        Arc::clone(&svc),
    ));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_bedrock_model_invocation_logging_configuration",
        "logging",
        json!({
            "logging_config": [{
                "text_data_delivery_enabled": true,
                "image_data_delivery_enabled": false,
                "embedding_data_delivery_enabled": true,
                "s3_configuration": [{
                    "bucket_name": "my-log-bucket",
                    "key_prefix": "bedrock-logs/",
                }],
            }],
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    let config = view.logging_config.unwrap();
    assert_eq!(config.text_data_delivery_enabled, Some(true));
    assert_eq!(config.image_data_delivery_enabled, Some(false));
    let s3 = config.s3_config.unwrap();
    assert_eq!(s3.bucket_name, "my-log-bucket");
}

// ---------------------------------------------------------------------------
// Bedrock Agent knowledge base injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_bedrockagent_knowledge_base() {
    use winterbaume_bedrockagent::BedrockAgentService;
    use winterbaume_terraform::converters::bedrockagent::AwsBedrockagentKnowledgeBaseConverter;

    let svc = Arc::new(BedrockAgentService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsBedrockagentKnowledgeBaseConverter::new(Arc::clone(&svc)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_bedrockagent_knowledge_base",
        "kb",
        json!({
            "name": "my-kb",
            "role_arn": "arn:aws:iam::123456789012:role/BedrockRole",
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert_eq!(view.knowledge_bases.len(), 1);
    let kb = view.knowledge_bases.values().next().unwrap();
    assert_eq!(kb.name, "my-kb");
    assert_eq!(kb.role_arn, "arn:aws:iam::123456789012:role/BedrockRole");
}

// ---------------------------------------------------------------------------
// Budgets injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_budgets_budget() {
    use winterbaume_budgets::BudgetsService;
    use winterbaume_terraform::converters::budgets::AwsBudgetsBudgetConverter;

    let svc = Arc::new(BudgetsService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsBudgetsBudgetConverter::new(Arc::clone(&svc)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_budgets_budget",
        "cost",
        json!({
            "name": "my-budget",
            "budget_type": "COST",
            "limit_amount": "100",
            "limit_unit": "USD",
            "time_unit": "MONTHLY",
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    let budget = &view.budgets["my-budget"];
    assert_eq!(budget.budget_type, "COST");
    assert_eq!(budget.budget_limit_amount, "100");
    assert_eq!(budget.budget_limit_unit, "USD");
    assert_eq!(budget.time_unit, "MONTHLY");
}

// ---------------------------------------------------------------------------
// Cost Explorer anomaly subscription injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_ce_anomaly_subscription() {
    use winterbaume_costexplorer::CostExplorerService;
    use winterbaume_terraform::converters::costexplorer::AwsCeAnomalySubscriptionConverter;

    let svc = Arc::new(CostExplorerService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsCeAnomalySubscriptionConverter::new(Arc::clone(&svc)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_ce_anomaly_subscription",
        "sub",
        json!({
            "name": "my-sub",
            "frequency": "DAILY",
            "monitor_arn_list": ["arn:aws:ce::123456789012:anomalymonitor/monitor-1"],
            "subscriber": [{
                "address": "alert@example.com",
                "type": "EMAIL",
            }],
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert_eq!(view.anomaly_subscriptions.len(), 1);
    let sub = view.anomaly_subscriptions.values().next().unwrap();
    assert_eq!(sub.subscription_name, "my-sub");
    assert_eq!(sub.frequency, "DAILY");
}

// ---------------------------------------------------------------------------
// CloudHSM V2 injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_cloudhsm_v2_cluster() {
    use winterbaume_cloudhsmv2::CloudHsmV2Service;
    use winterbaume_terraform::converters::cloudhsmv2::AwsCloudHsmV2ClusterConverter;

    let svc = Arc::new(CloudHsmV2Service::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsCloudHsmV2ClusterConverter::new(Arc::clone(&svc)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_cloudhsm_v2_cluster",
        "cluster",
        json!({
            "hsm_type": "hsm1.medium",
            "subnet_ids": ["subnet-1"],
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert_eq!(view.clusters.len(), 1);
    let cluster = view.clusters.values().next().unwrap();
    assert_eq!(cluster.hsm_type, "hsm1.medium");
    assert!(cluster.subnet_mapping.values().any(|s| s == "subnet-1"));
}

// ---------------------------------------------------------------------------
// Connect injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_connect_instance() {
    use winterbaume_connect::ConnectService;
    use winterbaume_terraform::converters::connect::AwsConnectInstanceConverter;

    let svc = Arc::new(ConnectService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsConnectInstanceConverter::new(Arc::clone(&svc)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_connect_instance",
        "main",
        json!({
            "id": "inst-001",
            "identity_management_type": "CONNECT_MANAGED",
            "instance_alias": "my-connect",
            "inbound_calls_enabled": true,
            "outbound_calls_enabled": true,
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    let inst = &view.instances["inst-001"];
    assert_eq!(inst.identity_management_type, "CONNECT_MANAGED");
    assert_eq!(inst.instance_alias, Some("my-connect".to_string()));
}

// ---------------------------------------------------------------------------
// DataSync injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_datasync_location_s3() {
    use winterbaume_datasync::DataSyncService;
    use winterbaume_terraform::converters::datasync::AwsDatasyncLocationS3Converter;

    let svc = Arc::new(DataSyncService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsDatasyncLocationS3Converter::new(Arc::clone(&svc)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_datasync_location_s3",
        "bucket",
        json!({
            "s3_bucket_arn": "arn:aws:s3:::mybucket",
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert_eq!(view.locations.len(), 1);
    let loc = view.locations.values().next().unwrap();
    assert!(loc.location_uri.contains("mybucket"));
}

// ---------------------------------------------------------------------------
// RDS injection tests (event subscription, option group, parameter group)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_db_event_subscription() {
    use winterbaume_terraform::converters::rds::AwsDbEventSubscriptionConverter;

    let svc = Arc::new(RdsService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsDbEventSubscriptionConverter::new(Arc::clone(&svc)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_db_event_subscription",
        "sub",
        json!({
            "name": "my-sub",
            "sns_topic": "arn:aws:sns:us-east-1:123456789012:my-topic",
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    let sub = &view.event_subscriptions["my-sub"];
    assert_eq!(sub.subscription_name, "my-sub");
    assert_eq!(
        sub.sns_topic_arn,
        "arn:aws:sns:us-east-1:123456789012:my-topic"
    );
}

#[tokio::test]
async fn test_inject_db_option_group() {
    use winterbaume_terraform::converters::rds::AwsDbOptionGroupConverter;

    let svc = Arc::new(RdsService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsDbOptionGroupConverter::new(Arc::clone(&svc)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_db_option_group",
        "og",
        json!({
            "name": "my-og",
            "engine_name": "mysql",
            "major_engine_version": "8.0",
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    let og = &view.option_groups["my-og"];
    assert_eq!(og.engine_name, "mysql");
    assert_eq!(og.major_engine_version, "8.0");
}

#[tokio::test]
async fn test_inject_db_parameter_group() {
    use winterbaume_terraform::converters::rds::AwsDbParameterGroupConverter;

    let svc = Arc::new(RdsService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsDbParameterGroupConverter::new(Arc::clone(&svc)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_db_parameter_group",
        "pg",
        json!({
            "name": "my-pg",
            "family": "postgres14",
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    let pg = &view.db_parameter_groups["my-pg"];
    assert_eq!(pg.family, "postgres14");
}

// ---------------------------------------------------------------------------
// DMS replication task injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_dms_replication_task() {
    use winterbaume_terraform::converters::dms::AwsDmsReplicationTaskConverter;

    let svc = Arc::new(DatabaseMigrationService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsDmsReplicationTaskConverter::new(Arc::clone(&svc)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_dms_replication_task",
        "task",
        json!({
            "replication_task_id": "my-task",
            "replication_instance_arn": "arn:aws:dms:us-east-1:123456789012:rep:INSTANCE",
            "source_endpoint_arn": "arn:aws:dms:us-east-1:123456789012:endpoint:SRC",
            "target_endpoint_arn": "arn:aws:dms:us-east-1:123456789012:endpoint:TGT",
            "migration_type": "full-load",
            "table_mappings": "{}",
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    let task = &view.replication_tasks["my-task"];
    assert_eq!(task.migration_type, "full-load");
    assert_eq!(task.replication_task_identifier, "my-task");
}

// ---------------------------------------------------------------------------
// EC2 EIP and Key Pair injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_eip() {
    use winterbaume_terraform::converters::ec2::AwsEipConverter;

    let svc = Arc::new(Ec2Service::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsEipConverter::new(Arc::clone(&svc)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_eip",
        "nat",
        json!({
            "id": "eipalloc-123",
            "allocation_id": "eipalloc-123",
            "public_ip": "1.2.3.4",
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    let eip = &view.elastic_ips["eipalloc-123"];
    assert_eq!(eip.public_ip, "1.2.3.4");
    assert_eq!(eip.allocation_id, "eipalloc-123");
}

#[tokio::test]
async fn test_inject_key_pair() {
    use winterbaume_terraform::converters::ec2::AwsKeyPairConverter;

    let svc = Arc::new(Ec2Service::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsKeyPairConverter::new(Arc::clone(&svc)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_key_pair",
        "deploy",
        json!({
            "key_name": "my-key",
            "key_pair_id": "key-123",
            "id": "key-123",
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    let kp = &view.key_pairs["my-key"];
    assert_eq!(kp.key_pair_id, "key-123");
    assert_eq!(kp.key_name, "my-key");
}

// ---------------------------------------------------------------------------
// FSx Windows file system injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_fsx_windows_file_system() {
    use winterbaume_terraform::converters::fsx::AwsFsxWindowsFileSystemConverter;

    let svc = Arc::new(FsxService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsFsxWindowsFileSystemConverter::new(Arc::clone(&svc)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_fsx_windows_file_system",
        "win",
        json!({
            "id": "fs-win-001",
            "storage_capacity": 100,
            "subnet_ids": ["subnet-1"],
            "storage_type": "SSD",
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    let fs = &view.file_systems["fs-win-001"];
    assert_eq!(fs.storage_capacity, 100);
    assert_eq!(fs.storage_type, "SSD");
    assert_eq!(fs.file_system_type, "WINDOWS");
}

// ---------------------------------------------------------------------------
// GuardDuty filter injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_guardduty_filter() {
    use winterbaume_guardduty::GuardDutyService;
    use winterbaume_terraform::converters::guardduty::AwsGuarddutyFilterConverter;

    let svc = Arc::new(GuardDutyService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsGuarddutyFilterConverter::new(Arc::clone(&svc)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_guardduty_filter",
        "f",
        json!({
            "name": "high-sev",
            "detector_id": "det-001",
            "action": "ARCHIVE",
            "rank": 1,
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    let det = &view.detectors["det-001"];
    assert!(det.filters.contains_key("high-sev"));
    assert_eq!(det.filters["high-sev"].action, "ARCHIVE");
}

// ---------------------------------------------------------------------------
// Identity Store user injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_identitystore_user() {
    use winterbaume_identitystore::IdentityStoreService;
    use winterbaume_terraform::converters::identitystore::AwsIdentitystoreUserConverter;

    let svc = Arc::new(IdentityStoreService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsIdentitystoreUserConverter::new(Arc::clone(&svc)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_identitystore_user",
        "jdoe",
        json!({
            "identity_store_id": "d-123",
            "user_id": "uid-001",
            "user_name": "jdoe",
            "display_name": "John Doe",
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    let user = &view.users["d-123/uid-001"];
    assert_eq!(user.user_name, Some("jdoe".to_string()));
    assert_eq!(user.display_name, Some("John Doe".to_string()));
}

// ---------------------------------------------------------------------------
// Inspector2 enabler injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_inspector2_enabler() {
    use winterbaume_inspector2::Inspector2Service;
    use winterbaume_terraform::converters::inspector2::AwsInspector2EnablerConverter;

    let svc = Arc::new(Inspector2Service::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsInspector2EnablerConverter::new(Arc::clone(&svc)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_inspector2_enabler",
        "main",
        json!({
            "account_ids": ["123456789012"],
            "resource_types": ["EC2", "ECR"],
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    let all_types: std::collections::HashSet<&String> = view
        .enabled_resource_types
        .values()
        .flat_map(|set| set.iter())
        .collect();
    assert!(all_types.iter().any(|s| s.as_str() == "EC2"));
    assert!(all_types.iter().any(|s| s.as_str() == "ECR"));
}

// ---------------------------------------------------------------------------
// IoT injection tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_iot_certificate() {
    use winterbaume_iot::IotService;
    use winterbaume_terraform::converters::iot::AwsIotCertificateConverter;

    let svc = Arc::new(IotService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsIotCertificateConverter::new(Arc::clone(&svc)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_iot_certificate",
        "cert",
        json!({
            "id": "cert-001",
            "active": true,
            "certificate_pem": "-----BEGIN CERTIFICATE-----\nMIIB...\n-----END CERTIFICATE-----",
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    let cert = &view.certificates["cert-001"];
    assert_eq!(cert.status, "ACTIVE");
    assert!(cert.certificate_pem.contains("BEGIN CERTIFICATE"));
}

#[tokio::test]
async fn test_inject_iot_policy() {
    use winterbaume_iot::IotService;
    use winterbaume_terraform::converters::iot::AwsIotPolicyConverter;

    let svc = Arc::new(IotService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsIotPolicyConverter::new(Arc::clone(&svc)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_iot_policy",
        "pol",
        json!({
            "name": "my-policy",
            "policy": "{}",
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    let policy = &view.policies["my-policy"];
    assert_eq!(policy.policy_name, "my-policy");
    assert_eq!(policy.policy_document, "{}");
}

#[tokio::test]
async fn test_inject_iot_thing_type() {
    use winterbaume_iot::IotService;
    use winterbaume_terraform::converters::iot::AwsIotThingTypeConverter;

    let svc = Arc::new(IotService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsIotThingTypeConverter::new(Arc::clone(&svc)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_iot_thing_type",
        "tt",
        json!({
            "name": "my-type",
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    let tt = &view.thing_types["my-type"];
    assert_eq!(tt.thing_type_name, "my-type");
}

// ---------------------------------------------------------------------------
// Known limitation: extract only queries the default region
// ---------------------------------------------------------------------------

/// Known limitation: `TerraformResourceConverter::extract` uses
/// `ctx.default_region` to snapshot state, so resources injected into a
/// non-default region are invisible to extraction.  Injection correctly
/// honours per-resource `region` attributes, but extraction does not iterate
/// over all regions.
#[tokio::test]
async fn test_extract_misses_non_default_region_known_limitation() {
    let ebs = Arc::new(EbsService::new());
    let ctx = default_ctx(); // default_region = "us-east-1"

    // Inject a volume into eu-west-1 (non-default region).
    let mut injector = TerraformInjector::new();
    injector.register(AwsEbsVolumeConverter::new(Arc::clone(&ebs)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_ebs_volume",
        "non_default",
        json!({
            "availability_zone": "eu-west-1a",
            "size": 20,
            "region": "eu-west-1"
        }),
    )]);
    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "injection failed: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    // Confirm the volume exists in eu-west-1.
    let eu_view = ebs.snapshot(&ctx.default_account_id, "eu-west-1").await;
    assert_eq!(
        eu_view.volumes.len(),
        1,
        "volume should exist in eu-west-1 after injection"
    );

    // Extract using the default context (us-east-1).
    // Known limitation: this returns nothing because extract only queries
    // the default region.
    let converter = AwsEbsVolumeConverter::new(Arc::clone(&ebs));
    let extracted = converter.extract(&ctx).await.unwrap();
    assert!(
        extracted.is_empty(),
        "known limitation: extract should not find resources in non-default regions, \
         but found {} resource(s)",
        extracted.len()
    );
}

// ---------------------------------------------------------------------------
// extract_all / register_with_scopes / collision detection tests
// ---------------------------------------------------------------------------

use std::sync::Mutex;

use winterbaume_terraform::ConversionError;
use winterbaume_terraform::converter::ExtractedResource;

/// A minimal mock converter whose `extract` returns resources parameterised by
/// the context's account_id and region.
struct MockConverter {
    type_name: String,
    /// Each entry becomes one ExtractedResource; the closure receives the ctx.
    resources: Arc<Mutex<Vec<String>>>,
}

impl MockConverter {
    fn new(type_name: &str, names: Vec<String>) -> Self {
        Self {
            type_name: type_name.to_string(),
            resources: Arc::new(Mutex::new(names)),
        }
    }
}

impl TerraformResourceConverter for MockConverter {
    fn resource_type(&self) -> &str {
        &self.type_name
    }

    fn inject<'a>(
        &'a self,
        _instance: &'a ResourceInstance,
        _ctx: &'a ConversionContext,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<winterbaume_terraform::ConversionResult, ConversionError>>
                + Send
                + 'a,
        >,
    > {
        Box::pin(async { unreachable!("inject not used in these tests") })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        let names = self.resources.lock().unwrap().clone();
        let account_id = ctx.default_account_id.clone();
        let region = ctx.default_region.clone();
        Box::pin(async move {
            Ok(names
                .into_iter()
                .map(|name| ExtractedResource {
                    name,
                    account_id: account_id.clone(),
                    region: region.clone(),
                    attributes: json!({}),
                })
                .collect())
        })
    }
}

use std::future::Future;
use std::pin::Pin;

#[tokio::test]
async fn extract_all_single_scope_uses_default_context() {
    let mut injector = TerraformInjector::new();
    injector.register(MockConverter::new(
        "aws_mock_thing",
        vec!["alpha".into(), "beta".into()],
    ));

    let ctx = default_ctx();
    let report = injector.extract_all(&ctx).await;

    assert!(report.errors.is_empty());
    let resources = report.resources.get("aws_mock_thing").unwrap();
    assert_eq!(resources.len(), 2);
    // All resources should use the default context values
    for r in resources {
        assert_eq!(r.account_id, "123456789012");
        assert_eq!(r.region, "us-east-1");
    }
}

#[tokio::test]
async fn extract_all_with_scopes_calls_extract_per_scope() {
    let mut injector = TerraformInjector::new();
    injector.register_with_scopes(
        MockConverter::new("aws_mock_thing", vec!["item".into()]),
        || {
            vec![
                ("111111111111".into(), "us-east-1".into()),
                ("222222222222".into(), "eu-west-1".into()),
            ]
        },
    );

    let ctx = default_ctx();
    let report = injector.extract_all(&ctx).await;

    assert!(report.errors.is_empty());
    let resources = report.resources.get("aws_mock_thing").unwrap();
    // One resource per scope = 2 total
    assert_eq!(resources.len(), 2);

    let accounts: Vec<&str> = resources.iter().map(|r| r.account_id.as_str()).collect();
    assert!(accounts.contains(&"111111111111"));
    assert!(accounts.contains(&"222222222222"));
}

#[tokio::test]
async fn extract_all_empty_scope_provider_falls_back_to_default() {
    let mut injector = TerraformInjector::new();
    injector.register_with_scopes(
        MockConverter::new("aws_mock_thing", vec!["item".into()]),
        Vec::new, // empty scope provider
    );

    let ctx = default_ctx();
    let report = injector.extract_all(&ctx).await;

    assert!(report.errors.is_empty());
    let resources = report.resources.get("aws_mock_thing").unwrap();
    assert_eq!(resources.len(), 1);
    assert_eq!(resources[0].account_id, "123456789012");
    assert_eq!(resources[0].region, "us-east-1");
}

#[tokio::test]
async fn extract_all_collision_detection_qualifies_duplicate_names() {
    let mut injector = TerraformInjector::new();
    // "processor" appears from two different scopes → collision
    injector.register_with_scopes(
        MockConverter::new("aws_mock_thing", vec!["processor".into()]),
        || {
            vec![
                ("111111111111".into(), "us-east-1".into()),
                ("222222222222".into(), "eu-west-1".into()),
            ]
        },
    );

    let ctx = default_ctx();
    let report = injector.extract_all(&ctx).await;

    assert!(report.errors.is_empty());
    let resources = report.resources.get("aws_mock_thing").unwrap();
    assert_eq!(resources.len(), 2);

    // Both should have been renamed with scope suffixes
    let names: Vec<&str> = resources.iter().map(|r| r.name.as_str()).collect();
    assert!(
        names.contains(&"processor_111111111111_us_east_1"),
        "expected qualified name for account 111, got: {names:?}"
    );
    assert!(
        names.contains(&"processor_222222222222_eu_west_1"),
        "expected qualified name for account 222, got: {names:?}"
    );
}

#[tokio::test]
async fn extract_all_no_collision_when_names_differ() {
    let mut injector = TerraformInjector::new();
    // Two scopes but the mock always returns the same names — however each
    // scope's extract call returns different names because we use two
    // separate converters with unique names.
    injector.register(MockConverter::new("aws_mock_a", vec!["alpha".into()]));
    injector.register(MockConverter::new("aws_mock_b", vec!["alpha".into()]));

    let ctx = default_ctx();
    let report = injector.extract_all(&ctx).await;

    assert!(report.errors.is_empty());
    // Same name but different resource types → no collision
    let a = &report.resources["aws_mock_a"];
    let b = &report.resources["aws_mock_b"];
    assert_eq!(a[0].name, "alpha");
    assert_eq!(b[0].name, "alpha");
}

// ---------------------------------------------------------------------------
// EC2 capacity reservation injection test
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_ec2_capacity_reservation() {
    use winterbaume_terraform::converters::ec2::AwsEc2CapacityReservationConverter;

    let svc = Arc::new(Ec2Service::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsEc2CapacityReservationConverter::new(Arc::clone(&svc)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_ec2_capacity_reservation",
        "test_cr",
        json!({
            "id": "cr-1234567890abcdef0",
            "instance_type": "t3.large",
            "instance_platform": "Linux/UNIX",
            "availability_zone": "us-east-1a",
            "instance_count": 4,
            "tenancy": "default",
            "ebs_optimized": true,
            "ephemeral_storage": false,
            "end_date_type": "unlimited",
            "instance_match_criteria": "open",
            "tags": {"Env": "test"},
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert_eq!(view.capacity_reservations.len(), 1);
    let cr = view
        .capacity_reservations
        .values()
        .next()
        .expect("capacity reservation should exist");
    assert_eq!(cr.instance_type, "t3.large");
    assert_eq!(cr.instance_platform, "Linux/UNIX");
    assert_eq!(cr.availability_zone, "us-east-1a");
    assert_eq!(cr.total_instance_count, 4);
    assert_eq!(cr.available_instance_count, 4);
    assert!(cr.ebs_optimized);
    assert_eq!(cr.tags.get("Env").map(|s| s.as_str()), Some("test"));

    // Round-trip through extract.
    let converter = AwsEc2CapacityReservationConverter::new(Arc::clone(&svc));
    let extracted = converter
        .extract(&ctx)
        .await
        .expect("extract should succeed");
    assert_eq!(extracted.len(), 1);
    assert_eq!(extracted[0].attributes["instance_type"], "t3.large");
    assert_eq!(extracted[0].attributes["instance_count"], 4);
}

// ---------------------------------------------------------------------------
// Network interface permission injection test
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_network_interface_permission() {
    use winterbaume_terraform::converters::ec2::AwsNetworkInterfacePermissionConverter;

    let svc = Arc::new(Ec2Service::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsNetworkInterfacePermissionConverter::new(Arc::clone(
        &svc,
    )));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_network_interface_permission",
        "test_perm",
        json!({
            "id": "eni-perm-0abcdef1234567890",
            "network_interface_id": "eni-0123456789abcdef0",
            "aws_account_id": "999988887777",
            "permission": "INSTANCE-ATTACH",
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert_eq!(view.network_interface_permissions.len(), 1);
    let perm = view
        .network_interface_permissions
        .values()
        .next()
        .expect("permission should exist");
    assert_eq!(perm.network_interface_id, "eni-0123456789abcdef0");
    assert_eq!(perm.aws_account_id.as_deref(), Some("999988887777"));
    assert_eq!(perm.permission, "INSTANCE-ATTACH");

    // Round-trip through extract.
    let converter = AwsNetworkInterfacePermissionConverter::new(Arc::clone(&svc));
    let extracted = converter
        .extract(&ctx)
        .await
        .expect("extract should succeed");
    assert_eq!(extracted.len(), 1);
    assert_eq!(extracted[0].attributes["permission"], "INSTANCE-ATTACH");
    assert_eq!(
        extracted[0].attributes["network_interface_id"],
        "eni-0123456789abcdef0"
    );
}

// ---------------------------------------------------------------------------
// EC2 instance connect endpoint injection test
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_ec2_instance_connect_endpoint_round_trip() {
    use winterbaume_terraform::converters::ec2::AwsEc2InstanceConnectEndpointConverter;

    let svc = Arc::new(Ec2Service::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsEc2InstanceConnectEndpointConverter::new(Arc::clone(
        &svc,
    )));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_ec2_instance_connect_endpoint",
        "test_ice",
        json!({
            "id": "eice-0abcdef1234567890",
            "subnet_id": "subnet-0123456789abcdef0",
            "preserve_client_ip": false,
            "security_group_ids": ["sg-0123456789abcdef0", "sg-0abcdef1234567890"],
            "tags": {"Env": "test"},
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert_eq!(view.instance_connect_endpoints.len(), 1);
    let ep = view
        .instance_connect_endpoints
        .values()
        .next()
        .expect("endpoint should exist");
    assert_eq!(ep.subnet_id, "subnet-0123456789abcdef0");
    assert!(!ep.preserve_client_ip);
    assert_eq!(ep.security_group_ids.len(), 2);
    assert_eq!(ep.tags.get("Env").map(|s| s.as_str()), Some("test"));

    // Round-trip through extract.
    let converter = AwsEc2InstanceConnectEndpointConverter::new(Arc::clone(&svc));
    let extracted = converter
        .extract(&ctx)
        .await
        .expect("extract should succeed");
    assert_eq!(extracted.len(), 1);
    assert_eq!(
        extracted[0].attributes["subnet_id"],
        "subnet-0123456789abcdef0"
    );
    assert_eq!(extracted[0].attributes["preserve_client_ip"], false);
}

// ---------------------------------------------------------------------------
// AppFlow injection test
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_appflow_flow_describe_via_sdk() {
    use winterbaume_appflow::AppFlowService;
    use winterbaume_terraform::converters::appflow::AwsAppFlowFlowConverter;

    let appflow = Arc::new(AppFlowService::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsAppFlowFlowConverter::new(Arc::clone(&appflow)));

    // Realistic tfstate with all four nested-block fields populated, mirroring
    // the AWS provider schema (snake_case keys, singleton blocks as length-1
    // arrays, block lists as JSON arrays).
    let tfstate = make_tfstate(vec![make_resource(
        "aws_appflow_flow",
        "test_flow",
        json!({
            "name": "tf-injected-flow",
            "description": "Flow injected from terraform state",
            "trigger_config": [{
                "trigger_type": "Scheduled",
                "trigger_properties": [{
                    "scheduled": [{
                        "schedule_expression": "rate(1hours)",
                        "data_pull_mode": "Incremental",
                    }],
                }],
            }],
            "source_flow_config": [{
                "connector_type": "S3",
                "source_connector_properties": [{
                    "s3": [{
                        "bucket_name": "tf-source-bucket",
                        "bucket_prefix": "in/",
                    }],
                }],
            }],
            "destination_flow_config": [{
                "connector_type": "S3",
                "destination_connector_properties": [{
                    "s3": [{
                        "bucket_name": "tf-dest-bucket",
                        "s3_output_format_config": [{
                            "file_type": "JSON",
                            "prefix_config": [{
                                "prefix_type": "PATH",
                            }],
                        }],
                    }],
                }],
            }],
            "task": [{
                "task_type": "Map",
                "source_fields": ["id"],
                "destination_field": "id",
                "task_properties": {"DESTINATION_DATA_TYPE": "string"},
                "connector_operator": [{"s3": "NO_OP"}],
            }],
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    // Spin up a fresh AppflowService backed by the injected state and call
    // DescribeFlow through aws-sdk-appflow.
    let snapshot = appflow.snapshot(&ctx.default_account_id, "us-east-1").await;
    let appflow_for_mock = AppFlowService::new();
    appflow_for_mock
        .restore(&ctx.default_account_id, "us-east-1", snapshot)
        .await
        .unwrap();

    let mock = MockAws::builder().with_service(appflow_for_mock).build();
    let config = mock.sdk_config("us-east-1").await;
    let client = aws_sdk_appflow::Client::new(&config);

    let resp = client
        .describe_flow()
        .flow_name("tf-injected-flow")
        .send()
        .await
        .expect("describe_flow should succeed");

    // trigger_config is reshaped, deserialized, and visible to the SDK.
    let trigger = resp
        .trigger_config()
        .expect("trigger_config should be present");
    assert_eq!(trigger.trigger_type().as_str(), "Scheduled");
    let scheduled = trigger
        .trigger_properties()
        .and_then(|p| p.scheduled())
        .expect("scheduled trigger properties should be present");
    assert_eq!(scheduled.schedule_expression(), "rate(1hours)");
    assert_eq!(
        scheduled.data_pull_mode().map(|m| m.as_str()),
        Some("Incremental")
    );

    // source_flow_config -> S3 source connector properties.
    let source = resp.source_flow_config().expect("source_flow_config");
    assert_eq!(source.connector_type().as_str(), "S3");
    let s3_src = source
        .source_connector_properties()
        .and_then(|p| p.s3())
        .expect("S3 source properties");
    assert_eq!(s3_src.bucket_name(), "tf-source-bucket");
    assert_eq!(s3_src.bucket_prefix(), Some("in/"));

    // destination_flow_config_list -> S3 destination connector properties.
    let dests = resp.destination_flow_config_list();
    assert_eq!(dests.len(), 1);
    assert_eq!(dests[0].connector_type().as_str(), "S3");
    let s3_dst = dests[0]
        .destination_connector_properties()
        .and_then(|p| p.s3())
        .expect("S3 destination properties");
    assert_eq!(s3_dst.bucket_name(), "tf-dest-bucket");
    let fmt_cfg = s3_dst
        .s3_output_format_config()
        .expect("s3_output_format_config");
    assert_eq!(fmt_cfg.file_type().map(|t| t.as_str()), Some("JSON"));
    assert_eq!(
        fmt_cfg
            .prefix_config()
            .and_then(|p| p.prefix_type())
            .map(|t| t.as_str()),
        Some("PATH")
    );

    // tasks -> Map task with connector operator and properties.
    let tasks = resp.tasks();
    assert_eq!(tasks.len(), 1);
    assert_eq!(tasks[0].task_type().as_str(), "Map");
    assert_eq!(tasks[0].destination_field(), Some("id"));
    assert_eq!(tasks[0].source_fields(), &["id".to_string()]);
    let op = tasks[0].connector_operator().expect("connectorOperator");
    assert_eq!(op.s3().map(|o| o.as_str()), Some("NO_OP"));
    let props = tasks[0].task_properties().expect("task_properties");
    assert_eq!(
        props
            .get(&aws_sdk_appflow::types::OperatorPropertiesKeys::from(
                "DESTINATION_DATA_TYPE"
            ))
            .map(String::as_str),
        Some("string")
    );
}

// ---------------------------------------------------------------------------
// IPAM round-trip tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_vpc_ipam_round_trip() {
    use winterbaume_terraform::converters::ec2::AwsVpcIpamConverter;

    let svc = Arc::new(Ec2Service::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsVpcIpamConverter::new(Arc::clone(&svc)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_vpc_ipam",
        "test_ipam",
        json!({
            "id": "ipam-0123456789abcdef0",
            "description": "test ipam",
            "tier": "advanced",
            "operating_regions": [{"region_name": "us-east-1"}],
            "tags": {"Env": "test"},
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert_eq!(view.ipams.len(), 1);
    let ipam = view.ipams.values().next().unwrap();
    assert_eq!(ipam.tier, "advanced");
    assert_eq!(ipam.description.as_deref(), Some("test ipam"));
    assert_eq!(ipam.operating_regions.len(), 1);

    let converter = AwsVpcIpamConverter::new(Arc::clone(&svc));
    let extracted = converter
        .extract(&ctx)
        .await
        .expect("extract should succeed");
    assert_eq!(extracted.len(), 1);
    assert_eq!(extracted[0].attributes["tier"], "advanced");
    assert_eq!(extracted[0].attributes["description"], "test ipam");
}

#[tokio::test]
async fn test_inject_vpc_ipam_scope_round_trip() {
    use winterbaume_terraform::converters::ec2::{AwsVpcIpamConverter, AwsVpcIpamScopeConverter};

    let svc = Arc::new(Ec2Service::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsVpcIpamConverter::new(Arc::clone(&svc)));
    injector.register(AwsVpcIpamScopeConverter::new(Arc::clone(&svc)));

    let tfstate = make_tfstate(vec![
        make_resource(
            "aws_vpc_ipam",
            "test_ipam",
            json!({
                "id": "ipam-0123456789abcdef0",
                "description": "test ipam",
                "operating_regions": [{"region_name": "us-east-1"}],
            }),
        ),
        make_resource(
            "aws_vpc_ipam_scope",
            "test_scope",
            json!({
                "id": "ipam-scope-0123456789abcdef0",
                "ipam_id": "ipam-0123456789abcdef0",
                "description": "extra scope",
                "ipam_scope_type": "private",
            }),
        ),
    ]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 2);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert_eq!(view.ipam_scopes.len(), 1);

    let converter = AwsVpcIpamScopeConverter::new(Arc::clone(&svc));
    let extracted = converter
        .extract(&ctx)
        .await
        .expect("extract should succeed");
    assert_eq!(extracted.len(), 1);
    assert_eq!(extracted[0].attributes["description"], "extra scope");
}

#[tokio::test]
async fn test_inject_vpc_ipam_pool_round_trip() {
    use winterbaume_terraform::converters::ec2::{
        AwsVpcIpamConverter, AwsVpcIpamPoolConverter, AwsVpcIpamScopeConverter,
    };

    let svc = Arc::new(Ec2Service::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsVpcIpamConverter::new(Arc::clone(&svc)));
    injector.register(AwsVpcIpamScopeConverter::new(Arc::clone(&svc)));
    injector.register(AwsVpcIpamPoolConverter::new(Arc::clone(&svc)));

    let tfstate = make_tfstate(vec![
        make_resource(
            "aws_vpc_ipam",
            "test_ipam",
            json!({
                "id": "ipam-0123456789abcdef0",
                "operating_regions": [{"region_name": "us-east-1"}],
            }),
        ),
        make_resource(
            "aws_vpc_ipam_scope",
            "test_scope",
            json!({
                "id": "ipam-scope-0123456789abcdef0",
                "ipam_id": "ipam-0123456789abcdef0",
            }),
        ),
        make_resource(
            "aws_vpc_ipam_pool",
            "test_pool",
            json!({
                "id": "ipam-pool-0123456789abcdef0",
                "ipam_scope_id": "ipam-scope-0123456789abcdef0",
                "address_family": "ipv4",
                "locale": "us-east-1",
                "auto_import": true,
                "description": "test pool",
            }),
        ),
    ]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 3);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert_eq!(view.ipam_pools.len(), 1);
    let pool = view.ipam_pools.values().next().unwrap();
    assert_eq!(pool.address_family, "ipv4");
    assert!(pool.auto_import);

    let converter = AwsVpcIpamPoolConverter::new(Arc::clone(&svc));
    let extracted = converter
        .extract(&ctx)
        .await
        .expect("extract should succeed");
    assert_eq!(extracted.len(), 1);
    assert_eq!(extracted[0].attributes["address_family"], "ipv4");
}

#[tokio::test]
async fn test_inject_vpc_ipam_pool_cidr_round_trip() {
    use winterbaume_terraform::converters::ec2::AwsVpcIpamPoolCidrConverter;

    let svc = Arc::new(Ec2Service::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsVpcIpamPoolCidrConverter::new(Arc::clone(&svc)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_vpc_ipam_pool_cidr",
        "test_cidr",
        json!({
            "id": "ipam-pool-cidr-abc",
            "ipam_pool_id": "ipam-pool-0123456789abcdef0",
            "cidr": "10.0.0.0/16",
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert_eq!(view.ipam_pool_cidrs.len(), 1);
    assert_eq!(view.ipam_pool_cidrs[0].cidr, "10.0.0.0/16");

    let converter = AwsVpcIpamPoolCidrConverter::new(Arc::clone(&svc));
    let extracted = converter
        .extract(&ctx)
        .await
        .expect("extract should succeed");
    assert_eq!(extracted.len(), 1);
    assert_eq!(extracted[0].attributes["cidr"], "10.0.0.0/16");
}

#[tokio::test]
async fn test_inject_vpc_ipam_resource_discovery_round_trip() {
    use winterbaume_terraform::converters::ec2::AwsVpcIpamResourceDiscoveryConverter;

    let svc = Arc::new(Ec2Service::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsVpcIpamResourceDiscoveryConverter::new(Arc::clone(&svc)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_vpc_ipam_resource_discovery",
        "test_disco",
        json!({
            "id": "ipam-res-disco-0123456789abcdef0",
            "description": "research discovery",
            "operating_regions": [{"region_name": "us-east-1"}],
            "tags": {"Env": "test"},
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert_eq!(view.ipam_resource_discoveries.len(), 1);
    let disco = view.ipam_resource_discoveries.values().next().unwrap();
    assert_eq!(disco.description.as_deref(), Some("research discovery"));

    let converter = AwsVpcIpamResourceDiscoveryConverter::new(Arc::clone(&svc));
    let extracted = converter
        .extract(&ctx)
        .await
        .expect("extract should succeed");
    assert_eq!(extracted.len(), 1);
    assert_eq!(extracted[0].attributes["description"], "research discovery");
}

// ---------------------------------------------------------------------------
// Verified Access round-trip tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_verifiedaccess_instance_round_trip() {
    use winterbaume_terraform::converters::ec2::AwsVerifiedaccessInstanceConverter;

    let svc = Arc::new(Ec2Service::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsVerifiedaccessInstanceConverter::new(Arc::clone(&svc)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_verifiedaccess_instance",
        "test_vai",
        json!({
            "id": "vai-0123456789abcdef0",
            "description": "test VAI",
            "fips_enabled": true,
            "tags": {"Env": "test"},
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert_eq!(view.verified_access_instances.len(), 1);
    let inst = view.verified_access_instances.values().next().unwrap();
    assert!(inst.fips_enabled);

    let converter = AwsVerifiedaccessInstanceConverter::new(Arc::clone(&svc));
    let extracted = converter
        .extract(&ctx)
        .await
        .expect("extract should succeed");
    assert_eq!(extracted.len(), 1);
    assert_eq!(extracted[0].attributes["fips_enabled"], true);
    assert_eq!(extracted[0].attributes["description"], "test VAI");
}

#[tokio::test]
async fn test_inject_verifiedaccess_trust_provider_round_trip() {
    use winterbaume_terraform::converters::ec2::AwsVerifiedaccessTrustProviderConverter;

    let svc = Arc::new(Ec2Service::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsVerifiedaccessTrustProviderConverter::new(Arc::clone(
        &svc,
    )));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_verifiedaccess_trust_provider",
        "test_vatp",
        json!({
            "id": "vatp-0123456789abcdef0",
            "trust_provider_type": "user",
            "user_trust_provider_type": "oidc",
            "policy_reference_name": "test_user",
            "description": "test TP",
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert_eq!(view.verified_access_trust_providers.len(), 1);
    let tp = view
        .verified_access_trust_providers
        .values()
        .next()
        .unwrap();
    assert_eq!(tp.trust_provider_type, "user");
    assert_eq!(tp.policy_reference_name, "test_user");

    let converter = AwsVerifiedaccessTrustProviderConverter::new(Arc::clone(&svc));
    let extracted = converter
        .extract(&ctx)
        .await
        .expect("extract should succeed");
    assert_eq!(extracted.len(), 1);
    assert_eq!(extracted[0].attributes["trust_provider_type"], "user");
}

#[tokio::test]
async fn test_inject_verifiedaccess_group_round_trip() {
    use winterbaume_terraform::converters::ec2::{
        AwsVerifiedaccessGroupConverter, AwsVerifiedaccessInstanceConverter,
    };

    let svc = Arc::new(Ec2Service::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsVerifiedaccessInstanceConverter::new(Arc::clone(&svc)));
    injector.register(AwsVerifiedaccessGroupConverter::new(Arc::clone(&svc)));

    let tfstate = make_tfstate(vec![
        make_resource(
            "aws_verifiedaccess_instance",
            "test_vai",
            json!({
                "id": "vai-0123456789abcdef0",
            }),
        ),
        make_resource(
            "aws_verifiedaccess_group",
            "test_vag",
            json!({
                "id": "vagr-0123456789abcdef0",
                "verifiedaccess_instance_id": "vai-0123456789abcdef0",
                "description": "test group",
                "policy_document": "permit(principal,action,resource);",
            }),
        ),
    ]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 2);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert_eq!(view.verified_access_groups.len(), 1);
    let g = view.verified_access_groups.values().next().unwrap();
    assert_eq!(g.verified_access_instance_id, "vai-0123456789abcdef0");

    let converter = AwsVerifiedaccessGroupConverter::new(Arc::clone(&svc));
    let extracted = converter
        .extract(&ctx)
        .await
        .expect("extract should succeed");
    assert_eq!(extracted.len(), 1);
    assert_eq!(extracted[0].attributes["description"], "test group");
}

#[tokio::test]
async fn test_inject_verifiedaccess_endpoint_round_trip() {
    use winterbaume_terraform::converters::ec2::{
        AwsVerifiedaccessEndpointConverter, AwsVerifiedaccessGroupConverter,
        AwsVerifiedaccessInstanceConverter,
    };

    let svc = Arc::new(Ec2Service::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsVerifiedaccessInstanceConverter::new(Arc::clone(&svc)));
    injector.register(AwsVerifiedaccessGroupConverter::new(Arc::clone(&svc)));
    injector.register(AwsVerifiedaccessEndpointConverter::new(Arc::clone(&svc)));

    let tfstate = make_tfstate(vec![
        make_resource(
            "aws_verifiedaccess_instance",
            "test_vai",
            json!({
                "id": "vai-0123456789abcdef0",
            }),
        ),
        make_resource(
            "aws_verifiedaccess_group",
            "test_vag",
            json!({
                "id": "vagr-0123456789abcdef0",
                "verifiedaccess_instance_id": "vai-0123456789abcdef0",
            }),
        ),
        make_resource(
            "aws_verifiedaccess_endpoint",
            "test_vae",
            json!({
                "id": "vae-0123456789abcdef0",
                "verifiedaccess_group_id": "vagr-0123456789abcdef0",
                "endpoint_type": "load-balancer",
                "attachment_type": "vpc",
                "application_domain": "app.example.com",
                "domain_certificate_arn": "arn:aws:acm:us-east-1:123:certificate/abc",
                "endpoint_domain_prefix": "myapp",
                "security_group_ids": ["sg-0123456789abcdef0"],
                "description": "test endpoint",
            }),
        ),
    ]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 3);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert_eq!(view.verified_access_endpoints.len(), 1);
    let ep = view.verified_access_endpoints.values().next().unwrap();
    assert_eq!(ep.endpoint_type, "load-balancer");
    assert_eq!(ep.attachment_type, "vpc");
    assert_eq!(ep.security_group_ids.len(), 1);
    assert_eq!(
        ep.verified_access_instance_id, "vai-0123456789abcdef0",
        "endpoint should derive instance id from its group"
    );

    let converter = AwsVerifiedaccessEndpointConverter::new(Arc::clone(&svc));
    let extracted = converter
        .extract(&ctx)
        .await
        .expect("extract should succeed");
    assert_eq!(extracted.len(), 1);
    assert_eq!(extracted[0].attributes["endpoint_type"], "load-balancer");
}

// ---------------------------------------------------------------------------
// Traffic Mirror round-trip tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_traffic_mirror_target_round_trip() {
    use winterbaume_terraform::converters::ec2::AwsEc2TrafficMirrorTargetConverter;

    let svc = Arc::new(Ec2Service::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsEc2TrafficMirrorTargetConverter::new(Arc::clone(&svc)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_ec2_traffic_mirror_target",
        "test_tmt",
        json!({
            "id": "tmt-0123456789abcdef0",
            "network_interface_id": "eni-0123456789abcdef0",
            "description": "mirror to ENI",
            "tags": {"Env": "test"},
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert_eq!(view.traffic_mirror_targets.len(), 1);
    let t = view.traffic_mirror_targets.values().next().unwrap();
    assert_eq!(
        t.network_interface_id.as_deref(),
        Some("eni-0123456789abcdef0")
    );
    assert_eq!(t.r#type, "network-interface");

    let converter = AwsEc2TrafficMirrorTargetConverter::new(Arc::clone(&svc));
    let extracted = converter
        .extract(&ctx)
        .await
        .expect("extract should succeed");
    assert_eq!(extracted.len(), 1);
    assert_eq!(extracted[0].attributes["type"], "network-interface");
}

#[tokio::test]
async fn test_inject_traffic_mirror_filter_and_rule_round_trip() {
    use winterbaume_terraform::converters::ec2::{
        AwsEc2TrafficMirrorFilterConverter, AwsEc2TrafficMirrorFilterRuleConverter,
    };

    let svc = Arc::new(Ec2Service::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsEc2TrafficMirrorFilterConverter::new(Arc::clone(&svc)));
    injector.register(AwsEc2TrafficMirrorFilterRuleConverter::new(Arc::clone(
        &svc,
    )));

    let tfstate = make_tfstate(vec![
        make_resource(
            "aws_ec2_traffic_mirror_filter",
            "test_tmf",
            json!({
                "id": "tmf-0123456789abcdef0",
                "description": "mirror filter",
                "network_services": ["amazon-dns"],
            }),
        ),
        make_resource(
            "aws_ec2_traffic_mirror_filter_rule",
            "test_tmfr",
            json!({
                "id": "tmfr-0123456789abcdef0",
                "traffic_mirror_filter_id": "tmf-0123456789abcdef0",
                "traffic_direction": "ingress",
                "rule_action": "accept",
                "rule_number": 100,
                "destination_cidr_block": "10.0.0.0/16",
                "source_cidr_block": "0.0.0.0/0",
                "protocol": 6,
                "destination_port_range": [{"from_port": 80, "to_port": 80}],
            }),
        ),
    ]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 2);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert_eq!(view.traffic_mirror_filters.len(), 1);
    let f = view.traffic_mirror_filters.values().next().unwrap();
    assert_eq!(f.ingress_filter_rules.len(), 1);
    assert_eq!(f.ingress_filter_rules[0].rule_number, 100);
    assert_eq!(
        f.ingress_filter_rules[0].destination_cidr_block,
        "10.0.0.0/16"
    );
}

#[tokio::test]
async fn test_inject_traffic_mirror_session_round_trip() {
    use winterbaume_terraform::converters::ec2::AwsEc2TrafficMirrorSessionConverter;

    let svc = Arc::new(Ec2Service::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsEc2TrafficMirrorSessionConverter::new(Arc::clone(&svc)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_ec2_traffic_mirror_session",
        "test_tms",
        json!({
            "id": "tms-0123456789abcdef0",
            "network_interface_id": "eni-aaaaaaaaaaaaaaaaa",
            "traffic_mirror_target_id": "tmt-0123456789abcdef0",
            "traffic_mirror_filter_id": "tmf-0123456789abcdef0",
            "session_number": 1,
            "packet_length": 8500,
            "virtual_network_id": 42,
            "description": "mirror session",
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert_eq!(view.traffic_mirror_sessions.len(), 1);
    let s = view.traffic_mirror_sessions.values().next().unwrap();
    assert_eq!(s.session_number, 1);
    assert_eq!(s.packet_length, Some(8500));
    assert_eq!(s.virtual_network_id, Some(42));

    let converter = AwsEc2TrafficMirrorSessionConverter::new(Arc::clone(&svc));
    let extracted = converter
        .extract(&ctx)
        .await
        .expect("extract should succeed");
    assert_eq!(extracted.len(), 1);
    assert_eq!(extracted[0].attributes["session_number"], 1);
}

// ---------------------------------------------------------------------------
// Transit Gateway extension round-trip tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_tgw_multicast_domain_round_trip() {
    use winterbaume_terraform::converters::ec2::AwsEc2TransitGatewayMulticastDomainConverter;

    let svc = Arc::new(Ec2Service::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsEc2TransitGatewayMulticastDomainConverter::new(
        Arc::clone(&svc),
    ));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_ec2_transit_gateway_multicast_domain",
        "test_md",
        json!({
            "id": "tgw-mcast-domain-0123456789abcdef0",
            "transit_gateway_id": "tgw-0123456789abcdef0",
            "igmpv2_support": "enable",
            "static_sources_support": "disable",
            "auto_accept_shared_associations": "enable",
            "tags": {"Env": "test"},
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert_eq!(view.tgw_multicast_domains.len(), 1);
    let d = view.tgw_multicast_domains.values().next().unwrap();
    assert_eq!(d.igmpv2_support, "enable");

    let converter = AwsEc2TransitGatewayMulticastDomainConverter::new(Arc::clone(&svc));
    let extracted = converter
        .extract(&ctx)
        .await
        .expect("extract should succeed");
    assert_eq!(extracted.len(), 1);
    assert_eq!(extracted[0].attributes["igmpv2_support"], "enable");
}

#[tokio::test]
async fn test_inject_tgw_connect_round_trip() {
    use winterbaume_terraform::converters::ec2::AwsEc2TransitGatewayConnectConverter;

    let svc = Arc::new(Ec2Service::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsEc2TransitGatewayConnectConverter::new(Arc::clone(&svc)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_ec2_transit_gateway_connect",
        "test_conn",
        json!({
            "id": "tgw-attach-0123456789abcdef0",
            "transport_attachment_id": "tgw-attach-1111111111aaaaaaa",
            "transit_gateway_id": "tgw-0123456789abcdef0",
            "protocol": "gre",
            "tags": {"Env": "test"},
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert_eq!(view.tgw_connects.len(), 1);
    let c = view.tgw_connects.values().next().unwrap();
    assert_eq!(c.protocol, "gre");
    assert_eq!(c.transit_gateway_id, "tgw-0123456789abcdef0");

    let converter = AwsEc2TransitGatewayConnectConverter::new(Arc::clone(&svc));
    let extracted = converter
        .extract(&ctx)
        .await
        .expect("extract should succeed");
    assert_eq!(extracted.len(), 1);
    assert_eq!(extracted[0].attributes["protocol"], "gre");
}

#[tokio::test]
async fn test_inject_tgw_policy_table_round_trip() {
    use winterbaume_terraform::converters::ec2::AwsEc2TransitGatewayPolicyTableConverter;

    let svc = Arc::new(Ec2Service::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsEc2TransitGatewayPolicyTableConverter::new(Arc::clone(
        &svc,
    )));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_ec2_transit_gateway_policy_table",
        "test_pt",
        json!({
            "id": "tgw-ptb-0123456789abcdef0",
            "transit_gateway_id": "tgw-0123456789abcdef0",
            "tags": {"Env": "test"},
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert_eq!(view.tgw_policy_tables.len(), 1);
    let t = view.tgw_policy_tables.values().next().unwrap();
    assert_eq!(t.transit_gateway_id, "tgw-0123456789abcdef0");

    let converter = AwsEc2TransitGatewayPolicyTableConverter::new(Arc::clone(&svc));
    let extracted = converter
        .extract(&ctx)
        .await
        .expect("extract should succeed");
    assert_eq!(extracted.len(), 1);
    assert_eq!(
        extracted[0].attributes["transit_gateway_id"],
        "tgw-0123456789abcdef0"
    );
}

// ---------------------------------------------------------------------------
// Network Insights round-trip tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_network_insights_path_round_trip() {
    use winterbaume_terraform::converters::ec2::AwsEc2NetworkInsightsPathConverter;

    let svc = Arc::new(Ec2Service::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsEc2NetworkInsightsPathConverter::new(Arc::clone(&svc)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_ec2_network_insights_path",
        "test_nip",
        json!({
            "id": "nip-0123456789abcdef0",
            "protocol": "tcp",
            "source": "i-0123456789abcdef0",
            "destination": "i-0aaaaaaaaaaaaaaa0",
            "destination_port": 443,
            "tags": {"Env": "test"},
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert_eq!(view.network_insights_paths.len(), 1);
    let p = view.network_insights_paths.values().next().unwrap();
    assert_eq!(p.protocol, "tcp");
    assert_eq!(p.destination_port, Some(443));

    let converter = AwsEc2NetworkInsightsPathConverter::new(Arc::clone(&svc));
    let extracted = converter
        .extract(&ctx)
        .await
        .expect("extract should succeed");
    assert_eq!(extracted.len(), 1);
    assert_eq!(extracted[0].attributes["protocol"], "tcp");
    assert_eq!(extracted[0].attributes["destination_port"], 443);
}

#[tokio::test]
async fn test_inject_network_insights_analysis_round_trip() {
    use winterbaume_terraform::converters::ec2::{
        AwsEc2NetworkInsightsAnalysisConverter, AwsEc2NetworkInsightsPathConverter,
    };

    let svc = Arc::new(Ec2Service::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsEc2NetworkInsightsPathConverter::new(Arc::clone(&svc)));
    injector.register(AwsEc2NetworkInsightsAnalysisConverter::new(Arc::clone(
        &svc,
    )));

    let tfstate = make_tfstate(vec![
        make_resource(
            "aws_ec2_network_insights_path",
            "test_nip",
            json!({
                "id": "nip-0123456789abcdef0",
                "protocol": "tcp",
                "source": "i-0123456789abcdef0",
                "destination": "i-0aaaaaaaaaaaaaaa0",
            }),
        ),
        make_resource(
            "aws_ec2_network_insights_analysis",
            "test_nia",
            json!({
                "id": "nia-0123456789abcdef0",
                "network_insights_path_id": "nip-0123456789abcdef0",
                "filter_in_arns": ["arn:aws:ec2:us-east-1:123:instance/i-0aaaaaaaa"],
                "tags": {"Env": "test"},
            }),
        ),
    ]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 2);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert_eq!(view.network_insights_analyses.len(), 1);
    let a = view.network_insights_analyses.values().next().unwrap();
    assert_eq!(a.network_insights_path_id, "nip-0123456789abcdef0");
    assert_eq!(a.filter_in_arns.len(), 1);

    let converter = AwsEc2NetworkInsightsAnalysisConverter::new(Arc::clone(&svc));
    let extracted = converter
        .extract(&ctx)
        .await
        .expect("extract should succeed");
    assert_eq!(extracted.len(), 1);
    assert_eq!(
        extracted[0].attributes["network_insights_path_id"],
        "nip-0123456789abcdef0"
    );
}

// ---------------------------------------------------------------------------
// Batch C round-trip tests: converters added to verify EC2 handler upgrades
// from Default::default() stubs to real state-backed implementations.
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_inject_ec2_local_gateway_route_round_trip() {
    use winterbaume_terraform::converters::ec2::AwsEc2LocalGatewayRouteConverter;

    let svc = Arc::new(Ec2Service::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsEc2LocalGatewayRouteConverter::new(Arc::clone(&svc)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_ec2_local_gateway_route",
        "test_lgw_route",
        json!({
            "local_gateway_route_table_id": "lgw-rtb-0123456789abcdef0",
            "destination_cidr_block": "10.99.0.0/16",
            "local_gateway_virtual_interface_group_id": "lgw-vif-grp-0aaaaaaaaaaaaaaaa",
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert_eq!(view.local_gateway_routes.len(), 1);
    let route = &view.local_gateway_routes[0];
    assert_eq!(route.destination_cidr_block, "10.99.0.0/16");
    assert_eq!(
        route.local_gateway_route_table_id,
        "lgw-rtb-0123456789abcdef0"
    );
    assert_eq!(
        route.local_gateway_virtual_interface_group_id.as_deref(),
        Some("lgw-vif-grp-0aaaaaaaaaaaaaaaa"),
    );

    let converter = AwsEc2LocalGatewayRouteConverter::new(Arc::clone(&svc));
    let extracted = converter
        .extract(&ctx)
        .await
        .expect("extract should succeed");
    assert_eq!(extracted.len(), 1);
    assert_eq!(
        extracted[0].attributes["destination_cidr_block"],
        "10.99.0.0/16"
    );
    assert_eq!(extracted[0].attributes["state"], "active");
}

#[tokio::test]
async fn test_inject_ec2_local_gateway_route_table_vpc_association_round_trip() {
    use winterbaume_terraform::converters::ec2::AwsEc2LocalGatewayRouteTableVpcAssociationConverter;

    let svc = Arc::new(Ec2Service::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsEc2LocalGatewayRouteTableVpcAssociationConverter::new(
        Arc::clone(&svc),
    ));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_ec2_local_gateway_route_table_vpc_association",
        "test_assoc",
        json!({
            "id": "lgw-vpc-assoc-0123456789abcdef0",
            "local_gateway_route_table_id": "lgw-rtb-0123456789abcdef0",
            "local_gateway_id": "lgw-0aaaaaaaaaaaaaaaa",
            "vpc_id": "vpc-0bbbbbbbbbbbbbbbb",
            "tags": {"Env": "test"},
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert_eq!(view.local_gateway_route_table_vpc_associations.len(), 1);
    let assoc = view
        .local_gateway_route_table_vpc_associations
        .values()
        .next()
        .unwrap();
    assert_eq!(assoc.vpc_id, "vpc-0bbbbbbbbbbbbbbbb");
    assert_eq!(assoc.local_gateway_id, "lgw-0aaaaaaaaaaaaaaaa");

    let converter = AwsEc2LocalGatewayRouteTableVpcAssociationConverter::new(Arc::clone(&svc));
    let extracted = converter
        .extract(&ctx)
        .await
        .expect("extract should succeed");
    assert_eq!(extracted.len(), 1);
    assert_eq!(extracted[0].attributes["vpc_id"], "vpc-0bbbbbbbbbbbbbbbb");
    assert_eq!(extracted[0].attributes["state"], "associated");
}

#[tokio::test]
async fn test_inject_vpc_ipam_pool_cidr_allocation_round_trip() {
    use winterbaume_terraform::converters::ec2::AwsVpcIpamPoolCidrAllocationConverter;

    let svc = Arc::new(Ec2Service::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsVpcIpamPoolCidrAllocationConverter::new(Arc::clone(&svc)));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_vpc_ipam_pool_cidr_allocation",
        "test_alloc",
        json!({
            "id": "ipam-pool-alloc-0123456789abcdef0",
            "ipam_pool_id": "ipam-pool-0aaaaaaaaaaaaaaaa",
            "cidr": "10.0.5.0/24",
            "description": "test allocation",
            "resource_type": "custom",
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert_eq!(view.ipam_pool_allocations.len(), 1);
    let alloc = &view.ipam_pool_allocations[0];
    assert_eq!(alloc.cidr, "10.0.5.0/24");
    assert_eq!(alloc.ipam_pool_id, "ipam-pool-0aaaaaaaaaaaaaaaa");
    assert_eq!(alloc.description.as_deref(), Some("test allocation"));

    let converter = AwsVpcIpamPoolCidrAllocationConverter::new(Arc::clone(&svc));
    let extracted = converter
        .extract(&ctx)
        .await
        .expect("extract should succeed");
    assert_eq!(extracted.len(), 1);
    assert_eq!(extracted[0].attributes["cidr"], "10.0.5.0/24");
    assert_eq!(extracted[0].attributes["description"], "test allocation");
}

#[tokio::test]
async fn test_inject_ec2_capacity_block_reservation_round_trip() {
    use winterbaume_terraform::converters::ec2::AwsEc2CapacityBlockReservationConverter;

    let svc = Arc::new(Ec2Service::new());
    let ctx = default_ctx();

    let mut injector = TerraformInjector::new();
    injector.register(AwsEc2CapacityBlockReservationConverter::new(Arc::clone(
        &svc,
    )));

    let tfstate = make_tfstate(vec![make_resource(
        "aws_ec2_capacity_block_reservation",
        "test_cb",
        json!({
            "id": "cb-0123456789abcdef0",
            "capacity_block_offering_id": "cbo-0aaaaaaaaaaaaaaaa",
            "instance_platform": "Linux/UNIX",
            "instance_type": "p4d.24xlarge",
            "instance_count": 4,
            "availability_zone": "us-east-1a",
            "tags": {"Env": "ml-train"},
        }),
    )]);

    let report = injector.inject_all(&tfstate, &ctx).await;
    assert!(report.is_success(), "errors: {:?}", report.errors);
    assert_eq!(report.injected, 1);

    let view = svc.snapshot(&ctx.default_account_id, "us-east-1").await;
    assert_eq!(view.capacity_blocks.len(), 1);
    let block = view.capacity_blocks.values().next().unwrap();
    assert_eq!(block.instance_type, "p4d.24xlarge");
    assert_eq!(block.instance_count, 4);
    assert_eq!(block.availability_zone, "us-east-1a");
    assert_eq!(block.capacity_block_offering_id, "cbo-0aaaaaaaaaaaaaaaa");

    let converter = AwsEc2CapacityBlockReservationConverter::new(Arc::clone(&svc));
    let extracted = converter
        .extract(&ctx)
        .await
        .expect("extract should succeed");
    assert_eq!(extracted.len(), 1);
    assert_eq!(extracted[0].attributes["instance_type"], "p4d.24xlarge");
    assert_eq!(extracted[0].attributes["instance_count"], 4);
}
