#!/usr/bin/env python3
"""Update workspace and service README.md files with API coverage data.

Parses .agents/docs/API_COVERAGE.md and rewrites the Supported Services
table in README.md to include per-service operation counts and coverage
percentages for both winterbaume and moto. Also rewrites
`crates/winterbaume-*/README.md` files so each service crate advertises
its current operation coverage.

Usage:
    python3 .agents/skills/update-readme/scripts/update_readme.py [--coverage PATH] [--readme PATH] [--crates-dir PATH]
"""

import argparse
import os
import re
import sys
from pathlib import Path

# Make the api-coverage helpers importable. This script lives at
# `.agents/skills/update-readme/scripts/update_readme.py` and the helper at
# `.agents/skills/api-coverage/scripts/generate_coverage.py`; both are
# siblings under `.agents/skills/`.
_API_COVERAGE_SCRIPTS = (
    Path(__file__).resolve().parent.parent.parent / "api-coverage" / "scripts"
)
if str(_API_COVERAGE_SCRIPTS) not in sys.path:
    sys.path.insert(0, str(_API_COVERAGE_SCRIPTS))

from generate_coverage import CRATE_TO_MODEL, derive_protocol  # noqa: E402

# Human-readable display name for each crate. The protocol is no longer
# carried here; it is derived from the Smithy service shape's protocol
# trait at coverage-generation time via
# `generate_coverage.derive_protocol`. See the
# `crate-display-info-derive` TODO entry for the migration rationale.
# Keep sorted alphabetically by crate name.
CRATE_DISPLAY_INFO: dict[str, str] = {
    "winterbaume-accessanalyzer": "IAM Access Analyzer",
    "winterbaume-account": "Account",
    "winterbaume-acm": "ACM",
    "winterbaume-acmpca": "ACM PCA",
    "winterbaume-aiops": "AIOps",
    "winterbaume-amp": "AMP/Prometheus",
    "winterbaume-amplify": "Amplify",
    "winterbaume-amplifybackend": "Amplify Backend",
    "winterbaume-amplifyuibuilder": "Amplify UI Builder",
    "winterbaume-apigateway": "API Gateway",
    "winterbaume-apigatewaymanagement": "API Gateway Management API",
    "winterbaume-apigatewayv2": "API Gateway V2",
    "winterbaume-appconfig": "AppConfig",
    "winterbaume-appconfigdata": "AppConfig Data",
    "winterbaume-appfabric": "AppFabric",
    "winterbaume-appflow": "AppFlow",
    "winterbaume-appintegrations": "AppIntegrations",
    "winterbaume-applicationautoscaling": "Application Auto Scaling",
    "winterbaume-applicationcostprofiler": "Application Cost Profiler",
    "winterbaume-applicationdiscoveryservice": "Application Discovery Service",
    "winterbaume-applicationinsights": "Application Insights",
    "winterbaume-applicationsignals": "Application Signals",
    "winterbaume-appmesh": "App Mesh",
    "winterbaume-apprunner": "App Runner",
    "winterbaume-appsync": "AppSync",
    "winterbaume-arczonalshift": "ARC Zonal Shift",
    "winterbaume-artifact": "Artifact",
    "winterbaume-athena": "Athena",
    "winterbaume-auditmanager": "Audit Manager",
    "winterbaume-autoscaling": "Auto Scaling",
    "winterbaume-autoscalingplans": "Auto Scaling Plans",
    "winterbaume-backup": "Backup",
    "winterbaume-backupgateway": "Backup Gateway",
    "winterbaume-backupsearch": "Backup Search",
    "winterbaume-batch": "Batch",
    "winterbaume-bcmdashboards": "BCM Dashboards",
    "winterbaume-bcmdataexports": "BCM Data Exports",
    "winterbaume-bcmrecommendedactions": "BCM Recommended Actions",
    "winterbaume-bedrock": "Bedrock",
    "winterbaume-bedrockagent": "Bedrock Agent",
    "winterbaume-billing": "Billing",
    "winterbaume-braket": "Braket",
    "winterbaume-budgets": "Budgets",
    "winterbaume-chatbot": "Chatbot",
    "winterbaume-chimesdkmeetings": "Chime SDK Meetings",
    "winterbaume-cloud9": "Cloud9",
    "winterbaume-cloudcontrol": "Cloud Control API",
    "winterbaume-clouddirectory": "Cloud Directory",
    "winterbaume-cloudformation": "CloudFormation",
    "winterbaume-cloudfront": "CloudFront",
    "winterbaume-cloudfrontkeyvaluestore": "CloudFront KeyValueStore",
    "winterbaume-cloudhsmv2": "CloudHSM v2",
    "winterbaume-cloudsearchdomain": "CloudSearch Domain",
    "winterbaume-cloudtrail": "CloudTrail",
    "winterbaume-cloudtraildata": "CloudTrail Data",
    "winterbaume-cloudwatch": "CloudWatch",
    "winterbaume-cloudwatchlogs": "CloudWatch Logs",
    "winterbaume-codeartifact": "CodeArtifact",
    "winterbaume-codebuild": "CodeBuild",
    "winterbaume-codecommit": "CodeCommit",
    "winterbaume-codedeploy": "CodeDeploy",
    "winterbaume-codegurureviewer": "CodeGuru Reviewer",
    "winterbaume-codegurusecurity": "CodeGuru Security",
    "winterbaume-codepipeline": "CodePipeline",
    "winterbaume-codestarnotifications": "CodeStar Notifications",
    "winterbaume-cognitoidentity": "Cognito Identity",
    "winterbaume-cognitoidentityprovider": "Cognito Identity Provider",
    "winterbaume-cognitosync": "Cognito Sync",
    "winterbaume-comprehend": "Comprehend",
    "winterbaume-config": "Config",
    "winterbaume-connect": "Connect",
    "winterbaume-connectcampaigns": "Connect Campaigns",
    "winterbaume-connectcontactlens": "Connect Contact Lens",
    "winterbaume-connectparticipant": "Connect Participant",
    "winterbaume-controlcatalog": "Control Catalog",
    "winterbaume-costandusagereport": "Cost and Usage Report Service",
    "winterbaume-costexplorer": "Cost Explorer",
    "winterbaume-costoptimizationhub": "Cost Optimization Hub",
    "winterbaume-databasemigration": "Database Migration Service (DMS)",
    "winterbaume-databrew": "DataBrew",
    "winterbaume-datapipeline": "Data Pipeline",
    "winterbaume-datasync": "DataSync",
    "winterbaume-dax": "DAX",
    "winterbaume-directconnect": "Direct Connect",
    "winterbaume-directory": "Directory Service",
    "winterbaume-dlm": "DLM",
    "winterbaume-dsql": "Aurora DSQL",
    "winterbaume-dynamodb": "DynamoDB",
    "winterbaume-dynamodbstreams": "DynamoDB Streams",
    "winterbaume-ebs": "EBS",
    "winterbaume-ec2": "EC2",
    "winterbaume-ec2instanceconnect": "EC2 Instance Connect",
    "winterbaume-ecr": "ECR",
    "winterbaume-ecs": "ECS",
    "winterbaume-efs": "EFS",
    "winterbaume-eks": "EKS",
    "winterbaume-elasticache": "ElastiCache",
    "winterbaume-elasticbeanstalk": "Elastic Beanstalk",
    "winterbaume-elasticloadbalancing": "ELB",
    "winterbaume-elasticloadbalancingv2": "ELBv2",
    "winterbaume-emr": "EMR",
    "winterbaume-emrcontainers": "EMR Containers",
    "winterbaume-emrserverless": "EMR Serverless",
    "winterbaume-eventbridge": "EventBridge",
    "winterbaume-firehose": "Firehose",
    "winterbaume-fis": "FIS",
    "winterbaume-forecast": "Forecast",
    "winterbaume-freetier": "Free Tier",
    "winterbaume-fsx": "FSx",
    "winterbaume-glacier": "Glacier",
    "winterbaume-glue": "Glue",
    "winterbaume-greengrass": "Greengrass",
    "winterbaume-guardduty": "GuardDuty",
    "winterbaume-iam": "IAM",
    "winterbaume-identitystore": "Identity Store",
    "winterbaume-inspector2": "Inspector2",
    "winterbaume-iot": "IoT",
    "winterbaume-iotdataplane": "IoT Data Plane",
    "winterbaume-ivs": "IVS",
    "winterbaume-kafka": "MSK",
    "winterbaume-keyspaces": "Keyspaces",
    "winterbaume-kinesis": "Kinesis",
    "winterbaume-kinesisanalyticsv2": "Kinesis Analytics V2",
    "winterbaume-kinesisvideo": "Kinesis Video",
    "winterbaume-kinesisvideoarchivedmedia": "Kinesis Video Archived Media",
    "winterbaume-kms": "KMS",
    "winterbaume-lakeformation": "Lake Formation",
    "winterbaume-lambda": "Lambda",
    "winterbaume-lexmodelsv2": "Lex Models V2",
    "winterbaume-macie2": "Macie2",
    "winterbaume-managedblockchain": "Managed Blockchain",
    "winterbaume-marketplacemetering": "Marketplace Metering",
    "winterbaume-mediaconnect": "MediaConnect",
    "winterbaume-medialive": "MediaLive",
    "winterbaume-mediapackage": "MediaPackage",
    "winterbaume-mediapackagev2": "MediaPackage v2",
    "winterbaume-mediastore": "MediaStore",
    "winterbaume-mediastoredata": "MediaStore Data",
    "winterbaume-memorydb": "MemoryDB",
    "winterbaume-mq": "MQ",
    "winterbaume-neptune": "Neptune",
    "winterbaume-networkfirewall": "Network Firewall",
    "winterbaume-networkmanager": "Network Manager",
    "winterbaume-opensearch": "OpenSearch",
    "winterbaume-opensearchserverless": "OpenSearch Serverless",
    "winterbaume-organizations": "Organizations",
    "winterbaume-osis": "OpenSearch Ingestion",
    "winterbaume-outposts": "Outposts",
    "winterbaume-panorama": "Panorama",
    "winterbaume-pcaconnectorscep": "Private CA Connector for SCEP",
    "winterbaume-personalize": "Personalize",
    "winterbaume-personalizeevents": "Personalize Events",
    "winterbaume-personalizeruntime": "Personalize Runtime",
    "winterbaume-pi": "Performance Insights",
    "winterbaume-pinpoint": "Pinpoint",
    "winterbaume-pinpointsmsvoice": "Pinpoint SMS Voice",
    "winterbaume-pipes": "EventBridge Pipes",
    "winterbaume-polly": "Polly",
    "winterbaume-pricing": "Pricing",
    "winterbaume-quicksight": "QuickSight",
    "winterbaume-ram": "RAM",
    "winterbaume-rbin": "Recycle Bin",
    "winterbaume-rds": "RDS",
    "winterbaume-rdsdata": "RDS Data",
    "winterbaume-redshift": "Redshift",
    "winterbaume-redshiftdata": "Redshift Data",
    "winterbaume-rekognition": "Rekognition",
    "winterbaume-resiliencehub": "Resilience Hub",
    "winterbaume-resourcegroups": "Resource Groups",
    "winterbaume-resourcegroupstagging": "Resource Groups Tagging",
    "winterbaume-rolesanywhere": "IAM Roles Anywhere",
    "winterbaume-route53": "Route 53",
    "winterbaume-route53domains": "Route 53 Domains",
    "winterbaume-route53recoverycluster": "Route 53 Recovery Cluster",
    "winterbaume-route53resolver": "Route 53 Resolver",
    "winterbaume-s3": "S3",
    "winterbaume-s3control": "S3 Control",
    "winterbaume-s3files": "S3 Files",
    "winterbaume-s3outposts": "S3 on Outposts",
    "winterbaume-s3tables": "S3 Tables",
    "winterbaume-s3vectors": "S3 Vectors",
    "winterbaume-sagemaker": "SageMaker",
    "winterbaume-sagemakermetrics": "SageMaker Metrics",
    "winterbaume-sagemakerruntime": "SageMaker Runtime",
    "winterbaume-savingsplans": "Savings Plans",
    "winterbaume-scheduler": "Scheduler",
    "winterbaume-secretsmanager": "Secrets Manager",
    "winterbaume-securityhub": "Security Hub",
    "winterbaume-servicecatalog": "Service Catalog",
    "winterbaume-servicecatalogappregistry": "Service Catalog AppRegistry",
    "winterbaume-servicediscovery": "Service Discovery",
    "winterbaume-servicequotas": "Service Quotas",
    "winterbaume-ses": "SES v1",
    "winterbaume-sesv2": "SES v2",
    "winterbaume-sfn": "Step Functions",
    "winterbaume-shield": "Shield",
    "winterbaume-signer": "Signer",
    "winterbaume-simpledbv2": "SimpleDB v2",
    "winterbaume-simspaceweaver": "SimSpace Weaver",
    "winterbaume-snowdevicemanagement": "Snow Device Management",
    "winterbaume-sns": "SNS",
    "winterbaume-sqs": "SQS",
    "winterbaume-ssm": "SSM",
    "winterbaume-ssmquicksetup": "SSM Quick Setup",
    "winterbaume-sso": "SSO",
    "winterbaume-ssoadmin": "SSO Admin",
    "winterbaume-sts": "STS",
    "winterbaume-support": "Support",
    "winterbaume-supportapp": "Support App",
    "winterbaume-swf": "SWF",
    "winterbaume-synthetics": "Synthetics",
    "winterbaume-taxsettings": "Tax Settings",
    "winterbaume-textract": "Textract",
    "winterbaume-timestreaminfluxdb": "Timestream InfluxDB",
    "winterbaume-timestreamquery": "Timestream Query",
    "winterbaume-timestreamwrite": "Timestream Write",
    "winterbaume-transcribe": "Transcribe",
    "winterbaume-transfer": "Transfer",
    "winterbaume-trustedadvisor": "Trusted Advisor",
    "winterbaume-vpclattice": "VPC Lattice",
    "winterbaume-wafv2": "WAFv2",
    "winterbaume-workspaces": "WorkSpaces",
    "winterbaume-workspacesweb": "WorkSpaces Web",
    "winterbaume-xray": "X-Ray",
}


