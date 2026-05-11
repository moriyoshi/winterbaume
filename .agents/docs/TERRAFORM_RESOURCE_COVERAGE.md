# Per-service Terraform resource coverage

Schema resources total: **1526**
Resources classified to a service via prefix: **1193**
Currently handled by winterbaume: **365** (360 verified against schema)
Missing within classified prefixes: **833**

Sorted by missing-count desc.

| Service | Prefix | Handled | Schema | Missing | Coverage | Note |
|---------|--------|---------|--------|---------|----------|------|
| ec2 | override (37 patterns) | 39 | 139 | 100 | 28% |  |
| sagemaker | `aws_sagemaker_` | 5 | 30 | 25 | 17% |  |
| rds | override (2 patterns) | 7 | 29 | 22 | 24% |  |
| redshift | `aws_redshift_` | 2 | 23 | 21 | 9% |  |
| apigateway | `aws_api_gateway_` | 6 | 26 | 20 | 23% |  |
| directconnect | `aws_dx_` | 1 | 19 | 18 | 5% |  |
| glue | `aws_glue_` | 3 | 20 | 17 | 15% |  |
| quicksight | `aws_quicksight_` | 3 | 20 | 17 | 15% |  |
| networkmanager | `aws_networkmanager_` | 3 | 19 | 16 | 16% |  |
| cloudfront | `aws_cloudfront_` | 1 | 16 | 15 | 6% |  |
| connect | `aws_connect_` | 1 | 16 | 15 | 6% |  |
| iot | `aws_iot_` | 4 | 19 | 15 | 21% |  |
| elbv2 | override (2 patterns) | 3 | 16 | 13 | 19% |  |
| logs | `aws_cloudwatch_log_` | 2 | 15 | 13 | 13% |  |
| s3control | `aws_s3control_` | 2 | 14 | 13 | 7% |  |
| securityhub | `aws_securityhub_` | 2 | 15 | 13 | 13% |  |
| sesv1 | `aws_ses_` | 2 | 14 | 12 | 14% |  |
| apigatewayv2 | `aws_apigatewayv2_` | 1 | 12 | 11 | 8% |  |
| backup | `aws_backup_` | 2 | 13 | 11 | 15% |  |
| datasync | `aws_datasync_` | 2 | 13 | 11 | 15% |  |
| servicecatalog | `aws_servicecatalog_` | 2 | 13 | 11 | 15% |  |
| ssm | `aws_ssm_` | 1 | 12 | 11 | 8% |  |
| config | `aws_config_` | 3 | 13 | 10 | 23% |  |
| guardduty | `aws_guardduty_` | 3 | 13 | 10 | 23% |  |
| pinpoint | `aws_pinpoint_` | 2 | 12 | 10 | 17% |  |
| route53resolver | `aws_route53_resolver_` | 2 | 12 | 10 | 17% |  |
| ssoadmin | `aws_ssoadmin_` | 2 | 12 | 10 | 17% |  |
| vpclattice | `aws_vpclattice_` | 4 | 14 | 10 | 29% |  |
| appsync | `aws_appsync_` | 1 | 10 | 9 | 10% |  |
| fsx | `aws_fsx_` | 2 | 11 | 9 | 18% |  |
| lambda | `aws_lambda_` | 4 | 13 | 9 | 31% |  |
| apprunner | `aws_apprunner_` | 1 | 9 | 8 | 11% |  |
| dynamodb | `aws_dynamodb_` | 1 | 9 | 8 | 11% |  |
| ecr | `aws_ecr_` | 1 | 9 | 8 | 11% |  |
| opensearch | `aws_opensearch_` | 1 | 9 | 8 | 11% |  |
| ses | `aws_sesv2_` | 3 | 11 | 8 | 27% |  |
| transfer | `aws_transfer_` | 2 | 10 | 8 | 20% |  |
| cognitoidp | override (5 patterns) | 2 | 9 | 7 | 22% |  |
| directory | `aws_directory_service_` | 1 | 8 | 7 | 12% |  |
| kafka | `aws_msk_` | 1 | 8 | 7 | 12% |  |
| kms | `aws_kms_` | 2 | 9 | 7 | 22% |  |
| macie2 | `aws_macie2_` | 2 | 9 | 7 | 22% |  |
| shield | `aws_shield_` | 1 | 8 | 7 | 12% |  |
| auditmanager | `aws_auditmanager_` | 2 | 8 | 6 | 25% |  |
| autoscaling | override (3 patterns) | 4 | 10 | 6 | 40% |  |
| bedrockagent | `aws_bedrockagent_` | 2 | 8 | 6 | 25% |  |
| ebs | `aws_ebs_` | 2 | 8 | 6 | 25% |  |
| eks | `aws_eks_` | 2 | 8 | 6 | 25% |  |
| elasticache | `aws_elasticache_` | 4 | 10 | 6 | 40% |  |
| emr | `aws_emr_` | 2 | 8 | 6 | 25% |  |
| events | `aws_cloudwatch_event_` | 3 | 9 | 6 | 33% |  |
| lakeformation | `aws_lakeformation_` | 2 | 8 | 6 | 25% |  |
| bedrock | `aws_bedrock_` | 1 | 6 | 5 | 17% |  |
| codebuild | `aws_codebuild_` | 1 | 6 | 5 | 17% |  |
| dms | `aws_dms_` | 3 | 8 | 5 | 38% |  |
| ecs | `aws_ecs_` | 3 | 8 | 5 | 38% |  |
| efs | `aws_efs_` | 1 | 6 | 5 | 17% |  |
| kinesis | `aws_kinesis_` | 1 | 6 | 5 | 17% |  |
| lexmodelsv2 | `aws_lexv2models_` | 1 | 6 | 5 | 17% |  |
| neptune | `aws_neptune_` | 4 | 9 | 5 | 44% |  |
| acmpca | `aws_acmpca_` | 1 | 5 | 4 | 20% |  |
| amp | `aws_prometheus_` | 1 | 5 | 4 | 20% |  |
| appconfig | `aws_appconfig_` | 4 | 8 | 4 | 50% |  |
| appfabric | `aws_appfabric_` | 1 | 5 | 4 | 20% |  |
| appmesh | `aws_appmesh_` | 3 | 7 | 4 | 43% |  |
| athena | `aws_athena_` | 2 | 6 | 4 | 33% |  |
| cloudformation | `aws_cloudformation_` | 1 | 5 | 4 | 20% |  |
| inspector2 | `aws_inspector2_` | 1 | 5 | 4 | 20% |  |
| memorydb | `aws_memorydb_` | 3 | 7 | 4 | 43% |  |
| opensearchserverless | `aws_opensearchserverless_` | 2 | 6 | 4 | 33% |  |
| organizations | `aws_organizations_` | 3 | 7 | 4 | 43% |  |
| ram | `aws_ram_` | 1 | 5 | 4 | 20% |  |
| sns | `aws_sns_` | 2 | 6 | 4 | 33% |  |
| wafv2 | `aws_wafv2_` | 3 | 7 | 4 | 43% |  |
| amplify | `aws_amplify_` | 2 | 5 | 3 | 40% |  |
| cloudwatch | override (3 patterns) | 1 | 4 | 3 | 25% |  |
| codecommit | `aws_codecommit_` | 1 | 4 | 3 | 25% |  |
| cognitoidentity | `aws_cognito_identity_` | 1 | 4 | 3 | 25% |  |
| medialive | `aws_medialive_` | 2 | 5 | 3 | 40% |  |
| networkfirewall | `aws_networkfirewall_` | 3 | 6 | 3 | 50% |  |
| s3 | `aws_s3_` | 23 | 26 | 3 | 88% |  |
| s3tables | `aws_s3tables_` | 2 | 5 | 3 | 40% |  |
| servicediscovery | `aws_service_discovery_` | 2 | 5 | 3 | 40% |  |
| sqs | `aws_sqs_` | 1 | 4 | 3 | 25% |  |
| account | `aws_account_` | 1 | 3 | 2 | 33% |  |
| cloudtrail | `aws_cloudtrail` | 1 | 3 | 2 | 33% |  |
| codeartifact | `aws_codeartifact_` | 2 | 4 | 2 | 50% |  |
| codepipeline | `aws_codepipeline` | 1 | 3 | 2 | 33% |  |
| costexplorer | `aws_ce_` | 2 | 4 | 2 | 50% |  |
| elasticbeanstalk | `aws_elastic_beanstalk_` | 2 | 4 | 2 | 50% |  |
| iam | `aws_iam_` | 32 | 34 | 2 | 94% |  |
| ivs | `aws_ivs_` | 1 | 3 | 2 | 33% |  |
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
| budgets | `aws_budgets_` | 1 | 2 | 1 | 50% |  |
| chatbot | `aws_chatbot_` | 2 | 2 | 1 | 50% |  |
| cloudhsmv2 | `aws_cloudhsm_v2_` | 1 | 2 | 1 | 50% |  |
| codedeploy | `aws_codedeploy_` | 2 | 3 | 1 | 67% |  |
| comprehend | `aws_comprehend_` | 1 | 2 | 1 | 50% |  |
| datapipeline | `aws_datapipeline_` | 1 | 2 | 1 | 50% |  |
| dsql | `aws_dsql_` | 1 | 2 | 1 | 50% |  |
| elasticloadbalancing | `aws_elb` | 1 | 2 | 1 | 50% |  |
| emrcontainers | `aws_emrcontainers_` | 1 | 2 | 1 | 50% |  |
| glacier | `aws_glacier_` | 1 | 2 | 1 | 50% |  |
| identitystore | `aws_identitystore_` | 2 | 3 | 1 | 67% |  |
| kinesisanalyticsv2 | `aws_kinesisanalyticsv2_` | 1 | 2 | 1 | 50% |  |
| mediastore | override (2 patterns) | 1 | 2 | 1 | 50% |  |
| resourcegroups | `aws_resourcegroups_` | 1 | 2 | 1 | 50% |  |
| applicationcostprofiler | `aws_applicationcostprofiler_` | 1 | 0 | 0 | — |  |
| batch | `aws_batch_` | 4 | 4 | 0 | 100% |  |
| dax | `aws_dax_` | 3 | 3 | 0 | 100% |  |
| ec2instanceconnect | `aws_ec2_instance_connect_` | 1 | 1 | 0 | 100% |  |
| emrserverless | `aws_emrserverless_` | 1 | 1 | 0 | 100% |  |
| firehose | `aws_kinesis_firehose_` | 1 | 1 | 0 | 100% |  |
| fis | `aws_fis_` | 1 | 1 | 0 | 100% |  |
| keyspaces | `aws_keyspaces_` | 2 | 2 | 0 | 100% |  |
| kinesisvideo | `aws_kinesis_video_` | 1 | 1 | 0 | 100% |  |
| mediapackage | `aws_media_package_` | 1 | 1 | 0 | 100% |  |
| mediapackagev2 | `aws_media_packagev2_` | 1 | 1 | 0 | 100% |  |
| mq | `aws_mq_` | 2 | 2 | 0 | 100% |  |
| osis | `aws_osis_` | 1 | 1 | 0 | 100% |  |
| outposts | `aws_outposts_` | 2 | 0 | 0 | — |  |
| pipes | `aws_pipes_` | 1 | 1 | 0 | 100% |  |
| resiliencehub | `aws_resiliencehub_` | 1 | 1 | 0 | 100% |  |
| rolesanywhere | `aws_rolesanywhere_` | 2 | 2 | 0 | 100% |  |
| route53 | override (11 patterns) | 13 | 13 | 0 | 100% |  |
| scheduler | `aws_scheduler_` | 2 | 2 | 0 | 100% |  |
| simpledbv2 | `aws_simpledb_` | 1 | 1 | 0 | 100% |  |
| swf | `aws_swf_` | 1 | 1 | 0 | 100% |  |
| timestreaminfluxdb | `aws_timestreaminfluxdb_` | 1 | 1 | 0 | 100% |  |
| timestreamquery | `aws_timestreamquery_` | 1 | 1 | 0 | 100% |  |
| timestreamwrite | `aws_timestreamwrite_` | 2 | 2 | 0 | 100% |  |

