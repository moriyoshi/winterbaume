#!/usr/bin/env python3
"""Generate AWS API coverage report for winterbaume.

Compares implemented operations in winterbaume handler crates against
the official AWS Smithy API models in vendor/api-models-aws/, with
moto, floci, and kumo coverage shown alongside for reference.
Moto coverage is fetched from GitHub and cached locally (like floci
and kumo) unless vendor/moto is present on disk.

Usage:
    python3 .agents/skills/api-coverage/scripts/generate_coverage.py [--output PATH]

Default output: .agents/docs/API_COVERAGE.md
"""

import argparse
import json
import os
import re
import subprocess
import sys
import urllib.error
import urllib.request
from datetime import date
from pathlib import Path

# Mapping from winterbaume crate directory name to api-models-aws model directory name.
CRATE_TO_MODEL = {
    "winterbaume-account": "account",
    "winterbaume-acm": "acm",
    "winterbaume-appconfig": "appconfig",
    "winterbaume-athena": "athena",
    "winterbaume-backup": "backup",
    "winterbaume-budgets": "budgets",
    "winterbaume-cloudfront": "cloudfront",
    "winterbaume-cloudtrail": "cloudtrail",
    "winterbaume-cloudwatch": "cloudwatch",
    "winterbaume-codebuild": "codebuild",
    "winterbaume-codecommit": "codecommit",
    "winterbaume-codedeploy": "codedeploy",
    "winterbaume-codepipeline": "codepipeline",
    "winterbaume-cognitoidentity": "cognito-identity",
    "winterbaume-cognitoidentityprovider": "cognito-identity-provider",
    "winterbaume-config": "config-service",
    "winterbaume-dax": "dax",
    "winterbaume-dynamodb": "dynamodb",
    "winterbaume-ec2": "ec2",
    "winterbaume-ec2instanceconnect": "ec2-instance-connect",
    "winterbaume-ecr": "ecr",
    "winterbaume-ecs": "ecs",
    "winterbaume-eks": "eks",
    "winterbaume-elasticloadbalancing": "elastic-load-balancing",
    "winterbaume-elasticloadbalancingv2": "elastic-load-balancing-v2",
    "winterbaume-eventbridge": "eventbridge",
    "winterbaume-firehose": "firehose",
    "winterbaume-glacier": "glacier",
    "winterbaume-guardduty": "guardduty",
    "winterbaume-iam": "iam",
    "winterbaume-ivs": "ivs",
    "winterbaume-kinesis": "kinesis",
    "winterbaume-kms": "kms",
    "winterbaume-lambda": "lambda",
    "winterbaume-cloudwatchlogs": "cloudwatch-logs",
    "winterbaume-organizations": "organizations",
    "winterbaume-pipes": "pipes",
    "winterbaume-polly": "polly",
    "winterbaume-rdsdata": "rds-data",
    "winterbaume-resourcegroups": "resource-groups",
    "winterbaume-route53": "route-53",
    "winterbaume-s3": "s3",
    "winterbaume-scheduler": "scheduler",
    "winterbaume-secretsmanager": "secrets-manager",
    "winterbaume-sesv2": "sesv2",
    "winterbaume-signer": "signer",
    "winterbaume-sns": "sns",
    "winterbaume-sqs": "sqs",
    "winterbaume-ssm": "ssm",
    "winterbaume-sfn": "sfn",
    "winterbaume-sts": "sts",
    "winterbaume-wafv2": "wafv2",
    "winterbaume-xray": "xray",
    "winterbaume-securityhub": "securityhub",
    "winterbaume-inspector2": "inspector2",
    "winterbaume-rekognition": "rekognition",
    "winterbaume-synthetics": "synthetics",
    "winterbaume-shield": "shield",
    "winterbaume-servicequotas": "service-quotas",
    "winterbaume-ram": "ram",
    "winterbaume-macie2": "macie2",
    "winterbaume-comprehend": "comprehend",
    "winterbaume-transcribe": "transcribe",
    "winterbaume-textract": "textract",
    "winterbaume-forecast": "forecast",
    "winterbaume-personalize": "personalize",
    "winterbaume-route53resolver": "route53resolver",
    "winterbaume-acmpca": "acm-pca",
    "winterbaume-directconnect": "direct-connect",
    "winterbaume-directory": "directory-service",
    "winterbaume-mq": "mq",
    "winterbaume-transfer": "transfer",
    "winterbaume-datasync": "datasync",
    "winterbaume-servicediscovery": "servicediscovery",
    "winterbaume-identitystore": "identitystore",
    "winterbaume-efs": "efs",
    "winterbaume-support": "support",
    "winterbaume-batch": "batch",
    "winterbaume-glue": "glue",
    "winterbaume-fsx": "fsx",
    "winterbaume-memorydb": "memorydb",
    "winterbaume-ebs": "ebs",
    "winterbaume-networkfirewall": "network-firewall",
    "winterbaume-sagemaker": "sagemaker",
    "winterbaume-appsync": "appsync",
    "winterbaume-iot": "iot",
    "winterbaume-kafka": "kafka",
    "winterbaume-lexmodelsv2": "lex-models-v2",
    "winterbaume-lakeformation": "lakeformation",
    "winterbaume-amp": "amp",
    "winterbaume-opensearch": "opensearch",
    "winterbaume-costexplorer": "cost-explorer",
    "winterbaume-bedrock": "bedrock",
    "winterbaume-cloudhsmv2": "cloudhsm-v2",
    "winterbaume-dynamodbstreams": "dynamodb-streams",
    "winterbaume-dsql": "dsql",
    "winterbaume-databrew": "databrew",
    "winterbaume-emrserverless": "emr-serverless",
    "winterbaume-workspaces": "workspaces",
    "winterbaume-emrcontainers": "emr-containers",
    "winterbaume-bedrockagent": "bedrock-agent",
    "winterbaume-connect": "connect",
    "winterbaume-kinesisanalyticsv2": "kinesis-analytics-v2",
    "winterbaume-appmesh": "app-mesh",
    "winterbaume-greengrass": "greengrass",
    "winterbaume-networkmanager": "networkmanager",
    "winterbaume-pinpoint": "pinpoint",
    "winterbaume-swf": "swf",
    "winterbaume-redshiftdata": "redshift-data",
    "winterbaume-mediaconnect": "mediaconnect",
    "winterbaume-medialive": "medialive",
    "winterbaume-mediapackage": "mediapackage",
    "winterbaume-managedblockchain": "managedblockchain",
    "winterbaume-resiliencehub": "resiliencehub",
    "winterbaume-quicksight": "quicksight",
    "winterbaume-vpclattice": "vpc-lattice",
    "winterbaume-workspacesweb": "workspaces-web",
    "winterbaume-mediastore": "mediastore",
    "winterbaume-route53domains": "route-53-domains",
    "winterbaume-mediapackagev2": "mediapackagev2",
    "winterbaume-osis": "osis",
    "winterbaume-servicecatalog": "service-catalog",
    "winterbaume-timestreamwrite": "timestream-write",
    "winterbaume-connectcampaigns": "connectcampaigns",
    "winterbaume-kinesisvideo": "kinesis-video",
    "winterbaume-sso": "sso",
    "winterbaume-ssoadmin": "sso-admin",
    "winterbaume-servicecatalogappregistry": "service-catalog-appregistry",
    "winterbaume-timestreamquery": "timestream-query",
    "winterbaume-timestreaminfluxdb": "timestream-influxdb",
    "winterbaume-marketplacemetering": "marketplace-metering",
    "winterbaume-sagemakermetrics": "sagemaker-metrics",
    "winterbaume-sagemakerruntime": "sagemaker-runtime",
    "winterbaume-iotdataplane": "iot-data-plane",
    "winterbaume-kinesisvideoarchivedmedia": "kinesis-video-archived-media",
    "winterbaume-resourcegroupstagging": "resource-groups-tagging-api",
    "winterbaume-applicationautoscaling": "application-auto-scaling",
    "winterbaume-autoscaling": "auto-scaling",
    "winterbaume-mediastoredata": "mediastore-data",
    "winterbaume-simpledbv2": "simpledbv2",
    "winterbaume-datapipeline": "data-pipeline",
    "winterbaume-apigateway": "api-gateway",
    "winterbaume-apigatewaymanagement": "apigatewaymanagementapi",
    "winterbaume-apigatewayv2": "apigatewayv2",
    "winterbaume-s3control": "s3-control",
    "winterbaume-s3files": "s3files",
    "winterbaume-s3vectors": "s3vectors",
    "winterbaume-s3tables": "s3tables",
    "winterbaume-cloudformation": "cloudformation",
    "winterbaume-elasticache": "elasticache",
    "winterbaume-emr": "emr",
    "winterbaume-rds": "rds",
    "winterbaume-neptune": "neptune",
    "winterbaume-redshift": "redshift",
    "winterbaume-ses": "ses",
    "winterbaume-databasemigration": "database-migration-service",
    "winterbaume-panorama": "panorama",
    "winterbaume-elasticbeanstalk": "elastic-beanstalk",
    "winterbaume-amplify": "amplify",
    "winterbaume-apprunner": "apprunner",
    "winterbaume-auditmanager": "auditmanager",
    "winterbaume-chatbot": "chatbot",
    "winterbaume-codeartifact": "codeartifact",
    "winterbaume-opensearchserverless": "opensearchserverless",
    "winterbaume-clouddirectory": "clouddirectory",
    "winterbaume-accessanalyzer": "accessanalyzer",
    "winterbaume-aiops": "aiops",
    "winterbaume-amplifybackend": "amplifybackend",
    "winterbaume-amplifyuibuilder": "amplifyuibuilder",
    "winterbaume-appconfigdata": "appconfigdata",
    "winterbaume-appfabric": "appfabric",
    "winterbaume-appflow": "appflow",
    "winterbaume-appintegrations": "appintegrations",
    "winterbaume-applicationcostprofiler": "applicationcostprofiler",
    "winterbaume-applicationdiscoveryservice": "application-discovery-service",
    "winterbaume-applicationinsights": "application-insights",
    "winterbaume-applicationsignals": "application-signals",
    "winterbaume-arczonalshift": "arc-zonal-shift",
    "winterbaume-artifact": "artifact",
    "winterbaume-autoscalingplans": "auto-scaling-plans",
    "winterbaume-backupgateway": "backup-gateway",
    "winterbaume-backupsearch": "backupsearch",
    "winterbaume-bcmdashboards": "bcm-dashboards",
    "winterbaume-bcmdataexports": "bcm-data-exports",
    "winterbaume-bcmrecommendedactions": "bcm-recommended-actions",
    "winterbaume-cloud9": "cloud9",
    "winterbaume-cloudcontrol": "cloudcontrol",
    "winterbaume-cloudfrontkeyvaluestore": "cloudfront-keyvaluestore",
    "winterbaume-cloudtraildata": "cloudtrail-data",
    "winterbaume-dlm": "dlm",
    "winterbaume-fis": "fis",
    "winterbaume-freetier": "freetier",
    "winterbaume-keyspaces": "keyspaces",
    "winterbaume-outposts": "outposts",
    "winterbaume-pcaconnectorscep": "pca-connector-scep",
    "winterbaume-personalizeevents": "personalize-events",
    "winterbaume-personalizeruntime": "personalize-runtime",
    "winterbaume-pi": "pi",
    "winterbaume-pinpointsmsvoice": "pinpoint-sms-voice",
    "winterbaume-pricing": "pricing",
    "winterbaume-rbin": "rbin",
    "winterbaume-rolesanywhere": "rolesanywhere",
    "winterbaume-route53recoverycluster": "route53-recovery-cluster",
    "winterbaume-s3outposts": "s3outposts",
    "winterbaume-savingsplans": "savingsplans",
    "winterbaume-simspaceweaver": "simspaceweaver",
    "winterbaume-snowdevicemanagement": "snow-device-management",
    "winterbaume-ssmquicksetup": "ssm-quicksetup",
    "winterbaume-supportapp": "support-app",
    "winterbaume-taxsettings": "taxsettings",
    "winterbaume-trustedadvisor": "trustedadvisor",
    "winterbaume-cloudsearchdomain": "cloudsearch-domain",
    "winterbaume-billing": "billing",
    "winterbaume-braket": "braket",
    "winterbaume-chimesdkmeetings": "chime-sdk-meetings",
    "winterbaume-codegurureviewer": "codeguru-reviewer",
    "winterbaume-codegurusecurity": "codeguru-security",
    "winterbaume-codestarnotifications": "codestar-notifications",
    "winterbaume-cognitosync": "cognito-sync",
    "winterbaume-connectcontactlens": "connect-contact-lens",
    "winterbaume-connectparticipant": "connectparticipant",
    "winterbaume-controlcatalog": "controlcatalog",
    "winterbaume-costandusagereport": "cost-and-usage-report-service",
    "winterbaume-costoptimizationhub": "cost-optimization-hub",
}