# Fallback protocol labels for crates that have no `CRATE_TO_MODEL` entry
# (typically because the Smithy model is not vendored under
# `vendor/api-models-aws/`). Empty today, retained as a transitional escape
# hatch for crates added before their Smithy model is wired in.
CRATE_DISPLAY_INFO_LEGACY: dict[str, str] = {}


def lookup_protocol(crate: str) -> str:
    """Return the protocol label for a crate.

    Prefers Smithy-derived values when the crate has a `CRATE_TO_MODEL`
    entry; otherwise falls back to `CRATE_DISPLAY_INFO_LEGACY`, finally
    returning ``"?"`` to keep the rendered table well-formed.
    """
    model_name = CRATE_TO_MODEL.get(crate)
    if model_name:
        derived = derive_protocol(model_name)
        if derived != "unknown":
            return derived
    return CRATE_DISPLAY_INFO_LEGACY.get(crate, "?")


# Maps crate name to (aws_cli_service_name, example_cli_command).
# The example_cli_command is a complete aws CLI invocation (without the endpoint
# variable prefix) that demonstrates a simple read operation against the service.
# Used to generate the "Server-mode usage" section in each service README.
CRATE_CLI_EXAMPLES: dict[str, tuple[str, str]] = {
    "winterbaume-account": ("account", "aws account list-regions"),
    "winterbaume-acm": ("acm", "aws acm list-certificates"),
    "winterbaume-acmpca": ("acm-pca", "aws acm-pca list-certificate-authorities"),
    "winterbaume-amp": ("amp", "aws amp list-workspaces"),
    "winterbaume-apigateway": ("apigateway", "aws apigateway get-rest-apis"),
    "winterbaume-apigatewaymanagement": ("apigatewaymanagementapi", "aws apigatewaymanagementapi get-connection --connection-id dummy"),
    "winterbaume-apigatewayv2": ("apigatewayv2", "aws apigatewayv2 get-apis"),
    "winterbaume-appconfig": ("appconfig", "aws appconfig list-applications"),
    "winterbaume-applicationautoscaling": ("application-autoscaling", "aws application-autoscaling describe-scalable-targets --service-namespace ecs"),
    "winterbaume-appmesh": ("appmesh", "aws appmesh list-meshes"),
    "winterbaume-appsync": ("appsync", "aws appsync list-graphql-apis"),
    "winterbaume-athena": ("athena", "aws athena list-work-groups"),
    "winterbaume-autoscaling": ("autoscaling", "aws autoscaling describe-auto-scaling-groups"),
    "winterbaume-backup": ("backup", "aws backup list-backup-plans"),
    "winterbaume-batch": ("batch", "aws batch describe-job-queues"),
    "winterbaume-bedrock": ("bedrock", "aws bedrock list-foundation-models"),
    "winterbaume-bedrockagent": ("bedrock-agent", "aws bedrock-agent list-agents"),
    "winterbaume-budgets": ("budgets", "aws budgets describe-budgets --account-id 123456789012"),
    "winterbaume-cloudformation": ("cloudformation", "aws cloudformation list-stacks"),
    "winterbaume-cloudfront": ("cloudfront", "aws cloudfront list-distributions"),
    "winterbaume-cloudhsmv2": ("cloudhsmv2", "aws cloudhsmv2 describe-clusters"),
    "winterbaume-cloudtrail": ("cloudtrail", "aws cloudtrail describe-trails"),
    "winterbaume-cloudwatch": ("cloudwatch", "aws cloudwatch list-metrics"),
    "winterbaume-codebuild": ("codebuild", "aws codebuild list-projects"),
    "winterbaume-codecommit": ("codecommit", "aws codecommit list-repositories"),
    "winterbaume-codedeploy": ("deploy", "aws deploy list-applications"),
    "winterbaume-codepipeline": ("codepipeline", "aws codepipeline list-pipelines"),
    "winterbaume-cognitoidentity": ("cognito-identity", "aws cognito-identity list-identity-pools --max-results 10"),
    "winterbaume-cognitoidentityprovider": ("cognito-idp", "aws cognito-idp list-user-pools --max-results 10"),
    "winterbaume-comprehend": ("comprehend", "aws comprehend list-endpoints"),
    "winterbaume-config": ("configservice", "aws configservice describe-config-rules"),
    "winterbaume-connect": ("connect", "aws connect list-instances"),
    "winterbaume-connectcampaigns": ("connectcampaigns", "aws connectcampaigns list-campaigns --instance-id 00000000-0000-0000-0000-000000000000"),
    "winterbaume-costexplorer": ("ce", "aws ce list-cost-category-definitions"),
    "winterbaume-databrew": ("databrew", "aws databrew list-projects"),
    "winterbaume-datapipeline": ("datapipeline", "aws datapipeline list-pipelines"),
    "winterbaume-datasync": ("datasync", "aws datasync list-agents"),
    "winterbaume-dax": ("dax", "aws dax describe-clusters"),
    "winterbaume-directconnect": ("directconnect", "aws directconnect describe-connections"),
    "winterbaume-directory": ("ds", "aws ds describe-directories"),
    "winterbaume-dsql": ("dsql", "aws dsql list-clusters"),
    "winterbaume-databasemigration": ("dms", "aws dms describe-replication-instances"),
    "winterbaume-dynamodb": ("dynamodb", "aws dynamodb list-tables"),
    "winterbaume-dynamodbstreams": ("dynamodbstreams", "aws dynamodbstreams list-streams"),
    "winterbaume-ebs": ("ebs", "aws ebs list-changed-blocks --first-snapshot-id snap-000000000000 --second-snapshot-id snap-000000000001"),
    "winterbaume-ec2": ("ec2", "aws ec2 describe-vpcs"),
    "winterbaume-ec2instanceconnect": ("ec2-instance-connect", "aws ec2-instance-connect send-ssh-public-key --instance-id i-00000000 --instance-os-user ec2-user --ssh-public-key file://key.pub"),
    "winterbaume-ecr": ("ecr", "aws ecr describe-repositories"),
    "winterbaume-ecs": ("ecs", "aws ecs list-clusters"),
    "winterbaume-efs": ("efs", "aws efs describe-file-systems"),
    "winterbaume-eks": ("eks", "aws eks list-clusters"),
    "winterbaume-elasticache": ("elasticache", "aws elasticache describe-cache-clusters"),
    "winterbaume-elasticloadbalancing": ("elb", "aws elb describe-load-balancers"),
    "winterbaume-elasticloadbalancingv2": ("elbv2", "aws elbv2 describe-load-balancers"),
    "winterbaume-emr": ("emr", "aws emr list-clusters"),
    "winterbaume-emrcontainers": ("emr-containers", "aws emr-containers list-virtual-clusters"),
    "winterbaume-emrserverless": ("emr-serverless", "aws emr-serverless list-applications"),
    "winterbaume-eventbridge": ("events", "aws events list-rules"),
    "winterbaume-firehose": ("firehose", "aws firehose list-delivery-streams"),
    "winterbaume-forecast": ("forecast", "aws forecast list-datasets"),
    "winterbaume-fsx": ("fsx", "aws fsx describe-file-systems"),
    "winterbaume-glacier": ("glacier", "aws glacier list-vaults --account-id -"),
    "winterbaume-glue": ("glue", "aws glue list-registries"),
    "winterbaume-greengrass": ("greengrass", "aws greengrass list-groups"),
    "winterbaume-guardduty": ("guardduty", "aws guardduty list-detectors"),
    "winterbaume-iam": ("iam", "aws iam list-users"),
    "winterbaume-identitystore": ("identitystore", "aws identitystore list-users --identity-store-id d-1234567890"),
    "winterbaume-inspector2": ("inspector2", "aws inspector2 list-findings"),
    "winterbaume-iot": ("iot", "aws iot list-things"),
    "winterbaume-iotdataplane": ("iot-data", "aws iot-data list-retained-messages"),
    "winterbaume-ivs": ("ivs", "aws ivs list-channels"),
    "winterbaume-kafka": ("kafka", "aws kafka list-clusters"),
    "winterbaume-kinesis": ("kinesis", "aws kinesis list-streams"),
    "winterbaume-kinesisanalyticsv2": ("kinesisanalyticsv2", "aws kinesisanalyticsv2 list-applications"),
    "winterbaume-kinesisvideo": ("kinesisvideo", "aws kinesisvideo list-streams"),
    "winterbaume-kinesisvideoarchivedmedia": ("kinesis-video-archived-media", "aws kinesis-video-archived-media list-fragments --stream-arn arn:aws:kinesisvideo:us-east-1:123456789012:stream/my-stream/000000000000"),
    "winterbaume-kms": ("kms", "aws kms list-keys"),
    "winterbaume-lakeformation": ("lakeformation", "aws lakeformation list-resources"),
    "winterbaume-lambda": ("lambda", "aws lambda list-functions"),
    "winterbaume-cloudwatchlogs": ("logs", "aws logs describe-log-groups"),
    "winterbaume-macie2": ("macie2", "aws macie2 list-findings"),
    "winterbaume-managedblockchain": ("managedblockchain", "aws managedblockchain list-networks"),
    "winterbaume-mediaconnect": ("mediaconnect", "aws mediaconnect list-flows"),
    "winterbaume-medialive": ("medialive", "aws medialive list-channels"),
    "winterbaume-mediapackage": ("mediapackage", "aws mediapackage list-channels"),
    "winterbaume-mediapackagev2": ("mediapackagev2", "aws mediapackagev2 list-channel-groups"),
    "winterbaume-mediastore": ("mediastore", "aws mediastore list-containers"),
    "winterbaume-mediastoredata": ("mediastore-data", "aws mediastore-data list-items --endpoint-url http://localhost:5555"),
    "winterbaume-memorydb": ("memorydb", "aws memorydb describe-clusters"),
    "winterbaume-marketplacemetering": ("meteringmarketplace", "aws meteringmarketplace resolve-customer --registration-token dummy-token"),
    "winterbaume-mq": ("mq", "aws mq list-brokers"),
    "winterbaume-neptune": ("neptune", "aws neptune describe-db-clusters"),
    "winterbaume-networkfirewall": ("network-firewall", "aws network-firewall list-firewalls"),
    "winterbaume-networkmanager": ("networkmanager", "aws networkmanager list-core-networks"),
    "winterbaume-opensearch": ("opensearch", "aws opensearch list-domain-names"),
    "winterbaume-organizations": ("organizations", "aws organizations list-accounts"),
    "winterbaume-osis": ("osis", "aws osis list-pipelines"),
    "winterbaume-personalize": ("personalize", "aws personalize list-datasets"),
    "winterbaume-pinpoint": ("pinpoint", "aws pinpoint get-apps"),
    "winterbaume-pipes": ("pipes", "aws pipes list-pipes"),
    "winterbaume-polly": ("polly", "aws polly describe-voices"),
    "winterbaume-quicksight": ("quicksight", "aws quicksight list-dashboards --aws-account-id 123456789012"),
    "winterbaume-ram": ("ram", "aws ram list-resources --resource-owner SELF"),
    "winterbaume-rds": ("rds", "aws rds describe-db-instances"),
    "winterbaume-rdsdata": ("rds-data", "aws rds-data execute-statement --resource-arn arn:aws:rds:us-east-1:123456789012:cluster:my-cluster --secret-arn arn:aws:secretsmanager:us-east-1:123456789012:secret:my-secret --sql 'SELECT 1'"),
    "winterbaume-redshift": ("redshift", "aws redshift describe-clusters"),
    "winterbaume-redshiftdata": ("redshift-data", "aws redshift-data list-statements"),
    "winterbaume-rekognition": ("rekognition", "aws rekognition list-collections"),
    "winterbaume-resiliencehub": ("resiliencehub", "aws resiliencehub list-apps"),
    "winterbaume-resourcegroups": ("resource-groups", "aws resource-groups list-groups"),
    "winterbaume-resourcegroupstagging": ("resourcegroupstaggingapi", "aws resourcegroupstaggingapi get-resources"),
    "winterbaume-route53": ("route53", "aws route53 list-hosted-zones"),
    "winterbaume-route53domains": ("route53domains", "aws route53domains list-domains"),
    "winterbaume-route53resolver": ("route53resolver", "aws route53resolver list-resolver-endpoints"),
    "winterbaume-s3": ("s3api", "aws s3api list-buckets"),
    "winterbaume-s3control": ("s3control", "aws s3control list-jobs --account-id 123456789012"),
    "winterbaume-s3tables": ("s3tables", "aws s3tables list-table-buckets"),
    "winterbaume-s3vectors": ("s3vectors", "aws s3vectors list-vector-buckets"),
    "winterbaume-sagemaker": ("sagemaker", "aws sagemaker list-models"),
    "winterbaume-sagemakermetrics": ("sagemaker-metrics", "aws sagemaker-metrics batch-get-metrics --metric-queries MetricName=loss,ResourceArn=arn:aws:sagemaker:us-east-1:123456789012:experiment-trial/my-trial,MetricStat=Max,Period=OneMinute,XAxisType=TIMESTAMP"),
    "winterbaume-sagemakerruntime": ("sagemaker-runtime", "aws sagemaker-runtime invoke-endpoint --endpoint-name my-endpoint --body '{}' /dev/stdout"),
    "winterbaume-scheduler": ("scheduler", "aws scheduler list-schedules"),
    "winterbaume-secretsmanager": ("secretsmanager", "aws secretsmanager list-secrets"),
    "winterbaume-securityhub": ("securityhub", "aws securityhub list-hubs"),
    "winterbaume-servicecatalog": ("servicecatalog", "aws servicecatalog list-portfolios"),
    "winterbaume-servicecatalogappregistry": ("servicecatalog-appregistry", "aws servicecatalog-appregistry list-applications"),
    "winterbaume-servicediscovery": ("servicediscovery", "aws servicediscovery list-namespaces"),
    "winterbaume-servicequotas": ("service-quotas", "aws service-quotas list-services"),
    "winterbaume-sesv2": ("sesv2", "aws sesv2 list-email-identities"),
    "winterbaume-ses": ("ses", "aws ses list-identities"),
    "winterbaume-shield": ("shield", "aws shield list-protections"),
    "winterbaume-signer": ("signer", "aws signer list-signing-jobs"),
    "winterbaume-simpledbv2": ("sdb", "aws sdb list-domains"),
    "winterbaume-sns": ("sns", "aws sns list-topics"),
    "winterbaume-sqs": ("sqs", "aws sqs list-queues"),
    "winterbaume-ssm": ("ssm", "aws ssm describe-parameters"),
    "winterbaume-sso": ("sso", "aws sso list-accounts --access-token dummy-token"),
    "winterbaume-sfn": ("stepfunctions", "aws stepfunctions list-state-machines"),
    "winterbaume-sts": ("sts", "aws sts get-caller-identity"),
    "winterbaume-support": ("support", "aws support describe-services"),
    "winterbaume-swf": ("swf", "aws swf list-domains --registration-status REGISTERED"),
    "winterbaume-synthetics": ("synthetics", "aws synthetics describe-canaries"),
    "winterbaume-textract": ("textract", "aws textract list-adapters"),
    "winterbaume-timestreaminfluxdb": ("timestream-influxdb", "aws timestream-influxdb list-db-instances"),
    "winterbaume-timestreamquery": ("timestream-query", "aws timestream-query list-scheduled-queries"),
    "winterbaume-timestreamwrite": ("timestream-write", "aws timestream-write list-databases"),
    "winterbaume-transcribe": ("transcribe", "aws transcribe list-transcription-jobs"),
    "winterbaume-transfer": ("transfer", "aws transfer list-servers"),
    "winterbaume-vpclattice": ("vpc-lattice", "aws vpc-lattice list-services"),
    "winterbaume-wafv2": ("wafv2", "aws wafv2 list-web-acls --scope REGIONAL"),
    "winterbaume-workspaces": ("workspaces", "aws workspaces describe-workspaces"),
    "winterbaume-workspacesweb": ("workspaces-web", "aws workspaces-web list-portals"),
    "winterbaume-xray": ("xray", "aws xray get-groups"),
}


