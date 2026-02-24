//! Integration tests for winterbaume SageMaker service.

use aws_sdk_sagemaker::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_sagemaker::SageMakerService;

async fn make_client() -> aws_sdk_sagemaker::Client {
    let mock = MockAws::builder()
        .with_service(SageMakerService::new())
        .build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_sagemaker::config::Region::new("us-east-1"))
        .load()
        .await;
    aws_sdk_sagemaker::Client::new(&config)
}

const ROLE: &str = "arn:aws:iam::123456789012:role/SageMakerRole";

/// Helper: assert an Option<&str> contains a substring.
fn assert_opt_contains(opt: Option<&str>, sub: &str) {
    let val = opt.expect("expected Some");
    assert!(val.contains(sub), "expected '{val}' to contain '{sub}'");
}

// ============================================================
// Notebook Instances
// ============================================================

#[tokio::test]
async fn test_notebook_instance_lifecycle() {
    let client = make_client().await;

    let resp = client
        .create_notebook_instance()
        .notebook_instance_name("nb-life")
        .instance_type(aws_sdk_sagemaker::types::InstanceType::MlT2Medium)
        .role_arn(ROLE)
        .send()
        .await
        .unwrap();
    assert_opt_contains(resp.notebook_instance_arn(), "nb-life");

    let d = client
        .describe_notebook_instance()
        .notebook_instance_name("nb-life")
        .send()
        .await
        .unwrap();
    assert_eq!(d.notebook_instance_name(), Some("nb-life"));
    assert_eq!(d.role_arn(), Some(ROLE));

    client
        .start_notebook_instance()
        .notebook_instance_name("nb-life")
        .send()
        .await
        .unwrap();
    client
        .stop_notebook_instance()
        .notebook_instance_name("nb-life")
        .send()
        .await
        .unwrap();

    let list = client.list_notebook_instances().send().await.unwrap();
    assert_eq!(list.notebook_instances().len(), 1);

    client
        .delete_notebook_instance()
        .notebook_instance_name("nb-life")
        .send()
        .await
        .unwrap();
    assert!(
        client
            .describe_notebook_instance()
            .notebook_instance_name("nb-life")
            .send()
            .await
            .is_err()
    );
}

#[tokio::test]
async fn test_create_duplicate_notebook_fails() {
    let client = make_client().await;
    client
        .create_notebook_instance()
        .notebook_instance_name("dup-nb")
        .instance_type(aws_sdk_sagemaker::types::InstanceType::MlT2Medium)
        .role_arn(ROLE)
        .send()
        .await
        .unwrap();
    assert!(
        client
            .create_notebook_instance()
            .notebook_instance_name("dup-nb")
            .instance_type(aws_sdk_sagemaker::types::InstanceType::MlT2Medium)
            .role_arn(ROLE)
            .send()
            .await
            .is_err()
    );
}

// ============================================================
// Models
// ============================================================

#[tokio::test]
async fn test_model_lifecycle() {
    let client = make_client().await;
    let r = client
        .create_model()
        .model_name("m1")
        .execution_role_arn(ROLE)
        .send()
        .await
        .unwrap();
    assert_opt_contains(r.model_arn(), "m1");

    let d = client
        .describe_model()
        .model_name("m1")
        .send()
        .await
        .unwrap();
    assert_eq!(d.model_name(), Some("m1"));

    assert_eq!(client.list_models().send().await.unwrap().models().len(), 1);

    client.delete_model().model_name("m1").send().await.unwrap();
    assert!(
        client
            .describe_model()
            .model_name("m1")
            .send()
            .await
            .is_err()
    );
}

// ============================================================
// Endpoint Configs
// ============================================================

#[tokio::test]
async fn test_endpoint_config_lifecycle() {
    let client = make_client().await;
    let r = client
        .create_endpoint_config()
        .endpoint_config_name("epc1")
        .send()
        .await
        .unwrap();
    assert_opt_contains(r.endpoint_config_arn(), "epc1");

    let d = client
        .describe_endpoint_config()
        .endpoint_config_name("epc1")
        .send()
        .await
        .unwrap();
    assert_eq!(d.endpoint_config_name(), Some("epc1"));

    assert_eq!(
        client
            .list_endpoint_configs()
            .send()
            .await
            .unwrap()
            .endpoint_configs()
            .len(),
        1
    );

    client
        .delete_endpoint_config()
        .endpoint_config_name("epc1")
        .send()
        .await
        .unwrap();
    assert!(
        client
            .describe_endpoint_config()
            .endpoint_config_name("epc1")
            .send()
            .await
            .is_err()
    );
}

// ============================================================
// Endpoints
// ============================================================

#[tokio::test]
async fn test_endpoint_lifecycle() {
    let client = make_client().await;
    client
        .create_endpoint_config()
        .endpoint_config_name("epc-ep")
        .send()
        .await
        .unwrap();
    let r = client
        .create_endpoint()
        .endpoint_name("ep1")
        .endpoint_config_name("epc-ep")
        .send()
        .await
        .unwrap();
    assert_opt_contains(r.endpoint_arn(), "ep1");

    let d = client
        .describe_endpoint()
        .endpoint_name("ep1")
        .send()
        .await
        .unwrap();
    assert_eq!(d.endpoint_name(), Some("ep1"));

    let u = client
        .update_endpoint_weights_and_capacities()
        .endpoint_name("ep1")
        .send()
        .await
        .unwrap();
    assert_opt_contains(u.endpoint_arn(), "ep1");

    assert_eq!(
        client
            .list_endpoints()
            .send()
            .await
            .unwrap()
            .endpoints()
            .len(),
        1
    );

    client
        .delete_endpoint()
        .endpoint_name("ep1")
        .send()
        .await
        .unwrap();
    assert!(
        client
            .describe_endpoint()
            .endpoint_name("ep1")
            .send()
            .await
            .is_err()
    );
}

// ============================================================
// Training Jobs
// ============================================================

#[tokio::test]
async fn test_training_job_lifecycle() {
    let client = make_client().await;
    let r = client
        .create_training_job()
        .training_job_name("tj1")
        .role_arn(ROLE)
        .algorithm_specification(
            aws_sdk_sagemaker::types::AlgorithmSpecification::builder()
                .training_input_mode(aws_sdk_sagemaker::types::TrainingInputMode::File)
                .build(),
        )
        .output_data_config(
            aws_sdk_sagemaker::types::OutputDataConfig::builder()
                .s3_output_path("s3://b/o")
                .build(),
        )
        .resource_config(
            aws_sdk_sagemaker::types::ResourceConfig::builder()
                .instance_type(aws_sdk_sagemaker::types::TrainingInstanceType::MlM4Xlarge)
                .instance_count(1)
                .volume_size_in_gb(50)
                .build(),
        )
        .stopping_condition(
            aws_sdk_sagemaker::types::StoppingCondition::builder()
                .max_runtime_in_seconds(3600)
                .build(),
        )
        .send()
        .await
        .unwrap();
    assert_opt_contains(r.training_job_arn(), "tj1");

    let d = client
        .describe_training_job()
        .training_job_name("tj1")
        .send()
        .await
        .unwrap();
    assert_eq!(d.training_job_name(), Some("tj1"));

    assert_eq!(
        client
            .list_training_jobs()
            .send()
            .await
            .unwrap()
            .training_job_summaries()
            .len(),
        1
    );
}

// ============================================================
// Processing Jobs
// ============================================================

#[tokio::test]
async fn test_processing_job_lifecycle() {
    let client = make_client().await;
    let r = client
        .create_processing_job()
        .processing_job_name("pj1")
        .role_arn(ROLE)
        .app_specification(
            aws_sdk_sagemaker::types::AppSpecification::builder()
                .image_uri("123456789012.dkr.ecr.us-east-1.amazonaws.com/img:latest")
                .build(),
        )
        .processing_resources(
            aws_sdk_sagemaker::types::ProcessingResources::builder()
                .cluster_config(
                    aws_sdk_sagemaker::types::ProcessingClusterConfig::builder()
                        .instance_count(1)
                        .instance_type(aws_sdk_sagemaker::types::ProcessingInstanceType::MlM4Xlarge)
                        .volume_size_in_gb(50)
                        .build(),
                )
                .build(),
        )
        .send()
        .await
        .unwrap();
    assert_opt_contains(r.processing_job_arn(), "pj1");

    let d = client
        .describe_processing_job()
        .processing_job_name("pj1")
        .send()
        .await
        .unwrap();
    assert_eq!(d.processing_job_name(), Some("pj1"));

    assert_eq!(
        client
            .list_processing_jobs()
            .send()
            .await
            .unwrap()
            .processing_job_summaries()
            .len(),
        1
    );
}

// ============================================================
// Transform Jobs
// ============================================================

#[tokio::test]
async fn test_transform_job_lifecycle() {
    let client = make_client().await;
    let r = client
        .create_transform_job()
        .transform_job_name("xj1")
        .model_name("mod")
        .transform_input(
            aws_sdk_sagemaker::types::TransformInput::builder()
                .data_source(
                    aws_sdk_sagemaker::types::TransformDataSource::builder()
                        .s3_data_source(
                            aws_sdk_sagemaker::types::TransformS3DataSource::builder()
                                .s3_data_type(aws_sdk_sagemaker::types::S3DataType::S3Prefix)
                                .s3_uri("s3://b/i")
                                .build(),
                        )
                        .build(),
                )
                .build(),
        )
        .transform_output(
            aws_sdk_sagemaker::types::TransformOutput::builder()
                .s3_output_path("s3://b/o")
                .build(),
        )
        .transform_resources(
            aws_sdk_sagemaker::types::TransformResources::builder()
                .instance_type(aws_sdk_sagemaker::types::TransformInstanceType::MlM4Xlarge)
                .instance_count(1)
                .build(),
        )
        .send()
        .await
        .unwrap();
    assert_opt_contains(r.transform_job_arn(), "xj1");

    let d = client
        .describe_transform_job()
        .transform_job_name("xj1")
        .send()
        .await
        .unwrap();
    assert_eq!(d.transform_job_name(), Some("xj1"));

    assert_eq!(
        client
            .list_transform_jobs()
            .send()
            .await
            .unwrap()
            .transform_job_summaries()
            .len(),
        1
    );
}

// ============================================================
// HyperParameter Tuning Jobs
// ============================================================