## Per-service missing resources

### ec2 — 100 missing

- `aws_ami`
- `aws_ami_copy`
- `aws_ami_from_instance`
- `aws_ami_launch_permission`
- `aws_default_network_acl`
- `aws_default_route_table`
- `aws_default_security_group`
- `aws_default_subnet`
- `aws_default_vpc`
- `aws_default_vpc_dhcp_options`
- `aws_ec2_availability_zone_group`
- `aws_ec2_carrier_gateway`
- `aws_ec2_client_vpn_authorization_rule`
- `aws_ec2_client_vpn_endpoint`
- `aws_ec2_client_vpn_network_association`
- `aws_ec2_client_vpn_route`
- `aws_ec2_default_credit_specification`
- `aws_ec2_fleet`
- `aws_ec2_host`
- `aws_ec2_image_block_public_access`
- `aws_ec2_instance_metadata_defaults`
- `aws_ec2_instance_state`
- `aws_ec2_managed_prefix_list`
- `aws_ec2_managed_prefix_list_entry`
- `aws_ec2_serial_console_access`
- `aws_ec2_subnet_cidr_reservation`
- `aws_ec2_tag`
- `aws_ec2_transit_gateway`
- `aws_ec2_transit_gateway_connect_peer`
- `aws_ec2_transit_gateway_default_route_table_association`
- `aws_ec2_transit_gateway_default_route_table_propagation`
- `aws_ec2_transit_gateway_multicast_domain_association`
- `aws_ec2_transit_gateway_multicast_group_member`
- `aws_ec2_transit_gateway_multicast_group_source`
- `aws_ec2_transit_gateway_peering_attachment`
- `aws_ec2_transit_gateway_peering_attachment_accepter`
- `aws_ec2_transit_gateway_policy_table_association`
- `aws_ec2_transit_gateway_prefix_list_reference`
- `aws_ec2_transit_gateway_route`
- `aws_ec2_transit_gateway_route_table`
- `aws_ec2_transit_gateway_route_table_association`
- `aws_ec2_transit_gateway_route_table_propagation`
- `aws_ec2_transit_gateway_vpc_attachment`
- `aws_ec2_transit_gateway_vpc_attachment_accepter`
- `aws_eip_association`
- `aws_eip_domain_name`
- `aws_flow_log`
- `aws_instance`
- `aws_internet_gateway_attachment`
- `aws_launch_template`
- `aws_main_route_table_association`
- `aws_network_acl_association`
- `aws_network_interface`
- `aws_network_interface_attachment`
- `aws_network_interface_sg_attachment`
- `aws_route_table_association`
- `aws_security_group_rule`
- `aws_spot_datafeed_subscription`
- `aws_spot_fleet_request`
- `aws_spot_instance_request`
- `aws_verifiedaccess_instance_logging_configuration`
- `aws_verifiedaccess_instance_trust_provider_attachment`
- `aws_volume_attachment`
- `aws_vpc_block_public_access_exclusion`
- `aws_vpc_block_public_access_options`
- `aws_vpc_dhcp_options`
- `aws_vpc_dhcp_options_association`
- `aws_vpc_endpoint`
- `aws_vpc_endpoint_connection_accepter`
- `aws_vpc_endpoint_connection_notification`
- `aws_vpc_endpoint_policy`
- `aws_vpc_endpoint_private_dns`
- `aws_vpc_endpoint_route_table_association`
- `aws_vpc_endpoint_security_group_association`
- `aws_vpc_endpoint_service`
- `aws_vpc_endpoint_service_allowed_principal`
- `aws_vpc_endpoint_service_private_dns_verification`
- `aws_vpc_endpoint_subnet_association`
- `aws_vpc_ipam_organization_admin_account`
- `aws_vpc_ipam_preview_next_cidr`
- `aws_vpc_ipam_resource_discovery_association`
- `aws_vpc_ipv4_cidr_block_association`
- `aws_vpc_ipv6_cidr_block_association`
- `aws_vpc_network_performance_metric_subscription`
- `aws_vpc_peering_connection`
- `aws_vpc_peering_connection_accepter`
- `aws_vpc_peering_connection_options`
- `aws_vpc_route_server`
- `aws_vpc_route_server_endpoint`
- `aws_vpc_route_server_peer`
- `aws_vpc_route_server_propagation`
- `aws_vpc_route_server_vpc_association`
- `aws_vpc_security_group_egress_rule`
- `aws_vpc_security_group_ingress_rule`
- `aws_vpc_security_group_vpc_association`
- `aws_vpn_connection`
- `aws_vpn_connection_route`
- `aws_vpn_gateway`
- `aws_vpn_gateway_attachment`
- `aws_vpn_gateway_route_propagation`