# Crates whose example file uses a different slug than the crate name.
_EXAMPLE_SLUG_ALIASES: dict[str, str] = {
    "winterbaume-databasemigration": "dms",
    "winterbaume-resourcegroupstagging": "tagging",
}

# Extra notes inserted after the umbrella-crate paragraph for specific crates.
_CRATE_NOTES: dict[str, str] = {
    "winterbaume-databasemigration": (
        "**Note:** the example below calls the service handler directly via "
        "`MockRequest`. To use the AWS SDK Rust client, depend on "
        "`aws-sdk-databasemigration` ( the SDK crate name omits the trailing "
        "`service` )."
    ),
}

DOSSIER_TRANSCRIBED_SECTIONS: tuple[str, ...] = (
    "Current Network Resource Stub Semantics",
)


def find_project_root() -> Path:
    """Find the project root by looking for Cargo.toml."""
    d = Path(__file__).resolve().parent
    while d != d.parent:
        if (d / "Cargo.toml").exists() and (d / "crates").exists():
            return d
        d = d.parent
    print("Error: Could not find project root", file=sys.stderr)
    sys.exit(1)


def parse_coverage_overview(
    coverage_path: Path,
) -> tuple[list[dict[str, str]], str, str, str, str, str, str]:
    """Parse the overview table from API_COVERAGE.md.

    Returns ``(rows, wb_summary, stub_summary, moto_summary, floci_summary,
    kumo_summary, generated_on)`` where each row is a dict with keys:
    crate, model, wb_impl, stub_impl, moto_impl, floci_impl, kumo_impl,
    total, wb_pct, stub_pct, moto_pct, floci_pct, kumo_pct.
    """
    if not coverage_path.exists():
        print(f"Error: {coverage_path} not found. Run api-coverage first.",
              file=sys.stderr)
        sys.exit(1)

    with open(coverage_path) as f:
        content = f.read()

    generated_on = ""
    generated_match = re.search(r"^Generated: ([0-9-]+)$", content, re.MULTILINE)
    if generated_match:
        generated_on = generated_match.group(1)

    rows = []
    # Parse table rows: | crate | model | wb | stubs | moto | floci | kumo
    #     | total | wb% | stub% | moto% | floci% | kumo% |
    new_format = re.compile(
        r'^\| (winterbaume-\S+) \| (\S+) '
        r'\| (\d+) \| (\d+) \| (\d+) \| (\d+) \| (\d+) \| (\d+) '
        r'\| ([\d.]+%) \| ([\d.]+%) \| ([\d.]+%) \| ([\d.]+%) \| ([\d.]+%) \|',
        re.MULTILINE,
    )
    for m in new_format.finditer(content):
        rows.append({
            "crate": m.group(1),
            "model": m.group(2),
            "wb_impl": m.group(3),
            "stub_impl": m.group(4),
            "moto_impl": m.group(5),
            "floci_impl": m.group(6),
            "kumo_impl": m.group(7),
            "total": m.group(8),
            "wb_pct": m.group(9),
            "stub_pct": m.group(10),
            "moto_pct": m.group(11),
            "floci_pct": m.group(12),
            "kumo_pct": m.group(13),
        })

    if not rows:
        # Fall back to the previous format (no stubs column) for compatibility.
        old_format = re.compile(
            r'^\| (winterbaume-\S+) \| (\S+) '
            r'\| (\d+) \| (\d+) \| (\d+) \| (\d+) \| (\d+) '
            r'\| ([\d.]+%) \| ([\d.]+%) \| ([\d.]+%) \| ([\d.]+%) \|',
            re.MULTILINE,
        )
        for m in old_format.finditer(content):
            rows.append({
                "crate": m.group(1),
                "model": m.group(2),
                "wb_impl": m.group(3),
                "stub_impl": "0",
                "moto_impl": m.group(4),
                "floci_impl": m.group(5),
                "kumo_impl": m.group(6),
                "total": m.group(7),
                "wb_pct": m.group(8),
                "stub_pct": "0.0%",
                "moto_pct": m.group(9),
                "floci_pct": m.group(10),
                "kumo_pct": m.group(11),
            })

    # Parse summary lines
    wb_summary = ""
    stub_summary = ""
    moto_summary = ""
    floci_summary = ""
    kumo_summary = ""
    wb_match = re.search(
        r'\*\*winterbaume [0-9a-f]\S+: (.+?)\*\*', content
    ) or re.search(r'\*\*winterbaume: (.+?)\*\*', content)
    stub_match = re.search(
        r'\*\*winterbaume stubs: (.+?)\*\*', content, re.DOTALL
    )
    moto_match = re.search(r'\*\*moto [^:]+: (.+?)\*\*', content)
    floci_match = re.search(r'\*\*floci [^:]+: (.+?)\*\*', content)
    kumo_match = re.search(r'\*\*kumo [^:]+: (.+?)\*\*', content)
    if wb_match:
        wb_summary = wb_match.group(1)
    if stub_match:
        stub_summary = stub_match.group(1).strip().replace("\n", " ")
    if moto_match:
        moto_summary = moto_match.group(1)
    if floci_match:
        floci_summary = floci_match.group(1)
    if kumo_match:
        kumo_summary = kumo_match.group(1)

    return rows, wb_summary, stub_summary, moto_summary, floci_summary, kumo_summary, generated_on