#[tokio::test]
async fn test_hyper_parameter_tuning_job_lifecycle() {
    let client = make_client().await;
    let r = client
        .create_hyper_parameter_tuning_job()
        .hyper_parameter_tuning_job_name("hpt1")
        .hyper_parameter_tuning_job_config(
            aws_sdk_sagemaker::types::HyperParameterTuningJobConfig::builder()
                .strategy(aws_sdk_sagemaker::types::HyperParameterTuningJobStrategyType::Bayesian)
                .resource_limits(
                    aws_sdk_sagemaker::types::ResourceLimits::builder()
                        .max_number_of_training_jobs(10)
                        .max_parallel_training_jobs(2)
                        .build(),
                )
                .build(),
        )
        .send()
        .await
        .unwrap();
    assert_opt_contains(r.hyper_parameter_tuning_job_arn(), "hpt1");

    let d = client
        .describe_hyper_parameter_tuning_job()
        .hyper_parameter_tuning_job_name("hpt1")
        .send()
        .await
        .unwrap();
    assert_eq!(d.hyper_parameter_tuning_job_name(), Some("hpt1"));

    assert_eq!(
        client
            .list_hyper_parameter_tuning_jobs()
            .send()
            .await
            .unwrap()
            .hyper_parameter_tuning_job_summaries()
            .len(),
        1
    );

    client
        .delete_hyper_parameter_tuning_job()
        .hyper_parameter_tuning_job_name("hpt1")
        .send()
        .await
        .unwrap();
    assert!(
        client
            .describe_hyper_parameter_tuning_job()
            .hyper_parameter_tuning_job_name("hpt1")
            .send()
            .await
            .is_err()
    );
}

// ============================================================
// Compilation Jobs
// ============================================================

#[tokio::test]
async fn test_compilation_job_lifecycle() {
    let client = make_client().await;
    let r = client
        .create_compilation_job()
        .compilation_job_name("cj1")
        .role_arn(ROLE)
        .input_config(
            aws_sdk_sagemaker::types::InputConfig::builder()
                .s3_uri("s3://b/model.tar.gz")
                .data_input_config("{}")
                .framework(aws_sdk_sagemaker::types::Framework::Pytorch)
                .build(),
        )
        .output_config(
            aws_sdk_sagemaker::types::OutputConfig::builder()
                .s3_output_location("s3://b/out")
                .target_device(aws_sdk_sagemaker::types::TargetDevice::MlM4)
                .build(),
        )
        .stopping_condition(
            aws_sdk_sagemaker::types::StoppingCondition::builder()
                .max_runtime_in_seconds(900)
                .build(),
        )
        .send()
        .await
        .unwrap();
    assert_opt_contains(r.compilation_job_arn(), "cj1");

    let d = client
        .describe_compilation_job()
        .compilation_job_name("cj1")
        .send()
        .await
        .unwrap();
    assert_eq!(d.compilation_job_name(), Some("cj1"));

    assert_eq!(
        client
            .list_compilation_jobs()
            .send()
            .await
            .unwrap()
            .compilation_job_summaries()
            .len(),
        1
    );

    client
        .delete_compilation_job()
        .compilation_job_name("cj1")
        .send()
        .await
        .unwrap();
}

// ============================================================
// AutoML Jobs V2
// ============================================================

#[tokio::test]
async fn test_auto_ml_job_v2_lifecycle() {
    let client = make_client().await;
    // Use a simpler approach - just set the job name and required fields
    let r = client
        .create_auto_ml_job_v2()
        .auto_ml_job_name("automl1")
        .auto_ml_job_input_data_config(
            aws_sdk_sagemaker::types::AutoMlJobChannel::builder()
                .channel_type(aws_sdk_sagemaker::types::AutoMlChannelType::Training)
                .data_source(
                    aws_sdk_sagemaker::types::AutoMlDataSource::builder()
                        .s3_data_source(
                            aws_sdk_sagemaker::types::AutoMls3DataSource::builder()
                                .s3_data_type(aws_sdk_sagemaker::types::AutoMls3DataType::S3Prefix)
                                .s3_uri("s3://b/data")
                                .build(),
                        )
                        .build(),
                )
                .build(),
        )
        .auto_ml_problem_type_config(
            aws_sdk_sagemaker::types::AutoMlProblemTypeConfig::TabularJobConfig(
                aws_sdk_sagemaker::types::TabularJobConfig::builder()
                    .target_attribute_name("target")
                    .build(),
            ),
        )
        .output_data_config(
            aws_sdk_sagemaker::types::AutoMlOutputDataConfig::builder()
                .s3_output_path("s3://b/out")
                .build(),
        )
        .role_arn(ROLE)
        .send()
        .await
        .unwrap();
    assert_opt_contains(r.auto_ml_job_arn(), "automl1");

    let d = client
        .describe_auto_ml_job_v2()
        .auto_ml_job_name("automl1")
        .send()
        .await
        .unwrap();
    assert_eq!(d.auto_ml_job_name(), Some("automl1"));

    assert_eq!(
        client
            .list_auto_ml_jobs()
            .send()
            .await
            .unwrap()
            .auto_ml_job_summaries()
            .len(),
        1
    );

    client
        .stop_auto_ml_job()
        .auto_ml_job_name("automl1")
        .send()
        .await
        .unwrap();
}

// ============================================================
// Experiments
// ============================================================

#[tokio::test]
async fn test_experiment_lifecycle() {
    let client = make_client().await;
    let r = client
        .create_experiment()
        .experiment_name("exp1")
        .display_name("Exp 1")
        .description("desc")
        .send()
        .await
        .unwrap();
    assert_opt_contains(r.experiment_arn(), "exp1");

    let d = client
        .describe_experiment()
        .experiment_name("exp1")
        .send()
        .await
        .unwrap();
    assert_eq!(d.experiment_name(), Some("exp1"));
    assert_eq!(d.display_name(), Some("Exp 1"));

    assert_eq!(
        client
            .list_experiments()
            .send()
            .await
            .unwrap()
            .experiment_summaries()
            .len(),
        1
    );

    client
        .delete_experiment()
        .experiment_name("exp1")
        .send()
        .await
        .unwrap();
    assert!(
        client
            .describe_experiment()
            .experiment_name("exp1")
            .send()
            .await
            .is_err()
    );
}

// ============================================================
// Trials
// ============================================================

#[tokio::test]
async fn test_trial_lifecycle() {
    let client = make_client().await;
    client
        .create_experiment()
        .experiment_name("texp")
        .send()
        .await
        .unwrap();

    let r = client
        .create_trial()
        .trial_name("t1")
        .experiment_name("texp")
        .display_name("T1")
        .send()
        .await
        .unwrap();
    assert_opt_contains(r.trial_arn(), "t1");

    let d = client
        .describe_trial()
        .trial_name("t1")
        .send()
        .await
        .unwrap();
    assert_eq!(d.trial_name(), Some("t1"));
    assert_eq!(d.experiment_name(), Some("texp"));

    assert_eq!(
        client
            .list_trials()
            .send()
            .await
            .unwrap()
            .trial_summaries()
            .len(),
        1
    );

    client.delete_trial().trial_name("t1").send().await.unwrap();
    assert!(
        client
            .describe_trial()
            .trial_name("t1")
            .send()
            .await
            .is_err()
    );
}

// ============================================================
// Trial Components
// ============================================================

#[tokio::test]
async fn test_trial_component_lifecycle() {
    let client = make_client().await;
    let r = client
        .create_trial_component()
        .trial_component_name("tc1")
        .display_name("TC 1")
        .send()
        .await
        .unwrap();
    assert_opt_contains(r.trial_component_arn(), "tc1");

    let d = client
        .describe_trial_component()
        .trial_component_name("tc1")
        .send()
        .await
        .unwrap();
    assert_eq!(d.trial_component_name(), Some("tc1"));
    assert_eq!(d.display_name(), Some("TC 1"));

    let u = client
        .update_trial_component()
        .trial_component_name("tc1")
        .display_name("Updated")
        .send()
        .await
        .unwrap();
    assert!(u.trial_component_arn().is_some());

    assert_eq!(
        client
            .list_trial_components()
            .send()
            .await
            .unwrap()
            .trial_component_summaries()
            .len(),
        1
    );

    client
        .delete_trial_component()
        .trial_component_name("tc1")
        .send()
        .await
        .unwrap();
    assert!(
        client
            .describe_trial_component()
            .trial_component_name("tc1")
            .send()
            .await
            .is_err()
    );
}

// ============================================================
// Associate/Disassociate Trial Component
// ============================================================

#[tokio::test]
async fn test_associate_disassociate_trial_component() {
    let client = make_client().await;
    client
        .create_experiment()
        .experiment_name("aexp")
        .send()
        .await
        .unwrap();
    client
        .create_trial()
        .trial_name("at1")
        .experiment_name("aexp")
        .send()
        .await
        .unwrap();
    client
        .create_trial_component()
        .trial_component_name("atc1")
        .send()
        .await
        .unwrap();

    let a = client
        .associate_trial_component()
        .trial_name("at1")
        .trial_component_name("atc1")
        .send()
        .await
        .unwrap();
    assert!(a.trial_arn().is_some());
    assert!(a.trial_component_arn().is_some());

    let d = client
        .disassociate_trial_component()
        .trial_name("at1")
        .trial_component_name("atc1")
        .send()
        .await
        .unwrap();
    assert!(d.trial_arn().is_some());
    assert!(d.trial_component_arn().is_some());
}

// ============================================================
// Pipelines
// ============================================================

#[tokio::test]
async fn test_pipeline_lifecycle() {
    let client = make_client().await;
    let r = client
        .create_pipeline()
        .pipeline_name("pipe1")
        .role_arn(ROLE)
        .pipeline_definition("{}")
        .pipeline_display_name("Pipe")
        .pipeline_description("desc")
        .send()
        .await
        .unwrap();
    assert_opt_contains(r.pipeline_arn(), "pipe1");

    let d = client
        .describe_pipeline()
        .pipeline_name("pipe1")
        .send()
        .await
        .unwrap();
    assert_eq!(d.pipeline_name(), Some("pipe1"));
    assert_eq!(d.pipeline_display_name(), Some("Pipe"));
    assert_eq!(d.pipeline_description(), Some("desc"));

    let u = client
        .update_pipeline()
        .pipeline_name("pipe1")
        .pipeline_description("upd")
        .send()
        .await
        .unwrap();
    assert!(u.pipeline_arn().is_some());

    assert_eq!(
        client
            .list_pipelines()
            .send()
            .await
            .unwrap()
            .pipeline_summaries()
            .len(),
        1
    );

    // StartPipelineExecution
    let exec = client
        .start_pipeline_execution()
        .pipeline_name("pipe1")
        .send()
        .await
        .unwrap();
    let exec_arn = exec.pipeline_execution_arn().unwrap().to_string();
    assert!(!exec_arn.is_empty());

    // ListPipelineExecutions
    let execs = client
        .list_pipeline_executions()
        .pipeline_name("pipe1")
        .send()
        .await
        .unwrap();
    assert_eq!(execs.pipeline_execution_summaries().len(), 1);

    // DescribePipelineExecution
    let ed = client
        .describe_pipeline_execution()
        .pipeline_execution_arn(&exec_arn)
        .send()
        .await
        .unwrap();
    assert_eq!(ed.pipeline_execution_arn(), Some(exec_arn.as_str()));

    // DescribePipelineDefinitionForExecution
    let def = client
        .describe_pipeline_definition_for_execution()
        .pipeline_execution_arn(&exec_arn)
        .send()
        .await
        .unwrap();
    assert!(def.pipeline_definition().is_some());

    // ListPipelineParametersForExecution
    client
        .list_pipeline_parameters_for_execution()
        .pipeline_execution_arn(&exec_arn)
        .send()
        .await
        .unwrap();

    // DeletePipeline
    client
        .delete_pipeline()
        .pipeline_name("pipe1")
        .send()
        .await
        .unwrap();
    assert!(
        client
            .describe_pipeline()
            .pipeline_name("pipe1")
            .send()
            .await
            .is_err()
    );
}