# (winterbaume-stubs has been removed — all services now have dedicated crates.)
STUB_TO_MODEL: dict[str, tuple[str, str]] = {}


# Priority-ordered list of Smithy protocol traits. The first match wins, so
# `awsJson1.x` takes precedence over `awsQueryCompatible` on services that
# carry both. The mapping value is the human-readable label used in
# `update_readme.py`'s `CRATE_DISPLAY_INFO` table.
_PROTOCOL_TRAIT_PRIORITY: tuple[tuple[str, str], ...] = (
    ("aws.protocols#awsJson1_0", "awsJson1.0"),
    ("aws.protocols#awsJson1_1", "awsJson1.1"),
    ("aws.protocols#restJson1", "restJson1"),
    ("aws.protocols#restXml", "restXml"),
    ("aws.protocols#awsQuery", "awsQuery"),
    ("aws.protocols#ec2Query", "ec2Query"),
    ("smithy.protocols#rpcv2Cbor", "rpcv2Cbor"),
)

# Memoisation cache for derive_protocol so we read each Smithy JSON only once
# per process, even when the helper is invoked across many crates.
_DERIVED_PROTOCOL_CACHE: dict[str, str] = {}


def derive_protocol(model_name: str) -> str:
    """Return the Smithy protocol label for a vendored AWS model directory.

    Locates the Smithy JSON under
    ``vendor/api-models-aws/models/<model_name>/service/<version>/<model_name>-<version>.json``,
    finds the single shape with ``"type": "service"`` and returns the highest-
    priority protocol trait carried in its ``traits`` map (see
    ``_PROTOCOL_TRAIT_PRIORITY``). Falls back to ``"unknown"`` when the
    submodule is missing, the JSON cannot be parsed, or no recognised
    protocol trait is present.

    Results are memoised per ``model_name`` for the lifetime of the
    interpreter.
    """
    cached = _DERIVED_PROTOCOL_CACHE.get(model_name)
    if cached is not None:
        return cached

    root = find_project_root()
    service_dir = root / "vendor" / "api-models-aws" / "models" / model_name / "service"
    if not service_dir.exists():
        _DERIVED_PROTOCOL_CACHE[model_name] = "unknown"
        return "unknown"

    json_path: Path | None = None
    for version_dir in sorted(service_dir.iterdir()):
        if not version_dir.is_dir():
            continue
        # The convention is "<model_name>-<version>.json"; glob to be tolerant
        # of any hyphenation differences, then prefer that exact stem when
        # multiple JSON files happen to coexist.
        candidates = sorted(version_dir.glob("*.json"))
        if not candidates:
            continue
        preferred = [
            p for p in candidates if p.stem.startswith(f"{model_name}-")
        ]
        json_path = (preferred or candidates)[0]
        break

    if json_path is None:
        _DERIVED_PROTOCOL_CACHE[model_name] = "unknown"
        return "unknown"

    try:
        with open(json_path) as f:
            model = json.load(f)
    except (json.JSONDecodeError, OSError):
        _DERIVED_PROTOCOL_CACHE[model_name] = "unknown"
        return "unknown"

    service_traits: dict[str, object] | None = None
    for shape in model.get("shapes", {}).values():
        if shape.get("type") == "service":
            service_traits = shape.get("traits", {}) or {}
            break

    if service_traits is None:
        _DERIVED_PROTOCOL_CACHE[model_name] = "unknown"
        return "unknown"

    for trait_id, label in _PROTOCOL_TRAIT_PRIORITY:
        if trait_id in service_traits:
            _DERIVED_PROTOCOL_CACHE[model_name] = label
            return label

    _DERIVED_PROTOCOL_CACHE[model_name] = "unknown"
    return "unknown"

# Mapping from api-models-aws model directory name to moto service name
# (as it appears in moto's IMPLEMENTATION_COVERAGE.md).
# Only entries that differ from the model dir name need to be listed.
MODEL_TO_MOTO = {
    "api-gateway": "apigateway",
    "auto-scaling": "autoscaling",
    "cognito-identity-provider": "cognito-idp",
    "config-service": "config",
    "elastic-load-balancing": "elb",
    "elastic-load-balancing-v2": "elbv2",
    "eventbridge": "events",
    "cloudwatch-logs": "logs",
    "secrets-manager": "secretsmanager",
    "sfn": "stepfunctions",
    "rds-data": "rds-data",
    "resource-groups": "resource-groups",
    "route-53": "route53",
    "ec2-instance-connect": "ec2-instance-connect",
    "application-auto-scaling": "application-autoscaling",
    "emr-containers": "emr-containers",
    "emr-serverless": "emr-serverless",
    "sso-admin": "sso-admin",
    "redshift-data": "redshift-data",
    "ses": "ses",
    "database-migration-service": "dms",
    "lex-models-v2": "lexv2-models",
}

# Mapping from floci service document stem to api-models-aws model directory name.
# Only entries where the stem differs from the model dir name need to be listed.
FLOCI_STEM_TO_MODEL: dict[str, str] = {
    "apigateway": "api-gateway",
    "apigatewayv2": "apigatewayv2",
    "appconfigdata": "appconfig-data",
    "cloudwatchlogs": "cloudwatch-logs",
    "cognito": "cognito-identity-provider",
    "dynamodbstreams": "dynamodb-streams",
    "opensearch": "opensearch",
    "secretsmanager": "secrets-manager",
    "sesv2": "sesv2",
    "stepfunctions": "sfn",
}

# Mapping from kumo service directory name to api-models-aws model directory name.
# Only entries where the directory name differs from the model dir name need to be listed.
KUMO_DIR_TO_MODEL: dict[str, str] = {
    "apigateway": "api-gateway",
    "appmesh": "app-mesh",
    "ce": "cost-explorer",
    "cloudwatchlogs": "cloudwatch-logs",
    "codegurureviewer": "codeguru-reviewer",
    "cognito": "cognito-identity-provider",
    "configservice": "config-service",
    "ds": "directory-service",
    "elasticbeanstalk": "elastic-beanstalk",
    "elbv2": "elastic-load-balancing-v2",
    "emrserverless": "emr-serverless",
    "globalaccelerator": "global-accelerator",
    "route53": "route-53",
    "s3control": "s3-control",
    "secretsmanager": "secrets-manager",
    "servicequotas": "service-quotas",
}

