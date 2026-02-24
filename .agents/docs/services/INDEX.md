# AWS Service Research Index

These documents describe AWS services and operations from the official AWS Smithy API models vendored under `vendor/api-models-aws`. They intentionally omit local implementation status.

The following service documents include an additional `Official AWS Documentation Research` section sourced through the AWS Documentation MCP: `cloudformation`, `cloudwatch-logs`, `dynamodb`, `ec2`, `eventbridge`, `iam`, `kms`, `lambda`, `s3`, `s3files`, `sfn`, `sns`, and `sqs`.

| Model Slug | SDK Slug | Title | Protocols | Operations | Resources |
|---|---|---|---|---:|---:|
| [accessanalyzer](accessanalyzer.md) | `accessanalyzer` | Access Analyzer | restJson1 | 37 | 2 |
| [account](account.md) | `account` | AWS Account | restJson1 | 15 | 6 |
| [acm](acm.md) | `acm` | AWS Certificate Manager | awsJson1_1 | 16 | 0 |
| [acm-pca](acm-pca.md) | `acmpca` | AWS Certificate Manager Private Certificate Authority | awsJson1_1 | 23 | 0 |
| [aiops](aiops.md) | `aiops` | AWS AI Ops | restJson1 | 11 | 2 |
| [amp](amp.md) | `amp` | Amazon Prometheus Service | restJson1 | 44 | 10 |
| [amplify](amplify.md) | `amplify` | AWS Amplify | restJson1 | 37 | 0 |
| [amplifybackend](amplifybackend.md) | `amplifybackend` | AmplifyBackend | restJson1 | 31 | 0 |
| [amplifyuibuilder](amplifyuibuilder.md) | `amplifyuibuilder` | AWS Amplify UI Builder | restJson1 | 28 | 4 |
| [api-gateway](api-gateway.md) | `apigateway` | Amazon API Gateway | restJson1 | 124 | 0 |
| [apigatewaymanagementapi](apigatewaymanagementapi.md) | `apigatewaymanagement` | AmazonApiGatewayManagementApi | restJson1 | 3 | 0 |
| [apigatewayv2](apigatewayv2.md) | `apigatewayv2` | AmazonApiGatewayV2 | restJson1 | 103 | 0 |
| [app-mesh](app-mesh.md) | `appmesh` | AWS App Mesh | restJson1 | 38 | 7 |
| [appconfig](appconfig.md) | `appconfig` | Amazon AppConfig | restJson1 | 45 | 0 |
| [appconfigdata](appconfigdata.md) | `appconfigdata` | AWS AppConfig Data | restJson1 | 2 | 1 |
| [appfabric](appfabric.md) | `appfabric` | AppFabric | restJson1 | 26 | 0 |
| [appflow](appflow.md) | `appflow` | Amazon Appflow | restJson1 | 25 | 0 |
| [appintegrations](appintegrations.md) | `appintegrations` | Amazon AppIntegrations Service | restJson1 | 23 | 0 |
| [application-auto-scaling](application-auto-scaling.md) | `applicationautoscaling` | Application Auto Scaling | awsJson1_1 | 14 | 0 |
| [application-discovery-service](application-discovery-service.md) | `applicationdiscovery` | AWS Application Discovery Service | awsJson1_1 | 28 | 0 |
| [application-insights](application-insights.md) | `applicationinsights` | Amazon CloudWatch Application Insights | awsJson1_1 | 33 | 0 |
| [application-signals](application-signals.md) | `applicationsignals` | Amazon CloudWatch Application Signals | restJson1 | 23 | 1 |
| [applicationcostprofiler](applicationcostprofiler.md) | `applicationcostprofiler` | AWS Application Cost Profiler | restJson1 | 6 | 0 |
| [apprunner](apprunner.md) | `apprunner` | AWS App Runner | awsJson1_0 | 37 | 0 |
| [appstream](appstream.md) | `appstream` | Amazon AppStream | awsJson1_1 | 88 | 0 |
| [appsync](appsync.md) | `appsync` | AWS AppSync | restJson1 | 74 | 0 |
| [arc-region-switch](arc-region-switch.md) | `arcregionswitch` | ARC - Region switch | awsJson1_0 | 21 | 1 |
| [arc-zonal-shift](arc-zonal-shift.md) | `arczonalshift` | AWS ARC - Zonal Shift | restJson1 | 15 | 7 |
| [artifact](artifact.md) | `artifact` | AWS Artifact | restJson1 | 8 | 4 |
| [athena](athena.md) | `athena` | Amazon Athena | awsJson1_1 | 70 | 0 |
| [auditmanager](auditmanager.md) | `auditmanager` | AWS Audit Manager | restJson1 | 62 | 0 |
| [auto-scaling](auto-scaling.md) | `autoscaling` | Auto Scaling | awsQuery | 66 | 0 |
| [auto-scaling-plans](auto-scaling-plans.md) | `autoscalingplans` | AWS Auto Scaling Plans | awsJson1_1 | 6 | 0 |
| [b2bi](b2bi.md) | `b2bi` | AWS B2B Data Interchange | awsJson1_0 | 30 | 4 |
| [backup](backup.md) | `backup` | AWS Backup | restJson1 | 108 | 0 |
| [backup-gateway](backup-gateway.md) | `backupgateway` | AWS Backup Gateway | awsJson1_0 | 25 | 5 |
| [backupsearch](backupsearch.md) | `backupsearch` | AWS Backup Search | restJson1 | 12 | 2 |
| [batch](batch.md) | `batch` | AWS Batch | restJson1 | 39 | 0 |
| [bcm-dashboards](bcm-dashboards.md) | `bcmdashboards` | AWS Billing and Cost Management Dashboards | awsJson1_0 | 9 | 0 |
| [bcm-data-exports](bcm-data-exports.md) | `bcmdataexports` | AWS Billing and Cost Management Data Exports | awsJson1_1 | 12 | 1 |
| [bcm-pricing-calculator](bcm-pricing-calculator.md) | `bcmpricingcalculator` | AWS Billing and Cost Management Pricing Calculator | awsJson1_0 | 36 | 10 |
| [bcm-recommended-actions](bcm-recommended-actions.md) | `bcmrecommendedactions` | AWS Billing and Cost Management Recommended Actions | awsJson1_0 | 1 | 0 |
| [bedrock](bedrock.md) | `bedrock` | Amazon Bedrock | restJson1 | 98 | 19 |
| [bedrock-agent](bedrock-agent.md) | `bedrockagent` | Agents for Amazon Bedrock | restJson1 | 72 | 14 |
| [bedrock-agent-runtime](bedrock-agent-runtime.md) | `bedrock` | Agents for Amazon Bedrock Runtime | restJson1 | 31 | 15 |
| [bedrock-agentcore](bedrock-agentcore.md) | `bedrockagentcore` | Amazon Bedrock AgentCore | restJson1 | 36 | 6 |
| [bedrock-agentcore-control](bedrock-agentcore-control.md) | `bedrockagentcorecontrol` | Amazon Bedrock AgentCore Control | restJson1 | 86 | 16 |
| [bedrock-data-automation](bedrock-data-automation.md) | `bedrock` | Data Automation for Amazon Bedrock | restJson1 | 17 | 3 |
| [bedrock-data-automation-runtime](bedrock-data-automation-runtime.md) | `bedrock` | Runtime for Amazon Bedrock Data Automation | awsJson1_1 | 6 | 1 |
| [bedrock-runtime](bedrock-runtime.md) | `bedrockruntime` | Amazon Bedrock Runtime | restJson1 | 10 | 4 |
| [billing](billing.md) | `billing` | AWS Billing | awsJson1_0 | 12 | 0 |
| [billingconductor](billingconductor.md) | `billingconductor` | AWSBillingConductor | restJson1 | 32 | 4 |
| [braket](braket.md) | `braket` | Braket | restJson1 | 17 | 4 |
| [budgets](budgets.md) | `budgets` | AWS Budgets | awsJson1_1 | 26 | 0 |
| [chatbot](chatbot.md) | `chatbot` | AWS Chatbot | restJson1 | 34 | 1 |
| [chime](chime.md) | `chime` | Amazon Chime | restJson1 | 62 | 0 |
| [chime-sdk-identity](chime-sdk-identity.md) | `chimesdkidentity` | Amazon Chime SDK Identity | restJson1 | 30 | 0 |
| [chime-sdk-media-pipelines](chime-sdk-media-pipelines.md) | `chimesdkmediapipelines` | Amazon Chime SDK Media Pipelines | restJson1 | 31 | 0 |
| [chime-sdk-meetings](chime-sdk-meetings.md) | `chimesdkmeetings` | Amazon Chime SDK Meetings | restJson1 | 16 | 0 |
| [chime-sdk-messaging](chime-sdk-messaging.md) | `chimesdkmessaging` | Amazon Chime SDK Messaging | restJson1 | 51 | 0 |
| [chime-sdk-voice](chime-sdk-voice.md) | `chimesdkvoice` | Amazon Chime SDK Voice | restJson1 | 96 | 0 |
| [cleanrooms](cleanrooms.md) | `cleanrooms` | AWS Clean Rooms Service | restJson1 | 88 | 9 |
| [cleanroomsml](cleanroomsml.md) | `cleanroomsml` | AWS Clean Rooms ML | restJson1 | 59 | 13 |
| [cloud9](cloud9.md) | `cloud9` | AWS Cloud9 | awsJson1_1 | 13 | 0 |
| [cloudcontrol](cloudcontrol.md) | `cloudcontrol` | AWS Cloud Control API | awsJson1_0 | 8 | 0 |
| [clouddirectory](clouddirectory.md) | `clouddirectory` | Amazon CloudDirectory | restJson1 | 66 | 0 |
| [cloudformation](cloudformation.md) | `cloudformation` | AWS CloudFormation | awsQuery | 90 | 0 |
| [cloudfront](cloudfront.md) | `cloudfront` | Amazon CloudFront | restXml | 167 | 0 |
| [cloudfront-keyvaluestore](cloudfront-keyvaluestore.md) | `cloudfrontkeyvaluestore` | Amazon CloudFront KeyValueStore | restJson1 | 6 | 0 |
| [cloudhsm](cloudhsm.md) | `cloudhsm` | Amazon CloudHSM | awsJson1_1 | 20 | 0 |
| [cloudhsm-v2](cloudhsm-v2.md) | `cloudhsmv2` | AWS CloudHSM V2 | awsJson1_1 | 18 | 0 |
| [cloudsearch](cloudsearch.md) | `cloudsearch` | Amazon CloudSearch | awsQuery | 26 | 0 |
| [cloudsearch-domain](cloudsearch-domain.md) | `cloudsearchdomain` | Amazon CloudSearch Domain | restJson1 | 3 | 0 |
| [cloudtrail](cloudtrail.md) | `cloudtrail` | AWS CloudTrail | awsJson1_1 | 60 | 0 |
| [cloudtrail-data](cloudtrail-data.md) | `cloudtraildata` | AWS CloudTrail Data Service | restJson1 | 1 | 0 |
| [cloudwatch](cloudwatch.md) | `cloudwatch` | Amazon CloudWatch | awsJson1_0, awsQuery, awsQueryCompatible | 43 | 0 |
| [cloudwatch-events](cloudwatch-events.md) | `cloudwatchevents` | Amazon CloudWatch Events | awsJson1_1 | 51 | 0 |
| [cloudwatch-logs](cloudwatch-logs.md) | `cloudwatchlogs` | Amazon CloudWatch Logs | awsJson1_1 | 108 | 0 |
| [codeartifact](codeartifact.md) | `codeartifact` | CodeArtifact | restJson1 | 48 | 0 |
| [codebuild](codebuild.md) | `codebuild` | AWS CodeBuild | awsJson1_1 | 59 | 0 |
| [codecatalyst](codecatalyst.md) | `codecatalyst` | Amazon CodeCatalyst | restJson1 | 38 | 10 |
| [codecommit](codecommit.md) | `codecommit` | AWS CodeCommit | awsJson1_1 | 79 | 0 |
| [codeconnections](codeconnections.md) | `codeconnections` | AWS CodeConnections | awsJson1_0 | 27 | 0 |
| [codedeploy](codedeploy.md) | `codedeploy` | AWS CodeDeploy | awsJson1_1 | 47 | 0 |
| [codeguru-reviewer](codeguru-reviewer.md) | `codegurureviewer` | Amazon CodeGuru Reviewer | restJson1 | 14 | 0 |
| [codeguru-security](codeguru-security.md) | `codegurusecurity` | Amazon CodeGuru Security | restJson1 | 13 | 0 |
| [codeguruprofiler](codeguruprofiler.md) | `codeguruprofiler` | Amazon CodeGuru Profiler | restJson1 | 23 | 1 |
| [codepipeline](codepipeline.md) | `codepipeline` | AWS CodePipeline | awsJson1_1 | 44 | 0 |
| [codestar-connections](codestar-connections.md) | `codestarconnections` | AWS CodeStar connections | awsJson1_0 | 27 | 0 |
| [codestar-notifications](codestar-notifications.md) | `codestarnotifications` | AWS CodeStar Notifications | restJson1 | 13 | 0 |
| [cognito-identity](cognito-identity.md) | `cognitoidentity` | Amazon Cognito Identity | awsJson1_1 | 23 | 0 |
| [cognito-identity-provider](cognito-identity-provider.md) | `cognitoidentityprovider` | Amazon Cognito Identity Provider | awsJson1_1 | 122 | 0 |
| [cognito-sync](cognito-sync.md) | `cognitosync` | Amazon Cognito Sync | restJson1 | 17 | 0 |
| [comprehend](comprehend.md) | `comprehend` | Amazon Comprehend | awsJson1_1 | 85 | 0 |
| [comprehendmedical](comprehendmedical.md) | `comprehendmedical` | AWS Comprehend Medical | awsJson1_1 | 26 | 0 |
| [compute-optimizer](compute-optimizer.md) | `computeoptimizer` | AWS Compute Optimizer | awsJson1_0 | 28 | 0 |
| [compute-optimizer-automation](compute-optimizer-automation.md) | `computeoptimizerautomation` | Compute Optimizer Automation | awsJson1_0 | 23 | 0 |
| [config-service](config-service.md) | `config` | AWS Config | awsJson1_1 | 97 | 0 |
| [connect](connect.md) | `connect` | Amazon Connect Service | restJson1 | 367 | 0 |
| [connect-contact-lens](connect-contact-lens.md) | `connectcontactlens` | Amazon Connect Contact Lens | restJson1 | 1 | 0 |
| [connectcampaigns](connectcampaigns.md) | `connectcampaigns` | AmazonConnectCampaignService | restJson1 | 22 | 0 |
| [connectcampaignsv2](connectcampaignsv2.md) | `connectcampaigns` | AmazonConnectCampaignServiceV2 | restJson1 | 35 | 0 |
| [connectcases](connectcases.md) | `connectcases` | Amazon Connect Cases | restJson1 | 42 | 7 |
| [connecthealth](connecthealth.md) | `connecthealth` | Connect Health | restJson1 | 16 | 0 |
| [connectparticipant](connectparticipant.md) | `connectparticipant` | Amazon Connect Participant Service | restJson1 | 11 | 0 |
| [controlcatalog](controlcatalog.md) | `controlcatalog` | AWS Control Catalog | restJson1 | 6 | 4 |
| [controltower](controltower.md) | `controltower` | AWS Control Tower | restJson1 | 28 | 8 |
| [cost-and-usage-report-service](cost-and-usage-report-service.md) | `costandusagereport` | AWS Cost and Usage Report Service | awsJson1_1 | 7 | 0 |
| [cost-explorer](cost-explorer.md) | `costexplorer` | AWS Cost Explorer Service | awsJson1_1 | 47 | 0 |
| [cost-optimization-hub](cost-optimization-hub.md) | `costoptimizationhub` | Cost Optimization Hub | awsJson1_0 | 8 | 0 |
| [customer-profiles](customer-profiles.md) | `customerprofiles` | Amazon Connect Customer Profiles | restJson1 | 102 | 0 |
| [data-pipeline](data-pipeline.md) | `datapipeline` | AWS Data Pipeline | awsJson1_1 | 19 | 0 |
| [database-migration-service](database-migration-service.md) | `databasemigration` | AWS Database Migration Service | awsJson1_1 | 119 | 0 |
| [databrew](databrew.md) | `databrew` | AWS Glue DataBrew | restJson1 | 44 | 0 |
| [dataexchange](dataexchange.md) | `dataexchange` | AWS Data Exchange | restJson1 | 37 | 0 |
| [datasync](datasync.md) | `datasync` | AWS DataSync | awsJson1_1 | 53 | 0 |
| [datazone](datazone.md) | `datazone` | Amazon DataZone | restJson1 | 176 | 14 |
| [dax](dax.md) | `dax` | Amazon DynamoDB Accelerator (DAX) | awsJson1_1 | 21 | 0 |
| [deadline](deadline.md) | `deadline` | AWSDeadlineCloud | restJson1 | 113 | 8 |
| [detective](detective.md) | `detective` | Amazon Detective | restJson1 | 29 | 0 |
| [device-farm](device-farm.md) | `devicefarm` | AWS Device Farm | awsJson1_1 | 77 | 0 |
| [devops-guru](devops-guru.md) | `devopsguru` | Amazon DevOps Guru | restJson1 | 31 | 0 |
| [direct-connect](direct-connect.md) | `directconnect` | AWS Direct Connect | awsJson1_1 | 63 | 0 |
| [directory-service](directory-service.md) | `directory` | AWS Directory Service | awsJson1_1 | 80 | 0 |
| [directory-service-data](directory-service-data.md) | `directoryservicedata` | AWS Directory Service Data | restJson1 | 17 | 0 |
| [dlm](dlm.md) | `dlm` | Amazon Data Lifecycle Manager | restJson1 | 8 | 0 |
| [docdb](docdb.md) | `rds` | Amazon DocumentDB with MongoDB compatibility | awsQuery | 55 | 0 |
| [docdb-elastic](docdb-elastic.md) | `docdbelastic` | Amazon DocumentDB Elastic Clusters | restJson1 | 19 | 0 |
| [drs](drs.md) | `drs` | Elastic Disaster Recovery Service | restJson1 | 50 | 7 |
| [dsql](dsql.md) | `dsql` | Amazon Aurora DSQL | restJson1 | 12 | 1 |
| [dynamodb](dynamodb.md) | `dynamodb` | Amazon DynamoDB | awsJson1_0 | 57 | 0 |
| [dynamodb-streams](dynamodb-streams.md) | `dynamodbstreams` | Amazon DynamoDB Streams | awsJson1_0 | 4 | 0 |
| [ebs](ebs.md) | `ebs` | Amazon Elastic Block Store | restJson1 | 6 | 0 |
| [ec2](ec2.md) | `ec2` | Amazon Elastic Compute Cloud | ec2Query | 756 | 0 |
| [ec2-instance-connect](ec2-instance-connect.md) | `ec2instanceconnect` | AWS EC2 Instance Connect | awsJson1_1 | 2 | 0 |
| [ecr](ecr.md) | `ecr` | Amazon Elastic Container Registry | awsJson1_1 | 58 | 0 |
| [ecr-public](ecr-public.md) | `ecrpublic` | Amazon Elastic Container Registry Public | awsJson1_1 | 23 | 0 |
| [ecs](ecs.md) | `ecs` | Amazon EC2 Container Service | awsJson1_1 | 64 | 9 |
| [efs](efs.md) | `efs` | Amazon Elastic File System | restJson1 | 31 | 0 |
| [eks](eks.md) | `eks` | Amazon Elastic Kubernetes Service | restJson1 | 64 | 0 |
| [eks-auth](eks-auth.md) | `eksauth` | Amazon EKS Auth | restJson1 | 1 | 0 |
| [elastic-beanstalk](elastic-beanstalk.md) | `elasticbeanstalk` | AWS Elastic Beanstalk | awsQuery | 47 | 0 |
| [elastic-load-balancing](elastic-load-balancing.md) | `elasticloadbalancing` | Elastic Load Balancing | awsQuery | 29 | 0 |
| [elastic-load-balancing-v2](elastic-load-balancing-v2.md) | `elasticloadbalancingv2` | Elastic Load Balancing | awsQuery | 51 | 0 |
| [elasticache](elasticache.md) | `elasticache` | Amazon ElastiCache | awsQuery | 75 | 0 |
| [elasticsearch-service](elasticsearch-service.md) | `elasticsearchservice` | Amazon Elasticsearch Service | restJson1 | 51 | 0 |
| [elementalinference](elementalinference.md) | `elementalinference` | AWS Elemental Inference | restJson1 | 10 | 1 |
| [emr](emr.md) | `emr` | Amazon EMR | awsJson1_1 | 60 | 0 |
| [emr-containers](emr-containers.md) | `emrcontainers` | Amazon EMR Containers | restJson1 | 23 | 0 |
| [emr-serverless](emr-serverless.md) | `emrserverless` | EMR Serverless | restJson1 | 16 | 2 |
| [entityresolution](entityresolution.md) | `entityresolution` | AWS EntityResolution | restJson1 | 38 | 0 |
| [eventbridge](eventbridge.md) | `eventbridge` | Amazon EventBridge | awsJson1_1 | 57 | 0 |
| [evs](evs.md) | `evs` | Amazon Elastic VMware Service | awsJson1_0 | 14 | 1 |
| [finspace](finspace.md) | `finspace` | FinSpace User Environment Management service | restJson1 | 50 | 0 |
| [finspace-data](finspace-data.md) | `finspacedata` | FinSpace Public API | restJson1 | 31 | 0 |
| [firehose](firehose.md) | `firehose` | Amazon Kinesis Firehose | awsJson1_1 | 12 | 0 |
| [fis](fis.md) | `fis` | AWS Fault Injection Simulator | restJson1 | 26 | 0 |
| [fms](fms.md) | `fms` | Firewall Management Service | awsJson1_1 | 42 | 0 |
| [forecast](forecast.md) | `forecast` | Amazon Forecast Service | awsJson1_1 | 63 | 0 |
| [forecastquery](forecastquery.md) | `forecast` | Amazon Forecast Query Service | awsJson1_1 | 2 | 0 |
| [frauddetector](frauddetector.md) | `frauddetector` | Amazon Fraud Detector | awsJson1_1 | 73 | 0 |
| [freetier](freetier.md) | `freetier` | AWS Free Tier | awsJson1_0 | 5 | 0 |
| [fsx](fsx.md) | `fsx` | Amazon FSx | awsJson1_1 | 48 | 0 |
| [gamelift](gamelift.md) | `gamelift` | Amazon GameLift | awsJson1_1 | 119 | 0 |
| [gameliftstreams](gameliftstreams.md) | `gameliftstreams` | Amazon GameLift Streams | restJson1 | 24 | 2 |
| [geo-maps](geo-maps.md) | `geomaps` | Amazon Location Service Maps V2 | restJson1 | 5 | 1 |
| [geo-places](geo-places.md) | `geoplaces` | Amazon Location Service Places V2 | restJson1 | 7 | 1 |
| [geo-routes](geo-routes.md) | `georoutes` | Amazon Location Service Routes V2 | restJson1 | 5 | 1 |
| [glacier](glacier.md) | `glacier` | Amazon Glacier | restJson1 | 33 | 0 |
| [global-accelerator](global-accelerator.md) | `globalaccelerator` | AWS Global Accelerator | awsJson1_1 | 56 | 0 |
| [glue](glue.md) | `glue` | AWS Glue | awsJson1_1 | 265 | 0 |
| [grafana](grafana.md) | `grafana` | Amazon Managed Grafana | restJson1 | 25 | 8 |
| [greengrass](greengrass.md) | `greengrass` | AWS Greengrass | restJson1 | 92 | 0 |
| [greengrassv2](greengrassv2.md) | `greengrass` | AWS IoT Greengrass V2 | restJson1 | 29 | 0 |
| [groundstation](groundstation.md) | `groundstation` | AWS Ground Station | restJson1 | 35 | 9 |
| [guardduty](guardduty.md) | `guardduty` | Amazon GuardDuty | restJson1 | 87 | 0 |
| [health](health.md) | `health` | AWS Health APIs and Notifications | awsJson1_1 | 14 | 0 |
| [healthlake](healthlake.md) | `healthlake` | Amazon HealthLake | awsJson1_0 | 13 | 0 |
| [iam](iam.md) | `iam` | AWS Identity and Access Management | awsQuery | 176 | 0 |
| [identitystore](identitystore.md) | `identitystore` | AWS SSO Identity Store | awsJson1_1 | 19 | 3 |
| [imagebuilder](imagebuilder.md) | `imagebuilder` | EC2 Image Builder | restJson1 | 77 | 0 |
| [inspector](inspector.md) | `inspector` | Amazon Inspector | awsJson1_1 | 37 | 0 |
| [inspector-scan](inspector-scan.md) | `inspectorscan` | Inspector Scan | restJson1 | 1 | 0 |
| [inspector2](inspector2.md) | `inspector2` | Inspector2 | restJson1 | 75 | 0 |
| [internetmonitor](internetmonitor.md) | `internetmonitor` | Amazon CloudWatch Internet Monitor | restJson1 | 16 | 3 |
| [invoicing](invoicing.md) | `invoicing` | AWS Invoicing | awsJson1_0 | 17 | 0 |
| [iot](iot.md) | `iot` | AWS IoT | restJson1 | 272 | 0 |
| [iot-data-plane](iot-data-plane.md) | `iotdataplane` | AWS IoT Data Plane | restJson1 | 8 | 0 |
| [iot-events](iot-events.md) | `iotevents` | AWS IoT Events | restJson1 | 26 | 0 |
| [iot-events-data](iot-events-data.md) | `ioteventsdata` | AWS IoT Events Data | restJson1 | 12 | 0 |
| [iot-jobs-data-plane](iot-jobs-data-plane.md) | `iotjobsdataplane` | AWS IoT Jobs Data Plane | restJson1 | 5 | 0 |
| [iot-managed-integrations](iot-managed-integrations.md) | `iotmanagedintegrations` | Managed integrations for AWS IoT Device Management | restJson1 | 83 | 19 |
| [iot-wireless](iot-wireless.md) | `iotwireless` | AWS IoT Wireless | restJson1 | 112 | 0 |
| [iotdeviceadvisor](iotdeviceadvisor.md) | `iotdeviceadvisor` | AWS IoT Core Device Advisor | restJson1 | 14 | 0 |
| [iotfleetwise](iotfleetwise.md) | `iotfleetwise` | AWS IoT FleetWise | awsJson1_0 | 57 | 9 |
| [iotsecuretunneling](iotsecuretunneling.md) | `iotsecuretunneling` | AWS IoT Secure Tunneling | awsJson1_1 | 8 | 0 |
| [iotsitewise](iotsitewise.md) | `iotsitewise` | AWS IoT SiteWise | restJson1 | 104 | 0 |
| [iotthingsgraph](iotthingsgraph.md) | `iotthingsgraph` | AWS IoT Things Graph | awsJson1_1 | 35 | 0 |
| [iottwinmaker](iottwinmaker.md) | `iottwinmaker` | AWS IoT TwinMaker | restJson1 | 40 | 0 |
| [ivs](ivs.md) | `ivs` | Amazon Interactive Video Service | restJson1 | 35 | 0 |
| [ivs-realtime](ivs-realtime.md) | `ivs` | Amazon Interactive Video Service RealTime | restJson1 | 39 | 0 |
| [ivschat](ivschat.md) | `ivschat` | Amazon Interactive Video Service Chat | restJson1 | 17 | 0 |
| [kafka](kafka.md) | `kafka` | Managed Streaming for Kafka | restJson1 | 59 | 0 |
| [kafkaconnect](kafkaconnect.md) | `kafkaconnect` | Managed Streaming for Kafka Connect | restJson1 | 18 | 0 |
| [kendra](kendra.md) | `kendra` | AWSKendraFrontendService | awsJson1_1 | 66 | 0 |
| [kendra-ranking](kendra-ranking.md) | `kendraranking` | Amazon Kendra Intelligent Ranking | awsJson1_0 | 9 | 0 |
| [keyspaces](keyspaces.md) | `keyspaces` | Amazon Keyspaces | awsJson1_0 | 19 | 0 |
| [keyspacesstreams](keyspacesstreams.md) | `keyspacesstreams` | Amazon Keyspaces Streams | awsJson1_0 | 4 | 0 |
| [kinesis](kinesis.md) | `kinesis` | Amazon Kinesis | awsJson1_1 | 39 | 0 |
| [kinesis-analytics](kinesis-analytics.md) | `kinesisanalytics` | Amazon Kinesis Analytics | awsJson1_1 | 20 | 0 |
| [kinesis-analytics-v2](kinesis-analytics-v2.md) | `kinesisanalyticsv2` | Amazon Kinesis Analytics | awsJson1_1 | 33 | 0 |
| [kinesis-video](kinesis-video.md) | `kinesisvideo` | Amazon Kinesis Video Streams | restJson1 | 32 | 0 |
| [kinesis-video-archived-media](kinesis-video-archived-media.md) | `kinesisvideoarchivedmedia` | Amazon Kinesis Video Streams Archived Media | restJson1 | 6 | 0 |
| [kinesis-video-media](kinesis-video-media.md) | `kinesisvideo` | Amazon Kinesis Video Streams Media | restJson1 | 1 | 0 |
| [kinesis-video-signaling](kinesis-video-signaling.md) | `kinesisvideo` | Amazon Kinesis Video Signaling Channels | restJson1 | 2 | 0 |
| [kinesis-video-webrtc-storage](kinesis-video-webrtc-storage.md) | `kinesisvideo` | Amazon Kinesis Video WebRTC Storage | restJson1 | 2 | 0 |
| [kms](kms.md) | `kms` | AWS Key Management Service | awsJson1_1 | 53 | 0 |
| [lakeformation](lakeformation.md) | `lakeformation` | AWS Lake Formation | restJson1 | 61 | 0 |
| [lambda](lambda.md) | `lambda` | AWS Lambda | restJson1 | 85 | 11 |
| [launch-wizard](launch-wizard.md) | `launchwizard` | AWS Launch Wizard | restJson1 | 15 | 6 |
| [lex-model-building-service](lex-model-building-service.md) | `lexmodelbuildingservice` | Amazon Lex Model Building Service | restJson1 | 42 | 0 |
| [lex-models-v2](lex-models-v2.md) | `lexmodelsv2` | Amazon Lex Model Building V2 | restJson1 | 107 | 0 |
| [lex-runtime-service](lex-runtime-service.md) | `lexruntimeservice` | Amazon Lex Runtime Service | restJson1 | 5 | 0 |
| [lex-runtime-v2](lex-runtime-v2.md) | `lexruntimev2` | Amazon Lex Runtime V2 | restJson1 | 6 | 0 |
| [license-manager](license-manager.md) | `licensemanager` | AWS License Manager | awsJson1_1 | 62 | 0 |
| [license-manager-linux-subscriptions](license-manager-linux-subscriptions.md) | `licensemanagerlinuxsubscriptions` | AWS License Manager Linux Subscriptions | restJson1 | 11 | 0 |
| [license-manager-user-subscriptions](license-manager-user-subscriptions.md) | `licensemanagerusersubscriptions` | AWS License Manager User Subscriptions | restJson1 | 17 | 0 |
| [lightsail](lightsail.md) | `lightsail` | Amazon Lightsail | awsJson1_1 | 161 | 0 |
| [location](location.md) | `location` | Amazon Location Service | restJson1 | 60 | 8 |
| [lookoutequipment](lookoutequipment.md) | `lookoutequipment` | Amazon Lookout for Equipment | awsJson1_0 | 49 | 0 |
| [m2](m2.md) | `m2` | AWSMainframeModernization | restJson1 | 37 | 2 |
| [machine-learning](machine-learning.md) | `machinelearning` | Amazon Machine Learning | awsJson1_1 | 28 | 0 |
| [macie2](macie2.md) | `macie2` | Amazon Macie 2 | restJson1 | 81 | 0 |
| [mailmanager](mailmanager.md) | `ses` | MailManager | awsJson1_0 | 60 | 8 |
| [managedblockchain](managedblockchain.md) | `managedblockchain` | Amazon Managed Blockchain | restJson1 | 27 | 0 |
| [managedblockchain-query](managedblockchain-query.md) | `managedblockchainquery` | Amazon Managed Blockchain Query | restJson1 | 9 | 0 |
| [marketplace-agreement](marketplace-agreement.md) | `marketplaceagreement` | AWS Marketplace Agreement Service | awsJson1_0 | 3 | 0 |
| [marketplace-catalog](marketplace-catalog.md) | `marketplacecatalog` | AWS Marketplace Catalog Service | restJson1 | 13 | 0 |
| [marketplace-commerce-analytics](marketplace-commerce-analytics.md) | `marketplacecommerceanalytics` | AWS Marketplace Commerce Analytics | awsJson1_1 | 2 | 0 |
| [marketplace-deployment](marketplace-deployment.md) | `marketplacedeployment` | AWS Marketplace Deployment Service | restJson1 | 4 | 1 |
| [marketplace-entitlement-service](marketplace-entitlement-service.md) | `marketplaceentitlementservice` | AWS Marketplace Entitlement Service | awsJson1_1 | 1 | 0 |
| [marketplace-metering](marketplace-metering.md) | `marketplacemetering` | AWSMarketplace Metering | awsJson1_1 | 4 | 0 |
| [marketplace-reporting](marketplace-reporting.md) | `marketplacereporting` | AWS Marketplace Reporting Service | restJson1 | 1 | 1 |
| [mediaconnect](mediaconnect.md) | `mediaconnect` | AWS MediaConnect | restJson1 | 82 | 14 |
| [mediaconvert](mediaconvert.md) | `mediaconvert` | AWS Elemental MediaConvert | restJson1 | 34 | 0 |
| [medialive](medialive.md) | `medialive` | AWS Elemental MediaLive | restJson1 | 123 | 0 |
| [mediapackage](mediapackage.md) | `mediapackage` | AWS Elemental MediaPackage | restJson1 | 19 | 0 |
| [mediapackage-vod](mediapackage-vod.md) | `mediapackagevod` | AWS Elemental MediaPackage VOD | restJson1 | 17 | 0 |
| [mediapackagev2](mediapackagev2.md) | `mediapackagev2` | AWS Elemental MediaPackage v2 | restJson1 | 30 | 6 |
| [mediastore](mediastore.md) | `mediastore` | AWS Elemental MediaStore | awsJson1_1 | 21 | 0 |
| [mediastore-data](mediastore-data.md) | `mediastoredata` | AWS Elemental MediaStore Data Plane | restJson1 | 5 | 0 |
| [mediatailor](mediatailor.md) | `mediatailor` | AWS MediaTailor | restJson1 | 44 | 8 |
| [medical-imaging](medical-imaging.md) | `medicalimaging` | AWS Health Imaging | restJson1 | 18 | 2 |
| [memorydb](memorydb.md) | `memorydb` | Amazon MemoryDB | awsJson1_1 | 45 | 0 |
| [mgn](mgn.md) | `mgn` | Application Migration Service | restJson1 | 95 | 13 |
| [migration-hub](migration-hub.md) | `migrationhub` | AWS Migration Hub | awsJson1_1 | 21 | 0 |
| [migration-hub-refactor-spaces](migration-hub-refactor-spaces.md) | `migrationhubrefactorspaces` | AWS Migration Hub Refactor Spaces | restJson1 | 24 | 0 |
| [migrationhub-config](migrationhub-config.md) | `migrationhubconfig` | AWS Migration Hub Config | awsJson1_1 | 4 | 0 |
| [migrationhuborchestrator](migrationhuborchestrator.md) | `migrationhuborchestrator` | AWS Migration Hub Orchestrator | restJson1 | 31 | 8 |
| [migrationhubstrategy](migrationhubstrategy.md) | `migrationhubstrategy` | Migration Hub Strategy Recommendations | restJson1 | 22 | 0 |
| [mpa](mpa.md) | `mpa` | AWS Multi-party Approval | restJson1 | 22 | 3 |
| [mq](mq.md) | `mq` | AmazonMQ | restJson1 | 24 | 0 |
| [mturk](mturk.md) | `mturk` | Amazon Mechanical Turk | awsJson1_1 | 39 | 0 |
| [mwaa](mwaa.md) | `mwaa` | AmazonMWAA | restJson1 | 12 | 0 |
| [mwaa-serverless](mwaa-serverless.md) | `mwaaserverless` | AmazonMWAAServerless | awsJson1_0 | 15 | 4 |
| [neptune](neptune.md) | `neptune` | Amazon Neptune | awsQuery | 70 | 0 |
| [neptune-graph](neptune-graph.md) | `neptunegraph` | Amazon Neptune Graph | restJson1 | 34 | 4 |
| [neptunedata](neptunedata.md) | `neptunedata` | Amazon NeptuneData | restJson1 | 43 | 0 |
| [network-firewall](network-firewall.md) | `networkfirewall` | AWS Network Firewall | awsJson1_0 | 79 | 0 |
| [networkflowmonitor](networkflowmonitor.md) | `networkflowmonitor` | Network Flow Monitor | restJson1 | 25 | 2 |
| [networkmanager](networkmanager.md) | `networkmanager` | AWS Network Manager | restJson1 | 95 | 0 |
| [networkmonitor](networkmonitor.md) | `networkmonitor` | Amazon CloudWatch Network Monitor | restJson1 | 12 | 2 |
| [notifications](notifications.md) | `notifications` | AWS User Notifications | restJson1 | 39 | 12 |
| [notificationscontacts](notificationscontacts.md) | `notificationscontacts` | AWS User Notifications Contacts | restJson1 | 9 | 1 |
| [nova-act](nova-act.md) | `novaact` | Nova Act Service | restJson1 | 16 | 6 |
| [oam](oam.md) | `oam` | CloudWatch Observability Access Manager | restJson1 | 15 | 0 |
| [observabilityadmin](observabilityadmin.md) | `observabilityadmin` | CloudWatch Observability Admin Service | restJson1 | 40 | 1 |
| [odb](odb.md) | `odb` | odb | awsJson1_0 | 43 | 6 |
| [omics](omics.md) | `omics` | Amazon Omics | restJson1 | 96 | 17 |
| [opensearch](opensearch.md) | `opensearch` | Amazon OpenSearch Service | restJson1 | 82 | 0 |
| [opensearchserverless](opensearchserverless.md) | `opensearchserverless` | OpenSearch Service Serverless | awsJson1_0 | 46 | 8 |
| [organizations](organizations.md) | `organizations` | AWS Organizations | awsJson1_1 | 63 | 0 |
| [osis](osis.md) | `osis` | Amazon OpenSearch Ingestion | restJson1 | 22 | 0 |
| [outposts](outposts.md) | `outposts` | AWS Outposts | restJson1 | 35 | 0 |
| [panorama](panorama.md) | `panorama` | AWS Panorama | restJson1 | 34 | 0 |
| [partnercentral-account](partnercentral-account.md) | `partnercentralaccount` | Partner Central Account API | awsJson1_0 | 29 | 4 |
| [partnercentral-benefits](partnercentral-benefits.md) | `partnercentralbenefits` | Partner Central Benefits API | awsJson1_0 | 17 | 3 |
| [partnercentral-channel](partnercentral-channel.md) | `partnercentralchannel` | Partner Central Channel API | awsJson1_0 | 17 | 3 |
| [partnercentral-selling](partnercentral-selling.md) | `partnercentralselling` | Partner Central Selling API | awsJson1_0 | 42 | 9 |
| [payment-cryptography](payment-cryptography.md) | `paymentcryptography` | Payment Cryptography Control Plane | awsJson1_0 | 26 | 2 |
| [payment-cryptography-data](payment-cryptography-data.md) | `paymentcryptographydata` | Payment Cryptography Data Plane | restJson1 | 14 | 0 |
| [pca-connector-ad](pca-connector-ad.md) | `pcaconnectorad` | PcaConnectorAd | restJson1 | 25 | 5 |
| [pca-connector-scep](pca-connector-scep.md) | `pcaconnectorscep` | Private CA Connector for SCEP | restJson1 | 12 | 2 |
| [pcs](pcs.md) | `pcs` | AWS Parallel Computing Service | awsJson1_0 | 19 | 3 |
| [personalize](personalize.md) | `personalize` | Amazon Personalize | awsJson1_1 | 71 | 0 |
| [personalize-events](personalize-events.md) | `personalizeevents` | Amazon Personalize Events | restJson1 | 5 | 0 |
| [personalize-runtime](personalize-runtime.md) | `personalizeruntime` | Amazon Personalize Runtime | restJson1 | 3 | 0 |
| [pi](pi.md) | `pi` | AWS Performance Insights | awsJson1_1 | 13 | 0 |
| [pinpoint](pinpoint.md) | `pinpoint` | Amazon Pinpoint | restJson1 | 122 | 0 |
| [pinpoint-email](pinpoint-email.md) | `ses` | Amazon Pinpoint Email Service | restJson1 | 42 | 0 |
| [pinpoint-sms-voice](pinpoint-sms-voice.md) | `pinpointsmsvoice` | Amazon Pinpoint SMS and Voice Service | restJson1 | 8 | 0 |
| [pinpoint-sms-voice-v2](pinpoint-sms-voice-v2.md) | `pinpointsmsvoicev2` | Amazon Pinpoint SMS Voice V2 | awsJson1_0 | 91 | 0 |
| [pipes](pipes.md) | `pipes` | Amazon EventBridge Pipes | restJson1 | 10 | 1 |
| [polly](polly.md) | `polly` | Amazon Polly | restJson1 | 9 | 0 |
| [pricing](pricing.md) | `pricing` | AWS Price List Service | awsJson1_1 | 5 | 0 |
| [proton](proton.md) | `proton` | AWS Proton | awsJson1_0 | 87 | 24 |
| [qapps](qapps.md) | `qapps` | QApps | restJson1 | 35 | 0 |
| [qbusiness](qbusiness.md) | `qbusiness` | QBusiness | restJson1 | 83 | 8 |
| [qconnect](qconnect.md) | `qconnect` | Amazon Q Connect | restJson1 | 93 | 11 |
| [quicksight](quicksight.md) | `quicksight` | Amazon QuickSight | restJson1 | 230 | 0 |
| [ram](ram.md) | `ram` | AWS Resource Access Manager | restJson1 | 35 | 0 |
| [rbin](rbin.md) | `rbin` | Amazon Recycle Bin | restJson1 | 10 | 0 |
| [rds](rds.md) | `rds` | Amazon Relational Database Service | awsQuery | 163 | 0 |
| [rds-data](rds-data.md) | `rdsdata` | AWS RDS DataService | restJson1 | 6 | 0 |
| [redshift](redshift.md) | `redshift` | Amazon Redshift | awsQuery | 141 | 0 |
| [redshift-data](redshift-data.md) | `redshiftdata` | Redshift Data API Service | awsJson1_1 | 11 | 0 |
| [redshift-serverless](redshift-serverless.md) | `redshiftserverless` | Redshift Serverless | awsJson1_1 | 65 | 9 |
| [rekognition](rekognition.md) | `rekognition` | Amazon Rekognition | awsJson1_1 | 75 | 0 |
| [repostspace](repostspace.md) | `repostspace` | AWS re:Post Private | restJson1 | 19 | 0 |
| [resiliencehub](resiliencehub.md) | `resiliencehub` | AWS Resilience Hub | restJson1 | 63 | 0 |
| [resource-explorer-2](resource-explorer-2.md) | `resourceexplorer2` | AWS Resource Explorer | restJson1 | 32 | 3 |
| [resource-groups](resource-groups.md) | `resourcegroups` | AWS Resource Groups | restJson1 | 23 | 0 |
| [resource-groups-tagging-api](resource-groups-tagging-api.md) | `resourcegroupstagging` | AWS Resource Groups Tagging API | awsJson1_1 | 9 | 0 |
| [rolesanywhere](rolesanywhere.md) | `rolesanywhere` | IAM Roles Anywhere | restJson1 | 30 | 4 |
| [route-53](route-53.md) | `route53` | Amazon Route 53 | restXml | 71 | 0 |
| [route-53-domains](route-53-domains.md) | `route53domains` | Amazon Route 53 Domains | awsJson1_1 | 34 | 0 |
| [route53-recovery-cluster](route53-recovery-cluster.md) | `route53recoverycluster` | Route53 Recovery Cluster | awsJson1_0 | 4 | 0 |
| [route53-recovery-control-config](route53-recovery-control-config.md) | `route53recoverycontrolconfig` | AWS Route53 Recovery Control Config | restJson1 | 25 | 0 |
| [route53-recovery-readiness](route53-recovery-readiness.md) | `route53recoveryreadiness` | AWS Route53 Recovery Readiness | restJson1 | 32 | 0 |
| [route53globalresolver](route53globalresolver.md) | `route53globalresolver` | Amazon Route 53 Global Resolver | restJson1 | 47 | 8 |
| [route53profiles](route53profiles.md) | `route53profiles` | Route 53 Profiles | restJson1 | 16 | 0 |
| [route53resolver](route53resolver.md) | `route53resolver` | Amazon Route 53 Resolver | awsJson1_1 | 68 | 0 |
| [rtbfabric](rtbfabric.md) | `rtbfabric` | RTBFabric | restJson1 | 27 | 6 |
| [rum](rum.md) | `rum` | CloudWatch RUM | restJson1 | 20 | 1 |
| [s3](s3.md) | `s3` | Amazon Simple Storage Service | restXml | 107 | 0 |
| [s3-control](s3-control.md) | `s3control` | AWS S3 Control | restXml | 97 | 0 |
| [s3files](s3files.md) | `s3files` | Amazon S3 Files | restJson1 | 21 | 0 |
| [s3outposts](s3outposts.md) | `s3outposts` | Amazon S3 on Outposts | restJson1 | 5 | 0 |
| [s3tables](s3tables.md) | `s3tables` | Amazon S3 Tables | restJson1 | 49 | 9 |
| [s3vectors](s3vectors.md) | `s3vectors` | Amazon S3 Vectors | restJson1 | 19 | 3 |
| [sagemaker](sagemaker.md) | `sagemaker` | Amazon SageMaker Service | awsJson1_1 | 381 | 0 |
| [sagemaker-a2i-runtime](sagemaker-a2i-runtime.md) | `sagemaker` | Amazon Augmented AI Runtime | restJson1 | 5 | 0 |
| [sagemaker-edge](sagemaker-edge.md) | `sagemaker` | Amazon Sagemaker Edge Manager | restJson1 | 3 | 0 |
| [sagemaker-featurestore-runtime](sagemaker-featurestore-runtime.md) | `sagemaker` | Amazon SageMaker Feature Store Runtime | restJson1 | 4 | 0 |
| [sagemaker-geospatial](sagemaker-geospatial.md) | `sagemakergeospatial` | Amazon SageMaker geospatial capabilities | restJson1 | 19 | 3 |
| [sagemaker-metrics](sagemaker-metrics.md) | `sagemakermetrics` | Amazon SageMaker Metrics Service | restJson1 | 2 | 0 |
| [sagemaker-runtime](sagemaker-runtime.md) | `sagemakerruntime` | Amazon SageMaker Runtime | restJson1 | 3 | 0 |
| [sagemaker-runtime-http2](sagemaker-runtime-http2.md) | `sagemaker` | Amazon SageMaker Runtime HTTP2 | restJson1 | 1 | 0 |
| [savingsplans](savingsplans.md) | `savingsplans` | AWS Savings Plans | restJson1 | 10 | 0 |
| [scheduler](scheduler.md) | `scheduler` | Amazon EventBridge Scheduler | restJson1 | 12 | 2 |
| [schemas](schemas.md) | `schemas` | Schemas | restJson1 | 31 | 0 |
| [secrets-manager](secrets-manager.md) | `secretsmanager` | AWS Secrets Manager | awsJson1_1 | 23 | 0 |
| [security-ir](security-ir.md) | `securityir` | Security Incident Response | restJson1 | 24 | 2 |
| [securityhub](securityhub.md) | `securityhub` | AWS SecurityHub | restJson1 | 107 | 0 |
| [securitylake](securitylake.md) | `securitylake` | Amazon Security Lake | restJson1 | 31 | 2 |
| [serverlessapplicationrepository](serverlessapplicationrepository.md) | `serverlessapplicationrepository` | AWSServerlessApplicationRepository | restJson1 | 14 | 0 |
| [service-catalog](service-catalog.md) | `servicecatalog` | AWS Service Catalog | awsJson1_1 | 90 | 0 |
| [service-catalog-appregistry](service-catalog-appregistry.md) | `servicecatalogappregistry` | AWS Service Catalog App Registry | restJson1 | 24 | 0 |
| [service-quotas](service-quotas.md) | `servicequotas` | Service Quotas | awsJson1_1 | 26 | 0 |
| [servicediscovery](servicediscovery.md) | `servicediscovery` | AWS Cloud Map | awsJson1_1 | 30 | 0 |
| [ses](ses.md) | `ses` | Amazon Simple Email Service | awsQuery | 71 | 0 |
| [sesv2](sesv2.md) | `sesv2` | Amazon Simple Email Service | restJson1 | 110 | 0 |
| [sfn](sfn.md) | `sfn` | AWS Step Functions | awsJson1_0 | 37 | 0 |
| [shield](shield.md) | `shield` | AWS Shield | awsJson1_1 | 36 | 0 |
| [signer](signer.md) | `signer` | AWS Signer | restJson1 | 19 | 0 |
| [signer-data](signer-data.md) | `signer` | AWS Signer Data Plane | restJson1 | 1 | 0 |
| [signin](signin.md) | `signin` | AWS Sign-In Service | restJson1 | 1 | 0 |
| [simpledbv2](simpledbv2.md) | `simpledbv2` | Amazon SimpleDB v2 | restJson1 | 3 | 0 |
| [simspaceweaver](simspaceweaver.md) | `simspaceweaver` | AWS SimSpace Weaver | restJson1 | 16 | 1 |
| [snow-device-management](snow-device-management.md) | `snowdevicemanagement` | AWS Snow Device Management | restJson1 | 13 | 3 |
| [snowball](snowball.md) | `snowball` | Amazon Import/Export Snowball | awsJson1_1 | 27 | 0 |
| [sns](sns.md) | `sns` | Amazon Simple Notification Service | awsQuery | 42 | 0 |
| [socialmessaging](socialmessaging.md) | `socialmessaging` | AWS End User Messaging Social | restJson1 | 21 | 2 |
| [sqs](sqs.md) | `sqs` | Amazon Simple Queue Service | awsJson1_0, awsQueryCompatible | 23 | 0 |
| [ssm](ssm.md) | `ssm` | Amazon Simple Systems Manager (SSM) | awsJson1_1 | 146 | 0 |
| [ssm-contacts](ssm-contacts.md) | `ssmcontacts` | AWS Systems Manager Incident Manager Contacts | awsJson1_1 | 39 | 0 |
| [ssm-guiconnect](ssm-guiconnect.md) | `ssmguiconnect` | AWS SSM-GUIConnect | restJson1 | 3 | 7 |
| [ssm-incidents](ssm-incidents.md) | `ssmincidents` | AWS Systems Manager Incident Manager | restJson1 | 31 | 0 |
| [ssm-quicksetup](ssm-quicksetup.md) | `ssmquicksetup` | AWS Systems Manager QuickSetup | restJson1 | 14 | 0 |
| [ssm-sap](ssm-sap.md) | `ssmsap` | AWS Systems Manager for SAP | restJson1 | 27 | 0 |
| [sso](sso.md) | `sso` | AWS Single Sign-On | restJson1 | 4 | 0 |
| [sso-admin](sso-admin.md) | `ssoadmin` | AWS Single Sign-On Admin | awsJson1_1 | 79 | 3 |
| [sso-oidc](sso-oidc.md) | `ssooidc` | AWS SSO OIDC | restJson1 | 4 | 0 |
| [storage-gateway](storage-gateway.md) | `storagegateway` | AWS Storage Gateway | awsJson1_1 | 96 | 0 |
| [sts](sts.md) | `sts` | AWS Security Token Service | awsQuery | 11 | 0 |
| [supplychain](supplychain.md) | `supplychain` | AWS Supply Chain | restJson1 | 30 | 5 |
| [support](support.md) | `support` | AWS Support | awsJson1_1 | 16 | 0 |
| [support-app](support-app.md) | `supportapp` | AWS Support App | restJson1 | 10 | 0 |
| [swf](swf.md) | `swf` | Amazon Simple Workflow Service | awsJson1_0 | 39 | 0 |
| [synthetics](synthetics.md) | `synthetics` | Synthetics | restJson1 | 22 | 0 |
| [taxsettings](taxsettings.md) | `taxsettings` | Tax Settings | restJson1 | 16 | 0 |
| [textract](textract.md) | `textract` | Amazon Textract | awsJson1_1 | 25 | 0 |
| [timestream-influxdb](timestream-influxdb.md) | `timestreaminfluxdb` | Timestream InfluxDB | awsJson1_0 | 19 | 3 |
| [timestream-query](timestream-query.md) | `timestreamquery` | Amazon Timestream Query | awsJson1_0 | 15 | 0 |
| [timestream-write](timestream-write.md) | `timestreamwrite` | Amazon Timestream Write | awsJson1_0 | 19 | 0 |
| [tnb](tnb.md) | `tnb` | AWS Telco Network Builder | restJson1 | 33 | 0 |
| [transcribe](transcribe.md) | `transcribe` | Amazon Transcribe Service | awsJson1_1 | 43 | 0 |
| [transcribe-streaming](transcribe-streaming.md) | `transcribe` | Amazon Transcribe Streaming Service | restJson1 | 5 | 0 |
| [transfer](transfer.md) | `transfer` | AWS Transfer Family | awsJson1_1 | 71 | 9 |
| [translate](translate.md) | `translate` | Amazon Translate | awsJson1_1 | 19 | 0 |
| [trustedadvisor](trustedadvisor.md) | `trustedadvisor` | TrustedAdvisor Public API | restJson1 | 11 | 0 |
| [verifiedpermissions](verifiedpermissions.md) | `verifiedpermissions` | Amazon Verified Permissions | awsJson1_0 | 30 | 4 |
| [voice-id](voice-id.md) | `voiceid` | Amazon Voice ID | awsJson1_0 | 29 | 1 |
| [vpc-lattice](vpc-lattice.md) | `vpclattice` | Amazon VPC Lattice | restJson1 | 73 | 14 |
| [waf](waf.md) | `waf` | AWS WAF | awsJson1_1 | 77 | 0 |
| [waf-regional](waf-regional.md) | `wafregional` | AWS WAF Regional | awsJson1_1 | 81 | 0 |
| [wafv2](wafv2.md) | `wafv2` | AWS WAFV2 | awsJson1_1 | 55 | 0 |
| [wellarchitected](wellarchitected.md) | `wellarchitected` | AWS Well-Architected Tool | restJson1 | 72 | 0 |
| [wickr](wickr.md) | `wickr` | AWS Wickr Admin API | restJson1 | 44 | 0 |
| [wisdom](wisdom.md) | `wisdom` | Amazon Connect Wisdom Service | restJson1 | 41 | 6 |
| [workdocs](workdocs.md) | `workdocs` | Amazon WorkDocs | restJson1 | 44 | 0 |
| [workmail](workmail.md) | `workmail` | Amazon WorkMail | awsJson1_1 | 92 | 0 |
| [workmailmessageflow](workmailmessageflow.md) | `workmailmessageflow` | Amazon WorkMail Message Flow | restJson1 | 2 | 0 |
| [workspaces](workspaces.md) | `workspaces` | Amazon WorkSpaces | awsJson1_1 | 91 | 0 |
| [workspaces-instances](workspaces-instances.md) | `workspacesinstances` | Amazon Workspaces Instances | awsJson1_0 | 13 | 0 |
| [workspaces-thin-client](workspaces-thin-client.md) | `workspacesthinclient` | Amazon WorkSpaces Thin Client | restJson1 | 16 | 0 |
| [workspaces-web](workspaces-web.md) | `workspacesweb` | Amazon WorkSpaces Web | restJson1 | 75 | 10 |
| [xray](xray.md) | `xray` | AWS X-Ray | restJson1 | 38 | 0 |

