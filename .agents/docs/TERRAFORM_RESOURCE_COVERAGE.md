# Per-service Terraform resource coverage

Schema resources total: **1526**
Resources classified to a service via prefix: **1191**
Currently handled by winterbaume: **1115** (1110 verified against schema)
Missing within classified prefixes: **89**

Sorted by missing-count desc.

| Service | Prefix | Handled | Schema | Missing | Coverage | Note |
|---------|--------|---------|--------|---------|----------|------|
| ebs | `aws_ebs_` | 2 | 8 | 6 | 25% |  |
| elbv2 | override (2 patterns) | 10 | 16 | 6 | 62% |  |
| kinesis | `aws_kinesis_` | 3 | 6 | 3 | 50% |  |
| medialive | `aws_medialive_` | 2 | 5 | 3 | 40% |  |
| networkfirewall | `aws_networkfirewall_` | 3 | 6 | 3 | 50% |  |
| s3 | `aws_s3_` | 23 | 26 | 3 | 88% |  |
| s3tables | `aws_s3tables_` | 2 | 5 | 3 | 40% |  |
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
| mediapackage | `aws_media_package_` | 1 | 1 | 0 | 100% |  |
| mediapackagev2 | `aws_media_packagev2_` | 1 | 1 | 0 | 100% |  |
| memorydb | `aws_memorydb_` | 7 | 7 | 0 | 100% |  |
| mq | `aws_mq_` | 2 | 2 | 0 | 100% |  |
| neptune | `aws_neptune_` | 9 | 9 | 0 | 100% |  |
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
| s3control | `aws_s3control_` | 15 | 14 | 0 | 100% |  |
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

## Per-service missing resources

### ebs — 6 missing

- `aws_ebs_default_kms_key`
- `aws_ebs_encryption_by_default`
- `aws_ebs_fast_snapshot_restore`
- `aws_ebs_snapshot_block_public_access`
- `aws_ebs_snapshot_copy`
- `aws_ebs_snapshot_import`

### elbv2 — 6 missing

- `aws_alb`
- `aws_alb_listener`
- `aws_alb_listener_certificate`
- `aws_alb_listener_rule`
- `aws_alb_target_group`
- `aws_alb_target_group_attachment`

### kinesis — 3 missing

- `aws_kinesis_analytics_application`
- `aws_kinesis_firehose_delivery_stream`
- `aws_kinesis_video_stream`

### medialive — 3 missing

- `aws_medialive_input_security_group`
- `aws_medialive_multiplex`
- `aws_medialive_multiplex_program`

### networkfirewall — 3 missing

- `aws_networkfirewall_logging_configuration`
- `aws_networkfirewall_resource_policy`
- `aws_networkfirewall_tls_inspection_configuration`

### s3 — 3 missing

- `aws_s3_access_point`
- `aws_s3_account_public_access_block`
- `aws_s3_bucket_inventory`

### s3tables — 3 missing

- `aws_s3tables_table`
- `aws_s3tables_table_bucket_policy`
- `aws_s3tables_table_policy`

### account — 2 missing

- `aws_account_primary_contact`
- `aws_account_region`

### apigateway — 2 missing

- `aws_api_gateway_method_settings`
- `aws_api_gateway_rest_api_put`

### cloudtrail — 2 missing

- `aws_cloudtrail_event_data_store`
- `aws_cloudtrail_organization_delegated_admin_account`

### codeartifact — 2 missing

- `aws_codeartifact_domain_permissions_policy`
- `aws_codeartifact_repository_permissions_policy`

### codepipeline — 2 missing

- `aws_codepipeline_custom_action_type`
- `aws_codepipeline_webhook`

### costexplorer — 2 missing

- `aws_ce_cost_allocation_tag`
- `aws_ce_cost_category`

### elasticbeanstalk — 2 missing

- `aws_elastic_beanstalk_application_version`
- `aws_elastic_beanstalk_configuration_template`

### iam — 2 missing

- `aws_iam_organizations_features`
- `aws_iam_security_token_service_preferences`

### ivs — 2 missing

- `aws_ivs_playback_key_pair`
- `aws_ivs_recording_configuration`

### rds — 2 missing

- `aws_db_proxy_target`
- `aws_rds_instance_state`

### rekognition — 2 missing

- `aws_rekognition_project`
- `aws_rekognition_stream_processor`

### route53domains — 2 missing

- `aws_route53domains_delegation_signer_record`
- `aws_route53domains_domain`

### secretsmanager — 2 missing

- `aws_secretsmanager_secret_policy`
- `aws_secretsmanager_secret_rotation`

### servicecatalogappregistry — 2 missing

- `aws_servicecatalogappregistry_attribute_group`
- `aws_servicecatalogappregistry_attribute_group_association`

### servicequotas — 2 missing

- `aws_servicequotas_template`
- `aws_servicequotas_template_association`

### signer — 2 missing

- `aws_signer_signing_job`
- `aws_signer_signing_profile_permission`

### stepfunctions — 2 missing

- `aws_sfn_activity`
- `aws_sfn_alias`

### synthetics — 2 missing

- `aws_synthetics_group`
- `aws_synthetics_group_association`

### transcribe — 2 missing

- `aws_transcribe_medical_vocabulary`
- `aws_transcribe_vocabulary_filter`

### workspaces — 2 missing

- `aws_workspaces_connection_alias`
- `aws_workspaces_ip_group`

### xray — 2 missing

- `aws_xray_encryption_config`
- `aws_xray_resource_policy`

### accessanalyzer — 1 missing

- `aws_accessanalyzer_archive_rule`

### acm — 1 missing

- `aws_acm_certificate_validation`

### appflow — 1 missing

- `aws_appflow_connector_profile`

### applicationautoscaling — 1 missing

- `aws_appautoscaling_scheduled_action`

### autoscaling — 1 missing

- `aws_launch_template`

### bedrock — 1 missing

- `aws_bedrock_model_invocation_logging_configuration`

### budgets — 1 missing

- `aws_budgets_budget_action`

### chatbot — 1 missing

- `aws_chatbot_teams_channel_configuration`

### cloudhsmv2 — 1 missing

- `aws_cloudhsm_v2_hsm`

### codedeploy — 1 missing

- `aws_codedeploy_deployment_config`

### cognitoidentity — 1 missing

- `aws_cognito_identity_provider`

### comprehend — 1 missing

- `aws_comprehend_document_classifier`

### datapipeline — 1 missing

- `aws_datapipeline_pipeline_definition`

### dsql — 1 missing

- `aws_dsql_cluster_peering`

### elasticloadbalancing — 1 missing

- `aws_elb_attachment`

### emrcontainers — 1 missing

- `aws_emrcontainers_job_template`

### glacier — 1 missing

- `aws_glacier_vault_lock`

### identitystore — 1 missing

- `aws_identitystore_group_membership`

### mediastore — 1 missing

- `aws_media_store_container_policy`

### resourcegroups — 1 missing

- `aws_resourcegroups_resource`

## Resources declared in specs but absent from the AWS provider schema

### chatbot

- `aws_chatbot_microsoft_teams_channel_configuration` (typo or removed in schema)

### applicationcostprofiler

- `aws_applicationcostprofiler_report_definition` (typo or removed in schema)

### outposts

- `aws_outposts_outpost` (typo or removed in schema)
- `aws_outposts_site` (typo or removed in schema)

### s3control

- `aws_s3control_access_point` (typo or removed in schema)