# Mapping from terraform AWS resource types exercised in
# crates/winterbaume-e2e-tests/tests/terraform/ to the owning winterbaume
# service crate. Keep the more specific cloudwatch prefixes ahead of the
# general cloudwatch one.
TERRAFORM_RESOURCE_PREFIX_TO_CRATE: list[tuple[str, str]] = [
    ("aws_acm_", "winterbaume-acm"),
    ("aws_cloudfront_", "winterbaume-cloudfront"),
    ("aws_cloudwatch_log_", "winterbaume-cloudwatchlogs"),
    ("aws_cloudwatch_event_", "winterbaume-eventbridge"),
    ("aws_cloudwatch_", "winterbaume-cloudwatch"),
    ("aws_cognito_", "winterbaume-cognitoidentityprovider"),
    ("aws_dynamodb_", "winterbaume-dynamodb"),
    ("aws_ecr_", "winterbaume-ecr"),
    ("aws_ecs_", "winterbaume-ecs"),
    ("aws_efs_", "winterbaume-efs"),
    ("aws_eks_", "winterbaume-eks"),
    ("aws_iam_", "winterbaume-iam"),
    ("aws_kinesis_firehose_", "winterbaume-firehose"),
    ("aws_kinesis_", "winterbaume-kinesis"),
    ("aws_kms_", "winterbaume-kms"),
    ("aws_lambda_", "winterbaume-lambda"),
    ("aws_organizations_", "winterbaume-organizations"),
    ("aws_route53_", "winterbaume-route53"),
    ("aws_s3_", "winterbaume-s3"),
    ("aws_secretsmanager_", "winterbaume-secretsmanager"),
    ("aws_sfn_", "winterbaume-sfn"),
    ("aws_sns_", "winterbaume-sns"),
    ("aws_sqs_", "winterbaume-sqs"),
    ("aws_ssm_", "winterbaume-ssm"),
    ("aws_wafv2_", "winterbaume-wafv2"),
    ("aws_appconfig_", "winterbaume-appconfig"),
    ("aws_autoscaling_", "winterbaume-autoscaling"),
    ("aws_batch_", "winterbaume-batch"),
    ("aws_bedrockagent_", "winterbaume-bedrockagent"),
    ("aws_codecommit_", "winterbaume-codecommit"),
    ("aws_config_", "winterbaume-config"),
    ("aws_networkmanager_", "winterbaume-networkmanager"),
    ("aws_db_", "winterbaume-rds"),
    ("aws_rds_", "winterbaume-rds"),
    ("aws_scheduler_", "winterbaume-scheduler"),
    ("aws_transfer_", "winterbaume-transfer"),
    ("aws_vpclattice_", "winterbaume-vpclattice"),
    ("aws_workspaces_", "winterbaume-workspaces"),
    ("aws_apigatewayv2_", "winterbaume-apigatewayv2"),
]

TERRAFORM_RESOURCE_EXACT_TO_CRATE: dict[str, str] = {
    "aws_internet_gateway": "winterbaume-ec2",
    "aws_key_pair": "winterbaume-ec2",
    "aws_lb": "winterbaume-elasticloadbalancingv2",
    "aws_route_table": "winterbaume-ec2",
    "aws_route_table_association": "winterbaume-ec2",
    "aws_security_group": "winterbaume-ec2",
    "aws_subnet": "winterbaume-ec2",
    "aws_vpc": "winterbaume-ec2",
    "aws_launch_configuration": "winterbaume-autoscaling",
    "aws_ec2_fleet": "winterbaume-ec2",
}

SDK_METHOD_ACRONYMS: dict[str, str] = {
    "hls": "HLS", "dash": "DASH", "url": "URL", "uri": "URI",
    "vpc": "VPC", "dns": "DNS", "ip": "IP", "arn": "ARN",
    "iam": "IAM", "ssl": "SSL", "tls": "TLS", "acl": "ACL",
    "api": "API", "sdk": "SDK", "id": "Id", "lf": "LF",
    "db": "Db", "mfa": "MFA", "oidc": "OIDC", "saml": "SAML",
    "ec2": "EC2", "s3": "S3", "kms": "KMS", "sms": "SMS",
    "sns": "SNS", "sqs": "SQS", "ses": "SES", "sts": "STS",
    "elb": "ELB", "rds": "RDS", "emr": "EMR", "waf": "WAF",
    "ecs": "ECS", "eks": "EKS", "ecr": "ECR",
}

# Per-crate overrides for handler-derived operation names that don't match the
# model name due to AWS using mixed-case acronyms (e.g. "Vpc" not "VPC",
# "Ip" not "IP", "Dns" not "DNS") in some CloudFront operation names.
CRATE_OPERATION_NAME_OVERRIDES: dict[str, dict[str, str]] = {
    "winterbaume-cloudfront": {
        # vpc → VPC but AWS uses Vpc in CloudFront operation names
        "CreateVPCOrigin": "CreateVpcOrigin",
        "GetVPCOrigin": "GetVpcOrigin",
        "UpdateVPCOrigin": "UpdateVpcOrigin",
        "DeleteVPCOrigin": "DeleteVpcOrigin",
        "ListVPCOrigins": "ListVpcOrigins",
        "ListDistributionsByVPCOriginId": "ListDistributionsByVpcOriginId",
        # ip → IP but AWS uses Ip in CloudFront operation names
        "CreateAnycastIPList": "CreateAnycastIpList",
        "GetAnycastIPList": "GetAnycastIpList",
        "UpdateAnycastIPList": "UpdateAnycastIpList",
        "DeleteAnycastIPList": "DeleteAnycastIpList",
        "ListAnycastIPLists": "ListAnycastIpLists",
        "ListDistributionsByAnycastIPListId": "ListDistributionsByAnycastIpListId",
        # dns → DNS but AWS uses Dns in CloudFront operation names
        "VerifyDNSConfiguration": "VerifyDnsConfiguration",
    },
}


def find_project_root() -> Path:
    """Find the project root by looking for Cargo.toml."""
    d = Path(__file__).resolve().parent
    while d != d.parent:
        if (d / "Cargo.toml").exists() and (d / "crates").exists():
            return d
        d = d.parent
    print("Error: Could not find project root", file=sys.stderr)
    sys.exit(1)


def pascal_to_snake(name: str) -> str:
    """Convert PascalCase to snake_case (matching botocore's xform_name)."""
    # Insert underscore before uppercase letters that follow lowercase/digits
    s = re.sub(r'([a-z0-9])([A-Z])', r'\1_\2', name)
    # Handle consecutive uppercase letters (e.g., "SSHPublicKey" -> "ssh_public_key")
    s = re.sub(r'([A-Z]+)([A-Z][a-z])', r'\1_\2', s)
    return s.lower()


def map_terraform_resource_to_crate(resource_type: str) -> str | None:
    """Map a terraform AWS resource type to the owning winterbaume crate."""
    exact = TERRAFORM_RESOURCE_EXACT_TO_CRATE.get(resource_type)
    if exact is not None:
        return exact
    if resource_type.startswith("aws_lb_"):
        return "winterbaume-elasticloadbalancingv2"
    for prefix, crate_name in TERRAFORM_RESOURCE_PREFIX_TO_CRATE:
        if resource_type.startswith(prefix):
            return crate_name
    return None


def find_matching_brace(content: str, open_brace_index: int) -> int:
    """Find the closing brace for a Rust block, skipping strings/comments."""
    depth = 0
    i = open_brace_index
    block_comment_depth = 0
    state = "normal"
    raw_hashes = 0

    while i < len(content):
        ch = content[i]

        if state == "line_comment":
            if ch == "\n":
                state = "normal"
            i += 1
            continue

        if state == "block_comment":
            if content.startswith("/*", i):
                block_comment_depth += 1
                i += 2
                continue
            if content.startswith("*/", i):
                block_comment_depth -= 1
                i += 2
                if block_comment_depth == 0:
                    state = "normal"
                continue
            i += 1
            continue

        if state == "string":
            if ch == "\\":
                i += 2
                continue
            if ch == '"':
                state = "normal"
            i += 1
            continue

        if state == "char":
            if ch == "\\":
                i += 2
                continue
            if ch == "'":
                state = "normal"
            i += 1
            continue

        if state == "raw_string":
            if ch == '"' and content.startswith("#" * raw_hashes, i + 1):
                i += 1 + raw_hashes
                state = "normal"
                continue
            i += 1
            continue

        if content.startswith("//", i):
            state = "line_comment"
            i += 2
            continue
        if content.startswith("/*", i):
            state = "block_comment"
            block_comment_depth = 1
            i += 2
            continue

        if ch == "r":
            j = i + 1
            while j < len(content) and content[j] == "#":
                j += 1
            if j < len(content) and content[j] == '"':
                raw_hashes = j - (i + 1)
                state = "raw_string"
                i = j + 1
                continue

        if ch == '"':
            state = "string"
            i += 1
            continue

        if ch == "'":
            state = "char"
            i += 1
            continue

        if ch == "{":
            depth += 1
        elif ch == "}":
            depth -= 1
            if depth == 0:
                return i
        i += 1

    return -1


def iter_rust_async_functions(content: str) -> list[tuple[str, str]]:
    """Extract top-level async function bodies from a Rust source file."""
    functions: list[tuple[str, str]] = []
    for match in re.finditer(r'async\s+fn\s+(\w+)\s*\(', content):
        name = match.group(1)
        open_brace_index = content.find("{", match.end())
        if open_brace_index == -1:
            continue
        close_brace_index = find_matching_brace(content, open_brace_index)
        if close_brace_index == -1:
            continue
        functions.append((name, content[open_brace_index + 1:close_brace_index]))
    return functions


def classify_terraform_e2e_test(name: str, body: str) -> str:
    """Classify a terraform E2E test as success-oriented, diagnostic, or other."""
    if (
        name.startswith("debug_")
        or "smoke_test_terraform(" in body
        or "run_terraform_with_timeout(" in body
        or "Expected timeout" in body
        or re.search(r'assert!\(\s*!\s*ok\b', body)
    ):
        return "diagnostic"
    if re.search(r'assert!\(\s*result\.success\b', body):
        return "success"
    if re.search(r'assert!\(\s*ok\b', body):
        return "success"
    return "other"


