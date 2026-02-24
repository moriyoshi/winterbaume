# Terraform Converter Coverage

Winterbaume includes a Terraform state converter layer (`winterbaume-terraform`) that can inject Terraform state into the emulator and extract it back out. This enables:

- **Seeding mock environments** from existing Terraform state files
- **Round-trip validation** — inject state, exercise the mock, then extract and compare
- **Test data generation** — programmatically build Terraform state from converter output

## Overview

- **267 Terraform resource types** across **137 service modules**
- **67.4% inject coverage** (reading TF state attributes into winterbaume)
- **59.7% extract coverage** (emitting winterbaume state back to TF attributes)
- **265 excellent-rated converters** out of 267 total

Coverage is measured against the official Terraform AWS provider schema (~5.x). "Inject" means reading attributes from a `terraform.tfstate` JSON file into winterbaume's in-memory state. "Extract" means producing Terraform-compatible attribute JSON from winterbaume's state.

### Rating criteria

| Rating | Inject threshold | Extract threshold |
|--------|-----------------|-------------------|
| Excellent | >= 60% | >= 50% |
| Good | >= 40% | >= 30% |
| Fair | >= 20% | >= 15% |
| Poor | < 20% | < 15% |

## Per-Resource Coverage

| Resource Type | Inject | Extract | TF Schema Attrs | Inject % | Extract % | Rating |
|---|---|---|---|---|---|---|
| `aws_account_alternate_contact` | 6 | 6 | 7 | 71% | 71% | excellent |
| `aws_acm_certificate` | 17 | 17 | 23 | 61% | 52% | excellent |
| `aws_acmpca_certificate_authority` | 14 | 25 | 17 | 65% | 65% | excellent |
| `aws_prometheus_workspace` | 8 | 6 | 7 | 71% | 57% | excellent |
| `aws_amplify_app` | 19 | 18 | 25 | 60% | 52% | excellent |
| `aws_amplify_branch` | 15 | 15 | 23 | 61% | 52% | excellent |
| `aws_api_gateway_api_key` | 9 | 8 | 10 | 60% | 60% | excellent |
| `aws_api_gateway_deployment` | 8 | 6 | 10 | 60% | 50% | excellent |
| `aws_api_gateway_method` | 11 | 11 | 11 | 91% | 91% | excellent |
| `aws_api_gateway_resource` | 6 | 5 | 4 | 100% | 100% | excellent |
| `aws_api_gateway_rest_api` | 14 | 15 | 18 | 61% | 61% | excellent |
| `aws_api_gateway_stage` | 12 | 13 | 18 | 61% | 56% | excellent |
| `aws_apigatewayv2_api` | 15 | 13 | 19 | 63% | 53% | excellent |
| `aws_appconfig_application` | 5 | 4 | 5 | 60% | 60% | excellent |
| `aws_appconfig_configuration_profile` | 10 | 8 | 12 | 75% | 58% | excellent |
| `aws_appconfig_deployment_strategy` | 9 | 8 | 10 | 70% | 70% | excellent |
| `aws_appconfig_environment` | 8 | 9 | 9 | 78% | 67% | excellent |
| `aws_appautoscaling_policy` | 9 | 7 | 9 | 89% | 67% | excellent |
| `aws_appautoscaling_target` | 9 | 12 | 10 | 80% | 80% | excellent |
| `aws_appmesh_mesh` | 12 | 11 | 9 | 100% | 89% | excellent |
| `aws_appmesh_virtual_node` | 13 | 10 | 10 | 100% | 90% | excellent |
| `aws_appmesh_virtual_service` | 13 | 10 | 10 | 100% | 90% | excellent |
| `aws_apprunner_service` | 11 | 8 | 14 | 71% | 50% | excellent |
| `aws_appsync_graphql_api` | 14 | 11 | 20 | 60% | 50% | excellent |
| `aws_athena_data_catalog` | 6 | 6 | 7 | 71% | 71% | excellent |
| `aws_athena_workgroup` | 6 | 6 | 8 | 62% | 62% | excellent |
| `aws_auditmanager_control` | 8 | 6 | 10 | 60% | 50% | excellent |
| `aws_auditmanager_framework` | 7 | 5 | 8 | 62% | 50% | excellent |
| `aws_autoscaling_group` | 30 | 28 | 45 | 60% | 51% | excellent |
| `aws_autoscaling_policy` | 10 | 8 | 14 | 64% | 50% | excellent |
| `aws_autoscaling_schedule` | 11 | 10 | 10 | 100% | 90% | excellent |
| `aws_launch_configuration` | 14 | 13 | 19 | 68% | 58% | excellent |
| `aws_backup_plan` | 7 | 5 | 7 | 71% | 57% | excellent |
| `aws_backup_vault` | 6 | 5 | 8 | 62% | 50% | excellent |
| `aws_batch_compute_environment` | 10 | 10 | 14 | 64% | 64% | excellent |
| `aws_batch_job_definition` | 12 | 16 | 18 | 61% | 50% | excellent |
| `aws_batch_job_queue` | 10 | 7 | 11 | 82% | 55% | excellent |
| `aws_batch_scheduling_policy` | 5 | 4 | 5 | 80% | 60% | excellent |
| `aws_bedrock_guardrail` | 14 | 13 | 18 | 67% | 61% | excellent |
| `aws_bedrock_model_invocation_logging_configuration` | 2 | 11 | 1 | 100% | 100% | excellent |
| `aws_bedrockagent_agent` | 22 | 20 | 20 | 70% | 65% | excellent |
| `aws_bedrockagent_knowledge_base` | 13 | 12 | 12 | 67% | 67% | excellent |
| `aws_budgets_budget` | 12 | 14 | 17 | 65% | 53% | excellent |
| `aws_chatbot_slack_channel_configuration` | 10 | 8 | 14 | 64% | 57% | excellent |
| `aws_cloudformation_stack` | 12 | 11 | 16 | 62% | 56% | excellent |
| `aws_cloudfront_distribution` | 22 | 27 | 32 | 62% | 50% | excellent |
| `aws_cloudhsm_v2_cluster` | 10 | 20 | 12 | 67% | 58% | excellent |
| `aws_cloudtrail` | 13 | 11 | 20 | 60% | 50% | excellent |
| `aws_cloudwatch_metric_alarm` | 17 | 17 | 24 | 67% | 67% | excellent |
| `aws_codeartifact_domain` | 7 | 7 | 10 | 60% | 60% | excellent |
| `aws_codeartifact_repository` | 7 | 7 | 10 | 60% | 60% | excellent |
| `aws_codebuild_project` | 18 | 22 | 27 | 63% | 56% | excellent |
| `aws_codecommit_repository` | 7 | 7 | 10 | 60% | 60% | excellent |
| `aws_codedeploy_app` | 6 | 5 | 8 | 62% | 50% | excellent |
| `aws_codedeploy_deployment_group` | 15 | 16 | 22 | 64% | 50% | excellent |
| `aws_codepipeline` | 9 | 8 | 12 | 67% | 58% | excellent |
| `aws_cognito_identity_pool` | 11 | 13 | 11 | 82% | 82% | excellent |
| `aws_cognito_user_pool` | 25 | 22 | 36 | 61% | 53% | excellent |
| `aws_cognito_user_pool_client` | 17 | 14 | 24 | 62% | 50% | excellent |
| `aws_comprehend_entity_recognizer` | 11 | 14 | 13 | 69% | 54% | excellent |
| `aws_config_config_rule` | 8 | 10 | 11 | 64% | 64% | excellent |
| `aws_config_configuration_recorder` | 4 | 3 | 4 | 75% | 50% | excellent |
| `aws_config_delivery_channel` | 5 | 4 | 6 | 67% | 50% | excellent |
| `aws_connect_instance` | 13 | 11 | 17 | 65% | 59% | excellent |
| `aws_ce_anomaly_monitor` | 10 | 8 | 7 | 86% | 57% | excellent |
| `aws_ce_anomaly_subscription` | 8 | 11 | 9 | 67% | 67% | excellent |
| `aws_datapipeline_pipeline` | 6 | 4 | 4 | 75% | 75% | excellent |
| `aws_datasync_location_s3` | 8 | 7 | 9 | 67% | 56% | excellent |
| `aws_datasync_task` | 12 | 10 | 14 | 64% | 50% | excellent |
| `aws_dax_cluster` | 14 | 14 | 21 | 62% | 52% | excellent |
| `aws_dax_parameter_group` | 3 | 3 | 3 | 67% | 67% | excellent |
| `aws_dax_subnet_group` | 5 | 5 | 4 | 100% | 100% | excellent |
| `aws_dx_connection` | 15 | 12 | 18 | 61% | 50% | excellent |
| `aws_directory_service_directory` | 17 | 23 | 18 | 61% | 67% | excellent |
| `aws_dms_endpoint` | 19 | 16 | 28 | 61% | 50% | excellent |
| `aws_dms_replication_instance` | 16 | 14 | 21 | 62% | 52% | excellent |
| `aws_dms_replication_task` | 14 | 13 | 15 | 73% | 67% | excellent |
| `aws_dsql_cluster` | 11 | 8 | 10 | 60% | 50% | excellent |
| `aws_dynamodb_table` | 20 | 19 | 29 | 62% | 52% | excellent |
| `aws_ebs_snapshot` | 15 | 12 | 16 | 62% | 50% | excellent |
| `aws_ebs_volume` | 13 | 13 | 16 | 62% | 62% | excellent |
| `aws_eip` | 16 | 13 | 23 | 61% | 52% | excellent |
| `aws_internet_gateway` | 6 | 6 | 6 | 67% | 83% | excellent |
| `aws_key_pair` | 9 | 7 | 9 | 78% | 67% | excellent |
| `aws_nat_gateway` | 10 | 11 | 13 | 62% | 54% | excellent |
| `aws_route` | 14 | 11 | 19 | 63% | 53% | excellent |
| `aws_route_table` | 7 | 17 | 8 | 62% | 62% | excellent |
| `aws_security_group` | 10 | 9 | 12 | 67% | 67% | excellent |
| `aws_subnet` | 17 | 17 | 22 | 64% | 55% | excellent |
| `aws_vpc` | 18 | 17 | 22 | 64% | 55% | excellent |
| `aws_ec2_instance_connect_endpoint` | 15 | 14 | 13 | 77% | 77% | excellent |
| `aws_ecr_repository` | 9 | 7 | 11 | 64% | 55% | excellent |
| `aws_ecs_cluster` | 7 | 6 | 7 | 71% | 57% | excellent |
| `aws_ecs_service` | 25 | 27 | 34 | 62% | 50% | excellent |
| `aws_ecs_task_definition` | 16 | 14 | 24 | 62% | 50% | excellent |
| `aws_efs_file_system` | 13 | 14 | 18 | 61% | 50% | excellent |
| `aws_eks_cluster` | 18 | 19 | 27 | 63% | 52% | excellent |
| `aws_eks_node_group` | 16 | 17 | 25 | 60% | 52% | excellent |
| `aws_elasticache_cluster` | 24 | 21 | 36 | 61% | 53% | excellent |
| `aws_elasticache_parameter_group` | 8 | 6 | 7 | 86% | 71% | excellent |
| `aws_elasticache_replication_group` | 32 | 27 | 48 | 60% | 50% | excellent |
| `aws_elasticache_subnet_group` | 8 | 6 | 7 | 86% | 71% | excellent |
| `aws_elastic_beanstalk_application` | 8 | 7 | 6 | 83% | 67% | excellent |
| `aws_elastic_beanstalk_environment` | 20 | 22 | 24 | 67% | 58% | excellent |
| `aws_elb` | 15 | 27 | 23 | 61% | 70% | excellent |
| `aws_lb` | 24 | 28 | 36 | 61% | 53% | excellent |
| `aws_lb_listener` | 22 | 20 | 32 | 66% | 50% | excellent |
| `aws_lb_target_group` | 18 | 24 | 27 | 63% | 52% | excellent |
| `aws_emr_cluster` | 28 | 25 | 35 | 60% | 51% | excellent |
| `aws_emr_security_configuration` | 5 | 4 | 4 | 75% | 75% | excellent |
| `aws_emrcontainers_virtual_cluster` | 7 | 8 | 6 | 83% | 67% | excellent |
| `aws_emrserverless_application` | 13 | 18 | 14 | 79% | 50% | excellent |
| `aws_cloudwatch_event_bus` | 8 | 6 | 8 | 62% | 50% | excellent |
| `aws_cloudwatch_event_rule` | 11 | 9 | 13 | 69% | 62% | excellent |
| `aws_cloudwatch_event_target` | 14 | 11 | 20 | 60% | 50% | excellent |
| `aws_kinesis_firehose_delivery_stream` | 13 | 12 | 20 | 60% | 50% | excellent |
| `aws_fsx_lustre_file_system` | 28 | 24 | 36 | 61% | 50% | excellent |
| `aws_fsx_windows_file_system` | 23 | 23 | 30 | 60% | 60% | excellent |
| `aws_glacier_vault` | 7 | 7 | 7 | 86% | 57% | excellent |
| `aws_glue_catalog_database` | 8 | 7 | 11 | 64% | 55% | excellent |
| `aws_glue_crawler` | 16 | 22 | 24 | 62% | 54% | excellent |
| `aws_glue_job` | 15 | 13 | 23 | 61% | 52% | excellent |
| `aws_guardduty_detector` | 8 | 7 | 7 | 71% | 57% | excellent |
| `aws_guardduty_filter` | 8 | 7 | 9 | 78% | 67% | excellent |
| `aws_guardduty_member` | 6 | 4 | 8 | 62% | 50% | excellent |
| `aws_iam_group` | 4 | 5 | 4 | 100% | 100% | excellent |
| `aws_iam_instance_profile` | 7 | 6 | 9 | 78% | 56% | excellent |
| `aws_iam_policy` | 9 | 7 | 10 | 80% | 60% | excellent |
| `aws_iam_role` | 10 | 10 | 15 | 67% | 60% | excellent |
| `aws_iam_role_policy_attachment` | 2 | 3 | 2 | 100% | 100% | excellent |
| `aws_iam_user` | 6 | 5 | 8 | 75% | 50% | excellent |
| `aws_iam_user_policy_attachment` | 2 | 3 | 2 | 100% | 100% | excellent |
| `aws_identitystore_group` | 5 | 6 | 5 | 80% | 100% | excellent |
| `aws_identitystore_user` | 14 | 21 | 16 | 62% | 94% | excellent |
| `aws_inspector2_enabler` | 3 | 3 | 3 | 67% | 67% | excellent |
| `aws_iot_certificate` | 7 | 5 | 8 | 62% | 50% | excellent |
| `aws_iot_policy` | 7 | 5 | 7 | 86% | 57% | excellent |
| `aws_iot_thing` | 7 | 7 | 6 | 100% | 100% | excellent |
| `aws_iot_thing_type` | 6 | 7 | 6 | 67% | 67% | excellent |
| `aws_ivs_channel` | 8 | 7 | 11 | 64% | 55% | excellent |
| `aws_msk_cluster` | 23 | 22 | 29 | 62% | 52% | excellent |
| `aws_kinesis_stream` | 11 | 8 | 12 | 75% | 58% | excellent |
| `aws_kinesisanalyticsv2_application` | 12 | 13 | 17 | 65% | 71% | excellent |
| `aws_kinesis_video_stream` | 12 | 11 | 11 | 91% | 82% | excellent |
| `aws_kms_alias` | 4 | 5 | 5 | 60% | 80% | excellent |
| `aws_kms_key` | 15 | 11 | 17 | 71% | 53% | excellent |
| `aws_lakeformation_data_lake_settings` | 8 | 10 | 11 | 64% | 55% | excellent |
| `aws_lakeformation_resource` | 5 | 4 | 6 | 67% | 50% | excellent |
| `aws_lambda_alias` | 6 | 6 | 7 | 71% | 71% | excellent |
| `aws_lambda_event_source_mapping` | 24 | 19 | 35 | 63% | 51% | excellent |
| `aws_lambda_function` | 28 | 27 | 45 | 60% | 51% | excellent |
| `aws_lambda_permission` | 8 | 7 | 11 | 64% | 55% | excellent |
| `aws_lexv2models_bot` | 14 | 13 | 12 | 92% | 92% | excellent |
| `aws_cloudwatch_log_group` | 8 | 6 | 9 | 67% | 56% | excellent |
| `aws_cloudwatch_log_stream` | 4 | 4 | 3 | 100% | 100% | excellent |
| `aws_macie2_account` | 6 | 6 | 5 | 100% | 100% | excellent |
| `aws_macie2_classification_job` | 21 | 19 | 17 | 82% | 76% | excellent |
| `aws_medialive_channel` | 15 | 14 | 17 | 71% | 65% | excellent |
| `aws_medialive_input` | 17 | 16 | 17 | 76% | 71% | excellent |
| `aws_media_package_channel` | 7 | 6 | 6 | 83% | 67% | excellent |
| `aws_media_packagev2_channel_group` | 10 | 9 | 6 | 100% | 83% | excellent |
| `aws_media_store_container` | 5 | 5 | 5 | 80% | 80% | excellent |
| `aws_memorydb_acl` | 7 | 7 | 7 | 71% | 71% | excellent |
| `aws_memorydb_cluster` | 22 | 21 | 32 | 66% | 56% | excellent |
| `aws_memorydb_subnet_group` | 6 | 6 | 8 | 62% | 62% | excellent |
| `aws_mq_broker` | 18 | 20 | 26 | 62% | 50% | excellent |
| `aws_mq_configuration` | 12 | 11 | 10 | 100% | 100% | excellent |
| `aws_neptune_cluster` | 27 | 33 | 38 | 61% | 61% | excellent |
| `aws_neptune_cluster_instance` | 19 | 21 | 28 | 61% | 61% | excellent |
| `aws_neptune_parameter_group` | 7 | 7 | 8 | 75% | 75% | excellent |
| `aws_neptune_subnet_group` | 8 | 9 | 7 | 86% | 86% | excellent |
| `aws_networkfirewall_firewall` | 12 | 13 | 16 | 62% | 69% | excellent |
| `aws_networkfirewall_firewall_policy` | 7 | 7 | 8 | 62% | 75% | excellent |
| `aws_networkfirewall_rule_group` | 9 | 9 | 11 | 64% | 73% | excellent |
| `aws_networkmanager_device` | 14 | 12 | 13 | 77% | 69% | excellent |
| `aws_networkmanager_global_network` | 8 | 6 | 5 | 80% | 60% | excellent |
| `aws_networkmanager_site` | 9 | 7 | 7 | 71% | 57% | excellent |
| `aws_opensearch_domain` | 19 | 24 | 29 | 62% | 52% | excellent |
| `aws_opensearchserverless_collection` | 9 | 7 | 11 | 64% | 55% | excellent |
| `aws_opensearchserverless_security_policy` | 6 | 5 | 5 | 100% | 100% | excellent |
| `aws_organizations_account` | 11 | 9 | 15 | 60% | 53% | excellent |
| `aws_organizations_organizational_unit` | 6 | 6 | 6 | 67% | 83% | excellent |
| `aws_organizations_policy` | 7 | 6 | 8 | 62% | 62% | excellent |
| `aws_osis_pipeline` | 9 | 8 | 13 | 62% | 54% | excellent |
| `aws_pinpoint_app` | 10 | 7 | 9 | 78% | 56% | excellent |
| `aws_pinpoint_email_channel` | 8 | 8 | 8 | 88% | 88% | excellent |
| `aws_pipes_pipe` | 15 | 14 | 17 | 65% | 59% | excellent |
| `aws_quicksight_data_source` | 12 | 10 | 12 | 67% | 50% | excellent |
| `aws_quicksight_group` | 6 | 6 | 5 | 80% | 80% | excellent |
| `aws_quicksight_user` | 8 | 9 | 10 | 60% | 60% | excellent |
| `aws_ram_resource_share` | 6 | 9 | 7 | 71% | 57% | excellent |
| `aws_db_event_subscription` | 9 | 8 | 12 | 67% | 58% | excellent |
| `aws_db_instance` | 53 | 45 | 84 | 61% | 51% | excellent |
| `aws_db_option_group` | 9 | 8 | 11 | 64% | 64% | excellent |
| `aws_db_parameter_group` | 7 | 6 | 9 | 67% | 56% | excellent |
| `aws_db_subnet_group` | 7 | 7 | 9 | 67% | 67% | excellent |
| `aws_rds_cluster` | 48 | 45 | 74 | 62% | 50% | excellent |
| `aws_rds_cluster_parameter_group` | 6 | 6 | 8 | 62% | 62% | excellent |
| `aws_redshift_cluster` | 36 | 30 | 52 | 62% | 50% | excellent |
| `aws_redshift_subnet_group` | 7 | 6 | 6 | 83% | 67% | excellent |
| `aws_rekognition_collection` | 6 | 8 | 6 | 83% | 67% | excellent |
| `aws_resiliencehub_resiliency_policy` | 10 | 12 | 10 | 60% | 50% | excellent |
| `aws_resourcegroups_group` | 8 | 10 | 8 | 88% | 62% | excellent |
| `aws_route53_record` | 12 | 10 | 18 | 61% | 50% | excellent |
| `aws_route53_zone` | 10 | 7 | 12 | 67% | 50% | excellent |
| `aws_route53domains_registered_domain` | 18 | 15 | 25 | 60% | 52% | excellent |
| `aws_route53_resolver_endpoint` | 12 | 16 | 11 | 91% | 82% | excellent |
| `aws_route53_resolver_rule` | 10 | 15 | 11 | 73% | 82% | excellent |
| `aws_s3_bucket` | 22 | 21 | 27 | 63% | 70% | excellent |
| `aws_s3control_bucket` | 8 | 8 | 7 | 86% | 86% | excellent |
| `aws_s3tables_namespace` | 9 | 8 | 5 | 100% | 100% | excellent |
| `aws_s3tables_table_bucket` | 14 | 13 | 6 | 67% | 67% | excellent |
| `aws_sagemaker_domain` | 18 | 17 | 20 | 60% | 60% | excellent |
| `aws_sagemaker_endpoint` | 9 | 8 | 6 | 83% | 67% | excellent |
| `aws_sagemaker_endpoint_configuration` | 8 | 7 | 10 | 60% | 50% | excellent |
| `aws_sagemaker_model` | 8 | 7 | 10 | 60% | 50% | excellent |
| `aws_sagemaker_notebook_instance` | 17 | 14 | 20 | 60% | 50% | excellent |
| `aws_scheduler_schedule` | 10 | 12 | 13 | 62% | 54% | excellent |
| `aws_scheduler_schedule_group` | 7 | 6 | 9 | 67% | 56% | excellent |
| `aws_secretsmanager_secret` | 9 | 7 | 11 | 64% | 55% | excellent |
| `aws_secretsmanager_secret_version` | 8 | 6 | 9 | 67% | 56% | excellent |
| `aws_securityhub_account` | 6 | 6 | 4 | 75% | 100% | excellent |
| `aws_securityhub_standards_subscription` | 5 | 5 | 2 | 100% | 50% | excellent |
| `aws_servicecatalog_portfolio` | 9 | 7 | 8 | 88% | 75% | excellent |
| `aws_servicecatalog_product` | 14 | 12 | 17 | 71% | 65% | excellent |
| `aws_servicecatalogappregistry_application` | 7 | 7 | 6 | 83% | 67% | excellent |
| `aws_service_discovery_private_dns_namespace` | 10 | 8 | 7 | 86% | 71% | excellent |
| `aws_service_discovery_service` | 12 | 16 | 11 | 91% | 82% | excellent |
| `aws_servicequotas_service_quota` | 11 | 11 | 11 | 64% | 64% | excellent |
| `aws_sesv2_configuration_set` | 9 | 9 | 10 | 80% | 80% | excellent |
| `aws_sesv2_dedicated_ip_pool` | 4 | 4 | 5 | 60% | 60% | excellent |
| `aws_sesv2_email_identity` | 6 | 10 | 8 | 62% | 62% | excellent |
| `aws_ses_domain_identity` | 3 | 4 | 3 | 67% | 100% | excellent |
| `aws_ses_email_identity` | 3 | 2 | 2 | 100% | 50% | excellent |
| `aws_shield_protection` | 7 | 7 | 5 | 100% | 80% | excellent |
| `aws_signer_signing_profile` | 10 | 9 | 13 | 62% | 54% | excellent |
| `aws_simpledb_domain` | 2 | 2 | 1 | 100% | 100% | excellent |
| `aws_sns_topic` | 22 | 26 | 32 | 62% | 78% | excellent |
| `aws_sns_topic_subscription` | 13 | 12 | 16 | 62% | 50% | excellent |
| `aws_sqs_queue` | 16 | 13 | 22 | 64% | 55% | excellent |
| `aws_ssm_parameter` | 12 | 11 | 17 | 65% | 53% | excellent |
| `aws_ssoadmin_account_assignment` | 6 | 6 | 7 | 71% | 71% | excellent |
| `aws_ssoadmin_permission_set` | 9 | 7 | 10 | 70% | 60% | excellent |
| `aws_sfn_state_machine` | 14 | 11 | 19 | 63% | 53% | excellent |
| `aws_swf_domain` | 7 | 5 | 7 | 86% | 57% | excellent |
| `aws_synthetics_canary` | 24 | 20 | 24 | 62% | 50% | excellent |
| `aws_timestreaminfluxdb_db_instance` | 17 | 17 | 24 | 62% | 67% | excellent |
| `aws_timestreamquery_scheduled_query` | 19 | 17 | 18 | 61% | 50% | excellent |
| `aws_timestreamwrite_database` | 7 | 6 | 6 | 100% | 83% | excellent |
| `aws_timestreamwrite_table` | 8 | 10 | 8 | 88% | 75% | excellent |
| `aws_transcribe_language_model` | 9 | 11 | 8 | 75% | 88% | excellent |
| `aws_transcribe_vocabulary` | 10 | 9 | 9 | 78% | 56% | excellent |
| `aws_transfer_server` | 18 | 14 | 26 | 62% | 50% | excellent |
| `aws_transfer_user` | 9 | 9 | 12 | 67% | 67% | excellent |
| `aws_vpclattice_listener` | 11 | 17 | 13 | 69% | 77% | excellent |
| `aws_vpclattice_service` | 8 | 8 | 10 | 60% | 50% | excellent |
| `aws_vpclattice_service_network` | 7 | 9 | 5 | 100% | 80% | excellent |
| `aws_vpclattice_target_group` | 9 | 15 | 8 | 88% | 75% | excellent |
| `aws_wafv2_ip_set` | 9 | 7 | 10 | 70% | 60% | excellent |
| `aws_wafv2_rule_group` | 10 | 7 | 12 | 67% | 50% | excellent |
| `aws_wafv2_web_acl` | 14 | 11 | 20 | 60% | 50% | excellent |
| `aws_workspaces_directory` | 16 | 14 | 23 | 61% | 52% | excellent |
| `aws_workspaces_workspace` | 12 | 17 | 13 | 69% | 77% | excellent |
| `aws_xray_group` | 5 | 6 | 6 | 67% | 50% | excellent |
| `aws_xray_sampling_rule` | 15 | 15 | 15 | 80% | 80% | excellent |
| `aws_chatbot_microsoft_teams_channel_configuration` | 12 | 10 | 0 | 0% | 0% | n/a |
| `aws_s3control_access_point` | 14 | 15 | 0 | 0% | 0% | n/a |

