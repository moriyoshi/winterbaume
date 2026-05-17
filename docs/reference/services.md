# Service Coverage

Winterbaume implements **7210 of 11367 operations across 224 AWS services (63.4%)** with real, state-backed logic, compared to moto's 29.1% across the same service set. A further set of operations are routed but return an empty/default response without real behaviour ( see the `Stubs` column ).

| Service | Crate | Protocol | Operations | Stubs | moto | floci | kumo |
|---|---|---|---|---|---|---|---|
| [Account](/services/account) | `winterbaume-account` | restJson1 | 14/15 (93.3%) | 1/15 (6.7%) | 3/15 (20.0%) | 0/15 (0.0%) | 0/15 (0.0%) |
| [ACM](/services/acm) | `winterbaume-acm` | awsJson1.1 | 16/17 (94.1%) | 0/17 (0.0%) | 11/17 (64.7%) | 0/17 (0.0%) | 6/17 (35.3%) |
| [ACM PCA](/services/acmpca) | `winterbaume-acmpca` | awsJson1.1 | 23/23 (100.0%) | 0/23 (0.0%) | 17/23 (73.9%) | 0/23 (0.0%) | 0/23 (0.0%) |
| [AIOps](/services/aiops) | `winterbaume-aiops` | restJson1 | 11/11 (100.0%) | 0/11 (0.0%) | 0/11 (0.0%) | 0/11 (0.0%) | 0/11 (0.0%) |
| [AMP/Prometheus](/services/amp) | `winterbaume-amp` | restJson1 | 17/44 (38.6%) | 0/44 (0.0%) | 17/44 (38.6%) | 0/44 (0.0%) | 0/44 (0.0%) |
| [Amplify](/services/amplify) | `winterbaume-amplify` | restJson1 | 23/37 (62.2%) | 0/37 (0.0%) | 0/37 (0.0%) | 0/37 (0.0%) | 9/37 (24.3%) |
| [Amplify Backend](/services/amplifybackend) | `winterbaume-amplifybackend` | restJson1 | 4/31 (12.9%) | 0/31 (0.0%) | 0/31 (0.0%) | 0/31 (0.0%) | 0/31 (0.0%) |
| [Amplify UI Builder](/services/amplifyuibuilder) | `winterbaume-amplifyuibuilder` | restJson1 | 28/28 (100.0%) | 0/28 (0.0%) | 0/28 (0.0%) | 0/28 (0.0%) | 0/28 (0.0%) |
| [API Gateway](/services/apigateway) | `winterbaume-apigateway` | restJson1 | 117/124 (94.4%) | 2/124 (1.6%) | 78/124 (62.9%) | 70/124 (56.5%) | 17/124 (13.7%) |
| [API Gateway Management API](/services/apigatewaymanagement) | `winterbaume-apigatewaymanagement` | restJson1 | 3/3 (100.0%) | 0/3 (0.0%) | 3/3 (100.0%) | 0/3 (0.0%) | 0/3 (0.0%) |
| [API Gateway V2](/services/apigatewayv2) | `winterbaume-apigatewayv2` | restJson1 | 62/103 (60.2%) | 0/103 (0.0%) | 54/103 (52.4%) | 0/103 (0.0%) | 0/103 (0.0%) |
| [App Mesh](/services/appmesh) | `winterbaume-appmesh` | restJson1 | 38/38 (100.0%) | 0/38 (0.0%) | 0/38 (0.0%) | 0/38 (0.0%) | 25/38 (65.8%) |
| [App Runner](/services/apprunner) | `winterbaume-apprunner` | awsJson1.0 | 23/37 (62.2%) | 0/37 (0.0%) | 0/37 (0.0%) | 0/37 (0.0%) | 0/37 (0.0%) |
| [AppConfig](/services/appconfig) | `winterbaume-appconfig` | restJson1 | 45/45 (100.0%) | 0/45 (0.0%) | 15/45 (33.3%) | 0/45 (0.0%) | 0/45 (0.0%) |
| [AppConfig Data](/services/appconfigdata) | `winterbaume-appconfigdata` | restJson1 | 2/2 (100.0%) | 0/2 (0.0%) | 0/2 (0.0%) | 0/2 (0.0%) | 0/2 (0.0%) |
| [AppFabric](/services/appfabric) | `winterbaume-appfabric` | restJson1 | 6/26 (23.1%) | 0/26 (0.0%) | 0/26 (0.0%) | 0/26 (0.0%) | 0/26 (0.0%) |
| [AppFlow](/services/appflow) | `winterbaume-appflow` | restJson1 | 9/25 (36.0%) | 0/25 (0.0%) | 0/25 (0.0%) | 0/25 (0.0%) | 0/25 (0.0%) |
| [AppIntegrations](/services/appintegrations) | `winterbaume-appintegrations` | restJson1 | 23/23 (100.0%) | 0/23 (0.0%) | 0/23 (0.0%) | 0/23 (0.0%) | 0/23 (0.0%) |
| [Application Auto Scaling](/services/applicationautoscaling) | `winterbaume-applicationautoscaling` | awsJson1.1 | 13/14 (92.9%) | 1/14 (7.1%) | 9/14 (64.3%) | 0/14 (0.0%) | 0/14 (0.0%) |
| [Application Cost Profiler](/services/applicationcostprofiler) | `winterbaume-applicationcostprofiler` | restJson1 | 6/6 (100.0%) | 0/6 (0.0%) | 0/6 (0.0%) | 0/6 (0.0%) | 0/6 (0.0%) |
| [Application Discovery Service](/services/applicationdiscoveryservice) | `winterbaume-applicationdiscoveryservice` | awsJson1.1 | 28/28 (100.0%) | 0/28 (0.0%) | 0/28 (0.0%) | 0/28 (0.0%) | 0/28 (0.0%) |
| [Application Insights](/services/applicationinsights) | `winterbaume-applicationinsights` | awsJson1.1 | 33/33 (100.0%) | 0/33 (0.0%) | 0/33 (0.0%) | 0/33 (0.0%) | 0/33 (0.0%) |
| [Application Signals](/services/applicationsignals) | `winterbaume-applicationsignals` | restJson1 | 10/23 (43.5%) | 3/23 (13.0%) | 0/23 (0.0%) | 0/23 (0.0%) | 0/23 (0.0%) |
| [AppSync](/services/appsync) | `winterbaume-appsync` | restJson1 | 27/74 (36.5%) | 0/74 (0.0%) | 27/74 (36.5%) | 0/74 (0.0%) | 3/74 (4.1%) |
| [ARC Zonal Shift](/services/arczonalshift) | `winterbaume-arczonalshift` | restJson1 | 14/15 (93.3%) | 1/15 (6.7%) | 0/15 (0.0%) | 0/15 (0.0%) | 0/15 (0.0%) |
| [Artifact](/services/artifact) | `winterbaume-artifact` | restJson1 | 8/8 (100.0%) | 0/8 (0.0%) | 0/8 (0.0%) | 0/8 (0.0%) | 0/8 (0.0%) |
| [Athena](/services/athena) | `winterbaume-athena` | awsJson1.1 | 25/70 (35.7%) | 0/70 (0.0%) | 27/70 (38.6%) | 0/70 (0.0%) | 7/70 (10.0%) |
| [Audit Manager](/services/auditmanager) | `winterbaume-auditmanager` | restJson1 | 15/62 (24.2%) | 0/62 (0.0%) | 0/62 (0.0%) | 0/62 (0.0%) | 0/62 (0.0%) |
| [Aurora DSQL](/services/dsql) | `winterbaume-dsql` | restJson1 | 12/12 (100.0%) | 0/12 (0.0%) | 5/12 (41.7%) | 0/12 (0.0%) | 0/12 (0.0%) |
| [Auto Scaling](/services/autoscaling) | `winterbaume-autoscaling` | awsQuery | 52/66 (78.8%) | 0/66 (0.0%) | 39/66 (59.1%) | 0/66 (0.0%) | 0/66 (0.0%) |
| [Auto Scaling Plans](/services/autoscalingplans) | `winterbaume-autoscalingplans` | awsJson1.1 | 6/6 (100.0%) | 0/6 (0.0%) | 0/6 (0.0%) | 0/6 (0.0%) | 0/6 (0.0%) |
| [Backup](/services/backup) | `winterbaume-backup` | restJson1 | 105/108 (97.2%) | 3/108 (2.8%) | 17/108 (15.7%) | 0/108 (0.0%) | 12/108 (11.1%) |
| [Backup Gateway](/services/backupgateway) | `winterbaume-backupgateway` | awsJson1.0 | 25/25 (100.0%) | 0/25 (0.0%) | 0/25 (0.0%) | 0/25 (0.0%) | 0/25 (0.0%) |
| [Backup Search](/services/backupsearch) | `winterbaume-backupsearch` | restJson1 | 9/12 (75.0%) | 0/12 (0.0%) | 0/12 (0.0%) | 0/12 (0.0%) | 0/12 (0.0%) |
| [Batch](/services/batch) | `winterbaume-batch` | restJson1 | 39/45 (86.7%) | 0/45 (0.0%) | 24/45 (53.3%) | 0/45 (0.0%) | 10/45 (22.2%) |
| [BCM Dashboards](/services/bcmdashboards) | `winterbaume-bcmdashboards` | awsJson1.0 | 9/15 (60.0%) | 0/15 (0.0%) | 0/15 (0.0%) | 0/15 (0.0%) | 0/15 (0.0%) |
| [BCM Data Exports](/services/bcmdataexports) | `winterbaume-bcmdataexports` | awsJson1.1 | 12/12 (100.0%) | 0/12 (0.0%) | 0/12 (0.0%) | 0/12 (0.0%) | 0/12 (0.0%) |
| [BCM Recommended Actions](/services/bcmrecommendedactions) | `winterbaume-bcmrecommendedactions` | awsJson1.0 | 1/1 (100.0%) | 0/1 (0.0%) | 0/1 (0.0%) | 0/1 (0.0%) | 0/1 (0.0%) |
| [Bedrock](/services/bedrock) | `winterbaume-bedrock` | restJson1 | 48/101 (47.5%) | 0/101 (0.0%) | 13/101 (12.9%) | 0/101 (0.0%) | 0/101 (0.0%) |
| [Bedrock Agent](/services/bedrockagent) | `winterbaume-bedrockagent` | restJson1 | 72/72 (100.0%) | 0/72 (0.0%) | 11/72 (15.3%) | 0/72 (0.0%) | 0/72 (0.0%) |
| [Billing](/services/billing) | `winterbaume-billing` | awsJson1.0 | 12/12 (100.0%) | 0/12 (0.0%) | 0/12 (0.0%) | 0/12 (0.0%) | 0/12 (0.0%) |
| [Braket](/services/braket) | `winterbaume-braket` | restJson1 | 12/17 (70.6%) | 0/17 (0.0%) | 0/17 (0.0%) | 0/17 (0.0%) | 0/17 (0.0%) |
| [Budgets](/services/budgets) | `winterbaume-budgets` | awsJson1.1 | 7/26 (26.9%) | 0/26 (0.0%) | 7/26 (26.9%) | 0/26 (0.0%) | 0/26 (0.0%) |
| [Chatbot](/services/chatbot) | `winterbaume-chatbot` | restJson1 | 15/34 (44.1%) | 0/34 (0.0%) | 0/34 (0.0%) | 0/34 (0.0%) | 0/34 (0.0%) |
| [Chime SDK Meetings](/services/chimesdkmeetings) | `winterbaume-chimesdkmeetings` | restJson1 | 12/16 (75.0%) | 0/16 (0.0%) | 0/16 (0.0%) | 0/16 (0.0%) | 0/16 (0.0%) |
| [Cloud Control API](/services/cloudcontrol) | `winterbaume-cloudcontrol` | awsJson1.0 | 8/8 (100.0%) | 0/8 (0.0%) | 0/8 (0.0%) | 0/8 (0.0%) | 0/8 (0.0%) |
| [Cloud Directory](/services/clouddirectory) | `winterbaume-clouddirectory` | restJson1 | 13/66 (19.7%) | 0/66 (0.0%) | 13/66 (19.7%) | 0/66 (0.0%) | 0/66 (0.0%) |
| [Cloud9](/services/cloud9) | `winterbaume-cloud9` | awsJson1.1 | 13/13 (100.0%) | 0/13 (0.0%) | 0/13 (0.0%) | 0/13 (0.0%) | 0/13 (0.0%) |
| [CloudFormation](/services/cloudformation) | `winterbaume-cloudformation` | awsQuery | 40/90 (44.4%) | 3/90 (3.3%) | 33/90 (36.7%) | 0/90 (0.0%) | 8/90 (8.9%) |
| [CloudFront](/services/cloudfront) | `winterbaume-cloudfront` | restXml | 156/167 (93.4%) | 11/167 (6.6%) | 25/167 (15.0%) | 0/167 (0.0%) | 8/167 (4.8%) |
| [CloudFront KeyValueStore](/services/cloudfrontkeyvaluestore) | `winterbaume-cloudfrontkeyvaluestore` | restJson1 | 5/6 (83.3%) | 0/6 (0.0%) | 0/6 (0.0%) | 0/6 (0.0%) | 0/6 (0.0%) |
| [CloudHSM v2](/services/cloudhsmv2) | `winterbaume-cloudhsmv2` | awsJson1.1 | 18/18 (100.0%) | 0/18 (0.0%) | 0/18 (0.0%) | 0/18 (0.0%) | 0/18 (0.0%) |
| [CloudSearch Domain](/services/cloudsearchdomain) | `winterbaume-cloudsearchdomain` | restJson1 | 2/3 (66.7%) | 0/3 (0.0%) | 0/3 (0.0%) | 0/3 (0.0%) | 0/3 (0.0%) |
| [CloudTrail](/services/cloudtrail) | `winterbaume-cloudtrail` | awsJson1.1 | 21/60 (35.0%) | 2/60 (3.3%) | 16/60 (26.7%) | 0/60 (0.0%) | 8/60 (13.3%) |
| [CloudTrail Data](/services/cloudtraildata) | `winterbaume-cloudtraildata` | restJson1 | 1/1 (100.0%) | 0/1 (0.0%) | 0/1 (0.0%) | 0/1 (0.0%) | 0/1 (0.0%) |
| [CloudWatch](/services/cloudwatch) | `winterbaume-cloudwatch` | awsJson1.0 | 38/46 (82.6%) | 5/46 (10.9%) | 20/46 (43.5%) | 0/46 (0.0%) | 10/46 (21.7%) |
| [CloudWatch Logs](/services/cloudwatchlogs) | `winterbaume-cloudwatchlogs` | awsJson1.1 | 93/113 (82.3%) | 15/113 (13.3%) | 51/113 (45.1%) | 0/113 (0.0%) | 11/113 (9.7%) |
| [CodeArtifact](/services/codeartifact) | `winterbaume-codeartifact` | restJson1 | 9/48 (18.8%) | 0/48 (0.0%) | 0/48 (0.0%) | 0/48 (0.0%) | 0/48 (0.0%) |
| [CodeBuild](/services/codebuild) | `winterbaume-codebuild` | awsJson1.1 | 29/59 (49.2%) | 0/59 (0.0%) | 9/59 (15.3%) | 0/59 (0.0%) | 0/59 (0.0%) |
| [CodeCommit](/services/codecommit) | `winterbaume-codecommit` | awsJson1.1 | 25/79 (31.6%) | 0/79 (0.0%) | 3/79 (3.8%) | 0/79 (0.0%) | 0/79 (0.0%) |
| [CodeDeploy](/services/codedeploy) | `winterbaume-codedeploy` | awsJson1.1 | 15/47 (31.9%) | 0/47 (0.0%) | 14/47 (29.8%) | 0/47 (0.0%) | 0/47 (0.0%) |
| [CodeGuru Reviewer](/services/codegurureviewer) | `winterbaume-codegurureviewer` | restJson1 | 9/14 (64.3%) | 0/14 (0.0%) | 0/14 (0.0%) | 0/14 (0.0%) | 11/14 (78.6%) |
| [CodeGuru Security](/services/codegurusecurity) | `winterbaume-codegurusecurity` | restJson1 | 11/13 (84.6%) | 0/13 (0.0%) | 0/13 (0.0%) | 0/13 (0.0%) | 0/13 (0.0%) |
| [CodePipeline](/services/codepipeline) | `winterbaume-codepipeline` | awsJson1.1 | 44/44 (100.0%) | 0/44 (0.0%) | 8/44 (18.2%) | 0/44 (0.0%) | 0/44 (0.0%) |
| [CodeStar Notifications](/services/codestarnotifications) | `winterbaume-codestarnotifications` | restJson1 | 7/13 (53.8%) | 0/13 (0.0%) | 0/13 (0.0%) | 0/13 (0.0%) | 0/13 (0.0%) |
| [Cognito Identity](/services/cognitoidentity) | `winterbaume-cognitoidentity` | awsJson1.1 | 20/23 (87.0%) | 3/23 (13.0%) | 10/23 (43.5%) | 0/23 (0.0%) | 0/23 (0.0%) |
| [Cognito Identity Provider](/services/cognitoidentityprovider) | `winterbaume-cognitoidentityprovider` | awsJson1.1 | 104/122 (85.2%) | 18/122 (14.8%) | 62/122 (50.8%) | 39/122 (32.0%) | 17/122 (13.9%) |
| [Cognito Sync](/services/cognitosync) | `winterbaume-cognitosync` | restJson1 | 11/17 (64.7%) | 0/17 (0.0%) | 0/17 (0.0%) | 0/17 (0.0%) | 0/17 (0.0%) |
| [Comprehend](/services/comprehend) | `winterbaume-comprehend` | awsJson1.1 | 60/85 (70.6%) | 5/85 (5.9%) | 63/85 (74.1%) | 0/85 (0.0%) | 12/85 (14.1%) |
| [Config](/services/config) | `winterbaume-config` | awsJson1.1 | 46/97 (47.4%) | 3/97 (3.1%) | 38/97 (39.2%) | 0/97 (0.0%) | 9/97 (9.3%) |
| [Connect](/services/connect) | `winterbaume-connect` | restJson1 | 10/370 (2.7%) | 0/370 (0.0%) | 10/370 (2.7%) | 0/370 (0.0%) | 0/370 (0.0%) |
| [Connect Campaigns](/services/connectcampaigns) | `winterbaume-connectcampaigns` | restJson1 | 14/22 (63.6%) | 0/22 (0.0%) | 14/22 (63.6%) | 0/22 (0.0%) | 0/22 (0.0%) |
| [Connect Contact Lens](/services/connectcontactlens) | `winterbaume-connectcontactlens` | restJson1 | 0/1 (0.0%) | 0/1 (0.0%) | 0/1 (0.0%) | 0/1 (0.0%) | 0/1 (0.0%) |
| [Connect Participant](/services/connectparticipant) | `winterbaume-connectparticipant` | restJson1 | 7/11 (63.6%) | 0/11 (0.0%) | 0/11 (0.0%) | 0/11 (0.0%) | 0/11 (0.0%) |
| [Control Catalog](/services/controlcatalog) | `winterbaume-controlcatalog` | restJson1 | 6/6 (100.0%) | 0/6 (0.0%) | 0/6 (0.0%) | 0/6 (0.0%) | 0/6 (0.0%) |
| [Cost and Usage Report Service](/services/costandusagereport) | `winterbaume-costandusagereport` | awsJson1.1 | 7/7 (100.0%) | 0/7 (0.0%) | 0/7 (0.0%) | 0/7 (0.0%) | 0/7 (0.0%) |
| [Cost Explorer](/services/costexplorer) | `winterbaume-costexplorer` | awsJson1.1 | 22/47 (46.8%) | 25/47 (53.2%) | 0/47 (0.0%) | 0/47 (0.0%) | 8/47 (17.0%) |
| [Cost Optimization Hub](/services/costoptimizationhub) | `winterbaume-costoptimizationhub` | awsJson1.0 | 8/8 (100.0%) | 0/8 (0.0%) | 0/8 (0.0%) | 0/8 (0.0%) | 0/8 (0.0%) |
| [Data Pipeline](/services/datapipeline) | `winterbaume-datapipeline` | awsJson1.1 | 19/19 (100.0%) | 0/19 (0.0%) | 0/19 (0.0%) | 0/19 (0.0%) | 0/19 (0.0%) |
| [Database Migration Service (DMS)](/services/databasemigration) | `winterbaume-databasemigration` | awsJson1.1 | 42/119 (35.3%) | 0/119 (0.0%) | 17/119 (14.3%) | 0/119 (0.0%) | 0/119 (0.0%) |
| [DataBrew](/services/databrew) | `winterbaume-databrew` | restJson1 | 32/44 (72.7%) | 1/44 (2.3%) | 24/44 (54.5%) | 0/44 (0.0%) | 0/44 (0.0%) |
| [DataSync](/services/datasync) | `winterbaume-datasync` | awsJson1.1 | 8/53 (15.1%) | 0/53 (0.0%) | 6/53 (11.3%) | 0/53 (0.0%) | 0/53 (0.0%) |
| [DAX](/services/dax) | `winterbaume-dax` | awsJson1.1 | 6/21 (28.6%) | 0/21 (0.0%) | 8/21 (38.1%) | 0/21 (0.0%) | 0/21 (0.0%) |
| [Direct Connect](/services/directconnect) | `winterbaume-directconnect` | awsJson1.1 | 7/63 (11.1%) | 0/63 (0.0%) | 0/63 (0.0%) | 0/63 (0.0%) | 0/63 (0.0%) |
| [Directory Service](/services/directory) | `winterbaume-directory` | awsJson1.1 | 4/80 (5.0%) | 0/80 (0.0%) | 0/80 (0.0%) | 0/80 (0.0%) | 6/80 (7.5%) |
| [DLM](/services/dlm) | `winterbaume-dlm` | restJson1 | 2/8 (25.0%) | 0/8 (0.0%) | 0/8 (0.0%) | 0/8 (0.0%) | 5/8 (62.5%) |
| [DynamoDB](/services/dynamodb) | `winterbaume-dynamodb` | awsJson1.0 | 57/57 (100.0%) | 0/57 (0.0%) | 39/57 (68.4%) | 0/57 (0.0%) | 20/57 (35.1%) |
| [DynamoDB Streams](/services/dynamodbstreams) | `winterbaume-dynamodbstreams` | awsJson1.0 | 4/4 (100.0%) | 0/4 (0.0%) | 0/4 (0.0%) | 0/4 (0.0%) | 0/4 (0.0%) |
| [EBS](/services/ebs) | `winterbaume-ebs` | restJson1 | 6/6 (100.0%) | 0/6 (0.0%) | 6/6 (100.0%) | 0/6 (0.0%) | 0/6 (0.0%) |
| [EC2](/services/ec2) | `winterbaume-ec2` | ec2Query | 713/763 (93.4%) | 43/763 (5.6%) | 223/763 (29.2%) | 0/763 (0.0%) | 39/763 (5.1%) |
| [EC2 Instance Connect](/services/ec2instanceconnect) | `winterbaume-ec2instanceconnect` | awsJson1.1 | 2/2 (100.0%) | 0/2 (0.0%) | 1/2 (50.0%) | 0/2 (0.0%) | 0/2 (0.0%) |
| [ECR](/services/ecr) | `winterbaume-ecr` | awsJson1.1 | 58/58 (100.0%) | 0/58 (0.0%) | 29/58 (50.0%) | 0/58 (0.0%) | 11/58 (19.0%) |
| [ECS](/services/ecs) | `winterbaume-ecs` | awsJson1.1 | 63/76 (82.9%) | 1/76 (1.3%) | 45/76 (59.2%) | 0/76 (0.0%) | 12/76 (15.8%) |
| [EFS](/services/efs) | `winterbaume-efs` | restJson1 | 31/31 (100.0%) | 0/31 (0.0%) | 19/31 (61.3%) | 0/31 (0.0%) | 0/31 (0.0%) |
| [EKS](/services/eks) | `winterbaume-eks` | restJson1 | 55/64 (85.9%) | 4/64 (6.2%) | 17/64 (26.6%) | 0/64 (0.0%) | 8/64 (12.5%) |
| [Elastic Beanstalk](/services/elasticbeanstalk) | `winterbaume-elasticbeanstalk` | awsQuery | 7/47 (14.9%) | 0/47 (0.0%) | 0/47 (0.0%) | 0/47 (0.0%) | 7/47 (14.9%) |
| [ElastiCache](/services/elasticache) | `winterbaume-elasticache` | awsQuery | 24/75 (32.0%) | 0/75 (0.0%) | 17/75 (22.7%) | 0/75 (0.0%) | 7/75 (9.3%) |
| [ELB](/services/elasticloadbalancing) | `winterbaume-elasticloadbalancing` | awsQuery | 29/29 (100.0%) | 0/29 (0.0%) | 21/29 (72.4%) | 0/29 (0.0%) | 0/29 (0.0%) |
| [ELBv2](/services/elasticloadbalancingv2) | `winterbaume-elasticloadbalancingv2` | awsQuery | 50/51 (98.0%) | 1/51 (2.0%) | 33/51 (64.7%) | 0/51 (0.0%) | 22/51 (43.1%) |
| [EMR](/services/emr) | `winterbaume-emr` | awsJson1.1 | 54/60 (90.0%) | 2/60 (3.3%) | 26/60 (43.3%) | 0/60 (0.0%) | 0/60 (0.0%) |
| [EMR Containers](/services/emrcontainers) | `winterbaume-emrcontainers` | restJson1 | 23/23 (100.0%) | 0/23 (0.0%) | 8/23 (34.8%) | 0/23 (0.0%) | 0/23 (0.0%) |
| [EMR Serverless](/services/emrserverless) | `winterbaume-emrserverless` | restJson1 | 16/22 (72.7%) | 0/22 (0.0%) | 11/22 (50.0%) | 0/22 (0.0%) | 11/22 (50.0%) |
| [EventBridge](/services/eventbridge) | `winterbaume-eventbridge` | awsJson1.1 | 57/57 (100.0%) | 0/57 (0.0%) | 45/57 (78.9%) | 0/57 (0.0%) | 15/57 (26.3%) |
| [EventBridge Pipes](/services/pipes) | `winterbaume-pipes` | restJson1 | 10/10 (100.0%) | 0/10 (0.0%) | 9/10 (90.0%) | 0/10 (0.0%) | 10/10 (100.0%) |
| [Firehose](/services/firehose) | `winterbaume-firehose` | awsJson1.1 | 12/12 (100.0%) | 0/12 (0.0%) | 12/12 (100.0%) | 0/12 (0.0%) | 7/12 (58.3%) |
| [FIS](/services/fis) | `winterbaume-fis` | restJson1 | 8/26 (30.8%) | 0/26 (0.0%) | 5/26 (19.2%) | 0/26 (0.0%) | 0/26 (0.0%) |
| [Forecast](/services/forecast) | `winterbaume-forecast` | awsJson1.1 | 5/63 (7.9%) | 0/63 (0.0%) | 5/63 (7.9%) | 0/63 (0.0%) | 17/63 (27.0%) |
| [Free Tier](/services/freetier) | `winterbaume-freetier` | awsJson1.0 | 5/5 (100.0%) | 0/5 (0.0%) | 0/5 (0.0%) | 0/5 (0.0%) | 0/5 (0.0%) |
| [FSx](/services/fsx) | `winterbaume-fsx` | awsJson1.1 | 9/48 (18.8%) | 0/48 (0.0%) | 9/48 (18.8%) | 0/48 (0.0%) | 0/48 (0.0%) |
| [Glacier](/services/glacier) | `winterbaume-glacier` | restJson1 | 33/33 (100.0%) | 0/33 (0.0%) | 10/33 (30.3%) | 0/33 (0.0%) | 4/33 (12.1%) |
| [Glue](/services/glue) | `winterbaume-glue` | awsJson1.1 | 132/265 (49.8%) | 0/265 (0.0%) | 96/265 (36.2%) | 0/265 (0.0%) | 14/265 (5.3%) |
| [Greengrass](/services/greengrass) | `winterbaume-greengrass` | restJson1 | 71/92 (77.2%) | 0/92 (0.0%) | 55/92 (59.8%) | 0/92 (0.0%) | 0/92 (0.0%) |
| [GuardDuty](/services/guardduty) | `winterbaume-guardduty` | restJson1 | 85/87 (97.7%) | 2/87 (2.3%) | 12/87 (13.8%) | 0/87 (0.0%) | 0/87 (0.0%) |
| [IAM](/services/iam) | `winterbaume-iam` | awsQuery | 154/176 (87.5%) | 22/176 (12.5%) | 119/176 (67.6%) | 0/176 (0.0%) | 39/176 (22.2%) |
| [IAM Access Analyzer](/services/accessanalyzer) | `winterbaume-accessanalyzer` | restJson1 | 11/37 (29.7%) | 0/37 (0.0%) | 0/37 (0.0%) | 0/37 (0.0%) | 0/37 (0.0%) |
| [IAM Roles Anywhere](/services/rolesanywhere) | `winterbaume-rolesanywhere` | restJson1 | 28/30 (93.3%) | 2/30 (6.7%) | 0/30 (0.0%) | 0/30 (0.0%) | 0/30 (0.0%) |
| [Identity Store](/services/identitystore) | `winterbaume-identitystore` | awsJson1.1 | 17/19 (89.5%) | 0/19 (0.0%) | 14/19 (73.7%) | 0/19 (0.0%) | 0/19 (0.0%) |
| [Inspector2](/services/inspector2) | `winterbaume-inspector2` | restJson1 | 49/75 (65.3%) | 21/75 (28.0%) | 19/75 (25.3%) | 0/75 (0.0%) | 0/75 (0.0%) |
| [IoT](/services/iot) | `winterbaume-iot` | restJson1 | 103/272 (37.9%) | 0/272 (0.0%) | 100/272 (36.8%) | 0/272 (0.0%) | 0/272 (0.0%) |
| [IoT Data Plane](/services/iotdataplane) | `winterbaume-iotdataplane` | restJson1 | 8/8 (100.0%) | 0/8 (0.0%) | 0/8 (0.0%) | 0/8 (0.0%) | 0/8 (0.0%) |
| [IVS](/services/ivs) | `winterbaume-ivs` | restJson1 | 30/40 (75.0%) | 5/40 (12.5%) | 6/40 (15.0%) | 0/40 (0.0%) | 0/40 (0.0%) |
| [Keyspaces](/services/keyspaces) | `winterbaume-keyspaces` | awsJson1.0 | 19/19 (100.0%) | 0/19 (0.0%) | 0/19 (0.0%) | 0/19 (0.0%) | 0/19 (0.0%) |
| [Kinesis](/services/kinesis) | `winterbaume-kinesis` | awsJson1.1 | 38/39 (97.4%) | 0/39 (0.0%) | 31/39 (79.5%) | 0/39 (0.0%) | 9/39 (23.1%) |
| [Kinesis Analytics V2](/services/kinesisanalyticsv2) | `winterbaume-kinesisanalyticsv2` | awsJson1.1 | 32/33 (97.0%) | 1/33 (3.0%) | 0/33 (0.0%) | 0/33 (0.0%) | 0/33 (0.0%) |
| [Kinesis Video](/services/kinesisvideo) | `winterbaume-kinesisvideo` | restJson1 | 32/32 (100.0%) | 0/32 (0.0%) | 0/32 (0.0%) | 0/32 (0.0%) | 0/32 (0.0%) |
| [Kinesis Video Archived Media](/services/kinesisvideoarchivedmedia) | `winterbaume-kinesisvideoarchivedmedia` | restJson1 | 6/6 (100.0%) | 0/6 (0.0%) | 3/6 (50.0%) | 0/6 (0.0%) | 0/6 (0.0%) |
| [KMS](/services/kms) | `winterbaume-kms` | awsJson1.1 | 53/54 (98.1%) | 0/54 (0.0%) | 40/54 (74.1%) | 0/54 (0.0%) | 19/54 (35.2%) |
| [Lake Formation](/services/lakeformation) | `winterbaume-lakeformation` | restJson1 | 19/61 (31.1%) | 1/61 (1.6%) | 20/61 (32.8%) | 0/61 (0.0%) | 0/61 (0.0%) |
| [Lambda](/services/lambda) | `winterbaume-lambda` | restJson1 | 85/85 (100.0%) | 0/85 (0.0%) | 46/85 (54.1%) | 0/85 (0.0%) | 17/85 (20.0%) |
| [Lex Models V2](/services/lexmodelsv2) | `winterbaume-lexmodelsv2` | restJson1 | 58/107 (54.2%) | 2/107 (1.9%) | 17/107 (15.9%) | 0/107 (0.0%) | 0/107 (0.0%) |
| [Macie2](/services/macie2) | `winterbaume-macie2` | restJson1 | 67/81 (82.7%) | 14/81 (17.3%) | 13/81 (16.0%) | 0/81 (0.0%) | 24/81 (29.6%) |
| [Managed Blockchain](/services/managedblockchain) | `winterbaume-managedblockchain` | restJson1 | 27/27 (100.0%) | 0/27 (0.0%) | 20/27 (74.1%) | 0/27 (0.0%) | 0/27 (0.0%) |
| [Marketplace Metering](/services/marketplacemetering) | `winterbaume-marketplacemetering` | awsJson1.1 | 4/4 (100.0%) | 0/4 (0.0%) | 0/4 (0.0%) | 0/4 (0.0%) | 0/4 (0.0%) |
| [MediaConnect](/services/mediaconnect) | `winterbaume-mediaconnect` | restJson1 | 21/82 (25.6%) | 0/82 (0.0%) | 18/82 (22.0%) | 0/82 (0.0%) | 0/82 (0.0%) |
| [MediaLive](/services/medialive) | `winterbaume-medialive` | restJson1 | 16/123 (13.0%) | 0/123 (0.0%) | 12/123 (9.8%) | 0/123 (0.0%) | 0/123 (0.0%) |
| [MediaPackage](/services/mediapackage) | `winterbaume-mediapackage` | restJson1 | 9/19 (47.4%) | 0/19 (0.0%) | 9/19 (47.4%) | 0/19 (0.0%) | 0/19 (0.0%) |
| [MediaPackage v2](/services/mediapackagev2) | `winterbaume-mediapackagev2` | restJson1 | 7/30 (23.3%) | 0/30 (0.0%) | 7/30 (23.3%) | 0/30 (0.0%) | 0/30 (0.0%) |
| [MediaStore](/services/mediastore) | `winterbaume-mediastore` | awsJson1.1 | 11/21 (52.4%) | 0/21 (0.0%) | 11/21 (52.4%) | 0/21 (0.0%) | 0/21 (0.0%) |
| [MediaStore Data](/services/mediastoredata) | `winterbaume-mediastoredata` | restJson1 | 5/5 (100.0%) | 0/5 (0.0%) | 4/5 (80.0%) | 0/5 (0.0%) | 0/5 (0.0%) |
| [MemoryDB](/services/memorydb) | `winterbaume-memorydb` | awsJson1.1 | 13/45 (28.9%) | 0/45 (0.0%) | 13/45 (28.9%) | 0/45 (0.0%) | 10/45 (22.2%) |
| [MQ](/services/mq) | `winterbaume-mq` | restJson1 | 23/24 (95.8%) | 1/24 (4.2%) | 19/24 (79.2%) | 0/24 (0.0%) | 6/24 (25.0%) |
| [MSK](/services/kafka) | `winterbaume-kafka` | restJson1 | 10/59 (16.9%) | 0/59 (0.0%) | 13/59 (22.0%) | 0/59 (0.0%) | 6/59 (10.2%) |
| [Neptune](/services/neptune) | `winterbaume-neptune` | awsQuery | 64/70 (91.4%) | 6/70 (8.6%) | 47/70 (67.1%) | 0/70 (0.0%) | 6/70 (8.6%) |
| [Network Firewall](/services/networkfirewall) | `winterbaume-networkfirewall` | awsJson1.0 | 79/79 (100.0%) | 0/79 (0.0%) | 5/79 (6.3%) | 0/79 (0.0%) | 0/79 (0.0%) |
| [Network Manager](/services/networkmanager) | `winterbaume-networkmanager` | restJson1 | 53/95 (55.8%) | 0/95 (0.0%) | 18/95 (18.9%) | 0/95 (0.0%) | 0/95 (0.0%) |
| [OpenSearch](/services/opensearch) | `winterbaume-opensearch` | restJson1 | 44/88 (50.0%) | 0/88 (0.0%) | 11/88 (12.5%) | 0/88 (0.0%) | 0/88 (0.0%) |
| [OpenSearch Ingestion](/services/osis) | `winterbaume-osis` | restJson1 | 10/22 (45.5%) | 0/22 (0.0%) | 13/22 (59.1%) | 0/22 (0.0%) | 0/22 (0.0%) |
| [OpenSearch Serverless](/services/opensearchserverless) | `winterbaume-opensearchserverless` | awsJson1.0 | 12/46 (26.1%) | 0/46 (0.0%) | 12/46 (26.1%) | 0/46 (0.0%) | 0/46 (0.0%) |
| [Organizations](/services/organizations) | `winterbaume-organizations` | awsJson1.1 | 60/63 (95.2%) | 3/63 (4.8%) | 41/63 (65.1%) | 0/63 (0.0%) | 11/63 (17.5%) |
| [Outposts](/services/outposts) | `winterbaume-outposts` | restJson1 | 13/37 (35.1%) | 0/37 (0.0%) | 0/37 (0.0%) | 0/37 (0.0%) | 0/37 (0.0%) |
| [Panorama](/services/panorama) | `winterbaume-panorama` | restJson1 | 10/34 (29.4%) | 1/34 (2.9%) | 11/34 (32.4%) | 0/34 (0.0%) | 0/34 (0.0%) |
| [Performance Insights](/services/pi) | `winterbaume-pi` | awsJson1.1 | 9/13 (69.2%) | 4/13 (30.8%) | 0/13 (0.0%) | 0/13 (0.0%) | 0/13 (0.0%) |
| [Personalize](/services/personalize) | `winterbaume-personalize` | awsJson1.1 | 66/71 (93.0%) | 5/71 (7.0%) | 4/71 (5.6%) | 0/71 (0.0%) | 0/71 (0.0%) |
| [Personalize Events](/services/personalizeevents) | `winterbaume-personalizeevents` | restJson1 | 5/5 (100.0%) | 0/5 (0.0%) | 0/5 (0.0%) | 0/5 (0.0%) | 0/5 (0.0%) |
| [Personalize Runtime](/services/personalizeruntime) | `winterbaume-personalizeruntime` | restJson1 | 3/3 (100.0%) | 0/3 (0.0%) | 0/3 (0.0%) | 0/3 (0.0%) | 0/3 (0.0%) |
| [Pinpoint](/services/pinpoint) | `winterbaume-pinpoint` | restJson1 | 15/122 (12.3%) | 0/122 (0.0%) | 12/122 (9.8%) | 0/122 (0.0%) | 0/122 (0.0%) |
| [Pinpoint SMS Voice](/services/pinpointsmsvoice) | `winterbaume-pinpointsmsvoice` | restJson1 | 4/8 (50.0%) | 0/8 (0.0%) | 0/8 (0.0%) | 0/8 (0.0%) | 0/8 (0.0%) |
| [Polly](/services/polly) | `winterbaume-polly` | restJson1 | 9/10 (90.0%) | 0/10 (0.0%) | 5/10 (50.0%) | 0/10 (0.0%) | 0/10 (0.0%) |
| [Pricing](/services/pricing) | `winterbaume-pricing` | awsJson1.1 | 5/5 (100.0%) | 0/5 (0.0%) | 0/5 (0.0%) | 0/5 (0.0%) | 0/5 (0.0%) |
| [Private CA Connector for SCEP](/services/pcaconnectorscep) | `winterbaume-pcaconnectorscep` | restJson1 | 11/12 (91.7%) | 0/12 (0.0%) | 0/12 (0.0%) | 0/12 (0.0%) | 0/12 (0.0%) |
| [QuickSight](/services/quicksight) | `winterbaume-quicksight` | restJson1 | 68/232 (29.3%) | 0/232 (0.0%) | 31/232 (13.4%) | 0/232 (0.0%) | 0/232 (0.0%) |
| [RAM](/services/ram) | `winterbaume-ram` | restJson1 | 35/35 (100.0%) | 0/35 (0.0%) | 8/35 (22.9%) | 0/35 (0.0%) | 0/35 (0.0%) |
| [RDS](/services/rds) | `winterbaume-rds` | awsQuery | 146/164 (89.0%) | 4/164 (2.4%) | 85/164 (51.8%) | 0/164 (0.0%) | 12/164 (7.3%) |
| [RDS Data](/services/rdsdata) | `winterbaume-rdsdata` | restJson1 | 6/6 (100.0%) | 0/6 (0.0%) | 1/6 (16.7%) | 0/6 (0.0%) | 0/6 (0.0%) |
| [Recycle Bin](/services/rbin) | `winterbaume-rbin` | restJson1 | 9/10 (90.0%) | 0/10 (0.0%) | 0/10 (0.0%) | 0/10 (0.0%) | 0/10 (0.0%) |
| [Redshift](/services/redshift) | `winterbaume-redshift` | awsQuery | 100/141 (70.9%) | 3/141 (2.1%) | 35/141 (24.8%) | 0/141 (0.0%) | 7/141 (5.0%) |
| [Redshift Data](/services/redshiftdata) | `winterbaume-redshiftdata` | awsJson1.1 | 11/11 (100.0%) | 0/11 (0.0%) | 4/11 (36.4%) | 0/11 (0.0%) | 0/11 (0.0%) |
| [Rekognition](/services/rekognition) | `winterbaume-rekognition` | awsJson1.1 | 8/75 (10.7%) | 4/75 (5.3%) | 8/75 (10.7%) | 0/75 (0.0%) | 13/75 (17.3%) |
| [Resilience Hub](/services/resiliencehub) | `winterbaume-resiliencehub` | restJson1 | 22/63 (34.9%) | 0/63 (0.0%) | 17/63 (27.0%) | 0/63 (0.0%) | 17/63 (27.0%) |
| [Resource Groups](/services/resourcegroups) | `winterbaume-resourcegroups` | restJson1 | 22/23 (95.7%) | 1/23 (4.3%) | 15/23 (65.2%) | 0/23 (0.0%) | 0/23 (0.0%) |
| [Resource Groups Tagging](/services/resourcegroupstagging) | `winterbaume-resourcegroupstagging` | awsJson1.1 | 5/9 (55.6%) | 0/9 (0.0%) | 0/9 (0.0%) | 0/9 (0.0%) | 0/9 (0.0%) |
| [Route 53](/services/route53) | `winterbaume-route53` | restXml | 71/71 (100.0%) | 0/71 (0.0%) | 30/71 (42.3%) | 0/71 (0.0%) | 10/71 (14.1%) |
| [Route 53 Domains](/services/route53domains) | `winterbaume-route53domains` | awsJson1.1 | 5/34 (14.7%) | 0/34 (0.0%) | 0/34 (0.0%) | 0/34 (0.0%) | 0/34 (0.0%) |
| [Route 53 Recovery Cluster](/services/route53recoverycluster) | `winterbaume-route53recoverycluster` | awsJson1.0 | 4/4 (100.0%) | 0/4 (0.0%) | 0/4 (0.0%) | 0/4 (0.0%) | 0/4 (0.0%) |
| [Route 53 Resolver](/services/route53resolver) | `winterbaume-route53resolver` | awsJson1.1 | 28/68 (41.2%) | 0/68 (0.0%) | 28/68 (41.2%) | 0/68 (0.0%) | 11/68 (16.2%) |
| [S3](/services/s3) | `winterbaume-s3` | restXml | 103/107 (96.3%) | 4/107 (3.7%) | 73/107 (68.2%) | 51/107 (47.7%) | 36/107 (33.6%) |
| [S3 Control](/services/s3control) | `winterbaume-s3control` | restXml | 87/97 (89.7%) | 10/97 (10.3%) | 0/97 (0.0%) | 0/97 (0.0%) | 7/97 (7.2%) |
| [S3 Files](/services/s3files) | `winterbaume-s3files` | restJson1 | 21/21 (100.0%) | 0/21 (0.0%) | 0/21 (0.0%) | 0/21 (0.0%) | 0/21 (0.0%) |
| [S3 on Outposts](/services/s3outposts) | `winterbaume-s3outposts` | restJson1 | 3/5 (60.0%) | 1/5 (20.0%) | 0/5 (0.0%) | 0/5 (0.0%) | 0/5 (0.0%) |
| [S3 Tables](/services/s3tables) | `winterbaume-s3tables` | restJson1 | 46/49 (93.9%) | 3/49 (6.1%) | 14/49 (28.6%) | 0/49 (0.0%) | 12/49 (24.5%) |
| [S3 Vectors](/services/s3vectors) | `winterbaume-s3vectors` | restJson1 | 19/19 (100.0%) | 0/19 (0.0%) | 15/19 (78.9%) | 0/19 (0.0%) | 0/19 (0.0%) |
| [SageMaker](/services/sagemaker) | `winterbaume-sagemaker` | awsJson1.1 | 141/396 (35.6%) | 1/396 (0.3%) | 112/396 (28.3%) | 0/396 (0.0%) | 11/396 (2.8%) |
| [SageMaker Metrics](/services/sagemakermetrics) | `winterbaume-sagemakermetrics` | restJson1 | 2/2 (100.0%) | 0/2 (0.0%) | 1/2 (50.0%) | 0/2 (0.0%) | 0/2 (0.0%) |
| [SageMaker Runtime](/services/sagemakerruntime) | `winterbaume-sagemakerruntime` | restJson1 | 3/3 (100.0%) | 0/3 (0.0%) | 2/3 (66.7%) | 0/3 (0.0%) | 0/3 (0.0%) |
| [Savings Plans](/services/savingsplans) | `winterbaume-savingsplans` | restJson1 | 10/10 (100.0%) | 0/10 (0.0%) | 0/10 (0.0%) | 0/10 (0.0%) | 0/10 (0.0%) |
| [Scheduler](/services/scheduler) | `winterbaume-scheduler` | restJson1 | 12/12 (100.0%) | 0/12 (0.0%) | 12/12 (100.0%) | 0/12 (0.0%) | 9/12 (75.0%) |
| [Secrets Manager](/services/secretsmanager) | `winterbaume-secretsmanager` | awsJson1.1 | 22/23 (95.7%) | 1/23 (4.3%) | 21/23 (91.3%) | 0/23 (0.0%) | 11/23 (47.8%) |
| [Security Hub](/services/securityhub) | `winterbaume-securityhub` | restJson1 | 97/107 (90.7%) | 10/107 (9.3%) | 13/107 (12.1%) | 0/107 (0.0%) | 0/107 (0.0%) |
| [Service Catalog](/services/servicecatalog) | `winterbaume-servicecatalog` | awsJson1.1 | 4/90 (4.4%) | 0/90 (0.0%) | 0/90 (0.0%) | 0/90 (0.0%) | 0/90 (0.0%) |
| [Service Catalog AppRegistry](/services/servicecatalogappregistry) | `winterbaume-servicecatalogappregistry` | restJson1 | 23/24 (95.8%) | 1/24 (4.2%) | 0/24 (0.0%) | 0/24 (0.0%) | 0/24 (0.0%) |
| [Service Discovery](/services/servicediscovery) | `winterbaume-servicediscovery` | awsJson1.1 | 27/30 (90.0%) | 0/30 (0.0%) | 27/30 (90.0%) | 0/30 (0.0%) | 0/30 (0.0%) |
| [Service Quotas](/services/servicequotas) | `winterbaume-servicequotas` | awsJson1.1 | 5/26 (19.2%) | 0/26 (0.0%) | 2/26 (7.7%) | 0/26 (0.0%) | 8/26 (30.8%) |
| [SES v1](/services/ses) | `winterbaume-ses` | awsQuery | 38/71 (53.5%) | 2/71 (2.8%) | 38/71 (53.5%) | 0/71 (0.0%) | 0/71 (0.0%) |
| [SES v2](/services/sesv2) | `winterbaume-sesv2` | restJson1 | 106/110 (96.4%) | 4/110 (3.6%) | 30/110 (27.3%) | 0/110 (0.0%) | 9/110 (8.2%) |
| [Shield](/services/shield) | `winterbaume-shield` | awsJson1.1 | 9/36 (25.0%) | 0/36 (0.0%) | 9/36 (25.0%) | 0/36 (0.0%) | 0/36 (0.0%) |
| [Signer](/services/signer) | `winterbaume-signer` | restJson1 | 19/19 (100.0%) | 0/19 (0.0%) | 7/19 (36.8%) | 0/19 (0.0%) | 0/19 (0.0%) |
| [SimpleDB v2](/services/simpledbv2) | `winterbaume-simpledbv2` | restJson1 | 3/3 (100.0%) | 0/3 (0.0%) | 0/3 (0.0%) | 0/3 (0.0%) | 0/3 (0.0%) |
| [SimSpace Weaver](/services/simspaceweaver) | `winterbaume-simspaceweaver` | restJson1 | 15/16 (93.8%) | 0/16 (0.0%) | 0/16 (0.0%) | 0/16 (0.0%) | 0/16 (0.0%) |
| [Snow Device Management](/services/snowdevicemanagement) | `winterbaume-snowdevicemanagement` | restJson1 | 11/13 (84.6%) | 0/13 (0.0%) | 0/13 (0.0%) | 0/13 (0.0%) | 0/13 (0.0%) |
| [SNS](/services/sns) | `winterbaume-sns` | awsQuery | 41/42 (97.6%) | 1/42 (2.4%) | 33/42 (78.6%) | 0/42 (0.0%) | 13/42 (31.0%) |
| [SQS](/services/sqs) | `winterbaume-sqs` | awsJson1.0 | 23/23 (100.0%) | 0/23 (0.0%) | 20/23 (87.0%) | 0/23 (0.0%) | 14/23 (60.9%) |
| [SSM](/services/ssm) | `winterbaume-ssm` | awsJson1.1 | 127/146 (87.0%) | 19/146 (13.0%) | 41/146 (28.1%) | 0/146 (0.0%) | 7/146 (4.8%) |
| [SSM Quick Setup](/services/ssmquicksetup) | `winterbaume-ssmquicksetup` | restJson1 | 6/14 (42.9%) | 0/14 (0.0%) | 0/14 (0.0%) | 0/14 (0.0%) | 0/14 (0.0%) |
| [SSO](/services/sso) | `winterbaume-sso` | restJson1 | 4/4 (100.0%) | 0/4 (0.0%) | 0/4 (0.0%) | 0/4 (0.0%) | 0/4 (0.0%) |
| [SSO Admin](/services/ssoadmin) | `winterbaume-ssoadmin` | awsJson1.1 | 27/79 (34.2%) | 1/79 (1.3%) | 25/79 (31.6%) | 0/79 (0.0%) | 0/79 (0.0%) |
| [Step Functions](/services/sfn) | `winterbaume-sfn` | awsJson1.0 | 35/37 (94.6%) | 2/37 (5.4%) | 29/37 (78.4%) | 0/37 (0.0%) | 18/37 (48.6%) |
| [STS](/services/sts) | `winterbaume-sts` | awsQuery | 11/11 (100.0%) | 0/11 (0.0%) | 7/11 (63.6%) | 0/11 (0.0%) | 6/11 (54.5%) |
| [Support](/services/support) | `winterbaume-support` | awsJson1.1 | 5/16 (31.2%) | 1/16 (6.2%) | 5/16 (31.2%) | 0/16 (0.0%) | 0/16 (0.0%) |
| [Support App](/services/supportapp) | `winterbaume-supportapp` | restJson1 | 3/10 (30.0%) | 0/10 (0.0%) | 0/10 (0.0%) | 0/10 (0.0%) | 0/10 (0.0%) |
| [SWF](/services/swf) | `winterbaume-swf` | awsJson1.0 | 30/39 (76.9%) | 0/39 (0.0%) | 19/39 (48.7%) | 0/39 (0.0%) | 0/39 (0.0%) |
| [Synthetics](/services/synthetics) | `winterbaume-synthetics` | restJson1 | 22/22 (100.0%) | 0/22 (0.0%) | 4/22 (18.2%) | 0/22 (0.0%) | 0/22 (0.0%) |
| [Tax Settings](/services/taxsettings) | `winterbaume-taxsettings` | restJson1 | 0/16 (0.0%) | 0/16 (0.0%) | 0/16 (0.0%) | 0/16 (0.0%) | 0/16 (0.0%) |
| [Textract](/services/textract) | `winterbaume-textract` | awsJson1.1 | 6/25 (24.0%) | 0/25 (0.0%) | 5/25 (20.0%) | 0/25 (0.0%) | 0/25 (0.0%) |
| [Timestream InfluxDB](/services/timestreaminfluxdb) | `winterbaume-timestreaminfluxdb` | awsJson1.0 | 19/19 (100.0%) | 0/19 (0.0%) | 13/19 (68.4%) | 0/19 (0.0%) | 0/19 (0.0%) |
| [Timestream Query](/services/timestreamquery) | `winterbaume-timestreamquery` | awsJson1.0 | 15/15 (100.0%) | 0/15 (0.0%) | 6/15 (40.0%) | 0/15 (0.0%) | 0/15 (0.0%) |
| [Timestream Write](/services/timestreamwrite) | `winterbaume-timestreamwrite` | awsJson1.0 | 19/19 (100.0%) | 0/19 (0.0%) | 15/19 (78.9%) | 0/19 (0.0%) | 0/19 (0.0%) |
| [Transcribe](/services/transcribe) | `winterbaume-transcribe` | awsJson1.1 | 16/43 (37.2%) | 0/43 (0.0%) | 16/43 (37.2%) | 0/43 (0.0%) | 0/43 (0.0%) |
| [Transfer](/services/transfer) | `winterbaume-transfer` | awsJson1.1 | 44/71 (62.0%) | 0/71 (0.0%) | 14/71 (19.7%) | 0/71 (0.0%) | 0/71 (0.0%) |
| [Trusted Advisor](/services/trustedadvisor) | `winterbaume-trustedadvisor` | restJson1 | 6/11 (54.5%) | 4/11 (36.4%) | 0/11 (0.0%) | 0/11 (0.0%) | 0/11 (0.0%) |
| [VPC Lattice](/services/vpclattice) | `winterbaume-vpclattice` | restJson1 | 66/73 (90.4%) | 2/73 (2.7%) | 35/73 (47.9%) | 0/73 (0.0%) | 0/73 (0.0%) |
| [WAFv2](/services/wafv2) | `winterbaume-wafv2` | awsJson1.1 | 38/55 (69.1%) | 0/55 (0.0%) | 29/55 (52.7%) | 0/55 (0.0%) | 0/55 (0.0%) |
| [WorkSpaces](/services/workspaces) | `winterbaume-workspaces` | awsJson1.1 | 50/91 (54.9%) | 0/91 (0.0%) | 16/91 (17.6%) | 0/91 (0.0%) | 0/91 (0.0%) |
| [WorkSpaces Web](/services/workspacesweb) | `winterbaume-workspacesweb` | restJson1 | 68/75 (90.7%) | 0/75 (0.0%) | 27/75 (36.0%) | 0/75 (0.0%) | 0/75 (0.0%) |
| [X-Ray](/services/xray) | `winterbaume-xray` | restJson1 | 34/38 (89.5%) | 4/38 (10.5%) | 0/38 (0.0%) | 0/38 (0.0%) | 6/38 (15.8%) |

**winterbaume: 7210 / 11367 operations across 224 services (63.4%)**

**winterbaume stubs: 326 / 11367 operations across 224 services (2.9%) - routed but return empty/default responses**

**moto: 3304 / 11367 operations across 224 services (29.1%)**

**floci: 160 / 11367 operations across 224 services (1.4%)**

**kumo: 790 / 11367 operations across 224 services (6.9%)**

---

See also: [Terraform Converter Coverage](/reference/terraform) — 1,140 Terraform resource types with inject/extract field coverage against the official AWS provider schema.