def parse_terraform_e2e_coverage(root: Path) -> dict[str, dict[str, set[str]]]:
    """Parse terraform E2E coverage from crates/winterbaume-e2e-tests."""
    tests_dir = root / "crates" / "winterbaume-e2e-tests" / "tests" / "terraform"
    if not tests_dir.exists():
        return {}

    coverage: dict[str, dict[str, set[str]]] = {}

    for test_path in sorted(tests_dir.glob("*.rs")):
        if test_path.name in {"harness.rs", "main.rs"}:
            continue

        content = test_path.read_text()
        for test_name, body in iter_rust_async_functions(content):
            classification = classify_terraform_e2e_test(test_name, body)
            if classification == "other":
                continue

            resources_by_crate: dict[str, set[str]] = {}
            for resource_type in set(re.findall(r'resource\s+"(aws_[a-z0-9_]+)"', body)):
                crate_name = map_terraform_resource_to_crate(resource_type)
                if crate_name is None:
                    continue
                resources_by_crate.setdefault(crate_name, set()).add(resource_type)

            if not resources_by_crate:
                continue

            qualified_test_name = f"{test_path.stem}::{test_name}"
            for crate_name, resource_types in resources_by_crate.items():
                entry = coverage.setdefault(
                    crate_name,
                    {
                        "success_tests": set(),
                        "diagnostic_tests": set(),
                        "resource_types": set(),
                    },
                )
                if classification == "success":
                    entry["success_tests"].add(qualified_test_name)
                    entry["resource_types"].update(resource_types)
                else:
                    entry["diagnostic_tests"].add(qualified_test_name)

    return coverage


def snake_to_pascal(name: str) -> str:
    """Convert a snake_case SDK method name to PascalCase operation name."""
    return "".join(SDK_METHOD_ACRONYMS.get(w, w.capitalize()) for w in name.split("_"))


def get_tested_operations(test_path: Path) -> set[str]:
    """Extract operation names exercised in a service integration test file."""
    if not test_path.exists():
        return set()

    content = test_path.read_text()
    ops: set[str] = set()

    for m in re.finditer(
        r'\b([a-z][a-z0-9_]*)\s*\.\s*([a-z][a-z0-9_]*)\s*\(',
        content,
    ):
        method = m.group(2)
        if method in (
            "send", "await", "unwrap", "expect", "build", "clone",
            "new", "builder", "with_service", "http_client",
            "credentials_provider", "region", "load",
            "set_account_id", "set_name", "set_bucket",
            "key", "value", "into", "as_ref", "as_str",
            "map", "is_some", "is_none", "len", "iter",
            "collect", "push", "contains", "to_string",
            "endpoint_url", "state_view", "from", "default",
        ):
            continue
        if method.startswith(("set_", "get_", "is_", "as_", "to_", "into_", "with_")):
            if method.startswith(("set_", "with_")):
                continue
        ops.add(snake_to_pascal(method))

    return ops


def canonicalise_implemented_operations(
    model_ops: list[str],
    impl_ops: list[str],
) -> tuple[set[str], dict[str, str]]:
    """Canonicalise implemented operation names against Smithy model names."""
    model_ops_set = set(model_ops)
    impl_ops_set = set(impl_ops)
    canonical_impl = impl_ops_set & model_ops_set if model_ops else impl_ops_set
    canonical_by_lower = {op.lower(): op for op in canonical_impl}
    return canonical_impl, canonical_by_lower


def analyse_service_test_coverage(
    root: Path,
    crate_name: str,
    model_ops: list[str] | None = None,
    implemented_ops: list[str] | None = None,
) -> dict[str, object] | None:
    """Analyse integration-test coverage for one service crate."""
    crate_dir = root / "crates" / crate_name
    handlers_path = crate_dir / "src" / "handlers.rs"
    test_path = crate_dir / "tests" / "integration_test.rs"

    if not handlers_path.exists():
        return None

    if model_ops is None:
        model_name = CRATE_TO_MODEL.get(crate_name)
        if model_name:
            models_dir = root / "vendor" / "api-models-aws" / "models"
            model_ops = get_model_operations(models_dir, model_name)
        else:
            model_ops = []

    if implemented_ops is None:
        implemented_ops = get_implemented_operations(handlers_path, crate_name)

    canonical_impl, canonical_by_lower = canonicalise_implemented_operations(
        model_ops,
        implemented_ops,
    )

    tested_ops_raw = get_tested_operations(test_path)
    tested_canonical: set[str] = set()
    for tested_op in tested_ops_raw:
        canonical = canonical_by_lower.get(tested_op.lower())
        if canonical:
            tested_canonical.add(canonical)

    tested = sorted(canonical_impl & tested_canonical)
    untested = sorted(canonical_impl - tested_canonical)

    return {
        "crate": crate_name,
        "implemented": sorted(canonical_impl),
        "tested": tested,
        "untested": untested,
        "has_test_file": test_path.exists(),
    }


def get_model_operations(models_dir: Path, model_name: str) -> list[str]:
    """Extract operation names from a Smithy JSON model."""
    model_path = models_dir / model_name / "service"
    if not model_path.exists():
        return []

    # Find the versioned JSON file
    for version_dir in sorted(model_path.iterdir()):
        if not version_dir.is_dir():
            continue
        for json_file in version_dir.glob("*.json"):
            try:
                with open(json_file) as f:
                    model = json.load(f)
            except (json.JSONDecodeError, OSError):
                continue

            shapes = model.get("shapes", {})
            ops = []
            for key, shape in shapes.items():
                if shape.get("type") == "operation":
                    # Extract operation name from "com.amazonaws.service#OperationName"
                    name = key.rsplit("#", 1)[-1] if "#" in key else key
                    ops.append(name)
            return sorted(ops)

    return []


# Substrings whose presence in a handler body indicates the handler is
# *not* a trivial stub: it consults service state, parses the request, or
# performs other observable work. Used by ``_handler_body_is_stub``.
_STATE_ACCESS_INDICATORS: tuple[str, ...] = (
    "self.backend",
    "self.state",
    "self.b.",
    "self.blob",
    "self.vfs",
    "self.notify",
    "self.events",
    "backend.state",
    ".state(",
    ".write().await",
    ".read().await",
    ".lock().await",
    "with_state",
    "with_backend",
    "with_account_state",
    "with_region_state",
    "from_slice",
    "from_str",
    "from_reader",
    "parse_query",
    "parse_body",
    "parse_json",
    "parse_xml",
    "request.body",
    "req.body",
    "request.headers",
    "request.uri",
    "request.path",
    "extract_path",
    "send_event",
    "publish",
    "deliver",
    "Bytes::from",
    "to_vec",
    "validate",
    "ensure",
    "?;",
    "match ",
    "if let ",
    "while ",
    "for ",
    "loop {",
    "return ",
    "let mut ",
)

# Substrings whose presence indicates a handler body is producing a
# trivial empty / default response. Used together with the absence of
# state-access indicators to flag unannotated stubs.
_TRIVIAL_RESPONSE_INDICATORS: tuple[str, ...] = (
    "::default()",
    "..Default::default()",
    "success_empty",
    "empty_success",
    "empty_response",
    "MockResponse::empty",
    "Vec::new()",
)


def _handler_body_is_stub(body_lines: list[str]) -> bool:
    """Heuristically classify a handler body as a stub.

    A handler is treated as a stub when its body, after stripping blank
    lines, comments, and ``use`` statements, contains a trivial-response
    indicator (``::default()``, ``success_empty``, ...) but no
    state-access indicator (``self.backend``, ``with_state``, ``?;``, ...)
    and is short (<= 12 effective lines).
    """
    effective: list[str] = []
    for raw in body_lines:
        stripped = raw.strip()
        if not stripped:
            continue
        if stripped.startswith("//"):
            continue
        if stripped.startswith("use "):
            continue
        effective.append(stripped)

    if not effective or len(effective) > 12:
        return False

    body_text = " ".join(effective)

    if any(ind in body_text for ind in _STATE_ACCESS_INDICATORS):
        return False

    return any(ind in body_text for ind in _TRIVIAL_RESPONSE_INDICATORS)


def _extract_handler_bodies(content: str) -> list[tuple[int, str, list[str]]]:
    """Locate every ``fn handle_<name>`` function in *content* and return
    its (header_line_index, snake_case name, body_line_list).

    The body excludes the opening and closing braces. Brace nesting is
    tracked so we extract the full body even when it spans many lines or
    contains nested blocks.
    """
    lines = content.split("\n")
    out: list[tuple[int, str, list[str]]] = []

    header_re = re.compile(r"^\s*(?:pub\s+)?(?:async\s+)?fn\s+handle_(\w+)")
    for i, line in enumerate(lines):
        m = header_re.match(line)
        if not m:
            continue
        # Find the line that contains the opening brace of the body.
        brace_line = i
        while brace_line < len(lines) and "{" not in lines[brace_line]:
            brace_line += 1
        if brace_line >= len(lines):
            continue

        depth = 0
        body: list[str] = []
        started = False
        # Begin scanning from the line containing the opening brace; track
        # depth so we stop at the matching close brace.
        for j in range(brace_line, len(lines)):
            ln = lines[j]
            opens = ln.count("{")
            closes = ln.count("}")
            if not started:
                # Strip out the part before/at the first '{' on the brace line
                # so the body starts cleanly.
                first = ln.find("{")
                if first != -1:
                    rest = ln[first + 1 :]
                    if rest.strip():
                        body.append(rest)
                    depth += opens - closes
                    started = True
                    if depth <= 0:
                        break
                    continue
            depth += opens - closes
            if depth <= 0:
                # Drop the trailing close brace from the body line.
                last = ln.rfind("}")
                if last > 0 and ln[:last].strip():
                    body.append(ln[:last])
                break
            body.append(ln)
        out.append((i, m.group(1), body))

    return out


