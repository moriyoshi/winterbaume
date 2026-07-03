# API Coverage Report

Generated: 2026-07-03

| Project | Version |
|---------|---------|
| winterbaume | winterbaume-appconfigdata-v0.3.1-2-g69abbcb0 |
| moto | 5.2.3.dev |
| floci | 1.5.30 |
| kumo | v0.25.3 |
| fakecloud | v0.33.0 |

## Overview

Legend: `winterbaume` = operations with real, state-backed logic. `stubs` = operations whose handler routes the request and returns an empty/default response without real behaviour (annotated with `// STUB[<category>]: ...` in `handlers.rs`). The two columns are disjoint -- stubs are excluded from the winterbaume count.

| Service | Model | winterbaume | stubs | moto | floci | kumo | fakecloud | Total | wb% | stub% | moto% | floci% | kumo% | fakecloud% |
|---------|-------|-------------|-------|------|-------|------|-----------|-------|-----|-------|-------|--------|------|------------|
| winterbaume-accessanalyzer | accessanalyzer | 11 | 0 | 0 | 0 | 0 | 0 | 37 | 29.7% | 0.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-account | account | 14 | 1 | 3 | 0 | 0 | 15 | 15 | 93.3% | 6.7% | 20.0% | 0.0% | 0.0% | 100.0% |
| winterbaume-acm | acm | 16 | 0 | 11 | 0 | 6 | 17 | 17 | 94.1% | 0.0% | 64.7% | 0.0% | 35.3% | 100.0% |
| winterbaume-acmpca | acm-pca | 23 | 0 | 17 | 0 | 0 | 0 | 23 | 100.0% | 0.0% | 73.9% | 0.0% | 0.0% | 0.0% |
| winterbaume-aiops | aiops | 11 | 0 | 0 | 0 | 0 | 0 | 11 | 100.0% | 0.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-amp | amp | 17 | 0 | 17 | 0 | 0 | 0 | 44 | 38.6% | 0.0% | 38.6% | 0.0% | 0.0% | 0.0% |
| winterbaume-amplify | amplify | 23 | 0 | 0 | 0 | 9 | 0 | 37 | 62.2% | 0.0% | 0.0% | 0.0% | 24.3% | 0.0% |
| winterbaume-amplifybackend | amplifybackend | 4 | 0 | 0 | 0 | 0 | 0 | 31 | 12.9% | 0.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-amplifyuibuilder | amplifyuibuilder | 28 | 0 | 0 | 0 | 0 | 0 | 28 | 100.0% | 0.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-apigateway | api-gateway | 117 | 2 | 78 | 72 | 17 | 124 | 124 | 94.4% | 1.6% | 62.9% | 58.1% | 13.7% | 100.0% |
| winterbaume-apigatewaymanagement | apigatewaymanagementapi | 3 | 0 | 3 | 0 | 0 | 0 | 3 | 100.0% | 0.0% | 100.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-apigatewayv2 | apigatewayv2 | 62 | 0 | 54 | 0 | 22 | 103 | 103 | 60.2% | 0.0% | 52.4% | 0.0% | 21.4% | 100.0% |
| winterbaume-appconfig | appconfig | 45 | 0 | 15 | 0 | 0 | 0 | 45 | 100.0% | 0.0% | 33.3% | 0.0% | 0.0% | 0.0% |
| winterbaume-appconfigdata | appconfigdata | 2 | 0 | 0 | 0 | 0 | 0 | 2 | 100.0% | 0.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-appfabric | appfabric | 6 | 0 | 0 | 0 | 0 | 0 | 26 | 23.1% | 0.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-appflow | appflow | 9 | 0 | 0 | 0 | 0 | 0 | 25 | 36.0% | 0.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-appintegrations | appintegrations | 23 | 0 | 0 | 0 | 0 | 0 | 23 | 100.0% | 0.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-applicationautoscaling | application-auto-scaling | 13 | 1 | 9 | 0 | 0 | 14 | 14 | 92.9% | 7.1% | 64.3% | 0.0% | 0.0% | 100.0% |
| winterbaume-applicationcostprofiler | applicationcostprofiler | 6 | 0 | 0 | 0 | 0 | 0 | 6 | 100.0% | 0.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-applicationdiscoveryservice | application-discovery-service | 28 | 0 | 0 | 0 | 0 | 0 | 28 | 100.0% | 0.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-applicationinsights | application-insights | 33 | 0 | 0 | 0 | 0 | 0 | 33 | 100.0% | 0.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-applicationsignals | application-signals | 10 | 3 | 0 | 0 | 0 | 0 | 23 | 43.5% | 13.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-appmesh | app-mesh | 38 | 0 | 0 | 0 | 25 | 0 | 38 | 100.0% | 0.0% | 0.0% | 0.0% | 65.8% | 0.0% |
| winterbaume-apprunner | apprunner | 23 | 0 | 0 | 0 | 0 | 0 | 37 | 62.2% | 0.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-appsync | appsync | 27 | 0 | 27 | 0 | 3 | 0 | 74 | 36.5% | 0.0% | 36.5% | 0.0% | 4.1% | 0.0% |
| winterbaume-arczonalshift | arc-zonal-shift | 14 | 1 | 0 | 0 | 0 | 0 | 15 | 93.3% | 6.7% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-artifact | artifact | 8 | 0 | 0 | 0 | 0 | 0 | 8 | 100.0% | 0.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-athena | athena | 25 | 0 | 27 | 0 | 7 | 70 | 70 | 35.7% | 0.0% | 38.6% | 0.0% | 10.0% | 100.0% |
| winterbaume-auditmanager | auditmanager | 15 | 0 | 0 | 0 | 0 | 0 | 62 | 24.2% | 0.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-autoscaling | auto-scaling | 52 | 0 | 39 | 0 | 0 | 13 | 66 | 78.8% | 0.0% | 59.1% | 0.0% | 0.0% | 19.7% |
| winterbaume-autoscalingplans | auto-scaling-plans | 6 | 0 | 0 | 0 | 0 | 0 | 6 | 100.0% | 0.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-backup | backup | 105 | 3 | 17 | 0 | 12 | 0 | 108 | 97.2% | 2.8% | 15.7% | 0.0% | 11.1% | 0.0% |
| winterbaume-backupgateway | backup-gateway | 25 | 0 | 0 | 0 | 0 | 0 | 25 | 100.0% | 0.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-backupsearch | backupsearch | 9 | 0 | 0 | 0 | 0 | 0 | 12 | 75.0% | 0.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-batch | batch | 39 | 0 | 24 | 0 | 10 | 45 | 45 | 86.7% | 0.0% | 53.3% | 0.0% | 22.2% | 100.0% |
| winterbaume-bcmdashboards | bcm-dashboards | 9 | 0 | 0 | 0 | 0 | 0 | 15 | 60.0% | 0.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-bcmdataexports | bcm-data-exports | 12 | 0 | 0 | 0 | 0 | 0 | 12 | 100.0% | 0.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-bcmrecommendedactions | bcm-recommended-actions | 1 | 0 | 0 | 0 | 0 | 0 | 1 | 100.0% | 0.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-bedrock | bedrock | 48 | 0 | 13 | 0 | 0 | 101 | 101 | 47.5% | 0.0% | 12.9% | 0.0% | 0.0% | 100.0% |
| winterbaume-bedrockagent | bedrock-agent | 72 | 0 | 11 | 0 | 0 | 72 | 72 | 100.0% | 0.0% | 15.3% | 0.0% | 0.0% | 100.0% |
| winterbaume-billing | billing | 12 | 0 | 0 | 0 | 0 | 0 | 12 | 100.0% | 0.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-braket | braket | 12 | 0 | 0 | 0 | 0 | 0 | 17 | 70.6% | 0.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-budgets | budgets | 7 | 0 | 7 | 0 | 0 | 0 | 26 | 26.9% | 0.0% | 26.9% | 0.0% | 0.0% | 0.0% |
| winterbaume-chatbot | chatbot | 15 | 0 | 0 | 0 | 0 | 0 | 34 | 44.1% | 0.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-chimesdkmeetings | chime-sdk-meetings | 12 | 0 | 0 | 0 | 0 | 0 | 16 | 75.0% | 0.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-cloud9 | cloud9 | 13 | 0 | 0 | 0 | 0 | 0 | 13 | 100.0% | 0.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-cloudcontrol | cloudcontrol | 8 | 0 | 0 | 0 | 6 | 8 | 8 | 100.0% | 0.0% | 0.0% | 0.0% | 75.0% | 100.0% |
| winterbaume-clouddirectory | clouddirectory | 13 | 0 | 13 | 0 | 0 | 0 | 66 | 19.7% | 0.0% | 19.7% | 0.0% | 0.0% | 0.0% |
| winterbaume-cloudformation | cloudformation | 40 | 3 | 33 | 0 | 8 | 90 | 90 | 44.4% | 3.3% | 36.7% | 0.0% | 8.9% | 100.0% |
| winterbaume-cloudfront | cloudfront | 156 | 11 | 25 | 0 | 16 | 167 | 167 | 93.4% | 6.6% | 15.0% | 0.0% | 9.6% | 100.0% |
| winterbaume-cloudfrontkeyvaluestore | cloudfront-keyvaluestore | 5 | 0 | 0 | 0 | 0 | 0 | 6 | 83.3% | 0.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-cloudhsmv2 | cloudhsm-v2 | 18 | 0 | 0 | 0 | 0 | 0 | 18 | 100.0% | 0.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-cloudsearchdomain | cloudsearch-domain | 2 | 0 | 0 | 0 | 0 | 0 | 3 | 66.7% | 0.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-cloudtrail | cloudtrail | 21 | 2 | 16 | 0 | 8 | 0 | 60 | 35.0% | 3.3% | 26.7% | 0.0% | 13.3% | 0.0% |
| winterbaume-cloudtraildata | cloudtrail-data | 1 | 0 | 0 | 0 | 0 | 0 | 1 | 100.0% | 0.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-cloudwatch | cloudwatch | 38 | 5 | 20 | 0 | 11 | 46 | 46 | 82.6% | 10.9% | 43.5% | 0.0% | 23.9% | 100.0% |
| winterbaume-cloudwatchlogs | cloudwatch-logs | 93 | 15 | 52 | 0 | 11 | 113 | 113 | 82.3% | 13.3% | 46.0% | 0.0% | 9.7% | 100.0% |
| winterbaume-codeartifact | codeartifact | 9 | 0 | 0 | 0 | 0 | 0 | 48 | 18.8% | 0.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-codebuild | codebuild | 29 | 0 | 9 | 0 | 0 | 0 | 59 | 49.2% | 0.0% | 15.3% | 0.0% | 0.0% | 0.0% |
| winterbaume-codecommit | codecommit | 25 | 0 | 3 | 0 | 0 | 0 | 79 | 31.6% | 0.0% | 3.8% | 0.0% | 0.0% | 0.0% |
| winterbaume-codedeploy | codedeploy | 15 | 0 | 14 | 0 | 0 | 0 | 47 | 31.9% | 0.0% | 29.8% | 0.0% | 0.0% | 0.0% |
| winterbaume-codegurureviewer | codeguru-reviewer | 9 | 0 | 0 | 0 | 11 | 0 | 14 | 64.3% | 0.0% | 0.0% | 0.0% | 78.6% | 0.0% |
| winterbaume-codegurusecurity | codeguru-security | 11 | 0 | 0 | 0 | 0 | 0 | 13 | 84.6% | 0.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-codepipeline | codepipeline | 44 | 0 | 8 | 0 | 0 | 0 | 44 | 100.0% | 0.0% | 18.2% | 0.0% | 0.0% | 0.0% |
| winterbaume-codestarnotifications | codestar-notifications | 7 | 0 | 0 | 0 | 0 | 0 | 13 | 53.8% | 0.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-cognitoidentity | cognito-identity | 20 | 3 | 10 | 0 | 0 | 23 | 23 | 87.0% | 13.0% | 43.5% | 0.0% | 0.0% | 100.0% |
| winterbaume-cognitoidentityprovider | cognito-identity-provider | 104 | 18 | 62 | 0 | 17 | 122 | 122 | 85.2% | 14.8% | 50.8% | 0.0% | 13.9% | 100.0% |
| winterbaume-cognitosync | cognito-sync | 11 | 0 | 0 | 0 | 0 | 0 | 17 | 64.7% | 0.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-comprehend | comprehend | 60 | 5 | 63 | 0 | 12 | 0 | 85 | 70.6% | 5.9% | 74.1% | 0.0% | 14.1% | 0.0% |
| winterbaume-config | config-service | 46 | 3 | 38 | 0 | 9 | 0 | 97 | 47.4% | 3.1% | 39.2% | 0.0% | 9.3% | 0.0% |
| winterbaume-connect | connect | 10 | 0 | 10 | 0 | 0 | 0 | 370 | 2.7% | 0.0% | 2.7% | 0.0% | 0.0% | 0.0% |
| winterbaume-connectcampaigns | connectcampaigns | 14 | 0 | 14 | 0 | 0 | 0 | 22 | 63.6% | 0.0% | 63.6% | 0.0% | 0.0% | 0.0% |
| winterbaume-connectcontactlens | connect-contact-lens | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0.0% | 0.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-connectparticipant | connectparticipant | 7 | 0 | 0 | 0 | 0 | 0 | 11 | 63.6% | 0.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-controlcatalog | controlcatalog | 6 | 0 | 0 | 0 | 0 | 0 | 6 | 100.0% | 0.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-costandusagereport | cost-and-usage-report-service | 7 | 0 | 0 | 0 | 0 | 0 | 7 | 100.0% | 0.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-costexplorer | cost-explorer | 22 | 25 | 0 | 0 | 8 | 0 | 47 | 46.8% | 53.2% | 0.0% | 0.0% | 17.0% | 0.0% |
| winterbaume-costoptimizationhub | cost-optimization-hub | 8 | 0 | 0 | 0 | 0 | 0 | 8 | 100.0% | 0.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-databasemigration | database-migration-service | 42 | 0 | 17 | 0 | 0 | 0 | 119 | 35.3% | 0.0% | 14.3% | 0.0% | 0.0% | 0.0% |
| winterbaume-databrew | databrew | 32 | 1 | 24 | 0 | 0 | 0 | 44 | 72.7% | 2.3% | 54.5% | 0.0% | 0.0% | 0.0% |
| winterbaume-datapipeline | data-pipeline | 19 | 0 | 0 | 0 | 0 | 0 | 19 | 100.0% | 0.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-datasync | datasync | 8 | 0 | 6 | 0 | 0 | 0 | 53 | 15.1% | 0.0% | 11.3% | 0.0% | 0.0% | 0.0% |
| winterbaume-dax | dax | 6 | 0 | 8 | 0 | 0 | 0 | 21 | 28.6% | 0.0% | 38.1% | 0.0% | 0.0% | 0.0% |
| winterbaume-directconnect | direct-connect | 7 | 0 | 0 | 0 | 0 | 0 | 63 | 11.1% | 0.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-directory | directory-service | 4 | 0 | 0 | 0 | 6 | 0 | 80 | 5.0% | 0.0% | 0.0% | 0.0% | 7.5% | 0.0% |
| winterbaume-dlm | dlm | 2 | 0 | 0 | 0 | 5 | 0 | 8 | 25.0% | 0.0% | 0.0% | 0.0% | 62.5% | 0.0% |
| winterbaume-dsql | dsql | 12 | 0 | 5 | 0 | 0 | 12 | 12 | 100.0% | 0.0% | 41.7% | 0.0% | 0.0% | 100.0% |
| winterbaume-dynamodb | dynamodb | 57 | 0 | 39 | 0 | 21 | 57 | 57 | 100.0% | 0.0% | 68.4% | 0.0% | 36.8% | 100.0% |
| winterbaume-dynamodbstreams | dynamodb-streams | 4 | 0 | 0 | 0 | 0 | 4 | 4 | 100.0% | 0.0% | 0.0% | 0.0% | 0.0% | 100.0% |
| winterbaume-ebs | ebs | 6 | 0 | 6 | 0 | 0 | 0 | 6 | 100.0% | 0.0% | 100.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-ec2 | ec2 | 713 | 50 | 223 | 0 | 39 | 763 | 763 | 93.4% | 6.6% | 29.2% | 0.0% | 5.1% | 100.0% |
| winterbaume-ec2instanceconnect | ec2-instance-connect | 2 | 0 | 1 | 0 | 0 | 0 | 2 | 100.0% | 0.0% | 50.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-ecr | ecr | 58 | 0 | 29 | 0 | 11 | 58 | 58 | 100.0% | 0.0% | 50.0% | 0.0% | 19.0% | 100.0% |
| winterbaume-ecs | ecs | 63 | 1 | 45 | 0 | 12 | 76 | 76 | 82.9% | 1.3% | 59.2% | 0.0% | 15.8% | 100.0% |
| winterbaume-efs | efs | 31 | 0 | 19 | 0 | 0 | 0 | 31 | 100.0% | 0.0% | 61.3% | 0.0% | 0.0% | 0.0% |
| winterbaume-eks | eks | 55 | 4 | 17 | 0 | 8 | 64 | 64 | 85.9% | 6.2% | 26.6% | 0.0% | 12.5% | 100.0% |
| winterbaume-elasticache | elasticache | 24 | 0 | 17 | 0 | 7 | 75 | 75 | 32.0% | 0.0% | 22.7% | 0.0% | 9.3% | 100.0% |
| winterbaume-elasticbeanstalk | elastic-beanstalk | 7 | 0 | 0 | 0 | 7 | 0 | 47 | 14.9% | 0.0% | 0.0% | 0.0% | 14.9% | 0.0% |
| winterbaume-elasticloadbalancing | elastic-load-balancing | 29 | 0 | 21 | 0 | 0 | 0 | 29 | 100.0% | 0.0% | 72.4% | 0.0% | 0.0% | 0.0% |
| winterbaume-elasticloadbalancingv2 | elastic-load-balancing-v2 | 50 | 1 | 33 | 0 | 28 | 51 | 51 | 98.0% | 2.0% | 64.7% | 0.0% | 54.9% | 100.0% |
| winterbaume-emr | emr | 54 | 2 | 26 | 0 | 0 | 0 | 60 | 90.0% | 3.3% | 43.3% | 0.0% | 0.0% | 0.0% |
| winterbaume-emrcontainers | emr-containers | 23 | 0 | 8 | 0 | 0 | 0 | 23 | 100.0% | 0.0% | 34.8% | 0.0% | 0.0% | 0.0% |
| winterbaume-emrserverless | emr-serverless | 16 | 0 | 11 | 0 | 11 | 0 | 22 | 72.7% | 0.0% | 50.0% | 0.0% | 50.0% | 0.0% |
| winterbaume-eventbridge | eventbridge | 57 | 0 | 45 | 0 | 18 | 57 | 57 | 100.0% | 0.0% | 78.9% | 0.0% | 31.6% | 100.0% |
| winterbaume-firehose | firehose | 12 | 0 | 12 | 0 | 7 | 12 | 12 | 100.0% | 0.0% | 100.0% | 0.0% | 58.3% | 100.0% |
| winterbaume-fis | fis | 8 | 0 | 5 | 0 | 0 | 0 | 26 | 30.8% | 0.0% | 19.2% | 0.0% | 0.0% | 0.0% |
| winterbaume-forecast | forecast | 5 | 0 | 5 | 0 | 17 | 0 | 63 | 7.9% | 0.0% | 7.9% | 0.0% | 27.0% | 0.0% |
| winterbaume-freetier | freetier | 5 | 0 | 0 | 0 | 0 | 0 | 5 | 100.0% | 0.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-fsx | fsx | 9 | 0 | 9 | 0 | 0 | 0 | 48 | 18.8% | 0.0% | 18.8% | 0.0% | 0.0% | 0.0% |
| winterbaume-glacier | glacier | 33 | 0 | 10 | 0 | 4 | 0 | 33 | 100.0% | 0.0% | 30.3% | 0.0% | 12.1% | 0.0% |
| winterbaume-glue | glue | 132 | 0 | 96 | 0 | 14 | 265 | 265 | 49.8% | 0.0% | 36.2% | 0.0% | 5.3% | 100.0% |
| winterbaume-greengrass | greengrass | 71 | 0 | 55 | 0 | 0 | 0 | 92 | 77.2% | 0.0% | 59.8% | 0.0% | 0.0% | 0.0% |
| winterbaume-guardduty | guardduty | 85 | 2 | 12 | 0 | 0 | 0 | 87 | 97.7% | 2.3% | 13.8% | 0.0% | 0.0% | 0.0% |
| winterbaume-iam | iam | 154 | 22 | 119 | 0 | 39 | 176 | 176 | 87.5% | 12.5% | 67.6% | 0.0% | 22.2% | 100.0% |
| winterbaume-identitystore | identitystore | 17 | 0 | 14 | 0 | 0 | 19 | 19 | 89.5% | 0.0% | 73.7% | 0.0% | 0.0% | 100.0% |
| winterbaume-inspector2 | inspector2 | 49 | 21 | 19 | 0 | 0 | 0 | 75 | 65.3% | 28.0% | 25.3% | 0.0% | 0.0% | 0.0% |
| winterbaume-iot | iot | 103 | 0 | 100 | 0 | 0 | 0 | 272 | 37.9% | 0.0% | 36.8% | 0.0% | 0.0% | 0.0% |
| winterbaume-iotdataplane | iot-data-plane | 8 | 0 | 0 | 0 | 0 | 0 | 8 | 100.0% | 0.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-ivs | ivs | 30 | 5 | 6 | 0 | 0 | 0 | 40 | 75.0% | 12.5% | 15.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-kafka | kafka | 10 | 0 | 13 | 0 | 6 | 0 | 59 | 16.9% | 0.0% | 22.0% | 0.0% | 10.2% | 0.0% |
| winterbaume-keyspaces | keyspaces | 19 | 0 | 0 | 0 | 0 | 0 | 19 | 100.0% | 0.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-kinesis | kinesis | 38 | 0 | 31 | 0 | 10 | 39 | 39 | 97.4% | 0.0% | 79.5% | 0.0% | 25.6% | 100.0% |
| winterbaume-kinesisanalyticsv2 | kinesis-analytics-v2 | 32 | 1 | 0 | 0 | 0 | 0 | 33 | 97.0% | 3.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-kinesisvideo | kinesis-video | 32 | 0 | 0 | 0 | 0 | 0 | 32 | 100.0% | 0.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-kinesisvideoarchivedmedia | kinesis-video-archived-media | 6 | 0 | 3 | 0 | 0 | 0 | 6 | 100.0% | 0.0% | 50.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-kms | kms | 53 | 0 | 40 | 0 | 22 | 54 | 54 | 98.1% | 0.0% | 74.1% | 0.0% | 40.7% | 100.0% |
| winterbaume-lakeformation | lakeformation | 19 | 1 | 20 | 0 | 0 | 0 | 61 | 31.1% | 1.6% | 32.8% | 0.0% | 0.0% | 0.0% |
| winterbaume-lambda | lambda | 85 | 0 | 46 | 0 | 23 | 85 | 85 | 100.0% | 0.0% | 54.1% | 0.0% | 27.1% | 100.0% |
| winterbaume-lexmodelsv2 | lex-models-v2 | 58 | 2 | 17 | 0 | 0 | 0 | 107 | 54.2% | 1.9% | 15.9% | 0.0% | 0.0% | 0.0% |
| winterbaume-macie2 | macie2 | 67 | 14 | 13 | 0 | 24 | 0 | 81 | 82.7% | 17.3% | 16.0% | 0.0% | 29.6% | 0.0% |
| winterbaume-managedblockchain | managedblockchain | 27 | 0 | 20 | 0 | 0 | 0 | 27 | 100.0% | 0.0% | 74.1% | 0.0% | 0.0% | 0.0% |
| winterbaume-marketplacemetering | marketplace-metering | 4 | 0 | 0 | 0 | 0 | 0 | 4 | 100.0% | 0.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-mediaconnect | mediaconnect | 21 | 0 | 18 | 0 | 0 | 0 | 82 | 25.6% | 0.0% | 22.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-medialive | medialive | 16 | 0 | 12 | 0 | 0 | 0 | 123 | 13.0% | 0.0% | 9.8% | 0.0% | 0.0% | 0.0% |
| winterbaume-mediapackage | mediapackage | 9 | 0 | 9 | 0 | 0 | 0 | 19 | 47.4% | 0.0% | 47.4% | 0.0% | 0.0% | 0.0% |
| winterbaume-mediapackagev2 | mediapackagev2 | 7 | 0 | 7 | 0 | 0 | 0 | 30 | 23.3% | 0.0% | 23.3% | 0.0% | 0.0% | 0.0% |
| winterbaume-mediastore | mediastore | 11 | 0 | 11 | 0 | 0 | 0 | 21 | 52.4% | 0.0% | 52.4% | 0.0% | 0.0% | 0.0% |
| winterbaume-mediastoredata | mediastore-data | 5 | 0 | 4 | 0 | 0 | 0 | 5 | 100.0% | 0.0% | 80.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-memorydb | memorydb | 13 | 0 | 13 | 0 | 10 | 45 | 45 | 28.9% | 0.0% | 28.9% | 0.0% | 22.2% | 100.0% |
| winterbaume-mq | mq | 23 | 1 | 19 | 0 | 6 | 0 | 24 | 95.8% | 4.2% | 79.2% | 0.0% | 25.0% | 0.0% |
| winterbaume-neptune | neptune | 64 | 6 | 47 | 0 | 6 | 0 | 70 | 91.4% | 8.6% | 67.1% | 0.0% | 8.6% | 0.0% |
| winterbaume-networkfirewall | network-firewall | 79 | 0 | 5 | 0 | 0 | 0 | 79 | 100.0% | 0.0% | 6.3% | 0.0% | 0.0% | 0.0% |
| winterbaume-networkmanager | networkmanager | 53 | 0 | 18 | 0 | 0 | 0 | 95 | 55.8% | 0.0% | 18.9% | 0.0% | 0.0% | 0.0% |
| winterbaume-opensearch | opensearch | 44 | 0 | 11 | 0 | 0 | 0 | 88 | 50.0% | 0.0% | 12.5% | 0.0% | 0.0% | 0.0% |
| winterbaume-opensearchserverless | opensearchserverless | 12 | 0 | 12 | 0 | 0 | 0 | 46 | 26.1% | 0.0% | 26.1% | 0.0% | 0.0% | 0.0% |
| winterbaume-organizations | organizations | 60 | 3 | 41 | 0 | 11 | 63 | 63 | 95.2% | 4.8% | 65.1% | 0.0% | 17.5% | 100.0% |
| winterbaume-osis | osis | 10 | 0 | 13 | 0 | 0 | 0 | 22 | 45.5% | 0.0% | 59.1% | 0.0% | 0.0% | 0.0% |
| winterbaume-outposts | outposts | 13 | 0 | 0 | 0 | 0 | 0 | 37 | 35.1% | 0.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-panorama | panorama | 10 | 1 | 0 | 0 | 0 | 0 | 34 | 29.4% | 2.9% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-pcaconnectorscep | pca-connector-scep | 11 | 0 | 0 | 0 | 0 | 0 | 12 | 91.7% | 0.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-personalize | personalize | 66 | 5 | 4 | 0 | 0 | 0 | 71 | 93.0% | 7.0% | 5.6% | 0.0% | 0.0% | 0.0% |
| winterbaume-personalizeevents | personalize-events | 5 | 0 | 0 | 0 | 0 | 0 | 5 | 100.0% | 0.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-personalizeruntime | personalize-runtime | 3 | 0 | 0 | 0 | 0 | 0 | 3 | 100.0% | 0.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-pi | pi | 9 | 4 | 0 | 0 | 0 | 0 | 13 | 69.2% | 30.8% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-pinpoint | pinpoint | 15 | 0 | 12 | 0 | 0 | 0 | 122 | 12.3% | 0.0% | 9.8% | 0.0% | 0.0% | 0.0% |
| winterbaume-pinpointsmsvoice | pinpoint-sms-voice | 4 | 0 | 0 | 0 | 0 | 0 | 8 | 50.0% | 0.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-pipes | pipes | 10 | 0 | 9 | 0 | 10 | 10 | 10 | 100.0% | 0.0% | 90.0% | 0.0% | 100.0% | 100.0% |
| winterbaume-polly | polly | 9 | 0 | 5 | 0 | 0 | 0 | 10 | 90.0% | 0.0% | 50.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-pricing | pricing | 5 | 0 | 0 | 0 | 0 | 0 | 5 | 100.0% | 0.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-quicksight | quicksight | 68 | 0 | 31 | 0 | 0 | 0 | 232 | 29.3% | 0.0% | 13.4% | 0.0% | 0.0% | 0.0% |
| winterbaume-ram | ram | 35 | 0 | 8 | 0 | 0 | 0 | 35 | 100.0% | 0.0% | 22.9% | 0.0% | 0.0% | 0.0% |
| winterbaume-rbin | rbin | 9 | 0 | 0 | 0 | 0 | 0 | 10 | 90.0% | 0.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-rds | rds | 146 | 4 | 85 | 0 | 12 | 164 | 164 | 89.0% | 2.4% | 51.8% | 0.0% | 7.3% | 100.0% |
| winterbaume-rdsdata | rds-data | 6 | 0 | 1 | 0 | 0 | 6 | 6 | 100.0% | 0.0% | 16.7% | 0.0% | 0.0% | 100.0% |
| winterbaume-redshift | redshift | 100 | 3 | 35 | 0 | 7 | 0 | 141 | 70.9% | 2.1% | 24.8% | 0.0% | 5.0% | 0.0% |
| winterbaume-redshiftdata | redshift-data | 11 | 0 | 4 | 0 | 0 | 0 | 11 | 100.0% | 0.0% | 36.4% | 0.0% | 0.0% | 0.0% |
| winterbaume-rekognition | rekognition | 8 | 4 | 8 | 0 | 13 | 0 | 75 | 10.7% | 5.3% | 10.7% | 0.0% | 17.3% | 0.0% |
| winterbaume-resiliencehub | resiliencehub | 22 | 0 | 17 | 0 | 17 | 0 | 63 | 34.9% | 0.0% | 27.0% | 0.0% | 27.0% | 0.0% |
| winterbaume-resourcegroups | resource-groups | 22 | 1 | 15 | 0 | 0 | 23 | 23 | 95.7% | 4.3% | 65.2% | 0.0% | 0.0% | 100.0% |
| winterbaume-resourcegroupstagging | resource-groups-tagging-api | 5 | 0 | 0 | 0 | 0 | 9 | 9 | 55.6% | 0.0% | 0.0% | 0.0% | 0.0% | 100.0% |
| winterbaume-rolesanywhere | rolesanywhere | 28 | 2 | 0 | 0 | 0 | 0 | 30 | 93.3% | 6.7% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-route53 | route-53 | 71 | 0 | 30 | 0 | 10 | 71 | 71 | 100.0% | 0.0% | 42.3% | 0.0% | 14.1% | 100.0% |
| winterbaume-route53domains | route-53-domains | 5 | 0 | 0 | 0 | 0 | 0 | 34 | 14.7% | 0.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-route53recoverycluster | route53-recovery-cluster | 4 | 0 | 0 | 0 | 0 | 0 | 4 | 100.0% | 0.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-route53resolver | route53resolver | 28 | 0 | 28 | 0 | 11 | 0 | 68 | 41.2% | 0.0% | 41.2% | 0.0% | 16.2% | 0.0% |
| winterbaume-s3 | s3 | 103 | 4 | 73 | 51 | 44 | 107 | 107 | 96.3% | 3.7% | 68.2% | 47.7% | 41.1% | 100.0% |
| winterbaume-s3control | s3-control | 87 | 10 | 0 | 0 | 7 | 0 | 97 | 89.7% | 10.3% | 0.0% | 0.0% | 7.2% | 0.0% |
| winterbaume-s3files | s3files | 21 | 0 | 0 | 0 | 0 | 0 | 21 | 100.0% | 0.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-s3outposts | s3outposts | 3 | 1 | 0 | 0 | 0 | 0 | 5 | 60.0% | 20.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-s3tables | s3tables | 46 | 3 | 14 | 0 | 12 | 0 | 49 | 93.9% | 6.1% | 28.6% | 0.0% | 24.5% | 0.0% |
| winterbaume-s3vectors | s3vectors | 19 | 0 | 15 | 12 | 0 | 0 | 19 | 100.0% | 0.0% | 78.9% | 63.2% | 0.0% | 0.0% |
| winterbaume-sagemaker | sagemaker | 141 | 1 | 112 | 0 | 11 | 0 | 396 | 35.6% | 0.3% | 28.3% | 0.0% | 2.8% | 0.0% |
| winterbaume-sagemakermetrics | sagemaker-metrics | 2 | 0 | 1 | 0 | 0 | 0 | 2 | 100.0% | 0.0% | 50.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-sagemakerruntime | sagemaker-runtime | 3 | 0 | 2 | 0 | 0 | 0 | 3 | 100.0% | 0.0% | 66.7% | 0.0% | 0.0% | 0.0% |
| winterbaume-savingsplans | savingsplans | 10 | 0 | 0 | 0 | 0 | 0 | 10 | 100.0% | 0.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-scheduler | scheduler | 12 | 0 | 12 | 0 | 9 | 12 | 12 | 100.0% | 0.0% | 100.0% | 0.0% | 75.0% | 100.0% |
| winterbaume-secretsmanager | secrets-manager | 22 | 1 | 21 | 0 | 11 | 23 | 23 | 95.7% | 4.3% | 91.3% | 0.0% | 47.8% | 100.0% |
| winterbaume-securityhub | securityhub | 97 | 10 | 13 | 0 | 0 | 0 | 107 | 90.7% | 9.3% | 12.1% | 0.0% | 0.0% | 0.0% |
| winterbaume-servicecatalog | service-catalog | 4 | 0 | 0 | 0 | 0 | 0 | 90 | 4.4% | 0.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-servicecatalogappregistry | service-catalog-appregistry | 23 | 1 | 0 | 0 | 0 | 0 | 24 | 95.8% | 4.2% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-servicediscovery | servicediscovery | 27 | 0 | 27 | 0 | 0 | 30 | 30 | 90.0% | 0.0% | 90.0% | 0.0% | 0.0% | 100.0% |
| winterbaume-servicequotas | service-quotas | 5 | 0 | 2 | 0 | 8 | 0 | 26 | 19.2% | 0.0% | 7.7% | 0.0% | 30.8% | 0.0% |
| winterbaume-ses | ses | 38 | 2 | 38 | 0 | 6 | 14 | 71 | 53.5% | 2.8% | 53.5% | 0.0% | 8.5% | 19.7% |
| winterbaume-sesv2 | sesv2 | 106 | 4 | 30 | 0 | 15 | 0 | 110 | 96.4% | 3.6% | 27.3% | 0.0% | 13.6% | 0.0% |
| winterbaume-sfn | sfn | 35 | 2 | 29 | 0 | 18 | 37 | 37 | 94.6% | 5.4% | 78.4% | 0.0% | 48.6% | 100.0% |
| winterbaume-shield | shield | 9 | 0 | 9 | 0 | 0 | 0 | 36 | 25.0% | 0.0% | 25.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-signer | signer | 19 | 0 | 7 | 0 | 0 | 0 | 19 | 100.0% | 0.0% | 36.8% | 0.0% | 0.0% | 0.0% |
| winterbaume-simpledbv2 | simpledbv2 | 3 | 0 | 0 | 0 | 0 | 0 | 3 | 100.0% | 0.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-simspaceweaver | simspaceweaver | 15 | 0 | 0 | 0 | 0 | 0 | 16 | 93.8% | 0.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-snowdevicemanagement | snow-device-management | 11 | 0 | 0 | 0 | 0 | 0 | 13 | 84.6% | 0.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-sns | sns | 41 | 1 | 33 | 0 | 15 | 42 | 42 | 97.6% | 2.4% | 78.6% | 0.0% | 35.7% | 100.0% |
| winterbaume-sqs | sqs | 23 | 0 | 20 | 0 | 16 | 23 | 23 | 100.0% | 0.0% | 87.0% | 0.0% | 69.6% | 100.0% |
| winterbaume-ssm | ssm | 127 | 19 | 41 | 0 | 10 | 146 | 146 | 87.0% | 13.0% | 28.1% | 0.0% | 6.8% | 100.0% |
| winterbaume-ssmquicksetup | ssm-quicksetup | 6 | 0 | 0 | 0 | 0 | 0 | 14 | 42.9% | 0.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-sso | sso | 4 | 0 | 0 | 0 | 0 | 0 | 4 | 100.0% | 0.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-ssoadmin | sso-admin | 27 | 1 | 25 | 0 | 0 | 79 | 79 | 34.2% | 1.3% | 31.6% | 0.0% | 0.0% | 100.0% |
| winterbaume-sts | sts | 11 | 0 | 7 | 0 | 6 | 11 | 11 | 100.0% | 0.0% | 63.6% | 0.0% | 54.5% | 100.0% |
| winterbaume-support | support | 5 | 1 | 5 | 0 | 0 | 0 | 16 | 31.2% | 6.2% | 31.2% | 0.0% | 0.0% | 0.0% |
| winterbaume-supportapp | support-app | 3 | 0 | 0 | 0 | 0 | 0 | 10 | 30.0% | 0.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-swf | swf | 30 | 0 | 21 | 0 | 0 | 0 | 39 | 76.9% | 0.0% | 53.8% | 0.0% | 0.0% | 0.0% |
| winterbaume-synthetics | synthetics | 22 | 0 | 4 | 0 | 0 | 0 | 22 | 100.0% | 0.0% | 18.2% | 0.0% | 0.0% | 0.0% |
| winterbaume-taxsettings | taxsettings | 0 | 0 | 0 | 0 | 0 | 0 | 16 | 0.0% | 0.0% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-textract | textract | 6 | 0 | 5 | 0 | 0 | 0 | 25 | 24.0% | 0.0% | 20.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-timestreaminfluxdb | timestream-influxdb | 19 | 0 | 13 | 0 | 0 | 0 | 19 | 100.0% | 0.0% | 68.4% | 0.0% | 0.0% | 0.0% |
| winterbaume-timestreamquery | timestream-query | 15 | 0 | 6 | 0 | 0 | 0 | 15 | 100.0% | 0.0% | 40.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-timestreamwrite | timestream-write | 19 | 0 | 15 | 0 | 0 | 0 | 19 | 100.0% | 0.0% | 78.9% | 0.0% | 0.0% | 0.0% |
| winterbaume-transcribe | transcribe | 16 | 0 | 16 | 0 | 0 | 0 | 43 | 37.2% | 0.0% | 37.2% | 0.0% | 0.0% | 0.0% |
| winterbaume-transfer | transfer | 44 | 0 | 14 | 0 | 0 | 0 | 71 | 62.0% | 0.0% | 19.7% | 0.0% | 0.0% | 0.0% |
| winterbaume-trustedadvisor | trustedadvisor | 6 | 4 | 0 | 0 | 0 | 0 | 11 | 54.5% | 36.4% | 0.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-vpclattice | vpc-lattice | 66 | 2 | 35 | 0 | 0 | 0 | 73 | 90.4% | 2.7% | 47.9% | 0.0% | 0.0% | 0.0% |
| winterbaume-wafv2 | wafv2 | 38 | 0 | 29 | 0 | 0 | 55 | 55 | 69.1% | 0.0% | 52.7% | 0.0% | 0.0% | 100.0% |
| winterbaume-workspaces | workspaces | 50 | 0 | 16 | 0 | 0 | 0 | 91 | 54.9% | 0.0% | 17.6% | 0.0% | 0.0% | 0.0% |
| winterbaume-workspacesweb | workspaces-web | 68 | 0 | 27 | 0 | 0 | 0 | 75 | 90.7% | 0.0% | 36.0% | 0.0% | 0.0% | 0.0% |
| winterbaume-xray | xray | 34 | 4 | 0 | 0 | 6 | 0 | 38 | 89.5% | 10.5% | 0.0% | 0.0% | 15.8% | 0.0% |

**winterbaume winterbaume-appconfigdata-v0.3.1-2-g69abbcb0: 7210 / 11367 operations across 224 services (63.4%)**

**winterbaume stubs: 333 / 11367 operations across 224 services (2.9%) - routed but return empty/default responses**

**moto 5.2.3.dev: 3296 / 11367 operations across 224 services (29.0%)**

**floci 1.5.30: 135 / 11367 operations across 224 services (1.2%)**

**kumo v0.25.3: 874 / 11367 operations across 224 services (7.7%)**

**fakecloud v0.33.0: 3980 / 11367 operations across 224 services (35.0%)**

**integration tests: 6153 / 7210 implemented operations tested (85.3%)**

**integration-test service coverage: 222 / 224 services with at least one tested implemented operation (99.1%)**

**terraform E2E: 40 / 224 services with success-oriented coverage (17.9%)**

**terraform E2E tests: 368 success-oriented tests covering 171 terraform resource types**

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
| winterbaume-apigateway | 117 | 66 | 51 | 56.4% | partial |
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
| winterbaume-cloudfront | 156 | 90 | 66 | 57.7% | partial |
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
| winterbaume-ec2 | 713 | 672 | 41 | 94.2% | partial |
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
| winterbaume-lambda | 85 | 61 | 24 | 71.8% | partial |
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
| winterbaume-s3 | 103 | 71 | 32 | 68.9% | partial |
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
| winterbaume-iam | 35 | 11 | 0 | covered |
| winterbaume-kinesis | 4 | 1 | 0 | covered |
| winterbaume-kms | 4 | 2 | 1 | covered |
| winterbaume-lambda | 15 | 8 | 0 | covered |
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

Legend: `W` = winterbaume (real implementation), `S` = stub (routed but returns empty/default), `M` = moto, `F` = floci, `K` = kumo, `C` = fakecloud

### winterbaume-accessanalyzer (accessanalyzer) - W: 11/37, S: 0/37, M: 0/37, F: 0/37, K: 0/37, C: 0/37

- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ApplyArchiveRule
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CancelPolicyGeneration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CheckAccessNotGranted
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CheckNoNewAccess
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CheckNoPublicAccess
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateAccessPreview
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateAnalyzer
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateArchiveRule
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteAnalyzer
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteArchiveRule
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GenerateFindingRecommendation
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetAccessPreview
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetAnalyzedResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetAnalyzer
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetArchiveRule
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetFinding
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetFindingRecommendation
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetFindingV2
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetFindingsStatistics
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetGeneratedPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListAccessPreviewFindings
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListAccessPreviews
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListAnalyzedResources
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListAnalyzers
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListArchiveRules
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListFindings
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListFindingsV2
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListPolicyGenerations
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListTagsForResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartPolicyGeneration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartResourceScan
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UntagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateAnalyzer
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateArchiveRule
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateFindings
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ValidatePolicy

Integration tests: 11/11 implemented operations tested (100.0%)

### winterbaume-account (account) - W: 14/15, S: 1/15, M: 3/15, F: 0/15, K: 0/15, C: 15/15

- W[x] S[ ] M[ ] F[ ] K[ ] C[x] AcceptPrimaryEmailUpdate
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteAlternateContact
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DisableRegion
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] EnableRegion
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetAccountInformation
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetAlternateContact
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetContactInformation
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] GetGovCloudAccountInformation
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetPrimaryEmail
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetRegionOptStatus
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListRegions
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] PutAccountName
- W[x] S[ ] M[x] F[ ] K[ ] C[x] PutAlternateContact
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] PutContactInformation
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] StartPrimaryEmailUpdate

Integration tests: 14/14 implemented operations tested (100.0%)

### winterbaume-acm (acm) - W: 16/17, S: 0/17, M: 11/17, F: 0/17, K: 6/17, C: 17/17

Terraform E2E: 3 tests across 1 terraform resource types

Resource types: aws_acm_certificate

- W[x] S[ ] M[x] F[ ] K[ ] C[x] AddTagsToCertificate
- W[x] S[ ] M[x] F[ ] K[x] C[x] DeleteCertificate
- W[x] S[ ] M[x] F[ ] K[x] C[x] DescribeCertificate
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ExportCertificate
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetAccountConfiguration
- W[x] S[ ] M[x] F[ ] K[x] C[x] GetCertificate
- W[x] S[ ] M[x] F[ ] K[x] C[x] ImportCertificate
- W[x] S[ ] M[x] F[ ] K[x] C[x] ListCertificates
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListTagsForCertificate
- W[x] S[ ] M[x] F[ ] K[ ] C[x] PutAccountConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] C[x] RemoveTagsFromCertificate
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] RenewCertificate
- W[x] S[ ] M[x] F[ ] K[x] C[x] RequestCertificate
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ResendValidationEmail
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] RevokeCertificate
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] SearchCertificates
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateCertificateOptions

Integration tests: 12/16 implemented operations tested (75.0%)
Untested implemented operations: 4

### winterbaume-acmpca (acm-pca) - W: 23/23, S: 0/23, M: 17/23, F: 0/23, K: 0/23, C: 0/23

- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateCertificateAuthority
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateCertificateAuthorityAuditReport
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreatePermission
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteCertificateAuthority
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeletePermission
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeletePolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeCertificateAuthority
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeCertificateAuthorityAuditReport
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetCertificate
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetCertificateAuthorityCertificate
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetCertificateAuthorityCsr
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetPolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ImportCertificateAuthorityCertificate
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] IssueCertificate
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListCertificateAuthorities
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListPermissions
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListTags
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] PutPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] RestoreCertificateAuthority
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] RevokeCertificate
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] TagCertificateAuthority
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UntagCertificateAuthority
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UpdateCertificateAuthority

Integration tests: 22/23 implemented operations tested (95.7%)
Untested implemented operations: 1

### winterbaume-aiops (aiops) - W: 11/11, S: 0/11, M: 0/11, F: 0/11, K: 0/11, C: 0/11

- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateInvestigationGroup
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteInvestigationGroup
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteInvestigationGroupPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetInvestigationGroup
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetInvestigationGroupPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListInvestigationGroups
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutInvestigationGroupPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateInvestigationGroup

Integration tests: 11/11 implemented operations tested (100.0%)

### winterbaume-amp (amp) - W: 17/44, S: 0/44, M: 17/44, F: 0/44, K: 0/44, C: 0/44

- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateAlertManagerDefinition
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateAnomalyDetector
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateLoggingConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateQueryLoggingConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateRuleGroupsNamespace
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateScraper
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateWorkspace
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteAlertManagerDefinition
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteAnomalyDetector
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteLoggingConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteQueryLoggingConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteResourcePolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteRuleGroupsNamespace
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteScraper
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteScraperLoggingConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteWorkspace
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeAlertManagerDefinition
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeAnomalyDetector
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeLoggingConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeQueryLoggingConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeResourcePolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeRuleGroupsNamespace
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeScraper
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeScraperLoggingConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeWorkspace
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeWorkspaceConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetDefaultScraperConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListAnomalyDetectors
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListRuleGroupsNamespaces
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListScrapers
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListTagsForResource
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListWorkspaces
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] PutAlertManagerDefinition
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] PutAnomalyDetector
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] PutResourcePolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] PutRuleGroupsNamespace
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] TagResource
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UntagResource
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UpdateLoggingConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateQueryLoggingConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateScraper
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateScraperLoggingConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UpdateWorkspaceAlias
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateWorkspaceConfiguration

Integration tests: 17/17 implemented operations tested (100.0%)

### winterbaume-amplify (amplify) - W: 23/37, S: 0/37, M: 0/37, F: 0/37, K: 9/37, C: 0/37

- W[x] S[ ] M[ ] F[ ] K[x] C[ ] CreateApp
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateBackendEnvironment
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] CreateBranch
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateDeployment
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateDomainAssociation
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateWebhook
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] DeleteApp
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteBackendEnvironment
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] DeleteBranch
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteDomainAssociation
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteJob
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteWebhook
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GenerateAccessLogs
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] GetApp
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetArtifactUrl
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetBackendEnvironment
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] GetBranch
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetDomainAssociation
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetJob
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetWebhook
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] ListApps
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListArtifacts
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListBackendEnvironments
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] ListBranches
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListDomainAssociations
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListJobs
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListTagsForResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListWebhooks
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartDeployment
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] StartJob
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] StopJob
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UntagResource
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] UpdateApp
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateBranch
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateDomainAssociation
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateWebhook

Integration tests: 21/23 implemented operations tested (91.3%)
Untested implemented operations: 2

### winterbaume-amplifybackend (amplifybackend) - W: 4/31, S: 0/31, M: 0/31, F: 0/31, K: 0/31, C: 0/31

- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CloneBackend
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateBackend
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateBackendAPI
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateBackendAuth
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateBackendConfig
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateBackendStorage
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateToken
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteBackend
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteBackendAPI
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteBackendAuth
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteBackendStorage
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteToken
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GenerateBackendAPIModels
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetBackend
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetBackendAPI
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetBackendAPIModels
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetBackendAuth
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetBackendJob
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetBackendStorage
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetToken
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ImportBackendAuth
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ImportBackendStorage
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListBackendJobs
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListS3Buckets
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] RemoveAllBackends
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] RemoveBackendConfig
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateBackendAPI
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateBackendAuth
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateBackendConfig
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateBackendJob
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateBackendStorage

Integration tests: 4/4 implemented operations tested (100.0%)

### winterbaume-amplifyuibuilder (amplifyuibuilder) - W: 28/28, S: 0/28, M: 0/28, F: 0/28, K: 0/28, C: 0/28

- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateComponent
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateForm
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateTheme
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteComponent
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteForm
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteTheme
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ExchangeCodeForToken
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ExportComponents
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ExportForms
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ExportThemes
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetCodegenJob
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetComponent
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetForm
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetMetadata
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetTheme
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListCodegenJobs
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListComponents
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListForms
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListThemes
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutMetadataFlag
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] RefreshToken
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] StartCodegenJob
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateComponent
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateForm
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateTheme

Integration tests: 18/28 implemented operations tested (64.3%)
Untested implemented operations: 10

### winterbaume-apigateway (api-gateway) - W: 117/124, S: 2/124, M: 78/124, F: 72/124, K: 17/124, C: 124/124

- W[x] S[ ] M[x] F[x] K[ ] C[x] CreateApiKey
- W[x] S[ ] M[x] F[x] K[ ] C[x] CreateAuthorizer
- W[x] S[ ] M[x] F[x] K[ ] C[x] CreateBasePathMapping
- W[x] S[ ] M[x] F[x] K[x] C[x] CreateDeployment
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateDocumentationPart
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateDocumentationVersion
- W[x] S[ ] M[x] F[x] K[ ] C[x] CreateDomainName
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateDomainNameAccessAssociation
- W[x] S[ ] M[x] F[x] K[ ] C[x] CreateModel
- W[x] S[ ] M[x] F[x] K[ ] C[x] CreateRequestValidator
- W[x] S[ ] M[x] F[x] K[x] C[x] CreateResource
- W[x] S[ ] M[x] F[x] K[ ] C[x] CreateRestApi
- W[x] S[ ] M[x] F[x] K[x] C[x] CreateStage
- W[x] S[ ] M[x] F[x] K[ ] C[x] CreateUsagePlan
- W[x] S[ ] M[x] F[x] K[ ] C[x] CreateUsagePlanKey
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateVpcLink
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteApiKey
- W[x] S[ ] M[x] F[x] K[ ] C[x] DeleteAuthorizer
- W[x] S[ ] M[x] F[x] K[ ] C[x] DeleteBasePathMapping
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteClientCertificate
- W[x] S[ ] M[x] F[x] K[x] C[x] DeleteDeployment
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteDocumentationPart
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteDocumentationVersion
- W[x] S[ ] M[x] F[x] K[ ] C[x] DeleteDomainName
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteDomainNameAccessAssociation
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteGatewayResponse
- W[x] S[ ] M[x] F[x] K[ ] C[x] DeleteIntegration
- W[x] S[ ] M[x] F[x] K[ ] C[x] DeleteIntegrationResponse
- W[x] S[ ] M[x] F[x] K[x] C[x] DeleteMethod
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteMethodResponse
- W[x] S[ ] M[x] F[x] K[ ] C[x] DeleteModel
- W[x] S[ ] M[x] F[x] K[ ] C[x] DeleteRequestValidator
- W[x] S[ ] M[x] F[x] K[x] C[x] DeleteResource
- W[x] S[ ] M[x] F[x] K[ ] C[x] DeleteRestApi
- W[x] S[ ] M[x] F[x] K[x] C[x] DeleteStage
- W[x] S[ ] M[x] F[x] K[ ] C[x] DeleteUsagePlan
- W[x] S[ ] M[x] F[x] K[ ] C[x] DeleteUsagePlanKey
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteVpcLink
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] FlushStageAuthorizersCache
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] FlushStageCache
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GenerateClientCertificate
- W[x] S[ ] M[x] F[x] K[ ] C[x] GetAccount
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetApiKey
- W[x] S[ ] M[x] F[x] K[ ] C[x] GetApiKeys
- W[x] S[ ] M[x] F[x] K[ ] C[x] GetAuthorizer
- W[x] S[ ] M[x] F[x] K[ ] C[x] GetAuthorizers
- W[x] S[ ] M[x] F[x] K[ ] C[x] GetBasePathMapping
- W[x] S[ ] M[x] F[x] K[ ] C[x] GetBasePathMappings
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetClientCertificate
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetClientCertificates
- W[x] S[ ] M[x] F[x] K[x] C[x] GetDeployment
- W[x] S[ ] M[x] F[x] K[x] C[x] GetDeployments
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetDocumentationPart
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetDocumentationParts
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetDocumentationVersion
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetDocumentationVersions
- W[x] S[ ] M[x] F[x] K[ ] C[x] GetDomainName
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetDomainNameAccessAssociations
- W[x] S[ ] M[x] F[x] K[ ] C[x] GetDomainNames
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] GetExport
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetGatewayResponse
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetGatewayResponses
- W[x] S[ ] M[x] F[x] K[x] C[x] GetIntegration
- W[x] S[ ] M[x] F[x] K[ ] C[x] GetIntegrationResponse
- W[x] S[ ] M[x] F[x] K[x] C[x] GetMethod
- W[x] S[ ] M[x] F[x] K[ ] C[x] GetMethodResponse
- W[x] S[ ] M[x] F[x] K[ ] C[x] GetModel
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] GetModelTemplate
- W[x] S[ ] M[x] F[x] K[ ] C[x] GetModels
- W[x] S[ ] M[x] F[x] K[ ] C[x] GetRequestValidator
- W[x] S[ ] M[x] F[x] K[ ] C[x] GetRequestValidators
- W[x] S[ ] M[x] F[x] K[x] C[x] GetResource
- W[x] S[ ] M[x] F[x] K[x] C[x] GetResources
- W[x] S[ ] M[x] F[x] K[ ] C[x] GetRestApi
- W[x] S[ ] M[ ] F[x] K[ ] C[x] GetRestApis
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetSdk
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetSdkType
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetSdkTypes
- W[x] S[ ] M[x] F[x] K[x] C[x] GetStage
- W[x] S[ ] M[x] F[x] K[x] C[x] GetStages
- W[x] S[ ] M[ ] F[x] K[ ] C[x] GetTags
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetUsage
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetUsagePlan
- W[x] S[ ] M[x] F[x] K[ ] C[x] GetUsagePlanKey
- W[x] S[ ] M[x] F[x] K[ ] C[x] GetUsagePlanKeys
- W[x] S[ ] M[x] F[x] K[ ] C[x] GetUsagePlans
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetVpcLink
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetVpcLinks
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ImportApiKeys
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ImportDocumentationParts
- W[x] S[ ] M[x] F[x] K[ ] C[x] ImportRestApi
- W[x] S[ ] M[x] F[ ] K[ ] C[x] PutGatewayResponse
- W[x] S[ ] M[x] F[x] K[x] C[x] PutIntegration
- W[x] S[ ] M[x] F[x] K[ ] C[x] PutIntegrationResponse
- W[x] S[ ] M[x] F[x] K[x] C[x] PutMethod
- W[x] S[ ] M[x] F[x] K[ ] C[x] PutMethodResponse
- W[x] S[ ] M[x] F[x] K[ ] C[x] PutRestApi
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] RejectDomainNameAccessAssociation
- W[x] S[ ] M[ ] F[x] K[ ] C[x] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] TestInvokeAuthorizer
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] TestInvokeMethod
- W[x] S[ ] M[ ] F[x] K[ ] C[x] UntagResource
- W[x] S[ ] M[x] F[x] K[ ] C[x] UpdateAccount
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UpdateApiKey
- W[x] S[ ] M[x] F[x] K[ ] C[x] UpdateAuthorizer
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UpdateBasePathMapping
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateClientCertificate
- W[x] S[ ] M[ ] F[x] K[ ] C[x] UpdateDeployment
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateDocumentationPart
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateDocumentationVersion
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateDomainName
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateGatewayResponse
- W[x] S[ ] M[ ] F[x] K[ ] C[x] UpdateIntegration
- W[x] S[ ] M[ ] F[x] K[ ] C[x] UpdateIntegrationResponse
- W[x] S[ ] M[ ] F[x] K[ ] C[x] UpdateMethod
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateMethodResponse
- W[x] S[ ] M[ ] F[x] K[ ] C[x] UpdateModel
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UpdateRequestValidator
- W[x] S[ ] M[ ] F[x] K[ ] C[x] UpdateResource
- W[x] S[ ] M[x] F[x] K[ ] C[x] UpdateRestApi
- W[x] S[ ] M[x] F[x] K[ ] C[x] UpdateStage
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateUsage
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UpdateUsagePlan
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] UpdateVpcLink

Integration tests: 66/117 implemented operations tested (56.4%)
Untested implemented operations: 51

### winterbaume-apigatewaymanagement (apigatewaymanagementapi) - W: 3/3, S: 0/3, M: 3/3, F: 0/3, K: 0/3, C: 0/3

- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteConnection
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetConnection
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] PostToConnection

Integration tests: 3/3 implemented operations tested (100.0%)

### winterbaume-apigatewayv2 (apigatewayv2) - W: 62/103, S: 0/103, M: 54/103, F: 0/103, K: 22/103, C: 103/103

Terraform E2E: 15 tests across 9 terraform resource types

Resource types: aws_apigatewayv2_api, aws_apigatewayv2_api_mapping, aws_apigatewayv2_authorizer, aws_apigatewayv2_deployment, aws_apigatewayv2_domain_name, aws_apigatewayv2_integration, aws_apigatewayv2_route, aws_apigatewayv2_stage, aws_apigatewayv2_vpc_link

- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateApi
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateApiMapping
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateAuthorizer
- W[x] S[ ] M[ ] F[ ] K[x] C[x] CreateDeployment
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateDomainName
- W[x] S[ ] M[x] F[ ] K[x] C[x] CreateIntegration
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateIntegrationResponse
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateModel
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] CreatePortal
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] CreatePortalProduct
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] CreateProductPage
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] CreateProductRestEndpointPage
- W[x] S[ ] M[x] F[ ] K[x] C[x] CreateRoute
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateRouteResponse
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] CreateRoutingRule
- W[x] S[ ] M[x] F[ ] K[x] C[x] CreateStage
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateVpcLink
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DeleteAccessLogSettings
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteApi
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteApiMapping
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteAuthorizer
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteCorsConfiguration
- W[x] S[ ] M[ ] F[ ] K[x] C[x] DeleteDeployment
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteDomainName
- W[x] S[ ] M[x] F[ ] K[x] C[x] DeleteIntegration
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteIntegrationResponse
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteModel
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DeletePortal
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DeletePortalProduct
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DeletePortalProductSharingPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DeleteProductPage
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DeleteProductRestEndpointPage
- W[x] S[ ] M[x] F[ ] K[x] C[x] DeleteRoute
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteRouteRequestParameter
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteRouteResponse
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DeleteRouteSettings
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DeleteRoutingRule
- W[x] S[ ] M[x] F[ ] K[x] C[x] DeleteStage
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteVpcLink
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DisablePortal
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ExportApi
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetApi
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetApiMapping
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetApiMappings
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetApis
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetAuthorizer
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetAuthorizers
- W[x] S[ ] M[ ] F[ ] K[x] C[x] GetDeployment
- W[x] S[ ] M[ ] F[ ] K[x] C[x] GetDeployments
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetDomainName
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetDomainNames
- W[x] S[ ] M[x] F[ ] K[x] C[x] GetIntegration
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetIntegrationResponse
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetIntegrationResponses
- W[x] S[ ] M[x] F[ ] K[x] C[x] GetIntegrations
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetModel
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetModelTemplate
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetModels
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetPortal
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetPortalProduct
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetPortalProductSharingPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetProductPage
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetProductRestEndpointPage
- W[x] S[ ] M[x] F[ ] K[x] C[x] GetRoute
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetRouteResponse
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetRouteResponses
- W[x] S[ ] M[x] F[ ] K[x] C[x] GetRoutes
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetRoutingRule
- W[x] S[ ] M[x] F[ ] K[x] C[x] GetStage
- W[x] S[ ] M[x] F[ ] K[x] C[x] GetStages
- W[x] S[ ] M[x] F[ ] K[x] C[x] GetTags
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetVpcLink
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetVpcLinks
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ImportApi
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ListPortalProducts
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ListPortals
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ListProductPages
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ListProductRestEndpointPages
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ListRoutingRules
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] PreviewPortal
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] PublishPortal
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] PutPortalProductSharingPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] PutRoutingRule
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ReimportApi
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ResetAuthorizersCache
- W[x] S[ ] M[x] F[ ] K[x] C[x] TagResource
- W[x] S[ ] M[x] F[ ] K[x] C[x] UntagResource
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UpdateApi
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] UpdateApiMapping
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UpdateAuthorizer
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] UpdateDeployment
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] UpdateDomainName
- W[x] S[ ] M[x] F[ ] K[x] C[x] UpdateIntegration
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UpdateIntegrationResponse
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UpdateModel
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] UpdatePortal
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] UpdatePortalProduct
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] UpdateProductPage
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] UpdateProductRestEndpointPage
- W[x] S[ ] M[x] F[ ] K[x] C[x] UpdateRoute
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] UpdateRouteResponse
- W[x] S[ ] M[ ] F[ ] K[x] C[x] UpdateStage
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UpdateVpcLink

Integration tests: 59/62 implemented operations tested (95.2%)
Untested implemented operations: 3

### winterbaume-appconfig (appconfig) - W: 45/45, S: 0/45, M: 15/45, F: 0/45, K: 0/45, C: 0/45

Terraform E2E: 8 tests across 7 terraform resource types

Resource types: aws_appconfig_application, aws_appconfig_configuration_profile, aws_appconfig_deployment, aws_appconfig_deployment_strategy, aws_appconfig_environment, aws_appconfig_extension, aws_appconfig_hosted_configuration_version

- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateApplication
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateConfigurationProfile
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateDeploymentStrategy
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateEnvironment
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateExtension
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateExtensionAssociation
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateHostedConfigurationVersion
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteApplication
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteConfigurationProfile
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteDeploymentStrategy
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteEnvironment
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteExtension
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteExtensionAssociation
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteHostedConfigurationVersion
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetAccountSettings
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetApplication
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetConfigurationProfile
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetDeployment
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetDeploymentStrategy
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetEnvironment
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetExtension
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetExtensionAssociation
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetHostedConfigurationVersion
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListApplications
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListConfigurationProfiles
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListDeploymentStrategies
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListDeployments
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListEnvironments
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListExtensionAssociations
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListExtensions
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListHostedConfigurationVersions
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] StartDeployment
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] StopDeployment
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] TagResource
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateAccountSettings
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UpdateApplication
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UpdateConfigurationProfile
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateDeploymentStrategy
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateEnvironment
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateExtension
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateExtensionAssociation
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ValidateConfiguration

Integration tests: 37/45 implemented operations tested (82.2%)
Untested implemented operations: 8

### winterbaume-appconfigdata (appconfigdata) - W: 2/2, S: 0/2, M: 0/2, F: 0/2, K: 0/2, C: 0/2

- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetLatestConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] StartConfigurationSession

Integration tests: 2/2 implemented operations tested (100.0%)

### winterbaume-appfabric (appfabric) - W: 6/26, S: 0/26, M: 0/26, F: 0/26, K: 0/26, C: 0/26

- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] BatchGetUserAccessTasks
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ConnectAppAuthorization
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateAppAuthorization
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateAppBundle
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateIngestion
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateIngestionDestination
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteAppAuthorization
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteAppBundle
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteIngestion
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteIngestionDestination
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetAppAuthorization
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetAppBundle
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetIngestion
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetIngestionDestination
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListAppAuthorizations
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListAppBundles
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListIngestionDestinations
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListIngestions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListTagsForResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartIngestion
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartUserAccessTasks
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StopIngestion
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UntagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateAppAuthorization
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateIngestionDestination

Integration tests: 6/6 implemented operations tested (100.0%)

### winterbaume-appflow (appflow) - W: 9/25, S: 0/25, M: 0/25, F: 0/25, K: 0/25, C: 0/25

- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CancelFlowExecutions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateConnectorProfile
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateFlow
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteConnectorProfile
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteFlow
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeConnector
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeConnectorEntity
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeConnectorProfiles
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeConnectors
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeFlow
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeFlowExecutionRecords
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListConnectorEntities
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListConnectors
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListFlows
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListTagsForResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] RegisterConnector
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ResetConnectorMetadataCache
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] StartFlow
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] StopFlow
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] TagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UnregisterConnector
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UntagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateConnectorProfile
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateConnectorRegistration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateFlow

Integration tests: 6/9 implemented operations tested (66.7%)
Untested implemented operations: 3

### winterbaume-appintegrations (appintegrations) - W: 23/23, S: 0/23, M: 0/23, F: 0/23, K: 0/23, C: 0/23

- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateApplication
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateDataIntegration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateDataIntegrationAssociation
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateEventIntegration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteApplication
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteDataIntegration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteEventIntegration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetApplication
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetDataIntegration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetEventIntegration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListApplicationAssociations
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListApplications
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListDataIntegrationAssociations
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListDataIntegrations
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListEventIntegrationAssociations
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListEventIntegrations
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateApplication
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateDataIntegration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateDataIntegrationAssociation
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateEventIntegration

Integration tests: 21/23 implemented operations tested (91.3%)
Untested implemented operations: 2

### winterbaume-applicationautoscaling (application-auto-scaling) - W: 13/14, S: 1/14, M: 9/14, F: 0/14, K: 0/14, C: 14/14

- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteScalingPolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteScheduledAction
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeregisterScalableTarget
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeScalableTargets
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeScalingActivities
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeScalingPolicies
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeScheduledActions
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] GetPredictiveScalingForecast
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListTagsForResource
- W[x] S[ ] M[x] F[ ] K[ ] C[x] PutScalingPolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[x] PutScheduledAction
- W[x] S[ ] M[x] F[ ] K[ ] C[x] RegisterScalableTarget
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UntagResource

Integration tests: 13/13 implemented operations tested (100.0%)

### winterbaume-applicationcostprofiler (applicationcostprofiler) - W: 6/6, S: 0/6, M: 0/6, F: 0/6, K: 0/6, C: 0/6

- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteReportDefinition
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetReportDefinition
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ImportApplicationUsage
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListReportDefinitions
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutReportDefinition
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateReportDefinition

Integration tests: 6/6 implemented operations tested (100.0%)

### winterbaume-applicationdiscoveryservice (application-discovery-service) - W: 28/28, S: 0/28, M: 0/28, F: 0/28, K: 0/28, C: 0/28

- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] AssociateConfigurationItemsToApplication
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] BatchDeleteAgents
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] BatchDeleteImportData
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateApplication
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateTags
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteApplications
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteTags
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeAgents
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeBatchDeleteConfigurationTask
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeConfigurations
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeContinuousExports
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeExportConfigurations
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeExportTasks
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeImportTasks
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeTags
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DisassociateConfigurationItemsFromApplication
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ExportConfigurations
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetDiscoverySummary
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListConfigurations
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListServerNeighbors
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] StartBatchDeleteConfigurationTask
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] StartContinuousExport
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] StartDataCollectionByAgentIds
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] StartExportTask
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] StartImportTask
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] StopContinuousExport
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] StopDataCollectionByAgentIds
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateApplication

Integration tests: 20/28 implemented operations tested (71.4%)
Untested implemented operations: 8

### winterbaume-applicationinsights (application-insights) - W: 33/33, S: 0/33, M: 0/33, F: 0/33, K: 0/33, C: 0/33

- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] AddWorkload
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateApplication
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateComponent
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateLogPattern
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteApplication
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteComponent
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteLogPattern
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeApplication
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeComponent
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeComponentConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeComponentConfigurationRecommendation
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeLogPattern
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeObservation
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeProblem
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeProblemObservations
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeWorkload
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListApplications
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListComponents
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListConfigurationHistory
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListLogPatternSets
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListLogPatterns
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListProblems
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListWorkloads
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] RemoveWorkload
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateApplication
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateComponent
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateComponentConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateLogPattern
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateProblem
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateWorkload

Integration tests: 26/33 implemented operations tested (78.8%)
Untested implemented operations: 7

### winterbaume-applicationsignals (application-signals) - W: 10/23, S: 3/23, M: 0/23, F: 0/23, K: 0/23, C: 0/23

- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] BatchGetServiceLevelObjectiveBudgetReport
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] BatchUpdateExclusionWindows
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateServiceLevelObjective
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteGroupingConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteServiceLevelObjective
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetService
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetServiceLevelObjective
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] ListAuditFindings
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] ListEntityEvents
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListGroupingAttributeDefinitions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListServiceDependencies
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListServiceDependents
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListServiceLevelObjectiveExclusionWindows
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListServiceLevelObjectives
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListServiceOperations
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] ListServiceStates
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListServices
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutGroupingConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] StartDiscovery
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UntagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateServiceLevelObjective

Integration tests: 8/10 implemented operations tested (80.0%)
Untested implemented operations: 2

### winterbaume-appmesh (app-mesh) - W: 38/38, S: 0/38, M: 0/38, F: 0/38, K: 25/38, C: 0/38

- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateGatewayRoute
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] CreateMesh
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] CreateRoute
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateVirtualGateway
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] CreateVirtualNode
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] CreateVirtualRouter
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] CreateVirtualService
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteGatewayRoute
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] DeleteMesh
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] DeleteRoute
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteVirtualGateway
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] DeleteVirtualNode
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] DeleteVirtualRouter
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] DeleteVirtualService
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeGatewayRoute
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] DescribeMesh
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] DescribeRoute
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeVirtualGateway
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] DescribeVirtualNode
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] DescribeVirtualRouter
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] DescribeVirtualService
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListGatewayRoutes
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] ListMeshes
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] ListRoutes
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListVirtualGateways
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] ListVirtualNodes
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] ListVirtualRouters
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] ListVirtualServices
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateGatewayRoute
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] UpdateMesh
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] UpdateRoute
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateVirtualGateway
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] UpdateVirtualNode
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] UpdateVirtualRouter
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] UpdateVirtualService

Integration tests: 36/38 implemented operations tested (94.7%)
Untested implemented operations: 2

### winterbaume-apprunner (apprunner) - W: 23/37, S: 0/37, M: 0/37, F: 0/37, K: 0/37, C: 0/37

- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AssociateCustomDomain
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateAutoScalingConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateConnection
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateObservabilityConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateService
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateVpcConnector
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateVpcIngressConnection
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteAutoScalingConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteConnection
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteObservabilityConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteService
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteVpcConnector
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteVpcIngressConnection
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeAutoScalingConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeCustomDomains
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeObservabilityConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeService
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeVpcConnector
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeVpcIngressConnection
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DisassociateCustomDomain
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListAutoScalingConfigurations
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListConnections
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListObservabilityConfigurations
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListOperations
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListServices
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListServicesForAutoScalingConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListVpcConnectors
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListVpcIngressConnections
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PauseService
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ResumeService
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] StartDeployment
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateDefaultAutoScalingConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateService
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateVpcIngressConnection

Integration tests: 19/23 implemented operations tested (82.6%)
Untested implemented operations: 4

### winterbaume-appsync (appsync) - W: 27/74, S: 0/74, M: 27/74, F: 0/74, K: 3/74, C: 0/74

- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AssociateApi
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AssociateMergedGraphqlApi
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AssociateSourceGraphqlApi
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateApi
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateApiCache
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateApiKey
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateChannelNamespace
- W[ ] S[ ] M[ ] F[ ] K[x] C[ ] CreateDataSource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateDomainName
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateFunction
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateGraphqlApi
- W[ ] S[ ] M[ ] F[ ] K[x] C[ ] CreateResolver
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateType
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteApi
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteApiCache
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteApiKey
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteChannelNamespace
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteDataSource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteDomainName
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteFunction
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteGraphqlApi
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteResolver
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteType
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DisassociateApi
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DisassociateMergedGraphqlApi
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DisassociateSourceGraphqlApi
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] EvaluateCode
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] EvaluateMappingTemplate
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] FlushApiCache
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetApi
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetApiAssociation
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetApiCache
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetChannelNamespace
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetDataSource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetDataSourceIntrospection
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetDomainName
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetFunction
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetGraphqlApi
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetGraphqlApiEnvironmentVariables
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetIntrospectionSchema
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetResolver
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetSchemaCreationStatus
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetSourceApiAssociation
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetType
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListApiKeys
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListApis
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListChannelNamespaces
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListDataSources
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListDomainNames
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListFunctions
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListGraphqlApis
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListResolvers
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListResolversByFunction
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListSourceApiAssociations
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListTagsForResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListTypes
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListTypesByAssociation
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] PutGraphqlApiEnvironmentVariables
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartDataSourceIntrospection
- W[x] S[ ] M[x] F[ ] K[x] C[ ] StartSchemaCreation
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartSchemaMerge
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] TagResource
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UntagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateApi
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UpdateApiCache
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UpdateApiKey
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateChannelNamespace
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateDataSource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateDomainName
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateFunction
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UpdateGraphqlApi
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateResolver
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateSourceApiAssociation
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateType

Integration tests: 27/27 implemented operations tested (100.0%)

### winterbaume-arczonalshift (arc-zonal-shift) - W: 14/15, S: 1/15, M: 0/15, F: 0/15, K: 0/15, C: 0/15

- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CancelPracticeRun
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CancelZonalShift
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreatePracticeRunConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeletePracticeRunConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetAutoshiftObserverNotificationStatus
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetManagedResource
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] ListAutoshifts
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListManagedResources
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListZonalShifts
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] StartPracticeRun
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] StartZonalShift
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateAutoshiftObserverNotificationStatus
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdatePracticeRunConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateZonalAutoshiftConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateZonalShift

Integration tests: 14/14 implemented operations tested (100.0%)

### winterbaume-artifact (artifact) - W: 8/8, S: 0/8, M: 0/8, F: 0/8, K: 0/8, C: 0/8

- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetAccountSettings
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetReport
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetReportMetadata
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetTermForReport
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListCustomerAgreements
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListReportVersions
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListReports
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutAccountSettings

Integration tests: 5/8 implemented operations tested (62.5%)
Untested implemented operations: 3

### winterbaume-athena (athena) - W: 25/70, S: 0/70, M: 27/70, F: 0/70, K: 7/70, C: 70/70

- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] BatchGetNamedQuery
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] BatchGetPreparedStatement
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] BatchGetQueryExecution
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] CancelCapacityReservation
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateCapacityReservation
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateDataCatalog
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateNamedQuery
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] CreateNotebook
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreatePreparedStatement
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] CreatePresignedNotebookUrl
- W[x] S[ ] M[x] F[ ] K[x] C[x] CreateWorkGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DeleteCapacityReservation
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DeleteDataCatalog
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DeleteNamedQuery
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DeleteNotebook
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DeletePreparedStatement
- W[x] S[ ] M[x] F[ ] K[x] C[x] DeleteWorkGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ExportNotebook
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetCalculationExecution
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetCalculationExecutionCode
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetCalculationExecutionStatus
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetCapacityAssignmentConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetCapacityReservation
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetDataCatalog
- W[ ] S[ ] M[x] F[ ] K[ ] C[x] GetDatabase
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetNamedQuery
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetNotebookMetadata
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetPreparedStatement
- W[x] S[ ] M[x] F[ ] K[x] C[x] GetQueryExecution
- W[x] S[ ] M[x] F[ ] K[x] C[x] GetQueryResults
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetQueryRuntimeStatistics
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetResourceDashboard
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetSession
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetSessionEndpoint
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetSessionStatus
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetTableMetadata
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetWorkGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ImportNotebook
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ListApplicationDPUSizes
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ListCalculationExecutions
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListCapacityReservations
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListDataCatalogs
- W[ ] S[ ] M[x] F[ ] K[ ] C[x] ListDatabases
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ListEngineVersions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ListExecutors
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListNamedQueries
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ListNotebookMetadata
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ListNotebookSessions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ListPreparedStatements
- W[x] S[ ] M[x] F[ ] K[x] C[x] ListQueryExecutions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ListSessions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ListTableMetadata
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListTagsForResource
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListWorkGroups
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] PutCapacityAssignmentConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] StartCalculationExecution
- W[x] S[ ] M[x] F[ ] K[x] C[x] StartQueryExecution
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] StartSession
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] StopCalculationExecution
- W[x] S[ ] M[x] F[ ] K[x] C[x] StopQueryExecution
- W[x] S[ ] M[x] F[ ] K[ ] C[x] TagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] TerminateSession
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UntagResource
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UpdateCapacityReservation
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] UpdateDataCatalog
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] UpdateNamedQuery
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] UpdateNotebook
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] UpdateNotebookMetadata
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] UpdatePreparedStatement
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] UpdateWorkGroup

Integration tests: 25/25 implemented operations tested (100.0%)

### winterbaume-auditmanager (auditmanager) - W: 15/62, S: 0/62, M: 0/62, F: 0/62, K: 0/62, C: 0/62

- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AssociateAssessmentReportEvidenceFolder
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] BatchAssociateAssessmentReportEvidence
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] BatchCreateDelegationByAssessment
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] BatchDeleteDelegationByAssessment
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] BatchDisassociateAssessmentReportEvidence
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] BatchImportEvidenceToAssessmentControl
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateAssessment
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateAssessmentFramework
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateAssessmentReport
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateControl
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteAssessment
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteAssessmentFramework
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteAssessmentFrameworkShare
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteAssessmentReport
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteControl
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeregisterAccount
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeregisterOrganizationAdminAccount
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DisassociateAssessmentReportEvidenceFolder
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetAccountStatus
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetAssessment
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetAssessmentFramework
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetAssessmentReportUrl
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetChangeLogs
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetControl
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetDelegations
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetEvidence
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetEvidenceByEvidenceFolder
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetEvidenceFileUploadUrl
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetEvidenceFolder
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetEvidenceFoldersByAssessment
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetEvidenceFoldersByAssessmentControl
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetInsights
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetInsightsByAssessment
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetOrganizationAdminAccount
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetServicesInScope
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetSettings
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListAssessmentControlInsightsByControlDomain
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListAssessmentFrameworkShareRequests
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListAssessmentFrameworks
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListAssessmentReports
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListAssessments
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListControlDomainInsights
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListControlDomainInsightsByAssessment
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListControlInsightsByControlDomain
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListControls
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListKeywordsForDataSource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListNotifications
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] RegisterAccount
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] RegisterOrganizationAdminAccount
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartAssessmentFrameworkShare
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] TagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UntagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateAssessment
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateAssessmentControl
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateAssessmentControlSetStatus
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateAssessmentFramework
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateAssessmentFrameworkShare
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateAssessmentStatus
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateControl
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateSettings
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ValidateAssessmentReportIntegrity

Integration tests: 15/15 implemented operations tested (100.0%)

### winterbaume-autoscaling (auto-scaling) - W: 52/66, S: 0/66, M: 39/66, F: 0/66, K: 0/66, C: 13/66

Terraform E2E: 10 tests across 5 terraform resource types

Resource types: aws_autoscaling_group, aws_autoscaling_lifecycle_hook, aws_autoscaling_policy, aws_autoscaling_schedule, aws_launch_configuration

- W[x] S[ ] M[x] F[ ] K[ ] C[ ] AttachInstances
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] AttachLoadBalancerTargetGroups
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] AttachLoadBalancers
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AttachTrafficSources
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] BatchDeleteScheduledAction
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] BatchPutScheduledUpdateGroupAction
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CancelInstanceRefresh
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CompleteLifecycleAction
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateAutoScalingGroup
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateLaunchConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateOrUpdateTags
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteAutoScalingGroup
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteLaunchConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteLifecycleHook
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteNotificationConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeletePolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteScheduledAction
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteTags
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteWarmPool
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeAccountLimits
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeAdjustmentTypes
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeAutoScalingGroups
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeAutoScalingInstances
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeAutoScalingNotificationTypes
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeInstanceRefreshes
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeLaunchConfigurations
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeLifecycleHookTypes
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeLifecycleHooks
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeLoadBalancerTargetGroups
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeLoadBalancers
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeMetricCollectionTypes
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeNotificationConfigurations
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribePolicies
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeScalingActivities
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeScalingProcessTypes
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeScheduledActions
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeTags
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeTerminationPolicyTypes
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeTrafficSources
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeWarmPool
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DetachInstances
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DetachLoadBalancerTargetGroups
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DetachLoadBalancers
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DetachTrafficSources
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DisableMetricsCollection
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] EnableMetricsCollection
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] EnterStandby
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ExecutePolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ExitStandby
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetPredictiveScalingForecast
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] LaunchInstances
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutLifecycleHook
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutNotificationConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] PutScalingPolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] PutScheduledUpdateGroupAction
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] PutWarmPool
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] RecordLifecycleActionHeartbeat
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ResumeProcesses
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] RollbackInstanceRefresh
- W[x] S[ ] M[x] F[ ] K[ ] C[x] SetDesiredCapacity
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] SetInstanceHealth
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] SetInstanceProtection
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartInstanceRefresh
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] SuspendProcesses
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] TerminateInstanceInAutoScalingGroup
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UpdateAutoScalingGroup

Integration tests: 49/52 implemented operations tested (94.2%)
Untested implemented operations: 3

### winterbaume-autoscalingplans (auto-scaling-plans) - W: 6/6, S: 0/6, M: 0/6, F: 0/6, K: 0/6, C: 0/6

- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateScalingPlan
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteScalingPlan
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeScalingPlanResources
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeScalingPlans
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetScalingPlanResourceForecastData
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateScalingPlan

Integration tests: 5/6 implemented operations tested (83.3%)
Untested implemented operations: 1

### winterbaume-backup (backup) - W: 105/108, S: 3/108, M: 17/108, F: 0/108, K: 12/108, C: 0/108

- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] AssociateBackupVaultMpaApprovalTeam
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CancelLegalHold
- W[x] S[ ] M[x] F[ ] K[x] C[ ] CreateBackupPlan
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] CreateBackupSelection
- W[x] S[ ] M[x] F[ ] K[x] C[ ] CreateBackupVault
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateFramework
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateLegalHold
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateLogicallyAirGappedBackupVault
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateReportPlan
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateRestoreAccessBackupVault
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateRestoreTestingPlan
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateRestoreTestingSelection
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateTieringConfiguration
- W[x] S[ ] M[x] F[ ] K[x] C[ ] DeleteBackupPlan
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] DeleteBackupSelection
- W[x] S[ ] M[x] F[ ] K[x] C[ ] DeleteBackupVault
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteBackupVaultAccessPolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteBackupVaultLockConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteBackupVaultNotifications
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteFramework
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteRecoveryPoint
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteReportPlan
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteRestoreTestingPlan
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteRestoreTestingSelection
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteTieringConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeBackupJob
- W[x] S[ ] M[x] F[ ] K[x] C[ ] DescribeBackupVault
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeCopyJob
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeFramework
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeGlobalSettings
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeProtectedResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeRecoveryPoint
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeRegionSettings
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeReportJob
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeReportPlan
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeRestoreJob
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeScanJob
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DisassociateBackupVaultMpaApprovalTeam
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DisassociateRecoveryPoint
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DisassociateRecoveryPointFromParent
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ExportBackupPlanTemplate
- W[x] S[ ] M[x] F[ ] K[x] C[ ] GetBackupPlan
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetBackupPlanFromJSON
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] GetBackupPlanFromTemplate
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] GetBackupSelection
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetBackupVaultAccessPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetBackupVaultNotifications
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetLegalHold
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetRecoveryPointIndexDetails
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetRecoveryPointRestoreMetadata
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetRestoreJobMetadata
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetRestoreTestingInferredMetadata
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetRestoreTestingPlan
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetRestoreTestingSelection
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetSupportedResourceTypes
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetTieringConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListBackupJobSummaries
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListBackupJobs
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] ListBackupPlanTemplates
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListBackupPlanVersions
- W[x] S[ ] M[x] F[ ] K[x] C[ ] ListBackupPlans
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] ListBackupSelections
- W[x] S[ ] M[x] F[ ] K[x] C[ ] ListBackupVaults
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListCopyJobSummaries
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListCopyJobs
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListFrameworks
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListIndexedRecoveryPoints
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListLegalHolds
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListProtectedResources
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListProtectedResourcesByBackupVault
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListRecoveryPointsByBackupVault
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListRecoveryPointsByLegalHold
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListRecoveryPointsByResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListReportJobs
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListReportPlans
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] ListRestoreAccessBackupVaults
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListRestoreJobSummaries
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListRestoreJobs
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListRestoreJobsByProtectedResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListRestoreTestingPlans
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListRestoreTestingSelections
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListScanJobSummaries
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListScanJobs
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListTags
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListTieringConfigurations
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutBackupVaultAccessPolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] PutBackupVaultLockConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutBackupVaultNotifications
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutRestoreValidationResult
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] RevokeRestoreAccessBackupVault
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] StartBackupJob
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] StartCopyJob
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] StartReportJob
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] StartRestoreJob
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] StartScanJob
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] StopBackupJob
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] TagResource
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateBackupPlan
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateFramework
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateGlobalSettings
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateRecoveryPointIndexSettings
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateRecoveryPointLifecycle
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateRegionSettings
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateReportPlan
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateRestoreTestingPlan
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateRestoreTestingSelection
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateTieringConfiguration

Integration tests: 54/105 implemented operations tested (51.4%)
Untested implemented operations: 51

### winterbaume-backupgateway (backup-gateway) - W: 25/25, S: 0/25, M: 0/25, F: 0/25, K: 0/25, C: 0/25

- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] AssociateGatewayToServer
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateGateway
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteGateway
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteHypervisor
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DisassociateGatewayFromServer
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetBandwidthRateLimitSchedule
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetGateway
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetHypervisor
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetHypervisorPropertyMappings
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetVirtualMachine
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ImportHypervisorConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListGateways
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListHypervisors
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListVirtualMachines
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutBandwidthRateLimitSchedule
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutHypervisorPropertyMappings
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutMaintenanceStartTime
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] StartVirtualMachinesMetadataSync
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] TestHypervisorConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateGatewayInformation
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateGatewaySoftwareNow
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateHypervisor

Integration tests: 20/25 implemented operations tested (80.0%)
Untested implemented operations: 5

### winterbaume-backupsearch (backupsearch) - W: 9/12, S: 0/12, M: 0/12, F: 0/12, K: 0/12, C: 0/12

- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetSearchJob
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetSearchResultExportJob
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListSearchJobBackups
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListSearchJobResults
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListSearchJobs
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListSearchResultExportJobs
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] StartSearchJob
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartSearchResultExportJob
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] StopSearchJob
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UntagResource

Integration tests: 9/9 implemented operations tested (100.0%)

### winterbaume-batch (batch) - W: 39/45, S: 0/45, M: 24/45, F: 0/45, K: 10/45, C: 45/45

Terraform E2E: 8 tests across 4 terraform resource types

Resource types: aws_batch_compute_environment, aws_batch_job_definition, aws_batch_job_queue, aws_batch_scheduling_policy

- W[x] S[ ] M[x] F[ ] K[ ] C[x] CancelJob
- W[x] S[ ] M[x] F[ ] K[x] C[x] CreateComputeEnvironment
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateConsumableResource
- W[x] S[ ] M[x] F[ ] K[x] C[x] CreateJobQueue
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] CreateQuotaShare
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateSchedulingPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateServiceEnvironment
- W[x] S[ ] M[x] F[ ] K[x] C[x] DeleteComputeEnvironment
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteConsumableResource
- W[x] S[ ] M[x] F[ ] K[x] C[x] DeleteJobQueue
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DeleteQuotaShare
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteSchedulingPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteServiceEnvironment
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeregisterJobDefinition
- W[x] S[ ] M[x] F[ ] K[x] C[x] DescribeComputeEnvironments
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeConsumableResource
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeJobDefinitions
- W[x] S[ ] M[x] F[ ] K[x] C[x] DescribeJobQueues
- W[x] S[ ] M[x] F[ ] K[x] C[x] DescribeJobs
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DescribeQuotaShare
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeSchedulingPolicies
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeServiceEnvironments
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeServiceJob
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetJobQueueSnapshot
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListConsumableResources
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListJobs
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListJobsByConsumableResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ListQuotaShares
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListSchedulingPolicies
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListServiceJobs
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListTagsForResource
- W[x] S[ ] M[x] F[ ] K[x] C[x] RegisterJobDefinition
- W[x] S[ ] M[x] F[ ] K[x] C[x] SubmitJob
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] SubmitServiceJob
- W[x] S[ ] M[x] F[ ] K[ ] C[x] TagResource
- W[x] S[ ] M[x] F[ ] K[x] C[x] TerminateJob
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] TerminateServiceJob
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UntagResource
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UpdateComputeEnvironment
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateConsumableResource
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UpdateJobQueue
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] UpdateQuotaShare
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UpdateSchedulingPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateServiceEnvironment
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] UpdateServiceJob

Integration tests: 39/39 implemented operations tested (100.0%)

### winterbaume-bcmdashboards (bcm-dashboards) - W: 9/15, S: 0/15, M: 0/15, F: 0/15, K: 0/15, C: 0/15

- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateDashboard
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateScheduledReport
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteDashboard
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteScheduledReport
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ExecuteScheduledReport
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetDashboard
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetResourcePolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetScheduledReport
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListDashboards
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListScheduledReports
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateDashboard
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateScheduledReport

Integration tests: 9/9 implemented operations tested (100.0%)

### winterbaume-bcmdataexports (bcm-data-exports) - W: 12/12, S: 0/12, M: 0/12, F: 0/12, K: 0/12, C: 0/12

- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateExport
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteExport
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetExecution
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetExport
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetTable
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListExecutions
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListExports
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListTables
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateExport

Integration tests: 10/12 implemented operations tested (83.3%)
Untested implemented operations: 2

### winterbaume-bcmrecommendedactions (bcm-recommended-actions) - W: 1/1, S: 0/1, M: 0/1, F: 0/1, K: 0/1, C: 0/1

- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListRecommendedActions

Integration tests: 1/1 implemented operations tested (100.0%)

### winterbaume-bedrock (bedrock) - W: 48/101, S: 0/101, M: 13/101, F: 0/101, K: 0/101, C: 101/101

- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] BatchDeleteEvaluationJob
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] CancelAutomatedReasoningPolicyBuildWorkflow
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] CreateAutomatedReasoningPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] CreateAutomatedReasoningPolicyTestCase
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] CreateAutomatedReasoningPolicyVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] CreateCustomModel
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] CreateCustomModelDeployment
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateEvaluationJob
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] CreateFoundationModelAgreement
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateGuardrail
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateGuardrailVersion
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateInferenceProfile
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] CreateMarketplaceModelEndpoint
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateModelCopyJob
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateModelCustomizationJob
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateModelImportJob
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateModelInvocationJob
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreatePromptRouter
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateProvisionedModelThroughput
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DeleteAutomatedReasoningPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DeleteAutomatedReasoningPolicyBuildWorkflow
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DeleteAutomatedReasoningPolicyTestCase
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteCustomModel
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DeleteCustomModelDeployment
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DeleteEnforcedGuardrailConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DeleteFoundationModelAgreement
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteGuardrail
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DeleteImportedModel
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteInferenceProfile
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DeleteMarketplaceModelEndpoint
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteModelInvocationLoggingConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeletePromptRouter
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteProvisionedModelThroughput
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DeleteResourcePolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DeregisterMarketplaceModelEndpoint
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ExportAutomatedReasoningPolicyVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetAutomatedReasoningPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetAutomatedReasoningPolicyAnnotations
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetAutomatedReasoningPolicyBuildWorkflow
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetAutomatedReasoningPolicyBuildWorkflowResultAssets
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetAutomatedReasoningPolicyNextScenario
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetAutomatedReasoningPolicyTestCase
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetAutomatedReasoningPolicyTestResult
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetCustomModel
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetCustomModelDeployment
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetEvaluationJob
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetFoundationModel
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetFoundationModelAvailability
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetGuardrail
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetImportedModel
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetInferenceProfile
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetMarketplaceModelEndpoint
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetModelCopyJob
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetModelCustomizationJob
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetModelImportJob
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetModelInvocationJob
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetModelInvocationLoggingConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetPromptRouter
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetProvisionedModelThroughput
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetResourcePolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetUseCaseForModelAccess
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ListAutomatedReasoningPolicies
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ListAutomatedReasoningPolicyBuildWorkflows
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ListAutomatedReasoningPolicyTestCases
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ListAutomatedReasoningPolicyTestResults
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ListCustomModelDeployments
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListCustomModels
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ListEnforcedGuardrailsConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListEvaluationJobs
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ListFoundationModelAgreementOffers
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListFoundationModels
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListGuardrails
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ListImportedModels
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListInferenceProfiles
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ListMarketplaceModelEndpoints
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListModelCopyJobs
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListModelCustomizationJobs
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListModelImportJobs
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListModelInvocationJobs
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListPromptRouters
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListProvisionedModelThroughputs
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListTagsForResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] PutEnforcedGuardrailConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] C[x] PutModelInvocationLoggingConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] PutResourcePolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] PutUseCaseForModelAccess
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] RegisterMarketplaceModelEndpoint
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] StartAutomatedReasoningPolicyBuildWorkflow
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] StartAutomatedReasoningPolicyTestWorkflow
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] StopEvaluationJob
- W[x] S[ ] M[x] F[ ] K[ ] C[x] StopModelCustomizationJob
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] StopModelInvocationJob
- W[x] S[ ] M[x] F[ ] K[ ] C[x] TagResource
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UntagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] UpdateAutomatedReasoningPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] UpdateAutomatedReasoningPolicyAnnotations
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] UpdateAutomatedReasoningPolicyTestCase
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] UpdateCustomModelDeployment
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateGuardrail
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] UpdateMarketplaceModelEndpoint
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateProvisionedModelThroughput

Integration tests: 43/48 implemented operations tested (89.6%)
Untested implemented operations: 5

### winterbaume-bedrockagent (bedrock-agent) - W: 72/72, S: 0/72, M: 11/72, F: 0/72, K: 0/72, C: 72/72

Terraform E2E: 9 tests across 6 terraform resource types

Resource types: aws_bedrockagent_agent, aws_bedrockagent_agent_action_group, aws_bedrockagent_agent_alias, aws_bedrockagent_agent_knowledge_base_association, aws_bedrockagent_data_source, aws_bedrockagent_knowledge_base

- W[x] S[ ] M[ ] F[ ] K[ ] C[x] AssociateAgentCollaborator
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] AssociateAgentKnowledgeBase
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateAgent
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateAgentActionGroup
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateAgentAlias
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateDataSource
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateFlow
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateFlowAlias
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateFlowVersion
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateKnowledgeBase
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreatePrompt
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreatePromptVersion
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteAgent
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteAgentActionGroup
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteAgentAlias
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteAgentVersion
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteDataSource
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteFlow
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteFlowAlias
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteFlowVersion
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteKnowledgeBase
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteKnowledgeBaseDocuments
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeletePrompt
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DisassociateAgentCollaborator
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DisassociateAgentKnowledgeBase
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetAgent
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetAgentActionGroup
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetAgentAlias
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetAgentCollaborator
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetAgentKnowledgeBase
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetAgentVersion
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetDataSource
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetFlow
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetFlowAlias
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetFlowVersion
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetIngestionJob
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetKnowledgeBase
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetKnowledgeBaseDocuments
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetPrompt
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] IngestKnowledgeBaseDocuments
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListAgentActionGroups
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListAgentAliases
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListAgentCollaborators
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListAgentKnowledgeBases
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListAgentVersions
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListAgents
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListDataSources
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListFlowAliases
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListFlowVersions
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListFlows
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListIngestionJobs
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListKnowledgeBaseDocuments
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListKnowledgeBases
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListPrompts
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] PrepareAgent
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] PrepareFlow
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] StartIngestionJob
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] StopIngestionJob
- W[x] S[ ] M[x] F[ ] K[ ] C[x] TagResource
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateAgent
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateAgentActionGroup
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateAgentAlias
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateAgentCollaborator
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateAgentKnowledgeBase
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateDataSource
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateFlow
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateFlowAlias
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateKnowledgeBase
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdatePrompt
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ValidateFlowDefinition

Integration tests: 71/72 implemented operations tested (98.6%)
Untested implemented operations: 1

### winterbaume-billing (billing) - W: 12/12, S: 0/12, M: 0/12, F: 0/12, K: 0/12, C: 0/12

- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] AssociateSourceViews
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateBillingView
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteBillingView
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DisassociateSourceViews
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetBillingView
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetResourcePolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListBillingViews
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListSourceViewsForBillingView
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateBillingView

Integration tests: 11/12 implemented operations tested (91.7%)
Untested implemented operations: 1

### winterbaume-braket (braket) - W: 12/17, S: 0/17, M: 0/17, F: 0/17, K: 0/17, C: 0/17

- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CancelJob
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CancelQuantumTask
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateJob
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateQuantumTask
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateSpendingLimit
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteSpendingLimit
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetDevice
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetJob
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetQuantumTask
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] SearchDevices
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] SearchJobs
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] SearchQuantumTasks
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] SearchSpendingLimits
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateSpendingLimit

Integration tests: 8/12 implemented operations tested (66.7%)
Untested implemented operations: 4

### winterbaume-budgets (budgets) - W: 7/26, S: 0/26, M: 7/26, F: 0/26, K: 0/26, C: 0/26

- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateBudget
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateBudgetAction
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateNotification
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateSubscriber
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteBudget
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteBudgetAction
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteNotification
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteSubscriber
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeBudget
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeBudgetAction
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeBudgetActionHistories
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeBudgetActionsForAccount
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeBudgetActionsForBudget
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeBudgetNotificationsForAccount
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeBudgetPerformanceHistory
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeBudgets
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeNotificationsForBudget
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeSubscribersForNotification
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ExecuteBudgetAction
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListTagsForResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] TagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UntagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateBudget
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateBudgetAction
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateNotification
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateSubscriber

Integration tests: 7/7 implemented operations tested (100.0%)

### winterbaume-chatbot (chatbot) - W: 15/34, S: 0/34, M: 0/34, F: 0/34, K: 0/34, C: 0/34

- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AssociateToConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateChimeWebhookConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateCustomAction
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateMicrosoftTeamsChannelConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateSlackChannelConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteChimeWebhookConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteCustomAction
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteMicrosoftTeamsChannelConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteMicrosoftTeamsConfiguredTeam
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteMicrosoftTeamsUserIdentity
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteSlackChannelConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteSlackUserIdentity
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteSlackWorkspaceAuthorization
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeChimeWebhookConfigurations
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeSlackChannelConfigurations
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeSlackUserIdentities
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeSlackWorkspaces
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DisassociateFromConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetAccountPreferences
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetCustomAction
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetMicrosoftTeamsChannelConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListAssociations
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListCustomActions
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListMicrosoftTeamsChannelConfigurations
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListMicrosoftTeamsConfiguredTeams
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListMicrosoftTeamsUserIdentities
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UntagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateAccountPreferences
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateChimeWebhookConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateCustomAction
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateMicrosoftTeamsChannelConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateSlackChannelConfiguration

Integration tests: 15/15 implemented operations tested (100.0%)

### winterbaume-chimesdkmeetings (chime-sdk-meetings) - W: 12/16, S: 0/16, M: 0/16, F: 0/16, K: 0/16, C: 0/16

- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] BatchCreateAttendee
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] BatchUpdateAttendeeCapabilitiesExcept
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateAttendee
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateMeeting
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateMeetingWithAttendees
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteAttendee
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteMeeting
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetAttendee
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetMeeting
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListAttendees
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListTagsForResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartMeetingTranscription
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StopMeetingTranscription
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateAttendeeCapabilities

Integration tests: 8/12 implemented operations tested (66.7%)
Untested implemented operations: 4

### winterbaume-cloud9 (cloud9) - W: 13/13, S: 0/13, M: 0/13, F: 0/13, K: 0/13, C: 0/13

- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateEnvironmentEC2
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateEnvironmentMembership
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteEnvironment
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteEnvironmentMembership
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeEnvironmentMemberships
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeEnvironmentStatus
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeEnvironments
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListEnvironments
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateEnvironment
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateEnvironmentMembership

Integration tests: 13/13 implemented operations tested (100.0%)

### winterbaume-cloudcontrol (cloudcontrol) - W: 8/8, S: 0/8, M: 0/8, F: 0/8, K: 6/8, C: 8/8

- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CancelResourceRequest
- W[x] S[ ] M[ ] F[ ] K[x] C[x] CreateResource
- W[x] S[ ] M[ ] F[ ] K[x] C[x] DeleteResource
- W[x] S[ ] M[ ] F[ ] K[x] C[x] GetResource
- W[x] S[ ] M[ ] F[ ] K[x] C[x] GetResourceRequestStatus
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListResourceRequests
- W[x] S[ ] M[ ] F[ ] K[x] C[x] ListResources
- W[x] S[ ] M[ ] F[ ] K[x] C[x] UpdateResource

Integration tests: 8/8 implemented operations tested (100.0%)

### winterbaume-clouddirectory (clouddirectory) - W: 13/66, S: 0/66, M: 13/66, F: 0/66, K: 0/66, C: 0/66

- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AddFacetToObject
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ApplySchema
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AttachObject
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AttachPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AttachToIndex
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AttachTypedLink
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] BatchRead
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] BatchWrite
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateDirectory
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateFacet
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateIndex
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateObject
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateSchema
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateTypedLinkFacet
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteDirectory
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteFacet
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteObject
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteSchema
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteTypedLinkFacet
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DetachFromIndex
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DetachObject
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DetachPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DetachTypedLink
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DisableDirectory
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] EnableDirectory
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetAppliedSchemaVersion
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetDirectory
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetFacet
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetLinkAttributes
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetObjectAttributes
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetObjectInformation
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetSchemaAsJson
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetTypedLinkFacetInformation
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListAppliedSchemaArns
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListAttachedIndices
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListDevelopmentSchemaArns
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListDirectories
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListFacetAttributes
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListFacetNames
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListIncomingTypedLinks
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListIndex
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListManagedSchemaArns
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListObjectAttributes
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListObjectChildren
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListObjectParentPaths
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListObjectParents
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListObjectPolicies
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListOutgoingTypedLinks
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListPolicyAttachments
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListPublishedSchemaArns
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListTagsForResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListTypedLinkFacetAttributes
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListTypedLinkFacetNames
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] LookupPolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] PublishSchema
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] PutSchemaFromJson
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] RemoveFacetFromObject
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] TagResource
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UntagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateFacet
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateLinkAttributes
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateObjectAttributes
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateSchema
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateTypedLinkFacet
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpgradeAppliedSchema
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpgradePublishedSchema

Integration tests: 12/13 implemented operations tested (92.3%)
Untested implemented operations: 1

### winterbaume-cloudformation (cloudformation) - W: 40/90, S: 3/90, M: 33/90, F: 0/90, K: 8/90, C: 90/90

- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ActivateOrganizationsAccess
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ActivateType
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] BatchDescribeTypeConfigurations
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CancelUpdateStack
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ContinueUpdateRollback
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateChangeSet
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] CreateGeneratedTemplate
- W[x] S[ ] M[x] F[ ] K[x] C[x] CreateStack
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateStackInstances
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] CreateStackRefactor
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateStackSet
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DeactivateOrganizationsAccess
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DeactivateType
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteChangeSet
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DeleteGeneratedTemplate
- W[x] S[ ] M[x] F[ ] K[x] C[x] DeleteStack
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteStackInstances
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteStackSet
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DeregisterType
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] DescribeAccountLimits
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeChangeSet
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DescribeChangeSetHooks
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DescribeEvents
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DescribeGeneratedTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DescribeOrganizationsAccess
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DescribePublisher
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DescribeResourceScan
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DescribeStackDriftDetectionStatus
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeStackEvents
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeStackInstance
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DescribeStackRefactor
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeStackResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DescribeStackResourceDrifts
- W[x] S[ ] M[x] F[ ] K[x] C[x] DescribeStackResources
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeStackSet
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeStackSetOperation
- W[x] S[ ] M[x] F[ ] K[x] C[x] DescribeStacks
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DescribeType
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DescribeTypeRegistration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DetectStackDrift
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DetectStackResourceDrift
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DetectStackSetDrift
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] EstimateTemplateCost
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ExecuteChangeSet
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ExecuteStackRefactor
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetGeneratedTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetHookResult
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetStackPolicy
- W[x] S[ ] M[x] F[ ] K[x] C[x] GetTemplate
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetTemplateSummary
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ImportStacksToStackSet
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListChangeSets
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListExports
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ListGeneratedTemplates
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ListHookResults
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListImports
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ListResourceScanRelatedResources
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ListResourceScanResources
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ListResourceScans
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ListStackInstanceResourceDrifts
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListStackInstances
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ListStackRefactorActions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ListStackRefactors
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListStackResources
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ListStackSetAutoDeploymentTargets
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListStackSetOperationResults
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListStackSetOperations
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListStackSets
- W[x] S[ ] M[x] F[ ] K[x] C[x] ListStacks
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ListTypeRegistrations
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ListTypeVersions
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListTypes
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] PublishType
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] RecordHandlerProgress
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] RegisterPublisher
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] RegisterType
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] RollbackStack
- W[x] S[ ] M[x] F[ ] K[ ] C[x] SetStackPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] SetTypeConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] SetTypeDefaultVersion
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] SignalResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] StartResourceScan
- W[x] S[ ] M[x] F[ ] K[ ] C[x] StopStackSetOperation
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] TestType
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] UpdateGeneratedTemplate
- W[x] S[ ] M[x] F[ ] K[x] C[x] UpdateStack
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UpdateStackInstances
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UpdateStackSet
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateTerminationProtection
- W[ ] S[x] M[x] F[ ] K[x] C[x] ValidateTemplate

Integration tests: 39/40 implemented operations tested (97.5%)
Untested implemented operations: 1

### winterbaume-cloudfront (cloudfront) - W: 156/167, S: 11/167, M: 25/167, F: 0/167, K: 16/167, C: 167/167

Terraform E2E: 6 tests across 2 terraform resource types

Resource types: aws_cloudfront_distribution, aws_cloudfront_origin_access_control

- W[x] S[ ] M[ ] F[ ] K[ ] C[x] AssociateAlias
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] AssociateDistributionTenantWebACL
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] AssociateDistributionWebACL
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CopyDistribution
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateAnycastIpList
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateCachePolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateCloudFrontOriginAccessIdentity
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateConnectionFunction
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateConnectionGroup
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateContinuousDeploymentPolicy
- W[x] S[ ] M[x] F[ ] K[x] C[x] CreateDistribution
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateDistributionTenant
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateDistributionWithTags
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateFieldLevelEncryptionConfig
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateFieldLevelEncryptionProfile
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateFunction
- W[x] S[ ] M[x] F[ ] K[x] C[x] CreateInvalidation
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateInvalidationForDistributionTenant
- W[x] S[ ] M[x] F[ ] K[x] C[x] CreateKeyGroup
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateKeyValueStore
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateMonitoringSubscription
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateOriginAccessControl
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateOriginRequestPolicy
- W[x] S[ ] M[x] F[ ] K[x] C[x] CreatePublicKey
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateRealtimeLogConfig
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateResponseHeadersPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateStreamingDistribution
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateStreamingDistributionWithTags
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateTrustStore
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateVpcOrigin
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteAnycastIpList
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteCachePolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteCloudFrontOriginAccessIdentity
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteConnectionFunction
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteConnectionGroup
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteContinuousDeploymentPolicy
- W[x] S[ ] M[x] F[ ] K[x] C[x] DeleteDistribution
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteDistributionTenant
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteFieldLevelEncryptionConfig
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteFieldLevelEncryptionProfile
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteFunction
- W[x] S[ ] M[ ] F[ ] K[x] C[x] DeleteKeyGroup
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteKeyValueStore
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteMonitoringSubscription
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteOriginAccessControl
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteOriginRequestPolicy
- W[x] S[ ] M[x] F[ ] K[x] C[x] DeletePublicKey
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteRealtimeLogConfig
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteResourcePolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteResponseHeadersPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteStreamingDistribution
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteTrustStore
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteVpcOrigin
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeConnectionFunction
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeFunction
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeKeyValueStore
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DisassociateDistributionTenantWebACL
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DisassociateDistributionWebACL
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetAnycastIpList
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetCachePolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetCachePolicyConfig
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetCloudFrontOriginAccessIdentity
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetCloudFrontOriginAccessIdentityConfig
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetConnectionFunction
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetConnectionGroup
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetConnectionGroupByRoutingEndpoint
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetContinuousDeploymentPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetContinuousDeploymentPolicyConfig
- W[x] S[ ] M[x] F[ ] K[x] C[x] GetDistribution
- W[x] S[ ] M[x] F[ ] K[x] C[x] GetDistributionConfig
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetDistributionTenant
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetDistributionTenantByDomain
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetFieldLevelEncryption
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetFieldLevelEncryptionConfig
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetFieldLevelEncryptionProfile
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetFieldLevelEncryptionProfileConfig
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetFunction
- W[x] S[ ] M[x] F[ ] K[x] C[x] GetInvalidation
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetInvalidationForDistributionTenant
- W[x] S[ ] M[x] F[ ] K[x] C[x] GetKeyGroup
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetKeyGroupConfig
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] GetManagedCertificateDetails
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetMonitoringSubscription
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetOriginAccessControl
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetOriginAccessControlConfig
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetOriginRequestPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetOriginRequestPolicyConfig
- W[x] S[ ] M[x] F[ ] K[x] C[x] GetPublicKey
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetPublicKeyConfig
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetRealtimeLogConfig
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetResourcePolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetResponseHeadersPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetResponseHeadersPolicyConfig
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetStreamingDistribution
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetStreamingDistributionConfig
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetTrustStore
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetVpcOrigin
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListAnycastIpLists
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListCachePolicies
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListCloudFrontOriginAccessIdentities
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] ListConflictingAliases
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListConnectionFunctions
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListConnectionGroups
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListContinuousDeploymentPolicies
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListDistributionTenants
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListDistributionTenantsByCustomization
- W[x] S[ ] M[x] F[ ] K[x] C[x] ListDistributions
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListDistributionsByAnycastIpListId
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListDistributionsByCachePolicyId
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] ListDistributionsByConnectionFunction
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] ListDistributionsByConnectionMode
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListDistributionsByKeyGroup
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListDistributionsByOriginRequestPolicyId
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] ListDistributionsByOwnedResource
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] ListDistributionsByRealtimeLogConfig
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListDistributionsByResponseHeadersPolicyId
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] ListDistributionsByTrustStore
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListDistributionsByVpcOriginId
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListDistributionsByWebACLId
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] ListDomainConflicts
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListFieldLevelEncryptionConfigs
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListFieldLevelEncryptionProfiles
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListFunctions
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListInvalidations
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListInvalidationsForDistributionTenant
- W[x] S[ ] M[x] F[ ] K[x] C[x] ListKeyGroups
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListKeyValueStores
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListOriginAccessControls
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListOriginRequestPolicies
- W[x] S[ ] M[x] F[ ] K[x] C[x] ListPublicKeys
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListRealtimeLogConfigs
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListResponseHeadersPolicies
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListStreamingDistributions
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListTrustStores
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListVpcOrigins
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] PublishConnectionFunction
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] PublishFunction
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] PutResourcePolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[x] TagResource
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] TestConnectionFunction
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] TestFunction
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateAnycastIpList
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateCachePolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateCloudFrontOriginAccessIdentity
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateConnectionFunction
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateConnectionGroup
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateContinuousDeploymentPolicy
- W[x] S[ ] M[x] F[ ] K[x] C[x] UpdateDistribution
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateDistributionTenant
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateDistributionWithStagingConfig
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateDomainAssociation
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateFieldLevelEncryptionConfig
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateFieldLevelEncryptionProfile
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateFunction
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateKeyGroup
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateKeyValueStore
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UpdateOriginAccessControl
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateOriginRequestPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdatePublicKey
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateRealtimeLogConfig
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateResponseHeadersPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateStreamingDistribution
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateTrustStore
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateVpcOrigin
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] VerifyDnsConfiguration

Integration tests: 90/156 implemented operations tested (57.7%)
Untested implemented operations: 66

### winterbaume-cloudfrontkeyvaluestore (cloudfront-keyvaluestore) - W: 5/6, S: 0/6, M: 0/6, F: 0/6, K: 0/6, C: 0/6

- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteKey
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeKeyValueStore
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetKey
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListKeys
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutKey
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateKeys

Integration tests: 5/5 implemented operations tested (100.0%)

### winterbaume-cloudhsmv2 (cloudhsm-v2) - W: 18/18, S: 0/18, M: 0/18, F: 0/18, K: 0/18, C: 0/18

- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CopyBackupToRegion
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateCluster
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateHsm
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteBackup
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteCluster
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteHsm
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteResourcePolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeBackups
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeClusters
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetResourcePolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] InitializeCluster
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListTags
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ModifyBackupAttributes
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ModifyCluster
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutResourcePolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] RestoreBackup
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UntagResource

Integration tests: 18/18 implemented operations tested (100.0%)

### winterbaume-cloudsearchdomain (cloudsearch-domain) - W: 2/3, S: 0/3, M: 0/3, F: 0/3, K: 0/3, C: 0/3

- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] Search
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] Suggest
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UploadDocuments

Integration tests: 2/2 implemented operations tested (100.0%)

### winterbaume-cloudtrail (cloudtrail) - W: 21/60, S: 2/60, M: 16/60, F: 0/60, K: 8/60, C: 0/60

- W[x] S[ ] M[x] F[ ] K[ ] C[ ] AddTags
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CancelQuery
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateChannel
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateDashboard
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateEventDataStore
- W[x] S[ ] M[x] F[ ] K[x] C[ ] CreateTrail
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteChannel
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteDashboard
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteEventDataStore
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteResourcePolicy
- W[x] S[ ] M[x] F[ ] K[x] C[ ] DeleteTrail
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeregisterOrganizationDelegatedAdmin
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeQuery
- W[x] S[ ] M[x] F[ ] K[x] C[ ] DescribeTrails
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DisableFederation
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] EnableFederation
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GenerateQuery
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetChannel
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetDashboard
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetEventConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetEventDataStore
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetEventSelectors
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetImport
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetInsightSelectors
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetQueryResults
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetResourcePolicy
- W[x] S[ ] M[x] F[ ] K[x] C[ ] GetTrail
- W[x] S[ ] M[x] F[ ] K[x] C[ ] GetTrailStatus
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListChannels
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListDashboards
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListEventDataStores
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListImportFailures
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListImports
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListInsightsData
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListInsightsMetricData
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] ListPublicKeys
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListQueries
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListTags
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListTrails
- W[ ] S[x] M[ ] F[ ] K[x] C[ ] LookupEvents
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] PutEventConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] PutEventSelectors
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] PutInsightSelectors
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] PutResourcePolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] RegisterOrganizationDelegatedAdmin
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] RemoveTags
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] RestoreEventDataStore
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] SearchSampleQueries
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartDashboardRefresh
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartEventDataStoreIngestion
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartImport
- W[x] S[ ] M[x] F[ ] K[x] C[ ] StartLogging
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartQuery
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StopEventDataStoreIngestion
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StopImport
- W[x] S[ ] M[x] F[ ] K[x] C[ ] StopLogging
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateChannel
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateDashboard
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateEventDataStore
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UpdateTrail

Integration tests: 16/21 implemented operations tested (76.2%)
Untested implemented operations: 5

### winterbaume-cloudtraildata (cloudtrail-data) - W: 1/1, S: 0/1, M: 0/1, F: 0/1, K: 0/1, C: 0/1

- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutAuditEvents

Integration tests: 1/1 implemented operations tested (100.0%)

### winterbaume-cloudwatch (cloudwatch) - W: 38/46, S: 5/46, M: 20/46, F: 0/46, K: 11/46, C: 46/46

Terraform E2E: 12 tests across 3 terraform resource types

Resource types: aws_cloudwatch_dashboard, aws_cloudwatch_metric_alarm, aws_cloudwatch_metric_stream

- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteAlarmMuteRule
- W[x] S[ ] M[x] F[ ] K[x] C[x] DeleteAlarms
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteAnomalyDetector
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteDashboards
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteInsightRules
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteMetricStream
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] DescribeAlarmContributors
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] DescribeAlarmHistory
- W[x] S[ ] M[x] F[ ] K[x] C[x] DescribeAlarms
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeAlarmsForMetric
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeAnomalyDetectors
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeInsightRules
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DisableAlarmActions
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DisableInsightRules
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] EnableAlarmActions
- W[x] S[ ] M[x] F[ ] K[ ] C[x] EnableInsightRules
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetAlarmMuteRule
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetDashboard
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] GetInsightRuleReport
- W[x] S[ ] M[x] F[ ] K[x] C[x] GetMetricData
- W[ ] S[x] M[x] F[ ] K[x] C[x] GetMetricStatistics
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetMetricStream
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] GetMetricWidgetImage
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetOTelEnrichment
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListAlarmMuteRules
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListDashboards
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListManagedInsightRules
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListMetricStreams
- W[x] S[ ] M[x] F[ ] K[x] C[x] ListMetrics
- W[x] S[ ] M[x] F[ ] K[x] C[x] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] PutAlarmMuteRule
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] PutAnomalyDetector
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] PutCompositeAlarm
- W[x] S[ ] M[x] F[ ] K[ ] C[x] PutDashboard
- W[x] S[ ] M[x] F[ ] K[ ] C[x] PutInsightRule
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] PutManagedInsightRules
- W[x] S[ ] M[x] F[ ] K[x] C[x] PutMetricAlarm
- W[x] S[ ] M[x] F[ ] K[x] C[x] PutMetricData
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] PutMetricStream
- W[x] S[ ] M[x] F[ ] K[x] C[x] SetAlarmState
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] StartMetricStreams
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] StartOTelEnrichment
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] StopMetricStreams
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] StopOTelEnrichment
- W[x] S[ ] M[x] F[ ] K[x] C[x] TagResource
- W[x] S[ ] M[x] F[ ] K[x] C[x] UntagResource

Integration tests: 29/38 implemented operations tested (76.3%)
Untested implemented operations: 9

### winterbaume-cloudwatchlogs (cloudwatch-logs) - W: 93/113, S: 15/113, M: 52/113, F: 0/113, K: 11/113, C: 113/113

Terraform E2E: 5 tests across 3 terraform resource types

Resource types: aws_cloudwatch_log_group, aws_cloudwatch_log_metric_filter, aws_cloudwatch_log_resource_policy

- W[x] S[ ] M[ ] F[ ] K[ ] C[x] AssociateKmsKey
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] AssociateSourceToS3TableIntegration
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CancelExportTask
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CancelImportTask
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateDelivery
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateExportTask
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateImportTask
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateLogAnomalyDetector
- W[x] S[ ] M[x] F[ ] K[x] C[x] CreateLogGroup
- W[x] S[ ] M[x] F[ ] K[x] C[x] CreateLogStream
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] CreateLookupTable
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateScheduledQuery
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteAccountPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteDataProtectionPolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteDelivery
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteDeliveryDestination
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteDeliveryDestinationPolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteDeliverySource
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteDestination
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteIndexPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteIntegration
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteLogAnomalyDetector
- W[x] S[ ] M[x] F[ ] K[x] C[x] DeleteLogGroup
- W[x] S[ ] M[x] F[ ] K[x] C[x] DeleteLogStream
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DeleteLookupTable
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteMetricFilter
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteQueryDefinition
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteResourcePolicy
- W[x] S[ ] M[x] F[ ] K[x] C[x] DeleteRetentionPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteScheduledQuery
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteSubscriptionFilter
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteTransformer
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeAccountPolicies
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeConfigurationTemplates
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeDeliveries
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeDeliveryDestinations
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeDeliverySources
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeDestinations
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeExportTasks
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] DescribeFieldIndexes
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] DescribeImportTaskBatches
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeImportTasks
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeIndexPolicies
- W[x] S[ ] M[x] F[ ] K[x] C[x] DescribeLogGroups
- W[x] S[ ] M[x] F[ ] K[x] C[x] DescribeLogStreams
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DescribeLookupTables
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeMetricFilters
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeQueries
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeQueryDefinitions
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeResourcePolicies
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeSubscriptionFilters
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DisassociateKmsKey
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] DisassociateSourceFromS3TableIntegration
- W[x] S[ ] M[x] F[ ] K[x] C[x] FilterLogEvents
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetDataProtectionPolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetDelivery
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetDeliveryDestination
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetDeliveryDestinationPolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetDeliverySource
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetIntegration
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetLogAnomalyDetector
- W[x] S[ ] M[x] F[ ] K[x] C[x] GetLogEvents
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] GetLogFields
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] GetLogGroupFields
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] GetLogObject
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] GetLogRecord
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetLookupTable
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetQueryResults
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetScheduledQuery
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] GetScheduledQueryHistory
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetTransformer
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] ListAggregateLogGroupSummaries
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListAnomalies
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListIntegrations
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListLogAnomalyDetectors
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListLogGroups
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] ListLogGroupsForQuery
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListScheduledQueries
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] ListSourcesForS3TableIntegration
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListTagsForResource
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListTagsLogGroup
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] PutAccountPolicy
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] PutBearerTokenAuthentication
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] PutDataProtectionPolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[x] PutDeliveryDestination
- W[x] S[ ] M[x] F[ ] K[ ] C[x] PutDeliveryDestinationPolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[x] PutDeliverySource
- W[x] S[ ] M[x] F[ ] K[ ] C[x] PutDestination
- W[x] S[ ] M[x] F[ ] K[ ] C[x] PutDestinationPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] PutIndexPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] PutIntegration
- W[x] S[ ] M[x] F[ ] K[x] C[x] PutLogEvents
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] PutLogGroupDeletionProtection
- W[x] S[ ] M[x] F[ ] K[ ] C[x] PutMetricFilter
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] PutQueryDefinition
- W[x] S[ ] M[x] F[ ] K[ ] C[x] PutResourcePolicy
- W[x] S[ ] M[x] F[ ] K[x] C[x] PutRetentionPolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[x] PutSubscriptionFilter
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] PutTransformer
- W[ ] S[x] M[x] F[ ] K[ ] C[x] StartLiveTail
- W[x] S[ ] M[x] F[ ] K[ ] C[x] StartQuery
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] StopQuery
- W[x] S[ ] M[x] F[ ] K[ ] C[x] TagLogGroup
- W[x] S[ ] M[x] F[ ] K[ ] C[x] TagResource
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] TestMetricFilter
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] TestTransformer
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UntagLogGroup
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateAnomaly
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateDeliveryConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateLogAnomalyDetector
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] UpdateLookupTable
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateScheduledQuery

Integration tests: 84/93 implemented operations tested (90.3%)
Untested implemented operations: 9

### winterbaume-codeartifact (codeartifact) - W: 9/48, S: 0/48, M: 0/48, F: 0/48, K: 0/48, C: 0/48

- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AssociateExternalConnection
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CopyPackageVersions
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateDomain
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreatePackageGroup
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateRepository
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteDomain
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteDomainPermissionsPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeletePackage
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeletePackageGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeletePackageVersions
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteRepository
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteRepositoryPermissionsPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeDomain
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribePackage
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribePackageGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribePackageVersion
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeRepository
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DisassociateExternalConnection
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DisposePackageVersions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetAssociatedPackageGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetAuthorizationToken
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetDomainPermissionsPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetPackageVersionAsset
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetPackageVersionReadme
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetRepositoryEndpoint
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetRepositoryPermissionsPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListAllowedRepositoriesForGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListAssociatedPackages
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListDomains
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListPackageGroups
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListPackageVersionAssets
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListPackageVersionDependencies
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListPackageVersions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListPackages
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListRepositories
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListRepositoriesInDomain
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListSubPackageGroups
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListTagsForResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] PublishPackageVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] PutDomainPermissionsPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] PutPackageOriginConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] PutRepositoryPermissionsPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] TagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UntagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdatePackageGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdatePackageGroupOriginConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdatePackageVersionsStatus
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateRepository

Integration tests: 9/9 implemented operations tested (100.0%)

### winterbaume-codebuild (codebuild) - W: 29/59, S: 0/59, M: 9/59, F: 0/59, K: 0/59, C: 0/59

- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] BatchDeleteBuilds
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] BatchGetBuildBatches
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] BatchGetBuilds
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] BatchGetCommandExecutions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] BatchGetFleets
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] BatchGetProjects
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] BatchGetReportGroups
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] BatchGetReports
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] BatchGetSandboxes
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateFleet
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateProject
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateReportGroup
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateWebhook
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteBuildBatch
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteFleet
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteProject
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteReport
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteReportGroup
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteResourcePolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteSourceCredentials
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteWebhook
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeCodeCoverages
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeTestCases
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetReportGroupTrend
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetResourcePolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ImportSourceCredentials
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] InvalidateProjectCache
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListBuildBatches
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListBuildBatchesForProject
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListBuilds
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListBuildsForProject
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListCommandExecutionsForSandbox
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListCuratedEnvironmentImages
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListFleets
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListProjects
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListReportGroups
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListReports
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListReportsForReportGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListSandboxes
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListSandboxesForProject
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListSharedProjects
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListSharedReportGroups
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListSourceCredentials
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutResourcePolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] RetryBuild
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] RetryBuildBatch
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] StartBuild
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartBuildBatch
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartCommandExecution
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartSandbox
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartSandboxConnection
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] StopBuild
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StopBuildBatch
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StopSandbox
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateFleet
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateProject
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateProjectVisibility
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateReportGroup
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateWebhook

Integration tests: 26/29 implemented operations tested (89.7%)
Untested implemented operations: 3

### winterbaume-codecommit (codecommit) - W: 25/79, S: 0/79, M: 3/79, F: 0/79, K: 0/79, C: 0/79

Terraform E2E: 4 tests across 1 terraform resource types

Resource types: aws_codecommit_repository

- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AssociateApprovalRuleTemplateWithRepository
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] BatchAssociateApprovalRuleTemplateWithRepositories
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] BatchDescribeMergeConflicts
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] BatchDisassociateApprovalRuleTemplateFromRepositories
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] BatchGetCommits
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] BatchGetRepositories
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateApprovalRuleTemplate
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateBranch
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateCommit
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreatePullRequest
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreatePullRequestApprovalRule
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateRepository
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateUnreferencedMergeCommit
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteApprovalRuleTemplate
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteBranch
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteCommentContent
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteFile
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeletePullRequestApprovalRule
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteRepository
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeMergeConflicts
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribePullRequestEvents
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DisassociateApprovalRuleTemplateFromRepository
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] EvaluatePullRequestApprovalRules
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetApprovalRuleTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetBlob
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetBranch
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetComment
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetCommentReactions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetCommentsForComparedCommit
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetCommentsForPullRequest
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetCommit
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetDifferences
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetFile
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetFolder
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetMergeCommit
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetMergeConflicts
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetMergeOptions
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetPullRequest
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetPullRequestApprovalStates
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetPullRequestOverrideState
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetRepository
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetRepositoryTriggers
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListApprovalRuleTemplates
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListAssociatedApprovalRuleTemplatesForRepository
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListBranches
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListFileCommitHistory
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListPullRequests
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListRepositories
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListRepositoriesForApprovalRuleTemplate
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListTagsForResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] MergeBranchesByFastForward
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] MergeBranchesBySquash
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] MergeBranchesByThreeWay
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] MergePullRequestByFastForward
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] MergePullRequestBySquash
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] MergePullRequestByThreeWay
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] OverridePullRequestApprovalRules
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] PostCommentForComparedCommit
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] PostCommentForPullRequest
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] PostCommentReply
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] PutCommentReaction
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutFile
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] PutRepositoryTriggers
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] TagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] TestRepositoryTriggers
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UntagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateApprovalRuleTemplateContent
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateApprovalRuleTemplateDescription
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateApprovalRuleTemplateName
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateComment
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateDefaultBranch
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdatePullRequestApprovalRuleContent
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdatePullRequestApprovalState
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdatePullRequestDescription
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdatePullRequestStatus
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdatePullRequestTitle
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateRepositoryDescription
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateRepositoryEncryptionKey
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateRepositoryName

Integration tests: 25/25 implemented operations tested (100.0%)

### winterbaume-codedeploy (codedeploy) - W: 15/47, S: 0/47, M: 14/47, F: 0/47, K: 0/47, C: 0/47

- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AddTagsToOnPremisesInstances
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] BatchGetApplicationRevisions
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] BatchGetApplications
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] BatchGetDeploymentGroups
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] BatchGetDeploymentInstances
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] BatchGetDeploymentTargets
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] BatchGetDeployments
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] BatchGetOnPremisesInstances
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ContinueDeployment
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateApplication
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateDeployment
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateDeploymentConfig
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateDeploymentGroup
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteApplication
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteDeploymentConfig
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteDeploymentGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteGitHubAccountToken
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteResourcesByExternalId
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeregisterOnPremisesInstance
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetApplication
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetApplicationRevision
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetDeployment
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetDeploymentConfig
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetDeploymentGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetDeploymentInstance
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetDeploymentTarget
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetOnPremisesInstance
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListApplicationRevisions
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListApplications
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListDeploymentConfigs
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListDeploymentGroups
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListDeploymentInstances
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListDeploymentTargets
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListDeployments
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListGitHubAccountTokenNames
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListOnPremisesInstances
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListTagsForResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] PutLifecycleEventHookExecutionStatus
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] RegisterApplicationRevision
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] RegisterOnPremisesInstance
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] RemoveTagsFromOnPremisesInstances
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] SkipWaitTimeForInstanceTermination
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StopDeployment
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] TagResource
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UntagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateApplication
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateDeploymentGroup

Integration tests: 15/15 implemented operations tested (100.0%)

### winterbaume-codegurureviewer (codeguru-reviewer) - W: 9/14, S: 0/14, M: 0/14, F: 0/14, K: 11/14, C: 0/14

- W[x] S[ ] M[ ] F[ ] K[x] C[ ] AssociateRepository
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] CreateCodeReview
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] DescribeCodeReview
- W[ ] S[ ] M[ ] F[ ] K[x] C[ ] DescribeRecommendationFeedback
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] DescribeRepositoryAssociation
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] DisassociateRepository
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] ListCodeReviews
- W[ ] S[ ] M[ ] F[ ] K[x] C[ ] ListRecommendationFeedback
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] ListRecommendations
- W[ ] S[ ] M[ ] F[ ] K[x] C[ ] ListRepositoryAssociations
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListTagsForResource
- W[ ] S[ ] M[ ] F[ ] K[x] C[ ] PutRecommendationFeedback
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UntagResource

Integration tests: 9/9 implemented operations tested (100.0%)

### winterbaume-codegurusecurity (codeguru-security) - W: 11/13, S: 0/13, M: 0/13, F: 0/13, K: 0/13, C: 0/13

- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] BatchGetFindings
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateScan
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateUploadUrl
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetAccountConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetFindings
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetMetricsSummary
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetScan
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListFindingsMetrics
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListScans
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateAccountConfiguration

Integration tests: 9/11 implemented operations tested (81.8%)
Untested implemented operations: 2

### winterbaume-codepipeline (codepipeline) - W: 44/44, S: 0/44, M: 8/44, F: 0/44, K: 0/44, C: 0/44

- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] AcknowledgeJob
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] AcknowledgeThirdPartyJob
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateCustomActionType
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreatePipeline
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteCustomActionType
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeletePipeline
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteWebhook
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeregisterWebhookWithThirdParty
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DisableStageTransition
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] EnableStageTransition
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetActionType
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetJobDetails
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetPipeline
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetPipelineExecution
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetPipelineState
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetThirdPartyJobDetails
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListActionExecutions
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListActionTypes
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListDeployActionExecutionTargets
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListPipelineExecutions
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListPipelines
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListRuleExecutions
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListRuleTypes
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListWebhooks
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] OverrideStageCondition
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PollForJobs
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PollForThirdPartyJobs
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutActionRevision
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutApprovalResult
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutJobFailureResult
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutJobSuccessResult
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutThirdPartyJobFailureResult
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutThirdPartyJobSuccessResult
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutWebhook
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] RegisterWebhookWithThirdParty
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] RetryStageExecution
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] RollbackStage
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] StartPipelineExecution
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] StopPipelineExecution
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] TagResource
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateActionType
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UpdatePipeline

Integration tests: 8/44 implemented operations tested (18.2%)
Untested implemented operations: 36

### winterbaume-codestarnotifications (codestar-notifications) - W: 7/13, S: 0/13, M: 0/13, F: 0/13, K: 0/13, C: 0/13

- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateNotificationRule
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteNotificationRule
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteTarget
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeNotificationRule
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListEventTypes
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListNotificationRules
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListTargets
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] Subscribe
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] Unsubscribe
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UntagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateNotificationRule

Integration tests: 5/7 implemented operations tested (71.4%)
Untested implemented operations: 2

### winterbaume-cognitoidentity (cognito-identity) - W: 20/23, S: 3/23, M: 10/23, F: 0/23, K: 0/23, C: 23/23

- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateIdentityPool
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteIdentities
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteIdentityPool
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeIdentity
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeIdentityPool
- W[ ] S[x] M[x] F[ ] K[ ] C[x] GetCredentialsForIdentity
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetId
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetIdentityPoolRoles
- W[ ] S[x] M[x] F[ ] K[ ] C[x] GetOpenIdToken
- W[ ] S[x] M[x] F[ ] K[ ] C[x] GetOpenIdTokenForDeveloperIdentity
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetPrincipalTagAttributeMap
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListIdentities
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListIdentityPools
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] LookupDeveloperIdentity
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] MergeDeveloperIdentities
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] SetIdentityPoolRoles
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] SetPrincipalTagAttributeMap
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UnlinkDeveloperIdentity
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UnlinkIdentity
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UntagResource
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UpdateIdentityPool

Integration tests: 18/20 implemented operations tested (90.0%)
Untested implemented operations: 2

### winterbaume-cognitoidentityprovider (cognito-identity-provider) - W: 104/122, S: 18/122, M: 62/122, F: 0/122, K: 17/122, C: 122/122

Terraform E2E: 15 tests across 7 terraform resource types

Resource types: aws_cognito_identity_provider, aws_cognito_resource_server, aws_cognito_user, aws_cognito_user_group, aws_cognito_user_pool, aws_cognito_user_pool_client, aws_cognito_user_pool_domain

- W[x] S[ ] M[x] F[ ] K[ ] C[x] AddCustomAttributes
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] AddUserPoolClientSecret
- W[x] S[ ] M[x] F[ ] K[ ] C[x] AdminAddUserToGroup
- W[x] S[ ] M[x] F[ ] K[ ] C[x] AdminConfirmSignUp
- W[x] S[ ] M[x] F[ ] K[x] C[x] AdminCreateUser
- W[x] S[ ] M[x] F[ ] K[x] C[x] AdminDeleteUser
- W[x] S[ ] M[x] F[ ] K[ ] C[x] AdminDeleteUserAttributes
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] AdminDisableProviderForUser
- W[x] S[ ] M[x] F[ ] K[ ] C[x] AdminDisableUser
- W[x] S[ ] M[x] F[ ] K[ ] C[x] AdminEnableUser
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] AdminForgetDevice
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] AdminGetDevice
- W[x] S[ ] M[x] F[ ] K[x] C[x] AdminGetUser
- W[x] S[ ] M[x] F[ ] K[ ] C[x] AdminInitiateAuth
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] AdminLinkProviderForUser
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] AdminListDevices
- W[x] S[ ] M[x] F[ ] K[ ] C[x] AdminListGroupsForUser
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] AdminListUserAuthEvents
- W[x] S[ ] M[x] F[ ] K[ ] C[x] AdminRemoveUserFromGroup
- W[x] S[ ] M[x] F[ ] K[ ] C[x] AdminResetUserPassword
- W[x] S[ ] M[x] F[ ] K[ ] C[x] AdminRespondToAuthChallenge
- W[x] S[ ] M[x] F[ ] K[ ] C[x] AdminSetUserMFAPreference
- W[x] S[ ] M[x] F[ ] K[ ] C[x] AdminSetUserPassword
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] AdminSetUserSettings
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] AdminUpdateAuthEventFeedback
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] AdminUpdateDeviceStatus
- W[x] S[ ] M[x] F[ ] K[ ] C[x] AdminUpdateUserAttributes
- W[x] S[ ] M[x] F[ ] K[ ] C[x] AdminUserGlobalSignOut
- W[ ] S[x] M[x] F[ ] K[ ] C[x] AssociateSoftwareToken
- W[ ] S[x] M[x] F[ ] K[ ] C[x] ChangePassword
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] CompleteWebAuthnRegistration
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ConfirmDevice
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ConfirmForgotPassword
- W[x] S[ ] M[x] F[ ] K[x] C[x] ConfirmSignUp
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateGroup
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateIdentityProvider
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateManagedLoginBranding
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateResourceServer
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateTerms
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateUserImportJob
- W[x] S[ ] M[x] F[ ] K[x] C[x] CreateUserPool
- W[x] S[ ] M[x] F[ ] K[x] C[x] CreateUserPoolClient
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateUserPoolDomain
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteGroup
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteIdentityProvider
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteManagedLoginBranding
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteResourceServer
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteTerms
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] DeleteUser
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteUserAttributes
- W[x] S[ ] M[x] F[ ] K[x] C[x] DeleteUserPool
- W[x] S[ ] M[x] F[ ] K[x] C[x] DeleteUserPoolClient
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteUserPoolClientSecret
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteUserPoolDomain
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteWebAuthnCredential
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeIdentityProvider
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeManagedLoginBranding
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeManagedLoginBrandingByClient
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeResourceServer
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeRiskConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeTerms
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeUserImportJob
- W[x] S[ ] M[x] F[ ] K[x] C[x] DescribeUserPool
- W[x] S[ ] M[x] F[ ] K[x] C[x] DescribeUserPoolClient
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeUserPoolDomain
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ForgetDevice
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ForgotPassword
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetCSVHeader
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetDevice
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetGroup
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetIdentityProviderByIdentifier
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetLogDeliveryConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetSigningCertificate
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] GetTokensFromRefreshToken
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetUICustomization
- W[ ] S[x] M[x] F[ ] K[ ] C[x] GetUser
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] GetUserAttributeVerificationCode
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] GetUserAuthFactors
- W[x] S[ ] M[x] F[ ] K[x] C[x] GetUserPoolMfaConfig
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GlobalSignOut
- W[x] S[ ] M[x] F[ ] K[x] C[x] InitiateAuth
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListDevices
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListGroups
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListIdentityProviders
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListResourceServers
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListTerms
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListUserImportJobs
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListUserPoolClientSecrets
- W[x] S[ ] M[x] F[ ] K[x] C[x] ListUserPoolClients
- W[x] S[ ] M[x] F[ ] K[x] C[x] ListUserPools
- W[x] S[ ] M[x] F[ ] K[x] C[x] ListUsers
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListUsersInGroup
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListWebAuthnCredentials
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ResendConfirmationCode
- W[x] S[ ] M[x] F[ ] K[ ] C[x] RespondToAuthChallenge
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] RevokeToken
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] SetLogDeliveryConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] SetRiskConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] SetUICustomization
- W[ ] S[x] M[x] F[ ] K[ ] C[x] SetUserMFAPreference
- W[x] S[ ] M[x] F[ ] K[x] C[x] SetUserPoolMfaConfig
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] SetUserSettings
- W[x] S[ ] M[x] F[ ] K[x] C[x] SignUp
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] StartUserImportJob
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] StartWebAuthnRegistration
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] StopUserImportJob
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UntagResource
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] UpdateAuthEventFeedback
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateDeviceStatus
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UpdateGroup
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UpdateIdentityProvider
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateManagedLoginBranding
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateResourceServer
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateTerms
- W[ ] S[x] M[x] F[ ] K[ ] C[x] UpdateUserAttributes
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UpdateUserPool
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UpdateUserPoolClient
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UpdateUserPoolDomain
- W[ ] S[x] M[x] F[ ] K[ ] C[x] VerifySoftwareToken
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] VerifyUserAttribute

Integration tests: 76/104 implemented operations tested (73.1%)
Untested implemented operations: 28

### winterbaume-cognitosync (cognito-sync) - W: 11/17, S: 0/17, M: 0/17, F: 0/17, K: 0/17, C: 0/17

- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] BulkPublish
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteDataset
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeDataset
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeIdentityPoolUsage
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeIdentityUsage
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetBulkPublishDetails
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetCognitoEvents
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetIdentityPoolConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListDatasets
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListIdentityPoolUsage
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListRecords
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] RegisterDevice
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] SetCognitoEvents
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] SetIdentityPoolConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] SubscribeToDataset
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UnsubscribeFromDataset
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateRecords

Integration tests: 6/11 implemented operations tested (54.5%)
Untested implemented operations: 5

### winterbaume-comprehend (comprehend) - W: 60/85, S: 5/85, M: 63/85, F: 0/85, K: 12/85, C: 0/85

- W[ ] S[ ] M[ ] F[ ] K[x] C[ ] BatchDetectDominantLanguage
- W[ ] S[ ] M[ ] F[ ] K[x] C[ ] BatchDetectEntities
- W[ ] S[ ] M[ ] F[ ] K[x] C[ ] BatchDetectKeyPhrases
- W[ ] S[ ] M[ ] F[ ] K[x] C[ ] BatchDetectSentiment
- W[ ] S[ ] M[ ] F[ ] K[x] C[ ] BatchDetectSyntax
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] BatchDetectTargetedSentiment
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ClassifyDocument
- W[ ] S[ ] M[ ] F[ ] K[x] C[ ] ContainsPiiEntities
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateDataset
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateDocumentClassifier
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateEndpoint
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateEntityRecognizer
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateFlywheel
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteDocumentClassifier
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteEndpoint
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteEntityRecognizer
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteFlywheel
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteResourcePolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeDataset
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeDocumentClassificationJob
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeDocumentClassifier
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeDominantLanguageDetectionJob
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeEndpoint
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeEntitiesDetectionJob
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeEntityRecognizer
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeEventsDetectionJob
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeFlywheel
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeFlywheelIteration
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeKeyPhrasesDetectionJob
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribePiiEntitiesDetectionJob
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeResourcePolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeSentimentDetectionJob
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeTargetedSentimentDetectionJob
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeTopicsDetectionJob
- W[ ] S[x] M[ ] F[ ] K[x] C[ ] DetectDominantLanguage
- W[ ] S[x] M[ ] F[ ] K[x] C[ ] DetectEntities
- W[ ] S[x] M[x] F[ ] K[x] C[ ] DetectKeyPhrases
- W[ ] S[x] M[x] F[ ] K[x] C[ ] DetectPiiEntities
- W[ ] S[x] M[x] F[ ] K[x] C[ ] DetectSentiment
- W[ ] S[ ] M[ ] F[ ] K[x] C[ ] DetectSyntax
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DetectTargetedSentiment
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DetectToxicContent
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ImportModel
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListDatasets
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListDocumentClassificationJobs
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListDocumentClassifierSummaries
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListDocumentClassifiers
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListDominantLanguageDetectionJobs
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListEndpoints
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListEntitiesDetectionJobs
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListEntityRecognizerSummaries
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListEntityRecognizers
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListEventsDetectionJobs
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListFlywheelIterationHistory
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListFlywheels
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListKeyPhrasesDetectionJobs
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListPiiEntitiesDetectionJobs
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListSentimentDetectionJobs
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListTagsForResource
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListTargetedSentimentDetectionJobs
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListTopicsDetectionJobs
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] PutResourcePolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] StartDocumentClassificationJob
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] StartDominantLanguageDetectionJob
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] StartEntitiesDetectionJob
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] StartEventsDetectionJob
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] StartFlywheelIteration
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] StartKeyPhrasesDetectionJob
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] StartPiiEntitiesDetectionJob
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] StartSentimentDetectionJob
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] StartTargetedSentimentDetectionJob
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] StartTopicsDetectionJob
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] StopDominantLanguageDetectionJob
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] StopEntitiesDetectionJob
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] StopEventsDetectionJob
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] StopKeyPhrasesDetectionJob
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] StopPiiEntitiesDetectionJob
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] StopSentimentDetectionJob
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] StopTargetedSentimentDetectionJob
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] StopTrainingDocumentClassifier
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] StopTrainingEntityRecognizer
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] TagResource
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UntagResource
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UpdateEndpoint
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateFlywheel

Integration tests: 60/60 implemented operations tested (100.0%)

### winterbaume-config (config-service) - W: 46/97, S: 3/97, M: 38/97, F: 0/97, K: 9/97, C: 0/97

Terraform E2E: 7 tests across 7 terraform resource types

Resource types: aws_config_aggregate_authorization, aws_config_config_rule, aws_config_configuration_aggregator, aws_config_configuration_recorder, aws_config_configuration_recorder_status, aws_config_delivery_channel, aws_config_retention_configuration

- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AssociateResourceTypes
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] BatchGetAggregateResourceConfig
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] BatchGetResourceConfig
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteAggregationAuthorization
- W[x] S[ ] M[x] F[ ] K[x] C[ ] DeleteConfigRule
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteConfigurationAggregator
- W[x] S[ ] M[x] F[ ] K[x] C[ ] DeleteConfigurationRecorder
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteConformancePack
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteDeliveryChannel
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteEvaluationResults
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteOrganizationConfigRule
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteOrganizationConformancePack
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeletePendingAggregationRequest
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteRemediationConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteRemediationExceptions
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteResourceConfig
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteRetentionConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteServiceLinkedConfigurationRecorder
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteStoredQuery
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeliverConfigSnapshot
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeAggregateComplianceByConfigRules
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeAggregateComplianceByConformancePacks
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeAggregationAuthorizations
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeComplianceByConfigRule
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeComplianceByResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeConfigRuleEvaluationStatus
- W[x] S[ ] M[x] F[ ] K[x] C[ ] DescribeConfigRules
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeConfigurationAggregatorSourcesStatus
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeConfigurationAggregators
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeConfigurationRecorderStatus
- W[x] S[ ] M[x] F[ ] K[x] C[ ] DescribeConfigurationRecorders
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeConformancePackCompliance
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeConformancePackStatus
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeConformancePacks
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeDeliveryChannelStatus
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeDeliveryChannels
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeOrganizationConfigRuleStatuses
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeOrganizationConfigRules
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeOrganizationConformancePackStatuses
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeOrganizationConformancePacks
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribePendingAggregationRequests
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeRemediationConfigurations
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeRemediationExceptions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeRemediationExecutionStatus
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeRetentionConfigurations
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DisassociateResourceTypes
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetAggregateComplianceDetailsByConfigRule
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetAggregateConfigRuleComplianceSummary
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetAggregateConformancePackComplianceSummary
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetAggregateDiscoveredResourceCounts
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetAggregateResourceConfig
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] GetComplianceDetailsByConfigRule
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetComplianceDetailsByResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetComplianceSummaryByConfigRule
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetComplianceSummaryByResourceType
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetConformancePackComplianceDetails
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetConformancePackComplianceSummary
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetCustomRulePolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetDiscoveredResourceCounts
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetOrganizationConfigRuleDetailedStatus
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetOrganizationConformancePackDetailedStatus
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetOrganizationCustomRulePolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetResourceConfigHistory
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetResourceEvaluationSummary
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetStoredQuery
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListAggregateDiscoveredResources
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListConfigurationRecorders
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListConformancePackComplianceScores
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListDiscoveredResources
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListResourceEvaluations
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListStoredQueries
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListTagsForResource
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] PutAggregationAuthorization
- W[x] S[ ] M[x] F[ ] K[x] C[ ] PutConfigRule
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] PutConfigurationAggregator
- W[x] S[ ] M[x] F[ ] K[x] C[ ] PutConfigurationRecorder
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] PutConformancePack
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] PutDeliveryChannel
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] PutEvaluations
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] PutExternalEvaluation
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutOrganizationConfigRule
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] PutOrganizationConformancePack
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutRemediationConfigurations
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] PutRemediationExceptions
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] PutResourceConfig
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] PutRetentionConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] PutServiceLinkedConfigurationRecorder
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] PutStoredQuery
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] SelectAggregateResourceConfig
- W[ ] S[x] M[x] F[ ] K[ ] C[ ] SelectResourceConfig
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] StartConfigRulesEvaluation
- W[x] S[ ] M[x] F[ ] K[x] C[ ] StartConfigurationRecorder
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] StartRemediationExecution
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartResourceEvaluation
- W[x] S[ ] M[x] F[ ] K[x] C[ ] StopConfigurationRecorder
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] TagResource
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UntagResource

Integration tests: 46/46 implemented operations tested (100.0%)

### winterbaume-connect (connect) - W: 10/370, S: 0/370, M: 10/370, F: 0/370, K: 0/370, C: 0/370

- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ActivateEvaluationForm
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] AssociateAnalyticsDataSet
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AssociateApprovedOrigin
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AssociateBot
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AssociateContactWithUser
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AssociateDefaultVocabulary
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AssociateEmailAddressAlias
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AssociateFlow
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AssociateHoursOfOperations
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AssociateInstanceStorageConfig
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AssociateLambdaFunction
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AssociateLexBot
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AssociatePhoneNumberContactFlow
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AssociateQueueEmailAddresses
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AssociateQueueQuickConnects
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AssociateRoutingProfileQueues
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AssociateSecurityKey
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AssociateSecurityProfiles
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AssociateTrafficDistributionGroupUser
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AssociateUserProficiencies
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AssociateWorkspace
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] BatchAssociateAnalyticsDataSet
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] BatchCreateDataTableValue
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] BatchDeleteDataTableValue
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] BatchDescribeDataTableValue
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] BatchDisassociateAnalyticsDataSet
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] BatchGetAttachedFileMetadata
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] BatchGetFlowAssociation
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] BatchPutContact
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] BatchUpdateDataTableValue
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ClaimPhoneNumber
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CompleteAttachedFileUpload
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateAgentStatus
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateContact
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateContactFlow
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateContactFlowModule
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateContactFlowModuleAlias
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateContactFlowModuleVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateContactFlowVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateDataTable
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateDataTableAttribute
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateEmailAddress
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateEvaluationForm
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateHoursOfOperation
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateHoursOfOperationOverride
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateInstance
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateIntegrationAssociation
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateNotification
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateParticipant
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreatePersistentContactAssociation
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreatePredefinedAttribute
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreatePrompt
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreatePushNotificationRegistration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateQueue
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateQuickConnect
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateRoutingProfile
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateRule
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateSecurityProfile
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateTaskTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateTestCase
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateTrafficDistributionGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateUseCase
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateUser
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateUserHierarchyGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateView
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateViewVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateVocabulary
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateWorkspace
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateWorkspacePage
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeactivateEvaluationForm
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteAttachedFile
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteContactEvaluation
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteContactFlow
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteContactFlowModule
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteContactFlowModuleAlias
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteContactFlowModuleVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteContactFlowVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteDataTable
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteDataTableAttribute
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteEmailAddress
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteEvaluationForm
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteHoursOfOperation
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteHoursOfOperationOverride
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteInstance
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteIntegrationAssociation
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteNotification
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeletePredefinedAttribute
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeletePrompt
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeletePushNotificationRegistration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteQueue
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteQuickConnect
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteRoutingProfile
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteRule
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteSecurityProfile
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteTaskTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteTestCase
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteTrafficDistributionGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteUseCase
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteUser
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteUserHierarchyGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteView
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteViewVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteVocabulary
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteWorkspace
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteWorkspaceMedia
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteWorkspacePage
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeAgentStatus
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeAttachedFilesConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeAuthenticationProfile
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeContact
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeContactEvaluation
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeContactFlow
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeContactFlowModule
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeContactFlowModuleAlias
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeDataTable
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeDataTableAttribute
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeEmailAddress
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeEvaluationForm
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeHoursOfOperation
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeHoursOfOperationOverride
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeInstance
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeInstanceAttribute
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeInstanceStorageConfig
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeNotification
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribePhoneNumber
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribePredefinedAttribute
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribePrompt
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeQueue
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeQuickConnect
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeRoutingProfile
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeRule
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeSecurityProfile
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeTestCase
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeTrafficDistributionGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeUser
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeUserHierarchyGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeUserHierarchyStructure
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeView
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeVocabulary
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeWorkspace
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DisassociateAnalyticsDataSet
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DisassociateApprovedOrigin
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DisassociateBot
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DisassociateEmailAddressAlias
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DisassociateFlow
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DisassociateHoursOfOperations
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DisassociateInstanceStorageConfig
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DisassociateLambdaFunction
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DisassociateLexBot
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DisassociatePhoneNumberContactFlow
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DisassociateQueueEmailAddresses
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DisassociateQueueQuickConnects
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DisassociateRoutingProfileQueues
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DisassociateSecurityKey
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DisassociateSecurityProfiles
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DisassociateTrafficDistributionGroupUser
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DisassociateUserProficiencies
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DisassociateWorkspace
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DismissUserContact
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] EvaluateDataTableValues
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetAttachedFile
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetContactAttributes
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetContactMetrics
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetCurrentMetricData
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetCurrentUserData
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetEffectiveHoursOfOperations
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetFederationToken
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetFlowAssociation
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetMetricData
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetMetricDataV2
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetPromptFile
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetTaskTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetTestCaseExecutionSummary
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetTrafficDistribution
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ImportPhoneNumber
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ImportWorkspaceMedia
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListAgentStatuses
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListAnalyticsDataAssociations
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListAnalyticsDataLakeDataSets
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListApprovedOrigins
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListAssociatedContacts
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListAttachedFilesConfigurations
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListAuthenticationProfiles
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListBots
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListChildHoursOfOperations
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListContactEvaluations
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListContactFlowModuleAliases
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListContactFlowModuleVersions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListContactFlowModules
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListContactFlowVersions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListContactFlows
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListContactReferences
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListDataTableAttributes
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListDataTablePrimaryValues
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListDataTableValues
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListDataTables
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListDefaultVocabularies
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListEntitySecurityProfiles
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListEvaluationFormVersions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListEvaluationForms
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListFlowAssociations
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListHoursOfOperationOverrides
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListHoursOfOperations
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListInstanceAttributes
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListInstanceStorageConfigs
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListInstances
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListIntegrationAssociations
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListLambdaFunctions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListLexBots
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListNotifications
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListPhoneNumbers
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListPhoneNumbersV2
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListPredefinedAttributes
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListPrompts
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListQueueEmailAddresses
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListQueueQuickConnects
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListQueues
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListQuickConnects
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListRealtimeContactAnalysisSegmentsV2
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListRoutingProfileManualAssignmentQueues
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListRoutingProfileQueues
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListRoutingProfiles
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListRules
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListSecurityKeys
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListSecurityProfileApplications
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListSecurityProfileFlowModules
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListSecurityProfilePermissions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListSecurityProfiles
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListTagsForResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListTaskTemplates
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListTestCaseExecutionRecords
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListTestCaseExecutions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListTestCases
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListTrafficDistributionGroupUsers
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListTrafficDistributionGroups
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListUseCases
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListUserHierarchyGroups
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListUserNotifications
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListUserProficiencies
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListUsers
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListViewVersions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListViews
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListWorkspaceMedia
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListWorkspacePages
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListWorkspaces
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] MonitorContact
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] PauseContact
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] PutUserStatus
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ReleasePhoneNumber
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ReplicateInstance
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ResumeContact
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ResumeContactRecording
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] SearchAgentStatuses
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] SearchAvailablePhoneNumbers
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] SearchContactEvaluations
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] SearchContactFlowModules
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] SearchContactFlows
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] SearchContacts
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] SearchDataTables
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] SearchEmailAddresses
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] SearchEvaluationForms
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] SearchHoursOfOperationOverrides
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] SearchHoursOfOperations
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] SearchNotifications
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] SearchPredefinedAttributes
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] SearchPrompts
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] SearchQueues
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] SearchQuickConnects
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] SearchResourceTags
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] SearchRoutingProfiles
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] SearchSecurityProfiles
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] SearchTestCases
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] SearchUserHierarchyGroups
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] SearchUsers
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] SearchViews
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] SearchVocabularies
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] SearchWorkspaceAssociations
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] SearchWorkspaces
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] SendChatIntegrationEvent
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] SendOutboundEmail
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartAttachedFileUpload
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartChatContact
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartContactEvaluation
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartContactMediaProcessing
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartContactRecording
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartContactStreaming
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartEmailContact
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartOutboundChatContact
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartOutboundEmailContact
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartOutboundVoiceContact
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartScreenSharing
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartTaskContact
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartTestCaseExecution
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartWebRTCContact
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StopContact
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StopContactMediaProcessing
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StopContactRecording
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StopContactStreaming
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StopTestCaseExecution
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] SubmitContactEvaluation
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] SuspendContactRecording
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] TagContact
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] TagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] TransferContact
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UntagContact
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UntagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateAgentStatus
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateAttachedFilesConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateAuthenticationProfile
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateContact
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateContactAttributes
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateContactEvaluation
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateContactFlowContent
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateContactFlowMetadata
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateContactFlowModuleAlias
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateContactFlowModuleContent
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateContactFlowModuleMetadata
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateContactFlowName
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateContactRoutingData
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateContactSchedule
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateDataTableAttribute
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateDataTableMetadata
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateDataTablePrimaryValues
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateEmailAddressMetadata
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateEvaluationForm
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateHoursOfOperation
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateHoursOfOperationOverride
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateInstanceAttribute
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateInstanceStorageConfig
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateNotificationContent
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateParticipantAuthentication
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateParticipantRoleConfig
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdatePhoneNumber
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdatePhoneNumberMetadata
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdatePredefinedAttribute
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdatePrompt
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateQueueHoursOfOperation
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateQueueMaxContacts
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateQueueName
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateQueueOutboundCallerConfig
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateQueueOutboundEmailConfig
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateQueueStatus
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateQuickConnectConfig
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateQuickConnectName
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateRoutingProfileAgentAvailabilityTimer
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateRoutingProfileConcurrency
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateRoutingProfileDefaultOutboundQueue
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateRoutingProfileName
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateRoutingProfileQueues
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateRule
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateSecurityProfile
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateTaskTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateTestCase
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateTrafficDistribution
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateUserConfig
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateUserHierarchy
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateUserHierarchyGroupName
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateUserHierarchyStructure
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateUserIdentityInfo
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateUserNotificationStatus
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateUserPhoneConfig
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateUserProficiencies
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateUserRoutingProfile
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateUserSecurityProfiles
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateViewContent
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateViewMetadata
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateWorkspaceMetadata
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateWorkspacePage
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateWorkspaceTheme
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateWorkspaceVisibility

Integration tests: 10/10 implemented operations tested (100.0%)

### winterbaume-connectcampaigns (connectcampaigns) - W: 14/22, S: 0/22, M: 14/22, F: 0/22, K: 0/22, C: 0/22

- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateCampaign
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteCampaign
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteConnectInstanceConfig
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteInstanceOnboardingJob
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeCampaign
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetCampaignState
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetCampaignStateBatch
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetConnectInstanceConfig
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetInstanceOnboardingJobStatus
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListCampaigns
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListTagsForResource
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] PauseCampaign
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] PutDialRequestBatch
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ResumeCampaign
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] StartCampaign
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] StartInstanceOnboardingJob
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] StopCampaign
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] TagResource
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UntagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateCampaignDialerConfig
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateCampaignName
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateCampaignOutboundCallConfig

Integration tests: 14/14 implemented operations tested (100.0%)

### winterbaume-connectcontactlens (connect-contact-lens) - W: 0/1, S: 0/1, M: 0/1, F: 0/1, K: 0/1, C: 0/1

- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListRealtimeContactAnalysisSegments

Integration tests: 0/0 implemented operations tested (0.0%)

### winterbaume-connectparticipant (connectparticipant) - W: 7/11, S: 0/11, M: 0/11, F: 0/11, K: 0/11, C: 0/11

- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CancelParticipantAuthentication
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CompleteAttachmentUpload
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateParticipantConnection
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeView
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DisconnectParticipant
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetAttachment
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetAuthenticationUrl
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetTranscript
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] SendEvent
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] SendMessage
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] StartAttachmentUpload

Integration tests: 5/7 implemented operations tested (71.4%)
Untested implemented operations: 2

### winterbaume-controlcatalog (controlcatalog) - W: 6/6, S: 0/6, M: 0/6, F: 0/6, K: 0/6, C: 0/6

- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetControl
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListCommonControls
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListControlMappings
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListControls
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListDomains
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListObjectives

Integration tests: 5/6 implemented operations tested (83.3%)
Untested implemented operations: 1

### winterbaume-costandusagereport (cost-and-usage-report-service) - W: 7/7, S: 0/7, M: 0/7, F: 0/7, K: 0/7, C: 0/7

- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteReportDefinition
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeReportDefinitions
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ModifyReportDefinition
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutReportDefinition
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UntagResource

Integration tests: 7/7 implemented operations tested (100.0%)

### winterbaume-costexplorer (cost-explorer) - W: 22/47, S: 25/47, M: 0/47, F: 0/47, K: 8/47, C: 0/47

- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateAnomalyMonitor
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateAnomalySubscription
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] CreateCostCategoryDefinition
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteAnomalyMonitor
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteAnomalySubscription
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] DeleteCostCategoryDefinition
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] DescribeCostCategoryDefinition
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] GetAnomalies
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetAnomalyMonitors
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetAnomalySubscriptions
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] GetApproximateUsageRecords
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] GetCommitmentPurchaseAnalysis
- W[ ] S[x] M[ ] F[ ] K[x] C[ ] GetCostAndUsage
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] GetCostAndUsageComparisons
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] GetCostAndUsageWithResources
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] GetCostCategories
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] GetCostComparisonDrivers
- W[ ] S[x] M[ ] F[ ] K[x] C[ ] GetCostForecast
- W[ ] S[x] M[ ] F[ ] K[x] C[ ] GetDimensionValues
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] GetReservationCoverage
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] GetReservationPurchaseRecommendation
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] GetReservationUtilization
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] GetRightsizingRecommendation
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] GetSavingsPlanPurchaseRecommendationDetails
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] GetSavingsPlansCoverage
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] GetSavingsPlansPurchaseRecommendation
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] GetSavingsPlansUtilization
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] GetSavingsPlansUtilizationDetails
- W[ ] S[x] M[ ] F[ ] K[x] C[ ] GetTags
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] GetUsageForecast
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] ListCommitmentPurchaseAnalyses
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListCostAllocationTagBackfillHistory
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListCostAllocationTags
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] ListCostCategoryDefinitions
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListCostCategoryResourceAssociations
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] ListSavingsPlansPurchaseRecommendationGeneration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ProvideAnomalyFeedback
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] StartCommitmentPurchaseAnalysis
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] StartCostAllocationTagBackfill
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] StartSavingsPlansPurchaseRecommendationGeneration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateAnomalyMonitor
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateAnomalySubscription
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateCostAllocationTagsStatus
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateCostCategoryDefinition

Integration tests: 21/22 implemented operations tested (95.5%)
Untested implemented operations: 1

### winterbaume-costoptimizationhub (cost-optimization-hub) - W: 8/8, S: 0/8, M: 0/8, F: 0/8, K: 0/8, C: 0/8

- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetPreferences
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetRecommendation
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListEfficiencyMetrics
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListEnrollmentStatuses
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListRecommendationSummaries
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListRecommendations
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateEnrollmentStatus
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdatePreferences

Integration tests: 8/8 implemented operations tested (100.0%)

### winterbaume-databasemigration (database-migration-service) - W: 42/119, S: 0/119, M: 17/119, F: 0/119, K: 0/119, C: 0/119

- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] AddTagsToResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ApplyPendingMaintenanceAction
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] BatchStartRecommendations
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CancelMetadataModelConversion
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CancelMetadataModelCreation
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CancelReplicationTaskAssessmentRun
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateDataMigration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateDataProvider
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateEndpoint
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateEventSubscription
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateFleetAdvisorCollector
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateInstanceProfile
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateMigrationProject
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateReplicationConfig
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateReplicationInstance
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateReplicationSubnetGroup
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateReplicationTask
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteCertificate
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteConnection
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteDataMigration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteDataProvider
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteEndpoint
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteEventSubscription
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteFleetAdvisorCollector
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteFleetAdvisorDatabases
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteInstanceProfile
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteMigrationProject
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteReplicationConfig
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteReplicationInstance
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteReplicationSubnetGroup
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteReplicationTask
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteReplicationTaskAssessmentRun
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeAccountAttributes
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeApplicableIndividualAssessments
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeCertificates
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeConnections
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeConversionConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeDataMigrations
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeDataProviders
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeEndpointSettings
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeEndpointTypes
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeEndpoints
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeEngineVersions
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeEventCategories
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeEventSubscriptions
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeEvents
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeExtensionPackAssociations
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeFleetAdvisorCollectors
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeFleetAdvisorDatabases
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeFleetAdvisorLsaAnalysis
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeFleetAdvisorSchemaObjectSummary
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeFleetAdvisorSchemas
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeInstanceProfiles
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeMetadataModel
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeMetadataModelAssessments
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeMetadataModelChildren
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeMetadataModelConversions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeMetadataModelCreations
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeMetadataModelExportsAsScript
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeMetadataModelExportsToTarget
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeMetadataModelImports
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeMigrationProjects
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeOrderableReplicationInstances
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribePendingMaintenanceActions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeRecommendationLimitations
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeRecommendations
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeRefreshSchemasStatus
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeReplicationConfigs
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeReplicationInstanceTaskLogs
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeReplicationInstances
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeReplicationSubnetGroups
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeReplicationTableStatistics
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeReplicationTaskAssessmentResults
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeReplicationTaskAssessmentRuns
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeReplicationTaskIndividualAssessments
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeReplicationTasks
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeReplications
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeSchemas
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeTableStatistics
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ExportMetadataModelAssessment
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetTargetSelectionRules
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ImportCertificate
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListTagsForResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ModifyConversionConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ModifyDataMigration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ModifyDataProvider
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ModifyEndpoint
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ModifyEventSubscription
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ModifyInstanceProfile
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ModifyMigrationProject
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ModifyReplicationConfig
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ModifyReplicationInstance
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ModifyReplicationSubnetGroup
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ModifyReplicationTask
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] MoveReplicationTask
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] RebootReplicationInstance
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] RefreshSchemas
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ReloadReplicationTables
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ReloadTables
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] RemoveTagsFromResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] RunFleetAdvisorLsaAnalysis
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartDataMigration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartExtensionPackAssociation
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartMetadataModelAssessment
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartMetadataModelConversion
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartMetadataModelCreation
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartMetadataModelExportAsScript
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartMetadataModelExportToTarget
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartMetadataModelImport
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartRecommendations
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartReplication
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] StartReplicationTask
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartReplicationTaskAssessment
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartReplicationTaskAssessmentRun
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StopDataMigration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StopReplication
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] StopReplicationTask
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] TestConnection
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateSubscriptionsToEventBridge

Integration tests: 41/42 implemented operations tested (97.6%)
Untested implemented operations: 1

### winterbaume-databrew (databrew) - W: 32/44, S: 1/44, M: 24/44, F: 0/44, K: 0/44, C: 0/44

- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] BatchDeleteRecipeVersion
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateDataset
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateProfileJob
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateProject
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateRecipe
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateRecipeJob
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateRuleset
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateSchedule
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteDataset
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteJob
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteProject
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteRecipeVersion
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteRuleset
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteSchedule
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeDataset
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeJob
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeJobRun
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeProject
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeRecipe
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeRuleset
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeSchedule
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListDatasets
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListJobRuns
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListJobs
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListProjects
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListRecipeVersions
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListRecipes
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListRulesets
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListSchedules
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListTagsForResource
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] PublishRecipe
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] SendProjectSessionAction
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartJobRun
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartProjectSession
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StopJobRun
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UntagResource
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UpdateDataset
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UpdateProfileJob
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateProject
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UpdateRecipe
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UpdateRecipeJob
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UpdateRuleset
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateSchedule

Integration tests: 32/32 implemented operations tested (100.0%)

### winterbaume-datapipeline (data-pipeline) - W: 19/19, S: 0/19, M: 0/19, F: 0/19, K: 0/19, C: 0/19

- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ActivatePipeline
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] AddTags
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreatePipeline
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeactivatePipeline
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeletePipeline
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeObjects
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribePipelines
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] EvaluateExpression
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetPipelineDefinition
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListPipelines
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PollForTask
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutPipelineDefinition
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] QueryObjects
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] RemoveTags
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ReportTaskProgress
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ReportTaskRunnerHeartbeat
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] SetStatus
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] SetTaskStatus
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ValidatePipelineDefinition

Integration tests: 17/19 implemented operations tested (89.5%)
Untested implemented operations: 2

### winterbaume-datasync (datasync) - W: 8/53, S: 0/53, M: 6/53, F: 0/53, K: 0/53, C: 0/53

- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CancelTaskExecution
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateAgent
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateLocationAzureBlob
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateLocationEfs
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateLocationFsxLustre
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateLocationFsxOntap
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateLocationFsxOpenZfs
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateLocationFsxWindows
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateLocationHdfs
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateLocationNfs
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateLocationObjectStorage
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateLocationS3
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateLocationSmb
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateTask
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteAgent
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteLocation
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteTask
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeAgent
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeLocationAzureBlob
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeLocationEfs
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeLocationFsxLustre
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeLocationFsxOntap
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeLocationFsxOpenZfs
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeLocationFsxWindows
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeLocationHdfs
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeLocationNfs
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeLocationObjectStorage
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeLocationS3
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeLocationSmb
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeTask
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeTaskExecution
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListAgents
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListLocations
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListTagsForResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListTaskExecutions
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListTasks
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] StartTaskExecution
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] TagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UntagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateAgent
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateLocationAzureBlob
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateLocationEfs
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateLocationFsxLustre
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateLocationFsxOntap
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateLocationFsxOpenZfs
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateLocationFsxWindows
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateLocationHdfs
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateLocationNfs
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateLocationObjectStorage
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateLocationS3
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateLocationSmb
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UpdateTask
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateTaskExecution

Integration tests: 8/8 implemented operations tested (100.0%)

### winterbaume-dax (dax) - W: 6/21, S: 0/21, M: 8/21, F: 0/21, K: 0/21, C: 0/21

- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateCluster
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateParameterGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateSubnetGroup
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DecreaseReplicationFactor
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteCluster
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteParameterGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteSubnetGroup
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeClusters
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeDefaultParameters
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeEvents
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeParameterGroups
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeParameters
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeSubnetGroups
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] IncreaseReplicationFactor
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListTags
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] RebootNode
- W[ ] S[ ] M[x] F[ ] K[ ] C[ ] TagResource
- W[ ] S[ ] M[x] F[ ] K[ ] C[ ] UntagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateCluster
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateParameterGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateSubnetGroup

Integration tests: 6/6 implemented operations tested (100.0%)

### winterbaume-directconnect (direct-connect) - W: 7/63, S: 0/63, M: 0/63, F: 0/63, K: 0/63, C: 0/63

- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AcceptDirectConnectGatewayAssociationProposal
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AllocateConnectionOnInterconnect
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AllocateHostedConnection
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AllocatePrivateVirtualInterface
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AllocatePublicVirtualInterface
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AllocateTransitVirtualInterface
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AssociateConnectionWithLag
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AssociateHostedConnection
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AssociateMacSecKey
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AssociateVirtualInterface
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ConfirmConnection
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ConfirmCustomerAgreement
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ConfirmPrivateVirtualInterface
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ConfirmPublicVirtualInterface
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ConfirmTransitVirtualInterface
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateBGPPeer
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateConnection
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateDirectConnectGateway
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateDirectConnectGatewayAssociation
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateDirectConnectGatewayAssociationProposal
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateInterconnect
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateLag
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreatePrivateVirtualInterface
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreatePublicVirtualInterface
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateTransitVirtualInterface
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteBGPPeer
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteConnection
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteDirectConnectGateway
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteDirectConnectGatewayAssociation
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteDirectConnectGatewayAssociationProposal
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteInterconnect
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteLag
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteVirtualInterface
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeConnectionLoa
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeConnections
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeConnectionsOnInterconnect
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeCustomerMetadata
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeDirectConnectGatewayAssociationProposals
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeDirectConnectGatewayAssociations
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeDirectConnectGatewayAttachments
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeDirectConnectGateways
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeHostedConnections
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeInterconnectLoa
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeInterconnects
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeLags
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeLoa
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeLocations
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeRouterConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeTags
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeVirtualGateways
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeVirtualInterfaces
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DisassociateConnectionFromLag
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DisassociateMacSecKey
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListVirtualInterfaceTestHistory
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartBgpFailoverTest
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StopBgpFailoverTest
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UntagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateConnection
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateDirectConnectGateway
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateDirectConnectGatewayAssociation
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateLag
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateVirtualInterfaceAttributes

Integration tests: 7/7 implemented operations tested (100.0%)

### winterbaume-directory (directory-service) - W: 4/80, S: 0/80, M: 0/80, F: 0/80, K: 6/80, C: 0/80

- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AcceptSharedDirectory
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AddIpRoutes
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AddRegion
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AddTagsToResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CancelSchemaExtension
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ConnectDirectory
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateAlias
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateComputer
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateConditionalForwarder
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] CreateDirectory
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateHybridAD
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateLogSubscription
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateMicrosoftAD
- W[ ] S[ ] M[ ] F[ ] K[x] C[ ] CreateSnapshot
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateTrust
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteADAssessment
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteConditionalForwarder
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] DeleteDirectory
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteLogSubscription
- W[ ] S[ ] M[ ] F[ ] K[x] C[ ] DeleteSnapshot
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteTrust
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeregisterCertificate
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeregisterEventTopic
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeADAssessment
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeCAEnrollmentPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeCertificate
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeClientAuthenticationSettings
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeConditionalForwarders
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] DescribeDirectories
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeDirectoryDataAccess
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeDomainControllers
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeEventTopics
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeHybridADUpdate
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeLDAPSSettings
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeRegions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeSettings
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeSharedDirectories
- W[ ] S[ ] M[ ] F[ ] K[x] C[ ] DescribeSnapshots
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeTrusts
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeUpdateDirectory
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DisableCAEnrollmentPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DisableClientAuthentication
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DisableDirectoryDataAccess
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DisableLDAPS
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DisableRadius
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DisableSso
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] EnableCAEnrollmentPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] EnableClientAuthentication
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] EnableDirectoryDataAccess
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] EnableLDAPS
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] EnableRadius
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] EnableSso
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetDirectoryLimits
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetSnapshotLimits
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListADAssessments
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListCertificates
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListIpRoutes
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListLogSubscriptions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListSchemaExtensions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListTagsForResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] RegisterCertificate
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] RegisterEventTopic
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] RejectSharedDirectory
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] RemoveIpRoutes
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] RemoveRegion
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] RemoveTagsFromResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ResetUserPassword
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] RestoreFromSnapshot
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ShareDirectory
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartADAssessment
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartSchemaExtension
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UnshareDirectory
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateConditionalForwarder
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateDirectorySetup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateHybridAD
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateNumberOfDomainControllers
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateRadius
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateSettings
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateTrust
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] VerifyTrust

Integration tests: 4/4 implemented operations tested (100.0%)

### winterbaume-dlm (dlm) - W: 2/8, S: 0/8, M: 0/8, F: 0/8, K: 5/8, C: 0/8

- W[ ] S[ ] M[ ] F[ ] K[x] C[ ] CreateLifecyclePolicy
- W[ ] S[ ] M[ ] F[ ] K[x] C[ ] DeleteLifecyclePolicy
- W[ ] S[ ] M[ ] F[ ] K[x] C[ ] GetLifecyclePolicies
- W[ ] S[ ] M[ ] F[ ] K[x] C[ ] GetLifecyclePolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UntagResource
- W[ ] S[ ] M[ ] F[ ] K[x] C[ ] UpdateLifecyclePolicy

Integration tests: 2/2 implemented operations tested (100.0%)

### winterbaume-dsql (dsql) - W: 12/12, S: 0/12, M: 5/12, F: 0/12, K: 0/12, C: 12/12

- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateCluster
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteCluster
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteClusterPolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetCluster
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetClusterPolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetVpcEndpointServiceName
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListClusters
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] PutClusterPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateCluster

Integration tests: 12/12 implemented operations tested (100.0%)

### winterbaume-dynamodb (dynamodb) - W: 57/57, S: 0/57, M: 39/57, F: 0/57, K: 21/57, C: 57/57

Terraform E2E: 8 tests across 2 terraform resource types

Resource types: aws_dynamodb_table, aws_dynamodb_table_item

- W[x] S[ ] M[x] F[ ] K[ ] C[x] BatchExecuteStatement
- W[x] S[ ] M[x] F[ ] K[x] C[x] BatchGetItem
- W[x] S[ ] M[x] F[ ] K[x] C[x] BatchWriteItem
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateBackup
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateGlobalTable
- W[x] S[ ] M[x] F[ ] K[x] C[x] CreateTable
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteBackup
- W[x] S[ ] M[x] F[ ] K[x] C[x] DeleteItem
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteResourcePolicy
- W[x] S[ ] M[x] F[ ] K[x] C[x] DeleteTable
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeBackup
- W[x] S[ ] M[x] F[ ] K[x] C[x] DescribeContinuousBackups
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeContributorInsights
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeEndpoints
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeExport
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeGlobalTable
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeGlobalTableSettings
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeImport
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeKinesisStreamingDestination
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeLimits
- W[x] S[ ] M[x] F[ ] K[x] C[x] DescribeTable
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeTableReplicaAutoScaling
- W[x] S[ ] M[x] F[ ] K[x] C[x] DescribeTimeToLive
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DisableKinesisStreamingDestination
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] EnableKinesisStreamingDestination
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ExecuteStatement
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ExecuteTransaction
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ExportTableToPointInTime
- W[x] S[ ] M[x] F[ ] K[x] C[x] GetItem
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetResourcePolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ImportTable
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListBackups
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListContributorInsights
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListExports
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListGlobalTables
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListImports
- W[x] S[ ] M[x] F[ ] K[x] C[x] ListTables
- W[x] S[ ] M[x] F[ ] K[x] C[x] ListTagsOfResource
- W[x] S[ ] M[x] F[ ] K[x] C[x] PutItem
- W[x] S[ ] M[x] F[ ] K[ ] C[x] PutResourcePolicy
- W[x] S[ ] M[x] F[ ] K[x] C[x] Query
- W[x] S[ ] M[x] F[ ] K[ ] C[x] RestoreTableFromBackup
- W[x] S[ ] M[x] F[ ] K[ ] C[x] RestoreTableToPointInTime
- W[x] S[ ] M[x] F[ ] K[x] C[x] Scan
- W[x] S[ ] M[x] F[ ] K[x] C[x] TagResource
- W[x] S[ ] M[x] F[ ] K[x] C[x] TransactGetItems
- W[x] S[ ] M[x] F[ ] K[x] C[x] TransactWriteItems
- W[x] S[ ] M[x] F[ ] K[x] C[x] UntagResource
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UpdateContinuousBackups
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateContributorInsights
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateGlobalTable
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateGlobalTableSettings
- W[x] S[ ] M[x] F[ ] K[x] C[x] UpdateItem
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateKinesisStreamingDestination
- W[x] S[ ] M[x] F[ ] K[x] C[x] UpdateTable
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateTableReplicaAutoScaling
- W[x] S[ ] M[x] F[ ] K[x] C[x] UpdateTimeToLive

Integration tests: 57/57 implemented operations tested (100.0%)

### winterbaume-dynamodbstreams (dynamodb-streams) - W: 4/4, S: 0/4, M: 0/4, F: 0/4, K: 0/4, C: 4/4

- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeStream
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetRecords
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetShardIterator
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListStreams

Integration tests: 4/4 implemented operations tested (100.0%)

### winterbaume-ebs (ebs) - W: 6/6, S: 0/6, M: 6/6, F: 0/6, K: 0/6, C: 0/6

- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CompleteSnapshot
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetSnapshotBlock
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListChangedBlocks
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListSnapshotBlocks
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] PutSnapshotBlock
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] StartSnapshot

Integration tests: 6/6 implemented operations tested (100.0%)

### winterbaume-ec2 (ec2) - W: 713/763, S: 50/763, M: 223/763, F: 0/763, K: 39/763, C: 763/763

Terraform E2E: 20 tests across 7 terraform resource types

Resource types: aws_internet_gateway, aws_key_pair, aws_route_table, aws_route_table_association, aws_security_group, aws_subnet, aws_vpc

- W[x] S[ ] M[ ] F[ ] K[ ] C[x] AcceptAddressTransfer
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] AcceptCapacityReservationBillingOwnership
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] AcceptReservedInstancesExchangeQuote
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] AcceptTransitGatewayClientVpnAttachment
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] AcceptTransitGatewayMulticastDomainAssociations
- W[x] S[ ] M[x] F[ ] K[ ] C[x] AcceptTransitGatewayPeeringAttachment
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] AcceptTransitGatewayVpcAttachment
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] AcceptVpcEndpointConnections
- W[x] S[ ] M[x] F[ ] K[ ] C[x] AcceptVpcPeeringConnection
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] AdvertiseByoipCidr
- W[x] S[ ] M[x] F[ ] K[ ] C[x] AllocateAddress
- W[x] S[ ] M[x] F[ ] K[ ] C[x] AllocateHosts
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] AllocateIpamPoolCidr
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ApplySecurityGroupsToClientVpnTargetNetwork
- W[x] S[ ] M[x] F[ ] K[ ] C[x] AssignIpv6Addresses
- W[x] S[ ] M[x] F[ ] K[ ] C[x] AssignPrivateIpAddresses
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] AssignPrivateNatGatewayAddress
- W[x] S[ ] M[x] F[ ] K[ ] C[x] AssociateAddress
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] AssociateCapacityReservationBillingOwner
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] AssociateClientVpnTargetNetwork
- W[x] S[ ] M[x] F[ ] K[ ] C[x] AssociateDhcpOptions
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] AssociateEnclaveCertificateIamRole
- W[x] S[ ] M[x] F[ ] K[ ] C[x] AssociateIamInstanceProfile
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] AssociateInstanceEventWindow
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] AssociateIpamByoasn
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] AssociateIpamResourceDiscovery
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] AssociateNatGatewayAddress
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] AssociateRouteServer
- W[x] S[ ] M[x] F[ ] K[x] C[x] AssociateRouteTable
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] AssociateSecurityGroupVpc
- W[x] S[ ] M[x] F[ ] K[ ] C[x] AssociateSubnetCidrBlock
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] AssociateTransitGatewayMulticastDomain
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] AssociateTransitGatewayPolicyTable
- W[x] S[ ] M[x] F[ ] K[ ] C[x] AssociateTransitGatewayRouteTable
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] AssociateTrunkInterface
- W[x] S[ ] M[x] F[ ] K[ ] C[x] AssociateVpcCidrBlock
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] AttachClassicLinkVpc
- W[x] S[ ] M[x] F[ ] K[x] C[x] AttachInternetGateway
- W[x] S[ ] M[x] F[ ] K[ ] C[x] AttachNetworkInterface
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] AttachVerifiedAccessTrustProvider
- W[x] S[ ] M[x] F[ ] K[ ] C[x] AttachVolume
- W[x] S[ ] M[x] F[ ] K[ ] C[x] AttachVpnGateway
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] AuthorizeClientVpnIngress
- W[x] S[ ] M[x] F[ ] K[x] C[x] AuthorizeSecurityGroupEgress
- W[x] S[ ] M[x] F[ ] K[x] C[x] AuthorizeSecurityGroupIngress
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] BundleInstance
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CancelBundleTask
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CancelCapacityReservation
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CancelCapacityReservationFleets
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CancelConversionTask
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CancelDeclarativePoliciesReport
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CancelExportTask
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CancelImageLaunchPermission
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CancelImportTask
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CancelReservedInstancesListing
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CancelSpotFleetRequests
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CancelSpotInstanceRequests
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ConfirmProductInstance
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CopyFpgaImage
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CopyImage
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CopySnapshot
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CopyVolumes
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateCapacityManagerDataExport
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateCapacityReservation
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateCapacityReservationBySplitting
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateCapacityReservationFleet
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateCarrierGateway
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateClientVpnEndpoint
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateClientVpnRoute
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateCoipCidr
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateCoipPool
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateCustomerGateway
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateDefaultSubnet
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateDefaultVpc
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateDelegateMacVolumeOwnershipTask
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateDhcpOptions
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateEgressOnlyInternetGateway
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateFleet
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateFlowLogs
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateFpgaImage
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateImage
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateImageUsageReport
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateInstanceConnectEndpoint
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateInstanceEventWindow
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateInstanceExportTask
- W[x] S[ ] M[x] F[ ] K[x] C[x] CreateInternetGateway
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateInterruptibleCapacityReservationAllocation
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateIpam
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateIpamExternalResourceVerificationToken
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateIpamPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateIpamPool
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateIpamPrefixListResolver
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateIpamPrefixListResolverTarget
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateIpamResourceDiscovery
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateIpamScope
- W[x] S[ ] M[x] F[ ] K[x] C[x] CreateKeyPair
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateLaunchTemplate
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateLaunchTemplateVersion
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateLocalGatewayRoute
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateLocalGatewayRouteTable
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateLocalGatewayRouteTableVirtualInterfaceGroupAssociation
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateLocalGatewayRouteTableVpcAssociation
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateLocalGatewayVirtualInterface
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateLocalGatewayVirtualInterfaceGroup
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateMacSystemIntegrityProtectionModificationTask
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateManagedPrefixList
- W[x] S[ ] M[x] F[ ] K[x] C[x] CreateNatGateway
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateNetworkAcl
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateNetworkAclEntry
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateNetworkInsightsAccessScope
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateNetworkInsightsPath
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateNetworkInterface
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateNetworkInterfacePermission
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreatePlacementGroup
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreatePublicIpv4Pool
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateReplaceRootVolumeTask
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateReservedInstancesListing
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateRestoreImageTask
- W[x] S[ ] M[x] F[ ] K[x] C[x] CreateRoute
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateRouteServer
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateRouteServerEndpoint
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateRouteServerPeer
- W[x] S[ ] M[x] F[ ] K[x] C[x] CreateRouteTable
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateSecondaryNetwork
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateSecondarySubnet
- W[x] S[ ] M[x] F[ ] K[x] C[x] CreateSecurityGroup
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateSnapshot
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateSnapshots
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateSpotDatafeedSubscription
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateStoreImageTask
- W[x] S[ ] M[x] F[ ] K[x] C[x] CreateSubnet
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateSubnetCidrReservation
- W[x] S[ ] M[x] F[ ] K[x] C[x] CreateTags
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateTrafficMirrorFilter
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateTrafficMirrorFilterRule
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateTrafficMirrorSession
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateTrafficMirrorTarget
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateTransitGateway
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateTransitGatewayConnect
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateTransitGatewayConnectPeer
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateTransitGatewayMeteringPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateTransitGatewayMeteringPolicyEntry
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateTransitGatewayMulticastDomain
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateTransitGatewayPeeringAttachment
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateTransitGatewayPolicyTable
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateTransitGatewayPrefixListReference
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateTransitGatewayRoute
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateTransitGatewayRouteTable
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateTransitGatewayRouteTableAnnouncement
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateTransitGatewayVpcAttachment
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateVerifiedAccessEndpoint
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateVerifiedAccessGroup
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateVerifiedAccessInstance
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateVerifiedAccessTrustProvider
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateVolume
- W[x] S[ ] M[x] F[ ] K[x] C[x] CreateVpc
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateVpcBlockPublicAccessExclusion
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateVpcEncryptionControl
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateVpcEndpoint
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateVpcEndpointConnectionNotification
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateVpcEndpointServiceConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateVpcPeeringConnection
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateVpnConcentrator
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateVpnConnection
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateVpnConnectionRoute
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateVpnGateway
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteCapacityManagerDataExport
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteCarrierGateway
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteClientVpnEndpoint
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteClientVpnRoute
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteCoipCidr
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteCoipPool
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteCustomerGateway
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteDhcpOptions
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteEgressOnlyInternetGateway
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteFleets
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteFlowLogs
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteFpgaImage
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteImageUsageReport
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteInstanceConnectEndpoint
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteInstanceEventWindow
- W[x] S[ ] M[x] F[ ] K[x] C[x] DeleteInternetGateway
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteIpam
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteIpamExternalResourceVerificationToken
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteIpamPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteIpamPool
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteIpamPrefixListResolver
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteIpamPrefixListResolverTarget
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteIpamResourceDiscovery
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteIpamScope
- W[x] S[ ] M[x] F[ ] K[x] C[x] DeleteKeyPair
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteLaunchTemplate
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteLaunchTemplateVersions
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteLocalGatewayRoute
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteLocalGatewayRouteTable
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteLocalGatewayRouteTableVirtualInterfaceGroupAssociation
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteLocalGatewayRouteTableVpcAssociation
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteLocalGatewayVirtualInterface
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteLocalGatewayVirtualInterfaceGroup
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteManagedPrefixList
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteNatGateway
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteNetworkAcl
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteNetworkAclEntry
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteNetworkInsightsAccessScope
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteNetworkInsightsAccessScopeAnalysis
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteNetworkInsightsAnalysis
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteNetworkInsightsPath
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteNetworkInterface
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteNetworkInterfacePermission
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeletePlacementGroup
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeletePublicIpv4Pool
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteQueuedReservedInstances
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteRoute
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteRouteServer
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteRouteServerEndpoint
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteRouteServerPeer
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteRouteTable
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteSecondaryNetwork
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteSecondarySubnet
- W[x] S[ ] M[x] F[ ] K[x] C[x] DeleteSecurityGroup
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteSnapshot
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteSpotDatafeedSubscription
- W[x] S[ ] M[x] F[ ] K[x] C[x] DeleteSubnet
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteSubnetCidrReservation
- W[x] S[ ] M[x] F[ ] K[x] C[x] DeleteTags
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteTrafficMirrorFilter
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteTrafficMirrorFilterRule
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteTrafficMirrorSession
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteTrafficMirrorTarget
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteTransitGateway
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] DeleteTransitGatewayClientVpnAttachment
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteTransitGatewayConnect
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteTransitGatewayConnectPeer
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteTransitGatewayMeteringPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteTransitGatewayMeteringPolicyEntry
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteTransitGatewayMulticastDomain
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteTransitGatewayPeeringAttachment
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteTransitGatewayPolicyTable
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteTransitGatewayPrefixListReference
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteTransitGatewayRoute
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteTransitGatewayRouteTable
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteTransitGatewayRouteTableAnnouncement
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteTransitGatewayVpcAttachment
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteVerifiedAccessEndpoint
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteVerifiedAccessGroup
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteVerifiedAccessInstance
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteVerifiedAccessTrustProvider
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteVolume
- W[x] S[ ] M[x] F[ ] K[x] C[x] DeleteVpc
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteVpcBlockPublicAccessExclusion
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteVpcEncryptionControl
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteVpcEndpointConnectionNotifications
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteVpcEndpointServiceConfigurations
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteVpcEndpoints
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteVpcPeeringConnection
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteVpnConcentrator
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteVpnConnection
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteVpnConnectionRoute
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteVpnGateway
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeprovisionByoipCidr
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeprovisionIpamByoasn
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeprovisionIpamPoolCidr
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeprovisionPublicIpv4PoolCidr
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeregisterImage
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeregisterInstanceEventNotificationAttributes
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeregisterTransitGatewayMulticastGroupMembers
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeregisterTransitGatewayMulticastGroupSources
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeAccountAttributes
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeAddressTransfers
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeAddresses
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeAddressesAttribute
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] DescribeAggregateIdFormat
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeAvailabilityZones
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeAwsNetworkPerformanceMetricSubscriptions
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeBundleTasks
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeByoipCidrs
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeCapacityBlockExtensionHistory
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeCapacityBlockExtensionOfferings
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeCapacityBlockOfferings
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeCapacityBlockStatus
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeCapacityBlocks
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeCapacityManagerDataExports
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeCapacityReservationBillingRequests
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeCapacityReservationFleets
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] DescribeCapacityReservationTopology
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeCapacityReservations
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeCarrierGateways
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeClassicLinkInstances
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeClientVpnAuthorizationRules
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeClientVpnConnections
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeClientVpnEndpoints
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeClientVpnRoutes
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeClientVpnTargetNetworks
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeCoipPools
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeConversionTasks
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeCustomerGateways
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeDeclarativePoliciesReports
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeDhcpOptions
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeEgressOnlyInternetGateways
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] DescribeElasticGpus
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeExportImageTasks
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] DescribeExportTasks
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] DescribeFastLaunchImages
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] DescribeFastSnapshotRestores
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] DescribeFleetHistory
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeFleetInstances
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeFleets
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeFlowLogs
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeFpgaImageAttribute
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeFpgaImages
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeHostReservationOfferings
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeHostReservations
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeHosts
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeIamInstanceProfileAssociations
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeIdFormat
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeIdentityIdFormat
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeImageAttribute
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeImageReferences
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeImageUsageReportEntries
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeImageUsageReports
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeImages
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeImportImageTasks
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeImportSnapshotTasks
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeInstanceAttribute
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeInstanceConnectEndpoints
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeInstanceCreditSpecifications
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] DescribeInstanceEventNotificationAttributes
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeInstanceEventWindows
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] DescribeInstanceImageMetadata
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] DescribeInstanceSqlHaHistoryStates
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] DescribeInstanceSqlHaStates
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeInstanceStatus
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] DescribeInstanceTopology
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeInstanceTypeOfferings
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeInstanceTypes
- W[x] S[ ] M[x] F[ ] K[x] C[x] DescribeInstances
- W[x] S[ ] M[x] F[ ] K[x] C[x] DescribeInternetGateways
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeIpamByoasn
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeIpamExternalResourceVerificationTokens
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeIpamPolicies
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeIpamPools
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeIpamPrefixListResolverTargets
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeIpamPrefixListResolvers
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeIpamResourceDiscoveries
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeIpamResourceDiscoveryAssociations
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeIpamScopes
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeIpams
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] DescribeIpv6Pools
- W[x] S[ ] M[x] F[ ] K[x] C[x] DescribeKeyPairs
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeLaunchTemplateVersions
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeLaunchTemplates
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeLocalGatewayRouteTableVirtualInterfaceGroupAssociations
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeLocalGatewayRouteTableVpcAssociations
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeLocalGatewayRouteTables
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeLocalGatewayVirtualInterfaceGroups
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeLocalGatewayVirtualInterfaces
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeLocalGateways
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] DescribeLockedSnapshots
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeMacHosts
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeMacModificationTasks
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeManagedPrefixLists
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] DescribeMovingAddresses
- W[x] S[ ] M[x] F[ ] K[x] C[x] DescribeNatGateways
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeNetworkAcls
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeNetworkInsightsAccessScopeAnalyses
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeNetworkInsightsAccessScopes
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeNetworkInsightsAnalyses
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeNetworkInsightsPaths
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeNetworkInterfaceAttribute
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeNetworkInterfacePermissions
- W[x] S[ ] M[x] F[ ] K[x] C[x] DescribeNetworkInterfaces
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeOutpostLags
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribePlacementGroups
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribePrefixLists
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribePrincipalIdFormat
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribePublicIpv4Pools
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeRegions
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeReplaceRootVolumeTasks
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeReservedInstances
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeReservedInstancesListings
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeReservedInstancesModifications
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeReservedInstancesOfferings
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeRouteServerEndpoints
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeRouteServerPeers
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeRouteServers
- W[x] S[ ] M[x] F[ ] K[x] C[x] DescribeRouteTables
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeScheduledInstanceAvailability
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeScheduledInstances
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeSecondaryInterfaces
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeSecondaryNetworks
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeSecondarySubnets
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeSecurityGroupReferences
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeSecurityGroupRules
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeSecurityGroupVpcAssociations
- W[x] S[ ] M[x] F[ ] K[x] C[x] DescribeSecurityGroups
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] DescribeServiceLinkVirtualInterfaces
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeSnapshotAttribute
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeSnapshotTierStatus
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeSnapshots
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeSpotDatafeedSubscription
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeSpotFleetInstances
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] DescribeSpotFleetRequestHistory
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeSpotFleetRequests
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeSpotInstanceRequests
- W[ ] S[x] M[x] F[ ] K[ ] C[x] DescribeSpotPriceHistory
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] DescribeStaleSecurityGroups
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeStoreImageTasks
- W[x] S[ ] M[x] F[ ] K[x] C[x] DescribeSubnets
- W[x] S[ ] M[x] F[ ] K[x] C[x] DescribeTags
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeTrafficMirrorFilterRules
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeTrafficMirrorFilters
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeTrafficMirrorSessions
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeTrafficMirrorTargets
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeTransitGatewayAttachments
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeTransitGatewayConnectPeers
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeTransitGatewayConnects
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeTransitGatewayMeteringPolicies
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeTransitGatewayMulticastDomains
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeTransitGatewayPeeringAttachments
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeTransitGatewayPolicyTables
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeTransitGatewayRouteTableAnnouncements
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeTransitGatewayRouteTables
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeTransitGatewayVpcAttachments
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeTransitGateways
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeTrunkInterfaceAssociations
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeVerifiedAccessEndpoints
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeVerifiedAccessGroups
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeVerifiedAccessInstanceLoggingConfigurations
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeVerifiedAccessInstances
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeVerifiedAccessTrustProviders
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeVolumeAttribute
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeVolumeStatus
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeVolumes
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeVolumesModifications
- W[x] S[ ] M[x] F[ ] K[x] C[x] DescribeVpcAttribute
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeVpcBlockPublicAccessExclusions
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeVpcBlockPublicAccessOptions
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeVpcClassicLink
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeVpcClassicLinkDnsSupport
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeVpcEncryptionControls
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeVpcEndpointAssociations
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeVpcEndpointConnectionNotifications
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeVpcEndpointConnections
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeVpcEndpointServiceConfigurations
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeVpcEndpointServicePermissions
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeVpcEndpointServices
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeVpcEndpoints
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeVpcPeeringConnections
- W[x] S[ ] M[x] F[ ] K[x] C[x] DescribeVpcs
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeVpnConcentrators
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeVpnConnections
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeVpnGateways
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DetachClassicLinkVpc
- W[x] S[ ] M[x] F[ ] K[x] C[x] DetachInternetGateway
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DetachNetworkInterface
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DetachVerifiedAccessTrustProvider
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DetachVolume
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DetachVpnGateway
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DisableAddressTransfer
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DisableAllowedImagesSettings
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DisableAwsNetworkPerformanceMetricSubscription
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DisableCapacityManager
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DisableEbsEncryptionByDefault
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DisableFastLaunch
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DisableFastSnapshotRestores
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DisableImage
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DisableImageBlockPublicAccess
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DisableImageDeprecation
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DisableImageDeregistrationProtection
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] DisableInstanceSqlHaStandbyDetections
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] DisableIpamOrganizationAdminAccount
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DisableIpamPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DisableRouteServerPropagation
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DisableSerialConsoleAccess
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] DisableSnapshotBlockPublicAccess
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DisableTransitGatewayRouteTablePropagation
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DisableVgwRoutePropagation
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DisableVpcClassicLink
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DisableVpcClassicLinkDnsSupport
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DisassociateAddress
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DisassociateCapacityReservationBillingOwner
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DisassociateClientVpnTargetNetwork
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DisassociateEnclaveCertificateIamRole
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DisassociateIamInstanceProfile
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DisassociateInstanceEventWindow
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DisassociateIpamByoasn
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DisassociateIpamResourceDiscovery
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DisassociateNatGatewayAddress
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DisassociateRouteServer
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DisassociateRouteTable
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DisassociateSecurityGroupVpc
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DisassociateSubnetCidrBlock
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DisassociateTransitGatewayMulticastDomain
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DisassociateTransitGatewayPolicyTable
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DisassociateTransitGatewayRouteTable
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DisassociateTrunkInterface
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DisassociateVpcCidrBlock
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] EnableAddressTransfer
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] EnableAllowedImagesSettings
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] EnableAwsNetworkPerformanceMetricSubscription
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] EnableCapacityManager
- W[x] S[ ] M[x] F[ ] K[ ] C[x] EnableEbsEncryptionByDefault
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] EnableFastLaunch
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] EnableFastSnapshotRestores
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] EnableImage
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] EnableImageBlockPublicAccess
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] EnableImageDeprecation
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] EnableImageDeregistrationProtection
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] EnableInstanceSqlHaStandbyDetections
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] EnableIpamOrganizationAdminAccount
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] EnableIpamPolicy
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] EnableReachabilityAnalyzerOrganizationSharing
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] EnableRouteServerPropagation
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] EnableSerialConsoleAccess
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] EnableSnapshotBlockPublicAccess
- W[x] S[ ] M[x] F[ ] K[ ] C[x] EnableTransitGatewayRouteTablePropagation
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] EnableVgwRoutePropagation
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] EnableVolumeIO
- W[x] S[ ] M[x] F[ ] K[ ] C[x] EnableVpcClassicLink
- W[x] S[ ] M[x] F[ ] K[ ] C[x] EnableVpcClassicLinkDnsSupport
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ExportClientVpnClientCertificateRevocationList
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ExportClientVpnClientConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ExportImage
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ExportTransitGatewayRoutes
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ExportVerifiedAccessInstanceClientConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetActiveVpnTunnelStatus
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetAllowedImagesSettings
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetAssociatedEnclaveCertificateIamRoles
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetAssociatedIpv6PoolCidrs
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] GetAwsNetworkPerformanceData
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] GetCapacityManagerAttributes
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] GetCapacityManagerMetricData
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] GetCapacityManagerMetricDimensions
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] GetCapacityManagerMonitoredTagKeys
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetCapacityReservationUsage
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetCoipPoolUsage
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetConsoleOutput
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] GetConsoleScreenshot
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetDeclarativePoliciesReportSummary
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetDefaultCreditSpecification
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetEbsDefaultKmsKeyId
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetEbsEncryptionByDefault
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetEnabledIpamPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetFlowLogsIntegrationTemplate
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetGroupsForCapacityReservation
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetHostReservationPurchasePreview
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] GetImageAncestry
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] GetImageBlockPublicAccessState
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetInstanceMetadataDefaults
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] GetInstanceTpmEkPub
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetInstanceTypesFromInstanceRequirements
- W[ ] S[x] M[x] F[ ] K[ ] C[x] GetInstanceUefiData
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetIpamAddressHistory
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetIpamDiscoveredAccounts
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetIpamDiscoveredPublicAddresses
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetIpamDiscoveredResourceCidrs
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetIpamPolicyAllocationRules
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] GetIpamPolicyOrganizationTargets
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetIpamPoolAllocations
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetIpamPoolCidrs
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetIpamPrefixListResolverRules
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetIpamPrefixListResolverVersionEntries
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetIpamPrefixListResolverVersions
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetIpamResourceCidrs
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetLaunchTemplateData
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetManagedPrefixListAssociations
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetManagedPrefixListEntries
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] GetManagedResourceVisibility
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetNetworkInsightsAccessScopeAnalysisFindings
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetNetworkInsightsAccessScopeContent
- W[ ] S[x] M[x] F[ ] K[ ] C[x] GetPasswordData
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetReservedInstancesExchangeQuote
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetRouteServerAssociations
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetRouteServerPropagations
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] GetRouteServerRoutingDatabase
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetSecurityGroupsForVpc
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] GetSerialConsoleAccessStatus
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] GetSnapshotBlockPublicAccessState
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] GetSpotPlacementScores
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetSubnetCidrReservations
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetTransitGatewayAttachmentPropagations
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetTransitGatewayMeteringPolicyEntries
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetTransitGatewayMulticastDomainAssociations
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetTransitGatewayPolicyTableAssociations
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetTransitGatewayPolicyTableEntries
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetTransitGatewayPrefixListReferences
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetTransitGatewayRouteTableAssociations
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetTransitGatewayRouteTablePropagations
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetVerifiedAccessEndpointPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetVerifiedAccessEndpointTargets
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetVerifiedAccessGroupPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetVpcResourcesBlockingEncryptionEnforcement
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetVpnConnectionDeviceSampleConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetVpnConnectionDeviceTypes
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetVpnTunnelReplacementStatus
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ImportClientVpnClientCertificateRevocationList
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ImportImage
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ImportInstance
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ImportKeyPair
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ImportSnapshot
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ImportVolume
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListImagesInRecycleBin
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListSnapshotsInRecycleBin
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListVolumesInRecycleBin
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] LockSnapshot
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ModifyAddressAttribute
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ModifyAvailabilityZoneGroup
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ModifyCapacityReservation
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ModifyCapacityReservationFleet
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ModifyClientVpnEndpoint
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ModifyDefaultCreditSpecification
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ModifyEbsDefaultKmsKeyId
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ModifyFleet
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ModifyFpgaImageAttribute
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ModifyHosts
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ModifyIdFormat
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ModifyIdentityIdFormat
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ModifyImageAttribute
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ModifyInstanceAttribute
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ModifyInstanceCapacityReservationAttributes
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ModifyInstanceConnectEndpoint
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ModifyInstanceCpuOptions
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ModifyInstanceCreditSpecification
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ModifyInstanceEventStartTime
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ModifyInstanceEventWindow
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ModifyInstanceMaintenanceOptions
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ModifyInstanceMetadataDefaults
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ModifyInstanceMetadataOptions
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ModifyInstanceNetworkPerformanceOptions
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ModifyInstancePlacement
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ModifyIpam
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ModifyIpamPolicyAllocationRules
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ModifyIpamPool
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ModifyIpamPrefixListResolver
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ModifyIpamPrefixListResolverTarget
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ModifyIpamResourceCidr
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ModifyIpamResourceDiscovery
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ModifyIpamScope
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ModifyLaunchTemplate
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ModifyLocalGatewayRoute
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ModifyManagedPrefixList
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] ModifyManagedResourceVisibility
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ModifyNetworkInterfaceAttribute
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ModifyPrivateDnsNameOptions
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ModifyPublicIpDnsNameOptions
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ModifyReservedInstances
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ModifyRouteServer
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ModifySecurityGroupRules
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ModifySnapshotAttribute
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ModifySnapshotTier
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ModifySpotFleetRequest
- W[x] S[ ] M[x] F[ ] K[x] C[x] ModifySubnetAttribute
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ModifyTrafficMirrorFilterNetworkServices
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ModifyTrafficMirrorFilterRule
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ModifyTrafficMirrorSession
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ModifyTransitGateway
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ModifyTransitGatewayMeteringPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ModifyTransitGatewayPrefixListReference
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ModifyTransitGatewayVpcAttachment
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ModifyVerifiedAccessEndpoint
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ModifyVerifiedAccessEndpointPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ModifyVerifiedAccessGroup
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ModifyVerifiedAccessGroupPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ModifyVerifiedAccessInstance
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ModifyVerifiedAccessInstanceLoggingConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ModifyVerifiedAccessTrustProvider
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ModifyVolume
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ModifyVolumeAttribute
- W[x] S[ ] M[x] F[ ] K[x] C[x] ModifyVpcAttribute
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ModifyVpcBlockPublicAccessExclusion
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ModifyVpcBlockPublicAccessOptions
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ModifyVpcEncryptionControl
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ModifyVpcEndpoint
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ModifyVpcEndpointConnectionNotification
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ModifyVpcEndpointServiceConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ModifyVpcEndpointServicePayerResponsibility
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ModifyVpcEndpointServicePermissions
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ModifyVpcPeeringConnectionOptions
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ModifyVpcTenancy
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ModifyVpnConnection
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ModifyVpnConnectionOptions
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ModifyVpnTunnelCertificate
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ModifyVpnTunnelOptions
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] MonitorInstances
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] MoveAddressToVpc
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] MoveByoipCidrToIpam
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] MoveCapacityReservationInstances
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ProvisionByoipCidr
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ProvisionIpamByoasn
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ProvisionIpamPoolCidr
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ProvisionPublicIpv4PoolCidr
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] PurchaseCapacityBlock
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] PurchaseCapacityBlockExtension
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] PurchaseHostReservation
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] PurchaseReservedInstancesOffering
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] PurchaseScheduledInstances
- W[x] S[ ] M[x] F[ ] K[ ] C[x] RebootInstances
- W[x] S[ ] M[x] F[ ] K[ ] C[x] RegisterImage
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] RegisterInstanceEventNotificationAttributes
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] RegisterTransitGatewayMulticastGroupMembers
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] RegisterTransitGatewayMulticastGroupSources
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] RejectCapacityReservationBillingOwnership
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] RejectTransitGatewayClientVpnAttachment
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] RejectTransitGatewayMulticastDomainAssociations
- W[x] S[ ] M[x] F[ ] K[ ] C[x] RejectTransitGatewayPeeringAttachment
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] RejectTransitGatewayVpcAttachment
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] RejectVpcEndpointConnections
- W[x] S[ ] M[x] F[ ] K[ ] C[x] RejectVpcPeeringConnection
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ReleaseAddress
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ReleaseHosts
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ReleaseIpamPoolAllocation
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ReplaceIamInstanceProfileAssociation
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ReplaceImageCriteriaInAllowedImagesSettings
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ReplaceNetworkAclAssociation
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ReplaceNetworkAclEntry
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ReplaceRoute
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ReplaceRouteTableAssociation
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ReplaceTransitGatewayRoute
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ReplaceVpnTunnel
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ReportInstanceStatus
- W[x] S[ ] M[x] F[ ] K[ ] C[x] RequestSpotFleet
- W[x] S[ ] M[x] F[ ] K[ ] C[x] RequestSpotInstances
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ResetAddressAttribute
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ResetEbsDefaultKmsKeyId
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ResetFpgaImageAttribute
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ResetImageAttribute
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ResetInstanceAttribute
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ResetNetworkInterfaceAttribute
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ResetSnapshotAttribute
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] RestoreAddressToClassic
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] RestoreImageFromRecycleBin
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] RestoreManagedPrefixListVersion
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] RestoreSnapshotFromRecycleBin
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] RestoreSnapshotTier
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] RestoreVolumeFromRecycleBin
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] RevokeClientVpnIngress
- W[x] S[ ] M[x] F[ ] K[x] C[x] RevokeSecurityGroupEgress
- W[x] S[ ] M[x] F[ ] K[x] C[x] RevokeSecurityGroupIngress
- W[x] S[ ] M[x] F[ ] K[x] C[x] RunInstances
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] RunScheduledInstances
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] SearchLocalGatewayRoutes
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] SearchTransitGatewayMulticastGroups
- W[x] S[ ] M[x] F[ ] K[ ] C[x] SearchTransitGatewayRoutes
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] SendDiagnosticInterrupt
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] StartDeclarativePoliciesReport
- W[x] S[ ] M[x] F[ ] K[x] C[x] StartInstances
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] StartNetworkInsightsAccessScopeAnalysis
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] StartNetworkInsightsAnalysis
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] StartVpcEndpointServicePrivateDnsVerification
- W[x] S[ ] M[x] F[ ] K[x] C[x] StopInstances
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] TerminateClientVpnConnections
- W[x] S[ ] M[x] F[ ] K[x] C[x] TerminateInstances
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UnassignIpv6Addresses
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UnassignPrivateIpAddresses
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UnassignPrivateNatGatewayAddress
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UnlockSnapshot
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] UnmonitorInstances
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] UpdateCapacityManagerMonitoredTagKeys
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateCapacityManagerOrganizationsAccess
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateInterruptibleCapacityReservationAllocation
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UpdateSecurityGroupRuleDescriptionsEgress
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UpdateSecurityGroupRuleDescriptionsIngress
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] WithdrawByoipCidr

Integration tests: 672/713 implemented operations tested (94.2%)
Untested implemented operations: 41

### winterbaume-ec2instanceconnect (ec2-instance-connect) - W: 2/2, S: 0/2, M: 1/2, F: 0/2, K: 0/2, C: 0/2

- W[x] S[ ] M[x] F[ ] K[ ] C[ ] SendSSHPublicKey
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] SendSerialConsoleSSHPublicKey

Integration tests: 2/2 implemented operations tested (100.0%)

### winterbaume-ecr (ecr) - W: 58/58, S: 0/58, M: 29/58, F: 0/58, K: 11/58, C: 58/58

Terraform E2E: 8 tests across 6 terraform resource types

Resource types: aws_ecr_lifecycle_policy, aws_ecr_pull_through_cache_rule, aws_ecr_registry_scanning_configuration, aws_ecr_replication_configuration, aws_ecr_repository, aws_ecr_repository_policy

- W[x] S[ ] M[ ] F[ ] K[ ] C[x] BatchCheckLayerAvailability
- W[x] S[ ] M[x] F[ ] K[x] C[x] BatchDeleteImage
- W[x] S[ ] M[x] F[ ] K[x] C[x] BatchGetImage
- W[x] S[ ] M[x] F[ ] K[ ] C[x] BatchGetRepositoryScanningConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CompleteLayerUpload
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreatePullThroughCacheRule
- W[x] S[ ] M[x] F[ ] K[x] C[x] CreateRepository
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateRepositoryCreationTemplate
- W[x] S[ ] M[x] F[ ] K[x] C[x] DeleteLifecyclePolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeletePullThroughCacheRule
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteRegistryPolicy
- W[x] S[ ] M[x] F[ ] K[x] C[x] DeleteRepository
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteRepositoryCreationTemplate
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteRepositoryPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteSigningConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeregisterPullTimeUpdateExclusion
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeImageReplicationStatus
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeImageScanFindings
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeImageSigningStatus
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeImages
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribePullThroughCacheRules
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeRegistry
- W[x] S[ ] M[x] F[ ] K[x] C[x] DescribeRepositories
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeRepositoryCreationTemplates
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetAccountSetting
- W[x] S[ ] M[ ] F[ ] K[x] C[x] GetAuthorizationToken
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetDownloadUrlForLayer
- W[x] S[ ] M[x] F[ ] K[x] C[x] GetLifecyclePolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetLifecyclePolicyPreview
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetRegistryPolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetRegistryScanningConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetRepositoryPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetSigningConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] InitiateLayerUpload
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListImageReferrers
- W[x] S[ ] M[x] F[ ] K[x] C[x] ListImages
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListPullTimeUpdateExclusions
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] PutAccountSetting
- W[x] S[ ] M[x] F[ ] K[x] C[x] PutImage
- W[x] S[ ] M[x] F[ ] K[ ] C[x] PutImageScanningConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] C[x] PutImageTagMutability
- W[x] S[ ] M[x] F[ ] K[x] C[x] PutLifecyclePolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[x] PutRegistryPolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[x] PutRegistryScanningConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] C[x] PutReplicationConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] PutSigningConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] RegisterPullTimeUpdateExclusion
- W[x] S[ ] M[x] F[ ] K[ ] C[x] SetRepositoryPolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[x] StartImageScan
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] StartLifecyclePolicyPreview
- W[x] S[ ] M[x] F[ ] K[ ] C[x] TagResource
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateImageStorageClass
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdatePullThroughCacheRule
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateRepositoryCreationTemplate
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UploadLayerPart
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ValidatePullThroughCacheRule

Integration tests: 44/58 implemented operations tested (75.9%)
Untested implemented operations: 14

### winterbaume-ecs (ecs) - W: 63/76, S: 1/76, M: 45/76, F: 0/76, K: 12/76, C: 76/76

Terraform E2E: 7 tests across 3 terraform resource types

Resource types: aws_ecs_cluster, aws_ecs_service, aws_ecs_task_definition

- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateCapacityProvider
- W[x] S[ ] M[x] F[ ] K[x] C[x] CreateCluster
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] CreateDaemon
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateExpressGatewayService
- W[x] S[ ] M[x] F[ ] K[x] C[x] CreateService
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateTaskSet
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteAccountSetting
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteAttributes
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteCapacityProvider
- W[x] S[ ] M[x] F[ ] K[x] C[x] DeleteCluster
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DeleteDaemon
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DeleteDaemonTaskDefinition
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteExpressGatewayService
- W[x] S[ ] M[x] F[ ] K[x] C[x] DeleteService
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteTaskDefinitions
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteTaskSet
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeregisterContainerInstance
- W[x] S[ ] M[x] F[ ] K[x] C[x] DeregisterTaskDefinition
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeCapacityProviders
- W[x] S[ ] M[x] F[ ] K[x] C[x] DescribeClusters
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeContainerInstances
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DescribeDaemon
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DescribeDaemonDeployments
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DescribeDaemonRevisions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DescribeDaemonTaskDefinition
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeExpressGatewayService
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeServiceDeployments
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeServiceRevisions
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeServices
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeTaskDefinition
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeTaskSets
- W[x] S[ ] M[x] F[ ] K[x] C[x] DescribeTasks
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DiscoverPollEndpoint
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] ExecuteCommand
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetTaskProtection
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListAccountSettings
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListAttributes
- W[x] S[ ] M[x] F[ ] K[x] C[x] ListClusters
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListContainerInstances
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ListDaemonDeployments
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ListDaemonTaskDefinitions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ListDaemons
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListServiceDeployments
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListServices
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListServicesByNamespace
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListTagsForResource
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListTaskDefinitionFamilies
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListTaskDefinitions
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListTasks
- W[x] S[ ] M[x] F[ ] K[ ] C[x] PutAccountSetting
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] PutAccountSettingDefault
- W[x] S[ ] M[x] F[ ] K[ ] C[x] PutAttributes
- W[x] S[ ] M[x] F[ ] K[ ] C[x] PutClusterCapacityProviders
- W[x] S[ ] M[x] F[ ] K[ ] C[x] RegisterContainerInstance
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] RegisterDaemonTaskDefinition
- W[x] S[ ] M[x] F[ ] K[x] C[x] RegisterTaskDefinition
- W[x] S[ ] M[x] F[ ] K[x] C[x] RunTask
- W[x] S[ ] M[x] F[ ] K[ ] C[x] StartTask
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] StopServiceDeployment
- W[x] S[ ] M[x] F[ ] K[x] C[x] StopTask
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] SubmitAttachmentStateChanges
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] SubmitContainerStateChange
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] SubmitTaskStateChange
- W[x] S[ ] M[x] F[ ] K[ ] C[x] TagResource
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UntagResource
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UpdateCapacityProvider
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UpdateCluster
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateClusterSettings
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateContainerAgent
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UpdateContainerInstancesState
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] UpdateDaemon
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateExpressGatewayService
- W[x] S[ ] M[x] F[ ] K[x] C[x] UpdateService
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UpdateServicePrimaryTaskSet
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateTaskProtection
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UpdateTaskSet

Integration tests: 58/63 implemented operations tested (92.1%)
Untested implemented operations: 5

### winterbaume-efs (efs) - W: 31/31, S: 0/31, M: 19/31, F: 0/31, K: 0/31, C: 0/31

Terraform E2E: 7 tests across 3 terraform resource types

Resource types: aws_efs_access_point, aws_efs_file_system, aws_efs_file_system_policy

- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateAccessPoint
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateFileSystem
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateMountTarget
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateReplicationConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateTags
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteAccessPoint
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteFileSystem
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteFileSystemPolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteMountTarget
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteReplicationConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteTags
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeAccessPoints
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeAccountPreferences
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeBackupPolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeFileSystemPolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeFileSystems
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeLifecycleConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeMountTargetSecurityGroups
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeMountTargets
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeReplicationConfigurations
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeTags
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListTagsForResource
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ModifyMountTargetSecurityGroups
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutAccountPreferences
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutBackupPolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] PutFileSystemPolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] PutLifecycleConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] TagResource
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateFileSystem
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateFileSystemProtection

Integration tests: 31/31 implemented operations tested (100.0%)

### winterbaume-eks (eks) - W: 55/64, S: 4/64, M: 17/64, F: 0/64, K: 8/64, C: 64/64

Terraform E2E: 11 tests across 6 terraform resource types

Resource types: aws_eks_access_entry, aws_eks_access_policy_association, aws_eks_addon, aws_eks_cluster, aws_eks_node_group, aws_eks_pod_identity_association

- W[x] S[ ] M[ ] F[ ] K[ ] C[x] AssociateAccessPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] AssociateEncryptionConfig
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] AssociateIdentityProviderConfig
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateAccessEntry
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateAddon
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateCapability
- W[x] S[ ] M[x] F[ ] K[x] C[x] CreateCluster
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] CreateEksAnywhereSubscription
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateFargateProfile
- W[x] S[ ] M[x] F[ ] K[x] C[x] CreateNodegroup
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreatePodIdentityAssociation
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteAccessEntry
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteAddon
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteCapability
- W[x] S[ ] M[x] F[ ] K[x] C[x] DeleteCluster
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DeleteEksAnywhereSubscription
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteFargateProfile
- W[x] S[ ] M[x] F[ ] K[x] C[x] DeleteNodegroup
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeletePodIdentityAssociation
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeregisterCluster
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeAccessEntry
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeAddon
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeAddonConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeAddonVersions
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeCapability
- W[x] S[ ] M[x] F[ ] K[x] C[x] DescribeCluster
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeClusterVersions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DescribeEksAnywhereSubscription
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeFargateProfile
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeIdentityProviderConfig
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] DescribeInsight
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] DescribeInsightsRefresh
- W[x] S[ ] M[x] F[ ] K[x] C[x] DescribeNodegroup
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribePodIdentityAssociation
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeUpdate
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DisassociateAccessPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DisassociateIdentityProviderConfig
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListAccessEntries
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListAccessPolicies
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListAddons
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListAssociatedAccessPolicies
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListCapabilities
- W[x] S[ ] M[x] F[ ] K[x] C[x] ListClusters
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ListEksAnywhereSubscriptions
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListFargateProfiles
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListIdentityProviderConfigs
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] ListInsights
- W[x] S[ ] M[x] F[ ] K[x] C[x] ListNodegroups
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListPodIdentityAssociations
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListUpdates
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] RegisterCluster
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] StartInsightsRefresh
- W[x] S[ ] M[x] F[ ] K[ ] C[x] TagResource
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateAccessEntry
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateAddon
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateCapability
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UpdateClusterConfig
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateClusterVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] UpdateEksAnywhereSubscription
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UpdateNodegroupConfig
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateNodegroupVersion
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdatePodIdentityAssociation

Integration tests: 41/55 implemented operations tested (74.5%)
Untested implemented operations: 14

### winterbaume-elasticache (elasticache) - W: 24/75, S: 0/75, M: 17/75, F: 0/75, K: 7/75, C: 75/75

- W[x] S[ ] M[x] F[ ] K[ ] C[x] AddTagsToResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] AuthorizeCacheSecurityGroupIngress
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] BatchApplyUpdateAction
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] BatchStopUpdateAction
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] CompleteMigration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] CopyServerlessCacheSnapshot
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] CopySnapshot
- W[x] S[ ] M[x] F[ ] K[x] C[x] CreateCacheCluster
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateCacheParameterGroup
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateCacheSecurityGroup
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateCacheSubnetGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] CreateGlobalReplicationGroup
- W[x] S[ ] M[x] F[ ] K[x] C[x] CreateReplicationGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] CreateServerlessCache
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] CreateServerlessCacheSnapshot
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateSnapshot
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateUser
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] CreateUserGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DecreaseNodeGroupsInGlobalReplicationGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DecreaseReplicaCount
- W[x] S[ ] M[x] F[ ] K[x] C[x] DeleteCacheCluster
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteCacheParameterGroup
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteCacheSecurityGroup
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteCacheSubnetGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DeleteGlobalReplicationGroup
- W[x] S[ ] M[x] F[ ] K[x] C[x] DeleteReplicationGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DeleteServerlessCache
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DeleteServerlessCacheSnapshot
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteSnapshot
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteUser
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DeleteUserGroup
- W[x] S[ ] M[x] F[ ] K[x] C[x] DescribeCacheClusters
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DescribeCacheEngineVersions
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeCacheParameterGroups
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DescribeCacheParameters
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeCacheSecurityGroups
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeCacheSubnetGroups
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DescribeEngineDefaultParameters
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DescribeEvents
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DescribeGlobalReplicationGroups
- W[x] S[ ] M[x] F[ ] K[x] C[x] DescribeReplicationGroups
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DescribeReservedCacheNodes
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DescribeReservedCacheNodesOfferings
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DescribeServerlessCacheSnapshots
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DescribeServerlessCaches
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DescribeServiceUpdates
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeSnapshots
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DescribeUpdateActions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DescribeUserGroups
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeUsers
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DisassociateGlobalReplicationGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ExportServerlessCacheSnapshot
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] FailoverGlobalReplicationGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] IncreaseNodeGroupsInGlobalReplicationGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] IncreaseReplicaCount
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ListAllowedNodeTypeModifications
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListTagsForResource
- W[ ] S[ ] M[ ] F[ ] K[x] C[x] ModifyCacheCluster
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ModifyCacheParameterGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ModifyCacheSubnetGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ModifyGlobalReplicationGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ModifyReplicationGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ModifyReplicationGroupShardConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ModifyServerlessCache
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ModifyUser
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ModifyUserGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] PurchaseReservedCacheNodesOffering
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] RebalanceSlotsInGlobalReplicationGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] RebootCacheCluster
- W[x] S[ ] M[x] F[ ] K[ ] C[x] RemoveTagsFromResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ResetCacheParameterGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] RevokeCacheSecurityGroupIngress
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] StartMigration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] TestFailover
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] TestMigration

Integration tests: 24/24 implemented operations tested (100.0%)

### winterbaume-elasticbeanstalk (elastic-beanstalk) - W: 7/47, S: 0/47, M: 0/47, F: 0/47, K: 7/47, C: 0/47

- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AbortEnvironmentUpdate
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ApplyEnvironmentManagedAction
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AssociateEnvironmentOperationsRole
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CheckDNSAvailability
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ComposeEnvironments
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] CreateApplication
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateApplicationVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateConfigurationTemplate
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] CreateEnvironment
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreatePlatformVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateStorageLocation
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] DeleteApplication
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteApplicationVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteConfigurationTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteEnvironmentConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeletePlatformVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeAccountAttributes
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeApplicationVersions
- W[ ] S[ ] M[ ] F[ ] K[x] C[ ] DescribeApplications
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeConfigurationOptions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeConfigurationSettings
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeEnvironmentHealth
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeEnvironmentManagedActionHistory
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeEnvironmentManagedActions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeEnvironmentResources
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] DescribeEnvironments
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeEvents
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeInstancesHealth
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribePlatformVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DisassociateEnvironmentOperationsRole
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListAvailableSolutionStacks
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListPlatformBranches
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListPlatformVersions
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListTagsForResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] RebuildEnvironment
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] RequestEnvironmentInfo
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] RestartAppServer
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] RetrieveEnvironmentInfo
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] SwapEnvironmentCNAMEs
- W[ ] S[ ] M[ ] F[ ] K[x] C[ ] TerminateEnvironment
- W[ ] S[ ] M[ ] F[ ] K[x] C[ ] UpdateApplication
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateApplicationResourceLifecycle
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateApplicationVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateConfigurationTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateEnvironment
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateTagsForResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ValidateConfigurationSettings

Integration tests: 7/7 implemented operations tested (100.0%)

### winterbaume-elasticloadbalancing (elastic-load-balancing) - W: 29/29, S: 0/29, M: 21/29, F: 0/29, K: 0/29, C: 0/29

- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] AddTags
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ApplySecurityGroupsToLoadBalancer
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] AttachLoadBalancerToSubnets
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ConfigureHealthCheck
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateAppCookieStickinessPolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateLBCookieStickinessPolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateLoadBalancer
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateLoadBalancerListeners
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateLoadBalancerPolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteLoadBalancer
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteLoadBalancerListeners
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteLoadBalancerPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeregisterInstancesFromLoadBalancer
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeAccountLimits
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeInstanceHealth
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeLoadBalancerAttributes
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeLoadBalancerPolicies
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeLoadBalancerPolicyTypes
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeLoadBalancers
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeTags
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DetachLoadBalancerFromSubnets
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DisableAvailabilityZonesForLoadBalancer
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] EnableAvailabilityZonesForLoadBalancer
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ModifyLoadBalancerAttributes
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] RegisterInstancesWithLoadBalancer
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] RemoveTags
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] SetLoadBalancerListenerSSLCertificate
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] SetLoadBalancerPoliciesForBackendServer
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] SetLoadBalancerPoliciesOfListener

Integration tests: 21/29 implemented operations tested (72.4%)
Untested implemented operations: 8

### winterbaume-elasticloadbalancingv2 (elastic-load-balancing-v2) - W: 50/51, S: 1/51, M: 33/51, F: 0/51, K: 28/51, C: 51/51

Terraform E2E: 12 tests across 6 terraform resource types

Resource types: aws_lb, aws_lb_listener, aws_lb_listener_rule, aws_lb_target_group, aws_lb_target_group_attachment, aws_lb_trust_store

- W[x] S[ ] M[x] F[ ] K[ ] C[x] AddListenerCertificates
- W[x] S[ ] M[x] F[ ] K[x] C[x] AddTags
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] AddTrustStoreRevocations
- W[x] S[ ] M[x] F[ ] K[x] C[x] CreateListener
- W[x] S[ ] M[x] F[ ] K[x] C[x] CreateLoadBalancer
- W[x] S[ ] M[x] F[ ] K[x] C[x] CreateRule
- W[x] S[ ] M[x] F[ ] K[x] C[x] CreateTargetGroup
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateTrustStore
- W[x] S[ ] M[x] F[ ] K[x] C[x] DeleteListener
- W[x] S[ ] M[x] F[ ] K[x] C[x] DeleteLoadBalancer
- W[x] S[ ] M[x] F[ ] K[x] C[x] DeleteRule
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteSharedTrustStoreAssociation
- W[x] S[ ] M[x] F[ ] K[x] C[x] DeleteTargetGroup
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteTrustStore
- W[x] S[ ] M[x] F[ ] K[x] C[x] DeregisterTargets
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] DescribeAccountLimits
- W[x] S[ ] M[ ] F[ ] K[x] C[x] DescribeCapacityReservation
- W[x] S[ ] M[x] F[ ] K[x] C[x] DescribeListenerAttributes
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeListenerCertificates
- W[x] S[ ] M[x] F[ ] K[x] C[x] DescribeListeners
- W[x] S[ ] M[x] F[ ] K[x] C[x] DescribeLoadBalancerAttributes
- W[x] S[ ] M[x] F[ ] K[x] C[x] DescribeLoadBalancers
- W[x] S[ ] M[x] F[ ] K[x] C[x] DescribeRules
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeSSLPolicies
- W[x] S[ ] M[x] F[ ] K[x] C[x] DescribeTags
- W[x] S[ ] M[ ] F[ ] K[x] C[x] DescribeTargetGroupAttributes
- W[x] S[ ] M[x] F[ ] K[x] C[x] DescribeTargetGroups
- W[x] S[ ] M[x] F[ ] K[x] C[x] DescribeTargetHealth
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeTrustStoreAssociations
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeTrustStoreRevocations
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeTrustStores
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetResourcePolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetTrustStoreCaCertificatesBundle
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetTrustStoreRevocationContent
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ModifyCapacityReservation
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ModifyIpPools
- W[x] S[ ] M[x] F[ ] K[x] C[x] ModifyListener
- W[x] S[ ] M[x] F[ ] K[x] C[x] ModifyListenerAttributes
- W[x] S[ ] M[x] F[ ] K[x] C[x] ModifyLoadBalancerAttributes
- W[x] S[ ] M[x] F[ ] K[x] C[x] ModifyRule
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ModifyTargetGroup
- W[x] S[ ] M[x] F[ ] K[x] C[x] ModifyTargetGroupAttributes
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ModifyTrustStore
- W[x] S[ ] M[x] F[ ] K[x] C[x] RegisterTargets
- W[x] S[ ] M[x] F[ ] K[ ] C[x] RemoveListenerCertificates
- W[x] S[ ] M[x] F[ ] K[x] C[x] RemoveTags
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] RemoveTrustStoreRevocations
- W[x] S[ ] M[x] F[ ] K[ ] C[x] SetIpAddressType
- W[x] S[ ] M[x] F[ ] K[x] C[x] SetRulePriorities
- W[x] S[ ] M[x] F[ ] K[ ] C[x] SetSecurityGroups
- W[x] S[ ] M[x] F[ ] K[ ] C[x] SetSubnets

Integration tests: 38/50 implemented operations tested (76.0%)
Untested implemented operations: 12

### winterbaume-emr (emr) - W: 54/60, S: 2/60, M: 26/60, F: 0/60, K: 0/60, C: 0/60

- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] AddInstanceFleet
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] AddInstanceGroups
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] AddJobFlowSteps
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] AddTags
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CancelSteps
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreatePersistentAppUI
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateSecurityConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateStudio
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateStudioSessionMapping
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteSecurityConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteStudio
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteStudioSessionMapping
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeCluster
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeJobFlows
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeNotebookExecution
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribePersistentAppUI
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeReleaseLabel
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeSecurityConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeStep
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeStudio
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetAutoTerminationPolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetBlockPublicAccessConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetClusterSessionCredentials
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetManagedScalingPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetOnClusterAppUIPresignedURL
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetPersistentAppUIPresignedURL
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetStudioSessionMapping
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListBootstrapActions
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListClusters
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListInstanceFleets
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListInstanceGroups
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListInstances
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListNotebookExecutions
- W[ ] S[x] M[x] F[ ] K[ ] C[ ] ListReleaseLabels
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListSecurityConfigurations
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListSteps
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListStudioSessionMappings
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListStudios
- W[ ] S[x] M[x] F[ ] K[ ] C[ ] ListSupportedInstanceTypes
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ModifyCluster
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ModifyInstanceFleet
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ModifyInstanceGroups
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] PutAutoScalingPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutAutoTerminationPolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] PutBlockPublicAccessConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutManagedScalingPolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] RemoveAutoScalingPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] RemoveAutoTerminationPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] RemoveManagedScalingPolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] RemoveTags
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] RunJobFlow
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] SetKeepJobFlowAliveWhenNoSteps
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] SetTerminationProtection
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] SetUnhealthyNodeReplacement
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] SetVisibleToAllUsers
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] StartNotebookExecution
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] StopNotebookExecution
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] TerminateJobFlows
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateStudio
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateStudioSessionMapping

Integration tests: 34/54 implemented operations tested (63.0%)
Untested implemented operations: 20

### winterbaume-emrcontainers (emr-containers) - W: 23/23, S: 0/23, M: 8/23, F: 0/23, K: 0/23, C: 0/23

- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CancelJobRun
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateJobTemplate
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateManagedEndpoint
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateSecurityConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateVirtualCluster
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteJobTemplate
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteManagedEndpoint
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteVirtualCluster
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeJobRun
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeJobTemplate
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeManagedEndpoint
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeSecurityConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeVirtualCluster
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetManagedEndpointSessionCredentials
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListJobRuns
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListJobTemplates
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListManagedEndpoints
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListSecurityConfigurations
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListTagsForResource
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListVirtualClusters
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] StartJobRun
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UntagResource

Integration tests: 23/23 implemented operations tested (100.0%)

### winterbaume-emrserverless (emr-serverless) - W: 16/22, S: 0/22, M: 11/22, F: 0/22, K: 11/22, C: 0/22

- W[x] S[ ] M[x] F[ ] K[x] C[ ] CancelJobRun
- W[x] S[ ] M[x] F[ ] K[x] C[ ] CreateApplication
- W[x] S[ ] M[x] F[ ] K[x] C[ ] DeleteApplication
- W[x] S[ ] M[x] F[ ] K[x] C[ ] GetApplication
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetDashboardForJobRun
- W[x] S[ ] M[x] F[ ] K[x] C[ ] GetJobRun
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetResourceDashboard
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetSession
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetSessionEndpoint
- W[x] S[ ] M[x] F[ ] K[x] C[ ] ListApplications
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListJobRunAttempts
- W[x] S[ ] M[x] F[ ] K[x] C[ ] ListJobRuns
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListSessions
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListTagsForResource
- W[x] S[ ] M[x] F[ ] K[x] C[ ] StartApplication
- W[x] S[ ] M[x] F[ ] K[x] C[ ] StartJobRun
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartSession
- W[x] S[ ] M[x] F[ ] K[x] C[ ] StopApplication
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] TagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] TerminateSession
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UntagResource
- W[x] S[ ] M[x] F[ ] K[x] C[ ] UpdateApplication

Integration tests: 14/16 implemented operations tested (87.5%)
Untested implemented operations: 2

### winterbaume-eventbridge (eventbridge) - W: 57/57, S: 0/57, M: 45/57, F: 0/57, K: 18/57, C: 57/57

Terraform E2E: 7 tests across 4 terraform resource types

Resource types: aws_cloudwatch_event_archive, aws_cloudwatch_event_bus, aws_cloudwatch_event_rule, aws_cloudwatch_event_target

- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ActivateEventSource
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CancelReplay
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateApiDestination
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateArchive
- W[x] S[ ] M[x] F[ ] K[x] C[x] CreateConnection
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateEndpoint
- W[x] S[ ] M[x] F[ ] K[x] C[x] CreateEventBus
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreatePartnerEventSource
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeactivateEventSource
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeauthorizeConnection
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteApiDestination
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteArchive
- W[x] S[ ] M[x] F[ ] K[x] C[x] DeleteConnection
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteEndpoint
- W[x] S[ ] M[x] F[ ] K[x] C[x] DeleteEventBus
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeletePartnerEventSource
- W[x] S[ ] M[x] F[ ] K[x] C[x] DeleteRule
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeApiDestination
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeArchive
- W[x] S[ ] M[x] F[ ] K[x] C[x] DescribeConnection
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeEndpoint
- W[x] S[ ] M[x] F[ ] K[x] C[x] DescribeEventBus
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeEventSource
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribePartnerEventSource
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeReplay
- W[x] S[ ] M[x] F[ ] K[x] C[x] DescribeRule
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DisableRule
- W[x] S[ ] M[x] F[ ] K[ ] C[x] EnableRule
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListApiDestinations
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListArchives
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListConnections
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListEndpoints
- W[x] S[ ] M[x] F[ ] K[x] C[x] ListEventBuses
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListEventSources
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListPartnerEventSourceAccounts
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListPartnerEventSources
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListReplays
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListRuleNamesByTarget
- W[x] S[ ] M[x] F[ ] K[x] C[x] ListRules
- W[x] S[ ] M[x] F[ ] K[x] C[x] ListTagsForResource
- W[x] S[ ] M[x] F[ ] K[x] C[x] ListTargetsByRule
- W[x] S[ ] M[x] F[ ] K[x] C[x] PutEvents
- W[x] S[ ] M[x] F[ ] K[ ] C[x] PutPartnerEvents
- W[x] S[ ] M[x] F[ ] K[ ] C[x] PutPermission
- W[x] S[ ] M[x] F[ ] K[x] C[x] PutRule
- W[x] S[ ] M[x] F[ ] K[x] C[x] PutTargets
- W[x] S[ ] M[x] F[ ] K[ ] C[x] RemovePermission
- W[x] S[ ] M[x] F[ ] K[x] C[x] RemoveTargets
- W[x] S[ ] M[x] F[ ] K[ ] C[x] StartReplay
- W[x] S[ ] M[x] F[ ] K[x] C[x] TagResource
- W[x] S[ ] M[x] F[ ] K[ ] C[x] TestEventPattern
- W[x] S[ ] M[x] F[ ] K[x] C[x] UntagResource
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UpdateApiDestination
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UpdateArchive
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UpdateConnection
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateEndpoint
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateEventBus

Integration tests: 45/57 implemented operations tested (78.9%)
Untested implemented operations: 12

### winterbaume-firehose (firehose) - W: 12/12, S: 0/12, M: 12/12, F: 0/12, K: 7/12, C: 12/12

Terraform E2E: 3 tests across 1 terraform resource types

Resource types: aws_kinesis_firehose_delivery_stream

- W[x] S[ ] M[x] F[ ] K[x] C[x] CreateDeliveryStream
- W[x] S[ ] M[x] F[ ] K[x] C[x] DeleteDeliveryStream
- W[x] S[ ] M[x] F[ ] K[x] C[x] DescribeDeliveryStream
- W[x] S[ ] M[x] F[ ] K[x] C[x] ListDeliveryStreams
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListTagsForDeliveryStream
- W[x] S[ ] M[x] F[ ] K[x] C[x] PutRecord
- W[x] S[ ] M[x] F[ ] K[x] C[x] PutRecordBatch
- W[x] S[ ] M[x] F[ ] K[ ] C[x] StartDeliveryStreamEncryption
- W[x] S[ ] M[x] F[ ] K[ ] C[x] StopDeliveryStreamEncryption
- W[x] S[ ] M[x] F[ ] K[ ] C[x] TagDeliveryStream
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UntagDeliveryStream
- W[x] S[ ] M[x] F[ ] K[x] C[x] UpdateDestination

Integration tests: 12/12 implemented operations tested (100.0%)

### winterbaume-fis (fis) - W: 8/26, S: 0/26, M: 5/26, F: 0/26, K: 0/26, C: 0/26

- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateExperimentTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateTargetAccountConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteExperimentTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteTargetAccountConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetAction
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetExperiment
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetExperimentTargetAccountConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetExperimentTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetSafetyLever
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetTargetAccountConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetTargetResourceType
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListActions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListExperimentResolvedTargets
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListExperimentTargetAccountConfigurations
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListExperimentTemplates
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListExperiments
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListTagsForResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListTargetAccountConfigurations
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListTargetResourceTypes
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartExperiment
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StopExperiment
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] TagResource
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateExperimentTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateSafetyLeverState
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateTargetAccountConfiguration

Integration tests: 8/8 implemented operations tested (100.0%)

### winterbaume-forecast (forecast) - W: 5/63, S: 0/63, M: 5/63, F: 0/63, K: 17/63, C: 0/63

- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateAutoPredictor
- W[ ] S[ ] M[ ] F[ ] K[x] C[ ] CreateDataset
- W[x] S[ ] M[x] F[ ] K[x] C[ ] CreateDatasetGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateDatasetImportJob
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateExplainability
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateExplainabilityExport
- W[ ] S[ ] M[ ] F[ ] K[x] C[ ] CreateForecast
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateForecastExportJob
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateMonitor
- W[ ] S[ ] M[ ] F[ ] K[x] C[ ] CreatePredictor
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreatePredictorBacktestExportJob
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateWhatIfAnalysis
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateWhatIfForecast
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateWhatIfForecastExport
- W[ ] S[ ] M[ ] F[ ] K[x] C[ ] DeleteDataset
- W[x] S[ ] M[x] F[ ] K[x] C[ ] DeleteDatasetGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteDatasetImportJob
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteExplainability
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteExplainabilityExport
- W[ ] S[ ] M[ ] F[ ] K[x] C[ ] DeleteForecast
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteForecastExportJob
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteMonitor
- W[ ] S[ ] M[ ] F[ ] K[x] C[ ] DeletePredictor
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeletePredictorBacktestExportJob
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteResourceTree
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteWhatIfAnalysis
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteWhatIfForecast
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteWhatIfForecastExport
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeAutoPredictor
- W[ ] S[ ] M[ ] F[ ] K[x] C[ ] DescribeDataset
- W[x] S[ ] M[x] F[ ] K[x] C[ ] DescribeDatasetGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeDatasetImportJob
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeExplainability
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeExplainabilityExport
- W[ ] S[ ] M[ ] F[ ] K[x] C[ ] DescribeForecast
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeForecastExportJob
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeMonitor
- W[ ] S[ ] M[ ] F[ ] K[x] C[ ] DescribePredictor
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribePredictorBacktestExportJob
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeWhatIfAnalysis
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeWhatIfForecast
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeWhatIfForecastExport
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetAccuracyMetrics
- W[x] S[ ] M[x] F[ ] K[x] C[ ] ListDatasetGroups
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListDatasetImportJobs
- W[ ] S[ ] M[ ] F[ ] K[x] C[ ] ListDatasets
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListExplainabilities
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListExplainabilityExports
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListForecastExportJobs
- W[ ] S[ ] M[ ] F[ ] K[x] C[ ] ListForecasts
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListMonitorEvaluations
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListMonitors
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListPredictorBacktestExportJobs
- W[ ] S[ ] M[ ] F[ ] K[x] C[ ] ListPredictors
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListTagsForResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListWhatIfAnalyses
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListWhatIfForecastExports
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListWhatIfForecasts
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ResumeResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StopResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] TagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UntagResource
- W[x] S[ ] M[x] F[ ] K[x] C[ ] UpdateDatasetGroup

Integration tests: 5/5 implemented operations tested (100.0%)

### winterbaume-freetier (freetier) - W: 5/5, S: 0/5, M: 0/5, F: 0/5, K: 0/5, C: 0/5

- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetAccountActivity
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetAccountPlanState
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetFreeTierUsage
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListAccountActivities
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpgradeAccountPlan

Integration tests: 5/5 implemented operations tested (100.0%)

### winterbaume-fsx (fsx) - W: 9/48, S: 0/48, M: 9/48, F: 0/48, K: 0/48, C: 0/48

- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AssociateFileSystemAliases
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CancelDataRepositoryTask
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CopyBackup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CopySnapshotAndUpdateVolume
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateAndAttachS3AccessPoint
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateBackup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateDataRepositoryAssociation
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateDataRepositoryTask
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateFileCache
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateFileSystem
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateFileSystemFromBackup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateSnapshot
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateStorageVirtualMachine
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateVolume
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateVolumeFromBackup
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteBackup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteDataRepositoryAssociation
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteFileCache
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteFileSystem
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteSnapshot
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteStorageVirtualMachine
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteVolume
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeBackups
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeDataRepositoryAssociations
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeDataRepositoryTasks
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeFileCaches
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeFileSystemAliases
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeFileSystems
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeS3AccessPointAttachments
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeSharedVpcConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeSnapshots
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeStorageVirtualMachines
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeVolumes
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DetachAndDeleteS3AccessPoint
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DisassociateFileSystemAliases
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListTagsForResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ReleaseFileSystemNfsV3Locks
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] RestoreVolumeFromSnapshot
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartMisconfiguredStateRecovery
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] TagResource
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UntagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateDataRepositoryAssociation
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateFileCache
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateFileSystem
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateSharedVpcConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateSnapshot
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateStorageVirtualMachine
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateVolume

Integration tests: 9/9 implemented operations tested (100.0%)

### winterbaume-glacier (glacier) - W: 33/33, S: 0/33, M: 10/33, F: 0/33, K: 4/33, C: 0/33

- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] AbortMultipartUpload
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] AbortVaultLock
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] AddTagsToVault
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CompleteMultipartUpload
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CompleteVaultLock
- W[x] S[ ] M[x] F[ ] K[x] C[ ] CreateVault
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteArchive
- W[x] S[ ] M[x] F[ ] K[x] C[ ] DeleteVault
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteVaultAccessPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteVaultNotifications
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeJob
- W[x] S[ ] M[x] F[ ] K[x] C[ ] DescribeVault
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetDataRetrievalPolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetJobOutput
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetVaultAccessPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetVaultLock
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetVaultNotifications
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] InitiateJob
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] InitiateMultipartUpload
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] InitiateVaultLock
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListJobs
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListMultipartUploads
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListParts
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListProvisionedCapacity
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListTagsForVault
- W[x] S[ ] M[x] F[ ] K[x] C[ ] ListVaults
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PurchaseProvisionedCapacity
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] RemoveTagsFromVault
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] SetDataRetrievalPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] SetVaultAccessPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] SetVaultNotifications
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UploadArchive
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UploadMultipartPart

Integration tests: 30/33 implemented operations tested (90.9%)
Untested implemented operations: 3

### winterbaume-glue (glue) - W: 132/265, S: 0/265, M: 96/265, F: 0/265, K: 14/265, C: 265/265

- W[x] S[ ] M[x] F[ ] K[ ] C[x] BatchCreatePartition
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] BatchDeleteConnection
- W[x] S[ ] M[x] F[ ] K[ ] C[x] BatchDeletePartition
- W[x] S[ ] M[x] F[ ] K[ ] C[x] BatchDeleteTable
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] BatchDeleteTableVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] BatchGetBlueprints
- W[x] S[ ] M[x] F[ ] K[ ] C[x] BatchGetCrawlers
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] BatchGetCustomEntityTypes
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] BatchGetDataQualityResult
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] BatchGetDevEndpoints
- W[x] S[ ] M[x] F[ ] K[ ] C[x] BatchGetJobs
- W[x] S[ ] M[x] F[ ] K[ ] C[x] BatchGetPartition
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] BatchGetTableOptimizer
- W[x] S[ ] M[x] F[ ] K[ ] C[x] BatchGetTriggers
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] BatchGetWorkflows
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] BatchPutDataQualityStatisticAnnotation
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] BatchStopJobRun
- W[x] S[ ] M[x] F[ ] K[ ] C[x] BatchUpdatePartition
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] CancelDataQualityRuleRecommendationRun
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] CancelDataQualityRulesetEvaluationRun
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] CancelMLTaskRun
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] CancelStatement
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CheckSchemaVersionValidity
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] CreateBlueprint
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] CreateCatalog
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] CreateClassifier
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] CreateColumnStatisticsTaskSettings
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateConnection
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateCrawler
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] CreateCustomEntityType
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] CreateDataQualityRuleset
- W[x] S[ ] M[x] F[ ] K[x] C[x] CreateDatabase
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateDevEndpoint
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] CreateGlueIdentityCenterConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] CreateIntegration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] CreateIntegrationResourceProperty
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] CreateIntegrationTableProperties
- W[x] S[ ] M[x] F[ ] K[x] C[x] CreateJob
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateMLTransform
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreatePartition
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] CreatePartitionIndex
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateRegistry
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateSchema
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] CreateScript
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateSecurityConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateSession
- W[x] S[ ] M[x] F[ ] K[x] C[x] CreateTable
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] CreateTableOptimizer
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateTrigger
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] CreateUsageProfile
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] CreateUserDefinedFunction
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateWorkflow
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DeleteBlueprint
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DeleteCatalog
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DeleteClassifier
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DeleteColumnStatisticsForPartition
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DeleteColumnStatisticsForTable
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DeleteColumnStatisticsTaskSettings
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteConnection
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DeleteConnectionType
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteCrawler
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DeleteCustomEntityType
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DeleteDataQualityRuleset
- W[x] S[ ] M[x] F[ ] K[x] C[x] DeleteDatabase
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteDevEndpoint
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DeleteGlueIdentityCenterConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DeleteIntegration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DeleteIntegrationResourceProperty
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DeleteIntegrationTableProperties
- W[x] S[ ] M[x] F[ ] K[x] C[x] DeleteJob
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteMLTransform
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeletePartition
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DeletePartitionIndex
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteRegistry
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteResourcePolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteSchema
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteSchemaVersions
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteSecurityConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteSession
- W[x] S[ ] M[x] F[ ] K[x] C[x] DeleteTable
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DeleteTableOptimizer
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteTableVersion
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteTrigger
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DeleteUsageProfile
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DeleteUserDefinedFunction
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteWorkflow
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DescribeConnectionType
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DescribeEntity
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DescribeInboundIntegrations
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DescribeIntegrations
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetBlueprint
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetBlueprintRun
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetBlueprintRuns
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetCatalog
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetCatalogImportStatus
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetCatalogs
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetClassifier
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetClassifiers
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetColumnStatisticsForPartition
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetColumnStatisticsForTable
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetColumnStatisticsTaskRun
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetColumnStatisticsTaskRuns
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetColumnStatisticsTaskSettings
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetConnection
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetConnections
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetCrawler
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetCrawlerMetrics
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetCrawlers
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetCustomEntityType
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetDataCatalogEncryptionSettings
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetDataQualityModel
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetDataQualityModelResult
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetDataQualityResult
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetDataQualityRuleRecommendationRun
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetDataQualityRuleset
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetDataQualityRulesetEvaluationRun
- W[x] S[ ] M[x] F[ ] K[x] C[x] GetDatabase
- W[x] S[ ] M[x] F[ ] K[x] C[x] GetDatabases
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetDataflowGraph
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetDevEndpoint
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetDevEndpoints
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetEntityRecords
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetGlueIdentityCenterConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetIntegrationResourceProperty
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetIntegrationTableProperties
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetJob
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetJobBookmark
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetJobRun
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetJobRuns
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetJobs
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetMLTaskRun
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetMLTaskRuns
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetMLTransform
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetMLTransforms
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetMapping
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetMaterializedViewRefreshTaskRun
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetPartition
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetPartitionIndexes
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetPartitions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetPlan
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetRegistry
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetResourcePolicies
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetResourcePolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetSchema
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetSchemaByDefinition
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetSchemaVersion
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetSchemaVersionsDiff
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetSecurityConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetSecurityConfigurations
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetSession
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetStatement
- W[x] S[ ] M[x] F[ ] K[x] C[x] GetTable
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetTableOptimizer
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetTableVersion
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetTableVersions
- W[x] S[ ] M[x] F[ ] K[x] C[x] GetTables
- W[x] S[ ] M[x] F[ ] K[x] C[x] GetTags
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetTrigger
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetTriggers
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetUnfilteredPartitionMetadata
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetUnfilteredPartitionsMetadata
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetUnfilteredTableMetadata
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetUsageProfile
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetUserDefinedFunction
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetUserDefinedFunctions
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetWorkflow
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetWorkflowRun
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetWorkflowRunProperties
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetWorkflowRuns
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ImportCatalogToGlue
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ListBlueprints
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ListColumnStatisticsTaskRuns
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ListConnectionTypes
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListCrawlers
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListCrawls
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ListCustomEntityTypes
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ListDataQualityResults
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ListDataQualityRuleRecommendationRuns
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ListDataQualityRulesetEvaluationRuns
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ListDataQualityRulesets
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ListDataQualityStatisticAnnotations
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ListDataQualityStatistics
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListDevEndpoints
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ListEntities
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ListIntegrationResourceProperties
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListJobs
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListMLTransforms
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ListMaterializedViewRefreshTaskRuns
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListRegistries
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListSchemaVersions
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListSchemas
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListSessions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ListStatements
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ListTableOptimizerRuns
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListTriggers
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ListUsageProfiles
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListWorkflows
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ModifyIntegration
- W[x] S[ ] M[x] F[ ] K[ ] C[x] PutDataCatalogEncryptionSettings
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] PutDataQualityProfileAnnotation
- W[x] S[ ] M[x] F[ ] K[ ] C[x] PutResourcePolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[x] PutSchemaVersionMetadata
- W[x] S[ ] M[x] F[ ] K[ ] C[x] PutWorkflowRunProperties
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] QuerySchemaVersionMetadata
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] RegisterConnectionType
- W[x] S[ ] M[x] F[ ] K[ ] C[x] RegisterSchemaVersion
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] RemoveSchemaVersionMetadata
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ResetJobBookmark
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ResumeWorkflowRun
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] RunStatement
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] SearchTables
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] StartBlueprintRun
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] StartColumnStatisticsTaskRun
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] StartColumnStatisticsTaskRunSchedule
- W[x] S[ ] M[x] F[ ] K[ ] C[x] StartCrawler
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] StartCrawlerSchedule
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] StartDataQualityRuleRecommendationRun
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] StartDataQualityRulesetEvaluationRun
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] StartExportLabelsTaskRun
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] StartImportLabelsTaskRun
- W[x] S[ ] M[x] F[ ] K[x] C[x] StartJobRun
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] StartMLEvaluationTaskRun
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] StartMLLabelingSetGenerationTaskRun
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] StartMaterializedViewRefreshTaskRun
- W[x] S[ ] M[x] F[ ] K[ ] C[x] StartTrigger
- W[x] S[ ] M[x] F[ ] K[ ] C[x] StartWorkflowRun
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] StopColumnStatisticsTaskRun
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] StopColumnStatisticsTaskRunSchedule
- W[x] S[ ] M[x] F[ ] K[ ] C[x] StopCrawler
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] StopCrawlerSchedule
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] StopMaterializedViewRefreshTaskRun
- W[x] S[ ] M[x] F[ ] K[ ] C[x] StopSession
- W[x] S[ ] M[x] F[ ] K[ ] C[x] StopTrigger
- W[x] S[ ] M[x] F[ ] K[ ] C[x] StopWorkflowRun
- W[x] S[ ] M[x] F[ ] K[x] C[x] TagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] TestConnection
- W[x] S[ ] M[x] F[ ] K[x] C[x] UntagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] UpdateBlueprint
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] UpdateCatalog
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] UpdateClassifier
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] UpdateColumnStatisticsForPartition
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] UpdateColumnStatisticsForTable
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] UpdateColumnStatisticsTaskSettings
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateConnection
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateCrawler
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateCrawlerSchedule
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] UpdateDataQualityRuleset
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UpdateDatabase
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateDevEndpoint
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] UpdateGlueIdentityCenterConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] UpdateIntegrationResourceProperty
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] UpdateIntegrationTableProperties
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateJob
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateJobFromSourceControl
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateMLTransform
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UpdatePartition
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateRegistry
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UpdateSchema
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateSourceControlFromJob
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UpdateTable
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] UpdateTableOptimizer
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateTrigger
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] UpdateUsageProfile
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] UpdateUserDefinedFunction
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UpdateWorkflow

Integration tests: 127/132 implemented operations tested (96.2%)
Untested implemented operations: 5

### winterbaume-greengrass (greengrass) - W: 71/92, S: 0/92, M: 55/92, F: 0/92, K: 0/92, C: 0/92

- W[x] S[ ] M[x] F[ ] K[ ] C[ ] AssociateRoleToGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AssociateServiceRoleToAccount
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateConnectorDefinition
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateConnectorDefinitionVersion
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateCoreDefinition
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateCoreDefinitionVersion
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateDeployment
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateDeviceDefinition
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateDeviceDefinitionVersion
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateFunctionDefinition
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateFunctionDefinitionVersion
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateGroupCertificateAuthority
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateGroupVersion
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateLoggerDefinition
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateLoggerDefinitionVersion
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateResourceDefinition
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateResourceDefinitionVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateSoftwareUpdateJob
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateSubscriptionDefinition
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateSubscriptionDefinitionVersion
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteConnectorDefinition
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteCoreDefinition
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteDeviceDefinition
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteFunctionDefinition
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteGroup
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteLoggerDefinition
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteResourceDefinition
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteSubscriptionDefinition
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DisassociateRoleFromGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DisassociateServiceRoleFromAccount
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetAssociatedRole
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetBulkDeploymentStatus
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetConnectivityInfo
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetConnectorDefinition
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetConnectorDefinitionVersion
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetCoreDefinition
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetCoreDefinitionVersion
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetDeploymentStatus
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetDeviceDefinition
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetDeviceDefinitionVersion
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetFunctionDefinition
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetFunctionDefinitionVersion
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetGroupCertificateAuthority
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetGroupCertificateConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetGroupVersion
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetLoggerDefinition
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetLoggerDefinitionVersion
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetResourceDefinition
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetResourceDefinitionVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetServiceRoleForAccount
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetSubscriptionDefinition
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetSubscriptionDefinitionVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetThingRuntimeConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListBulkDeploymentDetailedReports
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListBulkDeployments
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListConnectorDefinitionVersions
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListConnectorDefinitions
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListCoreDefinitionVersions
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListCoreDefinitions
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListDeployments
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListDeviceDefinitionVersions
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListDeviceDefinitions
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListFunctionDefinitionVersions
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListFunctionDefinitions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListGroupCertificateAuthorities
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListGroupVersions
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListGroups
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListLoggerDefinitionVersions
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListLoggerDefinitions
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListResourceDefinitionVersions
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListResourceDefinitions
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListSubscriptionDefinitionVersions
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListSubscriptionDefinitions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListTagsForResource
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ResetDeployments
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartBulkDeployment
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StopBulkDeployment
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] TagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UntagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateConnectivityInfo
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateConnectorDefinition
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UpdateCoreDefinition
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UpdateDeviceDefinition
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UpdateFunctionDefinition
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UpdateGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateGroupCertificateConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateLoggerDefinition
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UpdateResourceDefinition
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UpdateSubscriptionDefinition
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateThingRuntimeConfiguration

Integration tests: 71/71 implemented operations tested (100.0%)

### winterbaume-guardduty (guardduty) - W: 85/87, S: 2/87, M: 12/87, F: 0/87, K: 0/87, C: 0/87

- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] AcceptAdministratorInvitation
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] AcceptInvitation
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ArchiveFindings
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateDetector
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateFilter
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateIPSet
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateMalwareProtectionPlan
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateMembers
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreatePublishingDestination
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateSampleFindings
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateThreatEntitySet
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateThreatIntelSet
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateTrustedEntitySet
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeclineInvitations
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteDetector
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteFilter
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteIPSet
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteInvitations
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteMalwareProtectionPlan
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteMembers
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeletePublishingDestination
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteThreatEntitySet
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteThreatIntelSet
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteTrustedEntitySet
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeMalwareScans
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeOrganizationConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribePublishingDestination
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DisableOrganizationAdminAccount
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DisassociateFromAdministratorAccount
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DisassociateFromMasterAccount
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DisassociateMembers
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] EnableOrganizationAdminAccount
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetAdministratorAccount
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetCoverageStatistics
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetDetector
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetFilter
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetFindings
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetFindingsStatistics
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetIPSet
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetInvitationsCount
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetMalwareProtectionPlan
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetMalwareScan
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetMalwareScanSettings
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetMasterAccount
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetMemberDetectors
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetMembers
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetOrganizationStatistics
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] GetRemainingFreeTrialDays
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetThreatEntitySet
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetThreatIntelSet
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetTrustedEntitySet
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] GetUsageStatistics
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] InviteMembers
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListCoverage
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListDetectors
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListFilters
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListFindings
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListIPSets
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListInvitations
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListMalwareProtectionPlans
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListMalwareScans
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListMembers
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListOrganizationAdminAccounts
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListPublishingDestinations
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListThreatEntitySets
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListThreatIntelSets
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListTrustedEntitySets
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] SendObjectMalwareScan
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] StartMalwareScan
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] StartMonitoringMembers
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] StopMonitoringMembers
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UnarchiveFindings
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UntagResource
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UpdateDetector
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UpdateFilter
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateFindingsFeedback
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateIPSet
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateMalwareProtectionPlan
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateMalwareScanSettings
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateMemberDetectors
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateOrganizationConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdatePublishingDestination
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateThreatEntitySet
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateThreatIntelSet
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateTrustedEntitySet

Integration tests: 78/85 implemented operations tested (91.8%)
Untested implemented operations: 7

### winterbaume-iam (iam) - W: 154/176, S: 22/176, M: 119/176, F: 0/176, K: 39/176, C: 176/176

Terraform E2E: 35 tests across 11 terraform resource types

Resource types: aws_iam_access_key, aws_iam_group, aws_iam_group_membership, aws_iam_group_policy_attachment, aws_iam_instance_profile, aws_iam_policy, aws_iam_role, aws_iam_role_policy, aws_iam_role_policy_attachment, aws_iam_user, aws_iam_user_policy_attachment

- W[ ] S[x] M[ ] F[ ] K[ ] C[x] AcceptDelegationRequest
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] AddClientIDToOpenIDConnectProvider
- W[x] S[ ] M[x] F[ ] K[x] C[x] AddRoleToInstanceProfile
- W[x] S[ ] M[x] F[ ] K[ ] C[x] AddUserToGroup
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] AssociateDelegationRequest
- W[x] S[ ] M[x] F[ ] K[ ] C[x] AttachGroupPolicy
- W[x] S[ ] M[x] F[ ] K[x] C[x] AttachRolePolicy
- W[x] S[ ] M[x] F[ ] K[x] C[x] AttachUserPolicy
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] ChangePassword
- W[x] S[ ] M[x] F[ ] K[x] C[x] CreateAccessKey
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateAccountAlias
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] CreateDelegationRequest
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateGroup
- W[x] S[ ] M[x] F[ ] K[x] C[x] CreateInstanceProfile
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateLoginProfile
- W[x] S[ ] M[x] F[ ] K[x] C[x] CreateOpenIDConnectProvider
- W[x] S[ ] M[x] F[ ] K[x] C[x] CreatePolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreatePolicyVersion
- W[x] S[ ] M[x] F[ ] K[x] C[x] CreateRole
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateSAMLProvider
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateServiceLinkedRole
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateServiceSpecificCredential
- W[x] S[ ] M[x] F[ ] K[x] C[x] CreateUser
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateVirtualMFADevice
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeactivateMFADevice
- W[x] S[ ] M[x] F[ ] K[x] C[x] DeleteAccessKey
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteAccountAlias
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteAccountPasswordPolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteGroup
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteGroupPolicy
- W[x] S[ ] M[x] F[ ] K[x] C[x] DeleteInstanceProfile
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteLoginProfile
- W[x] S[ ] M[x] F[ ] K[x] C[x] DeleteOpenIDConnectProvider
- W[x] S[ ] M[x] F[ ] K[x] C[x] DeletePolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeletePolicyVersion
- W[x] S[ ] M[x] F[ ] K[x] C[x] DeleteRole
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteRolePermissionsBoundary
- W[x] S[ ] M[x] F[ ] K[x] C[x] DeleteRolePolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteSAMLProvider
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteSSHPublicKey
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteServerCertificate
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteServiceLinkedRole
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteServiceSpecificCredential
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteSigningCertificate
- W[x] S[ ] M[x] F[ ] K[x] C[x] DeleteUser
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteUserPermissionsBoundary
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteUserPolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteVirtualMFADevice
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DetachGroupPolicy
- W[x] S[ ] M[x] F[ ] K[x] C[x] DetachRolePolicy
- W[x] S[ ] M[x] F[ ] K[x] C[x] DetachUserPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DisableOrganizationsRootCredentialsManagement
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DisableOrganizationsRootSessions
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] DisableOutboundWebIdentityFederation
- W[x] S[ ] M[x] F[ ] K[ ] C[x] EnableMFADevice
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] EnableOrganizationsRootCredentialsManagement
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] EnableOrganizationsRootSessions
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] EnableOutboundWebIdentityFederation
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GenerateCredentialReport
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] GenerateOrganizationsAccessReport
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] GenerateServiceLastAccessedDetails
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetAccessKeyLastUsed
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetAccountAuthorizationDetails
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetAccountPasswordPolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetAccountSummary
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] GetContextKeysForCustomPolicy
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] GetContextKeysForPrincipalPolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetCredentialReport
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] GetDelegationRequest
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetGroup
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetGroupPolicy
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] GetHumanReadableSummary
- W[x] S[ ] M[x] F[ ] K[x] C[x] GetInstanceProfile
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetLoginProfile
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetMFADevice
- W[x] S[ ] M[x] F[ ] K[x] C[x] GetOpenIDConnectProvider
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] GetOrganizationsAccessReport
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] GetOutboundWebIdentityFederationInfo
- W[x] S[ ] M[x] F[ ] K[x] C[x] GetPolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetPolicyVersion
- W[x] S[ ] M[x] F[ ] K[x] C[x] GetRole
- W[x] S[ ] M[x] F[ ] K[x] C[x] GetRolePolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetSAMLProvider
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetSSHPublicKey
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetServerCertificate
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] GetServiceLastAccessedDetails
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] GetServiceLastAccessedDetailsWithEntities
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetServiceLinkedRoleDeletionStatus
- W[x] S[ ] M[x] F[ ] K[x] C[x] GetUser
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetUserPolicy
- W[x] S[ ] M[x] F[ ] K[x] C[x] ListAccessKeys
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListAccountAliases
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListAttachedGroupPolicies
- W[x] S[ ] M[x] F[ ] K[x] C[x] ListAttachedRolePolicies
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListAttachedUserPolicies
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] ListDelegationRequests
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListEntitiesForPolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListGroupPolicies
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListGroups
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListGroupsForUser
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListInstanceProfileTags
- W[x] S[ ] M[ ] F[ ] K[x] C[x] ListInstanceProfiles
- W[x] S[ ] M[ ] F[ ] K[x] C[x] ListInstanceProfilesForRole
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListMFADeviceTags
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListMFADevices
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListOpenIDConnectProviderTags
- W[x] S[ ] M[x] F[ ] K[x] C[x] ListOpenIDConnectProviders
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] ListOrganizationsFeatures
- W[x] S[ ] M[x] F[ ] K[x] C[x] ListPolicies
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] ListPoliciesGrantingServiceAccess
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListPolicyTags
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListPolicyVersions
- W[x] S[ ] M[x] F[ ] K[x] C[x] ListRolePolicies
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListRoleTags
- W[x] S[ ] M[x] F[ ] K[x] C[x] ListRoles
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListSAMLProviderTags
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListSAMLProviders
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListSSHPublicKeys
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListServerCertificateTags
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListServerCertificates
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListServiceSpecificCredentials
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListSigningCertificates
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListUserPolicies
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListUserTags
- W[x] S[ ] M[x] F[ ] K[x] C[x] ListUsers
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListVirtualMFADevices
- W[x] S[ ] M[x] F[ ] K[ ] C[x] PutGroupPolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[x] PutRolePermissionsBoundary
- W[x] S[ ] M[x] F[ ] K[x] C[x] PutRolePolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] PutUserPermissionsBoundary
- W[x] S[ ] M[x] F[ ] K[ ] C[x] PutUserPolicy
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] RejectDelegationRequest
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] RemoveClientIDFromOpenIDConnectProvider
- W[x] S[ ] M[x] F[ ] K[x] C[x] RemoveRoleFromInstanceProfile
- W[x] S[ ] M[x] F[ ] K[ ] C[x] RemoveUserFromGroup
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ResetServiceSpecificCredential
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ResyncMFADevice
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] SendDelegationToken
- W[x] S[ ] M[x] F[ ] K[ ] C[x] SetDefaultPolicyVersion
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] SetSecurityTokenServicePreferences
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] SimulateCustomPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] SimulatePrincipalPolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[x] TagInstanceProfile
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] TagMFADevice
- W[x] S[ ] M[x] F[ ] K[ ] C[x] TagOpenIDConnectProvider
- W[x] S[ ] M[x] F[ ] K[ ] C[x] TagPolicy
- W[x] S[ ] M[x] F[ ] K[x] C[x] TagRole
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] TagSAMLProvider
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] TagServerCertificate
- W[x] S[ ] M[x] F[ ] K[ ] C[x] TagUser
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UntagInstanceProfile
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UntagMFADevice
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UntagOpenIDConnectProvider
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UntagPolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UntagRole
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UntagSAMLProvider
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UntagServerCertificate
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UntagUser
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UpdateAccessKey
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UpdateAccountPasswordPolicy
- W[x] S[ ] M[x] F[ ] K[x] C[x] UpdateAssumeRolePolicy
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] UpdateDelegationRequest
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UpdateGroup
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UpdateLoginProfile
- W[x] S[ ] M[x] F[ ] K[x] C[x] UpdateOpenIDConnectProviderThumbprint
- W[x] S[ ] M[x] F[ ] K[x] C[x] UpdateRole
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UpdateRoleDescription
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UpdateSAMLProvider
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UpdateSSHPublicKey
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateServerCertificate
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateServiceSpecificCredential
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UpdateSigningCertificate
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UpdateUser
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UploadSSHPublicKey
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UploadServerCertificate
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UploadSigningCertificate

Integration tests: 135/154 implemented operations tested (87.7%)
Untested implemented operations: 19

### winterbaume-identitystore (identitystore) - W: 17/19, S: 0/19, M: 14/19, F: 0/19, K: 0/19, C: 19/19

- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateGroup
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateGroupMembership
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateUser
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteGroup
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteGroupMembership
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteUser
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeGroup
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeGroupMembership
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeUser
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetGroupId
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetGroupMembershipId
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetUserId
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] IsMemberInGroups
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListGroupMemberships
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListGroupMembershipsForMember
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListGroups
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListUsers
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateGroup
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateUser

Integration tests: 14/17 implemented operations tested (82.4%)
Untested implemented operations: 3

### winterbaume-inspector2 (inspector2) - W: 49/75, S: 21/75, M: 19/75, F: 0/75, K: 0/75, C: 0/75

- W[x] S[ ] M[x] F[ ] K[ ] C[ ] AssociateMember
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] BatchAssociateCodeSecurityScanConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] BatchDisassociateCodeSecurityScanConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] BatchGetAccountStatus
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] BatchGetCodeSnippet
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] BatchGetFindingDetails
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] BatchGetFreeTrialInfo
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] BatchGetMemberEc2DeepInspectionStatus
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] BatchUpdateMemberEc2DeepInspectionStatus
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CancelFindingsReport
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CancelSbomExport
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateCisScanConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateCodeSecurityIntegration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateCodeSecurityScanConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateFilter
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateFindingsReport
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateSbomExport
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteCisScanConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteCodeSecurityIntegration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteCodeSecurityScanConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteFilter
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeOrganizationConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] Disable
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DisableDelegatedAdminAccount
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DisassociateMember
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] Enable
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] EnableDelegatedAdminAccount
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] GetCisScanReport
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] GetCisScanResultDetails
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] GetClustersForImage
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetCodeSecurityIntegration
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] GetCodeSecurityScan
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetCodeSecurityScanConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetDelegatedAdminAccount
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetEc2DeepInspectionConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetEncryptionKey
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetFindingsReportStatus
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetMember
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetSbomExport
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] ListAccountPermissions
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListCisScanConfigurations
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] ListCisScanResultsAggregatedByChecks
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] ListCisScanResultsAggregatedByTargetResource
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] ListCisScans
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListCodeSecurityIntegrations
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListCodeSecurityScanConfigurationAssociations
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListCodeSecurityScanConfigurations
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] ListCoverage
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] ListCoverageStatistics
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListDelegatedAdminAccounts
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListFilters
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] ListFindingAggregations
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListFindings
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListMembers
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListTagsForResource
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] ListUsageTotals
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ResetEncryptionKey
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] SearchVulnerabilities
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] SendCisSessionHealth
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] SendCisSessionTelemetry
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] StartCisSession
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] StartCodeSecurityScan
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] StopCisSession
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] TagResource
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateCisScanConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateCodeSecurityIntegration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateCodeSecurityScanConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateEc2DeepInspectionConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateEncryptionKey
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateFilter
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateOrgEc2DeepInspectionConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UpdateOrganizationConfiguration

Integration tests: 38/49 implemented operations tested (77.6%)
Untested implemented operations: 11

### winterbaume-iot (iot) - W: 103/272, S: 0/272, M: 100/272, F: 0/272, K: 0/272, C: 0/272

- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AcceptCertificateTransfer
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] AddThingToBillingGroup
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] AddThingToThingGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AssociateSbomWithPackageVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AssociateTargetsWithJob
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] AttachPolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] AttachPrincipalPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AttachSecurityProfile
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] AttachThingPrincipal
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CancelAuditMitigationActionsTask
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CancelAuditTask
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CancelCertificateTransfer
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CancelDetectMitigationActionsTask
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CancelJob
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CancelJobExecution
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ClearDefaultAuthorizer
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ConfirmTopicRuleDestination
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateAuditSuppression
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateAuthorizer
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateBillingGroup
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateCertificateFromCsr
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateCertificateProvider
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateCommand
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateCustomMetric
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateDimension
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateDomainConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateDynamicThingGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateFleetMetric
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateJob
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateJobTemplate
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateKeysAndCertificate
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateMitigationAction
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateOTAUpdate
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreatePackage
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreatePackageVersion
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreatePolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreatePolicyVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateProvisioningClaim
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateProvisioningTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateProvisioningTemplateVersion
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateRoleAlias
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateScheduledAudit
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateSecurityProfile
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateStream
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateThing
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateThingGroup
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateThingType
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateTopicRule
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateTopicRuleDestination
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteAccountAuditConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteAuditSuppression
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteAuthorizer
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteBillingGroup
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteCACertificate
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteCertificate
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteCertificateProvider
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteCommand
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteCommandExecution
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteCustomMetric
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteDimension
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteDomainConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteDynamicThingGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteFleetMetric
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteJob
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteJobExecution
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteJobTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteMitigationAction
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteOTAUpdate
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeletePackage
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeletePackageVersion
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeletePolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeletePolicyVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteProvisioningTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteProvisioningTemplateVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteRegistrationCode
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteRoleAlias
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteScheduledAudit
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteSecurityProfile
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteStream
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteThing
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteThingGroup
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteThingType
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteTopicRule
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteTopicRuleDestination
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteV2LoggingLevel
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeprecateThingType
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeAccountAuditConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeAuditFinding
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeAuditMitigationActionsTask
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeAuditSuppression
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeAuditTask
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeAuthorizer
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeBillingGroup
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeCACertificate
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeCertificate
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeCertificateProvider
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeCustomMetric
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeDefaultAuthorizer
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeDetectMitigationActionsTask
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeDimension
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeDomainConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeEncryptionConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeEndpoint
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeEventConfigurations
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeFleetMetric
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeIndex
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeJob
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeJobExecution
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeJobTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeManagedJobTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeMitigationAction
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeProvisioningTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeProvisioningTemplateVersion
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeRoleAlias
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeScheduledAudit
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeSecurityProfile
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeStream
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeThing
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeThingGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeThingRegistrationTask
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeThingType
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DetachPolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DetachPrincipalPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DetachSecurityProfile
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DetachThingPrincipal
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DisableTopicRule
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DisassociateSbomFromPackageVersion
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] EnableTopicRule
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetBehaviorModelTrainingSummaries
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetBucketsAggregation
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetCardinality
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetCommand
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetCommandExecution
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetEffectivePolicies
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetIndexingConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetJobDocument
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetLoggingOptions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetOTAUpdate
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetPackage
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetPackageConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetPackageVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetPercentiles
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetPolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetPolicyVersion
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetRegistrationCode
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetStatistics
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetThingConnectivityData
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetTopicRule
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetTopicRuleDestination
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetV2LoggingOptions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListActiveViolations
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListAttachedPolicies
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListAuditFindings
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListAuditMitigationActionsExecutions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListAuditMitigationActionsTasks
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListAuditSuppressions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListAuditTasks
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListAuthorizers
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListBillingGroups
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListCACertificates
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListCertificateProviders
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListCertificates
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListCertificatesByCA
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListCommandExecutions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListCommands
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListCustomMetrics
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListDetectMitigationActionsExecutions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListDetectMitigationActionsTasks
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListDimensions
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListDomainConfigurations
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListFleetMetrics
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListIndices
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListJobExecutionsForJob
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListJobExecutionsForThing
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListJobTemplates
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListJobs
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListManagedJobTemplates
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListMetricValues
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListMitigationActions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListOTAUpdates
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListOutgoingCertificates
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListPackageVersions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListPackages
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListPolicies
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListPolicyPrincipals
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListPolicyVersions
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListPrincipalPolicies
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListPrincipalThings
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListPrincipalThingsV2
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListProvisioningTemplateVersions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListProvisioningTemplates
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListRelatedResourcesForAuditFinding
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListRoleAliases
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListSbomValidationResults
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListScheduledAudits
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListSecurityProfiles
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListSecurityProfilesForTarget
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListStreams
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListTagsForResource
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListTargetsForPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListTargetsForSecurityProfile
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListThingGroups
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListThingGroupsForThing
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListThingPrincipals
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListThingPrincipalsV2
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListThingRegistrationTaskReports
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListThingRegistrationTasks
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListThingTypes
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListThings
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListThingsInBillingGroup
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListThingsInThingGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListTopicRuleDestinations
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListTopicRules
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListV2LoggingLevels
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListViolationEvents
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] PutVerificationStateOnViolation
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] RegisterCACertificate
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] RegisterCertificate
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] RegisterCertificateWithoutCA
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] RegisterThing
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] RejectCertificateTransfer
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] RemoveThingFromBillingGroup
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] RemoveThingFromThingGroup
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ReplaceTopicRule
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] SearchIndex
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] SetDefaultAuthorizer
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] SetDefaultPolicyVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] SetLoggingOptions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] SetV2LoggingLevel
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] SetV2LoggingOptions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartAuditMitigationActionsTask
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartDetectMitigationActionsTask
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartOnDemandAuditTask
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartThingRegistrationTask
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StopThingRegistrationTask
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] TagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] TestAuthorization
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] TestInvokeAuthorizer
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] TransferCertificate
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UntagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateAccountAuditConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateAuditSuppression
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateAuthorizer
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UpdateBillingGroup
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UpdateCACertificate
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UpdateCertificate
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateCertificateProvider
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateCommand
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateCustomMetric
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateDimension
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UpdateDomainConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateDynamicThingGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateEncryptionConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateEventConfigurations
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateFleetMetric
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UpdateIndexingConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateJob
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateMitigationAction
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdatePackage
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdatePackageConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdatePackageVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateProvisioningTemplate
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UpdateRoleAlias
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateScheduledAudit
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateSecurityProfile
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateStream
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UpdateThing
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UpdateThingGroup
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UpdateThingGroupsForThing
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateThingType
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateTopicRuleDestination
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ValidateSecurityProfileBehaviors

Integration tests: 99/103 implemented operations tested (96.1%)
Untested implemented operations: 4

### winterbaume-iotdataplane (iot-data-plane) - W: 8/8, S: 0/8, M: 0/8, F: 0/8, K: 0/8, C: 0/8

- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteConnection
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteThingShadow
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetRetainedMessage
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetThingShadow
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListNamedShadowsForThing
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListRetainedMessages
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] Publish
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateThingShadow

Integration tests: 5/8 implemented operations tested (62.5%)
Untested implemented operations: 3

### winterbaume-ivs (ivs) - W: 30/40, S: 5/40, M: 6/40, F: 0/40, K: 0/40, C: 0/40

- W[x] S[ ] M[x] F[ ] K[ ] C[ ] BatchGetChannel
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] BatchGetStreamKey
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] BatchStartViewerSessionRevocation
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateAdConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateChannel
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreatePlaybackRestrictionPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateRecordingConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateStreamKey
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteAdConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteChannel
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeletePlaybackKeyPair
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeletePlaybackRestrictionPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteRecordingConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteStreamKey
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetAdConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetChannel
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetPlaybackKeyPair
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetPlaybackRestrictionPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetRecordingConfiguration
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] GetStream
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetStreamKey
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] GetStreamSession
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ImportPlaybackKeyPair
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] InsertAdBreak
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListAdConfigurations
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListChannels
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListPlaybackKeyPairs
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListPlaybackRestrictionPolicies
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListRecordingConfigurations
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListStreamKeys
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] ListStreamSessions
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] ListStreams
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutMetadata
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] StartViewerSessionRevocation
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] StopStream
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UntagResource
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UpdateChannel
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdatePlaybackRestrictionPolicy

Integration tests: 30/30 implemented operations tested (100.0%)

### winterbaume-kafka (kafka) - W: 10/59, S: 0/59, M: 13/59, F: 0/59, K: 6/59, C: 0/59

- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] BatchAssociateScramSecret
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] BatchDisassociateScramSecret
- W[x] S[ ] M[x] F[ ] K[x] C[ ] CreateCluster
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateClusterV2
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateReplicator
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateTopic
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateVpcConnection
- W[x] S[ ] M[x] F[ ] K[x] C[ ] DeleteCluster
- W[ ] S[ ] M[x] F[ ] K[ ] C[ ] DeleteClusterPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteReplicator
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteTopic
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteVpcConnection
- W[x] S[ ] M[x] F[ ] K[x] C[ ] DescribeCluster
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeClusterOperation
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeClusterOperationV2
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeClusterV2
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeConfigurationRevision
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeReplicator
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeTopic
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeTopicPartitions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeVpcConnection
- W[ ] S[ ] M[ ] F[ ] K[x] C[ ] GetBootstrapBrokers
- W[ ] S[ ] M[x] F[ ] K[ ] C[ ] GetClusterPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetCompatibleKafkaVersions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListClientVpcConnections
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListClusterOperations
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListClusterOperationsV2
- W[x] S[ ] M[x] F[ ] K[x] C[ ] ListClusters
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListClustersV2
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListConfigurationRevisions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListConfigurations
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListKafkaVersions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListNodes
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListReplicators
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListScramSecrets
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListTagsForResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListTopics
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListVpcConnections
- W[ ] S[ ] M[x] F[ ] K[ ] C[ ] PutClusterPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] RebootBroker
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] RejectClientVpcConnection
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] TagResource
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UntagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateBrokerCount
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateBrokerStorage
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateBrokerType
- W[ ] S[ ] M[ ] F[ ] K[x] C[ ] UpdateClusterConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateClusterKafkaVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateConnectivity
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateMonitoring
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateRebalancing
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateReplicationInfo
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateSecurity
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateStorage
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateTopic

Integration tests: 10/10 implemented operations tested (100.0%)

### winterbaume-keyspaces (keyspaces) - W: 19/19, S: 0/19, M: 0/19, F: 0/19, K: 0/19, C: 0/19

- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateKeyspace
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateTable
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateType
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteKeyspace
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteTable
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteType
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetKeyspace
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetTable
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetTableAutoScalingSettings
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetType
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListKeyspaces
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListTables
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListTypes
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] RestoreTable
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateKeyspace
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateTable

Integration tests: 13/19 implemented operations tested (68.4%)
Untested implemented operations: 6

### winterbaume-kinesis (kinesis) - W: 38/39, S: 0/39, M: 31/39, F: 0/39, K: 10/39, C: 39/39

Terraform E2E: 4 tests across 1 terraform resource types

Resource types: aws_kinesis_stream

- W[x] S[ ] M[x] F[ ] K[ ] C[x] AddTagsToStream
- W[x] S[ ] M[x] F[ ] K[x] C[x] CreateStream
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DecreaseStreamRetentionPeriod
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteResourcePolicy
- W[x] S[ ] M[x] F[ ] K[x] C[x] DeleteStream
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeregisterStreamConsumer
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeAccountSettings
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeLimits
- W[x] S[ ] M[x] F[ ] K[x] C[x] DescribeStream
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeStreamConsumer
- W[x] S[ ] M[x] F[ ] K[x] C[x] DescribeStreamSummary
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DisableEnhancedMonitoring
- W[x] S[ ] M[x] F[ ] K[ ] C[x] EnableEnhancedMonitoring
- W[x] S[ ] M[x] F[ ] K[x] C[x] GetRecords
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetResourcePolicy
- W[x] S[ ] M[x] F[ ] K[x] C[x] GetShardIterator
- W[x] S[ ] M[x] F[ ] K[ ] C[x] IncreaseStreamRetentionPeriod
- W[x] S[ ] M[x] F[ ] K[x] C[x] ListShards
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListStreamConsumers
- W[x] S[ ] M[x] F[ ] K[x] C[x] ListStreams
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListTagsForResource
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListTagsForStream
- W[x] S[ ] M[x] F[ ] K[ ] C[x] MergeShards
- W[x] S[ ] M[x] F[ ] K[x] C[x] PutRecord
- W[x] S[ ] M[x] F[ ] K[x] C[x] PutRecords
- W[x] S[ ] M[x] F[ ] K[ ] C[x] PutResourcePolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[x] RegisterStreamConsumer
- W[x] S[ ] M[x] F[ ] K[ ] C[x] RemoveTagsFromStream
- W[x] S[ ] M[x] F[ ] K[ ] C[x] SplitShard
- W[x] S[ ] M[x] F[ ] K[ ] C[x] StartStreamEncryption
- W[x] S[ ] M[x] F[ ] K[ ] C[x] StopStreamEncryption
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] SubscribeToShard
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateAccountSettings
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateMaxRecordSize
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UpdateShardCount
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UpdateStreamMode
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateStreamWarmThroughput

Integration tests: 31/38 implemented operations tested (81.6%)
Untested implemented operations: 7

### winterbaume-kinesisanalyticsv2 (kinesis-analytics-v2) - W: 32/33, S: 1/33, M: 0/33, F: 0/33, K: 0/33, C: 0/33

- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] AddApplicationCloudWatchLoggingOption
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] AddApplicationInput
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] AddApplicationInputProcessingConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] AddApplicationOutput
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] AddApplicationReferenceDataSource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] AddApplicationVpcConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateApplication
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateApplicationPresignedUrl
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateApplicationSnapshot
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteApplication
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteApplicationCloudWatchLoggingOption
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteApplicationInputProcessingConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteApplicationOutput
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteApplicationReferenceDataSource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteApplicationSnapshot
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteApplicationVpcConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeApplication
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeApplicationOperation
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeApplicationSnapshot
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeApplicationVersion
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] DiscoverInputSchema
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListApplicationOperations
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListApplicationSnapshots
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListApplicationVersions
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListApplications
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] RollbackApplication
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] StartApplication
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] StopApplication
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateApplication
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateApplicationMaintenanceConfiguration

Integration tests: 25/32 implemented operations tested (78.1%)
Untested implemented operations: 7

### winterbaume-kinesisvideo (kinesis-video) - W: 32/32, S: 0/32, M: 0/32, F: 0/32, K: 0/32, C: 0/32

- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateSignalingChannel
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateStream
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteEdgeConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteSignalingChannel
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteStream
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeEdgeConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeImageGenerationConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeMappedResourceConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeMediaStorageConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeNotificationConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeSignalingChannel
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeStream
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeStreamStorageConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetDataEndpoint
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetSignalingChannelEndpoint
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListEdgeAgentConfigurations
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListSignalingChannels
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListStreams
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListTagsForStream
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] StartEdgeConfigurationUpdate
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] TagStream
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UntagStream
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateDataRetention
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateImageGenerationConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateMediaStorageConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateNotificationConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateSignalingChannel
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateStream
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateStreamStorageConfiguration

Integration tests: 32/32 implemented operations tested (100.0%)

### winterbaume-kinesisvideoarchivedmedia (kinesis-video-archived-media) - W: 6/6, S: 0/6, M: 3/6, F: 0/6, K: 0/6, C: 0/6

- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetClip
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetDASHStreamingSessionURL
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetHLSStreamingSessionURL
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetImages
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetMediaForFragmentList
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListFragments

Integration tests: 6/6 implemented operations tested (100.0%)

### winterbaume-kms (kms) - W: 53/54, S: 0/54, M: 40/54, F: 0/54, K: 22/54, C: 54/54

Terraform E2E: 4 tests across 2 terraform resource types

Resource types: aws_kms_alias, aws_kms_key

- W[x] S[ ] M[x] F[ ] K[ ] C[x] CancelKeyDeletion
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ConnectCustomKeyStore
- W[x] S[ ] M[x] F[ ] K[x] C[x] CreateAlias
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateCustomKeyStore
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateGrant
- W[x] S[ ] M[x] F[ ] K[x] C[x] CreateKey
- W[x] S[ ] M[x] F[ ] K[x] C[x] Decrypt
- W[x] S[ ] M[x] F[ ] K[x] C[x] DeleteAlias
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteCustomKeyStore
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteImportedKeyMaterial
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeriveSharedSecret
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeCustomKeyStores
- W[x] S[ ] M[x] F[ ] K[x] C[x] DescribeKey
- W[x] S[ ] M[x] F[ ] K[x] C[x] DisableKey
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DisableKeyRotation
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DisconnectCustomKeyStore
- W[x] S[ ] M[x] F[ ] K[x] C[x] EnableKey
- W[x] S[ ] M[x] F[ ] K[ ] C[x] EnableKeyRotation
- W[x] S[ ] M[x] F[ ] K[x] C[x] Encrypt
- W[x] S[ ] M[x] F[ ] K[x] C[x] GenerateDataKey
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GenerateDataKeyPair
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GenerateDataKeyPairWithoutPlaintext
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GenerateDataKeyWithoutPlaintext
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GenerateMac
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GenerateRandom
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetKeyLastUsage
- W[x] S[ ] M[x] F[ ] K[x] C[x] GetKeyPolicy
- W[x] S[ ] M[x] F[ ] K[x] C[x] GetKeyRotationStatus
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetParametersForImport
- W[x] S[ ] M[x] F[ ] K[x] C[x] GetPublicKey
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ImportKeyMaterial
- W[x] S[ ] M[x] F[ ] K[x] C[x] ListAliases
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListGrants
- W[x] S[ ] M[x] F[ ] K[x] C[x] ListKeyPolicies
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListKeyRotations
- W[x] S[ ] M[x] F[ ] K[x] C[x] ListKeys
- W[x] S[ ] M[x] F[ ] K[x] C[x] ListResourceTags
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListRetirableGrants
- W[x] S[ ] M[x] F[ ] K[x] C[x] PutKeyPolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ReEncrypt
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ReplicateKey
- W[x] S[ ] M[x] F[ ] K[ ] C[x] RetireGrant
- W[x] S[ ] M[x] F[ ] K[ ] C[x] RevokeGrant
- W[x] S[ ] M[x] F[ ] K[ ] C[x] RotateKeyOnDemand
- W[x] S[ ] M[x] F[ ] K[x] C[x] ScheduleKeyDeletion
- W[x] S[ ] M[x] F[ ] K[x] C[x] Sign
- W[x] S[ ] M[x] F[ ] K[x] C[x] TagResource
- W[x] S[ ] M[x] F[ ] K[x] C[x] UntagResource
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UpdateAlias
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateCustomKeyStore
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UpdateKeyDescription
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdatePrimaryRegion
- W[x] S[ ] M[x] F[ ] K[x] C[x] Verify
- W[x] S[ ] M[x] F[ ] K[ ] C[x] VerifyMac

Integration tests: 40/53 implemented operations tested (75.5%)
Untested implemented operations: 13

### winterbaume-lakeformation (lakeformation) - W: 19/61, S: 1/61, M: 20/61, F: 0/61, K: 0/61, C: 0/61

- W[x] S[ ] M[x] F[ ] K[ ] C[ ] AddLFTagsToResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AssumeDecoratedRoleWithSAML
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] BatchGrantPermissions
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] BatchRevokePermissions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CancelTransaction
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CommitTransaction
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateDataCellsFilter
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateLFTag
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateLFTagExpression
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateLakeFormationIdentityCenterConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateLakeFormationOptIn
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteDataCellsFilter
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteLFTag
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteLFTagExpression
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteLakeFormationIdentityCenterConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteLakeFormationOptIn
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteObjectsOnCancel
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeregisterResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeLakeFormationIdentityCenterConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeTransaction
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ExtendTransaction
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetDataCellsFilter
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetDataLakePrincipal
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetDataLakeSettings
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetEffectivePermissionsForPath
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetLFTag
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetLFTagExpression
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetQueryState
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetQueryStatistics
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetResourceLFTags
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetTableObjects
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetTemporaryDataLocationCredentials
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetTemporaryGluePartitionCredentials
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetTemporaryGlueTableCredentials
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetWorkUnitResults
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetWorkUnits
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GrantPermissions
- W[ ] S[x] M[x] F[ ] K[ ] C[ ] ListDataCellsFilter
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListLFTagExpressions
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListLFTags
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListLakeFormationOptIns
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListPermissions
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListResources
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListTableStorageOptimizers
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListTransactions
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] PutDataLakeSettings
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] RegisterResource
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] RemoveLFTagsFromResource
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] RevokePermissions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] SearchDatabasesByLFTags
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] SearchTablesByLFTags
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartQueryPlanning
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartTransaction
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateDataCellsFilter
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UpdateLFTag
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateLFTagExpression
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateLakeFormationIdentityCenterConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateTableObjects
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateTableStorageOptimizer

Integration tests: 17/19 implemented operations tested (89.5%)
Untested implemented operations: 2

### winterbaume-lambda (lambda) - W: 85/85, S: 0/85, M: 46/85, F: 0/85, K: 23/85, C: 85/85

Terraform E2E: 15 tests across 8 terraform resource types

Resource types: aws_lambda_alias, aws_lambda_code_signing_config, aws_lambda_event_source_mapping, aws_lambda_function, aws_lambda_function_url, aws_lambda_layer_version, aws_lambda_layer_version_permission, aws_lambda_permission

- W[x] S[ ] M[x] F[ ] K[ ] C[x] AddLayerVersionPermission
- W[x] S[ ] M[x] F[ ] K[x] C[x] AddPermission
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CheckpointDurableExecution
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateAlias
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateCapacityProvider
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateCodeSigningConfig
- W[x] S[ ] M[x] F[ ] K[x] C[x] CreateEventSourceMapping
- W[x] S[ ] M[x] F[ ] K[x] C[x] CreateFunction
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateFunctionUrlConfig
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteAlias
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteCapacityProvider
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteCodeSigningConfig
- W[x] S[ ] M[x] F[ ] K[x] C[x] DeleteEventSourceMapping
- W[x] S[ ] M[x] F[ ] K[x] C[x] DeleteFunction
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteFunctionCodeSigningConfig
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteFunctionConcurrency
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteFunctionEventInvokeConfig
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteFunctionUrlConfig
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteLayerVersion
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteProvisionedConcurrencyConfig
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetAccountSettings
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetAlias
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetCapacityProvider
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetCodeSigningConfig
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetDurableExecution
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetDurableExecutionHistory
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetDurableExecutionState
- W[x] S[ ] M[x] F[ ] K[x] C[x] GetEventSourceMapping
- W[x] S[ ] M[x] F[ ] K[x] C[x] GetFunction
- W[x] S[ ] M[x] F[ ] K[x] C[x] GetFunctionCodeSigningConfig
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetFunctionConcurrency
- W[x] S[ ] M[ ] F[ ] K[x] C[x] GetFunctionConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetFunctionEventInvokeConfig
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetFunctionRecursionConfig
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetFunctionScalingConfig
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetFunctionUrlConfig
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetLayerVersion
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetLayerVersionByArn
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetLayerVersionPolicy
- W[x] S[ ] M[x] F[ ] K[x] C[x] GetPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetProvisionedConcurrencyConfig
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetRuntimeManagementConfig
- W[x] S[ ] M[x] F[ ] K[x] C[x] Invoke
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] InvokeAsync
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] InvokeWithResponseStream
- W[x] S[ ] M[x] F[ ] K[x] C[x] ListAliases
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListCapacityProviders
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListCodeSigningConfigs
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListDurableExecutionsByFunction
- W[x] S[ ] M[x] F[ ] K[x] C[x] ListEventSourceMappings
- W[x] S[ ] M[x] F[ ] K[x] C[x] ListFunctionEventInvokeConfigs
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListFunctionUrlConfigs
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListFunctionVersionsByCapacityProvider
- W[x] S[ ] M[x] F[ ] K[x] C[x] ListFunctions
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListFunctionsByCodeSigningConfig
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListLayerVersions
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListLayers
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListProvisionedConcurrencyConfigs
- W[x] S[ ] M[x] F[ ] K[x] C[x] ListTags
- W[x] S[ ] M[x] F[ ] K[x] C[x] ListVersionsByFunction
- W[x] S[ ] M[x] F[ ] K[ ] C[x] PublishLayerVersion
- W[x] S[ ] M[x] F[ ] K[ ] C[x] PublishVersion
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] PutFunctionCodeSigningConfig
- W[x] S[ ] M[x] F[ ] K[ ] C[x] PutFunctionConcurrency
- W[x] S[ ] M[x] F[ ] K[ ] C[x] PutFunctionEventInvokeConfig
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] PutFunctionRecursionConfig
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] PutFunctionScalingConfig
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] PutProvisionedConcurrencyConfig
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] PutRuntimeManagementConfig
- W[x] S[ ] M[x] F[ ] K[ ] C[x] RemoveLayerVersionPermission
- W[x] S[ ] M[x] F[ ] K[x] C[x] RemovePermission
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] SendDurableExecutionCallbackFailure
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] SendDurableExecutionCallbackHeartbeat
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] SendDurableExecutionCallbackSuccess
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] StopDurableExecution
- W[x] S[ ] M[x] F[ ] K[x] C[x] TagResource
- W[x] S[ ] M[x] F[ ] K[x] C[x] UntagResource
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UpdateAlias
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateCapacityProvider
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateCodeSigningConfig
- W[x] S[ ] M[x] F[ ] K[x] C[x] UpdateEventSourceMapping
- W[x] S[ ] M[x] F[ ] K[x] C[x] UpdateFunctionCode
- W[x] S[ ] M[x] F[ ] K[x] C[x] UpdateFunctionConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UpdateFunctionEventInvokeConfig
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UpdateFunctionUrlConfig

Integration tests: 61/85 implemented operations tested (71.8%)
Untested implemented operations: 24

### winterbaume-lexmodelsv2 (lex-models-v2) - W: 58/107, S: 2/107, M: 17/107, F: 0/107, K: 0/107, C: 0/107

- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] BatchCreateCustomVocabularyItem
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] BatchDeleteCustomVocabularyItem
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] BatchUpdateCustomVocabularyItem
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] BuildBotLocale
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateBot
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateBotAlias
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateBotLocale
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateBotReplica
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateBotVersion
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateExport
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateIntent
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateResourcePolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateResourcePolicyStatement
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateSlot
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateSlotType
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateTestSetDiscrepancyReport
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateUploadUrl
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteBot
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteBotAlias
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteBotAnalyzerRecommendation
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteBotLocale
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteBotReplica
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteBotVersion
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteCustomVocabulary
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteExport
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteImport
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteIntent
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteResourcePolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteResourcePolicyStatement
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteSlot
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteSlotType
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteTestSet
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteUtterances
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeBot
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeBotAlias
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeBotAnalyzerRecommendation
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeBotLocale
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeBotRecommendation
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeBotReplica
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeBotResourceGeneration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeBotVersion
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeCustomVocabularyMetadata
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeExport
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeImport
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeIntent
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeResourcePolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeSlot
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeSlotType
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeTestExecution
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeTestSet
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeTestSetDiscrepancyReport
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeTestSetGeneration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GenerateBotElement
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetTestExecutionArtifactsUrl
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListAggregatedUtterances
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListBotAliasReplicas
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListBotAliases
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListBotAnalyzerHistory
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListBotLocales
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListBotRecommendations
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListBotReplicas
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListBotResourceGenerations
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListBotVersionReplicas
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListBotVersions
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListBots
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] ListBuiltInIntents
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] ListBuiltInSlotTypes
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListCustomVocabularyItems
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListExports
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListImports
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListIntentMetrics
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListIntentPaths
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListIntentStageMetrics
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListIntents
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListRecommendedIntents
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListSessionAnalyticsData
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListSessionMetrics
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListSlotTypes
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListSlots
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListTagsForResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListTestExecutionResultItems
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListTestExecutions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListTestSetRecords
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListTestSets
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListUtteranceAnalyticsData
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListUtteranceMetrics
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] SearchAssociatedTranscripts
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartBotAnalyzer
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartBotRecommendation
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartBotResourceGeneration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] StartImport
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartTestExecution
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartTestSetGeneration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StopBotAnalyzer
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StopBotRecommendation
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] TagResource
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UntagResource
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UpdateBot
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UpdateBotAlias
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateBotLocale
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateBotRecommendation
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateExport
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateIntent
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UpdateResourcePolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateSlot
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateSlotType
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateTestSet

Integration tests: 58/58 implemented operations tested (100.0%)

### winterbaume-macie2 (macie2) - W: 67/81, S: 14/81, M: 13/81, F: 0/81, K: 24/81, C: 0/81

- W[x] S[ ] M[x] F[ ] K[ ] C[ ] AcceptInvitation
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] BatchGetCustomDataIdentifiers
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] BatchUpdateAutomatedDiscoveryAccounts
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] CreateAllowList
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] CreateClassificationJob
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] CreateCustomDataIdentifier
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] CreateFindingsFilter
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateInvitations
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateMember
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateSampleFindings
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeclineInvitations
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] DeleteAllowList
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] DeleteCustomDataIdentifier
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] DeleteFindingsFilter
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteInvitations
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteMember
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] DescribeBuckets
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] DescribeClassificationJob
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeOrganizationConfiguration
- W[x] S[ ] M[x] F[ ] K[x] C[ ] DisableMacie
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DisableOrganizationAdminAccount
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DisassociateFromAdministratorAccount
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DisassociateFromMasterAccount
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DisassociateMember
- W[x] S[ ] M[x] F[ ] K[x] C[ ] EnableMacie
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] EnableOrganizationAdminAccount
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetAdministratorAccount
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] GetAllowList
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetAutomatedDiscoveryConfiguration
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] GetBucketStatistics
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetClassificationExportConfiguration
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] GetClassificationScope
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] GetCustomDataIdentifier
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetFindingStatistics
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] GetFindings
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] GetFindingsFilter
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetFindingsPublicationConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetInvitationsCount
- W[x] S[ ] M[x] F[ ] K[x] C[ ] GetMacieSession
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetMasterAccount
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetMember
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] GetResourceProfile
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetRevealConfiguration
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] GetSensitiveDataOccurrences
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] GetSensitiveDataOccurrencesAvailability
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetSensitivityInspectionTemplate
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetUsageStatistics
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetUsageTotals
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] ListAllowLists
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListAutomatedDiscoveryAccounts
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] ListClassificationJobs
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] ListClassificationScopes
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] ListCustomDataIdentifiers
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] ListFindings
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] ListFindingsFilters
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListInvitations
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListManagedDataIdentifiers
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListMembers
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListOrganizationAdminAccounts
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] ListResourceProfileArtifacts
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] ListResourceProfileDetections
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListSensitivityInspectionTemplates
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutClassificationExportConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutFindingsPublicationConfiguration
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] SearchResources
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] TagResource
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] TestCustomDataIdentifier
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UntagResource
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] UpdateAllowList
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateAutomatedDiscoveryConfiguration
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] UpdateClassificationJob
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] UpdateClassificationScope
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] UpdateFindingsFilter
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] UpdateMacieSession
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateMemberSession
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateOrganizationConfiguration
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] UpdateResourceProfile
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] UpdateResourceProfileDetections
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateRevealConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateSensitivityInspectionTemplate

Integration tests: 59/67 implemented operations tested (88.1%)
Untested implemented operations: 8

### winterbaume-managedblockchain (managedblockchain) - W: 27/27, S: 0/27, M: 20/27, F: 0/27, K: 0/27, C: 0/27

- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateAccessor
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateMember
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateNetwork
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateNode
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateProposal
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteAccessor
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteMember
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteNode
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetAccessor
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetMember
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetNetwork
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetNode
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetProposal
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListAccessors
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListInvitations
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListMembers
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListNetworks
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListNodes
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListProposalVotes
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListProposals
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListTagsForResource
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] RejectInvitation
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UntagResource
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UpdateMember
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UpdateNode
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] VoteOnProposal

Integration tests: 27/27 implemented operations tested (100.0%)

### winterbaume-marketplacemetering (marketplace-metering) - W: 4/4, S: 0/4, M: 0/4, F: 0/4, K: 0/4, C: 0/4

- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] BatchMeterUsage
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] MeterUsage
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] RegisterUsage
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ResolveCustomer

Integration tests: 4/4 implemented operations tested (100.0%)

### winterbaume-mediaconnect (mediaconnect) - W: 21/82, S: 0/82, M: 18/82, F: 0/82, K: 0/82, C: 0/82

- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AddBridgeOutputs
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AddBridgeSources
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AddFlowMediaStreams
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] AddFlowOutputs
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] AddFlowSources
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] AddFlowVpcInterfaces
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] BatchGetRouterInput
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] BatchGetRouterNetworkInterface
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] BatchGetRouterOutput
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateBridge
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateFlow
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateGateway
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateRouterInput
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateRouterNetworkInterface
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateRouterOutput
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteBridge
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteFlow
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteGateway
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteRouterInput
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteRouterNetworkInterface
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteRouterOutput
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeregisterGatewayInstance
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeBridge
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeFlow
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeFlowSourceMetadata
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeFlowSourceThumbnail
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeGateway
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeGatewayInstance
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeOffering
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeReservation
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetRouterInput
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetRouterInputSourceMetadata
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetRouterInputThumbnail
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetRouterNetworkInterface
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetRouterOutput
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GrantFlowEntitlements
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListBridges
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListEntitlements
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListFlows
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListGatewayInstances
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListGateways
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListOfferings
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListReservations
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListRouterInputs
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListRouterNetworkInterfaces
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListRouterOutputs
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListTagsForGlobalResource
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListTagsForResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] PurchaseOffering
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] RemoveBridgeOutput
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] RemoveBridgeSource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] RemoveFlowMediaStream
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] RemoveFlowOutput
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] RemoveFlowSource
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] RemoveFlowVpcInterface
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] RestartRouterInput
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] RestartRouterOutput
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] RevokeFlowEntitlement
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] StartFlow
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartRouterInput
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartRouterOutput
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] StopFlow
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StopRouterInput
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StopRouterOutput
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] TagGlobalResource
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] TagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] TakeRouterInput
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UntagGlobalResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UntagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateBridge
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateBridgeOutput
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateBridgeSource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateBridgeState
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateFlow
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UpdateFlowEntitlement
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateFlowMediaStream
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UpdateFlowOutput
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UpdateFlowSource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateGatewayInstance
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateRouterInput
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateRouterNetworkInterface
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateRouterOutput

Integration tests: 21/21 implemented operations tested (100.0%)

### winterbaume-medialive (medialive) - W: 16/123, S: 0/123, M: 12/123, F: 0/123, K: 0/123, C: 0/123

- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AcceptInputDeviceTransfer
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] BatchDelete
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] BatchStart
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] BatchStop
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] BatchUpdateSchedule
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CancelInputDeviceTransfer
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ClaimDevice
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateChannel
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateChannelPlacementGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateCloudWatchAlarmTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateCloudWatchAlarmTemplateGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateCluster
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateEventBridgeRuleTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateEventBridgeRuleTemplateGroup
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateInput
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateInputSecurityGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateMultiplex
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateMultiplexProgram
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateNetwork
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateNode
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateNodeRegistrationScript
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreatePartnerInput
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateSdiSource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateSignalMap
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateTags
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteChannel
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteChannelPlacementGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteCloudWatchAlarmTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteCloudWatchAlarmTemplateGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteCluster
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteEventBridgeRuleTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteEventBridgeRuleTemplateGroup
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteInput
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteInputSecurityGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteMultiplex
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteMultiplexProgram
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteNetwork
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteNode
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteReservation
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteSchedule
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteSdiSource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteSignalMap
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteTags
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeAccountConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeChannel
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeChannelPlacementGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeCluster
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeInput
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeInputDevice
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeInputDeviceThumbnail
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeInputSecurityGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeMultiplex
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeMultiplexProgram
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeNetwork
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeNode
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeOffering
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeReservation
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeSchedule
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeSdiSource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeThumbnails
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetCloudWatchAlarmTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetCloudWatchAlarmTemplateGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetEventBridgeRuleTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetEventBridgeRuleTemplateGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetSignalMap
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListAlerts
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListChannelPlacementGroups
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListChannels
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListCloudWatchAlarmTemplateGroups
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListCloudWatchAlarmTemplates
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListClusterAlerts
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListClusters
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListEventBridgeRuleTemplateGroups
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListEventBridgeRuleTemplates
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListInputDeviceTransfers
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListInputDevices
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListInputSecurityGroups
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListInputs
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListMultiplexAlerts
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListMultiplexPrograms
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListMultiplexes
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListNetworks
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListNodes
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListOfferings
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListReservations
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListSdiSources
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListSignalMaps
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListTagsForResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListVersions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] PurchaseOffering
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] RebootInputDevice
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] RejectInputDeviceTransfer
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] RestartChannelPipelines
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] StartChannel
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartDeleteMonitorDeployment
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartInputDevice
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartInputDeviceMaintenanceWindow
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartMonitorDeployment
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartMultiplex
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartUpdateSignalMap
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] StopChannel
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StopInputDevice
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StopMultiplex
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] TransferInputDevice
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateAccountConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UpdateChannel
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateChannelClass
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateChannelPlacementGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateCloudWatchAlarmTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateCloudWatchAlarmTemplateGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateCluster
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateEventBridgeRuleTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateEventBridgeRuleTemplateGroup
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UpdateInput
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateInputDevice
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateInputSecurityGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateMultiplex
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateMultiplexProgram
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateNetwork
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateNode
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateNodeState
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateReservation
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateSdiSource

Integration tests: 12/16 implemented operations tested (75.0%)
Untested implemented operations: 4

### winterbaume-mediapackage (mediapackage) - W: 9/19, S: 0/19, M: 9/19, F: 0/19, K: 0/19, C: 0/19

- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ConfigureLogs
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateChannel
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateHarvestJob
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateOriginEndpoint
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteChannel
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteOriginEndpoint
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeChannel
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeHarvestJob
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeOriginEndpoint
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListChannels
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListHarvestJobs
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListOriginEndpoints
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListTagsForResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] RotateChannelCredentials
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] RotateIngestEndpointCredentials
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] TagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UntagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateChannel
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UpdateOriginEndpoint

Integration tests: 9/9 implemented operations tested (100.0%)

### winterbaume-mediapackagev2 (mediapackagev2) - W: 7/30, S: 0/30, M: 7/30, F: 0/30, K: 0/30, C: 0/30

- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CancelHarvestJob
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateChannel
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateChannelGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateHarvestJob
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateOriginEndpoint
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteChannel
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteChannelGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteChannelPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteOriginEndpoint
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteOriginEndpointPolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetChannel
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetChannelGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetChannelPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetHarvestJob
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetOriginEndpoint
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetOriginEndpointPolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListChannelGroups
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListChannels
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListHarvestJobs
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListOriginEndpoints
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListTagsForResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] PutChannelPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] PutOriginEndpointPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ResetChannelState
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ResetOriginEndpointState
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] TagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UntagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateChannel
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateChannelGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateOriginEndpoint

Integration tests: 7/7 implemented operations tested (100.0%)

### winterbaume-mediastore (mediastore) - W: 11/21, S: 0/21, M: 11/21, F: 0/21, K: 0/21, C: 0/21

- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateContainer
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteContainer
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteContainerPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteCorsPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteLifecyclePolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteMetricPolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeContainer
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetContainerPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetCorsPolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetLifecyclePolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetMetricPolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListContainers
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListTagsForResource
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] PutContainerPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] PutCorsPolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] PutLifecyclePolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] PutMetricPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartAccessLogging
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StopAccessLogging
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] TagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UntagResource

Integration tests: 11/11 implemented operations tested (100.0%)

### winterbaume-mediastoredata (mediastore-data) - W: 5/5, S: 0/5, M: 4/5, F: 0/5, K: 0/5, C: 0/5

- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteObject
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeObject
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetObject
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListItems
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] PutObject

Integration tests: 5/5 implemented operations tested (100.0%)

### winterbaume-memorydb (memorydb) - W: 13/45, S: 0/45, M: 13/45, F: 0/45, K: 10/45, C: 45/45

- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] BatchUpdateCluster
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] CopySnapshot
- W[ ] S[ ] M[ ] F[ ] K[x] C[x] CreateACL
- W[x] S[ ] M[x] F[ ] K[x] C[x] CreateCluster
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] CreateMultiRegionCluster
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] CreateParameterGroup
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateSnapshot
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateSubnetGroup
- W[ ] S[ ] M[ ] F[ ] K[x] C[x] CreateUser
- W[ ] S[ ] M[ ] F[ ] K[x] C[x] DeleteACL
- W[x] S[ ] M[x] F[ ] K[x] C[x] DeleteCluster
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DeleteMultiRegionCluster
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DeleteParameterGroup
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteSnapshot
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteSubnetGroup
- W[ ] S[ ] M[ ] F[ ] K[x] C[x] DeleteUser
- W[ ] S[ ] M[ ] F[ ] K[x] C[x] DescribeACLs
- W[x] S[ ] M[x] F[ ] K[x] C[x] DescribeClusters
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DescribeEngineVersions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DescribeEvents
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DescribeMultiRegionClusters
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DescribeMultiRegionParameterGroups
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DescribeMultiRegionParameters
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DescribeParameterGroups
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DescribeParameters
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DescribeReservedNodes
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DescribeReservedNodesOfferings
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DescribeServiceUpdates
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeSnapshots
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeSubnetGroups
- W[ ] S[ ] M[ ] F[ ] K[x] C[x] DescribeUsers
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] FailoverShard
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ListAllowedMultiRegionClusterUpdates
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ListAllowedNodeTypeUpdates
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListTags
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] PurchaseReservedNodesOffering
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ResetParameterGroup
- W[x] S[ ] M[x] F[ ] K[ ] C[x] TagResource
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UntagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] UpdateACL
- W[x] S[ ] M[x] F[ ] K[x] C[x] UpdateCluster
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] UpdateMultiRegionCluster
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] UpdateParameterGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] UpdateSubnetGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] UpdateUser

Integration tests: 13/13 implemented operations tested (100.0%)

### winterbaume-mq (mq) - W: 23/24, S: 1/24, M: 19/24, F: 0/24, K: 6/24, C: 0/24

- W[x] S[ ] M[x] F[ ] K[x] C[ ] CreateBroker
- W[x] S[ ] M[x] F[ ] K[x] C[ ] CreateConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateTags
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateUser
- W[x] S[ ] M[x] F[ ] K[x] C[ ] DeleteBroker
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteTags
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteUser
- W[x] S[ ] M[x] F[ ] K[x] C[ ] DescribeBroker
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeBrokerEngineTypes
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] DescribeBrokerInstanceOptions
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeConfigurationRevision
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeUser
- W[x] S[ ] M[x] F[ ] K[x] C[ ] ListBrokers
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListConfigurationRevisions
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListConfigurations
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListTags
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListUsers
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] Promote
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] RebootBroker
- W[x] S[ ] M[x] F[ ] K[x] C[ ] UpdateBroker
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UpdateConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UpdateUser

Integration tests: 21/23 implemented operations tested (91.3%)
Untested implemented operations: 2

### winterbaume-neptune (neptune) - W: 64/70, S: 6/70, M: 47/70, F: 0/70, K: 6/70, C: 0/70

- W[x] S[ ] M[x] F[ ] K[ ] C[ ] AddRoleToDBCluster
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] AddSourceIdentifierToSubscription
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] AddTagsToResource
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] ApplyPendingMaintenanceAction
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CopyDBClusterParameterGroup
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CopyDBClusterSnapshot
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CopyDBParameterGroup
- W[x] S[ ] M[x] F[ ] K[x] C[ ] CreateDBCluster
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateDBClusterEndpoint
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateDBClusterParameterGroup
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateDBClusterSnapshot
- W[x] S[ ] M[x] F[ ] K[x] C[ ] CreateDBInstance
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateDBParameterGroup
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateDBSubnetGroup
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateEventSubscription
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateGlobalCluster
- W[x] S[ ] M[x] F[ ] K[x] C[ ] DeleteDBCluster
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteDBClusterEndpoint
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteDBClusterParameterGroup
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteDBClusterSnapshot
- W[x] S[ ] M[x] F[ ] K[x] C[ ] DeleteDBInstance
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteDBParameterGroup
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteDBSubnetGroup
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteEventSubscription
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteGlobalCluster
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeDBClusterEndpoints
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeDBClusterParameterGroups
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeDBClusterParameters
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeDBClusterSnapshotAttributes
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeDBClusterSnapshots
- W[x] S[ ] M[x] F[ ] K[x] C[ ] DescribeDBClusters
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeDBEngineVersions
- W[x] S[ ] M[x] F[ ] K[x] C[ ] DescribeDBInstances
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeDBParameterGroups
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeDBParameters
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeDBSubnetGroups
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] DescribeEngineDefaultClusterParameters
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] DescribeEngineDefaultParameters
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] DescribeEventCategories
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeEventSubscriptions
- W[ ] S[x] M[x] F[ ] K[ ] C[ ] DescribeEvents
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeGlobalClusters
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeOrderableDBInstanceOptions
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] DescribePendingMaintenanceActions
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeValidDBInstanceModifications
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] FailoverDBCluster
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] FailoverGlobalCluster
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListTagsForResource
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ModifyDBCluster
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ModifyDBClusterEndpoint
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ModifyDBClusterParameterGroup
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ModifyDBClusterSnapshotAttribute
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ModifyDBInstance
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ModifyDBParameterGroup
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ModifyDBSubnetGroup
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ModifyEventSubscription
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ModifyGlobalCluster
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] PromoteReadReplicaDBCluster
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] RebootDBInstance
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] RemoveFromGlobalCluster
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] RemoveRoleFromDBCluster
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] RemoveSourceIdentifierFromSubscription
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] RemoveTagsFromResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ResetDBClusterParameterGroup
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ResetDBParameterGroup
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] RestoreDBClusterFromSnapshot
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] RestoreDBClusterToPointInTime
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] StartDBCluster
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] StopDBCluster
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] SwitchoverGlobalCluster

Integration tests: 24/64 implemented operations tested (37.5%)
Untested implemented operations: 40

### winterbaume-networkfirewall (network-firewall) - W: 79/79, S: 0/79, M: 5/79, F: 0/79, K: 0/79, C: 0/79

- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] AcceptNetworkFirewallTransitGatewayAttachment
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] AssociateAvailabilityZones
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] AssociateFirewallPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] AssociateSubnets
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] AttachRuleGroupsToProxyConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateFirewall
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateFirewallPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateProxy
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateProxyConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateProxyRuleGroup
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateProxyRules
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateRuleGroup
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateTLSInspectionConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateVpcEndpointAssociation
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteFirewall
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteFirewallPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteNetworkFirewallTransitGatewayAttachment
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteProxy
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteProxyConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteProxyRuleGroup
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteProxyRules
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteResourcePolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteRuleGroup
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteTLSInspectionConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteVpcEndpointAssociation
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeFirewall
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeFirewallMetadata
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeFirewallPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeFlowOperation
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeLoggingConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeProxy
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeProxyConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeProxyRule
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeProxyRuleGroup
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeResourcePolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeRuleGroup
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeRuleGroupMetadata
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeRuleGroupSummary
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeTLSInspectionConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeVpcEndpointAssociation
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DetachRuleGroupsFromProxyConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DisassociateAvailabilityZones
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DisassociateSubnets
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetAnalysisReportResults
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListAnalysisReports
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListFirewallPolicies
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListFirewalls
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListFlowOperationResults
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListFlowOperations
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListProxies
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListProxyConfigurations
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListProxyRuleGroups
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListRuleGroups
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListTLSInspectionConfigurations
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListVpcEndpointAssociations
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutResourcePolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] RejectNetworkFirewallTransitGatewayAttachment
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] StartAnalysisReport
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] StartFlowCapture
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] StartFlowFlush
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateAvailabilityZoneChangeProtection
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateFirewallAnalysisSettings
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateFirewallDeleteProtection
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateFirewallDescription
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateFirewallEncryptionConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateFirewallPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateFirewallPolicyChangeProtection
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UpdateLoggingConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateProxy
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateProxyConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateProxyRule
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateProxyRuleGroupPriorities
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateProxyRulePriorities
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateRuleGroup
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateSubnetChangeProtection
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateTLSInspectionConfiguration

Integration tests: 42/79 implemented operations tested (53.2%)
Untested implemented operations: 37

### winterbaume-networkmanager (networkmanager) - W: 53/95, S: 0/95, M: 18/95, F: 0/95, K: 0/95, C: 0/95

Terraform E2E: 8 tests across 6 terraform resource types

Resource types: aws_networkmanager_connection, aws_networkmanager_core_network, aws_networkmanager_device, aws_networkmanager_global_network, aws_networkmanager_link, aws_networkmanager_site

- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] AcceptAttachment
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] AssociateConnectPeer
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] AssociateCustomerGateway
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] AssociateLink
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] AssociateTransitGatewayConnectPeer
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateConnectAttachment
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateConnectPeer
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateConnection
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateCoreNetwork
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateCoreNetworkPrefixListAssociation
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateDevice
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateDirectConnectGatewayAttachment
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateGlobalNetwork
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateLink
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateSite
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateSiteToSiteVpnAttachment
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateTransitGatewayPeering
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateTransitGatewayRouteTableAttachment
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateVpcAttachment
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteAttachment
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteConnectPeer
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteConnection
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteCoreNetwork
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteCoreNetworkPolicyVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteCoreNetworkPrefixListAssociation
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteDevice
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteGlobalNetwork
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteLink
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeletePeering
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteResourcePolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteSite
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeregisterTransitGateway
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeGlobalNetworks
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DisassociateConnectPeer
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DisassociateCustomerGateway
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DisassociateLink
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DisassociateTransitGatewayConnectPeer
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ExecuteCoreNetworkChangeSet
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetConnectAttachment
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetConnectPeer
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetConnectPeerAssociations
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetConnections
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetCoreNetwork
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetCoreNetworkChangeEvents
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetCoreNetworkChangeSet
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetCoreNetworkPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetCustomerGatewayAssociations
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetDevices
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetDirectConnectGatewayAttachment
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetLinkAssociations
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetLinks
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetNetworkResourceCounts
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetNetworkResourceRelationships
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetNetworkResources
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetNetworkRoutes
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetNetworkTelemetry
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetResourcePolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetRouteAnalysis
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetSiteToSiteVpnAttachment
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetSites
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetTransitGatewayConnectPeerAssociations
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetTransitGatewayPeering
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetTransitGatewayRegistrations
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetTransitGatewayRouteTableAttachment
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetVpcAttachment
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListAttachmentRoutingPolicyAssociations
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListAttachments
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListConnectPeers
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListCoreNetworkPolicyVersions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListCoreNetworkPrefixListAssociations
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListCoreNetworkRoutingInformation
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListCoreNetworks
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListOrganizationServiceAccessStatus
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListPeerings
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListTagsForResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] PutAttachmentRoutingPolicyLabel
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] PutCoreNetworkPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] PutResourcePolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] RegisterTransitGateway
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] RejectAttachment
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] RemoveAttachmentRoutingPolicyLabel
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] RestoreCoreNetworkPolicyVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartOrganizationServiceAccessUpdate
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] StartRouteAnalysis
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] TagResource
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateConnection
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateCoreNetwork
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateDevice
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateDirectConnectGatewayAttachment
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateGlobalNetwork
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateLink
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateNetworkResourceMetadata
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateSite
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateVpcAttachment

Integration tests: 44/53 implemented operations tested (83.0%)
Untested implemented operations: 9

### winterbaume-opensearch (opensearch) - W: 44/88, S: 0/88, M: 11/88, F: 0/88, K: 0/88, C: 0/88

- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] AcceptInboundConnection
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] AddDataSource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] AddDirectQueryDataSource
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] AddTags
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] AssociatePackage
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AssociatePackages
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AuthorizeVpcEndpointAccess
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CancelDomainConfigChange
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CancelServiceSoftwareUpdate
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateApplication
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateDomain
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateIndex
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateOutboundConnection
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreatePackage
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateVpcEndpoint
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteApplication
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteDataSource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteDirectQueryDataSource
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteDomain
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteInboundConnection
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteIndex
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteOutboundConnection
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeletePackage
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteVpcEndpoint
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeregisterCapability
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeDomain
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeDomainAutoTunes
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeDomainChangeProgress
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeDomainConfig
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeDomainHealth
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeDomainNodes
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeDomains
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeDryRunProgress
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeInboundConnections
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeInsightDetails
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeInstanceTypeLimits
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeOutboundConnections
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribePackages
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeReservedInstanceOfferings
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeReservedInstances
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeVpcEndpoints
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DissociatePackage
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DissociatePackages
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetApplication
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetCapability
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetCompatibleVersions
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetDataSource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetDefaultApplicationSetting
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetDirectQueryDataSource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetDomainMaintenanceStatus
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetIndex
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetPackageVersionHistory
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetUpgradeHistory
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetUpgradeStatus
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListApplications
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListDataSources
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListDirectQueryDataSources
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListDomainMaintenances
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListDomainNames
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListDomainsForPackage
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListInsights
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListInstanceTypeDetails
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListPackagesForDomain
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListScheduledActions
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListTags
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListVersions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListVpcEndpointAccess
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListVpcEndpoints
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListVpcEndpointsForDomain
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PurchaseReservedInstanceOffering
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] PutDefaultApplicationSetting
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] RegisterCapability
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] RejectInboundConnection
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] RemoveTags
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] RevokeVpcEndpointAccess
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] RollbackServiceSoftwareUpdate
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartDomainMaintenance
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartServiceSoftwareUpdate
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateApplication
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateDataSource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateDirectQueryDataSource
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UpdateDomainConfig
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateIndex
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdatePackage
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdatePackageScope
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateScheduledAction
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateVpcEndpoint
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpgradeDomain

Integration tests: 35/44 implemented operations tested (79.5%)
Untested implemented operations: 9

### winterbaume-opensearchserverless (opensearchserverless) - W: 12/46, S: 0/46, M: 12/46, F: 0/46, K: 0/46, C: 0/46

- W[x] S[ ] M[x] F[ ] K[ ] C[ ] BatchGetCollection
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] BatchGetCollectionGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] BatchGetEffectiveLifecyclePolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] BatchGetLifecyclePolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] BatchGetVpcEndpoint
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateAccessPolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateCollection
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateCollectionGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateIndex
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateLifecyclePolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateSecurityConfig
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateSecurityPolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateVpcEndpoint
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteAccessPolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteCollection
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteCollectionGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteIndex
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteLifecyclePolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteSecurityConfig
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteSecurityPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteVpcEndpoint
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetAccessPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetAccountSettings
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetIndex
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetPoliciesStats
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetSecurityConfig
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetSecurityPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListAccessPolicies
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListCollectionGroups
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListCollections
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListLifecyclePolicies
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListSecurityConfigs
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListSecurityPolicies
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListTagsForResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListVpcEndpoints
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] TagResource
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UntagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateAccessPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateAccountSettings
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateCollection
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateCollectionGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateIndex
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateLifecyclePolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateSecurityConfig
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UpdateSecurityPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateVpcEndpoint

Integration tests: 12/12 implemented operations tested (100.0%)

### winterbaume-organizations (organizations) - W: 60/63, S: 3/63, M: 41/63, F: 0/63, K: 11/63, C: 63/63

Terraform E2E: 4 tests across 4 terraform resource types

Resource types: aws_organizations_organization, aws_organizations_organizational_unit, aws_organizations_policy, aws_organizations_policy_attachment

- W[x] S[ ] M[ ] F[ ] K[ ] C[x] AcceptHandshake
- W[x] S[ ] M[x] F[ ] K[x] C[x] AttachPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CancelHandshake
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CloseAccount
- W[x] S[ ] M[x] F[ ] K[x] C[x] CreateAccount
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateGovCloudAccount
- W[x] S[ ] M[x] F[ ] K[x] C[x] CreateOrganization
- W[x] S[ ] M[x] F[ ] K[x] C[x] CreateOrganizationalUnit
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreatePolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeclineHandshake
- W[x] S[ ] M[x] F[ ] K[x] C[x] DeleteOrganization
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteOrganizationalUnit
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeletePolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteResourcePolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeregisterDelegatedAdministrator
- W[x] S[ ] M[x] F[ ] K[x] C[x] DescribeAccount
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeCreateAccountStatus
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] DescribeEffectivePolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeHandshake
- W[x] S[ ] M[x] F[ ] K[x] C[x] DescribeOrganization
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeOrganizationalUnit
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribePolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeResourcePolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeResponsibilityTransfer
- W[x] S[ ] M[x] F[ ] K[x] C[x] DetachPolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DisableAWSServiceAccess
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DisablePolicyType
- W[x] S[ ] M[x] F[ ] K[ ] C[x] EnableAWSServiceAccess
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] EnableAllFeatures
- W[x] S[ ] M[x] F[ ] K[ ] C[x] EnablePolicyType
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] InviteAccountToOrganization
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] InviteOrganizationToTransferResponsibility
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] LeaveOrganization
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListAWSServiceAccessForOrganization
- W[x] S[ ] M[x] F[ ] K[x] C[x] ListAccounts
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListAccountsForParent
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] ListAccountsWithInvalidEffectivePolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListChildren
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListCreateAccountStatus
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListDelegatedAdministrators
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListDelegatedServicesForAccount
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] ListEffectivePolicyValidationErrors
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListHandshakesForAccount
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListHandshakesForOrganization
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListInboundResponsibilityTransfers
- W[x] S[ ] M[x] F[ ] K[x] C[x] ListOrganizationalUnitsForParent
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListOutboundResponsibilityTransfers
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListParents
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListPolicies
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListPoliciesForTarget
- W[x] S[ ] M[x] F[ ] K[x] C[x] ListRoots
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListTagsForResource
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListTargetsForPolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[x] MoveAccount
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] PutResourcePolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[x] RegisterDelegatedAdministrator
- W[x] S[ ] M[x] F[ ] K[ ] C[x] RemoveAccountFromOrganization
- W[x] S[ ] M[x] F[ ] K[ ] C[x] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] TerminateResponsibilityTransfer
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UntagResource
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UpdateOrganizationalUnit
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UpdatePolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateResponsibilityTransfer

Integration tests: 54/60 implemented operations tested (90.0%)
Untested implemented operations: 6

### winterbaume-osis (osis) - W: 10/22, S: 0/22, M: 13/22, F: 0/22, K: 0/22, C: 0/22

- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreatePipeline
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreatePipelineEndpoint
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeletePipeline
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeletePipelineEndpoint
- W[ ] S[ ] M[x] F[ ] K[ ] C[ ] DeleteResourcePolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetPipeline
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetPipelineBlueprint
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetPipelineChangeProgress
- W[ ] S[ ] M[x] F[ ] K[ ] C[ ] GetResourcePolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListPipelineBlueprints
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListPipelineEndpointConnections
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListPipelineEndpoints
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListPipelines
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListTagsForResource
- W[ ] S[ ] M[x] F[ ] K[ ] C[ ] PutResourcePolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] RevokePipelineEndpointConnections
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] StartPipeline
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] StopPipeline
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] TagResource
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UntagResource
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UpdatePipeline
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ValidatePipeline

Integration tests: 10/10 implemented operations tested (100.0%)

### winterbaume-outposts (outposts) - W: 13/37, S: 0/37, M: 0/37, F: 0/37, K: 0/37, C: 0/37

- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CancelCapacityTask
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CancelOrder
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateOrder
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateOutpost
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateRenewal
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateSite
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteOutpost
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteSite
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetCapacityTask
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetCatalogItem
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetConnection
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetOrder
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetOutpost
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetOutpostBillingInformation
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetOutpostInstanceTypes
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetOutpostSupportedInstanceTypes
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetRenewalPricing
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetSite
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetSiteAddress
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListAssetInstances
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListAssets
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListBlockingInstancesForCapacityTask
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListCapacityTasks
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListCatalogItems
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListOrders
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListOutposts
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListSites
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListTagsForResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartCapacityTask
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartConnection
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartOutpostDecommission
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateOutpost
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateSite
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateSiteAddress
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateSiteRackPhysicalProperties

Integration tests: 13/13 implemented operations tested (100.0%)

### winterbaume-panorama (panorama) - W: 10/34, S: 1/34, M: 0/34, F: 0/34, K: 0/34, C: 0/34

- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateApplicationInstance
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateJobForDevices
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateNodeFromTemplateJob
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreatePackage
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreatePackageImportJob
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteDevice
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeletePackage
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeregisterPackageVersion
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeApplicationInstance
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeApplicationInstanceDetails
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeDevice
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeDeviceJob
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeNode
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeNodeFromTemplateJob
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribePackage
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribePackageImportJob
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribePackageVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListApplicationInstanceDependencies
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListApplicationInstanceNodeInstances
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListApplicationInstances
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListDevices
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListDevicesJobs
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListNodeFromTemplateJobs
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] ListNodes
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListPackageImportJobs
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListPackages
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ProvisionDevice
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] RegisterPackageVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] RemoveApplicationInstance
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] SignalApplicationInstanceNodeInstances
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] TagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateDeviceMetadata

Integration tests: 10/10 implemented operations tested (100.0%)

### winterbaume-pcaconnectorscep (pca-connector-scep) - W: 11/12, S: 0/12, M: 0/12, F: 0/12, K: 0/12, C: 0/12

- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateChallenge
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateConnector
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteChallenge
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteConnector
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetChallengeMetadata
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetChallengePassword
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetConnector
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListChallengeMetadata
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListConnectors
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UntagResource

Integration tests: 9/11 implemented operations tested (81.8%)
Untested implemented operations: 2

### winterbaume-personalize (personalize) - W: 66/71, S: 5/71, M: 4/71, F: 0/71, K: 0/71, C: 0/71

- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateBatchInferenceJob
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateBatchSegmentJob
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateCampaign
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateDataDeletionJob
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateDataset
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateDatasetExportJob
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateDatasetGroup
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateDatasetImportJob
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateEventTracker
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateFilter
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateMetricAttribution
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateRecommender
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateSchema
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateSolution
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateSolutionVersion
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteCampaign
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteDataset
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteDatasetGroup
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteEventTracker
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteFilter
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteMetricAttribution
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteRecommender
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteSchema
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteSolution
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] DescribeAlgorithm
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeBatchInferenceJob
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeBatchSegmentJob
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeCampaign
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeDataDeletionJob
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeDataset
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeDatasetExportJob
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeDatasetGroup
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeDatasetImportJob
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeEventTracker
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] DescribeFeatureTransformation
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeFilter
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeMetricAttribution
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] DescribeRecipe
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeRecommender
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeSchema
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeSolution
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeSolutionVersion
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] GetSolutionMetrics
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListBatchInferenceJobs
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListBatchSegmentJobs
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListCampaigns
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListDataDeletionJobs
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListDatasetExportJobs
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListDatasetGroups
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListDatasetImportJobs
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListDatasets
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListEventTrackers
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListFilters
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListMetricAttributionMetrics
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListMetricAttributions
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] ListRecipes
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListRecommenders
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListSchemas
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListSolutionVersions
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListSolutions
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] StartRecommender
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] StopRecommender
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] StopSolutionVersionCreation
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateCampaign
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateDataset
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateMetricAttribution
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateRecommender
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateSolution

Integration tests: 8/66 implemented operations tested (12.1%)
Untested implemented operations: 58

### winterbaume-personalizeevents (personalize-events) - W: 5/5, S: 0/5, M: 0/5, F: 0/5, K: 0/5, C: 0/5

- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutActionInteractions
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutActions
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutEvents
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutItems
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutUsers

Integration tests: 3/5 implemented operations tested (60.0%)
Untested implemented operations: 2

### winterbaume-personalizeruntime (personalize-runtime) - W: 3/3, S: 0/3, M: 0/3, F: 0/3, K: 0/3, C: 0/3

- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetActionRecommendations
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetPersonalizedRanking
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetRecommendations

Integration tests: 3/3 implemented operations tested (100.0%)

### winterbaume-pi (pi) - W: 9/13, S: 4/13, M: 0/13, F: 0/13, K: 0/13, C: 0/13

- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreatePerformanceAnalysisReport
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeletePerformanceAnalysisReport
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] DescribeDimensionKeys
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] GetDimensionKeyDetails
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetPerformanceAnalysisReport
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] GetResourceMetadata
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] GetResourceMetrics
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListAvailableResourceDimensions
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListAvailableResourceMetrics
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListPerformanceAnalysisReports
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UntagResource

Integration tests: 7/9 implemented operations tested (77.8%)
Untested implemented operations: 2

### winterbaume-pinpoint (pinpoint) - W: 15/122, S: 0/122, M: 12/122, F: 0/122, K: 0/122, C: 0/122

- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateApp
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateCampaign
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateEmailTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateExportJob
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateImportJob
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateInAppTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateJourney
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreatePushTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateRecommenderConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateSegment
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateSmsTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateVoiceTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteAdmChannel
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteApnsChannel
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteApnsSandboxChannel
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteApnsVoipChannel
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteApnsVoipSandboxChannel
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteApp
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteBaiduChannel
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteCampaign
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteEmailChannel
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteEmailTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteEndpoint
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteEventStream
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteGcmChannel
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteInAppTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteJourney
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeletePushTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteRecommenderConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteSegment
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteSmsChannel
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteSmsTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteUserEndpoints
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteVoiceChannel
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteVoiceTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetAdmChannel
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetApnsChannel
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetApnsSandboxChannel
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetApnsVoipChannel
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetApnsVoipSandboxChannel
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetApp
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetApplicationDateRangeKpi
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetApplicationSettings
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetApps
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetBaiduChannel
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetCampaign
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetCampaignActivities
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetCampaignDateRangeKpi
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetCampaignVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetCampaignVersions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetCampaigns
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetChannels
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetEmailChannel
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetEmailTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetEndpoint
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetEventStream
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetExportJob
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetExportJobs
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetGcmChannel
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetImportJob
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetImportJobs
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetInAppMessages
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetInAppTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetJourney
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetJourneyDateRangeKpi
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetJourneyExecutionActivityMetrics
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetJourneyExecutionMetrics
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetJourneyRunExecutionActivityMetrics
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetJourneyRunExecutionMetrics
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetJourneyRuns
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetPushTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetRecommenderConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetRecommenderConfigurations
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetSegment
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetSegmentExportJobs
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetSegmentImportJobs
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetSegmentVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetSegmentVersions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetSegments
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetSmsChannel
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetSmsTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetUserEndpoints
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetVoiceChannel
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetVoiceTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListJourneys
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListTagsForResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListTemplateVersions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListTemplates
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] PhoneNumberValidate
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] PutEventStream
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] PutEvents
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] RemoveAttributes
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] SendMessages
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] SendOTPMessage
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] SendUsersMessages
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] TagResource
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UntagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateAdmChannel
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateApnsChannel
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateApnsSandboxChannel
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateApnsVoipChannel
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateApnsVoipSandboxChannel
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UpdateApplicationSettings
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateBaiduChannel
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateCampaign
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateEmailChannel
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateEmailTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateEndpoint
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateEndpointsBatch
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateGcmChannel
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateInAppTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateJourney
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateJourneyState
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdatePushTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateRecommenderConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateSegment
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateSmsChannel
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateSmsTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateTemplateActiveVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateVoiceChannel
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateVoiceTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] VerifyOTPMessage

Integration tests: 12/15 implemented operations tested (80.0%)
Untested implemented operations: 3

### winterbaume-pinpointsmsvoice (pinpoint-sms-voice) - W: 4/8, S: 0/8, M: 0/8, F: 0/8, K: 0/8, C: 0/8

- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateConfigurationSet
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateConfigurationSetEventDestination
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteConfigurationSet
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteConfigurationSetEventDestination
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetConfigurationSetEventDestinations
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListConfigurationSets
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] SendVoiceMessage
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateConfigurationSetEventDestination

Integration tests: 4/4 implemented operations tested (100.0%)

### winterbaume-pipes (pipes) - W: 10/10, S: 0/10, M: 9/10, F: 0/10, K: 10/10, C: 10/10

- W[x] S[ ] M[x] F[ ] K[x] C[x] CreatePipe
- W[x] S[ ] M[x] F[ ] K[x] C[x] DeletePipe
- W[x] S[ ] M[x] F[ ] K[x] C[x] DescribePipe
- W[x] S[ ] M[x] F[ ] K[x] C[x] ListPipes
- W[x] S[ ] M[x] F[ ] K[x] C[x] ListTagsForResource
- W[x] S[ ] M[x] F[ ] K[x] C[x] StartPipe
- W[x] S[ ] M[x] F[ ] K[x] C[x] StopPipe
- W[x] S[ ] M[x] F[ ] K[x] C[x] TagResource
- W[x] S[ ] M[x] F[ ] K[x] C[x] UntagResource
- W[x] S[ ] M[ ] F[ ] K[x] C[x] UpdatePipe

Integration tests: 7/10 implemented operations tested (70.0%)
Untested implemented operations: 3

### winterbaume-polly (polly) - W: 9/10, S: 0/10, M: 5/10, F: 0/10, K: 0/10, C: 0/10

- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteLexicon
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeVoices
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetLexicon
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetSpeechSynthesisTask
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListLexicons
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListSpeechSynthesisTasks
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] PutLexicon
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartSpeechSynthesisStream
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] StartSpeechSynthesisTask
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] SynthesizeSpeech

Integration tests: 9/9 implemented operations tested (100.0%)

### winterbaume-pricing (pricing) - W: 5/5, S: 0/5, M: 0/5, F: 0/5, K: 0/5, C: 0/5

- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeServices
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetAttributeValues
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetPriceListFileUrl
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetProducts
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListPriceLists

Integration tests: 5/5 implemented operations tested (100.0%)

### winterbaume-quicksight (quicksight) - W: 68/232, S: 0/232, M: 31/232, F: 0/232, K: 0/232, C: 0/232

- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] BatchCreateTopicReviewedAnswer
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] BatchDeleteTopicReviewedAnswer
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CancelIngestion
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateAccountCustomization
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateAccountSubscription
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateActionConnector
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateAnalysis
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateBrand
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateCustomPermissions
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateDashboard
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateDataSet
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateDataSource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateFolder
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateFolderMembership
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateGroup
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateGroupMembership
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateIAMPolicyAssignment
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateIngestion
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateNamespace
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateRefreshSchedule
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateRoleMembership
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateTemplateAlias
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateTheme
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateThemeAlias
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateTopic
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateTopicRefreshSchedule
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateVPCConnection
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteAccountCustomPermission
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteAccountCustomization
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteAccountSubscription
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteActionConnector
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteAnalysis
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteBrand
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteBrandAssignment
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteCustomPermissions
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteDashboard
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteDataSet
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteDataSetRefreshProperties
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteDataSource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteDefaultQBusinessApplication
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteFolder
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteFolderMembership
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteGroup
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteGroupMembership
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteIAMPolicyAssignment
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteIdentityPropagationConfig
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteNamespace
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteRefreshSchedule
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteRoleCustomPermission
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteRoleMembership
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteTemplateAlias
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteTheme
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteThemeAlias
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteTopic
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteTopicRefreshSchedule
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteUser
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteUserByPrincipalId
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteUserCustomPermission
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteVPCConnection
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeAccountCustomPermission
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeAccountCustomization
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeAccountSettings
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeAccountSubscription
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeActionConnector
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeActionConnectorPermissions
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeAnalysis
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeAnalysisDefinition
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeAnalysisPermissions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeAssetBundleExportJob
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeAssetBundleImportJob
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeAutomationJob
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeBrand
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeBrandAssignment
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeBrandPublishedVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeCustomPermissions
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeDashboard
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeDashboardDefinition
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeDashboardPermissions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeDashboardSnapshotJob
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeDashboardSnapshotJobResult
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeDashboardsQAConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeDataSet
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeDataSetPermissions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeDataSetRefreshProperties
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeDataSource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeDataSourcePermissions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeDefaultQBusinessApplication
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeFolder
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeFolderPermissions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeFolderResolvedPermissions
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeGroup
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeGroupMembership
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeIAMPolicyAssignment
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeIngestion
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeIpRestriction
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeKeyRegistration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeNamespace
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeQPersonalizationConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeQuickSightQSearchConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeRefreshSchedule
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeRoleCustomPermission
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeSelfUpgradeConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeTemplateAlias
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeTemplateDefinition
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeTemplatePermissions
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeTheme
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeThemeAlias
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeThemePermissions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeTopic
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeTopicPermissions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeTopicRefresh
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeTopicRefreshSchedule
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeUser
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeVPCConnection
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GenerateEmbedUrlForAnonymousUser
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GenerateEmbedUrlForRegisteredUser
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GenerateEmbedUrlForRegisteredUserWithIdentity
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetDashboardEmbedUrl
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetFlowMetadata
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetFlowPermissions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetIdentityContext
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetSessionEmbedUrl
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListActionConnectors
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListAnalyses
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListAssetBundleExportJobs
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListAssetBundleImportJobs
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListBrands
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListCustomPermissions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListDashboardVersions
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListDashboards
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListDataSets
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListDataSources
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListFlows
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListFolderMembers
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListFolders
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListFoldersForResource
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListGroupMemberships
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListGroups
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListIAMPolicyAssignments
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListIAMPolicyAssignmentsForUser
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListIdentityPropagationConfigs
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListIngestions
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListNamespaces
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListRefreshSchedules
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListRoleMemberships
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListSelfUpgrades
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListTagsForResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListTemplateAliases
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListTemplateVersions
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListTemplates
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListThemeAliases
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListThemeVersions
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListThemes
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListTopicRefreshSchedules
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListTopicReviewedAnswers
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListTopics
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListUserGroups
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListUsers
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListVPCConnections
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] PredictQAResults
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] PutDataSetRefreshProperties
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] RegisterUser
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] RestoreAnalysis
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] SearchActionConnectors
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] SearchAnalyses
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] SearchDashboards
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] SearchDataSets
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] SearchDataSources
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] SearchFlows
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] SearchFolders
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] SearchGroups
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] SearchTopics
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartAssetBundleExportJob
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartAssetBundleImportJob
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartAutomationJob
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartDashboardSnapshotJob
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartDashboardSnapshotJobSchedule
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] TagResource
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UntagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateAccountCustomPermission
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateAccountCustomization
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UpdateAccountSettings
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateActionConnector
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateActionConnectorPermissions
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateAnalysis
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateAnalysisPermissions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateApplicationWithTokenExchangeGrant
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateBrand
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateBrandAssignment
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateBrandPublishedVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateCustomPermissions
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateDashboard
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateDashboardLinks
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateDashboardPermissions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateDashboardPublishedVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateDashboardsQAConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateDataSet
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateDataSetPermissions
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UpdateDataSource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateDataSourcePermissions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateDefaultQBusinessApplication
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateFlowPermissions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateFolder
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateFolderPermissions
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UpdateGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateIAMPolicyAssignment
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateIdentityPropagationConfig
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateIpRestriction
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateKeyRegistration
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UpdatePublicSharingSettings
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateQPersonalizationConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateQuickSightQSearchConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateRefreshSchedule
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateRoleCustomPermission
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateSPICECapacityConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateSelfUpgrade
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateSelfUpgradeConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateTemplateAlias
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateTemplatePermissions
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateTheme
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateThemeAlias
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateThemePermissions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateTopic
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateTopicPermissions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateTopicRefreshSchedule
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UpdateUser
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateUserCustomPermission
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateVPCConnection

Integration tests: 66/68 implemented operations tested (97.1%)
Untested implemented operations: 2

### winterbaume-ram (ram) - W: 35/35, S: 0/35, M: 8/35, F: 0/35, K: 0/35, C: 0/35

- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] AcceptResourceShareInvitation
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] AssociateResourceShare
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] AssociateResourceSharePermission
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreatePermission
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreatePermissionVersion
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateResourceShare
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeletePermission
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeletePermissionVersion
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteResourceShare
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DisassociateResourceShare
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DisassociateResourceSharePermission
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] EnableSharingWithAwsOrganization
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetPermission
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetResourcePolicies
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetResourceShareAssociations
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetResourceShareInvitations
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetResourceShares
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListPendingInvitationResources
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListPermissionAssociations
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListPermissionVersions
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListPermissions
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListPrincipals
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListReplacePermissionAssociationsWork
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListResourceSharePermissions
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListResourceTypes
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListResources
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListSourceAssociations
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PromotePermissionCreatedFromPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PromoteResourceShareCreatedFromPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] RejectResourceShareInvitation
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ReplacePermissionAssociations
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] SetDefaultPermissionVersion
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UntagResource
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UpdateResourceShare

Integration tests: 32/35 implemented operations tested (91.4%)
Untested implemented operations: 3

### winterbaume-rbin (rbin) - W: 9/10, S: 0/10, M: 0/10, F: 0/10, K: 0/10, C: 0/10

- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateRule
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteRule
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetRule
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListRules
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] LockRule
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UnlockRule
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateRule

Integration tests: 9/9 implemented operations tested (100.0%)

### winterbaume-rds (rds) - W: 146/164, S: 4/164, M: 85/164, F: 0/164, K: 12/164, C: 164/164

Terraform E2E: 6 tests across 2 terraform resource types

Resource types: aws_db_parameter_group, aws_db_subnet_group

- W[x] S[ ] M[x] F[ ] K[ ] C[x] AddRoleToDBCluster
- W[x] S[ ] M[x] F[ ] K[ ] C[x] AddRoleToDBInstance
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] AddSourceIdentifierToSubscription
- W[x] S[ ] M[x] F[ ] K[ ] C[x] AddTagsToResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ApplyPendingMaintenanceAction
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] AuthorizeDBSecurityGroupIngress
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] BacktrackDBCluster
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CancelExportTask
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CopyDBClusterParameterGroup
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CopyDBClusterSnapshot
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CopyDBParameterGroup
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CopyDBSnapshot
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CopyOptionGroup
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateBlueGreenDeployment
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] CreateCustomDBEngineVersion
- W[x] S[ ] M[x] F[ ] K[x] C[x] CreateDBCluster
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateDBClusterEndpoint
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateDBClusterParameterGroup
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateDBClusterSnapshot
- W[x] S[ ] M[x] F[ ] K[x] C[x] CreateDBInstance
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateDBInstanceReadReplica
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateDBParameterGroup
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateDBProxy
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateDBProxyEndpoint
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateDBSecurityGroup
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateDBShardGroup
- W[x] S[ ] M[x] F[ ] K[x] C[x] CreateDBSnapshot
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateDBSubnetGroup
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateEventSubscription
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateGlobalCluster
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] CreateIntegration
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateOptionGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] CreateTenantDatabase
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteBlueGreenDeployment
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DeleteCustomDBEngineVersion
- W[x] S[ ] M[x] F[ ] K[x] C[x] DeleteDBCluster
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteDBClusterAutomatedBackup
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteDBClusterEndpoint
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteDBClusterParameterGroup
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteDBClusterSnapshot
- W[x] S[ ] M[x] F[ ] K[x] C[x] DeleteDBInstance
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteDBInstanceAutomatedBackup
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteDBParameterGroup
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteDBProxy
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteDBProxyEndpoint
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteDBSecurityGroup
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteDBShardGroup
- W[x] S[ ] M[x] F[ ] K[x] C[x] DeleteDBSnapshot
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteDBSubnetGroup
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteEventSubscription
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteGlobalCluster
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DeleteIntegration
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteOptionGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DeleteTenantDatabase
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeregisterDBProxyTargets
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeAccountAttributes
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeBlueGreenDeployments
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] DescribeCertificates
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeDBClusterAutomatedBackups
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeDBClusterBacktracks
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeDBClusterEndpoints
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeDBClusterParameterGroups
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeDBClusterParameters
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeDBClusterSnapshotAttributes
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeDBClusterSnapshots
- W[x] S[ ] M[x] F[ ] K[x] C[x] DescribeDBClusters
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeDBEngineVersions
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeDBInstanceAutomatedBackups
- W[x] S[ ] M[x] F[ ] K[x] C[x] DescribeDBInstances
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeDBLogFiles
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeDBMajorEngineVersions
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeDBParameterGroups
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeDBParameters
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeDBProxies
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeDBProxyEndpoints
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeDBProxyTargetGroups
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeDBProxyTargets
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeDBRecommendations
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeDBSecurityGroups
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeDBShardGroups
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeDBSnapshotAttributes
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeDBSnapshotTenantDatabases
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeDBSnapshots
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeDBSubnetGroups
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeEngineDefaultClusterParameters
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeEngineDefaultParameters
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeEventCategories
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeEventSubscriptions
- W[ ] S[x] M[x] F[ ] K[ ] C[x] DescribeEvents
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeExportTasks
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeGlobalClusters
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DescribeIntegrations
- W[ ] S[x] M[x] F[ ] K[ ] C[x] DescribeOptionGroupOptions
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeOptionGroups
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeOrderableDBInstanceOptions
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] DescribePendingMaintenanceActions
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeReservedDBInstances
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeReservedDBInstancesOfferings
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DescribeServerlessV2PlatformVersions
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeSourceRegions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DescribeTenantDatabases
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeValidDBInstanceModifications
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DisableHttpEndpoint
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DownloadDBLogFilePortion
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] EnableHttpEndpoint
- W[x] S[ ] M[x] F[ ] K[ ] C[x] FailoverDBCluster
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] FailoverGlobalCluster
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ModifyActivityStream
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ModifyCertificates
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ModifyCurrentDBClusterCapacity
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ModifyCustomDBEngineVersion
- W[x] S[ ] M[x] F[ ] K[x] C[x] ModifyDBCluster
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ModifyDBClusterEndpoint
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ModifyDBClusterParameterGroup
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ModifyDBClusterSnapshotAttribute
- W[x] S[ ] M[x] F[ ] K[x] C[x] ModifyDBInstance
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ModifyDBParameterGroup
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ModifyDBProxy
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ModifyDBProxyEndpoint
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ModifyDBProxyTargetGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ModifyDBRecommendation
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ModifyDBShardGroup
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ModifyDBSnapshot
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ModifyDBSnapshotAttribute
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ModifyDBSubnetGroup
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ModifyEventSubscription
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ModifyGlobalCluster
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ModifyIntegration
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ModifyOptionGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ModifyTenantDatabase
- W[x] S[ ] M[x] F[ ] K[ ] C[x] PromoteReadReplica
- W[x] S[ ] M[x] F[ ] K[ ] C[x] PromoteReadReplicaDBCluster
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] PurchaseReservedDBInstancesOffering
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] RebootDBCluster
- W[x] S[ ] M[x] F[ ] K[ ] C[x] RebootDBInstance
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] RebootDBShardGroup
- W[x] S[ ] M[x] F[ ] K[ ] C[x] RegisterDBProxyTargets
- W[x] S[ ] M[x] F[ ] K[ ] C[x] RemoveFromGlobalCluster
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] RemoveRoleFromDBCluster
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] RemoveRoleFromDBInstance
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] RemoveSourceIdentifierFromSubscription
- W[x] S[ ] M[x] F[ ] K[ ] C[x] RemoveTagsFromResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ResetDBClusterParameterGroup
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ResetDBParameterGroup
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] RestoreDBClusterFromS3
- W[x] S[ ] M[x] F[ ] K[ ] C[x] RestoreDBClusterFromSnapshot
- W[x] S[ ] M[x] F[ ] K[ ] C[x] RestoreDBClusterToPointInTime
- W[x] S[ ] M[x] F[ ] K[ ] C[x] RestoreDBInstanceFromDBSnapshot
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] RestoreDBInstanceFromS3
- W[x] S[ ] M[x] F[ ] K[ ] C[x] RestoreDBInstanceToPointInTime
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] RevokeDBSecurityGroupIngress
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] StartActivityStream
- W[x] S[ ] M[x] F[ ] K[ ] C[x] StartDBCluster
- W[x] S[ ] M[x] F[ ] K[x] C[x] StartDBInstance
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] StartDBInstanceAutomatedBackupsReplication
- W[x] S[ ] M[x] F[ ] K[ ] C[x] StartExportTask
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] StopActivityStream
- W[x] S[ ] M[x] F[ ] K[ ] C[x] StopDBCluster
- W[x] S[ ] M[x] F[ ] K[x] C[x] StopDBInstance
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] StopDBInstanceAutomatedBackupsReplication
- W[x] S[ ] M[x] F[ ] K[ ] C[x] SwitchoverBlueGreenDeployment
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] SwitchoverGlobalCluster
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] SwitchoverReadReplica

Integration tests: 78/146 implemented operations tested (53.4%)
Untested implemented operations: 68

### winterbaume-rdsdata (rds-data) - W: 6/6, S: 0/6, M: 1/6, F: 0/6, K: 0/6, C: 6/6

- W[x] S[ ] M[ ] F[ ] K[ ] C[x] BatchExecuteStatement
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] BeginTransaction
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CommitTransaction
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ExecuteSql
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ExecuteStatement
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] RollbackTransaction

Integration tests: 6/6 implemented operations tested (100.0%)

### winterbaume-redshift (redshift) - W: 100/141, S: 3/141, M: 35/141, F: 0/141, K: 7/141, C: 0/141

- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AcceptReservedNodeExchange
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] AddPartner
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AssociateDataShareConsumer
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] AuthorizeClusterSecurityGroupIngress
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AuthorizeDataShare
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AuthorizeEndpointAccess
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] AuthorizeSnapshotAccess
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] BatchDeleteClusterSnapshots
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] BatchModifyClusterSnapshots
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CancelResize
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CopyClusterSnapshot
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateAuthenticationProfile
- W[x] S[ ] M[x] F[ ] K[x] C[ ] CreateCluster
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateClusterParameterGroup
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateClusterSecurityGroup
- W[x] S[ ] M[x] F[ ] K[x] C[ ] CreateClusterSnapshot
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateClusterSubnetGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateCustomDomainAssociation
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateEndpointAccess
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateEventSubscription
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateHsmClientCertificate
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateHsmConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateIntegration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateRedshiftIdcApplication
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateScheduledAction
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateSnapshotCopyGrant
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateSnapshotSchedule
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateTags
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateUsageLimit
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeauthorizeDataShare
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteAuthenticationProfile
- W[x] S[ ] M[x] F[ ] K[x] C[ ] DeleteCluster
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteClusterParameterGroup
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteClusterSecurityGroup
- W[x] S[ ] M[x] F[ ] K[x] C[ ] DeleteClusterSnapshot
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteClusterSubnetGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteCustomDomainAssociation
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteEndpointAccess
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteEventSubscription
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteHsmClientCertificate
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteHsmConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteIntegration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeletePartner
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteRedshiftIdcApplication
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteResourcePolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteScheduledAction
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteSnapshotCopyGrant
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteSnapshotSchedule
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteTags
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteUsageLimit
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeregisterNamespace
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeAccountAttributes
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeAuthenticationProfiles
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeClusterDbRevisions
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeClusterParameterGroups
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeClusterParameters
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeClusterSecurityGroups
- W[x] S[ ] M[x] F[ ] K[x] C[ ] DescribeClusterSnapshots
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeClusterSubnetGroups
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeClusterTracks
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeClusterVersions
- W[x] S[ ] M[x] F[ ] K[x] C[ ] DescribeClusters
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeCustomDomainAssociations
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeDataShares
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeDataSharesForConsumer
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeDataSharesForProducer
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeDefaultClusterParameters
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeEndpointAccess
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeEndpointAuthorization
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeEventCategories
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeEventSubscriptions
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] DescribeEvents
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeHsmClientCertificates
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeHsmConfigurations
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeInboundIntegrations
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeIntegrations
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeLoggingStatus
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeNodeConfigurationOptions
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeOrderableClusterOptions
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribePartners
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeRedshiftIdcApplications
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeReservedNodeExchangeStatus
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeReservedNodeOfferings
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeReservedNodes
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeResize
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeScheduledActions
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeSnapshotCopyGrants
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeSnapshotSchedules
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] DescribeStorage
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeTableRestoreStatus
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeTags
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeUsageLimits
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DisableLogging
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DisableSnapshotCopy
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DisassociateDataShareConsumer
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] EnableLogging
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] EnableSnapshotCopy
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] FailoverPrimaryCompute
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetClusterCredentials
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetClusterCredentialsWithIAM
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetIdentityCenterAuthToken
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetReservedNodeExchangeConfigurationOptions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetReservedNodeExchangeOfferings
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetResourcePolicy
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] ListRecommendations
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ModifyAquaConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ModifyAuthenticationProfile
- W[x] S[ ] M[x] F[ ] K[x] C[ ] ModifyCluster
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ModifyClusterDbRevision
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ModifyClusterIamRoles
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ModifyClusterMaintenance
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ModifyClusterParameterGroup
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ModifyClusterSnapshot
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ModifyClusterSnapshotSchedule
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ModifyClusterSubnetGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ModifyCustomDomainAssociation
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ModifyEndpointAccess
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ModifyEventSubscription
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ModifyIntegration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ModifyLakehouseConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ModifyRedshiftIdcApplication
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ModifyScheduledAction
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ModifySnapshotCopyRetentionPeriod
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ModifySnapshotSchedule
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ModifyUsageLimit
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] PauseCluster
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] PurchaseReservedNodeOffering
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutResourcePolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] RebootCluster
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] RegisterNamespace
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] RejectDataShare
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ResetClusterParameterGroup
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ResizeCluster
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] RestoreFromClusterSnapshot
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] RestoreTableFromClusterSnapshot
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ResumeCluster
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] RevokeClusterSecurityGroupIngress
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] RevokeEndpointAccess
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] RevokeSnapshotAccess
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] RotateEncryptionKey
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdatePartnerStatus

Integration tests: 99/100 implemented operations tested (99.0%)
Untested implemented operations: 1

### winterbaume-redshiftdata (redshift-data) - W: 11/11, S: 0/11, M: 4/11, F: 0/11, K: 0/11, C: 0/11

- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] BatchExecuteStatement
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CancelStatement
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeStatement
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeTable
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ExecuteStatement
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetStatementResult
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetStatementResultV2
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListDatabases
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListSchemas
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListStatements
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListTables

Integration tests: 11/11 implemented operations tested (100.0%)

### winterbaume-rekognition (rekognition) - W: 8/75, S: 4/75, M: 8/75, F: 0/75, K: 13/75, C: 0/75

- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AssociateFaces
- W[ ] S[x] M[x] F[ ] K[ ] C[ ] CompareFaces
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CopyProjectVersion
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] CreateCollection
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateDataset
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateFaceLivenessSession
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateProject
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateProjectVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateStreamProcessor
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateUser
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] DeleteCollection
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteDataset
- W[ ] S[ ] M[ ] F[ ] K[x] C[ ] DeleteFaces
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteProject
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteProjectPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteProjectVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteStreamProcessor
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteUser
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] DescribeCollection
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeDataset
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeProjectVersions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeProjects
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeStreamProcessor
- W[ ] S[x] M[x] F[ ] K[ ] C[ ] DetectCustomLabels
- W[ ] S[ ] M[ ] F[ ] K[x] C[ ] DetectFaces
- W[ ] S[x] M[x] F[ ] K[x] C[ ] DetectLabels
- W[ ] S[ ] M[ ] F[ ] K[x] C[ ] DetectModerationLabels
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DetectProtectiveEquipment
- W[ ] S[x] M[x] F[ ] K[x] C[ ] DetectText
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DisassociateFaces
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DistributeDatasetEntries
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetCelebrityInfo
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetCelebrityRecognition
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetContentModeration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetFaceDetection
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetFaceLivenessSessionResults
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetFaceSearch
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetLabelDetection
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetMediaAnalysisJob
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetPersonTracking
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetSegmentDetection
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetTextDetection
- W[ ] S[ ] M[ ] F[ ] K[x] C[ ] IndexFaces
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] ListCollections
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListDatasetEntries
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListDatasetLabels
- W[ ] S[ ] M[ ] F[ ] K[x] C[ ] ListFaces
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListMediaAnalysisJobs
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListProjectPolicies
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListStreamProcessors
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListTagsForResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListUsers
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] PutProjectPolicy
- W[ ] S[ ] M[ ] F[ ] K[x] C[ ] RecognizeCelebrities
- W[ ] S[ ] M[ ] F[ ] K[x] C[ ] SearchFaces
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] SearchFacesByImage
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] SearchUsers
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] SearchUsersByImage
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartCelebrityRecognition
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartContentModeration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartFaceDetection
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] StartFaceSearch
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartLabelDetection
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartMediaAnalysisJob
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartPersonTracking
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartProjectVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartSegmentDetection
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartStreamProcessor
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] StartTextDetection
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StopProjectVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StopStreamProcessor
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] TagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UntagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateDatasetEntries
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateStreamProcessor

Integration tests: 8/8 implemented operations tested (100.0%)

### winterbaume-resiliencehub (resiliencehub) - W: 22/63, S: 0/63, M: 17/63, F: 0/63, K: 17/63, C: 0/63

- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AcceptResourceGroupingRecommendations
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AddDraftAppVersionResourceMappings
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] BatchUpdateRecommendationStatus
- W[x] S[ ] M[x] F[ ] K[x] C[ ] CreateApp
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateAppVersionAppComponent
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateAppVersionResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateRecommendationTemplate
- W[x] S[ ] M[x] F[ ] K[x] C[ ] CreateResiliencyPolicy
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] DeleteApp
- W[ ] S[ ] M[ ] F[ ] K[x] C[ ] DeleteAppAssessment
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteAppInputSource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteAppVersionAppComponent
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteAppVersionResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteRecommendationTemplate
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] DeleteResiliencyPolicy
- W[x] S[ ] M[x] F[ ] K[x] C[ ] DescribeApp
- W[ ] S[ ] M[ ] F[ ] K[x] C[ ] DescribeAppAssessment
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeAppVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeAppVersionAppComponent
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeAppVersionResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeAppVersionResourcesResolutionStatus
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeAppVersionTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeDraftAppVersionResourcesImportStatus
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeMetricsExport
- W[x] S[ ] M[x] F[ ] K[x] C[ ] DescribeResiliencyPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeResourceGroupingRecommendationTask
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ImportResourcesToDraftAppVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListAlarmRecommendations
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListAppAssessmentComplianceDrifts
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListAppAssessmentResourceDrifts
- W[x] S[ ] M[x] F[ ] K[x] C[ ] ListAppAssessments
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListAppComponentCompliances
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListAppComponentRecommendations
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListAppInputSources
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListAppVersionAppComponents
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListAppVersionResourceMappings
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListAppVersionResources
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListAppVersions
- W[x] S[ ] M[x] F[ ] K[x] C[ ] ListApps
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListMetrics
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListRecommendationTemplates
- W[x] S[ ] M[x] F[ ] K[x] C[ ] ListResiliencyPolicies
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListResourceGroupingRecommendations
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListSopRecommendations
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListSuggestedResiliencyPolicies
- W[x] S[ ] M[x] F[ ] K[x] C[ ] ListTagsForResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListTestRecommendations
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListUnsupportedAppVersionResources
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] PublishAppVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] PutDraftAppVersionTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] RejectResourceGroupingRecommendations
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] RemoveDraftAppVersionResourceMappings
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ResolveAppVersionResources
- W[ ] S[ ] M[ ] F[ ] K[x] C[ ] StartAppAssessment
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartMetricsExport
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartResourceGroupingRecommendationTask
- W[x] S[ ] M[x] F[ ] K[x] C[ ] TagResource
- W[x] S[ ] M[x] F[ ] K[x] C[ ] UntagResource
- W[ ] S[ ] M[ ] F[ ] K[x] C[ ] UpdateApp
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateAppVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateAppVersionAppComponent
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateAppVersionResource
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] UpdateResiliencyPolicy

Integration tests: 22/22 implemented operations tested (100.0%)

### winterbaume-resourcegroups (resource-groups) - W: 22/23, S: 1/23, M: 15/23, F: 0/23, K: 0/23, C: 23/23

- W[x] S[ ] M[x] F[ ] K[ ] C[x] CancelTagSyncTask
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateGroup
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteGroup
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetAccountSettings
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetGroup
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetGroupConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetGroupQuery
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetTagSyncTask
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetTags
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GroupResources
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListGroupResources
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListGroupingStatuses
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListGroups
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListTagSyncTasks
- W[x] S[ ] M[x] F[ ] K[ ] C[x] PutGroupConfiguration
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] SearchResources
- W[x] S[ ] M[x] F[ ] K[ ] C[x] StartTagSyncTask
- W[x] S[ ] M[x] F[ ] K[ ] C[x] Tag
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UngroupResources
- W[x] S[ ] M[x] F[ ] K[ ] C[x] Untag
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateAccountSettings
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UpdateGroup
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UpdateGroupQuery

Integration tests: 15/22 implemented operations tested (68.2%)
Untested implemented operations: 7

### winterbaume-resourcegroupstagging (resource-groups-tagging-api) - W: 5/9, S: 0/9, M: 0/9, F: 0/9, K: 0/9, C: 9/9

- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DescribeReportCreation
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetComplianceSummary
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetResources
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetTagKeys
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetTagValues
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ListRequiredTags
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] StartReportCreation
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] TagResources
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UntagResources

Integration tests: 5/5 implemented operations tested (100.0%)

### winterbaume-rolesanywhere (rolesanywhere) - W: 28/30, S: 2/30, M: 0/30, F: 0/30, K: 0/30, C: 0/30

- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateProfile
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateTrustAnchor
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteAttributeMapping
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteCrl
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteProfile
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteTrustAnchor
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DisableCrl
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DisableProfile
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DisableTrustAnchor
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] EnableCrl
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] EnableProfile
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] EnableTrustAnchor
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetCrl
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetProfile
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] GetSubject
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetTrustAnchor
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ImportCrl
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListCrls
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListProfiles
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] ListSubjects
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListTrustAnchors
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutAttributeMapping
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutNotificationSettings
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ResetNotificationSettings
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateCrl
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateProfile
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateTrustAnchor

Integration tests: 22/28 implemented operations tested (78.6%)
Untested implemented operations: 6

### winterbaume-route53 (route-53) - W: 71/71, S: 0/71, M: 30/71, F: 0/71, K: 10/71, C: 71/71

Terraform E2E: 6 tests across 2 terraform resource types

Resource types: aws_route53_record, aws_route53_zone

- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ActivateKeySigningKey
- W[x] S[ ] M[x] F[ ] K[ ] C[x] AssociateVPCWithHostedZone
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ChangeCidrCollection
- W[x] S[ ] M[x] F[ ] K[x] C[x] ChangeResourceRecordSets
- W[x] S[ ] M[x] F[ ] K[x] C[x] ChangeTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateCidrCollection
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateHealthCheck
- W[x] S[ ] M[x] F[ ] K[x] C[x] CreateHostedZone
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateKeySigningKey
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateQueryLoggingConfig
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateReusableDelegationSet
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateTrafficPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateTrafficPolicyInstance
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateTrafficPolicyVersion
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateVPCAssociationAuthorization
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeactivateKeySigningKey
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteCidrCollection
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteHealthCheck
- W[x] S[ ] M[x] F[ ] K[x] C[x] DeleteHostedZone
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteKeySigningKey
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteQueryLoggingConfig
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteReusableDelegationSet
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteTrafficPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteTrafficPolicyInstance
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteVPCAssociationAuthorization
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DisableHostedZoneDNSSEC
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DisassociateVPCFromHostedZone
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] EnableHostedZoneDNSSEC
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetAccountLimit
- W[x] S[ ] M[ ] F[ ] K[x] C[x] GetChange
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetCheckerIpRanges
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetDNSSEC
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetGeoLocation
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetHealthCheck
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetHealthCheckCount
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetHealthCheckLastFailureReason
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetHealthCheckStatus
- W[x] S[ ] M[x] F[ ] K[x] C[x] GetHostedZone
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetHostedZoneCount
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetHostedZoneLimit
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetQueryLoggingConfig
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetReusableDelegationSet
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetReusableDelegationSetLimit
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetTrafficPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetTrafficPolicyInstance
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetTrafficPolicyInstanceCount
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListCidrBlocks
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListCidrCollections
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListCidrLocations
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListGeoLocations
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListHealthChecks
- W[x] S[ ] M[x] F[ ] K[x] C[x] ListHostedZones
- W[x] S[ ] M[x] F[ ] K[x] C[x] ListHostedZonesByName
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListHostedZonesByVPC
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListQueryLoggingConfigs
- W[x] S[ ] M[x] F[ ] K[x] C[x] ListResourceRecordSets
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListReusableDelegationSets
- W[x] S[ ] M[x] F[ ] K[x] C[x] ListTagsForResource
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListTagsForResources
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListTrafficPolicies
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListTrafficPolicyInstances
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListTrafficPolicyInstancesByHostedZone
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListTrafficPolicyInstancesByPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListTrafficPolicyVersions
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListVPCAssociationAuthorizations
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] TestDNSAnswer
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UpdateHealthCheck
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UpdateHostedZoneComment
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateHostedZoneFeatures
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateTrafficPolicyComment
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateTrafficPolicyInstance

Integration tests: 70/71 implemented operations tested (98.6%)
Untested implemented operations: 1

### winterbaume-route53domains (route-53-domains) - W: 5/34, S: 0/34, M: 0/34, F: 0/34, K: 0/34, C: 0/34

- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AcceptDomainTransferFromAnotherAwsAccount
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AssociateDelegationSignerToDomain
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CancelDomainTransferToAnotherAwsAccount
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CheckDomainAvailability
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CheckDomainTransferability
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteDomain
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteTagsForDomain
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DisableDomainAutoRenew
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DisableDomainTransferLock
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DisassociateDelegationSignerFromDomain
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] EnableDomainAutoRenew
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] EnableDomainTransferLock
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetContactReachabilityStatus
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetDomainDetail
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetDomainSuggestions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetOperationDetail
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListDomains
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListOperations
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListPrices
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListTagsForDomain
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] PushDomain
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] RegisterDomain
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] RejectDomainTransferFromAnotherAwsAccount
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] RenewDomain
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ResendContactReachabilityEmail
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ResendOperationAuthorization
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] RetrieveDomainAuthCode
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] TransferDomain
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] TransferDomainToAnotherAwsAccount
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateDomainContact
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateDomainContactPrivacy
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateDomainNameservers
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateTagsForDomain
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ViewBilling

Integration tests: 5/5 implemented operations tested (100.0%)

### winterbaume-route53recoverycluster (route53-recovery-cluster) - W: 4/4, S: 0/4, M: 0/4, F: 0/4, K: 0/4, C: 0/4

- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetRoutingControlState
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListRoutingControls
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateRoutingControlState
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateRoutingControlStates

Integration tests: 4/4 implemented operations tested (100.0%)

### winterbaume-route53resolver (route53resolver) - W: 28/68, S: 0/68, M: 28/68, F: 0/68, K: 11/68, C: 0/68

- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AssociateFirewallRuleGroup
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] AssociateResolverEndpointIpAddress
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] AssociateResolverQueryLogConfig
- W[x] S[ ] M[x] F[ ] K[x] C[ ] AssociateResolverRule
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateFirewallDomainList
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateFirewallRule
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateFirewallRuleGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateOutpostResolver
- W[x] S[ ] M[x] F[ ] K[x] C[ ] CreateResolverEndpoint
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateResolverQueryLogConfig
- W[x] S[ ] M[x] F[ ] K[x] C[ ] CreateResolverRule
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteFirewallDomainList
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteFirewallRule
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteFirewallRuleGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteOutpostResolver
- W[x] S[ ] M[x] F[ ] K[x] C[ ] DeleteResolverEndpoint
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteResolverQueryLogConfig
- W[x] S[ ] M[x] F[ ] K[x] C[ ] DeleteResolverRule
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DisassociateFirewallRuleGroup
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DisassociateResolverEndpointIpAddress
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DisassociateResolverQueryLogConfig
- W[x] S[ ] M[x] F[ ] K[x] C[ ] DisassociateResolverRule
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetFirewallConfig
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetFirewallDomainList
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetFirewallRuleGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetFirewallRuleGroupAssociation
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetFirewallRuleGroupPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetOutpostResolver
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetResolverConfig
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetResolverDnssecConfig
- W[x] S[ ] M[x] F[ ] K[x] C[ ] GetResolverEndpoint
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetResolverQueryLogConfig
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetResolverQueryLogConfigAssociation
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetResolverQueryLogConfigPolicy
- W[x] S[ ] M[x] F[ ] K[x] C[ ] GetResolverRule
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetResolverRuleAssociation
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetResolverRulePolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ImportFirewallDomains
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListFirewallConfigs
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListFirewallDomainLists
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListFirewallDomains
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListFirewallRuleGroupAssociations
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListFirewallRuleGroups
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListFirewallRules
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListOutpostResolvers
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListResolverConfigs
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListResolverDnssecConfigs
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListResolverEndpointIpAddresses
- W[x] S[ ] M[x] F[ ] K[x] C[ ] ListResolverEndpoints
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListResolverQueryLogConfigAssociations
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListResolverQueryLogConfigs
- W[x] S[ ] M[x] F[ ] K[x] C[ ] ListResolverRuleAssociations
- W[x] S[ ] M[x] F[ ] K[x] C[ ] ListResolverRules
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListTagsForResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] PutFirewallRuleGroupPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] PutResolverQueryLogConfigPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] PutResolverRulePolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] TagResource
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UntagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateFirewallConfig
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateFirewallDomains
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateFirewallRule
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateFirewallRuleGroupAssociation
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateOutpostResolver
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateResolverConfig
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UpdateResolverDnssecConfig
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UpdateResolverEndpoint
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateResolverRule

Integration tests: 28/28 implemented operations tested (100.0%)

### winterbaume-s3 (s3) - W: 103/107, S: 4/107, M: 73/107, F: 51/107, K: 44/107, C: 107/107

Terraform E2E: 13 tests across 6 terraform resource types

Resource types: aws_s3_bucket, aws_s3_bucket_ownership_controls, aws_s3_bucket_policy, aws_s3_bucket_public_access_block, aws_s3_bucket_server_side_encryption_configuration, aws_s3_bucket_versioning

- W[x] S[ ] M[x] F[x] K[x] C[x] AbortMultipartUpload
- W[x] S[ ] M[x] F[x] K[x] C[x] CompleteMultipartUpload
- W[x] S[ ] M[x] F[x] K[x] C[x] CopyObject
- W[x] S[ ] M[x] F[x] K[x] C[x] CreateBucket
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateBucketMetadataConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateBucketMetadataTableConfiguration
- W[x] S[ ] M[x] F[x] K[x] C[x] CreateMultipartUpload
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] CreateSession
- W[x] S[ ] M[x] F[x] K[x] C[x] DeleteBucket
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteBucketAnalyticsConfiguration
- W[x] S[ ] M[x] F[x] K[ ] C[x] DeleteBucketCors
- W[x] S[ ] M[x] F[x] K[x] C[x] DeleteBucketEncryption
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteBucketIntelligentTieringConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteBucketInventoryConfiguration
- W[x] S[ ] M[x] F[x] K[x] C[x] DeleteBucketLifecycle
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteBucketMetadataConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteBucketMetadataTableConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteBucketMetricsConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteBucketOwnershipControls
- W[x] S[ ] M[x] F[x] K[x] C[x] DeleteBucketPolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteBucketReplication
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteBucketTagging
- W[x] S[ ] M[x] F[ ] K[x] C[x] DeleteBucketWebsite
- W[x] S[ ] M[x] F[x] K[x] C[x] DeleteObject
- W[x] S[ ] M[x] F[x] K[ ] C[x] DeleteObjectTagging
- W[x] S[ ] M[x] F[x] K[x] C[x] DeleteObjects
- W[x] S[ ] M[x] F[x] K[x] C[x] DeletePublicAccessBlock
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetBucketAbac
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetBucketAccelerateConfiguration
- W[x] S[ ] M[x] F[x] K[ ] C[x] GetBucketAcl
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetBucketAnalyticsConfiguration
- W[x] S[ ] M[x] F[x] K[x] C[x] GetBucketCors
- W[x] S[ ] M[x] F[x] K[x] C[x] GetBucketEncryption
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetBucketIntelligentTieringConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetBucketInventoryConfiguration
- W[x] S[ ] M[x] F[ ] K[x] C[x] GetBucketLifecycleConfiguration
- W[x] S[ ] M[x] F[x] K[ ] C[x] GetBucketLocation
- W[x] S[ ] M[x] F[ ] K[x] C[x] GetBucketLogging
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetBucketMetadataConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetBucketMetadataTableConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetBucketMetricsConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetBucketNotificationConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetBucketOwnershipControls
- W[x] S[ ] M[x] F[x] K[x] C[x] GetBucketPolicy
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] GetBucketPolicyStatus
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetBucketReplication
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetBucketRequestPayment
- W[x] S[ ] M[x] F[x] K[ ] C[x] GetBucketTagging
- W[x] S[ ] M[x] F[x] K[x] C[x] GetBucketVersioning
- W[x] S[ ] M[ ] F[ ] K[x] C[x] GetBucketWebsite
- W[x] S[ ] M[x] F[x] K[x] C[x] GetObject
- W[x] S[ ] M[x] F[x] K[ ] C[x] GetObjectAcl
- W[x] S[ ] M[x] F[x] K[ ] C[x] GetObjectAttributes
- W[x] S[ ] M[x] F[x] K[ ] C[x] GetObjectLegalHold
- W[x] S[ ] M[x] F[x] K[ ] C[x] GetObjectLockConfiguration
- W[x] S[ ] M[ ] F[x] K[ ] C[x] GetObjectRetention
- W[x] S[ ] M[x] F[x] K[x] C[x] GetObjectTagging
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] GetObjectTorrent
- W[x] S[ ] M[x] F[x] K[x] C[x] GetPublicAccessBlock
- W[x] S[ ] M[x] F[x] K[x] C[x] HeadBucket
- W[x] S[ ] M[x] F[x] K[x] C[x] HeadObject
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListBucketAnalyticsConfigurations
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListBucketIntelligentTieringConfigurations
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListBucketInventoryConfigurations
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListBucketMetricsConfigurations
- W[x] S[ ] M[x] F[x] K[x] C[x] ListBuckets
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListDirectoryBuckets
- W[x] S[ ] M[x] F[x] K[x] C[x] ListMultipartUploads
- W[x] S[ ] M[x] F[x] K[x] C[x] ListObjectVersions
- W[x] S[ ] M[x] F[x] K[x] C[x] ListObjects
- W[x] S[ ] M[x] F[x] K[ ] C[x] ListObjectsV2
- W[x] S[ ] M[x] F[ ] K[x] C[x] ListParts
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] PutBucketAbac
- W[x] S[ ] M[x] F[ ] K[ ] C[x] PutBucketAccelerateConfiguration
- W[x] S[ ] M[x] F[x] K[ ] C[x] PutBucketAcl
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] PutBucketAnalyticsConfiguration
- W[x] S[ ] M[x] F[x] K[x] C[x] PutBucketCors
- W[x] S[ ] M[x] F[x] K[x] C[x] PutBucketEncryption
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] PutBucketIntelligentTieringConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] C[x] PutBucketInventoryConfiguration
- W[x] S[ ] M[x] F[ ] K[x] C[x] PutBucketLifecycleConfiguration
- W[x] S[ ] M[x] F[ ] K[x] C[x] PutBucketLogging
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] PutBucketMetricsConfiguration
- W[x] S[ ] M[x] F[ ] K[x] C[x] PutBucketNotificationConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] C[x] PutBucketOwnershipControls
- W[x] S[ ] M[x] F[x] K[x] C[x] PutBucketPolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[x] PutBucketReplication
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] PutBucketRequestPayment
- W[x] S[ ] M[x] F[x] K[ ] C[x] PutBucketTagging
- W[x] S[ ] M[x] F[x] K[x] C[x] PutBucketVersioning
- W[x] S[ ] M[x] F[ ] K[x] C[x] PutBucketWebsite
- W[x] S[ ] M[x] F[x] K[x] C[x] PutObject
- W[x] S[ ] M[x] F[x] K[ ] C[x] PutObjectAcl
- W[x] S[ ] M[x] F[x] K[ ] C[x] PutObjectLegalHold
- W[x] S[ ] M[x] F[x] K[ ] C[x] PutObjectLockConfiguration
- W[x] S[ ] M[x] F[x] K[ ] C[x] PutObjectRetention
- W[x] S[ ] M[x] F[x] K[x] C[x] PutObjectTagging
- W[x] S[ ] M[x] F[x] K[x] C[x] PutPublicAccessBlock
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] RenameObject
- W[ ] S[x] M[x] F[ ] K[x] C[x] RestoreObject
- W[x] S[ ] M[x] F[x] K[ ] C[x] SelectObjectContent
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateBucketMetadataInventoryTableConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateBucketMetadataJournalTableConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateObjectEncryption
- W[x] S[ ] M[x] F[x] K[x] C[x] UploadPart
- W[x] S[ ] M[x] F[ ] K[x] C[x] UploadPartCopy
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] WriteGetObjectResponse

Integration tests: 71/103 implemented operations tested (68.9%)
Untested implemented operations: 32

### winterbaume-s3control (s3-control) - W: 87/97, S: 10/97, M: 0/97, F: 0/97, K: 7/97, C: 0/97

- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] AssociateAccessGrantsIdentityCenter
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateAccessGrant
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateAccessGrantsInstance
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateAccessGrantsLocation
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] CreateAccessPoint
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateAccessPointForObjectLambda
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateBucket
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateJob
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateMultiRegionAccessPoint
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateStorageLensGroup
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteAccessGrant
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteAccessGrantsInstance
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteAccessGrantsInstanceResourcePolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteAccessGrantsLocation
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] DeleteAccessPoint
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteAccessPointForObjectLambda
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteAccessPointPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteAccessPointPolicyForObjectLambda
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteAccessPointScope
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteBucket
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteBucketLifecycleConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteBucketPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteBucketReplication
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteBucketTagging
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteJobTagging
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteMultiRegionAccessPoint
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] DeletePublicAccessBlock
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteStorageLensConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteStorageLensConfigurationTagging
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteStorageLensGroup
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeJob
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] DescribeMultiRegionAccessPointOperation
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] DissociateAccessGrantsIdentityCenter
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetAccessGrant
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetAccessGrantsInstance
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] GetAccessGrantsInstanceForPrefix
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetAccessGrantsInstanceResourcePolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetAccessGrantsLocation
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] GetAccessPoint
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] GetAccessPointConfigurationForObjectLambda
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetAccessPointForObjectLambda
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetAccessPointPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetAccessPointPolicyForObjectLambda
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetAccessPointPolicyStatus
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetAccessPointPolicyStatusForObjectLambda
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] GetAccessPointScope
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetBucket
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] GetBucketLifecycleConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetBucketPolicy
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] GetBucketReplication
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetBucketTagging
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetBucketVersioning
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] GetDataAccess
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetJobTagging
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetMultiRegionAccessPoint
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetMultiRegionAccessPointPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetMultiRegionAccessPointPolicyStatus
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetMultiRegionAccessPointRoutes
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] GetPublicAccessBlock
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetStorageLensConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetStorageLensConfigurationTagging
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetStorageLensGroup
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListAccessGrants
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListAccessGrantsInstances
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListAccessGrantsLocations
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] ListAccessPoints
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListAccessPointsForDirectoryBuckets
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListAccessPointsForObjectLambda
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] ListCallerAccessGrants
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListJobs
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListMultiRegionAccessPoints
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListRegionalBuckets
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListStorageLensConfigurations
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListStorageLensGroups
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutAccessGrantsInstanceResourcePolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutAccessPointConfigurationForObjectLambda
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutAccessPointPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutAccessPointPolicyForObjectLambda
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutAccessPointScope
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutBucketLifecycleConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutBucketPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutBucketReplication
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutBucketTagging
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutBucketVersioning
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutJobTagging
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutMultiRegionAccessPointPolicy
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] PutPublicAccessBlock
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutStorageLensConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutStorageLensConfigurationTagging
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] SubmitMultiRegionAccessPointRoutes
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateAccessGrantsLocation
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateJobPriority
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateJobStatus
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateStorageLensGroup

Integration tests: 68/87 implemented operations tested (78.2%)
Untested implemented operations: 19

### winterbaume-s3files (s3files) - W: 21/21, S: 0/21, M: 0/21, F: 0/21, K: 0/21, C: 0/21

- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateAccessPoint
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateFileSystem
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateMountTarget
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteAccessPoint
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteFileSystem
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteFileSystemPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteMountTarget
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetAccessPoint
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetFileSystem
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetFileSystemPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetMountTarget
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetSynchronizationConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListAccessPoints
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListFileSystems
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListMountTargets
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutFileSystemPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutSynchronizationConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateMountTarget

Integration tests: 21/21 implemented operations tested (100.0%)

### winterbaume-s3outposts (s3outposts) - W: 3/5, S: 1/5, M: 0/5, F: 0/5, K: 0/5, C: 0/5

- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateEndpoint
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteEndpoint
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListEndpoints
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListOutpostsWithS3
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] ListSharedEndpoints

Integration tests: 3/3 implemented operations tested (100.0%)

### winterbaume-s3tables (s3tables) - W: 46/49, S: 3/49, M: 14/49, F: 0/49, K: 12/49, C: 0/49

- W[x] S[ ] M[x] F[ ] K[x] C[ ] CreateNamespace
- W[x] S[ ] M[x] F[ ] K[x] C[ ] CreateTable
- W[x] S[ ] M[x] F[ ] K[x] C[ ] CreateTableBucket
- W[x] S[ ] M[x] F[ ] K[x] C[ ] DeleteNamespace
- W[x] S[ ] M[x] F[ ] K[x] C[ ] DeleteTable
- W[x] S[ ] M[x] F[ ] K[x] C[ ] DeleteTableBucket
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteTableBucketEncryption
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteTableBucketMetricsConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteTableBucketPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteTableBucketReplication
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteTablePolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteTableReplication
- W[x] S[ ] M[x] F[ ] K[x] C[ ] GetNamespace
- W[x] S[ ] M[x] F[ ] K[x] C[ ] GetTable
- W[x] S[ ] M[x] F[ ] K[x] C[ ] GetTableBucket
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetTableBucketEncryption
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetTableBucketMaintenanceConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetTableBucketMetricsConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetTableBucketPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetTableBucketReplication
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetTableBucketStorageClass
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetTableEncryption
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetTableMaintenanceConfiguration
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] GetTableMaintenanceJobStatus
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetTableMetadataLocation
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetTablePolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetTableRecordExpirationConfiguration
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] GetTableRecordExpirationJobStatus
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetTableReplication
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] GetTableReplicationStatus
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetTableStorageClass
- W[x] S[ ] M[x] F[ ] K[x] C[ ] ListNamespaces
- W[x] S[ ] M[x] F[ ] K[x] C[ ] ListTableBuckets
- W[x] S[ ] M[x] F[ ] K[x] C[ ] ListTables
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutTableBucketEncryption
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutTableBucketMaintenanceConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutTableBucketMetricsConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutTableBucketPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutTableBucketReplication
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutTableBucketStorageClass
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutTableMaintenanceConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutTablePolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutTableRecordExpirationConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutTableReplication
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] RenameTable
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UntagResource
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UpdateTableMetadataLocation

Integration tests: 46/46 implemented operations tested (100.0%)

### winterbaume-s3vectors (s3vectors) - W: 19/19, S: 0/19, M: 15/19, F: 12/19, K: 0/19, C: 0/19

- W[x] S[ ] M[x] F[x] K[ ] C[ ] CreateIndex
- W[x] S[ ] M[x] F[x] K[ ] C[ ] CreateVectorBucket
- W[x] S[ ] M[x] F[x] K[ ] C[ ] DeleteIndex
- W[x] S[ ] M[x] F[x] K[ ] C[ ] DeleteVectorBucket
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteVectorBucketPolicy
- W[x] S[ ] M[x] F[x] K[ ] C[ ] DeleteVectors
- W[x] S[ ] M[x] F[x] K[ ] C[ ] GetIndex
- W[x] S[ ] M[x] F[x] K[ ] C[ ] GetVectorBucket
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetVectorBucketPolicy
- W[x] S[ ] M[x] F[x] K[ ] C[ ] GetVectors
- W[x] S[ ] M[x] F[x] K[ ] C[ ] ListIndexes
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListTagsForResource
- W[x] S[ ] M[x] F[x] K[ ] C[ ] ListVectorBuckets
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListVectors
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] PutVectorBucketPolicy
- W[x] S[ ] M[x] F[x] K[ ] C[ ] PutVectors
- W[x] S[ ] M[ ] F[x] K[ ] C[ ] QueryVectors
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UntagResource

Integration tests: 19/19 implemented operations tested (100.0%)

### winterbaume-sagemaker (sagemaker) - W: 141/396, S: 1/396, M: 112/396, F: 0/396, K: 11/396, C: 0/396

- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AddAssociation
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] AddTags
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] AssociateTrialComponent
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AttachClusterNodeVolume
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] BatchAddClusterNodes
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] BatchDeleteClusterNodes
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] BatchDescribeModelPackage
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] BatchRebootClusterNodes
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] BatchReplaceClusterNodes
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateAIBenchmarkJob
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateAIRecommendationJob
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateAIWorkloadConfig
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateAction
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateAlgorithm
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateApp
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateAppImageConfig
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateArtifact
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateAutoMLJob
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateAutoMLJobV2
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateCluster
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateClusterSchedulerConfig
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateCodeRepository
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateCompilationJob
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateComputeQuota
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateContext
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateDataQualityJobDefinition
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateDeviceFleet
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateDomain
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateEdgeDeploymentPlan
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateEdgeDeploymentStage
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateEdgePackagingJob
- W[x] S[ ] M[x] F[ ] K[x] C[ ] CreateEndpoint
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateEndpointConfig
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateExperiment
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateFeatureGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateFlowDefinition
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateHub
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateHubContentPresignedUrls
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateHubContentReference
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateHumanTaskUi
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateHyperParameterTuningJob
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateImage
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateImageVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateInferenceComponent
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateInferenceExperiment
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateInferenceRecommendationsJob
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateLabelingJob
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateMlflowApp
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateMlflowTrackingServer
- W[x] S[ ] M[x] F[ ] K[x] C[ ] CreateModel
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateModelBiasJobDefinition
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateModelCard
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateModelCardExportJob
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateModelExplainabilityJobDefinition
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateModelPackage
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateModelPackageGroup
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateModelQualityJobDefinition
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateMonitoringSchedule
- W[x] S[ ] M[x] F[ ] K[x] C[ ] CreateNotebookInstance
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateNotebookInstanceLifecycleConfig
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateOptimizationJob
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreatePartnerApp
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreatePartnerAppPresignedUrl
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreatePipeline
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreatePresignedDomainUrl
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreatePresignedMlflowAppUrl
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreatePresignedMlflowTrackingServerUrl
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreatePresignedNotebookInstanceUrl
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateProcessingJob
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateProject
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateSpace
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateStudioLifecycleConfig
- W[x] S[ ] M[x] F[ ] K[x] C[ ] CreateTrainingJob
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateTrainingPlan
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateTransformJob
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateTrial
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateTrialComponent
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateUserProfile
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateWorkforce
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateWorkteam
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteAIBenchmarkJob
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteAIRecommendationJob
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteAIWorkloadConfig
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteAction
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteAlgorithm
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteApp
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteAppImageConfig
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteArtifact
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteAssociation
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteCluster
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteClusterSchedulerConfig
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteCodeRepository
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteCompilationJob
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteComputeQuota
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteContext
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteDataQualityJobDefinition
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteDeviceFleet
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteDomain
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteEdgeDeploymentPlan
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteEdgeDeploymentStage
- W[x] S[ ] M[x] F[ ] K[x] C[ ] DeleteEndpoint
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteEndpointConfig
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteExperiment
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteFeatureGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteFlowDefinition
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteHub
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteHubContent
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteHubContentReference
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteHumanTaskUi
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteHyperParameterTuningJob
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteImage
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteImageVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteInferenceComponent
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteInferenceExperiment
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteMlflowApp
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteMlflowTrackingServer
- W[x] S[ ] M[x] F[ ] K[x] C[ ] DeleteModel
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteModelBiasJobDefinition
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteModelCard
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteModelExplainabilityJobDefinition
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteModelPackage
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteModelPackageGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteModelPackageGroupPolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteModelQualityJobDefinition
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteMonitoringSchedule
- W[x] S[ ] M[x] F[ ] K[x] C[ ] DeleteNotebookInstance
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteNotebookInstanceLifecycleConfig
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteOptimizationJob
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeletePartnerApp
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeletePipeline
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteProcessingJob
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteProject
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteSpace
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteStudioLifecycleConfig
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteTags
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteTrainingJob
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteTrial
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteTrialComponent
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteUserProfile
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteWorkforce
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteWorkteam
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeregisterDevices
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeAIBenchmarkJob
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeAIRecommendationJob
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeAIWorkloadConfig
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeAction
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeAlgorithm
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeApp
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeAppImageConfig
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeArtifact
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeAutoMLJob
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeAutoMLJobV2
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeCluster
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeClusterEvent
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeClusterNode
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeClusterSchedulerConfig
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeCodeRepository
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeCompilationJob
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeComputeQuota
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeContext
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeDataQualityJobDefinition
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeDevice
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeDeviceFleet
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeDomain
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeEdgeDeploymentPlan
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeEdgePackagingJob
- W[x] S[ ] M[x] F[ ] K[x] C[ ] DescribeEndpoint
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeEndpointConfig
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeExperiment
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeFeatureGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeFeatureMetadata
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeFlowDefinition
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeHub
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeHubContent
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeHumanTaskUi
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeHyperParameterTuningJob
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeImage
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeImageVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeInferenceComponent
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeInferenceExperiment
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeInferenceRecommendationsJob
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeLabelingJob
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeLineageGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeMlflowApp
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeMlflowTrackingServer
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeModel
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeModelBiasJobDefinition
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeModelCard
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeModelCardExportJob
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeModelExplainabilityJobDefinition
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeModelPackage
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeModelPackageGroup
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeModelQualityJobDefinition
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeMonitoringSchedule
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] DescribeNotebookInstance
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeNotebookInstanceLifecycleConfig
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeOptimizationJob
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribePartnerApp
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribePipeline
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribePipelineDefinitionForExecution
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribePipelineExecution
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeProcessingJob
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeProject
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeReservedCapacity
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeSpace
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeStudioLifecycleConfig
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeSubscribedWorkteam
- W[x] S[ ] M[x] F[ ] K[x] C[ ] DescribeTrainingJob
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeTrainingPlan
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeTrainingPlanExtensionHistory
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeTransformJob
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeTrial
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeTrialComponent
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeUserProfile
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeWorkforce
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeWorkteam
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DetachClusterNodeVolume
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DisableSagemakerServicecatalogPortfolio
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DisassociateTrialComponent
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] EnableSagemakerServicecatalogPortfolio
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ExtendTrainingPlan
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetDeviceFleetReport
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetLineageGroupPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetModelPackageGroupPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetSagemakerServicecatalogPortfolioStatus
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetScalingConfigurationRecommendation
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetSearchSuggestions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ImportHubContent
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListAIBenchmarkJobs
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListAIRecommendationJobs
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListAIWorkloadConfigs
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListActions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListAlgorithms
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListAliases
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListAppImageConfigs
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListApps
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListArtifacts
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListAssociations
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListAutoMLJobs
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListCandidatesForAutoMLJob
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListClusterEvents
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListClusterNodes
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListClusterSchedulerConfigs
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListClusters
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListCodeRepositories
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListCompilationJobs
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListComputeQuotas
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListContexts
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListDataQualityJobDefinitions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListDeviceFleets
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListDevices
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListDomains
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListEdgeDeploymentPlans
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListEdgePackagingJobs
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListEndpointConfigs
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListEndpoints
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListExperiments
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListFeatureGroups
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListFlowDefinitions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListHubContentVersions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListHubContents
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListHubs
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListHumanTaskUis
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListHyperParameterTuningJobs
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListImageVersions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListImages
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListInferenceComponents
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListInferenceExperiments
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListInferenceRecommendationsJobSteps
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListInferenceRecommendationsJobs
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListLabelingJobs
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListLabelingJobsForWorkteam
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListLineageGroups
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListMlflowApps
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListMlflowTrackingServers
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListModelBiasJobDefinitions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListModelCardExportJobs
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListModelCardVersions
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListModelCards
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListModelExplainabilityJobDefinitions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListModelMetadata
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListModelPackageGroups
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListModelPackages
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListModelQualityJobDefinitions
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListModels
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListMonitoringAlertHistory
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListMonitoringAlerts
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListMonitoringExecutions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListMonitoringSchedules
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListNotebookInstanceLifecycleConfigs
- W[x] S[ ] M[x] F[ ] K[x] C[ ] ListNotebookInstances
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListOptimizationJobs
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListPartnerApps
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListPipelineExecutionSteps
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListPipelineExecutions
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListPipelineParametersForExecution
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListPipelineVersions
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListPipelines
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListProcessingJobs
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListProjects
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListResourceCatalogs
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListSpaces
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListStageDevices
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListStudioLifecycleConfigs
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListSubscribedWorkteams
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListTags
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListTrainingJobs
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListTrainingJobsForHyperParameterTuningJob
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListTrainingPlans
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListTransformJobs
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListTrialComponents
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListTrials
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListUltraServersByReservedCapacity
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListUserProfiles
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListWorkforces
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListWorkteams
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] PutModelPackageGroupPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] QueryLineage
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] RegisterDevices
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] RenderUiTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] RetryPipelineExecution
- W[ ] S[x] M[x] F[ ] K[ ] C[ ] Search
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] SearchTrainingPlanOfferings
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] SendPipelineExecutionStepFailure
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] SendPipelineExecutionStepSuccess
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartClusterHealthCheck
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartEdgeDeploymentStage
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartInferenceExperiment
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartMlflowTrackingServer
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartMonitoringSchedule
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] StartNotebookInstance
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] StartPipelineExecution
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartSession
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StopAIBenchmarkJob
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StopAIRecommendationJob
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] StopAutoMLJob
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StopCompilationJob
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StopEdgeDeploymentStage
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StopEdgePackagingJob
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StopHyperParameterTuningJob
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StopInferenceExperiment
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StopInferenceRecommendationsJob
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StopLabelingJob
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StopMlflowTrackingServer
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StopMonitoringSchedule
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] StopNotebookInstance
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StopOptimizationJob
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StopPipelineExecution
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] StopProcessingJob
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] StopTrainingJob
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] StopTransformJob
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateAction
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateAppImageConfig
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateArtifact
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateCluster
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateClusterSchedulerConfig
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateClusterSoftware
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateCodeRepository
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateComputeQuota
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateContext
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateDeviceFleet
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateDevices
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateDomain
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateEndpoint
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UpdateEndpointWeightsAndCapacities
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateExperiment
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateFeatureGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateFeatureMetadata
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateHub
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateHubContent
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateHubContentReference
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateImage
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateImageVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateInferenceComponent
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateInferenceComponentRuntimeConfig
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateInferenceExperiment
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateMlflowApp
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateMlflowTrackingServer
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UpdateModelCard
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UpdateModelPackage
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateMonitoringAlert
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateMonitoringSchedule
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateNotebookInstance
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateNotebookInstanceLifecycleConfig
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdatePartnerApp
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UpdatePipeline
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdatePipelineExecution
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdatePipelineVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateProject
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateSpace
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateTrainingJob
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateTrial
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UpdateTrialComponent
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateUserProfile
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateWorkforce
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateWorkteam

Integration tests: 112/141 implemented operations tested (79.4%)
Untested implemented operations: 29

### winterbaume-sagemakermetrics (sagemaker-metrics) - W: 2/2, S: 0/2, M: 1/2, F: 0/2, K: 0/2, C: 0/2

- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] BatchGetMetrics
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] BatchPutMetrics

Integration tests: 2/2 implemented operations tested (100.0%)

### winterbaume-sagemakerruntime (sagemaker-runtime) - W: 3/3, S: 0/3, M: 2/3, F: 0/3, K: 0/3, C: 0/3

- W[x] S[ ] M[x] F[ ] K[ ] C[ ] InvokeEndpoint
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] InvokeEndpointAsync
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] InvokeEndpointWithResponseStream

Integration tests: 3/3 implemented operations tested (100.0%)

### winterbaume-savingsplans (savingsplans) - W: 10/10, S: 0/10, M: 0/10, F: 0/10, K: 0/10, C: 0/10

- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateSavingsPlan
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteQueuedSavingsPlan
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeSavingsPlanRates
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeSavingsPlans
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeSavingsPlansOfferingRates
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeSavingsPlansOfferings
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ReturnSavingsPlan
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UntagResource

Integration tests: 8/10 implemented operations tested (80.0%)
Untested implemented operations: 2

### winterbaume-scheduler (scheduler) - W: 12/12, S: 0/12, M: 12/12, F: 0/12, K: 9/12, C: 12/12

Terraform E2E: 7 tests across 2 terraform resource types

Resource types: aws_scheduler_schedule, aws_scheduler_schedule_group

- W[x] S[ ] M[x] F[ ] K[x] C[x] CreateSchedule
- W[x] S[ ] M[x] F[ ] K[x] C[x] CreateScheduleGroup
- W[x] S[ ] M[x] F[ ] K[x] C[x] DeleteSchedule
- W[x] S[ ] M[x] F[ ] K[x] C[x] DeleteScheduleGroup
- W[x] S[ ] M[x] F[ ] K[x] C[x] GetSchedule
- W[x] S[ ] M[x] F[ ] K[x] C[x] GetScheduleGroup
- W[x] S[ ] M[x] F[ ] K[x] C[x] ListScheduleGroups
- W[x] S[ ] M[x] F[ ] K[x] C[x] ListSchedules
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListTagsForResource
- W[x] S[ ] M[x] F[ ] K[ ] C[x] TagResource
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UntagResource
- W[x] S[ ] M[x] F[ ] K[x] C[x] UpdateSchedule

Integration tests: 12/12 implemented operations tested (100.0%)

### winterbaume-secretsmanager (secrets-manager) - W: 22/23, S: 1/23, M: 21/23, F: 0/23, K: 11/23, C: 23/23

Terraform E2E: 3 tests across 2 terraform resource types

Resource types: aws_secretsmanager_secret, aws_secretsmanager_secret_version

- W[x] S[ ] M[x] F[ ] K[ ] C[x] BatchGetSecretValue
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CancelRotateSecret
- W[x] S[ ] M[x] F[ ] K[x] C[x] CreateSecret
- W[x] S[ ] M[x] F[ ] K[x] C[x] DeleteResourcePolicy
- W[x] S[ ] M[x] F[ ] K[x] C[x] DeleteSecret
- W[x] S[ ] M[x] F[ ] K[x] C[x] DescribeSecret
- W[x] S[ ] M[x] F[ ] K[x] C[x] GetRandomPassword
- W[x] S[ ] M[x] F[ ] K[x] C[x] GetResourcePolicy
- W[x] S[ ] M[x] F[ ] K[x] C[x] GetSecretValue
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListSecretVersionIds
- W[x] S[ ] M[x] F[ ] K[x] C[x] ListSecrets
- W[x] S[ ] M[x] F[ ] K[x] C[x] PutResourcePolicy
- W[x] S[ ] M[x] F[ ] K[x] C[x] PutSecretValue
- W[x] S[ ] M[x] F[ ] K[ ] C[x] RemoveRegionsFromReplication
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ReplicateSecretToRegions
- W[x] S[ ] M[x] F[ ] K[ ] C[x] RestoreSecret
- W[x] S[ ] M[x] F[ ] K[ ] C[x] RotateSecret
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] StopReplicationToReplica
- W[x] S[ ] M[x] F[ ] K[ ] C[x] TagResource
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UntagResource
- W[x] S[ ] M[x] F[ ] K[x] C[x] UpdateSecret
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UpdateSecretVersionStage
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] ValidateResourcePolicy

Integration tests: 21/22 implemented operations tested (95.5%)
Untested implemented operations: 1

### winterbaume-securityhub (securityhub) - W: 97/107, S: 10/107, M: 13/107, F: 0/107, K: 0/107, C: 0/107

- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] AcceptAdministratorInvitation
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] AcceptInvitation
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] BatchDeleteAutomationRules
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] BatchDisableStandards
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] BatchEnableStandards
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] BatchGetAutomationRules
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] BatchGetConfigurationPolicyAssociations
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] BatchGetSecurityControls
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] BatchGetStandardsControlAssociations
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] BatchImportFindings
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] BatchUpdateAutomationRules
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] BatchUpdateFindings
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] BatchUpdateFindingsV2
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] BatchUpdateStandardsControlAssociations
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateActionTarget
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateAggregatorV2
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateAutomationRule
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateAutomationRuleV2
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateConfigurationPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateConnectorV2
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateFindingAggregator
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateInsight
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateMembers
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateTicketV2
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] DeclineInvitations
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteActionTarget
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteAggregatorV2
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteAutomationRuleV2
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteConfigurationPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteConnectorV2
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteFindingAggregator
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteInsight
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] DeleteInvitations
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteMembers
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeActionTargets
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeHub
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeOrganizationConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeProducts
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeProductsV2
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeSecurityHubV2
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeStandards
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeStandardsControls
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DisableImportFindingsForProduct
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DisableOrganizationAdminAccount
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DisableSecurityHub
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DisableSecurityHubV2
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] DisassociateFromAdministratorAccount
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] DisassociateFromMasterAccount
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] DisassociateMembers
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] EnableImportFindingsForProduct
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] EnableOrganizationAdminAccount
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] EnableSecurityHub
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] EnableSecurityHubV2
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetAdministratorAccount
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetAggregatorV2
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetAutomationRuleV2
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetConfigurationPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetConfigurationPolicyAssociation
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetConnectorV2
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetEnabledStandards
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetFindingAggregator
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetFindingHistory
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetFindingStatisticsV2
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetFindings
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetFindingsTrendsV2
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetFindingsV2
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetInsightResults
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetInsights
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] GetInvitationsCount
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetMasterAccount
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetMembers
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetResourcesStatisticsV2
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetResourcesTrendsV2
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetResourcesV2
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetSecurityControlDefinition
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] InviteMembers
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListAggregatorsV2
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListAutomationRules
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListAutomationRulesV2
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListConfigurationPolicies
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListConfigurationPolicyAssociations
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListConnectorsV2
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListEnabledProductsForImport
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListFindingAggregators
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] ListInvitations
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListMembers
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListOrganizationAdminAccounts
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListSecurityControlDefinitions
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListStandardsControlAssociations
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] RegisterConnectorV2
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] StartConfigurationPolicyAssociation
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] StartConfigurationPolicyDisassociation
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateActionTarget
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateAggregatorV2
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateAutomationRuleV2
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateConfigurationPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateConnectorV2
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateFindingAggregator
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateFindings
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateInsight
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UpdateOrganizationConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateSecurityControl
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateSecurityHubConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateStandardsControl

Integration tests: 94/97 implemented operations tested (96.9%)
Untested implemented operations: 3

### winterbaume-servicecatalog (service-catalog) - W: 4/90, S: 0/90, M: 0/90, F: 0/90, K: 0/90, C: 0/90

- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AcceptPortfolioShare
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AssociateBudgetWithResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AssociatePrincipalWithPortfolio
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AssociateProductWithPortfolio
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AssociateServiceActionWithProvisioningArtifact
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AssociateTagOptionWithResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] BatchAssociateServiceActionWithProvisioningArtifact
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] BatchDisassociateServiceActionFromProvisioningArtifact
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CopyProduct
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateConstraint
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreatePortfolio
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreatePortfolioShare
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateProduct
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateProvisionedProductPlan
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateProvisioningArtifact
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateServiceAction
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateTagOption
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteConstraint
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeletePortfolio
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeletePortfolioShare
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteProduct
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteProvisionedProductPlan
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteProvisioningArtifact
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteServiceAction
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteTagOption
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeConstraint
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeCopyProductStatus
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribePortfolio
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribePortfolioShareStatus
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribePortfolioShares
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeProduct
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeProductAsAdmin
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeProductView
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeProvisionedProduct
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeProvisionedProductPlan
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeProvisioningArtifact
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeProvisioningParameters
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeRecord
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeServiceAction
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeServiceActionExecutionParameters
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeTagOption
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DisableAWSOrganizationsAccess
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DisassociateBudgetFromResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DisassociatePrincipalFromPortfolio
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DisassociateProductFromPortfolio
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DisassociateServiceActionFromProvisioningArtifact
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DisassociateTagOptionFromResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] EnableAWSOrganizationsAccess
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ExecuteProvisionedProductPlan
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ExecuteProvisionedProductServiceAction
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetAWSOrganizationsAccessStatus
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetProvisionedProductOutputs
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ImportAsProvisionedProduct
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListAcceptedPortfolioShares
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListBudgetsForResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListConstraintsForPortfolio
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListLaunchPaths
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListOrganizationPortfolioAccess
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListPortfolioAccess
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListPortfolios
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListPortfoliosForProduct
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListPrincipalsForPortfolio
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListProvisionedProductPlans
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListProvisioningArtifacts
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListProvisioningArtifactsForServiceAction
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListRecordHistory
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListResourcesForTagOption
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListServiceActions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListServiceActionsForProvisioningArtifact
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListStackInstancesForProvisionedProduct
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListTagOptions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] NotifyProvisionProductEngineWorkflowResult
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] NotifyTerminateProvisionedProductEngineWorkflowResult
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] NotifyUpdateProvisionedProductEngineWorkflowResult
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ProvisionProduct
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] RejectPortfolioShare
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ScanProvisionedProducts
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] SearchProducts
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] SearchProductsAsAdmin
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] SearchProvisionedProducts
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] TerminateProvisionedProduct
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateConstraint
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdatePortfolio
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdatePortfolioShare
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateProduct
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateProvisionedProduct
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateProvisionedProductProperties
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateProvisioningArtifact
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateServiceAction
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateTagOption

Integration tests: 4/4 implemented operations tested (100.0%)

### winterbaume-servicecatalogappregistry (service-catalog-appregistry) - W: 23/24, S: 1/24, M: 0/24, F: 0/24, K: 0/24, C: 0/24

- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] AssociateAttributeGroup
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] AssociateResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateApplication
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateAttributeGroup
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteApplication
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteAttributeGroup
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DisassociateAttributeGroup
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DisassociateResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetApplication
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetAssociatedResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetAttributeGroup
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListApplications
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListAssociatedAttributeGroups
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListAssociatedResources
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListAttributeGroups
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListAttributeGroupsForApplication
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutConfiguration
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] SyncResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateApplication
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateAttributeGroup

Integration tests: 23/23 implemented operations tested (100.0%)

### winterbaume-servicediscovery (servicediscovery) - W: 27/30, S: 0/30, M: 27/30, F: 0/30, K: 0/30, C: 30/30

- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateHttpNamespace
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreatePrivateDnsNamespace
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreatePublicDnsNamespace
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateService
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteNamespace
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteService
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DeleteServiceAttributes
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeregisterInstance
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DiscoverInstances
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DiscoverInstancesRevision
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetInstance
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetInstancesHealthStatus
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetNamespace
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetOperation
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetService
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetServiceAttributes
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListInstances
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListNamespaces
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListOperations
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListServices
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListTagsForResource
- W[x] S[ ] M[x] F[ ] K[ ] C[x] RegisterInstance
- W[x] S[ ] M[x] F[ ] K[ ] C[x] TagResource
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UntagResource
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UpdateHttpNamespace
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UpdateInstanceCustomHealthStatus
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UpdatePrivateDnsNamespace
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UpdatePublicDnsNamespace
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UpdateService
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] UpdateServiceAttributes

Integration tests: 27/27 implemented operations tested (100.0%)

### winterbaume-servicequotas (service-quotas) - W: 5/26, S: 0/26, M: 2/26, F: 0/26, K: 8/26, C: 0/26

- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AssociateServiceQuotaTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateSupportCase
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteServiceQuotaIncreaseRequestFromTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DisassociateServiceQuotaTemplate
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] GetAWSDefaultServiceQuota
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetAssociationForServiceQuotaTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetAutoManagementConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetQuotaUtilizationReport
- W[ ] S[ ] M[ ] F[ ] K[x] C[ ] GetRequestedServiceQuotaChange
- W[x] S[ ] M[x] F[ ] K[x] C[ ] GetServiceQuota
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetServiceQuotaIncreaseRequestFromTemplate
- W[x] S[ ] M[x] F[ ] K[x] C[ ] ListAWSDefaultServiceQuotas
- W[ ] S[ ] M[ ] F[ ] K[x] C[ ] ListRequestedServiceQuotaChangeHistory
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListRequestedServiceQuotaChangeHistoryByQuota
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListServiceQuotaIncreaseRequestsInTemplate
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] ListServiceQuotas
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] ListServices
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListTagsForResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] PutServiceQuotaIncreaseRequestIntoTemplate
- W[ ] S[ ] M[ ] F[ ] K[x] C[ ] RequestServiceQuotaIncrease
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartAutoManagement
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartQuotaUtilizationReport
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StopAutoManagement
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] TagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UntagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateAutoManagement

Integration tests: 5/5 implemented operations tested (100.0%)

### winterbaume-ses (ses) - W: 38/71, S: 2/71, M: 38/71, F: 0/71, K: 6/71, C: 14/71

- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CloneReceiptRuleSet
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateConfigurationSet
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateConfigurationSetEventDestination
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateConfigurationSetTrackingOptions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] CreateCustomVerificationEmailTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateReceiptFilter
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateReceiptRule
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateReceiptRuleSet
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateTemplate
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteConfigurationSet
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DeleteConfigurationSetEventDestination
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteConfigurationSetTrackingOptions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DeleteCustomVerificationEmailTemplate
- W[x] S[ ] M[x] F[ ] K[x] C[ ] DeleteIdentity
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteIdentityPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteReceiptFilter
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteReceiptRule
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteReceiptRuleSet
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteVerifiedEmailAddress
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeActiveReceiptRuleSet
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeConfigurationSet
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeReceiptRule
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeReceiptRuleSet
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetAccountSendingEnabled
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetCustomVerificationEmailTemplate
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetIdentityDkimAttributes
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetIdentityMailFromDomainAttributes
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetIdentityNotificationAttributes
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetIdentityPolicies
- W[x] S[ ] M[x] F[ ] K[x] C[ ] GetIdentityVerificationAttributes
- W[ ] S[x] M[x] F[ ] K[ ] C[ ] GetSendQuota
- W[ ] S[x] M[x] F[ ] K[ ] C[ ] GetSendStatistics
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetTemplate
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListConfigurationSets
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ListCustomVerificationEmailTemplates
- W[x] S[ ] M[x] F[ ] K[x] C[ ] ListIdentities
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListIdentityPolicies
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListReceiptFilters
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListReceiptRuleSets
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListTemplates
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListVerifiedEmailAddresses
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] PutConfigurationSetDeliveryOptions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] PutIdentityPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ReorderReceiptRuleSet
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] SendBounce
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] SendBulkTemplatedEmail
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] SendCustomVerificationEmail
- W[x] S[ ] M[x] F[ ] K[x] C[x] SendEmail
- W[x] S[ ] M[x] F[ ] K[x] C[ ] SendRawEmail
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] SendTemplatedEmail
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] SetActiveReceiptRuleSet
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] SetIdentityDkimEnabled
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] SetIdentityFeedbackForwardingEnabled
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] SetIdentityHeadersInNotificationsEnabled
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] SetIdentityMailFromDomain
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] SetIdentityNotificationTopic
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] SetReceiptRulePosition
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] TestRenderTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateAccountSendingEnabled
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] UpdateConfigurationSetEventDestination
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UpdateConfigurationSetReputationMetricsEnabled
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateConfigurationSetSendingEnabled
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateConfigurationSetTrackingOptions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] UpdateCustomVerificationEmailTemplate
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UpdateReceiptRule
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UpdateTemplate
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] VerifyDomainDkim
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] VerifyDomainIdentity
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] VerifyEmailAddress
- W[x] S[ ] M[x] F[ ] K[x] C[ ] VerifyEmailIdentity

Integration tests: 28/38 implemented operations tested (73.7%)
Untested implemented operations: 10

### winterbaume-sesv2 (sesv2) - W: 106/110, S: 4/110, M: 30/110, F: 0/110, K: 15/110, C: 0/110

- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] BatchGetMetricData
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CancelExportJob
- W[x] S[ ] M[x] F[ ] K[x] C[ ] CreateConfigurationSet
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateConfigurationSetEventDestination
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateContact
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateContactList
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateCustomVerificationEmailTemplate
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateDedicatedIpPool
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateDeliverabilityTestReport
- W[x] S[ ] M[x] F[ ] K[x] C[ ] CreateEmailIdentity
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateEmailIdentityPolicy
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] CreateEmailTemplate
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateExportJob
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateImportJob
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateMultiRegionEndpoint
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateTenant
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateTenantResourceAssociation
- W[x] S[ ] M[x] F[ ] K[x] C[ ] DeleteConfigurationSet
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteConfigurationSetEventDestination
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteContact
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteContactList
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteCustomVerificationEmailTemplate
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteDedicatedIpPool
- W[x] S[ ] M[x] F[ ] K[x] C[ ] DeleteEmailIdentity
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteEmailIdentityPolicy
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] DeleteEmailTemplate
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteMultiRegionEndpoint
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteSuppressedDestination
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteTenant
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteTenantResourceAssociation
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetAccount
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetBlacklistReports
- W[x] S[ ] M[x] F[ ] K[x] C[ ] GetConfigurationSet
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetConfigurationSetEventDestinations
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetContact
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetContactList
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetCustomVerificationEmailTemplate
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetDedicatedIp
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetDedicatedIpPool
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetDedicatedIps
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetDeliverabilityDashboardOptions
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetDeliverabilityTestReport
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetDomainDeliverabilityCampaign
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] GetDomainStatisticsReport
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] GetEmailAddressInsights
- W[x] S[ ] M[x] F[ ] K[x] C[ ] GetEmailIdentity
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetEmailIdentityPolicies
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] GetEmailTemplate
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetExportJob
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetImportJob
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetMessageInsights
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetMultiRegionEndpoint
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetReputationEntity
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetSuppressedDestination
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetTenant
- W[x] S[ ] M[x] F[ ] K[x] C[ ] ListConfigurationSets
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListContactLists
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListContacts
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListCustomVerificationEmailTemplates
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListDedicatedIpPools
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListDeliverabilityTestReports
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListDomainDeliverabilityCampaigns
- W[x] S[ ] M[x] F[ ] K[x] C[ ] ListEmailIdentities
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] ListEmailTemplates
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListExportJobs
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListImportJobs
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListMultiRegionEndpoints
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] ListRecommendations
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListReputationEntities
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListResourceTenants
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListSuppressedDestinations
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListTenantResources
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListTenants
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutAccountDedicatedIpWarmupAttributes
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutAccountDetails
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutAccountSendingAttributes
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutAccountSuppressionAttributes
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutAccountVdmAttributes
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutConfigurationSetArchivingOptions
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutConfigurationSetDeliveryOptions
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutConfigurationSetReputationOptions
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutConfigurationSetSendingOptions
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutConfigurationSetSuppressionOptions
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutConfigurationSetTrackingOptions
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutConfigurationSetVdmOptions
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutDedicatedIpInPool
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutDedicatedIpPoolScalingAttributes
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutDedicatedIpWarmupAttributes
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutDeliverabilityDashboardOption
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutEmailIdentityConfigurationSetAttributes
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutEmailIdentityDkimAttributes
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutEmailIdentityDkimSigningAttributes
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutEmailIdentityFeedbackAttributes
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutEmailIdentityMailFromAttributes
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutSuppressedDestination
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] SendBulkEmail
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] SendCustomVerificationEmail
- W[x] S[ ] M[x] F[ ] K[x] C[ ] SendEmail
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] TestRenderEmailTemplate
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateConfigurationSetEventDestination
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UpdateContact
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UpdateContactList
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateCustomVerificationEmailTemplate
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UpdateEmailIdentityPolicy
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] UpdateEmailTemplate
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateReputationEntityCustomerManagedStatus
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateReputationEntityPolicy

Integration tests: 91/106 implemented operations tested (85.8%)
Untested implemented operations: 15

### winterbaume-sfn (sfn) - W: 35/37, S: 2/37, M: 29/37, F: 0/37, K: 18/37, C: 37/37

Terraform E2E: 10 tests across 3 terraform resource types

Resource types: aws_sfn_activity, aws_sfn_alias, aws_sfn_state_machine

- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateActivity
- W[x] S[ ] M[x] F[ ] K[x] C[x] CreateStateMachine
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateStateMachineAlias
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteActivity
- W[x] S[ ] M[x] F[ ] K[x] C[x] DeleteStateMachine
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteStateMachineAlias
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteStateMachineVersion
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeActivity
- W[x] S[ ] M[x] F[ ] K[x] C[x] DescribeExecution
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeMapRun
- W[x] S[ ] M[x] F[ ] K[x] C[x] DescribeStateMachine
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeStateMachineAlias
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeStateMachineForExecution
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetActivityTask
- W[x] S[ ] M[x] F[ ] K[x] C[x] GetExecutionHistory
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListActivities
- W[x] S[ ] M[x] F[ ] K[x] C[x] ListExecutions
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListMapRuns
- W[x] S[ ] M[x] F[ ] K[x] C[x] ListStateMachineAliases
- W[x] S[ ] M[ ] F[ ] K[x] C[x] ListStateMachineVersions
- W[x] S[ ] M[x] F[ ] K[x] C[x] ListStateMachines
- W[x] S[ ] M[x] F[ ] K[x] C[x] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] PublishStateMachineVersion
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] RedriveExecution
- W[x] S[ ] M[x] F[ ] K[x] C[x] SendTaskFailure
- W[x] S[ ] M[x] F[ ] K[x] C[x] SendTaskHeartbeat
- W[x] S[ ] M[x] F[ ] K[x] C[x] SendTaskSuccess
- W[x] S[ ] M[x] F[ ] K[x] C[x] StartExecution
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] StartSyncExecution
- W[x] S[ ] M[x] F[ ] K[x] C[x] StopExecution
- W[x] S[ ] M[x] F[ ] K[x] C[x] TagResource
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] TestState
- W[x] S[ ] M[x] F[ ] K[x] C[x] UntagResource
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UpdateMapRun
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UpdateStateMachine
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UpdateStateMachineAlias
- W[x] S[ ] M[ ] F[ ] K[x] C[x] ValidateStateMachineDefinition

Integration tests: 35/35 implemented operations tested (100.0%)

### winterbaume-shield (shield) - W: 9/36, S: 0/36, M: 9/36, F: 0/36, K: 0/36, C: 0/36

- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AssociateDRTLogBucket
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AssociateDRTRole
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AssociateHealthCheck
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AssociateProactiveEngagementDetails
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateProtection
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateProtectionGroup
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateSubscription
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteProtection
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteProtectionGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteSubscription
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeAttack
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeAttackStatistics
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeDRTAccess
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeEmergencyContactSettings
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeProtection
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeProtectionGroup
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeSubscription
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DisableApplicationLayerAutomaticResponse
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DisableProactiveEngagement
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DisassociateDRTLogBucket
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DisassociateDRTRole
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DisassociateHealthCheck
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] EnableApplicationLayerAutomaticResponse
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] EnableProactiveEngagement
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetSubscriptionState
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListAttacks
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListProtectionGroups
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListProtections
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListResourcesInProtectionGroup
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListTagsForResource
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] TagResource
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UntagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateApplicationLayerAutomaticResponse
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateEmergencyContactSettings
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateProtectionGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateSubscription

Integration tests: 9/9 implemented operations tested (100.0%)

### winterbaume-signer (signer) - W: 19/19, S: 0/19, M: 7/19, F: 0/19, K: 0/19, C: 0/19

- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] AddProfilePermission
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CancelSigningProfile
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeSigningJob
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetRevocationStatus
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetSigningPlatform
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetSigningProfile
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListProfilePermissions
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListSigningJobs
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListSigningPlatforms
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListSigningProfiles
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListTagsForResource
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] PutSigningProfile
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] RemoveProfilePermission
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] RevokeSignature
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] RevokeSigningProfile
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] SignPayload
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] StartSigningJob
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] TagResource
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UntagResource

Integration tests: 19/19 implemented operations tested (100.0%)

### winterbaume-simpledbv2 (simpledbv2) - W: 3/3, S: 0/3, M: 0/3, F: 0/3, K: 0/3, C: 0/3

- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetExport
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListExports
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] StartDomainExport

Integration tests: 3/3 implemented operations tested (100.0%)

### winterbaume-simspaceweaver (simspaceweaver) - W: 15/16, S: 0/16, M: 0/16, F: 0/16, K: 0/16, C: 0/16

- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateSnapshot
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteApp
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteSimulation
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeApp
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeSimulation
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListApps
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListSimulations
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] StartApp
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] StartClock
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] StartSimulation
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] StopApp
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] StopClock
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] StopSimulation
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UntagResource

Integration tests: 11/15 implemented operations tested (73.3%)
Untested implemented operations: 4

### winterbaume-snowdevicemanagement (snow-device-management) - W: 11/13, S: 0/13, M: 0/13, F: 0/13, K: 0/13, C: 0/13

- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CancelTask
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateTask
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeDevice
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeDeviceEc2Instances
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeExecution
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeTask
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListDeviceResources
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListDevices
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListExecutions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListTasks
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UntagResource

Integration tests: 7/11 implemented operations tested (63.6%)
Untested implemented operations: 4

### winterbaume-sns (sns) - W: 41/42, S: 1/42, M: 33/42, F: 0/42, K: 15/42, C: 42/42

Terraform E2E: 4 tests across 2 terraform resource types

Resource types: aws_sns_topic, aws_sns_topic_subscription

- W[x] S[ ] M[x] F[ ] K[ ] C[x] AddPermission
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CheckIfPhoneNumberIsOptedOut
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ConfirmSubscription
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreatePlatformApplication
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreatePlatformEndpoint
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateSMSSandboxPhoneNumber
- W[x] S[ ] M[x] F[ ] K[x] C[x] CreateTopic
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteEndpoint
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeletePlatformApplication
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteSMSSandboxPhoneNumber
- W[x] S[ ] M[x] F[ ] K[x] C[x] DeleteTopic
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetDataProtectionPolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetEndpointAttributes
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetPlatformApplicationAttributes
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetSMSAttributes
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetSMSSandboxAccountStatus
- W[x] S[ ] M[x] F[ ] K[x] C[x] GetSubscriptionAttributes
- W[x] S[ ] M[x] F[ ] K[x] C[x] GetTopicAttributes
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListEndpointsByPlatformApplication
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] ListOriginationNumbers
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListPhoneNumbersOptedOut
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListPlatformApplications
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListSMSSandboxPhoneNumbers
- W[x] S[ ] M[x] F[ ] K[x] C[x] ListSubscriptions
- W[x] S[ ] M[x] F[ ] K[x] C[x] ListSubscriptionsByTopic
- W[x] S[ ] M[x] F[ ] K[x] C[x] ListTagsForResource
- W[x] S[ ] M[x] F[ ] K[x] C[x] ListTopics
- W[x] S[ ] M[x] F[ ] K[ ] C[x] OptInPhoneNumber
- W[x] S[ ] M[x] F[ ] K[x] C[x] Publish
- W[x] S[ ] M[x] F[ ] K[ ] C[x] PublishBatch
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] PutDataProtectionPolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[x] RemovePermission
- W[x] S[ ] M[x] F[ ] K[ ] C[x] SetEndpointAttributes
- W[x] S[ ] M[x] F[ ] K[ ] C[x] SetPlatformApplicationAttributes
- W[x] S[ ] M[x] F[ ] K[ ] C[x] SetSMSAttributes
- W[x] S[ ] M[x] F[ ] K[x] C[x] SetSubscriptionAttributes
- W[x] S[ ] M[ ] F[ ] K[x] C[x] SetTopicAttributes
- W[x] S[ ] M[x] F[ ] K[x] C[x] Subscribe
- W[x] S[ ] M[x] F[ ] K[x] C[x] TagResource
- W[x] S[ ] M[x] F[ ] K[x] C[x] Unsubscribe
- W[x] S[ ] M[x] F[ ] K[x] C[x] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] VerifySMSSandboxPhoneNumber

Integration tests: 36/41 implemented operations tested (87.8%)
Untested implemented operations: 5

### winterbaume-sqs (sqs) - W: 23/23, S: 0/23, M: 20/23, F: 0/23, K: 16/23, C: 23/23

Terraform E2E: 10 tests across 2 terraform resource types

Resource types: aws_sqs_queue, aws_sqs_queue_policy

- W[x] S[ ] M[x] F[ ] K[ ] C[x] AddPermission
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CancelMessageMoveTask
- W[x] S[ ] M[x] F[ ] K[x] C[x] ChangeMessageVisibility
- W[x] S[ ] M[x] F[ ] K[x] C[x] ChangeMessageVisibilityBatch
- W[x] S[ ] M[x] F[ ] K[x] C[x] CreateQueue
- W[x] S[ ] M[x] F[ ] K[x] C[x] DeleteMessage
- W[x] S[ ] M[x] F[ ] K[x] C[x] DeleteMessageBatch
- W[x] S[ ] M[x] F[ ] K[x] C[x] DeleteQueue
- W[x] S[ ] M[x] F[ ] K[x] C[x] GetQueueAttributes
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetQueueUrl
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListDeadLetterSourceQueues
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListMessageMoveTasks
- W[x] S[ ] M[x] F[ ] K[x] C[x] ListQueueTags
- W[x] S[ ] M[x] F[ ] K[x] C[x] ListQueues
- W[x] S[ ] M[x] F[ ] K[x] C[x] PurgeQueue
- W[x] S[ ] M[x] F[ ] K[x] C[x] ReceiveMessage
- W[x] S[ ] M[x] F[ ] K[ ] C[x] RemovePermission
- W[x] S[ ] M[x] F[ ] K[x] C[x] SendMessage
- W[x] S[ ] M[x] F[ ] K[x] C[x] SendMessageBatch
- W[x] S[ ] M[x] F[ ] K[x] C[x] SetQueueAttributes
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] StartMessageMoveTask
- W[x] S[ ] M[x] F[ ] K[x] C[x] TagQueue
- W[x] S[ ] M[x] F[ ] K[x] C[x] UntagQueue

Integration tests: 22/23 implemented operations tested (95.7%)
Untested implemented operations: 1

### winterbaume-ssm (ssm) - W: 127/146, S: 19/146, M: 41/146, F: 0/146, K: 10/146, C: 146/146

Terraform E2E: 12 tests across 6 terraform resource types

Resource types: aws_ssm_association, aws_ssm_document, aws_ssm_maintenance_window, aws_ssm_maintenance_window_target, aws_ssm_parameter, aws_ssm_patch_baseline

- W[x] S[ ] M[x] F[ ] K[x] C[x] AddTagsToResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] AssociateOpsItemRelatedItem
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CancelCommand
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CancelMaintenanceWindowExecution
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateActivation
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateAssociation
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateAssociationBatch
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateDocument
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateMaintenanceWindow
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateOpsItem
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateOpsMetadata
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreatePatchBaseline
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateResourceDataSync
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteActivation
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteAssociation
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteDocument
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteInventory
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteMaintenanceWindow
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteOpsItem
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteOpsMetadata
- W[x] S[ ] M[x] F[ ] K[x] C[x] DeleteParameter
- W[x] S[ ] M[x] F[ ] K[x] C[x] DeleteParameters
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeletePatchBaseline
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteResourceDataSync
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteResourcePolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeregisterManagedInstance
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeregisterPatchBaselineForPatchGroup
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeregisterTargetFromMaintenanceWindow
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeregisterTaskFromMaintenanceWindow
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeActivations
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeAssociation
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeAssociationExecutionTargets
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeAssociationExecutions
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] DescribeAutomationExecutions
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] DescribeAutomationStepExecutions
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] DescribeAvailablePatches
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeDocument
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeDocumentPermission
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] DescribeEffectiveInstanceAssociations
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeEffectivePatchesForPatchBaseline
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] DescribeInstanceAssociationsStatus
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeInstanceInformation
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] DescribeInstancePatchStates
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] DescribeInstancePatchStatesForPatchGroup
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] DescribeInstancePatches
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeInstanceProperties
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeInventoryDeletions
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeMaintenanceWindowExecutionTaskInvocations
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeMaintenanceWindowExecutionTasks
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeMaintenanceWindowExecutions
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeMaintenanceWindowSchedule
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeMaintenanceWindowTargets
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeMaintenanceWindowTasks
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeMaintenanceWindows
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeMaintenanceWindowsForTarget
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeOpsItems
- W[x] S[ ] M[x] F[ ] K[x] C[x] DescribeParameters
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribePatchBaselines
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribePatchGroupState
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribePatchGroups
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] DescribePatchProperties
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DescribeSessions
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DisassociateOpsItemRelatedItem
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] GetAccessToken
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] GetAutomationExecution
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] GetCalendarState
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetCommandInvocation
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetConnectionStatus
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetDefaultPatchBaseline
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetDeployablePatchSnapshotForInstance
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetDocument
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] GetExecutionPreview
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetInventory
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetInventorySchema
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetMaintenanceWindow
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetMaintenanceWindowExecution
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetMaintenanceWindowExecutionTask
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetMaintenanceWindowExecutionTaskInvocation
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetMaintenanceWindowTask
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetOpsItem
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetOpsMetadata
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetOpsSummary
- W[x] S[ ] M[x] F[ ] K[x] C[x] GetParameter
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetParameterHistory
- W[x] S[ ] M[x] F[ ] K[x] C[x] GetParameters
- W[x] S[ ] M[x] F[ ] K[x] C[x] GetParametersByPath
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetPatchBaseline
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetPatchBaselineForPatchGroup
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetResourcePolicies
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetServiceSetting
- W[x] S[ ] M[x] F[ ] K[ ] C[x] LabelParameterVersion
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListAssociationVersions
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListAssociations
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListCommandInvocations
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListCommands
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListComplianceItems
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListComplianceSummaries
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListDocumentMetadataHistory
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListDocumentVersions
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListDocuments
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListInventoryEntries
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListNodes
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListNodesSummary
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListOpsItemEvents
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListOpsItemRelatedItems
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListOpsMetadata
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListResourceComplianceSummaries
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListResourceDataSync
- W[x] S[ ] M[x] F[ ] K[x] C[x] ListTagsForResource
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ModifyDocumentPermission
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] PutComplianceItems
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] PutInventory
- W[x] S[ ] M[x] F[ ] K[x] C[x] PutParameter
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] PutResourcePolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] RegisterDefaultPatchBaseline
- W[x] S[ ] M[x] F[ ] K[ ] C[x] RegisterPatchBaselineForPatchGroup
- W[x] S[ ] M[x] F[ ] K[ ] C[x] RegisterTargetWithMaintenanceWindow
- W[x] S[ ] M[x] F[ ] K[ ] C[x] RegisterTaskWithMaintenanceWindow
- W[x] S[ ] M[x] F[ ] K[x] C[x] RemoveTagsFromResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ResetServiceSetting
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ResumeSession
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] SendAutomationSignal
- W[x] S[ ] M[x] F[ ] K[ ] C[x] SendCommand
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] StartAccessRequest
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] StartAssociationsOnce
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] StartAutomationExecution
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] StartChangeRequestExecution
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] StartExecutionPreview
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] StartSession
- W[ ] S[x] M[ ] F[ ] K[ ] C[x] StopAutomationExecution
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] TerminateSession
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UnlabelParameterVersion
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateAssociation
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateAssociationStatus
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UpdateDocument
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UpdateDocumentDefaultVersion
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateDocumentMetadata
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateMaintenanceWindow
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateMaintenanceWindowTarget
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateMaintenanceWindowTask
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateManagedInstanceRole
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateOpsItem
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateOpsMetadata
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdatePatchBaseline
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateResourceDataSync
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UpdateServiceSetting

Integration tests: 125/127 implemented operations tested (98.4%)
Untested implemented operations: 2

### winterbaume-ssmquicksetup (ssm-quicksetup) - W: 6/14, S: 0/14, M: 0/14, F: 0/14, K: 0/14, C: 0/14

- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateConfigurationManager
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteConfigurationManager
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetConfigurationManager
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetServiceSettings
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListConfigurationManagers
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListConfigurations
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListQuickSetupTypes
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UntagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateConfigurationDefinition
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateConfigurationManager
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateServiceSettings

Integration tests: 2/6 implemented operations tested (33.3%)
Untested implemented operations: 4

### winterbaume-sso (sso) - W: 4/4, S: 0/4, M: 0/4, F: 0/4, K: 0/4, C: 0/4

- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetRoleCredentials
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListAccountRoles
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListAccounts
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] Logout

Integration tests: 4/4 implemented operations tested (100.0%)

### winterbaume-ssoadmin (sso-admin) - W: 27/79, S: 1/79, M: 25/79, F: 0/79, K: 0/79, C: 79/79

- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] AddRegion
- W[x] S[ ] M[x] F[ ] K[ ] C[x] AttachCustomerManagedPolicyReferenceToPermissionSet
- W[x] S[ ] M[x] F[ ] K[ ] C[x] AttachManagedPolicyToPermissionSet
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateAccountAssignment
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] CreateApplication
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] CreateApplicationAssignment
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] CreateInstance
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] CreateInstanceAccessControlAttributeConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreatePermissionSet
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] CreateTrustedTokenIssuer
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteAccountAssignment
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DeleteApplication
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DeleteApplicationAccessScope
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DeleteApplicationAssignment
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DeleteApplicationAuthenticationMethod
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DeleteApplicationGrant
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteInlinePolicyFromPermissionSet
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DeleteInstance
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DeleteInstanceAccessControlAttributeConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeletePermissionSet
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DeletePermissionsBoundaryFromPermissionSet
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DeleteTrustedTokenIssuer
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeAccountAssignmentCreationStatus
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribeAccountAssignmentDeletionStatus
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DescribeApplication
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DescribeApplicationAssignment
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DescribeApplicationProvider
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DescribeInstance
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DescribeInstanceAccessControlAttributeConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DescribePermissionSet
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DescribePermissionSetProvisioningStatus
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DescribeRegion
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DescribeTrustedTokenIssuer
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DetachCustomerManagedPolicyReferenceFromPermissionSet
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DetachManagedPolicyFromPermissionSet
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetApplicationAccessScope
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetApplicationAssignmentConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetApplicationAuthenticationMethod
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetApplicationGrant
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetApplicationSessionConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetInlinePolicyForPermissionSet
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetPermissionsBoundaryForPermissionSet
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ListAccountAssignmentCreationStatus
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ListAccountAssignmentDeletionStatus
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListAccountAssignments
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListAccountAssignmentsForPrincipal
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListAccountsForProvisionedPermissionSet
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ListApplicationAccessScopes
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ListApplicationAssignments
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ListApplicationAssignmentsForPrincipal
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ListApplicationAuthenticationMethods
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ListApplicationGrants
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ListApplicationProviders
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ListApplications
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListCustomerManagedPolicyReferencesInPermissionSet
- W[ ] S[x] M[x] F[ ] K[ ] C[x] ListInstances
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListManagedPoliciesInPermissionSet
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ListPermissionSetProvisioningStatus
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListPermissionSets
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListPermissionSetsProvisionedToAccount
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ListRegions
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListTagsForResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ListTrustedTokenIssuers
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ProvisionPermissionSet
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] PutApplicationAccessScope
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] PutApplicationAssignmentConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] PutApplicationAuthenticationMethod
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] PutApplicationGrant
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] PutApplicationSessionConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] C[x] PutInlinePolicyToPermissionSet
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] PutPermissionsBoundaryToPermissionSet
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] RemoveRegion
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] UntagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] UpdateApplication
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UpdateInstance
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] UpdateInstanceAccessControlAttributeConfiguration
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UpdatePermissionSet
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] UpdateTrustedTokenIssuer

Integration tests: 23/27 implemented operations tested (85.2%)
Untested implemented operations: 4

### winterbaume-sts (sts) - W: 11/11, S: 0/11, M: 7/11, F: 0/11, K: 6/11, C: 11/11

- W[x] S[ ] M[x] F[ ] K[x] C[x] AssumeRole
- W[x] S[ ] M[x] F[ ] K[x] C[x] AssumeRoleWithSAML
- W[x] S[ ] M[x] F[ ] K[x] C[x] AssumeRoleWithWebIdentity
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] AssumeRoot
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DecodeAuthorizationMessage
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetAccessKeyInfo
- W[x] S[ ] M[x] F[ ] K[x] C[x] GetCallerIdentity
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetDelegatedAccessToken
- W[x] S[ ] M[x] F[ ] K[x] C[x] GetFederationToken
- W[x] S[ ] M[x] F[ ] K[x] C[x] GetSessionToken
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetWebIdentityToken

Integration tests: 10/11 implemented operations tested (90.9%)
Untested implemented operations: 1

### winterbaume-support (support) - W: 5/16, S: 1/16, M: 5/16, F: 0/16, K: 0/16, C: 0/16

- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AddAttachmentsToSet
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AddCommunicationToCase
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateCase
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeAttachment
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeCases
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeCommunications
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeCreateCaseOptions
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeServices
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeSeverityLevels
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeSupportedLanguages
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeTrustedAdvisorCheckRefreshStatuses
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeTrustedAdvisorCheckResult
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeTrustedAdvisorCheckSummaries
- W[ ] S[x] M[x] F[ ] K[ ] C[ ] DescribeTrustedAdvisorChecks
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] RefreshTrustedAdvisorCheck
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ResolveCase

Integration tests: 5/5 implemented operations tested (100.0%)

### winterbaume-supportapp (support-app) - W: 3/10, S: 0/10, M: 0/10, F: 0/10, K: 0/10, C: 0/10

- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateSlackChannelConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteAccountAlias
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteSlackChannelConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteSlackWorkspaceConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetAccountAlias
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListSlackChannelConfigurations
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListSlackWorkspaceConfigurations
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutAccountAlias
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] RegisterSlackWorkspaceForOrganization
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateSlackChannelConfiguration

Integration tests: 3/3 implemented operations tested (100.0%)

### winterbaume-swf (swf) - W: 30/39, S: 0/39, M: 21/39, F: 0/39, K: 0/39, C: 0/39

- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CountClosedWorkflowExecutions
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CountOpenWorkflowExecutions
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CountPendingActivityTasks
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CountPendingDecisionTasks
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteActivityType
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteWorkflowType
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeprecateActivityType
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeprecateDomain
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeprecateWorkflowType
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeActivityType
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeDomain
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeWorkflowExecution
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeWorkflowType
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetWorkflowExecutionHistory
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListActivityTypes
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListClosedWorkflowExecutions
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListDomains
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListOpenWorkflowExecutions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListWorkflowTypes
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] PollForActivityTask
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] PollForDecisionTask
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] RecordActivityTaskHeartbeat
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] RegisterActivityType
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] RegisterDomain
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] RegisterWorkflowType
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] RequestCancelWorkflowExecution
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] RespondActivityTaskCanceled
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] RespondActivityTaskCompleted
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] RespondActivityTaskFailed
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] RespondDecisionTaskCompleted
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] SignalWorkflowExecution
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] StartWorkflowExecution
- W[ ] S[ ] M[x] F[ ] K[ ] C[ ] TagResource
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] TerminateWorkflowExecution
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UndeprecateActivityType
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UndeprecateDomain
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UndeprecateWorkflowType
- W[ ] S[ ] M[x] F[ ] K[ ] C[ ] UntagResource

Integration tests: 30/30 implemented operations tested (100.0%)

### winterbaume-synthetics (synthetics) - W: 22/22, S: 0/22, M: 4/22, F: 0/22, K: 0/22, C: 0/22

- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] AssociateResource
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateCanary
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateGroup
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteCanary
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteGroup
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeCanaries
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeCanariesLastRun
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeRuntimeVersions
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DisassociateResource
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetCanary
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetCanaryRuns
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetGroup
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListAssociatedGroups
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListGroupResources
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListGroups
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] StartCanary
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] StartCanaryDryRun
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] StopCanary
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateCanary

Integration tests: 22/22 implemented operations tested (100.0%)

### winterbaume-taxsettings (taxsettings) - W: 0/16, S: 0/16, M: 0/16, F: 0/16, K: 0/16, C: 0/16

- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] BatchDeleteTaxRegistration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] BatchGetTaxExemptions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] BatchPutTaxRegistration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteSupplementalTaxRegistration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteTaxRegistration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetTaxExemptionTypes
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetTaxInheritance
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetTaxRegistration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetTaxRegistrationDocument
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListSupplementalTaxRegistrations
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListTaxExemptions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListTaxRegistrations
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] PutSupplementalTaxRegistration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] PutTaxExemption
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] PutTaxInheritance
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] PutTaxRegistration

Integration tests: 0/0 implemented operations tested (0.0%)

### winterbaume-textract (textract) - W: 6/25, S: 0/25, M: 5/25, F: 0/25, K: 0/25, C: 0/25

- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] AnalyzeDocument
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AnalyzeExpense
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AnalyzeID
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateAdapter
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateAdapterVersion
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteAdapter
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteAdapterVersion
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DetectDocumentText
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetAdapter
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetAdapterVersion
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetDocumentAnalysis
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetDocumentTextDetection
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetExpenseAnalysis
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetLendingAnalysis
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetLendingAnalysisSummary
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListAdapterVersions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListAdapters
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListTagsForResource
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] StartDocumentAnalysis
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] StartDocumentTextDetection
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartExpenseAnalysis
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartLendingAnalysis
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] TagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UntagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateAdapter

Integration tests: 6/6 implemented operations tested (100.0%)

### winterbaume-timestreaminfluxdb (timestream-influxdb) - W: 19/19, S: 0/19, M: 13/19, F: 0/19, K: 0/19, C: 0/19

- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateDbCluster
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateDbInstance
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateDbParameterGroup
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteDbCluster
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteDbInstance
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetDbCluster
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetDbInstance
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetDbParameterGroup
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListDbClusters
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListDbInstances
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListDbInstancesForCluster
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListDbParameterGroups
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] RebootDbCluster
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] RebootDbInstance
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] TagResource
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateDbCluster
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateDbInstance

Integration tests: 13/19 implemented operations tested (68.4%)
Untested implemented operations: 6

### winterbaume-timestreamquery (timestream-query) - W: 15/15, S: 0/15, M: 6/15, F: 0/15, K: 0/15, C: 0/15

- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CancelQuery
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateScheduledQuery
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteScheduledQuery
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeAccountSettings
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeEndpoints
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeScheduledQuery
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ExecuteScheduledQuery
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListScheduledQueries
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PrepareQuery
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] Query
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateAccountSettings
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UpdateScheduledQuery

Integration tests: 15/15 implemented operations tested (100.0%)

### winterbaume-timestreamwrite (timestream-write) - W: 19/19, S: 0/19, M: 15/19, F: 0/19, K: 0/19, C: 0/19

- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateBatchLoadTask
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateDatabase
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateTable
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteDatabase
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteTable
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeBatchLoadTask
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeDatabase
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeEndpoints
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeTable
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListBatchLoadTasks
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListDatabases
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListTables
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ResumeBatchLoadTask
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] TagResource
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UntagResource
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UpdateDatabase
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UpdateTable
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] WriteRecords

Integration tests: 15/19 implemented operations tested (78.9%)
Untested implemented operations: 4

### winterbaume-transcribe (transcribe) - W: 16/43, S: 0/43, M: 16/43, F: 0/43, K: 0/43, C: 0/43

- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateCallAnalyticsCategory
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateLanguageModel
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateMedicalVocabulary
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateVocabulary
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateVocabularyFilter
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteCallAnalyticsCategory
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteCallAnalyticsJob
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteLanguageModel
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteMedicalScribeJob
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteMedicalTranscriptionJob
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteMedicalVocabulary
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteTranscriptionJob
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteVocabulary
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteVocabularyFilter
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeLanguageModel
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetCallAnalyticsCategory
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetCallAnalyticsJob
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetMedicalScribeJob
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetMedicalTranscriptionJob
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetMedicalVocabulary
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetTranscriptionJob
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetVocabulary
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetVocabularyFilter
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListCallAnalyticsCategories
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListCallAnalyticsJobs
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListLanguageModels
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListMedicalScribeJobs
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListMedicalTranscriptionJobs
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListMedicalVocabularies
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListTagsForResource
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListTranscriptionJobs
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListVocabularies
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListVocabularyFilters
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartCallAnalyticsJob
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartMedicalScribeJob
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] StartMedicalTranscriptionJob
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] StartTranscriptionJob
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] TagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UntagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateCallAnalyticsCategory
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateMedicalVocabulary
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateVocabulary
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateVocabularyFilter

Integration tests: 16/16 implemented operations tested (100.0%)

### winterbaume-transfer (transfer) - W: 44/71, S: 0/71, M: 14/71, F: 0/71, K: 0/71, C: 0/71

Terraform E2E: 8 tests across 4 terraform resource types

Resource types: aws_transfer_connector, aws_transfer_server, aws_transfer_user, aws_transfer_workflow

- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateAccess
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateAgreement
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateConnector
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateProfile
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateServer
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateUser
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateWebApp
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateWorkflow
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteAccess
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteAgreement
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteCertificate
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteConnector
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteHostKey
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteProfile
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteServer
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteSshPublicKey
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteUser
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteWebApp
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteWebAppCustomization
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteWorkflow
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeAccess
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeAgreement
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeCertificate
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeConnector
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeExecution
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeHostKey
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeProfile
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeSecurityPolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeServer
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeUser
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeWebApp
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeWebAppCustomization
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeWorkflow
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ImportCertificate
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ImportHostKey
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ImportSshPublicKey
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListAccesses
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListAgreements
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListCertificates
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListConnectors
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListExecutions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListFileTransferResults
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListHostKeys
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListProfiles
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListSecurityPolicies
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListServers
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListUsers
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListWebApps
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListWorkflows
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] SendWorkflowStepState
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartDirectoryListing
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartFileTransfer
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartRemoteDelete
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartRemoteMove
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StartServer
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] StopServer
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] TagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] TestConnection
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] TestIdentityProvider
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UntagResource
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateAccess
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateAgreement
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateCertificate
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UpdateConnector
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateHostKey
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateProfile
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateServer
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateUser
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateWebApp
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateWebAppCustomization

Integration tests: 40/44 implemented operations tested (90.9%)
Untested implemented operations: 4

### winterbaume-trustedadvisor (trustedadvisor) - W: 6/11, S: 4/11, M: 0/11, F: 0/11, K: 0/11, C: 0/11

- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] BatchUpdateRecommendationResourceExclusion
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetOrganizationRecommendation
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetRecommendation
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] ListChecks
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] ListOrganizationRecommendationAccounts
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] ListOrganizationRecommendationResources
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListOrganizationRecommendations
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] ListRecommendationResources
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListRecommendations
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateOrganizationRecommendationLifecycle
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateRecommendationLifecycle

Integration tests: 3/6 implemented operations tested (50.0%)
Untested implemented operations: 3

### winterbaume-vpclattice (vpc-lattice) - W: 66/73, S: 2/73, M: 35/73, F: 0/73, K: 0/73, C: 0/73

Terraform E2E: 18 tests across 9 terraform resource types

Resource types: aws_vpclattice_auth_policy, aws_vpclattice_listener, aws_vpclattice_listener_rule, aws_vpclattice_resource_policy, aws_vpclattice_service, aws_vpclattice_service_network, aws_vpclattice_service_network_service_association, aws_vpclattice_service_network_vpc_association, aws_vpclattice_target_group

- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] BatchUpdateRule
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateAccessLogSubscription
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateListener
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateResourceConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateResourceGateway
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateRule
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateService
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateServiceNetwork
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateServiceNetworkResourceAssociation
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateServiceNetworkServiceAssociation
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateServiceNetworkVpcAssociation
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateTargetGroup
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteAccessLogSubscription
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteAuthPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteDomainVerification
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteListener
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteResourceConfiguration
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] DeleteResourceEndpointAssociation
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteResourceGateway
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteResourcePolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteRule
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteService
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteServiceNetwork
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteServiceNetworkResourceAssociation
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteServiceNetworkServiceAssociation
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteServiceNetworkVpcAssociation
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteTargetGroup
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeregisterTargets
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetAccessLogSubscription
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetAuthPolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetDomainVerification
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetListener
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetResourceConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetResourceGateway
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetResourcePolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetRule
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetService
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetServiceNetwork
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetServiceNetworkResourceAssociation
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetServiceNetworkServiceAssociation
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetServiceNetworkVpcAssociation
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetTargetGroup
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListAccessLogSubscriptions
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListDomainVerifications
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListListeners
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListResourceConfigurations
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] ListResourceEndpointAssociations
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListResourceGateways
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListRules
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListServiceNetworkResourceAssociations
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListServiceNetworkServiceAssociations
- W[ ] S[ ] M[x] F[ ] K[ ] C[ ] ListServiceNetworkVpcAssociations
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListServiceNetworkVpcEndpointAssociations
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListServiceNetworks
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListServices
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListTagsForResource
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListTargetGroups
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListTargets
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] PutAuthPolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] PutResourcePolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] RegisterTargets
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] StartDomainVerification
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] TagResource
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UntagResource
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UpdateAccessLogSubscription
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateListener
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateResourceConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateResourceGateway
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateRule
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateService
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateServiceNetwork
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateServiceNetworkVpcAssociation
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateTargetGroup

Integration tests: 62/66 implemented operations tested (93.9%)
Untested implemented operations: 4

### winterbaume-wafv2 (wafv2) - W: 38/55, S: 0/55, M: 29/55, F: 0/55, K: 0/55, C: 55/55

Terraform E2E: 12 tests across 4 terraform resource types

Resource types: aws_wafv2_ip_set, aws_wafv2_regex_pattern_set, aws_wafv2_rule_group, aws_wafv2_web_acl

- W[x] S[ ] M[x] F[ ] K[ ] C[x] AssociateWebACL
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CheckCapacity
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] CreateAPIKey
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateIPSet
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateRegexPatternSet
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateRuleGroup
- W[x] S[ ] M[x] F[ ] K[ ] C[x] CreateWebACL
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeleteAPIKey
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DeleteFirewallManagerRuleGroups
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteIPSet
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteLoggingConfiguration
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] DeletePermissionPolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteRegexPatternSet
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteRuleGroup
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DeleteWebACL
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DescribeAllManagedProducts
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DescribeManagedProductsByVendor
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] DescribeManagedRuleGroup
- W[x] S[ ] M[x] F[ ] K[ ] C[x] DisassociateWebACL
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GenerateMobileSdkReleaseUrl
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetDecryptedAPIKey
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetIPSet
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetLoggingConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetManagedRuleSet
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetMobileSdkRelease
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] GetPermissionPolicy
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetRateBasedStatementManagedKeys
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetRegexPatternSet
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetRuleGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetSampledRequests
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] GetTopPathStatisticsByTraffic
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetWebACL
- W[x] S[ ] M[x] F[ ] K[ ] C[x] GetWebACLForResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListAPIKeys
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ListAvailableManagedRuleGroupVersions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ListAvailableManagedRuleGroups
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListIPSets
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListLoggingConfigurations
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ListManagedRuleSets
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] ListMobileSdkReleases
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListRegexPatternSets
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListResourcesForWebACL
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListRuleGroups
- W[x] S[ ] M[x] F[ ] K[ ] C[x] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] ListWebACLs
- W[x] S[ ] M[x] F[ ] K[ ] C[x] PutLoggingConfiguration
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] PutManagedRuleSetVersions
- W[x] S[ ] M[ ] F[ ] K[ ] C[x] PutPermissionPolicy
- W[x] S[ ] M[x] F[ ] K[ ] C[x] TagResource
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UntagResource
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UpdateIPSet
- W[ ] S[ ] M[ ] F[ ] K[ ] C[x] UpdateManagedRuleSetVersionExpiryDate
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UpdateRegexPatternSet
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UpdateRuleGroup
- W[x] S[ ] M[x] F[ ] K[ ] C[x] UpdateWebACL

Integration tests: 31/38 implemented operations tested (81.6%)
Untested implemented operations: 7

### winterbaume-workspaces (workspaces) - W: 50/91, S: 0/91, M: 16/91, F: 0/91, K: 0/91, C: 0/91

Terraform E2E: 4 tests across 2 terraform resource types

Resource types: aws_workspaces_connection_alias, aws_workspaces_ip_group

- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AcceptAccountLinkInvitation
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] AssociateConnectionAlias
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] AssociateIpGroups
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AssociateWorkspaceApplication
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] AuthorizeIpRules
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CopyWorkspaceImage
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateAccountLinkInvitation
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateConnectClientAddIn
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateConnectionAlias
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateIpGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateStandbyWorkspaces
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateTags
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateUpdatedWorkspaceImage
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateWorkspaceBundle
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateWorkspaceImage
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateWorkspaces
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateWorkspacesPool
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteAccountLinkInvitation
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteClientBranding
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteConnectClientAddIn
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteConnectionAlias
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteIpGroup
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteTags
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteWorkspaceBundle
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteWorkspaceImage
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeployWorkspaceApplications
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeregisterWorkspaceDirectory
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeAccount
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeAccountModifications
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeApplicationAssociations
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeApplications
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeBundleAssociations
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeClientBranding
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeClientProperties
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeConnectClientAddIns
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeConnectionAliasPermissions
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeConnectionAliases
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeCustomWorkspaceImageImport
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeImageAssociations
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeIpGroups
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeTags
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeWorkspaceAssociations
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeWorkspaceBundles
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeWorkspaceDirectories
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeWorkspaceImagePermissions
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeWorkspaceImages
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeWorkspaceSnapshots
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DescribeWorkspaces
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeWorkspacesConnectionStatus
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeWorkspacesPoolSessions
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DescribeWorkspacesPools
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DisassociateConnectionAlias
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DisassociateIpGroups
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DisassociateWorkspaceApplication
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetAccountLink
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ImportClientBranding
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ImportCustomWorkspaceImage
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ImportWorkspaceImage
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListAccountLinks
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListAvailableManagementCidrRanges
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] MigrateWorkspace
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ModifyAccount
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ModifyCertificateBasedAuthProperties
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ModifyClientProperties
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ModifyEndpointEncryptionMode
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ModifySamlProperties
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ModifySelfservicePermissions
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ModifyStreamingProperties
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ModifyWorkspaceAccessProperties
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ModifyWorkspaceCreationProperties
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ModifyWorkspaceProperties
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ModifyWorkspaceState
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] RebootWorkspaces
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] RebuildWorkspaces
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] RegisterWorkspaceDirectory
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] RejectAccountLinkInvitation
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] RestoreWorkspace
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] RevokeIpRules
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] StartWorkspaces
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] StartWorkspacesPool
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] StopWorkspaces
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] StopWorkspacesPool
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] TerminateWorkspaces
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] TerminateWorkspacesPool
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] TerminateWorkspacesPoolSession
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateConnectClientAddIn
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateConnectionAliasPermission
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateRulesOfIpGroup
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateWorkspaceBundle
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UpdateWorkspaceImagePermission
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateWorkspacesPool

Integration tests: 48/50 implemented operations tested (96.0%)
Untested implemented operations: 2

### winterbaume-workspacesweb (workspaces-web) - W: 68/75, S: 0/75, M: 27/75, F: 0/75, K: 0/75, C: 0/75

- W[x] S[ ] M[x] F[ ] K[ ] C[ ] AssociateBrowserSettings
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] AssociateDataProtectionSettings
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] AssociateIpAccessSettings
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] AssociateNetworkSettings
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] AssociateSessionLogger
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] AssociateTrustStore
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] AssociateUserAccessLoggingSettings
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] AssociateUserSettings
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateBrowserSettings
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateDataProtectionSettings
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateIdentityProvider
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] CreateIpAccessSettings
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateNetworkSettings
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreatePortal
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateSessionLogger
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateTrustStore
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateUserAccessLoggingSettings
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] CreateUserSettings
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteBrowserSettings
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteDataProtectionSettings
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteIdentityProvider
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteIpAccessSettings
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteNetworkSettings
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeletePortal
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteSessionLogger
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteTrustStore
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteUserAccessLoggingSettings
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] DeleteUserSettings
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DisassociateBrowserSettings
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DisassociateDataProtectionSettings
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] DisassociateIpAccessSettings
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DisassociateNetworkSettings
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DisassociateSessionLogger
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DisassociateTrustStore
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DisassociateUserAccessLoggingSettings
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DisassociateUserSettings
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ExpireSession
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetBrowserSettings
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetDataProtectionSettings
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetIdentityProvider
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] GetIpAccessSettings
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetNetworkSettings
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetPortal
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetPortalServiceProviderMetadata
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetSession
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetSessionLogger
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetTrustStore
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetTrustStoreCertificate
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetUserAccessLoggingSettings
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] GetUserSettings
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListBrowserSettings
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListDataProtectionSettings
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListIdentityProviders
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] ListIpAccessSettings
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListNetworkSettings
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListPortals
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListSessionLoggers
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListSessions
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListTrustStoreCertificates
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListTrustStores
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListUserAccessLoggingSettings
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] ListUserSettings
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] TagResource
- W[x] S[ ] M[x] F[ ] K[ ] C[ ] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateBrowserSettings
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateDataProtectionSettings
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateIdentityProvider
- W[ ] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateIpAccessSettings
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateNetworkSettings
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdatePortal
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateSessionLogger
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateTrustStore
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateUserAccessLoggingSettings
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateUserSettings

Integration tests: 63/68 implemented operations tested (92.6%)
Untested implemented operations: 5

### winterbaume-xray (xray) - W: 34/38, S: 4/38, M: 0/38, F: 0/38, K: 6/38, C: 0/38

- W[x] S[ ] M[ ] F[ ] K[x] C[ ] BatchGetTraces
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CancelTraceRetrieval
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] CreateGroup
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] CreateSamplingRule
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] DeleteGroup
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteResourcePolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] DeleteSamplingRule
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetEncryptionConfig
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetGroup
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetGroups
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetIndexingRules
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] GetInsight
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] GetInsightEvents
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] GetInsightImpactGraph
- W[ ] S[x] M[ ] F[ ] K[ ] C[ ] GetInsightSummaries
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetRetrievedTracesGraph
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetSamplingRules
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetSamplingStatisticSummaries
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetSamplingTargets
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] GetServiceGraph
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetTimeSeriesServiceStatistics
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetTraceGraph
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] GetTraceSegmentDestination
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] GetTraceSummaries
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListResourcePolicies
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListRetrievedTraces
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] ListTagsForResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutEncryptionConfig
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutResourcePolicy
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] PutTelemetryRecords
- W[x] S[ ] M[ ] F[ ] K[x] C[ ] PutTraceSegments
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] StartTraceRetrieval
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] TagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UntagResource
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateGroup
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateIndexingRule
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateSamplingRule
- W[x] S[ ] M[ ] F[ ] K[ ] C[ ] UpdateTraceSegmentDestination

Integration tests: 26/34 implemented operations tested (76.5%)
Untested implemented operations: 8
