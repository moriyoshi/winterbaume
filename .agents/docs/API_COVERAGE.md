# API Coverage Report

Generated: 2026-05-12

| Project | Version |
|---------|---------|
| winterbaume | winterbaume-server-v0.1.0-25-g535016a3 |
| moto | 5.2.1.dev |
| floci | 1.5.14 |
| kumo | v0.19.0 |

## Overview

Legend: `winterbaume` = operations with real, state-backed logic. `stubs` = operations whose handler routes the request and returns an empty/default response without real behaviour (annotated with `// STUB[<category>]: ...` in `handlers.rs`). The two columns are disjoint -- stubs are excluded from the winterbaume count.

| Service | Model | winterbaume | stubs | moto | floci | kumo | Total | wb% | stub% | moto% | floci% | kumo% |
|---------|-------|-------------|-------|------|-------|------|-------|-----|-------|-------|--------|------|
| winterbaume-accessanalyzer | accessanalyzer | 11 | 0 | 0 | 0 | 0 | 37 | 29.7% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-account | account | 14 | 1 | 3 | 0 | 0 | 15 | 93.3% | 6.7% | 20.0% | 0.0% | 0.0% |
| winterbaume-acm | acm | 16 | 0 | 11 | 0 | 6 | 17 | 94.1% | 0.0% | 64.7% | 0.0% | 35.3% |
| winterbaume-acmpca | acm-pca | 23 | 0 | 17 | 0 | 0 | 23 | 100.0% | 0.0% | 73.9% | 0.0% | 0.0% |
| winterbaume-aiops | aiops | 11 | 0 | 0 | 0 | 0 | 11 | 100.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-amp | amp | 17 | 0 | 17 | 0 | 0 | 44 | 38.6% | 0.0% | 38.6% | 0.0% | 0.0% |
| winterbaume-amplify | amplify | 23 | 0 | 0 | 0 | 9 | 37 | 62.2% | 0.0% | 0.0% | 0.0% | 24.3% |
| winterbaume-amplifybackend | amplifybackend | 4 | 0 | 0 | 0 | 0 | 31 | 12.9% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-amplifyuibuilder | amplifyuibuilder | 28 | 0 | 0 | 0 | 0 | 28 | 100.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-apigateway | api-gateway | 117 | 2 | 78 | 70 | 17 | 124 | 94.4% | 1.6% | 62.9% | 56.5% | 13.7% |
| winterbaume-apigatewaymanagement | apigatewaymanagementapi | 3 | 0 | 3 | 0 | 0 | 3 | 100.0% | 0.0% | 100.0% | 0.0% | 0.0% |
| winterbaume-apigatewayv2 | apigatewayv2 | 62 | 0 | 54 | 0 | 0 | 103 | 60.2% | 0.0% | 52.4% | 0.0% | 0.0% |
| winterbaume-appconfig | appconfig | 45 | 0 | 15 | 0 | 0 | 45 | 100.0% | 0.0% | 33.3% | 0.0% | 0.0% |
| winterbaume-appconfigdata | appconfigdata | 2 | 0 | 0 | 0 | 0 | 2 | 100.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-appfabric | appfabric | 6 | 0 | 0 | 0 | 0 | 26 | 23.1% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-appflow | appflow | 9 | 0 | 0 | 0 | 0 | 25 | 36.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-appintegrations | appintegrations | 23 | 0 | 0 | 0 | 0 | 23 | 100.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-applicationautoscaling | application-auto-scaling | 13 | 1 | 9 | 0 | 0 | 14 | 92.9% | 7.1% | 64.3% | 0.0% | 0.0% |
| winterbaume-applicationcostprofiler | applicationcostprofiler | 6 | 0 | 0 | 0 | 0 | 6 | 100.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-applicationdiscoveryservice | application-discovery-service | 28 | 0 | 0 | 0 | 0 | 28 | 100.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-applicationinsights | application-insights | 33 | 0 | 0 | 0 | 0 | 33 | 100.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-applicationsignals | application-signals | 10 | 3 | 0 | 0 | 0 | 23 | 43.5% | 13.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-appmesh | app-mesh | 38 | 0 | 0 | 0 | 25 | 38 | 100.0% | 0.0% | 0.0% | 0.0% | 65.8% |
| winterbaume-apprunner | apprunner | 23 | 0 | 0 | 0 | 0 | 37 | 62.2% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-appsync | appsync | 27 | 0 | 27 | 0 | 3 | 74 | 36.5% | 0.0% | 36.5% | 0.0% | 4.1% |
| winterbaume-arczonalshift | arc-zonal-shift | 14 | 1 | 0 | 0 | 0 | 15 | 93.3% | 6.7% | 0.0% | 0.0% | 0.0% |
| winterbaume-artifact | artifact | 8 | 0 | 0 | 0 | 0 | 8 | 100.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-athena | athena | 25 | 0 | 27 | 0 | 7 | 70 | 35.7% | 0.0% | 38.6% | 0.0% | 10.0% |
| winterbaume-auditmanager | auditmanager | 15 | 0 | 0 | 0 | 0 | 62 | 24.2% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-autoscaling | auto-scaling | 52 | 0 | 39 | 0 | 0 | 66 | 78.8% | 0.0% | 59.1% | 0.0% | 0.0% |
| winterbaume-autoscalingplans | auto-scaling-plans | 6 | 0 | 0 | 0 | 0 | 6 | 100.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-backup | backup | 105 | 3 | 17 | 0 | 12 | 108 | 97.2% | 2.8% | 15.7% | 0.0% | 11.1% |
| winterbaume-backupgateway | backup-gateway | 25 | 0 | 0 | 0 | 0 | 25 | 100.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-backupsearch | backupsearch | 9 | 0 | 0 | 0 | 0 | 12 | 75.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-batch | batch | 39 | 0 | 24 | 0 | 10 | 45 | 86.7% | 0.0% | 53.3% | 0.0% | 22.2% |
| winterbaume-bcmdashboards | bcm-dashboards | 9 | 0 | 0 | 0 | 0 | 15 | 60.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-bcmdataexports | bcm-data-exports | 12 | 0 | 0 | 0 | 0 | 12 | 100.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-bcmrecommendedactions | bcm-recommended-actions | 1 | 0 | 0 | 0 | 0 | 1 | 100.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-bedrock | bedrock | 48 | 0 | 13 | 0 | 0 | 101 | 47.5% | 0.0% | 12.9% | 0.0% | 0.0% |
| winterbaume-bedrockagent | bedrock-agent | 72 | 0 | 11 | 0 | 0 | 72 | 100.0% | 0.0% | 15.3% | 0.0% | 0.0% |
| winterbaume-billing | billing | 12 | 0 | 0 | 0 | 0 | 12 | 100.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-braket | braket | 12 | 0 | 0 | 0 | 0 | 17 | 70.6% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-budgets | budgets | 7 | 0 | 7 | 0 | 0 | 26 | 26.9% | 0.0% | 26.9% | 0.0% | 0.0% |
| winterbaume-chatbot | chatbot | 15 | 0 | 0 | 0 | 0 | 34 | 44.1% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-chimesdkmeetings | chime-sdk-meetings | 12 | 0 | 0 | 0 | 0 | 16 | 75.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-cloud9 | cloud9 | 13 | 0 | 0 | 0 | 0 | 13 | 100.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-cloudcontrol | cloudcontrol | 8 | 0 | 0 | 0 | 0 | 8 | 100.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-clouddirectory | clouddirectory | 13 | 0 | 13 | 0 | 0 | 66 | 19.7% | 0.0% | 19.7% | 0.0% | 0.0% |
| winterbaume-cloudformation | cloudformation | 40 | 3 | 33 | 0 | 8 | 90 | 44.4% | 3.3% | 36.7% | 0.0% | 8.9% |
| winterbaume-cloudfront | cloudfront | 156 | 11 | 25 | 0 | 8 | 167 | 93.4% | 6.6% | 15.0% | 0.0% | 4.8% |
| winterbaume-cloudfrontkeyvaluestore | cloudfront-keyvaluestore | 5 | 0 | 0 | 0 | 0 | 6 | 83.3% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-cloudhsmv2 | cloudhsm-v2 | 18 | 0 | 0 | 0 | 0 | 18 | 100.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-cloudsearchdomain | cloudsearch-domain | 2 | 0 | 0 | 0 | 0 | 3 | 66.7% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-cloudtrail | cloudtrail | 21 | 2 | 16 | 0 | 8 | 60 | 35.0% | 3.3% | 26.7% | 0.0% | 13.3% |
| winterbaume-cloudtraildata | cloudtrail-data | 1 | 0 | 0 | 0 | 0 | 1 | 100.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-cloudwatch | cloudwatch | 38 | 5 | 20 | 0 | 10 | 46 | 82.6% | 10.9% | 43.5% | 0.0% | 21.7% |
| winterbaume-cloudwatchlogs | cloudwatch-logs | 93 | 15 | 51 | 0 | 11 | 113 | 82.3% | 13.3% | 45.1% | 0.0% | 9.7% |
| winterbaume-codeartifact | codeartifact | 9 | 0 | 0 | 0 | 0 | 48 | 18.8% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-codebuild | codebuild | 29 | 0 | 9 | 0 | 0 | 59 | 49.2% | 0.0% | 15.3% | 0.0% | 0.0% |
| winterbaume-codecommit | codecommit | 25 | 0 | 3 | 0 | 0 | 79 | 31.6% | 0.0% | 3.8% | 0.0% | 0.0% |
| winterbaume-codedeploy | codedeploy | 15 | 0 | 14 | 0 | 0 | 47 | 31.9% | 0.0% | 29.8% | 0.0% | 0.0% |
| winterbaume-codegurureviewer | codeguru-reviewer | 9 | 0 | 0 | 0 | 11 | 14 | 64.3% | 0.0% | 0.0% | 0.0% | 78.6% |
| winterbaume-codegurusecurity | codeguru-security | 11 | 0 | 0 | 0 | 0 | 13 | 84.6% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-codepipeline | codepipeline | 44 | 0 | 8 | 0 | 0 | 44 | 100.0% | 0.0% | 18.2% | 0.0% | 0.0% |
| winterbaume-codestarnotifications | codestar-notifications | 7 | 0 | 0 | 0 | 0 | 13 | 53.8% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-cognitoidentity | cognito-identity | 20 | 3 | 10 | 0 | 0 | 23 | 87.0% | 13.0% | 43.5% | 0.0% | 0.0% |
| winterbaume-cognitoidentityprovider | cognito-identity-provider | 104 | 18 | 62 | 39 | 17 | 122 | 85.2% | 14.8% | 50.8% | 32.0% | 13.9% |
| winterbaume-cognitosync | cognito-sync | 11 | 0 | 0 | 0 | 0 | 17 | 64.7% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-comprehend | comprehend | 60 | 5 | 63 | 0 | 12 | 85 | 70.6% | 5.9% | 74.1% | 0.0% | 14.1% |
| winterbaume-config | config-service | 46 | 3 | 38 | 0 | 9 | 97 | 47.4% | 3.1% | 39.2% | 0.0% | 9.3% |
| winterbaume-connect | connect | 10 | 0 | 10 | 0 | 0 | 370 | 2.7% | 0.0% | 2.7% | 0.0% | 0.0% |
| winterbaume-connectcampaigns | connectcampaigns | 14 | 0 | 14 | 0 | 0 | 22 | 63.6% | 0.0% | 63.6% | 0.0% | 0.0% |
| winterbaume-connectcontactlens | connect-contact-lens | 0 | 0 | 0 | 0 | 0 | 1 | 0.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-connectparticipant | connectparticipant | 7 | 0 | 0 | 0 | 0 | 11 | 63.6% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-controlcatalog | controlcatalog | 6 | 0 | 0 | 0 | 0 | 6 | 100.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-costandusagereport | cost-and-usage-report-service | 7 | 0 | 0 | 0 | 0 | 7 | 100.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-costexplorer | cost-explorer | 22 | 25 | 0 | 0 | 8 | 47 | 46.8% | 53.2% | 0.0% | 0.0% | 17.0% |
| winterbaume-costoptimizationhub | cost-optimization-hub | 8 | 0 | 0 | 0 | 0 | 8 | 100.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-databasemigration | database-migration-service | 42 | 0 | 17 | 0 | 0 | 119 | 35.3% | 0.0% | 14.3% | 0.0% | 0.0% |
| winterbaume-databrew | databrew | 32 | 1 | 24 | 0 | 0 | 44 | 72.7% | 2.3% | 54.5% | 0.0% | 0.0% |
| winterbaume-datapipeline | data-pipeline | 19 | 0 | 0 | 0 | 0 | 19 | 100.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-datasync | datasync | 8 | 0 | 6 | 0 | 0 | 53 | 15.1% | 0.0% | 11.3% | 0.0% | 0.0% |
| winterbaume-dax | dax | 6 | 0 | 8 | 0 | 0 | 21 | 28.6% | 0.0% | 38.1% | 0.0% | 0.0% |
| winterbaume-directconnect | direct-connect | 7 | 0 | 0 | 0 | 0 | 63 | 11.1% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-directory | directory-service | 4 | 0 | 0 | 0 | 6 | 80 | 5.0% | 0.0% | 0.0% | 0.0% | 7.5% |
| winterbaume-dlm | dlm | 2 | 0 | 0 | 0 | 5 | 8 | 25.0% | 0.0% | 0.0% | 0.0% | 62.5% |
| winterbaume-dsql | dsql | 12 | 0 | 5 | 0 | 0 | 12 | 100.0% | 0.0% | 41.7% | 0.0% | 0.0% |
| winterbaume-dynamodb | dynamodb | 57 | 0 | 39 | 0 | 20 | 57 | 100.0% | 0.0% | 68.4% | 0.0% | 35.1% |
| winterbaume-dynamodbstreams | dynamodb-streams | 4 | 0 | 0 | 0 | 0 | 4 | 100.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-ebs | ebs | 6 | 0 | 6 | 0 | 0 | 6 | 100.0% | 0.0% | 100.0% | 0.0% | 0.0% |
| winterbaume-ec2 | ec2 | 713 | 43 | 223 | 0 | 39 | 763 | 93.4% | 5.6% | 29.2% | 0.0% | 5.1% |
| winterbaume-ec2instanceconnect | ec2-instance-connect | 2 | 0 | 1 | 0 | 0 | 2 | 100.0% | 0.0% | 50.0% | 0.0% | 0.0% |
| winterbaume-ecr | ecr | 58 | 0 | 29 | 0 | 11 | 58 | 100.0% | 0.0% | 50.0% | 0.0% | 19.0% |
| winterbaume-ecs | ecs | 63 | 1 | 45 | 0 | 12 | 76 | 82.9% | 1.3% | 59.2% | 0.0% | 15.8% |
| winterbaume-efs | efs | 31 | 0 | 19 | 0 | 0 | 31 | 100.0% | 0.0% | 61.3% | 0.0% | 0.0% |
| winterbaume-eks | eks | 55 | 4 | 17 | 0 | 8 | 64 | 85.9% | 6.2% | 26.6% | 0.0% | 12.5% |
| winterbaume-elasticache | elasticache | 24 | 0 | 17 | 0 | 7 | 75 | 32.0% | 0.0% | 22.7% | 0.0% | 9.3% |
| winterbaume-elasticbeanstalk | elastic-beanstalk | 7 | 0 | 0 | 0 | 7 | 47 | 14.9% | 0.0% | 0.0% | 0.0% | 14.9% |
| winterbaume-elasticloadbalancing | elastic-load-balancing | 29 | 0 | 21 | 0 | 0 | 29 | 100.0% | 0.0% | 72.4% | 0.0% | 0.0% |
| winterbaume-elasticloadbalancingv2 | elastic-load-balancing-v2 | 50 | 1 | 33 | 0 | 22 | 51 | 98.0% | 2.0% | 64.7% | 0.0% | 43.1% |
| winterbaume-emr | emr | 54 | 2 | 26 | 0 | 0 | 60 | 90.0% | 3.3% | 43.3% | 0.0% | 0.0% |
| winterbaume-emrcontainers | emr-containers | 23 | 0 | 8 | 0 | 0 | 23 | 100.0% | 0.0% | 34.8% | 0.0% | 0.0% |
| winterbaume-emrserverless | emr-serverless | 16 | 0 | 11 | 0 | 11 | 22 | 72.7% | 0.0% | 50.0% | 0.0% | 50.0% |
| winterbaume-eventbridge | eventbridge | 57 | 0 | 45 | 0 | 15 | 57 | 100.0% | 0.0% | 78.9% | 0.0% | 26.3% |
| winterbaume-firehose | firehose | 12 | 0 | 12 | 0 | 7 | 12 | 100.0% | 0.0% | 100.0% | 0.0% | 58.3% |
| winterbaume-fis | fis | 8 | 0 | 5 | 0 | 0 | 26 | 30.8% | 0.0% | 19.2% | 0.0% | 0.0% |
| winterbaume-forecast | forecast | 5 | 0 | 5 | 0 | 17 | 63 | 7.9% | 0.0% | 7.9% | 0.0% | 27.0% |
| winterbaume-freetier | freetier | 5 | 0 | 0 | 0 | 0 | 5 | 100.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-fsx | fsx | 9 | 0 | 9 | 0 | 0 | 48 | 18.8% | 0.0% | 18.8% | 0.0% | 0.0% |
| winterbaume-glacier | glacier | 33 | 0 | 10 | 0 | 4 | 33 | 100.0% | 0.0% | 30.3% | 0.0% | 12.1% |
| winterbaume-glue | glue | 132 | 0 | 96 | 0 | 14 | 265 | 49.8% | 0.0% | 36.2% | 0.0% | 5.3% |
| winterbaume-greengrass | greengrass | 71 | 0 | 55 | 0 | 0 | 92 | 77.2% | 0.0% | 59.8% | 0.0% | 0.0% |
| winterbaume-guardduty | guardduty | 85 | 2 | 12 | 0 | 0 | 87 | 97.7% | 2.3% | 13.8% | 0.0% | 0.0% |
| winterbaume-iam | iam | 154 | 22 | 119 | 0 | 39 | 176 | 87.5% | 12.5% | 67.6% | 0.0% | 22.2% |
| winterbaume-identitystore | identitystore | 17 | 0 | 14 | 0 | 0 | 19 | 89.5% | 0.0% | 73.7% | 0.0% | 0.0% |
| winterbaume-inspector2 | inspector2 | 49 | 21 | 19 | 0 | 0 | 75 | 65.3% | 28.0% | 25.3% | 0.0% | 0.0% |
| winterbaume-iot | iot | 103 | 0 | 100 | 0 | 0 | 272 | 37.9% | 0.0% | 36.8% | 0.0% | 0.0% |
| winterbaume-iotdataplane | iot-data-plane | 8 | 0 | 0 | 0 | 0 | 8 | 100.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-ivs | ivs | 30 | 5 | 6 | 0 | 0 | 40 | 75.0% | 12.5% | 15.0% | 0.0% | 0.0% |
| winterbaume-kafka | kafka | 10 | 0 | 13 | 0 | 6 | 59 | 16.9% | 0.0% | 22.0% | 0.0% | 10.2% |
| winterbaume-keyspaces | keyspaces | 19 | 0 | 0 | 0 | 0 | 19 | 100.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-kinesis | kinesis | 38 | 0 | 31 | 0 | 9 | 39 | 97.4% | 0.0% | 79.5% | 0.0% | 23.1% |
| winterbaume-kinesisanalyticsv2 | kinesis-analytics-v2 | 32 | 1 | 0 | 0 | 0 | 33 | 97.0% | 3.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-kinesisvideo | kinesis-video | 32 | 0 | 0 | 0 | 0 | 32 | 100.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-kinesisvideoarchivedmedia | kinesis-video-archived-media | 6 | 0 | 3 | 0 | 0 | 6 | 100.0% | 0.0% | 50.0% | 0.0% | 0.0% |
| winterbaume-kms | kms | 53 | 0 | 40 | 0 | 19 | 54 | 98.1% | 0.0% | 74.1% | 0.0% | 35.2% |
| winterbaume-lakeformation | lakeformation | 19 | 1 | 20 | 0 | 0 | 61 | 31.1% | 1.6% | 32.8% | 0.0% | 0.0% |
| winterbaume-lambda | lambda | 85 | 0 | 46 | 0 | 17 | 85 | 100.0% | 0.0% | 54.1% | 0.0% | 20.0% |
| winterbaume-lexmodelsv2 | lex-models-v2 | 58 | 2 | 17 | 0 | 0 | 107 | 54.2% | 1.9% | 15.9% | 0.0% | 0.0% |
| winterbaume-macie2 | macie2 | 67 | 14 | 13 | 0 | 24 | 81 | 82.7% | 17.3% | 16.0% | 0.0% | 29.6% |
| winterbaume-managedblockchain | managedblockchain | 27 | 0 | 20 | 0 | 0 | 27 | 100.0% | 0.0% | 74.1% | 0.0% | 0.0% |
| winterbaume-marketplacemetering | marketplace-metering | 4 | 0 | 0 | 0 | 0 | 4 | 100.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-mediaconnect | mediaconnect | 21 | 0 | 18 | 0 | 0 | 82 | 25.6% | 0.0% | 22.0% | 0.0% | 0.0% |
| winterbaume-medialive | medialive | 16 | 0 | 12 | 0 | 0 | 123 | 13.0% | 0.0% | 9.8% | 0.0% | 0.0% |
| winterbaume-mediapackage | mediapackage | 9 | 0 | 9 | 0 | 0 | 19 | 47.4% | 0.0% | 47.4% | 0.0% | 0.0% |
| winterbaume-mediapackagev2 | mediapackagev2 | 7 | 0 | 7 | 0 | 0 | 30 | 23.3% | 0.0% | 23.3% | 0.0% | 0.0% |
| winterbaume-mediastore | mediastore | 11 | 0 | 11 | 0 | 0 | 21 | 52.4% | 0.0% | 52.4% | 0.0% | 0.0% |
| winterbaume-mediastoredata | mediastore-data | 5 | 0 | 4 | 0 | 0 | 5 | 100.0% | 0.0% | 80.0% | 0.0% | 0.0% |
| winterbaume-memorydb | memorydb | 13 | 0 | 13 | 0 | 10 | 45 | 28.9% | 0.0% | 28.9% | 0.0% | 22.2% |
| winterbaume-mq | mq | 23 | 1 | 19 | 0 | 6 | 24 | 95.8% | 4.2% | 79.2% | 0.0% | 25.0% |
| winterbaume-neptune | neptune | 64 | 6 | 47 | 0 | 6 | 70 | 91.4% | 8.6% | 67.1% | 0.0% | 8.6% |
| winterbaume-networkfirewall | network-firewall | 79 | 0 | 5 | 0 | 0 | 79 | 100.0% | 0.0% | 6.3% | 0.0% | 0.0% |
| winterbaume-networkmanager | networkmanager | 53 | 0 | 18 | 0 | 0 | 95 | 55.8% | 0.0% | 18.9% | 0.0% | 0.0% |
| winterbaume-opensearch | opensearch | 44 | 0 | 11 | 0 | 0 | 88 | 50.0% | 0.0% | 12.5% | 0.0% | 0.0% |
| winterbaume-opensearchserverless | opensearchserverless | 12 | 0 | 12 | 0 | 0 | 46 | 26.1% | 0.0% | 26.1% | 0.0% | 0.0% |
| winterbaume-organizations | organizations | 60 | 3 | 41 | 0 | 11 | 63 | 95.2% | 4.8% | 65.1% | 0.0% | 17.5% |
| winterbaume-osis | osis | 10 | 0 | 13 | 0 | 0 | 22 | 45.5% | 0.0% | 59.1% | 0.0% | 0.0% |
| winterbaume-outposts | outposts | 13 | 0 | 0 | 0 | 0 | 37 | 35.1% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-panorama | panorama | 10 | 1 | 11 | 0 | 0 | 34 | 29.4% | 2.9% | 32.4% | 0.0% | 0.0% |
| winterbaume-pcaconnectorscep | pca-connector-scep | 11 | 0 | 0 | 0 | 0 | 12 | 91.7% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-personalize | personalize | 66 | 5 | 4 | 0 | 0 | 71 | 93.0% | 7.0% | 5.6% | 0.0% | 0.0% |
| winterbaume-personalizeevents | personalize-events | 5 | 0 | 0 | 0 | 0 | 5 | 100.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-personalizeruntime | personalize-runtime | 3 | 0 | 0 | 0 | 0 | 3 | 100.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-pi | pi | 9 | 4 | 0 | 0 | 0 | 13 | 69.2% | 30.8% | 0.0% | 0.0% | 0.0% |
| winterbaume-pinpoint | pinpoint | 15 | 0 | 12 | 0 | 0 | 122 | 12.3% | 0.0% | 9.8% | 0.0% | 0.0% |
| winterbaume-pinpointsmsvoice | pinpoint-sms-voice | 4 | 0 | 0 | 0 | 0 | 8 | 50.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-pipes | pipes | 10 | 0 | 9 | 0 | 10 | 10 | 100.0% | 0.0% | 90.0% | 0.0% | 100.0% |
| winterbaume-polly | polly | 9 | 0 | 5 | 0 | 0 | 10 | 90.0% | 0.0% | 50.0% | 0.0% | 0.0% |
| winterbaume-pricing | pricing | 5 | 0 | 0 | 0 | 0 | 5 | 100.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-quicksight | quicksight | 68 | 0 | 31 | 0 | 0 | 232 | 29.3% | 0.0% | 13.4% | 0.0% | 0.0% |
| winterbaume-ram | ram | 35 | 0 | 8 | 0 | 0 | 35 | 100.0% | 0.0% | 22.9% | 0.0% | 0.0% |
| winterbaume-rbin | rbin | 9 | 0 | 0 | 0 | 0 | 10 | 90.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-rds | rds | 146 | 4 | 85 | 0 | 12 | 164 | 89.0% | 2.4% | 51.8% | 0.0% | 7.3% |
| winterbaume-rdsdata | rds-data | 6 | 0 | 1 | 0 | 0 | 6 | 100.0% | 0.0% | 16.7% | 0.0% | 0.0% |
| winterbaume-redshift | redshift | 100 | 3 | 35 | 0 | 7 | 141 | 70.9% | 2.1% | 24.8% | 0.0% | 5.0% |
| winterbaume-redshiftdata | redshift-data | 11 | 0 | 4 | 0 | 0 | 11 | 100.0% | 0.0% | 36.4% | 0.0% | 0.0% |
| winterbaume-rekognition | rekognition | 8 | 4 | 8 | 0 | 13 | 75 | 10.7% | 5.3% | 10.7% | 0.0% | 17.3% |
| winterbaume-resiliencehub | resiliencehub | 22 | 0 | 17 | 0 | 17 | 63 | 34.9% | 0.0% | 27.0% | 0.0% | 27.0% |
| winterbaume-resourcegroups | resource-groups | 22 | 1 | 15 | 0 | 0 | 23 | 95.7% | 4.3% | 65.2% | 0.0% | 0.0% |
| winterbaume-resourcegroupstagging | resource-groups-tagging-api | 5 | 0 | 0 | 0 | 0 | 9 | 55.6% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-rolesanywhere | rolesanywhere | 28 | 2 | 0 | 0 | 0 | 30 | 93.3% | 6.7% | 0.0% | 0.0% | 0.0% |
| winterbaume-route53 | route-53 | 71 | 0 | 30 | 0 | 10 | 71 | 100.0% | 0.0% | 42.3% | 0.0% | 14.1% |
| winterbaume-route53domains | route-53-domains | 5 | 0 | 0 | 0 | 0 | 34 | 14.7% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-route53recoverycluster | route53-recovery-cluster | 4 | 0 | 0 | 0 | 0 | 4 | 100.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-route53resolver | route53resolver | 28 | 0 | 28 | 0 | 11 | 68 | 41.2% | 0.0% | 41.2% | 0.0% | 16.2% |
| winterbaume-s3 | s3 | 103 | 4 | 73 | 51 | 36 | 107 | 96.3% | 3.7% | 68.2% | 47.7% | 33.6% |
| winterbaume-s3control | s3-control | 87 | 10 | 0 | 0 | 7 | 97 | 89.7% | 10.3% | 0.0% | 0.0% | 7.2% |
| winterbaume-s3files | s3files | 21 | 0 | 0 | 0 | 0 | 21 | 100.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-s3outposts | s3outposts | 3 | 1 | 0 | 0 | 0 | 5 | 60.0% | 20.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-s3tables | s3tables | 46 | 3 | 14 | 0 | 12 | 49 | 93.9% | 6.1% | 28.6% | 0.0% | 24.5% |
| winterbaume-s3vectors | s3vectors | 19 | 0 | 15 | 0 | 0 | 19 | 100.0% | 0.0% | 78.9% | 0.0% | 0.0% |
| winterbaume-sagemaker | sagemaker | 141 | 1 | 112 | 0 | 11 | 396 | 35.6% | 0.3% | 28.3% | 0.0% | 2.8% |
| winterbaume-sagemakermetrics | sagemaker-metrics | 2 | 0 | 1 | 0 | 0 | 2 | 100.0% | 0.0% | 50.0% | 0.0% | 0.0% |
| winterbaume-sagemakerruntime | sagemaker-runtime | 3 | 0 | 2 | 0 | 0 | 3 | 100.0% | 0.0% | 66.7% | 0.0% | 0.0% |
| winterbaume-savingsplans | savingsplans | 10 | 0 | 0 | 0 | 0 | 10 | 100.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-scheduler | scheduler | 12 | 0 | 12 | 0 | 9 | 12 | 100.0% | 0.0% | 100.0% | 0.0% | 75.0% |
| winterbaume-secretsmanager | secrets-manager | 22 | 1 | 21 | 0 | 11 | 23 | 95.7% | 4.3% | 91.3% | 0.0% | 47.8% |
| winterbaume-securityhub | securityhub | 97 | 10 | 13 | 0 | 0 | 107 | 90.7% | 9.3% | 12.1% | 0.0% | 0.0% |
| winterbaume-servicecatalog | service-catalog | 4 | 0 | 0 | 0 | 0 | 90 | 4.4% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-servicecatalogappregistry | service-catalog-appregistry | 23 | 1 | 0 | 0 | 0 | 24 | 95.8% | 4.2% | 0.0% | 0.0% | 0.0% |
| winterbaume-servicediscovery | servicediscovery | 27 | 0 | 27 | 0 | 0 | 30 | 90.0% | 0.0% | 90.0% | 0.0% | 0.0% |
| winterbaume-servicequotas | service-quotas | 5 | 0 | 2 | 0 | 8 | 26 | 19.2% | 0.0% | 7.7% | 0.0% | 30.8% |
| winterbaume-ses | ses | 38 | 2 | 38 | 0 | 0 | 71 | 53.5% | 2.8% | 53.5% | 0.0% | 0.0% |
| winterbaume-sesv2 | sesv2 | 106 | 4 | 28 | 0 | 9 | 110 | 96.4% | 3.6% | 25.5% | 0.0% | 8.2% |
| winterbaume-sfn | sfn | 35 | 2 | 29 | 0 | 18 | 37 | 94.6% | 5.4% | 78.4% | 0.0% | 48.6% |
| winterbaume-shield | shield | 9 | 0 | 9 | 0 | 0 | 36 | 25.0% | 0.0% | 25.0% | 0.0% | 0.0% |
| winterbaume-signer | signer | 19 | 0 | 7 | 0 | 0 | 19 | 100.0% | 0.0% | 36.8% | 0.0% | 0.0% |
| winterbaume-simpledbv2 | simpledbv2 | 3 | 0 | 0 | 0 | 0 | 3 | 100.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-simspaceweaver | simspaceweaver | 15 | 0 | 0 | 0 | 0 | 16 | 93.8% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-snowdevicemanagement | snow-device-management | 11 | 0 | 0 | 0 | 0 | 13 | 84.6% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-sns | sns | 41 | 1 | 33 | 0 | 13 | 42 | 97.6% | 2.4% | 78.6% | 0.0% | 31.0% |
| winterbaume-sqs | sqs | 23 | 0 | 20 | 0 | 14 | 23 | 100.0% | 0.0% | 87.0% | 0.0% | 60.9% |
| winterbaume-ssm | ssm | 127 | 19 | 41 | 0 | 7 | 146 | 87.0% | 13.0% | 28.1% | 0.0% | 4.8% |
| winterbaume-ssmquicksetup | ssm-quicksetup | 6 | 0 | 0 | 0 | 0 | 14 | 42.9% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-sso | sso | 4 | 0 | 0 | 0 | 0 | 4 | 100.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-ssoadmin | sso-admin | 27 | 1 | 25 | 0 | 0 | 79 | 34.2% | 1.3% | 31.6% | 0.0% | 0.0% |
| winterbaume-sts | sts | 11 | 0 | 7 | 0 | 6 | 11 | 100.0% | 0.0% | 63.6% | 0.0% | 54.5% |
| winterbaume-support | support | 5 | 1 | 5 | 0 | 0 | 16 | 31.2% | 6.2% | 31.2% | 0.0% | 0.0% |
| winterbaume-supportapp | support-app | 3 | 0 | 0 | 0 | 0 | 10 | 30.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-swf | swf | 30 | 0 | 19 | 0 | 0 | 39 | 76.9% | 0.0% | 48.7% | 0.0% | 0.0% |
| winterbaume-synthetics | synthetics | 22 | 0 | 4 | 0 | 0 | 22 | 100.0% | 0.0% | 18.2% | 0.0% | 0.0% |
| winterbaume-taxsettings | taxsettings | 0 | 0 | 0 | 0 | 0 | 16 | 0.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-textract | textract | 6 | 0 | 5 | 0 | 0 | 25 | 24.0% | 0.0% | 20.0% | 0.0% | 0.0% |
| winterbaume-timestreaminfluxdb | timestream-influxdb | 19 | 0 | 13 | 0 | 0 | 19 | 100.0% | 0.0% | 68.4% | 0.0% | 0.0% |
| winterbaume-timestreamquery | timestream-query | 15 | 0 | 6 | 0 | 0 | 15 | 100.0% | 0.0% | 40.0% | 0.0% | 0.0% |
| winterbaume-timestreamwrite | timestream-write | 19 | 0 | 15 | 0 | 0 | 19 | 100.0% | 0.0% | 78.9% | 0.0% | 0.0% |
| winterbaume-transcribe | transcribe | 16 | 0 | 16 | 0 | 0 | 43 | 37.2% | 0.0% | 37.2% | 0.0% | 0.0% |
| winterbaume-transfer | transfer | 44 | 0 | 14 | 0 | 0 | 71 | 62.0% | 0.0% | 19.7% | 0.0% | 0.0% |
| winterbaume-trustedadvisor | trustedadvisor | 6 | 4 | 0 | 0 | 0 | 11 | 54.5% | 36.4% | 0.0% | 0.0% | 0.0% |
| winterbaume-vpclattice | vpc-lattice | 66 | 2 | 35 | 0 | 0 | 73 | 90.4% | 2.7% | 47.9% | 0.0% | 0.0% |
| winterbaume-wafv2 | wafv2 | 38 | 0 | 29 | 0 | 0 | 55 | 69.1% | 0.0% | 52.7% | 0.0% | 0.0% |
| winterbaume-workspaces | workspaces | 50 | 0 | 16 | 0 | 0 | 91 | 54.9% | 0.0% | 17.6% | 0.0% | 0.0% |
| winterbaume-workspacesweb | workspaces-web | 68 | 0 | 27 | 0 | 0 | 75 | 90.7% | 0.0% | 36.0% | 0.0% | 0.0% |
| winterbaume-xray | xray | 34 | 4 | 0 | 0 | 6 | 38 | 89.5% | 10.5% | 0.0% | 0.0% | 15.8% |

**winterbaume winterbaume-server-v0.1.0-25-g535016a3: 7210 / 11367 operations across 224 services (63.4%)**

**winterbaume stubs: 326 / 11367 operations across 224 services (2.9%) - routed but return empty/default responses**

**moto 5.2.1.dev: 3302 / 11367 operations across 224 services (29.0%)**

**floci 1.5.14: 160 / 11367 operations across 224 services (1.4%)**

**kumo v0.19.0: 790 / 11367 operations across 224 services (6.9%)**

**integration tests: 6126 / 7210 implemented operations tested (85.0%)**

**integration-test service coverage: 222 / 224 services with at least one tested implemented operation (99.1%)**

**terraform E2E: 40 / 224 services with success-oriented coverage (17.9%)**

**terraform E2E tests: 356 success-oriented tests covering 170 terraform resource types**

## Stub Service Moto Coverage

| Signing Name | Model | moto | Total | moto% |
|---|---|---|---|---|

## Integration Test Coverage

Legend: `Implemented` = implemented operations, `Tested` = implemented operations exercised by `tests/integration_test.rs`, `Untested` = implemented operations without integration coverage

| Service | Implemented | Tested | Untested | Coverage | Status |
|---|---|---|---|---|---|
| winterbaume-accessanalyzer | 11 | 11 | 0 | 100.0% | fully covered |
| winterbaume-account | 14 | 14 | 0 | 100.0% | fully covered |
| winterbaume-acm | 16 | 12 | 4 | 75.0% | partial |
| winterbaume-acmpca | 23 | 22 | 1 | 95.7% | partial |
| winterbaume-aiops | 11 | 11 | 0 | 100.0% | fully covered |
| winterbaume-amp | 17 | 17 | 0 | 100.0% | fully covered |
| winterbaume-amplify | 23 | 21 | 2 | 91.3% | partial |
| winterbaume-amplifybackend | 4 | 4 | 0 | 100.0% | fully covered |
| winterbaume-amplifyuibuilder | 28 | 18 | 10 | 64.3% | partial |
| winterbaume-apigateway | 117 | 65 | 52 | 55.6% | partial |
| winterbaume-apigatewaymanagement | 3 | 3 | 0 | 100.0% | fully covered |
| winterbaume-apigatewayv2 | 62 | 59 | 3 | 95.2% | partial |
| winterbaume-appconfig | 45 | 37 | 8 | 82.2% | partial |
| winterbaume-appconfigdata | 2 | 2 | 0 | 100.0% | fully covered |
| winterbaume-appfabric | 6 | 6 | 0 | 100.0% | fully covered |
| winterbaume-appflow | 9 | 6 | 3 | 66.7% | partial |
| winterbaume-appintegrations | 23 | 21 | 2 | 91.3% | partial |
| winterbaume-applicationautoscaling | 13 | 13 | 0 | 100.0% | fully covered |
| winterbaume-applicationcostprofiler | 6 | 6 | 0 | 100.0% | fully covered |
| winterbaume-applicationdiscoveryservice | 28 | 20 | 8 | 71.4% | partial |
| winterbaume-applicationinsights | 33 | 26 | 7 | 78.8% | partial |
| winterbaume-applicationsignals | 10 | 8 | 2 | 80.0% | partial |
| winterbaume-appmesh | 38 | 36 | 2 | 94.7% | partial |
| winterbaume-apprunner | 23 | 19 | 4 | 82.6% | partial |
| winterbaume-appsync | 27 | 27 | 0 | 100.0% | fully covered |
| winterbaume-arczonalshift | 14 | 14 | 0 | 100.0% | fully covered |
| winterbaume-artifact | 8 | 5 | 3 | 62.5% | partial |
| winterbaume-athena | 25 | 25 | 0 | 100.0% | fully covered |
| winterbaume-auditmanager | 15 | 15 | 0 | 100.0% | fully covered |
| winterbaume-autoscaling | 52 | 49 | 3 | 94.2% | partial |
| winterbaume-autoscalingplans | 6 | 5 | 1 | 83.3% | partial |
| winterbaume-backup | 105 | 54 | 51 | 51.4% | partial |
| winterbaume-backupgateway | 25 | 20 | 5 | 80.0% | partial |
| winterbaume-backupsearch | 9 | 9 | 0 | 100.0% | fully covered |
| winterbaume-batch | 39 | 39 | 0 | 100.0% | fully covered |
| winterbaume-bcmdashboards | 9 | 9 | 0 | 100.0% | fully covered |
| winterbaume-bcmdataexports | 12 | 10 | 2 | 83.3% | partial |
| winterbaume-bcmrecommendedactions | 1 | 1 | 0 | 100.0% | fully covered |
| winterbaume-bedrock | 48 | 43 | 5 | 89.6% | partial |
| winterbaume-bedrockagent | 72 | 71 | 1 | 98.6% | partial |
| winterbaume-billing | 12 | 11 | 1 | 91.7% | partial |
| winterbaume-braket | 12 | 8 | 4 | 66.7% | partial |
| winterbaume-budgets | 7 | 7 | 0 | 100.0% | fully covered |
| winterbaume-chatbot | 15 | 15 | 0 | 100.0% | fully covered |
| winterbaume-chimesdkmeetings | 12 | 8 | 4 | 66.7% | partial |
| winterbaume-cloud9 | 13 | 13 | 0 | 100.0% | fully covered |
| winterbaume-cloudcontrol | 8 | 8 | 0 | 100.0% | fully covered |
| winterbaume-clouddirectory | 13 | 12 | 1 | 92.3% | partial |
| winterbaume-cloudformation | 40 | 39 | 1 | 97.5% | partial |
| winterbaume-cloudfront | 156 | 69 | 87 | 44.2% | partial |
| winterbaume-cloudfrontkeyvaluestore | 5 | 5 | 0 | 100.0% | fully covered |
| winterbaume-cloudhsmv2 | 18 | 18 | 0 | 100.0% | fully covered |
| winterbaume-cloudsearchdomain | 2 | 2 | 0 | 100.0% | fully covered |
| winterbaume-cloudtrail | 21 | 16 | 5 | 76.2% | partial |
| winterbaume-cloudtraildata | 1 | 1 | 0 | 100.0% | fully covered |
| winterbaume-cloudwatch | 38 | 29 | 9 | 76.3% | partial |
| winterbaume-cloudwatchlogs | 93 | 84 | 9 | 90.3% | partial |
| winterbaume-codeartifact | 9 | 9 | 0 | 100.0% | fully covered |
| winterbaume-codebuild | 29 | 26 | 3 | 89.7% | partial |
| winterbaume-codecommit | 25 | 25 | 0 | 100.0% | fully covered |
| winterbaume-codedeploy | 15 | 15 | 0 | 100.0% | fully covered |
| winterbaume-codegurureviewer | 9 | 9 | 0 | 100.0% | fully covered |
| winterbaume-codegurusecurity | 11 | 9 | 2 | 81.8% | partial |
| winterbaume-codepipeline | 44 | 8 | 36 | 18.2% | partial |
| winterbaume-codestarnotifications | 7 | 5 | 2 | 71.4% | partial |
| winterbaume-cognitoidentity | 20 | 18 | 2 | 90.0% | partial |
| winterbaume-cognitoidentityprovider | 104 | 76 | 28 | 73.1% | partial |
| winterbaume-cognitosync | 11 | 6 | 5 | 54.5% | partial |
| winterbaume-comprehend | 60 | 60 | 0 | 100.0% | fully covered |
| winterbaume-config | 46 | 46 | 0 | 100.0% | fully covered |
| winterbaume-connect | 10 | 10 | 0 | 100.0% | fully covered |
| winterbaume-connectcampaigns | 14 | 14 | 0 | 100.0% | fully covered |
| winterbaume-connectcontactlens | 0 | 0 | 0 | 0.0% | fully covered |
| winterbaume-connectparticipant | 7 | 5 | 2 | 71.4% | partial |
| winterbaume-controlcatalog | 6 | 5 | 1 | 83.3% | partial |
| winterbaume-costandusagereport | 7 | 7 | 0 | 100.0% | fully covered |
| winterbaume-costexplorer | 22 | 21 | 1 | 95.5% | partial |
| winterbaume-costoptimizationhub | 8 | 8 | 0 | 100.0% | fully covered |
| winterbaume-databasemigration | 42 | 41 | 1 | 97.6% | partial |
| winterbaume-databrew | 32 | 32 | 0 | 100.0% | fully covered |
| winterbaume-datapipeline | 19 | 17 | 2 | 89.5% | partial |
| winterbaume-datasync | 8 | 8 | 0 | 100.0% | fully covered |
| winterbaume-dax | 6 | 6 | 0 | 100.0% | fully covered |
| winterbaume-directconnect | 7 | 7 | 0 | 100.0% | fully covered |
| winterbaume-directory | 4 | 4 | 0 | 100.0% | fully covered |
| winterbaume-dlm | 2 | 2 | 0 | 100.0% | fully covered |
| winterbaume-dsql | 12 | 12 | 0 | 100.0% | fully covered |
| winterbaume-dynamodb | 57 | 57 | 0 | 100.0% | fully covered |
| winterbaume-dynamodbstreams | 4 | 4 | 0 | 100.0% | fully covered |
| winterbaume-ebs | 6 | 6 | 0 | 100.0% | fully covered |
| winterbaume-ec2 | 713 | 669 | 44 | 93.8% | partial |
| winterbaume-ec2instanceconnect | 2 | 2 | 0 | 100.0% | fully covered |
| winterbaume-ecr | 58 | 44 | 14 | 75.9% | partial |
| winterbaume-ecs | 63 | 58 | 5 | 92.1% | partial |
| winterbaume-efs | 31 | 31 | 0 | 100.0% | fully covered |
| winterbaume-eks | 55 | 41 | 14 | 74.5% | partial |
| winterbaume-elasticache | 24 | 24 | 0 | 100.0% | fully covered |
| winterbaume-elasticbeanstalk | 7 | 7 | 0 | 100.0% | fully covered |
| winterbaume-elasticloadbalancing | 29 | 21 | 8 | 72.4% | partial |
| winterbaume-elasticloadbalancingv2 | 50 | 38 | 12 | 76.0% | partial |
| winterbaume-emr | 54 | 34 | 20 | 63.0% | partial |
| winterbaume-emrcontainers | 23 | 23 | 0 | 100.0% | fully covered |
| winterbaume-emrserverless | 16 | 14 | 2 | 87.5% | partial |
| winterbaume-eventbridge | 57 | 45 | 12 | 78.9% | partial |
| winterbaume-firehose | 12 | 12 | 0 | 100.0% | fully covered |
| winterbaume-fis | 8 | 8 | 0 | 100.0% | fully covered |
| winterbaume-forecast | 5 | 5 | 0 | 100.0% | fully covered |
| winterbaume-freetier | 5 | 5 | 0 | 100.0% | fully covered |
| winterbaume-fsx | 9 | 9 | 0 | 100.0% | fully covered |
| winterbaume-glacier | 33 | 30 | 3 | 90.9% | partial |
| winterbaume-glue | 132 | 127 | 5 | 96.2% | partial |
| winterbaume-greengrass | 71 | 71 | 0 | 100.0% | fully covered |
| winterbaume-guardduty | 85 | 78 | 7 | 91.8% | partial |
| winterbaume-iam | 154 | 135 | 19 | 87.7% | partial |
| winterbaume-identitystore | 17 | 14 | 3 | 82.4% | partial |
| winterbaume-inspector2 | 49 | 38 | 11 | 77.6% | partial |
| winterbaume-iot | 103 | 99 | 4 | 96.1% | partial |
| winterbaume-iotdataplane | 8 | 5 | 3 | 62.5% | partial |
| winterbaume-ivs | 30 | 30 | 0 | 100.0% | fully covered |
| winterbaume-kafka | 10 | 10 | 0 | 100.0% | fully covered |
| winterbaume-keyspaces | 19 | 13 | 6 | 68.4% | partial |
| winterbaume-kinesis | 38 | 31 | 7 | 81.6% | partial |
| winterbaume-kinesisanalyticsv2 | 32 | 25 | 7 | 78.1% | partial |
| winterbaume-kinesisvideo | 32 | 32 | 0 | 100.0% | fully covered |
| winterbaume-kinesisvideoarchivedmedia | 6 | 6 | 0 | 100.0% | fully covered |
| winterbaume-kms | 53 | 40 | 13 | 75.5% | partial |
| winterbaume-lakeformation | 19 | 17 | 2 | 89.5% | partial |
| winterbaume-lambda | 85 | 60 | 25 | 70.6% | partial |
| winterbaume-lexmodelsv2 | 58 | 58 | 0 | 100.0% | fully covered |
| winterbaume-macie2 | 67 | 59 | 8 | 88.1% | partial |
| winterbaume-managedblockchain | 27 | 27 | 0 | 100.0% | fully covered |
| winterbaume-marketplacemetering | 4 | 4 | 0 | 100.0% | fully covered |
| winterbaume-mediaconnect | 21 | 21 | 0 | 100.0% | fully covered |
| winterbaume-medialive | 16 | 12 | 4 | 75.0% | partial |
| winterbaume-mediapackage | 9 | 9 | 0 | 100.0% | fully covered |
| winterbaume-mediapackagev2 | 7 | 7 | 0 | 100.0% | fully covered |
| winterbaume-mediastore | 11 | 11 | 0 | 100.0% | fully covered |
| winterbaume-mediastoredata | 5 | 5 | 0 | 100.0% | fully covered |
| winterbaume-memorydb | 13 | 13 | 0 | 100.0% | fully covered |
| winterbaume-mq | 23 | 21 | 2 | 91.3% | partial |
| winterbaume-neptune | 64 | 24 | 40 | 37.5% | partial |
| winterbaume-networkfirewall | 79 | 42 | 37 | 53.2% | partial |
| winterbaume-networkmanager | 53 | 44 | 9 | 83.0% | partial |
| winterbaume-opensearch | 44 | 35 | 9 | 79.5% | partial |
| winterbaume-opensearchserverless | 12 | 12 | 0 | 100.0% | fully covered |
| winterbaume-organizations | 60 | 54 | 6 | 90.0% | partial |
| winterbaume-osis | 10 | 10 | 0 | 100.0% | fully covered |
| winterbaume-outposts | 13 | 13 | 0 | 100.0% | fully covered |
| winterbaume-panorama | 10 | 10 | 0 | 100.0% | fully covered |
| winterbaume-pcaconnectorscep | 11 | 9 | 2 | 81.8% | partial |
| winterbaume-personalize | 66 | 8 | 58 | 12.1% | partial |
| winterbaume-personalizeevents | 5 | 3 | 2 | 60.0% | partial |
| winterbaume-personalizeruntime | 3 | 3 | 0 | 100.0% | fully covered |
| winterbaume-pi | 9 | 7 | 2 | 77.8% | partial |
| winterbaume-pinpoint | 15 | 12 | 3 | 80.0% | partial |
| winterbaume-pinpointsmsvoice | 4 | 4 | 0 | 100.0% | fully covered |
| winterbaume-pipes | 10 | 7 | 3 | 70.0% | partial |
| winterbaume-polly | 9 | 9 | 0 | 100.0% | fully covered |
| winterbaume-pricing | 5 | 5 | 0 | 100.0% | fully covered |
| winterbaume-quicksight | 68 | 66 | 2 | 97.1% | partial |
| winterbaume-ram | 35 | 32 | 3 | 91.4% | partial |
| winterbaume-rbin | 9 | 9 | 0 | 100.0% | fully covered |
| winterbaume-rds | 146 | 78 | 68 | 53.4% | partial |
| winterbaume-rdsdata | 6 | 6 | 0 | 100.0% | fully covered |
| winterbaume-redshift | 100 | 99 | 1 | 99.0% | partial |
| winterbaume-redshiftdata | 11 | 11 | 0 | 100.0% | fully covered |
| winterbaume-rekognition | 8 | 8 | 0 | 100.0% | fully covered |
| winterbaume-resiliencehub | 22 | 22 | 0 | 100.0% | fully covered |
| winterbaume-resourcegroups | 22 | 15 | 7 | 68.2% | partial |
| winterbaume-resourcegroupstagging | 5 | 5 | 0 | 100.0% | fully covered |
| winterbaume-rolesanywhere | 28 | 22 | 6 | 78.6% | partial |
| winterbaume-route53 | 71 | 70 | 1 | 98.6% | partial |
| winterbaume-route53domains | 5 | 5 | 0 | 100.0% | fully covered |
| winterbaume-route53recoverycluster | 4 | 4 | 0 | 100.0% | fully covered |
| winterbaume-route53resolver | 28 | 28 | 0 | 100.0% | fully covered |
| winterbaume-s3 | 103 | 70 | 33 | 68.0% | partial |
| winterbaume-s3control | 87 | 68 | 19 | 78.2% | partial |
| winterbaume-s3files | 21 | 21 | 0 | 100.0% | fully covered |
| winterbaume-s3outposts | 3 | 3 | 0 | 100.0% | fully covered |
| winterbaume-s3tables | 46 | 46 | 0 | 100.0% | fully covered |
| winterbaume-s3vectors | 19 | 19 | 0 | 100.0% | fully covered |
| winterbaume-sagemaker | 141 | 112 | 29 | 79.4% | partial |
| winterbaume-sagemakermetrics | 2 | 2 | 0 | 100.0% | fully covered |
| winterbaume-sagemakerruntime | 3 | 3 | 0 | 100.0% | fully covered |
| winterbaume-savingsplans | 10 | 8 | 2 | 80.0% | partial |
| winterbaume-scheduler | 12 | 12 | 0 | 100.0% | fully covered |
| winterbaume-secretsmanager | 22 | 21 | 1 | 95.5% | partial |
| winterbaume-securityhub | 97 | 94 | 3 | 96.9% | partial |
| winterbaume-servicecatalog | 4 | 4 | 0 | 100.0% | fully covered |
| winterbaume-servicecatalogappregistry | 23 | 23 | 0 | 100.0% | fully covered |
| winterbaume-servicediscovery | 27 | 27 | 0 | 100.0% | fully covered |
| winterbaume-servicequotas | 5 | 5 | 0 | 100.0% | fully covered |
| winterbaume-ses | 38 | 28 | 10 | 73.7% | partial |
| winterbaume-sesv2 | 106 | 91 | 15 | 85.8% | partial |
| winterbaume-sfn | 35 | 35 | 0 | 100.0% | fully covered |
| winterbaume-shield | 9 | 9 | 0 | 100.0% | fully covered |
| winterbaume-signer | 19 | 19 | 0 | 100.0% | fully covered |
| winterbaume-simpledbv2 | 3 | 3 | 0 | 100.0% | fully covered |
| winterbaume-simspaceweaver | 15 | 11 | 4 | 73.3% | partial |
| winterbaume-snowdevicemanagement | 11 | 7 | 4 | 63.6% | partial |
| winterbaume-sns | 41 | 36 | 5 | 87.8% | partial |
| winterbaume-sqs | 23 | 22 | 1 | 95.7% | partial |
| winterbaume-ssm | 127 | 125 | 2 | 98.4% | partial |
| winterbaume-ssmquicksetup | 6 | 2 | 4 | 33.3% | partial |
| winterbaume-sso | 4 | 4 | 0 | 100.0% | fully covered |
| winterbaume-ssoadmin | 27 | 23 | 4 | 85.2% | partial |
| winterbaume-sts | 11 | 10 | 1 | 90.9% | partial |
| winterbaume-support | 5 | 5 | 0 | 100.0% | fully covered |
| winterbaume-supportapp | 3 | 3 | 0 | 100.0% | fully covered |
| winterbaume-swf | 30 | 30 | 0 | 100.0% | fully covered |
| winterbaume-synthetics | 22 | 22 | 0 | 100.0% | fully covered |
| winterbaume-taxsettings | 0 | 0 | 0 | 0.0% | fully covered |
| winterbaume-textract | 6 | 6 | 0 | 100.0% | fully covered |
| winterbaume-timestreaminfluxdb | 19 | 13 | 6 | 68.4% | partial |
| winterbaume-timestreamquery | 15 | 15 | 0 | 100.0% | fully covered |
| winterbaume-timestreamwrite | 19 | 15 | 4 | 78.9% | partial |
| winterbaume-transcribe | 16 | 16 | 0 | 100.0% | fully covered |
| winterbaume-transfer | 44 | 40 | 4 | 90.9% | partial |
| winterbaume-trustedadvisor | 6 | 3 | 3 | 50.0% | partial |
| winterbaume-vpclattice | 66 | 62 | 4 | 93.9% | partial |
| winterbaume-wafv2 | 38 | 31 | 7 | 81.6% | partial |
| winterbaume-workspaces | 50 | 48 | 2 | 96.0% | partial |
| winterbaume-workspacesweb | 68 | 63 | 5 | 92.6% | partial |
| winterbaume-xray | 34 | 26 | 8 | 76.5% | partial |

## Terraform E2E Coverage

Legend: `Tests` = success-oriented terraform tests, `Resources` = distinct terraform resource types exercised, `Diag` = diagnostic-only terraform tests

| Service | Tests | Resources | Diag | Status |
|---|---|---|---|---|
| winterbaume-acm | 3 | 1 | 0 | covered |
| winterbaume-apigatewayv2 | 15 | 9 | 0 | covered |
| winterbaume-appconfig | 8 | 7 | 0 | covered |
| winterbaume-autoscaling | 10 | 5 | 1 | covered |
| winterbaume-batch | 8 | 4 | 0 | covered |
| winterbaume-bedrockagent | 9 | 6 | 0 | covered |
| winterbaume-cloudfront | 6 | 2 | 0 | covered |
| winterbaume-cloudwatch | 12 | 3 | 0 | covered |
| winterbaume-cloudwatchlogs | 5 | 3 | 0 | covered |
| winterbaume-codecommit | 4 | 1 | 0 | covered |
| winterbaume-cognitoidentityprovider | 15 | 7 | 0 | covered |
| winterbaume-config | 7 | 7 | 0 | covered |
| winterbaume-dynamodb | 8 | 2 | 0 | covered |
| winterbaume-ec2 | 20 | 7 | 1 | covered |
| winterbaume-ecr | 8 | 6 | 0 | covered |
| winterbaume-ecs | 7 | 3 | 1 | covered |
| winterbaume-efs | 7 | 3 | 0 | covered |
| winterbaume-eks | 11 | 6 | 0 | covered |
| winterbaume-elasticloadbalancingv2 | 12 | 6 | 0 | covered |
| winterbaume-eventbridge | 7 | 4 | 0 | covered |
| winterbaume-firehose | 3 | 1 | 0 | covered |
| winterbaume-iam | 30 | 11 | 0 | covered |
| winterbaume-kinesis | 4 | 1 | 0 | covered |
| winterbaume-kms | 3 | 2 | 0 | covered |
| winterbaume-lambda | 9 | 7 | 0 | covered |
| winterbaume-networkmanager | 8 | 6 | 0 | covered |
| winterbaume-organizations | 4 | 4 | 0 | covered |
| winterbaume-rds | 6 | 2 | 1 | covered |
| winterbaume-route53 | 6 | 2 | 0 | covered |
| winterbaume-s3 | 13 | 6 | 0 | covered |
| winterbaume-scheduler | 7 | 2 | 0 | covered |
| winterbaume-secretsmanager | 3 | 2 | 0 | covered |
| winterbaume-sfn | 10 | 3 | 0 | covered |
| winterbaume-sns | 4 | 2 | 0 | covered |
| winterbaume-sqs | 10 | 2 | 0 | covered |
| winterbaume-ssm | 12 | 6 | 0 | covered |
| winterbaume-transfer | 8 | 4 | 0 | covered |
| winterbaume-vpclattice | 18 | 9 | 0 | covered |
| winterbaume-wafv2 | 12 | 4 | 0 | covered |
| winterbaume-workspaces | 4 | 2 | 0 | covered |

## Services Not Yet Implemented

- appstream
- arc-region-switch
- b2bi
- bcm-pricing-calculator
- bedrock-agent-runtime
- bedrock-agentcore
- bedrock-agentcore-control
- bedrock-data-automation
- bedrock-data-automation-runtime
- bedrock-runtime
- billingconductor
- chime
- chime-sdk-identity
- chime-sdk-media-pipelines
- chime-sdk-messaging
- chime-sdk-voice
- cleanrooms
- cleanroomsml
- cloudhsm
- cloudsearch
- cloudwatch-events
- codecatalyst
- codeconnections
- codeguruprofiler
- codestar-connections
- comprehendmedical
- compute-optimizer
- compute-optimizer-automation
- connectcampaignsv2
- connectcases
- connecthealth
- controltower
- customer-profiles
- dataexchange
- datazone
- deadline
- detective
- device-farm
- devops-agent
- devops-guru
- directory-service-data
- docdb
- docdb-elastic
- drs
- ecr-public
- eks-auth
- elasticsearch-service
- elementalinference
- entityresolution
- evs
- finspace
- finspace-data
- fms
- forecastquery
- frauddetector
- gamelift
- gameliftstreams
- geo-maps
- geo-places
- geo-routes
- global-accelerator
- grafana
- greengrassv2
- groundstation
- health
- healthlake
- imagebuilder
- inspector
- inspector-scan
- interconnect
- internetmonitor
- invoicing
- iot-events
- iot-events-data
- iot-jobs-data-plane
- iot-managed-integrations
- iot-wireless
- iotdeviceadvisor
- iotfleetwise
- iotsecuretunneling
- iotsitewise
- iotthingsgraph
- iottwinmaker
- ivs-realtime
- ivschat
- kafkaconnect
- kendra
- kendra-ranking
- keyspacesstreams
- kinesis-analytics
- kinesis-video-media
- kinesis-video-signaling
- kinesis-video-webrtc-storage
- launch-wizard
- lex-model-building-service
- lex-runtime-service
- lex-runtime-v2
- license-manager
- license-manager-linux-subscriptions
- license-manager-user-subscriptions
- lightsail
- location
- lookoutequipment
- m2
- machine-learning
- mailmanager
- managedblockchain-query
- marketplace-agreement
- marketplace-catalog
- marketplace-commerce-analytics
- marketplace-deployment
- marketplace-discovery
- marketplace-entitlement-service
- marketplace-reporting
- mediaconvert
- mediapackage-vod
- mediatailor
- medical-imaging
- mgn
- migration-hub
- migration-hub-refactor-spaces
- migrationhub-config
- migrationhuborchestrator
- migrationhubstrategy
- mpa
- mturk
- mwaa
- mwaa-serverless
- neptune-graph
- neptunedata
- networkflowmonitor
- networkmonitor
- notifications
- notificationscontacts
- nova-act
- oam
- observabilityadmin
- odb
- omics
- partnercentral-account
- partnercentral-benefits
- partnercentral-channel
- partnercentral-selling
- payment-cryptography
- payment-cryptography-data
- pca-connector-ad
- pcs
- pinpoint-email
- pinpoint-sms-voice-v2
- proton
- qapps
- qbusiness
- qconnect
- redshift-serverless
- repostspace
- resource-explorer-2
- route53-recovery-control-config
- route53-recovery-readiness
- route53globalresolver
- route53profiles
- rtbfabric
- rum
- sagemaker-a2i-runtime
- sagemaker-edge
- sagemaker-featurestore-runtime
- sagemaker-geospatial
- sagemaker-runtime-http2
- schemas
- security-ir
- securityagent
- securitylake
- serverlessapplicationrepository
- signer-data
- signin
- snowball
- socialmessaging
- ssm-contacts
- ssm-guiconnect
- ssm-incidents
- ssm-sap
- sso-oidc
- storage-gateway
- supplychain
- sustainability
- tnb
- transcribe-streaming
- translate
- uxc
- verifiedpermissions
- voice-id
- waf
- waf-regional
- wellarchitected
- wickr
- wisdom
- workdocs
- workmail
- workmailmessageflow
- workspaces-instances
- workspaces-thin-client

## Detailed Coverage

Legend: `W` = winterbaume (real implementation), `S` = stub (routed but returns empty/default), `M` = moto, `F` = floci, `K` = kumo

### winterbaume-accessanalyzer (accessanalyzer) - W: 11/37, S: 0/37, M: 0/37, F: 0/37, K: 0/37

- W[ ] S[ ] M[ ] F[ ] K[ ] ApplyArchiveRule
- W[ ] S[ ] M[ ] F[ ] K[ ] CancelPolicyGeneration
- W[ ] S[ ] M[ ] F[ ] K[ ] CheckAccessNotGranted
- W[ ] S[ ] M[ ] F[ ] K[ ] CheckNoNewAccess
- W[ ] S[ ] M[ ] F[ ] K[ ] CheckNoPublicAccess
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateAccessPreview
- W[x] S[ ] M[ ] F[ ] K[ ] CreateAnalyzer
- W[x] S[ ] M[ ] F[ ] K[ ] CreateArchiveRule
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteAnalyzer
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteArchiveRule
- W[ ] S[ ] M[ ] F[ ] K[ ] GenerateFindingRecommendation
- W[ ] S[ ] M[ ] F[ ] K[ ] GetAccessPreview
- W[ ] S[ ] M[ ] F[ ] K[ ] GetAnalyzedResource
- W[x] S[ ] M[ ] F[ ] K[ ] GetAnalyzer
- W[x] S[ ] M[ ] F[ ] K[ ] GetArchiveRule
- W[ ] S[ ] M[ ] F[ ] K[ ] GetFinding
- W[ ] S[ ] M[ ] F[ ] K[ ] GetFindingRecommendation
- W[ ] S[ ] M[ ] F[ ] K[ ] GetFindingV2
- W[ ] S[ ] M[ ] F[ ] K[ ] GetFindingsStatistics
- W[ ] S[ ] M[ ] F[ ] K[ ] GetGeneratedPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] ListAccessPreviewFindings
- W[ ] S[ ] M[ ] F[ ] K[ ] ListAccessPreviews
- W[ ] S[ ] M[ ] F[ ] K[ ] ListAnalyzedResources
- W[x] S[ ] M[ ] F[ ] K[ ] ListAnalyzers
- W[x] S[ ] M[ ] F[ ] K[ ] ListArchiveRules
- W[ ] S[ ] M[ ] F[ ] K[ ] ListFindings
- W[ ] S[ ] M[ ] F[ ] K[ ] ListFindingsV2
- W[ ] S[ ] M[ ] F[ ] K[ ] ListPolicyGenerations
- W[x] S[ ] M[ ] F[ ] K[ ] ListTagsForResource
- W[ ] S[ ] M[ ] F[ ] K[ ] StartPolicyGeneration
- W[ ] S[ ] M[ ] F[ ] K[ ] StartResourceScan
- W[x] S[ ] M[ ] F[ ] K[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UntagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateAnalyzer
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateArchiveRule
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateFindings
- W[ ] S[ ] M[ ] F[ ] K[ ] ValidatePolicy

Integration tests: 11/11 implemented operations tested (100.0%)

### winterbaume-account (account) - W: 14/15, S: 1/15, M: 3/15, F: 0/15, K: 0/15

- W[x] S[ ] M[ ] F[ ] K[ ] AcceptPrimaryEmailUpdate
- W[x] S[ ] M[x] F[ ] K[ ] DeleteAlternateContact
- W[x] S[ ] M[ ] F[ ] K[ ] DisableRegion
- W[x] S[ ] M[ ] F[ ] K[ ] EnableRegion
- W[x] S[ ] M[ ] F[ ] K[ ] GetAccountInformation
- W[x] S[ ] M[x] F[ ] K[ ] GetAlternateContact
- W[x] S[ ] M[ ] F[ ] K[ ] GetContactInformation
- W[ ] S[x] M[ ] F[ ] K[ ] GetGovCloudAccountInformation
- W[x] S[ ] M[ ] F[ ] K[ ] GetPrimaryEmail
- W[x] S[ ] M[ ] F[ ] K[ ] GetRegionOptStatus
- W[x] S[ ] M[ ] F[ ] K[ ] ListRegions
- W[x] S[ ] M[ ] F[ ] K[ ] PutAccountName
- W[x] S[ ] M[x] F[ ] K[ ] PutAlternateContact
- W[x] S[ ] M[ ] F[ ] K[ ] PutContactInformation
- W[x] S[ ] M[ ] F[ ] K[ ] StartPrimaryEmailUpdate

Integration tests: 14/14 implemented operations tested (100.0%)

### winterbaume-acm (acm) - W: 16/17, S: 0/17, M: 11/17, F: 0/17, K: 6/17

Terraform E2E: 3 tests across 1 terraform resource types

Resource types: aws_acm_certificate

- W[x] S[ ] M[x] F[ ] K[ ] AddTagsToCertificate
- W[x] S[ ] M[x] F[ ] K[x] DeleteCertificate
- W[x] S[ ] M[x] F[ ] K[x] DescribeCertificate
- W[x] S[ ] M[x] F[ ] K[ ] ExportCertificate
- W[x] S[ ] M[x] F[ ] K[ ] GetAccountConfiguration
- W[x] S[ ] M[x] F[ ] K[x] GetCertificate
- W[x] S[ ] M[x] F[ ] K[x] ImportCertificate
- W[x] S[ ] M[x] F[ ] K[x] ListCertificates
- W[x] S[ ] M[ ] F[ ] K[ ] ListTagsForCertificate
- W[x] S[ ] M[x] F[ ] K[ ] PutAccountConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] RemoveTagsFromCertificate
- W[x] S[ ] M[ ] F[ ] K[ ] RenewCertificate
- W[x] S[ ] M[x] F[ ] K[x] RequestCertificate
- W[x] S[ ] M[ ] F[ ] K[ ] ResendValidationEmail
- W[x] S[ ] M[ ] F[ ] K[ ] RevokeCertificate
- W[ ] S[ ] M[ ] F[ ] K[ ] SearchCertificates
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateCertificateOptions

Integration tests: 12/16 implemented operations tested (75.0%)
Untested implemented operations: 4

### winterbaume-acmpca (acm-pca) - W: 23/23, S: 0/23, M: 17/23, F: 0/23, K: 0/23

- W[x] S[ ] M[x] F[ ] K[ ] CreateCertificateAuthority
- W[x] S[ ] M[ ] F[ ] K[ ] CreateCertificateAuthorityAuditReport
- W[x] S[ ] M[ ] F[ ] K[ ] CreatePermission
- W[x] S[ ] M[x] F[ ] K[ ] DeleteCertificateAuthority
- W[x] S[ ] M[ ] F[ ] K[ ] DeletePermission
- W[x] S[ ] M[x] F[ ] K[ ] DeletePolicy
- W[x] S[ ] M[x] F[ ] K[ ] DescribeCertificateAuthority
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeCertificateAuthorityAuditReport
- W[x] S[ ] M[x] F[ ] K[ ] GetCertificate
- W[x] S[ ] M[x] F[ ] K[ ] GetCertificateAuthorityCertificate
- W[x] S[ ] M[x] F[ ] K[ ] GetCertificateAuthorityCsr
- W[x] S[ ] M[x] F[ ] K[ ] GetPolicy
- W[x] S[ ] M[x] F[ ] K[ ] ImportCertificateAuthorityCertificate
- W[x] S[ ] M[x] F[ ] K[ ] IssueCertificate
- W[x] S[ ] M[x] F[ ] K[ ] ListCertificateAuthorities
- W[x] S[ ] M[ ] F[ ] K[ ] ListPermissions
- W[x] S[ ] M[x] F[ ] K[ ] ListTags
- W[x] S[ ] M[x] F[ ] K[ ] PutPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] RestoreCertificateAuthority
- W[x] S[ ] M[x] F[ ] K[ ] RevokeCertificate
- W[x] S[ ] M[x] F[ ] K[ ] TagCertificateAuthority
- W[x] S[ ] M[x] F[ ] K[ ] UntagCertificateAuthority
- W[x] S[ ] M[x] F[ ] K[ ] UpdateCertificateAuthority

Integration tests: 22/23 implemented operations tested (95.7%)
Untested implemented operations: 1

### winterbaume-aiops (aiops) - W: 11/11, S: 0/11, M: 0/11, F: 0/11, K: 0/11

- W[x] S[ ] M[ ] F[ ] K[ ] CreateInvestigationGroup
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteInvestigationGroup
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteInvestigationGroupPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] GetInvestigationGroup
- W[x] S[ ] M[ ] F[ ] K[ ] GetInvestigationGroupPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] ListInvestigationGroups
- W[x] S[ ] M[ ] F[ ] K[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] PutInvestigationGroupPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateInvestigationGroup

Integration tests: 11/11 implemented operations tested (100.0%)

### winterbaume-amp (amp) - W: 17/44, S: 0/44, M: 17/44, F: 0/44, K: 0/44

- W[ ] S[ ] M[ ] F[ ] K[ ] CreateAlertManagerDefinition
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateAnomalyDetector
- W[x] S[ ] M[x] F[ ] K[ ] CreateLoggingConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateQueryLoggingConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] CreateRuleGroupsNamespace
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateScraper
- W[x] S[ ] M[x] F[ ] K[ ] CreateWorkspace
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteAlertManagerDefinition
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteAnomalyDetector
- W[x] S[ ] M[x] F[ ] K[ ] DeleteLoggingConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteQueryLoggingConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteResourcePolicy
- W[x] S[ ] M[x] F[ ] K[ ] DeleteRuleGroupsNamespace
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteScraper
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteScraperLoggingConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] DeleteWorkspace
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeAlertManagerDefinition
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeAnomalyDetector
- W[x] S[ ] M[x] F[ ] K[ ] DescribeLoggingConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeQueryLoggingConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeResourcePolicy
- W[x] S[ ] M[x] F[ ] K[ ] DescribeRuleGroupsNamespace
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeScraper
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeScraperLoggingConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] DescribeWorkspace
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeWorkspaceConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] GetDefaultScraperConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] ListAnomalyDetectors
- W[x] S[ ] M[x] F[ ] K[ ] ListRuleGroupsNamespaces
- W[ ] S[ ] M[ ] F[ ] K[ ] ListScrapers
- W[x] S[ ] M[x] F[ ] K[ ] ListTagsForResource
- W[x] S[ ] M[x] F[ ] K[ ] ListWorkspaces
- W[ ] S[ ] M[ ] F[ ] K[ ] PutAlertManagerDefinition
- W[ ] S[ ] M[ ] F[ ] K[ ] PutAnomalyDetector
- W[ ] S[ ] M[ ] F[ ] K[ ] PutResourcePolicy
- W[x] S[ ] M[x] F[ ] K[ ] PutRuleGroupsNamespace
- W[x] S[ ] M[x] F[ ] K[ ] TagResource
- W[x] S[ ] M[x] F[ ] K[ ] UntagResource
- W[x] S[ ] M[x] F[ ] K[ ] UpdateLoggingConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateQueryLoggingConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateScraper
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateScraperLoggingConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] UpdateWorkspaceAlias
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateWorkspaceConfiguration

Integration tests: 17/17 implemented operations tested (100.0%)

### winterbaume-amplify (amplify) - W: 23/37, S: 0/37, M: 0/37, F: 0/37, K: 9/37

- W[x] S[ ] M[ ] F[ ] K[x] CreateApp
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateBackendEnvironment
- W[x] S[ ] M[ ] F[ ] K[x] CreateBranch
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateDeployment
- W[x] S[ ] M[ ] F[ ] K[ ] CreateDomainAssociation
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateWebhook
- W[x] S[ ] M[ ] F[ ] K[x] DeleteApp
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteBackendEnvironment
- W[x] S[ ] M[ ] F[ ] K[x] DeleteBranch
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteDomainAssociation
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteJob
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteWebhook
- W[ ] S[ ] M[ ] F[ ] K[ ] GenerateAccessLogs
- W[x] S[ ] M[ ] F[ ] K[x] GetApp
- W[ ] S[ ] M[ ] F[ ] K[ ] GetArtifactUrl
- W[ ] S[ ] M[ ] F[ ] K[ ] GetBackendEnvironment
- W[x] S[ ] M[ ] F[ ] K[x] GetBranch
- W[x] S[ ] M[ ] F[ ] K[ ] GetDomainAssociation
- W[x] S[ ] M[ ] F[ ] K[ ] GetJob
- W[ ] S[ ] M[ ] F[ ] K[ ] GetWebhook
- W[x] S[ ] M[ ] F[ ] K[x] ListApps
- W[ ] S[ ] M[ ] F[ ] K[ ] ListArtifacts
- W[ ] S[ ] M[ ] F[ ] K[ ] ListBackendEnvironments
- W[x] S[ ] M[ ] F[ ] K[x] ListBranches
- W[x] S[ ] M[ ] F[ ] K[ ] ListDomainAssociations
- W[x] S[ ] M[ ] F[ ] K[ ] ListJobs
- W[x] S[ ] M[ ] F[ ] K[ ] ListTagsForResource
- W[ ] S[ ] M[ ] F[ ] K[ ] ListWebhooks
- W[ ] S[ ] M[ ] F[ ] K[ ] StartDeployment
- W[x] S[ ] M[ ] F[ ] K[ ] StartJob
- W[x] S[ ] M[ ] F[ ] K[ ] StopJob
- W[x] S[ ] M[ ] F[ ] K[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UntagResource
- W[x] S[ ] M[ ] F[ ] K[x] UpdateApp
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateBranch
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateDomainAssociation
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateWebhook

Integration tests: 21/23 implemented operations tested (91.3%)
Untested implemented operations: 2

### winterbaume-amplifybackend (amplifybackend) - W: 4/31, S: 0/31, M: 0/31, F: 0/31, K: 0/31

- W[x] S[ ] M[ ] F[ ] K[ ] CloneBackend
- W[x] S[ ] M[ ] F[ ] K[ ] CreateBackend
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateBackendAPI
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateBackendAuth
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateBackendConfig
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateBackendStorage
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateToken
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteBackend
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteBackendAPI
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteBackendAuth
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteBackendStorage
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteToken
- W[ ] S[ ] M[ ] F[ ] K[ ] GenerateBackendAPIModels
- W[x] S[ ] M[ ] F[ ] K[ ] GetBackend
- W[ ] S[ ] M[ ] F[ ] K[ ] GetBackendAPI
- W[ ] S[ ] M[ ] F[ ] K[ ] GetBackendAPIModels
- W[ ] S[ ] M[ ] F[ ] K[ ] GetBackendAuth
- W[ ] S[ ] M[ ] F[ ] K[ ] GetBackendJob
- W[ ] S[ ] M[ ] F[ ] K[ ] GetBackendStorage
- W[ ] S[ ] M[ ] F[ ] K[ ] GetToken
- W[ ] S[ ] M[ ] F[ ] K[ ] ImportBackendAuth
- W[ ] S[ ] M[ ] F[ ] K[ ] ImportBackendStorage
- W[ ] S[ ] M[ ] F[ ] K[ ] ListBackendJobs
- W[ ] S[ ] M[ ] F[ ] K[ ] ListS3Buckets
- W[ ] S[ ] M[ ] F[ ] K[ ] RemoveAllBackends
- W[ ] S[ ] M[ ] F[ ] K[ ] RemoveBackendConfig
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateBackendAPI
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateBackendAuth
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateBackendConfig
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateBackendJob
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateBackendStorage

Integration tests: 4/4 implemented operations tested (100.0%)

### winterbaume-amplifyuibuilder (amplifyuibuilder) - W: 28/28, S: 0/28, M: 0/28, F: 0/28, K: 0/28

- W[x] S[ ] M[ ] F[ ] K[ ] CreateComponent
- W[x] S[ ] M[ ] F[ ] K[ ] CreateForm
- W[x] S[ ] M[ ] F[ ] K[ ] CreateTheme
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteComponent
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteForm
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteTheme
- W[x] S[ ] M[ ] F[ ] K[ ] ExchangeCodeForToken
- W[x] S[ ] M[ ] F[ ] K[ ] ExportComponents
- W[x] S[ ] M[ ] F[ ] K[ ] ExportForms
- W[x] S[ ] M[ ] F[ ] K[ ] ExportThemes
- W[x] S[ ] M[ ] F[ ] K[ ] GetCodegenJob
- W[x] S[ ] M[ ] F[ ] K[ ] GetComponent
- W[x] S[ ] M[ ] F[ ] K[ ] GetForm
- W[x] S[ ] M[ ] F[ ] K[ ] GetMetadata
- W[x] S[ ] M[ ] F[ ] K[ ] GetTheme
- W[x] S[ ] M[ ] F[ ] K[ ] ListCodegenJobs
- W[x] S[ ] M[ ] F[ ] K[ ] ListComponents
- W[x] S[ ] M[ ] F[ ] K[ ] ListForms
- W[x] S[ ] M[ ] F[ ] K[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] ListThemes
- W[x] S[ ] M[ ] F[ ] K[ ] PutMetadataFlag
- W[x] S[ ] M[ ] F[ ] K[ ] RefreshToken
- W[x] S[ ] M[ ] F[ ] K[ ] StartCodegenJob
- W[x] S[ ] M[ ] F[ ] K[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateComponent
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateForm
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateTheme

Integration tests: 18/28 implemented operations tested (64.3%)
Untested implemented operations: 10

### winterbaume-apigateway (api-gateway) - W: 117/124, S: 2/124, M: 78/124, F: 70/124, K: 17/124

- W[x] S[ ] M[x] F[x] K[ ] CreateApiKey
- W[x] S[ ] M[x] F[x] K[ ] CreateAuthorizer
- W[x] S[ ] M[x] F[x] K[ ] CreateBasePathMapping
- W[x] S[ ] M[x] F[x] K[x] CreateDeployment
- W[x] S[ ] M[ ] F[ ] K[ ] CreateDocumentationPart
- W[x] S[ ] M[ ] F[ ] K[ ] CreateDocumentationVersion
- W[x] S[ ] M[x] F[x] K[ ] CreateDomainName
- W[x] S[ ] M[ ] F[ ] K[ ] CreateDomainNameAccessAssociation
- W[x] S[ ] M[x] F[x] K[ ] CreateModel
- W[x] S[ ] M[x] F[x] K[ ] CreateRequestValidator
- W[x] S[ ] M[x] F[x] K[x] CreateResource
- W[x] S[ ] M[x] F[x] K[ ] CreateRestApi
- W[x] S[ ] M[x] F[x] K[x] CreateStage
- W[x] S[ ] M[x] F[x] K[ ] CreateUsagePlan
- W[x] S[ ] M[x] F[x] K[ ] CreateUsagePlanKey
- W[x] S[ ] M[x] F[ ] K[ ] CreateVpcLink
- W[x] S[ ] M[x] F[ ] K[ ] DeleteApiKey
- W[x] S[ ] M[x] F[x] K[ ] DeleteAuthorizer
- W[x] S[ ] M[x] F[x] K[ ] DeleteBasePathMapping
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteClientCertificate
- W[x] S[ ] M[x] F[x] K[x] DeleteDeployment
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteDocumentationPart
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteDocumentationVersion
- W[x] S[ ] M[x] F[x] K[ ] DeleteDomainName
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteDomainNameAccessAssociation
- W[x] S[ ] M[x] F[ ] K[ ] DeleteGatewayResponse
- W[x] S[ ] M[x] F[x] K[ ] DeleteIntegration
- W[x] S[ ] M[x] F[x] K[ ] DeleteIntegrationResponse
- W[x] S[ ] M[x] F[x] K[x] DeleteMethod
- W[x] S[ ] M[x] F[ ] K[ ] DeleteMethodResponse
- W[x] S[ ] M[x] F[x] K[ ] DeleteModel
- W[x] S[ ] M[x] F[x] K[ ] DeleteRequestValidator
- W[x] S[ ] M[x] F[x] K[x] DeleteResource
- W[x] S[ ] M[x] F[x] K[ ] DeleteRestApi
- W[x] S[ ] M[x] F[x] K[x] DeleteStage
- W[x] S[ ] M[x] F[x] K[ ] DeleteUsagePlan
- W[x] S[ ] M[x] F[x] K[ ] DeleteUsagePlanKey
- W[x] S[ ] M[x] F[ ] K[ ] DeleteVpcLink
- W[x] S[ ] M[ ] F[ ] K[ ] FlushStageAuthorizersCache
- W[x] S[ ] M[ ] F[ ] K[ ] FlushStageCache
- W[x] S[ ] M[ ] F[ ] K[ ] GenerateClientCertificate
- W[x] S[ ] M[x] F[ ] K[ ] GetAccount
- W[x] S[ ] M[x] F[ ] K[ ] GetApiKey
- W[x] S[ ] M[x] F[x] K[ ] GetApiKeys
- W[x] S[ ] M[x] F[x] K[ ] GetAuthorizer
- W[x] S[ ] M[x] F[x] K[ ] GetAuthorizers
- W[x] S[ ] M[x] F[x] K[ ] GetBasePathMapping
- W[x] S[ ] M[x] F[x] K[ ] GetBasePathMappings
- W[x] S[ ] M[ ] F[ ] K[ ] GetClientCertificate
- W[x] S[ ] M[ ] F[ ] K[ ] GetClientCertificates
- W[x] S[ ] M[x] F[x] K[x] GetDeployment
- W[x] S[ ] M[x] F[x] K[x] GetDeployments
- W[x] S[ ] M[ ] F[ ] K[ ] GetDocumentationPart
- W[x] S[ ] M[ ] F[ ] K[ ] GetDocumentationParts
- W[x] S[ ] M[ ] F[ ] K[ ] GetDocumentationVersion
- W[x] S[ ] M[ ] F[ ] K[ ] GetDocumentationVersions
- W[x] S[ ] M[x] F[x] K[ ] GetDomainName
- W[x] S[ ] M[ ] F[ ] K[ ] GetDomainNameAccessAssociations
- W[x] S[ ] M[x] F[x] K[ ] GetDomainNames
- W[ ] S[x] M[ ] F[ ] K[ ] GetExport
- W[x] S[ ] M[x] F[ ] K[ ] GetGatewayResponse
- W[x] S[ ] M[x] F[ ] K[ ] GetGatewayResponses
- W[x] S[ ] M[x] F[x] K[x] GetIntegration
- W[x] S[ ] M[x] F[x] K[ ] GetIntegrationResponse
- W[x] S[ ] M[x] F[x] K[x] GetMethod
- W[x] S[ ] M[x] F[x] K[ ] GetMethodResponse
- W[x] S[ ] M[x] F[x] K[ ] GetModel
- W[ ] S[x] M[ ] F[ ] K[ ] GetModelTemplate
- W[x] S[ ] M[x] F[x] K[ ] GetModels
- W[x] S[ ] M[x] F[x] K[ ] GetRequestValidator
- W[x] S[ ] M[x] F[x] K[ ] GetRequestValidators
- W[x] S[ ] M[x] F[x] K[x] GetResource
- W[x] S[ ] M[x] F[x] K[x] GetResources
- W[x] S[ ] M[x] F[x] K[ ] GetRestApi
- W[x] S[ ] M[ ] F[x] K[ ] GetRestApis
- W[ ] S[ ] M[ ] F[ ] K[ ] GetSdk
- W[ ] S[ ] M[ ] F[ ] K[ ] GetSdkType
- W[ ] S[ ] M[ ] F[ ] K[ ] GetSdkTypes
- W[x] S[ ] M[x] F[x] K[x] GetStage
- W[x] S[ ] M[x] F[x] K[x] GetStages
- W[x] S[ ] M[ ] F[x] K[ ] GetTags
- W[x] S[ ] M[ ] F[ ] K[ ] GetUsage
- W[x] S[ ] M[x] F[ ] K[ ] GetUsagePlan
- W[x] S[ ] M[x] F[x] K[ ] GetUsagePlanKey
- W[x] S[ ] M[x] F[x] K[ ] GetUsagePlanKeys
- W[x] S[ ] M[x] F[x] K[ ] GetUsagePlans
- W[x] S[ ] M[x] F[ ] K[ ] GetVpcLink
- W[x] S[ ] M[x] F[ ] K[ ] GetVpcLinks
- W[ ] S[ ] M[ ] F[ ] K[ ] ImportApiKeys
- W[x] S[ ] M[ ] F[ ] K[ ] ImportDocumentationParts
- W[x] S[ ] M[x] F[x] K[ ] ImportRestApi
- W[x] S[ ] M[x] F[ ] K[ ] PutGatewayResponse
- W[x] S[ ] M[x] F[x] K[x] PutIntegration
- W[x] S[ ] M[x] F[x] K[ ] PutIntegrationResponse
- W[x] S[ ] M[x] F[x] K[x] PutMethod
- W[x] S[ ] M[x] F[x] K[ ] PutMethodResponse
- W[x] S[ ] M[x] F[x] K[ ] PutRestApi
- W[x] S[ ] M[ ] F[ ] K[ ] RejectDomainNameAccessAssociation
- W[x] S[ ] M[ ] F[x] K[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] TestInvokeAuthorizer
- W[x] S[ ] M[ ] F[ ] K[ ] TestInvokeMethod
- W[x] S[ ] M[ ] F[x] K[ ] UntagResource
- W[x] S[ ] M[x] F[ ] K[ ] UpdateAccount
- W[x] S[ ] M[x] F[ ] K[ ] UpdateApiKey
- W[x] S[ ] M[x] F[x] K[ ] UpdateAuthorizer
- W[x] S[ ] M[x] F[ ] K[ ] UpdateBasePathMapping
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateClientCertificate
- W[x] S[ ] M[ ] F[x] K[ ] UpdateDeployment
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateDocumentationPart
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateDocumentationVersion
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateDomainName
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateGatewayResponse
- W[x] S[ ] M[ ] F[x] K[ ] UpdateIntegration
- W[x] S[ ] M[ ] F[x] K[ ] UpdateIntegrationResponse
- W[x] S[ ] M[ ] F[x] K[ ] UpdateMethod
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateMethodResponse
- W[x] S[ ] M[ ] F[x] K[ ] UpdateModel
- W[x] S[ ] M[x] F[ ] K[ ] UpdateRequestValidator
- W[x] S[ ] M[ ] F[x] K[ ] UpdateResource
- W[x] S[ ] M[x] F[x] K[ ] UpdateRestApi
- W[x] S[ ] M[x] F[x] K[ ] UpdateStage
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateUsage
- W[x] S[ ] M[x] F[ ] K[ ] UpdateUsagePlan
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateVpcLink

Integration tests: 65/117 implemented operations tested (55.6%)
Untested implemented operations: 52

### winterbaume-apigatewaymanagement (apigatewaymanagementapi) - W: 3/3, S: 0/3, M: 3/3, F: 0/3, K: 0/3

- W[x] S[ ] M[x] F[ ] K[ ] DeleteConnection
- W[x] S[ ] M[x] F[ ] K[ ] GetConnection
- W[x] S[ ] M[x] F[ ] K[ ] PostToConnection

Integration tests: 3/3 implemented operations tested (100.0%)

### winterbaume-apigatewayv2 (apigatewayv2) - W: 62/103, S: 0/103, M: 54/103, F: 0/103, K: 0/103

Terraform E2E: 15 tests across 9 terraform resource types

Resource types: aws_apigatewayv2_api, aws_apigatewayv2_api_mapping, aws_apigatewayv2_authorizer, aws_apigatewayv2_deployment, aws_apigatewayv2_domain_name, aws_apigatewayv2_integration, aws_apigatewayv2_route, aws_apigatewayv2_stage, aws_apigatewayv2_vpc_link

- W[x] S[ ] M[x] F[ ] K[ ] CreateApi
- W[x] S[ ] M[x] F[ ] K[ ] CreateApiMapping
- W[x] S[ ] M[x] F[ ] K[ ] CreateAuthorizer
- W[x] S[ ] M[ ] F[ ] K[ ] CreateDeployment
- W[x] S[ ] M[x] F[ ] K[ ] CreateDomainName
- W[x] S[ ] M[x] F[ ] K[ ] CreateIntegration
- W[x] S[ ] M[x] F[ ] K[ ] CreateIntegrationResponse
- W[x] S[ ] M[x] F[ ] K[ ] CreateModel
- W[ ] S[ ] M[ ] F[ ] K[ ] CreatePortal
- W[ ] S[ ] M[ ] F[ ] K[ ] CreatePortalProduct
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateProductPage
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateProductRestEndpointPage
- W[x] S[ ] M[x] F[ ] K[ ] CreateRoute
- W[x] S[ ] M[x] F[ ] K[ ] CreateRouteResponse
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateRoutingRule
- W[x] S[ ] M[x] F[ ] K[ ] CreateStage
- W[x] S[ ] M[x] F[ ] K[ ] CreateVpcLink
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteAccessLogSettings
- W[x] S[ ] M[x] F[ ] K[ ] DeleteApi
- W[x] S[ ] M[x] F[ ] K[ ] DeleteApiMapping
- W[x] S[ ] M[x] F[ ] K[ ] DeleteAuthorizer
- W[x] S[ ] M[x] F[ ] K[ ] DeleteCorsConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteDeployment
- W[x] S[ ] M[x] F[ ] K[ ] DeleteDomainName
- W[x] S[ ] M[x] F[ ] K[ ] DeleteIntegration
- W[x] S[ ] M[x] F[ ] K[ ] DeleteIntegrationResponse
- W[x] S[ ] M[x] F[ ] K[ ] DeleteModel
- W[ ] S[ ] M[ ] F[ ] K[ ] DeletePortal
- W[ ] S[ ] M[ ] F[ ] K[ ] DeletePortalProduct
- W[ ] S[ ] M[ ] F[ ] K[ ] DeletePortalProductSharingPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteProductPage
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteProductRestEndpointPage
- W[x] S[ ] M[x] F[ ] K[ ] DeleteRoute
- W[x] S[ ] M[x] F[ ] K[ ] DeleteRouteRequestParameter
- W[x] S[ ] M[x] F[ ] K[ ] DeleteRouteResponse
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteRouteSettings
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteRoutingRule
- W[x] S[ ] M[x] F[ ] K[ ] DeleteStage
- W[x] S[ ] M[x] F[ ] K[ ] DeleteVpcLink
- W[ ] S[ ] M[ ] F[ ] K[ ] DisablePortal
- W[ ] S[ ] M[ ] F[ ] K[ ] ExportApi
- W[x] S[ ] M[x] F[ ] K[ ] GetApi
- W[x] S[ ] M[x] F[ ] K[ ] GetApiMapping
- W[x] S[ ] M[x] F[ ] K[ ] GetApiMappings
- W[x] S[ ] M[x] F[ ] K[ ] GetApis
- W[x] S[ ] M[x] F[ ] K[ ] GetAuthorizer
- W[x] S[ ] M[ ] F[ ] K[ ] GetAuthorizers
- W[x] S[ ] M[ ] F[ ] K[ ] GetDeployment
- W[x] S[ ] M[ ] F[ ] K[ ] GetDeployments
- W[x] S[ ] M[x] F[ ] K[ ] GetDomainName
- W[x] S[ ] M[x] F[ ] K[ ] GetDomainNames
- W[x] S[ ] M[x] F[ ] K[ ] GetIntegration
- W[x] S[ ] M[x] F[ ] K[ ] GetIntegrationResponse
- W[x] S[ ] M[x] F[ ] K[ ] GetIntegrationResponses
- W[x] S[ ] M[x] F[ ] K[ ] GetIntegrations
- W[x] S[ ] M[x] F[ ] K[ ] GetModel
- W[ ] S[ ] M[ ] F[ ] K[ ] GetModelTemplate
- W[x] S[ ] M[ ] F[ ] K[ ] GetModels
- W[ ] S[ ] M[ ] F[ ] K[ ] GetPortal
- W[ ] S[ ] M[ ] F[ ] K[ ] GetPortalProduct
- W[ ] S[ ] M[ ] F[ ] K[ ] GetPortalProductSharingPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] GetProductPage
- W[ ] S[ ] M[ ] F[ ] K[ ] GetProductRestEndpointPage
- W[x] S[ ] M[x] F[ ] K[ ] GetRoute
- W[x] S[ ] M[x] F[ ] K[ ] GetRouteResponse
- W[x] S[ ] M[ ] F[ ] K[ ] GetRouteResponses
- W[x] S[ ] M[x] F[ ] K[ ] GetRoutes
- W[ ] S[ ] M[ ] F[ ] K[ ] GetRoutingRule
- W[x] S[ ] M[x] F[ ] K[ ] GetStage
- W[x] S[ ] M[x] F[ ] K[ ] GetStages
- W[x] S[ ] M[x] F[ ] K[ ] GetTags
- W[x] S[ ] M[x] F[ ] K[ ] GetVpcLink
- W[x] S[ ] M[x] F[ ] K[ ] GetVpcLinks
- W[ ] S[ ] M[ ] F[ ] K[ ] ImportApi
- W[ ] S[ ] M[ ] F[ ] K[ ] ListPortalProducts
- W[ ] S[ ] M[ ] F[ ] K[ ] ListPortals
- W[ ] S[ ] M[ ] F[ ] K[ ] ListProductPages
- W[ ] S[ ] M[ ] F[ ] K[ ] ListProductRestEndpointPages
- W[ ] S[ ] M[ ] F[ ] K[ ] ListRoutingRules
- W[ ] S[ ] M[ ] F[ ] K[ ] PreviewPortal
- W[ ] S[ ] M[ ] F[ ] K[ ] PublishPortal
- W[ ] S[ ] M[ ] F[ ] K[ ] PutPortalProductSharingPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] PutRoutingRule
- W[x] S[ ] M[x] F[ ] K[ ] ReimportApi
- W[ ] S[ ] M[ ] F[ ] K[ ] ResetAuthorizersCache
- W[x] S[ ] M[x] F[ ] K[ ] TagResource
- W[x] S[ ] M[x] F[ ] K[ ] UntagResource
- W[x] S[ ] M[x] F[ ] K[ ] UpdateApi
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateApiMapping
- W[x] S[ ] M[x] F[ ] K[ ] UpdateAuthorizer
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateDeployment
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateDomainName
- W[x] S[ ] M[x] F[ ] K[ ] UpdateIntegration
- W[x] S[ ] M[x] F[ ] K[ ] UpdateIntegrationResponse
- W[x] S[ ] M[x] F[ ] K[ ] UpdateModel
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdatePortal
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdatePortalProduct
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateProductPage
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateProductRestEndpointPage
- W[x] S[ ] M[x] F[ ] K[ ] UpdateRoute
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateRouteResponse
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateStage
- W[x] S[ ] M[x] F[ ] K[ ] UpdateVpcLink

Integration tests: 59/62 implemented operations tested (95.2%)
Untested implemented operations: 3

### winterbaume-appconfig (appconfig) - W: 45/45, S: 0/45, M: 15/45, F: 0/45, K: 0/45

Terraform E2E: 8 tests across 7 terraform resource types

Resource types: aws_appconfig_application, aws_appconfig_configuration_profile, aws_appconfig_deployment, aws_appconfig_deployment_strategy, aws_appconfig_environment, aws_appconfig_extension, aws_appconfig_hosted_configuration_version

- W[x] S[ ] M[x] F[ ] K[ ] CreateApplication
- W[x] S[ ] M[x] F[ ] K[ ] CreateConfigurationProfile
- W[x] S[ ] M[ ] F[ ] K[ ] CreateDeploymentStrategy
- W[x] S[ ] M[ ] F[ ] K[ ] CreateEnvironment
- W[x] S[ ] M[ ] F[ ] K[ ] CreateExtension
- W[x] S[ ] M[ ] F[ ] K[ ] CreateExtensionAssociation
- W[x] S[ ] M[x] F[ ] K[ ] CreateHostedConfigurationVersion
- W[x] S[ ] M[x] F[ ] K[ ] DeleteApplication
- W[x] S[ ] M[x] F[ ] K[ ] DeleteConfigurationProfile
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteDeploymentStrategy
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteEnvironment
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteExtension
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteExtensionAssociation
- W[x] S[ ] M[x] F[ ] K[ ] DeleteHostedConfigurationVersion
- W[x] S[ ] M[ ] F[ ] K[ ] GetAccountSettings
- W[x] S[ ] M[x] F[ ] K[ ] GetApplication
- W[x] S[ ] M[ ] F[ ] K[ ] GetConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] GetConfigurationProfile
- W[x] S[ ] M[ ] F[ ] K[ ] GetDeployment
- W[x] S[ ] M[ ] F[ ] K[ ] GetDeploymentStrategy
- W[x] S[ ] M[ ] F[ ] K[ ] GetEnvironment
- W[x] S[ ] M[ ] F[ ] K[ ] GetExtension
- W[x] S[ ] M[ ] F[ ] K[ ] GetExtensionAssociation
- W[x] S[ ] M[x] F[ ] K[ ] GetHostedConfigurationVersion
- W[x] S[ ] M[ ] F[ ] K[ ] ListApplications
- W[x] S[ ] M[x] F[ ] K[ ] ListConfigurationProfiles
- W[x] S[ ] M[ ] F[ ] K[ ] ListDeploymentStrategies
- W[x] S[ ] M[ ] F[ ] K[ ] ListDeployments
- W[x] S[ ] M[ ] F[ ] K[ ] ListEnvironments
- W[x] S[ ] M[ ] F[ ] K[ ] ListExtensionAssociations
- W[x] S[ ] M[ ] F[ ] K[ ] ListExtensions
- W[x] S[ ] M[ ] F[ ] K[ ] ListHostedConfigurationVersions
- W[x] S[ ] M[x] F[ ] K[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] StartDeployment
- W[x] S[ ] M[ ] F[ ] K[ ] StopDeployment
- W[x] S[ ] M[x] F[ ] K[ ] TagResource
- W[x] S[ ] M[x] F[ ] K[ ] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateAccountSettings
- W[x] S[ ] M[x] F[ ] K[ ] UpdateApplication
- W[x] S[ ] M[x] F[ ] K[ ] UpdateConfigurationProfile
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateDeploymentStrategy
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateEnvironment
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateExtension
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateExtensionAssociation
- W[x] S[ ] M[ ] F[ ] K[ ] ValidateConfiguration

Integration tests: 37/45 implemented operations tested (82.2%)
Untested implemented operations: 8

### winterbaume-appconfigdata (appconfigdata) - W: 2/2, S: 0/2, M: 0/2, F: 0/2, K: 0/2

- W[x] S[ ] M[ ] F[ ] K[ ] GetLatestConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] StartConfigurationSession

Integration tests: 2/2 implemented operations tested (100.0%)

### winterbaume-appfabric (appfabric) - W: 6/26, S: 0/26, M: 0/26, F: 0/26, K: 0/26

- W[ ] S[ ] M[ ] F[ ] K[ ] BatchGetUserAccessTasks
- W[ ] S[ ] M[ ] F[ ] K[ ] ConnectAppAuthorization
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateAppAuthorization
- W[x] S[ ] M[ ] F[ ] K[ ] CreateAppBundle
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateIngestion
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateIngestionDestination
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteAppAuthorization
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteAppBundle
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteIngestion
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteIngestionDestination
- W[ ] S[ ] M[ ] F[ ] K[ ] GetAppAuthorization
- W[x] S[ ] M[ ] F[ ] K[ ] GetAppBundle
- W[ ] S[ ] M[ ] F[ ] K[ ] GetIngestion
- W[ ] S[ ] M[ ] F[ ] K[ ] GetIngestionDestination
- W[ ] S[ ] M[ ] F[ ] K[ ] ListAppAuthorizations
- W[x] S[ ] M[ ] F[ ] K[ ] ListAppBundles
- W[ ] S[ ] M[ ] F[ ] K[ ] ListIngestionDestinations
- W[ ] S[ ] M[ ] F[ ] K[ ] ListIngestions
- W[ ] S[ ] M[ ] F[ ] K[ ] ListTagsForResource
- W[ ] S[ ] M[ ] F[ ] K[ ] StartIngestion
- W[ ] S[ ] M[ ] F[ ] K[ ] StartUserAccessTasks
- W[ ] S[ ] M[ ] F[ ] K[ ] StopIngestion
- W[x] S[ ] M[ ] F[ ] K[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UntagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateAppAuthorization
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateIngestionDestination

Integration tests: 6/6 implemented operations tested (100.0%)

### winterbaume-appflow (appflow) - W: 9/25, S: 0/25, M: 0/25, F: 0/25, K: 0/25

- W[ ] S[ ] M[ ] F[ ] K[ ] CancelFlowExecutions
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateConnectorProfile
- W[x] S[ ] M[ ] F[ ] K[ ] CreateFlow
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteConnectorProfile
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteFlow
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeConnector
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeConnectorEntity
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeConnectorProfiles
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeConnectors
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeFlow
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeFlowExecutionRecords
- W[ ] S[ ] M[ ] F[ ] K[ ] ListConnectorEntities
- W[ ] S[ ] M[ ] F[ ] K[ ] ListConnectors
- W[x] S[ ] M[ ] F[ ] K[ ] ListFlows
- W[ ] S[ ] M[ ] F[ ] K[ ] ListTagsForResource
- W[ ] S[ ] M[ ] F[ ] K[ ] RegisterConnector
- W[ ] S[ ] M[ ] F[ ] K[ ] ResetConnectorMetadataCache
- W[x] S[ ] M[ ] F[ ] K[ ] StartFlow
- W[x] S[ ] M[ ] F[ ] K[ ] StopFlow
- W[x] S[ ] M[ ] F[ ] K[ ] TagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] UnregisterConnector
- W[x] S[ ] M[ ] F[ ] K[ ] UntagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateConnectorProfile
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateConnectorRegistration
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateFlow

Integration tests: 6/9 implemented operations tested (66.7%)
Untested implemented operations: 3

### winterbaume-appintegrations (appintegrations) - W: 23/23, S: 0/23, M: 0/23, F: 0/23, K: 0/23

- W[x] S[ ] M[ ] F[ ] K[ ] CreateApplication
- W[x] S[ ] M[ ] F[ ] K[ ] CreateDataIntegration
- W[x] S[ ] M[ ] F[ ] K[ ] CreateDataIntegrationAssociation
- W[x] S[ ] M[ ] F[ ] K[ ] CreateEventIntegration
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteApplication
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteDataIntegration
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteEventIntegration
- W[x] S[ ] M[ ] F[ ] K[ ] GetApplication
- W[x] S[ ] M[ ] F[ ] K[ ] GetDataIntegration
- W[x] S[ ] M[ ] F[ ] K[ ] GetEventIntegration
- W[x] S[ ] M[ ] F[ ] K[ ] ListApplicationAssociations
- W[x] S[ ] M[ ] F[ ] K[ ] ListApplications
- W[x] S[ ] M[ ] F[ ] K[ ] ListDataIntegrationAssociations
- W[x] S[ ] M[ ] F[ ] K[ ] ListDataIntegrations
- W[x] S[ ] M[ ] F[ ] K[ ] ListEventIntegrationAssociations
- W[x] S[ ] M[ ] F[ ] K[ ] ListEventIntegrations
- W[x] S[ ] M[ ] F[ ] K[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateApplication
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateDataIntegration
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateDataIntegrationAssociation
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateEventIntegration

Integration tests: 21/23 implemented operations tested (91.3%)
Untested implemented operations: 2

### winterbaume-applicationautoscaling (application-auto-scaling) - W: 13/14, S: 1/14, M: 9/14, F: 0/14, K: 0/14

- W[x] S[ ] M[x] F[ ] K[ ] DeleteScalingPolicy
- W[x] S[ ] M[x] F[ ] K[ ] DeleteScheduledAction
- W[x] S[ ] M[x] F[ ] K[ ] DeregisterScalableTarget
- W[x] S[ ] M[x] F[ ] K[ ] DescribeScalableTargets
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeScalingActivities
- W[x] S[ ] M[x] F[ ] K[ ] DescribeScalingPolicies
- W[x] S[ ] M[x] F[ ] K[ ] DescribeScheduledActions
- W[ ] S[x] M[ ] F[ ] K[ ] GetPredictiveScalingForecast
- W[x] S[ ] M[ ] F[ ] K[ ] ListTagsForResource
- W[x] S[ ] M[x] F[ ] K[ ] PutScalingPolicy
- W[x] S[ ] M[x] F[ ] K[ ] PutScheduledAction
- W[x] S[ ] M[x] F[ ] K[ ] RegisterScalableTarget
- W[x] S[ ] M[ ] F[ ] K[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UntagResource

Integration tests: 13/13 implemented operations tested (100.0%)

### winterbaume-applicationcostprofiler (applicationcostprofiler) - W: 6/6, S: 0/6, M: 0/6, F: 0/6, K: 0/6

- W[x] S[ ] M[ ] F[ ] K[ ] DeleteReportDefinition
- W[x] S[ ] M[ ] F[ ] K[ ] GetReportDefinition
- W[x] S[ ] M[ ] F[ ] K[ ] ImportApplicationUsage
- W[x] S[ ] M[ ] F[ ] K[ ] ListReportDefinitions
- W[x] S[ ] M[ ] F[ ] K[ ] PutReportDefinition
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateReportDefinition

Integration tests: 6/6 implemented operations tested (100.0%)

### winterbaume-applicationdiscoveryservice (application-discovery-service) - W: 28/28, S: 0/28, M: 0/28, F: 0/28, K: 0/28

- W[x] S[ ] M[ ] F[ ] K[ ] AssociateConfigurationItemsToApplication
- W[x] S[ ] M[ ] F[ ] K[ ] BatchDeleteAgents
- W[x] S[ ] M[ ] F[ ] K[ ] BatchDeleteImportData
- W[x] S[ ] M[ ] F[ ] K[ ] CreateApplication
- W[x] S[ ] M[ ] F[ ] K[ ] CreateTags
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteApplications
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteTags
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeAgents
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeBatchDeleteConfigurationTask
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeConfigurations
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeContinuousExports
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeExportConfigurations
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeExportTasks
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeImportTasks
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeTags
- W[x] S[ ] M[ ] F[ ] K[ ] DisassociateConfigurationItemsFromApplication
- W[x] S[ ] M[ ] F[ ] K[ ] ExportConfigurations
- W[x] S[ ] M[ ] F[ ] K[ ] GetDiscoverySummary
- W[x] S[ ] M[ ] F[ ] K[ ] ListConfigurations
- W[x] S[ ] M[ ] F[ ] K[ ] ListServerNeighbors
- W[x] S[ ] M[ ] F[ ] K[ ] StartBatchDeleteConfigurationTask
- W[x] S[ ] M[ ] F[ ] K[ ] StartContinuousExport
- W[x] S[ ] M[ ] F[ ] K[ ] StartDataCollectionByAgentIds
- W[x] S[ ] M[ ] F[ ] K[ ] StartExportTask
- W[x] S[ ] M[ ] F[ ] K[ ] StartImportTask
- W[x] S[ ] M[ ] F[ ] K[ ] StopContinuousExport
- W[x] S[ ] M[ ] F[ ] K[ ] StopDataCollectionByAgentIds
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateApplication

Integration tests: 20/28 implemented operations tested (71.4%)
Untested implemented operations: 8

### winterbaume-applicationinsights (application-insights) - W: 33/33, S: 0/33, M: 0/33, F: 0/33, K: 0/33

- W[x] S[ ] M[ ] F[ ] K[ ] AddWorkload
- W[x] S[ ] M[ ] F[ ] K[ ] CreateApplication
- W[x] S[ ] M[ ] F[ ] K[ ] CreateComponent
- W[x] S[ ] M[ ] F[ ] K[ ] CreateLogPattern
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteApplication
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteComponent
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteLogPattern
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeApplication
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeComponent
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeComponentConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeComponentConfigurationRecommendation
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeLogPattern
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeObservation
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeProblem
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeProblemObservations
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeWorkload
- W[x] S[ ] M[ ] F[ ] K[ ] ListApplications
- W[x] S[ ] M[ ] F[ ] K[ ] ListComponents
- W[x] S[ ] M[ ] F[ ] K[ ] ListConfigurationHistory
- W[x] S[ ] M[ ] F[ ] K[ ] ListLogPatternSets
- W[x] S[ ] M[ ] F[ ] K[ ] ListLogPatterns
- W[x] S[ ] M[ ] F[ ] K[ ] ListProblems
- W[x] S[ ] M[ ] F[ ] K[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] ListWorkloads
- W[x] S[ ] M[ ] F[ ] K[ ] RemoveWorkload
- W[x] S[ ] M[ ] F[ ] K[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateApplication
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateComponent
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateComponentConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateLogPattern
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateProblem
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateWorkload

Integration tests: 26/33 implemented operations tested (78.8%)
Untested implemented operations: 7

### winterbaume-applicationsignals (application-signals) - W: 10/23, S: 3/23, M: 0/23, F: 0/23, K: 0/23

- W[ ] S[ ] M[ ] F[ ] K[ ] BatchGetServiceLevelObjectiveBudgetReport
- W[x] S[ ] M[ ] F[ ] K[ ] BatchUpdateExclusionWindows
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateServiceLevelObjective
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteGroupingConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteServiceLevelObjective
- W[x] S[ ] M[ ] F[ ] K[ ] GetService
- W[ ] S[ ] M[ ] F[ ] K[ ] GetServiceLevelObjective
- W[ ] S[x] M[ ] F[ ] K[ ] ListAuditFindings
- W[ ] S[x] M[ ] F[ ] K[ ] ListEntityEvents
- W[x] S[ ] M[ ] F[ ] K[ ] ListGroupingAttributeDefinitions
- W[ ] S[ ] M[ ] F[ ] K[ ] ListServiceDependencies
- W[ ] S[ ] M[ ] F[ ] K[ ] ListServiceDependents
- W[ ] S[ ] M[ ] F[ ] K[ ] ListServiceLevelObjectiveExclusionWindows
- W[ ] S[ ] M[ ] F[ ] K[ ] ListServiceLevelObjectives
- W[ ] S[ ] M[ ] F[ ] K[ ] ListServiceOperations
- W[ ] S[x] M[ ] F[ ] K[ ] ListServiceStates
- W[x] S[ ] M[ ] F[ ] K[ ] ListServices
- W[x] S[ ] M[ ] F[ ] K[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] PutGroupingConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] StartDiscovery
- W[x] S[ ] M[ ] F[ ] K[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UntagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateServiceLevelObjective

Integration tests: 8/10 implemented operations tested (80.0%)
Untested implemented operations: 2

### winterbaume-appmesh (app-mesh) - W: 38/38, S: 0/38, M: 0/38, F: 0/38, K: 25/38

- W[x] S[ ] M[ ] F[ ] K[ ] CreateGatewayRoute
- W[x] S[ ] M[ ] F[ ] K[x] CreateMesh
- W[x] S[ ] M[ ] F[ ] K[x] CreateRoute
- W[x] S[ ] M[ ] F[ ] K[ ] CreateVirtualGateway
- W[x] S[ ] M[ ] F[ ] K[x] CreateVirtualNode
- W[x] S[ ] M[ ] F[ ] K[x] CreateVirtualRouter
- W[x] S[ ] M[ ] F[ ] K[x] CreateVirtualService
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteGatewayRoute
- W[x] S[ ] M[ ] F[ ] K[x] DeleteMesh
- W[x] S[ ] M[ ] F[ ] K[x] DeleteRoute
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteVirtualGateway
- W[x] S[ ] M[ ] F[ ] K[x] DeleteVirtualNode
- W[x] S[ ] M[ ] F[ ] K[x] DeleteVirtualRouter
- W[x] S[ ] M[ ] F[ ] K[x] DeleteVirtualService
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeGatewayRoute
- W[x] S[ ] M[ ] F[ ] K[x] DescribeMesh
- W[x] S[ ] M[ ] F[ ] K[x] DescribeRoute
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeVirtualGateway
- W[x] S[ ] M[ ] F[ ] K[x] DescribeVirtualNode
- W[x] S[ ] M[ ] F[ ] K[x] DescribeVirtualRouter
- W[x] S[ ] M[ ] F[ ] K[x] DescribeVirtualService
- W[x] S[ ] M[ ] F[ ] K[ ] ListGatewayRoutes
- W[x] S[ ] M[ ] F[ ] K[x] ListMeshes
- W[x] S[ ] M[ ] F[ ] K[x] ListRoutes
- W[x] S[ ] M[ ] F[ ] K[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] ListVirtualGateways
- W[x] S[ ] M[ ] F[ ] K[x] ListVirtualNodes
- W[x] S[ ] M[ ] F[ ] K[x] ListVirtualRouters
- W[x] S[ ] M[ ] F[ ] K[x] ListVirtualServices
- W[x] S[ ] M[ ] F[ ] K[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateGatewayRoute
- W[x] S[ ] M[ ] F[ ] K[x] UpdateMesh
- W[x] S[ ] M[ ] F[ ] K[x] UpdateRoute
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateVirtualGateway
- W[x] S[ ] M[ ] F[ ] K[x] UpdateVirtualNode
- W[x] S[ ] M[ ] F[ ] K[x] UpdateVirtualRouter
- W[x] S[ ] M[ ] F[ ] K[x] UpdateVirtualService

Integration tests: 36/38 implemented operations tested (94.7%)
Untested implemented operations: 2

### winterbaume-apprunner (apprunner) - W: 23/37, S: 0/37, M: 0/37, F: 0/37, K: 0/37

- W[ ] S[ ] M[ ] F[ ] K[ ] AssociateCustomDomain
- W[x] S[ ] M[ ] F[ ] K[ ] CreateAutoScalingConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] CreateConnection
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateObservabilityConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] CreateService
- W[x] S[ ] M[ ] F[ ] K[ ] CreateVpcConnector
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateVpcIngressConnection
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteAutoScalingConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteConnection
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteObservabilityConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteService
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteVpcConnector
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteVpcIngressConnection
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeAutoScalingConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeCustomDomains
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeObservabilityConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeService
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeVpcConnector
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeVpcIngressConnection
- W[ ] S[ ] M[ ] F[ ] K[ ] DisassociateCustomDomain
- W[x] S[ ] M[ ] F[ ] K[ ] ListAutoScalingConfigurations
- W[x] S[ ] M[ ] F[ ] K[ ] ListConnections
- W[ ] S[ ] M[ ] F[ ] K[ ] ListObservabilityConfigurations
- W[x] S[ ] M[ ] F[ ] K[ ] ListOperations
- W[x] S[ ] M[ ] F[ ] K[ ] ListServices
- W[ ] S[ ] M[ ] F[ ] K[ ] ListServicesForAutoScalingConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] ListVpcConnectors
- W[ ] S[ ] M[ ] F[ ] K[ ] ListVpcIngressConnections
- W[x] S[ ] M[ ] F[ ] K[ ] PauseService
- W[x] S[ ] M[ ] F[ ] K[ ] ResumeService
- W[x] S[ ] M[ ] F[ ] K[ ] StartDeployment
- W[x] S[ ] M[ ] F[ ] K[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateDefaultAutoScalingConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateService
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateVpcIngressConnection

Integration tests: 19/23 implemented operations tested (82.6%)
Untested implemented operations: 4

### winterbaume-appsync (appsync) - W: 27/74, S: 0/74, M: 27/74, F: 0/74, K: 3/74

- W[ ] S[ ] M[ ] F[ ] K[ ] AssociateApi
- W[ ] S[ ] M[ ] F[ ] K[ ] AssociateMergedGraphqlApi
- W[ ] S[ ] M[ ] F[ ] K[ ] AssociateSourceGraphqlApi
- W[x] S[ ] M[x] F[ ] K[ ] CreateApi
- W[x] S[ ] M[x] F[ ] K[ ] CreateApiCache
- W[x] S[ ] M[x] F[ ] K[ ] CreateApiKey
- W[x] S[ ] M[x] F[ ] K[ ] CreateChannelNamespace
- W[ ] S[ ] M[ ] F[ ] K[x] CreateDataSource
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateDomainName
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateFunction
- W[x] S[ ] M[x] F[ ] K[ ] CreateGraphqlApi
- W[ ] S[ ] M[ ] F[ ] K[x] CreateResolver
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateType
- W[x] S[ ] M[x] F[ ] K[ ] DeleteApi
- W[x] S[ ] M[x] F[ ] K[ ] DeleteApiCache
- W[x] S[ ] M[x] F[ ] K[ ] DeleteApiKey
- W[x] S[ ] M[x] F[ ] K[ ] DeleteChannelNamespace
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteDataSource
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteDomainName
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteFunction
- W[x] S[ ] M[x] F[ ] K[ ] DeleteGraphqlApi
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteResolver
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteType
- W[ ] S[ ] M[ ] F[ ] K[ ] DisassociateApi
- W[ ] S[ ] M[ ] F[ ] K[ ] DisassociateMergedGraphqlApi
- W[ ] S[ ] M[ ] F[ ] K[ ] DisassociateSourceGraphqlApi
- W[ ] S[ ] M[ ] F[ ] K[ ] EvaluateCode
- W[ ] S[ ] M[ ] F[ ] K[ ] EvaluateMappingTemplate
- W[x] S[ ] M[x] F[ ] K[ ] FlushApiCache
- W[x] S[ ] M[x] F[ ] K[ ] GetApi
- W[ ] S[ ] M[ ] F[ ] K[ ] GetApiAssociation
- W[x] S[ ] M[x] F[ ] K[ ] GetApiCache
- W[ ] S[ ] M[ ] F[ ] K[ ] GetChannelNamespace
- W[ ] S[ ] M[ ] F[ ] K[ ] GetDataSource
- W[ ] S[ ] M[ ] F[ ] K[ ] GetDataSourceIntrospection
- W[ ] S[ ] M[ ] F[ ] K[ ] GetDomainName
- W[ ] S[ ] M[ ] F[ ] K[ ] GetFunction
- W[x] S[ ] M[x] F[ ] K[ ] GetGraphqlApi
- W[ ] S[ ] M[ ] F[ ] K[ ] GetGraphqlApiEnvironmentVariables
- W[ ] S[ ] M[ ] F[ ] K[ ] GetIntrospectionSchema
- W[ ] S[ ] M[ ] F[ ] K[ ] GetResolver
- W[x] S[ ] M[x] F[ ] K[ ] GetSchemaCreationStatus
- W[ ] S[ ] M[ ] F[ ] K[ ] GetSourceApiAssociation
- W[x] S[ ] M[x] F[ ] K[ ] GetType
- W[x] S[ ] M[x] F[ ] K[ ] ListApiKeys
- W[x] S[ ] M[x] F[ ] K[ ] ListApis
- W[x] S[ ] M[x] F[ ] K[ ] ListChannelNamespaces
- W[ ] S[ ] M[ ] F[ ] K[ ] ListDataSources
- W[ ] S[ ] M[ ] F[ ] K[ ] ListDomainNames
- W[ ] S[ ] M[ ] F[ ] K[ ] ListFunctions
- W[x] S[ ] M[x] F[ ] K[ ] ListGraphqlApis
- W[ ] S[ ] M[ ] F[ ] K[ ] ListResolvers
- W[ ] S[ ] M[ ] F[ ] K[ ] ListResolversByFunction
- W[ ] S[ ] M[ ] F[ ] K[ ] ListSourceApiAssociations
- W[x] S[ ] M[x] F[ ] K[ ] ListTagsForResource
- W[ ] S[ ] M[ ] F[ ] K[ ] ListTypes
- W[ ] S[ ] M[ ] F[ ] K[ ] ListTypesByAssociation
- W[ ] S[ ] M[ ] F[ ] K[ ] PutGraphqlApiEnvironmentVariables
- W[ ] S[ ] M[ ] F[ ] K[ ] StartDataSourceIntrospection
- W[x] S[ ] M[x] F[ ] K[x] StartSchemaCreation
- W[ ] S[ ] M[ ] F[ ] K[ ] StartSchemaMerge
- W[x] S[ ] M[x] F[ ] K[ ] TagResource
- W[x] S[ ] M[x] F[ ] K[ ] UntagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateApi
- W[x] S[ ] M[x] F[ ] K[ ] UpdateApiCache
- W[x] S[ ] M[x] F[ ] K[ ] UpdateApiKey
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateChannelNamespace
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateDataSource
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateDomainName
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateFunction
- W[x] S[ ] M[x] F[ ] K[ ] UpdateGraphqlApi
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateResolver
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateSourceApiAssociation
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateType

Integration tests: 27/27 implemented operations tested (100.0%)

### winterbaume-arczonalshift (arc-zonal-shift) - W: 14/15, S: 1/15, M: 0/15, F: 0/15, K: 0/15

- W[x] S[ ] M[ ] F[ ] K[ ] CancelPracticeRun
- W[x] S[ ] M[ ] F[ ] K[ ] CancelZonalShift
- W[x] S[ ] M[ ] F[ ] K[ ] CreatePracticeRunConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] DeletePracticeRunConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] GetAutoshiftObserverNotificationStatus
- W[x] S[ ] M[ ] F[ ] K[ ] GetManagedResource
- W[ ] S[x] M[ ] F[ ] K[ ] ListAutoshifts
- W[x] S[ ] M[ ] F[ ] K[ ] ListManagedResources
- W[x] S[ ] M[ ] F[ ] K[ ] ListZonalShifts
- W[x] S[ ] M[ ] F[ ] K[ ] StartPracticeRun
- W[x] S[ ] M[ ] F[ ] K[ ] StartZonalShift
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateAutoshiftObserverNotificationStatus
- W[x] S[ ] M[ ] F[ ] K[ ] UpdatePracticeRunConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateZonalAutoshiftConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateZonalShift

Integration tests: 14/14 implemented operations tested (100.0%)

### winterbaume-artifact (artifact) - W: 8/8, S: 0/8, M: 0/8, F: 0/8, K: 0/8

- W[x] S[ ] M[ ] F[ ] K[ ] GetAccountSettings
- W[x] S[ ] M[ ] F[ ] K[ ] GetReport
- W[x] S[ ] M[ ] F[ ] K[ ] GetReportMetadata
- W[x] S[ ] M[ ] F[ ] K[ ] GetTermForReport
- W[x] S[ ] M[ ] F[ ] K[ ] ListCustomerAgreements
- W[x] S[ ] M[ ] F[ ] K[ ] ListReportVersions
- W[x] S[ ] M[ ] F[ ] K[ ] ListReports
- W[x] S[ ] M[ ] F[ ] K[ ] PutAccountSettings

Integration tests: 5/8 implemented operations tested (62.5%)
Untested implemented operations: 3

### winterbaume-athena (athena) - W: 25/70, S: 0/70, M: 27/70, F: 0/70, K: 7/70

- W[ ] S[ ] M[ ] F[ ] K[ ] BatchGetNamedQuery
- W[ ] S[ ] M[ ] F[ ] K[ ] BatchGetPreparedStatement
- W[ ] S[ ] M[ ] F[ ] K[ ] BatchGetQueryExecution
- W[ ] S[ ] M[ ] F[ ] K[ ] CancelCapacityReservation
- W[x] S[ ] M[x] F[ ] K[ ] CreateCapacityReservation
- W[x] S[ ] M[x] F[ ] K[ ] CreateDataCatalog
- W[x] S[ ] M[x] F[ ] K[ ] CreateNamedQuery
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateNotebook
- W[x] S[ ] M[x] F[ ] K[ ] CreatePreparedStatement
- W[ ] S[ ] M[ ] F[ ] K[ ] CreatePresignedNotebookUrl
- W[x] S[ ] M[x] F[ ] K[x] CreateWorkGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteCapacityReservation
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteDataCatalog
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteNamedQuery
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteNotebook
- W[ ] S[ ] M[ ] F[ ] K[ ] DeletePreparedStatement
- W[x] S[ ] M[x] F[ ] K[x] DeleteWorkGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] ExportNotebook
- W[ ] S[ ] M[ ] F[ ] K[ ] GetCalculationExecution
- W[ ] S[ ] M[ ] F[ ] K[ ] GetCalculationExecutionCode
- W[ ] S[ ] M[ ] F[ ] K[ ] GetCalculationExecutionStatus
- W[ ] S[ ] M[ ] F[ ] K[ ] GetCapacityAssignmentConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] GetCapacityReservation
- W[x] S[ ] M[x] F[ ] K[ ] GetDataCatalog
- W[ ] S[ ] M[x] F[ ] K[ ] GetDatabase
- W[x] S[ ] M[x] F[ ] K[ ] GetNamedQuery
- W[ ] S[ ] M[ ] F[ ] K[ ] GetNotebookMetadata
- W[x] S[ ] M[x] F[ ] K[ ] GetPreparedStatement
- W[x] S[ ] M[x] F[ ] K[x] GetQueryExecution
- W[x] S[ ] M[x] F[ ] K[x] GetQueryResults
- W[x] S[ ] M[x] F[ ] K[ ] GetQueryRuntimeStatistics
- W[ ] S[ ] M[ ] F[ ] K[ ] GetResourceDashboard
- W[ ] S[ ] M[ ] F[ ] K[ ] GetSession
- W[ ] S[ ] M[ ] F[ ] K[ ] GetSessionEndpoint
- W[ ] S[ ] M[ ] F[ ] K[ ] GetSessionStatus
- W[ ] S[ ] M[ ] F[ ] K[ ] GetTableMetadata
- W[x] S[ ] M[x] F[ ] K[ ] GetWorkGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] ImportNotebook
- W[ ] S[ ] M[ ] F[ ] K[ ] ListApplicationDPUSizes
- W[ ] S[ ] M[ ] F[ ] K[ ] ListCalculationExecutions
- W[x] S[ ] M[x] F[ ] K[ ] ListCapacityReservations
- W[x] S[ ] M[x] F[ ] K[ ] ListDataCatalogs
- W[ ] S[ ] M[x] F[ ] K[ ] ListDatabases
- W[ ] S[ ] M[ ] F[ ] K[ ] ListEngineVersions
- W[ ] S[ ] M[ ] F[ ] K[ ] ListExecutors
- W[x] S[ ] M[x] F[ ] K[ ] ListNamedQueries
- W[ ] S[ ] M[ ] F[ ] K[ ] ListNotebookMetadata
- W[ ] S[ ] M[ ] F[ ] K[ ] ListNotebookSessions
- W[ ] S[ ] M[ ] F[ ] K[ ] ListPreparedStatements
- W[x] S[ ] M[x] F[ ] K[x] ListQueryExecutions
- W[ ] S[ ] M[ ] F[ ] K[ ] ListSessions
- W[ ] S[ ] M[ ] F[ ] K[ ] ListTableMetadata
- W[x] S[ ] M[x] F[ ] K[ ] ListTagsForResource
- W[x] S[ ] M[x] F[ ] K[ ] ListWorkGroups
- W[ ] S[ ] M[ ] F[ ] K[ ] PutCapacityAssignmentConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] StartCalculationExecution
- W[x] S[ ] M[x] F[ ] K[x] StartQueryExecution
- W[ ] S[ ] M[ ] F[ ] K[ ] StartSession
- W[ ] S[ ] M[ ] F[ ] K[ ] StopCalculationExecution
- W[x] S[ ] M[x] F[ ] K[x] StopQueryExecution
- W[x] S[ ] M[x] F[ ] K[ ] TagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] TerminateSession
- W[x] S[ ] M[x] F[ ] K[ ] UntagResource
- W[x] S[ ] M[x] F[ ] K[ ] UpdateCapacityReservation
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateDataCatalog
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateNamedQuery
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateNotebook
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateNotebookMetadata
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdatePreparedStatement
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateWorkGroup

Integration tests: 25/25 implemented operations tested (100.0%)

### winterbaume-auditmanager (auditmanager) - W: 15/62, S: 0/62, M: 0/62, F: 0/62, K: 0/62

- W[ ] S[ ] M[ ] F[ ] K[ ] AssociateAssessmentReportEvidenceFolder
- W[ ] S[ ] M[ ] F[ ] K[ ] BatchAssociateAssessmentReportEvidence
- W[ ] S[ ] M[ ] F[ ] K[ ] BatchCreateDelegationByAssessment
- W[ ] S[ ] M[ ] F[ ] K[ ] BatchDeleteDelegationByAssessment
- W[ ] S[ ] M[ ] F[ ] K[ ] BatchDisassociateAssessmentReportEvidence
- W[ ] S[ ] M[ ] F[ ] K[ ] BatchImportEvidenceToAssessmentControl
- W[x] S[ ] M[ ] F[ ] K[ ] CreateAssessment
- W[x] S[ ] M[ ] F[ ] K[ ] CreateAssessmentFramework
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateAssessmentReport
- W[x] S[ ] M[ ] F[ ] K[ ] CreateControl
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteAssessment
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteAssessmentFramework
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteAssessmentFrameworkShare
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteAssessmentReport
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteControl
- W[x] S[ ] M[ ] F[ ] K[ ] DeregisterAccount
- W[ ] S[ ] M[ ] F[ ] K[ ] DeregisterOrganizationAdminAccount
- W[ ] S[ ] M[ ] F[ ] K[ ] DisassociateAssessmentReportEvidenceFolder
- W[x] S[ ] M[ ] F[ ] K[ ] GetAccountStatus
- W[x] S[ ] M[ ] F[ ] K[ ] GetAssessment
- W[x] S[ ] M[ ] F[ ] K[ ] GetAssessmentFramework
- W[ ] S[ ] M[ ] F[ ] K[ ] GetAssessmentReportUrl
- W[ ] S[ ] M[ ] F[ ] K[ ] GetChangeLogs
- W[x] S[ ] M[ ] F[ ] K[ ] GetControl
- W[ ] S[ ] M[ ] F[ ] K[ ] GetDelegations
- W[ ] S[ ] M[ ] F[ ] K[ ] GetEvidence
- W[ ] S[ ] M[ ] F[ ] K[ ] GetEvidenceByEvidenceFolder
- W[ ] S[ ] M[ ] F[ ] K[ ] GetEvidenceFileUploadUrl
- W[ ] S[ ] M[ ] F[ ] K[ ] GetEvidenceFolder
- W[ ] S[ ] M[ ] F[ ] K[ ] GetEvidenceFoldersByAssessment
- W[ ] S[ ] M[ ] F[ ] K[ ] GetEvidenceFoldersByAssessmentControl
- W[ ] S[ ] M[ ] F[ ] K[ ] GetInsights
- W[ ] S[ ] M[ ] F[ ] K[ ] GetInsightsByAssessment
- W[ ] S[ ] M[ ] F[ ] K[ ] GetOrganizationAdminAccount
- W[ ] S[ ] M[ ] F[ ] K[ ] GetServicesInScope
- W[ ] S[ ] M[ ] F[ ] K[ ] GetSettings
- W[ ] S[ ] M[ ] F[ ] K[ ] ListAssessmentControlInsightsByControlDomain
- W[ ] S[ ] M[ ] F[ ] K[ ] ListAssessmentFrameworkShareRequests
- W[x] S[ ] M[ ] F[ ] K[ ] ListAssessmentFrameworks
- W[ ] S[ ] M[ ] F[ ] K[ ] ListAssessmentReports
- W[x] S[ ] M[ ] F[ ] K[ ] ListAssessments
- W[ ] S[ ] M[ ] F[ ] K[ ] ListControlDomainInsights
- W[ ] S[ ] M[ ] F[ ] K[ ] ListControlDomainInsightsByAssessment
- W[ ] S[ ] M[ ] F[ ] K[ ] ListControlInsightsByControlDomain
- W[x] S[ ] M[ ] F[ ] K[ ] ListControls
- W[ ] S[ ] M[ ] F[ ] K[ ] ListKeywordsForDataSource
- W[ ] S[ ] M[ ] F[ ] K[ ] ListNotifications
- W[ ] S[ ] M[ ] F[ ] K[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] RegisterAccount
- W[ ] S[ ] M[ ] F[ ] K[ ] RegisterOrganizationAdminAccount
- W[ ] S[ ] M[ ] F[ ] K[ ] StartAssessmentFrameworkShare
- W[ ] S[ ] M[ ] F[ ] K[ ] TagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] UntagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateAssessment
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateAssessmentControl
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateAssessmentControlSetStatus
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateAssessmentFramework
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateAssessmentFrameworkShare
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateAssessmentStatus
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateControl
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateSettings
- W[ ] S[ ] M[ ] F[ ] K[ ] ValidateAssessmentReportIntegrity

Integration tests: 15/15 implemented operations tested (100.0%)

### winterbaume-autoscaling (auto-scaling) - W: 52/66, S: 0/66, M: 39/66, F: 0/66, K: 0/66

Terraform E2E: 10 tests across 5 terraform resource types

Resource types: aws_autoscaling_group, aws_autoscaling_lifecycle_hook, aws_autoscaling_policy, aws_autoscaling_schedule, aws_launch_configuration

- W[x] S[ ] M[x] F[ ] K[ ] AttachInstances
- W[x] S[ ] M[x] F[ ] K[ ] AttachLoadBalancerTargetGroups
- W[x] S[ ] M[x] F[ ] K[ ] AttachLoadBalancers
- W[ ] S[ ] M[ ] F[ ] K[ ] AttachTrafficSources
- W[x] S[ ] M[x] F[ ] K[ ] BatchDeleteScheduledAction
- W[x] S[ ] M[x] F[ ] K[ ] BatchPutScheduledUpdateGroupAction
- W[ ] S[ ] M[ ] F[ ] K[ ] CancelInstanceRefresh
- W[ ] S[ ] M[ ] F[ ] K[ ] CompleteLifecycleAction
- W[x] S[ ] M[x] F[ ] K[ ] CreateAutoScalingGroup
- W[x] S[ ] M[x] F[ ] K[ ] CreateLaunchConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] CreateOrUpdateTags
- W[x] S[ ] M[x] F[ ] K[ ] DeleteAutoScalingGroup
- W[x] S[ ] M[x] F[ ] K[ ] DeleteLaunchConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] DeleteLifecycleHook
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteNotificationConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] DeletePolicy
- W[x] S[ ] M[x] F[ ] K[ ] DeleteScheduledAction
- W[x] S[ ] M[x] F[ ] K[ ] DeleteTags
- W[x] S[ ] M[x] F[ ] K[ ] DeleteWarmPool
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeAccountLimits
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeAdjustmentTypes
- W[x] S[ ] M[x] F[ ] K[ ] DescribeAutoScalingGroups
- W[x] S[ ] M[x] F[ ] K[ ] DescribeAutoScalingInstances
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeAutoScalingNotificationTypes
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeInstanceRefreshes
- W[x] S[ ] M[x] F[ ] K[ ] DescribeLaunchConfigurations
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeLifecycleHookTypes
- W[x] S[ ] M[x] F[ ] K[ ] DescribeLifecycleHooks
- W[x] S[ ] M[x] F[ ] K[ ] DescribeLoadBalancerTargetGroups
- W[x] S[ ] M[x] F[ ] K[ ] DescribeLoadBalancers
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeMetricCollectionTypes
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeNotificationConfigurations
- W[x] S[ ] M[x] F[ ] K[ ] DescribePolicies
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeScalingActivities
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeScalingProcessTypes
- W[x] S[ ] M[x] F[ ] K[ ] DescribeScheduledActions
- W[x] S[ ] M[x] F[ ] K[ ] DescribeTags
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeTerminationPolicyTypes
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeTrafficSources
- W[x] S[ ] M[x] F[ ] K[ ] DescribeWarmPool
- W[x] S[ ] M[x] F[ ] K[ ] DetachInstances
- W[x] S[ ] M[x] F[ ] K[ ] DetachLoadBalancerTargetGroups
- W[x] S[ ] M[x] F[ ] K[ ] DetachLoadBalancers
- W[ ] S[ ] M[ ] F[ ] K[ ] DetachTrafficSources
- W[x] S[ ] M[ ] F[ ] K[ ] DisableMetricsCollection
- W[x] S[ ] M[x] F[ ] K[ ] EnableMetricsCollection
- W[ ] S[ ] M[ ] F[ ] K[ ] EnterStandby
- W[x] S[ ] M[x] F[ ] K[ ] ExecutePolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] ExitStandby
- W[ ] S[ ] M[ ] F[ ] K[ ] GetPredictiveScalingForecast
- W[ ] S[ ] M[ ] F[ ] K[ ] LaunchInstances
- W[x] S[ ] M[ ] F[ ] K[ ] PutLifecycleHook
- W[x] S[ ] M[ ] F[ ] K[ ] PutNotificationConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] PutScalingPolicy
- W[x] S[ ] M[x] F[ ] K[ ] PutScheduledUpdateGroupAction
- W[x] S[ ] M[x] F[ ] K[ ] PutWarmPool
- W[ ] S[ ] M[ ] F[ ] K[ ] RecordLifecycleActionHeartbeat
- W[x] S[ ] M[x] F[ ] K[ ] ResumeProcesses
- W[ ] S[ ] M[ ] F[ ] K[ ] RollbackInstanceRefresh
- W[x] S[ ] M[x] F[ ] K[ ] SetDesiredCapacity
- W[x] S[ ] M[x] F[ ] K[ ] SetInstanceHealth
- W[x] S[ ] M[x] F[ ] K[ ] SetInstanceProtection
- W[ ] S[ ] M[ ] F[ ] K[ ] StartInstanceRefresh
- W[x] S[ ] M[x] F[ ] K[ ] SuspendProcesses
- W[x] S[ ] M[ ] F[ ] K[ ] TerminateInstanceInAutoScalingGroup
- W[x] S[ ] M[x] F[ ] K[ ] UpdateAutoScalingGroup

Integration tests: 49/52 implemented operations tested (94.2%)
Untested implemented operations: 3

### winterbaume-autoscalingplans (auto-scaling-plans) - W: 6/6, S: 0/6, M: 0/6, F: 0/6, K: 0/6

- W[x] S[ ] M[ ] F[ ] K[ ] CreateScalingPlan
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteScalingPlan
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeScalingPlanResources
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeScalingPlans
- W[x] S[ ] M[ ] F[ ] K[ ] GetScalingPlanResourceForecastData
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateScalingPlan

Integration tests: 5/6 implemented operations tested (83.3%)
Untested implemented operations: 1

### winterbaume-backup (backup) - W: 105/108, S: 3/108, M: 17/108, F: 0/108, K: 12/108

- W[x] S[ ] M[ ] F[ ] K[ ] AssociateBackupVaultMpaApprovalTeam
- W[x] S[ ] M[ ] F[ ] K[ ] CancelLegalHold
- W[x] S[ ] M[x] F[ ] K[x] CreateBackupPlan
- W[x] S[ ] M[ ] F[ ] K[x] CreateBackupSelection
- W[x] S[ ] M[x] F[ ] K[x] CreateBackupVault
- W[x] S[ ] M[ ] F[ ] K[ ] CreateFramework
- W[x] S[ ] M[ ] F[ ] K[ ] CreateLegalHold
- W[x] S[ ] M[ ] F[ ] K[ ] CreateLogicallyAirGappedBackupVault
- W[x] S[ ] M[x] F[ ] K[ ] CreateReportPlan
- W[x] S[ ] M[ ] F[ ] K[ ] CreateRestoreAccessBackupVault
- W[x] S[ ] M[ ] F[ ] K[ ] CreateRestoreTestingPlan
- W[x] S[ ] M[ ] F[ ] K[ ] CreateRestoreTestingSelection
- W[x] S[ ] M[ ] F[ ] K[ ] CreateTieringConfiguration
- W[x] S[ ] M[x] F[ ] K[x] DeleteBackupPlan
- W[x] S[ ] M[ ] F[ ] K[x] DeleteBackupSelection
- W[x] S[ ] M[x] F[ ] K[x] DeleteBackupVault
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteBackupVaultAccessPolicy
- W[x] S[ ] M[x] F[ ] K[ ] DeleteBackupVaultLockConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteBackupVaultNotifications
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteFramework
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteRecoveryPoint
- W[x] S[ ] M[x] F[ ] K[ ] DeleteReportPlan
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteRestoreTestingPlan
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteRestoreTestingSelection
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteTieringConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeBackupJob
- W[x] S[ ] M[x] F[ ] K[x] DescribeBackupVault
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeCopyJob
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeFramework
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeGlobalSettings
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeProtectedResource
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeRecoveryPoint
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeRegionSettings
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeReportJob
- W[x] S[ ] M[x] F[ ] K[ ] DescribeReportPlan
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeRestoreJob
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeScanJob
- W[x] S[ ] M[ ] F[ ] K[ ] DisassociateBackupVaultMpaApprovalTeam
- W[x] S[ ] M[ ] F[ ] K[ ] DisassociateRecoveryPoint
- W[x] S[ ] M[ ] F[ ] K[ ] DisassociateRecoveryPointFromParent
- W[x] S[ ] M[ ] F[ ] K[ ] ExportBackupPlanTemplate
- W[x] S[ ] M[x] F[ ] K[x] GetBackupPlan
- W[x] S[ ] M[ ] F[ ] K[ ] GetBackupPlanFromJSON
- W[ ] S[x] M[ ] F[ ] K[ ] GetBackupPlanFromTemplate
- W[x] S[ ] M[ ] F[ ] K[x] GetBackupSelection
- W[x] S[ ] M[ ] F[ ] K[ ] GetBackupVaultAccessPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] GetBackupVaultNotifications
- W[x] S[ ] M[ ] F[ ] K[ ] GetLegalHold
- W[x] S[ ] M[ ] F[ ] K[ ] GetRecoveryPointIndexDetails
- W[x] S[ ] M[ ] F[ ] K[ ] GetRecoveryPointRestoreMetadata
- W[x] S[ ] M[ ] F[ ] K[ ] GetRestoreJobMetadata
- W[x] S[ ] M[ ] F[ ] K[ ] GetRestoreTestingInferredMetadata
- W[x] S[ ] M[ ] F[ ] K[ ] GetRestoreTestingPlan
- W[x] S[ ] M[ ] F[ ] K[ ] GetRestoreTestingSelection
- W[x] S[ ] M[ ] F[ ] K[ ] GetSupportedResourceTypes
- W[x] S[ ] M[ ] F[ ] K[ ] GetTieringConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] ListBackupJobSummaries
- W[x] S[ ] M[ ] F[ ] K[ ] ListBackupJobs
- W[ ] S[x] M[ ] F[ ] K[ ] ListBackupPlanTemplates
- W[x] S[ ] M[ ] F[ ] K[ ] ListBackupPlanVersions
- W[x] S[ ] M[x] F[ ] K[x] ListBackupPlans
- W[x] S[ ] M[ ] F[ ] K[x] ListBackupSelections
- W[x] S[ ] M[x] F[ ] K[x] ListBackupVaults
- W[x] S[ ] M[ ] F[ ] K[ ] ListCopyJobSummaries
- W[x] S[ ] M[ ] F[ ] K[ ] ListCopyJobs
- W[x] S[ ] M[ ] F[ ] K[ ] ListFrameworks
- W[x] S[ ] M[ ] F[ ] K[ ] ListIndexedRecoveryPoints
- W[x] S[ ] M[ ] F[ ] K[ ] ListLegalHolds
- W[x] S[ ] M[ ] F[ ] K[ ] ListProtectedResources
- W[x] S[ ] M[ ] F[ ] K[ ] ListProtectedResourcesByBackupVault
- W[x] S[ ] M[ ] F[ ] K[ ] ListRecoveryPointsByBackupVault
- W[x] S[ ] M[ ] F[ ] K[ ] ListRecoveryPointsByLegalHold
- W[x] S[ ] M[ ] F[ ] K[ ] ListRecoveryPointsByResource
- W[x] S[ ] M[ ] F[ ] K[ ] ListReportJobs
- W[x] S[ ] M[x] F[ ] K[ ] ListReportPlans
- W[ ] S[x] M[ ] F[ ] K[ ] ListRestoreAccessBackupVaults
- W[x] S[ ] M[ ] F[ ] K[ ] ListRestoreJobSummaries
- W[x] S[ ] M[ ] F[ ] K[ ] ListRestoreJobs
- W[x] S[ ] M[ ] F[ ] K[ ] ListRestoreJobsByProtectedResource
- W[x] S[ ] M[ ] F[ ] K[ ] ListRestoreTestingPlans
- W[x] S[ ] M[ ] F[ ] K[ ] ListRestoreTestingSelections
- W[x] S[ ] M[ ] F[ ] K[ ] ListScanJobSummaries
- W[x] S[ ] M[ ] F[ ] K[ ] ListScanJobs
- W[x] S[ ] M[x] F[ ] K[ ] ListTags
- W[x] S[ ] M[ ] F[ ] K[ ] ListTieringConfigurations
- W[x] S[ ] M[ ] F[ ] K[ ] PutBackupVaultAccessPolicy
- W[x] S[ ] M[x] F[ ] K[ ] PutBackupVaultLockConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] PutBackupVaultNotifications
- W[x] S[ ] M[ ] F[ ] K[ ] PutRestoreValidationResult
- W[x] S[ ] M[ ] F[ ] K[ ] RevokeRestoreAccessBackupVault
- W[x] S[ ] M[ ] F[ ] K[ ] StartBackupJob
- W[x] S[ ] M[ ] F[ ] K[ ] StartCopyJob
- W[x] S[ ] M[ ] F[ ] K[ ] StartReportJob
- W[x] S[ ] M[ ] F[ ] K[ ] StartRestoreJob
- W[x] S[ ] M[ ] F[ ] K[ ] StartScanJob
- W[x] S[ ] M[ ] F[ ] K[ ] StopBackupJob
- W[x] S[ ] M[x] F[ ] K[ ] TagResource
- W[x] S[ ] M[x] F[ ] K[ ] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateBackupPlan
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateFramework
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateGlobalSettings
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateRecoveryPointIndexSettings
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateRecoveryPointLifecycle
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateRegionSettings
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateReportPlan
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateRestoreTestingPlan
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateRestoreTestingSelection
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateTieringConfiguration

Integration tests: 54/105 implemented operations tested (51.4%)
Untested implemented operations: 51

### winterbaume-backupgateway (backup-gateway) - W: 25/25, S: 0/25, M: 0/25, F: 0/25, K: 0/25

- W[x] S[ ] M[ ] F[ ] K[ ] AssociateGatewayToServer
- W[x] S[ ] M[ ] F[ ] K[ ] CreateGateway
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteGateway
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteHypervisor
- W[x] S[ ] M[ ] F[ ] K[ ] DisassociateGatewayFromServer
- W[x] S[ ] M[ ] F[ ] K[ ] GetBandwidthRateLimitSchedule
- W[x] S[ ] M[ ] F[ ] K[ ] GetGateway
- W[x] S[ ] M[ ] F[ ] K[ ] GetHypervisor
- W[x] S[ ] M[ ] F[ ] K[ ] GetHypervisorPropertyMappings
- W[x] S[ ] M[ ] F[ ] K[ ] GetVirtualMachine
- W[x] S[ ] M[ ] F[ ] K[ ] ImportHypervisorConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] ListGateways
- W[x] S[ ] M[ ] F[ ] K[ ] ListHypervisors
- W[x] S[ ] M[ ] F[ ] K[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] ListVirtualMachines
- W[x] S[ ] M[ ] F[ ] K[ ] PutBandwidthRateLimitSchedule
- W[x] S[ ] M[ ] F[ ] K[ ] PutHypervisorPropertyMappings
- W[x] S[ ] M[ ] F[ ] K[ ] PutMaintenanceStartTime
- W[x] S[ ] M[ ] F[ ] K[ ] StartVirtualMachinesMetadataSync
- W[x] S[ ] M[ ] F[ ] K[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] TestHypervisorConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateGatewayInformation
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateGatewaySoftwareNow
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateHypervisor

Integration tests: 20/25 implemented operations tested (80.0%)
Untested implemented operations: 5

### winterbaume-backupsearch (backupsearch) - W: 9/12, S: 0/12, M: 0/12, F: 0/12, K: 0/12

- W[x] S[ ] M[ ] F[ ] K[ ] GetSearchJob
- W[ ] S[ ] M[ ] F[ ] K[ ] GetSearchResultExportJob
- W[x] S[ ] M[ ] F[ ] K[ ] ListSearchJobBackups
- W[x] S[ ] M[ ] F[ ] K[ ] ListSearchJobResults
- W[x] S[ ] M[ ] F[ ] K[ ] ListSearchJobs
- W[ ] S[ ] M[ ] F[ ] K[ ] ListSearchResultExportJobs
- W[x] S[ ] M[ ] F[ ] K[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] StartSearchJob
- W[ ] S[ ] M[ ] F[ ] K[ ] StartSearchResultExportJob
- W[x] S[ ] M[ ] F[ ] K[ ] StopSearchJob
- W[x] S[ ] M[ ] F[ ] K[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UntagResource

Integration tests: 9/9 implemented operations tested (100.0%)

### winterbaume-batch (batch) - W: 39/45, S: 0/45, M: 24/45, F: 0/45, K: 10/45

Terraform E2E: 8 tests across 4 terraform resource types

Resource types: aws_batch_compute_environment, aws_batch_job_definition, aws_batch_job_queue, aws_batch_scheduling_policy

- W[x] S[ ] M[x] F[ ] K[ ] CancelJob
- W[x] S[ ] M[x] F[ ] K[x] CreateComputeEnvironment
- W[x] S[ ] M[ ] F[ ] K[ ] CreateConsumableResource
- W[x] S[ ] M[x] F[ ] K[x] CreateJobQueue
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateQuotaShare
- W[x] S[ ] M[x] F[ ] K[ ] CreateSchedulingPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] CreateServiceEnvironment
- W[x] S[ ] M[x] F[ ] K[x] DeleteComputeEnvironment
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteConsumableResource
- W[x] S[ ] M[x] F[ ] K[x] DeleteJobQueue
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteQuotaShare
- W[x] S[ ] M[x] F[ ] K[ ] DeleteSchedulingPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteServiceEnvironment
- W[x] S[ ] M[x] F[ ] K[ ] DeregisterJobDefinition
- W[x] S[ ] M[x] F[ ] K[x] DescribeComputeEnvironments
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeConsumableResource
- W[x] S[ ] M[x] F[ ] K[ ] DescribeJobDefinitions
- W[x] S[ ] M[x] F[ ] K[x] DescribeJobQueues
- W[x] S[ ] M[x] F[ ] K[x] DescribeJobs
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeQuotaShare
- W[x] S[ ] M[x] F[ ] K[ ] DescribeSchedulingPolicies
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeServiceEnvironments
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeServiceJob
- W[x] S[ ] M[ ] F[ ] K[ ] GetJobQueueSnapshot
- W[x] S[ ] M[ ] F[ ] K[ ] ListConsumableResources
- W[x] S[ ] M[x] F[ ] K[ ] ListJobs
- W[x] S[ ] M[ ] F[ ] K[ ] ListJobsByConsumableResource
- W[ ] S[ ] M[ ] F[ ] K[ ] ListQuotaShares
- W[x] S[ ] M[x] F[ ] K[ ] ListSchedulingPolicies
- W[x] S[ ] M[ ] F[ ] K[ ] ListServiceJobs
- W[x] S[ ] M[x] F[ ] K[ ] ListTagsForResource
- W[x] S[ ] M[x] F[ ] K[x] RegisterJobDefinition
- W[x] S[ ] M[x] F[ ] K[x] SubmitJob
- W[x] S[ ] M[ ] F[ ] K[ ] SubmitServiceJob
- W[x] S[ ] M[x] F[ ] K[ ] TagResource
- W[x] S[ ] M[x] F[ ] K[x] TerminateJob
- W[x] S[ ] M[ ] F[ ] K[ ] TerminateServiceJob
- W[x] S[ ] M[x] F[ ] K[ ] UntagResource
- W[x] S[ ] M[x] F[ ] K[ ] UpdateComputeEnvironment
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateConsumableResource
- W[x] S[ ] M[x] F[ ] K[ ] UpdateJobQueue
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateQuotaShare
- W[x] S[ ] M[x] F[ ] K[ ] UpdateSchedulingPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateServiceEnvironment
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateServiceJob

Integration tests: 39/39 implemented operations tested (100.0%)

### winterbaume-bcmdashboards (bcm-dashboards) - W: 9/15, S: 0/15, M: 0/15, F: 0/15, K: 0/15

- W[x] S[ ] M[ ] F[ ] K[ ] CreateDashboard
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateScheduledReport
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteDashboard
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteScheduledReport
- W[ ] S[ ] M[ ] F[ ] K[ ] ExecuteScheduledReport
- W[x] S[ ] M[ ] F[ ] K[ ] GetDashboard
- W[x] S[ ] M[ ] F[ ] K[ ] GetResourcePolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] GetScheduledReport
- W[x] S[ ] M[ ] F[ ] K[ ] ListDashboards
- W[ ] S[ ] M[ ] F[ ] K[ ] ListScheduledReports
- W[x] S[ ] M[ ] F[ ] K[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateDashboard
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateScheduledReport

Integration tests: 9/9 implemented operations tested (100.0%)

### winterbaume-bcmdataexports (bcm-data-exports) - W: 12/12, S: 0/12, M: 0/12, F: 0/12, K: 0/12

- W[x] S[ ] M[ ] F[ ] K[ ] CreateExport
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteExport
- W[x] S[ ] M[ ] F[ ] K[ ] GetExecution
- W[x] S[ ] M[ ] F[ ] K[ ] GetExport
- W[x] S[ ] M[ ] F[ ] K[ ] GetTable
- W[x] S[ ] M[ ] F[ ] K[ ] ListExecutions
- W[x] S[ ] M[ ] F[ ] K[ ] ListExports
- W[x] S[ ] M[ ] F[ ] K[ ] ListTables
- W[x] S[ ] M[ ] F[ ] K[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateExport

Integration tests: 10/12 implemented operations tested (83.3%)
Untested implemented operations: 2

### winterbaume-bcmrecommendedactions (bcm-recommended-actions) - W: 1/1, S: 0/1, M: 0/1, F: 0/1, K: 0/1

- W[x] S[ ] M[ ] F[ ] K[ ] ListRecommendedActions

Integration tests: 1/1 implemented operations tested (100.0%)

### winterbaume-bedrock (bedrock) - W: 48/101, S: 0/101, M: 13/101, F: 0/101, K: 0/101

- W[ ] S[ ] M[ ] F[ ] K[ ] BatchDeleteEvaluationJob
- W[ ] S[ ] M[ ] F[ ] K[ ] CancelAutomatedReasoningPolicyBuildWorkflow
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateAutomatedReasoningPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateAutomatedReasoningPolicyTestCase
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateAutomatedReasoningPolicyVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateCustomModel
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateCustomModelDeployment
- W[x] S[ ] M[ ] F[ ] K[ ] CreateEvaluationJob
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateFoundationModelAgreement
- W[x] S[ ] M[ ] F[ ] K[ ] CreateGuardrail
- W[x] S[ ] M[ ] F[ ] K[ ] CreateGuardrailVersion
- W[x] S[ ] M[ ] F[ ] K[ ] CreateInferenceProfile
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateMarketplaceModelEndpoint
- W[x] S[ ] M[ ] F[ ] K[ ] CreateModelCopyJob
- W[x] S[ ] M[x] F[ ] K[ ] CreateModelCustomizationJob
- W[x] S[ ] M[ ] F[ ] K[ ] CreateModelImportJob
- W[x] S[ ] M[ ] F[ ] K[ ] CreateModelInvocationJob
- W[x] S[ ] M[ ] F[ ] K[ ] CreatePromptRouter
- W[x] S[ ] M[ ] F[ ] K[ ] CreateProvisionedModelThroughput
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteAutomatedReasoningPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteAutomatedReasoningPolicyBuildWorkflow
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteAutomatedReasoningPolicyTestCase
- W[x] S[ ] M[x] F[ ] K[ ] DeleteCustomModel
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteCustomModelDeployment
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteEnforcedGuardrailConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteFoundationModelAgreement
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteGuardrail
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteImportedModel
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteInferenceProfile
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteMarketplaceModelEndpoint
- W[x] S[ ] M[x] F[ ] K[ ] DeleteModelInvocationLoggingConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] DeletePromptRouter
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteProvisionedModelThroughput
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteResourcePolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] DeregisterMarketplaceModelEndpoint
- W[ ] S[ ] M[ ] F[ ] K[ ] ExportAutomatedReasoningPolicyVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] GetAutomatedReasoningPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] GetAutomatedReasoningPolicyAnnotations
- W[ ] S[ ] M[ ] F[ ] K[ ] GetAutomatedReasoningPolicyBuildWorkflow
- W[ ] S[ ] M[ ] F[ ] K[ ] GetAutomatedReasoningPolicyBuildWorkflowResultAssets
- W[ ] S[ ] M[ ] F[ ] K[ ] GetAutomatedReasoningPolicyNextScenario
- W[ ] S[ ] M[ ] F[ ] K[ ] GetAutomatedReasoningPolicyTestCase
- W[ ] S[ ] M[ ] F[ ] K[ ] GetAutomatedReasoningPolicyTestResult
- W[x] S[ ] M[x] F[ ] K[ ] GetCustomModel
- W[ ] S[ ] M[ ] F[ ] K[ ] GetCustomModelDeployment
- W[x] S[ ] M[ ] F[ ] K[ ] GetEvaluationJob
- W[x] S[ ] M[ ] F[ ] K[ ] GetFoundationModel
- W[ ] S[ ] M[ ] F[ ] K[ ] GetFoundationModelAvailability
- W[x] S[ ] M[ ] F[ ] K[ ] GetGuardrail
- W[ ] S[ ] M[ ] F[ ] K[ ] GetImportedModel
- W[x] S[ ] M[ ] F[ ] K[ ] GetInferenceProfile
- W[ ] S[ ] M[ ] F[ ] K[ ] GetMarketplaceModelEndpoint
- W[x] S[ ] M[ ] F[ ] K[ ] GetModelCopyJob
- W[x] S[ ] M[x] F[ ] K[ ] GetModelCustomizationJob
- W[x] S[ ] M[ ] F[ ] K[ ] GetModelImportJob
- W[x] S[ ] M[ ] F[ ] K[ ] GetModelInvocationJob
- W[x] S[ ] M[x] F[ ] K[ ] GetModelInvocationLoggingConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] GetPromptRouter
- W[x] S[ ] M[ ] F[ ] K[ ] GetProvisionedModelThroughput
- W[ ] S[ ] M[ ] F[ ] K[ ] GetResourcePolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] GetUseCaseForModelAccess
- W[ ] S[ ] M[ ] F[ ] K[ ] ListAutomatedReasoningPolicies
- W[ ] S[ ] M[ ] F[ ] K[ ] ListAutomatedReasoningPolicyBuildWorkflows
- W[ ] S[ ] M[ ] F[ ] K[ ] ListAutomatedReasoningPolicyTestCases
- W[ ] S[ ] M[ ] F[ ] K[ ] ListAutomatedReasoningPolicyTestResults
- W[ ] S[ ] M[ ] F[ ] K[ ] ListCustomModelDeployments
- W[x] S[ ] M[x] F[ ] K[ ] ListCustomModels
- W[ ] S[ ] M[ ] F[ ] K[ ] ListEnforcedGuardrailsConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] ListEvaluationJobs
- W[ ] S[ ] M[ ] F[ ] K[ ] ListFoundationModelAgreementOffers
- W[x] S[ ] M[ ] F[ ] K[ ] ListFoundationModels
- W[x] S[ ] M[ ] F[ ] K[ ] ListGuardrails
- W[ ] S[ ] M[ ] F[ ] K[ ] ListImportedModels
- W[x] S[ ] M[ ] F[ ] K[ ] ListInferenceProfiles
- W[ ] S[ ] M[ ] F[ ] K[ ] ListMarketplaceModelEndpoints
- W[x] S[ ] M[ ] F[ ] K[ ] ListModelCopyJobs
- W[x] S[ ] M[x] F[ ] K[ ] ListModelCustomizationJobs
- W[x] S[ ] M[ ] F[ ] K[ ] ListModelImportJobs
- W[x] S[ ] M[ ] F[ ] K[ ] ListModelInvocationJobs
- W[x] S[ ] M[ ] F[ ] K[ ] ListPromptRouters
- W[x] S[ ] M[ ] F[ ] K[ ] ListProvisionedModelThroughputs
- W[x] S[ ] M[x] F[ ] K[ ] ListTagsForResource
- W[ ] S[ ] M[ ] F[ ] K[ ] PutEnforcedGuardrailConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] PutModelInvocationLoggingConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] PutResourcePolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] PutUseCaseForModelAccess
- W[ ] S[ ] M[ ] F[ ] K[ ] RegisterMarketplaceModelEndpoint
- W[ ] S[ ] M[ ] F[ ] K[ ] StartAutomatedReasoningPolicyBuildWorkflow
- W[ ] S[ ] M[ ] F[ ] K[ ] StartAutomatedReasoningPolicyTestWorkflow
- W[x] S[ ] M[ ] F[ ] K[ ] StopEvaluationJob
- W[x] S[ ] M[x] F[ ] K[ ] StopModelCustomizationJob
- W[x] S[ ] M[ ] F[ ] K[ ] StopModelInvocationJob
- W[x] S[ ] M[x] F[ ] K[ ] TagResource
- W[x] S[ ] M[x] F[ ] K[ ] UntagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateAutomatedReasoningPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateAutomatedReasoningPolicyAnnotations
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateAutomatedReasoningPolicyTestCase
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateCustomModelDeployment
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateGuardrail
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateMarketplaceModelEndpoint
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateProvisionedModelThroughput

Integration tests: 43/48 implemented operations tested (89.6%)
Untested implemented operations: 5

### winterbaume-bedrockagent (bedrock-agent) - W: 72/72, S: 0/72, M: 11/72, F: 0/72, K: 0/72

Terraform E2E: 9 tests across 6 terraform resource types

Resource types: aws_bedrockagent_agent, aws_bedrockagent_agent_action_group, aws_bedrockagent_agent_alias, aws_bedrockagent_agent_knowledge_base_association, aws_bedrockagent_data_source, aws_bedrockagent_knowledge_base

- W[x] S[ ] M[ ] F[ ] K[ ] AssociateAgentCollaborator
- W[x] S[ ] M[ ] F[ ] K[ ] AssociateAgentKnowledgeBase
- W[x] S[ ] M[x] F[ ] K[ ] CreateAgent
- W[x] S[ ] M[ ] F[ ] K[ ] CreateAgentActionGroup
- W[x] S[ ] M[ ] F[ ] K[ ] CreateAgentAlias
- W[x] S[ ] M[ ] F[ ] K[ ] CreateDataSource
- W[x] S[ ] M[ ] F[ ] K[ ] CreateFlow
- W[x] S[ ] M[ ] F[ ] K[ ] CreateFlowAlias
- W[x] S[ ] M[ ] F[ ] K[ ] CreateFlowVersion
- W[x] S[ ] M[x] F[ ] K[ ] CreateKnowledgeBase
- W[x] S[ ] M[ ] F[ ] K[ ] CreatePrompt
- W[x] S[ ] M[ ] F[ ] K[ ] CreatePromptVersion
- W[x] S[ ] M[x] F[ ] K[ ] DeleteAgent
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteAgentActionGroup
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteAgentAlias
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteAgentVersion
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteDataSource
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteFlow
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteFlowAlias
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteFlowVersion
- W[x] S[ ] M[x] F[ ] K[ ] DeleteKnowledgeBase
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteKnowledgeBaseDocuments
- W[x] S[ ] M[ ] F[ ] K[ ] DeletePrompt
- W[x] S[ ] M[ ] F[ ] K[ ] DisassociateAgentCollaborator
- W[x] S[ ] M[ ] F[ ] K[ ] DisassociateAgentKnowledgeBase
- W[x] S[ ] M[x] F[ ] K[ ] GetAgent
- W[x] S[ ] M[ ] F[ ] K[ ] GetAgentActionGroup
- W[x] S[ ] M[ ] F[ ] K[ ] GetAgentAlias
- W[x] S[ ] M[ ] F[ ] K[ ] GetAgentCollaborator
- W[x] S[ ] M[ ] F[ ] K[ ] GetAgentKnowledgeBase
- W[x] S[ ] M[ ] F[ ] K[ ] GetAgentVersion
- W[x] S[ ] M[ ] F[ ] K[ ] GetDataSource
- W[x] S[ ] M[ ] F[ ] K[ ] GetFlow
- W[x] S[ ] M[ ] F[ ] K[ ] GetFlowAlias
- W[x] S[ ] M[ ] F[ ] K[ ] GetFlowVersion
- W[x] S[ ] M[ ] F[ ] K[ ] GetIngestionJob
- W[x] S[ ] M[x] F[ ] K[ ] GetKnowledgeBase
- W[x] S[ ] M[ ] F[ ] K[ ] GetKnowledgeBaseDocuments
- W[x] S[ ] M[ ] F[ ] K[ ] GetPrompt
- W[x] S[ ] M[ ] F[ ] K[ ] IngestKnowledgeBaseDocuments
- W[x] S[ ] M[ ] F[ ] K[ ] ListAgentActionGroups
- W[x] S[ ] M[ ] F[ ] K[ ] ListAgentAliases
- W[x] S[ ] M[ ] F[ ] K[ ] ListAgentCollaborators
- W[x] S[ ] M[ ] F[ ] K[ ] ListAgentKnowledgeBases
- W[x] S[ ] M[ ] F[ ] K[ ] ListAgentVersions
- W[x] S[ ] M[x] F[ ] K[ ] ListAgents
- W[x] S[ ] M[ ] F[ ] K[ ] ListDataSources
- W[x] S[ ] M[ ] F[ ] K[ ] ListFlowAliases
- W[x] S[ ] M[ ] F[ ] K[ ] ListFlowVersions
- W[x] S[ ] M[ ] F[ ] K[ ] ListFlows
- W[x] S[ ] M[ ] F[ ] K[ ] ListIngestionJobs
- W[x] S[ ] M[ ] F[ ] K[ ] ListKnowledgeBaseDocuments
- W[x] S[ ] M[x] F[ ] K[ ] ListKnowledgeBases
- W[x] S[ ] M[ ] F[ ] K[ ] ListPrompts
- W[x] S[ ] M[x] F[ ] K[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] PrepareAgent
- W[x] S[ ] M[ ] F[ ] K[ ] PrepareFlow
- W[x] S[ ] M[ ] F[ ] K[ ] StartIngestionJob
- W[x] S[ ] M[ ] F[ ] K[ ] StopIngestionJob
- W[x] S[ ] M[x] F[ ] K[ ] TagResource
- W[x] S[ ] M[x] F[ ] K[ ] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateAgent
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateAgentActionGroup
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateAgentAlias
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateAgentCollaborator
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateAgentKnowledgeBase
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateDataSource
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateFlow
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateFlowAlias
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateKnowledgeBase
- W[x] S[ ] M[ ] F[ ] K[ ] UpdatePrompt
- W[x] S[ ] M[ ] F[ ] K[ ] ValidateFlowDefinition

Integration tests: 71/72 implemented operations tested (98.6%)
Untested implemented operations: 1

### winterbaume-billing (billing) - W: 12/12, S: 0/12, M: 0/12, F: 0/12, K: 0/12

- W[x] S[ ] M[ ] F[ ] K[ ] AssociateSourceViews
- W[x] S[ ] M[ ] F[ ] K[ ] CreateBillingView
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteBillingView
- W[x] S[ ] M[ ] F[ ] K[ ] DisassociateSourceViews
- W[x] S[ ] M[ ] F[ ] K[ ] GetBillingView
- W[x] S[ ] M[ ] F[ ] K[ ] GetResourcePolicy
- W[x] S[ ] M[ ] F[ ] K[ ] ListBillingViews
- W[x] S[ ] M[ ] F[ ] K[ ] ListSourceViewsForBillingView
- W[x] S[ ] M[ ] F[ ] K[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateBillingView

Integration tests: 11/12 implemented operations tested (91.7%)
Untested implemented operations: 1

### winterbaume-braket (braket) - W: 12/17, S: 0/17, M: 0/17, F: 0/17, K: 0/17

- W[x] S[ ] M[ ] F[ ] K[ ] CancelJob
- W[ ] S[ ] M[ ] F[ ] K[ ] CancelQuantumTask
- W[x] S[ ] M[ ] F[ ] K[ ] CreateJob
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateQuantumTask
- W[x] S[ ] M[ ] F[ ] K[ ] CreateSpendingLimit
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteSpendingLimit
- W[x] S[ ] M[ ] F[ ] K[ ] GetDevice
- W[x] S[ ] M[ ] F[ ] K[ ] GetJob
- W[ ] S[ ] M[ ] F[ ] K[ ] GetQuantumTask
- W[ ] S[ ] M[ ] F[ ] K[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] SearchDevices
- W[x] S[ ] M[ ] F[ ] K[ ] SearchJobs
- W[ ] S[ ] M[ ] F[ ] K[ ] SearchQuantumTasks
- W[x] S[ ] M[ ] F[ ] K[ ] SearchSpendingLimits
- W[x] S[ ] M[ ] F[ ] K[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateSpendingLimit

Integration tests: 8/12 implemented operations tested (66.7%)
Untested implemented operations: 4

### winterbaume-budgets (budgets) - W: 7/26, S: 0/26, M: 7/26, F: 0/26, K: 0/26

- W[x] S[ ] M[x] F[ ] K[ ] CreateBudget
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateBudgetAction
- W[x] S[ ] M[x] F[ ] K[ ] CreateNotification
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateSubscriber
- W[x] S[ ] M[x] F[ ] K[ ] DeleteBudget
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteBudgetAction
- W[x] S[ ] M[x] F[ ] K[ ] DeleteNotification
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteSubscriber
- W[x] S[ ] M[x] F[ ] K[ ] DescribeBudget
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeBudgetAction
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeBudgetActionHistories
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeBudgetActionsForAccount
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeBudgetActionsForBudget
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeBudgetNotificationsForAccount
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeBudgetPerformanceHistory
- W[x] S[ ] M[x] F[ ] K[ ] DescribeBudgets
- W[x] S[ ] M[x] F[ ] K[ ] DescribeNotificationsForBudget
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeSubscribersForNotification
- W[ ] S[ ] M[ ] F[ ] K[ ] ExecuteBudgetAction
- W[ ] S[ ] M[ ] F[ ] K[ ] ListTagsForResource
- W[ ] S[ ] M[ ] F[ ] K[ ] TagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] UntagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateBudget
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateBudgetAction
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateNotification
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateSubscriber

Integration tests: 7/7 implemented operations tested (100.0%)

### winterbaume-chatbot (chatbot) - W: 15/34, S: 0/34, M: 0/34, F: 0/34, K: 0/34

- W[ ] S[ ] M[ ] F[ ] K[ ] AssociateToConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] CreateChimeWebhookConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateCustomAction
- W[x] S[ ] M[ ] F[ ] K[ ] CreateMicrosoftTeamsChannelConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] CreateSlackChannelConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteChimeWebhookConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteCustomAction
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteMicrosoftTeamsChannelConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteMicrosoftTeamsConfiguredTeam
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteMicrosoftTeamsUserIdentity
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteSlackChannelConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteSlackUserIdentity
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteSlackWorkspaceAuthorization
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeChimeWebhookConfigurations
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeSlackChannelConfigurations
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeSlackUserIdentities
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeSlackWorkspaces
- W[ ] S[ ] M[ ] F[ ] K[ ] DisassociateFromConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] GetAccountPreferences
- W[ ] S[ ] M[ ] F[ ] K[ ] GetCustomAction
- W[x] S[ ] M[ ] F[ ] K[ ] GetMicrosoftTeamsChannelConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] ListAssociations
- W[ ] S[ ] M[ ] F[ ] K[ ] ListCustomActions
- W[x] S[ ] M[ ] F[ ] K[ ] ListMicrosoftTeamsChannelConfigurations
- W[ ] S[ ] M[ ] F[ ] K[ ] ListMicrosoftTeamsConfiguredTeams
- W[ ] S[ ] M[ ] F[ ] K[ ] ListMicrosoftTeamsUserIdentities
- W[x] S[ ] M[ ] F[ ] K[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UntagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateAccountPreferences
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateChimeWebhookConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateCustomAction
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateMicrosoftTeamsChannelConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateSlackChannelConfiguration

Integration tests: 15/15 implemented operations tested (100.0%)

### winterbaume-chimesdkmeetings (chime-sdk-meetings) - W: 12/16, S: 0/16, M: 0/16, F: 0/16, K: 0/16

- W[x] S[ ] M[ ] F[ ] K[ ] BatchCreateAttendee
- W[ ] S[ ] M[ ] F[ ] K[ ] BatchUpdateAttendeeCapabilitiesExcept
- W[x] S[ ] M[ ] F[ ] K[ ] CreateAttendee
- W[x] S[ ] M[ ] F[ ] K[ ] CreateMeeting
- W[x] S[ ] M[ ] F[ ] K[ ] CreateMeetingWithAttendees
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteAttendee
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteMeeting
- W[x] S[ ] M[ ] F[ ] K[ ] GetAttendee
- W[x] S[ ] M[ ] F[ ] K[ ] GetMeeting
- W[x] S[ ] M[ ] F[ ] K[ ] ListAttendees
- W[ ] S[ ] M[ ] F[ ] K[ ] ListTagsForResource
- W[ ] S[ ] M[ ] F[ ] K[ ] StartMeetingTranscription
- W[ ] S[ ] M[ ] F[ ] K[ ] StopMeetingTranscription
- W[x] S[ ] M[ ] F[ ] K[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateAttendeeCapabilities

Integration tests: 8/12 implemented operations tested (66.7%)
Untested implemented operations: 4

### winterbaume-cloud9 (cloud9) - W: 13/13, S: 0/13, M: 0/13, F: 0/13, K: 0/13

- W[x] S[ ] M[ ] F[ ] K[ ] CreateEnvironmentEC2
- W[x] S[ ] M[ ] F[ ] K[ ] CreateEnvironmentMembership
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteEnvironment
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteEnvironmentMembership
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeEnvironmentMemberships
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeEnvironmentStatus
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeEnvironments
- W[x] S[ ] M[ ] F[ ] K[ ] ListEnvironments
- W[x] S[ ] M[ ] F[ ] K[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateEnvironment
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateEnvironmentMembership

Integration tests: 13/13 implemented operations tested (100.0%)

### winterbaume-cloudcontrol (cloudcontrol) - W: 8/8, S: 0/8, M: 0/8, F: 0/8, K: 0/8

- W[x] S[ ] M[ ] F[ ] K[ ] CancelResourceRequest
- W[x] S[ ] M[ ] F[ ] K[ ] CreateResource
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteResource
- W[x] S[ ] M[ ] F[ ] K[ ] GetResource
- W[x] S[ ] M[ ] F[ ] K[ ] GetResourceRequestStatus
- W[x] S[ ] M[ ] F[ ] K[ ] ListResourceRequests
- W[x] S[ ] M[ ] F[ ] K[ ] ListResources
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateResource

Integration tests: 8/8 implemented operations tested (100.0%)

### winterbaume-clouddirectory (clouddirectory) - W: 13/66, S: 0/66, M: 13/66, F: 0/66, K: 0/66

- W[ ] S[ ] M[ ] F[ ] K[ ] AddFacetToObject
- W[x] S[ ] M[x] F[ ] K[ ] ApplySchema
- W[ ] S[ ] M[ ] F[ ] K[ ] AttachObject
- W[ ] S[ ] M[ ] F[ ] K[ ] AttachPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] AttachToIndex
- W[ ] S[ ] M[ ] F[ ] K[ ] AttachTypedLink
- W[ ] S[ ] M[ ] F[ ] K[ ] BatchRead
- W[ ] S[ ] M[ ] F[ ] K[ ] BatchWrite
- W[x] S[ ] M[x] F[ ] K[ ] CreateDirectory
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateFacet
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateIndex
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateObject
- W[x] S[ ] M[x] F[ ] K[ ] CreateSchema
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateTypedLinkFacet
- W[x] S[ ] M[x] F[ ] K[ ] DeleteDirectory
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteFacet
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteObject
- W[x] S[ ] M[x] F[ ] K[ ] DeleteSchema
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteTypedLinkFacet
- W[ ] S[ ] M[ ] F[ ] K[ ] DetachFromIndex
- W[ ] S[ ] M[ ] F[ ] K[ ] DetachObject
- W[ ] S[ ] M[ ] F[ ] K[ ] DetachPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] DetachTypedLink
- W[ ] S[ ] M[ ] F[ ] K[ ] DisableDirectory
- W[ ] S[ ] M[ ] F[ ] K[ ] EnableDirectory
- W[ ] S[ ] M[ ] F[ ] K[ ] GetAppliedSchemaVersion
- W[x] S[ ] M[x] F[ ] K[ ] GetDirectory
- W[ ] S[ ] M[ ] F[ ] K[ ] GetFacet
- W[ ] S[ ] M[ ] F[ ] K[ ] GetLinkAttributes
- W[ ] S[ ] M[ ] F[ ] K[ ] GetObjectAttributes
- W[ ] S[ ] M[ ] F[ ] K[ ] GetObjectInformation
- W[ ] S[ ] M[ ] F[ ] K[ ] GetSchemaAsJson
- W[ ] S[ ] M[ ] F[ ] K[ ] GetTypedLinkFacetInformation
- W[ ] S[ ] M[ ] F[ ] K[ ] ListAppliedSchemaArns
- W[ ] S[ ] M[ ] F[ ] K[ ] ListAttachedIndices
- W[x] S[ ] M[x] F[ ] K[ ] ListDevelopmentSchemaArns
- W[x] S[ ] M[x] F[ ] K[ ] ListDirectories
- W[ ] S[ ] M[ ] F[ ] K[ ] ListFacetAttributes
- W[ ] S[ ] M[ ] F[ ] K[ ] ListFacetNames
- W[ ] S[ ] M[ ] F[ ] K[ ] ListIncomingTypedLinks
- W[ ] S[ ] M[ ] F[ ] K[ ] ListIndex
- W[ ] S[ ] M[ ] F[ ] K[ ] ListManagedSchemaArns
- W[ ] S[ ] M[ ] F[ ] K[ ] ListObjectAttributes
- W[ ] S[ ] M[ ] F[ ] K[ ] ListObjectChildren
- W[ ] S[ ] M[ ] F[ ] K[ ] ListObjectParentPaths
- W[ ] S[ ] M[ ] F[ ] K[ ] ListObjectParents
- W[ ] S[ ] M[ ] F[ ] K[ ] ListObjectPolicies
- W[ ] S[ ] M[ ] F[ ] K[ ] ListOutgoingTypedLinks
- W[ ] S[ ] M[ ] F[ ] K[ ] ListPolicyAttachments
- W[x] S[ ] M[x] F[ ] K[ ] ListPublishedSchemaArns
- W[x] S[ ] M[x] F[ ] K[ ] ListTagsForResource
- W[ ] S[ ] M[ ] F[ ] K[ ] ListTypedLinkFacetAttributes
- W[ ] S[ ] M[ ] F[ ] K[ ] ListTypedLinkFacetNames
- W[ ] S[ ] M[ ] F[ ] K[ ] LookupPolicy
- W[x] S[ ] M[x] F[ ] K[ ] PublishSchema
- W[ ] S[ ] M[ ] F[ ] K[ ] PutSchemaFromJson
- W[ ] S[ ] M[ ] F[ ] K[ ] RemoveFacetFromObject
- W[x] S[ ] M[x] F[ ] K[ ] TagResource
- W[x] S[ ] M[x] F[ ] K[ ] UntagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateFacet
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateLinkAttributes
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateObjectAttributes
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateSchema
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateTypedLinkFacet
- W[ ] S[ ] M[ ] F[ ] K[ ] UpgradeAppliedSchema
- W[ ] S[ ] M[ ] F[ ] K[ ] UpgradePublishedSchema

Integration tests: 12/13 implemented operations tested (92.3%)
Untested implemented operations: 1

### winterbaume-cloudformation (cloudformation) - W: 40/90, S: 3/90, M: 33/90, F: 0/90, K: 8/90

- W[ ] S[ ] M[ ] F[ ] K[ ] ActivateOrganizationsAccess
- W[ ] S[ ] M[ ] F[ ] K[ ] ActivateType
- W[ ] S[ ] M[ ] F[ ] K[ ] BatchDescribeTypeConfigurations
- W[x] S[ ] M[ ] F[ ] K[ ] CancelUpdateStack
- W[x] S[ ] M[ ] F[ ] K[ ] ContinueUpdateRollback
- W[x] S[ ] M[x] F[ ] K[ ] CreateChangeSet
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateGeneratedTemplate
- W[x] S[ ] M[x] F[ ] K[x] CreateStack
- W[x] S[ ] M[x] F[ ] K[ ] CreateStackInstances
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateStackRefactor
- W[x] S[ ] M[x] F[ ] K[ ] CreateStackSet
- W[ ] S[ ] M[ ] F[ ] K[ ] DeactivateOrganizationsAccess
- W[ ] S[ ] M[ ] F[ ] K[ ] DeactivateType
- W[x] S[ ] M[x] F[ ] K[ ] DeleteChangeSet
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteGeneratedTemplate
- W[x] S[ ] M[x] F[ ] K[x] DeleteStack
- W[x] S[ ] M[x] F[ ] K[ ] DeleteStackInstances
- W[x] S[ ] M[x] F[ ] K[ ] DeleteStackSet
- W[ ] S[ ] M[ ] F[ ] K[ ] DeregisterType
- W[ ] S[x] M[ ] F[ ] K[ ] DescribeAccountLimits
- W[x] S[ ] M[x] F[ ] K[ ] DescribeChangeSet
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeChangeSetHooks
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeEvents
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeGeneratedTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeOrganizationsAccess
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribePublisher
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeResourceScan
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeStackDriftDetectionStatus
- W[x] S[ ] M[x] F[ ] K[ ] DescribeStackEvents
- W[x] S[ ] M[x] F[ ] K[ ] DescribeStackInstance
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeStackRefactor
- W[x] S[ ] M[x] F[ ] K[ ] DescribeStackResource
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeStackResourceDrifts
- W[x] S[ ] M[x] F[ ] K[x] DescribeStackResources
- W[x] S[ ] M[x] F[ ] K[ ] DescribeStackSet
- W[x] S[ ] M[x] F[ ] K[ ] DescribeStackSetOperation
- W[x] S[ ] M[x] F[ ] K[x] DescribeStacks
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeType
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeTypeRegistration
- W[ ] S[ ] M[ ] F[ ] K[ ] DetectStackDrift
- W[ ] S[ ] M[ ] F[ ] K[ ] DetectStackResourceDrift
- W[ ] S[ ] M[ ] F[ ] K[ ] DetectStackSetDrift
- W[ ] S[x] M[ ] F[ ] K[ ] EstimateTemplateCost
- W[x] S[ ] M[x] F[ ] K[ ] ExecuteChangeSet
- W[ ] S[ ] M[ ] F[ ] K[ ] ExecuteStackRefactor
- W[ ] S[ ] M[ ] F[ ] K[ ] GetGeneratedTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] GetHookResult
- W[x] S[ ] M[x] F[ ] K[ ] GetStackPolicy
- W[x] S[ ] M[x] F[ ] K[x] GetTemplate
- W[x] S[ ] M[ ] F[ ] K[ ] GetTemplateSummary
- W[ ] S[ ] M[ ] F[ ] K[ ] ImportStacksToStackSet
- W[x] S[ ] M[x] F[ ] K[ ] ListChangeSets
- W[x] S[ ] M[x] F[ ] K[ ] ListExports
- W[ ] S[ ] M[ ] F[ ] K[ ] ListGeneratedTemplates
- W[ ] S[ ] M[ ] F[ ] K[ ] ListHookResults
- W[x] S[ ] M[ ] F[ ] K[ ] ListImports
- W[ ] S[ ] M[ ] F[ ] K[ ] ListResourceScanRelatedResources
- W[ ] S[ ] M[ ] F[ ] K[ ] ListResourceScanResources
- W[ ] S[ ] M[ ] F[ ] K[ ] ListResourceScans
- W[ ] S[ ] M[ ] F[ ] K[ ] ListStackInstanceResourceDrifts
- W[x] S[ ] M[x] F[ ] K[ ] ListStackInstances
- W[ ] S[ ] M[ ] F[ ] K[ ] ListStackRefactorActions
- W[ ] S[ ] M[ ] F[ ] K[ ] ListStackRefactors
- W[x] S[ ] M[x] F[ ] K[ ] ListStackResources
- W[ ] S[ ] M[ ] F[ ] K[ ] ListStackSetAutoDeploymentTargets
- W[x] S[ ] M[x] F[ ] K[ ] ListStackSetOperationResults
- W[x] S[ ] M[x] F[ ] K[ ] ListStackSetOperations
- W[x] S[ ] M[x] F[ ] K[ ] ListStackSets
- W[x] S[ ] M[x] F[ ] K[x] ListStacks
- W[ ] S[ ] M[ ] F[ ] K[ ] ListTypeRegistrations
- W[ ] S[ ] M[ ] F[ ] K[ ] ListTypeVersions
- W[x] S[ ] M[ ] F[ ] K[ ] ListTypes
- W[ ] S[ ] M[ ] F[ ] K[ ] PublishType
- W[ ] S[ ] M[ ] F[ ] K[ ] RecordHandlerProgress
- W[ ] S[ ] M[ ] F[ ] K[ ] RegisterPublisher
- W[ ] S[ ] M[ ] F[ ] K[ ] RegisterType
- W[x] S[ ] M[ ] F[ ] K[ ] RollbackStack
- W[x] S[ ] M[x] F[ ] K[ ] SetStackPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] SetTypeConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] SetTypeDefaultVersion
- W[x] S[ ] M[ ] F[ ] K[ ] SignalResource
- W[ ] S[ ] M[ ] F[ ] K[ ] StartResourceScan
- W[x] S[ ] M[x] F[ ] K[ ] StopStackSetOperation
- W[ ] S[ ] M[ ] F[ ] K[ ] TestType
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateGeneratedTemplate
- W[x] S[ ] M[x] F[ ] K[x] UpdateStack
- W[x] S[ ] M[x] F[ ] K[ ] UpdateStackInstances
- W[x] S[ ] M[x] F[ ] K[ ] UpdateStackSet
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateTerminationProtection
- W[ ] S[x] M[x] F[ ] K[x] ValidateTemplate

Integration tests: 39/40 implemented operations tested (97.5%)
Untested implemented operations: 1

### winterbaume-cloudfront (cloudfront) - W: 156/167, S: 11/167, M: 25/167, F: 0/167, K: 8/167

Terraform E2E: 6 tests across 2 terraform resource types

Resource types: aws_cloudfront_distribution, aws_cloudfront_origin_access_control

- W[x] S[ ] M[ ] F[ ] K[ ] AssociateAlias
- W[x] S[ ] M[ ] F[ ] K[ ] AssociateDistributionTenantWebACL
- W[x] S[ ] M[ ] F[ ] K[ ] AssociateDistributionWebACL
- W[x] S[ ] M[ ] F[ ] K[ ] CopyDistribution
- W[x] S[ ] M[ ] F[ ] K[ ] CreateAnycastIpList
- W[x] S[ ] M[ ] F[ ] K[ ] CreateCachePolicy
- W[x] S[ ] M[ ] F[ ] K[ ] CreateCloudFrontOriginAccessIdentity
- W[x] S[ ] M[ ] F[ ] K[ ] CreateConnectionFunction
- W[x] S[ ] M[ ] F[ ] K[ ] CreateConnectionGroup
- W[x] S[ ] M[ ] F[ ] K[ ] CreateContinuousDeploymentPolicy
- W[x] S[ ] M[x] F[ ] K[x] CreateDistribution
- W[x] S[ ] M[ ] F[ ] K[ ] CreateDistributionTenant
- W[x] S[ ] M[x] F[ ] K[ ] CreateDistributionWithTags
- W[x] S[ ] M[ ] F[ ] K[ ] CreateFieldLevelEncryptionConfig
- W[x] S[ ] M[ ] F[ ] K[ ] CreateFieldLevelEncryptionProfile
- W[x] S[ ] M[ ] F[ ] K[ ] CreateFunction
- W[x] S[ ] M[x] F[ ] K[x] CreateInvalidation
- W[x] S[ ] M[ ] F[ ] K[ ] CreateInvalidationForDistributionTenant
- W[x] S[ ] M[x] F[ ] K[ ] CreateKeyGroup
- W[x] S[ ] M[ ] F[ ] K[ ] CreateKeyValueStore
- W[x] S[ ] M[ ] F[ ] K[ ] CreateMonitoringSubscription
- W[x] S[ ] M[x] F[ ] K[ ] CreateOriginAccessControl
- W[x] S[ ] M[ ] F[ ] K[ ] CreateOriginRequestPolicy
- W[x] S[ ] M[x] F[ ] K[ ] CreatePublicKey
- W[x] S[ ] M[ ] F[ ] K[ ] CreateRealtimeLogConfig
- W[x] S[ ] M[ ] F[ ] K[ ] CreateResponseHeadersPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] CreateStreamingDistribution
- W[x] S[ ] M[ ] F[ ] K[ ] CreateStreamingDistributionWithTags
- W[x] S[ ] M[ ] F[ ] K[ ] CreateTrustStore
- W[x] S[ ] M[ ] F[ ] K[ ] CreateVpcOrigin
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteAnycastIpList
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteCachePolicy
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteCloudFrontOriginAccessIdentity
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteConnectionFunction
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteConnectionGroup
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteContinuousDeploymentPolicy
- W[x] S[ ] M[x] F[ ] K[x] DeleteDistribution
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteDistributionTenant
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteFieldLevelEncryptionConfig
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteFieldLevelEncryptionProfile
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteFunction
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteKeyGroup
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteKeyValueStore
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteMonitoringSubscription
- W[x] S[ ] M[x] F[ ] K[ ] DeleteOriginAccessControl
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteOriginRequestPolicy
- W[x] S[ ] M[x] F[ ] K[ ] DeletePublicKey
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteRealtimeLogConfig
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteResourcePolicy
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteResponseHeadersPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteStreamingDistribution
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteTrustStore
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteVpcOrigin
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeConnectionFunction
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeFunction
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeKeyValueStore
- W[x] S[ ] M[ ] F[ ] K[ ] DisassociateDistributionTenantWebACL
- W[x] S[ ] M[ ] F[ ] K[ ] DisassociateDistributionWebACL
- W[x] S[ ] M[ ] F[ ] K[ ] GetAnycastIpList
- W[x] S[ ] M[ ] F[ ] K[ ] GetCachePolicy
- W[x] S[ ] M[ ] F[ ] K[ ] GetCachePolicyConfig
- W[x] S[ ] M[ ] F[ ] K[ ] GetCloudFrontOriginAccessIdentity
- W[x] S[ ] M[ ] F[ ] K[ ] GetCloudFrontOriginAccessIdentityConfig
- W[x] S[ ] M[ ] F[ ] K[ ] GetConnectionFunction
- W[x] S[ ] M[ ] F[ ] K[ ] GetConnectionGroup
- W[x] S[ ] M[ ] F[ ] K[ ] GetConnectionGroupByRoutingEndpoint
- W[x] S[ ] M[ ] F[ ] K[ ] GetContinuousDeploymentPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] GetContinuousDeploymentPolicyConfig
- W[x] S[ ] M[x] F[ ] K[x] GetDistribution
- W[x] S[ ] M[x] F[ ] K[x] GetDistributionConfig
- W[x] S[ ] M[ ] F[ ] K[ ] GetDistributionTenant
- W[x] S[ ] M[ ] F[ ] K[ ] GetDistributionTenantByDomain
- W[x] S[ ] M[ ] F[ ] K[ ] GetFieldLevelEncryption
- W[x] S[ ] M[ ] F[ ] K[ ] GetFieldLevelEncryptionConfig
- W[x] S[ ] M[ ] F[ ] K[ ] GetFieldLevelEncryptionProfile
- W[x] S[ ] M[ ] F[ ] K[ ] GetFieldLevelEncryptionProfileConfig
- W[x] S[ ] M[ ] F[ ] K[ ] GetFunction
- W[x] S[ ] M[x] F[ ] K[x] GetInvalidation
- W[x] S[ ] M[ ] F[ ] K[ ] GetInvalidationForDistributionTenant
- W[x] S[ ] M[x] F[ ] K[ ] GetKeyGroup
- W[x] S[ ] M[ ] F[ ] K[ ] GetKeyGroupConfig
- W[ ] S[x] M[ ] F[ ] K[ ] GetManagedCertificateDetails
- W[x] S[ ] M[ ] F[ ] K[ ] GetMonitoringSubscription
- W[x] S[ ] M[x] F[ ] K[ ] GetOriginAccessControl
- W[x] S[ ] M[ ] F[ ] K[ ] GetOriginAccessControlConfig
- W[x] S[ ] M[ ] F[ ] K[ ] GetOriginRequestPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] GetOriginRequestPolicyConfig
- W[x] S[ ] M[x] F[ ] K[ ] GetPublicKey
- W[x] S[ ] M[ ] F[ ] K[ ] GetPublicKeyConfig
- W[x] S[ ] M[ ] F[ ] K[ ] GetRealtimeLogConfig
- W[x] S[ ] M[ ] F[ ] K[ ] GetResourcePolicy
- W[x] S[ ] M[ ] F[ ] K[ ] GetResponseHeadersPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] GetResponseHeadersPolicyConfig
- W[x] S[ ] M[ ] F[ ] K[ ] GetStreamingDistribution
- W[x] S[ ] M[ ] F[ ] K[ ] GetStreamingDistributionConfig
- W[x] S[ ] M[ ] F[ ] K[ ] GetTrustStore
- W[x] S[ ] M[ ] F[ ] K[ ] GetVpcOrigin
- W[x] S[ ] M[ ] F[ ] K[ ] ListAnycastIpLists
- W[x] S[ ] M[ ] F[ ] K[ ] ListCachePolicies
- W[x] S[ ] M[ ] F[ ] K[ ] ListCloudFrontOriginAccessIdentities
- W[ ] S[x] M[ ] F[ ] K[ ] ListConflictingAliases
- W[x] S[ ] M[ ] F[ ] K[ ] ListConnectionFunctions
- W[x] S[ ] M[ ] F[ ] K[ ] ListConnectionGroups
- W[x] S[ ] M[ ] F[ ] K[ ] ListContinuousDeploymentPolicies
- W[x] S[ ] M[ ] F[ ] K[ ] ListDistributionTenants
- W[x] S[ ] M[ ] F[ ] K[ ] ListDistributionTenantsByCustomization
- W[x] S[ ] M[x] F[ ] K[x] ListDistributions
- W[x] S[ ] M[ ] F[ ] K[ ] ListDistributionsByAnycastIpListId
- W[x] S[ ] M[ ] F[ ] K[ ] ListDistributionsByCachePolicyId
- W[ ] S[x] M[ ] F[ ] K[ ] ListDistributionsByConnectionFunction
- W[ ] S[x] M[ ] F[ ] K[ ] ListDistributionsByConnectionMode
- W[x] S[ ] M[ ] F[ ] K[ ] ListDistributionsByKeyGroup
- W[x] S[ ] M[ ] F[ ] K[ ] ListDistributionsByOriginRequestPolicyId
- W[ ] S[x] M[ ] F[ ] K[ ] ListDistributionsByOwnedResource
- W[ ] S[x] M[ ] F[ ] K[ ] ListDistributionsByRealtimeLogConfig
- W[x] S[ ] M[ ] F[ ] K[ ] ListDistributionsByResponseHeadersPolicyId
- W[ ] S[x] M[ ] F[ ] K[ ] ListDistributionsByTrustStore
- W[x] S[ ] M[ ] F[ ] K[ ] ListDistributionsByVpcOriginId
- W[x] S[ ] M[ ] F[ ] K[ ] ListDistributionsByWebACLId
- W[ ] S[x] M[ ] F[ ] K[ ] ListDomainConflicts
- W[x] S[ ] M[ ] F[ ] K[ ] ListFieldLevelEncryptionConfigs
- W[x] S[ ] M[ ] F[ ] K[ ] ListFieldLevelEncryptionProfiles
- W[x] S[ ] M[ ] F[ ] K[ ] ListFunctions
- W[x] S[ ] M[x] F[ ] K[ ] ListInvalidations
- W[x] S[ ] M[ ] F[ ] K[ ] ListInvalidationsForDistributionTenant
- W[x] S[ ] M[x] F[ ] K[ ] ListKeyGroups
- W[x] S[ ] M[ ] F[ ] K[ ] ListKeyValueStores
- W[x] S[ ] M[x] F[ ] K[ ] ListOriginAccessControls
- W[x] S[ ] M[ ] F[ ] K[ ] ListOriginRequestPolicies
- W[x] S[ ] M[x] F[ ] K[ ] ListPublicKeys
- W[x] S[ ] M[ ] F[ ] K[ ] ListRealtimeLogConfigs
- W[x] S[ ] M[ ] F[ ] K[ ] ListResponseHeadersPolicies
- W[x] S[ ] M[ ] F[ ] K[ ] ListStreamingDistributions
- W[x] S[ ] M[x] F[ ] K[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] ListTrustStores
- W[x] S[ ] M[ ] F[ ] K[ ] ListVpcOrigins
- W[x] S[ ] M[ ] F[ ] K[ ] PublishConnectionFunction
- W[x] S[ ] M[ ] F[ ] K[ ] PublishFunction
- W[x] S[ ] M[ ] F[ ] K[ ] PutResourcePolicy
- W[x] S[ ] M[x] F[ ] K[ ] TagResource
- W[ ] S[x] M[ ] F[ ] K[ ] TestConnectionFunction
- W[ ] S[x] M[ ] F[ ] K[ ] TestFunction
- W[x] S[ ] M[x] F[ ] K[ ] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateAnycastIpList
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateCachePolicy
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateCloudFrontOriginAccessIdentity
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateConnectionFunction
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateConnectionGroup
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateContinuousDeploymentPolicy
- W[x] S[ ] M[x] F[ ] K[x] UpdateDistribution
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateDistributionTenant
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateDistributionWithStagingConfig
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateDomainAssociation
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateFieldLevelEncryptionConfig
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateFieldLevelEncryptionProfile
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateFunction
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateKeyGroup
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateKeyValueStore
- W[x] S[ ] M[x] F[ ] K[ ] UpdateOriginAccessControl
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateOriginRequestPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] UpdatePublicKey
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateRealtimeLogConfig
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateResponseHeadersPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateStreamingDistribution
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateTrustStore
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateVpcOrigin
- W[ ] S[x] M[ ] F[ ] K[ ] VerifyDnsConfiguration

Integration tests: 69/156 implemented operations tested (44.2%)
Untested implemented operations: 87

### winterbaume-cloudfrontkeyvaluestore (cloudfront-keyvaluestore) - W: 5/6, S: 0/6, M: 0/6, F: 0/6, K: 0/6

- W[x] S[ ] M[ ] F[ ] K[ ] DeleteKey
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeKeyValueStore
- W[x] S[ ] M[ ] F[ ] K[ ] GetKey
- W[x] S[ ] M[ ] F[ ] K[ ] ListKeys
- W[x] S[ ] M[ ] F[ ] K[ ] PutKey
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateKeys

Integration tests: 5/5 implemented operations tested (100.0%)

### winterbaume-cloudhsmv2 (cloudhsm-v2) - W: 18/18, S: 0/18, M: 0/18, F: 0/18, K: 0/18

- W[x] S[ ] M[ ] F[ ] K[ ] CopyBackupToRegion
- W[x] S[ ] M[ ] F[ ] K[ ] CreateCluster
- W[x] S[ ] M[ ] F[ ] K[ ] CreateHsm
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteBackup
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteCluster
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteHsm
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteResourcePolicy
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeBackups
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeClusters
- W[x] S[ ] M[ ] F[ ] K[ ] GetResourcePolicy
- W[x] S[ ] M[ ] F[ ] K[ ] InitializeCluster
- W[x] S[ ] M[ ] F[ ] K[ ] ListTags
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyBackupAttributes
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyCluster
- W[x] S[ ] M[ ] F[ ] K[ ] PutResourcePolicy
- W[x] S[ ] M[ ] F[ ] K[ ] RestoreBackup
- W[x] S[ ] M[ ] F[ ] K[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UntagResource

Integration tests: 18/18 implemented operations tested (100.0%)

### winterbaume-cloudsearchdomain (cloudsearch-domain) - W: 2/3, S: 0/3, M: 0/3, F: 0/3, K: 0/3

- W[x] S[ ] M[ ] F[ ] K[ ] Search
- W[x] S[ ] M[ ] F[ ] K[ ] Suggest
- W[ ] S[ ] M[ ] F[ ] K[ ] UploadDocuments

Integration tests: 2/2 implemented operations tested (100.0%)

### winterbaume-cloudtrail (cloudtrail) - W: 21/60, S: 2/60, M: 16/60, F: 0/60, K: 8/60

- W[x] S[ ] M[x] F[ ] K[ ] AddTags
- W[ ] S[ ] M[ ] F[ ] K[ ] CancelQuery
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateChannel
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateDashboard
- W[x] S[ ] M[ ] F[ ] K[ ] CreateEventDataStore
- W[x] S[ ] M[x] F[ ] K[x] CreateTrail
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteChannel
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteDashboard
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteEventDataStore
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteResourcePolicy
- W[x] S[ ] M[x] F[ ] K[x] DeleteTrail
- W[ ] S[ ] M[ ] F[ ] K[ ] DeregisterOrganizationDelegatedAdmin
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeQuery
- W[x] S[ ] M[x] F[ ] K[x] DescribeTrails
- W[ ] S[ ] M[ ] F[ ] K[ ] DisableFederation
- W[ ] S[ ] M[ ] F[ ] K[ ] EnableFederation
- W[ ] S[ ] M[ ] F[ ] K[ ] GenerateQuery
- W[ ] S[ ] M[ ] F[ ] K[ ] GetChannel
- W[ ] S[ ] M[ ] F[ ] K[ ] GetDashboard
- W[ ] S[ ] M[ ] F[ ] K[ ] GetEventConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] GetEventDataStore
- W[x] S[ ] M[x] F[ ] K[ ] GetEventSelectors
- W[ ] S[ ] M[ ] F[ ] K[ ] GetImport
- W[x] S[ ] M[x] F[ ] K[ ] GetInsightSelectors
- W[ ] S[ ] M[ ] F[ ] K[ ] GetQueryResults
- W[ ] S[ ] M[ ] F[ ] K[ ] GetResourcePolicy
- W[x] S[ ] M[x] F[ ] K[x] GetTrail
- W[x] S[ ] M[x] F[ ] K[x] GetTrailStatus
- W[ ] S[ ] M[ ] F[ ] K[ ] ListChannels
- W[ ] S[ ] M[ ] F[ ] K[ ] ListDashboards
- W[x] S[ ] M[ ] F[ ] K[ ] ListEventDataStores
- W[ ] S[ ] M[ ] F[ ] K[ ] ListImportFailures
- W[ ] S[ ] M[ ] F[ ] K[ ] ListImports
- W[ ] S[ ] M[ ] F[ ] K[ ] ListInsightsData
- W[ ] S[ ] M[ ] F[ ] K[ ] ListInsightsMetricData
- W[ ] S[x] M[ ] F[ ] K[ ] ListPublicKeys
- W[ ] S[ ] M[ ] F[ ] K[ ] ListQueries
- W[x] S[ ] M[x] F[ ] K[ ] ListTags
- W[x] S[ ] M[x] F[ ] K[ ] ListTrails
- W[ ] S[x] M[ ] F[ ] K[x] LookupEvents
- W[ ] S[ ] M[ ] F[ ] K[ ] PutEventConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] PutEventSelectors
- W[x] S[ ] M[x] F[ ] K[ ] PutInsightSelectors
- W[ ] S[ ] M[ ] F[ ] K[ ] PutResourcePolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] RegisterOrganizationDelegatedAdmin
- W[x] S[ ] M[x] F[ ] K[ ] RemoveTags
- W[ ] S[ ] M[ ] F[ ] K[ ] RestoreEventDataStore
- W[ ] S[ ] M[ ] F[ ] K[ ] SearchSampleQueries
- W[ ] S[ ] M[ ] F[ ] K[ ] StartDashboardRefresh
- W[ ] S[ ] M[ ] F[ ] K[ ] StartEventDataStoreIngestion
- W[ ] S[ ] M[ ] F[ ] K[ ] StartImport
- W[x] S[ ] M[x] F[ ] K[x] StartLogging
- W[ ] S[ ] M[ ] F[ ] K[ ] StartQuery
- W[ ] S[ ] M[ ] F[ ] K[ ] StopEventDataStoreIngestion
- W[ ] S[ ] M[ ] F[ ] K[ ] StopImport
- W[x] S[ ] M[x] F[ ] K[x] StopLogging
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateChannel
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateDashboard
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateEventDataStore
- W[x] S[ ] M[x] F[ ] K[ ] UpdateTrail

Integration tests: 16/21 implemented operations tested (76.2%)
Untested implemented operations: 5

### winterbaume-cloudtraildata (cloudtrail-data) - W: 1/1, S: 0/1, M: 0/1, F: 0/1, K: 0/1

- W[x] S[ ] M[ ] F[ ] K[ ] PutAuditEvents

Integration tests: 1/1 implemented operations tested (100.0%)

### winterbaume-cloudwatch (cloudwatch) - W: 38/46, S: 5/46, M: 20/46, F: 0/46, K: 10/46

Terraform E2E: 12 tests across 3 terraform resource types

Resource types: aws_cloudwatch_dashboard, aws_cloudwatch_metric_alarm, aws_cloudwatch_metric_stream

- W[x] S[ ] M[ ] F[ ] K[ ] DeleteAlarmMuteRule
- W[x] S[ ] M[x] F[ ] K[x] DeleteAlarms
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteAnomalyDetector
- W[x] S[ ] M[x] F[ ] K[ ] DeleteDashboards
- W[x] S[ ] M[x] F[ ] K[ ] DeleteInsightRules
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteMetricStream
- W[ ] S[x] M[ ] F[ ] K[ ] DescribeAlarmContributors
- W[ ] S[x] M[ ] F[ ] K[ ] DescribeAlarmHistory
- W[x] S[ ] M[x] F[ ] K[x] DescribeAlarms
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeAlarmsForMetric
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeAnomalyDetectors
- W[x] S[ ] M[x] F[ ] K[ ] DescribeInsightRules
- W[x] S[ ] M[ ] F[ ] K[ ] DisableAlarmActions
- W[x] S[ ] M[x] F[ ] K[ ] DisableInsightRules
- W[x] S[ ] M[ ] F[ ] K[ ] EnableAlarmActions
- W[x] S[ ] M[x] F[ ] K[ ] EnableInsightRules
- W[x] S[ ] M[ ] F[ ] K[ ] GetAlarmMuteRule
- W[x] S[ ] M[x] F[ ] K[ ] GetDashboard
- W[ ] S[x] M[ ] F[ ] K[ ] GetInsightRuleReport
- W[x] S[ ] M[x] F[ ] K[x] GetMetricData
- W[ ] S[x] M[x] F[ ] K[x] GetMetricStatistics
- W[x] S[ ] M[ ] F[ ] K[ ] GetMetricStream
- W[ ] S[x] M[ ] F[ ] K[ ] GetMetricWidgetImage
- W[ ] S[ ] M[ ] F[ ] K[ ] GetOTelEnrichment
- W[x] S[ ] M[ ] F[ ] K[ ] ListAlarmMuteRules
- W[x] S[ ] M[x] F[ ] K[ ] ListDashboards
- W[x] S[ ] M[ ] F[ ] K[ ] ListManagedInsightRules
- W[x] S[ ] M[ ] F[ ] K[ ] ListMetricStreams
- W[x] S[ ] M[x] F[ ] K[x] ListMetrics
- W[x] S[ ] M[x] F[ ] K[x] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] PutAlarmMuteRule
- W[x] S[ ] M[ ] F[ ] K[ ] PutAnomalyDetector
- W[x] S[ ] M[ ] F[ ] K[ ] PutCompositeAlarm
- W[x] S[ ] M[x] F[ ] K[ ] PutDashboard
- W[x] S[ ] M[x] F[ ] K[ ] PutInsightRule
- W[x] S[ ] M[ ] F[ ] K[ ] PutManagedInsightRules
- W[x] S[ ] M[x] F[ ] K[x] PutMetricAlarm
- W[x] S[ ] M[x] F[ ] K[x] PutMetricData
- W[x] S[ ] M[ ] F[ ] K[ ] PutMetricStream
- W[x] S[ ] M[x] F[ ] K[ ] SetAlarmState
- W[x] S[ ] M[ ] F[ ] K[ ] StartMetricStreams
- W[ ] S[ ] M[ ] F[ ] K[ ] StartOTelEnrichment
- W[x] S[ ] M[ ] F[ ] K[ ] StopMetricStreams
- W[ ] S[ ] M[ ] F[ ] K[ ] StopOTelEnrichment
- W[x] S[ ] M[x] F[ ] K[x] TagResource
- W[x] S[ ] M[x] F[ ] K[x] UntagResource

Integration tests: 29/38 implemented operations tested (76.3%)
Untested implemented operations: 9

### winterbaume-cloudwatchlogs (cloudwatch-logs) - W: 93/113, S: 15/113, M: 51/113, F: 0/113, K: 11/113

Terraform E2E: 5 tests across 3 terraform resource types

Resource types: aws_cloudwatch_log_group, aws_cloudwatch_log_metric_filter, aws_cloudwatch_log_resource_policy

- W[x] S[ ] M[ ] F[ ] K[ ] AssociateKmsKey
- W[ ] S[x] M[ ] F[ ] K[ ] AssociateSourceToS3TableIntegration
- W[x] S[ ] M[x] F[ ] K[ ] CancelExportTask
- W[x] S[ ] M[ ] F[ ] K[ ] CancelImportTask
- W[x] S[ ] M[x] F[ ] K[ ] CreateDelivery
- W[x] S[ ] M[x] F[ ] K[ ] CreateExportTask
- W[x] S[ ] M[ ] F[ ] K[ ] CreateImportTask
- W[x] S[ ] M[ ] F[ ] K[ ] CreateLogAnomalyDetector
- W[x] S[ ] M[x] F[ ] K[x] CreateLogGroup
- W[x] S[ ] M[x] F[ ] K[x] CreateLogStream
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateLookupTable
- W[x] S[ ] M[ ] F[ ] K[ ] CreateScheduledQuery
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteAccountPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteDataProtectionPolicy
- W[x] S[ ] M[x] F[ ] K[ ] DeleteDelivery
- W[x] S[ ] M[x] F[ ] K[ ] DeleteDeliveryDestination
- W[x] S[ ] M[x] F[ ] K[ ] DeleteDeliveryDestinationPolicy
- W[x] S[ ] M[x] F[ ] K[ ] DeleteDeliverySource
- W[x] S[ ] M[x] F[ ] K[ ] DeleteDestination
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteIndexPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteIntegration
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteLogAnomalyDetector
- W[x] S[ ] M[x] F[ ] K[x] DeleteLogGroup
- W[x] S[ ] M[x] F[ ] K[x] DeleteLogStream
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteLookupTable
- W[x] S[ ] M[x] F[ ] K[ ] DeleteMetricFilter
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteQueryDefinition
- W[x] S[ ] M[x] F[ ] K[ ] DeleteResourcePolicy
- W[x] S[ ] M[x] F[ ] K[x] DeleteRetentionPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteScheduledQuery
- W[x] S[ ] M[x] F[ ] K[ ] DeleteSubscriptionFilter
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteTransformer
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeAccountPolicies
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeConfigurationTemplates
- W[x] S[ ] M[x] F[ ] K[ ] DescribeDeliveries
- W[x] S[ ] M[x] F[ ] K[ ] DescribeDeliveryDestinations
- W[x] S[ ] M[x] F[ ] K[ ] DescribeDeliverySources
- W[x] S[ ] M[x] F[ ] K[ ] DescribeDestinations
- W[x] S[ ] M[x] F[ ] K[ ] DescribeExportTasks
- W[ ] S[x] M[ ] F[ ] K[ ] DescribeFieldIndexes
- W[ ] S[x] M[ ] F[ ] K[ ] DescribeImportTaskBatches
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeImportTasks
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeIndexPolicies
- W[x] S[ ] M[x] F[ ] K[x] DescribeLogGroups
- W[x] S[ ] M[x] F[ ] K[x] DescribeLogStreams
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeLookupTables
- W[x] S[ ] M[x] F[ ] K[ ] DescribeMetricFilters
- W[x] S[ ] M[x] F[ ] K[ ] DescribeQueries
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeQueryDefinitions
- W[x] S[ ] M[x] F[ ] K[ ] DescribeResourcePolicies
- W[x] S[ ] M[x] F[ ] K[ ] DescribeSubscriptionFilters
- W[x] S[ ] M[ ] F[ ] K[ ] DisassociateKmsKey
- W[ ] S[x] M[ ] F[ ] K[ ] DisassociateSourceFromS3TableIntegration
- W[x] S[ ] M[x] F[ ] K[x] FilterLogEvents
- W[x] S[ ] M[ ] F[ ] K[ ] GetDataProtectionPolicy
- W[x] S[ ] M[x] F[ ] K[ ] GetDelivery
- W[x] S[ ] M[x] F[ ] K[ ] GetDeliveryDestination
- W[x] S[ ] M[x] F[ ] K[ ] GetDeliveryDestinationPolicy
- W[x] S[ ] M[x] F[ ] K[ ] GetDeliverySource
- W[x] S[ ] M[ ] F[ ] K[ ] GetIntegration
- W[x] S[ ] M[ ] F[ ] K[ ] GetLogAnomalyDetector
- W[x] S[ ] M[x] F[ ] K[x] GetLogEvents
- W[ ] S[x] M[ ] F[ ] K[ ] GetLogFields
- W[ ] S[x] M[ ] F[ ] K[ ] GetLogGroupFields
- W[ ] S[x] M[ ] F[ ] K[ ] GetLogObject
- W[ ] S[x] M[ ] F[ ] K[ ] GetLogRecord
- W[ ] S[ ] M[ ] F[ ] K[ ] GetLookupTable
- W[x] S[ ] M[x] F[ ] K[ ] GetQueryResults
- W[x] S[ ] M[ ] F[ ] K[ ] GetScheduledQuery
- W[ ] S[x] M[ ] F[ ] K[ ] GetScheduledQueryHistory
- W[x] S[ ] M[ ] F[ ] K[ ] GetTransformer
- W[ ] S[x] M[ ] F[ ] K[ ] ListAggregateLogGroupSummaries
- W[x] S[ ] M[ ] F[ ] K[ ] ListAnomalies
- W[x] S[ ] M[ ] F[ ] K[ ] ListIntegrations
- W[x] S[ ] M[ ] F[ ] K[ ] ListLogAnomalyDetectors
- W[x] S[ ] M[ ] F[ ] K[ ] ListLogGroups
- W[ ] S[x] M[ ] F[ ] K[ ] ListLogGroupsForQuery
- W[x] S[ ] M[ ] F[ ] K[ ] ListScheduledQueries
- W[ ] S[x] M[ ] F[ ] K[ ] ListSourcesForS3TableIntegration
- W[x] S[ ] M[x] F[ ] K[ ] ListTagsForResource
- W[x] S[ ] M[x] F[ ] K[ ] ListTagsLogGroup
- W[x] S[ ] M[ ] F[ ] K[ ] PutAccountPolicy
- W[ ] S[x] M[ ] F[ ] K[ ] PutBearerTokenAuthentication
- W[x] S[ ] M[ ] F[ ] K[ ] PutDataProtectionPolicy
- W[x] S[ ] M[x] F[ ] K[ ] PutDeliveryDestination
- W[x] S[ ] M[x] F[ ] K[ ] PutDeliveryDestinationPolicy
- W[x] S[ ] M[x] F[ ] K[ ] PutDeliverySource
- W[x] S[ ] M[x] F[ ] K[ ] PutDestination
- W[x] S[ ] M[x] F[ ] K[ ] PutDestinationPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] PutIndexPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] PutIntegration
- W[x] S[ ] M[x] F[ ] K[x] PutLogEvents
- W[x] S[ ] M[ ] F[ ] K[ ] PutLogGroupDeletionProtection
- W[x] S[ ] M[x] F[ ] K[ ] PutMetricFilter
- W[x] S[ ] M[ ] F[ ] K[ ] PutQueryDefinition
- W[x] S[ ] M[x] F[ ] K[ ] PutResourcePolicy
- W[x] S[ ] M[x] F[ ] K[x] PutRetentionPolicy
- W[x] S[ ] M[x] F[ ] K[ ] PutSubscriptionFilter
- W[x] S[ ] M[ ] F[ ] K[ ] PutTransformer
- W[ ] S[x] M[ ] F[ ] K[ ] StartLiveTail
- W[x] S[ ] M[x] F[ ] K[ ] StartQuery
- W[x] S[ ] M[ ] F[ ] K[ ] StopQuery
- W[x] S[ ] M[x] F[ ] K[ ] TagLogGroup
- W[x] S[ ] M[x] F[ ] K[ ] TagResource
- W[ ] S[x] M[ ] F[ ] K[ ] TestMetricFilter
- W[x] S[ ] M[ ] F[ ] K[ ] TestTransformer
- W[x] S[ ] M[x] F[ ] K[ ] UntagLogGroup
- W[x] S[ ] M[x] F[ ] K[ ] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateAnomaly
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateDeliveryConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateLogAnomalyDetector
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateLookupTable
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateScheduledQuery

Integration tests: 84/93 implemented operations tested (90.3%)
Untested implemented operations: 9

### winterbaume-codeartifact (codeartifact) - W: 9/48, S: 0/48, M: 0/48, F: 0/48, K: 0/48

- W[ ] S[ ] M[ ] F[ ] K[ ] AssociateExternalConnection
- W[ ] S[ ] M[ ] F[ ] K[ ] CopyPackageVersions
- W[x] S[ ] M[ ] F[ ] K[ ] CreateDomain
- W[ ] S[ ] M[ ] F[ ] K[ ] CreatePackageGroup
- W[x] S[ ] M[ ] F[ ] K[ ] CreateRepository
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteDomain
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteDomainPermissionsPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] DeletePackage
- W[ ] S[ ] M[ ] F[ ] K[ ] DeletePackageGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] DeletePackageVersions
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteRepository
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteRepositoryPermissionsPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeDomain
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribePackage
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribePackageGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribePackageVersion
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeRepository
- W[ ] S[ ] M[ ] F[ ] K[ ] DisassociateExternalConnection
- W[ ] S[ ] M[ ] F[ ] K[ ] DisposePackageVersions
- W[ ] S[ ] M[ ] F[ ] K[ ] GetAssociatedPackageGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] GetAuthorizationToken
- W[ ] S[ ] M[ ] F[ ] K[ ] GetDomainPermissionsPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] GetPackageVersionAsset
- W[ ] S[ ] M[ ] F[ ] K[ ] GetPackageVersionReadme
- W[ ] S[ ] M[ ] F[ ] K[ ] GetRepositoryEndpoint
- W[ ] S[ ] M[ ] F[ ] K[ ] GetRepositoryPermissionsPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] ListAllowedRepositoriesForGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] ListAssociatedPackages
- W[x] S[ ] M[ ] F[ ] K[ ] ListDomains
- W[ ] S[ ] M[ ] F[ ] K[ ] ListPackageGroups
- W[ ] S[ ] M[ ] F[ ] K[ ] ListPackageVersionAssets
- W[ ] S[ ] M[ ] F[ ] K[ ] ListPackageVersionDependencies
- W[ ] S[ ] M[ ] F[ ] K[ ] ListPackageVersions
- W[ ] S[ ] M[ ] F[ ] K[ ] ListPackages
- W[x] S[ ] M[ ] F[ ] K[ ] ListRepositories
- W[x] S[ ] M[ ] F[ ] K[ ] ListRepositoriesInDomain
- W[ ] S[ ] M[ ] F[ ] K[ ] ListSubPackageGroups
- W[ ] S[ ] M[ ] F[ ] K[ ] ListTagsForResource
- W[ ] S[ ] M[ ] F[ ] K[ ] PublishPackageVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] PutDomainPermissionsPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] PutPackageOriginConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] PutRepositoryPermissionsPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] TagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] UntagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdatePackageGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdatePackageGroupOriginConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdatePackageVersionsStatus
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateRepository

Integration tests: 9/9 implemented operations tested (100.0%)

### winterbaume-codebuild (codebuild) - W: 29/59, S: 0/59, M: 9/59, F: 0/59, K: 0/59

- W[x] S[ ] M[ ] F[ ] K[ ] BatchDeleteBuilds
- W[ ] S[ ] M[ ] F[ ] K[ ] BatchGetBuildBatches
- W[x] S[ ] M[x] F[ ] K[ ] BatchGetBuilds
- W[ ] S[ ] M[ ] F[ ] K[ ] BatchGetCommandExecutions
- W[ ] S[ ] M[ ] F[ ] K[ ] BatchGetFleets
- W[x] S[ ] M[x] F[ ] K[ ] BatchGetProjects
- W[x] S[ ] M[ ] F[ ] K[ ] BatchGetReportGroups
- W[ ] S[ ] M[ ] F[ ] K[ ] BatchGetReports
- W[ ] S[ ] M[ ] F[ ] K[ ] BatchGetSandboxes
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateFleet
- W[x] S[ ] M[x] F[ ] K[ ] CreateProject
- W[x] S[ ] M[ ] F[ ] K[ ] CreateReportGroup
- W[x] S[ ] M[ ] F[ ] K[ ] CreateWebhook
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteBuildBatch
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteFleet
- W[x] S[ ] M[x] F[ ] K[ ] DeleteProject
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteReport
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteReportGroup
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteResourcePolicy
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteSourceCredentials
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteWebhook
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeCodeCoverages
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeTestCases
- W[ ] S[ ] M[ ] F[ ] K[ ] GetReportGroupTrend
- W[x] S[ ] M[ ] F[ ] K[ ] GetResourcePolicy
- W[x] S[ ] M[ ] F[ ] K[ ] ImportSourceCredentials
- W[x] S[ ] M[ ] F[ ] K[ ] InvalidateProjectCache
- W[ ] S[ ] M[ ] F[ ] K[ ] ListBuildBatches
- W[ ] S[ ] M[ ] F[ ] K[ ] ListBuildBatchesForProject
- W[x] S[ ] M[x] F[ ] K[ ] ListBuilds
- W[x] S[ ] M[x] F[ ] K[ ] ListBuildsForProject
- W[ ] S[ ] M[ ] F[ ] K[ ] ListCommandExecutionsForSandbox
- W[ ] S[ ] M[ ] F[ ] K[ ] ListCuratedEnvironmentImages
- W[ ] S[ ] M[ ] F[ ] K[ ] ListFleets
- W[x] S[ ] M[x] F[ ] K[ ] ListProjects
- W[x] S[ ] M[ ] F[ ] K[ ] ListReportGroups
- W[ ] S[ ] M[ ] F[ ] K[ ] ListReports
- W[x] S[ ] M[ ] F[ ] K[ ] ListReportsForReportGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] ListSandboxes
- W[ ] S[ ] M[ ] F[ ] K[ ] ListSandboxesForProject
- W[ ] S[ ] M[ ] F[ ] K[ ] ListSharedProjects
- W[ ] S[ ] M[ ] F[ ] K[ ] ListSharedReportGroups
- W[x] S[ ] M[ ] F[ ] K[ ] ListSourceCredentials
- W[x] S[ ] M[ ] F[ ] K[ ] PutResourcePolicy
- W[x] S[ ] M[ ] F[ ] K[ ] RetryBuild
- W[ ] S[ ] M[ ] F[ ] K[ ] RetryBuildBatch
- W[x] S[ ] M[x] F[ ] K[ ] StartBuild
- W[ ] S[ ] M[ ] F[ ] K[ ] StartBuildBatch
- W[ ] S[ ] M[ ] F[ ] K[ ] StartCommandExecution
- W[ ] S[ ] M[ ] F[ ] K[ ] StartSandbox
- W[ ] S[ ] M[ ] F[ ] K[ ] StartSandboxConnection
- W[x] S[ ] M[x] F[ ] K[ ] StopBuild
- W[ ] S[ ] M[ ] F[ ] K[ ] StopBuildBatch
- W[ ] S[ ] M[ ] F[ ] K[ ] StopSandbox
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateFleet
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateProject
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateProjectVisibility
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateReportGroup
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateWebhook

Integration tests: 26/29 implemented operations tested (89.7%)
Untested implemented operations: 3

### winterbaume-codecommit (codecommit) - W: 25/79, S: 0/79, M: 3/79, F: 0/79, K: 0/79

Terraform E2E: 4 tests across 1 terraform resource types

Resource types: aws_codecommit_repository

- W[ ] S[ ] M[ ] F[ ] K[ ] AssociateApprovalRuleTemplateWithRepository
- W[ ] S[ ] M[ ] F[ ] K[ ] BatchAssociateApprovalRuleTemplateWithRepositories
- W[ ] S[ ] M[ ] F[ ] K[ ] BatchDescribeMergeConflicts
- W[ ] S[ ] M[ ] F[ ] K[ ] BatchDisassociateApprovalRuleTemplateFromRepositories
- W[ ] S[ ] M[ ] F[ ] K[ ] BatchGetCommits
- W[ ] S[ ] M[ ] F[ ] K[ ] BatchGetRepositories
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateApprovalRuleTemplate
- W[x] S[ ] M[ ] F[ ] K[ ] CreateBranch
- W[x] S[ ] M[ ] F[ ] K[ ] CreateCommit
- W[x] S[ ] M[ ] F[ ] K[ ] CreatePullRequest
- W[ ] S[ ] M[ ] F[ ] K[ ] CreatePullRequestApprovalRule
- W[x] S[ ] M[x] F[ ] K[ ] CreateRepository
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateUnreferencedMergeCommit
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteApprovalRuleTemplate
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteBranch
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteCommentContent
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteFile
- W[ ] S[ ] M[ ] F[ ] K[ ] DeletePullRequestApprovalRule
- W[x] S[ ] M[x] F[ ] K[ ] DeleteRepository
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeMergeConflicts
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribePullRequestEvents
- W[ ] S[ ] M[ ] F[ ] K[ ] DisassociateApprovalRuleTemplateFromRepository
- W[ ] S[ ] M[ ] F[ ] K[ ] EvaluatePullRequestApprovalRules
- W[ ] S[ ] M[ ] F[ ] K[ ] GetApprovalRuleTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] GetBlob
- W[x] S[ ] M[ ] F[ ] K[ ] GetBranch
- W[ ] S[ ] M[ ] F[ ] K[ ] GetComment
- W[ ] S[ ] M[ ] F[ ] K[ ] GetCommentReactions
- W[ ] S[ ] M[ ] F[ ] K[ ] GetCommentsForComparedCommit
- W[ ] S[ ] M[ ] F[ ] K[ ] GetCommentsForPullRequest
- W[x] S[ ] M[ ] F[ ] K[ ] GetCommit
- W[x] S[ ] M[ ] F[ ] K[ ] GetDifferences
- W[x] S[ ] M[ ] F[ ] K[ ] GetFile
- W[x] S[ ] M[ ] F[ ] K[ ] GetFolder
- W[ ] S[ ] M[ ] F[ ] K[ ] GetMergeCommit
- W[ ] S[ ] M[ ] F[ ] K[ ] GetMergeConflicts
- W[ ] S[ ] M[ ] F[ ] K[ ] GetMergeOptions
- W[x] S[ ] M[ ] F[ ] K[ ] GetPullRequest
- W[ ] S[ ] M[ ] F[ ] K[ ] GetPullRequestApprovalStates
- W[ ] S[ ] M[ ] F[ ] K[ ] GetPullRequestOverrideState
- W[x] S[ ] M[x] F[ ] K[ ] GetRepository
- W[ ] S[ ] M[ ] F[ ] K[ ] GetRepositoryTriggers
- W[ ] S[ ] M[ ] F[ ] K[ ] ListApprovalRuleTemplates
- W[ ] S[ ] M[ ] F[ ] K[ ] ListAssociatedApprovalRuleTemplatesForRepository
- W[x] S[ ] M[ ] F[ ] K[ ] ListBranches
- W[ ] S[ ] M[ ] F[ ] K[ ] ListFileCommitHistory
- W[x] S[ ] M[ ] F[ ] K[ ] ListPullRequests
- W[x] S[ ] M[ ] F[ ] K[ ] ListRepositories
- W[ ] S[ ] M[ ] F[ ] K[ ] ListRepositoriesForApprovalRuleTemplate
- W[x] S[ ] M[ ] F[ ] K[ ] ListTagsForResource
- W[ ] S[ ] M[ ] F[ ] K[ ] MergeBranchesByFastForward
- W[ ] S[ ] M[ ] F[ ] K[ ] MergeBranchesBySquash
- W[ ] S[ ] M[ ] F[ ] K[ ] MergeBranchesByThreeWay
- W[ ] S[ ] M[ ] F[ ] K[ ] MergePullRequestByFastForward
- W[ ] S[ ] M[ ] F[ ] K[ ] MergePullRequestBySquash
- W[ ] S[ ] M[ ] F[ ] K[ ] MergePullRequestByThreeWay
- W[ ] S[ ] M[ ] F[ ] K[ ] OverridePullRequestApprovalRules
- W[ ] S[ ] M[ ] F[ ] K[ ] PostCommentForComparedCommit
- W[ ] S[ ] M[ ] F[ ] K[ ] PostCommentForPullRequest
- W[ ] S[ ] M[ ] F[ ] K[ ] PostCommentReply
- W[ ] S[ ] M[ ] F[ ] K[ ] PutCommentReaction
- W[x] S[ ] M[ ] F[ ] K[ ] PutFile
- W[ ] S[ ] M[ ] F[ ] K[ ] PutRepositoryTriggers
- W[x] S[ ] M[ ] F[ ] K[ ] TagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] TestRepositoryTriggers
- W[x] S[ ] M[ ] F[ ] K[ ] UntagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateApprovalRuleTemplateContent
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateApprovalRuleTemplateDescription
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateApprovalRuleTemplateName
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateComment
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateDefaultBranch
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdatePullRequestApprovalRuleContent
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdatePullRequestApprovalState
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdatePullRequestDescription
- W[x] S[ ] M[ ] F[ ] K[ ] UpdatePullRequestStatus
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdatePullRequestTitle
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateRepositoryDescription
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateRepositoryEncryptionKey
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateRepositoryName

Integration tests: 25/25 implemented operations tested (100.0%)

### winterbaume-codedeploy (codedeploy) - W: 15/47, S: 0/47, M: 14/47, F: 0/47, K: 0/47

- W[ ] S[ ] M[ ] F[ ] K[ ] AddTagsToOnPremisesInstances
- W[ ] S[ ] M[ ] F[ ] K[ ] BatchGetApplicationRevisions
- W[x] S[ ] M[x] F[ ] K[ ] BatchGetApplications
- W[ ] S[ ] M[ ] F[ ] K[ ] BatchGetDeploymentGroups
- W[ ] S[ ] M[ ] F[ ] K[ ] BatchGetDeploymentInstances
- W[ ] S[ ] M[ ] F[ ] K[ ] BatchGetDeploymentTargets
- W[x] S[ ] M[x] F[ ] K[ ] BatchGetDeployments
- W[ ] S[ ] M[ ] F[ ] K[ ] BatchGetOnPremisesInstances
- W[ ] S[ ] M[ ] F[ ] K[ ] ContinueDeployment
- W[x] S[ ] M[x] F[ ] K[ ] CreateApplication
- W[x] S[ ] M[x] F[ ] K[ ] CreateDeployment
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateDeploymentConfig
- W[x] S[ ] M[x] F[ ] K[ ] CreateDeploymentGroup
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteApplication
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteDeploymentConfig
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteDeploymentGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteGitHubAccountToken
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteResourcesByExternalId
- W[ ] S[ ] M[ ] F[ ] K[ ] DeregisterOnPremisesInstance
- W[x] S[ ] M[x] F[ ] K[ ] GetApplication
- W[ ] S[ ] M[ ] F[ ] K[ ] GetApplicationRevision
- W[x] S[ ] M[x] F[ ] K[ ] GetDeployment
- W[ ] S[ ] M[ ] F[ ] K[ ] GetDeploymentConfig
- W[x] S[ ] M[x] F[ ] K[ ] GetDeploymentGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] GetDeploymentInstance
- W[ ] S[ ] M[ ] F[ ] K[ ] GetDeploymentTarget
- W[ ] S[ ] M[ ] F[ ] K[ ] GetOnPremisesInstance
- W[ ] S[ ] M[ ] F[ ] K[ ] ListApplicationRevisions
- W[x] S[ ] M[x] F[ ] K[ ] ListApplications
- W[ ] S[ ] M[ ] F[ ] K[ ] ListDeploymentConfigs
- W[x] S[ ] M[x] F[ ] K[ ] ListDeploymentGroups
- W[ ] S[ ] M[ ] F[ ] K[ ] ListDeploymentInstances
- W[ ] S[ ] M[ ] F[ ] K[ ] ListDeploymentTargets
- W[x] S[ ] M[x] F[ ] K[ ] ListDeployments
- W[ ] S[ ] M[ ] F[ ] K[ ] ListGitHubAccountTokenNames
- W[ ] S[ ] M[ ] F[ ] K[ ] ListOnPremisesInstances
- W[x] S[ ] M[x] F[ ] K[ ] ListTagsForResource
- W[ ] S[ ] M[ ] F[ ] K[ ] PutLifecycleEventHookExecutionStatus
- W[ ] S[ ] M[ ] F[ ] K[ ] RegisterApplicationRevision
- W[ ] S[ ] M[ ] F[ ] K[ ] RegisterOnPremisesInstance
- W[ ] S[ ] M[ ] F[ ] K[ ] RemoveTagsFromOnPremisesInstances
- W[ ] S[ ] M[ ] F[ ] K[ ] SkipWaitTimeForInstanceTermination
- W[ ] S[ ] M[ ] F[ ] K[ ] StopDeployment
- W[x] S[ ] M[x] F[ ] K[ ] TagResource
- W[x] S[ ] M[x] F[ ] K[ ] UntagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateApplication
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateDeploymentGroup

Integration tests: 15/15 implemented operations tested (100.0%)

### winterbaume-codegurureviewer (codeguru-reviewer) - W: 9/14, S: 0/14, M: 0/14, F: 0/14, K: 11/14

- W[x] S[ ] M[ ] F[ ] K[x] AssociateRepository
- W[x] S[ ] M[ ] F[ ] K[x] CreateCodeReview
- W[x] S[ ] M[ ] F[ ] K[x] DescribeCodeReview
- W[ ] S[ ] M[ ] F[ ] K[x] DescribeRecommendationFeedback
- W[x] S[ ] M[ ] F[ ] K[x] DescribeRepositoryAssociation
- W[x] S[ ] M[ ] F[ ] K[x] DisassociateRepository
- W[x] S[ ] M[ ] F[ ] K[x] ListCodeReviews
- W[ ] S[ ] M[ ] F[ ] K[x] ListRecommendationFeedback
- W[x] S[ ] M[ ] F[ ] K[x] ListRecommendations
- W[ ] S[ ] M[ ] F[ ] K[x] ListRepositoryAssociations
- W[ ] S[ ] M[ ] F[ ] K[ ] ListTagsForResource
- W[ ] S[ ] M[ ] F[ ] K[x] PutRecommendationFeedback
- W[x] S[ ] M[ ] F[ ] K[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UntagResource

Integration tests: 9/9 implemented operations tested (100.0%)

### winterbaume-codegurusecurity (codeguru-security) - W: 11/13, S: 0/13, M: 0/13, F: 0/13, K: 0/13

- W[x] S[ ] M[ ] F[ ] K[ ] BatchGetFindings
- W[x] S[ ] M[ ] F[ ] K[ ] CreateScan
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateUploadUrl
- W[x] S[ ] M[ ] F[ ] K[ ] GetAccountConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] GetFindings
- W[x] S[ ] M[ ] F[ ] K[ ] GetMetricsSummary
- W[x] S[ ] M[ ] F[ ] K[ ] GetScan
- W[x] S[ ] M[ ] F[ ] K[ ] ListFindingsMetrics
- W[x] S[ ] M[ ] F[ ] K[ ] ListScans
- W[ ] S[ ] M[ ] F[ ] K[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateAccountConfiguration

Integration tests: 9/11 implemented operations tested (81.8%)
Untested implemented operations: 2

### winterbaume-codepipeline (codepipeline) - W: 44/44, S: 0/44, M: 8/44, F: 0/44, K: 0/44

- W[x] S[ ] M[ ] F[ ] K[ ] AcknowledgeJob
- W[x] S[ ] M[ ] F[ ] K[ ] AcknowledgeThirdPartyJob
- W[x] S[ ] M[ ] F[ ] K[ ] CreateCustomActionType
- W[x] S[ ] M[x] F[ ] K[ ] CreatePipeline
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteCustomActionType
- W[x] S[ ] M[x] F[ ] K[ ] DeletePipeline
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteWebhook
- W[x] S[ ] M[ ] F[ ] K[ ] DeregisterWebhookWithThirdParty
- W[x] S[ ] M[ ] F[ ] K[ ] DisableStageTransition
- W[x] S[ ] M[ ] F[ ] K[ ] EnableStageTransition
- W[x] S[ ] M[ ] F[ ] K[ ] GetActionType
- W[x] S[ ] M[ ] F[ ] K[ ] GetJobDetails
- W[x] S[ ] M[x] F[ ] K[ ] GetPipeline
- W[x] S[ ] M[ ] F[ ] K[ ] GetPipelineExecution
- W[x] S[ ] M[ ] F[ ] K[ ] GetPipelineState
- W[x] S[ ] M[ ] F[ ] K[ ] GetThirdPartyJobDetails
- W[x] S[ ] M[ ] F[ ] K[ ] ListActionExecutions
- W[x] S[ ] M[ ] F[ ] K[ ] ListActionTypes
- W[x] S[ ] M[ ] F[ ] K[ ] ListDeployActionExecutionTargets
- W[x] S[ ] M[ ] F[ ] K[ ] ListPipelineExecutions
- W[x] S[ ] M[x] F[ ] K[ ] ListPipelines
- W[x] S[ ] M[ ] F[ ] K[ ] ListRuleExecutions
- W[x] S[ ] M[ ] F[ ] K[ ] ListRuleTypes
- W[x] S[ ] M[x] F[ ] K[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] ListWebhooks
- W[x] S[ ] M[ ] F[ ] K[ ] OverrideStageCondition
- W[x] S[ ] M[ ] F[ ] K[ ] PollForJobs
- W[x] S[ ] M[ ] F[ ] K[ ] PollForThirdPartyJobs
- W[x] S[ ] M[ ] F[ ] K[ ] PutActionRevision
- W[x] S[ ] M[ ] F[ ] K[ ] PutApprovalResult
- W[x] S[ ] M[ ] F[ ] K[ ] PutJobFailureResult
- W[x] S[ ] M[ ] F[ ] K[ ] PutJobSuccessResult
- W[x] S[ ] M[ ] F[ ] K[ ] PutThirdPartyJobFailureResult
- W[x] S[ ] M[ ] F[ ] K[ ] PutThirdPartyJobSuccessResult
- W[x] S[ ] M[ ] F[ ] K[ ] PutWebhook
- W[x] S[ ] M[ ] F[ ] K[ ] RegisterWebhookWithThirdParty
- W[x] S[ ] M[ ] F[ ] K[ ] RetryStageExecution
- W[x] S[ ] M[ ] F[ ] K[ ] RollbackStage
- W[x] S[ ] M[ ] F[ ] K[ ] StartPipelineExecution
- W[x] S[ ] M[ ] F[ ] K[ ] StopPipelineExecution
- W[x] S[ ] M[x] F[ ] K[ ] TagResource
- W[x] S[ ] M[x] F[ ] K[ ] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateActionType
- W[x] S[ ] M[x] F[ ] K[ ] UpdatePipeline

Integration tests: 8/44 implemented operations tested (18.2%)
Untested implemented operations: 36

### winterbaume-codestarnotifications (codestar-notifications) - W: 7/13, S: 0/13, M: 0/13, F: 0/13, K: 0/13

- W[ ] S[ ] M[ ] F[ ] K[ ] CreateNotificationRule
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteNotificationRule
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteTarget
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeNotificationRule
- W[x] S[ ] M[ ] F[ ] K[ ] ListEventTypes
- W[ ] S[ ] M[ ] F[ ] K[ ] ListNotificationRules
- W[ ] S[ ] M[ ] F[ ] K[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] ListTargets
- W[x] S[ ] M[ ] F[ ] K[ ] Subscribe
- W[x] S[ ] M[ ] F[ ] K[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] Unsubscribe
- W[x] S[ ] M[ ] F[ ] K[ ] UntagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateNotificationRule

Integration tests: 5/7 implemented operations tested (71.4%)
Untested implemented operations: 2

### winterbaume-cognitoidentity (cognito-identity) - W: 20/23, S: 3/23, M: 10/23, F: 0/23, K: 0/23

- W[x] S[ ] M[x] F[ ] K[ ] CreateIdentityPool
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteIdentities
- W[x] S[ ] M[x] F[ ] K[ ] DeleteIdentityPool
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeIdentity
- W[x] S[ ] M[x] F[ ] K[ ] DescribeIdentityPool
- W[ ] S[x] M[x] F[ ] K[ ] GetCredentialsForIdentity
- W[x] S[ ] M[x] F[ ] K[ ] GetId
- W[x] S[ ] M[ ] F[ ] K[ ] GetIdentityPoolRoles
- W[ ] S[x] M[x] F[ ] K[ ] GetOpenIdToken
- W[ ] S[x] M[x] F[ ] K[ ] GetOpenIdTokenForDeveloperIdentity
- W[x] S[ ] M[ ] F[ ] K[ ] GetPrincipalTagAttributeMap
- W[x] S[ ] M[x] F[ ] K[ ] ListIdentities
- W[x] S[ ] M[x] F[ ] K[ ] ListIdentityPools
- W[x] S[ ] M[ ] F[ ] K[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] LookupDeveloperIdentity
- W[x] S[ ] M[ ] F[ ] K[ ] MergeDeveloperIdentities
- W[x] S[ ] M[ ] F[ ] K[ ] SetIdentityPoolRoles
- W[x] S[ ] M[ ] F[ ] K[ ] SetPrincipalTagAttributeMap
- W[x] S[ ] M[ ] F[ ] K[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UnlinkDeveloperIdentity
- W[x] S[ ] M[ ] F[ ] K[ ] UnlinkIdentity
- W[x] S[ ] M[ ] F[ ] K[ ] UntagResource
- W[x] S[ ] M[x] F[ ] K[ ] UpdateIdentityPool

Integration tests: 18/20 implemented operations tested (90.0%)
Untested implemented operations: 2

### winterbaume-cognitoidentityprovider (cognito-identity-provider) - W: 104/122, S: 18/122, M: 62/122, F: 39/122, K: 17/122

Terraform E2E: 15 tests across 7 terraform resource types

Resource types: aws_cognito_identity_provider, aws_cognito_resource_server, aws_cognito_user, aws_cognito_user_group, aws_cognito_user_pool, aws_cognito_user_pool_client, aws_cognito_user_pool_domain

- W[x] S[ ] M[x] F[ ] K[ ] AddCustomAttributes
- W[x] S[ ] M[ ] F[ ] K[ ] AddUserPoolClientSecret
- W[x] S[ ] M[x] F[x] K[ ] AdminAddUserToGroup
- W[x] S[ ] M[x] F[ ] K[ ] AdminConfirmSignUp
- W[x] S[ ] M[x] F[ ] K[x] AdminCreateUser
- W[x] S[ ] M[x] F[x] K[x] AdminDeleteUser
- W[x] S[ ] M[x] F[ ] K[ ] AdminDeleteUserAttributes
- W[x] S[ ] M[ ] F[ ] K[ ] AdminDisableProviderForUser
- W[x] S[ ] M[x] F[ ] K[ ] AdminDisableUser
- W[x] S[ ] M[x] F[ ] K[ ] AdminEnableUser
- W[x] S[ ] M[ ] F[ ] K[ ] AdminForgetDevice
- W[x] S[ ] M[ ] F[ ] K[ ] AdminGetDevice
- W[x] S[ ] M[x] F[x] K[x] AdminGetUser
- W[x] S[ ] M[x] F[x] K[ ] AdminInitiateAuth
- W[x] S[ ] M[ ] F[ ] K[ ] AdminLinkProviderForUser
- W[x] S[ ] M[ ] F[ ] K[ ] AdminListDevices
- W[x] S[ ] M[x] F[x] K[ ] AdminListGroupsForUser
- W[ ] S[x] M[ ] F[ ] K[ ] AdminListUserAuthEvents
- W[x] S[ ] M[x] F[x] K[ ] AdminRemoveUserFromGroup
- W[x] S[ ] M[x] F[ ] K[ ] AdminResetUserPassword
- W[x] S[ ] M[x] F[ ] K[ ] AdminRespondToAuthChallenge
- W[x] S[ ] M[x] F[ ] K[ ] AdminSetUserMFAPreference
- W[x] S[ ] M[x] F[x] K[ ] AdminSetUserPassword
- W[x] S[ ] M[ ] F[ ] K[ ] AdminSetUserSettings
- W[ ] S[x] M[ ] F[ ] K[ ] AdminUpdateAuthEventFeedback
- W[x] S[ ] M[ ] F[ ] K[ ] AdminUpdateDeviceStatus
- W[x] S[ ] M[x] F[x] K[ ] AdminUpdateUserAttributes
- W[x] S[ ] M[x] F[ ] K[ ] AdminUserGlobalSignOut
- W[ ] S[x] M[x] F[ ] K[ ] AssociateSoftwareToken
- W[ ] S[x] M[x] F[x] K[ ] ChangePassword
- W[ ] S[x] M[ ] F[ ] K[ ] CompleteWebAuthnRegistration
- W[x] S[ ] M[ ] F[ ] K[ ] ConfirmDevice
- W[x] S[ ] M[x] F[x] K[ ] ConfirmForgotPassword
- W[x] S[ ] M[x] F[x] K[x] ConfirmSignUp
- W[x] S[ ] M[x] F[x] K[ ] CreateGroup
- W[x] S[ ] M[x] F[ ] K[ ] CreateIdentityProvider
- W[x] S[ ] M[ ] F[ ] K[ ] CreateManagedLoginBranding
- W[x] S[ ] M[x] F[x] K[ ] CreateResourceServer
- W[x] S[ ] M[ ] F[ ] K[ ] CreateTerms
- W[x] S[ ] M[ ] F[ ] K[ ] CreateUserImportJob
- W[x] S[ ] M[x] F[x] K[x] CreateUserPool
- W[x] S[ ] M[x] F[x] K[x] CreateUserPoolClient
- W[x] S[ ] M[x] F[ ] K[ ] CreateUserPoolDomain
- W[x] S[ ] M[x] F[x] K[ ] DeleteGroup
- W[x] S[ ] M[x] F[ ] K[ ] DeleteIdentityProvider
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteManagedLoginBranding
- W[x] S[ ] M[ ] F[x] K[ ] DeleteResourceServer
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteTerms
- W[ ] S[x] M[ ] F[ ] K[ ] DeleteUser
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteUserAttributes
- W[x] S[ ] M[x] F[x] K[x] DeleteUserPool
- W[x] S[ ] M[x] F[x] K[x] DeleteUserPoolClient
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteUserPoolClientSecret
- W[x] S[ ] M[x] F[ ] K[ ] DeleteUserPoolDomain
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteWebAuthnCredential
- W[x] S[ ] M[x] F[ ] K[ ] DescribeIdentityProvider
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeManagedLoginBranding
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeManagedLoginBrandingByClient
- W[x] S[ ] M[x] F[x] K[ ] DescribeResourceServer
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeRiskConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeTerms
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeUserImportJob
- W[x] S[ ] M[x] F[x] K[x] DescribeUserPool
- W[x] S[ ] M[x] F[x] K[x] DescribeUserPoolClient
- W[x] S[ ] M[x] F[ ] K[ ] DescribeUserPoolDomain
- W[x] S[ ] M[ ] F[ ] K[ ] ForgetDevice
- W[x] S[ ] M[x] F[x] K[ ] ForgotPassword
- W[x] S[ ] M[ ] F[ ] K[ ] GetCSVHeader
- W[x] S[ ] M[ ] F[ ] K[ ] GetDevice
- W[x] S[ ] M[x] F[x] K[ ] GetGroup
- W[x] S[ ] M[ ] F[ ] K[ ] GetIdentityProviderByIdentifier
- W[x] S[ ] M[ ] F[ ] K[ ] GetLogDeliveryConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] GetSigningCertificate
- W[ ] S[x] M[ ] F[ ] K[ ] GetTokensFromRefreshToken
- W[x] S[ ] M[ ] F[ ] K[ ] GetUICustomization
- W[ ] S[x] M[x] F[x] K[ ] GetUser
- W[ ] S[x] M[ ] F[ ] K[ ] GetUserAttributeVerificationCode
- W[ ] S[x] M[ ] F[ ] K[ ] GetUserAuthFactors
- W[x] S[ ] M[x] F[ ] K[x] GetUserPoolMfaConfig
- W[x] S[ ] M[x] F[ ] K[ ] GlobalSignOut
- W[x] S[ ] M[x] F[x] K[x] InitiateAuth
- W[x] S[ ] M[ ] F[ ] K[ ] ListDevices
- W[x] S[ ] M[x] F[x] K[ ] ListGroups
- W[x] S[ ] M[x] F[ ] K[ ] ListIdentityProviders
- W[x] S[ ] M[x] F[x] K[ ] ListResourceServers
- W[x] S[ ] M[ ] F[x] K[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] ListTerms
- W[x] S[ ] M[ ] F[ ] K[ ] ListUserImportJobs
- W[x] S[ ] M[ ] F[ ] K[ ] ListUserPoolClientSecrets
- W[x] S[ ] M[x] F[x] K[x] ListUserPoolClients
- W[x] S[ ] M[x] F[x] K[x] ListUserPools
- W[x] S[ ] M[x] F[x] K[x] ListUsers
- W[x] S[ ] M[x] F[x] K[ ] ListUsersInGroup
- W[x] S[ ] M[ ] F[ ] K[ ] ListWebAuthnCredentials
- W[x] S[ ] M[ ] F[ ] K[ ] ResendConfirmationCode
- W[x] S[ ] M[x] F[ ] K[ ] RespondToAuthChallenge
- W[ ] S[x] M[ ] F[ ] K[ ] RevokeToken
- W[x] S[ ] M[ ] F[ ] K[ ] SetLogDeliveryConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] SetRiskConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] SetUICustomization
- W[ ] S[x] M[x] F[ ] K[ ] SetUserMFAPreference
- W[x] S[ ] M[x] F[ ] K[x] SetUserPoolMfaConfig
- W[ ] S[x] M[ ] F[ ] K[ ] SetUserSettings
- W[x] S[ ] M[x] F[x] K[x] SignUp
- W[x] S[ ] M[ ] F[ ] K[ ] StartUserImportJob
- W[ ] S[x] M[ ] F[ ] K[ ] StartWebAuthnRegistration
- W[x] S[ ] M[ ] F[ ] K[ ] StopUserImportJob
- W[x] S[ ] M[ ] F[x] K[ ] TagResource
- W[x] S[ ] M[ ] F[x] K[ ] UntagResource
- W[ ] S[x] M[ ] F[ ] K[ ] UpdateAuthEventFeedback
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateDeviceStatus
- W[x] S[ ] M[x] F[x] K[ ] UpdateGroup
- W[x] S[ ] M[x] F[ ] K[ ] UpdateIdentityProvider
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateManagedLoginBranding
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateResourceServer
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateTerms
- W[ ] S[x] M[x] F[x] K[ ] UpdateUserAttributes
- W[x] S[ ] M[x] F[x] K[ ] UpdateUserPool
- W[x] S[ ] M[x] F[ ] K[ ] UpdateUserPoolClient
- W[x] S[ ] M[x] F[ ] K[ ] UpdateUserPoolDomain
- W[ ] S[x] M[x] F[ ] K[ ] VerifySoftwareToken
- W[ ] S[x] M[ ] F[ ] K[ ] VerifyUserAttribute

Integration tests: 76/104 implemented operations tested (73.1%)
Untested implemented operations: 28

### winterbaume-cognitosync (cognito-sync) - W: 11/17, S: 0/17, M: 0/17, F: 0/17, K: 0/17

- W[x] S[ ] M[ ] F[ ] K[ ] BulkPublish
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteDataset
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeDataset
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeIdentityPoolUsage
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeIdentityUsage
- W[x] S[ ] M[ ] F[ ] K[ ] GetBulkPublishDetails
- W[x] S[ ] M[ ] F[ ] K[ ] GetCognitoEvents
- W[ ] S[ ] M[ ] F[ ] K[ ] GetIdentityPoolConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] ListDatasets
- W[ ] S[ ] M[ ] F[ ] K[ ] ListIdentityPoolUsage
- W[x] S[ ] M[ ] F[ ] K[ ] ListRecords
- W[x] S[ ] M[ ] F[ ] K[ ] RegisterDevice
- W[x] S[ ] M[ ] F[ ] K[ ] SetCognitoEvents
- W[ ] S[ ] M[ ] F[ ] K[ ] SetIdentityPoolConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] SubscribeToDataset
- W[ ] S[ ] M[ ] F[ ] K[ ] UnsubscribeFromDataset
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateRecords

Integration tests: 6/11 implemented operations tested (54.5%)
Untested implemented operations: 5

### winterbaume-comprehend (comprehend) - W: 60/85, S: 5/85, M: 63/85, F: 0/85, K: 12/85

- W[ ] S[ ] M[ ] F[ ] K[x] BatchDetectDominantLanguage
- W[ ] S[ ] M[ ] F[ ] K[x] BatchDetectEntities
- W[ ] S[ ] M[ ] F[ ] K[x] BatchDetectKeyPhrases
- W[ ] S[ ] M[ ] F[ ] K[x] BatchDetectSentiment
- W[ ] S[ ] M[ ] F[ ] K[x] BatchDetectSyntax
- W[ ] S[ ] M[ ] F[ ] K[ ] BatchDetectTargetedSentiment
- W[ ] S[ ] M[ ] F[ ] K[ ] ClassifyDocument
- W[ ] S[ ] M[ ] F[ ] K[x] ContainsPiiEntities
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateDataset
- W[x] S[ ] M[x] F[ ] K[ ] CreateDocumentClassifier
- W[x] S[ ] M[x] F[ ] K[ ] CreateEndpoint
- W[x] S[ ] M[x] F[ ] K[ ] CreateEntityRecognizer
- W[x] S[ ] M[x] F[ ] K[ ] CreateFlywheel
- W[x] S[ ] M[x] F[ ] K[ ] DeleteDocumentClassifier
- W[x] S[ ] M[x] F[ ] K[ ] DeleteEndpoint
- W[x] S[ ] M[x] F[ ] K[ ] DeleteEntityRecognizer
- W[x] S[ ] M[x] F[ ] K[ ] DeleteFlywheel
- W[x] S[ ] M[x] F[ ] K[ ] DeleteResourcePolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeDataset
- W[x] S[ ] M[x] F[ ] K[ ] DescribeDocumentClassificationJob
- W[x] S[ ] M[x] F[ ] K[ ] DescribeDocumentClassifier
- W[x] S[ ] M[x] F[ ] K[ ] DescribeDominantLanguageDetectionJob
- W[x] S[ ] M[x] F[ ] K[ ] DescribeEndpoint
- W[x] S[ ] M[x] F[ ] K[ ] DescribeEntitiesDetectionJob
- W[x] S[ ] M[x] F[ ] K[ ] DescribeEntityRecognizer
- W[x] S[ ] M[x] F[ ] K[ ] DescribeEventsDetectionJob
- W[x] S[ ] M[x] F[ ] K[ ] DescribeFlywheel
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeFlywheelIteration
- W[x] S[ ] M[x] F[ ] K[ ] DescribeKeyPhrasesDetectionJob
- W[x] S[ ] M[x] F[ ] K[ ] DescribePiiEntitiesDetectionJob
- W[x] S[ ] M[x] F[ ] K[ ] DescribeResourcePolicy
- W[x] S[ ] M[x] F[ ] K[ ] DescribeSentimentDetectionJob
- W[x] S[ ] M[x] F[ ] K[ ] DescribeTargetedSentimentDetectionJob
- W[x] S[ ] M[x] F[ ] K[ ] DescribeTopicsDetectionJob
- W[ ] S[x] M[ ] F[ ] K[x] DetectDominantLanguage
- W[ ] S[x] M[ ] F[ ] K[x] DetectEntities
- W[ ] S[x] M[x] F[ ] K[x] DetectKeyPhrases
- W[ ] S[x] M[x] F[ ] K[x] DetectPiiEntities
- W[ ] S[x] M[x] F[ ] K[x] DetectSentiment
- W[ ] S[ ] M[ ] F[ ] K[x] DetectSyntax
- W[ ] S[ ] M[ ] F[ ] K[ ] DetectTargetedSentiment
- W[ ] S[ ] M[ ] F[ ] K[ ] DetectToxicContent
- W[ ] S[ ] M[ ] F[ ] K[ ] ImportModel
- W[ ] S[ ] M[ ] F[ ] K[ ] ListDatasets
- W[x] S[ ] M[x] F[ ] K[ ] ListDocumentClassificationJobs
- W[ ] S[ ] M[ ] F[ ] K[ ] ListDocumentClassifierSummaries
- W[x] S[ ] M[x] F[ ] K[ ] ListDocumentClassifiers
- W[x] S[ ] M[x] F[ ] K[ ] ListDominantLanguageDetectionJobs
- W[x] S[ ] M[x] F[ ] K[ ] ListEndpoints
- W[x] S[ ] M[x] F[ ] K[ ] ListEntitiesDetectionJobs
- W[ ] S[ ] M[ ] F[ ] K[ ] ListEntityRecognizerSummaries
- W[x] S[ ] M[x] F[ ] K[ ] ListEntityRecognizers
- W[x] S[ ] M[x] F[ ] K[ ] ListEventsDetectionJobs
- W[ ] S[ ] M[ ] F[ ] K[ ] ListFlywheelIterationHistory
- W[x] S[ ] M[x] F[ ] K[ ] ListFlywheels
- W[x] S[ ] M[x] F[ ] K[ ] ListKeyPhrasesDetectionJobs
- W[x] S[ ] M[x] F[ ] K[ ] ListPiiEntitiesDetectionJobs
- W[x] S[ ] M[x] F[ ] K[ ] ListSentimentDetectionJobs
- W[x] S[ ] M[x] F[ ] K[ ] ListTagsForResource
- W[x] S[ ] M[x] F[ ] K[ ] ListTargetedSentimentDetectionJobs
- W[x] S[ ] M[x] F[ ] K[ ] ListTopicsDetectionJobs
- W[x] S[ ] M[x] F[ ] K[ ] PutResourcePolicy
- W[x] S[ ] M[x] F[ ] K[ ] StartDocumentClassificationJob
- W[x] S[ ] M[x] F[ ] K[ ] StartDominantLanguageDetectionJob
- W[x] S[ ] M[x] F[ ] K[ ] StartEntitiesDetectionJob
- W[x] S[ ] M[x] F[ ] K[ ] StartEventsDetectionJob
- W[x] S[ ] M[x] F[ ] K[ ] StartFlywheelIteration
- W[x] S[ ] M[x] F[ ] K[ ] StartKeyPhrasesDetectionJob
- W[x] S[ ] M[x] F[ ] K[ ] StartPiiEntitiesDetectionJob
- W[x] S[ ] M[x] F[ ] K[ ] StartSentimentDetectionJob
- W[x] S[ ] M[x] F[ ] K[ ] StartTargetedSentimentDetectionJob
- W[x] S[ ] M[x] F[ ] K[ ] StartTopicsDetectionJob
- W[x] S[ ] M[x] F[ ] K[ ] StopDominantLanguageDetectionJob
- W[x] S[ ] M[x] F[ ] K[ ] StopEntitiesDetectionJob
- W[x] S[ ] M[x] F[ ] K[ ] StopEventsDetectionJob
- W[x] S[ ] M[x] F[ ] K[ ] StopKeyPhrasesDetectionJob
- W[x] S[ ] M[x] F[ ] K[ ] StopPiiEntitiesDetectionJob
- W[x] S[ ] M[x] F[ ] K[ ] StopSentimentDetectionJob
- W[x] S[ ] M[x] F[ ] K[ ] StopTargetedSentimentDetectionJob
- W[x] S[ ] M[x] F[ ] K[ ] StopTrainingDocumentClassifier
- W[x] S[ ] M[x] F[ ] K[ ] StopTrainingEntityRecognizer
- W[x] S[ ] M[x] F[ ] K[ ] TagResource
- W[x] S[ ] M[x] F[ ] K[ ] UntagResource
- W[x] S[ ] M[x] F[ ] K[ ] UpdateEndpoint
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateFlywheel

Integration tests: 60/60 implemented operations tested (100.0%)

### winterbaume-config (config-service) - W: 46/97, S: 3/97, M: 38/97, F: 0/97, K: 9/97

Terraform E2E: 7 tests across 7 terraform resource types

Resource types: aws_config_aggregate_authorization, aws_config_config_rule, aws_config_configuration_aggregator, aws_config_configuration_recorder, aws_config_configuration_recorder_status, aws_config_delivery_channel, aws_config_retention_configuration

- W[ ] S[ ] M[ ] F[ ] K[ ] AssociateResourceTypes
- W[x] S[ ] M[x] F[ ] K[ ] BatchGetAggregateResourceConfig
- W[x] S[ ] M[x] F[ ] K[ ] BatchGetResourceConfig
- W[x] S[ ] M[x] F[ ] K[ ] DeleteAggregationAuthorization
- W[x] S[ ] M[x] F[ ] K[x] DeleteConfigRule
- W[x] S[ ] M[x] F[ ] K[ ] DeleteConfigurationAggregator
- W[x] S[ ] M[x] F[ ] K[x] DeleteConfigurationRecorder
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteConformancePack
- W[x] S[ ] M[x] F[ ] K[ ] DeleteDeliveryChannel
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteEvaluationResults
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteOrganizationConfigRule
- W[x] S[ ] M[x] F[ ] K[ ] DeleteOrganizationConformancePack
- W[ ] S[ ] M[ ] F[ ] K[ ] DeletePendingAggregationRequest
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteRemediationConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteRemediationExceptions
- W[x] S[ ] M[x] F[ ] K[ ] DeleteResourceConfig
- W[x] S[ ] M[x] F[ ] K[ ] DeleteRetentionConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteServiceLinkedConfigurationRecorder
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteStoredQuery
- W[ ] S[ ] M[ ] F[ ] K[ ] DeliverConfigSnapshot
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeAggregateComplianceByConfigRules
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeAggregateComplianceByConformancePacks
- W[x] S[ ] M[x] F[ ] K[ ] DescribeAggregationAuthorizations
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeComplianceByConfigRule
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeComplianceByResource
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeConfigRuleEvaluationStatus
- W[x] S[ ] M[x] F[ ] K[x] DescribeConfigRules
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeConfigurationAggregatorSourcesStatus
- W[x] S[ ] M[x] F[ ] K[ ] DescribeConfigurationAggregators
- W[x] S[ ] M[x] F[ ] K[ ] DescribeConfigurationRecorderStatus
- W[x] S[ ] M[x] F[ ] K[x] DescribeConfigurationRecorders
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeConformancePackCompliance
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeConformancePackStatus
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeConformancePacks
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeDeliveryChannelStatus
- W[x] S[ ] M[x] F[ ] K[ ] DescribeDeliveryChannels
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeOrganizationConfigRuleStatuses
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeOrganizationConfigRules
- W[x] S[ ] M[x] F[ ] K[ ] DescribeOrganizationConformancePackStatuses
- W[x] S[ ] M[x] F[ ] K[ ] DescribeOrganizationConformancePacks
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribePendingAggregationRequests
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeRemediationConfigurations
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeRemediationExceptions
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeRemediationExecutionStatus
- W[x] S[ ] M[x] F[ ] K[ ] DescribeRetentionConfigurations
- W[ ] S[ ] M[ ] F[ ] K[ ] DisassociateResourceTypes
- W[ ] S[ ] M[ ] F[ ] K[ ] GetAggregateComplianceDetailsByConfigRule
- W[ ] S[ ] M[ ] F[ ] K[ ] GetAggregateConfigRuleComplianceSummary
- W[ ] S[ ] M[ ] F[ ] K[ ] GetAggregateConformancePackComplianceSummary
- W[ ] S[ ] M[ ] F[ ] K[ ] GetAggregateDiscoveredResourceCounts
- W[ ] S[ ] M[ ] F[ ] K[ ] GetAggregateResourceConfig
- W[x] S[ ] M[ ] F[ ] K[x] GetComplianceDetailsByConfigRule
- W[ ] S[ ] M[ ] F[ ] K[ ] GetComplianceDetailsByResource
- W[ ] S[ ] M[ ] F[ ] K[ ] GetComplianceSummaryByConfigRule
- W[ ] S[ ] M[ ] F[ ] K[ ] GetComplianceSummaryByResourceType
- W[ ] S[ ] M[ ] F[ ] K[ ] GetConformancePackComplianceDetails
- W[ ] S[ ] M[ ] F[ ] K[ ] GetConformancePackComplianceSummary
- W[ ] S[ ] M[ ] F[ ] K[ ] GetCustomRulePolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] GetDiscoveredResourceCounts
- W[ ] S[ ] M[ ] F[ ] K[ ] GetOrganizationConfigRuleDetailedStatus
- W[x] S[ ] M[x] F[ ] K[ ] GetOrganizationConformancePackDetailedStatus
- W[ ] S[ ] M[ ] F[ ] K[ ] GetOrganizationCustomRulePolicy
- W[x] S[ ] M[x] F[ ] K[ ] GetResourceConfigHistory
- W[ ] S[ ] M[ ] F[ ] K[ ] GetResourceEvaluationSummary
- W[ ] S[ ] M[ ] F[ ] K[ ] GetStoredQuery
- W[x] S[ ] M[x] F[ ] K[ ] ListAggregateDiscoveredResources
- W[ ] S[ ] M[ ] F[ ] K[ ] ListConfigurationRecorders
- W[ ] S[ ] M[ ] F[ ] K[ ] ListConformancePackComplianceScores
- W[x] S[ ] M[x] F[ ] K[ ] ListDiscoveredResources
- W[ ] S[ ] M[ ] F[ ] K[ ] ListResourceEvaluations
- W[ ] S[ ] M[ ] F[ ] K[ ] ListStoredQueries
- W[x] S[ ] M[x] F[ ] K[ ] ListTagsForResource
- W[x] S[ ] M[x] F[ ] K[ ] PutAggregationAuthorization
- W[x] S[ ] M[x] F[ ] K[x] PutConfigRule
- W[x] S[ ] M[x] F[ ] K[ ] PutConfigurationAggregator
- W[x] S[ ] M[x] F[ ] K[x] PutConfigurationRecorder
- W[ ] S[ ] M[ ] F[ ] K[ ] PutConformancePack
- W[x] S[ ] M[x] F[ ] K[ ] PutDeliveryChannel
- W[x] S[ ] M[x] F[ ] K[ ] PutEvaluations
- W[ ] S[ ] M[ ] F[ ] K[ ] PutExternalEvaluation
- W[x] S[ ] M[ ] F[ ] K[ ] PutOrganizationConfigRule
- W[x] S[ ] M[x] F[ ] K[ ] PutOrganizationConformancePack
- W[x] S[ ] M[ ] F[ ] K[ ] PutRemediationConfigurations
- W[ ] S[ ] M[ ] F[ ] K[ ] PutRemediationExceptions
- W[x] S[ ] M[x] F[ ] K[ ] PutResourceConfig
- W[x] S[ ] M[x] F[ ] K[ ] PutRetentionConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] PutServiceLinkedConfigurationRecorder
- W[ ] S[ ] M[ ] F[ ] K[ ] PutStoredQuery
- W[ ] S[ ] M[ ] F[ ] K[ ] SelectAggregateResourceConfig
- W[ ] S[x] M[x] F[ ] K[ ] SelectResourceConfig
- W[ ] S[x] M[ ] F[ ] K[ ] StartConfigRulesEvaluation
- W[x] S[ ] M[x] F[ ] K[x] StartConfigurationRecorder
- W[ ] S[x] M[ ] F[ ] K[ ] StartRemediationExecution
- W[ ] S[ ] M[ ] F[ ] K[ ] StartResourceEvaluation
- W[x] S[ ] M[x] F[ ] K[x] StopConfigurationRecorder
- W[x] S[ ] M[x] F[ ] K[ ] TagResource
- W[x] S[ ] M[x] F[ ] K[ ] UntagResource

Integration tests: 46/46 implemented operations tested (100.0%)

### winterbaume-connect (connect) - W: 10/370, S: 0/370, M: 10/370, F: 0/370, K: 0/370

- W[ ] S[ ] M[ ] F[ ] K[ ] ActivateEvaluationForm
- W[x] S[ ] M[x] F[ ] K[ ] AssociateAnalyticsDataSet
- W[ ] S[ ] M[ ] F[ ] K[ ] AssociateApprovedOrigin
- W[ ] S[ ] M[ ] F[ ] K[ ] AssociateBot
- W[ ] S[ ] M[ ] F[ ] K[ ] AssociateContactWithUser
- W[ ] S[ ] M[ ] F[ ] K[ ] AssociateDefaultVocabulary
- W[ ] S[ ] M[ ] F[ ] K[ ] AssociateEmailAddressAlias
- W[ ] S[ ] M[ ] F[ ] K[ ] AssociateFlow
- W[ ] S[ ] M[ ] F[ ] K[ ] AssociateHoursOfOperations
- W[ ] S[ ] M[ ] F[ ] K[ ] AssociateInstanceStorageConfig
- W[ ] S[ ] M[ ] F[ ] K[ ] AssociateLambdaFunction
- W[ ] S[ ] M[ ] F[ ] K[ ] AssociateLexBot
- W[ ] S[ ] M[ ] F[ ] K[ ] AssociatePhoneNumberContactFlow
- W[ ] S[ ] M[ ] F[ ] K[ ] AssociateQueueEmailAddresses
- W[ ] S[ ] M[ ] F[ ] K[ ] AssociateQueueQuickConnects
- W[ ] S[ ] M[ ] F[ ] K[ ] AssociateRoutingProfileQueues
- W[ ] S[ ] M[ ] F[ ] K[ ] AssociateSecurityKey
- W[ ] S[ ] M[ ] F[ ] K[ ] AssociateSecurityProfiles
- W[ ] S[ ] M[ ] F[ ] K[ ] AssociateTrafficDistributionGroupUser
- W[ ] S[ ] M[ ] F[ ] K[ ] AssociateUserProficiencies
- W[ ] S[ ] M[ ] F[ ] K[ ] AssociateWorkspace
- W[ ] S[ ] M[ ] F[ ] K[ ] BatchAssociateAnalyticsDataSet
- W[ ] S[ ] M[ ] F[ ] K[ ] BatchCreateDataTableValue
- W[ ] S[ ] M[ ] F[ ] K[ ] BatchDeleteDataTableValue
- W[ ] S[ ] M[ ] F[ ] K[ ] BatchDescribeDataTableValue
- W[ ] S[ ] M[ ] F[ ] K[ ] BatchDisassociateAnalyticsDataSet
- W[ ] S[ ] M[ ] F[ ] K[ ] BatchGetAttachedFileMetadata
- W[ ] S[ ] M[ ] F[ ] K[ ] BatchGetFlowAssociation
- W[ ] S[ ] M[ ] F[ ] K[ ] BatchPutContact
- W[ ] S[ ] M[ ] F[ ] K[ ] BatchUpdateDataTableValue
- W[ ] S[ ] M[ ] F[ ] K[ ] ClaimPhoneNumber
- W[ ] S[ ] M[ ] F[ ] K[ ] CompleteAttachedFileUpload
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateAgentStatus
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateContact
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateContactFlow
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateContactFlowModule
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateContactFlowModuleAlias
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateContactFlowModuleVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateContactFlowVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateDataTable
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateDataTableAttribute
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateEmailAddress
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateEvaluationForm
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateHoursOfOperation
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateHoursOfOperationOverride
- W[x] S[ ] M[x] F[ ] K[ ] CreateInstance
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateIntegrationAssociation
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateNotification
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateParticipant
- W[ ] S[ ] M[ ] F[ ] K[ ] CreatePersistentContactAssociation
- W[ ] S[ ] M[ ] F[ ] K[ ] CreatePredefinedAttribute
- W[ ] S[ ] M[ ] F[ ] K[ ] CreatePrompt
- W[ ] S[ ] M[ ] F[ ] K[ ] CreatePushNotificationRegistration
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateQueue
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateQuickConnect
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateRoutingProfile
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateRule
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateSecurityProfile
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateTaskTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateTestCase
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateTrafficDistributionGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateUseCase
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateUser
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateUserHierarchyGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateView
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateViewVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateVocabulary
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateWorkspace
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateWorkspacePage
- W[ ] S[ ] M[ ] F[ ] K[ ] DeactivateEvaluationForm
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteAttachedFile
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteContactEvaluation
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteContactFlow
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteContactFlowModule
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteContactFlowModuleAlias
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteContactFlowModuleVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteContactFlowVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteDataTable
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteDataTableAttribute
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteEmailAddress
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteEvaluationForm
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteHoursOfOperation
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteHoursOfOperationOverride
- W[x] S[ ] M[x] F[ ] K[ ] DeleteInstance
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteIntegrationAssociation
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteNotification
- W[ ] S[ ] M[ ] F[ ] K[ ] DeletePredefinedAttribute
- W[ ] S[ ] M[ ] F[ ] K[ ] DeletePrompt
- W[ ] S[ ] M[ ] F[ ] K[ ] DeletePushNotificationRegistration
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteQueue
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteQuickConnect
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteRoutingProfile
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteRule
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteSecurityProfile
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteTaskTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteTestCase
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteTrafficDistributionGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteUseCase
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteUser
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteUserHierarchyGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteView
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteViewVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteVocabulary
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteWorkspace
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteWorkspaceMedia
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteWorkspacePage
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeAgentStatus
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeAttachedFilesConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeAuthenticationProfile
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeContact
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeContactEvaluation
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeContactFlow
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeContactFlowModule
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeContactFlowModuleAlias
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeDataTable
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeDataTableAttribute
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeEmailAddress
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeEvaluationForm
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeHoursOfOperation
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeHoursOfOperationOverride
- W[x] S[ ] M[x] F[ ] K[ ] DescribeInstance
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeInstanceAttribute
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeInstanceStorageConfig
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeNotification
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribePhoneNumber
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribePredefinedAttribute
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribePrompt
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeQueue
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeQuickConnect
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeRoutingProfile
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeRule
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeSecurityProfile
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeTestCase
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeTrafficDistributionGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeUser
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeUserHierarchyGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeUserHierarchyStructure
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeView
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeVocabulary
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeWorkspace
- W[x] S[ ] M[x] F[ ] K[ ] DisassociateAnalyticsDataSet
- W[ ] S[ ] M[ ] F[ ] K[ ] DisassociateApprovedOrigin
- W[ ] S[ ] M[ ] F[ ] K[ ] DisassociateBot
- W[ ] S[ ] M[ ] F[ ] K[ ] DisassociateEmailAddressAlias
- W[ ] S[ ] M[ ] F[ ] K[ ] DisassociateFlow
- W[ ] S[ ] M[ ] F[ ] K[ ] DisassociateHoursOfOperations
- W[ ] S[ ] M[ ] F[ ] K[ ] DisassociateInstanceStorageConfig
- W[ ] S[ ] M[ ] F[ ] K[ ] DisassociateLambdaFunction
- W[ ] S[ ] M[ ] F[ ] K[ ] DisassociateLexBot
- W[ ] S[ ] M[ ] F[ ] K[ ] DisassociatePhoneNumberContactFlow
- W[ ] S[ ] M[ ] F[ ] K[ ] DisassociateQueueEmailAddresses
- W[ ] S[ ] M[ ] F[ ] K[ ] DisassociateQueueQuickConnects
- W[ ] S[ ] M[ ] F[ ] K[ ] DisassociateRoutingProfileQueues
- W[ ] S[ ] M[ ] F[ ] K[ ] DisassociateSecurityKey
- W[ ] S[ ] M[ ] F[ ] K[ ] DisassociateSecurityProfiles
- W[ ] S[ ] M[ ] F[ ] K[ ] DisassociateTrafficDistributionGroupUser
- W[ ] S[ ] M[ ] F[ ] K[ ] DisassociateUserProficiencies
- W[ ] S[ ] M[ ] F[ ] K[ ] DisassociateWorkspace
- W[ ] S[ ] M[ ] F[ ] K[ ] DismissUserContact
- W[ ] S[ ] M[ ] F[ ] K[ ] EvaluateDataTableValues
- W[ ] S[ ] M[ ] F[ ] K[ ] GetAttachedFile
- W[ ] S[ ] M[ ] F[ ] K[ ] GetContactAttributes
- W[ ] S[ ] M[ ] F[ ] K[ ] GetContactMetrics
- W[ ] S[ ] M[ ] F[ ] K[ ] GetCurrentMetricData
- W[ ] S[ ] M[ ] F[ ] K[ ] GetCurrentUserData
- W[ ] S[ ] M[ ] F[ ] K[ ] GetEffectiveHoursOfOperations
- W[ ] S[ ] M[ ] F[ ] K[ ] GetFederationToken
- W[ ] S[ ] M[ ] F[ ] K[ ] GetFlowAssociation
- W[ ] S[ ] M[ ] F[ ] K[ ] GetMetricData
- W[ ] S[ ] M[ ] F[ ] K[ ] GetMetricDataV2
- W[ ] S[ ] M[ ] F[ ] K[ ] GetPromptFile
- W[ ] S[ ] M[ ] F[ ] K[ ] GetTaskTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] GetTestCaseExecutionSummary
- W[ ] S[ ] M[ ] F[ ] K[ ] GetTrafficDistribution
- W[ ] S[ ] M[ ] F[ ] K[ ] ImportPhoneNumber
- W[ ] S[ ] M[ ] F[ ] K[ ] ImportWorkspaceMedia
- W[ ] S[ ] M[ ] F[ ] K[ ] ListAgentStatuses
- W[x] S[ ] M[x] F[ ] K[ ] ListAnalyticsDataAssociations
- W[ ] S[ ] M[ ] F[ ] K[ ] ListAnalyticsDataLakeDataSets
- W[ ] S[ ] M[ ] F[ ] K[ ] ListApprovedOrigins
- W[ ] S[ ] M[ ] F[ ] K[ ] ListAssociatedContacts
- W[ ] S[ ] M[ ] F[ ] K[ ] ListAttachedFilesConfigurations
- W[ ] S[ ] M[ ] F[ ] K[ ] ListAuthenticationProfiles
- W[ ] S[ ] M[ ] F[ ] K[ ] ListBots
- W[ ] S[ ] M[ ] F[ ] K[ ] ListChildHoursOfOperations
- W[ ] S[ ] M[ ] F[ ] K[ ] ListContactEvaluations
- W[ ] S[ ] M[ ] F[ ] K[ ] ListContactFlowModuleAliases
- W[ ] S[ ] M[ ] F[ ] K[ ] ListContactFlowModuleVersions
- W[ ] S[ ] M[ ] F[ ] K[ ] ListContactFlowModules
- W[ ] S[ ] M[ ] F[ ] K[ ] ListContactFlowVersions
- W[ ] S[ ] M[ ] F[ ] K[ ] ListContactFlows
- W[ ] S[ ] M[ ] F[ ] K[ ] ListContactReferences
- W[ ] S[ ] M[ ] F[ ] K[ ] ListDataTableAttributes
- W[ ] S[ ] M[ ] F[ ] K[ ] ListDataTablePrimaryValues
- W[ ] S[ ] M[ ] F[ ] K[ ] ListDataTableValues
- W[ ] S[ ] M[ ] F[ ] K[ ] ListDataTables
- W[ ] S[ ] M[ ] F[ ] K[ ] ListDefaultVocabularies
- W[ ] S[ ] M[ ] F[ ] K[ ] ListEntitySecurityProfiles
- W[ ] S[ ] M[ ] F[ ] K[ ] ListEvaluationFormVersions
- W[ ] S[ ] M[ ] F[ ] K[ ] ListEvaluationForms
- W[ ] S[ ] M[ ] F[ ] K[ ] ListFlowAssociations
- W[ ] S[ ] M[ ] F[ ] K[ ] ListHoursOfOperationOverrides
- W[ ] S[ ] M[ ] F[ ] K[ ] ListHoursOfOperations
- W[ ] S[ ] M[ ] F[ ] K[ ] ListInstanceAttributes
- W[ ] S[ ] M[ ] F[ ] K[ ] ListInstanceStorageConfigs
- W[x] S[ ] M[x] F[ ] K[ ] ListInstances
- W[ ] S[ ] M[ ] F[ ] K[ ] ListIntegrationAssociations
- W[ ] S[ ] M[ ] F[ ] K[ ] ListLambdaFunctions
- W[ ] S[ ] M[ ] F[ ] K[ ] ListLexBots
- W[ ] S[ ] M[ ] F[ ] K[ ] ListNotifications
- W[ ] S[ ] M[ ] F[ ] K[ ] ListPhoneNumbers
- W[ ] S[ ] M[ ] F[ ] K[ ] ListPhoneNumbersV2
- W[ ] S[ ] M[ ] F[ ] K[ ] ListPredefinedAttributes
- W[ ] S[ ] M[ ] F[ ] K[ ] ListPrompts
- W[ ] S[ ] M[ ] F[ ] K[ ] ListQueueEmailAddresses
- W[ ] S[ ] M[ ] F[ ] K[ ] ListQueueQuickConnects
- W[ ] S[ ] M[ ] F[ ] K[ ] ListQueues
- W[ ] S[ ] M[ ] F[ ] K[ ] ListQuickConnects
- W[ ] S[ ] M[ ] F[ ] K[ ] ListRealtimeContactAnalysisSegmentsV2
- W[ ] S[ ] M[ ] F[ ] K[ ] ListRoutingProfileManualAssignmentQueues
- W[ ] S[ ] M[ ] F[ ] K[ ] ListRoutingProfileQueues
- W[ ] S[ ] M[ ] F[ ] K[ ] ListRoutingProfiles
- W[ ] S[ ] M[ ] F[ ] K[ ] ListRules
- W[ ] S[ ] M[ ] F[ ] K[ ] ListSecurityKeys
- W[ ] S[ ] M[ ] F[ ] K[ ] ListSecurityProfileApplications
- W[ ] S[ ] M[ ] F[ ] K[ ] ListSecurityProfileFlowModules
- W[ ] S[ ] M[ ] F[ ] K[ ] ListSecurityProfilePermissions
- W[ ] S[ ] M[ ] F[ ] K[ ] ListSecurityProfiles
- W[x] S[ ] M[x] F[ ] K[ ] ListTagsForResource
- W[ ] S[ ] M[ ] F[ ] K[ ] ListTaskTemplates
- W[ ] S[ ] M[ ] F[ ] K[ ] ListTestCaseExecutionRecords
- W[ ] S[ ] M[ ] F[ ] K[ ] ListTestCaseExecutions
- W[ ] S[ ] M[ ] F[ ] K[ ] ListTestCases
- W[ ] S[ ] M[ ] F[ ] K[ ] ListTrafficDistributionGroupUsers
- W[ ] S[ ] M[ ] F[ ] K[ ] ListTrafficDistributionGroups
- W[ ] S[ ] M[ ] F[ ] K[ ] ListUseCases
- W[ ] S[ ] M[ ] F[ ] K[ ] ListUserHierarchyGroups
- W[ ] S[ ] M[ ] F[ ] K[ ] ListUserNotifications
- W[ ] S[ ] M[ ] F[ ] K[ ] ListUserProficiencies
- W[ ] S[ ] M[ ] F[ ] K[ ] ListUsers
- W[ ] S[ ] M[ ] F[ ] K[ ] ListViewVersions
- W[ ] S[ ] M[ ] F[ ] K[ ] ListViews
- W[ ] S[ ] M[ ] F[ ] K[ ] ListWorkspaceMedia
- W[ ] S[ ] M[ ] F[ ] K[ ] ListWorkspacePages
- W[ ] S[ ] M[ ] F[ ] K[ ] ListWorkspaces
- W[ ] S[ ] M[ ] F[ ] K[ ] MonitorContact
- W[ ] S[ ] M[ ] F[ ] K[ ] PauseContact
- W[ ] S[ ] M[ ] F[ ] K[ ] PutUserStatus
- W[ ] S[ ] M[ ] F[ ] K[ ] ReleasePhoneNumber
- W[ ] S[ ] M[ ] F[ ] K[ ] ReplicateInstance
- W[ ] S[ ] M[ ] F[ ] K[ ] ResumeContact
- W[ ] S[ ] M[ ] F[ ] K[ ] ResumeContactRecording
- W[ ] S[ ] M[ ] F[ ] K[ ] SearchAgentStatuses
- W[ ] S[ ] M[ ] F[ ] K[ ] SearchAvailablePhoneNumbers
- W[ ] S[ ] M[ ] F[ ] K[ ] SearchContactEvaluations
- W[ ] S[ ] M[ ] F[ ] K[ ] SearchContactFlowModules
- W[ ] S[ ] M[ ] F[ ] K[ ] SearchContactFlows
- W[ ] S[ ] M[ ] F[ ] K[ ] SearchContacts
- W[ ] S[ ] M[ ] F[ ] K[ ] SearchDataTables
- W[ ] S[ ] M[ ] F[ ] K[ ] SearchEmailAddresses
- W[ ] S[ ] M[ ] F[ ] K[ ] SearchEvaluationForms
- W[ ] S[ ] M[ ] F[ ] K[ ] SearchHoursOfOperationOverrides
- W[ ] S[ ] M[ ] F[ ] K[ ] SearchHoursOfOperations
- W[ ] S[ ] M[ ] F[ ] K[ ] SearchNotifications
- W[ ] S[ ] M[ ] F[ ] K[ ] SearchPredefinedAttributes
- W[ ] S[ ] M[ ] F[ ] K[ ] SearchPrompts
- W[ ] S[ ] M[ ] F[ ] K[ ] SearchQueues
- W[ ] S[ ] M[ ] F[ ] K[ ] SearchQuickConnects
- W[ ] S[ ] M[ ] F[ ] K[ ] SearchResourceTags
- W[ ] S[ ] M[ ] F[ ] K[ ] SearchRoutingProfiles
- W[ ] S[ ] M[ ] F[ ] K[ ] SearchSecurityProfiles
- W[ ] S[ ] M[ ] F[ ] K[ ] SearchTestCases
- W[ ] S[ ] M[ ] F[ ] K[ ] SearchUserHierarchyGroups
- W[ ] S[ ] M[ ] F[ ] K[ ] SearchUsers
- W[ ] S[ ] M[ ] F[ ] K[ ] SearchViews
- W[ ] S[ ] M[ ] F[ ] K[ ] SearchVocabularies
- W[ ] S[ ] M[ ] F[ ] K[ ] SearchWorkspaceAssociations
- W[ ] S[ ] M[ ] F[ ] K[ ] SearchWorkspaces
- W[ ] S[ ] M[ ] F[ ] K[ ] SendChatIntegrationEvent
- W[ ] S[ ] M[ ] F[ ] K[ ] SendOutboundEmail
- W[ ] S[ ] M[ ] F[ ] K[ ] StartAttachedFileUpload
- W[ ] S[ ] M[ ] F[ ] K[ ] StartChatContact
- W[ ] S[ ] M[ ] F[ ] K[ ] StartContactEvaluation
- W[ ] S[ ] M[ ] F[ ] K[ ] StartContactMediaProcessing
- W[ ] S[ ] M[ ] F[ ] K[ ] StartContactRecording
- W[ ] S[ ] M[ ] F[ ] K[ ] StartContactStreaming
- W[ ] S[ ] M[ ] F[ ] K[ ] StartEmailContact
- W[ ] S[ ] M[ ] F[ ] K[ ] StartOutboundChatContact
- W[ ] S[ ] M[ ] F[ ] K[ ] StartOutboundEmailContact
- W[ ] S[ ] M[ ] F[ ] K[ ] StartOutboundVoiceContact
- W[ ] S[ ] M[ ] F[ ] K[ ] StartScreenSharing
- W[ ] S[ ] M[ ] F[ ] K[ ] StartTaskContact
- W[ ] S[ ] M[ ] F[ ] K[ ] StartTestCaseExecution
- W[ ] S[ ] M[ ] F[ ] K[ ] StartWebRTCContact
- W[ ] S[ ] M[ ] F[ ] K[ ] StopContact
- W[ ] S[ ] M[ ] F[ ] K[ ] StopContactMediaProcessing
- W[ ] S[ ] M[ ] F[ ] K[ ] StopContactRecording
- W[ ] S[ ] M[ ] F[ ] K[ ] StopContactStreaming
- W[ ] S[ ] M[ ] F[ ] K[ ] StopTestCaseExecution
- W[ ] S[ ] M[ ] F[ ] K[ ] SubmitContactEvaluation
- W[ ] S[ ] M[ ] F[ ] K[ ] SuspendContactRecording
- W[ ] S[ ] M[ ] F[ ] K[ ] TagContact
- W[x] S[ ] M[x] F[ ] K[ ] TagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] TransferContact
- W[ ] S[ ] M[ ] F[ ] K[ ] UntagContact
- W[x] S[ ] M[x] F[ ] K[ ] UntagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateAgentStatus
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateAttachedFilesConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateAuthenticationProfile
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateContact
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateContactAttributes
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateContactEvaluation
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateContactFlowContent
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateContactFlowMetadata
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateContactFlowModuleAlias
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateContactFlowModuleContent
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateContactFlowModuleMetadata
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateContactFlowName
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateContactRoutingData
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateContactSchedule
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateDataTableAttribute
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateDataTableMetadata
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateDataTablePrimaryValues
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateEmailAddressMetadata
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateEvaluationForm
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateHoursOfOperation
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateHoursOfOperationOverride
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateInstanceAttribute
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateInstanceStorageConfig
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateNotificationContent
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateParticipantAuthentication
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateParticipantRoleConfig
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdatePhoneNumber
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdatePhoneNumberMetadata
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdatePredefinedAttribute
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdatePrompt
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateQueueHoursOfOperation
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateQueueMaxContacts
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateQueueName
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateQueueOutboundCallerConfig
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateQueueOutboundEmailConfig
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateQueueStatus
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateQuickConnectConfig
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateQuickConnectName
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateRoutingProfileAgentAvailabilityTimer
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateRoutingProfileConcurrency
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateRoutingProfileDefaultOutboundQueue
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateRoutingProfileName
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateRoutingProfileQueues
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateRule
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateSecurityProfile
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateTaskTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateTestCase
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateTrafficDistribution
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateUserConfig
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateUserHierarchy
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateUserHierarchyGroupName
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateUserHierarchyStructure
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateUserIdentityInfo
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateUserNotificationStatus
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateUserPhoneConfig
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateUserProficiencies
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateUserRoutingProfile
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateUserSecurityProfiles
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateViewContent
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateViewMetadata
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateWorkspaceMetadata
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateWorkspacePage
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateWorkspaceTheme
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateWorkspaceVisibility

Integration tests: 10/10 implemented operations tested (100.0%)

### winterbaume-connectcampaigns (connectcampaigns) - W: 14/22, S: 0/22, M: 14/22, F: 0/22, K: 0/22

- W[x] S[ ] M[x] F[ ] K[ ] CreateCampaign
- W[x] S[ ] M[x] F[ ] K[ ] DeleteCampaign
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteConnectInstanceConfig
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteInstanceOnboardingJob
- W[x] S[ ] M[x] F[ ] K[ ] DescribeCampaign
- W[x] S[ ] M[x] F[ ] K[ ] GetCampaignState
- W[ ] S[ ] M[ ] F[ ] K[ ] GetCampaignStateBatch
- W[x] S[ ] M[x] F[ ] K[ ] GetConnectInstanceConfig
- W[ ] S[ ] M[ ] F[ ] K[ ] GetInstanceOnboardingJobStatus
- W[x] S[ ] M[x] F[ ] K[ ] ListCampaigns
- W[x] S[ ] M[x] F[ ] K[ ] ListTagsForResource
- W[x] S[ ] M[x] F[ ] K[ ] PauseCampaign
- W[ ] S[ ] M[ ] F[ ] K[ ] PutDialRequestBatch
- W[x] S[ ] M[x] F[ ] K[ ] ResumeCampaign
- W[x] S[ ] M[x] F[ ] K[ ] StartCampaign
- W[x] S[ ] M[x] F[ ] K[ ] StartInstanceOnboardingJob
- W[x] S[ ] M[x] F[ ] K[ ] StopCampaign
- W[x] S[ ] M[x] F[ ] K[ ] TagResource
- W[x] S[ ] M[x] F[ ] K[ ] UntagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateCampaignDialerConfig
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateCampaignName
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateCampaignOutboundCallConfig

Integration tests: 14/14 implemented operations tested (100.0%)

### winterbaume-connectcontactlens (connect-contact-lens) - W: 0/1, S: 0/1, M: 0/1, F: 0/1, K: 0/1

- W[ ] S[ ] M[ ] F[ ] K[ ] ListRealtimeContactAnalysisSegments

Integration tests: 0/0 implemented operations tested (0.0%)

### winterbaume-connectparticipant (connectparticipant) - W: 7/11, S: 0/11, M: 0/11, F: 0/11, K: 0/11

- W[ ] S[ ] M[ ] F[ ] K[ ] CancelParticipantAuthentication
- W[x] S[ ] M[ ] F[ ] K[ ] CompleteAttachmentUpload
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateParticipantConnection
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeView
- W[ ] S[ ] M[ ] F[ ] K[ ] DisconnectParticipant
- W[x] S[ ] M[ ] F[ ] K[ ] GetAttachment
- W[ ] S[ ] M[ ] F[ ] K[ ] GetAuthenticationUrl
- W[x] S[ ] M[ ] F[ ] K[ ] GetTranscript
- W[x] S[ ] M[ ] F[ ] K[ ] SendEvent
- W[x] S[ ] M[ ] F[ ] K[ ] SendMessage
- W[x] S[ ] M[ ] F[ ] K[ ] StartAttachmentUpload

Integration tests: 5/7 implemented operations tested (71.4%)
Untested implemented operations: 2

### winterbaume-controlcatalog (controlcatalog) - W: 6/6, S: 0/6, M: 0/6, F: 0/6, K: 0/6

- W[x] S[ ] M[ ] F[ ] K[ ] GetControl
- W[x] S[ ] M[ ] F[ ] K[ ] ListCommonControls
- W[x] S[ ] M[ ] F[ ] K[ ] ListControlMappings
- W[x] S[ ] M[ ] F[ ] K[ ] ListControls
- W[x] S[ ] M[ ] F[ ] K[ ] ListDomains
- W[x] S[ ] M[ ] F[ ] K[ ] ListObjectives

Integration tests: 5/6 implemented operations tested (83.3%)
Untested implemented operations: 1

### winterbaume-costandusagereport (cost-and-usage-report-service) - W: 7/7, S: 0/7, M: 0/7, F: 0/7, K: 0/7

- W[x] S[ ] M[ ] F[ ] K[ ] DeleteReportDefinition
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeReportDefinitions
- W[x] S[ ] M[ ] F[ ] K[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyReportDefinition
- W[x] S[ ] M[ ] F[ ] K[ ] PutReportDefinition
- W[x] S[ ] M[ ] F[ ] K[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UntagResource

Integration tests: 7/7 implemented operations tested (100.0%)

### winterbaume-costexplorer (cost-explorer) - W: 22/47, S: 25/47, M: 0/47, F: 0/47, K: 8/47

- W[x] S[ ] M[ ] F[ ] K[ ] CreateAnomalyMonitor
- W[x] S[ ] M[ ] F[ ] K[ ] CreateAnomalySubscription
- W[x] S[ ] M[ ] F[ ] K[x] CreateCostCategoryDefinition
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteAnomalyMonitor
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteAnomalySubscription
- W[x] S[ ] M[ ] F[ ] K[x] DeleteCostCategoryDefinition
- W[x] S[ ] M[ ] F[ ] K[x] DescribeCostCategoryDefinition
- W[ ] S[x] M[ ] F[ ] K[ ] GetAnomalies
- W[x] S[ ] M[ ] F[ ] K[ ] GetAnomalyMonitors
- W[x] S[ ] M[ ] F[ ] K[ ] GetAnomalySubscriptions
- W[ ] S[x] M[ ] F[ ] K[ ] GetApproximateUsageRecords
- W[ ] S[x] M[ ] F[ ] K[ ] GetCommitmentPurchaseAnalysis
- W[ ] S[x] M[ ] F[ ] K[x] GetCostAndUsage
- W[ ] S[x] M[ ] F[ ] K[ ] GetCostAndUsageComparisons
- W[ ] S[x] M[ ] F[ ] K[ ] GetCostAndUsageWithResources
- W[ ] S[x] M[ ] F[ ] K[ ] GetCostCategories
- W[ ] S[x] M[ ] F[ ] K[ ] GetCostComparisonDrivers
- W[ ] S[x] M[ ] F[ ] K[x] GetCostForecast
- W[ ] S[x] M[ ] F[ ] K[x] GetDimensionValues
- W[ ] S[x] M[ ] F[ ] K[ ] GetReservationCoverage
- W[ ] S[x] M[ ] F[ ] K[ ] GetReservationPurchaseRecommendation
- W[ ] S[x] M[ ] F[ ] K[ ] GetReservationUtilization
- W[ ] S[x] M[ ] F[ ] K[ ] GetRightsizingRecommendation
- W[ ] S[x] M[ ] F[ ] K[ ] GetSavingsPlanPurchaseRecommendationDetails
- W[ ] S[x] M[ ] F[ ] K[ ] GetSavingsPlansCoverage
- W[ ] S[x] M[ ] F[ ] K[ ] GetSavingsPlansPurchaseRecommendation
- W[ ] S[x] M[ ] F[ ] K[ ] GetSavingsPlansUtilization
- W[ ] S[x] M[ ] F[ ] K[ ] GetSavingsPlansUtilizationDetails
- W[ ] S[x] M[ ] F[ ] K[x] GetTags
- W[ ] S[x] M[ ] F[ ] K[ ] GetUsageForecast
- W[ ] S[x] M[ ] F[ ] K[ ] ListCommitmentPurchaseAnalyses
- W[x] S[ ] M[ ] F[ ] K[ ] ListCostAllocationTagBackfillHistory
- W[x] S[ ] M[ ] F[ ] K[ ] ListCostAllocationTags
- W[x] S[ ] M[ ] F[ ] K[x] ListCostCategoryDefinitions
- W[x] S[ ] M[ ] F[ ] K[ ] ListCostCategoryResourceAssociations
- W[ ] S[x] M[ ] F[ ] K[ ] ListSavingsPlansPurchaseRecommendationGeneration
- W[x] S[ ] M[ ] F[ ] K[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] ProvideAnomalyFeedback
- W[ ] S[x] M[ ] F[ ] K[ ] StartCommitmentPurchaseAnalysis
- W[x] S[ ] M[ ] F[ ] K[ ] StartCostAllocationTagBackfill
- W[ ] S[x] M[ ] F[ ] K[ ] StartSavingsPlansPurchaseRecommendationGeneration
- W[x] S[ ] M[ ] F[ ] K[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateAnomalyMonitor
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateAnomalySubscription
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateCostAllocationTagsStatus
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateCostCategoryDefinition

Integration tests: 21/22 implemented operations tested (95.5%)
Untested implemented operations: 1

### winterbaume-costoptimizationhub (cost-optimization-hub) - W: 8/8, S: 0/8, M: 0/8, F: 0/8, K: 0/8

- W[x] S[ ] M[ ] F[ ] K[ ] GetPreferences
- W[x] S[ ] M[ ] F[ ] K[ ] GetRecommendation
- W[x] S[ ] M[ ] F[ ] K[ ] ListEfficiencyMetrics
- W[x] S[ ] M[ ] F[ ] K[ ] ListEnrollmentStatuses
- W[x] S[ ] M[ ] F[ ] K[ ] ListRecommendationSummaries
- W[x] S[ ] M[ ] F[ ] K[ ] ListRecommendations
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateEnrollmentStatus
- W[x] S[ ] M[ ] F[ ] K[ ] UpdatePreferences

Integration tests: 8/8 implemented operations tested (100.0%)

### winterbaume-databasemigration (database-migration-service) - W: 42/119, S: 0/119, M: 17/119, F: 0/119, K: 0/119

- W[x] S[ ] M[ ] F[ ] K[ ] AddTagsToResource
- W[ ] S[ ] M[ ] F[ ] K[ ] ApplyPendingMaintenanceAction
- W[ ] S[ ] M[ ] F[ ] K[ ] BatchStartRecommendations
- W[ ] S[ ] M[ ] F[ ] K[ ] CancelMetadataModelConversion
- W[ ] S[ ] M[ ] F[ ] K[ ] CancelMetadataModelCreation
- W[ ] S[ ] M[ ] F[ ] K[ ] CancelReplicationTaskAssessmentRun
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateDataMigration
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateDataProvider
- W[x] S[ ] M[x] F[ ] K[ ] CreateEndpoint
- W[x] S[ ] M[ ] F[ ] K[ ] CreateEventSubscription
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateFleetAdvisorCollector
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateInstanceProfile
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateMigrationProject
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateReplicationConfig
- W[x] S[ ] M[x] F[ ] K[ ] CreateReplicationInstance
- W[x] S[ ] M[x] F[ ] K[ ] CreateReplicationSubnetGroup
- W[x] S[ ] M[x] F[ ] K[ ] CreateReplicationTask
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteCertificate
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteConnection
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteDataMigration
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteDataProvider
- W[x] S[ ] M[x] F[ ] K[ ] DeleteEndpoint
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteEventSubscription
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteFleetAdvisorCollector
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteFleetAdvisorDatabases
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteInstanceProfile
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteMigrationProject
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteReplicationConfig
- W[x] S[ ] M[x] F[ ] K[ ] DeleteReplicationInstance
- W[x] S[ ] M[x] F[ ] K[ ] DeleteReplicationSubnetGroup
- W[x] S[ ] M[x] F[ ] K[ ] DeleteReplicationTask
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteReplicationTaskAssessmentRun
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeAccountAttributes
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeApplicableIndividualAssessments
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeCertificates
- W[x] S[ ] M[x] F[ ] K[ ] DescribeConnections
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeConversionConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeDataMigrations
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeDataProviders
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeEndpointSettings
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeEndpointTypes
- W[x] S[ ] M[x] F[ ] K[ ] DescribeEndpoints
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeEngineVersions
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeEventCategories
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeEventSubscriptions
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeEvents
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeExtensionPackAssociations
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeFleetAdvisorCollectors
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeFleetAdvisorDatabases
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeFleetAdvisorLsaAnalysis
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeFleetAdvisorSchemaObjectSummary
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeFleetAdvisorSchemas
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeInstanceProfiles
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeMetadataModel
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeMetadataModelAssessments
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeMetadataModelChildren
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeMetadataModelConversions
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeMetadataModelCreations
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeMetadataModelExportsAsScript
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeMetadataModelExportsToTarget
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeMetadataModelImports
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeMigrationProjects
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeOrderableReplicationInstances
- W[x] S[ ] M[ ] F[ ] K[ ] DescribePendingMaintenanceActions
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeRecommendationLimitations
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeRecommendations
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeRefreshSchemasStatus
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeReplicationConfigs
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeReplicationInstanceTaskLogs
- W[x] S[ ] M[x] F[ ] K[ ] DescribeReplicationInstances
- W[x] S[ ] M[x] F[ ] K[ ] DescribeReplicationSubnetGroups
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeReplicationTableStatistics
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeReplicationTaskAssessmentResults
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeReplicationTaskAssessmentRuns
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeReplicationTaskIndividualAssessments
- W[x] S[ ] M[x] F[ ] K[ ] DescribeReplicationTasks
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeReplications
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeSchemas
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeTableStatistics
- W[ ] S[ ] M[ ] F[ ] K[ ] ExportMetadataModelAssessment
- W[ ] S[ ] M[ ] F[ ] K[ ] GetTargetSelectionRules
- W[x] S[ ] M[ ] F[ ] K[ ] ImportCertificate
- W[x] S[ ] M[x] F[ ] K[ ] ListTagsForResource
- W[ ] S[ ] M[ ] F[ ] K[ ] ModifyConversionConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] ModifyDataMigration
- W[ ] S[ ] M[ ] F[ ] K[ ] ModifyDataProvider
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyEndpoint
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyEventSubscription
- W[ ] S[ ] M[ ] F[ ] K[ ] ModifyInstanceProfile
- W[ ] S[ ] M[ ] F[ ] K[ ] ModifyMigrationProject
- W[ ] S[ ] M[ ] F[ ] K[ ] ModifyReplicationConfig
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyReplicationInstance
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyReplicationSubnetGroup
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyReplicationTask
- W[ ] S[ ] M[ ] F[ ] K[ ] MoveReplicationTask
- W[ ] S[ ] M[ ] F[ ] K[ ] RebootReplicationInstance
- W[ ] S[ ] M[ ] F[ ] K[ ] RefreshSchemas
- W[ ] S[ ] M[ ] F[ ] K[ ] ReloadReplicationTables
- W[ ] S[ ] M[ ] F[ ] K[ ] ReloadTables
- W[x] S[ ] M[ ] F[ ] K[ ] RemoveTagsFromResource
- W[ ] S[ ] M[ ] F[ ] K[ ] RunFleetAdvisorLsaAnalysis
- W[ ] S[ ] M[ ] F[ ] K[ ] StartDataMigration
- W[ ] S[ ] M[ ] F[ ] K[ ] StartExtensionPackAssociation
- W[ ] S[ ] M[ ] F[ ] K[ ] StartMetadataModelAssessment
- W[ ] S[ ] M[ ] F[ ] K[ ] StartMetadataModelConversion
- W[ ] S[ ] M[ ] F[ ] K[ ] StartMetadataModelCreation
- W[ ] S[ ] M[ ] F[ ] K[ ] StartMetadataModelExportAsScript
- W[ ] S[ ] M[ ] F[ ] K[ ] StartMetadataModelExportToTarget
- W[ ] S[ ] M[ ] F[ ] K[ ] StartMetadataModelImport
- W[ ] S[ ] M[ ] F[ ] K[ ] StartRecommendations
- W[ ] S[ ] M[ ] F[ ] K[ ] StartReplication
- W[x] S[ ] M[x] F[ ] K[ ] StartReplicationTask
- W[ ] S[ ] M[ ] F[ ] K[ ] StartReplicationTaskAssessment
- W[ ] S[ ] M[ ] F[ ] K[ ] StartReplicationTaskAssessmentRun
- W[ ] S[ ] M[ ] F[ ] K[ ] StopDataMigration
- W[ ] S[ ] M[ ] F[ ] K[ ] StopReplication
- W[x] S[ ] M[x] F[ ] K[ ] StopReplicationTask
- W[x] S[ ] M[x] F[ ] K[ ] TestConnection
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateSubscriptionsToEventBridge

Integration tests: 41/42 implemented operations tested (97.6%)
Untested implemented operations: 1

### winterbaume-databrew (databrew) - W: 32/44, S: 1/44, M: 24/44, F: 0/44, K: 0/44

- W[ ] S[ ] M[ ] F[ ] K[ ] BatchDeleteRecipeVersion
- W[x] S[ ] M[x] F[ ] K[ ] CreateDataset
- W[x] S[ ] M[x] F[ ] K[ ] CreateProfileJob
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateProject
- W[x] S[ ] M[x] F[ ] K[ ] CreateRecipe
- W[x] S[ ] M[x] F[ ] K[ ] CreateRecipeJob
- W[x] S[ ] M[x] F[ ] K[ ] CreateRuleset
- W[x] S[ ] M[ ] F[ ] K[ ] CreateSchedule
- W[x] S[ ] M[x] F[ ] K[ ] DeleteDataset
- W[x] S[ ] M[x] F[ ] K[ ] DeleteJob
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteProject
- W[x] S[ ] M[x] F[ ] K[ ] DeleteRecipeVersion
- W[x] S[ ] M[x] F[ ] K[ ] DeleteRuleset
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteSchedule
- W[x] S[ ] M[x] F[ ] K[ ] DescribeDataset
- W[x] S[ ] M[x] F[ ] K[ ] DescribeJob
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeJobRun
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeProject
- W[x] S[ ] M[x] F[ ] K[ ] DescribeRecipe
- W[x] S[ ] M[x] F[ ] K[ ] DescribeRuleset
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeSchedule
- W[x] S[ ] M[x] F[ ] K[ ] ListDatasets
- W[ ] S[ ] M[ ] F[ ] K[ ] ListJobRuns
- W[x] S[ ] M[x] F[ ] K[ ] ListJobs
- W[ ] S[ ] M[ ] F[ ] K[ ] ListProjects
- W[x] S[ ] M[x] F[ ] K[ ] ListRecipeVersions
- W[x] S[ ] M[x] F[ ] K[ ] ListRecipes
- W[x] S[ ] M[x] F[ ] K[ ] ListRulesets
- W[x] S[ ] M[ ] F[ ] K[ ] ListSchedules
- W[x] S[ ] M[ ] F[ ] K[ ] ListTagsForResource
- W[x] S[ ] M[x] F[ ] K[ ] PublishRecipe
- W[ ] S[x] M[ ] F[ ] K[ ] SendProjectSessionAction
- W[ ] S[ ] M[ ] F[ ] K[ ] StartJobRun
- W[ ] S[ ] M[ ] F[ ] K[ ] StartProjectSession
- W[ ] S[ ] M[ ] F[ ] K[ ] StopJobRun
- W[x] S[ ] M[ ] F[ ] K[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UntagResource
- W[x] S[ ] M[x] F[ ] K[ ] UpdateDataset
- W[x] S[ ] M[x] F[ ] K[ ] UpdateProfileJob
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateProject
- W[x] S[ ] M[x] F[ ] K[ ] UpdateRecipe
- W[x] S[ ] M[x] F[ ] K[ ] UpdateRecipeJob
- W[x] S[ ] M[x] F[ ] K[ ] UpdateRuleset
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateSchedule

Integration tests: 32/32 implemented operations tested (100.0%)

### winterbaume-datapipeline (data-pipeline) - W: 19/19, S: 0/19, M: 0/19, F: 0/19, K: 0/19

- W[x] S[ ] M[ ] F[ ] K[ ] ActivatePipeline
- W[x] S[ ] M[ ] F[ ] K[ ] AddTags
- W[x] S[ ] M[ ] F[ ] K[ ] CreatePipeline
- W[x] S[ ] M[ ] F[ ] K[ ] DeactivatePipeline
- W[x] S[ ] M[ ] F[ ] K[ ] DeletePipeline
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeObjects
- W[x] S[ ] M[ ] F[ ] K[ ] DescribePipelines
- W[x] S[ ] M[ ] F[ ] K[ ] EvaluateExpression
- W[x] S[ ] M[ ] F[ ] K[ ] GetPipelineDefinition
- W[x] S[ ] M[ ] F[ ] K[ ] ListPipelines
- W[x] S[ ] M[ ] F[ ] K[ ] PollForTask
- W[x] S[ ] M[ ] F[ ] K[ ] PutPipelineDefinition
- W[x] S[ ] M[ ] F[ ] K[ ] QueryObjects
- W[x] S[ ] M[ ] F[ ] K[ ] RemoveTags
- W[x] S[ ] M[ ] F[ ] K[ ] ReportTaskProgress
- W[x] S[ ] M[ ] F[ ] K[ ] ReportTaskRunnerHeartbeat
- W[x] S[ ] M[ ] F[ ] K[ ] SetStatus
- W[x] S[ ] M[ ] F[ ] K[ ] SetTaskStatus
- W[x] S[ ] M[ ] F[ ] K[ ] ValidatePipelineDefinition

Integration tests: 17/19 implemented operations tested (89.5%)
Untested implemented operations: 2

### winterbaume-datasync (datasync) - W: 8/53, S: 0/53, M: 6/53, F: 0/53, K: 0/53

- W[x] S[ ] M[x] F[ ] K[ ] CancelTaskExecution
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateAgent
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateLocationAzureBlob
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateLocationEfs
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateLocationFsxLustre
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateLocationFsxOntap
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateLocationFsxOpenZfs
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateLocationFsxWindows
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateLocationHdfs
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateLocationNfs
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateLocationObjectStorage
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateLocationS3
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateLocationSmb
- W[x] S[ ] M[x] F[ ] K[ ] CreateTask
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteAgent
- W[x] S[ ] M[x] F[ ] K[ ] DeleteLocation
- W[x] S[ ] M[x] F[ ] K[ ] DeleteTask
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeAgent
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeLocationAzureBlob
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeLocationEfs
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeLocationFsxLustre
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeLocationFsxOntap
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeLocationFsxOpenZfs
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeLocationFsxWindows
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeLocationHdfs
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeLocationNfs
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeLocationObjectStorage
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeLocationS3
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeLocationSmb
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeTask
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeTaskExecution
- W[ ] S[ ] M[ ] F[ ] K[ ] ListAgents
- W[ ] S[ ] M[ ] F[ ] K[ ] ListLocations
- W[ ] S[ ] M[ ] F[ ] K[ ] ListTagsForResource
- W[ ] S[ ] M[ ] F[ ] K[ ] ListTaskExecutions
- W[x] S[ ] M[ ] F[ ] K[ ] ListTasks
- W[x] S[ ] M[x] F[ ] K[ ] StartTaskExecution
- W[ ] S[ ] M[ ] F[ ] K[ ] TagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] UntagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateAgent
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateLocationAzureBlob
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateLocationEfs
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateLocationFsxLustre
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateLocationFsxOntap
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateLocationFsxOpenZfs
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateLocationFsxWindows
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateLocationHdfs
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateLocationNfs
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateLocationObjectStorage
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateLocationS3
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateLocationSmb
- W[x] S[ ] M[x] F[ ] K[ ] UpdateTask
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateTaskExecution

Integration tests: 8/8 implemented operations tested (100.0%)

### winterbaume-dax (dax) - W: 6/21, S: 0/21, M: 8/21, F: 0/21, K: 0/21

- W[x] S[ ] M[x] F[ ] K[ ] CreateCluster
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateParameterGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateSubnetGroup
- W[x] S[ ] M[x] F[ ] K[ ] DecreaseReplicationFactor
- W[x] S[ ] M[x] F[ ] K[ ] DeleteCluster
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteParameterGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteSubnetGroup
- W[x] S[ ] M[x] F[ ] K[ ] DescribeClusters
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeDefaultParameters
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeEvents
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeParameterGroups
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeParameters
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeSubnetGroups
- W[x] S[ ] M[x] F[ ] K[ ] IncreaseReplicationFactor
- W[x] S[ ] M[x] F[ ] K[ ] ListTags
- W[ ] S[ ] M[ ] F[ ] K[ ] RebootNode
- W[ ] S[ ] M[x] F[ ] K[ ] TagResource
- W[ ] S[ ] M[x] F[ ] K[ ] UntagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateCluster
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateParameterGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateSubnetGroup

Integration tests: 6/6 implemented operations tested (100.0%)

### winterbaume-directconnect (direct-connect) - W: 7/63, S: 0/63, M: 0/63, F: 0/63, K: 0/63

- W[ ] S[ ] M[ ] F[ ] K[ ] AcceptDirectConnectGatewayAssociationProposal
- W[ ] S[ ] M[ ] F[ ] K[ ] AllocateConnectionOnInterconnect
- W[ ] S[ ] M[ ] F[ ] K[ ] AllocateHostedConnection
- W[ ] S[ ] M[ ] F[ ] K[ ] AllocatePrivateVirtualInterface
- W[ ] S[ ] M[ ] F[ ] K[ ] AllocatePublicVirtualInterface
- W[ ] S[ ] M[ ] F[ ] K[ ] AllocateTransitVirtualInterface
- W[ ] S[ ] M[ ] F[ ] K[ ] AssociateConnectionWithLag
- W[ ] S[ ] M[ ] F[ ] K[ ] AssociateHostedConnection
- W[ ] S[ ] M[ ] F[ ] K[ ] AssociateMacSecKey
- W[ ] S[ ] M[ ] F[ ] K[ ] AssociateVirtualInterface
- W[ ] S[ ] M[ ] F[ ] K[ ] ConfirmConnection
- W[ ] S[ ] M[ ] F[ ] K[ ] ConfirmCustomerAgreement
- W[ ] S[ ] M[ ] F[ ] K[ ] ConfirmPrivateVirtualInterface
- W[ ] S[ ] M[ ] F[ ] K[ ] ConfirmPublicVirtualInterface
- W[ ] S[ ] M[ ] F[ ] K[ ] ConfirmTransitVirtualInterface
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateBGPPeer
- W[x] S[ ] M[ ] F[ ] K[ ] CreateConnection
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateDirectConnectGateway
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateDirectConnectGatewayAssociation
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateDirectConnectGatewayAssociationProposal
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateInterconnect
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateLag
- W[ ] S[ ] M[ ] F[ ] K[ ] CreatePrivateVirtualInterface
- W[ ] S[ ] M[ ] F[ ] K[ ] CreatePublicVirtualInterface
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateTransitVirtualInterface
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteBGPPeer
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteConnection
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteDirectConnectGateway
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteDirectConnectGatewayAssociation
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteDirectConnectGatewayAssociationProposal
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteInterconnect
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteLag
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteVirtualInterface
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeConnectionLoa
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeConnections
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeConnectionsOnInterconnect
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeCustomerMetadata
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeDirectConnectGatewayAssociationProposals
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeDirectConnectGatewayAssociations
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeDirectConnectGatewayAttachments
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeDirectConnectGateways
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeHostedConnections
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeInterconnectLoa
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeInterconnects
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeLags
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeLoa
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeLocations
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeRouterConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeTags
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeVirtualGateways
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeVirtualInterfaces
- W[ ] S[ ] M[ ] F[ ] K[ ] DisassociateConnectionFromLag
- W[ ] S[ ] M[ ] F[ ] K[ ] DisassociateMacSecKey
- W[ ] S[ ] M[ ] F[ ] K[ ] ListVirtualInterfaceTestHistory
- W[ ] S[ ] M[ ] F[ ] K[ ] StartBgpFailoverTest
- W[ ] S[ ] M[ ] F[ ] K[ ] StopBgpFailoverTest
- W[x] S[ ] M[ ] F[ ] K[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UntagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateConnection
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateDirectConnectGateway
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateDirectConnectGatewayAssociation
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateLag
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateVirtualInterfaceAttributes

Integration tests: 7/7 implemented operations tested (100.0%)

### winterbaume-directory (directory-service) - W: 4/80, S: 0/80, M: 0/80, F: 0/80, K: 6/80

- W[ ] S[ ] M[ ] F[ ] K[ ] AcceptSharedDirectory
- W[ ] S[ ] M[ ] F[ ] K[ ] AddIpRoutes
- W[ ] S[ ] M[ ] F[ ] K[ ] AddRegion
- W[ ] S[ ] M[ ] F[ ] K[ ] AddTagsToResource
- W[ ] S[ ] M[ ] F[ ] K[ ] CancelSchemaExtension
- W[x] S[ ] M[ ] F[ ] K[ ] ConnectDirectory
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateAlias
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateComputer
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateConditionalForwarder
- W[x] S[ ] M[ ] F[ ] K[x] CreateDirectory
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateHybridAD
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateLogSubscription
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateMicrosoftAD
- W[ ] S[ ] M[ ] F[ ] K[x] CreateSnapshot
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateTrust
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteADAssessment
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteConditionalForwarder
- W[x] S[ ] M[ ] F[ ] K[x] DeleteDirectory
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteLogSubscription
- W[ ] S[ ] M[ ] F[ ] K[x] DeleteSnapshot
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteTrust
- W[ ] S[ ] M[ ] F[ ] K[ ] DeregisterCertificate
- W[ ] S[ ] M[ ] F[ ] K[ ] DeregisterEventTopic
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeADAssessment
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeCAEnrollmentPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeCertificate
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeClientAuthenticationSettings
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeConditionalForwarders
- W[x] S[ ] M[ ] F[ ] K[x] DescribeDirectories
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeDirectoryDataAccess
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeDomainControllers
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeEventTopics
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeHybridADUpdate
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeLDAPSSettings
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeRegions
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeSettings
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeSharedDirectories
- W[ ] S[ ] M[ ] F[ ] K[x] DescribeSnapshots
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeTrusts
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeUpdateDirectory
- W[ ] S[ ] M[ ] F[ ] K[ ] DisableCAEnrollmentPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] DisableClientAuthentication
- W[ ] S[ ] M[ ] F[ ] K[ ] DisableDirectoryDataAccess
- W[ ] S[ ] M[ ] F[ ] K[ ] DisableLDAPS
- W[ ] S[ ] M[ ] F[ ] K[ ] DisableRadius
- W[ ] S[ ] M[ ] F[ ] K[ ] DisableSso
- W[ ] S[ ] M[ ] F[ ] K[ ] EnableCAEnrollmentPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] EnableClientAuthentication
- W[ ] S[ ] M[ ] F[ ] K[ ] EnableDirectoryDataAccess
- W[ ] S[ ] M[ ] F[ ] K[ ] EnableLDAPS
- W[ ] S[ ] M[ ] F[ ] K[ ] EnableRadius
- W[ ] S[ ] M[ ] F[ ] K[ ] EnableSso
- W[ ] S[ ] M[ ] F[ ] K[ ] GetDirectoryLimits
- W[ ] S[ ] M[ ] F[ ] K[ ] GetSnapshotLimits
- W[ ] S[ ] M[ ] F[ ] K[ ] ListADAssessments
- W[ ] S[ ] M[ ] F[ ] K[ ] ListCertificates
- W[ ] S[ ] M[ ] F[ ] K[ ] ListIpRoutes
- W[ ] S[ ] M[ ] F[ ] K[ ] ListLogSubscriptions
- W[ ] S[ ] M[ ] F[ ] K[ ] ListSchemaExtensions
- W[ ] S[ ] M[ ] F[ ] K[ ] ListTagsForResource
- W[ ] S[ ] M[ ] F[ ] K[ ] RegisterCertificate
- W[ ] S[ ] M[ ] F[ ] K[ ] RegisterEventTopic
- W[ ] S[ ] M[ ] F[ ] K[ ] RejectSharedDirectory
- W[ ] S[ ] M[ ] F[ ] K[ ] RemoveIpRoutes
- W[ ] S[ ] M[ ] F[ ] K[ ] RemoveRegion
- W[ ] S[ ] M[ ] F[ ] K[ ] RemoveTagsFromResource
- W[ ] S[ ] M[ ] F[ ] K[ ] ResetUserPassword
- W[ ] S[ ] M[ ] F[ ] K[ ] RestoreFromSnapshot
- W[ ] S[ ] M[ ] F[ ] K[ ] ShareDirectory
- W[ ] S[ ] M[ ] F[ ] K[ ] StartADAssessment
- W[ ] S[ ] M[ ] F[ ] K[ ] StartSchemaExtension
- W[ ] S[ ] M[ ] F[ ] K[ ] UnshareDirectory
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateConditionalForwarder
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateDirectorySetup
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateHybridAD
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateNumberOfDomainControllers
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateRadius
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateSettings
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateTrust
- W[ ] S[ ] M[ ] F[ ] K[ ] VerifyTrust

Integration tests: 4/4 implemented operations tested (100.0%)

### winterbaume-dlm (dlm) - W: 2/8, S: 0/8, M: 0/8, F: 0/8, K: 5/8

- W[ ] S[ ] M[ ] F[ ] K[x] CreateLifecyclePolicy
- W[ ] S[ ] M[ ] F[ ] K[x] DeleteLifecyclePolicy
- W[ ] S[ ] M[ ] F[ ] K[x] GetLifecyclePolicies
- W[ ] S[ ] M[ ] F[ ] K[x] GetLifecyclePolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UntagResource
- W[ ] S[ ] M[ ] F[ ] K[x] UpdateLifecyclePolicy

Integration tests: 2/2 implemented operations tested (100.0%)

### winterbaume-dsql (dsql) - W: 12/12, S: 0/12, M: 5/12, F: 0/12, K: 0/12

- W[x] S[ ] M[x] F[ ] K[ ] CreateCluster
- W[x] S[ ] M[x] F[ ] K[ ] DeleteCluster
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteClusterPolicy
- W[x] S[ ] M[x] F[ ] K[ ] GetCluster
- W[x] S[ ] M[ ] F[ ] K[ ] GetClusterPolicy
- W[x] S[ ] M[x] F[ ] K[ ] GetVpcEndpointServiceName
- W[x] S[ ] M[ ] F[ ] K[ ] ListClusters
- W[x] S[ ] M[x] F[ ] K[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] PutClusterPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateCluster

Integration tests: 12/12 implemented operations tested (100.0%)

### winterbaume-dynamodb (dynamodb) - W: 57/57, S: 0/57, M: 39/57, F: 0/57, K: 20/57

Terraform E2E: 8 tests across 2 terraform resource types

Resource types: aws_dynamodb_table, aws_dynamodb_table_item

- W[x] S[ ] M[x] F[ ] K[ ] BatchExecuteStatement
- W[x] S[ ] M[x] F[ ] K[x] BatchGetItem
- W[x] S[ ] M[x] F[ ] K[x] BatchWriteItem
- W[x] S[ ] M[x] F[ ] K[ ] CreateBackup
- W[x] S[ ] M[ ] F[ ] K[ ] CreateGlobalTable
- W[x] S[ ] M[x] F[ ] K[x] CreateTable
- W[x] S[ ] M[x] F[ ] K[ ] DeleteBackup
- W[x] S[ ] M[x] F[ ] K[x] DeleteItem
- W[x] S[ ] M[x] F[ ] K[ ] DeleteResourcePolicy
- W[x] S[ ] M[x] F[ ] K[x] DeleteTable
- W[x] S[ ] M[x] F[ ] K[ ] DescribeBackup
- W[x] S[ ] M[x] F[ ] K[x] DescribeContinuousBackups
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeContributorInsights
- W[x] S[ ] M[x] F[ ] K[ ] DescribeEndpoints
- W[x] S[ ] M[x] F[ ] K[ ] DescribeExport
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeGlobalTable
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeGlobalTableSettings
- W[x] S[ ] M[x] F[ ] K[ ] DescribeImport
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeKinesisStreamingDestination
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeLimits
- W[x] S[ ] M[x] F[ ] K[x] DescribeTable
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeTableReplicaAutoScaling
- W[x] S[ ] M[x] F[ ] K[x] DescribeTimeToLive
- W[x] S[ ] M[ ] F[ ] K[ ] DisableKinesisStreamingDestination
- W[x] S[ ] M[ ] F[ ] K[ ] EnableKinesisStreamingDestination
- W[x] S[ ] M[x] F[ ] K[ ] ExecuteStatement
- W[x] S[ ] M[x] F[ ] K[ ] ExecuteTransaction
- W[x] S[ ] M[ ] F[ ] K[ ] ExportTableToPointInTime
- W[x] S[ ] M[x] F[ ] K[x] GetItem
- W[x] S[ ] M[x] F[ ] K[ ] GetResourcePolicy
- W[x] S[ ] M[x] F[ ] K[ ] ImportTable
- W[x] S[ ] M[x] F[ ] K[ ] ListBackups
- W[x] S[ ] M[ ] F[ ] K[ ] ListContributorInsights
- W[x] S[ ] M[x] F[ ] K[ ] ListExports
- W[x] S[ ] M[ ] F[ ] K[ ] ListGlobalTables
- W[x] S[ ] M[ ] F[ ] K[ ] ListImports
- W[x] S[ ] M[x] F[ ] K[x] ListTables
- W[x] S[ ] M[x] F[ ] K[x] ListTagsOfResource
- W[x] S[ ] M[x] F[ ] K[x] PutItem
- W[x] S[ ] M[x] F[ ] K[ ] PutResourcePolicy
- W[x] S[ ] M[x] F[ ] K[x] Query
- W[x] S[ ] M[x] F[ ] K[ ] RestoreTableFromBackup
- W[x] S[ ] M[x] F[ ] K[ ] RestoreTableToPointInTime
- W[x] S[ ] M[x] F[ ] K[x] Scan
- W[x] S[ ] M[x] F[ ] K[x] TagResource
- W[x] S[ ] M[x] F[ ] K[x] TransactGetItems
- W[x] S[ ] M[x] F[ ] K[x] TransactWriteItems
- W[x] S[ ] M[x] F[ ] K[x] UntagResource
- W[x] S[ ] M[x] F[ ] K[ ] UpdateContinuousBackups
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateContributorInsights
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateGlobalTable
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateGlobalTableSettings
- W[x] S[ ] M[x] F[ ] K[x] UpdateItem
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateKinesisStreamingDestination
- W[x] S[ ] M[x] F[ ] K[ ] UpdateTable
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateTableReplicaAutoScaling
- W[x] S[ ] M[x] F[ ] K[x] UpdateTimeToLive

Integration tests: 57/57 implemented operations tested (100.0%)

### winterbaume-dynamodbstreams (dynamodb-streams) - W: 4/4, S: 0/4, M: 0/4, F: 0/4, K: 0/4

- W[x] S[ ] M[ ] F[ ] K[ ] DescribeStream
- W[x] S[ ] M[ ] F[ ] K[ ] GetRecords
- W[x] S[ ] M[ ] F[ ] K[ ] GetShardIterator
- W[x] S[ ] M[ ] F[ ] K[ ] ListStreams

Integration tests: 4/4 implemented operations tested (100.0%)

### winterbaume-ebs (ebs) - W: 6/6, S: 0/6, M: 6/6, F: 0/6, K: 0/6

- W[x] S[ ] M[x] F[ ] K[ ] CompleteSnapshot
- W[x] S[ ] M[x] F[ ] K[ ] GetSnapshotBlock
- W[x] S[ ] M[x] F[ ] K[ ] ListChangedBlocks
- W[x] S[ ] M[x] F[ ] K[ ] ListSnapshotBlocks
- W[x] S[ ] M[x] F[ ] K[ ] PutSnapshotBlock
- W[x] S[ ] M[x] F[ ] K[ ] StartSnapshot

Integration tests: 6/6 implemented operations tested (100.0%)

### winterbaume-ec2 (ec2) - W: 713/763, S: 43/763, M: 223/763, F: 0/763, K: 39/763

Terraform E2E: 20 tests across 7 terraform resource types

Resource types: aws_internet_gateway, aws_key_pair, aws_route_table, aws_route_table_association, aws_security_group, aws_subnet, aws_vpc

- W[x] S[ ] M[ ] F[ ] K[ ] AcceptAddressTransfer
- W[x] S[ ] M[ ] F[ ] K[ ] AcceptCapacityReservationBillingOwnership
- W[x] S[ ] M[ ] F[ ] K[ ] AcceptReservedInstancesExchangeQuote
- W[ ] S[ ] M[ ] F[ ] K[ ] AcceptTransitGatewayClientVpnAttachment
- W[x] S[ ] M[ ] F[ ] K[ ] AcceptTransitGatewayMulticastDomainAssociations
- W[x] S[ ] M[x] F[ ] K[ ] AcceptTransitGatewayPeeringAttachment
- W[x] S[ ] M[ ] F[ ] K[ ] AcceptTransitGatewayVpcAttachment
- W[x] S[ ] M[ ] F[ ] K[ ] AcceptVpcEndpointConnections
- W[x] S[ ] M[x] F[ ] K[ ] AcceptVpcPeeringConnection
- W[x] S[ ] M[ ] F[ ] K[ ] AdvertiseByoipCidr
- W[x] S[ ] M[x] F[ ] K[ ] AllocateAddress
- W[x] S[ ] M[x] F[ ] K[ ] AllocateHosts
- W[x] S[ ] M[ ] F[ ] K[ ] AllocateIpamPoolCidr
- W[x] S[ ] M[ ] F[ ] K[ ] ApplySecurityGroupsToClientVpnTargetNetwork
- W[x] S[ ] M[x] F[ ] K[ ] AssignIpv6Addresses
- W[x] S[ ] M[x] F[ ] K[ ] AssignPrivateIpAddresses
- W[x] S[ ] M[ ] F[ ] K[ ] AssignPrivateNatGatewayAddress
- W[x] S[ ] M[x] F[ ] K[ ] AssociateAddress
- W[x] S[ ] M[ ] F[ ] K[ ] AssociateCapacityReservationBillingOwner
- W[x] S[ ] M[ ] F[ ] K[ ] AssociateClientVpnTargetNetwork
- W[x] S[ ] M[x] F[ ] K[ ] AssociateDhcpOptions
- W[x] S[ ] M[ ] F[ ] K[ ] AssociateEnclaveCertificateIamRole
- W[x] S[ ] M[x] F[ ] K[ ] AssociateIamInstanceProfile
- W[x] S[ ] M[ ] F[ ] K[ ] AssociateInstanceEventWindow
- W[x] S[ ] M[ ] F[ ] K[ ] AssociateIpamByoasn
- W[x] S[ ] M[ ] F[ ] K[ ] AssociateIpamResourceDiscovery
- W[x] S[ ] M[ ] F[ ] K[ ] AssociateNatGatewayAddress
- W[x] S[ ] M[ ] F[ ] K[ ] AssociateRouteServer
- W[x] S[ ] M[x] F[ ] K[x] AssociateRouteTable
- W[x] S[ ] M[ ] F[ ] K[ ] AssociateSecurityGroupVpc
- W[x] S[ ] M[x] F[ ] K[ ] AssociateSubnetCidrBlock
- W[x] S[ ] M[ ] F[ ] K[ ] AssociateTransitGatewayMulticastDomain
- W[x] S[ ] M[ ] F[ ] K[ ] AssociateTransitGatewayPolicyTable
- W[x] S[ ] M[x] F[ ] K[ ] AssociateTransitGatewayRouteTable
- W[x] S[ ] M[ ] F[ ] K[ ] AssociateTrunkInterface
- W[x] S[ ] M[x] F[ ] K[ ] AssociateVpcCidrBlock
- W[x] S[ ] M[ ] F[ ] K[ ] AttachClassicLinkVpc
- W[x] S[ ] M[x] F[ ] K[x] AttachInternetGateway
- W[x] S[ ] M[x] F[ ] K[ ] AttachNetworkInterface
- W[x] S[ ] M[ ] F[ ] K[ ] AttachVerifiedAccessTrustProvider
- W[x] S[ ] M[x] F[ ] K[ ] AttachVolume
- W[x] S[ ] M[x] F[ ] K[ ] AttachVpnGateway
- W[x] S[ ] M[ ] F[ ] K[ ] AuthorizeClientVpnIngress
- W[x] S[ ] M[x] F[ ] K[x] AuthorizeSecurityGroupEgress
- W[x] S[ ] M[x] F[ ] K[x] AuthorizeSecurityGroupIngress
- W[x] S[ ] M[ ] F[ ] K[ ] BundleInstance
- W[x] S[ ] M[ ] F[ ] K[ ] CancelBundleTask
- W[x] S[ ] M[ ] F[ ] K[ ] CancelCapacityReservation
- W[x] S[ ] M[ ] F[ ] K[ ] CancelCapacityReservationFleets
- W[x] S[ ] M[ ] F[ ] K[ ] CancelConversionTask
- W[x] S[ ] M[ ] F[ ] K[ ] CancelDeclarativePoliciesReport
- W[x] S[ ] M[ ] F[ ] K[ ] CancelExportTask
- W[x] S[ ] M[ ] F[ ] K[ ] CancelImageLaunchPermission
- W[x] S[ ] M[ ] F[ ] K[ ] CancelImportTask
- W[x] S[ ] M[ ] F[ ] K[ ] CancelReservedInstancesListing
- W[x] S[ ] M[x] F[ ] K[ ] CancelSpotFleetRequests
- W[x] S[ ] M[x] F[ ] K[ ] CancelSpotInstanceRequests
- W[x] S[ ] M[ ] F[ ] K[ ] ConfirmProductInstance
- W[x] S[ ] M[ ] F[ ] K[ ] CopyFpgaImage
- W[x] S[ ] M[x] F[ ] K[ ] CopyImage
- W[x] S[ ] M[x] F[ ] K[ ] CopySnapshot
- W[x] S[ ] M[ ] F[ ] K[ ] CopyVolumes
- W[x] S[ ] M[ ] F[ ] K[ ] CreateCapacityManagerDataExport
- W[x] S[ ] M[ ] F[ ] K[ ] CreateCapacityReservation
- W[x] S[ ] M[ ] F[ ] K[ ] CreateCapacityReservationBySplitting
- W[x] S[ ] M[ ] F[ ] K[ ] CreateCapacityReservationFleet
- W[x] S[ ] M[x] F[ ] K[ ] CreateCarrierGateway
- W[x] S[ ] M[ ] F[ ] K[ ] CreateClientVpnEndpoint
- W[x] S[ ] M[ ] F[ ] K[ ] CreateClientVpnRoute
- W[x] S[ ] M[ ] F[ ] K[ ] CreateCoipCidr
- W[x] S[ ] M[ ] F[ ] K[ ] CreateCoipPool
- W[x] S[ ] M[x] F[ ] K[ ] CreateCustomerGateway
- W[x] S[ ] M[x] F[ ] K[ ] CreateDefaultSubnet
- W[x] S[ ] M[x] F[ ] K[ ] CreateDefaultVpc
- W[x] S[ ] M[ ] F[ ] K[ ] CreateDelegateMacVolumeOwnershipTask
- W[x] S[ ] M[x] F[ ] K[ ] CreateDhcpOptions
- W[x] S[ ] M[x] F[ ] K[ ] CreateEgressOnlyInternetGateway
- W[x] S[ ] M[x] F[ ] K[ ] CreateFleet
- W[x] S[ ] M[x] F[ ] K[ ] CreateFlowLogs
- W[x] S[ ] M[ ] F[ ] K[ ] CreateFpgaImage
- W[x] S[ ] M[x] F[ ] K[ ] CreateImage
- W[x] S[ ] M[ ] F[ ] K[ ] CreateImageUsageReport
- W[x] S[ ] M[ ] F[ ] K[ ] CreateInstanceConnectEndpoint
- W[x] S[ ] M[ ] F[ ] K[ ] CreateInstanceEventWindow
- W[x] S[ ] M[ ] F[ ] K[ ] CreateInstanceExportTask
- W[x] S[ ] M[x] F[ ] K[x] CreateInternetGateway
- W[x] S[ ] M[ ] F[ ] K[ ] CreateInterruptibleCapacityReservationAllocation
- W[x] S[ ] M[ ] F[ ] K[ ] CreateIpam
- W[x] S[ ] M[ ] F[ ] K[ ] CreateIpamExternalResourceVerificationToken
- W[x] S[ ] M[ ] F[ ] K[ ] CreateIpamPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] CreateIpamPool
- W[x] S[ ] M[ ] F[ ] K[ ] CreateIpamPrefixListResolver
- W[x] S[ ] M[ ] F[ ] K[ ] CreateIpamPrefixListResolverTarget
- W[x] S[ ] M[ ] F[ ] K[ ] CreateIpamResourceDiscovery
- W[x] S[ ] M[ ] F[ ] K[ ] CreateIpamScope
- W[x] S[ ] M[x] F[ ] K[x] CreateKeyPair
- W[x] S[ ] M[x] F[ ] K[ ] CreateLaunchTemplate
- W[x] S[ ] M[ ] F[ ] K[ ] CreateLaunchTemplateVersion
- W[x] S[ ] M[ ] F[ ] K[ ] CreateLocalGatewayRoute
- W[x] S[ ] M[ ] F[ ] K[ ] CreateLocalGatewayRouteTable
- W[x] S[ ] M[ ] F[ ] K[ ] CreateLocalGatewayRouteTableVirtualInterfaceGroupAssociation
- W[x] S[ ] M[ ] F[ ] K[ ] CreateLocalGatewayRouteTableVpcAssociation
- W[x] S[ ] M[ ] F[ ] K[ ] CreateLocalGatewayVirtualInterface
- W[x] S[ ] M[ ] F[ ] K[ ] CreateLocalGatewayVirtualInterfaceGroup
- W[x] S[ ] M[ ] F[ ] K[ ] CreateMacSystemIntegrityProtectionModificationTask
- W[x] S[ ] M[x] F[ ] K[ ] CreateManagedPrefixList
- W[x] S[ ] M[x] F[ ] K[x] CreateNatGateway
- W[x] S[ ] M[x] F[ ] K[ ] CreateNetworkAcl
- W[x] S[ ] M[x] F[ ] K[ ] CreateNetworkAclEntry
- W[x] S[ ] M[ ] F[ ] K[ ] CreateNetworkInsightsAccessScope
- W[x] S[ ] M[ ] F[ ] K[ ] CreateNetworkInsightsPath
- W[x] S[ ] M[x] F[ ] K[ ] CreateNetworkInterface
- W[x] S[ ] M[ ] F[ ] K[ ] CreateNetworkInterfacePermission
- W[x] S[ ] M[ ] F[ ] K[ ] CreatePlacementGroup
- W[x] S[ ] M[ ] F[ ] K[ ] CreatePublicIpv4Pool
- W[x] S[ ] M[ ] F[ ] K[ ] CreateReplaceRootVolumeTask
- W[x] S[ ] M[ ] F[ ] K[ ] CreateReservedInstancesListing
- W[x] S[ ] M[ ] F[ ] K[ ] CreateRestoreImageTask
- W[x] S[ ] M[x] F[ ] K[x] CreateRoute
- W[x] S[ ] M[ ] F[ ] K[ ] CreateRouteServer
- W[x] S[ ] M[ ] F[ ] K[ ] CreateRouteServerEndpoint
- W[x] S[ ] M[ ] F[ ] K[ ] CreateRouteServerPeer
- W[x] S[ ] M[x] F[ ] K[x] CreateRouteTable
- W[x] S[ ] M[ ] F[ ] K[ ] CreateSecondaryNetwork
- W[x] S[ ] M[ ] F[ ] K[ ] CreateSecondarySubnet
- W[x] S[ ] M[x] F[ ] K[x] CreateSecurityGroup
- W[x] S[ ] M[x] F[ ] K[ ] CreateSnapshot
- W[x] S[ ] M[x] F[ ] K[ ] CreateSnapshots
- W[x] S[ ] M[ ] F[ ] K[ ] CreateSpotDatafeedSubscription
- W[x] S[ ] M[ ] F[ ] K[ ] CreateStoreImageTask
- W[x] S[ ] M[x] F[ ] K[x] CreateSubnet
- W[x] S[ ] M[x] F[ ] K[ ] CreateSubnetCidrReservation
- W[x] S[ ] M[x] F[ ] K[x] CreateTags
- W[x] S[ ] M[x] F[ ] K[ ] CreateTrafficMirrorFilter
- W[x] S[ ] M[ ] F[ ] K[ ] CreateTrafficMirrorFilterRule
- W[x] S[ ] M[ ] F[ ] K[ ] CreateTrafficMirrorSession
- W[x] S[ ] M[x] F[ ] K[ ] CreateTrafficMirrorTarget
- W[x] S[ ] M[x] F[ ] K[ ] CreateTransitGateway
- W[x] S[ ] M[ ] F[ ] K[ ] CreateTransitGatewayConnect
- W[x] S[ ] M[ ] F[ ] K[ ] CreateTransitGatewayConnectPeer
- W[x] S[ ] M[ ] F[ ] K[ ] CreateTransitGatewayMeteringPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] CreateTransitGatewayMeteringPolicyEntry
- W[x] S[ ] M[ ] F[ ] K[ ] CreateTransitGatewayMulticastDomain
- W[x] S[ ] M[x] F[ ] K[ ] CreateTransitGatewayPeeringAttachment
- W[x] S[ ] M[ ] F[ ] K[ ] CreateTransitGatewayPolicyTable
- W[x] S[ ] M[ ] F[ ] K[ ] CreateTransitGatewayPrefixListReference
- W[x] S[ ] M[x] F[ ] K[ ] CreateTransitGatewayRoute
- W[x] S[ ] M[x] F[ ] K[ ] CreateTransitGatewayRouteTable
- W[x] S[ ] M[ ] F[ ] K[ ] CreateTransitGatewayRouteTableAnnouncement
- W[x] S[ ] M[x] F[ ] K[ ] CreateTransitGatewayVpcAttachment
- W[x] S[ ] M[ ] F[ ] K[ ] CreateVerifiedAccessEndpoint
- W[x] S[ ] M[ ] F[ ] K[ ] CreateVerifiedAccessGroup
- W[x] S[ ] M[ ] F[ ] K[ ] CreateVerifiedAccessInstance
- W[x] S[ ] M[ ] F[ ] K[ ] CreateVerifiedAccessTrustProvider
- W[x] S[ ] M[x] F[ ] K[ ] CreateVolume
- W[x] S[ ] M[x] F[ ] K[x] CreateVpc
- W[x] S[ ] M[ ] F[ ] K[ ] CreateVpcBlockPublicAccessExclusion
- W[x] S[ ] M[ ] F[ ] K[ ] CreateVpcEncryptionControl
- W[x] S[ ] M[x] F[ ] K[ ] CreateVpcEndpoint
- W[x] S[ ] M[ ] F[ ] K[ ] CreateVpcEndpointConnectionNotification
- W[x] S[ ] M[x] F[ ] K[ ] CreateVpcEndpointServiceConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] CreateVpcPeeringConnection
- W[x] S[ ] M[ ] F[ ] K[ ] CreateVpnConcentrator
- W[x] S[ ] M[x] F[ ] K[ ] CreateVpnConnection
- W[x] S[ ] M[ ] F[ ] K[ ] CreateVpnConnectionRoute
- W[x] S[ ] M[x] F[ ] K[ ] CreateVpnGateway
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteCapacityManagerDataExport
- W[x] S[ ] M[x] F[ ] K[ ] DeleteCarrierGateway
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteClientVpnEndpoint
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteClientVpnRoute
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteCoipCidr
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteCoipPool
- W[x] S[ ] M[x] F[ ] K[ ] DeleteCustomerGateway
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteDhcpOptions
- W[x] S[ ] M[x] F[ ] K[ ] DeleteEgressOnlyInternetGateway
- W[x] S[ ] M[x] F[ ] K[ ] DeleteFleets
- W[x] S[ ] M[x] F[ ] K[ ] DeleteFlowLogs
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteFpgaImage
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteImageUsageReport
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteInstanceConnectEndpoint
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteInstanceEventWindow
- W[x] S[ ] M[x] F[ ] K[x] DeleteInternetGateway
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteIpam
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteIpamExternalResourceVerificationToken
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteIpamPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteIpamPool
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteIpamPrefixListResolver
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteIpamPrefixListResolverTarget
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteIpamResourceDiscovery
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteIpamScope
- W[x] S[ ] M[x] F[ ] K[x] DeleteKeyPair
- W[x] S[ ] M[x] F[ ] K[ ] DeleteLaunchTemplate
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteLaunchTemplateVersions
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteLocalGatewayRoute
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteLocalGatewayRouteTable
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteLocalGatewayRouteTableVirtualInterfaceGroupAssociation
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteLocalGatewayRouteTableVpcAssociation
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteLocalGatewayVirtualInterface
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteLocalGatewayVirtualInterfaceGroup
- W[x] S[ ] M[x] F[ ] K[ ] DeleteManagedPrefixList
- W[x] S[ ] M[x] F[ ] K[ ] DeleteNatGateway
- W[x] S[ ] M[x] F[ ] K[ ] DeleteNetworkAcl
- W[x] S[ ] M[x] F[ ] K[ ] DeleteNetworkAclEntry
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteNetworkInsightsAccessScope
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteNetworkInsightsAccessScopeAnalysis
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteNetworkInsightsAnalysis
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteNetworkInsightsPath
- W[x] S[ ] M[x] F[ ] K[ ] DeleteNetworkInterface
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteNetworkInterfacePermission
- W[x] S[ ] M[ ] F[ ] K[ ] DeletePlacementGroup
- W[x] S[ ] M[ ] F[ ] K[ ] DeletePublicIpv4Pool
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteQueuedReservedInstances
- W[x] S[ ] M[x] F[ ] K[ ] DeleteRoute
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteRouteServer
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteRouteServerEndpoint
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteRouteServerPeer
- W[x] S[ ] M[x] F[ ] K[ ] DeleteRouteTable
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteSecondaryNetwork
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteSecondarySubnet
- W[x] S[ ] M[x] F[ ] K[x] DeleteSecurityGroup
- W[x] S[ ] M[x] F[ ] K[ ] DeleteSnapshot
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteSpotDatafeedSubscription
- W[x] S[ ] M[x] F[ ] K[x] DeleteSubnet
- W[x] S[ ] M[x] F[ ] K[ ] DeleteSubnetCidrReservation
- W[x] S[ ] M[x] F[ ] K[x] DeleteTags
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteTrafficMirrorFilter
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteTrafficMirrorFilterRule
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteTrafficMirrorSession
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteTrafficMirrorTarget
- W[x] S[ ] M[x] F[ ] K[ ] DeleteTransitGateway
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteTransitGatewayClientVpnAttachment
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteTransitGatewayConnect
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteTransitGatewayConnectPeer
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteTransitGatewayMeteringPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteTransitGatewayMeteringPolicyEntry
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteTransitGatewayMulticastDomain
- W[x] S[ ] M[x] F[ ] K[ ] DeleteTransitGatewayPeeringAttachment
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteTransitGatewayPolicyTable
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteTransitGatewayPrefixListReference
- W[x] S[ ] M[x] F[ ] K[ ] DeleteTransitGatewayRoute
- W[x] S[ ] M[x] F[ ] K[ ] DeleteTransitGatewayRouteTable
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteTransitGatewayRouteTableAnnouncement
- W[x] S[ ] M[x] F[ ] K[ ] DeleteTransitGatewayVpcAttachment
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteVerifiedAccessEndpoint
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteVerifiedAccessGroup
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteVerifiedAccessInstance
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteVerifiedAccessTrustProvider
- W[x] S[ ] M[x] F[ ] K[ ] DeleteVolume
- W[x] S[ ] M[x] F[ ] K[x] DeleteVpc
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteVpcBlockPublicAccessExclusion
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteVpcEncryptionControl
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteVpcEndpointConnectionNotifications
- W[x] S[ ] M[x] F[ ] K[ ] DeleteVpcEndpointServiceConfigurations
- W[x] S[ ] M[x] F[ ] K[ ] DeleteVpcEndpoints
- W[x] S[ ] M[x] F[ ] K[ ] DeleteVpcPeeringConnection
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteVpnConcentrator
- W[x] S[ ] M[x] F[ ] K[ ] DeleteVpnConnection
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteVpnConnectionRoute
- W[x] S[ ] M[x] F[ ] K[ ] DeleteVpnGateway
- W[x] S[ ] M[ ] F[ ] K[ ] DeprovisionByoipCidr
- W[x] S[ ] M[ ] F[ ] K[ ] DeprovisionIpamByoasn
- W[x] S[ ] M[ ] F[ ] K[ ] DeprovisionIpamPoolCidr
- W[x] S[ ] M[ ] F[ ] K[ ] DeprovisionPublicIpv4PoolCidr
- W[x] S[ ] M[x] F[ ] K[ ] DeregisterImage
- W[x] S[ ] M[ ] F[ ] K[ ] DeregisterInstanceEventNotificationAttributes
- W[x] S[ ] M[ ] F[ ] K[ ] DeregisterTransitGatewayMulticastGroupMembers
- W[x] S[ ] M[ ] F[ ] K[ ] DeregisterTransitGatewayMulticastGroupSources
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeAccountAttributes
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeAddressTransfers
- W[x] S[ ] M[x] F[ ] K[ ] DescribeAddresses
- W[x] S[ ] M[x] F[ ] K[ ] DescribeAddressesAttribute
- W[ ] S[x] M[ ] F[ ] K[ ] DescribeAggregateIdFormat
- W[x] S[ ] M[x] F[ ] K[ ] DescribeAvailabilityZones
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeAwsNetworkPerformanceMetricSubscriptions
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeBundleTasks
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeByoipCidrs
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeCapacityBlockExtensionHistory
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeCapacityBlockExtensionOfferings
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeCapacityBlockOfferings
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeCapacityBlockStatus
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeCapacityBlocks
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeCapacityManagerDataExports
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeCapacityReservationBillingRequests
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeCapacityReservationFleets
- W[ ] S[x] M[ ] F[ ] K[ ] DescribeCapacityReservationTopology
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeCapacityReservations
- W[x] S[ ] M[x] F[ ] K[ ] DescribeCarrierGateways
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeClassicLinkInstances
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeClientVpnAuthorizationRules
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeClientVpnConnections
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeClientVpnEndpoints
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeClientVpnRoutes
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeClientVpnTargetNetworks
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeCoipPools
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeConversionTasks
- W[x] S[ ] M[x] F[ ] K[ ] DescribeCustomerGateways
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeDeclarativePoliciesReports
- W[x] S[ ] M[x] F[ ] K[ ] DescribeDhcpOptions
- W[x] S[ ] M[x] F[ ] K[ ] DescribeEgressOnlyInternetGateways
- W[ ] S[x] M[ ] F[ ] K[ ] DescribeElasticGpus
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeExportImageTasks
- W[ ] S[x] M[ ] F[ ] K[ ] DescribeExportTasks
- W[ ] S[x] M[ ] F[ ] K[ ] DescribeFastLaunchImages
- W[ ] S[x] M[ ] F[ ] K[ ] DescribeFastSnapshotRestores
- W[ ] S[x] M[ ] F[ ] K[ ] DescribeFleetHistory
- W[x] S[ ] M[x] F[ ] K[ ] DescribeFleetInstances
- W[x] S[ ] M[x] F[ ] K[ ] DescribeFleets
- W[x] S[ ] M[x] F[ ] K[ ] DescribeFlowLogs
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeFpgaImageAttribute
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeFpgaImages
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeHostReservationOfferings
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeHostReservations
- W[x] S[ ] M[x] F[ ] K[ ] DescribeHosts
- W[x] S[ ] M[x] F[ ] K[ ] DescribeIamInstanceProfileAssociations
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeIdFormat
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeIdentityIdFormat
- W[x] S[ ] M[x] F[ ] K[ ] DescribeImageAttribute
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeImageReferences
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeImageUsageReportEntries
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeImageUsageReports
- W[x] S[ ] M[x] F[ ] K[ ] DescribeImages
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeImportImageTasks
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeImportSnapshotTasks
- W[x] S[ ] M[x] F[ ] K[ ] DescribeInstanceAttribute
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeInstanceConnectEndpoints
- W[x] S[ ] M[x] F[ ] K[ ] DescribeInstanceCreditSpecifications
- W[ ] S[x] M[ ] F[ ] K[ ] DescribeInstanceEventNotificationAttributes
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeInstanceEventWindows
- W[ ] S[x] M[ ] F[ ] K[ ] DescribeInstanceImageMetadata
- W[ ] S[x] M[ ] F[ ] K[ ] DescribeInstanceSqlHaHistoryStates
- W[ ] S[x] M[ ] F[ ] K[ ] DescribeInstanceSqlHaStates
- W[x] S[ ] M[x] F[ ] K[ ] DescribeInstanceStatus
- W[ ] S[x] M[ ] F[ ] K[ ] DescribeInstanceTopology
- W[x] S[ ] M[x] F[ ] K[ ] DescribeInstanceTypeOfferings
- W[x] S[ ] M[x] F[ ] K[ ] DescribeInstanceTypes
- W[x] S[ ] M[x] F[ ] K[x] DescribeInstances
- W[x] S[ ] M[x] F[ ] K[x] DescribeInternetGateways
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeIpamByoasn
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeIpamExternalResourceVerificationTokens
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeIpamPolicies
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeIpamPools
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeIpamPrefixListResolverTargets
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeIpamPrefixListResolvers
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeIpamResourceDiscoveries
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeIpamResourceDiscoveryAssociations
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeIpamScopes
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeIpams
- W[ ] S[x] M[ ] F[ ] K[ ] DescribeIpv6Pools
- W[x] S[ ] M[x] F[ ] K[x] DescribeKeyPairs
- W[x] S[ ] M[x] F[ ] K[ ] DescribeLaunchTemplateVersions
- W[x] S[ ] M[x] F[ ] K[ ] DescribeLaunchTemplates
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeLocalGatewayRouteTableVirtualInterfaceGroupAssociations
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeLocalGatewayRouteTableVpcAssociations
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeLocalGatewayRouteTables
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeLocalGatewayVirtualInterfaceGroups
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeLocalGatewayVirtualInterfaces
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeLocalGateways
- W[ ] S[x] M[ ] F[ ] K[ ] DescribeLockedSnapshots
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeMacHosts
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeMacModificationTasks
- W[x] S[ ] M[x] F[ ] K[ ] DescribeManagedPrefixLists
- W[ ] S[x] M[ ] F[ ] K[ ] DescribeMovingAddresses
- W[x] S[ ] M[x] F[ ] K[x] DescribeNatGateways
- W[x] S[ ] M[x] F[ ] K[ ] DescribeNetworkAcls
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeNetworkInsightsAccessScopeAnalyses
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeNetworkInsightsAccessScopes
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeNetworkInsightsAnalyses
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeNetworkInsightsPaths
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeNetworkInterfaceAttribute
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeNetworkInterfacePermissions
- W[x] S[ ] M[x] F[ ] K[x] DescribeNetworkInterfaces
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeOutpostLags
- W[x] S[ ] M[ ] F[ ] K[ ] DescribePlacementGroups
- W[x] S[ ] M[ ] F[ ] K[ ] DescribePrefixLists
- W[x] S[ ] M[ ] F[ ] K[ ] DescribePrincipalIdFormat
- W[x] S[ ] M[ ] F[ ] K[ ] DescribePublicIpv4Pools
- W[x] S[ ] M[x] F[ ] K[ ] DescribeRegions
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeReplaceRootVolumeTasks
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeReservedInstances
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeReservedInstancesListings
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeReservedInstancesModifications
- W[x] S[ ] M[x] F[ ] K[ ] DescribeReservedInstancesOfferings
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeRouteServerEndpoints
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeRouteServerPeers
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeRouteServers
- W[x] S[ ] M[x] F[ ] K[x] DescribeRouteTables
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeScheduledInstanceAvailability
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeScheduledInstances
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeSecondaryInterfaces
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeSecondaryNetworks
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeSecondarySubnets
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeSecurityGroupReferences
- W[x] S[ ] M[x] F[ ] K[ ] DescribeSecurityGroupRules
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeSecurityGroupVpcAssociations
- W[x] S[ ] M[x] F[ ] K[x] DescribeSecurityGroups
- W[ ] S[x] M[ ] F[ ] K[ ] DescribeServiceLinkVirtualInterfaces
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeSnapshotAttribute
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeSnapshotTierStatus
- W[x] S[ ] M[x] F[ ] K[ ] DescribeSnapshots
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeSpotDatafeedSubscription
- W[x] S[ ] M[x] F[ ] K[ ] DescribeSpotFleetInstances
- W[ ] S[x] M[ ] F[ ] K[ ] DescribeSpotFleetRequestHistory
- W[x] S[ ] M[x] F[ ] K[ ] DescribeSpotFleetRequests
- W[x] S[ ] M[x] F[ ] K[ ] DescribeSpotInstanceRequests
- W[ ] S[x] M[x] F[ ] K[ ] DescribeSpotPriceHistory
- W[ ] S[x] M[ ] F[ ] K[ ] DescribeStaleSecurityGroups
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeStoreImageTasks
- W[x] S[ ] M[x] F[ ] K[x] DescribeSubnets
- W[x] S[ ] M[x] F[ ] K[x] DescribeTags
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeTrafficMirrorFilterRules
- W[x] S[ ] M[x] F[ ] K[ ] DescribeTrafficMirrorFilters
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeTrafficMirrorSessions
- W[x] S[ ] M[x] F[ ] K[ ] DescribeTrafficMirrorTargets
- W[x] S[ ] M[x] F[ ] K[ ] DescribeTransitGatewayAttachments
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeTransitGatewayConnectPeers
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeTransitGatewayConnects
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeTransitGatewayMeteringPolicies
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeTransitGatewayMulticastDomains
- W[x] S[ ] M[x] F[ ] K[ ] DescribeTransitGatewayPeeringAttachments
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeTransitGatewayPolicyTables
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeTransitGatewayRouteTableAnnouncements
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeTransitGatewayRouteTables
- W[x] S[ ] M[x] F[ ] K[ ] DescribeTransitGatewayVpcAttachments
- W[x] S[ ] M[x] F[ ] K[ ] DescribeTransitGateways
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeTrunkInterfaceAssociations
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeVerifiedAccessEndpoints
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeVerifiedAccessGroups
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeVerifiedAccessInstanceLoggingConfigurations
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeVerifiedAccessInstances
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeVerifiedAccessTrustProviders
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeVolumeAttribute
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeVolumeStatus
- W[x] S[ ] M[x] F[ ] K[ ] DescribeVolumes
- W[x] S[ ] M[x] F[ ] K[ ] DescribeVolumesModifications
- W[x] S[ ] M[x] F[ ] K[x] DescribeVpcAttribute
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeVpcBlockPublicAccessExclusions
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeVpcBlockPublicAccessOptions
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeVpcClassicLink
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeVpcClassicLinkDnsSupport
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeVpcEncryptionControls
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeVpcEndpointAssociations
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeVpcEndpointConnectionNotifications
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeVpcEndpointConnections
- W[x] S[ ] M[x] F[ ] K[ ] DescribeVpcEndpointServiceConfigurations
- W[x] S[ ] M[x] F[ ] K[ ] DescribeVpcEndpointServicePermissions
- W[x] S[ ] M[x] F[ ] K[ ] DescribeVpcEndpointServices
- W[x] S[ ] M[x] F[ ] K[ ] DescribeVpcEndpoints
- W[x] S[ ] M[x] F[ ] K[ ] DescribeVpcPeeringConnections
- W[x] S[ ] M[x] F[ ] K[x] DescribeVpcs
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeVpnConcentrators
- W[x] S[ ] M[x] F[ ] K[ ] DescribeVpnConnections
- W[x] S[ ] M[x] F[ ] K[ ] DescribeVpnGateways
- W[x] S[ ] M[ ] F[ ] K[ ] DetachClassicLinkVpc
- W[x] S[ ] M[x] F[ ] K[x] DetachInternetGateway
- W[x] S[ ] M[x] F[ ] K[ ] DetachNetworkInterface
- W[x] S[ ] M[ ] F[ ] K[ ] DetachVerifiedAccessTrustProvider
- W[x] S[ ] M[x] F[ ] K[ ] DetachVolume
- W[x] S[ ] M[x] F[ ] K[ ] DetachVpnGateway
- W[x] S[ ] M[ ] F[ ] K[ ] DisableAddressTransfer
- W[x] S[ ] M[ ] F[ ] K[ ] DisableAllowedImagesSettings
- W[x] S[ ] M[ ] F[ ] K[ ] DisableAwsNetworkPerformanceMetricSubscription
- W[x] S[ ] M[ ] F[ ] K[ ] DisableCapacityManager
- W[x] S[ ] M[x] F[ ] K[ ] DisableEbsEncryptionByDefault
- W[x] S[ ] M[ ] F[ ] K[ ] DisableFastLaunch
- W[x] S[ ] M[ ] F[ ] K[ ] DisableFastSnapshotRestores
- W[x] S[ ] M[ ] F[ ] K[ ] DisableImage
- W[x] S[ ] M[ ] F[ ] K[ ] DisableImageBlockPublicAccess
- W[x] S[ ] M[ ] F[ ] K[ ] DisableImageDeprecation
- W[x] S[ ] M[ ] F[ ] K[ ] DisableImageDeregistrationProtection
- W[ ] S[x] M[ ] F[ ] K[ ] DisableInstanceSqlHaStandbyDetections
- W[ ] S[x] M[ ] F[ ] K[ ] DisableIpamOrganizationAdminAccount
- W[x] S[ ] M[ ] F[ ] K[ ] DisableIpamPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] DisableRouteServerPropagation
- W[x] S[ ] M[ ] F[ ] K[ ] DisableSerialConsoleAccess
- W[ ] S[x] M[ ] F[ ] K[ ] DisableSnapshotBlockPublicAccess
- W[x] S[ ] M[x] F[ ] K[ ] DisableTransitGatewayRouteTablePropagation
- W[x] S[ ] M[ ] F[ ] K[ ] DisableVgwRoutePropagation
- W[x] S[ ] M[x] F[ ] K[ ] DisableVpcClassicLink
- W[x] S[ ] M[x] F[ ] K[ ] DisableVpcClassicLinkDnsSupport
- W[x] S[ ] M[x] F[ ] K[ ] DisassociateAddress
- W[x] S[ ] M[ ] F[ ] K[ ] DisassociateCapacityReservationBillingOwner
- W[x] S[ ] M[ ] F[ ] K[ ] DisassociateClientVpnTargetNetwork
- W[x] S[ ] M[ ] F[ ] K[ ] DisassociateEnclaveCertificateIamRole
- W[x] S[ ] M[x] F[ ] K[ ] DisassociateIamInstanceProfile
- W[x] S[ ] M[ ] F[ ] K[ ] DisassociateInstanceEventWindow
- W[x] S[ ] M[ ] F[ ] K[ ] DisassociateIpamByoasn
- W[x] S[ ] M[ ] F[ ] K[ ] DisassociateIpamResourceDiscovery
- W[x] S[ ] M[ ] F[ ] K[ ] DisassociateNatGatewayAddress
- W[x] S[ ] M[ ] F[ ] K[ ] DisassociateRouteServer
- W[x] S[ ] M[x] F[ ] K[ ] DisassociateRouteTable
- W[x] S[ ] M[ ] F[ ] K[ ] DisassociateSecurityGroupVpc
- W[x] S[ ] M[x] F[ ] K[ ] DisassociateSubnetCidrBlock
- W[x] S[ ] M[ ] F[ ] K[ ] DisassociateTransitGatewayMulticastDomain
- W[x] S[ ] M[ ] F[ ] K[ ] DisassociateTransitGatewayPolicyTable
- W[x] S[ ] M[x] F[ ] K[ ] DisassociateTransitGatewayRouteTable
- W[x] S[ ] M[ ] F[ ] K[ ] DisassociateTrunkInterface
- W[x] S[ ] M[x] F[ ] K[ ] DisassociateVpcCidrBlock
- W[x] S[ ] M[ ] F[ ] K[ ] EnableAddressTransfer
- W[x] S[ ] M[ ] F[ ] K[ ] EnableAllowedImagesSettings
- W[x] S[ ] M[ ] F[ ] K[ ] EnableAwsNetworkPerformanceMetricSubscription
- W[x] S[ ] M[ ] F[ ] K[ ] EnableCapacityManager
- W[x] S[ ] M[x] F[ ] K[ ] EnableEbsEncryptionByDefault
- W[x] S[ ] M[ ] F[ ] K[ ] EnableFastLaunch
- W[x] S[ ] M[ ] F[ ] K[ ] EnableFastSnapshotRestores
- W[x] S[ ] M[ ] F[ ] K[ ] EnableImage
- W[x] S[ ] M[ ] F[ ] K[ ] EnableImageBlockPublicAccess
- W[x] S[ ] M[ ] F[ ] K[ ] EnableImageDeprecation
- W[x] S[ ] M[ ] F[ ] K[ ] EnableImageDeregistrationProtection
- W[ ] S[x] M[ ] F[ ] K[ ] EnableInstanceSqlHaStandbyDetections
- W[ ] S[x] M[ ] F[ ] K[ ] EnableIpamOrganizationAdminAccount
- W[x] S[ ] M[ ] F[ ] K[ ] EnableIpamPolicy
- W[ ] S[x] M[ ] F[ ] K[ ] EnableReachabilityAnalyzerOrganizationSharing
- W[x] S[ ] M[ ] F[ ] K[ ] EnableRouteServerPropagation
- W[x] S[ ] M[ ] F[ ] K[ ] EnableSerialConsoleAccess
- W[ ] S[x] M[ ] F[ ] K[ ] EnableSnapshotBlockPublicAccess
- W[x] S[ ] M[x] F[ ] K[ ] EnableTransitGatewayRouteTablePropagation
- W[x] S[ ] M[ ] F[ ] K[ ] EnableVgwRoutePropagation
- W[x] S[ ] M[ ] F[ ] K[ ] EnableVolumeIO
- W[x] S[ ] M[x] F[ ] K[ ] EnableVpcClassicLink
- W[x] S[ ] M[x] F[ ] K[ ] EnableVpcClassicLinkDnsSupport
- W[x] S[ ] M[ ] F[ ] K[ ] ExportClientVpnClientCertificateRevocationList
- W[x] S[ ] M[ ] F[ ] K[ ] ExportClientVpnClientConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] ExportImage
- W[x] S[ ] M[ ] F[ ] K[ ] ExportTransitGatewayRoutes
- W[x] S[ ] M[ ] F[ ] K[ ] ExportVerifiedAccessInstanceClientConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] GetActiveVpnTunnelStatus
- W[x] S[ ] M[ ] F[ ] K[ ] GetAllowedImagesSettings
- W[x] S[ ] M[ ] F[ ] K[ ] GetAssociatedEnclaveCertificateIamRoles
- W[x] S[ ] M[ ] F[ ] K[ ] GetAssociatedIpv6PoolCidrs
- W[ ] S[x] M[ ] F[ ] K[ ] GetAwsNetworkPerformanceData
- W[ ] S[x] M[ ] F[ ] K[ ] GetCapacityManagerAttributes
- W[ ] S[x] M[ ] F[ ] K[ ] GetCapacityManagerMetricData
- W[ ] S[x] M[ ] F[ ] K[ ] GetCapacityManagerMetricDimensions
- W[ ] S[ ] M[ ] F[ ] K[ ] GetCapacityManagerMonitoredTagKeys
- W[x] S[ ] M[ ] F[ ] K[ ] GetCapacityReservationUsage
- W[x] S[ ] M[ ] F[ ] K[ ] GetCoipPoolUsage
- W[x] S[ ] M[ ] F[ ] K[ ] GetConsoleOutput
- W[ ] S[x] M[ ] F[ ] K[ ] GetConsoleScreenshot
- W[x] S[ ] M[ ] F[ ] K[ ] GetDeclarativePoliciesReportSummary
- W[x] S[ ] M[ ] F[ ] K[ ] GetDefaultCreditSpecification
- W[x] S[ ] M[ ] F[ ] K[ ] GetEbsDefaultKmsKeyId
- W[x] S[ ] M[x] F[ ] K[ ] GetEbsEncryptionByDefault
- W[x] S[ ] M[ ] F[ ] K[ ] GetEnabledIpamPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] GetFlowLogsIntegrationTemplate
- W[x] S[ ] M[ ] F[ ] K[ ] GetGroupsForCapacityReservation
- W[x] S[ ] M[ ] F[ ] K[ ] GetHostReservationPurchasePreview
- W[ ] S[x] M[ ] F[ ] K[ ] GetImageAncestry
- W[ ] S[x] M[ ] F[ ] K[ ] GetImageBlockPublicAccessState
- W[x] S[ ] M[ ] F[ ] K[ ] GetInstanceMetadataDefaults
- W[ ] S[x] M[ ] F[ ] K[ ] GetInstanceTpmEkPub
- W[x] S[ ] M[ ] F[ ] K[ ] GetInstanceTypesFromInstanceRequirements
- W[ ] S[x] M[x] F[ ] K[ ] GetInstanceUefiData
- W[x] S[ ] M[ ] F[ ] K[ ] GetIpamAddressHistory
- W[x] S[ ] M[ ] F[ ] K[ ] GetIpamDiscoveredAccounts
- W[x] S[ ] M[ ] F[ ] K[ ] GetIpamDiscoveredPublicAddresses
- W[x] S[ ] M[ ] F[ ] K[ ] GetIpamDiscoveredResourceCidrs
- W[x] S[ ] M[ ] F[ ] K[ ] GetIpamPolicyAllocationRules
- W[ ] S[x] M[ ] F[ ] K[ ] GetIpamPolicyOrganizationTargets
- W[x] S[ ] M[ ] F[ ] K[ ] GetIpamPoolAllocations
- W[x] S[ ] M[ ] F[ ] K[ ] GetIpamPoolCidrs
- W[x] S[ ] M[ ] F[ ] K[ ] GetIpamPrefixListResolverRules
- W[x] S[ ] M[ ] F[ ] K[ ] GetIpamPrefixListResolverVersionEntries
- W[x] S[ ] M[ ] F[ ] K[ ] GetIpamPrefixListResolverVersions
- W[x] S[ ] M[ ] F[ ] K[ ] GetIpamResourceCidrs
- W[x] S[ ] M[x] F[ ] K[ ] GetLaunchTemplateData
- W[x] S[ ] M[ ] F[ ] K[ ] GetManagedPrefixListAssociations
- W[x] S[ ] M[x] F[ ] K[ ] GetManagedPrefixListEntries
- W[ ] S[ ] M[ ] F[ ] K[ ] GetManagedResourceVisibility
- W[x] S[ ] M[ ] F[ ] K[ ] GetNetworkInsightsAccessScopeAnalysisFindings
- W[x] S[ ] M[ ] F[ ] K[ ] GetNetworkInsightsAccessScopeContent
- W[ ] S[x] M[x] F[ ] K[ ] GetPasswordData
- W[x] S[ ] M[ ] F[ ] K[ ] GetReservedInstancesExchangeQuote
- W[x] S[ ] M[ ] F[ ] K[ ] GetRouteServerAssociations
- W[x] S[ ] M[ ] F[ ] K[ ] GetRouteServerPropagations
- W[ ] S[x] M[ ] F[ ] K[ ] GetRouteServerRoutingDatabase
- W[x] S[ ] M[ ] F[ ] K[ ] GetSecurityGroupsForVpc
- W[ ] S[x] M[ ] F[ ] K[ ] GetSerialConsoleAccessStatus
- W[ ] S[x] M[ ] F[ ] K[ ] GetSnapshotBlockPublicAccessState
- W[ ] S[x] M[ ] F[ ] K[ ] GetSpotPlacementScores
- W[x] S[ ] M[x] F[ ] K[ ] GetSubnetCidrReservations
- W[x] S[ ] M[ ] F[ ] K[ ] GetTransitGatewayAttachmentPropagations
- W[x] S[ ] M[ ] F[ ] K[ ] GetTransitGatewayMeteringPolicyEntries
- W[x] S[ ] M[ ] F[ ] K[ ] GetTransitGatewayMulticastDomainAssociations
- W[x] S[ ] M[ ] F[ ] K[ ] GetTransitGatewayPolicyTableAssociations
- W[x] S[ ] M[ ] F[ ] K[ ] GetTransitGatewayPolicyTableEntries
- W[x] S[ ] M[ ] F[ ] K[ ] GetTransitGatewayPrefixListReferences
- W[x] S[ ] M[x] F[ ] K[ ] GetTransitGatewayRouteTableAssociations
- W[x] S[ ] M[x] F[ ] K[ ] GetTransitGatewayRouteTablePropagations
- W[x] S[ ] M[ ] F[ ] K[ ] GetVerifiedAccessEndpointPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] GetVerifiedAccessEndpointTargets
- W[x] S[ ] M[ ] F[ ] K[ ] GetVerifiedAccessGroupPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] GetVpcResourcesBlockingEncryptionEnforcement
- W[x] S[ ] M[ ] F[ ] K[ ] GetVpnConnectionDeviceSampleConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] GetVpnConnectionDeviceTypes
- W[x] S[ ] M[ ] F[ ] K[ ] GetVpnTunnelReplacementStatus
- W[x] S[ ] M[ ] F[ ] K[ ] ImportClientVpnClientCertificateRevocationList
- W[x] S[ ] M[ ] F[ ] K[ ] ImportImage
- W[x] S[ ] M[ ] F[ ] K[ ] ImportInstance
- W[x] S[ ] M[x] F[ ] K[ ] ImportKeyPair
- W[x] S[ ] M[ ] F[ ] K[ ] ImportSnapshot
- W[x] S[ ] M[ ] F[ ] K[ ] ImportVolume
- W[x] S[ ] M[ ] F[ ] K[ ] ListImagesInRecycleBin
- W[x] S[ ] M[ ] F[ ] K[ ] ListSnapshotsInRecycleBin
- W[x] S[ ] M[ ] F[ ] K[ ] ListVolumesInRecycleBin
- W[x] S[ ] M[ ] F[ ] K[ ] LockSnapshot
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyAddressAttribute
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyAvailabilityZoneGroup
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyCapacityReservation
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyCapacityReservationFleet
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyClientVpnEndpoint
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyDefaultCreditSpecification
- W[x] S[ ] M[x] F[ ] K[ ] ModifyEbsDefaultKmsKeyId
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyFleet
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyFpgaImageAttribute
- W[x] S[ ] M[x] F[ ] K[ ] ModifyHosts
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyIdFormat
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyIdentityIdFormat
- W[x] S[ ] M[x] F[ ] K[ ] ModifyImageAttribute
- W[x] S[ ] M[x] F[ ] K[ ] ModifyInstanceAttribute
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyInstanceCapacityReservationAttributes
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyInstanceConnectEndpoint
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyInstanceCpuOptions
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyInstanceCreditSpecification
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyInstanceEventStartTime
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyInstanceEventWindow
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyInstanceMaintenanceOptions
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyInstanceMetadataDefaults
- W[x] S[ ] M[x] F[ ] K[ ] ModifyInstanceMetadataOptions
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyInstanceNetworkPerformanceOptions
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyInstancePlacement
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyIpam
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyIpamPolicyAllocationRules
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyIpamPool
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyIpamPrefixListResolver
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyIpamPrefixListResolverTarget
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyIpamResourceCidr
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyIpamResourceDiscovery
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyIpamScope
- W[x] S[ ] M[x] F[ ] K[ ] ModifyLaunchTemplate
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyLocalGatewayRoute
- W[x] S[ ] M[x] F[ ] K[ ] ModifyManagedPrefixList
- W[ ] S[ ] M[ ] F[ ] K[ ] ModifyManagedResourceVisibility
- W[x] S[ ] M[x] F[ ] K[ ] ModifyNetworkInterfaceAttribute
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyPrivateDnsNameOptions
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyPublicIpDnsNameOptions
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyReservedInstances
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyRouteServer
- W[x] S[ ] M[x] F[ ] K[ ] ModifySecurityGroupRules
- W[x] S[ ] M[ ] F[ ] K[ ] ModifySnapshotAttribute
- W[x] S[ ] M[ ] F[ ] K[ ] ModifySnapshotTier
- W[x] S[ ] M[x] F[ ] K[ ] ModifySpotFleetRequest
- W[x] S[ ] M[x] F[ ] K[x] ModifySubnetAttribute
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyTrafficMirrorFilterNetworkServices
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyTrafficMirrorFilterRule
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyTrafficMirrorSession
- W[x] S[ ] M[x] F[ ] K[ ] ModifyTransitGateway
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyTransitGatewayMeteringPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyTransitGatewayPrefixListReference
- W[x] S[ ] M[x] F[ ] K[ ] ModifyTransitGatewayVpcAttachment
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyVerifiedAccessEndpoint
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyVerifiedAccessEndpointPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyVerifiedAccessGroup
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyVerifiedAccessGroupPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyVerifiedAccessInstance
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyVerifiedAccessInstanceLoggingConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyVerifiedAccessTrustProvider
- W[x] S[ ] M[x] F[ ] K[ ] ModifyVolume
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyVolumeAttribute
- W[x] S[ ] M[x] F[ ] K[x] ModifyVpcAttribute
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyVpcBlockPublicAccessExclusion
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyVpcBlockPublicAccessOptions
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyVpcEncryptionControl
- W[x] S[ ] M[x] F[ ] K[ ] ModifyVpcEndpoint
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyVpcEndpointConnectionNotification
- W[x] S[ ] M[x] F[ ] K[ ] ModifyVpcEndpointServiceConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyVpcEndpointServicePayerResponsibility
- W[x] S[ ] M[x] F[ ] K[ ] ModifyVpcEndpointServicePermissions
- W[x] S[ ] M[x] F[ ] K[ ] ModifyVpcPeeringConnectionOptions
- W[x] S[ ] M[x] F[ ] K[ ] ModifyVpcTenancy
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyVpnConnection
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyVpnConnectionOptions
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyVpnTunnelCertificate
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyVpnTunnelOptions
- W[ ] S[x] M[ ] F[ ] K[ ] MonitorInstances
- W[x] S[ ] M[ ] F[ ] K[ ] MoveAddressToVpc
- W[x] S[ ] M[ ] F[ ] K[ ] MoveByoipCidrToIpam
- W[x] S[ ] M[ ] F[ ] K[ ] MoveCapacityReservationInstances
- W[x] S[ ] M[ ] F[ ] K[ ] ProvisionByoipCidr
- W[x] S[ ] M[ ] F[ ] K[ ] ProvisionIpamByoasn
- W[x] S[ ] M[ ] F[ ] K[ ] ProvisionIpamPoolCidr
- W[x] S[ ] M[ ] F[ ] K[ ] ProvisionPublicIpv4PoolCidr
- W[x] S[ ] M[ ] F[ ] K[ ] PurchaseCapacityBlock
- W[x] S[ ] M[ ] F[ ] K[ ] PurchaseCapacityBlockExtension
- W[x] S[ ] M[ ] F[ ] K[ ] PurchaseHostReservation
- W[x] S[ ] M[ ] F[ ] K[ ] PurchaseReservedInstancesOffering
- W[x] S[ ] M[ ] F[ ] K[ ] PurchaseScheduledInstances
- W[x] S[ ] M[x] F[ ] K[ ] RebootInstances
- W[x] S[ ] M[x] F[ ] K[ ] RegisterImage
- W[x] S[ ] M[ ] F[ ] K[ ] RegisterInstanceEventNotificationAttributes
- W[x] S[ ] M[ ] F[ ] K[ ] RegisterTransitGatewayMulticastGroupMembers
- W[x] S[ ] M[ ] F[ ] K[ ] RegisterTransitGatewayMulticastGroupSources
- W[x] S[ ] M[ ] F[ ] K[ ] RejectCapacityReservationBillingOwnership
- W[ ] S[ ] M[ ] F[ ] K[ ] RejectTransitGatewayClientVpnAttachment
- W[x] S[ ] M[ ] F[ ] K[ ] RejectTransitGatewayMulticastDomainAssociations
- W[x] S[ ] M[x] F[ ] K[ ] RejectTransitGatewayPeeringAttachment
- W[x] S[ ] M[ ] F[ ] K[ ] RejectTransitGatewayVpcAttachment
- W[x] S[ ] M[ ] F[ ] K[ ] RejectVpcEndpointConnections
- W[x] S[ ] M[x] F[ ] K[ ] RejectVpcPeeringConnection
- W[x] S[ ] M[x] F[ ] K[ ] ReleaseAddress
- W[x] S[ ] M[x] F[ ] K[ ] ReleaseHosts
- W[x] S[ ] M[ ] F[ ] K[ ] ReleaseIpamPoolAllocation
- W[x] S[ ] M[x] F[ ] K[ ] ReplaceIamInstanceProfileAssociation
- W[x] S[ ] M[ ] F[ ] K[ ] ReplaceImageCriteriaInAllowedImagesSettings
- W[x] S[ ] M[x] F[ ] K[ ] ReplaceNetworkAclAssociation
- W[x] S[ ] M[x] F[ ] K[ ] ReplaceNetworkAclEntry
- W[x] S[ ] M[x] F[ ] K[ ] ReplaceRoute
- W[x] S[ ] M[x] F[ ] K[ ] ReplaceRouteTableAssociation
- W[x] S[ ] M[ ] F[ ] K[ ] ReplaceTransitGatewayRoute
- W[x] S[ ] M[ ] F[ ] K[ ] ReplaceVpnTunnel
- W[x] S[ ] M[ ] F[ ] K[ ] ReportInstanceStatus
- W[x] S[ ] M[x] F[ ] K[ ] RequestSpotFleet
- W[x] S[ ] M[x] F[ ] K[ ] RequestSpotInstances
- W[x] S[ ] M[ ] F[ ] K[ ] ResetAddressAttribute
- W[x] S[ ] M[ ] F[ ] K[ ] ResetEbsDefaultKmsKeyId
- W[x] S[ ] M[ ] F[ ] K[ ] ResetFpgaImageAttribute
- W[x] S[ ] M[ ] F[ ] K[ ] ResetImageAttribute
- W[x] S[ ] M[ ] F[ ] K[ ] ResetInstanceAttribute
- W[x] S[ ] M[ ] F[ ] K[ ] ResetNetworkInterfaceAttribute
- W[x] S[ ] M[ ] F[ ] K[ ] ResetSnapshotAttribute
- W[x] S[ ] M[ ] F[ ] K[ ] RestoreAddressToClassic
- W[x] S[ ] M[ ] F[ ] K[ ] RestoreImageFromRecycleBin
- W[x] S[ ] M[ ] F[ ] K[ ] RestoreManagedPrefixListVersion
- W[x] S[ ] M[ ] F[ ] K[ ] RestoreSnapshotFromRecycleBin
- W[x] S[ ] M[ ] F[ ] K[ ] RestoreSnapshotTier
- W[x] S[ ] M[ ] F[ ] K[ ] RestoreVolumeFromRecycleBin
- W[x] S[ ] M[ ] F[ ] K[ ] RevokeClientVpnIngress
- W[x] S[ ] M[x] F[ ] K[x] RevokeSecurityGroupEgress
- W[x] S[ ] M[x] F[ ] K[x] RevokeSecurityGroupIngress
- W[x] S[ ] M[x] F[ ] K[x] RunInstances
- W[x] S[ ] M[ ] F[ ] K[ ] RunScheduledInstances
- W[x] S[ ] M[ ] F[ ] K[ ] SearchLocalGatewayRoutes
- W[x] S[ ] M[ ] F[ ] K[ ] SearchTransitGatewayMulticastGroups
- W[x] S[ ] M[x] F[ ] K[ ] SearchTransitGatewayRoutes
- W[x] S[ ] M[ ] F[ ] K[ ] SendDiagnosticInterrupt
- W[x] S[ ] M[ ] F[ ] K[ ] StartDeclarativePoliciesReport
- W[x] S[ ] M[x] F[ ] K[x] StartInstances
- W[x] S[ ] M[ ] F[ ] K[ ] StartNetworkInsightsAccessScopeAnalysis
- W[x] S[ ] M[ ] F[ ] K[ ] StartNetworkInsightsAnalysis
- W[x] S[ ] M[ ] F[ ] K[ ] StartVpcEndpointServicePrivateDnsVerification
- W[x] S[ ] M[x] F[ ] K[x] StopInstances
- W[x] S[ ] M[ ] F[ ] K[ ] TerminateClientVpnConnections
- W[x] S[ ] M[x] F[ ] K[x] TerminateInstances
- W[x] S[ ] M[x] F[ ] K[ ] UnassignIpv6Addresses
- W[x] S[ ] M[x] F[ ] K[ ] UnassignPrivateIpAddresses
- W[x] S[ ] M[ ] F[ ] K[ ] UnassignPrivateNatGatewayAddress
- W[x] S[ ] M[ ] F[ ] K[ ] UnlockSnapshot
- W[ ] S[x] M[ ] F[ ] K[ ] UnmonitorInstances
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateCapacityManagerMonitoredTagKeys
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateCapacityManagerOrganizationsAccess
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateInterruptibleCapacityReservationAllocation
- W[x] S[ ] M[x] F[ ] K[ ] UpdateSecurityGroupRuleDescriptionsEgress
- W[x] S[ ] M[x] F[ ] K[ ] UpdateSecurityGroupRuleDescriptionsIngress
- W[x] S[ ] M[ ] F[ ] K[ ] WithdrawByoipCidr

Integration tests: 669/713 implemented operations tested (93.8%)
Untested implemented operations: 44

### winterbaume-ec2instanceconnect (ec2-instance-connect) - W: 2/2, S: 0/2, M: 1/2, F: 0/2, K: 0/2

- W[x] S[ ] M[x] F[ ] K[ ] SendSSHPublicKey
- W[x] S[ ] M[ ] F[ ] K[ ] SendSerialConsoleSSHPublicKey

Integration tests: 2/2 implemented operations tested (100.0%)

### winterbaume-ecr (ecr) - W: 58/58, S: 0/58, M: 29/58, F: 0/58, K: 11/58

Terraform E2E: 8 tests across 6 terraform resource types

Resource types: aws_ecr_lifecycle_policy, aws_ecr_pull_through_cache_rule, aws_ecr_registry_scanning_configuration, aws_ecr_replication_configuration, aws_ecr_repository, aws_ecr_repository_policy

- W[x] S[ ] M[ ] F[ ] K[ ] BatchCheckLayerAvailability
- W[x] S[ ] M[x] F[ ] K[x] BatchDeleteImage
- W[x] S[ ] M[x] F[ ] K[x] BatchGetImage
- W[x] S[ ] M[x] F[ ] K[ ] BatchGetRepositoryScanningConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] CompleteLayerUpload
- W[x] S[ ] M[ ] F[ ] K[ ] CreatePullThroughCacheRule
- W[x] S[ ] M[x] F[ ] K[x] CreateRepository
- W[x] S[ ] M[ ] F[ ] K[ ] CreateRepositoryCreationTemplate
- W[x] S[ ] M[x] F[ ] K[x] DeleteLifecyclePolicy
- W[x] S[ ] M[ ] F[ ] K[ ] DeletePullThroughCacheRule
- W[x] S[ ] M[x] F[ ] K[ ] DeleteRegistryPolicy
- W[x] S[ ] M[x] F[ ] K[x] DeleteRepository
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteRepositoryCreationTemplate
- W[x] S[ ] M[x] F[ ] K[ ] DeleteRepositoryPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteSigningConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] DeregisterPullTimeUpdateExclusion
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeImageReplicationStatus
- W[x] S[ ] M[x] F[ ] K[ ] DescribeImageScanFindings
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeImageSigningStatus
- W[x] S[ ] M[x] F[ ] K[ ] DescribeImages
- W[x] S[ ] M[ ] F[ ] K[ ] DescribePullThroughCacheRules
- W[x] S[ ] M[x] F[ ] K[ ] DescribeRegistry
- W[x] S[ ] M[x] F[ ] K[x] DescribeRepositories
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeRepositoryCreationTemplates
- W[x] S[ ] M[ ] F[ ] K[ ] GetAccountSetting
- W[x] S[ ] M[ ] F[ ] K[x] GetAuthorizationToken
- W[x] S[ ] M[ ] F[ ] K[ ] GetDownloadUrlForLayer
- W[x] S[ ] M[x] F[ ] K[x] GetLifecyclePolicy
- W[x] S[ ] M[ ] F[ ] K[ ] GetLifecyclePolicyPreview
- W[x] S[ ] M[x] F[ ] K[ ] GetRegistryPolicy
- W[x] S[ ] M[x] F[ ] K[ ] GetRegistryScanningConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] GetRepositoryPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] GetSigningConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] InitiateLayerUpload
- W[x] S[ ] M[ ] F[ ] K[ ] ListImageReferrers
- W[x] S[ ] M[x] F[ ] K[x] ListImages
- W[x] S[ ] M[ ] F[ ] K[ ] ListPullTimeUpdateExclusions
- W[x] S[ ] M[x] F[ ] K[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] PutAccountSetting
- W[x] S[ ] M[x] F[ ] K[x] PutImage
- W[x] S[ ] M[x] F[ ] K[ ] PutImageScanningConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] PutImageTagMutability
- W[x] S[ ] M[x] F[ ] K[x] PutLifecyclePolicy
- W[x] S[ ] M[x] F[ ] K[ ] PutRegistryPolicy
- W[x] S[ ] M[x] F[ ] K[ ] PutRegistryScanningConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] PutReplicationConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] PutSigningConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] RegisterPullTimeUpdateExclusion
- W[x] S[ ] M[x] F[ ] K[ ] SetRepositoryPolicy
- W[x] S[ ] M[x] F[ ] K[ ] StartImageScan
- W[x] S[ ] M[ ] F[ ] K[ ] StartLifecyclePolicyPreview
- W[x] S[ ] M[x] F[ ] K[ ] TagResource
- W[x] S[ ] M[x] F[ ] K[ ] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateImageStorageClass
- W[x] S[ ] M[ ] F[ ] K[ ] UpdatePullThroughCacheRule
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateRepositoryCreationTemplate
- W[x] S[ ] M[ ] F[ ] K[ ] UploadLayerPart
- W[x] S[ ] M[ ] F[ ] K[ ] ValidatePullThroughCacheRule

Integration tests: 44/58 implemented operations tested (75.9%)
Untested implemented operations: 14

### winterbaume-ecs (ecs) - W: 63/76, S: 1/76, M: 45/76, F: 0/76, K: 12/76

Terraform E2E: 7 tests across 3 terraform resource types

Resource types: aws_ecs_cluster, aws_ecs_service, aws_ecs_task_definition

- W[x] S[ ] M[x] F[ ] K[ ] CreateCapacityProvider
- W[x] S[ ] M[x] F[ ] K[x] CreateCluster
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateDaemon
- W[x] S[ ] M[ ] F[ ] K[ ] CreateExpressGatewayService
- W[x] S[ ] M[x] F[ ] K[x] CreateService
- W[x] S[ ] M[x] F[ ] K[ ] CreateTaskSet
- W[x] S[ ] M[x] F[ ] K[ ] DeleteAccountSetting
- W[x] S[ ] M[x] F[ ] K[ ] DeleteAttributes
- W[x] S[ ] M[x] F[ ] K[ ] DeleteCapacityProvider
- W[x] S[ ] M[x] F[ ] K[x] DeleteCluster
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteDaemon
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteDaemonTaskDefinition
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteExpressGatewayService
- W[x] S[ ] M[x] F[ ] K[x] DeleteService
- W[x] S[ ] M[x] F[ ] K[ ] DeleteTaskDefinitions
- W[x] S[ ] M[x] F[ ] K[ ] DeleteTaskSet
- W[x] S[ ] M[x] F[ ] K[ ] DeregisterContainerInstance
- W[x] S[ ] M[x] F[ ] K[x] DeregisterTaskDefinition
- W[x] S[ ] M[x] F[ ] K[ ] DescribeCapacityProviders
- W[x] S[ ] M[x] F[ ] K[x] DescribeClusters
- W[x] S[ ] M[x] F[ ] K[ ] DescribeContainerInstances
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeDaemon
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeDaemonDeployments
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeDaemonRevisions
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeDaemonTaskDefinition
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeExpressGatewayService
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeServiceDeployments
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeServiceRevisions
- W[x] S[ ] M[x] F[ ] K[ ] DescribeServices
- W[x] S[ ] M[x] F[ ] K[ ] DescribeTaskDefinition
- W[x] S[ ] M[x] F[ ] K[ ] DescribeTaskSets
- W[x] S[ ] M[x] F[ ] K[x] DescribeTasks
- W[x] S[ ] M[ ] F[ ] K[ ] DiscoverPollEndpoint
- W[ ] S[x] M[ ] F[ ] K[ ] ExecuteCommand
- W[x] S[ ] M[ ] F[ ] K[ ] GetTaskProtection
- W[x] S[ ] M[x] F[ ] K[ ] ListAccountSettings
- W[x] S[ ] M[x] F[ ] K[ ] ListAttributes
- W[x] S[ ] M[x] F[ ] K[x] ListClusters
- W[x] S[ ] M[x] F[ ] K[ ] ListContainerInstances
- W[ ] S[ ] M[ ] F[ ] K[ ] ListDaemonDeployments
- W[ ] S[ ] M[ ] F[ ] K[ ] ListDaemonTaskDefinitions
- W[ ] S[ ] M[ ] F[ ] K[ ] ListDaemons
- W[x] S[ ] M[ ] F[ ] K[ ] ListServiceDeployments
- W[x] S[ ] M[x] F[ ] K[ ] ListServices
- W[x] S[ ] M[ ] F[ ] K[ ] ListServicesByNamespace
- W[x] S[ ] M[x] F[ ] K[ ] ListTagsForResource
- W[x] S[ ] M[x] F[ ] K[ ] ListTaskDefinitionFamilies
- W[x] S[ ] M[x] F[ ] K[ ] ListTaskDefinitions
- W[x] S[ ] M[x] F[ ] K[ ] ListTasks
- W[x] S[ ] M[x] F[ ] K[ ] PutAccountSetting
- W[x] S[ ] M[ ] F[ ] K[ ] PutAccountSettingDefault
- W[x] S[ ] M[x] F[ ] K[ ] PutAttributes
- W[x] S[ ] M[x] F[ ] K[ ] PutClusterCapacityProviders
- W[x] S[ ] M[x] F[ ] K[ ] RegisterContainerInstance
- W[ ] S[ ] M[ ] F[ ] K[ ] RegisterDaemonTaskDefinition
- W[x] S[ ] M[x] F[ ] K[x] RegisterTaskDefinition
- W[x] S[ ] M[x] F[ ] K[x] RunTask
- W[x] S[ ] M[x] F[ ] K[ ] StartTask
- W[x] S[ ] M[ ] F[ ] K[ ] StopServiceDeployment
- W[x] S[ ] M[x] F[ ] K[x] StopTask
- W[x] S[ ] M[ ] F[ ] K[ ] SubmitAttachmentStateChanges
- W[x] S[ ] M[ ] F[ ] K[ ] SubmitContainerStateChange
- W[x] S[ ] M[ ] F[ ] K[ ] SubmitTaskStateChange
- W[x] S[ ] M[x] F[ ] K[ ] TagResource
- W[x] S[ ] M[x] F[ ] K[ ] UntagResource
- W[x] S[ ] M[x] F[ ] K[ ] UpdateCapacityProvider
- W[x] S[ ] M[x] F[ ] K[ ] UpdateCluster
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateClusterSettings
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateContainerAgent
- W[x] S[ ] M[x] F[ ] K[ ] UpdateContainerInstancesState
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateDaemon
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateExpressGatewayService
- W[x] S[ ] M[x] F[ ] K[x] UpdateService
- W[x] S[ ] M[x] F[ ] K[ ] UpdateServicePrimaryTaskSet
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateTaskProtection
- W[x] S[ ] M[x] F[ ] K[ ] UpdateTaskSet

Integration tests: 58/63 implemented operations tested (92.1%)
Untested implemented operations: 5

### winterbaume-efs (efs) - W: 31/31, S: 0/31, M: 19/31, F: 0/31, K: 0/31

Terraform E2E: 7 tests across 3 terraform resource types

Resource types: aws_efs_access_point, aws_efs_file_system, aws_efs_file_system_policy

- W[x] S[ ] M[x] F[ ] K[ ] CreateAccessPoint
- W[x] S[ ] M[x] F[ ] K[ ] CreateFileSystem
- W[x] S[ ] M[x] F[ ] K[ ] CreateMountTarget
- W[x] S[ ] M[ ] F[ ] K[ ] CreateReplicationConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] CreateTags
- W[x] S[ ] M[x] F[ ] K[ ] DeleteAccessPoint
- W[x] S[ ] M[x] F[ ] K[ ] DeleteFileSystem
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteFileSystemPolicy
- W[x] S[ ] M[x] F[ ] K[ ] DeleteMountTarget
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteReplicationConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteTags
- W[x] S[ ] M[x] F[ ] K[ ] DescribeAccessPoints
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeAccountPreferences
- W[x] S[ ] M[x] F[ ] K[ ] DescribeBackupPolicy
- W[x] S[ ] M[x] F[ ] K[ ] DescribeFileSystemPolicy
- W[x] S[ ] M[x] F[ ] K[ ] DescribeFileSystems
- W[x] S[ ] M[x] F[ ] K[ ] DescribeLifecycleConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] DescribeMountTargetSecurityGroups
- W[x] S[ ] M[x] F[ ] K[ ] DescribeMountTargets
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeReplicationConfigurations
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeTags
- W[x] S[ ] M[x] F[ ] K[ ] ListTagsForResource
- W[x] S[ ] M[x] F[ ] K[ ] ModifyMountTargetSecurityGroups
- W[x] S[ ] M[ ] F[ ] K[ ] PutAccountPreferences
- W[x] S[ ] M[ ] F[ ] K[ ] PutBackupPolicy
- W[x] S[ ] M[x] F[ ] K[ ] PutFileSystemPolicy
- W[x] S[ ] M[x] F[ ] K[ ] PutLifecycleConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] TagResource
- W[x] S[ ] M[x] F[ ] K[ ] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateFileSystem
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateFileSystemProtection

Integration tests: 31/31 implemented operations tested (100.0%)

### winterbaume-eks (eks) - W: 55/64, S: 4/64, M: 17/64, F: 0/64, K: 8/64

Terraform E2E: 11 tests across 6 terraform resource types

Resource types: aws_eks_access_entry, aws_eks_access_policy_association, aws_eks_addon, aws_eks_cluster, aws_eks_node_group, aws_eks_pod_identity_association

- W[x] S[ ] M[ ] F[ ] K[ ] AssociateAccessPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] AssociateEncryptionConfig
- W[x] S[ ] M[ ] F[ ] K[ ] AssociateIdentityProviderConfig
- W[x] S[ ] M[ ] F[ ] K[ ] CreateAccessEntry
- W[x] S[ ] M[ ] F[ ] K[ ] CreateAddon
- W[x] S[ ] M[ ] F[ ] K[ ] CreateCapability
- W[x] S[ ] M[x] F[ ] K[x] CreateCluster
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateEksAnywhereSubscription
- W[x] S[ ] M[x] F[ ] K[ ] CreateFargateProfile
- W[x] S[ ] M[x] F[ ] K[x] CreateNodegroup
- W[x] S[ ] M[ ] F[ ] K[ ] CreatePodIdentityAssociation
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteAccessEntry
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteAddon
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteCapability
- W[x] S[ ] M[x] F[ ] K[x] DeleteCluster
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteEksAnywhereSubscription
- W[x] S[ ] M[x] F[ ] K[ ] DeleteFargateProfile
- W[x] S[ ] M[x] F[ ] K[x] DeleteNodegroup
- W[x] S[ ] M[ ] F[ ] K[ ] DeletePodIdentityAssociation
- W[x] S[ ] M[ ] F[ ] K[ ] DeregisterCluster
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeAccessEntry
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeAddon
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeAddonConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeAddonVersions
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeCapability
- W[x] S[ ] M[x] F[ ] K[x] DescribeCluster
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeClusterVersions
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeEksAnywhereSubscription
- W[x] S[ ] M[x] F[ ] K[ ] DescribeFargateProfile
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeIdentityProviderConfig
- W[ ] S[x] M[ ] F[ ] K[ ] DescribeInsight
- W[ ] S[x] M[ ] F[ ] K[ ] DescribeInsightsRefresh
- W[x] S[ ] M[x] F[ ] K[x] DescribeNodegroup
- W[x] S[ ] M[ ] F[ ] K[ ] DescribePodIdentityAssociation
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeUpdate
- W[x] S[ ] M[ ] F[ ] K[ ] DisassociateAccessPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] DisassociateIdentityProviderConfig
- W[x] S[ ] M[ ] F[ ] K[ ] ListAccessEntries
- W[x] S[ ] M[ ] F[ ] K[ ] ListAccessPolicies
- W[x] S[ ] M[ ] F[ ] K[ ] ListAddons
- W[x] S[ ] M[ ] F[ ] K[ ] ListAssociatedAccessPolicies
- W[x] S[ ] M[ ] F[ ] K[ ] ListCapabilities
- W[x] S[ ] M[x] F[ ] K[x] ListClusters
- W[ ] S[ ] M[ ] F[ ] K[ ] ListEksAnywhereSubscriptions
- W[x] S[ ] M[x] F[ ] K[ ] ListFargateProfiles
- W[x] S[ ] M[ ] F[ ] K[ ] ListIdentityProviderConfigs
- W[ ] S[x] M[ ] F[ ] K[ ] ListInsights
- W[x] S[ ] M[x] F[ ] K[x] ListNodegroups
- W[x] S[ ] M[ ] F[ ] K[ ] ListPodIdentityAssociations
- W[x] S[ ] M[x] F[ ] K[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] ListUpdates
- W[x] S[ ] M[ ] F[ ] K[ ] RegisterCluster
- W[ ] S[x] M[ ] F[ ] K[ ] StartInsightsRefresh
- W[x] S[ ] M[x] F[ ] K[ ] TagResource
- W[x] S[ ] M[x] F[ ] K[ ] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateAccessEntry
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateAddon
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateCapability
- W[x] S[ ] M[x] F[ ] K[ ] UpdateClusterConfig
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateClusterVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateEksAnywhereSubscription
- W[x] S[ ] M[x] F[ ] K[ ] UpdateNodegroupConfig
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateNodegroupVersion
- W[x] S[ ] M[ ] F[ ] K[ ] UpdatePodIdentityAssociation

Integration tests: 41/55 implemented operations tested (74.5%)
Untested implemented operations: 14

### winterbaume-elasticache (elasticache) - W: 24/75, S: 0/75, M: 17/75, F: 0/75, K: 7/75

- W[x] S[ ] M[x] F[ ] K[ ] AddTagsToResource
- W[ ] S[ ] M[ ] F[ ] K[ ] AuthorizeCacheSecurityGroupIngress
- W[ ] S[ ] M[ ] F[ ] K[ ] BatchApplyUpdateAction
- W[ ] S[ ] M[ ] F[ ] K[ ] BatchStopUpdateAction
- W[ ] S[ ] M[ ] F[ ] K[ ] CompleteMigration
- W[ ] S[ ] M[ ] F[ ] K[ ] CopyServerlessCacheSnapshot
- W[ ] S[ ] M[ ] F[ ] K[ ] CopySnapshot
- W[x] S[ ] M[x] F[ ] K[x] CreateCacheCluster
- W[x] S[ ] M[ ] F[ ] K[ ] CreateCacheParameterGroup
- W[x] S[ ] M[ ] F[ ] K[ ] CreateCacheSecurityGroup
- W[x] S[ ] M[x] F[ ] K[ ] CreateCacheSubnetGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateGlobalReplicationGroup
- W[x] S[ ] M[x] F[ ] K[x] CreateReplicationGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateServerlessCache
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateServerlessCacheSnapshot
- W[x] S[ ] M[x] F[ ] K[ ] CreateSnapshot
- W[x] S[ ] M[x] F[ ] K[ ] CreateUser
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateUserGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] DecreaseNodeGroupsInGlobalReplicationGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] DecreaseReplicaCount
- W[x] S[ ] M[x] F[ ] K[x] DeleteCacheCluster
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteCacheParameterGroup
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteCacheSecurityGroup
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteCacheSubnetGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteGlobalReplicationGroup
- W[x] S[ ] M[x] F[ ] K[x] DeleteReplicationGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteServerlessCache
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteServerlessCacheSnapshot
- W[x] S[ ] M[x] F[ ] K[ ] DeleteSnapshot
- W[x] S[ ] M[x] F[ ] K[ ] DeleteUser
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteUserGroup
- W[x] S[ ] M[x] F[ ] K[x] DescribeCacheClusters
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeCacheEngineVersions
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeCacheParameterGroups
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeCacheParameters
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeCacheSecurityGroups
- W[x] S[ ] M[x] F[ ] K[ ] DescribeCacheSubnetGroups
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeEngineDefaultParameters
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeEvents
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeGlobalReplicationGroups
- W[x] S[ ] M[x] F[ ] K[x] DescribeReplicationGroups
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeReservedCacheNodes
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeReservedCacheNodesOfferings
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeServerlessCacheSnapshots
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeServerlessCaches
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeServiceUpdates
- W[x] S[ ] M[x] F[ ] K[ ] DescribeSnapshots
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeUpdateActions
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeUserGroups
- W[x] S[ ] M[x] F[ ] K[ ] DescribeUsers
- W[ ] S[ ] M[ ] F[ ] K[ ] DisassociateGlobalReplicationGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] ExportServerlessCacheSnapshot
- W[ ] S[ ] M[ ] F[ ] K[ ] FailoverGlobalReplicationGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] IncreaseNodeGroupsInGlobalReplicationGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] IncreaseReplicaCount
- W[ ] S[ ] M[ ] F[ ] K[ ] ListAllowedNodeTypeModifications
- W[x] S[ ] M[x] F[ ] K[ ] ListTagsForResource
- W[ ] S[ ] M[ ] F[ ] K[x] ModifyCacheCluster
- W[ ] S[ ] M[ ] F[ ] K[ ] ModifyCacheParameterGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] ModifyCacheSubnetGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] ModifyGlobalReplicationGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] ModifyReplicationGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] ModifyReplicationGroupShardConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] ModifyServerlessCache
- W[ ] S[ ] M[ ] F[ ] K[ ] ModifyUser
- W[ ] S[ ] M[ ] F[ ] K[ ] ModifyUserGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] PurchaseReservedCacheNodesOffering
- W[ ] S[ ] M[ ] F[ ] K[ ] RebalanceSlotsInGlobalReplicationGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] RebootCacheCluster
- W[x] S[ ] M[x] F[ ] K[ ] RemoveTagsFromResource
- W[ ] S[ ] M[ ] F[ ] K[ ] ResetCacheParameterGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] RevokeCacheSecurityGroupIngress
- W[ ] S[ ] M[ ] F[ ] K[ ] StartMigration
- W[ ] S[ ] M[ ] F[ ] K[ ] TestFailover
- W[ ] S[ ] M[ ] F[ ] K[ ] TestMigration

Integration tests: 24/24 implemented operations tested (100.0%)

### winterbaume-elasticbeanstalk (elastic-beanstalk) - W: 7/47, S: 0/47, M: 0/47, F: 0/47, K: 7/47

- W[ ] S[ ] M[ ] F[ ] K[ ] AbortEnvironmentUpdate
- W[ ] S[ ] M[ ] F[ ] K[ ] ApplyEnvironmentManagedAction
- W[ ] S[ ] M[ ] F[ ] K[ ] AssociateEnvironmentOperationsRole
- W[ ] S[ ] M[ ] F[ ] K[ ] CheckDNSAvailability
- W[ ] S[ ] M[ ] F[ ] K[ ] ComposeEnvironments
- W[x] S[ ] M[ ] F[ ] K[x] CreateApplication
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateApplicationVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateConfigurationTemplate
- W[x] S[ ] M[ ] F[ ] K[x] CreateEnvironment
- W[ ] S[ ] M[ ] F[ ] K[ ] CreatePlatformVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateStorageLocation
- W[x] S[ ] M[ ] F[ ] K[x] DeleteApplication
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteApplicationVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteConfigurationTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteEnvironmentConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] DeletePlatformVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeAccountAttributes
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeApplicationVersions
- W[ ] S[ ] M[ ] F[ ] K[x] DescribeApplications
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeConfigurationOptions
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeConfigurationSettings
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeEnvironmentHealth
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeEnvironmentManagedActionHistory
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeEnvironmentManagedActions
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeEnvironmentResources
- W[x] S[ ] M[ ] F[ ] K[x] DescribeEnvironments
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeEvents
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeInstancesHealth
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribePlatformVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] DisassociateEnvironmentOperationsRole
- W[x] S[ ] M[ ] F[ ] K[ ] ListAvailableSolutionStacks
- W[ ] S[ ] M[ ] F[ ] K[ ] ListPlatformBranches
- W[ ] S[ ] M[ ] F[ ] K[ ] ListPlatformVersions
- W[x] S[ ] M[ ] F[ ] K[ ] ListTagsForResource
- W[ ] S[ ] M[ ] F[ ] K[ ] RebuildEnvironment
- W[ ] S[ ] M[ ] F[ ] K[ ] RequestEnvironmentInfo
- W[ ] S[ ] M[ ] F[ ] K[ ] RestartAppServer
- W[ ] S[ ] M[ ] F[ ] K[ ] RetrieveEnvironmentInfo
- W[ ] S[ ] M[ ] F[ ] K[ ] SwapEnvironmentCNAMEs
- W[ ] S[ ] M[ ] F[ ] K[x] TerminateEnvironment
- W[ ] S[ ] M[ ] F[ ] K[x] UpdateApplication
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateApplicationResourceLifecycle
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateApplicationVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateConfigurationTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateEnvironment
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateTagsForResource
- W[ ] S[ ] M[ ] F[ ] K[ ] ValidateConfigurationSettings

Integration tests: 7/7 implemented operations tested (100.0%)

### winterbaume-elasticloadbalancing (elastic-load-balancing) - W: 29/29, S: 0/29, M: 21/29, F: 0/29, K: 0/29

- W[x] S[ ] M[ ] F[ ] K[ ] AddTags
- W[x] S[ ] M[x] F[ ] K[ ] ApplySecurityGroupsToLoadBalancer
- W[x] S[ ] M[x] F[ ] K[ ] AttachLoadBalancerToSubnets
- W[x] S[ ] M[x] F[ ] K[ ] ConfigureHealthCheck
- W[x] S[ ] M[x] F[ ] K[ ] CreateAppCookieStickinessPolicy
- W[x] S[ ] M[x] F[ ] K[ ] CreateLBCookieStickinessPolicy
- W[x] S[ ] M[x] F[ ] K[ ] CreateLoadBalancer
- W[x] S[ ] M[x] F[ ] K[ ] CreateLoadBalancerListeners
- W[x] S[ ] M[x] F[ ] K[ ] CreateLoadBalancerPolicy
- W[x] S[ ] M[x] F[ ] K[ ] DeleteLoadBalancer
- W[x] S[ ] M[x] F[ ] K[ ] DeleteLoadBalancerListeners
- W[x] S[ ] M[x] F[ ] K[ ] DeleteLoadBalancerPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] DeregisterInstancesFromLoadBalancer
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeAccountLimits
- W[x] S[ ] M[x] F[ ] K[ ] DescribeInstanceHealth
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeLoadBalancerAttributes
- W[x] S[ ] M[x] F[ ] K[ ] DescribeLoadBalancerPolicies
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeLoadBalancerPolicyTypes
- W[x] S[ ] M[x] F[ ] K[ ] DescribeLoadBalancers
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeTags
- W[x] S[ ] M[x] F[ ] K[ ] DetachLoadBalancerFromSubnets
- W[x] S[ ] M[x] F[ ] K[ ] DisableAvailabilityZonesForLoadBalancer
- W[x] S[ ] M[x] F[ ] K[ ] EnableAvailabilityZonesForLoadBalancer
- W[x] S[ ] M[x] F[ ] K[ ] ModifyLoadBalancerAttributes
- W[x] S[ ] M[ ] F[ ] K[ ] RegisterInstancesWithLoadBalancer
- W[x] S[ ] M[ ] F[ ] K[ ] RemoveTags
- W[x] S[ ] M[x] F[ ] K[ ] SetLoadBalancerListenerSSLCertificate
- W[x] S[ ] M[x] F[ ] K[ ] SetLoadBalancerPoliciesForBackendServer
- W[x] S[ ] M[x] F[ ] K[ ] SetLoadBalancerPoliciesOfListener

Integration tests: 21/29 implemented operations tested (72.4%)
Untested implemented operations: 8

### winterbaume-elasticloadbalancingv2 (elastic-load-balancing-v2) - W: 50/51, S: 1/51, M: 33/51, F: 0/51, K: 22/51

Terraform E2E: 12 tests across 6 terraform resource types

Resource types: aws_lb, aws_lb_listener, aws_lb_listener_rule, aws_lb_target_group, aws_lb_target_group_attachment, aws_lb_trust_store

- W[x] S[ ] M[x] F[ ] K[ ] AddListenerCertificates
- W[x] S[ ] M[x] F[ ] K[ ] AddTags
- W[x] S[ ] M[ ] F[ ] K[ ] AddTrustStoreRevocations
- W[x] S[ ] M[x] F[ ] K[x] CreateListener
- W[x] S[ ] M[x] F[ ] K[x] CreateLoadBalancer
- W[x] S[ ] M[x] F[ ] K[x] CreateRule
- W[x] S[ ] M[x] F[ ] K[x] CreateTargetGroup
- W[x] S[ ] M[ ] F[ ] K[ ] CreateTrustStore
- W[x] S[ ] M[x] F[ ] K[x] DeleteListener
- W[x] S[ ] M[x] F[ ] K[x] DeleteLoadBalancer
- W[x] S[ ] M[x] F[ ] K[x] DeleteRule
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteSharedTrustStoreAssociation
- W[x] S[ ] M[x] F[ ] K[x] DeleteTargetGroup
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteTrustStore
- W[x] S[ ] M[x] F[ ] K[x] DeregisterTargets
- W[ ] S[x] M[ ] F[ ] K[ ] DescribeAccountLimits
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeCapacityReservation
- W[x] S[ ] M[x] F[ ] K[ ] DescribeListenerAttributes
- W[x] S[ ] M[x] F[ ] K[ ] DescribeListenerCertificates
- W[x] S[ ] M[x] F[ ] K[x] DescribeListeners
- W[x] S[ ] M[x] F[ ] K[x] DescribeLoadBalancerAttributes
- W[x] S[ ] M[x] F[ ] K[x] DescribeLoadBalancers
- W[x] S[ ] M[x] F[ ] K[x] DescribeRules
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeSSLPolicies
- W[x] S[ ] M[x] F[ ] K[ ] DescribeTags
- W[x] S[ ] M[ ] F[ ] K[x] DescribeTargetGroupAttributes
- W[x] S[ ] M[x] F[ ] K[x] DescribeTargetGroups
- W[x] S[ ] M[x] F[ ] K[x] DescribeTargetHealth
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeTrustStoreAssociations
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeTrustStoreRevocations
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeTrustStores
- W[x] S[ ] M[ ] F[ ] K[ ] GetResourcePolicy
- W[x] S[ ] M[ ] F[ ] K[ ] GetTrustStoreCaCertificatesBundle
- W[x] S[ ] M[ ] F[ ] K[ ] GetTrustStoreRevocationContent
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyCapacityReservation
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyIpPools
- W[x] S[ ] M[x] F[ ] K[x] ModifyListener
- W[x] S[ ] M[x] F[ ] K[ ] ModifyListenerAttributes
- W[x] S[ ] M[x] F[ ] K[x] ModifyLoadBalancerAttributes
- W[x] S[ ] M[x] F[ ] K[x] ModifyRule
- W[x] S[ ] M[x] F[ ] K[ ] ModifyTargetGroup
- W[x] S[ ] M[x] F[ ] K[x] ModifyTargetGroupAttributes
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyTrustStore
- W[x] S[ ] M[x] F[ ] K[x] RegisterTargets
- W[x] S[ ] M[x] F[ ] K[ ] RemoveListenerCertificates
- W[x] S[ ] M[x] F[ ] K[ ] RemoveTags
- W[x] S[ ] M[ ] F[ ] K[ ] RemoveTrustStoreRevocations
- W[x] S[ ] M[x] F[ ] K[ ] SetIpAddressType
- W[x] S[ ] M[x] F[ ] K[x] SetRulePriorities
- W[x] S[ ] M[x] F[ ] K[ ] SetSecurityGroups
- W[x] S[ ] M[x] F[ ] K[ ] SetSubnets

Integration tests: 38/50 implemented operations tested (76.0%)
Untested implemented operations: 12

### winterbaume-emr (emr) - W: 54/60, S: 2/60, M: 26/60, F: 0/60, K: 0/60

- W[x] S[ ] M[ ] F[ ] K[ ] AddInstanceFleet
- W[x] S[ ] M[x] F[ ] K[ ] AddInstanceGroups
- W[x] S[ ] M[x] F[ ] K[ ] AddJobFlowSteps
- W[x] S[ ] M[x] F[ ] K[ ] AddTags
- W[x] S[ ] M[ ] F[ ] K[ ] CancelSteps
- W[x] S[ ] M[ ] F[ ] K[ ] CreatePersistentAppUI
- W[x] S[ ] M[x] F[ ] K[ ] CreateSecurityConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] CreateStudio
- W[x] S[ ] M[ ] F[ ] K[ ] CreateStudioSessionMapping
- W[x] S[ ] M[x] F[ ] K[ ] DeleteSecurityConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteStudio
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteStudioSessionMapping
- W[x] S[ ] M[x] F[ ] K[ ] DescribeCluster
- W[x] S[ ] M[x] F[ ] K[ ] DescribeJobFlows
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeNotebookExecution
- W[x] S[ ] M[ ] F[ ] K[ ] DescribePersistentAppUI
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeReleaseLabel
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeSecurityConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] DescribeStep
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeStudio
- W[x] S[ ] M[ ] F[ ] K[ ] GetAutoTerminationPolicy
- W[x] S[ ] M[x] F[ ] K[ ] GetBlockPublicAccessConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] GetClusterSessionCredentials
- W[x] S[ ] M[ ] F[ ] K[ ] GetManagedScalingPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] GetOnClusterAppUIPresignedURL
- W[ ] S[ ] M[ ] F[ ] K[ ] GetPersistentAppUIPresignedURL
- W[x] S[ ] M[ ] F[ ] K[ ] GetStudioSessionMapping
- W[x] S[ ] M[x] F[ ] K[ ] ListBootstrapActions
- W[x] S[ ] M[x] F[ ] K[ ] ListClusters
- W[x] S[ ] M[ ] F[ ] K[ ] ListInstanceFleets
- W[x] S[ ] M[x] F[ ] K[ ] ListInstanceGroups
- W[x] S[ ] M[x] F[ ] K[ ] ListInstances
- W[x] S[ ] M[ ] F[ ] K[ ] ListNotebookExecutions
- W[ ] S[x] M[x] F[ ] K[ ] ListReleaseLabels
- W[x] S[ ] M[ ] F[ ] K[ ] ListSecurityConfigurations
- W[x] S[ ] M[x] F[ ] K[ ] ListSteps
- W[x] S[ ] M[ ] F[ ] K[ ] ListStudioSessionMappings
- W[x] S[ ] M[ ] F[ ] K[ ] ListStudios
- W[ ] S[x] M[x] F[ ] K[ ] ListSupportedInstanceTypes
- W[x] S[ ] M[x] F[ ] K[ ] ModifyCluster
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyInstanceFleet
- W[x] S[ ] M[x] F[ ] K[ ] ModifyInstanceGroups
- W[x] S[ ] M[x] F[ ] K[ ] PutAutoScalingPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] PutAutoTerminationPolicy
- W[x] S[ ] M[x] F[ ] K[ ] PutBlockPublicAccessConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] PutManagedScalingPolicy
- W[x] S[ ] M[x] F[ ] K[ ] RemoveAutoScalingPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] RemoveAutoTerminationPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] RemoveManagedScalingPolicy
- W[x] S[ ] M[x] F[ ] K[ ] RemoveTags
- W[x] S[ ] M[x] F[ ] K[ ] RunJobFlow
- W[x] S[ ] M[ ] F[ ] K[ ] SetKeepJobFlowAliveWhenNoSteps
- W[x] S[ ] M[x] F[ ] K[ ] SetTerminationProtection
- W[ ] S[ ] M[ ] F[ ] K[ ] SetUnhealthyNodeReplacement
- W[x] S[ ] M[x] F[ ] K[ ] SetVisibleToAllUsers
- W[x] S[ ] M[ ] F[ ] K[ ] StartNotebookExecution
- W[x] S[ ] M[ ] F[ ] K[ ] StopNotebookExecution
- W[x] S[ ] M[x] F[ ] K[ ] TerminateJobFlows
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateStudio
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateStudioSessionMapping

Integration tests: 34/54 implemented operations tested (63.0%)
Untested implemented operations: 20

### winterbaume-emrcontainers (emr-containers) - W: 23/23, S: 0/23, M: 8/23, F: 0/23, K: 0/23

- W[x] S[ ] M[x] F[ ] K[ ] CancelJobRun
- W[x] S[ ] M[ ] F[ ] K[ ] CreateJobTemplate
- W[x] S[ ] M[ ] F[ ] K[ ] CreateManagedEndpoint
- W[x] S[ ] M[ ] F[ ] K[ ] CreateSecurityConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] CreateVirtualCluster
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteJobTemplate
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteManagedEndpoint
- W[x] S[ ] M[x] F[ ] K[ ] DeleteVirtualCluster
- W[x] S[ ] M[x] F[ ] K[ ] DescribeJobRun
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeJobTemplate
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeManagedEndpoint
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeSecurityConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] DescribeVirtualCluster
- W[x] S[ ] M[ ] F[ ] K[ ] GetManagedEndpointSessionCredentials
- W[x] S[ ] M[x] F[ ] K[ ] ListJobRuns
- W[x] S[ ] M[ ] F[ ] K[ ] ListJobTemplates
- W[x] S[ ] M[ ] F[ ] K[ ] ListManagedEndpoints
- W[x] S[ ] M[ ] F[ ] K[ ] ListSecurityConfigurations
- W[x] S[ ] M[ ] F[ ] K[ ] ListTagsForResource
- W[x] S[ ] M[x] F[ ] K[ ] ListVirtualClusters
- W[x] S[ ] M[x] F[ ] K[ ] StartJobRun
- W[x] S[ ] M[ ] F[ ] K[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UntagResource

Integration tests: 23/23 implemented operations tested (100.0%)

### winterbaume-emrserverless (emr-serverless) - W: 16/22, S: 0/22, M: 11/22, F: 0/22, K: 11/22

- W[x] S[ ] M[x] F[ ] K[x] CancelJobRun
- W[x] S[ ] M[x] F[ ] K[x] CreateApplication
- W[x] S[ ] M[x] F[ ] K[x] DeleteApplication
- W[x] S[ ] M[x] F[ ] K[x] GetApplication
- W[x] S[ ] M[ ] F[ ] K[ ] GetDashboardForJobRun
- W[x] S[ ] M[x] F[ ] K[x] GetJobRun
- W[ ] S[ ] M[ ] F[ ] K[ ] GetResourceDashboard
- W[ ] S[ ] M[ ] F[ ] K[ ] GetSession
- W[ ] S[ ] M[ ] F[ ] K[ ] GetSessionEndpoint
- W[x] S[ ] M[x] F[ ] K[x] ListApplications
- W[x] S[ ] M[ ] F[ ] K[ ] ListJobRunAttempts
- W[x] S[ ] M[x] F[ ] K[x] ListJobRuns
- W[ ] S[ ] M[ ] F[ ] K[ ] ListSessions
- W[x] S[ ] M[ ] F[ ] K[ ] ListTagsForResource
- W[x] S[ ] M[x] F[ ] K[x] StartApplication
- W[x] S[ ] M[x] F[ ] K[x] StartJobRun
- W[ ] S[ ] M[ ] F[ ] K[ ] StartSession
- W[x] S[ ] M[x] F[ ] K[x] StopApplication
- W[x] S[ ] M[ ] F[ ] K[ ] TagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] TerminateSession
- W[x] S[ ] M[ ] F[ ] K[ ] UntagResource
- W[x] S[ ] M[x] F[ ] K[x] UpdateApplication

Integration tests: 14/16 implemented operations tested (87.5%)
Untested implemented operations: 2

### winterbaume-eventbridge (eventbridge) - W: 57/57, S: 0/57, M: 45/57, F: 0/57, K: 15/57

Terraform E2E: 7 tests across 4 terraform resource types

Resource types: aws_cloudwatch_event_archive, aws_cloudwatch_event_bus, aws_cloudwatch_event_rule, aws_cloudwatch_event_target

- W[x] S[ ] M[ ] F[ ] K[ ] ActivateEventSource
- W[x] S[ ] M[x] F[ ] K[ ] CancelReplay
- W[x] S[ ] M[x] F[ ] K[ ] CreateApiDestination
- W[x] S[ ] M[x] F[ ] K[ ] CreateArchive
- W[x] S[ ] M[x] F[ ] K[x] CreateConnection
- W[x] S[ ] M[ ] F[ ] K[ ] CreateEndpoint
- W[x] S[ ] M[x] F[ ] K[x] CreateEventBus
- W[x] S[ ] M[x] F[ ] K[ ] CreatePartnerEventSource
- W[x] S[ ] M[ ] F[ ] K[ ] DeactivateEventSource
- W[x] S[ ] M[ ] F[ ] K[ ] DeauthorizeConnection
- W[x] S[ ] M[x] F[ ] K[ ] DeleteApiDestination
- W[x] S[ ] M[x] F[ ] K[ ] DeleteArchive
- W[x] S[ ] M[x] F[ ] K[x] DeleteConnection
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteEndpoint
- W[x] S[ ] M[x] F[ ] K[x] DeleteEventBus
- W[x] S[ ] M[x] F[ ] K[ ] DeletePartnerEventSource
- W[x] S[ ] M[x] F[ ] K[x] DeleteRule
- W[x] S[ ] M[x] F[ ] K[ ] DescribeApiDestination
- W[x] S[ ] M[x] F[ ] K[ ] DescribeArchive
- W[x] S[ ] M[x] F[ ] K[x] DescribeConnection
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeEndpoint
- W[x] S[ ] M[x] F[ ] K[x] DescribeEventBus
- W[x] S[ ] M[x] F[ ] K[ ] DescribeEventSource
- W[x] S[ ] M[x] F[ ] K[ ] DescribePartnerEventSource
- W[x] S[ ] M[x] F[ ] K[ ] DescribeReplay
- W[x] S[ ] M[x] F[ ] K[x] DescribeRule
- W[x] S[ ] M[x] F[ ] K[ ] DisableRule
- W[x] S[ ] M[x] F[ ] K[ ] EnableRule
- W[x] S[ ] M[x] F[ ] K[ ] ListApiDestinations
- W[x] S[ ] M[x] F[ ] K[ ] ListArchives
- W[x] S[ ] M[x] F[ ] K[ ] ListConnections
- W[x] S[ ] M[ ] F[ ] K[ ] ListEndpoints
- W[x] S[ ] M[x] F[ ] K[x] ListEventBuses
- W[x] S[ ] M[ ] F[ ] K[ ] ListEventSources
- W[x] S[ ] M[ ] F[ ] K[ ] ListPartnerEventSourceAccounts
- W[x] S[ ] M[ ] F[ ] K[ ] ListPartnerEventSources
- W[x] S[ ] M[x] F[ ] K[ ] ListReplays
- W[x] S[ ] M[x] F[ ] K[ ] ListRuleNamesByTarget
- W[x] S[ ] M[x] F[ ] K[x] ListRules
- W[x] S[ ] M[x] F[ ] K[ ] ListTagsForResource
- W[x] S[ ] M[x] F[ ] K[x] ListTargetsByRule
- W[x] S[ ] M[x] F[ ] K[x] PutEvents
- W[x] S[ ] M[x] F[ ] K[ ] PutPartnerEvents
- W[x] S[ ] M[x] F[ ] K[ ] PutPermission
- W[x] S[ ] M[x] F[ ] K[x] PutRule
- W[x] S[ ] M[x] F[ ] K[x] PutTargets
- W[x] S[ ] M[x] F[ ] K[ ] RemovePermission
- W[x] S[ ] M[x] F[ ] K[x] RemoveTargets
- W[x] S[ ] M[x] F[ ] K[ ] StartReplay
- W[x] S[ ] M[x] F[ ] K[ ] TagResource
- W[x] S[ ] M[x] F[ ] K[ ] TestEventPattern
- W[x] S[ ] M[x] F[ ] K[ ] UntagResource
- W[x] S[ ] M[x] F[ ] K[ ] UpdateApiDestination
- W[x] S[ ] M[x] F[ ] K[ ] UpdateArchive
- W[x] S[ ] M[x] F[ ] K[ ] UpdateConnection
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateEndpoint
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateEventBus

Integration tests: 45/57 implemented operations tested (78.9%)
Untested implemented operations: 12

### winterbaume-firehose (firehose) - W: 12/12, S: 0/12, M: 12/12, F: 0/12, K: 7/12

Terraform E2E: 3 tests across 1 terraform resource types

Resource types: aws_kinesis_firehose_delivery_stream

- W[x] S[ ] M[x] F[ ] K[x] CreateDeliveryStream
- W[x] S[ ] M[x] F[ ] K[x] DeleteDeliveryStream
- W[x] S[ ] M[x] F[ ] K[x] DescribeDeliveryStream
- W[x] S[ ] M[x] F[ ] K[x] ListDeliveryStreams
- W[x] S[ ] M[x] F[ ] K[ ] ListTagsForDeliveryStream
- W[x] S[ ] M[x] F[ ] K[x] PutRecord
- W[x] S[ ] M[x] F[ ] K[x] PutRecordBatch
- W[x] S[ ] M[x] F[ ] K[ ] StartDeliveryStreamEncryption
- W[x] S[ ] M[x] F[ ] K[ ] StopDeliveryStreamEncryption
- W[x] S[ ] M[x] F[ ] K[ ] TagDeliveryStream
- W[x] S[ ] M[x] F[ ] K[ ] UntagDeliveryStream
- W[x] S[ ] M[x] F[ ] K[x] UpdateDestination

Integration tests: 12/12 implemented operations tested (100.0%)

### winterbaume-fis (fis) - W: 8/26, S: 0/26, M: 5/26, F: 0/26, K: 0/26

- W[x] S[ ] M[x] F[ ] K[ ] CreateExperimentTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateTargetAccountConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] DeleteExperimentTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteTargetAccountConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] GetAction
- W[ ] S[ ] M[ ] F[ ] K[ ] GetExperiment
- W[ ] S[ ] M[ ] F[ ] K[ ] GetExperimentTargetAccountConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] GetExperimentTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] GetSafetyLever
- W[ ] S[ ] M[ ] F[ ] K[ ] GetTargetAccountConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] GetTargetResourceType
- W[ ] S[ ] M[ ] F[ ] K[ ] ListActions
- W[ ] S[ ] M[ ] F[ ] K[ ] ListExperimentResolvedTargets
- W[ ] S[ ] M[ ] F[ ] K[ ] ListExperimentTargetAccountConfigurations
- W[x] S[ ] M[ ] F[ ] K[ ] ListExperimentTemplates
- W[ ] S[ ] M[ ] F[ ] K[ ] ListExperiments
- W[x] S[ ] M[x] F[ ] K[ ] ListTagsForResource
- W[ ] S[ ] M[ ] F[ ] K[ ] ListTargetAccountConfigurations
- W[ ] S[ ] M[ ] F[ ] K[ ] ListTargetResourceTypes
- W[ ] S[ ] M[ ] F[ ] K[ ] StartExperiment
- W[ ] S[ ] M[ ] F[ ] K[ ] StopExperiment
- W[x] S[ ] M[x] F[ ] K[ ] TagResource
- W[x] S[ ] M[x] F[ ] K[ ] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateExperimentTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateSafetyLeverState
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateTargetAccountConfiguration

Integration tests: 8/8 implemented operations tested (100.0%)

### winterbaume-forecast (forecast) - W: 5/63, S: 0/63, M: 5/63, F: 0/63, K: 17/63

- W[ ] S[ ] M[ ] F[ ] K[ ] CreateAutoPredictor
- W[ ] S[ ] M[ ] F[ ] K[x] CreateDataset
- W[x] S[ ] M[x] F[ ] K[x] CreateDatasetGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateDatasetImportJob
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateExplainability
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateExplainabilityExport
- W[ ] S[ ] M[ ] F[ ] K[x] CreateForecast
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateForecastExportJob
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateMonitor
- W[ ] S[ ] M[ ] F[ ] K[x] CreatePredictor
- W[ ] S[ ] M[ ] F[ ] K[ ] CreatePredictorBacktestExportJob
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateWhatIfAnalysis
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateWhatIfForecast
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateWhatIfForecastExport
- W[ ] S[ ] M[ ] F[ ] K[x] DeleteDataset
- W[x] S[ ] M[x] F[ ] K[x] DeleteDatasetGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteDatasetImportJob
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteExplainability
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteExplainabilityExport
- W[ ] S[ ] M[ ] F[ ] K[x] DeleteForecast
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteForecastExportJob
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteMonitor
- W[ ] S[ ] M[ ] F[ ] K[x] DeletePredictor
- W[ ] S[ ] M[ ] F[ ] K[ ] DeletePredictorBacktestExportJob
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteResourceTree
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteWhatIfAnalysis
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteWhatIfForecast
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteWhatIfForecastExport
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeAutoPredictor
- W[ ] S[ ] M[ ] F[ ] K[x] DescribeDataset
- W[x] S[ ] M[x] F[ ] K[x] DescribeDatasetGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeDatasetImportJob
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeExplainability
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeExplainabilityExport
- W[ ] S[ ] M[ ] F[ ] K[x] DescribeForecast
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeForecastExportJob
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeMonitor
- W[ ] S[ ] M[ ] F[ ] K[x] DescribePredictor
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribePredictorBacktestExportJob
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeWhatIfAnalysis
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeWhatIfForecast
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeWhatIfForecastExport
- W[ ] S[ ] M[ ] F[ ] K[ ] GetAccuracyMetrics
- W[x] S[ ] M[x] F[ ] K[x] ListDatasetGroups
- W[ ] S[ ] M[ ] F[ ] K[ ] ListDatasetImportJobs
- W[ ] S[ ] M[ ] F[ ] K[x] ListDatasets
- W[ ] S[ ] M[ ] F[ ] K[ ] ListExplainabilities
- W[ ] S[ ] M[ ] F[ ] K[ ] ListExplainabilityExports
- W[ ] S[ ] M[ ] F[ ] K[ ] ListForecastExportJobs
- W[ ] S[ ] M[ ] F[ ] K[x] ListForecasts
- W[ ] S[ ] M[ ] F[ ] K[ ] ListMonitorEvaluations
- W[ ] S[ ] M[ ] F[ ] K[ ] ListMonitors
- W[ ] S[ ] M[ ] F[ ] K[ ] ListPredictorBacktestExportJobs
- W[ ] S[ ] M[ ] F[ ] K[x] ListPredictors
- W[ ] S[ ] M[ ] F[ ] K[ ] ListTagsForResource
- W[ ] S[ ] M[ ] F[ ] K[ ] ListWhatIfAnalyses
- W[ ] S[ ] M[ ] F[ ] K[ ] ListWhatIfForecastExports
- W[ ] S[ ] M[ ] F[ ] K[ ] ListWhatIfForecasts
- W[ ] S[ ] M[ ] F[ ] K[ ] ResumeResource
- W[ ] S[ ] M[ ] F[ ] K[ ] StopResource
- W[ ] S[ ] M[ ] F[ ] K[ ] TagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] UntagResource
- W[x] S[ ] M[x] F[ ] K[x] UpdateDatasetGroup

Integration tests: 5/5 implemented operations tested (100.0%)

### winterbaume-freetier (freetier) - W: 5/5, S: 0/5, M: 0/5, F: 0/5, K: 0/5

- W[x] S[ ] M[ ] F[ ] K[ ] GetAccountActivity
- W[x] S[ ] M[ ] F[ ] K[ ] GetAccountPlanState
- W[x] S[ ] M[ ] F[ ] K[ ] GetFreeTierUsage
- W[x] S[ ] M[ ] F[ ] K[ ] ListAccountActivities
- W[x] S[ ] M[ ] F[ ] K[ ] UpgradeAccountPlan

Integration tests: 5/5 implemented operations tested (100.0%)

### winterbaume-fsx (fsx) - W: 9/48, S: 0/48, M: 9/48, F: 0/48, K: 0/48

- W[ ] S[ ] M[ ] F[ ] K[ ] AssociateFileSystemAliases
- W[ ] S[ ] M[ ] F[ ] K[ ] CancelDataRepositoryTask
- W[ ] S[ ] M[ ] F[ ] K[ ] CopyBackup
- W[ ] S[ ] M[ ] F[ ] K[ ] CopySnapshotAndUpdateVolume
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateAndAttachS3AccessPoint
- W[x] S[ ] M[x] F[ ] K[ ] CreateBackup
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateDataRepositoryAssociation
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateDataRepositoryTask
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateFileCache
- W[x] S[ ] M[x] F[ ] K[ ] CreateFileSystem
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateFileSystemFromBackup
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateSnapshot
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateStorageVirtualMachine
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateVolume
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateVolumeFromBackup
- W[x] S[ ] M[x] F[ ] K[ ] DeleteBackup
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteDataRepositoryAssociation
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteFileCache
- W[x] S[ ] M[x] F[ ] K[ ] DeleteFileSystem
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteSnapshot
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteStorageVirtualMachine
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteVolume
- W[x] S[ ] M[x] F[ ] K[ ] DescribeBackups
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeDataRepositoryAssociations
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeDataRepositoryTasks
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeFileCaches
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeFileSystemAliases
- W[x] S[ ] M[x] F[ ] K[ ] DescribeFileSystems
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeS3AccessPointAttachments
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeSharedVpcConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeSnapshots
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeStorageVirtualMachines
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeVolumes
- W[ ] S[ ] M[ ] F[ ] K[ ] DetachAndDeleteS3AccessPoint
- W[ ] S[ ] M[ ] F[ ] K[ ] DisassociateFileSystemAliases
- W[x] S[ ] M[x] F[ ] K[ ] ListTagsForResource
- W[ ] S[ ] M[ ] F[ ] K[ ] ReleaseFileSystemNfsV3Locks
- W[ ] S[ ] M[ ] F[ ] K[ ] RestoreVolumeFromSnapshot
- W[ ] S[ ] M[ ] F[ ] K[ ] StartMisconfiguredStateRecovery
- W[x] S[ ] M[x] F[ ] K[ ] TagResource
- W[x] S[ ] M[x] F[ ] K[ ] UntagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateDataRepositoryAssociation
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateFileCache
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateFileSystem
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateSharedVpcConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateSnapshot
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateStorageVirtualMachine
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateVolume

Integration tests: 9/9 implemented operations tested (100.0%)

### winterbaume-glacier (glacier) - W: 33/33, S: 0/33, M: 10/33, F: 0/33, K: 4/33

- W[x] S[ ] M[ ] F[ ] K[ ] AbortMultipartUpload
- W[x] S[ ] M[ ] F[ ] K[ ] AbortVaultLock
- W[x] S[ ] M[ ] F[ ] K[ ] AddTagsToVault
- W[x] S[ ] M[ ] F[ ] K[ ] CompleteMultipartUpload
- W[x] S[ ] M[ ] F[ ] K[ ] CompleteVaultLock
- W[x] S[ ] M[x] F[ ] K[x] CreateVault
- W[x] S[ ] M[x] F[ ] K[ ] DeleteArchive
- W[x] S[ ] M[x] F[ ] K[x] DeleteVault
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteVaultAccessPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteVaultNotifications
- W[x] S[ ] M[x] F[ ] K[ ] DescribeJob
- W[x] S[ ] M[x] F[ ] K[x] DescribeVault
- W[x] S[ ] M[ ] F[ ] K[ ] GetDataRetrievalPolicy
- W[x] S[ ] M[x] F[ ] K[ ] GetJobOutput
- W[x] S[ ] M[ ] F[ ] K[ ] GetVaultAccessPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] GetVaultLock
- W[x] S[ ] M[ ] F[ ] K[ ] GetVaultNotifications
- W[x] S[ ] M[x] F[ ] K[ ] InitiateJob
- W[x] S[ ] M[ ] F[ ] K[ ] InitiateMultipartUpload
- W[x] S[ ] M[ ] F[ ] K[ ] InitiateVaultLock
- W[x] S[ ] M[x] F[ ] K[ ] ListJobs
- W[x] S[ ] M[ ] F[ ] K[ ] ListMultipartUploads
- W[x] S[ ] M[ ] F[ ] K[ ] ListParts
- W[x] S[ ] M[ ] F[ ] K[ ] ListProvisionedCapacity
- W[x] S[ ] M[ ] F[ ] K[ ] ListTagsForVault
- W[x] S[ ] M[x] F[ ] K[x] ListVaults
- W[x] S[ ] M[ ] F[ ] K[ ] PurchaseProvisionedCapacity
- W[x] S[ ] M[ ] F[ ] K[ ] RemoveTagsFromVault
- W[x] S[ ] M[ ] F[ ] K[ ] SetDataRetrievalPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] SetVaultAccessPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] SetVaultNotifications
- W[x] S[ ] M[x] F[ ] K[ ] UploadArchive
- W[x] S[ ] M[ ] F[ ] K[ ] UploadMultipartPart

Integration tests: 30/33 implemented operations tested (90.9%)
Untested implemented operations: 3

### winterbaume-glue (glue) - W: 132/265, S: 0/265, M: 96/265, F: 0/265, K: 14/265

- W[x] S[ ] M[x] F[ ] K[ ] BatchCreatePartition
- W[x] S[ ] M[ ] F[ ] K[ ] BatchDeleteConnection
- W[x] S[ ] M[x] F[ ] K[ ] BatchDeletePartition
- W[x] S[ ] M[x] F[ ] K[ ] BatchDeleteTable
- W[ ] S[ ] M[ ] F[ ] K[ ] BatchDeleteTableVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] BatchGetBlueprints
- W[x] S[ ] M[x] F[ ] K[ ] BatchGetCrawlers
- W[ ] S[ ] M[ ] F[ ] K[ ] BatchGetCustomEntityTypes
- W[ ] S[ ] M[ ] F[ ] K[ ] BatchGetDataQualityResult
- W[x] S[ ] M[ ] F[ ] K[ ] BatchGetDevEndpoints
- W[x] S[ ] M[x] F[ ] K[ ] BatchGetJobs
- W[x] S[ ] M[x] F[ ] K[ ] BatchGetPartition
- W[ ] S[ ] M[ ] F[ ] K[ ] BatchGetTableOptimizer
- W[x] S[ ] M[x] F[ ] K[ ] BatchGetTriggers
- W[x] S[ ] M[ ] F[ ] K[ ] BatchGetWorkflows
- W[ ] S[ ] M[ ] F[ ] K[ ] BatchPutDataQualityStatisticAnnotation
- W[x] S[ ] M[ ] F[ ] K[ ] BatchStopJobRun
- W[x] S[ ] M[x] F[ ] K[ ] BatchUpdatePartition
- W[ ] S[ ] M[ ] F[ ] K[ ] CancelDataQualityRuleRecommendationRun
- W[ ] S[ ] M[ ] F[ ] K[ ] CancelDataQualityRulesetEvaluationRun
- W[ ] S[ ] M[ ] F[ ] K[ ] CancelMLTaskRun
- W[ ] S[ ] M[ ] F[ ] K[ ] CancelStatement
- W[x] S[ ] M[ ] F[ ] K[ ] CheckSchemaVersionValidity
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateBlueprint
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateCatalog
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateClassifier
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateColumnStatisticsTaskSettings
- W[x] S[ ] M[x] F[ ] K[ ] CreateConnection
- W[x] S[ ] M[x] F[ ] K[ ] CreateCrawler
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateCustomEntityType
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateDataQualityRuleset
- W[x] S[ ] M[x] F[ ] K[x] CreateDatabase
- W[x] S[ ] M[x] F[ ] K[ ] CreateDevEndpoint
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateGlueIdentityCenterConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateIntegration
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateIntegrationResourceProperty
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateIntegrationTableProperties
- W[x] S[ ] M[x] F[ ] K[x] CreateJob
- W[x] S[ ] M[ ] F[ ] K[ ] CreateMLTransform
- W[x] S[ ] M[x] F[ ] K[ ] CreatePartition
- W[ ] S[ ] M[ ] F[ ] K[ ] CreatePartitionIndex
- W[x] S[ ] M[x] F[ ] K[ ] CreateRegistry
- W[x] S[ ] M[x] F[ ] K[ ] CreateSchema
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateScript
- W[x] S[ ] M[x] F[ ] K[ ] CreateSecurityConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] CreateSession
- W[x] S[ ] M[x] F[ ] K[x] CreateTable
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateTableOptimizer
- W[x] S[ ] M[x] F[ ] K[ ] CreateTrigger
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateUsageProfile
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateUserDefinedFunction
- W[x] S[ ] M[x] F[ ] K[ ] CreateWorkflow
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteBlueprint
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteCatalog
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteClassifier
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteColumnStatisticsForPartition
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteColumnStatisticsForTable
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteColumnStatisticsTaskSettings
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteConnection
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteConnectionType
- W[x] S[ ] M[x] F[ ] K[ ] DeleteCrawler
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteCustomEntityType
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteDataQualityRuleset
- W[x] S[ ] M[x] F[ ] K[x] DeleteDatabase
- W[x] S[ ] M[x] F[ ] K[ ] DeleteDevEndpoint
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteGlueIdentityCenterConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteIntegration
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteIntegrationResourceProperty
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteIntegrationTableProperties
- W[x] S[ ] M[x] F[ ] K[x] DeleteJob
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteMLTransform
- W[x] S[ ] M[x] F[ ] K[ ] DeletePartition
- W[ ] S[ ] M[ ] F[ ] K[ ] DeletePartitionIndex
- W[x] S[ ] M[x] F[ ] K[ ] DeleteRegistry
- W[x] S[ ] M[x] F[ ] K[ ] DeleteResourcePolicy
- W[x] S[ ] M[x] F[ ] K[ ] DeleteSchema
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteSchemaVersions
- W[x] S[ ] M[x] F[ ] K[ ] DeleteSecurityConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] DeleteSession
- W[x] S[ ] M[x] F[ ] K[x] DeleteTable
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteTableOptimizer
- W[x] S[ ] M[x] F[ ] K[ ] DeleteTableVersion
- W[x] S[ ] M[x] F[ ] K[ ] DeleteTrigger
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteUsageProfile
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteUserDefinedFunction
- W[x] S[ ] M[x] F[ ] K[ ] DeleteWorkflow
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeConnectionType
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeEntity
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeInboundIntegrations
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeIntegrations
- W[ ] S[ ] M[ ] F[ ] K[ ] GetBlueprint
- W[ ] S[ ] M[ ] F[ ] K[ ] GetBlueprintRun
- W[ ] S[ ] M[ ] F[ ] K[ ] GetBlueprintRuns
- W[ ] S[ ] M[ ] F[ ] K[ ] GetCatalog
- W[ ] S[ ] M[ ] F[ ] K[ ] GetCatalogImportStatus
- W[ ] S[ ] M[ ] F[ ] K[ ] GetCatalogs
- W[ ] S[ ] M[ ] F[ ] K[ ] GetClassifier
- W[ ] S[ ] M[ ] F[ ] K[ ] GetClassifiers
- W[ ] S[ ] M[ ] F[ ] K[ ] GetColumnStatisticsForPartition
- W[ ] S[ ] M[ ] F[ ] K[ ] GetColumnStatisticsForTable
- W[ ] S[ ] M[ ] F[ ] K[ ] GetColumnStatisticsTaskRun
- W[ ] S[ ] M[ ] F[ ] K[ ] GetColumnStatisticsTaskRuns
- W[ ] S[ ] M[ ] F[ ] K[ ] GetColumnStatisticsTaskSettings
- W[x] S[ ] M[x] F[ ] K[ ] GetConnection
- W[x] S[ ] M[x] F[ ] K[ ] GetConnections
- W[x] S[ ] M[x] F[ ] K[ ] GetCrawler
- W[x] S[ ] M[ ] F[ ] K[ ] GetCrawlerMetrics
- W[x] S[ ] M[x] F[ ] K[ ] GetCrawlers
- W[ ] S[ ] M[ ] F[ ] K[ ] GetCustomEntityType
- W[x] S[ ] M[x] F[ ] K[ ] GetDataCatalogEncryptionSettings
- W[ ] S[ ] M[ ] F[ ] K[ ] GetDataQualityModel
- W[ ] S[ ] M[ ] F[ ] K[ ] GetDataQualityModelResult
- W[ ] S[ ] M[ ] F[ ] K[ ] GetDataQualityResult
- W[ ] S[ ] M[ ] F[ ] K[ ] GetDataQualityRuleRecommendationRun
- W[ ] S[ ] M[ ] F[ ] K[ ] GetDataQualityRuleset
- W[ ] S[ ] M[ ] F[ ] K[ ] GetDataQualityRulesetEvaluationRun
- W[x] S[ ] M[x] F[ ] K[x] GetDatabase
- W[x] S[ ] M[x] F[ ] K[x] GetDatabases
- W[ ] S[ ] M[ ] F[ ] K[ ] GetDataflowGraph
- W[x] S[ ] M[x] F[ ] K[ ] GetDevEndpoint
- W[x] S[ ] M[x] F[ ] K[ ] GetDevEndpoints
- W[ ] S[ ] M[ ] F[ ] K[ ] GetEntityRecords
- W[ ] S[ ] M[ ] F[ ] K[ ] GetGlueIdentityCenterConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] GetIntegrationResourceProperty
- W[ ] S[ ] M[ ] F[ ] K[ ] GetIntegrationTableProperties
- W[x] S[ ] M[x] F[ ] K[ ] GetJob
- W[x] S[ ] M[ ] F[ ] K[ ] GetJobBookmark
- W[x] S[ ] M[x] F[ ] K[ ] GetJobRun
- W[x] S[ ] M[x] F[ ] K[ ] GetJobRuns
- W[x] S[ ] M[x] F[ ] K[ ] GetJobs
- W[ ] S[ ] M[ ] F[ ] K[ ] GetMLTaskRun
- W[ ] S[ ] M[ ] F[ ] K[ ] GetMLTaskRuns
- W[x] S[ ] M[ ] F[ ] K[ ] GetMLTransform
- W[x] S[ ] M[ ] F[ ] K[ ] GetMLTransforms
- W[ ] S[ ] M[ ] F[ ] K[ ] GetMapping
- W[ ] S[ ] M[ ] F[ ] K[ ] GetMaterializedViewRefreshTaskRun
- W[x] S[ ] M[x] F[ ] K[ ] GetPartition
- W[ ] S[ ] M[ ] F[ ] K[ ] GetPartitionIndexes
- W[x] S[ ] M[x] F[ ] K[ ] GetPartitions
- W[ ] S[ ] M[ ] F[ ] K[ ] GetPlan
- W[x] S[ ] M[x] F[ ] K[ ] GetRegistry
- W[ ] S[ ] M[ ] F[ ] K[ ] GetResourcePolicies
- W[x] S[ ] M[x] F[ ] K[ ] GetResourcePolicy
- W[x] S[ ] M[x] F[ ] K[ ] GetSchema
- W[x] S[ ] M[x] F[ ] K[ ] GetSchemaByDefinition
- W[x] S[ ] M[x] F[ ] K[ ] GetSchemaVersion
- W[x] S[ ] M[ ] F[ ] K[ ] GetSchemaVersionsDiff
- W[x] S[ ] M[x] F[ ] K[ ] GetSecurityConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] GetSecurityConfigurations
- W[x] S[ ] M[x] F[ ] K[ ] GetSession
- W[ ] S[ ] M[ ] F[ ] K[ ] GetStatement
- W[x] S[ ] M[x] F[ ] K[x] GetTable
- W[ ] S[ ] M[ ] F[ ] K[ ] GetTableOptimizer
- W[x] S[ ] M[x] F[ ] K[ ] GetTableVersion
- W[x] S[ ] M[x] F[ ] K[ ] GetTableVersions
- W[x] S[ ] M[x] F[ ] K[x] GetTables
- W[x] S[ ] M[x] F[ ] K[x] GetTags
- W[x] S[ ] M[x] F[ ] K[ ] GetTrigger
- W[x] S[ ] M[x] F[ ] K[ ] GetTriggers
- W[ ] S[ ] M[ ] F[ ] K[ ] GetUnfilteredPartitionMetadata
- W[ ] S[ ] M[ ] F[ ] K[ ] GetUnfilteredPartitionsMetadata
- W[ ] S[ ] M[ ] F[ ] K[ ] GetUnfilteredTableMetadata
- W[ ] S[ ] M[ ] F[ ] K[ ] GetUsageProfile
- W[ ] S[ ] M[ ] F[ ] K[ ] GetUserDefinedFunction
- W[ ] S[ ] M[ ] F[ ] K[ ] GetUserDefinedFunctions
- W[x] S[ ] M[x] F[ ] K[ ] GetWorkflow
- W[x] S[ ] M[x] F[ ] K[ ] GetWorkflowRun
- W[x] S[ ] M[x] F[ ] K[ ] GetWorkflowRunProperties
- W[x] S[ ] M[x] F[ ] K[ ] GetWorkflowRuns
- W[x] S[ ] M[ ] F[ ] K[ ] ImportCatalogToGlue
- W[ ] S[ ] M[ ] F[ ] K[ ] ListBlueprints
- W[ ] S[ ] M[ ] F[ ] K[ ] ListColumnStatisticsTaskRuns
- W[ ] S[ ] M[ ] F[ ] K[ ] ListConnectionTypes
- W[x] S[ ] M[x] F[ ] K[ ] ListCrawlers
- W[x] S[ ] M[x] F[ ] K[ ] ListCrawls
- W[ ] S[ ] M[ ] F[ ] K[ ] ListCustomEntityTypes
- W[ ] S[ ] M[ ] F[ ] K[ ] ListDataQualityResults
- W[ ] S[ ] M[ ] F[ ] K[ ] ListDataQualityRuleRecommendationRuns
- W[ ] S[ ] M[ ] F[ ] K[ ] ListDataQualityRulesetEvaluationRuns
- W[ ] S[ ] M[ ] F[ ] K[ ] ListDataQualityRulesets
- W[ ] S[ ] M[ ] F[ ] K[ ] ListDataQualityStatisticAnnotations
- W[ ] S[ ] M[ ] F[ ] K[ ] ListDataQualityStatistics
- W[x] S[ ] M[ ] F[ ] K[ ] ListDevEndpoints
- W[ ] S[ ] M[ ] F[ ] K[ ] ListEntities
- W[ ] S[ ] M[ ] F[ ] K[ ] ListIntegrationResourceProperties
- W[x] S[ ] M[x] F[ ] K[ ] ListJobs
- W[x] S[ ] M[ ] F[ ] K[ ] ListMLTransforms
- W[ ] S[ ] M[ ] F[ ] K[ ] ListMaterializedViewRefreshTaskRuns
- W[x] S[ ] M[x] F[ ] K[ ] ListRegistries
- W[x] S[ ] M[ ] F[ ] K[ ] ListSchemaVersions
- W[x] S[ ] M[ ] F[ ] K[ ] ListSchemas
- W[x] S[ ] M[x] F[ ] K[ ] ListSessions
- W[ ] S[ ] M[ ] F[ ] K[ ] ListStatements
- W[ ] S[ ] M[ ] F[ ] K[ ] ListTableOptimizerRuns
- W[x] S[ ] M[x] F[ ] K[ ] ListTriggers
- W[ ] S[ ] M[ ] F[ ] K[ ] ListUsageProfiles
- W[x] S[ ] M[x] F[ ] K[ ] ListWorkflows
- W[ ] S[ ] M[ ] F[ ] K[ ] ModifyIntegration
- W[x] S[ ] M[x] F[ ] K[ ] PutDataCatalogEncryptionSettings
- W[ ] S[ ] M[ ] F[ ] K[ ] PutDataQualityProfileAnnotation
- W[x] S[ ] M[x] F[ ] K[ ] PutResourcePolicy
- W[x] S[ ] M[x] F[ ] K[ ] PutSchemaVersionMetadata
- W[x] S[ ] M[x] F[ ] K[ ] PutWorkflowRunProperties
- W[x] S[ ] M[ ] F[ ] K[ ] QuerySchemaVersionMetadata
- W[ ] S[ ] M[ ] F[ ] K[ ] RegisterConnectionType
- W[x] S[ ] M[x] F[ ] K[ ] RegisterSchemaVersion
- W[x] S[ ] M[ ] F[ ] K[ ] RemoveSchemaVersionMetadata
- W[x] S[ ] M[ ] F[ ] K[ ] ResetJobBookmark
- W[x] S[ ] M[ ] F[ ] K[ ] ResumeWorkflowRun
- W[ ] S[ ] M[ ] F[ ] K[ ] RunStatement
- W[x] S[ ] M[ ] F[ ] K[ ] SearchTables
- W[ ] S[ ] M[ ] F[ ] K[ ] StartBlueprintRun
- W[ ] S[ ] M[ ] F[ ] K[ ] StartColumnStatisticsTaskRun
- W[ ] S[ ] M[ ] F[ ] K[ ] StartColumnStatisticsTaskRunSchedule
- W[x] S[ ] M[x] F[ ] K[ ] StartCrawler
- W[x] S[ ] M[ ] F[ ] K[ ] StartCrawlerSchedule
- W[ ] S[ ] M[ ] F[ ] K[ ] StartDataQualityRuleRecommendationRun
- W[ ] S[ ] M[ ] F[ ] K[ ] StartDataQualityRulesetEvaluationRun
- W[ ] S[ ] M[ ] F[ ] K[ ] StartExportLabelsTaskRun
- W[ ] S[ ] M[ ] F[ ] K[ ] StartImportLabelsTaskRun
- W[x] S[ ] M[x] F[ ] K[x] StartJobRun
- W[ ] S[ ] M[ ] F[ ] K[ ] StartMLEvaluationTaskRun
- W[ ] S[ ] M[ ] F[ ] K[ ] StartMLLabelingSetGenerationTaskRun
- W[ ] S[ ] M[ ] F[ ] K[ ] StartMaterializedViewRefreshTaskRun
- W[x] S[ ] M[x] F[ ] K[ ] StartTrigger
- W[x] S[ ] M[x] F[ ] K[ ] StartWorkflowRun
- W[ ] S[ ] M[ ] F[ ] K[ ] StopColumnStatisticsTaskRun
- W[ ] S[ ] M[ ] F[ ] K[ ] StopColumnStatisticsTaskRunSchedule
- W[x] S[ ] M[x] F[ ] K[ ] StopCrawler
- W[x] S[ ] M[ ] F[ ] K[ ] StopCrawlerSchedule
- W[ ] S[ ] M[ ] F[ ] K[ ] StopMaterializedViewRefreshTaskRun
- W[x] S[ ] M[x] F[ ] K[ ] StopSession
- W[x] S[ ] M[x] F[ ] K[ ] StopTrigger
- W[x] S[ ] M[x] F[ ] K[ ] StopWorkflowRun
- W[x] S[ ] M[x] F[ ] K[x] TagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] TestConnection
- W[x] S[ ] M[x] F[ ] K[x] UntagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateBlueprint
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateCatalog
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateClassifier
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateColumnStatisticsForPartition
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateColumnStatisticsForTable
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateColumnStatisticsTaskSettings
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateConnection
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateCrawler
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateCrawlerSchedule
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateDataQualityRuleset
- W[x] S[ ] M[x] F[ ] K[ ] UpdateDatabase
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateDevEndpoint
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateGlueIdentityCenterConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateIntegrationResourceProperty
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateIntegrationTableProperties
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateJob
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateJobFromSourceControl
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateMLTransform
- W[x] S[ ] M[x] F[ ] K[ ] UpdatePartition
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateRegistry
- W[x] S[ ] M[x] F[ ] K[ ] UpdateSchema
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateSourceControlFromJob
- W[x] S[ ] M[x] F[ ] K[ ] UpdateTable
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateTableOptimizer
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateTrigger
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateUsageProfile
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateUserDefinedFunction
- W[x] S[ ] M[x] F[ ] K[ ] UpdateWorkflow

Integration tests: 127/132 implemented operations tested (96.2%)
Untested implemented operations: 5

### winterbaume-greengrass (greengrass) - W: 71/92, S: 0/92, M: 55/92, F: 0/92, K: 0/92

- W[x] S[ ] M[x] F[ ] K[ ] AssociateRoleToGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] AssociateServiceRoleToAccount
- W[x] S[ ] M[ ] F[ ] K[ ] CreateConnectorDefinition
- W[x] S[ ] M[ ] F[ ] K[ ] CreateConnectorDefinitionVersion
- W[x] S[ ] M[x] F[ ] K[ ] CreateCoreDefinition
- W[x] S[ ] M[x] F[ ] K[ ] CreateCoreDefinitionVersion
- W[x] S[ ] M[x] F[ ] K[ ] CreateDeployment
- W[x] S[ ] M[x] F[ ] K[ ] CreateDeviceDefinition
- W[x] S[ ] M[x] F[ ] K[ ] CreateDeviceDefinitionVersion
- W[x] S[ ] M[x] F[ ] K[ ] CreateFunctionDefinition
- W[x] S[ ] M[x] F[ ] K[ ] CreateFunctionDefinitionVersion
- W[x] S[ ] M[x] F[ ] K[ ] CreateGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateGroupCertificateAuthority
- W[x] S[ ] M[x] F[ ] K[ ] CreateGroupVersion
- W[x] S[ ] M[ ] F[ ] K[ ] CreateLoggerDefinition
- W[x] S[ ] M[ ] F[ ] K[ ] CreateLoggerDefinitionVersion
- W[x] S[ ] M[x] F[ ] K[ ] CreateResourceDefinition
- W[x] S[ ] M[x] F[ ] K[ ] CreateResourceDefinitionVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateSoftwareUpdateJob
- W[x] S[ ] M[x] F[ ] K[ ] CreateSubscriptionDefinition
- W[x] S[ ] M[x] F[ ] K[ ] CreateSubscriptionDefinitionVersion
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteConnectorDefinition
- W[x] S[ ] M[x] F[ ] K[ ] DeleteCoreDefinition
- W[x] S[ ] M[x] F[ ] K[ ] DeleteDeviceDefinition
- W[x] S[ ] M[x] F[ ] K[ ] DeleteFunctionDefinition
- W[x] S[ ] M[x] F[ ] K[ ] DeleteGroup
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteLoggerDefinition
- W[x] S[ ] M[x] F[ ] K[ ] DeleteResourceDefinition
- W[x] S[ ] M[x] F[ ] K[ ] DeleteSubscriptionDefinition
- W[x] S[ ] M[x] F[ ] K[ ] DisassociateRoleFromGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] DisassociateServiceRoleFromAccount
- W[x] S[ ] M[x] F[ ] K[ ] GetAssociatedRole
- W[ ] S[ ] M[ ] F[ ] K[ ] GetBulkDeploymentStatus
- W[ ] S[ ] M[ ] F[ ] K[ ] GetConnectivityInfo
- W[x] S[ ] M[ ] F[ ] K[ ] GetConnectorDefinition
- W[x] S[ ] M[ ] F[ ] K[ ] GetConnectorDefinitionVersion
- W[x] S[ ] M[x] F[ ] K[ ] GetCoreDefinition
- W[x] S[ ] M[x] F[ ] K[ ] GetCoreDefinitionVersion
- W[x] S[ ] M[x] F[ ] K[ ] GetDeploymentStatus
- W[x] S[ ] M[x] F[ ] K[ ] GetDeviceDefinition
- W[x] S[ ] M[x] F[ ] K[ ] GetDeviceDefinitionVersion
- W[x] S[ ] M[x] F[ ] K[ ] GetFunctionDefinition
- W[x] S[ ] M[x] F[ ] K[ ] GetFunctionDefinitionVersion
- W[x] S[ ] M[x] F[ ] K[ ] GetGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] GetGroupCertificateAuthority
- W[ ] S[ ] M[ ] F[ ] K[ ] GetGroupCertificateConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] GetGroupVersion
- W[x] S[ ] M[ ] F[ ] K[ ] GetLoggerDefinition
- W[x] S[ ] M[ ] F[ ] K[ ] GetLoggerDefinitionVersion
- W[x] S[ ] M[x] F[ ] K[ ] GetResourceDefinition
- W[x] S[ ] M[x] F[ ] K[ ] GetResourceDefinitionVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] GetServiceRoleForAccount
- W[x] S[ ] M[x] F[ ] K[ ] GetSubscriptionDefinition
- W[x] S[ ] M[x] F[ ] K[ ] GetSubscriptionDefinitionVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] GetThingRuntimeConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] ListBulkDeploymentDetailedReports
- W[ ] S[ ] M[ ] F[ ] K[ ] ListBulkDeployments
- W[x] S[ ] M[ ] F[ ] K[ ] ListConnectorDefinitionVersions
- W[x] S[ ] M[ ] F[ ] K[ ] ListConnectorDefinitions
- W[x] S[ ] M[x] F[ ] K[ ] ListCoreDefinitionVersions
- W[x] S[ ] M[x] F[ ] K[ ] ListCoreDefinitions
- W[x] S[ ] M[x] F[ ] K[ ] ListDeployments
- W[x] S[ ] M[x] F[ ] K[ ] ListDeviceDefinitionVersions
- W[x] S[ ] M[x] F[ ] K[ ] ListDeviceDefinitions
- W[x] S[ ] M[x] F[ ] K[ ] ListFunctionDefinitionVersions
- W[x] S[ ] M[x] F[ ] K[ ] ListFunctionDefinitions
- W[ ] S[ ] M[ ] F[ ] K[ ] ListGroupCertificateAuthorities
- W[x] S[ ] M[x] F[ ] K[ ] ListGroupVersions
- W[x] S[ ] M[x] F[ ] K[ ] ListGroups
- W[x] S[ ] M[ ] F[ ] K[ ] ListLoggerDefinitionVersions
- W[x] S[ ] M[ ] F[ ] K[ ] ListLoggerDefinitions
- W[x] S[ ] M[x] F[ ] K[ ] ListResourceDefinitionVersions
- W[x] S[ ] M[x] F[ ] K[ ] ListResourceDefinitions
- W[x] S[ ] M[x] F[ ] K[ ] ListSubscriptionDefinitionVersions
- W[x] S[ ] M[x] F[ ] K[ ] ListSubscriptionDefinitions
- W[ ] S[ ] M[ ] F[ ] K[ ] ListTagsForResource
- W[x] S[ ] M[x] F[ ] K[ ] ResetDeployments
- W[ ] S[ ] M[ ] F[ ] K[ ] StartBulkDeployment
- W[ ] S[ ] M[ ] F[ ] K[ ] StopBulkDeployment
- W[ ] S[ ] M[ ] F[ ] K[ ] TagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] UntagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateConnectivityInfo
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateConnectorDefinition
- W[x] S[ ] M[x] F[ ] K[ ] UpdateCoreDefinition
- W[x] S[ ] M[x] F[ ] K[ ] UpdateDeviceDefinition
- W[x] S[ ] M[x] F[ ] K[ ] UpdateFunctionDefinition
- W[x] S[ ] M[x] F[ ] K[ ] UpdateGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateGroupCertificateConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateLoggerDefinition
- W[x] S[ ] M[x] F[ ] K[ ] UpdateResourceDefinition
- W[x] S[ ] M[x] F[ ] K[ ] UpdateSubscriptionDefinition
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateThingRuntimeConfiguration

Integration tests: 71/71 implemented operations tested (100.0%)

### winterbaume-guardduty (guardduty) - W: 85/87, S: 2/87, M: 12/87, F: 0/87, K: 0/87

- W[x] S[ ] M[ ] F[ ] K[ ] AcceptAdministratorInvitation
- W[x] S[ ] M[ ] F[ ] K[ ] AcceptInvitation
- W[x] S[ ] M[ ] F[ ] K[ ] ArchiveFindings
- W[x] S[ ] M[x] F[ ] K[ ] CreateDetector
- W[x] S[ ] M[x] F[ ] K[ ] CreateFilter
- W[x] S[ ] M[ ] F[ ] K[ ] CreateIPSet
- W[x] S[ ] M[ ] F[ ] K[ ] CreateMalwareProtectionPlan
- W[x] S[ ] M[ ] F[ ] K[ ] CreateMembers
- W[x] S[ ] M[ ] F[ ] K[ ] CreatePublishingDestination
- W[x] S[ ] M[ ] F[ ] K[ ] CreateSampleFindings
- W[x] S[ ] M[ ] F[ ] K[ ] CreateThreatEntitySet
- W[x] S[ ] M[ ] F[ ] K[ ] CreateThreatIntelSet
- W[x] S[ ] M[ ] F[ ] K[ ] CreateTrustedEntitySet
- W[x] S[ ] M[ ] F[ ] K[ ] DeclineInvitations
- W[x] S[ ] M[x] F[ ] K[ ] DeleteDetector
- W[x] S[ ] M[x] F[ ] K[ ] DeleteFilter
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteIPSet
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteInvitations
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteMalwareProtectionPlan
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteMembers
- W[x] S[ ] M[ ] F[ ] K[ ] DeletePublishingDestination
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteThreatEntitySet
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteThreatIntelSet
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteTrustedEntitySet
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeMalwareScans
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeOrganizationConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] DescribePublishingDestination
- W[x] S[ ] M[ ] F[ ] K[ ] DisableOrganizationAdminAccount
- W[x] S[ ] M[ ] F[ ] K[ ] DisassociateFromAdministratorAccount
- W[x] S[ ] M[ ] F[ ] K[ ] DisassociateFromMasterAccount
- W[x] S[ ] M[ ] F[ ] K[ ] DisassociateMembers
- W[x] S[ ] M[x] F[ ] K[ ] EnableOrganizationAdminAccount
- W[x] S[ ] M[x] F[ ] K[ ] GetAdministratorAccount
- W[x] S[ ] M[ ] F[ ] K[ ] GetCoverageStatistics
- W[x] S[ ] M[x] F[ ] K[ ] GetDetector
- W[x] S[ ] M[x] F[ ] K[ ] GetFilter
- W[x] S[ ] M[ ] F[ ] K[ ] GetFindings
- W[x] S[ ] M[ ] F[ ] K[ ] GetFindingsStatistics
- W[x] S[ ] M[ ] F[ ] K[ ] GetIPSet
- W[x] S[ ] M[ ] F[ ] K[ ] GetInvitationsCount
- W[x] S[ ] M[ ] F[ ] K[ ] GetMalwareProtectionPlan
- W[x] S[ ] M[ ] F[ ] K[ ] GetMalwareScan
- W[x] S[ ] M[ ] F[ ] K[ ] GetMalwareScanSettings
- W[x] S[ ] M[ ] F[ ] K[ ] GetMasterAccount
- W[x] S[ ] M[ ] F[ ] K[ ] GetMemberDetectors
- W[x] S[ ] M[ ] F[ ] K[ ] GetMembers
- W[x] S[ ] M[ ] F[ ] K[ ] GetOrganizationStatistics
- W[ ] S[x] M[ ] F[ ] K[ ] GetRemainingFreeTrialDays
- W[x] S[ ] M[ ] F[ ] K[ ] GetThreatEntitySet
- W[x] S[ ] M[ ] F[ ] K[ ] GetThreatIntelSet
- W[x] S[ ] M[ ] F[ ] K[ ] GetTrustedEntitySet
- W[ ] S[x] M[ ] F[ ] K[ ] GetUsageStatistics
- W[x] S[ ] M[ ] F[ ] K[ ] InviteMembers
- W[x] S[ ] M[ ] F[ ] K[ ] ListCoverage
- W[x] S[ ] M[x] F[ ] K[ ] ListDetectors
- W[x] S[ ] M[ ] F[ ] K[ ] ListFilters
- W[x] S[ ] M[ ] F[ ] K[ ] ListFindings
- W[x] S[ ] M[ ] F[ ] K[ ] ListIPSets
- W[x] S[ ] M[ ] F[ ] K[ ] ListInvitations
- W[x] S[ ] M[ ] F[ ] K[ ] ListMalwareProtectionPlans
- W[x] S[ ] M[ ] F[ ] K[ ] ListMalwareScans
- W[x] S[ ] M[ ] F[ ] K[ ] ListMembers
- W[x] S[ ] M[x] F[ ] K[ ] ListOrganizationAdminAccounts
- W[x] S[ ] M[ ] F[ ] K[ ] ListPublishingDestinations
- W[x] S[ ] M[ ] F[ ] K[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] ListThreatEntitySets
- W[x] S[ ] M[ ] F[ ] K[ ] ListThreatIntelSets
- W[x] S[ ] M[ ] F[ ] K[ ] ListTrustedEntitySets
- W[x] S[ ] M[ ] F[ ] K[ ] SendObjectMalwareScan
- W[x] S[ ] M[ ] F[ ] K[ ] StartMalwareScan
- W[x] S[ ] M[ ] F[ ] K[ ] StartMonitoringMembers
- W[x] S[ ] M[ ] F[ ] K[ ] StopMonitoringMembers
- W[x] S[ ] M[ ] F[ ] K[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UnarchiveFindings
- W[x] S[ ] M[ ] F[ ] K[ ] UntagResource
- W[x] S[ ] M[x] F[ ] K[ ] UpdateDetector
- W[x] S[ ] M[x] F[ ] K[ ] UpdateFilter
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateFindingsFeedback
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateIPSet
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateMalwareProtectionPlan
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateMalwareScanSettings
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateMemberDetectors
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateOrganizationConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] UpdatePublishingDestination
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateThreatEntitySet
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateThreatIntelSet
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateTrustedEntitySet

Integration tests: 78/85 implemented operations tested (91.8%)
Untested implemented operations: 7

### winterbaume-iam (iam) - W: 154/176, S: 22/176, M: 119/176, F: 0/176, K: 39/176

Terraform E2E: 30 tests across 11 terraform resource types

Resource types: aws_iam_access_key, aws_iam_group, aws_iam_group_membership, aws_iam_group_policy_attachment, aws_iam_instance_profile, aws_iam_policy, aws_iam_role, aws_iam_role_policy, aws_iam_role_policy_attachment, aws_iam_user, aws_iam_user_policy_attachment

- W[ ] S[x] M[ ] F[ ] K[ ] AcceptDelegationRequest
- W[x] S[ ] M[ ] F[ ] K[ ] AddClientIDToOpenIDConnectProvider
- W[x] S[ ] M[x] F[ ] K[x] AddRoleToInstanceProfile
- W[x] S[ ] M[x] F[ ] K[ ] AddUserToGroup
- W[ ] S[x] M[ ] F[ ] K[ ] AssociateDelegationRequest
- W[x] S[ ] M[x] F[ ] K[ ] AttachGroupPolicy
- W[x] S[ ] M[x] F[ ] K[x] AttachRolePolicy
- W[x] S[ ] M[x] F[ ] K[x] AttachUserPolicy
- W[ ] S[x] M[ ] F[ ] K[ ] ChangePassword
- W[x] S[ ] M[x] F[ ] K[x] CreateAccessKey
- W[x] S[ ] M[x] F[ ] K[ ] CreateAccountAlias
- W[ ] S[x] M[ ] F[ ] K[ ] CreateDelegationRequest
- W[x] S[ ] M[x] F[ ] K[ ] CreateGroup
- W[x] S[ ] M[x] F[ ] K[x] CreateInstanceProfile
- W[x] S[ ] M[x] F[ ] K[ ] CreateLoginProfile
- W[x] S[ ] M[x] F[ ] K[x] CreateOpenIDConnectProvider
- W[x] S[ ] M[x] F[ ] K[x] CreatePolicy
- W[x] S[ ] M[x] F[ ] K[ ] CreatePolicyVersion
- W[x] S[ ] M[x] F[ ] K[x] CreateRole
- W[x] S[ ] M[x] F[ ] K[ ] CreateSAMLProvider
- W[x] S[ ] M[x] F[ ] K[ ] CreateServiceLinkedRole
- W[x] S[ ] M[ ] F[ ] K[ ] CreateServiceSpecificCredential
- W[x] S[ ] M[x] F[ ] K[x] CreateUser
- W[x] S[ ] M[x] F[ ] K[ ] CreateVirtualMFADevice
- W[x] S[ ] M[x] F[ ] K[ ] DeactivateMFADevice
- W[x] S[ ] M[x] F[ ] K[x] DeleteAccessKey
- W[x] S[ ] M[x] F[ ] K[ ] DeleteAccountAlias
- W[x] S[ ] M[x] F[ ] K[ ] DeleteAccountPasswordPolicy
- W[x] S[ ] M[x] F[ ] K[ ] DeleteGroup
- W[x] S[ ] M[x] F[ ] K[ ] DeleteGroupPolicy
- W[x] S[ ] M[x] F[ ] K[x] DeleteInstanceProfile
- W[x] S[ ] M[x] F[ ] K[ ] DeleteLoginProfile
- W[x] S[ ] M[x] F[ ] K[x] DeleteOpenIDConnectProvider
- W[x] S[ ] M[x] F[ ] K[x] DeletePolicy
- W[x] S[ ] M[x] F[ ] K[ ] DeletePolicyVersion
- W[x] S[ ] M[x] F[ ] K[x] DeleteRole
- W[x] S[ ] M[x] F[ ] K[ ] DeleteRolePermissionsBoundary
- W[x] S[ ] M[x] F[ ] K[x] DeleteRolePolicy
- W[x] S[ ] M[x] F[ ] K[ ] DeleteSAMLProvider
- W[x] S[ ] M[x] F[ ] K[ ] DeleteSSHPublicKey
- W[x] S[ ] M[x] F[ ] K[ ] DeleteServerCertificate
- W[x] S[ ] M[x] F[ ] K[ ] DeleteServiceLinkedRole
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteServiceSpecificCredential
- W[x] S[ ] M[x] F[ ] K[ ] DeleteSigningCertificate
- W[x] S[ ] M[x] F[ ] K[x] DeleteUser
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteUserPermissionsBoundary
- W[x] S[ ] M[x] F[ ] K[ ] DeleteUserPolicy
- W[x] S[ ] M[x] F[ ] K[ ] DeleteVirtualMFADevice
- W[x] S[ ] M[x] F[ ] K[ ] DetachGroupPolicy
- W[x] S[ ] M[x] F[ ] K[x] DetachRolePolicy
- W[x] S[ ] M[x] F[ ] K[x] DetachUserPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] DisableOrganizationsRootCredentialsManagement
- W[x] S[ ] M[ ] F[ ] K[ ] DisableOrganizationsRootSessions
- W[ ] S[x] M[ ] F[ ] K[ ] DisableOutboundWebIdentityFederation
- W[x] S[ ] M[x] F[ ] K[ ] EnableMFADevice
- W[x] S[ ] M[ ] F[ ] K[ ] EnableOrganizationsRootCredentialsManagement
- W[x] S[ ] M[ ] F[ ] K[ ] EnableOrganizationsRootSessions
- W[ ] S[x] M[ ] F[ ] K[ ] EnableOutboundWebIdentityFederation
- W[x] S[ ] M[ ] F[ ] K[ ] GenerateCredentialReport
- W[ ] S[x] M[ ] F[ ] K[ ] GenerateOrganizationsAccessReport
- W[ ] S[x] M[ ] F[ ] K[ ] GenerateServiceLastAccessedDetails
- W[x] S[ ] M[x] F[ ] K[ ] GetAccessKeyLastUsed
- W[x] S[ ] M[x] F[ ] K[ ] GetAccountAuthorizationDetails
- W[x] S[ ] M[x] F[ ] K[ ] GetAccountPasswordPolicy
- W[x] S[ ] M[x] F[ ] K[ ] GetAccountSummary
- W[ ] S[x] M[ ] F[ ] K[ ] GetContextKeysForCustomPolicy
- W[ ] S[x] M[ ] F[ ] K[ ] GetContextKeysForPrincipalPolicy
- W[x] S[ ] M[x] F[ ] K[ ] GetCredentialReport
- W[ ] S[x] M[ ] F[ ] K[ ] GetDelegationRequest
- W[x] S[ ] M[x] F[ ] K[ ] GetGroup
- W[x] S[ ] M[x] F[ ] K[ ] GetGroupPolicy
- W[ ] S[x] M[ ] F[ ] K[ ] GetHumanReadableSummary
- W[x] S[ ] M[x] F[ ] K[x] GetInstanceProfile
- W[x] S[ ] M[x] F[ ] K[ ] GetLoginProfile
- W[x] S[ ] M[ ] F[ ] K[ ] GetMFADevice
- W[x] S[ ] M[x] F[ ] K[x] GetOpenIDConnectProvider
- W[ ] S[x] M[ ] F[ ] K[ ] GetOrganizationsAccessReport
- W[ ] S[x] M[ ] F[ ] K[ ] GetOutboundWebIdentityFederationInfo
- W[x] S[ ] M[x] F[ ] K[x] GetPolicy
- W[x] S[ ] M[x] F[ ] K[ ] GetPolicyVersion
- W[x] S[ ] M[x] F[ ] K[x] GetRole
- W[x] S[ ] M[x] F[ ] K[x] GetRolePolicy
- W[x] S[ ] M[x] F[ ] K[ ] GetSAMLProvider
- W[x] S[ ] M[x] F[ ] K[ ] GetSSHPublicKey
- W[x] S[ ] M[x] F[ ] K[ ] GetServerCertificate
- W[ ] S[x] M[ ] F[ ] K[ ] GetServiceLastAccessedDetails
- W[ ] S[x] M[ ] F[ ] K[ ] GetServiceLastAccessedDetailsWithEntities
- W[x] S[ ] M[x] F[ ] K[ ] GetServiceLinkedRoleDeletionStatus
- W[x] S[ ] M[x] F[ ] K[x] GetUser
- W[x] S[ ] M[x] F[ ] K[ ] GetUserPolicy
- W[x] S[ ] M[x] F[ ] K[x] ListAccessKeys
- W[x] S[ ] M[x] F[ ] K[ ] ListAccountAliases
- W[x] S[ ] M[x] F[ ] K[ ] ListAttachedGroupPolicies
- W[x] S[ ] M[x] F[ ] K[x] ListAttachedRolePolicies
- W[x] S[ ] M[x] F[ ] K[ ] ListAttachedUserPolicies
- W[ ] S[x] M[ ] F[ ] K[ ] ListDelegationRequests
- W[x] S[ ] M[ ] F[ ] K[ ] ListEntitiesForPolicy
- W[x] S[ ] M[x] F[ ] K[ ] ListGroupPolicies
- W[x] S[ ] M[x] F[ ] K[ ] ListGroups
- W[x] S[ ] M[ ] F[ ] K[ ] ListGroupsForUser
- W[x] S[ ] M[ ] F[ ] K[ ] ListInstanceProfileTags
- W[x] S[ ] M[ ] F[ ] K[x] ListInstanceProfiles
- W[x] S[ ] M[ ] F[ ] K[x] ListInstanceProfilesForRole
- W[x] S[ ] M[ ] F[ ] K[ ] ListMFADeviceTags
- W[x] S[ ] M[x] F[ ] K[ ] ListMFADevices
- W[x] S[ ] M[x] F[ ] K[ ] ListOpenIDConnectProviderTags
- W[x] S[ ] M[x] F[ ] K[x] ListOpenIDConnectProviders
- W[ ] S[x] M[ ] F[ ] K[ ] ListOrganizationsFeatures
- W[x] S[ ] M[x] F[ ] K[x] ListPolicies
- W[ ] S[x] M[ ] F[ ] K[ ] ListPoliciesGrantingServiceAccess
- W[x] S[ ] M[x] F[ ] K[ ] ListPolicyTags
- W[x] S[ ] M[x] F[ ] K[ ] ListPolicyVersions
- W[x] S[ ] M[x] F[ ] K[x] ListRolePolicies
- W[x] S[ ] M[x] F[ ] K[ ] ListRoleTags
- W[x] S[ ] M[x] F[ ] K[x] ListRoles
- W[x] S[ ] M[ ] F[ ] K[ ] ListSAMLProviderTags
- W[x] S[ ] M[x] F[ ] K[ ] ListSAMLProviders
- W[x] S[ ] M[ ] F[ ] K[ ] ListSSHPublicKeys
- W[x] S[ ] M[ ] F[ ] K[ ] ListServerCertificateTags
- W[x] S[ ] M[x] F[ ] K[ ] ListServerCertificates
- W[x] S[ ] M[ ] F[ ] K[ ] ListServiceSpecificCredentials
- W[x] S[ ] M[x] F[ ] K[ ] ListSigningCertificates
- W[x] S[ ] M[x] F[ ] K[ ] ListUserPolicies
- W[x] S[ ] M[x] F[ ] K[ ] ListUserTags
- W[x] S[ ] M[x] F[ ] K[x] ListUsers
- W[x] S[ ] M[x] F[ ] K[ ] ListVirtualMFADevices
- W[x] S[ ] M[x] F[ ] K[ ] PutGroupPolicy
- W[x] S[ ] M[x] F[ ] K[ ] PutRolePermissionsBoundary
- W[x] S[ ] M[x] F[ ] K[x] PutRolePolicy
- W[x] S[ ] M[ ] F[ ] K[ ] PutUserPermissionsBoundary
- W[x] S[ ] M[x] F[ ] K[ ] PutUserPolicy
- W[ ] S[x] M[ ] F[ ] K[ ] RejectDelegationRequest
- W[x] S[ ] M[ ] F[ ] K[ ] RemoveClientIDFromOpenIDConnectProvider
- W[x] S[ ] M[x] F[ ] K[x] RemoveRoleFromInstanceProfile
- W[x] S[ ] M[x] F[ ] K[ ] RemoveUserFromGroup
- W[x] S[ ] M[ ] F[ ] K[ ] ResetServiceSpecificCredential
- W[x] S[ ] M[ ] F[ ] K[ ] ResyncMFADevice
- W[ ] S[x] M[ ] F[ ] K[ ] SendDelegationToken
- W[x] S[ ] M[x] F[ ] K[ ] SetDefaultPolicyVersion
- W[x] S[ ] M[ ] F[ ] K[ ] SetSecurityTokenServicePreferences
- W[x] S[ ] M[ ] F[ ] K[ ] SimulateCustomPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] SimulatePrincipalPolicy
- W[x] S[ ] M[x] F[ ] K[ ] TagInstanceProfile
- W[x] S[ ] M[ ] F[ ] K[ ] TagMFADevice
- W[x] S[ ] M[x] F[ ] K[ ] TagOpenIDConnectProvider
- W[x] S[ ] M[x] F[ ] K[ ] TagPolicy
- W[x] S[ ] M[x] F[ ] K[x] TagRole
- W[x] S[ ] M[ ] F[ ] K[ ] TagSAMLProvider
- W[x] S[ ] M[ ] F[ ] K[ ] TagServerCertificate
- W[x] S[ ] M[x] F[ ] K[ ] TagUser
- W[x] S[ ] M[x] F[ ] K[ ] UntagInstanceProfile
- W[x] S[ ] M[ ] F[ ] K[ ] UntagMFADevice
- W[x] S[ ] M[x] F[ ] K[ ] UntagOpenIDConnectProvider
- W[x] S[ ] M[x] F[ ] K[ ] UntagPolicy
- W[x] S[ ] M[x] F[ ] K[ ] UntagRole
- W[x] S[ ] M[ ] F[ ] K[ ] UntagSAMLProvider
- W[x] S[ ] M[ ] F[ ] K[ ] UntagServerCertificate
- W[x] S[ ] M[x] F[ ] K[ ] UntagUser
- W[x] S[ ] M[x] F[ ] K[ ] UpdateAccessKey
- W[x] S[ ] M[x] F[ ] K[ ] UpdateAccountPasswordPolicy
- W[x] S[ ] M[x] F[ ] K[x] UpdateAssumeRolePolicy
- W[ ] S[x] M[ ] F[ ] K[ ] UpdateDelegationRequest
- W[x] S[ ] M[x] F[ ] K[ ] UpdateGroup
- W[x] S[ ] M[x] F[ ] K[ ] UpdateLoginProfile
- W[x] S[ ] M[x] F[ ] K[x] UpdateOpenIDConnectProviderThumbprint
- W[x] S[ ] M[x] F[ ] K[x] UpdateRole
- W[x] S[ ] M[x] F[ ] K[ ] UpdateRoleDescription
- W[x] S[ ] M[x] F[ ] K[ ] UpdateSAMLProvider
- W[x] S[ ] M[x] F[ ] K[ ] UpdateSSHPublicKey
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateServerCertificate
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateServiceSpecificCredential
- W[x] S[ ] M[x] F[ ] K[ ] UpdateSigningCertificate
- W[x] S[ ] M[x] F[ ] K[ ] UpdateUser
- W[x] S[ ] M[x] F[ ] K[ ] UploadSSHPublicKey
- W[x] S[ ] M[x] F[ ] K[ ] UploadServerCertificate
- W[x] S[ ] M[x] F[ ] K[ ] UploadSigningCertificate

Integration tests: 135/154 implemented operations tested (87.7%)
Untested implemented operations: 19

### winterbaume-identitystore (identitystore) - W: 17/19, S: 0/19, M: 14/19, F: 0/19, K: 0/19

- W[x] S[ ] M[x] F[ ] K[ ] CreateGroup
- W[x] S[ ] M[x] F[ ] K[ ] CreateGroupMembership
- W[x] S[ ] M[x] F[ ] K[ ] CreateUser
- W[x] S[ ] M[x] F[ ] K[ ] DeleteGroup
- W[x] S[ ] M[x] F[ ] K[ ] DeleteGroupMembership
- W[x] S[ ] M[x] F[ ] K[ ] DeleteUser
- W[x] S[ ] M[x] F[ ] K[ ] DescribeGroup
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeGroupMembership
- W[x] S[ ] M[x] F[ ] K[ ] DescribeUser
- W[x] S[ ] M[x] F[ ] K[ ] GetGroupId
- W[ ] S[ ] M[ ] F[ ] K[ ] GetGroupMembershipId
- W[x] S[ ] M[x] F[ ] K[ ] GetUserId
- W[ ] S[ ] M[ ] F[ ] K[ ] IsMemberInGroups
- W[x] S[ ] M[x] F[ ] K[ ] ListGroupMemberships
- W[x] S[ ] M[x] F[ ] K[ ] ListGroupMembershipsForMember
- W[x] S[ ] M[x] F[ ] K[ ] ListGroups
- W[x] S[ ] M[x] F[ ] K[ ] ListUsers
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateGroup
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateUser

Integration tests: 14/17 implemented operations tested (82.4%)
Untested implemented operations: 3

### winterbaume-inspector2 (inspector2) - W: 49/75, S: 21/75, M: 19/75, F: 0/75, K: 0/75

- W[x] S[ ] M[x] F[ ] K[ ] AssociateMember
- W[x] S[ ] M[ ] F[ ] K[ ] BatchAssociateCodeSecurityScanConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] BatchDisassociateCodeSecurityScanConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] BatchGetAccountStatus
- W[ ] S[x] M[ ] F[ ] K[ ] BatchGetCodeSnippet
- W[ ] S[x] M[ ] F[ ] K[ ] BatchGetFindingDetails
- W[ ] S[x] M[ ] F[ ] K[ ] BatchGetFreeTrialInfo
- W[ ] S[ ] M[ ] F[ ] K[ ] BatchGetMemberEc2DeepInspectionStatus
- W[ ] S[ ] M[ ] F[ ] K[ ] BatchUpdateMemberEc2DeepInspectionStatus
- W[x] S[ ] M[ ] F[ ] K[ ] CancelFindingsReport
- W[x] S[ ] M[ ] F[ ] K[ ] CancelSbomExport
- W[x] S[ ] M[ ] F[ ] K[ ] CreateCisScanConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] CreateCodeSecurityIntegration
- W[x] S[ ] M[ ] F[ ] K[ ] CreateCodeSecurityScanConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] CreateFilter
- W[x] S[ ] M[ ] F[ ] K[ ] CreateFindingsReport
- W[x] S[ ] M[ ] F[ ] K[ ] CreateSbomExport
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteCisScanConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteCodeSecurityIntegration
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteCodeSecurityScanConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] DeleteFilter
- W[x] S[ ] M[x] F[ ] K[ ] DescribeOrganizationConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] Disable
- W[x] S[ ] M[x] F[ ] K[ ] DisableDelegatedAdminAccount
- W[x] S[ ] M[x] F[ ] K[ ] DisassociateMember
- W[x] S[ ] M[x] F[ ] K[ ] Enable
- W[x] S[ ] M[x] F[ ] K[ ] EnableDelegatedAdminAccount
- W[ ] S[x] M[ ] F[ ] K[ ] GetCisScanReport
- W[ ] S[x] M[ ] F[ ] K[ ] GetCisScanResultDetails
- W[ ] S[x] M[ ] F[ ] K[ ] GetClustersForImage
- W[x] S[ ] M[ ] F[ ] K[ ] GetCodeSecurityIntegration
- W[ ] S[x] M[ ] F[ ] K[ ] GetCodeSecurityScan
- W[x] S[ ] M[ ] F[ ] K[ ] GetCodeSecurityScanConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] GetConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] GetDelegatedAdminAccount
- W[ ] S[ ] M[ ] F[ ] K[ ] GetEc2DeepInspectionConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] GetEncryptionKey
- W[x] S[ ] M[ ] F[ ] K[ ] GetFindingsReportStatus
- W[x] S[ ] M[x] F[ ] K[ ] GetMember
- W[x] S[ ] M[ ] F[ ] K[ ] GetSbomExport
- W[ ] S[x] M[ ] F[ ] K[ ] ListAccountPermissions
- W[x] S[ ] M[ ] F[ ] K[ ] ListCisScanConfigurations
- W[ ] S[x] M[ ] F[ ] K[ ] ListCisScanResultsAggregatedByChecks
- W[ ] S[x] M[ ] F[ ] K[ ] ListCisScanResultsAggregatedByTargetResource
- W[ ] S[x] M[ ] F[ ] K[ ] ListCisScans
- W[x] S[ ] M[ ] F[ ] K[ ] ListCodeSecurityIntegrations
- W[x] S[ ] M[ ] F[ ] K[ ] ListCodeSecurityScanConfigurationAssociations
- W[x] S[ ] M[ ] F[ ] K[ ] ListCodeSecurityScanConfigurations
- W[ ] S[x] M[ ] F[ ] K[ ] ListCoverage
- W[ ] S[x] M[ ] F[ ] K[ ] ListCoverageStatistics
- W[x] S[ ] M[x] F[ ] K[ ] ListDelegatedAdminAccounts
- W[x] S[ ] M[x] F[ ] K[ ] ListFilters
- W[ ] S[x] M[ ] F[ ] K[ ] ListFindingAggregations
- W[x] S[ ] M[x] F[ ] K[ ] ListFindings
- W[x] S[ ] M[x] F[ ] K[ ] ListMembers
- W[x] S[ ] M[x] F[ ] K[ ] ListTagsForResource
- W[ ] S[x] M[ ] F[ ] K[ ] ListUsageTotals
- W[x] S[ ] M[ ] F[ ] K[ ] ResetEncryptionKey
- W[ ] S[x] M[ ] F[ ] K[ ] SearchVulnerabilities
- W[ ] S[x] M[ ] F[ ] K[ ] SendCisSessionHealth
- W[ ] S[x] M[ ] F[ ] K[ ] SendCisSessionTelemetry
- W[ ] S[x] M[ ] F[ ] K[ ] StartCisSession
- W[ ] S[x] M[ ] F[ ] K[ ] StartCodeSecurityScan
- W[ ] S[x] M[ ] F[ ] K[ ] StopCisSession
- W[x] S[ ] M[x] F[ ] K[ ] TagResource
- W[x] S[ ] M[x] F[ ] K[ ] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateCisScanConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateCodeSecurityIntegration
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateCodeSecurityScanConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateEc2DeepInspectionConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateEncryptionKey
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateFilter
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateOrgEc2DeepInspectionConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] UpdateOrganizationConfiguration

Integration tests: 38/49 implemented operations tested (77.6%)
Untested implemented operations: 11

### winterbaume-iot (iot) - W: 103/272, S: 0/272, M: 100/272, F: 0/272, K: 0/272

- W[ ] S[ ] M[ ] F[ ] K[ ] AcceptCertificateTransfer
- W[x] S[ ] M[x] F[ ] K[ ] AddThingToBillingGroup
- W[x] S[ ] M[x] F[ ] K[ ] AddThingToThingGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] AssociateSbomWithPackageVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] AssociateTargetsWithJob
- W[x] S[ ] M[x] F[ ] K[ ] AttachPolicy
- W[x] S[ ] M[x] F[ ] K[ ] AttachPrincipalPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] AttachSecurityProfile
- W[x] S[ ] M[x] F[ ] K[ ] AttachThingPrincipal
- W[ ] S[ ] M[ ] F[ ] K[ ] CancelAuditMitigationActionsTask
- W[ ] S[ ] M[ ] F[ ] K[ ] CancelAuditTask
- W[ ] S[ ] M[ ] F[ ] K[ ] CancelCertificateTransfer
- W[ ] S[ ] M[ ] F[ ] K[ ] CancelDetectMitigationActionsTask
- W[x] S[ ] M[x] F[ ] K[ ] CancelJob
- W[x] S[ ] M[x] F[ ] K[ ] CancelJobExecution
- W[ ] S[ ] M[ ] F[ ] K[ ] ClearDefaultAuthorizer
- W[ ] S[ ] M[ ] F[ ] K[ ] ConfirmTopicRuleDestination
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateAuditSuppression
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateAuthorizer
- W[x] S[ ] M[x] F[ ] K[ ] CreateBillingGroup
- W[x] S[ ] M[x] F[ ] K[ ] CreateCertificateFromCsr
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateCertificateProvider
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateCommand
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateCustomMetric
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateDimension
- W[x] S[ ] M[x] F[ ] K[ ] CreateDomainConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateDynamicThingGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateFleetMetric
- W[x] S[ ] M[x] F[ ] K[ ] CreateJob
- W[x] S[ ] M[x] F[ ] K[ ] CreateJobTemplate
- W[x] S[ ] M[x] F[ ] K[ ] CreateKeysAndCertificate
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateMitigationAction
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateOTAUpdate
- W[ ] S[ ] M[ ] F[ ] K[ ] CreatePackage
- W[ ] S[ ] M[ ] F[ ] K[ ] CreatePackageVersion
- W[x] S[ ] M[x] F[ ] K[ ] CreatePolicy
- W[x] S[ ] M[x] F[ ] K[ ] CreatePolicyVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateProvisioningClaim
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateProvisioningTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateProvisioningTemplateVersion
- W[x] S[ ] M[x] F[ ] K[ ] CreateRoleAlias
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateScheduledAudit
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateSecurityProfile
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateStream
- W[x] S[ ] M[x] F[ ] K[ ] CreateThing
- W[x] S[ ] M[x] F[ ] K[ ] CreateThingGroup
- W[x] S[ ] M[x] F[ ] K[ ] CreateThingType
- W[x] S[ ] M[x] F[ ] K[ ] CreateTopicRule
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateTopicRuleDestination
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteAccountAuditConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteAuditSuppression
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteAuthorizer
- W[x] S[ ] M[x] F[ ] K[ ] DeleteBillingGroup
- W[x] S[ ] M[x] F[ ] K[ ] DeleteCACertificate
- W[x] S[ ] M[x] F[ ] K[ ] DeleteCertificate
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteCertificateProvider
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteCommand
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteCommandExecution
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteCustomMetric
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteDimension
- W[x] S[ ] M[x] F[ ] K[ ] DeleteDomainConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteDynamicThingGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteFleetMetric
- W[x] S[ ] M[x] F[ ] K[ ] DeleteJob
- W[x] S[ ] M[x] F[ ] K[ ] DeleteJobExecution
- W[x] S[ ] M[x] F[ ] K[ ] DeleteJobTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteMitigationAction
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteOTAUpdate
- W[ ] S[ ] M[ ] F[ ] K[ ] DeletePackage
- W[ ] S[ ] M[ ] F[ ] K[ ] DeletePackageVersion
- W[x] S[ ] M[x] F[ ] K[ ] DeletePolicy
- W[x] S[ ] M[x] F[ ] K[ ] DeletePolicyVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteProvisioningTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteProvisioningTemplateVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteRegistrationCode
- W[x] S[ ] M[x] F[ ] K[ ] DeleteRoleAlias
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteScheduledAudit
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteSecurityProfile
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteStream
- W[x] S[ ] M[x] F[ ] K[ ] DeleteThing
- W[x] S[ ] M[x] F[ ] K[ ] DeleteThingGroup
- W[x] S[ ] M[x] F[ ] K[ ] DeleteThingType
- W[x] S[ ] M[x] F[ ] K[ ] DeleteTopicRule
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteTopicRuleDestination
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteV2LoggingLevel
- W[x] S[ ] M[x] F[ ] K[ ] DeprecateThingType
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeAccountAuditConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeAuditFinding
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeAuditMitigationActionsTask
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeAuditSuppression
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeAuditTask
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeAuthorizer
- W[x] S[ ] M[x] F[ ] K[ ] DescribeBillingGroup
- W[x] S[ ] M[x] F[ ] K[ ] DescribeCACertificate
- W[x] S[ ] M[x] F[ ] K[ ] DescribeCertificate
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeCertificateProvider
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeCustomMetric
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeDefaultAuthorizer
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeDetectMitigationActionsTask
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeDimension
- W[x] S[ ] M[x] F[ ] K[ ] DescribeDomainConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeEncryptionConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] DescribeEndpoint
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeEventConfigurations
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeFleetMetric
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeIndex
- W[x] S[ ] M[x] F[ ] K[ ] DescribeJob
- W[x] S[ ] M[x] F[ ] K[ ] DescribeJobExecution
- W[x] S[ ] M[x] F[ ] K[ ] DescribeJobTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeManagedJobTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeMitigationAction
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeProvisioningTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeProvisioningTemplateVersion
- W[x] S[ ] M[x] F[ ] K[ ] DescribeRoleAlias
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeScheduledAudit
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeSecurityProfile
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeStream
- W[x] S[ ] M[x] F[ ] K[ ] DescribeThing
- W[x] S[ ] M[x] F[ ] K[ ] DescribeThingGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeThingRegistrationTask
- W[x] S[ ] M[x] F[ ] K[ ] DescribeThingType
- W[x] S[ ] M[x] F[ ] K[ ] DetachPolicy
- W[x] S[ ] M[x] F[ ] K[ ] DetachPrincipalPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] DetachSecurityProfile
- W[x] S[ ] M[x] F[ ] K[ ] DetachThingPrincipal
- W[x] S[ ] M[x] F[ ] K[ ] DisableTopicRule
- W[ ] S[ ] M[ ] F[ ] K[ ] DisassociateSbomFromPackageVersion
- W[x] S[ ] M[x] F[ ] K[ ] EnableTopicRule
- W[ ] S[ ] M[ ] F[ ] K[ ] GetBehaviorModelTrainingSummaries
- W[ ] S[ ] M[ ] F[ ] K[ ] GetBucketsAggregation
- W[ ] S[ ] M[ ] F[ ] K[ ] GetCardinality
- W[ ] S[ ] M[ ] F[ ] K[ ] GetCommand
- W[ ] S[ ] M[ ] F[ ] K[ ] GetCommandExecution
- W[ ] S[ ] M[ ] F[ ] K[ ] GetEffectivePolicies
- W[x] S[ ] M[x] F[ ] K[ ] GetIndexingConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] GetJobDocument
- W[ ] S[ ] M[ ] F[ ] K[ ] GetLoggingOptions
- W[ ] S[ ] M[ ] F[ ] K[ ] GetOTAUpdate
- W[ ] S[ ] M[ ] F[ ] K[ ] GetPackage
- W[ ] S[ ] M[ ] F[ ] K[ ] GetPackageConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] GetPackageVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] GetPercentiles
- W[x] S[ ] M[x] F[ ] K[ ] GetPolicy
- W[x] S[ ] M[x] F[ ] K[ ] GetPolicyVersion
- W[x] S[ ] M[x] F[ ] K[ ] GetRegistrationCode
- W[ ] S[ ] M[ ] F[ ] K[ ] GetStatistics
- W[ ] S[ ] M[ ] F[ ] K[ ] GetThingConnectivityData
- W[x] S[ ] M[x] F[ ] K[ ] GetTopicRule
- W[ ] S[ ] M[ ] F[ ] K[ ] GetTopicRuleDestination
- W[ ] S[ ] M[ ] F[ ] K[ ] GetV2LoggingOptions
- W[ ] S[ ] M[ ] F[ ] K[ ] ListActiveViolations
- W[x] S[ ] M[x] F[ ] K[ ] ListAttachedPolicies
- W[ ] S[ ] M[ ] F[ ] K[ ] ListAuditFindings
- W[ ] S[ ] M[ ] F[ ] K[ ] ListAuditMitigationActionsExecutions
- W[ ] S[ ] M[ ] F[ ] K[ ] ListAuditMitigationActionsTasks
- W[ ] S[ ] M[ ] F[ ] K[ ] ListAuditSuppressions
- W[ ] S[ ] M[ ] F[ ] K[ ] ListAuditTasks
- W[ ] S[ ] M[ ] F[ ] K[ ] ListAuthorizers
- W[x] S[ ] M[x] F[ ] K[ ] ListBillingGroups
- W[ ] S[ ] M[ ] F[ ] K[ ] ListCACertificates
- W[ ] S[ ] M[ ] F[ ] K[ ] ListCertificateProviders
- W[x] S[ ] M[x] F[ ] K[ ] ListCertificates
- W[x] S[ ] M[x] F[ ] K[ ] ListCertificatesByCA
- W[ ] S[ ] M[ ] F[ ] K[ ] ListCommandExecutions
- W[ ] S[ ] M[ ] F[ ] K[ ] ListCommands
- W[ ] S[ ] M[ ] F[ ] K[ ] ListCustomMetrics
- W[ ] S[ ] M[ ] F[ ] K[ ] ListDetectMitigationActionsExecutions
- W[ ] S[ ] M[ ] F[ ] K[ ] ListDetectMitigationActionsTasks
- W[ ] S[ ] M[ ] F[ ] K[ ] ListDimensions
- W[x] S[ ] M[x] F[ ] K[ ] ListDomainConfigurations
- W[ ] S[ ] M[ ] F[ ] K[ ] ListFleetMetrics
- W[ ] S[ ] M[ ] F[ ] K[ ] ListIndices
- W[x] S[ ] M[x] F[ ] K[ ] ListJobExecutionsForJob
- W[x] S[ ] M[x] F[ ] K[ ] ListJobExecutionsForThing
- W[x] S[ ] M[x] F[ ] K[ ] ListJobTemplates
- W[x] S[ ] M[x] F[ ] K[ ] ListJobs
- W[ ] S[ ] M[ ] F[ ] K[ ] ListManagedJobTemplates
- W[ ] S[ ] M[ ] F[ ] K[ ] ListMetricValues
- W[ ] S[ ] M[ ] F[ ] K[ ] ListMitigationActions
- W[ ] S[ ] M[ ] F[ ] K[ ] ListOTAUpdates
- W[ ] S[ ] M[ ] F[ ] K[ ] ListOutgoingCertificates
- W[ ] S[ ] M[ ] F[ ] K[ ] ListPackageVersions
- W[ ] S[ ] M[ ] F[ ] K[ ] ListPackages
- W[x] S[ ] M[x] F[ ] K[ ] ListPolicies
- W[x] S[ ] M[x] F[ ] K[ ] ListPolicyPrincipals
- W[x] S[ ] M[x] F[ ] K[ ] ListPolicyVersions
- W[x] S[ ] M[x] F[ ] K[ ] ListPrincipalPolicies
- W[x] S[ ] M[x] F[ ] K[ ] ListPrincipalThings
- W[ ] S[ ] M[ ] F[ ] K[ ] ListPrincipalThingsV2
- W[ ] S[ ] M[ ] F[ ] K[ ] ListProvisioningTemplateVersions
- W[ ] S[ ] M[ ] F[ ] K[ ] ListProvisioningTemplates
- W[ ] S[ ] M[ ] F[ ] K[ ] ListRelatedResourcesForAuditFinding
- W[x] S[ ] M[x] F[ ] K[ ] ListRoleAliases
- W[ ] S[ ] M[ ] F[ ] K[ ] ListSbomValidationResults
- W[ ] S[ ] M[ ] F[ ] K[ ] ListScheduledAudits
- W[ ] S[ ] M[ ] F[ ] K[ ] ListSecurityProfiles
- W[ ] S[ ] M[ ] F[ ] K[ ] ListSecurityProfilesForTarget
- W[ ] S[ ] M[ ] F[ ] K[ ] ListStreams
- W[x] S[ ] M[ ] F[ ] K[ ] ListTagsForResource
- W[x] S[ ] M[x] F[ ] K[ ] ListTargetsForPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] ListTargetsForSecurityProfile
- W[x] S[ ] M[x] F[ ] K[ ] ListThingGroups
- W[x] S[ ] M[x] F[ ] K[ ] ListThingGroupsForThing
- W[x] S[ ] M[x] F[ ] K[ ] ListThingPrincipals
- W[x] S[ ] M[x] F[ ] K[ ] ListThingPrincipalsV2
- W[ ] S[ ] M[ ] F[ ] K[ ] ListThingRegistrationTaskReports
- W[ ] S[ ] M[ ] F[ ] K[ ] ListThingRegistrationTasks
- W[x] S[ ] M[x] F[ ] K[ ] ListThingTypes
- W[x] S[ ] M[x] F[ ] K[ ] ListThings
- W[x] S[ ] M[x] F[ ] K[ ] ListThingsInBillingGroup
- W[x] S[ ] M[x] F[ ] K[ ] ListThingsInThingGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] ListTopicRuleDestinations
- W[x] S[ ] M[x] F[ ] K[ ] ListTopicRules
- W[ ] S[ ] M[ ] F[ ] K[ ] ListV2LoggingLevels
- W[ ] S[ ] M[ ] F[ ] K[ ] ListViolationEvents
- W[ ] S[ ] M[ ] F[ ] K[ ] PutVerificationStateOnViolation
- W[x] S[ ] M[x] F[ ] K[ ] RegisterCACertificate
- W[x] S[ ] M[x] F[ ] K[ ] RegisterCertificate
- W[x] S[ ] M[x] F[ ] K[ ] RegisterCertificateWithoutCA
- W[ ] S[ ] M[ ] F[ ] K[ ] RegisterThing
- W[ ] S[ ] M[ ] F[ ] K[ ] RejectCertificateTransfer
- W[x] S[ ] M[x] F[ ] K[ ] RemoveThingFromBillingGroup
- W[x] S[ ] M[x] F[ ] K[ ] RemoveThingFromThingGroup
- W[x] S[ ] M[x] F[ ] K[ ] ReplaceTopicRule
- W[x] S[ ] M[x] F[ ] K[ ] SearchIndex
- W[ ] S[ ] M[ ] F[ ] K[ ] SetDefaultAuthorizer
- W[x] S[ ] M[x] F[ ] K[ ] SetDefaultPolicyVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] SetLoggingOptions
- W[ ] S[ ] M[ ] F[ ] K[ ] SetV2LoggingLevel
- W[ ] S[ ] M[ ] F[ ] K[ ] SetV2LoggingOptions
- W[ ] S[ ] M[ ] F[ ] K[ ] StartAuditMitigationActionsTask
- W[ ] S[ ] M[ ] F[ ] K[ ] StartDetectMitigationActionsTask
- W[ ] S[ ] M[ ] F[ ] K[ ] StartOnDemandAuditTask
- W[ ] S[ ] M[ ] F[ ] K[ ] StartThingRegistrationTask
- W[ ] S[ ] M[ ] F[ ] K[ ] StopThingRegistrationTask
- W[x] S[ ] M[ ] F[ ] K[ ] TagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] TestAuthorization
- W[ ] S[ ] M[ ] F[ ] K[ ] TestInvokeAuthorizer
- W[ ] S[ ] M[ ] F[ ] K[ ] TransferCertificate
- W[x] S[ ] M[ ] F[ ] K[ ] UntagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateAccountAuditConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateAuditSuppression
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateAuthorizer
- W[x] S[ ] M[x] F[ ] K[ ] UpdateBillingGroup
- W[x] S[ ] M[x] F[ ] K[ ] UpdateCACertificate
- W[x] S[ ] M[x] F[ ] K[ ] UpdateCertificate
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateCertificateProvider
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateCommand
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateCustomMetric
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateDimension
- W[x] S[ ] M[x] F[ ] K[ ] UpdateDomainConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateDynamicThingGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateEncryptionConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateEventConfigurations
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateFleetMetric
- W[x] S[ ] M[x] F[ ] K[ ] UpdateIndexingConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateJob
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateMitigationAction
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdatePackage
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdatePackageConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdatePackageVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateProvisioningTemplate
- W[x] S[ ] M[x] F[ ] K[ ] UpdateRoleAlias
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateScheduledAudit
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateSecurityProfile
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateStream
- W[x] S[ ] M[x] F[ ] K[ ] UpdateThing
- W[x] S[ ] M[x] F[ ] K[ ] UpdateThingGroup
- W[x] S[ ] M[x] F[ ] K[ ] UpdateThingGroupsForThing
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateThingType
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateTopicRuleDestination
- W[ ] S[ ] M[ ] F[ ] K[ ] ValidateSecurityProfileBehaviors

Integration tests: 99/103 implemented operations tested (96.1%)
Untested implemented operations: 4

### winterbaume-iotdataplane (iot-data-plane) - W: 8/8, S: 0/8, M: 0/8, F: 0/8, K: 0/8

- W[x] S[ ] M[ ] F[ ] K[ ] DeleteConnection
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteThingShadow
- W[x] S[ ] M[ ] F[ ] K[ ] GetRetainedMessage
- W[x] S[ ] M[ ] F[ ] K[ ] GetThingShadow
- W[x] S[ ] M[ ] F[ ] K[ ] ListNamedShadowsForThing
- W[x] S[ ] M[ ] F[ ] K[ ] ListRetainedMessages
- W[x] S[ ] M[ ] F[ ] K[ ] Publish
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateThingShadow

Integration tests: 5/8 implemented operations tested (62.5%)
Untested implemented operations: 3

### winterbaume-ivs (ivs) - W: 30/40, S: 5/40, M: 6/40, F: 0/40, K: 0/40

- W[x] S[ ] M[x] F[ ] K[ ] BatchGetChannel
- W[x] S[ ] M[ ] F[ ] K[ ] BatchGetStreamKey
- W[x] S[ ] M[ ] F[ ] K[ ] BatchStartViewerSessionRevocation
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateAdConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] CreateChannel
- W[x] S[ ] M[ ] F[ ] K[ ] CreatePlaybackRestrictionPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] CreateRecordingConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] CreateStreamKey
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteAdConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] DeleteChannel
- W[x] S[ ] M[ ] F[ ] K[ ] DeletePlaybackKeyPair
- W[x] S[ ] M[ ] F[ ] K[ ] DeletePlaybackRestrictionPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteRecordingConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteStreamKey
- W[ ] S[ ] M[ ] F[ ] K[ ] GetAdConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] GetChannel
- W[x] S[ ] M[ ] F[ ] K[ ] GetPlaybackKeyPair
- W[x] S[ ] M[ ] F[ ] K[ ] GetPlaybackRestrictionPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] GetRecordingConfiguration
- W[ ] S[x] M[ ] F[ ] K[ ] GetStream
- W[x] S[ ] M[ ] F[ ] K[ ] GetStreamKey
- W[ ] S[x] M[ ] F[ ] K[ ] GetStreamSession
- W[x] S[ ] M[ ] F[ ] K[ ] ImportPlaybackKeyPair
- W[ ] S[ ] M[ ] F[ ] K[ ] InsertAdBreak
- W[ ] S[ ] M[ ] F[ ] K[ ] ListAdConfigurations
- W[x] S[ ] M[x] F[ ] K[ ] ListChannels
- W[x] S[ ] M[ ] F[ ] K[ ] ListPlaybackKeyPairs
- W[x] S[ ] M[ ] F[ ] K[ ] ListPlaybackRestrictionPolicies
- W[x] S[ ] M[ ] F[ ] K[ ] ListRecordingConfigurations
- W[x] S[ ] M[ ] F[ ] K[ ] ListStreamKeys
- W[ ] S[x] M[ ] F[ ] K[ ] ListStreamSessions
- W[ ] S[x] M[ ] F[ ] K[ ] ListStreams
- W[x] S[ ] M[ ] F[ ] K[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] PutMetadata
- W[x] S[ ] M[ ] F[ ] K[ ] StartViewerSessionRevocation
- W[ ] S[x] M[ ] F[ ] K[ ] StopStream
- W[x] S[ ] M[ ] F[ ] K[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UntagResource
- W[x] S[ ] M[x] F[ ] K[ ] UpdateChannel
- W[x] S[ ] M[ ] F[ ] K[ ] UpdatePlaybackRestrictionPolicy

Integration tests: 30/30 implemented operations tested (100.0%)

### winterbaume-kafka (kafka) - W: 10/59, S: 0/59, M: 13/59, F: 0/59, K: 6/59

- W[ ] S[ ] M[ ] F[ ] K[ ] BatchAssociateScramSecret
- W[ ] S[ ] M[ ] F[ ] K[ ] BatchDisassociateScramSecret
- W[x] S[ ] M[x] F[ ] K[x] CreateCluster
- W[x] S[ ] M[x] F[ ] K[ ] CreateClusterV2
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateReplicator
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateTopic
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateVpcConnection
- W[x] S[ ] M[x] F[ ] K[x] DeleteCluster
- W[ ] S[ ] M[x] F[ ] K[ ] DeleteClusterPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteReplicator
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteTopic
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteVpcConnection
- W[x] S[ ] M[x] F[ ] K[x] DescribeCluster
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeClusterOperation
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeClusterOperationV2
- W[x] S[ ] M[x] F[ ] K[ ] DescribeClusterV2
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeConfigurationRevision
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeReplicator
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeTopic
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeTopicPartitions
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeVpcConnection
- W[ ] S[ ] M[ ] F[ ] K[x] GetBootstrapBrokers
- W[ ] S[ ] M[x] F[ ] K[ ] GetClusterPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] GetCompatibleKafkaVersions
- W[ ] S[ ] M[ ] F[ ] K[ ] ListClientVpcConnections
- W[ ] S[ ] M[ ] F[ ] K[ ] ListClusterOperations
- W[ ] S[ ] M[ ] F[ ] K[ ] ListClusterOperationsV2
- W[x] S[ ] M[x] F[ ] K[x] ListClusters
- W[x] S[ ] M[x] F[ ] K[ ] ListClustersV2
- W[ ] S[ ] M[ ] F[ ] K[ ] ListConfigurationRevisions
- W[ ] S[ ] M[ ] F[ ] K[ ] ListConfigurations
- W[ ] S[ ] M[ ] F[ ] K[ ] ListKafkaVersions
- W[ ] S[ ] M[ ] F[ ] K[ ] ListNodes
- W[ ] S[ ] M[ ] F[ ] K[ ] ListReplicators
- W[ ] S[ ] M[ ] F[ ] K[ ] ListScramSecrets
- W[x] S[ ] M[x] F[ ] K[ ] ListTagsForResource
- W[ ] S[ ] M[ ] F[ ] K[ ] ListTopics
- W[ ] S[ ] M[ ] F[ ] K[ ] ListVpcConnections
- W[ ] S[ ] M[x] F[ ] K[ ] PutClusterPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] RebootBroker
- W[ ] S[ ] M[ ] F[ ] K[ ] RejectClientVpcConnection
- W[x] S[ ] M[x] F[ ] K[ ] TagResource
- W[x] S[ ] M[x] F[ ] K[ ] UntagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateBrokerCount
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateBrokerStorage
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateBrokerType
- W[ ] S[ ] M[ ] F[ ] K[x] UpdateClusterConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateClusterKafkaVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateConnectivity
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateMonitoring
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateRebalancing
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateReplicationInfo
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateSecurity
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateStorage
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateTopic

Integration tests: 10/10 implemented operations tested (100.0%)

### winterbaume-keyspaces (keyspaces) - W: 19/19, S: 0/19, M: 0/19, F: 0/19, K: 0/19

- W[x] S[ ] M[ ] F[ ] K[ ] CreateKeyspace
- W[x] S[ ] M[ ] F[ ] K[ ] CreateTable
- W[x] S[ ] M[ ] F[ ] K[ ] CreateType
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteKeyspace
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteTable
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteType
- W[x] S[ ] M[ ] F[ ] K[ ] GetKeyspace
- W[x] S[ ] M[ ] F[ ] K[ ] GetTable
- W[x] S[ ] M[ ] F[ ] K[ ] GetTableAutoScalingSettings
- W[x] S[ ] M[ ] F[ ] K[ ] GetType
- W[x] S[ ] M[ ] F[ ] K[ ] ListKeyspaces
- W[x] S[ ] M[ ] F[ ] K[ ] ListTables
- W[x] S[ ] M[ ] F[ ] K[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] ListTypes
- W[x] S[ ] M[ ] F[ ] K[ ] RestoreTable
- W[x] S[ ] M[ ] F[ ] K[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateKeyspace
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateTable

Integration tests: 13/19 implemented operations tested (68.4%)
Untested implemented operations: 6

### winterbaume-kinesis (kinesis) - W: 38/39, S: 0/39, M: 31/39, F: 0/39, K: 9/39

Terraform E2E: 4 tests across 1 terraform resource types

Resource types: aws_kinesis_stream

- W[x] S[ ] M[x] F[ ] K[ ] AddTagsToStream
- W[x] S[ ] M[x] F[ ] K[x] CreateStream
- W[x] S[ ] M[x] F[ ] K[ ] DecreaseStreamRetentionPeriod
- W[x] S[ ] M[x] F[ ] K[ ] DeleteResourcePolicy
- W[x] S[ ] M[x] F[ ] K[x] DeleteStream
- W[x] S[ ] M[x] F[ ] K[ ] DeregisterStreamConsumer
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeAccountSettings
- W[x] S[ ] M[x] F[ ] K[ ] DescribeLimits
- W[x] S[ ] M[x] F[ ] K[x] DescribeStream
- W[x] S[ ] M[x] F[ ] K[ ] DescribeStreamConsumer
- W[x] S[ ] M[x] F[ ] K[ ] DescribeStreamSummary
- W[x] S[ ] M[x] F[ ] K[ ] DisableEnhancedMonitoring
- W[x] S[ ] M[x] F[ ] K[ ] EnableEnhancedMonitoring
- W[x] S[ ] M[x] F[ ] K[x] GetRecords
- W[x] S[ ] M[x] F[ ] K[ ] GetResourcePolicy
- W[x] S[ ] M[x] F[ ] K[x] GetShardIterator
- W[x] S[ ] M[x] F[ ] K[ ] IncreaseStreamRetentionPeriod
- W[x] S[ ] M[x] F[ ] K[x] ListShards
- W[x] S[ ] M[x] F[ ] K[ ] ListStreamConsumers
- W[x] S[ ] M[x] F[ ] K[x] ListStreams
- W[x] S[ ] M[ ] F[ ] K[ ] ListTagsForResource
- W[x] S[ ] M[x] F[ ] K[ ] ListTagsForStream
- W[x] S[ ] M[x] F[ ] K[ ] MergeShards
- W[x] S[ ] M[x] F[ ] K[x] PutRecord
- W[x] S[ ] M[x] F[ ] K[x] PutRecords
- W[x] S[ ] M[x] F[ ] K[ ] PutResourcePolicy
- W[x] S[ ] M[x] F[ ] K[ ] RegisterStreamConsumer
- W[x] S[ ] M[x] F[ ] K[ ] RemoveTagsFromStream
- W[x] S[ ] M[x] F[ ] K[ ] SplitShard
- W[x] S[ ] M[x] F[ ] K[ ] StartStreamEncryption
- W[x] S[ ] M[x] F[ ] K[ ] StopStreamEncryption
- W[ ] S[ ] M[ ] F[ ] K[ ] SubscribeToShard
- W[x] S[ ] M[ ] F[ ] K[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateAccountSettings
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateMaxRecordSize
- W[x] S[ ] M[x] F[ ] K[ ] UpdateShardCount
- W[x] S[ ] M[x] F[ ] K[ ] UpdateStreamMode
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateStreamWarmThroughput

Integration tests: 31/38 implemented operations tested (81.6%)
Untested implemented operations: 7

### winterbaume-kinesisanalyticsv2 (kinesis-analytics-v2) - W: 32/33, S: 1/33, M: 0/33, F: 0/33, K: 0/33

- W[x] S[ ] M[ ] F[ ] K[ ] AddApplicationCloudWatchLoggingOption
- W[x] S[ ] M[ ] F[ ] K[ ] AddApplicationInput
- W[x] S[ ] M[ ] F[ ] K[ ] AddApplicationInputProcessingConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] AddApplicationOutput
- W[x] S[ ] M[ ] F[ ] K[ ] AddApplicationReferenceDataSource
- W[x] S[ ] M[ ] F[ ] K[ ] AddApplicationVpcConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] CreateApplication
- W[x] S[ ] M[ ] F[ ] K[ ] CreateApplicationPresignedUrl
- W[x] S[ ] M[ ] F[ ] K[ ] CreateApplicationSnapshot
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteApplication
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteApplicationCloudWatchLoggingOption
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteApplicationInputProcessingConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteApplicationOutput
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteApplicationReferenceDataSource
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteApplicationSnapshot
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteApplicationVpcConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeApplication
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeApplicationOperation
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeApplicationSnapshot
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeApplicationVersion
- W[ ] S[x] M[ ] F[ ] K[ ] DiscoverInputSchema
- W[x] S[ ] M[ ] F[ ] K[ ] ListApplicationOperations
- W[x] S[ ] M[ ] F[ ] K[ ] ListApplicationSnapshots
- W[x] S[ ] M[ ] F[ ] K[ ] ListApplicationVersions
- W[x] S[ ] M[ ] F[ ] K[ ] ListApplications
- W[x] S[ ] M[ ] F[ ] K[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] RollbackApplication
- W[x] S[ ] M[ ] F[ ] K[ ] StartApplication
- W[x] S[ ] M[ ] F[ ] K[ ] StopApplication
- W[x] S[ ] M[ ] F[ ] K[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateApplication
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateApplicationMaintenanceConfiguration

Integration tests: 25/32 implemented operations tested (78.1%)
Untested implemented operations: 7

### winterbaume-kinesisvideo (kinesis-video) - W: 32/32, S: 0/32, M: 0/32, F: 0/32, K: 0/32

- W[x] S[ ] M[ ] F[ ] K[ ] CreateSignalingChannel
- W[x] S[ ] M[ ] F[ ] K[ ] CreateStream
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteEdgeConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteSignalingChannel
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteStream
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeEdgeConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeImageGenerationConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeMappedResourceConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeMediaStorageConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeNotificationConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeSignalingChannel
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeStream
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeStreamStorageConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] GetDataEndpoint
- W[x] S[ ] M[ ] F[ ] K[ ] GetSignalingChannelEndpoint
- W[x] S[ ] M[ ] F[ ] K[ ] ListEdgeAgentConfigurations
- W[x] S[ ] M[ ] F[ ] K[ ] ListSignalingChannels
- W[x] S[ ] M[ ] F[ ] K[ ] ListStreams
- W[x] S[ ] M[ ] F[ ] K[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] ListTagsForStream
- W[x] S[ ] M[ ] F[ ] K[ ] StartEdgeConfigurationUpdate
- W[x] S[ ] M[ ] F[ ] K[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] TagStream
- W[x] S[ ] M[ ] F[ ] K[ ] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UntagStream
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateDataRetention
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateImageGenerationConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateMediaStorageConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateNotificationConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateSignalingChannel
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateStream
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateStreamStorageConfiguration

Integration tests: 32/32 implemented operations tested (100.0%)

### winterbaume-kinesisvideoarchivedmedia (kinesis-video-archived-media) - W: 6/6, S: 0/6, M: 3/6, F: 0/6, K: 0/6

- W[x] S[ ] M[x] F[ ] K[ ] GetClip
- W[x] S[ ] M[x] F[ ] K[ ] GetDASHStreamingSessionURL
- W[x] S[ ] M[x] F[ ] K[ ] GetHLSStreamingSessionURL
- W[x] S[ ] M[ ] F[ ] K[ ] GetImages
- W[x] S[ ] M[ ] F[ ] K[ ] GetMediaForFragmentList
- W[x] S[ ] M[ ] F[ ] K[ ] ListFragments

Integration tests: 6/6 implemented operations tested (100.0%)

### winterbaume-kms (kms) - W: 53/54, S: 0/54, M: 40/54, F: 0/54, K: 19/54

Terraform E2E: 3 tests across 2 terraform resource types

Resource types: aws_kms_alias, aws_kms_key

- W[x] S[ ] M[x] F[ ] K[ ] CancelKeyDeletion
- W[x] S[ ] M[ ] F[ ] K[ ] ConnectCustomKeyStore
- W[x] S[ ] M[x] F[ ] K[x] CreateAlias
- W[x] S[ ] M[ ] F[ ] K[ ] CreateCustomKeyStore
- W[x] S[ ] M[x] F[ ] K[ ] CreateGrant
- W[x] S[ ] M[x] F[ ] K[x] CreateKey
- W[x] S[ ] M[x] F[ ] K[x] Decrypt
- W[x] S[ ] M[x] F[ ] K[x] DeleteAlias
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteCustomKeyStore
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteImportedKeyMaterial
- W[x] S[ ] M[ ] F[ ] K[ ] DeriveSharedSecret
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeCustomKeyStores
- W[x] S[ ] M[x] F[ ] K[x] DescribeKey
- W[x] S[ ] M[x] F[ ] K[x] DisableKey
- W[x] S[ ] M[x] F[ ] K[ ] DisableKeyRotation
- W[x] S[ ] M[ ] F[ ] K[ ] DisconnectCustomKeyStore
- W[x] S[ ] M[x] F[ ] K[x] EnableKey
- W[x] S[ ] M[x] F[ ] K[ ] EnableKeyRotation
- W[x] S[ ] M[x] F[ ] K[x] Encrypt
- W[x] S[ ] M[x] F[ ] K[x] GenerateDataKey
- W[x] S[ ] M[ ] F[ ] K[ ] GenerateDataKeyPair
- W[x] S[ ] M[ ] F[ ] K[ ] GenerateDataKeyPairWithoutPlaintext
- W[x] S[ ] M[x] F[ ] K[ ] GenerateDataKeyWithoutPlaintext
- W[x] S[ ] M[x] F[ ] K[ ] GenerateMac
- W[x] S[ ] M[x] F[ ] K[ ] GenerateRandom
- W[ ] S[ ] M[ ] F[ ] K[ ] GetKeyLastUsage
- W[x] S[ ] M[x] F[ ] K[x] GetKeyPolicy
- W[x] S[ ] M[x] F[ ] K[x] GetKeyRotationStatus
- W[x] S[ ] M[ ] F[ ] K[ ] GetParametersForImport
- W[x] S[ ] M[x] F[ ] K[ ] GetPublicKey
- W[x] S[ ] M[ ] F[ ] K[ ] ImportKeyMaterial
- W[x] S[ ] M[x] F[ ] K[x] ListAliases
- W[x] S[ ] M[x] F[ ] K[ ] ListGrants
- W[x] S[ ] M[x] F[ ] K[x] ListKeyPolicies
- W[x] S[ ] M[x] F[ ] K[ ] ListKeyRotations
- W[x] S[ ] M[x] F[ ] K[x] ListKeys
- W[x] S[ ] M[x] F[ ] K[x] ListResourceTags
- W[x] S[ ] M[x] F[ ] K[ ] ListRetirableGrants
- W[x] S[ ] M[x] F[ ] K[x] PutKeyPolicy
- W[x] S[ ] M[x] F[ ] K[ ] ReEncrypt
- W[x] S[ ] M[x] F[ ] K[ ] ReplicateKey
- W[x] S[ ] M[x] F[ ] K[ ] RetireGrant
- W[x] S[ ] M[x] F[ ] K[ ] RevokeGrant
- W[x] S[ ] M[x] F[ ] K[ ] RotateKeyOnDemand
- W[x] S[ ] M[x] F[ ] K[x] ScheduleKeyDeletion
- W[x] S[ ] M[x] F[ ] K[ ] Sign
- W[x] S[ ] M[x] F[ ] K[x] TagResource
- W[x] S[ ] M[x] F[ ] K[x] UntagResource
- W[x] S[ ] M[x] F[ ] K[ ] UpdateAlias
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateCustomKeyStore
- W[x] S[ ] M[x] F[ ] K[ ] UpdateKeyDescription
- W[x] S[ ] M[ ] F[ ] K[ ] UpdatePrimaryRegion
- W[x] S[ ] M[x] F[ ] K[ ] Verify
- W[x] S[ ] M[x] F[ ] K[ ] VerifyMac

Integration tests: 40/53 implemented operations tested (75.5%)
Untested implemented operations: 13

### winterbaume-lakeformation (lakeformation) - W: 19/61, S: 1/61, M: 20/61, F: 0/61, K: 0/61

- W[x] S[ ] M[x] F[ ] K[ ] AddLFTagsToResource
- W[ ] S[ ] M[ ] F[ ] K[ ] AssumeDecoratedRoleWithSAML
- W[x] S[ ] M[x] F[ ] K[ ] BatchGrantPermissions
- W[x] S[ ] M[x] F[ ] K[ ] BatchRevokePermissions
- W[ ] S[ ] M[ ] F[ ] K[ ] CancelTransaction
- W[ ] S[ ] M[ ] F[ ] K[ ] CommitTransaction
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateDataCellsFilter
- W[x] S[ ] M[x] F[ ] K[ ] CreateLFTag
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateLFTagExpression
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateLakeFormationIdentityCenterConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateLakeFormationOptIn
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteDataCellsFilter
- W[x] S[ ] M[x] F[ ] K[ ] DeleteLFTag
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteLFTagExpression
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteLakeFormationIdentityCenterConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteLakeFormationOptIn
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteObjectsOnCancel
- W[x] S[ ] M[x] F[ ] K[ ] DeregisterResource
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeLakeFormationIdentityCenterConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] DescribeResource
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeTransaction
- W[ ] S[ ] M[ ] F[ ] K[ ] ExtendTransaction
- W[ ] S[ ] M[ ] F[ ] K[ ] GetDataCellsFilter
- W[ ] S[ ] M[ ] F[ ] K[ ] GetDataLakePrincipal
- W[x] S[ ] M[x] F[ ] K[ ] GetDataLakeSettings
- W[ ] S[ ] M[ ] F[ ] K[ ] GetEffectivePermissionsForPath
- W[x] S[ ] M[x] F[ ] K[ ] GetLFTag
- W[ ] S[ ] M[ ] F[ ] K[ ] GetLFTagExpression
- W[ ] S[ ] M[ ] F[ ] K[ ] GetQueryState
- W[ ] S[ ] M[ ] F[ ] K[ ] GetQueryStatistics
- W[x] S[ ] M[x] F[ ] K[ ] GetResourceLFTags
- W[ ] S[ ] M[ ] F[ ] K[ ] GetTableObjects
- W[ ] S[ ] M[ ] F[ ] K[ ] GetTemporaryDataLocationCredentials
- W[ ] S[ ] M[ ] F[ ] K[ ] GetTemporaryGluePartitionCredentials
- W[ ] S[ ] M[ ] F[ ] K[ ] GetTemporaryGlueTableCredentials
- W[ ] S[ ] M[ ] F[ ] K[ ] GetWorkUnitResults
- W[ ] S[ ] M[ ] F[ ] K[ ] GetWorkUnits
- W[x] S[ ] M[x] F[ ] K[ ] GrantPermissions
- W[ ] S[x] M[x] F[ ] K[ ] ListDataCellsFilter
- W[ ] S[ ] M[ ] F[ ] K[ ] ListLFTagExpressions
- W[x] S[ ] M[x] F[ ] K[ ] ListLFTags
- W[ ] S[ ] M[ ] F[ ] K[ ] ListLakeFormationOptIns
- W[x] S[ ] M[x] F[ ] K[ ] ListPermissions
- W[x] S[ ] M[x] F[ ] K[ ] ListResources
- W[ ] S[ ] M[ ] F[ ] K[ ] ListTableStorageOptimizers
- W[ ] S[ ] M[ ] F[ ] K[ ] ListTransactions
- W[x] S[ ] M[x] F[ ] K[ ] PutDataLakeSettings
- W[x] S[ ] M[x] F[ ] K[ ] RegisterResource
- W[x] S[ ] M[x] F[ ] K[ ] RemoveLFTagsFromResource
- W[x] S[ ] M[x] F[ ] K[ ] RevokePermissions
- W[ ] S[ ] M[ ] F[ ] K[ ] SearchDatabasesByLFTags
- W[ ] S[ ] M[ ] F[ ] K[ ] SearchTablesByLFTags
- W[ ] S[ ] M[ ] F[ ] K[ ] StartQueryPlanning
- W[ ] S[ ] M[ ] F[ ] K[ ] StartTransaction
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateDataCellsFilter
- W[x] S[ ] M[x] F[ ] K[ ] UpdateLFTag
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateLFTagExpression
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateLakeFormationIdentityCenterConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateResource
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateTableObjects
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateTableStorageOptimizer

Integration tests: 17/19 implemented operations tested (89.5%)
Untested implemented operations: 2

### winterbaume-lambda (lambda) - W: 85/85, S: 0/85, M: 46/85, F: 0/85, K: 17/85

Terraform E2E: 9 tests across 7 terraform resource types

Resource types: aws_lambda_alias, aws_lambda_code_signing_config, aws_lambda_event_source_mapping, aws_lambda_function, aws_lambda_function_url, aws_lambda_layer_version, aws_lambda_permission

- W[x] S[ ] M[x] F[ ] K[ ] AddLayerVersionPermission
- W[x] S[ ] M[x] F[ ] K[ ] AddPermission
- W[x] S[ ] M[ ] F[ ] K[ ] CheckpointDurableExecution
- W[x] S[ ] M[x] F[ ] K[ ] CreateAlias
- W[x] S[ ] M[ ] F[ ] K[ ] CreateCapacityProvider
- W[x] S[ ] M[ ] F[ ] K[ ] CreateCodeSigningConfig
- W[x] S[ ] M[x] F[ ] K[x] CreateEventSourceMapping
- W[x] S[ ] M[x] F[ ] K[x] CreateFunction
- W[x] S[ ] M[x] F[ ] K[ ] CreateFunctionUrlConfig
- W[x] S[ ] M[x] F[ ] K[ ] DeleteAlias
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteCapacityProvider
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteCodeSigningConfig
- W[x] S[ ] M[x] F[ ] K[x] DeleteEventSourceMapping
- W[x] S[ ] M[x] F[ ] K[x] DeleteFunction
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteFunctionCodeSigningConfig
- W[x] S[ ] M[x] F[ ] K[ ] DeleteFunctionConcurrency
- W[x] S[ ] M[x] F[ ] K[ ] DeleteFunctionEventInvokeConfig
- W[x] S[ ] M[x] F[ ] K[ ] DeleteFunctionUrlConfig
- W[x] S[ ] M[x] F[ ] K[ ] DeleteLayerVersion
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteProvisionedConcurrencyConfig
- W[x] S[ ] M[ ] F[ ] K[ ] GetAccountSettings
- W[x] S[ ] M[x] F[ ] K[ ] GetAlias
- W[x] S[ ] M[ ] F[ ] K[ ] GetCapacityProvider
- W[x] S[ ] M[ ] F[ ] K[ ] GetCodeSigningConfig
- W[x] S[ ] M[ ] F[ ] K[ ] GetDurableExecution
- W[x] S[ ] M[ ] F[ ] K[ ] GetDurableExecutionHistory
- W[x] S[ ] M[ ] F[ ] K[ ] GetDurableExecutionState
- W[x] S[ ] M[x] F[ ] K[x] GetEventSourceMapping
- W[x] S[ ] M[x] F[ ] K[x] GetFunction
- W[x] S[ ] M[x] F[ ] K[x] GetFunctionCodeSigningConfig
- W[x] S[ ] M[x] F[ ] K[ ] GetFunctionConcurrency
- W[x] S[ ] M[ ] F[ ] K[ ] GetFunctionConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] GetFunctionEventInvokeConfig
- W[x] S[ ] M[ ] F[ ] K[ ] GetFunctionRecursionConfig
- W[x] S[ ] M[ ] F[ ] K[ ] GetFunctionScalingConfig
- W[x] S[ ] M[x] F[ ] K[ ] GetFunctionUrlConfig
- W[x] S[ ] M[x] F[ ] K[ ] GetLayerVersion
- W[x] S[ ] M[ ] F[ ] K[ ] GetLayerVersionByArn
- W[x] S[ ] M[x] F[ ] K[ ] GetLayerVersionPolicy
- W[x] S[ ] M[x] F[ ] K[x] GetPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] GetProvisionedConcurrencyConfig
- W[x] S[ ] M[ ] F[ ] K[ ] GetRuntimeManagementConfig
- W[x] S[ ] M[x] F[ ] K[x] Invoke
- W[x] S[ ] M[ ] F[ ] K[ ] InvokeAsync
- W[x] S[ ] M[ ] F[ ] K[ ] InvokeWithResponseStream
- W[x] S[ ] M[x] F[ ] K[x] ListAliases
- W[x] S[ ] M[ ] F[ ] K[ ] ListCapacityProviders
- W[x] S[ ] M[ ] F[ ] K[ ] ListCodeSigningConfigs
- W[x] S[ ] M[ ] F[ ] K[ ] ListDurableExecutionsByFunction
- W[x] S[ ] M[x] F[ ] K[x] ListEventSourceMappings
- W[x] S[ ] M[x] F[ ] K[x] ListFunctionEventInvokeConfigs
- W[x] S[ ] M[ ] F[ ] K[ ] ListFunctionUrlConfigs
- W[x] S[ ] M[ ] F[ ] K[ ] ListFunctionVersionsByCapacityProvider
- W[x] S[ ] M[x] F[ ] K[x] ListFunctions
- W[x] S[ ] M[ ] F[ ] K[ ] ListFunctionsByCodeSigningConfig
- W[x] S[ ] M[x] F[ ] K[ ] ListLayerVersions
- W[x] S[ ] M[x] F[ ] K[ ] ListLayers
- W[x] S[ ] M[ ] F[ ] K[ ] ListProvisionedConcurrencyConfigs
- W[x] S[ ] M[x] F[ ] K[ ] ListTags
- W[x] S[ ] M[x] F[ ] K[x] ListVersionsByFunction
- W[x] S[ ] M[x] F[ ] K[ ] PublishLayerVersion
- W[x] S[ ] M[x] F[ ] K[ ] PublishVersion
- W[x] S[ ] M[ ] F[ ] K[ ] PutFunctionCodeSigningConfig
- W[x] S[ ] M[x] F[ ] K[ ] PutFunctionConcurrency
- W[x] S[ ] M[x] F[ ] K[ ] PutFunctionEventInvokeConfig
- W[x] S[ ] M[ ] F[ ] K[ ] PutFunctionRecursionConfig
- W[x] S[ ] M[ ] F[ ] K[ ] PutFunctionScalingConfig
- W[x] S[ ] M[ ] F[ ] K[ ] PutProvisionedConcurrencyConfig
- W[x] S[ ] M[ ] F[ ] K[ ] PutRuntimeManagementConfig
- W[x] S[ ] M[x] F[ ] K[ ] RemoveLayerVersionPermission
- W[x] S[ ] M[x] F[ ] K[ ] RemovePermission
- W[x] S[ ] M[ ] F[ ] K[ ] SendDurableExecutionCallbackFailure
- W[x] S[ ] M[ ] F[ ] K[ ] SendDurableExecutionCallbackHeartbeat
- W[x] S[ ] M[ ] F[ ] K[ ] SendDurableExecutionCallbackSuccess
- W[x] S[ ] M[ ] F[ ] K[ ] StopDurableExecution
- W[x] S[ ] M[x] F[ ] K[ ] TagResource
- W[x] S[ ] M[x] F[ ] K[ ] UntagResource
- W[x] S[ ] M[x] F[ ] K[ ] UpdateAlias
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateCapacityProvider
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateCodeSigningConfig
- W[x] S[ ] M[x] F[ ] K[x] UpdateEventSourceMapping
- W[x] S[ ] M[x] F[ ] K[x] UpdateFunctionCode
- W[x] S[ ] M[x] F[ ] K[x] UpdateFunctionConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] UpdateFunctionEventInvokeConfig
- W[x] S[ ] M[x] F[ ] K[ ] UpdateFunctionUrlConfig

Integration tests: 60/85 implemented operations tested (70.6%)
Untested implemented operations: 25

### winterbaume-lexmodelsv2 (lex-models-v2) - W: 58/107, S: 2/107, M: 17/107, F: 0/107, K: 0/107

- W[x] S[ ] M[ ] F[ ] K[ ] BatchCreateCustomVocabularyItem
- W[x] S[ ] M[ ] F[ ] K[ ] BatchDeleteCustomVocabularyItem
- W[x] S[ ] M[ ] F[ ] K[ ] BatchUpdateCustomVocabularyItem
- W[x] S[ ] M[ ] F[ ] K[ ] BuildBotLocale
- W[x] S[ ] M[x] F[ ] K[ ] CreateBot
- W[x] S[ ] M[x] F[ ] K[ ] CreateBotAlias
- W[x] S[ ] M[ ] F[ ] K[ ] CreateBotLocale
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateBotReplica
- W[x] S[ ] M[ ] F[ ] K[ ] CreateBotVersion
- W[x] S[ ] M[ ] F[ ] K[ ] CreateExport
- W[x] S[ ] M[ ] F[ ] K[ ] CreateIntent
- W[x] S[ ] M[x] F[ ] K[ ] CreateResourcePolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateResourcePolicyStatement
- W[x] S[ ] M[ ] F[ ] K[ ] CreateSlot
- W[x] S[ ] M[ ] F[ ] K[ ] CreateSlotType
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateTestSetDiscrepancyReport
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateUploadUrl
- W[x] S[ ] M[x] F[ ] K[ ] DeleteBot
- W[x] S[ ] M[x] F[ ] K[ ] DeleteBotAlias
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteBotAnalyzerRecommendation
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteBotLocale
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteBotReplica
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteBotVersion
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteCustomVocabulary
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteExport
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteImport
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteIntent
- W[x] S[ ] M[x] F[ ] K[ ] DeleteResourcePolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteResourcePolicyStatement
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteSlot
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteSlotType
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteTestSet
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteUtterances
- W[x] S[ ] M[x] F[ ] K[ ] DescribeBot
- W[x] S[ ] M[x] F[ ] K[ ] DescribeBotAlias
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeBotAnalyzerRecommendation
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeBotLocale
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeBotRecommendation
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeBotReplica
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeBotResourceGeneration
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeBotVersion
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeCustomVocabularyMetadata
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeExport
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeImport
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeIntent
- W[x] S[ ] M[x] F[ ] K[ ] DescribeResourcePolicy
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeSlot
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeSlotType
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeTestExecution
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeTestSet
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeTestSetDiscrepancyReport
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeTestSetGeneration
- W[ ] S[ ] M[ ] F[ ] K[ ] GenerateBotElement
- W[ ] S[ ] M[ ] F[ ] K[ ] GetTestExecutionArtifactsUrl
- W[ ] S[ ] M[ ] F[ ] K[ ] ListAggregatedUtterances
- W[ ] S[ ] M[ ] F[ ] K[ ] ListBotAliasReplicas
- W[x] S[ ] M[x] F[ ] K[ ] ListBotAliases
- W[ ] S[ ] M[ ] F[ ] K[ ] ListBotAnalyzerHistory
- W[x] S[ ] M[ ] F[ ] K[ ] ListBotLocales
- W[ ] S[ ] M[ ] F[ ] K[ ] ListBotRecommendations
- W[ ] S[ ] M[ ] F[ ] K[ ] ListBotReplicas
- W[ ] S[ ] M[ ] F[ ] K[ ] ListBotResourceGenerations
- W[ ] S[ ] M[ ] F[ ] K[ ] ListBotVersionReplicas
- W[x] S[ ] M[ ] F[ ] K[ ] ListBotVersions
- W[x] S[ ] M[x] F[ ] K[ ] ListBots
- W[ ] S[x] M[ ] F[ ] K[ ] ListBuiltInIntents
- W[ ] S[x] M[ ] F[ ] K[ ] ListBuiltInSlotTypes
- W[x] S[ ] M[ ] F[ ] K[ ] ListCustomVocabularyItems
- W[x] S[ ] M[ ] F[ ] K[ ] ListExports
- W[x] S[ ] M[ ] F[ ] K[ ] ListImports
- W[ ] S[ ] M[ ] F[ ] K[ ] ListIntentMetrics
- W[ ] S[ ] M[ ] F[ ] K[ ] ListIntentPaths
- W[ ] S[ ] M[ ] F[ ] K[ ] ListIntentStageMetrics
- W[x] S[ ] M[ ] F[ ] K[ ] ListIntents
- W[ ] S[ ] M[ ] F[ ] K[ ] ListRecommendedIntents
- W[ ] S[ ] M[ ] F[ ] K[ ] ListSessionAnalyticsData
- W[ ] S[ ] M[ ] F[ ] K[ ] ListSessionMetrics
- W[x] S[ ] M[ ] F[ ] K[ ] ListSlotTypes
- W[x] S[ ] M[ ] F[ ] K[ ] ListSlots
- W[x] S[ ] M[x] F[ ] K[ ] ListTagsForResource
- W[ ] S[ ] M[ ] F[ ] K[ ] ListTestExecutionResultItems
- W[ ] S[ ] M[ ] F[ ] K[ ] ListTestExecutions
- W[ ] S[ ] M[ ] F[ ] K[ ] ListTestSetRecords
- W[ ] S[ ] M[ ] F[ ] K[ ] ListTestSets
- W[ ] S[ ] M[ ] F[ ] K[ ] ListUtteranceAnalyticsData
- W[ ] S[ ] M[ ] F[ ] K[ ] ListUtteranceMetrics
- W[ ] S[ ] M[ ] F[ ] K[ ] SearchAssociatedTranscripts
- W[ ] S[ ] M[ ] F[ ] K[ ] StartBotAnalyzer
- W[ ] S[ ] M[ ] F[ ] K[ ] StartBotRecommendation
- W[ ] S[ ] M[ ] F[ ] K[ ] StartBotResourceGeneration
- W[x] S[ ] M[ ] F[ ] K[ ] StartImport
- W[ ] S[ ] M[ ] F[ ] K[ ] StartTestExecution
- W[ ] S[ ] M[ ] F[ ] K[ ] StartTestSetGeneration
- W[ ] S[ ] M[ ] F[ ] K[ ] StopBotAnalyzer
- W[ ] S[ ] M[ ] F[ ] K[ ] StopBotRecommendation
- W[x] S[ ] M[x] F[ ] K[ ] TagResource
- W[x] S[ ] M[x] F[ ] K[ ] UntagResource
- W[x] S[ ] M[x] F[ ] K[ ] UpdateBot
- W[x] S[ ] M[x] F[ ] K[ ] UpdateBotAlias
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateBotLocale
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateBotRecommendation
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateExport
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateIntent
- W[x] S[ ] M[x] F[ ] K[ ] UpdateResourcePolicy
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateSlot
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateSlotType
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateTestSet

Integration tests: 58/58 implemented operations tested (100.0%)

### winterbaume-macie2 (macie2) - W: 67/81, S: 14/81, M: 13/81, F: 0/81, K: 24/81

- W[x] S[ ] M[x] F[ ] K[ ] AcceptInvitation
- W[x] S[ ] M[ ] F[ ] K[ ] BatchGetCustomDataIdentifiers
- W[x] S[ ] M[ ] F[ ] K[ ] BatchUpdateAutomatedDiscoveryAccounts
- W[x] S[ ] M[ ] F[ ] K[x] CreateAllowList
- W[x] S[ ] M[ ] F[ ] K[x] CreateClassificationJob
- W[x] S[ ] M[ ] F[ ] K[x] CreateCustomDataIdentifier
- W[x] S[ ] M[ ] F[ ] K[x] CreateFindingsFilter
- W[x] S[ ] M[x] F[ ] K[ ] CreateInvitations
- W[x] S[ ] M[ ] F[ ] K[ ] CreateMember
- W[x] S[ ] M[ ] F[ ] K[ ] CreateSampleFindings
- W[x] S[ ] M[x] F[ ] K[ ] DeclineInvitations
- W[x] S[ ] M[ ] F[ ] K[x] DeleteAllowList
- W[x] S[ ] M[ ] F[ ] K[x] DeleteCustomDataIdentifier
- W[x] S[ ] M[ ] F[ ] K[x] DeleteFindingsFilter
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteInvitations
- W[x] S[ ] M[x] F[ ] K[ ] DeleteMember
- W[ ] S[x] M[ ] F[ ] K[ ] DescribeBuckets
- W[x] S[ ] M[ ] F[ ] K[x] DescribeClassificationJob
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeOrganizationConfiguration
- W[x] S[ ] M[x] F[ ] K[x] DisableMacie
- W[x] S[ ] M[ ] F[ ] K[ ] DisableOrganizationAdminAccount
- W[x] S[ ] M[ ] F[ ] K[ ] DisassociateFromAdministratorAccount
- W[x] S[ ] M[ ] F[ ] K[ ] DisassociateFromMasterAccount
- W[x] S[ ] M[x] F[ ] K[ ] DisassociateMember
- W[x] S[ ] M[x] F[ ] K[x] EnableMacie
- W[x] S[ ] M[x] F[ ] K[ ] EnableOrganizationAdminAccount
- W[x] S[ ] M[x] F[ ] K[ ] GetAdministratorAccount
- W[x] S[ ] M[ ] F[ ] K[x] GetAllowList
- W[x] S[ ] M[ ] F[ ] K[ ] GetAutomatedDiscoveryConfiguration
- W[ ] S[x] M[ ] F[ ] K[ ] GetBucketStatistics
- W[x] S[ ] M[ ] F[ ] K[ ] GetClassificationExportConfiguration
- W[ ] S[x] M[ ] F[ ] K[ ] GetClassificationScope
- W[x] S[ ] M[ ] F[ ] K[x] GetCustomDataIdentifier
- W[x] S[ ] M[ ] F[ ] K[ ] GetFindingStatistics
- W[x] S[ ] M[ ] F[ ] K[x] GetFindings
- W[x] S[ ] M[ ] F[ ] K[x] GetFindingsFilter
- W[x] S[ ] M[ ] F[ ] K[ ] GetFindingsPublicationConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] GetInvitationsCount
- W[x] S[ ] M[x] F[ ] K[x] GetMacieSession
- W[x] S[ ] M[ ] F[ ] K[ ] GetMasterAccount
- W[x] S[ ] M[ ] F[ ] K[ ] GetMember
- W[ ] S[x] M[ ] F[ ] K[ ] GetResourceProfile
- W[x] S[ ] M[ ] F[ ] K[ ] GetRevealConfiguration
- W[ ] S[x] M[ ] F[ ] K[ ] GetSensitiveDataOccurrences
- W[ ] S[x] M[ ] F[ ] K[ ] GetSensitiveDataOccurrencesAvailability
- W[x] S[ ] M[ ] F[ ] K[ ] GetSensitivityInspectionTemplate
- W[x] S[ ] M[ ] F[ ] K[ ] GetUsageStatistics
- W[x] S[ ] M[ ] F[ ] K[ ] GetUsageTotals
- W[x] S[ ] M[ ] F[ ] K[x] ListAllowLists
- W[x] S[ ] M[ ] F[ ] K[ ] ListAutomatedDiscoveryAccounts
- W[x] S[ ] M[ ] F[ ] K[x] ListClassificationJobs
- W[ ] S[x] M[ ] F[ ] K[ ] ListClassificationScopes
- W[x] S[ ] M[ ] F[ ] K[x] ListCustomDataIdentifiers
- W[x] S[ ] M[ ] F[ ] K[x] ListFindings
- W[x] S[ ] M[ ] F[ ] K[x] ListFindingsFilters
- W[x] S[ ] M[x] F[ ] K[ ] ListInvitations
- W[x] S[ ] M[ ] F[ ] K[ ] ListManagedDataIdentifiers
- W[x] S[ ] M[x] F[ ] K[ ] ListMembers
- W[x] S[ ] M[x] F[ ] K[ ] ListOrganizationAdminAccounts
- W[ ] S[x] M[ ] F[ ] K[ ] ListResourceProfileArtifacts
- W[ ] S[x] M[ ] F[ ] K[ ] ListResourceProfileDetections
- W[x] S[ ] M[ ] F[ ] K[ ] ListSensitivityInspectionTemplates
- W[x] S[ ] M[ ] F[ ] K[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] PutClassificationExportConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] PutFindingsPublicationConfiguration
- W[ ] S[x] M[ ] F[ ] K[ ] SearchResources
- W[x] S[ ] M[ ] F[ ] K[ ] TagResource
- W[ ] S[x] M[ ] F[ ] K[ ] TestCustomDataIdentifier
- W[x] S[ ] M[ ] F[ ] K[ ] UntagResource
- W[x] S[ ] M[ ] F[ ] K[x] UpdateAllowList
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateAutomatedDiscoveryConfiguration
- W[x] S[ ] M[ ] F[ ] K[x] UpdateClassificationJob
- W[ ] S[x] M[ ] F[ ] K[ ] UpdateClassificationScope
- W[x] S[ ] M[ ] F[ ] K[x] UpdateFindingsFilter
- W[x] S[ ] M[ ] F[ ] K[x] UpdateMacieSession
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateMemberSession
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateOrganizationConfiguration
- W[ ] S[x] M[ ] F[ ] K[ ] UpdateResourceProfile
- W[ ] S[x] M[ ] F[ ] K[ ] UpdateResourceProfileDetections
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateRevealConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateSensitivityInspectionTemplate

Integration tests: 59/67 implemented operations tested (88.1%)
Untested implemented operations: 8

### winterbaume-managedblockchain (managedblockchain) - W: 27/27, S: 0/27, M: 20/27, F: 0/27, K: 0/27

- W[x] S[ ] M[ ] F[ ] K[ ] CreateAccessor
- W[x] S[ ] M[x] F[ ] K[ ] CreateMember
- W[x] S[ ] M[x] F[ ] K[ ] CreateNetwork
- W[x] S[ ] M[x] F[ ] K[ ] CreateNode
- W[x] S[ ] M[x] F[ ] K[ ] CreateProposal
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteAccessor
- W[x] S[ ] M[x] F[ ] K[ ] DeleteMember
- W[x] S[ ] M[x] F[ ] K[ ] DeleteNode
- W[x] S[ ] M[ ] F[ ] K[ ] GetAccessor
- W[x] S[ ] M[x] F[ ] K[ ] GetMember
- W[x] S[ ] M[x] F[ ] K[ ] GetNetwork
- W[x] S[ ] M[x] F[ ] K[ ] GetNode
- W[x] S[ ] M[x] F[ ] K[ ] GetProposal
- W[x] S[ ] M[ ] F[ ] K[ ] ListAccessors
- W[x] S[ ] M[x] F[ ] K[ ] ListInvitations
- W[x] S[ ] M[x] F[ ] K[ ] ListMembers
- W[x] S[ ] M[x] F[ ] K[ ] ListNetworks
- W[x] S[ ] M[x] F[ ] K[ ] ListNodes
- W[x] S[ ] M[x] F[ ] K[ ] ListProposalVotes
- W[x] S[ ] M[x] F[ ] K[ ] ListProposals
- W[x] S[ ] M[ ] F[ ] K[ ] ListTagsForResource
- W[x] S[ ] M[x] F[ ] K[ ] RejectInvitation
- W[x] S[ ] M[ ] F[ ] K[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UntagResource
- W[x] S[ ] M[x] F[ ] K[ ] UpdateMember
- W[x] S[ ] M[x] F[ ] K[ ] UpdateNode
- W[x] S[ ] M[x] F[ ] K[ ] VoteOnProposal

Integration tests: 27/27 implemented operations tested (100.0%)

### winterbaume-marketplacemetering (marketplace-metering) - W: 4/4, S: 0/4, M: 0/4, F: 0/4, K: 0/4

- W[x] S[ ] M[ ] F[ ] K[ ] BatchMeterUsage
- W[x] S[ ] M[ ] F[ ] K[ ] MeterUsage
- W[x] S[ ] M[ ] F[ ] K[ ] RegisterUsage
- W[x] S[ ] M[ ] F[ ] K[ ] ResolveCustomer

Integration tests: 4/4 implemented operations tested (100.0%)

### winterbaume-mediaconnect (mediaconnect) - W: 21/82, S: 0/82, M: 18/82, F: 0/82, K: 0/82

- W[ ] S[ ] M[ ] F[ ] K[ ] AddBridgeOutputs
- W[ ] S[ ] M[ ] F[ ] K[ ] AddBridgeSources
- W[ ] S[ ] M[ ] F[ ] K[ ] AddFlowMediaStreams
- W[x] S[ ] M[x] F[ ] K[ ] AddFlowOutputs
- W[x] S[ ] M[x] F[ ] K[ ] AddFlowSources
- W[x] S[ ] M[x] F[ ] K[ ] AddFlowVpcInterfaces
- W[ ] S[ ] M[ ] F[ ] K[ ] BatchGetRouterInput
- W[ ] S[ ] M[ ] F[ ] K[ ] BatchGetRouterNetworkInterface
- W[ ] S[ ] M[ ] F[ ] K[ ] BatchGetRouterOutput
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateBridge
- W[x] S[ ] M[x] F[ ] K[ ] CreateFlow
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateGateway
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateRouterInput
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateRouterNetworkInterface
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateRouterOutput
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteBridge
- W[x] S[ ] M[x] F[ ] K[ ] DeleteFlow
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteGateway
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteRouterInput
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteRouterNetworkInterface
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteRouterOutput
- W[ ] S[ ] M[ ] F[ ] K[ ] DeregisterGatewayInstance
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeBridge
- W[x] S[ ] M[x] F[ ] K[ ] DescribeFlow
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeFlowSourceMetadata
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeFlowSourceThumbnail
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeGateway
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeGatewayInstance
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeOffering
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeReservation
- W[ ] S[ ] M[ ] F[ ] K[ ] GetRouterInput
- W[ ] S[ ] M[ ] F[ ] K[ ] GetRouterInputSourceMetadata
- W[ ] S[ ] M[ ] F[ ] K[ ] GetRouterInputThumbnail
- W[ ] S[ ] M[ ] F[ ] K[ ] GetRouterNetworkInterface
- W[ ] S[ ] M[ ] F[ ] K[ ] GetRouterOutput
- W[x] S[ ] M[x] F[ ] K[ ] GrantFlowEntitlements
- W[ ] S[ ] M[ ] F[ ] K[ ] ListBridges
- W[ ] S[ ] M[ ] F[ ] K[ ] ListEntitlements
- W[x] S[ ] M[x] F[ ] K[ ] ListFlows
- W[ ] S[ ] M[ ] F[ ] K[ ] ListGatewayInstances
- W[ ] S[ ] M[ ] F[ ] K[ ] ListGateways
- W[ ] S[ ] M[ ] F[ ] K[ ] ListOfferings
- W[ ] S[ ] M[ ] F[ ] K[ ] ListReservations
- W[ ] S[ ] M[ ] F[ ] K[ ] ListRouterInputs
- W[ ] S[ ] M[ ] F[ ] K[ ] ListRouterNetworkInterfaces
- W[ ] S[ ] M[ ] F[ ] K[ ] ListRouterOutputs
- W[ ] S[ ] M[ ] F[ ] K[ ] ListTagsForGlobalResource
- W[x] S[ ] M[x] F[ ] K[ ] ListTagsForResource
- W[ ] S[ ] M[ ] F[ ] K[ ] PurchaseOffering
- W[ ] S[ ] M[ ] F[ ] K[ ] RemoveBridgeOutput
- W[ ] S[ ] M[ ] F[ ] K[ ] RemoveBridgeSource
- W[ ] S[ ] M[ ] F[ ] K[ ] RemoveFlowMediaStream
- W[x] S[ ] M[x] F[ ] K[ ] RemoveFlowOutput
- W[x] S[ ] M[ ] F[ ] K[ ] RemoveFlowSource
- W[x] S[ ] M[x] F[ ] K[ ] RemoveFlowVpcInterface
- W[ ] S[ ] M[ ] F[ ] K[ ] RestartRouterInput
- W[ ] S[ ] M[ ] F[ ] K[ ] RestartRouterOutput
- W[x] S[ ] M[x] F[ ] K[ ] RevokeFlowEntitlement
- W[x] S[ ] M[x] F[ ] K[ ] StartFlow
- W[ ] S[ ] M[ ] F[ ] K[ ] StartRouterInput
- W[ ] S[ ] M[ ] F[ ] K[ ] StartRouterOutput
- W[x] S[ ] M[x] F[ ] K[ ] StopFlow
- W[ ] S[ ] M[ ] F[ ] K[ ] StopRouterInput
- W[ ] S[ ] M[ ] F[ ] K[ ] StopRouterOutput
- W[ ] S[ ] M[ ] F[ ] K[ ] TagGlobalResource
- W[x] S[ ] M[x] F[ ] K[ ] TagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] TakeRouterInput
- W[ ] S[ ] M[ ] F[ ] K[ ] UntagGlobalResource
- W[x] S[ ] M[ ] F[ ] K[ ] UntagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateBridge
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateBridgeOutput
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateBridgeSource
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateBridgeState
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateFlow
- W[x] S[ ] M[x] F[ ] K[ ] UpdateFlowEntitlement
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateFlowMediaStream
- W[x] S[ ] M[x] F[ ] K[ ] UpdateFlowOutput
- W[x] S[ ] M[x] F[ ] K[ ] UpdateFlowSource
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateGatewayInstance
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateRouterInput
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateRouterNetworkInterface
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateRouterOutput

Integration tests: 21/21 implemented operations tested (100.0%)

### winterbaume-medialive (medialive) - W: 16/123, S: 0/123, M: 12/123, F: 0/123, K: 0/123

- W[ ] S[ ] M[ ] F[ ] K[ ] AcceptInputDeviceTransfer
- W[ ] S[ ] M[ ] F[ ] K[ ] BatchDelete
- W[ ] S[ ] M[ ] F[ ] K[ ] BatchStart
- W[ ] S[ ] M[ ] F[ ] K[ ] BatchStop
- W[ ] S[ ] M[ ] F[ ] K[ ] BatchUpdateSchedule
- W[ ] S[ ] M[ ] F[ ] K[ ] CancelInputDeviceTransfer
- W[ ] S[ ] M[ ] F[ ] K[ ] ClaimDevice
- W[x] S[ ] M[x] F[ ] K[ ] CreateChannel
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateChannelPlacementGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateCloudWatchAlarmTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateCloudWatchAlarmTemplateGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateCluster
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateEventBridgeRuleTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateEventBridgeRuleTemplateGroup
- W[x] S[ ] M[x] F[ ] K[ ] CreateInput
- W[x] S[ ] M[ ] F[ ] K[ ] CreateInputSecurityGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateMultiplex
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateMultiplexProgram
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateNetwork
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateNode
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateNodeRegistrationScript
- W[ ] S[ ] M[ ] F[ ] K[ ] CreatePartnerInput
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateSdiSource
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateSignalMap
- W[x] S[ ] M[ ] F[ ] K[ ] CreateTags
- W[x] S[ ] M[x] F[ ] K[ ] DeleteChannel
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteChannelPlacementGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteCloudWatchAlarmTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteCloudWatchAlarmTemplateGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteCluster
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteEventBridgeRuleTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteEventBridgeRuleTemplateGroup
- W[x] S[ ] M[x] F[ ] K[ ] DeleteInput
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteInputSecurityGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteMultiplex
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteMultiplexProgram
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteNetwork
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteNode
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteReservation
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteSchedule
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteSdiSource
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteSignalMap
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteTags
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeAccountConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] DescribeChannel
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeChannelPlacementGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeCluster
- W[x] S[ ] M[x] F[ ] K[ ] DescribeInput
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeInputDevice
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeInputDeviceThumbnail
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeInputSecurityGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeMultiplex
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeMultiplexProgram
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeNetwork
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeNode
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeOffering
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeReservation
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeSchedule
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeSdiSource
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeThumbnails
- W[ ] S[ ] M[ ] F[ ] K[ ] GetCloudWatchAlarmTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] GetCloudWatchAlarmTemplateGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] GetEventBridgeRuleTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] GetEventBridgeRuleTemplateGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] GetSignalMap
- W[ ] S[ ] M[ ] F[ ] K[ ] ListAlerts
- W[ ] S[ ] M[ ] F[ ] K[ ] ListChannelPlacementGroups
- W[x] S[ ] M[x] F[ ] K[ ] ListChannels
- W[ ] S[ ] M[ ] F[ ] K[ ] ListCloudWatchAlarmTemplateGroups
- W[ ] S[ ] M[ ] F[ ] K[ ] ListCloudWatchAlarmTemplates
- W[ ] S[ ] M[ ] F[ ] K[ ] ListClusterAlerts
- W[ ] S[ ] M[ ] F[ ] K[ ] ListClusters
- W[ ] S[ ] M[ ] F[ ] K[ ] ListEventBridgeRuleTemplateGroups
- W[ ] S[ ] M[ ] F[ ] K[ ] ListEventBridgeRuleTemplates
- W[ ] S[ ] M[ ] F[ ] K[ ] ListInputDeviceTransfers
- W[ ] S[ ] M[ ] F[ ] K[ ] ListInputDevices
- W[ ] S[ ] M[ ] F[ ] K[ ] ListInputSecurityGroups
- W[x] S[ ] M[x] F[ ] K[ ] ListInputs
- W[ ] S[ ] M[ ] F[ ] K[ ] ListMultiplexAlerts
- W[ ] S[ ] M[ ] F[ ] K[ ] ListMultiplexPrograms
- W[ ] S[ ] M[ ] F[ ] K[ ] ListMultiplexes
- W[ ] S[ ] M[ ] F[ ] K[ ] ListNetworks
- W[ ] S[ ] M[ ] F[ ] K[ ] ListNodes
- W[ ] S[ ] M[ ] F[ ] K[ ] ListOfferings
- W[ ] S[ ] M[ ] F[ ] K[ ] ListReservations
- W[ ] S[ ] M[ ] F[ ] K[ ] ListSdiSources
- W[ ] S[ ] M[ ] F[ ] K[ ] ListSignalMaps
- W[x] S[ ] M[ ] F[ ] K[ ] ListTagsForResource
- W[ ] S[ ] M[ ] F[ ] K[ ] ListVersions
- W[ ] S[ ] M[ ] F[ ] K[ ] PurchaseOffering
- W[ ] S[ ] M[ ] F[ ] K[ ] RebootInputDevice
- W[ ] S[ ] M[ ] F[ ] K[ ] RejectInputDeviceTransfer
- W[ ] S[ ] M[ ] F[ ] K[ ] RestartChannelPipelines
- W[x] S[ ] M[x] F[ ] K[ ] StartChannel
- W[ ] S[ ] M[ ] F[ ] K[ ] StartDeleteMonitorDeployment
- W[ ] S[ ] M[ ] F[ ] K[ ] StartInputDevice
- W[ ] S[ ] M[ ] F[ ] K[ ] StartInputDeviceMaintenanceWindow
- W[ ] S[ ] M[ ] F[ ] K[ ] StartMonitorDeployment
- W[ ] S[ ] M[ ] F[ ] K[ ] StartMultiplex
- W[ ] S[ ] M[ ] F[ ] K[ ] StartUpdateSignalMap
- W[x] S[ ] M[x] F[ ] K[ ] StopChannel
- W[ ] S[ ] M[ ] F[ ] K[ ] StopInputDevice
- W[ ] S[ ] M[ ] F[ ] K[ ] StopMultiplex
- W[ ] S[ ] M[ ] F[ ] K[ ] TransferInputDevice
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateAccountConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] UpdateChannel
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateChannelClass
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateChannelPlacementGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateCloudWatchAlarmTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateCloudWatchAlarmTemplateGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateCluster
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateEventBridgeRuleTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateEventBridgeRuleTemplateGroup
- W[x] S[ ] M[x] F[ ] K[ ] UpdateInput
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateInputDevice
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateInputSecurityGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateMultiplex
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateMultiplexProgram
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateNetwork
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateNode
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateNodeState
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateReservation
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateSdiSource

Integration tests: 12/16 implemented operations tested (75.0%)
Untested implemented operations: 4

### winterbaume-mediapackage (mediapackage) - W: 9/19, S: 0/19, M: 9/19, F: 0/19, K: 0/19

- W[ ] S[ ] M[ ] F[ ] K[ ] ConfigureLogs
- W[x] S[ ] M[x] F[ ] K[ ] CreateChannel
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateHarvestJob
- W[x] S[ ] M[x] F[ ] K[ ] CreateOriginEndpoint
- W[x] S[ ] M[x] F[ ] K[ ] DeleteChannel
- W[x] S[ ] M[x] F[ ] K[ ] DeleteOriginEndpoint
- W[x] S[ ] M[x] F[ ] K[ ] DescribeChannel
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeHarvestJob
- W[x] S[ ] M[x] F[ ] K[ ] DescribeOriginEndpoint
- W[x] S[ ] M[x] F[ ] K[ ] ListChannels
- W[ ] S[ ] M[ ] F[ ] K[ ] ListHarvestJobs
- W[x] S[ ] M[x] F[ ] K[ ] ListOriginEndpoints
- W[ ] S[ ] M[ ] F[ ] K[ ] ListTagsForResource
- W[ ] S[ ] M[ ] F[ ] K[ ] RotateChannelCredentials
- W[ ] S[ ] M[ ] F[ ] K[ ] RotateIngestEndpointCredentials
- W[ ] S[ ] M[ ] F[ ] K[ ] TagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] UntagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateChannel
- W[x] S[ ] M[x] F[ ] K[ ] UpdateOriginEndpoint

Integration tests: 9/9 implemented operations tested (100.0%)

### winterbaume-mediapackagev2 (mediapackagev2) - W: 7/30, S: 0/30, M: 7/30, F: 0/30, K: 0/30

- W[ ] S[ ] M[ ] F[ ] K[ ] CancelHarvestJob
- W[x] S[ ] M[x] F[ ] K[ ] CreateChannel
- W[x] S[ ] M[x] F[ ] K[ ] CreateChannelGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateHarvestJob
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateOriginEndpoint
- W[x] S[ ] M[x] F[ ] K[ ] DeleteChannel
- W[x] S[ ] M[x] F[ ] K[ ] DeleteChannelGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteChannelPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteOriginEndpoint
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteOriginEndpointPolicy
- W[x] S[ ] M[x] F[ ] K[ ] GetChannel
- W[x] S[ ] M[x] F[ ] K[ ] GetChannelGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] GetChannelPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] GetHarvestJob
- W[ ] S[ ] M[ ] F[ ] K[ ] GetOriginEndpoint
- W[ ] S[ ] M[ ] F[ ] K[ ] GetOriginEndpointPolicy
- W[x] S[ ] M[x] F[ ] K[ ] ListChannelGroups
- W[ ] S[ ] M[ ] F[ ] K[ ] ListChannels
- W[ ] S[ ] M[ ] F[ ] K[ ] ListHarvestJobs
- W[ ] S[ ] M[ ] F[ ] K[ ] ListOriginEndpoints
- W[ ] S[ ] M[ ] F[ ] K[ ] ListTagsForResource
- W[ ] S[ ] M[ ] F[ ] K[ ] PutChannelPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] PutOriginEndpointPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] ResetChannelState
- W[ ] S[ ] M[ ] F[ ] K[ ] ResetOriginEndpointState
- W[ ] S[ ] M[ ] F[ ] K[ ] TagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] UntagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateChannel
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateChannelGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateOriginEndpoint

Integration tests: 7/7 implemented operations tested (100.0%)

### winterbaume-mediastore (mediastore) - W: 11/21, S: 0/21, M: 11/21, F: 0/21, K: 0/21

- W[x] S[ ] M[x] F[ ] K[ ] CreateContainer
- W[x] S[ ] M[x] F[ ] K[ ] DeleteContainer
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteContainerPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteCorsPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteLifecyclePolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteMetricPolicy
- W[x] S[ ] M[x] F[ ] K[ ] DescribeContainer
- W[x] S[ ] M[x] F[ ] K[ ] GetContainerPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] GetCorsPolicy
- W[x] S[ ] M[x] F[ ] K[ ] GetLifecyclePolicy
- W[x] S[ ] M[x] F[ ] K[ ] GetMetricPolicy
- W[x] S[ ] M[x] F[ ] K[ ] ListContainers
- W[x] S[ ] M[x] F[ ] K[ ] ListTagsForResource
- W[x] S[ ] M[x] F[ ] K[ ] PutContainerPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] PutCorsPolicy
- W[x] S[ ] M[x] F[ ] K[ ] PutLifecyclePolicy
- W[x] S[ ] M[x] F[ ] K[ ] PutMetricPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] StartAccessLogging
- W[ ] S[ ] M[ ] F[ ] K[ ] StopAccessLogging
- W[ ] S[ ] M[ ] F[ ] K[ ] TagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] UntagResource

Integration tests: 11/11 implemented operations tested (100.0%)

### winterbaume-mediastoredata (mediastore-data) - W: 5/5, S: 0/5, M: 4/5, F: 0/5, K: 0/5

- W[x] S[ ] M[x] F[ ] K[ ] DeleteObject
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeObject
- W[x] S[ ] M[x] F[ ] K[ ] GetObject
- W[x] S[ ] M[x] F[ ] K[ ] ListItems
- W[x] S[ ] M[x] F[ ] K[ ] PutObject

Integration tests: 5/5 implemented operations tested (100.0%)

### winterbaume-memorydb (memorydb) - W: 13/45, S: 0/45, M: 13/45, F: 0/45, K: 10/45

- W[ ] S[ ] M[ ] F[ ] K[ ] BatchUpdateCluster
- W[ ] S[ ] M[ ] F[ ] K[ ] CopySnapshot
- W[ ] S[ ] M[ ] F[ ] K[x] CreateACL
- W[x] S[ ] M[x] F[ ] K[x] CreateCluster
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateMultiRegionCluster
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateParameterGroup
- W[x] S[ ] M[x] F[ ] K[ ] CreateSnapshot
- W[x] S[ ] M[x] F[ ] K[ ] CreateSubnetGroup
- W[ ] S[ ] M[ ] F[ ] K[x] CreateUser
- W[ ] S[ ] M[ ] F[ ] K[x] DeleteACL
- W[x] S[ ] M[x] F[ ] K[x] DeleteCluster
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteMultiRegionCluster
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteParameterGroup
- W[x] S[ ] M[x] F[ ] K[ ] DeleteSnapshot
- W[x] S[ ] M[x] F[ ] K[ ] DeleteSubnetGroup
- W[ ] S[ ] M[ ] F[ ] K[x] DeleteUser
- W[ ] S[ ] M[ ] F[ ] K[x] DescribeACLs
- W[x] S[ ] M[x] F[ ] K[x] DescribeClusters
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeEngineVersions
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeEvents
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeMultiRegionClusters
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeMultiRegionParameterGroups
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeMultiRegionParameters
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeParameterGroups
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeParameters
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeReservedNodes
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeReservedNodesOfferings
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeServiceUpdates
- W[x] S[ ] M[x] F[ ] K[ ] DescribeSnapshots
- W[x] S[ ] M[x] F[ ] K[ ] DescribeSubnetGroups
- W[ ] S[ ] M[ ] F[ ] K[x] DescribeUsers
- W[ ] S[ ] M[ ] F[ ] K[ ] FailoverShard
- W[ ] S[ ] M[ ] F[ ] K[ ] ListAllowedMultiRegionClusterUpdates
- W[ ] S[ ] M[ ] F[ ] K[ ] ListAllowedNodeTypeUpdates
- W[x] S[ ] M[x] F[ ] K[ ] ListTags
- W[ ] S[ ] M[ ] F[ ] K[ ] PurchaseReservedNodesOffering
- W[ ] S[ ] M[ ] F[ ] K[ ] ResetParameterGroup
- W[x] S[ ] M[x] F[ ] K[ ] TagResource
- W[x] S[ ] M[x] F[ ] K[ ] UntagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateACL
- W[x] S[ ] M[x] F[ ] K[x] UpdateCluster
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateMultiRegionCluster
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateParameterGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateSubnetGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateUser

Integration tests: 13/13 implemented operations tested (100.0%)

### winterbaume-mq (mq) - W: 23/24, S: 1/24, M: 19/24, F: 0/24, K: 6/24

- W[x] S[ ] M[x] F[ ] K[x] CreateBroker
- W[x] S[ ] M[x] F[ ] K[x] CreateConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] CreateTags
- W[x] S[ ] M[x] F[ ] K[ ] CreateUser
- W[x] S[ ] M[x] F[ ] K[x] DeleteBroker
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] DeleteTags
- W[x] S[ ] M[x] F[ ] K[ ] DeleteUser
- W[x] S[ ] M[x] F[ ] K[x] DescribeBroker
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeBrokerEngineTypes
- W[ ] S[x] M[ ] F[ ] K[ ] DescribeBrokerInstanceOptions
- W[x] S[ ] M[x] F[ ] K[ ] DescribeConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] DescribeConfigurationRevision
- W[x] S[ ] M[x] F[ ] K[ ] DescribeUser
- W[x] S[ ] M[x] F[ ] K[x] ListBrokers
- W[x] S[ ] M[ ] F[ ] K[ ] ListConfigurationRevisions
- W[x] S[ ] M[x] F[ ] K[ ] ListConfigurations
- W[x] S[ ] M[x] F[ ] K[ ] ListTags
- W[x] S[ ] M[x] F[ ] K[ ] ListUsers
- W[x] S[ ] M[ ] F[ ] K[ ] Promote
- W[x] S[ ] M[x] F[ ] K[ ] RebootBroker
- W[x] S[ ] M[x] F[ ] K[x] UpdateBroker
- W[x] S[ ] M[x] F[ ] K[ ] UpdateConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] UpdateUser

Integration tests: 21/23 implemented operations tested (91.3%)
Untested implemented operations: 2

### winterbaume-neptune (neptune) - W: 64/70, S: 6/70, M: 47/70, F: 0/70, K: 6/70

- W[x] S[ ] M[x] F[ ] K[ ] AddRoleToDBCluster
- W[x] S[ ] M[ ] F[ ] K[ ] AddSourceIdentifierToSubscription
- W[x] S[ ] M[x] F[ ] K[ ] AddTagsToResource
- W[ ] S[x] M[ ] F[ ] K[ ] ApplyPendingMaintenanceAction
- W[x] S[ ] M[x] F[ ] K[ ] CopyDBClusterParameterGroup
- W[x] S[ ] M[x] F[ ] K[ ] CopyDBClusterSnapshot
- W[x] S[ ] M[x] F[ ] K[ ] CopyDBParameterGroup
- W[x] S[ ] M[x] F[ ] K[x] CreateDBCluster
- W[x] S[ ] M[ ] F[ ] K[ ] CreateDBClusterEndpoint
- W[x] S[ ] M[x] F[ ] K[ ] CreateDBClusterParameterGroup
- W[x] S[ ] M[x] F[ ] K[ ] CreateDBClusterSnapshot
- W[x] S[ ] M[x] F[ ] K[x] CreateDBInstance
- W[x] S[ ] M[x] F[ ] K[ ] CreateDBParameterGroup
- W[x] S[ ] M[ ] F[ ] K[ ] CreateDBSubnetGroup
- W[x] S[ ] M[x] F[ ] K[ ] CreateEventSubscription
- W[x] S[ ] M[x] F[ ] K[ ] CreateGlobalCluster
- W[x] S[ ] M[x] F[ ] K[x] DeleteDBCluster
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteDBClusterEndpoint
- W[x] S[ ] M[x] F[ ] K[ ] DeleteDBClusterParameterGroup
- W[x] S[ ] M[x] F[ ] K[ ] DeleteDBClusterSnapshot
- W[x] S[ ] M[x] F[ ] K[x] DeleteDBInstance
- W[x] S[ ] M[x] F[ ] K[ ] DeleteDBParameterGroup
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteDBSubnetGroup
- W[x] S[ ] M[x] F[ ] K[ ] DeleteEventSubscription
- W[x] S[ ] M[x] F[ ] K[ ] DeleteGlobalCluster
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeDBClusterEndpoints
- W[x] S[ ] M[x] F[ ] K[ ] DescribeDBClusterParameterGroups
- W[x] S[ ] M[x] F[ ] K[ ] DescribeDBClusterParameters
- W[x] S[ ] M[x] F[ ] K[ ] DescribeDBClusterSnapshotAttributes
- W[x] S[ ] M[x] F[ ] K[ ] DescribeDBClusterSnapshots
- W[x] S[ ] M[x] F[ ] K[x] DescribeDBClusters
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeDBEngineVersions
- W[x] S[ ] M[x] F[ ] K[x] DescribeDBInstances
- W[x] S[ ] M[x] F[ ] K[ ] DescribeDBParameterGroups
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeDBParameters
- W[x] S[ ] M[x] F[ ] K[ ] DescribeDBSubnetGroups
- W[ ] S[x] M[ ] F[ ] K[ ] DescribeEngineDefaultClusterParameters
- W[ ] S[x] M[ ] F[ ] K[ ] DescribeEngineDefaultParameters
- W[ ] S[x] M[ ] F[ ] K[ ] DescribeEventCategories
- W[x] S[ ] M[x] F[ ] K[ ] DescribeEventSubscriptions
- W[ ] S[x] M[x] F[ ] K[ ] DescribeEvents
- W[x] S[ ] M[x] F[ ] K[ ] DescribeGlobalClusters
- W[x] S[ ] M[x] F[ ] K[ ] DescribeOrderableDBInstanceOptions
- W[ ] S[x] M[ ] F[ ] K[ ] DescribePendingMaintenanceActions
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeValidDBInstanceModifications
- W[x] S[ ] M[x] F[ ] K[ ] FailoverDBCluster
- W[x] S[ ] M[ ] F[ ] K[ ] FailoverGlobalCluster
- W[x] S[ ] M[x] F[ ] K[ ] ListTagsForResource
- W[x] S[ ] M[x] F[ ] K[ ] ModifyDBCluster
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyDBClusterEndpoint
- W[x] S[ ] M[x] F[ ] K[ ] ModifyDBClusterParameterGroup
- W[x] S[ ] M[x] F[ ] K[ ] ModifyDBClusterSnapshotAttribute
- W[x] S[ ] M[x] F[ ] K[ ] ModifyDBInstance
- W[x] S[ ] M[x] F[ ] K[ ] ModifyDBParameterGroup
- W[x] S[ ] M[x] F[ ] K[ ] ModifyDBSubnetGroup
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyEventSubscription
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyGlobalCluster
- W[x] S[ ] M[x] F[ ] K[ ] PromoteReadReplicaDBCluster
- W[x] S[ ] M[x] F[ ] K[ ] RebootDBInstance
- W[x] S[ ] M[x] F[ ] K[ ] RemoveFromGlobalCluster
- W[x] S[ ] M[ ] F[ ] K[ ] RemoveRoleFromDBCluster
- W[x] S[ ] M[ ] F[ ] K[ ] RemoveSourceIdentifierFromSubscription
- W[x] S[ ] M[x] F[ ] K[ ] RemoveTagsFromResource
- W[x] S[ ] M[ ] F[ ] K[ ] ResetDBClusterParameterGroup
- W[x] S[ ] M[ ] F[ ] K[ ] ResetDBParameterGroup
- W[x] S[ ] M[x] F[ ] K[ ] RestoreDBClusterFromSnapshot
- W[x] S[ ] M[x] F[ ] K[ ] RestoreDBClusterToPointInTime
- W[x] S[ ] M[x] F[ ] K[ ] StartDBCluster
- W[x] S[ ] M[x] F[ ] K[ ] StopDBCluster
- W[x] S[ ] M[ ] F[ ] K[ ] SwitchoverGlobalCluster

Integration tests: 24/64 implemented operations tested (37.5%)
Untested implemented operations: 40

### winterbaume-networkfirewall (network-firewall) - W: 79/79, S: 0/79, M: 5/79, F: 0/79, K: 0/79

- W[x] S[ ] M[ ] F[ ] K[ ] AcceptNetworkFirewallTransitGatewayAttachment
- W[x] S[ ] M[ ] F[ ] K[ ] AssociateAvailabilityZones
- W[x] S[ ] M[ ] F[ ] K[ ] AssociateFirewallPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] AssociateSubnets
- W[x] S[ ] M[ ] F[ ] K[ ] AttachRuleGroupsToProxyConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] CreateFirewall
- W[x] S[ ] M[ ] F[ ] K[ ] CreateFirewallPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] CreateProxy
- W[x] S[ ] M[ ] F[ ] K[ ] CreateProxyConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] CreateProxyRuleGroup
- W[x] S[ ] M[ ] F[ ] K[ ] CreateProxyRules
- W[x] S[ ] M[ ] F[ ] K[ ] CreateRuleGroup
- W[x] S[ ] M[ ] F[ ] K[ ] CreateTLSInspectionConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] CreateVpcEndpointAssociation
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteFirewall
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteFirewallPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteNetworkFirewallTransitGatewayAttachment
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteProxy
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteProxyConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteProxyRuleGroup
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteProxyRules
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteResourcePolicy
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteRuleGroup
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteTLSInspectionConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteVpcEndpointAssociation
- W[x] S[ ] M[x] F[ ] K[ ] DescribeFirewall
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeFirewallMetadata
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeFirewallPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeFlowOperation
- W[x] S[ ] M[x] F[ ] K[ ] DescribeLoggingConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeProxy
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeProxyConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeProxyRule
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeProxyRuleGroup
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeResourcePolicy
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeRuleGroup
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeRuleGroupMetadata
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeRuleGroupSummary
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeTLSInspectionConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeVpcEndpointAssociation
- W[x] S[ ] M[ ] F[ ] K[ ] DetachRuleGroupsFromProxyConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] DisassociateAvailabilityZones
- W[x] S[ ] M[ ] F[ ] K[ ] DisassociateSubnets
- W[x] S[ ] M[ ] F[ ] K[ ] GetAnalysisReportResults
- W[x] S[ ] M[ ] F[ ] K[ ] ListAnalysisReports
- W[x] S[ ] M[ ] F[ ] K[ ] ListFirewallPolicies
- W[x] S[ ] M[x] F[ ] K[ ] ListFirewalls
- W[x] S[ ] M[ ] F[ ] K[ ] ListFlowOperationResults
- W[x] S[ ] M[ ] F[ ] K[ ] ListFlowOperations
- W[x] S[ ] M[ ] F[ ] K[ ] ListProxies
- W[x] S[ ] M[ ] F[ ] K[ ] ListProxyConfigurations
- W[x] S[ ] M[ ] F[ ] K[ ] ListProxyRuleGroups
- W[x] S[ ] M[ ] F[ ] K[ ] ListRuleGroups
- W[x] S[ ] M[ ] F[ ] K[ ] ListTLSInspectionConfigurations
- W[x] S[ ] M[ ] F[ ] K[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] ListVpcEndpointAssociations
- W[x] S[ ] M[ ] F[ ] K[ ] PutResourcePolicy
- W[x] S[ ] M[ ] F[ ] K[ ] RejectNetworkFirewallTransitGatewayAttachment
- W[x] S[ ] M[ ] F[ ] K[ ] StartAnalysisReport
- W[x] S[ ] M[ ] F[ ] K[ ] StartFlowCapture
- W[x] S[ ] M[ ] F[ ] K[ ] StartFlowFlush
- W[x] S[ ] M[ ] F[ ] K[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateAvailabilityZoneChangeProtection
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateFirewallAnalysisSettings
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateFirewallDeleteProtection
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateFirewallDescription
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateFirewallEncryptionConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateFirewallPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateFirewallPolicyChangeProtection
- W[x] S[ ] M[x] F[ ] K[ ] UpdateLoggingConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateProxy
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateProxyConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateProxyRule
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateProxyRuleGroupPriorities
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateProxyRulePriorities
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateRuleGroup
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateSubnetChangeProtection
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateTLSInspectionConfiguration

Integration tests: 42/79 implemented operations tested (53.2%)
Untested implemented operations: 37

### winterbaume-networkmanager (networkmanager) - W: 53/95, S: 0/95, M: 18/95, F: 0/95, K: 0/95

Terraform E2E: 8 tests across 6 terraform resource types

Resource types: aws_networkmanager_connection, aws_networkmanager_core_network, aws_networkmanager_device, aws_networkmanager_global_network, aws_networkmanager_link, aws_networkmanager_site

- W[x] S[ ] M[ ] F[ ] K[ ] AcceptAttachment
- W[x] S[ ] M[ ] F[ ] K[ ] AssociateConnectPeer
- W[x] S[ ] M[ ] F[ ] K[ ] AssociateCustomerGateway
- W[x] S[ ] M[ ] F[ ] K[ ] AssociateLink
- W[x] S[ ] M[ ] F[ ] K[ ] AssociateTransitGatewayConnectPeer
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateConnectAttachment
- W[x] S[ ] M[ ] F[ ] K[ ] CreateConnectPeer
- W[x] S[ ] M[ ] F[ ] K[ ] CreateConnection
- W[x] S[ ] M[x] F[ ] K[ ] CreateCoreNetwork
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateCoreNetworkPrefixListAssociation
- W[x] S[ ] M[x] F[ ] K[ ] CreateDevice
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateDirectConnectGatewayAttachment
- W[x] S[ ] M[x] F[ ] K[ ] CreateGlobalNetwork
- W[x] S[ ] M[x] F[ ] K[ ] CreateLink
- W[x] S[ ] M[x] F[ ] K[ ] CreateSite
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateSiteToSiteVpnAttachment
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateTransitGatewayPeering
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateTransitGatewayRouteTableAttachment
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateVpcAttachment
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteAttachment
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteConnectPeer
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteConnection
- W[x] S[ ] M[x] F[ ] K[ ] DeleteCoreNetwork
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteCoreNetworkPolicyVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteCoreNetworkPrefixListAssociation
- W[x] S[ ] M[x] F[ ] K[ ] DeleteDevice
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteGlobalNetwork
- W[x] S[ ] M[x] F[ ] K[ ] DeleteLink
- W[ ] S[ ] M[ ] F[ ] K[ ] DeletePeering
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteResourcePolicy
- W[x] S[ ] M[x] F[ ] K[ ] DeleteSite
- W[x] S[ ] M[ ] F[ ] K[ ] DeregisterTransitGateway
- W[x] S[ ] M[x] F[ ] K[ ] DescribeGlobalNetworks
- W[x] S[ ] M[ ] F[ ] K[ ] DisassociateConnectPeer
- W[x] S[ ] M[ ] F[ ] K[ ] DisassociateCustomerGateway
- W[x] S[ ] M[ ] F[ ] K[ ] DisassociateLink
- W[x] S[ ] M[ ] F[ ] K[ ] DisassociateTransitGatewayConnectPeer
- W[ ] S[ ] M[ ] F[ ] K[ ] ExecuteCoreNetworkChangeSet
- W[ ] S[ ] M[ ] F[ ] K[ ] GetConnectAttachment
- W[x] S[ ] M[ ] F[ ] K[ ] GetConnectPeer
- W[x] S[ ] M[ ] F[ ] K[ ] GetConnectPeerAssociations
- W[x] S[ ] M[ ] F[ ] K[ ] GetConnections
- W[x] S[ ] M[x] F[ ] K[ ] GetCoreNetwork
- W[ ] S[ ] M[ ] F[ ] K[ ] GetCoreNetworkChangeEvents
- W[ ] S[ ] M[ ] F[ ] K[ ] GetCoreNetworkChangeSet
- W[ ] S[ ] M[ ] F[ ] K[ ] GetCoreNetworkPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] GetCustomerGatewayAssociations
- W[x] S[ ] M[x] F[ ] K[ ] GetDevices
- W[ ] S[ ] M[ ] F[ ] K[ ] GetDirectConnectGatewayAttachment
- W[x] S[ ] M[ ] F[ ] K[ ] GetLinkAssociations
- W[x] S[ ] M[x] F[ ] K[ ] GetLinks
- W[ ] S[ ] M[ ] F[ ] K[ ] GetNetworkResourceCounts
- W[ ] S[ ] M[ ] F[ ] K[ ] GetNetworkResourceRelationships
- W[ ] S[ ] M[ ] F[ ] K[ ] GetNetworkResources
- W[ ] S[ ] M[ ] F[ ] K[ ] GetNetworkRoutes
- W[ ] S[ ] M[ ] F[ ] K[ ] GetNetworkTelemetry
- W[ ] S[ ] M[ ] F[ ] K[ ] GetResourcePolicy
- W[x] S[ ] M[ ] F[ ] K[ ] GetRouteAnalysis
- W[ ] S[ ] M[ ] F[ ] K[ ] GetSiteToSiteVpnAttachment
- W[x] S[ ] M[x] F[ ] K[ ] GetSites
- W[x] S[ ] M[ ] F[ ] K[ ] GetTransitGatewayConnectPeerAssociations
- W[ ] S[ ] M[ ] F[ ] K[ ] GetTransitGatewayPeering
- W[x] S[ ] M[ ] F[ ] K[ ] GetTransitGatewayRegistrations
- W[ ] S[ ] M[ ] F[ ] K[ ] GetTransitGatewayRouteTableAttachment
- W[ ] S[ ] M[ ] F[ ] K[ ] GetVpcAttachment
- W[ ] S[ ] M[ ] F[ ] K[ ] ListAttachmentRoutingPolicyAssociations
- W[x] S[ ] M[ ] F[ ] K[ ] ListAttachments
- W[x] S[ ] M[ ] F[ ] K[ ] ListConnectPeers
- W[ ] S[ ] M[ ] F[ ] K[ ] ListCoreNetworkPolicyVersions
- W[ ] S[ ] M[ ] F[ ] K[ ] ListCoreNetworkPrefixListAssociations
- W[ ] S[ ] M[ ] F[ ] K[ ] ListCoreNetworkRoutingInformation
- W[x] S[ ] M[x] F[ ] K[ ] ListCoreNetworks
- W[ ] S[ ] M[ ] F[ ] K[ ] ListOrganizationServiceAccessStatus
- W[ ] S[ ] M[ ] F[ ] K[ ] ListPeerings
- W[x] S[ ] M[x] F[ ] K[ ] ListTagsForResource
- W[ ] S[ ] M[ ] F[ ] K[ ] PutAttachmentRoutingPolicyLabel
- W[ ] S[ ] M[ ] F[ ] K[ ] PutCoreNetworkPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] PutResourcePolicy
- W[x] S[ ] M[ ] F[ ] K[ ] RegisterTransitGateway
- W[x] S[ ] M[ ] F[ ] K[ ] RejectAttachment
- W[ ] S[ ] M[ ] F[ ] K[ ] RemoveAttachmentRoutingPolicyLabel
- W[ ] S[ ] M[ ] F[ ] K[ ] RestoreCoreNetworkPolicyVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] StartOrganizationServiceAccessUpdate
- W[x] S[ ] M[ ] F[ ] K[ ] StartRouteAnalysis
- W[x] S[ ] M[x] F[ ] K[ ] TagResource
- W[x] S[ ] M[x] F[ ] K[ ] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateConnection
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateCoreNetwork
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateDevice
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateDirectConnectGatewayAttachment
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateGlobalNetwork
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateLink
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateNetworkResourceMetadata
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateSite
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateVpcAttachment

Integration tests: 44/53 implemented operations tested (83.0%)
Untested implemented operations: 9

### winterbaume-opensearch (opensearch) - W: 44/88, S: 0/88, M: 11/88, F: 0/88, K: 0/88

- W[x] S[ ] M[ ] F[ ] K[ ] AcceptInboundConnection
- W[x] S[ ] M[ ] F[ ] K[ ] AddDataSource
- W[x] S[ ] M[ ] F[ ] K[ ] AddDirectQueryDataSource
- W[x] S[ ] M[x] F[ ] K[ ] AddTags
- W[x] S[ ] M[ ] F[ ] K[ ] AssociatePackage
- W[ ] S[ ] M[ ] F[ ] K[ ] AssociatePackages
- W[ ] S[ ] M[ ] F[ ] K[ ] AuthorizeVpcEndpointAccess
- W[x] S[ ] M[ ] F[ ] K[ ] CancelDomainConfigChange
- W[ ] S[ ] M[ ] F[ ] K[ ] CancelServiceSoftwareUpdate
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateApplication
- W[x] S[ ] M[x] F[ ] K[ ] CreateDomain
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateIndex
- W[x] S[ ] M[ ] F[ ] K[ ] CreateOutboundConnection
- W[x] S[ ] M[ ] F[ ] K[ ] CreatePackage
- W[x] S[ ] M[ ] F[ ] K[ ] CreateVpcEndpoint
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteApplication
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteDataSource
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteDirectQueryDataSource
- W[x] S[ ] M[x] F[ ] K[ ] DeleteDomain
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteInboundConnection
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteIndex
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteOutboundConnection
- W[x] S[ ] M[ ] F[ ] K[ ] DeletePackage
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteVpcEndpoint
- W[ ] S[ ] M[ ] F[ ] K[ ] DeregisterCapability
- W[x] S[ ] M[x] F[ ] K[ ] DescribeDomain
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeDomainAutoTunes
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeDomainChangeProgress
- W[x] S[ ] M[x] F[ ] K[ ] DescribeDomainConfig
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeDomainHealth
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeDomainNodes
- W[x] S[ ] M[x] F[ ] K[ ] DescribeDomains
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeDryRunProgress
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeInboundConnections
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeInsightDetails
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeInstanceTypeLimits
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeOutboundConnections
- W[x] S[ ] M[ ] F[ ] K[ ] DescribePackages
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeReservedInstanceOfferings
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeReservedInstances
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeVpcEndpoints
- W[x] S[ ] M[ ] F[ ] K[ ] DissociatePackage
- W[ ] S[ ] M[ ] F[ ] K[ ] DissociatePackages
- W[ ] S[ ] M[ ] F[ ] K[ ] GetApplication
- W[ ] S[ ] M[ ] F[ ] K[ ] GetCapability
- W[x] S[ ] M[x] F[ ] K[ ] GetCompatibleVersions
- W[x] S[ ] M[ ] F[ ] K[ ] GetDataSource
- W[ ] S[ ] M[ ] F[ ] K[ ] GetDefaultApplicationSetting
- W[x] S[ ] M[ ] F[ ] K[ ] GetDirectQueryDataSource
- W[ ] S[ ] M[ ] F[ ] K[ ] GetDomainMaintenanceStatus
- W[ ] S[ ] M[ ] F[ ] K[ ] GetIndex
- W[ ] S[ ] M[ ] F[ ] K[ ] GetPackageVersionHistory
- W[ ] S[ ] M[ ] F[ ] K[ ] GetUpgradeHistory
- W[ ] S[ ] M[ ] F[ ] K[ ] GetUpgradeStatus
- W[ ] S[ ] M[ ] F[ ] K[ ] ListApplications
- W[x] S[ ] M[ ] F[ ] K[ ] ListDataSources
- W[x] S[ ] M[ ] F[ ] K[ ] ListDirectQueryDataSources
- W[ ] S[ ] M[ ] F[ ] K[ ] ListDomainMaintenances
- W[x] S[ ] M[x] F[ ] K[ ] ListDomainNames
- W[x] S[ ] M[ ] F[ ] K[ ] ListDomainsForPackage
- W[ ] S[ ] M[ ] F[ ] K[ ] ListInsights
- W[ ] S[ ] M[ ] F[ ] K[ ] ListInstanceTypeDetails
- W[x] S[ ] M[ ] F[ ] K[ ] ListPackagesForDomain
- W[ ] S[ ] M[ ] F[ ] K[ ] ListScheduledActions
- W[x] S[ ] M[x] F[ ] K[ ] ListTags
- W[ ] S[ ] M[ ] F[ ] K[ ] ListVersions
- W[ ] S[ ] M[ ] F[ ] K[ ] ListVpcEndpointAccess
- W[x] S[ ] M[ ] F[ ] K[ ] ListVpcEndpoints
- W[x] S[ ] M[ ] F[ ] K[ ] ListVpcEndpointsForDomain
- W[x] S[ ] M[ ] F[ ] K[ ] PurchaseReservedInstanceOffering
- W[ ] S[ ] M[ ] F[ ] K[ ] PutDefaultApplicationSetting
- W[ ] S[ ] M[ ] F[ ] K[ ] RegisterCapability
- W[x] S[ ] M[ ] F[ ] K[ ] RejectInboundConnection
- W[x] S[ ] M[x] F[ ] K[ ] RemoveTags
- W[ ] S[ ] M[ ] F[ ] K[ ] RevokeVpcEndpointAccess
- W[ ] S[ ] M[ ] F[ ] K[ ] RollbackServiceSoftwareUpdate
- W[ ] S[ ] M[ ] F[ ] K[ ] StartDomainMaintenance
- W[ ] S[ ] M[ ] F[ ] K[ ] StartServiceSoftwareUpdate
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateApplication
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateDataSource
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateDirectQueryDataSource
- W[x] S[ ] M[x] F[ ] K[ ] UpdateDomainConfig
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateIndex
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdatePackage
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdatePackageScope
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateScheduledAction
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateVpcEndpoint
- W[ ] S[ ] M[ ] F[ ] K[ ] UpgradeDomain

Integration tests: 35/44 implemented operations tested (79.5%)
Untested implemented operations: 9

### winterbaume-opensearchserverless (opensearchserverless) - W: 12/46, S: 0/46, M: 12/46, F: 0/46, K: 0/46

- W[x] S[ ] M[x] F[ ] K[ ] BatchGetCollection
- W[ ] S[ ] M[ ] F[ ] K[ ] BatchGetCollectionGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] BatchGetEffectiveLifecyclePolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] BatchGetLifecyclePolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] BatchGetVpcEndpoint
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateAccessPolicy
- W[x] S[ ] M[x] F[ ] K[ ] CreateCollection
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateCollectionGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateIndex
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateLifecyclePolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateSecurityConfig
- W[x] S[ ] M[x] F[ ] K[ ] CreateSecurityPolicy
- W[x] S[ ] M[x] F[ ] K[ ] CreateVpcEndpoint
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteAccessPolicy
- W[x] S[ ] M[x] F[ ] K[ ] DeleteCollection
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteCollectionGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteIndex
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteLifecyclePolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteSecurityConfig
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteSecurityPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteVpcEndpoint
- W[ ] S[ ] M[ ] F[ ] K[ ] GetAccessPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] GetAccountSettings
- W[ ] S[ ] M[ ] F[ ] K[ ] GetIndex
- W[ ] S[ ] M[ ] F[ ] K[ ] GetPoliciesStats
- W[ ] S[ ] M[ ] F[ ] K[ ] GetSecurityConfig
- W[x] S[ ] M[x] F[ ] K[ ] GetSecurityPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] ListAccessPolicies
- W[ ] S[ ] M[ ] F[ ] K[ ] ListCollectionGroups
- W[x] S[ ] M[x] F[ ] K[ ] ListCollections
- W[ ] S[ ] M[ ] F[ ] K[ ] ListLifecyclePolicies
- W[ ] S[ ] M[ ] F[ ] K[ ] ListSecurityConfigs
- W[x] S[ ] M[x] F[ ] K[ ] ListSecurityPolicies
- W[x] S[ ] M[x] F[ ] K[ ] ListTagsForResource
- W[ ] S[ ] M[ ] F[ ] K[ ] ListVpcEndpoints
- W[x] S[ ] M[x] F[ ] K[ ] TagResource
- W[x] S[ ] M[x] F[ ] K[ ] UntagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateAccessPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateAccountSettings
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateCollection
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateCollectionGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateIndex
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateLifecyclePolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateSecurityConfig
- W[x] S[ ] M[x] F[ ] K[ ] UpdateSecurityPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateVpcEndpoint

Integration tests: 12/12 implemented operations tested (100.0%)

### winterbaume-organizations (organizations) - W: 60/63, S: 3/63, M: 41/63, F: 0/63, K: 11/63

Terraform E2E: 4 tests across 4 terraform resource types

Resource types: aws_organizations_organization, aws_organizations_organizational_unit, aws_organizations_policy, aws_organizations_policy_attachment

- W[x] S[ ] M[ ] F[ ] K[ ] AcceptHandshake
- W[x] S[ ] M[x] F[ ] K[x] AttachPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] CancelHandshake
- W[x] S[ ] M[x] F[ ] K[ ] CloseAccount
- W[x] S[ ] M[x] F[ ] K[x] CreateAccount
- W[x] S[ ] M[ ] F[ ] K[ ] CreateGovCloudAccount
- W[x] S[ ] M[x] F[ ] K[x] CreateOrganization
- W[x] S[ ] M[x] F[ ] K[x] CreateOrganizationalUnit
- W[x] S[ ] M[x] F[ ] K[ ] CreatePolicy
- W[x] S[ ] M[ ] F[ ] K[ ] DeclineHandshake
- W[x] S[ ] M[x] F[ ] K[x] DeleteOrganization
- W[x] S[ ] M[x] F[ ] K[ ] DeleteOrganizationalUnit
- W[x] S[ ] M[x] F[ ] K[ ] DeletePolicy
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteResourcePolicy
- W[x] S[ ] M[x] F[ ] K[ ] DeregisterDelegatedAdministrator
- W[x] S[ ] M[x] F[ ] K[x] DescribeAccount
- W[x] S[ ] M[x] F[ ] K[ ] DescribeCreateAccountStatus
- W[ ] S[x] M[ ] F[ ] K[ ] DescribeEffectivePolicy
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeHandshake
- W[x] S[ ] M[x] F[ ] K[x] DescribeOrganization
- W[x] S[ ] M[x] F[ ] K[ ] DescribeOrganizationalUnit
- W[x] S[ ] M[x] F[ ] K[ ] DescribePolicy
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeResourcePolicy
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeResponsibilityTransfer
- W[x] S[ ] M[x] F[ ] K[x] DetachPolicy
- W[x] S[ ] M[x] F[ ] K[ ] DisableAWSServiceAccess
- W[x] S[ ] M[x] F[ ] K[ ] DisablePolicyType
- W[x] S[ ] M[x] F[ ] K[ ] EnableAWSServiceAccess
- W[x] S[ ] M[ ] F[ ] K[ ] EnableAllFeatures
- W[x] S[ ] M[x] F[ ] K[ ] EnablePolicyType
- W[x] S[ ] M[ ] F[ ] K[ ] InviteAccountToOrganization
- W[x] S[ ] M[ ] F[ ] K[ ] InviteOrganizationToTransferResponsibility
- W[x] S[ ] M[ ] F[ ] K[ ] LeaveOrganization
- W[x] S[ ] M[x] F[ ] K[ ] ListAWSServiceAccessForOrganization
- W[x] S[ ] M[x] F[ ] K[x] ListAccounts
- W[x] S[ ] M[x] F[ ] K[ ] ListAccountsForParent
- W[ ] S[x] M[ ] F[ ] K[ ] ListAccountsWithInvalidEffectivePolicy
- W[x] S[ ] M[x] F[ ] K[ ] ListChildren
- W[x] S[ ] M[x] F[ ] K[ ] ListCreateAccountStatus
- W[x] S[ ] M[x] F[ ] K[ ] ListDelegatedAdministrators
- W[x] S[ ] M[x] F[ ] K[ ] ListDelegatedServicesForAccount
- W[ ] S[x] M[ ] F[ ] K[ ] ListEffectivePolicyValidationErrors
- W[x] S[ ] M[ ] F[ ] K[ ] ListHandshakesForAccount
- W[x] S[ ] M[ ] F[ ] K[ ] ListHandshakesForOrganization
- W[x] S[ ] M[ ] F[ ] K[ ] ListInboundResponsibilityTransfers
- W[x] S[ ] M[x] F[ ] K[x] ListOrganizationalUnitsForParent
- W[x] S[ ] M[ ] F[ ] K[ ] ListOutboundResponsibilityTransfers
- W[x] S[ ] M[x] F[ ] K[ ] ListParents
- W[x] S[ ] M[x] F[ ] K[ ] ListPolicies
- W[x] S[ ] M[x] F[ ] K[ ] ListPoliciesForTarget
- W[x] S[ ] M[x] F[ ] K[x] ListRoots
- W[x] S[ ] M[x] F[ ] K[ ] ListTagsForResource
- W[x] S[ ] M[x] F[ ] K[ ] ListTargetsForPolicy
- W[x] S[ ] M[x] F[ ] K[ ] MoveAccount
- W[x] S[ ] M[ ] F[ ] K[ ] PutResourcePolicy
- W[x] S[ ] M[x] F[ ] K[ ] RegisterDelegatedAdministrator
- W[x] S[ ] M[x] F[ ] K[ ] RemoveAccountFromOrganization
- W[x] S[ ] M[x] F[ ] K[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] TerminateResponsibilityTransfer
- W[x] S[ ] M[x] F[ ] K[ ] UntagResource
- W[x] S[ ] M[x] F[ ] K[ ] UpdateOrganizationalUnit
- W[x] S[ ] M[x] F[ ] K[ ] UpdatePolicy
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateResponsibilityTransfer

Integration tests: 54/60 implemented operations tested (90.0%)
Untested implemented operations: 6

### winterbaume-osis (osis) - W: 10/22, S: 0/22, M: 13/22, F: 0/22, K: 0/22

- W[x] S[ ] M[x] F[ ] K[ ] CreatePipeline
- W[ ] S[ ] M[ ] F[ ] K[ ] CreatePipelineEndpoint
- W[x] S[ ] M[x] F[ ] K[ ] DeletePipeline
- W[ ] S[ ] M[ ] F[ ] K[ ] DeletePipelineEndpoint
- W[ ] S[ ] M[x] F[ ] K[ ] DeleteResourcePolicy
- W[x] S[ ] M[x] F[ ] K[ ] GetPipeline
- W[ ] S[ ] M[ ] F[ ] K[ ] GetPipelineBlueprint
- W[ ] S[ ] M[ ] F[ ] K[ ] GetPipelineChangeProgress
- W[ ] S[ ] M[x] F[ ] K[ ] GetResourcePolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] ListPipelineBlueprints
- W[ ] S[ ] M[ ] F[ ] K[ ] ListPipelineEndpointConnections
- W[ ] S[ ] M[ ] F[ ] K[ ] ListPipelineEndpoints
- W[x] S[ ] M[x] F[ ] K[ ] ListPipelines
- W[x] S[ ] M[x] F[ ] K[ ] ListTagsForResource
- W[ ] S[ ] M[x] F[ ] K[ ] PutResourcePolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] RevokePipelineEndpointConnections
- W[x] S[ ] M[x] F[ ] K[ ] StartPipeline
- W[x] S[ ] M[x] F[ ] K[ ] StopPipeline
- W[x] S[ ] M[x] F[ ] K[ ] TagResource
- W[x] S[ ] M[x] F[ ] K[ ] UntagResource
- W[x] S[ ] M[x] F[ ] K[ ] UpdatePipeline
- W[ ] S[ ] M[ ] F[ ] K[ ] ValidatePipeline

Integration tests: 10/10 implemented operations tested (100.0%)

### winterbaume-outposts (outposts) - W: 13/37, S: 0/37, M: 0/37, F: 0/37, K: 0/37

- W[ ] S[ ] M[ ] F[ ] K[ ] CancelCapacityTask
- W[ ] S[ ] M[ ] F[ ] K[ ] CancelOrder
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateOrder
- W[x] S[ ] M[ ] F[ ] K[ ] CreateOutpost
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateRenewal
- W[x] S[ ] M[ ] F[ ] K[ ] CreateSite
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteOutpost
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteSite
- W[ ] S[ ] M[ ] F[ ] K[ ] GetCapacityTask
- W[ ] S[ ] M[ ] F[ ] K[ ] GetCatalogItem
- W[ ] S[ ] M[ ] F[ ] K[ ] GetConnection
- W[ ] S[ ] M[ ] F[ ] K[ ] GetOrder
- W[x] S[ ] M[ ] F[ ] K[ ] GetOutpost
- W[ ] S[ ] M[ ] F[ ] K[ ] GetOutpostBillingInformation
- W[ ] S[ ] M[ ] F[ ] K[ ] GetOutpostInstanceTypes
- W[ ] S[ ] M[ ] F[ ] K[ ] GetOutpostSupportedInstanceTypes
- W[ ] S[ ] M[ ] F[ ] K[ ] GetRenewalPricing
- W[x] S[ ] M[ ] F[ ] K[ ] GetSite
- W[ ] S[ ] M[ ] F[ ] K[ ] GetSiteAddress
- W[ ] S[ ] M[ ] F[ ] K[ ] ListAssetInstances
- W[ ] S[ ] M[ ] F[ ] K[ ] ListAssets
- W[ ] S[ ] M[ ] F[ ] K[ ] ListBlockingInstancesForCapacityTask
- W[ ] S[ ] M[ ] F[ ] K[ ] ListCapacityTasks
- W[ ] S[ ] M[ ] F[ ] K[ ] ListCatalogItems
- W[ ] S[ ] M[ ] F[ ] K[ ] ListOrders
- W[x] S[ ] M[ ] F[ ] K[ ] ListOutposts
- W[x] S[ ] M[ ] F[ ] K[ ] ListSites
- W[x] S[ ] M[ ] F[ ] K[ ] ListTagsForResource
- W[ ] S[ ] M[ ] F[ ] K[ ] StartCapacityTask
- W[ ] S[ ] M[ ] F[ ] K[ ] StartConnection
- W[ ] S[ ] M[ ] F[ ] K[ ] StartOutpostDecommission
- W[x] S[ ] M[ ] F[ ] K[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateOutpost
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateSite
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateSiteAddress
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateSiteRackPhysicalProperties

Integration tests: 13/13 implemented operations tested (100.0%)

### winterbaume-panorama (panorama) - W: 10/34, S: 1/34, M: 11/34, F: 0/34, K: 0/34

- W[x] S[ ] M[x] F[ ] K[ ] CreateApplicationInstance
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateJobForDevices
- W[x] S[ ] M[x] F[ ] K[ ] CreateNodeFromTemplateJob
- W[ ] S[ ] M[ ] F[ ] K[ ] CreatePackage
- W[ ] S[ ] M[ ] F[ ] K[ ] CreatePackageImportJob
- W[x] S[ ] M[x] F[ ] K[ ] DeleteDevice
- W[ ] S[ ] M[ ] F[ ] K[ ] DeletePackage
- W[ ] S[ ] M[ ] F[ ] K[ ] DeregisterPackageVersion
- W[x] S[ ] M[x] F[ ] K[ ] DescribeApplicationInstance
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeApplicationInstanceDetails
- W[x] S[ ] M[x] F[ ] K[ ] DescribeDevice
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeDeviceJob
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeNode
- W[x] S[ ] M[x] F[ ] K[ ] DescribeNodeFromTemplateJob
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribePackage
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribePackageImportJob
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribePackageVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] ListApplicationInstanceDependencies
- W[ ] S[ ] M[ ] F[ ] K[ ] ListApplicationInstanceNodeInstances
- W[x] S[ ] M[x] F[ ] K[ ] ListApplicationInstances
- W[x] S[ ] M[x] F[ ] K[ ] ListDevices
- W[ ] S[ ] M[ ] F[ ] K[ ] ListDevicesJobs
- W[ ] S[ ] M[ ] F[ ] K[ ] ListNodeFromTemplateJobs
- W[ ] S[x] M[x] F[ ] K[ ] ListNodes
- W[ ] S[ ] M[ ] F[ ] K[ ] ListPackageImportJobs
- W[ ] S[ ] M[ ] F[ ] K[ ] ListPackages
- W[ ] S[ ] M[ ] F[ ] K[ ] ListTagsForResource
- W[x] S[ ] M[x] F[ ] K[ ] ProvisionDevice
- W[ ] S[ ] M[ ] F[ ] K[ ] RegisterPackageVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] RemoveApplicationInstance
- W[ ] S[ ] M[ ] F[ ] K[ ] SignalApplicationInstanceNodeInstances
- W[ ] S[ ] M[ ] F[ ] K[ ] TagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] UntagResource
- W[x] S[ ] M[x] F[ ] K[ ] UpdateDeviceMetadata

Integration tests: 10/10 implemented operations tested (100.0%)

### winterbaume-pcaconnectorscep (pca-connector-scep) - W: 11/12, S: 0/12, M: 0/12, F: 0/12, K: 0/12

- W[x] S[ ] M[ ] F[ ] K[ ] CreateChallenge
- W[x] S[ ] M[ ] F[ ] K[ ] CreateConnector
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteChallenge
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteConnector
- W[x] S[ ] M[ ] F[ ] K[ ] GetChallengeMetadata
- W[x] S[ ] M[ ] F[ ] K[ ] GetChallengePassword
- W[x] S[ ] M[ ] F[ ] K[ ] GetConnector
- W[x] S[ ] M[ ] F[ ] K[ ] ListChallengeMetadata
- W[x] S[ ] M[ ] F[ ] K[ ] ListConnectors
- W[ ] S[ ] M[ ] F[ ] K[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UntagResource

Integration tests: 9/11 implemented operations tested (81.8%)
Untested implemented operations: 2

### winterbaume-personalize (personalize) - W: 66/71, S: 5/71, M: 4/71, F: 0/71, K: 0/71

- W[x] S[ ] M[ ] F[ ] K[ ] CreateBatchInferenceJob
- W[x] S[ ] M[ ] F[ ] K[ ] CreateBatchSegmentJob
- W[x] S[ ] M[ ] F[ ] K[ ] CreateCampaign
- W[x] S[ ] M[ ] F[ ] K[ ] CreateDataDeletionJob
- W[x] S[ ] M[ ] F[ ] K[ ] CreateDataset
- W[x] S[ ] M[ ] F[ ] K[ ] CreateDatasetExportJob
- W[x] S[ ] M[ ] F[ ] K[ ] CreateDatasetGroup
- W[x] S[ ] M[ ] F[ ] K[ ] CreateDatasetImportJob
- W[x] S[ ] M[ ] F[ ] K[ ] CreateEventTracker
- W[x] S[ ] M[ ] F[ ] K[ ] CreateFilter
- W[x] S[ ] M[ ] F[ ] K[ ] CreateMetricAttribution
- W[x] S[ ] M[ ] F[ ] K[ ] CreateRecommender
- W[x] S[ ] M[x] F[ ] K[ ] CreateSchema
- W[x] S[ ] M[ ] F[ ] K[ ] CreateSolution
- W[x] S[ ] M[ ] F[ ] K[ ] CreateSolutionVersion
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteCampaign
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteDataset
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteDatasetGroup
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteEventTracker
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteFilter
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteMetricAttribution
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteRecommender
- W[x] S[ ] M[x] F[ ] K[ ] DeleteSchema
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteSolution
- W[ ] S[x] M[ ] F[ ] K[ ] DescribeAlgorithm
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeBatchInferenceJob
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeBatchSegmentJob
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeCampaign
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeDataDeletionJob
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeDataset
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeDatasetExportJob
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeDatasetGroup
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeDatasetImportJob
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeEventTracker
- W[ ] S[x] M[ ] F[ ] K[ ] DescribeFeatureTransformation
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeFilter
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeMetricAttribution
- W[ ] S[x] M[ ] F[ ] K[ ] DescribeRecipe
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeRecommender
- W[x] S[ ] M[x] F[ ] K[ ] DescribeSchema
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeSolution
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeSolutionVersion
- W[ ] S[x] M[ ] F[ ] K[ ] GetSolutionMetrics
- W[x] S[ ] M[ ] F[ ] K[ ] ListBatchInferenceJobs
- W[x] S[ ] M[ ] F[ ] K[ ] ListBatchSegmentJobs
- W[x] S[ ] M[ ] F[ ] K[ ] ListCampaigns
- W[x] S[ ] M[ ] F[ ] K[ ] ListDataDeletionJobs
- W[x] S[ ] M[ ] F[ ] K[ ] ListDatasetExportJobs
- W[x] S[ ] M[ ] F[ ] K[ ] ListDatasetGroups
- W[x] S[ ] M[ ] F[ ] K[ ] ListDatasetImportJobs
- W[x] S[ ] M[ ] F[ ] K[ ] ListDatasets
- W[x] S[ ] M[ ] F[ ] K[ ] ListEventTrackers
- W[x] S[ ] M[ ] F[ ] K[ ] ListFilters
- W[x] S[ ] M[ ] F[ ] K[ ] ListMetricAttributionMetrics
- W[x] S[ ] M[ ] F[ ] K[ ] ListMetricAttributions
- W[ ] S[x] M[ ] F[ ] K[ ] ListRecipes
- W[x] S[ ] M[ ] F[ ] K[ ] ListRecommenders
- W[x] S[ ] M[x] F[ ] K[ ] ListSchemas
- W[x] S[ ] M[ ] F[ ] K[ ] ListSolutionVersions
- W[x] S[ ] M[ ] F[ ] K[ ] ListSolutions
- W[x] S[ ] M[ ] F[ ] K[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] StartRecommender
- W[x] S[ ] M[ ] F[ ] K[ ] StopRecommender
- W[x] S[ ] M[ ] F[ ] K[ ] StopSolutionVersionCreation
- W[x] S[ ] M[ ] F[ ] K[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateCampaign
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateDataset
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateMetricAttribution
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateRecommender
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateSolution

Integration tests: 8/66 implemented operations tested (12.1%)
Untested implemented operations: 58

### winterbaume-personalizeevents (personalize-events) - W: 5/5, S: 0/5, M: 0/5, F: 0/5, K: 0/5

- W[x] S[ ] M[ ] F[ ] K[ ] PutActionInteractions
- W[x] S[ ] M[ ] F[ ] K[ ] PutActions
- W[x] S[ ] M[ ] F[ ] K[ ] PutEvents
- W[x] S[ ] M[ ] F[ ] K[ ] PutItems
- W[x] S[ ] M[ ] F[ ] K[ ] PutUsers

Integration tests: 3/5 implemented operations tested (60.0%)
Untested implemented operations: 2

### winterbaume-personalizeruntime (personalize-runtime) - W: 3/3, S: 0/3, M: 0/3, F: 0/3, K: 0/3

- W[x] S[ ] M[ ] F[ ] K[ ] GetActionRecommendations
- W[x] S[ ] M[ ] F[ ] K[ ] GetPersonalizedRanking
- W[x] S[ ] M[ ] F[ ] K[ ] GetRecommendations

Integration tests: 3/3 implemented operations tested (100.0%)

### winterbaume-pi (pi) - W: 9/13, S: 4/13, M: 0/13, F: 0/13, K: 0/13

- W[x] S[ ] M[ ] F[ ] K[ ] CreatePerformanceAnalysisReport
- W[x] S[ ] M[ ] F[ ] K[ ] DeletePerformanceAnalysisReport
- W[ ] S[x] M[ ] F[ ] K[ ] DescribeDimensionKeys
- W[ ] S[x] M[ ] F[ ] K[ ] GetDimensionKeyDetails
- W[x] S[ ] M[ ] F[ ] K[ ] GetPerformanceAnalysisReport
- W[ ] S[x] M[ ] F[ ] K[ ] GetResourceMetadata
- W[ ] S[x] M[ ] F[ ] K[ ] GetResourceMetrics
- W[x] S[ ] M[ ] F[ ] K[ ] ListAvailableResourceDimensions
- W[x] S[ ] M[ ] F[ ] K[ ] ListAvailableResourceMetrics
- W[x] S[ ] M[ ] F[ ] K[ ] ListPerformanceAnalysisReports
- W[x] S[ ] M[ ] F[ ] K[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UntagResource

Integration tests: 7/9 implemented operations tested (77.8%)
Untested implemented operations: 2

### winterbaume-pinpoint (pinpoint) - W: 15/122, S: 0/122, M: 12/122, F: 0/122, K: 0/122

- W[x] S[ ] M[x] F[ ] K[ ] CreateApp
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateCampaign
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateEmailTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateExportJob
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateImportJob
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateInAppTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateJourney
- W[ ] S[ ] M[ ] F[ ] K[ ] CreatePushTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateRecommenderConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateSegment
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateSmsTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateVoiceTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteAdmChannel
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteApnsChannel
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteApnsSandboxChannel
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteApnsVoipChannel
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteApnsVoipSandboxChannel
- W[x] S[ ] M[x] F[ ] K[ ] DeleteApp
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteBaiduChannel
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteCampaign
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteEmailChannel
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteEmailTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteEndpoint
- W[x] S[ ] M[x] F[ ] K[ ] DeleteEventStream
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteGcmChannel
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteInAppTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteJourney
- W[ ] S[ ] M[ ] F[ ] K[ ] DeletePushTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteRecommenderConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteSegment
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteSmsChannel
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteSmsTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteUserEndpoints
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteVoiceChannel
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteVoiceTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] GetAdmChannel
- W[ ] S[ ] M[ ] F[ ] K[ ] GetApnsChannel
- W[ ] S[ ] M[ ] F[ ] K[ ] GetApnsSandboxChannel
- W[ ] S[ ] M[ ] F[ ] K[ ] GetApnsVoipChannel
- W[ ] S[ ] M[ ] F[ ] K[ ] GetApnsVoipSandboxChannel
- W[x] S[ ] M[x] F[ ] K[ ] GetApp
- W[ ] S[ ] M[ ] F[ ] K[ ] GetApplicationDateRangeKpi
- W[x] S[ ] M[x] F[ ] K[ ] GetApplicationSettings
- W[x] S[ ] M[x] F[ ] K[ ] GetApps
- W[ ] S[ ] M[ ] F[ ] K[ ] GetBaiduChannel
- W[ ] S[ ] M[ ] F[ ] K[ ] GetCampaign
- W[ ] S[ ] M[ ] F[ ] K[ ] GetCampaignActivities
- W[ ] S[ ] M[ ] F[ ] K[ ] GetCampaignDateRangeKpi
- W[ ] S[ ] M[ ] F[ ] K[ ] GetCampaignVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] GetCampaignVersions
- W[ ] S[ ] M[ ] F[ ] K[ ] GetCampaigns
- W[ ] S[ ] M[ ] F[ ] K[ ] GetChannels
- W[x] S[ ] M[ ] F[ ] K[ ] GetEmailChannel
- W[ ] S[ ] M[ ] F[ ] K[ ] GetEmailTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] GetEndpoint
- W[x] S[ ] M[x] F[ ] K[ ] GetEventStream
- W[ ] S[ ] M[ ] F[ ] K[ ] GetExportJob
- W[ ] S[ ] M[ ] F[ ] K[ ] GetExportJobs
- W[ ] S[ ] M[ ] F[ ] K[ ] GetGcmChannel
- W[ ] S[ ] M[ ] F[ ] K[ ] GetImportJob
- W[ ] S[ ] M[ ] F[ ] K[ ] GetImportJobs
- W[ ] S[ ] M[ ] F[ ] K[ ] GetInAppMessages
- W[ ] S[ ] M[ ] F[ ] K[ ] GetInAppTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] GetJourney
- W[ ] S[ ] M[ ] F[ ] K[ ] GetJourneyDateRangeKpi
- W[ ] S[ ] M[ ] F[ ] K[ ] GetJourneyExecutionActivityMetrics
- W[ ] S[ ] M[ ] F[ ] K[ ] GetJourneyExecutionMetrics
- W[ ] S[ ] M[ ] F[ ] K[ ] GetJourneyRunExecutionActivityMetrics
- W[ ] S[ ] M[ ] F[ ] K[ ] GetJourneyRunExecutionMetrics
- W[ ] S[ ] M[ ] F[ ] K[ ] GetJourneyRuns
- W[ ] S[ ] M[ ] F[ ] K[ ] GetPushTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] GetRecommenderConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] GetRecommenderConfigurations
- W[ ] S[ ] M[ ] F[ ] K[ ] GetSegment
- W[ ] S[ ] M[ ] F[ ] K[ ] GetSegmentExportJobs
- W[ ] S[ ] M[ ] F[ ] K[ ] GetSegmentImportJobs
- W[ ] S[ ] M[ ] F[ ] K[ ] GetSegmentVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] GetSegmentVersions
- W[ ] S[ ] M[ ] F[ ] K[ ] GetSegments
- W[ ] S[ ] M[ ] F[ ] K[ ] GetSmsChannel
- W[ ] S[ ] M[ ] F[ ] K[ ] GetSmsTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] GetUserEndpoints
- W[ ] S[ ] M[ ] F[ ] K[ ] GetVoiceChannel
- W[ ] S[ ] M[ ] F[ ] K[ ] GetVoiceTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] ListJourneys
- W[x] S[ ] M[x] F[ ] K[ ] ListTagsForResource
- W[ ] S[ ] M[ ] F[ ] K[ ] ListTemplateVersions
- W[ ] S[ ] M[ ] F[ ] K[ ] ListTemplates
- W[ ] S[ ] M[ ] F[ ] K[ ] PhoneNumberValidate
- W[x] S[ ] M[x] F[ ] K[ ] PutEventStream
- W[ ] S[ ] M[ ] F[ ] K[ ] PutEvents
- W[ ] S[ ] M[ ] F[ ] K[ ] RemoveAttributes
- W[ ] S[ ] M[ ] F[ ] K[ ] SendMessages
- W[ ] S[ ] M[ ] F[ ] K[ ] SendOTPMessage
- W[ ] S[ ] M[ ] F[ ] K[ ] SendUsersMessages
- W[x] S[ ] M[x] F[ ] K[ ] TagResource
- W[x] S[ ] M[x] F[ ] K[ ] UntagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateAdmChannel
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateApnsChannel
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateApnsSandboxChannel
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateApnsVoipChannel
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateApnsVoipSandboxChannel
- W[x] S[ ] M[x] F[ ] K[ ] UpdateApplicationSettings
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateBaiduChannel
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateCampaign
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateEmailChannel
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateEmailTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateEndpoint
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateEndpointsBatch
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateGcmChannel
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateInAppTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateJourney
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateJourneyState
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdatePushTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateRecommenderConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateSegment
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateSmsChannel
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateSmsTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateTemplateActiveVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateVoiceChannel
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateVoiceTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] VerifyOTPMessage

Integration tests: 12/15 implemented operations tested (80.0%)
Untested implemented operations: 3

### winterbaume-pinpointsmsvoice (pinpoint-sms-voice) - W: 4/8, S: 0/8, M: 0/8, F: 0/8, K: 0/8

- W[x] S[ ] M[ ] F[ ] K[ ] CreateConfigurationSet
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateConfigurationSetEventDestination
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteConfigurationSet
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteConfigurationSetEventDestination
- W[ ] S[ ] M[ ] F[ ] K[ ] GetConfigurationSetEventDestinations
- W[x] S[ ] M[ ] F[ ] K[ ] ListConfigurationSets
- W[x] S[ ] M[ ] F[ ] K[ ] SendVoiceMessage
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateConfigurationSetEventDestination

Integration tests: 4/4 implemented operations tested (100.0%)

### winterbaume-pipes (pipes) - W: 10/10, S: 0/10, M: 9/10, F: 0/10, K: 10/10

- W[x] S[ ] M[x] F[ ] K[x] CreatePipe
- W[x] S[ ] M[x] F[ ] K[x] DeletePipe
- W[x] S[ ] M[x] F[ ] K[x] DescribePipe
- W[x] S[ ] M[x] F[ ] K[x] ListPipes
- W[x] S[ ] M[x] F[ ] K[x] ListTagsForResource
- W[x] S[ ] M[x] F[ ] K[x] StartPipe
- W[x] S[ ] M[x] F[ ] K[x] StopPipe
- W[x] S[ ] M[x] F[ ] K[x] TagResource
- W[x] S[ ] M[x] F[ ] K[x] UntagResource
- W[x] S[ ] M[ ] F[ ] K[x] UpdatePipe

Integration tests: 7/10 implemented operations tested (70.0%)
Untested implemented operations: 3

### winterbaume-polly (polly) - W: 9/10, S: 0/10, M: 5/10, F: 0/10, K: 0/10

- W[x] S[ ] M[x] F[ ] K[ ] DeleteLexicon
- W[x] S[ ] M[x] F[ ] K[ ] DescribeVoices
- W[x] S[ ] M[x] F[ ] K[ ] GetLexicon
- W[x] S[ ] M[ ] F[ ] K[ ] GetSpeechSynthesisTask
- W[x] S[ ] M[x] F[ ] K[ ] ListLexicons
- W[x] S[ ] M[ ] F[ ] K[ ] ListSpeechSynthesisTasks
- W[x] S[ ] M[x] F[ ] K[ ] PutLexicon
- W[ ] S[ ] M[ ] F[ ] K[ ] StartSpeechSynthesisStream
- W[x] S[ ] M[ ] F[ ] K[ ] StartSpeechSynthesisTask
- W[x] S[ ] M[ ] F[ ] K[ ] SynthesizeSpeech

Integration tests: 9/9 implemented operations tested (100.0%)

### winterbaume-pricing (pricing) - W: 5/5, S: 0/5, M: 0/5, F: 0/5, K: 0/5

- W[x] S[ ] M[ ] F[ ] K[ ] DescribeServices
- W[x] S[ ] M[ ] F[ ] K[ ] GetAttributeValues
- W[x] S[ ] M[ ] F[ ] K[ ] GetPriceListFileUrl
- W[x] S[ ] M[ ] F[ ] K[ ] GetProducts
- W[x] S[ ] M[ ] F[ ] K[ ] ListPriceLists

Integration tests: 5/5 implemented operations tested (100.0%)

### winterbaume-quicksight (quicksight) - W: 68/232, S: 0/232, M: 31/232, F: 0/232, K: 0/232

- W[ ] S[ ] M[ ] F[ ] K[ ] BatchCreateTopicReviewedAnswer
- W[ ] S[ ] M[ ] F[ ] K[ ] BatchDeleteTopicReviewedAnswer
- W[ ] S[ ] M[ ] F[ ] K[ ] CancelIngestion
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateAccountCustomization
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateAccountSubscription
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateActionConnector
- W[x] S[ ] M[ ] F[ ] K[ ] CreateAnalysis
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateBrand
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateCustomPermissions
- W[x] S[ ] M[x] F[ ] K[ ] CreateDashboard
- W[x] S[ ] M[x] F[ ] K[ ] CreateDataSet
- W[x] S[ ] M[x] F[ ] K[ ] CreateDataSource
- W[x] S[ ] M[ ] F[ ] K[ ] CreateFolder
- W[x] S[ ] M[ ] F[ ] K[ ] CreateFolderMembership
- W[x] S[ ] M[x] F[ ] K[ ] CreateGroup
- W[x] S[ ] M[x] F[ ] K[ ] CreateGroupMembership
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateIAMPolicyAssignment
- W[x] S[ ] M[x] F[ ] K[ ] CreateIngestion
- W[x] S[ ] M[ ] F[ ] K[ ] CreateNamespace
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateRefreshSchedule
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateRoleMembership
- W[x] S[ ] M[ ] F[ ] K[ ] CreateTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateTemplateAlias
- W[x] S[ ] M[ ] F[ ] K[ ] CreateTheme
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateThemeAlias
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateTopic
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateTopicRefreshSchedule
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateVPCConnection
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteAccountCustomPermission
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteAccountCustomization
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteAccountSubscription
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteActionConnector
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteAnalysis
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteBrand
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteBrandAssignment
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteCustomPermissions
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteDashboard
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteDataSet
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteDataSetRefreshProperties
- W[x] S[ ] M[x] F[ ] K[ ] DeleteDataSource
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteDefaultQBusinessApplication
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteFolder
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteFolderMembership
- W[x] S[ ] M[x] F[ ] K[ ] DeleteGroup
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteGroupMembership
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteIAMPolicyAssignment
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteIdentityPropagationConfig
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteNamespace
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteRefreshSchedule
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteRoleCustomPermission
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteRoleMembership
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteTemplateAlias
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteTheme
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteThemeAlias
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteTopic
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteTopicRefreshSchedule
- W[x] S[ ] M[x] F[ ] K[ ] DeleteUser
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteUserByPrincipalId
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteUserCustomPermission
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteVPCConnection
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeAccountCustomPermission
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeAccountCustomization
- W[x] S[ ] M[x] F[ ] K[ ] DescribeAccountSettings
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeAccountSubscription
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeActionConnector
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeActionConnectorPermissions
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeAnalysis
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeAnalysisDefinition
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeAnalysisPermissions
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeAssetBundleExportJob
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeAssetBundleImportJob
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeAutomationJob
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeBrand
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeBrandAssignment
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeBrandPublishedVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeCustomPermissions
- W[x] S[ ] M[x] F[ ] K[ ] DescribeDashboard
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeDashboardDefinition
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeDashboardPermissions
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeDashboardSnapshotJob
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeDashboardSnapshotJobResult
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeDashboardsQAConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeDataSet
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeDataSetPermissions
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeDataSetRefreshProperties
- W[x] S[ ] M[x] F[ ] K[ ] DescribeDataSource
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeDataSourcePermissions
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeDefaultQBusinessApplication
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeFolder
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeFolderPermissions
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeFolderResolvedPermissions
- W[x] S[ ] M[x] F[ ] K[ ] DescribeGroup
- W[x] S[ ] M[x] F[ ] K[ ] DescribeGroupMembership
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeIAMPolicyAssignment
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeIngestion
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeIpRestriction
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeKeyRegistration
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeNamespace
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeQPersonalizationConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeQuickSightQSearchConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeRefreshSchedule
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeRoleCustomPermission
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeSelfUpgradeConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeTemplateAlias
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeTemplateDefinition
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeTemplatePermissions
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeTheme
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeThemeAlias
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeThemePermissions
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeTopic
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeTopicPermissions
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeTopicRefresh
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeTopicRefreshSchedule
- W[x] S[ ] M[x] F[ ] K[ ] DescribeUser
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeVPCConnection
- W[ ] S[ ] M[ ] F[ ] K[ ] GenerateEmbedUrlForAnonymousUser
- W[ ] S[ ] M[ ] F[ ] K[ ] GenerateEmbedUrlForRegisteredUser
- W[ ] S[ ] M[ ] F[ ] K[ ] GenerateEmbedUrlForRegisteredUserWithIdentity
- W[ ] S[ ] M[ ] F[ ] K[ ] GetDashboardEmbedUrl
- W[ ] S[ ] M[ ] F[ ] K[ ] GetFlowMetadata
- W[ ] S[ ] M[ ] F[ ] K[ ] GetFlowPermissions
- W[ ] S[ ] M[ ] F[ ] K[ ] GetIdentityContext
- W[ ] S[ ] M[ ] F[ ] K[ ] GetSessionEmbedUrl
- W[ ] S[ ] M[ ] F[ ] K[ ] ListActionConnectors
- W[x] S[ ] M[ ] F[ ] K[ ] ListAnalyses
- W[ ] S[ ] M[ ] F[ ] K[ ] ListAssetBundleExportJobs
- W[ ] S[ ] M[ ] F[ ] K[ ] ListAssetBundleImportJobs
- W[ ] S[ ] M[ ] F[ ] K[ ] ListBrands
- W[ ] S[ ] M[ ] F[ ] K[ ] ListCustomPermissions
- W[ ] S[ ] M[ ] F[ ] K[ ] ListDashboardVersions
- W[x] S[ ] M[x] F[ ] K[ ] ListDashboards
- W[x] S[ ] M[ ] F[ ] K[ ] ListDataSets
- W[x] S[ ] M[x] F[ ] K[ ] ListDataSources
- W[ ] S[ ] M[ ] F[ ] K[ ] ListFlows
- W[x] S[ ] M[ ] F[ ] K[ ] ListFolderMembers
- W[x] S[ ] M[ ] F[ ] K[ ] ListFolders
- W[ ] S[ ] M[ ] F[ ] K[ ] ListFoldersForResource
- W[x] S[ ] M[x] F[ ] K[ ] ListGroupMemberships
- W[x] S[ ] M[x] F[ ] K[ ] ListGroups
- W[ ] S[ ] M[ ] F[ ] K[ ] ListIAMPolicyAssignments
- W[ ] S[ ] M[ ] F[ ] K[ ] ListIAMPolicyAssignmentsForUser
- W[ ] S[ ] M[ ] F[ ] K[ ] ListIdentityPropagationConfigs
- W[x] S[ ] M[ ] F[ ] K[ ] ListIngestions
- W[x] S[ ] M[ ] F[ ] K[ ] ListNamespaces
- W[ ] S[ ] M[ ] F[ ] K[ ] ListRefreshSchedules
- W[ ] S[ ] M[ ] F[ ] K[ ] ListRoleMemberships
- W[ ] S[ ] M[ ] F[ ] K[ ] ListSelfUpgrades
- W[x] S[ ] M[x] F[ ] K[ ] ListTagsForResource
- W[ ] S[ ] M[ ] F[ ] K[ ] ListTemplateAliases
- W[ ] S[ ] M[ ] F[ ] K[ ] ListTemplateVersions
- W[x] S[ ] M[ ] F[ ] K[ ] ListTemplates
- W[ ] S[ ] M[ ] F[ ] K[ ] ListThemeAliases
- W[ ] S[ ] M[ ] F[ ] K[ ] ListThemeVersions
- W[x] S[ ] M[ ] F[ ] K[ ] ListThemes
- W[ ] S[ ] M[ ] F[ ] K[ ] ListTopicRefreshSchedules
- W[ ] S[ ] M[ ] F[ ] K[ ] ListTopicReviewedAnswers
- W[ ] S[ ] M[ ] F[ ] K[ ] ListTopics
- W[x] S[ ] M[x] F[ ] K[ ] ListUserGroups
- W[x] S[ ] M[x] F[ ] K[ ] ListUsers
- W[ ] S[ ] M[ ] F[ ] K[ ] ListVPCConnections
- W[ ] S[ ] M[ ] F[ ] K[ ] PredictQAResults
- W[ ] S[ ] M[ ] F[ ] K[ ] PutDataSetRefreshProperties
- W[x] S[ ] M[x] F[ ] K[ ] RegisterUser
- W[ ] S[ ] M[ ] F[ ] K[ ] RestoreAnalysis
- W[ ] S[ ] M[ ] F[ ] K[ ] SearchActionConnectors
- W[ ] S[ ] M[ ] F[ ] K[ ] SearchAnalyses
- W[ ] S[ ] M[ ] F[ ] K[ ] SearchDashboards
- W[ ] S[ ] M[ ] F[ ] K[ ] SearchDataSets
- W[ ] S[ ] M[ ] F[ ] K[ ] SearchDataSources
- W[ ] S[ ] M[ ] F[ ] K[ ] SearchFlows
- W[ ] S[ ] M[ ] F[ ] K[ ] SearchFolders
- W[x] S[ ] M[x] F[ ] K[ ] SearchGroups
- W[ ] S[ ] M[ ] F[ ] K[ ] SearchTopics
- W[ ] S[ ] M[ ] F[ ] K[ ] StartAssetBundleExportJob
- W[ ] S[ ] M[ ] F[ ] K[ ] StartAssetBundleImportJob
- W[ ] S[ ] M[ ] F[ ] K[ ] StartAutomationJob
- W[ ] S[ ] M[ ] F[ ] K[ ] StartDashboardSnapshotJob
- W[ ] S[ ] M[ ] F[ ] K[ ] StartDashboardSnapshotJobSchedule
- W[x] S[ ] M[x] F[ ] K[ ] TagResource
- W[x] S[ ] M[x] F[ ] K[ ] UntagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateAccountCustomPermission
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateAccountCustomization
- W[x] S[ ] M[x] F[ ] K[ ] UpdateAccountSettings
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateActionConnector
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateActionConnectorPermissions
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateAnalysis
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateAnalysisPermissions
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateApplicationWithTokenExchangeGrant
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateBrand
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateBrandAssignment
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateBrandPublishedVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateCustomPermissions
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateDashboard
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateDashboardLinks
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateDashboardPermissions
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateDashboardPublishedVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateDashboardsQAConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateDataSet
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateDataSetPermissions
- W[x] S[ ] M[x] F[ ] K[ ] UpdateDataSource
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateDataSourcePermissions
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateDefaultQBusinessApplication
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateFlowPermissions
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateFolder
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateFolderPermissions
- W[x] S[ ] M[x] F[ ] K[ ] UpdateGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateIAMPolicyAssignment
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateIdentityPropagationConfig
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateIpRestriction
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateKeyRegistration
- W[x] S[ ] M[x] F[ ] K[ ] UpdatePublicSharingSettings
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateQPersonalizationConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateQuickSightQSearchConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateRefreshSchedule
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateRoleCustomPermission
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateSPICECapacityConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateSelfUpgrade
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateSelfUpgradeConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateTemplateAlias
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateTemplatePermissions
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateTheme
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateThemeAlias
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateThemePermissions
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateTopic
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateTopicPermissions
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateTopicRefreshSchedule
- W[x] S[ ] M[x] F[ ] K[ ] UpdateUser
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateUserCustomPermission
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateVPCConnection

Integration tests: 66/68 implemented operations tested (97.1%)
Untested implemented operations: 2

### winterbaume-ram (ram) - W: 35/35, S: 0/35, M: 8/35, F: 0/35, K: 0/35

- W[x] S[ ] M[ ] F[ ] K[ ] AcceptResourceShareInvitation
- W[x] S[ ] M[ ] F[ ] K[ ] AssociateResourceShare
- W[x] S[ ] M[ ] F[ ] K[ ] AssociateResourceSharePermission
- W[x] S[ ] M[ ] F[ ] K[ ] CreatePermission
- W[x] S[ ] M[ ] F[ ] K[ ] CreatePermissionVersion
- W[x] S[ ] M[x] F[ ] K[ ] CreateResourceShare
- W[x] S[ ] M[ ] F[ ] K[ ] DeletePermission
- W[x] S[ ] M[ ] F[ ] K[ ] DeletePermissionVersion
- W[x] S[ ] M[x] F[ ] K[ ] DeleteResourceShare
- W[x] S[ ] M[ ] F[ ] K[ ] DisassociateResourceShare
- W[x] S[ ] M[ ] F[ ] K[ ] DisassociateResourceSharePermission
- W[x] S[ ] M[x] F[ ] K[ ] EnableSharingWithAwsOrganization
- W[x] S[ ] M[ ] F[ ] K[ ] GetPermission
- W[x] S[ ] M[ ] F[ ] K[ ] GetResourcePolicies
- W[x] S[ ] M[x] F[ ] K[ ] GetResourceShareAssociations
- W[x] S[ ] M[ ] F[ ] K[ ] GetResourceShareInvitations
- W[x] S[ ] M[x] F[ ] K[ ] GetResourceShares
- W[x] S[ ] M[ ] F[ ] K[ ] ListPendingInvitationResources
- W[x] S[ ] M[ ] F[ ] K[ ] ListPermissionAssociations
- W[x] S[ ] M[ ] F[ ] K[ ] ListPermissionVersions
- W[x] S[ ] M[x] F[ ] K[ ] ListPermissions
- W[x] S[ ] M[ ] F[ ] K[ ] ListPrincipals
- W[x] S[ ] M[ ] F[ ] K[ ] ListReplacePermissionAssociationsWork
- W[x] S[ ] M[ ] F[ ] K[ ] ListResourceSharePermissions
- W[x] S[ ] M[x] F[ ] K[ ] ListResourceTypes
- W[x] S[ ] M[ ] F[ ] K[ ] ListResources
- W[x] S[ ] M[ ] F[ ] K[ ] ListSourceAssociations
- W[x] S[ ] M[ ] F[ ] K[ ] PromotePermissionCreatedFromPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] PromoteResourceShareCreatedFromPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] RejectResourceShareInvitation
- W[x] S[ ] M[ ] F[ ] K[ ] ReplacePermissionAssociations
- W[x] S[ ] M[ ] F[ ] K[ ] SetDefaultPermissionVersion
- W[x] S[ ] M[ ] F[ ] K[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UntagResource
- W[x] S[ ] M[x] F[ ] K[ ] UpdateResourceShare

Integration tests: 32/35 implemented operations tested (91.4%)
Untested implemented operations: 3

### winterbaume-rbin (rbin) - W: 9/10, S: 0/10, M: 0/10, F: 0/10, K: 0/10

- W[x] S[ ] M[ ] F[ ] K[ ] CreateRule
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteRule
- W[x] S[ ] M[ ] F[ ] K[ ] GetRule
- W[x] S[ ] M[ ] F[ ] K[ ] ListRules
- W[ ] S[ ] M[ ] F[ ] K[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] LockRule
- W[x] S[ ] M[ ] F[ ] K[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UnlockRule
- W[x] S[ ] M[ ] F[ ] K[ ] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateRule

Integration tests: 9/9 implemented operations tested (100.0%)

### winterbaume-rds (rds) - W: 146/164, S: 4/164, M: 85/164, F: 0/164, K: 12/164

Terraform E2E: 6 tests across 2 terraform resource types

Resource types: aws_db_parameter_group, aws_db_subnet_group

- W[x] S[ ] M[x] F[ ] K[ ] AddRoleToDBCluster
- W[x] S[ ] M[x] F[ ] K[ ] AddRoleToDBInstance
- W[x] S[ ] M[ ] F[ ] K[ ] AddSourceIdentifierToSubscription
- W[x] S[ ] M[x] F[ ] K[ ] AddTagsToResource
- W[x] S[ ] M[ ] F[ ] K[ ] ApplyPendingMaintenanceAction
- W[x] S[ ] M[ ] F[ ] K[ ] AuthorizeDBSecurityGroupIngress
- W[x] S[ ] M[ ] F[ ] K[ ] BacktrackDBCluster
- W[x] S[ ] M[x] F[ ] K[ ] CancelExportTask
- W[x] S[ ] M[x] F[ ] K[ ] CopyDBClusterParameterGroup
- W[x] S[ ] M[x] F[ ] K[ ] CopyDBClusterSnapshot
- W[x] S[ ] M[x] F[ ] K[ ] CopyDBParameterGroup
- W[x] S[ ] M[x] F[ ] K[ ] CopyDBSnapshot
- W[x] S[ ] M[ ] F[ ] K[ ] CopyOptionGroup
- W[x] S[ ] M[x] F[ ] K[ ] CreateBlueGreenDeployment
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateCustomDBEngineVersion
- W[x] S[ ] M[x] F[ ] K[x] CreateDBCluster
- W[x] S[ ] M[ ] F[ ] K[ ] CreateDBClusterEndpoint
- W[x] S[ ] M[x] F[ ] K[ ] CreateDBClusterParameterGroup
- W[x] S[ ] M[x] F[ ] K[ ] CreateDBClusterSnapshot
- W[x] S[ ] M[x] F[ ] K[x] CreateDBInstance
- W[x] S[ ] M[x] F[ ] K[ ] CreateDBInstanceReadReplica
- W[x] S[ ] M[x] F[ ] K[ ] CreateDBParameterGroup
- W[x] S[ ] M[x] F[ ] K[ ] CreateDBProxy
- W[x] S[ ] M[ ] F[ ] K[ ] CreateDBProxyEndpoint
- W[x] S[ ] M[x] F[ ] K[ ] CreateDBSecurityGroup
- W[x] S[ ] M[x] F[ ] K[ ] CreateDBShardGroup
- W[x] S[ ] M[x] F[ ] K[x] CreateDBSnapshot
- W[x] S[ ] M[ ] F[ ] K[ ] CreateDBSubnetGroup
- W[x] S[ ] M[x] F[ ] K[ ] CreateEventSubscription
- W[x] S[ ] M[x] F[ ] K[ ] CreateGlobalCluster
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateIntegration
- W[x] S[ ] M[x] F[ ] K[ ] CreateOptionGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateTenantDatabase
- W[x] S[ ] M[x] F[ ] K[ ] DeleteBlueGreenDeployment
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteCustomDBEngineVersion
- W[x] S[ ] M[x] F[ ] K[x] DeleteDBCluster
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteDBClusterAutomatedBackup
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteDBClusterEndpoint
- W[x] S[ ] M[x] F[ ] K[ ] DeleteDBClusterParameterGroup
- W[x] S[ ] M[x] F[ ] K[ ] DeleteDBClusterSnapshot
- W[x] S[ ] M[x] F[ ] K[x] DeleteDBInstance
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteDBInstanceAutomatedBackup
- W[x] S[ ] M[x] F[ ] K[ ] DeleteDBParameterGroup
- W[x] S[ ] M[x] F[ ] K[ ] DeleteDBProxy
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteDBProxyEndpoint
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteDBSecurityGroup
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteDBShardGroup
- W[x] S[ ] M[x] F[ ] K[x] DeleteDBSnapshot
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteDBSubnetGroup
- W[x] S[ ] M[x] F[ ] K[ ] DeleteEventSubscription
- W[x] S[ ] M[x] F[ ] K[ ] DeleteGlobalCluster
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteIntegration
- W[x] S[ ] M[x] F[ ] K[ ] DeleteOptionGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteTenantDatabase
- W[x] S[ ] M[x] F[ ] K[ ] DeregisterDBProxyTargets
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeAccountAttributes
- W[x] S[ ] M[x] F[ ] K[ ] DescribeBlueGreenDeployments
- W[ ] S[x] M[ ] F[ ] K[ ] DescribeCertificates
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeDBClusterAutomatedBackups
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeDBClusterBacktracks
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeDBClusterEndpoints
- W[x] S[ ] M[x] F[ ] K[ ] DescribeDBClusterParameterGroups
- W[x] S[ ] M[x] F[ ] K[ ] DescribeDBClusterParameters
- W[x] S[ ] M[x] F[ ] K[ ] DescribeDBClusterSnapshotAttributes
- W[x] S[ ] M[x] F[ ] K[ ] DescribeDBClusterSnapshots
- W[x] S[ ] M[x] F[ ] K[x] DescribeDBClusters
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeDBEngineVersions
- W[x] S[ ] M[x] F[ ] K[ ] DescribeDBInstanceAutomatedBackups
- W[x] S[ ] M[x] F[ ] K[x] DescribeDBInstances
- W[x] S[ ] M[x] F[ ] K[ ] DescribeDBLogFiles
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeDBMajorEngineVersions
- W[x] S[ ] M[x] F[ ] K[ ] DescribeDBParameterGroups
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeDBParameters
- W[x] S[ ] M[x] F[ ] K[ ] DescribeDBProxies
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeDBProxyEndpoints
- W[x] S[ ] M[x] F[ ] K[ ] DescribeDBProxyTargetGroups
- W[x] S[ ] M[x] F[ ] K[ ] DescribeDBProxyTargets
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeDBRecommendations
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeDBSecurityGroups
- W[x] S[ ] M[x] F[ ] K[ ] DescribeDBShardGroups
- W[x] S[ ] M[x] F[ ] K[ ] DescribeDBSnapshotAttributes
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeDBSnapshotTenantDatabases
- W[x] S[ ] M[x] F[ ] K[ ] DescribeDBSnapshots
- W[x] S[ ] M[x] F[ ] K[ ] DescribeDBSubnetGroups
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeEngineDefaultClusterParameters
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeEngineDefaultParameters
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeEventCategories
- W[x] S[ ] M[x] F[ ] K[ ] DescribeEventSubscriptions
- W[ ] S[x] M[x] F[ ] K[ ] DescribeEvents
- W[x] S[ ] M[x] F[ ] K[ ] DescribeExportTasks
- W[x] S[ ] M[x] F[ ] K[ ] DescribeGlobalClusters
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeIntegrations
- W[ ] S[x] M[x] F[ ] K[ ] DescribeOptionGroupOptions
- W[x] S[ ] M[x] F[ ] K[ ] DescribeOptionGroups
- W[x] S[ ] M[x] F[ ] K[ ] DescribeOrderableDBInstanceOptions
- W[ ] S[x] M[ ] F[ ] K[ ] DescribePendingMaintenanceActions
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeReservedDBInstances
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeReservedDBInstancesOfferings
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeServerlessV2PlatformVersions
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeSourceRegions
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeTenantDatabases
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeValidDBInstanceModifications
- W[x] S[ ] M[ ] F[ ] K[ ] DisableHttpEndpoint
- W[x] S[ ] M[ ] F[ ] K[ ] DownloadDBLogFilePortion
- W[x] S[ ] M[ ] F[ ] K[ ] EnableHttpEndpoint
- W[x] S[ ] M[x] F[ ] K[ ] FailoverDBCluster
- W[x] S[ ] M[ ] F[ ] K[ ] FailoverGlobalCluster
- W[x] S[ ] M[x] F[ ] K[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyActivityStream
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyCertificates
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyCurrentDBClusterCapacity
- W[ ] S[ ] M[ ] F[ ] K[ ] ModifyCustomDBEngineVersion
- W[x] S[ ] M[x] F[ ] K[x] ModifyDBCluster
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyDBClusterEndpoint
- W[x] S[ ] M[x] F[ ] K[ ] ModifyDBClusterParameterGroup
- W[x] S[ ] M[x] F[ ] K[ ] ModifyDBClusterSnapshotAttribute
- W[x] S[ ] M[x] F[ ] K[x] ModifyDBInstance
- W[x] S[ ] M[x] F[ ] K[ ] ModifyDBParameterGroup
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyDBProxy
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyDBProxyEndpoint
- W[x] S[ ] M[x] F[ ] K[ ] ModifyDBProxyTargetGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] ModifyDBRecommendation
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyDBShardGroup
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyDBSnapshot
- W[x] S[ ] M[x] F[ ] K[ ] ModifyDBSnapshotAttribute
- W[x] S[ ] M[x] F[ ] K[ ] ModifyDBSubnetGroup
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyEventSubscription
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyGlobalCluster
- W[ ] S[ ] M[ ] F[ ] K[ ] ModifyIntegration
- W[x] S[ ] M[x] F[ ] K[ ] ModifyOptionGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] ModifyTenantDatabase
- W[x] S[ ] M[x] F[ ] K[ ] PromoteReadReplica
- W[x] S[ ] M[x] F[ ] K[ ] PromoteReadReplicaDBCluster
- W[ ] S[ ] M[ ] F[ ] K[ ] PurchaseReservedDBInstancesOffering
- W[x] S[ ] M[ ] F[ ] K[ ] RebootDBCluster
- W[x] S[ ] M[x] F[ ] K[ ] RebootDBInstance
- W[x] S[ ] M[ ] F[ ] K[ ] RebootDBShardGroup
- W[x] S[ ] M[x] F[ ] K[ ] RegisterDBProxyTargets
- W[x] S[ ] M[x] F[ ] K[ ] RemoveFromGlobalCluster
- W[x] S[ ] M[ ] F[ ] K[ ] RemoveRoleFromDBCluster
- W[x] S[ ] M[ ] F[ ] K[ ] RemoveRoleFromDBInstance
- W[x] S[ ] M[ ] F[ ] K[ ] RemoveSourceIdentifierFromSubscription
- W[x] S[ ] M[x] F[ ] K[ ] RemoveTagsFromResource
- W[x] S[ ] M[ ] F[ ] K[ ] ResetDBClusterParameterGroup
- W[x] S[ ] M[ ] F[ ] K[ ] ResetDBParameterGroup
- W[x] S[ ] M[ ] F[ ] K[ ] RestoreDBClusterFromS3
- W[x] S[ ] M[x] F[ ] K[ ] RestoreDBClusterFromSnapshot
- W[x] S[ ] M[x] F[ ] K[ ] RestoreDBClusterToPointInTime
- W[x] S[ ] M[x] F[ ] K[ ] RestoreDBInstanceFromDBSnapshot
- W[x] S[ ] M[ ] F[ ] K[ ] RestoreDBInstanceFromS3
- W[x] S[ ] M[x] F[ ] K[ ] RestoreDBInstanceToPointInTime
- W[x] S[ ] M[ ] F[ ] K[ ] RevokeDBSecurityGroupIngress
- W[x] S[ ] M[ ] F[ ] K[ ] StartActivityStream
- W[x] S[ ] M[x] F[ ] K[ ] StartDBCluster
- W[x] S[ ] M[x] F[ ] K[x] StartDBInstance
- W[x] S[ ] M[ ] F[ ] K[ ] StartDBInstanceAutomatedBackupsReplication
- W[x] S[ ] M[x] F[ ] K[ ] StartExportTask
- W[x] S[ ] M[ ] F[ ] K[ ] StopActivityStream
- W[x] S[ ] M[x] F[ ] K[ ] StopDBCluster
- W[x] S[ ] M[x] F[ ] K[x] StopDBInstance
- W[x] S[ ] M[ ] F[ ] K[ ] StopDBInstanceAutomatedBackupsReplication
- W[x] S[ ] M[x] F[ ] K[ ] SwitchoverBlueGreenDeployment
- W[x] S[ ] M[ ] F[ ] K[ ] SwitchoverGlobalCluster
- W[x] S[ ] M[ ] F[ ] K[ ] SwitchoverReadReplica

Integration tests: 78/146 implemented operations tested (53.4%)
Untested implemented operations: 68

### winterbaume-rdsdata (rds-data) - W: 6/6, S: 0/6, M: 1/6, F: 0/6, K: 0/6

- W[x] S[ ] M[ ] F[ ] K[ ] BatchExecuteStatement
- W[x] S[ ] M[ ] F[ ] K[ ] BeginTransaction
- W[x] S[ ] M[ ] F[ ] K[ ] CommitTransaction
- W[x] S[ ] M[ ] F[ ] K[ ] ExecuteSql
- W[x] S[ ] M[x] F[ ] K[ ] ExecuteStatement
- W[x] S[ ] M[ ] F[ ] K[ ] RollbackTransaction

Integration tests: 6/6 implemented operations tested (100.0%)

### winterbaume-redshift (redshift) - W: 100/141, S: 3/141, M: 35/141, F: 0/141, K: 7/141

- W[ ] S[ ] M[ ] F[ ] K[ ] AcceptReservedNodeExchange
- W[x] S[ ] M[ ] F[ ] K[ ] AddPartner
- W[ ] S[ ] M[ ] F[ ] K[ ] AssociateDataShareConsumer
- W[x] S[ ] M[x] F[ ] K[ ] AuthorizeClusterSecurityGroupIngress
- W[ ] S[ ] M[ ] F[ ] K[ ] AuthorizeDataShare
- W[ ] S[ ] M[ ] F[ ] K[ ] AuthorizeEndpointAccess
- W[x] S[ ] M[ ] F[ ] K[ ] AuthorizeSnapshotAccess
- W[x] S[ ] M[ ] F[ ] K[ ] BatchDeleteClusterSnapshots
- W[x] S[ ] M[ ] F[ ] K[ ] BatchModifyClusterSnapshots
- W[x] S[ ] M[ ] F[ ] K[ ] CancelResize
- W[x] S[ ] M[ ] F[ ] K[ ] CopyClusterSnapshot
- W[x] S[ ] M[ ] F[ ] K[ ] CreateAuthenticationProfile
- W[x] S[ ] M[x] F[ ] K[x] CreateCluster
- W[x] S[ ] M[x] F[ ] K[ ] CreateClusterParameterGroup
- W[x] S[ ] M[x] F[ ] K[ ] CreateClusterSecurityGroup
- W[x] S[ ] M[x] F[ ] K[x] CreateClusterSnapshot
- W[x] S[ ] M[x] F[ ] K[ ] CreateClusterSubnetGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateCustomDomainAssociation
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateEndpointAccess
- W[x] S[ ] M[ ] F[ ] K[ ] CreateEventSubscription
- W[x] S[ ] M[ ] F[ ] K[ ] CreateHsmClientCertificate
- W[x] S[ ] M[ ] F[ ] K[ ] CreateHsmConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateIntegration
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateRedshiftIdcApplication
- W[x] S[ ] M[ ] F[ ] K[ ] CreateScheduledAction
- W[x] S[ ] M[x] F[ ] K[ ] CreateSnapshotCopyGrant
- W[x] S[ ] M[ ] F[ ] K[ ] CreateSnapshotSchedule
- W[x] S[ ] M[x] F[ ] K[ ] CreateTags
- W[x] S[ ] M[ ] F[ ] K[ ] CreateUsageLimit
- W[ ] S[ ] M[ ] F[ ] K[ ] DeauthorizeDataShare
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteAuthenticationProfile
- W[x] S[ ] M[x] F[ ] K[x] DeleteCluster
- W[x] S[ ] M[x] F[ ] K[ ] DeleteClusterParameterGroup
- W[x] S[ ] M[x] F[ ] K[ ] DeleteClusterSecurityGroup
- W[x] S[ ] M[x] F[ ] K[x] DeleteClusterSnapshot
- W[x] S[ ] M[x] F[ ] K[ ] DeleteClusterSubnetGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteCustomDomainAssociation
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteEndpointAccess
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteEventSubscription
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteHsmClientCertificate
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteHsmConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteIntegration
- W[x] S[ ] M[ ] F[ ] K[ ] DeletePartner
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteRedshiftIdcApplication
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteResourcePolicy
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteScheduledAction
- W[x] S[ ] M[x] F[ ] K[ ] DeleteSnapshotCopyGrant
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteSnapshotSchedule
- W[x] S[ ] M[x] F[ ] K[ ] DeleteTags
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteUsageLimit
- W[ ] S[ ] M[ ] F[ ] K[ ] DeregisterNamespace
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeAccountAttributes
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeAuthenticationProfiles
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeClusterDbRevisions
- W[x] S[ ] M[x] F[ ] K[ ] DescribeClusterParameterGroups
- W[x] S[ ] M[x] F[ ] K[ ] DescribeClusterParameters
- W[x] S[ ] M[x] F[ ] K[ ] DescribeClusterSecurityGroups
- W[x] S[ ] M[x] F[ ] K[x] DescribeClusterSnapshots
- W[x] S[ ] M[x] F[ ] K[ ] DescribeClusterSubnetGroups
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeClusterTracks
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeClusterVersions
- W[x] S[ ] M[x] F[ ] K[x] DescribeClusters
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeCustomDomainAssociations
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeDataShares
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeDataSharesForConsumer
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeDataSharesForProducer
- W[x] S[ ] M[x] F[ ] K[ ] DescribeDefaultClusterParameters
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeEndpointAccess
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeEndpointAuthorization
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeEventCategories
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeEventSubscriptions
- W[ ] S[x] M[ ] F[ ] K[ ] DescribeEvents
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeHsmClientCertificates
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeHsmConfigurations
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeInboundIntegrations
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeIntegrations
- W[x] S[ ] M[x] F[ ] K[ ] DescribeLoggingStatus
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeNodeConfigurationOptions
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeOrderableClusterOptions
- W[x] S[ ] M[ ] F[ ] K[ ] DescribePartners
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeRedshiftIdcApplications
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeReservedNodeExchangeStatus
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeReservedNodeOfferings
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeReservedNodes
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeResize
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeScheduledActions
- W[x] S[ ] M[x] F[ ] K[ ] DescribeSnapshotCopyGrants
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeSnapshotSchedules
- W[ ] S[x] M[ ] F[ ] K[ ] DescribeStorage
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeTableRestoreStatus
- W[x] S[ ] M[x] F[ ] K[ ] DescribeTags
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeUsageLimits
- W[x] S[ ] M[x] F[ ] K[ ] DisableLogging
- W[x] S[ ] M[x] F[ ] K[ ] DisableSnapshotCopy
- W[ ] S[ ] M[ ] F[ ] K[ ] DisassociateDataShareConsumer
- W[x] S[ ] M[x] F[ ] K[ ] EnableLogging
- W[x] S[ ] M[x] F[ ] K[ ] EnableSnapshotCopy
- W[x] S[ ] M[ ] F[ ] K[ ] FailoverPrimaryCompute
- W[x] S[ ] M[x] F[ ] K[ ] GetClusterCredentials
- W[x] S[ ] M[ ] F[ ] K[ ] GetClusterCredentialsWithIAM
- W[ ] S[ ] M[ ] F[ ] K[ ] GetIdentityCenterAuthToken
- W[ ] S[ ] M[ ] F[ ] K[ ] GetReservedNodeExchangeConfigurationOptions
- W[ ] S[ ] M[ ] F[ ] K[ ] GetReservedNodeExchangeOfferings
- W[x] S[ ] M[ ] F[ ] K[ ] GetResourcePolicy
- W[ ] S[x] M[ ] F[ ] K[ ] ListRecommendations
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyAquaConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyAuthenticationProfile
- W[x] S[ ] M[x] F[ ] K[x] ModifyCluster
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyClusterDbRevision
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyClusterIamRoles
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyClusterMaintenance
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyClusterParameterGroup
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyClusterSnapshot
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyClusterSnapshotSchedule
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyClusterSubnetGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] ModifyCustomDomainAssociation
- W[ ] S[ ] M[ ] F[ ] K[ ] ModifyEndpointAccess
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyEventSubscription
- W[ ] S[ ] M[ ] F[ ] K[ ] ModifyIntegration
- W[ ] S[ ] M[ ] F[ ] K[ ] ModifyLakehouseConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] ModifyRedshiftIdcApplication
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyScheduledAction
- W[x] S[ ] M[x] F[ ] K[ ] ModifySnapshotCopyRetentionPeriod
- W[x] S[ ] M[ ] F[ ] K[ ] ModifySnapshotSchedule
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyUsageLimit
- W[x] S[ ] M[x] F[ ] K[ ] PauseCluster
- W[ ] S[ ] M[ ] F[ ] K[ ] PurchaseReservedNodeOffering
- W[x] S[ ] M[ ] F[ ] K[ ] PutResourcePolicy
- W[x] S[ ] M[ ] F[ ] K[ ] RebootCluster
- W[ ] S[ ] M[ ] F[ ] K[ ] RegisterNamespace
- W[ ] S[ ] M[ ] F[ ] K[ ] RejectDataShare
- W[x] S[ ] M[ ] F[ ] K[ ] ResetClusterParameterGroup
- W[x] S[ ] M[ ] F[ ] K[ ] ResizeCluster
- W[x] S[ ] M[x] F[ ] K[ ] RestoreFromClusterSnapshot
- W[ ] S[ ] M[ ] F[ ] K[ ] RestoreTableFromClusterSnapshot
- W[x] S[ ] M[x] F[ ] K[ ] ResumeCluster
- W[x] S[ ] M[ ] F[ ] K[ ] RevokeClusterSecurityGroupIngress
- W[ ] S[ ] M[ ] F[ ] K[ ] RevokeEndpointAccess
- W[x] S[ ] M[ ] F[ ] K[ ] RevokeSnapshotAccess
- W[x] S[ ] M[ ] F[ ] K[ ] RotateEncryptionKey
- W[x] S[ ] M[ ] F[ ] K[ ] UpdatePartnerStatus

Integration tests: 99/100 implemented operations tested (99.0%)
Untested implemented operations: 1

### winterbaume-redshiftdata (redshift-data) - W: 11/11, S: 0/11, M: 4/11, F: 0/11, K: 0/11

- W[x] S[ ] M[ ] F[ ] K[ ] BatchExecuteStatement
- W[x] S[ ] M[x] F[ ] K[ ] CancelStatement
- W[x] S[ ] M[x] F[ ] K[ ] DescribeStatement
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeTable
- W[x] S[ ] M[x] F[ ] K[ ] ExecuteStatement
- W[x] S[ ] M[x] F[ ] K[ ] GetStatementResult
- W[x] S[ ] M[ ] F[ ] K[ ] GetStatementResultV2
- W[x] S[ ] M[ ] F[ ] K[ ] ListDatabases
- W[x] S[ ] M[ ] F[ ] K[ ] ListSchemas
- W[x] S[ ] M[ ] F[ ] K[ ] ListStatements
- W[x] S[ ] M[ ] F[ ] K[ ] ListTables

Integration tests: 11/11 implemented operations tested (100.0%)

### winterbaume-rekognition (rekognition) - W: 8/75, S: 4/75, M: 8/75, F: 0/75, K: 13/75

- W[ ] S[ ] M[ ] F[ ] K[ ] AssociateFaces
- W[ ] S[x] M[x] F[ ] K[ ] CompareFaces
- W[ ] S[ ] M[ ] F[ ] K[ ] CopyProjectVersion
- W[x] S[ ] M[ ] F[ ] K[x] CreateCollection
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateDataset
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateFaceLivenessSession
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateProject
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateProjectVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateStreamProcessor
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateUser
- W[x] S[ ] M[ ] F[ ] K[x] DeleteCollection
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteDataset
- W[ ] S[ ] M[ ] F[ ] K[x] DeleteFaces
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteProject
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteProjectPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteProjectVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteStreamProcessor
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteUser
- W[x] S[ ] M[ ] F[ ] K[x] DescribeCollection
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeDataset
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeProjectVersions
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeProjects
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeStreamProcessor
- W[ ] S[x] M[x] F[ ] K[ ] DetectCustomLabels
- W[ ] S[ ] M[ ] F[ ] K[x] DetectFaces
- W[ ] S[x] M[x] F[ ] K[x] DetectLabels
- W[ ] S[ ] M[ ] F[ ] K[x] DetectModerationLabels
- W[ ] S[ ] M[ ] F[ ] K[ ] DetectProtectiveEquipment
- W[ ] S[x] M[x] F[ ] K[x] DetectText
- W[ ] S[ ] M[ ] F[ ] K[ ] DisassociateFaces
- W[ ] S[ ] M[ ] F[ ] K[ ] DistributeDatasetEntries
- W[ ] S[ ] M[ ] F[ ] K[ ] GetCelebrityInfo
- W[ ] S[ ] M[ ] F[ ] K[ ] GetCelebrityRecognition
- W[ ] S[ ] M[ ] F[ ] K[ ] GetContentModeration
- W[ ] S[ ] M[ ] F[ ] K[ ] GetFaceDetection
- W[ ] S[ ] M[ ] F[ ] K[ ] GetFaceLivenessSessionResults
- W[x] S[ ] M[x] F[ ] K[ ] GetFaceSearch
- W[ ] S[ ] M[ ] F[ ] K[ ] GetLabelDetection
- W[ ] S[ ] M[ ] F[ ] K[ ] GetMediaAnalysisJob
- W[ ] S[ ] M[ ] F[ ] K[ ] GetPersonTracking
- W[ ] S[ ] M[ ] F[ ] K[ ] GetSegmentDetection
- W[x] S[ ] M[x] F[ ] K[ ] GetTextDetection
- W[ ] S[ ] M[ ] F[ ] K[x] IndexFaces
- W[x] S[ ] M[ ] F[ ] K[x] ListCollections
- W[ ] S[ ] M[ ] F[ ] K[ ] ListDatasetEntries
- W[ ] S[ ] M[ ] F[ ] K[ ] ListDatasetLabels
- W[ ] S[ ] M[ ] F[ ] K[x] ListFaces
- W[ ] S[ ] M[ ] F[ ] K[ ] ListMediaAnalysisJobs
- W[ ] S[ ] M[ ] F[ ] K[ ] ListProjectPolicies
- W[ ] S[ ] M[ ] F[ ] K[ ] ListStreamProcessors
- W[ ] S[ ] M[ ] F[ ] K[ ] ListTagsForResource
- W[ ] S[ ] M[ ] F[ ] K[ ] ListUsers
- W[ ] S[ ] M[ ] F[ ] K[ ] PutProjectPolicy
- W[ ] S[ ] M[ ] F[ ] K[x] RecognizeCelebrities
- W[ ] S[ ] M[ ] F[ ] K[x] SearchFaces
- W[ ] S[ ] M[ ] F[ ] K[ ] SearchFacesByImage
- W[ ] S[ ] M[ ] F[ ] K[ ] SearchUsers
- W[ ] S[ ] M[ ] F[ ] K[ ] SearchUsersByImage
- W[ ] S[ ] M[ ] F[ ] K[ ] StartCelebrityRecognition
- W[ ] S[ ] M[ ] F[ ] K[ ] StartContentModeration
- W[ ] S[ ] M[ ] F[ ] K[ ] StartFaceDetection
- W[x] S[ ] M[x] F[ ] K[ ] StartFaceSearch
- W[ ] S[ ] M[ ] F[ ] K[ ] StartLabelDetection
- W[ ] S[ ] M[ ] F[ ] K[ ] StartMediaAnalysisJob
- W[ ] S[ ] M[ ] F[ ] K[ ] StartPersonTracking
- W[ ] S[ ] M[ ] F[ ] K[ ] StartProjectVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] StartSegmentDetection
- W[ ] S[ ] M[ ] F[ ] K[ ] StartStreamProcessor
- W[x] S[ ] M[x] F[ ] K[ ] StartTextDetection
- W[ ] S[ ] M[ ] F[ ] K[ ] StopProjectVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] StopStreamProcessor
- W[ ] S[ ] M[ ] F[ ] K[ ] TagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] UntagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateDatasetEntries
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateStreamProcessor

Integration tests: 8/8 implemented operations tested (100.0%)

### winterbaume-resiliencehub (resiliencehub) - W: 22/63, S: 0/63, M: 17/63, F: 0/63, K: 17/63

- W[ ] S[ ] M[ ] F[ ] K[ ] AcceptResourceGroupingRecommendations
- W[ ] S[ ] M[ ] F[ ] K[ ] AddDraftAppVersionResourceMappings
- W[ ] S[ ] M[ ] F[ ] K[ ] BatchUpdateRecommendationStatus
- W[x] S[ ] M[x] F[ ] K[x] CreateApp
- W[x] S[ ] M[x] F[ ] K[ ] CreateAppVersionAppComponent
- W[x] S[ ] M[x] F[ ] K[ ] CreateAppVersionResource
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateRecommendationTemplate
- W[x] S[ ] M[x] F[ ] K[x] CreateResiliencyPolicy
- W[x] S[ ] M[ ] F[ ] K[x] DeleteApp
- W[ ] S[ ] M[ ] F[ ] K[x] DeleteAppAssessment
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteAppInputSource
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteAppVersionAppComponent
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteAppVersionResource
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteRecommendationTemplate
- W[x] S[ ] M[ ] F[ ] K[x] DeleteResiliencyPolicy
- W[x] S[ ] M[x] F[ ] K[x] DescribeApp
- W[ ] S[ ] M[ ] F[ ] K[x] DescribeAppAssessment
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeAppVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeAppVersionAppComponent
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeAppVersionResource
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeAppVersionResourcesResolutionStatus
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeAppVersionTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeDraftAppVersionResourcesImportStatus
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeMetricsExport
- W[x] S[ ] M[x] F[ ] K[x] DescribeResiliencyPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeResourceGroupingRecommendationTask
- W[x] S[ ] M[x] F[ ] K[ ] ImportResourcesToDraftAppVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] ListAlarmRecommendations
- W[ ] S[ ] M[ ] F[ ] K[ ] ListAppAssessmentComplianceDrifts
- W[ ] S[ ] M[ ] F[ ] K[ ] ListAppAssessmentResourceDrifts
- W[x] S[ ] M[x] F[ ] K[x] ListAppAssessments
- W[ ] S[ ] M[ ] F[ ] K[ ] ListAppComponentCompliances
- W[ ] S[ ] M[ ] F[ ] K[ ] ListAppComponentRecommendations
- W[ ] S[ ] M[ ] F[ ] K[ ] ListAppInputSources
- W[x] S[ ] M[x] F[ ] K[ ] ListAppVersionAppComponents
- W[ ] S[ ] M[ ] F[ ] K[ ] ListAppVersionResourceMappings
- W[x] S[ ] M[x] F[ ] K[ ] ListAppVersionResources
- W[x] S[ ] M[x] F[ ] K[ ] ListAppVersions
- W[x] S[ ] M[x] F[ ] K[x] ListApps
- W[ ] S[ ] M[ ] F[ ] K[ ] ListMetrics
- W[ ] S[ ] M[ ] F[ ] K[ ] ListRecommendationTemplates
- W[x] S[ ] M[x] F[ ] K[x] ListResiliencyPolicies
- W[ ] S[ ] M[ ] F[ ] K[ ] ListResourceGroupingRecommendations
- W[ ] S[ ] M[ ] F[ ] K[ ] ListSopRecommendations
- W[x] S[ ] M[ ] F[ ] K[ ] ListSuggestedResiliencyPolicies
- W[x] S[ ] M[x] F[ ] K[x] ListTagsForResource
- W[ ] S[ ] M[ ] F[ ] K[ ] ListTestRecommendations
- W[ ] S[ ] M[ ] F[ ] K[ ] ListUnsupportedAppVersionResources
- W[x] S[ ] M[x] F[ ] K[ ] PublishAppVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] PutDraftAppVersionTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] RejectResourceGroupingRecommendations
- W[ ] S[ ] M[ ] F[ ] K[ ] RemoveDraftAppVersionResourceMappings
- W[ ] S[ ] M[ ] F[ ] K[ ] ResolveAppVersionResources
- W[ ] S[ ] M[ ] F[ ] K[x] StartAppAssessment
- W[ ] S[ ] M[ ] F[ ] K[ ] StartMetricsExport
- W[ ] S[ ] M[ ] F[ ] K[ ] StartResourceGroupingRecommendationTask
- W[x] S[ ] M[x] F[ ] K[x] TagResource
- W[x] S[ ] M[x] F[ ] K[x] UntagResource
- W[ ] S[ ] M[ ] F[ ] K[x] UpdateApp
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateAppVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateAppVersionAppComponent
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateAppVersionResource
- W[x] S[ ] M[ ] F[ ] K[x] UpdateResiliencyPolicy

Integration tests: 22/22 implemented operations tested (100.0%)

### winterbaume-resourcegroups (resource-groups) - W: 22/23, S: 1/23, M: 15/23, F: 0/23, K: 0/23

- W[x] S[ ] M[x] F[ ] K[ ] CancelTagSyncTask
- W[x] S[ ] M[x] F[ ] K[ ] CreateGroup
- W[x] S[ ] M[x] F[ ] K[ ] DeleteGroup
- W[x] S[ ] M[ ] F[ ] K[ ] GetAccountSettings
- W[x] S[ ] M[x] F[ ] K[ ] GetGroup
- W[x] S[ ] M[x] F[ ] K[ ] GetGroupConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] GetGroupQuery
- W[x] S[ ] M[x] F[ ] K[ ] GetTagSyncTask
- W[x] S[ ] M[x] F[ ] K[ ] GetTags
- W[x] S[ ] M[ ] F[ ] K[ ] GroupResources
- W[x] S[ ] M[ ] F[ ] K[ ] ListGroupResources
- W[x] S[ ] M[ ] F[ ] K[ ] ListGroupingStatuses
- W[x] S[ ] M[x] F[ ] K[ ] ListGroups
- W[x] S[ ] M[x] F[ ] K[ ] ListTagSyncTasks
- W[x] S[ ] M[x] F[ ] K[ ] PutGroupConfiguration
- W[ ] S[x] M[ ] F[ ] K[ ] SearchResources
- W[x] S[ ] M[x] F[ ] K[ ] StartTagSyncTask
- W[x] S[ ] M[x] F[ ] K[ ] Tag
- W[x] S[ ] M[ ] F[ ] K[ ] UngroupResources
- W[x] S[ ] M[x] F[ ] K[ ] Untag
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateAccountSettings
- W[x] S[ ] M[x] F[ ] K[ ] UpdateGroup
- W[x] S[ ] M[x] F[ ] K[ ] UpdateGroupQuery

Integration tests: 15/22 implemented operations tested (68.2%)
Untested implemented operations: 7

### winterbaume-resourcegroupstagging (resource-groups-tagging-api) - W: 5/9, S: 0/9, M: 0/9, F: 0/9, K: 0/9

- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeReportCreation
- W[ ] S[ ] M[ ] F[ ] K[ ] GetComplianceSummary
- W[x] S[ ] M[ ] F[ ] K[ ] GetResources
- W[x] S[ ] M[ ] F[ ] K[ ] GetTagKeys
- W[x] S[ ] M[ ] F[ ] K[ ] GetTagValues
- W[ ] S[ ] M[ ] F[ ] K[ ] ListRequiredTags
- W[ ] S[ ] M[ ] F[ ] K[ ] StartReportCreation
- W[x] S[ ] M[ ] F[ ] K[ ] TagResources
- W[x] S[ ] M[ ] F[ ] K[ ] UntagResources

Integration tests: 5/5 implemented operations tested (100.0%)

### winterbaume-rolesanywhere (rolesanywhere) - W: 28/30, S: 2/30, M: 0/30, F: 0/30, K: 0/30

- W[x] S[ ] M[ ] F[ ] K[ ] CreateProfile
- W[x] S[ ] M[ ] F[ ] K[ ] CreateTrustAnchor
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteAttributeMapping
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteCrl
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteProfile
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteTrustAnchor
- W[x] S[ ] M[ ] F[ ] K[ ] DisableCrl
- W[x] S[ ] M[ ] F[ ] K[ ] DisableProfile
- W[x] S[ ] M[ ] F[ ] K[ ] DisableTrustAnchor
- W[x] S[ ] M[ ] F[ ] K[ ] EnableCrl
- W[x] S[ ] M[ ] F[ ] K[ ] EnableProfile
- W[x] S[ ] M[ ] F[ ] K[ ] EnableTrustAnchor
- W[x] S[ ] M[ ] F[ ] K[ ] GetCrl
- W[x] S[ ] M[ ] F[ ] K[ ] GetProfile
- W[ ] S[x] M[ ] F[ ] K[ ] GetSubject
- W[x] S[ ] M[ ] F[ ] K[ ] GetTrustAnchor
- W[x] S[ ] M[ ] F[ ] K[ ] ImportCrl
- W[x] S[ ] M[ ] F[ ] K[ ] ListCrls
- W[x] S[ ] M[ ] F[ ] K[ ] ListProfiles
- W[ ] S[x] M[ ] F[ ] K[ ] ListSubjects
- W[x] S[ ] M[ ] F[ ] K[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] ListTrustAnchors
- W[x] S[ ] M[ ] F[ ] K[ ] PutAttributeMapping
- W[x] S[ ] M[ ] F[ ] K[ ] PutNotificationSettings
- W[x] S[ ] M[ ] F[ ] K[ ] ResetNotificationSettings
- W[x] S[ ] M[ ] F[ ] K[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateCrl
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateProfile
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateTrustAnchor

Integration tests: 22/28 implemented operations tested (78.6%)
Untested implemented operations: 6

### winterbaume-route53 (route-53) - W: 71/71, S: 0/71, M: 30/71, F: 0/71, K: 10/71

Terraform E2E: 6 tests across 2 terraform resource types

Resource types: aws_route53_record, aws_route53_zone

- W[x] S[ ] M[ ] F[ ] K[ ] ActivateKeySigningKey
- W[x] S[ ] M[x] F[ ] K[ ] AssociateVPCWithHostedZone
- W[x] S[ ] M[ ] F[ ] K[ ] ChangeCidrCollection
- W[x] S[ ] M[x] F[ ] K[x] ChangeResourceRecordSets
- W[x] S[ ] M[x] F[ ] K[x] ChangeTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] CreateCidrCollection
- W[x] S[ ] M[x] F[ ] K[ ] CreateHealthCheck
- W[x] S[ ] M[x] F[ ] K[x] CreateHostedZone
- W[x] S[ ] M[ ] F[ ] K[ ] CreateKeySigningKey
- W[x] S[ ] M[x] F[ ] K[ ] CreateQueryLoggingConfig
- W[x] S[ ] M[x] F[ ] K[ ] CreateReusableDelegationSet
- W[x] S[ ] M[ ] F[ ] K[ ] CreateTrafficPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] CreateTrafficPolicyInstance
- W[x] S[ ] M[ ] F[ ] K[ ] CreateTrafficPolicyVersion
- W[x] S[ ] M[ ] F[ ] K[ ] CreateVPCAssociationAuthorization
- W[x] S[ ] M[ ] F[ ] K[ ] DeactivateKeySigningKey
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteCidrCollection
- W[x] S[ ] M[x] F[ ] K[ ] DeleteHealthCheck
- W[x] S[ ] M[x] F[ ] K[x] DeleteHostedZone
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteKeySigningKey
- W[x] S[ ] M[x] F[ ] K[ ] DeleteQueryLoggingConfig
- W[x] S[ ] M[x] F[ ] K[ ] DeleteReusableDelegationSet
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteTrafficPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteTrafficPolicyInstance
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteVPCAssociationAuthorization
- W[x] S[ ] M[ ] F[ ] K[ ] DisableHostedZoneDNSSEC
- W[x] S[ ] M[x] F[ ] K[ ] DisassociateVPCFromHostedZone
- W[x] S[ ] M[ ] F[ ] K[ ] EnableHostedZoneDNSSEC
- W[x] S[ ] M[ ] F[ ] K[ ] GetAccountLimit
- W[x] S[ ] M[ ] F[ ] K[x] GetChange
- W[x] S[ ] M[ ] F[ ] K[ ] GetCheckerIpRanges
- W[x] S[ ] M[x] F[ ] K[ ] GetDNSSEC
- W[x] S[ ] M[ ] F[ ] K[ ] GetGeoLocation
- W[x] S[ ] M[x] F[ ] K[ ] GetHealthCheck
- W[x] S[ ] M[ ] F[ ] K[ ] GetHealthCheckCount
- W[x] S[ ] M[ ] F[ ] K[ ] GetHealthCheckLastFailureReason
- W[x] S[ ] M[x] F[ ] K[ ] GetHealthCheckStatus
- W[x] S[ ] M[x] F[ ] K[x] GetHostedZone
- W[x] S[ ] M[x] F[ ] K[ ] GetHostedZoneCount
- W[x] S[ ] M[ ] F[ ] K[ ] GetHostedZoneLimit
- W[x] S[ ] M[x] F[ ] K[ ] GetQueryLoggingConfig
- W[x] S[ ] M[x] F[ ] K[ ] GetReusableDelegationSet
- W[x] S[ ] M[ ] F[ ] K[ ] GetReusableDelegationSetLimit
- W[x] S[ ] M[ ] F[ ] K[ ] GetTrafficPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] GetTrafficPolicyInstance
- W[x] S[ ] M[ ] F[ ] K[ ] GetTrafficPolicyInstanceCount
- W[x] S[ ] M[ ] F[ ] K[ ] ListCidrBlocks
- W[x] S[ ] M[ ] F[ ] K[ ] ListCidrCollections
- W[x] S[ ] M[ ] F[ ] K[ ] ListCidrLocations
- W[x] S[ ] M[ ] F[ ] K[ ] ListGeoLocations
- W[x] S[ ] M[x] F[ ] K[ ] ListHealthChecks
- W[x] S[ ] M[x] F[ ] K[x] ListHostedZones
- W[x] S[ ] M[x] F[ ] K[x] ListHostedZonesByName
- W[x] S[ ] M[x] F[ ] K[ ] ListHostedZonesByVPC
- W[x] S[ ] M[x] F[ ] K[ ] ListQueryLoggingConfigs
- W[x] S[ ] M[x] F[ ] K[x] ListResourceRecordSets
- W[x] S[ ] M[x] F[ ] K[ ] ListReusableDelegationSets
- W[x] S[ ] M[x] F[ ] K[x] ListTagsForResource
- W[x] S[ ] M[x] F[ ] K[ ] ListTagsForResources
- W[x] S[ ] M[ ] F[ ] K[ ] ListTrafficPolicies
- W[x] S[ ] M[ ] F[ ] K[ ] ListTrafficPolicyInstances
- W[x] S[ ] M[ ] F[ ] K[ ] ListTrafficPolicyInstancesByHostedZone
- W[x] S[ ] M[ ] F[ ] K[ ] ListTrafficPolicyInstancesByPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] ListTrafficPolicyVersions
- W[x] S[ ] M[ ] F[ ] K[ ] ListVPCAssociationAuthorizations
- W[x] S[ ] M[ ] F[ ] K[ ] TestDNSAnswer
- W[x] S[ ] M[x] F[ ] K[ ] UpdateHealthCheck
- W[x] S[ ] M[x] F[ ] K[ ] UpdateHostedZoneComment
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateHostedZoneFeatures
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateTrafficPolicyComment
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateTrafficPolicyInstance

Integration tests: 70/71 implemented operations tested (98.6%)
Untested implemented operations: 1

### winterbaume-route53domains (route-53-domains) - W: 5/34, S: 0/34, M: 0/34, F: 0/34, K: 0/34

- W[ ] S[ ] M[ ] F[ ] K[ ] AcceptDomainTransferFromAnotherAwsAccount
- W[ ] S[ ] M[ ] F[ ] K[ ] AssociateDelegationSignerToDomain
- W[ ] S[ ] M[ ] F[ ] K[ ] CancelDomainTransferToAnotherAwsAccount
- W[x] S[ ] M[ ] F[ ] K[ ] CheckDomainAvailability
- W[ ] S[ ] M[ ] F[ ] K[ ] CheckDomainTransferability
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteDomain
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteTagsForDomain
- W[ ] S[ ] M[ ] F[ ] K[ ] DisableDomainAutoRenew
- W[ ] S[ ] M[ ] F[ ] K[ ] DisableDomainTransferLock
- W[ ] S[ ] M[ ] F[ ] K[ ] DisassociateDelegationSignerFromDomain
- W[ ] S[ ] M[ ] F[ ] K[ ] EnableDomainAutoRenew
- W[ ] S[ ] M[ ] F[ ] K[ ] EnableDomainTransferLock
- W[ ] S[ ] M[ ] F[ ] K[ ] GetContactReachabilityStatus
- W[x] S[ ] M[ ] F[ ] K[ ] GetDomainDetail
- W[ ] S[ ] M[ ] F[ ] K[ ] GetDomainSuggestions
- W[ ] S[ ] M[ ] F[ ] K[ ] GetOperationDetail
- W[x] S[ ] M[ ] F[ ] K[ ] ListDomains
- W[ ] S[ ] M[ ] F[ ] K[ ] ListOperations
- W[ ] S[ ] M[ ] F[ ] K[ ] ListPrices
- W[ ] S[ ] M[ ] F[ ] K[ ] ListTagsForDomain
- W[ ] S[ ] M[ ] F[ ] K[ ] PushDomain
- W[x] S[ ] M[ ] F[ ] K[ ] RegisterDomain
- W[ ] S[ ] M[ ] F[ ] K[ ] RejectDomainTransferFromAnotherAwsAccount
- W[ ] S[ ] M[ ] F[ ] K[ ] RenewDomain
- W[ ] S[ ] M[ ] F[ ] K[ ] ResendContactReachabilityEmail
- W[ ] S[ ] M[ ] F[ ] K[ ] ResendOperationAuthorization
- W[ ] S[ ] M[ ] F[ ] K[ ] RetrieveDomainAuthCode
- W[ ] S[ ] M[ ] F[ ] K[ ] TransferDomain
- W[ ] S[ ] M[ ] F[ ] K[ ] TransferDomainToAnotherAwsAccount
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateDomainContact
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateDomainContactPrivacy
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateDomainNameservers
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateTagsForDomain
- W[ ] S[ ] M[ ] F[ ] K[ ] ViewBilling

Integration tests: 5/5 implemented operations tested (100.0%)

### winterbaume-route53recoverycluster (route53-recovery-cluster) - W: 4/4, S: 0/4, M: 0/4, F: 0/4, K: 0/4

- W[x] S[ ] M[ ] F[ ] K[ ] GetRoutingControlState
- W[x] S[ ] M[ ] F[ ] K[ ] ListRoutingControls
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateRoutingControlState
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateRoutingControlStates

Integration tests: 4/4 implemented operations tested (100.0%)

### winterbaume-route53resolver (route53resolver) - W: 28/68, S: 0/68, M: 28/68, F: 0/68, K: 11/68

- W[ ] S[ ] M[ ] F[ ] K[ ] AssociateFirewallRuleGroup
- W[x] S[ ] M[x] F[ ] K[ ] AssociateResolverEndpointIpAddress
- W[x] S[ ] M[x] F[ ] K[ ] AssociateResolverQueryLogConfig
- W[x] S[ ] M[x] F[ ] K[x] AssociateResolverRule
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateFirewallDomainList
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateFirewallRule
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateFirewallRuleGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateOutpostResolver
- W[x] S[ ] M[x] F[ ] K[x] CreateResolverEndpoint
- W[x] S[ ] M[x] F[ ] K[ ] CreateResolverQueryLogConfig
- W[x] S[ ] M[x] F[ ] K[x] CreateResolverRule
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteFirewallDomainList
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteFirewallRule
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteFirewallRuleGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteOutpostResolver
- W[x] S[ ] M[x] F[ ] K[x] DeleteResolverEndpoint
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteResolverQueryLogConfig
- W[x] S[ ] M[x] F[ ] K[x] DeleteResolverRule
- W[ ] S[ ] M[ ] F[ ] K[ ] DisassociateFirewallRuleGroup
- W[x] S[ ] M[x] F[ ] K[ ] DisassociateResolverEndpointIpAddress
- W[ ] S[ ] M[ ] F[ ] K[ ] DisassociateResolverQueryLogConfig
- W[x] S[ ] M[x] F[ ] K[x] DisassociateResolverRule
- W[ ] S[ ] M[ ] F[ ] K[ ] GetFirewallConfig
- W[ ] S[ ] M[ ] F[ ] K[ ] GetFirewallDomainList
- W[ ] S[ ] M[ ] F[ ] K[ ] GetFirewallRuleGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] GetFirewallRuleGroupAssociation
- W[ ] S[ ] M[ ] F[ ] K[ ] GetFirewallRuleGroupPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] GetOutpostResolver
- W[ ] S[ ] M[ ] F[ ] K[ ] GetResolverConfig
- W[x] S[ ] M[x] F[ ] K[ ] GetResolverDnssecConfig
- W[x] S[ ] M[x] F[ ] K[x] GetResolverEndpoint
- W[x] S[ ] M[x] F[ ] K[ ] GetResolverQueryLogConfig
- W[x] S[ ] M[x] F[ ] K[ ] GetResolverQueryLogConfigAssociation
- W[ ] S[ ] M[ ] F[ ] K[ ] GetResolverQueryLogConfigPolicy
- W[x] S[ ] M[x] F[ ] K[x] GetResolverRule
- W[x] S[ ] M[x] F[ ] K[ ] GetResolverRuleAssociation
- W[ ] S[ ] M[ ] F[ ] K[ ] GetResolverRulePolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] ImportFirewallDomains
- W[ ] S[ ] M[ ] F[ ] K[ ] ListFirewallConfigs
- W[ ] S[ ] M[ ] F[ ] K[ ] ListFirewallDomainLists
- W[ ] S[ ] M[ ] F[ ] K[ ] ListFirewallDomains
- W[ ] S[ ] M[ ] F[ ] K[ ] ListFirewallRuleGroupAssociations
- W[ ] S[ ] M[ ] F[ ] K[ ] ListFirewallRuleGroups
- W[ ] S[ ] M[ ] F[ ] K[ ] ListFirewallRules
- W[ ] S[ ] M[ ] F[ ] K[ ] ListOutpostResolvers
- W[ ] S[ ] M[ ] F[ ] K[ ] ListResolverConfigs
- W[x] S[ ] M[x] F[ ] K[ ] ListResolverDnssecConfigs
- W[x] S[ ] M[x] F[ ] K[ ] ListResolverEndpointIpAddresses
- W[x] S[ ] M[x] F[ ] K[x] ListResolverEndpoints
- W[x] S[ ] M[x] F[ ] K[ ] ListResolverQueryLogConfigAssociations
- W[x] S[ ] M[x] F[ ] K[ ] ListResolverQueryLogConfigs
- W[x] S[ ] M[x] F[ ] K[x] ListResolverRuleAssociations
- W[x] S[ ] M[x] F[ ] K[x] ListResolverRules
- W[x] S[ ] M[x] F[ ] K[ ] ListTagsForResource
- W[ ] S[ ] M[ ] F[ ] K[ ] PutFirewallRuleGroupPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] PutResolverQueryLogConfigPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] PutResolverRulePolicy
- W[x] S[ ] M[x] F[ ] K[ ] TagResource
- W[x] S[ ] M[x] F[ ] K[ ] UntagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateFirewallConfig
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateFirewallDomains
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateFirewallRule
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateFirewallRuleGroupAssociation
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateOutpostResolver
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateResolverConfig
- W[x] S[ ] M[x] F[ ] K[ ] UpdateResolverDnssecConfig
- W[x] S[ ] M[x] F[ ] K[ ] UpdateResolverEndpoint
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateResolverRule

Integration tests: 28/28 implemented operations tested (100.0%)

### winterbaume-s3 (s3) - W: 103/107, S: 4/107, M: 73/107, F: 51/107, K: 36/107

Terraform E2E: 13 tests across 6 terraform resource types

Resource types: aws_s3_bucket, aws_s3_bucket_ownership_controls, aws_s3_bucket_policy, aws_s3_bucket_public_access_block, aws_s3_bucket_server_side_encryption_configuration, aws_s3_bucket_versioning

- W[x] S[ ] M[x] F[x] K[x] AbortMultipartUpload
- W[x] S[ ] M[x] F[x] K[x] CompleteMultipartUpload
- W[x] S[ ] M[x] F[x] K[x] CopyObject
- W[x] S[ ] M[x] F[x] K[x] CreateBucket
- W[x] S[ ] M[ ] F[ ] K[ ] CreateBucketMetadataConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] CreateBucketMetadataTableConfiguration
- W[x] S[ ] M[x] F[x] K[x] CreateMultipartUpload
- W[ ] S[x] M[ ] F[ ] K[ ] CreateSession
- W[x] S[ ] M[x] F[x] K[x] DeleteBucket
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteBucketAnalyticsConfiguration
- W[x] S[ ] M[x] F[x] K[ ] DeleteBucketCors
- W[x] S[ ] M[x] F[x] K[x] DeleteBucketEncryption
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteBucketIntelligentTieringConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteBucketInventoryConfiguration
- W[x] S[ ] M[x] F[x] K[ ] DeleteBucketLifecycle
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteBucketMetadataConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteBucketMetadataTableConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteBucketMetricsConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] DeleteBucketOwnershipControls
- W[x] S[ ] M[x] F[x] K[x] DeleteBucketPolicy
- W[x] S[ ] M[x] F[ ] K[ ] DeleteBucketReplication
- W[x] S[ ] M[x] F[ ] K[ ] DeleteBucketTagging
- W[x] S[ ] M[x] F[ ] K[ ] DeleteBucketWebsite
- W[x] S[ ] M[x] F[x] K[x] DeleteObject
- W[x] S[ ] M[x] F[x] K[ ] DeleteObjectTagging
- W[x] S[ ] M[x] F[x] K[x] DeleteObjects
- W[x] S[ ] M[x] F[x] K[x] DeletePublicAccessBlock
- W[x] S[ ] M[ ] F[ ] K[ ] GetBucketAbac
- W[x] S[ ] M[x] F[ ] K[ ] GetBucketAccelerateConfiguration
- W[x] S[ ] M[x] F[x] K[ ] GetBucketAcl
- W[x] S[ ] M[ ] F[ ] K[ ] GetBucketAnalyticsConfiguration
- W[x] S[ ] M[x] F[x] K[ ] GetBucketCors
- W[x] S[ ] M[x] F[x] K[x] GetBucketEncryption
- W[x] S[ ] M[ ] F[ ] K[ ] GetBucketIntelligentTieringConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] GetBucketInventoryConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] GetBucketLifecycleConfiguration
- W[x] S[ ] M[x] F[x] K[ ] GetBucketLocation
- W[x] S[ ] M[x] F[ ] K[x] GetBucketLogging
- W[x] S[ ] M[ ] F[ ] K[ ] GetBucketMetadataConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] GetBucketMetadataTableConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] GetBucketMetricsConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] GetBucketNotificationConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] GetBucketOwnershipControls
- W[x] S[ ] M[x] F[x] K[x] GetBucketPolicy
- W[ ] S[x] M[ ] F[ ] K[ ] GetBucketPolicyStatus
- W[x] S[ ] M[x] F[ ] K[ ] GetBucketReplication
- W[x] S[ ] M[ ] F[ ] K[ ] GetBucketRequestPayment
- W[x] S[ ] M[x] F[x] K[ ] GetBucketTagging
- W[x] S[ ] M[x] F[x] K[x] GetBucketVersioning
- W[x] S[ ] M[ ] F[ ] K[ ] GetBucketWebsite
- W[x] S[ ] M[x] F[x] K[x] GetObject
- W[x] S[ ] M[x] F[x] K[ ] GetObjectAcl
- W[x] S[ ] M[x] F[x] K[ ] GetObjectAttributes
- W[x] S[ ] M[x] F[x] K[ ] GetObjectLegalHold
- W[x] S[ ] M[x] F[x] K[ ] GetObjectLockConfiguration
- W[x] S[ ] M[ ] F[x] K[ ] GetObjectRetention
- W[x] S[ ] M[x] F[x] K[x] GetObjectTagging
- W[ ] S[x] M[ ] F[ ] K[ ] GetObjectTorrent
- W[x] S[ ] M[x] F[x] K[x] GetPublicAccessBlock
- W[x] S[ ] M[x] F[x] K[x] HeadBucket
- W[x] S[ ] M[x] F[x] K[x] HeadObject
- W[x] S[ ] M[ ] F[ ] K[ ] ListBucketAnalyticsConfigurations
- W[x] S[ ] M[ ] F[ ] K[ ] ListBucketIntelligentTieringConfigurations
- W[x] S[ ] M[x] F[ ] K[ ] ListBucketInventoryConfigurations
- W[x] S[ ] M[ ] F[ ] K[ ] ListBucketMetricsConfigurations
- W[x] S[ ] M[x] F[x] K[x] ListBuckets
- W[x] S[ ] M[ ] F[ ] K[ ] ListDirectoryBuckets
- W[x] S[ ] M[x] F[x] K[x] ListMultipartUploads
- W[x] S[ ] M[x] F[x] K[x] ListObjectVersions
- W[x] S[ ] M[x] F[x] K[x] ListObjects
- W[x] S[ ] M[x] F[x] K[ ] ListObjectsV2
- W[x] S[ ] M[x] F[ ] K[x] ListParts
- W[x] S[ ] M[ ] F[ ] K[ ] PutBucketAbac
- W[x] S[ ] M[x] F[ ] K[ ] PutBucketAccelerateConfiguration
- W[x] S[ ] M[x] F[x] K[ ] PutBucketAcl
- W[x] S[ ] M[ ] F[ ] K[ ] PutBucketAnalyticsConfiguration
- W[x] S[ ] M[x] F[x] K[x] PutBucketCors
- W[x] S[ ] M[x] F[x] K[x] PutBucketEncryption
- W[x] S[ ] M[ ] F[ ] K[ ] PutBucketIntelligentTieringConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] PutBucketInventoryConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] PutBucketLifecycleConfiguration
- W[x] S[ ] M[x] F[ ] K[x] PutBucketLogging
- W[x] S[ ] M[ ] F[ ] K[ ] PutBucketMetricsConfiguration
- W[x] S[ ] M[x] F[ ] K[x] PutBucketNotificationConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] PutBucketOwnershipControls
- W[x] S[ ] M[x] F[x] K[x] PutBucketPolicy
- W[x] S[ ] M[x] F[ ] K[ ] PutBucketReplication
- W[x] S[ ] M[ ] F[ ] K[ ] PutBucketRequestPayment
- W[x] S[ ] M[x] F[x] K[ ] PutBucketTagging
- W[x] S[ ] M[x] F[x] K[x] PutBucketVersioning
- W[x] S[ ] M[x] F[ ] K[ ] PutBucketWebsite
- W[x] S[ ] M[x] F[x] K[x] PutObject
- W[x] S[ ] M[x] F[x] K[ ] PutObjectAcl
- W[x] S[ ] M[x] F[x] K[ ] PutObjectLegalHold
- W[x] S[ ] M[x] F[x] K[ ] PutObjectLockConfiguration
- W[x] S[ ] M[x] F[x] K[ ] PutObjectRetention
- W[x] S[ ] M[x] F[x] K[x] PutObjectTagging
- W[x] S[ ] M[x] F[x] K[x] PutPublicAccessBlock
- W[x] S[ ] M[ ] F[ ] K[ ] RenameObject
- W[ ] S[x] M[x] F[ ] K[ ] RestoreObject
- W[x] S[ ] M[x] F[x] K[ ] SelectObjectContent
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateBucketMetadataInventoryTableConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateBucketMetadataJournalTableConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateObjectEncryption
- W[x] S[ ] M[x] F[x] K[x] UploadPart
- W[x] S[ ] M[x] F[ ] K[x] UploadPartCopy
- W[x] S[ ] M[ ] F[ ] K[ ] WriteGetObjectResponse

Integration tests: 70/103 implemented operations tested (68.0%)
Untested implemented operations: 33

### winterbaume-s3control (s3-control) - W: 87/97, S: 10/97, M: 0/97, F: 0/97, K: 7/97

- W[ ] S[x] M[ ] F[ ] K[ ] AssociateAccessGrantsIdentityCenter
- W[x] S[ ] M[ ] F[ ] K[ ] CreateAccessGrant
- W[x] S[ ] M[ ] F[ ] K[ ] CreateAccessGrantsInstance
- W[x] S[ ] M[ ] F[ ] K[ ] CreateAccessGrantsLocation
- W[x] S[ ] M[ ] F[ ] K[x] CreateAccessPoint
- W[x] S[ ] M[ ] F[ ] K[ ] CreateAccessPointForObjectLambda
- W[x] S[ ] M[ ] F[ ] K[ ] CreateBucket
- W[x] S[ ] M[ ] F[ ] K[ ] CreateJob
- W[x] S[ ] M[ ] F[ ] K[ ] CreateMultiRegionAccessPoint
- W[x] S[ ] M[ ] F[ ] K[ ] CreateStorageLensGroup
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteAccessGrant
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteAccessGrantsInstance
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteAccessGrantsInstanceResourcePolicy
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteAccessGrantsLocation
- W[x] S[ ] M[ ] F[ ] K[x] DeleteAccessPoint
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteAccessPointForObjectLambda
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteAccessPointPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteAccessPointPolicyForObjectLambda
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteAccessPointScope
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteBucket
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteBucketLifecycleConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteBucketPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteBucketReplication
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteBucketTagging
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteJobTagging
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteMultiRegionAccessPoint
- W[x] S[ ] M[ ] F[ ] K[x] DeletePublicAccessBlock
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteStorageLensConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteStorageLensConfigurationTagging
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteStorageLensGroup
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeJob
- W[ ] S[x] M[ ] F[ ] K[ ] DescribeMultiRegionAccessPointOperation
- W[ ] S[x] M[ ] F[ ] K[ ] DissociateAccessGrantsIdentityCenter
- W[x] S[ ] M[ ] F[ ] K[ ] GetAccessGrant
- W[x] S[ ] M[ ] F[ ] K[ ] GetAccessGrantsInstance
- W[ ] S[x] M[ ] F[ ] K[ ] GetAccessGrantsInstanceForPrefix
- W[x] S[ ] M[ ] F[ ] K[ ] GetAccessGrantsInstanceResourcePolicy
- W[x] S[ ] M[ ] F[ ] K[ ] GetAccessGrantsLocation
- W[x] S[ ] M[ ] F[ ] K[x] GetAccessPoint
- W[ ] S[x] M[ ] F[ ] K[ ] GetAccessPointConfigurationForObjectLambda
- W[x] S[ ] M[ ] F[ ] K[ ] GetAccessPointForObjectLambda
- W[x] S[ ] M[ ] F[ ] K[ ] GetAccessPointPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] GetAccessPointPolicyForObjectLambda
- W[x] S[ ] M[ ] F[ ] K[ ] GetAccessPointPolicyStatus
- W[x] S[ ] M[ ] F[ ] K[ ] GetAccessPointPolicyStatusForObjectLambda
- W[ ] S[x] M[ ] F[ ] K[ ] GetAccessPointScope
- W[x] S[ ] M[ ] F[ ] K[ ] GetBucket
- W[ ] S[x] M[ ] F[ ] K[ ] GetBucketLifecycleConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] GetBucketPolicy
- W[ ] S[x] M[ ] F[ ] K[ ] GetBucketReplication
- W[x] S[ ] M[ ] F[ ] K[ ] GetBucketTagging
- W[x] S[ ] M[ ] F[ ] K[ ] GetBucketVersioning
- W[ ] S[x] M[ ] F[ ] K[ ] GetDataAccess
- W[x] S[ ] M[ ] F[ ] K[ ] GetJobTagging
- W[x] S[ ] M[ ] F[ ] K[ ] GetMultiRegionAccessPoint
- W[x] S[ ] M[ ] F[ ] K[ ] GetMultiRegionAccessPointPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] GetMultiRegionAccessPointPolicyStatus
- W[x] S[ ] M[ ] F[ ] K[ ] GetMultiRegionAccessPointRoutes
- W[x] S[ ] M[ ] F[ ] K[x] GetPublicAccessBlock
- W[x] S[ ] M[ ] F[ ] K[ ] GetStorageLensConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] GetStorageLensConfigurationTagging
- W[x] S[ ] M[ ] F[ ] K[ ] GetStorageLensGroup
- W[x] S[ ] M[ ] F[ ] K[ ] ListAccessGrants
- W[x] S[ ] M[ ] F[ ] K[ ] ListAccessGrantsInstances
- W[x] S[ ] M[ ] F[ ] K[ ] ListAccessGrantsLocations
- W[x] S[ ] M[ ] F[ ] K[x] ListAccessPoints
- W[x] S[ ] M[ ] F[ ] K[ ] ListAccessPointsForDirectoryBuckets
- W[x] S[ ] M[ ] F[ ] K[ ] ListAccessPointsForObjectLambda
- W[ ] S[x] M[ ] F[ ] K[ ] ListCallerAccessGrants
- W[x] S[ ] M[ ] F[ ] K[ ] ListJobs
- W[x] S[ ] M[ ] F[ ] K[ ] ListMultiRegionAccessPoints
- W[x] S[ ] M[ ] F[ ] K[ ] ListRegionalBuckets
- W[x] S[ ] M[ ] F[ ] K[ ] ListStorageLensConfigurations
- W[x] S[ ] M[ ] F[ ] K[ ] ListStorageLensGroups
- W[x] S[ ] M[ ] F[ ] K[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] PutAccessGrantsInstanceResourcePolicy
- W[x] S[ ] M[ ] F[ ] K[ ] PutAccessPointConfigurationForObjectLambda
- W[x] S[ ] M[ ] F[ ] K[ ] PutAccessPointPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] PutAccessPointPolicyForObjectLambda
- W[x] S[ ] M[ ] F[ ] K[ ] PutAccessPointScope
- W[x] S[ ] M[ ] F[ ] K[ ] PutBucketLifecycleConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] PutBucketPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] PutBucketReplication
- W[x] S[ ] M[ ] F[ ] K[ ] PutBucketTagging
- W[x] S[ ] M[ ] F[ ] K[ ] PutBucketVersioning
- W[x] S[ ] M[ ] F[ ] K[ ] PutJobTagging
- W[x] S[ ] M[ ] F[ ] K[ ] PutMultiRegionAccessPointPolicy
- W[x] S[ ] M[ ] F[ ] K[x] PutPublicAccessBlock
- W[x] S[ ] M[ ] F[ ] K[ ] PutStorageLensConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] PutStorageLensConfigurationTagging
- W[x] S[ ] M[ ] F[ ] K[ ] SubmitMultiRegionAccessPointRoutes
- W[x] S[ ] M[ ] F[ ] K[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateAccessGrantsLocation
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateJobPriority
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateJobStatus
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateStorageLensGroup

Integration tests: 68/87 implemented operations tested (78.2%)
Untested implemented operations: 19

### winterbaume-s3files (s3files) - W: 21/21, S: 0/21, M: 0/21, F: 0/21, K: 0/21

- W[x] S[ ] M[ ] F[ ] K[ ] CreateAccessPoint
- W[x] S[ ] M[ ] F[ ] K[ ] CreateFileSystem
- W[x] S[ ] M[ ] F[ ] K[ ] CreateMountTarget
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteAccessPoint
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteFileSystem
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteFileSystemPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteMountTarget
- W[x] S[ ] M[ ] F[ ] K[ ] GetAccessPoint
- W[x] S[ ] M[ ] F[ ] K[ ] GetFileSystem
- W[x] S[ ] M[ ] F[ ] K[ ] GetFileSystemPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] GetMountTarget
- W[x] S[ ] M[ ] F[ ] K[ ] GetSynchronizationConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] ListAccessPoints
- W[x] S[ ] M[ ] F[ ] K[ ] ListFileSystems
- W[x] S[ ] M[ ] F[ ] K[ ] ListMountTargets
- W[x] S[ ] M[ ] F[ ] K[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] PutFileSystemPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] PutSynchronizationConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateMountTarget

Integration tests: 21/21 implemented operations tested (100.0%)

### winterbaume-s3outposts (s3outposts) - W: 3/5, S: 1/5, M: 0/5, F: 0/5, K: 0/5

- W[x] S[ ] M[ ] F[ ] K[ ] CreateEndpoint
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteEndpoint
- W[x] S[ ] M[ ] F[ ] K[ ] ListEndpoints
- W[ ] S[ ] M[ ] F[ ] K[ ] ListOutpostsWithS3
- W[ ] S[x] M[ ] F[ ] K[ ] ListSharedEndpoints

Integration tests: 3/3 implemented operations tested (100.0%)

### winterbaume-s3tables (s3tables) - W: 46/49, S: 3/49, M: 14/49, F: 0/49, K: 12/49

- W[x] S[ ] M[x] F[ ] K[x] CreateNamespace
- W[x] S[ ] M[x] F[ ] K[x] CreateTable
- W[x] S[ ] M[x] F[ ] K[x] CreateTableBucket
- W[x] S[ ] M[x] F[ ] K[x] DeleteNamespace
- W[x] S[ ] M[x] F[ ] K[x] DeleteTable
- W[x] S[ ] M[x] F[ ] K[x] DeleteTableBucket
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteTableBucketEncryption
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteTableBucketMetricsConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteTableBucketPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteTableBucketReplication
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteTablePolicy
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteTableReplication
- W[x] S[ ] M[x] F[ ] K[x] GetNamespace
- W[x] S[ ] M[x] F[ ] K[x] GetTable
- W[x] S[ ] M[x] F[ ] K[x] GetTableBucket
- W[x] S[ ] M[ ] F[ ] K[ ] GetTableBucketEncryption
- W[x] S[ ] M[ ] F[ ] K[ ] GetTableBucketMaintenanceConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] GetTableBucketMetricsConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] GetTableBucketPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] GetTableBucketReplication
- W[x] S[ ] M[ ] F[ ] K[ ] GetTableBucketStorageClass
- W[x] S[ ] M[ ] F[ ] K[ ] GetTableEncryption
- W[x] S[ ] M[ ] F[ ] K[ ] GetTableMaintenanceConfiguration
- W[ ] S[x] M[ ] F[ ] K[ ] GetTableMaintenanceJobStatus
- W[x] S[ ] M[ ] F[ ] K[ ] GetTableMetadataLocation
- W[x] S[ ] M[ ] F[ ] K[ ] GetTablePolicy
- W[x] S[ ] M[ ] F[ ] K[ ] GetTableRecordExpirationConfiguration
- W[ ] S[x] M[ ] F[ ] K[ ] GetTableRecordExpirationJobStatus
- W[x] S[ ] M[ ] F[ ] K[ ] GetTableReplication
- W[ ] S[x] M[ ] F[ ] K[ ] GetTableReplicationStatus
- W[x] S[ ] M[ ] F[ ] K[ ] GetTableStorageClass
- W[x] S[ ] M[x] F[ ] K[x] ListNamespaces
- W[x] S[ ] M[x] F[ ] K[x] ListTableBuckets
- W[x] S[ ] M[x] F[ ] K[x] ListTables
- W[x] S[ ] M[ ] F[ ] K[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] PutTableBucketEncryption
- W[x] S[ ] M[ ] F[ ] K[ ] PutTableBucketMaintenanceConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] PutTableBucketMetricsConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] PutTableBucketPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] PutTableBucketReplication
- W[x] S[ ] M[ ] F[ ] K[ ] PutTableBucketStorageClass
- W[x] S[ ] M[ ] F[ ] K[ ] PutTableMaintenanceConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] PutTablePolicy
- W[x] S[ ] M[ ] F[ ] K[ ] PutTableRecordExpirationConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] PutTableReplication
- W[x] S[ ] M[x] F[ ] K[ ] RenameTable
- W[x] S[ ] M[ ] F[ ] K[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UntagResource
- W[x] S[ ] M[x] F[ ] K[ ] UpdateTableMetadataLocation

Integration tests: 46/46 implemented operations tested (100.0%)

### winterbaume-s3vectors (s3vectors) - W: 19/19, S: 0/19, M: 15/19, F: 0/19, K: 0/19

- W[x] S[ ] M[x] F[ ] K[ ] CreateIndex
- W[x] S[ ] M[x] F[ ] K[ ] CreateVectorBucket
- W[x] S[ ] M[x] F[ ] K[ ] DeleteIndex
- W[x] S[ ] M[x] F[ ] K[ ] DeleteVectorBucket
- W[x] S[ ] M[x] F[ ] K[ ] DeleteVectorBucketPolicy
- W[x] S[ ] M[x] F[ ] K[ ] DeleteVectors
- W[x] S[ ] M[x] F[ ] K[ ] GetIndex
- W[x] S[ ] M[x] F[ ] K[ ] GetVectorBucket
- W[x] S[ ] M[x] F[ ] K[ ] GetVectorBucketPolicy
- W[x] S[ ] M[x] F[ ] K[ ] GetVectors
- W[x] S[ ] M[x] F[ ] K[ ] ListIndexes
- W[x] S[ ] M[ ] F[ ] K[ ] ListTagsForResource
- W[x] S[ ] M[x] F[ ] K[ ] ListVectorBuckets
- W[x] S[ ] M[x] F[ ] K[ ] ListVectors
- W[x] S[ ] M[x] F[ ] K[ ] PutVectorBucketPolicy
- W[x] S[ ] M[x] F[ ] K[ ] PutVectors
- W[x] S[ ] M[ ] F[ ] K[ ] QueryVectors
- W[x] S[ ] M[ ] F[ ] K[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UntagResource

Integration tests: 19/19 implemented operations tested (100.0%)

### winterbaume-sagemaker (sagemaker) - W: 141/396, S: 1/396, M: 112/396, F: 0/396, K: 11/396

- W[ ] S[ ] M[ ] F[ ] K[ ] AddAssociation
- W[x] S[ ] M[x] F[ ] K[ ] AddTags
- W[x] S[ ] M[x] F[ ] K[ ] AssociateTrialComponent
- W[ ] S[ ] M[ ] F[ ] K[ ] AttachClusterNodeVolume
- W[ ] S[ ] M[ ] F[ ] K[ ] BatchAddClusterNodes
- W[ ] S[ ] M[ ] F[ ] K[ ] BatchDeleteClusterNodes
- W[ ] S[ ] M[ ] F[ ] K[ ] BatchDescribeModelPackage
- W[ ] S[ ] M[ ] F[ ] K[ ] BatchRebootClusterNodes
- W[ ] S[ ] M[ ] F[ ] K[ ] BatchReplaceClusterNodes
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateAIBenchmarkJob
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateAIRecommendationJob
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateAIWorkloadConfig
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateAction
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateAlgorithm
- W[x] S[ ] M[ ] F[ ] K[ ] CreateApp
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateAppImageConfig
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateArtifact
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateAutoMLJob
- W[x] S[ ] M[x] F[ ] K[ ] CreateAutoMLJobV2
- W[x] S[ ] M[x] F[ ] K[ ] CreateCluster
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateClusterSchedulerConfig
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateCodeRepository
- W[x] S[ ] M[x] F[ ] K[ ] CreateCompilationJob
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateComputeQuota
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateContext
- W[x] S[ ] M[x] F[ ] K[ ] CreateDataQualityJobDefinition
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateDeviceFleet
- W[x] S[ ] M[x] F[ ] K[ ] CreateDomain
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateEdgeDeploymentPlan
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateEdgeDeploymentStage
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateEdgePackagingJob
- W[x] S[ ] M[x] F[ ] K[x] CreateEndpoint
- W[x] S[ ] M[x] F[ ] K[ ] CreateEndpointConfig
- W[x] S[ ] M[x] F[ ] K[ ] CreateExperiment
- W[x] S[ ] M[x] F[ ] K[ ] CreateFeatureGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateFlowDefinition
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateHub
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateHubContentPresignedUrls
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateHubContentReference
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateHumanTaskUi
- W[x] S[ ] M[x] F[ ] K[ ] CreateHyperParameterTuningJob
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateImage
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateImageVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateInferenceComponent
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateInferenceExperiment
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateInferenceRecommendationsJob
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateLabelingJob
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateMlflowApp
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateMlflowTrackingServer
- W[x] S[ ] M[x] F[ ] K[x] CreateModel
- W[x] S[ ] M[x] F[ ] K[ ] CreateModelBiasJobDefinition
- W[x] S[ ] M[x] F[ ] K[ ] CreateModelCard
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateModelCardExportJob
- W[x] S[ ] M[x] F[ ] K[ ] CreateModelExplainabilityJobDefinition
- W[x] S[ ] M[x] F[ ] K[ ] CreateModelPackage
- W[x] S[ ] M[x] F[ ] K[ ] CreateModelPackageGroup
- W[x] S[ ] M[x] F[ ] K[ ] CreateModelQualityJobDefinition
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateMonitoringSchedule
- W[x] S[ ] M[x] F[ ] K[x] CreateNotebookInstance
- W[x] S[ ] M[x] F[ ] K[ ] CreateNotebookInstanceLifecycleConfig
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateOptimizationJob
- W[ ] S[ ] M[ ] F[ ] K[ ] CreatePartnerApp
- W[ ] S[ ] M[ ] F[ ] K[ ] CreatePartnerAppPresignedUrl
- W[x] S[ ] M[x] F[ ] K[ ] CreatePipeline
- W[ ] S[ ] M[ ] F[ ] K[ ] CreatePresignedDomainUrl
- W[ ] S[ ] M[ ] F[ ] K[ ] CreatePresignedMlflowAppUrl
- W[ ] S[ ] M[ ] F[ ] K[ ] CreatePresignedMlflowTrackingServerUrl
- W[ ] S[ ] M[ ] F[ ] K[ ] CreatePresignedNotebookInstanceUrl
- W[x] S[ ] M[x] F[ ] K[ ] CreateProcessingJob
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateProject
- W[x] S[ ] M[ ] F[ ] K[ ] CreateSpace
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateStudioLifecycleConfig
- W[x] S[ ] M[x] F[ ] K[x] CreateTrainingJob
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateTrainingPlan
- W[x] S[ ] M[x] F[ ] K[ ] CreateTransformJob
- W[x] S[ ] M[x] F[ ] K[ ] CreateTrial
- W[x] S[ ] M[x] F[ ] K[ ] CreateTrialComponent
- W[x] S[ ] M[ ] F[ ] K[ ] CreateUserProfile
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateWorkforce
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateWorkteam
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteAIBenchmarkJob
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteAIRecommendationJob
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteAIWorkloadConfig
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteAction
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteAlgorithm
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteApp
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteAppImageConfig
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteArtifact
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteAssociation
- W[x] S[ ] M[x] F[ ] K[ ] DeleteCluster
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteClusterSchedulerConfig
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteCodeRepository
- W[x] S[ ] M[x] F[ ] K[ ] DeleteCompilationJob
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteComputeQuota
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteContext
- W[x] S[ ] M[x] F[ ] K[ ] DeleteDataQualityJobDefinition
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteDeviceFleet
- W[x] S[ ] M[x] F[ ] K[ ] DeleteDomain
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteEdgeDeploymentPlan
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteEdgeDeploymentStage
- W[x] S[ ] M[x] F[ ] K[x] DeleteEndpoint
- W[x] S[ ] M[x] F[ ] K[ ] DeleteEndpointConfig
- W[x] S[ ] M[x] F[ ] K[ ] DeleteExperiment
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteFeatureGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteFlowDefinition
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteHub
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteHubContent
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteHubContentReference
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteHumanTaskUi
- W[x] S[ ] M[x] F[ ] K[ ] DeleteHyperParameterTuningJob
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteImage
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteImageVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteInferenceComponent
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteInferenceExperiment
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteMlflowApp
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteMlflowTrackingServer
- W[x] S[ ] M[x] F[ ] K[x] DeleteModel
- W[x] S[ ] M[x] F[ ] K[ ] DeleteModelBiasJobDefinition
- W[x] S[ ] M[x] F[ ] K[ ] DeleteModelCard
- W[x] S[ ] M[x] F[ ] K[ ] DeleteModelExplainabilityJobDefinition
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteModelPackage
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteModelPackageGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteModelPackageGroupPolicy
- W[x] S[ ] M[x] F[ ] K[ ] DeleteModelQualityJobDefinition
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteMonitoringSchedule
- W[x] S[ ] M[x] F[ ] K[x] DeleteNotebookInstance
- W[x] S[ ] M[x] F[ ] K[ ] DeleteNotebookInstanceLifecycleConfig
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteOptimizationJob
- W[ ] S[ ] M[ ] F[ ] K[ ] DeletePartnerApp
- W[x] S[ ] M[x] F[ ] K[ ] DeletePipeline
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteProcessingJob
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteProject
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteSpace
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteStudioLifecycleConfig
- W[x] S[ ] M[x] F[ ] K[ ] DeleteTags
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteTrainingJob
- W[x] S[ ] M[x] F[ ] K[ ] DeleteTrial
- W[x] S[ ] M[x] F[ ] K[ ] DeleteTrialComponent
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteUserProfile
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteWorkforce
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteWorkteam
- W[ ] S[ ] M[ ] F[ ] K[ ] DeregisterDevices
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeAIBenchmarkJob
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeAIRecommendationJob
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeAIWorkloadConfig
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeAction
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeAlgorithm
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeApp
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeAppImageConfig
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeArtifact
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeAutoMLJob
- W[x] S[ ] M[x] F[ ] K[ ] DescribeAutoMLJobV2
- W[x] S[ ] M[x] F[ ] K[ ] DescribeCluster
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeClusterEvent
- W[x] S[ ] M[x] F[ ] K[ ] DescribeClusterNode
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeClusterSchedulerConfig
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeCodeRepository
- W[x] S[ ] M[x] F[ ] K[ ] DescribeCompilationJob
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeComputeQuota
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeContext
- W[x] S[ ] M[x] F[ ] K[ ] DescribeDataQualityJobDefinition
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeDevice
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeDeviceFleet
- W[x] S[ ] M[x] F[ ] K[ ] DescribeDomain
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeEdgeDeploymentPlan
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeEdgePackagingJob
- W[x] S[ ] M[x] F[ ] K[x] DescribeEndpoint
- W[x] S[ ] M[x] F[ ] K[ ] DescribeEndpointConfig
- W[x] S[ ] M[x] F[ ] K[ ] DescribeExperiment
- W[x] S[ ] M[x] F[ ] K[ ] DescribeFeatureGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeFeatureMetadata
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeFlowDefinition
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeHub
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeHubContent
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeHumanTaskUi
- W[x] S[ ] M[x] F[ ] K[ ] DescribeHyperParameterTuningJob
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeImage
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeImageVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeInferenceComponent
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeInferenceExperiment
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeInferenceRecommendationsJob
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeLabelingJob
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeLineageGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeMlflowApp
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeMlflowTrackingServer
- W[x] S[ ] M[x] F[ ] K[ ] DescribeModel
- W[x] S[ ] M[x] F[ ] K[ ] DescribeModelBiasJobDefinition
- W[x] S[ ] M[x] F[ ] K[ ] DescribeModelCard
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeModelCardExportJob
- W[x] S[ ] M[x] F[ ] K[ ] DescribeModelExplainabilityJobDefinition
- W[x] S[ ] M[x] F[ ] K[ ] DescribeModelPackage
- W[x] S[ ] M[x] F[ ] K[ ] DescribeModelPackageGroup
- W[x] S[ ] M[x] F[ ] K[ ] DescribeModelQualityJobDefinition
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeMonitoringSchedule
- W[x] S[ ] M[ ] F[ ] K[x] DescribeNotebookInstance
- W[x] S[ ] M[x] F[ ] K[ ] DescribeNotebookInstanceLifecycleConfig
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeOptimizationJob
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribePartnerApp
- W[x] S[ ] M[x] F[ ] K[ ] DescribePipeline
- W[x] S[ ] M[x] F[ ] K[ ] DescribePipelineDefinitionForExecution
- W[x] S[ ] M[x] F[ ] K[ ] DescribePipelineExecution
- W[x] S[ ] M[x] F[ ] K[ ] DescribeProcessingJob
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeProject
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeReservedCapacity
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeSpace
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeStudioLifecycleConfig
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeSubscribedWorkteam
- W[x] S[ ] M[x] F[ ] K[x] DescribeTrainingJob
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeTrainingPlan
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeTrainingPlanExtensionHistory
- W[x] S[ ] M[x] F[ ] K[ ] DescribeTransformJob
- W[x] S[ ] M[x] F[ ] K[ ] DescribeTrial
- W[x] S[ ] M[x] F[ ] K[ ] DescribeTrialComponent
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeUserProfile
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeWorkforce
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeWorkteam
- W[ ] S[ ] M[ ] F[ ] K[ ] DetachClusterNodeVolume
- W[ ] S[ ] M[ ] F[ ] K[ ] DisableSagemakerServicecatalogPortfolio
- W[x] S[ ] M[x] F[ ] K[ ] DisassociateTrialComponent
- W[ ] S[ ] M[ ] F[ ] K[ ] EnableSagemakerServicecatalogPortfolio
- W[ ] S[ ] M[ ] F[ ] K[ ] ExtendTrainingPlan
- W[ ] S[ ] M[ ] F[ ] K[ ] GetDeviceFleetReport
- W[ ] S[ ] M[ ] F[ ] K[ ] GetLineageGroupPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] GetModelPackageGroupPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] GetSagemakerServicecatalogPortfolioStatus
- W[ ] S[ ] M[ ] F[ ] K[ ] GetScalingConfigurationRecommendation
- W[ ] S[ ] M[ ] F[ ] K[ ] GetSearchSuggestions
- W[ ] S[ ] M[ ] F[ ] K[ ] ImportHubContent
- W[ ] S[ ] M[ ] F[ ] K[ ] ListAIBenchmarkJobs
- W[ ] S[ ] M[ ] F[ ] K[ ] ListAIRecommendationJobs
- W[ ] S[ ] M[ ] F[ ] K[ ] ListAIWorkloadConfigs
- W[ ] S[ ] M[ ] F[ ] K[ ] ListActions
- W[ ] S[ ] M[ ] F[ ] K[ ] ListAlgorithms
- W[ ] S[ ] M[ ] F[ ] K[ ] ListAliases
- W[ ] S[ ] M[ ] F[ ] K[ ] ListAppImageConfigs
- W[x] S[ ] M[ ] F[ ] K[ ] ListApps
- W[ ] S[ ] M[ ] F[ ] K[ ] ListArtifacts
- W[ ] S[ ] M[ ] F[ ] K[ ] ListAssociations
- W[x] S[ ] M[x] F[ ] K[ ] ListAutoMLJobs
- W[ ] S[ ] M[ ] F[ ] K[ ] ListCandidatesForAutoMLJob
- W[ ] S[ ] M[ ] F[ ] K[ ] ListClusterEvents
- W[x] S[ ] M[x] F[ ] K[ ] ListClusterNodes
- W[ ] S[ ] M[ ] F[ ] K[ ] ListClusterSchedulerConfigs
- W[x] S[ ] M[x] F[ ] K[ ] ListClusters
- W[ ] S[ ] M[ ] F[ ] K[ ] ListCodeRepositories
- W[x] S[ ] M[x] F[ ] K[ ] ListCompilationJobs
- W[ ] S[ ] M[ ] F[ ] K[ ] ListComputeQuotas
- W[ ] S[ ] M[ ] F[ ] K[ ] ListContexts
- W[x] S[ ] M[x] F[ ] K[ ] ListDataQualityJobDefinitions
- W[ ] S[ ] M[ ] F[ ] K[ ] ListDeviceFleets
- W[ ] S[ ] M[ ] F[ ] K[ ] ListDevices
- W[x] S[ ] M[x] F[ ] K[ ] ListDomains
- W[ ] S[ ] M[ ] F[ ] K[ ] ListEdgeDeploymentPlans
- W[ ] S[ ] M[ ] F[ ] K[ ] ListEdgePackagingJobs
- W[x] S[ ] M[x] F[ ] K[ ] ListEndpointConfigs
- W[x] S[ ] M[x] F[ ] K[ ] ListEndpoints
- W[x] S[ ] M[x] F[ ] K[ ] ListExperiments
- W[x] S[ ] M[ ] F[ ] K[ ] ListFeatureGroups
- W[ ] S[ ] M[ ] F[ ] K[ ] ListFlowDefinitions
- W[ ] S[ ] M[ ] F[ ] K[ ] ListHubContentVersions
- W[ ] S[ ] M[ ] F[ ] K[ ] ListHubContents
- W[ ] S[ ] M[ ] F[ ] K[ ] ListHubs
- W[ ] S[ ] M[ ] F[ ] K[ ] ListHumanTaskUis
- W[x] S[ ] M[x] F[ ] K[ ] ListHyperParameterTuningJobs
- W[ ] S[ ] M[ ] F[ ] K[ ] ListImageVersions
- W[ ] S[ ] M[ ] F[ ] K[ ] ListImages
- W[ ] S[ ] M[ ] F[ ] K[ ] ListInferenceComponents
- W[ ] S[ ] M[ ] F[ ] K[ ] ListInferenceExperiments
- W[ ] S[ ] M[ ] F[ ] K[ ] ListInferenceRecommendationsJobSteps
- W[ ] S[ ] M[ ] F[ ] K[ ] ListInferenceRecommendationsJobs
- W[ ] S[ ] M[ ] F[ ] K[ ] ListLabelingJobs
- W[ ] S[ ] M[ ] F[ ] K[ ] ListLabelingJobsForWorkteam
- W[ ] S[ ] M[ ] F[ ] K[ ] ListLineageGroups
- W[ ] S[ ] M[ ] F[ ] K[ ] ListMlflowApps
- W[ ] S[ ] M[ ] F[ ] K[ ] ListMlflowTrackingServers
- W[x] S[ ] M[x] F[ ] K[ ] ListModelBiasJobDefinitions
- W[ ] S[ ] M[ ] F[ ] K[ ] ListModelCardExportJobs
- W[x] S[ ] M[x] F[ ] K[ ] ListModelCardVersions
- W[x] S[ ] M[x] F[ ] K[ ] ListModelCards
- W[x] S[ ] M[x] F[ ] K[ ] ListModelExplainabilityJobDefinitions
- W[ ] S[ ] M[ ] F[ ] K[ ] ListModelMetadata
- W[x] S[ ] M[x] F[ ] K[ ] ListModelPackageGroups
- W[x] S[ ] M[x] F[ ] K[ ] ListModelPackages
- W[x] S[ ] M[x] F[ ] K[ ] ListModelQualityJobDefinitions
- W[x] S[ ] M[x] F[ ] K[ ] ListModels
- W[ ] S[ ] M[ ] F[ ] K[ ] ListMonitoringAlertHistory
- W[ ] S[ ] M[ ] F[ ] K[ ] ListMonitoringAlerts
- W[ ] S[ ] M[ ] F[ ] K[ ] ListMonitoringExecutions
- W[ ] S[ ] M[ ] F[ ] K[ ] ListMonitoringSchedules
- W[x] S[ ] M[ ] F[ ] K[ ] ListNotebookInstanceLifecycleConfigs
- W[x] S[ ] M[x] F[ ] K[x] ListNotebookInstances
- W[ ] S[ ] M[ ] F[ ] K[ ] ListOptimizationJobs
- W[ ] S[ ] M[ ] F[ ] K[ ] ListPartnerApps
- W[ ] S[ ] M[ ] F[ ] K[ ] ListPipelineExecutionSteps
- W[x] S[ ] M[x] F[ ] K[ ] ListPipelineExecutions
- W[x] S[ ] M[x] F[ ] K[ ] ListPipelineParametersForExecution
- W[ ] S[ ] M[ ] F[ ] K[ ] ListPipelineVersions
- W[x] S[ ] M[x] F[ ] K[ ] ListPipelines
- W[x] S[ ] M[x] F[ ] K[ ] ListProcessingJobs
- W[ ] S[ ] M[ ] F[ ] K[ ] ListProjects
- W[ ] S[ ] M[ ] F[ ] K[ ] ListResourceCatalogs
- W[x] S[ ] M[ ] F[ ] K[ ] ListSpaces
- W[ ] S[ ] M[ ] F[ ] K[ ] ListStageDevices
- W[ ] S[ ] M[ ] F[ ] K[ ] ListStudioLifecycleConfigs
- W[ ] S[ ] M[ ] F[ ] K[ ] ListSubscribedWorkteams
- W[x] S[ ] M[x] F[ ] K[ ] ListTags
- W[x] S[ ] M[x] F[ ] K[ ] ListTrainingJobs
- W[ ] S[ ] M[ ] F[ ] K[ ] ListTrainingJobsForHyperParameterTuningJob
- W[ ] S[ ] M[ ] F[ ] K[ ] ListTrainingPlans
- W[x] S[ ] M[x] F[ ] K[ ] ListTransformJobs
- W[x] S[ ] M[x] F[ ] K[ ] ListTrialComponents
- W[x] S[ ] M[x] F[ ] K[ ] ListTrials
- W[ ] S[ ] M[ ] F[ ] K[ ] ListUltraServersByReservedCapacity
- W[x] S[ ] M[ ] F[ ] K[ ] ListUserProfiles
- W[ ] S[ ] M[ ] F[ ] K[ ] ListWorkforces
- W[ ] S[ ] M[ ] F[ ] K[ ] ListWorkteams
- W[ ] S[ ] M[ ] F[ ] K[ ] PutModelPackageGroupPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] QueryLineage
- W[ ] S[ ] M[ ] F[ ] K[ ] RegisterDevices
- W[ ] S[ ] M[ ] F[ ] K[ ] RenderUiTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] RetryPipelineExecution
- W[ ] S[x] M[x] F[ ] K[ ] Search
- W[ ] S[ ] M[ ] F[ ] K[ ] SearchTrainingPlanOfferings
- W[ ] S[ ] M[ ] F[ ] K[ ] SendPipelineExecutionStepFailure
- W[ ] S[ ] M[ ] F[ ] K[ ] SendPipelineExecutionStepSuccess
- W[ ] S[ ] M[ ] F[ ] K[ ] StartClusterHealthCheck
- W[ ] S[ ] M[ ] F[ ] K[ ] StartEdgeDeploymentStage
- W[ ] S[ ] M[ ] F[ ] K[ ] StartInferenceExperiment
- W[ ] S[ ] M[ ] F[ ] K[ ] StartMlflowTrackingServer
- W[ ] S[ ] M[ ] F[ ] K[ ] StartMonitoringSchedule
- W[x] S[ ] M[x] F[ ] K[ ] StartNotebookInstance
- W[x] S[ ] M[x] F[ ] K[ ] StartPipelineExecution
- W[ ] S[ ] M[ ] F[ ] K[ ] StartSession
- W[ ] S[ ] M[ ] F[ ] K[ ] StopAIBenchmarkJob
- W[ ] S[ ] M[ ] F[ ] K[ ] StopAIRecommendationJob
- W[x] S[ ] M[x] F[ ] K[ ] StopAutoMLJob
- W[ ] S[ ] M[ ] F[ ] K[ ] StopCompilationJob
- W[ ] S[ ] M[ ] F[ ] K[ ] StopEdgeDeploymentStage
- W[ ] S[ ] M[ ] F[ ] K[ ] StopEdgePackagingJob
- W[ ] S[ ] M[ ] F[ ] K[ ] StopHyperParameterTuningJob
- W[ ] S[ ] M[ ] F[ ] K[ ] StopInferenceExperiment
- W[ ] S[ ] M[ ] F[ ] K[ ] StopInferenceRecommendationsJob
- W[ ] S[ ] M[ ] F[ ] K[ ] StopLabelingJob
- W[ ] S[ ] M[ ] F[ ] K[ ] StopMlflowTrackingServer
- W[ ] S[ ] M[ ] F[ ] K[ ] StopMonitoringSchedule
- W[x] S[ ] M[x] F[ ] K[ ] StopNotebookInstance
- W[ ] S[ ] M[ ] F[ ] K[ ] StopOptimizationJob
- W[ ] S[ ] M[ ] F[ ] K[ ] StopPipelineExecution
- W[x] S[ ] M[ ] F[ ] K[ ] StopProcessingJob
- W[x] S[ ] M[ ] F[ ] K[ ] StopTrainingJob
- W[x] S[ ] M[ ] F[ ] K[ ] StopTransformJob
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateAction
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateAppImageConfig
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateArtifact
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateCluster
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateClusterSchedulerConfig
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateClusterSoftware
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateCodeRepository
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateComputeQuota
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateContext
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateDeviceFleet
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateDevices
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateDomain
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateEndpoint
- W[x] S[ ] M[x] F[ ] K[ ] UpdateEndpointWeightsAndCapacities
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateExperiment
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateFeatureGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateFeatureMetadata
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateHub
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateHubContent
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateHubContentReference
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateImage
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateImageVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateInferenceComponent
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateInferenceComponentRuntimeConfig
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateInferenceExperiment
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateMlflowApp
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateMlflowTrackingServer
- W[x] S[ ] M[x] F[ ] K[ ] UpdateModelCard
- W[x] S[ ] M[x] F[ ] K[ ] UpdateModelPackage
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateMonitoringAlert
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateMonitoringSchedule
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateNotebookInstance
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateNotebookInstanceLifecycleConfig
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdatePartnerApp
- W[x] S[ ] M[x] F[ ] K[ ] UpdatePipeline
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdatePipelineExecution
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdatePipelineVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateProject
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateSpace
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateTrainingJob
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateTrial
- W[x] S[ ] M[x] F[ ] K[ ] UpdateTrialComponent
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateUserProfile
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateWorkforce
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateWorkteam

Integration tests: 112/141 implemented operations tested (79.4%)
Untested implemented operations: 29

### winterbaume-sagemakermetrics (sagemaker-metrics) - W: 2/2, S: 0/2, M: 1/2, F: 0/2, K: 0/2

- W[x] S[ ] M[ ] F[ ] K[ ] BatchGetMetrics
- W[x] S[ ] M[x] F[ ] K[ ] BatchPutMetrics

Integration tests: 2/2 implemented operations tested (100.0%)

### winterbaume-sagemakerruntime (sagemaker-runtime) - W: 3/3, S: 0/3, M: 2/3, F: 0/3, K: 0/3

- W[x] S[ ] M[x] F[ ] K[ ] InvokeEndpoint
- W[x] S[ ] M[x] F[ ] K[ ] InvokeEndpointAsync
- W[x] S[ ] M[ ] F[ ] K[ ] InvokeEndpointWithResponseStream

Integration tests: 3/3 implemented operations tested (100.0%)

### winterbaume-savingsplans (savingsplans) - W: 10/10, S: 0/10, M: 0/10, F: 0/10, K: 0/10

- W[x] S[ ] M[ ] F[ ] K[ ] CreateSavingsPlan
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteQueuedSavingsPlan
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeSavingsPlanRates
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeSavingsPlans
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeSavingsPlansOfferingRates
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeSavingsPlansOfferings
- W[x] S[ ] M[ ] F[ ] K[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] ReturnSavingsPlan
- W[x] S[ ] M[ ] F[ ] K[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UntagResource

Integration tests: 8/10 implemented operations tested (80.0%)
Untested implemented operations: 2

### winterbaume-scheduler (scheduler) - W: 12/12, S: 0/12, M: 12/12, F: 0/12, K: 9/12

Terraform E2E: 7 tests across 2 terraform resource types

Resource types: aws_scheduler_schedule, aws_scheduler_schedule_group

- W[x] S[ ] M[x] F[ ] K[x] CreateSchedule
- W[x] S[ ] M[x] F[ ] K[x] CreateScheduleGroup
- W[x] S[ ] M[x] F[ ] K[x] DeleteSchedule
- W[x] S[ ] M[x] F[ ] K[x] DeleteScheduleGroup
- W[x] S[ ] M[x] F[ ] K[x] GetSchedule
- W[x] S[ ] M[x] F[ ] K[x] GetScheduleGroup
- W[x] S[ ] M[x] F[ ] K[x] ListScheduleGroups
- W[x] S[ ] M[x] F[ ] K[x] ListSchedules
- W[x] S[ ] M[x] F[ ] K[ ] ListTagsForResource
- W[x] S[ ] M[x] F[ ] K[ ] TagResource
- W[x] S[ ] M[x] F[ ] K[ ] UntagResource
- W[x] S[ ] M[x] F[ ] K[x] UpdateSchedule

Integration tests: 12/12 implemented operations tested (100.0%)

### winterbaume-secretsmanager (secrets-manager) - W: 22/23, S: 1/23, M: 21/23, F: 0/23, K: 11/23

Terraform E2E: 3 tests across 2 terraform resource types

Resource types: aws_secretsmanager_secret, aws_secretsmanager_secret_version

- W[x] S[ ] M[x] F[ ] K[ ] BatchGetSecretValue
- W[x] S[ ] M[x] F[ ] K[ ] CancelRotateSecret
- W[x] S[ ] M[x] F[ ] K[x] CreateSecret
- W[x] S[ ] M[x] F[ ] K[x] DeleteResourcePolicy
- W[x] S[ ] M[x] F[ ] K[x] DeleteSecret
- W[x] S[ ] M[x] F[ ] K[x] DescribeSecret
- W[x] S[ ] M[x] F[ ] K[x] GetRandomPassword
- W[x] S[ ] M[x] F[ ] K[x] GetResourcePolicy
- W[x] S[ ] M[x] F[ ] K[x] GetSecretValue
- W[x] S[ ] M[x] F[ ] K[ ] ListSecretVersionIds
- W[x] S[ ] M[x] F[ ] K[x] ListSecrets
- W[x] S[ ] M[x] F[ ] K[x] PutResourcePolicy
- W[x] S[ ] M[x] F[ ] K[x] PutSecretValue
- W[x] S[ ] M[x] F[ ] K[ ] RemoveRegionsFromReplication
- W[x] S[ ] M[x] F[ ] K[ ] ReplicateSecretToRegions
- W[x] S[ ] M[x] F[ ] K[ ] RestoreSecret
- W[x] S[ ] M[x] F[ ] K[ ] RotateSecret
- W[x] S[ ] M[ ] F[ ] K[ ] StopReplicationToReplica
- W[x] S[ ] M[x] F[ ] K[ ] TagResource
- W[x] S[ ] M[x] F[ ] K[ ] UntagResource
- W[x] S[ ] M[x] F[ ] K[x] UpdateSecret
- W[x] S[ ] M[x] F[ ] K[ ] UpdateSecretVersionStage
- W[ ] S[x] M[ ] F[ ] K[ ] ValidateResourcePolicy

Integration tests: 21/22 implemented operations tested (95.5%)
Untested implemented operations: 1

### winterbaume-securityhub (securityhub) - W: 97/107, S: 10/107, M: 13/107, F: 0/107, K: 0/107

- W[ ] S[x] M[ ] F[ ] K[ ] AcceptAdministratorInvitation
- W[ ] S[x] M[ ] F[ ] K[ ] AcceptInvitation
- W[x] S[ ] M[ ] F[ ] K[ ] BatchDeleteAutomationRules
- W[x] S[ ] M[ ] F[ ] K[ ] BatchDisableStandards
- W[x] S[ ] M[ ] F[ ] K[ ] BatchEnableStandards
- W[x] S[ ] M[ ] F[ ] K[ ] BatchGetAutomationRules
- W[x] S[ ] M[ ] F[ ] K[ ] BatchGetConfigurationPolicyAssociations
- W[x] S[ ] M[ ] F[ ] K[ ] BatchGetSecurityControls
- W[x] S[ ] M[ ] F[ ] K[ ] BatchGetStandardsControlAssociations
- W[x] S[ ] M[x] F[ ] K[ ] BatchImportFindings
- W[x] S[ ] M[ ] F[ ] K[ ] BatchUpdateAutomationRules
- W[x] S[ ] M[ ] F[ ] K[ ] BatchUpdateFindings
- W[x] S[ ] M[ ] F[ ] K[ ] BatchUpdateFindingsV2
- W[x] S[ ] M[ ] F[ ] K[ ] BatchUpdateStandardsControlAssociations
- W[x] S[ ] M[ ] F[ ] K[ ] CreateActionTarget
- W[x] S[ ] M[ ] F[ ] K[ ] CreateAggregatorV2
- W[x] S[ ] M[ ] F[ ] K[ ] CreateAutomationRule
- W[x] S[ ] M[ ] F[ ] K[ ] CreateAutomationRuleV2
- W[x] S[ ] M[ ] F[ ] K[ ] CreateConfigurationPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] CreateConnectorV2
- W[x] S[ ] M[ ] F[ ] K[ ] CreateFindingAggregator
- W[x] S[ ] M[ ] F[ ] K[ ] CreateInsight
- W[x] S[ ] M[x] F[ ] K[ ] CreateMembers
- W[x] S[ ] M[ ] F[ ] K[ ] CreateTicketV2
- W[ ] S[x] M[ ] F[ ] K[ ] DeclineInvitations
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteActionTarget
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteAggregatorV2
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteAutomationRuleV2
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteConfigurationPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteConnectorV2
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteFindingAggregator
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteInsight
- W[ ] S[x] M[ ] F[ ] K[ ] DeleteInvitations
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteMembers
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeActionTargets
- W[x] S[ ] M[x] F[ ] K[ ] DescribeHub
- W[x] S[ ] M[x] F[ ] K[ ] DescribeOrganizationConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeProducts
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeProductsV2
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeSecurityHubV2
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeStandards
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeStandardsControls
- W[x] S[ ] M[ ] F[ ] K[ ] DisableImportFindingsForProduct
- W[x] S[ ] M[ ] F[ ] K[ ] DisableOrganizationAdminAccount
- W[x] S[ ] M[x] F[ ] K[ ] DisableSecurityHub
- W[x] S[ ] M[ ] F[ ] K[ ] DisableSecurityHubV2
- W[ ] S[x] M[ ] F[ ] K[ ] DisassociateFromAdministratorAccount
- W[ ] S[x] M[ ] F[ ] K[ ] DisassociateFromMasterAccount
- W[ ] S[x] M[ ] F[ ] K[ ] DisassociateMembers
- W[x] S[ ] M[ ] F[ ] K[ ] EnableImportFindingsForProduct
- W[x] S[ ] M[x] F[ ] K[ ] EnableOrganizationAdminAccount
- W[x] S[ ] M[x] F[ ] K[ ] EnableSecurityHub
- W[x] S[ ] M[ ] F[ ] K[ ] EnableSecurityHubV2
- W[x] S[ ] M[x] F[ ] K[ ] GetAdministratorAccount
- W[x] S[ ] M[ ] F[ ] K[ ] GetAggregatorV2
- W[x] S[ ] M[ ] F[ ] K[ ] GetAutomationRuleV2
- W[x] S[ ] M[ ] F[ ] K[ ] GetConfigurationPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] GetConfigurationPolicyAssociation
- W[x] S[ ] M[ ] F[ ] K[ ] GetConnectorV2
- W[x] S[ ] M[ ] F[ ] K[ ] GetEnabledStandards
- W[x] S[ ] M[ ] F[ ] K[ ] GetFindingAggregator
- W[x] S[ ] M[ ] F[ ] K[ ] GetFindingHistory
- W[x] S[ ] M[ ] F[ ] K[ ] GetFindingStatisticsV2
- W[x] S[ ] M[x] F[ ] K[ ] GetFindings
- W[x] S[ ] M[ ] F[ ] K[ ] GetFindingsTrendsV2
- W[x] S[ ] M[ ] F[ ] K[ ] GetFindingsV2
- W[x] S[ ] M[ ] F[ ] K[ ] GetInsightResults
- W[x] S[ ] M[ ] F[ ] K[ ] GetInsights
- W[ ] S[x] M[ ] F[ ] K[ ] GetInvitationsCount
- W[x] S[ ] M[x] F[ ] K[ ] GetMasterAccount
- W[x] S[ ] M[x] F[ ] K[ ] GetMembers
- W[x] S[ ] M[ ] F[ ] K[ ] GetResourcesStatisticsV2
- W[x] S[ ] M[ ] F[ ] K[ ] GetResourcesTrendsV2
- W[x] S[ ] M[ ] F[ ] K[ ] GetResourcesV2
- W[x] S[ ] M[ ] F[ ] K[ ] GetSecurityControlDefinition
- W[ ] S[x] M[ ] F[ ] K[ ] InviteMembers
- W[x] S[ ] M[ ] F[ ] K[ ] ListAggregatorsV2
- W[x] S[ ] M[ ] F[ ] K[ ] ListAutomationRules
- W[x] S[ ] M[ ] F[ ] K[ ] ListAutomationRulesV2
- W[x] S[ ] M[ ] F[ ] K[ ] ListConfigurationPolicies
- W[x] S[ ] M[ ] F[ ] K[ ] ListConfigurationPolicyAssociations
- W[x] S[ ] M[ ] F[ ] K[ ] ListConnectorsV2
- W[x] S[ ] M[ ] F[ ] K[ ] ListEnabledProductsForImport
- W[x] S[ ] M[ ] F[ ] K[ ] ListFindingAggregators
- W[ ] S[x] M[ ] F[ ] K[ ] ListInvitations
- W[x] S[ ] M[x] F[ ] K[ ] ListMembers
- W[x] S[ ] M[ ] F[ ] K[ ] ListOrganizationAdminAccounts
- W[x] S[ ] M[ ] F[ ] K[ ] ListSecurityControlDefinitions
- W[x] S[ ] M[ ] F[ ] K[ ] ListStandardsControlAssociations
- W[x] S[ ] M[ ] F[ ] K[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] RegisterConnectorV2
- W[x] S[ ] M[ ] F[ ] K[ ] StartConfigurationPolicyAssociation
- W[x] S[ ] M[ ] F[ ] K[ ] StartConfigurationPolicyDisassociation
- W[x] S[ ] M[ ] F[ ] K[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateActionTarget
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateAggregatorV2
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateAutomationRuleV2
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateConfigurationPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateConnectorV2
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateFindingAggregator
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateFindings
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateInsight
- W[x] S[ ] M[x] F[ ] K[ ] UpdateOrganizationConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateSecurityControl
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateSecurityHubConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateStandardsControl

Integration tests: 94/97 implemented operations tested (96.9%)
Untested implemented operations: 3

### winterbaume-servicecatalog (service-catalog) - W: 4/90, S: 0/90, M: 0/90, F: 0/90, K: 0/90

- W[ ] S[ ] M[ ] F[ ] K[ ] AcceptPortfolioShare
- W[ ] S[ ] M[ ] F[ ] K[ ] AssociateBudgetWithResource
- W[ ] S[ ] M[ ] F[ ] K[ ] AssociatePrincipalWithPortfolio
- W[ ] S[ ] M[ ] F[ ] K[ ] AssociateProductWithPortfolio
- W[ ] S[ ] M[ ] F[ ] K[ ] AssociateServiceActionWithProvisioningArtifact
- W[ ] S[ ] M[ ] F[ ] K[ ] AssociateTagOptionWithResource
- W[ ] S[ ] M[ ] F[ ] K[ ] BatchAssociateServiceActionWithProvisioningArtifact
- W[ ] S[ ] M[ ] F[ ] K[ ] BatchDisassociateServiceActionFromProvisioningArtifact
- W[ ] S[ ] M[ ] F[ ] K[ ] CopyProduct
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateConstraint
- W[x] S[ ] M[ ] F[ ] K[ ] CreatePortfolio
- W[ ] S[ ] M[ ] F[ ] K[ ] CreatePortfolioShare
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateProduct
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateProvisionedProductPlan
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateProvisioningArtifact
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateServiceAction
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateTagOption
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteConstraint
- W[x] S[ ] M[ ] F[ ] K[ ] DeletePortfolio
- W[ ] S[ ] M[ ] F[ ] K[ ] DeletePortfolioShare
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteProduct
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteProvisionedProductPlan
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteProvisioningArtifact
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteServiceAction
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteTagOption
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeConstraint
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeCopyProductStatus
- W[x] S[ ] M[ ] F[ ] K[ ] DescribePortfolio
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribePortfolioShareStatus
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribePortfolioShares
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeProduct
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeProductAsAdmin
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeProductView
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeProvisionedProduct
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeProvisionedProductPlan
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeProvisioningArtifact
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeProvisioningParameters
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeRecord
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeServiceAction
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeServiceActionExecutionParameters
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeTagOption
- W[ ] S[ ] M[ ] F[ ] K[ ] DisableAWSOrganizationsAccess
- W[ ] S[ ] M[ ] F[ ] K[ ] DisassociateBudgetFromResource
- W[ ] S[ ] M[ ] F[ ] K[ ] DisassociatePrincipalFromPortfolio
- W[ ] S[ ] M[ ] F[ ] K[ ] DisassociateProductFromPortfolio
- W[ ] S[ ] M[ ] F[ ] K[ ] DisassociateServiceActionFromProvisioningArtifact
- W[ ] S[ ] M[ ] F[ ] K[ ] DisassociateTagOptionFromResource
- W[ ] S[ ] M[ ] F[ ] K[ ] EnableAWSOrganizationsAccess
- W[ ] S[ ] M[ ] F[ ] K[ ] ExecuteProvisionedProductPlan
- W[ ] S[ ] M[ ] F[ ] K[ ] ExecuteProvisionedProductServiceAction
- W[ ] S[ ] M[ ] F[ ] K[ ] GetAWSOrganizationsAccessStatus
- W[ ] S[ ] M[ ] F[ ] K[ ] GetProvisionedProductOutputs
- W[ ] S[ ] M[ ] F[ ] K[ ] ImportAsProvisionedProduct
- W[ ] S[ ] M[ ] F[ ] K[ ] ListAcceptedPortfolioShares
- W[ ] S[ ] M[ ] F[ ] K[ ] ListBudgetsForResource
- W[ ] S[ ] M[ ] F[ ] K[ ] ListConstraintsForPortfolio
- W[ ] S[ ] M[ ] F[ ] K[ ] ListLaunchPaths
- W[ ] S[ ] M[ ] F[ ] K[ ] ListOrganizationPortfolioAccess
- W[ ] S[ ] M[ ] F[ ] K[ ] ListPortfolioAccess
- W[x] S[ ] M[ ] F[ ] K[ ] ListPortfolios
- W[ ] S[ ] M[ ] F[ ] K[ ] ListPortfoliosForProduct
- W[ ] S[ ] M[ ] F[ ] K[ ] ListPrincipalsForPortfolio
- W[ ] S[ ] M[ ] F[ ] K[ ] ListProvisionedProductPlans
- W[ ] S[ ] M[ ] F[ ] K[ ] ListProvisioningArtifacts
- W[ ] S[ ] M[ ] F[ ] K[ ] ListProvisioningArtifactsForServiceAction
- W[ ] S[ ] M[ ] F[ ] K[ ] ListRecordHistory
- W[ ] S[ ] M[ ] F[ ] K[ ] ListResourcesForTagOption
- W[ ] S[ ] M[ ] F[ ] K[ ] ListServiceActions
- W[ ] S[ ] M[ ] F[ ] K[ ] ListServiceActionsForProvisioningArtifact
- W[ ] S[ ] M[ ] F[ ] K[ ] ListStackInstancesForProvisionedProduct
- W[ ] S[ ] M[ ] F[ ] K[ ] ListTagOptions
- W[ ] S[ ] M[ ] F[ ] K[ ] NotifyProvisionProductEngineWorkflowResult
- W[ ] S[ ] M[ ] F[ ] K[ ] NotifyTerminateProvisionedProductEngineWorkflowResult
- W[ ] S[ ] M[ ] F[ ] K[ ] NotifyUpdateProvisionedProductEngineWorkflowResult
- W[ ] S[ ] M[ ] F[ ] K[ ] ProvisionProduct
- W[ ] S[ ] M[ ] F[ ] K[ ] RejectPortfolioShare
- W[ ] S[ ] M[ ] F[ ] K[ ] ScanProvisionedProducts
- W[ ] S[ ] M[ ] F[ ] K[ ] SearchProducts
- W[ ] S[ ] M[ ] F[ ] K[ ] SearchProductsAsAdmin
- W[ ] S[ ] M[ ] F[ ] K[ ] SearchProvisionedProducts
- W[ ] S[ ] M[ ] F[ ] K[ ] TerminateProvisionedProduct
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateConstraint
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdatePortfolio
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdatePortfolioShare
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateProduct
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateProvisionedProduct
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateProvisionedProductProperties
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateProvisioningArtifact
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateServiceAction
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateTagOption

Integration tests: 4/4 implemented operations tested (100.0%)

### winterbaume-servicecatalogappregistry (service-catalog-appregistry) - W: 23/24, S: 1/24, M: 0/24, F: 0/24, K: 0/24

- W[x] S[ ] M[ ] F[ ] K[ ] AssociateAttributeGroup
- W[x] S[ ] M[ ] F[ ] K[ ] AssociateResource
- W[x] S[ ] M[ ] F[ ] K[ ] CreateApplication
- W[x] S[ ] M[ ] F[ ] K[ ] CreateAttributeGroup
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteApplication
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteAttributeGroup
- W[x] S[ ] M[ ] F[ ] K[ ] DisassociateAttributeGroup
- W[x] S[ ] M[ ] F[ ] K[ ] DisassociateResource
- W[x] S[ ] M[ ] F[ ] K[ ] GetApplication
- W[x] S[ ] M[ ] F[ ] K[ ] GetAssociatedResource
- W[x] S[ ] M[ ] F[ ] K[ ] GetAttributeGroup
- W[x] S[ ] M[ ] F[ ] K[ ] GetConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] ListApplications
- W[x] S[ ] M[ ] F[ ] K[ ] ListAssociatedAttributeGroups
- W[x] S[ ] M[ ] F[ ] K[ ] ListAssociatedResources
- W[x] S[ ] M[ ] F[ ] K[ ] ListAttributeGroups
- W[x] S[ ] M[ ] F[ ] K[ ] ListAttributeGroupsForApplication
- W[x] S[ ] M[ ] F[ ] K[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] PutConfiguration
- W[ ] S[x] M[ ] F[ ] K[ ] SyncResource
- W[x] S[ ] M[ ] F[ ] K[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateApplication
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateAttributeGroup

Integration tests: 23/23 implemented operations tested (100.0%)

### winterbaume-servicediscovery (servicediscovery) - W: 27/30, S: 0/30, M: 27/30, F: 0/30, K: 0/30

- W[x] S[ ] M[x] F[ ] K[ ] CreateHttpNamespace
- W[x] S[ ] M[x] F[ ] K[ ] CreatePrivateDnsNamespace
- W[x] S[ ] M[x] F[ ] K[ ] CreatePublicDnsNamespace
- W[x] S[ ] M[x] F[ ] K[ ] CreateService
- W[x] S[ ] M[x] F[ ] K[ ] DeleteNamespace
- W[x] S[ ] M[x] F[ ] K[ ] DeleteService
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteServiceAttributes
- W[x] S[ ] M[x] F[ ] K[ ] DeregisterInstance
- W[x] S[ ] M[x] F[ ] K[ ] DiscoverInstances
- W[x] S[ ] M[x] F[ ] K[ ] DiscoverInstancesRevision
- W[x] S[ ] M[x] F[ ] K[ ] GetInstance
- W[x] S[ ] M[x] F[ ] K[ ] GetInstancesHealthStatus
- W[x] S[ ] M[x] F[ ] K[ ] GetNamespace
- W[x] S[ ] M[x] F[ ] K[ ] GetOperation
- W[x] S[ ] M[x] F[ ] K[ ] GetService
- W[ ] S[ ] M[ ] F[ ] K[ ] GetServiceAttributes
- W[x] S[ ] M[x] F[ ] K[ ] ListInstances
- W[x] S[ ] M[x] F[ ] K[ ] ListNamespaces
- W[x] S[ ] M[x] F[ ] K[ ] ListOperations
- W[x] S[ ] M[x] F[ ] K[ ] ListServices
- W[x] S[ ] M[x] F[ ] K[ ] ListTagsForResource
- W[x] S[ ] M[x] F[ ] K[ ] RegisterInstance
- W[x] S[ ] M[x] F[ ] K[ ] TagResource
- W[x] S[ ] M[x] F[ ] K[ ] UntagResource
- W[x] S[ ] M[x] F[ ] K[ ] UpdateHttpNamespace
- W[x] S[ ] M[x] F[ ] K[ ] UpdateInstanceCustomHealthStatus
- W[x] S[ ] M[x] F[ ] K[ ] UpdatePrivateDnsNamespace
- W[x] S[ ] M[x] F[ ] K[ ] UpdatePublicDnsNamespace
- W[x] S[ ] M[x] F[ ] K[ ] UpdateService
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateServiceAttributes

Integration tests: 27/27 implemented operations tested (100.0%)

### winterbaume-servicequotas (service-quotas) - W: 5/26, S: 0/26, M: 2/26, F: 0/26, K: 8/26

- W[ ] S[ ] M[ ] F[ ] K[ ] AssociateServiceQuotaTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateSupportCase
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteServiceQuotaIncreaseRequestFromTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] DisassociateServiceQuotaTemplate
- W[x] S[ ] M[ ] F[ ] K[x] GetAWSDefaultServiceQuota
- W[ ] S[ ] M[ ] F[ ] K[ ] GetAssociationForServiceQuotaTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] GetAutoManagementConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] GetQuotaUtilizationReport
- W[ ] S[ ] M[ ] F[ ] K[x] GetRequestedServiceQuotaChange
- W[x] S[ ] M[x] F[ ] K[x] GetServiceQuota
- W[ ] S[ ] M[ ] F[ ] K[ ] GetServiceQuotaIncreaseRequestFromTemplate
- W[x] S[ ] M[x] F[ ] K[x] ListAWSDefaultServiceQuotas
- W[ ] S[ ] M[ ] F[ ] K[x] ListRequestedServiceQuotaChangeHistory
- W[ ] S[ ] M[ ] F[ ] K[ ] ListRequestedServiceQuotaChangeHistoryByQuota
- W[ ] S[ ] M[ ] F[ ] K[ ] ListServiceQuotaIncreaseRequestsInTemplate
- W[x] S[ ] M[ ] F[ ] K[x] ListServiceQuotas
- W[x] S[ ] M[ ] F[ ] K[x] ListServices
- W[ ] S[ ] M[ ] F[ ] K[ ] ListTagsForResource
- W[ ] S[ ] M[ ] F[ ] K[ ] PutServiceQuotaIncreaseRequestIntoTemplate
- W[ ] S[ ] M[ ] F[ ] K[x] RequestServiceQuotaIncrease
- W[ ] S[ ] M[ ] F[ ] K[ ] StartAutoManagement
- W[ ] S[ ] M[ ] F[ ] K[ ] StartQuotaUtilizationReport
- W[ ] S[ ] M[ ] F[ ] K[ ] StopAutoManagement
- W[ ] S[ ] M[ ] F[ ] K[ ] TagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] UntagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateAutoManagement

Integration tests: 5/5 implemented operations tested (100.0%)

### winterbaume-ses (ses) - W: 38/71, S: 2/71, M: 38/71, F: 0/71, K: 0/71

- W[x] S[ ] M[x] F[ ] K[ ] CloneReceiptRuleSet
- W[x] S[ ] M[x] F[ ] K[ ] CreateConfigurationSet
- W[x] S[ ] M[x] F[ ] K[ ] CreateConfigurationSetEventDestination
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateConfigurationSetTrackingOptions
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateCustomVerificationEmailTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateReceiptFilter
- W[x] S[ ] M[x] F[ ] K[ ] CreateReceiptRule
- W[x] S[ ] M[x] F[ ] K[ ] CreateReceiptRuleSet
- W[x] S[ ] M[ ] F[ ] K[ ] CreateTemplate
- W[x] S[ ] M[x] F[ ] K[ ] DeleteConfigurationSet
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteConfigurationSetEventDestination
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteConfigurationSetTrackingOptions
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteCustomVerificationEmailTemplate
- W[x] S[ ] M[x] F[ ] K[ ] DeleteIdentity
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteIdentityPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteReceiptFilter
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteReceiptRule
- W[x] S[ ] M[x] F[ ] K[ ] DeleteReceiptRuleSet
- W[x] S[ ] M[x] F[ ] K[ ] DeleteTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteVerifiedEmailAddress
- W[x] S[ ] M[x] F[ ] K[ ] DescribeActiveReceiptRuleSet
- W[x] S[ ] M[x] F[ ] K[ ] DescribeConfigurationSet
- W[x] S[ ] M[x] F[ ] K[ ] DescribeReceiptRule
- W[x] S[ ] M[x] F[ ] K[ ] DescribeReceiptRuleSet
- W[ ] S[ ] M[ ] F[ ] K[ ] GetAccountSendingEnabled
- W[ ] S[ ] M[ ] F[ ] K[ ] GetCustomVerificationEmailTemplate
- W[x] S[ ] M[x] F[ ] K[ ] GetIdentityDkimAttributes
- W[x] S[ ] M[x] F[ ] K[ ] GetIdentityMailFromDomainAttributes
- W[x] S[ ] M[x] F[ ] K[ ] GetIdentityNotificationAttributes
- W[ ] S[ ] M[ ] F[ ] K[ ] GetIdentityPolicies
- W[x] S[ ] M[x] F[ ] K[ ] GetIdentityVerificationAttributes
- W[ ] S[x] M[x] F[ ] K[ ] GetSendQuota
- W[ ] S[x] M[x] F[ ] K[ ] GetSendStatistics
- W[x] S[ ] M[x] F[ ] K[ ] GetTemplate
- W[x] S[ ] M[x] F[ ] K[ ] ListConfigurationSets
- W[ ] S[ ] M[ ] F[ ] K[ ] ListCustomVerificationEmailTemplates
- W[x] S[ ] M[x] F[ ] K[ ] ListIdentities
- W[ ] S[ ] M[ ] F[ ] K[ ] ListIdentityPolicies
- W[ ] S[ ] M[ ] F[ ] K[ ] ListReceiptFilters
- W[x] S[ ] M[x] F[ ] K[ ] ListReceiptRuleSets
- W[x] S[ ] M[x] F[ ] K[ ] ListTemplates
- W[x] S[ ] M[x] F[ ] K[ ] ListVerifiedEmailAddresses
- W[ ] S[ ] M[ ] F[ ] K[ ] PutConfigurationSetDeliveryOptions
- W[ ] S[ ] M[ ] F[ ] K[ ] PutIdentityPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] ReorderReceiptRuleSet
- W[ ] S[ ] M[ ] F[ ] K[ ] SendBounce
- W[x] S[ ] M[x] F[ ] K[ ] SendBulkTemplatedEmail
- W[ ] S[ ] M[ ] F[ ] K[ ] SendCustomVerificationEmail
- W[x] S[ ] M[x] F[ ] K[ ] SendEmail
- W[x] S[ ] M[x] F[ ] K[ ] SendRawEmail
- W[x] S[ ] M[x] F[ ] K[ ] SendTemplatedEmail
- W[x] S[ ] M[x] F[ ] K[ ] SetActiveReceiptRuleSet
- W[ ] S[ ] M[ ] F[ ] K[ ] SetIdentityDkimEnabled
- W[x] S[ ] M[x] F[ ] K[ ] SetIdentityFeedbackForwardingEnabled
- W[ ] S[ ] M[ ] F[ ] K[ ] SetIdentityHeadersInNotificationsEnabled
- W[x] S[ ] M[x] F[ ] K[ ] SetIdentityMailFromDomain
- W[x] S[ ] M[x] F[ ] K[ ] SetIdentityNotificationTopic
- W[ ] S[ ] M[ ] F[ ] K[ ] SetReceiptRulePosition
- W[ ] S[ ] M[ ] F[ ] K[ ] TestRenderTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateAccountSendingEnabled
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateConfigurationSetEventDestination
- W[x] S[ ] M[x] F[ ] K[ ] UpdateConfigurationSetReputationMetricsEnabled
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateConfigurationSetSendingEnabled
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateConfigurationSetTrackingOptions
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateCustomVerificationEmailTemplate
- W[x] S[ ] M[x] F[ ] K[ ] UpdateReceiptRule
- W[x] S[ ] M[x] F[ ] K[ ] UpdateTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] VerifyDomainDkim
- W[x] S[ ] M[ ] F[ ] K[ ] VerifyDomainIdentity
- W[x] S[ ] M[x] F[ ] K[ ] VerifyEmailAddress
- W[x] S[ ] M[x] F[ ] K[ ] VerifyEmailIdentity

Integration tests: 28/38 implemented operations tested (73.7%)
Untested implemented operations: 10

### winterbaume-sesv2 (sesv2) - W: 106/110, S: 4/110, M: 28/110, F: 0/110, K: 9/110

- W[ ] S[x] M[ ] F[ ] K[ ] BatchGetMetricData
- W[x] S[ ] M[ ] F[ ] K[ ] CancelExportJob
- W[x] S[ ] M[x] F[ ] K[x] CreateConfigurationSet
- W[x] S[ ] M[ ] F[ ] K[ ] CreateConfigurationSetEventDestination
- W[x] S[ ] M[x] F[ ] K[ ] CreateContact
- W[x] S[ ] M[x] F[ ] K[ ] CreateContactList
- W[x] S[ ] M[ ] F[ ] K[ ] CreateCustomVerificationEmailTemplate
- W[x] S[ ] M[x] F[ ] K[ ] CreateDedicatedIpPool
- W[x] S[ ] M[ ] F[ ] K[ ] CreateDeliverabilityTestReport
- W[x] S[ ] M[x] F[ ] K[x] CreateEmailIdentity
- W[x] S[ ] M[x] F[ ] K[ ] CreateEmailIdentityPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] CreateEmailTemplate
- W[x] S[ ] M[ ] F[ ] K[ ] CreateExportJob
- W[x] S[ ] M[ ] F[ ] K[ ] CreateImportJob
- W[x] S[ ] M[ ] F[ ] K[ ] CreateMultiRegionEndpoint
- W[x] S[ ] M[ ] F[ ] K[ ] CreateTenant
- W[x] S[ ] M[ ] F[ ] K[ ] CreateTenantResourceAssociation
- W[x] S[ ] M[x] F[ ] K[x] DeleteConfigurationSet
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteConfigurationSetEventDestination
- W[x] S[ ] M[x] F[ ] K[ ] DeleteContact
- W[x] S[ ] M[x] F[ ] K[ ] DeleteContactList
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteCustomVerificationEmailTemplate
- W[x] S[ ] M[x] F[ ] K[ ] DeleteDedicatedIpPool
- W[x] S[ ] M[x] F[ ] K[x] DeleteEmailIdentity
- W[x] S[ ] M[x] F[ ] K[ ] DeleteEmailIdentityPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteEmailTemplate
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteMultiRegionEndpoint
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteSuppressedDestination
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteTenant
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteTenantResourceAssociation
- W[x] S[ ] M[ ] F[ ] K[ ] GetAccount
- W[x] S[ ] M[ ] F[ ] K[ ] GetBlacklistReports
- W[x] S[ ] M[x] F[ ] K[x] GetConfigurationSet
- W[x] S[ ] M[ ] F[ ] K[ ] GetConfigurationSetEventDestinations
- W[x] S[ ] M[x] F[ ] K[ ] GetContact
- W[x] S[ ] M[x] F[ ] K[ ] GetContactList
- W[x] S[ ] M[ ] F[ ] K[ ] GetCustomVerificationEmailTemplate
- W[x] S[ ] M[ ] F[ ] K[ ] GetDedicatedIp
- W[x] S[ ] M[x] F[ ] K[ ] GetDedicatedIpPool
- W[x] S[ ] M[ ] F[ ] K[ ] GetDedicatedIps
- W[x] S[ ] M[ ] F[ ] K[ ] GetDeliverabilityDashboardOptions
- W[x] S[ ] M[ ] F[ ] K[ ] GetDeliverabilityTestReport
- W[x] S[ ] M[ ] F[ ] K[ ] GetDomainDeliverabilityCampaign
- W[ ] S[x] M[ ] F[ ] K[ ] GetDomainStatisticsReport
- W[ ] S[x] M[ ] F[ ] K[ ] GetEmailAddressInsights
- W[x] S[ ] M[x] F[ ] K[x] GetEmailIdentity
- W[x] S[ ] M[x] F[ ] K[ ] GetEmailIdentityPolicies
- W[x] S[ ] M[ ] F[ ] K[ ] GetEmailTemplate
- W[x] S[ ] M[ ] F[ ] K[ ] GetExportJob
- W[x] S[ ] M[ ] F[ ] K[ ] GetImportJob
- W[x] S[ ] M[ ] F[ ] K[ ] GetMessageInsights
- W[x] S[ ] M[ ] F[ ] K[ ] GetMultiRegionEndpoint
- W[x] S[ ] M[ ] F[ ] K[ ] GetReputationEntity
- W[x] S[ ] M[ ] F[ ] K[ ] GetSuppressedDestination
- W[x] S[ ] M[ ] F[ ] K[ ] GetTenant
- W[x] S[ ] M[x] F[ ] K[x] ListConfigurationSets
- W[x] S[ ] M[x] F[ ] K[ ] ListContactLists
- W[x] S[ ] M[x] F[ ] K[ ] ListContacts
- W[x] S[ ] M[ ] F[ ] K[ ] ListCustomVerificationEmailTemplates
- W[x] S[ ] M[x] F[ ] K[ ] ListDedicatedIpPools
- W[x] S[ ] M[ ] F[ ] K[ ] ListDeliverabilityTestReports
- W[x] S[ ] M[ ] F[ ] K[ ] ListDomainDeliverabilityCampaigns
- W[x] S[ ] M[x] F[ ] K[x] ListEmailIdentities
- W[x] S[ ] M[ ] F[ ] K[ ] ListEmailTemplates
- W[x] S[ ] M[ ] F[ ] K[ ] ListExportJobs
- W[x] S[ ] M[ ] F[ ] K[ ] ListImportJobs
- W[x] S[ ] M[ ] F[ ] K[ ] ListMultiRegionEndpoints
- W[ ] S[x] M[ ] F[ ] K[ ] ListRecommendations
- W[x] S[ ] M[ ] F[ ] K[ ] ListReputationEntities
- W[x] S[ ] M[ ] F[ ] K[ ] ListResourceTenants
- W[x] S[ ] M[ ] F[ ] K[ ] ListSuppressedDestinations
- W[x] S[ ] M[x] F[ ] K[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] ListTenantResources
- W[x] S[ ] M[ ] F[ ] K[ ] ListTenants
- W[x] S[ ] M[ ] F[ ] K[ ] PutAccountDedicatedIpWarmupAttributes
- W[x] S[ ] M[ ] F[ ] K[ ] PutAccountDetails
- W[x] S[ ] M[ ] F[ ] K[ ] PutAccountSendingAttributes
- W[x] S[ ] M[ ] F[ ] K[ ] PutAccountSuppressionAttributes
- W[x] S[ ] M[ ] F[ ] K[ ] PutAccountVdmAttributes
- W[x] S[ ] M[ ] F[ ] K[ ] PutConfigurationSetArchivingOptions
- W[x] S[ ] M[ ] F[ ] K[ ] PutConfigurationSetDeliveryOptions
- W[x] S[ ] M[ ] F[ ] K[ ] PutConfigurationSetReputationOptions
- W[x] S[ ] M[ ] F[ ] K[ ] PutConfigurationSetSendingOptions
- W[x] S[ ] M[ ] F[ ] K[ ] PutConfigurationSetSuppressionOptions
- W[x] S[ ] M[ ] F[ ] K[ ] PutConfigurationSetTrackingOptions
- W[x] S[ ] M[ ] F[ ] K[ ] PutConfigurationSetVdmOptions
- W[x] S[ ] M[ ] F[ ] K[ ] PutDedicatedIpInPool
- W[x] S[ ] M[ ] F[ ] K[ ] PutDedicatedIpPoolScalingAttributes
- W[x] S[ ] M[ ] F[ ] K[ ] PutDedicatedIpWarmupAttributes
- W[x] S[ ] M[ ] F[ ] K[ ] PutDeliverabilityDashboardOption
- W[x] S[ ] M[ ] F[ ] K[ ] PutEmailIdentityConfigurationSetAttributes
- W[x] S[ ] M[ ] F[ ] K[ ] PutEmailIdentityDkimAttributes
- W[x] S[ ] M[ ] F[ ] K[ ] PutEmailIdentityDkimSigningAttributes
- W[x] S[ ] M[ ] F[ ] K[ ] PutEmailIdentityFeedbackAttributes
- W[x] S[ ] M[ ] F[ ] K[ ] PutEmailIdentityMailFromAttributes
- W[x] S[ ] M[ ] F[ ] K[ ] PutSuppressedDestination
- W[x] S[ ] M[ ] F[ ] K[ ] SendBulkEmail
- W[x] S[ ] M[ ] F[ ] K[ ] SendCustomVerificationEmail
- W[x] S[ ] M[x] F[ ] K[x] SendEmail
- W[x] S[ ] M[x] F[ ] K[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] TestRenderEmailTemplate
- W[x] S[ ] M[x] F[ ] K[ ] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateConfigurationSetEventDestination
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateContact
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateContactList
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateCustomVerificationEmailTemplate
- W[x] S[ ] M[x] F[ ] K[ ] UpdateEmailIdentityPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateEmailTemplate
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateReputationEntityCustomerManagedStatus
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateReputationEntityPolicy

Integration tests: 91/106 implemented operations tested (85.8%)
Untested implemented operations: 15

### winterbaume-sfn (sfn) - W: 35/37, S: 2/37, M: 29/37, F: 0/37, K: 18/37

Terraform E2E: 10 tests across 3 terraform resource types

Resource types: aws_sfn_activity, aws_sfn_alias, aws_sfn_state_machine

- W[x] S[ ] M[x] F[ ] K[ ] CreateActivity
- W[x] S[ ] M[x] F[ ] K[x] CreateStateMachine
- W[x] S[ ] M[x] F[ ] K[ ] CreateStateMachineAlias
- W[x] S[ ] M[x] F[ ] K[ ] DeleteActivity
- W[x] S[ ] M[x] F[ ] K[x] DeleteStateMachine
- W[x] S[ ] M[x] F[ ] K[ ] DeleteStateMachineAlias
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteStateMachineVersion
- W[x] S[ ] M[x] F[ ] K[ ] DescribeActivity
- W[x] S[ ] M[x] F[ ] K[x] DescribeExecution
- W[x] S[ ] M[x] F[ ] K[ ] DescribeMapRun
- W[x] S[ ] M[x] F[ ] K[x] DescribeStateMachine
- W[x] S[ ] M[x] F[ ] K[ ] DescribeStateMachineAlias
- W[x] S[ ] M[x] F[ ] K[ ] DescribeStateMachineForExecution
- W[x] S[ ] M[ ] F[ ] K[ ] GetActivityTask
- W[x] S[ ] M[x] F[ ] K[x] GetExecutionHistory
- W[x] S[ ] M[x] F[ ] K[ ] ListActivities
- W[x] S[ ] M[x] F[ ] K[x] ListExecutions
- W[x] S[ ] M[x] F[ ] K[ ] ListMapRuns
- W[x] S[ ] M[x] F[ ] K[x] ListStateMachineAliases
- W[x] S[ ] M[ ] F[ ] K[x] ListStateMachineVersions
- W[x] S[ ] M[x] F[ ] K[x] ListStateMachines
- W[x] S[ ] M[x] F[ ] K[x] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] PublishStateMachineVersion
- W[x] S[ ] M[ ] F[ ] K[ ] RedriveExecution
- W[x] S[ ] M[x] F[ ] K[x] SendTaskFailure
- W[x] S[ ] M[x] F[ ] K[x] SendTaskHeartbeat
- W[x] S[ ] M[x] F[ ] K[x] SendTaskSuccess
- W[x] S[ ] M[x] F[ ] K[x] StartExecution
- W[ ] S[x] M[ ] F[ ] K[ ] StartSyncExecution
- W[x] S[ ] M[x] F[ ] K[x] StopExecution
- W[x] S[ ] M[x] F[ ] K[x] TagResource
- W[ ] S[x] M[ ] F[ ] K[ ] TestState
- W[x] S[ ] M[x] F[ ] K[x] UntagResource
- W[x] S[ ] M[x] F[ ] K[ ] UpdateMapRun
- W[x] S[ ] M[x] F[ ] K[ ] UpdateStateMachine
- W[x] S[ ] M[x] F[ ] K[ ] UpdateStateMachineAlias
- W[x] S[ ] M[ ] F[ ] K[x] ValidateStateMachineDefinition

Integration tests: 35/35 implemented operations tested (100.0%)

### winterbaume-shield (shield) - W: 9/36, S: 0/36, M: 9/36, F: 0/36, K: 0/36

- W[ ] S[ ] M[ ] F[ ] K[ ] AssociateDRTLogBucket
- W[ ] S[ ] M[ ] F[ ] K[ ] AssociateDRTRole
- W[ ] S[ ] M[ ] F[ ] K[ ] AssociateHealthCheck
- W[ ] S[ ] M[ ] F[ ] K[ ] AssociateProactiveEngagementDetails
- W[x] S[ ] M[x] F[ ] K[ ] CreateProtection
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateProtectionGroup
- W[x] S[ ] M[x] F[ ] K[ ] CreateSubscription
- W[x] S[ ] M[x] F[ ] K[ ] DeleteProtection
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteProtectionGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteSubscription
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeAttack
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeAttackStatistics
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeDRTAccess
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeEmergencyContactSettings
- W[x] S[ ] M[x] F[ ] K[ ] DescribeProtection
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeProtectionGroup
- W[x] S[ ] M[x] F[ ] K[ ] DescribeSubscription
- W[ ] S[ ] M[ ] F[ ] K[ ] DisableApplicationLayerAutomaticResponse
- W[ ] S[ ] M[ ] F[ ] K[ ] DisableProactiveEngagement
- W[ ] S[ ] M[ ] F[ ] K[ ] DisassociateDRTLogBucket
- W[ ] S[ ] M[ ] F[ ] K[ ] DisassociateDRTRole
- W[ ] S[ ] M[ ] F[ ] K[ ] DisassociateHealthCheck
- W[ ] S[ ] M[ ] F[ ] K[ ] EnableApplicationLayerAutomaticResponse
- W[ ] S[ ] M[ ] F[ ] K[ ] EnableProactiveEngagement
- W[ ] S[ ] M[ ] F[ ] K[ ] GetSubscriptionState
- W[ ] S[ ] M[ ] F[ ] K[ ] ListAttacks
- W[ ] S[ ] M[ ] F[ ] K[ ] ListProtectionGroups
- W[x] S[ ] M[x] F[ ] K[ ] ListProtections
- W[ ] S[ ] M[ ] F[ ] K[ ] ListResourcesInProtectionGroup
- W[x] S[ ] M[x] F[ ] K[ ] ListTagsForResource
- W[x] S[ ] M[x] F[ ] K[ ] TagResource
- W[x] S[ ] M[x] F[ ] K[ ] UntagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateApplicationLayerAutomaticResponse
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateEmergencyContactSettings
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateProtectionGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateSubscription

Integration tests: 9/9 implemented operations tested (100.0%)

### winterbaume-signer (signer) - W: 19/19, S: 0/19, M: 7/19, F: 0/19, K: 0/19

- W[x] S[ ] M[ ] F[ ] K[ ] AddProfilePermission
- W[x] S[ ] M[x] F[ ] K[ ] CancelSigningProfile
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeSigningJob
- W[x] S[ ] M[ ] F[ ] K[ ] GetRevocationStatus
- W[x] S[ ] M[ ] F[ ] K[ ] GetSigningPlatform
- W[x] S[ ] M[x] F[ ] K[ ] GetSigningProfile
- W[x] S[ ] M[ ] F[ ] K[ ] ListProfilePermissions
- W[x] S[ ] M[ ] F[ ] K[ ] ListSigningJobs
- W[x] S[ ] M[x] F[ ] K[ ] ListSigningPlatforms
- W[x] S[ ] M[ ] F[ ] K[ ] ListSigningProfiles
- W[x] S[ ] M[x] F[ ] K[ ] ListTagsForResource
- W[x] S[ ] M[x] F[ ] K[ ] PutSigningProfile
- W[x] S[ ] M[ ] F[ ] K[ ] RemoveProfilePermission
- W[x] S[ ] M[ ] F[ ] K[ ] RevokeSignature
- W[x] S[ ] M[ ] F[ ] K[ ] RevokeSigningProfile
- W[x] S[ ] M[ ] F[ ] K[ ] SignPayload
- W[x] S[ ] M[ ] F[ ] K[ ] StartSigningJob
- W[x] S[ ] M[x] F[ ] K[ ] TagResource
- W[x] S[ ] M[x] F[ ] K[ ] UntagResource

Integration tests: 19/19 implemented operations tested (100.0%)

### winterbaume-simpledbv2 (simpledbv2) - W: 3/3, S: 0/3, M: 0/3, F: 0/3, K: 0/3

- W[x] S[ ] M[ ] F[ ] K[ ] GetExport
- W[x] S[ ] M[ ] F[ ] K[ ] ListExports
- W[x] S[ ] M[ ] F[ ] K[ ] StartDomainExport

Integration tests: 3/3 implemented operations tested (100.0%)

### winterbaume-simspaceweaver (simspaceweaver) - W: 15/16, S: 0/16, M: 0/16, F: 0/16, K: 0/16

- W[x] S[ ] M[ ] F[ ] K[ ] CreateSnapshot
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteApp
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteSimulation
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeApp
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeSimulation
- W[x] S[ ] M[ ] F[ ] K[ ] ListApps
- W[x] S[ ] M[ ] F[ ] K[ ] ListSimulations
- W[ ] S[ ] M[ ] F[ ] K[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] StartApp
- W[x] S[ ] M[ ] F[ ] K[ ] StartClock
- W[x] S[ ] M[ ] F[ ] K[ ] StartSimulation
- W[x] S[ ] M[ ] F[ ] K[ ] StopApp
- W[x] S[ ] M[ ] F[ ] K[ ] StopClock
- W[x] S[ ] M[ ] F[ ] K[ ] StopSimulation
- W[x] S[ ] M[ ] F[ ] K[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UntagResource

Integration tests: 11/15 implemented operations tested (73.3%)
Untested implemented operations: 4

### winterbaume-snowdevicemanagement (snow-device-management) - W: 11/13, S: 0/13, M: 0/13, F: 0/13, K: 0/13

- W[x] S[ ] M[ ] F[ ] K[ ] CancelTask
- W[x] S[ ] M[ ] F[ ] K[ ] CreateTask
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeDevice
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeDeviceEc2Instances
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeExecution
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeTask
- W[x] S[ ] M[ ] F[ ] K[ ] ListDeviceResources
- W[x] S[ ] M[ ] F[ ] K[ ] ListDevices
- W[x] S[ ] M[ ] F[ ] K[ ] ListExecutions
- W[ ] S[ ] M[ ] F[ ] K[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] ListTasks
- W[x] S[ ] M[ ] F[ ] K[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UntagResource

Integration tests: 7/11 implemented operations tested (63.6%)
Untested implemented operations: 4

### winterbaume-sns (sns) - W: 41/42, S: 1/42, M: 33/42, F: 0/42, K: 13/42

Terraform E2E: 4 tests across 2 terraform resource types

Resource types: aws_sns_topic, aws_sns_topic_subscription

- W[x] S[ ] M[x] F[ ] K[ ] AddPermission
- W[x] S[ ] M[x] F[ ] K[ ] CheckIfPhoneNumberIsOptedOut
- W[x] S[ ] M[x] F[ ] K[ ] ConfirmSubscription
- W[x] S[ ] M[x] F[ ] K[ ] CreatePlatformApplication
- W[x] S[ ] M[x] F[ ] K[ ] CreatePlatformEndpoint
- W[x] S[ ] M[ ] F[ ] K[ ] CreateSMSSandboxPhoneNumber
- W[x] S[ ] M[x] F[ ] K[x] CreateTopic
- W[x] S[ ] M[x] F[ ] K[ ] DeleteEndpoint
- W[x] S[ ] M[x] F[ ] K[ ] DeletePlatformApplication
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteSMSSandboxPhoneNumber
- W[x] S[ ] M[x] F[ ] K[x] DeleteTopic
- W[x] S[ ] M[ ] F[ ] K[ ] GetDataProtectionPolicy
- W[x] S[ ] M[x] F[ ] K[ ] GetEndpointAttributes
- W[x] S[ ] M[x] F[ ] K[ ] GetPlatformApplicationAttributes
- W[x] S[ ] M[x] F[ ] K[ ] GetSMSAttributes
- W[x] S[ ] M[ ] F[ ] K[ ] GetSMSSandboxAccountStatus
- W[x] S[ ] M[x] F[ ] K[ ] GetSubscriptionAttributes
- W[x] S[ ] M[x] F[ ] K[x] GetTopicAttributes
- W[x] S[ ] M[x] F[ ] K[ ] ListEndpointsByPlatformApplication
- W[ ] S[x] M[ ] F[ ] K[ ] ListOriginationNumbers
- W[x] S[ ] M[x] F[ ] K[ ] ListPhoneNumbersOptedOut
- W[x] S[ ] M[x] F[ ] K[ ] ListPlatformApplications
- W[x] S[ ] M[ ] F[ ] K[ ] ListSMSSandboxPhoneNumbers
- W[x] S[ ] M[x] F[ ] K[x] ListSubscriptions
- W[x] S[ ] M[x] F[ ] K[x] ListSubscriptionsByTopic
- W[x] S[ ] M[x] F[ ] K[x] ListTagsForResource
- W[x] S[ ] M[x] F[ ] K[x] ListTopics
- W[x] S[ ] M[x] F[ ] K[ ] OptInPhoneNumber
- W[x] S[ ] M[x] F[ ] K[x] Publish
- W[x] S[ ] M[x] F[ ] K[ ] PublishBatch
- W[x] S[ ] M[ ] F[ ] K[ ] PutDataProtectionPolicy
- W[x] S[ ] M[x] F[ ] K[ ] RemovePermission
- W[x] S[ ] M[x] F[ ] K[ ] SetEndpointAttributes
- W[x] S[ ] M[x] F[ ] K[ ] SetPlatformApplicationAttributes
- W[x] S[ ] M[x] F[ ] K[ ] SetSMSAttributes
- W[x] S[ ] M[x] F[ ] K[ ] SetSubscriptionAttributes
- W[x] S[ ] M[ ] F[ ] K[x] SetTopicAttributes
- W[x] S[ ] M[x] F[ ] K[x] Subscribe
- W[x] S[ ] M[x] F[ ] K[x] TagResource
- W[x] S[ ] M[x] F[ ] K[x] Unsubscribe
- W[x] S[ ] M[x] F[ ] K[x] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] VerifySMSSandboxPhoneNumber

Integration tests: 36/41 implemented operations tested (87.8%)
Untested implemented operations: 5

### winterbaume-sqs (sqs) - W: 23/23, S: 0/23, M: 20/23, F: 0/23, K: 14/23

Terraform E2E: 10 tests across 2 terraform resource types

Resource types: aws_sqs_queue, aws_sqs_queue_policy

- W[x] S[ ] M[x] F[ ] K[ ] AddPermission
- W[x] S[ ] M[ ] F[ ] K[ ] CancelMessageMoveTask
- W[x] S[ ] M[x] F[ ] K[ ] ChangeMessageVisibility
- W[x] S[ ] M[x] F[ ] K[ ] ChangeMessageVisibilityBatch
- W[x] S[ ] M[x] F[ ] K[x] CreateQueue
- W[x] S[ ] M[x] F[ ] K[x] DeleteMessage
- W[x] S[ ] M[x] F[ ] K[x] DeleteMessageBatch
- W[x] S[ ] M[x] F[ ] K[x] DeleteQueue
- W[x] S[ ] M[x] F[ ] K[x] GetQueueAttributes
- W[x] S[ ] M[x] F[ ] K[ ] GetQueueUrl
- W[x] S[ ] M[x] F[ ] K[ ] ListDeadLetterSourceQueues
- W[x] S[ ] M[ ] F[ ] K[ ] ListMessageMoveTasks
- W[x] S[ ] M[x] F[ ] K[x] ListQueueTags
- W[x] S[ ] M[x] F[ ] K[x] ListQueues
- W[x] S[ ] M[x] F[ ] K[x] PurgeQueue
- W[x] S[ ] M[x] F[ ] K[x] ReceiveMessage
- W[x] S[ ] M[x] F[ ] K[ ] RemovePermission
- W[x] S[ ] M[x] F[ ] K[x] SendMessage
- W[x] S[ ] M[x] F[ ] K[x] SendMessageBatch
- W[x] S[ ] M[x] F[ ] K[x] SetQueueAttributes
- W[x] S[ ] M[ ] F[ ] K[ ] StartMessageMoveTask
- W[x] S[ ] M[x] F[ ] K[x] TagQueue
- W[x] S[ ] M[x] F[ ] K[x] UntagQueue

Integration tests: 22/23 implemented operations tested (95.7%)
Untested implemented operations: 1

### winterbaume-ssm (ssm) - W: 127/146, S: 19/146, M: 41/146, F: 0/146, K: 7/146

Terraform E2E: 12 tests across 6 terraform resource types

Resource types: aws_ssm_association, aws_ssm_document, aws_ssm_maintenance_window, aws_ssm_maintenance_window_target, aws_ssm_parameter, aws_ssm_patch_baseline

- W[x] S[ ] M[x] F[ ] K[ ] AddTagsToResource
- W[x] S[ ] M[ ] F[ ] K[ ] AssociateOpsItemRelatedItem
- W[x] S[ ] M[ ] F[ ] K[ ] CancelCommand
- W[x] S[ ] M[ ] F[ ] K[ ] CancelMaintenanceWindowExecution
- W[x] S[ ] M[ ] F[ ] K[ ] CreateActivation
- W[x] S[ ] M[ ] F[ ] K[ ] CreateAssociation
- W[x] S[ ] M[ ] F[ ] K[ ] CreateAssociationBatch
- W[x] S[ ] M[x] F[ ] K[ ] CreateDocument
- W[x] S[ ] M[x] F[ ] K[ ] CreateMaintenanceWindow
- W[x] S[ ] M[ ] F[ ] K[ ] CreateOpsItem
- W[x] S[ ] M[ ] F[ ] K[ ] CreateOpsMetadata
- W[x] S[ ] M[x] F[ ] K[ ] CreatePatchBaseline
- W[x] S[ ] M[ ] F[ ] K[ ] CreateResourceDataSync
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteActivation
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteAssociation
- W[x] S[ ] M[x] F[ ] K[ ] DeleteDocument
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteInventory
- W[x] S[ ] M[x] F[ ] K[ ] DeleteMaintenanceWindow
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteOpsItem
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteOpsMetadata
- W[x] S[ ] M[x] F[ ] K[x] DeleteParameter
- W[x] S[ ] M[x] F[ ] K[x] DeleteParameters
- W[x] S[ ] M[x] F[ ] K[ ] DeletePatchBaseline
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteResourceDataSync
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteResourcePolicy
- W[x] S[ ] M[ ] F[ ] K[ ] DeregisterManagedInstance
- W[x] S[ ] M[x] F[ ] K[ ] DeregisterPatchBaselineForPatchGroup
- W[x] S[ ] M[x] F[ ] K[ ] DeregisterTargetFromMaintenanceWindow
- W[x] S[ ] M[x] F[ ] K[ ] DeregisterTaskFromMaintenanceWindow
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeActivations
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeAssociation
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeAssociationExecutionTargets
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeAssociationExecutions
- W[ ] S[x] M[ ] F[ ] K[ ] DescribeAutomationExecutions
- W[ ] S[x] M[ ] F[ ] K[ ] DescribeAutomationStepExecutions
- W[ ] S[x] M[ ] F[ ] K[ ] DescribeAvailablePatches
- W[x] S[ ] M[x] F[ ] K[ ] DescribeDocument
- W[x] S[ ] M[x] F[ ] K[ ] DescribeDocumentPermission
- W[ ] S[x] M[ ] F[ ] K[ ] DescribeEffectiveInstanceAssociations
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeEffectivePatchesForPatchBaseline
- W[ ] S[x] M[ ] F[ ] K[ ] DescribeInstanceAssociationsStatus
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeInstanceInformation
- W[ ] S[x] M[ ] F[ ] K[ ] DescribeInstancePatchStates
- W[ ] S[x] M[ ] F[ ] K[ ] DescribeInstancePatchStatesForPatchGroup
- W[ ] S[x] M[ ] F[ ] K[ ] DescribeInstancePatches
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeInstanceProperties
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeInventoryDeletions
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeMaintenanceWindowExecutionTaskInvocations
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeMaintenanceWindowExecutionTasks
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeMaintenanceWindowExecutions
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeMaintenanceWindowSchedule
- W[x] S[ ] M[x] F[ ] K[ ] DescribeMaintenanceWindowTargets
- W[x] S[ ] M[x] F[ ] K[ ] DescribeMaintenanceWindowTasks
- W[x] S[ ] M[x] F[ ] K[ ] DescribeMaintenanceWindows
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeMaintenanceWindowsForTarget
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeOpsItems
- W[x] S[ ] M[x] F[ ] K[x] DescribeParameters
- W[x] S[ ] M[x] F[ ] K[ ] DescribePatchBaselines
- W[x] S[ ] M[ ] F[ ] K[ ] DescribePatchGroupState
- W[x] S[ ] M[ ] F[ ] K[ ] DescribePatchGroups
- W[ ] S[x] M[ ] F[ ] K[ ] DescribePatchProperties
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeSessions
- W[x] S[ ] M[ ] F[ ] K[ ] DisassociateOpsItemRelatedItem
- W[ ] S[x] M[ ] F[ ] K[ ] GetAccessToken
- W[ ] S[x] M[ ] F[ ] K[ ] GetAutomationExecution
- W[ ] S[x] M[ ] F[ ] K[ ] GetCalendarState
- W[x] S[ ] M[x] F[ ] K[ ] GetCommandInvocation
- W[x] S[ ] M[ ] F[ ] K[ ] GetConnectionStatus
- W[x] S[ ] M[ ] F[ ] K[ ] GetDefaultPatchBaseline
- W[x] S[ ] M[ ] F[ ] K[ ] GetDeployablePatchSnapshotForInstance
- W[x] S[ ] M[x] F[ ] K[ ] GetDocument
- W[ ] S[x] M[ ] F[ ] K[ ] GetExecutionPreview
- W[x] S[ ] M[ ] F[ ] K[ ] GetInventory
- W[x] S[ ] M[ ] F[ ] K[ ] GetInventorySchema
- W[x] S[ ] M[x] F[ ] K[ ] GetMaintenanceWindow
- W[x] S[ ] M[ ] F[ ] K[ ] GetMaintenanceWindowExecution
- W[x] S[ ] M[ ] F[ ] K[ ] GetMaintenanceWindowExecutionTask
- W[x] S[ ] M[ ] F[ ] K[ ] GetMaintenanceWindowExecutionTaskInvocation
- W[x] S[ ] M[ ] F[ ] K[ ] GetMaintenanceWindowTask
- W[x] S[ ] M[ ] F[ ] K[ ] GetOpsItem
- W[x] S[ ] M[ ] F[ ] K[ ] GetOpsMetadata
- W[x] S[ ] M[ ] F[ ] K[ ] GetOpsSummary
- W[x] S[ ] M[x] F[ ] K[x] GetParameter
- W[x] S[ ] M[x] F[ ] K[ ] GetParameterHistory
- W[x] S[ ] M[x] F[ ] K[x] GetParameters
- W[x] S[ ] M[x] F[ ] K[x] GetParametersByPath
- W[x] S[ ] M[ ] F[ ] K[ ] GetPatchBaseline
- W[x] S[ ] M[x] F[ ] K[ ] GetPatchBaselineForPatchGroup
- W[x] S[ ] M[ ] F[ ] K[ ] GetResourcePolicies
- W[x] S[ ] M[ ] F[ ] K[ ] GetServiceSetting
- W[x] S[ ] M[x] F[ ] K[ ] LabelParameterVersion
- W[x] S[ ] M[ ] F[ ] K[ ] ListAssociationVersions
- W[x] S[ ] M[ ] F[ ] K[ ] ListAssociations
- W[x] S[ ] M[ ] F[ ] K[ ] ListCommandInvocations
- W[x] S[ ] M[x] F[ ] K[ ] ListCommands
- W[x] S[ ] M[ ] F[ ] K[ ] ListComplianceItems
- W[x] S[ ] M[ ] F[ ] K[ ] ListComplianceSummaries
- W[x] S[ ] M[ ] F[ ] K[ ] ListDocumentMetadataHistory
- W[x] S[ ] M[ ] F[ ] K[ ] ListDocumentVersions
- W[x] S[ ] M[x] F[ ] K[ ] ListDocuments
- W[x] S[ ] M[ ] F[ ] K[ ] ListInventoryEntries
- W[x] S[ ] M[ ] F[ ] K[ ] ListNodes
- W[x] S[ ] M[ ] F[ ] K[ ] ListNodesSummary
- W[x] S[ ] M[ ] F[ ] K[ ] ListOpsItemEvents
- W[x] S[ ] M[ ] F[ ] K[ ] ListOpsItemRelatedItems
- W[x] S[ ] M[ ] F[ ] K[ ] ListOpsMetadata
- W[x] S[ ] M[ ] F[ ] K[ ] ListResourceComplianceSummaries
- W[x] S[ ] M[ ] F[ ] K[ ] ListResourceDataSync
- W[x] S[ ] M[x] F[ ] K[ ] ListTagsForResource
- W[x] S[ ] M[x] F[ ] K[ ] ModifyDocumentPermission
- W[x] S[ ] M[ ] F[ ] K[ ] PutComplianceItems
- W[x] S[ ] M[ ] F[ ] K[ ] PutInventory
- W[x] S[ ] M[x] F[ ] K[x] PutParameter
- W[x] S[ ] M[ ] F[ ] K[ ] PutResourcePolicy
- W[x] S[ ] M[ ] F[ ] K[ ] RegisterDefaultPatchBaseline
- W[x] S[ ] M[x] F[ ] K[ ] RegisterPatchBaselineForPatchGroup
- W[x] S[ ] M[x] F[ ] K[ ] RegisterTargetWithMaintenanceWindow
- W[x] S[ ] M[x] F[ ] K[ ] RegisterTaskWithMaintenanceWindow
- W[x] S[ ] M[x] F[ ] K[ ] RemoveTagsFromResource
- W[x] S[ ] M[ ] F[ ] K[ ] ResetServiceSetting
- W[x] S[ ] M[ ] F[ ] K[ ] ResumeSession
- W[ ] S[x] M[ ] F[ ] K[ ] SendAutomationSignal
- W[x] S[ ] M[x] F[ ] K[ ] SendCommand
- W[ ] S[x] M[ ] F[ ] K[ ] StartAccessRequest
- W[x] S[ ] M[ ] F[ ] K[ ] StartAssociationsOnce
- W[ ] S[x] M[ ] F[ ] K[ ] StartAutomationExecution
- W[ ] S[x] M[ ] F[ ] K[ ] StartChangeRequestExecution
- W[ ] S[x] M[ ] F[ ] K[ ] StartExecutionPreview
- W[x] S[ ] M[ ] F[ ] K[ ] StartSession
- W[ ] S[x] M[ ] F[ ] K[ ] StopAutomationExecution
- W[x] S[ ] M[ ] F[ ] K[ ] TerminateSession
- W[x] S[ ] M[x] F[ ] K[ ] UnlabelParameterVersion
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateAssociation
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateAssociationStatus
- W[x] S[ ] M[x] F[ ] K[ ] UpdateDocument
- W[x] S[ ] M[x] F[ ] K[ ] UpdateDocumentDefaultVersion
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateDocumentMetadata
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateMaintenanceWindow
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateMaintenanceWindowTarget
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateMaintenanceWindowTask
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateManagedInstanceRole
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateOpsItem
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateOpsMetadata
- W[x] S[ ] M[ ] F[ ] K[ ] UpdatePatchBaseline
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateResourceDataSync
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateServiceSetting

Integration tests: 125/127 implemented operations tested (98.4%)
Untested implemented operations: 2

### winterbaume-ssmquicksetup (ssm-quicksetup) - W: 6/14, S: 0/14, M: 0/14, F: 0/14, K: 0/14

- W[ ] S[ ] M[ ] F[ ] K[ ] CreateConfigurationManager
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteConfigurationManager
- W[x] S[ ] M[ ] F[ ] K[ ] GetConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] GetConfigurationManager
- W[x] S[ ] M[ ] F[ ] K[ ] GetServiceSettings
- W[ ] S[ ] M[ ] F[ ] K[ ] ListConfigurationManagers
- W[x] S[ ] M[ ] F[ ] K[ ] ListConfigurations
- W[ ] S[ ] M[ ] F[ ] K[ ] ListQuickSetupTypes
- W[ ] S[ ] M[ ] F[ ] K[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UntagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateConfigurationDefinition
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateConfigurationManager
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateServiceSettings

Integration tests: 2/6 implemented operations tested (33.3%)
Untested implemented operations: 4

### winterbaume-sso (sso) - W: 4/4, S: 0/4, M: 0/4, F: 0/4, K: 0/4

- W[x] S[ ] M[ ] F[ ] K[ ] GetRoleCredentials
- W[x] S[ ] M[ ] F[ ] K[ ] ListAccountRoles
- W[x] S[ ] M[ ] F[ ] K[ ] ListAccounts
- W[x] S[ ] M[ ] F[ ] K[ ] Logout

Integration tests: 4/4 implemented operations tested (100.0%)

### winterbaume-ssoadmin (sso-admin) - W: 27/79, S: 1/79, M: 25/79, F: 0/79, K: 0/79

- W[ ] S[ ] M[ ] F[ ] K[ ] AddRegion
- W[x] S[ ] M[x] F[ ] K[ ] AttachCustomerManagedPolicyReferenceToPermissionSet
- W[x] S[ ] M[x] F[ ] K[ ] AttachManagedPolicyToPermissionSet
- W[x] S[ ] M[x] F[ ] K[ ] CreateAccountAssignment
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateApplication
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateApplicationAssignment
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateInstance
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateInstanceAccessControlAttributeConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] CreatePermissionSet
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateTrustedTokenIssuer
- W[x] S[ ] M[x] F[ ] K[ ] DeleteAccountAssignment
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteApplication
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteApplicationAccessScope
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteApplicationAssignment
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteApplicationAuthenticationMethod
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteApplicationGrant
- W[x] S[ ] M[x] F[ ] K[ ] DeleteInlinePolicyFromPermissionSet
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteInstance
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteInstanceAccessControlAttributeConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] DeletePermissionSet
- W[ ] S[ ] M[ ] F[ ] K[ ] DeletePermissionsBoundaryFromPermissionSet
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteTrustedTokenIssuer
- W[x] S[ ] M[x] F[ ] K[ ] DescribeAccountAssignmentCreationStatus
- W[x] S[ ] M[x] F[ ] K[ ] DescribeAccountAssignmentDeletionStatus
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeApplication
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeApplicationAssignment
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeApplicationProvider
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeInstance
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeInstanceAccessControlAttributeConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] DescribePermissionSet
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribePermissionSetProvisioningStatus
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeRegion
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeTrustedTokenIssuer
- W[x] S[ ] M[x] F[ ] K[ ] DetachCustomerManagedPolicyReferenceFromPermissionSet
- W[x] S[ ] M[x] F[ ] K[ ] DetachManagedPolicyFromPermissionSet
- W[ ] S[ ] M[ ] F[ ] K[ ] GetApplicationAccessScope
- W[ ] S[ ] M[ ] F[ ] K[ ] GetApplicationAssignmentConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] GetApplicationAuthenticationMethod
- W[ ] S[ ] M[ ] F[ ] K[ ] GetApplicationGrant
- W[ ] S[ ] M[ ] F[ ] K[ ] GetApplicationSessionConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] GetInlinePolicyForPermissionSet
- W[ ] S[ ] M[ ] F[ ] K[ ] GetPermissionsBoundaryForPermissionSet
- W[ ] S[ ] M[ ] F[ ] K[ ] ListAccountAssignmentCreationStatus
- W[ ] S[ ] M[ ] F[ ] K[ ] ListAccountAssignmentDeletionStatus
- W[x] S[ ] M[x] F[ ] K[ ] ListAccountAssignments
- W[x] S[ ] M[x] F[ ] K[ ] ListAccountAssignmentsForPrincipal
- W[x] S[ ] M[x] F[ ] K[ ] ListAccountsForProvisionedPermissionSet
- W[ ] S[ ] M[ ] F[ ] K[ ] ListApplicationAccessScopes
- W[ ] S[ ] M[ ] F[ ] K[ ] ListApplicationAssignments
- W[ ] S[ ] M[ ] F[ ] K[ ] ListApplicationAssignmentsForPrincipal
- W[ ] S[ ] M[ ] F[ ] K[ ] ListApplicationAuthenticationMethods
- W[ ] S[ ] M[ ] F[ ] K[ ] ListApplicationGrants
- W[ ] S[ ] M[ ] F[ ] K[ ] ListApplicationProviders
- W[ ] S[ ] M[ ] F[ ] K[ ] ListApplications
- W[x] S[ ] M[x] F[ ] K[ ] ListCustomerManagedPolicyReferencesInPermissionSet
- W[ ] S[x] M[x] F[ ] K[ ] ListInstances
- W[x] S[ ] M[x] F[ ] K[ ] ListManagedPoliciesInPermissionSet
- W[ ] S[ ] M[ ] F[ ] K[ ] ListPermissionSetProvisioningStatus
- W[x] S[ ] M[x] F[ ] K[ ] ListPermissionSets
- W[x] S[ ] M[x] F[ ] K[ ] ListPermissionSetsProvisionedToAccount
- W[ ] S[ ] M[ ] F[ ] K[ ] ListRegions
- W[x] S[ ] M[ ] F[ ] K[ ] ListTagsForResource
- W[ ] S[ ] M[ ] F[ ] K[ ] ListTrustedTokenIssuers
- W[x] S[ ] M[x] F[ ] K[ ] ProvisionPermissionSet
- W[ ] S[ ] M[ ] F[ ] K[ ] PutApplicationAccessScope
- W[ ] S[ ] M[ ] F[ ] K[ ] PutApplicationAssignmentConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] PutApplicationAuthenticationMethod
- W[ ] S[ ] M[ ] F[ ] K[ ] PutApplicationGrant
- W[ ] S[ ] M[ ] F[ ] K[ ] PutApplicationSessionConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] PutInlinePolicyToPermissionSet
- W[ ] S[ ] M[ ] F[ ] K[ ] PutPermissionsBoundaryToPermissionSet
- W[ ] S[ ] M[ ] F[ ] K[ ] RemoveRegion
- W[x] S[ ] M[ ] F[ ] K[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UntagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateApplication
- W[x] S[ ] M[x] F[ ] K[ ] UpdateInstance
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateInstanceAccessControlAttributeConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] UpdatePermissionSet
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateTrustedTokenIssuer

Integration tests: 23/27 implemented operations tested (85.2%)
Untested implemented operations: 4

### winterbaume-sts (sts) - W: 11/11, S: 0/11, M: 7/11, F: 0/11, K: 6/11

- W[x] S[ ] M[x] F[ ] K[x] AssumeRole
- W[x] S[ ] M[x] F[ ] K[x] AssumeRoleWithSAML
- W[x] S[ ] M[x] F[ ] K[x] AssumeRoleWithWebIdentity
- W[x] S[ ] M[ ] F[ ] K[ ] AssumeRoot
- W[x] S[ ] M[ ] F[ ] K[ ] DecodeAuthorizationMessage
- W[x] S[ ] M[x] F[ ] K[ ] GetAccessKeyInfo
- W[x] S[ ] M[x] F[ ] K[x] GetCallerIdentity
- W[x] S[ ] M[ ] F[ ] K[ ] GetDelegatedAccessToken
- W[x] S[ ] M[x] F[ ] K[x] GetFederationToken
- W[x] S[ ] M[x] F[ ] K[x] GetSessionToken
- W[x] S[ ] M[ ] F[ ] K[ ] GetWebIdentityToken

Integration tests: 10/11 implemented operations tested (90.9%)
Untested implemented operations: 1

### winterbaume-support (support) - W: 5/16, S: 1/16, M: 5/16, F: 0/16, K: 0/16

- W[ ] S[ ] M[ ] F[ ] K[ ] AddAttachmentsToSet
- W[ ] S[ ] M[ ] F[ ] K[ ] AddCommunicationToCase
- W[x] S[ ] M[x] F[ ] K[ ] CreateCase
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeAttachment
- W[x] S[ ] M[x] F[ ] K[ ] DescribeCases
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeCommunications
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeCreateCaseOptions
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeServices
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeSeverityLevels
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeSupportedLanguages
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeTrustedAdvisorCheckRefreshStatuses
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeTrustedAdvisorCheckResult
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeTrustedAdvisorCheckSummaries
- W[ ] S[x] M[x] F[ ] K[ ] DescribeTrustedAdvisorChecks
- W[x] S[ ] M[x] F[ ] K[ ] RefreshTrustedAdvisorCheck
- W[x] S[ ] M[x] F[ ] K[ ] ResolveCase

Integration tests: 5/5 implemented operations tested (100.0%)

### winterbaume-supportapp (support-app) - W: 3/10, S: 0/10, M: 0/10, F: 0/10, K: 0/10

- W[ ] S[ ] M[ ] F[ ] K[ ] CreateSlackChannelConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteAccountAlias
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteSlackChannelConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteSlackWorkspaceConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] GetAccountAlias
- W[ ] S[ ] M[ ] F[ ] K[ ] ListSlackChannelConfigurations
- W[ ] S[ ] M[ ] F[ ] K[ ] ListSlackWorkspaceConfigurations
- W[x] S[ ] M[ ] F[ ] K[ ] PutAccountAlias
- W[ ] S[ ] M[ ] F[ ] K[ ] RegisterSlackWorkspaceForOrganization
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateSlackChannelConfiguration

Integration tests: 3/3 implemented operations tested (100.0%)

### winterbaume-swf (swf) - W: 30/39, S: 0/39, M: 19/39, F: 0/39, K: 0/39

- W[x] S[ ] M[ ] F[ ] K[ ] CountClosedWorkflowExecutions
- W[x] S[ ] M[ ] F[ ] K[ ] CountOpenWorkflowExecutions
- W[x] S[ ] M[x] F[ ] K[ ] CountPendingActivityTasks
- W[x] S[ ] M[x] F[ ] K[ ] CountPendingDecisionTasks
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteActivityType
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteWorkflowType
- W[x] S[ ] M[ ] F[ ] K[ ] DeprecateActivityType
- W[x] S[ ] M[x] F[ ] K[ ] DeprecateDomain
- W[x] S[ ] M[ ] F[ ] K[ ] DeprecateWorkflowType
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeActivityType
- W[x] S[ ] M[x] F[ ] K[ ] DescribeDomain
- W[x] S[ ] M[x] F[ ] K[ ] DescribeWorkflowExecution
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeWorkflowType
- W[x] S[ ] M[ ] F[ ] K[ ] GetWorkflowExecutionHistory
- W[x] S[ ] M[ ] F[ ] K[ ] ListActivityTypes
- W[x] S[ ] M[x] F[ ] K[ ] ListClosedWorkflowExecutions
- W[x] S[ ] M[x] F[ ] K[ ] ListDomains
- W[x] S[ ] M[x] F[ ] K[ ] ListOpenWorkflowExecutions
- W[ ] S[ ] M[ ] F[ ] K[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] ListWorkflowTypes
- W[x] S[ ] M[x] F[ ] K[ ] PollForActivityTask
- W[x] S[ ] M[x] F[ ] K[ ] PollForDecisionTask
- W[x] S[ ] M[x] F[ ] K[ ] RecordActivityTaskHeartbeat
- W[x] S[ ] M[ ] F[ ] K[ ] RegisterActivityType
- W[x] S[ ] M[x] F[ ] K[ ] RegisterDomain
- W[x] S[ ] M[ ] F[ ] K[ ] RegisterWorkflowType
- W[ ] S[ ] M[ ] F[ ] K[ ] RequestCancelWorkflowExecution
- W[ ] S[ ] M[ ] F[ ] K[ ] RespondActivityTaskCanceled
- W[x] S[ ] M[x] F[ ] K[ ] RespondActivityTaskCompleted
- W[x] S[ ] M[x] F[ ] K[ ] RespondActivityTaskFailed
- W[x] S[ ] M[x] F[ ] K[ ] RespondDecisionTaskCompleted
- W[x] S[ ] M[x] F[ ] K[ ] SignalWorkflowExecution
- W[x] S[ ] M[x] F[ ] K[ ] StartWorkflowExecution
- W[ ] S[ ] M[ ] F[ ] K[ ] TagResource
- W[x] S[ ] M[x] F[ ] K[ ] TerminateWorkflowExecution
- W[ ] S[ ] M[ ] F[ ] K[ ] UndeprecateActivityType
- W[x] S[ ] M[x] F[ ] K[ ] UndeprecateDomain
- W[ ] S[ ] M[ ] F[ ] K[ ] UndeprecateWorkflowType
- W[ ] S[ ] M[ ] F[ ] K[ ] UntagResource

Integration tests: 30/30 implemented operations tested (100.0%)

### winterbaume-synthetics (synthetics) - W: 22/22, S: 0/22, M: 4/22, F: 0/22, K: 0/22

- W[x] S[ ] M[ ] F[ ] K[ ] AssociateResource
- W[x] S[ ] M[x] F[ ] K[ ] CreateCanary
- W[x] S[ ] M[ ] F[ ] K[ ] CreateGroup
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteCanary
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteGroup
- W[x] S[ ] M[x] F[ ] K[ ] DescribeCanaries
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeCanariesLastRun
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeRuntimeVersions
- W[x] S[ ] M[ ] F[ ] K[ ] DisassociateResource
- W[x] S[ ] M[x] F[ ] K[ ] GetCanary
- W[x] S[ ] M[ ] F[ ] K[ ] GetCanaryRuns
- W[x] S[ ] M[ ] F[ ] K[ ] GetGroup
- W[x] S[ ] M[ ] F[ ] K[ ] ListAssociatedGroups
- W[x] S[ ] M[ ] F[ ] K[ ] ListGroupResources
- W[x] S[ ] M[ ] F[ ] K[ ] ListGroups
- W[x] S[ ] M[x] F[ ] K[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] StartCanary
- W[x] S[ ] M[ ] F[ ] K[ ] StartCanaryDryRun
- W[x] S[ ] M[ ] F[ ] K[ ] StopCanary
- W[x] S[ ] M[ ] F[ ] K[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateCanary

Integration tests: 22/22 implemented operations tested (100.0%)

### winterbaume-taxsettings (taxsettings) - W: 0/16, S: 0/16, M: 0/16, F: 0/16, K: 0/16

- W[ ] S[ ] M[ ] F[ ] K[ ] BatchDeleteTaxRegistration
- W[ ] S[ ] M[ ] F[ ] K[ ] BatchGetTaxExemptions
- W[ ] S[ ] M[ ] F[ ] K[ ] BatchPutTaxRegistration
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteSupplementalTaxRegistration
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteTaxRegistration
- W[ ] S[ ] M[ ] F[ ] K[ ] GetTaxExemptionTypes
- W[ ] S[ ] M[ ] F[ ] K[ ] GetTaxInheritance
- W[ ] S[ ] M[ ] F[ ] K[ ] GetTaxRegistration
- W[ ] S[ ] M[ ] F[ ] K[ ] GetTaxRegistrationDocument
- W[ ] S[ ] M[ ] F[ ] K[ ] ListSupplementalTaxRegistrations
- W[ ] S[ ] M[ ] F[ ] K[ ] ListTaxExemptions
- W[ ] S[ ] M[ ] F[ ] K[ ] ListTaxRegistrations
- W[ ] S[ ] M[ ] F[ ] K[ ] PutSupplementalTaxRegistration
- W[ ] S[ ] M[ ] F[ ] K[ ] PutTaxExemption
- W[ ] S[ ] M[ ] F[ ] K[ ] PutTaxInheritance
- W[ ] S[ ] M[ ] F[ ] K[ ] PutTaxRegistration

Integration tests: 0/0 implemented operations tested (0.0%)

### winterbaume-textract (textract) - W: 6/25, S: 0/25, M: 5/25, F: 0/25, K: 0/25

- W[x] S[ ] M[ ] F[ ] K[ ] AnalyzeDocument
- W[ ] S[ ] M[ ] F[ ] K[ ] AnalyzeExpense
- W[ ] S[ ] M[ ] F[ ] K[ ] AnalyzeID
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateAdapter
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateAdapterVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteAdapter
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteAdapterVersion
- W[x] S[ ] M[x] F[ ] K[ ] DetectDocumentText
- W[ ] S[ ] M[ ] F[ ] K[ ] GetAdapter
- W[ ] S[ ] M[ ] F[ ] K[ ] GetAdapterVersion
- W[x] S[ ] M[x] F[ ] K[ ] GetDocumentAnalysis
- W[x] S[ ] M[x] F[ ] K[ ] GetDocumentTextDetection
- W[ ] S[ ] M[ ] F[ ] K[ ] GetExpenseAnalysis
- W[ ] S[ ] M[ ] F[ ] K[ ] GetLendingAnalysis
- W[ ] S[ ] M[ ] F[ ] K[ ] GetLendingAnalysisSummary
- W[ ] S[ ] M[ ] F[ ] K[ ] ListAdapterVersions
- W[ ] S[ ] M[ ] F[ ] K[ ] ListAdapters
- W[ ] S[ ] M[ ] F[ ] K[ ] ListTagsForResource
- W[x] S[ ] M[x] F[ ] K[ ] StartDocumentAnalysis
- W[x] S[ ] M[x] F[ ] K[ ] StartDocumentTextDetection
- W[ ] S[ ] M[ ] F[ ] K[ ] StartExpenseAnalysis
- W[ ] S[ ] M[ ] F[ ] K[ ] StartLendingAnalysis
- W[ ] S[ ] M[ ] F[ ] K[ ] TagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] UntagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateAdapter

Integration tests: 6/6 implemented operations tested (100.0%)

### winterbaume-timestreaminfluxdb (timestream-influxdb) - W: 19/19, S: 0/19, M: 13/19, F: 0/19, K: 0/19

- W[x] S[ ] M[x] F[ ] K[ ] CreateDbCluster
- W[x] S[ ] M[x] F[ ] K[ ] CreateDbInstance
- W[x] S[ ] M[x] F[ ] K[ ] CreateDbParameterGroup
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteDbCluster
- W[x] S[ ] M[x] F[ ] K[ ] DeleteDbInstance
- W[x] S[ ] M[x] F[ ] K[ ] GetDbCluster
- W[x] S[ ] M[x] F[ ] K[ ] GetDbInstance
- W[x] S[ ] M[x] F[ ] K[ ] GetDbParameterGroup
- W[x] S[ ] M[x] F[ ] K[ ] ListDbClusters
- W[x] S[ ] M[x] F[ ] K[ ] ListDbInstances
- W[x] S[ ] M[ ] F[ ] K[ ] ListDbInstancesForCluster
- W[x] S[ ] M[x] F[ ] K[ ] ListDbParameterGroups
- W[x] S[ ] M[x] F[ ] K[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] RebootDbCluster
- W[x] S[ ] M[ ] F[ ] K[ ] RebootDbInstance
- W[x] S[ ] M[x] F[ ] K[ ] TagResource
- W[x] S[ ] M[x] F[ ] K[ ] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateDbCluster
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateDbInstance

Integration tests: 13/19 implemented operations tested (68.4%)
Untested implemented operations: 6

### winterbaume-timestreamquery (timestream-query) - W: 15/15, S: 0/15, M: 6/15, F: 0/15, K: 0/15

- W[x] S[ ] M[ ] F[ ] K[ ] CancelQuery
- W[x] S[ ] M[x] F[ ] K[ ] CreateScheduledQuery
- W[x] S[ ] M[x] F[ ] K[ ] DeleteScheduledQuery
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeAccountSettings
- W[x] S[ ] M[x] F[ ] K[ ] DescribeEndpoints
- W[x] S[ ] M[x] F[ ] K[ ] DescribeScheduledQuery
- W[x] S[ ] M[ ] F[ ] K[ ] ExecuteScheduledQuery
- W[x] S[ ] M[ ] F[ ] K[ ] ListScheduledQueries
- W[x] S[ ] M[ ] F[ ] K[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] PrepareQuery
- W[x] S[ ] M[x] F[ ] K[ ] Query
- W[x] S[ ] M[ ] F[ ] K[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateAccountSettings
- W[x] S[ ] M[x] F[ ] K[ ] UpdateScheduledQuery

Integration tests: 15/15 implemented operations tested (100.0%)

### winterbaume-timestreamwrite (timestream-write) - W: 19/19, S: 0/19, M: 15/19, F: 0/19, K: 0/19

- W[x] S[ ] M[ ] F[ ] K[ ] CreateBatchLoadTask
- W[x] S[ ] M[x] F[ ] K[ ] CreateDatabase
- W[x] S[ ] M[x] F[ ] K[ ] CreateTable
- W[x] S[ ] M[x] F[ ] K[ ] DeleteDatabase
- W[x] S[ ] M[x] F[ ] K[ ] DeleteTable
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeBatchLoadTask
- W[x] S[ ] M[x] F[ ] K[ ] DescribeDatabase
- W[x] S[ ] M[x] F[ ] K[ ] DescribeEndpoints
- W[x] S[ ] M[x] F[ ] K[ ] DescribeTable
- W[x] S[ ] M[ ] F[ ] K[ ] ListBatchLoadTasks
- W[x] S[ ] M[x] F[ ] K[ ] ListDatabases
- W[x] S[ ] M[x] F[ ] K[ ] ListTables
- W[x] S[ ] M[x] F[ ] K[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] ResumeBatchLoadTask
- W[x] S[ ] M[x] F[ ] K[ ] TagResource
- W[x] S[ ] M[x] F[ ] K[ ] UntagResource
- W[x] S[ ] M[x] F[ ] K[ ] UpdateDatabase
- W[x] S[ ] M[x] F[ ] K[ ] UpdateTable
- W[x] S[ ] M[x] F[ ] K[ ] WriteRecords

Integration tests: 15/19 implemented operations tested (78.9%)
Untested implemented operations: 4

### winterbaume-transcribe (transcribe) - W: 16/43, S: 0/43, M: 16/43, F: 0/43, K: 0/43

- W[ ] S[ ] M[ ] F[ ] K[ ] CreateCallAnalyticsCategory
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateLanguageModel
- W[x] S[ ] M[x] F[ ] K[ ] CreateMedicalVocabulary
- W[x] S[ ] M[x] F[ ] K[ ] CreateVocabulary
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateVocabularyFilter
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteCallAnalyticsCategory
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteCallAnalyticsJob
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteLanguageModel
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteMedicalScribeJob
- W[x] S[ ] M[x] F[ ] K[ ] DeleteMedicalTranscriptionJob
- W[x] S[ ] M[x] F[ ] K[ ] DeleteMedicalVocabulary
- W[x] S[ ] M[x] F[ ] K[ ] DeleteTranscriptionJob
- W[x] S[ ] M[x] F[ ] K[ ] DeleteVocabulary
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteVocabularyFilter
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeLanguageModel
- W[ ] S[ ] M[ ] F[ ] K[ ] GetCallAnalyticsCategory
- W[ ] S[ ] M[ ] F[ ] K[ ] GetCallAnalyticsJob
- W[ ] S[ ] M[ ] F[ ] K[ ] GetMedicalScribeJob
- W[x] S[ ] M[x] F[ ] K[ ] GetMedicalTranscriptionJob
- W[x] S[ ] M[x] F[ ] K[ ] GetMedicalVocabulary
- W[x] S[ ] M[x] F[ ] K[ ] GetTranscriptionJob
- W[x] S[ ] M[x] F[ ] K[ ] GetVocabulary
- W[ ] S[ ] M[ ] F[ ] K[ ] GetVocabularyFilter
- W[ ] S[ ] M[ ] F[ ] K[ ] ListCallAnalyticsCategories
- W[ ] S[ ] M[ ] F[ ] K[ ] ListCallAnalyticsJobs
- W[ ] S[ ] M[ ] F[ ] K[ ] ListLanguageModels
- W[ ] S[ ] M[ ] F[ ] K[ ] ListMedicalScribeJobs
- W[x] S[ ] M[x] F[ ] K[ ] ListMedicalTranscriptionJobs
- W[x] S[ ] M[x] F[ ] K[ ] ListMedicalVocabularies
- W[ ] S[ ] M[ ] F[ ] K[ ] ListTagsForResource
- W[x] S[ ] M[x] F[ ] K[ ] ListTranscriptionJobs
- W[x] S[ ] M[x] F[ ] K[ ] ListVocabularies
- W[ ] S[ ] M[ ] F[ ] K[ ] ListVocabularyFilters
- W[ ] S[ ] M[ ] F[ ] K[ ] StartCallAnalyticsJob
- W[ ] S[ ] M[ ] F[ ] K[ ] StartMedicalScribeJob
- W[x] S[ ] M[x] F[ ] K[ ] StartMedicalTranscriptionJob
- W[x] S[ ] M[x] F[ ] K[ ] StartTranscriptionJob
- W[ ] S[ ] M[ ] F[ ] K[ ] TagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] UntagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateCallAnalyticsCategory
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateMedicalVocabulary
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateVocabulary
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateVocabularyFilter

Integration tests: 16/16 implemented operations tested (100.0%)

### winterbaume-transfer (transfer) - W: 44/71, S: 0/71, M: 14/71, F: 0/71, K: 0/71

Terraform E2E: 8 tests across 4 terraform resource types

Resource types: aws_transfer_connector, aws_transfer_server, aws_transfer_user, aws_transfer_workflow

- W[ ] S[ ] M[ ] F[ ] K[ ] CreateAccess
- W[x] S[ ] M[ ] F[ ] K[ ] CreateAgreement
- W[x] S[ ] M[x] F[ ] K[ ] CreateConnector
- W[x] S[ ] M[ ] F[ ] K[ ] CreateProfile
- W[x] S[ ] M[x] F[ ] K[ ] CreateServer
- W[x] S[ ] M[x] F[ ] K[ ] CreateUser
- W[x] S[ ] M[ ] F[ ] K[ ] CreateWebApp
- W[x] S[ ] M[ ] F[ ] K[ ] CreateWorkflow
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteAccess
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteAgreement
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteCertificate
- W[x] S[ ] M[x] F[ ] K[ ] DeleteConnector
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteHostKey
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteProfile
- W[x] S[ ] M[x] F[ ] K[ ] DeleteServer
- W[x] S[ ] M[x] F[ ] K[ ] DeleteSshPublicKey
- W[x] S[ ] M[x] F[ ] K[ ] DeleteUser
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteWebApp
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteWebAppCustomization
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteWorkflow
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeAccess
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeAgreement
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeCertificate
- W[x] S[ ] M[x] F[ ] K[ ] DescribeConnector
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeExecution
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeHostKey
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeProfile
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeSecurityPolicy
- W[x] S[ ] M[x] F[ ] K[ ] DescribeServer
- W[x] S[ ] M[x] F[ ] K[ ] DescribeUser
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeWebApp
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeWebAppCustomization
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeWorkflow
- W[x] S[ ] M[ ] F[ ] K[ ] ImportCertificate
- W[ ] S[ ] M[ ] F[ ] K[ ] ImportHostKey
- W[x] S[ ] M[x] F[ ] K[ ] ImportSshPublicKey
- W[ ] S[ ] M[ ] F[ ] K[ ] ListAccesses
- W[x] S[ ] M[ ] F[ ] K[ ] ListAgreements
- W[x] S[ ] M[ ] F[ ] K[ ] ListCertificates
- W[x] S[ ] M[x] F[ ] K[ ] ListConnectors
- W[ ] S[ ] M[ ] F[ ] K[ ] ListExecutions
- W[ ] S[ ] M[ ] F[ ] K[ ] ListFileTransferResults
- W[ ] S[ ] M[ ] F[ ] K[ ] ListHostKeys
- W[x] S[ ] M[ ] F[ ] K[ ] ListProfiles
- W[ ] S[ ] M[ ] F[ ] K[ ] ListSecurityPolicies
- W[x] S[ ] M[x] F[ ] K[ ] ListServers
- W[ ] S[ ] M[ ] F[ ] K[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] ListUsers
- W[x] S[ ] M[ ] F[ ] K[ ] ListWebApps
- W[x] S[ ] M[ ] F[ ] K[ ] ListWorkflows
- W[ ] S[ ] M[ ] F[ ] K[ ] SendWorkflowStepState
- W[ ] S[ ] M[ ] F[ ] K[ ] StartDirectoryListing
- W[ ] S[ ] M[ ] F[ ] K[ ] StartFileTransfer
- W[ ] S[ ] M[ ] F[ ] K[ ] StartRemoteDelete
- W[ ] S[ ] M[ ] F[ ] K[ ] StartRemoteMove
- W[ ] S[ ] M[ ] F[ ] K[ ] StartServer
- W[ ] S[ ] M[ ] F[ ] K[ ] StopServer
- W[ ] S[ ] M[ ] F[ ] K[ ] TagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] TestConnection
- W[ ] S[ ] M[ ] F[ ] K[ ] TestIdentityProvider
- W[ ] S[ ] M[ ] F[ ] K[ ] UntagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateAccess
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateAgreement
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateCertificate
- W[x] S[ ] M[x] F[ ] K[ ] UpdateConnector
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateHostKey
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateProfile
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateServer
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateUser
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateWebApp
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateWebAppCustomization

Integration tests: 40/44 implemented operations tested (90.9%)
Untested implemented operations: 4

### winterbaume-trustedadvisor (trustedadvisor) - W: 6/11, S: 4/11, M: 0/11, F: 0/11, K: 0/11

- W[ ] S[ ] M[ ] F[ ] K[ ] BatchUpdateRecommendationResourceExclusion
- W[x] S[ ] M[ ] F[ ] K[ ] GetOrganizationRecommendation
- W[x] S[ ] M[ ] F[ ] K[ ] GetRecommendation
- W[ ] S[x] M[ ] F[ ] K[ ] ListChecks
- W[ ] S[x] M[ ] F[ ] K[ ] ListOrganizationRecommendationAccounts
- W[ ] S[x] M[ ] F[ ] K[ ] ListOrganizationRecommendationResources
- W[x] S[ ] M[ ] F[ ] K[ ] ListOrganizationRecommendations
- W[ ] S[x] M[ ] F[ ] K[ ] ListRecommendationResources
- W[x] S[ ] M[ ] F[ ] K[ ] ListRecommendations
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateOrganizationRecommendationLifecycle
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateRecommendationLifecycle

Integration tests: 3/6 implemented operations tested (50.0%)
Untested implemented operations: 3

### winterbaume-vpclattice (vpc-lattice) - W: 66/73, S: 2/73, M: 35/73, F: 0/73, K: 0/73

Terraform E2E: 18 tests across 9 terraform resource types

Resource types: aws_vpclattice_auth_policy, aws_vpclattice_listener, aws_vpclattice_listener_rule, aws_vpclattice_resource_policy, aws_vpclattice_service, aws_vpclattice_service_network, aws_vpclattice_service_network_service_association, aws_vpclattice_service_network_vpc_association, aws_vpclattice_target_group

- W[x] S[ ] M[ ] F[ ] K[ ] BatchUpdateRule
- W[x] S[ ] M[x] F[ ] K[ ] CreateAccessLogSubscription
- W[x] S[ ] M[x] F[ ] K[ ] CreateListener
- W[x] S[ ] M[ ] F[ ] K[ ] CreateResourceConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] CreateResourceGateway
- W[x] S[ ] M[x] F[ ] K[ ] CreateRule
- W[x] S[ ] M[x] F[ ] K[ ] CreateService
- W[x] S[ ] M[x] F[ ] K[ ] CreateServiceNetwork
- W[x] S[ ] M[x] F[ ] K[ ] CreateServiceNetworkResourceAssociation
- W[x] S[ ] M[x] F[ ] K[ ] CreateServiceNetworkServiceAssociation
- W[x] S[ ] M[x] F[ ] K[ ] CreateServiceNetworkVpcAssociation
- W[x] S[ ] M[x] F[ ] K[ ] CreateTargetGroup
- W[x] S[ ] M[x] F[ ] K[ ] DeleteAccessLogSubscription
- W[x] S[ ] M[x] F[ ] K[ ] DeleteAuthPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteDomainVerification
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteListener
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteResourceConfiguration
- W[ ] S[x] M[ ] F[ ] K[ ] DeleteResourceEndpointAssociation
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteResourceGateway
- W[x] S[ ] M[x] F[ ] K[ ] DeleteResourcePolicy
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteRule
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteService
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteServiceNetwork
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteServiceNetworkResourceAssociation
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteServiceNetworkServiceAssociation
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteServiceNetworkVpcAssociation
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteTargetGroup
- W[x] S[ ] M[ ] F[ ] K[ ] DeregisterTargets
- W[x] S[ ] M[x] F[ ] K[ ] GetAccessLogSubscription
- W[x] S[ ] M[x] F[ ] K[ ] GetAuthPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] GetDomainVerification
- W[x] S[ ] M[x] F[ ] K[ ] GetListener
- W[x] S[ ] M[ ] F[ ] K[ ] GetResourceConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] GetResourceGateway
- W[x] S[ ] M[x] F[ ] K[ ] GetResourcePolicy
- W[x] S[ ] M[ ] F[ ] K[ ] GetRule
- W[x] S[ ] M[x] F[ ] K[ ] GetService
- W[x] S[ ] M[x] F[ ] K[ ] GetServiceNetwork
- W[x] S[ ] M[x] F[ ] K[ ] GetServiceNetworkResourceAssociation
- W[x] S[ ] M[x] F[ ] K[ ] GetServiceNetworkServiceAssociation
- W[ ] S[ ] M[ ] F[ ] K[ ] GetServiceNetworkVpcAssociation
- W[x] S[ ] M[x] F[ ] K[ ] GetTargetGroup
- W[x] S[ ] M[x] F[ ] K[ ] ListAccessLogSubscriptions
- W[x] S[ ] M[ ] F[ ] K[ ] ListDomainVerifications
- W[x] S[ ] M[x] F[ ] K[ ] ListListeners
- W[x] S[ ] M[ ] F[ ] K[ ] ListResourceConfigurations
- W[ ] S[x] M[ ] F[ ] K[ ] ListResourceEndpointAssociations
- W[x] S[ ] M[ ] F[ ] K[ ] ListResourceGateways
- W[x] S[ ] M[ ] F[ ] K[ ] ListRules
- W[x] S[ ] M[x] F[ ] K[ ] ListServiceNetworkResourceAssociations
- W[x] S[ ] M[x] F[ ] K[ ] ListServiceNetworkServiceAssociations
- W[ ] S[ ] M[x] F[ ] K[ ] ListServiceNetworkVpcAssociations
- W[ ] S[ ] M[ ] F[ ] K[ ] ListServiceNetworkVpcEndpointAssociations
- W[x] S[ ] M[x] F[ ] K[ ] ListServiceNetworks
- W[x] S[ ] M[x] F[ ] K[ ] ListServices
- W[x] S[ ] M[x] F[ ] K[ ] ListTagsForResource
- W[x] S[ ] M[x] F[ ] K[ ] ListTargetGroups
- W[x] S[ ] M[ ] F[ ] K[ ] ListTargets
- W[x] S[ ] M[x] F[ ] K[ ] PutAuthPolicy
- W[x] S[ ] M[x] F[ ] K[ ] PutResourcePolicy
- W[x] S[ ] M[ ] F[ ] K[ ] RegisterTargets
- W[x] S[ ] M[ ] F[ ] K[ ] StartDomainVerification
- W[x] S[ ] M[x] F[ ] K[ ] TagResource
- W[x] S[ ] M[x] F[ ] K[ ] UntagResource
- W[x] S[ ] M[x] F[ ] K[ ] UpdateAccessLogSubscription
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateListener
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateResourceConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateResourceGateway
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateRule
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateService
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateServiceNetwork
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateServiceNetworkVpcAssociation
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateTargetGroup

Integration tests: 62/66 implemented operations tested (93.9%)
Untested implemented operations: 4

### winterbaume-wafv2 (wafv2) - W: 38/55, S: 0/55, M: 29/55, F: 0/55, K: 0/55

Terraform E2E: 12 tests across 4 terraform resource types

Resource types: aws_wafv2_ip_set, aws_wafv2_regex_pattern_set, aws_wafv2_rule_group, aws_wafv2_web_acl

- W[x] S[ ] M[x] F[ ] K[ ] AssociateWebACL
- W[x] S[ ] M[ ] F[ ] K[ ] CheckCapacity
- W[x] S[ ] M[ ] F[ ] K[ ] CreateAPIKey
- W[x] S[ ] M[x] F[ ] K[ ] CreateIPSet
- W[x] S[ ] M[x] F[ ] K[ ] CreateRegexPatternSet
- W[x] S[ ] M[x] F[ ] K[ ] CreateRuleGroup
- W[x] S[ ] M[x] F[ ] K[ ] CreateWebACL
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteAPIKey
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteFirewallManagerRuleGroups
- W[x] S[ ] M[x] F[ ] K[ ] DeleteIPSet
- W[x] S[ ] M[x] F[ ] K[ ] DeleteLoggingConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] DeletePermissionPolicy
- W[x] S[ ] M[x] F[ ] K[ ] DeleteRegexPatternSet
- W[x] S[ ] M[x] F[ ] K[ ] DeleteRuleGroup
- W[x] S[ ] M[x] F[ ] K[ ] DeleteWebACL
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeAllManagedProducts
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeManagedProductsByVendor
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeManagedRuleGroup
- W[x] S[ ] M[x] F[ ] K[ ] DisassociateWebACL
- W[ ] S[ ] M[ ] F[ ] K[ ] GenerateMobileSdkReleaseUrl
- W[ ] S[ ] M[ ] F[ ] K[ ] GetDecryptedAPIKey
- W[x] S[ ] M[x] F[ ] K[ ] GetIPSet
- W[x] S[ ] M[x] F[ ] K[ ] GetLoggingConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] GetManagedRuleSet
- W[ ] S[ ] M[ ] F[ ] K[ ] GetMobileSdkRelease
- W[x] S[ ] M[ ] F[ ] K[ ] GetPermissionPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] GetRateBasedStatementManagedKeys
- W[x] S[ ] M[x] F[ ] K[ ] GetRegexPatternSet
- W[x] S[ ] M[x] F[ ] K[ ] GetRuleGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] GetSampledRequests
- W[ ] S[ ] M[ ] F[ ] K[ ] GetTopPathStatisticsByTraffic
- W[x] S[ ] M[x] F[ ] K[ ] GetWebACL
- W[x] S[ ] M[x] F[ ] K[ ] GetWebACLForResource
- W[x] S[ ] M[ ] F[ ] K[ ] ListAPIKeys
- W[ ] S[ ] M[ ] F[ ] K[ ] ListAvailableManagedRuleGroupVersions
- W[ ] S[ ] M[ ] F[ ] K[ ] ListAvailableManagedRuleGroups
- W[x] S[ ] M[x] F[ ] K[ ] ListIPSets
- W[x] S[ ] M[x] F[ ] K[ ] ListLoggingConfigurations
- W[ ] S[ ] M[ ] F[ ] K[ ] ListManagedRuleSets
- W[ ] S[ ] M[ ] F[ ] K[ ] ListMobileSdkReleases
- W[x] S[ ] M[x] F[ ] K[ ] ListRegexPatternSets
- W[x] S[ ] M[ ] F[ ] K[ ] ListResourcesForWebACL
- W[x] S[ ] M[x] F[ ] K[ ] ListRuleGroups
- W[x] S[ ] M[x] F[ ] K[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] ListWebACLs
- W[x] S[ ] M[x] F[ ] K[ ] PutLoggingConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] PutManagedRuleSetVersions
- W[x] S[ ] M[ ] F[ ] K[ ] PutPermissionPolicy
- W[x] S[ ] M[x] F[ ] K[ ] TagResource
- W[x] S[ ] M[x] F[ ] K[ ] UntagResource
- W[x] S[ ] M[x] F[ ] K[ ] UpdateIPSet
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateManagedRuleSetVersionExpiryDate
- W[x] S[ ] M[x] F[ ] K[ ] UpdateRegexPatternSet
- W[x] S[ ] M[x] F[ ] K[ ] UpdateRuleGroup
- W[x] S[ ] M[x] F[ ] K[ ] UpdateWebACL

Integration tests: 31/38 implemented operations tested (81.6%)
Untested implemented operations: 7

### winterbaume-workspaces (workspaces) - W: 50/91, S: 0/91, M: 16/91, F: 0/91, K: 0/91

Terraform E2E: 4 tests across 2 terraform resource types

Resource types: aws_workspaces_connection_alias, aws_workspaces_ip_group

- W[ ] S[ ] M[ ] F[ ] K[ ] AcceptAccountLinkInvitation
- W[x] S[ ] M[ ] F[ ] K[ ] AssociateConnectionAlias
- W[x] S[ ] M[ ] F[ ] K[ ] AssociateIpGroups
- W[ ] S[ ] M[ ] F[ ] K[ ] AssociateWorkspaceApplication
- W[x] S[ ] M[ ] F[ ] K[ ] AuthorizeIpRules
- W[ ] S[ ] M[ ] F[ ] K[ ] CopyWorkspaceImage
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateAccountLinkInvitation
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateConnectClientAddIn
- W[x] S[ ] M[ ] F[ ] K[ ] CreateConnectionAlias
- W[x] S[ ] M[ ] F[ ] K[ ] CreateIpGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateStandbyWorkspaces
- W[x] S[ ] M[x] F[ ] K[ ] CreateTags
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateUpdatedWorkspaceImage
- W[x] S[ ] M[ ] F[ ] K[ ] CreateWorkspaceBundle
- W[x] S[ ] M[x] F[ ] K[ ] CreateWorkspaceImage
- W[x] S[ ] M[x] F[ ] K[ ] CreateWorkspaces
- W[x] S[ ] M[ ] F[ ] K[ ] CreateWorkspacesPool
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteAccountLinkInvitation
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteClientBranding
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteConnectClientAddIn
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteConnectionAlias
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteIpGroup
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteTags
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteWorkspaceBundle
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteWorkspaceImage
- W[ ] S[ ] M[ ] F[ ] K[ ] DeployWorkspaceApplications
- W[x] S[ ] M[x] F[ ] K[ ] DeregisterWorkspaceDirectory
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeAccount
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeAccountModifications
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeApplicationAssociations
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeApplications
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeBundleAssociations
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeClientBranding
- W[x] S[ ] M[x] F[ ] K[ ] DescribeClientProperties
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeConnectClientAddIns
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeConnectionAliasPermissions
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeConnectionAliases
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeCustomWorkspaceImageImport
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeImageAssociations
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeIpGroups
- W[x] S[ ] M[x] F[ ] K[ ] DescribeTags
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeWorkspaceAssociations
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeWorkspaceBundles
- W[x] S[ ] M[x] F[ ] K[ ] DescribeWorkspaceDirectories
- W[x] S[ ] M[x] F[ ] K[ ] DescribeWorkspaceImagePermissions
- W[x] S[ ] M[x] F[ ] K[ ] DescribeWorkspaceImages
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeWorkspaceSnapshots
- W[x] S[ ] M[x] F[ ] K[ ] DescribeWorkspaces
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeWorkspacesConnectionStatus
- W[ ] S[ ] M[ ] F[ ] K[ ] DescribeWorkspacesPoolSessions
- W[x] S[ ] M[ ] F[ ] K[ ] DescribeWorkspacesPools
- W[x] S[ ] M[ ] F[ ] K[ ] DisassociateConnectionAlias
- W[x] S[ ] M[ ] F[ ] K[ ] DisassociateIpGroups
- W[ ] S[ ] M[ ] F[ ] K[ ] DisassociateWorkspaceApplication
- W[ ] S[ ] M[ ] F[ ] K[ ] GetAccountLink
- W[ ] S[ ] M[ ] F[ ] K[ ] ImportClientBranding
- W[ ] S[ ] M[ ] F[ ] K[ ] ImportCustomWorkspaceImage
- W[ ] S[ ] M[ ] F[ ] K[ ] ImportWorkspaceImage
- W[ ] S[ ] M[ ] F[ ] K[ ] ListAccountLinks
- W[ ] S[ ] M[ ] F[ ] K[ ] ListAvailableManagementCidrRanges
- W[ ] S[ ] M[ ] F[ ] K[ ] MigrateWorkspace
- W[ ] S[ ] M[ ] F[ ] K[ ] ModifyAccount
- W[ ] S[ ] M[ ] F[ ] K[ ] ModifyCertificateBasedAuthProperties
- W[x] S[ ] M[x] F[ ] K[ ] ModifyClientProperties
- W[ ] S[ ] M[ ] F[ ] K[ ] ModifyEndpointEncryptionMode
- W[ ] S[ ] M[ ] F[ ] K[ ] ModifySamlProperties
- W[x] S[ ] M[x] F[ ] K[ ] ModifySelfservicePermissions
- W[ ] S[ ] M[ ] F[ ] K[ ] ModifyStreamingProperties
- W[ ] S[ ] M[ ] F[ ] K[ ] ModifyWorkspaceAccessProperties
- W[x] S[ ] M[x] F[ ] K[ ] ModifyWorkspaceCreationProperties
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyWorkspaceProperties
- W[x] S[ ] M[ ] F[ ] K[ ] ModifyWorkspaceState
- W[x] S[ ] M[ ] F[ ] K[ ] RebootWorkspaces
- W[x] S[ ] M[ ] F[ ] K[ ] RebuildWorkspaces
- W[x] S[ ] M[x] F[ ] K[ ] RegisterWorkspaceDirectory
- W[ ] S[ ] M[ ] F[ ] K[ ] RejectAccountLinkInvitation
- W[x] S[ ] M[ ] F[ ] K[ ] RestoreWorkspace
- W[x] S[ ] M[ ] F[ ] K[ ] RevokeIpRules
- W[x] S[ ] M[ ] F[ ] K[ ] StartWorkspaces
- W[x] S[ ] M[ ] F[ ] K[ ] StartWorkspacesPool
- W[x] S[ ] M[ ] F[ ] K[ ] StopWorkspaces
- W[x] S[ ] M[ ] F[ ] K[ ] StopWorkspacesPool
- W[x] S[ ] M[x] F[ ] K[ ] TerminateWorkspaces
- W[x] S[ ] M[ ] F[ ] K[ ] TerminateWorkspacesPool
- W[ ] S[ ] M[ ] F[ ] K[ ] TerminateWorkspacesPoolSession
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateConnectClientAddIn
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateConnectionAliasPermission
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateRulesOfIpGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateWorkspaceBundle
- W[x] S[ ] M[x] F[ ] K[ ] UpdateWorkspaceImagePermission
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateWorkspacesPool

Integration tests: 48/50 implemented operations tested (96.0%)
Untested implemented operations: 2

### winterbaume-workspacesweb (workspaces-web) - W: 68/75, S: 0/75, M: 27/75, F: 0/75, K: 0/75

- W[x] S[ ] M[x] F[ ] K[ ] AssociateBrowserSettings
- W[x] S[ ] M[ ] F[ ] K[ ] AssociateDataProtectionSettings
- W[ ] S[ ] M[ ] F[ ] K[ ] AssociateIpAccessSettings
- W[x] S[ ] M[x] F[ ] K[ ] AssociateNetworkSettings
- W[x] S[ ] M[ ] F[ ] K[ ] AssociateSessionLogger
- W[x] S[ ] M[ ] F[ ] K[ ] AssociateTrustStore
- W[x] S[ ] M[x] F[ ] K[ ] AssociateUserAccessLoggingSettings
- W[x] S[ ] M[x] F[ ] K[ ] AssociateUserSettings
- W[x] S[ ] M[x] F[ ] K[ ] CreateBrowserSettings
- W[x] S[ ] M[ ] F[ ] K[ ] CreateDataProtectionSettings
- W[x] S[ ] M[ ] F[ ] K[ ] CreateIdentityProvider
- W[ ] S[ ] M[ ] F[ ] K[ ] CreateIpAccessSettings
- W[x] S[ ] M[x] F[ ] K[ ] CreateNetworkSettings
- W[x] S[ ] M[x] F[ ] K[ ] CreatePortal
- W[x] S[ ] M[ ] F[ ] K[ ] CreateSessionLogger
- W[x] S[ ] M[ ] F[ ] K[ ] CreateTrustStore
- W[x] S[ ] M[x] F[ ] K[ ] CreateUserAccessLoggingSettings
- W[x] S[ ] M[x] F[ ] K[ ] CreateUserSettings
- W[x] S[ ] M[x] F[ ] K[ ] DeleteBrowserSettings
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteDataProtectionSettings
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteIdentityProvider
- W[ ] S[ ] M[ ] F[ ] K[ ] DeleteIpAccessSettings
- W[x] S[ ] M[x] F[ ] K[ ] DeleteNetworkSettings
- W[x] S[ ] M[x] F[ ] K[ ] DeletePortal
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteSessionLogger
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteTrustStore
- W[x] S[ ] M[x] F[ ] K[ ] DeleteUserAccessLoggingSettings
- W[x] S[ ] M[x] F[ ] K[ ] DeleteUserSettings
- W[x] S[ ] M[ ] F[ ] K[ ] DisassociateBrowserSettings
- W[x] S[ ] M[ ] F[ ] K[ ] DisassociateDataProtectionSettings
- W[ ] S[ ] M[ ] F[ ] K[ ] DisassociateIpAccessSettings
- W[x] S[ ] M[ ] F[ ] K[ ] DisassociateNetworkSettings
- W[x] S[ ] M[ ] F[ ] K[ ] DisassociateSessionLogger
- W[x] S[ ] M[ ] F[ ] K[ ] DisassociateTrustStore
- W[x] S[ ] M[ ] F[ ] K[ ] DisassociateUserAccessLoggingSettings
- W[x] S[ ] M[ ] F[ ] K[ ] DisassociateUserSettings
- W[x] S[ ] M[ ] F[ ] K[ ] ExpireSession
- W[x] S[ ] M[x] F[ ] K[ ] GetBrowserSettings
- W[x] S[ ] M[ ] F[ ] K[ ] GetDataProtectionSettings
- W[x] S[ ] M[ ] F[ ] K[ ] GetIdentityProvider
- W[ ] S[ ] M[ ] F[ ] K[ ] GetIpAccessSettings
- W[x] S[ ] M[x] F[ ] K[ ] GetNetworkSettings
- W[x] S[ ] M[x] F[ ] K[ ] GetPortal
- W[x] S[ ] M[ ] F[ ] K[ ] GetPortalServiceProviderMetadata
- W[x] S[ ] M[ ] F[ ] K[ ] GetSession
- W[x] S[ ] M[ ] F[ ] K[ ] GetSessionLogger
- W[x] S[ ] M[ ] F[ ] K[ ] GetTrustStore
- W[x] S[ ] M[ ] F[ ] K[ ] GetTrustStoreCertificate
- W[x] S[ ] M[x] F[ ] K[ ] GetUserAccessLoggingSettings
- W[x] S[ ] M[x] F[ ] K[ ] GetUserSettings
- W[x] S[ ] M[x] F[ ] K[ ] ListBrowserSettings
- W[x] S[ ] M[ ] F[ ] K[ ] ListDataProtectionSettings
- W[x] S[ ] M[ ] F[ ] K[ ] ListIdentityProviders
- W[ ] S[ ] M[ ] F[ ] K[ ] ListIpAccessSettings
- W[x] S[ ] M[x] F[ ] K[ ] ListNetworkSettings
- W[x] S[ ] M[x] F[ ] K[ ] ListPortals
- W[x] S[ ] M[ ] F[ ] K[ ] ListSessionLoggers
- W[x] S[ ] M[ ] F[ ] K[ ] ListSessions
- W[x] S[ ] M[x] F[ ] K[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] ListTrustStoreCertificates
- W[x] S[ ] M[ ] F[ ] K[ ] ListTrustStores
- W[x] S[ ] M[x] F[ ] K[ ] ListUserAccessLoggingSettings
- W[x] S[ ] M[x] F[ ] K[ ] ListUserSettings
- W[x] S[ ] M[x] F[ ] K[ ] TagResource
- W[x] S[ ] M[x] F[ ] K[ ] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateBrowserSettings
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateDataProtectionSettings
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateIdentityProvider
- W[ ] S[ ] M[ ] F[ ] K[ ] UpdateIpAccessSettings
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateNetworkSettings
- W[x] S[ ] M[ ] F[ ] K[ ] UpdatePortal
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateSessionLogger
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateTrustStore
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateUserAccessLoggingSettings
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateUserSettings

Integration tests: 63/68 implemented operations tested (92.6%)
Untested implemented operations: 5

### winterbaume-xray (xray) - W: 34/38, S: 4/38, M: 0/38, F: 0/38, K: 6/38

- W[x] S[ ] M[ ] F[ ] K[x] BatchGetTraces
- W[x] S[ ] M[ ] F[ ] K[ ] CancelTraceRetrieval
- W[x] S[ ] M[ ] F[ ] K[x] CreateGroup
- W[x] S[ ] M[ ] F[ ] K[ ] CreateSamplingRule
- W[x] S[ ] M[ ] F[ ] K[x] DeleteGroup
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteResourcePolicy
- W[x] S[ ] M[ ] F[ ] K[ ] DeleteSamplingRule
- W[x] S[ ] M[ ] F[ ] K[ ] GetEncryptionConfig
- W[x] S[ ] M[ ] F[ ] K[ ] GetGroup
- W[x] S[ ] M[ ] F[ ] K[ ] GetGroups
- W[x] S[ ] M[ ] F[ ] K[ ] GetIndexingRules
- W[ ] S[x] M[ ] F[ ] K[ ] GetInsight
- W[ ] S[x] M[ ] F[ ] K[ ] GetInsightEvents
- W[ ] S[x] M[ ] F[ ] K[ ] GetInsightImpactGraph
- W[ ] S[x] M[ ] F[ ] K[ ] GetInsightSummaries
- W[x] S[ ] M[ ] F[ ] K[ ] GetRetrievedTracesGraph
- W[x] S[ ] M[ ] F[ ] K[ ] GetSamplingRules
- W[x] S[ ] M[ ] F[ ] K[ ] GetSamplingStatisticSummaries
- W[x] S[ ] M[ ] F[ ] K[ ] GetSamplingTargets
- W[x] S[ ] M[ ] F[ ] K[x] GetServiceGraph
- W[x] S[ ] M[ ] F[ ] K[ ] GetTimeSeriesServiceStatistics
- W[x] S[ ] M[ ] F[ ] K[ ] GetTraceGraph
- W[x] S[ ] M[ ] F[ ] K[ ] GetTraceSegmentDestination
- W[x] S[ ] M[ ] F[ ] K[x] GetTraceSummaries
- W[x] S[ ] M[ ] F[ ] K[ ] ListResourcePolicies
- W[x] S[ ] M[ ] F[ ] K[ ] ListRetrievedTraces
- W[x] S[ ] M[ ] F[ ] K[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] PutEncryptionConfig
- W[x] S[ ] M[ ] F[ ] K[ ] PutResourcePolicy
- W[x] S[ ] M[ ] F[ ] K[ ] PutTelemetryRecords
- W[x] S[ ] M[ ] F[ ] K[x] PutTraceSegments
- W[x] S[ ] M[ ] F[ ] K[ ] StartTraceRetrieval
- W[x] S[ ] M[ ] F[ ] K[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateGroup
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateIndexingRule
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateSamplingRule
- W[x] S[ ] M[ ] F[ ] K[ ] UpdateTraceSegmentDestination

Integration tests: 26/34 implemented operations tested (76.5%)
Untested implemented operations: 8
