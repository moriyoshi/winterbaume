//! Winterbaume umbrella crate.
//!
//! Re-exports every winterbaume service crate behind a Cargo feature flag.
//! Enable individual services in your `Cargo.toml`, for example:
//!
//! ```toml
//! [dependencies]
//! winterbaume = { version = "0.1.0", features = ["service-s3", "service-dynamodb"] }
//! ```
//!
//! Enable every service at once with the `full` feature:
//!
//! ```toml
//! winterbaume = { version = "0.1.0", features = ["full"] }
//! ```

#[cfg(feature = "service-account")]
pub use winterbaume_account as account;
#[cfg(feature = "service-acm")]
pub use winterbaume_acm as acm;
#[cfg(feature = "service-acmpca")]
pub use winterbaume_acmpca as acmpca;
#[cfg(feature = "service-amp")]
pub use winterbaume_amp as amp;
#[cfg(feature = "service-appconfig")]
pub use winterbaume_appconfig as appconfig;
#[cfg(feature = "service-applicationautoscaling")]
pub use winterbaume_applicationautoscaling as applicationautoscaling;
#[cfg(feature = "service-appmesh")]
pub use winterbaume_appmesh as appmesh;
#[cfg(feature = "service-appsync")]
pub use winterbaume_appsync as appsync;
#[cfg(feature = "service-athena")]
pub use winterbaume_athena as athena;
#[cfg(feature = "service-backup")]
pub use winterbaume_backup as backup;
#[cfg(feature = "service-batch")]
pub use winterbaume_batch as batch;
#[cfg(feature = "service-bedrock")]
pub use winterbaume_bedrock as bedrock;
#[cfg(feature = "service-bedrockagent")]
pub use winterbaume_bedrockagent as bedrockagent;
#[cfg(feature = "service-budgets")]
pub use winterbaume_budgets as budgets;
#[cfg(feature = "service-cloudfront")]
pub use winterbaume_cloudfront as cloudfront;
#[cfg(feature = "service-cloudhsmv2")]
pub use winterbaume_cloudhsmv2 as cloudhsmv2;
#[cfg(feature = "service-cloudtrail")]
pub use winterbaume_cloudtrail as cloudtrail;
#[cfg(feature = "service-cloudwatch")]
pub use winterbaume_cloudwatch as cloudwatch;
#[cfg(feature = "service-cloudwatchlogs")]
pub use winterbaume_cloudwatchlogs as cloudwatchlogs;
#[cfg(feature = "service-codebuild")]
pub use winterbaume_codebuild as codebuild;
#[cfg(feature = "service-codecommit")]
pub use winterbaume_codecommit as codecommit;
#[cfg(feature = "service-codedeploy")]
pub use winterbaume_codedeploy as codedeploy;
#[cfg(feature = "service-codepipeline")]
pub use winterbaume_codepipeline as codepipeline;
#[cfg(feature = "service-cognitoidentity")]
pub use winterbaume_cognitoidentity as cognitoidentity;
#[cfg(feature = "service-cognitoidentityprovider")]
pub use winterbaume_cognitoidentityprovider as cognitoidentityprovider;
#[cfg(feature = "service-comprehend")]
pub use winterbaume_comprehend as comprehend;
#[cfg(feature = "service-config")]
pub use winterbaume_config as config;
#[cfg(feature = "service-connect")]
pub use winterbaume_connect as connect;
#[cfg(feature = "service-connectcampaigns")]
pub use winterbaume_connectcampaigns as connectcampaigns;
#[cfg(feature = "service-costexplorer")]
pub use winterbaume_costexplorer as costexplorer;
#[cfg(feature = "service-databrew")]
pub use winterbaume_databrew as databrew;
#[cfg(feature = "service-datapipeline")]
pub use winterbaume_datapipeline as datapipeline;
#[cfg(feature = "service-datasync")]
pub use winterbaume_datasync as datasync;
#[cfg(feature = "service-dax")]
pub use winterbaume_dax as dax;
#[cfg(feature = "service-directconnect")]
pub use winterbaume_directconnect as directconnect;
#[cfg(feature = "service-directory")]
pub use winterbaume_directory as directory;
#[cfg(feature = "service-dsql")]
pub use winterbaume_dsql as dsql;
#[cfg(feature = "service-dynamodb")]
pub use winterbaume_dynamodb as dynamodb;
#[cfg(feature = "service-dynamodbstreams")]
pub use winterbaume_dynamodbstreams as dynamodbstreams;
#[cfg(feature = "service-ebs")]
pub use winterbaume_ebs as ebs;
#[cfg(feature = "service-ec2instanceconnect")]
pub use winterbaume_ec2instanceconnect as ec2instanceconnect;
#[cfg(feature = "service-ecr")]
pub use winterbaume_ecr as ecr;
#[cfg(feature = "service-ecs")]
pub use winterbaume_ecs as ecs;
#[cfg(feature = "service-efs")]
pub use winterbaume_efs as efs;
#[cfg(feature = "service-eks")]
pub use winterbaume_eks as eks;
#[cfg(feature = "service-elasticloadbalancing")]
pub use winterbaume_elasticloadbalancing as elasticloadbalancing;
#[cfg(feature = "service-elasticloadbalancingv2")]
pub use winterbaume_elasticloadbalancingv2 as elasticloadbalancingv2;
#[cfg(feature = "service-emrcontainers")]
pub use winterbaume_emrcontainers as emrcontainers;
#[cfg(feature = "service-emrserverless")]
pub use winterbaume_emrserverless as emrserverless;
#[cfg(feature = "service-eventbridge")]
pub use winterbaume_eventbridge as eventbridge;
#[cfg(feature = "service-firehose")]
pub use winterbaume_firehose as firehose;
#[cfg(feature = "service-forecast")]
pub use winterbaume_forecast as forecast;
#[cfg(feature = "service-fsx")]
pub use winterbaume_fsx as fsx;
#[cfg(feature = "service-glacier")]
pub use winterbaume_glacier as glacier;
#[cfg(feature = "service-glue")]
pub use winterbaume_glue as glue;
#[cfg(feature = "service-greengrass")]
pub use winterbaume_greengrass as greengrass;
#[cfg(feature = "service-guardduty")]
pub use winterbaume_guardduty as guardduty;
#[cfg(feature = "service-iam")]
pub use winterbaume_iam as iam;
#[cfg(feature = "service-identitystore")]
pub use winterbaume_identitystore as identitystore;
#[cfg(feature = "service-inspector2")]
pub use winterbaume_inspector2 as inspector2;
#[cfg(feature = "service-iot")]
pub use winterbaume_iot as iot;
#[cfg(feature = "service-iotdataplane")]
pub use winterbaume_iotdataplane as iotdataplane;
#[cfg(feature = "service-ivs")]
pub use winterbaume_ivs as ivs;
#[cfg(feature = "service-kafka")]
pub use winterbaume_kafka as kafka;
#[cfg(feature = "service-kinesis")]
pub use winterbaume_kinesis as kinesis;
#[cfg(feature = "service-kinesisanalyticsv2")]
pub use winterbaume_kinesisanalyticsv2 as kinesisanalyticsv2;
#[cfg(feature = "service-kinesisvideo")]
pub use winterbaume_kinesisvideo as kinesisvideo;
#[cfg(feature = "service-kinesisvideoarchivedmedia")]
pub use winterbaume_kinesisvideoarchivedmedia as kinesisvideoarchivedmedia;
#[cfg(feature = "service-kms")]
pub use winterbaume_kms as kms;
#[cfg(feature = "service-lakeformation")]
pub use winterbaume_lakeformation as lakeformation;
#[cfg(feature = "service-lambda")]
pub use winterbaume_lambda as lambda;
#[cfg(feature = "service-lexmodelsv2")]
pub use winterbaume_lexmodelsv2 as lexmodelsv2;
#[cfg(feature = "service-macie2")]
pub use winterbaume_macie2 as macie2;
#[cfg(feature = "service-managedblockchain")]
pub use winterbaume_managedblockchain as managedblockchain;
#[cfg(feature = "service-marketplacemetering")]
pub use winterbaume_marketplacemetering as marketplacemetering;
#[cfg(feature = "service-mediaconnect")]
pub use winterbaume_mediaconnect as mediaconnect;
#[cfg(feature = "service-medialive")]
pub use winterbaume_medialive as medialive;
#[cfg(feature = "service-mediapackage")]
pub use winterbaume_mediapackage as mediapackage;
#[cfg(feature = "service-mediapackagev2")]
pub use winterbaume_mediapackagev2 as mediapackagev2;
#[cfg(feature = "service-mediastore")]
pub use winterbaume_mediastore as mediastore;
#[cfg(feature = "service-mediastoredata")]
pub use winterbaume_mediastoredata as mediastoredata;
#[cfg(feature = "service-memorydb")]
pub use winterbaume_memorydb as memorydb;
#[cfg(feature = "service-mq")]
pub use winterbaume_mq as mq;
#[cfg(feature = "service-networkfirewall")]
pub use winterbaume_networkfirewall as networkfirewall;
#[cfg(feature = "service-networkmanager")]
pub use winterbaume_networkmanager as networkmanager;
#[cfg(feature = "service-opensearch")]
pub use winterbaume_opensearch as opensearch;
#[cfg(feature = "service-organizations")]
pub use winterbaume_organizations as organizations;
#[cfg(feature = "service-osis")]
pub use winterbaume_osis as osis;
#[cfg(feature = "service-personalize")]
pub use winterbaume_personalize as personalize;
#[cfg(feature = "service-pinpoint")]
pub use winterbaume_pinpoint as pinpoint;
#[cfg(feature = "service-pipes")]
pub use winterbaume_pipes as pipes;
#[cfg(feature = "service-polly")]
pub use winterbaume_polly as polly;
#[cfg(feature = "service-quicksight")]
pub use winterbaume_quicksight as quicksight;
#[cfg(feature = "service-ram")]
pub use winterbaume_ram as ram;
#[cfg(feature = "service-rds")]
pub use winterbaume_rds as rds;
#[cfg(feature = "service-rdsdata")]
pub use winterbaume_rdsdata as rdsdata;
#[cfg(feature = "service-redshiftdata")]
pub use winterbaume_redshiftdata as redshiftdata;
#[cfg(feature = "service-rekognition")]
pub use winterbaume_rekognition as rekognition;
#[cfg(feature = "service-resiliencehub")]
pub use winterbaume_resiliencehub as resiliencehub;
#[cfg(feature = "service-resourcegroups")]
pub use winterbaume_resourcegroups as resourcegroups;
#[cfg(feature = "service-resourcegroupstagging")]
pub use winterbaume_resourcegroupstagging as resourcegroupstagging;
#[cfg(feature = "service-route53")]
pub use winterbaume_route53 as route53;
#[cfg(feature = "service-route53domains")]
pub use winterbaume_route53domains as route53domains;
#[cfg(feature = "service-route53resolver")]
pub use winterbaume_route53resolver as route53resolver;
#[cfg(feature = "service-s3")]
pub use winterbaume_s3 as s3;
#[cfg(feature = "service-sagemaker")]
pub use winterbaume_sagemaker as sagemaker;
#[cfg(feature = "service-sagemakermetrics")]
pub use winterbaume_sagemakermetrics as sagemakermetrics;
#[cfg(feature = "service-sagemakerruntime")]
pub use winterbaume_sagemakerruntime as sagemakerruntime;
#[cfg(feature = "service-scheduler")]
pub use winterbaume_scheduler as scheduler;
#[cfg(feature = "service-secretsmanager")]
pub use winterbaume_secretsmanager as secretsmanager;
#[cfg(feature = "service-securityhub")]
pub use winterbaume_securityhub as securityhub;
#[cfg(feature = "service-servicecatalog")]
pub use winterbaume_servicecatalog as servicecatalog;
#[cfg(feature = "service-servicecatalogappregistry")]
pub use winterbaume_servicecatalogappregistry as servicecatalogappregistry;
#[cfg(feature = "service-servicediscovery")]
pub use winterbaume_servicediscovery as servicediscovery;
#[cfg(feature = "service-servicequotas")]
pub use winterbaume_servicequotas as servicequotas;
#[cfg(feature = "service-ses")]
pub use winterbaume_ses as ses;
#[cfg(feature = "service-sesv2")]
pub use winterbaume_sesv2 as sesv2;
#[cfg(feature = "service-sfn")]
pub use winterbaume_sfn as sfn;
#[cfg(feature = "service-shield")]
pub use winterbaume_shield as shield;
#[cfg(feature = "service-signer")]
pub use winterbaume_signer as signer;
#[cfg(feature = "service-simpledbv2")]
pub use winterbaume_simpledbv2 as simpledbv2;
#[cfg(feature = "service-sns")]
pub use winterbaume_sns as sns;
#[cfg(feature = "service-sqs")]
pub use winterbaume_sqs as sqs;
#[cfg(feature = "service-ssm")]
pub use winterbaume_ssm as ssm;
#[cfg(feature = "service-sso")]
pub use winterbaume_sso as sso;
#[cfg(feature = "service-sts")]
pub use winterbaume_sts as sts;
#[cfg(feature = "service-support")]
pub use winterbaume_support as support;
#[cfg(feature = "service-swf")]
pub use winterbaume_swf as swf;
#[cfg(feature = "service-synthetics")]
pub use winterbaume_synthetics as synthetics;
#[cfg(feature = "service-textract")]
pub use winterbaume_textract as textract;
#[cfg(feature = "service-timestreaminfluxdb")]
pub use winterbaume_timestreaminfluxdb as timestreaminfluxdb;
#[cfg(feature = "service-timestreamquery")]
pub use winterbaume_timestreamquery as timestreamquery;
#[cfg(feature = "service-timestreamwrite")]
pub use winterbaume_timestreamwrite as timestreamwrite;
#[cfg(feature = "service-transcribe")]
pub use winterbaume_transcribe as transcribe;
#[cfg(feature = "service-transfer")]
pub use winterbaume_transfer as transfer;
#[cfg(feature = "service-vpclattice")]
pub use winterbaume_vpclattice as vpclattice;
#[cfg(feature = "service-wafv2")]
pub use winterbaume_wafv2 as wafv2;
#[cfg(feature = "service-workspaces")]
pub use winterbaume_workspaces as workspaces;
#[cfg(feature = "service-workspacesweb")]
pub use winterbaume_workspacesweb as workspacesweb;
#[cfg(feature = "service-xray")]
pub use winterbaume_xray as xray;