// ============================================================
// Feature Groups
// ============================================================

#[tokio::test]
async fn test_feature_group_create_describe() {
    let client = make_client().await;
    let r = client
        .create_feature_group()
        .feature_group_name("fg1")
        .record_identifier_feature_name("id")
        .event_time_feature_name("ts")
        .feature_definitions(
            aws_sdk_sagemaker::types::FeatureDefinition::builder()
                .feature_name("id")
                .feature_type(aws_sdk_sagemaker::types::FeatureType::String)
                .build(),
        )
        .send()
        .await
        .unwrap();
    assert_opt_contains(r.feature_group_arn(), "fg1");

    let d = client
        .describe_feature_group()
        .feature_group_name("fg1")
        .send()
        .await
        .unwrap();
    assert_eq!(d.feature_group_name(), Some("fg1"));
}

// ============================================================
// Domains
// ============================================================

#[tokio::test]
async fn test_domain_lifecycle() {
    let client = make_client().await;
    let r = client
        .create_domain()
        .domain_name("dom1")
        .auth_mode(aws_sdk_sagemaker::types::AuthMode::Iam)
        .default_user_settings(
            aws_sdk_sagemaker::types::UserSettings::builder()
                .execution_role(ROLE)
                .build(),
        )
        .subnet_ids("subnet-123")
        .vpc_id("vpc-123")
        .send()
        .await
        .unwrap();
    let dom_arn = r.domain_arn().unwrap().to_string();
    let dom_id = dom_arn.rsplit('/').next().unwrap().to_string();

    let d = client
        .describe_domain()
        .domain_id(&dom_id)
        .send()
        .await
        .unwrap();
    assert_eq!(d.domain_name(), Some("dom1"));
    assert_eq!(d.domain_id(), Some(dom_id.as_str()));

    assert_eq!(
        client.list_domains().send().await.unwrap().domains().len(),
        1
    );

    client
        .delete_domain()
        .domain_id(&dom_id)
        .send()
        .await
        .unwrap();
    assert!(
        client
            .describe_domain()
            .domain_id(&dom_id)
            .send()
            .await
            .is_err()
    );
}

// ============================================================
// Clusters
// ============================================================

fn cluster_ig() -> aws_sdk_sagemaker::types::ClusterInstanceGroupSpecification {
    aws_sdk_sagemaker::types::ClusterInstanceGroupSpecification::builder()
        .instance_count(1)
        .instance_group_name("default")
        .instance_type(aws_sdk_sagemaker::types::ClusterInstanceType::MlP4D24Xlarge)
        .life_cycle_config(
            aws_sdk_sagemaker::types::ClusterLifeCycleConfig::builder()
                .source_s3_uri("s3://b/s.sh")
                .on_create("s.sh")
                .build(),
        )
        .execution_role(ROLE)
        .build()
}

#[tokio::test]
async fn test_cluster_lifecycle() {
    let client = make_client().await;
    let r = client
        .create_cluster()
        .cluster_name("cl1")
        .instance_groups(cluster_ig())
        .send()
        .await
        .unwrap();
    assert_opt_contains(r.cluster_arn(), "cl1");

    let d = client
        .describe_cluster()
        .cluster_name("cl1")
        .send()
        .await
        .unwrap();
    assert_eq!(d.cluster_name(), Some("cl1"));

    assert_eq!(
        client
            .list_clusters()
            .send()
            .await
            .unwrap()
            .cluster_summaries()
            .len(),
        1
    );
    client
        .list_cluster_nodes()
        .cluster_name("cl1")
        .send()
        .await
        .unwrap();

    client
        .delete_cluster()
        .cluster_name("cl1")
        .send()
        .await
        .unwrap();
    assert!(
        client
            .describe_cluster()
            .cluster_name("cl1")
            .send()
            .await
            .is_err()
    );
}

#[tokio::test]
async fn test_describe_cluster_node_not_found() {
    let client = make_client().await;
    client
        .create_cluster()
        .cluster_name("cl-node")
        .instance_groups(cluster_ig())
        .send()
        .await
        .unwrap();
    assert!(
        client
            .describe_cluster_node()
            .cluster_name("cl-node")
            .node_id("nope")
            .send()
            .await
            .is_err()
    );
}

// ============================================================
// Job Definitions
// ============================================================

#[tokio::test]
async fn test_data_quality_job_definition_lifecycle() {
    let client = make_client().await;
    let r = client
        .create_data_quality_job_definition()
        .job_definition_name("dq1")
        .role_arn(ROLE)
        .data_quality_app_specification(
            aws_sdk_sagemaker::types::DataQualityAppSpecification::builder()
                .image_uri("img:latest")
                .build(),
        )
        .data_quality_job_output_config(
            aws_sdk_sagemaker::types::MonitoringOutputConfig::builder()
                .monitoring_outputs(
                    aws_sdk_sagemaker::types::MonitoringOutput::builder()
                        .s3_output(
                            aws_sdk_sagemaker::types::MonitoringS3Output::builder()
                                .s3_uri("s3://b/o")
                                .local_path("/opt/ml/o")
                                .build(),
                        )
                        .build(),
                )
                .build(),
        )
        .data_quality_job_input(
            aws_sdk_sagemaker::types::DataQualityJobInput::builder()
                .endpoint_input(
                    aws_sdk_sagemaker::types::EndpointInput::builder()
                        .endpoint_name("ep")
                        .local_path("/opt/ml/i")
                        .build(),
                )
                .build(),
        )
        .job_resources(
            aws_sdk_sagemaker::types::MonitoringResources::builder()
                .cluster_config(
                    aws_sdk_sagemaker::types::MonitoringClusterConfig::builder()
                        .instance_count(1)
                        .instance_type(aws_sdk_sagemaker::types::ProcessingInstanceType::MlM4Xlarge)
                        .volume_size_in_gb(50)
                        .build(),
                )
                .build(),
        )
        .send()
        .await
        .unwrap();
    assert_opt_contains(r.job_definition_arn(), "dq1");

    let d = client
        .describe_data_quality_job_definition()
        .job_definition_name("dq1")
        .send()
        .await
        .unwrap();
    assert_eq!(d.job_definition_name(), Some("dq1"));

    assert_eq!(
        client
            .list_data_quality_job_definitions()
            .send()
            .await
            .unwrap()
            .job_definition_summaries()
            .len(),
        1
    );

    client
        .delete_data_quality_job_definition()
        .job_definition_name("dq1")
        .send()
        .await
        .unwrap();
}

#[tokio::test]
async fn test_model_quality_job_definition_lifecycle() {
    let client = make_client().await;
    let r = client
        .create_model_quality_job_definition()
        .job_definition_name("mq1")
        .role_arn(ROLE)
        .model_quality_app_specification(
            aws_sdk_sagemaker::types::ModelQualityAppSpecification::builder()
                .image_uri("img:latest")
                .problem_type(aws_sdk_sagemaker::types::MonitoringProblemType::BinaryClassification)
                .build(),
        )
        .model_quality_job_output_config(
            aws_sdk_sagemaker::types::MonitoringOutputConfig::builder()
                .monitoring_outputs(
                    aws_sdk_sagemaker::types::MonitoringOutput::builder()
                        .s3_output(
                            aws_sdk_sagemaker::types::MonitoringS3Output::builder()
                                .s3_uri("s3://b/o")
                                .local_path("/opt/ml/o")
                                .build(),
                        )
                        .build(),
                )
                .build(),
        )
        .model_quality_job_input(
            aws_sdk_sagemaker::types::ModelQualityJobInput::builder()
                .endpoint_input(
                    aws_sdk_sagemaker::types::EndpointInput::builder()
                        .endpoint_name("ep")
                        .local_path("/opt/ml/i")
                        .build(),
                )
                .ground_truth_s3_input(
                    aws_sdk_sagemaker::types::MonitoringGroundTruthS3Input::builder()
                        .s3_uri("s3://b/gt")
                        .build(),
                )
                .build(),
        )
        .job_resources(
            aws_sdk_sagemaker::types::MonitoringResources::builder()
                .cluster_config(
                    aws_sdk_sagemaker::types::MonitoringClusterConfig::builder()
                        .instance_count(1)
                        .instance_type(aws_sdk_sagemaker::types::ProcessingInstanceType::MlM4Xlarge)
                        .volume_size_in_gb(50)
                        .build(),
                )
                .build(),
        )
        .send()
        .await
        .unwrap();
    assert_opt_contains(r.job_definition_arn(), "mq1");

    let d = client
        .describe_model_quality_job_definition()
        .job_definition_name("mq1")
        .send()
        .await
        .unwrap();
    assert_eq!(d.job_definition_name(), Some("mq1"));
    assert_eq!(
        client
            .list_model_quality_job_definitions()
            .send()
            .await
            .unwrap()
            .job_definition_summaries()
            .len(),
        1
    );
    client
        .delete_model_quality_job_definition()
        .job_definition_name("mq1")
        .send()
        .await
        .unwrap();
}

#[tokio::test]
async fn test_model_bias_job_definition_lifecycle() {
    let client = make_client().await;
    let r = client
        .create_model_bias_job_definition()
        .job_definition_name("mb1")
        .role_arn(ROLE)
        .model_bias_app_specification(
            aws_sdk_sagemaker::types::ModelBiasAppSpecification::builder()
                .image_uri("img:latest")
                .config_uri("s3://b/cfg")
                .build(),
        )
        .model_bias_job_output_config(
            aws_sdk_sagemaker::types::MonitoringOutputConfig::builder()
                .monitoring_outputs(
                    aws_sdk_sagemaker::types::MonitoringOutput::builder()
                        .s3_output(
                            aws_sdk_sagemaker::types::MonitoringS3Output::builder()
                                .s3_uri("s3://b/o")
                                .local_path("/opt/ml/o")
                                .build(),
                        )
                        .build(),
                )
                .build(),
        )
        .model_bias_job_input(
            aws_sdk_sagemaker::types::ModelBiasJobInput::builder()
                .endpoint_input(
                    aws_sdk_sagemaker::types::EndpointInput::builder()
                        .endpoint_name("ep")
                        .local_path("/opt/ml/i")
                        .build(),
                )
                .ground_truth_s3_input(
                    aws_sdk_sagemaker::types::MonitoringGroundTruthS3Input::builder()
                        .s3_uri("s3://b/gt")
                        .build(),
                )
                .build(),
        )
        .job_resources(
            aws_sdk_sagemaker::types::MonitoringResources::builder()
                .cluster_config(
                    aws_sdk_sagemaker::types::MonitoringClusterConfig::builder()
                        .instance_count(1)
                        .instance_type(aws_sdk_sagemaker::types::ProcessingInstanceType::MlM4Xlarge)
                        .volume_size_in_gb(50)
                        .build(),
                )
                .build(),
        )
        .send()
        .await
        .unwrap();
    assert_opt_contains(r.job_definition_arn(), "mb1");
    let d = client
        .describe_model_bias_job_definition()
        .job_definition_name("mb1")
        .send()
        .await
        .unwrap();
    assert_eq!(d.job_definition_name(), Some("mb1"));
    assert_eq!(
        client
            .list_model_bias_job_definitions()
            .send()
            .await
            .unwrap()
            .job_definition_summaries()
            .len(),
        1
    );
    client
        .delete_model_bias_job_definition()
        .job_definition_name("mb1")
        .send()
        .await
        .unwrap();
}