### sagemaker — 25 missing

- `aws_sagemaker_app`
- `aws_sagemaker_app_image_config`
- `aws_sagemaker_code_repository`
- `aws_sagemaker_data_quality_job_definition`
- `aws_sagemaker_device`
- `aws_sagemaker_device_fleet`
- `aws_sagemaker_feature_group`
- `aws_sagemaker_flow_definition`
- `aws_sagemaker_hub`
- `aws_sagemaker_human_task_ui`
- `aws_sagemaker_image`
- `aws_sagemaker_image_version`
- `aws_sagemaker_mlflow_tracking_server`
- `aws_sagemaker_model_package_group`
- `aws_sagemaker_model_package_group_policy`
- `aws_sagemaker_monitoring_schedule`
- `aws_sagemaker_notebook_instance_lifecycle_configuration`
- `aws_sagemaker_pipeline`
- `aws_sagemaker_project`
- `aws_sagemaker_servicecatalog_portfolio_status`
- `aws_sagemaker_space`
- `aws_sagemaker_studio_lifecycle_config`
- `aws_sagemaker_user_profile`
- `aws_sagemaker_workforce`
- `aws_sagemaker_workteam`

### rds — 22 missing

- `aws_db_cluster_snapshot`
- `aws_db_instance_automated_backups_replication`
- `aws_db_instance_role_association`
- `aws_db_proxy`
- `aws_db_proxy_default_target_group`
- `aws_db_proxy_endpoint`
- `aws_db_proxy_target`
- `aws_db_snapshot`
- `aws_db_snapshot_copy`
- `aws_rds_certificate`
- `aws_rds_cluster_activity_stream`
- `aws_rds_cluster_endpoint`
- `aws_rds_cluster_instance`
- `aws_rds_cluster_role_association`
- `aws_rds_cluster_snapshot_copy`
- `aws_rds_custom_db_engine_version`
- `aws_rds_export_task`
- `aws_rds_global_cluster`
- `aws_rds_instance_state`
- `aws_rds_integration`
- `aws_rds_reserved_instance`
- `aws_rds_shard_group`