MCP-enhanced service documents: accessanalyzer, account, acm, acm-pca, amplify, api-gateway, apigatewayv2, app-mesh, appconfig, appconfigdata, appfabric, appflow, appintegrations, application-auto-scaling, application-discovery-service, application-insights, application-signals, apprunner, appstream, appsync, arc-zonal-shift, artifact, athena, auditmanager, auto-scaling, backup, backup-gateway, batch, billing, billingconductor, braket, budgets, chatbot, chime, chime-sdk-identity, chime-sdk-media-pipelines, chime-sdk-meetings, chime-sdk-messaging, chime-sdk-voice, cloud9, cloudcontrol, clouddirectory, cloudformation, cloudfront, cloudhsm-v2, cloudsearch, cloudsearch-domain, cloudtrail, cloudwatch, cloudwatch-events, cloudwatch-logs, codeartifact, codebuild, codecatalyst, codecommit, codeconnections, codedeploy, codeguru-reviewer, codeguru-security, codepipeline, codestar-connections, codestar-notifications, cognito-identity, cognito-identity-provider, cognito-sync, comprehend, comprehendmedical, compute-optimizer, config-service, connect, connect-contact-lens, controltower, cost-and-usage-report-service, cost-explorer, cost-optimization-hub, customer-profiles, database-migration-service, databrew, dataexchange, datasync, datazone, dax, detective, device-farm, direct-connect, directory-service, dlm, docdb, docdb-elastic, drs, dsql, dynamodb, dynamodb-streams, ebs, ec2, ecr, ecr-public, ecs, efs, eks, elastic-beanstalk, elastic-load-balancing, elastic-load-balancing-v2, elasticache, elasticsearch-service, emr, eventbridge, firehose, fis, fsx, glacier, global-accelerator, glue, guardduty, health, iam, identitystore, inspector2, iot, kafka, kendra, keyspaces, kinesis, kinesis-analytics-v2, kinesis-video, kms, lakeformation, lambda, license-manager, lightsail, location, macie2, mq, mwaa, neptune, network-firewall, opensearch, opensearchserverless, organizations, outposts, ram, rds, rds-data, redshift, redshift-data, redshift-serverless, resource-groups, rolesanywhere, route-53, s3, scheduler, secrets-manager, sfn, sns, sqs, ssm, sso-admin, sso-oidc, sts, wafv2, xray.