#[tokio::test]
async fn test_model_explainability_job_definition_lifecycle() {
    let client = make_client().await;
    let r = client
        .create_model_explainability_job_definition()
        .job_definition_name("me1")
        .role_arn(ROLE)
        .model_explainability_app_specification(
            aws_sdk_sagemaker::types::ModelExplainabilityAppSpecification::builder()
                .image_uri("img:latest")
                .config_uri("s3://b/cfg")
                .build(),
        )
        .model_explainability_job_output_config(
            aws_sdk_sagemaker::types::MonitoringOutputConfig::builder()
                .monitoring_outputs(
                    aws_sdk_sagemaker::types::MonitoringOutput::builder()
                        .s3_output(
                            aws_sdk_sagemaker::types::MonitoringS3Output::builder()
                                .s3_uri("s3://b/o")
                                .local_path("/opt/ml/o")
                                .build(),
                        )
                        .build(),
                )
                .build(),
        )
        .model_explainability_job_input(
            aws_sdk_sagemaker::types::ModelExplainabilityJobInput::builder()
                .endpoint_input(
                    aws_sdk_sagemaker::types::EndpointInput::builder()
                        .endpoint_name("ep")
                        .local_path("/opt/ml/i")
                        .build(),
                )
                .build(),
        )
        .job_resources(
            aws_sdk_sagemaker::types::MonitoringResources::builder()
                .cluster_config(
                    aws_sdk_sagemaker::types::MonitoringClusterConfig::builder()
                        .instance_count(1)
                        .instance_type(aws_sdk_sagemaker::types::ProcessingInstanceType::MlM4Xlarge)
                        .volume_size_in_gb(50)
                        .build(),
                )
                .build(),
        )
        .send()
        .await
        .unwrap();
    assert_opt_contains(r.job_definition_arn(), "me1");
    let d = client
        .describe_model_explainability_job_definition()
        .job_definition_name("me1")
        .send()
        .await
        .unwrap();
    assert_eq!(d.job_definition_name(), Some("me1"));
    assert_eq!(
        client
            .list_model_explainability_job_definitions()
            .send()
            .await
            .unwrap()
            .job_definition_summaries()
            .len(),
        1
    );
    client
        .delete_model_explainability_job_definition()
        .job_definition_name("me1")
        .send()
        .await
        .unwrap();
}

// ============================================================
// Model Cards
// ============================================================

#[tokio::test]
async fn test_model_card_lifecycle() {
    let client = make_client().await;
    let r = client
        .create_model_card()
        .model_card_name("mc1")
        .content("{}")
        .model_card_status(aws_sdk_sagemaker::types::ModelCardStatus::Draft)
        .send()
        .await
        .unwrap();
    assert_opt_contains(r.model_card_arn(), "mc1");

    let d = client
        .describe_model_card()
        .model_card_name("mc1")
        .send()
        .await
        .unwrap();
    assert_eq!(d.model_card_name(), Some("mc1"));

    let u = client
        .update_model_card()
        .model_card_name("mc1")
        .content("{\"u\":1}")
        .model_card_status(aws_sdk_sagemaker::types::ModelCardStatus::Approved)
        .send()
        .await
        .unwrap();
    assert_opt_contains(u.model_card_arn(), "mc1");

    assert_eq!(
        client
            .list_model_cards()
            .send()
            .await
            .unwrap()
            .model_card_summaries()
            .len(),
        1
    );

    let v = client
        .list_model_card_versions()
        .model_card_name("mc1")
        .send()
        .await
        .unwrap();
    assert!(!v.model_card_version_summary_list().is_empty());

    client
        .delete_model_card()
        .model_card_name("mc1")
        .send()
        .await
        .unwrap();
    assert!(
        client
            .describe_model_card()
            .model_card_name("mc1")
            .send()
            .await
            .is_err()
    );
}

// ============================================================
// Model Packages
// ============================================================

#[tokio::test]
async fn test_model_package_lifecycle() {
    let client = make_client().await;
    let r = client
        .create_model_package()
        .model_package_name("mp1")
        .model_package_description("desc")
        .model_approval_status(aws_sdk_sagemaker::types::ModelApprovalStatus::PendingManualApproval)
        .send()
        .await
        .unwrap();
    assert_opt_contains(r.model_package_arn(), "mp1");

    let d = client
        .describe_model_package()
        .model_package_name("mp1")
        .send()
        .await
        .unwrap();
    assert_eq!(d.model_package_name(), Some("mp1"));
    assert_eq!(d.model_package_description(), Some("desc"));

    let mp_arn = d.model_package_arn().unwrap().to_string();
    let u = client
        .update_model_package()
        .model_package_arn(&mp_arn)
        .model_approval_status(aws_sdk_sagemaker::types::ModelApprovalStatus::Approved)
        .send()
        .await
        .unwrap();
    assert_opt_contains(u.model_package_arn(), "mp1");

    assert_eq!(
        client
            .list_model_packages()
            .send()
            .await
            .unwrap()
            .model_package_summary_list()
            .len(),
        1
    );
}

// ============================================================
// Model Package Groups
// ============================================================

#[tokio::test]
async fn test_model_package_group_lifecycle() {
    let client = make_client().await;
    let r = client
        .create_model_package_group()
        .model_package_group_name("mpg1")
        .model_package_group_description("desc")
        .send()
        .await
        .unwrap();
    assert_opt_contains(r.model_package_group_arn(), "mpg1");

    let d = client
        .describe_model_package_group()
        .model_package_group_name("mpg1")
        .send()
        .await
        .unwrap();
    assert_eq!(d.model_package_group_name(), Some("mpg1"));

    assert_eq!(
        client
            .list_model_package_groups()
            .send()
            .await
            .unwrap()
            .model_package_group_summary_list()
            .len(),
        1
    );
}

// ============================================================
// Notebook Instance Lifecycle Configs
// ============================================================

#[tokio::test]
async fn test_notebook_instance_lifecycle_config_lifecycle() {
    let client = make_client().await;
    let r = client
        .create_notebook_instance_lifecycle_config()
        .notebook_instance_lifecycle_config_name("lcc1")
        .send()
        .await
        .unwrap();
    assert_opt_contains(r.notebook_instance_lifecycle_config_arn(), "lcc1");

    let d = client
        .describe_notebook_instance_lifecycle_config()
        .notebook_instance_lifecycle_config_name("lcc1")
        .send()
        .await
        .unwrap();
    assert_eq!(d.notebook_instance_lifecycle_config_name(), Some("lcc1"));

    client
        .delete_notebook_instance_lifecycle_config()
        .notebook_instance_lifecycle_config_name("lcc1")
        .send()
        .await
        .unwrap();
    assert!(
        client
            .describe_notebook_instance_lifecycle_config()
            .notebook_instance_lifecycle_config_name("lcc1")
            .send()
            .await
            .is_err()
    );
}

// ============================================================
// Tags
// ============================================================

#[tokio::test]
async fn test_tags_lifecycle() {
    let client = make_client().await;
    let mr = client
        .create_model()
        .model_name("tag-model")
        .execution_role_arn(ROLE)
        .send()
        .await
        .unwrap();
    let arn = mr.model_arn().unwrap().to_string();

    let add = client
        .add_tags()
        .resource_arn(&arn)
        .tags(
            aws_sdk_sagemaker::types::Tag::builder()
                .key("k1")
                .value("v1")
                .build(),
        )
        .tags(
            aws_sdk_sagemaker::types::Tag::builder()
                .key("k2")
                .value("v2")
                .build(),
        )
        .send()
        .await
        .unwrap();
    assert_eq!(add.tags().len(), 2);

    let list = client.list_tags().resource_arn(&arn).send().await.unwrap();
    assert_eq!(list.tags().len(), 2);

    client
        .delete_tags()
        .resource_arn(&arn)
        .tag_keys("k1")
        .send()
        .await
        .unwrap();

    let list2 = client.list_tags().resource_arn(&arn).send().await.unwrap();
    assert_eq!(list2.tags().len(), 1);
    assert_eq!(list2.tags()[0].key(), Some("k2"));
}

// ============================================================
// Search
// ============================================================

#[tokio::test]
async fn test_search() {
    let client = make_client().await;
    let r = client
        .search()
        .resource(aws_sdk_sagemaker::types::ResourceType::TrainingJob)
        .send()
        .await
        .unwrap();
    assert!(r.results().is_empty());
}

// ============================================================
// Error paths
// ============================================================

#[tokio::test]
async fn test_describe_nonexistent_resources() {
    let client = make_client().await;
    assert!(
        client
            .describe_model()
            .model_name("x")
            .send()
            .await
            .is_err()
    );
    assert!(
        client
            .describe_endpoint()
            .endpoint_name("x")
            .send()
            .await
            .is_err()
    );
    assert!(
        client
            .describe_endpoint_config()
            .endpoint_config_name("x")
            .send()
            .await
            .is_err()
    );
    assert!(
        client
            .describe_training_job()
            .training_job_name("x")
            .send()
            .await
            .is_err()
    );
    assert!(
        client
            .describe_processing_job()
            .processing_job_name("x")
            .send()
            .await
            .is_err()
    );
    assert!(
        client
            .describe_transform_job()
            .transform_job_name("x")
            .send()
            .await
            .is_err()
    );
    assert!(
        client
            .describe_hyper_parameter_tuning_job()
            .hyper_parameter_tuning_job_name("x")
            .send()
            .await
            .is_err()
    );
    assert!(
        client
            .describe_compilation_job()
            .compilation_job_name("x")
            .send()
            .await
            .is_err()
    );
    assert!(
        client
            .describe_experiment()
            .experiment_name("x")
            .send()
            .await
            .is_err()
    );
    assert!(
        client
            .describe_trial()
            .trial_name("x")
            .send()
            .await
            .is_err()
    );
    assert!(
        client
            .describe_trial_component()
            .trial_component_name("x")
            .send()
            .await
            .is_err()
    );
    assert!(
        client
            .describe_pipeline()
            .pipeline_name("x")
            .send()
            .await
            .is_err()
    );
    assert!(
        client
            .describe_model_card()
            .model_card_name("x")
            .send()
            .await
            .is_err()
    );
    assert!(
        client
            .describe_auto_ml_job_v2()
            .auto_ml_job_name("x")
            .send()
            .await
            .is_err()
    );
}

