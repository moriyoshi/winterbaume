# Terraform Converter Coverage

Winterbaume includes a Terraform state converter layer (`winterbaume-terraform`) that can inject Terraform state into the emulator and extract it back out. This enables:

- **Seeding mock environments** from existing Terraform state files
- **Round-trip validation** — inject state, exercise the mock, then extract and compare
- **Test data generation** — programmatically build Terraform state from converter output

## Overview

- **1026 converters** covering **1025 distinct Terraform resource types**
- **14.3% overall inject coverage** (reading TF state attributes into winterbaume)
- **52.2% overall extract coverage** (emitting winterbaume state back to TF attributes)
- Rating distribution: 16 excellent, 728 good, 30 fair, 246 poor, 6 n/a

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

How many of the AWS Terraform provider's `aws_*` resource types each winterbaume service handles. Of 1191 schema resources classified to a winterbaume service, 1127 are handled (77 still missing — see the per-service missing list in `.agents/docs/TERRAFORM_RESOURCE_COVERAGE.md`).

| Service | Prefix | Handled | Schema | Missing | Coverage | Note |
|---------|--------|---------|--------|---------|----------|------|
| ebs | `aws_ebs_` | 2 | 8 | 6 | 25% |  |
| elbv2 | override (2 patterns) | 10 | 16 | 6 | 62% |  |
| kinesis | `aws_kinesis_` | 3 | 6 | 3 | 50% |  |
| account | `aws_account_` | 1 | 3 | 2 | 33% |  |
| apigateway | `aws_api_gateway_` | 24 | 26 | 2 | 92% |  |
| cloudtrail | `aws_cloudtrail` | 1 | 3 | 2 | 33% |  |
| codeartifact | `aws_codeartifact_` | 2 | 4 | 2 | 50% |  |
| codepipeline | `aws_codepipeline` | 1 | 3 | 2 | 33% |  |
| costexplorer | `aws_ce_` | 2 | 4 | 2 | 50% |  |
| elasticbeanstalk | `aws_elastic_beanstalk_` | 2 | 4 | 2 | 50% |  |
| iam | `aws_iam_` | 32 | 34 | 2 | 94% |  |
| ivs | `aws_ivs_` | 1 | 3 | 2 | 33% |  |
| rds | override (2 patterns) | 27 | 29 | 2 | 93% |  |
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
| autoscaling | override (3 patterns) | 9 | 10 | 1 | 90% |  |
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
| mediastore | override (2 patterns) | 1 | 2 | 1 | 50% |  |
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
| cloudwatch | override (3 patterns) | 4 | 4 | 0 | 100% |  |
| codebuild | `aws_codebuild_` | 6 | 6 | 0 | 100% |  |
| codecommit | `aws_codecommit_` | 4 | 4 | 0 | 100% |  |
| cognitoidp | override (5 patterns) | 9 | 9 | 0 | 100% |  |
| config | `aws_config_` | 13 | 13 | 0 | 100% |  |
| connect | `aws_connect_` | 16 | 16 | 0 | 100% |  |
| datasync | `aws_datasync_` | 13 | 13 | 0 | 100% |  |
| dax | `aws_dax_` | 3 | 3 | 0 | 100% |  |
| directconnect | `aws_dx_` | 19 | 19 | 0 | 100% |  |
| directory | `aws_directory_service_` | 8 | 8 | 0 | 100% |  |
| dms | `aws_dms_` | 8 | 8 | 0 | 100% |  |
| dynamodb | `aws_dynamodb_` | 9 | 9 | 0 | 100% |  |
| ec2 | override (37 patterns) | 145 | 139 | 0 | 104% |  |
| ec2instanceconnect | `aws_ec2_instance_connect_` | 1 | 1 | 0 | 100% |  |
| ecr | `aws_ecr_` | 9 | 9 | 0 | 100% |  |
| ecs | `aws_ecs_` | 8 | 8 | 0 | 100% |  |
| efs | `aws_efs_` | 6 | 6 | 0 | 100% |  |
| eks | `aws_eks_` | 8 | 8 | 0 | 100% |  |
| elasticache | `aws_elasticache_` | 10 | 10 | 0 | 100% |  |
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
| kinesisanalyticsv2 | aws_ | 2 | 0 | 0 | — | heterogeneous prefix; manual review needed |
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
| route53 | override (11 patterns) | 13 | 13 | 0 | 100% |  |
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
| `aws_accessanalyzer_analyzer` | 1 | 5 | 6 | 0% | 67% | good [~] |
| `aws_account_alternate_contact` | 1 | 6 | 7 | 0% | 71% | good [~] |
| `aws_acm_certificate` | 6 | 20 | 23 | 22% | 61% | good [~] |
| `aws_acmpca_certificate` | 1 | 0 | 9 | 0% | 0% | poor [!] |
| `aws_acmpca_certificate_authority` | 3 | 25 | 17 | 12% | 65% | good [~] |
| `aws_acmpca_certificate_authority_certificate` | 1 | 0 | 3 | 0% | 0% | poor [!] |
| `aws_acmpca_permission` | 1 | 0 | 5 | 0% | 0% | poor [!] |
| `aws_acmpca_policy` | 1 | 0 | 2 | 0% | 0% | poor [!] |
| `aws_prometheus_alert_manager_definition` | 1 | 0 | 2 | 0% | 0% | poor [!] |
| `aws_prometheus_rule_group_namespace` | 1 | 6 | 6 | 0% | 83% | good [~] |
| `aws_prometheus_scraper` | 1 | 0 | 10 | 0% | 0% | poor [!] |
| `aws_prometheus_workspace` | 1 | 6 | 7 | 0% | 57% | good [~] |
| `aws_prometheus_workspace_configuration` | 1 | 0 | 4 | 0% | 0% | poor [!] |
| `aws_amplify_app` | 5 | 20 | 25 | 12% | 64% | good [~] |
| `aws_amplify_branch` | 1 | 15 | 23 | 0% | 52% | good [~] |
| `aws_amplify_domain_association` | 1 | 7 | 8 | 0% | 50% | good [~] |
| `aws_api_gateway_account` | 3 | 7 | 5 | 40% | 80% | good [~] |
| `aws_api_gateway_api_key` | 5 | 8 | 10 | 30% | 60% | good [~] |
| `aws_api_gateway_authorizer` | 3 | 10 | 10 | 20% | 90% | good [~] |
| `aws_api_gateway_base_path_mapping` | 1 | 5 | 5 | 0% | 80% | good [~] |
| `aws_api_gateway_client_certificate` | 3 | 6 | 7 | 29% | 71% | good [~] |
| `aws_api_gateway_deployment` | 4 | 7 | 10 | 30% | 60% | good [~] |
| `aws_api_gateway_documentation_part` | 2 | 9 | 4 | 25% | 75% | good [~] |
| `aws_api_gateway_documentation_version` | 1 | 5 | 3 | 0% | 100% | good [~] |
| `aws_api_gateway_domain_name` | 3 | 5 | 22 | 9% | 18% | fair [-] |
| `aws_api_gateway_domain_name_access_association` | 4 | 6 | 6 | 50% | 83% | good [~] |
| `aws_api_gateway_gateway_response` | 3 | 6 | 5 | 40% | 100% | good [~] |
| `aws_api_gateway_integration` | 5 | 17 | 17 | 24% | 94% | good [~] |
| `aws_api_gateway_integration_response` | 3 | 9 | 8 | 25% | 100% | good [~] |
| `aws_api_gateway_method` | 4 | 11 | 11 | 27% | 91% | good [~] |
| `aws_api_gateway_method_response` | 3 | 7 | 6 | 33% | 100% | good [~] |
| `aws_api_gateway_model` | 1 | 6 | 5 | 0% | 100% | good [~] |
| `aws_api_gateway_request_validator` | 3 | 5 | 4 | 50% | 100% | good [~] |
| `aws_api_gateway_resource` | 1 | 5 | 4 | 0% | 100% | good [~] |
| `aws_api_gateway_rest_api` | 7 | 15 | 18 | 33% | 61% | good [~] |
| `aws_api_gateway_rest_api_policy` | 1 | 3 | 2 | 0% | 100% | good [~] |
| `aws_api_gateway_stage` | 8 | 15 | 18 | 39% | 67% | good [~] |
| `aws_api_gateway_usage_plan` | 6 | 15 | 9 | 56% | 78% | good [~] |
| `aws_api_gateway_usage_plan_key` | 1 | 6 | 5 | 0% | 100% | good [~] |
| `aws_api_gateway_vpc_link` | 6 | 7 | 6 | 50% | 67% | good [~] |
| `aws_apigatewayv2_api` | 1 | 13 | 19 | 0% | 53% | good [~] |
| `aws_apigatewayv2_api_mapping` | 1 | 6 | 4 | 0% | 100% | good [~] |
| `aws_apigatewayv2_authorizer` | 4 | 12 | 11 | 27% | 82% | good [~] |
| `aws_apigatewayv2_deployment` | 1 | 6 | 4 | 0% | 75% | good [~] |
| `aws_apigatewayv2_domain_name` | 2 | 8 | 8 | 12% | 50% | good [~] |
| `aws_apigatewayv2_integration` | 1 | 9 | 19 | 0% | 37% | good [~] |
| `aws_apigatewayv2_integration_response` | 3 | 9 | 6 | 17% | 100% | good [~] |
| `aws_apigatewayv2_model` | 1 | 7 | 5 | 0% | 100% | good [~] |
| `aws_apigatewayv2_route` | 1 | 7 | 12 | 0% | 42% | good [~] |
| `aws_apigatewayv2_route_response` | 1 | 6 | 5 | 0% | 80% | good [~] |
| `aws_apigatewayv2_stage` | 2 | 9 | 15 | 7% | 53% | good [~] |
| `aws_apigatewayv2_vpc_link` | 3 | 7 | 6 | 33% | 83% | good [~] |
| `aws_appconfig_application` | 1 | 4 | 5 | 0% | 60% | good [~] |
| `aws_appconfig_configuration_profile` | 1 | 8 | 12 | 0% | 58% | good [~] |
| `aws_appconfig_deployment` | 1 | 9 | 13 | 0% | 62% | good [~] |
| `aws_appconfig_deployment_strategy` | 1 | 8 | 10 | 0% | 70% | good [~] |
| `aws_appconfig_environment` | 1 | 9 | 9 | 0% | 67% | good [~] |
| `aws_appconfig_extension` | 1 | 5 | 8 | 0% | 50% | good [~] |
| `aws_appconfig_extension_association` | 1 | 6 | 5 | 0% | 100% | good [~] |
| `aws_appconfig_hosted_configuration_version` | 1 | 6 | 7 | 0% | 71% | good [~] |
| `aws_appfabric_app_authorization` | 1 | 0 | 13 | 0% | 0% | poor [!] |
| `aws_appfabric_app_authorization_connection` | 1 | 0 | 6 | 0% | 0% | poor [!] |
| `aws_appfabric_app_bundle` | 1 | 4 | 4 | 0% | 75% | good [~] |
| `aws_appfabric_ingestion` | 1 | 0 | 7 | 0% | 0% | poor [!] |
| `aws_appfabric_ingestion_destination` | 1 | 0 | 8 | 0% | 0% | poor [!] |
| `aws_appflow_flow` | 6 | 10 | 12 | 42% | 75% | good [~] |
| `aws_appautoscaling_policy` | 3 | 7 | 9 | 22% | 67% | good [~] |
| `aws_appautoscaling_target` | 2 | 12 | 10 | 10% | 80% | good [~] |
| `aws_applicationcostprofiler_report_definition` | 1 | 8 | 0 | 0% | 0% | n/a [?] |
| `aws_appmesh_gateway_route` | 3 | 11 | 11 | 9% | 91% | good [~] |
| `aws_appmesh_mesh` | 3 | 11 | 9 | 11% | 89% | good [~] |
| `aws_appmesh_route` | 3 | 11 | 11 | 9% | 91% | good [~] |
| `aws_appmesh_virtual_gateway` | 3 | 10 | 10 | 10% | 90% | good [~] |
| `aws_appmesh_virtual_node` | 3 | 10 | 10 | 10% | 90% | good [~] |
| `aws_appmesh_virtual_router` | 3 | 10 | 10 | 10% | 90% | good [~] |
| `aws_appmesh_virtual_service` | 3 | 10 | 10 | 10% | 90% | good [~] |
| `aws_apprunner_auto_scaling_configuration_version` | 1 | 11 | 12 | 0% | 83% | good [~] |
| `aws_apprunner_connection` | 1 | 7 | 6 | 0% | 100% | good [~] |
| `aws_apprunner_custom_domain_association` | 1 | 0 | 6 | 0% | 0% | poor [!] |
| `aws_apprunner_default_auto_scaling_configuration_version` | 1 | 2 | 1 | 0% | 100% | good [~] |
| `aws_apprunner_deployment` | 1 | 0 | 4 | 0% | 0% | poor [!] |
| `aws_apprunner_observability_configuration` | 1 | 0 | 8 | 0% | 0% | poor [!] |
| `aws_apprunner_service` | 7 | 13 | 14 | 43% | 86% | good [~] |
| `aws_apprunner_vpc_connector` | 1 | 0 | 8 | 0% | 0% | poor [!] |
| `aws_apprunner_vpc_ingress_connection` | 1 | 0 | 8 | 0% | 0% | poor [!] |
| `aws_appsync_api_cache` | 1 | 7 | 6 | 0% | 100% | good [~] |
| `aws_appsync_api_key` | 2 | 5 | 5 | 20% | 60% | good [~] |
| `aws_appsync_graphql_api` | 5 | 15 | 20 | 20% | 70% | good [~] |
| `aws_appsync_type` | 1 | 6 | 6 | 0% | 83% | good [~] |
| `aws_athena_capacity_reservation` | 1 | 6 | 8 | 0% | 62% | good [~] |
| `aws_athena_data_catalog` | 2 | 6 | 7 | 14% | 71% | good [~] |
| `aws_athena_database` | 1 | 0 | 8 | 0% | 0% | poor [!] |
| `aws_athena_named_query` | 1 | 6 | 5 | 0% | 100% | good [~] |
| `aws_athena_prepared_statement` | 1 | 5 | 5 | 0% | 80% | good [~] |
| `aws_athena_workgroup` | 3 | 6 | 8 | 25% | 62% | good [~] |
| `aws_auditmanager_account_registration` | 1 | 2 | 4 | 0% | 25% | fair [-] |
| `aws_auditmanager_assessment` | 1 | 8 | 11 | 0% | 64% | good [~] |
| `aws_auditmanager_assessment_delegation` | 1 | 0 | 7 | 0% | 0% | poor [!] |
| `aws_auditmanager_assessment_report` | 1 | 0 | 5 | 0% | 0% | poor [!] |
| `aws_auditmanager_control` | 2 | 7 | 10 | 10% | 60% | good [~] |
| `aws_auditmanager_framework` | 2 | 6 | 8 | 12% | 62% | good [~] |
| `aws_auditmanager_framework_share` | 1 | 0 | 5 | 0% | 0% | poor [!] |
| `aws_auditmanager_organization_admin_account_registration` | 1 | 0 | 2 | 0% | 0% | poor [!] |
| `aws_autoscaling_attachment` | 1 | 0 | 3 | 0% | 0% | poor [!] |
| `aws_autoscaling_group` | 4 | 14 | 45 | 2% | 27% | fair [-] |
| `aws_autoscaling_group_tag` | 1 | 0 | 2 | 0% | 0% | poor [!] |
| `aws_autoscaling_lifecycle_hook` | 2 | 9 | 8 | 12% | 100% | good [~] |
| `aws_autoscaling_notification` | 1 | 0 | 3 | 0% | 0% | poor [!] |
| `aws_autoscaling_policy` | 4 | 8 | 14 | 21% | 50% | good [~] |
| `aws_autoscaling_schedule` | 4 | 10 | 10 | 30% | 90% | good [~] |
| `aws_autoscaling_traffic_source_attachment` | 2 | 0 | 3 | 33% | 0% | fair [-] |
| `aws_launch_configuration` | 2 | 11 | 19 | 5% | 53% | good [~] |
| `aws_backup_framework` | 2 | 10 | 10 | 10% | 90% | good [~] |
| `aws_backup_global_settings` | 1 | 0 | 1 | 0% | 0% | poor [!] |
| `aws_backup_logically_air_gapped_vault` | 3 | 0 | 7 | 29% | 0% | fair [-] |
| `aws_backup_plan` | 4 | 7 | 7 | 43% | 86% | good [~] |
| `aws_backup_region_settings` | 1 | 0 | 2 | 0% | 0% | poor [!] |
| `aws_backup_report_plan` | 3 | 10 | 9 | 22% | 100% | good [~] |
| `aws_backup_restore_testing_plan` | 3 | 9 | 8 | 25% | 100% | good [~] |
| `aws_backup_restore_testing_selection` | 5 | 9 | 8 | 50% | 100% | good [~] |
| `aws_backup_selection` | 1 | 0 | 7 | 0% | 0% | poor [!] |
| `aws_backup_vault` | 1 | 5 | 8 | 0% | 50% | good [~] |
| `aws_backup_vault_lock_configuration` | 3 | 0 | 5 | 40% | 0% | good [~] |
| `aws_backup_vault_notifications` | 2 | 5 | 4 | 25% | 100% | good [~] |
| `aws_backup_vault_policy` | 1 | 4 | 3 | 0% | 100% | good [~] |
| `aws_batch_compute_environment` | 6 | 12 | 14 | 36% | 71% | good [~] |
| `aws_batch_job_definition` | 9 | 18 | 18 | 44% | 61% | good [~] |
| `aws_batch_job_queue` | 6 | 11 | 11 | 45% | 73% | good [~] |
| `aws_batch_scheduling_policy` | 4 | 10 | 5 | 60% | 80% | excellent [+] |
| `aws_bedrock_custom_model` | 2 | 14 | 19 | 5% | 63% | good [~] |
| `aws_bedrock_guardrail` | 7 | 18 | 18 | 33% | 89% | good [~] |
| `aws_bedrock_guardrail_version` | 1 | 5 | 5 | 0% | 80% | good [~] |
| `aws_bedrock_inference_profile` | 1 | 13 | 12 | 0% | 83% | good [~] |
| `aws_bedrock_model_invocation_logging_configuration` | 2 | 11 | 1 | 100% | 100% | excellent [+] |
| `aws_bedrock_provisioned_model_throughput` | 1 | 7 | 8 | 0% | 75% | good [~] |
| `aws_bedrockagent_agent` | 6 | 20 | 20 | 15% | 65% | good [~] |
| `aws_bedrockagent_agent_action_group` | 4 | 14 | 13 | 23% | 77% | good [~] |
| `aws_bedrockagent_agent_alias` | 2 | 11 | 9 | 11% | 67% | good [~] |
| `aws_bedrockagent_agent_collaborator` | 2 | 11 | 9 | 11% | 78% | good [~] |
| `aws_bedrockagent_agent_knowledge_base_association` | 1 | 8 | 6 | 0% | 83% | good [~] |
| `aws_bedrockagent_data_source` | 4 | 12 | 9 | 33% | 89% | good [~] |
| `aws_bedrockagent_knowledge_base` | 4 | 12 | 12 | 25% | 67% | good [~] |
| `aws_bedrockagent_prompt` | 2 | 10 | 11 | 9% | 82% | good [~] |
| `aws_budgets_budget` | 5 | 17 | 17 | 24% | 71% | good [~] |
| `aws_chatbot_microsoft_teams_channel_configuration` | 1 | 10 | 0 | 0% | 0% | n/a [?] |
| `aws_chatbot_slack_channel_configuration` | 1 | 8 | 14 | 0% | 57% | good [~] |
| `aws_cloudformation_stack` | 5 | 11 | 16 | 25% | 56% | good [~] |
| `aws_cloudformation_stack_instances` | 3 | 0 | 11 | 18% | 0% | poor [!] |
| `aws_cloudformation_stack_set` | 4 | 8 | 18 | 17% | 39% | good [~] |
| `aws_cloudformation_stack_set_instance` | 1 | 5 | 12 | 8% | 33% | good [~] |
| `aws_cloudformation_type` | 1 | 7 | 17 | 0% | 29% | fair [-] |
| `aws_cloudfront_cache_policy` | 1 | 7 | 8 | 0% | 75% | good [~] |
| `aws_cloudfront_continuous_deployment_policy` | 1 | 3 | 6 | 0% | 33% | good [~] |
| `aws_cloudfront_distribution` | 7 | 31 | 32 | 19% | 62% | good [~] |
| `aws_cloudfront_field_level_encryption_config` | 1 | 3 | 6 | 0% | 33% | good [~] |
| `aws_cloudfront_field_level_encryption_profile` | 1 | 4 | 6 | 0% | 50% | good [~] |
| `aws_cloudfront_function` | 1 | 8 | 10 | 0% | 70% | good [~] |
| `aws_cloudfront_key_group` | 2 | 5 | 4 | 25% | 100% | good [~] |
| `aws_cloudfront_key_value_store` | 1 | 6 | 6 | 0% | 67% | good [~] |
| `aws_cloudfront_monitoring_subscription` | 2 | 5 | 2 | 50% | 100% | good [~] |
| `aws_cloudfront_origin_access_control` | 1 | 7 | 7 | 0% | 86% | good [~] |
| `aws_cloudfront_origin_access_identity` | 1 | 7 | 7 | 0% | 86% | good [~] |
| `aws_cloudfront_origin_request_policy` | 1 | 4 | 7 | 0% | 43% | good [~] |
| `aws_cloudfront_public_key` | 1 | 6 | 6 | 0% | 83% | good [~] |
| `aws_cloudfront_realtime_log_config` | 2 | 5 | 5 | 20% | 80% | good [~] |
| `aws_cloudfront_response_headers_policy` | 1 | 4 | 9 | 0% | 33% | good [~] |
| `aws_cloudfront_vpc_origin` | 2 | 8 | 6 | 17% | 50% | good [~] |
| `aws_cloudhsm_v2_cluster` | 4 | 20 | 12 | 25% | 58% | good [~] |
| `aws_cloudtrail` | 4 | 20 | 20 | 15% | 60% | good [~] |
| `aws_cloudwatch_composite_alarm` | 1 | 9 | 11 | 0% | 73% | good [~] |
| `aws_cloudwatch_dashboard` | 1 | 4 | 3 | 0% | 100% | good [~] |
| `aws_cloudwatch_metric_alarm` | 6 | 17 | 24 | 21% | 67% | good [~] |
| `aws_cloudwatch_metric_stream` | 1 | 9 | 16 | 0% | 50% | good [~] |
| `aws_codeartifact_domain` | 1 | 7 | 10 | 0% | 60% | good [~] |
| `aws_codeartifact_repository` | 3 | 9 | 10 | 20% | 80% | good [~] |
| `aws_codebuild_fleet` | 0 | 0 | 16 | 0% | 0% | poor [!] |
| `aws_codebuild_project` | 12 | 29 | 27 | 41% | 81% | good [~] |
| `aws_codebuild_report_group` | 0 | 0 | 8 | 0% | 0% | poor [!] |
| `aws_codebuild_resource_policy` | 0 | 0 | 2 | 0% | 0% | poor [!] |
| `aws_codebuild_source_credential` | 0 | 0 | 5 | 0% | 0% | poor [!] |
| `aws_codebuild_webhook` | 0 | 0 | 9 | 0% | 0% | poor [!] |
| `aws_codecommit_repository` | 1 | 7 | 10 | 0% | 60% | good [~] |
| `aws_codedeploy_app` | 1 | 5 | 8 | 0% | 50% | good [~] |
| `aws_codedeploy_deployment_group` | 8 | 23 | 22 | 32% | 82% | good [~] |
| `aws_codepipeline` | 5 | 12 | 12 | 33% | 92% | good [~] |
| `aws_cognito_identity_pool` | 5 | 13 | 11 | 36% | 82% | good [~] |
| `aws_cognito_identity_pool_provider_principal_tag` | 2 | 5 | 4 | 25% | 100% | good [~] |
| `aws_cognito_identity_pool_roles_attachment` | 3 | 4 | 3 | 67% | 100% | excellent [+] |
| `aws_cognito_identity_provider` | 4 | 7 | 6 | 50% | 100% | good [~] |
| `aws_cognito_managed_user_pool_client` | 9 | 0 | 25 | 32% | 0% | fair [-] |
| `aws_cognito_resource_server` | 2 | 8 | 5 | 20% | 100% | good [~] |
| `aws_cognito_user_group` | 2 | 6 | 5 | 20% | 100% | good [~] |
| `aws_cognito_user_in_group` | 1 | 3 | 3 | 0% | 100% | good [~] |
| `aws_cognito_user_pool` | 18 | 22 | 36 | 44% | 53% | good [~] |
| `aws_cognito_user_pool_client` | 12 | 14 | 24 | 46% | 50% | good [~] |
| `aws_cognito_user_pool_domain` | 1 | 6 | 10 | 0% | 50% | good [~] |
| `aws_cognito_user_pool_ui_customization` | 1 | 0 | 8 | 0% | 0% | poor [!] |
| `aws_comprehend_entity_recognizer` | 2 | 14 | 13 | 8% | 54% | good [~] |
| `aws_config_aggregate_authorization` | 1 | 4 | 5 | 20% | 60% | good [~] |
| `aws_config_config_rule` | 4 | 12 | 11 | 27% | 82% | good [~] |
| `aws_config_configuration_aggregator` | 1 | 3 | 6 | 0% | 33% | good [~] |
| `aws_config_configuration_recorder` | 3 | 4 | 4 | 50% | 75% | good [~] |
| `aws_config_configuration_recorder_status` | 1 | 3 | 2 | 0% | 100% | good [~] |
| `aws_config_conformance_pack` | 1 | 0 | 7 | 0% | 0% | poor [!] |
| `aws_config_delivery_channel` | 2 | 5 | 6 | 17% | 67% | good [~] |
| `aws_config_organization_conformance_pack` | 1 | 5 | 9 | 0% | 44% | good [~] |
| `aws_config_organization_custom_policy_rule` | 1 | 8 | 15 | 0% | 47% | good [~] |
| `aws_config_organization_custom_rule` | 1 | 7 | 13 | 0% | 46% | good [~] |
| `aws_config_organization_managed_rule` | 1 | 7 | 12 | 0% | 50% | good [~] |
| `aws_config_remediation_configuration` | 1 | 9 | 11 | 0% | 73% | good [~] |
| `aws_config_retention_configuration` | 1 | 3 | 2 | 0% | 100% | good [~] |
| `aws_connect_instance` | 1 | 11 | 17 | 0% | 59% | good [~] |
| `aws_ce_anomaly_monitor` | 1 | 8 | 7 | 0% | 57% | good [~] |
| `aws_ce_anomaly_subscription` | 4 | 11 | 9 | 22% | 67% | good [~] |
| `aws_datapipeline_pipeline` | 1 | 4 | 4 | 0% | 75% | good [~] |
| `aws_datasync_agent` | 1 | 0 | 11 | 0% | 0% | poor [!] |
| `aws_datasync_location_azure_blob` | 1 | 0 | 11 | 0% | 0% | poor [!] |
| `aws_datasync_location_efs` | 1 | 0 | 10 | 0% | 0% | poor [!] |
| `aws_datasync_location_fsx_lustre_file_system` | 1 | 0 | 8 | 0% | 0% | poor [!] |
| `aws_datasync_location_fsx_ontap_file_system` | 1 | 0 | 10 | 0% | 0% | poor [!] |
| `aws_datasync_location_fsx_openzfs_file_system` | 1 | 0 | 9 | 0% | 0% | poor [!] |
| `aws_datasync_location_fsx_windows_file_system` | 1 | 0 | 11 | 0% | 0% | poor [!] |
| `aws_datasync_location_hdfs` | 1 | 0 | 18 | 0% | 0% | poor [!] |
| `aws_datasync_location_nfs` | 1 | 0 | 8 | 0% | 0% | poor [!] |
| `aws_datasync_location_object_storage` | 1 | 0 | 13 | 0% | 0% | poor [!] |
| `aws_datasync_location_s3` | 2 | 7 | 9 | 11% | 56% | good [~] |
| `aws_datasync_location_smb` | 1 | 0 | 11 | 0% | 0% | poor [!] |
| `aws_datasync_task` | 5 | 14 | 14 | 29% | 79% | good [~] |
| `aws_dax_cluster` | 3 | 14 | 21 | 10% | 52% | good [~] |
| `aws_dax_parameter_group` | 1 | 3 | 3 | 0% | 67% | good [~] |
| `aws_dax_subnet_group` | 2 | 5 | 4 | 25% | 100% | good [~] |
| `aws_dx_connection` | 1 | 12 | 18 | 0% | 50% | good [~] |
| `aws_dx_connection_association` | 1 | 0 | 2 | 0% | 0% | poor [!] |
| `aws_dx_connection_confirmation` | 1 | 0 | 1 | 0% | 0% | poor [!] |
| `aws_dx_hosted_connection` | 1 | 0 | 15 | 7% | 0% | poor [!] |
| `aws_dx_macsec_key_association` | 1 | 0 | 6 | 0% | 0% | poor [!] |
| `aws_directory_service_conditional_forwarder` | 1 | 0 | 3 | 0% | 0% | poor [!] |
| `aws_directory_service_directory` | 6 | 23 | 18 | 17% | 67% | good [~] |
| `aws_directory_service_log_subscription` | 1 | 0 | 2 | 0% | 0% | poor [!] |
| `aws_directory_service_radius_settings` | 1 | 0 | 10 | 0% | 0% | poor [!] |
| `aws_directory_service_region` | 1 | 0 | 7 | 0% | 0% | poor [!] |
| `aws_directory_service_shared_directory` | 1 | 0 | 6 | 0% | 0% | poor [!] |
| `aws_directory_service_shared_directory_accepter` | 1 | 0 | 6 | 0% | 0% | poor [!] |
| `aws_directory_service_trust` | 1 | 0 | 13 | 0% | 0% | poor [!] |
| `aws_dms_certificate` | 0 | 0 | 6 | 0% | 0% | poor [!] |
| `aws_dms_endpoint` | 8 | 20 | 28 | 25% | 64% | good [~] |
| `aws_dms_event_subscription` | 0 | 0 | 10 | 0% | 0% | poor [!] |
| `aws_dms_replication_config` | 0 | 0 | 14 | 0% | 0% | poor [!] |
| `aws_dms_replication_instance` | 2 | 14 | 21 | 0% | 52% | good [~] |
| `aws_dms_replication_subnet_group` | 0 | 0 | 7 | 0% | 0% | poor [!] |
| `aws_dms_replication_task` | 3 | 13 | 15 | 0% | 67% | good [~] |
| `aws_dms_s3_endpoint` | 0 | 0 | 54 | 0% | 0% | poor [!] |
| `aws_dsql_cluster` | 1 | 8 | 10 | 0% | 50% | good [~] |
| `aws_dynamodb_contributor_insights` | 1 | 3 | 3 | 0% | 67% | good [~] |
| `aws_dynamodb_global_table` | 2 | 5 | 4 | 25% | 75% | good [~] |
| `aws_dynamodb_kinesis_streaming_destination` | 1 | 4 | 3 | 0% | 100% | good [~] |
| `aws_dynamodb_resource_policy` | 1 | 4 | 4 | 0% | 75% | good [~] |
| `aws_dynamodb_table` | 7 | 26 | 29 | 21% | 72% | good [~] |
| `aws_dynamodb_table_export` | 1 | 0 | 18 | 0% | 0% | poor [!] |
| `aws_dynamodb_table_item` | 1 | 5 | 4 | 0% | 100% | good [~] |
| `aws_dynamodb_table_replica` | 1 | 0 | 9 | 0% | 0% | poor [!] |
| `aws_dynamodb_tag` | 1 | 0 | 3 | 0% | 0% | poor [!] |
| `aws_ebs_snapshot` | 3 | 12 | 16 | 0% | 50% | good [~] |
| `aws_ebs_volume` | 4 | 13 | 16 | 12% | 62% | good [~] |
| `aws_ami` | 3 | 13 | 32 | 6% | 38% | good [~] |
| `aws_ami_copy` | 3 | 0 | 37 | 5% | 0% | poor [!] |
| `aws_ami_from_instance` | 3 | 8 | 34 | 6% | 21% | fair [-] |
| `aws_ami_launch_permission` | 1 | 4 | 5 | 0% | 60% | good [~] |
| `aws_customer_gateway` | 4 | 7 | 9 | 33% | 67% | good [~] |
| `aws_default_network_acl` | 3 | 0 | 9 | 22% | 0% | fair [-] |
| `aws_default_route_table` | 3 | 0 | 9 | 22% | 0% | fair [-] |
| `aws_default_security_group` | 3 | 0 | 11 | 18% | 0% | poor [!] |
| `aws_default_subnet` | 3 | 0 | 24 | 8% | 0% | poor [!] |
| `aws_default_vpc` | 3 | 0 | 22 | 9% | 0% | poor [!] |
| `aws_default_vpc_dhcp_options` | 3 | 0 | 10 | 20% | 0% | fair [-] |
| `aws_ebs_default_kms_key` | 1 | 0 | 1 | 0% | 0% | poor [!] |
| `aws_ebs_encryption_by_default` | 1 | 0 | 1 | 0% | 0% | poor [!] |
| `aws_ebs_fast_snapshot_restore` | 1 | 4 | 4 | 0% | 75% | good [~] |
| `aws_ebs_snapshot_block_public_access` | 1 | 0 | 1 | 0% | 0% | poor [!] |
| `aws_ebs_snapshot_copy` | 2 | 0 | 19 | 0% | 0% | poor [!] |
| `aws_ebs_snapshot_import` | 4 | 7 | 19 | 5% | 26% | fair [-] |
| `aws_ec2_availability_zone_group` | 1 | 0 | 2 | 0% | 0% | poor [!] |
| `aws_ec2_capacity_block_reservation` | 4 | 15 | 18 | 17% | 50% | good [~] |
| `aws_ec2_capacity_reservation` | 4 | 18 | 17 | 18% | 94% | good [~] |
| `aws_ec2_carrier_gateway` | 3 | 3 | 5 | 40% | 40% | good [~] |
| `aws_ec2_client_vpn_authorization_rule` | 1 | 0 | 6 | 0% | 0% | poor [!] |
| `aws_ec2_client_vpn_endpoint` | 3 | 12 | 22 | 9% | 50% | good [~] |
| `aws_ec2_client_vpn_network_association` | 1 | 0 | 5 | 0% | 0% | poor [!] |
| `aws_ec2_client_vpn_route` | 1 | 0 | 7 | 0% | 0% | poor [!] |
| `aws_ec2_default_credit_specification` | 1 | 0 | 3 | 0% | 0% | poor [!] |
| `aws_ec2_fleet` | 3 | 4 | 20 | 10% | 15% | fair [-] |
| `aws_ec2_host` | 3 | 7 | 12 | 17% | 42% | good [~] |
| `aws_ec2_image_block_public_access` | 1 | 0 | 2 | 0% | 0% | poor [!] |
| `aws_ec2_instance_connect_endpoint` | 4 | 13 | 13 | 23% | 92% | good [~] |
| `aws_ec2_instance_metadata_defaults` | 1 | 0 | 4 | 0% | 0% | poor [!] |
| `aws_ec2_instance_state` | 1 | 0 | 4 | 0% | 0% | poor [!] |
| `aws_ec2_local_gateway_route` | 1 | 7 | 3 | 0% | 100% | good [~] |
| `aws_ec2_local_gateway_route_table_vpc_association` | 3 | 9 | 5 | 40% | 100% | good [~] |
| `aws_ec2_managed_prefix_list` | 4 | 8 | 9 | 33% | 56% | good [~] |
| `aws_ec2_managed_prefix_list_entry` | 1 | 0 | 3 | 0% | 0% | poor [!] |
| `aws_ec2_network_insights_analysis` | 4 | 10 | 15 | 20% | 53% | good [~] |
| `aws_ec2_network_insights_path` | 4 | 13 | 13 | 23% | 85% | good [~] |
| `aws_ec2_serial_console_access` | 1 | 0 | 1 | 0% | 0% | poor [!] |
| `aws_ec2_subnet_cidr_reservation` | 1 | 0 | 5 | 0% | 0% | poor [!] |
| `aws_ec2_tag` | 1 | 0 | 3 | 0% | 0% | poor [!] |
| `aws_ec2_traffic_mirror_filter` | 3 | 5 | 5 | 40% | 80% | good [~] |
| `aws_ec2_traffic_mirror_filter_rule` | 5 | 13 | 11 | 18% | 91% | good [~] |
| `aws_ec2_traffic_mirror_session` | 6 | 11 | 11 | 45% | 91% | good [~] |
| `aws_ec2_traffic_mirror_target` | 3 | 9 | 8 | 25% | 88% | good [~] |
| `aws_ec2_transit_gateway` | 3 | 10 | 17 | 12% | 53% | good [~] |
| `aws_ec2_transit_gateway_connect` | 3 | 8 | 8 | 25% | 62% | good [~] |
| `aws_ec2_transit_gateway_connect_peer` | 4 | 9 | 11 | 27% | 64% | good [~] |
| `aws_ec2_transit_gateway_default_route_table_association` | 1 | 0 | 4 | 0% | 0% | poor [!] |
| `aws_ec2_transit_gateway_default_route_table_propagation` | 1 | 0 | 4 | 0% | 0% | poor [!] |
| `aws_ec2_transit_gateway_multicast_domain` | 3 | 11 | 9 | 22% | 89% | good [~] |
| `aws_ec2_transit_gateway_multicast_domain_association` | 1 | 4 | 4 | 0% | 75% | good [~] |
| `aws_ec2_transit_gateway_multicast_group_member` | 1 | 4 | 3 | 0% | 100% | good [~] |
| `aws_ec2_transit_gateway_multicast_group_source` | 1 | 4 | 3 | 0% | 100% | good [~] |
| `aws_ec2_transit_gateway_peering_attachment` | 3 | 8 | 9 | 22% | 78% | good [~] |
| `aws_ec2_transit_gateway_peering_attachment_accepter` | 1 | 0 | 7 | 0% | 0% | poor [!] |
| `aws_ec2_transit_gateway_policy_table` | 3 | 6 | 5 | 40% | 80% | good [~] |
| `aws_ec2_transit_gateway_policy_table_association` | 1 | 4 | 4 | 0% | 75% | good [~] |
| `aws_ec2_transit_gateway_prefix_list_reference` | 1 | 6 | 5 | 0% | 100% | good [~] |
| `aws_ec2_transit_gateway_route` | 1 | 5 | 4 | 0% | 100% | good [~] |
| `aws_ec2_transit_gateway_route_table` | 3 | 7 | 6 | 33% | 100% | good [~] |
| `aws_ec2_transit_gateway_route_table_association` | 1 | 0 | 5 | 0% | 0% | poor [!] |
| `aws_ec2_transit_gateway_route_table_propagation` | 1 | 0 | 4 | 0% | 0% | poor [!] |
| `aws_ec2_transit_gateway_vpc_attachment` | 4 | 7 | 13 | 23% | 46% | good [~] |
| `aws_ec2_transit_gateway_vpc_attachment_accepter` | 1 | 0 | 13 | 0% | 0% | poor [!] |
| `aws_egress_only_internet_gateway` | 3 | 4 | 3 | 67% | 100% | excellent [+] |
| `aws_eip` | 8 | 13 | 23 | 30% | 52% | good [~] |
| `aws_eip_association` | 1 | 7 | 6 | 0% | 100% | good [~] |
| `aws_eip_domain_name` | 1 | 4 | 4 | 0% | 75% | good [~] |
| `aws_flow_log` | 3 | 13 | 17 | 12% | 71% | good [~] |
| `aws_instance` | 3 | 22 | 59 | 3% | 36% | good [~] |
| `aws_internet_gateway` | 3 | 6 | 6 | 33% | 83% | good [~] |
| `aws_internet_gateway_attachment` | 1 | 3 | 3 | 0% | 67% | good [~] |
| `aws_key_pair` | 3 | 7 | 9 | 22% | 67% | good [~] |
| `aws_launch_template` | 3 | 7 | 40 | 5% | 15% | fair [-] |
| `aws_main_route_table_association` | 1 | 3 | 4 | 0% | 50% | good [~] |
| `aws_nat_gateway` | 3 | 11 | 13 | 15% | 54% | good [~] |
| `aws_network_acl` | 4 | 9 | 8 | 38% | 100% | good [~] |
| `aws_network_acl_association` | 1 | 3 | 2 | 0% | 100% | good [~] |
| `aws_network_acl_rule` | 9 | 0 | 11 | 73% | 0% | good [~] |
| `aws_network_interface` | 4 | 12 | 27 | 11% | 41% | good [~] |
| `aws_network_interface_attachment` | 1 | 6 | 5 | 0% | 100% | good [~] |
| `aws_network_interface_permission` | 1 | 4 | 5 | 0% | 60% | good [~] |
| `aws_network_interface_sg_attachment` | 1 | 3 | 3 | 0% | 67% | good [~] |
| `aws_placement_group` | 4 | 9 | 8 | 38% | 100% | good [~] |
| `aws_route` | 1 | 11 | 19 | 0% | 53% | good [~] |
| `aws_route_table` | 3 | 17 | 8 | 25% | 62% | good [~] |
| `aws_route_table_association` | 1 | 3 | 4 | 0% | 50% | good [~] |
| `aws_security_group` | 3 | 9 | 12 | 17% | 67% | good [~] |
| `aws_security_group_rule` | 1 | 0 | 13 | 0% | 0% | poor [!] |
| `aws_spot_datafeed_subscription` | 1 | 0 | 2 | 0% | 0% | poor [!] |
| `aws_spot_fleet_request` | 3 | 4 | 29 | 7% | 10% | poor [!] |
| `aws_spot_instance_request` | 3 | 6 | 67 | 3% | 7% | poor [!] |
| `aws_subnet` | 5 | 17 | 22 | 14% | 55% | good [~] |
| `aws_verifiedaccess_endpoint` | 5 | 17 | 20 | 15% | 55% | good [~] |
| `aws_verifiedaccess_group` | 5 | 10 | 12 | 25% | 75% | good [~] |
| `aws_verifiedaccess_instance` | 3 | 10 | 9 | 22% | 78% | good [~] |
| `aws_verifiedaccess_instance_logging_configuration` | 1 | 0 | 2 | 0% | 0% | poor [!] |
| `aws_verifiedaccess_instance_trust_provider_attachment` | 1 | 0 | 2 | 0% | 0% | poor [!] |
| `aws_verifiedaccess_trust_provider` | 3 | 10 | 12 | 17% | 58% | good [~] |
| `aws_volume_attachment` | 1 | 7 | 7 | 0% | 86% | good [~] |
| `aws_vpc` | 4 | 17 | 22 | 9% | 55% | good [~] |
| `aws_vpc_block_public_access_exclusion` | 3 | 0 | 7 | 29% | 0% | fair [-] |
| `aws_vpc_block_public_access_options` | 1 | 0 | 4 | 0% | 0% | poor [!] |
| `aws_vpc_dhcp_options` | 3 | 11 | 10 | 20% | 100% | good [~] |
| `aws_vpc_dhcp_options_association` | 1 | 3 | 2 | 0% | 100% | good [~] |
| `aws_vpc_endpoint` | 3 | 15 | 26 | 8% | 54% | good [~] |
| `aws_vpc_endpoint_connection_accepter` | 1 | 4 | 3 | 0% | 100% | good [~] |
| `aws_vpc_endpoint_connection_notification` | 1 | 7 | 6 | 0% | 100% | good [~] |
| `aws_vpc_endpoint_policy` | 1 | 0 | 3 | 0% | 0% | poor [!] |
| `aws_vpc_endpoint_private_dns` | 1 | 0 | 2 | 0% | 0% | poor [!] |
| `aws_vpc_endpoint_route_table_association` | 1 | 0 | 2 | 0% | 0% | poor [!] |
| `aws_vpc_endpoint_security_group_association` | 1 | 0 | 3 | 0% | 0% | poor [!] |
| `aws_vpc_endpoint_service` | 3 | 12 | 18 | 11% | 56% | good [~] |
| `aws_vpc_endpoint_service_allowed_principal` | 1 | 0 | 2 | 0% | 0% | poor [!] |
| `aws_vpc_endpoint_service_private_dns_verification` | 1 | 0 | 3 | 0% | 0% | poor [!] |
| `aws_vpc_endpoint_subnet_association` | 1 | 0 | 3 | 0% | 0% | poor [!] |
| `aws_vpc_ipam` | 4 | 18 | 14 | 21% | 86% | good [~] |
| `aws_vpc_ipam_pool` | 7 | 22 | 21 | 29% | 90% | good [~] |
| `aws_vpc_ipam_pool_cidr` | 2 | 6 | 6 | 17% | 67% | good [~] |
| `aws_vpc_ipam_pool_cidr_allocation` | 1 | 8 | 9 | 0% | 67% | good [~] |
| `aws_vpc_ipam_resource_discovery` | 4 | 10 | 9 | 33% | 89% | good [~] |
| `aws_vpc_ipam_resource_discovery_association` | 3 | 0 | 11 | 18% | 0% | poor [!] |
| `aws_vpc_ipam_scope` | 3 | 10 | 10 | 20% | 80% | good [~] |
| `aws_vpc_ipv4_cidr_block_association` | 1 | 0 | 5 | 0% | 0% | poor [!] |
| `aws_vpc_ipv6_cidr_block_association` | 1 | 0 | 9 | 0% | 0% | poor [!] |
| `aws_vpc_network_performance_metric_subscription` | 1 | 0 | 5 | 0% | 0% | poor [!] |
| `aws_vpc_peering_connection` | 3 | 5 | 11 | 18% | 36% | good [~] |
| `aws_vpc_peering_connection_accepter` | 1 | 0 | 12 | 0% | 0% | poor [!] |
| `aws_vpc_peering_connection_options` | 1 | 0 | 3 | 0% | 0% | poor [!] |
| `aws_vpc_route_server` | 3 | 0 | 10 | 20% | 0% | fair [-] |
| `aws_vpc_route_server_endpoint` | 3 | 0 | 10 | 20% | 0% | fair [-] |
| `aws_vpc_route_server_peer` | 3 | 0 | 13 | 15% | 0% | poor [!] |
| `aws_vpc_route_server_propagation` | 1 | 0 | 3 | 0% | 0% | poor [!] |
| `aws_vpc_route_server_vpc_association` | 1 | 0 | 3 | 0% | 0% | poor [!] |
| `aws_vpc_security_group_egress_rule` | 0 | 0 | 13 | 0% | 0% | poor [!] |
| `aws_vpc_security_group_ingress_rule` | 1 | 0 | 13 | 0% | 0% | poor [!] |
| `aws_vpc_security_group_vpc_association` | 1 | 4 | 4 | 0% | 75% | good [~] |
| `aws_vpn_connection` | 3 | 15 | 74 | 3% | 18% | fair [-] |
| `aws_vpn_connection_route` | 1 | 3 | 2 | 0% | 100% | good [~] |
| `aws_vpn_gateway` | 3 | 6 | 6 | 33% | 83% | good [~] |
| `aws_vpn_gateway_attachment` | 1 | 3 | 2 | 0% | 100% | good [~] |
| `aws_vpn_gateway_route_propagation` | 1 | 0 | 3 | 0% | 0% | poor [!] |
| `aws_ec2_instance_connect_endpoint` | 1 | 14 | 13 | 0% | 77% | good [~] |
| `aws_ecr_account_setting` | 1 | 3 | 2 | 0% | 100% | good [~] |
| `aws_ecr_lifecycle_policy` | 1 | 4 | 3 | 0% | 100% | good [~] |
| `aws_ecr_pull_through_cache_rule` | 1 | 7 | 6 | 0% | 100% | good [~] |
| `aws_ecr_registry_policy` | 1 | 3 | 2 | 0% | 100% | good [~] |
| `aws_ecr_registry_scanning_configuration` | 1 | 8 | 3 | 0% | 100% | good [~] |
| `aws_ecr_replication_configuration` | 1 | 9 | 2 | 0% | 100% | good [~] |
| `aws_ecr_repository` | 3 | 11 | 11 | 18% | 73% | good [~] |
| `aws_ecr_repository_creation_template` | 1 | 8 | 10 | 0% | 70% | good [~] |
| `aws_ecr_repository_policy` | 1 | 4 | 3 | 0% | 100% | good [~] |
| `aws_ecs_account_setting_default` | 0 | 0 | 3 | 0% | 0% | poor [!] |
| `aws_ecs_capacity_provider` | 0 | 0 | 5 | 0% | 0% | poor [!] |
| `aws_ecs_cluster` | 5 | 6 | 7 | 43% | 57% | good [~] |
| `aws_ecs_cluster_capacity_providers` | 0 | 0 | 3 | 0% | 0% | poor [!] |
| `aws_ecs_service` | 18 | 27 | 34 | 44% | 50% | good [~] |
| `aws_ecs_tag` | 0 | 0 | 3 | 0% | 0% | poor [!] |
| `aws_ecs_task_definition` | 8 | 14 | 24 | 29% | 50% | good [~] |
| `aws_ecs_task_set` | 0 | 0 | 20 | 0% | 0% | poor [!] |
| `aws_efs_access_point` | 0 | 0 | 8 | 0% | 0% | poor [!] |
| `aws_efs_backup_policy` | 0 | 0 | 2 | 0% | 0% | poor [!] |
| `aws_efs_file_system` | 5 | 20 | 18 | 22% | 61% | good [~] |
| `aws_efs_file_system_policy` | 0 | 0 | 3 | 0% | 0% | poor [!] |
| `aws_efs_mount_target` | 0 | 0 | 12 | 0% | 0% | poor [!] |
| `aws_efs_replication_configuration` | 0 | 0 | 7 | 0% | 0% | poor [!] |
| `aws_eks_access_entry` | 3 | 11 | 11 | 18% | 91% | good [~] |
| `aws_eks_access_policy_association` | 2 | 9 | 7 | 14% | 86% | good [~] |
| `aws_eks_addon` | 7 | 11 | 16 | 38% | 56% | good [~] |
| `aws_eks_cluster` | 12 | 19 | 27 | 41% | 52% | good [~] |
| `aws_eks_fargate_profile` | 5 | 12 | 10 | 40% | 90% | good [~] |
| `aws_eks_identity_provider_config` | 4 | 10 | 7 | 43% | 86% | good [~] |
| `aws_eks_node_group` | 12 | 17 | 25 | 44% | 52% | good [~] |
| `aws_eks_pod_identity_association` | 3 | 9 | 8 | 25% | 100% | good [~] |
| `aws_elasticache_cluster` | 15 | 22 | 36 | 39% | 56% | good [~] |
| `aws_elasticache_global_replication_group` | 8 | 0 | 18 | 39% | 0% | fair [-] |
| `aws_elasticache_parameter_group` | 4 | 7 | 7 | 43% | 86% | good [~] |
| `aws_elasticache_replication_group` | 26 | 28 | 48 | 52% | 52% | good [~] |
| `aws_elasticache_reserved_cache_node` | 3 | 0 | 15 | 13% | 0% | poor [!] |
| `aws_elasticache_serverless_cache` | 7 | 0 | 21 | 29% | 0% | fair [-] |
| `aws_elasticache_subnet_group` | 4 | 6 | 7 | 43% | 71% | good [~] |
| `aws_elasticache_user` | 6 | 10 | 11 | 45% | 73% | good [~] |
| `aws_elasticache_user_group` | 3 | 0 | 6 | 33% | 0% | fair [-] |
| `aws_elasticache_user_group_association` | 1 | 0 | 3 | 0% | 0% | poor [!] |
| `aws_elastic_beanstalk_application` | 3 | 8 | 6 | 33% | 83% | good [~] |
| `aws_elastic_beanstalk_environment` | 4 | 23 | 24 | 8% | 62% | good [~] |
| `aws_elb` | 6 | 27 | 23 | 22% | 70% | good [~] |
| `aws_alb` | 0 | 0 | 36 | 0% | 0% | poor [!] |
| `aws_alb_listener` | 0 | 0 | 32 | 0% | 0% | poor [!] |
| `aws_alb_listener_certificate` | 0 | 0 | 2 | 0% | 0% | poor [!] |
| `aws_alb_listener_rule` | 0 | 0 | 7 | 0% | 0% | poor [!] |
| `aws_alb_target_group` | 0 | 0 | 27 | 0% | 0% | poor [!] |
| `aws_alb_target_group_attachment` | 0 | 0 | 4 | 0% | 0% | poor [!] |
| `aws_lb` | 21 | 31 | 36 | 56% | 61% | good [~] |
| `aws_lb_cookie_stickiness_policy` | 1 | 0 | 4 | 0% | 0% | poor [!] |
| `aws_lb_listener` | 17 | 20 | 32 | 50% | 50% | good [~] |
| `aws_lb_listener_certificate` | 1 | 3 | 2 | 0% | 100% | good [~] |
| `aws_lb_listener_rule` | 5 | 12 | 7 | 57% | 100% | good [~] |
| `aws_lb_ssl_negotiation_policy` | 1 | 0 | 5 | 0% | 0% | poor [!] |
| `aws_lb_target_group` | 15 | 27 | 27 | 52% | 63% | good [~] |
| `aws_lb_target_group_attachment` | 1 | 5 | 4 | 0% | 100% | good [~] |
| `aws_lb_trust_store` | 4 | 8 | 10 | 30% | 40% | good [~] |
| `aws_lb_trust_store_revocation` | 2 | 3 | 6 | 17% | 33% | good [~] |
| `aws_emr_block_public_access_configuration` | 2 | 5 | 2 | 50% | 100% | good [~] |
| `aws_emr_cluster` | 18 | 39 | 35 | 49% | 69% | good [~] |
| `aws_emr_instance_fleet` | 3 | 10 | 8 | 25% | 100% | good [~] |
| `aws_emr_instance_group` | 5 | 10 | 11 | 36% | 82% | good [~] |
| `aws_emr_managed_scaling_policy` | 2 | 10 | 2 | 50% | 100% | good [~] |
| `aws_emr_security_configuration` | 1 | 4 | 4 | 0% | 75% | good [~] |
| `aws_emr_studio` | 7 | 15 | 17 | 35% | 82% | good [~] |
| `aws_emr_studio_session_mapping` | 1 | 6 | 5 | 0% | 100% | good [~] |
| `aws_emrcontainers_virtual_cluster` | 2 | 8 | 6 | 17% | 67% | good [~] |
| `aws_emrserverless_application` | 10 | 21 | 14 | 64% | 50% | excellent [+] |
| `aws_cloudwatch_event_api_destination` | 1 | 8 | 7 | 0% | 100% | good [~] |
| `aws_cloudwatch_event_archive` | 1 | 7 | 6 | 0% | 100% | good [~] |
| `aws_cloudwatch_event_bus` | 4 | 7 | 8 | 38% | 62% | good [~] |
| `aws_cloudwatch_event_bus_policy` | 1 | 3 | 2 | 0% | 100% | good [~] |
| `aws_cloudwatch_event_connection` | 2 | 6 | 8 | 12% | 62% | good [~] |
| `aws_cloudwatch_event_endpoint` | 5 | 11 | 8 | 38% | 100% | good [~] |
| `aws_cloudwatch_event_permission` | 1 | 6 | 5 | 0% | 80% | good [~] |
| `aws_cloudwatch_event_rule` | 3 | 9 | 13 | 15% | 62% | good [~] |
| `aws_cloudwatch_event_target` | 3 | 11 | 20 | 5% | 50% | good [~] |
| `aws_kinesis_firehose_delivery_stream` | 1 | 12 | 20 | 0% | 50% | good [~] |
| `aws_fis_experiment_template` | 4 | 14 | 11 | 27% | 55% | good [~] |
| `aws_fsx_backup` | 3 | 5 | 9 | 22% | 44% | good [~] |
| `aws_fsx_lustre_file_system` | 6 | 28 | 36 | 11% | 61% | good [~] |
| `aws_fsx_ontap_file_system` | 5 | 25 | 26 | 15% | 77% | good [~] |
| `aws_fsx_openzfs_file_system` | 5 | 26 | 31 | 13% | 68% | good [~] |
| `aws_fsx_windows_file_system` | 10 | 26 | 30 | 30% | 70% | good [~] |
| `aws_glacier_vault` | 1 | 7 | 7 | 0% | 57% | good [~] |
| `aws_glue_catalog_database` | 2 | 7 | 11 | 9% | 55% | good [~] |
| `aws_glue_catalog_table` | 4 | 13 | 16 | 19% | 69% | good [~] |
| `aws_glue_catalog_table_optimizer` | 0 | 0 | 5 | 0% | 0% | poor [!] |
| `aws_glue_classifier` | 0 | 0 | 5 | 0% | 0% | poor [!] |
| `aws_glue_connection` | 4 | 10 | 11 | 27% | 82% | good [~] |
| `aws_glue_crawler` | 2 | 22 | 24 | 4% | 54% | good [~] |
| `aws_glue_data_catalog_encryption_settings` | 2 | 5 | 2 | 50% | 100% | good [~] |
| `aws_glue_data_quality_ruleset` | 0 | 0 | 10 | 0% | 0% | poor [!] |
| `aws_glue_dev_endpoint` | 5 | 19 | 25 | 16% | 64% | good [~] |
| `aws_glue_job` | 5 | 13 | 23 | 17% | 52% | good [~] |
| `aws_glue_ml_transform` | 8 | 14 | 16 | 38% | 81% | good [~] |
| `aws_glue_partition` | 5 | 10 | 9 | 33% | 89% | good [~] |
| `aws_glue_partition_index` | 0 | 0 | 5 | 0% | 0% | poor [!] |
| `aws_glue_registry` | 2 | 6 | 5 | 20% | 100% | good [~] |
| `aws_glue_resource_policy` | 1 | 5 | 2 | 0% | 50% | good [~] |
| `aws_glue_schema` | 2 | 14 | 13 | 8% | 92% | good [~] |
| `aws_glue_security_configuration` | 2 | 4 | 2 | 50% | 100% | good [~] |
| `aws_glue_trigger` | 3 | 11 | 15 | 13% | 67% | good [~] |
| `aws_glue_user_defined_function` | 0 | 0 | 9 | 0% | 0% | poor [!] |
| `aws_glue_workflow` | 2 | 7 | 7 | 14% | 86% | good [~] |
| `aws_guardduty_detector` | 4 | 7 | 7 | 43% | 57% | good [~] |
| `aws_guardduty_detector_feature` | 1 | 4 | 4 | 0% | 75% | good [~] |
| `aws_guardduty_filter` | 2 | 16 | 9 | 11% | 78% | good [~] |
| `aws_guardduty_invite_accepter` | 1 | 3 | 3 | 0% | 67% | good [~] |
| `aws_guardduty_ipset` | 1 | 8 | 8 | 0% | 88% | good [~] |
| `aws_guardduty_malware_protection_plan` | 5 | 12 | 8 | 50% | 88% | good [~] |
| `aws_guardduty_member` | 4 | 4 | 8 | 38% | 50% | good [~] |
| `aws_guardduty_member_detector_feature` | 1 | 5 | 5 | 0% | 80% | good [~] |
| `aws_guardduty_organization_admin_account` | 1 | 2 | 1 | 0% | 100% | good [~] |
| `aws_guardduty_organization_configuration` | 1 | 4 | 4 | 0% | 75% | good [~] |
| `aws_guardduty_organization_configuration_feature` | 1 | 0 | 4 | 0% | 0% | poor [!] |
| `aws_guardduty_publishing_destination` | 1 | 5 | 4 | 0% | 100% | good [~] |
| `aws_guardduty_threatintelset` | 1 | 8 | 8 | 0% | 88% | good [~] |
| `aws_iam_access_key` | 0 | 4 | 9 | 0% | 33% | good [~] |
| `aws_iam_account_alias` | 0 | 2 | 1 | 0% | 100% | good [~] |
| `aws_iam_account_password_policy` | 3 | 11 | 10 | 30% | 100% | good [~] |
| `aws_iam_group` | 0 | 5 | 4 | 0% | 100% | good [~] |
| `aws_iam_group_membership` | 1 | 4 | 3 | 33% | 100% | good [~] |
| `aws_iam_group_policies_exclusive` | 0 | 3 | 2 | 0% | 100% | good [~] |
| `aws_iam_group_policy` | 0 | 4 | 4 | 0% | 75% | good [~] |
| `aws_iam_group_policy_attachment` | 0 | 3 | 2 | 0% | 100% | good [~] |
| `aws_iam_group_policy_attachments_exclusive` | 0 | 3 | 2 | 0% | 100% | good [~] |
| `aws_iam_instance_profile` | 2 | 6 | 9 | 22% | 56% | good [~] |
| `aws_iam_openid_connect_provider` | 4 | 7 | 6 | 67% | 100% | excellent [+] |
| `aws_iam_policy` | 4 | 7 | 10 | 30% | 60% | good [~] |
| `aws_iam_policy_attachment` | 0 | 0 | 5 | 0% | 0% | poor [!] |
| `aws_iam_role` | 2 | 10 | 15 | 13% | 60% | good [~] |
| `aws_iam_role_policies_exclusive` | 0 | 3 | 2 | 0% | 100% | good [~] |
| `aws_iam_role_policy` | 0 | 4 | 4 | 0% | 75% | good [~] |
| `aws_iam_role_policy_attachment` | 0 | 3 | 2 | 0% | 100% | good [~] |
| `aws_iam_role_policy_attachments_exclusive` | 0 | 3 | 2 | 0% | 100% | good [~] |
| `aws_iam_saml_provider` | 2 | 7 | 6 | 33% | 100% | good [~] |
| `aws_iam_server_certificate` | 4 | 10 | 12 | 33% | 75% | good [~] |
| `aws_iam_service_linked_role` | 2 | 9 | 10 | 20% | 80% | good [~] |
| `aws_iam_service_specific_credential` | 0 | 7 | 6 | 0% | 100% | good [~] |
| `aws_iam_signing_certificate` | 0 | 5 | 4 | 0% | 100% | good [~] |
| `aws_iam_user` | 2 | 5 | 8 | 25% | 50% | good [~] |
| `aws_iam_user_group_membership` | 1 | 3 | 2 | 50% | 100% | good [~] |
| `aws_iam_user_login_profile` | 0 | 4 | 7 | 0% | 43% | good [~] |
| `aws_iam_user_policies_exclusive` | 0 | 3 | 2 | 0% | 100% | good [~] |
| `aws_iam_user_policy` | 0 | 4 | 4 | 0% | 75% | good [~] |
| `aws_iam_user_policy_attachment` | 0 | 3 | 2 | 0% | 100% | good [~] |
| `aws_iam_user_policy_attachments_exclusive` | 0 | 3 | 2 | 0% | 100% | good [~] |
| `aws_iam_user_ssh_key` | 0 | 7 | 6 | 0% | 100% | good [~] |
| `aws_iam_virtual_mfa_device` | 2 | 5 | 9 | 22% | 44% | good [~] |
| `aws_identitystore_group` | 1 | 6 | 5 | 0% | 100% | good [~] |
| `aws_identitystore_user` | 5 | 21 | 16 | 25% | 94% | good [~] |
| `aws_inspector2_delegated_admin_account` | 1 | 2 | 3 | 0% | 33% | good [~] |
| `aws_inspector2_enabler` | 3 | 3 | 3 | 67% | 67% | excellent [+] |
| `aws_inspector2_filter` | 1 | 6 | 8 | 0% | 62% | good [~] |
| `aws_inspector2_member_association` | 1 | 3 | 5 | 0% | 40% | good [~] |
| `aws_inspector2_organization_configuration` | 2 | 7 | 3 | 33% | 33% | good [~] |
| `aws_iot_authorizer` | 1 | 0 | 10 | 0% | 0% | poor [!] |
| `aws_iot_billing_group` | 1 | 3 | 7 | 0% | 29% | fair [-] |
| `aws_iot_ca_certificate` | 1 | 6 | 12 | 0% | 42% | good [~] |
| `aws_iot_certificate` | 1 | 5 | 8 | 0% | 50% | good [~] |
| `aws_iot_domain_configuration` | 1 | 6 | 14 | 0% | 36% | good [~] |
| `aws_iot_event_configurations` | 1 | 0 | 1 | 0% | 0% | poor [!] |
| `aws_iot_indexing_configuration` | 3 | 3 | 2 | 100% | 100% | excellent [+] |
| `aws_iot_logging_options` | 1 | 0 | 3 | 0% | 0% | poor [!] |
| `aws_iot_policy` | 1 | 5 | 7 | 0% | 57% | good [~] |
| `aws_iot_policy_attachment` | 1 | 3 | 2 | 0% | 100% | good [~] |
| `aws_iot_provisioning_template` | 1 | 0 | 11 | 0% | 0% | poor [!] |
| `aws_iot_role_alias` | 1 | 5 | 6 | 0% | 67% | good [~] |
| `aws_iot_thing` | 2 | 7 | 6 | 17% | 100% | good [~] |
| `aws_iot_thing_group` | 2 | 5 | 8 | 12% | 50% | good [~] |
| `aws_iot_thing_group_membership` | 1 | 3 | 3 | 0% | 67% | good [~] |
| `aws_iot_thing_principal_attachment` | 1 | 3 | 2 | 0% | 100% | good [~] |
| `aws_iot_thing_type` | 2 | 7 | 6 | 17% | 67% | good [~] |
| `aws_iot_topic_rule` | 4 | 7 | 28 | 4% | 21% | fair [-] |
| `aws_iot_topic_rule_destination` | 1 | 0 | 4 | 0% | 0% | poor [!] |
| `aws_ivs_channel` | 1 | 7 | 11 | 0% | 55% | good [~] |
| `aws_msk_cluster` | 8 | 27 | 29 | 24% | 69% | good [~] |
| `aws_msk_cluster_policy` | 1 | 0 | 3 | 0% | 0% | poor [!] |
| `aws_msk_configuration` | 1 | 0 | 6 | 0% | 0% | poor [!] |
| `aws_msk_replicator` | 1 | 0 | 10 | 0% | 0% | poor [!] |
| `aws_msk_scram_secret_association` | 1 | 0 | 2 | 0% | 0% | poor [!] |
| `aws_msk_serverless_cluster` | 3 | 11 | 9 | 22% | 89% | good [~] |
| `aws_msk_single_scram_secret_association` | 1 | 0 | 2 | 0% | 0% | poor [!] |
| `aws_msk_vpc_connection` | 1 | 0 | 8 | 0% | 0% | poor [!] |
| `aws_keyspaces_keyspace` | 2 | 4 | 6 | 17% | 67% | good [~] |
| `aws_keyspaces_table` | 2 | 3 | 14 | 7% | 21% | fair [-] |
| `aws_kinesis_resource_policy` | 0 | 0 | 2 | 0% | 0% | poor [!] |
| `aws_kinesis_stream` | 1 | 8 | 12 | 0% | 58% | good [~] |
| `aws_kinesis_stream_consumer` | 0 | 0 | 4 | 0% | 0% | poor [!] |
| `aws_kinesis_analytics_application` | 0 | 0 | 15 | 0% | 0% | poor [!] |
| `aws_kinesisanalyticsv2_application` | 1 | 15 | 17 | 0% | 82% | good [~] |
| `aws_kinesis_video_stream` | 1 | 11 | 11 | 0% | 82% | good [~] |
| `aws_kms_alias` | 1 | 5 | 5 | 0% | 80% | good [~] |
| `aws_kms_ciphertext` | 1 | 0 | 4 | 0% | 0% | poor [!] |
| `aws_kms_custom_key_store` | 1 | 7 | 11 | 0% | 36% | good [~] |
| `aws_kms_external_key` | 3 | 11 | 14 | 14% | 64% | good [~] |
| `aws_kms_grant` | 3 | 11 | 10 | 20% | 80% | good [~] |
| `aws_kms_key` | 4 | 11 | 17 | 18% | 53% | good [~] |
| `aws_kms_key_policy` | 1 | 3 | 3 | 0% | 67% | good [~] |
| `aws_kms_replica_external_key` | 3 | 11 | 15 | 13% | 67% | good [~] |
| `aws_kms_replica_key` | 2 | 12 | 13 | 8% | 77% | good [~] |
| `aws_lakeformation_data_cells_filter` | 1 | 0 | 2 | 0% | 0% | poor [!] |
| `aws_lakeformation_data_lake_settings` | 5 | 10 | 11 | 36% | 55% | good [~] |
| `aws_lakeformation_lf_tag` | 1 | 4 | 3 | 0% | 100% | good [~] |
| `aws_lakeformation_opt_in` | 1 | 0 | 5 | 0% | 0% | poor [!] |
| `aws_lakeformation_permissions` | 3 | 5 | 12 | 17% | 33% | good [~] |
| `aws_lakeformation_resource` | 1 | 4 | 6 | 0% | 50% | good [~] |
| `aws_lakeformation_resource_lf_tag` | 3 | 4 | 6 | 0% | 17% | fair [-] |
| `aws_lakeformation_resource_lf_tags` | 2 | 5 | 6 | 17% | 33% | good [~] |
| `aws_lambda_alias` | 1 | 6 | 7 | 0% | 71% | good [~] |
| `aws_lambda_code_signing_config` | 6 | 11 | 8 | 50% | 100% | good [~] |
| `aws_lambda_event_source_mapping` | 24 | 19 | 35 | 63% | 51% | excellent [+] |
| `aws_lambda_function` | 22 | 27 | 45 | 47% | 51% | good [~] |
| `aws_lambda_function_event_invoke_config` | 2 | 8 | 5 | 20% | 80% | good [~] |
| `aws_lambda_function_recursion_config` | 1 | 3 | 2 | 0% | 100% | good [~] |
| `aws_lambda_function_url` | 3 | 10 | 9 | 22% | 78% | good [~] |
| `aws_lambda_invocation` | 0 | 0 | 7 | 0% | 0% | poor [!] |
| `aws_lambda_layer_version` | 10 | 15 | 19 | 47% | 74% | good [~] |
| `aws_lambda_layer_version_permission` | 1 | 8 | 9 | 0% | 78% | good [~] |
| `aws_lambda_permission` | 1 | 7 | 11 | 0% | 55% | good [~] |
| `aws_lambda_provisioned_concurrency_config` | 1 | 6 | 5 | 0% | 60% | good [~] |
| `aws_lambda_runtime_management_config` | 1 | 4 | 5 | 0% | 60% | good [~] |
| `aws_lexv2models_bot` | 1 | 13 | 12 | 0% | 92% | good [~] |
| `aws_lexv2models_bot_locale` | 1 | 7 | 8 | 0% | 75% | good [~] |
| `aws_lexv2models_bot_version` | 1 | 4 | 5 | 0% | 60% | good [~] |
| `aws_lexv2models_intent` | 1 | 7 | 20 | 0% | 30% | good [~] |
| `aws_lexv2models_slot` | 1 | 9 | 13 | 0% | 62% | good [~] |
| `aws_lexv2models_slot_type` | 1 | 8 | 12 | 0% | 58% | good [~] |
| `aws_cloudwatch_log_account_policy` | 0 | 0 | 5 | 0% | 0% | poor [!] |
| `aws_cloudwatch_log_anomaly_detector` | 0 | 0 | 10 | 0% | 0% | poor [!] |
| `aws_cloudwatch_log_data_protection_policy` | 0 | 0 | 2 | 0% | 0% | poor [!] |
| `aws_cloudwatch_log_delivery` | 0 | 0 | 8 | 0% | 0% | poor [!] |
| `aws_cloudwatch_log_delivery_destination` | 0 | 0 | 7 | 0% | 0% | poor [!] |
| `aws_cloudwatch_log_delivery_destination_policy` | 0 | 0 | 2 | 0% | 0% | poor [!] |
| `aws_cloudwatch_log_delivery_source` | 0 | 0 | 7 | 0% | 0% | poor [!] |
| `aws_cloudwatch_log_destination` | 0 | 0 | 6 | 0% | 0% | poor [!] |
| `aws_cloudwatch_log_destination_policy` | 0 | 0 | 3 | 0% | 0% | poor [!] |
| `aws_cloudwatch_log_group` | 2 | 6 | 9 | 11% | 56% | good [~] |
| `aws_cloudwatch_log_index_policy` | 0 | 0 | 2 | 0% | 0% | poor [!] |
| `aws_cloudwatch_log_metric_filter` | 0 | 0 | 4 | 0% | 0% | poor [!] |
| `aws_cloudwatch_log_resource_policy` | 0 | 0 | 2 | 0% | 0% | poor [!] |
| `aws_cloudwatch_log_stream` | 1 | 4 | 3 | 0% | 100% | good [~] |
| `aws_cloudwatch_log_subscription_filter` | 0 | 0 | 6 | 0% | 0% | poor [!] |
| `aws_macie2_account` | 1 | 6 | 5 | 0% | 100% | good [~] |
| `aws_macie2_classification_export_configuration` | 1 | 2 | 1 | 0% | 100% | good [~] |
| `aws_macie2_classification_job` | 8 | 19 | 17 | 29% | 76% | good [~] |
| `aws_macie2_custom_data_identifier` | 5 | 13 | 12 | 25% | 75% | good [~] |
| `aws_macie2_findings_filter` | 3 | 8 | 10 | 20% | 70% | good [~] |
| `aws_macie2_invitation_accepter` | 1 | 3 | 3 | 0% | 67% | good [~] |
| `aws_macie2_member` | 1 | 10 | 15 | 0% | 60% | good [~] |
| `aws_macie2_organization_admin_account` | 1 | 2 | 1 | 0% | 100% | good [~] |
| `aws_macie2_organization_configuration` | 1 | 2 | 1 | 0% | 100% | good [~] |
| `aws_medialive_channel` | 9 | 17 | 17 | 41% | 82% | good [~] |
| `aws_medialive_input` | 8 | 17 | 17 | 35% | 76% | good [~] |
| `aws_medialive_input_security_group` | 0 | 0 | 6 | 0% | 0% | poor [!] |
| `aws_medialive_multiplex` | 0 | 0 | 8 | 0% | 0% | poor [!] |
| `aws_medialive_multiplex_program` | 0 | 0 | 4 | 0% | 0% | poor [!] |
| `aws_media_package_channel` | 1 | 6 | 6 | 0% | 67% | good [~] |
| `aws_media_packagev2_channel_group` | 1 | 9 | 6 | 0% | 83% | good [~] |
| `aws_media_store_container` | 1 | 5 | 5 | 0% | 80% | good [~] |
| `aws_memorydb_acl` | 3 | 7 | 7 | 29% | 71% | good [~] |
| `aws_memorydb_cluster` | 2 | 21 | 32 | 3% | 56% | good [~] |
| `aws_memorydb_snapshot` | 1 | 7 | 10 | 0% | 60% | good [~] |
| `aws_memorydb_subnet_group` | 2 | 6 | 8 | 12% | 62% | good [~] |
| `aws_mq_broker` | 12 | 24 | 26 | 42% | 65% | good [~] |
| `aws_mq_configuration` | 1 | 11 | 10 | 0% | 100% | good [~] |
| `aws_neptune_cluster` | 8 | 36 | 38 | 18% | 63% | good [~] |
| `aws_neptune_cluster_endpoint` | 3 | 8 | 9 | 22% | 78% | good [~] |
| `aws_neptune_cluster_instance` | 5 | 21 | 28 | 11% | 61% | good [~] |
| `aws_neptune_cluster_parameter_group` | 4 | 9 | 8 | 38% | 88% | good [~] |
| `aws_neptune_cluster_snapshot` | 5 | 16 | 16 | 12% | 81% | good [~] |
| `aws_neptune_event_subscription` | 3 | 10 | 12 | 17% | 67% | good [~] |
| `aws_neptune_global_cluster` | 1 | 10 | 11 | 0% | 73% | good [~] |
| `aws_neptune_parameter_group` | 4 | 9 | 8 | 38% | 88% | good [~] |
| `aws_neptune_subnet_group` | 4 | 9 | 7 | 43% | 86% | good [~] |
| `aws_networkfirewall_firewall` | 3 | 14 | 16 | 12% | 75% | good [~] |
| `aws_networkfirewall_firewall_policy` | 2 | 8 | 8 | 12% | 88% | good [~] |
| `aws_networkfirewall_logging_configuration` | 0 | 0 | 2 | 0% | 0% | poor [!] |
| `aws_networkfirewall_resource_policy` | 0 | 0 | 2 | 0% | 0% | poor [!] |
| `aws_networkfirewall_rule_group` | 2 | 10 | 11 | 9% | 82% | good [~] |
| `aws_networkfirewall_tls_inspection_configuration` | 0 | 0 | 13 | 0% | 0% | poor [!] |
| `aws_networkmanager_attachment_accepter` | 1 | 0 | 12 | 0% | 0% | poor [!] |
| `aws_networkmanager_connect_attachment` | 1 | 9 | 16 | 0% | 50% | good [~] |
| `aws_networkmanager_connect_peer` | 2 | 10 | 16 | 6% | 56% | good [~] |
| `aws_networkmanager_connection` | 1 | 11 | 10 | 0% | 80% | good [~] |
| `aws_networkmanager_core_network` | 1 | 7 | 14 | 0% | 43% | good [~] |
| `aws_networkmanager_core_network_policy_attachment` | 1 | 0 | 4 | 0% | 0% | poor [!] |
| `aws_networkmanager_customer_gateway_association` | 1 | 5 | 5 | 0% | 80% | good [~] |
| `aws_networkmanager_device` | 3 | 14 | 13 | 15% | 85% | good [~] |
| `aws_networkmanager_dx_gateway_attachment` | 1 | 9 | 13 | 0% | 62% | good [~] |
| `aws_networkmanager_global_network` | 1 | 6 | 5 | 0% | 60% | good [~] |
| `aws_networkmanager_link` | 2 | 10 | 10 | 10% | 70% | good [~] |
| `aws_networkmanager_link_association` | 1 | 4 | 4 | 0% | 75% | good [~] |
| `aws_networkmanager_site` | 2 | 8 | 7 | 14% | 71% | good [~] |
| `aws_networkmanager_site_to_site_vpn_attachment` | 1 | 9 | 14 | 0% | 57% | good [~] |
| `aws_networkmanager_transit_gateway_connect_peer_association` | 1 | 5 | 5 | 0% | 80% | good [~] |
| `aws_networkmanager_transit_gateway_peering` | 1 | 0 | 12 | 0% | 0% | poor [!] |
| `aws_networkmanager_transit_gateway_registration` | 1 | 3 | 3 | 0% | 67% | good [~] |
| `aws_networkmanager_transit_gateway_route_table_attachment` | 1 | 9 | 15 | 0% | 53% | good [~] |
| `aws_networkmanager_vpc_attachment` | 2 | 10 | 16 | 6% | 56% | good [~] |
| `aws_opensearch_authorize_vpc_endpoint_access` | 1 | 6 | 3 | 0% | 100% | good [~] |
| `aws_opensearch_domain` | 3 | 24 | 29 | 7% | 52% | good [~] |
| `aws_opensearch_domain_policy` | 1 | 3 | 3 | 0% | 67% | good [~] |
| `aws_opensearch_domain_saml_options` | 1 | 0 | 3 | 0% | 0% | poor [!] |
| `aws_opensearch_inbound_connection_accepter` | 1 | 3 | 3 | 0% | 67% | good [~] |
| `aws_opensearch_outbound_connection` | 5 | 10 | 8 | 38% | 62% | good [~] |
| `aws_opensearch_package` | 2 | 7 | 6 | 0% | 83% | good [~] |
| `aws_opensearch_package_association` | 1 | 3 | 4 | 0% | 50% | good [~] |
| `aws_opensearch_vpc_endpoint` | 4 | 8 | 4 | 50% | 75% | good [~] |
| `aws_opensearchserverless_access_policy` | 1 | 6 | 5 | 0% | 100% | good [~] |
| `aws_opensearchserverless_collection` | 1 | 7 | 11 | 0% | 55% | good [~] |
| `aws_opensearchserverless_security_policy` | 1 | 5 | 5 | 0% | 100% | good [~] |
| `aws_opensearchserverless_vpc_endpoint` | 3 | 5 | 5 | 40% | 80% | good [~] |
| `aws_organizations_account` | 1 | 6 | 15 | 0% | 33% | good [~] |
| `aws_organizations_delegated_administrator` | 1 | 10 | 9 | 0% | 100% | good [~] |
| `aws_organizations_organization` | 1 | 5 | 11 | 0% | 36% | good [~] |
| `aws_organizations_organizational_unit` | 1 | 4 | 6 | 0% | 50% | good [~] |
| `aws_organizations_policy` | 1 | 6 | 8 | 0% | 62% | good [~] |
| `aws_organizations_policy_attachment` | 1 | 3 | 3 | 0% | 67% | good [~] |
| `aws_organizations_resource_policy` | 1 | 3 | 4 | 0% | 50% | good [~] |
| `aws_osis_pipeline` | 6 | 12 | 13 | 38% | 85% | good [~] |
| `aws_outposts_outpost` | 1 | 13 | 0 | 0% | 0% | n/a [?] |
| `aws_outposts_site` | 1 | 6 | 0 | 0% | 0% | n/a [?] |
| `aws_pinpoint_app` | 5 | 18 | 9 | 44% | 89% | good [~] |
| `aws_pinpoint_email_channel` | 2 | 8 | 8 | 12% | 88% | good [~] |
| `aws_pinpoint_event_stream` | 1 | 4 | 3 | 0% | 100% | good [~] |
| `aws_pipes_pipe` | 1 | 18 | 17 | 0% | 82% | good [~] |
| `aws_quicksight_account_settings` | 1 | 5 | 4 | 0% | 75% | good [~] |
| `aws_quicksight_account_subscription` | 0 | 0 | 18 | 0% | 0% | poor [!] |
| `aws_quicksight_analysis` | 1 | 8 | 17 | 0% | 41% | good [~] |
| `aws_quicksight_dashboard` | 1 | 8 | 20 | 0% | 30% | good [~] |
| `aws_quicksight_data_set` | 1 | 9 | 18 | 0% | 33% | good [~] |
| `aws_quicksight_data_source` | 5 | 14 | 12 | 33% | 83% | good [~] |
| `aws_quicksight_folder` | 1 | 8 | 13 | 0% | 54% | good [~] |
| `aws_quicksight_folder_membership` | 1 | 5 | 4 | 0% | 100% | good [~] |
| `aws_quicksight_group` | 1 | 6 | 5 | 0% | 80% | good [~] |
| `aws_quicksight_group_membership` | 1 | 6 | 5 | 0% | 100% | good [~] |
| `aws_quicksight_iam_policy_assignment` | 0 | 0 | 7 | 0% | 0% | poor [!] |
| `aws_quicksight_ingestion` | 1 | 6 | 6 | 0% | 83% | good [~] |
| `aws_quicksight_namespace` | 1 | 7 | 9 | 0% | 67% | good [~] |
| `aws_quicksight_refresh_schedule` | 0 | 0 | 5 | 0% | 0% | poor [!] |
| `aws_quicksight_role_membership` | 0 | 0 | 4 | 0% | 0% | poor [!] |
| `aws_quicksight_template` | 1 | 9 | 16 | 0% | 44% | good [~] |
| `aws_quicksight_template_alias` | 0 | 0 | 5 | 0% | 0% | poor [!] |
| `aws_quicksight_theme` | 1 | 9 | 15 | 0% | 47% | good [~] |
| `aws_quicksight_user` | 1 | 9 | 10 | 0% | 60% | good [~] |
| `aws_quicksight_vpc_connection` | 0 | 0 | 12 | 0% | 0% | poor [!] |
| `aws_ram_principal_association` | 1 | 3 | 2 | 0% | 100% | good [~] |
| `aws_ram_resource_association` | 1 | 3 | 2 | 0% | 100% | good [~] |
| `aws_ram_resource_share` | 1 | 9 | 7 | 0% | 57% | good [~] |
| `aws_ram_resource_share_accepter` | 1 | 6 | 9 | 0% | 56% | good [~] |
| `aws_ram_sharing_with_organization` | 1 | 1 | 0 | 0% | 0% | n/a [?] |
| `aws_db_cluster_snapshot` | 3 | 18 | 19 | 11% | 84% | good [~] |
| `aws_db_event_subscription` | 4 | 8 | 12 | 25% | 58% | good [~] |
| `aws_db_instance` | 31 | 48 | 84 | 36% | 55% | good [~] |
| `aws_db_option_group` | 3 | 9 | 11 | 18% | 73% | good [~] |
| `aws_db_parameter_group` | 3 | 7 | 9 | 22% | 67% | good [~] |
| `aws_db_proxy` | 6 | 14 | 14 | 14% | 86% | good [~] |
| `aws_db_proxy_default_target_group` | 2 | 5 | 5 | 20% | 80% | good [~] |
| `aws_db_proxy_endpoint` | 2 | 10 | 12 | 8% | 75% | good [~] |
| `aws_db_proxy_target` | 11 | 9 | 10 | 100% | 80% | excellent [+] |
| `aws_db_snapshot` | 3 | 21 | 23 | 9% | 87% | good [~] |
| `aws_db_snapshot_copy` | 4 | 0 | 25 | 12% | 0% | poor [!] |
| `aws_db_subnet_group` | 3 | 7 | 9 | 22% | 67% | good [~] |
| `aws_rds_cluster` | 34 | 49 | 74 | 45% | 55% | good [~] |
| `aws_rds_cluster_endpoint` | 2 | 8 | 9 | 11% | 78% | good [~] |
| `aws_rds_cluster_instance` | 11 | 24 | 36 | 28% | 64% | good [~] |
| `aws_rds_cluster_parameter_group` | 2 | 7 | 8 | 12% | 75% | good [~] |
| `aws_rds_cluster_snapshot_copy` | 4 | 0 | 19 | 16% | 0% | poor [!] |
| `aws_rds_export_task` | 1 | 16 | 16 | 0% | 94% | good [~] |
| `aws_rds_global_cluster` | 4 | 9 | 17 | 18% | 47% | good [~] |
| `aws_rds_instance_state` | 3 | 0 | 3 | 67% | 0% | good [~] |
| `aws_rds_shard_group` | 5 | 11 | 12 | 33% | 83% | good [~] |
| `aws_redshift_authentication_profile` | 1 | 3 | 2 | 0% | 100% | good [~] |
| `aws_redshift_cluster` | 19 | 43 | 52 | 35% | 54% | good [~] |
| `aws_redshift_cluster_iam_roles` | 2 | 0 | 4 | 25% | 0% | fair [-] |
| `aws_redshift_cluster_snapshot` | 3 | 12 | 8 | 25% | 62% | good [~] |
| `aws_redshift_data_share_authorization` | 1 | 0 | 5 | 0% | 0% | poor [!] |
| `aws_redshift_data_share_consumer_association` | 1 | 0 | 7 | 0% | 0% | poor [!] |
| `aws_redshift_endpoint_access` | 2 | 0 | 8 | 12% | 0% | poor [!] |
| `aws_redshift_endpoint_authorization` | 2 | 0 | 8 | 12% | 0% | poor [!] |
| `aws_redshift_event_subscription` | 5 | 13 | 13 | 31% | 92% | good [~] |
| `aws_redshift_hsm_client_certificate` | 3 | 6 | 5 | 40% | 100% | good [~] |
| `aws_redshift_hsm_configuration` | 3 | 8 | 9 | 22% | 78% | good [~] |
| `aws_redshift_integration` | 4 | 0 | 10 | 30% | 0% | fair [-] |
| `aws_redshift_logging` | 1 | 6 | 5 | 0% | 100% | good [~] |
| `aws_redshift_parameter_group` | 4 | 9 | 7 | 43% | 100% | good [~] |
| `aws_redshift_partner` | 1 | 7 | 6 | 0% | 100% | good [~] |
| `aws_redshift_resource_policy` | 1 | 3 | 2 | 0% | 100% | good [~] |
| `aws_redshift_scheduled_action` | 1 | 9 | 8 | 0% | 88% | good [~] |
| `aws_redshift_snapshot_copy` | 1 | 5 | 5 | 0% | 80% | good [~] |
| `aws_redshift_snapshot_copy_grant` | 3 | 6 | 5 | 40% | 100% | good [~] |
| `aws_redshift_snapshot_schedule` | 4 | 7 | 8 | 38% | 75% | good [~] |
| `aws_redshift_snapshot_schedule_association` | 1 | 0 | 2 | 0% | 0% | poor [!] |
| `aws_redshift_subnet_group` | 4 | 6 | 6 | 50% | 67% | good [~] |
| `aws_redshift_usage_limit` | 4 | 10 | 9 | 22% | 100% | good [~] |
| `aws_rekognition_collection` | 1 | 8 | 6 | 0% | 67% | good [~] |
| `aws_resiliencehub_resiliency_policy` | 1 | 12 | 10 | 0% | 50% | good [~] |
| `aws_resourcegroups_group` | 3 | 10 | 8 | 25% | 62% | good [~] |
| `aws_rolesanywhere_profile` | 6 | 9 | 10 | 40% | 80% | good [~] |
| `aws_rolesanywhere_trust_anchor` | 2 | 5 | 7 | 14% | 57% | good [~] |
| `aws_route53_cidr_collection` | 1 | 4 | 3 | 0% | 100% | good [~] |
| `aws_route53_cidr_location` | 2 | 4 | 3 | 33% | 100% | good [~] |
| `aws_route53_delegation_set` | 1 | 3 | 3 | 0% | 67% | good [~] |
| `aws_route53_health_check` | 14 | 20 | 24 | 38% | 67% | good [~] |
| `aws_route53_hosted_zone_dnssec` | 1 | 3 | 3 | 0% | 67% | good [~] |
| `aws_route53_key_signing_key` | 1 | 5 | 15 | 0% | 27% | fair [-] |
| `aws_route53_query_log` | 2 | 3 | 3 | 0% | 67% | good [~] |
| `aws_route53_record` | 11 | 16 | 18 | 56% | 83% | good [~] |
| `aws_route53_traffic_policy` | 3 | 6 | 6 | 17% | 83% | good [~] |
| `aws_route53_traffic_policy_instance` | 2 | 7 | 6 | 0% | 83% | good [~] |
| `aws_route53_vpc_association_authorization` | 1 | 4 | 4 | 0% | 75% | good [~] |
| `aws_route53_zone` | 6 | 10 | 12 | 42% | 58% | good [~] |
| `aws_route53_zone_association` | 1 | 4 | 5 | 0% | 60% | good [~] |
| `aws_route53domains_registered_domain` | 3 | 20 | 25 | 8% | 64% | good [~] |
| `aws_route53_resolver_config` | 1 | 0 | 3 | 0% | 0% | poor [!] |
| `aws_route53_resolver_dnssec_config` | 1 | 4 | 4 | 0% | 75% | good [~] |
| `aws_route53_resolver_endpoint` | 6 | 16 | 11 | 45% | 82% | good [~] |
| `aws_route53_resolver_firewall_config` | 1 | 0 | 3 | 0% | 0% | poor [!] |
| `aws_route53_resolver_firewall_domain_list` | 1 | 0 | 5 | 0% | 0% | poor [!] |
| `aws_route53_resolver_firewall_rule` | 1 | 0 | 11 | 0% | 0% | poor [!] |
| `aws_route53_resolver_firewall_rule_group` | 1 | 0 | 6 | 0% | 0% | poor [!] |
| `aws_route53_resolver_firewall_rule_group_association` | 1 | 0 | 8 | 0% | 0% | poor [!] |
| `aws_route53_resolver_query_log_config` | 3 | 9 | 7 | 29% | 86% | good [~] |
| `aws_route53_resolver_query_log_config_association` | 1 | 6 | 2 | 0% | 100% | good [~] |
| `aws_route53_resolver_rule` | 4 | 15 | 11 | 27% | 82% | good [~] |
| `aws_route53_resolver_rule_association` | 1 | 6 | 4 | 0% | 75% | good [~] |
| `aws_s3_access_point` | 0 | 0 | 13 | 0% | 0% | poor [!] |
| `aws_s3_account_public_access_block` | 0 | 0 | 5 | 0% | 0% | poor [!] |
| `aws_s3_bucket` | 4 | 21 | 27 | 15% | 70% | good [~] |
| `aws_s3_bucket_inventory` | 0 | 0 | 8 | 0% | 0% | poor [!] |
| `aws_s3_bucket_object` | 0 | 0 | 27 | 0% | 0% | poor [!] |
| `aws_s3_bucket_public_access_block` | 0 | 0 | 5 | 0% | 0% | poor [!] |
| `aws_s3_bucket_request_payment_configuration` | 0 | 0 | 3 | 0% | 0% | poor [!] |
| `aws_s3_directory_bucket` | 0 | 0 | 6 | 0% | 0% | poor [!] |
| `aws_s3_object` | 0 | 0 | 34 | 0% | 0% | poor [!] |
| `aws_s3_object_copy` | 0 | 0 | 53 | 0% | 0% | poor [!] |
| `aws_s3control_access_grant` | 1 | 8 | 11 | 0% | 45% | good [~] |
| `aws_s3control_access_grants_instance` | 1 | 6 | 7 | 0% | 57% | good [~] |
| `aws_s3control_access_grants_instance_resource_policy` | 1 | 4 | 2 | 0% | 50% | good [~] |
| `aws_s3control_access_grants_location` | 1 | 6 | 7 | 0% | 57% | good [~] |
| `aws_s3control_access_point` | 1 | 15 | 0 | 0% | 0% | n/a [?] |
| `aws_s3control_access_point_policy` | 1 | 0 | 3 | 0% | 0% | poor [!] |
| `aws_s3control_bucket` | 1 | 8 | 7 | 0% | 86% | good [~] |
| `aws_s3control_bucket_lifecycle_configuration` | 1 | 3 | 2 | 0% | 100% | good [~] |
| `aws_s3control_bucket_policy` | 1 | 0 | 2 | 0% | 0% | poor [!] |
| `aws_s3control_directory_bucket_access_point_scope` | 1 | 5 | 3 | 0% | 67% | good [~] |
| `aws_s3control_multi_region_access_point` | 1 | 4 | 7 | 0% | 43% | good [~] |
| `aws_s3control_multi_region_access_point_policy` | 1 | 0 | 5 | 0% | 0% | poor [!] |
| `aws_s3control_object_lambda_access_point` | 1 | 5 | 5 | 0% | 80% | good [~] |
| `aws_s3control_object_lambda_access_point_policy` | 1 | 0 | 4 | 0% | 0% | poor [!] |
| `aws_s3control_storage_lens_configuration` | 1 | 4 | 6 | 0% | 50% | good [~] |
| `aws_s3tables_namespace` | 2 | 8 | 5 | 0% | 100% | good [~] |
| `aws_s3tables_table` | 2 | 14 | 16 | 0% | 81% | good [~] |
| `aws_s3tables_table_bucket` | 2 | 13 | 6 | 0% | 67% | good [~] |
| `aws_s3tables_table_bucket_policy` | 0 | 0 | 2 | 0% | 0% | poor [!] |
| `aws_s3tables_table_policy` | 0 | 0 | 4 | 0% | 0% | poor [!] |
| `aws_sagemaker_app` | 3 | 11 | 9 | 22% | 89% | good [~] |
| `aws_sagemaker_data_quality_job_definition` | 3 | 7 | 12 | 17% | 42% | good [~] |
| `aws_sagemaker_domain` | 9 | 20 | 20 | 35% | 75% | good [~] |
| `aws_sagemaker_endpoint` | 3 | 8 | 6 | 33% | 67% | good [~] |
| `aws_sagemaker_endpoint_configuration` | 7 | 10 | 10 | 60% | 80% | excellent [+] |
| `aws_sagemaker_feature_group` | 3 | 8 | 12 | 17% | 33% | good [~] |
| `aws_sagemaker_model` | 7 | 11 | 10 | 60% | 90% | excellent [+] |
| `aws_sagemaker_model_package_group` | 3 | 7 | 5 | 40% | 100% | good [~] |
| `aws_sagemaker_notebook_instance` | 7 | 15 | 20 | 25% | 55% | good [~] |
| `aws_sagemaker_notebook_instance_lifecycle_configuration` | 3 | 6 | 6 | 0% | 67% | good [~] |
| `aws_sagemaker_pipeline` | 3 | 11 | 10 | 20% | 80% | good [~] |
| `aws_sagemaker_space` | 3 | 9 | 11 | 18% | 45% | good [~] |
| `aws_sagemaker_user_profile` | 3 | 9 | 9 | 22% | 56% | good [~] |
| `aws_scheduler_schedule` | 4 | 20 | 13 | 15% | 69% | good [~] |
| `aws_scheduler_schedule_group` | 2 | 6 | 9 | 11% | 56% | good [~] |
| `aws_secretsmanager_secret` | 1 | 7 | 11 | 0% | 55% | good [~] |
| `aws_secretsmanager_secret_version` | 2 | 6 | 9 | 11% | 56% | good [~] |
| `aws_securityhub_account` | 1 | 6 | 4 | 0% | 100% | good [~] |
| `aws_securityhub_action_target` | 1 | 5 | 4 | 0% | 100% | good [~] |
| `aws_securityhub_automation_rule` | 1 | 7 | 10 | 0% | 60% | good [~] |
| `aws_securityhub_configuration_policy` | 1 | 5 | 4 | 0% | 100% | good [~] |
| `aws_securityhub_configuration_policy_association` | 1 | 3 | 3 | 0% | 67% | good [~] |
| `aws_securityhub_finding_aggregator` | 1 | 4 | 2 | 0% | 100% | good [~] |
| `aws_securityhub_insight` | 1 | 5 | 4 | 0% | 100% | good [~] |
| `aws_securityhub_invite_accepter` | 1 | 0 | 2 | 0% | 0% | poor [!] |
| `aws_securityhub_member` | 1 | 6 | 5 | 0% | 100% | good [~] |
| `aws_securityhub_organization_admin_account` | 1 | 2 | 1 | 0% | 100% | good [~] |
| `aws_securityhub_organization_configuration` | 1 | 3 | 4 | 0% | 50% | good [~] |
| `aws_securityhub_product_subscription` | 1 | 3 | 2 | 0% | 100% | good [~] |
| `aws_securityhub_standards_control` | 1 | 11 | 10 | 0% | 100% | good [~] |
| `aws_securityhub_standards_control_association` | 1 | 5 | 4 | 0% | 100% | good [~] |
| `aws_securityhub_standards_subscription` | 1 | 5 | 2 | 0% | 50% | good [~] |
| `aws_servicecatalog_budget_resource_association` | 0 | 0 | 3 | 0% | 0% | poor [!] |
| `aws_servicecatalog_constraint` | 0 | 0 | 9 | 0% | 0% | poor [!] |
| `aws_servicecatalog_organizations_access` | 0 | 0 | 2 | 0% | 0% | poor [!] |
| `aws_servicecatalog_portfolio` | 1 | 7 | 8 | 0% | 75% | good [~] |
| `aws_servicecatalog_portfolio_share` | 0 | 0 | 9 | 0% | 0% | poor [!] |
| `aws_servicecatalog_principal_portfolio_association` | 0 | 0 | 5 | 0% | 0% | poor [!] |
| `aws_servicecatalog_product` | 1 | 12 | 17 | 0% | 65% | good [~] |
| `aws_servicecatalog_product_portfolio_association` | 0 | 0 | 5 | 0% | 0% | poor [!] |
| `aws_servicecatalog_provisioned_product` | 0 | 0 | 27 | 0% | 0% | poor [!] |
| `aws_servicecatalog_provisioning_artifact` | 0 | 0 | 13 | 0% | 0% | poor [!] |
| `aws_servicecatalog_service_action` | 0 | 0 | 5 | 0% | 0% | poor [!] |
| `aws_servicecatalog_tag_option` | 0 | 0 | 5 | 0% | 0% | poor [!] |
| `aws_servicecatalog_tag_option_resource_association` | 0 | 0 | 7 | 0% | 0% | poor [!] |
| `aws_servicecatalogappregistry_application` | 1 | 7 | 6 | 0% | 67% | good [~] |
| `aws_service_discovery_http_namespace` | 1 | 6 | 6 | 0% | 83% | good [~] |
| `aws_service_discovery_instance` | 2 | 4 | 3 | 33% | 100% | good [~] |
| `aws_service_discovery_private_dns_namespace` | 2 | 8 | 7 | 0% | 71% | good [~] |
| `aws_service_discovery_public_dns_namespace` | 1 | 7 | 6 | 0% | 83% | good [~] |
| `aws_service_discovery_service` | 4 | 16 | 11 | 27% | 82% | good [~] |
| `aws_servicequotas_service_quota` | 1 | 11 | 11 | 0% | 64% | good [~] |
| `aws_sesv2_account_suppression_attributes` | 2 | 2 | 1 | 100% | 100% | excellent [+] |
| `aws_sesv2_account_vdm_attributes` | 3 | 4 | 3 | 67% | 100% | excellent [+] |
| `aws_sesv2_configuration_set` | 7 | 9 | 10 | 60% | 80% | excellent [+] |
| `aws_sesv2_configuration_set_event_destination` | 2 | 6 | 3 | 33% | 100% | good [~] |
| `aws_sesv2_contact_list` | 3 | 6 | 8 | 25% | 62% | good [~] |
| `aws_sesv2_dedicated_ip_assignment` | 1 | 3 | 3 | 0% | 67% | good [~] |
| `aws_sesv2_dedicated_ip_pool` | 1 | 4 | 5 | 0% | 60% | good [~] |
| `aws_sesv2_email_identity` | 4 | 10 | 8 | 38% | 62% | good [~] |
| `aws_sesv2_email_identity_feedback_attributes` | 1 | 3 | 2 | 0% | 100% | good [~] |
| `aws_sesv2_email_identity_mail_from_attributes` | 1 | 4 | 3 | 0% | 100% | good [~] |
| `aws_sesv2_email_identity_policy` | 1 | 4 | 3 | 0% | 100% | good [~] |
| `aws_ses_active_receipt_rule_set` | 1 | 3 | 2 | 0% | 100% | good [~] |
| `aws_ses_configuration_set` | 1 | 5 | 7 | 0% | 57% | good [~] |
| `aws_ses_domain_dkim` | 1 | 3 | 2 | 0% | 100% | good [~] |
| `aws_ses_domain_identity` | 1 | 4 | 3 | 0% | 100% | good [~] |
| `aws_ses_domain_identity_verification` | 1 | 3 | 3 | 0% | 67% | good [~] |
| `aws_ses_domain_mail_from` | 1 | 4 | 3 | 0% | 100% | good [~] |
| `aws_ses_email_identity` | 1 | 2 | 2 | 0% | 50% | good [~] |
| `aws_ses_event_destination` | 1 | 5 | 8 | 0% | 50% | good [~] |
| `aws_ses_identity_notification_topic` | 1 | 5 | 4 | 0% | 100% | good [~] |
| `aws_ses_identity_policy` | 0 | 0 | 3 | 0% | 0% | poor [!] |
| `aws_ses_receipt_filter` | 0 | 0 | 4 | 0% | 0% | poor [!] |
| `aws_ses_receipt_rule` | 1 | 6 | 15 | 0% | 33% | good [~] |
| `aws_ses_receipt_rule_set` | 1 | 3 | 2 | 0% | 100% | good [~] |
| `aws_ses_template` | 1 | 6 | 5 | 0% | 100% | good [~] |
| `aws_shield_application_layer_automatic_response` | 1 | 0 | 3 | 0% | 0% | poor [!] |
| `aws_shield_drt_access_log_bucket_association` | 1 | 0 | 3 | 0% | 0% | poor [!] |
| `aws_shield_drt_access_role_arn_association` | 1 | 0 | 2 | 0% | 0% | poor [!] |
| `aws_shield_proactive_engagement` | 1 | 0 | 2 | 0% | 0% | poor [!] |
| `aws_shield_protection` | 1 | 7 | 5 | 0% | 80% | good [~] |
| `aws_shield_protection_group` | 1 | 0 | 8 | 0% | 0% | poor [!] |
| `aws_shield_protection_health_check_association` | 1 | 3 | 2 | 0% | 100% | good [~] |
| `aws_shield_subscription` | 1 | 6 | 2 | 0% | 50% | good [~] |
| `aws_signer_signing_profile` | 1 | 12 | 13 | 0% | 69% | good [~] |
| `aws_simpledb_domain` | 1 | 2 | 1 | 0% | 100% | good [~] |
| `aws_sns_platform_application` | 1 | 6 | 14 | 0% | 36% | good [~] |
| `aws_sns_sms_preferences` | 1 | 7 | 6 | 0% | 100% | good [~] |
| `aws_sns_topic` | 1 | 0 | 32 | 0% | 0% | poor [!] |
| `aws_sns_topic_data_protection_policy` | 1 | 0 | 2 | 0% | 0% | poor [!] |
| `aws_sns_topic_policy` | 1 | 0 | 3 | 0% | 0% | poor [!] |
| `aws_sns_topic_subscription` | 1 | 12 | 16 | 0% | 50% | good [~] |
| `aws_sqs_queue` | 1 | 13 | 22 | 0% | 55% | good [~] |
| `aws_sqs_queue_policy` | 1 | 0 | 2 | 0% | 0% | poor [!] |
| `aws_sqs_queue_redrive_allow_policy` | 0 | 0 | 2 | 0% | 0% | poor [!] |
| `aws_sqs_queue_redrive_policy` | 1 | 0 | 2 | 0% | 0% | poor [!] |
| `aws_ssm_activation` | 1 | 12 | 10 | 0% | 100% | good [~] |
| `aws_ssm_association` | 1 | 12 | 19 | 0% | 47% | good [~] |
| `aws_ssm_default_patch_baseline` | 1 | 3 | 2 | 0% | 100% | good [~] |
| `aws_ssm_document` | 1 | 13 | 23 | 0% | 52% | good [~] |
| `aws_ssm_maintenance_window` | 1 | 8 | 13 | 0% | 54% | good [~] |
| `aws_ssm_maintenance_window_target` | 1 | 3 | 6 | 0% | 33% | good [~] |
| `aws_ssm_maintenance_window_task` | 1 | 5 | 14 | 0% | 29% | fair [-] |
| `aws_ssm_parameter` | 1 | 11 | 17 | 0% | 53% | good [~] |
| `aws_ssm_patch_baseline` | 1 | 7 | 15 | 0% | 40% | good [~] |
| `aws_ssm_patch_group` | 1 | 3 | 2 | 0% | 100% | good [~] |
| `aws_ssm_resource_data_sync` | 2 | 6 | 2 | 50% | 100% | good [~] |
| `aws_ssm_service_setting` | 1 | 6 | 4 | 0% | 100% | good [~] |
| `aws_ssoadmin_account_assignment` | 1 | 6 | 7 | 0% | 71% | good [~] |
| `aws_ssoadmin_customer_managed_policy_attachment` | 1 | 5 | 4 | 0% | 50% | good [~] |
| `aws_ssoadmin_managed_policy_attachment` | 1 | 4 | 5 | 0% | 60% | good [~] |
| `aws_ssoadmin_permission_set` | 1 | 7 | 10 | 0% | 60% | good [~] |
| `aws_ssoadmin_permission_set_inline_policy` | 1 | 3 | 4 | 0% | 50% | good [~] |
| `aws_sfn_state_machine` | 6 | 17 | 19 | 26% | 53% | good [~] |
| `aws_swf_domain` | 1 | 5 | 7 | 0% | 57% | good [~] |
| `aws_synthetics_canary` | 8 | 26 | 24 | 25% | 67% | good [~] |
| `aws_timestreaminfluxdb_db_instance` | 4 | 17 | 24 | 12% | 67% | good [~] |
| `aws_timestreamquery_scheduled_query` | 5 | 22 | 18 | 22% | 78% | good [~] |
| `aws_timestreamwrite_database` | 1 | 6 | 6 | 0% | 83% | good [~] |
| `aws_timestreamwrite_table` | 3 | 10 | 8 | 25% | 75% | good [~] |
| `aws_transcribe_language_model` | 3 | 11 | 8 | 12% | 88% | good [~] |
| `aws_transcribe_vocabulary` | 3 | 9 | 9 | 11% | 56% | good [~] |
| `aws_transfer_access` | 3 | 0 | 8 | 25% | 0% | fair [-] |
| `aws_transfer_agreement` | 1 | 12 | 11 | 0% | 100% | good [~] |
| `aws_transfer_certificate` | 3 | 14 | 11 | 18% | 100% | good [~] |
| `aws_transfer_connector` | 3 | 10 | 10 | 20% | 90% | good [~] |
| `aws_transfer_profile` | 2 | 8 | 7 | 14% | 100% | good [~] |
| `aws_transfer_server` | 12 | 18 | 26 | 42% | 65% | good [~] |
| `aws_transfer_ssh_key` | 1 | 0 | 4 | 0% | 0% | poor [!] |
| `aws_transfer_tag` | 1 | 0 | 3 | 0% | 0% | poor [!] |
| `aws_transfer_user` | 4 | 11 | 12 | 25% | 83% | good [~] |
| `aws_transfer_workflow` | 3 | 7 | 6 | 33% | 100% | good [~] |
| `aws_vpclattice_access_log_subscription` | 1 | 6 | 7 | 0% | 71% | good [~] |
| `aws_vpclattice_auth_policy` | 1 | 3 | 4 | 0% | 75% | good [~] |
| `aws_vpclattice_listener` | 3 | 17 | 13 | 15% | 77% | good [~] |
| `aws_vpclattice_listener_rule` | 1 | 7 | 11 | 0% | 55% | good [~] |
| `aws_vpclattice_resource_configuration` | 2 | 8 | 12 | 8% | 58% | good [~] |
| `aws_vpclattice_resource_gateway` | 3 | 9 | 10 | 20% | 80% | good [~] |
| `aws_vpclattice_resource_policy` | 1 | 2 | 2 | 0% | 100% | good [~] |
| `aws_vpclattice_service` | 1 | 8 | 10 | 0% | 50% | good [~] |
| `aws_vpclattice_service_network` | 1 | 9 | 5 | 0% | 80% | good [~] |
| `aws_vpclattice_service_network_resource_association` | 1 | 6 | 7 | 0% | 57% | good [~] |
| `aws_vpclattice_service_network_service_association` | 1 | 6 | 10 | 0% | 50% | good [~] |
| `aws_vpclattice_service_network_vpc_association` | 2 | 7 | 9 | 11% | 67% | good [~] |
| `aws_vpclattice_target_group` | 2 | 15 | 8 | 12% | 75% | good [~] |
| `aws_vpclattice_target_group_attachment` | 4 | 4 | 3 | 33% | 67% | good [~] |
| `aws_wafv2_api_key` | 2 | 4 | 3 | 33% | 100% | good [~] |
| `aws_wafv2_ip_set` | 3 | 7 | 10 | 20% | 60% | good [~] |
| `aws_wafv2_regex_pattern_set` | 3 | 10 | 9 | 22% | 89% | good [~] |
| `aws_wafv2_rule_group` | 5 | 12 | 12 | 33% | 92% | good [~] |
| `aws_wafv2_web_acl` | 12 | 18 | 20 | 55% | 85% | good [~] |
| `aws_wafv2_web_acl_association` | 1 | 3 | 3 | 0% | 67% | good [~] |
| `aws_wafv2_web_acl_logging_configuration` | 4 | 7 | 4 | 75% | 100% | excellent [+] |
| `aws_workspaces_directory` | 10 | 19 | 23 | 35% | 70% | good [~] |
| `aws_workspaces_workspace` | 4 | 17 | 13 | 23% | 77% | good [~] |
| `aws_xray_group` | 2 | 6 | 6 | 17% | 50% | good [~] |
| `aws_xray_sampling_rule` | 4 | 15 | 15 | 7% | 80% | good [~] |

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