### redshift — 21 missing

- `aws_redshift_authentication_profile`
- `aws_redshift_cluster_iam_roles`
- `aws_redshift_cluster_snapshot`
- `aws_redshift_data_share_authorization`
- `aws_redshift_data_share_consumer_association`
- `aws_redshift_endpoint_access`
- `aws_redshift_endpoint_authorization`
- `aws_redshift_event_subscription`
- `aws_redshift_hsm_client_certificate`
- `aws_redshift_hsm_configuration`
- `aws_redshift_integration`
- `aws_redshift_logging`
- `aws_redshift_parameter_group`
- `aws_redshift_partner`
- `aws_redshift_resource_policy`
- `aws_redshift_scheduled_action`
- `aws_redshift_snapshot_copy`
- `aws_redshift_snapshot_copy_grant`
- `aws_redshift_snapshot_schedule`
- `aws_redshift_snapshot_schedule_association`
- `aws_redshift_usage_limit`

### apigateway — 20 missing

- `aws_api_gateway_account`
- `aws_api_gateway_authorizer`
- `aws_api_gateway_base_path_mapping`
- `aws_api_gateway_client_certificate`
- `aws_api_gateway_documentation_part`
- `aws_api_gateway_documentation_version`
- `aws_api_gateway_domain_name`
- `aws_api_gateway_domain_name_access_association`
- `aws_api_gateway_gateway_response`
- `aws_api_gateway_integration`
- `aws_api_gateway_integration_response`
- `aws_api_gateway_method_response`
- `aws_api_gateway_method_settings`
- `aws_api_gateway_model`
- `aws_api_gateway_request_validator`
- `aws_api_gateway_rest_api_policy`
- `aws_api_gateway_rest_api_put`
- `aws_api_gateway_usage_plan`
- `aws_api_gateway_usage_plan_key`
- `aws_api_gateway_vpc_link`

### directconnect — 18 missing

- `aws_dx_bgp_peer`
- `aws_dx_connection_association`
- `aws_dx_connection_confirmation`
- `aws_dx_gateway`
- `aws_dx_gateway_association`
- `aws_dx_gateway_association_proposal`
- `aws_dx_hosted_connection`
- `aws_dx_hosted_private_virtual_interface`
- `aws_dx_hosted_private_virtual_interface_accepter`
- `aws_dx_hosted_public_virtual_interface`
- `aws_dx_hosted_public_virtual_interface_accepter`
- `aws_dx_hosted_transit_virtual_interface`
- `aws_dx_hosted_transit_virtual_interface_accepter`
- `aws_dx_lag`
- `aws_dx_macsec_key_association`
- `aws_dx_private_virtual_interface`
- `aws_dx_public_virtual_interface`
- `aws_dx_transit_virtual_interface`

### glue — 17 missing

- `aws_glue_catalog_table`
- `aws_glue_catalog_table_optimizer`
- `aws_glue_classifier`
- `aws_glue_connection`
- `aws_glue_data_catalog_encryption_settings`
- `aws_glue_data_quality_ruleset`
- `aws_glue_dev_endpoint`
- `aws_glue_ml_transform`
- `aws_glue_partition`
- `aws_glue_partition_index`
- `aws_glue_registry`
- `aws_glue_resource_policy`
- `aws_glue_schema`
- `aws_glue_security_configuration`
- `aws_glue_trigger`
- `aws_glue_user_defined_function`
- `aws_glue_workflow`

### quicksight — 17 missing

- `aws_quicksight_account_settings`
- `aws_quicksight_account_subscription`
- `aws_quicksight_analysis`
- `aws_quicksight_dashboard`
- `aws_quicksight_data_set`
- `aws_quicksight_folder`
- `aws_quicksight_folder_membership`
- `aws_quicksight_group_membership`
- `aws_quicksight_iam_policy_assignment`
- `aws_quicksight_ingestion`
- `aws_quicksight_namespace`
- `aws_quicksight_refresh_schedule`
- `aws_quicksight_role_membership`
- `aws_quicksight_template`
- `aws_quicksight_template_alias`
- `aws_quicksight_theme`
- `aws_quicksight_vpc_connection`

### networkmanager — 16 missing

- `aws_networkmanager_attachment_accepter`
- `aws_networkmanager_connect_attachment`
- `aws_networkmanager_connect_peer`
- `aws_networkmanager_connection`
- `aws_networkmanager_core_network`
- `aws_networkmanager_core_network_policy_attachment`
- `aws_networkmanager_customer_gateway_association`
- `aws_networkmanager_dx_gateway_attachment`
- `aws_networkmanager_link`
- `aws_networkmanager_link_association`
- `aws_networkmanager_site_to_site_vpn_attachment`
- `aws_networkmanager_transit_gateway_connect_peer_association`
- `aws_networkmanager_transit_gateway_peering`
- `aws_networkmanager_transit_gateway_registration`
- `aws_networkmanager_transit_gateway_route_table_attachment`
- `aws_networkmanager_vpc_attachment`

### cloudfront — 15 missing

- `aws_cloudfront_cache_policy`
- `aws_cloudfront_continuous_deployment_policy`
- `aws_cloudfront_field_level_encryption_config`
- `aws_cloudfront_field_level_encryption_profile`
- `aws_cloudfront_function`
- `aws_cloudfront_key_group`
- `aws_cloudfront_key_value_store`
- `aws_cloudfront_monitoring_subscription`
- `aws_cloudfront_origin_access_control`
- `aws_cloudfront_origin_access_identity`
- `aws_cloudfront_origin_request_policy`
- `aws_cloudfront_public_key`
- `aws_cloudfront_realtime_log_config`
- `aws_cloudfront_response_headers_policy`
- `aws_cloudfront_vpc_origin`

### connect — 15 missing