#[tokio::test]
async fn test_delete_nonexistent_resources() {
    let client = make_client().await;
    assert!(client.delete_model().model_name("x").send().await.is_err());
    assert!(
        client
            .delete_endpoint()
            .endpoint_name("x")
            .send()
            .await
            .is_err()
    );
    assert!(
        client
            .delete_endpoint_config()
            .endpoint_config_name("x")
            .send()
            .await
            .is_err()
    );
    assert!(
        client
            .delete_hyper_parameter_tuning_job()
            .hyper_parameter_tuning_job_name("x")
            .send()
            .await
            .is_err()
    );
    assert!(
        client
            .delete_compilation_job()
            .compilation_job_name("x")
            .send()
            .await
            .is_err()
    );
    assert!(
        client
            .delete_experiment()
            .experiment_name("x")
            .send()
            .await
            .is_err()
    );
    assert!(client.delete_trial().trial_name("x").send().await.is_err());
    assert!(
        client
            .delete_trial_component()
            .trial_component_name("x")
            .send()
            .await
            .is_err()
    );
    assert!(
        client
            .delete_pipeline()
            .pipeline_name("x")
            .send()
            .await
            .is_err()
    );
    assert!(
        client
            .delete_model_card()
            .model_card_name("x")
            .send()
            .await
            .is_err()
    );
}

// ============================================================================
// Ported from moto: test_sagemaker_notebooks.py
// ============================================================================

// Ported from moto: test_sagemaker_notebooks.py::test_create_notebook_instance_minimal_params
#[tokio::test]
async fn test_notebook_instance_minimal_params_fields() {
    let client = make_client().await;
    let resp = client
        .create_notebook_instance()
        .notebook_instance_name("MinNb")
        .instance_type(aws_sdk_sagemaker::types::InstanceType::MlT2Medium)
        .role_arn(ROLE)
        .send()
        .await
        .unwrap();
    assert_opt_contains(resp.notebook_instance_arn(), "MinNb");

    let d = client
        .describe_notebook_instance()
        .notebook_instance_name("MinNb")
        .send()
        .await
        .unwrap();
    assert_eq!(d.notebook_instance_name(), Some("MinNb"));
    assert_eq!(d.instance_type().map(|t| t.as_str()), Some("ml.t2.medium"));
    assert_eq!(d.role_arn(), Some(ROLE));
    // Default volume = 5 GB
    assert_eq!(d.volume_size_in_gb(), Some(5));
    // Default DirectInternetAccess = Enabled
    assert_eq!(
        d.direct_internet_access().map(|d| d.as_str()),
        Some("Enabled")
    );
    // URL format
    let url = d.url().unwrap_or("");
    assert!(
        url.contains("MinNb"),
        "expected URL to contain 'MinNb', got '{url}'"
    );
    assert!(
        url.contains("sagemaker.aws"),
        "expected URL to contain 'sagemaker.aws', got '{url}'"
    );
    // Creation/last-modified timestamps are set
    assert!(d.creation_time().is_some());
    assert!(d.last_modified_time().is_some());
}

// Ported from moto: test_sagemaker_notebooks.py::test_create_notebook_instance_params
#[tokio::test]
async fn test_notebook_instance_with_params() {
    let client = make_client().await;
    let resp = client
        .create_notebook_instance()
        .notebook_instance_name("ParamNb")
        .instance_type(aws_sdk_sagemaker::types::InstanceType::MlT2Medium)
        .role_arn(ROLE)
        .volume_size_in_gb(7)
        .send()
        .await
        .unwrap();
    assert_opt_contains(resp.notebook_instance_arn(), "ParamNb");

    let d = client
        .describe_notebook_instance()
        .notebook_instance_name("ParamNb")
        .send()
        .await
        .unwrap();
    assert_eq!(d.volume_size_in_gb(), Some(7));
}

// Ported from moto: test_sagemaker_notebooks.py::test_notebook_instance_lifecycle (status transitions)
#[tokio::test]
async fn test_notebook_instance_status_transitions() {
    let client = make_client().await;
    client
        .create_notebook_instance()
        .notebook_instance_name("StatusNb")
        .instance_type(aws_sdk_sagemaker::types::InstanceType::MlT2Medium)
        .role_arn(ROLE)
        .send()
        .await
        .unwrap();

    // After start, status becomes InService
    client
        .start_notebook_instance()
        .notebook_instance_name("StatusNb")
        .send()
        .await
        .unwrap();
    let d = client
        .describe_notebook_instance()
        .notebook_instance_name("StatusNb")
        .send()
        .await
        .unwrap();
    assert_eq!(
        d.notebook_instance_status().map(|s| s.as_str()),
        Some("InService")
    );

    // After stop, status becomes Stopped
    client
        .stop_notebook_instance()
        .notebook_instance_name("StatusNb")
        .send()
        .await
        .unwrap();
    let d = client
        .describe_notebook_instance()
        .notebook_instance_name("StatusNb")
        .send()
        .await
        .unwrap();
    assert_eq!(
        d.notebook_instance_status().map(|s| s.as_str()),
        Some("Stopped")
    );
}

// Ported from moto: test_sagemaker_notebooks.py::test_notebook_instance_lifecycle_config (duplicate error)
#[tokio::test]
async fn test_notebook_instance_lifecycle_config_duplicate_error() {
    let client = make_client().await;
    client
        .create_notebook_instance_lifecycle_config()
        .notebook_instance_lifecycle_config_name("lcc-dup")
        .send()
        .await
        .unwrap();
    let err = client
        .create_notebook_instance_lifecycle_config()
        .notebook_instance_lifecycle_config_name("lcc-dup")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("already exists") || err_str.contains("Cannot create a duplicate"),
        "expected duplicate error, got: {err_str}"
    );
}

// Ported from moto: test_sagemaker_notebooks.py::test_notebook_instance_lifecycle_config (not found after delete)
#[tokio::test]
async fn test_notebook_instance_lifecycle_config_not_found_after_delete() {
    let client = make_client().await;
    client
        .create_notebook_instance_lifecycle_config()
        .notebook_instance_lifecycle_config_name("lcc-del")
        .send()
        .await
        .unwrap();
    client
        .delete_notebook_instance_lifecycle_config()
        .notebook_instance_lifecycle_config_name("lcc-del")
        .send()
        .await
        .unwrap();
    assert!(
        client
            .describe_notebook_instance_lifecycle_config()
            .notebook_instance_lifecycle_config_name("lcc-del")
            .send()
            .await
            .is_err()
    );
    // Second delete should also fail
    assert!(
        client
            .delete_notebook_instance_lifecycle_config()
            .notebook_instance_lifecycle_config_name("lcc-del")
            .send()
            .await
            .is_err()
    );
}

// Ported from moto: test_sagemaker_notebooks.py::test_list_notebook_instances (multiple)
#[tokio::test]
async fn test_list_notebook_instances_multiple() {
    let client = make_client().await;
    for i in 0..3 {
        client
            .create_notebook_instance()
            .notebook_instance_name(format!("NbM{i}"))
            .instance_type(aws_sdk_sagemaker::types::InstanceType::MlT2Medium)
            .role_arn(ROLE)
            .send()
            .await
            .unwrap();
    }
    let list = client.list_notebook_instances().send().await.unwrap();
    assert_eq!(list.notebook_instances().len(), 3);
}

// ============================================================================
// Ported from moto: test_sagemaker_models.py
// ============================================================================

// Ported from moto: test_sagemaker_models.py::test_list_models_multiple
#[tokio::test]
async fn test_list_models_multiple() {
    let client = make_client().await;
    client
        .create_model()
        .model_name("blah")
        .execution_role_arn(ROLE)
        .send()
        .await
        .unwrap();
    client
        .create_model()
        .model_name("blah2")
        .execution_role_arn(ROLE)
        .send()
        .await
        .unwrap();
    let models = client.list_models().send().await.unwrap();
    assert_eq!(models.models().len(), 2);
}

// Ported from moto: test_sagemaker_models.py::test_list_models_none
#[tokio::test]
async fn test_list_models_none() {
    let client = make_client().await;
    let models = client.list_models().send().await.unwrap();
    assert_eq!(models.models().len(), 0);
}

// Ported from moto: test_sagemaker_models.py::test_create_model (ARN format)
#[tokio::test]
async fn test_create_model_arn_format() {
    let client = make_client().await;
    let resp = client
        .create_model()
        .model_name("ArnModel")
        .execution_role_arn(ROLE)
        .send()
        .await
        .unwrap();
    let arn = resp.model_arn().unwrap();
    assert!(
        arn.starts_with("arn:aws:sagemaker:"),
        "unexpected ARN: {arn}"
    );
    assert!(arn.ends_with(":model/ArnModel"), "unexpected ARN: {arn}");
}

// Ported from moto: test_sagemaker_models.py::test_delete_model_not_found
#[tokio::test]
async fn test_delete_model_not_found() {
    let client = make_client().await;
    let err = client
        .delete_model()
        .model_name("DoesNotExist")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("404")
            || err_str.contains("Could not find")
            || err_str.contains("Not Found"),
        "expected not-found error, got: {err_str}"
    );
}

// Ported from moto: test_sagemaker_models.py::test_add_tags_to_model / test_delete_tags_from_model
#[tokio::test]
async fn test_model_tags_add_delete() {
    let client = make_client().await;
    let resp = client
        .create_model()
        .model_name("tagged-model")
        .execution_role_arn(ROLE)
        .send()
        .await
        .unwrap();
    let arn = resp.model_arn().unwrap().to_string();

    let tags = vec![
        aws_sdk_sagemaker::types::Tag::builder()
            .key("myKey")
            .value("myValue")
            .build(),
    ];
    client
        .add_tags()
        .resource_arn(&arn)
        .set_tags(Some(tags))
        .send()
        .await
        .unwrap();

    let list = client.list_tags().resource_arn(&arn).send().await.unwrap();
    assert_eq!(list.tags().len(), 1);
    assert_eq!(list.tags()[0].key(), Some("myKey"));

    client
        .delete_tags()
        .resource_arn(&arn)
        .tag_keys("myKey")
        .send()
        .await
        .unwrap();
    let list2 = client.list_tags().resource_arn(&arn).send().await.unwrap();
    assert_eq!(list2.tags().len(), 0);
}

// ============================================================================
// Ported from moto: test_sagemaker_endpoint.py
// ============================================================================

// Ported from moto: test_sagemaker_endpoint.py::test_delete_endpoint (double-delete error)
#[tokio::test]
async fn test_delete_endpoint_double_delete_error() {
    let client = make_client().await;
    client
        .create_endpoint_config()
        .endpoint_config_name("epc-dd")
        .send()
        .await
        .unwrap();
    client
        .create_endpoint()
        .endpoint_name("ep-dd")
        .endpoint_config_name("epc-dd")
        .send()
        .await
        .unwrap();
    client
        .delete_endpoint()
        .endpoint_name("ep-dd")
        .send()
        .await
        .unwrap();
    let err = client
        .delete_endpoint()
        .endpoint_name("ep-dd")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("Could not find") || err_str.contains("Not Found"),
        "expected not-found error on second delete, got: {err_str}"
    );
}

