# Terraform Converter Coverage

Winterbaume includes a Terraform state converter layer (`winterbaume-terraform`) that can inject Terraform state into the emulator and extract it back out. This enables:

- **Seeding mock environments** from existing Terraform state files
- **Round-trip validation** — inject state, exercise the mock, then extract and compare
- **Test data generation** — programmatically build Terraform state from converter output

## Overview

- **1143 converters** covering **1140 distinct Terraform resource types**
- **56.1% overall inject coverage** (reading TF state attributes into winterbaume)
- **49.0% overall extract coverage** (emitting winterbaume state back to TF attributes)
- Rating distribution: 499 excellent, 490 good, 45 fair, 96 poor, 13 n/a

Coverage is measured against the official Terraform AWS provider schema (~5.x). "Inject" means reading attributes from a `terraform.tfstate` JSON file into winterbaume's in-memory state. "Extract" means producing Terraform-compatible attribute JSON from winterbaume's state.

### Rating criteria

| Rating | Threshold |
|--------|-----------|
| Excellent | inject >= 60% AND extract >= 50% |
| Good | inject >= 40% OR extract >= 30% |
| Fair | inject >= 20% OR extract >= 15% |
| Poor | below fair thresholds |
| n/a | resource type not present in TF provider schema |

## Per-Service Resource Coverage

How many of the AWS Terraform provider's `aws_*` resource types each winterbaume service handles. Of 1191 schema resources classified to a winterbaume service, 1133 are handled (69 still missing — see the per-service missing list in `.agents/docs/TERRAFORM_RESOURCE_COVERAGE.md`).

| Service | Prefix | Handled | Schema | Missing | Coverage | Note |
|---------|--------|---------|--------|---------|----------|------|
| ebs | `aws_ebs_` | 2 | 8 | 6 | 25% |  |
| account | `aws_account_` | 1 | 3 | 2 | 33% |  |
| apigateway | `aws_api_gateway_` | 24 | 26 | 2 | 92% |  |
| cloudtrail | `aws_cloudtrail` | 1 | 3 | 2 | 33% |  |
| codeartifact | `aws_codeartifact_` | 2 | 4 | 2 | 50% |  |
| codepipeline | `aws_codepipeline` | 1 | 3 | 2 | 33% |  |
| costexplorer | `aws_ce_` | 2 | 4 | 2 | 50% |  |
| elasticbeanstalk | `aws_elastic_beanstalk_` | 2 | 4 | 2 | 50% |  |
| iam | `aws_iam_` | 32 | 34 | 2 | 94% |  |
| ivs | `aws_ivs_` | 1 | 3 | 2 | 33% |  |
| rds | `aws_db_`, `aws_rds_` | 27 | 29 | 2 | 93% |  |
| rekognition | `aws_rekognition_` | 1 | 3 | 2 | 33% |  |
| route53domains | `aws_route53domains_` | 1 | 3 | 2 | 33% |  |
| secretsmanager | `aws_secretsmanager_` | 2 | 4 | 2 | 50% |  |
| servicecatalogappregistry | `aws_servicecatalogappregistry_` | 1 | 3 | 2 | 33% |  |
| servicequotas | `aws_servicequotas_` | 1 | 3 | 2 | 33% |  |
| signer | `aws_signer_` | 1 | 3 | 2 | 33% |  |
| stepfunctions | `aws_sfn_` | 1 | 3 | 2 | 33% |  |
| synthetics | `aws_synthetics_` | 1 | 3 | 2 | 33% |  |
| transcribe | `aws_transcribe_` | 2 | 4 | 2 | 50% |  |
| workspaces | `aws_workspaces_` | 2 | 4 | 2 | 50% |  |
| xray | `aws_xray_` | 2 | 4 | 2 | 50% |  |
| accessanalyzer | `aws_accessanalyzer_` | 1 | 2 | 1 | 50% |  |
| acm | `aws_acm_` | 1 | 2 | 1 | 50% |  |
| appflow | `aws_appflow_` | 1 | 2 | 1 | 50% |  |
| applicationautoscaling | `aws_appautoscaling_` | 2 | 3 | 1 | 67% |  |
| autoscaling | `aws_autoscaling_`, `aws_launch_configuration`, `aws_launch_template` | 9 | 10 | 1 | 90% |  |
| bedrock | `aws_bedrock_` | 5 | 6 | 1 | 83% |  |
| budgets | `aws_budgets_` | 1 | 2 | 1 | 50% |  |
| chatbot | `aws_chatbot_` | 2 | 2 | 1 | 50% |  |
| cloudhsmv2 | `aws_cloudhsm_v2_` | 1 | 2 | 1 | 50% |  |
| codedeploy | `aws_codedeploy_` | 2 | 3 | 1 | 67% |  |
| cognitoidentity | `aws_cognito_identity_` | 3 | 4 | 1 | 75% |  |
| comprehend | `aws_comprehend_` | 1 | 2 | 1 | 50% |  |
| datapipeline | `aws_datapipeline_` | 1 | 2 | 1 | 50% |  |
| dsql | `aws_dsql_` | 1 | 2 | 1 | 50% |  |
| elasticloadbalancing | `aws_elb` | 1 | 2 | 1 | 50% |  |
| emrcontainers | `aws_emrcontainers_` | 1 | 2 | 1 | 50% |  |
| glacier | `aws_glacier_` | 1 | 2 | 1 | 50% |  |
| identitystore | `aws_identitystore_` | 2 | 3 | 1 | 67% |  |
| kinesisanalyticsv2 | `aws_kinesis_analytics_`, `aws_kinesisanalyticsv2_` | 2 | 3 | 1 | 67% |  |
| mediastore | `aws_media_store_`, `aws_mediastore_` | 1 | 2 | 1 | 50% |  |
| resourcegroups | `aws_resourcegroups_` | 1 | 2 | 1 | 50% |  |
| acmpca | `aws_acmpca_` | 5 | 5 | 0 | 100% |  |
| amp | `aws_prometheus_` | 5 | 5 | 0 | 100% |  |
| amplify | `aws_amplify_` | 5 | 5 | 0 | 100% |  |
| apigatewayv2 | `aws_apigatewayv2_` | 12 | 12 | 0 | 100% |  |
| appconfig | `aws_appconfig_` | 8 | 8 | 0 | 100% |  |
| appfabric | `aws_appfabric_` | 5 | 5 | 0 | 100% |  |
| applicationcostprofiler | `aws_applicationcostprofiler_` | 1 | 0 | 0 | — |  |
| appmesh | `aws_appmesh_` | 7 | 7 | 0 | 100% |  |
| apprunner | `aws_apprunner_` | 9 | 9 | 0 | 100% |  |
| appsync | `aws_appsync_` | 10 | 10 | 0 | 100% |  |
| athena | `aws_athena_` | 6 | 6 | 0 | 100% |  |
| auditmanager | `aws_auditmanager_` | 8 | 8 | 0 | 100% |  |
| backup | `aws_backup_` | 13 | 13 | 0 | 100% |  |
| batch | `aws_batch_` | 4 | 4 | 0 | 100% |  |
| bedrockagent | `aws_bedrockagent_` | 8 | 8 | 0 | 100% |  |
| cloudformation | `aws_cloudformation_` | 5 | 5 | 0 | 100% |  |
| cloudfront | `aws_cloudfront_` | 16 | 16 | 0 | 100% |  |
| cloudwatch | `aws_cloudwatch_metric_`, `aws_cloudwatch_dashboard`, `aws_cloudwatch_composite_alarm` | 4 | 4 | 0 | 100% |  |
| codebuild | `aws_codebuild_` | 6 | 6 | 0 | 100% |  |
| codecommit | `aws_codecommit_` | 4 | 4 | 0 | 100% |  |
| cognitoidp | `aws_cognito_user_`, `aws_cognito_managed_user_pool_client`, +3 more | 9 | 9 | 0 | 100% |  |
| config | `aws_config_` | 13 | 13 | 0 | 100% |  |
| connect | `aws_connect_` | 16 | 16 | 0 | 100% |  |
| datasync | `aws_datasync_` | 13 | 13 | 0 | 100% |  |
| dax | `aws_dax_` | 3 | 3 | 0 | 100% |  |
| directconnect | `aws_dx_` | 19 | 19 | 0 | 100% |  |
| directory | `aws_directory_service_` | 8 | 8 | 0 | 100% |  |
| dms | `aws_dms_` | 8 | 8 | 0 | 100% |  |
| dynamodb | `aws_dynamodb_` | 9 | 9 | 0 | 100% |  |
| ec2 | `aws_vpc`, `aws_subnet`, +35 more | 145 | 139 | 0 | 104% |  |
| ec2instanceconnect | `aws_ec2_instance_connect_` | 1 | 1 | 0 | 100% |  |
| ecr | `aws_ecr_` | 9 | 9 | 0 | 100% |  |
| ecs | `aws_ecs_` | 8 | 8 | 0 | 100% |  |
| efs | `aws_efs_` | 6 | 6 | 0 | 100% |  |
| eks | `aws_eks_` | 8 | 8 | 0 | 100% |  |
| elasticache | `aws_elasticache_` | 10 | 10 | 0 | 100% |  |
| elbv2 | `aws_lb`, `aws_alb` | 16 | 16 | 0 | 100% |  |
| emr | `aws_emr_` | 8 | 8 | 0 | 100% |  |
| emrserverless | `aws_emrserverless_` | 1 | 1 | 0 | 100% |  |
| events | `aws_cloudwatch_event_` | 9 | 9 | 0 | 100% |  |
| firehose | `aws_kinesis_firehose_` | 1 | 1 | 0 | 100% |  |
| fis | `aws_fis_` | 1 | 1 | 0 | 100% |  |
| fsx | `aws_fsx_` | 11 | 11 | 0 | 100% |  |
| glue | `aws_glue_` | 20 | 20 | 0 | 100% |  |
| guardduty | `aws_guardduty_` | 13 | 13 | 0 | 100% |  |
| inspector2 | `aws_inspector2_` | 5 | 5 | 0 | 100% |  |
| iot | `aws_iot_` | 19 | 19 | 0 | 100% |  |
| kafka | `aws_msk_` | 8 | 8 | 0 | 100% |  |
| keyspaces | `aws_keyspaces_` | 2 | 2 | 0 | 100% |  |
| kinesis | `aws_kinesis_stream`, `aws_kinesis_resource_policy` | 3 | 3 | 0 | 100% |  |
| kinesisvideo | `aws_kinesis_video_` | 1 | 1 | 0 | 100% |  |
| kms | `aws_kms_` | 9 | 9 | 0 | 100% |  |
| lakeformation | `aws_lakeformation_` | 8 | 8 | 0 | 100% |  |
| lambda | `aws_lambda_` | 13 | 13 | 0 | 100% |  |
| lexmodelsv2 | `aws_lexv2models_` | 6 | 6 | 0 | 100% |  |
| logs | `aws_cloudwatch_log_` | 15 | 15 | 0 | 100% |  |
| macie2 | `aws_macie2_` | 9 | 9 | 0 | 100% |  |
| medialive | `aws_medialive_` | 5 | 5 | 0 | 100% |  |
| mediapackage | `aws_media_package_` | 1 | 1 | 0 | 100% |  |
| mediapackagev2 | `aws_media_packagev2_` | 1 | 1 | 0 | 100% |  |
| memorydb | `aws_memorydb_` | 7 | 7 | 0 | 100% |  |
| mq | `aws_mq_` | 2 | 2 | 0 | 100% |  |
| neptune | `aws_neptune_` | 9 | 9 | 0 | 100% |  |
| networkfirewall | `aws_networkfirewall_` | 6 | 6 | 0 | 100% |  |
| networkmanager | `aws_networkmanager_` | 19 | 19 | 0 | 100% |  |
| opensearch | `aws_opensearch_` | 9 | 9 | 0 | 100% |  |
| opensearchserverless | `aws_opensearchserverless_` | 6 | 6 | 0 | 100% |  |
| organizations | `aws_organizations_` | 7 | 7 | 0 | 100% |  |
| osis | `aws_osis_` | 1 | 1 | 0 | 100% |  |
| outposts | `aws_outposts_` | 2 | 0 | 0 | — |  |
| pinpoint | `aws_pinpoint_` | 12 | 12 | 0 | 100% |  |
| pipes | `aws_pipes_` | 1 | 1 | 0 | 100% |  |
| quicksight | `aws_quicksight_` | 20 | 20 | 0 | 100% |  |
| ram | `aws_ram_` | 5 | 5 | 0 | 100% |  |
| redshift | `aws_redshift_` | 23 | 23 | 0 | 100% |  |
| resiliencehub | `aws_resiliencehub_` | 1 | 1 | 0 | 100% |  |
| rolesanywhere | `aws_rolesanywhere_` | 2 | 2 | 0 | 100% |  |
| route53 | `aws_route53_cidr_`, `aws_route53_delegation_set`, +9 more | 13 | 13 | 0 | 100% |  |
| route53resolver | `aws_route53_resolver_` | 12 | 12 | 0 | 100% |  |
| s3 | `aws_s3_` | 26 | 26 | 0 | 100% |  |
| s3control | `aws_s3control_` | 15 | 14 | 0 | 100% |  |
| s3tables | `aws_s3tables_` | 5 | 5 | 0 | 100% |  |
| sagemaker | `aws_sagemaker_` | 30 | 30 | 0 | 100% |  |
| scheduler | `aws_scheduler_` | 2 | 2 | 0 | 100% |  |
| securityhub | `aws_securityhub_` | 15 | 15 | 0 | 100% |  |
| servicecatalog | `aws_servicecatalog_` | 13 | 13 | 0 | 100% |  |
| servicediscovery | `aws_service_discovery_` | 5 | 5 | 0 | 100% |  |
| ses | `aws_sesv2_` | 11 | 11 | 0 | 100% |  |
| sesv1 | `aws_ses_` | 14 | 14 | 0 | 100% |  |
| shield | `aws_shield_` | 8 | 8 | 0 | 100% |  |
| simpledbv2 | `aws_simpledb_` | 1 | 1 | 0 | 100% |  |
| sns | `aws_sns_` | 6 | 6 | 0 | 100% |  |
| sqs | `aws_sqs_` | 4 | 4 | 0 | 100% |  |
| ssm | `aws_ssm_` | 12 | 12 | 0 | 100% |  |
| ssoadmin | `aws_ssoadmin_` | 12 | 12 | 0 | 100% |  |
| swf | `aws_swf_` | 1 | 1 | 0 | 100% |  |
| timestreaminfluxdb | `aws_timestreaminfluxdb_` | 1 | 1 | 0 | 100% |  |
| timestreamquery | `aws_timestreamquery_` | 1 | 1 | 0 | 100% |  |
| timestreamwrite | `aws_timestreamwrite_` | 2 | 2 | 0 | 100% |  |
| transfer | `aws_transfer_` | 10 | 10 | 0 | 100% |  |
| vpclattice | `aws_vpclattice_` | 14 | 14 | 0 | 100% |  |
| wafv2 | `aws_wafv2_` | 7 | 7 | 0 | 100% |  |

## Per-Resource Attribute Coverage