def parse_detailed_coverage(
    coverage_path: Path,
) -> dict[str, list[dict[str, object]]]:
    """Parse the per-operation status lists from API_COVERAGE.md."""
    with open(coverage_path) as f:
        lines = f.readlines()

    details: dict[str, list[dict[str, object]]] = {}
    current_crate: str | None = None

    for raw_line in lines:
        line = raw_line.rstrip("\n")

        section_match = re.match(
            r"^### (winterbaume-\S+) \(\S+\) - W: \d+/\d+"
            r"(?:, S: \d+/\d+)?, M: \d+/\d+"
            r"(?:, F: \d+/\d+)?(?:, K: \d+/\d+)?$",
            line,
        )
        if section_match:
            current_crate = section_match.group(1)
            details[current_crate] = []
            continue

        if current_crate is None:
            continue

        # Detail line, new format: ``- W[ ] S[ ] M[ ] F[ ] K[ ] OperationName``
        new_op_match = re.match(
            r"^- W\[(x| )\] S\[(x| )\] M\[(x| )\] F\[(x| )\] K\[(x| )\] (\w+)$",
            line,
        )
        if new_op_match:
            wb, stub, moto, floci, kumo, name = new_op_match.groups()
            details[current_crate].append({
                "name": name,
                "winterbaume": wb == "x",
                "stub": stub == "x",
                "moto": moto == "x",
                "floci": floci == "x",
                "kumo": kumo == "x",
            })
            continue

        # Legacy detail line (no Stubs column).
        legacy_op_match = re.match(
            r"^- W\[(x| )\] M\[(x| )\](?: F\[(x| )\])?(?: K\[(x| )\])? (\w+)$",
            line,
        )
        if legacy_op_match:
            grps = legacy_op_match.groups()
            if len(grps) == 5:
                wb, moto, floci, kumo, name = grps
            else:
                wb, moto, name = grps[0], grps[1], grps[-1]
                floci = kumo = None
            details[current_crate].append({
                "name": name,
                "winterbaume": wb == "x",
                "stub": False,
                "moto": moto == "x",
                "floci": floci == "x" if floci is not None else False,
                "kumo": kumo == "x" if kumo is not None else False,
            })

    return details