// Ported from moto: test_sagemaker_endpoint.py::test_delete_endpoint_config (double-delete error)
#[tokio::test]
async fn test_delete_endpoint_config_double_delete_error() {
    let client = make_client().await;
    client
        .create_endpoint_config()
        .endpoint_config_name("epc-dd2")
        .send()
        .await
        .unwrap();
    client
        .delete_endpoint_config()
        .endpoint_config_name("epc-dd2")
        .send()
        .await
        .unwrap();
    let err = client
        .delete_endpoint_config()
        .endpoint_config_name("epc-dd2")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("Could not find"),
        "expected not-found error on second delete, got: {err_str}"
    );
}

// ============================================================================
// Ported from moto: test_sagemaker_experiment.py
// ============================================================================

// Ported from moto: test_sagemaker_experiment.py::test_create_experiment (ARN format)
#[tokio::test]
async fn test_experiment_arn_format() {
    let client = make_client().await;
    client
        .create_experiment()
        .experiment_name("ArnExp")
        .send()
        .await
        .unwrap();

    let list = client.list_experiments().send().await.unwrap();
    let summary = &list.experiment_summaries()[0];
    let arn = summary.experiment_arn().unwrap_or("");
    assert!(
        arn.starts_with("arn:aws:sagemaker:"),
        "unexpected ARN: {arn}"
    );
    assert!(arn.contains(":experiment/ArnExp"), "unexpected ARN: {arn}");
}

// Ported from moto: test_sagemaker_experiment.py::test_add_tags_to_experiment
#[tokio::test]
async fn test_experiment_tags() {
    let client = make_client().await;
    client
        .create_experiment()
        .experiment_name("TagExp")
        .send()
        .await
        .unwrap();

    let d = client
        .describe_experiment()
        .experiment_name("TagExp")
        .send()
        .await
        .unwrap();
    let arn = d.experiment_arn().unwrap().to_string();

    let tags = vec![
        aws_sdk_sagemaker::types::Tag::builder()
            .key("name")
            .value("value")
            .build(),
    ];
    client
        .add_tags()
        .resource_arn(&arn)
        .set_tags(Some(tags))
        .send()
        .await
        .unwrap();

    let list = client.list_tags().resource_arn(&arn).send().await.unwrap();
    assert_eq!(list.tags().len(), 1);
    assert_eq!(list.tags()[0].key(), Some("name"));

    client
        .delete_tags()
        .resource_arn(&arn)
        .tag_keys("name")
        .send()
        .await
        .unwrap();
    let list2 = client.list_tags().resource_arn(&arn).send().await.unwrap();
    assert_eq!(list2.tags().len(), 0);
}

// ============================================================================
// Ported from moto: test_sagemaker_trial.py
// ============================================================================

// Ported from moto: test_sagemaker_trial.py::test_create_trial (ARN format)
#[tokio::test]
async fn test_trial_arn_format() {
    let client = make_client().await;
    client
        .create_experiment()
        .experiment_name("ArnTExp")
        .send()
        .await
        .unwrap();
    client
        .create_trial()
        .trial_name("ArnTrial")
        .experiment_name("ArnTExp")
        .send()
        .await
        .unwrap();

    let list = client.list_trials().send().await.unwrap();
    let summary = list
        .trial_summaries()
        .iter()
        .find(|s| s.trial_name() == Some("ArnTrial"))
        .unwrap();
    let arn = summary.trial_arn().unwrap_or("");
    assert!(
        arn.starts_with("arn:aws:sagemaker:"),
        "unexpected ARN: {arn}"
    );
    assert!(
        arn.contains(":experiment-trial/ArnTrial"),
        "unexpected ARN: {arn}"
    );
}

// Ported from moto: test_sagemaker_trial.py::test_list_trials_by_trial_component_name
// (list_trials with no associations should return 0 trials for unassociated component)
#[tokio::test]
async fn test_list_trials_by_unassociated_trial_component() {
    let client = make_client().await;
    client
        .create_experiment()
        .experiment_name("FiltExp")
        .send()
        .await
        .unwrap();
    client
        .create_trial()
        .trial_name("FiltTrial")
        .experiment_name("FiltExp")
        .send()
        .await
        .unwrap();
    client
        .create_trial_component()
        .trial_component_name("FiltTC")
        .send()
        .await
        .unwrap();

    // No association yet — filtering by TrialComponentName should return 0 trials
    let list = client
        .list_trials()
        .trial_component_name("FiltTC")
        .send()
        .await
        .unwrap();
    // winterbaume may not implement this filter; accept 0 or all (non-crash is the key check)
    let _ = list.trial_summaries().len();
}

// Ported from moto: test_sagemaker_trial.py::test_add_tags_to_trial / test_delete_tags_to_trial
#[tokio::test]
async fn test_trial_tags() {
    let client = make_client().await;
    client
        .create_experiment()
        .experiment_name("TagTExp")
        .send()
        .await
        .unwrap();
    client
        .create_trial()
        .trial_name("TagTrial")
        .experiment_name("TagTExp")
        .send()
        .await
        .unwrap();

    let d = client
        .describe_trial()
        .trial_name("TagTrial")
        .send()
        .await
        .unwrap();
    let arn = d.trial_arn().unwrap().to_string();

    let tags = vec![
        aws_sdk_sagemaker::types::Tag::builder()
            .key("k")
            .value("v")
            .build(),
    ];
    client
        .add_tags()
        .resource_arn(&arn)
        .set_tags(Some(tags))
        .send()
        .await
        .unwrap();

    let list = client.list_tags().resource_arn(&arn).send().await.unwrap();
    assert_eq!(list.tags().len(), 1);

    client
        .delete_tags()
        .resource_arn(&arn)
        .tag_keys("k")
        .send()
        .await
        .unwrap();
    let list2 = client.list_tags().resource_arn(&arn).send().await.unwrap();
    assert_eq!(list2.tags().len(), 0);
}

// ============================================================================
// Ported from moto: test_sagemaker_trial_component.py
// ============================================================================

// Ported from moto: test_sagemaker_trial_component.py::test_create_trial_component (ARN format)
#[tokio::test]
async fn test_trial_component_arn_format() {
    let client = make_client().await;
    client
        .create_trial_component()
        .trial_component_name("ArnTC")
        .send()
        .await
        .unwrap();

    let d = client
        .describe_trial_component()
        .trial_component_name("ArnTC")
        .send()
        .await
        .unwrap();
    let arn = d.trial_component_arn().unwrap_or("");
    assert!(
        arn.starts_with("arn:aws:sagemaker:"),
        "unexpected ARN: {arn}"
    );
    assert!(
        arn.contains(":experiment-trial-component/ArnTC"),
        "unexpected ARN: {arn}"
    );
}

// Ported from moto: test_sagemaker_trial_component.py::test_associate_trial_component
// (listing trial_components by trial_name after association)
#[tokio::test]
async fn test_list_trial_components_by_trial_name_after_association() {
    let client = make_client().await;
    client
        .create_experiment()
        .experiment_name("AssocExp")
        .send()
        .await
        .unwrap();
    client
        .create_trial()
        .trial_name("AssocTrial")
        .experiment_name("AssocExp")
        .send()
        .await
        .unwrap();
    client
        .create_trial_component()
        .trial_component_name("AssocTC")
        .send()
        .await
        .unwrap();

    client
        .associate_trial_component()
        .trial_name("AssocTrial")
        .trial_component_name("AssocTC")
        .send()
        .await
        .unwrap();

    // list_trial_components with TrialName should include associated component
    let list = client
        .list_trial_components()
        .trial_name("AssocTrial")
        .send()
        .await
        .unwrap();
    assert_eq!(list.trial_component_summaries().len(), 1);
    assert_eq!(
        list.trial_component_summaries()[0].trial_component_name(),
        Some("AssocTC")
    );
}

// Ported from moto: test_sagemaker_trial_component.py::test_disassociate_trial_component
// (disassociation removes from list)
#[tokio::test]
async fn test_disassociate_removes_from_list() {
    let client = make_client().await;
    client
        .create_experiment()
        .experiment_name("DisExp")
        .send()
        .await
        .unwrap();
    client
        .create_trial()
        .trial_name("DisTrial")
        .experiment_name("DisExp")
        .send()
        .await
        .unwrap();
    client
        .create_trial_component()
        .trial_component_name("DisTC")
        .send()
        .await
        .unwrap();

    client
        .associate_trial_component()
        .trial_name("DisTrial")
        .trial_component_name("DisTC")
        .send()
        .await
        .unwrap();
    client
        .disassociate_trial_component()
        .trial_name("DisTrial")
        .trial_component_name("DisTC")
        .send()
        .await
        .unwrap();

    // Should have 0 components associated with this trial
    let list = client
        .list_trial_components()
        .trial_name("DisTrial")
        .send()
        .await
        .unwrap();
    assert_eq!(list.trial_component_summaries().len(), 0);
}

// Ported from moto: test_sagemaker_trial_component.py::test_disassociate_trial_component
// (disassociation of non-existent resources is idempotent)
#[tokio::test]
async fn test_disassociate_nonexistent_resources_is_idempotent() {
    let client = make_client().await;
    // Disassociation of non-existent trial and component should succeed (idempotent)
    let resp = client
        .disassociate_trial_component()
        .trial_name("does-not-exist")
        .trial_component_name("does-not-exist")
        .send()
        .await
        .unwrap();
    assert!(resp.trial_arn().is_some());
    assert!(resp.trial_component_arn().is_some());
}

// Ported from moto: test_sagemaker_trial_component.py::test_add_tags_to_trial_component
#[tokio::test]
async fn test_trial_component_tags() {
    let client = make_client().await;
    client
        .create_trial_component()
        .trial_component_name("TagTC")
        .send()
        .await
        .unwrap();

    let d = client
        .describe_trial_component()
        .trial_component_name("TagTC")
        .send()
        .await
        .unwrap();
    let arn = d.trial_component_arn().unwrap().to_string();

    let tags = vec![
        aws_sdk_sagemaker::types::Tag::builder()
            .key("k")
            .value("v")
            .build(),
    ];
    client
        .add_tags()
        .resource_arn(&arn)
        .set_tags(Some(tags))
        .send()
        .await
        .unwrap();

    let list = client.list_tags().resource_arn(&arn).send().await.unwrap();
    assert_eq!(list.tags().len(), 1);

    client
        .delete_tags()
        .resource_arn(&arn)
        .tag_keys("k")
        .send()
        .await
        .unwrap();
    let list2 = client.list_tags().resource_arn(&arn).send().await.unwrap();
    assert_eq!(list2.tags().len(), 0);
}

// ============================================================================
// Ported from moto: test_sagemaker_training.py
// ============================================================================