def get_implemented_operations(
    handlers_path: Path,
    crate_name: str | None = None,
) -> tuple[list[str], list[str]]:
    """Extract implemented and stub operation names from a handlers.rs file.

    Returns a tuple ``(implemented_ops, stub_ops)`` where each is a sorted
    list of PascalCase operation names. ``stub_ops`` contains handler
    methods that either (a) are immediately preceded by a
    ``// STUB[<category>]: ...`` annotation, or (b) have a trivial body
    that returns an empty/default response without consulting state.
    Both flavours satisfy AWS routing but do not implement service
    behaviour, so they are excluded from ``implemented_ops``.

    Supports multiple dispatch patterns:
    - awsQuery: match on Action parameter ("CreateBucket" => ...)
    - awsJson: match on X-Amz-Target ("Service.Operation" => ...)
    - REST: operation names from comments or handler method names
    """
    if not handlers_path.exists():
        return ([], [])

    with open(handlers_path) as f:
        content = f.read()

    ops = set()

    # Pre-pass: collect PascalCase names of all stub handler methods.
    # A handler is a stub when it is immediately preceded (allowing blank lines
    # and comment continuation lines) by a "// STUB[category]: ..." line.
    stub_ops: set[str] = set()
    lines = content.split('\n')

    def _snake_to_pascal(snake: str) -> str:
        return ''.join(
            SDK_METHOD_ACRONYMS.get(w, w.capitalize()) for w in snake.split('_')
        )

    # Pre-pass A: explicit `// STUB[<category>]: ...` annotations.
    # A handler is annotated as a stub when the run of comment / blank /
    # attribute lines immediately above it contains a STUB marker.
    for i, line in enumerate(lines):
        m = re.match(r'\s*(?:pub\s+)?(?:async\s+)?fn\s+handle_(\w+)', line)
        if not m:
            continue
        j = i - 1
        while j >= 0:
            prev = lines[j].strip()
            if prev == '':
                j -= 1
                continue
            # Skip Rust attributes (e.g. `#[cfg(...)]`, `#[allow(...)]`)
            # so a STUB comment placed above an attribute is still found.
            if prev.startswith('#['):
                j -= 1
                continue
            if prev.startswith('//'):
                if re.match(r'\s*//\s*STUB\[', lines[j]):
                    stub_ops.add(_snake_to_pascal(m.group(1)))
                    break
                j -= 1
                continue
            break  # non-comment, non-blank, non-attribute: not a stub

    # Pre-pass B: heuristic detection of unannotated stubs. A handler is
    # treated as a stub when its body returns a trivial empty/default
    # response without any state access. This catches common drift where
    # a handler was stubbed out without the canonical ``// STUB[...]``
    # marker required by `.agents/docs/QUALITY_GATE.md`.
    for header_idx, snake_name, body in _extract_handler_bodies(content):
        if _handler_body_is_stub(body):
            stub_ops.add(_snake_to_pascal(snake_name))

    # Pattern 1: awsQuery Action match arms
    #   "CreateLoadBalancer" => { ... }
    #   Also handles multi-line: "CreateLoadBalancer" => self\n    .handle_...
    for m in re.finditer(r'"(\w+)"\s*=>\s*(?:\{|self\s*\.handle_)', content, re.DOTALL):
        candidate = m.group(1)
        # Skip common non-operation strings
        if candidate in (
            "POST", "GET", "PUT", "DELETE", "HEAD", "PATCH",
            "HTTP", "HTTPS", "Sender", "User",
            "identities", "outbound-emails",  # SES path segments
        ):
            continue
        # Must look like a PascalCase operation name
        if re.match(r'^[A-Z][a-zA-Z0-9]+$', candidate):
            ops.add(candidate)

    # Pattern 2: awsJson X-Amz-Target match arms
    #   "TrentService.CreateKey" => ...
    #   Captures the part after the dot
    for m in re.finditer(r'"[\w.]+\.(\w+)"\s*=>', content):
        candidate = m.group(1)
        if re.match(r'^[A-Z][a-zA-Z0-9]+$', candidate):
            ops.add(candidate)

    # Pattern 3: REST services - extract from comments indicating operation names
    #   // CreateEmailIdentity
    #   // POST /v2/email/identities - CreateEmailIdentity
    for m in re.finditer(r'//\s*(?:POST|GET|PUT|DELETE|HEAD|PATCH)\s+\S+\s*-\s*(\w+)', content):
        candidate = m.group(1)
        if re.match(r'^[A-Z][a-zA-Z0-9]+$', candidate):
            ops.add(candidate)

    # Pattern 4: REST services - extract from handler method names
    #   fn handle_create_email_identity
    #   -> CreateEmailIdentity
    # Stub handlers are filtered out below via stub_ops.
    # Well-known acronyms that appear as lowercase snake_case words but
    # must be rendered fully uppercase in PascalCase operation names.
    for m in re.finditer(r'fn\s+handle_(\w+)', content):
        method_name = m.group(1)
        # Convert snake_case to PascalCase with acronym expansion
        pascal = ''.join(
            SDK_METHOD_ACRONYMS.get(word, word.capitalize())
            for word in method_name.split('_')
        )
        ops.add(pascal)

    # Pattern 5: REST-XML services (S3, Route53, CloudFront)
    # Check for comment-based operation markers
    for m in re.finditer(r'//\s+(\w+)\s*$', content, re.MULTILINE):
        candidate = m.group(1)
        if re.match(r'^[A-Z][a-z]+[A-Z]', candidate) and len(candidate) > 5:
            ops.add(candidate)

    # Clean up: remove handler-internal method artifacts that aren't real operations
    ops.discard("Dispatch")
    ops.discard("New")
    ops.discard("Default")

    # Remove stub handlers (preceded by // STUB[...] comment) — they satisfy
    # routing but return empty/default responses without real logic.
    ops -= stub_ops

    # Apply per-crate operation name overrides to correct capitalization
    # differences where AWS uses mixed-case acronyms (e.g. "Vpc" not "VPC").
    if crate_name:
        overrides = CRATE_OPERATION_NAME_OVERRIDES.get(crate_name, {})
        for wrong, correct in overrides.items():
            if wrong in ops:
                ops.discard(wrong)
                ops.add(correct)
            if wrong in stub_ops:
                stub_ops.discard(wrong)
                stub_ops.add(correct)

    return sorted(ops), sorted(stub_ops)


def parse_moto_coverage(moto_coverage_path: Path) -> dict[str, set[str]]:
    """Parse moto's IMPLEMENTATION_COVERAGE.md to extract implemented operations.

    Returns a dict mapping moto service name -> set of implemented operation
    names in snake_case.
    """
    if not moto_coverage_path.exists():
        return {}

    with open(moto_coverage_path) as f:
        content = f.read()

    result: dict[str, set[str]] = {}
    current_service = None

    for line in content.splitlines():
        # Service header: ## service-name
        svc_match = re.match(r'^## (\S+)', line)
        if svc_match:
            current_service = svc_match.group(1)
            result[current_service] = set()
            continue

        if current_service is None:
            continue

        # Implemented operation: - [X] operation_name
        impl_match = re.match(r'^- \[X\] (\S+)', line)
        if impl_match:
            result[current_service].add(impl_match.group(1))

    return result


_CACHE_TTL_DAYS = 7
_MOTO_RAW = "https://raw.githubusercontent.com/getmoto/moto/master/"
_FLOCI_API = "https://api.github.com/repos/floci-io/floci/contents/docs/services"
_KUMO_TREE_API = "https://api.github.com/repos/sivchari/kumo/git/trees/main?recursive=1"
_KUMO_RAW = "https://raw.githubusercontent.com/sivchari/kumo/main/"


def _github_get(url: str) -> str | None:
    """Fetch a URL with an optional GitHub token; return text or None on error."""
    headers: dict[str, str] = {"User-Agent": "winterbaume-coverage/1.0"}
    token = os.environ.get("GITHUB_TOKEN")
    if token:
        headers["Authorization"] = f"Bearer {token}"
    req = urllib.request.Request(url, headers=headers)
    try:
        with urllib.request.urlopen(req, timeout=15) as resp:
            return resp.read().decode()
    except Exception as exc:
        print(f"Warning: fetch {url} failed: {exc}", file=sys.stderr)
        return None


def _load_coverage_cache(cache_file: Path) -> dict[str, set[str]] | None:
    """Return cached coverage if it exists and is fresh, otherwise None."""
    if not cache_file.exists():
        return None
    try:
        data = json.loads(cache_file.read_text())
        fetched = date.fromisoformat(data.get("fetched", "2000-01-01"))
        if (date.today() - fetched).days > _CACHE_TTL_DAYS:
            return None
        coverage = {k: set(v) for k, v in data.get("coverage", {}).items()}
        version = data.get("version", "unknown")
        return coverage, version
    except Exception:
        return None


def _save_coverage_cache(
    cache_file: Path, coverage: dict[str, set[str]], version: str = "unknown"
) -> None:
    """Persist coverage and version to a JSON cache file."""
    cache_file.parent.mkdir(parents=True, exist_ok=True)
    payload = {
        "fetched": date.today().isoformat(),
        "version": version,
        "coverage": {k: sorted(v) for k, v in coverage.items()},
    }
    cache_file.write_text(json.dumps(payload, indent=2))