## Usage

### Injecting Terraform state

```rust
use winterbaume_terraform::{TerraformInjector, ConversionContext};
use winterbaume_terraform::converters::s3::AwsS3BucketConverter;
use winterbaume_s3::S3Service;
use std::sync::Arc;

let s3 = Arc::new(S3Service::new());
let mut injector = TerraformInjector::new();
injector.register(AwsS3BucketConverter::new(Arc::clone(&s3)));

let ctx = ConversionContext {
    default_account_id: "123456789012".to_string(),
    default_region: "us-east-1".to_string(),
};

// Parse a terraform.tfstate file
let tfstate: winterbaume_tfstate::TerraformState = serde_json::from_str(&state_json)?;
let report = injector.inject_all(&tfstate, &ctx).await;
assert!(report.is_success());
```

### Extracting state back to Terraform format

```rust
let converter = AwsS3BucketConverter::new(Arc::clone(&s3));
let extracted = converter.extract(&ctx).await?;
for resource in &extracted {
    println!("{}: {}", resource.name, resource.attributes);
}
```

## Regenerating This Report

```bash
python3 .agents/skills/api-coverage/scripts/generate_terraform_converter_coverage.py \
  --schema-cache .agents-workspace/tmp/tf-schema/aws_provider_schema.json
```
