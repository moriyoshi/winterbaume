//! Service discovery: maps winterbaume crate names to Smithy model files.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};

/// Mapping from winterbaume crate suffix to Smithy model directory name.
///
/// Most services follow the convention where the crate name matches the model
/// directory, but some differ.
const SERVICE_MAP: &[(&str, &str)] = &[
    ("account", "account"),
    ("acm", "acm"),
    ("apigateway", "api-gateway"),
    ("batch", "batch"),
    ("appconfig", "appconfig"),
    ("athena", "athena"),
    ("backup", "backup"),
    ("budgets", "budgets"),
    ("cloudfront", "cloudfront"),
    ("cloudtrail", "cloudtrail"),
    ("cloudwatch", "cloudwatch"),
    ("codebuild", "codebuild"),
    ("codecommit", "codecommit"),
    ("codedeploy", "codedeploy"),
    ("codepipeline", "codepipeline"),
    ("cognitoidentity", "cognito-identity"),
    ("cognitoidp", "cognito-identity-provider"),
    ("comprehend", "comprehend"),
    ("config", "config-service"),
    ("datasync", "datasync"),
    ("dax", "dax"),
    ("directconnect", "direct-connect"),
    ("dms", "database-migration-service"),
    ("ds", "directory-service"),
    ("dynamodb", "dynamodb"),
    ("ec2instanceconnect", "ec2-instance-connect"),
    ("ecr", "ecr"),
    ("ecs", "ecs"),
    ("efs", "efs"),
    ("eks", "eks"),
    ("elbv2", "elastic-load-balancing-v2"),
    ("events", "eventbridge"),
    ("firehose", "firehose"),
    ("glacier", "glacier"),
    ("guardduty", "guardduty"),
    ("iam", "iam"),
    ("identitystore", "identitystore"),
    ("inspector2", "inspector2"),
    ("ivs", "ivs"),
    ("kinesis", "kinesis"),
    ("kms", "kms"),
    ("lambda", "lambda"),
    ("lex", "lex-models-v2"),
    ("logs", "cloudwatch-logs"),
    ("macie2", "macie2"),
    ("mq", "mq"),
    ("organizations", "organizations"),
    ("pipes", "pipes"),
    ("polly", "polly"),
    ("ram", "ram"),
    ("rdsdata", "rds-data"),
    ("rekognition", "rekognition"),
    ("resourcegroups", "resource-groups"),
    ("route53", "route-53"),
    ("s3", "s3"),
    ("scheduler", "scheduler"),
    ("secretsmanager", "secrets-manager"),
    ("securityhub", "securityhub"),
    ("servicequotas", "service-quotas"),
    ("servicediscovery", "servicediscovery"),
    ("ses", "ses"),
    ("sesv2", "sesv2"),
    ("shield", "shield"),
    ("signer", "signer"),
    ("sns", "sns"),
    ("sqs", "sqs"),
    ("ssm", "ssm"),
    ("stepfunctions", "sfn"),
    ("sts", "sts"),
    ("support", "support"),
    ("synthetics", "synthetics"),
    ("transfer", "transfer"),
    ("transcribe", "transcribe"),
    ("wafv2", "wafv2"),
    ("xray", "xray"),
    // Batch 12
    ("textract", "textract"),
    ("forecast", "forecast"),
    ("personalize", "personalize"),
    ("route53resolver", "route53resolver"),
    ("acmpca", "acm-pca"),
    // Batch 13
    ("applicationautoscaling", "application-auto-scaling"),
    ("autoscaling", "auto-scaling"),
    ("cloudformation", "cloudformation"),
    ("ec2", "ec2"),
    ("elasticbeanstalk", "elastic-beanstalk"),
    ("elasticache", "elasticache"),
    ("elasticloadbalancing", "elastic-load-balancing"),
    ("elasticloadbalancingv2", "elastic-load-balancing-v2"),
    ("appmesh", "app-mesh"),
    ("bedrockagent", "bedrock-agent"),
    ("cloudhsmv2", "cloudhsm-v2"),
    ("costexplorer", "cost-explorer"),
    ("datapipeline", "data-pipeline"),
    ("dynamodbstreams", "dynamodb-streams"),
    ("emrcontainers", "emr-containers"),
    ("emrserverless", "emr-serverless"),
    ("ssoadmin", "sso-admin"),
    ("iotdataplane", "iot-data-plane"),
    ("kinesisanalyticsv2", "kinesis-analytics-v2"),
    ("kinesisvideo", "kinesis-video"),
    ("kinesisvideoarchivedmedia", "kinesis-video-archived-media"),
    ("medialive", "medialive"),
    ("mediastoredata", "mediastore-data"),
    ("meteringmarketplace", "marketplace-metering"),
    ("networkfirewall", "network-firewall"),
    ("redshiftdata", "redshift-data"),
    ("route53domains", "route-53-domains"),
    ("sagemakermetrics", "sagemaker-metrics"),
    ("sagemakerruntime", "sagemaker-runtime"),
    ("sdb", "simpledbv2"),
    ("servicecatalog", "service-catalog"),
    ("servicecatalogappregistry", "service-catalog-appregistry"),
    ("tagging", "resource-groups-tagging-api"),
    ("timestreaminfluxdb", "timestream-influxdb"),
    ("timestreamquery", "timestream-query"),
    ("timestreamwrite", "timestream-write"),
    ("vpclattice", "vpc-lattice"),
    ("workspacesweb", "workspaces-web"),
    ("savingsplans", "savingsplans"),
    ("personalizeevents", "personalize-events"),
    ("personalizeruntime", "personalize-runtime"),
    ("pinpointsmsvoice", "pinpoint-sms-voice"),
    ("pi", "pi"),
    ("trustedadvisor", "trustedadvisor"),
    ("supportapp", "support-app"),
    ("pricing", "pricing"),
    ("freetier", "freetier"),
    ("cloud9", "cloud9"),
    ("s3outposts", "s3outposts"),
    ("route53recoverycluster", "route53-recovery-cluster"),
    ("snowdevicemanagement", "snow-device-management"),
    ("simspaceweaver", "simspaceweaver"),
    ("taxsettings", "taxsettings"),
    ("ssmquicksetup", "ssm-quicksetup"),
    ("pcaconnectorscep", "pca-connector-scep"),
    ("cloudfrontkeyvaluestore", "cloudfront-keyvaluestore"),
    ("cloudtraildata", "cloudtrail-data"),
    ("cloudsearchdomain", "cloudsearch-domain"),
    ("connectparticipant", "connectparticipant"),
    ("controlcatalog", "controlcatalog"),
    ("costandusagereport", "cost-and-usage-report-service"),
    ("braket", "braket"),
    ("chimesdkmeetings", "chime-sdk-meetings"),
    ("cognitosync", "cognito-sync"),
    ("codestarnotifications", "codestar-notifications"),
    ("codegurusecurity", "codeguru-security"),
    ("codegurureviewer", "codeguru-reviewer"),
    // Batch 14 — 2026-05-02 mass regen sweep: stale wire.rs files needing first-time
    // request-deserializer codegen.
    (
        "applicationdiscoveryservice",
        "application-discovery-service",
    ),
    ("applicationinsights", "application-insights"),
    ("applicationsignals", "application-signals"),
    ("arczonalshift", "arc-zonal-shift"),
    ("autoscalingplans", "auto-scaling-plans"),
    ("backupgateway", "backup-gateway"),
    ("bcmdashboards", "bcm-dashboards"),
    ("bcmdataexports", "bcm-data-exports"),
    ("lexmodelsv2", "lex-models-v2"),
    ("marketplacemetering", "marketplace-metering"),
    // Crate-name aliases for crates whose suffix differs from the canonical
    // SERVICE_MAP key, so `gen-serializers <crate-suffix>` resolves directly.
    ("cloudwatchlogs", "cloudwatch-logs"),
    ("cognitoidentityprovider", "cognito-identity-provider"),
    ("databasemigration", "database-migration-service"),
    ("directory", "directory-service"),
];