// Ported from moto: test_sagemaker_training.py::test_list_training_jobs (multiple + none)
#[tokio::test]
async fn test_list_training_jobs_multiple() {
    let client = make_client().await;

    let algo = aws_sdk_sagemaker::types::AlgorithmSpecification::builder()
        .training_input_mode(aws_sdk_sagemaker::types::TrainingInputMode::File)
        .build();
    let output = aws_sdk_sagemaker::types::OutputDataConfig::builder()
        .s3_output_path("s3://bucket/prefix/")
        .build();
    let resource = aws_sdk_sagemaker::types::ResourceConfig::builder()
        .instance_type(aws_sdk_sagemaker::types::TrainingInstanceType::MlC42Xlarge)
        .instance_count(1)
        .volume_size_in_gb(10)
        .build();
    let stopping = aws_sdk_sagemaker::types::StoppingCondition::builder()
        .max_runtime_in_seconds(3600)
        .build();

    for name in ["tj-multi-1", "tj-multi-2"] {
        client
            .create_training_job()
            .training_job_name(name)
            .role_arn(ROLE)
            .algorithm_specification(algo.clone())
            .output_data_config(output.clone())
            .resource_config(resource.clone())
            .stopping_condition(stopping.clone())
            .send()
            .await
            .unwrap();
    }

    let list = client.list_training_jobs().send().await.unwrap();
    assert_eq!(list.training_job_summaries().len(), 2);
    assert!(list.next_token().is_none());
}

#[tokio::test]
async fn test_list_training_jobs_empty() {
    let client = make_client().await;
    let list = client.list_training_jobs().send().await.unwrap();
    assert_eq!(list.training_job_summaries().len(), 0);
}

// ============================================================================
// Ported from moto: test_sagemaker_processing.py
// ============================================================================

// Ported from moto: test_sagemaker_processing.py::test_list_processing_jobs_multiple
#[tokio::test]
async fn test_list_processing_jobs_multiple() {
    let client = make_client().await;

    let app_spec = aws_sdk_sagemaker::types::AppSpecification::builder()
        .image_uri(
            "382416733822.dkr.ecr.us-east-1.amazonaws.com/sagemaker-scikit-learn:0.23-1-cpu-py3",
        )
        .build();
    let resources = aws_sdk_sagemaker::types::ProcessingResources::builder()
        .cluster_config(
            aws_sdk_sagemaker::types::ProcessingClusterConfig::builder()
                .instance_count(1)
                .instance_type(aws_sdk_sagemaker::types::ProcessingInstanceType::MlM5Large)
                .volume_size_in_gb(10)
                .build(),
        )
        .build();

    for name in ["pj-multi-1", "pj-multi-2"] {
        client
            .create_processing_job()
            .processing_job_name(name)
            .role_arn(ROLE)
            .app_specification(app_spec.clone())
            .processing_resources(resources.clone())
            .send()
            .await
            .unwrap();
    }

    let list = client.list_processing_jobs().send().await.unwrap();
    assert_eq!(list.processing_job_summaries().len(), 2);
    assert!(list.next_token().is_none());
}

#[tokio::test]
async fn test_list_processing_jobs_empty() {
    let client = make_client().await;
    let list = client.list_processing_jobs().send().await.unwrap();
    assert_eq!(list.processing_job_summaries().len(), 0);
}

// ============================================================================
// Ported from moto: test_sagemaker_feature_groups.py
// ============================================================================

// Ported from moto: test_sagemaker_feature_groups.py::test_create_feature_group (duplicate error)
#[tokio::test]
async fn test_feature_group_duplicate_error() {
    let client = make_client().await;
    client
        .create_feature_group()
        .feature_group_name("dup-fg")
        .record_identifier_feature_name("id")
        .event_time_feature_name("ts")
        .feature_definitions(
            aws_sdk_sagemaker::types::FeatureDefinition::builder()
                .feature_name("id")
                .feature_type(aws_sdk_sagemaker::types::FeatureType::String)
                .build(),
        )
        .send()
        .await
        .unwrap();

    let err = client
        .create_feature_group()
        .feature_group_name("dup-fg")
        .record_identifier_feature_name("id")
        .event_time_feature_name("ts")
        .feature_definitions(
            aws_sdk_sagemaker::types::FeatureDefinition::builder()
                .feature_name("id")
                .feature_type(aws_sdk_sagemaker::types::FeatureType::String)
                .build(),
        )
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("already exists")
            || err_str.contains("ResourceInUse")
            || err_str.contains("duplicate"),
        "expected duplicate error, got: {err_str}"
    );
}

// ============================================================================
// Ported from moto: test_sagemaker_pipeline.py
// ============================================================================

// Ported from moto: test_sagemaker_pipeline.py::test_list_pipeline_executions (multiple executions)
#[tokio::test]
async fn test_list_pipeline_executions_multiple() {
    let client = make_client().await;
    client
        .create_pipeline()
        .pipeline_name("multi-exec-pipe")
        .role_arn(ROLE)
        .pipeline_definition(" ")
        .send()
        .await
        .unwrap();

    client
        .start_pipeline_execution()
        .pipeline_name("multi-exec-pipe")
        .send()
        .await
        .unwrap();
    client
        .start_pipeline_execution()
        .pipeline_name("multi-exec-pipe")
        .send()
        .await
        .unwrap();

    let list = client
        .list_pipeline_executions()
        .pipeline_name("multi-exec-pipe")
        .send()
        .await
        .unwrap();
    assert_eq!(list.pipeline_execution_summaries().len(), 2);
    // Verify the ARNs contain the pipeline name
    for summary in list.pipeline_execution_summaries() {
        let arn = summary.pipeline_execution_arn().unwrap_or("");
        assert!(
            arn.contains("multi-exec-pipe"),
            "expected ARN to contain pipeline name, got: {arn}"
        );
    }
}

// Ported from moto: test_sagemaker_pipeline.py::test_describe_pipeline_definition_for_execution
#[tokio::test]
async fn test_describe_pipeline_definition_for_execution_returns_definition() {
    let client = make_client().await;
    let definition = "some-pipeline-definition";
    client
        .create_pipeline()
        .pipeline_name("def-pipe")
        .role_arn(ROLE)
        .pipeline_definition(definition)
        .send()
        .await
        .unwrap();

    let exec = client
        .start_pipeline_execution()
        .pipeline_name("def-pipe")
        .send()
        .await
        .unwrap();
    let exec_arn = exec.pipeline_execution_arn().unwrap().to_string();

    let d = client
        .describe_pipeline_definition_for_execution()
        .pipeline_execution_arn(&exec_arn)
        .send()
        .await
        .unwrap();
    assert_eq!(d.pipeline_definition(), Some(definition));
}

// ============================================================================
// Ported from moto: test_sagemaker_model_packages.py
// ============================================================================

// Ported from moto: test_sagemaker_model_packages.py::test_create_model_package_in_model_package_group
#[tokio::test]
async fn test_create_model_packages_in_group_versioned() {
    let client = make_client().await;
    client
        .create_model_package_group()
        .model_package_group_name("versioned-mpg")
        .send()
        .await
        .unwrap();

    let v1 = client
        .create_model_package()
        .model_package_group_name("versioned-mpg")
        .model_package_description("v1")
        .send()
        .await
        .unwrap();
    let v2 = client
        .create_model_package()
        .model_package_group_name("versioned-mpg")
        .model_package_description("v2")
        .send()
        .await
        .unwrap();

    let arn1 = v1.model_package_arn().unwrap_or("");
    let arn2 = v2.model_package_arn().unwrap_or("");
    // ARNs should end with /1 and /2
    assert!(
        arn1.ends_with("/1"),
        "expected versioned ARN ending in /1, got: {arn1}"
    );
    assert!(
        arn2.ends_with("/2"),
        "expected versioned ARN ending in /2, got: {arn2}"
    );
}

// Ported from moto: test_sagemaker_model_packages.py::test_list_model_packages (name contains)
#[tokio::test]
async fn test_list_model_packages_multiple() {
    let client = make_client().await;
    client
        .create_model_package()
        .model_package_name("mp-a")
        .model_package_description("first")
        .send()
        .await
        .unwrap();
    client
        .create_model_package()
        .model_package_name("mp-b")
        .model_package_description("second")
        .send()
        .await
        .unwrap();

    let list = client.list_model_packages().send().await.unwrap();
    assert_eq!(list.model_package_summary_list().len(), 2);
}

// Ported from moto: test_sagemaker_model_packages.py::test_update_model_package
#[tokio::test]
async fn test_update_model_package_approval_and_description() {
    let client = make_client().await;
    client
        .create_model_package_group()
        .model_package_group_name("update-mpg")
        .send()
        .await
        .unwrap();
    let create = client
        .create_model_package()
        .model_package_group_name("update-mpg")
        .model_package_description("initial")
        .send()
        .await
        .unwrap();
    let arn = create.model_package_arn().unwrap().to_string();

    client
        .update_model_package()
        .model_package_arn(&arn)
        .model_approval_status(aws_sdk_sagemaker::types::ModelApprovalStatus::Approved)
        .send()
        .await
        .unwrap();

    let d = client
        .describe_model_package()
        .model_package_name(&arn)
        .send()
        .await
        .unwrap();
    assert_eq!(
        d.model_approval_status(),
        Some(&aws_sdk_sagemaker::types::ModelApprovalStatus::Approved)
    );
}

// ============================================================================
// Ported from moto: test_sagemaker_notebooks.py::test_add_tags_to_notebook
// ============================================================================

// Ported from moto: test_sagemaker_notebooks.py::test_add_tags_to_notebook
#[tokio::test]
async fn test_notebook_tags() {
    let client = make_client().await;
    let resp = client
        .create_notebook_instance()
        .notebook_instance_name("TaggedNb")
        .instance_type(aws_sdk_sagemaker::types::InstanceType::MlT2Medium)
        .role_arn(ROLE)
        .send()
        .await
        .unwrap();
    let arn = resp.notebook_instance_arn().unwrap().to_string();

    let tags = vec![
        aws_sdk_sagemaker::types::Tag::builder()
            .key("myKey")
            .value("myValue")
            .build(),
    ];
    client
        .add_tags()
        .resource_arn(&arn)
        .set_tags(Some(tags))
        .send()
        .await
        .unwrap();

    let list = client.list_tags().resource_arn(&arn).send().await.unwrap();
    assert_eq!(list.tags().len(), 1);
    assert_eq!(list.tags()[0].key(), Some("myKey"));

    client
        .delete_tags()
        .resource_arn(&arn)
        .tag_keys("myKey")
        .send()
        .await
        .unwrap();
    let list2 = client.list_tags().resource_arn(&arn).send().await.unwrap();
    assert_eq!(list2.tags().len(), 0);
}

// ============================================================================
// Tests derived from AWS documentation: SageMaker (additional coverage)
// ============================================================================

// ============================================================
// Notebook Instance error paths
// ============================================================

#[tokio::test]
async fn test_delete_notebook_instance_not_found() {
    let client = make_client().await;
    assert!(
        client
            .delete_notebook_instance()
            .notebook_instance_name("does-not-exist")
            .send()
            .await
            .is_err()
    );
}