def _git_describe(repo_dir: Path) -> str:
    """Return `git describe --tags --always` for a directory, or 'unknown'."""
    try:
        result = subprocess.run(
            ["git", "describe", "--tags", "--always"],
            cwd=repo_dir, capture_output=True, text=True, timeout=5,
        )
        return result.stdout.strip() or "unknown"
    except Exception:
        return "unknown"


def _github_release_tag(owner: str, repo: str) -> str:
    """Return the latest GitHub release tag for owner/repo, or 'unknown'."""
    raw = _github_get(
        f"https://api.github.com/repos/{owner}/{repo}/releases/latest"
    )
    if not raw:
        return "unknown"
    try:
        return json.loads(raw).get("tag_name", "unknown")
    except Exception:
        return "unknown"


def get_moto_version(moto_dir: Path) -> str:
    """Read moto's __version__ from its package __init__.py."""
    init = moto_dir / "moto" / "__init__.py"
    if not init.exists():
        return "unknown"
    for line in init.read_text().splitlines():
        m = re.match(r'^__version__\s*=\s*["\'](.+)["\']', line)
        if m:
            return m.group(1)
    return "unknown"


def _fetch_moto_version_remote() -> str:
    """Fetch moto's __version__ from GitHub."""
    raw = _github_get(_MOTO_RAW + "moto/__init__.py")
    if not raw:
        return "unknown"
    for line in raw.splitlines():
        m = re.match(r'^__version__\s*=\s*["\'](.+)["\']', line)
        if m:
            return m.group(1)
    return "unknown"


def _fetch_moto_remote() -> tuple[dict[str, set[str]], str]:
    """Fetch moto coverage and version directly from GitHub."""
    raw = _github_get(_MOTO_RAW + "IMPLEMENTATION_COVERAGE.md")
    if not raw:
        return {}, "unknown"
    coverage = _parse_moto_coverage_text(raw)
    version = _fetch_moto_version_remote()
    return coverage, version


def _parse_moto_coverage_text(content: str) -> dict[str, set[str]]:
    """Parse moto's IMPLEMENTATION_COVERAGE.md text to extract implemented operations."""
    result: dict[str, set[str]] = {}
    current_service = None

    for line in content.splitlines():
        svc_match = re.match(r'^## (\S+)', line)
        if svc_match:
            current_service = svc_match.group(1)
            result[current_service] = set()
            continue

        if current_service is None:
            continue

        impl_match = re.match(r'^- \[X\] (\S+)', line)
        if impl_match:
            result[current_service].add(impl_match.group(1))

    return result


def parse_moto_coverage_cached(
    moto_dir: Path, cache_file: Path
) -> tuple[dict[str, set[str]], str]:
    """Return (coverage, version) for moto.

    Priority: vendor/moto on disk → local cache → GitHub fetch + cache.
    """
    moto_coverage_path = moto_dir / "IMPLEMENTATION_COVERAGE.md"
    if moto_coverage_path.exists():
        cov = parse_moto_coverage(moto_coverage_path)
        return cov, get_moto_version(moto_dir)

    cached = _load_coverage_cache(cache_file)
    if cached is not None:
        cov, version = cached
        return cov, version

    print("moto: vendor/moto not found — fetching from GitHub...",
          file=sys.stderr)
    cov, version = _fetch_moto_remote()
    if cov:
        _save_coverage_cache(cache_file, cov, version)
        print(f"moto: cached to {cache_file}", file=sys.stderr)
    return cov, version


def _parse_ops_from_floci_doc(content: str) -> set[str]:
    """Extract PascalCase operation names from a floci service Markdown doc."""
    ops: set[str] = set()
    for m in re.finditer(r'^\|[^|]*\|([^|]+)\|', content, re.MULTILINE):
        cell = m.group(1).strip()
        for raw_op in re.split(r'[,·]', cell):
            op = raw_op.strip()
            if re.match(r'^[A-Z][a-zA-Z0-9]+$', op):
                ops.add(op)
    return ops


def _parse_ops_from_kumo_go(content: str) -> set[str]:
    """Extract PascalCase operation names from a kumo Go source file."""
    ops: set[str] = set()
    # Actions() []string return value (Query/JSON protocol services)
    for m in re.finditer(
        r'\bActions\(\s*\)\s*\[\]string\s*\{(.*?)\n\s*\}',
        content,
        re.DOTALL,
    ):
        for op_m in re.finditer(r'"([A-Z][a-zA-Z0-9]+)"', m.group(1)):
            ops.add(op_m.group(1))
    # Public HTTP handler methods: func (recv *Type) GetObject(w http.ResponseWriter, ...)
    # These are the actual operation implementations used by REST-based services.
    for m in re.finditer(
        r'func\s+\(\w+\s+\*\w+\)\s+([A-Z][a-zA-Z0-9]+)\s*\(\w+\s+http\.ResponseWriter',
        content,
    ):
        ops.add(m.group(1))
    return ops


def _fetch_floci_remote() -> tuple[dict[str, set[str]], str]:
    """Fetch floci coverage and version directly from GitHub."""
    listing_raw = _github_get(_FLOCI_API)
    if not listing_raw:
        return {}, "unknown"
    try:
        entries = json.loads(listing_raw)
    except Exception:
        return {}, "unknown"

    result: dict[str, set[str]] = {}
    for entry in entries:
        name = entry.get("name", "")
        if not name.endswith(".md") or name == "index.md":
            continue
        raw_url = entry.get("download_url")
        if not raw_url:
            continue
        content = _github_get(raw_url)
        if not content:
            continue
        stem = name[:-3]
        model_name = FLOCI_STEM_TO_MODEL.get(stem, stem)
        ops = _parse_ops_from_floci_doc(content)
        if ops:
            result.setdefault(model_name, set()).update(ops)
    version = _github_release_tag("floci-io", "floci")
    return result, version


def _fetch_kumo_remote() -> tuple[dict[str, set[str]], str]:
    """Fetch kumo coverage and version directly from GitHub.

    Uses one tree API call to discover all *.go paths, then fetches each
    file under internal/service/*/ via raw.githubusercontent.com.
    """
    tree_raw = _github_get(_KUMO_TREE_API)
    if not tree_raw:
        return {}, "unknown"
    try:
        tree_data = json.loads(tree_raw)
    except Exception:
        return {}, "unknown"

    # Collect .go files grouped by service directory
    service_go_paths: dict[str, list[str]] = {}
    for item in tree_data.get("tree", []):
        path = item.get("path", "")
        m = re.match(r'^internal/service/([^/]+)/[^/]+\.go$', path)
        if m:
            service_go_paths.setdefault(m.group(1), []).append(path)

    result: dict[str, set[str]] = {}
    for dir_name, paths in sorted(service_go_paths.items()):
        model_name = KUMO_DIR_TO_MODEL.get(dir_name, dir_name)
        ops: set[str] = set()
        for path in paths:
            content = _github_get(_KUMO_RAW + path)
            if content:
                ops.update(_parse_ops_from_kumo_go(content))
        if ops:
            result[model_name] = ops
    version = _github_release_tag("sivchari", "kumo")
    return result, version


def parse_floci_coverage(
    floci_dir: Path, cache_file: Path
) -> tuple[dict[str, set[str]], str]:
    """Return (coverage, version) for floci.

    Priority: vendor/floci on disk → local cache → GitHub fetch + cache.
    """
    if (floci_dir / "docs" / "services").exists():
        cov: dict[str, set[str]] = {}
        for md_file in sorted((floci_dir / "docs" / "services").glob("*.md")):
            if md_file.name == "index.md":
                continue
            stem = md_file.stem
            model_name = FLOCI_STEM_TO_MODEL.get(stem, stem)
            ops = _parse_ops_from_floci_doc(md_file.read_text())
            if ops:
                cov.setdefault(model_name, set()).update(ops)
        return cov, _git_describe(floci_dir)

    cached = _load_coverage_cache(cache_file)
    if cached is not None:
        cov, version = cached
        if version == "unknown":
            version = _github_release_tag("floci-io", "floci")
            if version != "unknown":
                _save_coverage_cache(cache_file, cov, version)
        return cov, version

    print("floci: vendor/floci not found — fetching from GitHub...",
          file=sys.stderr)
    cov, version = _fetch_floci_remote()
    if cov:
        _save_coverage_cache(cache_file, cov, version)
        print(f"floci: cached to {cache_file}", file=sys.stderr)
    return cov, version


def parse_kumo_coverage(
    kumo_dir: Path, cache_file: Path
) -> tuple[dict[str, set[str]], str]:
    """Return (coverage, version) for kumo.

    Priority: vendor/kumo on disk → local cache → GitHub fetch + cache.
    """
    if (kumo_dir / "internal" / "service").exists():
        cov: dict[str, set[str]] = {}
        for svc_dir in sorted((kumo_dir / "internal" / "service").iterdir()):
            if not svc_dir.is_dir():
                continue
            dir_name = svc_dir.name
            model_name = KUMO_DIR_TO_MODEL.get(dir_name, dir_name)
            ops: set[str] = set()
            for go_file in sorted(svc_dir.glob("*.go")):
                ops.update(_parse_ops_from_kumo_go(go_file.read_text()))
            if ops:
                cov[model_name] = ops
        return cov, _git_describe(kumo_dir)

    cached = _load_coverage_cache(cache_file)
    if cached is not None:
        cov, version = cached
        if version == "unknown":
            version = _github_release_tag("sivchari", "kumo")
            if version != "unknown":
                _save_coverage_cache(cache_file, cov, version)
        return cov, version

    print("kumo: vendor/kumo not found — fetching from GitHub...",
          file=sys.stderr)
    cov, version = _fetch_kumo_remote()
    if cov:
        _save_coverage_cache(cache_file, cov, version)
        print(f"kumo: cached to {cache_file}", file=sys.stderr)
    return cov, version