| Resource Type | Inject | Extract | TF Schema | Inject% | Extract% | Rating |
|---|---|---|---|---|---|---|
| `aws_accessanalyzer_analyzer` | 5 | 5 | 6 | 67% | 67% | excellent [+] |
| `aws_account_alternate_contact` | 6 | 6 | 7 | 71% | 71% | excellent [+] |
| `aws_acm_certificate` | 15 | 20 | 23 | 39% | 61% | good [~] |
| `aws_acmpca_certificate` | 10 | 0 | 9 | 33% | 0% | fair [-] |
| `aws_acmpca_certificate_authority` | 10 | 25 | 17 | 41% | 65% | good [~] |
| `aws_acmpca_certificate_authority_certificate` | 4 | 0 | 3 | 100% | 0% | good [~] |
| `aws_acmpca_permission` | 4 | 0 | 5 | 60% | 0% | good [~] |
| `aws_acmpca_policy` | 3 | 0 | 2 | 100% | 0% | good [~] |
| `aws_prometheus_alert_manager_definition` | 3 | 0 | 2 | 100% | 0% | good [~] |
| `aws_prometheus_rule_group_namespace` | 5 | 6 | 6 | 67% | 83% | excellent [+] |
| `aws_prometheus_scraper` | 5 | 0 | 10 | 30% | 0% | fair [-] |
| `aws_prometheus_workspace` | 9 | 6 | 7 | 0% | 57% | good [~] |
| `aws_prometheus_workspace_configuration` | 3 | 0 | 4 | 50% | 0% | good [~] |
| `aws_amplify_app` | 14 | 20 | 25 | 44% | 64% | good [~] |
| `aws_amplify_backend_environment` | 5 | 0 | 5 | 100% | 0% | good [~] |
| `aws_amplify_branch` | 7 | 15 | 23 | 26% | 52% | good [~] |
| `aws_amplify_domain_association` | 6 | 7 | 8 | 62% | 50% | excellent [+] |
| `aws_amplify_webhook` | 5 | 0 | 5 | 100% | 0% | good [~] |
| `aws_api_gateway_account` | 5 | 7 | 5 | 80% | 80% | excellent [+] |
| `aws_api_gateway_api_key` | 9 | 8 | 10 | 40% | 60% | good [~] |
| `aws_api_gateway_authorizer` | 10 | 10 | 10 | 90% | 90% | excellent [+] |
| `aws_api_gateway_base_path_mapping` | 5 | 5 | 5 | 80% | 80% | excellent [+] |
| `aws_api_gateway_client_certificate` | 5 | 6 | 7 | 57% | 71% | good [~] |
| `aws_api_gateway_deployment` | 8 | 7 | 10 | 70% | 60% | excellent [+] |
| `aws_api_gateway_documentation_part` | 4 | 9 | 4 | 75% | 75% | excellent [+] |
| `aws_api_gateway_documentation_version` | 4 | 5 | 3 | 100% | 100% | excellent [+] |
| `aws_api_gateway_domain_name` | 6 | 5 | 22 | 23% | 18% | fair [-] |
| `aws_api_gateway_domain_name_access_association` | 7 | 6 | 6 | 100% | 83% | excellent [+] |
| `aws_api_gateway_gateway_response` | 6 | 6 | 5 | 100% | 100% | excellent [+] |
| `aws_api_gateway_integration` | 17 | 17 | 17 | 94% | 94% | excellent [+] |
| `aws_api_gateway_integration_response` | 9 | 9 | 8 | 100% | 100% | excellent [+] |
| `aws_api_gateway_method` | 11 | 11 | 11 | 91% | 91% | excellent [+] |
| `aws_api_gateway_method_response` | 7 | 7 | 6 | 100% | 100% | excellent [+] |
| `aws_api_gateway_model` | 6 | 6 | 5 | 100% | 100% | excellent [+] |
| `aws_api_gateway_request_validator` | 5 | 5 | 4 | 100% | 100% | excellent [+] |
| `aws_api_gateway_resource` | 5 | 5 | 4 | 100% | 100% | excellent [+] |
| `aws_api_gateway_rest_api` | 13 | 15 | 18 | 61% | 61% | excellent [+] |
| `aws_api_gateway_rest_api_policy` | 3 | 3 | 2 | 100% | 100% | excellent [+] |
| `aws_api_gateway_stage` | 14 | 15 | 18 | 72% | 67% | excellent [+] |
| `aws_api_gateway_usage_plan` | 9 | 15 | 9 | 89% | 78% | excellent [+] |
| `aws_api_gateway_usage_plan_key` | 6 | 6 | 5 | 100% | 100% | excellent [+] |
| `aws_api_gateway_vpc_link` | 8 | 7 | 6 | 83% | 67% | excellent [+] |
| `aws_apigatewayv2_api` | 8 | 13 | 19 | 32% | 53% | good [~] |
| `aws_apigatewayv2_api_mapping` | 6 | 6 | 4 | 100% | 100% | excellent [+] |
| `aws_apigatewayv2_authorizer` | 12 | 12 | 11 | 82% | 82% | excellent [+] |
| `aws_apigatewayv2_deployment` | 4 | 6 | 4 | 50% | 75% | good [~] |
| `aws_apigatewayv2_domain_name` | 4 | 8 | 8 | 38% | 50% | good [~] |
| `aws_apigatewayv2_integration` | 9 | 9 | 19 | 37% | 37% | good [~] |
| `aws_apigatewayv2_integration_response` | 9 | 9 | 6 | 100% | 100% | excellent [+] |
| `aws_apigatewayv2_model` | 7 | 7 | 5 | 100% | 100% | excellent [+] |
| `aws_apigatewayv2_route` | 7 | 7 | 12 | 42% | 42% | good [~] |
| `aws_apigatewayv2_route_response` | 6 | 6 | 5 | 80% | 80% | excellent [+] |
| `aws_apigatewayv2_stage` | 7 | 9 | 15 | 40% | 53% | good [~] |
| `aws_apigatewayv2_vpc_link` | 6 | 7 | 6 | 67% | 83% | excellent [+] |
| `aws_appconfig_application` | 7 | 4 | 5 | 80% | 60% | excellent [+] |
| `aws_appconfig_configuration_profile` | 8 | 8 | 12 | 58% | 58% | good [~] |
| `aws_appconfig_deployment` | 10 | 9 | 13 | 69% | 62% | excellent [+] |
| `aws_appconfig_deployment_strategy` | 7 | 8 | 10 | 60% | 70% | excellent [+] |
| `aws_appconfig_environment` | 15 | 9 | 9 | 44% | 67% | good [~] |
| `aws_appconfig_extension` | 6 | 5 | 8 | 62% | 50% | excellent [+] |
| `aws_appconfig_extension_association` | 6 | 6 | 5 | 80% | 100% | excellent [+] |
| `aws_appconfig_hosted_configuration_version` | 6 | 6 | 7 | 71% | 71% | excellent [+] |
| `aws_appfabric_app_authorization` | 5 | 0 | 13 | 31% | 0% | fair [-] |
| `aws_appfabric_app_authorization_connection` | 3 | 0 | 6 | 33% | 0% | fair [-] |
| `aws_appfabric_app_bundle` | 4 | 4 | 4 | 75% | 75% | excellent [+] |
| `aws_appfabric_ingestion` | 6 | 0 | 7 | 71% | 0% | good [~] |
| `aws_appfabric_ingestion_destination` | 4 | 0 | 8 | 38% | 0% | fair [-] |
| `aws_appflow_flow` | 11 | 10 | 12 | 83% | 75% | excellent [+] |
| `aws_appautoscaling_policy` | 8 | 7 | 9 | 56% | 67% | good [~] |
| `aws_appautoscaling_target` | 9 | 12 | 10 | 80% | 80% | excellent [+] |
| `aws_applicationcostprofiler_report_definition` | 5 | 8 | 0 | 0% | 0% | n/a [?] |
| `aws_appmesh_gateway_route` | 12 | 11 | 11 | 91% | 91% | excellent [+] |
| `aws_appmesh_mesh` | 10 | 11 | 9 | 89% | 89% | excellent [+] |
| `aws_appmesh_route` | 8 | 11 | 11 | 9% | 91% | good [~] |
| `aws_appmesh_virtual_gateway` | 11 | 10 | 10 | 90% | 90% | excellent [+] |
| `aws_appmesh_virtual_node` | 11 | 10 | 10 | 90% | 90% | excellent [+] |
| `aws_appmesh_virtual_router` | 11 | 10 | 10 | 90% | 90% | excellent [+] |
| `aws_appmesh_virtual_service` | 11 | 10 | 10 | 90% | 90% | excellent [+] |
| `aws_apprunner_auto_scaling_configuration_version` | 10 | 11 | 12 | 75% | 83% | excellent [+] |
| `aws_apprunner_connection` | 6 | 7 | 6 | 83% | 100% | excellent [+] |
| `aws_apprunner_custom_domain_association` | 5 | 0 | 6 | 67% | 0% | good [~] |
| `aws_apprunner_default_auto_scaling_configuration_version` | 2 | 2 | 1 | 100% | 100% | excellent [+] |
| `aws_apprunner_deployment` | 4 | 0 | 4 | 75% | 0% | good [~] |
| `aws_apprunner_observability_configuration` | 7 | 0 | 8 | 75% | 0% | good [~] |
| `aws_apprunner_service` | 13 | 13 | 14 | 86% | 86% | excellent [+] |
| `aws_apprunner_vpc_connector` | 6 | 0 | 8 | 62% | 0% | good [~] |
| `aws_apprunner_vpc_ingress_connection` | 7 | 0 | 8 | 75% | 0% | good [~] |
| `aws_appsync_api_cache` | 8 | 7 | 6 | 100% | 100% | excellent [+] |
| `aws_appsync_api_key` | 5 | 5 | 5 | 60% | 60% | excellent [+] |
| `aws_appsync_datasource` | 6 | 0 | 13 | 46% | 0% | good [~] |
| `aws_appsync_domain_name` | 5 | 0 | 5 | 100% | 0% | good [~] |
| `aws_appsync_domain_name_api_association` | 2 | 0 | 2 | 100% | 0% | good [~] |
| `aws_appsync_function` | 11 | 0 | 13 | 77% | 0% | good [~] |
| `aws_appsync_graphql_api` | 10 | 15 | 20 | 45% | 70% | good [~] |
| `aws_appsync_resolver` | 9 | 0 | 14 | 64% | 0% | good [~] |
| `aws_appsync_source_api_association` | 5 | 0 | 9 | 44% | 0% | good [~] |
| `aws_appsync_type` | 6 | 6 | 6 | 83% | 83% | excellent [+] |
| `aws_athena_capacity_reservation` | 16 | 6 | 8 | 12% | 62% | good [~] |
| `aws_athena_data_catalog` | 6 | 6 | 7 | 71% | 71% | excellent [+] |
| `aws_athena_database` | 6 | 0 | 8 | 0% | 0% | poor [!] |
| `aws_athena_named_query` | 6 | 6 | 5 | 100% | 100% | excellent [+] |
| `aws_athena_prepared_statement` | 5 | 5 | 5 | 80% | 80% | excellent [+] |
| `aws_athena_workgroup` | 6 | 6 | 8 | 62% | 62% | excellent [+] |
| `aws_auditmanager_account_registration` | 4 | 2 | 4 | 75% | 25% | good [~] |
| `aws_auditmanager_assessment` | 5 | 8 | 11 | 36% | 64% | good [~] |
| `aws_auditmanager_assessment_delegation` | 6 | 0 | 7 | 71% | 0% | good [~] |
| `aws_auditmanager_assessment_report` | 4 | 0 | 5 | 60% | 0% | good [~] |
| `aws_auditmanager_control` | 5 | 7 | 10 | 40% | 60% | good [~] |
| `aws_auditmanager_framework` | 6 | 6 | 8 | 62% | 62% | excellent [+] |
| `aws_auditmanager_framework_share` | 5 | 0 | 5 | 80% | 0% | good [~] |
| `aws_auditmanager_organization_admin_account_registration` | 2 | 0 | 2 | 50% | 0% | good [~] |
| `aws_autoscaling_attachment` | 4 | 0 | 3 | 100% | 0% | good [~] |
| `aws_autoscaling_group` | 13 | 14 | 45 | 22% | 27% | fair [-] |
| `aws_autoscaling_group_tag` | 2 | 0 | 2 | 50% | 0% | good [~] |
| `aws_autoscaling_lifecycle_hook` | 9 | 9 | 8 | 100% | 100% | excellent [+] |
| `aws_autoscaling_notification` | 2 | 0 | 3 | 33% | 0% | fair [-] |
| `aws_autoscaling_policy` | 9 | 8 | 14 | 57% | 50% | good [~] |
| `aws_autoscaling_schedule` | 11 | 10 | 10 | 100% | 90% | excellent [+] |
| `aws_autoscaling_traffic_source_attachment` | 3 | 0 | 3 | 67% | 0% | good [~] |
| `aws_launch_configuration` | 11 | 11 | 19 | 53% | 53% | good [~] |
| `aws_backup_framework` | 6 | 10 | 10 | 40% | 90% | good [~] |
| `aws_backup_global_settings` | 1 | 0 | 1 | 0% | 0% | poor [!] |
| `aws_backup_logically_air_gapped_vault` | 6 | 0 | 7 | 71% | 0% | good [~] |
| `aws_backup_plan` | 7 | 7 | 7 | 86% | 86% | excellent [+] |
| `aws_backup_region_settings` | 1 | 0 | 2 | 0% | 0% | poor [!] |
| `aws_backup_report_plan` | 8 | 10 | 9 | 78% | 100% | excellent [+] |
| `aws_backup_restore_testing_plan` | 8 | 9 | 8 | 88% | 100% | excellent [+] |
| `aws_backup_restore_testing_selection` | 9 | 9 | 8 | 100% | 100% | excellent [+] |
| `aws_backup_selection` | 4 | 0 | 7 | 43% | 0% | good [~] |
| `aws_backup_vault` | 4 | 5 | 8 | 38% | 50% | good [~] |
| `aws_backup_vault_lock_configuration` | 5 | 0 | 5 | 80% | 0% | good [~] |
| `aws_backup_vault_notifications` | 5 | 5 | 4 | 100% | 100% | excellent [+] |
| `aws_backup_vault_policy` | 4 | 4 | 3 | 100% | 100% | excellent [+] |
| `aws_batch_compute_environment` | 11 | 12 | 14 | 64% | 71% | excellent [+] |
| `aws_batch_job_definition` | 13 | 18 | 18 | 67% | 61% | excellent [+] |
| `aws_batch_job_queue` | 11 | 11 | 11 | 91% | 73% | excellent [+] |
| `aws_batch_scheduling_policy` | 6 | 10 | 5 | 100% | 80% | excellent [+] |
| `aws_bedrock_custom_model` | 12 | 14 | 19 | 58% | 63% | good [~] |
| `aws_bedrock_guardrail` | 18 | 18 | 18 | 89% | 89% | excellent [+] |
| `aws_bedrock_guardrail_version` | 5 | 5 | 5 | 80% | 80% | excellent [+] |
| `aws_bedrock_inference_profile` | 9 | 13 | 12 | 67% | 83% | excellent [+] |
| `aws_bedrock_model_invocation_logging_configuration` | 2 | 11 | 1 | 100% | 100% | excellent [+] |
| `aws_bedrock_provisioned_model_throughput` | 7 | 7 | 8 | 75% | 75% | excellent [+] |
| `aws_bedrockagent_agent` | 21 | 20 | 20 | 70% | 65% | excellent [+] |
| `aws_bedrockagent_agent_action_group` | 14 | 14 | 13 | 77% | 77% | excellent [+] |
| `aws_bedrockagent_agent_alias` | 12 | 11 | 9 | 78% | 67% | excellent [+] |
| `aws_bedrockagent_agent_collaborator` | 11 | 11 | 9 | 78% | 78% | excellent [+] |
| `aws_bedrockagent_agent_knowledge_base_association` | 8 | 8 | 6 | 83% | 83% | excellent [+] |
| `aws_bedrockagent_data_source` | 12 | 12 | 9 | 89% | 89% | excellent [+] |
| `aws_bedrockagent_knowledge_base` | 13 | 12 | 12 | 75% | 67% | excellent [+] |
| `aws_bedrockagent_prompt` | 11 | 10 | 11 | 91% | 82% | excellent [+] |
| `aws_budgets_budget` | 10 | 17 | 17 | 53% | 71% | good [~] |
| `aws_chatbot_microsoft_teams_channel_configuration` | 11 | 10 | 0 | 0% | 0% | n/a [?] |
| `aws_chatbot_slack_channel_configuration` | 9 | 8 | 14 | 57% | 57% | good [~] |
| `aws_cloudformation_stack` | 8 | 11 | 16 | 44% | 56% | good [~] |
| `aws_cloudformation_stack_instances` | 4 | 0 | 11 | 27% | 0% | fair [-] |
| `aws_cloudformation_stack_set` | 9 | 8 | 18 | 44% | 39% | good [~] |
| `aws_cloudformation_stack_set_instance` | 4 | 5 | 12 | 33% | 33% | good [~] |
| `aws_cloudformation_type` | 6 | 7 | 17 | 29% | 29% | fair [-] |
| `aws_cloudfront_cache_policy` | 7 | 7 | 8 | 75% | 75% | excellent [+] |
| `aws_cloudfront_continuous_deployment_policy` | 3 | 3 | 6 | 33% | 33% | good [~] |
| `aws_cloudfront_distribution` | 11 | 31 | 32 | 31% | 62% | good [~] |
| `aws_cloudfront_field_level_encryption_config` | 3 | 3 | 6 | 33% | 33% | good [~] |
| `aws_cloudfront_field_level_encryption_profile` | 4 | 4 | 6 | 50% | 50% | good [~] |
| `aws_cloudfront_function` | 9 | 8 | 10 | 80% | 70% | excellent [+] |
| `aws_cloudfront_key_group` | 5 | 5 | 4 | 100% | 100% | excellent [+] |
| `aws_cloudfront_key_value_store` | 5 | 6 | 6 | 67% | 67% | excellent [+] |
| `aws_cloudfront_monitoring_subscription` | 3 | 5 | 2 | 100% | 100% | excellent [+] |
| `aws_cloudfront_origin_access_control` | 7 | 7 | 7 | 86% | 86% | excellent [+] |
| `aws_cloudfront_origin_access_identity` | 5 | 7 | 7 | 57% | 86% | good [~] |
| `aws_cloudfront_origin_request_policy` | 4 | 4 | 7 | 43% | 43% | good [~] |
| `aws_cloudfront_public_key` | 6 | 6 | 6 | 83% | 83% | excellent [+] |
| `aws_cloudfront_realtime_log_config` | 5 | 5 | 5 | 80% | 80% | excellent [+] |
| `aws_cloudfront_response_headers_policy` | 4 | 4 | 9 | 33% | 33% | good [~] |
| `aws_cloudfront_vpc_origin` | 5 | 8 | 6 | 67% | 50% | excellent [+] |
| `aws_cloudhsm_v2_cluster` | 20 | 20 | 12 | 25% | 58% | good [~] |
| `aws_cloudtrail` | 12 | 20 | 20 | 55% | 60% | good [~] |
| `aws_cloudwatch_composite_alarm` | 6 | 9 | 11 | 45% | 73% | good [~] |
| `aws_cloudwatch_dashboard` | 4 | 4 | 3 | 100% | 100% | excellent [+] |
| `aws_cloudwatch_metric_alarm` | 17 | 17 | 24 | 67% | 67% | excellent [+] |
| `aws_cloudwatch_metric_stream` | 7 | 9 | 16 | 38% | 50% | good [~] |
| `aws_codeartifact_domain` | 12 | 7 | 10 | 10% | 60% | good [~] |
| `aws_codeartifact_repository` | 8 | 9 | 10 | 40% | 80% | good [~] |
| `aws_codebuild_fleet` | 0 | 0 | 16 | 0% | 0% | poor [!] |
| `aws_codebuild_project` | 15 | 29 | 27 | 44% | 81% | good [~] |
| `aws_codebuild_report_group` | 0 | 0 | 8 | 0% | 0% | poor [!] |
| `aws_codebuild_resource_policy` | 0 | 0 | 2 | 0% | 0% | poor [!] |
| `aws_codebuild_source_credential` | 0 | 0 | 5 | 0% | 0% | poor [!] |
| `aws_codebuild_webhook` | 0 | 0 | 9 | 0% | 0% | poor [!] |
| `aws_codecommit_approval_rule_template` | 4 | 0 | 8 | 50% | 0% | good [~] |
| `aws_codecommit_approval_rule_template_association` | 2 | 0 | 2 | 100% | 0% | good [~] |
| `aws_codecommit_repository` | 6 | 7 | 10 | 20% | 60% | good [~] |
| `aws_codecommit_trigger` | 2 | 0 | 3 | 67% | 0% | good [~] |
| `aws_codedeploy_app` | 7 | 5 | 8 | 38% | 50% | good [~] |
| `aws_codedeploy_deployment_group` | 12 | 23 | 22 | 50% | 82% | good [~] |
| `aws_codepipeline` | 8 | 12 | 12 | 42% | 92% | good [~] |
| `aws_cognito_identity_pool` | 9 | 13 | 11 | 73% | 82% | excellent [+] |
| `aws_cognito_identity_pool_provider_principal_tag` | 5 | 5 | 4 | 100% | 100% | excellent [+] |
| `aws_cognito_identity_pool_roles_attachment` | 4 | 4 | 3 | 100% | 100% | excellent [+] |
| `aws_account_id` | 0 | 5 | 0 | 0% | 0% | n/a [?] |
| `aws_cognito_identity_provider` | 7 | 7 | 6 | 100% | 100% | excellent [+] |
| `aws_cognito_managed_user_pool_client` | 13 | 0 | 25 | 48% | 0% | good [~] |
| `aws_cognito_resource_server` | 5 | 8 | 5 | 80% | 100% | excellent [+] |
| `aws_cognito_user_group` | 5 | 6 | 5 | 20% | 100% | good [~] |
| `aws_cognito_user_in_group` | 4 | 3 | 3 | 100% | 100% | excellent [+] |
| `aws_cognito_user_pool` | 24 | 22 | 36 | 61% | 53% | excellent [+] |
| `aws_cognito_user_pool_client` | 16 | 14 | 24 | 62% | 50% | excellent [+] |
| `aws_cognito_user_pool_domain` | 4 | 6 | 10 | 30% | 50% | good [~] |
| `aws_cognito_user_pool_ui_customization` | 5 | 0 | 8 | 50% | 0% | good [~] |
| `aws_comprehend_entity_recognizer` | 8 | 14 | 13 | 46% | 54% | good [~] |
| `aws_config_aggregate_authorization` | 3 | 4 | 5 | 60% | 60% | excellent [+] |
| `aws_config_config_rule` | 9 | 12 | 11 | 73% | 82% | excellent [+] |
| `aws_config_configuration_aggregator` | 3 | 3 | 6 | 33% | 33% | good [~] |
| `aws_config_configuration_recorder` | 5 | 4 | 4 | 100% | 75% | excellent [+] |
| `aws_config_configuration_recorder_status` | 3 | 3 | 2 | 100% | 100% | excellent [+] |
| `aws_config_conformance_pack` | 5 | 0 | 7 | 57% | 0% | good [~] |
| `aws_config_delivery_channel` | 5 | 5 | 6 | 67% | 67% | excellent [+] |
| `aws_config_organization_conformance_pack` | 5 | 5 | 9 | 44% | 44% | good [~] |
| `aws_config_organization_custom_policy_rule` | 8 | 8 | 15 | 47% | 47% | good [~] |
| `aws_config_organization_custom_rule` | 7 | 7 | 13 | 46% | 46% | good [~] |
| `aws_config_organization_managed_rule` | 7 | 7 | 12 | 50% | 50% | good [~] |
| `aws_config_remediation_configuration` | 9 | 9 | 11 | 73% | 73% | excellent [+] |
| `aws_config_retention_configuration` | 3 | 3 | 2 | 100% | 100% | excellent [+] |
| `aws_connect_bot_association` | 2 | 0 | 2 | 100% | 0% | good [~] |
| `aws_connect_contact_flow` | 9 | 0 | 11 | 82% | 0% | good [~] |
| `aws_connect_contact_flow_module` | 9 | 0 | 10 | 90% | 0% | good [~] |
| `aws_connect_hours_of_operation` | 7 | 0 | 9 | 78% | 0% | good [~] |
| `aws_connect_instance` | 9 | 11 | 17 | 47% | 59% | good [~] |
| `aws_connect_instance_storage_config` | 2 | 0 | 4 | 50% | 0% | good [~] |
| `aws_connect_lambda_function_association` | 2 | 0 | 2 | 100% | 0% | good [~] |
| `aws_connect_phone_number` | 6 | 0 | 11 | 55% | 0% | good [~] |
| `aws_connect_queue` | 9 | 0 | 12 | 75% | 0% | good [~] |
| `aws_connect_quick_connect` | 6 | 0 | 8 | 75% | 0% | good [~] |
| `aws_connect_routing_profile` | 7 | 0 | 10 | 70% | 0% | good [~] |
| `aws_connect_security_profile` | 7 | 0 | 9 | 78% | 0% | good [~] |
| `aws_connect_user` | 9 | 0 | 13 | 69% | 0% | good [~] |
| `aws_connect_user_hierarchy_group` | 7 | 0 | 9 | 78% | 0% | good [~] |
| `aws_connect_user_hierarchy_structure` | 1 | 0 | 2 | 50% | 0% | good [~] |
| `aws_connect_vocabulary` | 10 | 0 | 12 | 83% | 0% | good [~] |
| `aws_ce_anomaly_monitor` | 9 | 8 | 7 | 71% | 57% | excellent [+] |
| `aws_ce_anomaly_subscription` | 9 | 11 | 9 | 78% | 67% | excellent [+] |
| `aws_datapipeline_pipeline` | 4 | 4 | 4 | 50% | 75% | good [~] |
| `aws_datasync_agent` | 6 | 0 | 11 | 45% | 0% | good [~] |
| `aws_datasync_location_azure_blob` | 8 | 0 | 11 | 64% | 0% | good [~] |
| `aws_datasync_location_efs` | 8 | 0 | 10 | 70% | 0% | good [~] |
| `aws_datasync_location_fsx_lustre_file_system` | 5 | 0 | 8 | 50% | 0% | good [~] |
| `aws_datasync_location_fsx_ontap_file_system` | 5 | 0 | 10 | 40% | 0% | good [~] |
| `aws_datasync_location_fsx_openzfs_file_system` | 5 | 0 | 9 | 44% | 0% | good [~] |
| `aws_datasync_location_fsx_windows_file_system` | 7 | 0 | 11 | 55% | 0% | good [~] |
| `aws_datasync_location_hdfs` | 10 | 0 | 18 | 50% | 0% | good [~] |
| `aws_datasync_location_nfs` | 5 | 0 | 8 | 50% | 0% | good [~] |
| `aws_datasync_location_object_storage` | 10 | 0 | 13 | 69% | 0% | good [~] |
| `aws_datasync_location_s3` | 6 | 7 | 9 | 56% | 56% | good [~] |
| `aws_datasync_location_smb` | 7 | 0 | 11 | 55% | 0% | good [~] |
| `aws_datasync_task` | 11 | 14 | 14 | 64% | 79% | excellent [+] |
| `aws_dax_cluster` | 11 | 14 | 21 | 48% | 52% | good [~] |
| `aws_dax_parameter_group` | 3 | 3 | 3 | 67% | 67% | excellent [+] |
| `aws_dax_subnet_group` | 5 | 5 | 4 | 100% | 100% | excellent [+] |
| `aws_device` | 0 | 11 | 0 | 0% | 0% | n/a [?] |
| `aws_dx_bgp_peer` | 8 | 0 | 10 | 80% | 0% | good [~] |
| `aws_dx_connection` | 4 | 12 | 18 | 6% | 50% | good [~] |
| `aws_dx_connection_association` | 3 | 0 | 2 | 100% | 0% | good [~] |
| `aws_dx_connection_confirmation` | 2 | 0 | 1 | 100% | 0% | good [~] |
| `aws_dx_gateway` | 4 | 0 | 5 | 60% | 0% | good [~] |
| `aws_dx_gateway_association` | 9 | 0 | 10 | 80% | 0% | good [~] |
| `aws_dx_gateway_association_proposal` | 5 | 0 | 6 | 83% | 0% | good [~] |
| `aws_dx_hosted_connection` | 13 | 0 | 15 | 80% | 0% | good [~] |
| `aws_dx_hosted_private_virtual_interface` | 13 | 0 | 15 | 87% | 0% | good [~] |
| `aws_dx_hosted_private_virtual_interface_accepter` | 4 | 0 | 7 | 57% | 0% | good [~] |
| `aws_dx_hosted_public_virtual_interface` | 11 | 0 | 14 | 79% | 0% | good [~] |
| `aws_dx_hosted_public_virtual_interface_accepter` | 2 | 0 | 5 | 40% | 0% | good [~] |
| `aws_dx_hosted_transit_virtual_interface` | 13 | 0 | 15 | 87% | 0% | good [~] |
| `aws_dx_hosted_transit_virtual_interface_accepter` | 3 | 0 | 6 | 50% | 0% | good [~] |
| `aws_dx_lag` | 10 | 0 | 12 | 83% | 0% | good [~] |
| `aws_dx_macsec_key_association` | 7 | 0 | 6 | 100% | 0% | good [~] |
| `aws_dx_private_virtual_interface` | 16 | 0 | 19 | 84% | 0% | good [~] |
| `aws_dx_public_virtual_interface` | 11 | 0 | 15 | 73% | 0% | good [~] |
| `aws_dx_transit_virtual_interface` | 15 | 0 | 18 | 83% | 0% | good [~] |
| `aws_directory_service_conditional_forwarder` | 3 | 0 | 3 | 67% | 0% | good [~] |
| `aws_directory_service_directory` | 16 | 23 | 18 | 61% | 67% | excellent [+] |
| `aws_directory_service_log_subscription` | 3 | 0 | 2 | 100% | 0% | good [~] |
| `aws_directory_service_radius_settings` | 9 | 0 | 10 | 80% | 0% | good [~] |
| `aws_directory_service_region` | 5 | 0 | 7 | 57% | 0% | good [~] |
| `aws_directory_service_shared_directory` | 5 | 0 | 6 | 67% | 0% | good [~] |
| `aws_directory_service_shared_directory_accepter` | 6 | 0 | 6 | 83% | 0% | good [~] |
| `aws_directory_service_trust` | 13 | 0 | 13 | 92% | 0% | good [~] |
| `aws_dms_certificate` | 0 | 0 | 6 | 0% | 0% | poor [!] |
| `aws_dms_endpoint` | 14 | 20 | 28 | 25% | 64% | good [~] |
| `aws_dms_event_subscription` | 0 | 0 | 10 | 0% | 0% | poor [!] |
| `aws_dms_replication_config` | 0 | 0 | 14 | 0% | 0% | poor [!] |
| `aws_dms_replication_instance` | 12 | 14 | 21 | 43% | 52% | good [~] |
| `aws_dms_replication_subnet_group` | 0 | 0 | 7 | 0% | 0% | poor [!] |
| `aws_dms_replication_task` | 13 | 13 | 15 | 67% | 67% | excellent [+] |
| `aws_dms_s3_endpoint` | 0 | 0 | 54 | 0% | 0% | poor [!] |
| `aws_dsql_cluster` | 17 | 8 | 10 | 10% | 50% | good [~] |
| `aws_dynamodb_contributor_insights` | 3 | 3 | 3 | 67% | 67% | excellent [+] |
| `aws_dynamodb_global_table` | 4 | 5 | 4 | 75% | 75% | excellent [+] |
| `aws_dynamodb_kinesis_streaming_destination` | 4 | 4 | 3 | 100% | 100% | excellent [+] |
| `aws_dynamodb_resource_policy` | 2 | 4 | 4 | 25% | 75% | good [~] |
| `aws_dynamodb_table` | 10 | 26 | 29 | 24% | 72% | good [~] |
| `aws_dynamodb_table_export` | 6 | 0 | 18 | 28% | 0% | fair [-] |
| `aws_dynamodb_table_item` | 5 | 5 | 4 | 100% | 100% | excellent [+] |
| `aws_dynamodb_table_replica` | 5 | 0 | 9 | 44% | 0% | good [~] |
| `aws_dynamodb_tag` | 4 | 0 | 3 | 100% | 0% | good [~] |
| `aws_ebs_snapshot` | 8 | 12 | 16 | 6% | 50% | good [~] |
| `aws_ebs_volume` | 12 | 13 | 16 | 56% | 62% | good [~] |
| `aws_account_id` | 0 | 3 | 0 | 0% | 0% | n/a [?] |
| `aws_ami` | 18 | 13 | 32 | 53% | 38% | good [~] |
| `aws_ami_copy` | 10 | 0 | 37 | 24% | 0% | fair [-] |
| `aws_ami_from_instance` | 8 | 8 | 34 | 21% | 21% | fair [-] |
| `aws_ami_launch_permission` | 6 | 4 | 5 | 100% | 60% | excellent [+] |
| `aws_customer_gateway` | 6 | 7 | 9 | 56% | 67% | good [~] |
| `aws_default_network_acl` | 5 | 0 | 9 | 44% | 0% | good [~] |
| `aws_default_route_table` | 5 | 0 | 9 | 44% | 0% | good [~] |
| `aws_default_security_group` | 4 | 0 | 11 | 27% | 0% | fair [-] |
| `aws_default_subnet` | 6 | 0 | 24 | 21% | 0% | fair [-] |
| `aws_default_vpc` | 6 | 0 | 22 | 23% | 0% | fair [-] |
| `aws_default_vpc_dhcp_options` | 4 | 0 | 10 | 30% | 0% | fair [-] |
| `aws_ebs_default_kms_key` | 2 | 0 | 1 | 100% | 0% | good [~] |
| `aws_ebs_encryption_by_default` | 2 | 0 | 1 | 100% | 0% | good [~] |
| `aws_ebs_fast_snapshot_restore` | 3 | 4 | 4 | 50% | 75% | good [~] |
| `aws_ebs_snapshot_block_public_access` | 2 | 0 | 1 | 100% | 0% | good [~] |
| `aws_ebs_snapshot_copy` | 9 | 0 | 19 | 37% | 0% | fair [-] |
| `aws_ebs_snapshot_import` | 10 | 7 | 19 | 37% | 26% | fair [-] |
| `aws_ec2_availability_zone_group` | 3 | 0 | 2 | 100% | 0% | good [~] |
| `aws_ec2_capacity_block_reservation` | 15 | 15 | 18 | 50% | 50% | good [~] |
| `aws_ec2_capacity_reservation` | 19 | 18 | 17 | 94% | 94% | excellent [+] |
| `aws_ec2_carrier_gateway` | 4 | 3 | 5 | 60% | 40% | good [~] |
| `aws_ec2_client_vpn_authorization_rule` | 6 | 0 | 6 | 83% | 0% | good [~] |
| `aws_ec2_client_vpn_endpoint` | 13 | 12 | 22 | 55% | 50% | good [~] |
| `aws_ec2_client_vpn_network_association` | 5 | 0 | 5 | 80% | 0% | good [~] |
| `aws_ec2_client_vpn_route` | 6 | 0 | 7 | 71% | 0% | good [~] |
| `aws_ec2_default_credit_specification` | 3 | 0 | 3 | 67% | 0% | good [~] |
| `aws_ec2_fleet` | 8 | 4 | 20 | 35% | 15% | fair [-] |
| `aws_ec2_host` | 10 | 7 | 12 | 67% | 42% | good [~] |
| `aws_ec2_image_block_public_access` | 2 | 0 | 2 | 50% | 0% | good [~] |
| `aws_ec2_instance_connect_endpoint` | 11 | 13 | 13 | 69% | 92% | excellent [+] |
| `aws_ec2_instance_metadata_defaults` | 5 | 0 | 4 | 100% | 0% | good [~] |
| `aws_ec2_instance_state` | 4 | 0 | 4 | 75% | 0% | good [~] |
| `aws_ec2_local_gateway_route` | 9 | 7 | 3 | 100% | 100% | excellent [+] |
| `aws_ec2_local_gateway_route_table_vpc_association` | 7 | 9 | 5 | 100% | 100% | excellent [+] |
| `aws_ec2_managed_prefix_list` | 7 | 8 | 9 | 67% | 56% | excellent [+] |
| `aws_ec2_managed_prefix_list_entry` | 4 | 0 | 3 | 100% | 0% | good [~] |
| `aws_ec2_network_insights_analysis` | 7 | 10 | 15 | 40% | 53% | good [~] |
| `aws_ec2_network_insights_path` | 13 | 13 | 13 | 85% | 85% | excellent [+] |
| `aws_ec2_serial_console_access` | 2 | 0 | 1 | 100% | 0% | good [~] |
| `aws_ec2_subnet_cidr_reservation` | 5 | 0 | 5 | 80% | 0% | good [~] |
| `aws_ec2_tag` | 4 | 0 | 3 | 100% | 0% | good [~] |
| `aws_ec2_traffic_mirror_filter` | 4 | 5 | 5 | 60% | 80% | excellent [+] |
| `aws_ec2_traffic_mirror_filter_rule` | 11 | 13 | 11 | 73% | 91% | excellent [+] |
| `aws_ec2_traffic_mirror_session` | 11 | 11 | 11 | 91% | 91% | excellent [+] |
| `aws_ec2_traffic_mirror_target` | 8 | 9 | 8 | 88% | 88% | excellent [+] |
| `aws_ec2_transit_gateway` | 16 | 10 | 17 | 88% | 53% | excellent [+] |
| `aws_ec2_transit_gateway_connect` | 7 | 8 | 8 | 62% | 62% | excellent [+] |
| `aws_ec2_transit_gateway_connect_peer` | 10 | 9 | 11 | 73% | 64% | excellent [+] |
| `aws_ec2_transit_gateway_default_route_table_association` | 3 | 0 | 4 | 50% | 0% | good [~] |
| `aws_ec2_transit_gateway_default_route_table_propagation` | 3 | 0 | 4 | 50% | 0% | good [~] |
| `aws_ec2_transit_gateway_multicast_domain` | 10 | 11 | 9 | 89% | 89% | excellent [+] |
| `aws_ec2_transit_gateway_multicast_domain_association` | 4 | 4 | 4 | 75% | 75% | excellent [+] |
| `aws_ec2_transit_gateway_multicast_group_member` | 4 | 4 | 3 | 100% | 100% | excellent [+] |
| `aws_ec2_transit_gateway_multicast_group_source` | 4 | 4 | 3 | 100% | 100% | excellent [+] |
| `aws_ec2_transit_gateway_peering_attachment` | 7 | 8 | 9 | 67% | 78% | excellent [+] |
| `aws_ec2_transit_gateway_peering_attachment_accepter` | 2 | 0 | 7 | 14% | 0% | poor [!] |
| `aws_ec2_transit_gateway_policy_table` | 5 | 6 | 5 | 60% | 80% | excellent [+] |
| `aws_ec2_transit_gateway_policy_table_association` | 4 | 4 | 4 | 75% | 75% | excellent [+] |
| `aws_ec2_transit_gateway_prefix_list_reference` | 6 | 6 | 5 | 100% | 100% | excellent [+] |
| `aws_ec2_transit_gateway_route` | 5 | 5 | 4 | 100% | 100% | excellent [+] |
| `aws_ec2_transit_gateway_route_table` | 7 | 7 | 6 | 100% | 100% | excellent [+] |
| `aws_ec2_transit_gateway_route_table_association` | 5 | 0 | 5 | 80% | 0% | good [~] |
| `aws_ec2_transit_gateway_route_table_propagation` | 4 | 0 | 4 | 75% | 0% | good [~] |
| `aws_ec2_transit_gateway_vpc_attachment` | 12 | 7 | 13 | 85% | 46% | good [~] |
| `aws_ec2_transit_gateway_vpc_attachment_accepter` | 4 | 0 | 13 | 23% | 0% | fair [-] |
| `aws_egress_only_internet_gateway` | 4 | 4 | 3 | 100% | 100% | excellent [+] |
| `aws_eip` | 11 | 13 | 23 | 43% | 52% | good [~] |
| `aws_eip_association` | 7 | 7 | 6 | 100% | 100% | excellent [+] |
| `aws_eip_domain_name` | 4 | 4 | 4 | 75% | 75% | excellent [+] |
| `aws_flow_log` | 17 | 13 | 17 | 94% | 71% | excellent [+] |
| `aws_instance` | 20 | 22 | 59 | 32% | 36% | good [~] |
| `aws_internet_gateway` | 4 | 6 | 6 | 50% | 83% | good [~] |
| `aws_internet_gateway_attachment` | 3 | 3 | 3 | 67% | 67% | excellent [+] |
| `aws_key_pair` | 6 | 7 | 9 | 56% | 67% | good [~] |
| `aws_launch_template` | 12 | 7 | 40 | 28% | 15% | fair [-] |
| `aws_main_route_table_association` | 4 | 3 | 4 | 75% | 50% | excellent [+] |
| `aws_nat_gateway` | 7 | 11 | 13 | 46% | 54% | good [~] |
| `aws_network_acl` | 5 | 9 | 8 | 50% | 100% | good [~] |
| `aws_network_acl_association` | 3 | 3 | 2 | 100% | 100% | excellent [+] |
| `aws_network_acl_rule` | 12 | 0 | 11 | 100% | 0% | good [~] |
| `aws_network_interface` | 12 | 12 | 27 | 41% | 41% | good [~] |
| `aws_network_interface_attachment` | 6 | 6 | 5 | 100% | 100% | excellent [+] |
| `aws_network_interface_permission` | 4 | 4 | 5 | 60% | 60% | excellent [+] |
| `aws_network_interface_sg_attachment` | 3 | 3 | 3 | 67% | 67% | excellent [+] |
| `aws_placement_group` | 9 | 9 | 8 | 100% | 100% | excellent [+] |
| `aws_route` | 6 | 11 | 19 | 26% | 53% | good [~] |
| `aws_route_table` | 4 | 17 | 8 | 38% | 62% | good [~] |
| `aws_route_table_association` | 4 | 3 | 4 | 75% | 50% | excellent [+] |
| `aws_security_group` | 6 | 9 | 12 | 42% | 67% | good [~] |
| `aws_security_group_rule` | 9 | 0 | 13 | 62% | 0% | good [~] |
| `aws_service` | 0 | 21 | 0 | 0% | 0% | n/a [?] |
| `aws_spot_datafeed_subscription` | 3 | 0 | 2 | 100% | 0% | good [~] |
| `aws_spot_fleet_request` | 11 | 4 | 29 | 34% | 10% | fair [-] |
| `aws_spot_instance_request` | 10 | 6 | 67 | 13% | 7% | poor [!] |
| `aws_subnet` | 9 | 17 | 22 | 32% | 55% | good [~] |
| `aws_verifiedaccess_endpoint` | 16 | 17 | 20 | 60% | 55% | excellent [+] |
| `aws_verifiedaccess_group` | 11 | 10 | 12 | 67% | 75% | excellent [+] |
| `aws_verifiedaccess_instance` | 9 | 10 | 9 | 78% | 78% | excellent [+] |
| `aws_verifiedaccess_instance_logging_configuration` | 2 | 0 | 2 | 50% | 0% | good [~] |
| `aws_verifiedaccess_instance_trust_provider_attachment` | 3 | 0 | 2 | 100% | 0% | good [~] |
| `aws_verifiedaccess_trust_provider` | 10 | 10 | 12 | 58% | 58% | good [~] |
| `aws_volume_attachment` | 7 | 7 | 7 | 86% | 86% | excellent [+] |
| `aws_vpc` | 10 | 17 | 22 | 32% | 55% | good [~] |
| `aws_vpc_block_public_access_exclusion` | 6 | 0 | 7 | 71% | 0% | good [~] |
| `aws_vpc_block_public_access_options` | 2 | 0 | 4 | 25% | 0% | fair [-] |
| `aws_vpc_dhcp_options` | 8 | 11 | 10 | 70% | 100% | excellent [+] |
| `aws_vpc_dhcp_options_association` | 3 | 3 | 2 | 100% | 100% | excellent [+] |
| `aws_vpc_endpoint` | 10 | 15 | 26 | 35% | 54% | good [~] |
| `aws_vpc_endpoint_connection_accepter` | 4 | 4 | 3 | 100% | 100% | excellent [+] |
| `aws_vpc_endpoint_connection_notification` | 6 | 7 | 6 | 83% | 100% | excellent [+] |
| `aws_vpc_endpoint_policy` | 3 | 0 | 3 | 67% | 0% | good [~] |
| `aws_vpc_endpoint_private_dns` | 3 | 0 | 2 | 100% | 0% | good [~] |
| `aws_vpc_endpoint_route_table_association` | 3 | 0 | 2 | 100% | 0% | good [~] |
| `aws_vpc_endpoint_security_group_association` | 4 | 0 | 3 | 100% | 0% | good [~] |
| `aws_vpc_endpoint_service` | 8 | 12 | 18 | 33% | 56% | good [~] |
| `aws_vpc_endpoint_service_allowed_principal` | 3 | 0 | 2 | 100% | 0% | good [~] |
| `aws_vpc_endpoint_service_private_dns_verification` | 3 | 0 | 3 | 67% | 0% | good [~] |
| `aws_vpc_endpoint_subnet_association` | 3 | 0 | 3 | 67% | 0% | good [~] |
| `aws_vpc_ipam` | 14 | 18 | 14 | 79% | 86% | excellent [+] |
| `aws_vpc_ipam_organization_admin_account` | 1 | 0 | 5 | 20% | 0% | fair [-] |
| `aws_vpc_ipam_pool` | 22 | 22 | 21 | 76% | 90% | excellent [+] |
| `aws_vpc_ipam_pool_cidr` | 5 | 6 | 6 | 67% | 67% | excellent [+] |
| `aws_vpc_ipam_pool_cidr_allocation` | 8 | 8 | 9 | 67% | 67% | excellent [+] |
| `aws_vpc_ipam_preview_next_cidr` | 2 | 0 | 4 | 50% | 0% | good [~] |
| `aws_vpc_ipam_resource_discovery` | 8 | 10 | 9 | 78% | 89% | excellent [+] |
| `aws_vpc_ipam_resource_discovery_association` | 5 | 0 | 11 | 36% | 0% | fair [-] |
| `aws_vpc_ipam_scope` | 9 | 10 | 10 | 70% | 80% | excellent [+] |
| `aws_vpc_ipv4_cidr_block_association` | 5 | 0 | 5 | 80% | 0% | good [~] |
| `aws_vpc_ipv6_cidr_block_association` | 5 | 0 | 9 | 44% | 0% | good [~] |
| `aws_vpc_network_performance_metric_subscription` | 6 | 0 | 5 | 100% | 0% | good [~] |
| `aws_vpc_peering_connection` | 8 | 5 | 11 | 64% | 36% | good [~] |
| `aws_vpc_peering_connection_accepter` | 3 | 0 | 12 | 17% | 0% | poor [!] |
| `aws_vpc_peering_connection_options` | 2 | 0 | 3 | 33% | 0% | fair [-] |
| `aws_vpc_route_server` | 8 | 0 | 10 | 70% | 0% | good [~] |
| `aws_vpc_route_server_endpoint` | 5 | 0 | 10 | 40% | 0% | good [~] |
| `aws_vpc_route_server_peer` | 5 | 0 | 13 | 31% | 0% | fair [-] |
| `aws_vpc_route_server_propagation` | 3 | 0 | 3 | 67% | 0% | good [~] |
| `aws_vpc_route_server_vpc_association` | 3 | 0 | 3 | 67% | 0% | good [~] |
| `aws_vpc_security_group_egress_rule` | 0 | 0 | 13 | 0% | 0% | poor [!] |
| `aws_vpc_security_group_ingress_rule` | 11 | 0 | 13 | 77% | 0% | good [~] |
| `aws_vpc_security_group_vpc_association` | 4 | 4 | 4 | 75% | 75% | excellent [+] |
| `aws_vpn_connection` | 15 | 15 | 74 | 19% | 18% | fair [-] |
| `aws_vpn_connection_route` | 3 | 3 | 2 | 100% | 100% | excellent [+] |
| `aws_vpn_gateway` | 6 | 6 | 6 | 83% | 83% | excellent [+] |
| `aws_vpn_gateway_attachment` | 3 | 3 | 2 | 100% | 100% | excellent [+] |
| `aws_vpn_gateway_route_propagation` | 3 | 0 | 3 | 67% | 0% | good [~] |
| `aws_ec2_instance_connect_endpoint` | 7 | 14 | 13 | 8% | 77% | good [~] |
| `aws_ecr_account_setting` | 3 | 3 | 2 | 100% | 100% | excellent [+] |
| `aws_ecr_lifecycle_policy` | 4 | 4 | 3 | 100% | 100% | excellent [+] |
| `aws_ecr_pull_through_cache_rule` | 7 | 7 | 6 | 100% | 100% | excellent [+] |
| `aws_ecr_registry_policy` | 3 | 3 | 2 | 100% | 100% | excellent [+] |
| `aws_ecr_registry_scanning_configuration` | 3 | 8 | 3 | 67% | 100% | excellent [+] |
| `aws_ecr_replication_configuration` | 2 | 9 | 2 | 0% | 100% | good [~] |
| `aws_ecr_repository` | 8 | 11 | 11 | 64% | 73% | excellent [+] |
| `aws_ecr_repository_creation_template` | 7 | 8 | 10 | 60% | 70% | excellent [+] |
| `aws_ecr_repository_policy` | 4 | 4 | 3 | 100% | 100% | excellent [+] |
| `aws_ecs_account_setting_default` | 0 | 0 | 3 | 0% | 0% | poor [!] |
| `aws_ecs_capacity_provider` | 0 | 0 | 5 | 0% | 0% | poor [!] |
| `aws_ecs_cluster` | 21 | 6 | 7 | 71% | 57% | excellent [+] |
| `aws_ecs_cluster_capacity_providers` | 0 | 0 | 3 | 0% | 0% | poor [!] |
| `aws_ecs_service` | 24 | 27 | 34 | 62% | 50% | excellent [+] |
| `aws_ecs_tag` | 0 | 0 | 3 | 0% | 0% | poor [!] |
| `aws_ecs_task_definition` | 16 | 14 | 24 | 62% | 50% | excellent [+] |
| `aws_ecs_task_set` | 0 | 0 | 20 | 0% | 0% | poor [!] |
| `aws_efs_access_point` | 0 | 0 | 8 | 0% | 0% | poor [!] |
| `aws_efs_backup_policy` | 0 | 0 | 2 | 0% | 0% | poor [!] |
| `aws_efs_file_system` | 10 | 20 | 18 | 50% | 61% | good [~] |
| `aws_efs_file_system_policy` | 0 | 0 | 3 | 0% | 0% | poor [!] |
| `aws_efs_mount_target` | 0 | 0 | 12 | 0% | 0% | poor [!] |
| `aws_efs_replication_configuration` | 0 | 0 | 7 | 0% | 0% | poor [!] |
| `aws_eks_access_entry` | 10 | 11 | 11 | 82% | 91% | excellent [+] |
| `aws_eks_access_policy_association` | 7 | 9 | 7 | 86% | 86% | excellent [+] |
| `aws_eks_addon` | 14 | 11 | 16 | 81% | 56% | excellent [+] |
| `aws_eks_cluster` | 28 | 19 | 27 | 48% | 52% | good [~] |
| `aws_eks_fargate_profile` | 10 | 12 | 10 | 90% | 90% | excellent [+] |
| `aws_eks_identity_provider_config` | 7 | 10 | 7 | 86% | 86% | excellent [+] |
| `aws_eks_node_group` | 16 | 17 | 25 | 60% | 52% | excellent [+] |
| `aws_eks_pod_identity_association` | 9 | 9 | 8 | 100% | 100% | excellent [+] |
| `aws_elasticache_cluster` | 23 | 22 | 36 | 61% | 56% | excellent [+] |
| `aws_elasticache_global_replication_group` | 16 | 0 | 18 | 83% | 0% | good [~] |
| `aws_elasticache_parameter_group` | 8 | 7 | 7 | 100% | 86% | excellent [+] |
| `aws_elasticache_replication_group` | 31 | 28 | 48 | 60% | 52% | excellent [+] |
| `aws_elasticache_reserved_cache_node` | 8 | 0 | 15 | 40% | 0% | good [~] |
| `aws_elasticache_serverless_cache` | 16 | 0 | 21 | 71% | 0% | good [~] |
| `aws_elasticache_subnet_group` | 7 | 6 | 7 | 86% | 71% | excellent [+] |
| `aws_elasticache_user` | 12 | 10 | 11 | 64% | 73% | excellent [+] |
| `aws_elasticache_user_group` | 6 | 0 | 6 | 83% | 0% | good [~] |
| `aws_elasticache_user_group_association` | 3 | 0 | 3 | 67% | 0% | good [~] |
| `aws_elastic_beanstalk_application` | 9 | 8 | 6 | 100% | 83% | excellent [+] |
| `aws_elastic_beanstalk_environment` | 18 | 23 | 24 | 58% | 62% | good [~] |
| `aws_elb` | 13 | 27 | 23 | 52% | 70% | good [~] |
| `aws_alb` | 0 | 0 | 36 | 0% | 0% | poor [!] |
| `aws_alb_listener` | 0 | 0 | 32 | 0% | 0% | poor [!] |
| `aws_alb_listener_certificate` | 0 | 0 | 2 | 0% | 0% | poor [!] |
| `aws_alb_listener_rule` | 0 | 0 | 7 | 0% | 0% | poor [!] |
| `aws_alb_target_group` | 0 | 0 | 27 | 0% | 0% | poor [!] |
| `aws_alb_target_group_attachment` | 0 | 0 | 4 | 0% | 0% | poor [!] |
| `aws_lb` | 28 | 31 | 36 | 72% | 61% | excellent [+] |
| `aws_lb_cookie_stickiness_policy` | 5 | 0 | 4 | 100% | 0% | good [~] |
| `aws_lb_listener` | 22 | 20 | 32 | 66% | 50% | excellent [+] |
| `aws_lb_listener_certificate` | 3 | 3 | 2 | 100% | 100% | excellent [+] |
| `aws_lb_listener_rule` | 8 | 12 | 7 | 100% | 100% | excellent [+] |
| `aws_lb_ssl_negotiation_policy` | 4 | 0 | 5 | 60% | 0% | good [~] |
| `aws_lb_target_group` | 21 | 27 | 27 | 74% | 63% | excellent [+] |
| `aws_lb_target_group_attachment` | 5 | 5 | 4 | 100% | 100% | excellent [+] |
| `aws_lb_trust_store` | 8 | 8 | 10 | 70% | 40% | good [~] |
| `aws_lb_trust_store_revocation` | 5 | 3 | 6 | 67% | 33% | good [~] |
| `aws_emr_block_public_access_configuration` | 3 | 5 | 2 | 100% | 100% | excellent [+] |
| `aws_emr_cluster` | 34 | 39 | 35 | 54% | 69% | good [~] |
| `aws_emr_instance_fleet` | 7 | 10 | 8 | 75% | 100% | excellent [+] |
| `aws_emr_instance_group` | 10 | 10 | 11 | 82% | 82% | excellent [+] |
| `aws_emr_managed_scaling_policy` | 5 | 10 | 2 | 100% | 100% | excellent [+] |
| `aws_emr_security_configuration` | 2 | 4 | 4 | 25% | 75% | good [~] |
| `aws_emr_studio` | 18 | 15 | 17 | 100% | 82% | excellent [+] |
| `aws_emr_studio_session_mapping` | 6 | 6 | 5 | 100% | 100% | excellent [+] |
| `aws_emrcontainers_virtual_cluster` | 5 | 8 | 6 | 67% | 67% | excellent [+] |
| `aws_emrserverless_application` | 15 | 21 | 14 | 79% | 50% | excellent [+] |
| `aws_cloudwatch_event_api_destination` | 8 | 8 | 7 | 100% | 100% | excellent [+] |
| `aws_cloudwatch_event_archive` | 7 | 7 | 6 | 100% | 100% | excellent [+] |
| `aws_cloudwatch_event_bus` | 8 | 7 | 8 | 75% | 62% | excellent [+] |
| `aws_cloudwatch_event_bus_policy` | 3 | 3 | 2 | 100% | 100% | excellent [+] |
| `aws_cloudwatch_event_connection` | 6 | 6 | 8 | 62% | 62% | excellent [+] |
| `aws_cloudwatch_event_endpoint` | 10 | 11 | 8 | 100% | 100% | excellent [+] |
| `aws_cloudwatch_event_permission` | 5 | 6 | 5 | 80% | 80% | excellent [+] |
| `aws_cloudwatch_event_rule` | 10 | 9 | 13 | 69% | 62% | excellent [+] |
| `aws_cloudwatch_event_target` | 9 | 11 | 20 | 35% | 50% | good [~] |
| `aws_kinesis_firehose_delivery_stream` | 6 | 12 | 20 | 25% | 50% | good [~] |
| `aws_fis_experiment_template` | 8 | 14 | 11 | 55% | 55% | good [~] |
| `aws_fsx_backup` | 9 | 5 | 9 | 89% | 44% | good [~] |
| `aws_fsx_data_repository_association` | 8 | 0 | 12 | 67% | 0% | good [~] |
| `aws_fsx_file_cache` | 12 | 0 | 19 | 53% | 0% | good [~] |
| `aws_fsx_lustre_file_system` | 21 | 28 | 36 | 44% | 61% | good [~] |
| `aws_fsx_ontap_file_system` | 25 | 25 | 26 | 81% | 77% | excellent [+] |
| `aws_fsx_ontap_storage_virtual_machine` | 8 | 0 | 12 | 58% | 0% | good [~] |
| `aws_fsx_ontap_volume` | 16 | 0 | 25 | 64% | 0% | good [~] |
| `aws_fsx_openzfs_file_system` | 25 | 26 | 31 | 68% | 68% | excellent [+] |
| `aws_fsx_openzfs_snapshot` | 4 | 0 | 7 | 57% | 0% | good [~] |
| `aws_fsx_openzfs_volume` | 11 | 0 | 17 | 65% | 0% | good [~] |
| `aws_fsx_windows_file_system` | 25 | 26 | 30 | 70% | 70% | excellent [+] |
| `aws_glacier_vault` | 4 | 7 | 7 | 43% | 57% | good [~] |
| `aws_glue_catalog_database` | 6 | 7 | 11 | 45% | 55% | good [~] |
| `aws_glue_catalog_table` | 11 | 13 | 16 | 62% | 69% | excellent [+] |
| `aws_glue_catalog_table_optimizer` | 0 | 0 | 5 | 0% | 0% | poor [!] |
| `aws_glue_classifier` | 0 | 0 | 5 | 0% | 0% | poor [!] |
| `aws_glue_connection` | 7 | 10 | 11 | 55% | 82% | good [~] |
| `aws_glue_crawler` | 8 | 22 | 24 | 29% | 54% | good [~] |
| `aws_glue_data_catalog_encryption_settings` | 3 | 5 | 2 | 100% | 100% | excellent [+] |
| `aws_glue_data_quality_ruleset` | 0 | 0 | 10 | 0% | 0% | poor [!] |
| `aws_glue_dev_endpoint` | 14 | 19 | 25 | 52% | 64% | good [~] |
| `aws_glue_job` | 12 | 13 | 23 | 48% | 52% | good [~] |
| `aws_glue_ml_transform` | 13 | 14 | 16 | 69% | 81% | excellent [+] |
| `aws_glue_partition` | 8 | 10 | 9 | 67% | 89% | excellent [+] |
| `aws_glue_partition_index` | 0 | 0 | 5 | 0% | 0% | poor [!] |
| `aws_glue_registry` | 4 | 6 | 5 | 60% | 100% | excellent [+] |
| `aws_glue_resource_policy` | 2 | 5 | 2 | 50% | 50% | good [~] |
| `aws_glue_schema` | 7 | 14 | 13 | 46% | 92% | good [~] |
| `aws_glue_security_configuration` | 3 | 4 | 2 | 100% | 100% | excellent [+] |
| `aws_glue_trigger` | 9 | 11 | 15 | 53% | 67% | good [~] |
| `aws_glue_user_defined_function` | 0 | 0 | 9 | 0% | 0% | poor [!] |
| `aws_glue_workflow` | 5 | 7 | 7 | 57% | 86% | good [~] |
| `aws_guardduty_detector` | 7 | 7 | 7 | 71% | 57% | excellent [+] |
| `aws_guardduty_detector_feature` | 4 | 4 | 4 | 75% | 75% | excellent [+] |
| `aws_guardduty_filter` | 8 | 16 | 9 | 78% | 78% | excellent [+] |
| `aws_guardduty_invite_accepter` | 2 | 3 | 3 | 0% | 67% | good [~] |
| `aws_guardduty_ipset` | 7 | 8 | 8 | 75% | 88% | excellent [+] |
| `aws_guardduty_malware_protection_plan` | 6 | 12 | 8 | 62% | 88% | excellent [+] |
| `aws_guardduty_member` | 14 | 4 | 8 | 62% | 50% | excellent [+] |
| `aws_guardduty_member_detector_feature` | 5 | 5 | 5 | 80% | 80% | excellent [+] |
| `aws_guardduty_organization_admin_account` | 2 | 2 | 1 | 100% | 100% | excellent [+] |
| `aws_guardduty_organization_configuration` | 2 | 4 | 4 | 25% | 75% | good [~] |
| `aws_guardduty_organization_configuration_feature` | 4 | 0 | 4 | 75% | 0% | good [~] |
| `aws_guardduty_publishing_destination` | 5 | 5 | 4 | 100% | 100% | excellent [+] |
| `aws_guardduty_threatintelset` | 7 | 8 | 8 | 75% | 88% | excellent [+] |
| `aws_iam_access_key` | 2 | 4 | 9 | 22% | 33% | good [~] |
| `aws_iam_account_alias` | 1 | 2 | 1 | 100% | 100% | excellent [+] |
| `aws_iam_account_password_policy` | 10 | 11 | 10 | 100% | 100% | excellent [+] |
| `aws_iam_group` | 4 | 5 | 4 | 100% | 100% | excellent [+] |
| `aws_iam_group_membership` | 3 | 4 | 3 | 100% | 100% | excellent [+] |
| `aws_iam_group_policies_exclusive` | 1 | 3 | 2 | 50% | 100% | good [~] |
| `aws_iam_group_policy` | 4 | 4 | 4 | 100% | 75% | excellent [+] |
| `aws_iam_group_policy_attachment` | 2 | 3 | 2 | 100% | 100% | excellent [+] |
| `aws_iam_group_policy_attachments_exclusive` | 1 | 3 | 2 | 50% | 100% | good [~] |
| `aws_iam_instance_profile` | 7 | 6 | 9 | 78% | 56% | excellent [+] |
| `aws_iam_openid_connect_provider` | 6 | 7 | 6 | 100% | 100% | excellent [+] |
| `aws_iam_policy` | 9 | 7 | 10 | 80% | 60% | excellent [+] |
| `aws_iam_policy_attachment` | 2 | 0 | 5 | 40% | 0% | good [~] |
| `aws_iam_role` | 10 | 10 | 15 | 67% | 60% | excellent [+] |
| `aws_iam_role_policies_exclusive` | 1 | 3 | 2 | 50% | 100% | good [~] |
| `aws_iam_role_policy` | 4 | 4 | 4 | 100% | 75% | excellent [+] |
| `aws_iam_role_policy_attachment` | 2 | 3 | 2 | 100% | 100% | excellent [+] |
| `aws_iam_role_policy_attachments_exclusive` | 1 | 3 | 2 | 50% | 100% | good [~] |
| `aws_iam_saml_provider` | 5 | 7 | 6 | 83% | 100% | excellent [+] |
| `aws_iam_server_certificate` | 9 | 10 | 12 | 75% | 75% | excellent [+] |
| `aws_iam_service_linked_role` | 5 | 9 | 10 | 50% | 80% | good [~] |
| `aws_iam_service_specific_credential` | 4 | 7 | 6 | 67% | 100% | excellent [+] |
| `aws_iam_signing_certificate` | 4 | 5 | 4 | 100% | 100% | excellent [+] |
| `aws_iam_user` | 6 | 5 | 8 | 75% | 50% | excellent [+] |
| `aws_iam_user_group_membership` | 2 | 3 | 2 | 100% | 100% | excellent [+] |
| `aws_iam_user_login_profile` | 3 | 4 | 7 | 43% | 43% | good [~] |
| `aws_iam_user_policies_exclusive` | 1 | 3 | 2 | 50% | 100% | good [~] |
| `aws_iam_user_policy` | 4 | 4 | 4 | 100% | 75% | excellent [+] |
| `aws_iam_user_policy_attachment` | 2 | 3 | 2 | 100% | 100% | excellent [+] |
| `aws_iam_user_policy_attachments_exclusive` | 1 | 3 | 2 | 50% | 100% | good [~] |
| `aws_iam_user_ssh_key` | 5 | 7 | 6 | 83% | 100% | excellent [+] |
| `aws_iam_virtual_mfa_device` | 5 | 5 | 9 | 56% | 44% | good [~] |
| `aws_service_name` | 0 | 8 | 0 | 0% | 0% | n/a [?] |
| `aws_identitystore_group` | 4 | 6 | 5 | 0% | 100% | good [~] |
| `aws_identitystore_user` | 11 | 21 | 16 | 31% | 94% | good [~] |
| `aws_inspector2_delegated_admin_account` | 2 | 2 | 3 | 33% | 33% | good [~] |
| `aws_inspector2_enabler` | 3 | 3 | 3 | 67% | 67% | excellent [+] |
| `aws_inspector2_filter` | 7 | 6 | 8 | 50% | 62% | good [~] |
| `aws_inspector2_member_association` | 2 | 3 | 5 | 20% | 40% | good [~] |
| `aws_inspector2_organization_configuration` | 2 | 7 | 3 | 33% | 33% | good [~] |
| `aws_iot_authorizer` | 8 | 0 | 10 | 70% | 0% | good [~] |
| `aws_iot_billing_group` | 3 | 3 | 7 | 29% | 29% | fair [-] |
| `aws_iot_ca_certificate` | 6 | 6 | 12 | 42% | 42% | good [~] |
| `aws_iot_certificate` | 10 | 5 | 8 | 25% | 50% | good [~] |
| `aws_iot_domain_configuration` | 6 | 6 | 14 | 36% | 36% | good [~] |
| `aws_iot_event_configurations` | 1 | 0 | 1 | 0% | 0% | poor [!] |
| `aws_iot_indexing_configuration` | 3 | 3 | 2 | 100% | 100% | excellent [+] |
| `aws_iot_logging_options` | 4 | 0 | 3 | 100% | 0% | good [~] |
| `aws_iot_policy` | 5 | 5 | 7 | 57% | 57% | good [~] |
| `aws_iot_policy_attachment` | 3 | 3 | 2 | 100% | 100% | excellent [+] |
| `aws_iot_provisioning_template` | 8 | 0 | 11 | 64% | 0% | good [~] |
| `aws_iot_role_alias` | 5 | 5 | 6 | 67% | 67% | excellent [+] |
| `aws_iot_thing` | 7 | 7 | 6 | 100% | 100% | excellent [+] |
| `aws_iot_thing_group` | 6 | 5 | 8 | 62% | 50% | excellent [+] |
| `aws_iot_thing_group_membership` | 4 | 3 | 3 | 100% | 67% | excellent [+] |
| `aws_iot_thing_principal_attachment` | 3 | 3 | 2 | 100% | 100% | excellent [+] |
| `aws_iot_thing_type` | 5 | 7 | 6 | 67% | 67% | excellent [+] |
| `aws_iot_topic_rule` | 10 | 7 | 28 | 25% | 21% | fair [-] |
| `aws_iot_topic_rule_destination` | 3 | 0 | 4 | 50% | 0% | good [~] |
| `aws_ivs_channel` | 7 | 7 | 11 | 55% | 55% | good [~] |
| `aws_msk_cluster` | 24 | 27 | 29 | 28% | 69% | good [~] |
| `aws_msk_cluster_policy` | 3 | 0 | 3 | 67% | 0% | good [~] |
| `aws_msk_configuration` | 6 | 0 | 6 | 83% | 0% | good [~] |
| `aws_msk_replicator` | 6 | 0 | 10 | 50% | 0% | good [~] |
| `aws_msk_scram_secret_association` | 2 | 0 | 2 | 50% | 0% | good [~] |
| `aws_msk_serverless_cluster` | 6 | 11 | 9 | 56% | 89% | good [~] |
| `aws_msk_single_scram_secret_association` | 3 | 0 | 2 | 100% | 0% | good [~] |
| `aws_msk_vpc_connection` | 6 | 0 | 8 | 62% | 0% | good [~] |
| `aws_keyspaces_keyspace` | 4 | 4 | 6 | 50% | 67% | good [~] |
| `aws_keyspaces_table` | 6 | 3 | 14 | 29% | 21% | fair [-] |
| `aws_kinesis_resource_policy` | 0 | 0 | 2 | 0% | 0% | poor [!] |
| `aws_kinesis_stream` | 11 | 8 | 12 | 33% | 58% | good [~] |
| `aws_kinesis_stream_consumer` | 0 | 0 | 4 | 0% | 0% | poor [!] |
| `aws_kinesis_analytics_application` | 0 | 0 | 15 | 0% | 0% | poor [!] |
| `aws_kinesisanalyticsv2_application` | 7 | 15 | 17 | 24% | 82% | good [~] |
| `aws_kinesis_video_stream` | 11 | 11 | 11 | 82% | 82% | excellent [+] |
| `aws_kms_alias` | 4 | 5 | 5 | 60% | 80% | excellent [+] |
| `aws_kms_ciphertext` | 4 | 0 | 4 | 75% | 0% | good [~] |
| `aws_kms_custom_key_store` | 6 | 7 | 11 | 36% | 36% | good [~] |
| `aws_kms_external_key` | 9 | 11 | 14 | 50% | 64% | good [~] |
| `aws_kms_grant` | 9 | 11 | 10 | 80% | 80% | excellent [+] |
| `aws_kms_key` | 14 | 11 | 17 | 71% | 53% | excellent [+] |
| `aws_kms_key_policy` | 3 | 3 | 3 | 67% | 67% | excellent [+] |
| `aws_kms_replica_external_key` | 9 | 11 | 15 | 53% | 67% | good [~] |
| `aws_kms_replica_key` | 9 | 12 | 13 | 62% | 77% | excellent [+] |
| `aws_lakeformation_data_cells_filter` | 1 | 0 | 2 | 0% | 0% | poor [!] |
| `aws_lakeformation_data_lake_settings` | 6 | 10 | 11 | 45% | 55% | good [~] |
| `aws_lakeformation_lf_tag` | 3 | 4 | 3 | 67% | 100% | excellent [+] |
| `aws_lakeformation_opt_in` | 1 | 0 | 5 | 0% | 0% | poor [!] |
| `aws_lakeformation_permissions` | 4 | 5 | 12 | 25% | 33% | good [~] |
| `aws_lakeformation_resource` | 4 | 4 | 6 | 50% | 50% | good [~] |
| `aws_lakeformation_resource_lf_tag` | 4 | 4 | 6 | 17% | 17% | fair [-] |
| `aws_lakeformation_resource_lf_tags` | 3 | 5 | 6 | 33% | 33% | good [~] |
| `aws_lambda_alias` | 6 | 6 | 7 | 71% | 71% | excellent [+] |
| `aws_lambda_code_signing_config` | 9 | 11 | 8 | 88% | 100% | excellent [+] |
| `aws_lambda_event_source_mapping` | 28 | 19 | 35 | 74% | 51% | excellent [+] |
| `aws_lambda_function` | 31 | 27 | 45 | 67% | 51% | excellent [+] |
| `aws_lambda_function_event_invoke_config` | 6 | 8 | 5 | 100% | 80% | excellent [+] |
| `aws_lambda_function_recursion_config` | 3 | 3 | 2 | 100% | 100% | excellent [+] |
| `aws_lambda_function_url` | 9 | 10 | 9 | 89% | 78% | excellent [+] |
| `aws_lambda_invocation` | 0 | 0 | 7 | 0% | 0% | poor [!] |
| `aws_lambda_layer_version` | 16 | 15 | 19 | 79% | 74% | excellent [+] |
| `aws_lambda_layer_version_permission` | 7 | 8 | 9 | 67% | 78% | excellent [+] |
| `aws_lambda_permission` | 8 | 7 | 11 | 64% | 55% | excellent [+] |
| `aws_lambda_provisioned_concurrency_config` | 4 | 6 | 5 | 60% | 60% | excellent [+] |
| `aws_lambda_runtime_management_config` | 5 | 4 | 5 | 80% | 60% | excellent [+] |
| `aws_lexv2models_bot` | 6 | 13 | 12 | 42% | 92% | good [~] |
| `aws_lexv2models_bot_locale` | 6 | 7 | 8 | 62% | 75% | excellent [+] |
| `aws_lexv2models_bot_version` | 3 | 4 | 5 | 40% | 60% | good [~] |
| `aws_lexv2models_intent` | 7 | 7 | 20 | 30% | 30% | good [~] |
| `aws_lexv2models_slot` | 9 | 9 | 13 | 62% | 62% | excellent [+] |
| `aws_lexv2models_slot_type` | 8 | 8 | 12 | 58% | 58% | good [~] |
| `aws_cloudwatch_log_account_policy` | 0 | 0 | 5 | 0% | 0% | poor [!] |
| `aws_cloudwatch_log_anomaly_detector` | 0 | 0 | 10 | 0% | 0% | poor [!] |
| `aws_cloudwatch_log_data_protection_policy` | 0 | 0 | 2 | 0% | 0% | poor [!] |
| `aws_cloudwatch_log_delivery` | 0 | 0 | 8 | 0% | 0% | poor [!] |
| `aws_cloudwatch_log_delivery_destination` | 0 | 0 | 7 | 0% | 0% | poor [!] |
| `aws_cloudwatch_log_delivery_destination_policy` | 0 | 0 | 2 | 0% | 0% | poor [!] |
| `aws_cloudwatch_log_delivery_source` | 0 | 0 | 7 | 0% | 0% | poor [!] |
| `aws_cloudwatch_log_destination` | 0 | 0 | 6 | 0% | 0% | poor [!] |
| `aws_cloudwatch_log_destination_policy` | 0 | 0 | 3 | 0% | 0% | poor [!] |
| `aws_cloudwatch_log_group` | 6 | 6 | 9 | 56% | 56% | good [~] |
| `aws_cloudwatch_log_index_policy` | 0 | 0 | 2 | 0% | 0% | poor [!] |
| `aws_cloudwatch_log_metric_filter` | 0 | 0 | 4 | 0% | 0% | poor [!] |
| `aws_cloudwatch_log_resource_policy` | 0 | 0 | 2 | 0% | 0% | poor [!] |
| `aws_cloudwatch_log_stream` | 4 | 4 | 3 | 100% | 100% | excellent [+] |
| `aws_cloudwatch_log_subscription_filter` | 0 | 0 | 6 | 0% | 0% | poor [!] |
| `aws_macie2_account` | 5 | 6 | 5 | 0% | 100% | good [~] |
| `aws_macie2_classification_export_configuration` | 1 | 2 | 1 | 0% | 100% | good [~] |
| `aws_macie2_classification_job` | 20 | 19 | 17 | 82% | 76% | excellent [+] |
| `aws_macie2_custom_data_identifier` | 13 | 13 | 12 | 92% | 75% | excellent [+] |
| `aws_macie2_findings_filter` | 10 | 8 | 10 | 90% | 70% | excellent [+] |
| `aws_macie2_invitation_accepter` | 3 | 3 | 3 | 67% | 67% | excellent [+] |
| `aws_macie2_member` | 14 | 10 | 15 | 87% | 60% | excellent [+] |
| `aws_macie2_organization_admin_account` | 2 | 2 | 1 | 100% | 100% | excellent [+] |
| `aws_macie2_organization_configuration` | 2 | 2 | 1 | 100% | 100% | excellent [+] |
| `aws_medialive_channel` | 16 | 17 | 17 | 82% | 82% | excellent [+] |
| `aws_medialive_input` | 16 | 17 | 17 | 76% | 76% | excellent [+] |
| `aws_medialive_input_security_group` | 0 | 0 | 6 | 0% | 0% | poor [!] |
| `aws_medialive_multiplex` | 0 | 0 | 8 | 0% | 0% | poor [!] |
| `aws_medialive_multiplex_program` | 0 | 0 | 4 | 0% | 0% | poor [!] |
| `aws_media_package_channel` | 7 | 6 | 6 | 33% | 67% | good [~] |
| `aws_media_packagev2_channel_group` | 9 | 9 | 6 | 83% | 83% | excellent [+] |
| `aws_media_store_container` | 5 | 5 | 5 | 80% | 80% | excellent [+] |
| `aws_memorydb_acl` | 7 | 7 | 7 | 71% | 71% | excellent [+] |
| `aws_memorydb_cluster` | 18 | 21 | 32 | 53% | 56% | good [~] |
| `aws_memorydb_multi_region_cluster` | 11 | 0 | 15 | 73% | 0% | good [~] |
| `aws_memorydb_parameter_group` | 4 | 0 | 8 | 50% | 0% | good [~] |
| `aws_memorydb_snapshot` | 6 | 7 | 10 | 50% | 60% | good [~] |
| `aws_memorydb_subnet_group` | 6 | 6 | 8 | 62% | 62% | excellent [+] |
| `aws_memorydb_user` | 6 | 0 | 7 | 29% | 0% | fair [-] |
| `aws_mq_broker` | 19 | 24 | 26 | 69% | 65% | excellent [+] |
| `aws_mq_configuration` | 10 | 11 | 10 | 90% | 100% | excellent [+] |
| `aws_neptune_cluster` | 24 | 36 | 38 | 53% | 63% | good [~] |
| `aws_neptune_cluster_endpoint` | 8 | 8 | 9 | 78% | 78% | excellent [+] |
| `aws_neptune_cluster_instance` | 19 | 21 | 28 | 61% | 61% | excellent [+] |
| `aws_neptune_cluster_parameter_group` | 8 | 9 | 8 | 88% | 88% | excellent [+] |
| `aws_neptune_cluster_snapshot` | 16 | 16 | 16 | 81% | 81% | excellent [+] |
| `aws_neptune_event_subscription` | 8 | 10 | 12 | 58% | 67% | good [~] |
| `aws_neptune_global_cluster` | 9 | 10 | 11 | 64% | 73% | excellent [+] |
| `aws_neptune_parameter_group` | 8 | 9 | 8 | 88% | 88% | excellent [+] |
| `aws_neptune_subnet_group` | 8 | 9 | 7 | 86% | 86% | excellent [+] |
| `aws_networkfirewall_firewall` | 11 | 14 | 16 | 62% | 75% | excellent [+] |
| `aws_networkfirewall_firewall_policy` | 5 | 8 | 8 | 50% | 88% | good [~] |
| `aws_networkfirewall_logging_configuration` | 0 | 0 | 2 | 0% | 0% | poor [!] |
| `aws_networkfirewall_resource_policy` | 0 | 0 | 2 | 0% | 0% | poor [!] |
| `aws_networkfirewall_rule_group` | 8 | 10 | 11 | 64% | 82% | excellent [+] |
| `aws_networkfirewall_tls_inspection_configuration` | 0 | 0 | 13 | 0% | 0% | poor [!] |
| `aws_location` | 0 | 13 | 0 | 0% | 0% | n/a [?] |
| `aws_networkmanager_attachment_accepter` | 4 | 0 | 12 | 25% | 0% | fair [-] |
| `aws_networkmanager_connect_attachment` | 11 | 9 | 16 | 62% | 50% | excellent [+] |
| `aws_networkmanager_connect_peer` | 11 | 10 | 16 | 62% | 56% | excellent [+] |
| `aws_networkmanager_connection` | 4 | 11 | 10 | 10% | 80% | good [~] |
| `aws_networkmanager_core_network` | 7 | 7 | 14 | 43% | 43% | good [~] |
| `aws_networkmanager_core_network_policy_attachment` | 4 | 0 | 4 | 75% | 0% | good [~] |
| `aws_networkmanager_customer_gateway_association` | 6 | 5 | 5 | 80% | 80% | excellent [+] |
| `aws_networkmanager_device` | 14 | 14 | 13 | 85% | 85% | excellent [+] |
| `aws_networkmanager_dx_gateway_attachment` | 10 | 9 | 13 | 69% | 62% | excellent [+] |
| `aws_networkmanager_global_network` | 6 | 6 | 5 | 60% | 60% | excellent [+] |
| `aws_networkmanager_link` | 11 | 10 | 10 | 80% | 70% | excellent [+] |
| `aws_networkmanager_link_association` | 5 | 4 | 4 | 75% | 75% | excellent [+] |
| `aws_networkmanager_site` | 8 | 8 | 7 | 71% | 71% | excellent [+] |
| `aws_networkmanager_site_to_site_vpn_attachment` | 10 | 9 | 14 | 64% | 57% | excellent [+] |
| `aws_networkmanager_transit_gateway_connect_peer_association` | 6 | 5 | 5 | 80% | 80% | excellent [+] |
| `aws_networkmanager_transit_gateway_peering` | 10 | 0 | 12 | 67% | 0% | good [~] |
| `aws_networkmanager_transit_gateway_registration` | 4 | 3 | 3 | 67% | 67% | excellent [+] |
| `aws_networkmanager_transit_gateway_route_table_attachment` | 11 | 9 | 15 | 67% | 53% | excellent [+] |
| `aws_networkmanager_vpc_attachment` | 11 | 10 | 16 | 62% | 56% | excellent [+] |
| `aws_opensearch_authorize_vpc_endpoint_access` | 3 | 6 | 3 | 67% | 100% | excellent [+] |
| `aws_opensearch_domain` | 14 | 24 | 29 | 14% | 52% | good [~] |
| `aws_opensearch_domain_policy` | 3 | 3 | 3 | 67% | 67% | excellent [+] |
| `aws_opensearch_domain_saml_options` | 2 | 0 | 3 | 33% | 0% | fair [-] |
| `aws_opensearch_inbound_connection_accepter` | 2 | 3 | 3 | 33% | 67% | good [~] |
| `aws_opensearch_outbound_connection` | 7 | 10 | 8 | 62% | 62% | excellent [+] |
| `aws_opensearch_package` | 6 | 7 | 6 | 50% | 83% | good [~] |
| `aws_opensearch_package_association` | 3 | 3 | 4 | 50% | 50% | good [~] |
| `aws_opensearch_vpc_endpoint` | 11 | 8 | 4 | 50% | 75% | good [~] |
| `aws_opensearchserverless_access_policy` | 6 | 6 | 5 | 100% | 100% | excellent [+] |
| `aws_opensearchserverless_collection` | 5 | 7 | 11 | 36% | 55% | good [~] |
| `aws_opensearchserverless_lifecycle_policy` | 4 | 0 | 5 | 80% | 0% | good [~] |
| `aws_opensearchserverless_security_config` | 3 | 0 | 5 | 60% | 0% | good [~] |
| `aws_opensearchserverless_security_policy` | 5 | 5 | 5 | 80% | 100% | excellent [+] |
| `aws_opensearchserverless_vpc_endpoint` | 5 | 5 | 5 | 80% | 80% | excellent [+] |
| `aws_organizations_account` | 5 | 6 | 15 | 27% | 33% | good [~] |
| `aws_organizations_delegated_administrator` | 3 | 10 | 9 | 22% | 100% | good [~] |
| `aws_organizations_organization` | 5 | 5 | 11 | 36% | 36% | good [~] |
| `aws_organizations_organizational_unit` | 4 | 4 | 6 | 50% | 50% | good [~] |
| `aws_organizations_policy` | 6 | 6 | 8 | 62% | 62% | excellent [+] |
| `aws_organizations_policy_attachment` | 3 | 3 | 3 | 67% | 67% | excellent [+] |
| `aws_organizations_resource_policy` | 3 | 3 | 4 | 50% | 50% | good [~] |
| `aws_osis_pipeline` | 9 | 12 | 13 | 38% | 85% | good [~] |
| `aws_outposts_outpost` | 11 | 13 | 0 | 0% | 0% | n/a [?] |
| `aws_outposts_site` | 7 | 6 | 0 | 0% | 0% | n/a [?] |
| `aws_pinpoint_adm_channel` | 4 | 0 | 4 | 100% | 0% | good [~] |
| `aws_pinpoint_apns_channel` | 9 | 0 | 9 | 100% | 0% | good [~] |
| `aws_pinpoint_apns_sandbox_channel` | 9 | 0 | 9 | 100% | 0% | good [~] |
| `aws_pinpoint_apns_voip_channel` | 9 | 0 | 9 | 100% | 0% | good [~] |
| `aws_pinpoint_apns_voip_sandbox_channel` | 9 | 0 | 9 | 100% | 0% | good [~] |
| `aws_pinpoint_app` | 10 | 18 | 9 | 89% | 89% | excellent [+] |
| `aws_pinpoint_baidu_channel` | 4 | 0 | 4 | 100% | 0% | good [~] |
| `aws_pinpoint_email_channel` | 8 | 8 | 8 | 88% | 88% | excellent [+] |
| `aws_pinpoint_email_template` | 2 | 0 | 5 | 40% | 0% | good [~] |
| `aws_pinpoint_event_stream` | 4 | 4 | 3 | 100% | 100% | excellent [+] |
| `aws_pinpoint_gcm_channel` | 5 | 0 | 5 | 100% | 0% | good [~] |
| `aws_pinpoint_sms_channel` | 6 | 0 | 6 | 100% | 0% | good [~] |
| `aws_pipes_pipe` | 13 | 18 | 17 | 53% | 82% | good [~] |
| `aws_account_id` | 0 | 4 | 0 | 0% | 0% | n/a [?] |
| `aws_quicksight_account_settings` | 5 | 5 | 4 | 75% | 75% | excellent [+] |
| `aws_quicksight_account_subscription` | 0 | 0 | 18 | 0% | 0% | poor [!] |
| `aws_quicksight_analysis` | 9 | 8 | 17 | 47% | 41% | good [~] |
| `aws_quicksight_dashboard` | 10 | 8 | 20 | 40% | 30% | good [~] |
| `aws_quicksight_data_set` | 8 | 9 | 18 | 28% | 33% | good [~] |
| `aws_quicksight_data_source` | 12 | 14 | 12 | 67% | 83% | excellent [+] |
| `aws_quicksight_folder` | 9 | 8 | 13 | 62% | 54% | excellent [+] |
| `aws_quicksight_folder_membership` | 5 | 5 | 4 | 100% | 100% | excellent [+] |
| `aws_quicksight_group` | 6 | 6 | 5 | 80% | 80% | excellent [+] |
| `aws_quicksight_group_membership` | 6 | 6 | 5 | 100% | 100% | excellent [+] |
| `aws_quicksight_iam_policy_assignment` | 0 | 0 | 7 | 0% | 0% | poor [!] |
| `aws_quicksight_ingestion` | 7 | 6 | 6 | 100% | 83% | excellent [+] |
| `aws_quicksight_namespace` | 7 | 7 | 9 | 67% | 67% | excellent [+] |
| `aws_quicksight_refresh_schedule` | 0 | 0 | 5 | 0% | 0% | poor [!] |
| `aws_quicksight_role_membership` | 0 | 0 | 4 | 0% | 0% | poor [!] |
| `aws_quicksight_template` | 9 | 9 | 16 | 44% | 44% | good [~] |
| `aws_quicksight_template_alias` | 0 | 0 | 5 | 0% | 0% | poor [!] |
| `aws_quicksight_theme` | 10 | 9 | 15 | 53% | 47% | good [~] |
| `aws_quicksight_user` | 8 | 9 | 10 | 60% | 60% | excellent [+] |
| `aws_quicksight_vpc_connection` | 0 | 0 | 12 | 0% | 0% | poor [!] |
| `aws_ram_principal_association` | 3 | 3 | 2 | 100% | 100% | excellent [+] |
| `aws_ram_resource_association` | 3 | 3 | 2 | 100% | 100% | excellent [+] |
| `aws_ram_resource_share` | 5 | 9 | 7 | 57% | 57% | good [~] |
| `aws_ram_resource_share_accepter` | 6 | 6 | 9 | 56% | 56% | good [~] |
| `aws_ram_sharing_with_organization` | 1 | 1 | 0 | 0% | 0% | n/a [?] |
| `aws_db_cluster_snapshot` | 14 | 18 | 19 | 68% | 84% | excellent [+] |
| `aws_db_event_subscription` | 9 | 8 | 12 | 67% | 58% | excellent [+] |
| `aws_db_instance` | 45 | 48 | 84 | 48% | 55% | good [~] |
| `aws_db_instance_automated_backups_replication` | 4 | 0 | 5 | 80% | 0% | good [~] |
| `aws_db_instance_role_association` | 3 | 0 | 4 | 75% | 0% | good [~] |
| `aws_db_option_group` | 9 | 9 | 11 | 64% | 73% | excellent [+] |
| `aws_db_parameter_group` | 7 | 7 | 9 | 67% | 67% | excellent [+] |
| `aws_db_proxy` | 15 | 14 | 14 | 71% | 86% | excellent [+] |
| `aws_db_proxy_default_target_group` | 5 | 5 | 5 | 80% | 80% | excellent [+] |
| `aws_db_proxy_endpoint` | 9 | 10 | 12 | 67% | 75% | excellent [+] |
| `aws_db_proxy_target` | 11 | 9 | 10 | 100% | 80% | excellent [+] |
| `aws_db_snapshot` | 22 | 21 | 23 | 91% | 87% | excellent [+] |
| `aws_db_snapshot_copy` | 23 | 0 | 25 | 88% | 0% | good [~] |
| `aws_db_subnet_group` | 7 | 7 | 9 | 67% | 67% | excellent [+] |
| `aws_rds_certificate` | 1 | 0 | 1 | 100% | 0% | good [~] |
| `aws_rds_cluster` | 52 | 49 | 74 | 68% | 55% | excellent [+] |
| `aws_rds_cluster_activity_stream` | 5 | 0 | 5 | 100% | 0% | good [~] |
| `aws_rds_cluster_endpoint` | 7 | 8 | 9 | 67% | 78% | excellent [+] |
| `aws_rds_cluster_instance` | 26 | 24 | 36 | 69% | 64% | excellent [+] |
| `aws_rds_cluster_parameter_group` | 6 | 7 | 8 | 62% | 75% | excellent [+] |
| `aws_rds_cluster_role_association` | 3 | 0 | 4 | 75% | 0% | good [~] |
| `aws_rds_cluster_snapshot_copy` | 18 | 0 | 19 | 89% | 0% | good [~] |
| `aws_rds_custom_db_engine_version` | 9 | 0 | 20 | 45% | 0% | good [~] |
| `aws_rds_export_task` | 15 | 16 | 16 | 88% | 94% | excellent [+] |
| `aws_rds_global_cluster` | 13 | 9 | 17 | 71% | 47% | good [~] |
| `aws_rds_instance_state` | 3 | 0 | 3 | 67% | 0% | good [~] |
| `aws_rds_integration` | 4 | 0 | 10 | 40% | 0% | good [~] |
| `aws_rds_reserved_instance` | 3 | 0 | 19 | 16% | 0% | poor [!] |
| `aws_rds_shard_group` | 11 | 11 | 12 | 83% | 83% | excellent [+] |
| `aws_redshift_authentication_profile` | 3 | 3 | 2 | 100% | 100% | excellent [+] |
| `aws_redshift_cluster` | 36 | 43 | 52 | 63% | 54% | excellent [+] |
| `aws_redshift_cluster_iam_roles` | 4 | 0 | 4 | 75% | 0% | good [~] |
| `aws_redshift_cluster_snapshot` | 6 | 12 | 8 | 62% | 62% | excellent [+] |
| `aws_redshift_data_share_authorization` | 4 | 0 | 5 | 60% | 0% | good [~] |
| `aws_redshift_data_share_consumer_association` | 6 | 0 | 7 | 71% | 0% | good [~] |
| `aws_redshift_endpoint_access` | 6 | 0 | 8 | 62% | 0% | good [~] |
| `aws_redshift_endpoint_authorization` | 5 | 0 | 8 | 50% | 0% | good [~] |
| `aws_redshift_event_subscription` | 12 | 13 | 13 | 85% | 92% | excellent [+] |
| `aws_redshift_hsm_client_certificate` | 5 | 6 | 5 | 80% | 100% | excellent [+] |
| `aws_redshift_hsm_configuration` | 7 | 8 | 9 | 67% | 78% | excellent [+] |
| `aws_redshift_integration` | 9 | 0 | 10 | 80% | 0% | good [~] |
| `aws_redshift_logging` | 5 | 6 | 5 | 80% | 100% | excellent [+] |
| `aws_redshift_parameter_group` | 7 | 9 | 7 | 86% | 100% | excellent [+] |
| `aws_redshift_partner` | 7 | 7 | 6 | 100% | 100% | excellent [+] |
| `aws_redshift_resource_policy` | 3 | 3 | 2 | 100% | 100% | excellent [+] |
| `aws_redshift_scheduled_action` | 8 | 9 | 8 | 88% | 88% | excellent [+] |
| `aws_redshift_snapshot_copy` | 5 | 5 | 5 | 80% | 80% | excellent [+] |
| `aws_redshift_snapshot_copy_grant` | 5 | 6 | 5 | 80% | 100% | excellent [+] |
| `aws_redshift_snapshot_schedule` | 6 | 7 | 8 | 62% | 75% | excellent [+] |
| `aws_redshift_snapshot_schedule_association` | 3 | 0 | 2 | 100% | 0% | good [~] |
| `aws_redshift_subnet_group` | 7 | 6 | 6 | 83% | 67% | excellent [+] |
| `aws_redshift_usage_limit` | 10 | 10 | 9 | 89% | 100% | excellent [+] |
| `aws_rekognition_collection` | 5 | 8 | 6 | 17% | 67% | good [~] |
| `aws_resiliencehub_resiliency_policy` | 6 | 12 | 10 | 20% | 50% | good [~] |
| `aws_resourcegroups_group` | 7 | 10 | 8 | 75% | 62% | excellent [+] |
| `aws_rolesanywhere_profile` | 10 | 9 | 10 | 50% | 80% | good [~] |
| `aws_rolesanywhere_trust_anchor` | 6 | 5 | 7 | 71% | 57% | excellent [+] |
| `aws_route53_cidr_collection` | 2 | 4 | 3 | 33% | 100% | good [~] |
| `aws_route53_cidr_location` | 4 | 4 | 3 | 100% | 100% | excellent [+] |
| `aws_route53_delegation_set` | 2 | 3 | 3 | 33% | 67% | good [~] |
| `aws_route53_health_check` | 24 | 20 | 24 | 79% | 67% | excellent [+] |
| `aws_route53_hosted_zone_dnssec` | 3 | 3 | 3 | 67% | 67% | excellent [+] |
| `aws_route53_key_signing_key` | 5 | 5 | 15 | 27% | 27% | fair [-] |
| `aws_route53_query_log` | 4 | 3 | 3 | 67% | 67% | excellent [+] |
| `aws_route53_record` | 16 | 16 | 18 | 83% | 83% | excellent [+] |
| `aws_route53_traffic_policy` | 7 | 6 | 6 | 83% | 83% | excellent [+] |
| `aws_route53_traffic_policy_instance` | 7 | 7 | 6 | 83% | 83% | excellent [+] |
| `aws_route53_vpc_association_authorization` | 4 | 4 | 4 | 75% | 75% | excellent [+] |
| `aws_route53_zone` | 9 | 10 | 12 | 67% | 58% | excellent [+] |
| `aws_route53_zone_association` | 4 | 4 | 5 | 60% | 60% | excellent [+] |
| `aws_route53domains_registered_domain` | 10 | 20 | 25 | 36% | 64% | good [~] |
| `aws_route53_resolver_config` | 4 | 0 | 3 | 100% | 0% | good [~] |
| `aws_route53_resolver_dnssec_config` | 4 | 4 | 4 | 75% | 75% | excellent [+] |
| `aws_route53_resolver_endpoint` | 11 | 16 | 11 | 91% | 82% | excellent [+] |
| `aws_route53_resolver_firewall_config` | 4 | 0 | 3 | 100% | 0% | good [~] |
| `aws_route53_resolver_firewall_domain_list` | 3 | 0 | 5 | 40% | 0% | good [~] |
| `aws_route53_resolver_firewall_rule` | 12 | 0 | 11 | 91% | 0% | good [~] |
| `aws_route53_resolver_firewall_rule_group` | 5 | 0 | 6 | 67% | 0% | good [~] |
| `aws_route53_resolver_firewall_rule_group_association` | 7 | 0 | 8 | 75% | 0% | good [~] |
| `aws_route53_resolver_query_log_config` | 6 | 9 | 7 | 71% | 86% | excellent [+] |
| `aws_route53_resolver_query_log_config_association` | 3 | 6 | 2 | 100% | 100% | excellent [+] |
| `aws_route53_resolver_rule` | 9 | 15 | 11 | 73% | 82% | excellent [+] |
| `aws_route53_resolver_rule_association` | 4 | 6 | 4 | 75% | 75% | excellent [+] |
| `aws_s3_access_point` | 0 | 0 | 13 | 0% | 0% | poor [!] |
| `aws_s3_account_public_access_block` | 0 | 0 | 5 | 0% | 0% | poor [!] |
| `aws_s3_bucket` | 21 | 21 | 27 | 63% | 70% | excellent [+] |
| `aws_s3_bucket_accelerate_configuration` | 2 | 2 | 3 | 67% | 67% | excellent [+] |
| `aws_s3_bucket_acl` | 3 | 2 | 4 | 75% | 50% | excellent [+] |
| `aws_s3_bucket_analytics_configuration` | 2 | 2 | 4 | 50% | 50% | good [~] |
| `aws_s3_bucket_cors_configuration` | 2 | 2 | 3 | 67% | 67% | excellent [+] |
| `aws_s3_bucket_intelligent_tiering_configuration` | 3 | 2 | 5 | 60% | 40% | good [~] |
| `aws_s3_bucket_inventory` | 0 | 0 | 8 | 0% | 0% | poor [!] |
| `aws_s3_bucket_lifecycle_configuration` | 2 | 2 | 5 | 40% | 40% | good [~] |
| `aws_s3_bucket_logging` | 3 | 4 | 6 | 50% | 50% | good [~] |
| `aws_s3_bucket_metric` | 2 | 2 | 3 | 67% | 67% | excellent [+] |
| `aws_s3_bucket_notification` | 4 | 5 | 5 | 80% | 80% | excellent [+] |
| `aws_s3_bucket_object` | 0 | 0 | 27 | 0% | 0% | poor [!] |
| `aws_s3_bucket_object_lock_configuration` | 3 | 4 | 5 | 60% | 60% | excellent [+] |
| `aws_s3_bucket_ownership_controls` | 2 | 2 | 2 | 100% | 100% | excellent [+] |
| `aws_s3_bucket_policy` | 2 | 2 | 2 | 100% | 100% | excellent [+] |
| `aws_s3_bucket_public_access_block` | 0 | 0 | 5 | 0% | 0% | poor [!] |
| `aws_s3_bucket_replication_configuration` | 3 | 4 | 4 | 75% | 75% | excellent [+] |
| `aws_s3_bucket_request_payment_configuration` | 0 | 0 | 3 | 0% | 0% | poor [!] |
| `aws_s3_bucket_server_side_encryption_configuration` | 2 | 2 | 3 | 67% | 67% | excellent [+] |
| `aws_s3_bucket_versioning` | 3 | 3 | 4 | 50% | 50% | good [~] |
| `aws_s3_bucket_website_configuration` | 5 | 6 | 9 | 56% | 56% | good [~] |
| `aws_s3_directory_bucket` | 0 | 0 | 6 | 0% | 0% | poor [!] |
| `aws_s3_object` | 0 | 0 | 34 | 0% | 0% | poor [!] |
| `aws_s3_object_copy` | 0 | 0 | 53 | 0% | 0% | poor [!] |
| `aws_s3control_access_grant` | 10 | 8 | 11 | 64% | 45% | good [~] |
| `aws_s3control_access_grants_instance` | 7 | 6 | 7 | 71% | 57% | excellent [+] |
| `aws_s3control_access_grants_instance_resource_policy` | 5 | 4 | 2 | 100% | 50% | excellent [+] |
| `aws_s3control_access_grants_location` | 7 | 6 | 7 | 71% | 57% | excellent [+] |
| `aws_s3control_access_point` | 14 | 15 | 0 | 0% | 0% | n/a [?] |
| `aws_s3control_access_point_policy` | 4 | 0 | 3 | 100% | 0% | good [~] |
| `aws_s3control_bucket` | 7 | 8 | 7 | 71% | 86% | excellent [+] |
| `aws_s3control_bucket_lifecycle_configuration` | 2 | 3 | 2 | 50% | 100% | good [~] |
| `aws_s3control_bucket_policy` | 3 | 0 | 2 | 100% | 0% | good [~] |
| `aws_s3control_directory_bucket_access_point_scope` | 3 | 5 | 3 | 67% | 67% | excellent [+] |
| `aws_s3control_multi_region_access_point` | 6 | 4 | 7 | 71% | 43% | good [~] |
| `aws_s3control_multi_region_access_point_policy` | 4 | 0 | 5 | 60% | 0% | good [~] |
| `aws_s3control_object_lambda_access_point` | 5 | 5 | 5 | 80% | 80% | excellent [+] |
| `aws_s3control_object_lambda_access_point_policy` | 5 | 0 | 4 | 100% | 0% | good [~] |
| `aws_s3control_storage_lens_configuration` | 5 | 4 | 6 | 67% | 50% | excellent [+] |
| `aws_s3tables_namespace` | 8 | 8 | 5 | 100% | 100% | excellent [+] |
| `aws_s3tables_table` | 15 | 14 | 16 | 81% | 81% | excellent [+] |
| `aws_s3tables_table_bucket` | 13 | 13 | 6 | 67% | 67% | excellent [+] |
| `aws_s3tables_table_bucket_policy` | 0 | 0 | 2 | 0% | 0% | poor [!] |
| `aws_s3tables_table_policy` | 0 | 0 | 4 | 0% | 0% | poor [!] |
| `aws_sagemaker_app` | 11 | 11 | 9 | 89% | 89% | excellent [+] |
| `aws_sagemaker_app_image_config` | 2 | 0 | 7 | 29% | 0% | fair [-] |
| `aws_sagemaker_code_repository` | 2 | 0 | 5 | 40% | 0% | good [~] |
| `aws_sagemaker_data_quality_job_definition` | 7 | 7 | 12 | 42% | 42% | good [~] |
| `aws_sagemaker_device` | 11 | 0 | 4 | 25% | 0% | fair [-] |
| `aws_sagemaker_device_fleet` | 4 | 0 | 9 | 44% | 0% | good [~] |
| `aws_sagemaker_domain` | 20 | 20 | 20 | 75% | 75% | excellent [+] |
| `aws_sagemaker_endpoint` | 9 | 8 | 6 | 83% | 67% | excellent [+] |
| `aws_sagemaker_endpoint_configuration` | 10 | 10 | 10 | 80% | 80% | excellent [+] |
| `aws_sagemaker_feature_group` | 8 | 8 | 12 | 33% | 33% | good [~] |
| `aws_sagemaker_flow_definition` | 3 | 0 | 9 | 33% | 0% | fair [-] |
| `aws_sagemaker_hub` | 3 | 0 | 8 | 38% | 0% | fair [-] |
| `aws_sagemaker_human_task_ui` | 2 | 0 | 5 | 40% | 0% | good [~] |
| `aws_sagemaker_image` | 5 | 0 | 7 | 71% | 0% | good [~] |
| `aws_sagemaker_image_version` | 4 | 0 | 13 | 31% | 0% | fair [-] |
| `aws_sagemaker_mlflow_tracking_server` | 4 | 0 | 11 | 36% | 0% | fair [-] |
| `aws_sagemaker_model` | 11 | 11 | 10 | 90% | 90% | excellent [+] |
| `aws_sagemaker_model_package_group` | 7 | 7 | 5 | 100% | 100% | excellent [+] |
| `aws_sagemaker_model_package_group_policy` | 2 | 0 | 2 | 100% | 0% | good [~] |
| `aws_sagemaker_monitoring_schedule` | 2 | 0 | 5 | 40% | 0% | good [~] |
| `aws_sagemaker_notebook_instance` | 18 | 15 | 20 | 65% | 55% | excellent [+] |
| `aws_sagemaker_notebook_instance_lifecycle_configuration` | 5 | 6 | 6 | 33% | 67% | good [~] |
| `aws_sagemaker_pipeline` | 6 | 11 | 10 | 20% | 80% | good [~] |
| `aws_sagemaker_project` | 3 | 0 | 7 | 43% | 0% | good [~] |
| `aws_sagemaker_servicecatalog_portfolio_status` | 1 | 0 | 1 | 100% | 0% | good [~] |
| `aws_sagemaker_space` | 9 | 9 | 11 | 45% | 45% | good [~] |
| `aws_sagemaker_studio_lifecycle_config` | 4 | 0 | 6 | 67% | 0% | good [~] |
| `aws_sagemaker_user_profile` | 9 | 9 | 9 | 56% | 56% | good [~] |
| `aws_sagemaker_workforce` | 3 | 0 | 7 | 43% | 0% | good [~] |
| `aws_sagemaker_workteam` | 5 | 0 | 10 | 50% | 0% | good [~] |
| `aws_scheduler_schedule` | 10 | 20 | 13 | 62% | 69% | excellent [+] |
| `aws_scheduler_schedule_group` | 4 | 6 | 9 | 33% | 56% | good [~] |
| `aws_secretsmanager_secret` | 6 | 7 | 11 | 45% | 55% | good [~] |
| `aws_secretsmanager_secret_version` | 5 | 6 | 9 | 44% | 56% | good [~] |
| `aws_securityhub_account` | 5 | 6 | 4 | 25% | 100% | good [~] |
| `aws_securityhub_action_target` | 4 | 5 | 4 | 75% | 100% | excellent [+] |
| `aws_securityhub_automation_rule` | 6 | 7 | 10 | 50% | 60% | good [~] |
| `aws_securityhub_configuration_policy` | 3 | 5 | 4 | 50% | 100% | good [~] |
| `aws_securityhub_configuration_policy_association` | 3 | 3 | 3 | 67% | 67% | excellent [+] |
| `aws_securityhub_finding_aggregator` | 2 | 4 | 2 | 50% | 100% | good [~] |
| `aws_securityhub_insight` | 3 | 5 | 4 | 50% | 100% | good [~] |
| `aws_securityhub_invite_accepter` | 2 | 0 | 2 | 50% | 0% | good [~] |
| `aws_securityhub_member` | 14 | 6 | 5 | 60% | 100% | excellent [+] |
| `aws_securityhub_organization_admin_account` | 2 | 2 | 1 | 100% | 100% | excellent [+] |
| `aws_securityhub_organization_configuration` | 2 | 3 | 4 | 25% | 50% | good [~] |
| `aws_securityhub_product_subscription` | 2 | 3 | 2 | 50% | 100% | good [~] |
| `aws_securityhub_standards_control` | 4 | 11 | 10 | 30% | 100% | good [~] |
| `aws_securityhub_standards_control_association` | 5 | 5 | 4 | 100% | 100% | excellent [+] |
| `aws_securityhub_standards_subscription` | 2 | 5 | 2 | 50% | 50% | good [~] |
| `aws_servicecatalog_budget_resource_association` | 0 | 0 | 3 | 0% | 0% | poor [!] |
| `aws_servicecatalog_constraint` | 0 | 0 | 9 | 0% | 0% | poor [!] |
| `aws_servicecatalog_organizations_access` | 0 | 0 | 2 | 0% | 0% | poor [!] |
| `aws_servicecatalog_portfolio` | 7 | 7 | 8 | 75% | 75% | excellent [+] |
| `aws_servicecatalog_portfolio_share` | 0 | 0 | 9 | 0% | 0% | poor [!] |
| `aws_servicecatalog_principal_portfolio_association` | 0 | 0 | 5 | 0% | 0% | poor [!] |
| `aws_servicecatalog_product` | 12 | 12 | 17 | 65% | 65% | excellent [+] |
| `aws_servicecatalog_product_portfolio_association` | 0 | 0 | 5 | 0% | 0% | poor [!] |
| `aws_servicecatalog_provisioned_product` | 0 | 0 | 27 | 0% | 0% | poor [!] |
| `aws_servicecatalog_provisioning_artifact` | 0 | 0 | 13 | 0% | 0% | poor [!] |
| `aws_servicecatalog_service_action` | 0 | 0 | 5 | 0% | 0% | poor [!] |
| `aws_servicecatalog_tag_option` | 0 | 0 | 5 | 0% | 0% | poor [!] |
| `aws_servicecatalog_tag_option_resource_association` | 0 | 0 | 7 | 0% | 0% | poor [!] |
| `aws_servicecatalogappregistry_application` | 7 | 7 | 6 | 67% | 67% | excellent [+] |
| `aws_service_discovery_http_namespace` | 5 | 6 | 6 | 67% | 83% | excellent [+] |
| `aws_service_discovery_instance` | 4 | 4 | 3 | 100% | 100% | excellent [+] |
| `aws_service_discovery_private_dns_namespace` | 8 | 8 | 7 | 71% | 71% | excellent [+] |
| `aws_service_discovery_public_dns_namespace` | 6 | 7 | 6 | 67% | 83% | excellent [+] |
| `aws_service_discovery_service` | 10 | 16 | 11 | 36% | 82% | good [~] |
| `aws_servicequotas_service_quota` | 10 | 11 | 11 | 55% | 64% | good [~] |
| `aws_sesv2_account_suppression_attributes` | 2 | 2 | 1 | 100% | 100% | excellent [+] |
| `aws_sesv2_account_vdm_attributes` | 4 | 4 | 3 | 100% | 100% | excellent [+] |
| `aws_sesv2_configuration_set` | 11 | 9 | 10 | 70% | 80% | excellent [+] |
| `aws_sesv2_configuration_set_event_destination` | 4 | 6 | 3 | 100% | 100% | excellent [+] |
| `aws_sesv2_contact_list` | 5 | 6 | 8 | 50% | 62% | good [~] |
| `aws_sesv2_dedicated_ip_assignment` | 3 | 3 | 3 | 67% | 67% | excellent [+] |
| `aws_sesv2_dedicated_ip_pool` | 4 | 4 | 5 | 60% | 60% | excellent [+] |
| `aws_sesv2_email_identity` | 6 | 10 | 8 | 50% | 62% | good [~] |
| `aws_sesv2_email_identity_feedback_attributes` | 3 | 3 | 2 | 100% | 100% | excellent [+] |
| `aws_sesv2_email_identity_mail_from_attributes` | 4 | 4 | 3 | 100% | 100% | excellent [+] |
| `aws_sesv2_email_identity_policy` | 4 | 4 | 3 | 100% | 100% | excellent [+] |
| `aws_ses_active_receipt_rule_set` | 3 | 3 | 2 | 100% | 100% | excellent [+] |
| `aws_ses_configuration_set` | 5 | 5 | 7 | 57% | 57% | good [~] |
| `aws_ses_domain_dkim` | 2 | 3 | 2 | 50% | 100% | good [~] |
| `aws_ses_domain_identity` | 4 | 4 | 3 | 100% | 100% | excellent [+] |
| `aws_ses_domain_identity_verification` | 3 | 3 | 3 | 67% | 67% | excellent [+] |
| `aws_ses_domain_mail_from` | 4 | 4 | 3 | 100% | 100% | excellent [+] |
| `aws_ses_email_identity` | 3 | 2 | 2 | 100% | 50% | excellent [+] |
| `aws_ses_event_destination` | 5 | 5 | 8 | 50% | 50% | good [~] |
| `aws_ses_identity_notification_topic` | 5 | 5 | 4 | 100% | 100% | excellent [+] |
| `aws_ses_identity_policy` | 0 | 0 | 3 | 0% | 0% | poor [!] |
| `aws_ses_receipt_filter` | 0 | 0 | 4 | 0% | 0% | poor [!] |
| `aws_ses_receipt_rule` | 7 | 6 | 15 | 40% | 33% | good [~] |
| `aws_ses_receipt_rule_set` | 3 | 3 | 2 | 100% | 100% | excellent [+] |
| `aws_ses_template` | 6 | 6 | 5 | 100% | 100% | excellent [+] |
| `aws_shield_application_layer_automatic_response` | 2 | 0 | 3 | 33% | 0% | fair [-] |
| `aws_shield_drt_access_log_bucket_association` | 3 | 0 | 3 | 67% | 0% | good [~] |
| `aws_shield_drt_access_role_arn_association` | 2 | 0 | 2 | 50% | 0% | good [~] |
| `aws_shield_proactive_engagement` | 2 | 0 | 2 | 50% | 0% | good [~] |
| `aws_shield_protection` | 5 | 7 | 5 | 80% | 80% | excellent [+] |
| `aws_shield_protection_group` | 6 | 0 | 8 | 62% | 0% | good [~] |
| `aws_shield_protection_health_check_association` | 3 | 3 | 2 | 100% | 100% | excellent [+] |
| `aws_shield_subscription` | 8 | 6 | 2 | 0% | 50% | good [~] |
| `aws_signer_signing_profile` | 8 | 12 | 13 | 46% | 69% | good [~] |
| `aws_simpledb_domain` | 12 | 2 | 1 | 0% | 100% | good [~] |
| `aws_sns_platform_application` | 6 | 6 | 14 | 36% | 36% | good [~] |
| `aws_sns_sms_preferences` | 7 | 7 | 6 | 100% | 100% | excellent [+] |
| `aws_sns_topic` | 25 | 0 | 32 | 75% | 0% | good [~] |
| `aws_sns_topic_data_protection_policy` | 3 | 0 | 2 | 100% | 0% | good [~] |
| `aws_sns_topic_policy` | 3 | 0 | 3 | 67% | 0% | good [~] |
| `aws_sns_topic_subscription` | 8 | 12 | 16 | 44% | 50% | good [~] |
| `aws_sqs_queue` | 14 | 13 | 22 | 59% | 55% | good [~] |
| `aws_sqs_queue_policy` | 3 | 0 | 2 | 100% | 0% | good [~] |
| `aws_sqs_queue_redrive_allow_policy` | 0 | 0 | 2 | 0% | 0% | poor [!] |
| `aws_sqs_queue_redrive_policy` | 3 | 0 | 2 | 100% | 0% | good [~] |
| `aws_ssm_activation` | 7 | 12 | 10 | 50% | 100% | good [~] |
| `aws_ssm_association` | 6 | 12 | 19 | 26% | 47% | good [~] |
| `aws_ssm_default_patch_baseline` | 3 | 3 | 2 | 100% | 100% | excellent [+] |
| `aws_ssm_document` | 10 | 13 | 23 | 39% | 52% | good [~] |
| `aws_ssm_maintenance_window` | 11 | 8 | 13 | 77% | 54% | excellent [+] |
| `aws_ssm_maintenance_window_target` | 5 | 3 | 6 | 67% | 33% | good [~] |
| `aws_ssm_maintenance_window_task` | 11 | 5 | 14 | 71% | 29% | good [~] |
| `aws_ssm_parameter` | 7 | 11 | 17 | 35% | 53% | good [~] |
| `aws_ssm_patch_baseline` | 7 | 7 | 15 | 40% | 40% | good [~] |
| `aws_ssm_patch_group` | 3 | 3 | 2 | 100% | 100% | excellent [+] |
| `aws_ssm_resource_data_sync` | 3 | 6 | 2 | 100% | 100% | excellent [+] |
| `aws_ssm_service_setting` | 3 | 6 | 4 | 50% | 100% | good [~] |
| `aws_ssoadmin_account_assignment` | 6 | 6 | 7 | 71% | 71% | excellent [+] |
| `aws_ssoadmin_application` | 6 | 0 | 11 | 27% | 0% | fair [-] |
| `aws_ssoadmin_application_access_scope` | 2 | 0 | 3 | 67% | 0% | good [~] |
| `aws_ssoadmin_application_assignment` | 3 | 0 | 3 | 100% | 0% | good [~] |
| `aws_ssoadmin_application_assignment_configuration` | 2 | 0 | 2 | 100% | 0% | good [~] |
| `aws_ssoadmin_customer_managed_policy_attachment` | 3 | 5 | 4 | 50% | 50% | good [~] |
| `aws_ssoadmin_instance_access_control_attributes` | 3 | 0 | 4 | 75% | 0% | good [~] |
| `aws_ssoadmin_managed_policy_attachment` | 5 | 4 | 5 | 80% | 60% | excellent [+] |
| `aws_ssoadmin_permission_set` | 7 | 7 | 10 | 60% | 60% | excellent [+] |
| `aws_ssoadmin_permission_set_inline_policy` | 4 | 3 | 4 | 75% | 50% | excellent [+] |
| `aws_ssoadmin_permissions_boundary_attachment` | 2 | 0 | 4 | 50% | 0% | good [~] |
| `aws_ssoadmin_trusted_token_issuer` | 6 | 0 | 8 | 75% | 0% | good [~] |
| `aws_sfn_state_machine` | 10 | 17 | 19 | 47% | 53% | good [~] |
| `aws_swf_domain` | 12 | 5 | 7 | 14% | 57% | good [~] |
| `aws_synthetics_canary` | 22 | 26 | 24 | 58% | 67% | good [~] |
| `aws_timestreaminfluxdb_db_instance` | 17 | 17 | 24 | 25% | 67% | good [~] |
| `aws_timestreamquery_scheduled_query` | 18 | 22 | 18 | 56% | 78% | good [~] |
| `aws_timestreamwrite_database` | 6 | 6 | 6 | 83% | 83% | excellent [+] |
| `aws_timestreamwrite_table` | 7 | 10 | 8 | 75% | 75% | excellent [+] |
| `aws_transcribe_language_model` | 8 | 11 | 8 | 62% | 88% | excellent [+] |
| `aws_transcribe_vocabulary` | 9 | 9 | 9 | 67% | 56% | excellent [+] |
| `aws_transfer_access` | 8 | 0 | 8 | 88% | 0% | good [~] |
| `aws_transfer_agreement` | 10 | 12 | 11 | 82% | 100% | excellent [+] |
| `aws_transfer_certificate` | 12 | 14 | 11 | 82% | 100% | excellent [+] |
| `aws_transfer_connector` | 9 | 10 | 10 | 80% | 90% | excellent [+] |
| `aws_transfer_profile` | 6 | 8 | 7 | 71% | 100% | excellent [+] |
| `aws_transfer_server` | 19 | 18 | 26 | 69% | 65% | excellent [+] |
| `aws_transfer_ssh_key` | 5 | 0 | 4 | 75% | 0% | good [~] |
| `aws_transfer_tag` | 4 | 0 | 3 | 100% | 0% | good [~] |
| `aws_transfer_user` | 10 | 11 | 12 | 75% | 83% | excellent [+] |
| `aws_transfer_workflow` | 6 | 7 | 6 | 50% | 100% | good [~] |
| `aws_vpclattice_access_log_subscription` | 7 | 6 | 7 | 86% | 71% | excellent [+] |
| `aws_vpclattice_auth_policy` | 4 | 3 | 4 | 75% | 75% | excellent [+] |
| `aws_vpclattice_listener` | 9 | 17 | 13 | 62% | 77% | excellent [+] |
| `aws_vpclattice_listener_rule` | 8 | 7 | 11 | 64% | 55% | excellent [+] |
| `aws_vpclattice_resource_configuration` | 8 | 8 | 12 | 58% | 58% | good [~] |
| `aws_vpclattice_resource_gateway` | 9 | 9 | 10 | 80% | 80% | excellent [+] |
| `aws_vpclattice_resource_policy` | 2 | 2 | 2 | 50% | 100% | good [~] |
| `aws_vpclattice_service` | 6 | 8 | 10 | 50% | 50% | good [~] |
| `aws_vpclattice_service_network` | 5 | 9 | 5 | 80% | 80% | excellent [+] |
| `aws_vpclattice_service_network_resource_association` | 5 | 6 | 7 | 57% | 57% | good [~] |
| `aws_vpclattice_service_network_service_association` | 6 | 6 | 10 | 50% | 50% | good [~] |
| `aws_vpclattice_service_network_vpc_association` | 6 | 7 | 9 | 56% | 67% | good [~] |
| `aws_vpclattice_target_group` | 7 | 15 | 8 | 75% | 75% | excellent [+] |
| `aws_vpclattice_target_group_attachment` | 5 | 4 | 3 | 67% | 67% | excellent [+] |
| `aws_wafv2_api_key` | 4 | 4 | 3 | 100% | 100% | excellent [+] |
| `aws_wafv2_ip_set` | 8 | 7 | 10 | 70% | 60% | excellent [+] |
| `aws_wafv2_regex_pattern_set` | 7 | 10 | 9 | 67% | 89% | excellent [+] |
| `aws_wafv2_rule_group` | 10 | 12 | 12 | 75% | 92% | excellent [+] |
| `aws_wafv2_web_acl` | 16 | 18 | 20 | 75% | 85% | excellent [+] |
| `aws_wafv2_web_acl_association` | 3 | 3 | 3 | 67% | 67% | excellent [+] |
| `aws_wafv2_web_acl_logging_configuration` | 7 | 7 | 4 | 100% | 100% | excellent [+] |
| `aws_workspaces_directory` | 16 | 19 | 23 | 61% | 70% | excellent [+] |
| `aws_workspaces_workspace` | 12 | 17 | 13 | 77% | 77% | excellent [+] |
| `aws_xray_group` | 5 | 6 | 6 | 67% | 50% | excellent [+] |
| `aws_xray_sampling_rule` | 15 | 15 | 15 | 80% | 80% | excellent [+] |

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

This page is rewritten by the `update-readme` skill, which invokes:

```bash
python3 .agents/skills/api-coverage/scripts/generate_terraform_resource_coverage.py
python3 .agents/skills/api-coverage/scripts/generate_terraform_converter_coverage.py
```
