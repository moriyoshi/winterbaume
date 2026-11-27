# Changelog

## 2026-11-27

- S3 stores `aws-chunked` uploads decoded and verifies their trailing checksums ( #84 ), so pyarrow and pyiceberg read back what they wrote.
- Opt-in STS session policy enforcement in S3 ( #85 ): `winterbaume-server --s3-enforce-session-policies` limits temporary credentials to the `Policy` they were issued with.
- The in-process client delivers streaming request bodies ( #88 ), so Rust SDK uploads from a file are no longer stored empty.
- `winterbaume-core` changes nothing public, so it is a patch release ( v0.3.2 ) and its dependants' releases are dependency-only. Breaking releases: `winterbaume-s3` v2.0.0, and new 0.y minors for STS, `winterbaume-s3control` and `winterbaume-terraform`.

- `winterbaume-accessanalyzer` v0.2.3: dependency-only. See [`crates/winterbaume-accessanalyzer/CHANGELOG.md`](crates/winterbaume-accessanalyzer/CHANGELOG.md).
- `winterbaume-account` v1.0.3: dependency-only. See [`crates/winterbaume-account/CHANGELOG.md`](crates/winterbaume-account/CHANGELOG.md).
- `winterbaume-acm` v0.2.3: dependency-only. See [`crates/winterbaume-acm/CHANGELOG.md`](crates/winterbaume-acm/CHANGELOG.md).
- `winterbaume-acmpca` v0.2.3: dependency-only. See [`crates/winterbaume-acmpca/CHANGELOG.md`](crates/winterbaume-acmpca/CHANGELOG.md).
- `winterbaume-aiops` v0.2.3: dependency-only. See [`crates/winterbaume-aiops/CHANGELOG.md`](crates/winterbaume-aiops/CHANGELOG.md).
- `winterbaume-amp` v0.2.3: dependency-only. See [`crates/winterbaume-amp/CHANGELOG.md`](crates/winterbaume-amp/CHANGELOG.md).
- `winterbaume-amplify` v0.2.3: dependency-only. See [`crates/winterbaume-amplify/CHANGELOG.md`](crates/winterbaume-amplify/CHANGELOG.md).
- `winterbaume-amplifybackend` v0.2.3: dependency-only. See [`crates/winterbaume-amplifybackend/CHANGELOG.md`](crates/winterbaume-amplifybackend/CHANGELOG.md).
- `winterbaume-amplifyuibuilder` v0.2.3: dependency-only. See [`crates/winterbaume-amplifyuibuilder/CHANGELOG.md`](crates/winterbaume-amplifyuibuilder/CHANGELOG.md).
- `winterbaume-apigateway` v1.0.3: dependency-only. See [`crates/winterbaume-apigateway/CHANGELOG.md`](crates/winterbaume-apigateway/CHANGELOG.md).
- `winterbaume-apigatewaymanagement` v0.2.3: dependency-only. See [`crates/winterbaume-apigatewaymanagement/CHANGELOG.md`](crates/winterbaume-apigatewaymanagement/CHANGELOG.md).
- `winterbaume-apigatewayv2` v0.2.3: dependency-only. See [`crates/winterbaume-apigatewayv2/CHANGELOG.md`](crates/winterbaume-apigatewayv2/CHANGELOG.md).
- `winterbaume-appconfig` v2.0.3: dependency-only. See [`crates/winterbaume-appconfig/CHANGELOG.md`](crates/winterbaume-appconfig/CHANGELOG.md).
- `winterbaume-appconfigdata` v0.3.4: dependency-only. See [`crates/winterbaume-appconfigdata/CHANGELOG.md`](crates/winterbaume-appconfigdata/CHANGELOG.md).
- `winterbaume-appfabric` v0.2.3: dependency-only. See [`crates/winterbaume-appfabric/CHANGELOG.md`](crates/winterbaume-appfabric/CHANGELOG.md).
- `winterbaume-appflow` v0.2.3: dependency-only. See [`crates/winterbaume-appflow/CHANGELOG.md`](crates/winterbaume-appflow/CHANGELOG.md).
- `winterbaume-appintegrations` v0.2.3: dependency-only. See [`crates/winterbaume-appintegrations/CHANGELOG.md`](crates/winterbaume-appintegrations/CHANGELOG.md).
- `winterbaume-applicationautoscaling` v0.2.3: dependency-only. See [`crates/winterbaume-applicationautoscaling/CHANGELOG.md`](crates/winterbaume-applicationautoscaling/CHANGELOG.md).
- `winterbaume-applicationcostprofiler` v0.2.3: dependency-only. See [`crates/winterbaume-applicationcostprofiler/CHANGELOG.md`](crates/winterbaume-applicationcostprofiler/CHANGELOG.md).
- `winterbaume-applicationdiscoveryservice` v0.2.3: dependency-only. See [`crates/winterbaume-applicationdiscoveryservice/CHANGELOG.md`](crates/winterbaume-applicationdiscoveryservice/CHANGELOG.md).
- `winterbaume-applicationinsights` v0.2.3: dependency-only. See [`crates/winterbaume-applicationinsights/CHANGELOG.md`](crates/winterbaume-applicationinsights/CHANGELOG.md).
- `winterbaume-applicationsignals` v1.0.3: dependency-only. See [`crates/winterbaume-applicationsignals/CHANGELOG.md`](crates/winterbaume-applicationsignals/CHANGELOG.md).
- `winterbaume-appmesh` v0.2.3: dependency-only. See [`crates/winterbaume-appmesh/CHANGELOG.md`](crates/winterbaume-appmesh/CHANGELOG.md).
- `winterbaume-apprunner` v0.2.3: dependency-only. See [`crates/winterbaume-apprunner/CHANGELOG.md`](crates/winterbaume-apprunner/CHANGELOG.md).
- `winterbaume-appsync` v0.2.3: dependency-only. See [`crates/winterbaume-appsync/CHANGELOG.md`](crates/winterbaume-appsync/CHANGELOG.md).
- `winterbaume-arczonalshift` v0.2.3: dependency-only. See [`crates/winterbaume-arczonalshift/CHANGELOG.md`](crates/winterbaume-arczonalshift/CHANGELOG.md).
- `winterbaume-artifact` v0.2.3: dependency-only. See [`crates/winterbaume-artifact/CHANGELOG.md`](crates/winterbaume-artifact/CHANGELOG.md).
- `winterbaume-athena` v0.2.3: dependency-only. See [`crates/winterbaume-athena/CHANGELOG.md`](crates/winterbaume-athena/CHANGELOG.md).
- `winterbaume-auditmanager` v0.2.3: dependency-only. See [`crates/winterbaume-auditmanager/CHANGELOG.md`](crates/winterbaume-auditmanager/CHANGELOG.md).
- `winterbaume-autoscaling` v0.2.4: dependency-only. See [`crates/winterbaume-autoscaling/CHANGELOG.md`](crates/winterbaume-autoscaling/CHANGELOG.md).
- `winterbaume-autoscalingplans` v0.2.3: dependency-only. See [`crates/winterbaume-autoscalingplans/CHANGELOG.md`](crates/winterbaume-autoscalingplans/CHANGELOG.md).
- `winterbaume-backup` v0.2.3: dependency-only. See [`crates/winterbaume-backup/CHANGELOG.md`](crates/winterbaume-backup/CHANGELOG.md).
- `winterbaume-backupgateway` v0.2.3: dependency-only. See [`crates/winterbaume-backupgateway/CHANGELOG.md`](crates/winterbaume-backupgateway/CHANGELOG.md).
- `winterbaume-backupsearch` v0.2.3: dependency-only. See [`crates/winterbaume-backupsearch/CHANGELOG.md`](crates/winterbaume-backupsearch/CHANGELOG.md).
- `winterbaume-batch` v1.0.3: dependency-only. See [`crates/winterbaume-batch/CHANGELOG.md`](crates/winterbaume-batch/CHANGELOG.md).
- `winterbaume-bcmdashboards` v0.2.3: dependency-only. See [`crates/winterbaume-bcmdashboards/CHANGELOG.md`](crates/winterbaume-bcmdashboards/CHANGELOG.md).
- `winterbaume-bcmdataexports` v0.2.3: dependency-only. See [`crates/winterbaume-bcmdataexports/CHANGELOG.md`](crates/winterbaume-bcmdataexports/CHANGELOG.md).
- `winterbaume-bcmrecommendedactions` v0.2.3: dependency-only. See [`crates/winterbaume-bcmrecommendedactions/CHANGELOG.md`](crates/winterbaume-bcmrecommendedactions/CHANGELOG.md).
- `winterbaume-bedrock` v0.2.3: dependency-only. See [`crates/winterbaume-bedrock/CHANGELOG.md`](crates/winterbaume-bedrock/CHANGELOG.md).
- `winterbaume-bedrockagent` v0.2.3: dependency-only. See [`crates/winterbaume-bedrockagent/CHANGELOG.md`](crates/winterbaume-bedrockagent/CHANGELOG.md).
- `winterbaume-billing` v0.2.3: dependency-only. See [`crates/winterbaume-billing/CHANGELOG.md`](crates/winterbaume-billing/CHANGELOG.md).
- `winterbaume-braket` v0.2.3: dependency-only. See [`crates/winterbaume-braket/CHANGELOG.md`](crates/winterbaume-braket/CHANGELOG.md).
- `winterbaume-budgets` v0.2.3: dependency-only. See [`crates/winterbaume-budgets/CHANGELOG.md`](crates/winterbaume-budgets/CHANGELOG.md).
- `winterbaume-chatbot` v0.2.3: dependency-only. See [`crates/winterbaume-chatbot/CHANGELOG.md`](crates/winterbaume-chatbot/CHANGELOG.md).
- `winterbaume-chimesdkmeetings` v0.2.3: dependency-only. See [`crates/winterbaume-chimesdkmeetings/CHANGELOG.md`](crates/winterbaume-chimesdkmeetings/CHANGELOG.md).
- `winterbaume-cloud9` v0.2.3: dependency-only. See [`crates/winterbaume-cloud9/CHANGELOG.md`](crates/winterbaume-cloud9/CHANGELOG.md).
- `winterbaume-cloudcontrol` v1.2.2: dependency-only. See [`crates/winterbaume-cloudcontrol/CHANGELOG.md`](crates/winterbaume-cloudcontrol/CHANGELOG.md).
- `winterbaume-clouddirectory` v0.2.3: dependency-only. See [`crates/winterbaume-clouddirectory/CHANGELOG.md`](crates/winterbaume-clouddirectory/CHANGELOG.md).
- `winterbaume-cloudformation` v0.2.3: dependency-only. See [`crates/winterbaume-cloudformation/CHANGELOG.md`](crates/winterbaume-cloudformation/CHANGELOG.md).
- `winterbaume-cloudfront` v1.1.2: dependency-only. See [`crates/winterbaume-cloudfront/CHANGELOG.md`](crates/winterbaume-cloudfront/CHANGELOG.md).
- `winterbaume-cloudfrontkeyvaluestore` v0.2.4: dependency-only. See [`crates/winterbaume-cloudfrontkeyvaluestore/CHANGELOG.md`](crates/winterbaume-cloudfrontkeyvaluestore/CHANGELOG.md).
- `winterbaume-cloudhsmv2` v0.2.3: dependency-only. See [`crates/winterbaume-cloudhsmv2/CHANGELOG.md`](crates/winterbaume-cloudhsmv2/CHANGELOG.md).
- `winterbaume-cloudsearchdomain` v0.2.3: dependency-only. See [`crates/winterbaume-cloudsearchdomain/CHANGELOG.md`](crates/winterbaume-cloudsearchdomain/CHANGELOG.md).
- `winterbaume-cloudtrail` v0.2.3: dependency-only. See [`crates/winterbaume-cloudtrail/CHANGELOG.md`](crates/winterbaume-cloudtrail/CHANGELOG.md).
- `winterbaume-cloudtraildata` v0.2.3: dependency-only. See [`crates/winterbaume-cloudtraildata/CHANGELOG.md`](crates/winterbaume-cloudtraildata/CHANGELOG.md).
- `winterbaume-cloudwatch` v0.3.1: dependency-only. See [`crates/winterbaume-cloudwatch/CHANGELOG.md`](crates/winterbaume-cloudwatch/CHANGELOG.md).
- `winterbaume-cloudwatchlogs` v0.2.3: dependency-only. See [`crates/winterbaume-cloudwatchlogs/CHANGELOG.md`](crates/winterbaume-cloudwatchlogs/CHANGELOG.md).
- `winterbaume-codeartifact` v0.2.3: dependency-only. See [`crates/winterbaume-codeartifact/CHANGELOG.md`](crates/winterbaume-codeartifact/CHANGELOG.md).
- `winterbaume-codebuild` v0.2.3: dependency-only. See [`crates/winterbaume-codebuild/CHANGELOG.md`](crates/winterbaume-codebuild/CHANGELOG.md).
- `winterbaume-codecommit` v0.2.3: dependency-only. See [`crates/winterbaume-codecommit/CHANGELOG.md`](crates/winterbaume-codecommit/CHANGELOG.md).
- `winterbaume-codedeploy` v0.2.4: dependency-only. See [`crates/winterbaume-codedeploy/CHANGELOG.md`](crates/winterbaume-codedeploy/CHANGELOG.md).
- `winterbaume-codegurureviewer` v0.2.3: dependency-only. See [`crates/winterbaume-codegurureviewer/CHANGELOG.md`](crates/winterbaume-codegurureviewer/CHANGELOG.md).
- `winterbaume-codegurusecurity` v0.2.3: dependency-only. See [`crates/winterbaume-codegurusecurity/CHANGELOG.md`](crates/winterbaume-codegurusecurity/CHANGELOG.md).
- `winterbaume-codepipeline` v0.2.3: dependency-only. See [`crates/winterbaume-codepipeline/CHANGELOG.md`](crates/winterbaume-codepipeline/CHANGELOG.md).
- `winterbaume-codestarnotifications` v0.2.3: dependency-only. See [`crates/winterbaume-codestarnotifications/CHANGELOG.md`](crates/winterbaume-codestarnotifications/CHANGELOG.md).
- `winterbaume-cognitoidentity` v0.2.3: dependency-only. See [`crates/winterbaume-cognitoidentity/CHANGELOG.md`](crates/winterbaume-cognitoidentity/CHANGELOG.md).
- `winterbaume-cognitoidentityprovider` v0.2.3: dependency-only. See [`crates/winterbaume-cognitoidentityprovider/CHANGELOG.md`](crates/winterbaume-cognitoidentityprovider/CHANGELOG.md).
- `winterbaume-cognitosync` v0.2.3: dependency-only. See [`crates/winterbaume-cognitosync/CHANGELOG.md`](crates/winterbaume-cognitosync/CHANGELOG.md).
- `winterbaume-comprehend` v0.2.3: dependency-only. See [`crates/winterbaume-comprehend/CHANGELOG.md`](crates/winterbaume-comprehend/CHANGELOG.md).
- `winterbaume-config` v0.2.3: dependency-only. See [`crates/winterbaume-config/CHANGELOG.md`](crates/winterbaume-config/CHANGELOG.md).
- `winterbaume-connect` v0.2.3: dependency-only. See [`crates/winterbaume-connect/CHANGELOG.md`](crates/winterbaume-connect/CHANGELOG.md).
- `winterbaume-connectcampaigns` v0.2.3: dependency-only. See [`crates/winterbaume-connectcampaigns/CHANGELOG.md`](crates/winterbaume-connectcampaigns/CHANGELOG.md).
- `winterbaume-connectcontactlens` v0.2.3: dependency-only. See [`crates/winterbaume-connectcontactlens/CHANGELOG.md`](crates/winterbaume-connectcontactlens/CHANGELOG.md).
- `winterbaume-connectparticipant` v0.2.3: dependency-only. See [`crates/winterbaume-connectparticipant/CHANGELOG.md`](crates/winterbaume-connectparticipant/CHANGELOG.md).
- `winterbaume-controlcatalog` v0.2.3: dependency-only. See [`crates/winterbaume-controlcatalog/CHANGELOG.md`](crates/winterbaume-controlcatalog/CHANGELOG.md).
- `winterbaume-core` v0.3.2: the in-process client delivers streaming request bodies (#88). See [`crates/winterbaume-core/CHANGELOG.md`](crates/winterbaume-core/CHANGELOG.md).
- `winterbaume-costandusagereport` v0.2.3: dependency-only. See [`crates/winterbaume-costandusagereport/CHANGELOG.md`](crates/winterbaume-costandusagereport/CHANGELOG.md).
- `winterbaume-costexplorer` v0.2.3: dependency-only. See [`crates/winterbaume-costexplorer/CHANGELOG.md`](crates/winterbaume-costexplorer/CHANGELOG.md).
- `winterbaume-costoptimizationhub` v0.2.3: dependency-only. See [`crates/winterbaume-costoptimizationhub/CHANGELOG.md`](crates/winterbaume-costoptimizationhub/CHANGELOG.md).
- `winterbaume-databasemigration` v0.2.3: dependency-only. See [`crates/winterbaume-databasemigration/CHANGELOG.md`](crates/winterbaume-databasemigration/CHANGELOG.md).
- `winterbaume-databrew` v0.2.3: dependency-only. See [`crates/winterbaume-databrew/CHANGELOG.md`](crates/winterbaume-databrew/CHANGELOG.md).
- `winterbaume-datapipeline` v0.2.3: dependency-only. See [`crates/winterbaume-datapipeline/CHANGELOG.md`](crates/winterbaume-datapipeline/CHANGELOG.md).
- `winterbaume-datasync` v0.2.3: dependency-only. See [`crates/winterbaume-datasync/CHANGELOG.md`](crates/winterbaume-datasync/CHANGELOG.md).
- `winterbaume-dax` v0.2.3: dependency-only. See [`crates/winterbaume-dax/CHANGELOG.md`](crates/winterbaume-dax/CHANGELOG.md).
- `winterbaume-directconnect` v0.2.3: dependency-only. See [`crates/winterbaume-directconnect/CHANGELOG.md`](crates/winterbaume-directconnect/CHANGELOG.md).
- `winterbaume-directory` v0.2.3: dependency-only. See [`crates/winterbaume-directory/CHANGELOG.md`](crates/winterbaume-directory/CHANGELOG.md).
- `winterbaume-dlm` v0.2.3: dependency-only. See [`crates/winterbaume-dlm/CHANGELOG.md`](crates/winterbaume-dlm/CHANGELOG.md).
- `winterbaume-dsql` v0.2.3: dependency-only. See [`crates/winterbaume-dsql/CHANGELOG.md`](crates/winterbaume-dsql/CHANGELOG.md).
- `winterbaume-dynamodb` v0.5.1: dependency-only. See [`crates/winterbaume-dynamodb/CHANGELOG.md`](crates/winterbaume-dynamodb/CHANGELOG.md).
- `winterbaume-dynamodb-redis` v0.3.1: dependency-only. See [`crates/winterbaume-dynamodb-redis/CHANGELOG.md`](crates/winterbaume-dynamodb-redis/CHANGELOG.md).
- `winterbaume-dynamodbstreams` v0.3.1: dependency-only. See [`crates/winterbaume-dynamodbstreams/CHANGELOG.md`](crates/winterbaume-dynamodbstreams/CHANGELOG.md).
- `winterbaume-ebs` v0.2.3: dependency-only. See [`crates/winterbaume-ebs/CHANGELOG.md`](crates/winterbaume-ebs/CHANGELOG.md).
- `winterbaume-ec2` v1.0.3: dependency-only. See [`crates/winterbaume-ec2/CHANGELOG.md`](crates/winterbaume-ec2/CHANGELOG.md).
- `winterbaume-ec2-generated` v0.2.3: dependency-only. See [`crates/winterbaume-ec2-generated/CHANGELOG.md`](crates/winterbaume-ec2-generated/CHANGELOG.md).
- `winterbaume-ec2instanceconnect` v0.2.3: dependency-only. See [`crates/winterbaume-ec2instanceconnect/CHANGELOG.md`](crates/winterbaume-ec2instanceconnect/CHANGELOG.md).
- `winterbaume-ecr` v0.2.3: dependency-only. See [`crates/winterbaume-ecr/CHANGELOG.md`](crates/winterbaume-ecr/CHANGELOG.md).
- `winterbaume-ecs` v0.2.3: dependency-only. See [`crates/winterbaume-ecs/CHANGELOG.md`](crates/winterbaume-ecs/CHANGELOG.md).
- `winterbaume-efs` v0.2.3: dependency-only. See [`crates/winterbaume-efs/CHANGELOG.md`](crates/winterbaume-efs/CHANGELOG.md).
- `winterbaume-eks` v0.2.3: dependency-only. See [`crates/winterbaume-eks/CHANGELOG.md`](crates/winterbaume-eks/CHANGELOG.md).
- `winterbaume-elasticache` v0.2.3: dependency-only. See [`crates/winterbaume-elasticache/CHANGELOG.md`](crates/winterbaume-elasticache/CHANGELOG.md).
- `winterbaume-elasticbeanstalk` v0.2.3: dependency-only. See [`crates/winterbaume-elasticbeanstalk/CHANGELOG.md`](crates/winterbaume-elasticbeanstalk/CHANGELOG.md).
- `winterbaume-elasticloadbalancing` v0.2.3: dependency-only. See [`crates/winterbaume-elasticloadbalancing/CHANGELOG.md`](crates/winterbaume-elasticloadbalancing/CHANGELOG.md).
- `winterbaume-elasticloadbalancingv2` v0.2.3: dependency-only. See [`crates/winterbaume-elasticloadbalancingv2/CHANGELOG.md`](crates/winterbaume-elasticloadbalancingv2/CHANGELOG.md).
- `winterbaume-emr` v0.2.3: dependency-only. See [`crates/winterbaume-emr/CHANGELOG.md`](crates/winterbaume-emr/CHANGELOG.md).
- `winterbaume-emrcontainers` v0.2.3: dependency-only. See [`crates/winterbaume-emrcontainers/CHANGELOG.md`](crates/winterbaume-emrcontainers/CHANGELOG.md).
- `winterbaume-emrserverless` v0.2.3: dependency-only. See [`crates/winterbaume-emrserverless/CHANGELOG.md`](crates/winterbaume-emrserverless/CHANGELOG.md).
- `winterbaume-eventbridge` v0.3.1: dependency-only. See [`crates/winterbaume-eventbridge/CHANGELOG.md`](crates/winterbaume-eventbridge/CHANGELOG.md).
- `winterbaume-firehose` v0.2.3: dependency-only. See [`crates/winterbaume-firehose/CHANGELOG.md`](crates/winterbaume-firehose/CHANGELOG.md).
- `winterbaume-fis` v0.2.4: dependency-only. See [`crates/winterbaume-fis/CHANGELOG.md`](crates/winterbaume-fis/CHANGELOG.md).
- `winterbaume-forecast` v0.2.3: dependency-only. See [`crates/winterbaume-forecast/CHANGELOG.md`](crates/winterbaume-forecast/CHANGELOG.md).
- `winterbaume-freetier` v0.2.3: dependency-only. See [`crates/winterbaume-freetier/CHANGELOG.md`](crates/winterbaume-freetier/CHANGELOG.md).
- `winterbaume-fsx` v0.2.3: dependency-only. See [`crates/winterbaume-fsx/CHANGELOG.md`](crates/winterbaume-fsx/CHANGELOG.md).
- `winterbaume-glacier` v0.2.3: dependency-only. See [`crates/winterbaume-glacier/CHANGELOG.md`](crates/winterbaume-glacier/CHANGELOG.md).
- `winterbaume-glue` v0.2.3: dependency-only. See [`crates/winterbaume-glue/CHANGELOG.md`](crates/winterbaume-glue/CHANGELOG.md).
- `winterbaume-greengrass` v0.2.3: dependency-only. See [`crates/winterbaume-greengrass/CHANGELOG.md`](crates/winterbaume-greengrass/CHANGELOG.md).
- `winterbaume-guardduty` v0.2.3: dependency-only. See [`crates/winterbaume-guardduty/CHANGELOG.md`](crates/winterbaume-guardduty/CHANGELOG.md).
- `winterbaume-iam` v0.2.4: dependency-only. See [`crates/winterbaume-iam/CHANGELOG.md`](crates/winterbaume-iam/CHANGELOG.md).
- `winterbaume-identitystore` v0.2.3: dependency-only. See [`crates/winterbaume-identitystore/CHANGELOG.md`](crates/winterbaume-identitystore/CHANGELOG.md).
- `winterbaume-inspector2` v0.2.3: dependency-only. See [`crates/winterbaume-inspector2/CHANGELOG.md`](crates/winterbaume-inspector2/CHANGELOG.md).
- `winterbaume-iot` v0.2.3: dependency-only. See [`crates/winterbaume-iot/CHANGELOG.md`](crates/winterbaume-iot/CHANGELOG.md).
- `winterbaume-iotdataplane` v0.2.3: dependency-only. See [`crates/winterbaume-iotdataplane/CHANGELOG.md`](crates/winterbaume-iotdataplane/CHANGELOG.md).
- `winterbaume-ivs` v1.0.3: dependency-only. See [`crates/winterbaume-ivs/CHANGELOG.md`](crates/winterbaume-ivs/CHANGELOG.md).
- `winterbaume-kafka` v0.2.3: dependency-only. See [`crates/winterbaume-kafka/CHANGELOG.md`](crates/winterbaume-kafka/CHANGELOG.md).
- `winterbaume-keyspaces` v0.2.4: dependency-only. See [`crates/winterbaume-keyspaces/CHANGELOG.md`](crates/winterbaume-keyspaces/CHANGELOG.md).
- `winterbaume-kinesis` v0.2.4: dependency-only. See [`crates/winterbaume-kinesis/CHANGELOG.md`](crates/winterbaume-kinesis/CHANGELOG.md).
- `winterbaume-kinesisanalyticsv2` v0.2.3: dependency-only. See [`crates/winterbaume-kinesisanalyticsv2/CHANGELOG.md`](crates/winterbaume-kinesisanalyticsv2/CHANGELOG.md).
- `winterbaume-kinesisvideo` v0.3.3: dependency-only. See [`crates/winterbaume-kinesisvideo/CHANGELOG.md`](crates/winterbaume-kinesisvideo/CHANGELOG.md).
- `winterbaume-kinesisvideoarchivedmedia` v0.3.3: dependency-only. See [`crates/winterbaume-kinesisvideoarchivedmedia/CHANGELOG.md`](crates/winterbaume-kinesisvideoarchivedmedia/CHANGELOG.md).
- `winterbaume-kms` v0.2.4: dependency-only. See [`crates/winterbaume-kms/CHANGELOG.md`](crates/winterbaume-kms/CHANGELOG.md).
- `winterbaume-lakeformation` v0.2.3: dependency-only. See [`crates/winterbaume-lakeformation/CHANGELOG.md`](crates/winterbaume-lakeformation/CHANGELOG.md).
- `winterbaume-lambda` v3.0.1: dependency-only. See [`crates/winterbaume-lambda/CHANGELOG.md`](crates/winterbaume-lambda/CHANGELOG.md).
- `winterbaume-lexmodelsv2` v0.2.3: dependency-only. See [`crates/winterbaume-lexmodelsv2/CHANGELOG.md`](crates/winterbaume-lexmodelsv2/CHANGELOG.md).
- `winterbaume-macie2` v0.2.3: dependency-only. See [`crates/winterbaume-macie2/CHANGELOG.md`](crates/winterbaume-macie2/CHANGELOG.md).
- `winterbaume-managedblockchain` v0.2.4: dependency-only. See [`crates/winterbaume-managedblockchain/CHANGELOG.md`](crates/winterbaume-managedblockchain/CHANGELOG.md).
- `winterbaume-marketplacemetering` v0.2.3: dependency-only. See [`crates/winterbaume-marketplacemetering/CHANGELOG.md`](crates/winterbaume-marketplacemetering/CHANGELOG.md).
- `winterbaume-mediaconnect` v0.2.3: dependency-only. See [`crates/winterbaume-mediaconnect/CHANGELOG.md`](crates/winterbaume-mediaconnect/CHANGELOG.md).
- `winterbaume-medialive` v0.2.3: dependency-only. See [`crates/winterbaume-medialive/CHANGELOG.md`](crates/winterbaume-medialive/CHANGELOG.md).
- `winterbaume-mediapackage` v0.2.3: dependency-only. See [`crates/winterbaume-mediapackage/CHANGELOG.md`](crates/winterbaume-mediapackage/CHANGELOG.md).
- `winterbaume-mediapackagev2` v0.2.3: dependency-only. See [`crates/winterbaume-mediapackagev2/CHANGELOG.md`](crates/winterbaume-mediapackagev2/CHANGELOG.md).
- `winterbaume-mediastore` v0.2.3: dependency-only. See [`crates/winterbaume-mediastore/CHANGELOG.md`](crates/winterbaume-mediastore/CHANGELOG.md).
- `winterbaume-mediastoredata` v0.2.3: dependency-only. See [`crates/winterbaume-mediastoredata/CHANGELOG.md`](crates/winterbaume-mediastoredata/CHANGELOG.md).
- `winterbaume-memorydb` v0.2.3: dependency-only. See [`crates/winterbaume-memorydb/CHANGELOG.md`](crates/winterbaume-memorydb/CHANGELOG.md).
- `winterbaume-mq` v0.2.3: dependency-only. See [`crates/winterbaume-mq/CHANGELOG.md`](crates/winterbaume-mq/CHANGELOG.md).
- `winterbaume-neptune` v0.2.3: dependency-only. See [`crates/winterbaume-neptune/CHANGELOG.md`](crates/winterbaume-neptune/CHANGELOG.md).
- `winterbaume-networkfirewall` v0.2.3: dependency-only. See [`crates/winterbaume-networkfirewall/CHANGELOG.md`](crates/winterbaume-networkfirewall/CHANGELOG.md).
- `winterbaume-networkmanager` v0.2.3: dependency-only. See [`crates/winterbaume-networkmanager/CHANGELOG.md`](crates/winterbaume-networkmanager/CHANGELOG.md).
- `winterbaume-opensearch` v1.0.3: dependency-only. See [`crates/winterbaume-opensearch/CHANGELOG.md`](crates/winterbaume-opensearch/CHANGELOG.md).
- `winterbaume-opensearchserverless` v0.2.4: dependency-only. See [`crates/winterbaume-opensearchserverless/CHANGELOG.md`](crates/winterbaume-opensearchserverless/CHANGELOG.md).
- `winterbaume-organizations` v1.0.3: dependency-only. See [`crates/winterbaume-organizations/CHANGELOG.md`](crates/winterbaume-organizations/CHANGELOG.md).
- `winterbaume-osis` v0.2.3: dependency-only. See [`crates/winterbaume-osis/CHANGELOG.md`](crates/winterbaume-osis/CHANGELOG.md).
- `winterbaume-outposts` v0.2.3: dependency-only. See [`crates/winterbaume-outposts/CHANGELOG.md`](crates/winterbaume-outposts/CHANGELOG.md).
- `winterbaume-panorama` v0.2.3: dependency-only. See [`crates/winterbaume-panorama/CHANGELOG.md`](crates/winterbaume-panorama/CHANGELOG.md).
- `winterbaume-pcaconnectorscep` v0.2.3: dependency-only. See [`crates/winterbaume-pcaconnectorscep/CHANGELOG.md`](crates/winterbaume-pcaconnectorscep/CHANGELOG.md).
- `winterbaume-personalize` v0.2.3: dependency-only. See [`crates/winterbaume-personalize/CHANGELOG.md`](crates/winterbaume-personalize/CHANGELOG.md).
- `winterbaume-personalizeevents` v0.2.3: dependency-only. See [`crates/winterbaume-personalizeevents/CHANGELOG.md`](crates/winterbaume-personalizeevents/CHANGELOG.md).
- `winterbaume-personalizeruntime` v0.2.3: dependency-only. See [`crates/winterbaume-personalizeruntime/CHANGELOG.md`](crates/winterbaume-personalizeruntime/CHANGELOG.md).
- `winterbaume-pi` v0.2.3: dependency-only. See [`crates/winterbaume-pi/CHANGELOG.md`](crates/winterbaume-pi/CHANGELOG.md).
- `winterbaume-pinpoint` v0.2.3: dependency-only. See [`crates/winterbaume-pinpoint/CHANGELOG.md`](crates/winterbaume-pinpoint/CHANGELOG.md).
- `winterbaume-pinpointsmsvoice` v0.2.3: dependency-only. See [`crates/winterbaume-pinpointsmsvoice/CHANGELOG.md`](crates/winterbaume-pinpointsmsvoice/CHANGELOG.md).
- `winterbaume-pipes` v0.2.3: dependency-only. See [`crates/winterbaume-pipes/CHANGELOG.md`](crates/winterbaume-pipes/CHANGELOG.md).
- `winterbaume-polly` v0.2.3: dependency-only. See [`crates/winterbaume-polly/CHANGELOG.md`](crates/winterbaume-polly/CHANGELOG.md).
- `winterbaume-pricing` v0.2.3: dependency-only. See [`crates/winterbaume-pricing/CHANGELOG.md`](crates/winterbaume-pricing/CHANGELOG.md).
- `winterbaume-quicksight` v0.2.3: dependency-only. See [`crates/winterbaume-quicksight/CHANGELOG.md`](crates/winterbaume-quicksight/CHANGELOG.md).
- `winterbaume-ram` v0.2.3: dependency-only. See [`crates/winterbaume-ram/CHANGELOG.md`](crates/winterbaume-ram/CHANGELOG.md).
- `winterbaume-rbin` v0.2.3: dependency-only. See [`crates/winterbaume-rbin/CHANGELOG.md`](crates/winterbaume-rbin/CHANGELOG.md).
- `winterbaume-rds` v0.2.3: dependency-only. See [`crates/winterbaume-rds/CHANGELOG.md`](crates/winterbaume-rds/CHANGELOG.md).
- `winterbaume-rdsdata` v0.2.3: dependency-only. See [`crates/winterbaume-rdsdata/CHANGELOG.md`](crates/winterbaume-rdsdata/CHANGELOG.md).
- `winterbaume-redshift` v0.2.3: dependency-only. See [`crates/winterbaume-redshift/CHANGELOG.md`](crates/winterbaume-redshift/CHANGELOG.md).
- `winterbaume-redshiftdata` v0.2.3: dependency-only. See [`crates/winterbaume-redshiftdata/CHANGELOG.md`](crates/winterbaume-redshiftdata/CHANGELOG.md).
- `winterbaume-rekognition` v0.2.3: dependency-only. See [`crates/winterbaume-rekognition/CHANGELOG.md`](crates/winterbaume-rekognition/CHANGELOG.md).
- `winterbaume-resiliencehub` v0.2.3: dependency-only. See [`crates/winterbaume-resiliencehub/CHANGELOG.md`](crates/winterbaume-resiliencehub/CHANGELOG.md).
- `winterbaume-resourcegroups` v0.2.3: dependency-only. See [`crates/winterbaume-resourcegroups/CHANGELOG.md`](crates/winterbaume-resourcegroups/CHANGELOG.md).
- `winterbaume-resourcegroupstagging` v0.2.3: dependency-only. See [`crates/winterbaume-resourcegroupstagging/CHANGELOG.md`](crates/winterbaume-resourcegroupstagging/CHANGELOG.md).
- `winterbaume-rolesanywhere` v0.2.3: dependency-only. See [`crates/winterbaume-rolesanywhere/CHANGELOG.md`](crates/winterbaume-rolesanywhere/CHANGELOG.md).
- `winterbaume-route53` v0.2.3: dependency-only. See [`crates/winterbaume-route53/CHANGELOG.md`](crates/winterbaume-route53/CHANGELOG.md).
- `winterbaume-route53domains` v0.2.3: dependency-only. See [`crates/winterbaume-route53domains/CHANGELOG.md`](crates/winterbaume-route53domains/CHANGELOG.md).
- `winterbaume-route53recoverycluster` v0.2.3: dependency-only. See [`crates/winterbaume-route53recoverycluster/CHANGELOG.md`](crates/winterbaume-route53recoverycluster/CHANGELOG.md).
- `winterbaume-route53resolver` v0.2.3: dependency-only. See [`crates/winterbaume-route53resolver/CHANGELOG.md`](crates/winterbaume-route53resolver/CHANGELOG.md).
- `winterbaume-s3` v2.0.0: `aws-chunked` bodies stored decoded (#84); opt-in STS session policy enforcement (#85); breaking `S3Error` variants. See [`crates/winterbaume-s3/CHANGELOG.md`](crates/winterbaume-s3/CHANGELOG.md).
- `winterbaume-s3control` v0.4.0: dependency-only, for `winterbaume-s3` v2.0.0. See [`crates/winterbaume-s3control/CHANGELOG.md`](crates/winterbaume-s3control/CHANGELOG.md).
- `winterbaume-s3files` v0.2.3: dependency-only. See [`crates/winterbaume-s3files/CHANGELOG.md`](crates/winterbaume-s3files/CHANGELOG.md).
- `winterbaume-s3outposts` v0.2.3: dependency-only. See [`crates/winterbaume-s3outposts/CHANGELOG.md`](crates/winterbaume-s3outposts/CHANGELOG.md).
- `winterbaume-s3tables` v0.2.3: dependency-only. See [`crates/winterbaume-s3tables/CHANGELOG.md`](crates/winterbaume-s3tables/CHANGELOG.md).
- `winterbaume-s3vectors` v0.2.3: dependency-only. See [`crates/winterbaume-s3vectors/CHANGELOG.md`](crates/winterbaume-s3vectors/CHANGELOG.md).
- `winterbaume-sagemaker` v0.3.3: dependency-only. See [`crates/winterbaume-sagemaker/CHANGELOG.md`](crates/winterbaume-sagemaker/CHANGELOG.md).
- `winterbaume-sagemakermetrics` v0.2.3: dependency-only. See [`crates/winterbaume-sagemakermetrics/CHANGELOG.md`](crates/winterbaume-sagemakermetrics/CHANGELOG.md).
- `winterbaume-sagemakerruntime` v0.3.3: dependency-only. See [`crates/winterbaume-sagemakerruntime/CHANGELOG.md`](crates/winterbaume-sagemakerruntime/CHANGELOG.md).
- `winterbaume-savingsplans` v0.2.3: dependency-only. See [`crates/winterbaume-savingsplans/CHANGELOG.md`](crates/winterbaume-savingsplans/CHANGELOG.md).
- `winterbaume-scheduler` v0.2.3: dependency-only. See [`crates/winterbaume-scheduler/CHANGELOG.md`](crates/winterbaume-scheduler/CHANGELOG.md).
- `winterbaume-secretsmanager` v0.2.3: dependency-only. See [`crates/winterbaume-secretsmanager/CHANGELOG.md`](crates/winterbaume-secretsmanager/CHANGELOG.md).
- `winterbaume-securityhub` v0.2.3: dependency-only. See [`crates/winterbaume-securityhub/CHANGELOG.md`](crates/winterbaume-securityhub/CHANGELOG.md).
- `winterbaume-server` v0.2.11: `--s3-enforce-session-policies` (#85). See [`crates/winterbaume-server/CHANGELOG.md`](crates/winterbaume-server/CHANGELOG.md).
- `winterbaume-servicecatalog` v0.2.3: dependency-only. See [`crates/winterbaume-servicecatalog/CHANGELOG.md`](crates/winterbaume-servicecatalog/CHANGELOG.md).
- `winterbaume-servicecatalogappregistry` v0.2.3: dependency-only. See [`crates/winterbaume-servicecatalogappregistry/CHANGELOG.md`](crates/winterbaume-servicecatalogappregistry/CHANGELOG.md).
- `winterbaume-servicediscovery` v0.2.4: dependency-only. See [`crates/winterbaume-servicediscovery/CHANGELOG.md`](crates/winterbaume-servicediscovery/CHANGELOG.md).
- `winterbaume-servicequotas` v0.2.3: dependency-only. See [`crates/winterbaume-servicequotas/CHANGELOG.md`](crates/winterbaume-servicequotas/CHANGELOG.md).
- `winterbaume-ses` v0.3.3: dependency-only. See [`crates/winterbaume-ses/CHANGELOG.md`](crates/winterbaume-ses/CHANGELOG.md).
- `winterbaume-sesv2` v0.3.3: dependency-only. See [`crates/winterbaume-sesv2/CHANGELOG.md`](crates/winterbaume-sesv2/CHANGELOG.md).
- `winterbaume-sfn` v0.2.3: dependency-only. See [`crates/winterbaume-sfn/CHANGELOG.md`](crates/winterbaume-sfn/CHANGELOG.md).
- `winterbaume-shield` v0.2.3: dependency-only. See [`crates/winterbaume-shield/CHANGELOG.md`](crates/winterbaume-shield/CHANGELOG.md).
- `winterbaume-signer` v0.2.3: dependency-only. See [`crates/winterbaume-signer/CHANGELOG.md`](crates/winterbaume-signer/CHANGELOG.md).
- `winterbaume-simpledbv2` v0.2.3: dependency-only. See [`crates/winterbaume-simpledbv2/CHANGELOG.md`](crates/winterbaume-simpledbv2/CHANGELOG.md).
- `winterbaume-simspaceweaver` v0.2.3: dependency-only. See [`crates/winterbaume-simspaceweaver/CHANGELOG.md`](crates/winterbaume-simspaceweaver/CHANGELOG.md).
- `winterbaume-snowdevicemanagement` v0.2.3: dependency-only. See [`crates/winterbaume-snowdevicemanagement/CHANGELOG.md`](crates/winterbaume-snowdevicemanagement/CHANGELOG.md).
- `winterbaume-sns` v0.2.3: dependency-only. See [`crates/winterbaume-sns/CHANGELOG.md`](crates/winterbaume-sns/CHANGELOG.md).
- `winterbaume-sqlengine-duckdb` v0.2.3: dependency-only. See [`crates/winterbaume-sqlengine-duckdb/CHANGELOG.md`](crates/winterbaume-sqlengine-duckdb/CHANGELOG.md).
- `winterbaume-sqs` v0.3.1: dependency-only. See [`crates/winterbaume-sqs/CHANGELOG.md`](crates/winterbaume-sqs/CHANGELOG.md).
- `winterbaume-sqs-redis` v0.3.1: dependency-only. See [`crates/winterbaume-sqs-redis/CHANGELOG.md`](crates/winterbaume-sqs-redis/CHANGELOG.md).
- `winterbaume-ssm` v0.3.3: dependency-only. See [`crates/winterbaume-ssm/CHANGELOG.md`](crates/winterbaume-ssm/CHANGELOG.md).
- `winterbaume-ssmquicksetup` v0.2.3: dependency-only. See [`crates/winterbaume-ssmquicksetup/CHANGELOG.md`](crates/winterbaume-ssmquicksetup/CHANGELOG.md).
- `winterbaume-sso` v0.2.3: dependency-only. See [`crates/winterbaume-sso/CHANGELOG.md`](crates/winterbaume-sso/CHANGELOG.md).
- `winterbaume-ssoadmin` v0.2.3: dependency-only. See [`crates/winterbaume-ssoadmin/CHANGELOG.md`](crates/winterbaume-ssoadmin/CHANGELOG.md).
- `winterbaume-sts` v0.4.0: session policies kept with the issued credentials (#85). See [`crates/winterbaume-sts/CHANGELOG.md`](crates/winterbaume-sts/CHANGELOG.md).
- `winterbaume-support` v0.2.3: dependency-only. See [`crates/winterbaume-support/CHANGELOG.md`](crates/winterbaume-support/CHANGELOG.md).
- `winterbaume-supportapp` v0.2.3: dependency-only. See [`crates/winterbaume-supportapp/CHANGELOG.md`](crates/winterbaume-supportapp/CHANGELOG.md).
- `winterbaume-swf` v0.2.3: dependency-only. See [`crates/winterbaume-swf/CHANGELOG.md`](crates/winterbaume-swf/CHANGELOG.md).
- `winterbaume-synthetics` v0.2.3: dependency-only. See [`crates/winterbaume-synthetics/CHANGELOG.md`](crates/winterbaume-synthetics/CHANGELOG.md).
- `winterbaume-taxsettings` v0.2.3: dependency-only. See [`crates/winterbaume-taxsettings/CHANGELOG.md`](crates/winterbaume-taxsettings/CHANGELOG.md).
- `winterbaume-terraform` v0.4.0: dependency-only, for `winterbaume-s3` v2.0.0. See [`crates/winterbaume-terraform/CHANGELOG.md`](crates/winterbaume-terraform/CHANGELOG.md).
- `winterbaume-textract` v0.2.3: dependency-only. See [`crates/winterbaume-textract/CHANGELOG.md`](crates/winterbaume-textract/CHANGELOG.md).
- `winterbaume-timestreaminfluxdb` v0.2.3: dependency-only. See [`crates/winterbaume-timestreaminfluxdb/CHANGELOG.md`](crates/winterbaume-timestreaminfluxdb/CHANGELOG.md).
- `winterbaume-timestreamquery` v0.2.3: dependency-only. See [`crates/winterbaume-timestreamquery/CHANGELOG.md`](crates/winterbaume-timestreamquery/CHANGELOG.md).
- `winterbaume-timestreamwrite` v0.2.3: dependency-only. See [`crates/winterbaume-timestreamwrite/CHANGELOG.md`](crates/winterbaume-timestreamwrite/CHANGELOG.md).
- `winterbaume-transcribe` v0.2.3: dependency-only. See [`crates/winterbaume-transcribe/CHANGELOG.md`](crates/winterbaume-transcribe/CHANGELOG.md).
- `winterbaume-transfer` v0.2.3: dependency-only. See [`crates/winterbaume-transfer/CHANGELOG.md`](crates/winterbaume-transfer/CHANGELOG.md).
- `winterbaume-trustedadvisor` v0.2.3: dependency-only. See [`crates/winterbaume-trustedadvisor/CHANGELOG.md`](crates/winterbaume-trustedadvisor/CHANGELOG.md).
- `winterbaume-vpclattice` v0.2.3: dependency-only. See [`crates/winterbaume-vpclattice/CHANGELOG.md`](crates/winterbaume-vpclattice/CHANGELOG.md).
- `winterbaume-wafv2` v0.2.3: dependency-only. See [`crates/winterbaume-wafv2/CHANGELOG.md`](crates/winterbaume-wafv2/CHANGELOG.md).
- `winterbaume-workspaces` v0.2.3: dependency-only. See [`crates/winterbaume-workspaces/CHANGELOG.md`](crates/winterbaume-workspaces/CHANGELOG.md).
- `winterbaume-workspacesweb` v0.2.3: dependency-only. See [`crates/winterbaume-workspacesweb/CHANGELOG.md`](crates/winterbaume-workspacesweb/CHANGELOG.md).
- `winterbaume-xray` v0.2.3: dependency-only. See [`crates/winterbaume-xray/CHANGELOG.md`](crates/winterbaume-xray/CHANGELOG.md).

## 2026-11-25

- Fixes for inputs the services accepted and then dropped or got wrong: atomic DynamoDB conditional writes ( #61 ), Lambda function settings ( #62 ), SQS FIFO parity ( #64 – #66 ), EventBridge rule tags and bus scoping ( #67 ), and CloudWatch alarm settings and AWS JSON 1.0 ( #68, #69 ).
- Named relaxations ( #77 ): `winterbaume-server --relax RULE` accepts one request AWS refuses, by name. The first rule, `sts-assume-role-without-role-arn`, lets `AssumeRole` omit `RoleArn` as MinIO does.
- `winterbaume-core` gains public items but changes nothing, so it is a patch release ( v0.3.1 ) and its dependants' releases are dependency-only. Breaking releases: `winterbaume-lambda` v3.0.0, and new 0.y minors for STS, SQS, CloudWatch, EventBridge, DynamoDB, the two Redis backends, DynamoDB Streams, `winterbaume-terraform` and `winterbaume-tfstate-resource-models`.

- `winterbaume-accessanalyzer` v0.2.2: dependency-only. See [`crates/winterbaume-accessanalyzer/CHANGELOG.md`](crates/winterbaume-accessanalyzer/CHANGELOG.md).
- `winterbaume-account` v1.0.2: dependency-only. See [`crates/winterbaume-account/CHANGELOG.md`](crates/winterbaume-account/CHANGELOG.md).
- `winterbaume-acm` v0.2.2: dependency-only. See [`crates/winterbaume-acm/CHANGELOG.md`](crates/winterbaume-acm/CHANGELOG.md).
- `winterbaume-acmpca` v0.2.2: dependency-only. See [`crates/winterbaume-acmpca/CHANGELOG.md`](crates/winterbaume-acmpca/CHANGELOG.md).
- `winterbaume-aiops` v0.2.2: dependency-only. See [`crates/winterbaume-aiops/CHANGELOG.md`](crates/winterbaume-aiops/CHANGELOG.md).
- `winterbaume-amp` v0.2.2: dependency-only. See [`crates/winterbaume-amp/CHANGELOG.md`](crates/winterbaume-amp/CHANGELOG.md).
- `winterbaume-amplify` v0.2.2: dependency-only. See [`crates/winterbaume-amplify/CHANGELOG.md`](crates/winterbaume-amplify/CHANGELOG.md).
- `winterbaume-amplifybackend` v0.2.2: dependency-only. See [`crates/winterbaume-amplifybackend/CHANGELOG.md`](crates/winterbaume-amplifybackend/CHANGELOG.md).
- `winterbaume-amplifyuibuilder` v0.2.2: dependency-only. See [`crates/winterbaume-amplifyuibuilder/CHANGELOG.md`](crates/winterbaume-amplifyuibuilder/CHANGELOG.md).
- `winterbaume-apigateway` v1.0.2: dependency-only. See [`crates/winterbaume-apigateway/CHANGELOG.md`](crates/winterbaume-apigateway/CHANGELOG.md).
- `winterbaume-apigatewaymanagement` v0.2.2: dependency-only. See [`crates/winterbaume-apigatewaymanagement/CHANGELOG.md`](crates/winterbaume-apigatewaymanagement/CHANGELOG.md).
- `winterbaume-apigatewayv2` v0.2.2: dependency-only. See [`crates/winterbaume-apigatewayv2/CHANGELOG.md`](crates/winterbaume-apigatewayv2/CHANGELOG.md).
- `winterbaume-appconfig` v2.0.2: dependency-only. See [`crates/winterbaume-appconfig/CHANGELOG.md`](crates/winterbaume-appconfig/CHANGELOG.md).
- `winterbaume-appconfigdata` v0.3.3: dependency-only. See [`crates/winterbaume-appconfigdata/CHANGELOG.md`](crates/winterbaume-appconfigdata/CHANGELOG.md).
- `winterbaume-appfabric` v0.2.2: dependency-only. See [`crates/winterbaume-appfabric/CHANGELOG.md`](crates/winterbaume-appfabric/CHANGELOG.md).
- `winterbaume-appflow` v0.2.2: dependency-only. See [`crates/winterbaume-appflow/CHANGELOG.md`](crates/winterbaume-appflow/CHANGELOG.md).
- `winterbaume-appintegrations` v0.2.2: dependency-only. See [`crates/winterbaume-appintegrations/CHANGELOG.md`](crates/winterbaume-appintegrations/CHANGELOG.md).
- `winterbaume-applicationautoscaling` v0.2.2: dependency-only. See [`crates/winterbaume-applicationautoscaling/CHANGELOG.md`](crates/winterbaume-applicationautoscaling/CHANGELOG.md).
- `winterbaume-applicationcostprofiler` v0.2.2: dependency-only. See [`crates/winterbaume-applicationcostprofiler/CHANGELOG.md`](crates/winterbaume-applicationcostprofiler/CHANGELOG.md).
- `winterbaume-applicationdiscoveryservice` v0.2.2: dependency-only. See [`crates/winterbaume-applicationdiscoveryservice/CHANGELOG.md`](crates/winterbaume-applicationdiscoveryservice/CHANGELOG.md).
- `winterbaume-applicationinsights` v0.2.2: dependency-only. See [`crates/winterbaume-applicationinsights/CHANGELOG.md`](crates/winterbaume-applicationinsights/CHANGELOG.md).
- `winterbaume-applicationsignals` v1.0.2: dependency-only. See [`crates/winterbaume-applicationsignals/CHANGELOG.md`](crates/winterbaume-applicationsignals/CHANGELOG.md).
- `winterbaume-appmesh` v0.2.2: dependency-only. See [`crates/winterbaume-appmesh/CHANGELOG.md`](crates/winterbaume-appmesh/CHANGELOG.md).
- `winterbaume-apprunner` v0.2.2: dependency-only. See [`crates/winterbaume-apprunner/CHANGELOG.md`](crates/winterbaume-apprunner/CHANGELOG.md).
- `winterbaume-appsync` v0.2.2: dependency-only. See [`crates/winterbaume-appsync/CHANGELOG.md`](crates/winterbaume-appsync/CHANGELOG.md).
- `winterbaume-arczonalshift` v0.2.2: dependency-only. See [`crates/winterbaume-arczonalshift/CHANGELOG.md`](crates/winterbaume-arczonalshift/CHANGELOG.md).
- `winterbaume-artifact` v0.2.2: dependency-only. See [`crates/winterbaume-artifact/CHANGELOG.md`](crates/winterbaume-artifact/CHANGELOG.md).
- `winterbaume-athena` v0.2.2: dependency-only. See [`crates/winterbaume-athena/CHANGELOG.md`](crates/winterbaume-athena/CHANGELOG.md).
- `winterbaume-auditmanager` v0.2.2: dependency-only. See [`crates/winterbaume-auditmanager/CHANGELOG.md`](crates/winterbaume-auditmanager/CHANGELOG.md).
- `winterbaume-autoscaling` v0.2.3: dependency-only. See [`crates/winterbaume-autoscaling/CHANGELOG.md`](crates/winterbaume-autoscaling/CHANGELOG.md).
- `winterbaume-autoscalingplans` v0.2.2: dependency-only. See [`crates/winterbaume-autoscalingplans/CHANGELOG.md`](crates/winterbaume-autoscalingplans/CHANGELOG.md).
- `winterbaume-backup` v0.2.2: dependency-only. See [`crates/winterbaume-backup/CHANGELOG.md`](crates/winterbaume-backup/CHANGELOG.md).
- `winterbaume-backupgateway` v0.2.2: dependency-only. See [`crates/winterbaume-backupgateway/CHANGELOG.md`](crates/winterbaume-backupgateway/CHANGELOG.md).
- `winterbaume-backupsearch` v0.2.2: dependency-only. See [`crates/winterbaume-backupsearch/CHANGELOG.md`](crates/winterbaume-backupsearch/CHANGELOG.md).
- `winterbaume-batch` v1.0.2: dependency-only. See [`crates/winterbaume-batch/CHANGELOG.md`](crates/winterbaume-batch/CHANGELOG.md).
- `winterbaume-bcmdashboards` v0.2.2: dependency-only. See [`crates/winterbaume-bcmdashboards/CHANGELOG.md`](crates/winterbaume-bcmdashboards/CHANGELOG.md).
- `winterbaume-bcmdataexports` v0.2.2: dependency-only. See [`crates/winterbaume-bcmdataexports/CHANGELOG.md`](crates/winterbaume-bcmdataexports/CHANGELOG.md).
- `winterbaume-bcmrecommendedactions` v0.2.2: dependency-only. See [`crates/winterbaume-bcmrecommendedactions/CHANGELOG.md`](crates/winterbaume-bcmrecommendedactions/CHANGELOG.md).
- `winterbaume-bedrock` v0.2.2: dependency-only. See [`crates/winterbaume-bedrock/CHANGELOG.md`](crates/winterbaume-bedrock/CHANGELOG.md).
- `winterbaume-bedrockagent` v0.2.2: dependency-only. See [`crates/winterbaume-bedrockagent/CHANGELOG.md`](crates/winterbaume-bedrockagent/CHANGELOG.md).
- `winterbaume-billing` v0.2.2: dependency-only. See [`crates/winterbaume-billing/CHANGELOG.md`](crates/winterbaume-billing/CHANGELOG.md).
- `winterbaume-braket` v0.2.2: dependency-only. See [`crates/winterbaume-braket/CHANGELOG.md`](crates/winterbaume-braket/CHANGELOG.md).
- `winterbaume-budgets` v0.2.2: dependency-only. See [`crates/winterbaume-budgets/CHANGELOG.md`](crates/winterbaume-budgets/CHANGELOG.md).
- `winterbaume-chatbot` v0.2.2: dependency-only. See [`crates/winterbaume-chatbot/CHANGELOG.md`](crates/winterbaume-chatbot/CHANGELOG.md).
- `winterbaume-chimesdkmeetings` v0.2.2: dependency-only. See [`crates/winterbaume-chimesdkmeetings/CHANGELOG.md`](crates/winterbaume-chimesdkmeetings/CHANGELOG.md).
- `winterbaume-cloud9` v0.2.2: dependency-only. See [`crates/winterbaume-cloud9/CHANGELOG.md`](crates/winterbaume-cloud9/CHANGELOG.md).
- `winterbaume-cloudcontrol` v1.2.1: dependency-only. See [`crates/winterbaume-cloudcontrol/CHANGELOG.md`](crates/winterbaume-cloudcontrol/CHANGELOG.md).
- `winterbaume-clouddirectory` v0.2.2: dependency-only. See [`crates/winterbaume-clouddirectory/CHANGELOG.md`](crates/winterbaume-clouddirectory/CHANGELOG.md).
- `winterbaume-cloudformation` v0.2.2: dependency-only. See [`crates/winterbaume-cloudformation/CHANGELOG.md`](crates/winterbaume-cloudformation/CHANGELOG.md).
- `winterbaume-cloudfront` v1.1.1: dependency-only. See [`crates/winterbaume-cloudfront/CHANGELOG.md`](crates/winterbaume-cloudfront/CHANGELOG.md).
- `winterbaume-cloudfrontkeyvaluestore` v0.2.3: dependency-only. See [`crates/winterbaume-cloudfrontkeyvaluestore/CHANGELOG.md`](crates/winterbaume-cloudfrontkeyvaluestore/CHANGELOG.md).
- `winterbaume-cloudhsmv2` v0.2.2: dependency-only. See [`crates/winterbaume-cloudhsmv2/CHANGELOG.md`](crates/winterbaume-cloudhsmv2/CHANGELOG.md).
- `winterbaume-cloudsearchdomain` v0.2.2: dependency-only. See [`crates/winterbaume-cloudsearchdomain/CHANGELOG.md`](crates/winterbaume-cloudsearchdomain/CHANGELOG.md).
- `winterbaume-cloudtrail` v0.2.2: dependency-only. See [`crates/winterbaume-cloudtrail/CHANGELOG.md`](crates/winterbaume-cloudtrail/CHANGELOG.md).
- `winterbaume-cloudtraildata` v0.2.2: dependency-only. See [`crates/winterbaume-cloudtraildata/CHANGELOG.md`](crates/winterbaume-cloudtraildata/CHANGELOG.md).
- `winterbaume-cloudwatch` v0.3.0: AWS JSON 1.0 and every `PutMetricAlarm` setting (#68, #69). See [`crates/winterbaume-cloudwatch/CHANGELOG.md`](crates/winterbaume-cloudwatch/CHANGELOG.md).
- `winterbaume-cloudwatchlogs` v0.2.2: dependency-only. See [`crates/winterbaume-cloudwatchlogs/CHANGELOG.md`](crates/winterbaume-cloudwatchlogs/CHANGELOG.md).
- `winterbaume-codeartifact` v0.2.2: dependency-only. See [`crates/winterbaume-codeartifact/CHANGELOG.md`](crates/winterbaume-codeartifact/CHANGELOG.md).
- `winterbaume-codebuild` v0.2.2: dependency-only. See [`crates/winterbaume-codebuild/CHANGELOG.md`](crates/winterbaume-codebuild/CHANGELOG.md).
- `winterbaume-codecommit` v0.2.2: dependency-only. See [`crates/winterbaume-codecommit/CHANGELOG.md`](crates/winterbaume-codecommit/CHANGELOG.md).
- `winterbaume-codedeploy` v0.2.3: dependency-only. See [`crates/winterbaume-codedeploy/CHANGELOG.md`](crates/winterbaume-codedeploy/CHANGELOG.md).
- `winterbaume-codegurureviewer` v0.2.2: dependency-only. See [`crates/winterbaume-codegurureviewer/CHANGELOG.md`](crates/winterbaume-codegurureviewer/CHANGELOG.md).
- `winterbaume-codegurusecurity` v0.2.2: dependency-only. See [`crates/winterbaume-codegurusecurity/CHANGELOG.md`](crates/winterbaume-codegurusecurity/CHANGELOG.md).
- `winterbaume-codepipeline` v0.2.2: dependency-only. See [`crates/winterbaume-codepipeline/CHANGELOG.md`](crates/winterbaume-codepipeline/CHANGELOG.md).
- `winterbaume-codestarnotifications` v0.2.2: dependency-only. See [`crates/winterbaume-codestarnotifications/CHANGELOG.md`](crates/winterbaume-codestarnotifications/CHANGELOG.md).
- `winterbaume-cognitoidentity` v0.2.2: dependency-only. See [`crates/winterbaume-cognitoidentity/CHANGELOG.md`](crates/winterbaume-cognitoidentity/CHANGELOG.md).
- `winterbaume-cognitoidentityprovider` v0.2.2: dependency-only. See [`crates/winterbaume-cognitoidentityprovider/CHANGELOG.md`](crates/winterbaume-cognitoidentityprovider/CHANGELOG.md).
- `winterbaume-cognitosync` v0.2.2: dependency-only. See [`crates/winterbaume-cognitosync/CHANGELOG.md`](crates/winterbaume-cognitosync/CHANGELOG.md).
- `winterbaume-comprehend` v0.2.2: dependency-only. See [`crates/winterbaume-comprehend/CHANGELOG.md`](crates/winterbaume-comprehend/CHANGELOG.md).
- `winterbaume-config` v0.2.2: dependency-only. See [`crates/winterbaume-config/CHANGELOG.md`](crates/winterbaume-config/CHANGELOG.md).
- `winterbaume-connect` v0.2.2: dependency-only. See [`crates/winterbaume-connect/CHANGELOG.md`](crates/winterbaume-connect/CHANGELOG.md).
- `winterbaume-connectcampaigns` v0.2.2: dependency-only. See [`crates/winterbaume-connectcampaigns/CHANGELOG.md`](crates/winterbaume-connectcampaigns/CHANGELOG.md).
- `winterbaume-connectcontactlens` v0.2.2: dependency-only. See [`crates/winterbaume-connectcontactlens/CHANGELOG.md`](crates/winterbaume-connectcontactlens/CHANGELOG.md).
- `winterbaume-connectparticipant` v0.2.2: dependency-only. See [`crates/winterbaume-connectparticipant/CHANGELOG.md`](crates/winterbaume-connectparticipant/CHANGELOG.md).
- `winterbaume-controlcatalog` v0.2.2: dependency-only. See [`crates/winterbaume-controlcatalog/CHANGELOG.md`](crates/winterbaume-controlcatalog/CHANGELOG.md).
- `winterbaume-core` v0.3.1: named relaxations and `auth::extract_access_key_from_headers` (#77); additive. See [`crates/winterbaume-core/CHANGELOG.md`](crates/winterbaume-core/CHANGELOG.md).
- `winterbaume-costandusagereport` v0.2.2: dependency-only. See [`crates/winterbaume-costandusagereport/CHANGELOG.md`](crates/winterbaume-costandusagereport/CHANGELOG.md).
- `winterbaume-costexplorer` v0.2.2: dependency-only. See [`crates/winterbaume-costexplorer/CHANGELOG.md`](crates/winterbaume-costexplorer/CHANGELOG.md).
- `winterbaume-costoptimizationhub` v0.2.2: dependency-only. See [`crates/winterbaume-costoptimizationhub/CHANGELOG.md`](crates/winterbaume-costoptimizationhub/CHANGELOG.md).
- `winterbaume-databasemigration` v0.2.2: dependency-only. See [`crates/winterbaume-databasemigration/CHANGELOG.md`](crates/winterbaume-databasemigration/CHANGELOG.md).
- `winterbaume-databrew` v0.2.2: dependency-only. See [`crates/winterbaume-databrew/CHANGELOG.md`](crates/winterbaume-databrew/CHANGELOG.md).
- `winterbaume-datapipeline` v0.2.2: dependency-only. See [`crates/winterbaume-datapipeline/CHANGELOG.md`](crates/winterbaume-datapipeline/CHANGELOG.md).
- `winterbaume-datasync` v0.2.2: dependency-only. See [`crates/winterbaume-datasync/CHANGELOG.md`](crates/winterbaume-datasync/CHANGELOG.md).
- `winterbaume-dax` v0.2.2: dependency-only. See [`crates/winterbaume-dax/CHANGELOG.md`](crates/winterbaume-dax/CHANGELOG.md).
- `winterbaume-directconnect` v0.2.2: dependency-only. See [`crates/winterbaume-directconnect/CHANGELOG.md`](crates/winterbaume-directconnect/CHANGELOG.md).
- `winterbaume-directory` v0.2.2: dependency-only. See [`crates/winterbaume-directory/CHANGELOG.md`](crates/winterbaume-directory/CHANGELOG.md).
- `winterbaume-dlm` v0.2.2: dependency-only. See [`crates/winterbaume-dlm/CHANGELOG.md`](crates/winterbaume-dlm/CHANGELOG.md).
- `winterbaume-dsql` v0.2.2: dependency-only. See [`crates/winterbaume-dsql/CHANGELOG.md`](crates/winterbaume-dsql/CHANGELOG.md).
- `winterbaume-dynamodb` v0.5.0: atomic conditional writes (#61). See [`crates/winterbaume-dynamodb/CHANGELOG.md`](crates/winterbaume-dynamodb/CHANGELOG.md).
- `winterbaume-dynamodb-redis` v0.3.0: atomic conditional writes in Redis (#61). See [`crates/winterbaume-dynamodb-redis/CHANGELOG.md`](crates/winterbaume-dynamodb-redis/CHANGELOG.md).
- `winterbaume-dynamodbstreams` v0.3.0: dependency-only, for `winterbaume-dynamodb` v0.5.0. See [`crates/winterbaume-dynamodbstreams/CHANGELOG.md`](crates/winterbaume-dynamodbstreams/CHANGELOG.md).
- `winterbaume-ebs` v0.2.2: dependency-only. See [`crates/winterbaume-ebs/CHANGELOG.md`](crates/winterbaume-ebs/CHANGELOG.md).
- `winterbaume-ec2` v1.0.2: security group descriptions and `+` in form bodies. See [`crates/winterbaume-ec2/CHANGELOG.md`](crates/winterbaume-ec2/CHANGELOG.md).
- `winterbaume-ec2-generated` v0.2.2: dependency-only. See [`crates/winterbaume-ec2-generated/CHANGELOG.md`](crates/winterbaume-ec2-generated/CHANGELOG.md).
- `winterbaume-ec2instanceconnect` v0.2.2: dependency-only. See [`crates/winterbaume-ec2instanceconnect/CHANGELOG.md`](crates/winterbaume-ec2instanceconnect/CHANGELOG.md).
- `winterbaume-ecr` v0.2.2: dependency-only. See [`crates/winterbaume-ecr/CHANGELOG.md`](crates/winterbaume-ecr/CHANGELOG.md).
- `winterbaume-ecs` v0.2.2: dependency-only. See [`crates/winterbaume-ecs/CHANGELOG.md`](crates/winterbaume-ecs/CHANGELOG.md).
- `winterbaume-efs` v0.2.2: dependency-only. See [`crates/winterbaume-efs/CHANGELOG.md`](crates/winterbaume-efs/CHANGELOG.md).
- `winterbaume-eks` v0.2.2: dependency-only. See [`crates/winterbaume-eks/CHANGELOG.md`](crates/winterbaume-eks/CHANGELOG.md).
- `winterbaume-elasticache` v0.2.2: dependency-only. See [`crates/winterbaume-elasticache/CHANGELOG.md`](crates/winterbaume-elasticache/CHANGELOG.md).
- `winterbaume-elasticbeanstalk` v0.2.2: dependency-only. See [`crates/winterbaume-elasticbeanstalk/CHANGELOG.md`](crates/winterbaume-elasticbeanstalk/CHANGELOG.md).
- `winterbaume-elasticloadbalancing` v0.2.2: dependency-only. See [`crates/winterbaume-elasticloadbalancing/CHANGELOG.md`](crates/winterbaume-elasticloadbalancing/CHANGELOG.md).
- `winterbaume-elasticloadbalancingv2` v0.2.2: dependency-only. See [`crates/winterbaume-elasticloadbalancingv2/CHANGELOG.md`](crates/winterbaume-elasticloadbalancingv2/CHANGELOG.md).
- `winterbaume-emr` v0.2.2: dependency-only. See [`crates/winterbaume-emr/CHANGELOG.md`](crates/winterbaume-emr/CHANGELOG.md).
- `winterbaume-emrcontainers` v0.2.2: dependency-only. See [`crates/winterbaume-emrcontainers/CHANGELOG.md`](crates/winterbaume-emrcontainers/CHANGELOG.md).
- `winterbaume-emrserverless` v0.2.2: dependency-only. See [`crates/winterbaume-emrserverless/CHANGELOG.md`](crates/winterbaume-emrserverless/CHANGELOG.md).
- `winterbaume-eventbridge` v0.3.0: `PutRule` tags on creation, rules scoped by event bus (#67). See [`crates/winterbaume-eventbridge/CHANGELOG.md`](crates/winterbaume-eventbridge/CHANGELOG.md).
- `winterbaume-firehose` v0.2.2: dependency-only. See [`crates/winterbaume-firehose/CHANGELOG.md`](crates/winterbaume-firehose/CHANGELOG.md).
- `winterbaume-fis` v0.2.3: dependency-only. See [`crates/winterbaume-fis/CHANGELOG.md`](crates/winterbaume-fis/CHANGELOG.md).
- `winterbaume-forecast` v0.2.2: dependency-only. See [`crates/winterbaume-forecast/CHANGELOG.md`](crates/winterbaume-forecast/CHANGELOG.md).
- `winterbaume-freetier` v0.2.2: dependency-only. See [`crates/winterbaume-freetier/CHANGELOG.md`](crates/winterbaume-freetier/CHANGELOG.md).
- `winterbaume-fsx` v0.2.2: dependency-only. See [`crates/winterbaume-fsx/CHANGELOG.md`](crates/winterbaume-fsx/CHANGELOG.md).
- `winterbaume-glacier` v0.2.2: dependency-only. See [`crates/winterbaume-glacier/CHANGELOG.md`](crates/winterbaume-glacier/CHANGELOG.md).
- `winterbaume-glue` v0.2.2: dependency-only. See [`crates/winterbaume-glue/CHANGELOG.md`](crates/winterbaume-glue/CHANGELOG.md).
- `winterbaume-greengrass` v0.2.2: dependency-only. See [`crates/winterbaume-greengrass/CHANGELOG.md`](crates/winterbaume-greengrass/CHANGELOG.md).
- `winterbaume-guardduty` v0.2.2: dependency-only. See [`crates/winterbaume-guardduty/CHANGELOG.md`](crates/winterbaume-guardduty/CHANGELOG.md).
- `winterbaume-iam` v0.2.3: dependency-only. See [`crates/winterbaume-iam/CHANGELOG.md`](crates/winterbaume-iam/CHANGELOG.md).
- `winterbaume-identitystore` v0.2.2: dependency-only. See [`crates/winterbaume-identitystore/CHANGELOG.md`](crates/winterbaume-identitystore/CHANGELOG.md).
- `winterbaume-inspector2` v0.2.2: dependency-only. See [`crates/winterbaume-inspector2/CHANGELOG.md`](crates/winterbaume-inspector2/CHANGELOG.md).
- `winterbaume-iot` v0.2.2: dependency-only. See [`crates/winterbaume-iot/CHANGELOG.md`](crates/winterbaume-iot/CHANGELOG.md).
- `winterbaume-iotdataplane` v0.2.2: dependency-only. See [`crates/winterbaume-iotdataplane/CHANGELOG.md`](crates/winterbaume-iotdataplane/CHANGELOG.md).
- `winterbaume-ivs` v1.0.2: dependency-only. See [`crates/winterbaume-ivs/CHANGELOG.md`](crates/winterbaume-ivs/CHANGELOG.md).
- `winterbaume-kafka` v0.2.2: dependency-only. See [`crates/winterbaume-kafka/CHANGELOG.md`](crates/winterbaume-kafka/CHANGELOG.md).
- `winterbaume-keyspaces` v0.2.3: dependency-only. See [`crates/winterbaume-keyspaces/CHANGELOG.md`](crates/winterbaume-keyspaces/CHANGELOG.md).
- `winterbaume-kinesis` v0.2.3: dependency-only. See [`crates/winterbaume-kinesis/CHANGELOG.md`](crates/winterbaume-kinesis/CHANGELOG.md).
- `winterbaume-kinesisanalyticsv2` v0.2.2: dependency-only. See [`crates/winterbaume-kinesisanalyticsv2/CHANGELOG.md`](crates/winterbaume-kinesisanalyticsv2/CHANGELOG.md).
- `winterbaume-kinesisvideo` v0.3.2: dependency-only. See [`crates/winterbaume-kinesisvideo/CHANGELOG.md`](crates/winterbaume-kinesisvideo/CHANGELOG.md).
- `winterbaume-kinesisvideoarchivedmedia` v0.3.2: dependency-only. See [`crates/winterbaume-kinesisvideoarchivedmedia/CHANGELOG.md`](crates/winterbaume-kinesisvideoarchivedmedia/CHANGELOG.md).
- `winterbaume-kms` v0.2.3: dependency-only. See [`crates/winterbaume-kms/CHANGELOG.md`](crates/winterbaume-kms/CHANGELOG.md).
- `winterbaume-lakeformation` v0.2.2: dependency-only. See [`crates/winterbaume-lakeformation/CHANGELOG.md`](crates/winterbaume-lakeformation/CHANGELOG.md).
- `winterbaume-lambda` v3.0.0: function settings stored and returned; typed `FunctionSettings` (#62). See [`crates/winterbaume-lambda/CHANGELOG.md`](crates/winterbaume-lambda/CHANGELOG.md).
- `winterbaume-lexmodelsv2` v0.2.2: dependency-only. See [`crates/winterbaume-lexmodelsv2/CHANGELOG.md`](crates/winterbaume-lexmodelsv2/CHANGELOG.md).
- `winterbaume-macie2` v0.2.2: dependency-only. See [`crates/winterbaume-macie2/CHANGELOG.md`](crates/winterbaume-macie2/CHANGELOG.md).
- `winterbaume-managedblockchain` v0.2.3: dependency-only. See [`crates/winterbaume-managedblockchain/CHANGELOG.md`](crates/winterbaume-managedblockchain/CHANGELOG.md).
- `winterbaume-marketplacemetering` v0.2.2: dependency-only. See [`crates/winterbaume-marketplacemetering/CHANGELOG.md`](crates/winterbaume-marketplacemetering/CHANGELOG.md).
- `winterbaume-mediaconnect` v0.2.2: dependency-only. See [`crates/winterbaume-mediaconnect/CHANGELOG.md`](crates/winterbaume-mediaconnect/CHANGELOG.md).
- `winterbaume-medialive` v0.2.2: dependency-only. See [`crates/winterbaume-medialive/CHANGELOG.md`](crates/winterbaume-medialive/CHANGELOG.md).
- `winterbaume-mediapackage` v0.2.2: dependency-only. See [`crates/winterbaume-mediapackage/CHANGELOG.md`](crates/winterbaume-mediapackage/CHANGELOG.md).
- `winterbaume-mediapackagev2` v0.2.2: dependency-only. See [`crates/winterbaume-mediapackagev2/CHANGELOG.md`](crates/winterbaume-mediapackagev2/CHANGELOG.md).
- `winterbaume-mediastore` v0.2.2: dependency-only. See [`crates/winterbaume-mediastore/CHANGELOG.md`](crates/winterbaume-mediastore/CHANGELOG.md).
- `winterbaume-mediastoredata` v0.2.2: dependency-only. See [`crates/winterbaume-mediastoredata/CHANGELOG.md`](crates/winterbaume-mediastoredata/CHANGELOG.md).
- `winterbaume-memorydb` v0.2.2: dependency-only. See [`crates/winterbaume-memorydb/CHANGELOG.md`](crates/winterbaume-memorydb/CHANGELOG.md).
- `winterbaume-mq` v0.2.2: dependency-only. See [`crates/winterbaume-mq/CHANGELOG.md`](crates/winterbaume-mq/CHANGELOG.md).
- `winterbaume-neptune` v0.2.2: dependency-only. See [`crates/winterbaume-neptune/CHANGELOG.md`](crates/winterbaume-neptune/CHANGELOG.md).
- `winterbaume-networkfirewall` v0.2.2: dependency-only. See [`crates/winterbaume-networkfirewall/CHANGELOG.md`](crates/winterbaume-networkfirewall/CHANGELOG.md).
- `winterbaume-networkmanager` v0.2.2: dependency-only. See [`crates/winterbaume-networkmanager/CHANGELOG.md`](crates/winterbaume-networkmanager/CHANGELOG.md).
- `winterbaume-opensearch` v1.0.2: dependency-only. See [`crates/winterbaume-opensearch/CHANGELOG.md`](crates/winterbaume-opensearch/CHANGELOG.md).
- `winterbaume-opensearchserverless` v0.2.3: dependency-only. See [`crates/winterbaume-opensearchserverless/CHANGELOG.md`](crates/winterbaume-opensearchserverless/CHANGELOG.md).
- `winterbaume-organizations` v1.0.2: dependency-only. See [`crates/winterbaume-organizations/CHANGELOG.md`](crates/winterbaume-organizations/CHANGELOG.md).
- `winterbaume-osis` v0.2.2: dependency-only. See [`crates/winterbaume-osis/CHANGELOG.md`](crates/winterbaume-osis/CHANGELOG.md).
- `winterbaume-outposts` v0.2.2: dependency-only. See [`crates/winterbaume-outposts/CHANGELOG.md`](crates/winterbaume-outposts/CHANGELOG.md).
- `winterbaume-panorama` v0.2.2: dependency-only. See [`crates/winterbaume-panorama/CHANGELOG.md`](crates/winterbaume-panorama/CHANGELOG.md).
- `winterbaume-pcaconnectorscep` v0.2.2: dependency-only. See [`crates/winterbaume-pcaconnectorscep/CHANGELOG.md`](crates/winterbaume-pcaconnectorscep/CHANGELOG.md).
- `winterbaume-personalize` v0.2.2: dependency-only. See [`crates/winterbaume-personalize/CHANGELOG.md`](crates/winterbaume-personalize/CHANGELOG.md).
- `winterbaume-personalizeevents` v0.2.2: dependency-only. See [`crates/winterbaume-personalizeevents/CHANGELOG.md`](crates/winterbaume-personalizeevents/CHANGELOG.md).
- `winterbaume-personalizeruntime` v0.2.2: dependency-only. See [`crates/winterbaume-personalizeruntime/CHANGELOG.md`](crates/winterbaume-personalizeruntime/CHANGELOG.md).
- `winterbaume-pi` v0.2.2: dependency-only. See [`crates/winterbaume-pi/CHANGELOG.md`](crates/winterbaume-pi/CHANGELOG.md).
- `winterbaume-pinpoint` v0.2.2: dependency-only. See [`crates/winterbaume-pinpoint/CHANGELOG.md`](crates/winterbaume-pinpoint/CHANGELOG.md).
- `winterbaume-pinpointsmsvoice` v0.2.2: dependency-only. See [`crates/winterbaume-pinpointsmsvoice/CHANGELOG.md`](crates/winterbaume-pinpointsmsvoice/CHANGELOG.md).
- `winterbaume-pipes` v0.2.2: dependency-only. See [`crates/winterbaume-pipes/CHANGELOG.md`](crates/winterbaume-pipes/CHANGELOG.md).
- `winterbaume-polly` v0.2.2: dependency-only. See [`crates/winterbaume-polly/CHANGELOG.md`](crates/winterbaume-polly/CHANGELOG.md).
- `winterbaume-pricing` v0.2.2: dependency-only. See [`crates/winterbaume-pricing/CHANGELOG.md`](crates/winterbaume-pricing/CHANGELOG.md).
- `winterbaume-quicksight` v0.2.2: dependency-only. See [`crates/winterbaume-quicksight/CHANGELOG.md`](crates/winterbaume-quicksight/CHANGELOG.md).
- `winterbaume-ram` v0.2.2: dependency-only. See [`crates/winterbaume-ram/CHANGELOG.md`](crates/winterbaume-ram/CHANGELOG.md).
- `winterbaume-rbin` v0.2.2: dependency-only. See [`crates/winterbaume-rbin/CHANGELOG.md`](crates/winterbaume-rbin/CHANGELOG.md).
- `winterbaume-rds` v0.2.2: dependency-only. See [`crates/winterbaume-rds/CHANGELOG.md`](crates/winterbaume-rds/CHANGELOG.md).
- `winterbaume-rdsdata` v0.2.2: dependency-only. See [`crates/winterbaume-rdsdata/CHANGELOG.md`](crates/winterbaume-rdsdata/CHANGELOG.md).
- `winterbaume-redshift` v0.2.2: dependency-only. See [`crates/winterbaume-redshift/CHANGELOG.md`](crates/winterbaume-redshift/CHANGELOG.md).
- `winterbaume-redshiftdata` v0.2.2: dependency-only. See [`crates/winterbaume-redshiftdata/CHANGELOG.md`](crates/winterbaume-redshiftdata/CHANGELOG.md).
- `winterbaume-rekognition` v0.2.2: dependency-only. See [`crates/winterbaume-rekognition/CHANGELOG.md`](crates/winterbaume-rekognition/CHANGELOG.md).
- `winterbaume-resiliencehub` v0.2.2: dependency-only. See [`crates/winterbaume-resiliencehub/CHANGELOG.md`](crates/winterbaume-resiliencehub/CHANGELOG.md).
- `winterbaume-resourcegroups` v0.2.2: dependency-only. See [`crates/winterbaume-resourcegroups/CHANGELOG.md`](crates/winterbaume-resourcegroups/CHANGELOG.md).
- `winterbaume-resourcegroupstagging` v0.2.2: dependency-only. See [`crates/winterbaume-resourcegroupstagging/CHANGELOG.md`](crates/winterbaume-resourcegroupstagging/CHANGELOG.md).
- `winterbaume-rolesanywhere` v0.2.2: dependency-only. See [`crates/winterbaume-rolesanywhere/CHANGELOG.md`](crates/winterbaume-rolesanywhere/CHANGELOG.md).
- `winterbaume-route53` v0.2.2: dependency-only. See [`crates/winterbaume-route53/CHANGELOG.md`](crates/winterbaume-route53/CHANGELOG.md).
- `winterbaume-route53domains` v0.2.2: dependency-only. See [`crates/winterbaume-route53domains/CHANGELOG.md`](crates/winterbaume-route53domains/CHANGELOG.md).
- `winterbaume-route53recoverycluster` v0.2.2: dependency-only. See [`crates/winterbaume-route53recoverycluster/CHANGELOG.md`](crates/winterbaume-route53recoverycluster/CHANGELOG.md).
- `winterbaume-route53resolver` v0.2.2: dependency-only. See [`crates/winterbaume-route53resolver/CHANGELOG.md`](crates/winterbaume-route53resolver/CHANGELOG.md).
- `winterbaume-s3` v1.1.1: dependency-only. See [`crates/winterbaume-s3/CHANGELOG.md`](crates/winterbaume-s3/CHANGELOG.md).
- `winterbaume-s3control` v0.3.1: dependency-only. See [`crates/winterbaume-s3control/CHANGELOG.md`](crates/winterbaume-s3control/CHANGELOG.md).
- `winterbaume-s3files` v0.2.2: dependency-only. See [`crates/winterbaume-s3files/CHANGELOG.md`](crates/winterbaume-s3files/CHANGELOG.md).
- `winterbaume-s3outposts` v0.2.2: dependency-only. See [`crates/winterbaume-s3outposts/CHANGELOG.md`](crates/winterbaume-s3outposts/CHANGELOG.md).
- `winterbaume-s3tables` v0.2.2: dependency-only. See [`crates/winterbaume-s3tables/CHANGELOG.md`](crates/winterbaume-s3tables/CHANGELOG.md).
- `winterbaume-s3vectors` v0.2.2: dependency-only. See [`crates/winterbaume-s3vectors/CHANGELOG.md`](crates/winterbaume-s3vectors/CHANGELOG.md).
- `winterbaume-sagemaker` v0.3.2: dependency-only. See [`crates/winterbaume-sagemaker/CHANGELOG.md`](crates/winterbaume-sagemaker/CHANGELOG.md).
- `winterbaume-sagemakermetrics` v0.2.2: dependency-only. See [`crates/winterbaume-sagemakermetrics/CHANGELOG.md`](crates/winterbaume-sagemakermetrics/CHANGELOG.md).
- `winterbaume-sagemakerruntime` v0.3.2: dependency-only. See [`crates/winterbaume-sagemakerruntime/CHANGELOG.md`](crates/winterbaume-sagemakerruntime/CHANGELOG.md).
- `winterbaume-savingsplans` v0.2.2: dependency-only. See [`crates/winterbaume-savingsplans/CHANGELOG.md`](crates/winterbaume-savingsplans/CHANGELOG.md).
- `winterbaume-scheduler` v0.2.2: dependency-only. See [`crates/winterbaume-scheduler/CHANGELOG.md`](crates/winterbaume-scheduler/CHANGELOG.md).
- `winterbaume-secretsmanager` v0.2.2: dependency-only. See [`crates/winterbaume-secretsmanager/CHANGELOG.md`](crates/winterbaume-secretsmanager/CHANGELOG.md).
- `winterbaume-securityhub` v0.2.2: dependency-only. See [`crates/winterbaume-securityhub/CHANGELOG.md`](crates/winterbaume-securityhub/CHANGELOG.md).
- `winterbaume-server` v0.2.10: `--relax` (#77). See [`crates/winterbaume-server/CHANGELOG.md`](crates/winterbaume-server/CHANGELOG.md).
- `winterbaume-servicecatalog` v0.2.2: dependency-only. See [`crates/winterbaume-servicecatalog/CHANGELOG.md`](crates/winterbaume-servicecatalog/CHANGELOG.md).
- `winterbaume-servicecatalogappregistry` v0.2.2: dependency-only. See [`crates/winterbaume-servicecatalogappregistry/CHANGELOG.md`](crates/winterbaume-servicecatalogappregistry/CHANGELOG.md).
- `winterbaume-servicediscovery` v0.2.3: dependency-only. See [`crates/winterbaume-servicediscovery/CHANGELOG.md`](crates/winterbaume-servicediscovery/CHANGELOG.md).
- `winterbaume-servicequotas` v0.2.2: dependency-only. See [`crates/winterbaume-servicequotas/CHANGELOG.md`](crates/winterbaume-servicequotas/CHANGELOG.md).
- `winterbaume-ses` v0.3.2: dependency-only. See [`crates/winterbaume-ses/CHANGELOG.md`](crates/winterbaume-ses/CHANGELOG.md).
- `winterbaume-sesv2` v0.3.2: dependency-only. See [`crates/winterbaume-sesv2/CHANGELOG.md`](crates/winterbaume-sesv2/CHANGELOG.md).
- `winterbaume-sfn` v0.2.2: dependency-only. See [`crates/winterbaume-sfn/CHANGELOG.md`](crates/winterbaume-sfn/CHANGELOG.md).
- `winterbaume-shield` v0.2.2: dependency-only. See [`crates/winterbaume-shield/CHANGELOG.md`](crates/winterbaume-shield/CHANGELOG.md).
- `winterbaume-signer` v0.2.2: dependency-only. See [`crates/winterbaume-signer/CHANGELOG.md`](crates/winterbaume-signer/CHANGELOG.md).
- `winterbaume-simpledbv2` v0.2.2: dependency-only. See [`crates/winterbaume-simpledbv2/CHANGELOG.md`](crates/winterbaume-simpledbv2/CHANGELOG.md).
- `winterbaume-simspaceweaver` v0.2.2: dependency-only. See [`crates/winterbaume-simspaceweaver/CHANGELOG.md`](crates/winterbaume-simspaceweaver/CHANGELOG.md).
- `winterbaume-snowdevicemanagement` v0.2.2: dependency-only. See [`crates/winterbaume-snowdevicemanagement/CHANGELOG.md`](crates/winterbaume-snowdevicemanagement/CHANGELOG.md).
- `winterbaume-sns` v0.2.2: dependency-only. See [`crates/winterbaume-sns/CHANGELOG.md`](crates/winterbaume-sns/CHANGELOG.md).
- `winterbaume-sqlengine-duckdb` v0.2.2: dependency-only. See [`crates/winterbaume-sqlengine-duckdb/CHANGELOG.md`](crates/winterbaume-sqlengine-duckdb/CHANGELOG.md).
- `winterbaume-sqs` v0.3.0: FIFO deduplication scope, group gating and system attributes (#64 – #66). See [`crates/winterbaume-sqs/CHANGELOG.md`](crates/winterbaume-sqs/CHANGELOG.md).
- `winterbaume-sqs-redis` v0.3.0: FIFO parity in the Redis backend (#64 – #66). See [`crates/winterbaume-sqs-redis/CHANGELOG.md`](crates/winterbaume-sqs-redis/CHANGELOG.md).
- `winterbaume-ssm` v0.3.2: dependency-only. See [`crates/winterbaume-ssm/CHANGELOG.md`](crates/winterbaume-ssm/CHANGELOG.md).
- `winterbaume-ssmquicksetup` v0.2.2: dependency-only. See [`crates/winterbaume-ssmquicksetup/CHANGELOG.md`](crates/winterbaume-ssmquicksetup/CHANGELOG.md).
- `winterbaume-sso` v0.2.2: dependency-only. See [`crates/winterbaume-sso/CHANGELOG.md`](crates/winterbaume-sso/CHANGELOG.md).
- `winterbaume-ssoadmin` v0.2.2: dependency-only. See [`crates/winterbaume-ssoadmin/CHANGELOG.md`](crates/winterbaume-ssoadmin/CHANGELOG.md).
- `winterbaume-sts` v0.3.0: `AssumeRole` without `RoleArn` behind a relaxation; `GetCallerIdentity` resolves the caller (#77). See [`crates/winterbaume-sts/CHANGELOG.md`](crates/winterbaume-sts/CHANGELOG.md).
- `winterbaume-support` v0.2.2: dependency-only. See [`crates/winterbaume-support/CHANGELOG.md`](crates/winterbaume-support/CHANGELOG.md).
- `winterbaume-supportapp` v0.2.2: dependency-only. See [`crates/winterbaume-supportapp/CHANGELOG.md`](crates/winterbaume-supportapp/CHANGELOG.md).
- `winterbaume-swf` v0.2.2: dependency-only. See [`crates/winterbaume-swf/CHANGELOG.md`](crates/winterbaume-swf/CHANGELOG.md).
- `winterbaume-synthetics` v0.2.2: dependency-only. See [`crates/winterbaume-synthetics/CHANGELOG.md`](crates/winterbaume-synthetics/CHANGELOG.md).
- `winterbaume-taxsettings` v0.2.2: dependency-only. See [`crates/winterbaume-taxsettings/CHANGELOG.md`](crates/winterbaume-taxsettings/CHANGELOG.md).
- `winterbaume-terraform` v0.3.0: new SQS, CloudWatch, EventBridge and Lambda arguments. See [`crates/winterbaume-terraform/CHANGELOG.md`](crates/winterbaume-terraform/CHANGELOG.md).
- `winterbaume-textract` v0.2.2: dependency-only. See [`crates/winterbaume-textract/CHANGELOG.md`](crates/winterbaume-textract/CHANGELOG.md).
- `winterbaume-tfstate-resource-models` v0.3.0: new SQS, CloudWatch and Lambda fields. See [`crates/winterbaume-tfstate-resource-models/CHANGELOG.md`](crates/winterbaume-tfstate-resource-models/CHANGELOG.md).
- `winterbaume-timestreaminfluxdb` v0.2.2: dependency-only. See [`crates/winterbaume-timestreaminfluxdb/CHANGELOG.md`](crates/winterbaume-timestreaminfluxdb/CHANGELOG.md).
- `winterbaume-timestreamquery` v0.2.2: dependency-only. See [`crates/winterbaume-timestreamquery/CHANGELOG.md`](crates/winterbaume-timestreamquery/CHANGELOG.md).
- `winterbaume-timestreamwrite` v0.2.2: dependency-only. See [`crates/winterbaume-timestreamwrite/CHANGELOG.md`](crates/winterbaume-timestreamwrite/CHANGELOG.md).
- `winterbaume-transcribe` v0.2.2: dependency-only. See [`crates/winterbaume-transcribe/CHANGELOG.md`](crates/winterbaume-transcribe/CHANGELOG.md).
- `winterbaume-transfer` v0.2.2: dependency-only. See [`crates/winterbaume-transfer/CHANGELOG.md`](crates/winterbaume-transfer/CHANGELOG.md).
- `winterbaume-trustedadvisor` v0.2.2: dependency-only. See [`crates/winterbaume-trustedadvisor/CHANGELOG.md`](crates/winterbaume-trustedadvisor/CHANGELOG.md).
- `winterbaume-vpclattice` v0.2.2: dependency-only. See [`crates/winterbaume-vpclattice/CHANGELOG.md`](crates/winterbaume-vpclattice/CHANGELOG.md).
- `winterbaume-wafv2` v0.2.2: dependency-only. See [`crates/winterbaume-wafv2/CHANGELOG.md`](crates/winterbaume-wafv2/CHANGELOG.md).
- `winterbaume-workspaces` v0.2.2: dependency-only. See [`crates/winterbaume-workspaces/CHANGELOG.md`](crates/winterbaume-workspaces/CHANGELOG.md).
- `winterbaume-workspacesweb` v0.2.2: dependency-only. See [`crates/winterbaume-workspacesweb/CHANGELOG.md`](crates/winterbaume-workspacesweb/CHANGELOG.md).
- `winterbaume-xray` v0.2.2: dependency-only. See [`crates/winterbaume-xray/CHANGELOG.md`](crates/winterbaume-xray/CHANGELOG.md).

## 2026-11-21

- Fixes for inputs the services accepted and then dropped ( #41 – #47, #50 – #54 ), CloudFront resources in Cloud Control backed by live CloudFront state ( #37, #38 ) and DynamoDB secondary-index projections ( #39 ).
- Opt-in S3 bucket policy enforcement ( #49 ), through `S3Service::with_bucket_policy_enforcement` or `winterbaume-server --s3-enforce-bucket-policies`.
- Percent-encoded UTF-8 is decoded as UTF-8 everywhere ( #58 ). The shared decoder in `winterbaume-core` and about sixty private copies of it turned each byte into its own character. `winterbaume-core` gains a public function, so every crate that depends on it is republished; most of those releases are dependency-only.

- `winterbaume-accessanalyzer` v0.2.1: non-ASCII percent-decoding (#58). See [`crates/winterbaume-accessanalyzer/CHANGELOG.md`](crates/winterbaume-accessanalyzer/CHANGELOG.md).
- `winterbaume-account` v1.0.1: dependency-only. See [`crates/winterbaume-account/CHANGELOG.md`](crates/winterbaume-account/CHANGELOG.md).
- `winterbaume-acm` v0.2.1: dependency-only. See [`crates/winterbaume-acm/CHANGELOG.md`](crates/winterbaume-acm/CHANGELOG.md).
- `winterbaume-acmpca` v0.2.1: dependency-only. See [`crates/winterbaume-acmpca/CHANGELOG.md`](crates/winterbaume-acmpca/CHANGELOG.md).
- `winterbaume-aiops` v0.2.1: dependency-only. See [`crates/winterbaume-aiops/CHANGELOG.md`](crates/winterbaume-aiops/CHANGELOG.md).
- `winterbaume-amp` v0.2.1: non-ASCII percent-decoding (#58). See [`crates/winterbaume-amp/CHANGELOG.md`](crates/winterbaume-amp/CHANGELOG.md).
- `winterbaume-amplify` v0.2.1: non-ASCII percent-decoding (#58). See [`crates/winterbaume-amplify/CHANGELOG.md`](crates/winterbaume-amplify/CHANGELOG.md).
- `winterbaume-amplifybackend` v0.2.1: dependency-only. See [`crates/winterbaume-amplifybackend/CHANGELOG.md`](crates/winterbaume-amplifybackend/CHANGELOG.md).
- `winterbaume-amplifyuibuilder` v0.2.1: dependency-only. See [`crates/winterbaume-amplifyuibuilder/CHANGELOG.md`](crates/winterbaume-amplifyuibuilder/CHANGELOG.md).
- `winterbaume-apigateway` v1.0.1: shared decoder; no behaviour change. See [`crates/winterbaume-apigateway/CHANGELOG.md`](crates/winterbaume-apigateway/CHANGELOG.md).
- `winterbaume-apigatewaymanagement` v0.2.1: non-ASCII percent-decoding (#58). See [`crates/winterbaume-apigatewaymanagement/CHANGELOG.md`](crates/winterbaume-apigatewaymanagement/CHANGELOG.md).
- `winterbaume-apigatewayv2` v0.2.1: non-ASCII percent-decoding (#58). See [`crates/winterbaume-apigatewayv2/CHANGELOG.md`](crates/winterbaume-apigatewayv2/CHANGELOG.md).
- `winterbaume-appconfig` v2.0.1: dependency-only. See [`crates/winterbaume-appconfig/CHANGELOG.md`](crates/winterbaume-appconfig/CHANGELOG.md).
- `winterbaume-appconfigdata` v0.3.2: dependency-only. See [`crates/winterbaume-appconfigdata/CHANGELOG.md`](crates/winterbaume-appconfigdata/CHANGELOG.md).
- `winterbaume-appfabric` v0.2.1: dependency-only. See [`crates/winterbaume-appfabric/CHANGELOG.md`](crates/winterbaume-appfabric/CHANGELOG.md).
- `winterbaume-appflow` v0.2.1: dependency-only. See [`crates/winterbaume-appflow/CHANGELOG.md`](crates/winterbaume-appflow/CHANGELOG.md).
- `winterbaume-appintegrations` v0.2.1: dependency-only. See [`crates/winterbaume-appintegrations/CHANGELOG.md`](crates/winterbaume-appintegrations/CHANGELOG.md).
- `winterbaume-applicationautoscaling` v0.2.1: dependency-only. See [`crates/winterbaume-applicationautoscaling/CHANGELOG.md`](crates/winterbaume-applicationautoscaling/CHANGELOG.md).
- `winterbaume-applicationcostprofiler` v0.2.1: dependency-only. See [`crates/winterbaume-applicationcostprofiler/CHANGELOG.md`](crates/winterbaume-applicationcostprofiler/CHANGELOG.md).
- `winterbaume-applicationdiscoveryservice` v0.2.1: dependency-only. See [`crates/winterbaume-applicationdiscoveryservice/CHANGELOG.md`](crates/winterbaume-applicationdiscoveryservice/CHANGELOG.md).
- `winterbaume-applicationinsights` v0.2.1: dependency-only. See [`crates/winterbaume-applicationinsights/CHANGELOG.md`](crates/winterbaume-applicationinsights/CHANGELOG.md).
- `winterbaume-applicationsignals` v1.0.1: dependency-only. See [`crates/winterbaume-applicationsignals/CHANGELOG.md`](crates/winterbaume-applicationsignals/CHANGELOG.md).
- `winterbaume-appmesh` v0.2.1: dependency-only. See [`crates/winterbaume-appmesh/CHANGELOG.md`](crates/winterbaume-appmesh/CHANGELOG.md).
- `winterbaume-apprunner` v0.2.1: dependency-only. See [`crates/winterbaume-apprunner/CHANGELOG.md`](crates/winterbaume-apprunner/CHANGELOG.md).
- `winterbaume-appsync` v0.2.1: non-ASCII percent-decoding (#58). See [`crates/winterbaume-appsync/CHANGELOG.md`](crates/winterbaume-appsync/CHANGELOG.md).
- `winterbaume-arczonalshift` v0.2.1: dependency-only. See [`crates/winterbaume-arczonalshift/CHANGELOG.md`](crates/winterbaume-arczonalshift/CHANGELOG.md).
- `winterbaume-artifact` v0.2.1: dependency-only. See [`crates/winterbaume-artifact/CHANGELOG.md`](crates/winterbaume-artifact/CHANGELOG.md).
- `winterbaume-athena` v0.2.1: dependency-only. See [`crates/winterbaume-athena/CHANGELOG.md`](crates/winterbaume-athena/CHANGELOG.md).
- `winterbaume-auditmanager` v0.2.1: non-ASCII percent-decoding (#58). See [`crates/winterbaume-auditmanager/CHANGELOG.md`](crates/winterbaume-auditmanager/CHANGELOG.md).
- `winterbaume-autoscaling` v0.2.2: non-ASCII percent-decoding (#58). See [`crates/winterbaume-autoscaling/CHANGELOG.md`](crates/winterbaume-autoscaling/CHANGELOG.md).
- `winterbaume-autoscalingplans` v0.2.1: dependency-only. See [`crates/winterbaume-autoscalingplans/CHANGELOG.md`](crates/winterbaume-autoscalingplans/CHANGELOG.md).
- `winterbaume-backup` v0.2.1: dependency-only. See [`crates/winterbaume-backup/CHANGELOG.md`](crates/winterbaume-backup/CHANGELOG.md).
- `winterbaume-backupgateway` v0.2.1: dependency-only. See [`crates/winterbaume-backupgateway/CHANGELOG.md`](crates/winterbaume-backupgateway/CHANGELOG.md).
- `winterbaume-backupsearch` v0.2.1: dependency-only. See [`crates/winterbaume-backupsearch/CHANGELOG.md`](crates/winterbaume-backupsearch/CHANGELOG.md).
- `winterbaume-batch` v1.0.1: dependency-only. See [`crates/winterbaume-batch/CHANGELOG.md`](crates/winterbaume-batch/CHANGELOG.md).
- `winterbaume-bcmdashboards` v0.2.1: dependency-only. See [`crates/winterbaume-bcmdashboards/CHANGELOG.md`](crates/winterbaume-bcmdashboards/CHANGELOG.md).
- `winterbaume-bcmdataexports` v0.2.1: dependency-only. See [`crates/winterbaume-bcmdataexports/CHANGELOG.md`](crates/winterbaume-bcmdataexports/CHANGELOG.md).
- `winterbaume-bcmrecommendedactions` v0.2.1: dependency-only. See [`crates/winterbaume-bcmrecommendedactions/CHANGELOG.md`](crates/winterbaume-bcmrecommendedactions/CHANGELOG.md).
- `winterbaume-bedrock` v0.2.1: non-ASCII percent-decoding (#58). See [`crates/winterbaume-bedrock/CHANGELOG.md`](crates/winterbaume-bedrock/CHANGELOG.md).
- `winterbaume-bedrockagent` v0.2.1: non-ASCII percent-decoding (#58). See [`crates/winterbaume-bedrockagent/CHANGELOG.md`](crates/winterbaume-bedrockagent/CHANGELOG.md).
- `winterbaume-billing` v0.2.1: dependency-only. See [`crates/winterbaume-billing/CHANGELOG.md`](crates/winterbaume-billing/CHANGELOG.md).
- `winterbaume-braket` v0.2.1: dependency-only. See [`crates/winterbaume-braket/CHANGELOG.md`](crates/winterbaume-braket/CHANGELOG.md).
- `winterbaume-budgets` v0.2.1: dependency-only. See [`crates/winterbaume-budgets/CHANGELOG.md`](crates/winterbaume-budgets/CHANGELOG.md).
- `winterbaume-chatbot` v0.2.1: dependency-only. See [`crates/winterbaume-chatbot/CHANGELOG.md`](crates/winterbaume-chatbot/CHANGELOG.md).
- `winterbaume-chimesdkmeetings` v0.2.1: dependency-only. See [`crates/winterbaume-chimesdkmeetings/CHANGELOG.md`](crates/winterbaume-chimesdkmeetings/CHANGELOG.md).
- `winterbaume-cloud9` v0.2.1: dependency-only. See [`crates/winterbaume-cloud9/CHANGELOG.md`](crates/winterbaume-cloud9/CHANGELOG.md).
- `winterbaume-cloudcontrol` v1.2.0: CloudFront resources backed by live CloudFront state (#37, #38). See [`crates/winterbaume-cloudcontrol/CHANGELOG.md`](crates/winterbaume-cloudcontrol/CHANGELOG.md).
- `winterbaume-clouddirectory` v0.2.1: dependency-only. See [`crates/winterbaume-clouddirectory/CHANGELOG.md`](crates/winterbaume-clouddirectory/CHANGELOG.md).
- `winterbaume-cloudformation` v0.2.1: non-ASCII percent-decoding (#58). See [`crates/winterbaume-cloudformation/CHANGELOG.md`](crates/winterbaume-cloudformation/CHANGELOG.md).
- `winterbaume-cloudfront` v1.1.0: shared state for Cloud Control (#37, #38). See [`crates/winterbaume-cloudfront/CHANGELOG.md`](crates/winterbaume-cloudfront/CHANGELOG.md).
- `winterbaume-cloudfrontkeyvaluestore` v0.2.2: dependency-only. See [`crates/winterbaume-cloudfrontkeyvaluestore/CHANGELOG.md`](crates/winterbaume-cloudfrontkeyvaluestore/CHANGELOG.md).
- `winterbaume-cloudhsmv2` v0.2.1: dependency-only. See [`crates/winterbaume-cloudhsmv2/CHANGELOG.md`](crates/winterbaume-cloudhsmv2/CHANGELOG.md).
- `winterbaume-cloudsearchdomain` v0.2.1: dependency-only. See [`crates/winterbaume-cloudsearchdomain/CHANGELOG.md`](crates/winterbaume-cloudsearchdomain/CHANGELOG.md).
- `winterbaume-cloudtrail` v0.2.1: dependency-only. See [`crates/winterbaume-cloudtrail/CHANGELOG.md`](crates/winterbaume-cloudtrail/CHANGELOG.md).
- `winterbaume-cloudtraildata` v0.2.1: dependency-only. See [`crates/winterbaume-cloudtraildata/CHANGELOG.md`](crates/winterbaume-cloudtraildata/CHANGELOG.md).
- `winterbaume-cloudwatch` v0.2.1: dependency-only. See [`crates/winterbaume-cloudwatch/CHANGELOG.md`](crates/winterbaume-cloudwatch/CHANGELOG.md).
- `winterbaume-cloudwatchlogs` v0.2.1: dependency-only. See [`crates/winterbaume-cloudwatchlogs/CHANGELOG.md`](crates/winterbaume-cloudwatchlogs/CHANGELOG.md).
- `winterbaume-codeartifact` v0.2.1: non-ASCII percent-decoding (#58). See [`crates/winterbaume-codeartifact/CHANGELOG.md`](crates/winterbaume-codeartifact/CHANGELOG.md).
- `winterbaume-codebuild` v0.2.1: dependency-only. See [`crates/winterbaume-codebuild/CHANGELOG.md`](crates/winterbaume-codebuild/CHANGELOG.md).
- `winterbaume-codecommit` v0.2.1: dependency-only. See [`crates/winterbaume-codecommit/CHANGELOG.md`](crates/winterbaume-codecommit/CHANGELOG.md).
- `winterbaume-codedeploy` v0.2.2: dependency-only. See [`crates/winterbaume-codedeploy/CHANGELOG.md`](crates/winterbaume-codedeploy/CHANGELOG.md).
- `winterbaume-codegurureviewer` v0.2.1: dependency-only. See [`crates/winterbaume-codegurureviewer/CHANGELOG.md`](crates/winterbaume-codegurureviewer/CHANGELOG.md).
- `winterbaume-codegurusecurity` v0.2.1: dependency-only. See [`crates/winterbaume-codegurusecurity/CHANGELOG.md`](crates/winterbaume-codegurusecurity/CHANGELOG.md).
- `winterbaume-codepipeline` v0.2.1: dependency-only. See [`crates/winterbaume-codepipeline/CHANGELOG.md`](crates/winterbaume-codepipeline/CHANGELOG.md).
- `winterbaume-codestarnotifications` v0.2.1: dependency-only. See [`crates/winterbaume-codestarnotifications/CHANGELOG.md`](crates/winterbaume-codestarnotifications/CHANGELOG.md).
- `winterbaume-cognitoidentity` v0.2.1: dependency-only. See [`crates/winterbaume-cognitoidentity/CHANGELOG.md`](crates/winterbaume-cognitoidentity/CHANGELOG.md).
- `winterbaume-cognitoidentityprovider` v0.2.1: dependency-only. See [`crates/winterbaume-cognitoidentityprovider/CHANGELOG.md`](crates/winterbaume-cognitoidentityprovider/CHANGELOG.md).
- `winterbaume-cognitosync` v0.2.1: dependency-only. See [`crates/winterbaume-cognitosync/CHANGELOG.md`](crates/winterbaume-cognitosync/CHANGELOG.md).
- `winterbaume-comprehend` v0.2.1: dependency-only. See [`crates/winterbaume-comprehend/CHANGELOG.md`](crates/winterbaume-comprehend/CHANGELOG.md).
- `winterbaume-config` v0.2.1: dependency-only. See [`crates/winterbaume-config/CHANGELOG.md`](crates/winterbaume-config/CHANGELOG.md).
- `winterbaume-connect` v0.2.1: non-ASCII percent-decoding (#58). See [`crates/winterbaume-connect/CHANGELOG.md`](crates/winterbaume-connect/CHANGELOG.md).
- `winterbaume-connectcampaigns` v0.2.1: dependency-only. See [`crates/winterbaume-connectcampaigns/CHANGELOG.md`](crates/winterbaume-connectcampaigns/CHANGELOG.md).
- `winterbaume-connectcontactlens` v0.2.1: dependency-only. See [`crates/winterbaume-connectcontactlens/CHANGELOG.md`](crates/winterbaume-connectcontactlens/CHANGELOG.md).
- `winterbaume-connectparticipant` v0.2.1: dependency-only. See [`crates/winterbaume-connectparticipant/CHANGELOG.md`](crates/winterbaume-connectparticipant/CHANGELOG.md).
- `winterbaume-controlcatalog` v0.2.1: dependency-only. See [`crates/winterbaume-controlcatalog/CHANGELOG.md`](crates/winterbaume-controlcatalog/CHANGELOG.md).
- `winterbaume-core` v0.3.0: UTF-8 percent-decoding, and `percent_decode_path` (#58, #60). See [`crates/winterbaume-core/CHANGELOG.md`](crates/winterbaume-core/CHANGELOG.md).
- `winterbaume-costandusagereport` v0.2.1: dependency-only. See [`crates/winterbaume-costandusagereport/CHANGELOG.md`](crates/winterbaume-costandusagereport/CHANGELOG.md).
- `winterbaume-costexplorer` v0.2.1: dependency-only. See [`crates/winterbaume-costexplorer/CHANGELOG.md`](crates/winterbaume-costexplorer/CHANGELOG.md).
- `winterbaume-costoptimizationhub` v0.2.1: dependency-only. See [`crates/winterbaume-costoptimizationhub/CHANGELOG.md`](crates/winterbaume-costoptimizationhub/CHANGELOG.md).
- `winterbaume-databasemigration` v0.2.1: dependency-only. See [`crates/winterbaume-databasemigration/CHANGELOG.md`](crates/winterbaume-databasemigration/CHANGELOG.md).
- `winterbaume-databrew` v0.2.1: non-ASCII percent-decoding (#58). See [`crates/winterbaume-databrew/CHANGELOG.md`](crates/winterbaume-databrew/CHANGELOG.md).
- `winterbaume-datapipeline` v0.2.1: dependency-only. See [`crates/winterbaume-datapipeline/CHANGELOG.md`](crates/winterbaume-datapipeline/CHANGELOG.md).
- `winterbaume-datasync` v0.2.1: dependency-only. See [`crates/winterbaume-datasync/CHANGELOG.md`](crates/winterbaume-datasync/CHANGELOG.md).
- `winterbaume-dax` v0.2.1: dependency-only. See [`crates/winterbaume-dax/CHANGELOG.md`](crates/winterbaume-dax/CHANGELOG.md).
- `winterbaume-directconnect` v0.2.1: dependency-only. See [`crates/winterbaume-directconnect/CHANGELOG.md`](crates/winterbaume-directconnect/CHANGELOG.md).
- `winterbaume-directory` v0.2.1: dependency-only. See [`crates/winterbaume-directory/CHANGELOG.md`](crates/winterbaume-directory/CHANGELOG.md).
- `winterbaume-dlm` v0.2.1: dependency-only. See [`crates/winterbaume-dlm/CHANGELOG.md`](crates/winterbaume-dlm/CHANGELOG.md).
- `winterbaume-dsql` v0.2.1: non-ASCII percent-decoding (#58). See [`crates/winterbaume-dsql/CHANGELOG.md`](crates/winterbaume-dsql/CHANGELOG.md).
- `winterbaume-dynamodb` v0.4.0: index projections, deletion protection, SSE, `CreateTable` tags (#39, #42, #43, #51). See [`crates/winterbaume-dynamodb/CHANGELOG.md`](crates/winterbaume-dynamodb/CHANGELOG.md).
- `winterbaume-dynamodb-redis` v0.2.2: the same DynamoDB fixes in the Redis backend (#39, #42, #43, #51). See [`crates/winterbaume-dynamodb-redis/CHANGELOG.md`](crates/winterbaume-dynamodb-redis/CHANGELOG.md).
- `winterbaume-dynamodbstreams` v0.2.2: dependency-only. See [`crates/winterbaume-dynamodbstreams/CHANGELOG.md`](crates/winterbaume-dynamodbstreams/CHANGELOG.md).
- `winterbaume-ebs` v0.2.1: non-ASCII percent-decoding (#58). See [`crates/winterbaume-ebs/CHANGELOG.md`](crates/winterbaume-ebs/CHANGELOG.md).
- `winterbaume-ec2` v1.0.1: Clippy cleanup only. See [`crates/winterbaume-ec2/CHANGELOG.md`](crates/winterbaume-ec2/CHANGELOG.md).
- `winterbaume-ec2-generated` v0.2.1: dependency-only. See [`crates/winterbaume-ec2-generated/CHANGELOG.md`](crates/winterbaume-ec2-generated/CHANGELOG.md).
- `winterbaume-ec2instanceconnect` v0.2.1: dependency-only. See [`crates/winterbaume-ec2instanceconnect/CHANGELOG.md`](crates/winterbaume-ec2instanceconnect/CHANGELOG.md).
- `winterbaume-ecr` v0.2.1: dependency-only. See [`crates/winterbaume-ecr/CHANGELOG.md`](crates/winterbaume-ecr/CHANGELOG.md).
- `winterbaume-ecs` v0.2.1: dependency-only. See [`crates/winterbaume-ecs/CHANGELOG.md`](crates/winterbaume-ecs/CHANGELOG.md).
- `winterbaume-efs` v0.2.1: non-ASCII percent-decoding (#58). See [`crates/winterbaume-efs/CHANGELOG.md`](crates/winterbaume-efs/CHANGELOG.md).
- `winterbaume-eks` v0.2.1: non-ASCII percent-decoding (#58). See [`crates/winterbaume-eks/CHANGELOG.md`](crates/winterbaume-eks/CHANGELOG.md).
- `winterbaume-elasticache` v0.2.1: non-ASCII percent-decoding (#58). See [`crates/winterbaume-elasticache/CHANGELOG.md`](crates/winterbaume-elasticache/CHANGELOG.md).
- `winterbaume-elasticbeanstalk` v0.2.1: non-ASCII percent-decoding (#58). See [`crates/winterbaume-elasticbeanstalk/CHANGELOG.md`](crates/winterbaume-elasticbeanstalk/CHANGELOG.md).
- `winterbaume-elasticloadbalancing` v0.2.1: non-ASCII percent-decoding (#58). See [`crates/winterbaume-elasticloadbalancing/CHANGELOG.md`](crates/winterbaume-elasticloadbalancing/CHANGELOG.md).
- `winterbaume-elasticloadbalancingv2` v0.2.1: non-ASCII percent-decoding (#58). See [`crates/winterbaume-elasticloadbalancingv2/CHANGELOG.md`](crates/winterbaume-elasticloadbalancingv2/CHANGELOG.md).
- `winterbaume-emr` v0.2.1: dependency-only. See [`crates/winterbaume-emr/CHANGELOG.md`](crates/winterbaume-emr/CHANGELOG.md).
- `winterbaume-emrcontainers` v0.2.1: non-ASCII percent-decoding (#58). See [`crates/winterbaume-emrcontainers/CHANGELOG.md`](crates/winterbaume-emrcontainers/CHANGELOG.md).
- `winterbaume-emrserverless` v0.2.1: dependency-only. See [`crates/winterbaume-emrserverless/CHANGELOG.md`](crates/winterbaume-emrserverless/CHANGELOG.md).
- `winterbaume-eventbridge` v0.2.1: `PutTargets` keeps every target member (#47). See [`crates/winterbaume-eventbridge/CHANGELOG.md`](crates/winterbaume-eventbridge/CHANGELOG.md).
- `winterbaume-firehose` v0.2.1: dependency-only. See [`crates/winterbaume-firehose/CHANGELOG.md`](crates/winterbaume-firehose/CHANGELOG.md).
- `winterbaume-fis` v0.2.2: non-ASCII percent-decoding (#58). See [`crates/winterbaume-fis/CHANGELOG.md`](crates/winterbaume-fis/CHANGELOG.md).
- `winterbaume-forecast` v0.2.1: dependency-only. See [`crates/winterbaume-forecast/CHANGELOG.md`](crates/winterbaume-forecast/CHANGELOG.md).
- `winterbaume-freetier` v0.2.1: dependency-only. See [`crates/winterbaume-freetier/CHANGELOG.md`](crates/winterbaume-freetier/CHANGELOG.md).
- `winterbaume-fsx` v0.2.1: dependency-only. See [`crates/winterbaume-fsx/CHANGELOG.md`](crates/winterbaume-fsx/CHANGELOG.md).
- `winterbaume-glacier` v0.2.1: dependency-only. See [`crates/winterbaume-glacier/CHANGELOG.md`](crates/winterbaume-glacier/CHANGELOG.md).
- `winterbaume-glue` v0.2.1: dependency-only. See [`crates/winterbaume-glue/CHANGELOG.md`](crates/winterbaume-glue/CHANGELOG.md).
- `winterbaume-greengrass` v0.2.1: dependency-only. See [`crates/winterbaume-greengrass/CHANGELOG.md`](crates/winterbaume-greengrass/CHANGELOG.md).
- `winterbaume-guardduty` v0.2.1: dependency-only. See [`crates/winterbaume-guardduty/CHANGELOG.md`](crates/winterbaume-guardduty/CHANGELOG.md).
- `winterbaume-iam` v0.2.2: role and user tags and permissions boundaries, non-ASCII form values (#52, #58). See [`crates/winterbaume-iam/CHANGELOG.md`](crates/winterbaume-iam/CHANGELOG.md).
- `winterbaume-iam-rule-eval` v0.3.0: condition and principal evaluation for resource policies (#49). See [`crates/winterbaume-iam-rule-eval/CHANGELOG.md`](crates/winterbaume-iam-rule-eval/CHANGELOG.md).
- `winterbaume-identitystore` v0.2.1: dependency-only. See [`crates/winterbaume-identitystore/CHANGELOG.md`](crates/winterbaume-identitystore/CHANGELOG.md).
- `winterbaume-inspector2` v0.2.1: non-ASCII percent-decoding (#58). See [`crates/winterbaume-inspector2/CHANGELOG.md`](crates/winterbaume-inspector2/CHANGELOG.md).
- `winterbaume-iot` v0.2.1: non-ASCII percent-decoding (#58). See [`crates/winterbaume-iot/CHANGELOG.md`](crates/winterbaume-iot/CHANGELOG.md).
- `winterbaume-iotdataplane` v0.2.1: non-ASCII percent-decoding (#58). See [`crates/winterbaume-iotdataplane/CHANGELOG.md`](crates/winterbaume-iotdataplane/CHANGELOG.md).
- `winterbaume-ivs` v1.0.1: non-ASCII percent-decoding (#58). See [`crates/winterbaume-ivs/CHANGELOG.md`](crates/winterbaume-ivs/CHANGELOG.md).
- `winterbaume-kafka` v0.2.1: non-ASCII percent-decoding (#58). See [`crates/winterbaume-kafka/CHANGELOG.md`](crates/winterbaume-kafka/CHANGELOG.md).
- `winterbaume-keyspaces` v0.2.2: dependency-only. See [`crates/winterbaume-keyspaces/CHANGELOG.md`](crates/winterbaume-keyspaces/CHANGELOG.md).
- `winterbaume-kinesis` v0.2.2: dependency-only. See [`crates/winterbaume-kinesis/CHANGELOG.md`](crates/winterbaume-kinesis/CHANGELOG.md).
- `winterbaume-kinesisanalyticsv2` v0.2.1: dependency-only. See [`crates/winterbaume-kinesisanalyticsv2/CHANGELOG.md`](crates/winterbaume-kinesisanalyticsv2/CHANGELOG.md).
- `winterbaume-kinesisvideo` v0.3.1: dependency-only. See [`crates/winterbaume-kinesisvideo/CHANGELOG.md`](crates/winterbaume-kinesisvideo/CHANGELOG.md).
- `winterbaume-kinesisvideoarchivedmedia` v0.3.1: dependency-only. See [`crates/winterbaume-kinesisvideoarchivedmedia/CHANGELOG.md`](crates/winterbaume-kinesisvideoarchivedmedia/CHANGELOG.md).
- `winterbaume-kms` v0.2.2: dependency-only. See [`crates/winterbaume-kms/CHANGELOG.md`](crates/winterbaume-kms/CHANGELOG.md).
- `winterbaume-lakeformation` v0.2.1: dependency-only. See [`crates/winterbaume-lakeformation/CHANGELOG.md`](crates/winterbaume-lakeformation/CHANGELOG.md).
- `winterbaume-lambda` v2.1.0: permission conditions, event source mapping settings, filters, ARN and tags (#44, #45, #46, #54). See [`crates/winterbaume-lambda/CHANGELOG.md`](crates/winterbaume-lambda/CHANGELOG.md).
- `winterbaume-lexmodelsv2` v0.2.1: dependency-only. See [`crates/winterbaume-lexmodelsv2/CHANGELOG.md`](crates/winterbaume-lexmodelsv2/CHANGELOG.md).
- `winterbaume-macie2` v0.2.1: dependency-only. See [`crates/winterbaume-macie2/CHANGELOG.md`](crates/winterbaume-macie2/CHANGELOG.md).
- `winterbaume-managedblockchain` v0.2.2: non-ASCII percent-decoding (#58). See [`crates/winterbaume-managedblockchain/CHANGELOG.md`](crates/winterbaume-managedblockchain/CHANGELOG.md).
- `winterbaume-marketplacemetering` v0.2.1: dependency-only. See [`crates/winterbaume-marketplacemetering/CHANGELOG.md`](crates/winterbaume-marketplacemetering/CHANGELOG.md).
- `winterbaume-mediaconnect` v0.2.1: dependency-only. See [`crates/winterbaume-mediaconnect/CHANGELOG.md`](crates/winterbaume-mediaconnect/CHANGELOG.md).
- `winterbaume-medialive` v0.2.1: dependency-only. See [`crates/winterbaume-medialive/CHANGELOG.md`](crates/winterbaume-medialive/CHANGELOG.md).
- `winterbaume-mediapackage` v0.2.1: non-ASCII percent-decoding (#58). See [`crates/winterbaume-mediapackage/CHANGELOG.md`](crates/winterbaume-mediapackage/CHANGELOG.md).
- `winterbaume-mediapackagev2` v0.2.1: dependency-only. See [`crates/winterbaume-mediapackagev2/CHANGELOG.md`](crates/winterbaume-mediapackagev2/CHANGELOG.md).
- `winterbaume-mediastore` v0.2.1: dependency-only. See [`crates/winterbaume-mediastore/CHANGELOG.md`](crates/winterbaume-mediastore/CHANGELOG.md).
- `winterbaume-mediastoredata` v0.2.1: non-ASCII percent-decoding (#58). See [`crates/winterbaume-mediastoredata/CHANGELOG.md`](crates/winterbaume-mediastoredata/CHANGELOG.md).
- `winterbaume-memorydb` v0.2.1: dependency-only. See [`crates/winterbaume-memorydb/CHANGELOG.md`](crates/winterbaume-memorydb/CHANGELOG.md).
- `winterbaume-mq` v0.2.1: non-ASCII percent-decoding (#58). See [`crates/winterbaume-mq/CHANGELOG.md`](crates/winterbaume-mq/CHANGELOG.md).
- `winterbaume-neptune` v0.2.1: non-ASCII percent-decoding (#58). See [`crates/winterbaume-neptune/CHANGELOG.md`](crates/winterbaume-neptune/CHANGELOG.md).
- `winterbaume-networkfirewall` v0.2.1: dependency-only. See [`crates/winterbaume-networkfirewall/CHANGELOG.md`](crates/winterbaume-networkfirewall/CHANGELOG.md).
- `winterbaume-networkmanager` v0.2.1: non-ASCII percent-decoding (#58). See [`crates/winterbaume-networkmanager/CHANGELOG.md`](crates/winterbaume-networkmanager/CHANGELOG.md).
- `winterbaume-opensearch` v1.0.1: non-ASCII percent-decoding (#58). See [`crates/winterbaume-opensearch/CHANGELOG.md`](crates/winterbaume-opensearch/CHANGELOG.md).
- `winterbaume-opensearchserverless` v0.2.2: dependency-only. See [`crates/winterbaume-opensearchserverless/CHANGELOG.md`](crates/winterbaume-opensearchserverless/CHANGELOG.md).
- `winterbaume-organizations` v1.0.1: dependency-only. See [`crates/winterbaume-organizations/CHANGELOG.md`](crates/winterbaume-organizations/CHANGELOG.md).
- `winterbaume-osis` v0.2.1: non-ASCII percent-decoding (#58). See [`crates/winterbaume-osis/CHANGELOG.md`](crates/winterbaume-osis/CHANGELOG.md).
- `winterbaume-outposts` v0.2.1: non-ASCII percent-decoding (#58). See [`crates/winterbaume-outposts/CHANGELOG.md`](crates/winterbaume-outposts/CHANGELOG.md).
- `winterbaume-panorama` v0.2.1: dependency-only. See [`crates/winterbaume-panorama/CHANGELOG.md`](crates/winterbaume-panorama/CHANGELOG.md).
- `winterbaume-pcaconnectorscep` v0.2.1: dependency-only. See [`crates/winterbaume-pcaconnectorscep/CHANGELOG.md`](crates/winterbaume-pcaconnectorscep/CHANGELOG.md).
- `winterbaume-personalize` v0.2.1: dependency-only. See [`crates/winterbaume-personalize/CHANGELOG.md`](crates/winterbaume-personalize/CHANGELOG.md).
- `winterbaume-personalizeevents` v0.2.1: dependency-only. See [`crates/winterbaume-personalizeevents/CHANGELOG.md`](crates/winterbaume-personalizeevents/CHANGELOG.md).
- `winterbaume-personalizeruntime` v0.2.1: dependency-only. See [`crates/winterbaume-personalizeruntime/CHANGELOG.md`](crates/winterbaume-personalizeruntime/CHANGELOG.md).
- `winterbaume-pi` v0.2.1: dependency-only. See [`crates/winterbaume-pi/CHANGELOG.md`](crates/winterbaume-pi/CHANGELOG.md).
- `winterbaume-pinpoint` v0.2.1: dependency-only. See [`crates/winterbaume-pinpoint/CHANGELOG.md`](crates/winterbaume-pinpoint/CHANGELOG.md).
- `winterbaume-pinpointsmsvoice` v0.2.1: dependency-only. See [`crates/winterbaume-pinpointsmsvoice/CHANGELOG.md`](crates/winterbaume-pinpointsmsvoice/CHANGELOG.md).
- `winterbaume-pipes` v0.2.1: dependency-only. See [`crates/winterbaume-pipes/CHANGELOG.md`](crates/winterbaume-pipes/CHANGELOG.md).
- `winterbaume-polly` v0.2.1: dependency-only. See [`crates/winterbaume-polly/CHANGELOG.md`](crates/winterbaume-polly/CHANGELOG.md).
- `winterbaume-pricing` v0.2.1: dependency-only. See [`crates/winterbaume-pricing/CHANGELOG.md`](crates/winterbaume-pricing/CHANGELOG.md).
- `winterbaume-quicksight` v0.2.1: non-ASCII percent-decoding (#58). See [`crates/winterbaume-quicksight/CHANGELOG.md`](crates/winterbaume-quicksight/CHANGELOG.md).
- `winterbaume-ram` v0.2.1: dependency-only. See [`crates/winterbaume-ram/CHANGELOG.md`](crates/winterbaume-ram/CHANGELOG.md).
- `winterbaume-rbin` v0.2.1: dependency-only. See [`crates/winterbaume-rbin/CHANGELOG.md`](crates/winterbaume-rbin/CHANGELOG.md).
- `winterbaume-rds` v0.2.1: dependency-only. See [`crates/winterbaume-rds/CHANGELOG.md`](crates/winterbaume-rds/CHANGELOG.md).
- `winterbaume-rdsdata` v0.2.1: dependency-only. See [`crates/winterbaume-rdsdata/CHANGELOG.md`](crates/winterbaume-rdsdata/CHANGELOG.md).
- `winterbaume-redshift` v0.2.1: non-ASCII percent-decoding (#58). See [`crates/winterbaume-redshift/CHANGELOG.md`](crates/winterbaume-redshift/CHANGELOG.md).
- `winterbaume-redshiftdata` v0.2.1: dependency-only. See [`crates/winterbaume-redshiftdata/CHANGELOG.md`](crates/winterbaume-redshiftdata/CHANGELOG.md).
- `winterbaume-rekognition` v0.2.1: dependency-only. See [`crates/winterbaume-rekognition/CHANGELOG.md`](crates/winterbaume-rekognition/CHANGELOG.md).
- `winterbaume-resiliencehub` v0.2.1: non-ASCII percent-decoding (#58). See [`crates/winterbaume-resiliencehub/CHANGELOG.md`](crates/winterbaume-resiliencehub/CHANGELOG.md).
- `winterbaume-resourcegroups` v0.2.1: dependency-only. See [`crates/winterbaume-resourcegroups/CHANGELOG.md`](crates/winterbaume-resourcegroups/CHANGELOG.md).
- `winterbaume-resourcegroupstagging` v0.2.1: dependency-only. See [`crates/winterbaume-resourcegroupstagging/CHANGELOG.md`](crates/winterbaume-resourcegroupstagging/CHANGELOG.md).
- `winterbaume-rolesanywhere` v0.2.1: non-ASCII percent-decoding (#58). See [`crates/winterbaume-rolesanywhere/CHANGELOG.md`](crates/winterbaume-rolesanywhere/CHANGELOG.md).
- `winterbaume-route53` v0.2.1: non-ASCII percent-decoding (#58). See [`crates/winterbaume-route53/CHANGELOG.md`](crates/winterbaume-route53/CHANGELOG.md).
- `winterbaume-route53domains` v0.2.1: dependency-only. See [`crates/winterbaume-route53domains/CHANGELOG.md`](crates/winterbaume-route53domains/CHANGELOG.md).
- `winterbaume-route53recoverycluster` v0.2.1: dependency-only. See [`crates/winterbaume-route53recoverycluster/CHANGELOG.md`](crates/winterbaume-route53recoverycluster/CHANGELOG.md).
- `winterbaume-route53resolver` v0.2.1: dependency-only. See [`crates/winterbaume-route53resolver/CHANGELOG.md`](crates/winterbaume-route53resolver/CHANGELOG.md).
- `winterbaume-s3` v1.1.0: opt-in bucket policy enforcement, lifecycle minimum object size, ownership controls delete, `CreateBucket` configuration, non-ASCII keys (#41, #49, #50, #53, #58). See [`crates/winterbaume-s3/CHANGELOG.md`](crates/winterbaume-s3/CHANGELOG.md).
- `winterbaume-s3control` v0.3.0: bucket tags served from S3, custom-endpoint routing, `UntagResource` and error envelope fixes (#53). See [`crates/winterbaume-s3control/CHANGELOG.md`](crates/winterbaume-s3control/CHANGELOG.md).
- `winterbaume-s3files` v0.2.1: dependency-only. See [`crates/winterbaume-s3files/CHANGELOG.md`](crates/winterbaume-s3files/CHANGELOG.md).
- `winterbaume-s3outposts` v0.2.1: dependency-only. See [`crates/winterbaume-s3outposts/CHANGELOG.md`](crates/winterbaume-s3outposts/CHANGELOG.md).
- `winterbaume-s3tables` v0.2.1: non-ASCII percent-decoding (#58). See [`crates/winterbaume-s3tables/CHANGELOG.md`](crates/winterbaume-s3tables/CHANGELOG.md).
- `winterbaume-s3vectors` v0.2.1: dependency-only. See [`crates/winterbaume-s3vectors/CHANGELOG.md`](crates/winterbaume-s3vectors/CHANGELOG.md).
- `winterbaume-sagemaker` v0.3.1: dependency-only. See [`crates/winterbaume-sagemaker/CHANGELOG.md`](crates/winterbaume-sagemaker/CHANGELOG.md).
- `winterbaume-sagemakermetrics` v0.2.1: dependency-only. See [`crates/winterbaume-sagemakermetrics/CHANGELOG.md`](crates/winterbaume-sagemakermetrics/CHANGELOG.md).
- `winterbaume-sagemakerruntime` v0.3.1: non-ASCII percent-decoding (#58). See [`crates/winterbaume-sagemakerruntime/CHANGELOG.md`](crates/winterbaume-sagemakerruntime/CHANGELOG.md).
- `winterbaume-savingsplans` v0.2.1: dependency-only. See [`crates/winterbaume-savingsplans/CHANGELOG.md`](crates/winterbaume-savingsplans/CHANGELOG.md).
- `winterbaume-scheduler` v0.2.1: dependency-only. See [`crates/winterbaume-scheduler/CHANGELOG.md`](crates/winterbaume-scheduler/CHANGELOG.md).
- `winterbaume-secretsmanager` v0.2.1: dependency-only. See [`crates/winterbaume-secretsmanager/CHANGELOG.md`](crates/winterbaume-secretsmanager/CHANGELOG.md).
- `winterbaume-securityhub` v0.2.1: non-ASCII percent-decoding (#58). See [`crates/winterbaume-securityhub/CHANGELOG.md`](crates/winterbaume-securityhub/CHANGELOG.md).
- `winterbaume-server` v0.2.9: `--s3-enforce-bucket-policies`, forwarded-transport headers, Cloud Control and S3 Control links (#49, #53). See [`crates/winterbaume-server/CHANGELOG.md`](crates/winterbaume-server/CHANGELOG.md).
- `winterbaume-servicecatalog` v0.2.1: dependency-only. See [`crates/winterbaume-servicecatalog/CHANGELOG.md`](crates/winterbaume-servicecatalog/CHANGELOG.md).
- `winterbaume-servicecatalogappregistry` v0.2.1: non-ASCII percent-decoding (#58). See [`crates/winterbaume-servicecatalogappregistry/CHANGELOG.md`](crates/winterbaume-servicecatalogappregistry/CHANGELOG.md).
- `winterbaume-servicediscovery` v0.2.2: dependency-only. See [`crates/winterbaume-servicediscovery/CHANGELOG.md`](crates/winterbaume-servicediscovery/CHANGELOG.md).
- `winterbaume-servicequotas` v0.2.1: dependency-only. See [`crates/winterbaume-servicequotas/CHANGELOG.md`](crates/winterbaume-servicequotas/CHANGELOG.md).
- `winterbaume-ses` v0.3.1: non-ASCII percent-decoding (#58). See [`crates/winterbaume-ses/CHANGELOG.md`](crates/winterbaume-ses/CHANGELOG.md).
- `winterbaume-sesv2` v0.3.1: non-ASCII percent-decoding (#58). See [`crates/winterbaume-sesv2/CHANGELOG.md`](crates/winterbaume-sesv2/CHANGELOG.md).
- `winterbaume-sfn` v0.2.1: dependency-only. See [`crates/winterbaume-sfn/CHANGELOG.md`](crates/winterbaume-sfn/CHANGELOG.md).
- `winterbaume-shield` v0.2.1: dependency-only. See [`crates/winterbaume-shield/CHANGELOG.md`](crates/winterbaume-shield/CHANGELOG.md).
- `winterbaume-signer` v0.2.1: dependency-only. See [`crates/winterbaume-signer/CHANGELOG.md`](crates/winterbaume-signer/CHANGELOG.md).
- `winterbaume-simpledbv2` v0.2.1: dependency-only. See [`crates/winterbaume-simpledbv2/CHANGELOG.md`](crates/winterbaume-simpledbv2/CHANGELOG.md).
- `winterbaume-simspaceweaver` v0.2.1: dependency-only. See [`crates/winterbaume-simspaceweaver/CHANGELOG.md`](crates/winterbaume-simspaceweaver/CHANGELOG.md).
- `winterbaume-snowdevicemanagement` v0.2.1: dependency-only. See [`crates/winterbaume-snowdevicemanagement/CHANGELOG.md`](crates/winterbaume-snowdevicemanagement/CHANGELOG.md).
- `winterbaume-sns` v0.2.1: non-ASCII percent-decoding (#58). See [`crates/winterbaume-sns/CHANGELOG.md`](crates/winterbaume-sns/CHANGELOG.md).
- `winterbaume-sqlengine-duckdb` v0.2.1: dependency-only. See [`crates/winterbaume-sqlengine-duckdb/CHANGELOG.md`](crates/winterbaume-sqlengine-duckdb/CHANGELOG.md).
- `winterbaume-sqs` v0.2.1: dependency-only. See [`crates/winterbaume-sqs/CHANGELOG.md`](crates/winterbaume-sqs/CHANGELOG.md).
- `winterbaume-sqs-redis` v0.2.2: dependency-only. See [`crates/winterbaume-sqs-redis/CHANGELOG.md`](crates/winterbaume-sqs-redis/CHANGELOG.md).
- `winterbaume-ssm` v0.3.1: dependency-only. See [`crates/winterbaume-ssm/CHANGELOG.md`](crates/winterbaume-ssm/CHANGELOG.md).
- `winterbaume-ssmquicksetup` v0.2.1: dependency-only. See [`crates/winterbaume-ssmquicksetup/CHANGELOG.md`](crates/winterbaume-ssmquicksetup/CHANGELOG.md).
- `winterbaume-sso` v0.2.1: non-ASCII percent-decoding (#58). See [`crates/winterbaume-sso/CHANGELOG.md`](crates/winterbaume-sso/CHANGELOG.md).
- `winterbaume-ssoadmin` v0.2.1: dependency-only. See [`crates/winterbaume-ssoadmin/CHANGELOG.md`](crates/winterbaume-ssoadmin/CHANGELOG.md).
- `winterbaume-sts` v0.2.1: non-ASCII percent-decoding (#58). See [`crates/winterbaume-sts/CHANGELOG.md`](crates/winterbaume-sts/CHANGELOG.md).
- `winterbaume-support` v0.2.1: dependency-only. See [`crates/winterbaume-support/CHANGELOG.md`](crates/winterbaume-support/CHANGELOG.md).
- `winterbaume-supportapp` v0.2.1: dependency-only. See [`crates/winterbaume-supportapp/CHANGELOG.md`](crates/winterbaume-supportapp/CHANGELOG.md).
- `winterbaume-swf` v0.2.1: dependency-only. See [`crates/winterbaume-swf/CHANGELOG.md`](crates/winterbaume-swf/CHANGELOG.md).
- `winterbaume-synthetics` v0.2.1: non-ASCII percent-decoding (#58). See [`crates/winterbaume-synthetics/CHANGELOG.md`](crates/winterbaume-synthetics/CHANGELOG.md).
- `winterbaume-taxsettings` v0.2.1: dependency-only. See [`crates/winterbaume-taxsettings/CHANGELOG.md`](crates/winterbaume-taxsettings/CHANGELOG.md).
- `winterbaume-terraform` v0.2.7: converters for the newly stored Lambda, EventBridge and DynamoDB attributes. See [`crates/winterbaume-terraform/CHANGELOG.md`](crates/winterbaume-terraform/CHANGELOG.md).
- `winterbaume-textract` v0.2.1: dependency-only. See [`crates/winterbaume-textract/CHANGELOG.md`](crates/winterbaume-textract/CHANGELOG.md).
- `winterbaume-tfstate-resource-models` v0.2.1: regenerated models for the converters. See [`crates/winterbaume-tfstate-resource-models/CHANGELOG.md`](crates/winterbaume-tfstate-resource-models/CHANGELOG.md).
- `winterbaume-timestreaminfluxdb` v0.2.1: dependency-only. See [`crates/winterbaume-timestreaminfluxdb/CHANGELOG.md`](crates/winterbaume-timestreaminfluxdb/CHANGELOG.md).
- `winterbaume-timestreamquery` v0.2.1: dependency-only. See [`crates/winterbaume-timestreamquery/CHANGELOG.md`](crates/winterbaume-timestreamquery/CHANGELOG.md).
- `winterbaume-timestreamwrite` v0.2.1: dependency-only. See [`crates/winterbaume-timestreamwrite/CHANGELOG.md`](crates/winterbaume-timestreamwrite/CHANGELOG.md).
- `winterbaume-transcribe` v0.2.1: dependency-only. See [`crates/winterbaume-transcribe/CHANGELOG.md`](crates/winterbaume-transcribe/CHANGELOG.md).
- `winterbaume-transfer` v0.2.1: dependency-only. See [`crates/winterbaume-transfer/CHANGELOG.md`](crates/winterbaume-transfer/CHANGELOG.md).
- `winterbaume-trustedadvisor` v0.2.1: dependency-only. See [`crates/winterbaume-trustedadvisor/CHANGELOG.md`](crates/winterbaume-trustedadvisor/CHANGELOG.md).
- `winterbaume-vpclattice` v0.2.1: non-ASCII percent-decoding (#58). See [`crates/winterbaume-vpclattice/CHANGELOG.md`](crates/winterbaume-vpclattice/CHANGELOG.md).
- `winterbaume-wafv2` v0.2.1: dependency-only. See [`crates/winterbaume-wafv2/CHANGELOG.md`](crates/winterbaume-wafv2/CHANGELOG.md).
- `winterbaume-workspaces` v0.2.1: dependency-only. See [`crates/winterbaume-workspaces/CHANGELOG.md`](crates/winterbaume-workspaces/CHANGELOG.md).
- `winterbaume-workspacesweb` v0.2.1: non-ASCII percent-decoding (#58). See [`crates/winterbaume-workspacesweb/CHANGELOG.md`](crates/winterbaume-workspacesweb/CHANGELOG.md).
- `winterbaume-xray` v0.2.1: dependency-only. See [`crates/winterbaume-xray/CHANGELOG.md`](crates/winterbaume-xray/CHANGELOG.md).

## 2026-10-27

- `winterbaume-cloudcontrol` v1.1.0: see [`crates/winterbaume-cloudcontrol/CHANGELOG.md`](crates/winterbaume-cloudcontrol/CHANGELOG.md).
- `winterbaume-server` v0.2.8: see [`crates/winterbaume-server/CHANGELOG.md`](crates/winterbaume-server/CHANGELOG.md).
- `winterbaume-ssm` v0.3.0: see [`crates/winterbaume-ssm/CHANGELOG.md`](crates/winterbaume-ssm/CHANGELOG.md).
- `winterbaume-terraform` v0.2.6: see [`crates/winterbaume-terraform/CHANGELOG.md`](crates/winterbaume-terraform/CHANGELOG.md).

## 2026-10-25

- `winterbaume-keyspaces` v0.2.1: see [`crates/winterbaume-keyspaces/CHANGELOG.md`](crates/winterbaume-keyspaces/CHANGELOG.md).
- `winterbaume-s3` v1.0.1: see [`crates/winterbaume-s3/CHANGELOG.md`](crates/winterbaume-s3/CHANGELOG.md).
- `winterbaume-server` v0.2.7: see [`crates/winterbaume-server/CHANGELOG.md`](crates/winterbaume-server/CHANGELOG.md).
- `winterbaume-terraform` v0.2.5: see [`crates/winterbaume-terraform/CHANGELOG.md`](crates/winterbaume-terraform/CHANGELOG.md).

## 2026-08-17

- `winterbaume-autoscaling` v0.2.1: see [`crates/winterbaume-autoscaling/CHANGELOG.md`](crates/winterbaume-autoscaling/CHANGELOG.md).
- `winterbaume-cloudfront` v1.0.1: see [`crates/winterbaume-cloudfront/CHANGELOG.md`](crates/winterbaume-cloudfront/CHANGELOG.md).
- `winterbaume-codedeploy` v0.2.1: see [`crates/winterbaume-codedeploy/CHANGELOG.md`](crates/winterbaume-codedeploy/CHANGELOG.md).
- `winterbaume-dynamodb` v0.3.0: see [`crates/winterbaume-dynamodb/CHANGELOG.md`](crates/winterbaume-dynamodb/CHANGELOG.md).
- `winterbaume-dynamodb-redis` v0.2.1: see [`crates/winterbaume-dynamodb-redis/CHANGELOG.md`](crates/winterbaume-dynamodb-redis/CHANGELOG.md).
- `winterbaume-dynamodbstreams` v0.2.1: see [`crates/winterbaume-dynamodbstreams/CHANGELOG.md`](crates/winterbaume-dynamodbstreams/CHANGELOG.md).
- `winterbaume-fis` v0.2.1: see [`crates/winterbaume-fis/CHANGELOG.md`](crates/winterbaume-fis/CHANGELOG.md).
- `winterbaume-iam` v0.2.1: see [`crates/winterbaume-iam/CHANGELOG.md`](crates/winterbaume-iam/CHANGELOG.md).
- `winterbaume-managedblockchain` v0.2.1: see [`crates/winterbaume-managedblockchain/CHANGELOG.md`](crates/winterbaume-managedblockchain/CHANGELOG.md).
- `winterbaume-server` v0.2.6: see [`crates/winterbaume-server/CHANGELOG.md`](crates/winterbaume-server/CHANGELOG.md).
- `winterbaume-servicediscovery` v0.2.1: see [`crates/winterbaume-servicediscovery/CHANGELOG.md`](crates/winterbaume-servicediscovery/CHANGELOG.md).
- `winterbaume-terraform` v0.2.4: see [`crates/winterbaume-terraform/CHANGELOG.md`](crates/winterbaume-terraform/CHANGELOG.md).

## 2026-07-03

- `winterbaume` v0.7.0: see [`crates/winterbaume/CHANGELOG.md`](crates/winterbaume/CHANGELOG.md).
- `winterbaume-appconfigdata` v0.3.1: see [`crates/winterbaume-appconfigdata/CHANGELOG.md`](crates/winterbaume-appconfigdata/CHANGELOG.md).
- `winterbaume-opensearchserverless` v0.2.1: see [`crates/winterbaume-opensearchserverless/CHANGELOG.md`](crates/winterbaume-opensearchserverless/CHANGELOG.md).
- `winterbaume-server` v0.2.5: see [`crates/winterbaume-server/CHANGELOG.md`](crates/winterbaume-server/CHANGELOG.md).
- `winterbaume-terraform` v0.2.3: see [`crates/winterbaume-terraform/CHANGELOG.md`](crates/winterbaume-terraform/CHANGELOG.md).

## v0.7.0 - 2026-07-03

### Fixed

- S3 `UploadPart` and similar streaming operations now correctly handle binary request bodies; `@httpPayload` blob members are treated as opaque bytes rather than UTF-8 strings, preventing corruption of non-text payloads.
- OpenSearch Serverless policy-version assignments no longer produce intermittent ordering failures under concurrent test runs.

### Internal

- Corrected `usage_options` interpolation in mise release tasks.

## 2026-07-03

- `winterbaume` v0.7.0: see [`crates/winterbaume/CHANGELOG.md`](crates/winterbaume/CHANGELOG.md).
- `winterbaume-apigateway` v0.3.0: see [`crates/winterbaume-apigateway/CHANGELOG.md`](crates/winterbaume-apigateway/CHANGELOG.md).
- `winterbaume-appconfig` v2.0.0: see [`crates/winterbaume-appconfig/CHANGELOG.md`](crates/winterbaume-appconfig/CHANGELOG.md).
- `winterbaume-appconfigdata` v0.3.1: see [`crates/winterbaume-appconfigdata/CHANGELOG.md`](crates/winterbaume-appconfigdata/CHANGELOG.md).
- `winterbaume-lambda` v2.0.0: see [`crates/winterbaume-lambda/CHANGELOG.md`](crates/winterbaume-lambda/CHANGELOG.md).
- `winterbaume-opensearchserverless` v0.2.1: see [`crates/winterbaume-opensearchserverless/CHANGELOG.md`](crates/winterbaume-opensearchserverless/CHANGELOG.md).
- `winterbaume-s3` v1.0.0: see [`crates/winterbaume-s3/CHANGELOG.md`](crates/winterbaume-s3/CHANGELOG.md).
- `winterbaume-server` v0.2.5: see [`crates/winterbaume-server/CHANGELOG.md`](crates/winterbaume-server/CHANGELOG.md).
- `winterbaume-terraform` v0.2.3: see [`crates/winterbaume-terraform/CHANGELOG.md`](crates/winterbaume-terraform/CHANGELOG.md).

## v0.7.0 - 2026-07-03

### Internal
- Maintenance release incorporating upstream fixes from `winterbaume-s3` (binary body handling for `UploadPart`) and `winterbaume-opensearchserverless` (policy version collision avoidance).

## 2026-06-04

- `winterbaume` v0.6.0: see [`crates/winterbaume/CHANGELOG.md`](crates/winterbaume/CHANGELOG.md).
- `winterbaume-cloudcontrol` v1.0.1: see [`crates/winterbaume-cloudcontrol/CHANGELOG.md`](crates/winterbaume-cloudcontrol/CHANGELOG.md).
- `winterbaume-server` v0.2.4: see [`crates/winterbaume-server/CHANGELOG.md`](crates/winterbaume-server/CHANGELOG.md).

## v0.6.0 - 2026-06-04

### Internal

- Maintenance release; no user-visible changes to the umbrella crate in this cycle.

## 2026-06-04

- `winterbaume` v0.5.0: see [`crates/winterbaume/CHANGELOG.md`](crates/winterbaume/CHANGELOG.md).
- `winterbaume-cloudcontrol` v0.3.0: see [`crates/winterbaume-cloudcontrol/CHANGELOG.md`](crates/winterbaume-cloudcontrol/CHANGELOG.md).
- `winterbaume-server` v0.2.3: see [`crates/winterbaume-server/CHANGELOG.md`](crates/winterbaume-server/CHANGELOG.md).

## v0.5.0 - 2026-06-04

### Internal

- Maintenance release — no user-visible changes to the root `winterbaume` crate; updates in this window belong to constituent service crates (`winterbaume-cloudcontrol`) and build tooling (`sccache-wrapper`).

## 2026-05-25

- `winterbaume` v0.4.0: see [`crates/winterbaume/CHANGELOG.md`](crates/winterbaume/CHANGELOG.md).
- `winterbaume-cloudfront` v0.3.0: see [`crates/winterbaume-cloudfront/CHANGELOG.md`](crates/winterbaume-cloudfront/CHANGELOG.md).
- `winterbaume-cloudfrontkeyvaluestore` v0.2.1: see [`crates/winterbaume-cloudfrontkeyvaluestore/CHANGELOG.md`](crates/winterbaume-cloudfrontkeyvaluestore/CHANGELOG.md).
- `winterbaume-kms` v0.2.1: see [`crates/winterbaume-kms/CHANGELOG.md`](crates/winterbaume-kms/CHANGELOG.md).
- `winterbaume-lambda` v0.3.0: see [`crates/winterbaume-lambda/CHANGELOG.md`](crates/winterbaume-lambda/CHANGELOG.md).
- `winterbaume-s3` v0.2.1: see [`crates/winterbaume-s3/CHANGELOG.md`](crates/winterbaume-s3/CHANGELOG.md).
- `winterbaume-server` v0.2.2: see [`crates/winterbaume-server/CHANGELOG.md`](crates/winterbaume-server/CHANGELOG.md).
- `winterbaume-terraform` v0.2.2: see [`crates/winterbaume-terraform/CHANGELOG.md`](crates/winterbaume-terraform/CHANGELOG.md).

## v0.4.0 - 2026-05-25

### Fixed

- **Lambda**: `Qualifier` is now honoured on `Invoke` and `InvokeWithResponseStream`; `RevisionId` is enforced on `UpdateFunctionCode`, `UpdateFunctionConfiguration`, `PublishVersion`, layer-version policy mutations, `AddPermission`, `RemovePermission`, and `GetPolicy`.
- **KMS**: Decryption operations now enforce the specified key ID against the key that actually performed the decryption.
- **CloudFront**: `If-Match` precondition is now enforced on mutating distribution and configuration APIs.
- **S3**: Conditional-request evaluation corrected; `HEAD` responses no longer include a message body.

### Internal

- Release harness: umbrella-crate changelog generation now scopes the `git log` path filter to `.` rather than `/`, fixing spurious commit inclusion in changelog drafts.

This file summarises notable winterbaume releases across the workspace and serves
as the changelog for the umbrella `winterbaume` crate. Detailed per-crate notes
live in each crate's own `CHANGELOG.md` under `crates/<crate>/`.

Each crate is versioned independently via `cargo-release`. Release tags use the
`<crate-name>-v<version>` convention (e.g. `winterbaume-s3-v0.2.0`); the umbrella
`winterbaume` crate uses `winterbaume-v<version>`.

## 2026-05-19 — `winterbaume` v0.3.0

Umbrella dependency refresh. Bumps the optional workspace-dep version
specifiers for every service crate that has been tagged since
`winterbaume-v0.2.0` (2026-05-13), so that downstream consumers enabling
feature flags pick up the new published service crates instead of the
versions captured at the previous umbrella release.

The umbrella is bumping `minor` (0.2.0 → 0.3.0) — pre-1.0 semver — because
several of the inherited service crates released breaking 0.x → 1.0 jumps:

- `winterbaume-account` v1.0.0
- `winterbaume-appconfig` v1.0.0
- `winterbaume-applicationsignals` v1.0.0
- `winterbaume-batch` v1.0.0
- `winterbaume-ec2` v1.0.0
- `winterbaume-ivs` v1.0.0
- `winterbaume-opensearch` v1.0.0
- `winterbaume-organizations` v1.0.0

Other refreshed service crates (non-breaking moves):

- `winterbaume-appconfigdata` v0.3.0
- `winterbaume-kinesisvideo` v0.3.0
- `winterbaume-kinesisvideoarchivedmedia` v0.3.0
- `winterbaume-sagemaker` v0.3.0
- `winterbaume-sesv2` v0.3.0

No new umbrella-level features or APIs; consult each per-crate
`CHANGELOG.md` for the substantive changes.

## 2026-05-18

- `winterbaume-account` v0.3.0: see [`crates/winterbaume-account/CHANGELOG.md`](crates/winterbaume-account/CHANGELOG.md).
- `winterbaume-appconfig` v0.3.0: see [`crates/winterbaume-appconfig/CHANGELOG.md`](crates/winterbaume-appconfig/CHANGELOG.md).
- `winterbaume-appconfigdata` v0.3.0: see [`crates/winterbaume-appconfigdata/CHANGELOG.md`](crates/winterbaume-appconfigdata/CHANGELOG.md).
- `winterbaume-applicationsignals` v0.3.0: see [`crates/winterbaume-applicationsignals/CHANGELOG.md`](crates/winterbaume-applicationsignals/CHANGELOG.md).
- `winterbaume-batch` v0.3.0: see [`crates/winterbaume-batch/CHANGELOG.md`](crates/winterbaume-batch/CHANGELOG.md).
- `winterbaume-ec2` v0.3.0: see [`crates/winterbaume-ec2/CHANGELOG.md`](crates/winterbaume-ec2/CHANGELOG.md).
- `winterbaume-ivs` v0.3.0: see [`crates/winterbaume-ivs/CHANGELOG.md`](crates/winterbaume-ivs/CHANGELOG.md).
- `winterbaume-kinesis` v0.2.1: see [`crates/winterbaume-kinesis/CHANGELOG.md`](crates/winterbaume-kinesis/CHANGELOG.md).
- `winterbaume-kinesisvideo` v0.3.0: see [`crates/winterbaume-kinesisvideo/CHANGELOG.md`](crates/winterbaume-kinesisvideo/CHANGELOG.md).
- `winterbaume-kinesisvideoarchivedmedia` v0.3.0: see [`crates/winterbaume-kinesisvideoarchivedmedia/CHANGELOG.md`](crates/winterbaume-kinesisvideoarchivedmedia/CHANGELOG.md).
- `winterbaume-opensearch` v0.3.0: see [`crates/winterbaume-opensearch/CHANGELOG.md`](crates/winterbaume-opensearch/CHANGELOG.md).
- `winterbaume-organizations` v0.3.0: see [`crates/winterbaume-organizations/CHANGELOG.md`](crates/winterbaume-organizations/CHANGELOG.md).
- `winterbaume-sagemaker` v0.3.0: see [`crates/winterbaume-sagemaker/CHANGELOG.md`](crates/winterbaume-sagemaker/CHANGELOG.md).
- `winterbaume-sagemakerruntime` v0.3.0: see [`crates/winterbaume-sagemakerruntime/CHANGELOG.md`](crates/winterbaume-sagemakerruntime/CHANGELOG.md).
- `winterbaume-ses` v0.3.0: see [`crates/winterbaume-ses/CHANGELOG.md`](crates/winterbaume-ses/CHANGELOG.md).
- `winterbaume-sesv2` v0.3.0: see [`crates/winterbaume-sesv2/CHANGELOG.md`](crates/winterbaume-sesv2/CHANGELOG.md).
- `winterbaume-sqs-redis` v0.2.1: see [`crates/winterbaume-sqs-redis/CHANGELOG.md`](crates/winterbaume-sqs-redis/CHANGELOG.md).
- `winterbaume-terraform` v0.2.1: see [`crates/winterbaume-terraform/CHANGELOG.md`](crates/winterbaume-terraform/CHANGELOG.md).

## Unreleased

- No changes since the v0.2.0 batch.

## 2026-05-13 — 2026-05-14: workspace v0.2.0 batch

The full workspace was rolled forward to v0.2.0 in a single coordinated batch
driven by `tools/release-batch`. 241 crates were tagged (125 on 2026-05-13 and
116 on 2026-05-14, paced by the crates.io `publish_new` rate limit). The
`winterbaume` umbrella crate itself was tagged on 2026-05-13.

Substantive crate changes:

- `winterbaume-terraform` v0.2.0: Adopted spec-driven serde codegen for all 145
  Terraform service serialisers, extracted generated Terraform-state projection
  types into the new `winterbaume-tfstate-resource-models` crate, and expanded
  Terraform resource converter coverage by roughly 700 new converters across
  EC2, S3, Route 53, IAM, API Gateway, Glue, RDS, Redshift, SageMaker,
  MediaLive, Network Firewall, S3 Tables and many other services. IAM Terraform
  coverage reached 32/34 (94%). See
  [`crates/winterbaume-terraform/CHANGELOG.md`](crates/winterbaume-terraform/CHANGELOG.md).
- `winterbaume-server` v0.2.0: Picked up the new Terraform resource converters
  wired through the server. See
  [`crates/winterbaume-server/CHANGELOG.md`](crates/winterbaume-server/CHANGELOG.md).
- `winterbaume-tfstate-resource-models` v0.2.0: Initial public release of the
  new crate that hosts the generated Terraform-state projection models extracted
  from `winterbaume-terraform`. Versioned at v0.2.0 to align with the rest of
  the workspace batch; no prior v0.1.0 was published. See
  [`crates/winterbaume-tfstate-resource-models/CHANGELOG.md`](crates/winterbaume-tfstate-resource-models/CHANGELOG.md).

The remaining 237 service crates (e.g. `winterbaume-s3`, `winterbaume-sqs`,
`winterbaume-ec2`, `winterbaume-iam`, `winterbaume-dynamodb`, ...) were
republished without user-facing behaviour changes to pick up refreshed
crates.io keyword metadata and updated API coverage data. Each crate's own
`CHANGELOG.md` records this under an `Internal` section.

## 2026-05-09 — 2026-05-11: workspace v0.1.0 initial public launch

The first public release of winterbaume to crates.io: 240 crates tagged across
three days (53 on 2026-05-09, 186 on 2026-05-10, and 1 on 2026-05-11), paced
by the crates.io `publish_new` rate limit and driven by `tools/release-batch`
with automatic 429 retry handling. The `winterbaume` umbrella crate itself was
tagged on 2026-05-10.

Highlights:

- Published the umbrella `winterbaume` crate and 239 service / utility crates
  to crates.io, including all supported AWS service implementations
  (`winterbaume-s3`, `winterbaume-sqs`, `winterbaume-ec2`, `winterbaume-iam`,
  `winterbaume-dynamodb`, `winterbaume-sns`, `winterbaume-sts`, ...), the
  standalone `winterbaume-server` binary, the `winterbaume-terraform` state
  injection / extraction crate, and supporting utilities such as
  `winterbaume-sqs-redis`, `winterbaume-dynamodb-redis`,
  `winterbaume-sqlengine-duckdb` and `winterbaume-wafv2-wcu-calculator`.
- Established the full public release workflow: `cargo-release` for
  per-crate publishing, `cargo-dist` for binary releases of
  `winterbaume-server`, generated per-crate changelogs, public security
  reporting via `SECURITY.md`, and contribution policy via `CONTRIBUTING.md`.
- Shipped the `tools/release-batch` driver that chunks `cargo-release`
  invocations to fit the crates.io `publish_new` rate limit and retries
  automatically on HTTP 429 responses.

Per-crate first-release notes live in each crate's `CHANGELOG.md`.