def generate_services_section(
    rows: list[dict[str, str]],
    wb_summary: str,
    moto_summary: str,
    floci_summary: str = "",
    kumo_summary: str = "",
    stub_summary: str = "",
) -> str:
    """Generate the Supported Services markdown section."""
    lines = []
    lines.append("## Supported Services")
    lines.append("")
    lines.append(
        "The `Operations` column shows real, state-backed implementations. "
        "The `Stubs` column shows operations whose handler routes the "
        "request and returns an empty/default response without real "
        "behaviour ( either annotated with `// STUB[<category>]: ...` "
        "in `handlers.rs`, or detected heuristically as a body that "
        "produces a default response without consulting state ). The two "
        "columns are disjoint -- stubs are excluded from the operation "
        "count."
    )
    lines.append("")
    lines.append(
        "| Service | Crate | Protocol | Operations | Stubs | moto | floci | kumo |"
    )
    lines.append("|---|---|---|---|---|---|---|---|")

    # Build a unified list of (sort_key, row_md) entries.
    table_entries: list[tuple[str, str]] = []

    for row in rows:
        crate = row["crate"]
        display_name = CRATE_DISPLAY_INFO.get(
            crate, crate.removeprefix("winterbaume-")
        )
        protocol = lookup_protocol(crate)
        wb_ops = f"{row['wb_impl']}/{row['total']} ({row['wb_pct']})"
        stub_ops = (
            f"{row.get('stub_impl', '0')}/{row['total']} "
            f"({row.get('stub_pct', '0.0%')})"
        )
        moto_ops = f"{row['moto_impl']}/{row['total']} ({row['moto_pct']})"
        floci_ops = f"{row['floci_impl']}/{row['total']} ({row['floci_pct']})"
        kumo_ops = f"{row['kumo_impl']}/{row['total']} ({row['kumo_pct']})"
        row_md = (
            f"| [{display_name}](crates/{crate}/README.md) | `{crate}` | {protocol} "
            f"| {wb_ops} | {stub_ops} | {moto_ops} | {floci_ops} | {kumo_ops} |"
        )
        table_entries.append((display_name.lower(), row_md))

    table_entries.sort(key=lambda t: t[0])
    lines.extend(row_md for _, row_md in table_entries)

    lines.append("")
    lines.append(f"**winterbaume: {wb_summary}**")
    lines.append("")
    if stub_summary:
        lines.append(f"**winterbaume stubs: {stub_summary}**")
        lines.append("")
    lines.append(f"**moto: {moto_summary}**")
    lines.append("")
    if floci_summary:
        lines.append(f"**floci: {floci_summary}**")
        lines.append("")
    if kumo_summary:
        lines.append(f"**kumo: {kumo_summary}**")
        lines.append("")

    return "\n".join(lines)