- `aws_connect_bot_association`
- `aws_connect_contact_flow`
- `aws_connect_contact_flow_module`
- `aws_connect_hours_of_operation`
- `aws_connect_instance_storage_config`
- `aws_connect_lambda_function_association`
- `aws_connect_phone_number`
- `aws_connect_queue`
- `aws_connect_quick_connect`
- `aws_connect_routing_profile`
- `aws_connect_security_profile`
- `aws_connect_user`
- `aws_connect_user_hierarchy_group`
- `aws_connect_user_hierarchy_structure`
- `aws_connect_vocabulary`

### iot — 15 missing

- `aws_iot_authorizer`
- `aws_iot_billing_group`
- `aws_iot_ca_certificate`
- `aws_iot_domain_configuration`
- `aws_iot_event_configurations`
- `aws_iot_indexing_configuration`
- `aws_iot_logging_options`
- `aws_iot_policy_attachment`
- `aws_iot_provisioning_template`
- `aws_iot_role_alias`
- `aws_iot_thing_group`
- `aws_iot_thing_group_membership`
- `aws_iot_thing_principal_attachment`
- `aws_iot_topic_rule`
- `aws_iot_topic_rule_destination`

### elbv2 — 13 missing

- `aws_alb`
- `aws_alb_listener`
- `aws_alb_listener_certificate`
- `aws_alb_listener_rule`
- `aws_alb_target_group`
- `aws_alb_target_group_attachment`
- `aws_lb_cookie_stickiness_policy`
- `aws_lb_listener_certificate`
- `aws_lb_listener_rule`
- `aws_lb_ssl_negotiation_policy`
- `aws_lb_target_group_attachment`
- `aws_lb_trust_store`
- `aws_lb_trust_store_revocation`

### logs — 13 missing

- `aws_cloudwatch_log_account_policy`
- `aws_cloudwatch_log_anomaly_detector`
- `aws_cloudwatch_log_data_protection_policy`
- `aws_cloudwatch_log_delivery`
- `aws_cloudwatch_log_delivery_destination`
- `aws_cloudwatch_log_delivery_destination_policy`
- `aws_cloudwatch_log_delivery_source`
- `aws_cloudwatch_log_destination`
- `aws_cloudwatch_log_destination_policy`
- `aws_cloudwatch_log_index_policy`
- `aws_cloudwatch_log_metric_filter`
- `aws_cloudwatch_log_resource_policy`
- `aws_cloudwatch_log_subscription_filter`

### s3control — 13 missing

- `aws_s3control_access_grant`
- `aws_s3control_access_grants_instance`
- `aws_s3control_access_grants_instance_resource_policy`
- `aws_s3control_access_grants_location`
- `aws_s3control_access_point_policy`
- `aws_s3control_bucket_lifecycle_configuration`
- `aws_s3control_bucket_policy`
- `aws_s3control_directory_bucket_access_point_scope`
- `aws_s3control_multi_region_access_point`
- `aws_s3control_multi_region_access_point_policy`
- `aws_s3control_object_lambda_access_point`
- `aws_s3control_object_lambda_access_point_policy`
- `aws_s3control_storage_lens_configuration`

### securityhub — 13 missing

- `aws_securityhub_action_target`
- `aws_securityhub_automation_rule`
- `aws_securityhub_configuration_policy`
- `aws_securityhub_configuration_policy_association`
- `aws_securityhub_finding_aggregator`
- `aws_securityhub_insight`
- `aws_securityhub_invite_accepter`
- `aws_securityhub_member`
- `aws_securityhub_organization_admin_account`
- `aws_securityhub_organization_configuration`
- `aws_securityhub_product_subscription`
- `aws_securityhub_standards_control`
- `aws_securityhub_standards_control_association`

### sesv1 — 12 missing

- `aws_ses_active_receipt_rule_set`
- `aws_ses_configuration_set`
- `aws_ses_domain_dkim`
- `aws_ses_domain_identity_verification`
- `aws_ses_domain_mail_from`
- `aws_ses_event_destination`
- `aws_ses_identity_notification_topic`
- `aws_ses_identity_policy`
- `aws_ses_receipt_filter`
- `aws_ses_receipt_rule`
- `aws_ses_receipt_rule_set`
- `aws_ses_template`

### apigatewayv2 — 11 missing

- `aws_apigatewayv2_api_mapping`
- `aws_apigatewayv2_authorizer`
- `aws_apigatewayv2_deployment`
- `aws_apigatewayv2_domain_name`
- `aws_apigatewayv2_integration`
- `aws_apigatewayv2_integration_response`
- `aws_apigatewayv2_model`
- `aws_apigatewayv2_route`
- `aws_apigatewayv2_route_response`
- `aws_apigatewayv2_stage`
- `aws_apigatewayv2_vpc_link`

### backup — 11 missing

- `aws_backup_framework`
- `aws_backup_global_settings`
- `aws_backup_logically_air_gapped_vault`
- `aws_backup_region_settings`
- `aws_backup_report_plan`
- `aws_backup_restore_testing_plan`
- `aws_backup_restore_testing_selection`
- `aws_backup_selection`
- `aws_backup_vault_lock_configuration`
- `aws_backup_vault_notifications`
- `aws_backup_vault_policy`

### datasync — 11 missing

- `aws_datasync_agent`
- `aws_datasync_location_azure_blob`
- `aws_datasync_location_efs`
- `aws_datasync_location_fsx_lustre_file_system`
- `aws_datasync_location_fsx_ontap_file_system`
- `aws_datasync_location_fsx_openzfs_file_system`
- `aws_datasync_location_fsx_windows_file_system`
- `aws_datasync_location_hdfs`
- `aws_datasync_location_nfs`
- `aws_datasync_location_object_storage`
- `aws_datasync_location_smb`

### servicecatalog — 11 missing

- `aws_servicecatalog_budget_resource_association`
- `aws_servicecatalog_constraint`
- `aws_servicecatalog_organizations_access`
- `aws_servicecatalog_portfolio_share`
- `aws_servicecatalog_principal_portfolio_association`
- `aws_servicecatalog_product_portfolio_association`
- `aws_servicecatalog_provisioned_product`
- `aws_servicecatalog_provisioning_artifact`
- `aws_servicecatalog_service_action`
- `aws_servicecatalog_tag_option`
- `aws_servicecatalog_tag_option_resource_association`