def get_all_model_dirs(models_dir: Path) -> list[str]:
    """Get all service model directory names."""
    if not models_dir.exists():
        return []
    return sorted(
        d.name for d in models_dir.iterdir()
        if d.is_dir() and (d / "service").exists()
    )


def generate_report(root: Path, output: Path) -> None:
    """Generate the full coverage report."""
    models_dir = root / "vendor" / "api-models-aws" / "models"
    crates_dir = root / "crates"
    moto_dir = root / "vendor" / "moto"
    floci_dir = root / "vendor" / "floci"
    kumo_dir = root / "vendor" / "kumo"
    cache_dir = root / ".agents" / "skills" / "api-coverage" / "cache"

    if not models_dir.exists():
        print(
            "Error: vendor/api-models-aws/models not found. "
            "Run: git submodule update --init vendor/api-models-aws",
            file=sys.stderr,
        )
        sys.exit(1)

    # Parse moto coverage (falls back to GitHub fetch + local cache
    # when vendor/moto is not present as a submodule)
    moto_coverage, moto_version = parse_moto_coverage_cached(
        moto_dir, cache_dir / "moto_coverage.json"
    )

    # Parse floci and kumo coverage (falls back to GitHub fetch + local cache
    # when vendor/floci or vendor/kumo are not present as submodules)
    floci_coverage, floci_version = parse_floci_coverage(
        floci_dir, cache_dir / "floci_coverage.json"
    )
    kumo_coverage, kumo_version = parse_kumo_coverage(
        kumo_dir, cache_dir / "kumo_coverage.json"
    )

    terraform_e2e = parse_terraform_e2e_coverage(root)

    # Collect coverage data for each implemented service
    services = []
    total_wb_impl = 0
    total_wb_stubs = 0
    total_moto_impl = 0
    total_floci_impl = 0
    total_kumo_impl = 0
    total_ops = 0
    total_it_tested = 0
    total_it_implemented = 0

    for crate_name in sorted(CRATE_TO_MODEL.keys()):
        model_name = CRATE_TO_MODEL[crate_name]
        handlers_path = crates_dir / crate_name / "src" / "handlers.rs"

        model_ops = get_model_operations(models_dir, model_name)
        impl_ops, stub_ops = get_implemented_operations(handlers_path, crate_name)

        # Normalize: only count operations that exist in the model
        model_ops_set = set(model_ops)
        valid_impl = sorted(set(impl_ops) & model_ops_set)
        valid_stubs = sorted(set(stub_ops) & model_ops_set)
        missing = sorted(model_ops_set - set(impl_ops) - set(stub_ops))

        # Get moto coverage for this service
        moto_service_name = MODEL_TO_MOTO.get(model_name, model_name)
        moto_impl_snake = moto_coverage.get(moto_service_name, set())

        # Build a snake_case -> PascalCase lookup for model operations
        snake_to_pascal: dict[str, str] = {}
        for op in model_ops:
            snake_to_pascal[pascal_to_snake(op)] = op

        # Map moto's snake_case operations to PascalCase model operations
        moto_impl_pascal = set()
        for snake_op in moto_impl_snake:
            if snake_op in snake_to_pascal:
                moto_impl_pascal.add(snake_to_pascal[snake_op])

        moto_valid = sorted(moto_impl_pascal & model_ops_set)

        # floci and kumo report PascalCase names directly — just intersect with model
        floci_valid = sorted(floci_coverage.get(model_name, set()) & model_ops_set)
        kumo_valid = sorted(kumo_coverage.get(model_name, set()) & model_ops_set)

        integration_test_coverage = analyse_service_test_coverage(
            root,
            crate_name,
            model_ops=model_ops,
            implemented_ops=impl_ops,
        )

        services.append({
            "crate": crate_name,
            "model": model_name,
            "moto_service": moto_service_name,
            "implemented": valid_impl,
            "stubs": valid_stubs,
            "moto_implemented": moto_valid,
            "floci_implemented": floci_valid,
            "kumo_implemented": kumo_valid,
            "missing": missing,
            "total": len(model_ops),
            "all_ops": model_ops,
            "e2e_success_tests": sorted(terraform_e2e.get(crate_name, {}).get("success_tests", set())),
            "e2e_diagnostic_tests": sorted(terraform_e2e.get(crate_name, {}).get("diagnostic_tests", set())),
            "e2e_resource_types": sorted(terraform_e2e.get(crate_name, {}).get("resource_types", set())),
            "it_implemented": integration_test_coverage["implemented"] if integration_test_coverage else [],
            "it_tested": integration_test_coverage["tested"] if integration_test_coverage else [],
            "it_untested": integration_test_coverage["untested"] if integration_test_coverage else [],
            "it_has_test_file": integration_test_coverage["has_test_file"] if integration_test_coverage else False,
        })

        total_wb_impl += len(valid_impl)
        total_wb_stubs += len(valid_stubs)
        total_moto_impl += len(moto_valid)
        total_floci_impl += len(floci_valid)
        total_kumo_impl += len(kumo_valid)
        total_ops += len(model_ops)
        if integration_test_coverage:
            total_it_tested += len(integration_test_coverage["tested"])
            total_it_implemented += len(integration_test_coverage["implemented"])

    # Compute moto coverage for stub-only services
    stub_data = []
    for signing_name, (model_dir, moto_service) in sorted(STUB_TO_MODEL.items()):
        stub_model_ops = get_model_operations(models_dir, model_dir)
        stub_total = len(stub_model_ops)
        stub_moto_snake = moto_coverage.get(moto_service, set())
        stub_snake_to_pascal: dict[str, str] = {
            pascal_to_snake(op): op for op in stub_model_ops
        }
        stub_moto_count = sum(
            1 for s in stub_moto_snake if s in stub_snake_to_pascal
        )
        stub_data.append({
            "signing_name": signing_name,
            "model": model_dir,
            "moto_count": stub_moto_count,
            "total": stub_total,
        })

    # Find unimplemented services
    implemented_models = set(CRATE_TO_MODEL.values())
    all_models = get_all_model_dirs(models_dir)
    unimplemented = [m for m in all_models if m not in implemented_models]

    wb_version = _git_describe(root)

    # Generate report
    lines = []
    lines.append("# API Coverage Report")
    lines.append("")
    lines.append(f"Generated: {date.today().isoformat()}")
    lines.append("")
    lines.append("| Project | Version |")
    lines.append("|---------|---------|")
    lines.append(f"| winterbaume | {wb_version} |")
    lines.append(f"| moto | {moto_version} |")
    lines.append(f"| floci | {floci_version} |")
    lines.append(f"| kumo | {kumo_version} |")
    lines.append("")
    lines.append("## Overview")
    lines.append("")
    lines.append(
        "Legend: `winterbaume` = operations with real, state-backed logic. "
        "`stubs` = operations whose handler routes the request and returns "
        "an empty/default response without real behaviour (annotated with "
        "`// STUB[<category>]: ...` in `handlers.rs`). The two columns are "
        "disjoint -- stubs are excluded from the winterbaume count."
    )
    lines.append("")
    lines.append("| Service | Model | winterbaume | stubs | moto | floci | kumo | Total | wb% | stub% | moto% | floci% | kumo% |")
    lines.append("|---------|-------|-------------|-------|------|-------|------|-------|-----|-------|-------|--------|------|")

    for svc in services:
        wb_count = len(svc["implemented"])
        stub_count = len(svc["stubs"])
        moto_count = len(svc["moto_implemented"])
        floci_count = len(svc["floci_implemented"])
        kumo_count = len(svc["kumo_implemented"])
        total = svc["total"]
        wb_pct = (wb_count / total * 100) if total > 0 else 0.0
        stub_pct = (stub_count / total * 100) if total > 0 else 0.0
        moto_pct = (moto_count / total * 100) if total > 0 else 0.0
        floci_pct = (floci_count / total * 100) if total > 0 else 0.0
        kumo_pct = (kumo_count / total * 100) if total > 0 else 0.0
        lines.append(
            f"| {svc['crate']} | {svc['model']} "
            f"| {wb_count} | {stub_count} | {moto_count} | {floci_count} | {kumo_count} | {total} "
            f"| {wb_pct:.1f}% | {stub_pct:.1f}% | {moto_pct:.1f}% | {floci_pct:.1f}% | {kumo_pct:.1f}% |"
        )

    lines.append("")
    wb_overall_pct = (total_wb_impl / total_ops * 100) if total_ops > 0 else 0.0
    stub_overall_pct = (total_wb_stubs / total_ops * 100) if total_ops > 0 else 0.0
    moto_overall_pct = (total_moto_impl / total_ops * 100) if total_ops > 0 else 0.0
    floci_overall_pct = (total_floci_impl / total_ops * 100) if total_ops > 0 else 0.0
    kumo_overall_pct = (total_kumo_impl / total_ops * 100) if total_ops > 0 else 0.0
    lines.append(
        f"**winterbaume {wb_version}: {total_wb_impl} / {total_ops} operations "
        f"across {len(services)} services ({wb_overall_pct:.1f}%)**"
    )
    lines.append("")
    lines.append(
        f"**winterbaume stubs: {total_wb_stubs} / {total_ops} operations "
        f"across {len(services)} services ({stub_overall_pct:.1f}%) "
        f"- routed but return empty/default responses**"
    )
    lines.append("")
    lines.append(
        f"**moto {moto_version}: {total_moto_impl} / {total_ops} operations "
        f"across {len(services)} services ({moto_overall_pct:.1f}%)**"
    )
    lines.append("")
    lines.append(
        f"**floci {floci_version}: {total_floci_impl} / {total_ops} operations "
        f"across {len(services)} services ({floci_overall_pct:.1f}%)**"
    )
    lines.append("")
    lines.append(
        f"**kumo {kumo_version}: {total_kumo_impl} / {total_ops} operations "
        f"across {len(services)} services ({kumo_overall_pct:.1f}%)**"
    )
    lines.append("")
    total_it_pct = (
        total_it_tested / total_it_implemented * 100
        if total_it_implemented > 0
        else 0.0
    )
    it_covered_services = sum(1 for svc in services if svc["it_tested"])
    it_no_test_file_services = sum(1 for svc in services if not svc["it_has_test_file"])
    lines.append(
        f"**integration tests: {total_it_tested} / {total_it_implemented} "
        f"implemented operations tested ({total_it_pct:.1f}%)**"
    )
    lines.append("")
    lines.append(
        f"**integration-test service coverage: {it_covered_services} / {len(services)} "
        "services with at least one tested implemented operation "
        f"({(it_covered_services / len(services) * 100) if services else 0.0:.1f}%)**"
    )
    if it_no_test_file_services:
        lines.append("")
        lines.append(
            f"**integration tests missing: {it_no_test_file_services} services "
            "currently have no `tests/integration_test.rs` file**"
        )

    e2e_covered_services = sum(1 for svc in services if svc["e2e_success_tests"])
    e2e_diagnostic_only_services = sum(
        1
        for svc in services
        if svc["e2e_diagnostic_tests"] and not svc["e2e_success_tests"]
    )
    total_e2e_tests = sum(len(svc["e2e_success_tests"]) for svc in services)
    total_e2e_resource_types = len({
        resource_type
        for svc in services
        for resource_type in svc["e2e_resource_types"]
    })
    lines.append("")
    lines.append(
        f"**terraform E2E: {e2e_covered_services} / {len(services)} services "
        f"with success-oriented coverage "
        f"({(e2e_covered_services / len(services) * 100) if services else 0.0:.1f}%)**"
    )
    lines.append("")
    lines.append(
        f"**terraform E2E tests: {total_e2e_tests} success-oriented tests "
        f"covering {total_e2e_resource_types} terraform resource types**"
    )
    if e2e_diagnostic_only_services:
        lines.append("")
        lines.append(
            f"**terraform E2E diagnostics: {e2e_diagnostic_only_services} services "
            "currently have diagnostic-only terraform coverage**"
        )

    # Stub service moto coverage section
    lines.append("")
    lines.append("## Stub Service Moto Coverage")
    lines.append("")
    lines.append("| Signing Name | Model | moto | Total | moto% |")
    lines.append("|---|---|---|---|---|")
    for entry in stub_data:
        stub_total = entry["total"]
        stub_moto_count = entry["moto_count"]
        stub_moto_pct = (stub_moto_count / stub_total * 100) if stub_total > 0 else 0.0
        lines.append(
            f"| {entry['signing_name']} | {entry['model']} "
            f"| {stub_moto_count} | {stub_total} | {stub_moto_pct:.1f}% |"
        )

    # Integration-test coverage section
    lines.append("")
    lines.append("## Integration Test Coverage")
    lines.append("")
    lines.append(
        "Legend: `Implemented` = implemented operations, `Tested` = implemented "
        "operations exercised by `tests/integration_test.rs`, `Untested` = "
        "implemented operations without integration coverage"
    )
    lines.append("")
    lines.append("| Service | Implemented | Tested | Untested | Coverage | Status |")
    lines.append("|---|---|---|---|---|---|")
    for svc in services:
        implemented_count = len(svc["it_implemented"])
        tested_count = len(svc["it_tested"])
        untested_count = len(svc["it_untested"])
        coverage_pct = (
            tested_count / implemented_count * 100
            if implemented_count > 0
            else 0.0
        )
        if not svc["it_has_test_file"]:
            status = "no test file"
        elif untested_count == 0:
            status = "fully covered"
        elif tested_count == 0:
            status = "file but no detected coverage"
        else:
            status = "partial"
        lines.append(
            f"| {svc['crate']} | {implemented_count} | {tested_count} | "
            f"{untested_count} | {coverage_pct:.1f}% | {status} |"
        )

    # Terraform E2E coverage section
    lines.append("")
    lines.append("## Terraform E2E Coverage")
    lines.append("")
    lines.append(
        "Legend: `Tests` = success-oriented terraform tests, `Resources` = "
        "distinct terraform resource types exercised, `Diag` = diagnostic-only "
        "terraform tests"
    )
    lines.append("")
    lines.append("| Service | Tests | Resources | Diag | Status |")
    lines.append("|---|---|---|---|---|")
    for svc in services:
        e2e_tests = len(svc["e2e_success_tests"])
        e2e_resources = len(svc["e2e_resource_types"])
        e2e_diag = len(svc["e2e_diagnostic_tests"])
        if e2e_tests == 0 and e2e_diag == 0:
            continue
        status = "covered" if e2e_tests else "diagnostic-only"
        lines.append(
            f"| {svc['crate']} | {e2e_tests} | {e2e_resources} | {e2e_diag} | {status} |"
        )

    # Unimplemented services section
    lines.append("")
    lines.append("## Services Not Yet Implemented")
    lines.append("")
    for model in unimplemented:
        lines.append(f"- {model}")

    # Detailed per-service sections
    lines.append("")
    lines.append("## Detailed Coverage")
    lines.append("")
    lines.append(
        "Legend: `W` = winterbaume (real implementation), `S` = stub "
        "(routed but returns empty/default), `M` = moto, `F` = floci, "
        "`K` = kumo"
    )

    for svc in services:
        wb_set = set(svc["implemented"])
        stub_set = set(svc["stubs"])
        moto_set = set(svc["moto_implemented"])
        floci_set = set(svc["floci_implemented"])
        kumo_set = set(svc["kumo_implemented"])
        wb_count = len(wb_set)
        stub_count = len(stub_set)
        moto_count = len(moto_set)
        floci_count = len(floci_set)
        kumo_count = len(kumo_set)
        total = svc["total"]

        lines.append("")
        lines.append(
            f"### {svc['crate']} ({svc['model']}) "
            f"- W: {wb_count}/{total}, S: {stub_count}/{total}, "
            f"M: {moto_count}/{total}, "
            f"F: {floci_count}/{total}, K: {kumo_count}/{total}"
        )
        lines.append("")
        e2e_tests = len(svc["e2e_success_tests"])
        e2e_diag = len(svc["e2e_diagnostic_tests"])
        e2e_resource_types = svc["e2e_resource_types"]
        if e2e_tests:
            lines.append(
                f"Terraform E2E: {e2e_tests} tests across "
                f"{len(e2e_resource_types)} terraform resource types"
            )
            lines.append("")
            lines.append(
                "Resource types: " + ", ".join(e2e_resource_types)
            )
            lines.append("")
        elif e2e_diag:
            lines.append(
                f"Terraform E2E: diagnostic-only ({e2e_diag} terraform tests)"
            )
            lines.append("")

        for op in svc["all_ops"]:
            w = "x" if op in wb_set else " "
            s = "x" if op in stub_set else " "
            m = "x" if op in moto_set else " "
            f = "x" if op in floci_set else " "
            k = "x" if op in kumo_set else " "
            lines.append(f"- W[{w}] S[{s}] M[{m}] F[{f}] K[{k}] {op}")

        lines.append("")
        implemented_count = len(svc["it_implemented"])
        tested_count = len(svc["it_tested"])
        untested_count = len(svc["it_untested"])
        if not svc["it_has_test_file"]:
            lines.append("Integration tests: no integration test file")
        else:
            coverage_pct = (
                tested_count / implemented_count * 100
                if implemented_count > 0
                else 0.0
            )
            lines.append(
                f"Integration tests: {tested_count}/{implemented_count} "
                f"implemented operations tested ({coverage_pct:.1f}%)"
            )
            if untested_count:
                lines.append(
                    f"Untested implemented operations: {untested_count}"
                )

    lines.append("")

    output.parent.mkdir(parents=True, exist_ok=True)
    with open(output, "w") as f:
        f.write("\n".join(lines))

    print(f"Report written to {output}")
    print(
        f"winterbaume {wb_version}: {total_wb_impl} / {total_ops} across "
        f"{len(services)} services ({wb_overall_pct:.1f}%)"
    )
    print(
        f"winterbaume stubs: {total_wb_stubs} / {total_ops} across "
        f"{len(services)} services ({stub_overall_pct:.1f}%)"
    )
    print(
        f"moto {moto_version}: {total_moto_impl} / {total_ops} across "
        f"{len(services)} services ({moto_overall_pct:.1f}%)"
    )
    print(
        f"floci {floci_version}: {total_floci_impl} / {total_ops} across "
        f"{len(services)} services ({floci_overall_pct:.1f}%)"
    )
    print(
        f"kumo {kumo_version}: {total_kumo_impl} / {total_ops} across "
        f"{len(services)} services ({kumo_overall_pct:.1f}%)"
    )


def main() -> None:
    parser = argparse.ArgumentParser(description="Generate AWS API coverage report")
    parser.add_argument(
        "--output", "-o",
        type=Path,
        default=None,
        help="Output file path (default: .agents/docs/API_COVERAGE.md)",
    )
    parser.add_argument(
        "--refresh-remote",
        action="store_true",
        help="Delete cached floci/kumo data and re-fetch from GitHub",
    )
    args = parser.parse_args()

    root = find_project_root()
    output = args.output or (root / ".agents" / "docs" / "API_COVERAGE.md")

    if args.refresh_remote:
        cache_dir = root / ".agents" / "skills" / "api-coverage" / "cache"
        for cache_file in (
            cache_dir / "moto_coverage.json",
            cache_dir / "floci_coverage.json",
            cache_dir / "kumo_coverage.json",
        ):
            if cache_file.exists():
                cache_file.unlink()
                print(f"Deleted cache: {cache_file}")

    generate_report(root, output)


if __name__ == "__main__":
    main()