def read_crate_description(crate_dir: Path, display_name: str) -> str:
    """Read the package description from a crate Cargo.toml."""
    cargo_toml = crate_dir / "Cargo.toml"
    if not cargo_toml.exists():
        return f"{display_name} service implementation for winterbaume."

    with open(cargo_toml) as f:
        cargo_text = f.read()

    match = re.search(r'^description\s*=\s*"(.+)"$', cargo_text, re.MULTILINE)
    if match:
        description = match.group(1).strip()
        if description.endswith("."):
            return description
        return f"{description}."

    return f"{display_name} service implementation for winterbaume."


def relative_link(from_dir: Path, target_path: Path) -> str:
    """Return a POSIX relative link path."""
    return Path(os.path.relpath(target_path, start=from_dir)).as_posix()


def extract_dossier_section(dossier_path: Path, heading: str) -> str | None:
    """Return a top-level dossier section body, including its heading."""
    if not dossier_path.exists():
        return None

    text = dossier_path.read_text(encoding="utf-8")
    pattern = re.compile(
        rf"(^## {re.escape(heading)}\n.*?)(?=^## |\Z)",
        re.MULTILINE | re.DOTALL,
    )
    match = pattern.search(text)
    if not match:
        return None

    section = match.group(1).strip()
    return section or None


def read_transcribed_dossier_sections(root: Path, model_slug: str) -> list[str]:
    """Read selected service dossier sections for crate README transcription."""
    if not model_slug:
        return []

    dossier_path = root / ".agents" / "docs" / "services" / f"{model_slug}.md"
    sections: list[str] = []
    for heading in DOSSIER_TRANSCRIBED_SECTIONS:
        section = extract_dossier_section(dossier_path, heading)
        if section:
            sections.append(section)
    return sections


def generate_service_readme(
    crate_dir: Path,
    row: dict[str, str],
    generated_on: str,
    root: Path,
    operation_details: list[dict[str, object]],
) -> str:
    """Generate a service crate README.md."""
    crate = row["crate"]
    display_name = CRATE_DISPLAY_INFO.get(
        crate, crate.removeprefix("winterbaume-")
    )
    protocol = lookup_protocol(crate)
    description = read_crate_description(crate_dir, display_name)
    implemented_ops = [
        str(operation["name"])
        for operation in operation_details
        if bool(operation["winterbaume"])
    ]
    stub_ops = [
        str(operation["name"])
        for operation in operation_details
        if bool(operation.get("stub"))
    ]
    missing_ops = [
        operation for operation in operation_details
        if not bool(operation["winterbaume"]) and not bool(operation.get("stub"))
    ]
    root_readme_link = relative_link(crate_dir, root / "README.md")
    coverage_link = relative_link(crate_dir, root / ".agents" / "docs" / "API_COVERAGE.md")
    generator_link = relative_link(
        crate_dir,
        root / ".agents" / "skills" / "update-readme" / "scripts" / "update_readme.py",
    )

    cli_example = CRATE_CLI_EXAMPLES.get(crate)

    lines = [
        "<!-- BEGIN AUTO (generated by .agents/skills/update-readme/scripts/update_readme.py) -->",
        "",
        f"# {crate}",
        "",
        description,
        "",
        (
            f"This crate is part of the [winterbaume](https://github.com/moriyoshi/winterbaume) "
            f"workspace — a suite of in-process AWS service mocks for Rust. "
            f"Use the umbrella [`winterbaume`](https://crates.io/crates/winterbaume) crate "
            f"to pull in all services at once, or depend on this crate directly for "
            f"{display_name} only."
        ),
        "",
        (
            "Winterbäume is not affiliated with, endorsed by, or sponsored by Amazon "
            "Web Services, Inc. or Amazon.com, Inc. Amazon Web Services, AWS, and "
            "related marks are trademarks of Amazon.com, Inc. or its affiliates. "
            "All other trademarks are the property of their respective owners."
        ),
        "",
    ]

    crate_note = _CRATE_NOTES.get(crate)
    if crate_note:
        lines += [crate_note, ""]

    lines += [
        "## Coverage",
        "",
        "| Metric | Value |",
        "|---|---|",
        f"| Service | {display_name} |",
        f"| AWS model | `{row['model']}` |",
        f"| Protocol | {protocol} |",
        f"| winterbaume coverage | {row['wb_impl']}/{row['total']} operations ({row['wb_pct']}) |",
        f"| stubs (routed, returns empty/default) | {row.get('stub_impl', '0')}/{row['total']} operations ({row.get('stub_pct', '0.0%')}) |",
        f"| moto coverage | {row['moto_impl']}/{row['total']} operations ({row['moto_pct']}) |",
        f"| floci coverage | {row.get('floci_impl', '?')}/{row['total']} operations ({row.get('floci_pct', '?')}) |",
        f"| kumo coverage | {row.get('kumo_impl', '?')}/{row['total']} operations ({row.get('kumo_pct', '?')}) |",
        f"| Coverage report date | {generated_on or 'unknown'} |",
        "",
        (
            f"Coverage is generated from "
            f"[`.agents/docs/API_COVERAGE.md`]({coverage_link}) by "
            f"[`update_readme.py`]({generator_link}). The `winterbaume` "
            f"row counts only operations with real, state-backed logic; "
            f"`stubs` counts handlers that route the request and return "
            f"an empty/default response without real behaviour. "
            f"Operation-count coverage is a prioritisation signal, not a "
            f"behavioural guarantee."
        ),
        "",
        (
            f"See the workspace [`README.md`]({root_readme_link}) for setup, usage, "
            f"and the full cross-service coverage table."
        ),
        "",
        "## Server-mode usage",
        "",
        f"Start `winterbaume-server` and point the AWS CLI at it:",
        "",
        "```sh",
        "cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555",
        "```",
        "",
        "```sh",
        "export AWS_ENDPOINT_URL=http://localhost:5555",
    ]

    if cli_example:
        lines.append(cli_example[1])
    else:
        lines.append(f"aws {crate.removeprefix('winterbaume-')} help")

    lines += [
        "```",
        "",
    ]

    transcribed_sections = read_transcribed_dossier_sections(root, row["model"])
    for section in transcribed_sections:
        lines.extend([section, ""])

    # Embed the example source if one exists in examples/<slug>.rs
    slug = crate.removeprefix("winterbaume-")
    example_path = root / "examples" / f"{slug}.rs"
    # Some crates have an example under an alternative name
    if not example_path.exists():
        alt = _EXAMPLE_SLUG_ALIASES.get(crate)
        if alt:
            example_path = root / "examples" / f"{alt}.rs"
    if example_path.exists():
        example_src = example_path.read_text(encoding="utf-8")
        # Strip the doc-comment header (//! lines) at the top
        stripped_lines: list[str] = []
        in_header = True
        for src_line in example_src.splitlines():
            if in_header and (src_line.startswith("//!") or src_line == ""):
                continue
            in_header = False
            stripped_lines.append(src_line)
        example_body = "\n".join(stripped_lines)

        lines += [
            "## Example",
            "",
            "```rust",
            example_body,
            "```",
            "",
        ]

    lines += [
        f"## Implemented APIs ({len(implemented_ops)})",
        "",
    ]

    if implemented_ops:
        lines.extend(f"- `{operation}`" for operation in implemented_ops)
    else:
        lines.append("No modeled operations are currently detected as implemented.")

    lines.append("")

    if stub_ops:
        lines.extend([
            (
                f"<details><summary>Stubbed APIs ({len(stub_ops)}) "
                "&mdash; routed but return an empty/default response"
                "</summary>"
            ),
            "",
        ])
        lines.extend(f"- `{operation}`" for operation in stub_ops)
        lines.extend([
            "",
            "</details>",
            "",
        ])

    if missing_ops:
        lines.extend([
            f"<details><summary>Not yet implemented APIs ({len(missing_ops)})</summary>",
            "",
        ])
        for operation in missing_ops:
            notes = []
            if bool(operation.get("moto")):
                notes.append("moto")
            if bool(operation.get("floci")):
                notes.append("floci")
            if bool(operation.get("kumo")):
                notes.append("kumo")
            note_str = f" (implemented by {', '.join(notes)})" if notes else ""
            lines.append(f"- `{operation['name']}`{note_str}")
        lines.extend([
            "",
            "</details>",
            "",
        ])

    lines.append("<!-- END AUTO -->")

    return "\n".join(lines)