### ssm — 11 missing

- `aws_ssm_activation`
- `aws_ssm_association`
- `aws_ssm_default_patch_baseline`
- `aws_ssm_document`
- `aws_ssm_maintenance_window`
- `aws_ssm_maintenance_window_target`
- `aws_ssm_maintenance_window_task`
- `aws_ssm_patch_baseline`
- `aws_ssm_patch_group`
- `aws_ssm_resource_data_sync`
- `aws_ssm_service_setting`

### config — 10 missing

- `aws_config_aggregate_authorization`
- `aws_config_configuration_aggregator`
- `aws_config_configuration_recorder_status`
- `aws_config_conformance_pack`
- `aws_config_organization_conformance_pack`
- `aws_config_organization_custom_policy_rule`
- `aws_config_organization_custom_rule`
- `aws_config_organization_managed_rule`
- `aws_config_remediation_configuration`
- `aws_config_retention_configuration`

### guardduty — 10 missing

- `aws_guardduty_detector_feature`
- `aws_guardduty_invite_accepter`
- `aws_guardduty_ipset`
- `aws_guardduty_malware_protection_plan`
- `aws_guardduty_member_detector_feature`
- `aws_guardduty_organization_admin_account`
- `aws_guardduty_organization_configuration`
- `aws_guardduty_organization_configuration_feature`
- `aws_guardduty_publishing_destination`
- `aws_guardduty_threatintelset`

### pinpoint — 10 missing

- `aws_pinpoint_adm_channel`
- `aws_pinpoint_apns_channel`
- `aws_pinpoint_apns_sandbox_channel`
- `aws_pinpoint_apns_voip_channel`
- `aws_pinpoint_apns_voip_sandbox_channel`
- `aws_pinpoint_baidu_channel`
- `aws_pinpoint_email_template`
- `aws_pinpoint_event_stream`
- `aws_pinpoint_gcm_channel`
- `aws_pinpoint_sms_channel`

### route53resolver — 10 missing

- `aws_route53_resolver_config`
- `aws_route53_resolver_dnssec_config`
- `aws_route53_resolver_firewall_config`
- `aws_route53_resolver_firewall_domain_list`
- `aws_route53_resolver_firewall_rule`
- `aws_route53_resolver_firewall_rule_group`
- `aws_route53_resolver_firewall_rule_group_association`
- `aws_route53_resolver_query_log_config`
- `aws_route53_resolver_query_log_config_association`
- `aws_route53_resolver_rule_association`

### ssoadmin — 10 missing

- `aws_ssoadmin_application`
- `aws_ssoadmin_application_access_scope`
- `aws_ssoadmin_application_assignment`
- `aws_ssoadmin_application_assignment_configuration`
- `aws_ssoadmin_customer_managed_policy_attachment`
- `aws_ssoadmin_instance_access_control_attributes`
- `aws_ssoadmin_managed_policy_attachment`
- `aws_ssoadmin_permission_set_inline_policy`
- `aws_ssoadmin_permissions_boundary_attachment`
- `aws_ssoadmin_trusted_token_issuer`

### vpclattice — 10 missing

- `aws_vpclattice_access_log_subscription`
- `aws_vpclattice_auth_policy`
- `aws_vpclattice_listener_rule`
- `aws_vpclattice_resource_configuration`
- `aws_vpclattice_resource_gateway`
- `aws_vpclattice_resource_policy`
- `aws_vpclattice_service_network_resource_association`
- `aws_vpclattice_service_network_service_association`
- `aws_vpclattice_service_network_vpc_association`
- `aws_vpclattice_target_group_attachment`

### appsync — 9 missing

- `aws_appsync_api_cache`
- `aws_appsync_api_key`
- `aws_appsync_datasource`
- `aws_appsync_domain_name`
- `aws_appsync_domain_name_api_association`
- `aws_appsync_function`
- `aws_appsync_resolver`
- `aws_appsync_source_api_association`
- `aws_appsync_type`

### fsx — 9 missing

- `aws_fsx_backup`
- `aws_fsx_data_repository_association`
- `aws_fsx_file_cache`
- `aws_fsx_ontap_file_system`
- `aws_fsx_ontap_storage_virtual_machine`
- `aws_fsx_ontap_volume`
- `aws_fsx_openzfs_file_system`
- `aws_fsx_openzfs_snapshot`
- `aws_fsx_openzfs_volume`

### lambda — 9 missing

- `aws_lambda_code_signing_config`
- `aws_lambda_function_event_invoke_config`
- `aws_lambda_function_recursion_config`
- `aws_lambda_function_url`
- `aws_lambda_invocation`
- `aws_lambda_layer_version`
- `aws_lambda_layer_version_permission`
- `aws_lambda_provisioned_concurrency_config`
- `aws_lambda_runtime_management_config`

### apprunner — 8 missing

- `aws_apprunner_auto_scaling_configuration_version`
- `aws_apprunner_connection`
- `aws_apprunner_custom_domain_association`
- `aws_apprunner_default_auto_scaling_configuration_version`
- `aws_apprunner_deployment`
- `aws_apprunner_observability_configuration`
- `aws_apprunner_vpc_connector`
- `aws_apprunner_vpc_ingress_connection`

### dynamodb — 8 missing

- `aws_dynamodb_contributor_insights`
- `aws_dynamodb_global_table`
- `aws_dynamodb_kinesis_streaming_destination`
- `aws_dynamodb_resource_policy`
- `aws_dynamodb_table_export`
- `aws_dynamodb_table_item`
- `aws_dynamodb_table_replica`
- `aws_dynamodb_tag`

### ecr — 8 missing

- `aws_ecr_account_setting`
- `aws_ecr_lifecycle_policy`
- `aws_ecr_pull_through_cache_rule`
- `aws_ecr_registry_policy`
- `aws_ecr_registry_scanning_configuration`
- `aws_ecr_replication_configuration`
- `aws_ecr_repository_creation_template`
- `aws_ecr_repository_policy`

### opensearch — 8 missing

