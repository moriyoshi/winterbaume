# Terraform Converter Field Coverage Report

Generated: 2026-05-17

## Summary

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

## Statistics

- **Total converters:** 1143
- **Distinct resource types:** 1140
- **Overall inject coverage:** 6036/10765 (56.1%)
- **Overall extract coverage:** 5278/10765 (49.0%)

### Rating distribution

- **excellent:** 499
- **good:** 490
- **fair:** 45
- **poor:** 96
- **n/a:** 13

### Rating criteria

- **excellent:** inject >= 60% AND extract >= 50% of TF schema attributes
- **good:** inject >= 40% OR extract >= 30%
- **fair:** inject >= 20% OR extract >= 15%
- **poor:** below fair thresholds
- **n/a:** resource type not found in TF provider schema

## Per-Resource Detail

### `aws_accessanalyzer_analyzer`

**Source:** `crates/winterbaume-terraform/src/converters/accessanalyzer.rs`

**Inject attributes** (5): `analyzer_name`, `arn`, `region`, `tags`, `type`

**Extract attributes** (5): `analyzer_name`, `arn`, `id`, `tags`, `type`

**Missing from inject** (2): `configuration`, `tags_all`

**Missing from extract** (2): `configuration`, `tags_all`

### `aws_account_alternate_contact`

**Source:** `crates/winterbaume-terraform/src/converters/account.rs`

**Inject attributes** (6): `alternate_contact_type`, `email_address`, `name`, `phone_number`, `region`, `title`

**Extract attributes** (6): `alternate_contact_type`, `email_address`, `id`, `name`, `phone_number`, `title`

**Missing from inject** (2): `account_id`, `timeouts`

**Missing from extract** (2): `account_id`, `timeouts`

### `aws_acm_certificate`

**Source:** `crates/winterbaume-terraform/src/converters/acm.rs`

**Inject attributes** (15): `arn`, `certificate`, `certificate_chain`, `certificate_id`, `certificate_type`, `description`, `options`, `private_key`, `region`, `status`, `subject_alternative_names`, `tags`, `tags_all`, `usage`, `validation_option`

**Extract attributes** (20): `arn`, `certificate_transparency_logging_preference`, `certificate_type`, `domain_name`, `domain_validation_options`, `id`, `issuer`, `key_algorithm`, `not_after`, `not_before`, `options`, `renewal_eligibility`, `status`, `subject_alternative_names`, `tags`, `tags_all`, `validation_domain`, `validation_method`, `validation_option`, `validation_status`

**Missing from inject** (14): `certificate_authority_arn`, `certificate_body`, `domain_name`, `domain_validation_options`, `early_renewal_duration`, `key_algorithm`, `not_after`, `not_before`, `pending_renewal`, `renewal_eligibility`, `renewal_summary`, `type`, `validation_emails`, `validation_method`

**Missing from extract** (9): `certificate_authority_arn`, `certificate_body`, `certificate_chain`, `early_renewal_duration`, `pending_renewal`, `private_key`, `renewal_summary`, `type`, `validation_emails`

### `aws_acmpca_certificate`

**Source:** `crates/winterbaume-terraform/src/converters/acmpca.rs`

**Inject attributes** (10): `arn`, `certificate`, `certificate_chain`, `certificate_id`, `certificate_type`, `description`, `private_key`, `region`, `status`, `usage`

**Extract attributes** (0): (none)

**Missing from inject** (6): `api_passthrough`, `certificate_authority_arn`, `certificate_signing_request`, `signing_algorithm`, `template_arn`, `validity`

**Missing from extract** (9): `api_passthrough`, `arn`, `certificate`, `certificate_authority_arn`, `certificate_chain`, `certificate_signing_request`, `signing_algorithm`, `template_arn`, `validity`

### `aws_acmpca_certificate_authority`

**Source:** `crates/winterbaume-terraform/src/converters/acmpca.rs`

**Inject attributes** (10): `arn`, `certificate`, `certificate_authority_configuration`, `certificate_chain`, `key_storage_security_standard`, `policy`, `region`, `revocation_configuration`, `status`, `tags`

**Extract attributes** (25): `arn`, `certificate`, `certificate_authority_configuration`, `certificate_chain`, `common_name`, `country`, `created_at`, `crl_configuration`, `enabled`, `id`, `key_algorithm`, `key_storage_security_standard`, `locality`, `not_after`, `not_before`, `organization`, `organizational_unit`, `revocation_configuration`, `s3_object_acl`, `signing_algorithm`, `state`, `status`, `subject`, `tags`, `type`

**Missing from inject** (10): `certificate_signing_request`, `enabled`, `not_after`, `not_before`, `permanent_deletion_time_in_days`, `serial`, `tags_all`, `timeouts`, `type`, `usage_mode`

**Missing from extract** (6): `certificate_signing_request`, `permanent_deletion_time_in_days`, `serial`, `tags_all`, `timeouts`, `usage_mode`

### `aws_acmpca_certificate_authority_certificate`

**Source:** `crates/winterbaume-terraform/src/converters/acmpca.rs`

**Inject attributes** (4): `certificate`, `certificate_authority_arn`, `certificate_chain`, `region`

**Extract attributes** (0): (none)

**Missing from extract** (3): `certificate`, `certificate_authority_arn`, `certificate_chain`

### `aws_acmpca_permission`

**Source:** `crates/winterbaume-terraform/src/converters/acmpca.rs`

**Inject attributes** (4): `certificate_authority_arn`, `principal`, `region`, `source_account`

**Extract attributes** (0): (none)

**Missing from inject** (2): `actions`, `policy`

**Missing from extract** (5): `actions`, `certificate_authority_arn`, `policy`, `principal`, `source_account`

### `aws_acmpca_policy`

**Source:** `crates/winterbaume-terraform/src/converters/acmpca.rs`

**Inject attributes** (3): `policy`, `region`, `resource_arn`

**Extract attributes** (0): (none)

**Missing from extract** (2): `policy`, `resource_arn`

### `aws_prometheus_alert_manager_definition`

**Source:** `crates/winterbaume-terraform/src/converters/amp.rs`

**Inject attributes** (3): `definition`, `region`, `workspace_id`

**Extract attributes** (0): (none)

**Missing from extract** (2): `definition`, `workspace_id`

### `aws_prometheus_rule_group_namespace`

**Source:** `crates/winterbaume-terraform/src/converters/amp.rs`

**Inject attributes** (5): `arn`, `data`, `name`, `region`, `workspace_id`

**Extract attributes** (6): `arn`, `data`, `id`, `name`, `tags`, `workspace_id`

**Missing from inject** (2): `tags`, `tags_all`

**Missing from extract** (1): `tags_all`

### `aws_prometheus_scraper`

**Source:** `crates/winterbaume-terraform/src/converters/amp.rs`

**Inject attributes** (5): `alias`, `arn`, `region`, `role_arn`, `scraper_id`

**Extract attributes** (0): (none)

**Missing from inject** (7): `destination`, `role_configuration`, `scrape_configuration`, `source`, `tags`, `tags_all`, `timeouts`

**Missing from extract** (10): `alias`, `arn`, `destination`, `role_arn`, `role_configuration`, `scrape_configuration`, `source`, `tags`, `tags_all`, `timeouts`

### `aws_prometheus_workspace`

**Source:** `crates/winterbaume-terraform/src/converters/amp.rs`

**Inject attributes** (9): `bundle_id`, `computer_name`, `directory_id`, `ip_address`, `region`, `state`, `subnet_id`, `user_name`, `volume_encryption_key`

**Extract attributes** (6): `alias`, `arn`, `created_at`, `id`, `prometheus_endpoint`, `tags`

**Missing from inject** (7): `alias`, `arn`, `kms_key_arn`, `logging_configuration`, `prometheus_endpoint`, `tags`, `tags_all`

**Missing from extract** (3): `kms_key_arn`, `logging_configuration`, `tags_all`

### `aws_prometheus_workspace_configuration`

**Source:** `crates/winterbaume-terraform/src/converters/amp.rs`

**Inject attributes** (3): `region`, `retention_period_in_days`, `workspace_id`

**Extract attributes** (0): (none)

**Missing from inject** (2): `limits_per_label_set`, `timeouts`

**Missing from extract** (4): `limits_per_label_set`, `retention_period_in_days`, `timeouts`, `workspace_id`

### `aws_amplify_app`

**Source:** `crates/winterbaume-terraform/src/converters/amplify.rs`

**Inject attributes** (14): `app_id`, `arn`, `auto_branch_creation_config`, `build_spec`, `cache_config`, `custom_rule`, `custom_rules`, `default_domain`, `description`, `iam_service_role_arn`, `name`, `platform`, `region`, `repository`

**Extract attributes** (20): `app_id`, `arn`, `auto_branch_creation_config`, `build_spec`, `cache_config`, `create_time`, `custom_rule`, `default_domain`, `description`, `enable_auto_branch_creation`, `enable_basic_auth`, `enable_branch_auto_build`, `environment_variables`, `id`, `name`, `platform`, `production_branch`, `repository`, `tags_all`, `update_time`

**Missing from inject** (14): `access_token`, `auto_branch_creation_patterns`, `basic_auth_credentials`, `compute_role_arn`, `custom_headers`, `enable_auto_branch_creation`, `enable_basic_auth`, `enable_branch_auto_build`, `enable_branch_auto_deletion`, `environment_variables`, `oauth_token`, `production_branch`, `tags`, `tags_all`

**Missing from extract** (9): `access_token`, `auto_branch_creation_patterns`, `basic_auth_credentials`, `compute_role_arn`, `custom_headers`, `enable_branch_auto_deletion`, `iam_service_role_arn`, `oauth_token`, `tags`

### `aws_amplify_backend_environment`

**Source:** `crates/winterbaume-terraform/src/converters/amplify.rs`

**Inject attributes** (5): `app_id`, `arn`, `deployment_artifacts`, `environment_name`, `stack_name`

**Extract attributes** (0): (none)

**Missing from extract** (5): `app_id`, `arn`, `deployment_artifacts`, `environment_name`, `stack_name`

### `aws_amplify_branch`

**Source:** `crates/winterbaume-terraform/src/converters/amplify.rs`

**Inject attributes** (7): `app_id`, `arn`, `branch_name`, `description`, `framework`, `region`, `stage`

**Extract attributes** (15): `app_id`, `arn`, `branch_name`, `create_time`, `description`, `display_name`, `enable_auto_build`, `enable_basic_auth`, `environment_variables`, `framework`, `id`, `stage`, `tags_all`, `ttl`, `update_time`

**Missing from inject** (17): `associated_resources`, `backend_environment_arn`, `basic_auth_credentials`, `custom_domains`, `destination_branch`, `display_name`, `enable_auto_build`, `enable_basic_auth`, `enable_notification`, `enable_performance_mode`, `enable_pull_request_preview`, `environment_variables`, `pull_request_environment_name`, `source_branch`, `tags`, `tags_all`, `ttl`

**Missing from extract** (11): `associated_resources`, `backend_environment_arn`, `basic_auth_credentials`, `custom_domains`, `destination_branch`, `enable_notification`, `enable_performance_mode`, `enable_pull_request_preview`, `pull_request_environment_name`, `source_branch`, `tags`

### `aws_amplify_domain_association`

**Source:** `crates/winterbaume-terraform/src/converters/amplify.rs`

**Inject attributes** (6): `app_id`, `arn`, `domain_name`, `enable_auto_sub_domain`, `region`, `wait_for_verification`

**Extract attributes** (7): `app_id`, `arn`, `domain_name`, `domain_status`, `enable_auto_sub_domain`, `id`, `status_reason`

**Missing from inject** (3): `certificate_settings`, `certificate_verification_dns_record`, `sub_domain`

**Missing from extract** (4): `certificate_settings`, `certificate_verification_dns_record`, `sub_domain`, `wait_for_verification`

### `aws_amplify_webhook`

**Source:** `crates/winterbaume-terraform/src/converters/amplify.rs`

**Inject attributes** (5): `app_id`, `arn`, `branch_name`, `description`, `url`

**Extract attributes** (0): (none)

**Missing from extract** (5): `app_id`, `arn`, `branch_name`, `description`, `url`

### `aws_api_gateway_account`

**Source:** `crates/winterbaume-terraform/src/converters/apigateway.rs`

**Inject attributes** (5): `api_key_version`, `cloudwatch_role_arn`, `features`, `region`, `throttle_settings`

**Extract attributes** (7): `api_key_version`, `burst_limit`, `cloudwatch_role_arn`, `features`, `id`, `rate_limit`, `throttle_settings`

**Missing from inject** (1): `reset_on_delete`

**Missing from extract** (1): `reset_on_delete`

### `aws_api_gateway_api_key`

**Source:** `crates/winterbaume-terraform/src/converters/apigateway.rs`

**Inject attributes** (9): `api_id`, `description`, `enabled`, `expires`, `key_id`, `region`, `stage_key`, `tags`, `tags_all`

**Extract attributes** (8): `created_date`, `description`, `enabled`, `id`, `name`, `stage_key`, `tags`, `value`

**Missing from inject** (6): `arn`, `created_date`, `customer_id`, `last_updated_date`, `name`, `value`

**Missing from extract** (4): `arn`, `customer_id`, `last_updated_date`, `tags_all`

### `aws_api_gateway_authorizer`

**Source:** `crates/winterbaume-terraform/src/converters/apigateway.rs`

**Inject attributes** (10): `authorizer_credentials`, `authorizer_result_ttl_in_seconds`, `authorizer_uri`, `identity_source`, `identity_validation_expression`, `name`, `provider_arns`, `region`, `rest_api_id`, `type`

**Extract attributes** (10): `authorizer_credentials`, `authorizer_result_ttl_in_seconds`, `authorizer_uri`, `id`, `identity_source`, `identity_validation_expression`, `name`, `provider_arns`, `rest_api_id`, `type`

**Missing from inject** (1): `arn`

**Missing from extract** (1): `arn`

### `aws_api_gateway_base_path_mapping`

**Source:** `crates/winterbaume-terraform/src/converters/apigateway.rs`

**Inject attributes** (5): `api_id`, `base_path`, `domain_name`, `region`, `stage_name`

**Extract attributes** (5): `api_id`, `base_path`, `domain_name`, `id`, `stage_name`

**Missing from inject** (1): `domain_name_id`

**Missing from extract** (1): `domain_name_id`

### `aws_api_gateway_client_certificate`

**Source:** `crates/winterbaume-terraform/src/converters/apigateway.rs`

**Inject attributes** (5): `description`, `pem_encoded_certificate`, `region`, `tags`, `tags_all`

**Extract attributes** (6): `created_date`, `description`, `expiration_date`, `id`, `pem_encoded_certificate`, `tags`

**Missing from inject** (3): `arn`, `created_date`, `expiration_date`

**Missing from extract** (2): `arn`, `tags_all`

### `aws_api_gateway_deployment`

**Source:** `crates/winterbaume-terraform/src/converters/apigateway.rs`

**Inject attributes** (8): `canary_settings`, `description`, `region`, `rest_api_id`, `stage_description`, `stage_name`, `triggers`, `variables`

**Extract attributes** (7): `canary_settings`, `created_date`, `description`, `execution_arn`, `id`, `invoke_url`, `rest_api_id`

**Missing from inject** (3): `created_date`, `execution_arn`, `invoke_url`

**Missing from extract** (4): `stage_description`, `stage_name`, `triggers`, `variables`

### `aws_api_gateway_documentation_part`

**Source:** `crates/winterbaume-terraform/src/converters/apigateway.rs`

**Inject attributes** (4): `location`, `properties`, `region`, `rest_api_id`

**Extract attributes** (9): `id`, `location`, `method`, `name`, `path`, `properties`, `rest_api_id`, `status_code`, `type`

**Missing from inject** (1): `documentation_part_id`

**Missing from extract** (1): `documentation_part_id`

### `aws_api_gateway_documentation_version`

**Source:** `crates/winterbaume-terraform/src/converters/apigateway.rs`

**Inject attributes** (4): `description`, `region`, `rest_api_id`, `version`

**Extract attributes** (5): `created_date`, `description`, `id`, `rest_api_id`, `version`

### `aws_api_gateway_domain_name`

**Source:** `crates/winterbaume-terraform/src/converters/apigateway.rs`

**Inject attributes** (6): `certificate_name`, `cloudfront_domain_name`, `domain_name`, `region`, `tags`, `tags_all`

**Extract attributes** (5): `certificate_name`, `cloudfront_domain_name`, `domain_name`, `id`, `tags`

**Missing from inject** (17): `arn`, `certificate_arn`, `certificate_body`, `certificate_chain`, `certificate_private_key`, `certificate_upload_date`, `cloudfront_zone_id`, `domain_name_id`, `endpoint_configuration`, `mutual_tls_authentication`, `ownership_verification_certificate_arn`, `policy`, `regional_certificate_arn`, `regional_certificate_name`, `regional_domain_name`, `regional_zone_id`, `security_policy`

**Missing from extract** (18): `arn`, `certificate_arn`, `certificate_body`, `certificate_chain`, `certificate_private_key`, `certificate_upload_date`, `cloudfront_zone_id`, `domain_name_id`, `endpoint_configuration`, `mutual_tls_authentication`, `ownership_verification_certificate_arn`, `policy`, `regional_certificate_arn`, `regional_certificate_name`, `regional_domain_name`, `regional_zone_id`, `security_policy`, `tags_all`

### `aws_api_gateway_domain_name_access_association`

**Source:** `crates/winterbaume-terraform/src/converters/apigateway.rs`

**Inject attributes** (7): `access_association_source`, `access_association_source_type`, `arn`, `domain_name_arn`, `region`, `tags`, `tags_all`

**Extract attributes** (6): `access_association_source`, `access_association_source_type`, `arn`, `domain_name_arn`, `id`, `tags`

**Missing from extract** (1): `tags_all`

### `aws_api_gateway_gateway_response`

**Source:** `crates/winterbaume-terraform/src/converters/apigateway.rs`

**Inject attributes** (6): `region`, `response_parameters`, `response_templates`, `response_type`, `rest_api_id`, `status_code`

**Extract attributes** (6): `id`, `response_parameters`, `response_templates`, `response_type`, `rest_api_id`, `status_code`

### `aws_api_gateway_integration`

**Source:** `crates/winterbaume-terraform/src/converters/apigateway.rs`

**Inject attributes** (17): `cache_key_parameters`, `cache_namespace`, `connection_id`, `connection_type`, `content_handling`, `credentials`, `http_method`, `integration_http_method`, `passthrough_behavior`, `region`, `request_parameters`, `request_templates`, `resource_id`, `rest_api_id`, `timeout_milliseconds`, `type`, `uri`

**Extract attributes** (17): `cache_key_parameters`, `cache_namespace`, `connection_id`, `connection_type`, `content_handling`, `credentials`, `http_method`, `id`, `integration_http_method`, `passthrough_behavior`, `request_parameters`, `request_templates`, `resource_id`, `rest_api_id`, `timeout_milliseconds`, `type`, `uri`

**Missing from inject** (1): `tls_config`

**Missing from extract** (1): `tls_config`

### `aws_api_gateway_integration_response`

**Source:** `crates/winterbaume-terraform/src/converters/apigateway.rs`

**Inject attributes** (9): `content_handling`, `http_method`, `region`, `resource_id`, `response_parameters`, `response_templates`, `rest_api_id`, `selection_pattern`, `status_code`

**Extract attributes** (9): `content_handling`, `http_method`, `id`, `resource_id`, `response_parameters`, `response_templates`, `rest_api_id`, `selection_pattern`, `status_code`

### `aws_api_gateway_method`

**Source:** `crates/winterbaume-terraform/src/converters/apigateway.rs`

**Inject attributes** (11): `api_key_required`, `authorization`, `authorizer_id`, `http_method`, `operation_name`, `region`, `request_models`, `request_parameters`, `request_validator_id`, `resource_id`, `rest_api_id`

**Extract attributes** (11): `api_key_required`, `authorization`, `authorizer_id`, `http_method`, `id`, `operation_name`, `request_models`, `request_parameters`, `request_validator_id`, `resource_id`, `rest_api_id`

**Missing from inject** (1): `authorization_scopes`

**Missing from extract** (1): `authorization_scopes`

### `aws_api_gateway_method_response`

**Source:** `crates/winterbaume-terraform/src/converters/apigateway.rs`

**Inject attributes** (7): `http_method`, `region`, `resource_id`, `response_models`, `response_parameters`, `rest_api_id`, `status_code`

**Extract attributes** (7): `http_method`, `id`, `resource_id`, `response_models`, `response_parameters`, `rest_api_id`, `status_code`

### `aws_api_gateway_model`

**Source:** `crates/winterbaume-terraform/src/converters/apigateway.rs`

**Inject attributes** (6): `content_type`, `description`, `name`, `region`, `rest_api_id`, `schema`

**Extract attributes** (6): `content_type`, `description`, `id`, `name`, `rest_api_id`, `schema`

### `aws_api_gateway_request_validator`

**Source:** `crates/winterbaume-terraform/src/converters/apigateway.rs`

**Inject attributes** (5): `name`, `region`, `rest_api_id`, `validate_request_body`, `validate_request_parameters`

**Extract attributes** (5): `id`, `name`, `rest_api_id`, `validate_request_body`, `validate_request_parameters`

### `aws_api_gateway_resource`

**Source:** `crates/winterbaume-terraform/src/converters/apigateway.rs`

**Inject attributes** (5): `parent_id`, `path`, `path_part`, `region`, `rest_api_id`

**Extract attributes** (5): `id`, `parent_id`, `path`, `path_part`, `rest_api_id`

### `aws_api_gateway_rest_api`

**Source:** `crates/winterbaume-terraform/src/converters/apigateway.rs`

**Inject attributes** (13): `api_key_source`, `binary_media_types`, `description`, `disable_execute_api_endpoint`, `endpoint_configuration`, `minimum_compression_size`, `name`, `policy`, `region`, `root_resource_id`, `tags`, `tags_all`, `version`

**Extract attributes** (15): `api_key_source`, `binary_media_types`, `created_date`, `description`, `disable_execute_api_endpoint`, `endpoint_configuration`, `id`, `minimum_compression_size`, `name`, `policy`, `root_resource_id`, `tags`, `types`, `version`, `vpc_endpoint_ids`

**Missing from inject** (7): `arn`, `body`, `created_date`, `execution_arn`, `fail_on_warnings`, `parameters`, `put_rest_api_mode`

**Missing from extract** (7): `arn`, `body`, `execution_arn`, `fail_on_warnings`, `parameters`, `put_rest_api_mode`, `tags_all`

### `aws_api_gateway_rest_api_policy`

**Source:** `crates/winterbaume-terraform/src/converters/apigateway.rs`

**Inject attributes** (3): `policy`, `region`, `rest_api_id`

**Extract attributes** (3): `id`, `policy`, `rest_api_id`

### `aws_api_gateway_stage`

**Source:** `crates/winterbaume-terraform/src/converters/apigateway.rs`

**Inject attributes** (14): `access_log_settings`, `cache_cluster_enabled`, `cache_cluster_size`, `canary_settings`, `deployment_id`, `description`, `documentation_version`, `region`, `rest_api_id`, `stage_name`, `tags`, `tags_all`, `variables`, `xray_tracing_enabled`

**Extract attributes** (15): `access_log_settings`, `cache_cluster_enabled`, `cache_cluster_size`, `canary_settings`, `created_date`, `deployment_id`, `description`, `documentation_version`, `id`, `last_updated_date`, `rest_api_id`, `stage_name`, `tags`, `variables`, `xray_tracing_enabled`

**Missing from inject** (5): `arn`, `client_certificate_id`, `execution_arn`, `invoke_url`, `web_acl_arn`

**Missing from extract** (6): `arn`, `client_certificate_id`, `execution_arn`, `invoke_url`, `tags_all`, `web_acl_arn`

### `aws_api_gateway_usage_plan`

**Source:** `crates/winterbaume-terraform/src/converters/apigateway.rs`

**Inject attributes** (9): `api_stages`, `description`, `name`, `product_code`, `quota_settings`, `region`, `tags`, `tags_all`, `throttle_settings`

**Extract attributes** (15): `api_id`, `api_stages`, `burst_limit`, `description`, `id`, `limit`, `name`, `offset`, `period`, `product_code`, `quota_settings`, `rate_limit`, `stage`, `tags`, `throttle_settings`

**Missing from inject** (1): `arn`

**Missing from extract** (2): `arn`, `tags_all`

### `aws_api_gateway_usage_plan_key`

**Source:** `crates/winterbaume-terraform/src/converters/apigateway.rs`

**Inject attributes** (6): `key_id`, `key_type`, `name`, `region`, `usage_plan_id`, `value`

**Extract attributes** (6): `id`, `key_id`, `key_type`, `name`, `usage_plan_id`, `value`

### `aws_api_gateway_vpc_link`

**Source:** `crates/winterbaume-terraform/src/converters/apigateway.rs`

**Inject attributes** (8): `description`, `name`, `region`, `status`, `status_message`, `tags`, `tags_all`, `target_arns`

**Extract attributes** (7): `description`, `id`, `name`, `status`, `status_message`, `tags`, `target_arns`

**Missing from inject** (1): `arn`

**Missing from extract** (2): `arn`, `tags_all`

### `aws_apigatewayv2_api`

**Source:** `crates/winterbaume-terraform/src/converters/apigatewayv2.rs`

**Inject attributes** (8): `api_endpoint`, `api_id`, `description`, `name`, `protocol_type`, `region`, `route_selection_expression`, `tags`

**Extract attributes** (13): `api_endpoint`, `api_id`, `api_key_selection_expression`, `cors_configuration`, `created_date`, `description`, `execution_arn`, `id`, `name`, `protocol_type`, `route_selection_expression`, `tags`, `tags_all`

**Missing from inject** (13): `api_key_selection_expression`, `arn`, `body`, `cors_configuration`, `credentials_arn`, `disable_execute_api_endpoint`, `execution_arn`, `fail_on_warnings`, `ip_address_type`, `route_key`, `tags_all`, `target`, `version`

**Missing from extract** (9): `arn`, `body`, `credentials_arn`, `disable_execute_api_endpoint`, `fail_on_warnings`, `ip_address_type`, `route_key`, `target`, `version`

### `aws_apigatewayv2_api_mapping`

**Source:** `crates/winterbaume-terraform/src/converters/apigatewayv2.rs`

**Inject attributes** (6): `api_id`, `api_mapping_id`, `api_mapping_key`, `domain_name`, `region`, `stage`

**Extract attributes** (6): `api_id`, `api_mapping_id`, `api_mapping_key`, `domain_name`, `id`, `stage`

### `aws_apigatewayv2_authorizer`

**Source:** `crates/winterbaume-terraform/src/converters/apigatewayv2.rs`

**Inject attributes** (12): `api_id`, `authorizer_credentials_arn`, `authorizer_id`, `authorizer_payload_format_version`, `authorizer_result_ttl_in_seconds`, `authorizer_type`, `authorizer_uri`, `enable_simple_responses`, `identity_sources`, `identity_validation_expression`, `name`, `region`

**Extract attributes** (12): `api_id`, `authorizer_credentials_arn`, `authorizer_id`, `authorizer_payload_format_version`, `authorizer_result_ttl_in_seconds`, `authorizer_type`, `authorizer_uri`, `enable_simple_responses`, `id`, `identity_sources`, `identity_validation_expression`, `name`

**Missing from inject** (2): `jwt_configuration`, `timeouts`

**Missing from extract** (2): `jwt_configuration`, `timeouts`

### `aws_apigatewayv2_deployment`

**Source:** `crates/winterbaume-terraform/src/converters/apigatewayv2.rs`

**Inject attributes** (4): `api_id`, `deployment_id`, `description`, `region`

**Extract attributes** (6): `api_id`, `auto_deployed`, `deployment_id`, `deployment_status`, `description`, `id`

**Missing from inject** (2): `auto_deployed`, `triggers`

**Missing from extract** (1): `triggers`

### `aws_apigatewayv2_domain_name`

**Source:** `crates/winterbaume-terraform/src/converters/apigatewayv2.rs`

**Inject attributes** (4): `domain_name`, `domain_name_configuration`, `region`, `tags`

**Extract attributes** (8): `certificate_arn`, `domain_name`, `domain_name_configuration`, `endpoint_type`, `id`, `security_policy`, `tags`, `tags_all`

**Missing from inject** (5): `api_mapping_selection_expression`, `arn`, `mutual_tls_authentication`, `tags_all`, `timeouts`

**Missing from extract** (4): `api_mapping_selection_expression`, `arn`, `mutual_tls_authentication`, `timeouts`

### `aws_apigatewayv2_integration`

**Source:** `crates/winterbaume-terraform/src/converters/apigatewayv2.rs`

**Inject attributes** (9): `api_id`, `connection_type`, `description`, `integration_id`, `integration_method`, `integration_type`, `integration_uri`, `payload_format_version`, `region`

**Extract attributes** (9): `api_id`, `connection_type`, `description`, `id`, `integration_id`, `integration_method`, `integration_type`, `integration_uri`, `payload_format_version`

**Missing from inject** (12): `connection_id`, `content_handling_strategy`, `credentials_arn`, `integration_response_selection_expression`, `integration_subtype`, `passthrough_behavior`, `request_parameters`, `request_templates`, `response_parameters`, `template_selection_expression`, `timeout_milliseconds`, `tls_config`

**Missing from extract** (12): `connection_id`, `content_handling_strategy`, `credentials_arn`, `integration_response_selection_expression`, `integration_subtype`, `passthrough_behavior`, `request_parameters`, `request_templates`, `response_parameters`, `template_selection_expression`, `timeout_milliseconds`, `tls_config`

### `aws_apigatewayv2_integration_response`

**Source:** `crates/winterbaume-terraform/src/converters/apigatewayv2.rs`

**Inject attributes** (9): `api_id`, `content_handling_strategy`, `integration_id`, `integration_response_id`, `integration_response_key`, `region`, `response_parameters`, `response_templates`, `template_selection_expression`

**Extract attributes** (9): `api_id`, `content_handling_strategy`, `id`, `integration_id`, `integration_response_id`, `integration_response_key`, `response_parameters`, `response_templates`, `template_selection_expression`

### `aws_apigatewayv2_model`

**Source:** `crates/winterbaume-terraform/src/converters/apigatewayv2.rs`

**Inject attributes** (7): `api_id`, `content_type`, `description`, `model_id`, `name`, `region`, `schema`

**Extract attributes** (7): `api_id`, `content_type`, `description`, `id`, `model_id`, `name`, `schema`

### `aws_apigatewayv2_route`

**Source:** `crates/winterbaume-terraform/src/converters/apigatewayv2.rs`

**Inject attributes** (7): `api_id`, `authorization_type`, `authorizer_id`, `region`, `route_id`, `route_key`, `target`

**Extract attributes** (7): `api_id`, `authorization_type`, `authorizer_id`, `id`, `route_id`, `route_key`, `target`

**Missing from inject** (7): `api_key_required`, `authorization_scopes`, `model_selection_expression`, `operation_name`, `request_models`, `request_parameter`, `route_response_selection_expression`

**Missing from extract** (7): `api_key_required`, `authorization_scopes`, `model_selection_expression`, `operation_name`, `request_models`, `request_parameter`, `route_response_selection_expression`

### `aws_apigatewayv2_route_response`

**Source:** `crates/winterbaume-terraform/src/converters/apigatewayv2.rs`

**Inject attributes** (6): `api_id`, `model_selection_expression`, `region`, `route_id`, `route_response_id`, `route_response_key`

**Extract attributes** (6): `api_id`, `id`, `model_selection_expression`, `route_id`, `route_response_id`, `route_response_key`

**Missing from inject** (1): `response_models`

**Missing from extract** (1): `response_models`

### `aws_apigatewayv2_stage`

**Source:** `crates/winterbaume-terraform/src/converters/apigatewayv2.rs`

**Inject attributes** (7): `api_id`, `auto_deploy`, `deployment_id`, `description`, `name`, `region`, `tags`

**Extract attributes** (9): `api_id`, `auto_deploy`, `deployment_id`, `description`, `id`, `invoke_url`, `name`, `tags`, `tags_all`

**Missing from inject** (9): `access_log_settings`, `arn`, `client_certificate_id`, `default_route_settings`, `execution_arn`, `invoke_url`, `route_settings`, `stage_variables`, `tags_all`

**Missing from extract** (7): `access_log_settings`, `arn`, `client_certificate_id`, `default_route_settings`, `execution_arn`, `route_settings`, `stage_variables`

### `aws_apigatewayv2_vpc_link`

**Source:** `crates/winterbaume-terraform/src/converters/apigatewayv2.rs`

**Inject attributes** (6): `name`, `region`, `security_group_ids`, `subnet_ids`, `tags`, `vpc_link_id`

**Extract attributes** (7): `id`, `name`, `security_group_ids`, `subnet_ids`, `tags`, `tags_all`, `vpc_link_id`

**Missing from inject** (2): `arn`, `tags_all`

**Missing from extract** (1): `arn`

### `aws_appconfig_application`

**Source:** `crates/winterbaume-terraform/src/converters/appconfig.rs`

**Inject attributes** (7): `arn`, `date_created`, `date_updated`, `description`, `name`, `region`, `tags`

**Extract attributes** (4): `description`, `id`, `name`, `tags_all`

**Missing from inject** (1): `tags_all`

**Missing from extract** (2): `arn`, `tags`

### `aws_appconfig_configuration_profile`

**Source:** `crates/winterbaume-terraform/src/converters/appconfig.rs`

**Inject attributes** (8): `application_id`, `configuration_profile_id`, `description`, `location_uri`, `name`, `region`, `retrieval_role_arn`, `tags`

**Extract attributes** (8): `application_id`, `configuration_profile_id`, `description`, `id`, `location_uri`, `name`, `retrieval_role_arn`, `type`

**Missing from inject** (5): `arn`, `kms_key_identifier`, `tags_all`, `type`, `validator`

**Missing from extract** (5): `arn`, `kms_key_identifier`, `tags`, `tags_all`, `validator`

### `aws_appconfig_deployment`

**Source:** `crates/winterbaume-terraform/src/converters/appconfig.rs`

**Inject attributes** (10): `application_id`, `configuration_profile_id`, `configuration_version`, `deployment_number`, `deployment_strategy_id`, `description`, `environment_id`, `region`, `state`, `tags`

**Extract attributes** (9): `application_id`, `configuration_profile_id`, `configuration_version`, `deployment_number`, `deployment_strategy_id`, `description`, `environment_id`, `id`, `state`

**Missing from inject** (4): `arn`, `kms_key_arn`, `kms_key_identifier`, `tags_all`

**Missing from extract** (5): `arn`, `kms_key_arn`, `kms_key_identifier`, `tags`, `tags_all`

### `aws_appconfig_deployment_strategy`

**Source:** `crates/winterbaume-terraform/src/converters/appconfig.rs`

**Inject attributes** (7): `deployment_duration_in_minutes`, `description`, `final_bake_time_in_minutes`, `growth_type`, `name`, `region`, `replicate_to`

**Extract attributes** (8): `deployment_duration_in_minutes`, `description`, `final_bake_time_in_minutes`, `growth_factor`, `growth_type`, `id`, `name`, `replicate_to`

**Missing from inject** (4): `arn`, `growth_factor`, `tags`, `tags_all`

**Missing from extract** (3): `arn`, `tags`, `tags_all`

### `aws_appconfig_environment`

**Source:** `crates/winterbaume-terraform/src/converters/appconfig.rs`

**Inject attributes** (15): `application`, `arn`, `cname`, `date_created`, `date_updated`, `description`, `endpoint_url`, `name`, `platform_arn`, `region`, `solution_stack_name`, `tags`, `template_name`, `tier`, `version_label`

**Extract attributes** (9): `alarm_arn`, `alarm_role_arn`, `application_id`, `description`, `environment_id`, `id`, `monitor`, `name`, `state`

**Missing from inject** (5): `application_id`, `environment_id`, `monitor`, `state`, `tags_all`

**Missing from extract** (3): `arn`, `tags`, `tags_all`

### `aws_appconfig_extension`

**Source:** `crates/winterbaume-terraform/src/converters/appconfig.rs`

**Inject attributes** (6): `arn`, `description`, `name`, `region`, `tags`, `version`

**Extract attributes** (5): `arn`, `description`, `id`, `name`, `version`

**Missing from inject** (3): `action_point`, `parameter`, `tags_all`

**Missing from extract** (4): `action_point`, `parameter`, `tags`, `tags_all`

### `aws_appconfig_extension_association`

**Source:** `crates/winterbaume-terraform/src/converters/appconfig.rs`

**Inject attributes** (6): `arn`, `extension_arn`, `extension_version`, `region`, `resource_arn`, `tags`

**Extract attributes** (6): `arn`, `extension_arn`, `extension_version`, `id`, `parameters`, `resource_arn`

**Missing from inject** (1): `parameters`

### `aws_appconfig_hosted_configuration_version`

**Source:** `crates/winterbaume-terraform/src/converters/appconfig.rs`

**Inject attributes** (6): `application_id`, `configuration_profile_id`, `content_type`, `description`, `region`, `version_number`

**Extract attributes** (6): `application_id`, `configuration_profile_id`, `content_type`, `description`, `id`, `version_number`

**Missing from inject** (2): `arn`, `content`

**Missing from extract** (2): `arn`, `content`

### `aws_appfabric_app_authorization`

**Source:** `crates/winterbaume-terraform/src/converters/appfabric.rs`

**Inject attributes** (5): `app`, `app_bundle_arn`, `arn`, `auth_type`, `region`

**Extract attributes** (0): (none)

**Missing from inject** (9): `auth_url`, `created_at`, `credential`, `persona`, `tags`, `tags_all`, `tenant`, `timeouts`, `updated_at`

**Missing from extract** (13): `app`, `app_bundle_arn`, `arn`, `auth_type`, `auth_url`, `created_at`, `credential`, `persona`, `tags`, `tags_all`, `tenant`, `timeouts`, `updated_at`

### `aws_appfabric_app_authorization_connection`

**Source:** `crates/winterbaume-terraform/src/converters/appfabric.rs`

**Inject attributes** (3): `app_authorization_arn`, `app_bundle_arn`, `region`

**Extract attributes** (0): (none)

**Missing from inject** (4): `app`, `auth_request`, `tenant`, `timeouts`

**Missing from extract** (6): `app`, `app_authorization_arn`, `app_bundle_arn`, `auth_request`, `tenant`, `timeouts`

### `aws_appfabric_app_bundle`

**Source:** `crates/winterbaume-terraform/src/converters/appfabric.rs`

**Inject attributes** (4): `arn`, `customer_managed_key_arn`, `region`, `tags`

**Extract attributes** (4): `arn`, `customer_managed_key_arn`, `id`, `tags`

**Missing from inject** (1): `tags_all`

**Missing from extract** (1): `tags_all`

### `aws_appfabric_ingestion`

**Source:** `crates/winterbaume-terraform/src/converters/appfabric.rs`

**Inject attributes** (6): `app`, `app_bundle_arn`, `arn`, `ingestion_type`, `region`, `tenant_id`

**Extract attributes** (0): (none)

**Missing from inject** (2): `tags`, `tags_all`

**Missing from extract** (7): `app`, `app_bundle_arn`, `arn`, `ingestion_type`, `tags`, `tags_all`, `tenant_id`

### `aws_appfabric_ingestion_destination`

**Source:** `crates/winterbaume-terraform/src/converters/appfabric.rs`

**Inject attributes** (4): `app_bundle_arn`, `arn`, `ingestion_arn`, `region`

**Extract attributes** (0): (none)

**Missing from inject** (5): `destination_configuration`, `processing_configuration`, `tags`, `tags_all`, `timeouts`

**Missing from extract** (8): `app_bundle_arn`, `arn`, `destination_configuration`, `ingestion_arn`, `processing_configuration`, `tags`, `tags_all`, `timeouts`

### `aws_appflow_flow`

**Source:** `crates/winterbaume-terraform/src/converters/appflow.rs`

**Inject attributes** (11): `arn`, `description`, `destination_flow_config`, `kms_arn`, `metadata_catalog_config`, `name`, `region`, `source_flow_config`, `tags`, `task`, `trigger_config`

**Extract attributes** (10): `arn`, `description`, `destination_flow_config`, `id`, `kms_arn`, `name`, `source_flow_config`, `tags`, `task`, `trigger_config`

**Missing from inject** (2): `flow_status`, `tags_all`

**Missing from extract** (3): `flow_status`, `metadata_catalog_config`, `tags_all`

### `aws_appautoscaling_policy`

**Source:** `crates/winterbaume-terraform/src/converters/applicationautoscaling.rs`

**Inject attributes** (8): `adjustment_type`, `arn`, `autoscaling_group_name`, `name`, `policy_type`, `region`, `step_scaling_policy_configuration`, `target_tracking_scaling_policy_configuration`

**Extract attributes** (7): `arn`, `id`, `name`, `policy_type`, `resource_id`, `scalable_dimension`, `service_namespace`

**Missing from inject** (4): `alarm_arns`, `resource_id`, `scalable_dimension`, `service_namespace`

**Missing from extract** (3): `alarm_arns`, `step_scaling_policy_configuration`, `target_tracking_scaling_policy_configuration`

### `aws_appautoscaling_target`

**Source:** `crates/winterbaume-terraform/src/converters/applicationautoscaling.rs`

**Inject attributes** (9): `arn`, `max_capacity`, `min_capacity`, `region`, `resource_id`, `role_arn`, `scalable_dimension`, `service_namespace`, `suspended_state`

**Extract attributes** (12): `arn`, `dynamic_scaling_in_suspended`, `dynamic_scaling_out_suspended`, `id`, `max_capacity`, `min_capacity`, `resource_id`, `role_arn`, `scalable_dimension`, `scheduled_scaling_suspended`, `service_namespace`, `suspended_state`

**Missing from inject** (2): `tags`, `tags_all`

**Missing from extract** (2): `tags`, `tags_all`

### `aws_applicationcostprofiler_report_definition`

**Source:** `crates/winterbaume-terraform/src/converters/applicationcostprofiler.rs`

**Inject attributes** (5): `format`, `region`, `report_description`, `report_frequency`, `report_id`

**Extract attributes** (8): `bucket`, `destination_s3_location`, `format`, `id`, `prefix`, `report_description`, `report_frequency`, `report_id`

**Note:** Resource type not found in Terraform AWS provider schema.

### `aws_appmesh_gateway_route`

**Source:** `crates/winterbaume-terraform/src/converters/appmesh.rs`

**Inject attributes** (12): `arn`, `created_date`, `last_updated_date`, `mesh_name`, `mesh_owner`, `name`, `region`, `resource_owner`, `spec`, `tags`, `version`, `virtual_gateway_name`

**Extract attributes** (11): `arn`, `created_date`, `id`, `last_updated_date`, `mesh_name`, `mesh_owner`, `name`, `resource_owner`, `spec`, `tags`, `virtual_gateway_name`

**Missing from inject** (1): `tags_all`

**Missing from extract** (1): `tags_all`

### `aws_appmesh_mesh`

**Source:** `crates/winterbaume-terraform/src/converters/appmesh.rs`

**Inject attributes** (10): `arn`, `created_date`, `last_updated_date`, `mesh_owner`, `name`, `region`, `resource_owner`, `spec`, `tags`, `version`

**Extract attributes** (11): `arn`, `created_date`, `egress_filter`, `id`, `last_updated_date`, `mesh_owner`, `name`, `resource_owner`, `spec`, `tags`, `type`

**Missing from inject** (1): `tags_all`

**Missing from extract** (1): `tags_all`

### `aws_appmesh_route`

**Source:** `crates/winterbaume-terraform/src/converters/appmesh.rs`

**Inject attributes** (8): `destination_cidr_block`, `destination_ipv6_cidr_block`, `gateway_id`, `nat_gateway_id`, `region`, `route_table_id`, `spec`, `version`

**Extract attributes** (11): `arn`, `created_date`, `id`, `last_updated_date`, `mesh_name`, `mesh_owner`, `name`, `resource_owner`, `spec`, `tags`, `virtual_router_name`

**Missing from inject** (10): `arn`, `created_date`, `last_updated_date`, `mesh_name`, `mesh_owner`, `name`, `resource_owner`, `tags`, `tags_all`, `virtual_router_name`

**Missing from extract** (1): `tags_all`

### `aws_appmesh_virtual_gateway`

**Source:** `crates/winterbaume-terraform/src/converters/appmesh.rs`

**Inject attributes** (11): `arn`, `created_date`, `last_updated_date`, `mesh_name`, `mesh_owner`, `name`, `region`, `resource_owner`, `spec`, `tags`, `version`

**Extract attributes** (10): `arn`, `created_date`, `id`, `last_updated_date`, `mesh_name`, `mesh_owner`, `name`, `resource_owner`, `spec`, `tags`

**Missing from inject** (1): `tags_all`

**Missing from extract** (1): `tags_all`

### `aws_appmesh_virtual_node`

**Source:** `crates/winterbaume-terraform/src/converters/appmesh.rs`

**Inject attributes** (11): `arn`, `created_date`, `last_updated_date`, `mesh_name`, `mesh_owner`, `name`, `region`, `resource_owner`, `spec`, `tags`, `version`

**Extract attributes** (10): `arn`, `created_date`, `id`, `last_updated_date`, `mesh_name`, `mesh_owner`, `name`, `resource_owner`, `spec`, `tags`

**Missing from inject** (1): `tags_all`

**Missing from extract** (1): `tags_all`

### `aws_appmesh_virtual_router`

**Source:** `crates/winterbaume-terraform/src/converters/appmesh.rs`

**Inject attributes** (11): `arn`, `created_date`, `last_updated_date`, `mesh_name`, `mesh_owner`, `name`, `region`, `resource_owner`, `spec`, `tags`, `version`

**Extract attributes** (10): `arn`, `created_date`, `id`, `last_updated_date`, `mesh_name`, `mesh_owner`, `name`, `resource_owner`, `spec`, `tags`

**Missing from inject** (1): `tags_all`

**Missing from extract** (1): `tags_all`

### `aws_appmesh_virtual_service`

**Source:** `crates/winterbaume-terraform/src/converters/appmesh.rs`

**Inject attributes** (11): `arn`, `created_date`, `last_updated_date`, `mesh_name`, `mesh_owner`, `name`, `region`, `resource_owner`, `spec`, `tags`, `version`

**Extract attributes** (10): `arn`, `created_date`, `id`, `last_updated_date`, `mesh_name`, `mesh_owner`, `name`, `resource_owner`, `spec`, `tags`

**Missing from inject** (1): `tags_all`

**Missing from extract** (1): `tags_all`

### `aws_apprunner_auto_scaling_configuration_version`

**Source:** `crates/winterbaume-terraform/src/converters/apprunner.rs`

**Inject attributes** (10): `arn`, `auto_scaling_configuration_name`, `auto_scaling_configuration_revision`, `latest`, `max_concurrency`, `max_size`, `min_size`, `region`, `status`, `tags`

**Extract attributes** (11): `arn`, `auto_scaling_configuration_name`, `auto_scaling_configuration_revision`, `id`, `latest`, `max_concurrency`, `max_size`, `min_size`, `status`, `tags`, `tags_all`

**Missing from inject** (3): `has_associated_service`, `is_default`, `tags_all`

**Missing from extract** (2): `has_associated_service`, `is_default`

### `aws_apprunner_connection`

**Source:** `crates/winterbaume-terraform/src/converters/apprunner.rs`

**Inject attributes** (6): `arn`, `connection_name`, `provider_type`, `region`, `status`, `tags`

**Extract attributes** (7): `arn`, `connection_name`, `id`, `provider_type`, `status`, `tags`, `tags_all`

**Missing from inject** (1): `tags_all`

### `aws_apprunner_custom_domain_association`

**Source:** `crates/winterbaume-terraform/src/converters/apprunner.rs`

**Inject attributes** (5): `domain_name`, `enable_www_subdomain`, `region`, `service_arn`, `status`

**Extract attributes** (0): (none)

**Missing from inject** (2): `certificate_validation_records`, `dns_target`

**Missing from extract** (6): `certificate_validation_records`, `dns_target`, `domain_name`, `enable_www_subdomain`, `service_arn`, `status`

### `aws_apprunner_default_auto_scaling_configuration_version`

**Source:** `crates/winterbaume-terraform/src/converters/apprunner.rs`

**Inject attributes** (2): `auto_scaling_configuration_arn`, `region`

**Extract attributes** (2): `auto_scaling_configuration_arn`, `id`

### `aws_apprunner_deployment`

**Source:** `crates/winterbaume-terraform/src/converters/apprunner.rs`

**Inject attributes** (4): `operation_id`, `region`, `service_arn`, `status`

**Extract attributes** (0): (none)

**Missing from inject** (1): `timeouts`

**Missing from extract** (4): `operation_id`, `service_arn`, `status`, `timeouts`

### `aws_apprunner_observability_configuration`

**Source:** `crates/winterbaume-terraform/src/converters/apprunner.rs`

**Inject attributes** (7): `arn`, `latest`, `observability_configuration_name`, `observability_configuration_revision`, `region`, `status`, `tags`

**Extract attributes** (0): (none)

**Missing from inject** (2): `tags_all`, `trace_configuration`

**Missing from extract** (8): `arn`, `latest`, `observability_configuration_name`, `observability_configuration_revision`, `status`, `tags`, `tags_all`, `trace_configuration`

### `aws_apprunner_service`

**Source:** `crates/winterbaume-terraform/src/converters/apprunner.rs`

**Inject attributes** (13): `arn`, `encryption_configuration`, `health_check_configuration`, `instance_configuration`, `network_configuration`, `observability_configuration`, `region`, `service_id`, `service_name`, `service_url`, `source_configuration`, `status`, `tags`

**Extract attributes** (13): `arn`, `encryption_configuration`, `health_check_configuration`, `id`, `instance_configuration`, `network_configuration`, `observability_configuration`, `service_id`, `service_name`, `service_url`, `source_configuration`, `status`, `tags_all`

**Missing from inject** (2): `auto_scaling_configuration_arn`, `tags_all`

**Missing from extract** (2): `auto_scaling_configuration_arn`, `tags`

### `aws_apprunner_vpc_connector`

**Source:** `crates/winterbaume-terraform/src/converters/apprunner.rs`

**Inject attributes** (6): `arn`, `region`, `status`, `tags`, `vpc_connector_name`, `vpc_connector_revision`

**Extract attributes** (0): (none)

**Missing from inject** (3): `security_groups`, `subnets`, `tags_all`

**Missing from extract** (8): `arn`, `security_groups`, `status`, `subnets`, `tags`, `tags_all`, `vpc_connector_name`, `vpc_connector_revision`

### `aws_apprunner_vpc_ingress_connection`

**Source:** `crates/winterbaume-terraform/src/converters/apprunner.rs`

**Inject attributes** (7): `arn`, `domain_name`, `name`, `region`, `service_arn`, `status`, `tags`

**Extract attributes** (0): (none)

**Missing from inject** (2): `ingress_vpc_configuration`, `tags_all`

**Missing from extract** (8): `arn`, `domain_name`, `ingress_vpc_configuration`, `name`, `service_arn`, `status`, `tags`, `tags_all`

### `aws_appsync_api_cache`

**Source:** `crates/winterbaume-terraform/src/converters/appsync.rs`

**Inject attributes** (8): `api_caching_behavior`, `api_id`, `at_rest_encryption_enabled`, `health_metrics_config`, `region`, `transit_encryption_enabled`, `ttl`, `type`

**Extract attributes** (7): `api_caching_behavior`, `api_id`, `at_rest_encryption_enabled`, `health_metrics_config`, `transit_encryption_enabled`, `ttl`, `type`

### `aws_appsync_api_key`

**Source:** `crates/winterbaume-terraform/src/converters/appsync.rs`

**Inject attributes** (5): `api_id`, `description`, `expires`, `key_id`, `region`

**Extract attributes** (5): `api_id`, `description`, `expires`, `id`, `key_id`

**Missing from inject** (2): `api_key_id`, `key`

**Missing from extract** (2): `api_key_id`, `key`

### `aws_appsync_datasource`

**Source:** `crates/winterbaume-terraform/src/converters/appsync.rs`

**Inject attributes** (6): `api_id`, `arn`, `description`, `name`, `service_role_arn`, `type`

**Extract attributes** (0): (none)

**Missing from inject** (7): `dynamodb_config`, `elasticsearch_config`, `event_bridge_config`, `http_config`, `lambda_config`, `opensearchservice_config`, `relational_database_config`

**Missing from extract** (13): `api_id`, `arn`, `description`, `dynamodb_config`, `elasticsearch_config`, `event_bridge_config`, `http_config`, `lambda_config`, `name`, `opensearchservice_config`, `relational_database_config`, `service_role_arn`, `type`

### `aws_appsync_domain_name`

**Source:** `crates/winterbaume-terraform/src/converters/appsync.rs`

**Inject attributes** (5): `appsync_domain_name`, `certificate_arn`, `description`, `domain_name`, `hosted_zone_id`

**Extract attributes** (0): (none)

**Missing from extract** (5): `appsync_domain_name`, `certificate_arn`, `description`, `domain_name`, `hosted_zone_id`

### `aws_appsync_domain_name_api_association`

**Source:** `crates/winterbaume-terraform/src/converters/appsync.rs`

**Inject attributes** (2): `api_id`, `domain_name`

**Extract attributes** (0): (none)

**Missing from extract** (2): `api_id`, `domain_name`

### `aws_appsync_function`

**Source:** `crates/winterbaume-terraform/src/converters/appsync.rs`

**Inject attributes** (11): `api_id`, `arn`, `code`, `data_source`, `description`, `function_id`, `function_version`, `kind`, `name`, `request_mapping_template`, `response_mapping_template`

**Extract attributes** (0): (none)

**Missing from inject** (3): `max_batch_size`, `runtime`, `sync_config`

**Missing from extract** (13): `api_id`, `arn`, `code`, `data_source`, `description`, `function_id`, `function_version`, `max_batch_size`, `name`, `request_mapping_template`, `response_mapping_template`, `runtime`, `sync_config`

### `aws_appsync_graphql_api`

**Source:** `crates/winterbaume-terraform/src/converters/appsync.rs`

**Inject attributes** (10): `additional_authentication_provider`, `arn`, `authentication_type`, `enhanced_metrics_config`, `lambda_authorizer_config`, `name`, `region`, `tags`, `user_pool_config`, `xray_enabled`

**Extract attributes** (15): `additional_authentication_provider`, `arn`, `authentication_type`, `enhanced_metrics_config`, `id`, `lambda_authorizer_config`, `log_config`, `name`, `openid_connect_config`, `schema`, `tags`, `tags_all`, `uris`, `user_pool_config`, `xray_enabled`

**Missing from inject** (11): `api_type`, `introspection_config`, `log_config`, `merged_api_execution_role_arn`, `openid_connect_config`, `query_depth_limit`, `resolver_count_limit`, `schema`, `tags_all`, `uris`, `visibility`

**Missing from extract** (6): `api_type`, `introspection_config`, `merged_api_execution_role_arn`, `query_depth_limit`, `resolver_count_limit`, `visibility`

### `aws_appsync_resolver`

**Source:** `crates/winterbaume-terraform/src/converters/appsync.rs`

**Inject attributes** (9): `api_id`, `arn`, `code`, `data_source`, `field`, `kind`, `request_template`, `response_template`, `type`

**Extract attributes** (0): (none)

**Missing from inject** (5): `caching_config`, `max_batch_size`, `pipeline_config`, `runtime`, `sync_config`

**Missing from extract** (14): `api_id`, `arn`, `caching_config`, `code`, `data_source`, `field`, `kind`, `max_batch_size`, `pipeline_config`, `request_template`, `response_template`, `runtime`, `sync_config`, `type`

### `aws_appsync_source_api_association`

**Source:** `crates/winterbaume-terraform/src/converters/appsync.rs`

**Inject attributes** (5): `association_arn`, `association_id`, `description`, `merged_api_id`, `source_api_id`

**Extract attributes** (0): (none)

**Missing from inject** (5): `arn`, `merged_api_arn`, `source_api_arn`, `source_api_association_config`, `timeouts`

**Missing from extract** (9): `arn`, `association_id`, `description`, `merged_api_arn`, `merged_api_id`, `source_api_arn`, `source_api_association_config`, `source_api_id`, `timeouts`

### `aws_appsync_type`

**Source:** `crates/winterbaume-terraform/src/converters/appsync.rs`

**Inject attributes** (6): `api_id`, `arn`, `definition`, `format`, `name`, `region`

**Extract attributes** (6): `api_id`, `arn`, `definition`, `format`, `id`, `name`

**Missing from inject** (1): `description`

**Missing from extract** (1): `description`

### `aws_athena_capacity_reservation`

**Source:** `crates/winterbaume-terraform/src/converters/athena.rs`

**Inject attributes** (16): `arn`, `availability_zone`, `create_date`, `ebs_optimized`, `end_date`, `end_date_type`, `ephemeral_storage`, `instance_match_criteria`, `instance_platform`, `instance_type`, `outpost_arn`, `owner_id`, `placement_group_arn`, `region`, `start_date`, `tenancy`

**Extract attributes** (6): `allocated_dpus`, `id`, `name`, `status`, `tags`, `target_dpus`

**Missing from inject** (7): `allocated_dpus`, `name`, `status`, `tags`, `tags_all`, `target_dpus`, `timeouts`

**Missing from extract** (3): `arn`, `tags_all`, `timeouts`

### `aws_athena_data_catalog`

**Source:** `crates/winterbaume-terraform/src/converters/athena.rs`

**Inject attributes** (6): `description`, `name`, `parameters`, `region`, `tags`, `type`

**Extract attributes** (6): `description`, `id`, `name`, `parameters`, `tags`, `type`

**Missing from inject** (2): `arn`, `tags_all`

**Missing from extract** (2): `arn`, `tags_all`

### `aws_athena_database`

**Source:** `crates/winterbaume-terraform/src/converters/athena.rs`

**Inject attributes** (6): `arn`, `database_name`, `kms_key_id`, `region`, `table_count`, `tags`

**Extract attributes** (0): (none)

**Missing from inject** (8): `acl_configuration`, `bucket`, `comment`, `encryption_configuration`, `expected_bucket_owner`, `force_destroy`, `name`, `properties`

**Missing from extract** (8): `acl_configuration`, `bucket`, `comment`, `encryption_configuration`, `expected_bucket_owner`, `force_destroy`, `name`, `properties`

### `aws_athena_named_query`

**Source:** `crates/winterbaume-terraform/src/converters/athena.rs`

**Inject attributes** (6): `database`, `description`, `name`, `query`, `region`, `workgroup`

**Extract attributes** (6): `database`, `description`, `id`, `name`, `query`, `workgroup`

### `aws_athena_prepared_statement`

**Source:** `crates/winterbaume-terraform/src/converters/athena.rs`

**Inject attributes** (5): `description`, `name`, `query_statement`, `region`, `workgroup`

**Extract attributes** (5): `description`, `id`, `name`, `query_statement`, `workgroup`

**Missing from inject** (1): `timeouts`

**Missing from extract** (1): `timeouts`

### `aws_athena_workgroup`

**Source:** `crates/winterbaume-terraform/src/converters/athena.rs`

**Inject attributes** (6): `configuration`, `description`, `name`, `region`, `tags`, `tags_all`

**Extract attributes** (6): `description`, `id`, `name`, `state`, `tags`, `tags_all`

**Missing from inject** (3): `arn`, `force_destroy`, `state`

**Missing from extract** (3): `arn`, `configuration`, `force_destroy`

### `aws_auditmanager_account_registration`

**Source:** `crates/winterbaume-terraform/src/converters/auditmanager.rs`

**Inject attributes** (4): `delegated_admin_account`, `deregister_on_destroy`, `kms_key`, `region`

**Extract attributes** (2): `id`, `status`

**Missing from inject** (1): `status`

**Missing from extract** (3): `delegated_admin_account`, `deregister_on_destroy`, `kms_key`

### `aws_auditmanager_assessment`

**Source:** `crates/winterbaume-terraform/src/converters/auditmanager.rs`

**Inject attributes** (5): `description`, `framework_id`, `name`, `region`, `tags`

**Extract attributes** (8): `arn`, `description`, `framework_id`, `id`, `name`, `status`, `tags`, `tags_all`

**Missing from inject** (7): `arn`, `assessment_reports_destination`, `roles`, `roles_all`, `scope`, `status`, `tags_all`

**Missing from extract** (4): `assessment_reports_destination`, `roles`, `roles_all`, `scope`

### `aws_auditmanager_assessment_delegation`

**Source:** `crates/winterbaume-terraform/src/converters/auditmanager.rs`

**Inject attributes** (6): `assessment_id`, `comment`, `control_set_id`, `region`, `role_arn`, `role_type`

**Extract attributes** (0): (none)

**Missing from inject** (2): `delegation_id`, `status`

**Missing from extract** (7): `assessment_id`, `comment`, `control_set_id`, `delegation_id`, `role_arn`, `role_type`, `status`

### `aws_auditmanager_assessment_report`

**Source:** `crates/winterbaume-terraform/src/converters/auditmanager.rs`

**Inject attributes** (4): `assessment_id`, `description`, `name`, `region`

**Extract attributes** (0): (none)

**Missing from inject** (2): `author`, `status`

**Missing from extract** (5): `assessment_id`, `author`, `description`, `name`, `status`

### `aws_auditmanager_control`

**Source:** `crates/winterbaume-terraform/src/converters/auditmanager.rs`

**Inject attributes** (5): `control_mapping_sources`, `description`, `name`, `region`, `tags`

**Extract attributes** (7): `control_mapping_sources`, `description`, `id`, `name`, `tags`, `tags_all`, `type`

**Missing from inject** (6): `action_plan_instructions`, `action_plan_title`, `arn`, `tags_all`, `testing_information`, `type`

**Missing from extract** (4): `action_plan_instructions`, `action_plan_title`, `arn`, `testing_information`

### `aws_auditmanager_framework`

**Source:** `crates/winterbaume-terraform/src/converters/auditmanager.rs`

**Inject attributes** (6): `compliance_type`, `control_sets`, `description`, `name`, `region`, `tags`

**Extract attributes** (6): `compliance_type`, `control_sets`, `description`, `id`, `name`, `tags`

**Missing from inject** (3): `arn`, `framework_type`, `tags_all`

**Missing from extract** (3): `arn`, `framework_type`, `tags_all`

### `aws_auditmanager_framework_share`

**Source:** `crates/winterbaume-terraform/src/converters/auditmanager.rs`

**Inject attributes** (5): `comment`, `destination_account`, `destination_region`, `framework_id`, `region`

**Extract attributes** (0): (none)

**Missing from inject** (1): `status`

**Missing from extract** (5): `comment`, `destination_account`, `destination_region`, `framework_id`, `status`

### `aws_auditmanager_organization_admin_account_registration`

**Source:** `crates/winterbaume-terraform/src/converters/auditmanager.rs`

**Inject attributes** (2): `admin_account_id`, `region`

**Extract attributes** (0): (none)

**Missing from inject** (1): `organization_id`

**Missing from extract** (2): `admin_account_id`, `organization_id`

### `aws_autoscaling_attachment`

**Source:** `crates/winterbaume-terraform/src/converters/autoscaling.rs`

**Inject attributes** (4): `autoscaling_group_name`, `elb`, `lb_target_group_arn`, `region`

**Extract attributes** (0): (none)

**Missing from extract** (3): `autoscaling_group_name`, `elb`, `lb_target_group_arn`

### `aws_autoscaling_group`

**Source:** `crates/winterbaume-terraform/src/converters/autoscaling.rs`

**Inject attributes** (13): `arn`, `default_cooldown`, `desired_capacity`, `health_check_grace_period`, `health_check_type`, `launch_configuration`, `max_size`, `min_size`, `name`, `region`, `tags`, `tags_all`, `vpc_zone_identifier`

**Extract attributes** (14): `arn`, `availability_zones`, `default_cooldown`, `desired_capacity`, `health_check_grace_period`, `health_check_type`, `id`, `launch_configuration`, `max_size`, `min_size`, `name`, `tags`, `termination_policies`, `vpc_zone_identifier`

**Missing from inject** (35): `availability_zone_distribution`, `availability_zones`, `capacity_rebalance`, `capacity_reservation_specification`, `context`, `default_instance_warmup`, `desired_capacity_type`, `enabled_metrics`, `force_delete`, `force_delete_warm_pool`, `ignore_failed_scaling_activities`, `initial_lifecycle_hook`, `instance_maintenance_policy`, `instance_refresh`, `launch_template`, `load_balancers`, `max_instance_lifetime`, `metrics_granularity`, `min_elb_capacity`, `mixed_instances_policy`, `name_prefix`, `placement_group`, `predicted_capacity`, `protect_from_scale_in`, `service_linked_role_arn`, `suspended_processes`, `tag`, `target_group_arns`, `termination_policies`, `timeouts`, `traffic_source`, `wait_for_capacity_timeout`, `wait_for_elb_capacity`, `warm_pool`, `warm_pool_size`

**Missing from extract** (33): `availability_zone_distribution`, `capacity_rebalance`, `capacity_reservation_specification`, `context`, `default_instance_warmup`, `desired_capacity_type`, `enabled_metrics`, `force_delete`, `force_delete_warm_pool`, `ignore_failed_scaling_activities`, `initial_lifecycle_hook`, `instance_maintenance_policy`, `instance_refresh`, `launch_template`, `load_balancers`, `max_instance_lifetime`, `metrics_granularity`, `min_elb_capacity`, `mixed_instances_policy`, `name_prefix`, `placement_group`, `predicted_capacity`, `protect_from_scale_in`, `service_linked_role_arn`, `suspended_processes`, `tag`, `target_group_arns`, `timeouts`, `traffic_source`, `wait_for_capacity_timeout`, `wait_for_elb_capacity`, `warm_pool`, `warm_pool_size`

### `aws_autoscaling_group_tag`

**Source:** `crates/winterbaume-terraform/src/converters/autoscaling.rs`

**Inject attributes** (2): `autoscaling_group_name`, `region`

**Extract attributes** (0): (none)

**Missing from inject** (1): `tag`

**Missing from extract** (2): `autoscaling_group_name`, `tag`

### `aws_autoscaling_lifecycle_hook`

**Source:** `crates/winterbaume-terraform/src/converters/autoscaling.rs`

**Inject attributes** (9): `autoscaling_group_name`, `default_result`, `heartbeat_timeout`, `lifecycle_transition`, `name`, `notification_metadata`, `notification_target_arn`, `region`, `role_arn`

**Extract attributes** (9): `autoscaling_group_name`, `default_result`, `heartbeat_timeout`, `id`, `lifecycle_transition`, `name`, `notification_metadata`, `notification_target_arn`, `role_arn`

### `aws_autoscaling_notification`

**Source:** `crates/winterbaume-terraform/src/converters/autoscaling.rs`

**Inject attributes** (2): `region`, `topic_arn`

**Extract attributes** (0): (none)

**Missing from inject** (2): `group_names`, `notifications`

**Missing from extract** (3): `group_names`, `notifications`, `topic_arn`

### `aws_autoscaling_policy`

**Source:** `crates/winterbaume-terraform/src/converters/autoscaling.rs`

**Inject attributes** (9): `adjustment_type`, `arn`, `autoscaling_group_name`, `cooldown`, `min_adjustment_magnitude`, `name`, `policy_type`, `region`, `scaling_adjustment`

**Extract attributes** (8): `adjustment_type`, `arn`, `autoscaling_group_name`, `cooldown`, `id`, `name`, `policy_type`, `scaling_adjustment`

**Missing from inject** (6): `enabled`, `estimated_instance_warmup`, `metric_aggregation_type`, `predictive_scaling_configuration`, `step_adjustment`, `target_tracking_configuration`

**Missing from extract** (7): `enabled`, `estimated_instance_warmup`, `metric_aggregation_type`, `min_adjustment_magnitude`, `predictive_scaling_configuration`, `step_adjustment`, `target_tracking_configuration`

### `aws_autoscaling_schedule`

**Source:** `crates/winterbaume-terraform/src/converters/autoscaling.rs`

**Inject attributes** (11): `arn`, `autoscaling_group_name`, `desired_capacity`, `end_time`, `max_size`, `min_size`, `recurrence`, `region`, `scheduled_action_name`, `start_time`, `time_zone`

**Extract attributes** (10): `arn`, `autoscaling_group_name`, `desired_capacity`, `end_time`, `id`, `max_size`, `min_size`, `recurrence`, `scheduled_action_name`, `start_time`

**Missing from extract** (1): `time_zone`

### `aws_autoscaling_traffic_source_attachment`

**Source:** `crates/winterbaume-terraform/src/converters/autoscaling.rs`

**Inject attributes** (3): `autoscaling_group_name`, `region`, `traffic_source`

**Extract attributes** (0): (none)

**Missing from inject** (1): `timeouts`

**Missing from extract** (3): `autoscaling_group_name`, `timeouts`, `traffic_source`

### `aws_launch_configuration`

**Source:** `crates/winterbaume-terraform/src/converters/autoscaling.rs`

**Inject attributes** (11): `arn`, `associate_public_ip_address`, `ebs_optimized`, `iam_instance_profile`, `image_id`, `instance_type`, `key_name`, `name`, `region`, `spot_price`, `user_data`

**Extract attributes** (11): `arn`, `associate_public_ip_address`, `ebs_optimized`, `iam_instance_profile`, `id`, `image_id`, `instance_type`, `key_name`, `name`, `security_groups`, `spot_price`

**Missing from inject** (9): `ebs_block_device`, `enable_monitoring`, `ephemeral_block_device`, `metadata_options`, `name_prefix`, `placement_tenancy`, `root_block_device`, `security_groups`, `user_data_base64`

**Missing from extract** (9): `ebs_block_device`, `enable_monitoring`, `ephemeral_block_device`, `metadata_options`, `name_prefix`, `placement_tenancy`, `root_block_device`, `user_data`, `user_data_base64`

### `aws_backup_framework`

**Source:** `crates/winterbaume-terraform/src/converters/backup.rs`

**Inject attributes** (6): `compliance_type`, `control`, `description`, `name`, `region`, `tags`

**Extract attributes** (10): `arn`, `control`, `creation_time`, `deployment_status`, `description`, `id`, `name`, `status`, `tags`, `tags_all`

**Missing from inject** (6): `arn`, `creation_time`, `deployment_status`, `status`, `tags_all`, `timeouts`

**Missing from extract** (1): `timeouts`

### `aws_backup_global_settings`

**Source:** `crates/winterbaume-terraform/src/converters/backup.rs`

**Inject attributes** (1): `region`

**Extract attributes** (0): (none)

**Missing from inject** (1): `global_settings`

**Missing from extract** (1): `global_settings`

### `aws_backup_logically_air_gapped_vault`

**Source:** `crates/winterbaume-terraform/src/converters/backup.rs`

**Inject attributes** (6): `arn`, `max_retention_days`, `min_retention_days`, `name`, `region`, `tags`

**Extract attributes** (0): (none)

**Missing from inject** (2): `tags_all`, `timeouts`

**Missing from extract** (7): `arn`, `max_retention_days`, `min_retention_days`, `name`, `tags`, `tags_all`, `timeouts`

### `aws_backup_plan`

**Source:** `crates/winterbaume-terraform/src/converters/backup.rs`

**Inject attributes** (7): `advanced_backup_setting`, `arn`, `name`, `region`, `rule`, `tags`, `tags_all`

**Extract attributes** (7): `advanced_backup_setting`, `arn`, `id`, `name`, `rule`, `tags`, `version`

**Missing from inject** (1): `version`

**Missing from extract** (1): `tags_all`

### `aws_backup_region_settings`

**Source:** `crates/winterbaume-terraform/src/converters/backup.rs`

**Inject attributes** (1): `region`

**Extract attributes** (0): (none)

**Missing from inject** (2): `resource_type_management_preference`, `resource_type_opt_in_preference`

**Missing from extract** (2): `resource_type_management_preference`, `resource_type_opt_in_preference`

### `aws_backup_report_plan`

**Source:** `crates/winterbaume-terraform/src/converters/backup.rs`

**Inject attributes** (8): `arn`, `deployment_status`, `description`, `name`, `region`, `report_delivery_channel`, `report_setting`, `tags`

**Extract attributes** (10): `arn`, `creation_time`, `deployment_status`, `description`, `id`, `name`, `report_delivery_channel`, `report_setting`, `tags`, `tags_all`

**Missing from inject** (2): `creation_time`, `tags_all`

### `aws_backup_restore_testing_plan`

**Source:** `crates/winterbaume-terraform/src/converters/backup.rs`

**Inject attributes** (8): `arn`, `name`, `recovery_point_selection`, `region`, `schedule_expression`, `schedule_expression_timezone`, `start_window_hours`, `tags`

**Extract attributes** (9): `arn`, `id`, `name`, `recovery_point_selection`, `schedule_expression`, `schedule_expression_timezone`, `start_window_hours`, `tags`, `tags_all`

**Missing from inject** (1): `tags_all`

### `aws_backup_restore_testing_selection`

**Source:** `crates/winterbaume-terraform/src/converters/backup.rs`

**Inject attributes** (9): `iam_role_arn`, `name`, `protected_resource_arns`, `protected_resource_conditions`, `protected_resource_type`, `region`, `restore_metadata_overrides`, `restore_testing_plan_name`, `validation_window_hours`

**Extract attributes** (9): `iam_role_arn`, `id`, `name`, `protected_resource_arns`, `protected_resource_conditions`, `protected_resource_type`, `restore_metadata_overrides`, `restore_testing_plan_name`, `validation_window_hours`

### `aws_backup_selection`

**Source:** `crates/winterbaume-terraform/src/converters/backup.rs`

**Inject attributes** (4): `iam_role_arn`, `name`, `plan_id`, `region`

**Extract attributes** (0): (none)

**Missing from inject** (4): `condition`, `not_resources`, `resources`, `selection_tag`

**Missing from extract** (7): `condition`, `iam_role_arn`, `name`, `not_resources`, `plan_id`, `resources`, `selection_tag`

### `aws_backup_vault`

**Source:** `crates/winterbaume-terraform/src/converters/backup.rs`

**Inject attributes** (4): `arn`, `name`, `region`, `tags`

**Extract attributes** (5): `arn`, `id`, `name`, `tags`, `tags_all`

**Missing from inject** (5): `force_destroy`, `kms_key_arn`, `recovery_points`, `tags_all`, `timeouts`

**Missing from extract** (4): `force_destroy`, `kms_key_arn`, `recovery_points`, `timeouts`

### `aws_backup_vault_lock_configuration`

**Source:** `crates/winterbaume-terraform/src/converters/backup.rs`

**Inject attributes** (5): `backup_vault_arn`, `backup_vault_name`, `max_retention_days`, `min_retention_days`, `region`

**Extract attributes** (0): (none)

**Missing from inject** (1): `changeable_for_days`

**Missing from extract** (5): `backup_vault_arn`, `backup_vault_name`, `changeable_for_days`, `max_retention_days`, `min_retention_days`

### `aws_backup_vault_notifications`

**Source:** `crates/winterbaume-terraform/src/converters/backup.rs`

**Inject attributes** (5): `backup_vault_arn`, `backup_vault_events`, `backup_vault_name`, `region`, `sns_topic_arn`

**Extract attributes** (5): `backup_vault_arn`, `backup_vault_events`, `backup_vault_name`, `id`, `sns_topic_arn`

### `aws_backup_vault_policy`

**Source:** `crates/winterbaume-terraform/src/converters/backup.rs`

**Inject attributes** (4): `backup_vault_arn`, `backup_vault_name`, `policy`, `region`

**Extract attributes** (4): `backup_vault_arn`, `backup_vault_name`, `id`, `policy`

### `aws_batch_compute_environment`

**Source:** `crates/winterbaume-terraform/src/converters/batch.rs`

**Inject attributes** (11): `arn`, `compute_resources`, `eks_configuration`, `name`, `region`, `service_role`, `state`, `tags`, `tags_all`, `type`, `update_policy`

**Extract attributes** (12): `arn`, `compute_resources`, `eks_configuration`, `id`, `name`, `service_role`, `state`, `status`, `tags`, `tags_all`, `type`, `update_policy`

**Missing from inject** (5): `compute_environment_name`, `compute_environment_name_prefix`, `ecs_cluster_arn`, `status`, `status_reason`

**Missing from extract** (4): `compute_environment_name`, `compute_environment_name_prefix`, `ecs_cluster_arn`, `status_reason`

### `aws_batch_job_definition`

**Source:** `crates/winterbaume-terraform/src/converters/batch.rs`

**Inject attributes** (13): `arn`, `container_properties`, `eks_properties`, `name`, `propagate_tags`, `region`, `retry_strategy`, `revision`, `scheduling_priority`, `tags`, `tags_all`, `timeout`, `type`

**Extract attributes** (18): `arn`, `arn_prefix`, `command`, `container_properties`, `eks_properties`, `id`, `image`, `name`, `resourceRequirements`, `retry_strategy`, `revision`, `status`, `tags`, `tags_all`, `tags_propagated`, `timeout`, `type`, `value`

**Missing from inject** (6): `arn_prefix`, `deregister_on_new_revision`, `ecs_properties`, `node_properties`, `parameters`, `platform_capabilities`

**Missing from extract** (7): `deregister_on_new_revision`, `ecs_properties`, `node_properties`, `parameters`, `platform_capabilities`, `propagate_tags`, `scheduling_priority`

### `aws_batch_job_queue`

**Source:** `crates/winterbaume-terraform/src/converters/batch.rs`

**Inject attributes** (11): `arn`, `compute_environment_order`, `compute_environments`, `job_state_time_limit_action`, `name`, `priority`, `region`, `scheduling_policy_arn`, `state`, `tags`, `tags_all`

**Extract attributes** (11): `arn`, `compute_environment`, `compute_environment_order`, `id`, `job_state_time_limit_action`, `name`, `order`, `priority`, `scheduling_policy_arn`, `state`, `tags`

**Missing from inject** (1): `timeouts`

**Missing from extract** (3): `compute_environments`, `tags_all`, `timeouts`

### `aws_batch_scheduling_policy`

**Source:** `crates/winterbaume-terraform/src/converters/batch.rs`

**Inject attributes** (6): `arn`, `fair_share_policy`, `name`, `region`, `tags`, `tags_all`

**Extract attributes** (10): `arn`, `compute_reservation`, `fair_share_policy`, `id`, `name`, `share_decay_seconds`, `share_distribution`, `share_identifier`, `tags`, `weight_factor`

**Missing from extract** (1): `tags_all`

### `aws_bedrock_custom_model`

**Source:** `crates/winterbaume-terraform/src/converters/bedrock.rs`

**Inject attributes** (12): `base_model_identifier`, `custom_model_arn`, `custom_model_kms_key_id`, `custom_model_name`, `customization_type`, `hyperparameters`, `job_arn`, `job_name`, `job_status`, `region`, `role_arn`, `tags`

**Extract attributes** (14): `base_model_identifier`, `custom_model_arn`, `custom_model_name`, `customization_type`, `hyperparameters`, `id`, `job_arn`, `job_name`, `job_status`, `output_data_config`, `role_arn`, `s3_uri`, `tags_all`, `training_data_config`

**Missing from inject** (8): `output_data_config`, `tags_all`, `timeouts`, `training_data_config`, `training_metrics`, `validation_data_config`, `validation_metrics`, `vpc_config`

**Missing from extract** (7): `custom_model_kms_key_id`, `tags`, `timeouts`, `training_metrics`, `validation_data_config`, `validation_metrics`, `vpc_config`

### `aws_bedrock_guardrail`

**Source:** `crates/winterbaume-terraform/src/converters/bedrock.rs`

**Inject attributes** (18): `blocked_input_messaging`, `blocked_outputs_messaging`, `content_policy_config`, `contextual_grounding_policy_config`, `created_at`, `description`, `guardrail_arn`, `guardrail_id`, `name`, `region`, `sensitive_information_policy_config`, `status`, `tags`, `tags_all`, `topic_policy_config`, `updated_at`, `version`, `word_policy_config`

**Extract attributes** (18): `blocked_input_messaging`, `blocked_outputs_messaging`, `content_policy_config`, `contextual_grounding_policy_config`, `created_at`, `description`, `guardrail_arn`, `guardrail_id`, `id`, `kms_key_arn`, `name`, `sensitive_information_policy_config`, `status`, `tags_all`, `topic_policy_config`, `updated_at`, `version`, `word_policy_config`

**Missing from inject** (2): `kms_key_arn`, `timeouts`

**Missing from extract** (2): `tags`, `timeouts`

### `aws_bedrock_guardrail_version`

**Source:** `crates/winterbaume-terraform/src/converters/bedrock.rs`

**Inject attributes** (5): `description`, `guardrail_arn`, `region`, `skip_destroy`, `version`

**Extract attributes** (5): `description`, `guardrail_arn`, `id`, `skip_destroy`, `version`

**Missing from inject** (1): `timeouts`

**Missing from extract** (1): `timeouts`

### `aws_bedrock_inference_profile`

**Source:** `crates/winterbaume-terraform/src/converters/bedrock.rs`

**Inject attributes** (9): `arn`, `created_at`, `description`, `name`, `region`, `status`, `tags`, `type`, `updated_at`

**Extract attributes** (13): `arn`, `copy_from`, `created_at`, `description`, `id`, `model_arn`, `model_source`, `models`, `name`, `status`, `tags_all`, `type`, `updated_at`

**Missing from inject** (4): `model_source`, `models`, `tags_all`, `timeouts`

**Missing from extract** (2): `tags`, `timeouts`

### `aws_bedrock_model_invocation_logging_configuration`

**Source:** `crates/winterbaume-terraform/src/converters/bedrock.rs`

**Inject attributes** (2): `logging_config`, `region`

**Extract attributes** (11): `bucket_name`, `cloudwatch_configuration`, `embedding_data_delivery_enabled`, `id`, `image_data_delivery_enabled`, `key_prefix`, `log_group_name`, `logging_config`, `role_arn`, `s3_configuration`, `text_data_delivery_enabled`

### `aws_bedrock_provisioned_model_throughput`

**Source:** `crates/winterbaume-terraform/src/converters/bedrock.rs`

**Inject attributes** (7): `commitment_duration`, `model_arn`, `model_units`, `provisioned_model_arn`, `provisioned_model_name`, `region`, `tags`

**Extract attributes** (7): `commitment_duration`, `id`, `model_arn`, `model_units`, `provisioned_model_arn`, `provisioned_model_name`, `tags_all`

**Missing from inject** (2): `tags_all`, `timeouts`

**Missing from extract** (2): `tags`, `timeouts`

### `aws_bedrockagent_agent`

**Source:** `crates/winterbaume-terraform/src/converters/bedrockagent.rs`

**Inject attributes** (21): `agent_arn`, `agent_id`, `agent_name`, `agent_resource_role_arn`, `agent_status`, `agent_version`, `client_token`, `created_at`, `customer_encryption_key_arn`, `description`, `failure_reasons`, `foundation_model`, `idle_session_ttl_in_seconds`, `instruction`, `prepare_agent`, `prepared_at`, `recommended_actions`, `region`, `tags`, `tags_all`, `updated_at`

**Extract attributes** (20): `agent_arn`, `agent_id`, `agent_name`, `agent_resource_role_arn`, `agent_status`, `agent_version`, `client_token`, `created_at`, `customer_encryption_key_arn`, `description`, `failure_reasons`, `foundation_model`, `id`, `idle_session_ttl_in_seconds`, `instruction`, `prepare_agent`, `prepared_at`, `recommended_actions`, `tags_all`, `updated_at`

**Missing from inject** (6): `agent_collaboration`, `guardrail_configuration`, `memory_configuration`, `prompt_override_configuration`, `skip_resource_in_use_check`, `timeouts`

**Missing from extract** (7): `agent_collaboration`, `guardrail_configuration`, `memory_configuration`, `prompt_override_configuration`, `skip_resource_in_use_check`, `tags`, `timeouts`

### `aws_bedrockagent_agent_action_group`

**Source:** `crates/winterbaume-terraform/src/converters/bedrockagent.rs`

**Inject attributes** (14): `action_group_executor`, `action_group_id`, `action_group_name`, `action_group_state`, `agent_id`, `agent_version`, `api_schema`, `client_token`, `created_at`, `description`, `function_schema`, `parent_action_group_signature`, `region`, `updated_at`

**Extract attributes** (14): `action_group_executor`, `action_group_id`, `action_group_name`, `action_group_state`, `agent_id`, `agent_version`, `api_schema`, `client_token`, `created_at`, `description`, `function_schema`, `id`, `parent_action_group_signature`, `updated_at`

**Missing from inject** (3): `prepare_agent`, `skip_resource_in_use_check`, `timeouts`

**Missing from extract** (3): `prepare_agent`, `skip_resource_in_use_check`, `timeouts`

### `aws_bedrockagent_agent_alias`

**Source:** `crates/winterbaume-terraform/src/converters/bedrockagent.rs`

**Inject attributes** (12): `agent_alias_arn`, `agent_alias_id`, `agent_alias_name`, `agent_alias_status`, `agent_id`, `client_token`, `created_at`, `description`, `region`, `tags`, `tags_all`, `updated_at`

**Extract attributes** (11): `agent_alias_arn`, `agent_alias_id`, `agent_alias_name`, `agent_alias_status`, `agent_id`, `client_token`, `created_at`, `description`, `id`, `tags_all`, `updated_at`

**Missing from inject** (2): `routing_configuration`, `timeouts`

**Missing from extract** (3): `routing_configuration`, `tags`, `timeouts`

### `aws_bedrockagent_agent_collaborator`

**Source:** `crates/winterbaume-terraform/src/converters/bedrockagent.rs`

**Inject attributes** (11): `agent_descriptor`, `agent_id`, `agent_version`, `client_token`, `collaboration_instruction`, `collaborator_id`, `collaborator_name`, `created_at`, `last_updated_at`, `region`, `relay_conversation_history`

**Extract attributes** (11): `agent_descriptor`, `agent_id`, `agent_version`, `client_token`, `collaboration_instruction`, `collaborator_id`, `collaborator_name`, `created_at`, `id`, `last_updated_at`, `relay_conversation_history`

**Missing from inject** (2): `prepare_agent`, `timeouts`

**Missing from extract** (2): `prepare_agent`, `timeouts`

### `aws_bedrockagent_agent_knowledge_base_association`

**Source:** `crates/winterbaume-terraform/src/converters/bedrockagent.rs`

**Inject attributes** (8): `agent_id`, `agent_version`, `created_at`, `description`, `knowledge_base_id`, `knowledge_base_state`, `region`, `updated_at`

**Extract attributes** (8): `agent_id`, `agent_version`, `created_at`, `description`, `id`, `knowledge_base_id`, `knowledge_base_state`, `updated_at`

**Missing from inject** (1): `timeouts`

**Missing from extract** (1): `timeouts`

### `aws_bedrockagent_data_source`

**Source:** `crates/winterbaume-terraform/src/converters/bedrockagent.rs`

**Inject attributes** (12): `created_at`, `data_deletion_policy`, `data_source_configuration`, `data_source_id`, `description`, `knowledge_base_id`, `name`, `region`, `server_side_encryption_configuration`, `status`, `updated_at`, `vector_ingestion_configuration`

**Extract attributes** (12): `created_at`, `data_deletion_policy`, `data_source_configuration`, `data_source_id`, `description`, `id`, `knowledge_base_id`, `name`, `server_side_encryption_configuration`, `status`, `updated_at`, `vector_ingestion_configuration`

**Missing from inject** (1): `timeouts`

**Missing from extract** (1): `timeouts`

### `aws_bedrockagent_knowledge_base`

**Source:** `crates/winterbaume-terraform/src/converters/bedrockagent.rs`

**Inject attributes** (13): `created_at`, `description`, `failure_reasons`, `knowledge_base_arn`, `knowledge_base_configuration`, `knowledge_base_id`, `name`, `region`, `role_arn`, `status`, `storage_configuration`, `tags`, `updated_at`

**Extract attributes** (12): `created_at`, `description`, `failure_reasons`, `id`, `knowledge_base_arn`, `knowledge_base_configuration`, `knowledge_base_id`, `name`, `role_arn`, `status`, `storage_configuration`, `updated_at`

**Missing from inject** (3): `arn`, `tags_all`, `timeouts`

**Missing from extract** (4): `arn`, `tags`, `tags_all`, `timeouts`

### `aws_bedrockagent_prompt`

**Source:** `crates/winterbaume-terraform/src/converters/bedrockagent.rs`

**Inject attributes** (11): `arn`, `created_at`, `customer_encryption_key_arn`, `default_variant`, `description`, `name`, `region`, `tags`, `tags_all`, `updated_at`, `version`

**Extract attributes** (10): `arn`, `created_at`, `customer_encryption_key_arn`, `default_variant`, `description`, `id`, `name`, `tags_all`, `updated_at`, `version`

**Missing from inject** (1): `variant`

**Missing from extract** (2): `tags`, `variant`

### `aws_budgets_budget`

**Source:** `crates/winterbaume-terraform/src/converters/budgets.rs`

**Inject attributes** (10): `auto_adjust_data`, `budget_type`, `cost_types`, `limit_amount`, `limit_unit`, `name`, `notification`, `planned_limit`, `region`, `time_unit`

**Extract attributes** (17): `arn`, `auto_adjust_data`, `budget_type`, `comparison_operator`, `cost_filter`, `cost_types`, `id`, `limit_amount`, `limit_unit`, `name`, `notification`, `notification_type`, `planned_limit`, `tags_all`, `threshold`, `threshold_type`, `time_unit`

**Missing from inject** (8): `account_id`, `arn`, `cost_filter`, `name_prefix`, `tags`, `tags_all`, `time_period_end`, `time_period_start`

**Missing from extract** (5): `account_id`, `name_prefix`, `tags`, `time_period_end`, `time_period_start`

### `aws_chatbot_microsoft_teams_channel_configuration`

**Source:** `crates/winterbaume-terraform/src/converters/chatbot.rs`

**Inject attributes** (11): `channel_id`, `channel_name`, `chat_configuration_arn`, `configuration_name`, `iam_role_arn`, `logging_level`, `region`, `tags`, `team_id`, `team_name`, `tenant_id`

**Extract attributes** (10): `channel_id`, `channel_name`, `chat_configuration_arn`, `configuration_name`, `iam_role_arn`, `logging_level`, `tags`, `team_id`, `team_name`, `tenant_id`

**Note:** Resource type not found in Terraform AWS provider schema.

### `aws_chatbot_slack_channel_configuration`

**Source:** `crates/winterbaume-terraform/src/converters/chatbot.rs`

**Inject attributes** (9): `chat_configuration_arn`, `configuration_name`, `iam_role_arn`, `logging_level`, `region`, `slack_channel_id`, `slack_channel_name`, `slack_team_id`, `tags`

**Extract attributes** (8): `chat_configuration_arn`, `configuration_name`, `iam_role_arn`, `logging_level`, `slack_channel_id`, `slack_channel_name`, `slack_team_id`, `tags`

**Missing from inject** (6): `guardrail_policy_arns`, `slack_team_name`, `sns_topic_arns`, `tags_all`, `timeouts`, `user_authorization_required`

**Missing from extract** (6): `guardrail_policy_arns`, `slack_team_name`, `sns_topic_arns`, `tags_all`, `timeouts`, `user_authorization_required`

### `aws_cloudformation_stack`

**Source:** `crates/winterbaume-terraform/src/converters/cloudformation.rs`

**Inject attributes** (8): `disable_rollback`, `iam_role_arn`, `name`, `parameters`, `region`, `tags`, `template_body`, `timeout_in_minutes`

**Extract attributes** (11): `capabilities`, `disable_rollback`, `iam_role_arn`, `id`, `name`, `notification_arns`, `on_failure`, `outputs`, `stack_status`, `tags_all`, `timeout_in_minutes`

**Missing from inject** (9): `capabilities`, `notification_arns`, `on_failure`, `outputs`, `policy_body`, `policy_url`, `tags_all`, `template_url`, `timeouts`

**Missing from extract** (7): `parameters`, `policy_body`, `policy_url`, `tags`, `template_body`, `template_url`, `timeouts`

### `aws_cloudformation_stack_instances`

**Source:** `crates/winterbaume-terraform/src/converters/cloudformation.rs`

**Inject attributes** (4): `accounts`, `region`, `regions`, `stack_set_name`

**Extract attributes** (0): (none)

**Missing from inject** (8): `call_as`, `deployment_targets`, `operation_preferences`, `parameter_overrides`, `retain_stacks`, `stack_instance_summaries`, `stack_set_id`, `timeouts`

**Missing from extract** (11): `accounts`, `call_as`, `deployment_targets`, `operation_preferences`, `parameter_overrides`, `regions`, `retain_stacks`, `stack_instance_summaries`, `stack_set_id`, `stack_set_name`, `timeouts`

### `aws_cloudformation_stack_set`

**Source:** `crates/winterbaume-terraform/src/converters/cloudformation.rs`

**Inject attributes** (9): `arn`, `capabilities`, `description`, `name`, `parameters`, `region`, `stack_set_id`, `tags`, `template_body`

**Extract attributes** (8): `arn`, `capabilities`, `description`, `id`, `name`, `stack_set_id`, `tags`, `template_body`

**Missing from inject** (10): `administration_role_arn`, `auto_deployment`, `call_as`, `execution_role_name`, `managed_execution`, `operation_preferences`, `permission_model`, `tags_all`, `template_url`, `timeouts`

**Missing from extract** (11): `administration_role_arn`, `auto_deployment`, `call_as`, `execution_role_name`, `managed_execution`, `operation_preferences`, `parameters`, `permission_model`, `tags_all`, `template_url`, `timeouts`

### `aws_cloudformation_stack_set_instance`

**Source:** `crates/winterbaume-terraform/src/converters/cloudformation.rs`

**Inject attributes** (4): `account_id`, `region`, `stack_id`, `stack_set_name`

**Extract attributes** (5): `account_id`, `id`, `region`, `stack_id`, `stack_set_name`

**Missing from inject** (8): `call_as`, `deployment_targets`, `operation_preferences`, `organizational_unit_id`, `parameter_overrides`, `retain_stack`, `stack_instance_summaries`, `timeouts`

**Missing from extract** (8): `call_as`, `deployment_targets`, `operation_preferences`, `organizational_unit_id`, `parameter_overrides`, `retain_stack`, `stack_instance_summaries`, `timeouts`

### `aws_cloudformation_type`

**Source:** `crates/winterbaume-terraform/src/converters/cloudformation.rs`

**Inject attributes** (6): `arn`, `default_version_id`, `description`, `region`, `type`, `type_name`

**Extract attributes** (7): `, rt.type_kind, rt.type_name.replace(`, `arn`, `default_version_id`, `description`, `id`, `type`, `type_name`

**Missing from inject** (12): `deprecated_status`, `documentation_url`, `execution_role_arn`, `is_default_version`, `logging_config`, `provisioning_type`, `schema`, `schema_handler_package`, `source_url`, `type_arn`, `version_id`, `visibility`

**Missing from extract** (12): `deprecated_status`, `documentation_url`, `execution_role_arn`, `is_default_version`, `logging_config`, `provisioning_type`, `schema`, `schema_handler_package`, `source_url`, `type_arn`, `version_id`, `visibility`

### `aws_cloudfront_cache_policy`

**Source:** `crates/winterbaume-terraform/src/converters/cloudfront.rs`

**Inject attributes** (7): `comment`, `default_ttl`, `etag`, `max_ttl`, `min_ttl`, `name`, `region`

**Extract attributes** (7): `comment`, `default_ttl`, `etag`, `id`, `max_ttl`, `min_ttl`, `name`

**Missing from inject** (2): `arn`, `parameters_in_cache_key_and_forwarded_to_origin`

**Missing from extract** (2): `arn`, `parameters_in_cache_key_and_forwarded_to_origin`

### `aws_cloudfront_continuous_deployment_policy`

**Source:** `crates/winterbaume-terraform/src/converters/cloudfront.rs`

**Inject attributes** (3): `enabled`, `etag`, `region`

**Extract attributes** (3): `enabled`, `etag`, `id`

**Missing from inject** (4): `arn`, `last_modified_time`, `staging_distribution_dns_names`, `traffic_config`

**Missing from extract** (4): `arn`, `last_modified_time`, `staging_distribution_dns_names`, `traffic_config`

### `aws_cloudfront_distribution`

**Source:** `crates/winterbaume-terraform/src/converters/cloudfront.rs`

**Inject attributes** (11): `arn`, `custom_error_response`, `default_cache_behavior`, `domain_name`, `enabled`, `logging_config`, `ordered_cache_behavior`, `origin`, `origin_group`, `region`, `tags`

**Extract attributes** (31): `allowed_methods`, `arn`, `cached_methods`, `caller_reference`, `cloudfront_default_certificate`, `compress`, `custom_error_response`, `default_cache_behavior`, `domain_name`, `enabled`, `etag`, `geo_restriction`, `hosted_zone_id`, `id`, `in_progress_validation_batches`, `last_modified_time`, `locations`, `logging_config`, `ordered_cache_behavior`, `origin`, `origin_group`, `origin_id`, `price_class`, `restriction_type`, `restrictions`, `status`, `tags`, `tags_all`, `target_origin_id`, `viewer_certificate`, `viewer_protocol_policy`

**Missing from inject** (22): `aliases`, `caller_reference`, `comment`, `continuous_deployment_policy_id`, `default_root_object`, `etag`, `hosted_zone_id`, `http_version`, `in_progress_validation_batches`, `is_ipv6_enabled`, `last_modified_time`, `price_class`, `restrictions`, `retain_on_delete`, `staging`, `status`, `tags_all`, `trusted_key_groups`, `trusted_signers`, `viewer_certificate`, `wait_for_deployment`, `web_acl_id`

**Missing from extract** (12): `aliases`, `comment`, `continuous_deployment_policy_id`, `default_root_object`, `http_version`, `is_ipv6_enabled`, `retain_on_delete`, `staging`, `trusted_key_groups`, `trusted_signers`, `wait_for_deployment`, `web_acl_id`

### `aws_cloudfront_field_level_encryption_config`

**Source:** `crates/winterbaume-terraform/src/converters/cloudfront.rs`

**Inject attributes** (3): `comment`, `etag`, `region`

**Extract attributes** (3): `comment`, `etag`, `id`

**Missing from inject** (4): `arn`, `caller_reference`, `content_type_profile_config`, `query_arg_profile_config`

**Missing from extract** (4): `arn`, `caller_reference`, `content_type_profile_config`, `query_arg_profile_config`

### `aws_cloudfront_field_level_encryption_profile`

**Source:** `crates/winterbaume-terraform/src/converters/cloudfront.rs`

**Inject attributes** (4): `comment`, `etag`, `name`, `region`

**Extract attributes** (4): `comment`, `etag`, `id`, `name`

**Missing from inject** (3): `arn`, `caller_reference`, `encryption_entities`

**Missing from extract** (3): `arn`, `caller_reference`, `encryption_entities`

### `aws_cloudfront_function`

**Source:** `crates/winterbaume-terraform/src/converters/cloudfront.rs`

**Inject attributes** (9): `arn`, `code`, `comment`, `etag`, `name`, `publish`, `region`, `runtime`, `status`

**Extract attributes** (8): `arn`, `code`, `comment`, `etag`, `id`, `name`, `runtime`, `status`

**Missing from inject** (2): `key_value_store_associations`, `live_stage_etag`

**Missing from extract** (3): `key_value_store_associations`, `live_stage_etag`, `publish`

### `aws_cloudfront_key_group`

**Source:** `crates/winterbaume-terraform/src/converters/cloudfront.rs`

**Inject attributes** (5): `comment`, `etag`, `items`, `name`, `region`

**Extract attributes** (5): `comment`, `etag`, `id`, `items`, `name`

### `aws_cloudfront_key_value_store`

**Source:** `crates/winterbaume-terraform/src/converters/cloudfront.rs`

**Inject attributes** (5): `arn`, `comment`, `etag`, `name`, `region`

**Extract attributes** (6): `arn`, `comment`, `etag`, `id`, `name`, `status`

**Missing from inject** (2): `last_modified_time`, `timeouts`

**Missing from extract** (2): `last_modified_time`, `timeouts`

### `aws_cloudfront_monitoring_subscription`

**Source:** `crates/winterbaume-terraform/src/converters/cloudfront.rs`

**Inject attributes** (3): `distribution_id`, `monitoring_subscription`, `region`

**Extract attributes** (5): `distribution_id`, `id`, `monitoring_subscription`, `realtime_metrics_subscription_config`, `realtime_metrics_subscription_status`

### `aws_cloudfront_origin_access_control`

**Source:** `crates/winterbaume-terraform/src/converters/cloudfront.rs`

**Inject attributes** (7): `description`, `etag`, `name`, `origin_access_control_origin_type`, `region`, `signing_behavior`, `signing_protocol`

**Extract attributes** (7): `description`, `etag`, `id`, `name`, `origin_access_control_origin_type`, `signing_behavior`, `signing_protocol`

**Missing from inject** (1): `arn`

**Missing from extract** (1): `arn`

### `aws_cloudfront_origin_access_identity`

**Source:** `crates/winterbaume-terraform/src/converters/cloudfront.rs`

**Inject attributes** (5): `caller_reference`, `comment`, `etag`, `region`, `s3_canonical_user_id`

**Extract attributes** (7): `caller_reference`, `cloudfront_access_identity_path`, `comment`, `etag`, `iam_arn`, `id`, `s3_canonical_user_id`

**Missing from inject** (3): `arn`, `cloudfront_access_identity_path`, `iam_arn`

**Missing from extract** (1): `arn`

### `aws_cloudfront_origin_request_policy`

**Source:** `crates/winterbaume-terraform/src/converters/cloudfront.rs`

**Inject attributes** (4): `comment`, `etag`, `name`, `region`

**Extract attributes** (4): `comment`, `etag`, `id`, `name`

**Missing from inject** (4): `arn`, `cookies_config`, `headers_config`, `query_strings_config`

**Missing from extract** (4): `arn`, `cookies_config`, `headers_config`, `query_strings_config`

### `aws_cloudfront_public_key`

**Source:** `crates/winterbaume-terraform/src/converters/cloudfront.rs`

**Inject attributes** (6): `caller_reference`, `comment`, `encoded_key`, `etag`, `name`, `region`

**Extract attributes** (6): `caller_reference`, `comment`, `encoded_key`, `etag`, `id`, `name`

**Missing from inject** (1): `name_prefix`

**Missing from extract** (1): `name_prefix`

### `aws_cloudfront_realtime_log_config`

**Source:** `crates/winterbaume-terraform/src/converters/cloudfront.rs`

**Inject attributes** (5): `arn`, `fields`, `name`, `region`, `sampling_rate`

**Extract attributes** (5): `arn`, `fields`, `id`, `name`, `sampling_rate`

**Missing from inject** (1): `endpoint`

**Missing from extract** (1): `endpoint`

### `aws_cloudfront_response_headers_policy`

**Source:** `crates/winterbaume-terraform/src/converters/cloudfront.rs`

**Inject attributes** (4): `comment`, `etag`, `name`, `region`

**Extract attributes** (4): `comment`, `etag`, `id`, `name`

**Missing from inject** (6): `arn`, `cors_config`, `custom_headers_config`, `remove_headers_config`, `security_headers_config`, `server_timing_headers_config`

**Missing from extract** (6): `arn`, `cors_config`, `custom_headers_config`, `remove_headers_config`, `security_headers_config`, `server_timing_headers_config`

### `aws_cloudfront_vpc_origin`

**Source:** `crates/winterbaume-terraform/src/converters/cloudfront.rs`

**Inject attributes** (5): `arn`, `etag`, `region`, `tags`, `vpc_origin_endpoint_config`

**Extract attributes** (8): `arn`, `etag`, `http_port`, `https_port`, `id`, `name`, `origin_protocol_policy`, `vpc_origin_endpoint_config`

**Missing from inject** (2): `tags_all`, `timeouts`

**Missing from extract** (3): `tags`, `tags_all`, `timeouts`

### `aws_cloudhsm_v2_cluster`

**Source:** `crates/winterbaume-terraform/src/converters/cloudhsmv2.rs`

**Inject attributes** (20): `acl_name`, `arn`, `auto_minor_version_upgrade`, `description`, `engine`, `engine_version`, `maintenance_window`, `name`, `node_type`, `num_replicas_per_shard`, `num_shards`, `parameter_group_name`, `region`, `snapshot_retention_limit`, `snapshot_window`, `subnet_group_name`, `subnet_ids`, `tags`, `tags_all`, `tls_enabled`

**Extract attributes** (20): `availability_zone`, `backup_policy`, `backup_retention_policy`, `cluster_id`, `cluster_state`, `create_timestamp`, `eni_id`, `eni_ip`, `hsm_id`, `hsm_type`, `hsms`, `id`, `security_group_id`, `source_backup_id`, `state`, `subnet_id`, `subnet_ids`, `subnet_mapping`, `tags`, `vpc_id`

**Missing from inject** (9): `cluster_certificates`, `cluster_id`, `cluster_state`, `hsm_type`, `mode`, `security_group_id`, `source_backup_identifier`, `timeouts`, `vpc_id`

**Missing from extract** (5): `cluster_certificates`, `mode`, `source_backup_identifier`, `tags_all`, `timeouts`

### `aws_cloudtrail`

**Source:** `crates/winterbaume-terraform/src/converters/cloudtrail.rs`

**Inject attributes** (12): `advanced_event_selector`, `arn`, `enable_logging`, `event_selector`, `include_global_service_events`, `insight_selector`, `is_multi_region_trail`, `name`, `region`, `s3_bucket_name`, `s3_key_prefix`, `tags`

**Extract attributes** (20): `advanced_event_selector`, `arn`, `data_resource`, `enable_logging`, `event_selector`, `exclude_management_event_sources`, `home_region`, `id`, `include_global_service_events`, `include_management_events`, `insight_selector`, `insight_type`, `is_multi_region_trail`, `name`, `read_write_type`, `s3_bucket_name`, `s3_key_prefix`, `tags`, `type`, `values`

**Missing from inject** (9): `cloud_watch_logs_group_arn`, `cloud_watch_logs_role_arn`, `enable_log_file_validation`, `home_region`, `is_organization_trail`, `kms_key_id`, `sns_topic_arn`, `sns_topic_name`, `tags_all`

**Missing from extract** (8): `cloud_watch_logs_group_arn`, `cloud_watch_logs_role_arn`, `enable_log_file_validation`, `is_organization_trail`, `kms_key_id`, `sns_topic_arn`, `sns_topic_name`, `tags_all`

### `aws_cloudwatch_composite_alarm`

**Source:** `crates/winterbaume-terraform/src/converters/cloudwatch.rs`

**Inject attributes** (6): `actions_enabled`, `alarm_description`, `alarm_name`, `alarm_rule`, `arn`, `region`

**Extract attributes** (9): `actions_enabled`, `alarm_actions`, `alarm_description`, `alarm_name`, `alarm_rule`, `arn`, `id`, `insufficient_data_actions`, `ok_actions`

**Missing from inject** (6): `actions_suppressor`, `alarm_actions`, `insufficient_data_actions`, `ok_actions`, `tags`, `tags_all`

**Missing from extract** (3): `actions_suppressor`, `tags`, `tags_all`

### `aws_cloudwatch_dashboard`

**Source:** `crates/winterbaume-terraform/src/converters/cloudwatch.rs`

**Inject attributes** (4): `dashboard_arn`, `dashboard_body`, `dashboard_name`, `region`

**Extract attributes** (4): `dashboard_arn`, `dashboard_body`, `dashboard_name`, `id`

### `aws_cloudwatch_metric_alarm`

**Source:** `crates/winterbaume-terraform/src/converters/cloudwatch.rs`

**Inject attributes** (17): `actions_enabled`, `alarm_actions`, `alarm_description`, `alarm_name`, `arn`, `comparison_operator`, `dimensions`, `evaluation_periods`, `insufficient_data_actions`, `metric_name`, `namespace`, `ok_actions`, `period`, `region`, `statistic`, `threshold`, `unit`

**Extract attributes** (17): `actions_enabled`, `alarm_actions`, `alarm_description`, `alarm_name`, `arn`, `comparison_operator`, `dimensions`, `evaluation_periods`, `id`, `insufficient_data_actions`, `metric_name`, `namespace`, `ok_actions`, `period`, `statistic`, `threshold`, `unit`

**Missing from inject** (8): `datapoints_to_alarm`, `evaluate_low_sample_count_percentiles`, `extended_statistic`, `metric_query`, `tags`, `tags_all`, `threshold_metric_id`, `treat_missing_data`

**Missing from extract** (8): `datapoints_to_alarm`, `evaluate_low_sample_count_percentiles`, `extended_statistic`, `metric_query`, `tags`, `tags_all`, `threshold_metric_id`, `treat_missing_data`

### `aws_cloudwatch_metric_stream`

**Source:** `crates/winterbaume-terraform/src/converters/cloudwatch.rs`

**Inject attributes** (7): `arn`, `firehose_arn`, `name`, `output_format`, `region`, `role_arn`, `state`

**Extract attributes** (9): `arn`, `creation_date`, `firehose_arn`, `id`, `last_update_date`, `name`, `output_format`, `role_arn`, `state`

**Missing from inject** (10): `creation_date`, `exclude_filter`, `include_filter`, `include_linked_accounts_metrics`, `last_update_date`, `name_prefix`, `statistics_configuration`, `tags`, `tags_all`, `timeouts`

**Missing from extract** (8): `exclude_filter`, `include_filter`, `include_linked_accounts_metrics`, `name_prefix`, `statistics_configuration`, `tags`, `tags_all`, `timeouts`

### `aws_codeartifact_domain`

**Source:** `crates/winterbaume-terraform/src/converters/codeartifact.rs`

**Inject attributes** (12): `app_network_access_type`, `arn`, `auth_mode`, `creation_time`, `domain_name`, `home_efs_file_system_id`, `kms_key_id`, `last_modified_time`, `region`, `status`, `url`, `vpc_id`

**Extract attributes** (7): `arn`, `domain`, `encryption_key`, `id`, `owner`, `tags`, `tags_all`

**Missing from inject** (9): `asset_size_bytes`, `created_time`, `domain`, `encryption_key`, `owner`, `repository_count`, `s3_bucket_arn`, `tags`, `tags_all`

**Missing from extract** (4): `asset_size_bytes`, `created_time`, `repository_count`, `s3_bucket_arn`

### `aws_codeartifact_repository`

**Source:** `crates/winterbaume-terraform/src/converters/codeartifact.rs`

**Inject attributes** (8): `arn`, `external_connections`, `image_tag_mutability`, `name`, `region`, `repository_url`, `tags`, `upstream`

**Extract attributes** (9): `arn`, `description`, `domain`, `domain_owner`, `external_connections`, `id`, `repository`, `tags`, `upstream`

**Missing from inject** (6): `administrator_account`, `description`, `domain`, `domain_owner`, `repository`, `tags_all`

**Missing from extract** (2): `administrator_account`, `tags_all`

### `aws_codebuild_fleet`

**Source:** `crates/winterbaume-terraform/src/converters/codebuild.rs`

**Inject attributes** (0): (none)

**Extract attributes** (0): (none)

**Missing from inject** (16): `arn`, `base_capacity`, `compute_configuration`, `compute_type`, `created`, `environment_type`, `fleet_service_role`, `image_id`, `last_modified`, `name`, `overflow_behavior`, `scaling_configuration`, `status`, `tags`, `tags_all`, `vpc_config`

**Missing from extract** (16): `arn`, `base_capacity`, `compute_configuration`, `compute_type`, `created`, `environment_type`, `fleet_service_role`, `image_id`, `last_modified`, `name`, `overflow_behavior`, `scaling_configuration`, `status`, `tags`, `tags_all`, `vpc_config`

### `aws_codebuild_project`

**Source:** `crates/winterbaume-terraform/src/converters/codebuild.rs`

**Inject attributes** (15): `arn`, `artifacts`, `build_batch_config`, `cache`, `environment`, `file_system_locations`, `logs_config`, `project_description`, `project_name`, `region`, `secondary_artifacts`, `secondary_sources`, `source`, `tags`, `vpc_config`

**Extract attributes** (29): `arn`, `artifacts`, `badge_enabled`, `badge_url`, `build_batch_config`, `build_timeout`, `cache`, `compute_type`, `concurrent_build_limit`, `created`, `description`, `encryption_key`, `environment`, `file_system_locations`, `id`, `image`, `last_modified`, `location`, `logs_config`, `name`, `queued_timeout`, `secondary_artifacts`, `secondary_sources`, `service_role`, `source`, `tags`, `tags_all`, `type`, `vpc_config`

**Missing from inject** (15): `badge_enabled`, `badge_url`, `build_timeout`, `concurrent_build_limit`, `description`, `encryption_key`, `name`, `project_visibility`, `public_project_alias`, `queued_timeout`, `resource_access_role`, `secondary_source_version`, `service_role`, `source_version`, `tags_all`

**Missing from extract** (5): `project_visibility`, `public_project_alias`, `resource_access_role`, `secondary_source_version`, `source_version`

### `aws_codebuild_report_group`

**Source:** `crates/winterbaume-terraform/src/converters/codebuild.rs`

**Inject attributes** (0): (none)

**Extract attributes** (0): (none)

**Missing from inject** (8): `arn`, `created`, `delete_reports`, `export_config`, `name`, `tags`, `tags_all`, `type`

**Missing from extract** (8): `arn`, `created`, `delete_reports`, `export_config`, `name`, `tags`, `tags_all`, `type`

### `aws_codebuild_resource_policy`

**Source:** `crates/winterbaume-terraform/src/converters/codebuild.rs`

**Inject attributes** (0): (none)

**Extract attributes** (0): (none)

**Missing from inject** (2): `policy`, `resource_arn`

**Missing from extract** (2): `policy`, `resource_arn`

### `aws_codebuild_source_credential`

**Source:** `crates/winterbaume-terraform/src/converters/codebuild.rs`

**Inject attributes** (0): (none)

**Extract attributes** (0): (none)

**Missing from inject** (5): `arn`, `auth_type`, `server_type`, `token`, `user_name`

**Missing from extract** (5): `arn`, `auth_type`, `server_type`, `token`, `user_name`

### `aws_codebuild_webhook`

**Source:** `crates/winterbaume-terraform/src/converters/codebuild.rs`

**Inject attributes** (0): (none)

**Extract attributes** (0): (none)

**Missing from inject** (9): `branch_filter`, `build_type`, `filter_group`, `manual_creation`, `payload_url`, `project_name`, `scope_configuration`, `secret`, `url`

**Missing from extract** (9): `branch_filter`, `build_type`, `filter_group`, `manual_creation`, `payload_url`, `project_name`, `scope_configuration`, `secret`, `url`

### `aws_codecommit_approval_rule_template`

**Source:** `crates/winterbaume-terraform/src/converters/codecommit.rs`

**Inject attributes** (4): `approval_rule_template_id`, `content`, `description`, `name`

**Extract attributes** (0): (none)

**Missing from inject** (4): `creation_date`, `last_modified_date`, `last_modified_user`, `rule_content_sha256`

**Missing from extract** (8): `approval_rule_template_id`, `content`, `creation_date`, `description`, `last_modified_date`, `last_modified_user`, `name`, `rule_content_sha256`

### `aws_codecommit_approval_rule_template_association`

**Source:** `crates/winterbaume-terraform/src/converters/codecommit.rs`

**Inject attributes** (2): `approval_rule_template_name`, `repository_name`

**Extract attributes** (0): (none)

**Missing from extract** (2): `approval_rule_template_name`, `repository_name`

### `aws_codecommit_repository`

**Source:** `crates/winterbaume-terraform/src/converters/codecommit.rs`

**Inject attributes** (6): `arn`, `image_tag_mutability`, `name`, `region`, `repository_url`, `tags`

**Extract attributes** (7): `arn`, `clone_url_http`, `clone_url_ssh`, `description`, `id`, `repository_id`, `repository_name`

**Missing from inject** (8): `clone_url_http`, `clone_url_ssh`, `default_branch`, `description`, `kms_key_id`, `repository_id`, `repository_name`, `tags_all`

**Missing from extract** (4): `default_branch`, `kms_key_id`, `tags`, `tags_all`

### `aws_codecommit_trigger`

**Source:** `crates/winterbaume-terraform/src/converters/codecommit.rs`

**Inject attributes** (2): `configuration_id`, `repository_name`

**Extract attributes** (0): (none)

**Missing from inject** (1): `trigger`

**Missing from extract** (3): `configuration_id`, `repository_name`, `trigger`

### `aws_codedeploy_app`

**Source:** `crates/winterbaume-terraform/src/converters/codedeploy.rs`

**Inject attributes** (7): `arn`, `date_created`, `date_updated`, `description`, `name`, `region`, `tags`

**Extract attributes** (5): `application_id`, `compute_platform`, `id`, `name`, `tags_all`

**Missing from inject** (5): `application_id`, `compute_platform`, `github_account_name`, `linked_to_github`, `tags_all`

**Missing from extract** (4): `arn`, `github_account_name`, `linked_to_github`, `tags`

### `aws_codedeploy_deployment_group`

**Source:** `crates/winterbaume-terraform/src/converters/codedeploy.rs`

**Inject attributes** (12): `alarm_configuration`, `app_name`, `blue_green_deployment_config`, `deployment_config_name`, `deployment_group_name`, `ec2_tag_filter`, `ec2_tag_set`, `ecs_service`, `load_balancer_info`, `region`, `service_role_arn`, `trigger_configuration`

**Extract attributes** (23): `alarm_configuration`, `app_name`, `arn`, `auto_rollback_configuration`, `blue_green_deployment_config`, `compute_platform`, `deployment_config_name`, `deployment_group_id`, `deployment_group_name`, `deployment_option`, `deployment_style`, `deployment_type`, `ec2_tag_filter`, `ec2_tag_set`, `ecs_service`, `enabled`, `events`, `id`, `load_balancer_info`, `service_role_arn`, `tags`, `tags_all`, `trigger_configuration`

**Missing from inject** (11): `arn`, `auto_rollback_configuration`, `autoscaling_groups`, `compute_platform`, `deployment_group_id`, `deployment_style`, `on_premises_instance_tag_filter`, `outdated_instances_strategy`, `tags`, `tags_all`, `termination_hook_enabled`

**Missing from extract** (4): `autoscaling_groups`, `on_premises_instance_tag_filter`, `outdated_instances_strategy`, `termination_hook_enabled`

### `aws_codepipeline`

**Source:** `crates/winterbaume-terraform/src/converters/codepipeline.rs`

**Inject attributes** (8): `artifact_store`, `description`, `name`, `region`, `stage`, `trigger`, `unique_id`, `variable`

**Extract attributes** (12): `arn`, `artifact_store`, `execution_mode`, `id`, `name`, `pipeline_type`, `role_arn`, `stage`, `tags`, `tags_all`, `trigger`, `variable`

**Missing from inject** (7): `arn`, `execution_mode`, `pipeline_type`, `role_arn`, `tags`, `tags_all`, `trigger_all`

**Missing from extract** (1): `trigger_all`

### `aws_cognito_identity_pool`

**Source:** `crates/winterbaume-terraform/src/converters/cognitoidentity.rs`

**Inject attributes** (9): `allow_unauthenticated_identities`, `cognito_identity_providers`, `developer_provider_name`, `identity_pool_name`, `openid_connect_provider_arns`, `region`, `saml_provider_arns`, `supported_login_providers`, `tags`

**Extract attributes** (13): `allow_unauthenticated_identities`, `arn`, `client_id`, `cognito_identity_providers`, `developer_provider_name`, `id`, `identity_pool_name`, `openid_connect_provider_arns`, `provider_name`, `saml_provider_arns`, `server_side_token_check`, `supported_login_providers`, `tags`

**Missing from inject** (3): `allow_classic_flow`, `arn`, `tags_all`

**Missing from extract** (2): `allow_classic_flow`, `tags_all`

### `aws_cognito_identity_pool_provider_principal_tag`

**Source:** `crates/winterbaume-terraform/src/converters/cognitoidentity.rs`

**Inject attributes** (5): `identity_pool_id`, `identity_provider_name`, `principal_tags`, `region`, `use_defaults`

**Extract attributes** (5): `id`, `identity_pool_id`, `identity_provider_name`, `principal_tags`, `use_defaults`

### `aws_cognito_identity_pool_roles_attachment`

**Source:** `crates/winterbaume-terraform/src/converters/cognitoidentity.rs`

**Inject attributes** (4): `identity_pool_id`, `region`, `role_mapping`, `roles`

**Extract attributes** (4): `id`, `identity_pool_id`, `role_mapping`, `roles`

### `aws_account_id`

**Source:** `crates/winterbaume-terraform/src/converters/cognitoidp.rs`

**Inject attributes** (0): (none)

**Extract attributes** (5): `certificate_arn`, `cloudfront_distribution_arn`, `domain`, `id`, `user_pool_id`

**Note:** Resource type not found in Terraform AWS provider schema.

### `aws_cognito_identity_provider`

**Source:** `crates/winterbaume-terraform/src/converters/cognitoidp.rs`

**Inject attributes** (7): `attribute_mapping`, `idp_identifiers`, `provider_details`, `provider_name`, `provider_type`, `region`, `user_pool_id`

**Extract attributes** (7): `attribute_mapping`, `id`, `idp_identifiers`, `provider_details`, `provider_name`, `provider_type`, `user_pool_id`

### `aws_cognito_managed_user_pool_client`

**Source:** `crates/winterbaume-terraform/src/converters/cognitoidp.rs`

**Inject attributes** (13): `allowed_oauth_flows`, `allowed_oauth_flows_user_pool_client`, `allowed_oauth_scopes`, `callback_urls`, `client_secret`, `explicit_auth_flows`, `logout_urls`, `name`, `name_prefix`, `refresh_token_validity`, `region`, `supported_identity_providers`, `user_pool_id`

**Extract attributes** (0): (none)

**Missing from inject** (13): `access_token_validity`, `analytics_configuration`, `auth_session_validity`, `default_redirect_uri`, `enable_propagate_additional_user_context_data`, `enable_token_revocation`, `id_token_validity`, `name_pattern`, `prevent_user_existence_errors`, `read_attributes`, `refresh_token_rotation`, `token_validity_units`, `write_attributes`

**Missing from extract** (25): `access_token_validity`, `allowed_oauth_flows`, `allowed_oauth_flows_user_pool_client`, `allowed_oauth_scopes`, `analytics_configuration`, `auth_session_validity`, `callback_urls`, `client_secret`, `default_redirect_uri`, `enable_propagate_additional_user_context_data`, `enable_token_revocation`, `explicit_auth_flows`, `id_token_validity`, `logout_urls`, `name`, `name_pattern`, `name_prefix`, `prevent_user_existence_errors`, `read_attributes`, `refresh_token_rotation`, `refresh_token_validity`, `supported_identity_providers`, `token_validity_units`, `user_pool_id`, `write_attributes`

### `aws_cognito_resource_server`

**Source:** `crates/winterbaume-terraform/src/converters/cognitoidp.rs`

**Inject attributes** (5): `identifier`, `name`, `region`, `scope`, `user_pool_id`

**Extract attributes** (8): `id`, `identifier`, `name`, `scope`, `scope_description`, `scope_identifiers`, `scope_name`, `user_pool_id`

**Missing from inject** (1): `scope_identifiers`

### `aws_cognito_user_group`

**Source:** `crates/winterbaume-terraform/src/converters/cognitoidp.rs`

**Inject attributes** (5): `arn`, `engine`, `precedence`, `region`, `user_group_id`

**Extract attributes** (6): `description`, `id`, `name`, `precedence`, `role_arn`, `user_pool_id`

**Missing from inject** (4): `description`, `name`, `role_arn`, `user_pool_id`

### `aws_cognito_user_in_group`

**Source:** `crates/winterbaume-terraform/src/converters/cognitoidp.rs`

**Inject attributes** (4): `group_name`, `region`, `user_pool_id`, `username`

**Extract attributes** (3): `group_name`, `user_pool_id`, `username`

### `aws_cognito_user_pool`

**Source:** `crates/winterbaume-terraform/src/converters/cognitoidp.rs`

**Inject attributes** (24): `account_recovery_setting`, `admin_create_user_config`, `alias_attributes`, `arn`, `auto_verified_attributes`, `custom_attributes`, `deletion_protection`, `device_configuration`, `domain`, `email_configuration`, `email_verification_message`, `email_verification_subject`, `lambda_config`, `mfa_configuration`, `name`, `password_policy`, `region`, `schema`, `sms_configuration`, `tags`, `tags_all`, `user_pool_add_ons`, `username_attributes`, `verification_message_template`

**Extract attributes** (22): `account_recovery_setting`, `admin_create_user_config`, `arn`, `auto_verified_attributes`, `creation_date`, `custom_attributes`, `custom_domain`, `deletion_protection`, `domain`, `email_configuration`, `endpoint`, `id`, `last_modified_date`, `mfa_configuration`, `name`, `password_policy`, `schema`, `sms_configuration`, `status`, `tags`, `tags_all`, `username_attributes`

**Missing from inject** (14): `creation_date`, `custom_domain`, `email_mfa_configuration`, `endpoint`, `estimated_number_of_users`, `last_modified_date`, `sign_in_policy`, `sms_authentication_message`, `sms_verification_message`, `software_token_mfa_configuration`, `user_attribute_update_settings`, `user_pool_tier`, `username_configuration`, `web_authn_configuration`

**Missing from extract** (17): `alias_attributes`, `device_configuration`, `email_mfa_configuration`, `email_verification_message`, `email_verification_subject`, `estimated_number_of_users`, `lambda_config`, `sign_in_policy`, `sms_authentication_message`, `sms_verification_message`, `software_token_mfa_configuration`, `user_attribute_update_settings`, `user_pool_add_ons`, `user_pool_tier`, `username_configuration`, `verification_message_template`, `web_authn_configuration`

### `aws_cognito_user_pool_client`

**Source:** `crates/winterbaume-terraform/src/converters/cognitoidp.rs`

**Inject attributes** (16): `access_token_validity`, `allowed_oauth_flows`, `allowed_oauth_flows_user_pool_client`, `allowed_oauth_scopes`, `analytics_configuration`, `callback_urls`, `client_secret`, `explicit_auth_flows`, `id_token_validity`, `logout_urls`, `name`, `refresh_token_validity`, `region`, `supported_identity_providers`, `token_validity_units`, `user_pool_id`

**Extract attributes** (14): `access_token_validity`, `allowed_oauth_flows`, `allowed_oauth_flows_user_pool_client`, `allowed_oauth_scopes`, `callback_urls`, `client_secret`, `explicit_auth_flows`, `id`, `logout_urls`, `name`, `refresh_token_validity`, `supported_identity_providers`, `tags_all`, `user_pool_id`

**Missing from inject** (9): `auth_session_validity`, `default_redirect_uri`, `enable_propagate_additional_user_context_data`, `enable_token_revocation`, `generate_secret`, `prevent_user_existence_errors`, `read_attributes`, `refresh_token_rotation`, `write_attributes`

**Missing from extract** (12): `analytics_configuration`, `auth_session_validity`, `default_redirect_uri`, `enable_propagate_additional_user_context_data`, `enable_token_revocation`, `generate_secret`, `id_token_validity`, `prevent_user_existence_errors`, `read_attributes`, `refresh_token_rotation`, `token_validity_units`, `write_attributes`

### `aws_cognito_user_pool_domain`

**Source:** `crates/winterbaume-terraform/src/converters/cognitoidp.rs`

**Inject attributes** (4): `certificate_arn`, `domain`, `region`, `user_pool_id`

**Extract attributes** (6): `aws_account_id`, `certificate_arn`, `cloudfront_distribution_arn`, `domain`, `id`, `user_pool_id`

**Missing from inject** (7): `aws_account_id`, `cloudfront_distribution`, `cloudfront_distribution_arn`, `cloudfront_distribution_zone_id`, `managed_login_version`, `s3_bucket`, `version`

**Missing from extract** (5): `cloudfront_distribution`, `cloudfront_distribution_zone_id`, `managed_login_version`, `s3_bucket`, `version`

### `aws_cognito_user_pool_ui_customization`

**Source:** `crates/winterbaume-terraform/src/converters/cognitoidp.rs`

**Inject attributes** (5): `client_id`, `css`, `image_file`, `region`, `user_pool_id`

**Extract attributes** (0): (none)

**Missing from inject** (4): `creation_date`, `css_version`, `image_url`, `last_modified_date`

**Missing from extract** (8): `client_id`, `creation_date`, `css`, `css_version`, `image_file`, `image_url`, `last_modified_date`, `user_pool_id`

### `aws_comprehend_entity_recognizer`

**Source:** `crates/winterbaume-terraform/src/converters/comprehend.rs`

**Inject attributes** (8): `arn`, `data_access_role_arn`, `input_data_config`, `language_code`, `name`, `region`, `status`, `tags`

**Extract attributes** (14): `arn`, `data_access_role_arn`, `entity_list`, `entity_types`, `id`, `input_data_config`, `language_code`, `name`, `s3_uri`, `status`, `submit_time`, `tags`, `tags_all`, `type`

**Missing from inject** (7): `model_kms_key_id`, `tags_all`, `timeouts`, `version_name`, `version_name_prefix`, `volume_kms_key_id`, `vpc_config`

**Missing from extract** (6): `model_kms_key_id`, `timeouts`, `version_name`, `version_name_prefix`, `volume_kms_key_id`, `vpc_config`

### `aws_config_aggregate_authorization`

**Source:** `crates/winterbaume-terraform/src/converters/config.rs`

**Inject attributes** (3): `account_id`, `arn`, `region`

**Extract attributes** (4): `account_id`, `arn`, `id`, `region`

**Missing from inject** (2): `tags`, `tags_all`

**Missing from extract** (2): `tags`, `tags_all`

### `aws_config_config_rule`

**Source:** `crates/winterbaume-terraform/src/converters/config.rs`

**Inject attributes** (9): `arn`, `description`, `evaluation_mode`, `input_parameters`, `name`, `region`, `rule_id`, `scope`, `source`

**Extract attributes** (12): `arn`, `description`, `evaluation_mode`, `id`, `name`, `owner`, `rule_id`, `scope`, `source`, `source_identifier`, `tags`, `tags_all`

**Missing from inject** (3): `maximum_execution_frequency`, `tags`, `tags_all`

**Missing from extract** (2): `input_parameters`, `maximum_execution_frequency`

### `aws_config_configuration_aggregator`

**Source:** `crates/winterbaume-terraform/src/converters/config.rs`

**Inject attributes** (3): `arn`, `name`, `region`

**Extract attributes** (3): `arn`, `id`, `name`

**Missing from inject** (4): `account_aggregation_source`, `organization_aggregation_source`, `tags`, `tags_all`

**Missing from extract** (4): `account_aggregation_source`, `organization_aggregation_source`, `tags`, `tags_all`

### `aws_config_configuration_recorder`

**Source:** `crates/winterbaume-terraform/src/converters/config.rs`

**Inject attributes** (5): `name`, `recording_group`, `recording_mode`, `region`, `role_arn`

**Extract attributes** (4): `id`, `name`, `recording_mode`, `role_arn`

**Missing from extract** (1): `recording_group`

### `aws_config_configuration_recorder_status`

**Source:** `crates/winterbaume-terraform/src/converters/config.rs`

**Inject attributes** (3): `is_enabled`, `name`, `region`

**Extract attributes** (3): `id`, `is_enabled`, `name`

### `aws_config_conformance_pack`

**Source:** `crates/winterbaume-terraform/src/converters/config.rs`

**Inject attributes** (5): `arn`, `delivery_s3_bucket`, `delivery_s3_key_prefix`, `name`, `region`

**Extract attributes** (0): (none)

**Missing from inject** (3): `input_parameter`, `template_body`, `template_s3_uri`

**Missing from extract** (7): `arn`, `delivery_s3_bucket`, `delivery_s3_key_prefix`, `input_parameter`, `name`, `template_body`, `template_s3_uri`

### `aws_config_delivery_channel`

**Source:** `crates/winterbaume-terraform/src/converters/config.rs`

**Inject attributes** (5): `name`, `region`, `s3_bucket_name`, `s3_key_prefix`, `snapshot_delivery_properties`

**Extract attributes** (5): `id`, `name`, `s3_bucket_name`, `s3_key_prefix`, `snapshot_delivery_properties`

**Missing from inject** (2): `s3_kms_key_arn`, `sns_topic_arn`

**Missing from extract** (2): `s3_kms_key_arn`, `sns_topic_arn`

### `aws_config_organization_conformance_pack`

**Source:** `crates/winterbaume-terraform/src/converters/config.rs`

**Inject attributes** (5): `arn`, `delivery_s3_bucket`, `delivery_s3_key_prefix`, `name`, `region`

**Extract attributes** (5): `arn`, `delivery_s3_bucket`, `delivery_s3_key_prefix`, `id`, `name`

**Missing from inject** (5): `excluded_accounts`, `input_parameter`, `template_body`, `template_s3_uri`, `timeouts`

**Missing from extract** (5): `excluded_accounts`, `input_parameter`, `template_body`, `template_s3_uri`, `timeouts`

### `aws_config_organization_custom_policy_rule`

**Source:** `crates/winterbaume-terraform/src/converters/config.rs`

**Inject attributes** (8): `arn`, `description`, `input_parameters`, `maximum_execution_frequency`, `name`, `policy_runtime`, `policy_text`, `region`

**Extract attributes** (8): `arn`, `description`, `id`, `input_parameters`, `maximum_execution_frequency`, `name`, `policy_runtime`, `policy_text`

**Missing from inject** (8): `debug_log_delivery_accounts`, `excluded_accounts`, `resource_id_scope`, `resource_types_scope`, `tag_key_scope`, `tag_value_scope`, `timeouts`, `trigger_types`

**Missing from extract** (8): `debug_log_delivery_accounts`, `excluded_accounts`, `resource_id_scope`, `resource_types_scope`, `tag_key_scope`, `tag_value_scope`, `timeouts`, `trigger_types`

### `aws_config_organization_custom_rule`

**Source:** `crates/winterbaume-terraform/src/converters/config.rs`

**Inject attributes** (7): `arn`, `description`, `input_parameters`, `lambda_function_arn`, `maximum_execution_frequency`, `name`, `region`

**Extract attributes** (7): `arn`, `description`, `id`, `input_parameters`, `lambda_function_arn`, `maximum_execution_frequency`, `name`

**Missing from inject** (7): `excluded_accounts`, `resource_id_scope`, `resource_types_scope`, `tag_key_scope`, `tag_value_scope`, `timeouts`, `trigger_types`

**Missing from extract** (7): `excluded_accounts`, `resource_id_scope`, `resource_types_scope`, `tag_key_scope`, `tag_value_scope`, `timeouts`, `trigger_types`

### `aws_config_organization_managed_rule`

**Source:** `crates/winterbaume-terraform/src/converters/config.rs`

**Inject attributes** (7): `arn`, `description`, `input_parameters`, `maximum_execution_frequency`, `name`, `region`, `rule_identifier`

**Extract attributes** (7): `arn`, `description`, `id`, `input_parameters`, `maximum_execution_frequency`, `name`, `rule_identifier`

**Missing from inject** (6): `excluded_accounts`, `resource_id_scope`, `resource_types_scope`, `tag_key_scope`, `tag_value_scope`, `timeouts`

**Missing from extract** (6): `excluded_accounts`, `resource_id_scope`, `resource_types_scope`, `tag_key_scope`, `tag_value_scope`, `timeouts`

### `aws_config_remediation_configuration`

**Source:** `crates/winterbaume-terraform/src/converters/config.rs`

**Inject attributes** (9): `arn`, `automatic`, `config_rule_name`, `maximum_automatic_attempts`, `region`, `retry_attempt_seconds`, `target_id`, `target_type`, `target_version`

**Extract attributes** (9): `arn`, `automatic`, `config_rule_name`, `id`, `maximum_automatic_attempts`, `retry_attempt_seconds`, `target_id`, `target_type`, `target_version`

**Missing from inject** (3): `execution_controls`, `parameter`, `resource_type`

**Missing from extract** (3): `execution_controls`, `parameter`, `resource_type`

### `aws_config_retention_configuration`

**Source:** `crates/winterbaume-terraform/src/converters/config.rs`

**Inject attributes** (3): `name`, `region`, `retention_period_in_days`

**Extract attributes** (3): `id`, `name`, `retention_period_in_days`

### `aws_connect_bot_association`

**Source:** `crates/winterbaume-terraform/src/converters/connect.rs`

**Inject attributes** (2): `instance_id`, `lex_bot`

**Extract attributes** (0): (none)

**Missing from extract** (2): `instance_id`, `lex_bot`

### `aws_connect_contact_flow`

**Source:** `crates/winterbaume-terraform/src/converters/connect.rs`

**Inject attributes** (9): `arn`, `contact_flow_id`, `content`, `content_hash`, `description`, `filename`, `instance_id`, `name`, `tags`

**Extract attributes** (0): (none)

**Missing from inject** (2): `tags_all`, `type`

**Missing from extract** (11): `arn`, `contact_flow_id`, `content`, `content_hash`, `description`, `filename`, `instance_id`, `name`, `tags`, `tags_all`, `type`

### `aws_connect_contact_flow_module`

**Source:** `crates/winterbaume-terraform/src/converters/connect.rs`

**Inject attributes** (9): `arn`, `contact_flow_module_id`, `content`, `content_hash`, `description`, `filename`, `instance_id`, `name`, `tags`

**Extract attributes** (0): (none)

**Missing from inject** (1): `tags_all`

**Missing from extract** (10): `arn`, `contact_flow_module_id`, `content`, `content_hash`, `description`, `filename`, `instance_id`, `name`, `tags`, `tags_all`

### `aws_connect_hours_of_operation`

**Source:** `crates/winterbaume-terraform/src/converters/connect.rs`

**Inject attributes** (7): `arn`, `description`, `hours_of_operation_id`, `instance_id`, `name`, `tags`, `time_zone`

**Extract attributes** (0): (none)

**Missing from inject** (2): `config`, `tags_all`

**Missing from extract** (9): `arn`, `config`, `description`, `hours_of_operation_id`, `instance_id`, `name`, `tags`, `tags_all`, `time_zone`

### `aws_connect_instance`

**Source:** `crates/winterbaume-terraform/src/converters/connect.rs`

**Inject attributes** (9): `arn`, `created_time`, `identity_management_type`, `inbound_calls_enabled`, `instance_alias`, `outbound_calls_enabled`, `region`, `status`, `tags`

**Extract attributes** (11): `arn`, `contact_flow_logs_enabled`, `created_time`, `early_media_enabled`, `id`, `identity_management_type`, `inbound_calls_enabled`, `instance_alias`, `outbound_calls_enabled`, `status`, `tags`

**Missing from inject** (9): `auto_resolve_best_voices_enabled`, `contact_flow_logs_enabled`, `contact_lens_enabled`, `directory_id`, `early_media_enabled`, `multi_party_conference_enabled`, `service_role`, `tags_all`, `timeouts`

**Missing from extract** (7): `auto_resolve_best_voices_enabled`, `contact_lens_enabled`, `directory_id`, `multi_party_conference_enabled`, `service_role`, `tags_all`, `timeouts`

### `aws_connect_instance_storage_config`

**Source:** `crates/winterbaume-terraform/src/converters/connect.rs`

**Inject attributes** (2): `association_id`, `instance_id`

**Extract attributes** (0): (none)

**Missing from inject** (2): `resource_type`, `storage_config`

**Missing from extract** (4): `association_id`, `instance_id`, `resource_type`, `storage_config`

### `aws_connect_lambda_function_association`

**Source:** `crates/winterbaume-terraform/src/converters/connect.rs`

**Inject attributes** (2): `function_arn`, `instance_id`

**Extract attributes** (0): (none)

**Missing from extract** (2): `function_arn`, `instance_id`

### `aws_connect_phone_number`

**Source:** `crates/winterbaume-terraform/src/converters/connect.rs`

**Inject attributes** (6): `arn`, `country_code`, `description`, `phone_number`, `tags`, `target_arn`

**Extract attributes** (0): (none)

**Missing from inject** (5): `prefix`, `status`, `tags_all`, `timeouts`, `type`

**Missing from extract** (11): `arn`, `country_code`, `description`, `phone_number`, `prefix`, `status`, `tags`, `tags_all`, `target_arn`, `timeouts`, `type`

### `aws_connect_queue`

**Source:** `crates/winterbaume-terraform/src/converters/connect.rs`

**Inject attributes** (9): `arn`, `description`, `hours_of_operation_id`, `instance_id`, `max_contacts`, `name`, `queue_id`, `status`, `tags`

**Extract attributes** (0): (none)

**Missing from inject** (3): `outbound_caller_config`, `quick_connect_ids`, `tags_all`

**Missing from extract** (12): `arn`, `description`, `hours_of_operation_id`, `instance_id`, `max_contacts`, `name`, `outbound_caller_config`, `queue_id`, `quick_connect_ids`, `status`, `tags`, `tags_all`

### `aws_connect_quick_connect`

**Source:** `crates/winterbaume-terraform/src/converters/connect.rs`

**Inject attributes** (6): `arn`, `description`, `instance_id`, `name`, `quick_connect_id`, `tags`

**Extract attributes** (0): (none)

**Missing from inject** (2): `quick_connect_config`, `tags_all`

**Missing from extract** (8): `arn`, `description`, `instance_id`, `name`, `quick_connect_config`, `quick_connect_id`, `tags`, `tags_all`

### `aws_connect_routing_profile`

**Source:** `crates/winterbaume-terraform/src/converters/connect.rs`

**Inject attributes** (7): `arn`, `default_outbound_queue_id`, `description`, `instance_id`, `name`, `routing_profile_id`, `tags`

**Extract attributes** (0): (none)

**Missing from inject** (3): `media_concurrencies`, `queue_configs`, `tags_all`

**Missing from extract** (10): `arn`, `default_outbound_queue_id`, `description`, `instance_id`, `media_concurrencies`, `name`, `queue_configs`, `routing_profile_id`, `tags`, `tags_all`

### `aws_connect_security_profile`

**Source:** `crates/winterbaume-terraform/src/converters/connect.rs`

**Inject attributes** (7): `arn`, `description`, `instance_id`, `name`, `organization_resource_id`, `security_profile_id`, `tags`

**Extract attributes** (0): (none)

**Missing from inject** (2): `permissions`, `tags_all`

**Missing from extract** (9): `arn`, `description`, `instance_id`, `name`, `organization_resource_id`, `permissions`, `security_profile_id`, `tags`, `tags_all`

### `aws_connect_user`

**Source:** `crates/winterbaume-terraform/src/converters/connect.rs`

**Inject attributes** (9): `arn`, `directory_user_id`, `hierarchy_group_id`, `instance_id`, `name`, `password`, `routing_profile_id`, `tags`, `user_id`

**Extract attributes** (0): (none)

**Missing from inject** (4): `identity_info`, `phone_config`, `security_profile_ids`, `tags_all`

**Missing from extract** (13): `arn`, `directory_user_id`, `hierarchy_group_id`, `identity_info`, `instance_id`, `name`, `password`, `phone_config`, `routing_profile_id`, `security_profile_ids`, `tags`, `tags_all`, `user_id`

### `aws_connect_user_hierarchy_group`

**Source:** `crates/winterbaume-terraform/src/converters/connect.rs`

**Inject attributes** (7): `arn`, `hierarchy_group_id`, `instance_id`, `level_id`, `name`, `parent_group_id`, `tags`

**Extract attributes** (0): (none)

**Missing from inject** (2): `hierarchy_path`, `tags_all`

**Missing from extract** (9): `arn`, `hierarchy_group_id`, `hierarchy_path`, `instance_id`, `level_id`, `name`, `parent_group_id`, `tags`, `tags_all`

### `aws_connect_user_hierarchy_structure`

**Source:** `crates/winterbaume-terraform/src/converters/connect.rs`

**Inject attributes** (1): `instance_id`

**Extract attributes** (0): (none)

**Missing from inject** (1): `hierarchy_structure`

**Missing from extract** (2): `hierarchy_structure`, `instance_id`

### `aws_connect_vocabulary`

**Source:** `crates/winterbaume-terraform/src/converters/connect.rs`

**Inject attributes** (10): `arn`, `content`, `failure_reason`, `instance_id`, `language_code`, `last_modified_time`, `name`, `state`, `tags`, `vocabulary_id`

**Extract attributes** (0): (none)

**Missing from inject** (2): `tags_all`, `timeouts`

**Missing from extract** (12): `arn`, `content`, `failure_reason`, `instance_id`, `language_code`, `last_modified_time`, `name`, `state`, `tags`, `tags_all`, `timeouts`, `vocabulary_id`

### `aws_ce_anomaly_monitor`

**Source:** `crates/winterbaume-terraform/src/converters/costexplorer.rs`

**Inject attributes** (9): `arn`, `creation_date`, `last_evaluated_date`, `last_updated_date`, `monitor_dimension`, `monitor_type`, `name`, `region`, `tags`

**Extract attributes** (8): `arn`, `creation_date`, `id`, `last_evaluated_date`, `last_updated_date`, `monitor_dimension`, `monitor_type`, `name`

**Missing from inject** (2): `monitor_specification`, `tags_all`

**Missing from extract** (3): `monitor_specification`, `tags`, `tags_all`

### `aws_ce_anomaly_subscription`

**Source:** `crates/winterbaume-terraform/src/converters/costexplorer.rs`

**Inject attributes** (9): `account_id`, `arn`, `frequency`, `monitor_arn_list`, `name`, `region`, `subscriber`, `tags`, `threshold`

**Extract attributes** (11): `account_id`, `address`, `arn`, `frequency`, `id`, `monitor_arn_list`, `name`, `status`, `subscriber`, `threshold`, `type`

**Missing from inject** (2): `tags_all`, `threshold_expression`

**Missing from extract** (3): `tags`, `tags_all`, `threshold_expression`

### `aws_datapipeline_pipeline`

**Source:** `crates/winterbaume-terraform/src/converters/datapipeline.rs`

**Inject attributes** (4): `description`, `name`, `region`, `unique_id`

**Extract attributes** (4): `description`, `id`, `name`, `tags`

**Missing from inject** (2): `tags`, `tags_all`

**Missing from extract** (1): `tags_all`

### `aws_datasync_agent`

**Source:** `crates/winterbaume-terraform/src/converters/datasync.rs`

**Inject attributes** (6): `activation_key`, `arn`, `ip_address`, `name`, `private_link_endpoint`, `region`

**Extract attributes** (0): (none)

**Missing from inject** (6): `security_group_arns`, `subnet_arns`, `tags`, `tags_all`, `timeouts`, `vpc_endpoint_id`

**Missing from extract** (11): `activation_key`, `arn`, `ip_address`, `name`, `private_link_endpoint`, `security_group_arns`, `subnet_arns`, `tags`, `tags_all`, `timeouts`, `vpc_endpoint_id`

### `aws_datasync_location_azure_blob`

**Source:** `crates/winterbaume-terraform/src/converters/datasync.rs`

**Inject attributes** (8): `access_tier`, `arn`, `authentication_type`, `blob_type`, `container_url`, `region`, `subdirectory`, `uri`

**Extract attributes** (0): (none)

**Missing from inject** (4): `agent_arns`, `sas_configuration`, `tags`, `tags_all`

**Missing from extract** (11): `access_tier`, `agent_arns`, `arn`, `authentication_type`, `blob_type`, `container_url`, `sas_configuration`, `subdirectory`, `tags`, `tags_all`, `uri`

### `aws_datasync_location_efs`

**Source:** `crates/winterbaume-terraform/src/converters/datasync.rs`

**Inject attributes** (8): `access_point_arn`, `arn`, `efs_file_system_arn`, `file_system_access_role_arn`, `in_transit_encryption`, `region`, `subdirectory`, `uri`

**Extract attributes** (0): (none)

**Missing from inject** (3): `ec2_config`, `tags`, `tags_all`

**Missing from extract** (10): `access_point_arn`, `arn`, `ec2_config`, `efs_file_system_arn`, `file_system_access_role_arn`, `in_transit_encryption`, `subdirectory`, `tags`, `tags_all`, `uri`

### `aws_datasync_location_fsx_lustre_file_system`

**Source:** `crates/winterbaume-terraform/src/converters/datasync.rs`

**Inject attributes** (5): `arn`, `fsx_filesystem_arn`, `region`, `subdirectory`, `uri`

**Extract attributes** (0): (none)

**Missing from inject** (4): `creation_time`, `security_group_arns`, `tags`, `tags_all`

**Missing from extract** (8): `arn`, `creation_time`, `fsx_filesystem_arn`, `security_group_arns`, `subdirectory`, `tags`, `tags_all`, `uri`

### `aws_datasync_location_fsx_ontap_file_system`

**Source:** `crates/winterbaume-terraform/src/converters/datasync.rs`

**Inject attributes** (5): `arn`, `region`, `storage_virtual_machine_arn`, `subdirectory`, `uri`

**Extract attributes** (0): (none)

**Missing from inject** (6): `creation_time`, `fsx_filesystem_arn`, `protocol`, `security_group_arns`, `tags`, `tags_all`

**Missing from extract** (10): `arn`, `creation_time`, `fsx_filesystem_arn`, `protocol`, `security_group_arns`, `storage_virtual_machine_arn`, `subdirectory`, `tags`, `tags_all`, `uri`

### `aws_datasync_location_fsx_openzfs_file_system`

**Source:** `crates/winterbaume-terraform/src/converters/datasync.rs`

**Inject attributes** (5): `arn`, `fsx_filesystem_arn`, `region`, `subdirectory`, `uri`

**Extract attributes** (0): (none)

**Missing from inject** (5): `creation_time`, `protocol`, `security_group_arns`, `tags`, `tags_all`

**Missing from extract** (9): `arn`, `creation_time`, `fsx_filesystem_arn`, `protocol`, `security_group_arns`, `subdirectory`, `tags`, `tags_all`, `uri`

### `aws_datasync_location_fsx_windows_file_system`

**Source:** `crates/winterbaume-terraform/src/converters/datasync.rs`

**Inject attributes** (7): `arn`, `domain`, `fsx_filesystem_arn`, `region`, `subdirectory`, `uri`, `user`

**Extract attributes** (0): (none)

**Missing from inject** (5): `creation_time`, `password`, `security_group_arns`, `tags`, `tags_all`

**Missing from extract** (11): `arn`, `creation_time`, `domain`, `fsx_filesystem_arn`, `password`, `security_group_arns`, `subdirectory`, `tags`, `tags_all`, `uri`, `user`

### `aws_datasync_location_hdfs`

**Source:** `crates/winterbaume-terraform/src/converters/datasync.rs`

**Inject attributes** (10): `arn`, `authentication_type`, `block_size`, `kerberos_principal`, `kms_key_provider_uri`, `region`, `replication_factor`, `simple_user`, `subdirectory`, `uri`

**Extract attributes** (0): (none)

**Missing from inject** (9): `agent_arns`, `kerberos_keytab`, `kerberos_keytab_base64`, `kerberos_krb5_conf`, `kerberos_krb5_conf_base64`, `name_node`, `qop_configuration`, `tags`, `tags_all`

**Missing from extract** (18): `agent_arns`, `arn`, `authentication_type`, `block_size`, `kerberos_keytab`, `kerberos_keytab_base64`, `kerberos_krb5_conf`, `kerberos_krb5_conf_base64`, `kerberos_principal`, `kms_key_provider_uri`, `name_node`, `qop_configuration`, `replication_factor`, `simple_user`, `subdirectory`, `tags`, `tags_all`, `uri`

### `aws_datasync_location_nfs`

**Source:** `crates/winterbaume-terraform/src/converters/datasync.rs`

**Inject attributes** (5): `arn`, `region`, `server_hostname`, `subdirectory`, `uri`

**Extract attributes** (0): (none)

**Missing from inject** (4): `mount_options`, `on_prem_config`, `tags`, `tags_all`

**Missing from extract** (8): `arn`, `mount_options`, `on_prem_config`, `server_hostname`, `subdirectory`, `tags`, `tags_all`, `uri`

### `aws_datasync_location_object_storage`

**Source:** `crates/winterbaume-terraform/src/converters/datasync.rs`

**Inject attributes** (10): `access_key`, `arn`, `bucket_name`, `region`, `server_certificate`, `server_hostname`, `server_port`, `server_protocol`, `subdirectory`, `uri`

**Extract attributes** (0): (none)

**Missing from inject** (4): `agent_arns`, `secret_key`, `tags`, `tags_all`

**Missing from extract** (13): `access_key`, `agent_arns`, `arn`, `bucket_name`, `secret_key`, `server_certificate`, `server_hostname`, `server_port`, `server_protocol`, `subdirectory`, `tags`, `tags_all`, `uri`

### `aws_datasync_location_s3`

**Source:** `crates/winterbaume-terraform/src/converters/datasync.rs`

**Inject attributes** (6): `arn`, `region`, `s3_bucket_arn`, `s3_config`, `subdirectory`, `uri`

**Extract attributes** (7): `agent_arns`, `arn`, `creation_time`, `id`, `s3_config`, `tags_all`, `uri`

**Missing from inject** (4): `agent_arns`, `s3_storage_class`, `tags`, `tags_all`

**Missing from extract** (4): `s3_bucket_arn`, `s3_storage_class`, `subdirectory`, `tags`

### `aws_datasync_location_smb`

**Source:** `crates/winterbaume-terraform/src/converters/datasync.rs`

**Inject attributes** (7): `arn`, `domain`, `region`, `server_hostname`, `subdirectory`, `uri`, `user`

**Extract attributes** (0): (none)

**Missing from inject** (5): `agent_arns`, `mount_options`, `password`, `tags`, `tags_all`

**Missing from extract** (11): `agent_arns`, `arn`, `domain`, `mount_options`, `password`, `server_hostname`, `subdirectory`, `tags`, `tags_all`, `uri`, `user`

### `aws_datasync_task`

**Source:** `crates/winterbaume-terraform/src/converters/datasync.rs`

**Inject attributes** (11): `arn`, `cloudwatch_log_group_arn`, `destination_location_arn`, `excludes`, `includes`, `name`, `region`, `schedule`, `source_location_arn`, `status`, `task_report_config`

**Extract attributes** (14): `arn`, `cloudwatch_log_group_arn`, `creation_time`, `destination_location_arn`, `excludes`, `id`, `includes`, `name`, `options`, `schedule`, `source_location_arn`, `status`, `tags_all`, `task_report_config`

**Missing from inject** (5): `options`, `tags`, `tags_all`, `task_mode`, `timeouts`

**Missing from extract** (3): `tags`, `task_mode`, `timeouts`

### `aws_dax_cluster`

**Source:** `crates/winterbaume-terraform/src/converters/dax.rs`

**Inject attributes** (11): `arn`, `cluster_endpoint_encryption_type`, `cluster_name`, `description`, `iam_role_arn`, `node_type`, `region`, `replication_factor`, `server_side_encryption`, `tags`, `tags_all`

**Extract attributes** (14): `arn`, `cluster_endpoint_encryption_type`, `cluster_name`, `description`, `enabled`, `iam_role_arn`, `id`, `node_type`, `parameter_group_name`, `replication_factor`, `server_side_encryption`, `status`, `tags`, `tags_all`

**Missing from inject** (11): `availability_zones`, `cluster_address`, `configuration_endpoint`, `maintenance_window`, `nodes`, `notification_topic_arn`, `parameter_group_name`, `port`, `security_group_ids`, `subnet_group_name`, `timeouts`

**Missing from extract** (10): `availability_zones`, `cluster_address`, `configuration_endpoint`, `maintenance_window`, `nodes`, `notification_topic_arn`, `port`, `security_group_ids`, `subnet_group_name`, `timeouts`

### `aws_dax_parameter_group`

**Source:** `crates/winterbaume-terraform/src/converters/dax.rs`

**Inject attributes** (3): `description`, `name`, `region`

**Extract attributes** (3): `description`, `id`, `name`

**Missing from inject** (1): `parameters`

**Missing from extract** (1): `parameters`

### `aws_dax_subnet_group`

**Source:** `crates/winterbaume-terraform/src/converters/dax.rs`

**Inject attributes** (5): `description`, `name`, `region`, `subnet_ids`, `vpc_id`

**Extract attributes** (5): `description`, `id`, `name`, `subnet_ids`, `vpc_id`

### `aws_device`

**Source:** `crates/winterbaume-terraform/src/converters/directconnect.rs`

**Inject attributes** (0): (none)

**Extract attributes** (11): `bandwidth`, `connection_state`, `has_logical_redundancy`, `id`, `location`, `name`, `owner_account_id`, `partner_name`, `tags`, `tags_all`, `vlan`

**Note:** Resource type not found in Terraform AWS provider schema.

### `aws_dx_bgp_peer`

**Source:** `crates/winterbaume-terraform/src/converters/directconnect.rs`

**Inject attributes** (8): `address_family`, `amazon_address`, `bgp_asn`, `bgp_auth_key`, `bgp_peer_id`, `bgp_status`, `customer_address`, `virtual_interface_id`

**Extract attributes** (0): (none)

**Missing from inject** (2): `aws_device`, `timeouts`

**Missing from extract** (10): `address_family`, `amazon_address`, `aws_device`, `bgp_asn`, `bgp_auth_key`, `bgp_peer_id`, `bgp_status`, `customer_address`, `timeouts`, `virtual_interface_id`

### `aws_dx_connection`

**Source:** `crates/winterbaume-terraform/src/converters/directconnect.rs`

**Inject attributes** (4): `connection_type`, `description`, `name`, `region`

**Extract attributes** (12): `aws_device`, `bandwidth`, `connection_state`, `has_logical_redundancy`, `id`, `location`, `name`, `owner_account_id`, `partner_name`, `tags`, `tags_all`, `vlan`

**Missing from inject** (17): `arn`, `aws_device`, `bandwidth`, `encryption_mode`, `has_logical_redundancy`, `jumbo_frame_capable`, `location`, `macsec_capable`, `owner_account_id`, `partner_name`, `port_encryption_status`, `provider_name`, `request_macsec`, `skip_destroy`, `tags`, `tags_all`, `vlan_id`

**Missing from extract** (9): `arn`, `encryption_mode`, `jumbo_frame_capable`, `macsec_capable`, `port_encryption_status`, `provider_name`, `request_macsec`, `skip_destroy`, `vlan_id`

### `aws_dx_connection_association`

**Source:** `crates/winterbaume-terraform/src/converters/directconnect.rs`

**Inject attributes** (3): `connection_id`, `lag_id`, `region`

**Extract attributes** (0): (none)

**Missing from extract** (2): `connection_id`, `lag_id`

### `aws_dx_connection_confirmation`

**Source:** `crates/winterbaume-terraform/src/converters/directconnect.rs`

**Inject attributes** (2): `connection_id`, `region`

**Extract attributes** (0): (none)

**Missing from extract** (1): `connection_id`

### `aws_dx_gateway`

**Source:** `crates/winterbaume-terraform/src/converters/directconnect.rs`

**Inject attributes** (4): `amazon_side_asn`, `name`, `owner_account_id`, `tags`

**Extract attributes** (0): (none)

**Missing from inject** (2): `arn`, `timeouts`

**Missing from extract** (5): `amazon_side_asn`, `arn`, `name`, `owner_account_id`, `timeouts`

### `aws_dx_gateway_association`

**Source:** `crates/winterbaume-terraform/src/converters/directconnect.rs`

**Inject attributes** (9): `associated_gateway_id`, `associated_gateway_owner_account_id`, `associated_gateway_type`, `dx_gateway_association_id`, `dx_gateway_id`, `dx_gateway_owner_account_id`, `proposal_id`, `tags`, `vpn_gateway_id`

**Extract attributes** (0): (none)

**Missing from inject** (2): `allowed_prefixes`, `timeouts`

**Missing from extract** (10): `allowed_prefixes`, `associated_gateway_id`, `associated_gateway_owner_account_id`, `associated_gateway_type`, `dx_gateway_association_id`, `dx_gateway_id`, `dx_gateway_owner_account_id`, `proposal_id`, `timeouts`, `vpn_gateway_id`

### `aws_dx_gateway_association_proposal`

**Source:** `crates/winterbaume-terraform/src/converters/directconnect.rs`

**Inject attributes** (5): `associated_gateway_id`, `associated_gateway_owner_account_id`, `associated_gateway_type`, `dx_gateway_id`, `dx_gateway_owner_account_id`

**Extract attributes** (0): (none)

**Missing from inject** (1): `allowed_prefixes`

**Missing from extract** (6): `allowed_prefixes`, `associated_gateway_id`, `associated_gateway_owner_account_id`, `associated_gateway_type`, `dx_gateway_id`, `dx_gateway_owner_account_id`

### `aws_dx_hosted_connection`

**Source:** `crates/winterbaume-terraform/src/converters/directconnect.rs`

**Inject attributes** (13): `aws_device`, `bandwidth`, `connection_id`, `connection_state`, `has_logical_redundancy`, `jumbo_frame_capable`, `loa_issue_time`, `location`, `name`, `owner_account_id`, `partner_name`, `region`, `vlan`

**Extract attributes** (0): (none)

**Missing from inject** (3): `lag_id`, `provider_name`, `state`

**Missing from extract** (15): `aws_device`, `bandwidth`, `connection_id`, `has_logical_redundancy`, `jumbo_frame_capable`, `lag_id`, `loa_issue_time`, `location`, `name`, `owner_account_id`, `partner_name`, `provider_name`, `region`, `state`, `vlan`

### `aws_dx_hosted_private_virtual_interface`

**Source:** `crates/winterbaume-terraform/src/converters/directconnect.rs`

**Inject attributes** (13): `address_family`, `amazon_address`, `amazon_side_asn`, `aws_device`, `bgp_asn`, `bgp_auth_key`, `connection_id`, `customer_address`, `jumbo_frame_capable`, `mtu`, `name`, `owner_account_id`, `vlan`

**Extract attributes** (0): (none)

**Missing from inject** (2): `arn`, `timeouts`

**Missing from extract** (15): `address_family`, `amazon_address`, `amazon_side_asn`, `arn`, `aws_device`, `bgp_asn`, `bgp_auth_key`, `connection_id`, `customer_address`, `jumbo_frame_capable`, `mtu`, `name`, `owner_account_id`, `timeouts`, `vlan`

### `aws_dx_hosted_private_virtual_interface_accepter`

**Source:** `crates/winterbaume-terraform/src/converters/directconnect.rs`

**Inject attributes** (4): `dx_gateway_id`, `tags`, `virtual_interface_id`, `vpn_gateway_id`

**Extract attributes** (0): (none)

**Missing from inject** (3): `arn`, `tags_all`, `timeouts`

**Missing from extract** (7): `arn`, `dx_gateway_id`, `tags`, `tags_all`, `timeouts`, `virtual_interface_id`, `vpn_gateway_id`

### `aws_dx_hosted_public_virtual_interface`

**Source:** `crates/winterbaume-terraform/src/converters/directconnect.rs`

**Inject attributes** (11): `address_family`, `amazon_address`, `amazon_side_asn`, `aws_device`, `bgp_asn`, `bgp_auth_key`, `connection_id`, `customer_address`, `name`, `owner_account_id`, `vlan`

**Extract attributes** (0): (none)

**Missing from inject** (3): `arn`, `route_filter_prefixes`, `timeouts`

**Missing from extract** (14): `address_family`, `amazon_address`, `amazon_side_asn`, `arn`, `aws_device`, `bgp_asn`, `bgp_auth_key`, `connection_id`, `customer_address`, `name`, `owner_account_id`, `route_filter_prefixes`, `timeouts`, `vlan`

### `aws_dx_hosted_public_virtual_interface_accepter`

**Source:** `crates/winterbaume-terraform/src/converters/directconnect.rs`

**Inject attributes** (2): `tags`, `virtual_interface_id`

**Extract attributes** (0): (none)

**Missing from inject** (3): `arn`, `tags_all`, `timeouts`

**Missing from extract** (5): `arn`, `tags`, `tags_all`, `timeouts`, `virtual_interface_id`

### `aws_dx_hosted_transit_virtual_interface`

**Source:** `crates/winterbaume-terraform/src/converters/directconnect.rs`

**Inject attributes** (13): `address_family`, `amazon_address`, `amazon_side_asn`, `aws_device`, `bgp_asn`, `bgp_auth_key`, `connection_id`, `customer_address`, `jumbo_frame_capable`, `mtu`, `name`, `owner_account_id`, `vlan`

**Extract attributes** (0): (none)

**Missing from inject** (2): `arn`, `timeouts`

**Missing from extract** (15): `address_family`, `amazon_address`, `amazon_side_asn`, `arn`, `aws_device`, `bgp_asn`, `bgp_auth_key`, `connection_id`, `customer_address`, `jumbo_frame_capable`, `mtu`, `name`, `owner_account_id`, `timeouts`, `vlan`

### `aws_dx_hosted_transit_virtual_interface_accepter`

**Source:** `crates/winterbaume-terraform/src/converters/directconnect.rs`

**Inject attributes** (3): `dx_gateway_id`, `tags`, `virtual_interface_id`

**Extract attributes** (0): (none)

**Missing from inject** (3): `arn`, `tags_all`, `timeouts`

**Missing from extract** (6): `arn`, `dx_gateway_id`, `tags`, `tags_all`, `timeouts`, `virtual_interface_id`

### `aws_dx_lag`

**Source:** `crates/winterbaume-terraform/src/converters/directconnect.rs`

**Inject attributes** (10): `connection_id`, `connections_bandwidth`, `force_destroy`, `has_logical_redundancy`, `jumbo_frame_capable`, `location`, `name`, `owner_account_id`, `provider_name`, `tags`

**Extract attributes** (0): (none)

**Missing from inject** (2): `arn`, `tags_all`

**Missing from extract** (12): `arn`, `connection_id`, `connections_bandwidth`, `force_destroy`, `has_logical_redundancy`, `jumbo_frame_capable`, `location`, `name`, `owner_account_id`, `provider_name`, `tags`, `tags_all`

### `aws_dx_macsec_key_association`

**Source:** `crates/winterbaume-terraform/src/converters/directconnect.rs`

**Inject attributes** (7): `cak`, `ckn`, `connection_id`, `region`, `secret_arn`, `start_on`, `state`

**Extract attributes** (0): (none)

**Missing from extract** (6): `cak`, `ckn`, `connection_id`, `secret_arn`, `start_on`, `state`

### `aws_dx_private_virtual_interface`

**Source:** `crates/winterbaume-terraform/src/converters/directconnect.rs`

**Inject attributes** (16): `address_family`, `amazon_address`, `amazon_side_asn`, `aws_device`, `bgp_asn`, `bgp_auth_key`, `connection_id`, `customer_address`, `dx_gateway_id`, `jumbo_frame_capable`, `mtu`, `name`, `sitelink_enabled`, `tags`, `vlan`, `vpn_gateway_id`

**Extract attributes** (0): (none)

**Missing from inject** (3): `arn`, `tags_all`, `timeouts`

**Missing from extract** (19): `address_family`, `amazon_address`, `amazon_side_asn`, `arn`, `aws_device`, `bgp_asn`, `bgp_auth_key`, `connection_id`, `customer_address`, `dx_gateway_id`, `jumbo_frame_capable`, `mtu`, `name`, `sitelink_enabled`, `tags`, `tags_all`, `timeouts`, `vlan`, `vpn_gateway_id`

### `aws_dx_public_virtual_interface`

**Source:** `crates/winterbaume-terraform/src/converters/directconnect.rs`

**Inject attributes** (11): `address_family`, `amazon_address`, `amazon_side_asn`, `aws_device`, `bgp_asn`, `bgp_auth_key`, `connection_id`, `customer_address`, `name`, `tags`, `vlan`

**Extract attributes** (0): (none)

**Missing from inject** (4): `arn`, `route_filter_prefixes`, `tags_all`, `timeouts`

**Missing from extract** (15): `address_family`, `amazon_address`, `amazon_side_asn`, `arn`, `aws_device`, `bgp_asn`, `bgp_auth_key`, `connection_id`, `customer_address`, `name`, `route_filter_prefixes`, `tags`, `tags_all`, `timeouts`, `vlan`

### `aws_dx_transit_virtual_interface`

**Source:** `crates/winterbaume-terraform/src/converters/directconnect.rs`

**Inject attributes** (15): `address_family`, `amazon_address`, `amazon_side_asn`, `aws_device`, `bgp_asn`, `bgp_auth_key`, `connection_id`, `customer_address`, `dx_gateway_id`, `jumbo_frame_capable`, `mtu`, `name`, `sitelink_enabled`, `tags`, `vlan`

**Extract attributes** (0): (none)

**Missing from inject** (3): `arn`, `tags_all`, `timeouts`

**Missing from extract** (18): `address_family`, `amazon_address`, `amazon_side_asn`, `arn`, `aws_device`, `bgp_asn`, `bgp_auth_key`, `connection_id`, `customer_address`, `dx_gateway_id`, `jumbo_frame_capable`, `mtu`, `name`, `sitelink_enabled`, `tags`, `tags_all`, `timeouts`, `vlan`

### `aws_directory_service_conditional_forwarder`

**Source:** `crates/winterbaume-terraform/src/converters/directory.rs`

**Inject attributes** (3): `directory_id`, `region`, `remote_domain_name`

**Extract attributes** (0): (none)

**Missing from inject** (1): `dns_ips`

**Missing from extract** (3): `directory_id`, `dns_ips`, `remote_domain_name`

### `aws_directory_service_directory`

**Source:** `crates/winterbaume-terraform/src/converters/directory.rs`

**Inject attributes** (16): `access_url`, `alias`, `connect_settings`, `description`, `directory_id`, `dns_ip_addresses`, `enable_sso`, `launch_time`, `name`, `region`, `short_name`, `size`, `stage`, `stage_last_updated_date_time`, `type`, `vpc_settings`

**Extract attributes** (23): `access_url`, `alias`, `availability_zones`, `connect_ips`, `connect_settings`, `customer_dns_ips`, `customer_username`, `description`, `directory_id`, `dns_ip_addresses`, `enable_sso`, `id`, `launch_time`, `name`, `security_group_id`, `short_name`, `size`, `stage`, `stage_last_updated_date_time`, `subnet_ids`, `type`, `vpc_id`, `vpc_settings`

**Missing from inject** (7): `desired_number_of_domain_controllers`, `edition`, `password`, `security_group_id`, `tags`, `tags_all`, `timeouts`

**Missing from extract** (6): `desired_number_of_domain_controllers`, `edition`, `password`, `tags`, `tags_all`, `timeouts`

### `aws_directory_service_log_subscription`

**Source:** `crates/winterbaume-terraform/src/converters/directory.rs`

**Inject attributes** (3): `directory_id`, `log_group_name`, `region`

**Extract attributes** (0): (none)

**Missing from extract** (2): `directory_id`, `log_group_name`

### `aws_directory_service_radius_settings`

**Source:** `crates/winterbaume-terraform/src/converters/directory.rs`

**Inject attributes** (9): `authentication_protocol`, `directory_id`, `display_label`, `radius_port`, `radius_retries`, `radius_timeout`, `region`, `shared_secret`, `use_same_username`

**Extract attributes** (0): (none)

**Missing from inject** (2): `radius_servers`, `timeouts`

**Missing from extract** (10): `authentication_protocol`, `directory_id`, `display_label`, `radius_port`, `radius_retries`, `radius_servers`, `radius_timeout`, `shared_secret`, `timeouts`, `use_same_username`

### `aws_directory_service_region`

**Source:** `crates/winterbaume-terraform/src/converters/directory.rs`

**Inject attributes** (5): `desired_number_of_domain_controllers`, `directory_id`, `region`, `region_name`, `tags`

**Extract attributes** (0): (none)

**Missing from inject** (3): `tags_all`, `timeouts`, `vpc_settings`

**Missing from extract** (7): `desired_number_of_domain_controllers`, `directory_id`, `region_name`, `tags`, `tags_all`, `timeouts`, `vpc_settings`

### `aws_directory_service_shared_directory`

**Source:** `crates/winterbaume-terraform/src/converters/directory.rs`

**Inject attributes** (5): `directory_id`, `method`, `notes`, `region`, `shared_directory_id`

**Extract attributes** (0): (none)

**Missing from inject** (2): `target`, `timeouts`

**Missing from extract** (6): `directory_id`, `method`, `notes`, `shared_directory_id`, `target`, `timeouts`

### `aws_directory_service_shared_directory_accepter`

**Source:** `crates/winterbaume-terraform/src/converters/directory.rs`

**Inject attributes** (6): `method`, `notes`, `owner_account_id`, `owner_directory_id`, `region`, `shared_directory_id`

**Extract attributes** (0): (none)

**Missing from inject** (1): `timeouts`

**Missing from extract** (6): `method`, `notes`, `owner_account_id`, `owner_directory_id`, `shared_directory_id`, `timeouts`

### `aws_directory_service_trust`

**Source:** `crates/winterbaume-terraform/src/converters/directory.rs`

**Inject attributes** (13): `created_date_time`, `delete_associated_conditional_forwarder`, `directory_id`, `last_updated_date_time`, `region`, `remote_domain_name`, `selective_auth`, `state_last_updated_date_time`, `trust_direction`, `trust_password`, `trust_state`, `trust_state_reason`, `trust_type`

**Extract attributes** (0): (none)

**Missing from inject** (1): `conditional_forwarder_ip_addrs`

**Missing from extract** (13): `conditional_forwarder_ip_addrs`, `created_date_time`, `delete_associated_conditional_forwarder`, `directory_id`, `last_updated_date_time`, `remote_domain_name`, `selective_auth`, `state_last_updated_date_time`, `trust_direction`, `trust_password`, `trust_state`, `trust_state_reason`, `trust_type`

### `aws_dms_certificate`

**Source:** `crates/winterbaume-terraform/src/converters/dms.rs`

**Inject attributes** (0): (none)

**Extract attributes** (0): (none)

**Missing from inject** (6): `certificate_arn`, `certificate_id`, `certificate_pem`, `certificate_wallet`, `tags`, `tags_all`

**Missing from extract** (6): `certificate_arn`, `certificate_id`, `certificate_pem`, `certificate_wallet`, `tags`, `tags_all`

### `aws_dms_endpoint`

**Source:** `crates/winterbaume-terraform/src/converters/dms.rs`

**Inject attributes** (14): `arn`, `creation_time`, `elasticsearch_settings`, `endpoint_config_name`, `endpoint_status`, `kafka_settings`, `kinesis_settings`, `last_modified_time`, `mongodb_settings`, `name`, `port`, `redis_settings`, `region`, `s3_settings`

**Extract attributes** (20): `database_name`, `elasticsearch_settings`, `endpoint_arn`, `endpoint_id`, `endpoint_type`, `engine_name`, `extra_connection_attributes`, `id`, `kafka_settings`, `kinesis_settings`, `mongodb_settings`, `port`, `redis_settings`, `s3_settings`, `server_name`, `ssl_mode`, `status`, `tags`, `tags_all`, `username`

**Missing from inject** (21): `certificate_arn`, `database_name`, `endpoint_arn`, `endpoint_id`, `endpoint_type`, `engine_name`, `extra_connection_attributes`, `kms_key_arn`, `password`, `pause_replication_tasks`, `postgres_settings`, `redshift_settings`, `secrets_manager_access_role_arn`, `secrets_manager_arn`, `server_name`, `service_access_role`, `ssl_mode`, `tags`, `tags_all`, `timeouts`, `username`

**Missing from extract** (10): `certificate_arn`, `kms_key_arn`, `password`, `pause_replication_tasks`, `postgres_settings`, `redshift_settings`, `secrets_manager_access_role_arn`, `secrets_manager_arn`, `service_access_role`, `timeouts`

### `aws_dms_event_subscription`

**Source:** `crates/winterbaume-terraform/src/converters/dms.rs`

**Inject attributes** (0): (none)

**Extract attributes** (0): (none)

**Missing from inject** (10): `arn`, `enabled`, `event_categories`, `name`, `sns_topic_arn`, `source_ids`, `source_type`, `tags`, `tags_all`, `timeouts`

**Missing from extract** (10): `arn`, `enabled`, `event_categories`, `name`, `sns_topic_arn`, `source_ids`, `source_type`, `tags`, `tags_all`, `timeouts`

### `aws_dms_replication_config`

**Source:** `crates/winterbaume-terraform/src/converters/dms.rs`

**Inject attributes** (0): (none)

**Extract attributes** (0): (none)

**Missing from inject** (14): `arn`, `compute_config`, `replication_config_identifier`, `replication_settings`, `replication_type`, `resource_identifier`, `source_endpoint_arn`, `start_replication`, `supplemental_settings`, `table_mappings`, `tags`, `tags_all`, `target_endpoint_arn`, `timeouts`

**Missing from extract** (14): `arn`, `compute_config`, `replication_config_identifier`, `replication_settings`, `replication_type`, `resource_identifier`, `source_endpoint_arn`, `start_replication`, `supplemental_settings`, `table_mappings`, `tags`, `tags_all`, `target_endpoint_arn`, `timeouts`

### `aws_dms_replication_instance`

**Source:** `crates/winterbaume-terraform/src/converters/dms.rs`

**Inject attributes** (12): `allocated_storage`, `availability_zone`, `engine_version`, `multi_az`, `publicly_accessible`, `region`, `replication_instance_arn`, `replication_instance_class`, `replication_instance_create_time`, `replication_instance_id`, `status`, `tags`

**Extract attributes** (14): `allocated_storage`, `availability_zone`, `engine_version`, `id`, `multi_az`, `publicly_accessible`, `replication_instance_arn`, `replication_instance_class`, `replication_instance_create_time`, `replication_instance_id`, `replication_instance_private_ips`, `status`, `tags`, `tags_all`

**Missing from inject** (12): `allow_major_version_upgrade`, `apply_immediately`, `auto_minor_version_upgrade`, `kms_key_arn`, `network_type`, `preferred_maintenance_window`, `replication_instance_private_ips`, `replication_instance_public_ips`, `replication_subnet_group_id`, `tags_all`, `timeouts`, `vpc_security_group_ids`

**Missing from extract** (10): `allow_major_version_upgrade`, `apply_immediately`, `auto_minor_version_upgrade`, `kms_key_arn`, `network_type`, `preferred_maintenance_window`, `replication_instance_public_ips`, `replication_subnet_group_id`, `timeouts`, `vpc_security_group_ids`

### `aws_dms_replication_subnet_group`

**Source:** `crates/winterbaume-terraform/src/converters/dms.rs`

**Inject attributes** (0): (none)

**Extract attributes** (0): (none)

**Missing from inject** (7): `replication_subnet_group_arn`, `replication_subnet_group_description`, `replication_subnet_group_id`, `subnet_ids`, `tags`, `tags_all`, `vpc_id`

**Missing from extract** (7): `replication_subnet_group_arn`, `replication_subnet_group_description`, `replication_subnet_group_id`, `subnet_ids`, `tags`, `tags_all`, `vpc_id`

### `aws_dms_replication_task`

**Source:** `crates/winterbaume-terraform/src/converters/dms.rs`

**Inject attributes** (13): `migration_type`, `region`, `replication_instance_arn`, `replication_task_arn`, `replication_task_creation_date`, `replication_task_id`, `replication_task_settings`, `replication_task_start_date`, `source_endpoint_arn`, `status`, `table_mappings`, `tags`, `target_endpoint_arn`

**Extract attributes** (13): `id`, `migration_type`, `replication_instance_arn`, `replication_task_arn`, `replication_task_creation_date`, `replication_task_id`, `replication_task_settings`, `replication_task_start_date`, `source_endpoint_arn`, `status`, `table_mappings`, `tags`, `target_endpoint_arn`

**Missing from inject** (5): `cdc_start_position`, `cdc_start_time`, `resource_identifier`, `start_replication_task`, `tags_all`

**Missing from extract** (5): `cdc_start_position`, `cdc_start_time`, `resource_identifier`, `start_replication_task`, `tags_all`

### `aws_dms_s3_endpoint`

**Source:** `crates/winterbaume-terraform/src/converters/dms.rs`

**Inject attributes** (0): (none)

**Extract attributes** (0): (none)

**Missing from inject** (54): `add_column_name`, `add_trailing_padding_character`, `bucket_folder`, `bucket_name`, `canned_acl_for_objects`, `cdc_inserts_and_updates`, `cdc_inserts_only`, `cdc_max_batch_interval`, `cdc_min_file_size`, `cdc_path`, `certificate_arn`, `compression_type`, `csv_delimiter`, `csv_no_sup_value`, `csv_null_value`, `csv_row_delimiter`, `data_format`, `data_page_size`, `date_partition_delimiter`, `date_partition_enabled`, `date_partition_sequence`, `date_partition_timezone`, `detach_target_on_lob_lookup_failure_parquet`, `dict_page_size_limit`, `enable_statistics`, `encoding_type`, `encryption_mode`, `endpoint_arn`, `endpoint_id`, `endpoint_type`, `engine_display_name`, `expected_bucket_owner`, `external_id`, `external_table_definition`, `glue_catalog_generation`, `ignore_header_rows`, `include_op_for_full_load`, `kms_key_arn`, `max_file_size`, `parquet_timestamp_in_millisecond`, `parquet_version`, `preserve_transactions`, `rfc_4180`, `row_group_length`, `server_side_encryption_kms_key_id`, `service_access_role_arn`, `ssl_mode`, `status`, `tags`, `tags_all`, `timeouts`, `timestamp_column_name`, `use_csv_no_sup_value`, `use_task_start_time_for_full_load_timestamp`

**Missing from extract** (54): `add_column_name`, `add_trailing_padding_character`, `bucket_folder`, `bucket_name`, `canned_acl_for_objects`, `cdc_inserts_and_updates`, `cdc_inserts_only`, `cdc_max_batch_interval`, `cdc_min_file_size`, `cdc_path`, `certificate_arn`, `compression_type`, `csv_delimiter`, `csv_no_sup_value`, `csv_null_value`, `csv_row_delimiter`, `data_format`, `data_page_size`, `date_partition_delimiter`, `date_partition_enabled`, `date_partition_sequence`, `date_partition_timezone`, `detach_target_on_lob_lookup_failure_parquet`, `dict_page_size_limit`, `enable_statistics`, `encoding_type`, `encryption_mode`, `endpoint_arn`, `endpoint_id`, `endpoint_type`, `engine_display_name`, `expected_bucket_owner`, `external_id`, `external_table_definition`, `glue_catalog_generation`, `ignore_header_rows`, `include_op_for_full_load`, `kms_key_arn`, `max_file_size`, `parquet_timestamp_in_millisecond`, `parquet_version`, `preserve_transactions`, `rfc_4180`, `row_group_length`, `server_side_encryption_kms_key_id`, `service_access_role_arn`, `ssl_mode`, `status`, `tags`, `tags_all`, `timeouts`, `timestamp_column_name`, `use_csv_no_sup_value`, `use_task_start_time_for_full_load_timestamp`

### `aws_dsql_cluster`

**Source:** `crates/winterbaume-terraform/src/converters/dsql.rs`

**Inject attributes** (17): `acl_name`, `arn`, `auto_minor_version_upgrade`, `description`, `engine`, `engine_version`, `maintenance_window`, `name`, `node_type`, `num_replicas_per_shard`, `num_shards`, `parameter_group_name`, `region`, `snapshot_retention_limit`, `snapshot_window`, `subnet_group_name`, `tls_enabled`

**Extract attributes** (8): `arn`, `deletion_protection_enabled`, `endpoint`, `id`, `tags`, `tags_all`, `vpc_endpoint_service_name`, `witness_region`

**Missing from inject** (9): `deletion_protection_enabled`, `encryption_details`, `identifier`, `kms_encryption_key`, `multi_region_properties`, `tags`, `tags_all`, `timeouts`, `vpc_endpoint_service_name`

**Missing from extract** (5): `encryption_details`, `identifier`, `kms_encryption_key`, `multi_region_properties`, `timeouts`

### `aws_dynamodb_contributor_insights`

**Source:** `crates/winterbaume-terraform/src/converters/dynamodb.rs`

**Inject attributes** (3): `index_name`, `region`, `table_name`

**Extract attributes** (3): `id`, `index_name`, `table_name`

**Missing from inject** (1): `timeouts`

**Missing from extract** (1): `timeouts`

### `aws_dynamodb_global_table`

**Source:** `crates/winterbaume-terraform/src/converters/dynamodb.rs`

**Inject attributes** (4): `arn`, `name`, `region`, `replica`

**Extract attributes** (5): `arn`, `id`, `name`, `region_name`, `replica`

**Missing from inject** (1): `timeouts`

**Missing from extract** (1): `timeouts`

### `aws_dynamodb_kinesis_streaming_destination`

**Source:** `crates/winterbaume-terraform/src/converters/dynamodb.rs`

**Inject attributes** (4): `approximate_creation_date_time_precision`, `region`, `stream_arn`, `table_name`

**Extract attributes** (4): `approximate_creation_date_time_precision`, `id`, `stream_arn`, `table_name`

### `aws_dynamodb_resource_policy`

**Source:** `crates/winterbaume-terraform/src/converters/dynamodb.rs`

**Inject attributes** (2): `policy`, `region`

**Extract attributes** (4): `id`, `policy`, `resource_arn`, `revision_id`

**Missing from inject** (3): `confirm_remove_self_resource_access`, `resource_arn`, `revision_id`

**Missing from extract** (1): `confirm_remove_self_resource_access`

### `aws_dynamodb_table`

**Source:** `crates/winterbaume-terraform/src/converters/dynamodb.rs`

**Inject attributes** (10): `arn`, `attribute`, `database_name`, `import_table`, `on_demand_throughput`, `region`, `replica`, `table_name`, `tags`, `tags_all`

**Extract attributes** (26): `arn`, `attribute`, `attribute_name`, `billing_mode`, `enabled`, `global_secondary_index`, `hash_key`, `id`, `import_table`, `local_secondary_index`, `name`, `on_demand_throughput`, `point_in_time_recovery`, `range_key`, `read_capacity`, `replica`, `server_side_encryption`, `status`, `stream_enabled`, `stream_view_type`, `table_class`, `tags`, `tags_all`, `ttl`, `type`, `write_capacity`

**Missing from inject** (22): `billing_mode`, `deletion_protection_enabled`, `global_secondary_index`, `hash_key`, `local_secondary_index`, `name`, `point_in_time_recovery`, `range_key`, `read_capacity`, `restore_date_time`, `restore_source_name`, `restore_source_table_arn`, `restore_to_latest_time`, `server_side_encryption`, `stream_arn`, `stream_enabled`, `stream_label`, `stream_view_type`, `table_class`, `timeouts`, `ttl`, `write_capacity`

**Missing from extract** (8): `deletion_protection_enabled`, `restore_date_time`, `restore_source_name`, `restore_source_table_arn`, `restore_to_latest_time`, `stream_arn`, `stream_label`, `timeouts`

### `aws_dynamodb_table_export`

**Source:** `crates/winterbaume-terraform/src/converters/dynamodb.rs`

**Inject attributes** (6): `export_format`, `export_type`, `region`, `s3_bucket`, `s3_prefix`, `table_arn`

**Extract attributes** (0): (none)

**Missing from inject** (13): `arn`, `billed_size_in_bytes`, `end_time`, `export_status`, `export_time`, `incremental_export_specification`, `item_count`, `manifest_files_s3_key`, `s3_bucket_owner`, `s3_sse_algorithm`, `s3_sse_kms_key_id`, `start_time`, `timeouts`

**Missing from extract** (18): `arn`, `billed_size_in_bytes`, `end_time`, `export_format`, `export_status`, `export_time`, `export_type`, `incremental_export_specification`, `item_count`, `manifest_files_s3_key`, `s3_bucket`, `s3_bucket_owner`, `s3_prefix`, `s3_sse_algorithm`, `s3_sse_kms_key_id`, `start_time`, `table_arn`, `timeouts`

### `aws_dynamodb_table_item`

**Source:** `crates/winterbaume-terraform/src/converters/dynamodb.rs`

**Inject attributes** (5): `hash_key`, `item`, `range_key`, `region`, `table_name`

**Extract attributes** (5): `hash_key`, `id`, `item`, `range_key`, `table_name`

### `aws_dynamodb_table_replica`

**Source:** `crates/winterbaume-terraform/src/converters/dynamodb.rs`

**Inject attributes** (5): `global_table_arn`, `kms_key_arn`, `point_in_time_recovery`, `region`, `table_class_override`

**Extract attributes** (0): (none)

**Missing from inject** (5): `arn`, `deletion_protection_enabled`, `tags`, `tags_all`, `timeouts`

**Missing from extract** (9): `arn`, `deletion_protection_enabled`, `global_table_arn`, `kms_key_arn`, `point_in_time_recovery`, `table_class_override`, `tags`, `tags_all`, `timeouts`

### `aws_dynamodb_tag`

**Source:** `crates/winterbaume-terraform/src/converters/dynamodb.rs`

**Inject attributes** (4): `key`, `region`, `resource_arn`, `value`

**Extract attributes** (0): (none)

**Missing from extract** (3): `key`, `resource_arn`, `value`

### `aws_ebs_snapshot`

**Source:** `crates/winterbaume-terraform/src/converters/ebs.rs`

**Inject attributes** (8): `arn`, `block_size`, `cluster_name`, `id`, `kms_key_arn`, `name`, `region`, `source`

**Extract attributes** (12): `block_size`, `data_encryption_key_id`, `description`, `encrypted`, `id`, `owner_alias`, `owner_id`, `snapshot_id`, `status`, `tags`, `tags_all`, `volume_size`

**Missing from inject** (15): `data_encryption_key_id`, `description`, `encrypted`, `kms_key_id`, `outpost_arn`, `owner_alias`, `owner_id`, `permanent_restore`, `storage_tier`, `tags`, `tags_all`, `temporary_restore_days`, `timeouts`, `volume_id`, `volume_size`

**Missing from extract** (8): `arn`, `kms_key_id`, `outpost_arn`, `permanent_restore`, `storage_tier`, `temporary_restore_days`, `timeouts`, `volume_id`

### `aws_ebs_volume`

**Source:** `crates/winterbaume-terraform/src/converters/ebs.rs`

**Inject attributes** (12): `availability_zone`, `encrypted`, `id`, `iops`, `kms_key_id`, `region`, `size`, `snapshot_id`, `tags`, `throughput`, `type`, `volume_id`

**Extract attributes** (13): `availability_zone`, `encrypted`, `id`, `iops`, `kms_key_id`, `size`, `snapshot_id`, `state`, `tags`, `tags_all`, `throughput`, `type`, `volume_id`

**Missing from inject** (7): `arn`, `create_time`, `final_snapshot`, `multi_attach_enabled`, `outpost_arn`, `tags_all`, `timeouts`

**Missing from extract** (6): `arn`, `create_time`, `final_snapshot`, `multi_attach_enabled`, `outpost_arn`, `timeouts`

### `aws_account_id`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (0): (none)

**Extract attributes** (3): `id`, `network_interface_id`, `permission`

**Note:** Resource type not found in Terraform AWS provider schema.

### `aws_ami`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (18): `architecture`, `boot_mode`, `deprecation_time`, `description`, `ena_support`, `image_location`, `imds_support`, `kernel_id`, `name`, `public`, `ramdisk_id`, `region`, `root_device_name`, `sriov_net_support`, `tags`, `tags_all`, `tpm_support`, `virtualization_type`

**Extract attributes** (13): `architecture`, `arn`, `deprecation_time`, `description`, `id`, `name`, `owner_id`, `platform`, `public`, `root_device_name`, `tags`, `tags_all`, `virtualization_type`

**Missing from inject** (15): `arn`, `ebs_block_device`, `ephemeral_block_device`, `hypervisor`, `image_owner_alias`, `image_type`, `last_launched_time`, `manage_ebs_snapshots`, `owner_id`, `platform`, `platform_details`, `root_snapshot_id`, `timeouts`, `uefi_data`, `usage_operation`

**Missing from extract** (20): `boot_mode`, `ebs_block_device`, `ena_support`, `ephemeral_block_device`, `hypervisor`, `image_location`, `image_owner_alias`, `image_type`, `imds_support`, `kernel_id`, `last_launched_time`, `manage_ebs_snapshots`, `platform_details`, `ramdisk_id`, `root_snapshot_id`, `sriov_net_support`, `timeouts`, `tpm_support`, `uefi_data`, `usage_operation`

### `aws_ami_copy`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (10): `deprecation_time`, `description`, `encrypted`, `kms_key_id`, `name`, `region`, `source_ami_id`, `source_ami_region`, `tags`, `tags_all`

**Extract attributes** (0): (none)

**Missing from inject** (28): `architecture`, `arn`, `boot_mode`, `destination_outpost_arn`, `ebs_block_device`, `ena_support`, `ephemeral_block_device`, `hypervisor`, `image_location`, `image_owner_alias`, `image_type`, `imds_support`, `kernel_id`, `last_launched_time`, `manage_ebs_snapshots`, `owner_id`, `platform`, `platform_details`, `public`, `ramdisk_id`, `root_device_name`, `root_snapshot_id`, `sriov_net_support`, `timeouts`, `tpm_support`, `uefi_data`, `usage_operation`, `virtualization_type`

**Missing from extract** (37): `architecture`, `arn`, `boot_mode`, `deprecation_time`, `description`, `destination_outpost_arn`, `ebs_block_device`, `ena_support`, `encrypted`, `ephemeral_block_device`, `hypervisor`, `image_location`, `image_owner_alias`, `image_type`, `imds_support`, `kernel_id`, `kms_key_id`, `last_launched_time`, `manage_ebs_snapshots`, `name`, `owner_id`, `platform`, `platform_details`, `public`, `ramdisk_id`, `root_device_name`, `root_snapshot_id`, `source_ami_id`, `source_ami_region`, `sriov_net_support`, `tags`, `tags_all`, `timeouts`, `tpm_support`, `uefi_data`, `usage_operation`, `virtualization_type`

### `aws_ami_from_instance`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (8): `deprecation_time`, `description`, `name`, `region`, `snapshot_without_reboot`, `source_instance_id`, `tags`, `tags_all`

**Extract attributes** (8): `arn`, `deprecation_time`, `description`, `id`, `name`, `source_instance_id`, `tags`, `tags_all`

**Missing from inject** (27): `architecture`, `arn`, `boot_mode`, `ebs_block_device`, `ena_support`, `ephemeral_block_device`, `hypervisor`, `image_location`, `image_owner_alias`, `image_type`, `imds_support`, `kernel_id`, `last_launched_time`, `manage_ebs_snapshots`, `owner_id`, `platform`, `platform_details`, `public`, `ramdisk_id`, `root_device_name`, `root_snapshot_id`, `sriov_net_support`, `timeouts`, `tpm_support`, `uefi_data`, `usage_operation`, `virtualization_type`

**Missing from extract** (27): `architecture`, `boot_mode`, `ebs_block_device`, `ena_support`, `ephemeral_block_device`, `hypervisor`, `image_location`, `image_owner_alias`, `image_type`, `imds_support`, `kernel_id`, `last_launched_time`, `manage_ebs_snapshots`, `owner_id`, `platform`, `platform_details`, `public`, `ramdisk_id`, `root_device_name`, `root_snapshot_id`, `snapshot_without_reboot`, `sriov_net_support`, `timeouts`, `tpm_support`, `uefi_data`, `usage_operation`, `virtualization_type`

### `aws_ami_launch_permission`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (6): `account_id`, `group`, `image_id`, `organization_arn`, `organizational_unit_arn`, `region`

**Extract attributes** (4): `account_id`, `group`, `id`, `image_id`

**Missing from extract** (2): `organization_arn`, `organizational_unit_arn`

### `aws_customer_gateway`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (6): `bgp_asn`, `ip_address`, `region`, `tags`, `tags_all`, `type`

**Extract attributes** (7): `arn`, `bgp_asn`, `id`, `ip_address`, `tags`, `tags_all`, `type`

**Missing from inject** (4): `arn`, `bgp_asn_extended`, `certificate_arn`, `device_name`

**Missing from extract** (3): `bgp_asn_extended`, `certificate_arn`, `device_name`

### `aws_default_network_acl`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (5): `default_network_acl_id`, `region`, `tags`, `tags_all`, `vpc_id`

**Extract attributes** (0): (none)

**Missing from inject** (5): `arn`, `egress`, `ingress`, `owner_id`, `subnet_ids`

**Missing from extract** (9): `arn`, `default_network_acl_id`, `egress`, `ingress`, `owner_id`, `subnet_ids`, `tags`, `tags_all`, `vpc_id`

### `aws_default_route_table`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (5): `default_route_table_id`, `region`, `tags`, `tags_all`, `vpc_id`

**Extract attributes** (0): (none)

**Missing from inject** (5): `arn`, `owner_id`, `propagating_vgws`, `route`, `timeouts`

**Missing from extract** (9): `arn`, `default_route_table_id`, `owner_id`, `propagating_vgws`, `route`, `tags`, `tags_all`, `timeouts`, `vpc_id`

### `aws_default_security_group`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (4): `region`, `tags`, `tags_all`, `vpc_id`

**Extract attributes** (0): (none)

**Missing from inject** (8): `arn`, `description`, `egress`, `ingress`, `name`, `name_prefix`, `owner_id`, `revoke_rules_on_delete`

**Missing from extract** (11): `arn`, `description`, `egress`, `ingress`, `name`, `name_prefix`, `owner_id`, `revoke_rules_on_delete`, `tags`, `tags_all`, `vpc_id`

### `aws_default_subnet`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (6): `availability_zone`, `cidr_block`, `region`, `tags`, `tags_all`, `vpc_id`

**Extract attributes** (0): (none)

**Missing from inject** (19): `arn`, `assign_ipv6_address_on_creation`, `availability_zone_id`, `customer_owned_ipv4_pool`, `enable_dns64`, `enable_lni_at_device_index`, `enable_resource_name_dns_a_record_on_launch`, `enable_resource_name_dns_aaaa_record_on_launch`, `existing_default_subnet`, `force_destroy`, `ipv6_cidr_block`, `ipv6_cidr_block_association_id`, `ipv6_native`, `map_customer_owned_ip_on_launch`, `map_public_ip_on_launch`, `outpost_arn`, `owner_id`, `private_dns_hostname_type_on_launch`, `timeouts`

**Missing from extract** (24): `arn`, `assign_ipv6_address_on_creation`, `availability_zone`, `availability_zone_id`, `cidr_block`, `customer_owned_ipv4_pool`, `enable_dns64`, `enable_lni_at_device_index`, `enable_resource_name_dns_a_record_on_launch`, `enable_resource_name_dns_aaaa_record_on_launch`, `existing_default_subnet`, `force_destroy`, `ipv6_cidr_block`, `ipv6_cidr_block_association_id`, `ipv6_native`, `map_customer_owned_ip_on_launch`, `map_public_ip_on_launch`, `outpost_arn`, `owner_id`, `private_dns_hostname_type_on_launch`, `tags`, `tags_all`, `timeouts`, `vpc_id`

### `aws_default_vpc`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (6): `cidr_block`, `enable_dns_hostnames`, `enable_dns_support`, `region`, `tags`, `tags_all`

**Extract attributes** (0): (none)

**Missing from inject** (17): `arn`, `assign_generated_ipv6_cidr_block`, `default_network_acl_id`, `default_route_table_id`, `default_security_group_id`, `dhcp_options_id`, `enable_network_address_usage_metrics`, `existing_default_vpc`, `force_destroy`, `instance_tenancy`, `ipv6_association_id`, `ipv6_cidr_block`, `ipv6_cidr_block_network_border_group`, `ipv6_ipam_pool_id`, `ipv6_netmask_length`, `main_route_table_id`, `owner_id`

**Missing from extract** (22): `arn`, `assign_generated_ipv6_cidr_block`, `cidr_block`, `default_network_acl_id`, `default_route_table_id`, `default_security_group_id`, `dhcp_options_id`, `enable_dns_hostnames`, `enable_dns_support`, `enable_network_address_usage_metrics`, `existing_default_vpc`, `force_destroy`, `instance_tenancy`, `ipv6_association_id`, `ipv6_cidr_block`, `ipv6_cidr_block_network_border_group`, `ipv6_ipam_pool_id`, `ipv6_netmask_length`, `main_route_table_id`, `owner_id`, `tags`, `tags_all`

### `aws_default_vpc_dhcp_options`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (4): `owner_id`, `region`, `tags`, `tags_all`

**Extract attributes** (0): (none)

**Missing from inject** (7): `arn`, `domain_name`, `domain_name_servers`, `ipv6_address_preferred_lease_time`, `netbios_name_servers`, `netbios_node_type`, `ntp_servers`

**Missing from extract** (10): `arn`, `domain_name`, `domain_name_servers`, `ipv6_address_preferred_lease_time`, `netbios_name_servers`, `netbios_node_type`, `ntp_servers`, `owner_id`, `tags`, `tags_all`

### `aws_ebs_default_kms_key`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (2): `key_arn`, `region`

**Extract attributes** (0): (none)

**Missing from extract** (1): `key_arn`

### `aws_ebs_encryption_by_default`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (2): `enabled`, `region`

**Extract attributes** (0): (none)

**Missing from extract** (1): `enabled`

### `aws_ebs_fast_snapshot_restore`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (3): `availability_zone`, `region`, `snapshot_id`

**Extract attributes** (4): `availability_zone`, `id`, `snapshot_id`, `state`

**Missing from inject** (2): `state`, `timeouts`

**Missing from extract** (1): `timeouts`

### `aws_ebs_snapshot_block_public_access`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (2): `region`, `state`

**Extract attributes** (0): (none)

**Missing from extract** (1): `state`

### `aws_ebs_snapshot_copy`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (9): `description`, `encrypted`, `id`, `kms_key_id`, `region`, `source_region`, `source_snapshot_id`, `storage_tier`, `tags`

**Extract attributes** (0): (none)

**Missing from inject** (12): `arn`, `completion_duration_minutes`, `data_encryption_key_id`, `outpost_arn`, `owner_alias`, `owner_id`, `permanent_restore`, `tags_all`, `temporary_restore_days`, `timeouts`, `volume_id`, `volume_size`

**Missing from extract** (19): `arn`, `completion_duration_minutes`, `data_encryption_key_id`, `description`, `encrypted`, `kms_key_id`, `outpost_arn`, `owner_alias`, `owner_id`, `permanent_restore`, `source_region`, `source_snapshot_id`, `storage_tier`, `tags`, `tags_all`, `temporary_restore_days`, `timeouts`, `volume_id`, `volume_size`

### `aws_ebs_snapshot_import`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (10): `description`, `disk_container`, `encrypted`, `id`, `import_task_id`, `kms_key_id`, `region`, `role_name`, `storage_tier`, `tags`

**Extract attributes** (7): `description`, `encrypted`, `id`, `import_task_id`, `kms_key_id`, `tags`, `tags_all`

**Missing from inject** (12): `arn`, `client_data`, `data_encryption_key_id`, `outpost_arn`, `owner_alias`, `owner_id`, `permanent_restore`, `tags_all`, `temporary_restore_days`, `timeouts`, `volume_id`, `volume_size`

**Missing from extract** (14): `arn`, `client_data`, `data_encryption_key_id`, `disk_container`, `outpost_arn`, `owner_alias`, `owner_id`, `permanent_restore`, `role_name`, `storage_tier`, `temporary_restore_days`, `timeouts`, `volume_id`, `volume_size`

### `aws_ec2_availability_zone_group`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (3): `group_name`, `opt_in_status`, `region`

**Extract attributes** (0): (none)

**Missing from extract** (2): `group_name`, `opt_in_status`

### `aws_ec2_capacity_block_reservation`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (15): `availability_zone`, `capacity_block_id`, `capacity_block_offering_id`, `capacity_reservation_arn`, `capacity_reservation_id`, `currency_code`, `end_date`, `instance_count`, `instance_type`, `region`, `start_date`, `tags`, `tags_all`, `tenancy`, `upfront_fee`

**Extract attributes** (15): `availability_zone`, `capacity_block_id`, `capacity_block_offering_id`, `capacity_reservation_arn`, `capacity_reservation_id`, `currency_code`, `end_date`, `id`, `instance_count`, `instance_type`, `start_date`, `tags`, `tags_all`, `tenancy`, `upfront_fee`

**Missing from inject** (9): `arn`, `created_date`, `ebs_optimized`, `end_date_type`, `instance_platform`, `outpost_arn`, `placement_group_arn`, `reservation_type`, `timeouts`

**Missing from extract** (9): `arn`, `created_date`, `ebs_optimized`, `end_date_type`, `instance_platform`, `outpost_arn`, `placement_group_arn`, `reservation_type`, `timeouts`

### `aws_ec2_capacity_reservation`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (19): `arn`, `availability_zone`, `create_date`, `ebs_optimized`, `end_date`, `end_date_type`, `ephemeral_storage`, `instance_count`, `instance_match_criteria`, `instance_platform`, `instance_type`, `outpost_arn`, `owner_id`, `placement_group_arn`, `region`, `start_date`, `tags`, `tags_all`, `tenancy`

**Extract attributes** (18): `arn`, `availability_zone`, `capacity_reservation_arn`, `ebs_optimized`, `end_date`, `end_date_type`, `ephemeral_storage`, `id`, `instance_count`, `instance_match_criteria`, `instance_platform`, `instance_type`, `outpost_arn`, `owner_id`, `placement_group_arn`, `tags`, `tags_all`, `tenancy`

**Missing from inject** (1): `timeouts`

**Missing from extract** (1): `timeouts`

### `aws_ec2_carrier_gateway`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (4): `region`, `tags`, `tags_all`, `vpc_id`

**Extract attributes** (3): `id`, `tags`, `vpc_id`

**Missing from inject** (2): `arn`, `owner_id`

**Missing from extract** (3): `arn`, `owner_id`, `tags_all`

### `aws_ec2_client_vpn_authorization_rule`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (6): `access_group_id`, `authorize_all_groups`, `client_vpn_endpoint_id`, `description`, `region`, `target_network_cidr`

**Extract attributes** (0): (none)

**Missing from inject** (1): `timeouts`

**Missing from extract** (6): `access_group_id`, `authorize_all_groups`, `client_vpn_endpoint_id`, `description`, `target_network_cidr`, `timeouts`

### `aws_ec2_client_vpn_endpoint`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (13): `client_cidr_block`, `description`, `dns_name`, `region`, `self_service_portal`, `server_certificate_arn`, `session_timeout_hours`, `split_tunnel`, `tags`, `tags_all`, `transport_protocol`, `vpc_id`, `vpn_port`

**Extract attributes** (12): `client_cidr_block`, `description`, `dns_name`, `id`, `self_service_portal`, `server_certificate_arn`, `session_timeout_hours`, `split_tunnel`, `tags`, `transport_protocol`, `vpc_id`, `vpn_port`

**Missing from inject** (10): `arn`, `authentication_options`, `client_connect_options`, `client_login_banner_options`, `client_route_enforcement_options`, `connection_log_options`, `disconnect_on_session_timeout`, `dns_servers`, `security_group_ids`, `self_service_portal_url`

**Missing from extract** (11): `arn`, `authentication_options`, `client_connect_options`, `client_login_banner_options`, `client_route_enforcement_options`, `connection_log_options`, `disconnect_on_session_timeout`, `dns_servers`, `security_group_ids`, `self_service_portal_url`, `tags_all`

### `aws_ec2_client_vpn_network_association`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (5): `association_id`, `client_vpn_endpoint_id`, `region`, `subnet_id`, `vpc_id`

**Extract attributes** (0): (none)

**Missing from inject** (1): `timeouts`

**Missing from extract** (5): `association_id`, `client_vpn_endpoint_id`, `subnet_id`, `timeouts`, `vpc_id`

### `aws_ec2_client_vpn_route`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (6): `client_vpn_endpoint_id`, `description`, `destination_cidr_block`, `origin`, `region`, `target_vpc_subnet_id`

**Extract attributes** (0): (none)

**Missing from inject** (2): `timeouts`, `type`

**Missing from extract** (7): `client_vpn_endpoint_id`, `description`, `destination_cidr_block`, `origin`, `target_vpc_subnet_id`, `timeouts`, `type`

### `aws_ec2_default_credit_specification`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (3): `cpu_credits`, `instance_family`, `region`

**Extract attributes** (0): (none)

**Missing from inject** (1): `timeouts`

**Missing from extract** (3): `cpu_credits`, `instance_family`, `timeouts`

### `aws_ec2_fleet`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (8): `context`, `excess_capacity_termination_policy`, `region`, `replace_unhealthy_instances`, `tags`, `tags_all`, `terminate_instances`, `terminate_instances_with_expiration`

**Extract attributes** (4): `context`, `id`, `tags`, `type`

**Missing from inject** (13): `arn`, `fleet_instance_set`, `fleet_state`, `fulfilled_capacity`, `fulfilled_on_demand_capacity`, `launch_template_config`, `on_demand_options`, `spot_options`, `target_capacity_specification`, `timeouts`, `type`, `valid_from`, `valid_until`

**Missing from extract** (17): `arn`, `excess_capacity_termination_policy`, `fleet_instance_set`, `fleet_state`, `fulfilled_capacity`, `fulfilled_on_demand_capacity`, `launch_template_config`, `on_demand_options`, `replace_unhealthy_instances`, `spot_options`, `tags_all`, `target_capacity_specification`, `terminate_instances`, `terminate_instances_with_expiration`, `timeouts`, `valid_from`, `valid_until`

### `aws_ec2_host`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (10): `auto_placement`, `availability_zone`, `host_id`, `host_recovery`, `instance_family`, `instance_type`, `outpost_arn`, `region`, `tags`, `tags_all`

**Extract attributes** (7): `auto_placement`, `availability_zone`, `host_id`, `host_recovery`, `id`, `instance_type`, `tags`

**Missing from inject** (4): `arn`, `asset_id`, `owner_id`, `timeouts`

**Missing from extract** (7): `arn`, `asset_id`, `instance_family`, `outpost_arn`, `owner_id`, `tags_all`, `timeouts`

### `aws_ec2_image_block_public_access`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (2): `region`, `state`

**Extract attributes** (0): (none)

**Missing from inject** (1): `timeouts`

**Missing from extract** (2): `state`, `timeouts`

### `aws_ec2_instance_connect_endpoint`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (11): `arn`, `created_at`, `dns_name`, `fips_dns_name`, `owner_id`, `preserve_client_ip`, `region`, `security_group_ids`, `subnet_id`, `tags`, `tags_all`

**Extract attributes** (13): `arn`, `availability_zone`, `dns_name`, `fips_dns_name`, `id`, `network_interface_ids`, `owner_id`, `preserve_client_ip`, `security_group_ids`, `subnet_id`, `tags`, `tags_all`, `vpc_id`

**Missing from inject** (4): `availability_zone`, `network_interface_ids`, `timeouts`, `vpc_id`

**Missing from extract** (1): `timeouts`

### `aws_ec2_instance_metadata_defaults`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (5): `http_endpoint`, `http_put_response_hop_limit`, `http_tokens`, `instance_metadata_tags`, `region`

**Extract attributes** (0): (none)

**Missing from extract** (4): `http_endpoint`, `http_put_response_hop_limit`, `http_tokens`, `instance_metadata_tags`

### `aws_ec2_instance_state`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (4): `force`, `instance_id`, `region`, `state`

**Extract attributes** (0): (none)

**Missing from inject** (1): `timeouts`

**Missing from extract** (4): `force`, `instance_id`, `state`, `timeouts`

### `aws_ec2_local_gateway_route`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (9): `coip_pool_id`, `destination_cidr_block`, `destination_prefix_list_id`, `local_gateway_route_table_arn`, `local_gateway_route_table_id`, `local_gateway_virtual_interface_group_id`, `network_interface_id`, `region`, `subnet_id`

**Extract attributes** (7): `destination_cidr_block`, `id`, `local_gateway_route_table_id`, `local_gateway_virtual_interface_group_id`, `network_interface_id`, `state`, `type`

### `aws_ec2_local_gateway_route_table_vpc_association`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (7): `local_gateway_id`, `local_gateway_route_table_arn`, `local_gateway_route_table_id`, `region`, `tags`, `tags_all`, `vpc_id`

**Extract attributes** (9): `id`, `local_gateway_id`, `local_gateway_route_table_arn`, `local_gateway_route_table_id`, `owner_id`, `state`, `tags`, `tags_all`, `vpc_id`

### `aws_ec2_managed_prefix_list`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (7): `address_family`, `entry`, `max_entries`, `name`, `region`, `tags`, `tags_all`

**Extract attributes** (8): `address_family`, `cidr`, `description`, `entry`, `id`, `max_entries`, `name`, `tags`

**Missing from inject** (3): `arn`, `owner_id`, `version`

**Missing from extract** (4): `arn`, `owner_id`, `tags_all`, `version`

### `aws_ec2_managed_prefix_list_entry`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (4): `cidr`, `description`, `prefix_list_id`, `region`

**Extract attributes** (0): (none)

**Missing from extract** (3): `cidr`, `description`, `prefix_list_id`

### `aws_ec2_network_insights_analysis`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (7): `arn`, `network_insights_path_id`, `path_found`, `region`, `start_date`, `tags`, `tags_all`

**Extract attributes** (10): `additional_accounts`, `arn`, `filter_in_arns`, `id`, `network_insights_path_id`, `path_found`, `start_date`, `status`, `tags`, `tags_all`

**Missing from inject** (9): `alternate_path_hints`, `explanations`, `filter_in_arns`, `forward_path_components`, `return_path_components`, `status`, `status_message`, `wait_for_completion`, `warning_message`

**Missing from extract** (7): `alternate_path_hints`, `explanations`, `forward_path_components`, `return_path_components`, `status_message`, `wait_for_completion`, `warning_message`

### `aws_ec2_network_insights_path`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (13): `arn`, `created_date`, `destination`, `destination_arn`, `destination_ip`, `destination_port`, `protocol`, `region`, `source`, `source_arn`, `source_ip`, `tags`, `tags_all`

**Extract attributes** (13): `arn`, `created_date`, `destination`, `destination_arn`, `destination_ip`, `destination_port`, `id`, `protocol`, `source`, `source_arn`, `source_ip`, `tags`, `tags_all`

**Missing from inject** (2): `filter_at_destination`, `filter_at_source`

**Missing from extract** (2): `filter_at_destination`, `filter_at_source`

### `aws_ec2_serial_console_access`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (2): `enabled`, `region`

**Extract attributes** (0): (none)

**Missing from extract** (1): `enabled`

### `aws_ec2_subnet_cidr_reservation`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (5): `cidr_block`, `description`, `region`, `reservation_type`, `subnet_id`

**Extract attributes** (0): (none)

**Missing from inject** (1): `owner_id`

**Missing from extract** (5): `cidr_block`, `description`, `owner_id`, `reservation_type`, `subnet_id`

### `aws_ec2_tag`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (4): `key`, `region`, `resource_id`, `value`

**Extract attributes** (0): (none)

**Missing from extract** (3): `key`, `resource_id`, `value`

### `aws_ec2_traffic_mirror_filter`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (4): `description`, `region`, `tags`, `tags_all`

**Extract attributes** (5): `description`, `id`, `network_services`, `tags`, `tags_all`

**Missing from inject** (2): `arn`, `network_services`

**Missing from extract** (1): `arn`

### `aws_ec2_traffic_mirror_filter_rule`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (11): `description`, `destination_cidr_block`, `protocol`, `region`, `rule_action`, `rule_number`, `source_cidr_block`, `tags`, `tags_all`, `traffic_direction`, `traffic_mirror_filter_id`

**Extract attributes** (13): `description`, `destination_cidr_block`, `destination_port_range`, `from_port`, `id`, `protocol`, `rule_action`, `rule_number`, `source_cidr_block`, `source_port_range`, `to_port`, `traffic_direction`, `traffic_mirror_filter_id`

**Missing from inject** (3): `arn`, `destination_port_range`, `source_port_range`

**Missing from extract** (1): `arn`

### `aws_ec2_traffic_mirror_session`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (11): `description`, `network_interface_id`, `owner_id`, `packet_length`, `region`, `session_number`, `tags`, `tags_all`, `traffic_mirror_filter_id`, `traffic_mirror_target_id`, `virtual_network_id`

**Extract attributes** (11): `description`, `id`, `network_interface_id`, `owner_id`, `packet_length`, `session_number`, `tags`, `tags_all`, `traffic_mirror_filter_id`, `traffic_mirror_target_id`, `virtual_network_id`

**Missing from inject** (1): `arn`

**Missing from extract** (1): `arn`

### `aws_ec2_traffic_mirror_target`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (8): `description`, `gateway_load_balancer_endpoint_id`, `network_interface_id`, `network_load_balancer_arn`, `owner_id`, `region`, `tags`, `tags_all`

**Extract attributes** (9): `description`, `gateway_load_balancer_endpoint_id`, `id`, `network_interface_id`, `network_load_balancer_arn`, `owner_id`, `tags`, `tags_all`, `type`

**Missing from inject** (1): `arn`

**Missing from extract** (1): `arn`

### `aws_ec2_transit_gateway`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (16): `amazon_side_asn`, `arn`, `association_default_route_table_id`, `auto_accept_shared_attachments`, `default_route_table_association`, `default_route_table_propagation`, `description`, `dns_support`, `multicast_support`, `owner_id`, `propagation_default_route_table_id`, `region`, `tags`, `tags_all`, `transit_gateway_cidr_blocks`, `vpn_ecmp_support`

**Extract attributes** (10): `amazon_side_asn`, `arn`, `description`, `dns_support`, `id`, `multicast_support`, `owner_id`, `tags`, `tags_all`, `vpn_ecmp_support`

**Missing from inject** (2): `security_group_referencing_support`, `timeouts`

**Missing from extract** (8): `association_default_route_table_id`, `auto_accept_shared_attachments`, `default_route_table_association`, `default_route_table_propagation`, `propagation_default_route_table_id`, `security_group_referencing_support`, `timeouts`, `transit_gateway_cidr_blocks`

### `aws_ec2_transit_gateway_connect`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (7): `creation_time`, `protocol`, `region`, `tags`, `tags_all`, `transit_gateway_id`, `transport_attachment_id`

**Extract attributes** (8): `creation_time`, `id`, `protocol`, `state`, `tags`, `tags_all`, `transit_gateway_id`, `transport_attachment_id`

**Missing from inject** (3): `timeouts`, `transit_gateway_default_route_table_association`, `transit_gateway_default_route_table_propagation`

**Missing from extract** (3): `timeouts`, `transit_gateway_default_route_table_association`, `transit_gateway_default_route_table_propagation`

### `aws_ec2_transit_gateway_connect_peer`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (10): `arn`, `bgp_asn`, `creation_time`, `inside_cidr_blocks`, `peer_address`, `region`, `tags`, `tags_all`, `transit_gateway_address`, `transit_gateway_attachment_id`

**Extract attributes** (9): `arn`, `creation_time`, `id`, `inside_cidr_blocks`, `peer_address`, `tags`, `tags_all`, `transit_gateway_address`, `transit_gateway_attachment_id`

**Missing from inject** (3): `bgp_peer_address`, `bgp_transit_gateway_addresses`, `timeouts`

**Missing from extract** (4): `bgp_asn`, `bgp_peer_address`, `bgp_transit_gateway_addresses`, `timeouts`

### `aws_ec2_transit_gateway_default_route_table_association`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (3): `region`, `transit_gateway_id`, `transit_gateway_route_table_id`

**Extract attributes** (0): (none)

**Missing from inject** (2): `original_default_route_table_id`, `timeouts`

**Missing from extract** (4): `original_default_route_table_id`, `timeouts`, `transit_gateway_id`, `transit_gateway_route_table_id`

### `aws_ec2_transit_gateway_default_route_table_propagation`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (3): `region`, `transit_gateway_id`, `transit_gateway_route_table_id`

**Extract attributes** (0): (none)

**Missing from inject** (2): `original_default_route_table_id`, `timeouts`

**Missing from extract** (4): `original_default_route_table_id`, `timeouts`, `transit_gateway_id`, `transit_gateway_route_table_id`

### `aws_ec2_transit_gateway_multicast_domain`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (10): `arn`, `auto_accept_shared_associations`, `creation_time`, `igmpv2_support`, `owner_id`, `region`, `static_sources_support`, `tags`, `tags_all`, `transit_gateway_id`

**Extract attributes** (11): `arn`, `auto_accept_shared_associations`, `creation_time`, `id`, `igmpv2_support`, `owner_id`, `state`, `static_sources_support`, `tags`, `tags_all`, `transit_gateway_id`

**Missing from inject** (1): `timeouts`

**Missing from extract** (1): `timeouts`

### `aws_ec2_transit_gateway_multicast_domain_association`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (4): `region`, `subnet_id`, `transit_gateway_attachment_id`, `transit_gateway_multicast_domain_id`

**Extract attributes** (4): `id`, `subnet_id`, `transit_gateway_attachment_id`, `transit_gateway_multicast_domain_id`

**Missing from inject** (1): `timeouts`

**Missing from extract** (1): `timeouts`

### `aws_ec2_transit_gateway_multicast_group_member`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (4): `group_ip_address`, `network_interface_id`, `region`, `transit_gateway_multicast_domain_id`

**Extract attributes** (4): `group_ip_address`, `id`, `network_interface_id`, `transit_gateway_multicast_domain_id`

### `aws_ec2_transit_gateway_multicast_group_source`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (4): `group_ip_address`, `network_interface_id`, `region`, `transit_gateway_multicast_domain_id`

**Extract attributes** (4): `group_ip_address`, `id`, `network_interface_id`, `transit_gateway_multicast_domain_id`

### `aws_ec2_transit_gateway_peering_attachment`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (7): `peer_account_id`, `peer_region`, `peer_transit_gateway_id`, `region`, `tags`, `tags_all`, `transit_gateway_id`

**Extract attributes** (8): `id`, `peer_account_id`, `peer_region`, `peer_transit_gateway_id`, `state`, `tags`, `tags_all`, `transit_gateway_id`

**Missing from inject** (3): `arn`, `options`, `state`

**Missing from extract** (2): `arn`, `options`

### `aws_ec2_transit_gateway_peering_attachment_accepter`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (2): `region`, `transit_gateway_attachment_id`

**Extract attributes** (0): (none)

**Missing from inject** (6): `peer_account_id`, `peer_region`, `peer_transit_gateway_id`, `tags`, `tags_all`, `transit_gateway_id`

**Missing from extract** (7): `peer_account_id`, `peer_region`, `peer_transit_gateway_id`, `tags`, `tags_all`, `transit_gateway_attachment_id`, `transit_gateway_id`

### `aws_ec2_transit_gateway_policy_table`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (5): `creation_time`, `region`, `tags`, `tags_all`, `transit_gateway_id`

**Extract attributes** (6): `creation_time`, `id`, `state`, `tags`, `tags_all`, `transit_gateway_id`

**Missing from inject** (2): `arn`, `state`

**Missing from extract** (1): `arn`

### `aws_ec2_transit_gateway_policy_table_association`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (4): `region`, `resource_id`, `transit_gateway_attachment_id`, `transit_gateway_policy_table_id`

**Extract attributes** (4): `id`, `resource_id`, `transit_gateway_attachment_id`, `transit_gateway_policy_table_id`

**Missing from inject** (1): `resource_type`

**Missing from extract** (1): `resource_type`

### `aws_ec2_transit_gateway_prefix_list_reference`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (6): `blackhole`, `prefix_list_id`, `prefix_list_owner_id`, `region`, `transit_gateway_attachment_id`, `transit_gateway_route_table_id`

**Extract attributes** (6): `blackhole`, `id`, `prefix_list_id`, `prefix_list_owner_id`, `transit_gateway_attachment_id`, `transit_gateway_route_table_id`

### `aws_ec2_transit_gateway_route`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (5): `blackhole`, `destination_cidr_block`, `region`, `transit_gateway_attachment_id`, `transit_gateway_route_table_id`

**Extract attributes** (5): `blackhole`, `destination_cidr_block`, `id`, `transit_gateway_attachment_id`, `transit_gateway_route_table_id`

### `aws_ec2_transit_gateway_route_table`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (7): `arn`, `default_association_route_table`, `default_propagation_route_table`, `region`, `tags`, `tags_all`, `transit_gateway_id`

**Extract attributes** (7): `arn`, `default_association_route_table`, `default_propagation_route_table`, `id`, `tags`, `tags_all`, `transit_gateway_id`

### `aws_ec2_transit_gateway_route_table_association`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (5): `region`, `replace_existing_association`, `resource_id`, `transit_gateway_attachment_id`, `transit_gateway_route_table_id`

**Extract attributes** (0): (none)

**Missing from inject** (1): `resource_type`

**Missing from extract** (5): `replace_existing_association`, `resource_id`, `resource_type`, `transit_gateway_attachment_id`, `transit_gateway_route_table_id`

### `aws_ec2_transit_gateway_route_table_propagation`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (4): `region`, `resource_id`, `transit_gateway_attachment_id`, `transit_gateway_route_table_id`

**Extract attributes** (0): (none)

**Missing from inject** (1): `resource_type`

**Missing from extract** (4): `resource_id`, `resource_type`, `transit_gateway_attachment_id`, `transit_gateway_route_table_id`

### `aws_ec2_transit_gateway_vpc_attachment`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (12): `appliance_mode_support`, `dns_support`, `ipv6_support`, `region`, `subnet_ids`, `tags`, `tags_all`, `transit_gateway_default_route_table_association`, `transit_gateway_default_route_table_propagation`, `transit_gateway_id`, `vpc_id`, `vpc_owner_id`

**Extract attributes** (7): `id`, `subnet_ids`, `tags`, `tags_all`, `transit_gateway_id`, `vpc_id`, `vpc_owner_id`

**Missing from inject** (2): `arn`, `security_group_referencing_support`

**Missing from extract** (7): `appliance_mode_support`, `arn`, `dns_support`, `ipv6_support`, `security_group_referencing_support`, `transit_gateway_default_route_table_association`, `transit_gateway_default_route_table_propagation`

### `aws_ec2_transit_gateway_vpc_attachment_accepter`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (4): `region`, `transit_gateway_attachment_id`, `transit_gateway_default_route_table_association`, `transit_gateway_default_route_table_propagation`

**Extract attributes** (0): (none)

**Missing from inject** (10): `appliance_mode_support`, `dns_support`, `ipv6_support`, `security_group_referencing_support`, `subnet_ids`, `tags`, `tags_all`, `transit_gateway_id`, `vpc_id`, `vpc_owner_id`

**Missing from extract** (13): `appliance_mode_support`, `dns_support`, `ipv6_support`, `security_group_referencing_support`, `subnet_ids`, `tags`, `tags_all`, `transit_gateway_attachment_id`, `transit_gateway_default_route_table_association`, `transit_gateway_default_route_table_propagation`, `transit_gateway_id`, `vpc_id`, `vpc_owner_id`

### `aws_egress_only_internet_gateway`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (4): `region`, `tags`, `tags_all`, `vpc_id`

**Extract attributes** (4): `id`, `tags`, `tags_all`, `vpc_id`

### `aws_eip`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (11): `allocation_id`, `association_id`, `domain`, `instance`, `network_interface`, `private_ip`, `ptr_record`, `public_ip`, `region`, `tags`, `tags_all`

**Extract attributes** (13): `allocation_id`, `association_id`, `carrier_ip`, `domain`, `id`, `instance`, `network_interface`, `private_dns`, `private_ip`, `public_dns`, `public_ip`, `tags`, `tags_all`

**Missing from inject** (13): `address`, `arn`, `associate_with_private_ip`, `carrier_ip`, `customer_owned_ip`, `customer_owned_ipv4_pool`, `ipam_pool_id`, `network_border_group`, `private_dns`, `public_dns`, `public_ipv4_pool`, `timeouts`, `vpc`

**Missing from extract** (11): `address`, `arn`, `associate_with_private_ip`, `customer_owned_ip`, `customer_owned_ipv4_pool`, `ipam_pool_id`, `network_border_group`, `ptr_record`, `public_ipv4_pool`, `timeouts`, `vpc`

### `aws_eip_association`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (7): `allocation_id`, `allow_reassociation`, `instance_id`, `network_interface_id`, `private_ip_address`, `public_ip`, `region`

**Extract attributes** (7): `allocation_id`, `allow_reassociation`, `id`, `instance_id`, `network_interface_id`, `private_ip_address`, `public_ip`

### `aws_eip_domain_name`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (4): `allocation_id`, `domain_name`, `ptr_record`, `region`

**Extract attributes** (4): `allocation_id`, `domain_name`, `id`, `ptr_record`

**Missing from inject** (1): `timeouts`

**Missing from extract** (1): `timeouts`

### `aws_flow_log`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (17): `arn`, `deliver_cross_account_role`, `eni_id`, `iam_role_arn`, `log_destination`, `log_destination_type`, `log_format`, `log_group_name`, `max_aggregation_interval`, `region`, `subnet_id`, `tags`, `tags_all`, `traffic_type`, `transit_gateway_attachment_id`, `transit_gateway_id`, `vpc_id`

**Extract attributes** (13): `arn`, `eni_id`, `id`, `log_destination`, `log_destination_type`, `log_group_name`, `subnet_id`, `tags`, `tags_all`, `traffic_type`, `transit_gateway_attachment_id`, `transit_gateway_id`, `vpc_id`

**Missing from inject** (1): `destination_options`

**Missing from extract** (5): `deliver_cross_account_role`, `destination_options`, `iam_role_arn`, `log_format`, `max_aggregation_interval`

### `aws_instance`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (20): `ami`, `availability_zone`, `host_id`, `iam_instance_profile`, `instance_lifecycle`, `instance_type`, `key_name`, `monitoring`, `outpost_arn`, `placement_group`, `placement_partition_number`, `private_ip`, `public_ip`, `region`, `subnet_id`, `tags`, `tags_all`, `tenancy`, `user_data`, `user_data_base64`

**Extract attributes** (22): `ami`, `arn`, `availability_zone`, `host_id`, `iam_instance_profile`, `id`, `instance_lifecycle`, `instance_state`, `instance_type`, `key_name`, `monitoring`, `outpost_arn`, `placement_group`, `placement_partition_number`, `private_ip`, `public_ip`, `security_groups`, `subnet_id`, `tags`, `tags_all`, `tenancy`, `vpc_security_group_ids`

**Missing from inject** (40): `arn`, `associate_public_ip_address`, `capacity_reservation_specification`, `cpu_core_count`, `cpu_options`, `cpu_threads_per_core`, `credit_specification`, `disable_api_stop`, `disable_api_termination`, `ebs_block_device`, `ebs_optimized`, `enable_primary_ipv6`, `enclave_options`, `ephemeral_block_device`, `get_password_data`, `hibernation`, `host_resource_group_arn`, `instance_initiated_shutdown_behavior`, `instance_market_options`, `instance_state`, `ipv6_address_count`, `ipv6_addresses`, `launch_template`, `maintenance_options`, `metadata_options`, `network_interface`, `password_data`, `primary_network_interface_id`, `private_dns`, `private_dns_name_options`, `public_dns`, `root_block_device`, `secondary_private_ips`, `security_groups`, `source_dest_check`, `spot_instance_request_id`, `timeouts`, `user_data_replace_on_change`, `volume_tags`, `vpc_security_group_ids`

**Missing from extract** (38): `associate_public_ip_address`, `capacity_reservation_specification`, `cpu_core_count`, `cpu_options`, `cpu_threads_per_core`, `credit_specification`, `disable_api_stop`, `disable_api_termination`, `ebs_block_device`, `ebs_optimized`, `enable_primary_ipv6`, `enclave_options`, `ephemeral_block_device`, `get_password_data`, `hibernation`, `host_resource_group_arn`, `instance_initiated_shutdown_behavior`, `instance_market_options`, `ipv6_address_count`, `ipv6_addresses`, `launch_template`, `maintenance_options`, `metadata_options`, `network_interface`, `password_data`, `primary_network_interface_id`, `private_dns`, `private_dns_name_options`, `public_dns`, `root_block_device`, `secondary_private_ips`, `source_dest_check`, `spot_instance_request_id`, `timeouts`, `user_data`, `user_data_base64`, `user_data_replace_on_change`, `volume_tags`

### `aws_internet_gateway`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (4): `region`, `tags`, `tags_all`, `vpc_id`

**Extract attributes** (6): `arn`, `id`, `owner_id`, `tags`, `tags_all`, `vpc_id`

**Missing from inject** (3): `arn`, `owner_id`, `timeouts`

**Missing from extract** (1): `timeouts`

### `aws_internet_gateway_attachment`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (3): `internet_gateway_id`, `region`, `vpc_id`

**Extract attributes** (3): `id`, `internet_gateway_id`, `vpc_id`

**Missing from inject** (1): `timeouts`

**Missing from extract** (1): `timeouts`

### `aws_key_pair`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (6): `fingerprint`, `key_name`, `key_pair_id`, `region`, `tags`, `tags_all`

**Extract attributes** (7): `arn`, `fingerprint`, `id`, `key_name`, `key_pair_id`, `tags`, `tags_all`

**Missing from inject** (4): `arn`, `key_name_prefix`, `key_type`, `public_key`

**Missing from extract** (3): `key_name_prefix`, `key_type`, `public_key`

### `aws_launch_template`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (12): `default_version`, `description`, `image_id`, `instance_type`, `key_name`, `latest_version`, `name`, `name_prefix`, `region`, `tags`, `tags_all`, `user_data`

**Extract attributes** (7): `arn`, `default_version`, `id`, `latest_version`, `name`, `tags`, `tags_all`

**Missing from inject** (29): `arn`, `block_device_mappings`, `capacity_reservation_specification`, `cpu_options`, `credit_specification`, `disable_api_stop`, `disable_api_termination`, `ebs_optimized`, `elastic_gpu_specifications`, `elastic_inference_accelerator`, `enclave_options`, `hibernation_options`, `iam_instance_profile`, `instance_initiated_shutdown_behavior`, `instance_market_options`, `instance_requirements`, `kernel_id`, `license_specification`, `maintenance_options`, `metadata_options`, `monitoring`, `network_interfaces`, `placement`, `private_dns_name_options`, `ram_disk_id`, `security_group_names`, `tag_specifications`, `update_default_version`, `vpc_security_group_ids`

**Missing from extract** (34): `block_device_mappings`, `capacity_reservation_specification`, `cpu_options`, `credit_specification`, `description`, `disable_api_stop`, `disable_api_termination`, `ebs_optimized`, `elastic_gpu_specifications`, `elastic_inference_accelerator`, `enclave_options`, `hibernation_options`, `iam_instance_profile`, `image_id`, `instance_initiated_shutdown_behavior`, `instance_market_options`, `instance_requirements`, `instance_type`, `kernel_id`, `key_name`, `license_specification`, `maintenance_options`, `metadata_options`, `monitoring`, `name_prefix`, `network_interfaces`, `placement`, `private_dns_name_options`, `ram_disk_id`, `security_group_names`, `tag_specifications`, `update_default_version`, `user_data`, `vpc_security_group_ids`

### `aws_main_route_table_association`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (4): `original_route_table_id`, `region`, `route_table_id`, `vpc_id`

**Extract attributes** (3): `id`, `route_table_id`, `vpc_id`

**Missing from inject** (1): `timeouts`

**Missing from extract** (2): `original_route_table_id`, `timeouts`

### `aws_nat_gateway`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (7): `allocation_id`, `connectivity_type`, `public_ip`, `region`, `subnet_id`, `tags`, `tags_all`

**Extract attributes** (11): `allocation_id`, `association_id`, `connectivity_type`, `id`, `nat_gateway_id`, `public_ip`, `state`, `subnet_id`, `tags`, `tags_all`, `vpc_id`

**Missing from inject** (7): `association_id`, `network_interface_id`, `private_ip`, `secondary_allocation_ids`, `secondary_private_ip_address_count`, `secondary_private_ip_addresses`, `timeouts`

**Missing from extract** (6): `network_interface_id`, `private_ip`, `secondary_allocation_ids`, `secondary_private_ip_address_count`, `secondary_private_ip_addresses`, `timeouts`

### `aws_network_acl`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (5): `region`, `subnet_ids`, `tags`, `tags_all`, `vpc_id`

**Extract attributes** (9): `arn`, `egress`, `id`, `ingress`, `owner_id`, `subnet_ids`, `tags`, `tags_all`, `vpc_id`

**Missing from inject** (4): `arn`, `egress`, `ingress`, `owner_id`

### `aws_network_acl_association`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (3): `network_acl_id`, `region`, `subnet_id`

**Extract attributes** (3): `id`, `network_acl_id`, `subnet_id`

### `aws_network_acl_rule`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (12): `cidr_block`, `egress`, `from_port`, `icmp_code`, `icmp_type`, `ipv6_cidr_block`, `network_acl_id`, `protocol`, `region`, `rule_action`, `rule_number`, `to_port`

**Extract attributes** (0): (none)

**Missing from extract** (11): `cidr_block`, `egress`, `from_port`, `icmp_code`, `icmp_type`, `ipv6_cidr_block`, `network_acl_id`, `protocol`, `rule_action`, `rule_number`, `to_port`

### `aws_network_interface`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (12): `description`, `interface_type`, `mac_address`, `outpost_arn`, `owner_id`, `private_dns_name`, `private_ip`, `region`, `source_dest_check`, `subnet_id`, `tags`, `tags_all`

**Extract attributes** (12): `arn`, `description`, `id`, `interface_type`, `owner_id`, `private_ip`, `private_ips`, `security_groups`, `source_dest_check`, `subnet_id`, `tags`, `tags_all`

**Missing from inject** (16): `arn`, `attachment`, `enable_primary_ipv6`, `ipv4_prefix_count`, `ipv4_prefixes`, `ipv6_address_count`, `ipv6_address_list`, `ipv6_address_list_enabled`, `ipv6_addresses`, `ipv6_prefix_count`, `ipv6_prefixes`, `private_ip_list`, `private_ip_list_enabled`, `private_ips`, `private_ips_count`, `security_groups`

**Missing from extract** (16): `attachment`, `enable_primary_ipv6`, `ipv4_prefix_count`, `ipv4_prefixes`, `ipv6_address_count`, `ipv6_address_list`, `ipv6_address_list_enabled`, `ipv6_addresses`, `ipv6_prefix_count`, `ipv6_prefixes`, `mac_address`, `outpost_arn`, `private_dns_name`, `private_ip_list`, `private_ip_list_enabled`, `private_ips_count`

### `aws_network_interface_attachment`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (6): `attachment_id`, `device_index`, `instance_id`, `network_interface_id`, `region`, `status`

**Extract attributes** (6): `attachment_id`, `device_index`, `id`, `instance_id`, `network_interface_id`, `status`

### `aws_network_interface_permission`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (4): `aws_account_id`, `network_interface_id`, `permission`, `region`

**Extract attributes** (4): `aws_account_id`, `id`, `network_interface_id`, `permission`

**Missing from inject** (2): `network_interface_permission_id`, `timeouts`

**Missing from extract** (2): `network_interface_permission_id`, `timeouts`

### `aws_network_interface_sg_attachment`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (3): `network_interface_id`, `region`, `security_group_id`

**Extract attributes** (3): `id`, `network_interface_id`, `security_group_id`

**Missing from inject** (1): `timeouts`

**Missing from extract** (1): `timeouts`

### `aws_placement_group`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (9): `arn`, `name`, `partition_count`, `placement_group_id`, `region`, `spread_level`, `strategy`, `tags`, `tags_all`

**Extract attributes** (9): `arn`, `id`, `name`, `partition_count`, `placement_group_id`, `spread_level`, `strategy`, `tags`, `tags_all`

### `aws_route`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (6): `destination_cidr_block`, `destination_ipv6_cidr_block`, `gateway_id`, `nat_gateway_id`, `region`, `route_table_id`

**Extract attributes** (11): `carrier_gateway_id`, `destination_cidr_block`, `destination_ipv6_cidr_block`, `gateway_id`, `id`, `nat_gateway_id`, `origin`, `route_table_id`, `state`, `transit_gateway_id`, `vpc_endpoint_id`

**Missing from inject** (14): `carrier_gateway_id`, `core_network_arn`, `destination_prefix_list_id`, `egress_only_gateway_id`, `instance_id`, `instance_owner_id`, `local_gateway_id`, `network_interface_id`, `origin`, `state`, `timeouts`, `transit_gateway_id`, `vpc_endpoint_id`, `vpc_peering_connection_id`

**Missing from extract** (9): `core_network_arn`, `destination_prefix_list_id`, `egress_only_gateway_id`, `instance_id`, `instance_owner_id`, `local_gateway_id`, `network_interface_id`, `timeouts`, `vpc_peering_connection_id`

### `aws_route_table`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (4): `region`, `tags`, `tags_all`, `vpc_id`

**Extract attributes** (17): `arn`, `associations`, `cidr_block`, `gateway_id`, `id`, `ipv6_cidr_block`, `main`, `nat_gateway_id`, `origin`, `owner_id`, `route`, `route_table_association_id`, `route_table_id`, `state`, `subnet_id`, `tags`, `vpc_id`

**Missing from inject** (5): `arn`, `owner_id`, `propagating_vgws`, `route`, `timeouts`

**Missing from extract** (3): `propagating_vgws`, `tags_all`, `timeouts`

### `aws_route_table_association`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (4): `gateway_id`, `region`, `route_table_id`, `subnet_id`

**Extract attributes** (3): `id`, `route_table_id`, `subnet_id`

**Missing from inject** (1): `timeouts`

**Missing from extract** (2): `gateway_id`, `timeouts`

### `aws_security_group`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (6): `description`, `name`, `region`, `tags`, `tags_all`, `vpc_id`

**Extract attributes** (9): `arn`, `description`, `egress`, `id`, `ingress`, `name`, `owner_id`, `tags`, `vpc_id`

**Missing from inject** (7): `arn`, `egress`, `ingress`, `name_prefix`, `owner_id`, `revoke_rules_on_delete`, `timeouts`

**Missing from extract** (4): `name_prefix`, `revoke_rules_on_delete`, `tags_all`, `timeouts`

### `aws_security_group_rule`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (9): `description`, `from_port`, `protocol`, `region`, `security_group_id`, `self`, `source_security_group_id`, `to_port`, `type`

**Extract attributes** (0): (none)

**Missing from inject** (5): `cidr_blocks`, `ipv6_cidr_blocks`, `prefix_list_ids`, `security_group_rule_id`, `timeouts`

**Missing from extract** (13): `cidr_blocks`, `description`, `from_port`, `ipv6_cidr_blocks`, `prefix_list_ids`, `protocol`, `security_group_id`, `security_group_rule_id`, `self`, `source_security_group_id`, `timeouts`, `to_port`, `type`

### `aws_service`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (0): (none)

**Extract attributes** (21): `address_family`, `allocation_default_netmask_length`, `allocation_max_netmask_length`, `allocation_min_netmask_length`, `allocation_resource_tags`, `arn`, `auto_import`, `description`, `id`, `ipam_scope_arn`, `ipam_scope_id`, `ipam_scope_type`, `locale`, `owner_id`, `pool_depth`, `public_ip_source`, `publicly_advertisable`, `source_ipam_pool_id`, `state`, `tags`, `tags_all`

**Note:** Resource type not found in Terraform AWS provider schema.

### `aws_spot_datafeed_subscription`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (3): `bucket`, `prefix`, `region`

**Extract attributes** (0): (none)

**Missing from extract** (2): `bucket`, `prefix`

### `aws_spot_fleet_request`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (11): `allocation_strategy`, `fleet_type`, `iam_fleet_role`, `instance_interruption_behaviour`, `region`, `spot_price`, `tags`, `tags_all`, `target_capacity`, `terminate_instances_with_expiration`, `wait_for_fulfillment`

**Extract attributes** (4): `iam_fleet_role`, `id`, `tags`, `target_capacity`

**Missing from inject** (19): `client_token`, `context`, `excess_capacity_termination_policy`, `instance_pools_to_use_count`, `launch_specification`, `launch_template_config`, `load_balancers`, `on_demand_allocation_strategy`, `on_demand_max_total_price`, `on_demand_target_capacity`, `replace_unhealthy_instances`, `spot_maintenance_strategies`, `spot_request_state`, `target_capacity_unit_type`, `target_group_arns`, `terminate_instances_on_delete`, `timeouts`, `valid_from`, `valid_until`

**Missing from extract** (26): `allocation_strategy`, `client_token`, `context`, `excess_capacity_termination_policy`, `fleet_type`, `instance_interruption_behaviour`, `instance_pools_to_use_count`, `launch_specification`, `launch_template_config`, `load_balancers`, `on_demand_allocation_strategy`, `on_demand_max_total_price`, `on_demand_target_capacity`, `replace_unhealthy_instances`, `spot_maintenance_strategies`, `spot_price`, `spot_request_state`, `tags_all`, `target_capacity_unit_type`, `target_group_arns`, `terminate_instances_on_delete`, `terminate_instances_with_expiration`, `timeouts`, `valid_from`, `valid_until`, `wait_for_fulfillment`

### `aws_spot_instance_request`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (10): `ami`, `instance_interruption_behavior`, `instance_type`, `region`, `spot_instance_id`, `spot_price`, `spot_type`, `tags`, `tags_all`, `wait_for_fulfillment`

**Extract attributes** (6): `ami`, `id`, `instance_type`, `spot_instance_id`, `spot_price`, `tags`

**Missing from inject** (58): `arn`, `associate_public_ip_address`, `availability_zone`, `block_duration_minutes`, `capacity_reservation_specification`, `cpu_core_count`, `cpu_options`, `cpu_threads_per_core`, `credit_specification`, `disable_api_stop`, `disable_api_termination`, `ebs_block_device`, `ebs_optimized`, `enable_primary_ipv6`, `enclave_options`, `ephemeral_block_device`, `get_password_data`, `hibernation`, `host_id`, `host_resource_group_arn`, `iam_instance_profile`, `instance_initiated_shutdown_behavior`, `instance_state`, `ipv6_address_count`, `ipv6_addresses`, `key_name`, `launch_group`, `launch_template`, `maintenance_options`, `metadata_options`, `monitoring`, `network_interface`, `outpost_arn`, `password_data`, `placement_group`, `placement_partition_number`, `primary_network_interface_id`, `private_dns`, `private_dns_name_options`, `private_ip`, `public_dns`, `public_ip`, `root_block_device`, `secondary_private_ips`, `security_groups`, `source_dest_check`, `spot_bid_status`, `spot_request_state`, `subnet_id`, `tenancy`, `timeouts`, `user_data`, `user_data_base64`, `user_data_replace_on_change`, `valid_from`, `valid_until`, `volume_tags`, `vpc_security_group_ids`

**Missing from extract** (62): `arn`, `associate_public_ip_address`, `availability_zone`, `block_duration_minutes`, `capacity_reservation_specification`, `cpu_core_count`, `cpu_options`, `cpu_threads_per_core`, `credit_specification`, `disable_api_stop`, `disable_api_termination`, `ebs_block_device`, `ebs_optimized`, `enable_primary_ipv6`, `enclave_options`, `ephemeral_block_device`, `get_password_data`, `hibernation`, `host_id`, `host_resource_group_arn`, `iam_instance_profile`, `instance_initiated_shutdown_behavior`, `instance_interruption_behavior`, `instance_state`, `ipv6_address_count`, `ipv6_addresses`, `key_name`, `launch_group`, `launch_template`, `maintenance_options`, `metadata_options`, `monitoring`, `network_interface`, `outpost_arn`, `password_data`, `placement_group`, `placement_partition_number`, `primary_network_interface_id`, `private_dns`, `private_dns_name_options`, `private_ip`, `public_dns`, `public_ip`, `root_block_device`, `secondary_private_ips`, `security_groups`, `source_dest_check`, `spot_bid_status`, `spot_request_state`, `spot_type`, `subnet_id`, `tags_all`, `tenancy`, `timeouts`, `user_data`, `user_data_base64`, `user_data_replace_on_change`, `valid_from`, `valid_until`, `volume_tags`, `vpc_security_group_ids`, `wait_for_fulfillment`

### `aws_subnet`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (9): `availability_zone`, `cidr_block`, `ipv6_cidr_block`, `ipv6_cidr_block_associations`, `map_public_ip_on_launch`, `region`, `tags`, `tags_all`, `vpc_id`

**Extract attributes** (17): `arn`, `assign_ipv6_address_on_creation`, `association_id`, `availability_zone`, `availability_zone_id`, `available_ip_address_count`, `cidr_block`, `id`, `ipv6_cidr_block`, `ipv6_cidr_block_associations`, `map_public_ip_on_launch`, `owner_id`, `private_dns_hostname_type_on_launch`, `state`, `tags`, `tags_all`, `vpc_id`

**Missing from inject** (15): `arn`, `assign_ipv6_address_on_creation`, `availability_zone_id`, `customer_owned_ipv4_pool`, `enable_dns64`, `enable_lni_at_device_index`, `enable_resource_name_dns_a_record_on_launch`, `enable_resource_name_dns_aaaa_record_on_launch`, `ipv6_cidr_block_association_id`, `ipv6_native`, `map_customer_owned_ip_on_launch`, `outpost_arn`, `owner_id`, `private_dns_hostname_type_on_launch`, `timeouts`

**Missing from extract** (10): `customer_owned_ipv4_pool`, `enable_dns64`, `enable_lni_at_device_index`, `enable_resource_name_dns_a_record_on_launch`, `enable_resource_name_dns_aaaa_record_on_launch`, `ipv6_cidr_block_association_id`, `ipv6_native`, `map_customer_owned_ip_on_launch`, `outpost_arn`, `timeouts`

### `aws_verifiedaccess_endpoint`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (16): `application_domain`, `attachment_type`, `creation_time`, `description`, `device_validation_domain`, `domain_certificate_arn`, `endpoint_domain`, `endpoint_domain_prefix`, `endpoint_type`, `last_updated_time`, `policy_document`, `region`, `tags`, `tags_all`, `verified_access_group_id`, `verifiedaccess_group_id`

**Extract attributes** (17): `application_domain`, `attachment_type`, `creation_time`, `description`, `device_validation_domain`, `domain_certificate_arn`, `endpoint_domain`, `endpoint_type`, `id`, `last_updated_time`, `policy_document`, `security_group_ids`, `status_code`, `tags`, `tags_all`, `verifiedaccess_group_id`, `verifiedaccess_instance_id`

**Missing from inject** (8): `cidr_options`, `load_balancer_options`, `network_interface_options`, `rds_options`, `security_group_ids`, `sse_specification`, `timeouts`, `verified_access_instance_id`

**Missing from extract** (9): `cidr_options`, `endpoint_domain_prefix`, `load_balancer_options`, `network_interface_options`, `rds_options`, `sse_specification`, `timeouts`, `verified_access_group_id`, `verified_access_instance_id`

### `aws_verifiedaccess_group`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (11): `arn`, `creation_time`, `description`, `last_updated_time`, `owner`, `policy_document`, `region`, `tags`, `tags_all`, `verified_access_instance_id`, `verifiedaccess_instance_id`

**Extract attributes** (10): `creation_time`, `description`, `id`, `last_updated_time`, `owner`, `policy_document`, `tags`, `tags_all`, `verifiedaccess_group_arn`, `verifiedaccess_instance_id`

**Missing from inject** (4): `deletion_time`, `sse_configuration`, `verifiedaccess_group_arn`, `verifiedaccess_group_id`

**Missing from extract** (3): `deletion_time`, `sse_configuration`, `verifiedaccess_group_id`

### `aws_verifiedaccess_instance`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (9): `cidr_endpoints_custom_subdomain`, `creation_time`, `description`, `fips_enabled`, `last_updated_time`, `name`, `region`, `tags`, `tags_all`

**Extract attributes** (10): `cidr_endpoints_custom_subdomain`, `creation_time`, `description`, `fips_enabled`, `id`, `last_updated_time`, `name`, `tags`, `tags_all`, `verified_access_trust_provider_ids`

**Missing from inject** (2): `name_servers`, `verified_access_trust_providers`

**Missing from extract** (2): `name_servers`, `verified_access_trust_providers`

### `aws_verifiedaccess_instance_logging_configuration`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (2): `region`, `verifiedaccess_instance_id`

**Extract attributes** (0): (none)

**Missing from inject** (1): `access_logs`

**Missing from extract** (2): `access_logs`, `verifiedaccess_instance_id`

### `aws_verifiedaccess_instance_trust_provider_attachment`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (3): `region`, `verifiedaccess_instance_id`, `verifiedaccess_trust_provider_id`

**Extract attributes** (0): (none)

**Missing from extract** (2): `verifiedaccess_instance_id`, `verifiedaccess_trust_provider_id`

### `aws_verifiedaccess_trust_provider`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (10): `creation_time`, `description`, `device_trust_provider_type`, `last_updated_time`, `policy_reference_name`, `region`, `tags`, `tags_all`, `trust_provider_type`, `user_trust_provider_type`

**Extract attributes** (10): `creation_time`, `description`, `device_trust_provider_type`, `id`, `last_updated_time`, `policy_reference_name`, `tags`, `tags_all`, `trust_provider_type`, `user_trust_provider_type`

**Missing from inject** (5): `device_options`, `native_application_oidc_options`, `oidc_options`, `sse_specification`, `timeouts`

**Missing from extract** (5): `device_options`, `native_application_oidc_options`, `oidc_options`, `sse_specification`, `timeouts`

### `aws_volume_attachment`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (7): `device_name`, `force_detach`, `instance_id`, `region`, `skip_destroy`, `stop_instance_before_detaching`, `volume_id`

**Extract attributes** (7): `device_name`, `force_detach`, `id`, `instance_id`, `skip_destroy`, `stop_instance_before_detaching`, `volume_id`

**Missing from inject** (1): `timeouts`

**Missing from extract** (1): `timeouts`

### `aws_vpc`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (10): `cidr_block`, `default`, `dhcp_options_id`, `enable_dns_hostnames`, `enable_dns_support`, `instance_tenancy`, `region`, `secondary_cidr_blocks`, `tags`, `tags_all`

**Extract attributes** (17): `arn`, `association_id`, `cidr_block`, `default`, `default_network_acl_id`, `default_route_table_id`, `default_security_group_id`, `dhcp_options_id`, `enable_dns_hostnames`, `enable_dns_support`, `id`, `instance_tenancy`, `main_route_table_id`, `owner_id`, `secondary_cidr_blocks`, `state`, `tags`

**Missing from inject** (15): `arn`, `assign_generated_ipv6_cidr_block`, `default_network_acl_id`, `default_route_table_id`, `default_security_group_id`, `enable_network_address_usage_metrics`, `ipv4_ipam_pool_id`, `ipv4_netmask_length`, `ipv6_association_id`, `ipv6_cidr_block`, `ipv6_cidr_block_network_border_group`, `ipv6_ipam_pool_id`, `ipv6_netmask_length`, `main_route_table_id`, `owner_id`

**Missing from extract** (10): `assign_generated_ipv6_cidr_block`, `enable_network_address_usage_metrics`, `ipv4_ipam_pool_id`, `ipv4_netmask_length`, `ipv6_association_id`, `ipv6_cidr_block`, `ipv6_cidr_block_network_border_group`, `ipv6_ipam_pool_id`, `ipv6_netmask_length`, `tags_all`

### `aws_vpc_block_public_access_exclusion`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (6): `internet_gateway_exclusion_mode`, `region`, `subnet_id`, `tags`, `tags_all`, `vpc_id`

**Extract attributes** (0): (none)

**Missing from inject** (2): `resource_arn`, `timeouts`

**Missing from extract** (7): `internet_gateway_exclusion_mode`, `resource_arn`, `subnet_id`, `tags`, `tags_all`, `timeouts`, `vpc_id`

### `aws_vpc_block_public_access_options`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (2): `internet_gateway_block_mode`, `region`

**Extract attributes** (0): (none)

**Missing from inject** (3): `aws_account_id`, `aws_region`, `timeouts`

**Missing from extract** (4): `aws_account_id`, `aws_region`, `internet_gateway_block_mode`, `timeouts`

### `aws_vpc_dhcp_options`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (8): `arn`, `domain_name`, `ipv6_address_preferred_lease_time`, `netbios_node_type`, `owner_id`, `region`, `tags`, `tags_all`

**Extract attributes** (11): `arn`, `domain_name`, `domain_name_servers`, `id`, `ipv6_address_preferred_lease_time`, `netbios_name_servers`, `netbios_node_type`, `ntp_servers`, `owner_id`, `tags`, `tags_all`

**Missing from inject** (3): `domain_name_servers`, `netbios_name_servers`, `ntp_servers`

### `aws_vpc_dhcp_options_association`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (3): `dhcp_options_id`, `region`, `vpc_id`

**Extract attributes** (3): `dhcp_options_id`, `id`, `vpc_id`

### `aws_vpc_endpoint`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (10): `auto_accept`, `policy`, `private_dns_enabled`, `region`, `service_name`, `state`, `tags`, `tags_all`, `vpc_endpoint_type`, `vpc_id`

**Extract attributes** (15): `arn`, `auto_accept`, `id`, `owner_id`, `policy`, `private_dns_enabled`, `route_table_ids`, `security_group_ids`, `service_name`, `state`, `subnet_ids`, `tags`, `tags_all`, `vpc_endpoint_type`, `vpc_id`

**Missing from inject** (17): `arn`, `cidr_blocks`, `dns_entry`, `dns_options`, `ip_address_type`, `network_interface_ids`, `owner_id`, `prefix_list_id`, `requester_managed`, `resource_configuration_arn`, `route_table_ids`, `security_group_ids`, `service_network_arn`, `service_region`, `subnet_configuration`, `subnet_ids`, `timeouts`

**Missing from extract** (12): `cidr_blocks`, `dns_entry`, `dns_options`, `ip_address_type`, `network_interface_ids`, `prefix_list_id`, `requester_managed`, `resource_configuration_arn`, `service_network_arn`, `service_region`, `subnet_configuration`, `timeouts`

### `aws_vpc_endpoint_connection_accepter`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (4): `region`, `vpc_endpoint_id`, `vpc_endpoint_service_id`, `vpc_endpoint_state`

**Extract attributes** (4): `id`, `vpc_endpoint_id`, `vpc_endpoint_service_id`, `vpc_endpoint_state`

### `aws_vpc_endpoint_connection_notification`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (6): `connection_notification_arn`, `notification_type`, `region`, `state`, `vpc_endpoint_id`, `vpc_endpoint_service_id`

**Extract attributes** (7): `connection_events`, `connection_notification_arn`, `id`, `notification_type`, `state`, `vpc_endpoint_id`, `vpc_endpoint_service_id`

**Missing from inject** (1): `connection_events`

### `aws_vpc_endpoint_policy`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (3): `policy`, `region`, `vpc_endpoint_id`

**Extract attributes** (0): (none)

**Missing from inject** (1): `timeouts`

**Missing from extract** (3): `policy`, `timeouts`, `vpc_endpoint_id`

### `aws_vpc_endpoint_private_dns`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (3): `private_dns_enabled`, `region`, `vpc_endpoint_id`

**Extract attributes** (0): (none)

**Missing from extract** (2): `private_dns_enabled`, `vpc_endpoint_id`

### `aws_vpc_endpoint_route_table_association`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (3): `region`, `route_table_id`, `vpc_endpoint_id`

**Extract attributes** (0): (none)

**Missing from extract** (2): `route_table_id`, `vpc_endpoint_id`

### `aws_vpc_endpoint_security_group_association`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (4): `region`, `replace_default_association`, `security_group_id`, `vpc_endpoint_id`

**Extract attributes** (0): (none)

**Missing from extract** (3): `replace_default_association`, `security_group_id`, `vpc_endpoint_id`

### `aws_vpc_endpoint_service`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (8): `acceptance_required`, `payer_responsibility`, `private_dns_name`, `region`, `service_type`, `state`, `tags`, `tags_all`

**Extract attributes** (12): `acceptance_required`, `allowed_principals`, `arn`, `gateway_load_balancer_arns`, `id`, `network_load_balancer_arns`, `payer_responsibility`, `service_name`, `service_type`, `state`, `tags`, `tags_all`

**Missing from inject** (12): `allowed_principals`, `arn`, `availability_zones`, `base_endpoint_dns_names`, `gateway_load_balancer_arns`, `manages_vpc_endpoints`, `network_load_balancer_arns`, `private_dns_name_configuration`, `service_name`, `supported_ip_address_types`, `supported_regions`, `timeouts`

**Missing from extract** (8): `availability_zones`, `base_endpoint_dns_names`, `manages_vpc_endpoints`, `private_dns_name`, `private_dns_name_configuration`, `supported_ip_address_types`, `supported_regions`, `timeouts`

### `aws_vpc_endpoint_service_allowed_principal`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (3): `principal_arn`, `region`, `vpc_endpoint_service_id`

**Extract attributes** (0): (none)

**Missing from extract** (2): `principal_arn`, `vpc_endpoint_service_id`

### `aws_vpc_endpoint_service_private_dns_verification`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (3): `region`, `service_id`, `wait_for_verification`

**Extract attributes** (0): (none)

**Missing from inject** (1): `timeouts`

**Missing from extract** (3): `service_id`, `timeouts`, `wait_for_verification`

### `aws_vpc_endpoint_subnet_association`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (3): `region`, `subnet_id`, `vpc_endpoint_id`

**Extract attributes** (0): (none)

**Missing from inject** (1): `timeouts`

**Missing from extract** (3): `subnet_id`, `timeouts`, `vpc_endpoint_id`

### `aws_vpc_ipam`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (14): `arn`, `default_resource_discovery_association_id`, `default_resource_discovery_id`, `description`, `enable_private_gua`, `metered_account`, `operating_regions`, `owner_id`, `private_default_scope_id`, `public_default_scope_id`, `region`, `tags`, `tags_all`, `tier`

**Extract attributes** (18): `arn`, `default_resource_discovery_association_id`, `default_resource_discovery_id`, `description`, `enable_private_gua`, `id`, `ipam_region`, `operating_regions`, `owner_id`, `private_default_scope_id`, `public_default_scope_id`, `region_name`, `resource_discovery_association_count`, `scope_count`, `state`, `tags`, `tags_all`, `tier`

**Missing from inject** (3): `cascade`, `scope_count`, `timeouts`

**Missing from extract** (2): `cascade`, `timeouts`

### `aws_vpc_ipam_organization_admin_account`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (1): `delegated_admin_account_id`

**Extract attributes** (0): (none)

**Missing from inject** (4): `arn`, `email`, `name`, `service_principal`

**Missing from extract** (5): `arn`, `delegated_admin_account_id`, `email`, `name`, `service_principal`

### `aws_vpc_ipam_pool`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (22): `address_family`, `allocation_default_netmask_length`, `allocation_max_netmask_length`, `allocation_min_netmask_length`, `allocation_resource_tags`, `arn`, `auto_import`, `aws_service`, `description`, `ipam_scope_id`, `locale`, `owner_id`, `public_ip_source`, `publicly_advertisable`, `region`, `source_ipam_pool_id`, `source_resource_id`, `source_resource_owner`, `source_resource_region`, `source_resource_type`, `tags`, `tags_all`

**Extract attributes** (22): `address_family`, `allocation_default_netmask_length`, `allocation_max_netmask_length`, `allocation_min_netmask_length`, `allocation_resource_tags`, `arn`, `auto_import`, `aws_service`, `description`, `id`, `ipam_scope_arn`, `ipam_scope_id`, `ipam_scope_type`, `locale`, `owner_id`, `pool_depth`, `public_ip_source`, `publicly_advertisable`, `source_ipam_pool_id`, `state`, `tags`, `tags_all`

**Missing from inject** (5): `cascade`, `ipam_scope_type`, `pool_depth`, `state`, `timeouts`

**Missing from extract** (2): `cascade`, `timeouts`

### `aws_vpc_ipam_pool_cidr`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (5): `cidr`, `ipam_pool_cidr_id`, `ipam_pool_id`, `netmask_length`, `region`

**Extract attributes** (6): `cidr`, `id`, `ipam_pool_cidr_id`, `ipam_pool_id`, `netmask_length`, `state`

**Missing from inject** (2): `cidr_authorization_context`, `timeouts`

**Missing from extract** (2): `cidr_authorization_context`, `timeouts`

### `aws_vpc_ipam_pool_cidr_allocation`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (8): `cidr`, `description`, `ipam_pool_allocation_id`, `ipam_pool_id`, `region`, `resource_id`, `resource_owner`, `resource_region`

**Extract attributes** (8): `cidr`, `description`, `id`, `ipam_pool_allocation_id`, `ipam_pool_id`, `resource_id`, `resource_owner`, `resource_region`

**Missing from inject** (3): `disallowed_cidrs`, `netmask_length`, `resource_type`

**Missing from extract** (3): `disallowed_cidrs`, `netmask_length`, `resource_type`

### `aws_vpc_ipam_preview_next_cidr`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (2): `ipam_pool_id`, `netmask_length`

**Extract attributes** (0): (none)

**Missing from inject** (2): `cidr`, `disallowed_cidrs`

**Missing from extract** (4): `cidr`, `disallowed_cidrs`, `ipam_pool_id`, `netmask_length`

### `aws_vpc_ipam_resource_discovery`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (8): `arn`, `description`, `is_default`, `operating_regions`, `owner_id`, `region`, `tags`, `tags_all`

**Extract attributes** (10): `arn`, `description`, `id`, `ipam_resource_discovery_region`, `is_default`, `operating_regions`, `owner_id`, `region_name`, `tags`, `tags_all`

**Missing from inject** (2): `ipam_resource_discovery_region`, `timeouts`

**Missing from extract** (1): `timeouts`

### `aws_vpc_ipam_resource_discovery_association`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (5): `ipam_id`, `ipam_resource_discovery_id`, `region`, `tags`, `tags_all`

**Extract attributes** (0): (none)

**Missing from inject** (7): `arn`, `ipam_arn`, `ipam_region`, `is_default`, `owner_id`, `state`, `timeouts`

**Missing from extract** (11): `arn`, `ipam_arn`, `ipam_id`, `ipam_region`, `ipam_resource_discovery_id`, `is_default`, `owner_id`, `state`, `tags`, `tags_all`, `timeouts`

### `aws_vpc_ipam_scope`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (9): `arn`, `description`, `ipam_id`, `ipam_scope_type`, `is_default`, `owner_id`, `region`, `tags`, `tags_all`

**Extract attributes** (10): `arn`, `description`, `id`, `ipam_arn`, `ipam_scope_type`, `is_default`, `owner_id`, `pool_count`, `tags`, `tags_all`

**Missing from inject** (3): `ipam_arn`, `pool_count`, `timeouts`

**Missing from extract** (2): `ipam_id`, `timeouts`

### `aws_vpc_ipv4_cidr_block_association`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (5): `cidr_block`, `ipv4_ipam_pool_id`, `ipv4_netmask_length`, `region`, `vpc_id`

**Extract attributes** (0): (none)

**Missing from inject** (1): `timeouts`

**Missing from extract** (5): `cidr_block`, `ipv4_ipam_pool_id`, `ipv4_netmask_length`, `timeouts`, `vpc_id`

### `aws_vpc_ipv6_cidr_block_association`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (5): `ipv6_cidr_block`, `ipv6_ipam_pool_id`, `ipv6_netmask_length`, `region`, `vpc_id`

**Extract attributes** (0): (none)

**Missing from inject** (5): `assign_generated_ipv6_cidr_block`, `ip_source`, `ipv6_address_attribute`, `ipv6_pool`, `timeouts`

**Missing from extract** (9): `assign_generated_ipv6_cidr_block`, `ip_source`, `ipv6_address_attribute`, `ipv6_cidr_block`, `ipv6_ipam_pool_id`, `ipv6_netmask_length`, `ipv6_pool`, `timeouts`, `vpc_id`

### `aws_vpc_network_performance_metric_subscription`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (6): `destination`, `metric`, `period`, `region`, `source`, `statistic`

**Extract attributes** (0): (none)

**Missing from extract** (5): `destination`, `metric`, `period`, `source`, `statistic`

### `aws_vpc_peering_connection`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (8): `auto_accept`, `peer_owner_id`, `peer_region`, `peer_vpc_id`, `region`, `tags`, `tags_all`, `vpc_id`

**Extract attributes** (5): `auto_accept`, `id`, `peer_vpc_id`, `tags`, `vpc_id`

**Missing from inject** (4): `accept_status`, `accepter`, `requester`, `timeouts`

**Missing from extract** (7): `accept_status`, `accepter`, `peer_owner_id`, `peer_region`, `requester`, `tags_all`, `timeouts`

### `aws_vpc_peering_connection_accepter`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (3): `auto_accept`, `region`, `vpc_peering_connection_id`

**Extract attributes** (0): (none)

**Missing from inject** (10): `accept_status`, `accepter`, `peer_owner_id`, `peer_region`, `peer_vpc_id`, `requester`, `tags`, `tags_all`, `timeouts`, `vpc_id`

**Missing from extract** (12): `accept_status`, `accepter`, `auto_accept`, `peer_owner_id`, `peer_region`, `peer_vpc_id`, `requester`, `tags`, `tags_all`, `timeouts`, `vpc_id`, `vpc_peering_connection_id`

### `aws_vpc_peering_connection_options`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (2): `region`, `vpc_peering_connection_id`

**Extract attributes** (0): (none)

**Missing from inject** (2): `accepter`, `requester`

**Missing from extract** (3): `accepter`, `requester`, `vpc_peering_connection_id`

### `aws_vpc_route_server`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (8): `amazon_side_asn`, `persist_routes`, `persist_routes_duration`, `region`, `sns_notifications_enabled`, `sns_topic_arn`, `tags`, `tags_all`

**Extract attributes** (0): (none)

**Missing from inject** (3): `arn`, `route_server_id`, `timeouts`

**Missing from extract** (10): `amazon_side_asn`, `arn`, `persist_routes`, `persist_routes_duration`, `route_server_id`, `sns_notifications_enabled`, `sns_topic_arn`, `tags`, `tags_all`, `timeouts`

### `aws_vpc_route_server_endpoint`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (5): `region`, `route_server_id`, `subnet_id`, `tags`, `tags_all`

**Extract attributes** (0): (none)

**Missing from inject** (6): `arn`, `eni_address`, `eni_id`, `route_server_endpoint_id`, `timeouts`, `vpc_id`

**Missing from extract** (10): `arn`, `eni_address`, `eni_id`, `route_server_endpoint_id`, `route_server_id`, `subnet_id`, `tags`, `tags_all`, `timeouts`, `vpc_id`

### `aws_vpc_route_server_peer`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (5): `peer_address`, `region`, `route_server_endpoint_id`, `tags`, `tags_all`

**Extract attributes** (0): (none)

**Missing from inject** (9): `arn`, `bgp_options`, `endpoint_eni_address`, `endpoint_eni_id`, `route_server_id`, `route_server_peer_id`, `subnet_id`, `timeouts`, `vpc_id`

**Missing from extract** (13): `arn`, `bgp_options`, `endpoint_eni_address`, `endpoint_eni_id`, `peer_address`, `route_server_endpoint_id`, `route_server_id`, `route_server_peer_id`, `subnet_id`, `tags`, `tags_all`, `timeouts`, `vpc_id`

### `aws_vpc_route_server_propagation`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (3): `region`, `route_server_id`, `route_table_id`

**Extract attributes** (0): (none)

**Missing from inject** (1): `timeouts`

**Missing from extract** (3): `route_server_id`, `route_table_id`, `timeouts`

### `aws_vpc_route_server_vpc_association`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (3): `region`, `route_server_id`, `vpc_id`

**Extract attributes** (0): (none)

**Missing from inject** (1): `timeouts`

**Missing from extract** (3): `route_server_id`, `timeouts`, `vpc_id`

### `aws_vpc_security_group_egress_rule`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (0): (none)

**Extract attributes** (0): (none)

**Missing from inject** (13): `arn`, `cidr_ipv4`, `cidr_ipv6`, `description`, `from_port`, `ip_protocol`, `prefix_list_id`, `referenced_security_group_id`, `security_group_id`, `security_group_rule_id`, `tags`, `tags_all`, `to_port`

**Missing from extract** (13): `arn`, `cidr_ipv4`, `cidr_ipv6`, `description`, `from_port`, `ip_protocol`, `prefix_list_id`, `referenced_security_group_id`, `security_group_id`, `security_group_rule_id`, `tags`, `tags_all`, `to_port`

### `aws_vpc_security_group_ingress_rule`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (11): `arn`, `cidr_ipv4`, `cidr_ipv6`, `description`, `from_port`, `ip_protocol`, `prefix_list_id`, `referenced_security_group_id`, `region`, `security_group_id`, `to_port`

**Extract attributes** (0): (none)

**Missing from inject** (3): `security_group_rule_id`, `tags`, `tags_all`

**Missing from extract** (13): `arn`, `cidr_ipv4`, `cidr_ipv6`, `description`, `from_port`, `ip_protocol`, `prefix_list_id`, `referenced_security_group_id`, `security_group_id`, `security_group_rule_id`, `tags`, `tags_all`, `to_port`

### `aws_vpc_security_group_vpc_association`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (4): `region`, `security_group_id`, `state`, `vpc_id`

**Extract attributes** (4): `id`, `security_group_id`, `state`, `vpc_id`

**Missing from inject** (1): `timeouts`

**Missing from extract** (1): `timeouts`

### `aws_vpn_connection`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (15): `customer_gateway_id`, `enable_acceleration`, `local_ipv4_network_cidr`, `local_ipv6_network_cidr`, `outside_ip_address_type`, `region`, `remote_ipv4_network_cidr`, `remote_ipv6_network_cidr`, `static_routes_only`, `tags`, `tags_all`, `transit_gateway_id`, `tunnel_inside_ip_version`, `type`, `vpn_gateway_id`

**Extract attributes** (15): `arn`, `customer_gateway_id`, `id`, `local_ipv4_network_cidr`, `local_ipv6_network_cidr`, `remote_ipv4_network_cidr`, `remote_ipv6_network_cidr`, `state`, `static_routes_only`, `tags`, `tags_all`, `transit_gateway_id`, `tunnel_inside_ip_version`, `type`, `vpn_gateway_id`

**Missing from inject** (60): `arn`, `core_network_arn`, `core_network_attachment_arn`, `customer_gateway_configuration`, `preshared_key_arn`, `preshared_key_storage`, `routes`, `transit_gateway_attachment_id`, `transport_transit_gateway_attachment_id`, `tunnel1_address`, `tunnel1_bgp_asn`, `tunnel1_bgp_holdtime`, `tunnel1_cgw_inside_address`, `tunnel1_dpd_timeout_action`, `tunnel1_dpd_timeout_seconds`, `tunnel1_enable_tunnel_lifecycle_control`, `tunnel1_ike_versions`, `tunnel1_inside_cidr`, `tunnel1_inside_ipv6_cidr`, `tunnel1_log_options`, `tunnel1_phase1_dh_group_numbers`, `tunnel1_phase1_encryption_algorithms`, `tunnel1_phase1_integrity_algorithms`, `tunnel1_phase1_lifetime_seconds`, `tunnel1_phase2_dh_group_numbers`, `tunnel1_phase2_encryption_algorithms`, `tunnel1_phase2_integrity_algorithms`, `tunnel1_phase2_lifetime_seconds`, `tunnel1_preshared_key`, `tunnel1_rekey_fuzz_percentage`, `tunnel1_rekey_margin_time_seconds`, `tunnel1_replay_window_size`, `tunnel1_startup_action`, `tunnel1_vgw_inside_address`, `tunnel2_address`, `tunnel2_bgp_asn`, `tunnel2_bgp_holdtime`, `tunnel2_cgw_inside_address`, `tunnel2_dpd_timeout_action`, `tunnel2_dpd_timeout_seconds`, `tunnel2_enable_tunnel_lifecycle_control`, `tunnel2_ike_versions`, `tunnel2_inside_cidr`, `tunnel2_inside_ipv6_cidr`, `tunnel2_log_options`, `tunnel2_phase1_dh_group_numbers`, `tunnel2_phase1_encryption_algorithms`, `tunnel2_phase1_integrity_algorithms`, `tunnel2_phase1_lifetime_seconds`, `tunnel2_phase2_dh_group_numbers`, `tunnel2_phase2_encryption_algorithms`, `tunnel2_phase2_integrity_algorithms`, `tunnel2_phase2_lifetime_seconds`, `tunnel2_preshared_key`, `tunnel2_rekey_fuzz_percentage`, `tunnel2_rekey_margin_time_seconds`, `tunnel2_replay_window_size`, `tunnel2_startup_action`, `tunnel2_vgw_inside_address`, `vgw_telemetry`

**Missing from extract** (61): `core_network_arn`, `core_network_attachment_arn`, `customer_gateway_configuration`, `enable_acceleration`, `outside_ip_address_type`, `preshared_key_arn`, `preshared_key_storage`, `routes`, `transit_gateway_attachment_id`, `transport_transit_gateway_attachment_id`, `tunnel1_address`, `tunnel1_bgp_asn`, `tunnel1_bgp_holdtime`, `tunnel1_cgw_inside_address`, `tunnel1_dpd_timeout_action`, `tunnel1_dpd_timeout_seconds`, `tunnel1_enable_tunnel_lifecycle_control`, `tunnel1_ike_versions`, `tunnel1_inside_cidr`, `tunnel1_inside_ipv6_cidr`, `tunnel1_log_options`, `tunnel1_phase1_dh_group_numbers`, `tunnel1_phase1_encryption_algorithms`, `tunnel1_phase1_integrity_algorithms`, `tunnel1_phase1_lifetime_seconds`, `tunnel1_phase2_dh_group_numbers`, `tunnel1_phase2_encryption_algorithms`, `tunnel1_phase2_integrity_algorithms`, `tunnel1_phase2_lifetime_seconds`, `tunnel1_preshared_key`, `tunnel1_rekey_fuzz_percentage`, `tunnel1_rekey_margin_time_seconds`, `tunnel1_replay_window_size`, `tunnel1_startup_action`, `tunnel1_vgw_inside_address`, `tunnel2_address`, `tunnel2_bgp_asn`, `tunnel2_bgp_holdtime`, `tunnel2_cgw_inside_address`, `tunnel2_dpd_timeout_action`, `tunnel2_dpd_timeout_seconds`, `tunnel2_enable_tunnel_lifecycle_control`, `tunnel2_ike_versions`, `tunnel2_inside_cidr`, `tunnel2_inside_ipv6_cidr`, `tunnel2_log_options`, `tunnel2_phase1_dh_group_numbers`, `tunnel2_phase1_encryption_algorithms`, `tunnel2_phase1_integrity_algorithms`, `tunnel2_phase1_lifetime_seconds`, `tunnel2_phase2_dh_group_numbers`, `tunnel2_phase2_encryption_algorithms`, `tunnel2_phase2_integrity_algorithms`, `tunnel2_phase2_lifetime_seconds`, `tunnel2_preshared_key`, `tunnel2_rekey_fuzz_percentage`, `tunnel2_rekey_margin_time_seconds`, `tunnel2_replay_window_size`, `tunnel2_startup_action`, `tunnel2_vgw_inside_address`, `vgw_telemetry`

### `aws_vpn_connection_route`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (3): `destination_cidr_block`, `region`, `vpn_connection_id`

**Extract attributes** (3): `destination_cidr_block`, `id`, `vpn_connection_id`

### `aws_vpn_gateway`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (6): `amazon_side_asn`, `availability_zone`, `region`, `tags`, `tags_all`, `vpc_id`

**Extract attributes** (6): `amazon_side_asn`, `arn`, `id`, `tags`, `tags_all`, `vpc_id`

**Missing from inject** (1): `arn`

**Missing from extract** (1): `availability_zone`

### `aws_vpn_gateway_attachment`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (3): `region`, `vpc_id`, `vpn_gateway_id`

**Extract attributes** (3): `id`, `vpc_id`, `vpn_gateway_id`

### `aws_vpn_gateway_route_propagation`

**Source:** `crates/winterbaume-terraform/src/converters/ec2.rs`

**Inject attributes** (3): `region`, `route_table_id`, `vpn_gateway_id`

**Extract attributes** (0): (none)

**Missing from inject** (1): `timeouts`

**Missing from extract** (3): `route_table_id`, `timeouts`, `vpn_gateway_id`

### `aws_ec2_instance_connect_endpoint`

**Source:** `crates/winterbaume-terraform/src/converters/ec2instanceconnect.rs`

**Inject attributes** (7): `arn`, `creation_time`, `endpoint_config_name`, `endpoint_status`, `last_modified_time`, `name`, `region`

**Extract attributes** (14): `availability_zone`, `created_at`, `dns_name`, `fips_dns_name`, `id`, `instance_connect_endpoint_id`, `network_interface_ids`, `owner_id`, `security_group_ids`, `state`, `subnet_id`, `tags`, `tags_all`, `vpc_id`

**Missing from inject** (12): `availability_zone`, `dns_name`, `fips_dns_name`, `network_interface_ids`, `owner_id`, `preserve_client_ip`, `security_group_ids`, `subnet_id`, `tags`, `tags_all`, `timeouts`, `vpc_id`

**Missing from extract** (3): `arn`, `preserve_client_ip`, `timeouts`

### `aws_ecr_account_setting`

**Source:** `crates/winterbaume-terraform/src/converters/ecr.rs`

**Inject attributes** (3): `name`, `region`, `value`

**Extract attributes** (3): `id`, `name`, `value`

### `aws_ecr_lifecycle_policy`

**Source:** `crates/winterbaume-terraform/src/converters/ecr.rs`

**Inject attributes** (4): `policy`, `region`, `registry_id`, `repository`

**Extract attributes** (4): `id`, `policy`, `registry_id`, `repository`

### `aws_ecr_pull_through_cache_rule`

**Source:** `crates/winterbaume-terraform/src/converters/ecr.rs`

**Inject attributes** (7): `credential_arn`, `custom_role_arn`, `ecr_repository_prefix`, `region`, `registry_id`, `upstream_registry_url`, `upstream_repository_prefix`

**Extract attributes** (7): `credential_arn`, `custom_role_arn`, `ecr_repository_prefix`, `id`, `registry_id`, `upstream_registry_url`, `upstream_repository_prefix`

### `aws_ecr_registry_policy`

**Source:** `crates/winterbaume-terraform/src/converters/ecr.rs`

**Inject attributes** (3): `policy`, `region`, `registry_id`

**Extract attributes** (3): `id`, `policy`, `registry_id`

### `aws_ecr_registry_scanning_configuration`

**Source:** `crates/winterbaume-terraform/src/converters/ecr.rs`

**Inject attributes** (3): `region`, `registry_id`, `scan_type`

**Extract attributes** (8): `filter`, `filter_type`, `id`, `registry_id`, `repository_filter`, `rule`, `scan_frequency`, `scan_type`

**Missing from inject** (1): `rule`

### `aws_ecr_replication_configuration`

**Source:** `crates/winterbaume-terraform/src/converters/ecr.rs`

**Inject attributes** (2): `region`, `source_file_system_id`

**Extract attributes** (9): `destination`, `filter`, `filter_type`, `id`, `region`, `registry_id`, `replication_configuration`, `repository_filter`, `rule`

**Missing from inject** (2): `registry_id`, `replication_configuration`

### `aws_ecr_repository`

**Source:** `crates/winterbaume-terraform/src/converters/ecr.rs`

**Inject attributes** (8): `arn`, `encryption_configuration`, `image_scanning_configuration`, `image_tag_mutability`, `name`, `region`, `repository_url`, `tags`

**Extract attributes** (11): `arn`, `encryption_configuration`, `encryption_type`, `id`, `image_scanning_configuration`, `image_tag_mutability`, `name`, `registry_id`, `repository_url`, `scan_on_push`, `tags`

**Missing from inject** (4): `force_delete`, `registry_id`, `tags_all`, `timeouts`

**Missing from extract** (3): `force_delete`, `tags_all`, `timeouts`

### `aws_ecr_repository_creation_template`

**Source:** `crates/winterbaume-terraform/src/converters/ecr.rs`

**Inject attributes** (7): `custom_role_arn`, `description`, `image_tag_mutability`, `lifecycle_policy`, `prefix`, `region`, `repository_policy`

**Extract attributes** (8): `applied_for`, `custom_role_arn`, `description`, `id`, `image_tag_mutability`, `lifecycle_policy`, `prefix`, `repository_policy`

**Missing from inject** (4): `applied_for`, `encryption_configuration`, `registry_id`, `resource_tags`

**Missing from extract** (3): `encryption_configuration`, `registry_id`, `resource_tags`

### `aws_ecr_repository_policy`

**Source:** `crates/winterbaume-terraform/src/converters/ecr.rs`

**Inject attributes** (4): `policy`, `region`, `registry_id`, `repository`

**Extract attributes** (4): `id`, `policy`, `registry_id`, `repository`

### `aws_ecs_account_setting_default`

**Source:** `crates/winterbaume-terraform/src/converters/ecs.rs`

**Inject attributes** (0): (none)

**Extract attributes** (0): (none)

**Missing from inject** (3): `name`, `principal_arn`, `value`

**Missing from extract** (3): `name`, `principal_arn`, `value`

### `aws_ecs_capacity_provider`

**Source:** `crates/winterbaume-terraform/src/converters/ecs.rs`

**Inject attributes** (0): (none)

**Extract attributes** (0): (none)

**Missing from inject** (5): `arn`, `auto_scaling_group_provider`, `name`, `tags`, `tags_all`

**Missing from extract** (5): `arn`, `auto_scaling_group_provider`, `name`, `tags`, `tags_all`

### `aws_ecs_cluster`

**Source:** `crates/winterbaume-terraform/src/converters/ecs.rs`

**Inject attributes** (21): `acl_name`, `arn`, `auto_minor_version_upgrade`, `capacity_providers`, `configuration`, `description`, `engine`, `engine_version`, `maintenance_window`, `name`, `node_type`, `num_replicas_per_shard`, `num_shards`, `parameter_group_name`, `region`, `snapshot_retention_limit`, `snapshot_window`, `subnet_group_name`, `tags`, `tags_all`, `tls_enabled`

**Extract attributes** (6): `arn`, `capacity_providers`, `id`, `name`, `tags`, `tags_all`

**Missing from inject** (2): `service_connect_defaults`, `setting`

**Missing from extract** (3): `configuration`, `service_connect_defaults`, `setting`

### `aws_ecs_cluster_capacity_providers`

**Source:** `crates/winterbaume-terraform/src/converters/ecs.rs`

**Inject attributes** (0): (none)

**Extract attributes** (0): (none)

**Missing from inject** (3): `capacity_providers`, `cluster_name`, `default_capacity_provider_strategy`

**Missing from extract** (3): `capacity_providers`, `cluster_name`, `default_capacity_provider_strategy`

### `aws_ecs_service`

**Source:** `crates/winterbaume-terraform/src/converters/ecs.rs`

**Inject attributes** (24): `capacity_provider_strategy`, `cluster`, `deployment_circuit_breaker`, `deployment_controller`, `deployment_maximum_percent`, `deployment_minimum_healthy_percent`, `desired_count`, `enable_ecs_managed_tags`, `enable_execute_command`, `force_new_deployment`, `health_check_grace_period_seconds`, `launch_type`, `name`, `network_configuration`, `ordered_placement_strategy`, `pending_count`, `placement_constraints`, `region`, `running_count`, `scheduling_strategy`, `service_registries`, `tags`, `tags_all`, `task_definition`

**Extract attributes** (27): `arn`, `capacity_provider_strategy`, `cluster`, `container_name`, `container_port`, `deployment_controller`, `desired_count`, `elb_name`, `enable_ecs_managed_tags`, `enable_execute_command`, `health_check_grace_period_seconds`, `iam_role`, `id`, `launch_type`, `load_balancer`, `name`, `network_configuration`, `pending_count`, `propagate_tags`, `running_count`, `scheduling_strategy`, `status`, `tags`, `tags_all`, `target_group_arn`, `task_definition`, `type`

**Missing from inject** (13): `alarms`, `availability_zone_rebalancing`, `force_delete`, `iam_role`, `load_balancer`, `platform_version`, `propagate_tags`, `service_connect_configuration`, `timeouts`, `triggers`, `volume_configuration`, `vpc_lattice_configurations`, `wait_for_steady_state`

**Missing from extract** (17): `alarms`, `availability_zone_rebalancing`, `deployment_circuit_breaker`, `deployment_maximum_percent`, `deployment_minimum_healthy_percent`, `force_delete`, `force_new_deployment`, `ordered_placement_strategy`, `placement_constraints`, `platform_version`, `service_connect_configuration`, `service_registries`, `timeouts`, `triggers`, `volume_configuration`, `vpc_lattice_configurations`, `wait_for_steady_state`

### `aws_ecs_tag`

**Source:** `crates/winterbaume-terraform/src/converters/ecs.rs`

**Inject attributes** (0): (none)

**Extract attributes** (0): (none)

**Missing from inject** (3): `key`, `resource_arn`, `value`

**Missing from extract** (3): `key`, `resource_arn`, `value`

### `aws_ecs_task_definition`

**Source:** `crates/winterbaume-terraform/src/converters/ecs.rs`

**Inject attributes** (16): `arn`, `container_definitions`, `cpu`, `execution_role_arn`, `family`, `ipc_mode`, `memory`, `network_mode`, `pid_mode`, `region`, `requires_compatibilities`, `revision`, `skip_destroy`, `tags_all`, `task_role_arn`, `track_latest`

**Extract attributes** (14): `arn`, `arn_without_revision`, `cpu`, `execution_role_arn`, `family`, `id`, `memory`, `network_mode`, `requires_compatibilities`, `revision`, `status`, `tags_all`, `task_role_arn`, `track_latest`

**Missing from inject** (9): `arn_without_revision`, `enable_fault_injection`, `ephemeral_storage`, `inference_accelerator`, `placement_constraints`, `proxy_configuration`, `runtime_platform`, `tags`, `volume`

**Missing from extract** (12): `container_definitions`, `enable_fault_injection`, `ephemeral_storage`, `inference_accelerator`, `ipc_mode`, `pid_mode`, `placement_constraints`, `proxy_configuration`, `runtime_platform`, `skip_destroy`, `tags`, `volume`

### `aws_ecs_task_set`

**Source:** `crates/winterbaume-terraform/src/converters/ecs.rs`

**Inject attributes** (0): (none)

**Extract attributes** (0): (none)

**Missing from inject** (20): `arn`, `capacity_provider_strategy`, `cluster`, `external_id`, `force_delete`, `launch_type`, `load_balancer`, `network_configuration`, `platform_version`, `scale`, `service`, `service_registries`, `stability_status`, `status`, `tags`, `tags_all`, `task_definition`, `task_set_id`, `wait_until_stable`, `wait_until_stable_timeout`

**Missing from extract** (20): `arn`, `capacity_provider_strategy`, `cluster`, `external_id`, `force_delete`, `launch_type`, `load_balancer`, `network_configuration`, `platform_version`, `scale`, `service`, `service_registries`, `stability_status`, `status`, `tags`, `tags_all`, `task_definition`, `task_set_id`, `wait_until_stable`, `wait_until_stable_timeout`

### `aws_efs_access_point`

**Source:** `crates/winterbaume-terraform/src/converters/efs.rs`

**Inject attributes** (0): (none)

**Extract attributes** (0): (none)

**Missing from inject** (8): `arn`, `file_system_arn`, `file_system_id`, `owner_id`, `posix_user`, `root_directory`, `tags`, `tags_all`

**Missing from extract** (8): `arn`, `file_system_arn`, `file_system_id`, `owner_id`, `posix_user`, `root_directory`, `tags`, `tags_all`

### `aws_efs_backup_policy`

**Source:** `crates/winterbaume-terraform/src/converters/efs.rs`

**Inject attributes** (0): (none)

**Extract attributes** (0): (none)

**Missing from inject** (2): `backup_policy`, `file_system_id`

**Missing from extract** (2): `backup_policy`, `file_system_id`

### `aws_efs_file_system`

**Source:** `crates/winterbaume-terraform/src/converters/efs.rs`

**Inject attributes** (10): `arn`, `creation_token`, `encrypted`, `lifecycle_policy`, `performance_mode`, `protection`, `region`, `tags`, `tags_all`, `throughput_mode`

**Extract attributes** (20): `arn`, `creation_token`, `encrypted`, `id`, `life_cycle_state`, `lifecycle_policy`, `name`, `number_of_mount_targets`, `performance_mode`, `protection`, `replication_overwrite`, `size_in_bytes`, `tags`, `throughput_mode`, `transition_to_archive`, `transition_to_ia`, `transition_to_primary_storage_class`, `value`, `value_in_ia`, `value_in_standard`

**Missing from inject** (9): `availability_zone_id`, `availability_zone_name`, `dns_name`, `kms_key_id`, `name`, `number_of_mount_targets`, `owner_id`, `provisioned_throughput_in_mibps`, `size_in_bytes`

**Missing from extract** (7): `availability_zone_id`, `availability_zone_name`, `dns_name`, `kms_key_id`, `owner_id`, `provisioned_throughput_in_mibps`, `tags_all`

### `aws_efs_file_system_policy`

**Source:** `crates/winterbaume-terraform/src/converters/efs.rs`

**Inject attributes** (0): (none)

**Extract attributes** (0): (none)

**Missing from inject** (3): `bypass_policy_lockout_safety_check`, `file_system_id`, `policy`

**Missing from extract** (3): `bypass_policy_lockout_safety_check`, `file_system_id`, `policy`

### `aws_efs_mount_target`

**Source:** `crates/winterbaume-terraform/src/converters/efs.rs`

**Inject attributes** (0): (none)

**Extract attributes** (0): (none)

**Missing from inject** (12): `availability_zone_id`, `availability_zone_name`, `dns_name`, `file_system_arn`, `file_system_id`, `ip_address`, `mount_target_dns_name`, `network_interface_id`, `owner_id`, `security_groups`, `subnet_id`, `timeouts`

**Missing from extract** (12): `availability_zone_id`, `availability_zone_name`, `dns_name`, `file_system_arn`, `file_system_id`, `ip_address`, `mount_target_dns_name`, `network_interface_id`, `owner_id`, `security_groups`, `subnet_id`, `timeouts`

### `aws_efs_replication_configuration`

**Source:** `crates/winterbaume-terraform/src/converters/efs.rs`

**Inject attributes** (0): (none)

**Extract attributes** (0): (none)

**Missing from inject** (7): `creation_time`, `destination`, `original_source_file_system_arn`, `source_file_system_arn`, `source_file_system_id`, `source_file_system_region`, `timeouts`

**Missing from extract** (7): `creation_time`, `destination`, `original_source_file_system_arn`, `source_file_system_arn`, `source_file_system_id`, `source_file_system_region`, `timeouts`

### `aws_eks_access_entry`

**Source:** `crates/winterbaume-terraform/src/converters/eks.rs`

**Inject attributes** (10): `access_entry_arn`, `cluster_name`, `created_at`, `kubernetes_groups`, `modified_at`, `principal_arn`, `region`, `tags`, `type`, `user_name`

**Extract attributes** (11): `access_entry_arn`, `cluster_name`, `created_at`, `id`, `kubernetes_groups`, `modified_at`, `principal_arn`, `tags`, `tags_all`, `type`, `user_name`

**Missing from inject** (2): `tags_all`, `timeouts`

**Missing from extract** (1): `timeouts`

### `aws_eks_access_policy_association`

**Source:** `crates/winterbaume-terraform/src/converters/eks.rs`

**Inject attributes** (7): `access_scope`, `associated_at`, `cluster_name`, `modified_at`, `policy_arn`, `principal_arn`, `region`

**Extract attributes** (9): `access_scope`, `associated_at`, `cluster_name`, `id`, `modified_at`, `namespaces`, `policy_arn`, `principal_arn`, `type`

**Missing from inject** (1): `timeouts`

**Missing from extract** (1): `timeouts`

### `aws_eks_addon`

**Source:** `crates/winterbaume-terraform/src/converters/eks.rs`

**Inject attributes** (14): `addon_name`, `addon_version`, `arn`, `cluster_name`, `configuration_values`, `created_at`, `modified_at`, `preserve`, `region`, `resolve_conflicts_on_create`, `resolve_conflicts_on_update`, `service_account_role_arn`, `tags`, `tags_all`

**Extract attributes** (11): `addon_name`, `addon_version`, `arn`, `cluster_name`, `created_at`, `id`, `modified_at`, `service_account_role_arn`, `status`, `tags`, `tags_all`

**Missing from inject** (3): `pod_identity_association`, `resolve_conflicts`, `timeouts`

**Missing from extract** (7): `configuration_values`, `pod_identity_association`, `preserve`, `resolve_conflicts`, `resolve_conflicts_on_create`, `resolve_conflicts_on_update`, `timeouts`

### `aws_eks_cluster`

**Source:** `crates/winterbaume-terraform/src/converters/eks.rs`

**Inject attributes** (28): `access_config`, `acl_name`, `arn`, `auto_minor_version_upgrade`, `bootstrap_self_managed_addons`, `compute_config`, `description`, `encryption_config`, `engine`, `engine_version`, `kubernetes_network_config`, `maintenance_window`, `name`, `node_type`, `num_replicas_per_shard`, `num_shards`, `parameter_group_name`, `region`, `remote_network_config`, `snapshot_retention_limit`, `snapshot_window`, `storage_config`, `subnet_group_name`, `tags`, `tags_all`, `tls_enabled`, `vpc_config`, `zonal_shift_config`

**Extract attributes** (19): `access_config`, `arn`, `certificate_authority`, `created_at`, `data`, `endpoint`, `id`, `identity`, `issuer`, `kubernetes_network_config`, `name`, `oidc`, `platform_version`, `role_arn`, `service_ipv4_cidr`, `status`, `tags`, `tags_all`, `version`

**Missing from inject** (14): `certificate_authority`, `cluster_id`, `created_at`, `enabled_cluster_log_types`, `endpoint`, `force_update_version`, `identity`, `outpost_config`, `platform_version`, `role_arn`, `status`, `timeouts`, `upgrade_policy`, `version`

**Missing from extract** (13): `bootstrap_self_managed_addons`, `cluster_id`, `compute_config`, `enabled_cluster_log_types`, `encryption_config`, `force_update_version`, `outpost_config`, `remote_network_config`, `storage_config`, `timeouts`, `upgrade_policy`, `vpc_config`, `zonal_shift_config`

### `aws_eks_fargate_profile`

**Source:** `crates/winterbaume-terraform/src/converters/eks.rs`

**Inject attributes** (10): `arn`, `cluster_name`, `fargate_profile_name`, `pod_execution_role_arn`, `region`, `selector`, `status`, `subnet_ids`, `tags`, `tags_all`

**Extract attributes** (12): `arn`, `cluster_name`, `fargate_profile_name`, `id`, `labels`, `namespace`, `pod_execution_role_arn`, `selector`, `status`, `subnet_ids`, `tags`, `tags_all`

**Missing from inject** (1): `timeouts`

**Missing from extract** (1): `timeouts`

### `aws_eks_identity_provider_config`

**Source:** `crates/winterbaume-terraform/src/converters/eks.rs`

**Inject attributes** (7): `arn`, `cluster_name`, `oidc`, `region`, `status`, `tags`, `tags_all`

**Extract attributes** (10): `arn`, `client_id`, `cluster_name`, `id`, `identity_provider_config_name`, `issuer_url`, `oidc`, `status`, `tags`, `tags_all`

**Missing from inject** (1): `timeouts`

**Missing from extract** (1): `timeouts`

### `aws_eks_node_group`

**Source:** `crates/winterbaume-terraform/src/converters/eks.rs`

**Inject attributes** (16): `ami_type`, `arn`, `capacity_type`, `cluster_name`, `disk_size`, `force_update_version`, `instance_types`, `labels`, `node_group_name`, `node_role_arn`, `region`, `remote_access`, `scaling_config`, `tags`, `tags_all`, `update_config`

**Extract attributes** (17): `ami_type`, `arn`, `capacity_type`, `cluster_name`, `desired_size`, `disk_size`, `id`, `instance_types`, `labels`, `max_size`, `min_size`, `node_group_name`, `node_role_arn`, `scaling_config`, `status`, `tags`, `tags_all`

**Missing from inject** (10): `launch_template`, `node_group_name_prefix`, `node_repair_config`, `release_version`, `resources`, `status`, `subnet_ids`, `taint`, `timeouts`, `version`

**Missing from extract** (12): `force_update_version`, `launch_template`, `node_group_name_prefix`, `node_repair_config`, `release_version`, `remote_access`, `resources`, `subnet_ids`, `taint`, `timeouts`, `update_config`, `version`

### `aws_eks_pod_identity_association`

**Source:** `crates/winterbaume-terraform/src/converters/eks.rs`

**Inject attributes** (9): `association_arn`, `association_id`, `cluster_name`, `namespace`, `region`, `role_arn`, `service_account`, `tags`, `tags_all`

**Extract attributes** (9): `association_arn`, `association_id`, `cluster_name`, `id`, `namespace`, `role_arn`, `service_account`, `tags`, `tags_all`

### `aws_elasticache_cluster`

**Source:** `crates/winterbaume-terraform/src/converters/elasticache.rs`

**Inject attributes** (23): `apply_immediately`, `arn`, `auto_minor_version_upgrade`, `az_mode`, `cluster_id`, `engine`, `engine_version`, `final_snapshot_identifier`, `ip_discovery`, `log_delivery_configuration`, `maintenance_window`, `node_type`, `notification_topic_arn`, `num_cache_nodes`, `outpost_mode`, `preferred_outpost_arn`, `region`, `replication_group_id`, `snapshot_name`, `snapshot_retention_limit`, `subnet_group_name`, `tags`, `tags_all`

**Extract attributes** (22): `arn`, `availability_zone`, `cache_nodes`, `cache_status`, `cluster_address`, `cluster_id`, `configuration_endpoint`, `engine`, `engine_version`, `engine_version_actual`, `id`, `ip_discovery`, `log_delivery_configuration`, `maintenance_window`, `node_type`, `num_cache_nodes`, `port`, `replication_group_id`, `snapshot_retention_limit`, `subnet_group_name`, `tags`, `tags_all`

**Missing from inject** (14): `availability_zone`, `cache_nodes`, `cluster_address`, `configuration_endpoint`, `engine_version_actual`, `network_type`, `parameter_group_name`, `port`, `preferred_availability_zones`, `security_group_ids`, `snapshot_arns`, `snapshot_window`, `timeouts`, `transit_encryption_enabled`

**Missing from extract** (16): `apply_immediately`, `auto_minor_version_upgrade`, `az_mode`, `final_snapshot_identifier`, `network_type`, `notification_topic_arn`, `outpost_mode`, `parameter_group_name`, `preferred_availability_zones`, `preferred_outpost_arn`, `security_group_ids`, `snapshot_arns`, `snapshot_name`, `snapshot_window`, `timeouts`, `transit_encryption_enabled`

### `aws_elasticache_global_replication_group`

**Source:** `crates/winterbaume-terraform/src/converters/elasticache.rs`

**Inject attributes** (16): `arn`, `at_rest_encryption_enabled`, `auth_token_enabled`, `automatic_failover_enabled`, `cache_node_type`, `engine`, `engine_version`, `global_node_groups`, `global_replication_group_description`, `global_replication_group_id`, `global_replication_group_id_suffix`, `num_node_groups`, `parameter_group_name`, `primary_replication_group_id`, `region`, `transit_encryption_enabled`

**Extract attributes** (0): (none)

**Missing from inject** (3): `cluster_enabled`, `engine_version_actual`, `timeouts`

**Missing from extract** (18): `arn`, `at_rest_encryption_enabled`, `auth_token_enabled`, `automatic_failover_enabled`, `cache_node_type`, `cluster_enabled`, `engine`, `engine_version`, `engine_version_actual`, `global_node_groups`, `global_replication_group_description`, `global_replication_group_id`, `global_replication_group_id_suffix`, `num_node_groups`, `parameter_group_name`, `primary_replication_group_id`, `timeouts`, `transit_encryption_enabled`

### `aws_elasticache_parameter_group`

**Source:** `crates/winterbaume-terraform/src/converters/elasticache.rs`

**Inject attributes** (8): `arn`, `description`, `family`, `name`, `parameter`, `region`, `tags`, `tags_all`

**Extract attributes** (7): `arn`, `description`, `family`, `id`, `name`, `parameter`, `tags`

**Missing from extract** (1): `tags_all`

### `aws_elasticache_replication_group`

**Source:** `crates/winterbaume-terraform/src/converters/elasticache.rs`

**Inject attributes** (31): `apply_immediately`, `arn`, `at_rest_encryption_enabled`, `auth_token`, `auto_minor_version_upgrade`, `automatic_failover_enabled`, `cluster_mode`, `data_tiering_enabled`, `description`, `engine_version`, `final_snapshot_identifier`, `global_replication_group_id`, `ip_discovery`, `kms_key_id`, `log_delivery_configuration`, `maintenance_window`, `multi_az_enabled`, `network_type`, `node_type`, `notification_topic_arn`, `num_cache_clusters`, `parameter_group_name`, `preferred_cache_cluster_azs`, `region`, `replicas_per_node_group`, `replication_group_description`, `replication_group_id`, `snapshot_retention_limit`, `tags`, `tags_all`, `transit_encryption_enabled`

**Extract attributes** (28): `arn`, `at_rest_encryption_enabled`, `automatic_failover_enabled`, `cluster_enabled`, `configuration_endpoint_address`, `description`, `engine`, `engine_version`, `global_replication_group_id`, `id`, `log_delivery_configuration`, `maintenance_window`, `member_clusters`, `multi_az_enabled`, `node_type`, `num_cache_clusters`, `num_node_groups`, `parameter_group_name`, `port`, `primary_endpoint_address`, `reader_endpoint_address`, `replication_group_description`, `replication_group_id`, `snapshot_retention_limit`, `status`, `tags`, `tags_all`, `transit_encryption_enabled`

**Missing from inject** (19): `auth_token_update_strategy`, `cluster_enabled`, `configuration_endpoint_address`, `engine`, `engine_version_actual`, `member_clusters`, `num_node_groups`, `port`, `primary_endpoint_address`, `reader_endpoint_address`, `security_group_ids`, `security_group_names`, `snapshot_arns`, `snapshot_name`, `snapshot_window`, `subnet_group_name`, `timeouts`, `transit_encryption_mode`, `user_group_ids`

**Missing from extract** (23): `apply_immediately`, `auth_token`, `auth_token_update_strategy`, `auto_minor_version_upgrade`, `cluster_mode`, `data_tiering_enabled`, `engine_version_actual`, `final_snapshot_identifier`, `ip_discovery`, `kms_key_id`, `network_type`, `notification_topic_arn`, `preferred_cache_cluster_azs`, `replicas_per_node_group`, `security_group_ids`, `security_group_names`, `snapshot_arns`, `snapshot_name`, `snapshot_window`, `subnet_group_name`, `timeouts`, `transit_encryption_mode`, `user_group_ids`

### `aws_elasticache_reserved_cache_node`

**Source:** `crates/winterbaume-terraform/src/converters/elasticache.rs`

**Inject attributes** (8): `arn`, `cache_node_count`, `cache_node_type`, `region`, `reservation_id`, `reserved_cache_nodes_offering_id`, `tags`, `tags_all`

**Extract attributes** (0): (none)

**Missing from inject** (9): `duration`, `fixed_price`, `offering_type`, `product_description`, `recurring_charges`, `start_time`, `state`, `timeouts`, `usage_price`

**Missing from extract** (15): `arn`, `cache_node_count`, `cache_node_type`, `duration`, `fixed_price`, `offering_type`, `product_description`, `recurring_charges`, `reserved_cache_nodes_offering_id`, `start_time`, `state`, `tags`, `tags_all`, `timeouts`, `usage_price`

### `aws_elasticache_serverless_cache`

**Source:** `crates/winterbaume-terraform/src/converters/elasticache.rs`

**Inject attributes** (16): `arn`, `cache_usage_limits`, `daily_snapshot_time`, `description`, `engine`, `kms_key_id`, `major_engine_version`, `name`, `region`, `security_group_ids`, `snapshot_arns_to_restore`, `snapshot_retention_limit`, `subnet_ids`, `tags`, `tags_all`, `user_group_id`

**Extract attributes** (0): (none)

**Missing from inject** (6): `create_time`, `endpoint`, `full_engine_version`, `reader_endpoint`, `status`, `timeouts`

**Missing from extract** (21): `arn`, `cache_usage_limits`, `create_time`, `daily_snapshot_time`, `description`, `endpoint`, `engine`, `full_engine_version`, `kms_key_id`, `major_engine_version`, `name`, `reader_endpoint`, `security_group_ids`, `snapshot_arns_to_restore`, `snapshot_retention_limit`, `status`, `subnet_ids`, `tags`, `tags_all`, `timeouts`, `user_group_id`

### `aws_elasticache_subnet_group`

**Source:** `crates/winterbaume-terraform/src/converters/elasticache.rs`

**Inject attributes** (7): `arn`, `description`, `name`, `region`, `subnet_ids`, `tags`, `tags_all`

**Extract attributes** (6): `arn`, `description`, `id`, `name`, `subnet_ids`, `tags`

**Missing from inject** (1): `vpc_id`

**Missing from extract** (2): `tags_all`, `vpc_id`

### `aws_elasticache_user`

**Source:** `crates/winterbaume-terraform/src/converters/elasticache.rs`

**Inject attributes** (12): `arn`, `authentication_mode`, `home_directory`, `home_directory_type`, `no_password_required`, `passwords`, `region`, `role`, `server_id`, `tags`, `tags_all`, `user_name`

**Extract attributes** (10): `access_string`, `arn`, `engine`, `id`, `no_password_required`, `status`, `tags`, `tags_all`, `user_id`, `user_name`

**Missing from inject** (4): `access_string`, `engine`, `timeouts`, `user_id`

**Missing from extract** (3): `authentication_mode`, `passwords`, `timeouts`

### `aws_elasticache_user_group`

**Source:** `crates/winterbaume-terraform/src/converters/elasticache.rs`

**Inject attributes** (6): `arn`, `engine`, `region`, `tags_all`, `user_group_id`, `user_ids`

**Extract attributes** (0): (none)

**Missing from inject** (1): `tags`

**Missing from extract** (6): `arn`, `engine`, `tags`, `tags_all`, `user_group_id`, `user_ids`

### `aws_elasticache_user_group_association`

**Source:** `crates/winterbaume-terraform/src/converters/elasticache.rs`

**Inject attributes** (3): `region`, `user_group_id`, `user_id`

**Extract attributes** (0): (none)

**Missing from inject** (1): `timeouts`

**Missing from extract** (3): `timeouts`, `user_group_id`, `user_id`

### `aws_elastic_beanstalk_application`

**Source:** `crates/winterbaume-terraform/src/converters/elasticbeanstalk.rs`

**Inject attributes** (9): `appversion_lifecycle`, `arn`, `date_created`, `date_updated`, `description`, `name`, `region`, `tags`, `tags_all`

**Extract attributes** (8): `appversion_lifecycle`, `arn`, `date_created`, `date_updated`, `description`, `id`, `name`, `tags`

**Missing from extract** (1): `tags_all`

### `aws_elastic_beanstalk_environment`

**Source:** `crates/winterbaume-terraform/src/converters/elasticbeanstalk.rs`

**Inject attributes** (18): `application`, `arn`, `cname`, `date_created`, `date_updated`, `description`, `endpoint_url`, `id`, `name`, `platform_arn`, `region`, `setting`, `solution_stack_name`, `tags`, `tags_all`, `template_name`, `tier`, `version_label`

**Extract attributes** (23): `application`, `arn`, `cname`, `date_created`, `date_updated`, `description`, `endpoint_url`, `environment_id`, `health`, `id`, `name`, `platform_arn`, `setting`, `solution_stack_name`, `status`, `tags`, `tags_all`, `template_name`, `tier`, `tier_name`, `tier_type`, `version_label`, `wait_for_ready_timeout`

**Missing from inject** (10): `all_settings`, `autoscaling_groups`, `cname_prefix`, `instances`, `launch_configurations`, `load_balancers`, `poll_interval`, `queues`, `triggers`, `wait_for_ready_timeout`

**Missing from extract** (9): `all_settings`, `autoscaling_groups`, `cname_prefix`, `instances`, `launch_configurations`, `load_balancers`, `poll_interval`, `queues`, `triggers`

### `aws_elb`

**Source:** `crates/winterbaume-terraform/src/converters/elasticloadbalancing.rs`

**Inject attributes** (13): `access_logs`, `connection_draining`, `connection_draining_timeout`, `cross_zone_load_balancing`, `dns_name`, `health_check`, `idle_timeout`, `internal`, `listener`, `name`, `region`, `tags`, `tags_all`

**Extract attributes** (27): `availability_zones`, `connection_draining`, `connection_draining_timeout`, `cross_zone_load_balancing`, `dns_name`, `health_check`, `healthy_threshold`, `id`, `idle_timeout`, `instance_port`, `instance_protocol`, `instances`, `internal`, `interval`, `lb_port`, `lb_protocol`, `listener`, `name`, `security_groups`, `source_security_group`, `ssl_certificate_id`, `subnets`, `tags`, `tags_all`, `target`, `timeout`, `unhealthy_threshold`

**Missing from inject** (11): `arn`, `availability_zones`, `desync_mitigation_mode`, `instances`, `name_prefix`, `security_groups`, `source_security_group`, `source_security_group_id`, `subnets`, `timeouts`, `zone_id`

**Missing from extract** (7): `access_logs`, `arn`, `desync_mitigation_mode`, `name_prefix`, `source_security_group_id`, `timeouts`, `zone_id`

### `aws_alb`

**Source:** `crates/winterbaume-terraform/src/converters/elbv2.rs`

**Inject attributes** (0): (none)

**Extract attributes** (0): (none)

**Missing from inject** (36): `access_logs`, `arn`, `arn_suffix`, `client_keep_alive`, `connection_logs`, `customer_owned_ipv4_pool`, `desync_mitigation_mode`, `dns_name`, `dns_record_client_routing_policy`, `drop_invalid_header_fields`, `enable_cross_zone_load_balancing`, `enable_deletion_protection`, `enable_http2`, `enable_tls_version_and_cipher_suite_headers`, `enable_waf_fail_open`, `enable_xff_client_port`, `enable_zonal_shift`, `enforce_security_group_inbound_rules_on_private_link_traffic`, `idle_timeout`, `internal`, `ip_address_type`, `ipam_pools`, `load_balancer_type`, `minimum_load_balancer_capacity`, `name`, `name_prefix`, `preserve_host_header`, `security_groups`, `subnet_mapping`, `subnets`, `tags`, `tags_all`, `timeouts`, `vpc_id`, `xff_header_processing_mode`, `zone_id`

**Missing from extract** (36): `access_logs`, `arn`, `arn_suffix`, `client_keep_alive`, `connection_logs`, `customer_owned_ipv4_pool`, `desync_mitigation_mode`, `dns_name`, `dns_record_client_routing_policy`, `drop_invalid_header_fields`, `enable_cross_zone_load_balancing`, `enable_deletion_protection`, `enable_http2`, `enable_tls_version_and_cipher_suite_headers`, `enable_waf_fail_open`, `enable_xff_client_port`, `enable_zonal_shift`, `enforce_security_group_inbound_rules_on_private_link_traffic`, `idle_timeout`, `internal`, `ip_address_type`, `ipam_pools`, `load_balancer_type`, `minimum_load_balancer_capacity`, `name`, `name_prefix`, `preserve_host_header`, `security_groups`, `subnet_mapping`, `subnets`, `tags`, `tags_all`, `timeouts`, `vpc_id`, `xff_header_processing_mode`, `zone_id`

### `aws_alb_listener`

**Source:** `crates/winterbaume-terraform/src/converters/elbv2.rs`

**Inject attributes** (0): (none)

**Extract attributes** (0): (none)

**Missing from inject** (32): `alpn_policy`, `arn`, `certificate_arn`, `default_action`, `load_balancer_arn`, `mutual_authentication`, `port`, `protocol`, `routing_http_request_x_amzn_mtls_clientcert_header_name`, `routing_http_request_x_amzn_mtls_clientcert_issuer_header_name`, `routing_http_request_x_amzn_mtls_clientcert_leaf_header_name`, `routing_http_request_x_amzn_mtls_clientcert_serial_number_header_name`, `routing_http_request_x_amzn_mtls_clientcert_subject_header_name`, `routing_http_request_x_amzn_mtls_clientcert_validity_header_name`, `routing_http_request_x_amzn_tls_cipher_suite_header_name`, `routing_http_request_x_amzn_tls_version_header_name`, `routing_http_response_access_control_allow_credentials_header_value`, `routing_http_response_access_control_allow_headers_header_value`, `routing_http_response_access_control_allow_methods_header_value`, `routing_http_response_access_control_allow_origin_header_value`, `routing_http_response_access_control_expose_headers_header_value`, `routing_http_response_access_control_max_age_header_value`, `routing_http_response_content_security_policy_header_value`, `routing_http_response_server_enabled`, `routing_http_response_strict_transport_security_header_value`, `routing_http_response_x_content_type_options_header_value`, `routing_http_response_x_frame_options_header_value`, `ssl_policy`, `tags`, `tags_all`, `tcp_idle_timeout_seconds`, `timeouts`

**Missing from extract** (32): `alpn_policy`, `arn`, `certificate_arn`, `default_action`, `load_balancer_arn`, `mutual_authentication`, `port`, `protocol`, `routing_http_request_x_amzn_mtls_clientcert_header_name`, `routing_http_request_x_amzn_mtls_clientcert_issuer_header_name`, `routing_http_request_x_amzn_mtls_clientcert_leaf_header_name`, `routing_http_request_x_amzn_mtls_clientcert_serial_number_header_name`, `routing_http_request_x_amzn_mtls_clientcert_subject_header_name`, `routing_http_request_x_amzn_mtls_clientcert_validity_header_name`, `routing_http_request_x_amzn_tls_cipher_suite_header_name`, `routing_http_request_x_amzn_tls_version_header_name`, `routing_http_response_access_control_allow_credentials_header_value`, `routing_http_response_access_control_allow_headers_header_value`, `routing_http_response_access_control_allow_methods_header_value`, `routing_http_response_access_control_allow_origin_header_value`, `routing_http_response_access_control_expose_headers_header_value`, `routing_http_response_access_control_max_age_header_value`, `routing_http_response_content_security_policy_header_value`, `routing_http_response_server_enabled`, `routing_http_response_strict_transport_security_header_value`, `routing_http_response_x_content_type_options_header_value`, `routing_http_response_x_frame_options_header_value`, `ssl_policy`, `tags`, `tags_all`, `tcp_idle_timeout_seconds`, `timeouts`

### `aws_alb_listener_certificate`

**Source:** `crates/winterbaume-terraform/src/converters/elbv2.rs`

**Inject attributes** (0): (none)

**Extract attributes** (0): (none)

**Missing from inject** (2): `certificate_arn`, `listener_arn`

**Missing from extract** (2): `certificate_arn`, `listener_arn`

### `aws_alb_listener_rule`

**Source:** `crates/winterbaume-terraform/src/converters/elbv2.rs`

**Inject attributes** (0): (none)

**Extract attributes** (0): (none)

**Missing from inject** (7): `action`, `arn`, `condition`, `listener_arn`, `priority`, `tags`, `tags_all`

**Missing from extract** (7): `action`, `arn`, `condition`, `listener_arn`, `priority`, `tags`, `tags_all`

### `aws_alb_target_group`

**Source:** `crates/winterbaume-terraform/src/converters/elbv2.rs`

**Inject attributes** (0): (none)

**Extract attributes** (0): (none)

**Missing from inject** (27): `arn`, `arn_suffix`, `connection_termination`, `deregistration_delay`, `health_check`, `ip_address_type`, `lambda_multi_value_headers_enabled`, `load_balancer_arns`, `load_balancing_algorithm_type`, `load_balancing_anomaly_mitigation`, `load_balancing_cross_zone_enabled`, `name`, `name_prefix`, `port`, `preserve_client_ip`, `protocol`, `protocol_version`, `proxy_protocol_v2`, `slow_start`, `stickiness`, `tags`, `tags_all`, `target_failover`, `target_group_health`, `target_health_state`, `target_type`, `vpc_id`

**Missing from extract** (27): `arn`, `arn_suffix`, `connection_termination`, `deregistration_delay`, `health_check`, `ip_address_type`, `lambda_multi_value_headers_enabled`, `load_balancer_arns`, `load_balancing_algorithm_type`, `load_balancing_anomaly_mitigation`, `load_balancing_cross_zone_enabled`, `name`, `name_prefix`, `port`, `preserve_client_ip`, `protocol`, `protocol_version`, `proxy_protocol_v2`, `slow_start`, `stickiness`, `tags`, `tags_all`, `target_failover`, `target_group_health`, `target_health_state`, `target_type`, `vpc_id`

### `aws_alb_target_group_attachment`

**Source:** `crates/winterbaume-terraform/src/converters/elbv2.rs`

**Inject attributes** (0): (none)

**Extract attributes** (0): (none)

**Missing from inject** (4): `availability_zone`, `port`, `target_group_arn`, `target_id`

**Missing from extract** (4): `availability_zone`, `port`, `target_group_arn`, `target_id`

### `aws_lb`

**Source:** `crates/winterbaume-terraform/src/converters/elbv2.rs`

**Inject attributes** (28): `access_logs`, `arn`, `client_keep_alive`, `connection_logs`, `desync_mitigation_mode`, `dns_name`, `drop_invalid_header_fields`, `enable_cross_zone_load_balancing`, `enable_deletion_protection`, `enable_http2`, `enable_tls_version_and_cipher_suite_headers`, `enable_waf_fail_open`, `idle_timeout`, `internal`, `ip_address_type`, `ipam_pools`, `load_balancer_type`, `minimum_load_balancer_capacity`, `name`, `preserve_host_header`, `region`, `scheme`, `security_groups`, `subnet_mapping`, `subnets`, `tags`, `tags_all`, `vpc_id`

**Extract attributes** (31): `access_logs`, `arn`, `arn_suffix`, `availability_zone`, `bucket`, `connection_logs`, `created_time`, `desync_mitigation_mode`, `dns_name`, `enable_deletion_protection`, `enable_http2`, `enabled`, `id`, `idle_timeout`, `internal`, `ip_address_type`, `ipam_pools`, `load_balancer_type`, `minimum_load_balancer_capacity`, `name`, `prefix`, `scheme`, `security_groups`, `state`, `subnet_id`, `subnet_mapping`, `subnets`, `tags`, `tags_all`, `vpc_id`, `zone_id`

**Missing from inject** (10): `arn_suffix`, `customer_owned_ipv4_pool`, `dns_record_client_routing_policy`, `enable_xff_client_port`, `enable_zonal_shift`, `enforce_security_group_inbound_rules_on_private_link_traffic`, `name_prefix`, `timeouts`, `xff_header_processing_mode`, `zone_id`

**Missing from extract** (14): `client_keep_alive`, `customer_owned_ipv4_pool`, `dns_record_client_routing_policy`, `drop_invalid_header_fields`, `enable_cross_zone_load_balancing`, `enable_tls_version_and_cipher_suite_headers`, `enable_waf_fail_open`, `enable_xff_client_port`, `enable_zonal_shift`, `enforce_security_group_inbound_rules_on_private_link_traffic`, `name_prefix`, `preserve_host_header`, `timeouts`, `xff_header_processing_mode`

### `aws_lb_cookie_stickiness_policy`

**Source:** `crates/winterbaume-terraform/src/converters/elbv2.rs`

**Inject attributes** (5): `cookie_expiration_period`, `lb_port`, `load_balancer`, `name`, `region`

**Extract attributes** (0): (none)

**Missing from extract** (4): `cookie_expiration_period`, `lb_port`, `load_balancer`, `name`

### `aws_lb_listener`

**Source:** `crates/winterbaume-terraform/src/converters/elbv2.rs`

**Inject attributes** (22): `alpn_policy`, `arn`, `certificate_arn`, `default_action`, `load_balancer_arn`, `mutual_authentication`, `port`, `protocol`, `region`, `routing_http_request_x_amzn_mtls_clientcert_header_name`, `routing_http_request_x_amzn_mtls_clientcert_issuer_header_name`, `routing_http_request_x_amzn_mtls_clientcert_serial_number_header_name`, `routing_http_request_x_amzn_mtls_clientcert_subject_header_name`, `routing_http_request_x_amzn_mtls_clientcert_validity_header_name`, `routing_http_request_x_amzn_tls_cipher_suite_header_name`, `routing_http_request_x_amzn_tls_version_header_name`, `routing_http_response_access_control_allow_credentials_header_value`, `routing_http_response_server_enabled`, `ssl_policy`, `tags`, `tags_all`, `tcp_idle_timeout_seconds`

**Extract attributes** (20): `alpn_policy`, `arn`, `arn_suffix`, `certificate_arn`, `default_action`, `id`, `load_balancer_arn`, `mutual_authentication`, `port`, `protocol`, `routing_http_request_x_amzn_mtls_clientcert_header_name`, `routing_http_request_x_amzn_mtls_clientcert_issuer_header_name`, `routing_http_request_x_amzn_mtls_clientcert_leaf_header_name`, `routing_http_response_server_enabled`, `ssl_policy`, `tags`, `tags_all`, `target_group_arn`, `tcp_idle_timeout_seconds`, `type`

**Missing from inject** (11): `routing_http_request_x_amzn_mtls_clientcert_leaf_header_name`, `routing_http_response_access_control_allow_headers_header_value`, `routing_http_response_access_control_allow_methods_header_value`, `routing_http_response_access_control_allow_origin_header_value`, `routing_http_response_access_control_expose_headers_header_value`, `routing_http_response_access_control_max_age_header_value`, `routing_http_response_content_security_policy_header_value`, `routing_http_response_strict_transport_security_header_value`, `routing_http_response_x_content_type_options_header_value`, `routing_http_response_x_frame_options_header_value`, `timeouts`

**Missing from extract** (16): `routing_http_request_x_amzn_mtls_clientcert_serial_number_header_name`, `routing_http_request_x_amzn_mtls_clientcert_subject_header_name`, `routing_http_request_x_amzn_mtls_clientcert_validity_header_name`, `routing_http_request_x_amzn_tls_cipher_suite_header_name`, `routing_http_request_x_amzn_tls_version_header_name`, `routing_http_response_access_control_allow_credentials_header_value`, `routing_http_response_access_control_allow_headers_header_value`, `routing_http_response_access_control_allow_methods_header_value`, `routing_http_response_access_control_allow_origin_header_value`, `routing_http_response_access_control_expose_headers_header_value`, `routing_http_response_access_control_max_age_header_value`, `routing_http_response_content_security_policy_header_value`, `routing_http_response_strict_transport_security_header_value`, `routing_http_response_x_content_type_options_header_value`, `routing_http_response_x_frame_options_header_value`, `timeouts`

### `aws_lb_listener_certificate`

**Source:** `crates/winterbaume-terraform/src/converters/elbv2.rs`

**Inject attributes** (3): `certificate_arn`, `listener_arn`, `region`

**Extract attributes** (3): `certificate_arn`, `id`, `listener_arn`

### `aws_lb_listener_rule`

**Source:** `crates/winterbaume-terraform/src/converters/elbv2.rs`

**Inject attributes** (8): `action`, `arn`, `condition`, `listener_arn`, `priority`, `region`, `tags`, `tags_all`

**Extract attributes** (12): `action`, `arn`, `condition`, `field`, `id`, `listener_arn`, `priority`, `tags`, `tags_all`, `target_group_arn`, `type`, `values`

### `aws_lb_ssl_negotiation_policy`

**Source:** `crates/winterbaume-terraform/src/converters/elbv2.rs`

**Inject attributes** (4): `lb_port`, `load_balancer`, `name`, `region`

**Extract attributes** (0): (none)

**Missing from inject** (2): `attribute`, `triggers`

**Missing from extract** (5): `attribute`, `lb_port`, `load_balancer`, `name`, `triggers`

### `aws_lb_target_group`

**Source:** `crates/winterbaume-terraform/src/converters/elbv2.rs`

**Inject attributes** (21): `arn`, `deregistration_delay`, `health_check`, `lambda_multi_value_headers_enabled`, `load_balancing_algorithm_type`, `load_balancing_anomaly_mitigation`, `load_balancing_cross_zone_enabled`, `name`, `port`, `protocol`, `protocol_version`, `region`, `slow_start`, `stickiness`, `tags`, `tags_all`, `target_failover`, `target_group_health`, `target_health_state`, `target_type`, `vpc_id`

**Extract attributes** (27): `arn`, `arn_suffix`, `cookie_duration`, `deregistration_delay`, `enabled`, `health_check`, `healthy_threshold`, `id`, `interval`, `load_balancing_algorithm_type`, `matcher`, `name`, `path`, `port`, `protocol`, `protocol_version`, `stickiness`, `tags`, `tags_all`, `target_failover`, `target_group_health`, `target_health_state`, `target_type`, `timeout`, `type`, `unhealthy_threshold`, `vpc_id`

**Missing from inject** (7): `arn_suffix`, `connection_termination`, `ip_address_type`, `load_balancer_arns`, `name_prefix`, `preserve_client_ip`, `proxy_protocol_v2`

**Missing from extract** (10): `connection_termination`, `ip_address_type`, `lambda_multi_value_headers_enabled`, `load_balancer_arns`, `load_balancing_anomaly_mitigation`, `load_balancing_cross_zone_enabled`, `name_prefix`, `preserve_client_ip`, `proxy_protocol_v2`, `slow_start`

### `aws_lb_target_group_attachment`

**Source:** `crates/winterbaume-terraform/src/converters/elbv2.rs`

**Inject attributes** (5): `availability_zone`, `port`, `region`, `target_group_arn`, `target_id`

**Extract attributes** (5): `availability_zone`, `id`, `port`, `target_group_arn`, `target_id`

### `aws_lb_trust_store`

**Source:** `crates/winterbaume-terraform/src/converters/elbv2.rs`

**Inject attributes** (8): `arn`, `ca_certificates_bundle_s3_bucket`, `ca_certificates_bundle_s3_key`, `ca_certificates_bundle_s3_object_version`, `name`, `region`, `tags`, `tags_all`

**Extract attributes** (8): `arn`, `id`, `name`, `number_of_ca_certificates`, `status`, `tags`, `tags_all`, `total_revoked_entries`

**Missing from inject** (3): `arn_suffix`, `name_prefix`, `timeouts`

**Missing from extract** (6): `arn_suffix`, `ca_certificates_bundle_s3_bucket`, `ca_certificates_bundle_s3_key`, `ca_certificates_bundle_s3_object_version`, `name_prefix`, `timeouts`

### `aws_lb_trust_store_revocation`

**Source:** `crates/winterbaume-terraform/src/converters/elbv2.rs`

**Inject attributes** (5): `region`, `revocations_s3_bucket`, `revocations_s3_key`, `revocations_s3_object_version`, `trust_store_arn`

**Extract attributes** (3): `id`, `revocation_id`, `trust_store_arn`

**Missing from inject** (2): `revocation_id`, `timeouts`

**Missing from extract** (4): `revocations_s3_bucket`, `revocations_s3_key`, `revocations_s3_object_version`, `timeouts`

### `aws_emr_block_public_access_configuration`

**Source:** `crates/winterbaume-terraform/src/converters/emr.rs`

**Inject attributes** (3): `block_public_security_group_rules`, `permitted_public_security_group_rule_range`, `region`

**Extract attributes** (5): `block_public_security_group_rules`, `id`, `max_range`, `min_range`, `permitted_public_security_group_rule_range`

### `aws_emr_cluster`

**Source:** `crates/winterbaume-terraform/src/converters/emr.rs`

**Inject attributes** (34): `acl_name`, `additional_info`, `arn`, `auto_minor_version_upgrade`, `auto_termination_policy`, `bootstrap_action`, `configurations`, `configurations_json`, `core_instance_group`, `custom_ami_id`, `description`, `ebs_root_volume_size`, `ec2_attributes`, `engine`, `engine_version`, `keep_job_flow_alive_when_no_steps`, `kerberos_attributes`, `maintenance_window`, `master_instance_group`, `name`, `node_type`, `num_replicas_per_shard`, `num_shards`, `parameter_group_name`, `region`, `snapshot_retention_limit`, `snapshot_window`, `step_concurrency_level`, `subnet_group_name`, `tags`, `tags_all`, `termination_protection`, `tls_enabled`, `visible_to_all_users`

**Extract attributes** (39): `applications`, `args`, `arn`, `auto_scaling_role`, `auto_termination_policy`, `bid_price`, `bootstrap_action`, `cluster_arn`, `cluster_state`, `core_instance_fleet`, `core_instance_group`, `creation_date_time`, `ec2_attributes`, `id`, `idle_timeout`, `instance_count`, `instance_type`, `job_flow_role`, `keep_job_flow_alive_when_no_steps`, `kerberos_attributes`, `log_uri`, `master_instance_fleet`, `master_instance_group`, `master_public_dns_name`, `name`, `path`, `release_label`, `scale_down_behavior`, `security_configuration`, `service_role`, `status`, `step`, `step_concurrency_level`, `tags`, `tags_all`, `target_on_demand_capacity`, `target_spot_capacity`, `termination_protection`, `visible_to_all_users`

**Missing from inject** (16): `applications`, `autoscaling_role`, `cluster_state`, `core_instance_fleet`, `list_steps_states`, `log_encryption_kms_key_id`, `log_uri`, `master_instance_fleet`, `master_public_dns`, `placement_group_config`, `release_label`, `scale_down_behavior`, `security_configuration`, `service_role`, `step`, `unhealthy_node_replacement`

**Missing from extract** (11): `additional_info`, `autoscaling_role`, `configurations`, `configurations_json`, `custom_ami_id`, `ebs_root_volume_size`, `list_steps_states`, `log_encryption_kms_key_id`, `master_public_dns`, `placement_group_config`, `unhealthy_node_replacement`

### `aws_emr_instance_fleet`

**Source:** `crates/winterbaume-terraform/src/converters/emr.rs`

**Inject attributes** (7): `cluster_id`, `instance_type_configs`, `launch_specifications`, `name`, `region`, `target_on_demand_capacity`, `target_spot_capacity`

**Extract attributes** (10): `cluster_id`, `id`, `instance_type_configs`, `launch_specifications`, `name`, `provisioned_on_demand_capacity`, `provisioned_spot_capacity`, `status`, `target_on_demand_capacity`, `target_spot_capacity`

**Missing from inject** (2): `provisioned_on_demand_capacity`, `provisioned_spot_capacity`

### `aws_emr_instance_group`

**Source:** `crates/winterbaume-terraform/src/converters/emr.rs`

**Inject attributes** (10): `autoscaling_policy`, `bid_price`, `cluster_id`, `configurations_json`, `ebs_config`, `ebs_optimized`, `instance_count`, `instance_type`, `name`, `region`

**Extract attributes** (10): `bid_price`, `cluster_id`, `ebs_config`, `ebs_optimized`, `id`, `instance_count`, `instance_type`, `name`, `running_instance_count`, `status`

**Missing from inject** (2): `running_instance_count`, `status`

**Missing from extract** (2): `autoscaling_policy`, `configurations_json`

### `aws_emr_managed_scaling_policy`

**Source:** `crates/winterbaume-terraform/src/converters/emr.rs`

**Inject attributes** (5): `cluster_id`, `compute_limits`, `region`, `scaling_strategy`, `utilization_performance_index`

**Extract attributes** (10): `cluster_id`, `compute_limits`, `id`, `maximum_capacity_units`, `maximum_core_capacity_units`, `maximum_ondemand_capacity_units`, `minimum_capacity_units`, `scaling_strategy`, `unit_type`, `utilization_performance_index`

### `aws_emr_security_configuration`

**Source:** `crates/winterbaume-terraform/src/converters/emr.rs`

**Inject attributes** (2): `name`, `region`

**Extract attributes** (4): `configuration`, `creation_date`, `id`, `name`

**Missing from inject** (3): `configuration`, `creation_date`, `name_prefix`

**Missing from extract** (1): `name_prefix`

### `aws_emr_studio`

**Source:** `crates/winterbaume-terraform/src/converters/emr.rs`

**Inject attributes** (18): `arn`, `auth_mode`, `default_s3_location`, `description`, `encryption_key_arn`, `engine_security_group_id`, `idp_auth_url`, `idp_relay_state_parameter_name`, `name`, `region`, `service_role`, `subnet_ids`, `tags`, `tags_all`, `url`, `user_role`, `vpc_id`, `workspace_security_group_id`

**Extract attributes** (15): `arn`, `auth_mode`, `default_s3_location`, `description`, `engine_security_group_id`, `id`, `name`, `service_role`, `subnet_ids`, `tags`, `tags_all`, `url`, `user_role`, `vpc_id`, `workspace_security_group_id`

**Missing from extract** (3): `encryption_key_arn`, `idp_auth_url`, `idp_relay_state_parameter_name`

### `aws_emr_studio_session_mapping`

**Source:** `crates/winterbaume-terraform/src/converters/emr.rs`

**Inject attributes** (6): `identity_id`, `identity_name`, `identity_type`, `region`, `session_policy_arn`, `studio_id`

**Extract attributes** (6): `id`, `identity_id`, `identity_name`, `identity_type`, `session_policy_arn`, `studio_id`

### `aws_emrcontainers_virtual_cluster`

**Source:** `crates/winterbaume-terraform/src/converters/emrcontainers.rs`

**Inject attributes** (5): `arn`, `container_provider`, `name`, `region`, `tags`

**Extract attributes** (8): `arn`, `container_provider`, `eks_info`, `id`, `name`, `namespace`, `tags`, `type`

**Missing from inject** (2): `tags_all`, `timeouts`

**Missing from extract** (2): `tags_all`, `timeouts`

### `aws_emrserverless_application`

**Source:** `crates/winterbaume-terraform/src/converters/emrserverless.rs`

**Inject attributes** (15): `arn`, `auto_start_configuration`, `auto_stop_configuration`, `date_created`, `date_updated`, `description`, `image_configuration`, `initial_capacity`, `interactive_configuration`, `maximum_capacity`, `name`, `network_configuration`, `region`, `tags`, `tags_all`

**Extract attributes** (21): `arn`, `auto_start_configuration`, `auto_stop_configuration`, `cpu`, `disk`, `enabled`, `id`, `idle_timeout_minutes`, `image_uri`, `initial_capacity_config`, `initial_capacity_type`, `livy_endpoint_enabled`, `memory`, `name`, `release_label`, `security_group_ids`, `studio_enabled`, `subnet_ids`, `tags`, `type`, `worker_count`

**Missing from inject** (3): `architecture`, `release_label`, `type`

**Missing from extract** (7): `architecture`, `image_configuration`, `initial_capacity`, `interactive_configuration`, `maximum_capacity`, `network_configuration`, `tags_all`

### `aws_cloudwatch_event_api_destination`

**Source:** `crates/winterbaume-terraform/src/converters/events.rs`

**Inject attributes** (8): `arn`, `connection_arn`, `description`, `http_method`, `invocation_endpoint`, `invocation_rate_limit_per_second`, `name`, `region`

**Extract attributes** (8): `arn`, `connection_arn`, `description`, `http_method`, `id`, `invocation_endpoint`, `invocation_rate_limit_per_second`, `name`

### `aws_cloudwatch_event_archive`

**Source:** `crates/winterbaume-terraform/src/converters/events.rs`

**Inject attributes** (7): `arn`, `description`, `event_pattern`, `event_source_arn`, `name`, `region`, `retention_days`

**Extract attributes** (7): `arn`, `description`, `event_pattern`, `event_source_arn`, `id`, `name`, `retention_days`

### `aws_cloudwatch_event_bus`

**Source:** `crates/winterbaume-terraform/src/converters/events.rs`

**Inject attributes** (8): `arn`, `dead_letter_config`, `event_source_name`, `name`, `policy`, `region`, `tags`, `tags_all`

**Extract attributes** (7): `arn`, `dead_letter_config`, `event_source_name`, `id`, `name`, `policy`, `tags`

**Missing from inject** (2): `description`, `kms_key_identifier`

**Missing from extract** (3): `description`, `kms_key_identifier`, `tags_all`

### `aws_cloudwatch_event_bus_policy`

**Source:** `crates/winterbaume-terraform/src/converters/events.rs`

**Inject attributes** (3): `event_bus_name`, `policy`, `region`

**Extract attributes** (3): `event_bus_name`, `id`, `policy`

### `aws_cloudwatch_event_connection`

**Source:** `crates/winterbaume-terraform/src/converters/events.rs`

**Inject attributes** (6): `arn`, `auth_parameters`, `authorization_type`, `description`, `name`, `region`

**Extract attributes** (6): `arn`, `auth_parameters`, `authorization_type`, `description`, `id`, `name`

**Missing from inject** (3): `invocation_connectivity_parameters`, `kms_key_identifier`, `secret_arn`

**Missing from extract** (3): `invocation_connectivity_parameters`, `kms_key_identifier`, `secret_arn`

### `aws_cloudwatch_event_endpoint`

**Source:** `crates/winterbaume-terraform/src/converters/events.rs`

**Inject attributes** (10): `arn`, `description`, `endpoint_id`, `endpoint_url`, `event_bus`, `name`, `region`, `replication_config`, `role_arn`, `routing_config`

**Extract attributes** (11): `arn`, `description`, `endpoint_id`, `endpoint_url`, `event_bus`, `event_bus_arn`, `id`, `name`, `replication_config`, `role_arn`, `routing_config`

### `aws_cloudwatch_event_permission`

**Source:** `crates/winterbaume-terraform/src/converters/events.rs`

**Inject attributes** (5): `action`, `event_bus_name`, `principal`, `region`, `statement_id`

**Extract attributes** (6): `)
                                .and_then(|rest| rest.strip_suffix(`, `action`, `event_bus_name`, `id`, `principal`, `statement_id`

**Missing from inject** (1): `condition`

**Missing from extract** (1): `condition`

### `aws_cloudwatch_event_rule`

**Source:** `crates/winterbaume-terraform/src/converters/events.rs`

**Inject attributes** (10): `arn`, `description`, `event_bus_name`, `event_pattern`, `name`, `region`, `schedule_expression`, `state`, `tags`, `tags_all`

**Extract attributes** (9): `arn`, `description`, `event_bus_name`, `event_pattern`, `id`, `name`, `schedule_expression`, `state`, `tags`

**Missing from inject** (4): `force_destroy`, `is_enabled`, `name_prefix`, `role_arn`

**Missing from extract** (5): `force_destroy`, `is_enabled`, `name_prefix`, `role_arn`, `tags_all`

### `aws_cloudwatch_event_target`

**Source:** `crates/winterbaume-terraform/src/converters/events.rs`

**Inject attributes** (9): `arn`, `event_bus_name`, `input`, `input_path`, `region`, `retry_policy`, `rule`, `tags_all`, `target_id`

**Extract attributes** (11): `arn`, `batch_target`, `dead_letter_config`, `ecs_target`, `event_bus_name`, `id`, `input`, `input_path`, `retry_policy`, `rule`, `target_id`

**Missing from inject** (13): `appsync_target`, `batch_target`, `dead_letter_config`, `ecs_target`, `force_destroy`, `http_target`, `input_transformer`, `kinesis_target`, `redshift_target`, `role_arn`, `run_command_targets`, `sagemaker_pipeline_target`, `sqs_target`

**Missing from extract** (10): `appsync_target`, `force_destroy`, `http_target`, `input_transformer`, `kinesis_target`, `redshift_target`, `role_arn`, `run_command_targets`, `sagemaker_pipeline_target`, `sqs_target`

### `aws_kinesis_firehose_delivery_stream`

**Source:** `crates/winterbaume-terraform/src/converters/firehose.rs`

**Inject attributes** (6): `arn`, `destination`, `name`, `region`, `tags`, `version_id`

**Extract attributes** (12): `arn`, `destination`, `destination_id`, `elasticsearch_configuration`, `enabled`, `id`, `kinesis_source_configuration`, `name`, `server_side_encryption`, `tags`, `tags_all`, `version_id`

**Missing from inject** (15): `destination_id`, `elasticsearch_configuration`, `extended_s3_configuration`, `http_endpoint_configuration`, `iceberg_configuration`, `kinesis_source_configuration`, `msk_source_configuration`, `opensearch_configuration`, `opensearchserverless_configuration`, `redshift_configuration`, `server_side_encryption`, `snowflake_configuration`, `splunk_configuration`, `tags_all`, `timeouts`

**Missing from extract** (10): `extended_s3_configuration`, `http_endpoint_configuration`, `iceberg_configuration`, `msk_source_configuration`, `opensearch_configuration`, `opensearchserverless_configuration`, `redshift_configuration`, `snowflake_configuration`, `splunk_configuration`, `timeouts`

### `aws_fis_experiment_template`

**Source:** `crates/winterbaume-terraform/src/converters/fis.rs`

**Inject attributes** (8): `action`, `arn`, `description`, `region`, `role_arn`, `stop_condition`, `tags`, `target`

**Extract attributes** (14): `action`, `action_id`, `arn`, `description`, `id`, `name`, `resource_arns`, `role_arn`, `selection_mode`, `source`, `stop_condition`, `tags`, `target`, `value`

**Missing from inject** (5): `experiment_options`, `experiment_report_configuration`, `log_configuration`, `tags_all`, `timeouts`

**Missing from extract** (5): `experiment_options`, `experiment_report_configuration`, `log_configuration`, `tags_all`, `timeouts`

### `aws_fsx_backup`

**Source:** `crates/winterbaume-terraform/src/converters/fsx.rs`

**Inject attributes** (9): `arn`, `file_system_id`, `kms_key_id`, `owner_id`, `region`, `tags`, `tags_all`, `type`, `volume_id`

**Extract attributes** (5): `arn`, `file_system_id`, `id`, `tags`, `tags_all`

**Missing from inject** (1): `timeouts`

**Missing from extract** (5): `kms_key_id`, `owner_id`, `timeouts`, `type`, `volume_id`

### `aws_fsx_data_repository_association`

**Source:** `crates/winterbaume-terraform/src/converters/fsx.rs`

**Inject attributes** (8): `arn`, `association_id`, `batch_import_meta_data_on_create`, `data_repository_path`, `delete_data_in_filesystem`, `file_system_id`, `file_system_path`, `imported_file_chunk_size`

**Extract attributes** (0): (none)

**Missing from inject** (4): `s3`, `tags`, `tags_all`, `timeouts`

**Missing from extract** (12): `arn`, `association_id`, `batch_import_meta_data_on_create`, `data_repository_path`, `delete_data_in_filesystem`, `file_system_id`, `file_system_path`, `imported_file_chunk_size`, `s3`, `tags`, `tags_all`, `timeouts`

### `aws_fsx_file_cache`

**Source:** `crates/winterbaume-terraform/src/converters/fsx.rs`

**Inject attributes** (12): `arn`, `copy_tags_to_data_repository_associations`, `creation_time`, `dns_name`, `file_cache_id`, `file_cache_type`, `file_cache_type_version`, `kms_key_id`, `lifecycle`, `owner_id`, `storage_capacity`, `vpc_id`

**Extract attributes** (0): (none)

**Missing from inject** (9): `data_repository_association`, `data_repository_association_ids`, `lustre_configuration`, `network_interface_ids`, `security_group_ids`, `subnet_ids`, `tags`, `tags_all`, `timeouts`

**Missing from extract** (19): `arn`, `copy_tags_to_data_repository_associations`, `data_repository_association`, `data_repository_association_ids`, `dns_name`, `file_cache_id`, `file_cache_type`, `file_cache_type_version`, `kms_key_id`, `lustre_configuration`, `network_interface_ids`, `owner_id`, `security_group_ids`, `storage_capacity`, `subnet_ids`, `tags`, `tags_all`, `timeouts`, `vpc_id`

### `aws_fsx_lustre_file_system`

**Source:** `crates/winterbaume-terraform/src/converters/fsx.rs`

**Inject attributes** (21): `arn`, `automatic_backup_retention_days`, `copy_tags_to_backups`, `creation_time`, `daily_automatic_backup_start_time`, `deployment_type`, `dns_name`, `kms_key_id`, `lifecycle`, `lifecycle_status`, `lustre_configuration`, `owner_id`, `region`, `security_group_ids`, `storage_capacity`, `storage_type`, `subnet_ids`, `tags`, `tags_all`, `vpc_id`, `weekly_maintenance_start_time`

**Extract attributes** (28): `arn`, `automatic_backup_retention_days`, `copy_tags_to_backups`, `creation_time`, `daily_automatic_backup_start_time`, `data_read_cache_configuration`, `deployment_type`, `dns_name`, `file_system_type`, `id`, `kms_key_id`, `lifecycle`, `lifecycle_status`, `log_configuration`, `lustre_configuration`, `metadata_configuration`, `mount_name`, `network_interface_ids`, `owner_id`, `root_squash_configuration`, `security_group_ids`, `storage_capacity`, `storage_type`, `subnet_ids`, `tags`, `tags_all`, `vpc_id`, `weekly_maintenance_start_time`

**Missing from inject** (20): `auto_import_policy`, `backup_id`, `data_compression_type`, `data_read_cache_configuration`, `drive_cache_type`, `efa_enabled`, `export_path`, `file_system_type_version`, `final_backup_tags`, `import_path`, `imported_file_chunk_size`, `log_configuration`, `metadata_configuration`, `mount_name`, `network_interface_ids`, `per_unit_storage_throughput`, `root_squash_configuration`, `skip_final_backup`, `throughput_capacity`, `timeouts`

**Missing from extract** (14): `auto_import_policy`, `backup_id`, `data_compression_type`, `drive_cache_type`, `efa_enabled`, `export_path`, `file_system_type_version`, `final_backup_tags`, `import_path`, `imported_file_chunk_size`, `per_unit_storage_throughput`, `skip_final_backup`, `throughput_capacity`, `timeouts`

### `aws_fsx_ontap_file_system`

**Source:** `crates/winterbaume-terraform/src/converters/fsx.rs`

**Inject attributes** (25): `arn`, `automatic_backup_retention_days`, `creation_time`, `daily_automatic_backup_start_time`, `deployment_type`, `dns_name`, `endpoint_ip_address_range`, `fsx_admin_password`, `ha_pairs`, `kms_key_id`, `lifecycle`, `lifecycle_status`, `owner_id`, `preferred_subnet_id`, `region`, `security_group_ids`, `storage_capacity`, `storage_type`, `subnet_ids`, `tags`, `tags_all`, `throughput_capacity`, `throughput_capacity_per_ha_pair`, `vpc_id`, `weekly_maintenance_start_time`

**Extract attributes** (25): `arn`, `automatic_backup_retention_days`, `creation_time`, `daily_automatic_backup_start_time`, `deployment_type`, `dns_name`, `endpoint_ip_address_range`, `file_system_type`, `ha_pairs`, `id`, `kms_key_id`, `lifecycle`, `lifecycle_status`, `owner_id`, `preferred_subnet_id`, `security_group_ids`, `storage_capacity`, `storage_type`, `subnet_ids`, `tags`, `tags_all`, `throughput_capacity`, `throughput_capacity_per_ha_pair`, `vpc_id`, `weekly_maintenance_start_time`

**Missing from inject** (5): `disk_iops_configuration`, `endpoints`, `network_interface_ids`, `route_table_ids`, `timeouts`

**Missing from extract** (6): `disk_iops_configuration`, `endpoints`, `fsx_admin_password`, `network_interface_ids`, `route_table_ids`, `timeouts`

### `aws_fsx_ontap_storage_virtual_machine`

**Source:** `crates/winterbaume-terraform/src/converters/fsx.rs`

**Inject attributes** (8): `arn`, `file_system_id`, `lifecycle_status`, `name`, `root_volume_security_style`, `subtype`, `svm_admin_password`, `uuid`

**Extract attributes** (0): (none)

**Missing from inject** (5): `active_directory_configuration`, `endpoints`, `tags`, `tags_all`, `timeouts`

**Missing from extract** (12): `active_directory_configuration`, `arn`, `endpoints`, `file_system_id`, `name`, `root_volume_security_style`, `subtype`, `svm_admin_password`, `tags`, `tags_all`, `timeouts`, `uuid`

### `aws_fsx_ontap_volume`

**Source:** `crates/winterbaume-terraform/src/converters/fsx.rs`

**Inject attributes** (16): `arn`, `bypass_snaplock_enterprise_retention`, `copy_tags_to_backups`, `file_system_id`, `junction_path`, `name`, `ontap_volume_type`, `security_style`, `size_in_bytes`, `size_in_megabytes`, `skip_final_backup`, `snapshot_policy`, `storage_efficiency_enabled`, `storage_virtual_machine_id`, `uuid`, `volume_type`

**Extract attributes** (0): (none)

**Missing from inject** (9): `aggregate_configuration`, `final_backup_tags`, `flexcache_endpoint_type`, `snaplock_configuration`, `tags`, `tags_all`, `tiering_policy`, `timeouts`, `volume_style`

**Missing from extract** (25): `aggregate_configuration`, `arn`, `bypass_snaplock_enterprise_retention`, `copy_tags_to_backups`, `file_system_id`, `final_backup_tags`, `flexcache_endpoint_type`, `junction_path`, `name`, `ontap_volume_type`, `security_style`, `size_in_bytes`, `size_in_megabytes`, `skip_final_backup`, `snaplock_configuration`, `snapshot_policy`, `storage_efficiency_enabled`, `storage_virtual_machine_id`, `tags`, `tags_all`, `tiering_policy`, `timeouts`, `uuid`, `volume_style`, `volume_type`

### `aws_fsx_openzfs_file_system`

**Source:** `crates/winterbaume-terraform/src/converters/fsx.rs`

**Inject attributes** (25): `arn`, `automatic_backup_retention_days`, `copy_tags_to_backups`, `copy_tags_to_volumes`, `creation_time`, `daily_automatic_backup_start_time`, `deployment_type`, `dns_name`, `endpoint_ip_address_range`, `kms_key_id`, `lifecycle`, `lifecycle_status`, `owner_id`, `preferred_subnet_id`, `region`, `root_volume_id`, `security_group_ids`, `storage_capacity`, `storage_type`, `subnet_ids`, `tags`, `tags_all`, `throughput_capacity`, `vpc_id`, `weekly_maintenance_start_time`

**Extract attributes** (26): `arn`, `automatic_backup_retention_days`, `copy_tags_to_backups`, `copy_tags_to_volumes`, `creation_time`, `daily_automatic_backup_start_time`, `deployment_type`, `dns_name`, `endpoint_ip_address_range`, `file_system_type`, `id`, `kms_key_id`, `lifecycle`, `lifecycle_status`, `owner_id`, `preferred_subnet_id`, `root_volume_id`, `security_group_ids`, `storage_capacity`, `storage_type`, `subnet_ids`, `tags`, `tags_all`, `throughput_capacity`, `vpc_id`, `weekly_maintenance_start_time`

**Missing from inject** (10): `backup_id`, `delete_options`, `disk_iops_configuration`, `endpoint_ip_address`, `final_backup_tags`, `network_interface_ids`, `root_volume_configuration`, `route_table_ids`, `skip_final_backup`, `timeouts`

**Missing from extract** (10): `backup_id`, `delete_options`, `disk_iops_configuration`, `endpoint_ip_address`, `final_backup_tags`, `network_interface_ids`, `root_volume_configuration`, `route_table_ids`, `skip_final_backup`, `timeouts`

### `aws_fsx_openzfs_snapshot`

**Source:** `crates/winterbaume-terraform/src/converters/fsx.rs`

**Inject attributes** (4): `arn`, `creation_time`, `name`, `volume_id`

**Extract attributes** (0): (none)

**Missing from inject** (3): `tags`, `tags_all`, `timeouts`

**Missing from extract** (7): `arn`, `creation_time`, `name`, `tags`, `tags_all`, `timeouts`, `volume_id`

### `aws_fsx_openzfs_volume`

**Source:** `crates/winterbaume-terraform/src/converters/fsx.rs`

**Inject attributes** (11): `arn`, `copy_tags_to_snapshots`, `data_compression_type`, `delete_volume_options`, `name`, `parent_volume_id`, `read_only`, `record_size_kib`, `storage_capacity_quota_gib`, `storage_capacity_reservation_gib`, `volume_type`

**Extract attributes** (0): (none)

**Missing from inject** (6): `nfs_exports`, `origin_snapshot`, `tags`, `tags_all`, `timeouts`, `user_and_group_quotas`

**Missing from extract** (17): `arn`, `copy_tags_to_snapshots`, `data_compression_type`, `delete_volume_options`, `name`, `nfs_exports`, `origin_snapshot`, `parent_volume_id`, `read_only`, `record_size_kib`, `storage_capacity_quota_gib`, `storage_capacity_reservation_gib`, `tags`, `tags_all`, `timeouts`, `user_and_group_quotas`, `volume_type`

### `aws_fsx_windows_file_system`

**Source:** `crates/winterbaume-terraform/src/converters/fsx.rs`

**Inject attributes** (25): `active_directory_id`, `arn`, `audit_log_configuration`, `automatic_backup_retention_days`, `copy_tags_to_backups`, `creation_time`, `daily_automatic_backup_start_time`, `deployment_type`, `disk_iops_configuration`, `dns_name`, `kms_key_id`, `lifecycle`, `lifecycle_status`, `owner_id`, `region`, `security_group_ids`, `self_managed_active_directory`, `storage_capacity`, `storage_type`, `subnet_ids`, `tags`, `tags_all`, `throughput_capacity`, `vpc_id`, `weekly_maintenance_start_time`

**Extract attributes** (26): `active_directory_id`, `arn`, `audit_log_configuration`, `automatic_backup_retention_days`, `copy_tags_to_backups`, `creation_time`, `daily_automatic_backup_start_time`, `deployment_type`, `disk_iops_configuration`, `dns_name`, `file_system_type`, `id`, `kms_key_id`, `lifecycle`, `lifecycle_status`, `owner_id`, `security_group_ids`, `self_managed_active_directory`, `storage_capacity`, `storage_type`, `subnet_ids`, `tags`, `tags_all`, `throughput_capacity`, `vpc_id`, `weekly_maintenance_start_time`

**Missing from inject** (9): `aliases`, `backup_id`, `final_backup_tags`, `network_interface_ids`, `preferred_file_server_ip`, `preferred_subnet_id`, `remote_administration_endpoint`, `skip_final_backup`, `timeouts`

**Missing from extract** (9): `aliases`, `backup_id`, `final_backup_tags`, `network_interface_ids`, `preferred_file_server_ip`, `preferred_subnet_id`, `remote_administration_endpoint`, `skip_final_backup`, `timeouts`

### `aws_glacier_vault`

**Source:** `crates/winterbaume-terraform/src/converters/glacier.rs`

**Inject attributes** (4): `arn`, `name`, `region`, `tags`

**Extract attributes** (7): `arn`, `events`, `id`, `name`, `sns_topic`, `tags`, `tags_all`

**Missing from inject** (4): `access_policy`, `location`, `notification`, `tags_all`

**Missing from extract** (3): `access_policy`, `location`, `notification`

### `aws_glue_catalog_database`

**Source:** `crates/winterbaume-terraform/src/converters/glue.rs`

**Inject attributes** (6): `catalog_id`, `description`, `location_uri`, `name`, `parameters`, `region`

**Extract attributes** (7): `catalog_id`, `description`, `id`, `location_uri`, `name`, `parameters`, `tags_all`

**Missing from inject** (6): `arn`, `create_table_default_permission`, `federated_database`, `tags`, `tags_all`, `target_database`

**Missing from extract** (5): `arn`, `create_table_default_permission`, `federated_database`, `tags`, `target_database`

### `aws_glue_catalog_table`

**Source:** `crates/winterbaume-terraform/src/converters/glue.rs`

**Inject attributes** (11): `catalog_id`, `database_name`, `description`, `name`, `owner`, `parameters`, `partition_keys`, `region`, `retention`, `storage_descriptor`, `table_type`

**Extract attributes** (13): `arn`, `catalog_id`, `database_name`, `description`, `id`, `name`, `owner`, `parameters`, `partition_keys`, `retention`, `storage_descriptor`, `table_type`, `tags_all`

**Missing from inject** (6): `arn`, `open_table_format_input`, `partition_index`, `target_table`, `view_expanded_text`, `view_original_text`

**Missing from extract** (5): `open_table_format_input`, `partition_index`, `target_table`, `view_expanded_text`, `view_original_text`

### `aws_glue_catalog_table_optimizer`

**Source:** `crates/winterbaume-terraform/src/converters/glue.rs`

**Inject attributes** (0): (none)

**Extract attributes** (0): (none)

**Missing from inject** (5): `catalog_id`, `configuration`, `database_name`, `table_name`, `type`

**Missing from extract** (5): `catalog_id`, `configuration`, `database_name`, `table_name`, `type`

### `aws_glue_classifier`

**Source:** `crates/winterbaume-terraform/src/converters/glue.rs`

**Inject attributes** (0): (none)

**Extract attributes** (0): (none)

**Missing from inject** (5): `csv_classifier`, `grok_classifier`, `json_classifier`, `name`, `xml_classifier`

**Missing from extract** (5): `csv_classifier`, `grok_classifier`, `json_classifier`, `name`, `xml_classifier`

### `aws_glue_connection`

**Source:** `crates/winterbaume-terraform/src/converters/glue.rs`

**Inject attributes** (7): `connection_properties`, `connection_type`, `description`, `match_criteria`, `name`, `physical_connection_requirements`, `region`

**Extract attributes** (10): `arn`, `catalog_id`, `connection_properties`, `connection_type`, `description`, `id`, `match_criteria`, `name`, `physical_connection_requirements`, `tags_all`

**Missing from inject** (5): `arn`, `athena_properties`, `catalog_id`, `tags`, `tags_all`

**Missing from extract** (2): `athena_properties`, `tags`

### `aws_glue_crawler`

**Source:** `crates/winterbaume-terraform/src/converters/glue.rs`

**Inject attributes** (8): `database_name`, `description`, `name`, `region`, `role`, `s3_target`, `schedule`, `table_prefix`

**Extract attributes** (22): `arn`, `classifiers`, `configuration`, `crawler_lineage_settings`, `creation_time`, `database_name`, `delete_behavior`, `description`, `id`, `last_updated`, `lineage_configuration`, `name`, `recrawl_behavior`, `recrawl_policy`, `role`, `schedule`, `schema_change_policy`, `state`, `table_prefix`, `tags_all`, `update_behavior`, `version`

**Missing from inject** (17): `arn`, `catalog_target`, `classifiers`, `configuration`, `delta_target`, `dynamodb_target`, `hudi_target`, `iceberg_target`, `jdbc_target`, `lake_formation_configuration`, `lineage_configuration`, `mongodb_target`, `recrawl_policy`, `schema_change_policy`, `security_configuration`, `tags`, `tags_all`

**Missing from extract** (11): `catalog_target`, `delta_target`, `dynamodb_target`, `hudi_target`, `iceberg_target`, `jdbc_target`, `lake_formation_configuration`, `mongodb_target`, `s3_target`, `security_configuration`, `tags`

### `aws_glue_data_catalog_encryption_settings`

**Source:** `crates/winterbaume-terraform/src/converters/glue.rs`

**Inject attributes** (3): `catalog_id`, `data_catalog_encryption_settings`, `region`

**Extract attributes** (5): `catalog_id`, `connection_password_encryption`, `data_catalog_encryption_settings`, `encryption_at_rest`, `id`

### `aws_glue_data_quality_ruleset`

**Source:** `crates/winterbaume-terraform/src/converters/glue.rs`

**Inject attributes** (0): (none)

**Extract attributes** (0): (none)

**Missing from inject** (10): `arn`, `created_on`, `description`, `last_modified_on`, `name`, `recommendation_run_id`, `ruleset`, `tags`, `tags_all`, `target_table`

**Missing from extract** (10): `arn`, `created_on`, `description`, `last_modified_on`, `name`, `recommendation_run_id`, `ruleset`, `tags`, `tags_all`, `target_table`

### `aws_glue_dev_endpoint`

**Source:** `crates/winterbaume-terraform/src/converters/glue.rs`

**Inject attributes** (14): `arguments`, `extra_jars_s3_path`, `extra_python_libs_s3_path`, `glue_version`, `name`, `number_of_nodes`, `number_of_workers`, `public_key`, `public_keys`, `region`, `role_arn`, `security_group_ids`, `subnet_id`, `worker_type`

**Extract attributes** (19): `arguments`, `arn`, `created_timestamp`, `extra_jars_s3_path`, `extra_python_libs_s3_path`, `glue_version`, `id`, `last_modified_timestamp`, `name`, `number_of_nodes`, `number_of_workers`, `public_key`, `public_keys`, `role_arn`, `security_group_ids`, `status`, `subnet_id`, `tags_all`, `worker_type`

**Missing from inject** (12): `arn`, `availability_zone`, `failure_reason`, `private_address`, `public_address`, `security_configuration`, `status`, `tags`, `tags_all`, `vpc_id`, `yarn_endpoint_address`, `zeppelin_remote_spark_interpreter_port`

**Missing from extract** (9): `availability_zone`, `failure_reason`, `private_address`, `public_address`, `security_configuration`, `tags`, `vpc_id`, `yarn_endpoint_address`, `zeppelin_remote_spark_interpreter_port`

### `aws_glue_job`

**Source:** `crates/winterbaume-terraform/src/converters/glue.rs`

**Inject attributes** (12): `command`, `default_arguments`, `description`, `glue_version`, `max_capacity`, `max_retries`, `name`, `number_of_workers`, `region`, `role_arn`, `timeout`, `worker_type`

**Extract attributes** (13): `arn`, `connections`, `description`, `glue_version`, `id`, `max_capacity`, `max_retries`, `name`, `number_of_workers`, `role_arn`, `tags_all`, `timeout`, `worker_type`

**Missing from inject** (12): `arn`, `connections`, `execution_class`, `execution_property`, `job_run_queuing_enabled`, `maintenance_window`, `non_overridable_arguments`, `notification_property`, `security_configuration`, `source_control_details`, `tags`, `tags_all`

**Missing from extract** (11): `command`, `default_arguments`, `execution_class`, `execution_property`, `job_run_queuing_enabled`, `maintenance_window`, `non_overridable_arguments`, `notification_property`, `security_configuration`, `source_control_details`, `tags`

### `aws_glue_ml_transform`

**Source:** `crates/winterbaume-terraform/src/converters/glue.rs`

**Inject attributes** (13): `description`, `glue_version`, `id`, `input_record_tables`, `max_capacity`, `max_retries`, `name`, `number_of_workers`, `parameters`, `region`, `role_arn`, `timeout`, `worker_type`

**Extract attributes** (14): `arn`, `description`, `glue_version`, `id`, `input_record_tables`, `max_capacity`, `max_retries`, `name`, `number_of_workers`, `parameters`, `role_arn`, `tags_all`, `timeout`, `worker_type`

**Missing from inject** (5): `arn`, `label_count`, `schema`, `tags`, `tags_all`

**Missing from extract** (3): `label_count`, `schema`, `tags`

### `aws_glue_partition`

**Source:** `crates/winterbaume-terraform/src/converters/glue.rs`

**Inject attributes** (8): `catalog_id`, `database_name`, `parameters`, `partition_values`, `region`, `storage_descriptor`, `table_name`, `values`

**Extract attributes** (10): `catalog_id`, `creation_time`, `database_name`, `id`, `last_accessed_time`, `parameters`, `partition_values`, `storage_descriptor`, `table_name`, `values`

**Missing from inject** (3): `creation_time`, `last_accessed_time`, `last_analyzed_time`

**Missing from extract** (1): `last_analyzed_time`

### `aws_glue_partition_index`

**Source:** `crates/winterbaume-terraform/src/converters/glue.rs`

**Inject attributes** (0): (none)

**Extract attributes** (0): (none)

**Missing from inject** (5): `catalog_id`, `database_name`, `partition_index`, `table_name`, `timeouts`

**Missing from extract** (5): `catalog_id`, `database_name`, `partition_index`, `table_name`, `timeouts`

### `aws_glue_registry`

**Source:** `crates/winterbaume-terraform/src/converters/glue.rs`

**Inject attributes** (4): `description`, `region`, `registry_name`, `tags_all`

**Extract attributes** (6): `arn`, `description`, `id`, `registry_name`, `tags`, `tags_all`

**Missing from inject** (2): `arn`, `tags`

### `aws_glue_resource_policy`

**Source:** `crates/winterbaume-terraform/src/converters/glue.rs`

**Inject attributes** (2): `policy`, `region`

**Extract attributes** (5): `create_time`, `id`, `policy`, `policy_hash`, `update_time`

**Missing from inject** (1): `enable_hybrid`

**Missing from extract** (1): `enable_hybrid`

### `aws_glue_schema`

**Source:** `crates/winterbaume-terraform/src/converters/glue.rs`

**Inject attributes** (7): `compatibility`, `data_format`, `description`, `region`, `registry_arn`, `schema_name`, `tags_all`

**Extract attributes** (14): `arn`, `compatibility`, `data_format`, `description`, `id`, `latest_schema_version`, `next_schema_version`, `registry_arn`, `registry_name`, `schema_checkpoint`, `schema_name`, `schema_status`, `tags`, `tags_all`

**Missing from inject** (7): `arn`, `latest_schema_version`, `next_schema_version`, `registry_name`, `schema_checkpoint`, `schema_definition`, `tags`

**Missing from extract** (1): `schema_definition`

### `aws_glue_security_configuration`

**Source:** `crates/winterbaume-terraform/src/converters/glue.rs`

**Inject attributes** (3): `encryption_configuration`, `name`, `region`

**Extract attributes** (4): `created_time_stamp`, `encryption_configuration`, `id`, `name`

### `aws_glue_trigger`

**Source:** `crates/winterbaume-terraform/src/converters/glue.rs`

**Inject attributes** (9): `actions`, `description`, `name`, `predicate`, `region`, `schedule`, `state`, `type`, `workflow_name`

**Extract attributes** (11): `actions`, `arn`, `description`, `id`, `name`, `predicate`, `schedule`, `state`, `tags_all`, `type`, `workflow_name`

**Missing from inject** (7): `arn`, `enabled`, `event_batching_condition`, `start_on_creation`, `tags`, `tags_all`, `timeouts`

**Missing from extract** (5): `enabled`, `event_batching_condition`, `start_on_creation`, `tags`, `timeouts`

### `aws_glue_user_defined_function`

**Source:** `crates/winterbaume-terraform/src/converters/glue.rs`

**Inject attributes** (0): (none)

**Extract attributes** (0): (none)

**Missing from inject** (9): `arn`, `catalog_id`, `class_name`, `create_time`, `database_name`, `name`, `owner_name`, `owner_type`, `resource_uris`

**Missing from extract** (9): `arn`, `catalog_id`, `class_name`, `create_time`, `database_name`, `name`, `owner_name`, `owner_type`, `resource_uris`

### `aws_glue_workflow`

**Source:** `crates/winterbaume-terraform/src/converters/glue.rs`

**Inject attributes** (5): `default_run_properties`, `description`, `max_concurrent_runs`, `name`, `region`

**Extract attributes** (7): `arn`, `default_run_properties`, `description`, `id`, `max_concurrent_runs`, `name`, `tags_all`

**Missing from inject** (3): `arn`, `tags`, `tags_all`

**Missing from extract** (1): `tags`

### `aws_guardduty_detector`

**Source:** `crates/winterbaume-terraform/src/converters/guardduty.rs`

**Inject attributes** (7): `datasources`, `detector_id`, `enable`, `finding_publishing_frequency`, `region`, `tags`, `tags_all`

**Extract attributes** (7): `created_at`, `detector_id`, `enable`, `finding_publishing_frequency`, `id`, `tags`, `tags_all`

**Missing from inject** (2): `account_id`, `arn`

**Missing from extract** (3): `account_id`, `arn`, `datasources`

### `aws_guardduty_detector_feature`

**Source:** `crates/winterbaume-terraform/src/converters/guardduty.rs`

**Inject attributes** (4): `detector_id`, `name`, `region`, `status`

**Extract attributes** (4): `detector_id`, `id`, `name`, `status`

**Missing from inject** (1): `additional_configuration`

**Missing from extract** (1): `additional_configuration`

### `aws_guardduty_filter`

**Source:** `crates/winterbaume-terraform/src/converters/guardduty.rs`

**Inject attributes** (8): `action`, `description`, `detector_id`, `finding_criteria`, `name`, `rank`, `region`, `tags`

**Extract attributes** (16): `action`, `criterion`, `description`, `detector_id`, `equals`, `field`, `finding_criteria`, `greater_than`, `greater_than_or_equal`, `id`, `less_than`, `less_than_or_equal`, `name`, `not_equals`, `rank`, `tags`

**Missing from inject** (2): `arn`, `tags_all`

**Missing from extract** (2): `arn`, `tags_all`

### `aws_guardduty_invite_accepter`

**Source:** `crates/winterbaume-terraform/src/converters/guardduty.rs`

**Inject attributes** (2): `master_id`, `region`

**Extract attributes** (3): `detector_id`, `id`, `master_account_id`

**Missing from inject** (3): `detector_id`, `master_account_id`, `timeouts`

**Missing from extract** (1): `timeouts`

### `aws_guardduty_ipset`

**Source:** `crates/winterbaume-terraform/src/converters/guardduty.rs`

**Inject attributes** (7): `activate`, `detector_id`, `format`, `location`, `name`, `region`, `tags`

**Extract attributes** (8): `activate`, `detector_id`, `format`, `id`, `location`, `name`, `tags`, `tags_all`

**Missing from inject** (2): `arn`, `tags_all`

**Missing from extract** (1): `arn`

### `aws_guardduty_malware_protection_plan`

**Source:** `crates/winterbaume-terraform/src/converters/guardduty.rs`

**Inject attributes** (6): `actions`, `protected_resource`, `region`, `role`, `tags`, `tags_all`

**Extract attributes** (12): `actions`, `arn`, `bucket_name`, `id`, `object_prefixes`, `protected_resource`, `role`, `s3_bucket`, `status`, `tagging`, `tags`, `tags_all`

**Missing from inject** (3): `arn`, `created_at`, `status`

**Missing from extract** (1): `created_at`

### `aws_guardduty_member`

**Source:** `crates/winterbaume-terraform/src/converters/guardduty.rs`

**Inject attributes** (14): `account_id`, `administrator_account_id`, `arn`, `email`, `invitation_disable_email_notification`, `invitation_message`, `invite`, `invited_at`, `master_account_id`, `region`, `relationship_status`, `status`, `tags`, `updated_at`

**Extract attributes** (4): `account_id`, `detector_id`, `email`, `relationship_status`

**Missing from inject** (3): `detector_id`, `disable_email_notification`, `timeouts`

**Missing from extract** (4): `disable_email_notification`, `invitation_message`, `invite`, `timeouts`

### `aws_guardduty_member_detector_feature`

**Source:** `crates/winterbaume-terraform/src/converters/guardduty.rs`

**Inject attributes** (5): `account_id`, `detector_id`, `name`, `region`, `status`

**Extract attributes** (5): `account_id`, `detector_id`, `id`, `name`, `status`

**Missing from inject** (1): `additional_configuration`

**Missing from extract** (1): `additional_configuration`

### `aws_guardduty_organization_admin_account`

**Source:** `crates/winterbaume-terraform/src/converters/guardduty.rs`

**Inject attributes** (2): `admin_account_id`, `region`

**Extract attributes** (2): `admin_account_id`, `id`

### `aws_guardduty_organization_configuration`

**Source:** `crates/winterbaume-terraform/src/converters/guardduty.rs`

**Inject attributes** (2): `auto_enable`, `region`

**Extract attributes** (4): `auto_enable`, `auto_enable_organization_members`, `detector_id`, `id`

**Missing from inject** (3): `auto_enable_organization_members`, `datasources`, `detector_id`

**Missing from extract** (1): `datasources`

### `aws_guardduty_organization_configuration_feature`

**Source:** `crates/winterbaume-terraform/src/converters/guardduty.rs`

**Inject attributes** (4): `auto_enable`, `detector_id`, `name`, `region`

**Extract attributes** (0): (none)

**Missing from inject** (1): `additional_configuration`

**Missing from extract** (4): `additional_configuration`, `auto_enable`, `detector_id`, `name`

### `aws_guardduty_publishing_destination`

**Source:** `crates/winterbaume-terraform/src/converters/guardduty.rs`

**Inject attributes** (5): `destination_arn`, `destination_type`, `detector_id`, `kms_key_arn`, `region`

**Extract attributes** (5): `destination_arn`, `destination_type`, `detector_id`, `id`, `kms_key_arn`

### `aws_guardduty_threatintelset`

**Source:** `crates/winterbaume-terraform/src/converters/guardduty.rs`

**Inject attributes** (7): `activate`, `detector_id`, `format`, `location`, `name`, `region`, `tags`

**Extract attributes** (8): `activate`, `detector_id`, `format`, `id`, `location`, `name`, `tags`, `tags_all`

**Missing from inject** (2): `arn`, `tags_all`

**Missing from extract** (1): `arn`

### `aws_iam_access_key`

**Source:** `crates/winterbaume-terraform/src/converters/iam.rs`

**Inject attributes** (2): `status`, `user`

**Extract attributes** (4): `create_date`, `id`, `status`, `user`

**Missing from inject** (7): `create_date`, `encrypted_secret`, `encrypted_ses_smtp_password_v4`, `key_fingerprint`, `pgp_key`, `secret`, `ses_smtp_password_v4`

**Missing from extract** (6): `encrypted_secret`, `encrypted_ses_smtp_password_v4`, `key_fingerprint`, `pgp_key`, `secret`, `ses_smtp_password_v4`

### `aws_iam_account_alias`

**Source:** `crates/winterbaume-terraform/src/converters/iam.rs`

**Inject attributes** (1): `account_alias`

**Extract attributes** (2): `account_alias`, `id`

### `aws_iam_account_password_policy`

**Source:** `crates/winterbaume-terraform/src/converters/iam.rs`

**Inject attributes** (10): `allow_users_to_change_password`, `expire_passwords`, `hard_expiry`, `max_password_age`, `minimum_password_length`, `password_reuse_prevention`, `require_lowercase_characters`, `require_numbers`, `require_symbols`, `require_uppercase_characters`

**Extract attributes** (11): `allow_users_to_change_password`, `expire_passwords`, `hard_expiry`, `id`, `max_password_age`, `minimum_password_length`, `password_reuse_prevention`, `require_lowercase_characters`, `require_numbers`, `require_symbols`, `require_uppercase_characters`

### `aws_iam_group`

**Source:** `crates/winterbaume-terraform/src/converters/iam.rs`

**Inject attributes** (4): `arn`, `name`, `path`, `unique_id`

**Extract attributes** (5): `arn`, `id`, `name`, `path`, `unique_id`

### `aws_iam_group_membership`

**Source:** `crates/winterbaume-terraform/src/converters/iam.rs`

**Inject attributes** (3): `group`, `name`, `users`

**Extract attributes** (4): `group`, `id`, `name`, `users`

### `aws_iam_group_policies_exclusive`

**Source:** `crates/winterbaume-terraform/src/converters/iam.rs`

**Inject attributes** (1): `group_name`

**Extract attributes** (3): `group_name`, `id`, `policy_names`

**Missing from inject** (1): `policy_names`

### `aws_iam_group_policy`

**Source:** `crates/winterbaume-terraform/src/converters/iam.rs`

**Inject attributes** (4): `group`, `name`, `name_prefix`, `policy`

**Extract attributes** (4): `group`, `id`, `name`, `policy`

**Missing from extract** (1): `name_prefix`

### `aws_iam_group_policy_attachment`

**Source:** `crates/winterbaume-terraform/src/converters/iam.rs`

**Inject attributes** (2): `group`, `policy_arn`

**Extract attributes** (3): `group`, `id`, `policy_arn`

### `aws_iam_group_policy_attachments_exclusive`

**Source:** `crates/winterbaume-terraform/src/converters/iam.rs`

**Inject attributes** (1): `group_name`

**Extract attributes** (3): `group_name`, `id`, `policy_arns`

**Missing from inject** (1): `policy_arns`

### `aws_iam_instance_profile`

**Source:** `crates/winterbaume-terraform/src/converters/iam.rs`

**Inject attributes** (7): `arn`, `name`, `path`, `role`, `tags`, `tags_all`, `unique_id`

**Extract attributes** (6): `arn`, `id`, `name`, `path`, `role`, `unique_id`

**Missing from inject** (2): `create_date`, `name_prefix`

**Missing from extract** (4): `create_date`, `name_prefix`, `tags`, `tags_all`

### `aws_iam_openid_connect_provider`

**Source:** `crates/winterbaume-terraform/src/converters/iam.rs`

**Inject attributes** (6): `arn`, `client_id_list`, `tags`, `tags_all`, `thumbprint_list`, `url`

**Extract attributes** (7): `arn`, `client_id_list`, `id`, `tags`, `tags_all`, `thumbprint_list`, `url`

### `aws_iam_policy`

**Source:** `crates/winterbaume-terraform/src/converters/iam.rs`

**Inject attributes** (9): `arn`, `description`, `name`, `path`, `policy`, `policy_id`, `policy_name`, `tags`, `tags_all`

**Extract attributes** (7): `arn`, `description`, `id`, `name`, `path`, `policy`, `policy_id`

**Missing from inject** (2): `attachment_count`, `name_prefix`

**Missing from extract** (4): `attachment_count`, `name_prefix`, `tags`, `tags_all`

### `aws_iam_policy_attachment`

**Source:** `crates/winterbaume-terraform/src/converters/iam.rs`

**Inject attributes** (2): `name`, `policy_arn`

**Extract attributes** (0): (none)

**Missing from inject** (3): `groups`, `roles`, `users`

**Missing from extract** (5): `groups`, `name`, `policy_arn`, `roles`, `users`

### `aws_iam_role`

**Source:** `crates/winterbaume-terraform/src/converters/iam.rs`

**Inject attributes** (10): `arn`, `assume_role_policy`, `description`, `max_session_duration`, `name`, `path`, `permissions_boundary`, `tags`, `tags_all`, `unique_id`

**Extract attributes** (10): `arn`, `assume_role_policy`, `description`, `id`, `max_session_duration`, `name`, `path`, `tags`, `tags_all`, `unique_id`

**Missing from inject** (5): `create_date`, `force_detach_policies`, `inline_policy`, `managed_policy_arns`, `name_prefix`

**Missing from extract** (6): `create_date`, `force_detach_policies`, `inline_policy`, `managed_policy_arns`, `name_prefix`, `permissions_boundary`

### `aws_iam_role_policies_exclusive`

**Source:** `crates/winterbaume-terraform/src/converters/iam.rs`

**Inject attributes** (1): `role_name`

**Extract attributes** (3): `id`, `policy_names`, `role_name`

**Missing from inject** (1): `policy_names`

### `aws_iam_role_policy`

**Source:** `crates/winterbaume-terraform/src/converters/iam.rs`

**Inject attributes** (4): `name`, `name_prefix`, `policy`, `role`

**Extract attributes** (4): `id`, `name`, `policy`, `role`

**Missing from extract** (1): `name_prefix`

### `aws_iam_role_policy_attachment`

**Source:** `crates/winterbaume-terraform/src/converters/iam.rs`

**Inject attributes** (2): `policy_arn`, `role`

**Extract attributes** (3): `id`, `policy_arn`, `role`

### `aws_iam_role_policy_attachments_exclusive`

**Source:** `crates/winterbaume-terraform/src/converters/iam.rs`

**Inject attributes** (1): `role_name`

**Extract attributes** (3): `id`, `policy_arns`, `role_name`

**Missing from inject** (1): `policy_arns`

### `aws_iam_saml_provider`

**Source:** `crates/winterbaume-terraform/src/converters/iam.rs`

**Inject attributes** (5): `arn`, `name`, `saml_metadata_document`, `tags`, `tags_all`

**Extract attributes** (7): `arn`, `id`, `name`, `saml_metadata_document`, `tags`, `tags_all`, `valid_until`

**Missing from inject** (1): `valid_until`

### `aws_iam_server_certificate`

**Source:** `crates/winterbaume-terraform/src/converters/iam.rs`

**Inject attributes** (9): `arn`, `certificate_body`, `certificate_chain`, `name`, `name_prefix`, `path`, `private_key`, `tags`, `tags_all`

**Extract attributes** (10): `arn`, `certificate_body`, `certificate_chain`, `expiration`, `id`, `name`, `path`, `tags`, `tags_all`, `upload_date`

**Missing from inject** (3): `expiration`, `timeouts`, `upload_date`

**Missing from extract** (3): `name_prefix`, `private_key`, `timeouts`

### `aws_iam_service_linked_role`

**Source:** `crates/winterbaume-terraform/src/converters/iam.rs`

**Inject attributes** (5): `aws_service_name`, `custom_suffix`, `description`, `tags`, `tags_all`

**Extract attributes** (9): `arn`, `aws_service_name`, `description`, `id`, `name`, `path`, `tags`, `tags_all`, `unique_id`

**Missing from inject** (5): `arn`, `create_date`, `name`, `path`, `unique_id`

**Missing from extract** (2): `create_date`, `custom_suffix`

### `aws_iam_service_specific_credential`

**Source:** `crates/winterbaume-terraform/src/converters/iam.rs`

**Inject attributes** (4): `service_name`, `service_specific_credential_id`, `status`, `user_name`

**Extract attributes** (7): `id`, `service_name`, `service_password`, `service_specific_credential_id`, `service_user_name`, `status`, `user_name`

**Missing from inject** (2): `service_password`, `service_user_name`

### `aws_iam_signing_certificate`

**Source:** `crates/winterbaume-terraform/src/converters/iam.rs`

**Inject attributes** (4): `certificate_body`, `certificate_id`, `status`, `user_name`

**Extract attributes** (5): `certificate_body`, `certificate_id`, `id`, `status`, `user_name`

### `aws_iam_user`

**Source:** `crates/winterbaume-terraform/src/converters/iam.rs`

**Inject attributes** (6): `arn`, `name`, `path`, `tags`, `tags_all`, `unique_id`

**Extract attributes** (5): `arn`, `id`, `name`, `path`, `unique_id`

**Missing from inject** (2): `force_destroy`, `permissions_boundary`

**Missing from extract** (4): `force_destroy`, `permissions_boundary`, `tags`, `tags_all`

### `aws_iam_user_group_membership`

**Source:** `crates/winterbaume-terraform/src/converters/iam.rs`

**Inject attributes** (2): `groups`, `user`

**Extract attributes** (3): `groups`, `id`, `user`

### `aws_iam_user_login_profile`

**Source:** `crates/winterbaume-terraform/src/converters/iam.rs`

**Inject attributes** (3): `password_length`, `password_reset_required`, `user`

**Extract attributes** (4): `id`, `password_length`, `password_reset_required`, `user`

**Missing from inject** (4): `encrypted_password`, `key_fingerprint`, `password`, `pgp_key`

**Missing from extract** (4): `encrypted_password`, `key_fingerprint`, `password`, `pgp_key`

### `aws_iam_user_policies_exclusive`

**Source:** `crates/winterbaume-terraform/src/converters/iam.rs`

**Inject attributes** (1): `user_name`

**Extract attributes** (3): `id`, `policy_names`, `user_name`

**Missing from inject** (1): `policy_names`

### `aws_iam_user_policy`

**Source:** `crates/winterbaume-terraform/src/converters/iam.rs`

**Inject attributes** (4): `name`, `name_prefix`, `policy`, `user`

**Extract attributes** (4): `id`, `name`, `policy`, `user`

**Missing from extract** (1): `name_prefix`

### `aws_iam_user_policy_attachment`

**Source:** `crates/winterbaume-terraform/src/converters/iam.rs`

**Inject attributes** (2): `policy_arn`, `user`

**Extract attributes** (3): `id`, `policy_arn`, `user`

### `aws_iam_user_policy_attachments_exclusive`

**Source:** `crates/winterbaume-terraform/src/converters/iam.rs`

**Inject attributes** (1): `user_name`

**Extract attributes** (3): `id`, `policy_arns`, `user_name`

**Missing from inject** (1): `policy_arns`

### `aws_iam_user_ssh_key`

**Source:** `crates/winterbaume-terraform/src/converters/iam.rs`

**Inject attributes** (5): `encoding`, `public_key`, `ssh_public_key_id`, `status`, `username`

**Extract attributes** (7): `encoding`, `fingerprint`, `id`, `public_key`, `ssh_public_key_id`, `status`, `username`

**Missing from inject** (1): `fingerprint`

### `aws_iam_virtual_mfa_device`

**Source:** `crates/winterbaume-terraform/src/converters/iam.rs`

**Inject attributes** (5): `arn`, `path`, `tags`, `tags_all`, `virtual_mfa_device_name`

**Extract attributes** (5): `arn`, `id`, `tags`, `tags_all`, `virtual_mfa_device_name`

**Missing from inject** (4): `base_32_string_seed`, `enable_date`, `qr_code_png`, `user_name`

**Missing from extract** (5): `base_32_string_seed`, `enable_date`, `path`, `qr_code_png`, `user_name`

### `aws_service_name`

**Source:** `crates/winterbaume-terraform/src/converters/iam.rs`

**Inject attributes** (0): (none)

**Extract attributes** (8): `arn`, `description`, `id`, `name`, `path`, `tags`, `tags_all`, `unique_id`

**Note:** Resource type not found in Terraform AWS provider schema.

### `aws_identitystore_group`

**Source:** `crates/winterbaume-terraform/src/converters/identitystore.rs`

**Inject attributes** (4): `arn`, `filter_expression`, `group_name`, `region`

**Extract attributes** (6): `description`, `display_name`, `external_ids`, `group_id`, `id`, `identity_store_id`

**Missing from inject** (5): `description`, `display_name`, `external_ids`, `group_id`, `identity_store_id`

### `aws_identitystore_user`

**Source:** `crates/winterbaume-terraform/src/converters/identitystore.rs`

**Inject attributes** (11): `addresses`, `arn`, `emails`, `home_directory`, `home_directory_type`, `name`, `phone_numbers`, `region`, `role`, `server_id`, `user_name`

**Extract attributes** (21): `addresses`, `birthdate`, `display_name`, `emails`, `external_ids`, `id`, `identity_store_id`, `locale`, `name`, `nick_name`, `phone_numbers`, `photos`, `preferred_language`, `profile_url`, `roles`, `timezone`, `title`, `user_id`, `user_name`, `user_type`, `website`

**Missing from inject** (11): `display_name`, `external_ids`, `identity_store_id`, `locale`, `nickname`, `preferred_language`, `profile_url`, `timezone`, `title`, `user_id`, `user_type`

**Missing from extract** (1): `nickname`

### `aws_inspector2_delegated_admin_account`

**Source:** `crates/winterbaume-terraform/src/converters/inspector2.rs`

**Inject attributes** (2): `account_id`, `region`

**Extract attributes** (2): `account_id`, `id`

**Missing from inject** (2): `relationship_status`, `timeouts`

**Missing from extract** (2): `relationship_status`, `timeouts`

### `aws_inspector2_enabler`

**Source:** `crates/winterbaume-terraform/src/converters/inspector2.rs`

**Inject attributes** (3): `account_ids`, `region`, `resource_types`

**Extract attributes** (3): `account_ids`, `id`, `resource_types`

**Missing from inject** (1): `timeouts`

**Missing from extract** (1): `timeouts`

### `aws_inspector2_filter`

**Source:** `crates/winterbaume-terraform/src/converters/inspector2.rs`

**Inject attributes** (7): `action`, `description`, `detector_id`, `name`, `rank`, `region`, `tags`

**Extract attributes** (6): `action`, `arn`, `description`, `id`, `name`, `tags`

**Missing from inject** (4): `arn`, `filter_criteria`, `reason`, `tags_all`

**Missing from extract** (3): `filter_criteria`, `reason`, `tags_all`

### `aws_inspector2_member_association`

**Source:** `crates/winterbaume-terraform/src/converters/inspector2.rs`

**Inject attributes** (2): `account_id`, `region`

**Extract attributes** (3): `account_id`, `id`, `relationship_status`

**Missing from inject** (4): `delegated_admin_account_id`, `relationship_status`, `timeouts`, `updated_at`

**Missing from extract** (3): `delegated_admin_account_id`, `timeouts`, `updated_at`

### `aws_inspector2_organization_configuration`

**Source:** `crates/winterbaume-terraform/src/converters/inspector2.rs`

**Inject attributes** (2): `auto_enable`, `region`

**Extract attributes** (7): `auto_enable`, `code_repository`, `ec2`, `ecr`, `id`, `lambda`, `lambda_code`

**Missing from inject** (2): `max_account_limit_reached`, `timeouts`

**Missing from extract** (2): `max_account_limit_reached`, `timeouts`

### `aws_iot_authorizer`

**Source:** `crates/winterbaume-terraform/src/converters/iot.rs`

**Inject attributes** (8): `arn`, `authorizer_function_arn`, `enable_caching_for_http`, `name`, `region`, `signing_disabled`, `status`, `token_key_name`

**Extract attributes** (0): (none)

**Missing from inject** (3): `tags`, `tags_all`, `token_signing_public_keys`

**Missing from extract** (10): `arn`, `authorizer_function_arn`, `enable_caching_for_http`, `name`, `signing_disabled`, `status`, `tags`, `tags_all`, `token_key_name`, `token_signing_public_keys`

### `aws_iot_billing_group`

**Source:** `crates/winterbaume-terraform/src/converters/iot.rs`

**Inject attributes** (3): `arn`, `name`, `region`

**Extract attributes** (3): `arn`, `id`, `name`

**Missing from inject** (5): `metadata`, `properties`, `tags`, `tags_all`, `version`

**Missing from extract** (5): `metadata`, `properties`, `tags`, `tags_all`, `version`

### `aws_iot_ca_certificate`

**Source:** `crates/winterbaume-terraform/src/converters/iot.rs`

**Inject attributes** (6): `active`, `allow_auto_registration`, `arn`, `ca_certificate_pem`, `certificate_mode`, `region`

**Extract attributes** (6): `active`, `allow_auto_registration`, `arn`, `ca_certificate_pem`, `certificate_mode`, `id`

**Missing from inject** (7): `customer_version`, `generation_id`, `registration_config`, `tags`, `tags_all`, `validity`, `verification_certificate_pem`

**Missing from extract** (7): `customer_version`, `generation_id`, `registration_config`, `tags`, `tags_all`, `validity`, `verification_certificate_pem`

### `aws_iot_certificate`

**Source:** `crates/winterbaume-terraform/src/converters/iot.rs`

**Inject attributes** (10): `arn`, `certificate`, `certificate_chain`, `certificate_id`, `certificate_type`, `description`, `private_key`, `region`, `status`, `usage`

**Extract attributes** (5): `active`, `arn`, `ca_pem`, `certificate_pem`, `id`

**Missing from inject** (6): `active`, `ca_certificate_id`, `ca_pem`, `certificate_pem`, `csr`, `public_key`

**Missing from extract** (4): `ca_certificate_id`, `csr`, `private_key`, `public_key`

### `aws_iot_domain_configuration`

**Source:** `crates/winterbaume-terraform/src/converters/iot.rs`

**Inject attributes** (6): `arn`, `domain_name`, `name`, `region`, `service_type`, `status`

**Extract attributes** (6): `arn`, `domain_name`, `id`, `name`, `service_type`, `status`

**Missing from inject** (9): `application_protocol`, `authentication_type`, `authorizer_config`, `domain_type`, `server_certificate_arns`, `tags`, `tags_all`, `tls_config`, `validation_certificate_arn`

**Missing from extract** (9): `application_protocol`, `authentication_type`, `authorizer_config`, `domain_type`, `server_certificate_arns`, `tags`, `tags_all`, `tls_config`, `validation_certificate_arn`

### `aws_iot_event_configurations`

**Source:** `crates/winterbaume-terraform/src/converters/iot.rs`

**Inject attributes** (1): `region`

**Extract attributes** (0): (none)

**Missing from inject** (1): `event_configurations`

**Missing from extract** (1): `event_configurations`

### `aws_iot_indexing_configuration`

**Source:** `crates/winterbaume-terraform/src/converters/iot.rs`

**Inject attributes** (3): `region`, `thing_group_indexing_configuration`, `thing_indexing_configuration`

**Extract attributes** (3): `id`, `thing_group_indexing_configuration`, `thing_indexing_configuration`

### `aws_iot_logging_options`

**Source:** `crates/winterbaume-terraform/src/converters/iot.rs`

**Inject attributes** (4): `default_log_level`, `disable_all_logs`, `region`, `role_arn`

**Extract attributes** (0): (none)

**Missing from extract** (3): `default_log_level`, `disable_all_logs`, `role_arn`

### `aws_iot_policy`

**Source:** `crates/winterbaume-terraform/src/converters/iot.rs`

**Inject attributes** (5): `arn`, `default_version_id`, `name`, `policy`, `region`

**Extract attributes** (5): `arn`, `default_version_id`, `id`, `name`, `policy`

**Missing from inject** (3): `tags`, `tags_all`, `timeouts`

**Missing from extract** (3): `tags`, `tags_all`, `timeouts`

### `aws_iot_policy_attachment`

**Source:** `crates/winterbaume-terraform/src/converters/iot.rs`

**Inject attributes** (3): `policy`, `region`, `target`

**Extract attributes** (3): `id`, `policy`, `target`

### `aws_iot_provisioning_template`

**Source:** `crates/winterbaume-terraform/src/converters/iot.rs`

**Inject attributes** (8): `arn`, `default_version_id`, `description`, `enabled`, `name`, `provisioning_role_arn`, `region`, `template_body`

**Extract attributes** (0): (none)

**Missing from inject** (4): `pre_provisioning_hook`, `tags`, `tags_all`, `type`

**Missing from extract** (11): `arn`, `default_version_id`, `description`, `enabled`, `name`, `pre_provisioning_hook`, `provisioning_role_arn`, `tags`, `tags_all`, `template_body`, `type`

### `aws_iot_role_alias`

**Source:** `crates/winterbaume-terraform/src/converters/iot.rs`

**Inject attributes** (5): `alias`, `arn`, `credential_duration`, `region`, `role_arn`

**Extract attributes** (5): `alias`, `arn`, `credential_duration`, `id`, `role_arn`

**Missing from inject** (2): `tags`, `tags_all`

**Missing from extract** (2): `tags`, `tags_all`

### `aws_iot_thing`

**Source:** `crates/winterbaume-terraform/src/converters/iot.rs`

**Inject attributes** (7): `arn`, `attributes`, `default_client_id`, `name`, `region`, `thing_type_name`, `version`

**Extract attributes** (7): `arn`, `attributes`, `default_client_id`, `id`, `name`, `thing_type_name`, `version`

### `aws_iot_thing_group`

**Source:** `crates/winterbaume-terraform/src/converters/iot.rs`

**Inject attributes** (6): `arn`, `name`, `parent_group_name`, `properties`, `region`, `version`

**Extract attributes** (5): `arn`, `id`, `name`, `parent_group_name`, `version`

**Missing from inject** (3): `metadata`, `tags`, `tags_all`

**Missing from extract** (4): `metadata`, `properties`, `tags`, `tags_all`

### `aws_iot_thing_group_membership`

**Source:** `crates/winterbaume-terraform/src/converters/iot.rs`

**Inject attributes** (4): `override_dynamic_group`, `region`, `thing_group_name`, `thing_name`

**Extract attributes** (3): `id`, `thing_group_name`, `thing_name`

**Missing from extract** (1): `override_dynamic_group`

### `aws_iot_thing_principal_attachment`

**Source:** `crates/winterbaume-terraform/src/converters/iot.rs`

**Inject attributes** (3): `principal`, `region`, `thing`

**Extract attributes** (3): `id`, `principal`, `thing`

### `aws_iot_thing_type`

**Source:** `crates/winterbaume-terraform/src/converters/iot.rs`

**Inject attributes** (5): `arn`, `deprecated`, `name`, `properties`, `region`

**Extract attributes** (7): `arn`, `deprecated`, `description`, `id`, `name`, `properties`, `searchable_attributes`

**Missing from inject** (2): `tags`, `tags_all`

**Missing from extract** (2): `tags`, `tags_all`

### `aws_iot_topic_rule`

**Source:** `crates/winterbaume-terraform/src/converters/iot.rs`

**Inject attributes** (10): `action`, `actions`, `arn`, `description`, `enabled`, `error_action`, `name`, `region`, `sql`, `sql_version`

**Extract attributes** (7): `arn`, `description`, `enabled`, `id`, `name`, `sql`, `sql_version`

**Missing from inject** (21): `cloudwatch_alarm`, `cloudwatch_logs`, `cloudwatch_metric`, `dynamodb`, `dynamodbv2`, `elasticsearch`, `firehose`, `http`, `iot_analytics`, `iot_events`, `kafka`, `kinesis`, `lambda`, `republish`, `s3`, `sns`, `sqs`, `step_functions`, `tags`, `tags_all`, `timestream`

**Missing from extract** (22): `cloudwatch_alarm`, `cloudwatch_logs`, `cloudwatch_metric`, `dynamodb`, `dynamodbv2`, `elasticsearch`, `error_action`, `firehose`, `http`, `iot_analytics`, `iot_events`, `kafka`, `kinesis`, `lambda`, `republish`, `s3`, `sns`, `sqs`, `step_functions`, `tags`, `tags_all`, `timestream`

### `aws_iot_topic_rule_destination`

**Source:** `crates/winterbaume-terraform/src/converters/iot.rs`

**Inject attributes** (3): `arn`, `enabled`, `region`

**Extract attributes** (0): (none)

**Missing from inject** (2): `timeouts`, `vpc_configuration`

**Missing from extract** (4): `arn`, `enabled`, `timeouts`, `vpc_configuration`

### `aws_ivs_channel`

**Source:** `crates/winterbaume-terraform/src/converters/ivs.rs`

**Inject attributes** (7): `arn`, `authorized`, `latency_mode`, `name`, `region`, `tags`, `type`

**Extract attributes** (7): `arn`, `authorized`, `id`, `latency_mode`, `name`, `tags`, `type`

**Missing from inject** (5): `ingest_endpoint`, `playback_url`, `recording_configuration_arn`, `tags_all`, `timeouts`

**Missing from extract** (5): `ingest_endpoint`, `playback_url`, `recording_configuration_arn`, `tags_all`, `timeouts`

### `aws_msk_cluster`

**Source:** `crates/winterbaume-terraform/src/converters/kafka.rs`

**Inject attributes** (24): `acl_name`, `arn`, `auto_minor_version_upgrade`, `broker_node_group_info`, `client_authentication`, `configuration_info`, `description`, `encryption_info`, `engine`, `engine_version`, `logging_info`, `maintenance_window`, `name`, `node_type`, `num_replicas_per_shard`, `num_shards`, `number_of_broker_nodes`, `open_monitoring`, `parameter_group_name`, `region`, `snapshot_retention_limit`, `snapshot_window`, `subnet_group_name`, `tls_enabled`

**Extract attributes** (27): `arn`, `bootstrap_brokers`, `bootstrap_brokers_tls`, `bootstrap_brokers_vpc_connectivity_tls`, `broker_node_group_info`, `client_authentication`, `client_subnets`, `cluster_name`, `cluster_type`, `cluster_uuid`, `configuration_info`, `creation_time`, `current_version`, `encryption_info`, `enhanced_monitoring`, `id`, `instance_type`, `kafka_version`, `logging_info`, `number_of_broker_nodes`, `open_monitoring`, `security_groups`, `state`, `tags`, `tags_all`, `zookeeper_connect_string`, `zookeeper_connect_string_tls`

**Missing from inject** (21): `bootstrap_brokers`, `bootstrap_brokers_public_sasl_iam`, `bootstrap_brokers_public_sasl_scram`, `bootstrap_brokers_public_tls`, `bootstrap_brokers_sasl_iam`, `bootstrap_brokers_sasl_scram`, `bootstrap_brokers_tls`, `bootstrap_brokers_vpc_connectivity_sasl_iam`, `bootstrap_brokers_vpc_connectivity_sasl_scram`, `bootstrap_brokers_vpc_connectivity_tls`, `cluster_name`, `cluster_uuid`, `current_version`, `enhanced_monitoring`, `kafka_version`, `storage_mode`, `tags`, `tags_all`, `timeouts`, `zookeeper_connect_string`, `zookeeper_connect_string_tls`

**Missing from extract** (9): `bootstrap_brokers_public_sasl_iam`, `bootstrap_brokers_public_sasl_scram`, `bootstrap_brokers_public_tls`, `bootstrap_brokers_sasl_iam`, `bootstrap_brokers_sasl_scram`, `bootstrap_brokers_vpc_connectivity_sasl_iam`, `bootstrap_brokers_vpc_connectivity_sasl_scram`, `storage_mode`, `timeouts`

### `aws_msk_cluster_policy`

**Source:** `crates/winterbaume-terraform/src/converters/kafka.rs`

**Inject attributes** (3): `cluster_arn`, `policy`, `region`

**Extract attributes** (0): (none)

**Missing from inject** (1): `current_version`

**Missing from extract** (3): `cluster_arn`, `current_version`, `policy`

### `aws_msk_configuration`

**Source:** `crates/winterbaume-terraform/src/converters/kafka.rs`

**Inject attributes** (6): `arn`, `description`, `latest_revision`, `name`, `region`, `server_properties`

**Extract attributes** (0): (none)

**Missing from inject** (1): `kafka_versions`

**Missing from extract** (6): `arn`, `description`, `kafka_versions`, `latest_revision`, `name`, `server_properties`

### `aws_msk_replicator`

**Source:** `crates/winterbaume-terraform/src/converters/kafka.rs`

**Inject attributes** (6): `arn`, `description`, `region`, `replicator_name`, `service_execution_role_arn`, `tags`

**Extract attributes** (0): (none)

**Missing from inject** (5): `current_version`, `kafka_cluster`, `replication_info_list`, `tags_all`, `timeouts`

**Missing from extract** (10): `arn`, `current_version`, `description`, `kafka_cluster`, `replication_info_list`, `replicator_name`, `service_execution_role_arn`, `tags`, `tags_all`, `timeouts`

### `aws_msk_scram_secret_association`

**Source:** `crates/winterbaume-terraform/src/converters/kafka.rs`

**Inject attributes** (2): `cluster_arn`, `region`

**Extract attributes** (0): (none)

**Missing from inject** (1): `secret_arn_list`

**Missing from extract** (2): `cluster_arn`, `secret_arn_list`

### `aws_msk_serverless_cluster`

**Source:** `crates/winterbaume-terraform/src/converters/kafka.rs`

**Inject attributes** (6): `arn`, `client_authentication`, `cluster_name`, `region`, `tags`, `vpc_config`

**Extract attributes** (11): `arn`, `bootstrap_brokers_sasl_iam`, `client_authentication`, `cluster_name`, `cluster_uuid`, `id`, `security_group_ids`, `subnet_ids`, `tags`, `tags_all`, `vpc_config`

**Missing from inject** (4): `bootstrap_brokers_sasl_iam`, `cluster_uuid`, `tags_all`, `timeouts`

**Missing from extract** (1): `timeouts`

### `aws_msk_single_scram_secret_association`

**Source:** `crates/winterbaume-terraform/src/converters/kafka.rs`

**Inject attributes** (3): `cluster_arn`, `region`, `secret_arn`

**Extract attributes** (0): (none)

**Missing from extract** (2): `cluster_arn`, `secret_arn`

### `aws_msk_vpc_connection`

**Source:** `crates/winterbaume-terraform/src/converters/kafka.rs`

**Inject attributes** (6): `arn`, `authentication`, `region`, `tags`, `target_cluster_arn`, `vpc_id`

**Extract attributes** (0): (none)

**Missing from inject** (3): `client_subnets`, `security_groups`, `tags_all`

**Missing from extract** (8): `arn`, `authentication`, `client_subnets`, `security_groups`, `tags`, `tags_all`, `target_cluster_arn`, `vpc_id`

### `aws_keyspaces_keyspace`

**Source:** `crates/winterbaume-terraform/src/converters/keyspaces.rs`

**Inject attributes** (4): `arn`, `name`, `region`, `tags_all`

**Extract attributes** (4): `arn`, `name`, `tags`, `tags_all`

**Missing from inject** (3): `replication_specification`, `tags`, `timeouts`

**Missing from extract** (2): `replication_specification`, `timeouts`

### `aws_keyspaces_table`

**Source:** `crates/winterbaume-terraform/src/converters/keyspaces.rs`

**Inject attributes** (6): `arn`, `database_name`, `region`, `table_name`, `tags`, `tags_all`

**Extract attributes** (3): `arn`, `keyspace_name`, `table_name`

**Missing from inject** (10): `capacity_specification`, `client_side_timestamps`, `comment`, `default_time_to_live`, `encryption_specification`, `keyspace_name`, `point_in_time_recovery`, `schema_definition`, `timeouts`, `ttl`

**Missing from extract** (11): `capacity_specification`, `client_side_timestamps`, `comment`, `default_time_to_live`, `encryption_specification`, `point_in_time_recovery`, `schema_definition`, `tags`, `tags_all`, `timeouts`, `ttl`

### `aws_kinesis_resource_policy`

**Source:** `crates/winterbaume-terraform/src/converters/kinesis.rs`

**Inject attributes** (0): (none)

**Extract attributes** (0): (none)

**Missing from inject** (2): `policy`, `resource_arn`

**Missing from extract** (2): `policy`, `resource_arn`

### `aws_kinesis_stream`

**Source:** `crates/winterbaume-terraform/src/converters/kinesis.rs`

**Inject attributes** (11): `arn`, `creation_time`, `data_retention_in_hours`, `device_name`, `kms_key_id`, `media_type`, `name`, `region`, `status`, `tags`, `version`

**Extract attributes** (8): `arn`, `encryption_type`, `id`, `kms_key_id`, `name`, `retention_period`, `shard_count`, `tags`

**Missing from inject** (8): `encryption_type`, `enforce_consumer_deletion`, `retention_period`, `shard_count`, `shard_level_metrics`, `stream_mode_details`, `tags_all`, `timeouts`

**Missing from extract** (5): `enforce_consumer_deletion`, `shard_level_metrics`, `stream_mode_details`, `tags_all`, `timeouts`

### `aws_kinesis_stream_consumer`

**Source:** `crates/winterbaume-terraform/src/converters/kinesis.rs`

**Inject attributes** (0): (none)

**Extract attributes** (0): (none)

**Missing from inject** (4): `arn`, `creation_timestamp`, `name`, `stream_arn`

**Missing from extract** (4): `arn`, `creation_timestamp`, `name`, `stream_arn`

### `aws_kinesis_analytics_application`

**Source:** `crates/winterbaume-terraform/src/converters/kinesisanalyticsv2.rs`

**Inject attributes** (0): (none)

**Extract attributes** (0): (none)

**Missing from inject** (15): `arn`, `cloudwatch_logging_options`, `code`, `create_timestamp`, `description`, `inputs`, `last_update_timestamp`, `name`, `outputs`, `reference_data_sources`, `start_application`, `status`, `tags`, `tags_all`, `version`

**Missing from extract** (15): `arn`, `cloudwatch_logging_options`, `code`, `create_timestamp`, `description`, `inputs`, `last_update_timestamp`, `name`, `outputs`, `reference_data_sources`, `start_application`, `status`, `tags`, `tags_all`, `version`

### `aws_kinesisanalyticsv2_application`

**Source:** `crates/winterbaume-terraform/src/converters/kinesisanalyticsv2.rs`

**Inject attributes** (7): `arn`, `date_created`, `date_updated`, `description`, `name`, `region`, `tags`

**Extract attributes** (15): `application_configuration`, `arn`, `cloudwatch_logging_options`, `create_timestamp`, `description`, `id`, `last_update_timestamp`, `name`, `runtime_environment`, `service_execution_role`, `start_application`, `status`, `tags`, `tags_all`, `version_id`

**Missing from inject** (13): `application_configuration`, `application_mode`, `cloudwatch_logging_options`, `create_timestamp`, `force_stop`, `last_update_timestamp`, `runtime_environment`, `service_execution_role`, `start_application`, `status`, `tags_all`, `timeouts`, `version_id`

**Missing from extract** (3): `application_mode`, `force_stop`, `timeouts`

### `aws_kinesis_video_stream`

**Source:** `crates/winterbaume-terraform/src/converters/kinesisvideo.rs`

**Inject attributes** (11): `arn`, `creation_time`, `data_retention_in_hours`, `device_name`, `kms_key_id`, `media_type`, `name`, `region`, `status`, `tags`, `version`

**Extract attributes** (11): `arn`, `creation_time`, `data_retention_in_hours`, `device_name`, `id`, `kms_key_id`, `media_type`, `name`, `status`, `tags`, `version`

**Missing from inject** (2): `tags_all`, `timeouts`

**Missing from extract** (2): `tags_all`, `timeouts`

### `aws_kms_alias`

**Source:** `crates/winterbaume-terraform/src/converters/kms.rs`

**Inject attributes** (4): `arn`, `name`, `region`, `target_key_id`

**Extract attributes** (5): `arn`, `id`, `name`, `target_key_arn`, `target_key_id`

**Missing from inject** (2): `name_prefix`, `target_key_arn`

**Missing from extract** (1): `name_prefix`

### `aws_kms_ciphertext`

**Source:** `crates/winterbaume-terraform/src/converters/kms.rs`

**Inject attributes** (4): `ciphertext_blob`, `key_id`, `plaintext`, `region`

**Extract attributes** (0): (none)

**Missing from inject** (1): `context`

**Missing from extract** (4): `ciphertext_blob`, `context`, `key_id`, `plaintext`

### `aws_kms_custom_key_store`

**Source:** `crates/winterbaume-terraform/src/converters/kms.rs`

**Inject attributes** (6): `cloud_hsm_cluster_id`, `custom_key_store_id`, `custom_key_store_name`, `custom_key_store_type`, `region`, `trust_anchor_certificate`

**Extract attributes** (7): `cloud_hsm_cluster_id`, `connection_state`, `custom_key_store_id`, `custom_key_store_name`, `custom_key_store_type`, `id`, `trust_anchor_certificate`

**Missing from inject** (7): `key_store_password`, `timeouts`, `xks_proxy_authentication_credential`, `xks_proxy_connectivity`, `xks_proxy_uri_endpoint`, `xks_proxy_uri_path`, `xks_proxy_vpc_endpoint_service_name`

**Missing from extract** (7): `key_store_password`, `timeouts`, `xks_proxy_authentication_credential`, `xks_proxy_connectivity`, `xks_proxy_uri_endpoint`, `xks_proxy_uri_path`, `xks_proxy_vpc_endpoint_service_name`

### `aws_kms_external_key`

**Source:** `crates/winterbaume-terraform/src/converters/kms.rs`

**Inject attributes** (9): `arn`, `description`, `enabled`, `key_id`, `key_material_base64`, `multi_region`, `region`, `tags`, `tags_all`

**Extract attributes** (11): `arn`, `description`, `enabled`, `expiration_model`, `id`, `key_id`, `key_state`, `key_usage`, `multi_region`, `tags`, `tags_all`

**Missing from inject** (7): `bypass_policy_lockout_safety_check`, `deletion_window_in_days`, `expiration_model`, `key_state`, `key_usage`, `policy`, `valid_to`

**Missing from extract** (5): `bypass_policy_lockout_safety_check`, `deletion_window_in_days`, `key_material_base64`, `policy`, `valid_to`

### `aws_kms_grant`

**Source:** `crates/winterbaume-terraform/src/converters/kms.rs`

**Inject attributes** (9): `constraints`, `grant_id`, `grant_token`, `grantee_principal`, `key_id`, `name`, `operations`, `region`, `retiring_principal`

**Extract attributes** (11): `constraints`, `encryption_context_equals`, `encryption_context_subset`, `grant_id`, `grant_token`, `grantee_principal`, `id`, `key_id`, `name`, `operations`, `retiring_principal`

**Missing from inject** (2): `grant_creation_tokens`, `retire_on_delete`

**Missing from extract** (2): `grant_creation_tokens`, `retire_on_delete`

### `aws_kms_key`

**Source:** `crates/winterbaume-terraform/src/converters/kms.rs`

**Inject attributes** (14): `arn`, `customer_master_key_spec`, `deletion_window_in_days`, `description`, `enable_key_rotation`, `is_enabled`, `key_id`, `key_spec`, `key_usage`, `multi_region`, `policy`, `region`, `tags`, `tags_all`

**Extract attributes** (11): `arn`, `description`, `enable_key_rotation`, `id`, `is_enabled`, `key_id`, `key_spec`, `key_usage`, `multi_region`, `tags`, `tags_all`

**Missing from inject** (5): `bypass_policy_lockout_safety_check`, `custom_key_store_id`, `rotation_period_in_days`, `timeouts`, `xks_key_id`

**Missing from extract** (8): `bypass_policy_lockout_safety_check`, `custom_key_store_id`, `customer_master_key_spec`, `deletion_window_in_days`, `policy`, `rotation_period_in_days`, `timeouts`, `xks_key_id`

### `aws_kms_key_policy`

**Source:** `crates/winterbaume-terraform/src/converters/kms.rs`

**Inject attributes** (3): `key_id`, `policy`, `region`

**Extract attributes** (3): `id`, `key_id`, `policy`

**Missing from inject** (1): `bypass_policy_lockout_safety_check`

**Missing from extract** (1): `bypass_policy_lockout_safety_check`

### `aws_kms_replica_external_key`

**Source:** `crates/winterbaume-terraform/src/converters/kms.rs`

**Inject attributes** (9): `arn`, `description`, `enabled`, `key_id`, `key_material_base64`, `primary_key_arn`, `region`, `tags`, `tags_all`

**Extract attributes** (11): `arn`, `description`, `enabled`, `expiration_model`, `id`, `key_id`, `key_state`, `key_usage`, `primary_key_arn`, `tags`, `tags_all`

**Missing from inject** (7): `bypass_policy_lockout_safety_check`, `deletion_window_in_days`, `expiration_model`, `key_state`, `key_usage`, `policy`, `valid_to`

**Missing from extract** (5): `bypass_policy_lockout_safety_check`, `deletion_window_in_days`, `key_material_base64`, `policy`, `valid_to`

### `aws_kms_replica_key`

**Source:** `crates/winterbaume-terraform/src/converters/kms.rs`

**Inject attributes** (9): `arn`, `description`, `enabled`, `key_id`, `key_rotation_enabled`, `primary_key_arn`, `region`, `tags`, `tags_all`

**Extract attributes** (12): `arn`, `description`, `enabled`, `id`, `key_id`, `key_rotation_enabled`, `key_spec`, `key_state`, `key_usage`, `primary_key_arn`, `tags`, `tags_all`

**Missing from inject** (5): `bypass_policy_lockout_safety_check`, `deletion_window_in_days`, `key_spec`, `key_usage`, `policy`

**Missing from extract** (3): `bypass_policy_lockout_safety_check`, `deletion_window_in_days`, `policy`

### `aws_lakeformation_data_cells_filter`

**Source:** `crates/winterbaume-terraform/src/converters/lakeformation.rs`

**Inject attributes** (1): `region`

**Extract attributes** (0): (none)

**Missing from inject** (2): `table_data`, `timeouts`

**Missing from extract** (2): `table_data`, `timeouts`

### `aws_lakeformation_data_lake_settings`

**Source:** `crates/winterbaume-terraform/src/converters/lakeformation.rs`

**Inject attributes** (6): `admins`, `allow_external_data_filtering`, `authorized_session_tag_value_list`, `create_database_default_permissions`, `create_table_default_permissions`, `region`

**Extract attributes** (10): `admins`, `allow_external_data_filtering`, `authorized_session_tag_value_list`, `create_database_default_permissions`, `create_table_default_permissions`, `id`, `permissions`, `principal`, `tags_all`, `trusted_resource_owners`

**Missing from inject** (6): `allow_full_table_external_data_access`, `catalog_id`, `external_data_filtering_allow_list`, `parameters`, `read_only_admins`, `trusted_resource_owners`

**Missing from extract** (5): `allow_full_table_external_data_access`, `catalog_id`, `external_data_filtering_allow_list`, `parameters`, `read_only_admins`

### `aws_lakeformation_lf_tag`

**Source:** `crates/winterbaume-terraform/src/converters/lakeformation.rs`

**Inject attributes** (3): `catalog_id`, `key`, `region`

**Extract attributes** (4): `catalog_id`, `id`, `key`, `values`

**Missing from inject** (1): `values`

### `aws_lakeformation_opt_in`

**Source:** `crates/winterbaume-terraform/src/converters/lakeformation.rs`

**Inject attributes** (1): `region`

**Extract attributes** (0): (none)

**Missing from inject** (5): `condition`, `last_modified`, `last_updated_by`, `principal`, `resource_data`

**Missing from extract** (5): `condition`, `last_modified`, `last_updated_by`, `principal`, `resource_data`

### `aws_lakeformation_permissions`

**Source:** `crates/winterbaume-terraform/src/converters/lakeformation.rs`

**Inject attributes** (4): `permissions`, `permissions_with_grant_option`, `principal`, `region`

**Extract attributes** (5): `catalog_resource`, `id`, `permissions`, `permissions_with_grant_option`, `principal`

**Missing from inject** (9): `catalog_id`, `catalog_resource`, `data_cells_filter`, `data_location`, `database`, `lf_tag`, `lf_tag_policy`, `table`, `table_with_columns`

**Missing from extract** (8): `catalog_id`, `data_cells_filter`, `data_location`, `database`, `lf_tag`, `lf_tag_policy`, `table`, `table_with_columns`

### `aws_lakeformation_resource`

**Source:** `crates/winterbaume-terraform/src/converters/lakeformation.rs`

**Inject attributes** (4): `arn`, `region`, `role_arn`, `use_service_linked_role`

**Extract attributes** (4): `arn`, `id`, `role_arn`, `use_service_linked_role`

**Missing from inject** (3): `hybrid_access_enabled`, `last_modified`, `with_federation`

**Missing from extract** (3): `hybrid_access_enabled`, `last_modified`, `with_federation`

### `aws_lakeformation_resource_lf_tag`

**Source:** `crates/winterbaume-terraform/src/converters/lakeformation.rs`

**Inject attributes** (4): `catalog_id`, `region`, `tag_key`, `tag_value`

**Extract attributes** (4): `catalog_id`, `id`, `tag_key`, `tag_value`

**Missing from inject** (5): `database`, `lf_tag`, `table`, `table_with_columns`, `timeouts`

**Missing from extract** (5): `database`, `lf_tag`, `table`, `table_with_columns`, `timeouts`

### `aws_lakeformation_resource_lf_tags`

**Source:** `crates/winterbaume-terraform/src/converters/lakeformation.rs`

**Inject attributes** (3): `catalog_id`, `lf_tag`, `region`

**Extract attributes** (5): `catalog_id`, `id`, `key`, `lf_tag`, `value`

**Missing from inject** (4): `database`, `table`, `table_with_columns`, `timeouts`

**Missing from extract** (4): `database`, `table`, `table_with_columns`, `timeouts`

### `aws_lambda_alias`

**Source:** `crates/winterbaume-terraform/src/converters/lambda.rs`

**Inject attributes** (6): `arn`, `description`, `function_name`, `function_version`, `name`, `region`

**Extract attributes** (6): `arn`, `description`, `function_name`, `function_version`, `id`, `name`

**Missing from inject** (2): `invoke_arn`, `routing_config`

**Missing from extract** (2): `invoke_arn`, `routing_config`

### `aws_lambda_code_signing_config`

**Source:** `crates/winterbaume-terraform/src/converters/lambda.rs`

**Inject attributes** (9): `allowed_publishers`, `arn`, `config_id`, `description`, `id`, `policies`, `region`, `tags`, `tags_all`

**Extract attributes** (11): `allowed_publishers`, `arn`, `config_id`, `description`, `id`, `last_modified`, `policies`, `signing_profile_version_arns`, `tags`, `tags_all`, `untrusted_artifact_on_deployment`

**Missing from inject** (1): `last_modified`

### `aws_lambda_event_source_mapping`

**Source:** `crates/winterbaume-terraform/src/converters/lambda.rs`

**Inject attributes** (28): `amazon_managed_kafka_event_source_config`, `batch_size`, `bisect_batch_on_function_error`, `destination_config`, `document_db_event_source_config`, `enabled`, `event_source_arn`, `filter_criteria`, `function_name`, `function_response_types`, `id`, `maximum_batching_window_in_seconds`, `maximum_record_age_in_seconds`, `maximum_retry_attempts`, `metrics_config`, `parallelization_factor`, `provisioned_poller_config`, `queues`, `region`, `scaling_config`, `self_managed_event_source`, `self_managed_kafka_event_source_config`, `source_access_configuration`, `starting_position`, `tags_all`, `topics`, `tumbling_window_in_seconds`, `uuid`

**Extract attributes** (19): `batch_size`, `enabled`, `event_source_arn`, `function_arn`, `function_name`, `function_response_types`, `id`, `last_modified`, `maximum_batching_window_in_seconds`, `maximum_record_age_in_seconds`, `maximum_retry_attempts`, `parallelization_factor`, `scaling_config`, `starting_position`, `state`, `state_transition_reason`, `tags_all`, `tumbling_window_in_seconds`, `uuid`

**Missing from inject** (9): `arn`, `function_arn`, `kms_key_arn`, `last_modified`, `last_processing_result`, `starting_position_timestamp`, `state`, `state_transition_reason`, `tags`

**Missing from extract** (17): `amazon_managed_kafka_event_source_config`, `arn`, `bisect_batch_on_function_error`, `destination_config`, `document_db_event_source_config`, `filter_criteria`, `kms_key_arn`, `last_processing_result`, `metrics_config`, `provisioned_poller_config`, `queues`, `self_managed_event_source`, `self_managed_kafka_event_source_config`, `source_access_configuration`, `starting_position_timestamp`, `tags`, `topics`

### `aws_lambda_function`

**Source:** `crates/winterbaume-terraform/src/converters/lambda.rs`

**Inject attributes** (31): `architectures`, `arn`, `code_signing_config_arn`, `dead_letter_config`, `description`, `environment`, `ephemeral_storage`, `file_system_config`, `function_name`, `handler`, `image_config`, `image_uri`, `kms_key_arn`, `layers`, `logging_config`, `memory_size`, `package_type`, `publish`, `region`, `replace_security_groups_on_destroy`, `reserved_concurrent_executions`, `role`, `runtime`, `s3_bucket`, `skip_destroy`, `snap_start`, `tags`, `tags_all`, `timeout`, `tracing_config`, `vpc_config`

**Extract attributes** (27): `architectures`, `arn`, `code_sha256`, `code_signing_config_arn`, `code_size`, `description`, `environment`, `function_name`, `handler`, `id`, `invoke_arn`, `last_modified`, `memory_size`, `qualified_arn`, `qualified_invoke_arn`, `reserved_concurrent_executions`, `role`, `runtime`, `signing_job_arn`, `signing_profile_version_arn`, `source_code_size`, `state`, `tags`, `tags_all`, `timeout`, `variables`, `version`

**Missing from inject** (15): `code_sha256`, `filename`, `invoke_arn`, `last_modified`, `qualified_arn`, `qualified_invoke_arn`, `replacement_security_group_ids`, `s3_key`, `s3_object_version`, `signing_job_arn`, `signing_profile_version_arn`, `source_code_hash`, `source_code_size`, `timeouts`, `version`

**Missing from extract** (22): `dead_letter_config`, `ephemeral_storage`, `file_system_config`, `filename`, `image_config`, `image_uri`, `kms_key_arn`, `layers`, `logging_config`, `package_type`, `publish`, `replace_security_groups_on_destroy`, `replacement_security_group_ids`, `s3_bucket`, `s3_key`, `s3_object_version`, `skip_destroy`, `snap_start`, `source_code_hash`, `timeouts`, `tracing_config`, `vpc_config`

### `aws_lambda_function_event_invoke_config`

**Source:** `crates/winterbaume-terraform/src/converters/lambda.rs`

**Inject attributes** (6): `destination_config`, `function_name`, `maximum_event_age_in_seconds`, `maximum_retry_attempts`, `qualifier`, `region`

**Extract attributes** (8): `destination`, `destination_config`, `function_name`, `id`, `maximum_event_age_in_seconds`, `maximum_retry_attempts`, `on_failure`, `on_success`

**Missing from extract** (1): `qualifier`

### `aws_lambda_function_recursion_config`

**Source:** `crates/winterbaume-terraform/src/converters/lambda.rs`

**Inject attributes** (3): `function_name`, `recursive_loop`, `region`

**Extract attributes** (3): `function_name`, `id`, `recursive_loop`

### `aws_lambda_function_url`

**Source:** `crates/winterbaume-terraform/src/converters/lambda.rs`

**Inject attributes** (9): `authorization_type`, `cors`, `function_arn`, `function_name`, `function_url`, `invoke_mode`, `qualifier`, `region`, `url_id`

**Extract attributes** (10): `authorization_type`, `cors`, `creation_time`, `function_arn`, `function_name`, `function_url`, `id`, `invoke_mode`, `last_modified_time`, `url_id`

**Missing from inject** (1): `timeouts`

**Missing from extract** (2): `qualifier`, `timeouts`

### `aws_lambda_invocation`

**Source:** `crates/winterbaume-terraform/src/converters/lambda.rs`

**Inject attributes** (0): (none)

**Extract attributes** (0): (none)

**Missing from inject** (7): `function_name`, `input`, `lifecycle_scope`, `qualifier`, `result`, `terraform_key`, `triggers`

**Missing from extract** (7): `function_name`, `input`, `lifecycle_scope`, `qualifier`, `result`, `terraform_key`, `triggers`

### `aws_lambda_layer_version`

**Source:** `crates/winterbaume-terraform/src/converters/lambda.rs`

**Inject attributes** (16): `arn`, `compatible_architectures`, `compatible_runtimes`, `description`, `filename`, `layer_arn`, `layer_name`, `license_info`, `region`, `s3_bucket`, `s3_key`, `s3_object_version`, `skip_destroy`, `source_code_hash`, `source_code_size`, `version`

**Extract attributes** (15): `arn`, `compatible_architectures`, `compatible_runtimes`, `created_date`, `description`, `id`, `layer_arn`, `layer_name`, `license_info`, `signing_job_arn`, `signing_profile_version_arn`, `skip_destroy`, `source_code_hash`, `source_code_size`, `version`

**Missing from inject** (4): `code_sha256`, `created_date`, `signing_job_arn`, `signing_profile_version_arn`

**Missing from extract** (5): `code_sha256`, `filename`, `s3_bucket`, `s3_key`, `s3_object_version`

### `aws_lambda_layer_version_permission`

**Source:** `crates/winterbaume-terraform/src/converters/lambda.rs`

**Inject attributes** (7): `action`, `layer_name`, `organization_id`, `principal`, `region`, `statement_id`, `version_number`

**Extract attributes** (8): `action`, `id`, `layer_name`, `organization_id`, `principal`, `revision_id`, `statement_id`, `version_number`

**Missing from inject** (3): `policy`, `revision_id`, `skip_destroy`

**Missing from extract** (2): `policy`, `skip_destroy`

### `aws_lambda_permission`

**Source:** `crates/winterbaume-terraform/src/converters/lambda.rs`

**Inject attributes** (8): `action`, `function_name`, `principal`, `principal_org_id`, `region`, `source_account`, `source_arn`, `statement_id`

**Extract attributes** (7): `action`, `function_name`, `id`, `principal`, `source_account`, `source_arn`, `statement_id`

**Missing from inject** (4): `event_source_token`, `function_url_auth_type`, `qualifier`, `statement_id_prefix`

**Missing from extract** (5): `event_source_token`, `function_url_auth_type`, `principal_org_id`, `qualifier`, `statement_id_prefix`

### `aws_lambda_provisioned_concurrency_config`

**Source:** `crates/winterbaume-terraform/src/converters/lambda.rs`

**Inject attributes** (4): `function_name`, `provisioned_concurrent_executions`, `qualifier`, `region`

**Extract attributes** (6): `function_name`, `id`, `last_modified`, `provisioned_concurrent_executions`, `qualifier`, `status`

**Missing from inject** (2): `skip_destroy`, `timeouts`

**Missing from extract** (2): `skip_destroy`, `timeouts`

### `aws_lambda_runtime_management_config`

**Source:** `crates/winterbaume-terraform/src/converters/lambda.rs`

**Inject attributes** (5): `function_name`, `qualifier`, `region`, `runtime_version_arn`, `update_runtime_on`

**Extract attributes** (4): `function_name`, `id`, `runtime_version_arn`, `update_runtime_on`

**Missing from inject** (1): `function_arn`

**Missing from extract** (2): `function_arn`, `qualifier`

### `aws_lexv2models_bot`

**Source:** `crates/winterbaume-terraform/src/converters/lexmodelsv2.rs`

**Inject attributes** (6): `description`, `idle_session_ttl_in_seconds`, `name`, `region`, `role_arn`, `tags`

**Extract attributes** (13): `arn`, `child_directed`, `data_privacy`, `description`, `id`, `idle_session_ttl_in_seconds`, `members`, `name`, `role_arn`, `tags`, `tags_all`, `test_bot_alias_tags`, `type`

**Missing from inject** (7): `arn`, `data_privacy`, `members`, `tags_all`, `test_bot_alias_tags`, `timeouts`, `type`

**Missing from extract** (1): `timeouts`

### `aws_lexv2models_bot_locale`

**Source:** `crates/winterbaume-terraform/src/converters/lexmodelsv2.rs`

**Inject attributes** (6): `bot_id`, `bot_version`, `description`, `locale_id`, `name`, `region`

**Extract attributes** (7): `bot_id`, `bot_version`, `description`, `id`, `locale_id`, `n_lu_intent_confidence_threshold`, `name`

**Missing from inject** (3): `n_lu_intent_confidence_threshold`, `timeouts`, `voice_settings`

**Missing from extract** (2): `timeouts`, `voice_settings`

### `aws_lexv2models_bot_version`

**Source:** `crates/winterbaume-terraform/src/converters/lexmodelsv2.rs`

**Inject attributes** (3): `bot_id`, `description`, `region`

**Extract attributes** (4): `bot_id`, `bot_version`, `description`, `id`

**Missing from inject** (3): `bot_version`, `locale_specification`, `timeouts`

**Missing from extract** (2): `locale_specification`, `timeouts`

### `aws_lexv2models_intent`

**Source:** `crates/winterbaume-terraform/src/converters/lexmodelsv2.rs`

**Inject attributes** (7): `bot_id`, `bot_version`, `description`, `intent_id`, `locale_id`, `name`, `region`

**Extract attributes** (7): `bot_id`, `bot_version`, `description`, `id`, `intent_id`, `locale_id`, `name`

**Missing from inject** (14): `closing_setting`, `confirmation_setting`, `creation_date_time`, `dialog_code_hook`, `fulfillment_code_hook`, `initial_response_setting`, `input_context`, `kendra_configuration`, `last_updated_date_time`, `output_context`, `parent_intent_signature`, `sample_utterance`, `slot_priority`, `timeouts`

**Missing from extract** (14): `closing_setting`, `confirmation_setting`, `creation_date_time`, `dialog_code_hook`, `fulfillment_code_hook`, `initial_response_setting`, `input_context`, `kendra_configuration`, `last_updated_date_time`, `output_context`, `parent_intent_signature`, `sample_utterance`, `slot_priority`, `timeouts`

### `aws_lexv2models_slot`

**Source:** `crates/winterbaume-terraform/src/converters/lexmodelsv2.rs`

**Inject attributes** (9): `bot_id`, `bot_version`, `description`, `intent_id`, `locale_id`, `name`, `region`, `slot_id`, `slot_type_id`

**Extract attributes** (9): `bot_id`, `bot_version`, `description`, `id`, `intent_id`, `locale_id`, `name`, `slot_id`, `slot_type_id`

**Missing from inject** (5): `multiple_values_setting`, `obfuscation_setting`, `sub_slot_setting`, `timeouts`, `value_elicitation_setting`

**Missing from extract** (5): `multiple_values_setting`, `obfuscation_setting`, `sub_slot_setting`, `timeouts`, `value_elicitation_setting`

### `aws_lexv2models_slot_type`

**Source:** `crates/winterbaume-terraform/src/converters/lexmodelsv2.rs`

**Inject attributes** (8): `bot_id`, `bot_version`, `description`, `locale_id`, `name`, `parent_slot_type_signature`, `region`, `slot_type_id`

**Extract attributes** (8): `bot_id`, `bot_version`, `description`, `id`, `locale_id`, `name`, `parent_slot_type_signature`, `slot_type_id`

**Missing from inject** (5): `composite_slot_type_setting`, `external_source_setting`, `slot_type_values`, `timeouts`, `value_selection_setting`

**Missing from extract** (5): `composite_slot_type_setting`, `external_source_setting`, `slot_type_values`, `timeouts`, `value_selection_setting`

### `aws_cloudwatch_log_account_policy`

**Source:** `crates/winterbaume-terraform/src/converters/logs.rs`

**Inject attributes** (0): (none)

**Extract attributes** (0): (none)

**Missing from inject** (5): `policy_document`, `policy_name`, `policy_type`, `scope`, `selection_criteria`

**Missing from extract** (5): `policy_document`, `policy_name`, `policy_type`, `scope`, `selection_criteria`

### `aws_cloudwatch_log_anomaly_detector`

**Source:** `crates/winterbaume-terraform/src/converters/logs.rs`

**Inject attributes** (0): (none)

**Extract attributes** (0): (none)

**Missing from inject** (10): `anomaly_visibility_time`, `arn`, `detector_name`, `enabled`, `evaluation_frequency`, `filter_pattern`, `kms_key_id`, `log_group_arn_list`, `tags`, `tags_all`

**Missing from extract** (10): `anomaly_visibility_time`, `arn`, `detector_name`, `enabled`, `evaluation_frequency`, `filter_pattern`, `kms_key_id`, `log_group_arn_list`, `tags`, `tags_all`

### `aws_cloudwatch_log_data_protection_policy`

**Source:** `crates/winterbaume-terraform/src/converters/logs.rs`

**Inject attributes** (0): (none)

**Extract attributes** (0): (none)

**Missing from inject** (2): `log_group_name`, `policy_document`

**Missing from extract** (2): `log_group_name`, `policy_document`

### `aws_cloudwatch_log_delivery`

**Source:** `crates/winterbaume-terraform/src/converters/logs.rs`

**Inject attributes** (0): (none)

**Extract attributes** (0): (none)

**Missing from inject** (8): `arn`, `delivery_destination_arn`, `delivery_source_name`, `field_delimiter`, `record_fields`, `s3_delivery_configuration`, `tags`, `tags_all`

**Missing from extract** (8): `arn`, `delivery_destination_arn`, `delivery_source_name`, `field_delimiter`, `record_fields`, `s3_delivery_configuration`, `tags`, `tags_all`

### `aws_cloudwatch_log_delivery_destination`

**Source:** `crates/winterbaume-terraform/src/converters/logs.rs`

**Inject attributes** (0): (none)

**Extract attributes** (0): (none)

**Missing from inject** (7): `arn`, `delivery_destination_configuration`, `delivery_destination_type`, `name`, `output_format`, `tags`, `tags_all`

**Missing from extract** (7): `arn`, `delivery_destination_configuration`, `delivery_destination_type`, `name`, `output_format`, `tags`, `tags_all`

### `aws_cloudwatch_log_delivery_destination_policy`

**Source:** `crates/winterbaume-terraform/src/converters/logs.rs`

**Inject attributes** (0): (none)

**Extract attributes** (0): (none)

**Missing from inject** (2): `delivery_destination_name`, `delivery_destination_policy`

**Missing from extract** (2): `delivery_destination_name`, `delivery_destination_policy`

### `aws_cloudwatch_log_delivery_source`

**Source:** `crates/winterbaume-terraform/src/converters/logs.rs`

**Inject attributes** (0): (none)

**Extract attributes** (0): (none)

**Missing from inject** (7): `arn`, `log_type`, `name`, `resource_arn`, `service`, `tags`, `tags_all`

**Missing from extract** (7): `arn`, `log_type`, `name`, `resource_arn`, `service`, `tags`, `tags_all`

### `aws_cloudwatch_log_destination`

**Source:** `crates/winterbaume-terraform/src/converters/logs.rs`

**Inject attributes** (0): (none)

**Extract attributes** (0): (none)

**Missing from inject** (6): `arn`, `name`, `role_arn`, `tags`, `tags_all`, `target_arn`

**Missing from extract** (6): `arn`, `name`, `role_arn`, `tags`, `tags_all`, `target_arn`

### `aws_cloudwatch_log_destination_policy`

**Source:** `crates/winterbaume-terraform/src/converters/logs.rs`

**Inject attributes** (0): (none)

**Extract attributes** (0): (none)

**Missing from inject** (3): `access_policy`, `destination_name`, `force_update`

**Missing from extract** (3): `access_policy`, `destination_name`, `force_update`

### `aws_cloudwatch_log_group`

**Source:** `crates/winterbaume-terraform/src/converters/logs.rs`

**Inject attributes** (6): `arn`, `kms_key_id`, `name`, `region`, `retention_in_days`, `tags`

**Extract attributes** (6): `arn`, `id`, `kms_key_id`, `name`, `retention_in_days`, `tags`

**Missing from inject** (4): `log_group_class`, `name_prefix`, `skip_destroy`, `tags_all`

**Missing from extract** (4): `log_group_class`, `name_prefix`, `skip_destroy`, `tags_all`

### `aws_cloudwatch_log_index_policy`

**Source:** `crates/winterbaume-terraform/src/converters/logs.rs`

**Inject attributes** (0): (none)

**Extract attributes** (0): (none)

**Missing from inject** (2): `log_group_name`, `policy_document`

**Missing from extract** (2): `log_group_name`, `policy_document`

### `aws_cloudwatch_log_metric_filter`

**Source:** `crates/winterbaume-terraform/src/converters/logs.rs`

**Inject attributes** (0): (none)

**Extract attributes** (0): (none)

**Missing from inject** (4): `log_group_name`, `metric_transformation`, `name`, `pattern`

**Missing from extract** (4): `log_group_name`, `metric_transformation`, `name`, `pattern`

### `aws_cloudwatch_log_resource_policy`

**Source:** `crates/winterbaume-terraform/src/converters/logs.rs`

**Inject attributes** (0): (none)

**Extract attributes** (0): (none)

**Missing from inject** (2): `policy_document`, `policy_name`

**Missing from extract** (2): `policy_document`, `policy_name`

### `aws_cloudwatch_log_stream`

**Source:** `crates/winterbaume-terraform/src/converters/logs.rs`

**Inject attributes** (4): `arn`, `log_group_name`, `name`, `region`

**Extract attributes** (4): `arn`, `id`, `log_group_name`, `name`

### `aws_cloudwatch_log_subscription_filter`

**Source:** `crates/winterbaume-terraform/src/converters/logs.rs`

**Inject attributes** (0): (none)

**Extract attributes** (0): (none)

**Missing from inject** (6): `destination_arn`, `distribution`, `filter_pattern`, `log_group_name`, `name`, `role_arn`

**Missing from extract** (6): `destination_arn`, `distribution`, `filter_pattern`, `log_group_name`, `name`, `role_arn`

### `aws_macie2_account`

**Source:** `crates/winterbaume-terraform/src/converters/macie2.rs`

**Inject attributes** (5): `arn`, `email`, `name`, `parent_id`, `region`

**Extract attributes** (6): `created_at`, `finding_publishing_frequency`, `id`, `service_role`, `status`, `updated_at`

**Missing from inject** (5): `created_at`, `finding_publishing_frequency`, `service_role`, `status`, `updated_at`

### `aws_macie2_classification_export_configuration`

**Source:** `crates/winterbaume-terraform/src/converters/macie2.rs`

**Inject attributes** (1): `region`

**Extract attributes** (2): `id`, `s3_destination`

**Missing from inject** (1): `s3_destination`

### `aws_macie2_classification_job`

**Source:** `crates/winterbaume-terraform/src/converters/macie2.rs`

**Inject attributes** (20): `allow_list_ids`, `client_token`, `created_at`, `custom_data_identifier_ids`, `description`, `initial_run`, `job_arn`, `job_id`, `job_status`, `job_type`, `last_run_time`, `managed_data_identifier_ids`, `managed_data_identifier_selector`, `name`, `region`, `s3_job_definition`, `sampling_percentage`, `schedule_frequency`, `tags`, `tags_all`

**Extract attributes** (19): `allow_list_ids`, `client_token`, `created_at`, `custom_data_identifier_ids`, `description`, `id`, `initial_run`, `job_arn`, `job_id`, `job_status`, `job_type`, `last_run_time`, `managed_data_identifier_ids`, `managed_data_identifier_selector`, `name`, `s3_job_definition`, `sampling_percentage`, `schedule_frequency`, `tags`

**Missing from inject** (3): `name_prefix`, `timeouts`, `user_paused_details`

**Missing from extract** (4): `name_prefix`, `tags_all`, `timeouts`, `user_paused_details`

### `aws_macie2_custom_data_identifier`

**Source:** `crates/winterbaume-terraform/src/converters/macie2.rs`

**Inject attributes** (13): `arn`, `created_at`, `description`, `ignore_words`, `keywords`, `maximum_match_distance`, `name`, `name_prefix`, `regex`, `region`, `severity_levels`, `tags`, `tags_all`

**Extract attributes** (13): `arn`, `created_at`, `description`, `id`, `ignore_words`, `keywords`, `maximum_match_distance`, `name`, `occurrences_threshold`, `regex`, `severity`, `severity_levels`, `tags`

**Missing from inject** (1): `timeouts`

**Missing from extract** (3): `name_prefix`, `tags_all`, `timeouts`

### `aws_macie2_findings_filter`

**Source:** `crates/winterbaume-terraform/src/converters/macie2.rs`

**Inject attributes** (10): `action`, `arn`, `description`, `finding_criteria`, `name`, `name_prefix`, `position`, `region`, `tags`, `tags_all`

**Extract attributes** (8): `action`, `arn`, `description`, `finding_criteria`, `id`, `name`, `position`, `tags`

**Missing from inject** (1): `timeouts`

**Missing from extract** (3): `name_prefix`, `tags_all`, `timeouts`

### `aws_macie2_invitation_accepter`

**Source:** `crates/winterbaume-terraform/src/converters/macie2.rs`

**Inject attributes** (3): `administrator_account_id`, `invitation_id`, `region`

**Extract attributes** (3): `administrator_account_id`, `id`, `invitation_id`

**Missing from inject** (1): `timeouts`

**Missing from extract** (1): `timeouts`

### `aws_macie2_member`

**Source:** `crates/winterbaume-terraform/src/converters/macie2.rs`

**Inject attributes** (14): `account_id`, `administrator_account_id`, `arn`, `email`, `invitation_disable_email_notification`, `invitation_message`, `invite`, `invited_at`, `master_account_id`, `region`, `relationship_status`, `status`, `tags`, `updated_at`

**Extract attributes** (10): `account_id`, `administrator_account_id`, `arn`, `email`, `id`, `invite`, `invited_at`, `master_account_id`, `relationship_status`, `updated_at`

**Missing from inject** (2): `tags_all`, `timeouts`

**Missing from extract** (6): `invitation_disable_email_notification`, `invitation_message`, `status`, `tags`, `tags_all`, `timeouts`

### `aws_macie2_organization_admin_account`

**Source:** `crates/winterbaume-terraform/src/converters/macie2.rs`

**Inject attributes** (2): `admin_account_id`, `region`

**Extract attributes** (2): `admin_account_id`, `id`

### `aws_macie2_organization_configuration`

**Source:** `crates/winterbaume-terraform/src/converters/macie2.rs`

**Inject attributes** (2): `auto_enable`, `region`

**Extract attributes** (2): `auto_enable`, `id`

### `aws_medialive_channel`

**Source:** `crates/winterbaume-terraform/src/converters/medialive.rs`

**Inject attributes** (16): `arn`, `cdi_input_specification`, `channel_class`, `channel_id`, `destinations`, `encoder_settings`, `input_attachments`, `input_specification`, `log_level`, `maintenance`, `name`, `pipelines_running_count`, `region`, `role_arn`, `tags`, `vpc`

**Extract attributes** (17): `arn`, `cdi_input_specification`, `channel_class`, `channel_id`, `destinations`, `encoder_settings`, `id`, `input_attachments`, `input_specification`, `log_level`, `maintenance`, `name`, `pipelines_running_count`, `role_arn`, `state`, `tags`, `vpc`

**Missing from inject** (3): `start_channel`, `tags_all`, `timeouts`

**Missing from extract** (3): `start_channel`, `tags_all`, `timeouts`

### `aws_medialive_input`

**Source:** `crates/winterbaume-terraform/src/converters/medialive.rs`

**Inject attributes** (16): `arn`, `attached_channels`, `destinations`, `input_class`, `input_devices`, `input_id`, `input_source_type`, `media_connect_flows`, `name`, `region`, `role_arn`, `security_groups`, `sources`, `tags`, `type`, `vpc`

**Extract attributes** (17): `arn`, `attached_channels`, `destinations`, `id`, `input_class`, `input_devices`, `input_id`, `input_source_type`, `media_connect_flows`, `name`, `role_arn`, `security_groups`, `sources`, `state`, `tags`, `type`, `vpc`

**Missing from inject** (4): `input_partner_ids`, `input_security_groups`, `tags_all`, `timeouts`

**Missing from extract** (4): `input_partner_ids`, `input_security_groups`, `tags_all`, `timeouts`

### `aws_medialive_input_security_group`

**Source:** `crates/winterbaume-terraform/src/converters/medialive.rs`

**Inject attributes** (0): (none)

**Extract attributes** (0): (none)

**Missing from inject** (6): `arn`, `inputs`, `tags`, `tags_all`, `timeouts`, `whitelist_rules`

**Missing from extract** (6): `arn`, `inputs`, `tags`, `tags_all`, `timeouts`, `whitelist_rules`

### `aws_medialive_multiplex`

**Source:** `crates/winterbaume-terraform/src/converters/medialive.rs`

**Inject attributes** (0): (none)

**Extract attributes** (0): (none)

**Missing from inject** (8): `arn`, `availability_zones`, `multiplex_settings`, `name`, `start_multiplex`, `tags`, `tags_all`, `timeouts`

**Missing from extract** (8): `arn`, `availability_zones`, `multiplex_settings`, `name`, `start_multiplex`, `tags`, `tags_all`, `timeouts`

### `aws_medialive_multiplex_program`

**Source:** `crates/winterbaume-terraform/src/converters/medialive.rs`

**Inject attributes** (0): (none)

**Extract attributes** (0): (none)

**Missing from inject** (4): `multiplex_id`, `multiplex_program_settings`, `program_name`, `timeouts`

**Missing from extract** (4): `multiplex_id`, `multiplex_program_settings`, `program_name`, `timeouts`

### `aws_media_package_channel`

**Source:** `crates/winterbaume-terraform/src/converters/mediapackage.rs`

**Inject attributes** (7): `arn`, `authorized`, `latency_mode`, `name`, `region`, `tags`, `type`

**Extract attributes** (6): `arn`, `channel_id`, `created_at`, `description`, `id`, `tags`

**Missing from inject** (4): `channel_id`, `description`, `hls_ingest`, `tags_all`

**Missing from extract** (2): `hls_ingest`, `tags_all`

### `aws_media_packagev2_channel_group`

**Source:** `crates/winterbaume-terraform/src/converters/mediapackagev2.rs`

**Inject attributes** (9): `arn`, `created_at`, `description`, `e_tag`, `egress_domain`, `modified_at`, `name`, `region`, `tags`

**Extract attributes** (9): `arn`, `created_at`, `description`, `e_tag`, `egress_domain`, `id`, `modified_at`, `name`, `tags`

**Missing from inject** (1): `tags_all`

**Missing from extract** (1): `tags_all`

### `aws_media_store_container`

**Source:** `crates/winterbaume-terraform/src/converters/mediastore.rs`

**Inject attributes** (5): `arn`, `endpoint`, `name`, `region`, `tags`

**Extract attributes** (5): `arn`, `endpoint`, `id`, `name`, `tags`

**Missing from inject** (1): `tags_all`

**Missing from extract** (1): `tags_all`

### `aws_memorydb_acl`

**Source:** `crates/winterbaume-terraform/src/converters/memorydb.rs`

**Inject attributes** (7): `arn`, `minimum_engine_version`, `name`, `region`, `status`, `tags`, `user_names`

**Extract attributes** (7): `arn`, `id`, `minimum_engine_version`, `name`, `status`, `tags`, `user_names`

**Missing from inject** (2): `name_prefix`, `tags_all`

**Missing from extract** (2): `name_prefix`, `tags_all`

### `aws_memorydb_cluster`

**Source:** `crates/winterbaume-terraform/src/converters/memorydb.rs`

**Inject attributes** (18): `acl_name`, `arn`, `auto_minor_version_upgrade`, `description`, `engine`, `engine_version`, `maintenance_window`, `name`, `node_type`, `num_replicas_per_shard`, `num_shards`, `parameter_group_name`, `region`, `security_group_ids`, `snapshot_retention_limit`, `snapshot_window`, `subnet_group_name`, `tls_enabled`

**Extract attributes** (21): `acl_name`, `arn`, `auto_minor_version_upgrade`, `creation_time`, `description`, `engine`, `engine_version`, `id`, `maintenance_window`, `name`, `node_type`, `num_replicas_per_shard`, `num_shards`, `parameter_group_name`, `security_group_ids`, `snapshot_retention_limit`, `snapshot_window`, `status`, `subnet_group_name`, `tags_all`, `tls_enabled`

**Missing from inject** (15): `cluster_endpoint`, `data_tiering`, `engine_patch_version`, `final_snapshot_name`, `kms_key_arn`, `multi_region_cluster_name`, `name_prefix`, `port`, `shards`, `snapshot_arns`, `snapshot_name`, `sns_topic_arn`, `tags`, `tags_all`, `timeouts`

**Missing from extract** (14): `cluster_endpoint`, `data_tiering`, `engine_patch_version`, `final_snapshot_name`, `kms_key_arn`, `multi_region_cluster_name`, `name_prefix`, `port`, `shards`, `snapshot_arns`, `snapshot_name`, `sns_topic_arn`, `tags`, `timeouts`

### `aws_memorydb_multi_region_cluster`

**Source:** `crates/winterbaume-terraform/src/converters/memorydb.rs`

**Inject attributes** (11): `arn`, `description`, `engine`, `engine_version`, `multi_region_cluster_name`, `multi_region_cluster_name_suffix`, `multi_region_parameter_group_name`, `node_type`, `num_shards`, `tls_enabled`, `update_strategy`

**Extract attributes** (0): (none)

**Missing from inject** (4): `status`, `tags`, `tags_all`, `timeouts`

**Missing from extract** (15): `arn`, `description`, `engine`, `engine_version`, `multi_region_cluster_name`, `multi_region_cluster_name_suffix`, `multi_region_parameter_group_name`, `node_type`, `num_shards`, `status`, `tags`, `tags_all`, `timeouts`, `tls_enabled`, `update_strategy`

### `aws_memorydb_parameter_group`

**Source:** `crates/winterbaume-terraform/src/converters/memorydb.rs`

**Inject attributes** (4): `arn`, `description`, `family`, `name`

**Extract attributes** (0): (none)

**Missing from inject** (4): `name_prefix`, `parameter`, `tags`, `tags_all`

**Missing from extract** (8): `arn`, `description`, `family`, `name`, `name_prefix`, `parameter`, `tags`, `tags_all`

### `aws_memorydb_snapshot`

**Source:** `crates/winterbaume-terraform/src/converters/memorydb.rs`

**Inject attributes** (6): `arn`, `cluster_name`, `kms_key_arn`, `name`, `region`, `source`

**Extract attributes** (7): `arn`, `cluster_name`, `id`, `kms_key_arn`, `name`, `source`, `tags_all`

**Missing from inject** (5): `cluster_configuration`, `name_prefix`, `tags`, `tags_all`, `timeouts`

**Missing from extract** (4): `cluster_configuration`, `name_prefix`, `tags`, `timeouts`

### `aws_memorydb_subnet_group`

**Source:** `crates/winterbaume-terraform/src/converters/memorydb.rs`

**Inject attributes** (6): `arn`, `description`, `name`, `region`, `subnet_ids`, `vpc_id`

**Extract attributes** (6): `arn`, `description`, `id`, `name`, `subnet_ids`, `vpc_id`

**Missing from inject** (3): `name_prefix`, `tags`, `tags_all`

**Missing from extract** (3): `name_prefix`, `tags`, `tags_all`

### `aws_memorydb_user`

**Source:** `crates/winterbaume-terraform/src/converters/memorydb.rs`

**Inject attributes** (6): `arn`, `home_directory`, `home_directory_type`, `role`, `server_id`, `user_name`

**Extract attributes** (0): (none)

**Missing from inject** (5): `access_string`, `authentication_mode`, `minimum_engine_version`, `tags`, `tags_all`

**Missing from extract** (7): `access_string`, `arn`, `authentication_mode`, `minimum_engine_version`, `tags`, `tags_all`, `user_name`

### `aws_mq_broker`

**Source:** `crates/winterbaume-terraform/src/converters/mq.rs`

**Inject attributes** (19): `apply_immediately`, `arn`, `auto_minor_version_upgrade`, `broker_name`, `configuration`, `data_replication_mode`, `deployment_mode`, `encryption_options`, `engine_type`, `engine_version`, `host_instance_type`, `ldap_server_metadata`, `logs`, `maintenance_window_start_time`, `publicly_accessible`, `region`, `tags`, `tags_all`, `user`

**Extract attributes** (24): `arn`, `audit`, `auto_minor_version_upgrade`, `broker_id`, `broker_name`, `configuration`, `console_access`, `deployment_mode`, `encryption_options`, `engine_type`, `engine_version`, `general`, `groups`, `host_instance_type`, `id`, `instances`, `ldap_server_metadata`, `logs`, `maintenance_window_start_time`, `publicly_accessible`, `tags`, `tags_all`, `user`, `username`

**Missing from inject** (8): `authentication_strategy`, `data_replication_primary_broker_arn`, `instances`, `pending_data_replication_mode`, `security_groups`, `storage_type`, `subnet_ids`, `timeouts`

**Missing from extract** (9): `apply_immediately`, `authentication_strategy`, `data_replication_mode`, `data_replication_primary_broker_arn`, `pending_data_replication_mode`, `security_groups`, `storage_type`, `subnet_ids`, `timeouts`

### `aws_mq_configuration`

**Source:** `crates/winterbaume-terraform/src/converters/mq.rs`

**Inject attributes** (10): `arn`, `authentication_strategy`, `data`, `description`, `engine_type`, `engine_version`, `latest_revision`, `name`, `region`, `tags`

**Extract attributes** (11): `arn`, `authentication_strategy`, `data`, `description`, `engine_type`, `engine_version`, `id`, `latest_revision`, `name`, `tags`, `tags_all`

**Missing from inject** (1): `tags_all`

### `aws_neptune_cluster`

**Source:** `crates/winterbaume-terraform/src/converters/neptune.rs`

**Inject attributes** (24): `arn`, `availability_zones`, `backup_retention_period`, `cluster_identifier`, `copy_tags_to_snapshot`, `database_name`, `deletion_protection`, `endpoint`, `engine`, `engine_mode`, `engine_version`, `iam_roles`, `kms_key_arn`, `master_username`, `neptune_cluster_parameter_group_name`, `neptune_subnet_group_name`, `port`, `reader_endpoint`, `region`, `serverless_v2_scaling_configuration`, `storage_encrypted`, `tags`, `tags_all`, `vpc_security_group_ids`

**Extract attributes** (36): `apply_immediately`, `arn`, `availability_zones`, `backup_retention_period`, `cluster_create_time`, `cluster_identifier`, `cluster_members`, `copy_tags_to_snapshot`, `database_name`, `db_cluster_parameter_group_status`, `db_instance_identifier`, `deletion_protection`, `endpoint`, `engine`, `engine_mode`, `engine_version`, `iam_roles`, `id`, `is_cluster_writer`, `kms_key_arn`, `master_username`, `max_capacity`, `min_capacity`, `neptune_cluster_parameter_group_name`, `neptune_subnet_group_name`, `port`, `preferred_backup_window`, `promotion_tier`, `reader_endpoint`, `serverless_v2_scaling_configuration`, `skip_final_snapshot`, `status`, `storage_encrypted`, `tags`, `tags_all`, `vpc_security_group_ids`

**Missing from inject** (18): `allow_major_version_upgrade`, `apply_immediately`, `cluster_identifier_prefix`, `cluster_members`, `cluster_resource_id`, `enable_cloudwatch_logs_exports`, `final_snapshot_identifier`, `global_cluster_identifier`, `hosted_zone_id`, `iam_database_authentication_enabled`, `neptune_instance_parameter_group_name`, `preferred_backup_window`, `preferred_maintenance_window`, `replication_source_identifier`, `skip_final_snapshot`, `snapshot_identifier`, `storage_type`, `timeouts`

**Missing from extract** (14): `allow_major_version_upgrade`, `cluster_identifier_prefix`, `cluster_resource_id`, `enable_cloudwatch_logs_exports`, `final_snapshot_identifier`, `global_cluster_identifier`, `hosted_zone_id`, `iam_database_authentication_enabled`, `neptune_instance_parameter_group_name`, `preferred_maintenance_window`, `replication_source_identifier`, `snapshot_identifier`, `storage_type`, `timeouts`

### `aws_neptune_cluster_endpoint`

**Source:** `crates/winterbaume-terraform/src/converters/neptune.rs`

**Inject attributes** (8): `arn`, `cluster_endpoint_identifier`, `cluster_identifier`, `endpoint`, `endpoint_type`, `excluded_members`, `region`, `static_members`

**Extract attributes** (8): `arn`, `cluster_endpoint_identifier`, `cluster_identifier`, `endpoint`, `endpoint_type`, `excluded_members`, `id`, `static_members`

**Missing from inject** (2): `tags`, `tags_all`

**Missing from extract** (2): `tags`, `tags_all`

### `aws_neptune_cluster_instance`

**Source:** `crates/winterbaume-terraform/src/converters/neptune.rs`

**Inject attributes** (19): `arn`, `auto_minor_version_upgrade`, `availability_zone`, `cluster_identifier`, `endpoint`, `engine`, `engine_version`, `identifier`, `instance_class`, `kms_key_arn`, `neptune_parameter_group_name`, `neptune_subnet_group_name`, `port`, `publicly_accessible`, `region`, `storage_encrypted`, `tags`, `tags_all`, `vpc_security_group_ids`

**Extract attributes** (21): `arn`, `auto_minor_version_upgrade`, `availability_zone`, `cluster_identifier`, `endpoint`, `engine`, `engine_version`, `id`, `identifier`, `instance_class`, `instance_create_time`, `kms_key_arn`, `neptune_parameter_group_name`, `neptune_subnet_group_name`, `port`, `publicly_accessible`, `status`, `storage_encrypted`, `tags`, `tags_all`, `vpc_security_group_ids`

**Missing from inject** (11): `address`, `apply_immediately`, `dbi_resource_id`, `identifier_prefix`, `preferred_backup_window`, `preferred_maintenance_window`, `promotion_tier`, `skip_final_snapshot`, `storage_type`, `timeouts`, `writer`

**Missing from extract** (11): `address`, `apply_immediately`, `dbi_resource_id`, `identifier_prefix`, `preferred_backup_window`, `preferred_maintenance_window`, `promotion_tier`, `skip_final_snapshot`, `storage_type`, `timeouts`, `writer`

### `aws_neptune_cluster_parameter_group`

**Source:** `crates/winterbaume-terraform/src/converters/neptune.rs`

**Inject attributes** (8): `arn`, `description`, `family`, `name`, `parameter`, `region`, `tags`, `tags_all`

**Extract attributes** (9): `arn`, `description`, `family`, `id`, `name`, `parameter`, `tags`, `tags_all`, `value`

**Missing from inject** (1): `name_prefix`

**Missing from extract** (1): `name_prefix`

### `aws_neptune_cluster_snapshot`

**Source:** `crates/winterbaume-terraform/src/converters/neptune.rs`

**Inject attributes** (16): `allocated_storage`, `availability_zones`, `db_cluster_identifier`, `db_cluster_snapshot_arn`, `db_cluster_snapshot_identifier`, `engine`, `engine_version`, `kms_key_id`, `port`, `region`, `snapshot_type`, `status`, `storage_encrypted`, `tags`, `tags_all`, `vpc_id`

**Extract attributes** (16): `allocated_storage`, `availability_zones`, `db_cluster_identifier`, `db_cluster_snapshot_arn`, `db_cluster_snapshot_identifier`, `engine`, `engine_version`, `id`, `kms_key_id`, `port`, `snapshot_type`, `status`, `storage_encrypted`, `tags`, `tags_all`, `vpc_id`

**Missing from inject** (3): `license_model`, `source_db_cluster_snapshot_arn`, `timeouts`

**Missing from extract** (3): `license_model`, `source_db_cluster_snapshot_arn`, `timeouts`

### `aws_neptune_event_subscription`

**Source:** `crates/winterbaume-terraform/src/converters/neptune.rs`

**Inject attributes** (8): `arn`, `enabled`, `event_categories`, `name`, `region`, `sns_topic_arn`, `source_ids`, `source_type`

**Extract attributes** (10): `arn`, `customer_aws_id`, `enabled`, `event_categories`, `id`, `name`, `sns_topic_arn`, `source_ids`, `source_type`, `status`

**Missing from inject** (5): `customer_aws_id`, `name_prefix`, `tags`, `tags_all`, `timeouts`

**Missing from extract** (4): `name_prefix`, `tags`, `tags_all`, `timeouts`

### `aws_neptune_global_cluster`

**Source:** `crates/winterbaume-terraform/src/converters/neptune.rs`

**Inject attributes** (9): `arn`, `database_name`, `deletion_protection`, `engine`, `engine_version`, `global_cluster_identifier`, `region`, `source_db_cluster_identifier`, `storage_encrypted`

**Extract attributes** (10): `arn`, `database_name`, `deletion_protection`, `engine`, `engine_version`, `global_cluster_identifier`, `id`, `source_db_cluster_identifier`, `status`, `storage_encrypted`

**Missing from inject** (4): `global_cluster_members`, `global_cluster_resource_id`, `status`, `timeouts`

**Missing from extract** (3): `global_cluster_members`, `global_cluster_resource_id`, `timeouts`

### `aws_neptune_parameter_group`

**Source:** `crates/winterbaume-terraform/src/converters/neptune.rs`

**Inject attributes** (8): `arn`, `description`, `family`, `name`, `parameter`, `region`, `tags`, `tags_all`

**Extract attributes** (9): `arn`, `description`, `family`, `id`, `name`, `parameter`, `tags`, `tags_all`, `value`

**Missing from inject** (1): `name_prefix`

**Missing from extract** (1): `name_prefix`

### `aws_neptune_subnet_group`

**Source:** `crates/winterbaume-terraform/src/converters/neptune.rs`

**Inject attributes** (8): `arn`, `description`, `name`, `region`, `subnet_ids`, `tags`, `tags_all`, `vpc_id`

**Extract attributes** (9): `arn`, `description`, `id`, `name`, `status`, `subnet_ids`, `tags`, `tags_all`, `vpc_id`

**Missing from inject** (1): `name_prefix`

**Missing from extract** (1): `name_prefix`

### `aws_networkfirewall_firewall`

**Source:** `crates/winterbaume-terraform/src/converters/networkfirewall.rs`

**Inject attributes** (11): `arn`, `delete_protection`, `description`, `firewall_policy_arn`, `firewall_policy_change_protection`, `firewall_status`, `name`, `region`, `subnet_change_protection`, `subnet_mapping`, `vpc_id`

**Extract attributes** (14): `arn`, `delete_protection`, `description`, `encryption_configuration`, `firewall_policy_arn`, `firewall_policy_change_protection`, `id`, `name`, `subnet_change_protection`, `subnet_id`, `subnet_mapping`, `tags`, `tags_all`, `vpc_id`

**Missing from inject** (6): `enabled_analysis_types`, `encryption_configuration`, `tags`, `tags_all`, `timeouts`, `update_token`

**Missing from extract** (4): `enabled_analysis_types`, `firewall_status`, `timeouts`, `update_token`

### `aws_networkfirewall_firewall_policy`

**Source:** `crates/winterbaume-terraform/src/converters/networkfirewall.rs`

**Inject attributes** (5): `arn`, `description`, `firewall_policy`, `name`, `region`

**Extract attributes** (8): `arn`, `description`, `encryption_configuration`, `firewall_policy`, `id`, `name`, `tags`, `tags_all`

**Missing from inject** (4): `encryption_configuration`, `tags`, `tags_all`, `update_token`

**Missing from extract** (1): `update_token`

### `aws_networkfirewall_logging_configuration`

**Source:** `crates/winterbaume-terraform/src/converters/networkfirewall.rs`

**Inject attributes** (0): (none)

**Extract attributes** (0): (none)

**Missing from inject** (2): `firewall_arn`, `logging_configuration`

**Missing from extract** (2): `firewall_arn`, `logging_configuration`

### `aws_networkfirewall_resource_policy`

**Source:** `crates/winterbaume-terraform/src/converters/networkfirewall.rs`

**Inject attributes** (0): (none)

**Extract attributes** (0): (none)

**Missing from inject** (2): `policy`, `resource_arn`

**Missing from extract** (2): `policy`, `resource_arn`

### `aws_networkfirewall_rule_group`

**Source:** `crates/winterbaume-terraform/src/converters/networkfirewall.rs`

**Inject attributes** (8): `arn`, `capacity`, `description`, `name`, `region`, `rule_group`, `rules`, `type`

**Extract attributes** (10): `arn`, `capacity`, `description`, `encryption_configuration`, `id`, `name`, `rule_group`, `rules`, `tags`, `type`

**Missing from inject** (4): `encryption_configuration`, `tags`, `tags_all`, `update_token`

**Missing from extract** (2): `tags_all`, `update_token`

### `aws_networkfirewall_tls_inspection_configuration`

**Source:** `crates/winterbaume-terraform/src/converters/networkfirewall.rs`

**Inject attributes** (0): (none)

**Extract attributes** (0): (none)

**Missing from inject** (13): `arn`, `certificate_authority`, `certificates`, `description`, `encryption_configuration`, `name`, `number_of_associations`, `tags`, `tags_all`, `timeouts`, `tls_inspection_configuration`, `tls_inspection_configuration_id`, `update_token`

**Missing from extract** (13): `arn`, `certificate_authority`, `certificates`, `description`, `encryption_configuration`, `name`, `number_of_associations`, `tags`, `tags_all`, `timeouts`, `tls_inspection_configuration`, `tls_inspection_configuration_id`, `update_token`

### `aws_location`

**Source:** `crates/winterbaume-terraform/src/converters/networkmanager.rs`

**Inject attributes** (0): (none)

**Extract attributes** (13): `arn`, `created_at`, `description`, `global_network_id`, `id`, `location`, `model`, `serial_number`, `site_id`, `state`, `tags`, `type`, `vendor`

**Note:** Resource type not found in Terraform AWS provider schema.

### `aws_networkmanager_attachment_accepter`

**Source:** `crates/winterbaume-terraform/src/converters/networkmanager.rs`

**Inject attributes** (4): `attachment_id`, `attachment_type`, `region`, `state`

**Extract attributes** (0): (none)

**Missing from inject** (9): `attachment_policy_rule_number`, `core_network_arn`, `core_network_id`, `edge_location`, `edge_locations`, `owner_account_id`, `resource_arn`, `segment_name`, `timeouts`

**Missing from extract** (12): `attachment_id`, `attachment_policy_rule_number`, `attachment_type`, `core_network_arn`, `core_network_id`, `edge_location`, `edge_locations`, `owner_account_id`, `resource_arn`, `segment_name`, `state`, `timeouts`

### `aws_networkmanager_connect_attachment`

**Source:** `crates/winterbaume-terraform/src/converters/networkmanager.rs`

**Inject attributes** (11): `arn`, `attachment_policy_rule_number`, `core_network_id`, `edge_location`, `owner_account_id`, `region`, `resource_arn`, `segment_name`, `state`, `tags`, `transport_attachment_id`

**Extract attributes** (9): `arn`, `core_network_id`, `edge_location`, `id`, `owner_account_id`, `resource_arn`, `segment_name`, `state`, `tags`

**Missing from inject** (6): `attachment_id`, `attachment_type`, `core_network_arn`, `options`, `tags_all`, `timeouts`

**Missing from extract** (8): `attachment_id`, `attachment_policy_rule_number`, `attachment_type`, `core_network_arn`, `options`, `tags_all`, `timeouts`, `transport_attachment_id`

### `aws_networkmanager_connect_peer`

**Source:** `crates/winterbaume-terraform/src/converters/networkmanager.rs`

**Inject attributes** (11): `arn`, `connect_attachment_id`, `core_network_address`, `core_network_id`, `created_at`, `edge_location`, `inside_cidr_blocks`, `peer_address`, `region`, `state`, `tags`

**Extract attributes** (10): `arn`, `connect_attachment_id`, `core_network_address`, `core_network_id`, `created_at`, `edge_location`, `id`, `peer_address`, `state`, `tags`

**Missing from inject** (6): `bgp_options`, `configuration`, `connect_peer_id`, `subnet_arn`, `tags_all`, `timeouts`

**Missing from extract** (7): `bgp_options`, `configuration`, `connect_peer_id`, `inside_cidr_blocks`, `subnet_arn`, `tags_all`, `timeouts`

### `aws_networkmanager_connection`

**Source:** `crates/winterbaume-terraform/src/converters/networkmanager.rs`

**Inject attributes** (4): `connection_type`, `description`, `name`, `region`

**Extract attributes** (11): `arn`, `connected_device_id`, `connected_link_id`, `created_at`, `description`, `device_id`, `global_network_id`, `id`, `link_id`, `state`, `tags`

**Missing from inject** (9): `arn`, `connected_device_id`, `connected_link_id`, `device_id`, `global_network_id`, `link_id`, `tags`, `tags_all`, `timeouts`

**Missing from extract** (2): `tags_all`, `timeouts`

### `aws_networkmanager_core_network`

**Source:** `crates/winterbaume-terraform/src/converters/networkmanager.rs`

**Inject attributes** (7): `arn`, `created_at`, `description`, `global_network_id`, `region`, `state`, `tags`

**Extract attributes** (7): `arn`, `created_at`, `description`, `global_network_id`, `id`, `state`, `tags`

**Missing from inject** (8): `base_policy_document`, `base_policy_region`, `base_policy_regions`, `create_base_policy`, `edges`, `segments`, `tags_all`, `timeouts`

**Missing from extract** (8): `base_policy_document`, `base_policy_region`, `base_policy_regions`, `create_base_policy`, `edges`, `segments`, `tags_all`, `timeouts`

### `aws_networkmanager_core_network_policy_attachment`

**Source:** `crates/winterbaume-terraform/src/converters/networkmanager.rs`

**Inject attributes** (4): `core_network_id`, `policy_document`, `region`, `state`

**Extract attributes** (0): (none)

**Missing from inject** (1): `timeouts`

**Missing from extract** (4): `core_network_id`, `policy_document`, `state`, `timeouts`

### `aws_networkmanager_customer_gateway_association`

**Source:** `crates/winterbaume-terraform/src/converters/networkmanager.rs`

**Inject attributes** (6): `customer_gateway_arn`, `device_id`, `global_network_id`, `link_id`, `region`, `state`

**Extract attributes** (5): `customer_gateway_arn`, `device_id`, `global_network_id`, `link_id`, `state`

**Missing from inject** (1): `timeouts`

**Missing from extract** (1): `timeouts`

### `aws_networkmanager_device`

**Source:** `crates/winterbaume-terraform/src/converters/networkmanager.rs`

**Inject attributes** (14): `arn`, `aws_location`, `created_at`, `description`, `global_network_id`, `location`, `model`, `region`, `serial_number`, `site_id`, `state`, `tags`, `type`, `vendor`

**Extract attributes** (14): `arn`, `aws_location`, `created_at`, `description`, `global_network_id`, `id`, `location`, `model`, `serial_number`, `site_id`, `state`, `tags`, `type`, `vendor`

**Missing from inject** (2): `tags_all`, `timeouts`

**Missing from extract** (2): `tags_all`, `timeouts`

### `aws_networkmanager_dx_gateway_attachment`

**Source:** `crates/winterbaume-terraform/src/converters/networkmanager.rs`

**Inject attributes** (10): `arn`, `attachment_policy_rule_number`, `core_network_id`, `direct_connect_gateway_arn`, `edge_locations`, `owner_account_id`, `region`, `segment_name`, `state`, `tags`

**Extract attributes** (9): `arn`, `core_network_id`, `direct_connect_gateway_arn`, `edge_locations`, `id`, `owner_account_id`, `segment_name`, `state`, `tags`

**Missing from inject** (4): `attachment_type`, `core_network_arn`, `tags_all`, `timeouts`

**Missing from extract** (5): `attachment_policy_rule_number`, `attachment_type`, `core_network_arn`, `tags_all`, `timeouts`

### `aws_networkmanager_global_network`

**Source:** `crates/winterbaume-terraform/src/converters/networkmanager.rs`

**Inject attributes** (6): `arn`, `created_at`, `description`, `region`, `state`, `tags`

**Extract attributes** (6): `arn`, `created_at`, `description`, `id`, `state`, `tags`

**Missing from inject** (2): `tags_all`, `timeouts`

**Missing from extract** (2): `tags_all`, `timeouts`

### `aws_networkmanager_link`

**Source:** `crates/winterbaume-terraform/src/converters/networkmanager.rs`

**Inject attributes** (11): `arn`, `bandwidth`, `created_at`, `description`, `global_network_id`, `provider_name`, `region`, `site_id`, `state`, `tags`, `type`

**Extract attributes** (10): `arn`, `created_at`, `description`, `global_network_id`, `id`, `provider_name`, `site_id`, `state`, `tags`, `type`

**Missing from inject** (2): `tags_all`, `timeouts`

**Missing from extract** (3): `bandwidth`, `tags_all`, `timeouts`

### `aws_networkmanager_link_association`

**Source:** `crates/winterbaume-terraform/src/converters/networkmanager.rs`

**Inject attributes** (5): `device_id`, `global_network_id`, `link_id`, `region`, `state`

**Extract attributes** (4): `device_id`, `global_network_id`, `link_id`, `state`

**Missing from inject** (1): `timeouts`

**Missing from extract** (1): `timeouts`

### `aws_networkmanager_site`

**Source:** `crates/winterbaume-terraform/src/converters/networkmanager.rs`

**Inject attributes** (8): `arn`, `created_at`, `description`, `global_network_id`, `location`, `region`, `state`, `tags`

**Extract attributes** (8): `arn`, `created_at`, `description`, `global_network_id`, `id`, `location`, `state`, `tags`

**Missing from inject** (2): `tags_all`, `timeouts`

**Missing from extract** (2): `tags_all`, `timeouts`

### `aws_networkmanager_site_to_site_vpn_attachment`

**Source:** `crates/winterbaume-terraform/src/converters/networkmanager.rs`

**Inject attributes** (10): `arn`, `attachment_policy_rule_number`, `core_network_id`, `edge_location`, `owner_account_id`, `region`, `segment_name`, `state`, `tags`, `vpn_connection_arn`

**Extract attributes** (9): `arn`, `core_network_id`, `edge_location`, `id`, `owner_account_id`, `segment_name`, `state`, `tags`, `vpn_connection_arn`

**Missing from inject** (5): `attachment_type`, `core_network_arn`, `resource_arn`, `tags_all`, `timeouts`

**Missing from extract** (6): `attachment_policy_rule_number`, `attachment_type`, `core_network_arn`, `resource_arn`, `tags_all`, `timeouts`

### `aws_networkmanager_transit_gateway_connect_peer_association`

**Source:** `crates/winterbaume-terraform/src/converters/networkmanager.rs`

**Inject attributes** (6): `device_id`, `global_network_id`, `link_id`, `region`, `state`, `transit_gateway_connect_peer_arn`

**Extract attributes** (5): `device_id`, `global_network_id`, `link_id`, `state`, `transit_gateway_connect_peer_arn`

**Missing from inject** (1): `timeouts`

**Missing from extract** (1): `timeouts`

### `aws_networkmanager_transit_gateway_peering`

**Source:** `crates/winterbaume-terraform/src/converters/networkmanager.rs`

**Inject attributes** (10): `arn`, `core_network_id`, `edge_location`, `owner_account_id`, `peering_type`, `region`, `resource_arn`, `state`, `tags`, `transit_gateway_arn`

**Extract attributes** (0): (none)

**Missing from inject** (4): `core_network_arn`, `tags_all`, `timeouts`, `transit_gateway_peering_attachment_id`

**Missing from extract** (12): `arn`, `core_network_arn`, `core_network_id`, `edge_location`, `owner_account_id`, `peering_type`, `resource_arn`, `tags`, `tags_all`, `timeouts`, `transit_gateway_arn`, `transit_gateway_peering_attachment_id`

### `aws_networkmanager_transit_gateway_registration`

**Source:** `crates/winterbaume-terraform/src/converters/networkmanager.rs`

**Inject attributes** (4): `global_network_id`, `region`, `state`, `transit_gateway_arn`

**Extract attributes** (3): `global_network_id`, `state`, `transit_gateway_arn`

**Missing from inject** (1): `timeouts`

**Missing from extract** (1): `timeouts`

### `aws_networkmanager_transit_gateway_route_table_attachment`

**Source:** `crates/winterbaume-terraform/src/converters/networkmanager.rs`

**Inject attributes** (11): `arn`, `attachment_policy_rule_number`, `core_network_id`, `edge_location`, `owner_account_id`, `peering_id`, `region`, `segment_name`, `state`, `tags`, `transit_gateway_route_table_arn`

**Extract attributes** (9): `arn`, `core_network_id`, `edge_location`, `id`, `owner_account_id`, `segment_name`, `state`, `tags`, `transit_gateway_route_table_arn`

**Missing from inject** (5): `attachment_type`, `core_network_arn`, `resource_arn`, `tags_all`, `timeouts`

**Missing from extract** (7): `attachment_policy_rule_number`, `attachment_type`, `core_network_arn`, `peering_id`, `resource_arn`, `tags_all`, `timeouts`

### `aws_networkmanager_vpc_attachment`

**Source:** `crates/winterbaume-terraform/src/converters/networkmanager.rs`

**Inject attributes** (11): `arn`, `attachment_policy_rule_number`, `core_network_id`, `edge_location`, `owner_account_id`, `region`, `segment_name`, `state`, `subnet_arns`, `tags`, `vpc_arn`

**Extract attributes** (10): `arn`, `core_network_id`, `edge_location`, `id`, `owner_account_id`, `segment_name`, `state`, `subnet_arns`, `tags`, `vpc_arn`

**Missing from inject** (6): `attachment_type`, `core_network_arn`, `options`, `resource_arn`, `tags_all`, `timeouts`

**Missing from extract** (7): `attachment_policy_rule_number`, `attachment_type`, `core_network_arn`, `options`, `resource_arn`, `tags_all`, `timeouts`

### `aws_opensearch_authorize_vpc_endpoint_access`

**Source:** `crates/winterbaume-terraform/src/converters/opensearch.rs`

**Inject attributes** (3): `account`, `domain_name`, `region`

**Extract attributes** (6): `account`, `authorized_principal`, `domain_name`, `id`, `principal`, `principal_type`

**Missing from inject** (1): `authorized_principal`

### `aws_opensearch_domain`

**Source:** `crates/winterbaume-terraform/src/converters/opensearch.rs`

**Inject attributes** (14): `app_network_access_type`, `arn`, `auth_mode`, `cluster_config`, `creation_time`, `domain_name`, `ebs_options`, `home_efs_file_system_id`, `kms_key_id`, `last_modified_time`, `region`, `status`, `url`, `vpc_id`

**Extract attributes** (24): `access_policies`, `advanced_options`, `arn`, `cluster_config`, `dashboard_endpoint`, `dedicated_master_enabled`, `domain_id`, `domain_name`, `ebs_enabled`, `ebs_options`, `enabled`, `endpoint`, `engine_version`, `id`, `instance_count`, `instance_type`, `kibana_endpoint`, `node_to_node_encryption`, `tags`, `tags_all`, `volume_size`, `volume_type`, `vpc_options`, `zone_awareness_enabled`

**Missing from inject** (25): `access_policies`, `advanced_options`, `advanced_security_options`, `auto_tune_options`, `cognito_options`, `dashboard_endpoint`, `dashboard_endpoint_v2`, `domain_endpoint_options`, `domain_endpoint_v2_hosted_zone_id`, `domain_id`, `encrypt_at_rest`, `endpoint`, `endpoint_v2`, `engine_version`, `ip_address_type`, `kibana_endpoint`, `log_publishing_options`, `node_to_node_encryption`, `off_peak_window_options`, `snapshot_options`, `software_update_options`, `tags`, `tags_all`, `timeouts`, `vpc_options`

**Missing from extract** (14): `advanced_security_options`, `auto_tune_options`, `cognito_options`, `dashboard_endpoint_v2`, `domain_endpoint_options`, `domain_endpoint_v2_hosted_zone_id`, `encrypt_at_rest`, `endpoint_v2`, `ip_address_type`, `log_publishing_options`, `off_peak_window_options`, `snapshot_options`, `software_update_options`, `timeouts`

### `aws_opensearch_domain_policy`

**Source:** `crates/winterbaume-terraform/src/converters/opensearch.rs`

**Inject attributes** (3): `access_policies`, `domain_name`, `region`

**Extract attributes** (3): `access_policies`, `domain_name`, `id`

**Missing from inject** (1): `timeouts`

**Missing from extract** (1): `timeouts`

### `aws_opensearch_domain_saml_options`

**Source:** `crates/winterbaume-terraform/src/converters/opensearch.rs`

**Inject attributes** (2): `domain_name`, `region`

**Extract attributes** (0): (none)

**Missing from inject** (2): `saml_options`, `timeouts`

**Missing from extract** (3): `domain_name`, `saml_options`, `timeouts`

### `aws_opensearch_inbound_connection_accepter`

**Source:** `crates/winterbaume-terraform/src/converters/opensearch.rs`

**Inject attributes** (2): `connection_id`, `region`

**Extract attributes** (3): `connection_id`, `connection_status`, `id`

**Missing from inject** (2): `connection_status`, `timeouts`

**Missing from extract** (1): `timeouts`

### `aws_opensearch_outbound_connection`

**Source:** `crates/winterbaume-terraform/src/converters/opensearch.rs`

**Inject attributes** (7): `connection_alias`, `connection_id`, `connection_mode`, `connection_status`, `local_domain_info`, `region`, `remote_domain_info`

**Extract attributes** (10): `connection_alias`, `connection_id`, `connection_mode`, `connection_status`, `domain_name`, `id`, `local_domain_info`, `owner_id`, `region`, `remote_domain_info`

**Missing from inject** (3): `accept_connection`, `connection_properties`, `timeouts`

**Missing from extract** (3): `accept_connection`, `connection_properties`, `timeouts`

### `aws_opensearch_package`

**Source:** `crates/winterbaume-terraform/src/converters/opensearch.rs`

**Inject attributes** (6): `engine_version`, `id`, `package_description`, `package_name`, `package_type`, `region`

**Extract attributes** (7): `available_package_version`, `engine_version`, `id`, `package_description`, `package_id`, `package_name`, `package_type`

**Missing from inject** (3): `available_package_version`, `package_id`, `package_source`

**Missing from extract** (1): `package_source`

### `aws_opensearch_package_association`

**Source:** `crates/winterbaume-terraform/src/converters/opensearch.rs`

**Inject attributes** (3): `domain_name`, `package_id`, `region`

**Extract attributes** (3): `domain_name`, `id`, `package_id`

**Missing from inject** (2): `reference_path`, `timeouts`

**Missing from extract** (2): `reference_path`, `timeouts`

### `aws_opensearch_vpc_endpoint`

**Source:** `crates/winterbaume-terraform/src/converters/opensearch.rs`

**Inject attributes** (11): `auto_accept`, `endpoint`, `id`, `policy`, `private_dns_enabled`, `region`, `service_name`, `state`, `vpc_endpoint_type`, `vpc_id`, `vpc_options`

**Extract attributes** (8): `domain_arn`, `endpoint`, `id`, `security_group_ids`, `subnet_ids`, `vpc_endpoint_id`, `vpc_endpoint_owner`, `vpc_options`

**Missing from inject** (2): `domain_arn`, `timeouts`

**Missing from extract** (1): `timeouts`

### `aws_opensearchserverless_access_policy`

**Source:** `crates/winterbaume-terraform/src/converters/opensearchserverless.rs`

**Inject attributes** (6): `description`, `name`, `policy`, `policy_version`, `region`, `type`

**Extract attributes** (6): `description`, `id`, `name`, `policy`, `policy_version`, `type`

### `aws_opensearchserverless_collection`

**Source:** `crates/winterbaume-terraform/src/converters/opensearchserverless.rs`

**Inject attributes** (5): `arn`, `description`, `kms_key_arn`, `name`, `region`

**Extract attributes** (7): `arn`, `description`, `id`, `name`, `standby_replicas`, `tags_all`, `type`

**Missing from inject** (7): `collection_endpoint`, `dashboard_endpoint`, `standby_replicas`, `tags`, `tags_all`, `timeouts`, `type`

**Missing from extract** (5): `collection_endpoint`, `dashboard_endpoint`, `kms_key_arn`, `tags`, `timeouts`

### `aws_opensearchserverless_lifecycle_policy`

**Source:** `crates/winterbaume-terraform/src/converters/opensearchserverless.rs`

**Inject attributes** (4): `description`, `name`, `policy`, `type`

**Extract attributes** (0): (none)

**Missing from inject** (1): `policy_version`

**Missing from extract** (5): `description`, `name`, `policy`, `policy_version`, `type`

### `aws_opensearchserverless_security_config`

**Source:** `crates/winterbaume-terraform/src/converters/opensearchserverless.rs`

**Inject attributes** (3): `description`, `name`, `type`

**Extract attributes** (0): (none)

**Missing from inject** (2): `config_version`, `saml_options`

**Missing from extract** (5): `config_version`, `description`, `name`, `saml_options`, `type`

### `aws_opensearchserverless_security_policy`

**Source:** `crates/winterbaume-terraform/src/converters/opensearchserverless.rs`

**Inject attributes** (5): `description`, `name`, `policy`, `policy_version`, `region`

**Extract attributes** (5): `description`, `name`, `policy`, `policy_version`, `type`

**Missing from inject** (1): `type`

### `aws_opensearchserverless_vpc_endpoint`

**Source:** `crates/winterbaume-terraform/src/converters/opensearchserverless.rs`

**Inject attributes** (5): `name`, `region`, `security_group_ids`, `subnet_ids`, `vpc_id`

**Extract attributes** (5): `id`, `name`, `security_group_ids`, `subnet_ids`, `vpc_id`

**Missing from inject** (1): `timeouts`

**Missing from extract** (1): `timeouts`

### `aws_organizations_account`

**Source:** `crates/winterbaume-terraform/src/converters/organizations.rs`

**Inject attributes** (5): `arn`, `email`, `name`, `parent_id`, `region`

**Extract attributes** (6): `arn`, `email`, `id`, `name`, `parent_id`, `status`

**Missing from inject** (11): `close_on_deletion`, `create_govcloud`, `govcloud_id`, `iam_user_access_to_billing`, `joined_method`, `joined_timestamp`, `role_name`, `status`, `tags`, `tags_all`, `timeouts`

**Missing from extract** (10): `close_on_deletion`, `create_govcloud`, `govcloud_id`, `iam_user_access_to_billing`, `joined_method`, `joined_timestamp`, `role_name`, `tags`, `tags_all`, `timeouts`

### `aws_organizations_delegated_administrator`

**Source:** `crates/winterbaume-terraform/src/converters/organizations.rs`

**Inject attributes** (3): `account_id`, `region`, `service_principal`

**Extract attributes** (10): `account_id`, `arn`, `delegation_enabled_date`, `email`, `id`, `joined_method`, `joined_timestamp`, `name`, `service_principal`, `status`

**Missing from inject** (7): `arn`, `delegation_enabled_date`, `email`, `joined_method`, `joined_timestamp`, `name`, `status`

### `aws_organizations_organization`

**Source:** `crates/winterbaume-terraform/src/converters/organizations.rs`

**Inject attributes** (5): `arn`, `feature_set`, `master_account_email`, `master_account_id`, `region`

**Extract attributes** (5): `arn`, `feature_set`, `id`, `master_account_email`, `master_account_id`

**Missing from inject** (7): `accounts`, `aws_service_access_principals`, `enabled_policy_types`, `master_account_arn`, `master_account_name`, `non_master_accounts`, `roots`

**Missing from extract** (7): `accounts`, `aws_service_access_principals`, `enabled_policy_types`, `master_account_arn`, `master_account_name`, `non_master_accounts`, `roots`

### `aws_organizations_organizational_unit`

**Source:** `crates/winterbaume-terraform/src/converters/organizations.rs`

**Inject attributes** (4): `arn`, `name`, `parent_id`, `region`

**Extract attributes** (4): `arn`, `id`, `name`, `parent_id`

**Missing from inject** (3): `accounts`, `tags`, `tags_all`

**Missing from extract** (3): `accounts`, `tags`, `tags_all`

### `aws_organizations_policy`

**Source:** `crates/winterbaume-terraform/src/converters/organizations.rs`

**Inject attributes** (6): `arn`, `content`, `description`, `name`, `region`, `type`

**Extract attributes** (6): `arn`, `content`, `description`, `id`, `name`, `type`

**Missing from inject** (3): `skip_destroy`, `tags`, `tags_all`

**Missing from extract** (3): `skip_destroy`, `tags`, `tags_all`

### `aws_organizations_policy_attachment`

**Source:** `crates/winterbaume-terraform/src/converters/organizations.rs`

**Inject attributes** (3): `policy_id`, `region`, `target_id`

**Extract attributes** (3): `id`, `policy_id`, `target_id`

**Missing from inject** (1): `skip_destroy`

**Missing from extract** (1): `skip_destroy`

### `aws_organizations_resource_policy`

**Source:** `crates/winterbaume-terraform/src/converters/organizations.rs`

**Inject attributes** (3): `arn`, `content`, `region`

**Extract attributes** (3): `arn`, `content`, `id`

**Missing from inject** (2): `tags`, `tags_all`

**Missing from extract** (2): `tags`, `tags_all`

### `aws_osis_pipeline`

**Source:** `crates/winterbaume-terraform/src/converters/osis.rs`

**Inject attributes** (9): `buffer_options`, `description`, `encryption_at_rest_options`, `ingest_endpoint_urls`, `log_publishing_options`, `name`, `region`, `unique_id`, `vpc_options`

**Extract attributes** (12): `buffer_options`, `encryption_at_rest_options`, `id`, `ingest_endpoint_urls`, `log_publishing_options`, `max_units`, `min_units`, `pipeline_arn`, `pipeline_configuration_body`, `pipeline_name`, `tags`, `vpc_options`

**Missing from inject** (8): `max_units`, `min_units`, `pipeline_arn`, `pipeline_configuration_body`, `pipeline_name`, `tags`, `tags_all`, `timeouts`

**Missing from extract** (2): `tags_all`, `timeouts`

### `aws_outposts_outpost`

**Source:** `crates/winterbaume-terraform/src/converters/outposts.rs`

**Inject attributes** (11): `arn`, `availability_zone`, `availability_zone_id`, `description`, `name`, `outpost_id`, `region`, `site_arn`, `site_id`, `supported_hardware_type`, `tags`

**Extract attributes** (13): `arn`, `availability_zone`, `availability_zone_id`, `description`, `id`, `lifecycle_status`, `name`, `outpost_id`, `owner_id`, `site_arn`, `site_id`, `supported_hardware_type`, `tags`

**Note:** Resource type not found in Terraform AWS provider schema.

### `aws_outposts_site`

**Source:** `crates/winterbaume-terraform/src/converters/outposts.rs`

**Inject attributes** (7): `arn`, `created_at`, `description`, `global_network_id`, `region`, `state`, `tags`

**Extract attributes** (6): `arn`, `description`, `id`, `name`, `site_id`, `tags`

**Note:** Resource type not found in Terraform AWS provider schema.

### `aws_pinpoint_adm_channel`

**Source:** `crates/winterbaume-terraform/src/converters/pinpoint.rs`

**Inject attributes** (4): `application_id`, `client_id`, `client_secret`, `enabled`

**Extract attributes** (0): (none)

**Missing from extract** (4): `application_id`, `client_id`, `client_secret`, `enabled`

### `aws_pinpoint_apns_channel`

**Source:** `crates/winterbaume-terraform/src/converters/pinpoint.rs`

**Inject attributes** (9): `application_id`, `bundle_id`, `certificate`, `default_authentication_method`, `enabled`, `private_key`, `team_id`, `token_key`, `token_key_id`

**Extract attributes** (0): (none)

**Missing from extract** (9): `application_id`, `bundle_id`, `certificate`, `default_authentication_method`, `enabled`, `private_key`, `team_id`, `token_key`, `token_key_id`

### `aws_pinpoint_apns_sandbox_channel`

**Source:** `crates/winterbaume-terraform/src/converters/pinpoint.rs`

**Inject attributes** (9): `application_id`, `bundle_id`, `certificate`, `default_authentication_method`, `enabled`, `private_key`, `team_id`, `token_key`, `token_key_id`

**Extract attributes** (0): (none)

**Missing from extract** (9): `application_id`, `bundle_id`, `certificate`, `default_authentication_method`, `enabled`, `private_key`, `team_id`, `token_key`, `token_key_id`

### `aws_pinpoint_apns_voip_channel`

**Source:** `crates/winterbaume-terraform/src/converters/pinpoint.rs`

**Inject attributes** (9): `application_id`, `bundle_id`, `certificate`, `default_authentication_method`, `enabled`, `private_key`, `team_id`, `token_key`, `token_key_id`

**Extract attributes** (0): (none)

**Missing from extract** (9): `application_id`, `bundle_id`, `certificate`, `default_authentication_method`, `enabled`, `private_key`, `team_id`, `token_key`, `token_key_id`

### `aws_pinpoint_apns_voip_sandbox_channel`

**Source:** `crates/winterbaume-terraform/src/converters/pinpoint.rs`

**Inject attributes** (9): `application_id`, `bundle_id`, `certificate`, `default_authentication_method`, `enabled`, `private_key`, `team_id`, `token_key`, `token_key_id`

**Extract attributes** (0): (none)

**Missing from extract** (9): `application_id`, `bundle_id`, `certificate`, `default_authentication_method`, `enabled`, `private_key`, `team_id`, `token_key`, `token_key_id`

### `aws_pinpoint_app`

**Source:** `crates/winterbaume-terraform/src/converters/pinpoint.rs`

**Inject attributes** (10): `application_id`, `arn`, `campaign_hook`, `creation_date`, `limits`, `name`, `quiet_time`, `region`, `tags`, `tags_all`

**Extract attributes** (18): `application_id`, `arn`, `campaign_hook`, `creation_date`, `daily`, `id`, `lambda_function_name`, `limits`, `maximum_duration`, `messages_per_second`, `mode`, `name`, `quiet_time`, `session`, `tags`, `tags_all`, `total`, `web_url`

**Missing from inject** (1): `name_prefix`

**Missing from extract** (1): `name_prefix`

### `aws_pinpoint_baidu_channel`

**Source:** `crates/winterbaume-terraform/src/converters/pinpoint.rs`

**Inject attributes** (4): `api_key`, `application_id`, `enabled`, `secret_key`

**Extract attributes** (0): (none)

**Missing from extract** (4): `api_key`, `application_id`, `enabled`, `secret_key`

### `aws_pinpoint_email_channel`

**Source:** `crates/winterbaume-terraform/src/converters/pinpoint.rs`

**Inject attributes** (8): `application_id`, `configuration_set`, `enabled`, `from_address`, `identity`, `messages_per_second`, `region`, `role_arn`

**Extract attributes** (8): `application_id`, `configuration_set`, `enabled`, `from_address`, `id`, `identity`, `messages_per_second`, `role_arn`

**Missing from inject** (1): `orchestration_sending_role_arn`

**Missing from extract** (1): `orchestration_sending_role_arn`

### `aws_pinpoint_email_template`

**Source:** `crates/winterbaume-terraform/src/converters/pinpoint.rs`

**Inject attributes** (2): `arn`, `template_name`

**Extract attributes** (0): (none)

**Missing from inject** (3): `email_template`, `tags`, `tags_all`

**Missing from extract** (5): `arn`, `email_template`, `tags`, `tags_all`, `template_name`

### `aws_pinpoint_event_stream`

**Source:** `crates/winterbaume-terraform/src/converters/pinpoint.rs`

**Inject attributes** (4): `application_id`, `destination_stream_arn`, `region`, `role_arn`

**Extract attributes** (4): `application_id`, `destination_stream_arn`, `id`, `role_arn`

### `aws_pinpoint_gcm_channel`

**Source:** `crates/winterbaume-terraform/src/converters/pinpoint.rs`

**Inject attributes** (5): `api_key`, `application_id`, `default_authentication_method`, `enabled`, `service_json`

**Extract attributes** (0): (none)

**Missing from extract** (5): `api_key`, `application_id`, `default_authentication_method`, `enabled`, `service_json`

### `aws_pinpoint_sms_channel`

**Source:** `crates/winterbaume-terraform/src/converters/pinpoint.rs`

**Inject attributes** (6): `application_id`, `enabled`, `promotional_messages_per_second`, `sender_id`, `short_code`, `transactional_messages_per_second`

**Extract attributes** (0): (none)

**Missing from extract** (6): `application_id`, `enabled`, `promotional_messages_per_second`, `sender_id`, `short_code`, `transactional_messages_per_second`

### `aws_pipes_pipe`

**Source:** `crates/winterbaume-terraform/src/converters/pipes.rs`

**Inject attributes** (13): `arn`, `creation_time`, `current_state`, `description`, `desired_state`, `enrichment`, `last_modified_time`, `name`, `region`, `role_arn`, `source`, `tags`, `target`

**Extract attributes** (18): `arn`, `creation_time`, `current_state`, `description`, `desired_state`, `enrichment`, `enrichment_parameters`, `id`, `last_modified_time`, `log_configuration`, `name`, `role_arn`, `source`, `source_parameters`, `tags`, `tags_all`, `target`, `target_parameters`

**Missing from inject** (8): `enrichment_parameters`, `kms_key_identifier`, `log_configuration`, `name_prefix`, `source_parameters`, `tags_all`, `target_parameters`, `timeouts`

**Missing from extract** (3): `kms_key_identifier`, `name_prefix`, `timeouts`

### `aws_account_id`

**Source:** `crates/winterbaume-terraform/src/converters/quicksight.rs`

**Inject attributes** (0): (none)

**Extract attributes** (4): `default_namespace`, `id`, `notification_email`, `termination_protection_enabled`

**Note:** Resource type not found in Terraform AWS provider schema.

### `aws_quicksight_account_settings`

**Source:** `crates/winterbaume-terraform/src/converters/quicksight.rs`

**Inject attributes** (5): `aws_account_id`, `default_namespace`, `notification_email`, `region`, `termination_protection_enabled`

**Extract attributes** (5): `aws_account_id`, `default_namespace`, `id`, `notification_email`, `termination_protection_enabled`

**Missing from inject** (1): `timeouts`

**Missing from extract** (1): `timeouts`

### `aws_quicksight_account_subscription`

**Source:** `crates/winterbaume-terraform/src/converters/quicksight.rs`

**Inject attributes** (0): (none)

**Extract attributes** (0): (none)

**Missing from inject** (18): `account_name`, `account_subscription_status`, `active_directory_name`, `admin_group`, `authentication_method`, `author_group`, `aws_account_id`, `contact_number`, `directory_id`, `edition`, `email_address`, `first_name`, `iam_identity_center_instance_arn`, `last_name`, `notification_email`, `reader_group`, `realm`, `timeouts`

**Missing from extract** (18): `account_name`, `account_subscription_status`, `active_directory_name`, `admin_group`, `authentication_method`, `author_group`, `aws_account_id`, `contact_number`, `directory_id`, `edition`, `email_address`, `first_name`, `iam_identity_center_instance_arn`, `last_name`, `notification_email`, `reader_group`, `realm`, `timeouts`

### `aws_quicksight_analysis`

**Source:** `crates/winterbaume-terraform/src/converters/quicksight.rs`

**Inject attributes** (9): `analysis_id`, `arn`, `aws_account_id`, `created_time`, `last_updated_time`, `name`, `region`, `status`, `theme_arn`

**Extract attributes** (8): `analysis_id`, `arn`, `aws_account_id`, `created_time`, `id`, `last_updated_time`, `name`, `status`

**Missing from inject** (9): `definition`, `last_published_time`, `parameters`, `permissions`, `recovery_window_in_days`, `source_entity`, `tags`, `tags_all`, `timeouts`

**Missing from extract** (10): `definition`, `last_published_time`, `parameters`, `permissions`, `recovery_window_in_days`, `source_entity`, `tags`, `tags_all`, `theme_arn`, `timeouts`

### `aws_quicksight_dashboard`

**Source:** `crates/winterbaume-terraform/src/converters/quicksight.rs`

**Inject attributes** (10): `arn`, `aws_account_id`, `created_time`, `dashboard_id`, `last_published_time`, `last_updated_time`, `name`, `region`, `version_arn`, `version_number`

**Extract attributes** (8): `arn`, `aws_account_id`, `created_time`, `dashboard_id`, `id`, `last_updated_time`, `name`, `version_arn`

**Missing from inject** (12): `dashboard_publish_options`, `definition`, `parameters`, `permissions`, `source_entity`, `source_entity_arn`, `status`, `tags`, `tags_all`, `theme_arn`, `timeouts`, `version_description`

**Missing from extract** (14): `dashboard_publish_options`, `definition`, `last_published_time`, `parameters`, `permissions`, `source_entity`, `source_entity_arn`, `status`, `tags`, `tags_all`, `theme_arn`, `timeouts`, `version_description`, `version_number`

### `aws_quicksight_data_set`

**Source:** `crates/winterbaume-terraform/src/converters/quicksight.rs`

**Inject attributes** (8): `arn`, `aws_account_id`, `created_time`, `data_set_id`, `import_mode`, `last_updated_time`, `name`, `region`

**Extract attributes** (9): `arn`, `aws_account_id`, `created_time`, `data_set_id`, `id`, `import_mode`, `last_updated_time`, `name`, `physical_table_map`

**Missing from inject** (13): `column_groups`, `column_level_permission_rules`, `data_set_usage_configuration`, `field_folders`, `logical_table_map`, `output_columns`, `permissions`, `physical_table_map`, `refresh_properties`, `row_level_permission_data_set`, `row_level_permission_tag_configuration`, `tags`, `tags_all`

**Missing from extract** (12): `column_groups`, `column_level_permission_rules`, `data_set_usage_configuration`, `field_folders`, `logical_table_map`, `output_columns`, `permissions`, `refresh_properties`, `row_level_permission_data_set`, `row_level_permission_tag_configuration`, `tags`, `tags_all`

### `aws_quicksight_data_source`

**Source:** `crates/winterbaume-terraform/src/converters/quicksight.rs`

**Inject attributes** (12): `arn`, `created_time`, `credentials`, `data_source_id`, `last_updated_time`, `name`, `parameters`, `permission`, `region`, `status`, `type`, `vpc_connection_properties`

**Extract attributes** (14): `arn`, `created_time`, `credentials`, `data_source_id`, `id`, `last_updated_time`, `name`, `parameters`, `permission`, `ssl_properties`, `status`, `tags_all`, `type`, `vpc_connection_properties`

**Missing from inject** (4): `aws_account_id`, `ssl_properties`, `tags`, `tags_all`

**Missing from extract** (2): `aws_account_id`, `tags`

### `aws_quicksight_folder`

**Source:** `crates/winterbaume-terraform/src/converters/quicksight.rs`

**Inject attributes** (9): `arn`, `aws_account_id`, `created_time`, `folder_id`, `folder_type`, `last_updated_time`, `name`, `parent_folder_arn`, `region`

**Extract attributes** (8): `arn`, `aws_account_id`, `created_time`, `folder_id`, `folder_type`, `id`, `last_updated_time`, `name`

**Missing from inject** (5): `folder_path`, `permissions`, `tags`, `tags_all`, `timeouts`

**Missing from extract** (6): `folder_path`, `parent_folder_arn`, `permissions`, `tags`, `tags_all`, `timeouts`

### `aws_quicksight_folder_membership`

**Source:** `crates/winterbaume-terraform/src/converters/quicksight.rs`

**Inject attributes** (5): `aws_account_id`, `folder_id`, `member_id`, `member_type`, `region`

**Extract attributes** (5): `aws_account_id`, `folder_id`, `id`, `member_id`, `member_type`

### `aws_quicksight_group`

**Source:** `crates/winterbaume-terraform/src/converters/quicksight.rs`

**Inject attributes** (6): `arn`, `description`, `group_name`, `namespace`, `principal_id`, `region`

**Extract attributes** (6): `arn`, `description`, `group_name`, `id`, `namespace`, `principal_id`

**Missing from inject** (1): `aws_account_id`

**Missing from extract** (1): `aws_account_id`

### `aws_quicksight_group_membership`

**Source:** `crates/winterbaume-terraform/src/converters/quicksight.rs`

**Inject attributes** (6): `arn`, `aws_account_id`, `group_name`, `member_name`, `namespace`, `region`

**Extract attributes** (6): `arn`, `aws_account_id`, `group_name`, `id`, `member_name`, `namespace`

### `aws_quicksight_iam_policy_assignment`

**Source:** `crates/winterbaume-terraform/src/converters/quicksight.rs`

**Inject attributes** (0): (none)

**Extract attributes** (0): (none)

**Missing from inject** (7): `assignment_id`, `assignment_name`, `assignment_status`, `aws_account_id`, `identities`, `namespace`, `policy_arn`

**Missing from extract** (7): `assignment_id`, `assignment_name`, `assignment_status`, `aws_account_id`, `identities`, `namespace`, `policy_arn`

### `aws_quicksight_ingestion`

**Source:** `crates/winterbaume-terraform/src/converters/quicksight.rs`

**Inject attributes** (7): `arn`, `aws_account_id`, `data_set_id`, `ingestion_id`, `ingestion_status`, `ingestion_type`, `region`

**Extract attributes** (6): `arn`, `aws_account_id`, `data_set_id`, `id`, `ingestion_id`, `ingestion_status`

**Missing from extract** (1): `ingestion_type`

### `aws_quicksight_namespace`

**Source:** `crates/winterbaume-terraform/src/converters/quicksight.rs`

**Inject attributes** (7): `arn`, `aws_account_id`, `capacity_region`, `creation_status`, `identity_store`, `namespace`, `region`

**Extract attributes** (7): `arn`, `aws_account_id`, `capacity_region`, `creation_status`, `id`, `identity_store`, `namespace`

**Missing from inject** (3): `tags`, `tags_all`, `timeouts`

**Missing from extract** (3): `tags`, `tags_all`, `timeouts`

### `aws_quicksight_refresh_schedule`

**Source:** `crates/winterbaume-terraform/src/converters/quicksight.rs`

**Inject attributes** (0): (none)

**Extract attributes** (0): (none)

**Missing from inject** (5): `arn`, `aws_account_id`, `data_set_id`, `schedule`, `schedule_id`

**Missing from extract** (5): `arn`, `aws_account_id`, `data_set_id`, `schedule`, `schedule_id`

### `aws_quicksight_role_membership`

**Source:** `crates/winterbaume-terraform/src/converters/quicksight.rs`

**Inject attributes** (0): (none)

**Extract attributes** (0): (none)

**Missing from inject** (4): `aws_account_id`, `member_name`, `namespace`, `role`

**Missing from extract** (4): `aws_account_id`, `member_name`, `namespace`, `role`

### `aws_quicksight_template`

**Source:** `crates/winterbaume-terraform/src/converters/quicksight.rs`

**Inject attributes** (9): `arn`, `aws_account_id`, `created_time`, `last_updated_time`, `name`, `region`, `template_id`, `version_arn`, `version_number`

**Extract attributes** (9): `arn`, `aws_account_id`, `created_time`, `id`, `last_updated_time`, `name`, `template_id`, `version_arn`, `version_number`

**Missing from inject** (9): `definition`, `permissions`, `source_entity`, `source_entity_arn`, `status`, `tags`, `tags_all`, `timeouts`, `version_description`

**Missing from extract** (9): `definition`, `permissions`, `source_entity`, `source_entity_arn`, `status`, `tags`, `tags_all`, `timeouts`, `version_description`

### `aws_quicksight_template_alias`

**Source:** `crates/winterbaume-terraform/src/converters/quicksight.rs`

**Inject attributes** (0): (none)

**Extract attributes** (0): (none)

**Missing from inject** (5): `alias_name`, `arn`, `aws_account_id`, `template_id`, `template_version_number`

**Missing from extract** (5): `alias_name`, `arn`, `aws_account_id`, `template_id`, `template_version_number`

### `aws_quicksight_theme`

**Source:** `crates/winterbaume-terraform/src/converters/quicksight.rs`

**Inject attributes** (10): `arn`, `aws_account_id`, `base_theme_id`, `created_time`, `last_updated_time`, `name`, `region`, `theme_id`, `version_arn`, `version_number`

**Extract attributes** (9): `arn`, `aws_account_id`, `created_time`, `id`, `last_updated_time`, `name`, `theme_id`, `version_arn`, `version_number`

**Missing from inject** (7): `configuration`, `permissions`, `status`, `tags`, `tags_all`, `timeouts`, `version_description`

**Missing from extract** (8): `base_theme_id`, `configuration`, `permissions`, `status`, `tags`, `tags_all`, `timeouts`, `version_description`

### `aws_quicksight_user`

**Source:** `crates/winterbaume-terraform/src/converters/quicksight.rs`

**Inject attributes** (8): `arn`, `email`, `identity_type`, `namespace`, `principal_id`, `region`, `user_name`, `user_role`

**Extract attributes** (9): `active`, `arn`, `email`, `id`, `identity_type`, `namespace`, `principal_id`, `user_name`, `user_role`

**Missing from inject** (4): `aws_account_id`, `iam_arn`, `session_name`, `user_invitation_url`

**Missing from extract** (4): `aws_account_id`, `iam_arn`, `session_name`, `user_invitation_url`

### `aws_quicksight_vpc_connection`

**Source:** `crates/winterbaume-terraform/src/converters/quicksight.rs`

**Inject attributes** (0): (none)

**Extract attributes** (0): (none)

**Missing from inject** (12): `arn`, `availability_status`, `aws_account_id`, `dns_resolvers`, `name`, `role_arn`, `security_group_ids`, `subnet_ids`, `tags`, `tags_all`, `timeouts`, `vpc_connection_id`

**Missing from extract** (12): `arn`, `availability_status`, `aws_account_id`, `dns_resolvers`, `name`, `role_arn`, `security_group_ids`, `subnet_ids`, `tags`, `tags_all`, `timeouts`, `vpc_connection_id`

### `aws_ram_principal_association`

**Source:** `crates/winterbaume-terraform/src/converters/ram.rs`

**Inject attributes** (3): `principal`, `region`, `resource_share_arn`

**Extract attributes** (3): `id`, `principal`, `resource_share_arn`

### `aws_ram_resource_association`

**Source:** `crates/winterbaume-terraform/src/converters/ram.rs`

**Inject attributes** (3): `region`, `resource_arn`, `resource_share_arn`

**Extract attributes** (3): `id`, `resource_arn`, `resource_share_arn`

### `aws_ram_resource_share`

**Source:** `crates/winterbaume-terraform/src/converters/ram.rs`

**Inject attributes** (5): `allow_external_principals`, `arn`, `name`, `region`, `tags`

**Extract attributes** (9): `allow_external_principals`, `arn`, `creation_time`, `id`, `last_updated_time`, `name`, `owning_account_id`, `status`, `tags`

**Missing from inject** (3): `permission_arns`, `tags_all`, `timeouts`

**Missing from extract** (3): `permission_arns`, `tags_all`, `timeouts`

### `aws_ram_resource_share_accepter`

**Source:** `crates/winterbaume-terraform/src/converters/ram.rs`

**Inject attributes** (6): `invitation_arn`, `receiver_account_id`, `region`, `sender_account_id`, `share_arn`, `status`

**Extract attributes** (6): `id`, `invitation_arn`, `receiver_account_id`, `sender_account_id`, `share_arn`, `status`

**Missing from inject** (4): `resources`, `share_id`, `share_name`, `timeouts`

**Missing from extract** (4): `resources`, `share_id`, `share_name`, `timeouts`

### `aws_ram_sharing_with_organization`

**Source:** `crates/winterbaume-terraform/src/converters/ram.rs`

**Inject attributes** (1): `region`

**Extract attributes** (1): `id`

**Note:** Resource type not found in Terraform AWS provider schema.

### `aws_db_cluster_snapshot`

**Source:** `crates/winterbaume-terraform/src/converters/rds.rs`

**Inject attributes** (14): `allocated_storage`, `db_cluster_identifier`, `db_cluster_snapshot_arn`, `db_cluster_snapshot_identifier`, `engine`, `engine_version`, `kms_key_id`, `region`, `shared_accounts`, `snapshot_type`, `status`, `storage_encrypted`, `tags_all`, `vpc_id`

**Extract attributes** (18): `allocated_storage`, `availability_zones`, `db_cluster_identifier`, `db_cluster_snapshot_arn`, `db_cluster_snapshot_identifier`, `engine`, `engine_version`, `id`, `kms_key_id`, `port`, `snapshot_type`, `source_db_cluster_snapshot_arn`, `status`, `storage_encrypted`, `storage_type`, `tags`, `tags_all`, `vpc_id`

**Missing from inject** (6): `availability_zones`, `license_model`, `port`, `source_db_cluster_snapshot_arn`, `tags`, `timeouts`

**Missing from extract** (3): `license_model`, `shared_accounts`, `timeouts`

### `aws_db_event_subscription`

**Source:** `crates/winterbaume-terraform/src/converters/rds.rs`

**Inject attributes** (9): `arn`, `enabled`, `name`, `name_prefix`, `region`, `sns_topic`, `source_type`, `tags_all`, `timeouts`

**Extract attributes** (8): `arn`, `enabled`, `event_categories`, `id`, `name`, `sns_topic`, `source_ids`, `source_type`

**Missing from inject** (4): `customer_aws_id`, `event_categories`, `source_ids`, `tags`

**Missing from extract** (5): `customer_aws_id`, `name_prefix`, `tags`, `tags_all`, `timeouts`

### `aws_db_instance`

**Source:** `crates/winterbaume-terraform/src/converters/rds.rs`

**Inject attributes** (45): `allow_major_version_upgrade`, `apply_immediately`, `arn`, `auto_minor_version_upgrade`, `availability_zone`, `backup_window`, `blue_green_update`, `character_set_name`, `cluster_identifier`, `custom_iam_instance_profile`, `dedicated_log_volume`, `domain`, `domain_iam_role_name`, `enabled_cloudwatch_logs_exports`, `endpoint`, `engine`, `engine_version`, `final_snapshot_identifier`, `identifier`, `instance_class`, `iops`, `kms_key_arn`, `maintenance_window`, `manage_master_user_password`, `master_user_secret_kms_key_id`, `max_allocated_storage`, `monitoring_interval`, `nchar_character_set_name`, `neptune_parameter_group_name`, `neptune_subnet_group_name`, `network_type`, `option_group_name`, `parameter_group_name`, `performance_insights_kms_key_id`, `performance_insights_retention_period`, `port`, `publicly_accessible`, `region`, `replica_mode`, `restore_to_point_in_time`, `s3_import`, `skip_final_snapshot`, `storage_encrypted`, `tags_all`, `timezone`

**Extract attributes** (48): `address`, `allocated_storage`, `arn`, `availability_zone`, `backup_retention_period`, `blue_green_update`, `ca_cert_identifier`, `character_set_name`, `copy_tags_to_snapshot`, `db_name`, `db_subnet_group_name`, `deletion_protection`, `endpoint`, `engine`, `engine_version`, `hosted_zone_id`, `id`, `identifier`, `instance_class`, `instance_create_time`, `iops`, `kms_key_id`, `latest_restorable_time`, `license_model`, `maintenance_window`, `master_user_secret`, `monitoring_interval`, `multi_az`, `network_type`, `option_group_name`, `parameter_group_name`, `performance_insights_enabled`, `performance_insights_kms_key_id`, `performance_insights_retention_period`, `port`, `publicly_accessible`, `replica_mode`, `replicas`, `resource_id`, `restore_to_point_in_time`, `s3_import`, `status`, `storage_encrypted`, `storage_type`, `tags`, `tags_all`, `username`, `vpc_security_group_ids`

**Missing from inject** (44): `address`, `allocated_storage`, `backup_retention_period`, `backup_target`, `ca_cert_identifier`, `copy_tags_to_snapshot`, `customer_owned_ip_enabled`, `database_insights_mode`, `db_name`, `db_subnet_group_name`, `delete_automated_backups`, `deletion_protection`, `domain_auth_secret_arn`, `domain_dns_ips`, `domain_fqdn`, `domain_ou`, `engine_lifecycle_support`, `engine_version_actual`, `hosted_zone_id`, `iam_database_authentication_enabled`, `identifier_prefix`, `kms_key_id`, `latest_restorable_time`, `license_model`, `listener_endpoint`, `master_user_secret`, `monitoring_role_arn`, `multi_az`, `password`, `password_wo`, `password_wo_version`, `performance_insights_enabled`, `replicas`, `replicate_source_db`, `resource_id`, `snapshot_identifier`, `status`, `storage_throughput`, `storage_type`, `tags`, `timeouts`, `upgrade_storage_config`, `username`, `vpc_security_group_ids`

**Missing from extract** (38): `allow_major_version_upgrade`, `apply_immediately`, `auto_minor_version_upgrade`, `backup_target`, `backup_window`, `custom_iam_instance_profile`, `customer_owned_ip_enabled`, `database_insights_mode`, `dedicated_log_volume`, `delete_automated_backups`, `domain`, `domain_auth_secret_arn`, `domain_dns_ips`, `domain_fqdn`, `domain_iam_role_name`, `domain_ou`, `enabled_cloudwatch_logs_exports`, `engine_lifecycle_support`, `engine_version_actual`, `final_snapshot_identifier`, `iam_database_authentication_enabled`, `identifier_prefix`, `listener_endpoint`, `manage_master_user_password`, `master_user_secret_kms_key_id`, `max_allocated_storage`, `monitoring_role_arn`, `nchar_character_set_name`, `password`, `password_wo`, `password_wo_version`, `replicate_source_db`, `skip_final_snapshot`, `snapshot_identifier`, `storage_throughput`, `timeouts`, `timezone`, `upgrade_storage_config`

### `aws_db_instance_automated_backups_replication`

**Source:** `crates/winterbaume-terraform/src/converters/rds.rs`

**Inject attributes** (4): `kms_key_id`, `pre_signed_url`, `retention_period`, `source_db_instance_arn`

**Extract attributes** (0): (none)

**Missing from inject** (1): `timeouts`

**Missing from extract** (5): `kms_key_id`, `pre_signed_url`, `retention_period`, `source_db_instance_arn`, `timeouts`

### `aws_db_instance_role_association`

**Source:** `crates/winterbaume-terraform/src/converters/rds.rs`

**Inject attributes** (3): `db_instance_identifier`, `feature_name`, `role_arn`

**Extract attributes** (0): (none)

**Missing from inject** (1): `timeouts`

**Missing from extract** (4): `db_instance_identifier`, `feature_name`, `role_arn`, `timeouts`

### `aws_db_option_group`

**Source:** `crates/winterbaume-terraform/src/converters/rds.rs`

**Inject attributes** (9): `arn`, `engine_name`, `major_engine_version`, `name`, `name_prefix`, `option_group_description`, `region`, `tags_all`, `vpc_id`

**Extract attributes** (9): `arn`, `engine_name`, `id`, `major_engine_version`, `name`, `option`, `option_group_description`, `tags`, `tags_all`

**Missing from inject** (4): `option`, `skip_destroy`, `tags`, `timeouts`

**Missing from extract** (3): `name_prefix`, `skip_destroy`, `timeouts`

### `aws_db_parameter_group`

**Source:** `crates/winterbaume-terraform/src/converters/rds.rs`

**Inject attributes** (7): `arn`, `description`, `family`, `name`, `name_prefix`, `region`, `tags_all`

**Extract attributes** (7): `arn`, `description`, `family`, `id`, `name`, `parameter`, `tags`

**Missing from inject** (3): `parameter`, `skip_destroy`, `tags`

**Missing from extract** (3): `name_prefix`, `skip_destroy`, `tags_all`

### `aws_db_proxy`

**Source:** `crates/winterbaume-terraform/src/converters/rds.rs`

**Inject attributes** (15): `arn`, `auth`, `debug_logging`, `default_auth_scheme`, `endpoint`, `endpoint_network_type`, `engine_family`, `idle_client_timeout`, `name`, `region`, `require_tls`, `role_arn`, `tags_all`, `target_connection_network_type`, `vpc_id`

**Extract attributes** (14): `arn`, `debug_logging`, `endpoint`, `engine_family`, `id`, `idle_client_timeout`, `name`, `require_tls`, `role_arn`, `tags`, `tags_all`, `vpc_id`, `vpc_security_group_ids`, `vpc_subnet_ids`

**Missing from inject** (4): `tags`, `timeouts`, `vpc_security_group_ids`, `vpc_subnet_ids`

**Missing from extract** (2): `auth`, `timeouts`

### `aws_db_proxy_default_target_group`

**Source:** `crates/winterbaume-terraform/src/converters/rds.rs`

**Inject attributes** (5): `arn`, `connection_pool_config`, `db_proxy_name`, `name`, `region`

**Extract attributes** (5): `arn`, `connection_pool_config`, `db_proxy_name`, `id`, `name`

**Missing from inject** (1): `timeouts`

**Missing from extract** (1): `timeouts`

### `aws_db_proxy_endpoint`

**Source:** `crates/winterbaume-terraform/src/converters/rds.rs`

**Inject attributes** (9): `arn`, `db_proxy_endpoint_name`, `db_proxy_name`, `endpoint`, `is_default`, `region`, `tags_all`, `target_role`, `vpc_id`

**Extract attributes** (10): `arn`, `db_proxy_endpoint_name`, `db_proxy_name`, `endpoint`, `id`, `is_default`, `target_role`, `vpc_id`, `vpc_security_group_ids`, `vpc_subnet_ids`

**Missing from inject** (4): `tags`, `timeouts`, `vpc_security_group_ids`, `vpc_subnet_ids`

**Missing from extract** (3): `tags`, `tags_all`, `timeouts`

### `aws_db_proxy_target`

**Source:** `crates/winterbaume-terraform/src/converters/rds.rs`

**Inject attributes** (11): `db_cluster_identifier`, `db_instance_identifier`, `db_proxy_name`, `endpoint`, `port`, `rds_resource_id`, `region`, `target_arn`, `target_group_name`, `tracked_cluster_id`, `type`

**Extract attributes** (9): `db_proxy_name`, `endpoint`, `id`, `port`, `rds_resource_id`, `target_arn`, `target_group_name`, `tracked_cluster_id`, `type`

**Missing from extract** (2): `db_cluster_identifier`, `db_instance_identifier`

### `aws_db_snapshot`

**Source:** `crates/winterbaume-terraform/src/converters/rds.rs`

**Inject attributes** (22): `allocated_storage`, `availability_zone`, `db_instance_identifier`, `db_snapshot_arn`, `db_snapshot_identifier`, `encrypted`, `engine`, `engine_version`, `iops`, `kms_key_id`, `license_model`, `option_group_name`, `port`, `region`, `shared_accounts`, `snapshot_type`, `source_db_snapshot_identifier`, `source_region`, `status`, `storage_type`, `tags_all`, `vpc_id`

**Extract attributes** (21): `allocated_storage`, `availability_zone`, `db_instance_identifier`, `db_snapshot_arn`, `db_snapshot_identifier`, `encrypted`, `engine`, `engine_version`, `id`, `iops`, `kms_key_id`, `option_group_name`, `port`, `snapshot_type`, `source_db_snapshot_identifier`, `source_region`, `status`, `storage_type`, `tags`, `tags_all`, `vpc_id`

**Missing from inject** (2): `tags`, `timeouts`

**Missing from extract** (3): `license_model`, `shared_accounts`, `timeouts`

### `aws_db_snapshot_copy`

**Source:** `crates/winterbaume-terraform/src/converters/rds.rs`

**Inject attributes** (23): `allocated_storage`, `copy_tags`, `db_snapshot_arn`, `destination_region`, `encrypted`, `engine`, `engine_version`, `iops`, `kms_key_id`, `license_model`, `option_group_name`, `port`, `presigned_url`, `region`, `shared_accounts`, `snapshot_type`, `source_db_snapshot_identifier`, `source_region`, `storage_type`, `tags_all`, `target_custom_availability_zone`, `target_db_snapshot_identifier`, `vpc_id`

**Extract attributes** (0): (none)

**Missing from inject** (3): `availability_zone`, `tags`, `timeouts`

**Missing from extract** (25): `allocated_storage`, `availability_zone`, `copy_tags`, `db_snapshot_arn`, `destination_region`, `encrypted`, `engine`, `engine_version`, `iops`, `kms_key_id`, `license_model`, `option_group_name`, `port`, `presigned_url`, `shared_accounts`, `snapshot_type`, `source_db_snapshot_identifier`, `source_region`, `storage_type`, `tags`, `tags_all`, `target_custom_availability_zone`, `target_db_snapshot_identifier`, `timeouts`, `vpc_id`

### `aws_db_subnet_group`

**Source:** `crates/winterbaume-terraform/src/converters/rds.rs`

**Inject attributes** (7): `arn`, `description`, `name`, `name_prefix`, `region`, `tags_all`, `vpc_id`

**Extract attributes** (7): `arn`, `description`, `id`, `name`, `subnet_ids`, `tags`, `vpc_id`

**Missing from inject** (3): `subnet_ids`, `supported_network_types`, `tags`

**Missing from extract** (3): `name_prefix`, `supported_network_types`, `tags_all`

### `aws_rds_certificate`

**Source:** `crates/winterbaume-terraform/src/converters/rds.rs`

**Inject attributes** (1): `certificate_identifier`

**Extract attributes** (0): (none)

**Missing from extract** (1): `certificate_identifier`

### `aws_rds_cluster`

**Source:** `crates/winterbaume-terraform/src/converters/rds.rs`

**Inject attributes** (52): `allocated_storage`, `allow_major_version_upgrade`, `apply_immediately`, `arn`, `backtrack_window`, `backup_retention_period`, `ca_certificate_identifier`, `cluster_identifier`, `cluster_identifier_prefix`, `cluster_members`, `copy_tags_to_snapshot`, `database_name`, `db_cluster_instance_class`, `db_cluster_parameter_group_name`, `db_subnet_group_name`, `db_system_id`, `delete_automated_backups`, `deletion_protection`, `enable_global_write_forwarding`, `enable_http_endpoint`, `enable_local_write_forwarding`, `enabled_cloudwatch_logs_exports`, `endpoint`, `engine`, `engine_mode`, `engine_version`, `final_snapshot_identifier`, `global_cluster_identifier`, `iam_database_authentication_enabled`, `iam_roles`, `kms_key_id`, `manage_master_user_password`, `master_user_secret_kms_key_id`, `master_username`, `multi_az`, `network_type`, `performance_insights_enabled`, `performance_insights_kms_key_id`, `performance_insights_retention_period`, `port`, `preferred_backup_window`, `preferred_maintenance_window`, `reader_endpoint`, `region`, `restore_to_point_in_time`, `s3_import`, `scaling_configuration`, `serverlessv2_scaling_configuration`, `skip_final_snapshot`, `storage_encrypted`, `storage_type`, `tags_all`

**Extract attributes** (49): `allocated_storage`, `arn`, `availability_zones`, `backup_retention_period`, `ca_certificate_identifier`, `cluster_create_time`, `cluster_identifier`, `cluster_members`, `cluster_resource_id`, `copy_tags_to_snapshot`, `database_name`, `db_cluster_instance_class`, `db_cluster_parameter_group_name`, `db_cluster_parameter_group_status`, `db_instance_identifier`, `db_subnet_group_name`, `deletion_protection`, `enabled_cloudwatch_logs_exports`, `endpoint`, `engine`, `engine_mode`, `engine_version`, `engine_version_actual`, `hosted_zone_id`, `iam_database_authentication_enabled`, `iam_roles`, `id`, `is_cluster_writer`, `kms_key_id`, `master_user_secret`, `master_username`, `multi_az`, `network_type`, `performance_insights_enabled`, `port`, `preferred_backup_window`, `preferred_maintenance_window`, `promotion_tier`, `reader_endpoint`, `restore_to_point_in_time`, `s3_import`, `scaling_configuration`, `serverlessv2_scaling_configuration`, `status`, `storage_encrypted`, `storage_type`, `tags`, `tags_all`, `vpc_security_group_ids`

**Missing from inject** (24): `availability_zones`, `ca_certificate_valid_till`, `cluster_resource_id`, `cluster_scalability_type`, `database_insights_mode`, `db_instance_parameter_group_name`, `domain`, `domain_iam_role_name`, `engine_lifecycle_support`, `engine_version_actual`, `hosted_zone_id`, `iops`, `master_password`, `master_password_wo`, `master_password_wo_version`, `master_user_secret`, `monitoring_interval`, `monitoring_role_arn`, `replication_source_identifier`, `snapshot_identifier`, `source_region`, `tags`, `timeouts`, `vpc_security_group_ids`

**Missing from extract** (33): `allow_major_version_upgrade`, `apply_immediately`, `backtrack_window`, `ca_certificate_valid_till`, `cluster_identifier_prefix`, `cluster_scalability_type`, `database_insights_mode`, `db_instance_parameter_group_name`, `db_system_id`, `delete_automated_backups`, `domain`, `domain_iam_role_name`, `enable_global_write_forwarding`, `enable_http_endpoint`, `enable_local_write_forwarding`, `engine_lifecycle_support`, `final_snapshot_identifier`, `global_cluster_identifier`, `iops`, `manage_master_user_password`, `master_password`, `master_password_wo`, `master_password_wo_version`, `master_user_secret_kms_key_id`, `monitoring_interval`, `monitoring_role_arn`, `performance_insights_kms_key_id`, `performance_insights_retention_period`, `replication_source_identifier`, `skip_final_snapshot`, `snapshot_identifier`, `source_region`, `timeouts`

### `aws_rds_cluster_activity_stream`

**Source:** `crates/winterbaume-terraform/src/converters/rds.rs`

**Inject attributes** (5): `engine_native_audit_fields_included`, `kinesis_stream_name`, `kms_key_id`, `mode`, `resource_arn`

**Extract attributes** (0): (none)

**Missing from extract** (5): `engine_native_audit_fields_included`, `kinesis_stream_name`, `kms_key_id`, `mode`, `resource_arn`

### `aws_rds_cluster_endpoint`

**Source:** `crates/winterbaume-terraform/src/converters/rds.rs`

**Inject attributes** (7): `arn`, `cluster_endpoint_identifier`, `cluster_identifier`, `custom_endpoint_type`, `endpoint`, `region`, `tags_all`

**Extract attributes** (8): `arn`, `cluster_endpoint_identifier`, `cluster_identifier`, `custom_endpoint_type`, `endpoint`, `excluded_members`, `id`, `static_members`

**Missing from inject** (3): `excluded_members`, `static_members`, `tags`

**Missing from extract** (2): `tags`, `tags_all`

### `aws_rds_cluster_instance`

**Source:** `crates/winterbaume-terraform/src/converters/rds.rs`

**Inject attributes** (26): `apply_immediately`, `arn`, `auto_minor_version_upgrade`, `availability_zone`, `ca_cert_identifier`, `cluster_identifier`, `copy_tags_to_snapshot`, `db_parameter_group_name`, `db_subnet_group_name`, `engine`, `engine_version`, `identifier`, `identifier_prefix`, `instance_class`, `kms_key_id`, `monitoring_interval`, `monitoring_role_arn`, `performance_insights_enabled`, `performance_insights_kms_key_id`, `port`, `preferred_backup_window`, `preferred_maintenance_window`, `promotion_tier`, `publicly_accessible`, `region`, `tags_all`

**Extract attributes** (24): `arn`, `auto_minor_version_upgrade`, `availability_zone`, `ca_cert_identifier`, `cluster_identifier`, `copy_tags_to_snapshot`, `db_parameter_group_name`, `db_subnet_group_name`, `dbi_resource_id`, `endpoint`, `engine`, `engine_version`, `id`, `identifier`, `instance_class`, `kms_key_id`, `monitoring_interval`, `performance_insights_enabled`, `port`, `publicly_accessible`, `storage_encrypted`, `tags`, `tags_all`, `writer`

**Missing from inject** (11): `custom_iam_instance_profile`, `dbi_resource_id`, `endpoint`, `engine_version_actual`, `force_destroy`, `network_type`, `performance_insights_retention_period`, `storage_encrypted`, `tags`, `timeouts`, `writer`

**Missing from extract** (13): `apply_immediately`, `custom_iam_instance_profile`, `engine_version_actual`, `force_destroy`, `identifier_prefix`, `monitoring_role_arn`, `network_type`, `performance_insights_kms_key_id`, `performance_insights_retention_period`, `preferred_backup_window`, `preferred_maintenance_window`, `promotion_tier`, `timeouts`

### `aws_rds_cluster_parameter_group`

**Source:** `crates/winterbaume-terraform/src/converters/rds.rs`

**Inject attributes** (6): `arn`, `description`, `family`, `name`, `name_prefix`, `region`

**Extract attributes** (7): `arn`, `description`, `family`, `id`, `name`, `parameter`, `tags`

**Missing from inject** (3): `parameter`, `tags`, `tags_all`

**Missing from extract** (2): `name_prefix`, `tags_all`

### `aws_rds_cluster_role_association`

**Source:** `crates/winterbaume-terraform/src/converters/rds.rs`

**Inject attributes** (3): `db_cluster_identifier`, `feature_name`, `role_arn`

**Extract attributes** (0): (none)

**Missing from inject** (1): `timeouts`

**Missing from extract** (4): `db_cluster_identifier`, `feature_name`, `role_arn`, `timeouts`

### `aws_rds_cluster_snapshot_copy`

**Source:** `crates/winterbaume-terraform/src/converters/rds.rs`

**Inject attributes** (18): `allocated_storage`, `copy_tags`, `db_cluster_snapshot_arn`, `destination_region`, `engine`, `engine_version`, `kms_key_id`, `license_model`, `presigned_url`, `region`, `shared_accounts`, `snapshot_type`, `source_db_cluster_snapshot_identifier`, `storage_encrypted`, `storage_type`, `tags_all`, `target_db_cluster_snapshot_identifier`, `vpc_id`

**Extract attributes** (0): (none)

**Missing from inject** (2): `tags`, `timeouts`

**Missing from extract** (19): `allocated_storage`, `copy_tags`, `db_cluster_snapshot_arn`, `destination_region`, `engine`, `engine_version`, `kms_key_id`, `license_model`, `presigned_url`, `shared_accounts`, `snapshot_type`, `source_db_cluster_snapshot_identifier`, `storage_encrypted`, `storage_type`, `tags`, `tags_all`, `target_db_cluster_snapshot_identifier`, `timeouts`, `vpc_id`

### `aws_rds_custom_db_engine_version`

**Source:** `crates/winterbaume-terraform/src/converters/rds.rs`

**Inject attributes** (9): `database_installation_files_s3_bucket_name`, `database_installation_files_s3_prefix`, `description`, `engine`, `engine_version`, `kms_key_id`, `manifest`, `source_image_id`, `status`

**Extract attributes** (0): (none)

**Missing from inject** (11): `arn`, `create_time`, `db_parameter_group_family`, `filename`, `image_id`, `major_engine_version`, `manifest_computed`, `manifest_hash`, `tags`, `tags_all`, `timeouts`

**Missing from extract** (20): `arn`, `create_time`, `database_installation_files_s3_bucket_name`, `database_installation_files_s3_prefix`, `db_parameter_group_family`, `description`, `engine`, `engine_version`, `filename`, `image_id`, `kms_key_id`, `major_engine_version`, `manifest`, `manifest_computed`, `manifest_hash`, `source_image_id`, `status`, `tags`, `tags_all`, `timeouts`

### `aws_rds_export_task`

**Source:** `crates/winterbaume-terraform/src/converters/rds.rs`

**Inject attributes** (15): `export_task_identifier`, `failure_cause`, `iam_role_arn`, `kms_key_id`, `percent_progress`, `region`, `s3_bucket_name`, `s3_prefix`, `snapshot_time`, `source_arn`, `source_type`, `status`, `task_end_time`, `task_start_time`, `warning_message`

**Extract attributes** (16): `export_only`, `export_task_identifier`, `failure_cause`, `iam_role_arn`, `id`, `kms_key_id`, `percent_progress`, `s3_bucket_name`, `s3_prefix`, `snapshot_time`, `source_arn`, `source_type`, `status`, `task_end_time`, `task_start_time`, `warning_message`

**Missing from inject** (2): `export_only`, `timeouts`

**Missing from extract** (1): `timeouts`

### `aws_rds_global_cluster`

**Source:** `crates/winterbaume-terraform/src/converters/rds.rs`

**Inject attributes** (13): `arn`, `database_name`, `deletion_protection`, `engine`, `engine_lifecycle_support`, `engine_version`, `force_destroy`, `global_cluster_identifier`, `global_cluster_resource_id`, `region`, `source_db_cluster_identifier`, `storage_encrypted`, `tags_all`

**Extract attributes** (9): `arn`, `database_name`, `deletion_protection`, `engine`, `engine_version`, `global_cluster_identifier`, `global_cluster_resource_id`, `id`, `storage_encrypted`

**Missing from inject** (5): `endpoint`, `engine_version_actual`, `global_cluster_members`, `tags`, `timeouts`

**Missing from extract** (9): `endpoint`, `engine_lifecycle_support`, `engine_version_actual`, `force_destroy`, `global_cluster_members`, `source_db_cluster_identifier`, `tags`, `tags_all`, `timeouts`

### `aws_rds_instance_state`

**Source:** `crates/winterbaume-terraform/src/converters/rds.rs`

**Inject attributes** (3): `identifier`, `region`, `state`

**Extract attributes** (0): (none)

**Missing from inject** (1): `timeouts`

**Missing from extract** (3): `identifier`, `state`, `timeouts`

### `aws_rds_integration`

**Source:** `crates/winterbaume-terraform/src/converters/rds.rs`

**Inject attributes** (4): `integration_name`, `kms_key_id`, `source_arn`, `target_arn`

**Extract attributes** (0): (none)

**Missing from inject** (6): `additional_encryption_context`, `arn`, `data_filter`, `tags`, `tags_all`, `timeouts`

**Missing from extract** (10): `additional_encryption_context`, `arn`, `data_filter`, `integration_name`, `kms_key_id`, `source_arn`, `tags`, `tags_all`, `target_arn`, `timeouts`

### `aws_rds_reserved_instance`

**Source:** `crates/winterbaume-terraform/src/converters/rds.rs`

**Inject attributes** (3): `instance_count`, `offering_id`, `reservation_id`

**Extract attributes** (0): (none)

**Missing from inject** (16): `arn`, `currency_code`, `db_instance_class`, `duration`, `fixed_price`, `lease_id`, `multi_az`, `offering_type`, `product_description`, `recurring_charges`, `start_time`, `state`, `tags`, `tags_all`, `timeouts`, `usage_price`

**Missing from extract** (19): `arn`, `currency_code`, `db_instance_class`, `duration`, `fixed_price`, `instance_count`, `lease_id`, `multi_az`, `offering_id`, `offering_type`, `product_description`, `recurring_charges`, `reservation_id`, `start_time`, `state`, `tags`, `tags_all`, `timeouts`, `usage_price`

### `aws_rds_shard_group`

**Source:** `crates/winterbaume-terraform/src/converters/rds.rs`

**Inject attributes** (11): `arn`, `compute_redundancy`, `db_cluster_identifier`, `db_shard_group_identifier`, `db_shard_group_resource_id`, `endpoint`, `max_acu`, `min_acu`, `publicly_accessible`, `region`, `tags_all`

**Extract attributes** (11): `arn`, `db_cluster_identifier`, `db_shard_group_identifier`, `db_shard_group_resource_id`, `endpoint`, `id`, `max_acu`, `min_acu`, `publicly_accessible`, `tags`, `tags_all`

**Missing from inject** (2): `tags`, `timeouts`

**Missing from extract** (2): `compute_redundancy`, `timeouts`

### `aws_redshift_authentication_profile`

**Source:** `crates/winterbaume-terraform/src/converters/redshift.rs`

**Inject attributes** (3): `authentication_profile_content`, `authentication_profile_name`, `region`

**Extract attributes** (3): `authentication_profile_content`, `authentication_profile_name`, `id`

### `aws_redshift_cluster`

**Source:** `crates/winterbaume-terraform/src/converters/redshift.rs`

**Inject attributes** (36): `apply_immediately`, `arn`, `automated_snapshot_retention_period`, `availability_zone`, `availability_zone_relocation_enabled`, `cluster_identifier`, `cluster_namespace_arn`, `cluster_parameter_group_name`, `cluster_status`, `cluster_subnet_group_name`, `cluster_version`, `database_name`, `default_iam_role_arn`, `encrypted`, `endpoint`, `enhanced_vpc_routing`, `final_snapshot_identifier`, `iam_roles`, `logging`, `manage_master_password`, `master_password_secret_kms_key_id`, `master_username`, `multi_az`, `node_type`, `number_of_nodes`, `port`, `preferred_maintenance_window`, `publicly_accessible`, `region`, `skip_final_snapshot`, `snapshot_cluster_identifier`, `snapshot_copy`, `snapshot_identifier`, `tags`, `tags_all`, `vpc_id`

**Extract attributes** (43): `arn`, `automated_snapshot_retention_period`, `availability_zone`, `bucket_name`, `cluster_identifier`, `cluster_nodes`, `cluster_parameter_group_name`, `cluster_public_key`, `cluster_revision_number`, `cluster_status`, `cluster_subnet_group_name`, `cluster_type`, `cluster_version`, `database_name`, `destination_region`, `dns_name`, `enable`, `encrypted`, `endpoint`, `grant_name`, `id`, `log_destination_type`, `log_exports`, `logging`, `logging_enabled`, `maintenance_track_name`, `master_password_secret_arn`, `master_username`, `node_role`, `node_type`, `number_of_nodes`, `port`, `preferred_maintenance_window`, `private_ip_address`, `public_ip_address`, `publicly_accessible`, `retention_period`, `s3_key_prefix`, `snapshot_copy`, `tags`, `tags_all`, `vpc_id`, `vpc_security_group_ids`

**Missing from inject** (19): `allow_version_upgrade`, `aqua_configuration_status`, `cluster_nodes`, `cluster_public_key`, `cluster_revision_number`, `cluster_type`, `dns_name`, `elastic_ip`, `kms_key_id`, `maintenance_track_name`, `manual_snapshot_retention_period`, `master_password`, `master_password_secret_arn`, `master_password_wo`, `master_password_wo_version`, `owner_account`, `snapshot_arn`, `timeouts`, `vpc_security_group_ids`

**Missing from extract** (24): `allow_version_upgrade`, `apply_immediately`, `aqua_configuration_status`, `availability_zone_relocation_enabled`, `cluster_namespace_arn`, `default_iam_role_arn`, `elastic_ip`, `enhanced_vpc_routing`, `final_snapshot_identifier`, `iam_roles`, `kms_key_id`, `manage_master_password`, `manual_snapshot_retention_period`, `master_password`, `master_password_secret_kms_key_id`, `master_password_wo`, `master_password_wo_version`, `multi_az`, `owner_account`, `skip_final_snapshot`, `snapshot_arn`, `snapshot_cluster_identifier`, `snapshot_identifier`, `timeouts`

### `aws_redshift_cluster_iam_roles`

**Source:** `crates/winterbaume-terraform/src/converters/redshift.rs`

**Inject attributes** (4): `cluster_identifier`, `default_iam_role_arn`, `iam_role_arns`, `region`

**Extract attributes** (0): (none)

**Missing from inject** (1): `timeouts`

**Missing from extract** (4): `cluster_identifier`, `default_iam_role_arn`, `iam_role_arns`, `timeouts`

### `aws_redshift_cluster_snapshot`

**Source:** `crates/winterbaume-terraform/src/converters/redshift.rs`

**Inject attributes** (6): `arn`, `cluster_identifier`, `region`, `snapshot_identifier`, `tags`, `tags_all`

**Extract attributes** (12): `arn`, `cluster_identifier`, `cluster_version`, `database_name`, `id`, `master_username`, `node_type`, `number_of_nodes`, `snapshot_identifier`, `status`, `tags`, `tags_all`

**Missing from inject** (3): `kms_key_id`, `manual_snapshot_retention_period`, `owner_account`

**Missing from extract** (3): `kms_key_id`, `manual_snapshot_retention_period`, `owner_account`

### `aws_redshift_data_share_authorization`

**Source:** `crates/winterbaume-terraform/src/converters/redshift.rs`

**Inject attributes** (4): `allow_writes`, `consumer_identifier`, `data_share_arn`, `region`

**Extract attributes** (0): (none)

**Missing from inject** (2): `managed_by`, `producer_arn`

**Missing from extract** (5): `allow_writes`, `consumer_identifier`, `data_share_arn`, `managed_by`, `producer_arn`

### `aws_redshift_data_share_consumer_association`

**Source:** `crates/winterbaume-terraform/src/converters/redshift.rs`

**Inject attributes** (6): `allow_writes`, `associate_entire_account`, `consumer_arn`, `consumer_region`, `data_share_arn`, `region`

**Extract attributes** (0): (none)

**Missing from inject** (2): `managed_by`, `producer_arn`

**Missing from extract** (7): `allow_writes`, `associate_entire_account`, `consumer_arn`, `consumer_region`, `data_share_arn`, `managed_by`, `producer_arn`

### `aws_redshift_endpoint_access`

**Source:** `crates/winterbaume-terraform/src/converters/redshift.rs`

**Inject attributes** (6): `cluster_identifier`, `endpoint_name`, `region`, `resource_owner`, `subnet_group_name`, `vpc_security_group_ids`

**Extract attributes** (0): (none)

**Missing from inject** (3): `address`, `port`, `vpc_endpoint`

**Missing from extract** (8): `address`, `cluster_identifier`, `endpoint_name`, `port`, `resource_owner`, `subnet_group_name`, `vpc_endpoint`, `vpc_security_group_ids`

### `aws_redshift_endpoint_authorization`

**Source:** `crates/winterbaume-terraform/src/converters/redshift.rs`

**Inject attributes** (5): `account`, `cluster_identifier`, `force_delete`, `region`, `vpc_ids`

**Extract attributes** (0): (none)

**Missing from inject** (4): `allowed_all_vpcs`, `endpoint_count`, `grantee`, `grantor`

**Missing from extract** (8): `account`, `allowed_all_vpcs`, `cluster_identifier`, `endpoint_count`, `force_delete`, `grantee`, `grantor`, `vpc_ids`

### `aws_redshift_event_subscription`

**Source:** `crates/winterbaume-terraform/src/converters/redshift.rs`

**Inject attributes** (12): `customer_aws_id`, `enabled`, `event_categories`, `name`, `region`, `severity`, `sns_topic_arn`, `source_ids`, `source_type`, `status`, `tags`, `tags_all`

**Extract attributes** (13): `arn`, `customer_aws_id`, `enabled`, `event_categories`, `id`, `name`, `severity`, `sns_topic_arn`, `source_ids`, `source_type`, `status`, `tags`, `tags_all`

**Missing from inject** (2): `arn`, `timeouts`

**Missing from extract** (1): `timeouts`

### `aws_redshift_hsm_client_certificate`

**Source:** `crates/winterbaume-terraform/src/converters/redshift.rs`

**Inject attributes** (5): `hsm_client_certificate_identifier`, `hsm_client_certificate_public_key`, `region`, `tags`, `tags_all`

**Extract attributes** (6): `arn`, `hsm_client_certificate_identifier`, `hsm_client_certificate_public_key`, `id`, `tags`, `tags_all`

**Missing from inject** (1): `arn`

### `aws_redshift_hsm_configuration`

**Source:** `crates/winterbaume-terraform/src/converters/redshift.rs`

**Inject attributes** (7): `description`, `hsm_configuration_identifier`, `hsm_ip_address`, `hsm_partition_name`, `region`, `tags`, `tags_all`

**Extract attributes** (8): `arn`, `description`, `hsm_configuration_identifier`, `hsm_ip_address`, `hsm_partition_name`, `id`, `tags`, `tags_all`

**Missing from inject** (3): `arn`, `hsm_partition_password`, `hsm_server_public_certificate`

**Missing from extract** (2): `hsm_partition_password`, `hsm_server_public_certificate`

### `aws_redshift_integration`

**Source:** `crates/winterbaume-terraform/src/converters/redshift.rs`

**Inject attributes** (9): `additional_encryption_context`, `description`, `integration_name`, `kms_key_id`, `region`, `source_arn`, `tags`, `tags_all`, `target_arn`

**Extract attributes** (0): (none)

**Missing from inject** (2): `arn`, `timeouts`

**Missing from extract** (10): `additional_encryption_context`, `arn`, `description`, `integration_name`, `kms_key_id`, `source_arn`, `tags`, `tags_all`, `target_arn`, `timeouts`

### `aws_redshift_logging`

**Source:** `crates/winterbaume-terraform/src/converters/redshift.rs`

**Inject attributes** (5): `bucket_name`, `cluster_identifier`, `log_destination_type`, `region`, `s3_key_prefix`

**Extract attributes** (6): `bucket_name`, `cluster_identifier`, `id`, `log_destination_type`, `log_exports`, `s3_key_prefix`

**Missing from inject** (1): `log_exports`

### `aws_redshift_parameter_group`

**Source:** `crates/winterbaume-terraform/src/converters/redshift.rs`

**Inject attributes** (7): `description`, `family`, `name`, `parameter`, `region`, `tags`, `tags_all`

**Extract attributes** (9): `arn`, `description`, `family`, `id`, `name`, `parameter`, `tags`, `tags_all`, `value`

**Missing from inject** (1): `arn`

### `aws_redshift_partner`

**Source:** `crates/winterbaume-terraform/src/converters/redshift.rs`

**Inject attributes** (7): `account_id`, `cluster_identifier`, `database_name`, `partner_name`, `region`, `status`, `status_message`

**Extract attributes** (7): `account_id`, `cluster_identifier`, `database_name`, `id`, `partner_name`, `status`, `status_message`

### `aws_redshift_resource_policy`

**Source:** `crates/winterbaume-terraform/src/converters/redshift.rs`

**Inject attributes** (3): `policy`, `region`, `resource_arn`

**Extract attributes** (3): `id`, `policy`, `resource_arn`

### `aws_redshift_scheduled_action`

**Source:** `crates/winterbaume-terraform/src/converters/redshift.rs`

**Inject attributes** (8): `description`, `enable`, `end_time`, `iam_role`, `name`, `region`, `schedule`, `start_time`

**Extract attributes** (9): `description`, `enable`, `end_time`, `iam_role`, `id`, `name`, `schedule`, `start_time`, `state`

**Missing from inject** (1): `target_action`

**Missing from extract** (1): `target_action`

### `aws_redshift_snapshot_copy`

**Source:** `crates/winterbaume-terraform/src/converters/redshift.rs`

**Inject attributes** (5): `cluster_identifier`, `destination_region`, `region`, `retention_period`, `snapshot_copy_grant_name`

**Extract attributes** (5): `cluster_identifier`, `destination_region`, `id`, `retention_period`, `snapshot_copy_grant_name`

**Missing from inject** (1): `manual_snapshot_retention_period`

**Missing from extract** (1): `manual_snapshot_retention_period`

### `aws_redshift_snapshot_copy_grant`

**Source:** `crates/winterbaume-terraform/src/converters/redshift.rs`

**Inject attributes** (5): `kms_key_id`, `region`, `snapshot_copy_grant_name`, `tags`, `tags_all`

**Extract attributes** (6): `arn`, `id`, `kms_key_id`, `snapshot_copy_grant_name`, `tags`, `tags_all`

**Missing from inject** (1): `arn`

### `aws_redshift_snapshot_schedule`

**Source:** `crates/winterbaume-terraform/src/converters/redshift.rs`

**Inject attributes** (6): `definitions`, `description`, `identifier`, `region`, `tags`, `tags_all`

**Extract attributes** (7): `arn`, `definitions`, `description`, `id`, `identifier`, `tags`, `tags_all`

**Missing from inject** (3): `arn`, `force_destroy`, `identifier_prefix`

**Missing from extract** (2): `force_destroy`, `identifier_prefix`

### `aws_redshift_snapshot_schedule_association`

**Source:** `crates/winterbaume-terraform/src/converters/redshift.rs`

**Inject attributes** (3): `cluster_identifier`, `region`, `schedule_identifier`

**Extract attributes** (0): (none)

**Missing from extract** (2): `cluster_identifier`, `schedule_identifier`

### `aws_redshift_subnet_group`

**Source:** `crates/winterbaume-terraform/src/converters/redshift.rs`

**Inject attributes** (7): `description`, `name`, `region`, `subnet_ids`, `tags`, `tags_all`, `vpc_id`

**Extract attributes** (6): `description`, `id`, `name`, `subnet_ids`, `tags`, `vpc_id`

**Missing from inject** (1): `arn`

**Missing from extract** (2): `arn`, `tags_all`

### `aws_redshift_usage_limit`

**Source:** `crates/winterbaume-terraform/src/converters/redshift.rs`

**Inject attributes** (10): `amount`, `breach_action`, `cluster_identifier`, `feature_type`, `id`, `limit_type`, `period`, `region`, `tags`, `tags_all`

**Extract attributes** (10): `amount`, `arn`, `breach_action`, `cluster_identifier`, `feature_type`, `id`, `limit_type`, `period`, `tags`, `tags_all`

**Missing from inject** (1): `arn`

### `aws_rekognition_collection`

**Source:** `crates/winterbaume-terraform/src/converters/rekognition.rs`

**Inject attributes** (5): `arn`, `description`, `kms_key_arn`, `name`, `region`

**Extract attributes** (8): `arn`, `collection_id`, `creation_timestamp`, `face_count`, `face_model_version`, `id`, `tags`, `user_count`

**Missing from inject** (5): `collection_id`, `face_model_version`, `tags`, `tags_all`, `timeouts`

**Missing from extract** (2): `tags_all`, `timeouts`

### `aws_resiliencehub_resiliency_policy`

**Source:** `crates/winterbaume-terraform/src/converters/resiliencehub.rs`

**Inject attributes** (6): `data_location_constraint`, `policy_arn`, `policy_description`, `policy_name`, `region`, `tier`

**Extract attributes** (12): `creation_time`, `data_location_constraint`, `id`, `policy`, `policy_arn`, `policy_description`, `policy_name`, `rpo_in_secs`, `rto_in_secs`, `tags`, `tags_all`, `tier`

**Missing from inject** (8): `arn`, `description`, `estimated_cost_tier`, `name`, `policy`, `tags`, `tags_all`, `timeouts`

**Missing from extract** (5): `arn`, `description`, `estimated_cost_tier`, `name`, `timeouts`

### `aws_resourcegroups_group`

**Source:** `crates/winterbaume-terraform/src/converters/resourcegroups.rs`

**Inject attributes** (7): `arn`, `configuration`, `description`, `name`, `region`, `resource_query`, `tags`

**Extract attributes** (10): `arn`, `description`, `id`, `name`, `parameters`, `query`, `resource_query`, `tags`, `type`, `values`

**Missing from inject** (2): `tags_all`, `timeouts`

**Missing from extract** (3): `configuration`, `tags_all`, `timeouts`

### `aws_rolesanywhere_profile`

**Source:** `crates/winterbaume-terraform/src/converters/rolesanywhere.rs`

**Inject attributes** (10): `accept_role_session_name`, `arn`, `as2_id`, `duration_seconds`, `managed_policy_arns`, `profile_id`, `profile_type`, `region`, `require_instance_properties`, `role_arns`

**Extract attributes** (9): `arn`, `duration_seconds`, `enabled`, `id`, `managed_policy_arns`, `name`, `role_arns`, `session_policy`, `tags`

**Missing from inject** (5): `enabled`, `name`, `session_policy`, `tags`, `tags_all`

**Missing from extract** (2): `require_instance_properties`, `tags_all`

### `aws_rolesanywhere_trust_anchor`

**Source:** `crates/winterbaume-terraform/src/converters/rolesanywhere.rs`

**Inject attributes** (6): `arn`, `enabled`, `name`, `region`, `source`, `tags`

**Extract attributes** (5): `arn`, `enabled`, `id`, `name`, `tags`

**Missing from inject** (2): `notification_settings`, `tags_all`

**Missing from extract** (3): `notification_settings`, `source`, `tags_all`

### `aws_route53_cidr_collection`

**Source:** `crates/winterbaume-terraform/src/converters/route53.rs`

**Inject attributes** (2): `name`, `region`

**Extract attributes** (4): `arn`, `id`, `name`, `version`

**Missing from inject** (2): `arn`, `version`

### `aws_route53_cidr_location`

**Source:** `crates/winterbaume-terraform/src/converters/route53.rs`

**Inject attributes** (4): `cidr_blocks`, `cidr_collection_id`, `name`, `region`

**Extract attributes** (4): `cidr_blocks`, `cidr_collection_id`, `id`, `name`

### `aws_route53_delegation_set`

**Source:** `crates/winterbaume-terraform/src/converters/route53.rs`

**Inject attributes** (2): `reference_name`, `region`

**Extract attributes** (3): `id`, `name_servers`, `reference_name`

**Missing from inject** (2): `arn`, `name_servers`

**Missing from extract** (1): `arn`

### `aws_route53_health_check`

**Source:** `crates/winterbaume-terraform/src/converters/route53.rs`

**Inject attributes** (24): `child_health_checks`, `child_healthchecks_threshold`, `cloudwatch_alarm_name`, `cloudwatch_alarm_region`, `disabled`, `enable_sni`, `failure_threshold`, `fqdn`, `id`, `insufficient_data_health_status`, `inverted`, `ip_address`, `measure_latency`, `port`, `reference_name`, `region`, `regions`, `request_interval`, `resource_path`, `routing_control_arn`, `search_string`, `tags`, `tags_all`, `type`

**Extract attributes** (20): `child_health_checks`, `child_healthchecks_threshold`, `disabled`, `enable_sni`, `failure_threshold`, `fqdn`, `id`, `insufficient_data_health_status`, `inverted`, `ip_address`, `measure_latency`, `port`, `reference_name`, `regions`, `request_interval`, `resource_path`, `search_string`, `tags`, `tags_all`, `type`

**Missing from inject** (5): `arn`, `child_health_threshold`, `child_healthchecks`, `invert_healthcheck`, `triggers`

**Missing from extract** (8): `arn`, `child_health_threshold`, `child_healthchecks`, `cloudwatch_alarm_name`, `cloudwatch_alarm_region`, `invert_healthcheck`, `routing_control_arn`, `triggers`

### `aws_route53_hosted_zone_dnssec`

**Source:** `crates/winterbaume-terraform/src/converters/route53.rs`

**Inject attributes** (3): `hosted_zone_id`, `region`, `signing_status`

**Extract attributes** (3): `hosted_zone_id`, `id`, `signing_status`

**Missing from inject** (1): `timeouts`

**Missing from extract** (1): `timeouts`

### `aws_route53_key_signing_key`

**Source:** `crates/winterbaume-terraform/src/converters/route53.rs`

**Inject attributes** (5): `hosted_zone_id`, `key_management_service_arn`, `name`, `region`, `status`

**Extract attributes** (5): `hosted_zone_id`, `id`, `key_management_service_arn`, `name`, `status`

**Missing from inject** (11): `digest_algorithm_mnemonic`, `digest_algorithm_type`, `digest_value`, `dnskey_record`, `ds_record`, `flag`, `key_tag`, `public_key`, `signing_algorithm_mnemonic`, `signing_algorithm_type`, `timeouts`

**Missing from extract** (11): `digest_algorithm_mnemonic`, `digest_algorithm_type`, `digest_value`, `dnskey_record`, `ds_record`, `flag`, `key_tag`, `public_key`, `signing_algorithm_mnemonic`, `signing_algorithm_type`, `timeouts`

### `aws_route53_query_log`

**Source:** `crates/winterbaume-terraform/src/converters/route53.rs`

**Inject attributes** (4): `cloudwatch_log_group_arn`, `id`, `region`, `zone_id`

**Extract attributes** (3): `cloudwatch_log_group_arn`, `id`, `zone_id`

**Missing from inject** (1): `arn`

**Missing from extract** (1): `arn`

### `aws_route53_record`

**Source:** `crates/winterbaume-terraform/src/converters/route53.rs`

**Inject attributes** (16): `alias`, `allow_overwrite`, `cidr_routing_policy`, `failover_routing_policy`, `geolocation_routing_policy`, `health_check_id`, `latency_routing_policy`, `multivalue_answer_routing_policy`, `name`, `records`, `region`, `set_identifier`, `ttl`, `type`, `weighted_routing_policy`, `zone_id`

**Extract attributes** (16): `alias`, `cidr_routing_policy`, `failover_routing_policy`, `fqdn`, `geolocation_routing_policy`, `health_check_id`, `id`, `latency_routing_policy`, `multivalue_answer_routing_policy`, `name`, `records`, `set_identifier`, `ttl`, `type`, `weighted_routing_policy`, `zone_id`

**Missing from inject** (3): `fqdn`, `geoproximity_routing_policy`, `timeouts`

**Missing from extract** (3): `allow_overwrite`, `geoproximity_routing_policy`, `timeouts`

### `aws_route53_traffic_policy`

**Source:** `crates/winterbaume-terraform/src/converters/route53.rs`

**Inject attributes** (7): `comment`, `document`, `id`, `name`, `region`, `type`, `version`

**Extract attributes** (6): `comment`, `document`, `id`, `name`, `type`, `version`

**Missing from inject** (1): `arn`

**Missing from extract** (1): `arn`

### `aws_route53_traffic_policy_instance`

**Source:** `crates/winterbaume-terraform/src/converters/route53.rs`

**Inject attributes** (7): `hosted_zone_id`, `id`, `name`, `region`, `traffic_policy_id`, `traffic_policy_version`, `ttl`

**Extract attributes** (7): `hosted_zone_id`, `id`, `name`, `traffic_policy_id`, `traffic_policy_type`, `traffic_policy_version`, `ttl`

**Missing from inject** (1): `arn`

**Missing from extract** (1): `arn`

### `aws_route53_vpc_association_authorization`

**Source:** `crates/winterbaume-terraform/src/converters/route53.rs`

**Inject attributes** (4): `region`, `vpc_id`, `vpc_region`, `zone_id`

**Extract attributes** (4): `id`, `vpc_id`, `vpc_region`, `zone_id`

**Missing from inject** (1): `timeouts`

**Missing from extract** (1): `timeouts`

### `aws_route53_zone`

**Source:** `crates/winterbaume-terraform/src/converters/route53.rs`

**Inject attributes** (9): `comment`, `delegation_set_id`, `force_destroy`, `name`, `region`, `tags`, `tags_all`, `vpc`, `zone_id`

**Extract attributes** (10): `comment`, `id`, `name`, `name_servers`, `tags`, `tags_all`, `vpc`, `vpc_id`, `vpc_region`, `zone_id`

**Missing from inject** (4): `arn`, `name_servers`, `primary_name_server`, `timeouts`

**Missing from extract** (5): `arn`, `delegation_set_id`, `force_destroy`, `primary_name_server`, `timeouts`

### `aws_route53_zone_association`

**Source:** `crates/winterbaume-terraform/src/converters/route53.rs`

**Inject attributes** (4): `region`, `vpc_id`, `vpc_region`, `zone_id`

**Extract attributes** (4): `id`, `vpc_id`, `vpc_region`, `zone_id`

**Missing from inject** (2): `owning_account`, `timeouts`

**Missing from extract** (2): `owning_account`, `timeouts`

### `aws_route53domains_registered_domain`

**Source:** `crates/winterbaume-terraform/src/converters/route53domains.rs`

**Inject attributes** (10): `admin_privacy`, `auto_renew`, `billing_contact`, `billing_privacy`, `domain_name`, `name_server`, `region`, `registrant_privacy`, `tech_privacy`, `transfer_lock`

**Extract attributes** (20): `admin_contact`, `admin_privacy`, `auto_renew`, `billing_contact`, `billing_privacy`, `creation_date`, `domain_name`, `expiration_date`, `glue_ips`, `id`, `name`, `name_server`, `nameservers`, `registrant_contact`, `registrant_privacy`, `status_list`, `tech_contact`, `tech_privacy`, `transfer_lock`, `updated_date`

**Missing from inject** (16): `abuse_contact_email`, `abuse_contact_phone`, `admin_contact`, `creation_date`, `expiration_date`, `registrant_contact`, `registrar_name`, `registrar_url`, `reseller`, `status_list`, `tags`, `tags_all`, `tech_contact`, `timeouts`, `updated_date`, `whois_server`

**Missing from extract** (9): `abuse_contact_email`, `abuse_contact_phone`, `registrar_name`, `registrar_url`, `reseller`, `tags`, `tags_all`, `timeouts`, `whois_server`

### `aws_route53_resolver_config`

**Source:** `crates/winterbaume-terraform/src/converters/route53resolver.rs`

**Inject attributes** (4): `autodefined_reverse_flag`, `owner_id`, `region`, `resource_id`

**Extract attributes** (0): (none)

**Missing from extract** (3): `autodefined_reverse_flag`, `owner_id`, `resource_id`

### `aws_route53_resolver_dnssec_config`

**Source:** `crates/winterbaume-terraform/src/converters/route53resolver.rs`

**Inject attributes** (4): `owner_id`, `region`, `resource_id`, `validation_status`

**Extract attributes** (4): `id`, `owner_id`, `resource_id`, `validation_status`

**Missing from inject** (1): `arn`

**Missing from extract** (1): `arn`

### `aws_route53_resolver_endpoint`

**Source:** `crates/winterbaume-terraform/src/converters/route53resolver.rs`

**Inject attributes** (11): `arn`, `direction`, `host_vpc_id`, `ip_address`, `name`, `protocols`, `region`, `resolver_endpoint_type`, `security_group_ids`, `tags`, `tags_all`

**Extract attributes** (16): `arn`, `direction`, `host_vpc_id`, `id`, `ip`, `ip_address`, `ip_address_count`, `ip_id`, `name`, `protocols`, `resolver_endpoint_type`, `security_group_ids`, `status`, `status_message`, `subnet_id`, `tags`

**Missing from inject** (1): `timeouts`

**Missing from extract** (2): `tags_all`, `timeouts`

### `aws_route53_resolver_firewall_config`

**Source:** `crates/winterbaume-terraform/src/converters/route53resolver.rs`

**Inject attributes** (4): `firewall_fail_open`, `owner_id`, `region`, `resource_id`

**Extract attributes** (0): (none)

**Missing from extract** (3): `firewall_fail_open`, `owner_id`, `resource_id`

### `aws_route53_resolver_firewall_domain_list`

**Source:** `crates/winterbaume-terraform/src/converters/route53resolver.rs`

**Inject attributes** (3): `arn`, `name`, `region`

**Extract attributes** (0): (none)

**Missing from inject** (3): `domains`, `tags`, `tags_all`

**Missing from extract** (5): `arn`, `domains`, `name`, `tags`, `tags_all`

### `aws_route53_resolver_firewall_rule`

**Source:** `crates/winterbaume-terraform/src/converters/route53resolver.rs`

**Inject attributes** (12): `action`, `block_override_dns_type`, `block_override_domain`, `block_override_ttl`, `block_response`, `firewall_domain_list_id`, `firewall_domain_redirection_action`, `firewall_rule_group_id`, `name`, `priority`, `qtype`, `region`

**Extract attributes** (0): (none)

**Missing from inject** (1): `q_type`

**Missing from extract** (11): `action`, `block_override_dns_type`, `block_override_domain`, `block_override_ttl`, `block_response`, `firewall_domain_list_id`, `firewall_domain_redirection_action`, `firewall_rule_group_id`, `name`, `priority`, `q_type`

### `aws_route53_resolver_firewall_rule_group`

**Source:** `crates/winterbaume-terraform/src/converters/route53resolver.rs`

**Inject attributes** (5): `arn`, `name`, `owner_id`, `region`, `share_status`

**Extract attributes** (0): (none)

**Missing from inject** (2): `tags`, `tags_all`

**Missing from extract** (6): `arn`, `name`, `owner_id`, `share_status`, `tags`, `tags_all`

### `aws_route53_resolver_firewall_rule_group_association`

**Source:** `crates/winterbaume-terraform/src/converters/route53resolver.rs`

**Inject attributes** (7): `arn`, `firewall_rule_group_id`, `mutation_protection`, `name`, `priority`, `region`, `vpc_id`

**Extract attributes** (0): (none)

**Missing from inject** (2): `tags`, `tags_all`

**Missing from extract** (8): `arn`, `firewall_rule_group_id`, `mutation_protection`, `name`, `priority`, `tags`, `tags_all`, `vpc_id`

### `aws_route53_resolver_query_log_config`

**Source:** `crates/winterbaume-terraform/src/converters/route53resolver.rs`

**Inject attributes** (6): `arn`, `destination_arn`, `name`, `region`, `tags`, `tags_all`

**Extract attributes** (9): `arn`, `association_count`, `destination_arn`, `id`, `name`, `owner_id`, `share_status`, `status`, `tags`

**Missing from inject** (2): `owner_id`, `share_status`

**Missing from extract** (1): `tags_all`

### `aws_route53_resolver_query_log_config_association`

**Source:** `crates/winterbaume-terraform/src/converters/route53resolver.rs`

**Inject attributes** (3): `region`, `resolver_query_log_config_id`, `resource_id`

**Extract attributes** (6): `error`, `error_message`, `id`, `resolver_query_log_config_id`, `resource_id`, `status`

### `aws_route53_resolver_rule`

**Source:** `crates/winterbaume-terraform/src/converters/route53resolver.rs`

**Inject attributes** (9): `arn`, `domain_name`, `name`, `region`, `resolver_endpoint_id`, `rule_type`, `tags`, `tags_all`, `target_ip`

**Extract attributes** (15): `arn`, `domain_name`, `id`, `ip`, `ipv6`, `name`, `owner_id`, `port`, `protocol`, `resolver_endpoint_id`, `rule_type`, `share_status`, `status`, `tags`, `target_ip`

**Missing from inject** (3): `owner_id`, `share_status`, `timeouts`

**Missing from extract** (2): `tags_all`, `timeouts`

### `aws_route53_resolver_rule_association`

**Source:** `crates/winterbaume-terraform/src/converters/route53resolver.rs`

**Inject attributes** (4): `name`, `region`, `resolver_rule_id`, `vpc_id`

**Extract attributes** (6): `id`, `name`, `resolver_rule_id`, `status`, `status_message`, `vpc_id`

**Missing from inject** (1): `timeouts`

**Missing from extract** (1): `timeouts`

### `aws_s3_access_point`

**Source:** `crates/winterbaume-terraform/src/converters/s3.rs`

**Inject attributes** (0): (none)

**Extract attributes** (0): (none)

**Missing from inject** (13): `account_id`, `alias`, `arn`, `bucket`, `bucket_account_id`, `domain_name`, `endpoints`, `has_public_access_policy`, `name`, `network_origin`, `policy`, `public_access_block_configuration`, `vpc_configuration`

**Missing from extract** (13): `account_id`, `alias`, `arn`, `bucket`, `bucket_account_id`, `domain_name`, `endpoints`, `has_public_access_policy`, `name`, `network_origin`, `policy`, `public_access_block_configuration`, `vpc_configuration`

### `aws_s3_account_public_access_block`

**Source:** `crates/winterbaume-terraform/src/converters/s3.rs`

**Inject attributes** (0): (none)

**Extract attributes** (0): (none)

**Missing from inject** (5): `account_id`, `block_public_acls`, `block_public_policy`, `ignore_public_acls`, `restrict_public_buckets`

**Missing from extract** (5): `account_id`, `block_public_acls`, `block_public_policy`, `ignore_public_acls`, `restrict_public_buckets`

### `aws_s3_bucket`

**Source:** `crates/winterbaume-terraform/src/converters/s3.rs`

**Inject attributes** (21): `acceleration_status`, `acl`, `bucket`, `cors_rule`, `creation_date`, `force_destroy`, `lifecycle_rule`, `logging`, `notification_configuration`, `object_lock_configuration`, `ownership_controls`, `policy`, `region`, `replication_configuration`, `request_payer`, `server_side_encryption_configuration`, `tags`, `tags_all`, `versioning`, `versioning_status`, `website`

**Extract attributes** (21): `acceleration_status`, `acl`, `arn`, `bucket`, `bucket_domain_name`, `bucket_regional_domain_name`, `cors_rule`, `id`, `lifecycle_rule`, `logging`, `object_lock_configuration`, `ownership_controls`, `policy`, `region`, `replication_configuration`, `request_payer`, `server_side_encryption_configuration`, `tags`, `tags_all`, `versioning`, `website`

**Missing from inject** (10): `arn`, `bucket_domain_name`, `bucket_prefix`, `bucket_regional_domain_name`, `grant`, `hosted_zone_id`, `object_lock_enabled`, `timeouts`, `website_domain`, `website_endpoint`

**Missing from extract** (8): `bucket_prefix`, `force_destroy`, `grant`, `hosted_zone_id`, `object_lock_enabled`, `timeouts`, `website_domain`, `website_endpoint`

### `aws_s3_bucket_accelerate_configuration`

**Source:** `crates/winterbaume-terraform/src/converters/s3.rs`

**Inject attributes** (2): `bucket`, `status`

**Extract attributes** (2): `bucket`, `status`

**Missing from inject** (1): `expected_bucket_owner`

**Missing from extract** (1): `expected_bucket_owner`

### `aws_s3_bucket_acl`

**Source:** `crates/winterbaume-terraform/src/converters/s3.rs`

**Inject attributes** (3): `access_control_policy`, `acl`, `bucket`

**Extract attributes** (2): `acl`, `bucket`

**Missing from inject** (1): `expected_bucket_owner`

**Missing from extract** (2): `access_control_policy`, `expected_bucket_owner`

### `aws_s3_bucket_analytics_configuration`

**Source:** `crates/winterbaume-terraform/src/converters/s3.rs`

**Inject attributes** (2): `bucket`, `name`

**Extract attributes** (2): `bucket`, `name`

**Missing from inject** (2): `filter`, `storage_class_analysis`

**Missing from extract** (2): `filter`, `storage_class_analysis`

### `aws_s3_bucket_cors_configuration`

**Source:** `crates/winterbaume-terraform/src/converters/s3.rs`

**Inject attributes** (2): `bucket`, `cors_rule`

**Extract attributes** (2): `bucket`, `cors_rule`

**Missing from inject** (1): `expected_bucket_owner`

**Missing from extract** (1): `expected_bucket_owner`

### `aws_s3_bucket_intelligent_tiering_configuration`

**Source:** `crates/winterbaume-terraform/src/converters/s3.rs`

**Inject attributes** (3): `bucket`, `name`, `status`

**Extract attributes** (2): `bucket`, `name`

**Missing from inject** (2): `filter`, `tiering`

**Missing from extract** (3): `filter`, `status`, `tiering`

### `aws_s3_bucket_inventory`

**Source:** `crates/winterbaume-terraform/src/converters/s3.rs`

**Inject attributes** (0): (none)

**Extract attributes** (0): (none)

**Missing from inject** (8): `bucket`, `destination`, `enabled`, `filter`, `included_object_versions`, `name`, `optional_fields`, `schedule`

**Missing from extract** (8): `bucket`, `destination`, `enabled`, `filter`, `included_object_versions`, `name`, `optional_fields`, `schedule`

### `aws_s3_bucket_lifecycle_configuration`

**Source:** `crates/winterbaume-terraform/src/converters/s3.rs`

**Inject attributes** (2): `bucket`, `rule`

**Extract attributes** (2): `bucket`, `rule`

**Missing from inject** (3): `expected_bucket_owner`, `timeouts`, `transition_default_minimum_object_size`

**Missing from extract** (3): `expected_bucket_owner`, `timeouts`, `transition_default_minimum_object_size`

### `aws_s3_bucket_logging`

**Source:** `crates/winterbaume-terraform/src/converters/s3.rs`

**Inject attributes** (3): `bucket`, `target_bucket`, `target_prefix`

**Extract attributes** (4): `bucket`, `logging`, `target_bucket`, `target_prefix`

**Missing from inject** (3): `expected_bucket_owner`, `target_grant`, `target_object_key_format`

**Missing from extract** (3): `expected_bucket_owner`, `target_grant`, `target_object_key_format`

### `aws_s3_bucket_metric`

**Source:** `crates/winterbaume-terraform/src/converters/s3.rs`

**Inject attributes** (2): `bucket`, `name`

**Extract attributes** (2): `bucket`, `name`

**Missing from inject** (1): `filter`

**Missing from extract** (1): `filter`

### `aws_s3_bucket_notification`

**Source:** `crates/winterbaume-terraform/src/converters/s3.rs`

**Inject attributes** (4): `bucket`, `lambda_function`, `queue`, `topic`

**Extract attributes** (5): `bucket`, `lambda_function`, `notification_configuration`, `queue`, `topic`

**Missing from inject** (1): `eventbridge`

**Missing from extract** (1): `eventbridge`

### `aws_s3_bucket_object`

**Source:** `crates/winterbaume-terraform/src/converters/s3.rs`

**Inject attributes** (0): (none)

**Extract attributes** (0): (none)

**Missing from inject** (27): `acl`, `arn`, `bucket`, `bucket_key_enabled`, `cache_control`, `content`, `content_base64`, `content_disposition`, `content_encoding`, `content_language`, `content_type`, `etag`, `force_destroy`, `key`, `kms_key_id`, `metadata`, `object_lock_legal_hold_status`, `object_lock_mode`, `object_lock_retain_until_date`, `server_side_encryption`, `source`, `source_hash`, `storage_class`, `tags`, `tags_all`, `version_id`, `website_redirect`

**Missing from extract** (27): `acl`, `arn`, `bucket`, `bucket_key_enabled`, `cache_control`, `content`, `content_base64`, `content_disposition`, `content_encoding`, `content_language`, `content_type`, `etag`, `force_destroy`, `key`, `kms_key_id`, `metadata`, `object_lock_legal_hold_status`, `object_lock_mode`, `object_lock_retain_until_date`, `server_side_encryption`, `source`, `source_hash`, `storage_class`, `tags`, `tags_all`, `version_id`, `website_redirect`

### `aws_s3_bucket_object_lock_configuration`

**Source:** `crates/winterbaume-terraform/src/converters/s3.rs`

**Inject attributes** (3): `bucket`, `object_lock_enabled`, `rule`

**Extract attributes** (4): `bucket`, `object_lock_configuration`, `object_lock_enabled`, `rule`

**Missing from inject** (2): `expected_bucket_owner`, `token`

**Missing from extract** (2): `expected_bucket_owner`, `token`

### `aws_s3_bucket_ownership_controls`

**Source:** `crates/winterbaume-terraform/src/converters/s3.rs`

**Inject attributes** (2): `bucket`, `rule`

**Extract attributes** (2): `bucket`, `rule`

### `aws_s3_bucket_policy`

**Source:** `crates/winterbaume-terraform/src/converters/s3.rs`

**Inject attributes** (2): `bucket`, `policy`

**Extract attributes** (2): `bucket`, `policy`

### `aws_s3_bucket_public_access_block`

**Source:** `crates/winterbaume-terraform/src/converters/s3.rs`

**Inject attributes** (0): (none)

**Extract attributes** (0): (none)

**Missing from inject** (5): `block_public_acls`, `block_public_policy`, `bucket`, `ignore_public_acls`, `restrict_public_buckets`

**Missing from extract** (5): `block_public_acls`, `block_public_policy`, `bucket`, `ignore_public_acls`, `restrict_public_buckets`

### `aws_s3_bucket_replication_configuration`

**Source:** `crates/winterbaume-terraform/src/converters/s3.rs`

**Inject attributes** (3): `bucket`, `role`, `rule`

**Extract attributes** (4): `bucket`, `replication_configuration`, `role`, `rule`

**Missing from inject** (1): `token`

**Missing from extract** (1): `token`

### `aws_s3_bucket_request_payment_configuration`

**Source:** `crates/winterbaume-terraform/src/converters/s3.rs`

**Inject attributes** (0): (none)

**Extract attributes** (0): (none)

**Missing from inject** (3): `bucket`, `expected_bucket_owner`, `payer`

**Missing from extract** (3): `bucket`, `expected_bucket_owner`, `payer`

### `aws_s3_bucket_server_side_encryption_configuration`

**Source:** `crates/winterbaume-terraform/src/converters/s3.rs`

**Inject attributes** (2): `bucket`, `rule`

**Extract attributes** (2): `bucket`, `rule`

**Missing from inject** (1): `expected_bucket_owner`

**Missing from extract** (1): `expected_bucket_owner`

### `aws_s3_bucket_versioning`

**Source:** `crates/winterbaume-terraform/src/converters/s3.rs`

**Inject attributes** (3): `bucket`, `status`, `versioning_configuration`

**Extract attributes** (3): `bucket`, `status`, `versioning_configuration`

**Missing from inject** (2): `expected_bucket_owner`, `mfa`

**Missing from extract** (2): `expected_bucket_owner`, `mfa`

### `aws_s3_bucket_website_configuration`

**Source:** `crates/winterbaume-terraform/src/converters/s3.rs`

**Inject attributes** (5): `bucket`, `error_document`, `index_document`, `redirect_all_requests_to`, `routing_rule`

**Extract attributes** (6): `bucket`, `error_document`, `index_document`, `redirect_all_requests_to`, `routing_rule`, `website_configuration`

**Missing from inject** (4): `expected_bucket_owner`, `routing_rules`, `website_domain`, `website_endpoint`

**Missing from extract** (4): `expected_bucket_owner`, `routing_rules`, `website_domain`, `website_endpoint`

### `aws_s3_directory_bucket`

**Source:** `crates/winterbaume-terraform/src/converters/s3.rs`

**Inject attributes** (0): (none)

**Extract attributes** (0): (none)

**Missing from inject** (6): `arn`, `bucket`, `data_redundancy`, `force_destroy`, `location`, `type`

**Missing from extract** (6): `arn`, `bucket`, `data_redundancy`, `force_destroy`, `location`, `type`

### `aws_s3_object`

**Source:** `crates/winterbaume-terraform/src/converters/s3.rs`

**Inject attributes** (0): (none)

**Extract attributes** (0): (none)

**Missing from inject** (34): `acl`, `arn`, `bucket`, `bucket_key_enabled`, `cache_control`, `checksum_algorithm`, `checksum_crc32`, `checksum_crc32c`, `checksum_crc64nvme`, `checksum_sha1`, `checksum_sha256`, `content`, `content_base64`, `content_disposition`, `content_encoding`, `content_language`, `content_type`, `etag`, `force_destroy`, `key`, `kms_key_id`, `metadata`, `object_lock_legal_hold_status`, `object_lock_mode`, `object_lock_retain_until_date`, `override_provider`, `server_side_encryption`, `source`, `source_hash`, `storage_class`, `tags`, `tags_all`, `version_id`, `website_redirect`

**Missing from extract** (34): `acl`, `arn`, `bucket`, `bucket_key_enabled`, `cache_control`, `checksum_algorithm`, `checksum_crc32`, `checksum_crc32c`, `checksum_crc64nvme`, `checksum_sha1`, `checksum_sha256`, `content`, `content_base64`, `content_disposition`, `content_encoding`, `content_language`, `content_type`, `etag`, `force_destroy`, `key`, `kms_key_id`, `metadata`, `object_lock_legal_hold_status`, `object_lock_mode`, `object_lock_retain_until_date`, `override_provider`, `server_side_encryption`, `source`, `source_hash`, `storage_class`, `tags`, `tags_all`, `version_id`, `website_redirect`

### `aws_s3_object_copy`

**Source:** `crates/winterbaume-terraform/src/converters/s3.rs`

**Inject attributes** (0): (none)

**Extract attributes** (0): (none)

**Missing from inject** (53): `acl`, `arn`, `bucket`, `bucket_key_enabled`, `cache_control`, `checksum_algorithm`, `checksum_crc32`, `checksum_crc32c`, `checksum_crc64nvme`, `checksum_sha1`, `checksum_sha256`, `content_disposition`, `content_encoding`, `content_language`, `content_type`, `copy_if_match`, `copy_if_modified_since`, `copy_if_none_match`, `copy_if_unmodified_since`, `customer_algorithm`, `customer_key`, `customer_key_md5`, `etag`, `expected_bucket_owner`, `expected_source_bucket_owner`, `expiration`, `expires`, `force_destroy`, `grant`, `key`, `kms_encryption_context`, `kms_key_id`, `last_modified`, `metadata`, `metadata_directive`, `object_lock_legal_hold_status`, `object_lock_mode`, `object_lock_retain_until_date`, `override_provider`, `request_charged`, `request_payer`, `server_side_encryption`, `source`, `source_customer_algorithm`, `source_customer_key`, `source_customer_key_md5`, `source_version_id`, `storage_class`, `tagging_directive`, `tags`, `tags_all`, `version_id`, `website_redirect`

**Missing from extract** (53): `acl`, `arn`, `bucket`, `bucket_key_enabled`, `cache_control`, `checksum_algorithm`, `checksum_crc32`, `checksum_crc32c`, `checksum_crc64nvme`, `checksum_sha1`, `checksum_sha256`, `content_disposition`, `content_encoding`, `content_language`, `content_type`, `copy_if_match`, `copy_if_modified_since`, `copy_if_none_match`, `copy_if_unmodified_since`, `customer_algorithm`, `customer_key`, `customer_key_md5`, `etag`, `expected_bucket_owner`, `expected_source_bucket_owner`, `expiration`, `expires`, `force_destroy`, `grant`, `key`, `kms_encryption_context`, `kms_key_id`, `last_modified`, `metadata`, `metadata_directive`, `object_lock_legal_hold_status`, `object_lock_mode`, `object_lock_retain_until_date`, `override_provider`, `request_charged`, `request_payer`, `server_side_encryption`, `source`, `source_customer_algorithm`, `source_customer_key`, `source_customer_key_md5`, `source_version_id`, `storage_class`, `tagging_directive`, `tags`, `tags_all`, `version_id`, `website_redirect`

### `aws_s3control_access_grant`

**Source:** `crates/winterbaume-terraform/src/converters/s3control.rs`

**Inject attributes** (10): `access_grant_arn`, `access_grant_id`, `access_grants_location_id`, `account_id`, `application_arn`, `created_at`, `grant_scope`, `permission`, `region`, `s3_prefix_type`

**Extract attributes** (8): `access_grant_arn`, `access_grant_id`, `access_grants_location_id`, `application_arn`, `created_at`, `grant_scope`, `id`, `permission`

**Missing from inject** (4): `access_grants_location_configuration`, `grantee`, `tags`, `tags_all`

**Missing from extract** (6): `access_grants_location_configuration`, `account_id`, `grantee`, `s3_prefix_type`, `tags`, `tags_all`

### `aws_s3control_access_grants_instance`

**Source:** `crates/winterbaume-terraform/src/converters/s3control.rs`

**Inject attributes** (7): `access_grants_instance_arn`, `access_grants_instance_id`, `account_id`, `created_at`, `identity_center_application_arn`, `identity_center_arn`, `region`

**Extract attributes** (6): `access_grants_instance_arn`, `access_grants_instance_id`, `created_at`, `id`, `identity_center_application_arn`, `identity_center_arn`

**Missing from inject** (2): `tags`, `tags_all`

**Missing from extract** (3): `account_id`, `tags`, `tags_all`

### `aws_s3control_access_grants_instance_resource_policy`

**Source:** `crates/winterbaume-terraform/src/converters/s3control.rs`

**Inject attributes** (5): `account_id`, `created_at`, `organization`, `policy`, `region`

**Extract attributes** (4): `created_at`, `id`, `organization`, `policy`

**Missing from extract** (1): `account_id`

### `aws_s3control_access_grants_location`

**Source:** `crates/winterbaume-terraform/src/converters/s3control.rs`

**Inject attributes** (7): `access_grants_location_arn`, `access_grants_location_id`, `account_id`, `created_at`, `iam_role_arn`, `location_scope`, `region`

**Extract attributes** (6): `access_grants_location_arn`, `access_grants_location_id`, `created_at`, `iam_role_arn`, `id`, `location_scope`

**Missing from inject** (2): `tags`, `tags_all`

**Missing from extract** (3): `account_id`, `tags`, `tags_all`

### `aws_s3control_access_point`

**Source:** `crates/winterbaume-terraform/src/converters/s3control.rs`

**Inject attributes** (14): `account_id`, `alias`, `arn`, `block_public_acls`, `block_public_policy`, `bucket`, `creation_date`, `ignore_public_acls`, `name`, `network_origin`, `policy`, `region`, `restrict_public_buckets`, `vpc_id`

**Extract attributes** (15): `account_id`, `alias`, `arn`, `block_public_acls`, `block_public_policy`, `bucket`, `creation_date`, `id`, `ignore_public_acls`, `name`, `network_origin`, `policy`, `region`, `restrict_public_buckets`, `vpc_id`

**Note:** Resource type not found in Terraform AWS provider schema.

### `aws_s3control_access_point_policy`

**Source:** `crates/winterbaume-terraform/src/converters/s3control.rs`

**Inject attributes** (4): `access_point_arn`, `has_public_access_policy`, `policy`, `region`

**Extract attributes** (0): (none)

**Missing from extract** (3): `access_point_arn`, `has_public_access_policy`, `policy`

### `aws_s3control_bucket`

**Source:** `crates/winterbaume-terraform/src/converters/s3control.rs`

**Inject attributes** (7): `arn`, `bucket`, `creation_date`, `outpost_id`, `policy`, `region`, `tags`

**Extract attributes** (8): `arn`, `bucket`, `creation_date`, `id`, `outpost_id`, `policy`, `public_access_block_enabled`, `tags`

**Missing from inject** (2): `public_access_block_enabled`, `tags_all`

**Missing from extract** (1): `tags_all`

### `aws_s3control_bucket_lifecycle_configuration`

**Source:** `crates/winterbaume-terraform/src/converters/s3control.rs`

**Inject attributes** (2): `bucket`, `region`

**Extract attributes** (3): `bucket`, `id`, `rule`

**Missing from inject** (1): `rule`

### `aws_s3control_bucket_policy`

**Source:** `crates/winterbaume-terraform/src/converters/s3control.rs`

**Inject attributes** (3): `bucket`, `policy`, `region`

**Extract attributes** (0): (none)

**Missing from extract** (2): `bucket`, `policy`

### `aws_s3control_directory_bucket_access_point_scope`

**Source:** `crates/winterbaume-terraform/src/converters/s3control.rs`

**Inject attributes** (3): `account_id`, `name`, `region`

**Extract attributes** (5): `id`, `name`, `permissions`, `prefixes`, `scope`

**Missing from inject** (1): `scope`

**Missing from extract** (1): `account_id`

### `aws_s3control_multi_region_access_point`

**Source:** `crates/winterbaume-terraform/src/converters/s3control.rs`

**Inject attributes** (6): `account_id`, `alias`, `arn`, `domain_name`, `region`, `status`

**Extract attributes** (4): `alias`, `arn`, `id`, `status`

**Missing from inject** (2): `details`, `timeouts`

**Missing from extract** (4): `account_id`, `details`, `domain_name`, `timeouts`

### `aws_s3control_multi_region_access_point_policy`

**Source:** `crates/winterbaume-terraform/src/converters/s3control.rs`

**Inject attributes** (4): `account_id`, `established`, `proposed`, `region`

**Extract attributes** (0): (none)

**Missing from inject** (2): `details`, `timeouts`

**Missing from extract** (5): `account_id`, `details`, `established`, `proposed`, `timeouts`

### `aws_s3control_object_lambda_access_point`

**Source:** `crates/winterbaume-terraform/src/converters/s3control.rs`

**Inject attributes** (5): `account_id`, `alias`, `arn`, `name`, `region`

**Extract attributes** (5): `alias`, `arn`, `configuration`, `id`, `name`

**Missing from inject** (1): `configuration`

**Missing from extract** (1): `account_id`

### `aws_s3control_object_lambda_access_point_policy`

**Source:** `crates/winterbaume-terraform/src/converters/s3control.rs`

**Inject attributes** (5): `account_id`, `has_public_access_policy`, `name`, `policy`, `region`

**Extract attributes** (0): (none)

**Missing from extract** (4): `account_id`, `has_public_access_policy`, `name`, `policy`

### `aws_s3control_storage_lens_configuration`

**Source:** `crates/winterbaume-terraform/src/converters/s3control.rs`

**Inject attributes** (5): `account_id`, `arn`, `config_id`, `region`, `tags`

**Extract attributes** (4): `arn`, `config_id`, `id`, `tags`

**Missing from inject** (2): `storage_lens_configuration`, `tags_all`

**Missing from extract** (3): `account_id`, `storage_lens_configuration`, `tags_all`

### `aws_s3tables_namespace`

**Source:** `crates/winterbaume-terraform/src/converters/s3tables.rs`

**Inject attributes** (8): `created_at`, `created_by`, `namespace`, `namespace_list`, `owner_account_id`, `region`, `table_bucket_arn`, `tags`

**Extract attributes** (8): `created_at`, `created_by`, `id`, `namespace`, `namespace_list`, `owner_account_id`, `table_bucket_arn`, `tags`

### `aws_s3tables_table`

**Source:** `crates/winterbaume-terraform/src/converters/s3tables.rs`

**Inject attributes** (15): `arn`, `created_at`, `created_by`, `format`, `maintenance_config`, `metadata_location`, `modified_at`, `modified_by`, `name`, `namespace`, `owner_account_id`, `region`, `table_bucket_arn`, `version_token`, `warehouse_location`

**Extract attributes** (14): `arn`, `created_at`, `created_by`, `format`, `id`, `metadata_location`, `modified_at`, `modified_by`, `name`, `namespace`, `owner_account_id`, `table_bucket_arn`, `version_token`, `warehouse_location`

**Missing from inject** (3): `encryption_configuration`, `maintenance_configuration`, `type`

**Missing from extract** (3): `encryption_configuration`, `maintenance_configuration`, `type`

### `aws_s3tables_table_bucket`

**Source:** `crates/winterbaume-terraform/src/converters/s3tables.rs`

**Inject attributes** (13): `arn`, `created_at`, `encryption_kms_key_arn`, `encryption_sse_algorithm`, `maintenance_config`, `metrics_config`, `name`, `owner_account_id`, `policy`, `region`, `replication_config`, `storage_class`, `tags`

**Extract attributes** (13): `arn`, `created_at`, `encryption_kms_key_arn`, `encryption_sse_algorithm`, `id`, `maintenance_config`, `metrics_config`, `name`, `owner_account_id`, `policy`, `replication_config`, `storage_class`, `tags`

**Missing from inject** (2): `encryption_configuration`, `maintenance_configuration`

**Missing from extract** (2): `encryption_configuration`, `maintenance_configuration`

### `aws_s3tables_table_bucket_policy`

**Source:** `crates/winterbaume-terraform/src/converters/s3tables.rs`

**Inject attributes** (0): (none)

**Extract attributes** (0): (none)

**Missing from inject** (2): `resource_policy`, `table_bucket_arn`

**Missing from extract** (2): `resource_policy`, `table_bucket_arn`

### `aws_s3tables_table_policy`

**Source:** `crates/winterbaume-terraform/src/converters/s3tables.rs`

**Inject attributes** (0): (none)

**Extract attributes** (0): (none)

**Missing from inject** (4): `name`, `namespace`, `resource_policy`, `table_bucket_arn`

**Missing from extract** (4): `name`, `namespace`, `resource_policy`, `table_bucket_arn`

### `aws_sagemaker_app`

**Source:** `crates/winterbaume-terraform/src/converters/sagemaker.rs`

**Inject attributes** (11): `app_name`, `app_type`, `arn`, `creation_time`, `domain_id`, `region`, `space_name`, `status`, `tags`, `tags_all`, `user_profile_name`

**Extract attributes** (11): `app_name`, `app_type`, `arn`, `creation_time`, `domain_id`, `id`, `space_name`, `status`, `tags`, `tags_all`, `user_profile_name`

**Missing from inject** (1): `resource_spec`

**Missing from extract** (1): `resource_spec`

### `aws_sagemaker_app_image_config`

**Source:** `crates/winterbaume-terraform/src/converters/sagemaker.rs`

**Inject attributes** (2): `app_image_config_name`, `arn`

**Extract attributes** (0): (none)

**Missing from inject** (5): `code_editor_app_image_config`, `jupyter_lab_image_config`, `kernel_gateway_image_config`, `tags`, `tags_all`

**Missing from extract** (7): `app_image_config_name`, `arn`, `code_editor_app_image_config`, `jupyter_lab_image_config`, `kernel_gateway_image_config`, `tags`, `tags_all`

### `aws_sagemaker_code_repository`

**Source:** `crates/winterbaume-terraform/src/converters/sagemaker.rs`

**Inject attributes** (2): `arn`, `code_repository_name`

**Extract attributes** (0): (none)

**Missing from inject** (3): `git_config`, `tags`, `tags_all`

**Missing from extract** (5): `arn`, `code_repository_name`, `git_config`, `tags`, `tags_all`

### `aws_sagemaker_data_quality_job_definition`

**Source:** `crates/winterbaume-terraform/src/converters/sagemaker.rs`

**Inject attributes** (7): `arn`, `creation_time`, `name`, `region`, `role_arn`, `tags`, `tags_all`

**Extract attributes** (7): `arn`, `creation_time`, `id`, `name`, `role_arn`, `tags`, `tags_all`

**Missing from inject** (7): `data_quality_app_specification`, `data_quality_baseline_config`, `data_quality_job_input`, `data_quality_job_output_config`, `job_resources`, `network_config`, `stopping_condition`

**Missing from extract** (7): `data_quality_app_specification`, `data_quality_baseline_config`, `data_quality_job_input`, `data_quality_job_output_config`, `job_resources`, `network_config`, `stopping_condition`

### `aws_sagemaker_device`

**Source:** `crates/winterbaume-terraform/src/converters/sagemaker.rs`

**Inject attributes** (11): `arn`, `created_at`, `description`, `global_network_id`, `model`, `serial_number`, `site_id`, `state`, `tags`, `type`, `vendor`

**Extract attributes** (0): (none)

**Missing from inject** (3): `agent_version`, `device`, `device_fleet_name`

**Missing from extract** (4): `agent_version`, `arn`, `device`, `device_fleet_name`

### `aws_sagemaker_device_fleet`

**Source:** `crates/winterbaume-terraform/src/converters/sagemaker.rs`

**Inject attributes** (4): `arn`, `description`, `device_fleet_name`, `role_arn`

**Extract attributes** (0): (none)

**Missing from inject** (5): `enable_iot_role_alias`, `iot_role_alias`, `output_config`, `tags`, `tags_all`

**Missing from extract** (9): `arn`, `description`, `device_fleet_name`, `enable_iot_role_alias`, `iot_role_alias`, `output_config`, `role_arn`, `tags`, `tags_all`

### `aws_sagemaker_domain`

**Source:** `crates/winterbaume-terraform/src/converters/sagemaker.rs`

**Inject attributes** (20): `app_network_access_type`, `arn`, `auth_mode`, `creation_time`, `default_space_settings`, `default_user_settings`, `domain_name`, `domain_settings`, `home_efs_file_system_id`, `kms_key_id`, `last_modified_time`, `region`, `retention_policy`, `security_group_ids`, `status`, `subnet_ids`, `tags`, `tags_all`, `url`, `vpc_id`

**Extract attributes** (20): `app_network_access_type`, `arn`, `auth_mode`, `creation_time`, `default_space_settings`, `default_user_settings`, `domain_name`, `domain_settings`, `home_efs_file_system_id`, `id`, `kms_key_id`, `last_modified_time`, `retention_policy`, `security_group_ids`, `status`, `subnet_ids`, `tags`, `tags_all`, `url`, `vpc_id`

**Missing from inject** (5): `app_security_group_management`, `security_group_id_for_domain_boundary`, `single_sign_on_application_arn`, `single_sign_on_managed_application_instance_id`, `tag_propagation`

**Missing from extract** (5): `app_security_group_management`, `security_group_id_for_domain_boundary`, `single_sign_on_application_arn`, `single_sign_on_managed_application_instance_id`, `tag_propagation`

### `aws_sagemaker_endpoint`

**Source:** `crates/winterbaume-terraform/src/converters/sagemaker.rs`

**Inject attributes** (9): `arn`, `creation_time`, `endpoint_config_name`, `endpoint_status`, `last_modified_time`, `name`, `region`, `tags`, `tags_all`

**Extract attributes** (8): `arn`, `creation_time`, `endpoint_config_name`, `endpoint_status`, `id`, `last_modified_time`, `name`, `tags`

**Missing from inject** (1): `deployment_config`

**Missing from extract** (2): `deployment_config`, `tags_all`

### `aws_sagemaker_endpoint_configuration`

**Source:** `crates/winterbaume-terraform/src/converters/sagemaker.rs`

**Inject attributes** (10): `arn`, `async_inference_config`, `creation_time`, `data_capture_config`, `kms_key_arn`, `name`, `production_variants`, `region`, `tags`, `tags_all`

**Extract attributes** (10): `arn`, `async_inference_config`, `creation_time`, `data_capture_config`, `id`, `kms_key_arn`, `name`, `production_variants`, `tags`, `tags_all`

**Missing from inject** (2): `name_prefix`, `shadow_production_variants`

**Missing from extract** (2): `name_prefix`, `shadow_production_variants`

### `aws_sagemaker_feature_group`

**Source:** `crates/winterbaume-terraform/src/converters/sagemaker.rs`

**Inject attributes** (8): `arn`, `creation_time`, `feature_group_name`, `feature_group_status`, `last_modified_time`, `region`, `tags`, `tags_all`

**Extract attributes** (8): `arn`, `creation_time`, `feature_group_name`, `feature_group_status`, `id`, `last_modified_time`, `tags`, `tags_all`

**Missing from inject** (8): `description`, `event_time_feature_name`, `feature_definition`, `offline_store_config`, `online_store_config`, `record_identifier_feature_name`, `role_arn`, `throughput_config`

**Missing from extract** (8): `description`, `event_time_feature_name`, `feature_definition`, `offline_store_config`, `online_store_config`, `record_identifier_feature_name`, `role_arn`, `throughput_config`

### `aws_sagemaker_flow_definition`

**Source:** `crates/winterbaume-terraform/src/converters/sagemaker.rs`

**Inject attributes** (3): `arn`, `flow_definition_name`, `role_arn`

**Extract attributes** (0): (none)

**Missing from inject** (6): `human_loop_activation_config`, `human_loop_config`, `human_loop_request_source`, `output_config`, `tags`, `tags_all`

**Missing from extract** (9): `arn`, `flow_definition_name`, `human_loop_activation_config`, `human_loop_config`, `human_loop_request_source`, `output_config`, `role_arn`, `tags`, `tags_all`

### `aws_sagemaker_hub`

**Source:** `crates/winterbaume-terraform/src/converters/sagemaker.rs`

**Inject attributes** (3): `arn`, `hub_description`, `hub_name`

**Extract attributes** (0): (none)

**Missing from inject** (5): `hub_display_name`, `hub_search_keywords`, `s3_storage_config`, `tags`, `tags_all`

**Missing from extract** (8): `arn`, `hub_description`, `hub_display_name`, `hub_name`, `hub_search_keywords`, `s3_storage_config`, `tags`, `tags_all`

### `aws_sagemaker_human_task_ui`

**Source:** `crates/winterbaume-terraform/src/converters/sagemaker.rs`

**Inject attributes** (2): `arn`, `human_task_ui_name`

**Extract attributes** (0): (none)

**Missing from inject** (3): `tags`, `tags_all`, `ui_template`

**Missing from extract** (5): `arn`, `human_task_ui_name`, `tags`, `tags_all`, `ui_template`

### `aws_sagemaker_image`

**Source:** `crates/winterbaume-terraform/src/converters/sagemaker.rs`

**Inject attributes** (5): `arn`, `description`, `display_name`, `image_name`, `role_arn`

**Extract attributes** (0): (none)

**Missing from inject** (2): `tags`, `tags_all`

**Missing from extract** (7): `arn`, `description`, `display_name`, `image_name`, `role_arn`, `tags`, `tags_all`

### `aws_sagemaker_image_version`

**Source:** `crates/winterbaume-terraform/src/converters/sagemaker.rs`

**Inject attributes** (4): `arn`, `base_image`, `image_name`, `version`

**Extract attributes** (0): (none)

**Missing from inject** (9): `container_image`, `horovod`, `image_arn`, `job_type`, `ml_framework`, `processor`, `programming_lang`, `release_notes`, `vendor_guidance`

**Missing from extract** (13): `arn`, `base_image`, `container_image`, `horovod`, `image_arn`, `image_name`, `job_type`, `ml_framework`, `processor`, `programming_lang`, `release_notes`, `vendor_guidance`, `version`

### `aws_sagemaker_mlflow_tracking_server`

**Source:** `crates/winterbaume-terraform/src/converters/sagemaker.rs`

**Inject attributes** (4): `arn`, `artifact_store_uri`, `role_arn`, `tracking_server_name`

**Extract attributes** (0): (none)

**Missing from inject** (7): `automatic_model_registration`, `mlflow_version`, `tags`, `tags_all`, `tracking_server_size`, `tracking_server_url`, `weekly_maintenance_window_start`

**Missing from extract** (11): `arn`, `artifact_store_uri`, `automatic_model_registration`, `mlflow_version`, `role_arn`, `tags`, `tags_all`, `tracking_server_name`, `tracking_server_size`, `tracking_server_url`, `weekly_maintenance_window_start`

### `aws_sagemaker_model`

**Source:** `crates/winterbaume-terraform/src/converters/sagemaker.rs`

**Inject attributes** (11): `arn`, `container`, `creation_time`, `execution_role_arn`, `inference_execution_config`, `name`, `primary_container`, `region`, `tags`, `tags_all`, `vpc_config`

**Extract attributes** (11): `arn`, `container`, `creation_time`, `execution_role_arn`, `id`, `inference_execution_config`, `name`, `primary_container`, `tags`, `tags_all`, `vpc_config`

**Missing from inject** (1): `enable_network_isolation`

**Missing from extract** (1): `enable_network_isolation`

### `aws_sagemaker_model_package_group`

**Source:** `crates/winterbaume-terraform/src/converters/sagemaker.rs`

**Inject attributes** (7): `arn`, `creation_time`, `model_package_group_description`, `model_package_group_name`, `region`, `tags`, `tags_all`

**Extract attributes** (7): `arn`, `creation_time`, `id`, `model_package_group_description`, `model_package_group_name`, `tags`, `tags_all`

### `aws_sagemaker_model_package_group_policy`

**Source:** `crates/winterbaume-terraform/src/converters/sagemaker.rs`

**Inject attributes** (2): `model_package_group_name`, `resource_policy`

**Extract attributes** (0): (none)

**Missing from extract** (2): `model_package_group_name`, `resource_policy`

### `aws_sagemaker_monitoring_schedule`

**Source:** `crates/winterbaume-terraform/src/converters/sagemaker.rs`

**Inject attributes** (2): `arn`, `name`

**Extract attributes** (0): (none)

**Missing from inject** (3): `monitoring_schedule_config`, `tags`, `tags_all`

**Missing from extract** (5): `arn`, `monitoring_schedule_config`, `name`, `tags`, `tags_all`

### `aws_sagemaker_notebook_instance`

**Source:** `crates/winterbaume-terraform/src/converters/sagemaker.rs`

**Inject attributes** (18): `arn`, `creation_time`, `direct_internet_access`, `instance_metadata_service_configuration`, `instance_type`, `kms_key_id`, `last_modified_time`, `lifecycle_config_name`, `name`, `notebook_instance_status`, `region`, `role_arn`, `root_access`, `status`, `subnet_id`, `tags_all`, `url`, `volume_size`

**Extract attributes** (15): `arn`, `creation_time`, `direct_internet_access`, `id`, `instance_metadata_service_configuration`, `instance_type`, `last_modified_time`, `name`, `network_interface_id`, `notebook_instance_status`, `role_arn`, `root_access`, `tags_all`, `url`, `volume_size`

**Missing from inject** (7): `accelerator_types`, `additional_code_repositories`, `default_code_repository`, `network_interface_id`, `platform_identifier`, `security_groups`, `tags`

**Missing from extract** (9): `accelerator_types`, `additional_code_repositories`, `default_code_repository`, `kms_key_id`, `lifecycle_config_name`, `platform_identifier`, `security_groups`, `subnet_id`, `tags`

### `aws_sagemaker_notebook_instance_lifecycle_configuration`

**Source:** `crates/winterbaume-terraform/src/converters/sagemaker.rs`

**Inject attributes** (5): `arn`, `creation_time`, `last_modified_time`, `name`, `region`

**Extract attributes** (6): `arn`, `content`, `id`, `name`, `on_create`, `on_start`

**Missing from inject** (4): `on_create`, `on_start`, `tags`, `tags_all`

**Missing from extract** (2): `tags`, `tags_all`

### `aws_sagemaker_pipeline`

**Source:** `crates/winterbaume-terraform/src/converters/sagemaker.rs`

**Inject attributes** (6): `description`, `name`, `region`, `tags`, `tags_all`, `unique_id`

**Extract attributes** (11): `arn`, `creation_time`, `id`, `last_modified_time`, `pipeline_definition`, `pipeline_description`, `pipeline_display_name`, `pipeline_name`, `role_arn`, `tags`, `tags_all`

**Missing from inject** (8): `arn`, `parallelism_configuration`, `pipeline_definition`, `pipeline_definition_s3_location`, `pipeline_description`, `pipeline_display_name`, `pipeline_name`, `role_arn`

**Missing from extract** (2): `parallelism_configuration`, `pipeline_definition_s3_location`

### `aws_sagemaker_project`

**Source:** `crates/winterbaume-terraform/src/converters/sagemaker.rs`

**Inject attributes** (3): `arn`, `project_description`, `project_name`

**Extract attributes** (0): (none)

**Missing from inject** (4): `project_id`, `service_catalog_provisioning_details`, `tags`, `tags_all`

**Missing from extract** (7): `arn`, `project_description`, `project_id`, `project_name`, `service_catalog_provisioning_details`, `tags`, `tags_all`

### `aws_sagemaker_servicecatalog_portfolio_status`

**Source:** `crates/winterbaume-terraform/src/converters/sagemaker.rs`

**Inject attributes** (1): `status`

**Extract attributes** (0): (none)

**Missing from extract** (1): `status`

### `aws_sagemaker_space`

**Source:** `crates/winterbaume-terraform/src/converters/sagemaker.rs`

**Inject attributes** (9): `arn`, `creation_time`, `domain_id`, `last_modified_time`, `region`, `space_name`, `status`, `tags`, `tags_all`

**Extract attributes** (9): `arn`, `creation_time`, `domain_id`, `id`, `last_modified_time`, `space_name`, `status`, `tags`, `tags_all`

**Missing from inject** (6): `home_efs_file_system_uid`, `ownership_settings`, `space_display_name`, `space_settings`, `space_sharing_settings`, `url`

**Missing from extract** (6): `home_efs_file_system_uid`, `ownership_settings`, `space_display_name`, `space_settings`, `space_sharing_settings`, `url`

### `aws_sagemaker_studio_lifecycle_config`

**Source:** `crates/winterbaume-terraform/src/converters/sagemaker.rs`

**Inject attributes** (4): `arn`, `studio_lifecycle_config_app_type`, `studio_lifecycle_config_content`, `studio_lifecycle_config_name`

**Extract attributes** (0): (none)

**Missing from inject** (2): `tags`, `tags_all`

**Missing from extract** (6): `arn`, `studio_lifecycle_config_app_type`, `studio_lifecycle_config_content`, `studio_lifecycle_config_name`, `tags`, `tags_all`

### `aws_sagemaker_user_profile`

**Source:** `crates/winterbaume-terraform/src/converters/sagemaker.rs`

**Inject attributes** (9): `arn`, `creation_time`, `domain_id`, `last_modified_time`, `region`, `status`, `tags`, `tags_all`, `user_profile_name`

**Extract attributes** (9): `arn`, `creation_time`, `domain_id`, `id`, `last_modified_time`, `status`, `tags`, `tags_all`, `user_profile_name`

**Missing from inject** (4): `home_efs_file_system_uid`, `single_sign_on_user_identifier`, `single_sign_on_user_value`, `user_settings`

**Missing from extract** (4): `home_efs_file_system_uid`, `single_sign_on_user_identifier`, `single_sign_on_user_value`, `user_settings`

### `aws_sagemaker_workforce`

**Source:** `crates/winterbaume-terraform/src/converters/sagemaker.rs`

**Inject attributes** (3): `arn`, `subdomain`, `workforce_name`

**Extract attributes** (0): (none)

**Missing from inject** (4): `cognito_config`, `oidc_config`, `source_ip_config`, `workforce_vpc_config`

**Missing from extract** (7): `arn`, `cognito_config`, `oidc_config`, `source_ip_config`, `subdomain`, `workforce_name`, `workforce_vpc_config`

### `aws_sagemaker_workteam`

**Source:** `crates/winterbaume-terraform/src/converters/sagemaker.rs`

**Inject attributes** (5): `arn`, `description`, `subdomain`, `workforce_name`, `workteam_name`

**Extract attributes** (0): (none)

**Missing from inject** (5): `member_definition`, `notification_configuration`, `tags`, `tags_all`, `worker_access_configuration`

**Missing from extract** (10): `arn`, `description`, `member_definition`, `notification_configuration`, `subdomain`, `tags`, `tags_all`, `worker_access_configuration`, `workforce_name`, `workteam_name`

### `aws_scheduler_schedule`

**Source:** `crates/winterbaume-terraform/src/converters/scheduler.rs`

**Inject attributes** (10): `arn`, `description`, `flexible_time_window`, `group_name`, `name`, `region`, `schedule_expression`, `state`, `tags`, `target`

**Extract attributes** (20): `arn`, `creation_date`, `description`, `flexible_time_window`, `group_name`, `id`, `kms_key_arn`, `last_modification_date`, `maximum_event_age_in_seconds`, `maximum_retry_attempts`, `maximum_window_in_minutes`, `mode`, `name`, `retry_policy`, `role_arn`, `schedule_expression`, `state`, `tags`, `tags_all`, `target`

**Missing from inject** (5): `end_date`, `kms_key_arn`, `name_prefix`, `schedule_expression_timezone`, `start_date`

**Missing from extract** (4): `end_date`, `name_prefix`, `schedule_expression_timezone`, `start_date`

### `aws_scheduler_schedule_group`

**Source:** `crates/winterbaume-terraform/src/converters/scheduler.rs`

**Inject attributes** (4): `arn`, `name`, `region`, `tags`

**Extract attributes** (6): `arn`, `id`, `name`, `state`, `tags`, `tags_all`

**Missing from inject** (6): `creation_date`, `last_modification_date`, `name_prefix`, `state`, `tags_all`, `timeouts`

**Missing from extract** (4): `creation_date`, `last_modification_date`, `name_prefix`, `timeouts`

### `aws_secretsmanager_secret`

**Source:** `crates/winterbaume-terraform/src/converters/secretsmanager.rs`

**Inject attributes** (6): `arn`, `description`, `name`, `policy`, `region`, `tags`

**Extract attributes** (7): `arn`, `description`, `id`, `name`, `name_prefix`, `tags`, `tags_all`

**Missing from inject** (6): `force_overwrite_replica_secret`, `kms_key_id`, `name_prefix`, `recovery_window_in_days`, `replica`, `tags_all`

**Missing from extract** (5): `force_overwrite_replica_secret`, `kms_key_id`, `policy`, `recovery_window_in_days`, `replica`

### `aws_secretsmanager_secret_version`

**Source:** `crates/winterbaume-terraform/src/converters/secretsmanager.rs`

**Inject attributes** (5): `region`, `secret_id`, `secret_string`, `version_id`, `version_stages`

**Extract attributes** (6): `has_secret_string_wo`, `id`, `secret_id`, `secret_string`, `version_id`, `version_stages`

**Missing from inject** (5): `arn`, `has_secret_string_wo`, `secret_binary`, `secret_string_wo`, `secret_string_wo_version`

**Missing from extract** (4): `arn`, `secret_binary`, `secret_string_wo`, `secret_string_wo_version`

### `aws_securityhub_account`

**Source:** `crates/winterbaume-terraform/src/converters/securityhub.rs`

**Inject attributes** (5): `arn`, `email`, `name`, `parent_id`, `region`

**Extract attributes** (6): `arn`, `auto_enable_controls`, `control_finding_generator`, `enable_default_standards`, `id`, `tags`

**Missing from inject** (3): `auto_enable_controls`, `control_finding_generator`, `enable_default_standards`

### `aws_securityhub_action_target`

**Source:** `crates/winterbaume-terraform/src/converters/securityhub.rs`

**Inject attributes** (4): `description`, `identifier`, `name`, `region`

**Extract attributes** (5): `arn`, `description`, `id`, `identifier`, `name`

**Missing from inject** (1): `arn`

### `aws_securityhub_automation_rule`

**Source:** `crates/winterbaume-terraform/src/converters/securityhub.rs`

**Inject attributes** (6): `description`, `is_terminal`, `region`, `rule_name`, `rule_order`, `rule_status`

**Extract attributes** (7): `arn`, `description`, `id`, `is_terminal`, `rule_name`, `rule_order`, `rule_status`

**Missing from inject** (5): `actions`, `arn`, `criteria`, `tags`, `tags_all`

**Missing from extract** (4): `actions`, `criteria`, `tags`, `tags_all`

### `aws_securityhub_configuration_policy`

**Source:** `crates/winterbaume-terraform/src/converters/securityhub.rs`

**Inject attributes** (3): `description`, `name`, `region`

**Extract attributes** (5): `arn`, `configuration_policy`, `description`, `id`, `name`

**Missing from inject** (2): `arn`, `configuration_policy`

### `aws_securityhub_configuration_policy_association`

**Source:** `crates/winterbaume-terraform/src/converters/securityhub.rs`

**Inject attributes** (3): `policy_id`, `region`, `target_id`

**Extract attributes** (3): `id`, `policy_id`, `target_id`

**Missing from inject** (1): `timeouts`

**Missing from extract** (1): `timeouts`

### `aws_securityhub_finding_aggregator`

**Source:** `crates/winterbaume-terraform/src/converters/securityhub.rs`

**Inject attributes** (2): `linking_mode`, `region`

**Extract attributes** (4): `arn`, `id`, `linking_mode`, `specified_regions`

**Missing from inject** (1): `specified_regions`

### `aws_securityhub_insight`

**Source:** `crates/winterbaume-terraform/src/converters/securityhub.rs`

**Inject attributes** (3): `group_by_attribute`, `name`, `region`

**Extract attributes** (5): `arn`, `filters`, `group_by_attribute`, `id`, `name`

**Missing from inject** (2): `arn`, `filters`

### `aws_securityhub_invite_accepter`

**Source:** `crates/winterbaume-terraform/src/converters/securityhub.rs`

**Inject attributes** (2): `master_id`, `region`

**Extract attributes** (0): (none)

**Missing from inject** (1): `invitation_id`

**Missing from extract** (2): `invitation_id`, `master_id`

### `aws_securityhub_member`

**Source:** `crates/winterbaume-terraform/src/converters/securityhub.rs`

**Inject attributes** (14): `account_id`, `administrator_account_id`, `arn`, `email`, `invitation_disable_email_notification`, `invitation_message`, `invite`, `invited_at`, `master_account_id`, `region`, `relationship_status`, `status`, `tags`, `updated_at`

**Extract attributes** (6): `account_id`, `email`, `id`, `invite`, `master_id`, `member_status`

**Missing from inject** (2): `master_id`, `member_status`

### `aws_securityhub_organization_admin_account`

**Source:** `crates/winterbaume-terraform/src/converters/securityhub.rs`

**Inject attributes** (2): `admin_account_id`, `region`

**Extract attributes** (2): `admin_account_id`, `id`

### `aws_securityhub_organization_configuration`

**Source:** `crates/winterbaume-terraform/src/converters/securityhub.rs`

**Inject attributes** (2): `auto_enable`, `region`

**Extract attributes** (3): `auto_enable`, `auto_enable_standards`, `id`

**Missing from inject** (3): `auto_enable_standards`, `organization_configuration`, `timeouts`

**Missing from extract** (2): `organization_configuration`, `timeouts`

### `aws_securityhub_product_subscription`

**Source:** `crates/winterbaume-terraform/src/converters/securityhub.rs`

**Inject attributes** (2): `product_arn`, `region`

**Extract attributes** (3): `arn`, `id`, `product_arn`

**Missing from inject** (1): `arn`

### `aws_securityhub_standards_control`

**Source:** `crates/winterbaume-terraform/src/converters/securityhub.rs`

**Inject attributes** (4): `control_status`, `disabled_reason`, `region`, `standards_control_arn`

**Extract attributes** (11): `control_id`, `control_status`, `control_status_updated_at`, `description`, `disabled_reason`, `id`, `related_requirements`, `remediation_url`, `severity_rating`, `standards_control_arn`, `title`

**Missing from inject** (7): `control_id`, `control_status_updated_at`, `description`, `related_requirements`, `remediation_url`, `severity_rating`, `title`

### `aws_securityhub_standards_control_association`

**Source:** `crates/winterbaume-terraform/src/converters/securityhub.rs`

**Inject attributes** (5): `association_status`, `region`, `security_control_id`, `standards_arn`, `updated_reason`

**Extract attributes** (5): `association_status`, `id`, `security_control_id`, `standards_arn`, `updated_reason`

### `aws_securityhub_standards_subscription`

**Source:** `crates/winterbaume-terraform/src/converters/securityhub.rs`

**Inject attributes** (2): `region`, `standards_arn`

**Extract attributes** (5): `id`, `standards_arn`, `standards_input`, `standards_status`, `standards_subscription_arn`

**Missing from inject** (1): `timeouts`

**Missing from extract** (1): `timeouts`

### `aws_servicecatalog_budget_resource_association`

**Source:** `crates/winterbaume-terraform/src/converters/servicecatalog.rs`

**Inject attributes** (0): (none)

**Extract attributes** (0): (none)

**Missing from inject** (3): `budget_name`, `resource_id`, `timeouts`

**Missing from extract** (3): `budget_name`, `resource_id`, `timeouts`

### `aws_servicecatalog_constraint`

**Source:** `crates/winterbaume-terraform/src/converters/servicecatalog.rs`

**Inject attributes** (0): (none)

**Extract attributes** (0): (none)

**Missing from inject** (9): `accept_language`, `description`, `owner`, `parameters`, `portfolio_id`, `product_id`, `status`, `timeouts`, `type`

**Missing from extract** (9): `accept_language`, `description`, `owner`, `parameters`, `portfolio_id`, `product_id`, `status`, `timeouts`, `type`

### `aws_servicecatalog_organizations_access`

**Source:** `crates/winterbaume-terraform/src/converters/servicecatalog.rs`

**Inject attributes** (0): (none)

**Extract attributes** (0): (none)

**Missing from inject** (2): `enabled`, `timeouts`

**Missing from extract** (2): `enabled`, `timeouts`

### `aws_servicecatalog_portfolio`

**Source:** `crates/winterbaume-terraform/src/converters/servicecatalog.rs`

**Inject attributes** (7): `arn`, `created_time`, `description`, `name`, `provider_name`, `region`, `tags`

**Extract attributes** (7): `arn`, `created_time`, `description`, `id`, `name`, `provider_name`, `tags`

**Missing from inject** (2): `tags_all`, `timeouts`

**Missing from extract** (2): `tags_all`, `timeouts`

### `aws_servicecatalog_portfolio_share`

**Source:** `crates/winterbaume-terraform/src/converters/servicecatalog.rs`

**Inject attributes** (0): (none)

**Extract attributes** (0): (none)

**Missing from inject** (9): `accept_language`, `accepted`, `portfolio_id`, `principal_id`, `share_principals`, `share_tag_options`, `timeouts`, `type`, `wait_for_acceptance`

**Missing from extract** (9): `accept_language`, `accepted`, `portfolio_id`, `principal_id`, `share_principals`, `share_tag_options`, `timeouts`, `type`, `wait_for_acceptance`

### `aws_servicecatalog_principal_portfolio_association`

**Source:** `crates/winterbaume-terraform/src/converters/servicecatalog.rs`

**Inject attributes** (0): (none)

**Extract attributes** (0): (none)

**Missing from inject** (5): `accept_language`, `portfolio_id`, `principal_arn`, `principal_type`, `timeouts`

**Missing from extract** (5): `accept_language`, `portfolio_id`, `principal_arn`, `principal_type`, `timeouts`

### `aws_servicecatalog_product`

**Source:** `crates/winterbaume-terraform/src/converters/servicecatalog.rs`

**Inject attributes** (12): `arn`, `created_time`, `description`, `distributor`, `name`, `owner`, `region`, `support_description`, `support_email`, `support_url`, `tags`, `type`

**Extract attributes** (12): `arn`, `created_time`, `description`, `distributor`, `id`, `name`, `owner`, `support_description`, `support_email`, `support_url`, `tags`, `type`

**Missing from inject** (6): `accept_language`, `has_default_path`, `provisioning_artifact_parameters`, `status`, `tags_all`, `timeouts`

**Missing from extract** (6): `accept_language`, `has_default_path`, `provisioning_artifact_parameters`, `status`, `tags_all`, `timeouts`

### `aws_servicecatalog_product_portfolio_association`

**Source:** `crates/winterbaume-terraform/src/converters/servicecatalog.rs`

**Inject attributes** (0): (none)

**Extract attributes** (0): (none)

**Missing from inject** (5): `accept_language`, `portfolio_id`, `product_id`, `source_portfolio_id`, `timeouts`

**Missing from extract** (5): `accept_language`, `portfolio_id`, `product_id`, `source_portfolio_id`, `timeouts`

### `aws_servicecatalog_provisioned_product`

**Source:** `crates/winterbaume-terraform/src/converters/servicecatalog.rs`

**Inject attributes** (0): (none)

**Extract attributes** (0): (none)

**Missing from inject** (27): `accept_language`, `arn`, `cloudwatch_dashboard_names`, `created_time`, `ignore_errors`, `last_provisioning_record_id`, `last_record_id`, `last_successful_provisioning_record_id`, `launch_role_arn`, `name`, `notification_arns`, `outputs`, `path_id`, `path_name`, `product_id`, `product_name`, `provisioning_artifact_id`, `provisioning_artifact_name`, `provisioning_parameters`, `retain_physical_resources`, `stack_set_provisioning_preferences`, `status`, `status_message`, `tags`, `tags_all`, `timeouts`, `type`

**Missing from extract** (27): `accept_language`, `arn`, `cloudwatch_dashboard_names`, `created_time`, `ignore_errors`, `last_provisioning_record_id`, `last_record_id`, `last_successful_provisioning_record_id`, `launch_role_arn`, `name`, `notification_arns`, `outputs`, `path_id`, `path_name`, `product_id`, `product_name`, `provisioning_artifact_id`, `provisioning_artifact_name`, `provisioning_parameters`, `retain_physical_resources`, `stack_set_provisioning_preferences`, `status`, `status_message`, `tags`, `tags_all`, `timeouts`, `type`

### `aws_servicecatalog_provisioning_artifact`

**Source:** `crates/winterbaume-terraform/src/converters/servicecatalog.rs`

**Inject attributes** (0): (none)

**Extract attributes** (0): (none)

**Missing from inject** (13): `accept_language`, `active`, `created_time`, `description`, `disable_template_validation`, `guidance`, `name`, `product_id`, `provisioning_artifact_id`, `template_physical_id`, `template_url`, `timeouts`, `type`

**Missing from extract** (13): `accept_language`, `active`, `created_time`, `description`, `disable_template_validation`, `guidance`, `name`, `product_id`, `provisioning_artifact_id`, `template_physical_id`, `template_url`, `timeouts`, `type`

### `aws_servicecatalog_service_action`

**Source:** `crates/winterbaume-terraform/src/converters/servicecatalog.rs`

**Inject attributes** (0): (none)

**Extract attributes** (0): (none)

**Missing from inject** (5): `accept_language`, `definition`, `description`, `name`, `timeouts`

**Missing from extract** (5): `accept_language`, `definition`, `description`, `name`, `timeouts`

### `aws_servicecatalog_tag_option`

**Source:** `crates/winterbaume-terraform/src/converters/servicecatalog.rs`

**Inject attributes** (0): (none)

**Extract attributes** (0): (none)

**Missing from inject** (5): `active`, `key`, `owner`, `timeouts`, `value`

**Missing from extract** (5): `active`, `key`, `owner`, `timeouts`, `value`

### `aws_servicecatalog_tag_option_resource_association`

**Source:** `crates/winterbaume-terraform/src/converters/servicecatalog.rs`

**Inject attributes** (0): (none)

**Extract attributes** (0): (none)

**Missing from inject** (7): `resource_arn`, `resource_created_time`, `resource_description`, `resource_id`, `resource_name`, `tag_option_id`, `timeouts`

**Missing from extract** (7): `resource_arn`, `resource_created_time`, `resource_description`, `resource_id`, `resource_name`, `tag_option_id`, `timeouts`

### `aws_servicecatalogappregistry_application`

**Source:** `crates/winterbaume-terraform/src/converters/servicecatalogappregistry.rs`

**Inject attributes** (7): `arn`, `date_created`, `date_updated`, `description`, `name`, `region`, `tags`

**Extract attributes** (7): `arn`, `creation_time`, `description`, `id`, `last_update_time`, `name`, `tags`

**Missing from inject** (2): `application_tag`, `tags_all`

**Missing from extract** (2): `application_tag`, `tags_all`

### `aws_service_discovery_http_namespace`

**Source:** `crates/winterbaume-terraform/src/converters/servicediscovery.rs`

**Inject attributes** (5): `arn`, `description`, `name`, `region`, `tags`

**Extract attributes** (6): `arn`, `description`, `id`, `name`, `tags`, `tags_all`

**Missing from inject** (2): `http_name`, `tags_all`

**Missing from extract** (1): `http_name`

### `aws_service_discovery_instance`

**Source:** `crates/winterbaume-terraform/src/converters/servicediscovery.rs`

**Inject attributes** (4): `attributes`, `instance_id`, `region`, `service_id`

**Extract attributes** (4): `attributes`, `id`, `instance_id`, `service_id`

### `aws_service_discovery_private_dns_namespace`

**Source:** `crates/winterbaume-terraform/src/converters/servicediscovery.rs`

**Inject attributes** (8): `arn`, `description`, `hosted_zone_id`, `name`, `region`, `soa_ttl`, `tags`, `vpc`

**Extract attributes** (8): `arn`, `description`, `hosted_zone_id`, `id`, `name`, `soa_ttl`, `tags`, `vpc`

**Missing from inject** (2): `hosted_zone`, `tags_all`

**Missing from extract** (2): `hosted_zone`, `tags_all`

### `aws_service_discovery_public_dns_namespace`

**Source:** `crates/winterbaume-terraform/src/converters/servicediscovery.rs`

**Inject attributes** (6): `arn`, `description`, `hosted_zone_id`, `name`, `region`, `tags`

**Extract attributes** (7): `arn`, `description`, `hosted_zone_id`, `id`, `name`, `tags`, `tags_all`

**Missing from inject** (2): `hosted_zone`, `tags_all`

**Missing from extract** (1): `hosted_zone`

### `aws_service_discovery_service`

**Source:** `crates/winterbaume-terraform/src/converters/servicediscovery.rs`

**Inject attributes** (10): `cluster`, `desired_count`, `dns_config`, `health_check_config`, `health_check_custom_config`, `launch_type`, `name`, `region`, `scheduling_strategy`, `task_definition`

**Extract attributes** (16): `arn`, `description`, `dns_config`, `dns_records`, `failure_threshold`, `health_check_config`, `health_check_custom_config`, `id`, `instance_count`, `name`, `namespace_id`, `resource_path`, `routing_policy`, `tags`, `ttl`, `type`

**Missing from inject** (7): `arn`, `description`, `force_destroy`, `namespace_id`, `tags`, `tags_all`, `type`

**Missing from extract** (2): `force_destroy`, `tags_all`

### `aws_servicequotas_service_quota`

**Source:** `crates/winterbaume-terraform/src/converters/servicequotas.rs`

**Inject attributes** (10): `adjustable`, `arn`, `description`, `global_quota`, `quota_code`, `quota_name`, `region`, `service_code`, `service_name`, `unit`

**Extract attributes** (11): `adjustable`, `arn`, `description`, `global_quota`, `id`, `quota_code`, `quota_name`, `service_code`, `service_name`, `unit`, `value`

**Missing from inject** (5): `default_value`, `request_id`, `request_status`, `usage_metric`, `value`

**Missing from extract** (4): `default_value`, `request_id`, `request_status`, `usage_metric`

### `aws_sesv2_account_suppression_attributes`

**Source:** `crates/winterbaume-terraform/src/converters/ses.rs`

**Inject attributes** (2): `region`, `suppressed_reasons`

**Extract attributes** (2): `id`, `suppressed_reasons`

### `aws_sesv2_account_vdm_attributes`

**Source:** `crates/winterbaume-terraform/src/converters/ses.rs`

**Inject attributes** (4): `dashboard_attributes`, `guardian_attributes`, `region`, `vdm_enabled`

**Extract attributes** (4): `dashboard_attributes`, `guardian_attributes`, `id`, `vdm_enabled`

### `aws_sesv2_configuration_set`

**Source:** `crates/winterbaume-terraform/src/converters/ses.rs`

**Inject attributes** (11): `arn`, `delivery_options`, `name`, `region`, `reputation_metrics_enabled`, `reputation_options`, `sending_enabled`, `sending_options`, `suppression_options`, `tracking_options`, `vdm_options`

**Extract attributes** (9): `configuration_set_name`, `delivery_options`, `id`, `reputation_options`, `sending_options`, `suppression_options`, `tags`, `tracking_options`, `vdm_options`

**Missing from inject** (3): `configuration_set_name`, `tags`, `tags_all`

**Missing from extract** (2): `arn`, `tags_all`

### `aws_sesv2_configuration_set_event_destination`

**Source:** `crates/winterbaume-terraform/src/converters/ses.rs`

**Inject attributes** (4): `configuration_set_name`, `event_destination`, `event_destination_name`, `region`

**Extract attributes** (6): `configuration_set_name`, `enabled`, `event_destination`, `event_destination_name`, `id`, `matching_event_types`

### `aws_sesv2_contact_list`

**Source:** `crates/winterbaume-terraform/src/converters/ses.rs`

**Inject attributes** (5): `contact_list_name`, `description`, `region`, `tags`, `tags_all`

**Extract attributes** (6): `contact_list_name`, `created_timestamp`, `description`, `id`, `last_updated_timestamp`, `tags`

**Missing from inject** (4): `arn`, `created_timestamp`, `last_updated_timestamp`, `topic`

**Missing from extract** (3): `arn`, `tags_all`, `topic`

### `aws_sesv2_dedicated_ip_assignment`

**Source:** `crates/winterbaume-terraform/src/converters/ses.rs`

**Inject attributes** (3): `destination_pool_name`, `ip`, `region`

**Extract attributes** (3): `destination_pool_name`, `id`, `ip`

**Missing from inject** (1): `timeouts`

**Missing from extract** (1): `timeouts`

### `aws_sesv2_dedicated_ip_pool`

**Source:** `crates/winterbaume-terraform/src/converters/ses.rs`

**Inject attributes** (4): `pool_name`, `region`, `scaling_mode`, `tags`

**Extract attributes** (4): `id`, `pool_name`, `scaling_mode`, `tags`

**Missing from inject** (2): `arn`, `tags_all`

**Missing from extract** (2): `arn`, `tags_all`

### `aws_sesv2_email_identity`

**Source:** `crates/winterbaume-terraform/src/converters/ses.rs`

**Inject attributes** (6): `arn`, `dkim_signing_attributes`, `email`, `region`, `tags`, `tags_all`

**Extract attributes** (10): `configuration_set_name`, `dkim_signing_attributes`, `domain_signing_selector`, `email_identity`, `id`, `identity_type`, `signing_enabled`, `signing_key_type`, `tags`, `verified`

**Missing from inject** (4): `configuration_set_name`, `email_identity`, `identity_type`, `verified_for_sending_status`

**Missing from extract** (3): `arn`, `tags_all`, `verified_for_sending_status`

### `aws_sesv2_email_identity_feedback_attributes`

**Source:** `crates/winterbaume-terraform/src/converters/ses.rs`

**Inject attributes** (3): `email_forwarding_enabled`, `email_identity`, `region`

**Extract attributes** (3): `email_forwarding_enabled`, `email_identity`, `id`

### `aws_sesv2_email_identity_mail_from_attributes`

**Source:** `crates/winterbaume-terraform/src/converters/ses.rs`

**Inject attributes** (4): `behavior_on_mx_failure`, `email_identity`, `mail_from_domain`, `region`

**Extract attributes** (4): `behavior_on_mx_failure`, `email_identity`, `id`, `mail_from_domain`

### `aws_sesv2_email_identity_policy`

**Source:** `crates/winterbaume-terraform/src/converters/ses.rs`

**Inject attributes** (4): `email_identity`, `policy`, `policy_name`, `region`

**Extract attributes** (4): `email_identity`, `id`, `policy`, `policy_name`

### `aws_ses_active_receipt_rule_set`

**Source:** `crates/winterbaume-terraform/src/converters/sesv1.rs`

**Inject attributes** (3): `arn`, `region`, `rule_set_name`

**Extract attributes** (3): `arn`, `id`, `rule_set_name`

### `aws_ses_configuration_set`

**Source:** `crates/winterbaume-terraform/src/converters/sesv1.rs`

**Inject attributes** (5): `arn`, `name`, `region`, `reputation_metrics_enabled`, `sending_enabled`

**Extract attributes** (5): `arn`, `id`, `name`, `reputation_metrics_enabled`, `sending_enabled`

**Missing from inject** (3): `delivery_options`, `last_fresh_start`, `tracking_options`

**Missing from extract** (3): `delivery_options`, `last_fresh_start`, `tracking_options`

### `aws_ses_domain_dkim`

**Source:** `crates/winterbaume-terraform/src/converters/sesv1.rs`

**Inject attributes** (2): `domain`, `region`

**Extract attributes** (3): `dkim_tokens`, `domain`, `id`

**Missing from inject** (1): `dkim_tokens`

### `aws_ses_domain_identity`

**Source:** `crates/winterbaume-terraform/src/converters/sesv1.rs`

**Inject attributes** (4): `arn`, `domain`, `region`, `verification_token`

**Extract attributes** (4): `arn`, `domain`, `id`, `verification_token`

### `aws_ses_domain_identity_verification`

**Source:** `crates/winterbaume-terraform/src/converters/sesv1.rs`

**Inject attributes** (3): `arn`, `domain`, `region`

**Extract attributes** (3): `arn`, `domain`, `id`

**Missing from inject** (1): `timeouts`

**Missing from extract** (1): `timeouts`

### `aws_ses_domain_mail_from`

**Source:** `crates/winterbaume-terraform/src/converters/sesv1.rs`

**Inject attributes** (4): `behavior_on_mx_failure`, `domain`, `mail_from_domain`, `region`

**Extract attributes** (4): `behavior_on_mx_failure`, `domain`, `id`, `mail_from_domain`

### `aws_ses_email_identity`

**Source:** `crates/winterbaume-terraform/src/converters/sesv1.rs`

**Inject attributes** (3): `arn`, `email`, `region`

**Extract attributes** (2): `email`, `id`

**Missing from extract** (1): `arn`

### `aws_ses_event_destination`

**Source:** `crates/winterbaume-terraform/src/converters/sesv1.rs`

**Inject attributes** (5): `arn`, `configuration_set_name`, `enabled`, `name`, `region`

**Extract attributes** (5): `arn`, `configuration_set_name`, `enabled`, `id`, `name`

**Missing from inject** (4): `cloudwatch_destination`, `kinesis_destination`, `matching_types`, `sns_destination`

**Missing from extract** (4): `cloudwatch_destination`, `kinesis_destination`, `matching_types`, `sns_destination`

### `aws_ses_identity_notification_topic`

**Source:** `crates/winterbaume-terraform/src/converters/sesv1.rs`

**Inject attributes** (5): `identity`, `include_original_headers`, `notification_type`, `region`, `topic_arn`

**Extract attributes** (5): `id`, `identity`, `include_original_headers`, `notification_type`, `topic_arn`

### `aws_ses_identity_policy`

**Source:** `crates/winterbaume-terraform/src/converters/sesv1.rs`

**Inject attributes** (0): (none)

**Extract attributes** (0): (none)

**Missing from inject** (3): `identity`, `name`, `policy`

**Missing from extract** (3): `identity`, `name`, `policy`

### `aws_ses_receipt_filter`

**Source:** `crates/winterbaume-terraform/src/converters/sesv1.rs`

**Inject attributes** (0): (none)

**Extract attributes** (0): (none)

**Missing from inject** (4): `arn`, `cidr`, `name`, `policy`

**Missing from extract** (4): `arn`, `cidr`, `name`, `policy`

### `aws_ses_receipt_rule`

**Source:** `crates/winterbaume-terraform/src/converters/sesv1.rs`

**Inject attributes** (7): `after`, `enabled`, `name`, `region`, `rule_set_name`, `scan_enabled`, `tls_policy`

**Extract attributes** (6): `enabled`, `id`, `name`, `rule_set_name`, `scan_enabled`, `tls_policy`

**Missing from inject** (9): `add_header_action`, `arn`, `bounce_action`, `lambda_action`, `recipients`, `s3_action`, `sns_action`, `stop_action`, `workmail_action`

**Missing from extract** (10): `add_header_action`, `after`, `arn`, `bounce_action`, `lambda_action`, `recipients`, `s3_action`, `sns_action`, `stop_action`, `workmail_action`

### `aws_ses_receipt_rule_set`

**Source:** `crates/winterbaume-terraform/src/converters/sesv1.rs`

**Inject attributes** (3): `arn`, `region`, `rule_set_name`

**Extract attributes** (3): `arn`, `id`, `rule_set_name`

### `aws_ses_template`

**Source:** `crates/winterbaume-terraform/src/converters/sesv1.rs`

**Inject attributes** (6): `arn`, `html`, `name`, `region`, `subject`, `text`

**Extract attributes** (6): `arn`, `html`, `id`, `name`, `subject`, `text`

### `aws_shield_application_layer_automatic_response`

**Source:** `crates/winterbaume-terraform/src/converters/shield.rs`

**Inject attributes** (2): `region`, `resource_arn`

**Extract attributes** (0): (none)

**Missing from inject** (2): `action`, `timeouts`

**Missing from extract** (3): `action`, `resource_arn`, `timeouts`

### `aws_shield_drt_access_log_bucket_association`

**Source:** `crates/winterbaume-terraform/src/converters/shield.rs`

**Inject attributes** (3): `log_bucket`, `region`, `role_arn_association_id`

**Extract attributes** (0): (none)

**Missing from inject** (1): `timeouts`

**Missing from extract** (3): `log_bucket`, `role_arn_association_id`, `timeouts`

### `aws_shield_drt_access_role_arn_association`

**Source:** `crates/winterbaume-terraform/src/converters/shield.rs`

**Inject attributes** (2): `region`, `role_arn`

**Extract attributes** (0): (none)

**Missing from inject** (1): `timeouts`

**Missing from extract** (2): `role_arn`, `timeouts`

### `aws_shield_proactive_engagement`

**Source:** `crates/winterbaume-terraform/src/converters/shield.rs`

**Inject attributes** (2): `enabled`, `region`

**Extract attributes** (0): (none)

**Missing from inject** (1): `emergency_contact`

**Missing from extract** (2): `emergency_contact`, `enabled`

### `aws_shield_protection`

**Source:** `crates/winterbaume-terraform/src/converters/shield.rs`

**Inject attributes** (5): `arn`, `name`, `region`, `resource_arn`, `tags`

**Extract attributes** (7): `arn`, `health_check_ids`, `id`, `name`, `protection_arn`, `resource_arn`, `tags`

**Missing from inject** (1): `tags_all`

**Missing from extract** (1): `tags_all`

### `aws_shield_protection_group`

**Source:** `crates/winterbaume-terraform/src/converters/shield.rs`

**Inject attributes** (6): `aggregation`, `pattern`, `protection_group_arn`, `protection_group_id`, `region`, `tags`

**Extract attributes** (0): (none)

**Missing from inject** (3): `members`, `resource_type`, `tags_all`

**Missing from extract** (8): `aggregation`, `members`, `pattern`, `protection_group_arn`, `protection_group_id`, `resource_type`, `tags`, `tags_all`

### `aws_shield_protection_health_check_association`

**Source:** `crates/winterbaume-terraform/src/converters/shield.rs`

**Inject attributes** (3): `health_check_arn`, `region`, `shield_protection_id`

**Extract attributes** (3): `health_check_arn`, `id`, `shield_protection_id`

### `aws_shield_subscription`

**Source:** `crates/winterbaume-terraform/src/converters/shield.rs`

**Inject attributes** (8): `arn`, `endpoint`, `filter_policy`, `protocol`, `raw_message_delivery`, `region`, `subscription_role_arn`, `topic_arn`

**Extract attributes** (6): `auto_renew`, `end_time`, `id`, `start_time`, `subscription_arn`, `time_commitment_in_seconds`

**Missing from inject** (2): `auto_renew`, `skip_destroy`

**Missing from extract** (1): `skip_destroy`

### `aws_signer_signing_profile`

**Source:** `crates/winterbaume-terraform/src/converters/signer.rs`

**Inject attributes** (8): `arn`, `name`, `platform_id`, `region`, `revision_id`, `status`, `version`, `version_arn`

**Extract attributes** (12): `arn`, `certificate_arn`, `id`, `name`, `platform_id`, `revision_id`, `signature_validity_period`, `signing_material`, `status`, `tags`, `version`, `version_arn`

**Missing from inject** (7): `name_prefix`, `platform_display_name`, `revocation_record`, `signature_validity_period`, `signing_material`, `tags`, `tags_all`

**Missing from extract** (4): `name_prefix`, `platform_display_name`, `revocation_record`, `tags_all`

### `aws_simpledb_domain`

**Source:** `crates/winterbaume-terraform/src/converters/simpledbv2.rs`

**Inject attributes** (12): `app_network_access_type`, `arn`, `auth_mode`, `creation_time`, `domain_name`, `home_efs_file_system_id`, `kms_key_id`, `last_modified_time`, `region`, `status`, `url`, `vpc_id`

**Extract attributes** (2): `id`, `name`

**Missing from inject** (1): `name`

### `aws_sns_platform_application`

**Source:** `crates/winterbaume-terraform/src/converters/sns.rs`

**Inject attributes** (6): `arn`, `name`, `platform`, `platform_credential`, `platform_principal`, `region`

**Extract attributes** (6): `arn`, `id`, `name`, `platform`, `platform_credential`, `platform_principal`

**Missing from inject** (9): `apple_platform_bundle_id`, `apple_platform_team_id`, `event_delivery_failure_topic_arn`, `event_endpoint_created_topic_arn`, `event_endpoint_deleted_topic_arn`, `event_endpoint_updated_topic_arn`, `failure_feedback_role_arn`, `success_feedback_role_arn`, `success_feedback_sample_rate`

**Missing from extract** (9): `apple_platform_bundle_id`, `apple_platform_team_id`, `event_delivery_failure_topic_arn`, `event_endpoint_created_topic_arn`, `event_endpoint_deleted_topic_arn`, `event_endpoint_updated_topic_arn`, `failure_feedback_role_arn`, `success_feedback_role_arn`, `success_feedback_sample_rate`

### `aws_sns_sms_preferences`

**Source:** `crates/winterbaume-terraform/src/converters/sns.rs`

**Inject attributes** (7): `default_sender_id`, `default_sms_type`, `delivery_status_iam_role_arn`, `delivery_status_success_sampling_rate`, `monthly_spend_limit`, `region`, `usage_report_s3_bucket`

**Extract attributes** (7): `default_sender_id`, `default_sms_type`, `delivery_status_iam_role_arn`, `delivery_status_success_sampling_rate`, `id`, `monthly_spend_limit`, `usage_report_s3_bucket`

### `aws_sns_topic`

**Source:** `crates/winterbaume-terraform/src/converters/sns.rs`

**Inject attributes** (25): `application_failure_feedback_role_arn`, `application_success_feedback_role_arn`, `application_success_feedback_sample_rate`, `arn`, `content_based_deduplication`, `delivery_policy`, `display_name`, `fifo_topic`, `firehose_failure_feedback_role_arn`, `firehose_success_feedback_role_arn`, `firehose_success_feedback_sample_rate`, `http_failure_feedback_role_arn`, `http_success_feedback_role_arn`, `http_success_feedback_sample_rate`, `kms_master_key_id`, `lambda_failure_feedback_role_arn`, `lambda_success_feedback_role_arn`, `lambda_success_feedback_sample_rate`, `name`, `policy`, `region`, `sqs_failure_feedback_role_arn`, `sqs_success_feedback_role_arn`, `sqs_success_feedback_sample_rate`, `tags`

**Extract attributes** (0): (none)

**Missing from inject** (8): `archive_policy`, `beginning_archive_time`, `fifo_throughput_scope`, `name_prefix`, `owner`, `signature_version`, `tags_all`, `tracing_config`

**Missing from extract** (32): `application_failure_feedback_role_arn`, `application_success_feedback_role_arn`, `application_success_feedback_sample_rate`, `archive_policy`, `arn`, `beginning_archive_time`, `content_based_deduplication`, `delivery_policy`, `display_name`, `fifo_throughput_scope`, `fifo_topic`, `firehose_failure_feedback_role_arn`, `firehose_success_feedback_role_arn`, `firehose_success_feedback_sample_rate`, `http_failure_feedback_role_arn`, `http_success_feedback_role_arn`, `http_success_feedback_sample_rate`, `kms_master_key_id`, `lambda_failure_feedback_role_arn`, `lambda_success_feedback_role_arn`, `lambda_success_feedback_sample_rate`, `name`, `name_prefix`, `owner`, `policy`, `signature_version`, `sqs_failure_feedback_role_arn`, `sqs_success_feedback_role_arn`, `sqs_success_feedback_sample_rate`, `tags`, `tags_all`, `tracing_config`

### `aws_sns_topic_data_protection_policy`

**Source:** `crates/winterbaume-terraform/src/converters/sns.rs`

**Inject attributes** (3): `arn`, `policy`, `region`

**Extract attributes** (0): (none)

**Missing from extract** (2): `arn`, `policy`

### `aws_sns_topic_policy`

**Source:** `crates/winterbaume-terraform/src/converters/sns.rs`

**Inject attributes** (3): `arn`, `policy`, `region`

**Extract attributes** (0): (none)

**Missing from inject** (1): `owner`

**Missing from extract** (3): `arn`, `owner`, `policy`

### `aws_sns_topic_subscription`

**Source:** `crates/winterbaume-terraform/src/converters/sns.rs`

**Inject attributes** (8): `arn`, `endpoint`, `filter_policy`, `protocol`, `raw_message_delivery`, `region`, `subscription_role_arn`, `topic_arn`

**Extract attributes** (12): `arn`, `confirmation_was_authenticated`, `confirmed`, `endpoint`, `filter_policy`, `id`, `owner`, `protocol`, `raw_message_delivery`, `subscription_role_arn`, `tags_all`, `topic_arn`

**Missing from inject** (9): `confirmation_timeout_in_minutes`, `confirmation_was_authenticated`, `delivery_policy`, `endpoint_auto_confirms`, `filter_policy_scope`, `owner_id`, `pending_confirmation`, `redrive_policy`, `replay_policy`

**Missing from extract** (8): `confirmation_timeout_in_minutes`, `delivery_policy`, `endpoint_auto_confirms`, `filter_policy_scope`, `owner_id`, `pending_confirmation`, `redrive_policy`, `replay_policy`

### `aws_sqs_queue`

**Source:** `crates/winterbaume-terraform/src/converters/sqs.rs`

**Inject attributes** (14): `arn`, `content_based_deduplication`, `delay_seconds`, `fifo_queue`, `max_message_size`, `message_retention_seconds`, `name`, `policy`, `receive_wait_time_seconds`, `redrive_policy`, `region`, `tags`, `url`, `visibility_timeout_seconds`

**Extract attributes** (13): `arn`, `content_based_deduplication`, `delay_seconds`, `fifo_queue`, `id`, `max_message_size`, `message_retention_seconds`, `name`, `receive_wait_time_seconds`, `tags`, `tags_all`, `url`, `visibility_timeout_seconds`

**Missing from inject** (9): `deduplication_scope`, `fifo_throughput_limit`, `kms_data_key_reuse_period_seconds`, `kms_master_key_id`, `name_prefix`, `redrive_allow_policy`, `sqs_managed_sse_enabled`, `tags_all`, `timeouts`

**Missing from extract** (10): `deduplication_scope`, `fifo_throughput_limit`, `kms_data_key_reuse_period_seconds`, `kms_master_key_id`, `name_prefix`, `policy`, `redrive_allow_policy`, `redrive_policy`, `sqs_managed_sse_enabled`, `timeouts`

### `aws_sqs_queue_policy`

**Source:** `crates/winterbaume-terraform/src/converters/sqs.rs`

**Inject attributes** (3): `policy`, `queue_url`, `region`

**Extract attributes** (0): (none)

**Missing from extract** (2): `policy`, `queue_url`

### `aws_sqs_queue_redrive_allow_policy`

**Source:** `crates/winterbaume-terraform/src/converters/sqs.rs`

**Inject attributes** (0): (none)

**Extract attributes** (0): (none)

**Missing from inject** (2): `queue_url`, `redrive_allow_policy`

**Missing from extract** (2): `queue_url`, `redrive_allow_policy`

### `aws_sqs_queue_redrive_policy`

**Source:** `crates/winterbaume-terraform/src/converters/sqs.rs`

**Inject attributes** (3): `queue_url`, `redrive_policy`, `region`

**Extract attributes** (0): (none)

**Missing from extract** (2): `queue_url`, `redrive_policy`

### `aws_ssm_activation`

**Source:** `crates/winterbaume-terraform/src/converters/ssm.rs`

**Inject attributes** (7): `activation_code`, `default_instance_name`, `description`, `expiration_date`, `iam_role`, `name`, `region`

**Extract attributes** (12): `activation_code`, `default_instance_name`, `description`, `expiration_date`, `expired`, `iam_role`, `id`, `name`, `registration_count`, `registration_limit`, `tags`, `tags_all`

**Missing from inject** (5): `expired`, `registration_count`, `registration_limit`, `tags`, `tags_all`

### `aws_ssm_association`

**Source:** `crates/winterbaume-terraform/src/converters/ssm.rs`

**Inject attributes** (6): `association_id`, `association_name`, `document_version`, `name`, `region`, `schedule_expression`

**Extract attributes** (12): `association_id`, `association_name`, `association_version`, `document_version`, `id`, `name`, `parameters`, `schedule_expression`, `status`, `tags`, `tags_all`, `targets`

**Missing from inject** (14): `apply_only_at_cron_interval`, `arn`, `automation_target_parameter_name`, `compliance_severity`, `instance_id`, `max_concurrency`, `max_errors`, `output_location`, `parameters`, `sync_compliance`, `tags`, `tags_all`, `targets`, `wait_for_success_timeout_seconds`

**Missing from extract** (10): `apply_only_at_cron_interval`, `arn`, `automation_target_parameter_name`, `compliance_severity`, `instance_id`, `max_concurrency`, `max_errors`, `output_location`, `sync_compliance`, `wait_for_success_timeout_seconds`

### `aws_ssm_default_patch_baseline`

**Source:** `crates/winterbaume-terraform/src/converters/ssm.rs`

**Inject attributes** (3): `baseline_id`, `operating_system`, `region`

**Extract attributes** (3): `baseline_id`, `id`, `operating_system`

### `aws_ssm_document`

**Source:** `crates/winterbaume-terraform/src/converters/ssm.rs`

**Inject attributes** (10): `content`, `default_version`, `document_format`, `document_type`, `latest_version`, `name`, `owner`, `region`, `status`, `version_name`

**Extract attributes** (13): `arn`, `content`, `created_date`, `default_version`, `document_format`, `document_type`, `id`, `latest_version`, `name`, `owner`, `status`, `tags`, `tags_all`

**Missing from inject** (14): `arn`, `attachments_source`, `created_date`, `description`, `document_version`, `hash`, `hash_type`, `parameter`, `permissions`, `platform_types`, `schema_version`, `tags`, `tags_all`, `target_type`

**Missing from extract** (11): `attachments_source`, `description`, `document_version`, `hash`, `hash_type`, `parameter`, `permissions`, `platform_types`, `schema_version`, `target_type`, `version_name`

### `aws_ssm_maintenance_window`

**Source:** `crates/winterbaume-terraform/src/converters/ssm.rs`

**Inject attributes** (11): `cutoff`, `description`, `duration`, `enabled`, `end_date`, `name`, `region`, `schedule`, `schedule_offset`, `schedule_timezone`, `start_date`

**Extract attributes** (8): `cutoff`, `duration`, `enabled`, `id`, `name`, `schedule`, `tags`, `tags_all`

**Missing from inject** (3): `allow_unassociated_targets`, `tags`, `tags_all`

**Missing from extract** (6): `allow_unassociated_targets`, `description`, `end_date`, `schedule_offset`, `schedule_timezone`, `start_date`

### `aws_ssm_maintenance_window_target`

**Source:** `crates/winterbaume-terraform/src/converters/ssm.rs`

**Inject attributes** (5): `description`, `name`, `owner_information`, `region`, `window_id`

**Extract attributes** (3): `id`, `targets`, `window_id`

**Missing from inject** (2): `resource_type`, `targets`

**Missing from extract** (4): `description`, `name`, `owner_information`, `resource_type`

### `aws_ssm_maintenance_window_task`

**Source:** `crates/winterbaume-terraform/src/converters/ssm.rs`

**Inject attributes** (11): `cutoff_behavior`, `description`, `max_concurrency`, `max_errors`, `name`, `priority`, `region`, `service_role_arn`, `task_arn`, `task_type`, `window_id`

**Extract attributes** (5): `id`, `targets`, `task_arn`, `task_type`, `window_id`

**Missing from inject** (4): `arn`, `targets`, `task_invocation_parameters`, `window_task_id`

**Missing from extract** (10): `arn`, `cutoff_behavior`, `description`, `max_concurrency`, `max_errors`, `name`, `priority`, `service_role_arn`, `task_invocation_parameters`, `window_task_id`

### `aws_ssm_parameter`

**Source:** `crates/winterbaume-terraform/src/converters/ssm.rs`

**Inject attributes** (7): `arn`, `data_type`, `name`, `region`, `tags`, `value`, `version`

**Extract attributes** (11): `arn`, `data_type`, `id`, `last_modified_date`, `name`, `tags`, `tags_all`, `tier`, `type`, `value`, `version`

**Missing from inject** (11): `allowed_pattern`, `description`, `has_value_wo`, `insecure_value`, `key_id`, `overwrite`, `tags_all`, `tier`, `type`, `value_wo`, `value_wo_version`

**Missing from extract** (8): `allowed_pattern`, `description`, `has_value_wo`, `insecure_value`, `key_id`, `overwrite`, `value_wo`, `value_wo_version`

### `aws_ssm_patch_baseline`

**Source:** `crates/winterbaume-terraform/src/converters/ssm.rs`

**Inject attributes** (7): `approved_patches_compliance_level`, `approved_patches_enable_non_security`, `description`, `name`, `operating_system`, `region`, `rejected_patches_action`

**Extract attributes** (7): `arn`, `description`, `id`, `name`, `operating_system`, `tags`, `tags_all`

**Missing from inject** (9): `approval_rule`, `approved_patches`, `arn`, `global_filter`, `json`, `rejected_patches`, `source`, `tags`, `tags_all`

**Missing from extract** (9): `approval_rule`, `approved_patches`, `approved_patches_compliance_level`, `approved_patches_enable_non_security`, `global_filter`, `json`, `rejected_patches`, `rejected_patches_action`, `source`

### `aws_ssm_patch_group`

**Source:** `crates/winterbaume-terraform/src/converters/ssm.rs`

**Inject attributes** (3): `baseline_id`, `patch_group`, `region`

**Extract attributes** (3): `baseline_id`, `id`, `patch_group`

### `aws_ssm_resource_data_sync`

**Source:** `crates/winterbaume-terraform/src/converters/ssm.rs`

**Inject attributes** (3): `name`, `region`, `s3_destination`

**Extract attributes** (6): `bucket_name`, `id`, `name`, `prefix`, `region`, `s3_destination`

### `aws_ssm_service_setting`

**Source:** `crates/winterbaume-terraform/src/converters/ssm.rs`

**Inject attributes** (3): `region`, `setting_id`, `setting_value`

**Extract attributes** (6): `arn`, `id`, `last_modified_date`, `setting_id`, `setting_value`, `status`

**Missing from inject** (2): `arn`, `status`

### `aws_ssoadmin_account_assignment`

**Source:** `crates/winterbaume-terraform/src/converters/ssoadmin.rs`

**Inject attributes** (6): `permission_set_arn`, `principal_id`, `principal_type`, `region`, `target_id`, `target_type`

**Extract attributes** (6): `id`, `permission_set_arn`, `principal_id`, `principal_type`, `target_id`, `target_type`

**Missing from inject** (2): `instance_arn`, `timeouts`

**Missing from extract** (2): `instance_arn`, `timeouts`

### `aws_ssoadmin_application`

**Source:** `crates/winterbaume-terraform/src/converters/ssoadmin.rs`

**Inject attributes** (6): `arn`, `date_created`, `date_updated`, `description`, `name`, `tags`

**Extract attributes** (0): (none)

**Missing from inject** (8): `application_account`, `application_arn`, `application_provider_arn`, `client_token`, `instance_arn`, `portal_options`, `status`, `tags_all`

**Missing from extract** (11): `application_account`, `application_arn`, `application_provider_arn`, `client_token`, `description`, `instance_arn`, `name`, `portal_options`, `status`, `tags`, `tags_all`

### `aws_ssoadmin_application_access_scope`

**Source:** `crates/winterbaume-terraform/src/converters/ssoadmin.rs`

**Inject attributes** (2): `application_arn`, `scope`

**Extract attributes** (0): (none)

**Missing from inject** (1): `authorized_targets`

**Missing from extract** (3): `application_arn`, `authorized_targets`, `scope`

### `aws_ssoadmin_application_assignment`

**Source:** `crates/winterbaume-terraform/src/converters/ssoadmin.rs`

**Inject attributes** (3): `application_arn`, `principal_id`, `principal_type`

**Extract attributes** (0): (none)

**Missing from extract** (3): `application_arn`, `principal_id`, `principal_type`

### `aws_ssoadmin_application_assignment_configuration`

**Source:** `crates/winterbaume-terraform/src/converters/ssoadmin.rs`

**Inject attributes** (2): `application_arn`, `assignment_required`

**Extract attributes** (0): (none)

**Missing from extract** (2): `application_arn`, `assignment_required`

### `aws_ssoadmin_customer_managed_policy_attachment`

**Source:** `crates/winterbaume-terraform/src/converters/ssoadmin.rs`

**Inject attributes** (3): `instance_arn`, `permission_set_arn`, `region`

**Extract attributes** (5): `customer_managed_policy_reference`, `id`, `name`, `path`, `permission_set_arn`

**Missing from inject** (2): `customer_managed_policy_reference`, `timeouts`

**Missing from extract** (2): `instance_arn`, `timeouts`

### `aws_ssoadmin_instance_access_control_attributes`

**Source:** `crates/winterbaume-terraform/src/converters/ssoadmin.rs`

**Inject attributes** (3): `instance_arn`, `status`, `status_reason`

**Extract attributes** (0): (none)

**Missing from inject** (1): `attribute`

**Missing from extract** (4): `attribute`, `instance_arn`, `status`, `status_reason`

### `aws_ssoadmin_managed_policy_attachment`

**Source:** `crates/winterbaume-terraform/src/converters/ssoadmin.rs`

**Inject attributes** (5): `instance_arn`, `managed_policy_arn`, `managed_policy_name`, `permission_set_arn`, `region`

**Extract attributes** (4): `id`, `managed_policy_arn`, `managed_policy_name`, `permission_set_arn`

**Missing from inject** (1): `timeouts`

**Missing from extract** (2): `instance_arn`, `timeouts`

### `aws_ssoadmin_permission_set`

**Source:** `crates/winterbaume-terraform/src/converters/ssoadmin.rs`

**Inject attributes** (7): `arn`, `description`, `name`, `region`, `relay_state`, `session_duration`, `tags`

**Extract attributes** (7): `arn`, `description`, `id`, `name`, `relay_state`, `session_duration`, `tags`

**Missing from inject** (4): `created_date`, `instance_arn`, `tags_all`, `timeouts`

**Missing from extract** (4): `created_date`, `instance_arn`, `tags_all`, `timeouts`

### `aws_ssoadmin_permission_set_inline_policy`

**Source:** `crates/winterbaume-terraform/src/converters/ssoadmin.rs`

**Inject attributes** (4): `inline_policy`, `instance_arn`, `permission_set_arn`, `region`

**Extract attributes** (3): `id`, `inline_policy`, `permission_set_arn`

**Missing from inject** (1): `timeouts`

**Missing from extract** (2): `instance_arn`, `timeouts`

### `aws_ssoadmin_permissions_boundary_attachment`

**Source:** `crates/winterbaume-terraform/src/converters/ssoadmin.rs`

**Inject attributes** (2): `instance_arn`, `permission_set_arn`

**Extract attributes** (0): (none)

**Missing from inject** (2): `permissions_boundary`, `timeouts`

**Missing from extract** (4): `instance_arn`, `permission_set_arn`, `permissions_boundary`, `timeouts`

### `aws_ssoadmin_trusted_token_issuer`

**Source:** `crates/winterbaume-terraform/src/converters/ssoadmin.rs`

**Inject attributes** (6): `arn`, `client_token`, `instance_arn`, `name`, `tags`, `trusted_token_issuer_type`

**Extract attributes** (0): (none)

**Missing from inject** (2): `tags_all`, `trusted_token_issuer_configuration`

**Missing from extract** (8): `arn`, `client_token`, `instance_arn`, `name`, `tags`, `tags_all`, `trusted_token_issuer_configuration`, `trusted_token_issuer_type`

### `aws_sfn_state_machine`

**Source:** `crates/winterbaume-terraform/src/converters/stepfunctions.rs`

**Inject attributes** (10): `arn`, `definition`, `encryption_configuration`, `logging_configuration`, `name`, `region`, `role_arn`, `tags`, `tags_all`, `tracing_configuration`

**Extract attributes** (17): `arn`, `creation_date`, `definition`, `enabled`, `id`, `include_execution_data`, `kms_data_key_reuse_period_seconds`, `kms_key_id`, `level`, `log_destination`, `name`, `revision_id`, `role_arn`, `status`, `tags`, `tags_all`, `type`

**Missing from inject** (10): `creation_date`, `description`, `name_prefix`, `publish`, `revision_id`, `state_machine_version_arn`, `status`, `timeouts`, `type`, `version_description`

**Missing from extract** (9): `description`, `encryption_configuration`, `logging_configuration`, `name_prefix`, `publish`, `state_machine_version_arn`, `timeouts`, `tracing_configuration`, `version_description`

### `aws_swf_domain`

**Source:** `crates/winterbaume-terraform/src/converters/swf.rs`

**Inject attributes** (12): `app_network_access_type`, `arn`, `auth_mode`, `creation_time`, `domain_name`, `home_efs_file_system_id`, `kms_key_id`, `last_modified_time`, `region`, `status`, `url`, `vpc_id`

**Extract attributes** (5): `arn`, `description`, `id`, `name`, `workflow_execution_retention_period_in_days`

**Missing from inject** (6): `description`, `name`, `name_prefix`, `tags`, `tags_all`, `workflow_execution_retention_period_in_days`

**Missing from extract** (3): `name_prefix`, `tags`, `tags_all`

### `aws_synthetics_canary`

**Source:** `crates/winterbaume-terraform/src/converters/synthetics.rs`

**Inject attributes** (22): `arn`, `artifact_config`, `artifact_s3_location`, `created_at`, `execution_role_arn`, `failure_retention_period`, `handler`, `last_modified`, `name`, `region`, `run_config`, `runtime_version`, `s3_encryption_mode`, `schedule`, `schedule_duration_in_seconds`, `schedule_expression`, `status`, `status_state_reason`, `status_state_reason_code`, `success_retention_period`, `tags`, `vpc_config`

**Extract attributes** (26): `arn`, `artifact_config`, `artifact_s3_location`, `created_at`, `duration_in_seconds`, `execution_role_arn`, `expression`, `failure_retention_period`, `handler`, `id`, `last_modified`, `name`, `run_config`, `runtime_version`, `s3_encryption_mode`, `schedule`, `schedule_duration_in_seconds`, `schedule_expression`, `source_location_arn`, `status`, `status_state_reason`, `status_state_reason_code`, `success_retention_period`, `tags`, `tags_all`, `vpc_config`

**Missing from inject** (10): `delete_lambda`, `engine_arn`, `s3_bucket`, `s3_key`, `s3_version`, `source_location_arn`, `start_canary`, `tags_all`, `timeline`, `zip_file`

**Missing from extract** (8): `delete_lambda`, `engine_arn`, `s3_bucket`, `s3_key`, `s3_version`, `start_canary`, `timeline`, `zip_file`

### `aws_timestreaminfluxdb_db_instance`

**Source:** `crates/winterbaume-terraform/src/converters/timestreaminfluxdb.rs`

**Inject attributes** (17): `allocated_storage`, `arn`, `auto_minor_version_upgrade`, `availability_zone`, `cluster_identifier`, `endpoint`, `engine`, `engine_version`, `identifier`, `instance_class`, `kms_key_arn`, `neptune_parameter_group_name`, `neptune_subnet_group_name`, `port`, `publicly_accessible`, `region`, `storage_encrypted`

**Extract attributes** (17): `allocated_storage`, `arn`, `availability_zone`, `db_instance_type`, `db_parameter_group_identifier`, `db_storage_type`, `deployment_type`, `endpoint`, `id`, `name`, `port`, `publicly_accessible`, `tags`, `tags_all`, `username`, `vpc_security_group_ids`, `vpc_subnet_ids`

**Missing from inject** (18): `bucket`, `db_instance_type`, `db_parameter_group_identifier`, `db_storage_type`, `deployment_type`, `influx_auth_parameters_secret_arn`, `log_delivery_configuration`, `name`, `network_type`, `organization`, `password`, `secondary_availability_zone`, `tags`, `tags_all`, `timeouts`, `username`, `vpc_security_group_ids`, `vpc_subnet_ids`

**Missing from extract** (8): `bucket`, `influx_auth_parameters_secret_arn`, `log_delivery_configuration`, `network_type`, `organization`, `password`, `secondary_availability_zone`, `timeouts`

### `aws_timestreamquery_scheduled_query`

**Source:** `crates/winterbaume-terraform/src/converters/timestreamquery.rs`

**Inject attributes** (18): `arn`, `creation_time`, `error_report_configuration`, `last_run_status`, `name`, `notification_configuration`, `notification_topic_arn`, `query_string`, `region`, `role_arn`, `s3_error_report_bucket`, `schedule_configuration`, `schedule_expression`, `state`, `tags`, `target_configuration`, `target_database`, `target_table`

**Extract attributes** (22): `arn`, `creation_time`, `error_report_configuration`, `id`, `last_run_status`, `last_run_summary`, `name`, `next_invocation_time`, `notification_configuration`, `notification_topic_arn`, `query_string`, `recently_failed_runs`, `role_arn`, `s3_error_report_bucket`, `schedule_configuration`, `schedule_expression`, `state`, `tags`, `tags_all`, `target_configuration`, `target_database`, `target_table`

**Missing from inject** (8): `execution_role_arn`, `kms_key_id`, `last_run_summary`, `next_invocation_time`, `previous_invocation_time`, `recently_failed_runs`, `tags_all`, `timeouts`

**Missing from extract** (4): `execution_role_arn`, `kms_key_id`, `previous_invocation_time`, `timeouts`

### `aws_timestreamwrite_database`

**Source:** `crates/winterbaume-terraform/src/converters/timestreamwrite.rs`

**Inject attributes** (6): `arn`, `database_name`, `kms_key_id`, `region`, `table_count`, `tags`

**Extract attributes** (6): `arn`, `database_name`, `id`, `kms_key_id`, `table_count`, `tags`

**Missing from inject** (1): `tags_all`

**Missing from extract** (1): `tags_all`

### `aws_timestreamwrite_table`

**Source:** `crates/winterbaume-terraform/src/converters/timestreamwrite.rs`

**Inject attributes** (7): `arn`, `database_name`, `magnetic_store_write_properties`, `region`, `retention_properties`, `table_name`, `tags`

**Extract attributes** (10): `arn`, `database_name`, `enable_magnetic_store_writes`, `id`, `magnetic_store_retention_period_in_days`, `magnetic_store_write_properties`, `memory_store_retention_period_in_hours`, `retention_properties`, `table_name`, `tags`

**Missing from inject** (2): `schema`, `tags_all`

**Missing from extract** (2): `schema`, `tags_all`

### `aws_transcribe_language_model`

**Source:** `crates/winterbaume-terraform/src/converters/transcribe.rs`

**Inject attributes** (8): `base_model_name`, `failure_reason`, `input_data_config`, `language_code`, `last_modified_time`, `model_name`, `region`, `tags`

**Extract attributes** (11): `arn`, `base_model_name`, `data_access_role_arn`, `id`, `input_data_config`, `language_code`, `model_name`, `s3_uri`, `tags`, `tags_all`, `tuning_data_s3_uri`

**Missing from inject** (3): `arn`, `tags_all`, `timeouts`

**Missing from extract** (1): `timeouts`

### `aws_transcribe_vocabulary`

**Source:** `crates/winterbaume-terraform/src/converters/transcribe.rs`

**Inject attributes** (9): `download_uri`, `failure_reason`, `language_code`, `last_modified_time`, `phrases`, `region`, `tags`, `vocabulary_file_uri`, `vocabulary_name`

**Extract attributes** (9): `download_uri`, `failure_reason`, `id`, `language_code`, `last_modified_time`, `phrases`, `vocabulary_file_uri`, `vocabulary_name`, `vocabulary_state`

**Missing from inject** (3): `arn`, `tags_all`, `timeouts`

**Missing from extract** (4): `arn`, `tags`, `tags_all`, `timeouts`

### `aws_transfer_access`

**Source:** `crates/winterbaume-terraform/src/converters/transfer.rs`

**Inject attributes** (8): `external_id`, `home_directory`, `home_directory_mappings`, `home_directory_type`, `posix_profile`, `region`, `role`, `server_id`

**Extract attributes** (0): (none)

**Missing from inject** (1): `policy`

**Missing from extract** (8): `external_id`, `home_directory`, `home_directory_mappings`, `home_directory_type`, `policy`, `posix_profile`, `role`, `server_id`

### `aws_transfer_agreement`

**Source:** `crates/winterbaume-terraform/src/converters/transfer.rs`

**Inject attributes** (10): `access_role`, `agreement_id`, `arn`, `base_directory`, `description`, `local_profile_id`, `partner_profile_id`, `region`, `server_id`, `status`

**Extract attributes** (12): `access_role`, `agreement_id`, `arn`, `base_directory`, `description`, `id`, `local_profile_id`, `partner_profile_id`, `server_id`, `status`, `tags`, `tags_all`

**Missing from inject** (2): `tags`, `tags_all`

### `aws_transfer_certificate`

**Source:** `crates/winterbaume-terraform/src/converters/transfer.rs`

**Inject attributes** (12): `active_date`, `arn`, `certificate`, `certificate_chain`, `certificate_id`, `certificate_type`, `description`, `inactive_date`, `private_key`, `region`, `status`, `usage`

**Extract attributes** (14): `active_date`, `arn`, `certificate`, `certificate_chain`, `certificate_id`, `certificate_type`, `description`, `id`, `inactive_date`, `private_key`, `status`, `tags`, `tags_all`, `usage`

**Missing from inject** (2): `tags`, `tags_all`

### `aws_transfer_connector`

**Source:** `crates/winterbaume-terraform/src/converters/transfer.rs`

**Inject attributes** (9): `access_role`, `arn`, `as2_config`, `connector_id`, `logging_role`, `region`, `security_policy_name`, `sftp_config`, `url`

**Extract attributes** (10): `access_role`, `arn`, `as2_config`, `connector_id`, `id`, `logging_role`, `sftp_config`, `tags`, `tags_all`, `url`

**Missing from inject** (2): `tags`, `tags_all`

**Missing from extract** (1): `security_policy_name`

### `aws_transfer_profile`

**Source:** `crates/winterbaume-terraform/src/converters/transfer.rs`

**Inject attributes** (6): `arn`, `as2_id`, `certificate_ids`, `profile_id`, `profile_type`, `region`

**Extract attributes** (8): `arn`, `as2_id`, `certificate_ids`, `id`, `profile_id`, `profile_type`, `tags`, `tags_all`

**Missing from inject** (2): `tags`, `tags_all`

### `aws_transfer_server`

**Source:** `crates/winterbaume-terraform/src/converters/transfer.rs`

**Inject attributes** (19): `arn`, `certificate`, `domain`, `endpoint_details`, `endpoint_type`, `force_destroy`, `host_key`, `identity_provider_type`, `logging_role`, `post_authentication_login_banner`, `pre_authentication_login_banner`, `protocol_details`, `protocols`, `region`, `s3_storage_options`, `security_policy_name`, `structured_log_destinations`, `tags_all`, `workflow_details`

**Extract attributes** (18): `arn`, `certificate`, `domain`, `endpoint`, `endpoint_details`, `endpoint_type`, `host_key_fingerprint`, `id`, `identity_provider_type`, `logging_role`, `protocol_details`, `protocols`, `s3_storage_options`, `security_policy_name`, `sftp_authentication_methods`, `tags`, `tags_all`, `workflow_details`

**Missing from inject** (8): `directory_id`, `endpoint`, `function`, `host_key_fingerprint`, `invocation_role`, `sftp_authentication_methods`, `tags`, `url`

**Missing from extract** (9): `directory_id`, `force_destroy`, `function`, `host_key`, `invocation_role`, `post_authentication_login_banner`, `pre_authentication_login_banner`, `structured_log_destinations`, `url`

### `aws_transfer_ssh_key`

**Source:** `crates/winterbaume-terraform/src/converters/transfer.rs`

**Inject attributes** (5): `body`, `region`, `server_id`, `ssh_public_key_id`, `user_name`

**Extract attributes** (0): (none)

**Missing from inject** (1): `ssh_key_id`

**Missing from extract** (4): `body`, `server_id`, `ssh_key_id`, `user_name`

### `aws_transfer_tag`

**Source:** `crates/winterbaume-terraform/src/converters/transfer.rs`

**Inject attributes** (4): `key`, `region`, `resource_arn`, `value`

**Extract attributes** (0): (none)

**Missing from extract** (3): `key`, `resource_arn`, `value`

### `aws_transfer_user`

**Source:** `crates/winterbaume-terraform/src/converters/transfer.rs`

**Inject attributes** (10): `arn`, `home_directory`, `home_directory_mappings`, `home_directory_type`, `policy`, `posix_profile`, `region`, `role`, `server_id`, `user_name`

**Extract attributes** (11): `arn`, `home_directory`, `home_directory_mappings`, `home_directory_type`, `id`, `posix_profile`, `role`, `server_id`, `tags`, `tags_all`, `user_name`

**Missing from inject** (3): `tags`, `tags_all`, `timeouts`

**Missing from extract** (2): `policy`, `timeouts`

### `aws_transfer_workflow`

**Source:** `crates/winterbaume-terraform/src/converters/transfer.rs`

**Inject attributes** (6): `description`, `max_concurrent_runs`, `name`, `on_exception_steps`, `region`, `steps`

**Extract attributes** (7): `arn`, `description`, `id`, `on_exception_steps`, `steps`, `tags`, `tags_all`

**Missing from inject** (3): `arn`, `tags`, `tags_all`

### `aws_vpclattice_access_log_subscription`

**Source:** `crates/winterbaume-terraform/src/converters/vpclattice.rs`

**Inject attributes** (7): `arn`, `destination_arn`, `region`, `resource_arn`, `resource_identifier`, `service_network_log_type`, `tags`

**Extract attributes** (6): `arn`, `destination_arn`, `id`, `resource_arn`, `resource_identifier`, `tags`

**Missing from inject** (1): `tags_all`

**Missing from extract** (2): `service_network_log_type`, `tags_all`

### `aws_vpclattice_auth_policy`

**Source:** `crates/winterbaume-terraform/src/converters/vpclattice.rs`

**Inject attributes** (4): `policy`, `region`, `resource_identifier`, `state`

**Extract attributes** (3): `policy`, `resource_identifier`, `state`

**Missing from inject** (1): `timeouts`

**Missing from extract** (1): `timeouts`

### `aws_vpclattice_listener`

**Source:** `crates/winterbaume-terraform/src/converters/vpclattice.rs`

**Inject attributes** (9): `arn`, `default_action`, `name`, `port`, `protocol`, `region`, `service_arn`, `service_identifier`, `tags`

**Extract attributes** (17): `arn`, `created_at`, `default_action`, `fixed_response`, `forward`, `id`, `last_updated_at`, `name`, `port`, `protocol`, `service_arn`, `service_identifier`, `status_code`, `tags`, `target_group_identifier`, `target_groups`, `weight`

**Missing from inject** (5): `created_at`, `last_updated_at`, `listener_id`, `tags_all`, `timeouts`

**Missing from extract** (3): `listener_id`, `tags_all`, `timeouts`

### `aws_vpclattice_listener_rule`

**Source:** `crates/winterbaume-terraform/src/converters/vpclattice.rs`

**Inject attributes** (8): `arn`, `listener_identifier`, `name`, `priority`, `region`, `rule_id`, `service_identifier`, `tags`

**Extract attributes** (7): `arn`, `id`, `listener_identifier`, `name`, `priority`, `rule_id`, `service_identifier`

**Missing from inject** (4): `action`, `match`, `tags_all`, `timeouts`

**Missing from extract** (5): `action`, `match`, `tags`, `tags_all`, `timeouts`

### `aws_vpclattice_resource_configuration`

**Source:** `crates/winterbaume-terraform/src/converters/vpclattice.rs`

**Inject attributes** (8): `arn`, `name`, `port_ranges`, `protocol`, `region`, `resource_gateway_identifier`, `tags`, `type`

**Extract attributes** (8): `arn`, `id`, `name`, `port_ranges`, `protocol`, `resource_gateway_identifier`, `tags`, `type`

**Missing from inject** (5): `allow_association_to_shareable_service_network`, `resource_configuration_definition`, `resource_configuration_group_id`, `tags_all`, `timeouts`

**Missing from extract** (5): `allow_association_to_shareable_service_network`, `resource_configuration_definition`, `resource_configuration_group_id`, `tags_all`, `timeouts`

### `aws_vpclattice_resource_gateway`

**Source:** `crates/winterbaume-terraform/src/converters/vpclattice.rs`

**Inject attributes** (9): `arn`, `ip_address_type`, `name`, `region`, `security_group_ids`, `status`, `subnet_ids`, `tags`, `vpc_id`

**Extract attributes** (9): `arn`, `id`, `ip_address_type`, `name`, `security_group_ids`, `status`, `subnet_ids`, `tags`, `vpc_id`

**Missing from inject** (2): `tags_all`, `timeouts`

**Missing from extract** (2): `tags_all`, `timeouts`

### `aws_vpclattice_resource_policy`

**Source:** `crates/winterbaume-terraform/src/converters/vpclattice.rs`

**Inject attributes** (2): `policy`, `region`

**Extract attributes** (2): `policy`, `resource_arn`

**Missing from inject** (1): `resource_arn`

### `aws_vpclattice_service`

**Source:** `crates/winterbaume-terraform/src/converters/vpclattice.rs`

**Inject attributes** (6): `arn`, `auth_type`, `name`, `region`, `status`, `tags`

**Extract attributes** (8): `arn`, `auth_type`, `created_at`, `id`, `last_updated_at`, `name`, `status`, `tags`

**Missing from inject** (5): `certificate_arn`, `custom_domain_name`, `dns_entry`, `tags_all`, `timeouts`

**Missing from extract** (5): `certificate_arn`, `custom_domain_name`, `dns_entry`, `tags_all`, `timeouts`

### `aws_vpclattice_service_network`

**Source:** `crates/winterbaume-terraform/src/converters/vpclattice.rs`

**Inject attributes** (5): `arn`, `auth_type`, `name`, `region`, `tags`

**Extract attributes** (9): `arn`, `auth_type`, `created_at`, `id`, `last_updated_at`, `name`, `number_of_associated_services`, `number_of_associated_v_p_cs`, `tags`

**Missing from inject** (1): `tags_all`

**Missing from extract** (1): `tags_all`

### `aws_vpclattice_service_network_resource_association`

**Source:** `crates/winterbaume-terraform/src/converters/vpclattice.rs`

**Inject attributes** (5): `arn`, `region`, `resource_configuration_identifier`, `service_network_identifier`, `tags`

**Extract attributes** (6): `arn`, `id`, `resource_configuration_identifier`, `service_network_identifier`, `status`, `tags`

**Missing from inject** (3): `dns_entry`, `tags_all`, `timeouts`

**Missing from extract** (3): `dns_entry`, `tags_all`, `timeouts`

### `aws_vpclattice_service_network_service_association`

**Source:** `crates/winterbaume-terraform/src/converters/vpclattice.rs`

**Inject attributes** (6): `arn`, `custom_domain_name`, `region`, `service_identifier`, `service_network_identifier`, `tags`

**Extract attributes** (6): `arn`, `id`, `service_identifier`, `service_network_identifier`, `status`, `tags`

**Missing from inject** (5): `created_by`, `dns_entry`, `status`, `tags_all`, `timeouts`

**Missing from extract** (5): `created_by`, `custom_domain_name`, `dns_entry`, `tags_all`, `timeouts`

### `aws_vpclattice_service_network_vpc_association`

**Source:** `crates/winterbaume-terraform/src/converters/vpclattice.rs`

**Inject attributes** (6): `arn`, `region`, `security_group_ids`, `service_network_identifier`, `tags`, `vpc_identifier`

**Extract attributes** (7): `arn`, `id`, `security_group_ids`, `service_network_identifier`, `status`, `tags`, `vpc_identifier`

**Missing from inject** (4): `created_by`, `status`, `tags_all`, `timeouts`

**Missing from extract** (3): `created_by`, `tags_all`, `timeouts`

### `aws_vpclattice_target_group`

**Source:** `crates/winterbaume-terraform/src/converters/vpclattice.rs`

**Inject attributes** (7): `arn`, `config`, `name`, `region`, `status`, `tags`, `type`

**Extract attributes** (15): `arn`, `config`, `created_at`, `id`, `ip_address_type`, `lambda_event_structure_version`, `last_updated_at`, `name`, `port`, `protocol`, `protocol_version`, `status`, `tags`, `type`, `vpc_identifier`

**Missing from inject** (2): `tags_all`, `timeouts`

**Missing from extract** (2): `tags_all`, `timeouts`

### `aws_vpclattice_target_group_attachment`

**Source:** `crates/winterbaume-terraform/src/converters/vpclattice.rs`

**Inject attributes** (5): `id`, `port`, `region`, `target`, `target_group_identifier`

**Extract attributes** (4): `id`, `port`, `target`, `target_group_identifier`

**Missing from inject** (1): `timeouts`

**Missing from extract** (1): `timeouts`

### `aws_wafv2_api_key`

**Source:** `crates/winterbaume-terraform/src/converters/wafv2.rs`

**Inject attributes** (4): `api_key`, `region`, `scope`, `token_domains`

**Extract attributes** (4): `api_key`, `id`, `scope`, `token_domains`

### `aws_wafv2_ip_set`

**Source:** `crates/winterbaume-terraform/src/converters/wafv2.rs`

**Inject attributes** (8): `addresses`, `arn`, `description`, `ip_address_version`, `name`, `region`, `scope`, `tags`

**Extract attributes** (7): `addresses`, `arn`, `id`, `ip_address_version`, `name`, `scope`, `tags`

**Missing from inject** (3): `lock_token`, `name_prefix`, `tags_all`

**Missing from extract** (4): `description`, `lock_token`, `name_prefix`, `tags_all`

### `aws_wafv2_regex_pattern_set`

**Source:** `crates/winterbaume-terraform/src/converters/wafv2.rs`

**Inject attributes** (7): `arn`, `description`, `name`, `region`, `regular_expression`, `scope`, `tags`

**Extract attributes** (10): `arn`, `description`, `id`, `lock_token`, `name`, `regex_string`, `regular_expression`, `scope`, `tags`, `tags_all`

**Missing from inject** (3): `lock_token`, `name_prefix`, `tags_all`

**Missing from extract** (1): `name_prefix`

### `aws_wafv2_rule_group`

**Source:** `crates/winterbaume-terraform/src/converters/wafv2.rs`

**Inject attributes** (10): `arn`, `capacity`, `custom_response_body`, `description`, `name`, `region`, `rule`, `scope`, `tags`, `visibility_config`

**Extract attributes** (12): `arn`, `capacity`, `custom_response_body`, `description`, `id`, `lock_token`, `name`, `rule`, `scope`, `tags`, `tags_all`, `visibility_config`

**Missing from inject** (3): `lock_token`, `name_prefix`, `tags_all`

**Missing from extract** (1): `name_prefix`

### `aws_wafv2_web_acl`

**Source:** `crates/winterbaume-terraform/src/converters/wafv2.rs`

**Inject attributes** (16): `arn`, `association_config`, `captcha_config`, `challenge_config`, `custom_response_body`, `data_protection_config`, `default_action`, `description`, `name`, `region`, `rule`, `scope`, `tags`, `tags_all`, `token_domains`, `visibility_config`

**Extract attributes** (18): `application_integration_url`, `arn`, `association_config`, `capacity`, `captcha_config`, `challenge_config`, `custom_response_body`, `default_action`, `description`, `id`, `lock_token`, `name`, `rule`, `scope`, `tags`, `tags_all`, `token_domains`, `visibility_config`

**Missing from inject** (5): `application_integration_url`, `capacity`, `lock_token`, `name_prefix`, `rule_json`

**Missing from extract** (3): `data_protection_config`, `name_prefix`, `rule_json`

### `aws_wafv2_web_acl_association`

**Source:** `crates/winterbaume-terraform/src/converters/wafv2.rs`

**Inject attributes** (3): `region`, `resource_arn`, `web_acl_arn`

**Extract attributes** (3): `id`, `resource_arn`, `web_acl_arn`

**Missing from inject** (1): `timeouts`

**Missing from extract** (1): `timeouts`

### `aws_wafv2_web_acl_logging_configuration`

**Source:** `crates/winterbaume-terraform/src/converters/wafv2.rs`

**Inject attributes** (7): `log_destination_configs`, `log_scope`, `log_type`, `logging_filter`, `redacted_fields`, `region`, `resource_arn`

**Extract attributes** (7): `id`, `log_destination_configs`, `log_scope`, `log_type`, `logging_filter`, `redacted_fields`, `resource_arn`

### `aws_workspaces_directory`

**Source:** `crates/winterbaume-terraform/src/converters/workspaces.rs`

**Inject attributes** (16): `active_directory_config`, `alias`, `directory_id`, `directory_name`, `directory_type`, `ip_group_ids`, `region`, `registration_code`, `saml_properties`, `self_service_permissions`, `streaming_properties`, `tags_all`, `workspace_access_properties`, `workspace_creation_properties`, `workspace_security_group_id`, `workspace_type`

**Extract attributes** (19): `active_directory_config`, `alias`, `customer_user_name`, `directory_id`, `directory_name`, `directory_type`, `dns_ip_addresses`, `iam_role_id`, `id`, `ip_group_ids`, `registration_code`, `self_service_permissions`, `state`, `streaming_properties`, `tags_all`, `workspace_access_properties`, `workspace_creation_properties`, `workspace_security_group_id`, `workspace_type`

**Missing from inject** (9): `certificate_based_auth_properties`, `customer_user_name`, `dns_ip_addresses`, `iam_role_id`, `subnet_ids`, `tags`, `user_identity_type`, `workspace_directory_description`, `workspace_directory_name`

**Missing from extract** (7): `certificate_based_auth_properties`, `saml_properties`, `subnet_ids`, `tags`, `user_identity_type`, `workspace_directory_description`, `workspace_directory_name`

### `aws_workspaces_workspace`

**Source:** `crates/winterbaume-terraform/src/converters/workspaces.rs`

**Inject attributes** (12): `bundle_id`, `computer_name`, `directory_id`, `ip_address`, `region`, `state`, `subnet_id`, `tags`, `tags_all`, `user_name`, `volume_encryption_key`, `workspace_properties`

**Extract attributes** (17): `bundle_id`, `computer_name`, `directory_id`, `id`, `ip_address`, `root_volume_encryption_enabled`, `root_volume_size_gib`, `running_mode`, `running_mode_auto_stop_timeout_in_minutes`, `state`, `subnet_id`, `user_name`, `user_volume_encryption_enabled`, `user_volume_size_gib`, `volume_encryption_key`, `workspace_id`, `workspace_properties`

**Missing from inject** (3): `root_volume_encryption_enabled`, `timeouts`, `user_volume_encryption_enabled`

**Missing from extract** (3): `tags`, `tags_all`, `timeouts`

### `aws_xray_group`

**Source:** `crates/winterbaume-terraform/src/converters/xray.rs`

**Inject attributes** (5): `arn`, `filter_expression`, `group_name`, `insights_configuration`, `region`

**Extract attributes** (6): `arn`, `filter_expression`, `group_name`, `id`, `insights_enabled`, `notifications_enabled`

**Missing from inject** (2): `tags`, `tags_all`

**Missing from extract** (3): `insights_configuration`, `tags`, `tags_all`

### `aws_xray_sampling_rule`

**Source:** `crates/winterbaume-terraform/src/converters/xray.rs`

**Inject attributes** (15): `arn`, `created_at`, `fixed_rate`, `host`, `http_method`, `modified_at`, `priority`, `region`, `reservoir_size`, `resource_arn`, `rule_name`, `service_name`, `service_type`, `url_path`, `version`

**Extract attributes** (15): `arn`, `created_at`, `fixed_rate`, `host`, `http_method`, `id`, `modified_at`, `priority`, `reservoir_size`, `resource_arn`, `rule_name`, `service_name`, `service_type`, `url_path`, `version`

**Missing from inject** (3): `attributes`, `tags`, `tags_all`

**Missing from extract** (3): `attributes`, `tags`, `tags_all`