_AUTO_BEGIN_RE = re.compile(r"^<!-- BEGIN AUTO\b.*-->$")
_AUTO_END_RE = re.compile(r"^<!-- END AUTO -->$")


def _splice_auto_block(existing: str, generated: str) -> str:
    """Replace the AUTO block in *existing* with *generated*, preserving the rest.

    If the existing content has no AUTO markers yet, return *generated* as-is
    (first-time generation).
    """
    old_lines = existing.splitlines(keepends=True)

    # Find the marker boundaries in the existing file.
    begin_idx: int | None = None
    end_idx: int | None = None
    for i, line in enumerate(old_lines):
        if begin_idx is None and _AUTO_BEGIN_RE.match(line.rstrip("\n")):
            begin_idx = i
        elif begin_idx is not None and _AUTO_END_RE.match(line.rstrip("\n")):
            end_idx = i
            break

    if begin_idx is None or end_idx is None:
        # No markers found — first-time generation; emit generated content only.
        return generated

    # Preserve everything before BEGIN AUTO and after END AUTO.
    before = old_lines[:begin_idx]
    after = old_lines[end_idx + 1:]

    # Ensure generated content ends with a newline for clean splicing.
    gen_block = generated if generated.endswith("\n") else generated + "\n"

    return "".join(before) + gen_block + "".join(after)


def update_service_readmes(
    root: Path,
    crates_dir: Path,
    rows: list[dict[str, str]],
    generated_on: str,
    detailed_coverage: dict[str, list[dict[str, object]]],
) -> int:
    """Rewrite service crate README.md files from coverage data."""
    updated_count = 0

    for row in rows:
        crate = row["crate"]
        crate_dir = crates_dir / crate
        if not crate_dir.exists():
            print(f"Warning: crate directory not found for {crate}", file=sys.stderr)
            continue

        crate_readme = crate_dir / "README.md"
        generated = generate_service_readme(
            crate_dir,
            row,
            generated_on,
            root,
            detailed_coverage.get(crate, []),
        )

        if crate_readme.exists():
            existing = crate_readme.read_text(encoding="utf-8")
            final = _splice_auto_block(existing, generated)
        else:
            final = generated

        crate_readme.write_text(final, encoding="utf-8")
        updated_count += 1

    return updated_count


def update_readme(
    readme_path: Path,
    rows: list[dict[str, str]],
    wb_summary: str,
    moto_summary: str,
    floci_summary: str = "",
    kumo_summary: str = "",
    stub_summary: str = "",
) -> None:
    """Update the Supported Services section in README.md."""
    with open(readme_path) as f:
        readme = f.read()

    new_section = generate_services_section(
        rows,
        wb_summary,
        moto_summary,
        floci_summary,
        kumo_summary,
        stub_summary=stub_summary,
    )

    # Try to consume both "## Supported Services" and the old "### Stub Services"
    # subsection in one replacement so we don't leave an orphaned subsection.
    pattern = r'## Supported Services\n.*?(?=\n## [^#])'
    if not re.search(pattern, readme, re.DOTALL):
        print(
            "Error: Could not find '## Supported Services' section in README.md",
            file=sys.stderr,
        )
        sys.exit(1)

    updated = re.sub(pattern, new_section, readme, count=1, flags=re.DOTALL)

    with open(readme_path, "w") as f:
        f.write(updated)

    print(f"Updated {readme_path}")
    print(f"  {len(rows)} services, winterbaume: {wb_summary}")


def _parse_summary_parts(
    wb_summary: str,
    moto_summary: str,
) -> tuple[str, str, str, str, str]:
    """Extract (wb_impl, total, service_count, wb_pct_decimal, moto_pct_decimal)."""
    wb_match = re.search(
        r'(\d[\d,]*)\s*/\s*(\d[\d,]*) operations across (\d+) services \((\d+\.\d+)%\)',
        wb_summary,
    )
    moto_match = re.search(r'\((\d+\.\d+)%\)', moto_summary)
    wb_impl = wb_match.group(1) if wb_match else "?"
    total = wb_match.group(2) if wb_match else "?"
    service_count = wb_match.group(3) if wb_match else "?"
    wb_pct = wb_match.group(4) if wb_match else "?"
    moto_pct = moto_match.group(1) if moto_match else "?"
    return wb_impl, total, service_count, wb_pct, moto_pct


def generate_docs_reference_content(
    rows: list[dict[str, str]],
    wb_summary: str,
    moto_summary: str,
    floci_summary: str = "",
    kumo_summary: str = "",
    stub_summary: str = "",
) -> str:
    """Generate the complete docs/reference/services.md content."""
    wb_impl, total, service_count, wb_pct, moto_pct = _parse_summary_parts(wb_summary, moto_summary)

    lines = [
        "# Service Coverage",
        "",
        (
            f"Winterbaume implements **{wb_impl} of {total} operations across"
            f" {service_count} AWS services ({wb_pct}%)** with real,"
            f" state-backed logic, compared to moto's {moto_pct}% across the"
            f" same service set. A further set of operations are routed but"
            f" return an empty/default response without real behaviour"
            f" ( see the `Stubs` column )."
        ),
        "",
        "| Service | Crate | Protocol | Operations | Stubs | moto | floci | kumo |",
        "|---|---|---|---|---|---|---|---|",
    ]

    table_entries: list[tuple[str, str]] = []

    for row in rows:
        crate = row["crate"]
        display_name = CRATE_DISPLAY_INFO.get(
            crate, crate.removeprefix("winterbaume-")
        )
        protocol = lookup_protocol(crate)
        wb_ops = f"{row['wb_impl']}/{row['total']} ({row['wb_pct']})"
        stub_ops = (
            f"{row.get('stub_impl', '0')}/{row['total']} "
            f"({row.get('stub_pct', '0.0%')})"
        )
        moto_ops = f"{row['moto_impl']}/{row['total']} ({row['moto_pct']})"
        floci_ops = f"{row.get('floci_impl', '?')}/{row['total']} ({row.get('floci_pct', '?')})"
        kumo_ops = f"{row.get('kumo_impl', '?')}/{row['total']} ({row.get('kumo_pct', '?')})"
        slug = crate.removeprefix("winterbaume-")
        row_md = (
            f"| [{display_name}](/services/{slug}) | `{crate}` | {protocol} "
            f"| {wb_ops} | {stub_ops} | {moto_ops} | {floci_ops} | {kumo_ops} |"
        )
        table_entries.append((display_name.lower(), row_md))

    table_entries.sort(key=lambda t: t[0])
    lines.extend(row_md for _, row_md in table_entries)

    lines.extend([
        "",
        f"**winterbaume: {wb_impl} / {total} operations across {service_count} services ({wb_pct}%)**",
        "",
    ])
    if stub_summary:
        lines.extend([f"**winterbaume stubs: {stub_summary}**", ""])
    lines.extend([
        f"**moto: {moto_summary}**",
        "",
    ])
    if floci_summary:
        lines.extend([f"**floci: {floci_summary}**", ""])
    if kumo_summary:
        lines.extend([f"**kumo: {kumo_summary}**", ""])

    lines.extend([
        "---",
        "",
        "See also: [Terraform Converter Coverage](/reference/terraform)"
        " \u2014 267 Terraform resource types with inject/extract field coverage"
        " against the official AWS provider schema.",
    ])

    return "\n".join(lines)