- `aws_opensearch_authorize_vpc_endpoint_access`
- `aws_opensearch_domain_policy`
- `aws_opensearch_domain_saml_options`
- `aws_opensearch_inbound_connection_accepter`
- `aws_opensearch_outbound_connection`
- `aws_opensearch_package`
- `aws_opensearch_package_association`
- `aws_opensearch_vpc_endpoint`

### ses — 8 missing

- `aws_sesv2_account_suppression_attributes`
- `aws_sesv2_account_vdm_attributes`
- `aws_sesv2_configuration_set_event_destination`
- `aws_sesv2_contact_list`
- `aws_sesv2_dedicated_ip_assignment`
- `aws_sesv2_email_identity_feedback_attributes`
- `aws_sesv2_email_identity_mail_from_attributes`
- `aws_sesv2_email_identity_policy`

### transfer — 8 missing

- `aws_transfer_access`
- `aws_transfer_agreement`
- `aws_transfer_certificate`
- `aws_transfer_connector`
- `aws_transfer_profile`
- `aws_transfer_ssh_key`
- `aws_transfer_tag`
- `aws_transfer_workflow`

### cognitoidp — 7 missing

- `aws_cognito_identity_provider`
- `aws_cognito_managed_user_pool_client`
- `aws_cognito_resource_server`
- `aws_cognito_user_group`
- `aws_cognito_user_in_group`
- `aws_cognito_user_pool_domain`
- `aws_cognito_user_pool_ui_customization`

### directory — 7 missing

- `aws_directory_service_conditional_forwarder`
- `aws_directory_service_log_subscription`
- `aws_directory_service_radius_settings`
- `aws_directory_service_region`
- `aws_directory_service_shared_directory`
- `aws_directory_service_shared_directory_accepter`
- `aws_directory_service_trust`

### kafka — 7 missing

- `aws_msk_cluster_policy`
- `aws_msk_configuration`
- `aws_msk_replicator`
- `aws_msk_scram_secret_association`
- `aws_msk_serverless_cluster`
- `aws_msk_single_scram_secret_association`
- `aws_msk_vpc_connection`

### kms — 7 missing

- `aws_kms_ciphertext`
- `aws_kms_custom_key_store`
- `aws_kms_external_key`
- `aws_kms_grant`
- `aws_kms_key_policy`
- `aws_kms_replica_external_key`
- `aws_kms_replica_key`

### macie2 — 7 missing

- `aws_macie2_classification_export_configuration`
- `aws_macie2_custom_data_identifier`
- `aws_macie2_findings_filter`
- `aws_macie2_invitation_accepter`
- `aws_macie2_member`
- `aws_macie2_organization_admin_account`
- `aws_macie2_organization_configuration`

### shield — 7 missing

- `aws_shield_application_layer_automatic_response`
- `aws_shield_drt_access_log_bucket_association`
- `aws_shield_drt_access_role_arn_association`
- `aws_shield_proactive_engagement`
- `aws_shield_protection_group`
- `aws_shield_protection_health_check_association`
- `aws_shield_subscription`

### auditmanager — 6 missing

- `aws_auditmanager_account_registration`
- `aws_auditmanager_assessment`
- `aws_auditmanager_assessment_delegation`
- `aws_auditmanager_assessment_report`
- `aws_auditmanager_framework_share`
- `aws_auditmanager_organization_admin_account_registration`

### autoscaling — 6 missing

- `aws_autoscaling_attachment`
- `aws_autoscaling_group_tag`
- `aws_autoscaling_lifecycle_hook`
- `aws_autoscaling_notification`
- `aws_autoscaling_traffic_source_attachment`
- `aws_launch_template`

### bedrockagent — 6 missing

- `aws_bedrockagent_agent_action_group`
- `aws_bedrockagent_agent_alias`
- `aws_bedrockagent_agent_collaborator`
- `aws_bedrockagent_agent_knowledge_base_association`
- `aws_bedrockagent_data_source`
- `aws_bedrockagent_prompt`

### ebs — 6 missing

- `aws_ebs_default_kms_key`
- `aws_ebs_encryption_by_default`
- `aws_ebs_fast_snapshot_restore`
- `aws_ebs_snapshot_block_public_access`
- `aws_ebs_snapshot_copy`
- `aws_ebs_snapshot_import`

### eks — 6 missing

- `aws_eks_access_entry`
- `aws_eks_access_policy_association`
- `aws_eks_addon`
- `aws_eks_fargate_profile`
- `aws_eks_identity_provider_config`
- `aws_eks_pod_identity_association`

### elasticache — 6 missing

- `aws_elasticache_global_replication_group`
- `aws_elasticache_reserved_cache_node`
- `aws_elasticache_serverless_cache`
- `aws_elasticache_user`
- `aws_elasticache_user_group`
- `aws_elasticache_user_group_association`

### emr — 6 missing

- `aws_emr_block_public_access_configuration`
- `aws_emr_instance_fleet`
- `aws_emr_instance_group`
- `aws_emr_managed_scaling_policy`
- `aws_emr_studio`
- `aws_emr_studio_session_mapping`

### events — 6 missing

- `aws_cloudwatch_event_api_destination`
- `aws_cloudwatch_event_archive`
- `aws_cloudwatch_event_bus_policy`
- `aws_cloudwatch_event_connection`
- `aws_cloudwatch_event_endpoint`
- `aws_cloudwatch_event_permission`

### lakeformation — 6 missing

- `aws_lakeformation_data_cells_filter`
- `aws_lakeformation_lf_tag`
- `aws_lakeformation_opt_in`
- `aws_lakeformation_permissions`
- `aws_lakeformation_resource_lf_tag`
- `aws_lakeformation_resource_lf_tags`

### bedrock — 5 missing

- `aws_bedrock_custom_model`
- `aws_bedrock_guardrail_version`
- `aws_bedrock_inference_profile`
- `aws_bedrock_model_invocation_logging_configuration`
- `aws_bedrock_provisioned_model_throughput`

### codebuild — 5 missing

- `aws_codebuild_fleet`
- `aws_codebuild_report_group`
- `aws_codebuild_resource_policy`
- `aws_codebuild_source_credential`
- `aws_codebuild_webhook`

### dms — 5 missing

- `aws_dms_certificate`
- `aws_dms_event_subscription`
- `aws_dms_replication_config`
- `aws_dms_replication_subnet_group`
- `aws_dms_s3_endpoint`