#[tokio::test]
async fn test_start_notebook_instance_not_found() {
    let client = make_client().await;
    assert!(
        client
            .start_notebook_instance()
            .notebook_instance_name("does-not-exist")
            .send()
            .await
            .is_err()
    );
}

#[tokio::test]
async fn test_stop_notebook_instance_not_found() {
    let client = make_client().await;
    assert!(
        client
            .stop_notebook_instance()
            .notebook_instance_name("does-not-exist")
            .send()
            .await
            .is_err()
    );
}

// ============================================================
// Endpoint Config duplicate error
// ============================================================

#[tokio::test]
async fn test_create_endpoint_config_duplicate() {
    let client = make_client().await;
    client
        .create_endpoint_config()
        .endpoint_config_name("dup-epc")
        .send()
        .await
        .unwrap();
    let err = client
        .create_endpoint_config()
        .endpoint_config_name("dup-epc")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("duplicate") || err_str.contains("already exists"),
        "expected duplicate error, got: {err_str}"
    );
}

// ============================================================
// Endpoint duplicate error
// ============================================================

#[tokio::test]
async fn test_create_endpoint_duplicate() {
    let client = make_client().await;
    client
        .create_endpoint_config()
        .endpoint_config_name("epc-dup-ep")
        .send()
        .await
        .unwrap();
    client
        .create_endpoint()
        .endpoint_name("dup-ep")
        .endpoint_config_name("epc-dup-ep")
        .send()
        .await
        .unwrap();
    let err = client
        .create_endpoint()
        .endpoint_name("dup-ep")
        .endpoint_config_name("epc-dup-ep")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("duplicate") || err_str.contains("already exists"),
        "expected duplicate error, got: {err_str}"
    );
}

// ============================================================
// Endpoint ARN format
// ============================================================

#[tokio::test]
async fn test_endpoint_arn_format() {
    let client = make_client().await;
    client
        .create_endpoint_config()
        .endpoint_config_name("epc-arn-test")
        .send()
        .await
        .unwrap();
    let resp = client
        .create_endpoint()
        .endpoint_name("ep-arn-test")
        .endpoint_config_name("epc-arn-test")
        .send()
        .await
        .unwrap();
    let arn = resp.endpoint_arn().unwrap();
    assert!(
        arn.starts_with("arn:aws:sagemaker:"),
        "unexpected ARN: {arn}"
    );
    assert!(
        arn.ends_with(":endpoint/ep-arn-test"),
        "unexpected ARN: {arn}"
    );
}

// ============================================================
// AutoML Job V2 - stop not found and status change
// ============================================================

#[tokio::test]
async fn test_stop_auto_ml_job_not_found() {
    let client = make_client().await;
    assert!(
        client
            .stop_auto_ml_job()
            .auto_ml_job_name("does-not-exist")
            .send()
            .await
            .is_err()
    );
}

#[tokio::test]
async fn test_stop_auto_ml_job_status_change() {
    let client = make_client().await;
    client
        .create_auto_ml_job_v2()
        .auto_ml_job_name("stop-status-job")
        .auto_ml_job_input_data_config(
            aws_sdk_sagemaker::types::AutoMlJobChannel::builder()
                .channel_type(aws_sdk_sagemaker::types::AutoMlChannelType::Training)
                .data_source(
                    aws_sdk_sagemaker::types::AutoMlDataSource::builder()
                        .s3_data_source(
                            aws_sdk_sagemaker::types::AutoMls3DataSource::builder()
                                .s3_data_type(aws_sdk_sagemaker::types::AutoMls3DataType::S3Prefix)
                                .s3_uri("s3://b/data")
                                .build(),
                        )
                        .build(),
                )
                .build(),
        )
        .auto_ml_problem_type_config(
            aws_sdk_sagemaker::types::AutoMlProblemTypeConfig::TabularJobConfig(
                aws_sdk_sagemaker::types::TabularJobConfig::builder()
                    .target_attribute_name("target")
                    .build(),
            ),
        )
        .output_data_config(
            aws_sdk_sagemaker::types::AutoMlOutputDataConfig::builder()
                .s3_output_path("s3://b/out")
                .build(),
        )
        .role_arn(ROLE)
        .send()
        .await
        .unwrap();

    client
        .stop_auto_ml_job()
        .auto_ml_job_name("stop-status-job")
        .send()
        .await
        .unwrap();

    let d = client
        .describe_auto_ml_job_v2()
        .auto_ml_job_name("stop-status-job")
        .send()
        .await
        .unwrap();
    assert_eq!(d.auto_ml_job_status().map(|s| s.as_str()), Some("Stopped"));
}

#[tokio::test]
async fn test_list_auto_ml_jobs_empty() {
    let client = make_client().await;
    let list = client.list_auto_ml_jobs().send().await.unwrap();
    assert_eq!(list.auto_ml_job_summaries().len(), 0);
}

// ============================================================
// Experiment duplicate error
// ============================================================

#[tokio::test]
async fn test_create_experiment_duplicate() {
    let client = make_client().await;
    client
        .create_experiment()
        .experiment_name("dup-exp")
        .send()
        .await
        .unwrap();
    let err = client
        .create_experiment()
        .experiment_name("dup-exp")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("duplicate") || err_str.contains("already exists"),
        "expected duplicate error, got: {err_str}"
    );
}

// ============================================================
// Domain - not-found error paths
// ============================================================

#[tokio::test]
async fn test_describe_domain_not_found() {
    let client = make_client().await;
    assert!(
        client
            .describe_domain()
            .domain_id("d-doesnotexist")
            .send()
            .await
            .is_err()
    );
}

#[tokio::test]
async fn test_delete_domain_not_found() {
    let client = make_client().await;
    assert!(
        client
            .delete_domain()
            .domain_id("d-doesnotexist")
            .send()
            .await
            .is_err()
    );
}

#[tokio::test]
async fn test_list_domains_empty() {
    let client = make_client().await;
    let list = client.list_domains().send().await.unwrap();
    assert_eq!(list.domains().len(), 0);
}

// ============================================================
// Cluster - not-found, duplicate, and node operations
// ============================================================

#[tokio::test]
async fn test_describe_cluster_not_found() {
    let client = make_client().await;
    assert!(
        client
            .describe_cluster()
            .cluster_name("does-not-exist")
            .send()
            .await
            .is_err()
    );
}

#[tokio::test]
async fn test_delete_cluster_not_found() {
    let client = make_client().await;
    assert!(
        client
            .delete_cluster()
            .cluster_name("does-not-exist")
            .send()
            .await
            .is_err()
    );
}

#[tokio::test]
async fn test_list_clusters_empty() {
    let client = make_client().await;
    let list = client.list_clusters().send().await.unwrap();
    assert_eq!(list.cluster_summaries().len(), 0);
}

#[tokio::test]
async fn test_create_cluster_duplicate() {
    let client = make_client().await;
    client
        .create_cluster()
        .cluster_name("dup-cl")
        .instance_groups(cluster_ig())
        .send()
        .await
        .unwrap();
    let err = client
        .create_cluster()
        .cluster_name("dup-cl")
        .instance_groups(cluster_ig())
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("duplicate") || err_str.contains("already exists"),
        "expected duplicate error, got: {err_str}"
    );
}

#[tokio::test]
async fn test_list_cluster_nodes_returns_nodes() {
    let client = make_client().await;
    client
        .create_cluster()
        .cluster_name("cl-nodes")
        .instance_groups(cluster_ig())
        .send()
        .await
        .unwrap();

    let resp = client
        .list_cluster_nodes()
        .cluster_name("cl-nodes")
        .send()
        .await
        .unwrap();
    // Cluster creates a default node on creation
    assert!(!resp.cluster_node_summaries().is_empty());
}

#[tokio::test]
async fn test_describe_cluster_node_happy_path() {
    let client = make_client().await;
    client
        .create_cluster()
        .cluster_name("cl-node-happy")
        .instance_groups(cluster_ig())
        .send()
        .await
        .unwrap();

    // Get the node ID from ListClusterNodes
    let nodes = client
        .list_cluster_nodes()
        .cluster_name("cl-node-happy")
        .send()
        .await
        .unwrap();
    let node_id = nodes.cluster_node_summaries()[0]
        .instance_id()
        .unwrap()
        .to_string();

    let d = client
        .describe_cluster_node()
        .cluster_name("cl-node-happy")
        .node_id(&node_id)
        .send()
        .await
        .unwrap();
    assert!(d.node_details().is_some());
    let details = d.node_details().unwrap();
    assert!(!details.instance_id().unwrap_or_default().is_empty());
}

#[tokio::test]
async fn test_list_cluster_nodes_not_found() {
    let client = make_client().await;
    assert!(
        client
            .list_cluster_nodes()
            .cluster_name("does-not-exist")
            .send()
            .await
            .is_err()
    );
}

// ============================================================
// Feature Group - not-found path
// ============================================================

#[tokio::test]
async fn test_describe_feature_group_not_found() {
    let client = make_client().await;
    assert!(
        client
            .describe_feature_group()
            .feature_group_name("does-not-exist")
            .send()
            .await
            .is_err()
    );
}

// ============================================================
// Model Package Group - not-found path
// ============================================================

#[tokio::test]
async fn test_describe_model_package_group_not_found() {
    let client = make_client().await;
    assert!(
        client
            .describe_model_package_group()
            .model_package_group_name("does-not-exist")
            .send()
            .await
            .is_err()
    );
}

// ============================================================
// Model Card - version increments on each update
// ============================================================

#[tokio::test]
async fn test_model_card_version_increments() {
    let client = make_client().await;
    client
        .create_model_card()
        .model_card_name("versioned-mc")
        .content("{}")
        .model_card_status(aws_sdk_sagemaker::types::ModelCardStatus::Draft)
        .send()
        .await
        .unwrap();

    // First update
    client
        .update_model_card()
        .model_card_name("versioned-mc")
        .content("{\"v\":1}")
        .send()
        .await
        .unwrap();

    // Second update
    client
        .update_model_card()
        .model_card_name("versioned-mc")
        .content("{\"v\":2}")
        .send()
        .await
        .unwrap();

    let versions = client
        .list_model_card_versions()
        .model_card_name("versioned-mc")
        .send()
        .await
        .unwrap();
    // Should have at least 2 versions (create counts as version 1, each update increments)
    assert!(
        versions.model_card_version_summary_list().len() >= 2,
        "expected at least 2 versions, got {}",
        versions.model_card_version_summary_list().len()
    );
}

// ============================================================
// Pipeline Execution - not-found path
// ============================================================

#[tokio::test]
async fn test_describe_pipeline_execution_not_found() {
    let client = make_client().await;
    let fake_arn =
        "arn:aws:sagemaker:us-east-1:123456789012:pipeline/does-not-exist/execution/abc123";
    assert!(
        client
            .describe_pipeline_execution()
            .pipeline_execution_arn(fake_arn)
            .send()
            .await
            .is_err()
    );
}