def update_docs_reference(
    docs_dir: Path,
    rows: list[dict[str, str]],
    wb_summary: str,
    moto_summary: str,
    floci_summary: str = "",
    kumo_summary: str = "",
    stub_summary: str = "",
) -> None:
    """Rewrite docs/reference/services.md with current coverage data."""
    services_path = docs_dir / "reference" / "services.md"
    if not services_path.parent.exists():
        print(f"Warning: {services_path.parent} does not exist; skipping docs reference update", file=sys.stderr)
        return
    content = generate_docs_reference_content(
        rows,
        wb_summary,
        moto_summary,
        floci_summary,
        kumo_summary,
        stub_summary=stub_summary,
    )
    services_path.write_text(content, encoding="utf-8")
    print(f"Updated {services_path}")


def update_docs_index(
    docs_dir: Path,
    wb_summary: str,
    moto_summary: str,
    stub_summary: str = "",
) -> None:
    """Update coverage statistics in docs/index.md."""
    index_path = docs_dir / "index.md"
    if not index_path.exists():
        print(f"Warning: {index_path} does not exist; skipping", file=sys.stderr)
        return

    wb_impl, total, service_count, wb_pct, moto_pct = _parse_summary_parts(wb_summary, moto_summary)
    wb_pct_int = str(round(float(wb_pct))) if wb_pct != "?" else wb_pct
    stub_match = re.search(r'(\d[\d,]*)\s*/\s*\d[\d,]* operations.*?\((\d+\.\d+)%\)', stub_summary)
    stub_impl = stub_match.group(1) if stub_match else "?"
    stub_pct = stub_match.group(2) if stub_match else "?"

    with open(index_path) as f:
        content = f.read()

    updated = re.sub(
        r'(- title: )\d+% API Coverage',
        f'\\g<1>{wb_pct_int}% API Coverage',
        content,
    )
    if stub_summary:
        details = (
            f"{wb_impl} of {total} operations across {service_count} AWS services"
            f" have real, state-backed behaviour, surpassing moto's {moto_pct}%."
            f" A further {stub_impl} operations ({stub_pct}%) are stubs that"
            " route the request and return an empty/default response &mdash;"
            " clearly broken out per service. Implemented surfaces include S3,"
            " DynamoDB, SQS, Lambda, KMS, ECS, EKS, IAM, and many more."
        )
        updated = re.sub(
            r"(    details: ).*operations across \d+ AWS services.*many more\.",
            f"\\g<1>{details}",
            updated,
        )
    else:
        updated = re.sub(
            r"(    details: )\d[\d, ]* of \d[\d, ]* operations across \d+ AWS services, surpassing moto's \d+\.\d+%\.",
            f"\\g<1>{wb_impl} of {total} operations across {service_count} AWS services, surpassing moto's {moto_pct}%.",
            updated,
        )

    if updated != content:
        with open(index_path, "w") as f:
            f.write(updated)
        print(f"Updated {index_path}")
    else:
        print(f"Note: no changes needed in {index_path}")


_DOCS_SERVICE_SKIP: frozenset[str] = frozenset([
    "winterbaume-core",
    "winterbaume-server",
    "winterbaume-stubs",
    "winterbaume-terraform",
    "winterbaume-tfstate",
    "winterbaume-partiql",
    "winterbaume-sqlengine-duckdb",
    "winterbaume-sqs-redis",
    "winterbaume-dynamodb-redis",
    "winterbaume-e2e-tests",
    "winterbaume-bedrock-flow-parser",
    "winterbaume-bedrock-flow-validator",
    "winterbaume-iam-rule-eval",
    "winterbaume-sfn-asl-eval",
    "winterbaume-wafv2-wcu-calculator",
    "winterbaume-wafv2-webacl-rule-parser",
    "winterbaume-ec2-generated",
])


def generate_docs_service_pages(crates_dir: Path, docs_dir: Path) -> int:
    """Copy crate READMEs into docs/services/<slug>.md with crate-only prose removed.

    Mirrors the logic of docs/scripts/generate-service-pages.mjs so that Node.js
    is not required at runtime.

    Returns the number of pages written.
    """
    services_dir = docs_dir / "services"
    services_dir.mkdir(parents=True, exist_ok=True)

    crates = sorted(
        d for d in crates_dir.iterdir()
        if d.is_dir() and d.name.startswith("winterbaume-") and d.name not in _DOCS_SERVICE_SKIP
    )

    count = 0
    written: set[str] = set()
    for crate_dir in crates:
        readme = crate_dir / "README.md"
        if not readme.exists():
            continue

        content = readme.read_text(encoding="utf-8")
        content = re.sub(r"^<!--.*?-->\n\n?", "", content, flags=re.DOTALL)
        content = re.sub(r"^<!-- END AUTO -->\n?", "", content, flags=re.MULTILINE)
        content = re.sub(r"^This crate is part of .*\n\n?", "", content, flags=re.MULTILINE)
        content = re.sub(
            r"^Winterbäume is not affiliated with, endorsed by, or sponsored by Amazon .*\n\n?",
            "",
            content,
            flags=re.MULTILINE,
        )
        content = re.sub(r"^Coverage is generated from \[.*?\n\n?", "", content, flags=re.MULTILINE)
        content = re.sub(r"^See the workspace \[.*?\n\n?", "", content, flags=re.MULTILINE)
        slug = crate_dir.name.removeprefix("winterbaume-")
        out_path = services_dir / f"{slug}.md"
        out_path.write_text(content.rstrip() + "\n", encoding="utf-8")
        written.add(out_path.name)
        count += 1

    for existing in services_dir.glob("*.md"):
        if existing.name not in written:
            existing.unlink()

    return count


def main() -> None:
    parser = argparse.ArgumentParser(
        description="Update workspace and service README.md files with API coverage data"
    )
    parser.add_argument(
        "--coverage",
        type=Path,
        default=None,
        help="Path to API_COVERAGE.md (default: .agents/docs/API_COVERAGE.md)",
    )
    parser.add_argument(
        "--readme",
        type=Path,
        default=None,
        help="Path to README.md (default: README.md)",
    )
    parser.add_argument(
        "--crates-dir",
        type=Path,
        default=None,
        help="Path to crates directory (default: crates)",
    )
    parser.add_argument(
        "--docs-dir",
        type=Path,
        default=None,
        help="Path to docs directory (default: docs); pass --no-docs to skip",
    )
    parser.add_argument(
        "--no-docs",
        action="store_true",
        help="Skip updating the docs/ directory",
    )
    args = parser.parse_args()

    root = find_project_root()
    coverage_path = args.coverage or (root / ".agents" / "docs" / "API_COVERAGE.md")
    readme_path = args.readme or (root / "README.md")
    crates_dir = args.crates_dir or (root / "crates")
    docs_dir = args.docs_dir or (root / "docs")
    rows, wb_summary, stub_summary, moto_summary, floci_summary, kumo_summary, generated_on = parse_coverage_overview(coverage_path)
    detailed_coverage = parse_detailed_coverage(coverage_path)

    if not rows:
        print("Error: No coverage data found in the report", file=sys.stderr)
        sys.exit(1)

    update_readme(
        readme_path,
        rows,
        wb_summary,
        moto_summary,
        floci_summary,
        kumo_summary,
        stub_summary=stub_summary,
    )
    updated_service_readmes = update_service_readmes(
        root,
        crates_dir,
        rows,
        generated_on,
        detailed_coverage,
    )
    print(f"Updated {updated_service_readmes} service crate README files")

    if not args.no_docs and docs_dir.exists():
        update_docs_reference(
            docs_dir,
            rows,
            wb_summary,
            moto_summary,
            floci_summary,
            kumo_summary,
            stub_summary=stub_summary,
        )
        update_docs_index(docs_dir, wb_summary, moto_summary, stub_summary)
        n_pages = generate_docs_service_pages(crates_dir, docs_dir)
        print(f"Generated {n_pages} docs/services/ pages")
    elif not args.no_docs:
        print(f"Note: docs/ directory not found at {docs_dir}; skipping docs updates")


if __name__ == "__main__":
    main()