### ecs — 5 missing

- `aws_ecs_account_setting_default`
- `aws_ecs_capacity_provider`
- `aws_ecs_cluster_capacity_providers`
- `aws_ecs_tag`
- `aws_ecs_task_set`

### efs — 5 missing

- `aws_efs_access_point`
- `aws_efs_backup_policy`
- `aws_efs_file_system_policy`
- `aws_efs_mount_target`
- `aws_efs_replication_configuration`

### kinesis — 5 missing

- `aws_kinesis_analytics_application`
- `aws_kinesis_firehose_delivery_stream`
- `aws_kinesis_resource_policy`
- `aws_kinesis_stream_consumer`
- `aws_kinesis_video_stream`

### lexmodelsv2 — 5 missing

- `aws_lexv2models_bot_locale`
- `aws_lexv2models_bot_version`
- `aws_lexv2models_intent`
- `aws_lexv2models_slot`
- `aws_lexv2models_slot_type`

### neptune — 5 missing

- `aws_neptune_cluster_endpoint`
- `aws_neptune_cluster_parameter_group`
- `aws_neptune_cluster_snapshot`
- `aws_neptune_event_subscription`
- `aws_neptune_global_cluster`

### acmpca — 4 missing

- `aws_acmpca_certificate`
- `aws_acmpca_certificate_authority_certificate`
- `aws_acmpca_permission`
- `aws_acmpca_policy`

### amp — 4 missing

- `aws_prometheus_alert_manager_definition`
- `aws_prometheus_rule_group_namespace`
- `aws_prometheus_scraper`
- `aws_prometheus_workspace_configuration`

### appconfig — 4 missing

- `aws_appconfig_deployment`
- `aws_appconfig_extension`
- `aws_appconfig_extension_association`
- `aws_appconfig_hosted_configuration_version`

### appfabric — 4 missing

- `aws_appfabric_app_authorization`
- `aws_appfabric_app_authorization_connection`
- `aws_appfabric_ingestion`
- `aws_appfabric_ingestion_destination`

### appmesh — 4 missing

- `aws_appmesh_gateway_route`
- `aws_appmesh_route`
- `aws_appmesh_virtual_gateway`
- `aws_appmesh_virtual_router`

### athena — 4 missing

- `aws_athena_capacity_reservation`
- `aws_athena_database`
- `aws_athena_named_query`
- `aws_athena_prepared_statement`

### cloudformation — 4 missing

- `aws_cloudformation_stack_instances`
- `aws_cloudformation_stack_set`
- `aws_cloudformation_stack_set_instance`
- `aws_cloudformation_type`

### inspector2 — 4 missing

- `aws_inspector2_delegated_admin_account`
- `aws_inspector2_filter`
- `aws_inspector2_member_association`
- `aws_inspector2_organization_configuration`

### memorydb — 4 missing

- `aws_memorydb_multi_region_cluster`
- `aws_memorydb_parameter_group`
- `aws_memorydb_snapshot`
- `aws_memorydb_user`

### opensearchserverless — 4 missing

- `aws_opensearchserverless_access_policy`
- `aws_opensearchserverless_lifecycle_policy`
- `aws_opensearchserverless_security_config`
- `aws_opensearchserverless_vpc_endpoint`

### organizations — 4 missing

- `aws_organizations_delegated_administrator`
- `aws_organizations_organization`
- `aws_organizations_policy_attachment`
- `aws_organizations_resource_policy`

### ram — 4 missing

- `aws_ram_principal_association`
- `aws_ram_resource_association`
- `aws_ram_resource_share_accepter`
- `aws_ram_sharing_with_organization`

### sns — 4 missing

- `aws_sns_platform_application`
- `aws_sns_sms_preferences`
- `aws_sns_topic_data_protection_policy`
- `aws_sns_topic_policy`

### wafv2 — 4 missing

- `aws_wafv2_api_key`
- `aws_wafv2_regex_pattern_set`
- `aws_wafv2_web_acl_association`
- `aws_wafv2_web_acl_logging_configuration`

### amplify — 3 missing

- `aws_amplify_backend_environment`
- `aws_amplify_domain_association`
- `aws_amplify_webhook`

### cloudwatch — 3 missing

- `aws_cloudwatch_composite_alarm`
- `aws_cloudwatch_dashboard`
- `aws_cloudwatch_metric_stream`

### codecommit — 3 missing

- `aws_codecommit_approval_rule_template`
- `aws_codecommit_approval_rule_template_association`
- `aws_codecommit_trigger`

### cognitoidentity — 3 missing

- `aws_cognito_identity_pool_provider_principal_tag`
- `aws_cognito_identity_pool_roles_attachment`
- `aws_cognito_identity_provider`

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

### servicediscovery — 3 missing

- `aws_service_discovery_http_namespace`
- `aws_service_discovery_instance`
- `aws_service_discovery_public_dns_namespace`

### sqs — 3 missing

- `aws_sqs_queue_policy`
- `aws_sqs_queue_redrive_allow_policy`
- `aws_sqs_queue_redrive_policy`

### account — 2 missing

- `aws_account_primary_contact`
- `aws_account_region`

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

### budgets — 1 missing

- `aws_budgets_budget_action`

### chatbot — 1 missing

- `aws_chatbot_teams_channel_configuration`

### cloudhsmv2 — 1 missing

- `aws_cloudhsm_v2_hsm`

### codedeploy — 1 missing

- `aws_codedeploy_deployment_config`

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

### kinesisanalyticsv2 — 1 missing

- `aws_kinesisanalyticsv2_application_snapshot`

### mediastore — 1 missing

- `aws_media_store_container_policy`

### resourcegroups — 1 missing

- `aws_resourcegroups_resource`

## Resources declared in specs but absent from the AWS provider schema

### s3control

- `aws_s3control_access_point` (typo or removed in schema)

### chatbot

- `aws_chatbot_microsoft_teams_channel_configuration` (typo or removed in schema)

### applicationcostprofiler

- `aws_applicationcostprofiler_report_definition` (typo or removed in schema)

### outposts

- `aws_outposts_outpost` (typo or removed in schema)
- `aws_outposts_site` (typo or removed in schema)