/// Find the Smithy JSON model file for a given winterbaume service name.
pub fn find_model_file(service_name: &str, models_dir: &Path) -> Result<PathBuf> {
    let model_dir_name = SERVICE_MAP
        .iter()
        .find(|(crate_name, _)| *crate_name == service_name)
        .map(|(_, model_name)| *model_name)
        .unwrap_or(service_name);

    let service_dir = models_dir.join(model_dir_name).join("service");
    if !service_dir.exists() {
        bail!(
            "Model directory not found: {} (looked for {})",
            service_dir.display(),
            model_dir_name,
        );
    }

    // Find the version directory (there's usually just one)
    let version_dir = std::fs::read_dir(&service_dir)
        .with_context(|| format!("Cannot read {}", service_dir.display()))?
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().map(|t| t.is_dir()).unwrap_or(false))
        .max_by_key(|e| e.file_name())
        .context(format!("No version directory in {}", service_dir.display()))?;

    // Find the .json file in the version directory
    let json_file = std::fs::read_dir(version_dir.path())
        .with_context(|| format!("Cannot read {}", version_dir.path().display()))?
        .filter_map(|e| e.ok())
        .find(|e| {
            e.path()
                .extension()
                .map(|ext| ext == "json")
                .unwrap_or(false)
        })
        .context(format!("No JSON file in {}", version_dir.path().display()))?;

    Ok(json_file.path())
}

/// List all known service mappings.
pub fn list_services() -> &'static [(&'static str, &'static str)] {
    SERVICE_MAP
}
